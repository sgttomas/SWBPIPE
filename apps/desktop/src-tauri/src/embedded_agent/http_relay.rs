//! Explicit bounded HTTP relay. Not installed into application startup. Only a
//! Ready relay may serve a supplier; Joined is relay-thread observation, never
//! supplier exit or a domain outcome. No request is retried by this layer.
use super::{
    loopback::{Loopback, SOCKET_BUFFER},
    relay_policy::{self as policy, Failure, Provider, Secret, SecretGuard},
};
use crate::{memory_pool::Lease, solve_jobs::SolveJobRegistry};
use bytes::Bytes;
use http_body_util::BodyExt;
use std::{
    io,
    net::TcpStream as StdStream,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        mpsc, Arc,
    },
    thread::{self, JoinHandle},
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    task::JoinSet,
};

pub const MAX_CONNECTIONS: usize = 4;
const STACK_BYTES: usize = 1024 * 1024;
/// Requested GENERAL userspace/vendor allowance, not a hard allocator/RSS cap.
pub const STATE_BYTES: u64 = 16 * 1024 * 1024;
pub const CONNECTION_BYTES: u64 = 16 * 1024 * 1024;
pub const THREAD_BYTES: u64 = 2 * 1024 * 1024; // main 1 MiB + <=2 DNS stacks 512 KiB
/// Nominal kernel socket/queue allowance; requested buffers/backlogs also bound
/// the configured topology. OS rounding/caches are not a measured RSS claim.
pub const KERNEL_BYTES: u64 = 8 * 1024 * 1024;
const HEADER_TIMEOUT: Duration = Duration::from_secs(5);
const IO_TIMEOUT: Duration = Duration::from_secs(30);
const TOTAL_TIMEOUT: Duration = Duration::from_secs(300);

struct Resources {
    scope: Arc<Loopback>,
    provider: Provider,
    secret: Option<Secret>, // None is explicit no-key configuration, never fallback.
    _leases: Vec<Lease>,
}
struct Shared {
    stop: AtomicBool,
    active: AtomicUsize,
    finished: AtomicUsize,
    failed: AtomicUsize,
    fatal: AtomicBool,
    ready: AtomicBool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Starting,
    Ready,
    Stopping,
    Joined,
    Failed,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Status {
    pub state: State,
    pub joined: bool,
    pub active: usize,
    pub finished: usize,
    pub failed: usize,
}
/// No process/credential/provider work occurs on global construction. Keep this
/// owner until actual join. Dropping unfinished custody deliberately retains all
/// backing/listeners, even if the detached worker later finishes.
pub struct Relay {
    resources: Option<Arc<Resources>>,
    shared: Arc<Shared>,
    startup: mpsc::Receiver<Result<(), Failure>>,
    worker: Option<JoinHandle<()>>,
    state: State,
    endpoint_issued: bool,
}
impl Relay {
    pub fn start(
        registry: &SolveJobRegistry,
        scope: Arc<Loopback>,
        provider: Provider,
        secret: Option<Secret>,
    ) -> Result<Self, Failure> {
        // All owned buffers, stacks/runtime/client/DNS/connection slots admitted
        // before creating any worker/runtime/client. No pressure-time eviction.
        let state = registry
            .acquire_interactive_ingress(STATE_BYTES)
            .map_err(|_| Failure::Admission)?;
        let mut leases = Vec::with_capacity(MAX_CONNECTIONS + 3);
        leases.push(state);
        leases.push(
            registry
                .acquire_interactive_ingress(THREAD_BYTES)
                .map_err(|_| Failure::Admission)?,
        );
        leases.push(
            registry
                .acquire_interactive_ingress(KERNEL_BYTES)
                .map_err(|_| Failure::Admission)?,
        );
        for _ in 0..MAX_CONNECTIONS {
            leases.push(
                registry
                    .acquire_interactive_ingress(CONNECTION_BYTES)
                    .map_err(|_| Failure::Admission)?,
            );
        }
        scope.claim_service().map_err(|_| Failure::Configuration)?;
        let resources = Arc::new(Resources {
            scope,
            provider,
            secret,
            _leases: leases,
        });
        let shared = Arc::new(Shared {
            stop: AtomicBool::new(false),
            active: AtomicUsize::new(0),
            finished: AtomicUsize::new(0),
            failed: AtomicUsize::new(0),
            fatal: AtomicBool::new(false),
            ready: AtomicBool::new(false),
        });
        let (sender, startup) = mpsc::sync_channel(1);
        let worker_resources = resources.clone();
        let worker_shared = shared.clone();
        let worker = thread::Builder::new()
            .name("swbpipe-provider-relay".into())
            .stack_size(STACK_BYTES)
            .spawn(move || {
                let _lifetime = WorkerLife(worker_shared.clone());
                let runtime = match tokio::runtime::Builder::new_current_thread()
                    .enable_io()
                    .enable_time()
                    .max_blocking_threads(2)
                    .thread_stack_size(STACK_BYTES / 2)
                    .build()
                {
                    Ok(runtime) => runtime,
                    Err(_) => {
                        let _ = sender.send(Err(Failure::Startup));
                        return;
                    }
                };
                runtime.block_on(async {
                    let tls = match tls_config() {
                        Ok(tls) => tls,
                        Err(error) => {
                            let _ = sender.send(Err(error));
                            return;
                        }
                    };
                    let tls = Arc::new(tls);
                    if worker_shared.stop.load(Ordering::SeqCst) {
                        return;
                    }
                    worker_shared.ready.store(true, Ordering::SeqCst);
                    if sender.send(Ok(())).is_err() {
                        return;
                    }
                    run(worker_resources, worker_shared, tls).await;
                });
                // Runtime drop waits for actual DNS blocking workers. It can remain
                // blocked in the OS resolver; the outer owner never claims a join
                // or releases accounting until this thread actually returns.
            })
            .map_err(|_| Failure::Startup)?;
        Ok(Self {
            resources: Some(resources),
            shared,
            startup,
            worker: Some(worker),
            state: State::Starting,
            endpoint_issued: false,
        })
    }
    pub fn scope(&self) -> &Arc<Loopback> {
        &self.resources.as_ref().expect("owned relay custody").scope
    }
    /// One supplier endpoint, minted only after actual readiness observation.
    pub fn endpoint(&mut self) -> Result<Endpoint, Failure> {
        if self.poll().state != State::Ready
            || self.endpoint_issued
            || !self.shared.ready.load(Ordering::SeqCst)
            || self.shared.stop.load(Ordering::SeqCst)
        {
            return Err(Failure::Startup);
        }
        self.endpoint_issued = true;
        Ok(Endpoint {
            scope: self.scope().clone(),
            shared: self.shared.clone(),
        })
    }
    pub fn request_stop(&mut self) {
        self.shared.ready.store(false, Ordering::SeqCst);
        self.shared.stop.store(true, Ordering::SeqCst);
        if matches!(self.state, State::Ready | State::Starting) {
            self.state = State::Stopping;
        }
    }
    pub fn poll(&mut self) -> Status {
        if let Ok(result) = self.startup.try_recv() {
            match result {
                Ok(()) if self.state == State::Starting => self.state = State::Ready,
                Err(_) => self.state = State::Failed,
                _ => {}
            }
        }
        if self
            .worker
            .as_ref()
            .is_some_and(|worker| worker.is_finished())
        {
            let failed = self.worker.take().unwrap().join().is_err();
            if failed || self.state == State::Failed || self.shared.fatal.load(Ordering::SeqCst) {
                self.state = State::Failed;
            } else {
                self.state = State::Joined;
            }
        }
        Status {
            state: self.state,
            joined: self.worker.is_none(),
            active: self.shared.active.load(Ordering::SeqCst),
            finished: self.shared.finished.load(Ordering::SeqCst),
            failed: self.shared.failed.load(Ordering::SeqCst),
        }
    }
}
impl std::fmt::Debug for Relay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Relay")
    }
}
impl Drop for Relay {
    fn drop(&mut self) {
        self.request_stop();
        if let Some(worker) = self.worker.take() {
            if worker.is_finished() {
                let _ = worker.join();
            } else {
                std::mem::forget(worker);
                // Failure to observe termination cannot free memory/listeners
                // that a blocked worker may still use. Explicit close required.
                if let Some(resources) = self.resources.take() {
                    std::mem::forget(resources);
                }
            }
        }
    }
}
fn tls_config() -> Result<rustls::ClientConfig, Failure> {
    use rustls_platform_verifier::BuilderVerifierExt;
    // Agent 0 selected the published platform verifier with ordinary SecTrust
    // validation traffic. No extra roots, custom verifier, fetch disable or
    // hostname/certificate bypass; application data remains origin-pinned.
    rustls::ClientConfig::builder_with_provider(Arc::new(rustls::crypto::ring::default_provider()))
        .with_protocol_versions(&[&rustls::version::TLS13, &rustls::version::TLS12])
        .map_err(|_| Failure::Startup)?
        .with_platform_verifier()
        .map_err(|_| Failure::Startup)
        .map(|builder| builder.with_no_client_auth())
}
/// Not constructible from a port or a command payload; carries listener custody
/// and live readiness. It contains no provider credential and is not Clone.
pub struct Endpoint {
    scope: Arc<Loopback>,
    shared: Arc<Shared>,
}
impl Endpoint {
    pub(super) fn scope(&self) -> &Loopback {
        &self.scope
    }
    pub(super) fn is_ready(&self) -> bool {
        self.shared.ready.load(Ordering::SeqCst) && !self.shared.stop.load(Ordering::SeqCst)
    }
}
impl std::fmt::Debug for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RelayEndpoint")
    }
}
struct WorkerLife(Arc<Shared>);
impl Drop for WorkerLife {
    fn drop(&mut self) {
        self.0.ready.store(false, Ordering::SeqCst);
    }
}
struct Active(Arc<Shared>);
impl Drop for Active {
    fn drop(&mut self) {
        self.0.active.fetch_sub(1, Ordering::SeqCst);
    }
}
async fn run(resources: Arc<Resources>, shared: Arc<Shared>, tls: Arc<rustls::ClientConfig>) {
    let mut tasks = JoinSet::new();
    let mut tick = tokio::time::interval(Duration::from_millis(10));
    tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    let mut ipv6_first = false;
    while !shared.stop.load(Ordering::SeqCst) {
        tick.tick().await;
        while let Some(result) = tasks.try_join_next() {
            if result.is_err() {
                shared.failed.fetch_add(1, Ordering::SeqCst);
            }
        }
        while tasks.len() < MAX_CONNECTIONS && !shared.stop.load(Ordering::SeqCst) {
            let accept = if ipv6_first {
                resources.scope.accept_ipv6()
            } else {
                resources.scope.accept_ipv4()
            };
            let accept = match accept {
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                    if ipv6_first {
                        resources.scope.accept_ipv4()
                    } else {
                        resources.scope.accept_ipv6()
                    }
                }
                other => other,
            };
            ipv6_first = !ipv6_first;
            let (stream, peer) = match accept {
                Ok(pair) => pair,
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => break,
                Err(_) => {
                    shared.fatal.store(true, Ordering::SeqCst);
                    shared.stop.store(true, Ordering::SeqCst);
                    break;
                }
            };
            if !peer.ip().is_loopback() {
                continue;
            }
            let resources = resources.clone();
            let shared = shared.clone();
            let tls = tls.clone();
            shared.active.fetch_add(1, Ordering::SeqCst);
            let active = Active(shared.clone());
            tasks.spawn(async move {
                let _active = active;
                let result =
                    tokio::time::timeout(TOTAL_TIMEOUT, serve(stream, &resources, &tls, &shared))
                        .await;
                match result {
                    Ok(Ok(())) => {
                        shared.finished.fetch_add(1, Ordering::SeqCst);
                    }
                    _ => {
                        shared.failed.fetch_add(1, Ordering::SeqCst);
                    }
                }
            });
        }
    }
    // Aborting drops owned socket/provider futures. No retry, application
    // cancellation, rollback or peer-delivery guarantee follows from this.
    tasks.abort_all();
    while tasks.join_next().await.is_some() {}
}
fn stream(stream: StdStream) -> Result<TcpStream, Failure> {
    let socket = socket2::SockRef::from(&stream);
    socket
        .set_recv_buffer_size(SOCKET_BUFFER)
        .map_err(|_| Failure::Io)?;
    socket
        .set_send_buffer_size(SOCKET_BUFFER)
        .map_err(|_| Failure::Io)?;
    stream.set_nonblocking(true).map_err(|_| Failure::Io)?;
    stream.set_nodelay(true).map_err(|_| Failure::Io)?;
    TcpStream::from_std(stream).map_err(|_| Failure::Io)
}
async fn serve(
    raw: StdStream,
    resources: &Resources,
    tls: &Arc<rustls::ClientConfig>,
    shared: &Shared,
) -> Result<(), Failure> {
    let mut stream = stream(raw)?;
    let head = tokio::time::timeout(HEADER_TIMEOUT, read_head(&mut stream))
        .await
        .map_err(|_| Failure::Io)??;
    let (raw, header_end) = head;
    let head = match policy::request_head(&raw[..header_end], &resources.scope, &resources.provider)
    {
        Ok(head) => head,
        Err(error) => {
            reject(&mut stream, error == Failure::Authentication).await;
            return Err(error);
        }
    };
    let buffered = &raw[header_end..];
    if buffered.len() > head.body_len {
        reject(&mut stream, false).await;
        return Err(Failure::InvalidRequest);
    }
    let mut body = Vec::with_capacity(head.body_len);
    body.extend_from_slice(buffered);
    body.resize(head.body_len, 0);
    tokio::time::timeout(IO_TIMEOUT, stream.read_exact(&mut body[buffered.len()..]))
        .await
        .map_err(|_| Failure::Io)?
        .map_err(|_| Failure::Io)?;
    if shared.stop.load(Ordering::SeqCst) {
        return Err(Failure::Io);
    }
    let (response, connection) = match forward(head, body, resources.secret.as_ref(), tls).await {
        Ok(response) => response,
        Err(_) => {
            gateway(&mut stream).await;
            return Err(Failure::Upstream);
        }
    };
    let headers = response.headers();
    let header_bytes: usize = headers
        .iter()
        .map(|(name, value)| name.as_str().len() + value.len() + 4)
        .sum();
    // Hyper's HTTP/1 parser has the same finite pre-parse header buffer/count
    // limits. Check the parsed totals before any forwarding.
    if response.status().is_redirection()
        || response.status().as_u16() < 200
        || response.status().as_u16() > 599
        || headers.len() > policy::HEADER_COUNT
        || header_bytes > policy::HEADER_BYTES
        || !unencoded_response(headers)
        || headers.get(http::header::CONTENT_LENGTH).is_some_and(|n| {
            n.to_str()
                .ok()
                .and_then(|n| n.parse::<u64>().ok())
                .map(|n| n > policy::RESPONSE_BYTES)
                .unwrap_or(true)
        })
    {
        gateway(&mut stream).await;
        return Err(Failure::Upstream);
    }
    let content_type = match headers
        .get(http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(';').next())
    {
        Some("text/event-stream") => "text/event-stream",
        Some("application/json") => "application/json",
        _ => {
            gateway(&mut stream).await;
            return Err(Failure::Upstream);
        }
    };
    let header = format!("HTTP/1.1 {} Provider\r\nContent-Type: {}\r\nTransfer-Encoding: chunked\r\nConnection: close\r\nCache-Control: no-store\r\n\r\n", response.status().as_u16(), content_type);
    write(&mut stream, header.as_bytes()).await?;
    let mut source = response.into_body();
    let mut guard = SecretGuard::new();
    let mut total = 0u64;
    while let Some(frame) = tokio::time::timeout(IO_TIMEOUT, source.frame())
        .await
        .map_err(|_| Failure::Io)?
    {
        let frame = frame.map_err(|_| Failure::Upstream)?;
        let chunk: Bytes = match frame.into_data() {
            Ok(data) => data,
            Err(_) => continue,
        };
        total = total
            .checked_add(chunk.len() as u64)
            .ok_or(Failure::Limit)?;
        if total > policy::RESPONSE_BYTES {
            return Err(Failure::Limit);
        }
        for chunk in chunk.chunks(policy::CHUNK_BYTES) {
            let (safe, consumed) = guard.push(chunk, resources.secret.as_ref())?;
            emit(&mut stream, safe).await?;
            guard.consumed(consumed);
        }
    }
    drop(connection);
    emit(&mut stream, guard.finish()).await?;
    write(&mut stream, b"0\r\n\r\n").await?;
    let _ = stream.shutdown().await;
    Ok(())
}
/// Hyper removes chunk framing but does not decompress representation or other
/// transfer codings. Check every field before projecting JSON/SSE headers.
fn unencoded_response(headers: &http::HeaderMap) -> bool {
    if headers
        .get_all(http::header::CONTENT_ENCODING)
        .iter()
        .any(|value| !value.as_bytes().eq_ignore_ascii_case(b"identity"))
    {
        return false;
    }
    let mut transfer = headers.get_all(http::header::TRANSFER_ENCODING).iter();
    match (transfer.next(), transfer.next()) {
        (None, None) => true,
        (Some(value), None) => value.as_bytes().eq_ignore_ascii_case(b"chunked"),
        _ => false,
    }
}
struct Connection(tokio::task::JoinHandle<Result<(), hyper::Error>>);
impl Drop for Connection {
    fn drop(&mut self) {
        self.0.abort();
    }
}
async fn forward(
    head: policy::RequestHead,
    body: Vec<u8>,
    secret: Option<&Secret>,
    tls: &Arc<rustls::ClientConfig>,
) -> Result<(http::Response<hyper::body::Incoming>, Connection), Failure> {
    let host = match head.target.host().ok_or(Failure::Upstream)? {
        url::Host::Domain(name) => name.to_owned(),
        url::Host::Ipv4(ip) => ip.to_string(),
        url::Host::Ipv6(ip) => ip.to_string(),
    };
    let port = head
        .target
        .port_or_known_default()
        .ok_or(Failure::Upstream)?;
    let deadline = tokio::time::Instant::now() + IO_TIMEOUT;
    let addresses =
        tokio::time::timeout_at(deadline, tokio::net::lookup_host((host.as_str(), port)))
            .await
            .map_err(|_| Failure::Io)?
            .map_err(|_| Failure::Upstream)?;
    let mut stream = None;
    // Address fallback occurs only before HTTP bytes exist. No request replay,
    // pool reuse, proxy selection or redirect implementation is present.
    for address in addresses.take(16) {
        let socket = if address.is_ipv4() {
            tokio::net::TcpSocket::new_v4()
        } else {
            tokio::net::TcpSocket::new_v6()
        }
        .map_err(|_| Failure::Io)?;
        socket
            .set_recv_buffer_size(SOCKET_BUFFER as u32)
            .map_err(|_| Failure::Io)?;
        socket
            .set_send_buffer_size(SOCKET_BUFFER as u32)
            .map_err(|_| Failure::Io)?;
        match tokio::time::timeout_at(deadline, socket.connect(address)).await {
            Ok(Ok(connected)) => {
                stream = Some(connected);
                break;
            }
            _ => continue,
        }
    }
    let stream = stream.ok_or(Failure::Upstream)?;
    stream.set_nodelay(true).map_err(|_| Failure::Io)?;
    let mut builder = hyper::client::conn::http1::Builder::new();
    builder
        .max_buf_size(policy::HEADER_BYTES)
        .max_headers(policy::HEADER_COUNT);
    let (mut sender, connection) = if head.target.scheme() == "https" {
        let name = rustls::pki_types::ServerName::try_from(host).map_err(|_| Failure::Upstream)?;
        let stream = tokio::time::timeout_at(
            deadline,
            tokio_rustls::TlsConnector::from(tls.clone()).connect(name, stream),
        )
        .await
        .map_err(|_| Failure::Io)?
        .map_err(|_| Failure::Upstream)?;
        let (sender, connection) = builder
            .handshake::<_, http_body_util::Full<Bytes>>(hyper_util::rt::TokioIo::new(stream))
            .await
            .map_err(|_| Failure::Upstream)?;
        (sender, Connection(tokio::spawn(connection)))
    } else {
        let (sender, connection) = builder
            .handshake::<_, http_body_util::Full<Bytes>>(hyper_util::rt::TokioIo::new(stream))
            .await
            .map_err(|_| Failure::Upstream)?;
        (sender, Connection(tokio::spawn(connection)))
    };
    let authority = &head.target[url::Position::BeforeHost..url::Position::AfterPort];
    let mut request = http::Request::builder()
        .method(head.method)
        .uri(head.target.path())
        .header(http::header::HOST, authority)
        .header(http::header::CONTENT_TYPE, "application/json")
        .header(http::header::ACCEPT, "application/json, text/event-stream")
        .header(http::header::ACCEPT_ENCODING, "identity")
        .header(http::header::CONNECTION, "close");
    if let Some(secret) = secret {
        let mut value = http::HeaderValue::from_str(&format!("Bearer {}", secret.value()))
            .map_err(|_| Failure::Configuration)?;
        value.set_sensitive(true);
        request = request.header(http::header::AUTHORIZATION, value);
    }
    let request = request
        .body(http_body_util::Full::new(Bytes::from(body)))
        .map_err(|_| Failure::InvalidRequest)?;
    let response = tokio::time::timeout(IO_TIMEOUT, sender.send_request(request))
        .await
        .map_err(|_| Failure::Io)?
        .map_err(|_| Failure::Upstream)?;
    Ok((response, connection))
}
async fn read_head(stream: &mut TcpStream) -> Result<(Vec<u8>, usize), Failure> {
    let mut raw = Vec::with_capacity(policy::HEADER_BYTES);
    loop {
        if let Some(end) = raw.windows(4).position(|w| w == b"\r\n\r\n") {
            return Ok((raw, end + 4));
        }
        if raw.len() == policy::HEADER_BYTES {
            return Err(Failure::Limit);
        }
        let mut chunk = [0u8; 2048];
        let available = (policy::HEADER_BYTES - raw.len()).min(chunk.len());
        let n = stream
            .read(&mut chunk[..available])
            .await
            .map_err(|_| Failure::Io)?;
        if n == 0 {
            return Err(Failure::InvalidRequest);
        }
        raw.extend_from_slice(&chunk[..n]);
    }
}
async fn write(stream: &mut TcpStream, bytes: &[u8]) -> Result<(), Failure> {
    tokio::time::timeout(IO_TIMEOUT, stream.write_all(bytes))
        .await
        .map_err(|_| Failure::Io)?
        .map_err(|_| Failure::Io)
}
async fn emit(stream: &mut TcpStream, bytes: &[u8]) -> Result<(), Failure> {
    if bytes.is_empty() {
        return Ok(());
    }
    write(stream, format!("{:x}\r\n", bytes.len()).as_bytes()).await?;
    write(stream, bytes).await?;
    write(stream, b"\r\n").await
}
async fn reject(stream: &mut TcpStream, auth: bool) {
    let response = if auth {
        b"HTTP/1.1 401 Unauthorized\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".as_slice()
    } else {
        b"HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".as_slice()
    };
    let _ = write(stream, response).await;
}
async fn gateway(stream: &mut TcpStream) {
    let _ = write(
        stream,
        b"HTTP/1.1 502 Bad Gateway\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
    )
    .await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_pool::{test_host::ScriptedHost, MemoryPool};
    use std::{
        io::{Read, Write},
        net::{TcpListener, TcpStream as Client},
        sync::Mutex,
        time::Instant,
    };
    const KEY: &str = "fixture-provider-credential-long";
    fn registry() -> SolveJobRegistry {
        SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()))
    }
    fn wait_ready(relay: &mut Relay) {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            let s = relay.poll();
            if s.state == State::Ready {
                return;
            }
            assert!(!s.joined && Instant::now() < deadline, "relay readiness");
            thread::sleep(Duration::from_millis(5));
        }
    }
    fn stop(relay: &mut Relay) {
        relay.request_stop();
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            let s = relay.poll();
            if s.joined {
                assert_eq!(s.active, 0);
                assert_eq!(s.state, State::Joined);
                return;
            }
            assert!(Instant::now() < deadline, "relay join");
            thread::sleep(Duration::from_millis(5));
        }
    }
    fn start(endpoint: &str, key: bool) -> (Relay, Arc<Loopback>) {
        let registry = registry();
        let scope = Loopback::bind(&registry).unwrap();
        let mut relay = Relay::start(
            &registry,
            scope.clone(),
            Provider::new(endpoint).unwrap(),
            if key {
                Some(Secret::new(KEY.into()).unwrap())
            } else {
                None
            },
        )
        .unwrap();
        wait_ready(&mut relay);
        (relay, scope)
    }
    fn wire(scope: &Loopback, extra: &str) -> Vec<u8> {
        format!("POST /v1/responses HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nAuthorization: Bearer {}\r\nContent-Length: 2\r\n{}\r\n{{}}",scope.port(),scope.capability(),extra).into_bytes()
    }
    fn connect(scope: &Loopback, ipv6: bool) -> Client {
        let client = Client::connect(if ipv6 {
            format!("[::1]:{}", scope.port())
        } else {
            format!("127.0.0.1:{}", scope.port())
        })
        .unwrap();
        client
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        client
            .set_write_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        client
    }
    fn exchange(scope: &Loopback, raw: &[u8], ipv6: bool) -> Vec<u8> {
        let mut c = connect(scope, ipv6);
        c.write_all(raw).unwrap();
        let mut out = Vec::new();
        c.read_to_end(&mut out).unwrap();
        out
    }
    fn upstream_once(
        listener: TcpListener,
        response: Vec<Vec<u8>>,
        captured: Arc<Mutex<Vec<u8>>>,
    ) -> JoinHandle<()> {
        thread::spawn(move || {
            listener.set_nonblocking(true).unwrap();
            let deadline = Instant::now() + Duration::from_secs(5);
            let mut stream = loop {
                match listener.accept() {
                    Ok((stream, _)) => break stream,
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                        if Instant::now() > deadline {
                            return;
                        }
                        thread::sleep(Duration::from_millis(5));
                    }
                    Err(_) => return,
                }
            };
            stream.set_nonblocking(false).unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(3)))
                .unwrap();
            stream
                .set_write_timeout(Some(Duration::from_secs(3)))
                .unwrap();
            let mut raw = Vec::new();
            let mut chunk = [0u8; 4096];
            loop {
                let n = match stream.read(&mut chunk) {
                    Ok(n) => n,
                    Err(_) => return,
                };
                if n == 0 {
                    return;
                }
                raw.extend_from_slice(&chunk[..n]);
                if raw
                    .windows(4)
                    .position(|s| s == b"\r\n\r\n")
                    .is_some_and(|i| raw.len() >= i + 6)
                {
                    break;
                }
            }
            *captured.lock().unwrap() = raw;
            for chunk in response {
                if stream.write_all(&chunk).is_err() {
                    break;
                }
            }
        })
    }
    #[test]
    pub(super) fn forwards_actual_json_on_both_families_with_only_host_provider_auth() {
        for ipv6 in [false, true] {
            let backend = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = backend.local_addr().unwrap();
            let captured = Arc::new(Mutex::new(Vec::new()));
            let owner=upstream_once(backend,vec![b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 11\r\nConnection: close\r\n\r\n{\"ok\":true}".to_vec()],captured.clone());
            let (mut relay, scope) = start(&format!("http://{addr}/api/v1"), true);
            let response = exchange(
                &scope,
                &wire(
                    &scope,
                    "X-Api-Key: ignored\r\nOpenAI-Organization: ignored\r\n",
                ),
                ipv6,
            );
            assert!(response.starts_with(b"HTTP/1.1 200"));
            assert!(response.windows(11).any(|w| w == b"{\"ok\":true}"));
            stop(&mut relay);
            owner.join().unwrap();
            let raw = captured.lock().unwrap();
            let text = std::str::from_utf8(&raw).unwrap();
            assert!(text.starts_with("POST /api/v1/responses HTTP/1.1"));
            assert!(text.contains(&format!("authorization: Bearer {KEY}")));
            assert!(!text.contains(scope.capability()));
            assert!(!text.to_lowercase().contains("x-api-key"));
            assert!(!text.to_lowercase().contains("openai-organization"));
            assert!(!response.windows(KEY.len()).any(|w| w == KEY.as_bytes()));
        }
    }
    #[test]
    fn rejected_auth_routing_smuggling_and_paths_never_contact_provider() {
        let backend = TcpListener::bind("127.0.0.1:0").unwrap();
        backend.set_nonblocking(true).unwrap();
        let addr = backend.local_addr().unwrap();
        let (mut relay, scope) = start(&format!("http://{addr}/v1"), true);
        let raw = String::from_utf8(wire(&scope, "")).unwrap();
        for bytes in [
            raw.replace(scope.capability(), "wrong"),
            raw.replace("/v1/responses", "https://other.example/v1/responses"),
            raw.replace("/v1/responses", "/v1/../admin"),
            String::from_utf8(wire(&scope, "Transfer-Encoding: chunked\r\n")).unwrap(),
            String::from_utf8(wire(&scope, "Forwarded: host=other\r\n")).unwrap(),
        ] {
            let response = exchange(&scope, bytes.as_bytes(), false);
            assert!(response.starts_with(b"HTTP/1.1 400") || response.starts_with(b"HTTP/1.1 401"));
        }
        assert_eq!(
            backend.accept().unwrap_err().kind(),
            io::ErrorKind::WouldBlock
        );
        stop(&mut relay);
    }
    #[test]
    fn never_follows_provider_redirect_or_relays_location_headers() {
        let backend = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = backend.local_addr().unwrap();
        let other = TcpListener::bind("127.0.0.1:0").unwrap();
        other.set_nonblocking(true).unwrap();
        let response=format!("HTTP/1.1 307 Temporary Redirect\r\nLocation: http://{}/secret\r\nContent-Type: application/json\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",other.local_addr().unwrap());
        let owner = upstream_once(
            backend,
            vec![response.into_bytes()],
            Arc::new(Mutex::new(Vec::new())),
        );
        let (mut relay, scope) = start(&format!("http://{addr}/v1"), true);
        let out = exchange(&scope, &wire(&scope, ""), false);
        assert!(out.starts_with(b"HTTP/1.1 502"));
        assert!(!out.windows(8).any(|w| w.eq_ignore_ascii_case(b"location")));
        assert_eq!(
            other.accept().unwrap_err().kind(),
            io::ErrorKind::WouldBlock
        );
        stop(&mut relay);
        owner.join().unwrap();
    }
    #[test]
    fn provider_encoding_rejects_duplicate_content_encoding() {
        let payload: &[u8] = &[
            31, 139, 8, 0, 0, 0, 0, 0, 2, 255, 171, 86, 202, 207, 86, 178, 42, 41, 42, 77, 173, 5,
            0, 144, 95, 212, 167, 11, 0, 0, 0,
        ];
        let backend = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = backend.local_addr().unwrap();
        let mut response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Encoding: identity\r\nContent-Encoding: gzip\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", payload.len()).into_bytes();
        response.extend_from_slice(payload);
        let owner = upstream_once(backend, vec![response], Arc::new(Mutex::new(Vec::new())));
        let (mut relay, scope) = start(&format!("http://{addr}/v1"), false);
        let out = exchange(&scope, &wire(&scope, ""), false);
        stop(&mut relay);
        owner.join().unwrap();
        assert!(
            out.starts_with(b"HTTP/1.1 502"),
            "compressed content reached the supplier"
        );
    }
    #[test]
    fn provider_encoding_rejects_compressed_transfer_coding() {
        let payload: &[u8] = &[
            31, 139, 8, 0, 0, 0, 0, 0, 2, 255, 171, 86, 202, 207, 86, 178, 42, 41, 42, 77, 173, 5,
            0, 144, 95, 212, 167, 11, 0, 0, 0,
        ];
        let backend = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = backend.local_addr().unwrap();
        let mut response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nTransfer-Encoding: gzip, chunked\r\nConnection: close\r\n\r\n{:x}\r\n", payload.len()).into_bytes();
        response.extend_from_slice(payload);
        response.extend_from_slice(b"\r\n0\r\n\r\n");
        let owner = upstream_once(backend, vec![response], Arc::new(Mutex::new(Vec::new())));
        let (mut relay, scope) = start(&format!("http://{addr}/v1"), false);
        let out = exchange(&scope, &wire(&scope, ""), false);
        stop(&mut relay);
        owner.join().unwrap();
        assert!(
            out.starts_with(b"HTTP/1.1 502"),
            "compressed transfer reached the supplier"
        );
    }
    #[test]
    fn provider_encoding_keeps_identity_and_plain_chunked() {
        for response in [
            b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Encoding: identity\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{}".as_slice(),
            b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Encoding: identity\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n2\r\n{}\r\n0\r\n\r\n".as_slice(),
        ] {
            let backend = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = backend.local_addr().unwrap();
            let owner = upstream_once(backend, vec![response.to_vec()], Arc::new(Mutex::new(Vec::new())));
            let (mut relay, scope) = start(&format!("http://{addr}/v1"), false);
            let out = exchange(&scope, &wire(&scope, ""), false);
            stop(&mut relay); owner.join().unwrap();
            assert!(out.starts_with(b"HTTP/1.1 200"));
            assert!(out.ends_with(b"0\r\n\r\n"));
        }
    }
    #[test]
    fn reflected_credential_is_not_released_even_across_provider_chunks() {
        let backend = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = backend.local_addr().unwrap();
        let header=format!("HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",KEY.len());
        let owner = upstream_once(
            backend,
            vec![
                header.into_bytes(),
                KEY.as_bytes()[..7].to_vec(),
                KEY.as_bytes()[7..].to_vec(),
            ],
            Arc::new(Mutex::new(Vec::new())),
        );
        let (mut relay, scope) = start(&format!("http://{addr}/v1"), true);
        let out = exchange(&scope, &wire(&scope, ""), false);
        assert!(!out.windows(KEY.len()).any(|w| w == KEY.as_bytes()));
        assert!(!out.ends_with(b"0\r\n\r\n"));
        stop(&mut relay);
        owner.join().unwrap();
    }
    #[test]
    fn no_key_is_explicit_and_never_forwards_the_supplier_capability() {
        let backend = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = backend.local_addr().unwrap();
        let captured = Arc::new(Mutex::new(Vec::new()));
        let owner=upstream_once(backend,vec![b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{}".to_vec()],captured.clone());
        let (mut relay, scope) = start(&format!("http://{addr}/v1"), false);
        assert!(exchange(&scope, &wire(&scope, ""), false).starts_with(b"HTTP/1.1 200"));
        stop(&mut relay);
        owner.join().unwrap();
        assert!(!String::from_utf8_lossy(&captured.lock().unwrap())
            .to_ascii_lowercase()
            .contains("authorization:"));
    }
    #[test]
    fn active_connections_are_bounded_and_stop_aborts_stalled_headers_without_provider_work() {
        let backend = TcpListener::bind("127.0.0.1:0").unwrap();
        backend.set_nonblocking(true).unwrap();
        let (mut relay, scope) = start(
            &format!("http://{}/v1", backend.local_addr().unwrap()),
            false,
        );
        let clients: Vec<_> = (0..6).map(|_| connect(&scope, false)).collect();
        let deadline = Instant::now() + Duration::from_secs(2);
        loop {
            let s = relay.poll();
            assert!(s.active <= MAX_CONNECTIONS);
            if s.active == MAX_CONNECTIONS {
                break;
            }
            assert!(Instant::now() < deadline);
            thread::sleep(Duration::from_millis(5));
        }
        assert_eq!(
            backend.accept().unwrap_err().kind(),
            io::ErrorKind::WouldBlock
        );
        stop(&mut relay);
        drop(clients);
        assert!(Relay::start(
            &registry(),
            scope.clone(),
            Provider::new("http://127.0.0.1:9/v1").unwrap(),
            None
        )
        .is_err());
    }
    #[test]
    fn admission_failure_precedes_worker_and_does_not_consume_service_claim() {
        let host = ScriptedHost::normal();
        let registry = SolveJobRegistry::new(MemoryPool::new(host.clone()));
        let scope = Loopback::bind(&registry).unwrap();
        host.set(|s| s.headroom_bytes = Some(0));
        assert!(matches!(
            Relay::start(
                &registry,
                scope.clone(),
                Provider::new("http://127.0.0.1:9/v1").unwrap(),
                None
            ),
            Err(Failure::Admission)
        ));
        host.set(|s| s.headroom_bytes = Some(64 * 1024 * 1024 * 1024));
        let mut relay = Relay::start(
            &registry,
            scope,
            Provider::new("http://127.0.0.1:9/v1").unwrap(),
            None,
        )
        .unwrap();
        wait_ready(&mut relay);
        stop(&mut relay);
    }
    #[test]
    fn failed_provider_delivery_is_not_automatically_replayed() {
        let backend = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = backend.local_addr().unwrap();
        let monitor = backend.try_clone().unwrap();
        monitor.set_nonblocking(true).unwrap();
        let captured = Arc::new(Mutex::new(Vec::new()));
        let owner = upstream_once(backend, vec![], captured.clone());
        let (mut relay, scope) = start(&format!("http://{addr}/v1"), true);
        assert!(exchange(&scope, &wire(&scope, ""), false).starts_with(b"HTTP/1.1 502"));
        owner.join().unwrap();
        assert!(!captured.lock().unwrap().is_empty());
        assert_eq!(
            monitor.accept().unwrap_err().kind(),
            io::ErrorKind::WouldBlock
        );
        stop(&mut relay);
    }
}

#[cfg(test)]
mod additional_tests {
    use super::*;
    use crate::memory_pool::{test_host::ScriptedHost, MemoryPool};
    use std::{
        io::{Read, Write},
        net::{TcpListener, TcpStream as Client},
        sync::mpsc,
        time::Instant,
    };
    fn registry() -> SolveJobRegistry {
        SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()))
    }
    fn ready(relay: &mut Relay) {
        let end = Instant::now() + Duration::from_secs(5);
        while relay.poll().state != State::Ready {
            assert!(Instant::now() < end);
            thread::sleep(Duration::from_millis(5));
        }
    }
    fn join(relay: &mut Relay) {
        relay.request_stop();
        let end = Instant::now() + Duration::from_secs(5);
        while !relay.poll().joined {
            assert!(Instant::now() < end);
            thread::sleep(Duration::from_millis(5));
        }
    }
    fn request(scope: &Loopback) -> Vec<u8> {
        format!("POST /v1/responses HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nAuthorization: Bearer {}\r\nContent-Length: 2\r\n\r\n{{}}",scope.port(),scope.capability()).into_bytes()
    }
    fn client(scope: &Loopback) -> Client {
        let mut c = Client::connect((std::net::Ipv4Addr::LOCALHOST, scope.port())).unwrap();
        c.set_nonblocking(false).unwrap();
        c.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        c.write_all(&request(scope)).unwrap();
        c
    }
    fn accepted(listener: TcpListener) -> Client {
        listener.set_nonblocking(true).unwrap();
        let end = Instant::now() + Duration::from_secs(5);
        let mut c = loop {
            match listener.accept() {
                Ok((c, _)) => break c,
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    assert!(Instant::now() < end);
                    thread::sleep(Duration::from_millis(5));
                }
                Err(_) => panic!("fixture accept"),
            }
        };
        c.set_nonblocking(false).unwrap();
        c.set_read_timeout(Some(Duration::from_secs(3))).unwrap();
        c.set_write_timeout(Some(Duration::from_secs(3))).unwrap();
        let mut bytes = Vec::new();
        let mut buffer = [0; 2048];
        loop {
            let n = c.read(&mut buffer).unwrap();
            assert_ne!(n, 0);
            bytes.extend_from_slice(&buffer[..n]);
            if bytes
                .windows(4)
                .position(|x| x == b"\r\n\r\n")
                .is_some_and(|n| bytes.len() >= n + 6)
            {
                break;
            }
        }
        c
    }
    #[test]
    fn one_ready_endpoint_is_issued_and_stop_revokes_its_start_standing() {
        let r = registry();
        let scope = Loopback::bind(&r).unwrap();
        let mut relay = Relay::start(
            &r,
            scope,
            Provider::new("http://127.0.0.1:9/v1").unwrap(),
            None,
        )
        .unwrap();
        ready(&mut relay);
        let endpoint = relay.endpoint().unwrap();
        assert!(endpoint.is_ready());
        assert!(relay.endpoint().is_err());
        assert_eq!(format!("{endpoint:?}"), "RelayEndpoint");
        join(&mut relay);
        assert!(!endpoint.is_ready());
    }
    #[test]
    fn response_is_streamed_before_provider_completion() {
        let backend = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = backend.local_addr().unwrap();
        let (release, gate) = mpsc::sync_channel(1);
        let worker = thread::spawn(move || {
            let mut c = accepted(backend);
            c.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n400\r\n").unwrap();
            c.write_all(&vec![b'x'; 1024]).unwrap();
            c.write_all(b"\r\n").unwrap();
            gate.recv_timeout(Duration::from_secs(3)).unwrap();
            let _ = c.write_all(b"0\r\n\r\n");
        });
        let r = registry();
        let scope = Loopback::bind(&r).unwrap();
        let mut relay = Relay::start(
            &r,
            scope.clone(),
            Provider::new(&format!("http://{addr}/v1")).unwrap(),
            None,
        )
        .unwrap();
        ready(&mut relay);
        let mut c = client(&scope);
        let mut buffer = [0; 4096];
        let mut data = Vec::new();
        loop {
            let n = c.read(&mut buffer).unwrap();
            assert_ne!(n, 0);
            data.extend_from_slice(&buffer[..n]);
            if data.windows(32).any(|w| w == [b'x'; 32]) {
                break;
            }
        }
        // The provider has not completed: its gate is still held by this test.
        release.send(()).unwrap();
        let mut rest = Vec::new();
        c.read_to_end(&mut rest).unwrap();
        join(&mut relay);
        worker.join().unwrap();
    }
    #[test]
    fn stop_joins_with_a_backpressured_consumer_and_provider() {
        let backend = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = backend.local_addr().unwrap();
        let (sent, seen) = mpsc::sync_channel(1);
        let worker = thread::spawn(move || {
            let mut c = accepted(backend);
            c.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n").unwrap();
            let _ = sent.send(());
            for _ in 0..4096 {
                if c.write_all(b"4000\r\n")
                    .and_then(|_| c.write_all(&[b'x'; 16384]))
                    .and_then(|_| c.write_all(b"\r\n"))
                    .is_err()
                {
                    break;
                }
            }
        });
        let r = registry();
        let scope = Loopback::bind(&r).unwrap();
        let mut relay = Relay::start(
            &r,
            scope.clone(),
            Provider::new(&format!("http://{addr}/v1")).unwrap(),
            None,
        )
        .unwrap();
        ready(&mut relay);
        let _consumer = client(&scope);
        seen.recv_timeout(Duration::from_secs(3)).unwrap();
        join(&mut relay);
        worker.join().unwrap();
        assert_eq!(relay.poll().active, 0);
    }
    #[test]
    fn oversized_provider_response_is_cut_off_without_successful_chunk_termination() {
        let backend = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = backend.local_addr().unwrap();
        let worker = thread::spawn(move || {
            let mut c = accepted(backend);
            c.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n").unwrap();
            for _ in 0..2100 {
                if c.write_all(b"4000\r\n")
                    .and_then(|_| c.write_all(&[b'x'; 16384]))
                    .and_then(|_| c.write_all(b"\r\n"))
                    .is_err()
                {
                    break;
                }
            }
            let _ = c.write_all(b"0\r\n\r\n");
        });
        let r = registry();
        let scope = Loopback::bind(&r).unwrap();
        let mut relay = Relay::start(
            &r,
            scope.clone(),
            Provider::new(&format!("http://{addr}/v1")).unwrap(),
            None,
        )
        .unwrap();
        ready(&mut relay);
        let mut c = client(&scope);
        let mut raw = Vec::new();
        c.read_to_end(&mut raw).unwrap();
        join(&mut relay);
        worker.join().unwrap();
        assert!(!raw.ends_with(b"0\r\n\r\n"));
        assert!(raw.len() < (policy::RESPONSE_BYTES as usize) + 65536);
        assert_eq!(relay.poll().failed, 1);
    }
    #[test]
    fn dropping_unfinished_worker_retains_backing_even_after_later_finish() {
        let r = registry();
        let scope = Loopback::bind(&r).unwrap();
        let mut relay = Relay::start(
            &r,
            scope.clone(),
            Provider::new("http://127.0.0.1:9/v1").unwrap(),
            None,
        )
        .unwrap();
        ready(&mut relay);
        join(&mut relay);
        // Install a synthetic original worker held at a deterministic stop gate;
        // no provider or supplier runs, and no scheduling race can certify join.
        let (release, gate) = mpsc::sync_channel(1);
        let (done, observed) = mpsc::sync_channel(1);
        relay.worker = Some(thread::spawn(move || {
            gate.recv().unwrap();
            done.send(()).unwrap();
        }));
        let backing = Arc::downgrade(relay.resources.as_ref().unwrap());
        let before = r.pool_status().reserved_bytes;
        drop(relay);
        release.send(()).unwrap();
        observed.recv_timeout(Duration::from_secs(2)).unwrap();
        assert!(backing.upgrade().is_some());
        assert_eq!(r.pool_status().reserved_bytes, before);
    }
}

#[cfg(test)]
mod native_control_tests {
    use super::*;
    use crate::memory_pool::{test_host::ScriptedHost, MemoryPool};
    use std::{
        net::TcpListener,
        process::{Command, Stdio},
    };
    #[test]
    fn proxy_environment_is_ignored_and_loopback_listeners_do_not_survive_exec() {
        let registry = SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()));
        let scope = Loopback::bind(&registry).unwrap();
        let proxy = TcpListener::bind("127.0.0.1:0").unwrap();
        proxy.set_nonblocking(true).unwrap();
        let proxy_url = format!("http://{}", proxy.local_addr().unwrap());
        // Only a new, owned fixture process gets this environment. No global
        // test/process settings or actual provider credentials are changed.
        let output = Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "embedded_agent::http_relay::native_control_tests::isolated_proxy_child",
                "--ignored",
                "--nocapture",
                "--test-threads=1",
            ])
            .env_clear()
            .env("SWBPIPE_RELAY_CONTROL_CHILD", "1")
            .env("SWBPIPE_RELAY_PARENT_PORT", scope.port().to_string())
            .env("HTTP_PROXY", &proxy_url)
            .env("HTTPS_PROXY", &proxy_url)
            .env("ALL_PROXY", &proxy_url)
            .env("http_proxy", &proxy_url)
            .env("https_proxy", &proxy_url)
            .env("all_proxy", &proxy_url)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "isolated relay control fixture failed"
        );
        assert_eq!(
            proxy.accept().unwrap_err().kind(),
            io::ErrorKind::WouldBlock
        );
    }
    #[test]
    #[ignore = "invoked only by owned isolated proxy/descriptor fixture"]
    fn isolated_proxy_child() {
        assert_eq!(
            std::env::var("SWBPIPE_RELAY_CONTROL_CHILD").as_deref(),
            Ok("1")
        );
        let port: u16 = std::env::var("SWBPIPE_RELAY_PARENT_PORT")
            .unwrap()
            .parse()
            .unwrap();
        let registry = SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()));
        let control = Loopback::bind(&registry).unwrap();
        let positive = Command::new("/usr/sbin/lsof")
            .args([
                "-nP",
                "-a",
                "-p",
                &std::process::id().to_string(),
                &format!("-iTCP:{}", control.port()),
            ])
            .env_clear()
            .env("LC_ALL", "C")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap();
        assert!(
            positive.status.success()
                && positive.stdout.windows(4).any(|b| b == b"IPv4")
                && positive.stdout.windows(4).any(|b| b == b"IPv6")
                && positive.stderr.is_empty(),
            "listener observer positive control failed"
        );
        // Read-only self observation. No PID is saved or used as signal authority;
        // raw lsof output (which can contain host identity) is never printed.
        let output = Command::new("/usr/sbin/lsof")
            .args([
                "-nP",
                "-a",
                "-p",
                &std::process::id().to_string(),
                &format!("-iTCP:{port}"),
            ])
            .env_clear()
            .env("LC_ALL", "C")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(1),
            "owned loopback descriptor survived exec or observer unavailable"
        );
        assert!(
            output.stdout.is_empty() && output.stderr.is_empty(),
            "unexpected inherited listener observation"
        );
        super::tests::forwards_actual_json_on_both_families_with_only_host_provider_auth();
    }
}

#[cfg(test)]
mod tls_tests {
    use super::*;
    use crate::memory_pool::{test_host::ScriptedHost, MemoryPool};
    use std::{
        fs,
        io::{Read, Write},
        net::{TcpListener, TcpStream},
        os::unix::fs::DirBuilderExt,
        path::PathBuf,
        process::{Command, Stdio},
        time::Instant,
    };
    struct Fixture(PathBuf);
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    #[test]
    fn untrusted_tls_is_refused_before_provider_auth_or_application_bytes() {
        let mut nonce = [0u8; 16];
        getrandom::fill(&mut nonce).unwrap();
        let nonce: String = nonce.iter().map(|b| format!("{b:02x}")).collect();
        let folder = PathBuf::from("/private/tmp").join(format!("swbpipe-relay-tls-{nonce}"));
        fs::DirBuilder::new().mode(0o700).create(&folder).unwrap();
        let _fixture = Fixture(folder.clone());
        let openssl_config = folder.join("openssl.cnf");
        fs::write(&openssl_config, b"[req]\ndistinguished_name=dn\nx509_extensions=v3\nprompt=no\n[dn]\nCN=localhost\n[v3]\nbasicConstraints=critical,CA:FALSE\nkeyUsage=critical,digitalSignature,keyEncipherment\nextendedKeyUsage=serverAuth\nsubjectAltName=DNS:localhost\n").unwrap();
        let cert = folder.join("cert.der");
        let pem = folder.join("key.pem");
        let key = folder.join("key.der");
        // Ephemeral invented TLS key material, never a provider credential or
        // trusted root installation. Outputs stay in this private fixture only.
        assert!(Command::new("/usr/bin/openssl")
            .args([
                "req",
                "-new",
                "-x509",
                "-newkey",
                "rsa:2048",
                "-nodes",
                "-subj",
                "/CN=localhost",
                "-days",
                "1",
                "-outform",
                "DER",
                "-out"
            ])
            .arg(&cert)
            .arg("-config")
            .arg(&openssl_config)
            .arg("-keyout")
            .arg(&pem)
            .env_clear()
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .unwrap()
            .success());
        assert!(Command::new("/usr/bin/openssl")
            .args(["pkcs8", "-topk8", "-nocrypt", "-inform", "PEM", "-outform", "DER", "-in"])
            .arg(&pem)
            .arg("-out")
            .arg(&key)
            .env_clear()
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .unwrap()
            .success());
        let certificate = rustls::pki_types::CertificateDer::from(fs::read(cert).unwrap());
        let key = rustls::pki_types::PrivateKeyDer::Pkcs8(
            rustls::pki_types::PrivatePkcs8KeyDer::from(fs::read(key).unwrap()),
        );
        let config = rustls::ServerConfig::builder_with_provider(Arc::new(
            rustls::crypto::ring::default_provider(),
        ))
        .with_protocol_versions(&[&rustls::version::TLS13, &rustls::version::TLS12])
        .unwrap()
        .with_no_client_auth()
        .with_single_cert(vec![certificate], key)
        .unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        listener.set_nonblocking(true).unwrap();
        let server = thread::spawn(move || {
            let deadline = Instant::now() + Duration::from_secs(5);
            let mut stream = loop {
                match listener.accept() {
                    Ok((s, _)) => break s,
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                        assert!(Instant::now() < deadline);
                        thread::sleep(Duration::from_millis(5));
                    }
                    Err(_) => panic!("TLS fixture accept"),
                }
            };
            stream.set_nonblocking(false).unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(3)))
                .unwrap();
            stream
                .set_write_timeout(Some(Duration::from_secs(3)))
                .unwrap();
            let mut connection = rustls::ServerConnection::new(Arc::new(config)).unwrap();
            let result = connection.complete_io(&mut stream);
            let mut bytes = [0u8; 1024];
            let read = connection.reader().read(&mut bytes).unwrap_or(0);
            (result.is_err(), read)
        });
        let registry = SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()));
        let scope = Loopback::bind(&registry).unwrap();
        let mut relay = Relay::start(
            &registry,
            scope.clone(),
            Provider::new(&format!("https://{address}/v1")).unwrap(),
            Some(Secret::new("fixture-TLS-credential-long".into()).unwrap()),
        )
        .unwrap();
        let deadline = Instant::now() + Duration::from_secs(5);
        while relay.poll().state != State::Ready {
            assert!(Instant::now() < deadline);
            thread::sleep(Duration::from_millis(5));
        }
        let mut client = TcpStream::connect((std::net::Ipv4Addr::LOCALHOST, scope.port())).unwrap();
        client
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        write!(client,"POST /v1/responses HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nAuthorization: Bearer {}\r\nContent-Length: 2\r\n\r\n{{}}",scope.port(),scope.capability()).unwrap();
        let mut response = Vec::new();
        client.read_to_end(&mut response).unwrap();
        assert!(response.starts_with(b"HTTP/1.1 502"));
        let (failed, read) = server.join().unwrap();
        assert!(failed);
        assert_eq!(read, 0);
        relay.request_stop();
        let deadline = Instant::now() + Duration::from_secs(5);
        while !relay.poll().joined {
            assert!(Instant::now() < deadline);
            thread::sleep(Duration::from_millis(5));
        }
    }
}
