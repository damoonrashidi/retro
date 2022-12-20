use tui_textarea::{Input, Key};

use crate::{
    app::{mode::Mode, state::State},
    network::actions::NetworkAction,
};

pub fn handle_mode(input: &Input, state: &mut State) {
    match state.mode {
        Mode::Normal => match input {
            Input {
                key: Key::Char('v'),
                ..
            } => {
                state.mode = Mode::Vote;
                state.select_row(0);
            }
            Input {
                key: Key::Char('g'),
                ..
            } => {
                state.mode = Mode::Group;
                state.select_row(0);
            }
            Input {
                key: Key::Char('i'),
                ..
            } => state.mode = Mode::Insert,
            Input {
                key: Key::Char('f'),
                ..
            } => state.mode = Mode::Find,
            Input {
                key: Key::Char('s'),
                ..
            } => {
                state.dispatch(NetworkAction::GetNotes);
            }
            _ => {}
        },
        _ => {
            if let Input { key: Key::Esc, .. } = input {
                state.mode = Mode::Normal;
                state.deselect_row();
            }
        }
    }
}
