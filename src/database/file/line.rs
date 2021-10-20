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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() -> Result<()> {
        let original = "なんとか今日を生き延びた僕は、帰りに七海さんと一緒に生活雑貨を扱うバラエティショップへと来ていた。";

        let mut tokenizer = Tokenizer::new()?;
        let line = Line::new(&mut tokenizer, original)?;

        assert_eq!(line.sentence, original, "line.sentence {} is not equal to original {}.", line.sentence, original);
        
        let sentence = line.words.iter().fold(String::new(), |sentence, word| sentence + &word.tokens.iter().fold(String::new(), |word, token| word + &token.text));

        assert_eq!(sentence, original, "sentence from words {} is not equal to original {}.", sentence, original);

        Ok(())
    }
}
