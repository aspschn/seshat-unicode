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

// Returns reversed decomposition map as u32.
// If not composed, returns 0.
pub(crate) fn rdm(s: &str) -> u32 {
    // For hangul.
    let chars = s.chars().collect::<Vec<char>>();
    if chars.len() == 2
        && ((0x1100..=0x1112).contains(&(chars[0] as u32))
            && (0x1161..=0x1175).contains(&(chars[1] as u32)))
    {
        let (l, v) = (chars[0] as u32, chars[1] as u32);
        let composed
            = hangul::arithmetic_primary_composite_mapping(l, v, 0);
        return composed;
    }
    if chars.len() == 3
        && ((0x1100..=0x1112).contains(&(chars[0] as u32))
            && (0x1161..=0x1175).contains(&(chars[1] as u32))
            && (0x11A8..0x11C2).contains(&(chars[2] as u32)))
    {
        let (l, v, t) = (chars[0] as u32, chars[1] as u32, chars[2] as u32);
        let composed
            = hangul::arithmetic_primary_composite_mapping(l, v, t);
        return composed;
    }

    let found = dm_map::RDM_MAP.binary_search_by_key(&s, |&(k,_)| k);
    match found {
        Ok(idx) => dm_map::RDM_MAP[idx].1,
        Err(_) => 0,
    }
}