// Tests for the convert_speed operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations convert_speed::

use rxchef::operation::ArgValue;
use rxchef::operations::convert_speed::ConvertSpeed;
use rxchef::Operation;

fn run(input: &str, from: &str, to: &str) -> f64 {
    let op = ConvertSpeed;
    let args = [ArgValue::Str(from.into()), ArgValue::Str(to.into())];
    String::from_utf8(op.run(input.as_bytes().to_vec(), &args).unwrap())
        .unwrap()
        .parse()
        .unwrap()
}
#[test]
fn test_ms_to_kmh() {
    let r = run("1", "Metres per second (m/s)", "Kilometres per hour (km/h)");
    assert!((r - 3.6).abs() < 1e-6);
}
#[test]
fn test_mph_to_kmh() {
    // One international mile is exactly 1.609344 km, so 60 mph is exactly
    // 96.56064 km/h (NIST Handbook 44, Appendix C).
    let actual = run("60", "Miles per hour (mph)", "Kilometres per hour (km/h)");
    assert!((actual - 96.56064).abs() < 1e-12);
}
