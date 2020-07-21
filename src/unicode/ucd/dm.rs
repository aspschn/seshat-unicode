pub(self) mod dm_map;

use crate::unicode::hangul;

pub(crate) fn dm(cp: u32) -> String {
    // Hangul syllables.
    // AC00..D7AF; Hangul Syllables
    // For performance, the range of block is hard coded.
    if (0xAC00..=0xD7AF).contains(&cp) {
        let decomposed = hangul::arithmetic_decomposition_mapping(cp);
        let mut mapping = String::new();
        for code in decomposed.iter() {
            mapping.push(std::char::from_u32(*code).unwrap());
        }
        return mapping;
    }

    let found = dm_map::DM_MAP.binary_search_by_key(&cp, |&(k,_)| k);
    match found {
        Ok(idx) => String::from(dm_map::DM_MAP[idx].1),
        Err(_) => String::from(""),
    }
}