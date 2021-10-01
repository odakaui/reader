use crate::Token;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Line {
    pub sentence: String,
    pub tokens: Vec<Token>,
}
