//! Provider origin and local HTTP policy. No networking or secret diagnostics.
use super::loopback::Loopback;
use std::{fmt, net::IpAddr};
use url::Url;

pub const HEADER_BYTES: usize = 16 * 1024;
pub const HEADER_COUNT: usize = 64;
pub const REQUEST_BYTES: usize = 2 * 1024 * 1024;
pub const RESPONSE_BYTES: u64 = 32 * 1024 * 1024;
pub const CHUNK_BYTES: usize = 16 * 1024;
pub const SECRET_BYTES: usize = 8192;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Failure {
    Configuration,
    Admission,
    Startup,
    InvalidRequest,
    Authentication,
    Upstream,
    Limit,
    Io,
    CredentialEcho,
}

pub struct Provider {
    base: Url,
    prefix: String,
}
impl Provider {
    /// The configured URL is an API base, not a per-request destination. Plain
    /// HTTP requires a literal loopback IP; DNS names always require TLS.
    pub fn new(endpoint: &str) -> Result<Self, Failure> {
        if endpoint.len() > 2048
            || !endpoint.is_ascii()
            || endpoint.bytes().any(|b| b <= 32 || b == 127)
            || endpoint.contains(['\\', '%', '?', '#'])
        {
            return Err(Failure::Configuration);
        }
        let url = Url::parse(endpoint).map_err(|_| Failure::Configuration)?;
        if !matches!(url.scheme(), "http" | "https")
            || url.host_str().is_none()
            || !url.username().is_empty()
            || url.password().is_some()
            || url.port_or_known_default() == Some(0)
        {
            return Err(Failure::Configuration);
        }
        let authority = endpoint
            .split_once("://")
            .ok_or(Failure::Configuration)?
            .1
            .split('/')
            .next()
            .ok_or(Failure::Configuration)?;
        if authority.contains('@') || authority.ends_with(':') {
            return Err(Failure::Configuration);
        }
        if url.scheme() == "http" {
            let raw_host = if authority.starts_with('[') {
                let end = authority.find(']').ok_or(Failure::Configuration)?;
                &authority[1..end]
            } else {
                authority.split(':').next().ok_or(Failure::Configuration)?
            };
            if !raw_host
                .parse::<IpAddr>()
                .map(|ip| ip.is_loopback())
                .unwrap_or(false)
            {
                return Err(Failure::Configuration);
            }
        }
        let raw_path = endpoint
            .split_once("://")
            .unwrap()
            .1
            .find('/')
            .map(|offset| &endpoint[endpoint.find("://").unwrap() + 3 + offset..])
            .unwrap_or("/");
        if !safe_path(raw_path) {
            return Err(Failure::Configuration);
        }
        let prefix = url.path().trim_end_matches('/').to_owned();
        Ok(Self { base: url, prefix })
    }
    pub(super) fn endpoint(&self) -> &str { self.base.as_str().trim_end_matches('/') }
    pub fn target(&self, path: &str) -> Result<Url, Failure> {
        if path.len() > 2048 || !safe_path(path) {
            return Err(Failure::InvalidRequest);
        }
        let suffix = path.strip_prefix("/v1/").ok_or(Failure::InvalidRequest)?;
        if suffix.is_empty() {
            return Err(Failure::InvalidRequest);
        }
        let mut url = self.base.clone();
        url.set_path(&format!("{}/{suffix}", self.prefix));
        Ok(url)
    }
}
impl fmt::Debug for Provider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Provider")
    }
}

/// No Serialize/Clone/Display. Safe allocation/header copies are transient but
/// not a zeroization or OS crash-output guarantee; native qualification remains.
pub struct Secret(String);
impl Secret {
    pub fn new(value: String) -> Result<Self, Failure> {
        if value.is_empty()
            || value.len() > SECRET_BYTES
            || !value.bytes().all(|b| (33..=126).contains(&b))
        {
            return Err(Failure::Configuration);
        }
        Ok(Self(value))
    }
    pub(super) fn value(&self) -> &str {
        &self.0
    }
}
impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Secret([REDACTED])")
    }
}

pub struct RequestHead {
    pub method: http::Method,
    pub target: Url,
    pub body_len: usize,
}
/// This parses only the bounded header region. The caller admits and caps it
/// before parsing. Nothing untrusted is forwarded as a routing/auth header.
pub fn request_head(
    raw: &[u8],
    scope: &Loopback,
    provider: &Provider,
) -> Result<RequestHead, Failure> {
    if raw.len() > HEADER_BYTES
        || !raw.ends_with(b"\r\n\r\n")
        || raw.iter().enumerate().any(|(i, b)| {
            (*b == b'\n' && (i == 0 || raw[i - 1] != b'\r'))
                || (*b == b'\r' && raw.get(i + 1) != Some(&b'\n'))
        })
    {
        return Err(Failure::InvalidRequest);
    }
    let mut headers = [httparse::EMPTY_HEADER; HEADER_COUNT];
    let mut request = httparse::Request::new(&mut headers);
    if request.parse(raw).map_err(|_| Failure::InvalidRequest)?
        != httparse::Status::Complete(raw.len())
        || request.version != Some(1)
    {
        return Err(Failure::InvalidRequest);
    }
    let method = match request.method {
        Some("POST") => http::Method::POST,
        Some("GET") => http::Method::GET,
        _ => return Err(Failure::InvalidRequest),
    };
    let target = provider.target(request.path.ok_or(Failure::InvalidRequest)?)?;
    let mut host = false;
    let mut auth = false;
    let mut length = None;
    let mut seen = std::collections::HashSet::with_capacity(HEADER_COUNT);
    for header in request.headers.iter() {
        let name = header.name.to_ascii_lowercase();
        if !seen.insert(name.clone()) || header.value.iter().any(|b| *b < 32 || *b >= 127) {
            return Err(Failure::InvalidRequest);
        }
        match name.as_str() {
            "host" => {
                host = header.value == format!("127.0.0.1:{}", scope.port()).as_bytes()
                    || header.value == format!("[::1]:{}", scope.port()).as_bytes();
                if !host {
                    return Err(Failure::InvalidRequest);
                }
            }
            "authorization" => {
                auth = header
                    .value
                    .strip_prefix(b"Bearer ")
                    .map(|key| scope.accepts_capability(key))
                    .unwrap_or(false);
                if !auth {
                    return Err(Failure::Authentication);
                }
            }
            "content-length" => {
                let text =
                    std::str::from_utf8(header.value).map_err(|_| Failure::InvalidRequest)?;
                if text.is_empty()
                    || text.len() > 9
                    || !text.bytes().all(|b| b.is_ascii_digit())
                    || (text.len() > 1 && text.starts_with('0'))
                {
                    return Err(Failure::InvalidRequest);
                }
                let n: usize = text.parse().map_err(|_| Failure::InvalidRequest)?;
                if n > REQUEST_BYTES {
                    return Err(Failure::Limit);
                }
                length = Some(n);
            }
            "transfer-encoding"
            | "expect"
            | "upgrade"
            | "proxy-authorization"
            | "proxy-connection"
            | "forwarded"
            | "x-original-url"
            | "x-rewrite-url"
            | "x-http-method-override" => return Err(Failure::InvalidRequest),
            n if n.starts_with("x-forwarded-") => return Err(Failure::InvalidRequest),
            "connection"
                if !header.value.eq_ignore_ascii_case(b"close")
                    && !header.value.eq_ignore_ascii_case(b"keep-alive") =>
            {
                return Err(Failure::InvalidRequest)
            }
            _ => {} // All incoming headers are discarded, including other auth/routing hints.
        }
    }
    if !host {
        return Err(Failure::InvalidRequest);
    }
    if !auth {
        return Err(Failure::Authentication);
    }
    let body_len = match (method.as_str(), length) {
        ("POST", Some(n)) => n,
        ("GET", None | Some(0)) => 0,
        _ => return Err(Failure::InvalidRequest),
    };
    Ok(RequestHead {
        method,
        target,
        body_len,
    })
}
fn safe_path(path: &str) -> bool {
    path.starts_with('/')
        && !path.contains("//")
        && path
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'/' | b'-' | b'_' | b'.'))
        && !path.split('/').any(|s| s == "." || s == "..")
}

/// Hold a suffix until the next provider chunk so a reflected provider secret
/// cannot be released across a chunk boundary. This is exact-byte exclusion,
/// not a claim to detect encoded/partial disclosures by a hostile provider.
pub struct SecretGuard {
    pending: Vec<u8>,
}
impl SecretGuard {
    pub fn new() -> Self {
        Self {
            pending: Vec::with_capacity(SECRET_BYTES + CHUNK_BYTES),
        }
    }
    pub fn push<'a>(
        &'a mut self,
        chunk: &[u8],
        secret: Option<&Secret>,
    ) -> Result<(&'a [u8], usize), Failure> {
        if chunk.len() > CHUNK_BYTES {
            return Err(Failure::Limit);
        }
        self.pending.extend_from_slice(chunk);
        let hold = if let Some(secret) = secret {
            let key = secret.value().as_bytes();
            if self.pending.windows(key.len()).any(|s| s == key) {
                return Err(Failure::CredentialEcho);
            }
            key.len() - 1
        } else {
            0
        };
        let emit = self.pending.len().saturating_sub(hold);
        Ok((&self.pending[..emit], emit))
    }
    pub fn consumed(&mut self, bytes: usize) {
        self.pending.drain(..bytes);
    }
    pub fn finish(&self) -> &[u8] {
        &self.pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        memory_pool::{test_host::ScriptedHost, MemoryPool},
        solve_jobs::SolveJobRegistry,
    };
    fn scope() -> std::sync::Arc<Loopback> {
        Loopback::bind(&SolveJobRegistry::new(MemoryPool::new(
            ScriptedHost::normal(),
        )))
        .unwrap()
    }
    fn head(scope: &Loopback, extra: &str) -> Vec<u8> {
        format!("POST /v1/responses HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nAuthorization: Bearer {}\r\nContent-Length: 2\r\n{}\r\n",scope.port(),scope.capability(),extra).into_bytes()
    }
    #[test]
    fn provider_requires_tls_except_literal_loopback_and_has_safe_api_prefix() {
        for endpoint in [
            "http://127.0.0.1:8123/v1",
            "http://[::1]:8123/api/v1/",
            "https://provider.example/api/v1",
            "https://provider.example:8443/",
        ] {
            assert!(Provider::new(endpoint).is_ok());
        }
        for endpoint in [
            "http://localhost/v1",
            "http://provider.example/v1",
            "http://127.1/v1",
            "http://2130706433/v1",
            "http://0x7f000001/v1",
            "http://192.0.2.1/v1",
            "https://user:password@provider.example/v1",
            "https://provider.example:0/v1",
            "https://provider.example:/v1",
            "file:///v1",
            "https:provider.example/v1",
            "https://provider.example/v1?key=x",
            "https://provider.example/v1#fragment",
            "https://provider.example/a/../v1",
            "https://provider.example/a%2fv1",
            "https://provider.example/a//v1",
            "https://provider.example\\v1",
        ] {
            assert!(Provider::new(endpoint).is_err(), "configuration refusal");
        }
    }
    #[test]
    fn mapping_keeps_origin_and_prefix_without_join_or_encoded_escape() {
        let p = Provider::new("https://provider.example:8443/api/v1/").unwrap();
        assert_eq!(
            p.target("/v1/responses").unwrap().as_str(),
            "https://provider.example:8443/api/v1/responses"
        );
        for target in [
            "https://other.example/v1/responses",
            "//other.example/x",
            "/v10/responses",
            "/v1/../admin",
            "/v1/%2e%2e/admin",
            "/v1/responses?api_key=x",
            "/v1/responses#x",
            "/v1//responses",
            "/v1/",
            "/v1/\\admin",
        ] {
            assert!(p.target(target).is_err());
        }
    }
    #[test]
    fn authenticates_exact_capability_and_owned_authority_before_work() {
        let s = scope();
        let p = Provider::new("https://provider.example/v1").unwrap();
        let raw = head(&s, "X-Api-Key: ignored\r\nOpenAI-Organization: ignored\r\n");
        let r = request_head(&raw, &s, &p).unwrap();
        assert_eq!(r.body_len, 2);
        assert_eq!(r.method, http::Method::POST);
        let text = String::from_utf8(raw).unwrap();
        assert!(matches!(
            request_head(text.replace(s.capability(), "invalid").as_bytes(), &s, &p),
            Err(Failure::Authentication)
        ));
        assert!(request_head(
            text.replace("Host: 127.0.0.1", "Host: 127.0.0.2")
                .as_bytes(),
            &s,
            &p
        )
        .is_err());
        assert!(request_head(
            text.replace("Host: 127.0.0.1", "Host: provider.example")
                .as_bytes(),
            &s,
            &p
        )
        .is_err());
    }
    #[test]
    fn refuses_smuggling_upgrade_routing_and_unbounded_body_headers() {
        let s = scope();
        let p = Provider::new("https://provider.example/v1").unwrap();
        for extra in [
            "Content-Length: 2\r\n",
            "content-length: 3\r\n",
            "Transfer-Encoding: chunked\r\n",
            "Upgrade: websocket\r\n",
            "Expect: 100-continue\r\n",
            "Forwarded: host=other\r\n",
            "X-Forwarded-Host: other\r\n",
            "Proxy-Authorization: other\r\n",
            "Connection: Upgrade\r\n",
            "X-Original-URL: /admin\r\n",
        ] {
            assert!(request_head(&head(&s, extra), &s, &p).is_err());
        }
        let raw = String::from_utf8(head(&s, "")).unwrap();
        for changed in [
            raw.replace("Content-Length: 2", "Content-Length: 999999999"),
            raw.replace("Content-Length: 2", "Content-Length: 02"),
            raw.replace("POST ", "CONNECT "),
            raw.replace("HTTP/1.1", "HTTP/1.0"),
            raw.replace("\r\n", "\n"),
        ] {
            assert!(request_head(changed.as_bytes(), &s, &p).is_err());
        }
        assert!(request_head(&vec![b'x'; HEADER_BYTES + 1], &s, &p).is_err());
    }
    #[test]
    fn excludes_reflected_secret_across_every_chunk_boundary_without_holding_body() {
        let secret = Secret::new("fixture-credential-long".into()).unwrap();
        for split in 1..secret.value().len() {
            let mut guard = SecretGuard::new();
            let (first, n) = guard
                .push(&secret.value().as_bytes()[..split], Some(&secret))
                .unwrap();
            assert!(first.is_empty());
            guard.consumed(n);
            assert!(matches!(
                guard.push(&secret.value().as_bytes()[split..], Some(&secret)),
                Err(Failure::CredentialEcho)
            ));
        }
        let mut guard = SecretGuard::new();
        let mut total = 0;
        for _ in 0..100 {
            let (bytes, n) = guard.push(&vec![b'x'; CHUNK_BYTES], Some(&secret)).unwrap();
            total += bytes.len();
            guard.consumed(n);
            assert!(guard.pending.len() < secret.value().len());
        }
        total += guard.finish().len();
        assert_eq!(total, 100 * CHUNK_BYTES);
    }
    #[test]
    fn secret_is_bounded_header_safe_and_all_debug_output_is_redacted() {
        for value in ["", "x y", "x\r\ny", &"x".repeat(SECRET_BYTES + 1)] {
            assert!(Secret::new(value.into()).is_err());
        }
        let secret = Secret::new("fixture-credential".into()).unwrap();
        let provider = Provider::new("https://provider.example/v1").unwrap();
        assert_eq!(
            format!("{secret:?} {provider:?}"),
            "Secret([REDACTED]) Provider"
        );
    }
}
