// const NA_MAP: &[(u32, &'static str)] = &[
//     (0xAC00, "GA"),
// ];
pub(self) mod na_table;

use crate::unicode::hangul;

pub(crate) fn na(cp: u32) -> String {
    // AC00..D7A3 NR1 “hangul syllable”
    if (0xAC00..0xD7A3 + 1).contains(&cp) {
        return hangul::syllable_name(cp);
    }
    // 3400..4DBF NR2 “cjk unified ideograph-”
    // 4E00..9FFC NR2 “cjk unified ideograph-”
    // 20000..2A6DD NR2 “cjk unified ideograph-”
    // 2A700..2B734 NR2 “cjk unified ideograph-”
    // 2B740..2B81D NR2 “cjk unified ideograph-”
    // 2B820..2CEA1 NR2 “cjk unified ideograph-”
    // 2CEB0..2EBE0 NR2 “cjk unified ideograph-”
    // 30000..3134A NR2 “cjk unified ideograph-”
    if (0x3400..0x4DBF + 1).contains(&cp) ||
            (0x4E00..0x9FFC + 1).contains(&cp) ||
            (0x20000..0x2A6DD + 1).contains(&cp) ||
            (0x2A700..0x2B734 + 1).contains(&cp) ||
            (0x2B740..0x2B81D + 1).contains(&cp) ||
            (0x2B820..0x2CEA1 + 1).contains(&cp) ||
            (0x2CEB0..0x2EBE0 + 1).contains(&cp) ||
            (0x30000..0x3134A + 1).contains(&cp) {
        return format!("CJK UNIFIED IDEOGRAPH-{:04X}", cp);
    }

    let found = na_table::NA_MAP.binary_search_by_key(&cp, |&(k,_)| k);
    match found {
        Ok(idx) => String::from(na_table::NA_MAP[idx].1),
        Err(_) => String::from(""),
    }
}