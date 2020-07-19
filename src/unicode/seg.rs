use crate::unicode::Ucd;
use crate::unicode::props::Gcb;

pub(super) fn is_gb3(curr: char, next: char) -> bool {
    if curr.gcb() == Gcb::CR && next.gcb() == Gcb::LF {
        return true;
    }
    false
}

pub(super) fn is_gb4(curr: char) -> bool {
    if curr.gcb() == Gcb::CN
        || curr.gcb() == Gcb::CR
        || curr.gcb() == Gcb::LF
    {
        return true;
    }
    false
}

pub(super) fn is_gb5(next: char) -> bool {
    if next.gcb() == Gcb::CN
        || next.gcb() == Gcb::CR
        || next.gcb() == Gcb::LF
    {
        return true;
    }
    false
}

pub(super) fn is_gb6(curr: char, next: char) -> bool {
    if curr.gcb() == Gcb::L
        && (next.gcb() == Gcb::L
            || next.gcb() == Gcb::V
            || next.gcb() == Gcb::LV
            || next.gcb() == Gcb::LVT)
    {
        return true;
    }
    false
}

pub(super) fn is_gb7(curr: char, next: char) -> bool {
    if (curr.gcb() == Gcb:: LV
        || curr.gcb() == Gcb::V)
        && (next.gcb() == Gcb::V
            || next.gcb() == Gcb::T)
    {
        return true;
    }
    false
}

pub(super) fn is_gb8(curr: char, next: char) -> bool {
    if (curr.gcb() == Gcb::LVT || curr.gcb() == Gcb::T)
        && next.gcb() == Gcb::T
    {
        return true;
    }
    false
}

pub(super) fn is_gb9(next: char) -> bool {
    if next.gcb() == Gcb::EX || next.gcb() == Gcb::ZWJ {
        return true;
    }
    false
}

pub(super) fn is_gb9a(next: char) -> bool {
    if next.gcb() == Gcb::SM {
        return true;
    }
    false
}

pub(super) fn is_gb9b(curr: char) -> bool {
    if curr.gcb() == Gcb::PP {
        return true;
    }
    false
}
