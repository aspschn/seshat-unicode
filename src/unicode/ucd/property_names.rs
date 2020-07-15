use crate::unicode::props::*;

pub(crate) fn binary_property_name(prop: BinaryProperty) -> PropertyName {
    match prop {
        BinaryProperty::Y => PropertyName {
            full: "Yes",
            abbr: "Y",
        },
        BinaryProperty::N => PropertyName {
            full: "No",
            abbr: "N",
        },
    }
}

pub(crate) fn gc_name(prop: Gc) -> PropertyName {
    match prop {
        Gc::Cc => PropertyName {
            full:  "Control",
            abbr: "Cc",
        },
        Gc::Cf => PropertyName {
            full:  "Format",
            abbr: "Cf",
        },
        Gc::Cn => PropertyName {
            full:  "Unassigned",
            abbr: "Cn",
        },
        Gc::Co => PropertyName {
            full:  "Private_Use",
            abbr: "Co",
        },
        Gc::Cs => PropertyName {
            full:  "Surrogate",
            abbr: "Cs",
        },
        Gc::Ll => PropertyName {
            full:  "Lowercase_Letter",
            abbr: "Ll",
        },
        Gc::Lm => PropertyName {
            full:  "Modifier_Letter",
            abbr: "Lm",
        },
        Gc::Lo => PropertyName {
            full:  "Other_Letter",
            abbr: "Lo",
        },
        Gc::Lt => PropertyName {
            full:  "Titlecase_Letter",
            abbr: "Lt",
        },
        Gc::Lu => PropertyName {
            full:  "Uppercase_Letter",
            abbr: "Lu",
        },
        Gc::Mc => PropertyName {
            full:  "Spacing_Mark",
            abbr: "Mc",
        },
        Gc::Me => PropertyName {
            full:  "Enclosing_Mark",
            abbr: "Me",
        },
        Gc::Mn => PropertyName {
            full:  "Nonspacing_Mark",
            abbr: "Mn",
        },
        Gc::Nd => PropertyName {
            full:  "Decimal_Number",
            abbr: "Nd",
        },
        Gc::Nl => PropertyName {
            full:  "Letter_Number",
            abbr: "Nl",
        },
        Gc::No => PropertyName {
            full:  "Other_Number",
            abbr: "No",
        },
        Gc::Pc => PropertyName {
            full: "Ctor_Punctuation",
            abbr: "Pc",
        },
        Gc::Pd => PropertyName {
            full:  "Dash_Punctuation",
            abbr: "Pd",
        },
        Gc::Pe => PropertyName {
            full:  "Close_Punctuation",
            abbr: "Pe",
        },
        Gc::Pf => PropertyName {
            full:  "Final_Punctuation",
            abbr: "Pf",
        },
        Gc::Pi => PropertyName {
            full: "Il_Punctuation",
            abbr: "Pi",
        },
        Gc::Po => PropertyName {
            full:  "Other_Punctuation",
            abbr: "Po",
        },
        Gc::Ps => PropertyName {
            full:  "Open_Punctuation",
            abbr: "Ps",
        },
        Gc::Sc => PropertyName {
            full: "Currency_Symbol",
            abbr: "Sc",
        },
        Gc::Sk => PropertyName {
            full: "Modifier_Symbol",
            abbr: "Sk",
        },
        Gc::Sm => PropertyName {
            full: "Math_Symbol",
            abbr: "Sm",
        },
        Gc::So => PropertyName {
            full: "Other_Symbol",
            abbr: "So",
        },
        Gc::Zl => PropertyName {
            full: "Line_Separator",
            abbr: "Zl",
        },
        Gc::Zp => PropertyName {
            full: "Paragraph_Separator",
            abbr: "Zp",
        },
        Gc::Zs => PropertyName {
            full: "Space_Separator",
            abbr: "Zs",
        },
    }
}

pub(crate) fn hst_name(prop: Hst) -> PropertyName {
    match prop {
        Hst::L => PropertyName {
            full: "Leading_Jamo",
            abbr: "L",
        },
        Hst::LV => PropertyName {
            full: "LV_Syllable",
            abbr: "LV",
        },
        Hst::LVT => PropertyName {
            full: "LVT_Syllable",
            abbr: "LVT",
        },
        Hst::NA => PropertyName {
            full: "Not_Applicable",
            abbr: "NA",
        },
        Hst::T => PropertyName {
            full: "Trailing_Jamo",
            abbr: "T",
        },
        Hst::V => PropertyName {
            full: "Vowel_Jamo",
            abbr: "V",
        },
    }
}