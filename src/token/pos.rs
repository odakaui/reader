use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum POS {
    EMPTY,
    PRON,
    ADV,
    AUX,
    PART,
    VERB,
    NOUN,
    ADJ,
    ADJNOUN,
    INTJ,
    SUFF,
    CONJ,
    PREF,
    WHITE,
    SUPPLEMENTARY,
    PUNCT,
    ADN,
    UNKNOWN,
}
