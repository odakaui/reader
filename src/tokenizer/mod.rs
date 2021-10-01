use anyhow::{Context, Result};
use lindera;

use crate::token::{Token, POS};

pub mod converter;

pub struct Tokenizer {
    tokenizer: lindera::tokenizer::Tokenizer,
}

impl Tokenizer {
    pub fn new() -> Result<Self> {
        Ok(Tokenizer {
            tokenizer: lindera::tokenizer::Tokenizer::new()
                .context("Failed to initialize the tokenizer")?,
        })
    }

    pub fn tokenize(&mut self, text: &str) -> Result<Vec<Token>> {
        Ok(Tokenizer::convert_tokens(
            self.tokenizer
                .tokenize(text)
                .context("Failed to tokenize the text")?,
        ))
    }

    fn convert_tokens(tokens: Vec<lindera::tokenizer::Token>) -> Vec<Token> {
        tokens
            .iter()
            .map(|x| Token {
                lemma: if x.detail.len() > 1 {
                    x.detail[6].to_string()
                } else {
                    x.text.to_string()
                },
                pos: converter::convert_pos(x.detail[0].as_ref()),
                text: x.text.to_string(),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::Tokenizer;
    use super::{Token, POS::*};

    #[test]
    fn test_convert_tokens() -> Result<()> {
        // The setup for the test
        let line = r#"『愛妻弁当だー？』"#;
        let mut tokenizer = Tokenizer::new()?;

        let expected_results = vec![
            Token {
                lemma: "『".to_string(),
                pos: PUNCT,
                text: "『".to_string(),
            },
            Token {
                lemma: "愛妻".to_string(),
                pos: NOUN,
                text: "愛妻".to_string(),
            },
            Token {
                lemma: "弁当".to_string(),
                pos: NOUN,
                text: "弁当".to_string(),
            },
            Token {
                lemma: "だ".to_string(),
                pos: AUX,
                text: "だ".to_string(),
            },
            Token {
                lemma: "ー".to_string(),
                pos: UNKNOWN,
                text: "ー".to_string(),
            },
            Token {
                lemma: "？".to_string(),
                pos: PUNCT,
                text: "？".to_string(),
            },
            Token {
                lemma: "』".to_string(),
                pos: PUNCT,
                text: "』".to_string(),
            },
        ];

        let actual_results = tokenizer.tokenize(line)?;

        assert_eq!(expected_results, actual_results);

        Ok(())
    }
}
