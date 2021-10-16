use super::word;
use super::{Deserialize, Result, Serialize, Tokenizer, Word, POS};
use druid::{Data, Lens};
use std::sync::Arc;

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