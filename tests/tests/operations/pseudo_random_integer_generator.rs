// Tests for the pseudo_random_integer_generator operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations pseudo_random_integer_generator::

use rxchef::operations::pseudo_random_integer_generator::PseudoRandomIntegerGenerator;
use rxchef::Operation;

#[test]
fn test_pseudo_random_integer_generator_default() {
    let op = PseudoRandomIntegerGenerator;
    let args = [
        rxchef::operation::ArgValue::Num(1.0),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Num(99.0),
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Str("Decimal".to_string()),
    ];
    let result = op.run(vec![], &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    // Should generate one number between 0 and 99
    let num = result_str.parse::<i64>().unwrap();
    assert!((0..=99).contains(&num));
}

#[test]
fn test_pseudo_random_integer_generator_multiple() {
    let op = PseudoRandomIntegerGenerator;
    let args = [
        rxchef::operation::ArgValue::Num(5.0),
        rxchef::operation::ArgValue::Num(1.0),
        rxchef::operation::ArgValue::Num(10.0),
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Str("Decimal".to_string()),
    ];
    let result = op.run(vec![], &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    // Should generate 5 numbers
    let numbers: Vec<i64> = result_str.split(' ').map(|s| s.parse().unwrap()).collect();
    assert_eq!(numbers.len(), 5);
    // All numbers should be between 1 and 10
    for num in numbers {
        assert!((1..=10).contains(&num));
    }
}

#[test]
fn test_pseudo_random_integer_generator_hex_output() {
    let op = PseudoRandomIntegerGenerator;
    let args = [
        rxchef::operation::ArgValue::Num(3.0),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Num(255.0),
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Str("Hex".to_string()),
    ];
    let result = op.run(vec![], &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    // Should generate 3 hex numbers
    let hex_numbers: Vec<&str> = result_str.split(' ').collect();
    assert_eq!(hex_numbers.len(), 3);
    // Each should be a valid hex number
    for hex_num in hex_numbers {
        assert!(i64::from_str_radix(&hex_num, 16).is_ok());
    }
}

#[test]
fn test_pseudo_random_integer_generator_negative_range() {
    let op = PseudoRandomIntegerGenerator;
    let args = [
        rxchef::operation::ArgValue::Num(2.0),
        rxchef::operation::ArgValue::Num(-10.0),
        rxchef::operation::ArgValue::Num(10.0),
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Str("Decimal".to_string()),
    ];
    let result = op.run(vec![], &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    // Should generate 2 numbers between -10 and 10
    let numbers: Vec<i64> = result_str.split(' ').map(|s| s.parse().unwrap()).collect();
    assert_eq!(numbers.len(), 2);
    for num in numbers {
        assert!((-10..=10).contains(&num));
    }
}

#[test]
fn test_pseudo_random_integer_generator_custom_delimiter() {
    let op = PseudoRandomIntegerGenerator;
    let args = [
        rxchef::operation::ArgValue::Num(3.0),
        rxchef::operation::ArgValue::Num(1.0),
        rxchef::operation::ArgValue::Num(5.0),
        rxchef::operation::ArgValue::Str(",".to_string()),
        rxchef::operation::ArgValue::Str("Decimal".to_string()),
    ];
    let result = op.run(vec![], &args).unwrap();
    // Should successfully generate output with custom delimiter
    assert!(result.len() > 0);
}

#[test]
fn test_pseudo_random_integer_generator_no_delimiter() {
    let op = PseudoRandomIntegerGenerator;
    let args = [
        rxchef::operation::ArgValue::Num(3.0),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Num(9.0),
        rxchef::operation::ArgValue::Str("None".to_string()),
        rxchef::operation::ArgValue::Str("Decimal".to_string()),
    ];
    let result = op.run(vec![], &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    // Should have no delimiter
    assert!(!result_str.contains(" "));
    assert!(!result_str.contains(","));
    // Should be exactly 3 digits concatenated
    assert_eq!(result_str.len(), 3);
}

#[test]
fn test_pseudo_random_integer_generator_invalid_range() {
    let op = PseudoRandomIntegerGenerator;
    let args = [
        rxchef::operation::ArgValue::Num(1.0),
        rxchef::operation::ArgValue::Num(100.0),
        rxchef::operation::ArgValue::Num(10.0), // Min > Max
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Str("Decimal".to_string()),
    ];
    let result = op.run(vec![], &args);
    assert!(result.is_err());
}

#[test]
fn test_pseudo_random_integer_generator_large_range() {
    let op = PseudoRandomIntegerGenerator;
    let args = [
        rxchef::operation::ArgValue::Num(1.0),
        rxchef::operation::ArgValue::Num(-1000.0),
        rxchef::operation::ArgValue::Num(1000.0),
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Str("Decimal".to_string()),
    ];
    let result = op.run(vec![], &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    // Should generate one number between -1000 and 1000
    let num = result_str.parse::<i64>().unwrap();
    assert!((-1000..=1000).contains(&num));
}

#[test]
fn test_pseudo_random_integer_generator_raw_output() {
    let op = PseudoRandomIntegerGenerator;
    let args = [
        rxchef::operation::ArgValue::Num(2.0),
        rxchef::operation::ArgValue::Num(65.0),
        rxchef::operation::ArgValue::Num(90.0), // A-Z in ASCII
        rxchef::operation::ArgValue::Str("None".to_string()),
        rxchef::operation::ArgValue::Str("Raw".to_string()),
    ];
    let result = op.run(vec![], &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    // Should generate 2 raw characters (letters A-Z)
    assert_eq!(result_str.len(), 2);
    // Should be uppercase letters
    for ch in result_str.chars() {
        assert!(ch.is_ascii_uppercase());
    }
}

#[test]
fn test_pseudo_random_integer_generator_line_feed_delimiter() {
    let op = PseudoRandomIntegerGenerator;
    let args = [
        rxchef::operation::ArgValue::Num(2.0),
        rxchef::operation::ArgValue::Num(1.0),
        rxchef::operation::ArgValue::Num(3.0),
        rxchef::operation::ArgValue::Str("Line feed".to_string()),
        rxchef::operation::ArgValue::Str("Decimal".to_string()),
    ];
    let result = op.run(vec![], &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    // Should use line feed delimiter
    assert!(result_str.contains('\n'));
    // Should have 2 numbers
    let numbers: Vec<i64> = result_str.split('\n').map(|s| s.parse().unwrap()).collect();
    assert_eq!(numbers.len(), 2);
}