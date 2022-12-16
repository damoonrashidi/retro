// use std::io;

// use firestore::FirestoreDb;

// use crate::note::Note;
// const PROJECT_ID: &str = "retrodog-23512";

/// Fetch remote retro information
#[derive(Debug)]
pub struct Remote<'a> {
    /// name of the retro
    pub retro: &'a String,
}

impl<'a> Remote<'a> {}
