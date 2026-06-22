// Tests for the convert_coordinate_format operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations convert_coordinate_format::

use rxchef::operation::ArgValue;
use rxchef::operations::convert_coordinate_format::ConvertCoordinateFormat;
use rxchef::Operation;

fn run_test(
    in_format: &str,
    in_delim: &str,
    out_format: &str,
    out_delim: &str,
    dir: &str,
    precision: f64,
    input: &str,
    expected: &str,
) {
    let op = ConvertCoordinateFormat;
    let args = vec![
        ArgValue::Str(in_format.to_string()),
        ArgValue::Str(in_delim.to_string()),
        ArgValue::Str(out_format.to_string()),
        ArgValue::Str(out_delim.to_string()),
        ArgValue::Str(dir.to_string()),
        ArgValue::Num(precision),
    ];
    let result = op.run(input.as_bytes().to_vec(), &args).unwrap();
    assert_eq!(String::from_utf8_lossy(&result), expected);
}
#[test]
fn test_convert_coordinate_format() {
    run_test(
        "Decimal Degrees",
        "Comma",
        "Degrees Minutes Seconds",
        "Comma",
        "None",
        1.0,
        "51.504,-0.126,",
        "51 30' 14.4\",-0 7' 33.6\",",
    );
    run_test(
        "Degrees Minutes Seconds",
        "Comma",
        "Decimal Degrees",
        "Comma",
        "None",
        3.0,
        "51 30' 14.4\",-0 7' 33.6\",",
        "51.504,-0.126,",
    );
    run_test(
        "Decimal Degrees",
        "Comma",
        "Degrees Decimal Minutes",
        "Comma",
        "None",
        2.0,
        "51.504,-0.126,",
        "51 30.24',-0 7.56',",
    );
    run_test(
        "Degrees Decimal Minutes",
        "Comma",
        "Decimal Degrees",
        "Comma",
        "None",
        3.0,
        "51 30.24',-0 7.56',",
        "51.504,-0.126,",
    );
    run_test(
        "Decimal Degrees",
        "Comma",
        "Decimal Degrees",
        "Comma",
        "None",
        3.0,
        "51.504,-0.126,",
        "51.504,-0.126,",
    );
    run_test(
        "Decimal Degrees",
        "Comma",
        "Geohash",
        "Comma",
        "None",
        9.0,
        "51.504,-0.126,",
        "gcpvj0h0x,",
    );
    run_test(
        "Geohash",
        "Comma",
        "Decimal Degrees",
        "Comma",
        "None",
        3.0,
        "gcpvj0h0x,",
        "51.504,-0.126,",
    );
    run_test(
        "Decimal Degrees",
        "Comma",
        "Military Grid Reference System",
        "Comma",
        "None",
        10.0,
        "51.504,-0.126,",
        "30U XC 99455 09790,",
    );
    run_test(
        "Military Grid Reference System",
        "Comma",
        "Decimal Degrees",
        "Comma",
        "None",
        3.0,
        "30U XC 99455 09790,",
        "51.504,-0.126,",
    );
    run_test(
        "Decimal Degrees",
        "Comma",
        "Ordnance Survey National Grid",
        "Comma",
        "None",
        10.0,
        "51.504,-0.126,",
        "TQ 30163 80005,",
    );
    run_test(
        "Ordnance Survey National Grid",
        "Comma",
        "Decimal Degrees",
        "Comma",
        "None",
        3.0,
        "TQ 30163 80005,",
        "51.504,-0.126,",
    );
    run_test(
        "Decimal Degrees",
        "Comma",
        "Universal Transverse Mercator",
        "Comma",
        "None",
        0.0,
        "51.504,-0.126,",
        "30 N 699456 5709791,",
    );
    run_test(
        "Universal Transverse Mercator",
        "Comma",
        "Decimal Degrees",
        "Comma",
        "None",
        3.0,
        "30 N 699456 5709791,",
        "51.504,-0.126,",
    );
    run_test(
        "Decimal Degrees",
        "Comma",
        "Decimal Degrees",
        "Comma",
        "None",
        3.0,
        "N51.504,W0.126,",
        "51.504,-0.126,",
    );
    run_test(
        "Decimal Degrees",
        "Comma",
        "Decimal Degrees",
        "Comma",
        "Before",
        3.0,
        "N51.504,W0.126,",
        "N 51.504,W 0.126,",
    );
    run_test(
        "Decimal Degrees",
        "Comma",
        "Decimal Degrees",
        "Comma",
        "Before",
        3.0,
        "51.504,-0.126,",
        "N 51.504,W 0.126,",
    );
    run_test(
        "Decimal Degrees",
        "Comma",
        "Degrees Minutes Seconds",
        "Comma",
        "Before",
        3.0,
        "51.504,-0.126,",
        "N 51 30' 14.4\",W 0 7' 33.6\",",
    );
}
