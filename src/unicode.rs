pub mod props;

pub(crate) mod ucd;

pub(crate) mod hangul;

pub(crate) mod seg;

pub(crate) mod normalization;

use self::props::*;

#[derive(Clone, Copy)]
pub struct CodePoint {
    code_point: u32,
}

impl CodePoint {
    pub fn new(cp: u32) -> Result<CodePoint, &'static str> {
        if cp > 0x10FFFF {
            return Err("IllegalCodePoint: Code point cannot be over U+10FFFF.");
        }
        Ok(CodePoint { code_point: cp })
    }

    pub fn to_u32(&self) -> u32 {
        self.code_point
    }
}

pub trait ToCodePoint {
    fn to_code_point(&self) -> CodePoint;
}

impl ToCodePoint for char {
    fn to_code_point(&self) -> CodePoint {
        CodePoint::new(*self as u32).unwrap()
    }
}

impl std::fmt::Display for CodePoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "U+{:04X}", self.code_point)
    }
}

pub trait Ucd {
    fn dm(&self) -> String;

    fn na(&self) -> String;
    fn bc(&self) -> Bc;
    fn ccc(&self) -> Ccc;
    fn dt(&self) -> Dt;
    fn gc(&self) -> Gc;
    fn gcb(&self) -> Gcb;
    fn hst(&self) -> Hst;
    fn wspace(&self) -> bool;
    fn bidi_c(&self) -> bool;
    fn join_c(&self) -> bool;
    fn dash(&self) -> bool;
    fn hyphen(&self) -> bool;
    fn qmark(&self) -> bool;
    fn term(&self) -> bool;
    fn omath(&self) -> bool;
    fn hex(&self) -> bool;
    fn ahex(&self) -> bool;
    fn oalpha(&self) -> bool;
    fn ideo(&self) -> bool;
    fn dia(&self) -> bool;
    fn ext(&self) -> bool;
    fn olower(&self) -> bool;
    fn oupper(&self) -> bool;
    fn nchar(&self) -> bool;
    fn ogr_ext(&self) -> bool;
    fn idsb(&self) -> bool;
    fn idst(&self) -> bool;
    fn radical(&self) -> bool;
    fn uideo(&self) -> bool;
    fn odi(&self) -> bool;
    fn dep(&self) -> bool;
    fn sd(&self) -> bool;
    fn loe(&self) -> bool;
    fn oids(&self) -> bool;
    fn oidc(&self) -> bool;
    fn sterm(&self) -> bool;
    fn vs(&self) -> bool;
    fn pat_ws(&self) -> bool;
    fn pat_syn(&self) -> bool;
    fn pcm(&self) -> bool;
    fn ri(&self) -> bool;
    fn ce(&self) -> bool;
    fn comp_ex(&self) -> bool;

    fn math(&self) -> bool;
    fn alpha(&self) -> bool;
    fn lower(&self) -> bool;
    fn upper(&self) -> bool;

    fn emoji(&self) -> bool;
    fn epres(&self) -> bool;
    fn emod(&self) -> bool;
    fn ebase(&self) -> bool;
    fn ecomp(&self) -> bool;
    fn ext_pict(&self) -> bool;
}

impl Ucd for CodePoint {
    fn dm(&self) -> String {
        ucd::dm::dm(self.code_point)
    }

    fn na(&self) -> String {
        ucd::na::na(self.code_point)
    }

    fn bc(&self) -> Bc {
        ucd::bc::bc(self.code_point)
    }

    fn ccc(&self) -> Ccc {
        ucd::ccc::ccc(self.code_point)
    }

    fn dt(&self) -> Dt {
        ucd::dt::dt(self.code_point)
    }

    fn gc(&self) -> Gc {
        ucd::gc::gc(self.code_point)
    }

    fn gcb(&self) -> Gcb {
        ucd::gcb::gcb(self.code_point)
    }

    fn hst(&self) -> Hst {
        ucd::hst::hst(self.code_point)
    }

    fn wspace(&self) -> bool {
        ucd::binary_props::wspace(self.code_point)
    }

    fn bidi_c(&self) -> bool {
        ucd::binary_props::bidi_c(self.code_point)
    }

    fn join_c(&self) -> bool {
        ucd::binary_props::join_c(self.code_point)
    }

    fn dash(&self) -> bool {
        ucd::binary_props::dash(self.code_point)
    }

    fn hyphen(&self) -> bool {
        ucd::binary_props::hyphen(self.code_point)
    }

    fn qmark(&self) -> bool {
        ucd::binary_props::qmark(self.code_point)
    }

    fn term(&self) -> bool {
        ucd::binary_props::term(self.code_point)
    }

    fn omath(&self) -> bool {
        ucd::binary_props::omath(self.code_point)
    }

    fn hex(&self) -> bool {
        ucd::binary_props::hex(self.code_point)
    }

    fn ahex(&self) -> bool {
        ucd::binary_props::ahex(self.code_point)
    }

    fn oalpha(&self) -> bool {
        ucd::binary_props::oalpha(self.code_point)
    }

    fn ideo(&self) -> bool {
        ucd::binary_props::ideo(self.code_point)
    }

    fn dia(&self) -> bool {
        ucd::binary_props::dia(self.code_point)
    }

    fn ext(&self) -> bool {
        ucd::binary_props::ext(self.code_point)
    }

    fn olower(&self) -> bool {
        ucd::binary_props::olower(self.code_point)
    }

    fn oupper(&self) -> bool {
        ucd::binary_props::oupper(self.code_point)
    }

    fn nchar(&self) -> bool {
        ucd::binary_props::nchar(self.code_point)
    }

    fn ogr_ext(&self) -> bool {
        ucd::binary_props::ogr_ext(self.code_point)
    }

    fn idsb(&self) -> bool {
        ucd::binary_props::idsb(self.code_point)
    }

    fn idst(&self) -> bool {
        ucd::binary_props::idst(self.code_point)
    }

    fn radical(&self) -> bool {
        ucd::binary_props::radical(self.code_point)
    }

    fn uideo(&self) -> bool {
        ucd::binary_props::uideo(self.code_point)
    }

    fn odi(&self) -> bool {
        ucd::binary_props::odi(self.code_point)
    }

    fn dep(&self) -> bool {
        ucd::binary_props::dep(self.code_point)
    }

    fn sd(&self) -> bool {
        ucd::binary_props::sd(self.code_point)
    }

    fn loe(&self) -> bool {
        ucd::binary_props::loe(self.code_point)
    }

    fn oids(&self) -> bool {
        ucd::binary_props::oids(self.code_point)
    }

    fn oidc(&self) -> bool {
        ucd::binary_props::oidc(self.code_point)
    }

    fn sterm(&self) -> bool {
        ucd::binary_props::sterm(self.code_point)
    }

    fn vs(&self) -> bool {
        ucd::binary_props::vs(self.code_point)
    }

    fn pat_ws(&self) -> bool {
        ucd::binary_props::pat_ws(self.code_point)
    }

    fn pat_syn(&self) -> bool {
        ucd::binary_props::pat_syn(self.code_point)
    }

    fn pcm(&self) -> bool {
        ucd::binary_props::pcm(self.code_point)
    }

    fn ri(&self) -> bool {
        ucd::binary_props::ri(self.code_point)
    }

    fn ce(&self) -> bool {
        ucd::ce::ce(self.code_point)
    }

    fn comp_ex(&self) -> bool {
        ucd::normalization_props::comp_ex(self.code_point)
    }

    fn math(&self) -> bool {
        ucd::derived_props::math(self.code_point)
    }

    fn alpha(&self) -> bool {
        ucd::derived_props::alpha(self.code_point)
    }

    fn lower(&self) -> bool {
        ucd::derived_props::lower(self.code_point)
    }

    fn upper(&self) -> bool {
        ucd::derived_props::upper(self.code_point)
    }

    fn emoji(&self) -> bool {
        ucd::emoji_props::emoji(self.code_point)
    }

    fn epres(&self) -> bool {
        ucd::emoji_props::epres(self.code_point)
    }

    fn emod(&self) -> bool {
        ucd::emoji_props::emod(self.code_point)
    }

    fn ebase(&self) -> bool {
        ucd::emoji_props::ebase(self.code_point)
    }

    fn ecomp(&self) -> bool {
        ucd::emoji_props::ecomp(self.code_point)
    }

    fn ext_pict(&self) -> bool {
        ucd::emoji_props::ext_pict(self.code_point)
    }
}

impl Ucd for char {
    fn dm(&self) -> String {
        ucd::dm::dm(*self as u32)
    }

    fn na(&self) -> String {
        ucd::na::na(*self as u32)
    }

    fn bc(&self) -> Bc {
        ucd::bc::bc(*self as u32)
    }

    fn ccc(&self) -> Ccc {
        ucd::ccc::ccc(*self as u32)
    }

    fn dt(&self) -> Dt {
        ucd::dt::dt(*self as u32)
    }

    fn gc(&self) -> Gc {
        ucd::gc::gc(*self as u32)
    }

    fn gcb(&self) -> Gcb {
        ucd::gcb::gcb(*self as u32)
    }

    fn hst(&self) -> Hst {
        ucd::hst::hst(*self as u32)
    }

    fn wspace(&self) -> bool {
        ucd::binary_props::wspace(*self as u32)
    }

    fn bidi_c(&self) -> bool {
        ucd::binary_props::bidi_c(*self as u32)
    }

    fn join_c(&self) -> bool {
        ucd::binary_props::join_c(*self as u32)
    }

    fn dash(&self) -> bool {
        ucd::binary_props::dash(*self as u32)
    }

    fn hyphen(&self) -> bool {
        ucd::binary_props::hyphen(*self as u32)
    }

    fn qmark(&self) -> bool {
        ucd::binary_props::qmark(*self as u32)
    }

    fn term(&self) -> bool {
        ucd::binary_props::term(*self as u32)
    }

    fn omath(&self) -> bool {
        ucd::binary_props::omath(*self as u32)
    }

    fn hex(&self) -> bool {
        ucd::binary_props::hex(*self as u32)
    }

    fn ahex(&self) -> bool {
        ucd::binary_props::ahex(*self as u32)
    }

    fn oalpha(&self) -> bool {
        ucd::binary_props::oalpha(*self as u32)
    }

    fn ideo(&self) -> bool {
        ucd::binary_props::ideo(*self as u32)
    }

    fn dia(&self) -> bool {
        ucd::binary_props::dia(*self as u32)
    }

    fn ext(&self) -> bool {
        ucd::binary_props::ext(*self as u32)
    }

    fn olower(&self) -> bool {
        ucd::binary_props::olower(*self as u32)
    }

    fn oupper(&self) -> bool {
        ucd::binary_props::oupper(*self as u32)
    }

    fn nchar(&self) -> bool {
        ucd::binary_props::nchar(*self as u32)
    }

    fn ogr_ext(&self) -> bool {
        ucd::binary_props::ogr_ext(*self as u32)
    }

    fn idsb(&self) -> bool {
        ucd::binary_props::idsb(*self as u32)
    }

    fn idst(&self) -> bool {
        ucd::binary_props::idst(*self as u32)
    }

    fn radical(&self) -> bool {
        ucd::binary_props::radical(*self as u32)
    }

    fn uideo(&self) -> bool {
        ucd::binary_props::uideo(*self as u32)
    }

    fn odi(&self) -> bool {
        ucd::binary_props::odi(*self as u32)
    }

    fn dep(&self) -> bool {
        ucd::binary_props::dep(*self as u32)
    }

    fn sd(&self) -> bool {
        ucd::binary_props::sd(*self as u32)
    }

    fn loe(&self) -> bool {
        ucd::binary_props::loe(*self as u32)
    }

    fn oids(&self) -> bool {
        ucd::binary_props::oids(*self as u32)
    }

    fn oidc(&self) -> bool {
        ucd::binary_props::oidc(*self as u32)
    }

    fn sterm(&self) -> bool {
        ucd::binary_props::sterm(*self as u32)
    }

    fn vs(&self) -> bool {
        ucd::binary_props::vs(*self as u32)
    }

    fn pat_ws(&self) -> bool {
        ucd::binary_props::pat_ws(*self as u32)
    }

    fn pat_syn(&self) -> bool {
        ucd::binary_props::pat_syn(*self as u32)
    }

    fn pcm(&self) -> bool {
        ucd::binary_props::pcm(*self as u32)
    }

    fn ri(&self) -> bool {
        ucd::binary_props::ri(*self as u32)
    }

    fn ce(&self) -> bool {
        ucd::ce::ce(*self as u32)
    }

    fn comp_ex(&self) -> bool {
        ucd::normalization_props::comp_ex(*self as u32)
    }

    fn math(&self) -> bool {
        ucd::derived_props::math(*self as u32)
    }

    fn alpha(&self) -> bool {
        ucd::derived_props::alpha(*self as u32)
    }

    fn lower(&self) -> bool {
        ucd::derived_props::lower(*self as u32)
    }

    fn upper(&self) -> bool {
        ucd::derived_props::upper(*self as u32)
    }

    fn emoji(&self) -> bool {
        ucd::emoji_props::emoji(*self as u32)
    }

    fn epres(&self) -> bool {
        ucd::emoji_props::epres(*self as u32)
    }

    fn emod(&self) -> bool {
        ucd::emoji_props::emod(*self as u32)
    }

    fn ebase(&self) -> bool {
        ucd::emoji_props::ebase(*self as u32)
    }

    fn ecomp(&self) -> bool {
        ucd::emoji_props::ecomp(*self as u32)
    }

    fn ext_pict(&self) -> bool {
        ucd::emoji_props::ext_pict(*self as u32)
    }
}

pub struct BreakGraphemes<'a> {
    slice: &'a str,
}

impl<'a> Iterator for BreakGraphemes<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        // Not iterate if empty string.
        if self.slice.len() == 0 {
            return None;
        }

        let mut iter = self.slice.char_indices();
        let mut curr = iter.next();
        let mut next = iter.next();
        let mut in_ext_pict = false;
        let mut _ri_count = 0; // Ignore compiler not used warning.
        loop {
            let curr_ch = curr.unwrap();
            let next_ch = next.unwrap_or((self.slice.len(), '\u{0000}'));
            // Prepare for GB11
            if curr_ch.1.ext_pict() {
                in_ext_pict = true;
            }
            // Prepare for GB12, GB13
            if curr_ch.1.gcb() == Gcb::RI {
                _ri_count += 1;
            }
            // Do not break between a CR and LF. Otherwise, break before and after controls.
            // GB3:                  CR × LF
            if seg::is_gb3(curr_ch.1, next_ch.1) {
                curr = next;
                next = iter.next();
                continue;
            }
            // GB4: (Control | CR | LF) ÷
            if seg::is_gb4(curr_ch.1) {
                curr = next;
                // next = iter.next();
                break;
            }
            // GB5:                     ÷ (Control | CR | LF)
            if seg::is_gb5(next_ch.1) {
                curr = next;
                // next = iter.next();
                break;
            }
            // Do not break Hangul syllable sequences.
            // GB6 	        L × (L | V | LV | LVT)
            if seg::is_gb6(curr_ch.1, next_ch.1) {
                curr = next;
                next = iter.next();
                continue;
            }
            // GB7:  (LV | V) × (V | T)
            if seg::is_gb7(curr_ch.1, next_ch.1) {
                curr = next;
                next = iter.next();
                continue;
            }
            // GB8: (LVT | T) × T
            if seg::is_gb8(curr_ch.1, next_ch.1) {
                curr = next;
                next = iter.next();
                continue;
            }
            // Do not break before extending characters or ZWJ.
            // GB9:       × (Extend | ZWJ)
            if seg::is_gb9(next_ch.1) {
                curr = next;
                next = iter.next();
                continue;
            }
            // The GB9a and GB9b rules only apply to extended grapheme clusters:
            // Do not break before SpacingMarks, or after Prepend characters.
            // GB9a:         × SpacingMark
            if seg::is_gb9a(next_ch.1) {
                curr = next;
                next = iter.next();
                continue;
            }
            // GB9b: Prepend ×
            if seg::is_gb9b(curr_ch.1) {
                curr = next;
                next = iter.next();
                continue;
            }
            // Do not break within emoji modifier sequences or emoji zwj sequences.
            // GB11: \p{ExtPict} Extend* ZWJ × \p{ExtPict}
            if curr_ch.1.ext_pict() {
                in_ext_pict = true;
                curr = next;
                next = iter.next();
                continue;
            }
            if in_ext_pict
                && (curr_ch.1.gcb() == Gcb::ZWJ
                    && next_ch.1.ext_pict())
            {
                // ExtPict, ZWJ, ExtPict
                curr = next;
                next = iter.next();
                in_ext_pict = false;
                continue;
            } else if in_ext_pict
                && (curr_ch.1.gcb() == Gcb::EX
                    && next_ch.1.gcb() == Gcb::EX)
            {
                // ExtPict, EX
                curr = next;
                next = iter.next();
                continue;
            } else if in_ext_pict
                && (curr_ch.1.gcb() == Gcb::EX
                    && next_ch.1.gcb() == Gcb::ZWJ)
            {
                // EX, ZWJ
                curr = next;
                next = iter.next();
                if next.unwrap().1.ext_pict() {
                    curr = next;
                    continue;
                }
                break;
            }
            // Do not break within emoji flag sequences.
            // That is, do not break between regional indicator (RI) symbols
            // if there is an odd number of RI characters before the break point.
            // GB12:   sot (RI RI)* RI × RI
            // GB13: [^RI] (RI RI)* RI × RI
            if _ri_count % 2 != 0 {
                curr = next;
                next = iter.next();
                continue;
            } else {
                _ri_count = 0;
            }

            // GB999: Any ÷ Any
            curr = next;
            break;
        }

        let tmp = self.slice;
        // println!("curr: {:?}", curr);
        // println!("next: {:?}", next);
        // println!("slice: {}", self.slice);
        if curr.is_none() {
            self.slice = &tmp[self.slice.len()..];
            return Some(&tmp[..]);
        } else {
            self.slice = &tmp[curr.unwrap().0..];
        }

        Some(&tmp[..curr.unwrap().0])
    }
}

pub trait Segmentation {
    fn break_graphemes(&self) -> BreakGraphemes;
}

impl Segmentation for str {
    fn break_graphemes(&self) -> BreakGraphemes {
        BreakGraphemes { slice: self }
    }
}

pub trait Normalization {
    fn to_nfd(&self) -> String;
    fn to_nfkd(&self) -> String;
    fn to_nfc(&self) -> String;
}

impl Normalization for str {
    fn to_nfd(&self) -> String {
        let mut result = String::new();
        let v = normalization::nfd(self);
        for ch in v.iter() {
            result.push(*ch);
        }

        result
    }

    fn to_nfkd(&self) -> String {
        let mut result = String::new();
        let v = normalization::nfkd(self);
        for ch in v.iter() {
            result.push(*ch);
        }

        result
    }

    fn to_nfc(&self) -> String {
        let mut result = String::new();
        let v = normalization::nfc(self);
        for ch in v.iter() {
            result.push(*ch);
        }

        result
    }
}