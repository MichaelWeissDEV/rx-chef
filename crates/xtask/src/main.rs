use rxchef::runtime::{data_type_name, operation_info, operation_names};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::PathBuf;

mod bench;
mod operation_audit;
mod registry;

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
        _ => Ok(()),
    }
}

fn cmd_docs(args: &[String]) -> Result<(), String> {
    let check = args.iter().any(|arg| arg == "--check");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_root = PathBuf::from(manifest_dir).join("../../");
    let docs_dir = workspace_root.join("docs/operations");

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
            "| Field | Value |\n|---|---|\n| Implementation | `{:?}` |\n| Parity | `{:?}` |\n| Availability | {} |\n| Features | {} |\n| Side effects | `{:?}` |\n| Deterministic | {} |\n\n",
            info.status,
            info.parity,
            if info.is_broken { "unavailable in this build" } else { "available" },
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
                    "| {} | {} | `{:?}` | no | {} | — | {} | {} |\n",
                    index + 1,
                    markdown_escape(arg.name),
                    rxchef::runtime::inferred_arg_kind(arg),
                    inline_code(&markdown_escape(arg.default_value)),
                    if rxchef::runtime::is_sensitive_arg(arg) {
                        "yes"
                    } else {
                        "no"
                    },
                    markdown_escape(arg.description)
                ));
            }
            out.push('\n');
        }
        let escaped_name = info.name.replace('"', "\\\"");
        out.push_str(&format!(
            "## How it works\n\nThe shared execution engine validates the ordered arguments, passes the declared input representation to this operation, and validates the declared output contract. See the overview for the operation-specific format or algorithm.\n\n## Implementation\n\nSource module: `src/operations/{}.rs`. Execution uses `rxchef::execute`; CLI, recipes, and the stdio server do not carry separate operation logic.\n\n## Examples\n\n```console\nprintf 'input' | rxchef run \"{}\"\n```\n\nFor file or binary input use `rxchef run \"{}\" --input-file INPUT --output-file OUTPUT`.\n\n## Pipeline usage\n\n```console\nprintf 'input' | rxchef pipe \"{}\" to_base64\n```\n\n## Error conditions\n\nInvalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.\n\n## CyberChef compatibility\n\nParity status: `{:?}`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.\n\n## Security considerations\n\nSide effects: `{:?}`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.\n\n## Testing\n\nThe mapped Rust test and available KAT/differential/property/fuzz evidence are recorded in the [operation quality matrix](../reference/operation-matrix.md).\n\n## Performance\n\nSee [benchmark results](../performance/results.md). Operations outside the representative catalog are explicitly marked with a skip rationale in the machine-readable quality inventory. Measurements are hardware-dependent reference values, not guarantees.\n\n## Limitations\n\n{}\n\n## References\n\n- [Operation quality matrix](../reference/operation-matrix.md)\n- [CLI run documentation](../cli/run.md)\n",
            rxchef::runtime::operation_source(info.name)?,
            escaped_name,
            escaped_name,
            escaped_name,
            info.parity,
            info.side_effects,
            if info.known_limitations.is_empty() { "No verified limitation metadata is currently recorded; this is not a claim of perfect upstream parity.".to_string() } else { info.known_limitations.join("\n- ") },
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
