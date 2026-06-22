// Tests for the convert_distance operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations convert_distance::

use rxchef::operation::ArgValue;
use rxchef::operations::convert_distance::ConvertDistance;
use rxchef::Operation;

fn run(input: &str, from: &str, to: &str) -> String {
    let op = ConvertDistance;
    let args = [ArgValue::Str(from.into()), ArgValue::Str(to.into())];
    String::from_utf8(op.run(input.as_bytes().to_vec(), &args).unwrap()).unwrap()
}
#[test]
fn test_m_to_km() {
    let r: f64 = run("1000", "Metres (m)", "Kilometers (km)")
        .parse()
        .unwrap();
    assert!((r - 1.0).abs() < 1e-9);
}
#[test]
fn test_miles_to_km() {
    let r: f64 = run("1", "Miles (mi)", "Kilometers (km)").parse().unwrap();
    assert!((r - 1.609344).abs() < 1e-6);
}
#[test]
fn test_invalid_number() {
    let op = ConvertDistance;
    let args = [
        ArgValue::Str("Metres (m)".into()),
        ArgValue::Str("Kilometers (km)".into()),
    ];
    assert!(op.run(b"not_a_number".to_vec(), &args).is_err());
}
