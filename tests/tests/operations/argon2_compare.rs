// Tests for the argon2_compare operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations argon2_compare::

use rxchef::operation::ArgValue;
use rxchef::operations::argon2_compare::Argon2Compare;
use rxchef::Operation;

#[test]
fn test_argon2_compare_reference_argon2id_vector() {
    let operation = Argon2Compare;
    // Argon2 reference vector, also published by the RustCrypto argon2 KATs:
    // Argon2id v=19, m=256 KiB, t=2, p=1, password "password", salt "somesalt".
    let args = [ArgValue::Str(
        "$argon2id$v=19$m=256,t=2,p=1$c29tZXNhbHQ$nf65EOgLrQMR/uIPnA4rEsF5h7TKyQwu9U1bMCHGi/4"
            .to_string(),
    )];

    assert_eq!(
        operation.run(b"password".to_vec(), &args).unwrap(),
        b"Match: password"
    );
}

#[test]
fn test_argon2_compare_empty_password_boundary() {
    let operation = Argon2Compare;
    let args = [ArgValue::Str(
        "$argon2id$v=19$m=256,t=2,p=1$c29tZXNhbHQ$nf65EOgLrQMR/uIPnA4rEsF5h7TKyQwu9U1bMCHGi/4"
            .to_string(),
    )];

    assert_eq!(operation.run(Vec::new(), &args).unwrap(), b"No match");
}

#[test]
fn test_argon2_compare_rejects_non_utf8_password() {
    let operation = Argon2Compare;
    let args = [ArgValue::Str(
        "$argon2id$v=19$m=256,t=2,p=1$c29tZXNhbHQ$nf65EOgLrQMR/uIPnA4rEsF5h7TKyQwu9U1bMCHGi/4"
            .to_string(),
    )];

    assert!(operation.run(vec![0xff], &args).is_err());
}
