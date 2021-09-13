#[derive(Debug, PartialEq, PartialOrd)]
pub struct Word {
    pub tokens: Vec<Token>,
}

impl Word {
    pub fn new(tokens: Vec<Token>) -> Self {
        Word {
            tokens
        }
    }

    pub fn text(&self) -> String {
        &self.tokens.map(|x| x.text).collect().join("").to_string()
    }
}
