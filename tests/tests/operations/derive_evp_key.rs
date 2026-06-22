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
    let salt = b"Market".to_vec();
    let args = [
        ArgValue::Bytes(passphrase),
        ArgValue::Num(128.0),
        ArgValue::Num(1.0),
        ArgValue::Str("MD5".to_string()),
        ArgValue::Bytes(salt),
    ];
    let result = op.run(vec![], &args).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "ae8a09fb645c04b153312959f9328efd");
}
