use tui_textarea::{Input, Key};

use crate::app::{mode::Mode, state::State};

pub fn handle_vote(input: &Input, state: &mut State) {
    if state.mode != Mode::Vote {
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

        Input {
            key: Key::Enter, ..
        } => {
            if let Some(index) = state.selected_row {
                if let Some(note) = &state.notes.clone().get(index) {
                    state.upvote(&note.id);
                }
            }
        }

        Input {
            key: Key::Backspace,
            ..
        } => {
            if let Some(index) = state.selected_row {
                if let Some(note) = &state.notes.clone().get(index) {
                    state.downvote(&note.id);
                }
            }
        }

        _ => {}
    }
}
