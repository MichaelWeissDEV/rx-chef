/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the P-list Viewer operation.
 * -----------------------------------------------------------------------------
 */

use plist::Value;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// P-list Viewer operation
pub struct PLISTViewer;

impl Operation for PLISTViewer {
    fn name(&self) -> &'static str {
        "P-list Viewer"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "In the macOS, iOS, NeXTSTEP, and GNUstep programming frameworks, property list files are files that store serialized objects. Property list files use the filename extension .plist, and thus are often referred to as p-list files.<br><br>This operation displays plist files in a human readable format."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        if input.is_empty() {
            return Ok(Vec::new());
        }

        let value = Value::from_reader(std::io::Cursor::new(input))
            .map_err(|e| OperationError::InvalidInput(format!("Invalid PLIST: {}", e)))?;

        let mut output = String::from("plist => \n");
        format_plist_value(&value, &mut output, 0, None);
        output.push_str("/plist");

        Ok(output.into_bytes())
    }
}

fn format_plist_value(value: &Value, output: &mut String, depth: usize, index: Option<usize>) {
    let indent = "\t".repeat(depth);

    let prefix = if let Some(i) = index {
        format!("{}{}{} => ", indent, i, if depth > 0 { "" } else { "" }) // CyberChef's logic is a bit weird with index
    } else {
        indent.clone()
    };

    match value {
        Value::Dictionary(dict) => {
            output.push_str(&format!("{}{}\n", prefix, "{"));
            for (key, val) in dict {
                let key_prefix = format!("{}\t{} => ", indent, key);
                match val {
                    Value::Dictionary(_) | Value::Array(_) => {
                        output.push_str(&key_prefix);
                        format_plist_value(val, output, depth + 1, None);
                    }
                    _ => {
                        output.push_str(&key_prefix);
                        format_plist_value(val, output, 0, None); // Reset depth for basic values if they are on same line
                    }
                }
            }
            output.push_str(&format!("{}{}\n", indent, "}"));
        }
        Value::Array(arr) => {
            output.push_str(&format!("{}{}\n", prefix, "["));
            for (i, val) in arr.iter().enumerate() {
                format_plist_value(val, output, depth + 1, Some(i));
            }
            output.push_str(&format!("{}]\n", indent));
        }
        Value::String(s) => {
            output.push_str(&format!("\"{}\"\n", s));
        }
        Value::Integer(i) => {
            output.push_str(&format!("{}\n", i));
        }
        Value::Real(f) => {
            output.push_str(&format!("{}\n", f));
        }
        Value::Boolean(b) => {
            output.push_str(&format!("{}\n", b));
        }
        Value::Date(d) => {
            // plist::Date doesn't implement Display, so use Debug formatting
            output.push_str(&format!("{:?}\n", d));
        }
        Value::Data(data) => {
            output.push_str(&format!("{}\n", hex::encode(data)));
        }
        _ => {
            output.push_str("unknown\n");
        }
    }
}
