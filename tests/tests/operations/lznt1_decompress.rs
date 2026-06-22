// Tests for the lznt1_decompress operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations lznt1_decompress::

use rxchef::operations::lznt1_decompress::LZNT1Decompress;
use rxchef::Operation;

#[test]
#[ignore]
fn test_lznt1_decompress() {
    let op = LZNT1Decompress;
    // "This is a test. This is a test." compressed with LZNT1
    let input = vec![
        0x19, 0xb0, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x74, 0x65, 0x73,
        0x74, 0x2e, 0x20, 0x02, 0x00, 0x0b, 0xf0, 0x74, 0x65, 0x73, 0x74, 0x2e,
    ];
    let result = op.run(input, &[]).unwrap();
    assert_eq!(
        String::from_utf8_lossy(&result),
        "This is a test. This is a test."
    );
}
