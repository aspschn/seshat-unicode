use crate::unicode::props::*;
use crate::unicode::CodePoint;
use crate::unicode::Ucd;
use crate::unicode::Incb;

pub(crate) fn math(cp: u32) -> bool {
    // Derived Property: Math
    //  Generated from: Sm + Other_Math
    let cp = CodePoint::new(cp).unwrap();
    if cp.gc() == Gc::Sm || cp.omath() {
        return true;
    }

    false
}

pub(crate) fn alpha(cp: u32) -> bool {
    // Derived Property: Alphabetic
    //  Generated from: Uppercase + Lowercase + Lt + Lm + Lo + Nl + Other_Alphabetic
    let cp = CodePoint::new(cp).unwrap();
    let gc = cp.gc();

    cp.upper()
        || cp.lower()
        || gc == Gc::Lt
        || gc == Gc::Lm
        || gc == Gc::Lo
        || gc == Gc::Nl
        || cp.oalpha()
}

pub(crate) fn lower(cp: u32) -> bool {
    // Derived Property: Lowercase
    //  Generated from: Ll + Other_Lowercase
    let cp = CodePoint::new(cp).unwrap();

    cp.gc() == Gc::Ll || cp.olower()
}

pub(crate) fn upper(cp: u32) -> bool {
    // Derived Property: Uppercase
    //  Generated from: Lu + Other_Uppercase
    let cp = CodePoint::new(cp).unwrap();

    cp.gc() == Gc::Lu || cp.oupper()
}

pub(crate) fn cased(cp: u32) -> bool {
    // Derived Property:   Cased (Cased)
    //  As defined by Unicode Standard Definition D135
    //  C has the Lowercase or Uppercase property or has a General_Category value of Titlecase_Letter.
    let cp = CodePoint::new(cp).unwrap();

    cp.lower() || cp.upper() || cp.gc() == Gc::Lt
}

pub(crate) fn ci(cp: u32) -> bool {
    // Derived Property:   Case_Ignorable (CI)
    //  As defined by Unicode Standard Definition D136
    //  C is defined to be case-ignorable if
    //    Word_Break(C) = MidLetter or MidNumLet or Single_Quote, or
    //    General_Category(C) = Nonspacing_Mark (Mn), Enclosing_Mark (Me), Format (Cf), Modifier_Letter (Lm), or Modifier_Symbol (Sk).
    let cp = CodePoint::new(cp).unwrap();
    let wb = cp.wb();
    let gc = cp.gc();

    wb == Wb::ML || wb == Wb::MB || wb == Wb::SQ ||
        gc == Gc::Mn || gc == Gc::Me || gc == Gc::Cf || gc == Gc::Lm || gc == Gc::Sk
}

// Derived Property:   Changes_When_Lowercased (CWL)
//  Characters whose normalized forms are not stable under a toLowercase mapping.
//  For more information, see D139 in Section 3.13, "Default Case Algorithms".
//  Changes_When_Lowercased(X) is true when toLowercase(toNFD(X)) != toNFD(X)

// Derived Property:   Changes_When_Uppercased (CWU)
//  Characters whose normalized forms are not stable under a toUppercase mapping.
//  For more information, see D140 in Section 3.13, "Default Case Algorithms".
//  Changes_When_Uppercased(X) is true when toUppercase(toNFD(X)) != toNFD(X)

// Derived Property:   Changes_When_Titlecased (CWT)
//  Characters whose normalized forms are not stable under a toTitlecase mapping.
//  For more information, see D141 in Section 3.13, "Default Case Algorithms".
//  Changes_When_Titlecased(X) is true when toTitlecase(toNFD(X)) != toNFD(X)

// # Derived Property:   Changes_When_Casefolded (CWCF)
// #  Characters whose normalized forms are not stable under case folding.
// #  For more information, see D142 in Section 3.13, "Default Case Algorithms".
// #  Changes_When_Casefolded(X) is true when toCasefold(toNFD(X)) != toNFD(X)

// # Derived Property:   Changes_When_Casemapped (CWCM)
// #  Characters whose normalized forms are not stable under case mapping.
// #  For more information, see D143 in Section 3.13, "Default Case Algorithms".
// #  Changes_When_Casemapped(X) is true when CWL(X), or CWT(X), or CWU(X)

// # Derived Property: ID_Start
// #  Characters that can start an identifier.
// #  Generated from:
// #      Lu + Ll + Lt + Lm + Lo + Nl
// #    + Other_ID_Start
// #    - Pattern_Syntax
// #    - Pattern_White_Space
// #  NOTE: See UAX #31 for more information

// # Derived Property: ID_Continue
// #  Characters that can continue an identifier.
// #  Generated from:
// #      ID_Start
// #    + Mn + Mc + Nd + Pc
// #    + Other_ID_Continue
// #    - Pattern_Syntax
// #    - Pattern_White_Space
// #  NOTE: See UAX #31 for more information

// # Derived Property: XID_Start
// #  ID_Start modified for closure under NFKx
// #  Modified as described in UAX #15
// #  NOTE: Does NOT remove the non-NFKx characters.
// #        Merely ensures that if isIdentifer(string) then isIdentifier(NFKx(string))
// #  NOTE: See UAX #31 for more information

// # Derived Property: XID_Continue
// #  Mod_ID_Continue modified for closure under NFKx
// #  Modified as described in UAX #15
// #  NOTE: Does NOT remove the non-NFKx characters.
// #        Merely ensures that if isIdentifer(string) then isIdentifier(NFKx(string))
// #  NOTE: See UAX #31 for more information

pub(crate) fn di(cp: u32) -> bool {
    // # Derived Property: Default_Ignorable_Code_Point
    // #  Generated from
    // #    Other_Default_Ignorable_Code_Point
    // #  + Cf (Format characters)
    // #  + Variation_Selector
    // #  - White_Space
    // #  - FFF9..FFFB (Interlinear annotation format characters)
    // #  - 13430..13440 (Egyptian hieroglyph format characters)
    // #  - Prepended_Concatenation_Mark (Exceptional format characters that should be visible)
    let cp = CodePoint::new(cp).unwrap();
    if cp.odi() || cp.gc() == Gc::Cf || cp.vs() {
        if !cp.wspace() && !(0xFFF9..=0xFFFB).contains(&cp.to_u32()) &&
                !(0x13430..=0x13440).contains(&cp.to_u32()) &&
                !cp.pcm() {
            return true;
        }
    }
    return false;
}

pub(crate) fn gr_ext(cp: u32) -> bool {
    // # Derived Property: Grapheme_Extend
    // #  Generated from: Me + Mn + Other_Grapheme_Extend
    // #  Note: depending on an application's interpretation of Co (private use),
    // #  they may be either in Grapheme_Base, or in Grapheme_Extend, or in neither.
    let cp = CodePoint::new(cp).unwrap();
    cp.gc() == Gc::Me || cp.gc() == Gc::Mn || cp.ogr_ext()
}

// # Derived Property: Grapheme_Base
// #  Generated from: [0..10FFFF] - Cc - Cf - Cs - Co - Cn - Zl - Zp - Grapheme_Extend
// #  Note: depending on an application's interpretation of Co (private use),
// #  they may be either in Grapheme_Base, or in Grapheme_Extend, or in neither.

// # Derived Property: Grapheme_Link (deprecated)
// #  Generated from: Canonical_Combining_Class=Virama
// #  Use Canonical_Combining_Class=Virama directly instead

// # Derived Property: Indic_Conjunct_Break
// #  Generated from the Grapheme_Cluster_Break, Indic_Syllabic_Category,
// #  Canonical_Combining_Class, and Script properties as described in UAX #44:
// #  https://www.unicode.org/reports/tr44/.
// #
// #  All code points not explicitly listed for Indic_Conjunct_Break
// #  have the value None.
pub(crate) fn incb(cp: u32) -> Incb {
    let set = [Sc::Beng, Sc::Deva, Sc::Gujr, Sc::Mlym, Sc::Orya, Sc::Telu];

    let cp = CodePoint::new(cp).unwrap();
    let sc = cp.sc();

    if set.contains(&sc) && cp.insc() == Insc::Virama {
        return Incb::Linker;
    }
    if set.contains(&sc) && cp.insc() == Insc::Consonant {
        return Incb::Consonant;
    }
    if cp.gcb() == Gcb::EX && cp.ccc() != Ccc::NR {
        if cp.insc() != Insc::Virama && cp.insc() != Insc::Consonant {
            return Incb::Extend;
        }

        return Incb::None;
    }
    if cp.gcb() == Gcb::ZWJ && cp.insc() != Insc::Virama && cp.insc() != Insc::Consonant {
        return Incb::Extend;
    }

    Incb::None
}