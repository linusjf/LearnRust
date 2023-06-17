use testing::doctests::shorten_string;

#[test]
fn test_shorten_string() {
    assert_eq!(shorten_string("Hello World", 5), "Hello");
}
