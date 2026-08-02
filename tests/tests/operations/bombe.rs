// Tests for the bombe operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations bombe::

use rxchef::operation::ArgValue;
use rxchef::operations::bombe::Bombe;
use rxchef::Operation;

#[test]
fn test_bombe_empty_crib() {
    let op = Bombe;
    let input = b"HELLOWORLD".to_vec();
    let args = [
        ArgValue::Str("3-rotor".to_string()),
        ArgValue::Str("LEYJVCNIXWPBQMDRTAKZGFUHOS".to_string()),
        ArgValue::Str("EKMFLGDQVZNTOWYHXUSPAIBRCJ".to_string()),
        ArgValue::Str("AJDKSIRUXBLHWTMCQGZNPYFVOE".to_string()),
        ArgValue::Str("BDFHJLCPRTXVZNYEIWGAKMUSQO".to_string()),
        ArgValue::Str("AY BR CU DH EQ FS GL IP JX KN MO TZ VW".to_string()),
        ArgValue::Str("".to_string()), // Empty crib
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
    ];
    
    let result = op.run(input, &args);
    
    // Should return error for empty crib
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("Crib cannot be empty"));
    }
}

#[test]
fn test_bombe_crib_overrun() {
    let op = Bombe;
    let input = b"HELLO".to_vec(); // 5 characters
    let args = [
        ArgValue::Str("3-rotor".to_string()),
        ArgValue::Str("LEYJVCNIXWPBQMDRTAKZGFUHOS".to_string()),
        ArgValue::Str("EKMFLGDQVZNTOWYHXUSPAIBRCJ".to_string()),
        ArgValue::Str("AJDKSIRUXBLHWTMCQGZNPYFVOE".to_string()),
        ArgValue::Str("BDFHJLCPRTXVZNYEIWGAKMUSQO".to_string()),
        ArgValue::Str("AY BR CU DH EQ FS GL IP JX KN MO TZ VW".to_string()),
        ArgValue::Str("WORLDHELLO".to_string()), // 10 character crib
        ArgValue::Num(0.0), // offset 0
        ArgValue::Bool(true),
    ];
    
    let result = op.run(input, &args);
    
    // Should return error for crib overrun
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("Crib overruns supplied ciphertext"));
    }
}

#[test]
fn test_bombe_basic_operation() {
    let op = Bombe;
    let input = b"HELLOWORLD".to_vec();
    let args = [
        ArgValue::Str("3-rotor".to_string()),
        ArgValue::Str("LEYJVCNIXWPBQMDRTAKZGFUHOS".to_string()),
        ArgValue::Str("EKMFLGDQVZNTOWYHXUSPAIBRCJ".to_string()),
        ArgValue::Str("AJDKSIRUXBLHWTMCQGZNPYFVOE".to_string()),
        ArgValue::Str("BDFHJLCPRTXVZNYEIWGAKMUSQO".to_string()),
        ArgValue::Str("AY BR CU DH EQ FS GL IP JX KN MO TZ VW".to_string()),
        ArgValue::Str("HELLO".to_string()), // Crib
        ArgValue::Num(0.0), // offset 0
        ArgValue::Bool(true),
    ];
    
    let result = op.run(input, &args);
    
    // Should successfully run the bombe operation
    assert!(result.is_ok(), "Bombe operation should succeed");
    if let Ok(output) = result {
        assert!(!output.is_empty(), "Output should not be empty");
        // Output should be valid JSON
        let output_str = String::from_utf8_lossy(&output);
        assert!(output_str.contains("n_loops"), "Output should contain n_loops");
        assert!(output_str.contains("result"), "Output should contain result");
    }
}

#[test]
fn test_bombe_with_offset() {
    let op = Bombe;
    let input = b"HELLOWORLD".to_vec();
    let args = [
        ArgValue::Str("3-rotor".to_string()),
        ArgValue::Str("LEYJVCNIXWPBQMDRTAKZGFUHOS".to_string()),
        ArgValue::Str("EKMFLGDQVZNTOWYHXUSPAIBRCJ".to_string()),
        ArgValue::Str("AJDKSIRUXBLHWTMCQGZNPYFVOE".to_string()),
        ArgValue::Str("BDFHJLCPRTXVZNYEIWGAKMUSQO".to_string()),
        ArgValue::Str("AY BR CU DH EQ FS GL IP JX KN MO TZ VW".to_string()),
        ArgValue::Str("ORLD".to_string()), // Crib
        ArgValue::Num(1.0), // offset 1
        ArgValue::Bool(false), // Don't use checking machine
    ];
    
    let result = op.run(input, &args);
    
    // Should successfully run with offset
    assert!(result.is_ok(), "Bombe operation with offset should succeed");
}

#[test]
fn test_bombe_4_rotor() {
    let op = Bombe;
    let input = b"HELLOWORLD".to_vec();
    let args = [
        ArgValue::Str("4-rotor".to_string()),
        ArgValue::Str("LEYJVCNIXWPBQMDRTAKZGFUHOS".to_string()),
        ArgValue::Str("EKMFLGDQVZNTOWYHXUSPAIBRCJ".to_string()),
        ArgValue::Str("AJDKSIRUXBLHWTMCQGZNPYFVOE".to_string()),
        ArgValue::Str("BDFHJLCPRTXVZNYEIWGAKMUSQO".to_string()),
        ArgValue::Str("AY BR CU DH EQ FS GL IP JX KN MO TZ VW".to_string()),
        ArgValue::Str("HELLO".to_string()), // Crib
        ArgValue::Num(0.0), // offset 0
        ArgValue::Bool(true),
    ];
    
    let result = op.run(input, &args);
    
    // Should successfully run with 4-rotor model
    assert!(result.is_ok(), "Bombe operation with 4-rotor should succeed");
}
