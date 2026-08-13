use rxchef::{
    operation::OperationStatus,
    runtime::{self, data_type_name},
};
use serde_json::{json, Value};
use std::{
    collections::{BTreeSet, HashSet},
    env, fs,
    path::{Path, PathBuf},
};

const BENCHMARKED: &[&str] = &[
    "to_hex",
    "to_base64",
    "sha2",
    "aes_encrypt",
    "gzip_compress",
    "magic",
    "scan",
];

pub fn run() -> Result<(), String> {
    let root =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").map_err(|e| e.to_string())?).join("../..");
    let names = runtime::operation_names(None);
    let mut errors = Vec::new();
    let mut seen_names = HashSet::new();
    let mut seen_ids = HashSet::new();
    let mut rows = Vec::with_capacity(names.len());

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
        if info.is_broken != (info.status == OperationStatus::FeatureGated) {
            errors.push(format!(
                "is_broken/feature-gated status mismatch: {}",
                info.name
            ));
        }
        if info.status == OperationStatus::FeatureGated && info.feature_requirements.is_empty() {
            errors.push(format!(
                "feature-gated operation without feature metadata: {}",
                info.name
            ));
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

        let test = root.join(format!("tests/tests/operations/{source_id}.rs"));
        let test_text = fs::read_to_string(&test).unwrap_or_default();
        if test_text.is_empty() {
            errors.push(format!("operation without test mapping: {}", info.name));
        }
        let docs = root.join(format!(
            "docs/operations/{}.md",
            info.name.replace('/', "_")
        ));
        if !docs.is_file() {
            errors.push(format!("operation without docs: {}", info.name));
        }
        if info
            .args
            .iter()
            .any(|arg| arg.name.trim().is_empty() || arg.description.trim().is_empty())
        {
            errors.push(format!(
                "operation has undocumented arguments: {}",
                info.name
            ));
        }

        let benchmark = BENCHMARKED.contains(&info.id.as_str());
        let benchmark_skip_reason =
            (!benchmark).then_some("not selected for the representative benchmark catalog");
        if !benchmark && benchmark_skip_reason.is_none() {
            errors.push(format!(
                "operation without benchmark case or skip reason: {}",
                info.name
            ));
        }

        rows.push(json!({
            "name": info.name,
            "slug": info.id.replace('_', "-"),
            "id": info.id,
            "module": info.module,
            "status": info.status,
            "feature": info.feature_requirements,
            "input": data_type_name(info.input_type),
            "output": data_type_name(info.output_type),
            "args_documented": info.args.iter().all(|arg| !arg.name.trim().is_empty() && !arg.description.trim().is_empty()),
            "correctness_test": test_text.contains("#[test]"),
            "known_answer_test": test_text.contains("assert_eq!"),
            "differential_test": contains_any(&test_text, &["differential", "openssl", "upstream vector"]),
            "property_test": contains_any(&test_text, &["proptest", "quickcheck", "property"]),
            "test_mapping": relative(&root, &test),
            "fuzz_target": has_fuzz_target(&root, &info.id),
            "benchmark": benchmark,
            "benchmark_skip_reason": benchmark_skip_reason,
            "docs": docs.is_file(),
            "parity": info.parity,
        }));
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
        "schema_version": 1,
        "operation_count": rows.len(),
        "operations": rows,
    }))
    .map_err(|e| e.to_string())?;
    fs::write(&generated, format!("{json}\n")).map_err(|e| e.to_string())?;
    fs::write(&reference, render_markdown(&rows)).map_err(|e| e.to_string())?;
    println!("operation audit passed ({} operations)", rows.len());
    Ok(())
}

fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    let haystack = haystack.to_ascii_lowercase();
    needles.iter().any(|needle| haystack.contains(needle))
}

fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn has_fuzz_target(root: &Path, id: &str) -> bool {
    let directory = root.join("fuzz/fuzz_targets");
    fs::read_dir(directory)
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .any(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .to_ascii_lowercase()
                .contains(id)
        })
}

fn text_field<'a>(value: &'a Value, field: &str) -> &'a str {
    value[field].as_str().unwrap_or("")
}

fn render_markdown(rows: &[Value]) -> String {
    let mut modules = BTreeSet::new();
    for row in rows {
        modules.insert(text_field(row, "module"));
    }
    let mut output = format!(
        "# Operation quality matrix\n\n<!-- Generated by `cargo xtask audit-operations`; do not edit. -->\n\nRegistered operations: **{}**. `partial` and `unknown` are deliberate audit results, not release-completeness claims.\n\n",
        rows.len()
    );
    let module_count = modules.len();
    for (module_index, module) in modules.into_iter().enumerate() {
        output.push_str(&format!("## {module}\n\n"));
        output.push_str("| Operation | Status | Parity | Args | Test | KAT | Diff | Property | Fuzz | Bench | Docs |\n|---|---|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|\n");
        for row in rows
            .iter()
            .filter(|row| text_field(row, "module") == module)
        {
            output.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
                text_field(row, "name").replace('|', "\\|"),
                text_field(row, "status"),
                text_field(row, "parity"),
                mark(row["args_documented"].as_bool().unwrap_or(false)),
                mark(row["correctness_test"].as_bool().unwrap_or(false)),
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

fn mark(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "—"
    }
}
