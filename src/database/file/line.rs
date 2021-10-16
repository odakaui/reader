use super::word;
use super::{Deserialize, Result, Serialize, Tokenizer, Word, POS};
use std::sync::Arc;
use druid::{Data, Lens};

#[derive(Clone, Debug, PartialEq, Data, Lens, Deserialize, Serialize)]
pub struct Line {
    pub sentence: String,
    pub words: Arc<Vec<Word>>,
}

impl Line {
    pub fn new(tokenizer: &mut Tokenizer, text: &str) -> Result<Self> {
        let sentence = text.to_string();
        let tokens = tokenizer.tokenize(text)?;
        let words = Arc::new(word::to_words(&tokens));

        Ok(Line { sentence, words })
    }
}
