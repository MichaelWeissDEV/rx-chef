/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Render Markdown operation.
 * -----------------------------------------------------------------------------
 */

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Render Markdown operation
pub struct RenderMarkdown;

impl Operation for RenderMarkdown {
    fn name(&self) -> &'static str {
        "Render Markdown"
    }

    fn module(&self) -> &'static str {
        "Code"
    }

    fn description(&self) -> &'static str {
        "Renders Markdown as safe HTML. Raw HTML is escaped, URLs can be linked automatically, fenced code blocks can be syntax-highlighted, and links can open in a new tab."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Autoconvert URLs to links",
                description: "Autoconvert URLs to links",
                default_value: "false",
            },
            ArgSchema {
                name: "Enable syntax highlighting",
                description: "Highlight strings, numbers, comments, and common language keywords in fenced code blocks",
                default_value: "true",
            },
            ArgSchema {
                name: "Open links in new tab.",
                description: "Adds target=\"_blank\" to links.",
                default_value: "false",
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
        let input_str = String::from_utf8_lossy(&input);
        let convert_links = args.first().and_then(|a| a.as_bool()).unwrap_or(false);
        let syntax_highlighting = args.get(1).and_then(|a| a.as_bool()).unwrap_or(true);
        let open_links_blank = args.get(2).and_then(|a| a.as_bool()).unwrap_or(false);

        let mut html = input_str.to_string();

        // Escape HTML to avoid XSS (as CyberChef does with html: false)
        html = html
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;");

        // Protect code before applying emphasis and link substitutions.
        let fenced = Regex::new(r"(?s)```([A-Za-z0-9_+.-]*)\r?\n(.*?)```").unwrap();
        let mut code_blocks = Vec::new();
        html = fenced
            .replace_all(&html, |captures: &regex::Captures<'_>| {
                let language = &captures[1];
                let code = if syntax_highlighting {
                    highlight_code(&captures[2])
                } else {
                    captures[2].to_string()
                };
                let class = if language.is_empty() {
                    String::new()
                } else {
                    format!(" class=\"language-{language}\"")
                };
                let placeholder = format!("\u{e000}RXCHEFCODE{}\u{e001}", code_blocks.len());
                code_blocks.push(format!("<pre><code{class}>{code}</code></pre>"));
                placeholder
            })
            .into_owned();

        let inline_code = Regex::new(r"`([^`\n]+)`").unwrap();
        html = inline_code
            .replace_all(&html, |captures: &regex::Captures<'_>| {
                let placeholder = format!("\u{e000}RXCHEFCODE{}\u{e001}", code_blocks.len());
                code_blocks.push(format!("<code>{}</code>", &captures[1]));
                placeholder
            })
            .into_owned();

        // Headers
        for i in (1..=6).rev() {
            let re = Regex::new(&format!(r"(?m)^#{{{}}} (.*)$", i)).unwrap();
            html = re
                .replace_all(&html, &format!("<h{}>$1</h{}>", i, i))
                .to_string();
        }

        // Bold
        let re_bold = Regex::new(r"\*\*(.*?)\*\*").unwrap();
        html = re_bold
            .replace_all(&html, "<strong>$1</strong>")
            .to_string();

        // Italic
        let re_italic = Regex::new(r"\*(.*?)\*").unwrap();
        html = re_italic.replace_all(&html, "<em>$1</em>").to_string();

        // Links
        let target = if open_links_blank {
            " target=\"_blank\""
        } else {
            ""
        };
        let re_link = Regex::new(r"\[(.*?)\]\((.*?)\)").unwrap();
        html = re_link
            .replace_all(&html, &format!("<a href=\"$2\" {}>$1</a>", target))
            .to_string();

        if convert_links {
            let re_auto_link = Regex::new(r"(https?://\S+)").unwrap();
            html = re_auto_link
                .replace_all(&html, &format!("<a href=\"$1\" {}>$1</a>", target))
                .to_string();
        }

        // Newlines to <br> or paragraphs
        html = html.replace('\n', "<br>\n");

        for (index, block) in code_blocks.into_iter().enumerate() {
            html = html.replace(&format!("\u{e000}RXCHEFCODE{index}\u{e001}"), &block);
        }

        let result = format!(
            "<div style=\"font-family: var(--primary-font-family)\">{}</div>",
            html
        );
        Ok(result.into_bytes())
    }
}

fn highlight_code(code: &str) -> String {
    let tokens = Regex::new(
        r#"(?m)(//[^\n]*|#[^\n]*|/\*[\s\S]*?\*/)|(&quot;.*?&quot;|&#39;.*?&#39;)|\b([0-9]+(?:\.[0-9]+)?)\b|\b(fn|let|mut|struct|enum|impl|pub|use|mod|if|else|for|while|loop|match|return|class|function|const|var|true|false|null|None|Some|Ok|Err)\b"#,
    )
    .unwrap();
    tokens
        .replace_all(code, |captures: &regex::Captures<'_>| {
            let (class, token) = if let Some(value) = captures.get(1) {
                ("comment", value.as_str())
            } else if let Some(value) = captures.get(2) {
                ("string", value.as_str())
            } else if let Some(value) = captures.get(3) {
                ("number", value.as_str())
            } else {
                ("keyword", captures.get(4).unwrap().as_str())
            };
            format!("<span class=\"rxchef-{class}\">{token}</span>")
        })
        .into_owned()
}
