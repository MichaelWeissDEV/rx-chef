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
    assert_eq!(ast["type"], "Program");
    assert_eq!(ast["sourceType"], "script");
    assert_eq!(ast["body"][0]["type"], "VariableDeclaration");
    assert_eq!(ast["body"][0]["kind"], "const");
    assert_eq!(ast["body"][0]["declarations"][0]["id"]["name"], "x");
    assert_eq!(ast["body"][0]["declarations"][0]["init"]["value"], 1);
    assert!(op.run(b"const = ;".to_vec(), &[]).is_err());
}
