/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the CSV to JSON operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// CSV to JSON operation
///
/// Converts a CSV file to JSON format. The first row is treated as the header
/// when producing an array of dictionaries. Handles quoted fields and escaped
/// double-quotes within quoted fields.
pub struct CsvToJson;

impl Operation for CsvToJson {
    fn name(&self) -> &'static str {
        "CSV to JSON"
    }

    fn module(&self) -> &'static str {
        "Conversion"
    }

    fn description(&self) -> &'static str {
        "Converts a CSV file to JSON format. The first row is used as the header for \
        'Array of dictionaries' format. Supports quoted fields with embedded delimiters \
        and escaped double-quotes."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Cell delimiter",
                description: "Character used to separate fields",
                default_value: ",",
            },
            ArgSchema {
                name: "Row delimiter",
                description: "Character(s) used to separate rows",
                default_value: "\n",
            },
            ArgSchema {
                name: "Format",
                description: "Output format: 'Array of dictionaries' or 'Array of arrays'",
                default_value: "Array of dictionaries",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Json
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let text = String::from_utf8_lossy(&input);

        let cell_delim = args.first().and_then(|a| a.as_str()).unwrap_or(",");
        let row_delim = args.get(1).and_then(|a| a.as_str()).unwrap_or("\n");
        let format = args
            .get(2)
            .and_then(|a| a.as_str())
            .unwrap_or("Array of dictionaries");

        let cell_delim_char = cell_delim.chars().next().unwrap_or(',');

        // Split into rows using row_delim
        let rows: Vec<Vec<String>> = split_rows(&text, row_delim)
            .iter()
            .filter(|r| !r.trim().is_empty())
            .map(|r| parse_csv_row(r, cell_delim_char))
            .collect();

        if rows.is_empty() {
            return Ok(b"[]".to_vec());
        }

        let json_value = match format {
            "Array of dictionaries" => {
                let header = &rows[0];
                let records: Vec<serde_json::Value> = rows[1..]
                    .iter()
                    .map(|row| {
                        let mut obj = serde_json::Map::new();
                        for (idx, key) in header.iter().enumerate() {
                            let val = row.get(idx).map(|s| s.as_str()).unwrap_or("");
                            obj.insert(key.clone(), serde_json::Value::String(val.to_string()));
                        }
                        serde_json::Value::Object(obj)
                    })
                    .collect();
                serde_json::Value::Array(records)
            }
            _ => {
                // Array of arrays
                let arrays: Vec<serde_json::Value> = rows
                    .iter()
                    .map(|row| {
                        serde_json::Value::Array(
                            row.iter()
                                .map(|s| serde_json::Value::String(s.clone()))
                                .collect(),
                        )
                    })
                    .collect();
                serde_json::Value::Array(arrays)
            }
        };

        let output = serde_json::to_string_pretty(&json_value).map_err(|e| {
            OperationError::ProcessingError(format!("Failed to serialise JSON: {}", e))
        })?;

        Ok(output.into_bytes())
    }
}

/// Split text into rows using the given row delimiter string.
fn split_rows<'a>(text: &'a str, row_delim: &str) -> Vec<&'a str> {
    if row_delim == "\n" {
        // Also handle \r\n
        text.split('\n').map(|s| s.trim_end_matches('\r')).collect()
    } else {
        text.split(row_delim).collect()
    }
}

/// Parse a single CSV row into fields. Handles RFC 4180 quoting.
fn parse_csv_row(row: &str, delim: char) -> Vec<String> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut in_quotes = false;
    let chars: Vec<char> = row.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let ch = chars[i];

        if in_quotes {
            if ch == '"' {
                // Check for escaped quote ""
                if i + 1 < len && chars[i + 1] == '"' {
                    field.push('"');
                    i += 2;
                    continue;
                } else {
                    in_quotes = false;
                }
            } else {
                field.push(ch);
            }
        } else if ch == '"' {
            in_quotes = true;
        } else if ch == delim {
            fields.push(field.clone());
            field.clear();
        } else {
            field.push(ch);
        }

        i += 1;
    }

    fields.push(field);
    fields
}
