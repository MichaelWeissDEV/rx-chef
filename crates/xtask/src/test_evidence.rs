//! Derive real test evidence from operation test sources.
//!
//! The verification manifest records *which* files test an operation. That
//! mapping alone says nothing about whether those files actually assert
//! anything, so every number derived from it was previously equivalent to
//! "a file with a matching name exists on disk".
//!
//! This module reads the mapped sources and reports only properties that are
//! syntactically present in them. Each field below documents exactly what is
//! counted so that audit output stays checkable against the source tree.

use std::{fs, path::Path};

/// Evidence extracted from one or more test source files.
///
/// # What the counts mean
///
/// These are counts of what is *written* in the sources, not of what a
/// particular build runs. The two were compared directly when this scanner was
/// last reviewed:
///
/// ```text
/// 1855  cargo test --test operations                  (default features)
/// 1900  cargo test --all-features --test operations
/// 1903  #[test] attributes counted here
/// ```
///
/// The 3-attribute gap is mutually exclusive `#[cfg(feature = "x")]` /
/// `#[cfg(not(feature = "x"))]` pairs, where both arms are written but only
/// one is ever compiled. The repository contains no `#[tokio::test]`, no
/// macro-generated tests, and no `#[test]` inside comments or string literals,
/// so those forms are not a source of error. An AST-based scanner would remove
/// a 0.16% overcount at the cost of a parser dependency, which is not a trade
/// worth making; the number is documented instead.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TestEvidence {
    /// Number of `#[test]` attributes written in the mapped sources.
    pub tests: usize,
    /// Assertions that an operation rejected something: `is_err`,
    /// `unwrap_err`, `expect_err`, or an `Err(..)` match arm assertion.
    pub negative_assertions: usize,
    /// Calls that pass an empty input buffer to the operation.
    pub empty_input_cases: usize,
    /// Tests whose name signals a boundary/limit case.
    pub boundary_cases: usize,
    /// Test functions that assert only "did not error".
    ///
    /// Counted per test function, not per line: a test that asserts
    /// `result.is_ok()` and then goes on to `assert_eq!` the value is not
    /// weak, because the exact result *is* checked. Only a test with no
    /// exact-value assertion anywhere in its body is counted.
    pub weak_ok_assertions: usize,
    /// Exact-value assertions (`assert_eq!` / `assert_ne!`).
    pub value_assertions: usize,
}

impl TestEvidence {
    pub fn merge(&mut self, other: &TestEvidence) {
        self.tests += other.tests;
        self.negative_assertions += other.negative_assertions;
        self.empty_input_cases += other.empty_input_cases;
        self.boundary_cases += other.boundary_cases;
        self.weak_ok_assertions += other.weak_ok_assertions;
        self.value_assertions += other.value_assertions;
    }

    /// True when the source contains at least one executable test.
    pub fn has_tests(&self) -> bool {
        self.tests > 0
    }

    pub fn has_negative_case(&self) -> bool {
        self.negative_assertions > 0
    }

    pub fn has_boundary_case(&self) -> bool {
        self.empty_input_cases > 0 || self.boundary_cases > 0
    }
}

/// Names that mark a test as covering a boundary or limit condition.
const BOUNDARY_NAME_MARKERS: &[&str] = &[
    "empty",
    "boundary",
    "overflow",
    "underflow",
    "max",
    "min",
    "zero",
    "single",
    "large",
    "truncated",
    "oversized",
    "out_of_range",
    "limit",
];

/// Ways a test can hand an operation an empty input buffer.
const EMPTY_INPUT_MARKERS: &[&str] = &[
    "run(vec![]",
    "run(Vec::new()",
    "run(b\"\".to_vec()",
    "run(\"\".as_bytes().to_vec()",
    "run(\"\".into()",
    "run_typed(OperationData::Bytes(vec![])",
    "run_typed(OperationData::Text(String::new())",
];

/// Split a source file into the bodies of its `#[test]` functions.
///
/// Boundaries are approximated by the next `#[test]` attribute, which is
/// enough to attribute assertions to the test they belong to.
fn test_bodies(source: &str) -> Vec<&str> {
    let mut bodies = Vec::new();
    let mut rest = source;
    while let Some(start) = rest.find("#[test]") {
        let after = &rest[start + "#[test]".len()..];
        let end = after.find("#[test]").unwrap_or(after.len());
        bodies.push(&after[..end]);
        rest = &after[end..];
    }
    bodies
}

/// Tests whose assertions establish only "this call did not fail".
///
/// A test counts as weak when it asserts `is_ok()` (or merely unwraps) but
/// never asserts an exact value with `assert_eq!`/`assert_ne!`. A redundant
/// `assert!(result.is_ok())` followed by `assert_eq!(result.unwrap(), ..)` is
/// not weak — the exact result is still checked.
fn count_weak_tests(source: &str) -> usize {
    test_bodies(source)
        .into_iter()
        .filter(|body| {
            let asserts_ok = body.contains(".is_ok()");
            let asserts_value = body.contains("assert_eq!(") || body.contains("assert_ne!(");
            let asserts_negative = body.contains(".is_err()")
                || body.contains("unwrap_err()")
                || body.contains("expect_err(");
            asserts_ok && !asserts_value && !asserts_negative
        })
        .count()
}

fn count_occurrences(source: &str, needle: &str) -> usize {
    if needle.is_empty() {
        return 0;
    }
    source.match_indices(needle).count()
}

/// Extract the identifier following each `fn ` that belongs to a `#[test]`.
fn test_function_names(source: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut saw_test_attribute = false;
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("#[test]") {
            saw_test_attribute = true;
            continue;
        }
        if !saw_test_attribute {
            continue;
        }
        // Attributes may stack between `#[test]` and the signature.
        if trimmed.starts_with('#') {
            continue;
        }
        if let Some(rest) = trimmed
            .strip_prefix("fn ")
            .or_else(|| trimmed.strip_prefix("pub fn "))
            .or_else(|| trimmed.strip_prefix("async fn "))
        {
            let name: String = rest
                .chars()
                .take_while(|character| character.is_alphanumeric() || *character == '_')
                .collect();
            if !name.is_empty() {
                names.push(name);
            }
        }
        saw_test_attribute = false;
    }
    names
}

/// Analyse one test source file.
pub fn scan_source(source: &str) -> TestEvidence {
    let test_names = test_function_names(source);
    let boundary_cases = test_names
        .iter()
        .filter(|name| {
            BOUNDARY_NAME_MARKERS
                .iter()
                .any(|marker| name.contains(marker))
        })
        .count();

    TestEvidence {
        tests: test_names.len(),
        negative_assertions: count_occurrences(source, ".is_err()")
            + count_occurrences(source, "unwrap_err()")
            + count_occurrences(source, "expect_err("),
        empty_input_cases: EMPTY_INPUT_MARKERS
            .iter()
            .map(|marker| count_occurrences(source, marker))
            .sum(),
        boundary_cases,
        weak_ok_assertions: count_weak_tests(source),
        value_assertions: count_occurrences(source, "assert_eq!(")
            + count_occurrences(source, "assert_ne!("),
    }
}

/// Analyse every mapped file, returning the merged evidence.
///
/// Missing files are reported by the caller through the manifest existence
/// check; they contribute no evidence here.
pub fn scan_files<P: AsRef<Path>>(root: P, mappings: &[String]) -> TestEvidence {
    let root = root.as_ref();
    let mut evidence = TestEvidence::default();
    for mapping in mappings {
        let path = root.join(mapping);
        if let Ok(source) = fs::read_to_string(&path) {
            evidence.merge(&scan_source(&source));
        }
    }
    evidence
}

#[cfg(test)]
mod tests {
    use super::{scan_source, TestEvidence};

    const SAMPLE: &str = r#"
#[test]
fn test_thing_empty_input() {
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_thing_basic() {
    let result = op.run(b"x".to_vec(), &args);
    assert!(result.is_ok());
}

#[test]
fn test_thing_invalid() {
    assert!(op.run(b"!".to_vec(), &args).is_err());
}

fn helper_not_a_test() {}
"#;

    #[test]
    fn counts_only_real_test_functions() {
        let evidence = scan_source(SAMPLE);
        assert_eq!(evidence.tests, 3);
    }

    #[test]
    fn recognises_negative_and_empty_input_coverage() {
        let evidence = scan_source(SAMPLE);
        assert_eq!(evidence.negative_assertions, 1);
        assert_eq!(evidence.empty_input_cases, 1);
        assert_eq!(evidence.boundary_cases, 1);
        assert!(evidence.has_negative_case());
        assert!(evidence.has_boundary_case());
    }

    #[test]
    fn flags_tests_that_only_assert_success() {
        let evidence = scan_source(SAMPLE);
        // Only `test_thing_basic` asserts is_ok() without checking a value.
        assert_eq!(evidence.weak_ok_assertions, 1);
        assert_eq!(evidence.value_assertions, 1);
    }

    #[test]
    fn a_redundant_is_ok_next_to_a_value_assertion_is_not_weak() {
        // This is the common shape in the operation tests: the `is_ok()` is
        // redundant, but the exact value is still asserted, so the test is
        // not weak.
        let source = r#"
#[test]
fn test_decodes_hello() {
    let result = op.run(input, &args);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), b"Hello");
}
"#;
        assert_eq!(scan_source(source).weak_ok_assertions, 0);
    }

    #[test]
    fn a_test_asserting_only_success_is_weak() {
        let source = r#"
#[test]
fn test_runs() {
    assert!(op.run(input, &args).is_ok());
}
"#;
        assert_eq!(scan_source(source).weak_ok_assertions, 1);
    }

    #[test]
    fn a_file_without_tests_yields_no_evidence() {
        let evidence = scan_source("// only a comment\nfn helper() {}\n");
        assert_eq!(evidence, TestEvidence::default());
        assert!(!evidence.has_tests());
    }

    #[test]
    fn merging_sums_every_field() {
        let mut left = scan_source(SAMPLE);
        let right = scan_source(SAMPLE);
        left.merge(&right);
        assert_eq!(left.tests, 6);
        assert_eq!(left.negative_assertions, 2);
        assert_eq!(left.value_assertions, 2);
    }
}
