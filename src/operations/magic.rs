/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Magic operation.
 * -----------------------------------------------------------------------------
 */

use crate::magic::{magic as run_magic, MagicOptions};
use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Magic operation
pub struct Magic;

impl Operation for Magic {
    fn name(&self) -> &'static str {
        "Magic"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "The Magic operation attempts to detect various properties of the input data and suggests which operations could help to make more sense of it."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Depth",
                description: "Maximum number of levels of recursion",
                default_value: "3",
            },
            ArgSchema {
                name: "Intensive mode",
                description: "Brute-force XOR, bit rotates, etc.",
                default_value: "false",
            },
            ArgSchema {
                name: "Extensive language support",
                description: "Compare byte frequencies to a large number of languages",
                default_value: "false",
            },
            ArgSchema {
                name: "Crib (known plaintext string or regex)",
                description: "Filter results by matching this string or regex",
                default_value: "",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Json
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let depth = args.first().and_then(|v| v.as_f64()).unwrap_or(3.0) as usize;
        let intensive = args.get(1).and_then(|v| v.as_bool()).unwrap_or(false);
        // args[2] "Extensive language support" is accepted for CyberChef parity
        // but not yet used by the engine.
        let crib_raw = args.get(3).and_then(|v| v.as_str()).unwrap_or("");

        let crib = if crib_raw.is_empty() {
            None
        } else {
            Some(
                regex::Regex::new(crib_raw).map_err(|e| OperationError::InvalidArgument {
                    name: "Crib".to_string(),
                    reason: format!("invalid crib regex: {e}"),
                })?,
            )
        };

        let opts = MagicOptions {
            depth,
            crib,
            intensive,
            max_results: 20,
            ..MagicOptions::default()
        };
        let results = run_magic(&input, &opts);
        serde_json::to_vec(&results).map_err(|e| OperationError::ProcessingError(e.to_string()))
    }
}
