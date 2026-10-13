use std::{io::Read, time::Duration};

use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    redirect::Policy,
    Url,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::*;

/// HTTP client for `/api/v1`. Redirects and proxy discovery are disabled so a
/// configured bearer token cannot be forwarded to a different service.
/// Credentials are deliberately excluded from Debug and error messages.
pub struct RemoteEngineClient {
    client: Client,
    base: Url,
    max_response_bytes: u64,
}

impl RemoteEngineClient {
    pub fn new(base: &str, token: Option<&str>, timeout: Duration) -> Result<Self, ClientError> {
        let mut base = Url::parse(base)
            .map_err(|_| ClientError::new(ErrorKind::InvalidInput, "invalid server URL"))?;
        let loopback = base
            .host_str()
            .and_then(|host| {
                host.trim_start_matches('[')
                    .trim_end_matches(']')
                    .parse::<std::net::IpAddr>()
                    .ok()
            })
            .is_some_and(|ip| ip.is_loopback());
        if !base.username().is_empty()
            || base.password().is_some()
            || base.query().is_some()
            || base.fragment().is_some()
            || !matches!(base.path(), "" | "/")
            || !(base.scheme() == "https" || base.scheme() == "http" && loopback)
        {
            return Err(ClientError::new(ErrorKind::InvalidInput, "use an HTTPS server origin (HTTP is allowed only for loopback IPs), without credentials, path, query or fragment"));
        }
        if timeout.is_zero() {
            return Err(ClientError::new(
                ErrorKind::InvalidInput,
                "timeout must be positive",
            ));
        }
        base.set_path("/api/v1/");
        let mut headers = HeaderMap::new();
        if let Some(token) = token {
            if token.is_empty() {
                return Err(ClientError::new(
                    ErrorKind::InvalidInput,
                    "token must not be empty",
                ));
            }
            let mut value = HeaderValue::from_str(&format!("Bearer {token}"))
                .map_err(|_| ClientError::new(ErrorKind::InvalidInput, "invalid token"))?;
            value.set_sensitive(true);
            headers.insert(AUTHORIZATION, value);
        }
        let client = Client::builder()
            .no_proxy()
            .redirect(Policy::none())
            .timeout(timeout)
            .default_headers(headers)
            .build()
            .map_err(|_| ClientError::new(ErrorKind::Transport, "cannot initialize HTTP client"))?;
        Ok(Self {
            client,
            base,
            max_response_bytes: 64 << 20,
        })
    }

    fn request<T: DeserializeOwned>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<&impl Serialize>,
    ) -> Result<T, ClientError> {
        let url = self
            .base
            .join(path)
            .map_err(|_| ClientError::new(ErrorKind::Protocol, "invalid endpoint"))?;
        let mut request = self.client.request(method, url);
        if let Some(body) = body {
            request = request.json(body);
        }
        let response = request
            .send()
            .map_err(|_| ClientError::new(ErrorKind::Transport, "server request failed"))?;
        let status = response.status();
        let mut bytes = Vec::new();
        response
            .take(self.max_response_bytes + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| ClientError::new(ErrorKind::Transport, "cannot read server response"))?;
        if bytes.len() as u64 > self.max_response_bytes {
            return Err(ClientError::new(
                ErrorKind::LimitExceeded,
                "server response exceeds byte limit",
            ));
        }
        if !status.is_success() {
            return Err(
                serde_json::from_slice::<ClientError>(&bytes).unwrap_or_else(|_| {
                    ClientError::new(
                        ErrorKind::Transport,
                        format!("server returned HTTP {}", status.as_u16()),
                    )
                }),
            );
        }
        serde_json::from_slice(&bytes)
            .map_err(|_| ClientError::new(ErrorKind::Protocol, "invalid server response"))
    }
}

impl EngineClient for RemoteEngineClient {
    fn submit_job(&self, task: EngineTask) -> Result<JobSnapshot, ClientError> {
        self.request(reqwest::Method::POST, "jobs", Some(&task))
    }
    fn job(&self, id: &str) -> Result<JobSnapshot, ClientError> {
        validate_job_id(id)?;
        self.request(reqwest::Method::GET, &format!("jobs/{id}"), None::<&()>)
    }
    fn cancel_job(&self, id: &str) -> Result<JobSnapshot, ClientError> {
        validate_job_id(id)?;
        self.request(reqwest::Method::POST, &format!("jobs/{id}/cancel"), Some(&()))
    }
    fn magic(&self, request: MagicRequest) -> Result<Vec<MagicCandidate>, ClientError> {
        self.request(reqwest::Method::POST, "magic", Some(&request))
    }
    fn scan(&self, request: ScanRequest) -> Result<Vec<rxchef::scan::Finding>, ClientError> {
        self.request(reqwest::Method::POST, "scan", Some(&request))
    }
    fn capabilities(&self) -> Result<Capabilities, ClientError> {
        let capabilities: Capabilities =
            self.request(reqwest::Method::GET, "capabilities", None::<&()>)?;
        if capabilities.protocol_version != PROTOCOL_VERSION
            || capabilities.binary_transport != "base64"
        {
            return Err(ClientError::new(
                ErrorKind::Protocol,
                "incompatible server protocol",
            ));
        }
        Ok(capabilities)
    }
    fn operations(&self) -> Result<Vec<integration::OperationDescriptor>, ClientError> {
        self.request(reqwest::Method::GET, "operations", None::<&()>)
    }
    fn describe(&self, operation: &str) -> Result<integration::OperationDescriptor, ClientError> {
        self.request(
            reqwest::Method::POST,
            "describe",
            Some(&serde_json::json!({"operation": operation})),
        )
    }
    fn validate(&self, request: &ExecuteRequest) -> Result<(), ClientError> {
        self.request(reqwest::Method::POST, "validate", Some(request))
    }
    fn execute(&self, request: ExecuteRequest) -> Result<ExecuteResponse, ClientError> {
        let response: ExecuteResponse =
            self.request(reqwest::Method::POST, "execute", Some(&request))?;
        export_bytes(&response.result)?;
        Ok(response)
    }
}

fn validate_job_id(id: &str) -> Result<(), ClientError> {
    if id.len() != 36 || !id.bytes().all(|byte| byte.is_ascii_hexdigit() || byte == b'-') {
        return Err(ClientError::new(ErrorKind::InvalidInput, "invalid job ID"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::Write, net::TcpListener, thread, time::Instant};

    fn server(
        reply: impl FnOnce(&str) -> String + Send + 'static,
    ) -> (String, thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let handle = thread::spawn(move || {
            let deadline = Instant::now() + Duration::from_secs(5);
            let mut stream = loop {
                match listener.accept() {
                    Ok((stream, _)) => break stream,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        assert!(Instant::now() < deadline, "test request did not arrive");
                        thread::sleep(Duration::from_millis(5));
                    }
                    Err(error) => panic!("{error}"),
                }
            };
            stream
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            let mut bytes = Vec::new();
            loop {
                let mut chunk = [0; 4096];
                let read = stream.read(&mut chunk).unwrap();
                assert!(read > 0);
                bytes.extend_from_slice(&chunk[..read]);
                assert!(bytes.len() < 1024 * 1024);
                if let Some(end) = bytes.windows(4).position(|window| window == b"\r\n\r\n") {
                    let headers = String::from_utf8_lossy(&bytes[..end]).to_lowercase();
                    let length = headers
                        .lines()
                        .find_map(|line| {
                            line.strip_prefix("content-length:")
                                .map(|v| v.trim().parse::<usize>().unwrap())
                        })
                        .unwrap_or(0);
                    if bytes.len() >= end + 4 + length {
                        break;
                    }
                }
            }
            let response = reply(std::str::from_utf8(&bytes).unwrap());
            stream.write_all(response.as_bytes()).unwrap();
        });
        (url, handle)
    }

    fn json_response(status: &str, value: &impl Serialize) -> String {
        let body = serde_json::to_string(value).unwrap();
        format!("HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len())
    }

    #[test]
    fn http_execute_uses_same_binary_contract_and_bearer_header() {
        let (url, handle) = server(|request| {
            assert!(request.starts_with("POST /api/v1/execute HTTP/1.1\r\n"));
            assert!(request
                .to_lowercase()
                .contains("authorization: bearer test-secret\r\n"));
            let body = request.split_once("\r\n\r\n").unwrap().1;
            let input: ExecuteRequest = serde_json::from_str(body).unwrap();
            let response = LocalEngineClient::default().execute(input).unwrap();
            json_response("200 OK", &response)
        });
        let remote =
            RemoteEngineClient::new(&url, Some("test-secret"), Duration::from_secs(5)).unwrap();
        let input: Vec<u8> = (0..=255).collect();
        let result = remote.run("To Base64", &input, vec![]).unwrap();
        assert_eq!(
            export_bytes(&result.result).unwrap(),
            STANDARD.encode(input).as_bytes()
        );
        handle.join().unwrap();
    }

    #[test]
    fn remote_keeps_structured_policy_errors() {
        let (url, handle) = server(|_| {
            json_response(
                "403 Forbidden",
                &ClientError::new(ErrorKind::PolicyDenied, "operation denied"),
            )
        });
        let remote = RemoteEngineClient::new(&url, None, Duration::from_secs(5)).unwrap();
        assert_eq!(
            remote.run("To Base64", b"hi", vec![]).unwrap_err().kind,
            ErrorKind::PolicyDenied
        );
        handle.join().unwrap();
    }

    #[test]
    fn incompatible_protocol_is_rejected() {
        let (url, handle) = server(|_| {
            let mut capabilities = LocalEngineClient::default().capabilities().unwrap();
            capabilities.protocol_version += 1;
            json_response("200 OK", &capabilities)
        });
        let remote = RemoteEngineClient::new(&url, None, Duration::from_secs(5)).unwrap();
        assert_eq!(remote.capabilities().unwrap_err().kind, ErrorKind::Protocol);
        handle.join().unwrap();
    }

    #[test]
    fn responses_are_bounded_even_without_content_length() {
        let (url, handle) =
            server(|_| "HTTP/1.1 200 OK\r\nConnection: close\r\n\r\n0123456789".into());
        let mut remote = RemoteEngineClient::new(&url, None, Duration::from_secs(5)).unwrap();
        remote.max_response_bytes = 4;
        assert_eq!(
            remote.capabilities().unwrap_err().kind,
            ErrorKind::LimitExceeded
        );
        handle.join().unwrap();
    }

    #[test]
    fn redirects_are_not_followed_and_errors_do_not_echo_secrets() {
        let (url, handle) = server(|_| {
            "HTTP/1.1 302 Found\r\nLocation: http://127.0.0.1:1/?token=secret\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".into()
        });
        let remote =
            RemoteEngineClient::new(&url, Some("private-secret"), Duration::from_secs(5)).unwrap();
        let error = remote.capabilities().unwrap_err();
        assert_eq!(error.message, "server returned HTTP 302");
        handle.join().unwrap();
    }
}
