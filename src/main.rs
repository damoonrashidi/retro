mod cli;
mod note;
mod state;

use core::fmt;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::io;
use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem};
use tui::Terminal;
use tui_textarea::{CursorMove, Input, Key, TextArea};

use crate::cli::RetroArgs;
use crate::note::Note;
use crate::state::State;

#[derive(PartialEq, Eq)]
enum Mode {
    Normal,
    Insert,
    Vote,
    Group,
}

impl Mode {
    fn get_color(&self) -> (Color, Color) {
        match self {
            Self::Normal => (Color::Reset, Color::Reset),
            Self::Insert => (Color::Reset, Color::LightBlue),
            Self::Group => (Color::Reset, Color::LightRed),
            Self::Vote => (Color::Reset, Color::LightGreen),
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Normal => write!(f, "NORMAL"),
            Self::Insert => write!(f, "INSERT"),
            Self::Group => write!(f, "GROUP"),
            Self::Vote => write!(f, "VOTE"),
        }
    }
}

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let args = RetroArgs::new();

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut textarea = TextArea::default();
    let mut mode = Mode::Normal;
    let mut state = State::new();

    loop {
        let note_list = List::new(
            state
                .notes_as_list()
                .into_iter()
                .enumerate()
                .map(|(i, note)| {
                    if let Some(selected_row) = state.selected_row {
                        if i == selected_row && (mode == Mode::Group || mode == Mode::Vote) {
                            return build_list_item(&note, &state, &mode, selected_row == i);
                        }
                    }
                    build_list_item(&note, &state, &mode, false)
                })
                .collect::<Vec<ListItem>>(),
        )
        .block(Block::default().borders(Borders::ALL).title("Notes"));

        let block = Block::default().borders(Borders::ALL).title("Note");
        textarea.set_block(block);
        let (fg, bg) = mode.get_color();

        let help_text = Block::default()
            .title(format!("{}", mode))
            .style(Style::default().fg(fg).bg(bg));

        let room_info = Block::default().title(format!("{} @ {}", args.display_name, args.room));

        let participants_info =
            Block::default().title(format!("{} participants", state.participants.len()));

        textarea.set_cursor_style(
            Style::default()
                .fg(Color::Reset)
                .add_modifier(Modifier::REVERSED)
                .add_modifier(Modifier::RAPID_BLINK),
        );

        let rect = term.size()?;

        term.draw(|f| {
            f.render_widget(textarea.widget(), Rect::new(0, 0, rect.width, 5));
            f.render_widget(note_list, Rect::new(0, 6, rect.width, rect.height - 7));
            f.render_widget(
                help_text,
                Rect::new(0, rect.height - 1, format!("{}", mode).len() as u16, 1),
            );
            f.render_widget(
                room_info,
                Rect::new(format!("{}", mode).len() as u16 + 1, rect.height - 1, 50, 1),
            );
            f.render_widget(
                participants_info,
                Rect::new(rect.width - 14, rect.height - 1, 14, 1),
            );
        })?;

        let input = crossterm::event::read()?.into();

        match mode {
            Mode::Normal => match input {
                Input {
                    key: Key::Char('i'),
                    ..
                } => {
                    mode = Mode::Insert;
                }
                Input {
                    key: Key::Char('g'),
                    ..
                } => {
                    mode = Mode::Group;
                    state.select_row(0);
                }
                Input {
                    key: Key::Char('v'),
                    ..
                } => {
                    mode = Mode::Vote;
                    state.select_row(0);
                }
                Input {
                    key: Key::Enter, ..
                } => {
                    let value = textarea.lines().join("\n");
                    if !textarea.is_empty() {
                        state.add_note(Note::new(args.display_name.clone(), value.clone()));
                    }
                }
                Input {
                    key: Key::Char('q'),
                    ..
                } => break,

                _ => {}
            },
            Mode::Insert => match input {
                Input { key: Key::Esc, .. } => {
                    mode = Mode::Normal; // Back to normal mode with Esc or Ctrl+C
                }
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
                input => {
                    textarea.input(input); // Use default key mappings in insert mode
                }
            },
            Mode::Group => match input {
                Input { key: Key::Esc, .. } => {
                    mode = Mode::Normal;
                    state.deselect_row()
                }
                Input { key: Key::Up, .. } => {
                    if let Some(selected_row) = state.selected_row {
                        let list_len = state.notes_as_list().len();
                        state.select_row((selected_row.max(1) - 1).min(list_len - 1).max(0));
                    } else {
                        state.select_row(0);
                    }
                }
                Input { key: Key::Down, .. } => {
                    if let Some(selected_row) = state.selected_row {
                        let list_len = state.notes_as_list().len();
                        state.select_row((selected_row + 1).min(list_len - 1).max(0));
                    } else {
                        state.select_row(0);
                    }
                }
                _ => {}
            },
            Mode::Vote => match input {
                Input { key: Key::Esc, .. } => {
                    mode = Mode::Normal;
                    state.deselect_row()
                }
                Input { key: Key::Up, .. } => {
                    if let Some(selected_row) = state.selected_row {
                        let list_len = state.notes_as_list().len();
                        state.select_row((selected_row.max(1) - 1).min(list_len - 1).max(0));
                    } else {
                        state.select_row(0);
                    }
                }
                Input { key: Key::Down, .. } => {
                    if let Some(selected_row) = state.selected_row {
                        let list_len = state.notes_as_list().len();
                        state.select_row((selected_row + 1).min(list_len - 1).max(0));
                    } else {
                        state.select_row(0);
                    }
                }
                _ => {}
            },
        }
    }

    disable_raw_mode()?;
    crossterm::execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    term.show_cursor()?;

    println!("Lines: {:?}", textarea.lines());

    Ok(())
}

fn build_list_item<'a>(note: &Note, state: &State, mode: &Mode, is_selected: bool) -> ListItem<'a> {
    ListItem::new(format!(">> {}: {}", note.author, note.text)).style(
        Style::default()
            .fg(Color::Black)
            .bg(Color::White)
            .add_modifier(Modifier::BOLD),
    )
}
