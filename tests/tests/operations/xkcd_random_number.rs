// Tests for the xkcd_random_number operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations xkcd_random_number::

use rxchef::operation::ArgValue;
use rxchef::operations::xkcd_random_number::XkcdRandomNumberOp;
use rxchef::Operation;

#[test]
fn test_xkcd_random_number_basic() {
    let op = XkcdRandomNumberOp;
    let input = b"any input".to_vec();
    let args: [ArgValue; 0] = [];
    
    let result = op.run(input, &args);
    
    assert!(result.is_ok(), "XKCD random number should succeed");
    let output_bytes = result.unwrap();
    let output = String::from_utf8(output_bytes).unwrap();
    assert_eq!(output, "4", "Should always return 4 (RFC 1149.5)");
}

#[test]
fn test_xkcd_random_number_empty_input() {
    let op = XkcdRandomNumberOp;
    let input = b"".to_vec();
    let args: [ArgValue; 0] = [];
    
    let result = op.run(input, &args);
    
    assert!(result.is_ok(), "XKCD random number with empty input should succeed");
    let output_bytes = result.unwrap();
    let output = String::from_utf8(output_bytes).unwrap();
    assert_eq!(output, "4", "Should always return 4 even with empty input");
}

#[test]
fn test_xkcd_random_number_ignores_input() {
    let op = XkcdRandomNumberOp;
    
    // Test with different inputs - should always return 4
    for input in [b"test".to_vec(), b"12345".to_vec(), b"hello world".to_vec()] {
        let result = op.run(input, &[]);
        assert!(result.is_ok(), "Should succeed with any input");
        let output_bytes = result.unwrap();
        let output = String::from_utf8(output_bytes).unwrap();
        assert_eq!(output, "4", "Should always return 4 regardless of input");
    }
}

#[test]
fn test_xkcd_random_number_output_type() {
    let op = XkcdRandomNumberOp;
    let input = b"test".to_vec();
    let args: [ArgValue; 0] = [];
    
    let result = op.run(input, &args);
    
    assert!(result.is_ok(), "Should succeed");
    // Output should be a number (as string "4")
    let output_bytes = result.unwrap();
    let output_str = String::from_utf8(output_bytes).unwrap();
    assert!(output_str.parse::<u32>().is_ok(), "Output should be a valid number");
}

#[test]
fn test_xkcd_random_number_rfc_compliance() {
    let op = XkcdRandomNumberOp;
    let result = op.run(vec![], &[]);
    
    assert!(result.is_ok(), "Should comply with RFC 1149.5");
    let output_bytes = result.unwrap();
    let output = String::from_utf8(output_bytes).unwrap();
    assert_eq!(output, "4", "RFC 1149.5 specifies 4 as the standard IEEE-vetted random number");
}
