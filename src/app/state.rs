use std::{collections::HashSet, fmt::Debug, sync::mpsc::Sender};

use crate::{app::mode::Mode, app::note::Note, cli::RetroArgs, network::actions::NetworkAction};

use super::sentiment::Sentiment;

#[derive(Clone, Debug)]
/// Application state
pub struct State {
    /// what row is selected (used by the vote/group modes)
    pub selected_rows: Vec<usize>,

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

    /// Tick count, decides when to redraw the ui
    pub tick_count: usize,
}

impl State {
    pub fn new(sender: Sender<NetworkAction>, args: RetroArgs) -> Self {
        State {
            selected_rows: vec![],
            participants: HashSet::new(),
            filter: None,
            mode: Mode::Normal,
            notes: vec![],
            my_votes: HashSet::new(),
            show_help: false,
            display_name: args.display_name,
            sender,
            tick_count: 0,
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

    pub fn upvote(&mut self, ids: &Vec<String>) {
        for id in ids {
            if let (Some(note), false) = (
                self.notes.clone().into_iter().find(|note| note.id == *id),
                self.my_votes.contains(id),
            ) {
                self.dispatch(NetworkAction::Vote(note.clone()));
                self.my_votes.insert(note.id.clone());
            }
        }
    }

    pub fn unvote(&mut self, ids: &Vec<String>) {
        for id in ids {
            if let (Some(note), true) = (
                &self.notes.clone().into_iter().find(|note| note.id == *id),
                self.my_votes.contains(id),
            ) {
                let cloned = note.clone();
                self.dispatch(NetworkAction::Unvote(cloned));

                // @TODO remove the current
            }
        }
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;
    }

    pub fn remove_note(&mut self, id: &String) {
        println!("{}", id);
    }

    pub fn select_rows(&mut self, rows: &Vec<usize>) {
        self.selected_rows = rows.to_owned();
    }

    pub fn deselect_rows(&mut self) {
        self.selected_rows = vec![];
    }

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
