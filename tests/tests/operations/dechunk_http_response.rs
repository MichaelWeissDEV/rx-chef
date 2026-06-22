// Tests for the dechunk_http_response operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations dechunk_http_response::

use rxchef::operations::dechunk_http_response::DechunkHttpResponse;
use rxchef::Operation;

#[test]
fn test_dechunk_http_response_basic() {
    let op = DechunkHttpResponse;
    // Chunked encoding: 5\r\nHello\r\n0\r\n\r\n
    let input = b"5\r\nHello\r\n0\r\n\r\n".to_vec();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "Hello");
}
#[test]
fn test_dechunk_http_response_multiple_chunks() {
    let op = DechunkHttpResponse;
    // 5\r\nHello\r\n5\r\nWorld\r\n0\r\n\r\n
    let input = b"5\r\nHello\r\n5\r\nWorld\r\n0\r\n\r\n".to_vec();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "HelloWorld");
}
#[test]
fn test_dechunk_http_response_empty() {
    let op = DechunkHttpResponse;
    // 0\r\n\r\n
    let input = b"0\r\n\r\n".to_vec();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "");
}
#[test]
fn test_dechunk_http_response_with_crlf() {
    let op = DechunkHttpResponse;
    // 5\r\nHello\r\n0\r\n\r\n with CRLF
    let input = b"5\r\nHello\r\n0\r\n\r\n".to_vec();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "Hello");
}
#[test]
fn test_dechunk_http_response_invalid_chunk_size() {
    let op = DechunkHttpResponse;
    // Invalid chunk size
    let input = b"abc\r\nHello\r\n0\r\n\r\n".to_vec();
    let result = op.run(input, &[]);
    assert!(result.is_err());
}
#[test]
fn test_dechunk_http_response_truncated_chunk() {
    let op = DechunkHttpResponse;
    // Chunk says 10 bytes but only 5 provided
    let input = b"10\r\nHello\r\n0\r\n\r\n".to_vec();
    let result = op.run(input, &[]);
    assert!(result.is_err());
}
