/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Syntax highlighter operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Syntax highlighter operation
pub struct SyntaxHighlighter;

impl Operation for SyntaxHighlighter {
    fn name(&self) -> &'static str {
        "Syntax highlighter"
    }

    fn module(&self) -> &'static str {
        "Code"
    }

    fn description(&self) -> &'static str {
        "Adds syntax highlighting to a range of source code languages. Note that this will not indent the code. Use one of the 'Beautify' operations for that."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Language",
            description: "Language to highlight",
            default_value: "auto detect",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Html
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let language = args
            .get(0)
            .and_then(|v| v.as_str())
            .unwrap_or("auto detect");
        let input_str = String::from_utf8_lossy(&input);

        // Note: CyberChef uses highlight.js which is not available as a direct dependency.
        // We provide a basic wrapper and escape the HTML.
        let escaped = html_escape::encode_safe(&input_str);

        let class = if language == "auto detect" {
            "".to_string()
        } else {
            format!(" class=\"language-{}\"", language)
        };

        let result = format!("<pre><code{}>{}</code></pre>", class, escaped);

        Ok(result.into_bytes())
    }
}
