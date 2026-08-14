/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the JavaScript Parser operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};
use serde_json::{json, Value};
use swc_common::{comments::SingleThreadedComments, sync::Lrc, FileName, SourceMap};
use swc_ecma_parser::{lexer::Lexer, EsSyntax, Parser, StringInput, Syntax};

/// JavaScript Parser operation
pub struct JavaScriptParser;

impl Operation for JavaScriptParser {
    fn name(&self) -> &'static str {
        "JavaScript Parser"
    }

    fn module(&self) -> &'static str {
        "Code"
    }

    fn description(&self) -> &'static str {
        "Parses JavaScript and returns a SWC Abstract Syntax Tree as JSON. Optional source locations, byte ranges, tokens, comments, and recoverable parser errors can be included."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Location info",
                description: "Include line and column location information",
                default_value: "false",
                kind: crate::operation::ArgKind::Boolean,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Range info",
                description: "Include range information",
                default_value: "false",
                kind: crate::operation::ArgKind::Boolean,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Include tokens array",
                description: "Include tokens array",
                default_value: "false",
                kind: crate::operation::ArgKind::Boolean,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: true,
            },
            ArgSchema {
                name: "Include comments array",
                description: "Include comments array",
                default_value: "false",
                kind: crate::operation::ArgKind::Boolean,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Report errors and try to continue",
                description: "Report errors and try to continue",
                default_value: "false",
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
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let source = String::from_utf8(input)
            .map_err(|error| OperationError::InvalidInput(error.to_string()))?;
        let locations = args.first().and_then(ArgValue::as_bool).unwrap_or(false);
        let ranges = args.get(1).and_then(ArgValue::as_bool).unwrap_or(false);
        let include_tokens = args.get(2).and_then(ArgValue::as_bool).unwrap_or(false);
        let include_comments = args.get(3).and_then(ArgValue::as_bool).unwrap_or(false);
        let tolerate_errors = args.get(4).and_then(ArgValue::as_bool).unwrap_or(false);

        let source_map: Lrc<SourceMap> = Default::default();
        let file =
            source_map.new_source_file(FileName::Custom("input.js".into()).into(), source.clone());
        let comments = SingleThreadedComments::default();
        let lexer = Lexer::new(
            Syntax::Es(EsSyntax {
                jsx: true,
                ..Default::default()
            }),
            Default::default(),
            StringInput::from(&*file),
            Some(&comments),
        );
        let mut parser = Parser::new_from(lexer);
        let parsed = parser.parse_program();
        let mut errors = parser
            .take_errors()
            .into_iter()
            .map(|error| format!("{:?}", error.kind()))
            .collect::<Vec<_>>();
        let program = match parsed {
            Ok(program) => program,
            Err(error) => {
                errors.push(format!("{:?}", error.kind()));
                if !tolerate_errors {
                    return Err(OperationError::InvalidInput(errors.join("; ")));
                }
                swc_ecma_ast::Program::Script(swc_ecma_ast::Script {
                    span: Default::default(),
                    body: Vec::new(),
                    shebang: None,
                })
            }
        };
        if !tolerate_errors && !errors.is_empty() {
            return Err(OperationError::InvalidInput(errors.join("; ")));
        }

        let mut output = serde_json::to_value(program)
            .map_err(|error| OperationError::ProcessingError(error.to_string()))?;
        let line_starts = line_starts(&source);
        decorate_spans(&mut output, locations, ranges, &line_starts);
        let object = output.as_object_mut().ok_or_else(|| {
            OperationError::ProcessingError("SWC returned a non-object AST".into())
        })?;

        if include_tokens {
            let lexer = Lexer::new(
                Syntax::Es(EsSyntax {
                    jsx: true,
                    ..Default::default()
                }),
                Default::default(),
                StringInput::from(&*file),
                None,
            );
            let mut tokens = lexer
                .map(|token| {
                    json!({
                        "type": format!("{:?}", token.token),
                        "span": { "start": token.span.lo.0, "end": token.span.hi.0 }
                    })
                })
                .collect::<Vec<_>>();
            for token in &mut tokens {
                decorate_spans(token, locations, ranges, &line_starts);
            }
            object.insert("tokens".into(), Value::Array(tokens));
        }
        if include_comments {
            let (leading, trailing) = comments.take_all();
            let mut collected = leading
                .borrow()
                .values()
                .chain(trailing.borrow().values())
                .flatten()
                .cloned()
                .map(|comment| serde_json::to_value(comment).unwrap())
                .collect::<Vec<_>>();
            collected.sort_by_key(|comment| comment["span"]["start"].as_u64().unwrap_or_default());
            for comment in &mut collected {
                decorate_spans(comment, locations, ranges, &line_starts);
            }
            object.insert("comments".into(), Value::Array(collected));
        }
        if tolerate_errors {
            object.insert("errors".into(), json!(errors));
        }
        serde_json::to_vec_pretty(&output)
            .map_err(|error| OperationError::ProcessingError(error.to_string()))
    }
}

fn line_starts(source: &str) -> Vec<usize> {
    std::iter::once(0)
        .chain(source.match_indices('\n').map(|(index, _)| index + 1))
        .collect()
}

fn decorate_spans(value: &mut Value, locations: bool, ranges: bool, lines: &[usize]) {
    match value {
        Value::Array(values) => {
            for value in values {
                decorate_spans(value, locations, ranges, lines);
            }
        }
        Value::Object(object) => {
            for child in object.values_mut() {
                decorate_spans(child, locations, ranges, lines);
            }
            let span = object.remove("span");
            if let Some(span) = span {
                let start = span["start"].as_u64().unwrap_or(1).saturating_sub(1) as usize;
                let end = span["end"].as_u64().unwrap_or(1).saturating_sub(1) as usize;
                if ranges {
                    object.insert("range".into(), json!([start, end]));
                }
                if locations {
                    object.insert(
                        "loc".into(),
                        json!({ "start": position(start, lines), "end": position(end, lines) }),
                    );
                }
            }
        }
        _ => {}
    }
}

fn position(offset: usize, lines: &[usize]) -> Value {
    let line = lines
        .partition_point(|start| *start <= offset)
        .saturating_sub(1);
    json!({ "line": line + 1, "column": offset.saturating_sub(lines[line]) })
}
