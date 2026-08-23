// Tests for the derive_evp_key operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations derive_evp_key::

use rxchef::operation::ArgValue;
use rxchef::operations::derive_evp_key::DeriveEvpKey;
use rxchef::Operation;

#[test]
fn test_derive_evp_key() {
    let op = DeriveEvpKey;
    let passphrase = hex::decode("466c6561204d61726b6574").unwrap(); // "Flea Market"
                                                                     // OpenSSL's EVP_BytesToKey consumes an 8-byte salt. This exact invocation
                                                                     // is independently reproducible with:
                                                                     // openssl enc -aes-128-cbc -P -md md5 -S 4d61726b65740000 -k 'Flea Market'
    let salt = b"Market\0\0".to_vec();
    let args = [
        ArgValue::Bytes(passphrase),
        ArgValue::Num(128.0),
        ArgValue::Num(1.0),
        ArgValue::Str("MD5".to_string()),
        ArgValue::Bytes(salt),
    ];
    let result = op.run(vec![], &args).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "3e3a4d492c0c1b3704134b886e7e12a3");
}

#[test]
fn test_derive_evp_key_rejects_unknown_digest() {
    let error = DeriveEvpKey
        .run(
            vec![],
            &[
                ArgValue::Bytes(b"password".to_vec()),
                ArgValue::Num(128.0),
                ArgValue::Num(1.0),
                ArgValue::Str("NOT-A-DIGEST".into()),
                ArgValue::Bytes(b"12345678".to_vec()),
            ],
        )
        .unwrap_err();
    assert!(matches!(
        error,
        rxchef::operation::OperationError::InvalidArgument { ref name, .. }
            if name == "Hashing function"
    ));
}
