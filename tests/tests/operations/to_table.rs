// Tests for the to_table operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_table::

use rxchef::operation::ArgValue;
use rxchef::operations::to_table::ToTable;
use rxchef::Operation;

fn render(input: &str, header: bool, format: &str) -> String {
    let args = [
        ArgValue::Str(",".to_string()),
        ArgValue::Str("\\r\\n".to_string()),
        ArgValue::Bool(header),
        ArgValue::Str(format.to_string()),
    ];
    String::from_utf8(ToTable.run(input.as_bytes().to_vec(), &args).unwrap()).unwrap()
}

#[test]
fn test_to_table_ascii_borders_match_cell_widths() {
    // Regression: the horizontal border drew `width + 2` dashes while a cell
    // occupied `width + 3` columns, so every border was one short.
    let table = render("name,value\nlonger,x", true, "ASCII");
    let widths: Vec<usize> = table.lines().map(str::len).collect();
    assert!(
        widths.windows(2).all(|pair| pair[0] == pair[1]),
        "every rendered line must be the same width, got {widths:?}:\n{table}"
    );
}

#[test]
fn test_to_table_ascii_exact_layout() {
    let table = render("name,value\nlonger,x", true, "ASCII");
    assert_eq!(
        table,
        "+--------+-------+\n\
         | name   | value |\n\
         +--------+-------+\n\
         | longer | x     |\n\
         +--------+-------+\n"
    );
}

#[test]
fn test_to_table_ascii_aligns_multibyte_cells() {
    // Regression: widths were measured in UTF-8 bytes, so "café" reserved one
    // column too many and skewed the row.
    let table = render("café,b\nx,y", true, "ASCII");
    let widths: Vec<usize> = table.lines().map(|line| line.chars().count()).collect();
    assert!(
        widths.windows(2).all(|pair| pair[0] == pair[1]),
        "multibyte cells must not skew alignment, got {widths:?}:\n{table}"
    );
}

#[test]
fn test_to_table_markdown_divider_matches_row_width() {
    let table = render("a,bb\n1,2", true, "Markdown");
    let widths: Vec<usize> = table.lines().map(str::len).collect();
    assert!(
        widths.windows(2).all(|pair| pair[0] == pair[1]),
        "markdown rows and divider must line up, got {widths:?}:\n{table}"
    );
}

#[test]
fn test_to_table_html_escapes_structure() {
    let table = render("a,b\n1,2", true, "HTML");
    assert!(table.starts_with("<table"));
    assert!(table.contains("<th>a</th>"));
    assert!(table.contains("<td>1</td>"));
    assert!(table.ends_with("</table>"));
}

#[test]
fn test_to_table_without_header_row() {
    let table = render("1,2", false, "ASCII");
    assert_eq!(table, "+---+---+\n| 1 | 2 |\n+---+---+\n");
}

#[test]
fn test_to_table_empty_input() {
    let table = render("", true, "ASCII");
    assert!(
        table.is_empty() || table.trim().is_empty(),
        "empty input must not produce a table body, got {table:?}"
    );
}

#[test]
fn test_to_table_ragged_rows_are_padded_to_the_widest_row() {
    // A short row must still close its columns so borders stay aligned.
    let table = render("a,b,c\n1", true, "ASCII");
    let widths: Vec<usize> = table.lines().map(str::len).collect();
    assert!(
        widths.windows(2).all(|pair| pair[0] == pair[1]),
        "ragged rows must be padded, got {widths:?}:\n{table}"
    );
}

