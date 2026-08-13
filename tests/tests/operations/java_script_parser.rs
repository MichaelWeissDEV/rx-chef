use rxchef::operation::ArgValue;
use rxchef::operations::java_script_parser::JavaScriptParser;
use rxchef::Operation;

#[test]
fn test_parser_ast_metadata_and_errors() {
    let op = JavaScriptParser;
    let output = op
        .run(
            b"// value\nconst x = 1;".to_vec(),
            &[
                ArgValue::Bool(true),
                ArgValue::Bool(true),
                ArgValue::Bool(true),
                ArgValue::Bool(true),
                ArgValue::Bool(false),
            ],
        )
        .unwrap();
    let ast: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert!(!op.is_broken());
    assert!(ast["body"].is_array());
    assert!(ast["tokens"].as_array().unwrap().len() >= 5);
    assert_eq!(ast["comments"].as_array().unwrap().len(), 1);
    assert!(output.windows(7).any(|window| window == b"\"range\""));
    assert!(output.windows(5).any(|window| window == b"\"loc\""));
    assert!(op.run(b"const = ;".to_vec(), &[]).is_err());
}
