/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Show Base64 Offsets operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Show Base64 Offsets operation
pub struct ShowBase64Offsets;

impl Operation for ShowBase64Offsets {
    fn name(&self) -> &'static str {
        "Show Base64 Offsets"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Show the possible Base64 offsets for a given string."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Alphabet",
                description: "The Base64 alphabet to use",
                default_value: "A-Za-z0-9+/=",
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Show variable chars and padding",
                description: "Highlight characters affected by surrounding data and padding",
                default_value: "true",
                kind: crate::operation::ArgKind::Boolean,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Input format",
                description: "Whether the input is raw data or Base64 text",
                default_value: "Raw",
                kind: crate::operation::ArgKind::String,
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
        DataType::Html
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let alphabet = args
            .first()
            .and_then(|a| a.as_str())
            .unwrap_or("A-Za-z0-9+/=");
        let show_variable = args.get(1).and_then(|a| a.as_bool()).unwrap_or(true);
        let format = args.get(2).and_then(|a| a.as_str()).unwrap_or("Raw");
        let input = if format == "Base64" {
            crate::operations::from_base64::FromBase64.run(
                input,
                &[
                    ArgValue::Str(alphabet.to_string()),
                    ArgValue::Bool(false),
                    ArgValue::Bool(false),
                ],
            )?
        } else if format == "Raw" {
            input
        } else {
            return Err(OperationError::InvalidArgument {
                name: "Input format".into(),
                reason: format!("expected Raw or Base64, got {format:?}"),
            });
        };
        if input.is_empty() {
            return Err(OperationError::InvalidInput(
                "Please enter a string.".into(),
            ));
        }

        let encode = |bytes: Vec<u8>| -> Result<String, OperationError> {
            let encoded = crate::operations::to_base64::ToBase64
                .run(bytes, &[ArgValue::Str(alphabet.to_string())])?;
            String::from_utf8(encoded)
                .map_err(|error| OperationError::ProcessingError(error.to_string()))
        };
        let offset0 = encode(input.clone())?;
        let mut prefixed1 = vec![0];
        prefixed1.extend_from_slice(&input);
        let offset1 = encode(prefixed1)?;
        let mut prefixed2 = vec![0, 0];
        prefixed2.extend_from_slice(&input);
        let offset2 = encode(prefixed2)?;

        let plain0 = static_base64_section(&offset0, 0);
        let plain1 = static_base64_section(&offset1, 2);
        let plain2 = static_base64_section(&offset2, 3);
        if !show_variable {
            return Ok(format!("{plain0}\n{plain1}\n{plain2}").into_bytes());
        }

        let rendered0 = highlight_offset(&offset0, 0, alphabet)?;
        let rendered1 = highlight_offset(&offset1, 1, alphabet)?;
        let rendered2 = highlight_offset(&offset2, 2, alphabet)?;
        let script = "<script type='application/javascript'>$('[data-toggle=\"tooltip\"]').tooltip()</script>";
        Ok(format!(
            "Characters highlighted in <span class='hl5'>green</span> could change if the input is surrounded by more data.\n\
Characters highlighted in <span class='hl3'>red</span> are for padding purposes only.\n\
Unhighlighted characters are <span data-toggle='tooltip' data-placement='top' title='Tooltip on left'>static</span>.\n\
Hover over the static sections to see what they decode to on their own.\n\n\
Offset 0: {rendered0}\nOffset 1: {rendered1}\nOffset 2: {rendered2}{script}"
        )
        .into_bytes())
    }
}

fn static_base64_section(encoded: &str, leading: usize) -> String {
    let padding_index = encoded.find('=').map(|index| index as isize).unwrap_or(-1);
    let trailing = match padding_index % 4 {
        2 => 3,
        3 => 2,
        _ => 0,
    };
    encoded
        .get(leading..encoded.len().saturating_sub(trailing))
        .unwrap_or("")
        .to_string()
}

fn highlight_offset(
    encoded: &str,
    offset: usize,
    alphabet: &str,
) -> Result<String, OperationError> {
    let padding_index = encoded.find('=').map(|index| index as isize).unwrap_or(-1);
    let remainder = padding_index % 4;
    let trailing = if remainder == 2 {
        3
    } else if remainder == 3 {
        2
    } else {
        0
    };
    let static_end = encoded.len().saturating_sub(trailing);
    let static_part = encoded
        .get(offset + usize::from(offset > 0)..static_end)
        .unwrap_or("");
    let prefix = match offset {
        0 => String::new(),
        1 => format!(
            "<span class='hl3'>{}</span><span class='hl5'>{}</span>",
            &encoded[0..1],
            &encoded[1..2]
        ),
        2 => format!(
            "<span class='hl3'>{}</span><span class='hl5'>{}</span>",
            &encoded[0..2],
            &encoded[2..3]
        ),
        _ => unreachable!(),
    };
    let decode_prefix = if offset == 1 {
        "AA"
    } else if offset == 2 {
        "AAA"
    } else {
        ""
    };
    let tooltip_input = format!("{decode_prefix}{static_part}");
    let decoded = crate::operations::from_base64::FromBase64.run(
        tooltip_input.into_bytes(),
        &[
            ArgValue::Str(alphabet.to_string()),
            ArgValue::Bool(false),
            ArgValue::Bool(false),
        ],
    )?;
    let front = offset;
    let back = if offset == 2 && remainder == 3 {
        2
    } else if remainder == 2 {
        2
    } else if remainder == 3 {
        1
    } else {
        0
    };
    let tooltip = decoded
        .get(front..decoded.len().saturating_sub(back))
        .unwrap_or(&[]);
    let tooltip_text = String::from_utf8_lossy(tooltip);
    let tooltip = html_escape::encode_safe(&tooltip_text);
    let mut result = format!(
        "{prefix}<span data-toggle='tooltip' data-placement='top' title='{tooltip}'>{static_part}</span>"
    );
    if trailing > 0 {
        let variable = &encoded[static_end..static_end + 1];
        let padding = &encoded[static_end + 1..];
        result.push_str(&format!(
            "<span class='hl5'>{variable}</span><span class='hl3'>{padding}</span>"
        ));
    }
    Ok(result)
}
