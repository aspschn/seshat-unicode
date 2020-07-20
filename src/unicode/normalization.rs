use crate::unicode::CodePoint;
use crate::unicode::Ucd;
use crate::unicode::props::Gc;

pub(super) fn starter(cp: u32) -> bool {
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