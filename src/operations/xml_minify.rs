/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the XML Minify operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Cursor;

use quick_xml::{events::Event, reader::Reader, writer::Writer};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// XML Minify operation
pub struct XMLMinify;

impl Operation for XMLMinify {
    fn name(&self) -> &'static str {
        "XML Minify"
    }

    fn module(&self) -> &'static str {
        "Code"
    }

    fn description(&self) -> &'static str {
        "Compresses eXtensible Markup Language (XML) code."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Preserve comments",
            description: "Preserve XML comments",
            default_value: "false",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let preserve_comments = args.first().and_then(|v| v.as_bool()).unwrap_or(false);

        let mut reader = Reader::from_reader(Cursor::new(input));
        reader.config_mut().trim_text(true);

        let mut writer = Writer::new(Vec::new());

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Eof) => break,
                Ok(Event::Comment(_)) if !preserve_comments => {}
                Ok(event) => {
                    writer.write_event(event).map_err(|e| {
                        OperationError::ProcessingError(format!("XML Error: {}", e))
                    })?;
                }
                Err(e) => return Err(OperationError::InvalidInput(format!("Invalid XML: {}", e))),
            }
            buf.clear();
        }

        Ok(writer.into_inner())
    }
}
