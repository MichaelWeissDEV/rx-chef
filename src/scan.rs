/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * License:     Apache-2.0
 * Description: Streaming scanner — find (and optionally decode) encoded or
 *              high-entropy strings inside large byte streams.
 *
 * Built for GB-scale inputs (PCAPs, RAM dumps): data is fed in chunks and a
 * token that straddles a chunk boundary is stitched across pushes, so nothing
 * is ever fully buffered in memory. Detection/decoding is delegated to the
 * `magic` engine so there is one detection implementation.
 * -----------------------------------------------------------------------------
 */

use regex::Regex;
use serde::Serialize;

use crate::magic::{self, MagicOptions, RecipeStep};

/// A single hit produced by the scanner.
#[derive(Debug, Clone, Serialize)]
pub struct Finding {
    /// Byte offset of the token in the original stream.
    pub offset: u64,
    /// Length of the matched token in bytes.
    pub len: usize,
    /// Encodings that fired on the token (e.g. `["From Base64"]`).
    pub kinds: Vec<String>,
    /// Shannon entropy of the token bytes.
    pub entropy: f64,
    /// The matched token text (length-capped preview).
    pub token: String,
    /// Best decoded preview, when `decode` was requested and a decode succeeded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoded: Option<String>,
    /// Recipe that produced `decoded`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe: Option<Vec<RecipeStep>>,
}

/// Scanner configuration.
#[derive(Debug, Clone)]
pub struct ScanOptions {
    /// Minimum token length to consider (bytes).
    pub min_len: usize,
    /// Cap token length; longer runs are truncated to avoid huge allocations.
    pub max_len: usize,
    /// Attempt to decode each candidate with the magic engine.
    pub decode: bool,
    /// Recursion depth passed to the magic engine when decoding.
    pub depth: usize,
    /// Only report findings whose decode matches this crib (implies `decode`).
    pub crib: Option<Regex>,
    /// Also report any token whose entropy is at least this value, even if no
    /// decoder fired (0 disables the entropy filter).
    pub min_entropy: f64,
    /// Restrict reported findings to these decoders (matched against `kinds`).
    /// Empty means "any".
    pub only_kinds: Vec<String>,
    /// Stop emitting after this many findings.
    pub max_findings: usize,
}

impl Default for ScanOptions {
    fn default() -> Self {
        ScanOptions {
            min_len: 16,
            max_len: 1 << 20, // 1 MiB
            decode: false,
            depth: 3,
            crib: None,
            min_entropy: 0.0,
            only_kinds: Vec::new(),
            max_findings: 10_000,
        }
    }
}

/// Characters that may appear inside an encoded token: the base64, base64url,
/// base32 and hex alphabets (`-`/`_` included so URL-safe base64 and JWT
/// segments survive). `=` (padding) is treated as a delimiter, not a token
/// byte — otherwise `key=SGVsbG8=` glues the key onto the payload and the whole
/// run decodes as nothing. Base64 decoders tolerate the missing padding, so
/// dropping `=` loses no real data. `.` is intentionally *not* a token byte:
/// gluing it would break base64 adjacent to filenames, so JWT segments are
/// caught individually (each `eyJ…` part is base64 and decodes to its JSON).
#[inline]
fn is_token_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || matches!(b, b'+' | b'/' | b'-' | b'_')
}

/// Streaming, boundary-aware token scanner.
///
/// Feed bytes with [`Scanner::push`] (repeatedly, in any chunk sizes) and end
/// with [`Scanner::finish`]. Completed tokens are delivered to the callback as
/// they are recognised.
pub struct Scanner {
    opts: ScanOptions,
    /// Bytes of the run currently being accumulated across pushes.
    pending: Vec<u8>,
    /// Absolute stream offset where `pending` began.
    pending_start: u64,
    /// Absolute offset of the next byte to be consumed.
    pos: u64,
    /// True once `pending` hit `max_len` and further run bytes are dropped.
    truncated: bool,
    findings: usize,
}

impl Scanner {
    pub fn new(opts: ScanOptions) -> Self {
        Scanner {
            opts,
            pending: Vec::new(),
            pending_start: 0,
            pos: 0,
            truncated: false,
            findings: 0,
        }
    }

    /// Whether the configured finding budget has been exhausted.
    pub fn limit_reached(&self) -> bool {
        self.findings >= self.opts.max_findings
    }

    /// Feed a chunk. `emit` is called for every completed token that passes the
    /// configured filters.
    pub fn push<F: FnMut(Finding)>(&mut self, chunk: &[u8], emit: &mut F) {
        if self.limit_reached() {
            return;
        }
        for &b in chunk {
            if is_token_byte(b) {
                if self.pending.is_empty() {
                    self.pending_start = self.pos;
                }
                if self.pending.len() < self.opts.max_len {
                    self.pending.push(b);
                } else {
                    self.truncated = true;
                }
            } else if !self.pending.is_empty() {
                self.flush_pending(emit);
                if self.limit_reached() {
                    return;
                }
            }
            self.pos += 1;
        }
    }

    /// Signal end-of-stream and flush any trailing token.
    pub fn finish<F: FnMut(Finding)>(&mut self, emit: &mut F) {
        if !self.pending.is_empty() {
            self.flush_pending(emit);
        }
    }

    fn flush_pending<F: FnMut(Finding)>(&mut self, emit: &mut F) {
        let token = std::mem::take(&mut self.pending);
        let start = self.pending_start;
        let truncated = std::mem::replace(&mut self.truncated, false);
        if self.limit_reached() {
            return;
        }
        if let Some(f) = self.evaluate(token, start, truncated) {
            self.findings += 1;
            emit(f);
        }
    }

    /// Classify / optionally decode a completed run and decide whether to keep it.
    fn evaluate(&self, token: Vec<u8>, offset: u64, truncated: bool) -> Option<Finding> {
        if token.len() < self.opts.min_len {
            return None;
        }
        let kinds = magic::detect(&token);
        let entropy = magic::shannon_entropy(&token);

        let entropy_hit = self.opts.min_entropy > 0.0 && entropy >= self.opts.min_entropy;
        let mut keep = !kinds.is_empty() || entropy_hit;
        if !self.opts.only_kinds.is_empty() {
            keep = keep
                && kinds.iter().any(|k| {
                    let k = k.to_ascii_lowercase();
                    self.opts
                        .only_kinds
                        .iter()
                        .any(|w| k.contains(&w.to_ascii_lowercase()))
                });
        }
        if !keep {
            return None;
        }

        let mut decoded = None;
        let mut recipe = None;
        let want_decode = self.opts.decode || self.opts.crib.is_some();
        if want_decode {
            let opts = MagicOptions {
                depth: self.opts.depth,
                crib: self.opts.crib.clone(),
                intensive: false,
                max_results: 1,
                ..MagicOptions::default()
            };
            let results = magic::magic(&token, &opts);
            // With a crib set, only keep findings that actually matched it.
            if let Some(best) = results.into_iter().next() {
                if self.opts.crib.is_some() && !best.matched_crib {
                    return None;
                }
                // Quality gate: a common word can be valid base64 that decodes to
                // noise. Keep the finding only if the decode is meaningful — valid
                // UTF-8 text, a crib hit, or backed by a high-entropy token (a real
                // encoded binary payload rather than a dictionary word).
                let meaningful = best.valid_utf8 || best.matched_crib || entropy >= 4.5;
                if !meaningful {
                    return None;
                }
                decoded = Some(best.preview.clone());
                recipe = Some(best.recipe);
            } else if self.opts.crib.is_some() {
                return None;
            }
        }

        let mut token_str = magic::preview(&token, 120);
        if truncated {
            token_str.push('…');
        }

        Some(Finding {
            offset,
            len: token.len(),
            kinds,
            entropy,
            token: token_str,
            decoded,
            recipe,
        })
    }
}

/// Convenience: scan an in-memory buffer and collect all findings.
pub fn scan_bytes(data: &[u8], opts: ScanOptions) -> Vec<Finding> {
    let mut out = Vec::new();
    let mut scanner = Scanner::new(opts);
    scanner.push(data, &mut |f| out.push(f));
    scanner.finish(&mut |f| out.push(f));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_embedded_base64() {
        let data = b"noise before SGVsbG8gV29ybGQ= noise after";
        let opts = ScanOptions {
            min_len: 8,
            decode: true,
            ..Default::default()
        };
        let findings = scan_bytes(data, opts);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].decoded.as_deref(), Some("Hello World"));
        assert!(findings[0].kinds.iter().any(|k| k == "From Base64"));
    }

    #[test]
    fn token_survives_chunk_boundary() {
        // Split the base64 token across two pushes.
        let opts = ScanOptions {
            min_len: 8,
            decode: true,
            ..Default::default()
        };
        let mut scanner = Scanner::new(opts);
        let mut found = Vec::new();
        scanner.push(b"xx SGVsbG8g", &mut |f| found.push(f));
        scanner.push(b"V29ybGQ= yy", &mut |f| found.push(f));
        scanner.finish(&mut |f| found.push(f));
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].decoded.as_deref(), Some("Hello World"));
    }

    #[test]
    fn crib_filters_findings() {
        // base64("goodbye") and base64("hello world"); only the latter matches.
        let data = b"Z29vZGJ5ZQ== aGVsbG8gd29ybGQ=";
        let opts = ScanOptions {
            min_len: 8,
            crib: Some(Regex::new("world").unwrap()),
            ..Default::default()
        };
        let findings = scan_bytes(data, opts);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].decoded.as_deref(), Some("hello world"));
    }

    #[test]
    fn short_tokens_ignored() {
        let data = b"hi ab cd";
        assert!(scan_bytes(data, ScanOptions::default()).is_empty());
    }

    #[test]
    fn finds_jwt_segments() {
        // A JWT in a log line: header and payload decode to JSON individually.
        let data = b"auth Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c end";
        let opts = ScanOptions {
            min_len: 12,
            decode: true,
            ..Default::default()
        };
        let findings = scan_bytes(data, opts);
        assert!(
            findings
                .iter()
                .any(|f| f.decoded.as_deref() == Some("{\"alg\":\"HS256\",\"typ\":\"JWT\"}")),
            "expected the JWT header to decode to JSON, got {:?}",
            findings.iter().map(|f| &f.decoded).collect::<Vec<_>>()
        );
    }

    #[test]
    fn finds_and_decodes_unpadded_base32() {
        // Base32("Hello world") without padding. The token also looks like
        // base64, so this exercises the engine picking the meaningful decode.
        let data = b"secret JBSWY3DPEB3W64TMMQ tail";
        let opts = ScanOptions {
            min_len: 8,
            decode: true,
            ..Default::default()
        };
        let findings = scan_bytes(data, opts);
        assert!(
            findings
                .iter()
                .any(|f| f.decoded.as_deref() == Some("Hello world")),
            "expected an unpadded base32 token to decode to 'Hello world', got {:?}",
            findings.iter().map(|f| &f.decoded).collect::<Vec<_>>()
        );
    }

    #[test]
    fn dictionary_word_dropped_in_decode_mode() {
        // "hexpayload" is valid base64 but decodes to noise; the quality gate
        // should suppress it when decoding, while keeping the real payload.
        let data = b"hexpayload SGVsbG8gV29ybGQ=";
        let opts = ScanOptions {
            min_len: 8,
            decode: true,
            ..Default::default()
        };
        let findings = scan_bytes(data, opts);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].decoded.as_deref(), Some("Hello World"));
    }

    #[test]
    fn finding_and_token_limits_bound_streaming_work() {
        let opts = ScanOptions {
            min_len: 8,
            max_len: 12,
            max_findings: 1,
            ..ScanOptions::default()
        };
        let findings = scan_bytes(
            b"SGVsbG8gV29ybGQ= U0dWc2JHOD0= QUJDREVGR0hJSktMTU5PUA==",
            opts,
        );
        assert_eq!(findings.len(), 1);
        assert!(findings[0].token.ends_with('…'));
        assert!(findings[0].len <= 12);
    }
}
