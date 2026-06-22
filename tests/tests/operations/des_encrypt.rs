// Tests for the des_encrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations des_encrypt::

use rxchef::operation::ArgValue;
use rxchef::operation::OperationError;
use rxchef::operations::des_encrypt::DesEncrypt;
use rxchef::Operation;

#[test]
fn test_des_encrypt_no_key() {
    let op = DesEncrypt;
    let args = [
        ArgValue::Str("".to_string()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("CBC".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    let result = op.run(vec![], &args);
    assert!(result.is_err());
    if let Err(OperationError::InvalidArgument { reason, .. }) = result {
        assert!(reason.contains("Invalid key length"));
    } else {
        panic!("Expected InvalidArgument error");
    }
}
fn run_mode_test(mode: &str, expected_hex: &str) {
    let op = DesEncrypt;
    let key = hex::decode("58345efb0a64e87e").unwrap();
    let iv = hex::decode("533ed1378bfd929e").unwrap();
    let input_hex = "7a0e643132750e96d805d11e9e48e281fa39a41039286423cc1c045e5442b40bf1c3f2822bded3f9c8ef11cb25da64dda9c7ab87c246bd305385150c98f31465c2a6180fe81d31ea289b916504d5a12e1de26cb10adba84a0cb0c86f94bc14bc554f3018";
    let input_bytes = hex::decode(input_hex).unwrap();
    let args = [
        ArgValue::Bytes(key),
        ArgValue::Bytes(iv),
        ArgValue::Str(mode.to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    let result = op.run(input_bytes, &args).unwrap();
    let output_hex = String::from_utf8(result).unwrap();
    assert_eq!(output_hex, expected_hex);
}
#[test]
fn test_des_encrypt_cbc() {
    run_mode_test("CBC", "6500defb824b0eb8ccbf1fa9689c6f5bcc65247d93ecb0e573232824bca82dd41e2361f8fd82ef187de9f3b74f7ba3ca2b4e735f3ca6304fb8dd1675933c576424b1ea72b3219bdab62fce56d49c820d5ac02a4702a6d688e90b0933de97da21e4829e5cf85caae8");
}
#[test]
fn test_des_encrypt_cfb() {
    run_mode_test("CFB", "09015087e15b09374bc9edba80ce41e6809e332fc1e988858749fb2f4ebbd6483a6fce01a43271280c07c90e13d517729acac45beef7d088339eb7e084bbbb7459fc8bb592d2ca76b90066dc79b1fbc5e016208e1d02c6e48ab675530f8040e53e1a138b");
}
#[test]
fn test_des_encrypt_ofb() {
    run_mode_test("OFB", "09015087e15b09374d8879bac14dbad851dd08fb131353a8c510acc4570e97720dd159465f1c7da3cac4a50521e1c1ab87e8cf5b0aa0c1d2eaa8a1ed914a26c13b2b0a76a368f08812fc7fa4b7c047f27df0c35e5f53b8a20e2ffc10e55d388cae8070db");
}
#[test]
fn test_des_encrypt_ctr() {
    run_mode_test("CTR", "09015087e15b0937c462fd5974af0c4b5880de136a5680453c99f4500628cbeca769623515d836985110b93eacfea7fa4a7b2b3cb4f67acbb5f7e8ddb5a5d445da74bf6572b0a874befa3888c81110776388e400afd8dc908dcc0c018c7753355f8a1c9f");
}
#[test]
fn test_des_encrypt_ecb() {
    run_mode_test("ECB", "8dea4c6a35d5f6a419232159a0b039798d0a0b20fd1e559b1d04f8eb1120e8bca6ed5b3a4bc2b23d3b62312e6085d9e837677569fe79a65eba7cb4a2969e099fc1bd649e9c8aeb2c4c519e085db6974819257c20fde70acabc976308cc41635038c91acf5eefff1e");
}
