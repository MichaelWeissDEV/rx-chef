/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To Braille operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// To Braille operation - converts ASCII text to six-dot Braille symbols.
pub struct ToBraille;

// Braille lookup from CyberChef lib/Braille.mjs
const BRAILLE_ASCII: &str = " A1B'K2L@CIF/MSP\"E3H9O6R^DJG>NTQ,*5<-U8V.%[$+X!&;:4\\0Z7(_?W]#Y)=";
const BRAILLE_DOT6: &str = "\u{2800}\u{2801}\u{2802}\u{2803}\u{2804}\u{2805}\u{2806}\u{2807}\u{2808}\u{2809}\u{280A}\u{280B}\u{280C}\u{280D}\u{280E}\u{280F}\u{2810}\u{2811}\u{2812}\u{2813}\u{2814}\u{2815}\u{2816}\u{2817}\u{2818}\u{2819}\u{281A}\u{281B}\u{281C}\u{281D}\u{281E}\u{281F}\u{2820}\u{2821}\u{2822}\u{2823}\u{2824}\u{2825}\u{2826}\u{2827}\u{2828}\u{2829}\u{282A}\u{282B}\u{282C}\u{282D}\u{282E}\u{282F}\u{2830}\u{2831}\u{2832}\u{2833}\u{2834}\u{2835}\u{2836}\u{2837}\u{2838}\u{2839}\u{283A}\u{283B}\u{283C}\u{283D}\u{283E}\u{283F}";

impl Operation for ToBraille {
    fn name(&self) -> &'static str {
        "To Braille"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts text to six-dot braille symbols."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8 input".to_string()))?;

        let ascii_chars: Vec<char> = BRAILLE_ASCII.chars().collect();
        let dot6_chars: Vec<char> = BRAILLE_DOT6.chars().collect();

        let result: String = input_str
            .chars()
            .map(|c| {
                let upper = c.to_uppercase().next().unwrap_or(c);
                if let Some(idx) = ascii_chars.iter().position(|&a| a == upper) {
                    dot6_chars.get(idx).copied().unwrap_or(c)
                } else {
                    c
                }
            })
            .collect();

        Ok(result.into_bytes())
    }
}
