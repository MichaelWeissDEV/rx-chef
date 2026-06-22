// Tests for the convert_data_units operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations convert_data_units::

use rxchef::operation::ArgValue;
use rxchef::operations::convert_data_units::ConvertDataUnits;
use rxchef::Operation;

fn run(input: &str, from: &str, to: &str) -> f64 {
    let op = ConvertDataUnits;
    let args = [ArgValue::Str(from.into()), ArgValue::Str(to.into())];
    String::from_utf8(op.run(input.as_bytes().to_vec(), &args).unwrap())
        .unwrap()
        .parse()
        .unwrap()
}
#[test]
fn test_bytes_to_bits() {
    assert!((run("1", "Bytes (B)", "Bits (b)") - 8.0).abs() < 1e-9);
}
#[test]
fn test_kb_to_bytes() {
    assert!((run("1", "Kilobytes (kB)", "Bytes (B)") - 1000.0).abs() < 1e-6);
}
#[test]
fn test_gib_to_gb() {
    let r = run("1", "Gibibytes (GiB)", "Gigabytes (GB)");
    assert!((r - 1.073741824).abs() < 1e-6);
}
