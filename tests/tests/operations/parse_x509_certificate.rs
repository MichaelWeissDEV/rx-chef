// Tests for the parse_x509_certificate operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations parse_x509_certificate::
//
// The fixture is tests/fixtures/certificates/rxchef-test-cert.pem, a
// self-signed certificate generated with OpenSSL 3.6.3:
//
//   openssl req -x509 -newkey rsa:2048 -nodes -sha256 -days 3650 \
//       -subj "/C=DE/ST=Bavaria/L=Munich/O=rxchef test/CN=rxchef.example" \
//       -keyout key.pem -out rxchef-test-cert.pem
//
// Expected values are OpenSSL's own report of that certificate
// (`openssl x509 -noout -subject -issuer -fingerprint -serial`), not rx-chef's
// output — OpenSSL is the reference implementation that produced the file.

use rxchef::runtime::{self, RuntimeError};

const FIXTURE: &str = include_str!("../../fixtures/certificates/rxchef-test-cert.pem");

fn parse(input: &str, format: &str) -> Result<String, RuntimeError> {
    runtime::run_operation(
        "Parse X.509 certificate",
        input.as_bytes().to_vec(),
        &[format.to_string()],
    )
    .map(|out| String::from_utf8_lossy(&out).into_owned())
}

fn parsed() -> String {
    parse(FIXTURE, "PEM").expect("the fixture certificate must parse")
}

#[test]
fn test_parse_x509_certificate_reports_the_subject_distinguished_name() {
    // openssl: subject=C=DE, ST=Bavaria, L=Munich, O=rxchef test, CN=rxchef.example
    let output = parsed();
    assert!(
        output.contains("C=DE, ST=Bavaria, L=Munich, O=rxchef test, CN=rxchef.example"),
        "subject DN missing from:\n{output}"
    );
}

#[test]
fn test_parse_x509_certificate_reports_issuer_and_subject_for_a_self_signed_certificate() {
    let output = parsed();
    let issuer_line = output
        .lines()
        .skip_while(|line| !line.starts_with("Issuer"))
        .nth(1)
        .expect("an Issuer section");
    let subject_line = output
        .lines()
        .skip_while(|line| !line.starts_with("Subject"))
        .nth(1)
        .expect("a Subject section");
    assert_eq!(
        issuer_line.trim(),
        subject_line.trim(),
        "a self-signed certificate has the same issuer and subject"
    );
}

#[test]
fn test_parse_x509_certificate_reports_version_3() {
    // openssl: Version: 3 (0x2)
    assert!(parsed().contains("Version:          3 (0x2)"));
}

#[test]
fn test_parse_x509_certificate_reports_the_serial_number() {
    // openssl: serial=6CCD1DCC089A4887DB4691780A941C1F99DB48CD
    assert!(
        parsed().contains("6ccd1dcc089a4887db4691780a941c1f99db48cd"),
        "serial number missing or different"
    );
}

#[test]
fn test_parse_x509_certificate_fingerprints_match_openssl() {
    let output = parsed();
    // openssl -fingerprint -sha256 / -sha1 / -md5, colons removed, lowercased.
    assert!(
        output.contains("77b239688f8dfc24e5e4d99b744b96129f3b7aabd0a150c80d4e41b4277b363d"),
        "SHA-256 fingerprint missing"
    );
    assert!(
        output.contains("d506d6df894933fcb6260192dde341c63d30e06f"),
        "SHA-1 fingerprint missing"
    );
    assert!(
        output.contains("24f6f4aab1f9a58cfe095db4be5a6264"),
        "MD5 fingerprint missing"
    );
}

#[test]
fn test_parse_x509_certificate_reports_the_signature_and_key_algorithms() {
    let output = parsed();
    // sha256WithRSAEncryption and rsaEncryption.
    assert!(
        output.contains("1.2.840.113549.1.1.11"),
        "signature OID missing"
    );
    assert!(output.contains("1.2.840.113549.1.1.1"), "key OID missing");
}

#[test]
fn test_parse_x509_certificate_reports_the_validity_window() {
    let output = parsed();
    assert!(output.contains("Not Before"), "validity start missing");
    assert!(output.contains("Not After"), "validity end missing");
    // Generated with -days 3650, so the window spans ten years.
    assert!(output.contains("2026") && output.contains("2036"));
}

#[test]
fn test_parse_x509_certificate_rejects_input_that_is_not_a_certificate() {
    assert!(parse("definitely not a certificate", "PEM").is_err());
}

#[test]
fn test_parse_x509_certificate_empty_input_reports_no_input() {
    // Matches upstream, which returns the literal string "No input" rather
    // than erroring:
    //   run(input, args) { if (!input.length) { return "No input"; } ... }
    // Kept for parity; this is upstream's behaviour, not an rx-chef defect.
    assert_eq!(parse("", "PEM").unwrap(), "No input");
}

#[test]
fn test_parse_x509_certificate_rejects_a_truncated_pem_block() {
    let truncated: String = FIXTURE.lines().take(4).collect::<Vec<_>>().join("\n");
    assert!(
        parse(&truncated, "PEM").is_err(),
        "a truncated PEM block must not parse"
    );
}

#[test]
fn test_parse_x509_certificate_rejects_corrupted_base64_body() {
    // Flip a character inside the body; the DER underneath becomes invalid.
    let corrupted = FIXTURE.replacen("MII", "MIX", 1);
    assert!(
        parse(&corrupted, "PEM").is_err(),
        "a corrupted certificate body must not parse"
    );
}

#[test]
fn test_parse_x509_certificate_does_not_panic_on_binary_input() {
    // Untrusted bytes must produce an error, never a panic.
    let binary: Vec<u8> = (0u8..=255).collect();
    let result = runtime::run_operation("Parse X.509 certificate", binary, &["PEM".to_string()]);
    assert!(result.is_err(), "binary input should be rejected");
}
