use rxchef::operation::ArgValue;
use rxchef::operations::fuzzy_match::FuzzyMatch;
use rxchef::Operation;

fn default_args(search: &str) -> [ArgValue; 8] {
    [
        ArgValue::Str(search.to_string()),
        ArgValue::Num(15.0),   // sequential_bonus
        ArgValue::Num(30.0),   // separator_bonus
        ArgValue::Num(30.0),   // camel_bonus
        ArgValue::Num(15.0),   // first_letter_bonus
        ArgValue::Num(-5.0),   // leading_letter_penalty
        ArgValue::Num(-15.0),  // max_leading_letter_penalty
        ArgValue::Num(-1.0),   // unmatched_letter_penalty
    ]
}

#[test]
fn test_fuzzy_match_empty_input_boundary() {
    let op = FuzzyMatch;
    let args = default_args("search");
    let result = op.run(b"".to_vec(), &args).unwrap();
    assert_eq!(result, b"No matches.");
}

#[test]
fn test_fuzzy_match_empty_search_boundary() {
    let op = FuzzyMatch;
    let args = default_args("");
    let result = op.run(b"input".to_vec(), &args).unwrap();
    assert_eq!(result, b"No matches.");
}

#[test]
fn test_fuzzy_match_exact_match() {
    let op = FuzzyMatch;
    let args = default_args("hello");
    let result = op.run(b"hello".to_vec(), &args).unwrap();
    let result_str = String::from_utf8_lossy(&result);
    assert!(result_str.contains("<b>h</b><b>e</b><b>l</b><b>l</b><b>o</b>") || result_str.contains("<b>hello</b>") || result_str.contains("<span class=\"hl1\"><b>h</b><b>e</b><b>l</b><b>l</b><b>o</b></span>"));
}

#[test]
fn test_fuzzy_match_partial_match() {
    let op = FuzzyMatch;
    let args = default_args("hl");
    let result = op.run(b"hello".to_vec(), &args).unwrap();
    let result_str = String::from_utf8_lossy(&result);
    assert!(result_str.contains("<b>h</b>") && result_str.contains("<b>l</b>"));
    assert!(!result_str.contains("<b>e</b>"));
}

#[test]
fn test_fuzzy_match_no_match() {
    let op = FuzzyMatch;
    let args = default_args("xyz");
    let result = op.run(b"hello".to_vec(), &args).unwrap();
    assert_eq!(result, b"No matches.");
}

#[test]
fn test_fuzzy_match_case_insensitive() {
    let op = FuzzyMatch;
    let args = default_args("HELLO");
    let result = op.run(b"hello".to_vec(), &args).unwrap();
    let result_str = String::from_utf8_lossy(&result);
    assert!(result_str.contains("<b>h</b>") || result_str.contains("<b>hello</b>") || result_str.contains("<b>h</b><b>e</b><b>l</b><b>l</b><b>o</b>"));
}

#[test]
fn test_fuzzy_match_html_escaping() {
    let op = FuzzyMatch;
    let args = default_args("test");
    let result = op.run(b"<test> & '\"".to_vec(), &args).unwrap();
    let result_str = String::from_utf8_lossy(&result);
    assert!(result_str.contains("&lt;"));
    assert!(result_str.contains("&gt;"));
    assert!(result_str.contains("&amp;"));
    assert!(result_str.contains("&quot;"));
    assert!(result_str.contains("&#039;"));
    assert!(result_str.contains("<b>t</b><b>e</b><b>s</b><b>t</b>"));
}

#[test]
fn test_fuzzy_match_invalid_utf8_boundary() {
    let op = FuzzyMatch;
    let args = default_args("search");
    let result = op.run(vec![0xFF, 0xFE], &args);
    assert!(result.is_err());
}

#[test]
fn test_fuzzy_match_single_char_boundary() {
    let op = FuzzyMatch;
    let args = default_args("a");
    let result = op.run(b"a".to_vec(), &args).unwrap();
    let result_str = String::from_utf8_lossy(&result);
    assert!(result_str.contains("<b>a</b>"));
}

#[test]
fn test_fuzzy_match_deterministic() {
    let op = FuzzyMatch;
    let args = default_args("test");
    let result1 = op.run(b"this is a test".to_vec(), &args).unwrap();
    let result2 = op.run(b"this is a test".to_vec(), &args).unwrap();
    assert_eq!(result1, result2);
}

#[test]
fn test_fuzzy_match_multiline_input() {
    let op = FuzzyMatch;
    let args = default_args("line");
    let result = op.run(b"first line\nsecond line".to_vec(), &args).unwrap();
    let result_str = String::from_utf8_lossy(&result);
    // Should highlight 'l' 'i' 'n' 'e' in 'first line'
    assert!(result_str.contains("<b>l</b><b>i</b><b>n</b><b>e</b>"));
}

#[test]
fn test_fuzzy_match_boundary_unicode() {
    let op = FuzzyMatch;
    let args = default_args("🌍");
    let result = op.run("hello 🌍".as_bytes().to_vec(), &args).unwrap();
    let result_str = String::from_utf8_lossy(&result);
    assert!(result_str.contains("<b>🌍</b>"));
}
