/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the From Base32 operation.
 * -----------------------------------------------------------------------------
 */

use data_encoding::Specification;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// From Base32 operation
pub struct FromBase32;

impl Operation for FromBase32 {
    fn name(&self) -> &'static str {
        "From Base32"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Base32 is a notation for encoding arbitrary byte data using a restricted set of symbols that can be conveniently used by humans and processed by computers. It uses a smaller set of characters than Base64, usually the uppercase alphabet and the numbers 2 to 7."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Alphabet",
                description: "The Base32 alphabet",
                default_value: "A-Z2-7",
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Remove non-alphabet chars",
                description: "Remove characters not in the alphabet before decoding",
                default_value: "true",
                kind: crate::operation::ArgKind::Boolean,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    /// Matches upstream CyberChef byte for byte on the recorded differential case.
    fn parity(&self) -> crate::operation::ParityStatus {
        crate::operation::ParityStatus::Exact
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;

        if input_str.is_empty() {
            return Ok(Vec::new());
        }

        let alphabet_arg = args.first().and_then(|v| v.as_str()).unwrap_or("A-Z2-7");
        let remove_non_alph = args.get(1).and_then(|v| v.as_bool()).unwrap_or(true);

        let alphabet_str = expand_base32_alphabet(alphabet_arg);

        let clean_input = if remove_non_alph {
            input_str
                .chars()
                .filter(|&c| alphabet_str.contains(c) || c == '=')
                .collect::<String>()
        } else {
            input_str
        };

        // Normalise padding: strip any `=` then re-pad to a multiple of 8 so
        // both correctly-padded and unpadded Base32 decode (the scanner and
        // other tools routinely produce unpadded tokens).
        let mut clean_input: String = clean_input.trim().chars().filter(|&c| c != '=').collect();
        let rem = clean_input.len() % 8;
        if rem != 0 {
            clean_input.push_str(&"=".repeat(8 - rem));
        }

        let mut spec = Specification::new();
        spec.symbols = alphabet_str;
        spec.padding = Some('=');
        let encoding = spec
            .encoding()
            .map_err(|e| OperationError::InvalidArgument {
                name: "Alphabet".to_string(),
                reason: format!("Invalid alphabet: {}", e),
            })?;

        encoding
            .decode(clean_input.trim().as_bytes())
            .map_err(|e| OperationError::InvalidInput(format!("Base32 decode failed: {}", e)))
    }
}

/// Expand a Base32 alphabet written in range notation (`A-Z2-7`) into its 32
/// literal symbols, dropping the padding character.
///
/// Shared with `To Base32` so both directions accept exactly the same
/// alphabet spellings.
/// Expand a Base32 alphabet argument into its 32 symbols.
///
/// Delegates to the shared expansion every Base-N operation uses. The previous
/// local copy special-cased the two standard alphabets — both of which the
/// general path already produces identically — and reached
/// `char::from_u32(..).unwrap()`, which panics on surrogate code points from a
/// caller-supplied range.
pub(crate) fn expand_base32_alphabet(alphabet: &str) -> String {
    crate::alphabet::expand_alphabet_without_padding(alphabet)
}
