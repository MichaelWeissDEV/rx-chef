/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the JSON to CSV operation.
 * -----------------------------------------------------------------------------
 */

use serde_json::Value;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// JSON to CSV operation
pub struct JSONToCSV;

impl Operation for JSONToCSV {
    fn name(&self) -> &'static str {
        "JSON to CSV"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts JSON data to a CSV based on the definition in RFC 4180."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Cell delimiter",
                description: "The character(s) to use to separate cells",
                default_value: ",",
            },
            ArgSchema {
                name: "Row delimiter",
                description: "The character(s) to use to separate rows",
                default_value: "\\r\\n",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Json
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);
        if input_str.trim().is_empty() {
            return Ok(Vec::new());
        }

        let json: Value = serde_json::from_str(&input_str)
            .map_err(|e| OperationError::InvalidInput(format!("Unable to parse JSON: {}", e)))?;

        let cell_delim = args.first().and_then(|v| v.as_str()).unwrap_or(",");
        let row_delim = args.get(1).and_then(|v| v.as_str()).unwrap_or("\r\n");
        let row_delim = row_delim.replace("\\r", "\r").replace("\\n", "\n");

        let flattened = if let Value::Array(arr) = json {
            arr
        } else {
            vec![json]
        };

        if flattened.is_empty() {
            return Ok(Vec::new());
        }

        let mut output = String::new();

        // Check if first element is an array
        if let Some(Value::Array(_first_row)) = flattened.first() {
            for row in flattened {
                if let Value::Array(cols) = row {
                    let mut row_str = Vec::new();
                    for col in cols {
                        row_str.push(escape_cell(&col, cell_delim, &row_delim));
                    }
                    output.push_str(&row_str.join(cell_delim));
                    output.push_str(&row_delim);
                }
            }
        } else if let Some(Value::Object(first_obj)) = flattened.first() {
            // Array of objects
            let header: Vec<String> = first_obj.keys().cloned().collect();

            // Header
            let mut header_str = Vec::new();
            for h in &header {
                header_str.push(escape_cell(
                    &Value::String(h.clone()),
                    cell_delim,
                    &row_delim,
                ));
            }
            output.push_str(&header_str.join(cell_delim));
            output.push_str(&row_delim);

            // Rows
            for row in flattened {
                if let Value::Object(obj) = row {
                    let mut row_str = Vec::new();
                    for h in &header {
                        let val = obj.get(h).unwrap_or(&Value::Null);
                        row_str.push(escape_cell(val, cell_delim, &row_delim));
                    }
                    output.push_str(&row_str.join(cell_delim));
                    output.push_str(&row_delim);
                }
            }
        } else {
            // Single value or other
            output.push_str(&escape_cell(&flattened[0], cell_delim, &row_delim));
            output.push_str(&row_delim);
        }

        Ok(output.into_bytes())
    }
}

fn escape_cell(val: &Value, cell_delim: &str, row_delim: &str) -> String {
    let mut data = match val {
        Value::Null => "".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        _ => val.to_string(),
    };

    let needs_quotes = data.contains(cell_delim)
        || data.contains(row_delim)
        || data.contains('\n')
        || data.contains('\r')
        || data.contains('"');

    data = data.replace('"', "\"\"");

    if needs_quotes {
        format!("\"{}\"", data)
    } else {
        data
    }
}
