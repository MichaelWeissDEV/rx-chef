/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.1.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the YARA Rules operation.
 * -----------------------------------------------------------------------------
 */

use yara_x::{Compiler, Scanner};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// YARA Rules operation
///
/// YARA is a tool developed at VirusTotal, primarily aimed at helping malware researchers to identify and classify malware samples.
pub struct YARARules;

impl Operation for YARARules {
    fn name(&self) -> &'static str {
        "YARA Rules"
    }

    fn module(&self) -> &'static str {
        "Yara"
    }

    fn description(&self) -> &'static str {
        "YARA is a tool developed at VirusTotal, primarily aimed at helping malware researchers to identify and classify malware samples. It matches based on rules specified by the user containing textual or binary patterns and a boolean expression. For help on writing rules, see the <a href='https://yara.readthedocs.io/en/latest/writingrules.html'>YARA documentation.</a>"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Rules",
                description: "YARA rules to match against the input",
                default_value: "",
            },
            ArgSchema {
                name: "Show strings",
                description: "Display the strings that matched",
                default_value: "false",
            },
            ArgSchema {
                name: "Show string lengths",
                description: "Display the lengths of the strings that matched",
                default_value: "false",
            },
            ArgSchema {
                name: "Show metadata",
                description: "Display the metadata for each rule",
                default_value: "false",
            },
            ArgSchema {
                name: "Show counts",
                description: "Display the number of times each rule matched",
                default_value: "true",
            },
            ArgSchema {
                name: "Show rule warnings",
                description: "Display any warnings generated during rule compilation",
                default_value: "true",
            },
            ArgSchema {
                name: "Show console module messages",
                description: "Display any messages from the console module",
                default_value: "true",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let rules_str = args.first().and_then(|a| a.as_str()).unwrap_or("");
        let show_strings = args.get(1).and_then(|a| a.as_bool()).unwrap_or(false);
        let show_lengths = args.get(2).and_then(|a| a.as_bool()).unwrap_or(false);
        let show_meta = args.get(3).and_then(|a| a.as_bool()).unwrap_or(false);
        let show_counts = args.get(4).and_then(|a| a.as_bool()).unwrap_or(true);

        let mut compiler = Compiler::new();
        if let Err(e) = compiler.add_source(rules_str) {
            return Err(OperationError::InvalidArgument {
                name: "Rules".to_string(),
                reason: format!("YARA compilation error: {}", e),
            });
        }

        let rules = compiler.build();
        let mut scanner = Scanner::new(&rules);
        let scan_results = scanner
            .scan(&input)
            .map_err(|e| OperationError::ProcessingError(format!("YARA scan error: {:?}", e)))?;

        let mut match_string = String::new();

        for rule in scan_results.matching_rules() {
            let mut meta_str = String::new();
            if show_meta {
                let metadata = rule.metadata();
                if !metadata.is_empty() {
                    meta_str.push_str(" [");
                    for (id, val) in metadata {
                        meta_str.push_str(&format!("{}: {:?}, ", id, val));
                    }
                    meta_str.truncate(meta_str.len() - 2);
                    meta_str.push(']');
                }
            }

            let matches = rule.patterns();
            let mut match_count = 0;
            for pattern in matches {
                match_count += pattern.matches().len();
            }

            let count_suffix = if match_count == 0 {
                String::new()
            } else if show_counts {
                format!(
                    "({} time{})",
                    match_count,
                    if match_count > 1 { "s" } else { "" }
                )
            } else {
                String::new()
            };

            let count_suffix_str = if count_suffix.is_empty() {
                String::new()
            } else {
                format!(" {}", count_suffix)
            };

            if match_count == 0 || !(show_strings || show_lengths) {
                match_string.push_str(&format!(
                    "Input matches rule \"{}\"{}{}.\n",
                    rule.identifier(),
                    meta_str,
                    count_suffix_str
                ));
            } else {
                match_string.push_str(&format!(
                    "Rule \"{}\"{} matches{}:\n",
                    rule.identifier(),
                    meta_str,
                    count_suffix_str
                ));
                for pattern in rule.patterns() {
                    for m in pattern.matches() {
                        let data_str = if show_strings {
                            format!(", data: \"{:?}\"", &input[m.range().start..m.range().end])
                        } else {
                            String::new()
                        };
                        let length_str = if show_lengths {
                            format!("length {}, ", m.range().end - m.range().start)
                        } else {
                            String::new()
                        };
                        match_string.push_str(&format!(
                            "Pos {}, {}identifier {}{}\n",
                            m.range().start,
                            length_str,
                            pattern.identifier(),
                            data_str
                        ));
                    }
                }
            }
        }

        Ok(match_string.into_bytes())
    }
}
