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

pub(crate) fn ccc_name(prop: Ccc) -> PropertyName {
    match prop {
        Ccc::NR => PropertyName {
            full: "Not_Reordered",
            abbr: "NR",
        },
        Ccc::OV => PropertyName {
            full: "Overlay",
            abbr: "OV",
        },
        Ccc::HANR => PropertyName {
            full: "Han_Reading",
            abbr: "HANR",
        },
        Ccc::NK => PropertyName {
            full: "Nukta",
            abbr: "NK",
        },
        Ccc::KV => PropertyName {
            full: "Kana_Voicing",
            abbr: "KV",
        },
        Ccc::VR => PropertyName {
            full: "Virama",
            abbr: "VR",
        },
        Ccc::CCC10 => PropertyName {
            full: "CCC10",
            abbr: "CCC10",
        },
        Ccc::CCC11 => PropertyName {
            full: "CCC11",
            abbr: "CCC11",
        },
        Ccc::CCC12 => PropertyName {
            full: "CCC12",
            abbr: "CCC12",
        },
        Ccc::CCC13 => PropertyName {
            full: "CCC13",
            abbr: "CCC13",
        },
        Ccc::CCC14 => PropertyName {
            full: "CCC14",
            abbr: "CCC14",
        },
        Ccc::CCC15 => PropertyName {
            full: "CCC15",
            abbr: "CCC15",
        },
        Ccc::CCC16 => PropertyName {
            full: "CCC16",
            abbr: "CCC16",
        },
        Ccc::CCC17 => PropertyName {
            full: "CCC17",
            abbr: "CCC17",
        },
        Ccc::CCC18 => PropertyName {
            full: "CCC18",
            abbr: "CCC18",
        },
        Ccc::CCC19 => PropertyName {
            full: "CCC19",
            abbr: "CCC19",
        },
        Ccc::CCC20 => PropertyName {
            full: "CCC20",
            abbr: "CCC20",
        },
        Ccc::CCC21 => PropertyName {
            full: "CCC21",
            abbr: "CCC21",
        },
        Ccc::CCC22 => PropertyName {
            full: "CCC22",
            abbr: "CCC22",
        },
        Ccc::CCC23 => PropertyName {
            full: "CCC23",
            abbr: "CCC23",
        },
        Ccc::CCC24 => PropertyName {
            full: "CCC24",
            abbr: "CCC24",
        },
        Ccc::CCC25 => PropertyName {
            full: "CCC25",
            abbr: "CCC25",
        },
        Ccc::CCC26 => PropertyName {
            full: "CCC26",
            abbr: "CCC26",
        },
        Ccc::CCC27 => PropertyName {
            full: "CCC27",
            abbr: "CCC27",
        },
        Ccc::CCC28 => PropertyName {
            full: "CCC28",
            abbr: "CCC28",
        },
        Ccc::CCC29 => PropertyName {
            full: "CCC29",
            abbr: "CCC29",
        },
        Ccc::CCC30 => PropertyName {
            full: "CCC30",
            abbr: "CCC30",
        },
        Ccc::CCC31 => PropertyName {
            full: "CCC31",
            abbr: "CCC31",
        },
        Ccc::CCC32 => PropertyName {
            full: "CCC32",
            abbr: "CCC32",
        },
        Ccc::CCC33 => PropertyName {
            full: "CCC33",
            abbr: "CCC33",
        },
        Ccc::CCC34 => PropertyName {
            full: "CCC34",
            abbr: "CCC34",
        },
        Ccc::CCC35 => PropertyName {
            full: "CCC35",
            abbr: "CCC35",
        },
        Ccc::CCC36 => PropertyName {
            full: "CCC36",
            abbr: "CCC36",
        },
        Ccc::CCC84 => PropertyName {
            full: "CCC84",
            abbr: "CCC84",
        },
        Ccc::CCC91 => PropertyName {
            full: "CCC91",
            abbr: "CCC91",
        },
        Ccc::CCC103 => PropertyName {
            full: "CCC103",
            abbr: "CCC103",
        },
        Ccc::CCC107 => PropertyName {
            full: "CCC107",
            abbr: "CCC107",
        },
        Ccc::CCC118 => PropertyName {
            full: "CCC118",
            abbr: "CCC118",
        },
        Ccc::CCC122 => PropertyName {
            full: "CCC122",
            abbr: "CCC122",
        },
        Ccc::CCC129 => PropertyName {
            full: "CCC129",
            abbr: "CCC129",
        },
        Ccc::CCC130 => PropertyName {
            full: "CCC130",
            abbr: "CCC130",
        },
        Ccc::CCC132 => PropertyName {
            full: "CCC132",
            abbr: "CCC132",
        },
        Ccc::CCC133 => PropertyName {
            full: "RESERVED",
            abbr: "CCC133",
        },
        Ccc::ATBL => PropertyName {
            full: "Attached_Below_Left",
            abbr: "ATBL",
        },
        Ccc::ATB => PropertyName {
            full: "Attached_Below",
            abbr: "ATB",
        },
        Ccc::ATA => PropertyName {
            full: "Attached_Above",
            abbr: "ATA",
        },
        Ccc::ATAR => PropertyName {
            full: "Attached_Above_Right",
            abbr: "ATAR",
        },
        Ccc::BL => PropertyName {
            full: "Below_Left",
            abbr: "BL",
        },
        Ccc::B => PropertyName {
            full: "Below",
            abbr: "B",
        },
        Ccc::BR => PropertyName {
            full: "Below_Right",
            abbr: "BR",
        },
        Ccc::L => PropertyName {
            full: "Left",
            abbr: "L",
        },
        Ccc::R => PropertyName {
            full: "Right",
            abbr: "R",
        },
        Ccc::AL => PropertyName {
            full: "Above_Left",
            abbr: "AL",
        },
        Ccc::A => PropertyName {
            full: "Above",
            abbr: "A",
        },
        Ccc::AR => PropertyName {
            full: "Above_Right",
            abbr: "AR",
        },
        Ccc::DB => PropertyName {
            full: "Double_Below",
            abbr: "DB",
        },
        Ccc::DA => PropertyName {
            full: "Double_Above",
            abbr: "DA",
        },
        Ccc::IS => PropertyName {
            full: "Iota_Subscript",
            abbr: "IS",
        },
    }
}

pub(crate) fn dt_name(prop: Dt) -> PropertyName {
    match prop {
        Dt::Can => PropertyName {
            full: "Canonical",
            abbr: "Can",
        },
        Dt::Com => PropertyName {
            full: "Compat",
            abbr: "Com",
        },
        Dt::Enc => PropertyName {
            full: "Circle",
            abbr: "Enc",
        },
        Dt::Fin => PropertyName {
            full: "Final",
            abbr: "Fin",
        },
        Dt::Font => PropertyName {
            full: "Font",
            abbr: "Font",
        },
        Dt::Fra => PropertyName {
            full: "Fraction",
            abbr: "Fra",
        },
        Dt::Init => PropertyName {
            full: "Initial",
            abbr: "Init",
        },
        Dt::Iso => PropertyName {
            full: "Isolated",
            abbr: "Iso",
        },
        Dt::Med => PropertyName {
            full: "Medial",
            abbr: "Med",
        },
        Dt::Nar => PropertyName {
            full: "Narrow",
            abbr: "Nar",
        },
        Dt::Nb => PropertyName {
            full: "Nobreak",
            abbr: "Nb",
        },
        Dt::None => PropertyName {
            full: "None",
            abbr: "None",
        },
        Dt::Sml => PropertyName {
            full: "Small",
            abbr: "Sml",
        },
        Dt::Sqr => PropertyName {
            full: "Square",
            abbr: "Sqr",
        },
        Dt::Sub => PropertyName {
            full: "Sub",
            abbr: "Sub",
        },
        Dt::Sup => PropertyName {
            full: "Super",
            abbr: "Sup",
        },
        Dt::Vert => PropertyName {
            full: "Vertical",
            abbr: "Vert",
        },
        Dt::Wide => PropertyName {
            full: "Wide",
            abbr: "Wide",
        },
    }
}