use rxchef::operation::ArgValue;
use rxchef::operations::{gost_sign::GostSign, gost_verify::GOSTVerifyOp};
use rxchef::Operation;

#[test]
fn test_gost_verify_accepts_valid_and_rejects_invalid_mac() {
    let key = ArgValue::Str("0123456789abcdef0123456789abcdef".into());
    let iv = ArgValue::Str("12345678".into());
    let message = b"authenticated message".to_vec();
    let mac = GostSign
        .run(
            message.clone(),
            &[
                key.clone(),
                iv.clone(),
                ArgValue::Str("Raw".into()),
                ArgValue::Str("Hex".into()),
                ArgValue::Str("GOST R 34.12 (Magma, 2015)".into()),
                ArgValue::Str("E-TEST".into()),
                ArgValue::Str("32".into()),
            ],
        )
        .unwrap();
    let mut args = vec![
        key,
        iv,
        ArgValue::Str(String::from_utf8(mac).unwrap()),
        ArgValue::Str("Raw".into()),
        ArgValue::Str("GOST R 34.12 (Magma, 2015)".into()),
        ArgValue::Str("E-TEST".into()),
    ];
    assert_eq!(GOSTVerifyOp.run(message.clone(), &args).unwrap(), b"true");
    args[2] = ArgValue::Str("00000000".into());
    assert_eq!(GOSTVerifyOp.run(message, &args).unwrap(), b"false");
}
