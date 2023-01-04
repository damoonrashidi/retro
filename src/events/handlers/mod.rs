use std::sync::MutexGuard;

use crossterm::event::KeyEvent;
use tui_textarea::TextArea;

use crate::app::state::State;

pub mod command_handler;
pub mod help_handler;
pub mod insert_handler;
pub mod mode_handler;

pub fn handle_input(
    input: KeyEvent,
    state: &mut MutexGuard<'_, State>,
    textarea: &mut TextArea<'_>,
    command_textarea: &mut TextArea<'_>,
) {
    help_handler::handle_show_help(input, state);
    insert_handler::handle_insert(input, state, textarea);
    command_handler::handle_command(input, state, command_textarea);
    mode_handler::handle_mode(input, state);
}
