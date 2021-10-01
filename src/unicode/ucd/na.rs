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
    // ! 9FFD..9FFF is not listed but added since 14.0.0
    // 20000..2A6DD NR2 “cjk unified ideograph-”
    // ! 2A6DE..2A6DF is not listed but added since 14.0.0
    // 2A700..2B734 NR2 “cjk unified ideograph-”
    // ! 2B735..2B738 is not listed but added since 14.0.0
    // 2B740..2B81D NR2 “cjk unified ideograph-”
    // 2B820..2CEA1 NR2 “cjk unified ideograph-”
    // 2CEB0..2EBE0 NR2 “cjk unified ideograph-”
    // 30000..3134A NR2 “cjk unified ideograph-”
    if (0x3400..0x4DBF + 1).contains(&cp) ||
            (0x4E00..0x9FFC + 1).contains(&cp) ||
            (0x9FFD..0x9FFF + 1).contains(&cp) ||
            (0x20000..0x2A6DD + 1).contains(&cp) ||
            (0x2A6DE..0x2A6DF + 1).contains(&cp) ||
            (0x2A700..0x2B734 + 1).contains(&cp) ||
            (0x2B735..0x2B738 + 1).contains(&cp) ||
            (0x2B740..0x2B81D + 1).contains(&cp) ||
            (0x2B820..0x2CEA1 + 1).contains(&cp) ||
            (0x2CEB0..0x2EBE0 + 1).contains(&cp) ||
            (0x30000..0x3134A + 1).contains(&cp) {
        return format!("CJK UNIFIED IDEOGRAPH-{:04X}", cp);
    }
    // 17000..187F7 NR2 “tangut ideograph-”
    // 18D00..18D08 NR2 “tangut ideograph-”
    if (0x17000..0x187F7 + 1).contains(&cp) ||
            (0x18D00..0x18D08 + 1).contains(&cp) {
        return format!("TANGUT IDEOGRAPH-{:04X}", cp);
    }
    // 18B00..18CD5 NR2 “khitan small script character-”
    if (0x18B00..0x18CD5 + 1).contains(&cp) {
        return format!("KHITAN SMALL SCRIPT CHARACTER-{:04X}", cp);
    }
    // 1B170..1B2FB NR2 “nushu character-”
    if (0x1B170..0x1B2FB + 1).contains(&cp) {
        return format!("NUSHU CHARACTER-{:04X}", cp);
    }
    // F900..FA6D NR2 “cjk compatibility ideograph-”
    // FA70..FAD9 NR2 “cjk compatibility ideograph-”
    // 2F800..2FA1D NR2 “cjk compatibility ideograph-”
    if (0xF900..0xFA6D + 1).contains(&cp) ||
            (0xFA70..0xFAD9 + 1).contains(&cp) ||
            (0x2F800..0x2FA1D + 1).contains(&cp) {
        return format!("CJK COMPATIBILITY IDEOGRAPH-{:04X}", cp);
    }

    let found = na_table::NA_MAP.binary_search_by_key(&cp, |&(k,_)| k);
    match found {
        Ok(idx) => String::from(na_table::NA_MAP[idx].1),
        Err(_) => String::from(""),
    }
}