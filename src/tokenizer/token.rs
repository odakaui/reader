use super::pos::POS;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Token {
    pub lemma: String,
    pub pos: POS,
    pub sentence: String,
    pub text: String,
}
