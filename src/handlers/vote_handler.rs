use tui_textarea::{Input, Key};

use crate::app::{mode::Mode, state::State};

pub fn handle_vote(input: &Input, state: &mut State) -> () {
    if state.mode != Mode::Vote {
        return;
    }

    match input {
        Input { key: Key::Down, .. } => {
            let next_row = match state.selected_row {
                Some(row) => (row + 1).min(state.notes.len()),
                None => 0,
            };
            state.select_row(next_row);
        }
        _ => {}
    }
}
