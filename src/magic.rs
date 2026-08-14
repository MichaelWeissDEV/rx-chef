/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * License:     Apache-2.0
 * Description: Magic — recursive detect-and-decode engine.
 *
 * This is the single source of truth for input analysis in rxchef. The CLI
 * `magic` / `scan` commands, the `Magic` operation, the TUI and the FFI all
 * delegate here so there is exactly one detection/scoring implementation.
 *
 * The engine is a thin orchestrator: it never contains its own base64/hex/gzip
 * decoders. It fires cheap heuristics to decide which *existing* operations
 * (looked up through `operations::get_operation`) are worth trying, runs them,
 * scores the result, and recurses — pruning branches that do not improve.
 * -----------------------------------------------------------------------------
 */

use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use regex::Regex;
use serde::Serialize;

use crate::execution;

/// One decode step in a recovered recipe.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct RecipeStep {
    pub op: String,
    pub args: Vec<String>,
}

/// A candidate decoding discovered by the engine.
#[derive(Debug, Clone, Serialize)]
pub struct MagicMatch {
    /// Ordered operations that transform the original input into `data`.
    pub recipe: Vec<RecipeStep>,
    /// The decoded bytes (skipped in JSON; use `preview`).
    #[serde(skip)]
    pub data: Vec<u8>,
    /// Human-readable, lossy preview of `data` (truncated).
    pub preview: String,
    /// Shannon entropy of `data` (0-8).
    pub entropy: f64,
    /// Fraction of `data` that is printable ASCII / common whitespace (0-1).
    pub printable_ratio: f64,
    /// Whether `data` is valid UTF-8.
    pub valid_utf8: bool,
    /// Whether `data` matched the user-supplied crib.
    pub matched_crib: bool,
    /// Heuristic ranking score (higher = more likely meaningful).
    pub score: f64,
}

/// Tuning knobs for the recursive engine.
#[derive(Debug, Clone)]
pub struct MagicOptions {
    /// Maximum recursion depth (number of chained decodes).
    pub depth: usize,
    /// Optional known-plaintext filter (substring or regex).
    pub crib: Option<Regex>,
    /// Try more aggressive/expensive decoders (ROT, charcode, base58/85).
    pub intensive: bool,
    /// Maximum number of ranked matches to return.
    pub max_results: usize,
    /// Maximum decoder attempts across the complete search tree.
    pub max_candidates: usize,
    /// Maximum input and output size of an individual candidate.
    pub max_candidate_bytes: usize,
    /// Maximum cumulative decoded bytes accepted across all candidates.
    pub max_total_decoded_bytes: usize,
}

impl Default for MagicOptions {
    fn default() -> Self {
        MagicOptions {
            depth: 3,
            crib: None,
            intensive: false,
            max_results: 20,
            max_candidates: 256,
            max_candidate_bytes: 8 << 20,
            max_total_decoded_bytes: 32 << 20,
        }
    }
}

/// A decoder the engine may try, plus the operation arguments to run it with.
struct Candidate {
    op: &'static str,
    args: Vec<String>,
}

// ─── Public API ────────────────────────────────────────────────────────────────

/// Run the recursive detect-and-decode engine and return ranked candidate
/// decodings, best first. The original (undecoded) input is never returned as
/// a match; only the results of one or more successful decode steps are.
pub fn magic(input: &[u8], opts: &MagicOptions) -> Vec<MagicMatch> {
    let mut matches: Vec<MagicMatch> = Vec::new();
    let mut visited: HashSet<u64> = HashSet::new();
    visited.insert(hash_bytes(input));

    let start = Metrics::of(input);
    let mut budget = MagicBudget::default();
    recurse(
        input,
        &start,
        &[],
        opts,
        0,
        &mut budget,
        &mut visited,
        &mut matches,
    );

    matches.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    matches.truncate(opts.max_results);
    matches
}

/// Single-level detection: which decoders *would* fire on this input, without
/// actually decoding or recursing. Useful for quick "what is this?" hints and
/// for the scanner to label findings.
pub fn detect(input: &[u8]) -> Vec<String> {
    candidates(input, false)
        .into_iter()
        .map(|c| c.op.to_string())
        .collect()
}

/// Shannon entropy of `data`, in bits per byte (0-8).
pub fn shannon_entropy(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let mut counts = [0usize; 256];
    for &b in data {
        counts[b as usize] += 1;
    }
    let len = data.len() as f64;
    let mut entropy = 0.0;
    for &count in counts.iter() {
        if count > 0 {
            let p = count as f64 / len;
            entropy -= p * p.log2();
        }
    }
    entropy
}

// ─── Recursion ─────────────────────────────────────────────────────────────────

#[derive(Default)]
struct MagicBudget {
    candidates: usize,
    total_decoded_bytes: usize,
}

fn recurse(
    data: &[u8],
    metrics: &Metrics,
    recipe: &[RecipeStep],
    opts: &MagicOptions,
    depth: usize,
    budget: &mut MagicBudget,
    visited: &mut HashSet<u64>,
    out: &mut Vec<MagicMatch>,
) {
    if depth >= opts.depth
        || data.len() > opts.max_candidate_bytes
        || budget.candidates >= opts.max_candidates
    {
        return;
    }
    for cand in candidates(data, opts.intensive) {
        if budget.candidates >= opts.max_candidates {
            break;
        }
        budget.candidates += 1;
        let decoded = match execution::execute(execution::ExecutionRequest {
            input: data.to_vec(),
            input_supplied: true,
            recipe: execution::Recipe::from(vec![execution::RecipeStep {
                op: cand.op.to_string(),
                args: cand.args.clone(),
            }]),
            variables: execution::VariableContext::default(),
            options: execution::ExecutionOptions {
                max_steps: 1,
                max_output_bytes: Some(opts.max_candidate_bytes),
                ..execution::ExecutionOptions::default()
            },
        }) {
            Ok(outcome) => outcome.output,
            Err(_) => continue,
        };
        // Reject useless decodes: empty, unchanged, or already-seen output.
        if decoded.is_empty() || decoded.as_slice() == data {
            continue;
        }
        let h = hash_bytes(&decoded);
        if !visited.insert(h) {
            continue;
        }
        let Some(total_decoded_bytes) = budget.total_decoded_bytes.checked_add(decoded.len())
        else {
            return;
        };
        if total_decoded_bytes > opts.max_total_decoded_bytes {
            return;
        }
        budget.total_decoded_bytes = total_decoded_bytes;

        let child = Metrics::of(&decoded);
        let matched_crib = opts
            .crib
            .as_ref()
            .map(|re| re.is_match(&String::from_utf8_lossy(&decoded)))
            .unwrap_or(false);

        let mut child_recipe = recipe.to_vec();
        child_recipe.push(RecipeStep {
            op: cand.op.to_string(),
            args: cand.args.clone(),
        });

        out.push(MagicMatch {
            recipe: child_recipe.clone(),
            preview: preview(&decoded, 200),
            entropy: child.entropy,
            printable_ratio: child.printable_ratio,
            valid_utf8: child.valid_utf8,
            matched_crib,
            score: score(&child, matched_crib, depth + 1),
            data: decoded.clone(),
        });

        // Only recurse into branches that keep looking like meaningful data:
        // printability must not collapse, or the branch must match the crib, or
        // entropy must drop notably (a sign of a successful decode). This is
        // what keeps recursive decoding from exploding into noise.
        let improved = child.printable_ratio + 1e-9 >= metrics.printable_ratio
            || child.entropy + 0.5 < metrics.entropy
            || matched_crib;
        if improved {
            recurse(
                &decoded,
                &child,
                &child_recipe,
                opts,
                depth + 1,
                budget,
                visited,
                out,
            );
        }
    }
}

// ─── Metrics & scoring ──────────────────────────────────────────────────────────

struct Metrics {
    entropy: f64,
    printable_ratio: f64,
    /// Fraction of bytes that are ASCII letters or spaces (natural-language signal).
    text_ratio: f64,
    valid_utf8: bool,
    data_len: usize,
}

impl Metrics {
    fn of(data: &[u8]) -> Metrics {
        let len = data.len().max(1) as f64;
        let mut printable = 0usize;
        let mut texty = 0usize;
        for &b in data {
            if (0x20..=0x7e).contains(&b) || matches!(b, b'\n' | b'\r' | b'\t') {
                printable += 1;
            }
            if b.is_ascii_alphabetic() || b == b' ' {
                texty += 1;
            }
        }
        Metrics {
            entropy: shannon_entropy(data),
            printable_ratio: printable as f64 / len,
            text_ratio: texty as f64 / len,
            valid_utf8: std::str::from_utf8(data).is_ok(),
            data_len: data.len(),
        }
    }
}

/// Heuristic score. Higher is more likely to be the intended plaintext.
fn score(m: &Metrics, matched_crib: bool, depth: usize) -> f64 {
    let mut s = 0.0;
    if matched_crib {
        s += 1000.0;
    }
    s += m.printable_ratio * 30.0;
    if m.valid_utf8 {
        s += 8.0;
    }
    s += m.text_ratio * 15.0;
    // Natural-language byte entropy sits around 3.5-5 bits; reward proximity to
    // ~4 and penalise high-entropy (still-encrypted / compressed) output.
    s += 5.0 - (m.entropy - 4.0).abs();
    // Reward reaching a deeper fully-decoded leaf (each hop that survived the
    // printability prune is evidence of a real layer).
    s += depth as f64 * 3.0;
    // Trivially short outputs are usually decode artifacts, not the payload.
    if m.data_len < 3 {
        s -= 20.0;
    }
    s
}

// ─── Candidate detection ─────────────────────────────────────────────────────────

/// Decide which decoders are worth trying on `data`. Ordering here is only a
/// hint; final ranking is done by `score`.
fn candidates(data: &[u8], intensive: bool) -> Vec<Candidate> {
    let mut out = Vec::new();
    if data.is_empty() {
        return out;
    }

    // --- Compression / binary formats (magic bytes) ---
    if data.starts_with(&[0x1f, 0x8b]) {
        out.push(Candidate {
            op: "Gunzip",
            args: vec![],
        });
    }
    if data.len() >= 2 && data[0] == 0x78 && matches!(data[1], 0x01 | 0x5e | 0x9c | 0xda) {
        out.push(Candidate {
            op: "Zlib Inflate",
            args: vec![],
        });
    }

    // --- Structured tokens (very specific, checked first) ---
    if looks_jwt(data) {
        out.push(Candidate {
            op: "JWT Decode",
            args: vec![],
        });
    }

    // --- Text-shaped encodings ---
    let non_ws: Vec<u8> = data
        .iter()
        .copied()
        .filter(|b| !b.is_ascii_whitespace())
        .collect();

    if looks_hex(&non_ws) {
        out.push(Candidate {
            op: "From Hex",
            args: vec!["Auto".into()],
        });
    }
    if looks_base64(&non_ws) {
        out.push(Candidate {
            op: "From Base64",
            args: vec!["A-Za-z0-9+/=".into(), "true".into(), "false".into()],
        });
    }
    if looks_base64url(&non_ws) {
        out.push(Candidate {
            op: "From Base64",
            args: vec!["A-Za-z0-9-_".into(), "true".into(), "false".into()],
        });
    }
    if looks_base32(&non_ws) {
        out.push(Candidate {
            op: "From Base32",
            args: vec!["A-Z2-7=".into(), "true".into()],
        });
    }
    if looks_url_encoded(data) {
        out.push(Candidate {
            op: "URL Decode",
            args: vec!["true".into()],
        });
    }
    if looks_quoted_printable(data) {
        out.push(Candidate {
            op: "From Quoted Printable",
            args: vec![],
        });
    }
    if looks_binary_text(&non_ws) {
        out.push(Candidate {
            op: "From Binary",
            args: vec!["Space".into(), "8".into()],
        });
    }
    if looks_morse(data) {
        out.push(Candidate {
            op: "From Morse Code",
            args: vec!["Space".into(), "Line feed".into()],
        });
    }

    // --- Intensive / brute-force decoders (opt-in; they fire broadly) ---
    if intensive {
        if looks_base58(&non_ws) {
            out.push(Candidate {
                op: "From Base58",
                args: vec![
                    "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".into(),
                    "true".into(),
                ],
            });
        }
        if looks_base85(&non_ws) {
            out.push(Candidate {
                op: "From Base85",
                args: vec!["!-u".into(), "true".into(), "z".into()],
            });
        }
        if data.iter().all(|&b| b.is_ascii()) {
            out.push(Candidate {
                op: "ROT13",
                args: vec!["true".into(), "true".into(), "13".into()],
            });
        }
    }

    out
}

// ─── Firing heuristics ───────────────────────────────────────────────────────────

fn looks_hex(non_ws: &[u8]) -> bool {
    non_ws.len() >= 4
        && non_ws.len() % 2 == 0
        && non_ws.iter().all(|b| b.is_ascii_hexdigit())
        // Pure-decimal strings are hex-valid but almost never hex; skip them so
        // "12345678" isn't reported as hex bytes.
        && non_ws.iter().any(|b| !b.is_ascii_digit())
}

fn looks_base64(non_ws: &[u8]) -> bool {
    if non_ws.len() < 8 {
        return false;
    }
    let all_ok = non_ws
        .iter()
        .all(|&b| b.is_ascii_alphanumeric() || b == b'+' || b == b'/' || b == b'=');
    // Valid base64 length (with or without padding) is never `% 4 == 1`, and
    // there must be at least one char outside the hex alphabet — otherwise the
    // input is better classified as hex.
    all_ok
        && non_ws.len() % 4 != 1
        && non_ws
            .iter()
            .any(|b| matches!(b, b'g'..=b'z' | b'G'..=b'Z' | b'+' | b'/' | b'='))
}

/// URL-safe base64: uses `-`/`_` instead of `+`/`/`. Requires at least one of
/// those two chars so plain base64 stays classified as plain base64.
fn looks_base64url(non_ws: &[u8]) -> bool {
    if non_ws.len() < 8 {
        return false;
    }
    let all_ok = non_ws
        .iter()
        .all(|&b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'=');
    let has_urlsafe = non_ws.iter().any(|&b| b == b'-' || b == b'_');
    all_ok && has_urlsafe && non_ws.len() % 4 != 1
}

/// JSON Web Token: three `.`-separated base64url segments whose header segment
/// begins with `eyJ` — the base64url encoding of `{"`, which every JWT header
/// starts with. This is specific enough to almost never false-positive.
fn looks_jwt(data: &[u8]) -> bool {
    let s = match std::str::from_utf8(data) {
        Ok(s) => s.trim(),
        Err(_) => return false,
    };
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    if !parts[0].starts_with("eyJ") {
        return false;
    }
    let is_b64url = |p: &str| {
        p.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
    };
    !parts[0].is_empty()
        && !parts[1].is_empty()
        && is_b64url(parts[0])
        && is_b64url(parts[1])
        && is_b64url(parts[2])
}

fn looks_base32(non_ws: &[u8]) -> bool {
    if non_ws.len() < 8 {
        return false;
    }
    let all_ok = non_ws
        .iter()
        .all(|&b| matches!(b, b'A'..=b'Z' | b'2'..=b'7' | b'='));
    let has_letter = non_ws.iter().any(|b| b.is_ascii_uppercase());
    all_ok && has_letter
}

fn looks_base58(non_ws: &[u8]) -> bool {
    const ALPHA: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    non_ws.len() >= 8 && non_ws.iter().all(|b| ALPHA.contains(b))
}

fn looks_base85(non_ws: &[u8]) -> bool {
    non_ws.len() >= 8 && non_ws.iter().all(|&b| (b'!'..=b'u').contains(&b))
}

fn looks_url_encoded(data: &[u8]) -> bool {
    // At least one %XX escape.
    data.windows(3)
        .any(|w| w[0] == b'%' && w[1].is_ascii_hexdigit() && w[2].is_ascii_hexdigit())
}

fn looks_quoted_printable(data: &[u8]) -> bool {
    data.windows(3)
        .any(|w| w[0] == b'=' && w[1].is_ascii_hexdigit() && w[2].is_ascii_hexdigit())
}

fn looks_binary_text(non_ws: &[u8]) -> bool {
    non_ws.len() >= 8 && non_ws.len() % 8 == 0 && non_ws.iter().all(|&b| b == b'0' || b == b'1')
}

fn looks_morse(data: &[u8]) -> bool {
    let relevant: Vec<u8> = data
        .iter()
        .copied()
        .filter(|b| !b.is_ascii_whitespace())
        .collect();
    !relevant.is_empty()
        && relevant.iter().all(|&b| matches!(b, b'.' | b'-' | b'/'))
        && relevant.iter().any(|&b| b == b'.' || b == b'-')
}

// ─── Helpers ─────────────────────────────────────────────────────────────────────

fn hash_bytes(data: &[u8]) -> u64 {
    let mut h = DefaultHasher::new();
    data.hash(&mut h);
    h.finish()
}

/// Lossy, single-line, length-capped preview of arbitrary bytes.
pub fn preview(data: &[u8], max: usize) -> String {
    let text = String::from_utf8_lossy(data);
    let mut s: String = text
        .chars()
        .map(|c| {
            if c == '\n' || c == '\r' || c == '\t' {
                ' '
            } else if c.is_control() {
                '.'
            } else {
                c
            }
        })
        .take(max)
        .collect();
    if text.chars().count() > max {
        s.push('…');
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_base64_decodes() {
        let m = magic(b"SGVsbG8gV29ybGQ=", &MagicOptions::default());
        assert!(
            m.iter().any(|x| x.data == b"Hello World"),
            "expected to recover 'Hello World'"
        );
    }

    #[test]
    fn double_base64_recovers_plaintext() {
        // base64(base64("Hello")) = base64("SGVsbG8=") = "U0dWc2JHOD0="
        let m = magic(b"U0dWc2JHOD0=", &MagicOptions::default());
        assert!(
            m.iter().any(|x| x.data == b"Hello"),
            "expected to recover 'Hello' through two base64 layers"
        );
    }

    #[test]
    fn crib_ranks_match_first() {
        let opts = MagicOptions {
            crib: Some(Regex::new("Hello").unwrap()),
            ..Default::default()
        };
        let m = magic(b"U0dWc2JHOD0=", &opts);
        assert_eq!(m[0].data, b"Hello", "crib match should rank first");
    }

    #[test]
    fn hex_is_detected() {
        let m = magic(b"48656c6c6f", &MagicOptions::default());
        assert!(m.iter().any(|x| x.data == b"Hello"));
    }

    #[test]
    fn pure_decimal_is_not_hex() {
        // Should not be misread as hex bytes.
        assert!(!looks_hex(b"12345678"));
    }

    #[test]
    fn empty_input_no_matches() {
        assert!(magic(b"", &MagicOptions::default()).is_empty());
    }

    #[test]
    fn resource_budgets_stop_candidate_expansion() {
        let input = b"U0dWc2JHOD0=";
        let no_candidates = MagicOptions {
            max_candidates: 0,
            ..MagicOptions::default()
        };
        assert!(magic(input, &no_candidates).is_empty());

        let too_small = MagicOptions {
            max_candidate_bytes: input.len() - 1,
            ..MagicOptions::default()
        };
        assert!(magic(input, &too_small).is_empty());

        let no_decoded_budget = MagicOptions {
            max_total_decoded_bytes: 0,
            ..MagicOptions::default()
        };
        assert!(magic(input, &no_decoded_budget).is_empty());
    }

    #[test]
    fn jwt_is_decoded() {
        let jwt = b"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
        let opts = MagicOptions {
            crib: Some(Regex::new("John Doe").unwrap()),
            ..Default::default()
        };
        let m = magic(jwt, &opts);
        assert!(
            m.first().map(|x| x.matched_crib).unwrap_or(false),
            "expected JWT Decode to surface the payload containing 'John Doe'"
        );
    }

    #[test]
    fn base64url_is_detected() {
        // base64url of bytes that use the '-'/'_' chars (0xFB 0xFF 0xBF).
        assert!(looks_base64url(b"-_-_abcd"));
        // plain base64 (no '-'/'_') must NOT be treated as base64url.
        assert!(!looks_base64url(b"SGVsbG8gV29ybGQ"));
    }

    #[test]
    fn jwt_structure_rejects_non_jwt() {
        assert!(!looks_jwt(b"192.168.1.1"));
        assert!(!looks_jwt(b"file.tar.gz"));
        assert!(!looks_jwt(b"not.a.jwt"));
    }
}
