// Tests for the microsoft_script_decoder operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations microsoft_script_decoder::

use rxchef::operations::microsoft_script_decoder::MicrosoftScriptDecoder;
use rxchef::Operation;

#[test]
fn test_microsoft_script_decoder_basic() {
    let op = MicrosoftScriptDecoder;
    let input = b"#@~^RQAAAA==-mD~sX|:/TP{~J:+dYbxL~@!F@*@!+@*@!&@*eEI@#@&@#@&.jm.raY 214Wv:zms/obI0xEAAA==^#~@".to_vec();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("var my_msg"));
}
