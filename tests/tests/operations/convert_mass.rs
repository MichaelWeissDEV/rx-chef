// Tests for the convert_mass operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations convert_mass::

use rxchef::operation::ArgValue;
use rxchef::operations::convert_mass::ConvertMass;
use rxchef::Operation;

fn run(input: &str, from: &str, to: &str) -> f64 {
    let op = ConvertMass;
    let args = [ArgValue::Str(from.into()), ArgValue::Str(to.into())];
    String::from_utf8(op.run(input.as_bytes().to_vec(), &args).unwrap())
        .unwrap()
        .parse()
        .unwrap()
}
#[test]
fn test_kg_to_g() {
    assert!((run("1", "Kilograms (kg)", "Grams (g)") - 1000.0).abs() < 1e-6);
}
#[test]
fn test_lb_to_kg() {
    assert!((run("1", "Pounds (lb)", "Kilograms (kg)") - 0.45359237).abs() < 1e-6);
}
