/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the XML Beautify operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Cursor;

use quick_xml::{events::Event, reader::Reader, writer::Writer};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// XML Beautify operation
pub struct XMLBeautify;

impl Operation for XMLBeautify {
    fn name(&self) -> &'static str {
        "XML Beautify"
    }

    fn module(&self) -> &'static str {
        "Code"
    }

    fn description(&self) -> &'static str {
        "Indents and prettifies eXtensible Markup Language (XML) code."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Indent string",
            description: "The string to use for indentation",
            default_value: "\\t",
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
        let indent_str_arg = args.first().and_then(|v| v.as_str()).unwrap_or("\\t");
        let indent_str = indent_str_arg
            .replace("\\t", "\t")
            .replace("\\n", "\n")
            .replace("\\r", "\r");

        let mut reader = Reader::from_reader(Cursor::new(input));
        reader.config_mut().trim_text(true);

        let mut writer =
            Writer::new_with_indent(Vec::new(), indent_str.as_bytes()[0], indent_str.len());
        // quick-xml's Writer::new_with_indent takes a byte and a count.
        // If indent_str is more complex than just one char repeated, we might need a different approach.
        // But usually it's just tabs or spaces.

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Eof) => break,
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
