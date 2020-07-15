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