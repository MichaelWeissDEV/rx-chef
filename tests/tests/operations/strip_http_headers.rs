// Tests for the strip_http_headers operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations strip_http_headers::

use rxchef::operations::strip_http_headers::StripHTTPHeaders;
use rxchef::Operation;

#[test]
fn test_strip_http_headers_crlf() {
    let op = StripHTTPHeaders;
    let input = b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, World!".to_vec();
    let result = op.run(input, &[]).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "Hello, World!");
}
#[test]
fn test_strip_http_headers_lf() {
    let op = StripHTTPHeaders;
    let input = b"HTTP/1.1 200 OK\nContent-Length: 5\n\nHello".to_vec();
    let result = op.run(input, &[]).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "Hello");
}
#[test]
fn test_strip_http_headers_no_headers() {
    let op = StripHTTPHeaders;
    let input = b"no headers here".to_vec();
    let result = op.run(input, &[]).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "no headers here");
}
