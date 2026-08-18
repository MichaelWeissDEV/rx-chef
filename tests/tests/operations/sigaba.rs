// Tests for the sigaba operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations sigaba::

use rxchef::operation::ArgValue;
use rxchef::operations::sigaba::SigabaOp;
use rxchef::Operation;

#[test]
fn test_sigaba_basic_encrypt_decrypt() {
    let op = SigabaOp;
    let input = b"HELLO WORLD".to_vec();
    // Use default rotors and initial values
    let mut args = Vec::new();
    for _ in 0..10 {
        // 5 cipher + 5 control
        args.push(ArgValue::Str("SRGWANHPJZFXVIDQCEUKBYOLMT".to_string())); // wiring
        args.push(ArgValue::Bool(false)); // reversed
        args.push(ArgValue::Str("A".to_string())); // initial value
    }
    for _ in 0..5 {
        // 5 index
        args.push(ArgValue::Str("6201348957".to_string())); // wiring
        args.push(ArgValue::Str("0".to_string())); // initial value
    }
    args.push(ArgValue::Str("Encrypt".to_string()));
    let encrypted = op.run(input.clone(), &args).unwrap();
    args[40] = ArgValue::Str("Decrypt".to_string());
    let decrypted = op.run(encrypted, &args).unwrap();
    assert_eq!(String::from_utf8_lossy(&decrypted), "HELLO WORLD");
}
#[test]
fn test_sigaba_z_handling() {
    let op = SigabaOp;
    let input = b"Z".to_vec(); // Should be converted to X then encrypted
    let mut args = Vec::new();
    for _ in 0..10 {
        args.push(ArgValue::Str("SRGWANHPJZFXVIDQCEUKBYOLMT".to_string()));
        args.push(ArgValue::Bool(false));
        args.push(ArgValue::Str("A".to_string()));
    }
    for _ in 0..5 {
        args.push(ArgValue::Str("6201348957".to_string()));
        args.push(ArgValue::Str("0".to_string()));
    }
    args.push(ArgValue::Str("Encrypt".to_string()));
    let encrypted = op.run(input, &args).unwrap();
    args[40] = ArgValue::Str("Decrypt".to_string());
    let decrypted = op.run(encrypted, &args).unwrap();
    // If it was encrypted as X, it will decrypt as X.
    // Wait, JS says: if (letter === "Z") { letter = "X"; }
    // and decrypt: if (decryptedLetter === "Z") { decryptedLetter = " "; }
    // So Z -> X -> encrypted -> decrypted -> X
    // And ' ' -> Z -> encrypted -> decrypted -> Z -> ' '
    assert_eq!(String::from_utf8_lossy(&decrypted), "X");
}
