use crate::unicode::CodePoint;
use crate::unicode::Ucd;
use crate::unicode::props::*;

pub(crate) fn math(cp: u32) -> bool {
    // Derived Property: Math
    //  Generated from: Sm + Other_Math
    let cp = CodePoint::new(cp).unwrap();
    if cp.gc() == Gc::Sm || cp.omath() {
        return true;
    }

    false
}