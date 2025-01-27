//! An integrated Unicode library

pub mod unicode;

pub mod collections;

#[cfg(test)]
mod tests {
    use super::unicode::{CodePoint, UnicodeVersion};
    // use super::unicode::ToCodePoint;
    use super::unicode::Ucd;
    use super::unicode::Normalization;
    use super::unicode::props::*;
    use super::unicode::UNICODE_VERSION;

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

    #[test]
    fn str_nfd() {
        let s = "각";
        assert_eq!(s.to_nfd(), "\u{1100}\u{1161}\u{11A8}");
    }

    #[test]
    fn version_check() {
        let ver = UnicodeVersion {
            major: 16,
            minor: 0,
            update: 0,
        };
        assert_eq!(ver, UNICODE_VERSION);
    }

    //==============================
    // Tests for private functions
    //==============================
    #[test]
    fn test_starter() {
        let cp1 = 0x00A0;
        assert_eq!(crate::unicode::normalization::starter(cp1), true);
        let cp2 = 0x0344;
        assert_eq!(crate::unicode::normalization::starter(cp2), false);
    }

    #[test]
    fn test_singleton() {
        let cp = 0x00A0;
        assert_eq!(crate::unicode::normalization::singleton_decomposition(cp), false);
        let cp2 = 0x0300;
        assert_eq!(crate::unicode::normalization::singleton_decomposition(cp2), false);
        let cp3 = 0x2126;
        assert_eq!(crate::unicode::normalization::singleton_decomposition(cp3), true);
    }

    #[test]
    fn test_non_starter_decomposition() {
        let cp1 = 0x0344;
        assert_eq!(crate::unicode::normalization::non_starter_decomposition(cp1), true);
        let cp2 = 0x0F73;
        assert_eq!(crate::unicode::normalization::non_starter_decomposition(cp2), true);
    }

    #[test]
    fn test_rdm() {
        let s1 = "\u{003B}";
        let composed = crate::unicode::ucd::dm::rdm(s1);
        assert_eq!(composed, 0x037E);

        let s2 = "\u{1100}\u{1161}";
        let composed = crate::unicode::ucd::dm::rdm(s2);
        assert_eq!(composed, 0xAC00);
    }

    #[test]
    fn test_nfc() {
        let s1 = "\u{0065}\u{0301}";
        let composed = crate::unicode::normalization::nfc(s1);
        assert_eq!(composed, vec!['é']);

        let s2 = "\u{1100}\u{1161}";
        let composed = crate::unicode::normalization::nfc(s2);
        assert_eq!(composed, vec!['가']);
    }
}
