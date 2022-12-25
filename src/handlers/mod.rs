use tui_textarea::{Input, TextArea};

use crate::app::state::State;

use self::{
    group_handler::handle_group, help_handler::handle_show_help, insert_handler::handle_insert,
    mode_handler::handle_mode, vote_handler::handle_vote,
};

pub mod group_handler;
pub mod help_handler;
pub mod insert_handler;
pub mod mode_handler;
pub mod vote_handler;

pub fn handle_input(input: &Input, state: &mut State, textarea: &mut TextArea<'_>) {
    handle_show_help(input, state);
    handle_vote(input, state);
    handle_group(input, state);
    handle_insert(input, state, textarea);
    handle_mode(input, state);
}
