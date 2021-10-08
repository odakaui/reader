use druid::{Data, Lens};
use serde::{Deserialize, Serialize};

pub use pos::POS;

mod pos;

#[derive(Clone, Data, Lens, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Token {
    pub lemma: String,
    pub text: String,

    #[data(ignore)]
    pub pos: POS,
}

#[derive(Clone, Data, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TokenInfo {
    pub token: Token,
    pub total_seen: i32,
    pub total_unknown: i32,
}
