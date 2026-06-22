/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Diff operation.
 * -----------------------------------------------------------------------------
 */

use similar::{ChangeTag, TextDiff};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Diff operation
///
/// Compares two inputs (separated by the specified delimiter) and highlights the differences.
pub struct Diff;

impl Operation for Diff {
    fn name(&self) -> &'static str {
        "Diff"
    }

    fn module(&self) -> &'static str {
        "Diff"
    }

    fn description(&self) -> &'static str {
        "Compares two inputs (separated by the specified delimiter) and highlights the differences \
         between them."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Sample delimiter",
                description: "Delimiter separating the two input samples",
                default_value: "\\n\\n",
            },
            ArgSchema {
                name: "Show added",
                description: "Show added text (wrapped in <ins> tags)",
                default_value: "true",
            },
            ArgSchema {
                name: "Show removed",
                description: "Show removed text (wrapped in <del> tags)",
                default_value: "true",
            },
            ArgSchema {
                name: "Show subtraction",
                description: "Show unchanged text",
                default_value: "false",
            },
            ArgSchema {
                name: "Ignore whitespace",
                description: "Ignore leading/trailing whitespace when comparing",
                default_value: "false",
            },
            ArgSchema {
                name: "Ignore case",
                description: "Perform case-insensitive comparison",
                default_value: "false",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Html
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input).to_string();

        let delim = args.first().and_then(|a| a.as_str()).unwrap_or("\\n\\n");
        let show_added = args.get(1).and_then(|a| a.as_bool()).unwrap_or(true);
        let show_removed = args.get(2).and_then(|a| a.as_bool()).unwrap_or(true);
        let show_subtraction = args.get(3).and_then(|a| a.as_bool()).unwrap_or(false);
        let ignore_whitespace = args.get(4).and_then(|a| a.as_bool()).unwrap_or(false);
        let ignore_case = args.get(5).and_then(|a| a.as_bool()).unwrap_or(false);

        let parts: Vec<&str> = input_str.splitn(3, delim).collect();
        if parts.len() != 2 {
            return Err(OperationError::InvalidInput(
                "Incorrect number of samples, perhaps you need to modify the sample delimiter or \
                 add more samples?"
                    .to_string(),
            ));
        }

        let sample_a = parts[0];
        let sample_b = parts[1];

        // Optionally normalize the strings
        let (a_work, b_work) = if ignore_case || ignore_whitespace {
            let mut a = sample_a.to_string();
            let mut b = sample_b.to_string();
            if ignore_whitespace {
                a = a.split_whitespace().collect::<Vec<_>>().join(" ");
                b = b.split_whitespace().collect::<Vec<_>>().join(" ");
            }
            if ignore_case {
                a = a.to_lowercase();
                b = b.to_lowercase();
            }
            (a, b)
        } else {
            (sample_a.to_string(), sample_b.to_string())
        };

        // Use similar crate for character-level diff
        let diff = TextDiff::from_chars(&a_work, &b_work);

        let mut output = String::new();

        for change in diff.iter_all_changes() {
            let text = change.value();
            let escaped = escape_html(text);
            match change.tag() {
                ChangeTag::Insert => {
                    if show_added {
                        output.push_str(&format!("<ins>{}</ins>", escaped));
                    }
                }
                ChangeTag::Delete => {
                    if show_removed {
                        output.push_str(&format!("<del>{}</del>", escaped));
                    }
                }
                ChangeTag::Equal => {
                    if !show_subtraction {
                        output.push_str(&escaped);
                    }
                }
            }
        }

        Ok(output.into_bytes())
    }
}

/// Escape HTML special characters.
fn escape_html(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '&' => "&amp;".to_string(),
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&#x27;".to_string(),
            _ => c.to_string(),
        })
        .collect()
}
