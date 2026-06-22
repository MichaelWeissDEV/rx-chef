// Tests for the streebog operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations streebog::

use rxchef::operation::ArgValue;
use rxchef::operations::streebog::Streebog;
use rxchef::Operation;

#[test]
fn test_streebog_256_empty() {
    let op = Streebog;
    let input = b"".to_vec();
    let args = [ArgValue::Str("256".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(
        String::from_utf8(result).unwrap(),
        "3f539a213e97c802cc229d474c6aa32a825a360b2a933a949fd925208d9ce1bb"
    );
}
#[test]
fn test_streebog_512_empty() {
    let op = Streebog;
    let input = b"".to_vec();
    let args = [ArgValue::Str("512".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(
        String::from_utf8(result).unwrap(), "8e945da209aa869f0455928529bcae4679e9873ab707b55315f56ceb98bef0a7362f715528356ee83cda5f2aac4c6ad2ba3a715c1bcd81cb8e9f90bf4c1c1a8a");
}
#[test]
fn test_streebog_512_abc() {
    let op = Streebog;
    let input = b"abc".to_vec();
    let args = [ArgValue::Str("512".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(
        String::from_utf8(result).unwrap(), "28156e28317da7c98f4fe2bed6b542d0dab85bb224445fcedaf75d46e26d7eb8d5997f3e0915dd6b7f0aab08d9c8beb0d8c64bae2ab8b3c8c6bc53b3bf0db728");
}
#[test]
fn test_streebog_invalid_length() {
    let op = Streebog;
    let input = b"abc".to_vec();
    let args = [ArgValue::Str("128".to_string())];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
