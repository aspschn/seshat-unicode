use std::fs::File;
use std::io::BufReader;

use seshat::unicode::CodePoint;
use seshat::unicode::Ucd;
use seshat::unicode::props::*;

use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;

fn check_property(cp: CodePoint, val: &str, prop: &str, attrs: &Vec<OwnedAttribute>) {
    match attrs.iter().find(|&attr| attr.name.local_name == prop) {
        Some(should) => {
            if val != should.value {
                panic!("{}: Not equal. Should `{}` but found `{}`.", cp, should, val);
            }
        }
        None => panic!("Attribute {} is not in XML file!", prop),
    }
}

fn check_age_property(cp: CodePoint, val: &str, prop: &str, attrs: &Vec<OwnedAttribute>) {
    let mut cmp_val = val;
    if cmp_val == "NA" {
        cmp_val = "unassigned";
    }
    match attrs.iter().find(|&attr| attr.name.local_name == prop) {
        Some(should) => {
            if cmp_val != should.value {
                panic!("{}: Not equal. Should `{}` but found `{}`.", cp, should, val);
            }
        }
        None => panic!("Attribute {} is not in XML file!", prop),
    }
}

// dt in the XML file exceptionally using lower case rather than title case.
fn check_dt_property(cp: CodePoint, val: &str, attrs: &Vec<OwnedAttribute>) {
    match attrs.iter().find(|&attr| attr.name.local_name == "dt") {
        Some(should) => {
            let val = val.to_lowercase();
            if val != should.value {
                panic!("{}: Not equal. Should `{}` but found `{}`.", cp, should, val);
            }
        }
        None => panic!("Attribute dt is not in XML file!"),
    }
}

fn check_na_property(cp: CodePoint, val: &str, attrs: &Vec<OwnedAttribute>) {
    match attrs.iter().find(|&attr| attr.name.local_name == "na") {
        Some(should) => {
            let mut should = should.value.clone();
            if should.contains("#") {
                should = should[..should.len() - 1].to_string();
                should.push_str(&format!("{:04X}", cp.to_u32()));
            }
            if val != should {
                panic!("{}: Not equal. Should \"{}\" but found \"{}\".", cp, should, val);
            }
        }
        None => panic!("Attribute `na` is not in XML file!"),
    }
}

fn check_dm_property(cp: CodePoint, val: &str, attrs: &Vec<OwnedAttribute>) {
    match attrs.iter().find(|&attr| attr.name.local_name == "dm") {
        Some(should) => {
            let mut cmp = val.chars()
                .map(|x| format!("{:04X}", x as u32))
                .collect::<Vec<String>>()
                .join(" ");
            if cmp == "" {
                cmp = String::from("#");
            }
            if cmp != should.value {
                panic!("{}: Not equal. Should \"{}\" but found \"{}\".", cp, should, cmp);
            }
        }
        None => panic!("Attribute `dm` is not in XML file!"),
    }
}

fn check_properties(cp: CodePoint, el: &Vec<OwnedAttribute>) -> bool {
    //================================================
    // Numeric Properties
    //================================================
    // cjkAccountingNumeric     ; kAccountingNumeric
    // cjkOtherNumeric          ; kOtherNumeric
    // cjkPrimaryNumeric        ; kPrimaryNumeric
    // nv                       ; Numeric_Value
 
    //================================================
    // String Properties
    //================================================
    // cf                       ; Case_Folding
    // cjkCompatibilityVariant  ; kCompatibilityVariant
    // dm (Decomposition_Mapping)
    let cp_dm = cp.dm();
    check_dm_property(cp, &cp_dm, el);
    // FC_NFKC                  ; FC_NFKC_Closure
    // lc                       ; Lowercase_Mapping
    // NFKC_CF                  ; NFKC_Casefold
    // scf                      ; Simple_Case_Folding         ; sfc
    // slc                      ; Simple_Lowercase_Mapping
    // stc                      ; Simple_Titlecase_Mapping
    // suc                      ; Simple_Uppercase_Mapping
    // tc                       ; Titlecase_Mapping
    // uc                       ; Uppercase_Mapping

    //================================================
    // Miscellaneous Properties
    //================================================
    // na (Name)
    let cp_na = cp.na();
    check_na_property(cp, &cp_na, el);

    //================================================
    // Catalog Properties
    //================================================
    // age                      ; Age
    let cp_age = cp.age().property_value_name().abbr;
    check_age_property(cp, cp_age, "age", el);
    // blk (Block)              ; Block
    let cp_blk = cp.blk().property_value_name().abbr;
    check_property(cp, cp_blk, "blk", el);
    // sc (Script)              ; Script
    let cp_sc = cp.sc().property_value_name().abbr;
    check_property(cp, cp_sc, "sc", el);

    //================================================
    // Enumerated properties
    //================================================
    // bc (Bidi_Class)
    let cp_bc = cp.bc().property_value_name().abbr;
    check_property(cp, cp_bc, "bc", el);
    // bpt                      ; Bidi_Paired_Bracket_Type
    // ccc (Canonical_Combining_Class)
    let cp_ccc = cp.ccc() as u8;
    check_property(cp, &cp_ccc.to_string(), "ccc", el);
    // dt (Decomposition_Type)
    let cp_dt = cp.dt().property_value_name().abbr;
    check_dt_property(cp, cp_dt, el);
    // ea                       ; East_Asian_Width
    // gc (General_Category)
    let cp_gc = cp.gc().property_value_name().abbr;
    check_property(cp, cp_gc, "gc", el);
    // GCB (Grapheme_Cluster_Break)
    let cp_gcb = cp.gcb().property_value_name().abbr;
    check_property(cp, cp_gcb, "GCB", el);
    // hst (Hangul_Syllable_Type)
    let cp_hst = cp.hst().property_value_name().abbr;
    check_property(cp, cp_hst, "hst", el);
    // InPC                     ; Indic_Positional_Category
    // InSC                     ; Indic_Syllabic_Category
    // jg                       ; Joining_Group
    // jt                       ; Joining_Type
    // lb                       ; Line_Break
    // NFC_QC                   ; NFC_Quick_Check
    // NFD_QC                   ; NFD_Quick_Check
    // NFKC_QC                  ; NFKC_Quick_Check
    // NFKD_QC                  ; NFKD_Quick_Check
    // nt                       ; Numeric_Type
    // SB                       ; Sentence_Break
    // vo                       ; Vertical_Orientation
    // WB                       ; Word_Break

    //================================================
    // Binary Properties
    //================================================
    // AHex (ASCII_Hex_Digit)
    let cp_ahex = BinaryProperty::from(cp.ahex()).property_value_name().abbr;
    check_property(cp, cp_ahex, "AHex", el);
    // Alpha (Alphabetic)
    let cp_alpha = BinaryProperty::from(cp.alpha()).property_value_name().abbr;
    check_property(cp, cp_alpha, "Alpha", el);
    // Bidi_C (Bidi_Control)
    let cp_bidi_c = BinaryProperty::from(cp.bidi_c()).property_value_name().abbr;
    check_property(cp, cp_bidi_c, "Bidi_C", el);
    // Bidi_M                   ; Bidi_Mirrored
    // Cased (Cased)
    let cp_cased = BinaryProperty::from(cp.cased()).property_value_name().abbr;
    check_property(cp, cp_cased, "Cased", el);
    // CE (Composition_Exclusion)
    let cp_ce = BinaryProperty::from(cp.ce()).property_value_name().abbr;
    check_property(cp, cp_ce, "CE", el);
    // CI                       ; Case_Ignorable
    // Comp_Ex (Full_Composition_Exclusion)
    let cp_comp_ex = BinaryProperty::from(cp.comp_ex()).property_value_name().abbr;
    check_property(cp, cp_comp_ex, "Comp_Ex", el);
    // CWCF                     ; Changes_When_Casefolded
    // CWCM                     ; Changes_When_Casemapped
    // CWKCF                    ; Changes_When_NFKC_Casefolded
    // CWL                      ; Changes_When_Lowercased
    // CWT                      ; Changes_When_Titlecased
    // CWU                      ; Changes_When_Uppercased
    // Dash (Dash)
    let cp_dash = BinaryProperty::from(cp.dash()).property_value_name().abbr;
    check_property(cp, cp_dash, "Dash", el);
    // Dep (Deprecated)
    let cp_dep = BinaryProperty::from(cp.dep()).property_value_name().abbr;
    check_property(cp, cp_dep, "Dep", el);
    // Default_Ignorable_Code_Point (DI)
    let cp_di = BinaryProperty::from(cp.di()).property_value_name().abbr;
    // Dia (Diacritic)
    let cp_dia = BinaryProperty::from(cp.dia()).property_value_name().abbr;
    check_property(cp, cp_dia, "Dia", el);
    // EBase (Emoji_Modifier_Base)
    let cp_ebase = BinaryProperty::from(cp.ebase()).property_value_name().abbr;
    check_property(cp, cp_ebase, "EBase", el);
    // EComp (Emoji_Component)
    let cp_ecomp = BinaryProperty::from(cp.ecomp()).property_value_name().abbr;
    check_property(cp, cp_ecomp, "EComp", el);
    // EMod (Emoji_Modifier)
    let cp_emod = BinaryProperty::from(cp.emod()).property_value_name().abbr;
    check_property(cp, cp_emod, "EMod", el);
    // Emoji (Emoji)
    let cp_emoji = BinaryProperty::from(cp.emoji()).property_value_name().abbr;
    check_property(cp, cp_emoji, "Emoji", el);
    // EPres (Emoji_Presentation)
    let cp_epres = BinaryProperty::from(cp.epres()).property_value_name().abbr;
    check_property(cp, cp_epres, "EPres", el);
    // Ext (Extender)
    let cp_ext = BinaryProperty::from(cp.ext()).property_value_name().abbr;
    check_property(cp, cp_ext, "Ext", el);
    // ExtPict (Extended_Pictographic)
    let cp_ext_pict = BinaryProperty::from(cp.ext_pict()).property_value_name().abbr;
    check_property(cp, cp_ext_pict, "ExtPict", el);
    // Gr_Base                  ; Grapheme_Base
    // Gr_Ext                   ; Grapheme_Extend
    // Gr_Link                  ; Grapheme_Link
    // Hex (Hex_Digit)
    let cp_hex = BinaryProperty::from(cp.hex()).property_value_name().abbr;
    check_property(cp, cp_hex, "Hex", el);
    // Hyphen (Hyphen)
    let cp_hyphen = BinaryProperty::from(cp.hyphen()).property_value_name().abbr;
    check_property(cp, cp_hyphen, "Hyphen", el);
    // IDC                      ; ID_Continue
    // Ideo (Ideographic)
    let cp_ideo = BinaryProperty::from(cp.ideo()).property_value_name().abbr;
    check_property(cp, cp_ideo, "Ideo", el);
    // IDS                      ; ID_Start
    // IDSB (IDS_Binary_Operator)
    let cp_idsb = BinaryProperty::from(cp.idsb()).property_value_name().abbr;
    check_property(cp, cp_idsb, "IDSB", el);
    // IDST (IDS_Trinary_Operator)
    let cp_idst = BinaryProperty::from(cp.idst()).property_value_name().abbr;
    check_property(cp, cp_idst, "IDST", el);
    // Join_C (Join_Control)
    let cp_join_c = BinaryProperty::from(cp.join_c()).property_value_name().abbr;
    check_property(cp, cp_join_c, "Join_C", el);
    // LOE (Logical_Order_Exception)
    let cp_loe = BinaryProperty::from(cp.loe()).property_value_name().abbr;
    check_property(cp, cp_loe, "LOE", el);
    // Lower (Lowercase)
    let cp_lower = BinaryProperty::from(cp.lower()).property_value_name().abbr;
    check_property(cp, cp_lower, "Lower", el);
    // Math (Math)
    let cp_math = BinaryProperty::from(cp.math()).property_value_name().abbr;
    check_property(cp, cp_math, "Math", el);
    // NChar (Noncharacter_Code_Point)
    let cp_nchar = BinaryProperty::from(cp.nchar()).property_value_name().abbr;
    check_property(cp, cp_nchar, "NChar", el);
    // OAlpha (Other_Alphabetic)
    let cp_oalpha = BinaryProperty::from(cp.oalpha()).property_value_name().abbr;
    check_property(cp, cp_oalpha, "OAlpha", el);
    // ODI (Other_Default_Ignorable_Code_Point)
    let cp_odi = BinaryProperty::from(cp.odi()).property_value_name().abbr;
    check_property(cp, cp_odi, "ODI", el);
    // OGr_Ext (Other_Grapheme_Extend)
    let cp_ogr_ext = BinaryProperty::from(cp.ogr_ext()).property_value_name().abbr;
    check_property(cp, cp_ogr_ext, "OGr_Ext", el);
    // OIDC (Other_ID_Continue)
    let cp_oidc = BinaryProperty::from(cp.oidc()).property_value_name().abbr;
    check_property(cp, cp_oidc, "OIDC", el);
    // OIDS (Other_ID_Start)
    let cp_oids = BinaryProperty::from(cp.oids()).property_value_name().abbr;
    check_property(cp, cp_oids, "OIDS", el);
    // OLower (Other_Lowercase)
    let cp_olower = BinaryProperty::from(cp.olower()).property_value_name().abbr;
    check_property(cp, cp_olower, "OLower", el);
    // OMath (Other_Math)
    let cp_omath = BinaryProperty::from(cp.omath()).property_value_name().abbr;
    check_property(cp, cp_omath, "OMath", el);
    // OUpper (Other_Uppercase)
    let cp_oupper = BinaryProperty::from(cp.oupper()).property_value_name().abbr;
    check_property(cp, cp_oupper, "OUpper", el);
    // Pat_Syn (Pattern_Syntax)
    let cp_pat_syn = BinaryProperty::from(cp.pat_syn()).property_value_name().abbr;
    check_property(cp, cp_pat_syn, "Pat_Syn", el);
    // Pat_WS (Pattern_White_Space)
    let cp_pat_ws = BinaryProperty::from(cp.pat_ws()).property_value_name().abbr;
    check_property(cp, cp_pat_ws, "Pat_WS", el);
    // PCM (Prepended_Concatenation_Mark)
    let cp_pcm = BinaryProperty::from(cp.pcm()).property_value_name().abbr;
    check_property(cp, cp_pcm, "PCM", el);
     // QMark (Quotation_Mark)
    let cp_qmark = BinaryProperty::from(cp.qmark()).property_value_name().abbr;
    check_property(cp, cp_qmark, "QMark", el);
    // Radical (Radical)
    let cp_radical = BinaryProperty::from(cp.radical()).property_value_name().abbr;
    check_property(cp, cp_radical, "Radical", el);
    // RI (Regional_Indicator)
    let cp_ri = BinaryProperty::from(cp.ri()).property_value_name().abbr;
    check_property(cp, cp_ri, "RI", el);
    // SD (Soft_Dotted)
    let cp_sd = BinaryProperty::from(cp.sd()).property_value_name().abbr;
    check_property(cp, cp_sd, "SD", el);
    // STerm (Sentence_Terminal)
    let cp_sterm = BinaryProperty::from(cp.sterm()).property_value_name().abbr;
    check_property(cp, cp_sterm, "STerm", el);
    // Term (Terminal_Punctuation)
    let cp_term = BinaryProperty::from(cp.term()).property_value_name().abbr;
    check_property(cp, cp_term, "Term", el);
    // UIdeo (Unified_Ideograph)
    let cp_uideo = BinaryProperty::from(cp.uideo()).property_value_name().abbr;
    check_property(cp, cp_uideo, "UIdeo", el);
    // Upper (Uppercase)
    let cp_upper = BinaryProperty::from(cp.upper()).property_value_name().abbr;
    check_property(cp, cp_upper, "Upper", el);
    // VS (Variation_Selector)
    let cp_vs = BinaryProperty::from(cp.vs()).property_value_name().abbr;
    check_property(cp, cp_vs, "VS", el);
    // WSpace (White_Space)
    let cp_wspace = BinaryProperty::from(cp.wspace()).property_value_name().abbr;
    check_property(cp, cp_wspace, "WSpace", el);
    // XIDC                     ; XID_Continue
    // XIDS                     ; XID_Start
    // XO_NFC                   ; Expands_On_NFC
    // XO_NFD                   ; Expands_On_NFD
    // XO_NFKC                  ; Expands_On_NFKC
    // XO_NFKD                  ; Expands_On_NFKD

    true
}

#[test]
fn validate_properties() {
    let file = match File::open("tests/data/ucd.nounihan.flat.xml") {
        Ok(f) => f,
        Err(_) => panic!("File not exists!"),
    };
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                // println!("{:?}", attributes);
                if name.local_name != "char" && name.local_name != "reserved" {
                    continue;
                }
                let cp = attributes.iter().find(|&attr| attr.name.local_name == "cp");
                match cp {
                    Some(val) => {
                        let cp = CodePoint::new(u32::from_str_radix(&val.value, 16).unwrap())
                            .unwrap();
                        // println!("{}", cp);
                        check_properties(cp, &attributes);
                    }
                    None => {
                        let first_cp = attributes.iter()
                            .find(|&attr| attr.name.local_name == "first-cp")
                            .unwrap();
                        let first_cp = CodePoint::new(
                            u32::from_str_radix(&first_cp.value, 16).unwrap()
                        ).unwrap();
                        let last_cp = attributes.iter()
                            .find(|&attr| attr.name.local_name == "last-cp")
                            .unwrap();
                        let last_cp = CodePoint::new(
                            u32::from_str_radix(&last_cp.value, 16).unwrap()
                        ).unwrap();
                        for cp in first_cp.to_u32()..last_cp.to_u32() + 1 {
                            check_properties(CodePoint::new(cp).unwrap(), &attributes);
                        }
                    }
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == "repertoire" {
                    break;
                }
            }
            Err(e) => {
                panic!("Error: {}", e);
            }
            _ => {},
        }
    }
}
