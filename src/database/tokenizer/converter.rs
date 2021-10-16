use super::super::{POS, POS::*};

pub fn convert_pos(pos: &str) -> POS {
    match pos {
        "*" => EMPTY,
        "代名詞" => PRON,
        "副詞" => ADV,
        "助動詞" => AUX,
        "助詞" => PART,
        "動詞" => VERB,
        "名詞" => NOUN,
        "形容詞" => ADJ,
        "形状詞" => ADJNOUN,
        "感動詞" => INTJ,
        "接尾辞" => SUFF,
        "接続詞" => CONJ,
        "接頭辞" => PREF,
        "空白" => WHITE,
        "補助記号" => SUPPLEMENTARY,
        "記号" => PUNCT,
        "連体詞" => ADN,
        _ => UNKNOWN,
    }
}
