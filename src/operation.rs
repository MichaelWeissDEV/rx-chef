/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Shared operation traits, argument types, and runtime errors.
 * -----------------------------------------------------------------------------
 */

use serde_json::Value as JsonValue;
use std::fmt;

/// Machine-readable argument value category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArgKind {
    String,
    Integer,
    UnsignedInteger,
    Float,
    Boolean,
    Bytes,
    HexBytes,
    Base64Bytes,
    Enum,
    Regex,
    Path,
    Url,
}

/// Whether an operation consumes input bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputRequirement {
    Required,
    Optional,
    Ignored,
}

/// Verification state of an operation implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationStatus {
    Complete,
    Partial,
    Unsupported,
    FeatureGated,
    Experimental,
}

/// Compatibility level with upstream CyberChef behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParityStatus {
    Exact,
    Compatible,
    IntentionalDifference,
    Unknown,
    NotApplicable,
}

/// Externally observable behavior an operation may perform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SideEffect {
    Network,
    FilesystemRead,
    FilesystemWrite,
    Random,
    Time,
    ExternalProcess,
    NativeLibrary,
}

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
    #[deprecated(note = "use OperationData::from_raw_strict in execution paths")]
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

    /// Construct typed data while enforcing the operation's declared output
    /// contract.
    ///
    /// # Errors
    ///
    /// Returns [`OperationError::ProcessingError`] when text, JSON, or numeric
    /// output is not valid for the declared [`DataType`].
    pub fn from_raw_strict(bytes: Vec<u8>, dtype: DataType) -> Result<Self, OperationError> {
        match dtype {
            DataType::String | DataType::Html => String::from_utf8(bytes)
                .map(OperationData::Text)
                .map_err(|error| {
                    OperationError::ProcessingError(format!(
                        "operation declared UTF-8 output but produced invalid UTF-8: {error}"
                    ))
                }),
            DataType::Json => serde_json::from_slice(&bytes)
                .map(OperationData::Json)
                .map_err(|error| {
                    OperationError::ProcessingError(format!(
                        "operation declared JSON output but produced invalid JSON: {error}"
                    ))
                }),
            DataType::Number => {
                let text = String::from_utf8(bytes).map_err(|error| {
                    OperationError::ProcessingError(format!(
                        "operation declared numeric output but produced invalid UTF-8: {error}"
                    ))
                })?;
                let number = text.trim().parse::<f64>().map_err(|error| {
                    OperationError::ProcessingError(format!(
                        "operation declared numeric output but produced a non-number: {error}"
                    ))
                })?;
                if !number.is_finite() {
                    return Err(OperationError::ProcessingError(
                        "operation declared numeric output but produced a non-finite number".into(),
                    ));
                }
                Ok(OperationData::Number(number))
            }
            DataType::Bytes | DataType::Binary => Ok(OperationData::Bytes(bytes)),
        }
    }

    /// Validate raw bytes against a declared output type without changing the
    /// original byte representation.
    pub fn validate_raw(bytes: &[u8], dtype: DataType) -> Result<(), OperationError> {
        Self::from_raw_strict(bytes.to_vec(), dtype).map(|_| ())
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
     * - Str uses UTF-8 text by default. `hex:`/`bytes:` and `base64:` prefixes
     *   request explicit decoding; `text:` forces literal text.
     * - Num and Bool are converted to their string or single-byte forms.
     *
     * @param arg The argument value to convert.
     * @return Result containing the byte vector or an error.
     */
    pub fn convert_to_byte_array(arg: &ArgValue) -> Result<Vec<u8>, OperationError> {
        match arg {
            ArgValue::Bytes(b) => Ok(b.clone()),
            ArgValue::Str(s) => {
                if s.is_empty() {
                    return Ok(Vec::new());
                }
                if let Some(text) = s.strip_prefix("text:") {
                    return Ok(text.as_bytes().to_vec());
                }
                if let Some(encoded) = s.strip_prefix("hex:").or_else(|| s.strip_prefix("bytes:")) {
                    let cleaned = encoded.replace([' ', '\n', '\r', '\t'], "");
                    return hex::decode(cleaned).map_err(|error| OperationError::InvalidArgument {
                        name: "Argument".to_string(),
                        reason: format!("invalid hex bytes: {error}"),
                    });
                }
                if let Some(encoded) = s.strip_prefix("base64:") {
                    use base64::{engine::general_purpose, Engine as _};
                    return general_purpose::STANDARD.decode(encoded).map_err(|error| {
                        OperationError::InvalidArgument {
                            name: "Argument".to_string(),
                            reason: format!("invalid Base64 bytes: {error}"),
                        }
                    });
                }
                Ok(s.as_bytes().to_vec())
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
        match self {
            ArgValue::Num(number) if number.is_finite() => Some(*number),
            ArgValue::Str(value) => value
                .parse::<f64>()
                .ok()
                .filter(|number| number.is_finite()),
            _ => None,
        }
    }

    pub fn as_usize(&self) -> Option<usize> {
        let number = self.as_f64()?;
        if !number.is_finite()
            || number.fract() != 0.0
            || number < 0.0
            || number > usize::MAX as f64
        {
            return None;
        }
        Some(number as usize)
    }

    pub fn as_i64(&self) -> Option<i64> {
        let number = self.as_f64()?;
        if !number.is_finite()
            || number.fract() != 0.0
            || number < i64::MIN as f64
            || number > i64::MAX as f64
        {
            return None;
        }
        Some(number as i64)
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            ArgValue::Bool(value) => Some(*value),
            ArgValue::Str(value) if value.eq_ignore_ascii_case("true") => Some(true),
            ArgValue::Str(value) if value.eq_ignore_ascii_case("false") => Some(false),
            _ => None,
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

    /// Whether the operation consumes its input.
    fn input_requirement(&self) -> InputRequirement {
        InputRequirement::Required
    }

    /// Current verified implementation state. The conservative default is
    /// `Partial`; individual operations are promoted only after their release
    /// quality gates are mapped and checked.
    fn status(&self) -> OperationStatus {
        if self.is_broken() {
            OperationStatus::FeatureGated
        } else {
            OperationStatus::Partial
        }
    }

    /// Verified upstream compatibility state.
    fn parity(&self) -> ParityStatus {
        ParityStatus::Unknown
    }

    /// Side effects beyond transforming the supplied bytes.
    fn side_effects(&self) -> &'static [SideEffect] {
        &[]
    }

    /// Whether equal inputs and arguments produce equal outputs.
    fn deterministic(&self) -> bool {
        true
    }

    /// Cargo features required for availability.
    fn feature_requirements(&self) -> &'static [&'static str] {
        &[]
    }

    /// Known behavioral limitations suitable for generated documentation.
    fn known_limitations(&self) -> &'static [&'static str] {
        &[]
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
        OperationData::from_raw_strict(output, self.output_type())
    }
}

#[cfg(test)]
mod tests {
    use super::{ArgValue, DataType, OperationData};

    #[test]
    fn strict_raw_data_rejects_broken_type_contracts() {
        assert!(OperationData::from_raw_strict(vec![0xff], DataType::String).is_err());
        assert!(OperationData::from_raw_strict(b"not json".to_vec(), DataType::Json).is_err());
        assert!(OperationData::from_raw_strict(b"NaN".to_vec(), DataType::Number).is_err());
        assert!(OperationData::from_raw_strict(b"42.5".to_vec(), DataType::Number).is_ok());
    }

    #[test]
    fn integer_conversions_are_checked() {
        for number in [f64::NAN, f64::INFINITY, -1.0, 1.5] {
            assert_eq!(ArgValue::Num(number).as_usize(), None);
        }
        assert_eq!(ArgValue::Num(42.0).as_usize(), Some(42));
        assert_eq!(ArgValue::Num(1.5).as_i64(), None);
        assert_eq!(ArgValue::Num(-1.0).as_i64(), Some(-1));
    }

    #[test]
    fn byte_arguments_never_guess_hex_from_text() {
        use super::Utils;

        assert_eq!(
            Utils::convert_to_byte_array(&ArgValue::Str("deadbeef".into())).unwrap(),
            b"deadbeef"
        );
        assert_eq!(
            Utils::convert_to_byte_array(&ArgValue::Str("hex:deadbeef".into())).unwrap(),
            [0xde, 0xad, 0xbe, 0xef]
        );
        assert_eq!(
            Utils::convert_to_byte_array(&ArgValue::Str("base64:SGk=".into())).unwrap(),
            b"Hi"
        );
    }
}
