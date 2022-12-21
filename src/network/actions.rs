use crate::app::note::Note;

#[derive(Debug, Clone)]
pub enum NetworkAction {
    JoinRetro(String),
    PublishNote(Note),
    Vote(Note),
    Unvote(Note),
    Group(Note, Note),
    GetNotes,
    ListenForChanges,
}
