// Tests for the des_decrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations des_decrypt::

use rxchef::operation::ArgValue;
use rxchef::operation::OperationError;
use rxchef::operations::des_decrypt::DesDecrypt;
use rxchef::Operation;

#[test]
fn test_des_decrypt_no_key() {
    let op = DesDecrypt;
    let args = [
        ArgValue::Str("".to_string()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("CBC".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    // The test explicitly passes an empty input, but CyberChef uses empty input string in the test case.
    let result = op.run(vec![], &args);
    assert!(result.is_err());
    let err = result.unwrap_err();
    if let OperationError::InvalidArgument { name, reason } = err {
        assert_eq!(name, "Key");
        assert!(reason.contains("Invalid key length: 0 bytes"));
    } else {
        panic!("Expected InvalidArgument error");
    }
}
#[test]
fn test_des_decrypt_cbc() {
    let op = DesDecrypt;
    let input = "6500defb824b0eb8ccbf1fa9689c6f5bcc65247d93ecb0e573232824bca82dd41e2361f8fd82ef187de9f3b74f7ba3ca2b4e735f3ca6304fb8dd1675933c576424b1ea72b3219bdab62fce56d49c820d5ac02a4702a6d688e90b0933de97da21e4829e5cf85caae8".as_bytes().to_vec();
    let expected = "7a0e643132750e96d805d11e9e48e281fa39a41039286423cc1c045e5442b40bf1c3f2822bded3f9c8ef11cb25da64dda9c7ab87c246bd305385150c98f31465c2a6180fe81d31ea289b916504d5a12e1de26cb10adba84a0cb0c86f94bc14bc554f3018";
    // Let's manually parse the args to bytes for the test, mimicking CyberChef's Hex option
    let key = hex::decode("58345efb0a64e87e").unwrap();
    let iv = hex::decode("533ed1378bfd929e").unwrap();
    let args = [
        ArgValue::Bytes(key),
        ArgValue::Bytes(iv),
        ArgValue::Str("CBC".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8_lossy(&result);
    assert_eq!(result_str, expected);
}
#[test]
fn test_des_decrypt_cfb() {
    let op = DesDecrypt;
    let input = "09015087e15b09374bc9edba80ce41e6809e332fc1e988858749fb2f4ebbd6483a6fce01a43271280c07c90e13d517729acac45beef7d088339eb7e084bbbb7459fc8bb592d2ca76b90066dc79b1fbc5e016208e1d02c6e48ab675530f8040e53e1a138b".as_bytes().to_vec();
    let expected = "7a0e643132750e96d805d11e9e48e281fa39a41039286423cc1c045e5442b40bf1c3f2822bded3f9c8ef11cb25da64dda9c7ab87c246bd305385150c98f31465c2a6180fe81d31ea289b916504d5a12e1de26cb10adba84a0cb0c86f94bc14bc554f3018";
    let key = hex::decode("58345efb0a64e87e").unwrap();
    let iv = hex::decode("533ed1378bfd929e").unwrap();
    let args = [
        ArgValue::Bytes(key),
        ArgValue::Bytes(iv),
        ArgValue::Str("CFB".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8_lossy(&result);
    assert_eq!(result_str, expected);
}
#[test]
fn test_des_decrypt_ofb() {
    let op = DesDecrypt;
    let input = "09015087e15b09374d8879bac14dbad851dd08fb131353a8c510acc4570e97720dd159465f1c7da3cac4a50521e1c1ab87e8cf5b0aa0c1d2eaa8a1ed914a26c13b2b0a76a368f08812fc7fa4b7c047f27df0c35e5f53b8a20e2ffc10e55d388cae8070db".as_bytes().to_vec();
    let expected = "7a0e643132750e96d805d11e9e48e281fa39a41039286423cc1c045e5442b40bf1c3f2822bded3f9c8ef11cb25da64dda9c7ab87c246bd305385150c98f31465c2a6180fe81d31ea289b916504d5a12e1de26cb10adba84a0cb0c86f94bc14bc554f3018";
    let key = hex::decode("58345efb0a64e87e").unwrap();
    let iv = hex::decode("533ed1378bfd929e").unwrap();
    let args = [
        ArgValue::Bytes(key),
        ArgValue::Bytes(iv),
        ArgValue::Str("OFB".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8_lossy(&result);
    assert_eq!(result_str, expected);
}
#[test]
fn test_des_decrypt_ctr() {
    let op = DesDecrypt;
    let input = "09015087e15b0937ab0ae5a84d66e520893690a6ea066382bf1330e8876cb3aa82ccc634f8f0d458bbe0257df6f4637cdac89f311168ba91208a21ba4bdd13c4b1a92cb93b33364b5b94a5d3d7fba68f6eed5807d9f5afeb7fbffcd94792131d264004ae".as_bytes().to_vec();
    let expected = "7a0e643132750e96b76dc9efa7810bea2b8feaa5b97887e44f96c0e6d506cc4dd4665683c6f63139221f8d887fd0a05b39741f8a67d87d6ac6f8dc6b668bd3e4a97b8bd3a19eafd5cdf50c3e1b3f17d61087d0b67cf6db31fec338b75f5954942c852829";
    let key = hex::decode("58345efb0a64e87e").unwrap();
    let iv = hex::decode("533ed1378bfd929e").unwrap();
    let args = [
        ArgValue::Bytes(key),
        ArgValue::Bytes(iv),
        ArgValue::Str("CTR".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8_lossy(&result);
    assert_eq!(result_str, expected);
}
#[test]
fn test_des_decrypt_ecb() {
    let op = DesDecrypt;
    let input = "8dea4c6a35d5f6a419232159a0b039798d0a0b20fd1e559b1d04f8eb1120e8bca6ed5b3a4bc2b23d3b62312e6085d9e837677569fe79a65eba7cb4a2969e099fc1bd649e9c8aeb2c4c519e085db6974819257c20fde70acabc976308cc41635038c91acf5eefff1e".as_bytes().to_vec();
    let expected = "7a0e643132750e96d805d11e9e48e281fa39a41039286423cc1c045e5442b40bf1c3f2822bded3f9c8ef11cb25da64dda9c7ab87c246bd305385150c98f31465c2a6180fe81d31ea289b916504d5a12e1de26cb10adba84a0cb0c86f94bc14bc554f3018";
    let key = hex::decode("58345efb0a64e87e").unwrap();
    let iv = hex::decode("533ed1378bfd929e").unwrap();
    let args = [
        ArgValue::Bytes(key),
        ArgValue::Bytes(iv),
        ArgValue::Str("ECB".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8_lossy(&result);
    assert_eq!(result_str, expected);
}
