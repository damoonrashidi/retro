use std::fmt::Display;

#[allow(unused)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// The sentiment for a given note
pub enum Sentiment {
    /// Positive (or continue doing)
    Happy,

    /// Negative (or stop doing)
    Sad,

    /// Neutral (no opinion)
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

impl From<Sentiment> for String {
    fn from(sentiment: Sentiment) -> Self {
        format!("{}", sentiment)
    }
}
