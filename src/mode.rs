use core::fmt;

use tui::style::Color;

#[derive(PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
    Vote,
    Group,
}

impl Mode {
    pub fn get_color(&self) -> (Color, Color) {
        match self {
            Self::Normal => (Color::Reset, Color::Reset),
            Self::Insert => (Color::Reset, Color::LightBlue),
            Self::Group => (Color::Reset, Color::LightRed),
            Self::Vote => (Color::Reset, Color::LightGreen),
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Normal => write!(f, "NORMAL"),
            Self::Insert => write!(f, "INSERT"),
            Self::Group => write!(f, "GROUP"),
            Self::Vote => write!(f, "VOTE"),
        }
    }
}
