// Tests for the sha3 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations sha3::

use rxchef::operation::ArgValue;
use rxchef::operations::sha3::SHA3;
use rxchef::Operation;

#[test]
fn test_sha3_256_basic() {
    let operation = SHA3;
    let input = b"Hello, World!".to_vec();
    let result = operation.run(input, &[ArgValue::Num(256.0)]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // SHA3-256 hash of "Hello, World!"
    assert_eq!(
        output,
        "1af17a664e3fa8e419b8ba05c2a173169df76162a5a286e0c405b460d478f7ef"
    );
}
#[test]
fn test_sha3_224_basic() {
    let operation = SHA3;
    let input = b"Hello, World!".to_vec();
    let result = operation.run(input, &[ArgValue::Num(224.0)]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // SHA3-224 hash of "Hello, World!"
    assert_eq!(
        output,
        "853048fb8b11462b6100385633c0cc8dcdc6e2b8e376c28102bc84f2"
    );
}
#[test]
fn test_sha3_384_basic() {
    let operation = SHA3;
    let input = b"Hello, World!".to_vec();
    let result = operation.run(input, &[ArgValue::Num(384.0)]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // SHA3-384 hash of "Hello, World!"
    assert_eq!(
        output,
        "aa9ad8a49f31d2ddcabbb7010a1566417cff803fef50eba239558826f872e468c5743e7f026b0a8e5b2d7a1cc465cdbe"
    );
}
#[test]
fn test_sha3_512_basic() {
    let operation = SHA3;
    let input = b"Hello, World!".to_vec();
    let result = operation.run(input, &[ArgValue::Num(512.0)]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // SHA3-512 hash of "Hello, World!"
    assert_eq!(
        output,
        "38e05c33d7b067127f217d8c856e554fcff09c9320b8a5979ce2ff5d95dd27ba35d1fba50c562dfd1d6cc48bc9c5baa4390894418cc942d968f97bcb659419ed"
    );
}
#[test]
fn test_sha3_empty() {
    let operation = SHA3;
    let input = b"".to_vec();
    let result = operation.run(input, &[ArgValue::Num(256.0)]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // SHA3-256 hash of empty string
    assert_eq!(
        output,
        "a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a"
    );
}
#[test]
fn test_sha3_invalid_size() {
    let operation = SHA3;
    let input = b"test".to_vec();
    let args = [ArgValue::Num(100.0)]; // Invalid size
    let result = operation.run(input, &args);
    assert!(result.is_err());
}
