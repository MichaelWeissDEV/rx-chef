use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;

use rxchef::operation::ArgValue;
use rxchef::operations::http_request::HTTPRequest;
use rxchef::Operation;

#[test]
fn test_http_request_against_local_protocol_stub() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let (sender, receiver) = mpsc::channel();
    let server = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut request = Vec::new();
        let mut buffer = [0u8; 1024];
        loop {
            let read = stream.read(&mut buffer).unwrap();
            request.extend_from_slice(&buffer[..read]);
            if read == 0 || request.windows(4).any(|window| window == b"\r\n\r\n") {
                break;
            }
        }
        let header_end = request.windows(4).position(|w| w == b"\r\n\r\n").unwrap() + 4;
        let headers = String::from_utf8_lossy(&request[..header_end]);
        let length = headers.lines().find_map(|line| {
            line.to_ascii_lowercase().strip_prefix("content-length:")?.trim().parse::<usize>().ok()
        }).unwrap_or(0);
        while request.len() < header_end + length {
            let read = stream.read(&mut buffer).unwrap();
            request.extend_from_slice(&buffer[..read]);
        }
        sender.send(request).unwrap();
        stream.write_all(b"HTTP/1.1 201 Created\r\nX-Test: yes\r\nContent-Length: 4\r\nConnection: close\r\n\r\npong").unwrap();
    });

    let output = HTTPRequest.run(
        b"ping".to_vec(),
        &[
            ArgValue::Str("POST".into()),
            ArgValue::Str(format!("http://{address}/submit")),
            ArgValue::Str("X-Recipe: rxchef".into()),
            ArgValue::Str("Cross-Origin Resource Sharing".into()),
            ArgValue::Bool(false),
        ],
    ).unwrap();
    assert_eq!(output, b"pong");
    let request = receiver.recv().unwrap();
    let request = String::from_utf8(request).unwrap();
    assert!(request.starts_with("POST /submit HTTP/1.1\r\n"));
    assert!(request.to_ascii_lowercase().contains("x-recipe: rxchef\r\n"));
    assert!(request.ends_with("\r\n\r\nping"));
    server.join().unwrap();
}

#[test]
fn test_http_request_rejects_invalid_method() {
    let result = HTTPRequest.run(
        Vec::new(),
        &[ArgValue::Str("BAD METHOD".into()), ArgValue::Str("http://127.0.0.1/".into())],
    );
    assert!(result.is_err());
}
