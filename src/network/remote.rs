use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use crate::app::{note::Note, state::State};

use super::actions::NetworkAction;
use anyhow::Result;

use firestore_grpc::{
    tonic::{metadata::MetadataValue, transport::Channel, Request},
    v1::{
        firestore_client::FirestoreClient,
        listen_request::TargetChange,
        listen_response::ResponseType,
        target::{DocumentsTarget, TargetType},
        CreateDocumentRequest, Document, ListDocumentsRequest, ListenRequest, ListenResponse,
        Target, UpdateDocumentRequest,
    },
};

use futures::{stream, StreamExt};

#[derive(Clone, Debug)]
pub struct Remote<'a> {
    project_id: &'static str,

    room_id: &'a String,

    pub state: &'a Arc<Mutex<State>>,
}

enum VoteDirection {
    Up,
    Down,
}

impl<'a> Remote<'a> {
    pub fn new(room_id: &'a String, state: &'a Arc<Mutex<State>>) -> Self {
        let project_id = "retrodog-23512";

        Remote {
            project_id,
            room_id,
            state,
        }
    }

    pub async fn handle_event(&self, action: NetworkAction) -> Result<()> {
        match action {
            NetworkAction::JoinRetro(room_id) => println!("join {}", room_id),
            NetworkAction::PublishNote(note) => {
                self.create_note(&note).await?;
            }
            NetworkAction::Vote(note) => {
                self.vote(&note, VoteDirection::Up).await?;
            }
            NetworkAction::Unvote(note) => {
                self.vote(&note, VoteDirection::Down).await?;
            }
            NetworkAction::Group(id1, id2) => println!("group {}{}", id1, id2),
            NetworkAction::GetNotes => {
                self.get_notes().await?;
            }
            NetworkAction::ListenForChanges => {
                self.listen_for_changes().await?;
            }
        }
        Ok(())
    }

    async fn create_note(&self, note: &'a Note) -> Result<&Note> {
        let (root, mut client, _) = self.get_client().await?;

        client
            .create_document(CreateDocumentRequest {
                parent: root.to_string(),
                collection_id: "notes".into(),
                document_id: "".into(),
                document: Some(firestore_grpc::v1::Document {
                    name: "".into(),
                    fields: note.into(),
                    create_time: None,
                    update_time: None,
                }),
                mask: None,
            })
            .await?;

        Ok(note)
    }
    async fn listen_for_changes(&self) -> Result<()> {
        let (room, mut client, db) = self.get_client().await?;

        let req = ListenRequest {
            database: db.clone(),
            labels: HashMap::new(),
            target_change: Some(TargetChange::AddTarget(Target {
                target_id: 0x52757374,
                once: false,
                target_type: Some(TargetType::Documents(DocumentsTarget {
                    documents: vec![room],
                })),
                resume_type: None,
            })),
        };

        let mut req = Request::new(stream::iter(vec![req]).chain(stream::pending()));
        let metadata = req.metadata_mut();
        metadata.insert(
            "google-cloud-resource-prefix",
            MetadataValue::from_str(&db).unwrap(),
        );

        let mut res = client.listen(req).await?.into_inner();

        while let Some(msg) = res.next().await {
            if let Ok(msg) = msg {
                if let ListenResponse {
                    response_type: Some(ResponseType::DocumentChange(document)),
                } = msg
                {
                    println!("{:?}", document)
                }
                let mut state = self.state.lock().expect("oh no");
                state.dispatch(NetworkAction::GetNotes);
                // state.dispatch(NetworkAction::ListenForChanges);
                break;
            }
        }

        Ok(())
    }

    async fn get_notes(&self) -> Result<()> {
        let (root, mut client, _db) = self.get_client().await?;

        let res = client
            .list_documents(ListDocumentsRequest {
                parent: root,
                collection_id: "notes".to_string(),
                page_size: 1000,
                page_token: "".to_string(),
                order_by: "votes".to_string(),
                mask: None,
                show_missing: false,
                consistency_selector: None,
            })
            .await?;

        let notes: Vec<Note> = res
            .into_inner()
            .documents
            .into_iter()
            .rev()
            .map(|note| {
                let mut converted: Note = note.fields.into();
                converted.id = note.name;
                converted
            })
            .collect();

        let participants = HashSet::from_iter(
            notes
                .iter()
                .map(|note| &note.author)
                .cloned()
                .collect::<Vec<String>>(),
        );

        let mut state = self.state.lock().expect("oh no");
        state.set_notes(notes);
        state.set_participants(participants);

        Ok(())
    }

    async fn vote(&self, note: &Note, direction: VoteDirection) -> Result<()> {
        let (_root, mut client, _) = self.get_client().await?;

        let changed = &Note {
            votes: if let VoteDirection::Up = direction {
                note.votes.checked_add(1).unwrap()
            } else {
                note.votes.checked_sub(1).unwrap()
            },
            ..note.clone()
        };

        client
            .update_document(UpdateDocumentRequest {
                document: Some(Document {
                    name: note.id.clone(),
                    fields: changed.into(),
                    create_time: None,
                    update_time: None,
                }),
                update_mask: None,
                mask: None,
                current_document: None,
            })
            .await?;

        Ok(())
    }

    async fn get_client(&self) -> Result<(String, FirestoreClient<Channel>, String)> {
        let db = format!("projects/{}/databases/(default)", self.project_id);
        let room = format!("{db}/documents/retros/{}", self.room_id);

        let service = FirestoreClient::connect("https://firestore.googleapis.com").await?;

        Ok((room, service, db))
    }
}
