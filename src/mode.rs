use core::fmt;

use tui::style::Color;

#[derive(PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
    Vote,
    Group,
    Find,
}

impl Mode {
    pub fn get_color(&self) -> (Color, Color) {
        match self {
            Self::Normal => (Color::Reset, Color::Reset),
            Self::Insert => (Color::Reset, Color::LightBlue),
            Self::Group => (Color::Reset, Color::LightRed),
            Self::Vote => (Color::Reset, Color::LightGreen),
            Self::Find => (Color::Reset, Color::LightYellow),
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
            Self::Find => write!(f, "FND"),
        }
    }
}
