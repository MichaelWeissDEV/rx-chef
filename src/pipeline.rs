/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.1.0
 * Author:      Michael Weiss
 * License:     Apache-2.0
 * Description: Operation pipeline — chain any operations in any order.
 * -----------------------------------------------------------------------------
 */

use std::fmt;

use crate::operation::{ArgValue, Operation, OperationData, OperationError};

/// Error produced when a [`Pipeline`] step fails.
#[derive(Debug)]
pub struct PipelineError {
    /// Zero-based index of the failing step.
    pub step_index: usize,
    /// Name of the operation that failed.
    pub step_name: String,
    /// Underlying operation error.
    pub cause: OperationError,
}

impl fmt::Display for PipelineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "pipeline step {} ('{}') failed: {}",
            self.step_index, self.step_name, self.cause
        )
    }
}

impl std::error::Error for PipelineError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.cause)
    }
}

/// A linear chain of operations where the typed output of one step flows into
/// the next.
///
/// # Type safety
///
/// Each step declares its expected input and output [`DataType`].  The pipeline
/// calls [`OperationData::coerce_to`] at every boundary, so mismatches that
/// are representationally compatible (e.g. `Bytes` → `Text` via UTF-8) are
/// handled transparently.  Incompatible mismatches surface as a
/// [`PipelineError`] at runtime.
///
/// # Example
///
/// ```rust
/// use rxchef::pipeline::Pipeline;
/// use rxchef::operation::ArgValue;
/// use rxchef::operations::get_operation;
///
/// let result = Pipeline::new()
///     .then(get_operation("To Hex").unwrap(),    vec![ArgValue::Str("None".into()), ArgValue::Num(0.0)])
///     .then(get_operation("From Hex").unwrap(),  vec![ArgValue::Str("Auto".into())])
///     .run_text("Hello")
///     .unwrap();
///
/// assert_eq!(result, "Hello");
/// ```
pub struct Pipeline {
    steps: Vec<(Box<dyn Operation>, Vec<ArgValue>)>,
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl Pipeline {
    pub fn new() -> Self {
        Pipeline { steps: Vec::new() }
    }

    /// Append an operation step.
    ///
    /// `op` must be a boxed [`Operation`].  Use
    /// [`rxchef::operations::get_operation`] to look up by name, or pass a
    /// concrete struct directly.
    pub fn then(mut self, op: Box<dyn Operation>, args: Vec<ArgValue>) -> Self {
        self.steps.push((op, args));
        self
    }

    /// Run the pipeline on typed input data.
    ///
    /// Returns the typed output of the last step, or the first
    /// [`PipelineError`] encountered.
    pub fn run(&self, input: OperationData) -> Result<OperationData, PipelineError> {
        let mut current = input;
        for (i, (op, args)) in self.steps.iter().enumerate() {
            current = op.run_typed(current, args).map_err(|cause| PipelineError {
                step_index: i,
                step_name: op.name().to_string(),
                cause,
            })?;
        }
        Ok(current)
    }

    /// Convenience wrapper: run on raw bytes, return raw bytes.
    pub fn run_bytes(&self, input: Vec<u8>) -> Result<Vec<u8>, PipelineError> {
        let out = self.run(OperationData::Bytes(input))?;
        out.into_bytes().map_err(|cause| PipelineError {
            step_index: self.steps.len(),
            step_name: "<output>".into(),
            cause,
        })
    }

    /// Convenience wrapper: run on a string, return a string.
    pub fn run_text(&self, input: &str) -> Result<String, PipelineError> {
        let out = self.run(OperationData::Text(input.to_owned()))?;
        out.into_text().map_err(|cause| PipelineError {
            step_index: self.steps.len(),
            step_name: "<output>".into(),
            cause,
        })
    }

    /// Number of steps in this pipeline.
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operations::get_operation;

    fn op(name: &str) -> Box<dyn Operation> {
        get_operation(name).unwrap_or_else(|| panic!("operation not found: {name}"))
    }

    #[test]
    fn roundtrip_hex() {
        let result = Pipeline::new()
            .then(
                op("To Hex"),
                vec![ArgValue::Str("None".into()), ArgValue::Num(0.0)],
            )
            .then(op("From Hex"), vec![ArgValue::Str("Auto".into())])
            .run_text("Hello world")
            .unwrap();
        assert_eq!(result, "Hello world");
    }

    #[test]
    fn roundtrip_base64() {
        let result = Pipeline::new()
            .then(op("To Base64"), vec![ArgValue::Str("A-Za-z0-9+/=".into())])
            .then(
                op("From Base64"),
                vec![ArgValue::Str("A-Za-z0-9+/=".into()), ArgValue::Bool(false)],
            )
            .run_text("rxchef pipeline test")
            .unwrap();
        assert_eq!(result, "rxchef pipeline test");
    }

    #[test]
    fn three_step_pipeline() {
        // "Hello" → To Hex → From Hex → To Base64
        // From Hex undoes To Hex, so the final result is Base64("Hello") = "SGVsbG8="
        let result = Pipeline::new()
            .then(
                op("To Hex"),
                vec![ArgValue::Str("None".into()), ArgValue::Num(0.0)],
            )
            .then(op("From Hex"), vec![ArgValue::Str("Auto".into())])
            .then(op("To Base64"), vec![ArgValue::Str("A-Za-z0-9+/=".into())])
            .run_text("Hello")
            .unwrap();

        assert_eq!(result, "SGVsbG8=");
    }

    #[test]
    fn empty_pipeline_is_passthrough() {
        let result = Pipeline::new().run_text("unchanged").unwrap();
        assert_eq!(result, "unchanged");
    }

    #[test]
    fn bytes_pipeline() {
        let input = b"binary data".to_vec();
        let result = Pipeline::new()
            .then(
                op("To Hex"),
                vec![ArgValue::Str("None".into()), ArgValue::Num(0.0)],
            )
            .then(op("From Hex"), vec![ArgValue::Str("Auto".into())])
            .run_bytes(input.clone())
            .unwrap();
        assert_eq!(result, input);
    }

    #[test]
    fn type_coercion_bytes_to_text() {
        // "To Hex" outputs String, "To Hex" expects Bytes — coercion bridges gap
        let result = Pipeline::new()
            .then(
                op("To Hex"),
                vec![ArgValue::Str("None".into()), ArgValue::Num(0.0)],
            )
            .then(
                op("To Hex"),
                vec![ArgValue::Str("None".into()), ArgValue::Num(0.0)],
            )
            .run_text("A")
            .unwrap();
        // "A" → hex "41" → hex of "41" → "3431"
        assert_eq!(result, "3431");
    }

    #[test]
    fn aes_roundtrip_with_empty_iv_preserves_positions() {
        // Key=16 bytes, IV=empty (defaults to null), Mode=CBC
        // The empty IV at position 1 must NOT be compacted — Mode must stay at position 2.
        let key = ArgValue::Bytes(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
        let empty_iv = ArgValue::Str(String::new());
        let encrypt_args = vec![
            key.clone(),
            empty_iv.clone(),
            ArgValue::Str("CBC".into()),
            ArgValue::Str("Raw".into()),
            ArgValue::Str("Hex".into()),
            ArgValue::Str(String::new()),
        ];
        let decrypt_args = vec![
            key,
            empty_iv,
            ArgValue::Str("CBC".into()),
            ArgValue::Str("Hex".into()),
            ArgValue::Str("Raw".into()),
            ArgValue::Str(String::new()),
            ArgValue::Str(String::new()),
        ];

        let result = Pipeline::new()
            .then(op("AES Encrypt"), encrypt_args)
            .then(op("AES Decrypt"), decrypt_args)
            .run_bytes(b"Nonce test data!".to_vec())
            .unwrap();

        assert_eq!(result, b"Nonce test data!");
    }

    #[test]
    fn arbitrary_three_op_chain() {
        // To Upper Case → To Base64 → From Base64 → verify uppercase preserved
        let result = Pipeline::new()
            .then(op("To Upper Case"), vec![])
            .then(op("To Base64"), vec![ArgValue::Str("A-Za-z0-9+/=".into())])
            .then(
                op("From Base64"),
                vec![ArgValue::Str("A-Za-z0-9+/=".into()), ArgValue::Bool(false)],
            )
            .run_text("hello rxchef")
            .unwrap();
        assert_eq!(result, "HELLO RXCHEF");
    }
}
