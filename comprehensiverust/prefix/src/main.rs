#![allow(unused_variables, dead_code)]

pub fn matches(prefix: &str, request_path: &str) {
    // ANCHOR_END: prefix_matches
    let prefixes = prefix.split('/');
    let request_paths = request_path
        .split('/')
        .map(|p| Some(p))
        .chain(std::iter::once(None));
    for path in request_paths {
        println!("{:#?}", path);
    }
}
// ANCHOR: prefix_matches
pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    // ANCHOR_END: prefix_matches
    let prefixes = prefix.split('/');
    let request_paths = request_path
        .split('/')
        .map(|p| Some(p))
        .chain(std::iter::once(None));

    let zipped = prefixes.zip(request_paths);
    for (prefix, request_path) in zipped {
        match request_path {
            Some(request_path) => {
                if (prefix != "*") && (prefix != request_path) {
                    return false;
                }
            }
            None => return false,
        }
    }
    true
}

#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}

fn main() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));
    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
    matches("/v1/publishers/*/books", "/v1/publishers")
}
