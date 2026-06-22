/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.1.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Generic Code Beautify operation.
 * -----------------------------------------------------------------------------
 */

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Generic Code Beautify operation
pub struct GenericCodeBeautify;

impl Operation for GenericCodeBeautify {
    fn name(&self) -> &'static str {
        "Generic Code Beautify"
    }

    fn module(&self) -> &'static str {
        "Code"
    }

    fn description(&self) -> &'static str {
        "Attempts to pretty print C-style languages such as C, C++, C#, Java, PHP, JavaScript etc.<br><br>This will not do a perfect job, and the resulting code may not work any more. This operation is designed purely to make obfuscated or minified code more easy to read and understand.<br><br>Things which will not work properly:<ul><li>For loop formatting</li><li>Do-While loop formatting</li><li>Switch/Case indentation</li><li>Certain bit shift operators</li></ul>"
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
        let mut code = String::from_utf8_lossy(&input).to_string();
        let mut preserved_tokens = Vec::new();

        // Remove strings, comments, and regexes to preserve them
        let patterns = [
            (r"'([^'\\]|\\.)*'", false),      // single strings
            (r#""([^"\\]|\\.)*""#, false),    // double strings
            (r"//[^\n\r]*", false),           // single line comments
            (r"(?m)/\*[\s\S]*?\*/", true),    // multi-line comments
            (r"(^|\n)#[^\n\r#]+", false),     // hash comments
            (r"/.*?[^\\]/[gim]{0,3}", false), // regexes
        ];

        for (pattern, _is_m) in patterns {
            let re =
                Regex::new(pattern).map_err(|e| OperationError::ProcessingError(e.to_string()))?;
            loop {
                let mut found = false;
                if let Some(m) = re.find(&code) {
                    let token = m.as_str().to_string();
                    let placeholder = format!("###preservedToken{}###", preserved_tokens.len());
                    preserved_tokens.push(token);
                    code.replace_range(m.range(), &placeholder);
                    found = true;
                }
                if !found {
                    break;
                }
            }
        }

        code = code
            .replace(';', ";\n")
            .replace('{', "{\n")
            .replace('}', "\n}\n")
            .replace('\r', "");

        // Remove indentation
        let re_indent1 = Regex::new(r"(?m)^\s+").unwrap();
        code = re_indent1.replace_all(&code, "").to_string();
        let re_indent2 = Regex::new(r"(?m)\n\s+").unwrap();
        code = re_indent2.replace_all(&code, "\n").to_string();

        // Remove trailing spaces
        let re_trailing = Regex::new(r"(?m)\s*$").unwrap();
        code = re_trailing.replace_all(&code, "").to_string();
        code = code.replace("\n{", "{");

        // Indent
        let mut indented_code = String::new();
        let mut level = 0;
        let lines: Vec<&str> = code.split('\n').collect();
        for (i, line) in lines.iter().enumerate() {
            let current_line = line.trim().to_string();
            if current_line.is_empty() && i < lines.len() - 1 {
                indented_code.push('\n');
                continue;
            }

            if current_line.starts_with('}') {
                level -= 1;
            }

            let indent = if level > 0 {
                " ".repeat(level * 4)
            } else {
                "".to_string()
            };
            indented_code.push_str(&indent);
            indented_code.push_str(&current_line);
            if i < lines.len() - 1 {
                indented_code.push('\n');
            }

            if current_line.ends_with('{') {
                level += 1;
            }
        }
        code = indented_code;

        // char-by-char indentation was more robust in JS
        // Let's re-implement that part
        let mut final_code = String::new();
        let mut level = 0i32;
        let chars: Vec<char> = code.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            match chars[i] {
                '{' => {
                    level += 1;
                    final_code.push('{');
                }
                '\n' => {
                    final_code.push('\n');
                    if i + 1 < chars.len() {
                        let mut next_is_close = false;
                        if chars[i + 1] == '}' {
                            next_is_close = true;
                        }
                        if next_is_close {
                            level -= 1;
                        }
                        let indent_level = if level > 0 { level as usize } else { 0 };
                        final_code.push_str(&" ".repeat(indent_level * 4));
                    }
                }
                '}' => {
                    // level handled by \n before
                    final_code.push('}');
                }
                c => final_code.push(c),
            }
            i += 1;
        }
        code = final_code;

        // Strategic spaces
        let space_patterns = [
            (r"\s*([!<>=+\-/*]?)=\s*", " $1= "),
            (r"\s*<([=]?)\s*", " <$1 "),
            (r"\s*>([=]?)\s*", " >$1 "),
            (r"([^+])\+([^+=])", "$1 + $2"),
            (r"([^-])-([^-=])", "$1 - $2"),
            (r"([^*])\*([^*=])", "$1 * $2"),
            (r"([^/])\/([^/=])", "$1 / $2"),
            (r"\s*,\s*", ", "),
            (r"\s*\{", " {"),
            (r"}\n", "}\n\n"),
        ];

        for (pattern, replacement) in space_patterns {
            let re = Regex::new(pattern).unwrap();
            code = re.replace_all(&code, replacement).to_string();
        }

        // Hacky horribleness
        let hack_patterns = [
            (
                r"(?i)(if|for|while|with|elif|elseif)\s*\(([^\n]*)\)\s*\n([^{])",
                "$1 ($2)\n    $3",
            ),
            (
                r"(?i)(if|for|while|with|elif|elseif)\s*\(([^\n]*)\)([^{])",
                "$1 ($2) $3",
            ),
            (r"(?i)else\s*\n([^{])", "else\n    $1"),
            (r"(?i)else\s+([^{])", "else $1"),
        ];

        for (pattern, replacement) in hack_patterns {
            let re = Regex::new(pattern).unwrap();
            code = re.replace_all(&code, replacement).to_string();
        }

        code = code
            .replace(" ;", ";")
            .replace("{ }", "{}")
            .replace("[ ]", "[]");

        let re_else_catch =
            Regex::new(r"(?i)}\s*(else|catch|except|finally|elif|elseif|else if)").unwrap();
        code = re_else_catch.replace_all(&code, "} $1").to_string();

        // Restore preserved tokens
        for (idx, token) in preserved_tokens.iter().enumerate() {
            let placeholder = format!("###preservedToken{}###", idx);
            code = code.replace(&placeholder, token);
        }

        Ok(code.into_bytes())
    }
}
