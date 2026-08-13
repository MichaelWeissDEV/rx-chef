use rxchef::operation::ArgValue;
use rxchef::operations::amf_decode::AmfDecode;
use rxchef::Operation;

#[test]
fn test_amf_decode_rejects_invalid_input_and_format() {
    assert!(AmfDecode
        .run(b"not amf".to_vec(), &[ArgValue::Str("AMF3".into())])
        .is_err());
    assert!(AmfDecode
        .run(Vec::new(), &[ArgValue::Str("AMF4".into())])
        .is_err());
}
