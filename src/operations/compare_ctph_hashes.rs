/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Compare CTPH hashes operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Compare CTPH Hashes operation
///
/// Compares two Context Triggered Piecewise Hashing (CTPH) fuzzy hashes to
/// determine the similarity between them on a scale of 0 to 100.
pub struct CompareCTPHHashes;

impl Operation for CompareCTPHHashes {
    fn name(&self) -> &'static str {
        "Compare CTPH hashes"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "Compares two Context Triggered Piecewise Hashing (CTPH) fuzzy hashes to determine the similarity between them on a scale of 0 to 100."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Delimiter",
            description:
                "Delimiter between the two hashes (Line feed, CRLF, Space, Comma, Semi-colon)",
            default_value: "Line feed",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let delimiter = args.first().and_then(|a| a.as_str()).unwrap_or("Line feed");
        let delim_char = delimiter_char(delimiter);

        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8 input".to_string()))?;

        let samples: Vec<&str> = input_str.splitn(2, &delim_char as &str).collect();
        if samples.len() != 2 {
            return Err(OperationError::InvalidInput(
                "Incorrect number of samples. Expected exactly 2 hashes separated by the delimiter."
                    .to_string(),
            ));
        }

        let hash1 = samples[0].trim();
        let hash2 = samples[1].trim();

        let score = ssdeep::compare(hash1, hash2)
            .map_err(|e| OperationError::InvalidInput(format!("CTPH compare error: {}", e)))?;

        Ok(score.to_string().into_bytes())
    }
}

fn delimiter_char(name: &str) -> String {
    match name {
        "Line feed" => "\n".to_string(),
        "CRLF" => "\r\n".to_string(),
        "Space" => " ".to_string(),
        "Comma" => ",".to_string(),
        "Semi-colon" => ";".to_string(),
        other => other.to_string(),
    }
}
