use rxchef::runtime::{data_type_name, operation_info, operation_names};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::PathBuf;

mod bench;
mod operation_audit;
mod registry;
mod test_evidence;
mod verify_version;

fn markdown_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('|', "\\|")
        .replace('\n', " ")
}

fn inline_code(value: &str) -> String {
    if value.is_empty() {
        "`<empty>`".to_string()
    } else if value.contains('`') {
        format!("``{value}``")
    } else {
        format!("`{value}`")
    }
}

fn write_or_check(path: &std::path::Path, content: &str, check: bool) -> Result<(), String> {
    if check {
        let current = fs::read_to_string(path)
            .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
        if current != content {
            return Err(format!(
                "{} is stale; run `cargo run -p xtask -- docs`",
                path.display()
            ));
        }
        Ok(())
    } else {
        fs::write(path, content).map_err(|error| error.to_string())
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("docs") => cmd_docs(&args),
        Some("bench") => bench::run(&args[2.min(args.len())..]),
        Some("bench-docs") => bench::run_docs(&args[2.min(args.len())..]),
        Some("bench-docs-internal") => bench::run_docs_internal(&args[2.min(args.len())..]),
        Some("generate-registry") => registry::generate(false),
        Some("check-registry") => registry::generate(true),
        Some("audit-operations") => operation_audit::run(),
        Some("verify-version") => verify_version::run(!args.iter().any(|arg| arg == "--write")),
        Some("generate-verification-manifest") => operation_audit::generate_manifest(),
        _ => Ok(()),
    }
}

fn cmd_docs(args: &[String]) -> Result<(), String> {
    let check = args.iter().any(|arg| arg == "--check");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_root = PathBuf::from(manifest_dir).join("../../");
    let docs_dir = workspace_root.join("docs/operations");
    let verification_document: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(workspace_root.join("verification/operations.json"))
            .map_err(|error| format!("cannot read verification manifest: {error}"))?,
    )
    .map_err(|error| format!("invalid verification manifest: {error}"))?;
    let verification = verification_document["operations"]
        .as_object()
        .ok_or_else(|| "verification manifest operations must be an object".to_string())?;

    fs::create_dir_all(&docs_dir).map_err(|e| e.to_string())?;

    let names = operation_names(None);
    let mut modules: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for name in &names {
        let info = operation_info(name)?;
        modules
            .entry(info.module.to_string())
            .or_default()
            .push(name.clone());

        let mut out = String::new();
        out.push_str(&format!("# {}\n\n", info.name));
        if info.is_broken {
            out.push_str("!!! warning \"Optional backend unavailable\"\n\n    This operation is feature-gated and unavailable in the minimal documentation build. See the feature matrix for the required Cargo feature.\n\n");
        }
        out.push_str("## Overview\n\n");
        out.push_str(info.description.trim());
        out.push_str("\n\n## Status\n\n");
        out.push_str(&format!(
            "| Field | Value |\n|---|---|\n| Implementation | `{:?}` |\n| Parity | `{:?}` |\n| Availability | {} |\n| Input requirement | `{:?}` |\n| Features | {} |\n| Side effects | `{:?}` |\n| Deterministic | {} |\n\n",
            info.implementation_status,
            info.parity,
            format!("{:?}", info.availability),
            info.input_requirement,
            if info.feature_requirements.is_empty() { "none".into() } else { info.feature_requirements.join(", ") },
            info.side_effects,
            info.deterministic,
        ));
        out.push_str(&format!(
            "## Input\n\nDeclared input type: `{}`.\n\n## Output\n\nDeclared output type: `{}`. Redirect stdout or use `--output-file` for exact binary bytes.\n\n",
            data_type_name(info.input_type), data_type_name(info.output_type)
        ));
        if info.args.is_empty() {
            out.push_str("## Arguments\n\nThis operation has no arguments.\n\n");
        } else {
            out.push_str(
                "## Arguments\n\n| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |\n|---:|---|---|:---:|---|---|:---:|---|\n",
            );
            for (index, arg) in info.args.iter().enumerate() {
                out.push_str(&format!(
                    "| {} | {} | `{:?}` | {} | {} | {} | {} | {} |\n",
                    index + 1,
                    markdown_escape(arg.name),
                    arg.kind,
                    if arg.required { "yes" } else { "no" },
                    inline_code(&markdown_escape(arg.default_value)),
                    if arg.choices.is_empty() {
                        "—".to_string()
                    } else {
                        markdown_escape(&arg.choices.join(", "))
                    },
                    if arg.sensitive { "yes" } else { "no" },
                    markdown_escape(arg.description)
                ));
            }
            out.push('\n');
        }
        let escaped_name = info.name.replace('"', "\\\"");
        let command_line = match info.input_requirement {
            rxchef::operation::InputRequirement::Required => format!(
                "This operation requires input. Supply literal UTF-8 with `--input`, exact bytes with `--input-file`, or pipe bytes on stdin.\n\n```console\nrxchef run \"{escaped_name}\" --input-file input.bin --output-file output.bin\n```"
            ),
            rxchef::operation::InputRequirement::Optional => format!(
                "Input is optional. Omit all input selectors to use the operation's no-input behavior, or provide text, a file, or stdin explicitly.\n\n```console\nrxchef run \"{escaped_name}\" --input-file input.bin --output-file output.bin\n```"
            ),
            rxchef::operation::InputRequirement::Ignored => format!(
                "This operation does not consume pipeline input. Its result is produced from its arguments and runtime state.\n\n```console\nrxchef run \"{escaped_name}\"\n```"
            ),
        };
        let evidence = verification
            .get(&info.id)
            .ok_or_else(|| format!("missing verification evidence for {}", info.name))?;
        let mut testing = Vec::new();
        for (label, field) in [
            ("Correctness tests", "correctness"),
            ("Known-answer tests", "known_answer"),
            ("Differential tests", "differential"),
            ("Property tests", "property"),
            ("Fuzz targets", "fuzz"),
        ] {
            if evidence[field]
                .as_array()
                .is_some_and(|entries| !entries.is_empty())
            {
                testing.push(format!("{label}:\n{}", evidence_lines(evidence, field)));
            }
        }
        let testing = testing.join("\n\n");
        let performance = if evidence["benchmark"]
            .as_array()
            .is_some_and(|items| !items.is_empty())
        {
            format!("Benchmark evidence:\n{}\n\nSee [benchmark results](../performance/results.md) for measured environment and statistics.", evidence_lines(evidence, "benchmark"))
        } else {
            format!(
                "Excluded from the committed representative benchmark set: {}",
                evidence["benchmark_skip_reason"]
                    .as_str()
                    .unwrap_or("no reviewed benchmark rationale recorded")
            )
        };
        let limitations = if info.known_limitations.is_empty() {
            String::new()
        } else {
            format!(
                "\n\n## Known limitations\n\n- {}",
                info.known_limitations.join("\n- ")
            )
        };
        out.push_str(&format!(
            "## Implementation\n\nThe implementation is in `src/operations/{}.rs` and declares `{}` input and `{}` output. The operation module owns conversion and domain-error rules; registry resolution, argument validation, input-requirement enforcement, tracing, and output validation are performed by `rxchef::execution`.\n\n## Command-line use\n\n{}\n\nArguments may be supplied positionally in the table order or by name with repeatable `--arg NAME=VALUE`. Omitted optional arguments use the documented defaults.\n\n## Pipeline use\n\nPlace the operation anywhere a `{}` value is valid. Its `{}` result becomes the next step's input. Compact syntax uses the operation name followed by comma-separated arguments; JSON/YAML recipes use an `op` field and an `args` array.\n\n## Error conditions\n\nSchema violations are rejected before the operation runs. Malformed input, unsupported parameter combinations, unavailable optional backends, and domain processing failures produce structured errors and a non-zero CLI status; partial output is never reported as success.\n\n## CyberChef compatibility\n\nParity status: `{:?}`. `Unknown` records an unassessed compatibility claim; it does not imply equality or incompatibility.\n\n## Security considerations\n\nDeclared side effects: `{:?}`. Treat parser inputs as untrusted and apply execution limits to large data. Arguments marked sensitive in the schema are redacted from metadata-aware History displays.\n\n## Testing evidence\n\n{}\n\n## Performance classification\n\n{}{}\n\n## References\n\n- [Operation quality matrix](../reference/operation-matrix.md)\n- [Operation arguments](../concepts/operation-arguments.md)\n- [CLI run documentation](../cli/run.md)\n",
            rxchef::runtime::operation_source(info.name)?,
            data_type_name(info.input_type),
            data_type_name(info.output_type),
            command_line,
            data_type_name(info.input_type),
            data_type_name(info.output_type),
            info.parity,
            info.side_effects,
            testing,
            performance,
            limitations,
        ));

        let file_name = format!("{}.md", name.replace('/', "_"));
        write_or_check(&docs_dir.join(&file_name), &out, check)?;
    }

    let mut index = String::new();
    index.push_str("# Operations\n\n");
    index.push_str("<!-- Generated by xtask docs; do not edit manually. -->\n\n");
    index.push_str(&format!(
        "This reference contains all **{} operations** registered by rxchef. Operation names are accepted case-insensitively and in normalized forms such as `to_hex`, `to-hex`, and `ToHex`. Use `rxchef info <NAME>` for the same metadata in the terminal and add `--json` for machine-readable output.\n\n",
        names.len()
    ));
    index.push_str("Arguments are positional in the order shown. Omitted arguments use their defaults. CLI values are strings unless prefixed with `num:`, `bool:`, `hex:`, or `bytes:`. For named arguments use `rxchef run <OP> --arg NAME=VALUE`.\n\n");

    for (module, operations) in &modules {
        index.push_str(&format!("## {}\n\n", module));
        for name in operations {
            let info = operation_info(name)?;
            let file_name = format!("{}.md", name.replace('/', "_"));
            index.push_str(&format!("- [{}]({})\n", info.name, file_name));
        }
        index.push('\n');
    }

    write_or_check(&docs_dir.join("index.md"), &index, check)?;

    if check {
        let markdown_files = fs::read_dir(&docs_dir)
            .map_err(|error| error.to_string())?
            .filter_map(Result::ok)
            .filter(|entry| entry.path().extension().and_then(|value| value.to_str()) == Some("md"))
            .count();
        let expected = names.len() + 1;
        if markdown_files != expected {
            return Err(format!(
                "operation docs contain {markdown_files} Markdown files, expected {expected}"
            ));
        }
        println!(
            "operation documentation is current ({} operations)",
            names.len()
        );
    } else {
        println!("generated operations docs in docs/operations/");
    }
    Ok(())
}

fn evidence_lines(value: &serde_json::Value, field: &str) -> String {
    let entries = value[field]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.as_str())
        .collect::<Vec<_>>();
    if entries.is_empty() {
        "- none recorded".to_string()
    } else {
        entries
            .into_iter()
            .map(|entry| format!("- {entry}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
