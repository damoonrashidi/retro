use std::{collections::HashMap, fmt::Display};

use firestore_grpc::v1::{value::ValueType, Value};

use super::sentiment::Sentiment;

#[derive(Clone, Debug)]
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
        let (author, text, sentiment) = (&self.author, &self.text, &self.sentiment);
        let votes = if self.votes > 0 {
            format!("[+{}]", self.votes)
        } else {
            "".to_string()
        };

        write!(f, "{:<8} {sentiment} {text} {votes}", author)
    }
}

#[allow(clippy::from_over_into)]
impl Into<HashMap<String, Value>> for &Note {
    fn into(self) -> HashMap<String, Value> {
        let mut fields = HashMap::new();

        fields.insert(
            "author".to_string(),
            Value {
                value_type: Some(ValueType::StringValue(self.author.clone())),
            },
        );

        fields.insert(
            "text".to_string(),
            Value {
                value_type: Some(ValueType::StringValue(self.text.clone())),
            },
        );

        fields.insert(
            "sentiment".to_string(),
            Value {
                value_type: Some(ValueType::StringValue(self.sentiment.into())),
            },
        );

        fields.insert(
            "votes".to_string(),
            Value {
                value_type: Some(ValueType::IntegerValue(self.votes.into())),
            },
        );

        fields
    }
}

impl From<HashMap<String, Value>> for Note {
    fn from(values: HashMap<String, Value>) -> Self {
        let id = String::from("id");

        let text: String = match values.get("text").unwrap().value_type.clone().unwrap() {
            ValueType::StringValue(text) => text,
            _ => "".to_string(),
        };

        let author: String = match values.get("author").unwrap().value_type.clone().unwrap() {
            ValueType::StringValue(author) => author,
            _ => "".to_string(),
        };

        let votes: u8 = match values.get("votes").unwrap().value_type.clone().unwrap() {
            ValueType::IntegerValue(votes) => votes as u8,
            _ => 0,
        };

        let sentiment = match values.get("sentiment").unwrap().value_type.clone().unwrap() {
            ValueType::StringValue(sentiment) => match sentiment.as_str() {
                ":)" => Sentiment::Happy,
                ":(" => Sentiment::Sad,
                _ => Sentiment::Neutral,
            },
            _ => Sentiment::Neutral,
        };

        Note {
            id,
            text,
            author,
            sentiment,
            votes,
        }
    }
}
