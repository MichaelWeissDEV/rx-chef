// Tests for the dns_over_https operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations dns_over_https::

use rxchef::operation::ArgValue;
use rxchef::operations::dns_over_https::DnsOverHttps;
use rxchef::Operation;
use std::{
    io::{Read, Write},
    net::TcpListener,
    thread::JoinHandle,
};

fn local_resolver() -> (String, JoinHandle<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let handle = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut request = [0_u8; 4096];
        let length = stream.read(&mut request).unwrap();
        let request = String::from_utf8_lossy(&request[..length]).into_owned();
        let body = r#"{"Status":0,"Answer":[{"name":"example.com.","type":1,"TTL":60,"data":"192.0.2.1"}]}"#;
        write!(
            stream,
            "HTTP/1.1 200 OK\r\nContent-Type: application/dns-json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        )
        .unwrap();
        request
    });
    (format!("http://{address}/dns-query"), handle)
}

#[test]
fn test_dns_over_https_basic() {
    let (resolver, server) = local_resolver();
    let op = DnsOverHttps;
    let input = b"example.com".to_vec();
    let args = [
        ArgValue::Str(resolver),
        ArgValue::Str("A".to_string()),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(output.contains("\"Status\": 0"));
    assert!(output.contains("\"name\": \"example.com.\""));
    let request = server.join().unwrap();
    assert!(request.starts_with("GET /dns-query?"));
    assert!(request.contains("name=example.com"));
    assert!(request.contains("type=A"));
}
#[test]
fn test_dns_over_https_just_answer() {
    let (resolver, server) = local_resolver();
    let op = DnsOverHttps;
    let input = b"example.com".to_vec();
    let args = [
        ArgValue::Str(resolver),
        ArgValue::Str("A".to_string()),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    let output = String::from_utf8(result).unwrap();
    // Should be a JSON array of strings, e.g. ["93.184.216.34"]
    assert!(output.starts_with('['));
    assert!(output.ends_with(']'));
    assert!(output.contains("192.0.2.1"));
    server.join().unwrap();
}
