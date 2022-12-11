use std::fmt::Display;

#[allow(unused)]
#[derive(Clone, Copy, Debug)]
pub enum Sentiment {
    Happy,
    Sad,
    Neutral,
}

impl Display for Sentiment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rep = match self {
            Sentiment::Happy => ":)",
            Sentiment::Sad => ":(",
            Sentiment::Neutral => ":|",
        };

        write!(f, "{}", rep)
    }
}

#[derive(Clone, Debug)]
pub struct Note {
    pub id: String,
    pub text: String,
    pub sentiment: Sentiment,
    pub votes: u8,
}

impl Note {
    pub fn new(text: String, id: String) -> Self {
        Note {
            text,
            id,
            sentiment: Sentiment::Neutral,
            votes: 0,
        }
    }
}
