use super::{Token, POS::*};
use druid::{Data, Lens};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Data, Deserialize, Lens, Serialize)]
pub struct Word {
    pub text: String,
    pub tokens: Arc<Vec<Token>>,
}

impl Word {
    pub fn empty() -> Self {
        Word {
            text: "".to_string(),
            tokens: Arc::new(Vec::new()),
        }
    }
}

pub fn to_words(tokens: &Vec<Token>) -> Vec<Word> {
    let mut stored: Vec<Token> = Vec::new();
    let mut words: Vec<Word> = Vec::new();

    for token in tokens.iter() {
        if !is_legal(token, &stored) {
            let word = Word {
                text: to_string(&stored),
                tokens: Arc::new(stored.clone()),
            };

            words.push(word);

            stored.clear();
        }
        stored.push(token.clone());
    }

    words
}

pub fn to_string(tokens: &Vec<Token>) -> String {
    tokens
        .iter()
        .fold(String::new(), |_, token| token.text.clone())
}

fn is_legal(token: &Token, stored: &[Token]) -> bool {
    is_punctuation(token)
        || (is_term(token) && term_legal(stored))
        || (is_filler(token) && filler_legal(stored))
        || (is_unknown(token) && unknown_legal(stored))
}

fn term_legal(stored: &[Token]) -> bool {
    !(contains_term(stored) || contains_all(stored))
}

fn filler_legal(stored: &[Token]) -> bool {
    !contains_all(stored)
}

fn unknown_legal(stored: &[Token]) -> bool {
    !contains_all(stored)
}

fn contains_all(tokens: &[Token]) -> bool {
    if tokens.is_empty() {
        return false;
    }

    is_punctuation(
        tokens
            .last()
            .expect("Failed to retrieve the last element of tokens"),
    ) && (contains_term(tokens) || contains_filler(tokens) || contains_unknown(tokens))
}

fn contains_punctuation(tokens: &[Token]) -> bool {
    for token in tokens {
        if is_punctuation(token) {
            return true;
        }
    }

    false
}

fn contains_other(tokens: &[Token]) -> bool {
    for token in tokens {
        if is_term(token) || is_unknown(token) || is_filler(token) {
            return true;
        }
    }

    false
}

fn is_punctuation(token: &Token) -> bool {
    token.pos == PUNCT
}

fn contains_term(tokens: &[Token]) -> bool {
    for token in tokens {
        if is_term(token) {
            return true;
        }
    }

    false
}

fn is_term(token: &Token) -> bool {
    let pos = [VERB, NOUN, ADJ, ADJNOUN];

    pos.contains(&token.pos)
}

fn contains_unknown(tokens: &[Token]) -> bool {
    for token in tokens {
        if is_unknown(token) {
            return true;
        }
    }

    false
}

fn is_unknown(token: &Token) -> bool {
    token.pos == UNKNOWN
}

fn contains_filler(tokens: &[Token]) -> bool {
    for token in tokens {
        if is_filler(token) {
            return true;
        }
    }

    false
}

fn is_filler(token: &Token) -> bool {
    !(is_term(token) || is_punctuation(token) || is_unknown(token))
}
