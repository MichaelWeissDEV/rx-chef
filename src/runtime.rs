use crate::{
    operation::{
        ArgKind, ArgSchema, ArgValue, Availability, DataType, ImplementationStatus,
        InputRequirement, NumericBound, ParityStatus, SideEffect,
    },
    operations,
};

/// Structured failures produced by registry lookup, schema validation, and
/// operation dispatch. Frontends use the variant, never message inspection,
/// to select protocol errors and process exit codes.
#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("operation '{0}' was not found")]
    UnknownOperation(String),
    #[error(
        "operation '{operation}' is unavailable in this build; requires feature(s): {features}"
    )]
    Unavailable { operation: String, features: String },
    #[error("argument '{name}': {reason}")]
    InvalidArgument { name: String, reason: String },
    #[error("{0}")]
    Operation(#[from] crate::operation::OperationError),
    #[error("operation output violates its declared type: {0}")]
    OutputValidation(String),
}

#[derive(Debug, Clone)]
pub struct OperationInfo {
    pub name: &'static str,
    pub id: String,
    pub module: &'static str,
    pub description: &'static str,
    pub input_type: DataType,
    pub output_type: DataType,
    pub is_broken: bool,
    pub args: &'static [ArgSchema],
    pub input_requirement: InputRequirement,
    pub implementation_status: ImplementationStatus,
    pub availability: Availability,
    pub parity: ParityStatus,
    pub side_effects: &'static [SideEffect],
    pub deterministic: bool,
    pub feature_requirements: &'static [&'static str],
    pub known_limitations: &'static [&'static str],
}

pub fn operation_names(search: Option<&str>) -> Vec<String> {
    let mut names = operations::operation_names();
    if let Some(search) = search {
        let needle = search.to_lowercase();
        names.retain(|name| name.to_lowercase().contains(&needle));
    }
    names
}

pub fn operation_names_with_modules(search: Option<&str>) -> Result<Vec<(String, String)>, String> {
    operation_names(search)
        .into_iter()
        .map(|name| {
            let operation = operations::get_operation(&name)
                .ok_or_else(|| format!("registry returned unknown operation '{}'", name))?;
            Ok((operation.module().to_string(), name))
        })
        .collect()
}

pub fn operation_info(query: &str) -> Result<OperationInfo, String> {
    let canonical_name =
        resolve_operation_name(query).ok_or_else(|| not_found_message("operation", query))?;
    let operation = operations::get_operation(&canonical_name)
        .ok_or_else(|| not_found_message("operation", query))?;

    Ok(OperationInfo {
        name: operation.name(),
        id: canonical_identifier(operation.name()),
        module: operation.module(),
        description: operation.description(),
        input_type: operation.input_type(),
        output_type: operation.output_type(),
        is_broken: operation.is_broken(),
        args: operation.args_schema(),
        input_requirement: operation.input_requirement(),
        implementation_status: operation.implementation_status(),
        availability: operation.availability(),
        parity: operation.parity(),
        side_effects: operation.side_effects(),
        deterministic: operation.deterministic(),
        feature_requirements: operation.feature_requirements(),
        known_limitations: operation.known_limitations(),
    })
}

/// Source module identifier for audit and documentation tooling.
pub fn operation_source(query: &str) -> Result<&'static str, String> {
    let canonical_name =
        resolve_operation_name(query).ok_or_else(|| not_found_message("operation", query))?;
    operations::operation_source(&canonical_name)
        .ok_or_else(|| format!("registry has no source module for '{canonical_name}'"))
}

pub fn run_operation(
    operation_name: &str,
    input: Vec<u8>,
    raw_args: &[String],
) -> Result<Vec<u8>, RuntimeError> {
    let canonical_name = resolve_operation_name(operation_name)
        .ok_or_else(|| RuntimeError::UnknownOperation(operation_name.to_string()))?;
    let operation = operations::get_operation(&canonical_name)
        .ok_or_else(|| RuntimeError::UnknownOperation(operation_name.to_string()))?;
    if operation.availability() != Availability::Available {
        return Err(RuntimeError::Unavailable {
            operation: operation.name().to_string(),
            features: operation.feature_requirements().join(", "),
        });
    }
    let parsed_args = validate_operation_args(operation_name, raw_args)?;

    let output = operation
        .run(input, &parsed_args)
        .map_err(RuntimeError::Operation)?;
    crate::operation::OperationData::validate_raw(&output, operation.output_type())
        .map_err(|error| RuntimeError::OutputValidation(error.to_string()))?;
    Ok(output)
}

/// Validate and parse ordered arguments without executing an operation.
pub fn validate_operation_args(
    operation_name: &str,
    raw_args: &[String],
) -> Result<Vec<ArgValue>, RuntimeError> {
    let canonical_name = resolve_operation_name(operation_name)
        .ok_or_else(|| RuntimeError::UnknownOperation(operation_name.to_string()))?;
    let operation = operations::get_operation(&canonical_name)
        .ok_or_else(|| RuntimeError::UnknownOperation(operation_name.to_string()))?;
    let schema = operation.args_schema();
    if raw_args.len() > schema.len() {
        return Err(RuntimeError::InvalidArgument {
            name: "arguments".into(),
            reason: format!(
                "'{}' accepts {} value(s), but {} were provided",
                operation.name(),
                schema.len(),
                raw_args.len()
            ),
        });
    }
    schema
        .iter()
        .enumerate()
        .map(|(index, argument)| {
            let supplied = raw_args.get(index).map(String::as_str);
            if supplied.is_none() && argument.required {
                return Err(RuntimeError::InvalidArgument {
                    name: argument.name.into(),
                    reason: "value is required".into(),
                });
            }
            let raw = supplied.unwrap_or(argument.default_value);
            let value = parse_schema_argument(raw, argument.kind).map_err(|reason| {
                RuntimeError::InvalidArgument {
                    name: argument.name.into(),
                    reason,
                }
            })?;
            validate_schema_argument(raw, &value, argument).map_err(|reason| {
                RuntimeError::InvalidArgument {
                    name: argument.name.into(),
                    reason,
                }
            })?;
            Ok(value)
        })
        .collect::<Result<Vec<_>, _>>()
}

pub fn resolve_named_args(
    op_name: &str,
    named: &[String],
    positional: &[String],
) -> Result<Vec<String>, String> {
    let info = operation_info(op_name)?;
    let schema_len = info.args.len();
    if positional.len() > schema_len {
        return Err(format!(
            "'{}' accepts {} argument(s), but {} positional values were provided",
            info.name,
            schema_len,
            positional.len()
        ));
    }
    let mut result: Vec<String> = info
        .args
        .iter()
        .map(|argument| argument.default_value.to_string())
        .collect();
    for (index, value) in positional.iter().enumerate() {
        result[index] = value.clone();
    }
    let mut assigned = std::collections::HashSet::new();
    for kv in named {
        let (name, value) = kv
            .split_once('=')
            .ok_or_else(|| format!("invalid --arg '{}': expected NAME=VALUE", kv))?;
        let normalized_name = slugify(name);
        let idx = info
            .args
            .iter()
            .position(|argument| slugify(argument.name) == normalized_name)
            .ok_or_else(|| format!("argument '{}' not found in '{}'", name, op_name))?;
        if idx < positional.len() || !assigned.insert(idx) {
            return Err(format!(
                "argument '{}' was provided more than once for '{}'",
                info.args[idx].name, info.name
            ));
        }
        result[idx] = value.to_string();
    }
    Ok(result)
}

/// Redact argument positions explicitly marked sensitive by operation metadata.
pub fn redact_sensitive_args(operation: &str, arguments: &[String]) -> Vec<String> {
    let Ok(info) = operation_info(operation) else {
        return arguments.to_vec();
    };
    arguments
        .iter()
        .enumerate()
        .map(|(position, argument)| {
            if info.args.get(position).is_some_and(|arg| arg.sensitive) {
                "<redacted>".to_string()
            } else {
                argument.clone()
            }
        })
        .collect()
}

pub fn parse_operation_arg(raw: &str) -> Result<ArgValue, String> {
    if let Some(rest) = raw.strip_prefix("num:") {
        let number = rest
            .parse::<f64>()
            .map_err(|error| format!("invalid numeric argument '{}': {}", raw, error))?;
        if !number.is_finite() {
            return Err(format!(
                "invalid numeric argument '{}': must be finite",
                raw
            ));
        }
        return Ok(ArgValue::Num(number));
    }

    if let Some(rest) = raw.strip_prefix("bool:") {
        let value = match rest.to_ascii_lowercase().as_str() {
            "true" => true,
            "false" => false,
            _ => return Err(format!("invalid boolean argument '{}'", raw)),
        };
        return Ok(ArgValue::Bool(value));
    }

    if let Some(rest) = raw
        .strip_prefix("hex:")
        .or_else(|| raw.strip_prefix("bytes:"))
    {
        let cleaned = rest.replace([' ', '\n', '\r', '\t'], "");
        let cleaned = cleaned.trim_start_matches("0x");
        let bytes = hex::decode(cleaned)
            .map_err(|error| format!("invalid hex argument '{}': {}", raw, error))?;
        return Ok(ArgValue::Bytes(bytes));
    }

    Ok(ArgValue::Str(raw.to_string()))
}

fn parse_schema_argument(raw: &str, kind: ArgKind) -> Result<ArgValue, String> {
    if raw.starts_with("num:")
        || raw.starts_with("bool:")
        || raw.starts_with("hex:")
        || raw.starts_with("bytes:")
    {
        return parse_operation_arg(raw);
    }
    match kind {
        ArgKind::Boolean => match raw.to_ascii_lowercase().as_str() {
            "true" | "false" => Ok(ArgValue::Str(raw.to_string())),
            _ => Err(format!("expected boolean, got '{raw}'")),
        },
        ArgKind::Integer | ArgKind::UnsignedInteger => {
            let number = raw
                .parse::<i128>()
                .map_err(|error| format!("expected integer: {error}"))?;
            if kind == ArgKind::UnsignedInteger && number < 0 {
                return Err("expected an unsigned integer".into());
            }
            const MAX_EXACT_F64_INTEGER: i128 = 9_007_199_254_740_992;
            if !(-MAX_EXACT_F64_INTEGER..=MAX_EXACT_F64_INTEGER).contains(&number) {
                return Err("integer is outside the exact legacy numeric range".into());
            }
            Ok(ArgValue::Str(raw.to_string()))
        }
        ArgKind::Float => {
            let number = raw
                .parse::<f64>()
                .map_err(|error| format!("expected number: {error}"))?;
            if !number.is_finite() {
                return Err("number must be finite".into());
            }
            Ok(ArgValue::Str(raw.to_string()))
        }
        _ => Ok(ArgValue::Str(raw.to_string())),
    }
}

fn numeric_bound(bound: NumericBound) -> f64 {
    match bound {
        NumericBound::Integer(value) => value as f64,
        NumericBound::Unsigned(value) => value as f64,
        NumericBound::Float(value) => value,
    }
}

fn validate_schema_argument(raw: &str, value: &ArgValue, schema: &ArgSchema) -> Result<(), String> {
    if !schema.choices.is_empty()
        && !schema
            .choices
            .iter()
            .any(|choice| choice.eq_ignore_ascii_case(raw))
    {
        return Err(format!("expected one of: {}", schema.choices.join(", ")));
    }
    if schema.minimum.is_some() || schema.maximum.is_some() {
        let number = value
            .as_f64()
            .ok_or_else(|| "expected a finite numeric value".to_string())?;
        if let Some(minimum) = schema.minimum {
            if number < numeric_bound(minimum) {
                return Err(format!("must be at least {minimum}"));
            }
        }
        if let Some(maximum) = schema.maximum {
            if number > numeric_bound(maximum) {
                return Err(format!("must be at most {maximum}"));
            }
        }
    }
    Ok(())
}

/// Stable snake-case identifier used by registry and argument normalization.
pub fn canonical_identifier(value: &str) -> String {
    let mut output = String::new();
    let mut separator = false;
    for character in value.chars() {
        if character.is_alphanumeric() {
            if separator && !output.is_empty() {
                output.push('_');
            }
            output.extend(character.to_lowercase());
            separator = false;
        } else {
            separator = true;
        }
    }
    output
}

pub fn data_type_name(data_type: DataType) -> &'static str {
    match data_type {
        DataType::String => "String",
        DataType::Binary => "Binary",
        DataType::Number => "Number",
        DataType::Json => "JSON",
        DataType::Html => "HTML",
        DataType::Bytes => "Bytes",
    }
}

pub fn display_default(value: &str) -> &str {
    if value.is_empty() {
        "<empty>"
    } else {
        value
    }
}

pub fn not_found_message(kind: &str, name: &str) -> String {
    let mut message = format!("{} '{}' was not found", kind, name);
    let matches = operations::operation_names()
        .iter()
        .filter(|candidate| candidate.to_lowercase().contains(&name.to_lowercase()))
        .take(8)
        .cloned()
        .collect::<Vec<_>>();

    if !matches.is_empty() {
        message.push_str(". Similar entries: ");
        message.push_str(&matches.join(", "));
    }

    message
}

pub fn resolve_operation_name(query: &str) -> Option<String> {
    let names = operations::operation_names();
    // 1. Exact match
    names
        .iter()
        .find(|n| *n == query)
        .cloned()
        // 2. Case-insensitive exact match ("to hex" → "To Hex")
        .or_else(|| {
            names
                .iter()
                .find(|n| n.eq_ignore_ascii_case(query))
                .cloned()
        })
        // 3. Slug match: "to_hex", "ToHex", "to-hex" → "To Hex"
        .or_else(|| {
            let q = slugify(query);
            names.iter().find(|n| slugify(n) == q).cloned()
        })
}

/// Strip non-alphanumeric chars and lowercase — makes "to_hex", "ToHex",
/// "to-hex", "toHex" all collapse to "tohex" for fuzzy matching.
fn slugify(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{
        canonical_identifier, operation_info, operation_names, parse_operation_arg,
        resolve_named_args, resolve_operation_name,
    };
    use crate::operation::ArgValue;

    #[test]
    fn every_registered_operation_has_complete_unique_metadata() {
        let names = operation_names(None);
        assert!(!names.is_empty());
        let mut unique = HashSet::new();
        let mut unique_ids = HashSet::new();
        for name in names {
            assert!(
                unique.insert(name.clone()),
                "duplicate operation name: {name}"
            );
            let info = operation_info(&name).unwrap();
            assert!(
                unique_ids.insert(info.id.clone()),
                "duplicate normalized operation id: {}",
                info.id
            );
            assert!(!info.name.trim().is_empty(), "empty name for {name}");
            assert!(!info.module.trim().is_empty(), "empty module for {name}");
            assert!(
                !info.description.trim().is_empty(),
                "empty description for {name}"
            );
            let mut arg_names = HashSet::new();
            for arg in info.args {
                assert!(
                    !arg.name.trim().is_empty(),
                    "empty argument name for {name}"
                );
                assert!(
                    !arg.description.trim().is_empty(),
                    "empty description for {name} argument {}",
                    arg.name
                );
                assert!(
                    arg_names.insert(canonical_identifier(arg.name)),
                    "duplicate argument '{}' for {name}",
                    arg.name
                );
            }
        }
    }

    #[test]
    fn operation_names_accept_cli_friendly_spellings() {
        for alias in ["To Hex", "to hex", "to_hex", "to-hex", "ToHex"] {
            assert_eq!(resolve_operation_name(alias).as_deref(), Some("To Hex"));
        }
    }

    #[test]
    fn typed_argument_prefixes_are_parsed() {
        assert!(matches!(parse_operation_arg("num:12.5"), Ok(ArgValue::Num(n)) if n == 12.5));
        assert!(matches!(
            parse_operation_arg("bool:TRUE"),
            Ok(ArgValue::Bool(true))
        ));
        assert!(matches!(parse_operation_arg("hex:48 69"), Ok(ArgValue::Bytes(v)) if v == b"Hi"));
        assert!(matches!(parse_operation_arg("plain"), Ok(ArgValue::Str(v)) if v == "plain"));
        assert!(parse_operation_arg("num:NaN").is_err());
        assert!(parse_operation_arg("num:inf").is_err());
    }

    #[test]
    fn named_arguments_reject_duplicates_and_fill_defaults() {
        let args =
            resolve_named_args("From Base64", &["Strict-mode=true".to_string()], &[]).unwrap();
        assert_eq!(args, ["A-Za-z0-9+/=", "true", "true"]);

        let duplicate = resolve_named_args(
            "From Base64",
            &[
                "Strict mode=true".to_string(),
                "strict_mode=false".to_string(),
            ],
            &[],
        );
        assert!(duplicate.is_err());

        let positional_and_named = resolve_named_args(
            "From Base64",
            &["Alphabet=custom".to_string()],
            &["standard".to_string()],
        );
        assert!(positional_and_named.is_err());
    }
}
