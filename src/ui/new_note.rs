use tui::{
    style::{Color, Style},
    widgets::{Block, Borders},
};
use tui_textarea::TextArea;

pub fn new_note() -> TextArea<'static> {
    let mut textarea = TextArea::default();

    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("Note")
            .style(Style::default().bg(Color::DarkGray).fg(Color::White)),
    );

    textarea
}
