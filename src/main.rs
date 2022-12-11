use core::fmt;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::io;
use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Table, TableState};
use tui::Terminal;
use tui_textarea::{CursorMove, Input, Key, TextArea};

use crate::note::Note;
use crate::state::State;

mod note;
mod state;

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

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut textarea = TextArea::default();
    let mut mode = Mode::Normal;
    let mut state = State::new();
    let mut table_state = TableState::default();

    loop {
        let note_table = Table::new([])
            .block(Block::default().borders(Borders::ALL).title("Notes"))
            .highlight_symbol(">> ");

        let block = Block::default().borders(Borders::ALL).title("Note");
        textarea.set_block(block);
        let (fg, bg) = mode.get_color();

        let help_text = Block::default()
            .title(format!("{}", mode))
            .style(Style::default().fg(fg).bg(bg));

        textarea.set_cursor_style(
            Style::default()
                .fg(Color::Reset)
                .add_modifier(Modifier::REVERSED)
                .add_modifier(Modifier::RAPID_BLINK),
        );

        let rect = term.size()?;

        term.draw(|f| {
            f.render_widget(textarea.widget(), Rect::new(0, 0, rect.width, 5));
            f.render_stateful_widget(
                note_table,
                Rect::new(0, 6, rect.width, rect.height - 10),
                &mut table_state,
            );
            f.render_widget(
                help_text.clone(),
                Rect::new(0, rect.height - 1, format!("{}", mode).len() as u16, 1),
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
                }
                Input {
                    key: Key::Char('v'),
                    ..
                } => {
                    mode = Mode::Vote;
                }
                Input {
                    key: Key::Enter, ..
                } => {
                    let value = textarea.lines().join("\n");
                    if !textarea.is_empty() {
                        state.add_note(Note::new(value.clone(), value.clone()));
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
                Input { key: Key::Esc, .. }
                | Input {
                    key: Key::Char('c'),
                    ctrl: true,
                    ..
                } => {
                    mode = Mode::Normal; // Back to normal mode with Esc or Ctrl+C
                }
                _ => println!("{:?}", input),
            },
            Mode::Vote => match input {
                Input { key: Key::Esc, .. }
                | Input {
                    key: Key::Char('c'),
                    ctrl: true,
                    ..
                } => {
                    mode = Mode::Normal; // Back to normal mode with Esc or Ctrl+C
                }
                _ => println!("{:?}", input),
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
