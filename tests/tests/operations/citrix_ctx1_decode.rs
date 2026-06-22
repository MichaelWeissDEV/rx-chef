// Tests for the citrix_ctx1_decode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations citrix_ctx1_decode::

use rxchef::operations::citrix_ctx1_decode::CitrixCtx1Decode;
use rxchef::operations::citrix_ctx1_encode::CitrixCtx1Encode;
use rxchef::Operation;

#[test]
fn test_ctx1_decode_invalid_length() {
    let op = CitrixCtx1Decode;
    // length not divisible by 4
    let result = op.run(b"ABC".to_vec(), &[]);
    assert!(result.is_err());
}
#[test]
fn test_ctx1_decode_empty() {
    let op = CitrixCtx1Decode;
    let result = op.run(vec![], &[]).unwrap();
    assert!(result.is_empty());
}
#[test]
fn test_ctx1_roundtrip() {
    // Known CTX1 encoded value: "Password" encodes to "PABLCAAAAAAAAAAAAAAA" (approx)
    // We verify encode->decode gives original (using our encoder)
    let encode_op = CitrixCtx1Encode;
    let decode_op = CitrixCtx1Decode;
    let original = "Hi";
    let encoded = encode_op.run(original.as_bytes().to_vec(), &[]).unwrap();
    let decoded = decode_op.run(encoded, &[]).unwrap();
    let decoded_str = String::from_utf8(decoded).unwrap();
    assert_eq!(decoded_str, original);
}
