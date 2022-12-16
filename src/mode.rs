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
    Filter,
}

impl Mode {
    /// Get the highlight color for the note
    pub fn get_color(&self) -> (Color, Color) {
        match self {
            Self::Normal => (Color::Reset, Color::Reset),
            Self::Insert => (Color::Reset, Color::LightBlue),
            Self::Group => (Color::Reset, Color::LightRed),
            Self::Vote => (Color::Reset, Color::LightGreen),
            Self::Filter => (Color::Reset, Color::LightYellow),
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Normal => write!(f, "NOR"),
            Self::Insert => write!(f, "INS"),
            Self::Group => write!(f, "GRP"),
            Self::Vote => write!(f, "VOT"),
            Self::Filter => write!(f, "FND"),
        }
    }
}
