use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{mode::Mode, note::Note, state::State},
    network::actions::NetworkAction,
};

pub fn handle_vote(input: KeyEvent, state: &mut State) {
    if state.mode != Mode::Vote {
        return;
    }

    match input {
        KeyEvent {
            code: KeyCode::Down,
            ..
        } => {
            let next_row = match state.selected_row {
                Some(row) => (row + 1).min(state.notes.len()),
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

        KeyEvent {
            code: KeyCode::Enter,
            ..
        } => {
            if let Some(index) = state.selected_row {
                if let Some(note) = &state.notes.clone().get(index) {
                    state.dispatch(NetworkAction::Vote(Note::clone(note)));
                }
            }
        }

        KeyEvent {
            code: KeyCode::Backspace,
            ..
        } => {
            if let Some(index) = state.selected_row {
                if let Some(note) = &state.notes.clone().get(index) {
                    state.dispatch(NetworkAction::Unvote(Note::clone(note)));
                }
            }
        }

        _ => {}
    }
}
