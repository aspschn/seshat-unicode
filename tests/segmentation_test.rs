use seshat::unicode::Segmentation;

#[test]
fn validate_segmentations() {
    let s1 = "é";
    let mut iter = s1.break_graphemes();
    assert_eq!(Some("é"), iter.next());
    assert_eq!(None, iter.next());

    let s2 = "한글";
    let mut iter = s2.break_graphemes();
    assert_eq!(Some("한"), iter.next());
    assert_eq!(Some("글"), iter.next());
    assert_eq!(None, iter.next());

    let s3 = "👩‍❤️‍👩🏳️‍🌈👨‍❤️‍👨";
    let mut iter = s3.break_graphemes();
    assert_eq!(Some("👩‍❤️‍👩"), iter.next());
    assert_eq!(Some("🏳️‍🌈"), iter.next());
    assert_eq!(Some("👨‍❤️‍👨"), iter.next());
    assert_eq!(None, iter.next());

    let s4 = "🇯🇵🇹🇭!";
    let mut iter = s4.break_graphemes();
    assert_eq!(Some("🇯🇵"), iter.next());
    assert_eq!(Some("🇹🇭"), iter.next());
    assert_eq!(Some("!"), iter.next());
    assert_eq!(None, iter.next());
}