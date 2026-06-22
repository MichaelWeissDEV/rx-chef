// Tests for the caret_mdecode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations caret_mdecode::

use rxchef::operations::caret_mdecode::CaretMdecode;
use rxchef::Operation;

#[test]
fn test_caret_decode_basic() {
    let op = CaretMdecode;
    let input = b"^M".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(result, vec![13]); // \r
}
#[test]
fn test_m_caret_decode() {
    let op = CaretMdecode;
    let input = b"M-^]".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(result, vec![0x9d]);
}
#[test]
fn test_m_decode() {
    let op = CaretMdecode;
    let input = b"M-a".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(result, vec![0xe1]); // 'a' is 97, 97 + 128 = 225 = 0xe1
}
#[test]
fn test_complex_decode() {
    let op = CaretMdecode;
    let input = b"hello^MworldM-!^?".to_vec();
    let result = op.run(input, &[]).unwrap();
    // hello (104, 101, 108, 108, 111)
    // ^M (13)
    // world (119, 111, 114, 108, 100)
    // M-! (33 + 128 = 161)
    // ^? (127)
    assert_eq!(
        result,
        vec![104, 101, 108, 108, 111, 13, 119, 111, 114, 108, 100, 161, 127]
    );
}
