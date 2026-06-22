/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the HTML To Text operation.
 * -----------------------------------------------------------------------------
 */

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// HTML To Text operation
///
/// Strips HTML tags and decodes common HTML entities, returning plain text.
pub struct HTMLToText;

fn decode_entities(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
        .replace("&nbsp;", " ")
        .replace("&copy;", "(c)")
        .replace("&reg;", "(R)")
        .replace("&trade;", "(TM)")
        .replace("&mdash;", "--")
        .replace("&ndash;", "-")
        .replace("&laquo;", "<<")
        .replace("&raquo;", ">>")
}

impl Operation for HTMLToText {
    fn name(&self) -> &'static str {
        "HTML To Text"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts HTML to readable plain text by stripping tags and decoding common HTML entities."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::Html
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let text = String::from_utf8_lossy(&input);

        let tag_re =
            Regex::new(r"<[^>]*>").map_err(|e| OperationError::ProcessingError(e.to_string()))?;
        let stripped = tag_re.replace_all(&text, "");
        let decoded = decode_entities(&stripped);
        Ok(decoded.into_bytes())
    }
}
