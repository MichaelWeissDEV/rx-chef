/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.1.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Shared operation traits, argument types, and runtime errors.
 * -----------------------------------------------------------------------------
 */

use serde_json::Value as JsonValue;
use std::fmt;

/**
 * @enum DataType
 * @brief The data type flowing into or out of an operation.
 */
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataType {
    String,
    Binary,
    Number,
    Json,
    Html,
    Bytes,
}

/// Typed data that flows through a [`Pipeline`].
///
/// Every operation receives and produces `OperationData`.  The pipeline
/// automatically coerces the output of one step to the [`DataType`] expected
/// by the next step via [`OperationData::coerce_to`].
#[derive(Debug, Clone)]
pub enum OperationData {
    Bytes(Vec<u8>),
    Text(String),
    Number(f64),
    Json(JsonValue),
}

impl OperationData {
    /// The [`DataType`] variant that represents this data.
    pub fn data_type(&self) -> DataType {
        match self {
            OperationData::Bytes(_) => DataType::Bytes,
            OperationData::Text(_) => DataType::String,
            OperationData::Number(_) => DataType::Number,
            OperationData::Json(_) => DataType::Json,
        }
    }

    /// Coerce `self` to `target`, converting between representations.
    ///
    /// If `self` is already the right type, no work is done.  Conversion
    /// failures (e.g. invalid UTF-8, non-numeric text) surface as
    /// [`OperationError::InvalidInput`].
    pub fn coerce_to(self, target: DataType) -> Result<Self, OperationError> {
        // DataType::Html and DataType::Binary are aliases for String / Bytes.
        let target = match target {
            DataType::Html => DataType::String,
            DataType::Binary => DataType::Bytes,
            other => other,
        };
        // Normalise our own type too.
        let self_type = match self.data_type() {
            DataType::Html => DataType::String,
            DataType::Binary => DataType::Bytes,
            t => t,
        };
        if self_type == target {
            return Ok(self);
        }
        match (self, target) {
            // --- Bytes → * ---
            (OperationData::Bytes(b), DataType::String) => {
                String::from_utf8(b).map(OperationData::Text).map_err(|e| {
                    OperationError::InvalidInput(format!("bytes are not valid UTF-8: {e}"))
                })
            }
            (OperationData::Bytes(b), DataType::Number) => {
                let s = String::from_utf8_lossy(&b).into_owned();
                s.trim()
                    .parse::<f64>()
                    .map(OperationData::Number)
                    .map_err(|e| {
                        OperationError::InvalidInput(format!("cannot parse as number: {e}"))
                    })
            }
            (OperationData::Bytes(b), DataType::Json) => serde_json::from_slice(&b)
                .map(OperationData::Json)
                .map_err(|e| OperationError::InvalidInput(format!("cannot parse as JSON: {e}"))),

            // --- Text → * ---
            (OperationData::Text(s), DataType::Bytes) => Ok(OperationData::Bytes(s.into_bytes())),
            (OperationData::Text(s), DataType::Number) => s
                .trim()
                .parse::<f64>()
                .map(OperationData::Number)
                .map_err(|e| OperationError::InvalidInput(format!("cannot parse as number: {e}"))),
            (OperationData::Text(s), DataType::Json) => serde_json::from_str(&s)
                .map(OperationData::Json)
                .map_err(|e| OperationError::InvalidInput(format!("cannot parse as JSON: {e}"))),

            // --- Number → * ---
            (OperationData::Number(n), DataType::Bytes) => {
                Ok(OperationData::Bytes(n.to_string().into_bytes()))
            }
            (OperationData::Number(n), DataType::String) => Ok(OperationData::Text(n.to_string())),
            (OperationData::Number(n), DataType::Json) => {
                Ok(OperationData::Json(serde_json::json!(n)))
            }

            // --- Json → * ---
            (OperationData::Json(v), DataType::Bytes) => serde_json::to_vec(&v)
                .map(OperationData::Bytes)
                .map_err(|e| OperationError::ProcessingError(e.to_string())),
            (OperationData::Json(v), DataType::String) => serde_json::to_string(&v)
                .map(OperationData::Text)
                .map_err(|e| OperationError::ProcessingError(e.to_string())),
            (OperationData::Json(v), DataType::Number) => v
                .as_f64()
                .map(OperationData::Number)
                .ok_or_else(|| OperationError::InvalidInput("JSON value is not a number".into())),

            (data, target) => Err(OperationError::InvalidInput(format!(
                "cannot coerce {:?} to {target:?}",
                data.data_type()
            ))),
        }
    }

    /// Consume `self` and return the raw bytes.
    pub fn into_bytes(self) -> Result<Vec<u8>, OperationError> {
        match self {
            OperationData::Bytes(b) => Ok(b),
            OperationData::Text(s) => Ok(s.into_bytes()),
            OperationData::Number(n) => Ok(n.to_string().into_bytes()),
            OperationData::Json(v) => {
                serde_json::to_vec(&v).map_err(|e| OperationError::ProcessingError(e.to_string()))
            }
        }
    }

    /// Consume `self` and return a UTF-8 string.
    pub fn into_text(self) -> Result<String, OperationError> {
        match self {
            OperationData::Text(s) => Ok(s),
            OperationData::Bytes(b) => String::from_utf8(b).map_err(|e| {
                OperationError::InvalidInput(format!("bytes are not valid UTF-8: {e}"))
            }),
            OperationData::Number(n) => Ok(n.to_string()),
            OperationData::Json(v) => serde_json::to_string(&v)
                .map_err(|e| OperationError::ProcessingError(e.to_string())),
        }
    }

    /// Construct `OperationData` from raw bytes, attempting to parse into the
    /// declared `DataType`.  Falls back to `Bytes` if parsing fails, so this
    /// method is infallible.
    pub fn from_raw(bytes: Vec<u8>, dtype: DataType) -> Self {
        match dtype {
            DataType::String | DataType::Html => String::from_utf8(bytes.clone())
                .map(OperationData::Text)
                .unwrap_or(OperationData::Bytes(bytes)),
            DataType::Json => serde_json::from_slice(&bytes)
                .map(OperationData::Json)
                .unwrap_or(OperationData::Bytes(bytes)),
            DataType::Number => {
                let s = String::from_utf8_lossy(&bytes).into_owned();
                s.trim()
                    .parse::<f64>()
                    .map(OperationData::Number)
                    .unwrap_or(OperationData::Bytes(bytes))
            }
            DataType::Bytes | DataType::Binary => OperationData::Bytes(bytes),
        }
    }
}

impl From<Vec<u8>> for OperationData {
    fn from(b: Vec<u8>) -> Self {
        OperationData::Bytes(b)
    }
}

impl From<String> for OperationData {
    fn from(s: String) -> Self {
        OperationData::Text(s)
    }
}

impl From<&str> for OperationData {
    fn from(s: &str) -> Self {
        OperationData::Text(s.to_owned())
    }
}

impl From<f64> for OperationData {
    fn from(n: f64) -> Self {
        OperationData::Number(n)
    }
}

impl From<JsonValue> for OperationData {
    fn from(v: JsonValue) -> Self {
        OperationData::Json(v)
    }
}

/**
 * @class Utils
 * @brief Small utility helpers used by some operations.
 */
pub struct Utils;

impl Utils {
    /**
     * @brief Convert an ArgValue to a byte vector.
     *
     * - Bytes returns the inner bytes.
     * - Str attempts to decode hex (even-length hex digits) after removing
     *   spaces/newlines; on failure it returns the UTF-8 bytes of the string.
     * - Num and Bool are converted to their string or single-byte forms.
     *
     * @param arg The argument value to convert.
     * @return Result containing the byte vector or an error.
     */
    pub fn convert_to_byte_array(arg: &ArgValue) -> Result<Vec<u8>, OperationError> {
        match arg {
            ArgValue::Bytes(b) => Ok(b.clone()),
            ArgValue::Str(s) => {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    return Ok(Vec::new());
                }
                let cleaned = trimmed.replace([' ', '\n', '\r', '\t'], "");
                if cleaned.len() % 2 == 0 && cleaned.chars().all(|c| c.is_ascii_hexdigit()) {
                    match hex::decode(&cleaned) {
                        Ok(v) => Ok(v),
                        Err(e) => Err(OperationError::InvalidArgument {
                            name: "Argument".to_string(),
                            reason: format!("Invalid hex: {}", e),
                        }),
                    }
                } else {
                    Ok(s.as_bytes().to_vec())
                }
            }
            ArgValue::Num(n) => Ok(n.to_string().into_bytes()),
            ArgValue::Bool(b) => Ok(vec![if *b { 1 } else { 0 }]),
        }
    }
}

/**
 * @enum ArgValue
 * @brief A runtime value supplied to an operation argument.
 */
#[derive(Debug, Clone)]
pub enum ArgValue {
    Str(String),
    Num(f64),
    Bool(bool),
    Bytes(Vec<u8>),
}

impl ArgValue {
    pub fn as_str(&self) -> Option<&str> {
        if let ArgValue::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        if let ArgValue::Num(n) = self {
            Some(*n)
        } else {
            None
        }
    }

    pub fn as_usize(&self) -> Option<usize> {
        self.as_f64().map(|n| n as usize)
    }

    pub fn as_i64(&self) -> Option<i64> {
        self.as_f64().map(|n| n as i64)
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let ArgValue::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    pub fn as_bytes(&self) -> Option<&[u8]> {
        if let ArgValue::Bytes(b) = self {
            Some(b)
        } else {
            None
        }
    }
}

impl fmt::Display for ArgValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgValue::Str(s) => write!(f, "{}", s),
            ArgValue::Num(n) => write!(f, "{}", n),
            ArgValue::Bool(b) => write!(f, "{}", b),
            ArgValue::Bytes(b) => write!(f, "{}", hex::encode(b)),
        }
    }
}

/**
 * @struct ArgSchema
 * @brief Static description of one accepted argument.
 */
#[derive(Debug, Clone)]
pub struct ArgSchema {
    pub name: &'static str,
    pub description: &'static str,
    pub default_value: &'static str,
}

/**
 * @enum OperationError
 * @brief Errors returned by operations at runtime.
 */
#[derive(Debug, thiserror::Error)]
pub enum OperationError {
    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("invalid argument '{name}': {reason}")]
    InvalidArgument { name: String, reason: String },

    #[error("processing failed: {0}")]
    ProcessingError(String),
}

/**
 * @trait Operation
 * @brief Every ported CyberChef operation must implement this trait.
 *
 * Implementations are expected to be zero-size structs; state is never stored
 * across `run` calls.  All argument access is positional: `args[n]` matches
 * `args_schema()[n]`.
 */
pub trait Operation: Send + Sync {
    /**
     * @brief Short human-readable name shown in the recipe editor (e.g. "AES Decrypt").
     */
    fn name(&self) -> &'static str;

    /**
     * @brief Top-level category (e.g. "Ciphers", "Encodings", "Hashing").
     */
    fn module(&self) -> &'static str;

    /**
     * @brief One-paragraph description of what the operation does.
     */
    fn description(&self) -> &'static str;

    /**
     * @brief Ordered list of argument descriptors.
     */
    fn args_schema(&self) -> &'static [ArgSchema];

    /**
     * @brief Execute the operation.
     *
     * @param input carries raw bytes; cast to String::from_utf8_lossy when a text representation is needed.
     * @param args are positional and correspond index-for-index with args_schema().
     * @return Result containing the output bytes or an OperationError.
     */
    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError>;

    /**
     * @brief Expected input data type (hint for the recipe editor UI).
     */
    fn input_type(&self) -> DataType {
        DataType::String
    }

    /**
     * @brief Produced output data type (hint for the recipe editor UI).
     */
    fn output_type(&self) -> DataType {
        DataType::String
    }

    /**
     * @brief Returns true if this operation is known broken or experimental.
     */
    fn is_broken(&self) -> bool {
        false
    }

    /// Execute the operation with typed input/output, suitable for use in a [`Pipeline`].
    ///
    /// The default implementation coerces `input` to the type declared by
    /// [`Operation::input_type`], delegates to [`Operation::run`], then wraps
    /// the raw output bytes as [`OperationData`] according to
    /// [`Operation::output_type`].
    ///
    /// Override this method if the operation natively understands structured
    /// data and can avoid the bytes round-trip.
    fn run_typed(
        &self,
        input: OperationData,
        args: &[ArgValue],
    ) -> Result<OperationData, OperationError> {
        let bytes = input.coerce_to(self.input_type())?.into_bytes()?;
        let output = self.run(bytes, args)?;
        Ok(OperationData::from_raw(output, self.output_type()))
    }
}
