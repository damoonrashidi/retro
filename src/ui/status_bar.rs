use tui::{
    style::{Color, Style},
    widgets::Block,
};

use crate::app::state::State;

pub fn status_bar(state: &State) -> Block<'static> {
    Block::default()
        .title(state.mode.to_string())
        .style(Style::default().fg(Color::Black).bg(state.mode.get_color()))
}
