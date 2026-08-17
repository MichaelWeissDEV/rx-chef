//! Detect version and status drift between the workspace and the files that
//! restate it.
//!
//! The single source of truth is `[workspace.package] version` in the root
//! `Cargo.toml`. `xtask` inherits it via `version.workspace = true`, so
//! `CARGO_PKG_VERSION` is that value at compile time.
//!
//! Only places that genuinely have to agree are checked. Generated operation
//! documentation is deliberately excluded: it quotes examples, not the release
//! version, and synchronising it would be churn.

use rxchef::runtime;
use serde_json::{json, Value};
use std::{env, fs, path::PathBuf};

/// The workspace version, inherited from `[workspace.package]`.
const WORKSPACE_VERSION: &str = env!("CARGO_PKG_VERSION");

fn workspace_root() -> Result<PathBuf, String> {
    Ok(PathBuf::from(env::var("CARGO_MANIFEST_DIR").map_err(|e| e.to_string())?).join("../.."))
}

/// Cargo features declared by the root crate, excluding `default` and the
/// `full` aggregate.
fn declared_features(manifest: &str) -> Vec<String> {
    let mut features = Vec::new();
    let mut in_features = false;
    for line in manifest.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_features = trimmed == "[features]";
            continue;
        }
        if !in_features || trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some((name, _)) = trimmed.split_once('=') {
            let name = name.trim();
            if name != "default" && name != "full" {
                features.push(name.to_string());
            }
        }
    }
    features.sort();
    features
}

/// Rebuild `docs/_generated/operation-status.json` from the live workspace.
///
/// This file previously carried a hand-maintained version, feature list, and
/// test count that had drifted from the code. Regenerating it makes drift
/// impossible rather than merely detectable.
fn render_status(root: &std::path::Path) -> Result<String, String> {
    let manifest = fs::read_to_string(root.join("Cargo.toml"))
        .map_err(|error| format!("cannot read root Cargo.toml: {error}"))?;

    let names = runtime::operation_names(None);
    let mut feature_gated = Vec::new();
    for name in &names {
        let info = runtime::operation_info(name)?;
        if !info.feature_requirements.is_empty() {
            feature_gated.push(info.name.to_string());
        }
    }
    feature_gated.sort();

    let document = json!({
        "version": WORKSPACE_VERSION,
        "workspace_crates": [
            "rxchef",
            "rxchef_cli",
            "rxchef_tui",
            "rxchef_store",
            "cyberchef-rust-tests",
        ],
        "binaries": ["rxchef"],
        "features": declared_features(&manifest),
        "operations_count": names.len(),
        "feature_gated_operations": feature_gated,
        "native_dependencies": ["tesseract"],
    });
    Ok(format!(
        "{}\n",
        serde_json::to_string_pretty(&document).map_err(|e| e.to_string())?
    ))
}

/// `check` reports drift; otherwise the generated file is rewritten.
pub fn run(check: bool) -> Result<(), String> {
    let root = workspace_root()?;
    let mut errors = Vec::new();

    // 1. The generated status document must match the live workspace.
    let status_path = root.join("docs/_generated/operation-status.json");
    let expected = render_status(&root)?;
    if check {
        let current = fs::read_to_string(&status_path)
            .map_err(|error| format!("cannot read {}: {error}", status_path.display()))?;
        if current != expected {
            let current_version = serde_json::from_str::<Value>(&current)
                .ok()
                .and_then(|value| value["version"].as_str().map(str::to_string))
                .unwrap_or_else(|| "<unparseable>".into());
            errors.push(format!(
                "{} is stale (records version {current_version}, workspace is {WORKSPACE_VERSION}); \
                 run `cargo run -p xtask -- verify-version --write`",
                status_path.display()
            ));
        }
    } else {
        fs::create_dir_all(status_path.parent().unwrap()).map_err(|e| e.to_string())?;
        fs::write(&status_path, &expected).map_err(|e| e.to_string())?;
        println!("wrote {}", status_path.display());
    }

    // 2. The README states the version and operation count in prose.
    let readme_path = root.join("README.md");
    let readme = fs::read_to_string(&readme_path)
        .map_err(|error| format!("cannot read {}: {error}", readme_path.display()))?;
    let operation_count = runtime::operation_names(None).len();
    let expected_claim = format!(
        "The current v{WORKSPACE_VERSION} workspace registers {operation_count} operations."
    );
    if !readme.contains(&expected_claim) {
        errors.push(format!(
            "README.md does not contain the current claim {expected_claim:?}"
        ));
    }

    if !errors.is_empty() {
        return Err(format!(
            "version consistency failed with {} issue(s):\n{}",
            errors.len(),
            errors.join("\n")
        ));
    }

    println!(
        "version consistency passed (workspace {WORKSPACE_VERSION}, {operation_count} operations)"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::declared_features;

    #[test]
    fn declared_features_skips_default_and_full_aggregates() {
        let manifest = "\
[package]
name = \"x\"

[features]
default = []
full = [\"pgp\", \"yara\"]
pgp = [\"dep:sequoia-openpgp\"]
yara = [\"dep:yara-x\"]

[dependencies]
serde = \"1\"
";
        assert_eq!(declared_features(manifest), vec!["pgp", "yara"]);
    }

    #[test]
    fn declared_features_ignores_other_sections() {
        let manifest = "[dependencies]\nnot_a_feature = \"1\"\n";
        assert!(declared_features(manifest).is_empty());
    }
}
