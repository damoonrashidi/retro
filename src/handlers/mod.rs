use std::sync::MutexGuard;

use crossterm::event::KeyEvent;
use tui_textarea::TextArea;

use crate::app::state::State;

use self::mode_handler::handle_mode;

pub mod group_handler;
pub mod help_handler;
pub mod insert_handler;
pub mod mode_handler;
pub mod vote_handler;

pub fn handle_input(input: KeyEvent, state: MutexGuard<'_, State>, _textarea: &mut TextArea<'_>) {
    // handle_show_help(input, state);
    // handle_vote(input, state);
    // handle_group(input, state);
    // handle_insert(input, state, textarea);
    handle_mode(input, state);
}
