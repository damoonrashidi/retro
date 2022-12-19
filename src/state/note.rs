use std::fmt::Display;

#[allow(unused)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
/// A retro Note
pub struct Note {
    /// Used for storing notes
    pub id: String,

    /// The actual text of the note
    pub text: String,

    /// who wrote the note, will be overwritten for grouped notes
    pub author: String,

    /// If the note was positive, negative or neutral
    pub sentiment: Sentiment,

    /// How many votes the note has received
    pub votes: u8,
}

impl Note {
    /// Create a new note, if the note contains a happy smiley it will be
    /// tagged with a happy sentiment, sad smiley will be negative and a
    /// neutral smiley (or no smiley at all) will yield a neutral sentiment.
    pub fn new(author: String, text: String) -> Self {
        Note {
            text: text.replace(":(", "").replace(":)", ""),
            author,
            id: text.clone(),
            sentiment: Self::get_sentiment(&text),
            votes: 0,
        }
    }

    /// Get the sentiment for the note based on what emoji was used.
    fn get_sentiment(text: &str) -> Sentiment {
        if text.contains(":)") {
            return Sentiment::Happy;
        }
        if text.contains(":(") {
            return Sentiment::Sad;
        }
        Sentiment::Neutral
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.author, self.text)
    }
}
