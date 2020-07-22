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
    pair.0 > pair.1 && pair.1 > 0
}

pub(super) fn canonical_ordering(sequence: &mut Vec<char>) {
    // This ordering algorithm acts like a bubble sort.
    if sequence.len() == 0 {
        return ();
    }

    let mut last_idx = sequence.len() - 1;
    while last_idx > 0 {
        for i in 0..sequence.len() {
            if i + 1 == sequence.len() {
                continue;
            }
            if reorderable_pair((sequence[i] as u32, sequence[i + 1] as u32)) {
                sequence.swap(i, i + 1);
            }
        }
        last_idx -= 1;
    }
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
