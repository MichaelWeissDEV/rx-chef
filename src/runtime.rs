use crate::{
    operation::{ArgSchema, ArgValue, DataType},
    operations,
};

#[derive(Debug, Clone)]
pub struct OperationInfo {
    pub name: &'static str,
    pub module: &'static str,
    pub description: &'static str,
    pub input_type: DataType,
    pub output_type: DataType,
    pub is_broken: bool,
    pub args: &'static [ArgSchema],
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
        module: operation.module(),
        description: operation.description(),
        input_type: operation.input_type(),
        output_type: operation.output_type(),
        is_broken: operation.is_broken(),
        args: operation.args_schema(),
    })
}

pub fn run_operation(
    operation_name: &str,
    input: Vec<u8>,
    raw_args: &[String],
) -> Result<Vec<u8>, String> {
    let canonical_name = resolve_operation_name(operation_name)
        .ok_or_else(|| not_found_message("operation", operation_name))?;
    let operation = operations::get_operation(&canonical_name)
        .ok_or_else(|| not_found_message("operation", operation_name))?;
    let parsed_args = raw_args
        .iter()
        .map(|arg| parse_operation_arg(arg))
        .collect::<Result<Vec<_>, _>>()?;

    operation
        .run(input, &parsed_args)
        .map_err(|error| error.to_string())
}

pub fn resolve_named_args(
    op_name: &str,
    named: &[String],
    positional: &[String],
) -> Result<Vec<String>, String> {
    if named.is_empty() {
        return Ok(positional.to_vec());
    }
    let info = operation_info(op_name)?;
    let schema_len = info.args.len();
    let mut result: Vec<String> = positional.to_vec();
    while result.len() < schema_len {
        result.push(String::new());
    }
    for kv in named {
        let (name, value) = kv
            .split_once('=')
            .ok_or_else(|| format!("invalid --arg '{}': expected NAME=VALUE", kv))?;
        let name_lower = name.to_lowercase();
        let idx = info
            .args
            .iter()
            .position(|a| a.name.to_lowercase() == name_lower)
            .ok_or_else(|| format!("argument '{}' not found in '{}'", name, op_name))?;
        while result.len() <= idx {
            result.push(String::new());
        }
        result[idx] = value.to_string();
    }
    Ok(result)
}

pub fn parse_operation_arg(raw: &str) -> Result<ArgValue, String> {
    if let Some(rest) = raw.strip_prefix("num:") {
        let number = rest
            .parse::<f64>()
            .map_err(|error| format!("invalid numeric argument '{}': {}", raw, error))?;
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
