// Tests for the md6 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations md6::

use rxchef::operations::md6::MD6;
use rxchef::Operation;

#[test]
fn test_md6_256_empty_vector() {
    let op = MD6;
    let result = op.run(Vec::new(), &[]).unwrap();
    assert!(!op.is_broken());
    assert_eq!(
        String::from_utf8(result).unwrap(),
        "bca38b24a804aa37d821d31af00f5598230122c5bbfc4c4ad5ed40e4258f04ca"
    );
}

#[test]
fn test_md6_honours_levels_and_key() {
    use rxchef::operation::ArgValue;

    let op = MD6;
    let unkeyed = op.run(b"test".to_vec(), &[]).unwrap();
    let keyed = op
        .run(
            b"test".to_vec(),
            &[
                ArgValue::Num(256.0),
                ArgValue::Num(0.0),
                ArgValue::Str("secret".to_string()),
            ],
        )
        .unwrap();
    assert_ne!(unkeyed, keyed);
    assert_eq!(keyed.len(), 64);
}

#[test]
fn test_md6_invalid_size_zero() {
    use rxchef::operation::ArgValue;
    let op = MD6;
    let res = op.run(
        b"test".to_vec(),
        &[ArgValue::Num(0.0), ArgValue::Num(64.0), ArgValue::Str("".to_string())],
    );
    assert!(res.is_err());
}

#[test]
fn test_md6_invalid_size_too_large() {
    use rxchef::operation::ArgValue;
    let op = MD6;
    let res = op.run(
        b"test".to_vec(),
        &[ArgValue::Num(513.0), ArgValue::Num(64.0), ArgValue::Str("".to_string())],
    );
    assert!(res.is_err());
}

#[test]
fn test_md6_invalid_levels() {
    use rxchef::operation::ArgValue;
    let op = MD6;
    let res = op.run(
        b"test".to_vec(),
        &[ArgValue::Num(256.0), ArgValue::Num(256.0), ArgValue::Str("".to_string())],
    );
    assert!(res.is_err());
}

#[test]
fn test_md6_invalid_key_too_long() {
    use rxchef::operation::ArgValue;
    let op = MD6;
    let res = op.run(
        b"test".to_vec(),
        &[
            ArgValue::Num(256.0),
            ArgValue::Num(64.0),
            ArgValue::Str("a".repeat(65)),
        ],
    );
    assert!(res.is_err());
}
