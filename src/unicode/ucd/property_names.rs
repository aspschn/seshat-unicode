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

pub(crate) fn gcb_name(prop: Gcb) -> PropertyName {
    match prop {
        Gcb::CN => PropertyName {
            full: "Control",
            abbr: "CN",
        },
        Gcb::CR => PropertyName {
            full: "CR",
            abbr: "CR",
        },
        Gcb::EB => PropertyName {
            full: "E_Base",
            abbr: "EB",
        },
        Gcb::EBG => PropertyName {
            full: "E_Base_GAZ",
            abbr: "EBG",
        },
        Gcb::EM => PropertyName {
            full: "E_Modifier",
            abbr: "EM",
        },
        Gcb::EX => PropertyName {
            full: "Extend",
            abbr: "EX",
        },
        Gcb::GAZ => PropertyName {
            full: "Glue_After_Zwj",
            abbr: "GAZ",
        },
        Gcb::L => PropertyName {
            full: "L",
            abbr: "L",
        }, 
        Gcb::LF => PropertyName {
            full: "LF",
            abbr: "LF",
        },
        Gcb::LV => PropertyName {
            full: "LV",
            abbr: "LV",
        },
        Gcb::LVT => PropertyName {
            full: "LVT",
            abbr: "LVT",
        },
        Gcb::PP => PropertyName {
            full: "Prepend",
            abbr: "PP",
        },
        Gcb::RI => PropertyName {
            full: "Regional_Indicator",
            abbr: "RI",
        },
        Gcb::SM => PropertyName {
            full: "SpacingMark",
            abbr: "SM",
        },
        Gcb::T => PropertyName {
            full: "T",
            abbr: "T",
        }, 
        Gcb::V => PropertyName {
            full: "V",
            abbr: "V",
        }, 
        Gcb::XX => PropertyName {
            full: "Other",
            abbr: "XX",
        },
        Gcb::ZWJ => PropertyName {
            full: "ZWJ",
            abbr: "ZWJ",
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

pub(crate) fn bc_name(prop: Bc) -> PropertyName {
    match prop {
        Bc::AL => PropertyName {
            full: "Arabic_Letter",
            abbr: "AL",
        },
        Bc::AN => PropertyName {
            full: "Arabic_Number",
            abbr: "AN",
        },
        Bc::B => PropertyName {
            full: "Paragraph_Separator",
            abbr: "B",
        },
        Bc::BN => PropertyName {
            full: "Boundary_Neutral",
            abbr: "BN",
        },
        Bc::CS => PropertyName {
            full: "Common_Separator",
            abbr: "CS",
        },
        Bc::EN => PropertyName {
            full: "European_Number",
            abbr: "EN",
        },
        Bc::ES => PropertyName {
            full: "European_Separator",
            abbr: "ES",
        },
        Bc::ET => PropertyName {
            full: "European_Terminator",
            abbr: "ET",
        },
        Bc::FSI => PropertyName {
            full: "First_Strong_Isolate",
            abbr: "FSI",
        },
        Bc::L => PropertyName {
            full: "Left_To_Right",
            abbr: "L",
        },
        Bc::LRE => PropertyName {
            full: "Left_To_Right_Embedding",
            abbr: "LRE",
        },
        Bc::LRI => PropertyName {
            full: "Left_To_Right_Isolate",
            abbr: "LRI",
        },
        Bc::LRO => PropertyName {
            full: "Left_To_Right_Override",
            abbr: "LRO",
        },
        Bc::NSM => PropertyName {
            full: "Nonspacing_Mark",
            abbr: "NSM",
        },
        Bc::ON => PropertyName {
            full: "Other_Neutral",
            abbr: "ON",
        },
        Bc::PDF => PropertyName {
            full: "Pop_Directional_Format",
            abbr: "PDF",
        },
        Bc::PDI => PropertyName {
            full: "Pop_Directional_Isolate",
            abbr: "PDI",
        },
        Bc::R => PropertyName {
            full: "Right_To_Left",
            abbr: "R",
        },
        Bc::RLE => PropertyName {
            full: "Right_To_Left_Embedding",
            abbr: "RLE",
        },
        Bc::RLI => PropertyName {
            full: "Right_To_Left_Isolate",
            abbr: "RLI",
        },
        Bc::RLO => PropertyName {
            full: "Right_To_Left_Override",
            abbr: "RLO",
        },
        Bc::S => PropertyName {
            full: "Segment_Separator",
            abbr: "S",
        },
        Bc::WS => PropertyName {
            full: "White_Space",
            abbr: "WS",
        },
    }
}