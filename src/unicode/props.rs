use crate::unicode::ucd::property_names;

#[derive(Clone, Copy)]
pub struct PropertyName {
    pub full: &'static str,
    pub abbr: &'static str,
}

/// The Unicode property enums should implement this trait.
///
/// # Examples
///
/// ```
/// use seshat::unicode::Ucd;
/// use seshat::unicode::props::Sc;
/// use seshat::unicode::props::UnicodeProperty;
///
/// fn main() {
///     let capital_a = 'A';
///     let sc_prop: Sc = capital_a.sc();
///     let sc_name = sc_prop.property_value_name();
///
///     println!("{} ({})", sc_name.full, sc_name.abbr);
///     // This will prints "Latin (Latn)".
/// }
/// ```
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

/// General_Category (gc)
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

/// Hangul_Syllable_Type (hst)
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

/// Grapheme_Cluster_Break (GCB)
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

/// Bidi_Class (bc)
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

/// Canonical_Combining_Class (ccc)
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

/// Decomposition_Type (dt)
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

/// Block (blk)
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u16)]
pub enum Blk {
    // These enum contents are generated by `gen-blk-enum.py` script.
    Adlam,
    AegeanNumbers,
    Ahom,
    Alchemical,
    AlphabeticPf,
    AnatolianHieroglyphs,
    AncientGreekMusic,
    AncientGreekNumbers,
    AncientSymbols,
    Arabic,
    ArabicExtA,
    ArabicExtB,
    ArabicExtC,
    ArabicMath,
    ArabicPfA,
    ArabicPfB,
    ArabicSup,
    Armenian,
    Arrows,
    Ascii,
    Avestan,
    Balinese,
    Bamum,
    BamumSup,
    BassaVah,
    Batak,
    Bengali,
    Bhaiksuki,
    BlockElements,
    Bopomofo,
    BopomofoExt,
    BoxDrawing,
    Brahmi,
    Braille,
    Buginese,
    Buhid,
    ByzantineMusic,
    Carian,
    CaucasianAlbanian,
    Chakma,
    Cham,
    Cherokee,
    CherokeeSup,
    ChessSymbols,
    Chorasmian,
    Cjk,
    CjkCompat,
    CjkCompatForms,
    CjkCompatIdeographs,
    CjkCompatIdeographsSup,
    CjkExtA,
    CjkExtB,
    CjkExtC,
    CjkExtD,
    CjkExtE,
    CjkExtF,
    CjkExtG,
    CjkExtH,
    CjkExtI,
    CjkRadicalsSup,
    CjkStrokes,
    CjkSymbols,
    CompatJamo,
    ControlPictures,
    Coptic,
    CopticEpactNumbers,
    CountingRod,
    Cuneiform,
    CuneiformNumbers,
    CurrencySymbols,
    CypriotSyllabary,
    CyproMinoan,
    Cyrillic,
    CyrillicExtA,
    CyrillicExtB,
    CyrillicExtC,
    CyrillicExtD,
    CyrillicSup,
    Deseret,
    Devanagari,
    DevanagariExt,
    DevanagariExtA,
    Diacriticals,
    DiacriticalsExt,
    DiacriticalsForSymbols,
    DiacriticalsSup,
    Dingbats,
    DivesAkuru,
    Dogra,
    Domino,
    Duployan,
    EarlyDynasticCuneiform,
    EgyptianHieroglyphFormatControls,
    EgyptianHieroglyphs,
    EgyptianHieroglyphsExtA,
    Elbasan,
    Elymaic,
    Emoticons,
    EnclosedAlphanum,
    EnclosedAlphanumSup,
    EnclosedCjk,
    EnclosedIdeographicSup,
    Ethiopic,
    EthiopicExt,
    EthiopicExtA,
    EthiopicExtB,
    EthiopicSup,
    Garay,
    GeometricShapes,
    GeometricShapesExt,
    Georgian,
    GeorgianExt,
    GeorgianSup,
    Glagolitic,
    GlagoliticSup,
    Gothic,
    Grantha,
    Greek,
    GreekExt,
    Gujarati,
    GunjalaGondi,
    Gurmukhi,
    GurungKhema,
    HalfAndFullForms,
    HalfMarks,
    Hangul,
    HanifiRohingya,
    Hanunoo,
    Hatran,
    Hebrew,
    HighPuSurrogates,
    HighSurrogates,
    Hiragana,
    Idc,
    IdeographicSymbols,
    ImperialAramaic,
    IndicNumberForms,
    IndicSiyaqNumbers,
    InscriptionalPahlavi,
    InscriptionalParthian,
    IpaExt,
    Jamo,
    JamoExtA,
    JamoExtB,
    Javanese,
    Kaithi,
    KaktovikNumerals,
    KanaExtA,
    KanaExtB,
    KanaSup,
    Kanbun,
    Kangxi,
    Kannada,
    Katakana,
    KatakanaExt,
    Kawi,
    KayahLi,
    Kharoshthi,
    KhitanSmallScript,
    Khmer,
    KhmerSymbols,
    Khojki,
    Khudawadi,
    KiratRai,
    Lao,
    Latin1Sup,
    LatinExtA,
    LatinExtAdditional,
    LatinExtB,
    LatinExtC,
    LatinExtD,
    LatinExtE,
    LatinExtF,
    LatinExtG,
    Lepcha,
    LetterlikeSymbols,
    Limbu,
    LinearA,
    LinearBIdeograms,
    LinearBSyllabary,
    Lisu,
    LisuSup,
    LowSurrogates,
    Lycian,
    Lydian,
    Mahajani,
    Mahjong,
    Makasar,
    Malayalam,
    Mandaic,
    Manichaean,
    Marchen,
    MasaramGondi,
    MathAlphanum,
    MathOperators,
    MayanNumerals,
    Medefaidrin,
    MeeteiMayek,
    MeeteiMayekExt,
    MendeKikakui,
    MeroiticCursive,
    MeroiticHieroglyphs,
    Miao,
    MiscArrows,
    MiscMathSymbolsA,
    MiscMathSymbolsB,
    MiscPictographs,
    MiscSymbols,
    MiscTechnical,
    Modi,
    ModifierLetters,
    ModifierToneLetters,
    Mongolian,
    MongolianSup,
    Mro,
    Multani,
    Music,
    Myanmar,
    MyanmarExtA,
    MyanmarExtB,
    MyanmarExtC,
    Nabataean,
    NagMundari,
    Nandinagari,
    Nb,
    NewTaiLue,
    Newa,
    Nko,
    NumberForms,
    Nushu,
    NyiakengPuachueHmong,
    Ocr,
    Ogham,
    OlChiki,
    OlOnal,
    OldHungarian,
    OldItalic,
    OldNorthArabian,
    OldPermic,
    OldPersian,
    OldSogdian,
    OldSouthArabian,
    OldTurkic,
    OldUyghur,
    Oriya,
    OrnamentalDingbats,
    Osage,
    Osmanya,
    OttomanSiyaqNumbers,
    PahawhHmong,
    Palmyrene,
    PauCinHau,
    PhagsPa,
    Phaistos,
    Phoenician,
    PhoneticExt,
    PhoneticExtSup,
    PlayingCards,
    PsalterPahlavi,
    Pua,
    Punctuation,
    Rejang,
    Rumi,
    Runic,
    Samaritan,
    Saurashtra,
    Sharada,
    Shavian,
    ShorthandFormatControls,
    Siddham,
    Sinhala,
    SinhalaArchaicNumbers,
    SmallForms,
    SmallKanaExt,
    Sogdian,
    SoraSompeng,
    Soyombo,
    Specials,
    Sundanese,
    SundaneseSup,
    Sunuwar,
    SupArrowsA,
    SupArrowsB,
    SupArrowsC,
    SupMathOperators,
    SupPuaA,
    SupPuaB,
    SupPunctuation,
    SupSymbolsAndPictographs,
    SuperAndSub,
    SuttonSignwriting,
    SylotiNagri,
    SymbolsAndPictographsExtA,
    SymbolsForLegacyComputing,
    SymbolsForLegacyComputingSup,
    Syriac,
    SyriacSup,
    Tagalog,
    Tagbanwa,
    Tags,
    TaiLe,
    TaiTham,
    TaiViet,
    TaiXuanJing,
    Takri,
    Tamil,
    TamilSup,
    Tangsa,
    Tangut,
    TangutComponents,
    TangutSup,
    Telugu,
    Thaana,
    Thai,
    Tibetan,
    Tifinagh,
    Tirhuta,
    Todhri,
    Toto,
    TransportAndMap,
    TuluTigalari,
    Ucas,
    UcasExt,
    UcasExtA,
    Ugaritic,
    Vai,
    VedicExt,
    VerticalForms,
    Vithkuqi,
    Vs,
    VsSup,
    Wancho,
    WarangCiti,
    Yezidi,
    YiRadicals,
    YiSyllables,
    Yijing,
    ZanabazarSquare,
    ZnamennyMusic,
}

impl UnicodeProperty for Blk {
    fn property_value_name(&self) -> PropertyName {
        property_names::blk_name(*self)
    }
}

/// Unicode property Script (sc).
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Sc {
    // These enum contents are generated by `gen-sc-enum.py` script.
    Adlm,
    Aghb,
    Ahom,
    Arab,
    Armi,
    Armn,
    Avst,
    Bali,
    Bamu,
    Bass,
    Batk,
    Beng,
    Bhks,
    Bopo,
    Brah,
    Brai,
    Bugi,
    Buhd,
    Cakm,
    Cans,
    Cari,
    Cham,
    Cher,
    Chrs,
    Copt,
    Cpmn,
    Cprt,
    Cyrl,
    Deva,
    Diak,
    Dogr,
    Dsrt,
    Dupl,
    Egyp,
    Elba,
    Elym,
    Ethi,
    Gara,
    Geor,
    Glag,
    Gong,
    Gonm,
    Goth,
    Gran,
    Grek,
    Gujr,
    Gukh,
    Guru,
    Hang,
    Hani,
    Hano,
    Hatr,
    Hebr,
    Hira,
    Hluw,
    Hmng,
    Hmnp,
    Hrkt,
    Hung,
    Ital,
    Java,
    Kali,
    Kana,
    Kawi,
    Khar,
    Khmr,
    Khoj,
    Kits,
    Knda,
    Krai,
    Kthi,
    Lana,
    Laoo,
    Latn,
    Lepc,
    Limb,
    Lina,
    Linb,
    Lisu,
    Lyci,
    Lydi,
    Mahj,
    Maka,
    Mand,
    Mani,
    Marc,
    Medf,
    Mend,
    Merc,
    Mero,
    Mlym,
    Modi,
    Mong,
    Mroo,
    Mtei,
    Mult,
    Mymr,
    Nagm,
    Nand,
    Narb,
    Nbat,
    Newa,
    Nkoo,
    Nshu,
    Ogam,
    Olck,
    Onao,
    Orkh,
    Orya,
    Osge,
    Osma,
    Ougr,
    Palm,
    Pauc,
    Perm,
    Phag,
    Phli,
    Phlp,
    Phnx,
    Plrd,
    Prti,
    Rjng,
    Rohg,
    Runr,
    Samr,
    Sarb,
    Saur,
    Sgnw,
    Shaw,
    Shrd,
    Sidd,
    Sind,
    Sinh,
    Sogd,
    Sogo,
    Sora,
    Soyo,
    Sund,
    Sunu,
    Sylo,
    Syrc,
    Tagb,
    Takr,
    Tale,
    Talu,
    Taml,
    Tang,
    Tavt,
    Telu,
    Tfng,
    Tglg,
    Thaa,
    Thai,
    Tibt,
    Tirh,
    Tnsa,
    Todr,
    Toto,
    Tutg,
    Ugar,
    Vaii,
    Vith,
    Wara,
    Wcho,
    Xpeo,
    Xsux,
    Yezi,
    Yiii,
    Zanb,
    Zinh,
    Zyyy,
    Zzzz,
}

impl UnicodeProperty for Sc {
    fn property_value_name(&self) -> PropertyName {
        property_names::sc_name(*self)
    }
}

/// Unicode property Age(age).
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Age {
    V1_1,
    V2_0,
    V2_1,
    V3_0,
    V3_1,
    V3_2,
    V4_0,
    V4_1,
    V5_0,
    V5_1,
    V5_2,
    V6_0,
    V6_1,
    V6_2,
    V6_3,
    V7_0,
    V8_0,
    V9_0,
    V10_0,
    V11_0,
    V12_0,
    V12_1,
    V13_0,
    V14_0,
    V15_0,
    V15_1,
    V16_0,
    NA,
}

impl UnicodeProperty for Age {
    fn property_value_name(&self) -> PropertyName {
        property_names::age_name(*self)
    }
}

/// Unicode property Word_Break (WB).
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Wb {
    CR,     // CR
    DQ,     // Double_Quote
    EB,     // E_Base
    EBG,    // E_Base_GAZ
    EM,     // E_Modifier
    EX,     // ExtendNumLet
    Extend, // Extend
    FO,     // Format
    GAZ,    // Glue_After_Zwj
    HL,     // Hebrew_Letter
    KA,     // Katakana
    LE,     // ALetter
    LF,     // LF
    MB,     // MidNumLet
    ML,     // MidLetter
    MN,     // MidNum
    NL,     // Newline
    NU,     // Numeric
    RI,     // Regional_Indicator
    SQ,     // Single_Quote
    WSegSpace,  // WSegSpace
    XX,     // Other
    ZWJ,    // ZWJ
}

impl UnicodeProperty for Wb {
    fn property_value_name(&self) -> PropertyName {
        property_names::wb_name(*self)
    }
}

/// Unicode property Indic_Syllabic_Category (InSC).
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Insc {
    Avagraha,                        // Avagraha
    Bindu,                           // Bindu
    Brahmi_Joining_Number,           // Brahmi_Joining_Number
    Cantillation_Mark,               // Cantillation_Mark
    Consonant,                       // Consonant
    Consonant_Dead,                  // Consonant_Dead
    Consonant_Final,                 // Consonant_Final
    Consonant_Head_Letter,           // Consonant_Head_Letter
    Consonant_Initial_Postfixed,     // Consonant_Initial_Postfixed
    Consonant_Killer,                // Consonant_Killer
    Consonant_Medial,                // Consonant_Medial
    Consonant_Placeholder,           // Consonant_Placeholder
    Consonant_Preceding_Repha,       // Consonant_Preceding_Repha
    Consonant_Prefixed,              // Consonant_Prefixed
    Consonant_Subjoined,             // Consonant_Subjoined
    Consonant_Succeeding_Repha,      // Consonant_Succeeding_Repha
    Consonant_With_Stacker,          // Consonant_With_Stacker
    Gemination_Mark,                 // Gemination_Mark
    Invisible_Stacker,               // Invisible_Stacker
    Joiner,                          // Joiner
    Modifying_Letter,                // Modifying_Letter
    Non_Joiner,                      // Non_Joiner
    Nukta,                           // Nukta
    Number,                          // Number
    Number_Joiner,                   // Number_Joiner
    Other,                           // Other
    Pure_Killer,                     // Pure_Killer
    Register_Shifter,                // Register_Shifter
    Reordering_Killer,               // Reordering_Killer
    Syllable_Modifier,               // Syllable_Modifier
    Tone_Letter,                     // Tone_Letter
    Tone_Mark,                       // Tone_Mark
    Virama,                          // Virama
    Visarga,                         // Visarga
    Vowel,                           // Vowel
    Vowel_Dependent,                 // Vowel_Dependent
    Vowel_Independent,               // Vowel_Independent
}

impl UnicodeProperty for Insc {
    fn property_value_name(&self) -> PropertyName {
        property_names::insc_name(*self)
    }
}

/// Unicode property Indic_Conjunct_Break (InCB).
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Incb {
    Consonant,  // Consonant
    Extend,     // Extend
    Linker,     // Linker
    None,       // None
}

impl UnicodeProperty for Incb {
    fn property_value_name(&self) -> PropertyName {
        property_names::incb_name(*self)
    }
}
