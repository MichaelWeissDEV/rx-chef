/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Frequency distribution operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Frequency distribution operation - shows byte frequency as a text table.
pub struct FrequencyDistribution;

impl Operation for FrequencyDistribution {
    fn name(&self) -> &'static str {
        "Frequency distribution"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Displays the distribution of bytes in the data as a frequency table."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Show 0%s",
                description: "Include bytes that do not appear in the input",
                default_value: "true",
            },
            ArgSchema {
                name: "Show ASCII",
                description: "Include ASCII character column",
                default_value: "true",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        if input.is_empty() {
            return Err(OperationError::InvalidInput("No data".to_string()));
        }

        let show_zeroes = args.first().and_then(|v| v.as_bool()).unwrap_or(true);
        let show_ascii = args.get(1).and_then(|v| v.as_bool()).unwrap_or(true);

        let len = input.len();
        let mut distrib = [0usize; 256];
        for &b in &input {
            distrib[b as usize] += 1;
        }

        let bytes_represented = distrib.iter().filter(|&&c| c > 0).count();

        let mut output = String::new();
        output.push_str(&format!("Total data length: {}\n", len));
        output.push_str(&format!(
            "Number of bytes represented: {}\n",
            bytes_represented
        ));
        output.push_str(&format!(
            "Number of bytes not represented: {}\n\n",
            256 - bytes_represented
        ));

        // Header
        if show_ascii {
            output.push_str(&format!(
                "{:<6} {:<8} {:<12} {}\n",
                "Byte", "ASCII", "Percentage", "Bar"
            ));
        } else {
            output.push_str(&format!("{:<6} {:<12} {}\n", "Byte", "Percentage", "Bar"));
        }

        for i in 0usize..256 {
            if distrib[i] == 0 && !show_zeroes {
                continue;
            }
            let percentage = distrib[i] as f64 / len as f64 * 100.0;
            let bar_len = percentage.ceil() as usize;
            let bar: String = "|".repeat(bar_len);
            let pct_str = format!("{:.2}%", percentage);

            if show_ascii {
                let ascii_char = if i <= 32 {
                    // Control characters: use Unicode Control Pictures block
                    char::from_u32(0x2400 + i as u32).unwrap_or(' ')
                } else if i == 127 {
                    '\u{2421}'
                } else if i < 128 {
                    i as u8 as char
                } else {
                    '.'
                };
                output.push_str(&format!(
                    "{:<6} {:<8} {:<12} {}\n",
                    format!("{:02x}", i),
                    ascii_char,
                    pct_str,
                    bar
                ));
            } else {
                output.push_str(&format!(
                    "{:<6} {:<12} {}\n",
                    format!("{:02x}", i),
                    pct_str,
                    bar
                ));
            }
        }

        Ok(output.into_bytes())
    }
}
