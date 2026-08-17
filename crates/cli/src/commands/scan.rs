//! `rxchef scan` — find and decode encoded strings in files or streams.

use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

use crate::cli::ScanArgs;
use crate::commands::magic::format_recipe;

pub(crate) fn cmd_scan(a: ScanArgs) -> Result<(), String> {
    use rxchef::scan::{ScanOptions, Scanner};

    let crib = match &a.crib {
        Some(c) => {
            Some(regex::Regex::new(c).map_err(|e| format!("invalid crib regex '{c}': {e}"))?)
        }
        None => None,
    };
    let opts = ScanOptions {
        min_len: a.min_len,
        max_len: a.max_token_size,
        decode: a.decode,
        depth: a.depth,
        crib,
        min_entropy: a.entropy.unwrap_or(0.0),
        only_kinds: a.kind.clone(),
        max_findings: a.max_findings,
    };

    let stdout = io::stdout();
    let mut out = stdout.lock();
    let total = std::cell::Cell::new(0usize);

    // Collect the source list: explicit paths (walked) or stdin.
    let mut sources: Vec<Option<PathBuf>> = Vec::new();
    if a.paths.is_empty() {
        sources.push(None); // stdin
    } else {
        for p in &a.paths {
            collect_paths(p, a.recursive, &mut sources)?;
        }
    }

    'sources: for src in sources {
        let label = src
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "<stdin>".to_string());

        let reader: Box<dyn Read> = match &src {
            Some(p) => Box::new(
                fs::File::open(p).map_err(|e| format!("cannot open '{}': {e}", p.display()))?,
            ),
            None => Box::new(io::stdin().lock()),
        };
        let mut reader = io::BufReader::with_capacity(64 * 1024, reader);

        let mut scanner = Scanner::new(opts.clone());
        let mut buf = vec![0u8; 64 * 1024];
        let mut emit = |f: rxchef::scan::Finding| {
            total.set(total.get() + 1);
            print_finding(&mut out, &label, &f, a.json);
        };
        loop {
            let n = reader
                .read(&mut buf)
                .map_err(|e| format!("read error on '{label}': {e}"))?;
            if n == 0 {
                break;
            }
            scanner.push(&buf[..n], &mut emit);
            if scanner.limit_reached() || total.get() >= a.max_findings {
                break 'sources;
            }
        }
        scanner.finish(&mut emit);
        if scanner.limit_reached() || total.get() >= a.max_findings {
            break;
        }
    }

    eprintln!("\n{} finding(s)", total.get());
    Ok(())
}

/// Expand a path into a list of files to scan, walking directories.
pub(crate) fn collect_paths(
    path: &std::path::Path,
    recursive: bool,
    out: &mut Vec<Option<PathBuf>>,
) -> Result<(), String> {
    let meta = fs::metadata(path).map_err(|e| format!("cannot stat '{}': {e}", path.display()))?;
    if meta.is_dir() {
        let entries =
            fs::read_dir(path).map_err(|e| format!("cannot read dir '{}': {e}", path.display()))?;
        let mut children: Vec<PathBuf> = entries.filter_map(|e| e.ok().map(|e| e.path())).collect();
        children.sort();
        for child in children {
            if child.is_dir() {
                if recursive {
                    collect_paths(&child, recursive, out)?;
                }
            } else {
                out.push(Some(child));
            }
        }
    } else {
        out.push(Some(path.to_path_buf()));
    }
    Ok(())
}

pub(crate) fn print_finding<W: Write>(
    w: &mut W,
    file: &str,
    f: &rxchef::scan::Finding,
    json: bool,
) {
    if json {
        let mut v = serde_json::to_value(f).unwrap_or(serde_json::json!({}));
        v["file"] = serde_json::json!(file);
        let _ = writeln!(w, "{}", serde_json::to_string(&v).unwrap());
    } else {
        let kinds = if f.kinds.is_empty() {
            "high-entropy".to_string()
        } else {
            f.kinds.join(",")
        };
        let _ = writeln!(
            w,
            "{}:{}  [{}]  entropy {:.2}  len {}",
            file, f.offset, kinds, f.entropy, f.len
        );
        let _ = writeln!(w, "    token:  {}", f.token);
        if let Some(dec) = &f.decoded {
            let recipe = f
                .recipe
                .as_ref()
                .map(|r| format_recipe(r))
                .unwrap_or_default();
            let _ = writeln!(w, "    decode: {}  [{}]", dec, recipe);
        }
    }
}

// ─── Pipeline execution helpers ───────────────────────────────────────────────
