use rxchef::operation::ArgValue;
use rxchef::operations::{gost_key_unwrap::GOSTKeyUnwrapOp, gost_key_wrap::GostKeyWrap};
use rxchef::Operation;

#[test]
fn test_gost_key_wrap_roundtrip() {
    for algorithm in [
        "GOST R 34.12 (Magma, 2015)",
        "GOST R 34.12 (Kuznyechik, 2015)",
    ] {
        let key = ArgValue::Str("0123456789abcdef0123456789abcdef".into());
        let cek = b"abcdefghijklmnopqrstuvwxyz012345".to_vec();
        let wrap_args = [
            key,
            ArgValue::Str("12345678".into()),
            ArgValue::Str("Raw".into()),
            ArgValue::Str("Hex".into()),
            ArgValue::Str(algorithm.into()),
            ArgValue::Str("E-TEST".into()),
            ArgValue::Str("NO".into()),
        ];
        let wrapped = GostKeyWrap.run(cek.clone(), &wrap_args).unwrap();
        let mut unwrap_args = wrap_args;
        unwrap_args[2] = ArgValue::Str("Hex".into());
        unwrap_args[3] = ArgValue::Str("Raw".into());
        let unwrapped = GOSTKeyUnwrapOp.run(wrapped, &unwrap_args).unwrap();
        assert_eq!(unwrapped, cek);
    }
}
