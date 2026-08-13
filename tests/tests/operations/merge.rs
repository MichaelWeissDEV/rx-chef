// Tests for the merge operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations merge::

use rxchef::operation::ArgValue;
use rxchef::operations::merge::Merge;
use rxchef::Operation;

#[test]
fn test_merge_passes_input_through_unchanged() {
    let op = Merge;
    let args = [ArgValue::Bool(true)];
    let result = op.run(b"hello world".to_vec(), &args).unwrap();
    assert_eq!(result, b"hello world".to_vec());
}

#[test]
fn test_merge_all_false_still_passes_through() {
    // The actual branch-consolidation behaviour lives in the pipeline/fork
    // handling; in isolation Merge::run is always a no-op regardless of the
    // "Merge All" flag.
    let op = Merge;
    let args = [ArgValue::Bool(false)];
    let result = op.run(b"branch data".to_vec(), &args).unwrap();
    assert_eq!(result, b"branch data".to_vec());
}

#[test]
fn test_merge_empty_input() {
    let op = Merge;
    let args = [ArgValue::Bool(true)];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_merge_no_args_still_works() {
    // args_schema has a default, and run() ignores args entirely, so an empty
    // args slice must behave identically.
    let op = Merge;
    let result = op.run(b"data".to_vec(), &[]).unwrap();
    assert_eq!(result, b"data".to_vec());
}

#[test]
fn test_merge_binary_data_roundtrips() {
    let op = Merge;
    let input = vec![0u8, 159, 255, 1, 2, 3];
    let result = op.run(input.clone(), &[]).unwrap();
    assert_eq!(result, input);
}
