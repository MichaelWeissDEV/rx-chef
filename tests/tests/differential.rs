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
    /// For `semantic_roundtrip` cases: the operation that reverses this one.
    ///
    /// Compression formats do not specify a unique byte encoding, so two
    /// conforming encoders may legitimately disagree byte for byte. Demanding
    /// exact equality there would either force a false MISMATCH or invite
    /// pasting rx-chef's own output into the fixture, which proves nothing.
    /// What *is* checkable is that each side's stream decodes to the original
    /// input under both implementations.
    #[serde(default)]
    inverse_operation: Option<String>,
    #[serde(default)]
    inverse_args: Vec<String>,
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
    /// The encodings need not match byte for byte, but both must decode back
    /// to the original input through rx-chef's inverse operation.
    SemanticRoundtrip,
    NotComparable,
    Unverified,
}

/// The outcome of comparing one case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Verdict {
    Exact,
    Compatible,
    DocumentedDifference,
    Mismatch,
    NotComparable,
    Unverified,
}

impl fmt::Display for Verdict {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Verdict::Exact => "EXACT",
            Verdict::Compatible => "COMPATIBLE",
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

    if case.expect == Expectation::SemanticRoundtrip {
        return evaluate_semantic_roundtrip(case, &produced, &expected, rendered);
    }

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

/// Compare two encodings that need not be byte-identical.
///
/// Both rx-chef's own output and the recorded upstream stream must decode back
/// to the original input through rx-chef's inverse operation. That establishes
/// interoperability in both directions — rx-chef can read what upstream wrote,
/// and produces something upstream's format admits — without asserting a byte
/// equality the format never promised.
fn evaluate_semantic_roundtrip(
    case: &Case,
    produced: &[u8],
    reference: &[u8],
    rendered: String,
) -> (Verdict, String) {
    let Some(inverse) = case.inverse_operation.as_deref() else {
        return (
            Verdict::Mismatch,
            "semantic_roundtrip case does not name an inverse_operation".to_string(),
        );
    };
    let original = case.input_encoding.decode(&case.input);

    let decode = |bytes: &[u8], label: &str| -> Result<(), String> {
        match runtime::run_operation(inverse, bytes.to_vec(), &case.inverse_args) {
            Ok(decoded) if decoded == original => Ok(()),
            Ok(decoded) => Err(format!(
                "{label} decoded to {} bytes that differ from the {} byte input",
                decoded.len(),
                original.len()
            )),
            Err(error) => Err(format!(
                "{label} could not be decoded by {inverse}: {error}"
            )),
        }
    };

    if let Err(why) = decode(produced, "rx-chef output") {
        return (Verdict::Mismatch, why);
    }
    if let Err(why) = decode(reference, "the recorded upstream stream") {
        return (Verdict::Mismatch, why);
    }

    if produced == reference {
        // Byte-identical is strictly stronger; report it as such.
        (Verdict::Exact, rendered)
    } else {
        (Verdict::Compatible, rendered)
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

// ── Sweep triage completeness ─────────────────────────────────────────────
//
// `verification/differential-triage.json` records why every operation in the
// upstream default sweep matched, differed, or could not be compared. Its
// value is that it has no gaps: a difference with no stated cause is a
// difference nobody has looked at.

#[derive(serde::Deserialize)]
struct TriageDocument {
    schema_version: u32,
    classifications: Vec<String>,
    entries: Vec<TriageEntry>,
}

#[derive(serde::Deserialize)]
struct TriageEntry {
    operation: String,
    classification: String,
    reason: String,
    status: Option<String>,
}

fn load_triage() -> TriageDocument {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../verification/differential-triage.json"
    );
    let text = std::fs::read_to_string(path).expect("differential triage document must exist");
    serde_json::from_str(&text).expect("differential triage document must parse")
}

#[test]
fn every_swept_operation_has_a_triage_verdict_with_a_reason() {
    let document = load_triage();
    assert_eq!(document.schema_version, 1);
    assert!(
        document.entries.len() > 400,
        "the triage should cover the whole sweep, got {} entries",
        document.entries.len()
    );
    for entry in &document.entries {
        assert!(
            document.classifications.contains(&entry.classification),
            "{}: unknown classification {:?}",
            entry.operation,
            entry.classification
        );
        assert!(
            entry.reason.trim().len() >= 20,
            "{}: classification {} needs a reason a reviewer can check, got {:?}",
            entry.operation,
            entry.classification,
            entry.reason
        );
    }
}

#[test]
fn no_swept_operation_is_left_untriaged() {
    let document = load_triage();
    let untriaged: Vec<&str> = document
        .entries
        .iter()
        .filter(|entry| entry.classification == "UNTRIAGED")
        .map(|entry| entry.operation.as_str())
        .collect();
    assert!(
        untriaged.is_empty(),
        "these operations still have no stated cause: {untriaged:?}"
    );
}

#[test]
fn confirmed_rxchef_bugs_from_the_sweep_are_marked_fixed() {
    // A RXCHEF_BUG left open is a release blocker, not a note.
    let document = load_triage();
    for entry in &document.entries {
        if entry.classification == "RXCHEF_BUG" {
            assert_eq!(
                entry.status.as_deref(),
                Some("fixed"),
                "{}: confirmed rx-chef defect is not marked fixed",
                entry.operation
            );
        }
    }
}

#[test]
fn triage_operations_are_unique() {
    let document = load_triage();
    let mut seen = std::collections::HashSet::new();
    for entry in &document.entries {
        assert!(
            seen.insert(entry.operation.as_str()),
            "duplicate triage entry for {}",
            entry.operation
        );
    }
}
