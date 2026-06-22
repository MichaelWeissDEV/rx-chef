// Tests for the dns_over_https operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations dns_over_https::

use rxchef::operation::ArgValue;
use rxchef::operations::dns_over_https::DnsOverHttps;
use rxchef::Operation;

// Since this operation performs network requests, we mark these tests with `#[ignore]`
// to prevent them from failing in offline environments or slowing down standard test runs.
#[test]
#[ignore]
fn test_dns_over_https_basic() {
    let op = DnsOverHttps;
    let input = b"example.com".to_vec();
    let args = [
        ArgValue::Str("https://cloudflare-dns.com/dns-query".to_string()),
        ArgValue::Str("A".to_string()),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(output.contains("\"Status\": 0"));
    assert!(output.contains("\"name\": \"example.com.\""));
}
#[test]
#[ignore]
fn test_dns_over_https_just_answer() {
    let op = DnsOverHttps;
    let input = b"example.com".to_vec();
    let args = [
        ArgValue::Str("https://cloudflare-dns.com/dns-query".to_string()),
        ArgValue::Str("A".to_string()),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    let output = String::from_utf8(result).unwrap();
    // Should be a JSON array of strings, e.g. ["93.184.216.34"]
    assert!(output.starts_with('['));
    assert!(output.ends_with(']'));
}
