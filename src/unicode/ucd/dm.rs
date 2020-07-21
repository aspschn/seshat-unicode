pub(self) mod dm_map;

pub(crate) fn dm(cp: u32) -> String {
    // For hangul syllables.

    let found = dm_map::DM_MAP.binary_search_by_key(&cp, |&(k,_)| k);
    match found {
        Ok(idx) => String::from(dm_map::DM_MAP[idx].1),
        Err(_) => String::from(""),
    }
}