use tui_textarea::Input;

use crate::app::state::State;

use self::{help_handler::handle_show_help, mode_handler::handle_mode, vote_handler::handle_vote};

pub mod help_handler;
pub mod mode_handler;
pub mod vote_handler;

pub fn handle_input(input: &Input, state: &mut State) -> () {
    handle_show_help(input, state);
    handle_vote(input, state);
    handle_mode(input, state);
}
