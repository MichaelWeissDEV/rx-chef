// Tests for the levenshtein_distance operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations levenshtein_distance::

use rxchef::operation::ArgValue;
use rxchef::operations::levenshtein_distance::LevenshteinDistance;
use rxchef::Operation;

fn run_op(input: &str, delim: &str, ins: f64, del: f64, sub: f64) -> String {
    let op = LevenshteinDistance;
    let args = [
        ArgValue::Str(delim.to_string()),
        ArgValue::Num(ins),
        ArgValue::Num(del),
        ArgValue::Num(sub),
    ];
    let result = op.run(input.as_bytes().to_vec(), &args).unwrap();
    String::from_utf8(result).unwrap()
}
#[test]
fn test_levenshtein_wikipedia_1() {
    // kitten -> sitting = 3
    assert_eq!(run_op("kitten\nsitting", "\n", 1.0, 1.0, 1.0), "3");
}
#[test]
fn test_levenshtein_wikipedia_2() {
    // saturday -> sunday = 3
    assert_eq!(run_op("saturday\nsunday", "\n", 1.0, 1.0, 1.0), "3");
}
#[test]
fn test_levenshtein_sub_cost_2() {
    // kitten -> sitting with sub cost 2 = 5
    assert_eq!(run_op("kitten\nsitting", "\n", 1.0, 1.0, 2.0), "5");
}
#[test]
fn test_levenshtein_varied_costs_1() {
    // kitten -> sitting with ins=10, del=100, sub=1000 = 230
    assert_eq!(run_op("kitten\nsitting", "\n", 10.0, 100.0, 1000.0), "230");
}
#[test]
fn test_levenshtein_varied_costs_2() {
    // kitten -> sitting with ins=1000, del=100, sub=10 = 1020
    assert_eq!(run_op("kitten\nsitting", "\n", 1000.0, 100.0, 10.0), "1020");
}
#[test]
fn test_levenshtein_space_delimiter() {
    // kitten sitting split by " "
    assert_eq!(run_op("kitten sitting", " ", 1.0, 1.0, 1.0), "3");
}
#[test]
fn test_levenshtein_zero_costs() {
    assert_eq!(run_op("kitten\nsitting", "\n", 0.0, 0.0, 0.0), "0");
}
#[test]
fn test_levenshtein_too_few_samples() {
    let op = LevenshteinDistance;
    let args = [
        ArgValue::Str("\n".to_string()),
        ArgValue::Num(1.0),
        ArgValue::Num(1.0),
        ArgValue::Num(1.0),
    ];
    let result = op.run(b"kitten".to_vec(), &args);
    assert!(result.is_err());
}
#[test]
fn test_levenshtein_negative_cost() {
    let op = LevenshteinDistance;
    let args = [
        ArgValue::Str("\n".to_string()),
        ArgValue::Num(-1.0),
        ArgValue::Num(1.0),
        ArgValue::Num(1.0),
    ];
    let result = op.run(b"kitten\nsitting".to_vec(), &args);
    assert!(result.is_err());
}
