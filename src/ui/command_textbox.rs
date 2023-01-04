use tui::{
    style::{Color, Style},
    widgets::{Block, Borders},
};
use tui_textarea::TextArea;

pub fn command_textbox() -> TextArea<'static> {
    let mut textarea = TextArea::default();

    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("Command")
            .style(Style::default().bg(Color::Reset).fg(Color::LightRed)),
    );

    textarea
}
