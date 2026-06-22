/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * License:     Apache-2.0
 * Description: Input analysis (Magic) implementation.
 * -----------------------------------------------------------------------------
 */

use serde::Serialize;

#[derive(Serialize)]
pub struct MagicResult {
    pub op_name: String,
    pub confidence: f32,
    pub description: String,
}

/// Analyzes the input to suggest possible operations.
pub fn analyze_input(input: &[u8]) -> Vec<MagicResult> {
    let mut results = Vec::new();
    if input.is_empty() {
        return results;
    }

    let input_str = String::from_utf8_lossy(input);

    // 1. Check for Base64
    if is_base64(input) {
        results.push(MagicResult {
            op_name: "From Base64".to_string(),
            confidence: 0.9,
            description: "Input looks like Base64 encoded data.".to_string(),
        });
    }

    // 2. Check for Hex
    if is_hex(&input_str) {
        results.push(MagicResult {
            op_name: "From Hex".to_string(),
            confidence: 0.8,
            description: "Input looks like Hexadecimal encoded data.".to_string(),
        });
    }

    // 3. Check for URL Encoding
    if input_str.contains('%') && input_str.len() > 3 {
        results.push(MagicResult {
            op_name: "URL Decode".to_string(),
            confidence: 0.7,
            description: "Input contains percent signs, might be URL encoded.".to_string(),
        });
    }

    // 4. Check for JSON
    if ((input_str.trim().starts_with('{') && input_str.trim().ends_with('}'))
        || (input_str.trim().starts_with('[') && input_str.trim().ends_with(']')))
        && serde_json::from_str::<serde_json::Value>(&input_str).is_ok()
    {
        results.push(MagicResult {
            op_name: "JSON Beautify".to_string(),
            confidence: 0.95,
            description: "Input is valid JSON.".to_string(),
        });
    }

    results.sort_by(|a, b| {
        b.confidence
            .partial_cmp(&a.confidence)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    results
}

fn is_base64(input: &[u8]) -> bool {
    if input.is_empty() {
        return false;
    }
    let mut count = 0;
    for &b in input {
        match b {
            b' ' | b'\n' | b'\r' | b'\t' => continue,
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'+' | b'/' | b'=' => count += 1,
            _ => return false,
        }
    }
    count >= 4 && count % 4 == 0
}

fn is_hex(s: &str) -> bool {
    let cleaned: String = s.chars().filter(|c| !c.is_ascii_whitespace()).collect();
    if cleaned.len() < 4 || !cleaned.len().is_multiple_of(2) {
        return false;
    }
    cleaned.chars().all(|c| c.is_ascii_hexdigit())
}
