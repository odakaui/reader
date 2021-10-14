use super::{Deserialize, Serialize, Token};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Line {
    pub sentence: String,
    pub tokens: Vec<Token>,
}

