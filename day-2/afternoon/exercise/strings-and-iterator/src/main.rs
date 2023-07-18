// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

// unfinished
pub fn prefix_matches_mine(prefix: &str, request_path: &str) -> bool {
    // unimplemented!()
    let prefix = prefix.split('/');
    let request_path = request_path.split('/');

    // this zip need to be by ref but idk how
    for (prefix_segment, request_segment) in prefix.zip(request_path) {
        if prefix_segment != request_segment && request_segment != "*" {
            return false;
        }
    }

    // // idk
    // match (prefix.next(), request_path.next()) {
    //     (None, None) => todo!(),
    //     (None, Some(_)) => todo!(),
    //     (Some(_), None) => todo!(),
    //     (Some(_), Some(_)) => todo!(),
    // }
    return false;
}

pub fn prefix_matches_other(prefix: &str, request_path: &str) -> bool {
    let mut request_segments = request_path.split('/');

    for prefix_segment in prefix.split('/') {
        let Some(request_segment) = request_segments.next() else {
            return false;
        };
        if request_segment != prefix_segment && prefix_segment != "*" {
            return false;
        }
    }
    true
}

// Alternatively, Iterator::zip() lets us iterate simultaneously over prefix
// and request segments. The zip() iterator is finished as soon as one of
// the source iterators is finished, but we need to iterate over all request
// segments. A neat trick that makes zip() work is to use map() and chain()
// to produce an iterator that returns Some(str) for each pattern segments,
// and then returns None indefinitely.

pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    // prefix_matches_mine(prefix, request_path)
    prefix_matches_other(prefix, request_path)
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

fn main() {}
