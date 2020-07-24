pub(self) mod normalization_test_cases;

use normalization_test_cases as test_cases;

use seshat::unicode::Normalization;

/* From https://www.unicode.org/Public/13.0.0/ucd/NormalizationTest.txt
# CONFORMANCE:
# 1. The following invariants must be true for all conformant implementations
#
#    NFC
#      c2 ==  toNFC(c1) ==  toNFC(c2) ==  toNFC(c3)
#      c4 ==  toNFC(c4) ==  toNFC(c5)
#
#    NFD
#      c3 ==  toNFD(c1) ==  toNFD(c2) ==  toNFD(c3)
#      c5 ==  toNFD(c4) ==  toNFD(c5)
#
#    NFKC
#      c4 == toNFKC(c1) == toNFKC(c2) == toNFKC(c3) == toNFKC(c4) == toNFKC(c5)
#
#    NFKD
#      c5 == toNFKD(c1) == toNFKD(c2) == toNFKD(c3) == toNFKD(c4) == toNFKD(c5)
#
# 2. For every code point X assigned in this version of Unicode that is not specifically
#    listed in Part 1, the following invariants must be true for all conformant
#    implementations:
#
#      X == toNFC(X) == toNFD(X) == toNFKC(X) == toNFKD(X)
#
*/
#[test]
fn validate_normalizations() {
    for case in test_cases::TEST_CASES.iter() {
        let c1 = case.0;
        let c2 = case.1;
        let c3 = case.2;
        let c4 = case.3;
        let c5 = case.4;

        // NFC

        // NFD
        assert_eq!(c3, c1.to_nfd(), "c3 != c1.to_nfd() - source: {}", c1);
        assert_eq!(c3, c2.to_nfd(), "c3 != c2.to_nfd() - source: {}", c1);
        assert_eq!(c3, c3.to_nfd(), "c3 != c3.to_nfd() - source: {}", c1);
        assert_eq!(c5, c4.to_nfd(), "c5 != c4.to_nfd() - source: {}", c1);
        assert_eq!(c5, c5.to_nfd(), "c5 != c5.to_nfd() - source: {}", c1);

        // NFKC

        // NFKD
        assert_eq!(c5, c1.to_nfkd(), "c5 != c1.to_nfkd() - source: {}", c1);
        assert_eq!(c5, c2.to_nfkd(), "c5 != c2.to_nfkd() - source: {}", c1);
        assert_eq!(c5, c3.to_nfkd(), "c5 != c3.to_nfkd() - source: {}", c1);
        assert_eq!(c5, c4.to_nfkd(), "c5 != c4.to_nfkd() - source: {}", c1);
        assert_eq!(c5, c5.to_nfkd(), "c5 != c5.to_nfkd() - source: {}", c1);
    }
}