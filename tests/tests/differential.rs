//! CyberChef differential harness.
//!
//! ```text
//! tests/fixtures/differential/cases.json
//!            |
//!            +--> recorded CyberChef reference output
//!            |
//!            +--> rx-chef output for the same operation/input/args
//!                       |
//!                  normalize both
//!                       |
//!                    compare -> verdict
//! ```
//!
//! Verdicts are EXACT, DOCUMENTED_DIFFERENCE, MISMATCH, NOT_COMPARABLE and
//! UNVERIFIED. Reference values are recorded by hand from upstream CyberChef
//! and are never regenerated from rx-chef — a case that is merely "what we
//! currently produce" proves nothing about parity.
//!
//! When a case reports MISMATCH, do not edit the expected value. Establish
//! first whether rx-chef is wrong, CyberChef differs, the normalization is
//! wrong, or the difference is deliberate; only then change the fixture, and
//! record deliberate differences as `documented_difference` with a reason.
//!
//! Run only these tests:
//!   cargo test -p cyberchef-rust-tests --test differential

use rxchef::runtime;
use serde::Deserialize;
use std::{collections::BTreeMap, fmt, fs, path::PathBuf};

#[derive(Debug, Deserialize)]
struct CaseFile {
    schema_version: u32,
    cases: Vec<Case>,
}

#[derive(Debug, Deserialize)]
struct Case {
    operation: String,
    input: String,
    input_encoding: Encoding,
    #[serde(default)]
    args: Vec<String>,
    reference: String,
    reference_encoding: Encoding,
    expect: Expectation,
    /// Where the expected value comes from. A case without a checkable source
    /// is an assertion about upstream that nobody can verify, so the harness
    /// requires one.
    reference_source: String,
    #[serde(default)]
    divergence: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum Encoding {
    Text,
    Hex,
    Empty,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum Expectation {
    Exact,
    DocumentedDifference,
    NotComparable,
    Unverified,
}

/// The outcome of comparing one case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Verdict {
    Exact,
    DocumentedDifference,
    Mismatch,
    NotComparable,
    Unverified,
}

impl fmt::Display for Verdict {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Verdict::Exact => "EXACT",
            Verdict::DocumentedDifference => "DOCUMENTED_DIFFERENCE",
            Verdict::Mismatch => "MISMATCH",
            Verdict::NotComparable => "NOT_COMPARABLE",
            Verdict::Unverified => "UNVERIFIED",
        };
        formatter.write_str(text)
    }
}

impl Encoding {
    fn decode(self, value: &str) -> Vec<u8> {
        match self {
            Encoding::Empty => Vec::new(),
            Encoding::Text => value.as_bytes().to_vec(),
            Encoding::Hex => {
                hex::decode(value).unwrap_or_else(|error| panic!("invalid hex {value:?}: {error}"))
            }
        }
    }
}

/// Normalize output before comparison.
///
/// Only line endings are normalized: CRLF and a lone CR become LF, and a
/// single trailing newline is dropped. Nothing else is touched, because any
/// further normalization risks hiding a genuine behavioural difference.
fn normalize(bytes: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'\r' if bytes.get(index + 1) == Some(&b'\n') => {
                output.push(b'\n');
                index += 2;
            }
            b'\r' => {
                output.push(b'\n');
                index += 1;
            }
            byte => {
                output.push(byte);
                index += 1;
            }
        }
    }
    if output.last() == Some(&b'\n') {
        output.pop();
    }
    output
}

fn cases_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/differential/cases.json")
}

fn load_cases() -> CaseFile {
    let path = cases_path();
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()));
    serde_json::from_str(&text)
        .unwrap_or_else(|error| panic!("invalid {}: {error}", path.display()))
}

/// Run one case and report its verdict alongside the actual rx-chef output.
fn evaluate(case: &Case) -> (Verdict, String) {
    if case.expect == Expectation::NotComparable {
        return (
            Verdict::NotComparable,
            case.divergence.clone().unwrap_or_default(),
        );
    }
    if case.expect == Expectation::Unverified {
        return (Verdict::Unverified, String::new());
    }

    let input = case.input_encoding.decode(&case.input);
    let produced = match runtime::run_operation(&case.operation, input, &case.args) {
        Ok(output) => output,
        Err(error) => {
            return (
                Verdict::Mismatch,
                format!("rx-chef returned an error: {error}"),
            )
        }
    };

    let expected = case.reference_encoding.decode(&case.reference);
    let matches = normalize(&produced) == normalize(&expected);

    let rendered = String::from_utf8(produced.clone())
        .unwrap_or_else(|_| format!("hex:{}", hex::encode(&produced)));

    match (case.expect, matches) {
        (Expectation::Exact, true) => (Verdict::Exact, rendered),
        (Expectation::Exact, false) => (Verdict::Mismatch, rendered),
        // A documented difference records what CyberChef produces; rx-chef is
        // expected to agree with the recorded value where one is given.
        (Expectation::DocumentedDifference, true) => (Verdict::DocumentedDifference, rendered),
        (Expectation::DocumentedDifference, false) => (Verdict::Mismatch, rendered),
        _ => unreachable!("handled above"),
    }
}

#[test]
fn differential_fixture_is_well_formed() {
    let file = load_cases();
    assert_eq!(file.schema_version, 1, "unsupported fixture schema version");
    assert!(!file.cases.is_empty(), "the fixture must contain cases");

    for case in &file.cases {
        // Every case must name a registered operation, otherwise the harness
        // would silently skip coverage it claims to have.
        assert!(
            runtime::resolve_operation_name(&case.operation).is_some(),
            "case names an unregistered operation: {}",
            case.operation
        );
        assert!(
            !case.reference_source.trim().is_empty(),
            "{} has no reference_source; an expected value nobody can check \
             is not evidence",
            case.operation
        );
        // Deliberate differences and exclusions must carry a reason.
        if matches!(
            case.expect,
            Expectation::DocumentedDifference | Expectation::NotComparable
        ) {
            assert!(
                case.divergence
                    .as_deref()
                    .is_some_and(|reason| !reason.trim().is_empty()),
                "{} is marked {:?} without a divergence reason",
                case.operation,
                case.expect
            );
        }
    }
}

#[test]
fn non_deterministic_operations_are_excluded_from_comparison() {
    // Operations that declare side effects or non-determinism cannot be
    // compared byte for byte and must be marked not_comparable.
    let file = load_cases();
    for case in &file.cases {
        let info = runtime::operation_info(&case.operation).unwrap();
        let comparable = info.deterministic && info.side_effects.is_empty();
        if !comparable {
            assert_eq!(
                case.expect,
                Expectation::NotComparable,
                "{} is non-deterministic or has side effects, so it must be \
                 marked not_comparable rather than compared byte for byte",
                case.operation
            );
        }
    }
}

#[test]
fn normalization_only_touches_line_endings() {
    assert_eq!(normalize(b"a\r\nb"), b"a\nb");
    assert_eq!(normalize(b"a\rb"), b"a\nb");
    assert_eq!(normalize(b"a\nb"), b"a\nb");
    // A single trailing newline is dropped.
    assert_eq!(normalize(b"abc\n"), b"abc");
    assert_eq!(normalize(b"abc\r\n"), b"abc");
    // Interior whitespace and case are preserved.
    assert_eq!(normalize(b"  A b  "), b"  A b  ");
    // Binary content passes through untouched.
    assert_eq!(normalize(&[0x00, 0xff, 0x7f]), vec![0x00, 0xff, 0x7f]);
}

#[test]
fn cyberchef_differential_cases_match_their_recorded_verdict() {
    let file = load_cases();
    let mut tally: BTreeMap<Verdict, usize> = BTreeMap::new();
    let mut mismatches = Vec::new();

    for case in &file.cases {
        let (verdict, detail) = evaluate(case);
        *tally.entry(verdict).or_default() += 1;
        if verdict == Verdict::Mismatch {
            mismatches.push(format!(
                "{} args={:?}\n    expected: {:?}\n    produced: {}",
                case.operation, case.args, case.reference, detail
            ));
        }
    }

    println!("CyberChef differential summary");
    for (verdict, count) in &tally {
        println!("  {verdict:<22} {count}");
    }

    assert!(
        mismatches.is_empty(),
        "{} differential case(s) diverged from the recorded CyberChef reference.\n\
         Do not edit the expected value first: establish whether rx-chef is wrong, \
         CyberChef differs, or the normalization is wrong.\n\n{}",
        mismatches.len(),
        mismatches.join("\n\n")
    );
}
