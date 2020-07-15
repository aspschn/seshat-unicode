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

fn check_properties(cp: CodePoint, el: &Vec<OwnedAttribute>) -> bool {
    // na (Name)
    let cp_na = cp.na();
    check_na_property(cp, &cp_na, el);
    // gc (General_Category)
    let cp_gc = cp.gc().property_value_name().abbr;
    check_property(cp, cp_gc, "gc", el);
    // hst (Hangul_Syllable_Type)
    let cp_hst = cp.hst().property_value_name().abbr;
    check_property(cp, cp_hst, "hst", el);
    // WSpace (White_Space)
    let cp_wspace = BinaryProperty::from(cp.wspace()).property_value_name().abbr;
    check_property(cp, cp_wspace, "WSpace", el);
    // Bidi_C (Bidi_Control)
    let cp_bidi_c = BinaryProperty::from(cp.bidi_c()).property_value_name().abbr;
    check_property(cp, cp_bidi_c, "Bidi_C", el);
    // Join_C (Join_Control)
    let cp_join_c = BinaryProperty::from(cp.join_c()).property_value_name().abbr;
    check_property(cp, cp_join_c, "Join_C", el);
    // Dash (Dash)
    let cp_dash = BinaryProperty::from(cp.dash()).property_value_name().abbr;
    check_property(cp, cp_dash, "Dash", el);
    // Hyphen (Hyphen)
    let cp_hyphen = BinaryProperty::from(cp.hyphen()).property_value_name().abbr;
    check_property(cp, cp_hyphen, "Hyphen", el);
    // QMark (Quotation_Mark)
    let cp_qmark = BinaryProperty::from(cp.qmark()).property_value_name().abbr;
    check_property(cp, cp_qmark, "QMark", el);
    // Term (Terminal_Punctuation)
    let cp_term = BinaryProperty::from(cp.term()).property_value_name().abbr;
    check_property(cp, cp_term, "Term", el);
    // OMath (Other_Math)
    let cp_omath = BinaryProperty::from(cp.omath()).property_value_name().abbr;
    check_property(cp, cp_omath, "OMath", el);
    // Hex (Hex_Digit)
    let cp_hex = BinaryProperty::from(cp.hex()).property_value_name().abbr;
    check_property(cp, cp_hex, "Hex", el);
    // AHex (ASCII_Hex_Digit)
    let cp_ahex = BinaryProperty::from(cp.ahex()).property_value_name().abbr;
    check_property(cp, cp_ahex, "AHex", el);
    // OAlpha (Other_Alphabetic)
    let cp_oalpha = BinaryProperty::from(cp.oalpha()).property_value_name().abbr;
    check_property(cp, cp_oalpha, "OAlpha", el);
    // Ideo (Ideographic)
    let cp_ideo = BinaryProperty::from(cp.ideo()).property_value_name().abbr;
    check_property(cp, cp_ideo, "Ideo", el);
    // Dia (Diacritic)
    let cp_dia = BinaryProperty::from(cp.dia()).property_value_name().abbr;
    check_property(cp, cp_dia, "Dia", el);
    // Ext (Extender)
    let cp_ext = BinaryProperty::from(cp.ext()).property_value_name().abbr;
    check_property(cp, cp_ext, "Ext", el);
    // OLower (Other_Lowercase)
    let cp_olower = BinaryProperty::from(cp.olower()).property_value_name().abbr;
    check_property(cp, cp_olower, "OLower", el);
    // OUpper (Other_Uppercase)
    let cp_oupper = BinaryProperty::from(cp.oupper()).property_value_name().abbr;
    check_property(cp, cp_oupper, "OUpper", el);
    // NChar (Noncharacter_Code_Point)
    let cp_nchar = BinaryProperty::from(cp.nchar()).property_value_name().abbr;
    check_property(cp, cp_nchar, "NChar", el);
    // OGr_Ext (Other_Grapheme_Extend)
    let cp_ogr_ext = BinaryProperty::from(cp.ogr_ext()).property_value_name().abbr;
    check_property(cp, cp_ogr_ext, "OGr_Ext", el);
    // IDSB (IDS_Binary_Operator)
    let cp_idsb = BinaryProperty::from(cp.idsb()).property_value_name().abbr;
    check_property(cp, cp_idsb, "IDSB", el);
    // IDST (IDS_Trinary_Operator)
    let cp_idst = BinaryProperty::from(cp.idst()).property_value_name().abbr;
    check_property(cp, cp_idst, "IDST", el);
    // Radical (Radical)
    let cp_radical = BinaryProperty::from(cp.radical()).property_value_name().abbr;
    check_property(cp, cp_radical, "Radical", el);
    // UIdeo (Unified_Ideograph)
    let cp_uideo = BinaryProperty::from(cp.uideo()).property_value_name().abbr;
    check_property(cp, cp_uideo, "UIdeo", el);
    // ODI (Other_Default_Ignorable_Code_Point)
    let cp_odi = BinaryProperty::from(cp.odi()).property_value_name().abbr;
    check_property(cp, cp_odi, "ODI", el);
    // Dep (Deprecated)
    let cp_dep = BinaryProperty::from(cp.dep()).property_value_name().abbr;
    check_property(cp, cp_dep, "Dep", el);
    // SD (Soft_Dotted)
    let cp_sd = BinaryProperty::from(cp.sd()).property_value_name().abbr;
    check_property(cp, cp_sd, "SD", el);
    // LOE (Logical_Order_Exception)
    let cp_loe = BinaryProperty::from(cp.loe()).property_value_name().abbr;
    check_property(cp, cp_loe, "LOE", el);
    // OIDS (Other_ID_Start)
    let cp_oids = BinaryProperty::from(cp.oids()).property_value_name().abbr;
    check_property(cp, cp_oids, "OIDS", el);
    // OIDC (Other_ID_Continue)
    let cp_oidc = BinaryProperty::from(cp.oidc()).property_value_name().abbr;
    check_property(cp, cp_oidc, "OIDC", el);
    // STerm (Sentence_Terminal)
    let cp_sterm = BinaryProperty::from(cp.sterm()).property_value_name().abbr;
    check_property(cp, cp_sterm, "STerm", el);
    // VS (Variation_Selector)
    let cp_vs = BinaryProperty::from(cp.vs()).property_value_name().abbr;
    check_property(cp, cp_vs, "VS", el);
    // Pat_WS (Pattern_White_Space)
    let cp_pat_ws = BinaryProperty::from(cp.pat_ws()).property_value_name().abbr;
    check_property(cp, cp_pat_ws, "Pat_WS", el);
    // Pat_Syn (Pattern_Syntax)
    let cp_pat_syn = BinaryProperty::from(cp.pat_syn()).property_value_name().abbr;
    check_property(cp, cp_pat_syn, "Pat_Syn", el);
    // PCM (Prepended_Concatenation_Mark)
    let cp_pcm = BinaryProperty::from(cp.pcm()).property_value_name().abbr;
    check_property(cp, cp_pcm, "PCM", el);
    // RI (Regional_Indicator)
    let cp_ri = BinaryProperty::from(cp.ri()).property_value_name().abbr;
    check_property(cp, cp_ri, "RI", el);

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
