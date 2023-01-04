use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{mode::Mode, state::State};

pub fn handle_mode(input: KeyEvent, state: &mut State) {
    match state.mode {
        Mode::Normal => match input {
            KeyEvent {
                code: KeyCode::Char('i'),
                ..
            } => state.mode = Mode::Insert,

            KeyEvent {
                code: KeyCode::Char(':'),
                ..
            } => state.mode = Mode::Command,

            _ => {}
        },
        _ => {
            if let KeyEvent {
                code: KeyCode::Esc, ..
            } = input
            {
                state.mode = Mode::Normal;
                state.deselect_rows();
            }
        }
    };
}
