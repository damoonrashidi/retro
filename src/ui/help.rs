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
                .title(format!("Help ({})", state.mode))
                .borders(Borders::all()),
        )
        .style(Style::default().bg(Color::White).fg(Color::Black))
}
