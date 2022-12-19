use core::fmt;

use tui::style::Color;

/// The mode the retro app currently is in
#[derive(PartialEq, Eq, Debug)]
pub enum Mode {
    /// Normal mode
    Normal,

    /// Writing a new note
    Insert,

    /// Vote or unvote for a note
    Vote,

    /// Group two notes together
    Group,

    /// Filter notes based on sentiment
    Find,
}

impl Mode {
    /// Get the highlight color for the note
    pub fn get_color(&self) -> Color {
        match self {
            Self::Normal => Color::White,
            Self::Insert => Color::Blue,
            Self::Group => Color::Red,
            Self::Vote => Color::Green,
            Self::Find => Color::Yellow,
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Normal => write!(f, " NOR "),
            Self::Insert => write!(f, " INS "),
            Self::Group => write!(f, " GRP "),
            Self::Vote => write!(f, " VOT "),
            Self::Find => write!(f, " FND "),
        }
    }
}
