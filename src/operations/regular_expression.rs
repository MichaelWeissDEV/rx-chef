/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Regular expression operation.
 * -----------------------------------------------------------------------------
 */

use html_escape::encode_safe;
use regex::{Regex, RegexBuilder};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Regular expression operation
pub struct RegularExpressionOp;

impl Operation for RegularExpressionOp {
    fn name(&self) -> &'static str {
        "Regular expression"
    }

    fn module(&self) -> &'static str {
        "Regex"
    }

    fn description(&self) -> &'static str {
        "Define your own regular expression (regex) to search the input data with, optionally choosing from a list of pre-defined patterns."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Built in regexes",
                description: "Built in regexes",
                default_value: "User defined",
            },
            ArgSchema {
                name: "Regex",
                description: "Regular expression",
                default_value: "",
            },
            ArgSchema {
                name: "Case insensitive",
                description: "Case insensitive",
                default_value: "true",
            },
            ArgSchema {
                name: "^ and $ match at newlines",
                description: "^ and $ match at newlines",
                default_value: "true",
            },
            ArgSchema {
                name: "Dot matches all",
                description: "Dot matches all",
                default_value: "false",
            },
            ArgSchema {
                name: "Unicode support",
                description: "Unicode support",
                default_value: "false",
            },
            ArgSchema {
                name: "Astral support",
                description: "Astral support",
                default_value: "false",
            },
            ArgSchema {
                name: "Display total",
                description: "Display total",
                default_value: "false",
            },
            ArgSchema {
                name: "Output format",
                description: "Output format",
                default_value: "Highlight matches",
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
        let input_str = String::from_utf8_lossy(&input);

        let user_regex = args.get(1).and_then(|v| v.as_str()).unwrap_or("");
        if user_regex.is_empty() || user_regex == "^" || user_regex == "$" {
            return Ok(encode_safe(&input_str).into_owned().into_bytes());
        }

        let case_insensitive = args.get(2).and_then(|v| v.as_bool()).unwrap_or(true);
        let multi_line = args.get(3).and_then(|v| v.as_bool()).unwrap_or(true);
        let dot_matches_new_line = args.get(4).and_then(|v| v.as_bool()).unwrap_or(false);
        let unicode = args.get(5).and_then(|v| v.as_bool()).unwrap_or(false);
        let display_total = args.get(7).and_then(|v| v.as_bool()).unwrap_or(false);
        let output_format = args
            .get(8)
            .and_then(|v| v.as_str())
            .unwrap_or("Highlight matches");

        let regex = RegexBuilder::new(user_regex)
            .case_insensitive(case_insensitive)
            .multi_line(multi_line)
            .dot_matches_new_line(dot_matches_new_line)
            .unicode(unicode || true)
            .build()
            .map_err(|e| OperationError::InvalidArgument {
                name: "Regex".to_string(),
                reason: format!("Invalid regex: {}", e),
            })?;

        match output_format {
            "Highlight matches" => {
                Ok(highlight_matches(&input_str, &regex, display_total).into_bytes())
            }
            "List matches" => {
                Ok(list_matches(&input_str, &regex, display_total, true, false).into_bytes())
            }
            "List capture groups" => {
                Ok(list_matches(&input_str, &regex, display_total, false, true).into_bytes())
            }
            "List matches with capture groups" => {
                Ok(list_matches(&input_str, &regex, display_total, true, true).into_bytes())
            }
            _ => Err(OperationError::InvalidArgument {
                name: "Output format".to_string(),
                reason: "Invalid output format".to_string(),
            }),
        }
    }
}

fn highlight_matches(input: &str, regex: &Regex, display_total: bool) -> String {
    let mut output = String::new();
    let mut last_end = 0;
    let mut total = 0;
    let mut hl = 1;

    for mat in regex.find_iter(input) {
        total += 1;
        output.push_str(&encode_safe(&input[last_end..mat.start()]));

        let mut title = format!("Offset: {}\n", mat.start());
        if let Some(caps) = regex.captures(&input[mat.start()..mat.end()]) {
            if caps.len() > 1 {
                title.push_str("Groups:\n");
                for i in 1..caps.len() {
                    let group_val = caps.get(i).map(|m| m.as_str()).unwrap_or("");
                    title.push_str(&format!("\t{}: {}\n", i, encode_safe(group_val)));
                }
            }
        }

        hl = if hl == 1 { 2 } else { 1 };
        output.push_str(&format!(
            "<span class='hl{}' title='{}'>{}</span>",
            hl,
            title.replace('\'', "&apos;"),
            encode_safe(mat.as_str())
        ));
        last_end = mat.end();
    }
    output.push_str(&encode_safe(&input[last_end..]));

    if display_total {
        format!("Total found: {}\n\n{}", total, output)
    } else {
        output
    }
}

fn list_matches(
    input: &str,
    regex: &Regex,
    display_total: bool,
    show_matches: bool,
    show_groups: bool,
) -> String {
    let mut output = String::new();
    let mut total = 0;

    for caps in regex.captures_iter(input) {
        total += 1;
        if show_matches {
            output.push_str(caps.get(0).unwrap().as_str());
            output.push('\n');
        }
        if show_groups {
            for i in 1..caps.len() {
                if show_matches {
                    output.push_str("  Group ");
                    output.push_str(&i.to_string());
                    output.push_str(": ");
                }
                output.push_str(caps.get(i).map(|m| m.as_str()).unwrap_or(""));
                output.push('\n');
            }
        }
    }

    let mut result = if display_total {
        format!("Total found: {}\n\n{}", total, output)
    } else {
        output
    };

    if result.ends_with('\n') {
        result.pop();
    }

    encode_safe(&result).into_owned()
}
