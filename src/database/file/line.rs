use super::{Result, Tokenizer, Word, Deserialize, Serialize, POS};
use super::word;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Line {
    pub sentence: String,
    pub words: Vec<Word>,
}

impl Line {
    pub fn new(tokenizer: &mut Tokenizer, text: &str) -> Result<Self> {
        let sentence = text.to_string();
        let tokens = tokenizer.tokenize(text)?; 
        let words = word::to_words(&tokens);

        Ok(Line {
            sentence,
            words,
        })
    }
}
