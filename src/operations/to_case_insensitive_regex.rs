/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To Case Insensitive Regex operation.
 * -----------------------------------------------------------------------------
 */

use regex::{Captures, Regex};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// To Case Insensitive Regex operation
pub struct ToCaseInsensitiveRegex;

impl Operation for ToCaseInsensitiveRegex {
    fn name(&self) -> &'static str {
        "To Case Insensitive Regex"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts a case-sensitive regex string into a case-insensitive regex string in case the i flag is unavailable to you.<br><br>e.g. <code>Mozilla/[0-9].[0-9] .*</code> becomes <code>[mM][oO][zZ][iI][lL][lL][aA]/[0-9].[0-9] .*</code>"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);

        // Validate regex
        if let Err(e) = Regex::new(&input_str) {
            return Err(OperationError::InvalidInput(format!(
                "Invalid Regular Expression: {}",
                e
            )));
        }

        let preprocessed = pre_process(&input_str);

        let re1 = Regex::new(r"([A-Z]-[A-Z]|[a-z]-[a-z])").unwrap();
        let res = re1.replace_all(&preprocessed, |caps: &Captures| {
            let m = &caps[0];
            let c0 = m.chars().nth(0).unwrap();
            let c2 = m.chars().nth(2).unwrap();
            format!(
                "{}-{}{}-{}",
                c0.to_uppercase(),
                c2.to_uppercase(),
                c0.to_lowercase(),
                c2.to_lowercase()
            )
        });

        let re2 = Regex::new(r"[A-Z]-[a-z]").unwrap();
        let res = re2.replace_all(&res, |caps: &Captures| {
            let m = &caps[0];
            let c0 = m.chars().nth(0).unwrap();
            let c2 = m.chars().nth(2).unwrap();
            format!("A-{}{}{}a-z", c2.to_uppercase(), m, c0.to_lowercase())
        });

        let re3 = Regex::new(r"\\?[ -@]-[A-Z]").unwrap();
        let res = re3.replace_all(&res, |caps: &Captures| {
            let m = &caps[0];
            let c_last = m.chars().last().unwrap();
            format!("{}a-{}", m, c_last.to_lowercase())
        });

        let re4 = Regex::new(r"\\?[ -@]-\\?[\[-`]").unwrap();
        let res = re4.replace_all(&res, |caps: &Captures| format!("{}a-z", &caps[0]));

        let re5 = Regex::new(r"[A-Z]-\\?[\[-`]").unwrap();
        let res = re5.replace_all(&res, |caps: &Captures| {
            let m = &caps[0];
            let c0 = m.chars().nth(0).unwrap();
            format!("{}{}a-z", m, c0.to_lowercase())
        });

        let re6 = Regex::new(r"\\?[\[-`]-\\?[\{-~]").unwrap();
        let res = re6.replace_all(&res, |caps: &Captures| format!("{}A-Z", &caps[0]));

        let re7 = Regex::new(r"[a-z]-\\?[\{-~]").unwrap();
        let res = re7.replace_all(&res, |caps: &Captures| {
            let m = &caps[0];
            let c0 = m.chars().nth(0).unwrap();
            format!("{}{}A-Z", m, c0.to_uppercase())
        });

        let re8 = Regex::new(r"\\?[ -@]-[a-z]").unwrap();
        let res = re8.replace_all(&res, |caps: &Captures| {
            let m = &caps[0];
            let c0 = m.chars().next().unwrap();
            format!("{}-z", c0)
        });

        let re9 = Regex::new(r"\\?[\[-`]-[a-z]").unwrap();
        let res = re9.replace_all(&res, |caps: &Captures| {
            let m = &caps[0];
            let c_last = m.chars().last().unwrap();
            format!("A-{}{}", c_last.to_uppercase(), m)
        });

        Ok(res.into_owned().into_bytes())
    }
}

fn pre_process(input: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = input.chars().collect();
    for i in 0..chars.len() {
        let temp = chars[i];
        let prev = if i > 0 { Some(chars[i - 1]) } else { None };
        let next = if i < chars.len() - 1 {
            Some(chars[i + 1])
        } else {
            None
        };

        if temp.is_ascii_alphabetic() && prev != Some('-') && next != Some('-') {
            result.push('[');
            result.push(temp.to_lowercase().next().unwrap());
            result.push(temp.to_uppercase().next().unwrap());
            result.push(']');
        } else {
            result.push(temp);
        }
    }
    result
}
