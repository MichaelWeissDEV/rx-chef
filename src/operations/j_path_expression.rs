/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the JPath expression operation.
 * -----------------------------------------------------------------------------
 */

use serde_json::Value;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// JPath expression operation
pub struct JPathExpression;

impl Operation for JPathExpression {
    fn name(&self) -> &'static str {
        "JPath expression"
    }

    fn module(&self) -> &'static str {
        "Code"
    }

    fn description(&self) -> &'static str {
        "Extract information from a JSON object with a JPath query."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Query",
                description: "The JPath query to run",
                default_value: "",
            },
            ArgSchema {
                name: "Result delimiter",
                description: "The delimiter to use between results",
                default_value: "\\n",
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
        let query = args.first().and_then(|v| v.as_str()).unwrap_or("");
        let delimiter = args
            .get(1)
            .and_then(|v| v.as_str())
            .unwrap_or("\n")
            .replace("\\n", "\n")
            .replace("\\r", "\r")
            .replace("\\t", "\t");

        if input.is_empty() {
            return Err(OperationError::ProcessingError(
                "Invalid input JSON: Unexpected end of JSON input".to_string(),
            ));
        }

        let json_obj: Value = serde_json::from_slice(&input)
            .map_err(|e| OperationError::ProcessingError(format!("Invalid input JSON: {}", e)))?;

        if query.is_empty() {
            return Ok(Vec::new());
        }

        let results = match evaluate_jpath(&json_obj, query) {
            Ok(res) => res,
            Err(e) => {
                return Err(OperationError::ProcessingError(format!(
                    "Invalid JPath expression: {}",
                    e
                )))
            }
        };

        let output = results
            .iter()
            .map(|r| serde_json::to_string(r).unwrap_or_default())
            .collect::<Vec<String>>()
            .join(&delimiter);

        Ok(output.into_bytes())
    }
}

/// A very basic JPath evaluator that supports root ($), child (.) and wildcard (*)
/// This is a simplified implementation.
fn evaluate_jpath(json: &Value, query: &str) -> Result<Vec<Value>, String> {
    if !query.starts_with('$') {
        return Err("Query must start with $".to_string());
    }

    let mut current_nodes = vec![json.clone()];
    let segments = query[1..].split('.');

    for segment in segments {
        if segment.is_empty() {
            continue;
        }

        let mut next_nodes = Vec::new();

        for node in current_nodes {
            if segment == "*" {
                if let Some(obj) = node.as_object() {
                    for val in obj.values() {
                        next_nodes.push(val.clone());
                    }
                } else if let Some(arr) = node.as_array() {
                    for val in arr {
                        next_nodes.push(val.clone());
                    }
                }
            } else if segment.contains('[') && segment.contains(']') {
                // Basic support for array indexing or wildcards in brackets
                let parts: Vec<&str> = segment.split('[').collect();
                let field = parts[0];
                let index_part = parts[1].replace(']', "");

                let base_node = if field.is_empty() {
                    Some(&node)
                } else {
                    node.get(field)
                };

                if let Some(base) = base_node {
                    if index_part == "*" {
                        if let Some(arr) = base.as_array() {
                            for val in arr {
                                next_nodes.push(val.clone());
                            }
                        } else if let Some(obj) = base.as_object() {
                            for val in obj.values() {
                                next_nodes.push(val.clone());
                            }
                        }
                    } else if let Ok(idx) = index_part.parse::<usize>() {
                        if let Some(val) = base.get(idx) {
                            next_nodes.push(val.clone());
                        }
                    } else {
                        // For more complex expressions, we just return empty or error
                    }
                }
            } else {
                if let Some(val) = node.get(segment) {
                    next_nodes.push(val.clone());
                }
            }
        }
        current_nodes = next_nodes;
    }

    Ok(current_nodes)
}
