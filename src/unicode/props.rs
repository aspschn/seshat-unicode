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