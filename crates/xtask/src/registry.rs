use regex::Regex;
use std::{
    collections::HashSet,
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

const HELPER_MODULES: &[&str] = &["pgp", "protobuf_schema", "test_x509", "gost_mac"];

pub fn generate(check: bool) -> Result<(), String> {
    let workspace =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").map_err(|e| e.to_string())?).join("../..");
    let operations = workspace.join("src/operations");
    let committed = operations.join("mod.rs");
    let generated = render(&operations)?;

    if check {
        let temporary = workspace.join("target/xtask/operations-mod.rs");
        if let Some(parent) = temporary.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(&temporary, generated).map_err(|e| e.to_string())?;
        rustfmt(&temporary)?;
        let expected = fs::read(&temporary).map_err(|e| e.to_string())?;
        let actual = fs::read(&committed)
            .map_err(|e| format!("cannot read {}: {e}", committed.display()))?;
        if actual != expected {
            return Err(format!(
                "{} is stale; run `cargo xtask generate-registry`",
                committed.display()
            ));
        }
        println!("operation registry is current");
    } else {
        fs::write(&committed, generated).map_err(|e| e.to_string())?;
        rustfmt(&committed)?;
        println!("generated {}", committed.display());
    }
    Ok(())
}

fn render(operations: &Path) -> Result<String, String> {
    let mut entries = fs::read_dir(operations)
        .map_err(|e| format!("cannot read {}: {e}", operations.display()))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|value| value.to_str()) == Some("rs"))
        .filter(|path| path.file_name().and_then(|value| value.to_str()) != Some("mod.rs"))
        .collect::<Vec<_>>();
    entries.sort_by(|left, right| left.file_name().cmp(&right.file_name()));

    let unit_struct = Regex::new(r"pub struct (\w+);").map_err(|e| e.to_string())?;
    let mut modules = HashSet::new();
    let mut operations_found = Vec::new();
    let mut declarations = String::new();

    for path in entries {
        let file = path
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(|| format!("non-UTF-8 operation filename: {}", path.display()))?;
        let base = file.trim_end_matches(".rs");
        let module = if base == "return" {
            "return_rs".to_string()
        } else {
            safe_identifier(base)
        };
        if !modules.insert(module.clone()) {
            return Err(format!(
                "operation module identifier collision at '{}'",
                module
            ));
        }
        if module == base {
            declarations.push_str(&format!("pub mod {module};\n"));
        } else {
            declarations.push_str(&format!("pub mod {module} {{ include!(\"{file}\"); }}\n"));
        }

        let source = fs::read_to_string(&path)
            .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        let structs = unit_struct
            .captures_iter(&source)
            .map(|capture| capture[1].to_string())
            .collect::<Vec<_>>();
        let structure = match structs.as_slice() {
            [structure] => structure.clone(),
            [] if HELPER_MODULES.contains(&base) => continue,
            [] => {
                return Err(format!(
                    "{} has no public unit operation implementation; add it to the explicit helper allowlist if it is not an operation",
                    path.display()
                ));
            }
            _ => {
                return Err(format!(
                    "{} has multiple public unit structs; registry entry is ambiguous",
                    path.display()
                ));
            }
        };
        operations_found.push((module, structure));
    }

    let mut output = String::from(
        "/*\n * -----------------------------------------------------------------------------\n * Project:     rxchef\n * Author:      Michael Weiss\n * Source:      Generated operations registry for rxchef\n * License:     Apache-2.0\n * Description: Auto-generated registry of rxchef operations.\n * -----------------------------------------------------------------------------\n */\n\n#[allow(dead_code)]\nuse crate::operation::Operation;\n\n",
    );
    output.push_str(&declarations);
    output.push_str("\npub fn operation_names() -> Vec<String> {\n    let mut names: Vec<String> = Vec::new();\n");
    for (module, structure) in &operations_found {
        output.push_str(&format!(
            "    names.push({module}::{structure}.name().to_string());\n"
        ));
    }
    output.push_str("    names.sort();\n    names\n}\n");
    output.push_str(
        "\n/// Returns an operation by its name.\npub fn get_operation(name: &str) -> Option<Box<dyn Operation>> {\n    let lowered = name.to_lowercase();\n",
    );
    for (module, structure) in &operations_found {
        output.push_str(&format!(
            "    {{ let op = {module}::{structure}; if op.name().to_lowercase() == lowered {{ return Some(Box::new({module}::{structure})); }} }}\n"
        ));
    }
    output.push_str("    None\n}\n");
    output.push_str(
        "\n/// Returns the source module identifier for an operation.\npub fn operation_source(name: &str) -> Option<&'static str> {\n    let lowered = name.to_lowercase();\n",
    );
    for (module, structure) in &operations_found {
        output.push_str(&format!(
            "    {{ let op = {module}::{structure}; if op.name().to_lowercase() == lowered {{ return Some(\"{module}\"); }} }}\n"
        ));
    }
    output.push_str("    None\n}\n");
    Ok(output)
}

fn safe_identifier(value: &str) -> String {
    let mut result = String::new();
    for (index, character) in value.chars().enumerate() {
        let valid = if index == 0 {
            character.is_ascii_alphabetic() || character == '_'
        } else {
            character.is_ascii_alphanumeric() || character == '_'
        };
        if valid {
            result.push(character);
        } else if character.is_ascii() {
            result.push('_');
        } else {
            result.push(match character {
                'è' | 'é' | 'ê' | 'ë' => 'e',
                'ò' | 'ó' | 'ô' | 'ö' => 'o',
                _ => '_',
            });
        }
    }
    if result.is_empty() || result.starts_with(|character: char| character.is_ascii_digit()) {
        result.insert(0, '_');
    }
    result
}

fn rustfmt(path: &Path) -> Result<(), String> {
    let rustfmt = env::var_os("RUSTFMT").unwrap_or_else(|| "rustfmt".into());
    let status = Command::new(rustfmt)
        .args(["--edition", "2021", "--config", "skip_children=true"])
        .arg(path)
        .status()
        .map_err(|e| format!("cannot run rustfmt: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err("rustfmt failed for generated registry".into())
    }
}
