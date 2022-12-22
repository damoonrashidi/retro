use tui_textarea::{Input, Key};

use crate::app::state::State;

pub fn handle_show_help(input: &Input, state: &mut State) {
    if let Input {
        key: Key::Char('?'),
        ..
    } = input
    {
        state.show_help = !state.show_help;
    }
}
