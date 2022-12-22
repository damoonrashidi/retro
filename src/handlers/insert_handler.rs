use tui_textarea::{CursorMove, Input, Key, TextArea};

use crate::{
    app::{mode::Mode, note::Note, state::State},
    network::actions::NetworkAction,
};

pub fn handle_insert(input: &Input, state: &mut State, textarea: &mut TextArea<'_>) {
    if state.mode != Mode::Insert {
        return;
    }

    match input {
        Input {
            key: Key::Backspace,
            alt: true,
            ..
        } => {
            textarea.delete_word();
        }
        Input {
            key: Key::Left,
            alt: true,
            ..
        } => textarea.move_cursor(CursorMove::WordBack),

        Input {
            key: Key::Right,
            alt: true,
            ..
        } => textarea.move_cursor(CursorMove::WordForward),
        Input {
            key: Key::Enter, ..
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
    }
}
