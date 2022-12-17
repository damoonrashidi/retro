use std::{
    collections::{HashMap, HashSet},
    sync::mpsc::Sender,
};

use crate::{
    network::actions::NetworkAction,
    state::mode::Mode,
    state::note::{Note, Sentiment},
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

    sender: Sender<NetworkAction>,
}

impl State {
    pub fn new(sender: Sender<NetworkAction>) -> Self {
        State {
            mode: Mode::Normal,
            notes: HashMap::new(),
            selected_row: None,
            participants: vec![],
            my_votes: HashSet::new(),
            show_help: false,
            filter: None,
            sender,
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

    pub fn dispatch(&mut self, action: NetworkAction) {
        if let Err(e) = self.sender.send(action) {
            println!("{}", e);
        }
    }

    pub fn add_note(&mut self, note: Note) {
        self.dispatch(NetworkAction::PublishNote(note));
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
                sentiment: crate::state::note::Sentiment::Neutral,
                votes: first.votes + second.votes,
            };

            self.notes.insert(first.id.clone(), merged.clone());

            return Ok(merged);
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

impl Default for State {
    fn default() -> Self {
        State {
            selected_row: None,
            participants: vec![],
            filter: None,
            mode: Mode::Normal,
            notes: HashMap::new(),
            my_votes: HashSet::new(),
            show_help: false,
            sender: std::sync::mpsc::channel::<NetworkAction>().0,
        }
    }
}
