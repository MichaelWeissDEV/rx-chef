// Tests for the to_message_pack operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_message_pack::
//
// Expected byte sequences follow the MessagePack specification
// (https://github.com/msgpack/msgpack/blob/master/spec.md) format families:
// positive fixint 0x00-0x7f, fixstr 0xa0-0xbf, fixarray 0x90-0x9f,
// fixmap 0x80-0x8f, nil 0xc0, false 0xc2, true 0xc3.

use rxchef::operations::to_message_pack::ToMessagePack;
use rxchef::Operation;

fn pack(json: &str) -> Vec<u8> {
    ToMessagePack.run(json.as_bytes().to_vec(), &[]).unwrap()
}

#[test]
fn test_to_message_pack_positive_fixint() {
    // 0x00-0x7f encode themselves.
    assert_eq!(pack("0"), vec![0x00]);
    assert_eq!(pack("1"), vec![0x01]);
    assert_eq!(pack("127"), vec![0x7F]);
}

#[test]
fn test_to_message_pack_nil_and_booleans() {
    assert_eq!(pack("null"), vec![0xC0]);
    assert_eq!(pack("false"), vec![0xC2]);
    assert_eq!(pack("true"), vec![0xC3]);
}

#[test]
fn test_to_message_pack_fixstr() {
    // fixstr: 0b101XXXXX where XXXXX is the byte length.
    assert_eq!(pack("\"\""), vec![0xA0]);
    assert_eq!(pack("\"a\""), vec![0xA1, b'a']);
    assert_eq!(pack("\"abc\""), vec![0xA3, b'a', b'b', b'c']);
}

#[test]
fn test_to_message_pack_fixarray() {
    // fixarray: 0b1001XXXX where XXXX is the element count.
    assert_eq!(pack("[]"), vec![0x90]);
    assert_eq!(pack("[1,2,3]"), vec![0x93, 0x01, 0x02, 0x03]);
}

#[test]
fn test_to_message_pack_fixmap() {
    // fixmap: 0b1000XXXX where XXXX is the pair count.
    assert_eq!(pack("{}"), vec![0x80]);
    assert_eq!(pack("{\"a\":1}"), vec![0x81, 0xA1, b'a', 0x01]);
}

#[test]
fn test_to_message_pack_nested_structure() {
    // {"a":[1,2]} -> fixmap(1), fixstr(1)"a", fixarray(2), 1, 2
    assert_eq!(
        pack("{\"a\":[1,2]}"),
        vec![0x81, 0xA1, b'a', 0x92, 0x01, 0x02]
    );
}

#[test]
fn test_to_message_pack_utf8_string_length_is_in_bytes() {
    // "é" is one character but two UTF-8 bytes, so the fixstr length is 2.
    assert_eq!(pack("\"é\""), vec![0xA2, 0xC3, 0xA9]);
}

#[test]
fn test_to_message_pack_rejects_invalid_json() {
    assert!(ToMessagePack.run(b"{not json".to_vec(), &[]).is_err());
    assert!(ToMessagePack.run(b"[1,".to_vec(), &[]).is_err());
    // Invalid UTF-8 cannot be JSON either.
    assert!(ToMessagePack.run(vec![0xFF, 0xFE], &[]).is_err());
}

#[test]
fn test_to_message_pack_empty_input_produces_empty_output() {
    // Deliberate: an empty buffer short-circuits to an empty result rather
    // than being reported as malformed JSON, so empty pipeline stages pass
    // through unchanged.
    assert_eq!(
        ToMessagePack.run(Vec::new(), &[]).unwrap(),
        Vec::<u8>::new()
    );
}

#[test]
fn test_to_message_pack_roundtrips_through_from_message_pack() {
    use rxchef::operations::from_message_pack::FromMessagePack;

    for document in ["1", "\"abc\"", "[1,2,3]", "{\"a\":1}", "null", "true"] {
        let decoded = FromMessagePack.run(pack(document), &[]).unwrap();
        let decoded: serde_json::Value = serde_json::from_slice(&decoded).unwrap();
        let expected: serde_json::Value = serde_json::from_str(document).unwrap();
        assert_eq!(decoded, expected);
    }
}
