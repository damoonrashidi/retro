use tui::{
    style::{Color, Modifier, Style},
    widgets::Block,
};

use crate::app::state::State;

pub fn status_bar(state: &State) -> Block<'static> {
    Block::default().title(state.mode.to_string()).style(
        Style::default()
            .fg(Color::Reset)
            .bg(state.mode.get_color())
            .add_modifier(Modifier::BOLD),
    )
}
