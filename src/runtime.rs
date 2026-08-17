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
    /// A required argument was not supplied at all.
    ///
    /// Distinct from [`RuntimeError::InvalidArgument`] so frontends can tell
    /// "you forgot this" from "what you gave me is wrong".
    #[error("operation '{operation}' requires a value for argument '{name}'")]
    MissingArgument { operation: String, name: String },
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
            // A recipe may simply stop short of the schema length; every slot
            // past the end of `raw_args` was not supplied.
            let supplied = raw_args.get(index).map(String::as_str);
            if supplied.is_none() && argument.required {
                return Err(RuntimeError::MissingArgument {
                    operation: operation.name().to_string(),
                    name: argument.name.into(),
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

/// Bind supplied positional and named values to an operation's schema.
///
/// # Why this keeps `Option` internally
///
/// "the caller did not supply this argument" and "the caller supplied the
/// empty string" are different facts, and only the first one may be answered
/// with a default. Materialising defaults into every slot up front — which is
/// what this function used to do — erases that distinction before
/// [`ArgSchema::required`] is ever consulted, so a required argument could
/// never be reported as missing. `rxchef run "AES Encrypt"` then reached the
/// cipher with an empty key and failed with "Invalid key length: 0 bytes"
/// instead of naming the argument the user forgot.
///
/// Slots therefore stay `None` until every supplied value has been placed and
/// the required check has run. Defaults are substituted only afterwards.
///
/// An explicitly supplied empty string counts as supplied: `required` guards
/// absence, not emptiness. Whether an empty value is *acceptable* is the
/// operation's own business.
pub fn bind_arguments(
    operation_name: &str,
    named: &[String],
    positional: &[String],
) -> Result<Vec<String>, RuntimeError> {
    let canonical_name = resolve_operation_name(operation_name)
        .ok_or_else(|| RuntimeError::UnknownOperation(operation_name.to_string()))?;
    let operation = operations::get_operation(&canonical_name)
        .ok_or_else(|| RuntimeError::UnknownOperation(operation_name.to_string()))?;
    let schema = operation.args_schema();

    // 1. One empty slot per schema argument.
    let mut slots: Vec<Option<String>> = vec![None; schema.len()];

    // 2. Positional values, left to right.
    if positional.len() > schema.len() {
        return Err(RuntimeError::InvalidArgument {
            name: "arguments".into(),
            reason: format!(
                "'{}' accepts {} argument(s), but {} positional values were provided",
                operation.name(),
                schema.len(),
                positional.len()
            ),
        });
    }
    for (index, value) in positional.iter().enumerate() {
        slots[index] = Some(value.clone());
    }

    // 3. Named values, rejecting unknown names and 4. duplicate assignments.
    for entry in named {
        let (name, value) = entry
            .split_once('=')
            .ok_or_else(|| RuntimeError::InvalidArgument {
                name: entry.clone(),
                reason: "expected NAME=VALUE".into(),
            })?;
        let normalized = slugify(name);
        let index = schema
            .iter()
            .position(|argument| slugify(argument.name) == normalized)
            .ok_or_else(|| RuntimeError::InvalidArgument {
                name: name.to_string(),
                reason: format!("'{}' has no such argument", operation.name()),
            })?;
        if slots[index].is_some() {
            return Err(RuntimeError::InvalidArgument {
                name: schema[index].name.into(),
                reason: format!("provided more than once for '{}'", operation.name()),
            });
        }
        slots[index] = Some(value.to_string());
    }

    // 5. Required check first, defaults only for absent optional arguments.
    schema
        .iter()
        .zip(slots)
        .map(|(argument, slot)| match slot {
            Some(value) => Ok(value),
            None if argument.required => Err(RuntimeError::MissingArgument {
                operation: operation.name().to_string(),
                name: argument.name.into(),
            }),
            // 6. Only now is a default allowed to stand in for a value.
            None => Ok(argument.default_value.to_string()),
        })
        .collect()
}

/// Backwards-compatible wrapper returning a plain message.
///
/// Prefer [`bind_arguments`], which returns a structured [`RuntimeError`] the
/// frontends can map to an exit code.
pub fn resolve_named_args(
    op_name: &str,
    named: &[String],
    positional: &[String],
) -> Result<Vec<String>, String> {
    bind_arguments(op_name, named, positional).map_err(|error| error.to_string())
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
        bind_arguments, canonical_identifier, operation_info, operation_names, parse_operation_arg,
        resolve_named_args, resolve_operation_name, validate_operation_args, RuntimeError,
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

    // ── Required-argument binding ──────────────────────────────────────────
    //
    // `bind_arguments` must not substitute a default before the required check
    // has run, otherwise a missing value is indistinguishable from an empty
    // one. "AES Encrypt" is the canonical case: its `Key` is required and its
    // default is empty, so the old behaviour reached the cipher with a
    // zero-length key and reported an invalid key *length*.

    fn missing_argument(error: &RuntimeError) -> Option<(&str, &str)> {
        match error {
            RuntimeError::MissingArgument { operation, name } => {
                Some((operation.as_str(), name.as_str()))
            }
            _ => None,
        }
    }

    #[test]
    fn missing_required_positional_argument_is_reported_by_name() {
        let error = bind_arguments("AES Encrypt", &[], &[]).unwrap_err();
        assert_eq!(missing_argument(&error), Some(("AES Encrypt", "Key")));
    }

    #[test]
    fn missing_required_named_argument_is_reported_by_name() {
        // Supplying only an optional argument by name leaves `Key` absent.
        let error = bind_arguments("AES Encrypt", &["Mode=CBC".to_string()], &[]).unwrap_err();
        assert_eq!(missing_argument(&error), Some(("AES Encrypt", "Key")));
    }

    #[test]
    fn supplied_required_argument_binds_and_fills_remaining_defaults() {
        let bound = bind_arguments(
            "AES Encrypt",
            &[],
            &["hex:00112233445566778899aabbccddeeff".to_string()],
        )
        .unwrap();
        assert_eq!(bound[0], "hex:00112233445566778899aabbccddeeff");
        // Every remaining slot is filled from the schema defaults.
        let info = operation_info("AES Encrypt").unwrap();
        assert_eq!(bound.len(), info.args.len());
        for (index, argument) in info.args.iter().enumerate().skip(1) {
            assert_eq!(bound[index], argument.default_value);
        }
    }

    #[test]
    fn explicit_empty_string_counts_as_supplied() {
        // `required` guards absence, not emptiness: an explicit "" is a value
        // the user chose, so binding succeeds and the operation decides
        // whether it is acceptable.
        let bound = bind_arguments("AES Encrypt", &[], &[String::new()]).unwrap();
        assert_eq!(bound[0], "");
    }

    #[test]
    fn absent_optional_argument_receives_its_default() {
        let bound = bind_arguments("From Base64", &[], &[]).unwrap();
        let info = operation_info("From Base64").unwrap();
        for (index, argument) in info.args.iter().enumerate() {
            assert_eq!(bound[index], argument.default_value);
        }
    }

    #[test]
    fn named_argument_may_follow_an_omitted_optional_argument() {
        // "Strict mode" sits after "Alphabet"; naming it must not require
        // positionally filling the arguments before it.
        let bound = bind_arguments("From Base64", &["Strict mode=true".to_string()], &[]).unwrap();
        let info = operation_info("From Base64").unwrap();
        assert_eq!(bound[0], info.args[0].default_value);
        assert_eq!(bound[1], "true");
    }

    #[test]
    fn unknown_named_argument_is_rejected() {
        let error =
            bind_arguments("From Base64", &["NoSuchArgument=1".to_string()], &[]).unwrap_err();
        assert!(
            matches!(&error, RuntimeError::InvalidArgument { name, .. } if name == "NoSuchArgument"),
            "expected InvalidArgument naming the unknown argument, got {error:?}"
        );
    }

    #[test]
    fn named_argument_without_a_separator_is_rejected() {
        let error = bind_arguments("From Base64", &["Alphabet".to_string()], &[]).unwrap_err();
        assert!(
            matches!(&error, RuntimeError::InvalidArgument { reason, .. } if reason.contains("NAME=VALUE")),
            "expected a NAME=VALUE hint, got {error:?}"
        );
    }

    #[test]
    fn duplicate_named_argument_is_rejected() {
        let error = bind_arguments(
            "From Base64",
            &[
                "Strict mode=true".to_string(),
                "strict_mode=false".to_string(),
            ],
            &[],
        )
        .unwrap_err();
        assert!(
            matches!(&error, RuntimeError::InvalidArgument { reason, .. } if reason.contains("more than once")),
            "expected a duplicate-assignment error, got {error:?}"
        );
    }

    #[test]
    fn named_argument_colliding_with_a_positional_one_is_rejected() {
        let error = bind_arguments(
            "From Base64",
            &["Alphabet=custom".to_string()],
            &["standard".to_string()],
        )
        .unwrap_err();
        assert!(
            matches!(&error, RuntimeError::InvalidArgument { reason, .. } if reason.contains("more than once")),
            "expected a duplicate-assignment error, got {error:?}"
        );
    }

    #[test]
    fn too_many_positional_arguments_are_rejected() {
        let extra = vec!["a".to_string(); 32];
        let error = bind_arguments("From Base64", &[], &extra).unwrap_err();
        assert!(
            matches!(&error, RuntimeError::InvalidArgument { name, .. } if name == "arguments"),
            "expected an arity error, got {error:?}"
        );
    }

    #[test]
    fn recipe_style_short_argument_vectors_still_report_missing_required_values() {
        // Recipes hand the runtime a plain vector that may stop short of the
        // schema length; the slots past the end were not supplied.
        let error = validate_operation_args("AES Encrypt", &[]).unwrap_err();
        assert_eq!(missing_argument(&error), Some(("AES Encrypt", "Key")));
    }

    #[test]
    fn binding_is_consistent_across_every_operation_with_required_arguments() {
        // No operation may declare a required argument that also carries a
        // default: that combination makes "required" unenforceable.
        for name in operation_names(None) {
            let info = operation_info(&name).unwrap();
            for argument in info.args {
                if argument.required {
                    assert_eq!(
                        argument.default_value, "",
                        "{name}: required argument '{}' also declares a default",
                        argument.name
                    );
                }
            }
            // An operation with no required arguments must bind with none given.
            if info.args.iter().all(|argument| !argument.required) {
                assert!(
                    bind_arguments(&name, &[], &[]).is_ok(),
                    "{name}: binding with no arguments should succeed"
                );
            } else {
                assert!(
                    matches!(
                        bind_arguments(&name, &[], &[]),
                        Err(RuntimeError::MissingArgument { .. })
                    ),
                    "{name}: binding with no arguments should report a missing argument"
                );
            }
        }
    }
}
