/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Build-time generator for the operations registry.
 * -----------------------------------------------------------------------------
 */

use std::{
    collections::HashSet,
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

use regex::Regex;

fn is_valid_ident(ch: char, first: bool) -> bool {
    if first {
        ch.is_ascii_alphabetic() || ch == '_'
    } else {
        ch.is_ascii_alphanumeric() || ch == '_'
    }
}

fn make_safe_ident(s: &str) -> String {
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if is_valid_ident(ch, i == 0) {
            out.push(ch);
        } else if ch.is_ascii() {
            out.push('_');
        } else {
            match ch {
                '\u{00E8}' | '\u{00E9}' | '\u{00EA}' | '\u{00EB}' => out.push('e'),
                '\u{00F2}' | '\u{00F3}' | '\u{00F4}' | '\u{00F6}' => out.push('o'),
                _ => out.push('_'),
            }
        }
    }
    if out.is_empty() || out.chars().next().unwrap().is_numeric() {
        out = format!("_{}", out);
    }
    out
}

fn main() {
    let ops_dir = Path::new("src/operations");
    let mut entries: Vec<_> = fs::read_dir(ops_dir)
        .expect("read_dir operations")
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let p = e.path();
            if p.is_file() {
                if let Some(ext) = p.extension() {
                    if ext == "rs" {
                        return Some(p);
                    }
                }
            }
            None
        })
        .collect();

    entries.sort_by_key(|p| p.file_name().map(|s| s.to_os_string()));

    let mut seen_modules: HashSet<String> = HashSet::new();
    let mut struct_list: Vec<(String, String)> = Vec::new(); // (safe_module, struct_name)

    let struct_regex = Regex::new(r"pub struct (\w+);").unwrap();

    let mut out = String::new();
    out.push_str("/*\n");
    out.push_str(
        " * -----------------------------------------------------------------------------\n",
    );
    out.push_str(" * Project:     rxchef\n");
    out.push_str(" * Version:     1.0.0\n");
    out.push_str(" * Author:      Michael Weiss\n");
    out.push_str(" * Source:      Generated operations registry for rxchef\n");
    out.push_str(" * License:     Apache-2.0\n");
    out.push_str(" * Description: Auto-generated registry of rxchef operations.\n");
    out.push_str(
        " * -----------------------------------------------------------------------------\n",
    );
    out.push_str(" */\n\n#[allow(dead_code)]\n\n");
    out.push_str("use crate::operation::Operation;\n\n");

    for path in entries {
        let fname = path.file_name().unwrap().to_string_lossy().into_owned();
        if fname == "mod.rs" {
            continue;
        }
        if let Some(base) = fname.strip_suffix(".rs") {
            let safe = match base {
                "return" => "return_rs".to_string(),
                other => make_safe_ident(other),
            };
            if seen_modules.contains(&safe) {
                continue;
            }
            seen_modules.insert(safe.clone());

            if safe != base {
                out.push_str(&format!(
                    "pub mod {} {{ include!(\"{}\"); }}\n",
                    safe, fname
                ));
            } else {
                out.push_str(&format!("pub mod {};\n", safe));
            }

            // Extract struct name for registration (unit structs)
            let mut content = String::new();
            if let Ok(mut f) = File::open(&path) {
                if f.read_to_string(&mut content).is_ok() {
                    if let Some(struct_cap) = struct_regex.captures(&content) {
                        let struct_name = struct_cap[1].to_string();
                        struct_list.push((safe.clone(), struct_name));
                    }
                }
            }
        }
    }

    // Generate a runtime list of operation names by instantiating each unit struct.
    out.push_str("\npub fn operation_names() -> Vec<String> {\n");
    out.push_str("    let mut names: Vec<String> = Vec::new();\n");
    for (safe, struct_name) in &struct_list {
        out.push_str(&format!(
            "    names.push({}::{}{}.name().to_string());\n",
            safe, struct_name, ""
        ));
    }
    out.push_str("    names.sort();\n");
    out.push_str("    names\n");
    out.push_str("}\n");

    out.push_str("\n/// Returns an operation by its name.\n");
    out.push_str("pub fn get_operation(name: &str) -> Option<Box<dyn Operation>> {\n");
    out.push_str("    let lowered = name.to_lowercase();\n");
    for (safe, struct_name) in &struct_list {
        out.push_str(&format!(
            "    {{ let op = {}::{}; if op.name().to_lowercase() == lowered {{ return Some(Box::new({}::{})); }} }}\n",
            safe, struct_name, safe, struct_name
        ));
    }
    out.push_str("    None\n");
    out.push_str("}\n");

    let dst = ops_dir.join("mod.rs");
    let mut f = File::create(&dst).expect("create mod.rs");
    f.write_all(out.as_bytes()).expect("write mod.rs");
}
