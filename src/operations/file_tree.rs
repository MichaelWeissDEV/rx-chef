/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the File Tree operation.
 * -----------------------------------------------------------------------------
 */

use itertools::Itertools;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// File Tree operation
pub struct FileTree;

impl Operation for FileTree {
    fn name(&self) -> &'static str {
        "File Tree"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Creates a file tree from a list of file paths (similar to the tree command in Linux)"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "File Path Delimiter",
                description: "Delimiter between path components",
                default_value: "/",
            },
            ArgSchema {
                name: "Delimiter",
                description: "Delimiter between entries",
                default_value: "Line feed",
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

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);
        let file_delim = args.first().and_then(|v| v.as_str()).unwrap_or("/");
        let entry_delim_name = args.get(1).and_then(|v| v.as_str()).unwrap_or("Line feed");

        let entry_delim = match entry_delim_name {
            "Line feed" | "\\n" | "\n" => "\n",
            "CRLF" | "\\r\\n" | "\r\n" => "\r\n",
            "Space" | " " => " ",
            "Comma" | "," => ",",
            "Semi-colon" | ";" => ";",
            "Colon" | ":" => ":",
            "Nothing (separate chars)" | "" => "",
            _ => "\n",
        };

        let arrow = "|---";
        let pipe = "|   ";

        let mut completed_list = std::collections::HashSet::new();
        let mut print_list = Vec::new();

        let file_paths: Vec<&str> = if entry_delim.is_empty() {
            input_str
                .split("")
                .filter(|s| !s.is_empty())
                .unique()
                .sorted()
                .collect()
        } else {
            input_str.split(entry_delim).unique().sorted().collect()
        };

        for path_str in file_paths {
            if path_str.is_empty() {
                continue;
            }
            let mut components: Vec<&str> = path_str.split(file_delim).collect();
            if components.first() == Some(&"") {
                components.remove(0);
            }

            for j in 0..components.len() {
                let key = components[0..=j].join("/");
                if !completed_list.contains(&key) {
                    let print_line = if j == 0 {
                        components[j].to_string()
                    } else {
                        format!("{}{}{}", pipe.repeat(j - 1), arrow, components[j])
                    };
                    completed_list.insert(key);
                    print_list.push(print_line);
                }
            }
        }

        Ok(print_list.join("\n").into_bytes())
    }
}
