// Tests for the cipher_saber2_encrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations cipher_saber2_encrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::cipher_saber2_encrypt::CipherSaber2Encrypt;
use rxchef::Operation;

#[test]
fn test_encrypt_output_longer_than_input() {
    let op = CipherSaber2Encrypt;
    let input = b"Hello World".to_vec();
    let args = [ArgValue::Str("mykey".to_string()), ArgValue::Num(20.0)];
    let result = op.run(input.clone(), &args).unwrap();
    assert_eq!(result.len(), 10 + input.len());
}
