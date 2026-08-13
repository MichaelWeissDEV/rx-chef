// Tests for the hex_density_chart operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations hex_density_chart::

use rxchef::operation::ArgValue;
use rxchef::operations::hex_density_chart::HexDensityChartOp;
use rxchef::Operation;

#[test]
fn test_hex_density_chart_svg() {
    let op = HexDensityChartOp;
    let input = b"1,2\n3,4\n5,6".to_vec();
    let args = [
        ArgValue::Str("\\n".to_string()),
        ArgValue::Str(",".to_string()),
        ArgValue::Num(25.0),
        ArgValue::Num(15.0),
        ArgValue::Bool(true),
        ArgValue::Str("X Axis".to_string()),
        ArgValue::Str("Y Axis".to_string()),
        ArgValue::Bool(false),
        ArgValue::Str("#ffffff".to_string()),
        ArgValue::Str("#000000".to_string()),
        ArgValue::Bool(false),
    ];
    assert!(!op.is_broken());
    let output = String::from_utf8(op.run(input, &args).unwrap()).unwrap();
    assert!(output.starts_with("<svg"));
    assert!(output.contains("<polygon"));
    assert!(output.contains("X Axis"));
    assert!(output.contains("Y Axis"));
}
