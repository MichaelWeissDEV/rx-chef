// Tests for the show_on_map operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations show_on_map::

use rxchef::operation::ArgValue;
use rxchef::operations::show_on_map::ShowOnMap;
use rxchef::Operation;

#[test]
fn test_show_on_map_basic() {
    let op = ShowOnMap;
    let input = b"51.504, -0.126".to_vec();
    let args = [
        ArgValue::Num(13.0),
        ArgValue::Str("Auto".to_string()),
        ArgValue::Str("Auto".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("51.504, -0.126"));
    assert!(result_str.contains("leaflet.js"));
}
#[test]
fn test_show_on_map_dms() {
    let op = ShowOnMap;
    let input = b"51\xC2\xB0 30' 14.4\" N, 0\xC2\xB0 7' 33.6\" W".to_vec();
    let args = [
        ArgValue::Num(13.0),
        ArgValue::Str("Auto".to_string()),
        ArgValue::Str("Auto".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    // 51 + 30/60 + 14.4/3600 = 51.504
    // -(0 + 7/60 + 33.6/3600) = -0.126
    assert!(result_str.contains("51.504, -0.126"));
}
#[test]
fn test_show_on_map_empty() {
    let op = ShowOnMap;
    let input = b"".to_vec();
    let args = [
        ArgValue::Num(10.0),
        ArgValue::Str("Auto".to_string()),
        ArgValue::Str("Auto".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("0, 0"));
    assert!(result_str.contains("setView([0, 0], 10)"));
}
