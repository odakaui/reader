use crate::Token;

#[derive(Debug)]
pub struct Word {
    pub text: String,
    pub tokens: Vec<Token>,
}

impl Word {
    pub fn new(text: String, tokens: Vec<Token>) -> Self {
        Word { text, tokens }
    }
}
