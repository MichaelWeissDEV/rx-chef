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

#[test]
fn test_hex_density_two_bin_geometry_exactly() {
    let output = HexDensityChartOp
        .run(
            b"1,2\n3,4\n5,6".to_vec(),
            &[
                ArgValue::Str("\\n".into()),
                ArgValue::Str(",".into()),
                ArgValue::Num(2.0),
                ArgValue::Num(2.0),
                ArgValue::Bool(true),
                ArgValue::Str("X Axis".into()),
                ArgValue::Str("Y Axis".into()),
                ArgValue::Bool(false),
                ArgValue::Str("#ffffff".into()),
                ArgValue::Str("#000000".into()),
                ArgValue::Bool(false),
            ],
        )
        .unwrap();
    assert_eq!(
        String::from_utf8(output).unwrap(),
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 800 500\" role=\"img\"><rect width=\"800\" height=\"500\" fill=\"white\"/><path d=\"M70 25V435H770\" fill=\"none\" stroke=\"#333\"/><polygon points=\"72.00,425.00 71.00,426.73 69.00,426.73 68.00,425.00 69.00,423.27 71.00,423.27\" fill=\"#000000\" stroke=\"none\"><title>1 point(s)</title></polygon><polygon points=\"751.50,26.63 750.50,28.36 748.50,28.36 747.50,26.63 748.50,24.90 750.50,24.90\" fill=\"#000000\" stroke=\"none\"><title>1 point(s)</title></polygon><text x=\"420\" y=\"485\" text-anchor=\"middle\">X Axis</text><text x=\"18\" y=\"230\" text-anchor=\"middle\" transform=\"rotate(-90 18 230)\">Y Axis</text></svg>"
    );
}
