pub(self) mod grapheme_break_test_cases;

use grapheme_break_test_cases as test_cases;

use seshat::unicode::Segmentation;

#[test]
fn validate_grapheme_break() {
    for case in test_cases::TEST_CASES.iter() {
        let source = case.0;
        let breaks = case.1;

        for (i, grapheme) in source.break_graphemes().enumerate() {
            assert_eq!(grapheme, breaks[i]);
        }
    }
}