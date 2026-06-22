/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Generate De Bruijn Sequence operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Generate De Bruijn Sequence operation
///
/// Generates a De Bruijn sequence B(k, n) - a cyclic sequence in which every
/// possible subsequence of length n appears exactly once.
pub struct GenerateDeBruijnSequence;

/// Recursive De Bruijn algorithm (Martin's algorithm).
/// This mirrors the JavaScript implementation exactly.
fn de_bruijn(k: usize, n: usize) -> Vec<u8> {
    let mut a = vec![0u8; k * n + 1];
    let mut sequence: Vec<u8> = Vec::new();

    fn db(a: &mut Vec<u8>, sequence: &mut Vec<u8>, t: usize, p: usize, k: usize, n: usize) {
        if t > n {
            if n % p == 0 {
                for j in 1..=p {
                    sequence.push(a[j]);
                }
            }
            return;
        }
        a[t] = a[t - p];
        db(a, sequence, t + 1, p, k, n);
        for j in (a[t - p] + 1) as usize..k {
            a[t] = j as u8;
            db(a, sequence, t + 1, t, k, n);
        }
    }

    db(&mut a, &mut sequence, 1, 1, k, n);
    sequence
}

impl Operation for GenerateDeBruijnSequence {
    fn name(&self) -> &'static str {
        "Generate De Bruijn Sequence"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Generates rolling keycode combinations given a certain alphabet size (k) and key length (n). The result is a De Bruijn sequence B(k,n)."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Alphabet size (k)",
                description: "Size of the alphabet (2-9)",
                default_value: "2",
            },
            ArgSchema {
                name: "Key length (n)",
                description: "Length of each key (2 or more)",
                default_value: "3",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, _input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let k = args.first().and_then(|a| a.as_usize()).unwrap_or(2);
        let n = args.get(1).and_then(|a| a.as_usize()).unwrap_or(3);

        if k < 2 || k > 9 {
            return Err(OperationError::InvalidArgument {
                name: "Alphabet size (k)".to_string(),
                reason: "Invalid alphabet size, required to be between 2 and 9 (inclusive)."
                    .to_string(),
            });
        }

        if n < 2 {
            return Err(OperationError::InvalidArgument {
                name: "Key length (n)".to_string(),
                reason: "Invalid key length, required to be at least 2.".to_string(),
            });
        }

        let permutations =
            k.checked_pow(n as u32)
                .ok_or_else(|| OperationError::InvalidArgument {
                    name: "k^n".to_string(),
                    reason: "Too many permutations, please reduce k^n to under 50,000.".to_string(),
                })?;

        if permutations > 50_000 {
            return Err(OperationError::InvalidArgument {
                name: "k^n".to_string(),
                reason: "Too many permutations, please reduce k^n to under 50,000.".to_string(),
            });
        }

        let seq = de_bruijn(k, n);
        let result: String = seq.iter().map(|b| b.to_string()).collect();
        Ok(result.into_bytes())
    }
}
