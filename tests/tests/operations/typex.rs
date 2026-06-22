// Tests for the typex operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations typex::

use rxchef::operation::ArgValue;
use rxchef::operations::typex::Typex;
use rxchef::Operation;

#[test]
fn test_typex_basic() {
    let op = Typex;
    let input = b"HELLO".to_vec();
    // Use defaults
    let args = vec![
        ArgValue::Str("MCYLPQUVRXGSAOWNBJEZDTFKHI<BFHNQUW".to_string()), // 1
        ArgValue::Bool(false),                                           // reversed
        ArgValue::Str("A".to_string()),                                  // ring
        ArgValue::Str("A".to_string()),                                  // initial
        ArgValue::Str("KHWENRCBISXJQGOFMAPVYZDLTU<BFHNQUW".to_string()), // 2
        ArgValue::Bool(false),
        ArgValue::Str("A".to_string()),
        ArgValue::Str("A".to_string()),
        ArgValue::Str("BYPDZMGIKQCUSATREHOJNLFWXV<BFHNQUW".to_string()), // 3
        ArgValue::Bool(false),
        ArgValue::Str("A".to_string()),
        ArgValue::Str("A".to_string()),
        ArgValue::Str("ZANJCGDLVHIXOBRPMSWQUKFYET<BFHNQUW".to_string()), // 4
        ArgValue::Bool(false),
        ArgValue::Str("A".to_string()),
        ArgValue::Str("A".to_string()),
        ArgValue::Str("QXBGUTOVFCZPJIHSWERYNDAMLK<BFHNQUW".to_string()), // 5
        ArgValue::Bool(false),
        ArgValue::Str("A".to_string()),
        ArgValue::Str("A".to_string()),
        ArgValue::Str("AN BC FG IE KD LU MH OR TS VZ WQ XJ YP".to_string()), // Reflector
        ArgValue::Str("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string()),             // Plugboard
        ArgValue::Str("None".to_string()),                                   // Keyboard
        ArgValue::Bool(true),                                                // Strict
    ];
    let result = op.run(input, &args).unwrap();
    // Result should be 5 chars
    assert_eq!(result.len(), 5);
}
#[test]
fn test_typex_keyboard() {
    let op = Typex;
    let input = b"HELLO 123".to_vec();
    let mut args = vec![
        ArgValue::Str("MCYLPQUVRXGSAOWNBJEZDTFKHI<BFHNQUW".to_string()),
        ArgValue::Bool(false),
        ArgValue::Str("A".to_string()),
        ArgValue::Str("A".to_string()),
        ArgValue::Str("KHWENRCBISXJQGOFMAPVYZDLTU<BFHNQUW".to_string()),
        ArgValue::Bool(false),
        ArgValue::Str("A".to_string()),
        ArgValue::Str("A".to_string()),
        ArgValue::Str("BYPDZMGIKQCUSATREHOJNLFWXV<BFHNQUW".to_string()),
        ArgValue::Bool(false),
        ArgValue::Str("A".to_string()),
        ArgValue::Str("A".to_string()),
        ArgValue::Str("ZANJCGDLVHIXOBRPMSWQUKFYET<BFHNQUW".to_string()),
        ArgValue::Bool(false),
        ArgValue::Str("A".to_string()),
        ArgValue::Str("A".to_string()),
        ArgValue::Str("QXBGUTOVFCZPJIHSWERYNDAMLK<BFHNQUW".to_string()),
        ArgValue::Bool(false),
        ArgValue::Str("A".to_string()),
        ArgValue::Str("A".to_string()),
        ArgValue::Str("AN BC FG IE KD LU MH OR TS VZ WQ XJ YP".to_string()),
        ArgValue::Str("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string()),
        ArgValue::Str("Encrypt".to_string()),
        ArgValue::Bool(true),
    ];
    let encrypted = op.run(input, &args).unwrap();
    args[22] = ArgValue::Str("Decrypt".to_string());
    let decrypted = op.run(encrypted, &args).unwrap();
    assert_eq!(String::from_utf8_lossy(&decrypted), "HELLO 123");
}
