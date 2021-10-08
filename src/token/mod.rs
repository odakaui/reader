use serde::{Deserialize, Serialize};

pub use pos::POS;

mod pos;

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Token {
    pub lemma: String,
    pub pos: POS,
    pub text: String,
}


#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TokenInfo {
    pub token: Token,
    pub total_seen: i32,
    pub total_unknown: i32,
}
