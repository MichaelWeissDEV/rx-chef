/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Compare SSDEEP hashes operation.
 * -----------------------------------------------------------------------------
 */

use fuzzyhash::FuzzyHash;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

pub struct CompareSSDEEPHashes;

impl Operation for CompareSSDEEPHashes {
    fn name(&self) -> &'static str {
        "Compare SSDEEP hashes"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "Compares two SSDEEP fuzzy hashes to determine the similarity between them on a scale of 0 to 100."
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Number
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[ArgSchema {
            name: "Delimiter",
            description: "The delimiter separating the two samples",
            default_value: "Line feed",
            kind: crate::operation::ArgKind::String,
            required: false,
            choices: &[],
            minimum: None,
            maximum: None,
            sensitive: false,
        }]
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);

        let delim_name = args.first().and_then(|a| a.as_str()).unwrap_or("Line feed");
        let delim = match delim_name {
            "Space" => " ",
            "Comma" => ",",
            "Semi-colon" => ";",
            "Colon" => ":",
            "Line feed" => "\n",
            "CRLF" => "\r\n",
            _ => "\n",
        };

        let samples: Vec<&str> = input_str.split(delim).collect();
        if samples.len() != 2 {
            return Err(OperationError::InvalidInput(
                "Incorrect number of samples.".to_string(),
            ));
        }

        let similarity = FuzzyHash::compare(samples[0].trim(), samples[1].trim())
            .map_err(|e| OperationError::InvalidInput(format!("SSDEEP compare error: {}", e)))?;

        Ok(similarity.to_string().into_bytes())
    }
}
