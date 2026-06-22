/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Magic operation.
 * -----------------------------------------------------------------------------
 */

use serde::{Deserialize, Serialize};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Magic operation
pub struct Magic;

#[derive(Serialize, Deserialize)]
struct MagicResult {
    recipe: Vec<RecipeStep>,
    data: String,
    entropy: f64,
    matches_crib: bool,
    // Add more fields as needed
}

#[derive(Serialize, Deserialize)]
struct RecipeStep {
    op: String,
    args: Vec<String>,
}

impl Operation for Magic {
    fn name(&self) -> &'static str {
        "Magic"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "The Magic operation attempts to detect various properties of the input data and suggests which operations could help to make more sense of it."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Depth",
                description: "Maximum number of levels of recursion",
                default_value: "3",
            },
            ArgSchema {
                name: "Intensive mode",
                description: "Brute-force XOR, bit rotates, etc.",
                default_value: "false",
            },
            ArgSchema {
                name: "Extensive language support",
                description: "Compare byte frequencies to a large number of languages",
                default_value: "false",
            },
            ArgSchema {
                name: "Crib (known plaintext string or regex)",
                description: "Filter results by matching this string or regex",
                default_value: "",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Json
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let _depth = args.first().and_then(|v| v.as_f64()).unwrap_or(3.0) as usize;
        let _intensive = args.get(1).and_then(|v| v.as_bool()).unwrap_or(false);
        let _ext_lang = args.get(2).and_then(|v| v.as_bool()).unwrap_or(false);
        let crib = args.get(3).and_then(|v| v.as_str()).unwrap_or("");

        let entropy = calculate_entropy(&input);

        let mut results = Vec::new();

        // Basic detection logic
        if is_base64(&input) {
            results.push(MagicResult {
                recipe: vec![RecipeStep {
                    op: "From Base64".to_string(),
                    args: vec![],
                }],
                data: "Detected Base64".to_string(),
                entropy,
                matches_crib: crib.is_empty() || "Detected Base64".contains(crib),
            });
        }

        if is_hex(&input) {
            results.push(MagicResult {
                recipe: vec![RecipeStep {
                    op: "From Hex".to_string(),
                    args: vec![],
                }],
                data: "Detected Hex".to_string(),
                entropy,
                matches_crib: crib.is_empty() || "Detected Hex".contains(crib),
            });
        }

        if input.starts_with(&[0x1f, 0x8b]) {
            results.push(MagicResult {
                recipe: vec![RecipeStep {
                    op: "Gunzip".to_string(),
                    args: vec![],
                }],
                data: "Detected Gzip".to_string(),
                entropy,
                matches_crib: crib.is_empty() || "Detected Gzip".contains(crib),
            });
        }

        serde_json::to_vec(&results).map_err(|e| OperationError::ProcessingError(e.to_string()))
    }
}

fn calculate_entropy(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let mut counts = [0usize; 256];
    for &b in data {
        counts[b as usize] += 1;
    }
    let len = data.len() as f64;
    let mut entropy = 0.0;
    for &count in &counts {
        if count > 0 {
            let p = count as f64 / len;
            entropy -= p * p.log2();
        }
    }
    entropy
}

fn is_base64(data: &[u8]) -> bool {
    if data.len() < 4 {
        return false;
    }
    data.iter().all(|&b| {
        (b >= b'A' && b <= b'Z')
            || (b >= b'a' && b <= b'z')
            || (b >= b'0' && b <= b'9')
            || b == b'+'
            || b == b'/'
            || b == b'='
            || b == b'\r'
            || b == b'\n'
    }) && data.len().is_multiple_of(4)
}

fn is_hex(data: &[u8]) -> bool {
    if data.len() < 2 {
        return false;
    }
    data.iter().all(|&b| {
        (b >= b'0' && b <= b'9')
            || (b >= b'a' && b <= b'f')
            || (b >= b'A' && b <= b'F')
            || b == b' '
            || b == b'\r'
            || b == b'\n'
            || b == b':'
    })
}
