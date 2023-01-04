use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui_textarea::{CursorMove, TextArea};

use crate::app::{mode::Mode, state::State};

pub fn handle_command(input: KeyEvent, state: &mut State, textarea: &mut TextArea<'_>) {
    if state.mode != Mode::Command {
        return;
    }

    match input {
        KeyEvent {
            code: KeyCode::Esc, ..
        } => {
            textarea.delete_line_by_head();
            state.deselect_rows();
        }

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
            let selected = state.selected_rows.clone();

            let ids = selected
                .iter()
                .filter_map(|index| state.notes.get(*index).map(|note| note.id.clone()))
                .collect();

            match textarea.lines().join("").get(0..1).unwrap() {
                "v" => state.upvote(&ids),
                "d" => state.unvote(&ids),
                _ => {}
            }
        }

        input => {
            textarea.input(input);
        }
    };

    if let Some((command, indices_str)) = textarea
        .lines()
        .first()
        .unwrap()
        .as_str()
        .split_once(char::is_whitespace)
    {
        match command {
            "group" | "g" => {
                let indicies = get_indicies(indices_str.to_owned());
                state.select_rows(&indicies);
            }
            "vote" | "v" => {
                let indicies = get_indicies(indices_str.to_owned());
                state.select_rows(&indicies);
            }
            _ => {}
        }
    }

    fn get_indicies(indices: String) -> Vec<usize> {
        indices
            .split(char::is_whitespace)
            .map(|it| it.parse())
            .filter_map(|it| it.ok())
            .collect()
    }
}
