use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
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

impl POS {
    pub fn to_int(&self) -> u8 {
        match self {
            POS::EMPTY => 0,
            POS::PRON => 1,
            POS::ADV => 2,
            POS::AUX => 3,
            POS::PART => 4,
            POS::VERB => 5,
            POS::NOUN => 6,
            POS::ADJ => 7,
            POS::ADJNOUN => 8,
            POS::INTJ => 9,
            POS::SUFF => 10,
            POS::CONJ => 11,
            POS::PREF => 12,
            POS::WHITE => 13,
            POS::SUPPLEMENTARY => 14,
            POS::PUNCT => 15,
            POS::ADN => 16,
            POS::UNKNOWN => 17,
        }
    }

    pub fn to_pos(num: u8) -> POS {
        match num {
            0 => POS::EMPTY,
            1 => POS::PRON,
            2 => POS::ADV,
            3 => POS::AUX,
            4 => POS::PART,
            5 => POS::VERB,
            6 => POS::NOUN,
            7 => POS::ADJ,
            8 => POS::ADJNOUN,
            9 => POS::INTJ,
            10 => POS::SUFF,
            11 => POS::CONJ,
            12 => POS::PREF,
            13 => POS::WHITE,
            14 => POS::SUPPLEMENTARY,
            15 => POS::PUNCT,
            16 => POS::ADN,
            17 => POS::UNKNOWN,
            _ => POS::EMPTY,
        }
    }
}

