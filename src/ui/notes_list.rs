use tui::{
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
};

use crate::app::{mode::Mode, state::State};

pub fn notes_list(state: &State) -> List<'static> {
    let items: Vec<ListItem<'static>> = state
        .notes
        .iter()
        .enumerate()
        .map(|(index, note)| ListItem::new(note.to_string()).style(get_style(index, state)))
        .collect();

    List::new(items).block(Block::default().borders(Borders::all()).title("Notes"))
}

fn get_style(index: usize, state: &State) -> Style {
    let is_selected = match state.selected_row {
        Some(selected) => selected == index,
        None => false,
    };

    let bg = match state.mode {
        Mode::Group => {
            if is_selected {
                Color::LightRed
            } else {
                Color::Reset
            }
        }
        Mode::Vote => {
            if is_selected {
                Color::LightGreen
            } else {
                Color::Reset
            }
        }
        _ => Color::Reset,
    };

    Style::default().bg(bg)
}
