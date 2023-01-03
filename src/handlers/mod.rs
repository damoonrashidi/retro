use std::sync::MutexGuard;

use crossterm::event::KeyEvent;
use tui_textarea::TextArea;

use crate::app::state::State;

pub mod group_handler;
pub mod help_handler;
pub mod insert_handler;
pub mod mode_handler;
pub mod vote_handler;

pub fn handle_input(
    input: KeyEvent,
    state: &mut MutexGuard<'_, State>,
    textarea: &mut TextArea<'_>,
) {
    help_handler::handle_show_help(input, state);
    vote_handler::handle_vote(input, state);
    group_handler::handle_group(input, state);
    insert_handler::handle_insert(input, state, textarea);
    mode_handler::handle_mode(input, state);
}
