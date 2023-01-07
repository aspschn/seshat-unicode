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

pub(crate) fn blk_name(prop: Blk) -> PropertyName {
    // This method is generated by `gen-blk-name.py` script.
    match prop {
        Blk::Adlam => PropertyName {
            full: "Adlam",
            abbr: "Adlam",
        },
        Blk::AegeanNumbers => PropertyName {
            full: "Aegean_Numbers",
            abbr: "Aegean_Numbers",
        },
        Blk::Ahom => PropertyName {
            full: "Ahom",
            abbr: "Ahom",
        },
        Blk::Alchemical => PropertyName {
            full: "Alchemical_Symbols",
            abbr: "Alchemical",
        },
        Blk::AlphabeticPf => PropertyName {
            full: "Alphabetic_Presentation_Forms",
            abbr: "Alphabetic_PF",
        },
        Blk::AnatolianHieroglyphs => PropertyName {
            full: "Anatolian_Hieroglyphs",
            abbr: "Anatolian_Hieroglyphs",
        },
        Blk::AncientGreekMusic => PropertyName {
            full: "Ancient_Greek_Musical_Notation",
            abbr: "Ancient_Greek_Music",
        },
        Blk::AncientGreekNumbers => PropertyName {
            full: "Ancient_Greek_Numbers",
            abbr: "Ancient_Greek_Numbers",
        },
        Blk::AncientSymbols => PropertyName {
            full: "Ancient_Symbols",
            abbr: "Ancient_Symbols",
        },
        Blk::Arabic => PropertyName {
            full: "Arabic",
            abbr: "Arabic",
        },
        Blk::ArabicExtA => PropertyName {
            full: "Arabic_Extended_A",
            abbr: "Arabic_Ext_A",
        },
        Blk::ArabicExtB => PropertyName {
            full: "Arabic_Extended_B",
            abbr: "Arabic_Ext_B",
        },
        Blk::ArabicExtC => PropertyName {
            full: "Arabic_Extended_C",
            abbr: "Arabic_Ext_C",
        },
        Blk::ArabicMath => PropertyName {
            full: "Arabic_Mathematical_Alphabetic_Symbols",
            abbr: "Arabic_Math",
        },
        Blk::ArabicPfA => PropertyName {
            full: "Arabic_Presentation_Forms_A",
            abbr: "Arabic_PF_A",
        },
        Blk::ArabicPfB => PropertyName {
            full: "Arabic_Presentation_Forms_B",
            abbr: "Arabic_PF_B",
        },
        Blk::ArabicSup => PropertyName {
            full: "Arabic_Supplement",
            abbr: "Arabic_Sup",
        },
        Blk::Armenian => PropertyName {
            full: "Armenian",
            abbr: "Armenian",
        },
        Blk::Arrows => PropertyName {
            full: "Arrows",
            abbr: "Arrows",
        },
        Blk::Ascii => PropertyName {
            full: "Basic_Latin",
            abbr: "ASCII",
        },
        Blk::Avestan => PropertyName {
            full: "Avestan",
            abbr: "Avestan",
        },
        Blk::Balinese => PropertyName {
            full: "Balinese",
            abbr: "Balinese",
        },
        Blk::Bamum => PropertyName {
            full: "Bamum",
            abbr: "Bamum",
        },
        Blk::BamumSup => PropertyName {
            full: "Bamum_Supplement",
            abbr: "Bamum_Sup",
        },
        Blk::BassaVah => PropertyName {
            full: "Bassa_Vah",
            abbr: "Bassa_Vah",
        },
        Blk::Batak => PropertyName {
            full: "Batak",
            abbr: "Batak",
        },
        Blk::Bengali => PropertyName {
            full: "Bengali",
            abbr: "Bengali",
        },
        Blk::Bhaiksuki => PropertyName {
            full: "Bhaiksuki",
            abbr: "Bhaiksuki",
        },
        Blk::BlockElements => PropertyName {
            full: "Block_Elements",
            abbr: "Block_Elements",
        },
        Blk::Bopomofo => PropertyName {
            full: "Bopomofo",
            abbr: "Bopomofo",
        },
        Blk::BopomofoExt => PropertyName {
            full: "Bopomofo_Extended",
            abbr: "Bopomofo_Ext",
        },
        Blk::BoxDrawing => PropertyName {
            full: "Box_Drawing",
            abbr: "Box_Drawing",
        },
        Blk::Brahmi => PropertyName {
            full: "Brahmi",
            abbr: "Brahmi",
        },
        Blk::Braille => PropertyName {
            full: "Braille_Patterns",
            abbr: "Braille",
        },
        Blk::Buginese => PropertyName {
            full: "Buginese",
            abbr: "Buginese",
        },
        Blk::Buhid => PropertyName {
            full: "Buhid",
            abbr: "Buhid",
        },
        Blk::ByzantineMusic => PropertyName {
            full: "Byzantine_Musical_Symbols",
            abbr: "Byzantine_Music",
        },
        Blk::Carian => PropertyName {
            full: "Carian",
            abbr: "Carian",
        },
        Blk::CaucasianAlbanian => PropertyName {
            full: "Caucasian_Albanian",
            abbr: "Caucasian_Albanian",
        },
        Blk::Chakma => PropertyName {
            full: "Chakma",
            abbr: "Chakma",
        },
        Blk::Cham => PropertyName {
            full: "Cham",
            abbr: "Cham",
        },
        Blk::Cherokee => PropertyName {
            full: "Cherokee",
            abbr: "Cherokee",
        },
        Blk::CherokeeSup => PropertyName {
            full: "Cherokee_Supplement",
            abbr: "Cherokee_Sup",
        },
        Blk::ChessSymbols => PropertyName {
            full: "Chess_Symbols",
            abbr: "Chess_Symbols",
        },
        Blk::Chorasmian => PropertyName {
            full: "Chorasmian",
            abbr: "Chorasmian",
        },
        Blk::Cjk => PropertyName {
            full: "CJK_Unified_Ideographs",
            abbr: "CJK",
        },
        Blk::CjkCompat => PropertyName {
            full: "CJK_Compatibility",
            abbr: "CJK_Compat",
        },
        Blk::CjkCompatForms => PropertyName {
            full: "CJK_Compatibility_Forms",
            abbr: "CJK_Compat_Forms",
        },
        Blk::CjkCompatIdeographs => PropertyName {
            full: "CJK_Compatibility_Ideographs",
            abbr: "CJK_Compat_Ideographs",
        },
        Blk::CjkCompatIdeographsSup => PropertyName {
            full: "CJK_Compatibility_Ideographs_Supplement",
            abbr: "CJK_Compat_Ideographs_Sup",
        },
        Blk::CjkExtA => PropertyName {
            full: "CJK_Unified_Ideographs_Extension_A",
            abbr: "CJK_Ext_A",
        },
        Blk::CjkExtB => PropertyName {
            full: "CJK_Unified_Ideographs_Extension_B",
            abbr: "CJK_Ext_B",
        },
        Blk::CjkExtC => PropertyName {
            full: "CJK_Unified_Ideographs_Extension_C",
            abbr: "CJK_Ext_C",
        },
        Blk::CjkExtD => PropertyName {
            full: "CJK_Unified_Ideographs_Extension_D",
            abbr: "CJK_Ext_D",
        },
        Blk::CjkExtE => PropertyName {
            full: "CJK_Unified_Ideographs_Extension_E",
            abbr: "CJK_Ext_E",
        },
        Blk::CjkExtF => PropertyName {
            full: "CJK_Unified_Ideographs_Extension_F",
            abbr: "CJK_Ext_F",
        },
        Blk::CjkExtG => PropertyName {
            full: "CJK_Unified_Ideographs_Extension_G",
            abbr: "CJK_Ext_G",
        },
        Blk::CjkExtH => PropertyName {
            full: "CJK_Unified_Ideographs_Extension_H",
            abbr: "CJK_Ext_H",
        },
        Blk::CjkRadicalsSup => PropertyName {
            full: "CJK_Radicals_Supplement",
            abbr: "CJK_Radicals_Sup",
        },
        Blk::CjkStrokes => PropertyName {
            full: "CJK_Strokes",
            abbr: "CJK_Strokes",
        },
        Blk::CjkSymbols => PropertyName {
            full: "CJK_Symbols_And_Punctuation",
            abbr: "CJK_Symbols",
        },
        Blk::CompatJamo => PropertyName {
            full: "Hangul_Compatibility_Jamo",
            abbr: "Compat_Jamo",
        },
        Blk::ControlPictures => PropertyName {
            full: "Control_Pictures",
            abbr: "Control_Pictures",
        },
        Blk::Coptic => PropertyName {
            full: "Coptic",
            abbr: "Coptic",
        },
        Blk::CopticEpactNumbers => PropertyName {
            full: "Coptic_Epact_Numbers",
            abbr: "Coptic_Epact_Numbers",
        },
        Blk::CountingRod => PropertyName {
            full: "Counting_Rod_Numerals",
            abbr: "Counting_Rod",
        },
        Blk::Cuneiform => PropertyName {
            full: "Cuneiform",
            abbr: "Cuneiform",
        },
        Blk::CuneiformNumbers => PropertyName {
            full: "Cuneiform_Numbers_And_Punctuation",
            abbr: "Cuneiform_Numbers",
        },
        Blk::CurrencySymbols => PropertyName {
            full: "Currency_Symbols",
            abbr: "Currency_Symbols",
        },
        Blk::CypriotSyllabary => PropertyName {
            full: "Cypriot_Syllabary",
            abbr: "Cypriot_Syllabary",
        },
        Blk::CyproMinoan => PropertyName {
            full: "Cypro_Minoan",
            abbr: "Cypro_Minoan",
        },
        Blk::Cyrillic => PropertyName {
            full: "Cyrillic",
            abbr: "Cyrillic",
        },
        Blk::CyrillicExtA => PropertyName {
            full: "Cyrillic_Extended_A",
            abbr: "Cyrillic_Ext_A",
        },
        Blk::CyrillicExtB => PropertyName {
            full: "Cyrillic_Extended_B",
            abbr: "Cyrillic_Ext_B",
        },
        Blk::CyrillicExtC => PropertyName {
            full: "Cyrillic_Extended_C",
            abbr: "Cyrillic_Ext_C",
        },
        Blk::CyrillicExtD => PropertyName {
            full: "Cyrillic_Extended_D",
            abbr: "Cyrillic_Ext_D",
        },
        Blk::CyrillicSup => PropertyName {
            full: "Cyrillic_Supplement",
            abbr: "Cyrillic_Sup",
        },
        Blk::Deseret => PropertyName {
            full: "Deseret",
            abbr: "Deseret",
        },
        Blk::Devanagari => PropertyName {
            full: "Devanagari",
            abbr: "Devanagari",
        },
        Blk::DevanagariExt => PropertyName {
            full: "Devanagari_Extended",
            abbr: "Devanagari_Ext",
        },
        Blk::DevanagariExtA => PropertyName {
            full: "Devanagari_Extended_A",
            abbr: "Devanagari_Ext_A",
        },
        Blk::Diacriticals => PropertyName {
            full: "Combining_Diacritical_Marks",
            abbr: "Diacriticals",
        },
        Blk::DiacriticalsExt => PropertyName {
            full: "Combining_Diacritical_Marks_Extended",
            abbr: "Diacriticals_Ext",
        },
        Blk::DiacriticalsForSymbols => PropertyName {
            full: "Combining_Diacritical_Marks_For_Symbols",
            abbr: "Diacriticals_For_Symbols",
        },
        Blk::DiacriticalsSup => PropertyName {
            full: "Combining_Diacritical_Marks_Supplement",
            abbr: "Diacriticals_Sup",
        },
        Blk::Dingbats => PropertyName {
            full: "Dingbats",
            abbr: "Dingbats",
        },
        Blk::DivesAkuru => PropertyName {
            full: "Dives_Akuru",
            abbr: "Dives_Akuru",
        },
        Blk::Dogra => PropertyName {
            full: "Dogra",
            abbr: "Dogra",
        },
        Blk::Domino => PropertyName {
            full: "Domino_Tiles",
            abbr: "Domino",
        },
        Blk::Duployan => PropertyName {
            full: "Duployan",
            abbr: "Duployan",
        },
        Blk::EarlyDynasticCuneiform => PropertyName {
            full: "Early_Dynastic_Cuneiform",
            abbr: "Early_Dynastic_Cuneiform",
        },
        Blk::EgyptianHieroglyphFormatControls => PropertyName {
            full: "Egyptian_Hieroglyph_Format_Controls",
            abbr: "Egyptian_Hieroglyph_Format_Controls",
        },
        Blk::EgyptianHieroglyphs => PropertyName {
            full: "Egyptian_Hieroglyphs",
            abbr: "Egyptian_Hieroglyphs",
        },
        Blk::Elbasan => PropertyName {
            full: "Elbasan",
            abbr: "Elbasan",
        },
        Blk::Elymaic => PropertyName {
            full: "Elymaic",
            abbr: "Elymaic",
        },
        Blk::Emoticons => PropertyName {
            full: "Emoticons",
            abbr: "Emoticons",
        },
        Blk::EnclosedAlphanum => PropertyName {
            full: "Enclosed_Alphanumerics",
            abbr: "Enclosed_Alphanum",
        },
        Blk::EnclosedAlphanumSup => PropertyName {
            full: "Enclosed_Alphanumeric_Supplement",
            abbr: "Enclosed_Alphanum_Sup",
        },
        Blk::EnclosedCjk => PropertyName {
            full: "Enclosed_CJK_Letters_And_Months",
            abbr: "Enclosed_CJK",
        },
        Blk::EnclosedIdeographicSup => PropertyName {
            full: "Enclosed_Ideographic_Supplement",
            abbr: "Enclosed_Ideographic_Sup",
        },
        Blk::Ethiopic => PropertyName {
            full: "Ethiopic",
            abbr: "Ethiopic",
        },
        Blk::EthiopicExt => PropertyName {
            full: "Ethiopic_Extended",
            abbr: "Ethiopic_Ext",
        },
        Blk::EthiopicExtA => PropertyName {
            full: "Ethiopic_Extended_A",
            abbr: "Ethiopic_Ext_A",
        },
        Blk::EthiopicExtB => PropertyName {
            full: "Ethiopic_Extended_B",
            abbr: "Ethiopic_Ext_B",
        },
        Blk::EthiopicSup => PropertyName {
            full: "Ethiopic_Supplement",
            abbr: "Ethiopic_Sup",
        },
        Blk::GeometricShapes => PropertyName {
            full: "Geometric_Shapes",
            abbr: "Geometric_Shapes",
        },
        Blk::GeometricShapesExt => PropertyName {
            full: "Geometric_Shapes_Extended",
            abbr: "Geometric_Shapes_Ext",
        },
        Blk::Georgian => PropertyName {
            full: "Georgian",
            abbr: "Georgian",
        },
        Blk::GeorgianExt => PropertyName {
            full: "Georgian_Extended",
            abbr: "Georgian_Ext",
        },
        Blk::GeorgianSup => PropertyName {
            full: "Georgian_Supplement",
            abbr: "Georgian_Sup",
        },
        Blk::Glagolitic => PropertyName {
            full: "Glagolitic",
            abbr: "Glagolitic",
        },
        Blk::GlagoliticSup => PropertyName {
            full: "Glagolitic_Supplement",
            abbr: "Glagolitic_Sup",
        },
        Blk::Gothic => PropertyName {
            full: "Gothic",
            abbr: "Gothic",
        },
        Blk::Grantha => PropertyName {
            full: "Grantha",
            abbr: "Grantha",
        },
        Blk::Greek => PropertyName {
            full: "Greek_And_Coptic",
            abbr: "Greek",
        },
        Blk::GreekExt => PropertyName {
            full: "Greek_Extended",
            abbr: "Greek_Ext",
        },
        Blk::Gujarati => PropertyName {
            full: "Gujarati",
            abbr: "Gujarati",
        },
        Blk::GunjalaGondi => PropertyName {
            full: "Gunjala_Gondi",
            abbr: "Gunjala_Gondi",
        },
        Blk::Gurmukhi => PropertyName {
            full: "Gurmukhi",
            abbr: "Gurmukhi",
        },
        Blk::HalfAndFullForms => PropertyName {
            full: "Halfwidth_And_Fullwidth_Forms",
            abbr: "Half_And_Full_Forms",
        },
        Blk::HalfMarks => PropertyName {
            full: "Combining_Half_Marks",
            abbr: "Half_Marks",
        },
        Blk::Hangul => PropertyName {
            full: "Hangul_Syllables",
            abbr: "Hangul",
        },
        Blk::HanifiRohingya => PropertyName {
            full: "Hanifi_Rohingya",
            abbr: "Hanifi_Rohingya",
        },
        Blk::Hanunoo => PropertyName {
            full: "Hanunoo",
            abbr: "Hanunoo",
        },
        Blk::Hatran => PropertyName {
            full: "Hatran",
            abbr: "Hatran",
        },
        Blk::Hebrew => PropertyName {
            full: "Hebrew",
            abbr: "Hebrew",
        },
        Blk::HighPuSurrogates => PropertyName {
            full: "High_Private_Use_Surrogates",
            abbr: "High_PU_Surrogates",
        },
        Blk::HighSurrogates => PropertyName {
            full: "High_Surrogates",
            abbr: "High_Surrogates",
        },
        Blk::Hiragana => PropertyName {
            full: "Hiragana",
            abbr: "Hiragana",
        },
        Blk::Idc => PropertyName {
            full: "Ideographic_Description_Characters",
            abbr: "IDC",
        },
        Blk::IdeographicSymbols => PropertyName {
            full: "Ideographic_Symbols_And_Punctuation",
            abbr: "Ideographic_Symbols",
        },
        Blk::ImperialAramaic => PropertyName {
            full: "Imperial_Aramaic",
            abbr: "Imperial_Aramaic",
        },
        Blk::IndicNumberForms => PropertyName {
            full: "Common_Indic_Number_Forms",
            abbr: "Indic_Number_Forms",
        },
        Blk::IndicSiyaqNumbers => PropertyName {
            full: "Indic_Siyaq_Numbers",
            abbr: "Indic_Siyaq_Numbers",
        },
        Blk::InscriptionalPahlavi => PropertyName {
            full: "Inscriptional_Pahlavi",
            abbr: "Inscriptional_Pahlavi",
        },
        Blk::InscriptionalParthian => PropertyName {
            full: "Inscriptional_Parthian",
            abbr: "Inscriptional_Parthian",
        },
        Blk::IpaExt => PropertyName {
            full: "IPA_Extensions",
            abbr: "IPA_Ext",
        },
        Blk::Jamo => PropertyName {
            full: "Hangul_Jamo",
            abbr: "Jamo",
        },
        Blk::JamoExtA => PropertyName {
            full: "Hangul_Jamo_Extended_A",
            abbr: "Jamo_Ext_A",
        },
        Blk::JamoExtB => PropertyName {
            full: "Hangul_Jamo_Extended_B",
            abbr: "Jamo_Ext_B",
        },
        Blk::Javanese => PropertyName {
            full: "Javanese",
            abbr: "Javanese",
        },
        Blk::Kaithi => PropertyName {
            full: "Kaithi",
            abbr: "Kaithi",
        },
        Blk::KaktovikNumerals => PropertyName {
            full: "Kaktovik_Numerals",
            abbr: "Kaktovik_Numerals",
        },
        Blk::KanaExtA => PropertyName {
            full: "Kana_Extended_A",
            abbr: "Kana_Ext_A",
        },
        Blk::KanaExtB => PropertyName {
            full: "Kana_Extended_B",
            abbr: "Kana_Ext_B",
        },
        Blk::KanaSup => PropertyName {
            full: "Kana_Supplement",
            abbr: "Kana_Sup",
        },
        Blk::Kanbun => PropertyName {
            full: "Kanbun",
            abbr: "Kanbun",
        },
        Blk::Kangxi => PropertyName {
            full: "Kangxi_Radicals",
            abbr: "Kangxi",
        },
        Blk::Kannada => PropertyName {
            full: "Kannada",
            abbr: "Kannada",
        },
        Blk::Katakana => PropertyName {
            full: "Katakana",
            abbr: "Katakana",
        },
        Blk::KatakanaExt => PropertyName {
            full: "Katakana_Phonetic_Extensions",
            abbr: "Katakana_Ext",
        },
        Blk::Kawi => PropertyName {
            full: "Kawi",
            abbr: "Kawi",
        },
        Blk::KayahLi => PropertyName {
            full: "Kayah_Li",
            abbr: "Kayah_Li",
        },
        Blk::Kharoshthi => PropertyName {
            full: "Kharoshthi",
            abbr: "Kharoshthi",
        },
        Blk::KhitanSmallScript => PropertyName {
            full: "Khitan_Small_Script",
            abbr: "Khitan_Small_Script",
        },
        Blk::Khmer => PropertyName {
            full: "Khmer",
            abbr: "Khmer",
        },
        Blk::KhmerSymbols => PropertyName {
            full: "Khmer_Symbols",
            abbr: "Khmer_Symbols",
        },
        Blk::Khojki => PropertyName {
            full: "Khojki",
            abbr: "Khojki",
        },
        Blk::Khudawadi => PropertyName {
            full: "Khudawadi",
            abbr: "Khudawadi",
        },
        Blk::Lao => PropertyName {
            full: "Lao",
            abbr: "Lao",
        },
        Blk::Latin1Sup => PropertyName {
            full: "Latin_1_Supplement",
            abbr: "Latin_1_Sup",
        },
        Blk::LatinExtA => PropertyName {
            full: "Latin_Extended_A",
            abbr: "Latin_Ext_A",
        },
        Blk::LatinExtAdditional => PropertyName {
            full: "Latin_Extended_Additional",
            abbr: "Latin_Ext_Additional",
        },
        Blk::LatinExtB => PropertyName {
            full: "Latin_Extended_B",
            abbr: "Latin_Ext_B",
        },
        Blk::LatinExtC => PropertyName {
            full: "Latin_Extended_C",
            abbr: "Latin_Ext_C",
        },
        Blk::LatinExtD => PropertyName {
            full: "Latin_Extended_D",
            abbr: "Latin_Ext_D",
        },
        Blk::LatinExtE => PropertyName {
            full: "Latin_Extended_E",
            abbr: "Latin_Ext_E",
        },
        Blk::LatinExtF => PropertyName {
            full: "Latin_Extended_F",
            abbr: "Latin_Ext_F",
        },
        Blk::LatinExtG => PropertyName {
            full: "Latin_Extended_G",
            abbr: "Latin_Ext_G",
        },
        Blk::Lepcha => PropertyName {
            full: "Lepcha",
            abbr: "Lepcha",
        },
        Blk::LetterlikeSymbols => PropertyName {
            full: "Letterlike_Symbols",
            abbr: "Letterlike_Symbols",
        },
        Blk::Limbu => PropertyName {
            full: "Limbu",
            abbr: "Limbu",
        },
        Blk::LinearA => PropertyName {
            full: "Linear_A",
            abbr: "Linear_A",
        },
        Blk::LinearBIdeograms => PropertyName {
            full: "Linear_B_Ideograms",
            abbr: "Linear_B_Ideograms",
        },
        Blk::LinearBSyllabary => PropertyName {
            full: "Linear_B_Syllabary",
            abbr: "Linear_B_Syllabary",
        },
        Blk::Lisu => PropertyName {
            full: "Lisu",
            abbr: "Lisu",
        },
        Blk::LisuSup => PropertyName {
            full: "Lisu_Supplement",
            abbr: "Lisu_Sup",
        },
        Blk::LowSurrogates => PropertyName {
            full: "Low_Surrogates",
            abbr: "Low_Surrogates",
        },
        Blk::Lycian => PropertyName {
            full: "Lycian",
            abbr: "Lycian",
        },
        Blk::Lydian => PropertyName {
            full: "Lydian",
            abbr: "Lydian",
        },
        Blk::Mahajani => PropertyName {
            full: "Mahajani",
            abbr: "Mahajani",
        },
        Blk::Mahjong => PropertyName {
            full: "Mahjong_Tiles",
            abbr: "Mahjong",
        },
        Blk::Makasar => PropertyName {
            full: "Makasar",
            abbr: "Makasar",
        },
        Blk::Malayalam => PropertyName {
            full: "Malayalam",
            abbr: "Malayalam",
        },
        Blk::Mandaic => PropertyName {
            full: "Mandaic",
            abbr: "Mandaic",
        },
        Blk::Manichaean => PropertyName {
            full: "Manichaean",
            abbr: "Manichaean",
        },
        Blk::Marchen => PropertyName {
            full: "Marchen",
            abbr: "Marchen",
        },
        Blk::MasaramGondi => PropertyName {
            full: "Masaram_Gondi",
            abbr: "Masaram_Gondi",
        },
        Blk::MathAlphanum => PropertyName {
            full: "Mathematical_Alphanumeric_Symbols",
            abbr: "Math_Alphanum",
        },
        Blk::MathOperators => PropertyName {
            full: "Mathematical_Operators",
            abbr: "Math_Operators",
        },
        Blk::MayanNumerals => PropertyName {
            full: "Mayan_Numerals",
            abbr: "Mayan_Numerals",
        },
        Blk::Medefaidrin => PropertyName {
            full: "Medefaidrin",
            abbr: "Medefaidrin",
        },
        Blk::MeeteiMayek => PropertyName {
            full: "Meetei_Mayek",
            abbr: "Meetei_Mayek",
        },
        Blk::MeeteiMayekExt => PropertyName {
            full: "Meetei_Mayek_Extensions",
            abbr: "Meetei_Mayek_Ext",
        },
        Blk::MendeKikakui => PropertyName {
            full: "Mende_Kikakui",
            abbr: "Mende_Kikakui",
        },
        Blk::MeroiticCursive => PropertyName {
            full: "Meroitic_Cursive",
            abbr: "Meroitic_Cursive",
        },
        Blk::MeroiticHieroglyphs => PropertyName {
            full: "Meroitic_Hieroglyphs",
            abbr: "Meroitic_Hieroglyphs",
        },
        Blk::Miao => PropertyName {
            full: "Miao",
            abbr: "Miao",
        },
        Blk::MiscArrows => PropertyName {
            full: "Miscellaneous_Symbols_And_Arrows",
            abbr: "Misc_Arrows",
        },
        Blk::MiscMathSymbolsA => PropertyName {
            full: "Miscellaneous_Mathematical_Symbols_A",
            abbr: "Misc_Math_Symbols_A",
        },
        Blk::MiscMathSymbolsB => PropertyName {
            full: "Miscellaneous_Mathematical_Symbols_B",
            abbr: "Misc_Math_Symbols_B",
        },
        Blk::MiscPictographs => PropertyName {
            full: "Miscellaneous_Symbols_And_Pictographs",
            abbr: "Misc_Pictographs",
        },
        Blk::MiscSymbols => PropertyName {
            full: "Miscellaneous_Symbols",
            abbr: "Misc_Symbols",
        },
        Blk::MiscTechnical => PropertyName {
            full: "Miscellaneous_Technical",
            abbr: "Misc_Technical",
        },
        Blk::Modi => PropertyName {
            full: "Modi",
            abbr: "Modi",
        },
        Blk::ModifierLetters => PropertyName {
            full: "Spacing_Modifier_Letters",
            abbr: "Modifier_Letters",
        },
        Blk::ModifierToneLetters => PropertyName {
            full: "Modifier_Tone_Letters",
            abbr: "Modifier_Tone_Letters",
        },
        Blk::Mongolian => PropertyName {
            full: "Mongolian",
            abbr: "Mongolian",
        },
        Blk::MongolianSup => PropertyName {
            full: "Mongolian_Supplement",
            abbr: "Mongolian_Sup",
        },
        Blk::Mro => PropertyName {
            full: "Mro",
            abbr: "Mro",
        },
        Blk::Multani => PropertyName {
            full: "Multani",
            abbr: "Multani",
        },
        Blk::Music => PropertyName {
            full: "Musical_Symbols",
            abbr: "Music",
        },
        Blk::Myanmar => PropertyName {
            full: "Myanmar",
            abbr: "Myanmar",
        },
        Blk::MyanmarExtA => PropertyName {
            full: "Myanmar_Extended_A",
            abbr: "Myanmar_Ext_A",
        },
        Blk::MyanmarExtB => PropertyName {
            full: "Myanmar_Extended_B",
            abbr: "Myanmar_Ext_B",
        },
        Blk::Nabataean => PropertyName {
            full: "Nabataean",
            abbr: "Nabataean",
        },
        Blk::NagMundari => PropertyName {
            full: "Nag_Mundari",
            abbr: "Nag_Mundari",
        },
        Blk::Nandinagari => PropertyName {
            full: "Nandinagari",
            abbr: "Nandinagari",
        },
        Blk::Nb => PropertyName {
            full: "No_Block",
            abbr: "NB",
        },
        Blk::NewTaiLue => PropertyName {
            full: "New_Tai_Lue",
            abbr: "New_Tai_Lue",
        },
        Blk::Newa => PropertyName {
            full: "Newa",
            abbr: "Newa",
        },
        Blk::Nko => PropertyName {
            full: "NKo",
            abbr: "NKo",
        },
        Blk::NumberForms => PropertyName {
            full: "Number_Forms",
            abbr: "Number_Forms",
        },
        Blk::Nushu => PropertyName {
            full: "Nushu",
            abbr: "Nushu",
        },
        Blk::NyiakengPuachueHmong => PropertyName {
            full: "Nyiakeng_Puachue_Hmong",
            abbr: "Nyiakeng_Puachue_Hmong",
        },
        Blk::Ocr => PropertyName {
            full: "Optical_Character_Recognition",
            abbr: "OCR",
        },
        Blk::Ogham => PropertyName {
            full: "Ogham",
            abbr: "Ogham",
        },
        Blk::OlChiki => PropertyName {
            full: "Ol_Chiki",
            abbr: "Ol_Chiki",
        },
        Blk::OldHungarian => PropertyName {
            full: "Old_Hungarian",
            abbr: "Old_Hungarian",
        },
        Blk::OldItalic => PropertyName {
            full: "Old_Italic",
            abbr: "Old_Italic",
        },
        Blk::OldNorthArabian => PropertyName {
            full: "Old_North_Arabian",
            abbr: "Old_North_Arabian",
        },
        Blk::OldPermic => PropertyName {
            full: "Old_Permic",
            abbr: "Old_Permic",
        },
        Blk::OldPersian => PropertyName {
            full: "Old_Persian",
            abbr: "Old_Persian",
        },
        Blk::OldSogdian => PropertyName {
            full: "Old_Sogdian",
            abbr: "Old_Sogdian",
        },
        Blk::OldSouthArabian => PropertyName {
            full: "Old_South_Arabian",
            abbr: "Old_South_Arabian",
        },
        Blk::OldTurkic => PropertyName {
            full: "Old_Turkic",
            abbr: "Old_Turkic",
        },
        Blk::OldUyghur => PropertyName {
            full: "Old_Uyghur",
            abbr: "Old_Uyghur",
        },
        Blk::Oriya => PropertyName {
            full: "Oriya",
            abbr: "Oriya",
        },
        Blk::OrnamentalDingbats => PropertyName {
            full: "Ornamental_Dingbats",
            abbr: "Ornamental_Dingbats",
        },
        Blk::Osage => PropertyName {
            full: "Osage",
            abbr: "Osage",
        },
        Blk::Osmanya => PropertyName {
            full: "Osmanya",
            abbr: "Osmanya",
        },
        Blk::OttomanSiyaqNumbers => PropertyName {
            full: "Ottoman_Siyaq_Numbers",
            abbr: "Ottoman_Siyaq_Numbers",
        },
        Blk::PahawhHmong => PropertyName {
            full: "Pahawh_Hmong",
            abbr: "Pahawh_Hmong",
        },
        Blk::Palmyrene => PropertyName {
            full: "Palmyrene",
            abbr: "Palmyrene",
        },
        Blk::PauCinHau => PropertyName {
            full: "Pau_Cin_Hau",
            abbr: "Pau_Cin_Hau",
        },
        Blk::PhagsPa => PropertyName {
            full: "Phags_Pa",
            abbr: "Phags_Pa",
        },
        Blk::Phaistos => PropertyName {
            full: "Phaistos_Disc",
            abbr: "Phaistos",
        },
        Blk::Phoenician => PropertyName {
            full: "Phoenician",
            abbr: "Phoenician",
        },
        Blk::PhoneticExt => PropertyName {
            full: "Phonetic_Extensions",
            abbr: "Phonetic_Ext",
        },
        Blk::PhoneticExtSup => PropertyName {
            full: "Phonetic_Extensions_Supplement",
            abbr: "Phonetic_Ext_Sup",
        },
        Blk::PlayingCards => PropertyName {
            full: "Playing_Cards",
            abbr: "Playing_Cards",
        },
        Blk::PsalterPahlavi => PropertyName {
            full: "Psalter_Pahlavi",
            abbr: "Psalter_Pahlavi",
        },
        Blk::Pua => PropertyName {
            full: "Private_Use_Area",
            abbr: "PUA",
        },
        Blk::Punctuation => PropertyName {
            full: "General_Punctuation",
            abbr: "Punctuation",
        },
        Blk::Rejang => PropertyName {
            full: "Rejang",
            abbr: "Rejang",
        },
        Blk::Rumi => PropertyName {
            full: "Rumi_Numeral_Symbols",
            abbr: "Rumi",
        },
        Blk::Runic => PropertyName {
            full: "Runic",
            abbr: "Runic",
        },
        Blk::Samaritan => PropertyName {
            full: "Samaritan",
            abbr: "Samaritan",
        },
        Blk::Saurashtra => PropertyName {
            full: "Saurashtra",
            abbr: "Saurashtra",
        },
        Blk::Sharada => PropertyName {
            full: "Sharada",
            abbr: "Sharada",
        },
        Blk::Shavian => PropertyName {
            full: "Shavian",
            abbr: "Shavian",
        },
        Blk::ShorthandFormatControls => PropertyName {
            full: "Shorthand_Format_Controls",
            abbr: "Shorthand_Format_Controls",
        },
        Blk::Siddham => PropertyName {
            full: "Siddham",
            abbr: "Siddham",
        },
        Blk::Sinhala => PropertyName {
            full: "Sinhala",
            abbr: "Sinhala",
        },
        Blk::SinhalaArchaicNumbers => PropertyName {
            full: "Sinhala_Archaic_Numbers",
            abbr: "Sinhala_Archaic_Numbers",
        },
        Blk::SmallForms => PropertyName {
            full: "Small_Form_Variants",
            abbr: "Small_Forms",
        },
        Blk::SmallKanaExt => PropertyName {
            full: "Small_Kana_Extension",
            abbr: "Small_Kana_Ext",
        },
        Blk::Sogdian => PropertyName {
            full: "Sogdian",
            abbr: "Sogdian",
        },
        Blk::SoraSompeng => PropertyName {
            full: "Sora_Sompeng",
            abbr: "Sora_Sompeng",
        },
        Blk::Soyombo => PropertyName {
            full: "Soyombo",
            abbr: "Soyombo",
        },
        Blk::Specials => PropertyName {
            full: "Specials",
            abbr: "Specials",
        },
        Blk::Sundanese => PropertyName {
            full: "Sundanese",
            abbr: "Sundanese",
        },
        Blk::SundaneseSup => PropertyName {
            full: "Sundanese_Supplement",
            abbr: "Sundanese_Sup",
        },
        Blk::SupArrowsA => PropertyName {
            full: "Supplemental_Arrows_A",
            abbr: "Sup_Arrows_A",
        },
        Blk::SupArrowsB => PropertyName {
            full: "Supplemental_Arrows_B",
            abbr: "Sup_Arrows_B",
        },
        Blk::SupArrowsC => PropertyName {
            full: "Supplemental_Arrows_C",
            abbr: "Sup_Arrows_C",
        },
        Blk::SupMathOperators => PropertyName {
            full: "Supplemental_Mathematical_Operators",
            abbr: "Sup_Math_Operators",
        },
        Blk::SupPuaA => PropertyName {
            full: "Supplementary_Private_Use_Area_A",
            abbr: "Sup_PUA_A",
        },
        Blk::SupPuaB => PropertyName {
            full: "Supplementary_Private_Use_Area_B",
            abbr: "Sup_PUA_B",
        },
        Blk::SupPunctuation => PropertyName {
            full: "Supplemental_Punctuation",
            abbr: "Sup_Punctuation",
        },
        Blk::SupSymbolsAndPictographs => PropertyName {
            full: "Supplemental_Symbols_And_Pictographs",
            abbr: "Sup_Symbols_And_Pictographs",
        },
        Blk::SuperAndSub => PropertyName {
            full: "Superscripts_And_Subscripts",
            abbr: "Super_And_Sub",
        },
        Blk::SuttonSignwriting => PropertyName {
            full: "Sutton_SignWriting",
            abbr: "Sutton_SignWriting",
        },
        Blk::SylotiNagri => PropertyName {
            full: "Syloti_Nagri",
            abbr: "Syloti_Nagri",
        },
        Blk::SymbolsAndPictographsExtA => PropertyName {
            full: "Symbols_And_Pictographs_Extended_A",
            abbr: "Symbols_And_Pictographs_Ext_A",
        },
        Blk::SymbolsForLegacyComputing => PropertyName {
            full: "Symbols_For_Legacy_Computing",
            abbr: "Symbols_For_Legacy_Computing",
        },
        Blk::Syriac => PropertyName {
            full: "Syriac",
            abbr: "Syriac",
        },
        Blk::SyriacSup => PropertyName {
            full: "Syriac_Supplement",
            abbr: "Syriac_Sup",
        },
        Blk::Tagalog => PropertyName {
            full: "Tagalog",
            abbr: "Tagalog",
        },
        Blk::Tagbanwa => PropertyName {
            full: "Tagbanwa",
            abbr: "Tagbanwa",
        },
        Blk::Tags => PropertyName {
            full: "Tags",
            abbr: "Tags",
        },
        Blk::TaiLe => PropertyName {
            full: "Tai_Le",
            abbr: "Tai_Le",
        },
        Blk::TaiTham => PropertyName {
            full: "Tai_Tham",
            abbr: "Tai_Tham",
        },
        Blk::TaiViet => PropertyName {
            full: "Tai_Viet",
            abbr: "Tai_Viet",
        },
        Blk::TaiXuanJing => PropertyName {
            full: "Tai_Xuan_Jing_Symbols",
            abbr: "Tai_Xuan_Jing",
        },
        Blk::Takri => PropertyName {
            full: "Takri",
            abbr: "Takri",
        },
        Blk::Tamil => PropertyName {
            full: "Tamil",
            abbr: "Tamil",
        },
        Blk::TamilSup => PropertyName {
            full: "Tamil_Supplement",
            abbr: "Tamil_Sup",
        },
        Blk::Tangsa => PropertyName {
            full: "Tangsa",
            abbr: "Tangsa",
        },
        Blk::Tangut => PropertyName {
            full: "Tangut",
            abbr: "Tangut",
        },
        Blk::TangutComponents => PropertyName {
            full: "Tangut_Components",
            abbr: "Tangut_Components",
        },
        Blk::TangutSup => PropertyName {
            full: "Tangut_Supplement",
            abbr: "Tangut_Sup",
        },
        Blk::Telugu => PropertyName {
            full: "Telugu",
            abbr: "Telugu",
        },
        Blk::Thaana => PropertyName {
            full: "Thaana",
            abbr: "Thaana",
        },
        Blk::Thai => PropertyName {
            full: "Thai",
            abbr: "Thai",
        },
        Blk::Tibetan => PropertyName {
            full: "Tibetan",
            abbr: "Tibetan",
        },
        Blk::Tifinagh => PropertyName {
            full: "Tifinagh",
            abbr: "Tifinagh",
        },
        Blk::Tirhuta => PropertyName {
            full: "Tirhuta",
            abbr: "Tirhuta",
        },
        Blk::Toto => PropertyName {
            full: "Toto",
            abbr: "Toto",
        },
        Blk::TransportAndMap => PropertyName {
            full: "Transport_And_Map_Symbols",
            abbr: "Transport_And_Map",
        },
        Blk::Ucas => PropertyName {
            full: "Unified_Canadian_Aboriginal_Syllabics",
            abbr: "UCAS",
        },
        Blk::UcasExt => PropertyName {
            full: "Unified_Canadian_Aboriginal_Syllabics_Extended",
            abbr: "UCAS_Ext",
        },
        Blk::UcasExtA => PropertyName {
            full: "Unified_Canadian_Aboriginal_Syllabics_Extended_A",
            abbr: "UCAS_Ext_A",
        },
        Blk::Ugaritic => PropertyName {
            full: "Ugaritic",
            abbr: "Ugaritic",
        },
        Blk::Vai => PropertyName {
            full: "Vai",
            abbr: "Vai",
        },
        Blk::VedicExt => PropertyName {
            full: "Vedic_Extensions",
            abbr: "Vedic_Ext",
        },
        Blk::VerticalForms => PropertyName {
            full: "Vertical_Forms",
            abbr: "Vertical_Forms",
        },
        Blk::Vithkuqi => PropertyName {
            full: "Vithkuqi",
            abbr: "Vithkuqi",
        },
        Blk::Vs => PropertyName {
            full: "Variation_Selectors",
            abbr: "VS",
        },
        Blk::VsSup => PropertyName {
            full: "Variation_Selectors_Supplement",
            abbr: "VS_Sup",
        },
        Blk::Wancho => PropertyName {
            full: "Wancho",
            abbr: "Wancho",
        },
        Blk::WarangCiti => PropertyName {
            full: "Warang_Citi",
            abbr: "Warang_Citi",
        },
        Blk::Yezidi => PropertyName {
            full: "Yezidi",
            abbr: "Yezidi",
        },
        Blk::YiRadicals => PropertyName {
            full: "Yi_Radicals",
            abbr: "Yi_Radicals",
        },
        Blk::YiSyllables => PropertyName {
            full: "Yi_Syllables",
            abbr: "Yi_Syllables",
        },
        Blk::Yijing => PropertyName {
            full: "Yijing_Hexagram_Symbols",
            abbr: "Yijing",
        },
        Blk::ZanabazarSquare => PropertyName {
            full: "Zanabazar_Square",
            abbr: "Zanabazar_Square",
        },
        Blk::ZnamennyMusic => PropertyName {
            full: "Znamenny_Musical_Notation",
            abbr: "Znamenny_Music",
        },
    }
}

pub(crate) fn sc_name(prop: Sc) -> PropertyName {
    // This method is generated by `gen-sc-name.py` script.
    match prop {
        Sc::Adlm => PropertyName {
            full: "Adlam",
            abbr: "Adlm",
        },
        Sc::Aghb => PropertyName {
            full: "Caucasian_Albanian",
            abbr: "Aghb",
        },
        Sc::Ahom => PropertyName {
            full: "Ahom",
            abbr: "Ahom",
        },
        Sc::Arab => PropertyName {
            full: "Arabic",
            abbr: "Arab",
        },
        Sc::Armi => PropertyName {
            full: "Imperial_Aramaic",
            abbr: "Armi",
        },
        Sc::Armn => PropertyName {
            full: "Armenian",
            abbr: "Armn",
        },
        Sc::Avst => PropertyName {
            full: "Avestan",
            abbr: "Avst",
        },
        Sc::Bali => PropertyName {
            full: "Balinese",
            abbr: "Bali",
        },
        Sc::Bamu => PropertyName {
            full: "Bamum",
            abbr: "Bamu",
        },
        Sc::Bass => PropertyName {
            full: "Bassa_Vah",
            abbr: "Bass",
        },
        Sc::Batk => PropertyName {
            full: "Batak",
            abbr: "Batk",
        },
        Sc::Beng => PropertyName {
            full: "Bengali",
            abbr: "Beng",
        },
        Sc::Bhks => PropertyName {
            full: "Bhaiksuki",
            abbr: "Bhks",
        },
        Sc::Bopo => PropertyName {
            full: "Bopomofo",
            abbr: "Bopo",
        },
        Sc::Brah => PropertyName {
            full: "Brahmi",
            abbr: "Brah",
        },
        Sc::Brai => PropertyName {
            full: "Braille",
            abbr: "Brai",
        },
        Sc::Bugi => PropertyName {
            full: "Buginese",
            abbr: "Bugi",
        },
        Sc::Buhd => PropertyName {
            full: "Buhid",
            abbr: "Buhd",
        },
        Sc::Cakm => PropertyName {
            full: "Chakma",
            abbr: "Cakm",
        },
        Sc::Cans => PropertyName {
            full: "Canadian_Aboriginal",
            abbr: "Cans",
        },
        Sc::Cari => PropertyName {
            full: "Carian",
            abbr: "Cari",
        },
        Sc::Cham => PropertyName {
            full: "Cham",
            abbr: "Cham",
        },
        Sc::Cher => PropertyName {
            full: "Cherokee",
            abbr: "Cher",
        },
        Sc::Chrs => PropertyName {
            full: "Chorasmian",
            abbr: "Chrs",
        },
        Sc::Copt => PropertyName {
            full: "Coptic",
            abbr: "Copt",
        },
        Sc::Cpmn => PropertyName {
            full: "Cypro_Minoan",
            abbr: "Cpmn",
        },
        Sc::Cprt => PropertyName {
            full: "Cypriot",
            abbr: "Cprt",
        },
        Sc::Cyrl => PropertyName {
            full: "Cyrillic",
            abbr: "Cyrl",
        },
        Sc::Deva => PropertyName {
            full: "Devanagari",
            abbr: "Deva",
        },
        Sc::Diak => PropertyName {
            full: "Dives_Akuru",
            abbr: "Diak",
        },
        Sc::Dogr => PropertyName {
            full: "Dogra",
            abbr: "Dogr",
        },
        Sc::Dsrt => PropertyName {
            full: "Deseret",
            abbr: "Dsrt",
        },
        Sc::Dupl => PropertyName {
            full: "Duployan",
            abbr: "Dupl",
        },
        Sc::Egyp => PropertyName {
            full: "Egyptian_Hieroglyphs",
            abbr: "Egyp",
        },
        Sc::Elba => PropertyName {
            full: "Elbasan",
            abbr: "Elba",
        },
        Sc::Elym => PropertyName {
            full: "Elymaic",
            abbr: "Elym",
        },
        Sc::Ethi => PropertyName {
            full: "Ethiopic",
            abbr: "Ethi",
        },
        Sc::Geor => PropertyName {
            full: "Georgian",
            abbr: "Geor",
        },
        Sc::Glag => PropertyName {
            full: "Glagolitic",
            abbr: "Glag",
        },
        Sc::Gong => PropertyName {
            full: "Gunjala_Gondi",
            abbr: "Gong",
        },
        Sc::Gonm => PropertyName {
            full: "Masaram_Gondi",
            abbr: "Gonm",
        },
        Sc::Goth => PropertyName {
            full: "Gothic",
            abbr: "Goth",
        },
        Sc::Gran => PropertyName {
            full: "Grantha",
            abbr: "Gran",
        },
        Sc::Grek => PropertyName {
            full: "Greek",
            abbr: "Grek",
        },
        Sc::Gujr => PropertyName {
            full: "Gujarati",
            abbr: "Gujr",
        },
        Sc::Guru => PropertyName {
            full: "Gurmukhi",
            abbr: "Guru",
        },
        Sc::Hang => PropertyName {
            full: "Hangul",
            abbr: "Hang",
        },
        Sc::Hani => PropertyName {
            full: "Han",
            abbr: "Hani",
        },
        Sc::Hano => PropertyName {
            full: "Hanunoo",
            abbr: "Hano",
        },
        Sc::Hatr => PropertyName {
            full: "Hatran",
            abbr: "Hatr",
        },
        Sc::Hebr => PropertyName {
            full: "Hebrew",
            abbr: "Hebr",
        },
        Sc::Hira => PropertyName {
            full: "Hiragana",
            abbr: "Hira",
        },
        Sc::Hluw => PropertyName {
            full: "Anatolian_Hieroglyphs",
            abbr: "Hluw",
        },
        Sc::Hmng => PropertyName {
            full: "Pahawh_Hmong",
            abbr: "Hmng",
        },
        Sc::Hmnp => PropertyName {
            full: "Nyiakeng_Puachue_Hmong",
            abbr: "Hmnp",
        },
        Sc::Hrkt => PropertyName {
            full: "Katakana_Or_Hiragana",
            abbr: "Hrkt",
        },
        Sc::Hung => PropertyName {
            full: "Old_Hungarian",
            abbr: "Hung",
        },
        Sc::Ital => PropertyName {
            full: "Old_Italic",
            abbr: "Ital",
        },
        Sc::Java => PropertyName {
            full: "Javanese",
            abbr: "Java",
        },
        Sc::Kali => PropertyName {
            full: "Kayah_Li",
            abbr: "Kali",
        },
        Sc::Kana => PropertyName {
            full: "Katakana",
            abbr: "Kana",
        },
        Sc::Kawi => PropertyName {
            full: "Kawi",
            abbr: "Kawi",
        },
        Sc::Khar => PropertyName {
            full: "Kharoshthi",
            abbr: "Khar",
        },
        Sc::Khmr => PropertyName {
            full: "Khmer",
            abbr: "Khmr",
        },
        Sc::Khoj => PropertyName {
            full: "Khojki",
            abbr: "Khoj",
        },
        Sc::Kits => PropertyName {
            full: "Khitan_Small_Script",
            abbr: "Kits",
        },
        Sc::Knda => PropertyName {
            full: "Kannada",
            abbr: "Knda",
        },
        Sc::Kthi => PropertyName {
            full: "Kaithi",
            abbr: "Kthi",
        },
        Sc::Lana => PropertyName {
            full: "Tai_Tham",
            abbr: "Lana",
        },
        Sc::Laoo => PropertyName {
            full: "Lao",
            abbr: "Laoo",
        },
        Sc::Latn => PropertyName {
            full: "Latin",
            abbr: "Latn",
        },
        Sc::Lepc => PropertyName {
            full: "Lepcha",
            abbr: "Lepc",
        },
        Sc::Limb => PropertyName {
            full: "Limbu",
            abbr: "Limb",
        },
        Sc::Lina => PropertyName {
            full: "Linear_A",
            abbr: "Lina",
        },
        Sc::Linb => PropertyName {
            full: "Linear_B",
            abbr: "Linb",
        },
        Sc::Lisu => PropertyName {
            full: "Lisu",
            abbr: "Lisu",
        },
        Sc::Lyci => PropertyName {
            full: "Lycian",
            abbr: "Lyci",
        },
        Sc::Lydi => PropertyName {
            full: "Lydian",
            abbr: "Lydi",
        },
        Sc::Mahj => PropertyName {
            full: "Mahajani",
            abbr: "Mahj",
        },
        Sc::Maka => PropertyName {
            full: "Makasar",
            abbr: "Maka",
        },
        Sc::Mand => PropertyName {
            full: "Mandaic",
            abbr: "Mand",
        },
        Sc::Mani => PropertyName {
            full: "Manichaean",
            abbr: "Mani",
        },
        Sc::Marc => PropertyName {
            full: "Marchen",
            abbr: "Marc",
        },
        Sc::Medf => PropertyName {
            full: "Medefaidrin",
            abbr: "Medf",
        },
        Sc::Mend => PropertyName {
            full: "Mende_Kikakui",
            abbr: "Mend",
        },
        Sc::Merc => PropertyName {
            full: "Meroitic_Cursive",
            abbr: "Merc",
        },
        Sc::Mero => PropertyName {
            full: "Meroitic_Hieroglyphs",
            abbr: "Mero",
        },
        Sc::Mlym => PropertyName {
            full: "Malayalam",
            abbr: "Mlym",
        },
        Sc::Modi => PropertyName {
            full: "Modi",
            abbr: "Modi",
        },
        Sc::Mong => PropertyName {
            full: "Mongolian",
            abbr: "Mong",
        },
        Sc::Mroo => PropertyName {
            full: "Mro",
            abbr: "Mroo",
        },
        Sc::Mtei => PropertyName {
            full: "Meetei_Mayek",
            abbr: "Mtei",
        },
        Sc::Mult => PropertyName {
            full: "Multani",
            abbr: "Mult",
        },
        Sc::Mymr => PropertyName {
            full: "Myanmar",
            abbr: "Mymr",
        },
        Sc::Nagm => PropertyName {
            full: "Nag_Mundari",
            abbr: "Nagm",
        },
        Sc::Nand => PropertyName {
            full: "Nandinagari",
            abbr: "Nand",
        },
        Sc::Narb => PropertyName {
            full: "Old_North_Arabian",
            abbr: "Narb",
        },
        Sc::Nbat => PropertyName {
            full: "Nabataean",
            abbr: "Nbat",
        },
        Sc::Newa => PropertyName {
            full: "Newa",
            abbr: "Newa",
        },
        Sc::Nkoo => PropertyName {
            full: "Nko",
            abbr: "Nkoo",
        },
        Sc::Nshu => PropertyName {
            full: "Nushu",
            abbr: "Nshu",
        },
        Sc::Ogam => PropertyName {
            full: "Ogham",
            abbr: "Ogam",
        },
        Sc::Olck => PropertyName {
            full: "Ol_Chiki",
            abbr: "Olck",
        },
        Sc::Orkh => PropertyName {
            full: "Old_Turkic",
            abbr: "Orkh",
        },
        Sc::Orya => PropertyName {
            full: "Oriya",
            abbr: "Orya",
        },
        Sc::Osge => PropertyName {
            full: "Osage",
            abbr: "Osge",
        },
        Sc::Osma => PropertyName {
            full: "Osmanya",
            abbr: "Osma",
        },
        Sc::Ougr => PropertyName {
            full: "Old_Uyghur",
            abbr: "Ougr",
        },
        Sc::Palm => PropertyName {
            full: "Palmyrene",
            abbr: "Palm",
        },
        Sc::Pauc => PropertyName {
            full: "Pau_Cin_Hau",
            abbr: "Pauc",
        },
        Sc::Perm => PropertyName {
            full: "Old_Permic",
            abbr: "Perm",
        },
        Sc::Phag => PropertyName {
            full: "Phags_Pa",
            abbr: "Phag",
        },
        Sc::Phli => PropertyName {
            full: "Inscriptional_Pahlavi",
            abbr: "Phli",
        },
        Sc::Phlp => PropertyName {
            full: "Psalter_Pahlavi",
            abbr: "Phlp",
        },
        Sc::Phnx => PropertyName {
            full: "Phoenician",
            abbr: "Phnx",
        },
        Sc::Plrd => PropertyName {
            full: "Miao",
            abbr: "Plrd",
        },
        Sc::Prti => PropertyName {
            full: "Inscriptional_Parthian",
            abbr: "Prti",
        },
        Sc::Rjng => PropertyName {
            full: "Rejang",
            abbr: "Rjng",
        },
        Sc::Rohg => PropertyName {
            full: "Hanifi_Rohingya",
            abbr: "Rohg",
        },
        Sc::Runr => PropertyName {
            full: "Runic",
            abbr: "Runr",
        },
        Sc::Samr => PropertyName {
            full: "Samaritan",
            abbr: "Samr",
        },
        Sc::Sarb => PropertyName {
            full: "Old_South_Arabian",
            abbr: "Sarb",
        },
        Sc::Saur => PropertyName {
            full: "Saurashtra",
            abbr: "Saur",
        },
        Sc::Sgnw => PropertyName {
            full: "SignWriting",
            abbr: "Sgnw",
        },
        Sc::Shaw => PropertyName {
            full: "Shavian",
            abbr: "Shaw",
        },
        Sc::Shrd => PropertyName {
            full: "Sharada",
            abbr: "Shrd",
        },
        Sc::Sidd => PropertyName {
            full: "Siddham",
            abbr: "Sidd",
        },
        Sc::Sind => PropertyName {
            full: "Khudawadi",
            abbr: "Sind",
        },
        Sc::Sinh => PropertyName {
            full: "Sinhala",
            abbr: "Sinh",
        },
        Sc::Sogd => PropertyName {
            full: "Sogdian",
            abbr: "Sogd",
        },
        Sc::Sogo => PropertyName {
            full: "Old_Sogdian",
            abbr: "Sogo",
        },
        Sc::Sora => PropertyName {
            full: "Sora_Sompeng",
            abbr: "Sora",
        },
        Sc::Soyo => PropertyName {
            full: "Soyombo",
            abbr: "Soyo",
        },
        Sc::Sund => PropertyName {
            full: "Sundanese",
            abbr: "Sund",
        },
        Sc::Sylo => PropertyName {
            full: "Syloti_Nagri",
            abbr: "Sylo",
        },
        Sc::Syrc => PropertyName {
            full: "Syriac",
            abbr: "Syrc",
        },
        Sc::Tagb => PropertyName {
            full: "Tagbanwa",
            abbr: "Tagb",
        },
        Sc::Takr => PropertyName {
            full: "Takri",
            abbr: "Takr",
        },
        Sc::Tale => PropertyName {
            full: "Tai_Le",
            abbr: "Tale",
        },
        Sc::Talu => PropertyName {
            full: "New_Tai_Lue",
            abbr: "Talu",
        },
        Sc::Taml => PropertyName {
            full: "Tamil",
            abbr: "Taml",
        },
        Sc::Tang => PropertyName {
            full: "Tangut",
            abbr: "Tang",
        },
        Sc::Tavt => PropertyName {
            full: "Tai_Viet",
            abbr: "Tavt",
        },
        Sc::Telu => PropertyName {
            full: "Telugu",
            abbr: "Telu",
        },
        Sc::Tfng => PropertyName {
            full: "Tifinagh",
            abbr: "Tfng",
        },
        Sc::Tglg => PropertyName {
            full: "Tagalog",
            abbr: "Tglg",
        },
        Sc::Thaa => PropertyName {
            full: "Thaana",
            abbr: "Thaa",
        },
        Sc::Thai => PropertyName {
            full: "Thai",
            abbr: "Thai",
        },
        Sc::Tibt => PropertyName {
            full: "Tibetan",
            abbr: "Tibt",
        },
        Sc::Tirh => PropertyName {
            full: "Tirhuta",
            abbr: "Tirh",
        },
        Sc::Tnsa => PropertyName {
            full: "Tangsa",
            abbr: "Tnsa",
        },
        Sc::Toto => PropertyName {
            full: "Toto",
            abbr: "Toto",
        },
        Sc::Ugar => PropertyName {
            full: "Ugaritic",
            abbr: "Ugar",
        },
        Sc::Vaii => PropertyName {
            full: "Vai",
            abbr: "Vaii",
        },
        Sc::Vith => PropertyName {
            full: "Vithkuqi",
            abbr: "Vith",
        },
        Sc::Wara => PropertyName {
            full: "Warang_Citi",
            abbr: "Wara",
        },
        Sc::Wcho => PropertyName {
            full: "Wancho",
            abbr: "Wcho",
        },
        Sc::Xpeo => PropertyName {
            full: "Old_Persian",
            abbr: "Xpeo",
        },
        Sc::Xsux => PropertyName {
            full: "Cuneiform",
            abbr: "Xsux",
        },
        Sc::Yezi => PropertyName {
            full: "Yezidi",
            abbr: "Yezi",
        },
        Sc::Yiii => PropertyName {
            full: "Yi",
            abbr: "Yiii",
        },
        Sc::Zanb => PropertyName {
            full: "Zanabazar_Square",
            abbr: "Zanb",
        },
        Sc::Zinh => PropertyName {
            full: "Inherited",
            abbr: "Zinh",
        },
        Sc::Zyyy => PropertyName {
            full: "Common",
            abbr: "Zyyy",
        },
        Sc::Zzzz => PropertyName {
            full: "Unknown",
            abbr: "Zzzz",
        },
    }
}

pub(crate) fn age_name(prop: Age) -> PropertyName {
    match prop {
        Age::V1_1 => PropertyName {
            full: "V1_1",
            abbr: "1.1",
        },
        Age::V2_0 => PropertyName {
            full: "V2_0",
            abbr: "2.0",
        },
        Age::V2_1 => PropertyName {
            full: "V2_1",
            abbr: "2.1",
        },
        Age::V3_0 => PropertyName {
            full: "V3_0",
            abbr: "3.0",
        },
        Age::V3_1 => PropertyName {
            full: "V3_1",
            abbr: "3.1",
        },
        Age::V3_2 => PropertyName {
            full: "V3_2",
            abbr: "3.2",
        },
        Age::V4_0 => PropertyName {
            full: "V4_0",
            abbr: "4.0",
        },
        Age::V4_1 => PropertyName {
            full: "V4_1",
            abbr: "4.1",
        },
        Age::V5_0 => PropertyName {
            full: "V5_0",
            abbr: "5.0",
        },
        Age::V5_1 => PropertyName {
            full: "V5_1",
            abbr: "5.1",
        },
        Age::V5_2 => PropertyName {
            full: "V5_2",
            abbr: "5.2",
        },
        Age::V6_0 => PropertyName {
            full: "V6_0",
            abbr: "6.0",
        },
        Age::V6_1 => PropertyName {
            full: "V6_1",
            abbr: "6.1",
        },
        Age::V6_2 => PropertyName {
            full: "V6_2",
            abbr: "6.2",
        },
        Age::V6_3 => PropertyName {
            full: "V6_3",
            abbr: "6.3",
        },
        Age::V7_0 => PropertyName {
            full: "V7_0",
            abbr: "7.0",
        },
        Age::V8_0 => PropertyName {
            full: "V8_0",
            abbr: "8.0",
        },
        Age::V9_0 => PropertyName {
            full: "V9_0",
            abbr: "9.0",
        },
        Age::V10_0 => PropertyName {
            full: "V10_0",
            abbr: "10.0",
        },
        Age::V11_0 => PropertyName {
            full: "V11_0",
            abbr: "11.0",
        },
        Age::V12_0 => PropertyName {
            full: "V12_0",
            abbr: "12.0",
        },
        Age::V12_1 => PropertyName {
            full: "V12_1",
            abbr: "12.1",
        },
        Age::V13_0 => PropertyName {
            full: "V13_0",
            abbr: "13.0",
        },
        Age::V14_0 => PropertyName {
            full: "V14_0",
            abbr: "14.0",
        },
        Age::V15_0 => PropertyName {
            full: "V15_0",
            abbr: "15.0",
        },
        Age::NA => PropertyName {
            full: "Unassigned",
            abbr: "NA",
        },
     }
}

pub(crate) fn wb_name(prop: Wb) -> PropertyName {
    match prop {
        Wb::CR => PropertyName {
            full: "CR",
            abbr: "CR",
        },
        Wb::DQ => PropertyName {
            full: "Double_Quote",
            abbr: "DQ",
        },
        Wb::EB => PropertyName {
            full: "E_Base",
            abbr: "EB",
        },
        Wb::EBG => PropertyName {
            full: "E_Base_GAZ",
            abbr: "EBG",
        },
        Wb::EM => PropertyName {
            full: "E_Modifier",
            abbr: "EM",
        },
        Wb::EX => PropertyName {
            full: "ExtendNumLet",
            abbr: "EX",
        },
        Wb::Extend => PropertyName {
            full: "Extend",
            abbr: "Extend",
        },
        Wb::FO => PropertyName {
            full: "Format",
            abbr: "FO",
        },
        Wb::GAZ => PropertyName {
            full: "Glue_After_Zwj",
            abbr: "GAZ",
        },
        Wb::HL => PropertyName {
            full: "Hebrew_Letter",
            abbr: "HL",
        },
        Wb::KA => PropertyName {
            full: "Katakana",
            abbr: "KA",
        },
        Wb::LE => PropertyName {
            full: "ALetter",
            abbr: "LE",
        },
        Wb::LF => PropertyName {
            full: "LF",
            abbr: "LF",
        },
        Wb::MB => PropertyName {
            full: "MidNumLet",
            abbr: "MB",
        },
        Wb::ML => PropertyName {
            full: "MidLetter",
            abbr: "ML",
        },
        Wb::MN => PropertyName {
            full: "MidNum",
            abbr: "MN",
        },
        Wb::NL => PropertyName {
            full: "Newline",
            abbr: "NL",
        },
        Wb::NU => PropertyName {
            full: "Numeric",
            abbr: "NU",
        },
        Wb::RI => PropertyName {
            full: "Regional_Indicator",
            abbr: "RI",
        },
        Wb::SQ => PropertyName {
            full: "Single_Quote",
            abbr: "SQ",
        },
        Wb::WSegSpace => PropertyName {
            full: "WSegSpace",
            abbr: "WSegSpace",
        },
        Wb::XX => PropertyName {
            full: "Other",
            abbr: "XX",
        },
        Wb::ZWJ => PropertyName {
            full: "ZWJ",
            abbr: "ZWJ",
        },
    }
}
