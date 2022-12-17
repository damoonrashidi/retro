use crate::state::note::Note;

use super::actions::NetworkAction;
use anyhow::Result;
use firestore::FirestoreDb;

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

        let db = FirestoreDb::new(self.project_id).await?;

        println!("{:?}, \n...getting notes", db);
        let retros: Vec<Note> = db.fluent().select().from("retros").obj().query().await?;

        println!("notes: {:?}", retros);

        Ok(())
    }
}
