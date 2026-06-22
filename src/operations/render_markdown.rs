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
        "Renders input Markdown as HTML. HTML rendering is disabled to avoid XSS. (Simplified port using Regex)"
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
                description: "Enable syntax highlighting (Not supported in this port)",
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
        // Syntax highlighting is not supported without a heavy dependency like syntect.
        let open_links_blank = args.get(2).and_then(|a| a.as_bool()).unwrap_or(false);

        let mut html = input_str.to_string();

        // Escape HTML to avoid XSS (as CyberChef does with html: false)
        html = html
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;");

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

        let result = format!(
            "<div style=\"font-family: var(--primary-font-family)\">{}</div>",
            html
        );
        Ok(result.into_bytes())
    }
}
