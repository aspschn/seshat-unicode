pub(self) mod grapheme_break_test_cases;

use grapheme_break_test_cases as test_cases;

use seshat::unicode::Segmentation;

#[test]
fn validate_grapheme_break() {
    let mut case_number = 1;
    for case in test_cases::TEST_CASES.iter() {
        let source = case.0;
        let breaks = case.1;

        for (i, grapheme) in source.break_graphemes().enumerate() {
            assert_eq!(grapheme, breaks[i], "Error in the test case {}", case_number);
        }
        case_number += 1;
    }

    // Additional test cases.
    let s1 = "\u{1F469}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F469}-\u{1F468}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F468}";
    let mut iter = s1.break_graphemes();
    assert_eq!(iter.next(), Some("\u{1F469}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F469}"));
    assert_eq!(iter.next(), Some("-"));
    assert_eq!(iter.next(), Some("\u{1F468}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F468}"));
    assert_eq!(iter.next(), None);

    let s2 = " \u{1F1F0}\u{1F1F7}\u{1F1F7}";
    let mut iter = s2.break_graphemes();
    assert_eq!(iter.next(), Some(" "));
    assert_eq!(iter.next(), Some("\u{1F1F0}\u{1F1F7}"));
    assert_eq!(iter.next(), Some("\u{1F1F7}"));
    assert_eq!(iter.next(), None);
}