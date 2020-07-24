use crate::unicode::CodePoint;
use crate::unicode::Ucd;
use crate::unicode::props::Gc;
use crate::unicode::props::Dt;

pub(crate) fn starter(cp: u32) -> bool {
    let cp = CodePoint::new(cp).unwrap();
    let cp_ccc = cp.ccc();
    let cp_gc = cp.gc();

    if cp_gc == Gc::Mn {
        if cp_ccc as u8 == 0 {
            return true;
        } else if cp_ccc as u8 > 0 {
            return false;
        }
    }

    if cp_gc == Gc::Mc {
        if cp_ccc as u8 == 0 {
            return true;
        } else if cp_ccc as u8 > 0 {
            return false;
        }
    }

    if cp_gc == Gc::Me {
        if cp_ccc as u8 == 0 {
            return true;
        }
    }

    if cp_ccc as u8 == 0 {
        return true;
    }

    false
}

pub(super) fn reorderable_pair(pair: (u32, u32)) -> bool {
    let a = CodePoint::new(pair.0).unwrap();
    let b = CodePoint::new(pair.1).unwrap();
    a.ccc() as u8 > b.ccc() as u8 && b.ccc() as u8 > 0
}

pub(super) fn canonical_ordering(sequence: &mut Vec<char>) {
    // This ordering algorithm acts like a bubble sort.
    if sequence.len() == 0 {
        return ();
    }

    let mut last_idx = sequence.len() - 1;
    while last_idx > 0 {
        for i in 0..=last_idx {
            if i + 1 == last_idx + 1 {
                continue;
            }
            if reorderable_pair((sequence[i] as u32, sequence[i + 1] as u32)) {
                sequence.swap(i, i + 1);
            }
        }
        last_idx -= 1;
    }
}

pub(super) fn compatibility_decomposition(s: &Vec<char>) -> Vec<char> {
    let mut count = 0;
    let mut decomposed = vec![];
    for ch in s.iter() {
        if ch.dm() == "" {
            decomposed.push(*ch);
        } else {
            match ch.dt() {
                Dt::None => {
                    decomposed.push(*ch);
                }
                _ => {
                    for decomposed_char in ch.dm().chars() {
                        decomposed.push(decomposed_char);
                        count += 1;
                    }
                }
            }
        }
    }
    if count == 0 {
        return decomposed;
    }
    compatibility_decomposition(&decomposed)
}

pub(super) fn canonical_decomposition(s: Vec<char>) -> Vec<char>{
    let mut count = 0;
    let mut decomposed = vec![];
    for ch in s.iter() {
        if ch.dm() == "" {
            decomposed.push(*ch);
        } else {
            if ch.dt() == Dt::Can {
                for decomposed_char in ch.dm().chars() {
                    decomposed.push(decomposed_char);
                    count += 1;
                }
            } else {
                decomposed.push(*ch);
            }
        }
    }
    if count == 0 {
        return decomposed;
    }
    canonical_decomposition(decomposed)
}

pub(crate) fn singleton_decomposition(cp: u32) -> bool {
    let code_point = CodePoint::new(cp).unwrap();
    // Default value (the code point itself) is not singletons.
    let mut self_char = String::new();
    self_char.push(std::char::from_u32(code_point.to_u32()).unwrap());
    if code_point.dm() == self_char {
        return false;
    }
    // Single character with canonical decomposition is singleton.
    if code_point.dm().chars().collect::<Vec<char>>().len() == 1 {
        if code_point.dt() != Dt::Can {
            return false;
        }
        return true;
    }

    false
}

// D111     Non-starter decomposition: An expanding canonical decomposition which is not
//          a starter decomposition.
pub(crate) fn non_starter_decomposition(cp: u32) -> bool {
    let code_point = CodePoint::new(cp).unwrap();
    let decomposed = canonical_decomposition(
        vec![std::char::from_u32(code_point.to_u32()).unwrap()]
    );
    if decomposed.len() > 1 && !starter(decomposed[0] as u32) {
        return true;
    }

    false
}

// D114
fn primary_composite(cp: u32) -> bool {
    let code_point = CodePoint::new(cp).unwrap();
    if code_point.dt() == Dt::Can && !code_point.comp_ex() {
        return true;
    }

    false
}

// D115
fn blocked(sequence: &[char]) -> bool {
    if sequence[0].ccc() as u8 != 0 {
        return false;
    }
    let first_i = 0;
    let last_i = sequence.len() - 1;
    if last_i - 1 == first_i {
        return false;
    }
    if sequence[last_i - 1].ccc() as u8 == 0
        || sequence[last_i - 1].ccc() as u8 > sequence[last_i].ccc() as u8
    {
        return true;
    }

    false
}

// D117
pub(super) fn canonical_composition(s: &mut Vec<char>) {
    if s.len() == 1 {
        return ();
    }

    let mut offset = 1;
    while offset < s.len() {
        let i = offset;
        let mut back_i = i - 1;
        // R1 - Seek back (left) in the coded character sequence from the
        // character C to find the last Starter L preceding C in the
        // character sequence.
        while back_i != 0 && !starter(s[back_i] as u32) {
            back_i -= 1;
        }
        // R2 - If there is such an L, and C is not blocked from L, and there
        // exists a Primary Composite P which is canonically equivalent to the
        // sequence <L, C>, then replace L by P in the sequence and delete C
        // from the sequence.
        let mut lc = String::new();
        lc.push(s[back_i]);
        lc.push(s[i]);

        let mapping = crate::unicode::ucd::dm::rdm(&lc);
        let is_primary_composite = primary_composite(mapping);
        if (starter(s[back_i] as u32)
            && !blocked(&s[back_i..=i]))
            && (mapping != 0x0 && is_primary_composite)
        {
            s[back_i] = std::char::from_u32(mapping).unwrap();
            s.remove(i);
            offset -= 1;
        }
        offset += 1;
    }
}

pub(crate) fn nfd(s: &str) -> Vec<char> {
    let seq = s.chars().collect::<Vec<char>>();
    let mut seq = canonical_decomposition(seq);
    canonical_ordering(&mut seq);

    seq
}

pub(crate) fn nfkd(s: &str) -> Vec<char> {
    let seq = s.chars().collect::<Vec<char>>();
    let mut seq = compatibility_decomposition(&seq);
    canonical_ordering(&mut seq);

    seq
}

pub(crate) fn nfc(s: &str) -> Vec<char> {
    let mut seq = nfd(s);
    canonical_composition(&mut seq);

    seq
}

mod tests {
    #[test]
    fn test_canonical_ordering() {
        let mut s1 = vec!['a', '\u{0305}', '\u{0315}', '\u{0300}', '\u{05AE}', 'b'];
        super::canonical_ordering(&mut s1);
        assert_eq!(
            s1,
            vec!['a', '\u{05AE}', '\u{0305}', '\u{0300}', '\u{0315}', 'b']
        )
    }
}