use core::fmt;

use tui::style::Color;

/// The mode the retro app currently is in
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Mode {
    /// Normal mode
    Normal,

    /// Writing a new note
    Insert,

    /// Command mode - Group / vote / find
    Command,
}

impl Mode {
    /// Get the highlight color for the note
    pub fn get_color(&self) -> Color {
        match self {
            Self::Normal => Color::White,
            Self::Insert => Color::Blue,
            Self::Command => Color::Red,
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Normal => write!(f, " NOR "),
            Self::Insert => write!(f, " INS "),
            Self::Command => write!(f, " CMD "),
        }
    }
}
