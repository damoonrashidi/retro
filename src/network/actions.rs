use crate::state::note::Note;

#[derive(Debug, Clone)]
pub enum NetworkAction {
    JoinRetro(String),
    PublishNote(Note),
    Vote(String),
    Unvote(String),
    Group(String, String),
    GetNotes,
}
