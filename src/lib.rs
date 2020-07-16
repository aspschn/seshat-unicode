//! An integrated Unicode library

pub mod unicode;

pub mod collections;

#[cfg(test)]
mod tests {
    use super::unicode::CodePoint;
    // use super::unicode::ToCodePoint;
    use super::unicode::Ucd;
    use super::unicode::props::*;

    #[test]
    fn code_point_to_string() {
        let cp_string = CodePoint::new(0xAC00).unwrap().to_string();
        assert_eq!("U+AC00", cp_string);
    }

    #[test]
    fn code_point_gc() {
        let cp = CodePoint::new(0xAC00).unwrap();
        match cp.gc() {
            Gc::Lo => (),
            _ => panic!("Not Lo"),
        }
    }

    #[test]
    fn code_point_na() {
        let c = '🦀';
        if c.na() != "CRAB" {
            panic!("Not \"CRAB\"");
        }
    }
}
