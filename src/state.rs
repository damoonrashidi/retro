use std::collections::{HashMap, HashSet};

use crate::{
    mode::Mode,
    note::{Note, Sentiment},
};

/// Application state
#[derive(Debug)]
pub struct State {
    /// what row is selected (used by the vote/group modes)
    pub selected_row: Option<usize>,

    /// List of participants (display_names)
    pub participants: Vec<String>,

    /// if in filter mode display only the notes matching this filter
    pub filter: Option<Sentiment>,

    /// Active mode
    pub mode: Mode,

    /// List of all notes, by any author
    notes: HashMap<String, Note>,

    // A set of ids for the notes the current user has voted for
    my_votes: HashSet<String>,

    /// If true, a box with a list of shorcuts for the active mode will be shown
    pub show_help: bool,
}

impl State {
    pub fn new() -> Self {
        State {
            mode: Mode::Normal,
            notes: HashMap::new(),
            selected_row: None,
            participants: vec![],
            my_votes: HashSet::new(),
            show_help: false,
            filter: None,
        }
    }

    pub fn notes_as_list(&self) -> Vec<Note> {
        if let Some(filter) = self.filter {
            return self
                .notes
                .values()
                .cloned()
                .into_iter()
                .filter(|note| note.sentiment == filter)
                .collect();
        }

        self.notes.values().cloned().collect()
    }

    pub fn add_note(&mut self, note: Note) {
        self.notes.insert(note.id.clone(), note);
    }

    pub fn upvote(&mut self, id: String) {
        if let Some(note) = self.notes.get_mut(&id) {
            if !self.my_votes.contains(&note.id) {
                note.votes += 1;
                self.my_votes.insert(note.id.clone());
            }
        }
    }

    pub fn downvote(&mut self, id: String) {
        if let Some(note) = self.notes.get_mut(&id) {
            if self.my_votes.contains(&note.id) && note.votes > 0 {
                note.votes -= 1;
                self.my_votes.remove(&note.id);
            }
        }
    }

    pub fn group_notes(&mut self, id1: &String, id2: &String) -> Result<Note, &str> {
        if id1 == id2 {
            return Err("");
        }

        if let (Some(first), Some(second)) = (self.notes.get(id1), self.notes.get(id2)) {
            let merged = Note {
                author: "Multiple authors".into(),
                text: format!("{}\n{}", first.text, second.text),
                id: first.id.clone(),
                sentiment: crate::note::Sentiment::Neutral,
                votes: first.votes + second.votes,
            };

            self.notes.insert(first.id.clone(), merged.clone());

            return Ok(merged.clone());
        }

        Err("No")
    }

    pub fn set_filter(&mut self, sentiment: Sentiment) {
        self.filter = Some(sentiment);
    }

    pub fn reset_filter(&mut self) {
        self.filter = None;
    }

    pub fn remove_note(&mut self, id: &String) {
        self.notes.remove(id);
    }

    pub fn select_row(&mut self, index: usize) {
        self.selected_row = Some(index);
    }

    pub fn deselect_row(&mut self) {
        self.selected_row = None;
    }

    #[allow(unused)]
    pub fn sentiment_count(&self) -> [(Sentiment, usize); 3] {
        let total = self
            .notes
            .values()
            .fold((0, 0, 0), |counts, note| match note.sentiment {
                Sentiment::Happy => (counts.0 + 1, counts.1, counts.2),
                Sentiment::Sad => (counts.0, counts.1 + 1, counts.2),
                Sentiment::Neutral => (counts.0, counts.1, counts.2 + 1),
            });

        [
            (Sentiment::Happy, total.0),
            (Sentiment::Sad, total.1),
            (Sentiment::Neutral, total.2),
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::note::Note;

    use super::State;

    #[test]
    fn test_add_note() {
        let mut state = State::new();
        state.add_note(Note::new("test note".into(), "note-id".into()));

        let inserted = state.notes.get("note-id".into()).unwrap();
        assert_eq!(inserted.text, String::from("test note"));
    }

    #[test]
    fn test_vote_up() {
        let mut state = State::new();
        state.add_note(Note::new("test note".into(), "note-id".into()));
        state.upvote(String::from("note-id"));
        let inserted = state.notes.get("note-id".into()).unwrap();

        assert_eq!(inserted.votes, 1);
    }

    #[test]
    fn test_vote_down() {
        let mut state = State::new();
        state.add_note(Note {
            id: "id".to_string(),
            text: "text".to_string(),
            author: "Retro Guy".to_string(),
            sentiment: crate::note::Sentiment::Happy,
            votes: 5,
        });
        state.downvote("id".to_string());
        let inserted = state.notes.get("note-id".into()).unwrap();

        assert_eq!(inserted.votes, 4);
    }
}
