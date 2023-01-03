use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{mode::Mode, state::State};

pub fn handle_group(input: KeyEvent, state: &mut State) {
    if state.mode != Mode::Group {
        return;
    }
    match input {
        KeyEvent {
            code: KeyCode::Down,
            ..
        } => {
            let next_row = match state.selected_row {
                Some(row) => (row + 1).min(state.notes.len().max(1) - 1),
                None => 0,
            };
            state.select_row(next_row);
        }

        KeyEvent {
            code: KeyCode::Up, ..
        } => {
            let next_row = match state.selected_row {
                Some(row) => row.max(1) - 1,
                None => 0,
            };
            state.select_row(next_row);
        }
        _ => {}
    }
}
