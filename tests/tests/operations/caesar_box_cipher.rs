// Tests for the caesar_box_cipher operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations caesar_box_cipher::

use rxchef::operation::ArgValue;
use rxchef::operations::caesar_box_cipher::CaesarBoxCipher;
use rxchef::Operation;

#[test]
fn test_caesar_box_cipher_basic() {
    let op = CaesarBoxCipher;
    let input = "HELLO WORLD".to_string();
    let args = [ArgValue::Num(2.0)]; // Box height = 2
    
    let result = op.run(input.into_bytes(), &args);
    
    assert!(result.is_ok(), "Caesar box cipher should succeed");
    if let Ok(output) = result {
        let output_str = String::from_utf8_lossy(&output);
        // "HELLO WORLD" with spaces removed = "HELLOWORLD" (10 chars)
        // With height=2, width=5, reading by columns:
        // H E L L O
        // W O R L D
        // Columns: H W, E O, L R, L L, O D -> "HWEOLRLLOD"
        // But the actual algorithm reads: H(0), W(5), E(1), O(6), L(2), R(7), L(3), L(8), O(4), D(9) -> "HWEOLRLLOD"
        // Wait, let me recalculate: the algorithm does (i..chars.len()).step_by(table_height)
        // For i=0: 0, 2, 4, 6, 8 -> H, L, O, R, L
        // For i=1: 1, 3, 5, 7, 9 -> E, L, W, L, D
        // So the actual output is "HLOOLELWRD"
        assert_eq!(output_str, "HLOOLELWRD", "Caesar box cipher output should match expected");
    }
}

#[test]
fn test_caesar_box_cipher_height_1() {
    let op = CaesarBoxCipher;
    let input = "HELLO".to_string();
    let args = [ArgValue::Num(1.0)]; // Box height = 1 (identity)
    
    let result = op.run(input.into_bytes(), &args);
    
    assert!(result.is_ok(), "Caesar box cipher with height 1 should succeed");
    if let Ok(output) = result {
        let output_str = String::from_utf8_lossy(&output);
        // Height=1 means no transposition
        assert_eq!(output_str, "HELLO", "Height 1 should be identity");
    }
}

#[test]
fn test_caesar_box_cipher_height_3() {
    let op = CaesarBoxCipher;
    let input = "HELLOWORLD".to_string();
    let args = [ArgValue::Num(3.0)]; // Box height = 3
    
    let result = op.run(input.into_bytes(), &args);
    
    assert!(result.is_ok(), "Caesar box cipher with height 3 should succeed");
    if let Ok(output) = result {
        let output_str = String::from_utf8_lossy(&output);
        // "HELLOWORLD" (10 chars) with height=3, width=4:
        // H E L L
        // O W O R
        // L D    
        // Algorithm: for i in 0..3: (i..10).step_by(3)
        // i=0: 0, 3, 6, 9 -> H, L, O, D
        // i=1: 1, 4, 7 -> E, L, R
        // i=2: 2, 5, 8 -> L, W, L
        // Result: "HLODEORLWL"
        assert_eq!(output_str, "HLODEORLWL", "Caesar box cipher output should match expected");
    }
}

#[test]
fn test_caesar_box_cipher_empty_input() {
    let op = CaesarBoxCipher;
    let input = "".to_string();
    let args = [ArgValue::Num(2.0)];
    
    let result = op.run(input.into_bytes(), &args);
    
    assert!(result.is_ok(), "Empty input should be handled");
    assert_eq!(result.unwrap(), vec![0u8; 0], "Empty input should return empty output");
}

#[test]
fn test_caesar_box_cipher_zero_height() {
    let op = CaesarBoxCipher;
    let input = "HELLO".to_string();
    let args = [ArgValue::Num(0.0)]; // Invalid height = 0
    
    let result = op.run(input.into_bytes(), &args);
    
    // Should return error for zero height
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("Box Height must be greater than 0"));
    }
}

#[test]
fn test_caesar_box_cipher_with_spaces() {
    let op = CaesarBoxCipher;
    let input = "HELLO WORLD THIS IS A TEST".to_string();
    let args = [ArgValue::Num(4.0)]; // Box height = 4
    
    let result = op.run(input.into_bytes(), &args);
    
    assert!(result.is_ok(), "Caesar box cipher should handle spaces");
    if let Ok(output) = result {
        let output_str = String::from_utf8_lossy(&output);
        // Spaces should be removed before processing
        assert!(!output_str.contains(' '), "Spaces should be removed");
    }
}

#[test]
fn test_caesar_box_cipher_large_height() {
    let op = CaesarBoxCipher;
    let input = "HELLO".to_string();
    let args = [ArgValue::Num(100.0)]; // Box height larger than input
    
    let result = op.run(input.into_bytes(), &args);
    
    assert!(result.is_ok(), "Large height should be handled");
    if let Ok(output) = result {
        let output_str = String::from_utf8_lossy(&output);
        // With height=100, width=1, reading by columns should give same as input
        assert_eq!(output_str, "HELLO", "Large height should work like height 1");
    }
}
