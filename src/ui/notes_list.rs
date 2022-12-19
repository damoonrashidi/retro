use tui::widgets::{Block, Borders, List, ListItem};

use crate::app::state::State;

pub fn notes_list(state: &State) -> List<'static> {
    let items: Vec<ListItem<'static>> = state
        .notes
        .iter()
        .map(|note| ListItem::new(note.to_string()))
        .collect();

    List::new(items).block(Block::default().borders(Borders::all()).title("Notes"))
}
