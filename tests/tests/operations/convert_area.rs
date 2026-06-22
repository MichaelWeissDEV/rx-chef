// Tests for the convert_area operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations convert_area::

use rxchef::operation::ArgValue;
use rxchef::operations::convert_area::ConvertArea;
use rxchef::Operation;

fn run(input: &str, from: &str, to: &str) -> f64 {
    let op = ConvertArea;
    let args = [ArgValue::Str(from.into()), ArgValue::Str(to.into())];
    String::from_utf8(op.run(input.as_bytes().to_vec(), &args).unwrap())
        .unwrap()
        .parse()
        .unwrap()
}
#[test]
fn test_sqm_to_sqkm() {
    let r = run("1000000", "Square metre (sq m)", "Square kilometre (sq km)");
    assert!((r - 1.0).abs() < 1e-6);
}
#[test]
fn test_ha_to_sqm() {
    let r = run("1", "Hectare (ha)", "Square metre (sq m)");
    assert!((r - 10000.0).abs() < 1e-3);
}
