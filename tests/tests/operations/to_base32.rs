// Tests for the to_base32 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_base32::

use rxchef::operation::ArgValue;
use rxchef::operations::from_base32::FromBase32;
use rxchef::operations::to_base32::ToBase32;
use rxchef::Operation;

fn run(input: &[u8]) -> String {
    let op = ToBase32;
    let result = op.run(input.to_vec(), &[]).unwrap();
    String::from_utf8(result).unwrap()
}

// RFC 4648 section 10 test vectors.
#[test]
fn test_rfc4648_vectors() {
    assert_eq!(run(b""), "");
    assert_eq!(run(b"f"), "MY======");
    assert_eq!(run(b"fo"), "MZXQ====");
    assert_eq!(run(b"foo"), "MZXW6===");
    assert_eq!(run(b"foob"), "MZXW6YQ=");
    assert_eq!(run(b"fooba"), "MZXW6YTB");
    assert_eq!(run(b"foobar"), "MZXW6YTBOI======");
}

#[test]
fn test_empty_input() {
    let op = ToBase32;
    let result = op.run(vec![], &[]).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_custom_alphabet() {
    // Fully expanded custom alphabet (the "0-9A-V" style hextet alphabet from
    // RFC 4648 section 7), passed as a literal 32-character string since
    // to_base32's own alphabet argument does no range expansion.
    let op = ToBase32;
    let alphabet = "0123456789ABCDEFGHIJKLMNOPQRSTUV";
    let args = [ArgValue::Str(alphabet.to_string())];
    let result = op.run(b"foobar".to_vec(), &args).unwrap();
    // Same bit pattern as the standard alphabet test above but shifted
    // through a different symbol table: MZXW6YTBOI -> CPNMUOJ1E8
    let standard = run(b"foobar");
    // Re-derive expected value by mapping each standard symbol to its
    // equivalent in the custom alphabet (both are contiguous 32-symbol
    // tables sharing the same value order: A-Z2-7 vs 0-9A-V).
    let standard_alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567".chars().collect();
    let custom_alphabet: Vec<char> = alphabet.chars().collect();
    let expected: String = standard
        .chars()
        .map(|c| {
            if c == '=' {
                '='
            } else {
                let idx = standard_alphabet.iter().position(|&x| x == c).unwrap();
                custom_alphabet[idx]
            }
        })
        .collect();
    assert_eq!(String::from_utf8(result).unwrap(), expected);
}

#[test]
fn test_invalid_alphabet_length_errors() {
    let op = ToBase32;
    let args = [ArgValue::Str("ABC".to_string())];
    let result = op.run(b"data".to_vec(), &args);
    assert!(result.is_err());
}

#[test]
fn test_roundtrip_with_from_base32() {
    let to_op = ToBase32;
    let from_op = FromBase32;
    let input = b"The quick brown fox jumps over the lazy dog".to_vec();
    let encoded = to_op.run(input.clone(), &[]).unwrap();
    let decoded = from_op.run(encoded, &[]).unwrap();
    assert_eq!(decoded, input);
}

#[test]
fn test_binary_data_roundtrip() {
    let to_op = ToBase32;
    let from_op = FromBase32;
    let input: Vec<u8> = (0..=255).collect();
    let encoded = to_op.run(input.clone(), &[]).unwrap();
    let decoded = from_op.run(encoded, &[]).unwrap();
    assert_eq!(decoded, input);
}
