use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{mode::Mode, state::State};

pub fn handle_mode(input: KeyEvent, state: &mut State) {
    match state.mode {
        Mode::Normal => match input {
            KeyEvent {
                code: KeyCode::Char('v'),
                ..
            } => {
                state.mode = Mode::Vote;
                if !state.notes.is_empty() {
                    state.select_row(0);
                }
            }

            KeyEvent {
                code: KeyCode::Char('g'),
                ..
            } => {
                state.mode = Mode::Group;
                if !state.notes.is_empty() {
                    state.select_row(0);
                }
            }
            KeyEvent {
                code: KeyCode::Char('i'),
                ..
            } => state.mode = Mode::Insert,
            KeyEvent {
                code: KeyCode::Char('f'),
                ..
            } => state.mode = Mode::Find,
            _ => {}
        },
        _ => {
            if let KeyEvent {
                code: KeyCode::Esc, ..
            } = input
            {
                state.mode = Mode::Normal;
                state.deselect_row();
            }
        }
    };
}
