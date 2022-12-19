use super::actions::NetworkAction;
use anyhow::Result;
use firestore_grpc::{
    tonic::transport::Channel,
    v1::{firestore_client::FirestoreClient, ListDocumentsRequest},
};

#[derive(Debug, Clone, Copy)]
pub struct Network<'a> {
    project_id: &'static str,

    #[allow(unused)]
    room_id: &'a String,
}

impl<'a> Network<'a> {
    pub fn new(room_id: &'a String) -> Self {
        let project_id = "retrodog-23512";

        Network {
            project_id,
            room_id,
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

    pub async fn get_notes(&self) -> Result<()> {
        println!("setting up db connection");

        let (root, mut client) = self.get_client().await?;

        match client
            .list_documents(ListDocumentsRequest {
                parent: format!("{}/notes", root),
                consistency_selector: None,
                mask: None,
                collection_id: "notes".into(),
                page_size: 10,
                page_token: "".into(),
                order_by: "author".into(),
                show_missing: false,
            })
            .await
        {
            Ok(res) => {
                println!("{:?}", res);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        };

        Ok(())
    }

    async fn get_client(&self) -> Result<(String, FirestoreClient<Channel>)> {
        let service = FirestoreClient::connect("https://firestore.googleapis.com").await?;
        let root = format!(
            "projects/{}/databases/(default)/documents/{}",
            self.project_id, self.room_id
        );

        Ok((root, service))
    }
}
