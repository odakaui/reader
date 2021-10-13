use super::Token;

pub struct Word {
    pub tokens: Vec<Token>,
}

impl Word {
    pub fn text(&self) -> String {
        let mut text = String::new();
        for token in self.tokens.iter() {
            text.push_str(&token.text);
        }

        text
    }
}
