use super::pos::POS;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Token {
    pub lemma: String,
    pub pos: POS,
    pub text: String,
}
