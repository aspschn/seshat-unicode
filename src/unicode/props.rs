use crate::unicode::ucd::property_names;

#[derive(Clone, Copy)]
pub struct PropertyName {
    pub full: &'static str,
    pub abbr: &'static str,
}

pub trait UnicodeProperty {
    fn property_value_name(&self) -> PropertyName;
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum BinaryProperty {
    Y,
    N,
}

impl From<bool> for BinaryProperty {
    fn from(val: bool) -> Self {
        match val {
            true => BinaryProperty::Y,
            false => BinaryProperty::N,
        }
    }
}

impl UnicodeProperty for BinaryProperty {
    fn property_value_name(&self) -> PropertyName {
        property_names::binary_property_name(*self)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Gc {
    // C - Other
    Cc,
    Cf,
    Cn,
    Co,
    Cs,
    // L - Letter
    // LC - Cased_Letter (Ll | Lt | Lu)
    Ll,
    Lm,
    Lo,
    Lt,
    Lu,
    // M - Mark
    Mc,
    Me,
    Mn,
    // N - Number
    Nd,
    Nl,
    No,
    // P - Punctuation
    Pc,
    Pd,
    Pe,
    Pf,
    Pi,
    Po,
    Ps,
    // S - Symbol
    Sc,
    Sk,
    Sm,
    So,
    // Z - Separator
    Zl,
    Zp,
    Zs,
}

impl UnicodeProperty for Gc {
    fn property_value_name(&self) -> PropertyName {
        property_names::gc_name(*self)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Hst {
    L,      //Leading_Jamo
    LV,     //LV_Syllable
    LVT,    //LVT_Syllable
    NA,     //Not_Applicable
    T,      //Trailing_Jamo
    V,      //Vowel_Jamo
}

impl UnicodeProperty for Hst {
    fn property_value_name(&self) -> PropertyName {
        property_names::hst_name(*self)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Gcb {
    CN,     // Control
    CR,     // CR
    EB,     // E_Base
    EBG,    // E_Base_GAZ
    EM,     // E_Modifier
    EX,     // Extend
    GAZ,    // Glue_After_Zwj
    L,      // L
    LF,     // LF
    LV,     // LV
    LVT,    // LVT
    PP,     // Prepend
    RI,     // Regional_Indicator
    SM,     // SpacingMark
    T,      // T
    V,      // V
    XX,     // Other
    ZWJ,    // ZWJ
}

impl UnicodeProperty for Gcb {
    fn property_value_name(&self) -> PropertyName {
        property_names::gcb_name(*self)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Bc {
    AL,     // Arabic_Letter
    AN,     // Arabic_Number
    B,      // Paragraph_Separator
    BN,     // Boundary_Neutral
    CS,     // Common_Separator
    EN,     // European_Number
    ES,     // European_Separator
    ET,     // European_Terminator
    FSI,    // First_Strong_Isolate
    L,      // Left_To_Right
    LRE,    // Left_To_Right_Embedding
    LRI,    // Left_To_Right_Isolate
    LRO,    // Left_To_Right_Override
    NSM,    // Nonspacing_Mark
    ON,     // Other_Neutral
    PDF,    // Pop_Directional_Format
    PDI,    // Pop_Directional_Isolate
    R,      // Right_To_Left
    RLE,    // Right_To_Left_Embedding
    RLI,    // Right_To_Left_Isolate
    RLO,    // Right_To_Left_Override
    S,      // Segment_Separator
    WS,     // White_Space
}

impl UnicodeProperty for Bc {
    fn property_value_name(&self) -> PropertyName {
        property_names::bc_name(*self)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Ccc {
    NR = 0,         // Not_Reordered
    OV = 1,         // Overlay
    HANR = 6,       // Han_Reading
    NK = 7,         // Nukta
    KV = 8,         // Kana_Voicing
    VR = 9,         // Virama
    CCC10 = 10,     // CCC10
    CCC11 = 11,     // CCC11
    CCC12 = 12,     // CCC12
    CCC13 = 13,     // CCC13
    CCC14 = 14,     // CCC14
    CCC15 = 15,     // CCC15
    CCC16 = 16,     // CCC16
    CCC17 = 17,     // CCC17
    CCC18 = 18,     // CCC18
    CCC19 = 19,     // CCC19
    CCC20 = 20,     // CCC20
    CCC21 = 21,     // CCC21
    CCC22 = 22,     // CCC22
    CCC23 = 23,     // CCC23
    CCC24 = 24,     // CCC24
    CCC25 = 25,     // CCC25
    CCC26 = 26,     // CCC26
    CCC27 = 27,     // CCC27
    CCC28 = 28,     // CCC28
    CCC29 = 29,     // CCC29
    CCC30 = 30,     // CCC30
    CCC31 = 31,     // CCC31
    CCC32 = 32,     // CCC32
    CCC33 = 33,     // CCC33
    CCC34 = 34,     // CCC34
    CCC35 = 35,     // CCC35
    CCC36 = 36,     // CCC36
    CCC84 = 84,     // CCC84
    CCC91 = 91,     // CCC91
    CCC103 = 103,   // CCC103
    CCC107 = 107,   // CCC107
    CCC118 = 118,   // CCC118
    CCC122 = 122,   // CCC122
    CCC129 = 129,   // CCC129
    CCC130 = 130,   // CCC130
    CCC132 = 132,   // CCC132
    CCC133 = 133,   // CCC133 # RESERVED
    ATBL = 200,     // Attached_Below_Left
    ATB = 202,      // Attached_Below
    ATA = 214,      // Attached_Above
    ATAR = 216,     // Attached_Above_Right
    BL = 218,       // Below_Left
    B = 220,        // Below
    BR = 222,       // Below_Right
    L = 224,        // Left
    R = 226,        // Right
    AL = 228,       // Above_Left
    A = 230,        // Above
    AR = 232,       // Above_Right
    DB = 233,       // Double_Below
    DA = 234,       // Double_Above
    IS = 240,       // Iota_Subscript
}

impl UnicodeProperty for Ccc {
    fn property_value_name(&self) -> PropertyName {
        property_names::ccc_name(*self)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Dt {
    Can,    // Canonical
    Com,    // Compat
    Enc,    // Circle
    Fin,    // Final
    Font,   // Font
    Fra,    // Fraction
    Init,   // Initial
    Iso,    // Isolated
    Med,    // Medial
    Nar,    // Narrow
    Nb,     // Nobreak
    None,   // None
    Sml,    // Small
    Sqr,    // Square
    Sub,    // Sub
    Sup,    // Super
    Vert,   // Vertical
    Wide,   // Wide
}

impl UnicodeProperty for Dt {
    fn property_value_name(&self) -> PropertyName {
        property_names::dt_name(*self)
    }
}