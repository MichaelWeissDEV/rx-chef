/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To Table operation.
 * -----------------------------------------------------------------------------
 */

use html_escape::encode_safe;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// To Table operation
pub struct ToTable;

impl Operation for ToTable {
    fn name(&self) -> &'static str {
        "To Table"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Data can be split on different characters and rendered as an HTML, ASCII or Markdown table with an optional header row.<br><br>Supports the CSV (Comma Separated Values) file format by default. Change the cell delimiter argument to <code>\\t</code> to support TSV (Tab Separated Values) or <code>|</code> for PSV (Pipe Separated Values).<br><br>You can enter as many delimiters as you like. Each character will be treat as a separate possible delimiter."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Cell delimiters",
                description: "Characters used to separate cells",
                default_value: ",",
            },
            ArgSchema {
                name: "Row delimiters",
                description: "Characters used to separate rows",
                default_value: "\\r\\n",
            },
            ArgSchema {
                name: "Make first row header",
                description: "Treat the first row as a header row",
                default_value: "false",
            },
            ArgSchema {
                name: "Format",
                description: "The output format",
                default_value: "ASCII",
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

        let cell_delims_raw = args.first().and_then(|v| v.as_str()).unwrap_or(",");
        let row_delims_raw = args.get(1).and_then(|v| v.as_str()).unwrap_or("\\r\\n");
        let first_row_header = args.get(2).and_then(|v| v.as_bool()).unwrap_or(false);
        let format = args.get(3).and_then(|v| v.as_str()).unwrap_or("ASCII");

        let cell_delims: Vec<char> = parse_escaped_chars(cell_delims_raw).chars().collect();
        let row_delims: Vec<char> = parse_escaped_chars(row_delims_raw).chars().collect();

        // CyberChef escapes HTML before parsing CSV
        let escaped_input = encode_safe(&input_str).to_string();
        let table_data = parse_csv(&escaped_input, &cell_delims, &row_delims);

        if table_data.is_empty() {
            return Ok(Vec::new());
        }

        let output = match format {
            "ASCII" => ascii_output(table_data, first_row_header),
            "HTML" => html_output(table_data, first_row_header),
            "Markdown" => markdown_output(table_data),
            _ => html_output(table_data, first_row_header),
        };

        Ok(output.into_bytes())
    }
}

fn parse_escaped_chars(s: &str) -> String {
    s.replace("\\r", "\r")
        .replace("\\n", "\n")
        .replace("\\t", "\t")
}

fn parse_csv(data: &str, cell_delims: &[char], line_delims: &[char]) -> Vec<Vec<String>> {
    let mut lines = Vec::new();
    let mut line = Vec::new();
    let mut cell = String::new();
    let mut in_string = false;
    let mut render_next = false;

    let chars: Vec<char> = data.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let b = chars[i];
        let next = chars.get(i + 1).cloned().unwrap_or('\0');

        if render_next {
            cell.push(b);
            render_next = false;
        } else if b == '"' && !in_string {
            in_string = true;
        } else if b == '"' && in_string {
            if next == '"' {
                render_next = true;
                i += 1; // skip next quote
            } else {
                in_string = false;
            }
        } else if !in_string && cell_delims.contains(&b) {
            line.push(cell);
            cell = String::new();
        } else if !in_string && line_delims.contains(&b) {
            line.push(cell);
            cell = String::new();
            lines.push(line);
            line = Vec::new();
            // Skip next byte if it is also a line delim (e.g. \r\n)
            if line_delims.contains(&next) && next != b {
                i += 1;
            }
        } else {
            cell.push(b);
        }
        i += 1;
    }

    if !line.is_empty() || !cell.is_empty() {
        line.push(cell);
        lines.push(line);
    }

    lines
}

fn get_longest_cells(table_data: &[Vec<String>]) -> Vec<usize> {
    let mut longest_cells = Vec::new();
    for row in table_data {
        for (idx, cell) in row.iter().enumerate() {
            if idx >= longest_cells.len() {
                longest_cells.push(cell.len());
            } else if cell.len() > longest_cells[idx] {
                longest_cells[idx] = cell.len();
            }
        }
    }
    longest_cells
}

fn ascii_output(mut table_data: Vec<Vec<String>>, first_row_header: bool) -> String {
    let longest_cells = get_longest_cells(&table_data);
    let mut output = String::new();

    let horizontal_border = |longest_cells: &[usize]| {
        let mut border = String::from("+");
        for &len in longest_cells {
            border.push_str(&"-".repeat(len + 2));
            border.push('+');
        }
        border.push('\n');
        border
    };

    let output_row = |row: &[String], longest_cells: &[usize]| {
        let mut row_output = String::from("|");
        for (idx, cell) in row.iter().enumerate() {
            let padding = if idx < longest_cells.len() {
                longest_cells[idx] - cell.len()
            } else {
                0
            };
            row_output.push_str(&format!(" {} {} |", cell, " ".repeat(padding)));
        }
        // Handle cases where row has fewer cells than header
        for idx in row.len()..longest_cells.len() {
            row_output.push_str(&format!(" {} |", " ".repeat(longest_cells[idx] + 1)));
        }
        row_output.push('\n');
        row_output
    };

    output.push_str(&horizontal_border(&longest_cells));

    if first_row_header && !table_data.is_empty() {
        let header = table_data.remove(0);
        output.push_str(&output_row(&header, &longest_cells));
        output.push_str(&horizontal_border(&longest_cells));
    }

    for row in table_data {
        output.push_str(&output_row(&row, &longest_cells));
    }

    output.push_str(&horizontal_border(&longest_cells));
    output
}

fn html_output(mut table_data: Vec<Vec<String>>, first_row_header: bool) -> String {
    let mut output =
        String::from("<table class='table table-hover table-sm table-bordered table-nonfluid'>");

    let output_row = |row: &[String], cell_type: &str| {
        let mut row_output = String::from("<tr>");
        for cell in row {
            row_output.push_str(&format!("<{}>{}</{}>", cell_type, cell, cell_type));
        }
        row_output.push_str("</tr>");
        row_output
    };

    if first_row_header && !table_data.is_empty() {
        let header = table_data.remove(0);
        output.push_str("<thead class='thead-light'>");
        output.push_str(&output_row(&header, "th"));
        output.push_str("</thead>");
    }

    output.push_str("<tbody>");
    for row in table_data {
        output.push_str(&output_row(&row, "td"));
    }
    output.push_str("</tbody></table>");
    output
}

fn markdown_output(mut table_data: Vec<Vec<String>>) -> String {
    if table_data.is_empty() {
        return String::new();
    }
    let longest_cells = get_longest_cells(&table_data);
    let mut output = String::new();

    let output_row = |row: &[String], longest_cells: &[usize]| {
        let mut row_output = String::from("|");
        for (idx, cell) in row.iter().enumerate() {
            let padding = if idx < longest_cells.len() {
                longest_cells[idx] - cell.len()
            } else {
                0
            };
            row_output.push_str(&format!(" {} {} |", cell, " ".repeat(padding)));
        }
        for idx in row.len()..longest_cells.len() {
            row_output.push_str(&format!(" {} |", " ".repeat(longest_cells[idx] + 1)));
        }
        row_output.push('\n');
        row_output
    };

    let header = table_data.remove(0);
    output.push_str(&output_row(&header, &longest_cells));

    let mut divider = String::from("|");
    for &len in &longest_cells {
        divider.push_str(&format!(" {} |", "-".repeat(len)));
    }
    divider.push('\n');
    output.push_str(&divider);

    for row in table_data {
        output.push_str(&output_row(&row, &longest_cells));
    }

    output
}
