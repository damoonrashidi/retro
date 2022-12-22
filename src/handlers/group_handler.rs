use tui_textarea::{Input, Key};

use crate::app::{mode::Mode, state::State};

pub fn handle_group(input: &Input, state: &mut State) {
    if state.mode != Mode::Group {
        return;
    }
    match input {
        Input { key: Key::Down, .. } => {
            let next_row = match state.selected_row {
                Some(row) => (row + 1).min(state.notes.len() - 1),
                None => 0,
            };
            state.select_row(next_row);
        }

        Input { key: Key::Up, .. } => {
            let next_row = match state.selected_row {
                Some(row) => row.max(1) - 1,
                None => 0,
            };
            state.select_row(next_row);
        }
        _ => {}
    }
}
