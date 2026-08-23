use rxchef::operation::ArgValue;
use rxchef::operations::amf_decode::AmfDecode;
use rxchef::Operation;

#[test]
fn test_amf0_object_from_published_wire_format() {
    // Adobe AMF0 specification, section 4: object marker 0x03, UTF-8 property
    // names, Number/Boolean values, and the 00 00 09 object-end marker.  This
    // is independently constructed wire data, not output from AmfEncode.
    let wire = hex::decode("03000178003ff000000000000000026f6b0101000009").unwrap();
    let decoded = AmfDecode
        .run(wire, &[ArgValue::Str("AMF0".into())])
        .unwrap();
    let value: serde_json::Value = serde_json::from_slice(&decoded).unwrap();
    assert_eq!(value, serde_json::json!({"x": 1.0, "ok": true}));
}

#[test]
fn test_amf_decode_rejects_invalid_input_and_format() {
    assert!(AmfDecode
        .run(b"not amf".to_vec(), &[ArgValue::Str("AMF3".into())])
        .is_err());
    assert!(AmfDecode
        .run(Vec::new(), &[ArgValue::Str("AMF4".into())])
        .is_err());
}
