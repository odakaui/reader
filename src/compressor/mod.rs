use crate::{Article, State, Token, POS::*};

pub use word::Word;

mod word;

trait Rule {
    fn test(token: &Token) -> bool;
}

pub fn compress(article: &Article, state: &State) -> Word {
    let position = state.position.as_ref().expect("Failed to unwrap position");
    let words = compress_line(article, state);

    words[position.index].clone()
}

pub fn compress_line(article: &Article, state: &State) -> Vec<Word> {
    let position = state.position.as_ref().expect("Failed to unwrap position");
    let line = &article.lines[position.line];
    let tokens = &line.tokens;

    let mut stored: Vec<Token> = Vec::new();
    let mut words: Vec<Word> = Vec::new();

    for token in tokens.iter() {
        if !is_legal(token, &stored) {
            let word = Word {
                text: string_from_tokens(&stored),
                tokens: stored.clone(),
            };

            words.push(word);

            stored.clear();
        }
        stored.push(token.clone());
    }

    words
}

fn string_from_tokens(tokens: &[Token]) -> String {
    let mut text = String::new();

    for token in tokens {
        text.push_str(&token.text);
    }

    text
}

