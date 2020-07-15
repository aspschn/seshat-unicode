// Constants from Unicode 3.12 (Version 13.0.0)
const S_BASE: u32 = 0xAC00;
const L_BASE: u32 = 0x1100;
const V_BASE: u32 = 0x1161;
const T_BASE: u32 = 0x11A7;

const L_COUNT: u32 = 19;
const V_COUNT: u32 = 21;
const T_COUNT: u32 = 28;
const N_COUNT: u32 = 588; // (VCount * TCount)
const S_COUNT: u32 = 11172; // (LCount * NCount)

pub(crate) fn decompose(s: u32) -> Vec<u32> {
    let s_index = s - S_BASE;
    let l_index = s_index / N_COUNT;
    let v_index = (s_index % N_COUNT) / T_COUNT;
    let t_index = s_index % T_COUNT;

    let l_part = L_BASE + l_index;
    let v_part = V_BASE + v_index;
    let t_part = T_BASE + t_index;

    if t_index > 0 {
        vec![l_part, v_part, t_part]
    } else {
        vec![l_part, v_part]
    }
}

pub(crate) fn syllable_name(cp: u32) -> String {
    let decomposed = decompose(cp);
    let mut name = String::from("HANGUL SYLLABLE ");
    for c in decomposed {
        name.push_str(super::ucd::jsn::jsn(c));
    }

    name
}
