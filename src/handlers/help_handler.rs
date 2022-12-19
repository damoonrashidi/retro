use tui_textarea::{Input, Key};

use crate::app::state::State;

pub fn handle_show_help(input: &Input, state: &mut State) -> () {
    match input {
        Input {
            key: Key::Char('/'),
            ..
        } => {
            state.show_help = true;
        }
        _ => {}
    }
}
