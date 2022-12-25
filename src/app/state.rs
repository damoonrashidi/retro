use std::{collections::HashSet, sync::mpsc::Sender};

use crate::{app::mode::Mode, app::note::Note, cli::RetroArgs, network::actions::NetworkAction};

use super::sentiment::Sentiment;

/// Application state
#[derive(Debug)]
pub struct State {
    /// what row is selected (used by the vote/group modes)
    pub selected_row: Option<usize>,

    /// List of participants (display_names)
    pub participants: HashSet<String>,

    /// if in filter mode display only the notes matching this filter
    pub filter: Option<Sentiment>,

    /// Active mode
    pub mode: Mode,

    /// List of all notes, by any author
    pub notes: Vec<Note>,

    // A set of ids for the notes the current user has voted for
    my_votes: HashSet<String>,

    /// If true, a box with a list of shorcuts for the active mode will be shown
    pub show_help: bool,

    /// Display name of the current user
    pub display_name: String,

    sender: Sender<NetworkAction>,
}

impl State {
    pub fn new(sender: Sender<NetworkAction>, args: RetroArgs) -> Self {
        State {
            mode: Mode::Normal,
            notes: vec![],
            selected_row: None,
            participants: HashSet::new(),
            my_votes: HashSet::new(),
            show_help: false,
            filter: None,
            sender,
            display_name: args.display_name,
        }
    }

    pub fn dispatch(&mut self, action: NetworkAction) {
        if let Err(e) = self.sender.send(action) {
            println!("error {}", e);
        }
    }

    pub fn add_note(&mut self, note: Note) {
        self.dispatch(NetworkAction::PublishNote(note));
    }

    pub fn set_notes(&mut self, notes: Vec<Note>) {
        self.notes = notes;
    }

    pub fn set_participants(&mut self, participants: HashSet<String>) {
        self.participants = participants;
    }

    pub fn upvote(&mut self, note: &Note) {
        if !self.my_votes.contains(&note.id) {
            self.dispatch(NetworkAction::Vote(note.clone()));
            self.my_votes.insert(note.id.clone());
        }
    }

    pub fn downvote(&mut self, note: &Note) {
        if self.my_votes.contains(&note.id) {
            self.dispatch(NetworkAction::Unvote(note.clone()));
            self.my_votes.remove(&note.id);
        }
    }

    pub fn group_notes(&mut self, id1: &String, id2: &String) -> Result<Note, &str> {
        if id1 == id2 {
            return Err("");
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
        println!("{}", id);
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
            .iter()
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
            participants: HashSet::new(),
            filter: None,
            mode: Mode::Normal,
            notes: vec![],
            my_votes: HashSet::new(),
            show_help: false,
            sender: std::sync::mpsc::channel::<NetworkAction>().0,
            display_name: "".to_string(),
        }
    }
}
