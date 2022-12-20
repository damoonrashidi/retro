use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::app::{
    note::{Note, Sentiment},
    state::State,
};

use super::actions::NetworkAction;
use anyhow::Result;
use firestore_grpc::{
    tonic::transport::Channel,
    v1::{
        firestore_client::FirestoreClient, value::ValueType, CreateDocumentRequest,
        ListDocumentsRequest, Value,
    },
};

#[derive(Debug, Clone)]
pub struct Remote<'a> {
    project_id: &'static str,

    room_id: &'a String,

    pub state: &'a Arc<Mutex<State>>,
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

    pub async fn handle_event(&self, action: NetworkAction) {
        match action {
            NetworkAction::JoinRetro(room_id) => println!("{}", room_id),
            NetworkAction::PublishNote(note) => println!("{}", note),
            NetworkAction::Vote(note_id) => println!("{}", note_id),
            NetworkAction::Unvote(note_id) => println!("{}", note_id),
            NetworkAction::Group(id1, id2) => println!("{}{}", id1, id2),
            NetworkAction::GetNotes => {
                let _ = self.get_notes().await;
            }
        }
    }

    pub async fn create_note(&self, note: &'a Note) -> Result<&Note> {
        let mut fields = HashMap::new();

        fields.insert(
            "author".into(),
            Value {
                value_type: Some(ValueType::StringValue(note.author.clone())),
            },
        );

        fields.insert(
            "text".into(),
            Value {
                value_type: Some(ValueType::StringValue(note.text.clone())),
            },
        );

        fields.insert(
            "sentiment".into(),
            Value {
                value_type: Some(ValueType::StringValue(note.sentiment.into())),
            },
        );

        fields.insert(
            "votes".into(),
            Value {
                value_type: Some(ValueType::IntegerValue(note.votes.into())),
            },
        );

        let (root, mut client) = self.get_client().await?;

        client
            .create_document(CreateDocumentRequest {
                parent: root.to_string(),
                collection_id: "notes".into(),
                document_id: "".into(),
                document: Some(firestore_grpc::v1::Document {
                    name: "".into(),
                    fields,
                    create_time: None,
                    update_time: None,
                }),
                mask: None,
            })
            .await?;

        Ok(note)
    }

    pub async fn get_notes(&self) -> Result<()> {
        let (root, mut client) = self.get_client().await?;

        let res = client
            .list_documents(ListDocumentsRequest {
                parent: root,
                consistency_selector: None,
                mask: None,
                collection_id: "notes".into(),
                page_size: 10,
                page_token: "".into(),
                order_by: "author".into(),
                show_missing: false,
            })
            .await?;

        let notes: Vec<Note> = res
            .into_inner()
            .documents
            .iter()
            .map(|doc| {
                let id: String = doc.name.clone();

                let author = match doc.fields.get("author").unwrap().value_type.clone() {
                    Some(ValueType::StringValue(author)) => author,
                    _ => "".into(),
                };

                let text = match doc.fields.get("text").unwrap().value_type.clone() {
                    Some(ValueType::StringValue(text)) => text,
                    _ => "".into(),
                };

                Note {
                    id,
                    text,
                    author,
                    sentiment: Sentiment::Neutral,
                    votes: 0,
                }
            })
            .collect();

        let mut state = match self.state.lock() {
            Ok(state) => state,
            Err(_) => panic!("oh no!"),
        };

        if let Some(sentiment) = state.filter {
            state.set_notes(
                notes
                    .into_iter()
                    .filter(|note| note.sentiment == sentiment)
                    .collect(),
            );
        } else {
            state.set_notes(notes);
        }

        Ok(())
    }

    async fn get_client(&self) -> Result<(String, FirestoreClient<Channel>)> {
        let service = FirestoreClient::connect("https://firestore.googleapis.com").await?;
        let root = format!(
            "projects/{}/databases/(default)/documents/retros/{}",
            self.project_id, self.room_id
        );

        Ok((root, service))
    }
}
