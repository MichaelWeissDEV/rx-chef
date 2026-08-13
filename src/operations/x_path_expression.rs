/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the XPath expression operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// XPath expression operation
///
/// Extract information from an XML document with an XPath query.
pub struct XPathExpression;

impl Operation for XPathExpression {
    fn name(&self) -> &'static str {
        "XPath expression"
    }

    fn module(&self) -> &'static str {
        "Code"
    }

    fn description(&self) -> &'static str {
        "Extract information from an XML document with an XPath query"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "XPath",
                description: "The XPath query",
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
        let input = std::str::from_utf8(&input)
            .map_err(|error| OperationError::InvalidInput(error.to_string()))?;
        let expression = args.first().and_then(ArgValue::as_str).unwrap_or("");
        if expression.trim().is_empty() {
            return Err(OperationError::InvalidArgument {
                name: "XPath".to_string(),
                reason: "must not be empty".to_string(),
            });
        }
        let delimiter = args
            .get(1)
            .and_then(ArgValue::as_str)
            .unwrap_or("\\n")
            .replace("\\n", "\n")
            .replace("\\r", "\r")
            .replace("\\t", "\t");
        let package = sxd_document::parser::parse(input)
            .map_err(|error| OperationError::InvalidInput(error.to_string()))?;
        let factory = sxd_xpath::Factory::new();
        let xpath = factory
            .build(expression)
            .map_err(|error| OperationError::InvalidArgument {
                name: "XPath".to_string(),
                reason: error.to_string(),
            })?
            .ok_or_else(|| OperationError::InvalidArgument {
                name: "XPath".to_string(),
                reason: "empty expression".to_string(),
            })?;
        let context = sxd_xpath::Context::new();
        let value = xpath
            .evaluate(&context, package.as_document().root())
            .map_err(|error| OperationError::ProcessingError(error.to_string()))?;
        let output = match value {
            sxd_xpath::Value::Nodeset(nodes) => nodes
                .document_order()
                .iter()
                .map(|node| node.string_value())
                .collect::<Vec<_>>()
                .join(&delimiter),
            value => value.string(),
        };
        Ok(output.into_bytes())
    }
}
