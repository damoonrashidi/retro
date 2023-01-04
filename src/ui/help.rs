use tui::{
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{mode::Mode, state::State};

pub fn help(state: &State) -> Paragraph<'static> {
    let shortcuts: &'static str = match state.mode {
        Mode::Normal => {
            r#"
?  Show/hide help
________________
:  command mode
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
        Mode::Command => {
            r#"
ESC  Normal mode
________________
 g   1 2 .. n
 u   1 2 .. n upvote
 d   1 2 .. n downvote
 f   query
"#
        }
    };

    Paragraph::new(shortcuts)
        .block(
            Block::default()
                .title(format!("Help ({})", state.mode))
                .borders(Borders::all()),
        )
        .style(Style::default().bg(Color::White).fg(Color::Black))
}
