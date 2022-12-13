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
    pub author: String,
    pub sentiment: Sentiment,
    pub votes: u8,
}

impl Note {
    pub fn new(author: String, text: String) -> Self {
        Note {
            text: text.clone().replace(":(", "").replace(":)", ""),
            author,
            id: text.clone(),
            sentiment: Self::get_sentiment(&text),
            votes: 0,
        }
    }

    fn get_sentiment(text: &String) -> Sentiment {
        if text.contains(":)") {
            return Sentiment::Happy;
        }
        if text.contains(":(") {
            return Sentiment::Sad;
        }
        Sentiment::Neutral
    }
}
