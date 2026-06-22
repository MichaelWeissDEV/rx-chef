// Tests for the fernet_decrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations fernet_decrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::fernet_decrypt::FernetDecrypt;
use rxchef::Operation;

#[test]
fn test_fernet_decrypt_valid() {
    let op = FernetDecrypt;
    // key: VGhpc0lzVGhpcnR5VHdvQ2hhcmFjdGVyc0xvbmdLZXk=
    // plaintext: "This is a secret message.\n"
    let token = b"gAAAAABce-Tycae8klRxhDX2uenJ-uwV8-A1XZ2HRnfOXlNzkKKfRxviNLlgtemhT_fd1Fw5P_zFUAjd69zaJBQyWppAxVV00SExe77ql8c5n62HYJOnoIU=".to_vec();
    let result = op
        .run(
            token,
            &[ArgValue::Str(
                "VGhpc0lzVGhpcnR5VHdvQ2hhcmFjdGVyc0xvbmdLZXk=".to_string(),
            )],
        )
        .expect("should decrypt");
    assert_eq!(result, b"This is a secret message.\n");
}
#[test]
fn test_fernet_decrypt_no_key() {
    let op = FernetDecrypt;
    let token = b"gAAAAABce-Tycae8klRxhDX2uenJ-uwV8-A1XZ2HRnfOXlNzkKKfRxviNLlgtemhT_fd1Fw5P_zFUAjd69zaJBQyWppAxVV00SExe77ql8c5n62HYJOnoIU=".to_vec();
    let result = op.run(token, &[ArgValue::Str("".to_string())]);
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("32 url-safe base64-encoded bytes"));
}
#[test]
fn test_fernet_decrypt_empty_input() {
    let op = FernetDecrypt;
    let result = op.run(
        b"".to_vec(),
        &[ArgValue::Str(
            "MTIzNDU2Nzg5MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTI=".to_string(),
        )],
    );
    assert!(result.is_err());
}
