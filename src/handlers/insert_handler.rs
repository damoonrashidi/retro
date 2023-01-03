use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui_textarea::{CursorMove, TextArea};

use crate::{
    app::{mode::Mode, note::Note, state::State},
    network::actions::NetworkAction,
};

pub fn handle_insert(input: KeyEvent, state: &mut State, textarea: &mut TextArea<'_>) {
    if state.mode != Mode::Insert {
        return;
    }

    match input {
        KeyEvent {
            code: KeyCode::Backspace,
            modifiers: KeyModifiers::ALT,
            ..
        } => {
            textarea.delete_word();
        }
        KeyEvent {
            code: KeyCode::Left,
            modifiers: KeyModifiers::ALT,
            ..
        } => textarea.move_cursor(CursorMove::WordBack),

        KeyEvent {
            code: KeyCode::Right,
            modifiers: KeyModifiers::ALT,
            ..
        } => textarea.move_cursor(CursorMove::WordForward),
        KeyEvent {
            code: KeyCode::Enter,
            ..
        } => {
            if !textarea.is_empty() {
                let text = textarea.lines().join("\n");
                state.dispatch(NetworkAction::PublishNote(Note::new(
                    state.display_name.clone(),
                    text,
                )));
            }
            textarea.delete_line_by_head();
        }
        input => {
            textarea.input(input.clone());
        }
    };
}
