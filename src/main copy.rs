use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use retro::cli::RetroArgs;
use retro::mode::Mode;
use retro::note::{Note, Sentiment};
use retro::state::State;
use std::path::Path;
use std::{fs, io};
use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui::Terminal;
use tui_textarea::{CursorMove, Input, Key, TextArea};

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let args = RetroArgs::new();

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut textarea = TextArea::default();
    let mut state = State::new();

    loop {
        let note_list = List::new(
            state
                .notes_as_list()
                .into_iter()
                .enumerate()
                .flat_map(|(i, note)| {
                    if let Some(selected_row) = state.selected_row {
                        if i == selected_row
                            && (state.mode == Mode::Group || state.mode == Mode::Vote)
                        {
                            return vec![
                                build_list_item(&note, &i, &state.mode, true),
                                ListItem::new("----------------"),
                            ];
                        }
                    }
                    vec![
                        build_list_item(&note, &i, &state.mode, false),
                        ListItem::new("----------------"),
                    ]
                })
                .collect::<Vec<ListItem<'_>>>(),
        )
        .block(Block::default().borders(Borders::ALL).title("Notes"));

        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title("Note")
                .style(Style::default().bg(Color::DarkGray).fg(Color::White)),
        );

        let help_text = {
            let (fg, bg) = state.mode.get_color();
            Block::default()
                .title(format!("{}", state.mode))
                .style(Style::default().fg(fg).bg(bg))
        };

        let room_info = Block::default().title(format!("{} @ {}", args.display_name, args.room));

        let participants_info =
            Block::default().title(format!("{} participants", state.participants.len()));

        let rect = term.size()?;

        term.draw(|f| {
            f.render_widget(note_list, Rect::new(0, 0, rect.width, rect.height - 1));
            if state.mode == Mode::Insert {
                f.render_widget(
                    textarea.widget(),
                    Rect::new(rect.width / 2 - 20, rect.height / 2 - 10, 40, 20),
                );
            }
            f.render_widget(
                help_text,
                Rect::new(
                    0,
                    rect.height - 1,
                    format!("{}", state.mode).len() as u16,
                    1,
                ),
            );
            f.render_widget(
                room_info,
                Rect::new(
                    format!("{}", state.mode).len() as u16 + 1,
                    rect.height - 1,
                    50,
                    1,
                ),
            );
            f.render_widget(
                participants_info,
                Rect::new(rect.width - 14, rect.height - 1, 14, 1),
            );
            if let Some(filter) = state.filter {
                f.render_widget(
                    Block::default()
                        .title(filter.to_string())
                        .style(Style::default().fg(Color::Black).bg(Color::LightGreen)),
                    Rect::new(rect.width / 2 - 3, rect.height - 1, 7, 1),
                );
            }
            if state.show_help {
                f.render_widget(
                    help_box(&state.mode),
                    Rect::new(rect.width - 40, rect.height - 20, 35, 15),
                );
            }
        })?;

        let input = crossterm::event::read()?.into();

        match state.mode {
            Mode::Normal => match input {
                Input {
                    key: Key::Char('i'),
                    ..
                } => {
                    state.mode = Mode::Insert;
                }
                Input {
                    key: Key::Char('g'),
                    ..
                } => {
                    state.mode = Mode::Group;
                    state.select_row(0);
                }
                Input {
                    key: Key::Char('f'),
                    ..
                } => {
                    state.mode = Mode::Find;
                }
                Input {
                    key: Key::Char('v'),
                    ..
                } => {
                    state.mode = Mode::Vote;
                    state.select_row(0);
                }
                Input {
                    key: Key::Char('e'),
                    ..
                } => {
                    let notes = state
                        .notes_as_list()
                        .iter()
                        .map(|note| format!("{},{},{}", note.author, note.sentiment, note.text))
                        .collect::<Vec<String>>()
                        .join("\n");

                    let _ = export_retro(&format!("{}.csv", &args.room), &notes);
                }
                Input {
                    key: Key::Char('?'),
                    ..
                } => {
                    state.show_help = !state.show_help;
                }
                Input {
                    key: Key::Char('q'),
                    ..
                } => break,

                _ => {}
            },
            Mode::Find => match input {
                Input { key: Key::Esc, .. } => {
                    state.reset_filter();
                    state.mode = Mode::Normal
                }
                Input {
                    key: Key::Char(')'),
                    ..
                } => {
                    state.set_filter(Sentiment::Happy);
                }
                Input {
                    key: Key::Char('('),
                    ..
                } => {
                    state.set_filter(Sentiment::Sad);
                }
                Input {
                    key: Key::Char('|'),
                    ..
                } => {
                    state.set_filter(Sentiment::Neutral);
                }
                _ => {}
            },
            Mode::Insert => match input {
                Input { key: Key::Esc, .. } => {
                    state.mode = Mode::Normal;
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
                    key: Key::Enter, ..
                } => {
                    if !textarea.is_empty() {
                        let value = textarea.lines().join("\n");
                        state.add_note(Note::new(args.display_name.clone(), value.clone()));
                    }
                    textarea.delete_line_by_head();
                }
                Input {
                    key: Key::Right,
                    alt: true,
                    ..
                } => textarea.move_cursor(CursorMove::WordForward),
                input => {
                    textarea.input(input);
                }
            },
            Mode::Group => match input {
                Input { key: Key::Esc, .. } => {
                    state.mode = Mode::Normal;
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
                Input {
                    key: Key::Char(chr),
                    ..
                } => {
                    if let (Some(index), true) = (state.selected_row, ('0'..='9').contains(&chr)) {
                        let notes = state.notes_as_list();
                        let selected_id = notes.get(index).unwrap().id.clone();
                        if let Some(merge_note) =
                            notes.get(chr.to_string().parse::<usize>().unwrap())
                        {
                            let merge_id = merge_note.id.clone();
                            let _ = state.group_notes(&merge_id, &selected_id);
                            state.remove_note(&selected_id);
                            state.deselect_row();
                        }
                    }
                }
                _ => {}
            },
            Mode::Vote => match input {
                Input { key: Key::Esc, .. } => {
                    state.mode = Mode::Normal;
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
                Input {
                    key: Key::Enter, ..
                } => {
                    if let Some(index) = state.selected_row {
                        let note_id = state.notes_as_list().get(index).unwrap().id.clone();
                        state.upvote(note_id);
                    }
                }
                Input {
                    key: Key::Backspace,
                    ..
                } => {
                    if let Some(index) = state.selected_row {
                        let note_id = state.notes_as_list().get(index).unwrap().id.clone();
                        state.downvote(note_id);
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

    Ok(())
}

fn export_retro(name: &String, notes: &String) -> Result<(), io::Error> {
    let path = Path::new(name);
    fs::write(path, notes)
}

fn build_list_item<'a>(note: &Note, index: &usize, mode: &Mode, is_selected: bool) -> ListItem<'a> {
    let style = if is_selected && (mode == &Mode::Group || mode == &Mode::Vote) {
        Style::default()
            .fg(Color::Black)
            .bg(match mode {
                Mode::Vote => Color::Green,
                Mode::Group => Color::Red,
                _ => unreachable!(),
            })
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let votes = if note.votes > 0 {
        format!(" [+{}]", note.votes)
    } else {
        "".to_string()
    };

    let display = match mode {
        Mode::Vote => format!(
            "{}{}: {} {}",
            if is_selected { ">>" } else { "" },
            note.author,
            note.text,
            votes
        ),
        Mode::Group => format!(
            "{}{} {}: {} {}",
            if is_selected {
                "".to_string()
            } else {
                format!("{}. ", index)
            },
            if is_selected { ">>" } else { "" },
            note.author,
            note.text,
            votes
        ),
        _ => format!("{}: {} {}", note.author, note.text, votes),
    };

    ListItem::new(display).style(style)
}

fn help_box(mode: &Mode) -> Paragraph<'static> {
    let shortcuts: &'static str = match mode {
        Mode::Normal => {
            r#"
?  Show/hide help
________________
i  insert mode
g  group mode
v  vote mode
________________
e  export to csv
q  quit retro
"#
        }
        Mode::Insert => {
            r#"
 ?   Show/hide help
ESC  Normal mode
________________
↵    Create note
"#
        }
        Mode::Find => {
            r#"
 ?   Show/hide help
ESC  Normal mode
________________
 )   Show happy notes 
 (   Show sad notes 
 |   Show neutral notes 
"#
        }
        Mode::Vote => {
            r#"
 ?   Show/hide help
ESC  Normal mode
________________
 ↑   Select Previous
 ↓   Select next
 ↵   Vote up selected
 ⌫   Unvote selected
"#
        }

        Mode::Group => {
            r#"
 ?   Show/hide help
ESC  Normal mode
________________
 ↑   Select Previous
 ↓   Select next
0..9  Group selected with number
"#
        }
    };

    Paragraph::new(shortcuts)
        .block(
            Block::default()
                .title(format!("Help ({mode})"))
                .borders(Borders::all()),
        )
        .style(Style::default().bg(Color::White).fg(Color::Black))
}
