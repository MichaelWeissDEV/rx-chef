// Tests for the yara_rules operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations yara_rules::

use rxchef::operation::ArgValue;
use rxchef::operations::yara_rules::YARARules;
use rxchef::Operation;

#[test]
fn test_yara_rules_simple_match() {
    let op = YARARules;
    let input = b"Hello CyberChef World!".to_vec();
    let rules = r#"
        rule TestRule {
            strings:
                $a = "CyberChef"
            condition:
                $a
        }
    "#;
    let args = [
        ArgValue::Str(rules.to_string()),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8_lossy(&result);
    assert!(result_str.contains("Input matches rule \"TestRule\" (1 time)."));
}
#[test]
fn test_yara_rules_show_strings() {
    let op = YARARules;
    let input = b"ABC 123 ABC".to_vec();
    let rules = r#"
        rule MultiMatch {
            strings:
                $a = "ABC"
            condition:
                $a
        }
    "#;
    let args = [
        ArgValue::Str(rules.to_string()),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8_lossy(&result);
    assert!(result_str.contains("Rule \"MultiMatch\" matches (2 times):"));
    assert!(result_str.contains("Pos 0, length 3, identifier $a"));
    assert!(result_str.contains("Pos 8, length 3, identifier $a"));
}
#[test]
fn test_yara_rules_invalid_rule() {
    let op = YARARules;
    let input = b"test".to_vec();
    let rules = "invalid rule syntax";
    let args = [
        ArgValue::Str(rules.to_string()),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
