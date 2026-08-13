use rxchef::operation::ArgValue;
use rxchef::operations::{amf_decode::AmfDecode, amf_encode::AmfEncode};
use rxchef::Operation;

#[test]
fn test_amf_roundtrip_both_formats() {
    let input = br#"{"active":true,"items":[1,"two",null],"name":"rxchef"}"#;
    for format in ["AMF0", "AMF3"] {
        let args = [ArgValue::Str(format.into())];
        let encoded = AmfEncode.run(input.to_vec(), &args).unwrap();
        assert!(!encoded.is_empty());
        let decoded = AmfDecode.run(encoded, &args).unwrap();
        let decoded: serde_json::Value = serde_json::from_slice(&decoded).unwrap();
        assert_eq!(decoded["active"], true);
        assert_eq!(decoded["items"][0].as_f64(), Some(1.0));
        assert_eq!(decoded["items"][1], "two");
        assert!(decoded["items"][2].is_null());
        assert_eq!(decoded["name"], "rxchef");
    }
}
