use crossterm::event::{KeyCode, KeyEvent};

use crate::app::state::State;

pub fn handle_show_help(input: KeyEvent, state: &mut State) {
    if let KeyEvent {
        code: KeyCode::Char('?'),
        ..
    } = input
    {
        state.show_help = !state.show_help;
    }
}
