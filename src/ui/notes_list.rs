use tui::{
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
};

use crate::app::{mode::Mode, note::Note, state::State};

pub fn notes_list(state: &State) -> List<'static> {
    let items: Vec<ListItem<'static>> = state
        .notes
        .iter()
        .enumerate()
        .map(|(index, note)| {
            ListItem::new(display_note(note, &state.mode, &index)).style(get_style(&index, state))
        })
        .collect();

    List::new(items).block(Block::default().borders(Borders::all()).title("Notes"))
}

pub fn display_note(note: &Note, mode: &Mode, index: &usize) -> String {
    match mode {
        Mode::Command => format!("{index} {note}",),
        _ => note.to_string(),
    }
}

fn get_style(index: &usize, state: &State) -> Style {
    let is_included = state.selected_rows.contains(index);

    let bg = match state.mode {
        Mode::Command => {
            if is_included {
                Color::LightGreen
            } else {
                Color::Reset
            }
        }
        _ => Color::Reset,
    };

    Style::default().bg(bg)
}
