// Tests for the ls47_encrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations ls47_encrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::ls47_encrypt::LS47Encrypt;
use rxchef::Operation;

#[test]
fn test_ls47_encrypt_basic() {
    let op = LS47Encrypt;
    let input = b"hello".to_vec();
    let args = vec![
        ArgValue::Str("password".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Str("".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    assert!(!result.is_empty());
}

#[test]
fn test_ls47_matches_upstream_reference_vector() {
    let args = [
        ArgValue::Str("helloworld".into()),
        ArgValue::Num(0.0),
        ArgValue::Str("test".into()),
    ];
    assert_eq!(
        LS47Encrypt
            .run(b"thequickbrownfoxjumped".to_vec(), &args)
            .unwrap(),
        b"(,t74ci78cp/8trx*yesu:alp1wqy"
    );
}

#[test]
fn test_ls47_empty_message_boundary() {
    let args = [
        ArgValue::Str("key".into()),
        ArgValue::Num(0.0),
        ArgValue::Str(String::new()),
    ];
    assert_eq!(LS47Encrypt.run(Vec::new(), &args).unwrap().len(), 3);
}

#[test]
fn test_ls47_rejects_character_outside_alphabet() {
    let args = [
        ArgValue::Str("key".into()),
        ArgValue::Num(0.0),
        ArgValue::Str(String::new()),
    ];
    assert!(LS47Encrypt.run(b"UPPERCASE".to_vec(), &args).is_err());
}
