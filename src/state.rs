use std::collections::{HashMap, HashSet};

use crate::note::{Note, Sentiment};

pub struct State {
    notes: HashMap<String, Note>,
    pub selected_row: Option<usize>,
    pub participants: Vec<String>,
    my_votes: HashSet<String>,
}

impl State {
    pub fn new() -> Self {
        State {
            notes: HashMap::new(),
            selected_row: None,
            participants: vec![],
            my_votes: HashSet::new(),
        }
    }

    pub fn notes_as_list(&self) -> Vec<Note> {
        self.notes.values().cloned().collect()
    }

    pub fn add_note(&mut self, note: Note) -> () {
        self.notes.insert(note.id.clone(), note);
    }

    #[allow(unused)]
    pub fn upvote(&mut self, id: String) -> () {
        if let Some(note) = self.notes.get_mut(&id) {
            if !self.my_votes.contains(&note.id) {
                note.votes += 1;
                self.my_votes.insert(note.id.clone());
            }
        }
    }

    #[allow(unused)]
    pub fn downvote(&mut self, id: String) -> () {
        if let Some(note) = self.notes.get_mut(&id) {
            if self.my_votes.contains(&note.id) && note.votes > 0 {
                note.votes -= 1;
                self.my_votes.remove(&note.id);
            }
        }
    }

    #[allow(unused)]
    pub fn group_notes(&mut self, id1: &String, id2: &String) -> () {
        let first = self.notes.get(id1).unwrap();
        let second = self.notes.get(id2).unwrap();

        let merged = Note {
            author: "Multiple authors".into(),
            text: format!("{}\n{}", first.text, second.text),
            id: first.id.clone(),
            sentiment: crate::note::Sentiment::Neutral,
            votes: first.votes + second.votes,
        };

        self.notes.insert(first.id.clone(), merged);
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
