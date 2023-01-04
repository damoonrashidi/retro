use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui_textarea::{CursorMove, TextArea};

use crate::{
    app::{mode::Mode, note::Note, state::State},
    network::actions::NetworkAction,
};

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
                state.select_rows(indicies);
            }
            "vote" | "v" => {
                let indicies = get_indicies(indices_str.to_owned());
                state.select_rows(indicies);
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
