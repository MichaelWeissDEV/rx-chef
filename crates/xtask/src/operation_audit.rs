use crate::test_evidence::{self, TestEvidence};
use rxchef::{
    operation::{ArgKind, Availability, NumericBound, ParityStatus},
    runtime::{self, data_type_name},
};
use serde_json::{json, Value};
use std::{
    collections::{BTreeSet, HashSet},
    env, fs,
    path::PathBuf,
};

/// Aggregate counters behind the audit summary.
///
/// Every field is incremented from evidence read out of the repository, so the
/// printed summary can be re-derived from `docs/_generated/operation-quality.json`.
#[derive(Debug, Default)]
struct AuditTotals {
    registered: usize,
    tested: usize,
    total_tests: usize,
    negative_tested: usize,
    boundary_tested: usize,
    reference_verified: usize,
    differential_verified: usize,
    parity_exact: usize,
    parity_compatible: usize,
    parity_documented_difference: usize,
    parity_unknown: usize,
    parity_not_applicable: usize,
    weak_ok_assertions: usize,
    known_limitations: usize,
    correctness_verified: usize,
    correctness_partial: usize,
    correctness_unverified: usize,
    negative_not_applicable: usize,
    with_typed_provenance: usize,
    independent_evidence_operations: usize,
    parity_exact_missing_independent_evidence: usize,
}

impl AuditTotals {
    fn record(
        &mut self,
        evidence: &TestEvidence,
        fixture: FixtureEvidence,
        negative_policy: &NegativeTestPolicy,
        info: &runtime::OperationInfo,
        has_known_answer: bool,
        has_differential: bool,
        provenance: &[ProvenanceRecord],
    ) {
        self.registered += 1;
        if !provenance.is_empty() {
            self.with_typed_provenance += 1;
        }
        let has_independent_evidence = provenance.iter().any(ProvenanceRecord::is_independent);
        if has_independent_evidence {
            self.independent_evidence_operations += 1;
        }
        if info.parity == ParityStatus::Exact && !has_independent_evidence {
            self.parity_exact_missing_independent_evidence += 1;
        }
        match Correctness::derive(
            evidence,
            fixture,
            negative_policy,
            has_known_answer,
            has_differential,
        ) {
            Correctness::Verified => self.correctness_verified += 1,
            Correctness::PartiallyVerified => self.correctness_partial += 1,
            Correctness::Unverified => self.correctness_unverified += 1,
        }
        self.total_tests += evidence.tests;
        self.weak_ok_assertions += evidence.weak_ok_assertions;
        if evidence.has_tests() || fixture.any {
            self.tested += 1;
        }
        if evidence.has_negative_case() || fixture.rejection {
            self.negative_tested += 1;
        }
        if negative_policy.is_not_applicable() {
            self.negative_not_applicable += 1;
        }
        if evidence.has_boundary_case() || fixture.empty_input || fixture.binary_or_unicode {
            self.boundary_tested += 1;
        }
        if has_known_answer {
            self.reference_verified += 1;
        }
        if has_differential {
            self.differential_verified += 1;
        }
        if !info.known_limitations.is_empty() {
            self.known_limitations += 1;
        }
        match info.parity {
            ParityStatus::Exact => self.parity_exact += 1,
            ParityStatus::Compatible => self.parity_compatible += 1,
            ParityStatus::IntentionalDifference => self.parity_documented_difference += 1,
            ParityStatus::Unknown => self.parity_unknown += 1,
            ParityStatus::NotApplicable => self.parity_not_applicable += 1,
        }
    }

    fn print_summary(&self) {
        println!("operation audit summary");
        println!("  {:<28} {}", "registered", self.registered);
        println!(
            "  {:<28} {}",
            "correctness: verified", self.correctness_verified
        );
        println!(
            "  {:<28} {}",
            "correctness: partial", self.correctness_partial
        );
        println!(
            "  {:<28} {}",
            "correctness: unverified", self.correctness_unverified
        );
        println!("  {:<28} {}", "with executable tests", self.tested);
        println!(
            "  {:<28} {} (attributes in source; a single build runs slightly fewer, see TestEvidence)",
            "test functions total", self.total_tests
        );
        println!("  {:<28} {}", "with negative tests", self.negative_tested);
        println!(
            "  {:<28} {}",
            "negative N/A (justified)", self.negative_not_applicable
        );
        println!("  {:<28} {}", "with boundary tests", self.boundary_tested);
        println!(
            "  {:<28} {}",
            "reference-verified (KAT)", self.reference_verified
        );
        println!(
            "  {:<28} {}",
            "differential-verified", self.differential_verified
        );
        println!("  {:<28} {}", "parity exact", self.parity_exact);
        println!("  {:<28} {}", "parity compatible", self.parity_compatible);
        println!(
            "  {:<28} {}",
            "parity documented difference", self.parity_documented_difference
        );
        println!("  {:<28} {}", "parity unverified", self.parity_unknown);
        println!(
            "  {:<28} {}",
            "parity not applicable", self.parity_not_applicable
        );
        println!(
            "  {:<28} {}",
            "documented divergences", self.known_limitations
        );
        println!(
            "  {:<28} {}",
            "tests asserting only success", self.weak_ok_assertions
        );
        println!(
            "  {:<28} {}",
            "with typed provenance", self.with_typed_provenance
        );
        println!(
            "  {:<28} {}",
            "independent evidence (typed)", self.independent_evidence_operations
        );
        if self.parity_exact_missing_independent_evidence > 0 {
            println!(
                "  {:<28} {}  (warning: exact parity without a typed independent-evidence record)",
                "parity exact, untyped", self.parity_exact_missing_independent_evidence
            );
        }
        println!();
        println!("operations audit progress");
        println!("  TOTAL:        {}", self.registered);
        println!(
            "  VERIFIED:     {}  ({:.0}%)",
            self.correctness_verified,
            100.0 * self.correctness_verified as f64 / self.registered.max(1) as f64
        );
        println!("  PARTIAL:      {}", self.correctness_partial);
        println!("  UNVERIFIED:   {}", self.correctness_unverified);
        println!(
            "  gaps: {} need negative tests, {} need boundary tests, {} need known-answer or differential evidence, {} need typed independent evidence",
            self.registered - self.negative_tested,
            self.registered - self.boundary_tested,
            self.registered - self.reference_verified.max(self.differential_verified),
            self.registered - self.independent_evidence_operations
        );
    }

    fn as_json(&self) -> Value {
        json!({
            "registered": self.registered,
            "correctness_verified": self.correctness_verified,
            "correctness_partially_verified": self.correctness_partial,
            "correctness_unverified": self.correctness_unverified,
            "tested": self.tested,
            "test_functions": self.total_tests,
            "negative_tested": self.negative_tested,
            "negative_not_applicable": self.negative_not_applicable,
            "boundary_tested": self.boundary_tested,
            "reference_verified": self.reference_verified,
            "differential_verified": self.differential_verified,
            "parity_exact": self.parity_exact,
            "parity_compatible": self.parity_compatible,
            "parity_documented_difference": self.parity_documented_difference,
            "parity_unverified": self.parity_unknown,
            "parity_not_applicable": self.parity_not_applicable,
            "documented_divergences": self.known_limitations,
            "tests_asserting_only_success": self.weak_ok_assertions,
            "with_typed_provenance": self.with_typed_provenance,
            "independent_evidence_operations": self.independent_evidence_operations,
            "parity_exact_missing_independent_evidence": self.parity_exact_missing_independent_evidence,
        })
    }
}

/// Whether a negative test is meaningful for this operation.
///
/// Some operations have no semantically invalid input: they accept arbitrary
/// bytes and every input has a defined output. Demanding a negative test there
/// would keep them `partially_verified` forever because of a weakness in the
/// audit model rather than a weakness in the operation.
///
/// `NotApplicable` is therefore available — but it is never inferred. It must
/// be declared per operation in `verification/operations.json` together with a
/// justification a reviewer can check:
///
/// ```json
/// "negative_test_policy": {
///     "not_applicable": "Accepts arbitrary bytes; every input has a defined
///                        output and the operation declares no arguments."
/// }
/// ```
///
/// "we could not find a failure case", "the test would be laborious" and
/// "upstream does not error either" are not justifications. An operation with
/// arguments can still reject an invalid *argument* even when it accepts any
/// input, and that case stays `Required`.
#[derive(Debug, Clone, PartialEq, Eq)]
enum NegativeTestPolicy {
    Required,
    NotApplicable { justification: String },
}

impl NegativeTestPolicy {
    /// Read the declared policy, defaulting to `Required`.
    fn read(verification: &Value, operation: &str, errors: &mut Vec<String>) -> Self {
        let Some(declared) = verification.get("negative_test_policy") else {
            return NegativeTestPolicy::Required;
        };
        if declared == "required" {
            return NegativeTestPolicy::Required;
        }
        let justification = declared
            .get("not_applicable")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .trim()
            .to_string();
        if justification.is_empty() {
            errors.push(format!(
                "{operation}: negative_test_policy must be \"required\" or \
                 {{\"not_applicable\": \"<justification>\"}}"
            ));
            return NegativeTestPolicy::Required;
        }
        // A justification short enough to be a shrug is not a justification.
        if justification.len() < 40 {
            errors.push(format!(
                "{operation}: negative_test_policy justification is too thin to review: {justification:?}"
            ));
            return NegativeTestPolicy::Required;
        }
        for excuse in [
            "could not find",
            "no failure case",
            "would be laborious",
            "too much work",
            "cyberchef does not error",
            "upstream does not error",
        ] {
            if justification.to_ascii_lowercase().contains(excuse) {
                errors.push(format!(
                    "{operation}: negative_test_policy justification is an excuse, not a reason: {justification:?}"
                ));
                return NegativeTestPolicy::Required;
            }
        }
        NegativeTestPolicy::NotApplicable { justification }
    }

    fn is_not_applicable(&self) -> bool {
        matches!(self, NegativeTestPolicy::NotApplicable { .. })
    }

    fn as_json(&self) -> Value {
        match self {
            NegativeTestPolicy::Required => json!("required"),
            NegativeTestPolicy::NotApplicable { justification } => {
                json!({ "not_applicable": justification })
            }
        }
    }
}

/// The kind of source behind a piece of verification evidence.
///
/// Whether an operation is "verified" (it has executable tests asserting
/// exact values) and whether that verification is backed by *independent*
/// evidence are different claims. A known-answer test can assert an exact
/// value that was never checked against anything outside this repository —
/// e.g. hardcoded by running this crate's own implementation once and
/// pasting the result. That is real regression coverage, but it cannot prove
/// the value is *correct*, only that it hasn't changed.
///
/// This enum records which kind of source a given piece of evidence actually
/// traces back to, so the two claims can be reported separately instead of
/// conflated. See `INDEPENDENT_PROVENANCE_TYPES` for which of these variants
/// count as independent evidence.
const PROVENANCE_TYPES: &[&str] = &[
    "rfc",
    "nist",
    "fips",
    "standard",
    "published_test_vector",
    "upstream_fixture",
    "cyberchef_differential",
    "independent_implementation",
    "mathematical_invariant",
    "property_test",
    "internal_regression",
    "roundtrip",
    "self_generated_expected_value",
];

/// Provenance types that may, on their own, satisfy an "independent
/// evidence" claim: the expected value was established by something other
/// than running this crate's own implementation and recording the output.
///
/// `internal_regression`, `roundtrip`, `self_generated_expected_value`,
/// `property_test`, and `mathematical_invariant` are deliberately excluded:
/// each is valuable regression coverage, but none of them checks the value
/// against anything outside this repository, so none of them alone can
/// answer "is this correct" rather than "did this change".
const INDEPENDENT_PROVENANCE_TYPES: &[&str] = &[
    "rfc",
    "nist",
    "fips",
    "standard",
    "published_test_vector",
    "upstream_fixture",
    "cyberchef_differential",
    "independent_implementation",
];

/// Evidence buckets a provenance record may document. Kept separate from the
/// manifest's raw field names so a typo in `target` is caught rather than
/// silently treated as "documents nothing".
const PROVENANCE_TARGETS: &[&str] = &["correctness", "known_answer", "differential", "property"];

/// A single sourced claim about where an evidence bucket's expected values
/// actually came from.
///
/// Read from the optional `evidence_provenance` array in
/// `verification/operations.json`:
///
/// ```json
/// "evidence_provenance": [
///   {
///     "target": "differential",
///     "type": "cyberchef_differential",
///     "source": "gchq/CyberChef",
///     "commit": "b92501ee354256a127479f93d4c31a4f1d0dd657",
///     "path_in_source": "src/core/vendor/gost/gostCipher.mjs",
///     "notes": "wrapKeyGOST/wrapKeyCP invoked directly under Node to
///               generate reference vectors."
///   }
/// ]
/// ```
///
/// This is additive and optional: an operation with no `evidence_provenance`
/// entries is not an error. It means its evidence's origin has not been
/// classified yet — which must be visible as "not yet classified", not
/// silently counted as either independent or asserted to be self-generated.
/// Nothing here is invented; an operation stays unclassified until someone
/// checks its actual source.
#[derive(Debug, Clone)]
struct ProvenanceRecord {
    target: String,
    kind: String,
}

impl ProvenanceRecord {
    fn is_independent(&self) -> bool {
        INDEPENDENT_PROVENANCE_TYPES.contains(&self.kind.as_str())
    }
}

/// Reads and validates `verification[operation]["evidence_provenance"]`.
///
/// `present_targets` is the set of evidence buckets (`correctness`,
/// `known_answer`, `differential`, `property`) that are non-empty for this
/// operation; a provenance record targeting an empty or unknown bucket is a
/// dangling claim and is rejected rather than silently ignored.
fn read_evidence_provenance(
    verification: &Value,
    operation: &str,
    present_targets: &BTreeSet<&'static str>,
    errors: &mut Vec<String>,
) -> Vec<ProvenanceRecord> {
    let Some(entries) = verification.get("evidence_provenance") else {
        return Vec::new();
    };
    let Some(entries) = entries.as_array() else {
        errors.push(format!("{operation}: evidence_provenance must be an array"));
        return Vec::new();
    };
    let mut records = Vec::with_capacity(entries.len());
    for entry in entries {
        let target = entry["target"].as_str().unwrap_or_default().to_string();
        let kind = entry["type"].as_str().unwrap_or_default().to_string();
        let source = entry["source"].as_str().unwrap_or_default().trim();
        let commit = entry["commit"].as_str().unwrap_or_default().trim();
        let version = entry["version"].as_str().unwrap_or_default().trim();

        if !PROVENANCE_TARGETS.contains(&target.as_str()) {
            errors.push(format!(
                "{operation}: evidence_provenance target must be one of {PROVENANCE_TARGETS:?}, got {target:?}"
            ));
            continue;
        }
        if !present_targets.contains(target.as_str()) {
            errors.push(format!(
                "{operation}: evidence_provenance targets '{target}', which has no evidence entries for this operation"
            ));
            continue;
        }
        if !PROVENANCE_TYPES.contains(&kind.as_str()) {
            errors.push(format!(
                "{operation}: evidence_provenance type must be one of {PROVENANCE_TYPES:?}, got {kind:?}"
            ));
            continue;
        }
        if matches!(
            kind.as_str(),
            "rfc" | "nist" | "fips" | "standard" | "published_test_vector"
        ) && source.is_empty()
        {
            errors.push(format!(
                "{operation}: evidence_provenance type '{kind}' requires a non-empty 'source'"
            ));
            continue;
        }
        if matches!(
            kind.as_str(),
            "cyberchef_differential" | "upstream_fixture" | "independent_implementation"
        ) && commit.is_empty()
            && version.is_empty()
        {
            errors.push(format!(
                "{operation}: evidence_provenance type '{kind}' requires a 'commit' or 'version' identifying exactly what was checked against"
            ));
            continue;
        }
        records.push(ProvenanceRecord { target, kind });
    }
    records
}

/// Per-operation evidence held in the differential fixture.
///
/// The fixture is executed by `tests/tests/differential.rs`, so a case in it is
/// a test that ran — it simply lives in one data file rather than in 478
/// hand-written ones. Not reading it would have understated coverage and
/// pushed toward duplicating the same assertions per operation.
#[derive(Debug, Default, Clone, Copy)]
struct FixtureEvidence {
    /// A case feeding this operation an empty input.
    empty_input: bool,
    /// A case asserting this operation rejects an input upstream also rejects.
    rejection: bool,
    /// A case feeding this operation non-UTF-8 or multi-byte bytes.
    binary_or_unicode: bool,
    /// Any case at all.
    any: bool,
}

/// Index the differential fixture by operation name.
fn fixture_evidence(
    root: &std::path::Path,
) -> Result<std::collections::HashMap<String, FixtureEvidence>, String> {
    let path = root.join("tests/fixtures/differential/cases.json");
    let text = fs::read_to_string(&path)
        .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
    let document: Value = serde_json::from_str(&text)
        .map_err(|error| format!("invalid {}: {error}", path.display()))?;
    let mut index: std::collections::HashMap<String, FixtureEvidence> =
        std::collections::HashMap::new();
    for case in document["cases"].as_array().into_iter().flatten() {
        let Some(name) = case["operation"].as_str() else {
            continue;
        };
        let entry = index.entry(name.to_string()).or_default();
        entry.any = true;
        let encoding = case["input_encoding"].as_str().unwrap_or("");
        let input = case["input"].as_str().unwrap_or("");
        if encoding == "empty" || (encoding == "text" && input.is_empty()) {
            entry.empty_input = true;
        }
        if case["expect"] == "rejected" {
            entry.rejection = true;
        }
        if encoding == "hex" || !input.is_ascii() {
            entry.binary_or_unicode = true;
        }
    }
    Ok(index)
}

/// The correctness dimension, **derived** from evidence rather than declared.
///
/// Keeping this out of the operation trait is deliberate. `implementation_status`
/// is a hand-written trait method that defaulted to `Partial` for all 478
/// operations because nobody ever set it, and 473 of them carried a benchmark
/// skip reason reading "operation remains Partial until performance evidence is
/// reviewed" — conflating how fast an operation is with whether it is correct.
/// A verdict that can be raised by editing a line is not evidence, so this one
/// can only be raised by adding tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Correctness {
    /// Executable tests covering normal, negative and boundary input, plus at
    /// least one independent reference (known-answer or differential).
    Verified,
    /// Executable tests, but at least one of those dimensions is missing.
    PartiallyVerified,
    /// No executable tests at all.
    Unverified,
}

impl Correctness {
    fn derive(
        evidence: &TestEvidence,
        fixture: FixtureEvidence,
        negative_policy: &NegativeTestPolicy,
        has_known_answer: bool,
        has_differential: bool,
    ) -> Self {
        // A fixture case is an executed test; it just lives in a data file
        // rather than in a per-operation source file.
        if !evidence.has_tests() && !fixture.any {
            return Correctness::Unverified;
        }
        let independent = has_known_answer || has_differential;
        let negative = evidence.has_negative_case()
            || fixture.rejection
            || negative_policy.is_not_applicable();
        let boundary =
            evidence.has_boundary_case() || fixture.empty_input || fixture.binary_or_unicode;
        if negative && boundary && independent {
            Correctness::Verified
        } else {
            Correctness::PartiallyVerified
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Correctness::Verified => "verified",
            Correctness::PartiallyVerified => "partially_verified",
            Correctness::Unverified => "unverified",
        }
    }

    /// What this operation still needs before it can reach `verified`.
    fn missing(
        evidence: &TestEvidence,
        fixture: FixtureEvidence,
        negative_policy: &NegativeTestPolicy,
        has_known_answer: bool,
        has_differential: bool,
    ) -> Vec<&'static str> {
        let mut gaps = Vec::new();
        if !evidence.has_tests() && !fixture.any {
            gaps.push("executable tests");
            return gaps;
        }
        if !evidence.has_negative_case()
            && !fixture.rejection
            && !negative_policy.is_not_applicable()
        {
            gaps.push("negative tests");
        }
        if !evidence.has_boundary_case() && !fixture.empty_input && !fixture.binary_or_unicode {
            gaps.push("boundary tests");
        }
        if !has_known_answer && !has_differential {
            gaps.push("independent reference evidence");
        }
        gaps
    }
}

const REPRESENTATIVE_BENCHMARKS: &[&str] = &[
    "to_hex",
    "to_base64",
    "sha2",
    "aes_encrypt",
    "gzip_compress",
    "magic",
    "scan",
];

/// Add missing operations to the verification inventory.
///
/// New entries are deliberately conservative: they map the operation to its
/// dedicated test module, record only the representative benchmark catalog,
/// and make no KAT, differential, property, or fuzz claim. Reviewers add those
/// manually.
///
/// Entries that already exist are left untouched. This used to rewrite the
/// whole file from scratch, which silently discarded every reviewed
/// `untested_reason` and every recorded evidence path — the audit would then
/// fail with dozens of errors and the reasoning behind them would be gone.
pub fn generate_manifest() -> Result<(), String> {
    let root = workspace_root()?;
    let destination = root.join("verification/operations.json");

    let mut operations = match fs::read_to_string(&destination) {
        Ok(text) => {
            let existing: Value = serde_json::from_str(&text)
                .map_err(|error| format!("cannot merge into {}: {error}", destination.display()))?;
            existing["operations"]
                .as_object()
                .cloned()
                .ok_or_else(|| "verification manifest operations must be an object".to_string())?
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => serde_json::Map::new(),
        Err(error) => return Err(format!("cannot read {}: {error}", destination.display())),
    };
    let preserved = operations.len();

    for name in runtime::operation_names(None) {
        let info = runtime::operation_info(&name)?;
        let source_id = runtime::operation_source(&name)?;
        let benchmark = REPRESENTATIVE_BENCHMARKS.contains(&info.id.as_str());
        if operations.contains_key(&info.id) {
            continue;
        }
        operations.insert(
            info.id,
            json!({
                "correctness": [format!("tests/tests/operations/{source_id}.rs")],
                "known_answer": [],
                "differential": [],
                "property": [],
                "fuzz": [],
                "benchmark": if benchmark { vec!["representative release benchmark"] } else { Vec::<&str>::new() },
                "benchmark_skip_reason": if benchmark { Value::Null } else { json!("No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.") },
            }),
        );
    }
    let added = operations.len() - preserved;
    fs::create_dir_all(destination.parent().unwrap()).map_err(|e| e.to_string())?;
    let document = json!({"schema_version": 1, "operations": operations});
    fs::write(
        &destination,
        format!(
            "{}\n",
            serde_json::to_string_pretty(&document).map_err(|e| e.to_string())?
        ),
    )
    .map_err(|e| e.to_string())?;
    println!(
        "wrote {} ({added} added, {preserved} preserved)",
        destination.display()
    );
    Ok(())
}

fn workspace_root() -> Result<PathBuf, String> {
    Ok(PathBuf::from(env::var("CARGO_MANIFEST_DIR").map_err(|e| e.to_string())?).join("../.."))
}

pub fn run() -> Result<(), String> {
    let root = workspace_root()?;
    let manifest_path = root.join("verification/operations.json");
    let manifest: Value = serde_json::from_str(
        &fs::read_to_string(&manifest_path)
            .map_err(|e| format!("cannot read {}: {e}", manifest_path.display()))?,
    )
    .map_err(|e| format!("invalid {}: {e}", manifest_path.display()))?;
    if manifest["schema_version"] != 1 {
        return Err("verification manifest schema_version must be 1".into());
    }
    let evidence = manifest["operations"]
        .as_object()
        .ok_or_else(|| "verification manifest operations must be an object".to_string())?;
    let names = runtime::operation_names(None);
    let mut errors = Vec::new();
    let mut seen_names = HashSet::new();
    let mut seen_ids = HashSet::new();
    let mut rows = Vec::with_capacity(names.len());
    let mut totals = AuditTotals::default();
    let fixture_index = fixture_evidence(&root)?;

    for name in names {
        let info = runtime::operation_info(&name)?;
        if !seen_names.insert(info.name.to_ascii_lowercase()) {
            errors.push(format!("duplicate operation name: {}", info.name));
        }
        if !seen_ids.insert(info.id.clone()) {
            errors.push(format!("duplicate normalized operation id: {}", info.id));
        }
        if info.name.trim().is_empty() || info.module.trim().is_empty() {
            errors.push(format!("operation without metadata: {}", info.id));
        }
        if info.description.trim().is_empty() {
            errors.push(format!("empty description: {}", info.name));
        }
        if info.is_broken != (info.availability == Availability::FeatureDisabled) {
            errors.push(format!("is_broken/availability mismatch: {}", info.name));
        }
        if info.availability == Availability::FeatureDisabled
            && info.feature_requirements.is_empty()
        {
            errors.push(format!(
                "feature-gated operation without feature metadata: {}",
                info.name
            ));
        }

        for argument in info.args {
            if argument.name.trim().is_empty() || argument.description.trim().is_empty() {
                errors.push(format!(
                    "operation has undocumented arguments: {}",
                    info.name
                ));
            }
            if argument.kind == ArgKind::Enum && argument.choices.is_empty() {
                errors.push(format!(
                    "enum argument without choices: {} / {}",
                    info.name, argument.name
                ));
            }
            if argument.kind != ArgKind::Enum && !argument.choices.is_empty() {
                errors.push(format!(
                    "non-enum argument has choices: {} / {}",
                    info.name, argument.name
                ));
            }
            if !argument.choices.is_empty()
                && !argument.default_value.is_empty()
                && !argument
                    .choices
                    .iter()
                    .any(|choice| choice.eq_ignore_ascii_case(argument.default_value))
            {
                errors.push(format!(
                    "default is not an allowed choice: {} / {}",
                    info.name, argument.name
                ));
            }
            if argument.required && !argument.default_value.is_empty() {
                errors.push(format!(
                    "required argument also declares a default: {} / {}",
                    info.name, argument.name
                ));
            }
            if let (Some(minimum), Some(maximum)) = (argument.minimum, argument.maximum) {
                if bound_value(minimum) > bound_value(maximum) {
                    errors.push(format!(
                        "invalid numeric bounds: {} / {}",
                        info.name, argument.name
                    ));
                }
            }
            if (argument.minimum.is_some() || argument.maximum.is_some())
                && !matches!(
                    argument.kind,
                    ArgKind::Integer | ArgKind::UnsignedInteger | ArgKind::Float
                )
            {
                errors.push(format!(
                    "non-numeric argument has numeric bounds: {} / {}",
                    info.name, argument.name
                ));
            }
        }

        let source_id = runtime::operation_source(&name)?;
        let source = root.join(format!("src/operations/{source_id}.rs"));
        if !source.is_file() {
            errors.push(format!(
                "missing module for {}: {}",
                info.name,
                source.display()
            ));
            continue;
        }
        let source_text = fs::read_to_string(&source).map_err(|e| e.to_string())?;
        for marker in ["todo!", "unimplemented!", "stub implementation"] {
            if source_text.to_ascii_lowercase().contains(marker) {
                errors.push(format!(
                    "placeholder marker '{marker}' in {}",
                    source.display()
                ));
            }
        }

        let Some(verification) = evidence.get(&info.id) else {
            errors.push(format!(
                "operation without verification evidence: {}",
                info.name
            ));
            continue;
        };
        let correctness = string_array(verification, "correctness", &info.name, &mut errors);
        let known_answer = string_array(verification, "known_answer", &info.name, &mut errors);
        let differential = string_array(verification, "differential", &info.name, &mut errors);
        let property = string_array(verification, "property", &info.name, &mut errors);
        let fuzz = string_array(verification, "fuzz", &info.name, &mut errors);
        let benchmark_entries = string_array(verification, "benchmark", &info.name, &mut errors);
        if correctness.is_empty() {
            errors.push(format!(
                "operation without correctness evidence: {}",
                info.name
            ));
        }
        for mapping in &correctness {
            if !root.join(mapping).is_file() {
                errors.push(format!(
                    "missing correctness evidence for {}: {mapping}",
                    info.name
                ));
            }
        }
        let docs = root.join(format!(
            "docs/operations/{}.md",
            info.name.replace('/', "_")
        ));
        if !docs.is_file() {
            errors.push(format!("operation without docs: {}", info.name));
        }
        let benchmark_skip_reason = verification["benchmark_skip_reason"].as_str();
        if benchmark_entries.is_empty()
            && benchmark_skip_reason.is_none_or(|reason| reason.trim().is_empty())
        {
            errors.push(format!(
                "operation without benchmark case or skip reason: {}",
                info.name
            ));
        }

        // Evidence is read out of the mapped sources rather than inferred from
        // the mapping existing. A file that contains no `#[test]` proves
        // nothing and must carry an explicit, reviewed reason.
        let evidence = test_evidence::scan_files(&root, &correctness);
        let fixture = fixture_index.get(info.name).copied().unwrap_or_default();
        let untested_reason = verification["untested_reason"].as_str();
        if !evidence.has_tests() {
            match untested_reason {
                Some(reason) if !reason.trim().is_empty() => {}
                _ => errors.push(format!(
                    "correctness evidence for {} contains no `#[test]`: {}",
                    info.name,
                    correctness.join(", ")
                )),
            }
        } else if untested_reason.is_some() {
            errors.push(format!(
                "{} declares untested_reason but its tests do execute",
                info.name
            ));
        }

        // A known-answer claim is only meaningful next to an exact-value
        // assertion, and parity claims stronger than `Compatible` require a
        // recorded differential case rather than a spec vector.
        if !known_answer.is_empty() && evidence.value_assertions == 0 {
            errors.push(format!(
                "{} claims known-answer evidence but asserts no exact value",
                info.name
            ));
        }
        if info.parity == ParityStatus::Exact && differential.is_empty() {
            errors.push(format!(
                "{} claims exact CyberChef parity without differential evidence",
                info.name
            ));
        }
        if matches!(info.parity, ParityStatus::Compatible) && known_answer.is_empty() {
            errors.push(format!(
                "{} claims compatible parity without known-answer evidence",
                info.name
            ));
        }

        let negative_policy = NegativeTestPolicy::read(verification, info.name, &mut errors);

        let mut present_targets = BTreeSet::new();
        if !correctness.is_empty() {
            present_targets.insert("correctness");
        }
        if !known_answer.is_empty() {
            present_targets.insert("known_answer");
        }
        if !differential.is_empty() {
            present_targets.insert("differential");
        }
        if !property.is_empty() {
            present_targets.insert("property");
        }
        let provenance =
            read_evidence_provenance(verification, info.name, &present_targets, &mut errors);

        totals.record(
            &evidence,
            fixture,
            &negative_policy,
            &info,
            !known_answer.is_empty(),
            !differential.is_empty(),
            &provenance,
        );
        let has_independent_evidence = provenance.iter().any(ProvenanceRecord::is_independent);
        let evidence_provenance_json: Vec<Value> = provenance
            .iter()
            .map(|record| json!({"target": record.target, "type": record.kind}))
            .collect();

        // Named distinctly: `correctness` above is the manifest's list of test
        // files, which this row still serialises as `test_mapping`.
        let correctness_verdict = Correctness::derive(
            &evidence,
            fixture,
            &negative_policy,
            !known_answer.is_empty(),
            !differential.is_empty(),
        );
        let correctness_gaps = Correctness::missing(
            &evidence,
            fixture,
            &negative_policy,
            !known_answer.is_empty(),
            !differential.is_empty(),
        );

        rows.push(json!({
            "correctness": correctness_verdict.as_str(),
            "correctness_gaps": correctness_gaps,
            "negative_test_policy": negative_policy.as_json(),
            "fixture_empty_input": fixture.empty_input,
            "fixture_rejection": fixture.rejection,
            "fixture_binary_or_unicode": fixture.binary_or_unicode,
            "tests": evidence.tests,
            "negative_test": evidence.has_negative_case(),
            "boundary_test": evidence.has_boundary_case(),
            "empty_input_test": evidence.empty_input_cases > 0,
            "value_assertions": evidence.value_assertions,
            "tests_asserting_only_success": evidence.weak_ok_assertions,
            "untested_reason": untested_reason,
            "name": info.name,
            "slug": info.id.replace('_', "-"),
            "id": info.id,
            "module": info.module,
            "status": info.implementation_status,
            "availability": info.availability,
            "feature": info.feature_requirements,
            "input": data_type_name(info.input_type),
            "output": data_type_name(info.output_type),
            "args_documented": info.args.iter().all(|arg| !arg.name.trim().is_empty() && !arg.description.trim().is_empty()),
            "correctness_test": evidence.has_tests(),
            "known_answer_test": !known_answer.is_empty(),
            "differential_test": !differential.is_empty(),
            "property_test": !property.is_empty(),
            "test_mapping": correctness,
            "known_answer": known_answer,
            "differential": differential,
            "property": property,
            "fuzz": fuzz,
            "fuzz_target": !fuzz.is_empty(),
            "benchmark": !benchmark_entries.is_empty(),
            "benchmark_evidence": benchmark_entries,
            "benchmark_skip_reason": benchmark_skip_reason,
            "docs": docs.is_file(),
            "parity": info.parity,
            "evidence_provenance": evidence_provenance_json,
            "independent_evidence": has_independent_evidence,
        }));
    }

    for id in evidence.keys() {
        if !seen_ids.contains(id) {
            errors.push(format!(
                "verification evidence references unknown operation id: {id}"
            ));
        }
    }

    if !errors.is_empty() {
        errors.sort();
        return Err(format!(
            "operation audit failed with {} issue(s):\n{}",
            errors.len(),
            errors.join("\n")
        ));
    }

    rows.sort_by(|left, right| text_field(left, "name").cmp(text_field(right, "name")));
    let generated = root.join("docs/_generated/operation-quality.json");
    let reference = root.join("docs/reference/operation-matrix.md");
    fs::create_dir_all(generated.parent().unwrap()).map_err(|e| e.to_string())?;
    fs::create_dir_all(reference.parent().unwrap()).map_err(|e| e.to_string())?;
    let json = serde_json::to_string_pretty(&json!({
        "schema_version": 2,
        "operation_count": rows.len(),
        "summary": totals.as_json(),
        "operations": rows,
    }))
    .map_err(|e| e.to_string())?;
    fs::write(&generated, format!("{json}\n")).map_err(|e| e.to_string())?;
    fs::write(&reference, render_markdown(&rows, &totals)).map_err(|e| e.to_string())?;
    let backlog = root.join("docs/_generated/verification-backlog.md");
    fs::write(&backlog, render_backlog(&rows, &totals)).map_err(|e| e.to_string())?;
    println!("operation audit passed ({} operations)", rows.len());
    totals.print_summary();
    Ok(())
}

fn bound_value(bound: NumericBound) -> f64 {
    match bound {
        NumericBound::Integer(value) => value as f64,
        NumericBound::Unsigned(value) => value as f64,
        NumericBound::Float(value) => value,
    }
}

fn string_array(
    value: &Value,
    field: &str,
    operation: &str,
    errors: &mut Vec<String>,
) -> Vec<String> {
    let Some(array) = value[field].as_array() else {
        errors.push(format!(
            "verification field '{field}' is not an array: {operation}"
        ));
        return Vec::new();
    };
    let mut output = Vec::with_capacity(array.len());
    for entry in array {
        if let Some(entry) = entry.as_str().filter(|entry| !entry.trim().is_empty()) {
            output.push(entry.to_string());
        } else {
            errors.push(format!(
                "verification field '{field}' has an invalid entry: {operation}"
            ));
        }
    }
    output
}

fn text_field<'a>(value: &'a Value, field: &str) -> &'a str {
    value[field].as_str().unwrap_or("")
}

fn render_markdown(rows: &[Value], totals: &AuditTotals) -> String {
    let mut modules = BTreeSet::new();
    for row in rows {
        modules.insert(text_field(row, "module"));
    }
    let mut output = String::from(
        "# Operation quality matrix\n\n<!-- Generated by `cargo xtask audit-operations`; do not edit. -->\n\n",
    );
    output.push_str(
        "`Tests` counts `#[test]` functions in the mapped sources. `Neg` and `Bound` mark negative and boundary/empty-input coverage found in those sources. `KAT` and `Diff` are reviewer claims recorded in `verification/operations.json`; `partial` and `unknown` are deliberate audit results, not release-completeness claims.\n\n",
    );
    output.push_str("## Summary\n\n| Metric | Count |\n|---|---:|\n");
    for (label, value) in [
        ("Registered operations", totals.registered),
        ("With executable tests", totals.tested),
        ("Test functions", totals.total_tests),
        ("With negative tests", totals.negative_tested),
        ("With boundary tests", totals.boundary_tested),
        (
            "Reference-verified (known answer)",
            totals.reference_verified,
        ),
        ("Differential-verified", totals.differential_verified),
        ("Parity: exact", totals.parity_exact),
        ("Parity: compatible", totals.parity_compatible),
        (
            "Parity: documented difference",
            totals.parity_documented_difference,
        ),
        ("Parity: unverified", totals.parity_unknown),
        ("Parity: not applicable", totals.parity_not_applicable),
        ("Documented divergences", totals.known_limitations),
        ("Tests asserting only success", totals.weak_ok_assertions),
        (
            "With typed evidence provenance",
            totals.with_typed_provenance,
        ),
        (
            "With independent evidence (typed)",
            totals.independent_evidence_operations,
        ),
        (
            "Parity: exact, without typed independent evidence",
            totals.parity_exact_missing_independent_evidence,
        ),
    ] {
        output.push_str(&format!("| {label} | {value} |\n"));
    }
    output.push_str(
        "\nSee [What \"verified\" means](../reference/verification.md) for what each evidence type in `evidence_provenance` does and does not prove.\n",
    );
    output.push('\n');

    let module_count = modules.len();
    for (module_index, module) in modules.into_iter().enumerate() {
        output.push_str(&format!("## {module}\n\n"));
        output.push_str("| Operation | Status | Parity | Args | Tests | Neg | Bound | KAT | Diff | Property | Fuzz | Bench | Docs |\n|---|---|---|:---:|---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|\n");
        for row in rows
            .iter()
            .filter(|row| text_field(row, "module") == module)
        {
            output.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
                text_field(row, "name").replace('|', "\\|"),
                text_field(row, "status"),
                text_field(row, "parity"),
                mark(row["args_documented"].as_bool().unwrap_or(false)),
                row["tests"].as_u64().unwrap_or(0),
                mark(row["negative_test"].as_bool().unwrap_or(false)),
                mark(row["boundary_test"].as_bool().unwrap_or(false)),
                mark(row["known_answer_test"].as_bool().unwrap_or(false)),
                mark(row["differential_test"].as_bool().unwrap_or(false)),
                mark(row["property_test"].as_bool().unwrap_or(false)),
                mark(row["fuzz_target"].as_bool().unwrap_or(false)),
                mark(row["benchmark"].as_bool().unwrap_or(false)),
                mark(row["docs"].as_bool().unwrap_or(false)),
            ));
        }
        if module_index + 1 < module_count {
            output.push('\n');
        }
    }
    output
}

/// Classify why an operation is not yet `verified`.
///
/// The groups mirror the work each one needs, so the backlog can be picked up
/// in batches instead of read operation by operation.
fn backlog_group(row: &Value) -> &'static str {
    if row["correctness"] == "verified" {
        return "verified";
    }
    let gaps: Vec<&str> = row["correctness_gaps"]
        .as_array()
        .map(|list| list.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();
    let independent = gaps.contains(&"independent reference evidence");
    let negative = gaps.contains(&"negative tests");
    let boundary = gaps.contains(&"boundary tests");
    let weak = row["tests_asserting_only_success"].as_u64().unwrap_or(0) > 0;

    if gaps.contains(&"executable tests") {
        return "F: no executable tests";
    }
    match (independent, negative, boundary) {
        (true, false, false) if weak => "J: independent evidence missing, tests are weak",
        (true, false, false) => "A: only independent evidence missing",
        (false, true, false) => "B: only negative evidence missing",
        (false, false, true) => "C: only boundary evidence missing",
        (false, true, true) => "D: negative + boundary missing",
        (true, true, false) | (true, false, true) => "E: independent evidence + one test class",
        (true, true, true) => "F: multiple test classes missing",
        (false, false, false) => "H: special semantic verification required",
    }
}

/// Suggest how to close the gap, from what the operation looks like.
fn recommended_strategy(row: &Value) -> String {
    let module = row["module"].as_str().unwrap_or("");
    let name = row["name"].as_str().unwrap_or("");
    let gaps: Vec<&str> = row["correctness_gaps"]
        .as_array()
        .map(|list| list.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();
    let mut advice = Vec::new();

    if gaps.contains(&"independent reference evidence") {
        let standardised = matches!(
            module,
            "Hashing" | "Crypto" | "Ciphers" | "Encodings" | "Compression" | "PublicKey"
        );
        if standardised {
            advice.push(
                "authoritative known-answer vectors (RFC/NIST/FIPS or the algorithm's own published test set), preferred over a differential comparison",
            );
        } else {
            advice.push(
                "CyberChef differential fixture with an input this operation actually accepts",
            );
        }
    }
    if gaps.contains(&"negative tests") {
        advice.push(
            "a negative case asserting the error variant, or a reviewed negative_test_policy.not_applicable if no invalid input class exists",
        );
    }
    if gaps.contains(&"boundary tests") {
        advice.push("a boundary case at a real limit of this operation, not merely empty input");
    }
    if row["tests_asserting_only_success"].as_u64().unwrap_or(0) > 0 {
        advice.push("replace is_ok()-only assertions with exact values or invariants");
    }
    if advice.is_empty() {
        advice.push("review manually: no mechanical gap remains");
    }
    let _ = name;
    advice.join("; ")
}

/// Render the generated verification backlog.
fn render_backlog(rows: &[Value], totals: &AuditTotals) -> String {
    let mut grouped: BTreeSet<&'static str> = BTreeSet::new();
    for row in rows {
        grouped.insert(backlog_group(row));
    }

    let mut output = String::from(
        "# Verification backlog\n\n<!-- Generated by `cargo run -p xtask -- audit-operations`; do not edit. -->\n\n",
    );
    output.push_str(
        "Why each operation is not yet `verified`, and what would close the gap. Groups are ordered by how much work they need, so the list can be worked in batches.\n\n",
    );
    output.push_str(&format!(
        "**{} verified · {} partially verified · {} unverified** of {} registered.\n\n",
        totals.correctness_verified,
        totals.correctness_partial,
        totals.correctness_unverified,
        totals.registered
    ));

    for group in grouped {
        if group == "verified" {
            continue;
        }
        let members: Vec<&Value> = rows
            .iter()
            .filter(|row| backlog_group(row) == group)
            .collect();
        output.push_str(&format!("## {group} ({})\n\n", members.len()));
        output.push_str(
            "| Operation | Module | Tests | Neg | Bound | KAT | Diff | Parity | Weak | Remaining gaps | Strategy |\n|---|---|---:|:---:|:---:|:---:|:---:|---|---:|---|---|\n",
        );
        for row in members {
            let gaps = row["correctness_gaps"]
                .as_array()
                .map(|list| {
                    list.iter()
                        .filter_map(Value::as_str)
                        .collect::<Vec<_>>()
                        .join(", ")
                })
                .unwrap_or_default();
            let negative = if row["negative_test_policy"]["not_applicable"].is_string() {
                "n/a"
            } else {
                mark(
                    row["negative_test"].as_bool().unwrap_or(false)
                        || row["fixture_rejection"].as_bool().unwrap_or(false),
                )
            };
            output.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
                text_field(row, "name").replace('|', "\\|"),
                text_field(row, "module"),
                row["tests"].as_u64().unwrap_or(0),
                negative,
                mark(
                    row["boundary_test"].as_bool().unwrap_or(false)
                        || row["fixture_empty_input"].as_bool().unwrap_or(false)
                        || row["fixture_binary_or_unicode"].as_bool().unwrap_or(false)
                ),
                mark(row["known_answer_test"].as_bool().unwrap_or(false)),
                mark(row["differential_test"].as_bool().unwrap_or(false)),
                text_field(row, "parity"),
                row["tests_asserting_only_success"].as_u64().unwrap_or(0),
                gaps,
                recommended_strategy(row),
            ));
        }
        output.push('\n');
    }
    output
}

fn mark(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "—"
    }
}

#[cfg(test)]
mod tests {
    use super::{Correctness, FixtureEvidence, NegativeTestPolicy};
    use crate::test_evidence::TestEvidence;
    use serde_json::json;

    fn tested() -> TestEvidence {
        TestEvidence {
            tests: 3,
            value_assertions: 3,
            ..TestEvidence::default()
        }
    }

    fn with_boundary() -> FixtureEvidence {
        FixtureEvidence {
            empty_input: true,
            any: true,
            ..FixtureEvidence::default()
        }
    }

    fn policy(value: serde_json::Value) -> (NegativeTestPolicy, Vec<String>) {
        let mut errors = Vec::new();
        let verification = json!({ "negative_test_policy": value });
        let parsed = NegativeTestPolicy::read(&verification, "Example", &mut errors);
        (parsed, errors)
    }

    // ── The policy may never be inferred ──────────────────────────────────

    #[test]
    fn an_absent_policy_defaults_to_required() {
        let mut errors = Vec::new();
        let parsed = NegativeTestPolicy::read(&json!({}), "Example", &mut errors);
        assert_eq!(parsed, NegativeTestPolicy::Required);
        assert!(errors.is_empty());
    }

    #[test]
    fn not_applicable_without_a_justification_is_rejected() {
        let (parsed, errors) = policy(json!({ "not_applicable": "" }));
        assert_eq!(
            parsed,
            NegativeTestPolicy::Required,
            "an unjustified exemption must not take effect"
        );
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn a_justification_too_short_to_review_is_rejected() {
        let (parsed, errors) = policy(json!({ "not_applicable": "accepts anything" }));
        assert_eq!(parsed, NegativeTestPolicy::Required);
        assert!(errors[0].contains("too thin"));
    }

    #[test]
    fn excuses_are_not_justifications() {
        for excuse in [
            "We could not find a failure case for this operation at all here",
            "Writing this test would be laborious and is not worth the effort",
            "CyberChef does not error either so there is nothing to assert here",
        ] {
            let (parsed, errors) = policy(json!({ "not_applicable": excuse }));
            assert_eq!(
                parsed,
                NegativeTestPolicy::Required,
                "excuse accepted as justification: {excuse}"
            );
            assert!(
                errors.iter().any(|e| e.contains("excuse")),
                "expected an excuse diagnostic for: {excuse}"
            );
        }
    }

    #[test]
    fn a_reviewed_justification_is_accepted() {
        let (parsed, errors) = policy(json!({
            "not_applicable":
                "Accepts arbitrary bytes; every input has a defined output per the \
                 specification and the operation declares no arguments."
        }));
        assert!(parsed.is_not_applicable());
        assert!(errors.is_empty());
    }

    // ── The exemption must not shortcut the other dimensions ──────────────

    #[test]
    fn not_applicable_alone_does_not_make_an_operation_verified() {
        let exempt = NegativeTestPolicy::NotApplicable {
            justification: "Accepts arbitrary bytes; every input has a defined output.".into(),
        };
        // Boundary evidence and independent evidence are still required.
        assert_eq!(
            Correctness::derive(&tested(), FixtureEvidence::default(), &exempt, false, false),
            Correctness::PartiallyVerified
        );
        assert_eq!(
            Correctness::derive(&tested(), with_boundary(), &exempt, false, false),
            Correctness::PartiallyVerified,
            "independent evidence is still required"
        );
        assert_eq!(
            Correctness::derive(&tested(), with_boundary(), &exempt, true, false),
            Correctness::Verified
        );
    }

    #[test]
    fn an_operation_without_tests_stays_unverified_regardless_of_policy() {
        let exempt = NegativeTestPolicy::NotApplicable {
            justification: "Accepts arbitrary bytes; every input has a defined output.".into(),
        };
        assert_eq!(
            Correctness::derive(
                &TestEvidence::default(),
                FixtureEvidence::default(),
                &exempt,
                true,
                true
            ),
            Correctness::Unverified
        );
    }

    #[test]
    fn required_policy_still_demands_a_negative_case() {
        assert_eq!(
            Correctness::derive(
                &tested(),
                with_boundary(),
                &NegativeTestPolicy::Required,
                true,
                true
            ),
            Correctness::PartiallyVerified,
            "a required negative test cannot be skipped"
        );
    }

    #[test]
    fn the_gap_list_reflects_the_policy() {
        let exempt = NegativeTestPolicy::NotApplicable {
            justification: "Accepts arbitrary bytes; every input has a defined output.".into(),
        };
        let gaps = Correctness::missing(&tested(), with_boundary(), &exempt, false, false);
        assert_eq!(gaps, vec!["independent reference evidence"]);

        let gaps = Correctness::missing(
            &tested(),
            with_boundary(),
            &NegativeTestPolicy::Required,
            false,
            false,
        );
        assert!(gaps.contains(&"negative tests"));
    }
}
