//! Explicitly owned loopback scope for the future provider relay. This is not
//! an HTTP relay, authentication server, process starter or background worker.
//! No listener is created until bind is called after GENERAL admission.
use crate::{memory_pool::Lease, solve_jobs::SolveJobRegistry};
use std::{
    fmt, io,
    net::{Ipv4Addr, Ipv6Addr, SocketAddr, TcpListener, TcpStream},
    sync::Arc,
};

/// Userspace state, capability and socket wrapper allowance, not kernel socket
/// queues/RSS or future HTTP/worker pricing. Those need separate admission.
pub const STATE_BYTES: u64 = 64 * 1024;
const ATTEMPTS: usize = 16;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Admission,
    Entropy,
    Bind,
}

/// Both named loopback endpoints are held at one port. No mutable listener
/// accessor, Clone, Serialize or port-only constructor can sever this custody.
/// The native launcher retains an Arc through original supplier-group cleanup.
pub struct Loopback {
    ipv4: TcpListener,
    ipv6: TcpListener,
    port: u16,
    capability: String,
    _backing: Lease,
}
impl Loopback {
    pub fn bind(registry: &SolveJobRegistry) -> Result<Arc<Self>, Error> {
        let backing = registry
            .acquire_interactive_ingress(STATE_BYTES)
            .map_err(|_| Error::Admission)?;
        let mut random = [0u8; 32];
        getrandom::fill(&mut random).map_err(|_| Error::Entropy)?;
        let capability: String = random.iter().map(|b| format!("{b:02x}")).collect();
        for _ in 0..ATTEMPTS {
            let ipv4 = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).map_err(|_| Error::Bind)?;
            let port = ipv4.local_addr().map_err(|_| Error::Bind)?.port();
            let ipv6 = match TcpListener::bind((Ipv6Addr::LOCALHOST, port)) {
                Ok(listener) => listener,
                Err(error) if error.kind() == io::ErrorKind::AddrInUse => continue,
                Err(_) => return Err(Error::Bind),
            };
            ipv4.set_nonblocking(true).map_err(|_| Error::Bind)?;
            ipv6.set_nonblocking(true).map_err(|_| Error::Bind)?;
            return Ok(Arc::new(Self {
                ipv4,
                ipv6,
                port,
                capability,
                _backing: backing,
            }));
        }
        Err(Error::Bind)
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    /// Returned only to the private child-environment/relay owner, never settings
    /// or diagnostics. It is not the configured provider's credential.
    pub(super) fn capability(&self) -> &str {
        &self.capability
    }
    pub fn accepts_capability(&self, candidate: &[u8]) -> bool {
        if candidate.len() != self.capability.len() {
            return false;
        }
        candidate
            .iter()
            .zip(self.capability.bytes())
            .fold(0u8, |diff, (a, b)| diff | (*a ^ b))
            == 0
    }
    /// Future relay workers must reserve their streams/buffers before accepting.
    /// Nonblocking: WouldBlock is not EOF, termination or failed authentication.
    pub fn accept_ipv4(&self) -> io::Result<(TcpStream, SocketAddr)> {
        self.ipv4.accept()
    }
    pub fn accept_ipv6(&self) -> io::Result<(TcpStream, SocketAddr)> {
        self.ipv6.accept()
    }
}
impl fmt::Debug for Loopback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Loopback([REDACTED])")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_pool::{test_host::ScriptedHost, MemoryPool};
    fn registry() -> SolveJobRegistry {
        SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()))
    }
    #[test]
    fn owns_both_families_on_one_runtime_selected_port_without_workers() {
        let scope = Loopback::bind(&registry()).unwrap();
        assert_ne!(scope.port(), 0);
        assert_eq!(
            scope.ipv4.local_addr().unwrap(),
            SocketAddr::from((Ipv4Addr::LOCALHOST, scope.port()))
        );
        assert_eq!(
            scope.ipv6.local_addr().unwrap(),
            SocketAddr::from((Ipv6Addr::LOCALHOST, scope.port()))
        );
        assert!(TcpListener::bind((Ipv4Addr::LOCALHOST, scope.port())).is_err());
        assert!(TcpListener::bind((Ipv6Addr::LOCALHOST, scope.port())).is_err());
        assert_eq!(
            scope.accept_ipv4().unwrap_err().kind(),
            io::ErrorKind::WouldBlock
        );
        assert_eq!(
            scope.accept_ipv6().unwrap_err().kind(),
            io::ErrorKind::WouldBlock
        );
    }
    #[test]
    fn shared_custody_retains_both_endpoints() {
        let scope = Loopback::bind(&registry()).unwrap();
        let port = scope.port();
        let owner = scope.clone();
        drop(scope);
        assert!(TcpListener::bind((Ipv4Addr::LOCALHOST, port)).is_err());
        assert!(TcpListener::bind((Ipv6Addr::LOCALHOST, port)).is_err());
        drop(owner);
        assert!(TcpListener::bind((Ipv4Addr::LOCALHOST, port)).is_ok());
        assert!(TcpListener::bind((Ipv6Addr::LOCALHOST, port)).is_ok());
    }
    #[test]
    fn fresh_per_start_capability_is_redacted_and_never_accepts_a_prefix() {
        let a = Loopback::bind(&registry()).unwrap();
        let b = Loopback::bind(&registry()).unwrap();
        assert_ne!(a.capability(), b.capability());
        assert_eq!(a.capability().len(), 64);
        assert!(a.capability().bytes().all(|c| c.is_ascii_hexdigit()));
        assert!(a.accepts_capability(a.capability().as_bytes()));
        assert!(!a.accepts_capability(b.capability().as_bytes()));
        assert!(!a.accepts_capability(&a.capability().as_bytes()[..63]));
        assert_eq!(format!("{a:?}"), "Loopback([REDACTED])");
    }
    #[test]
    fn resource_refusal_precedes_any_endpoint_or_capability() {
        let host = ScriptedHost::normal();
        host.set(|s| s.headroom_bytes = Some(0));
        let registry = SolveJobRegistry::new(MemoryPool::new(host));
        assert_eq!(Loopback::bind(&registry).unwrap_err(), Error::Admission);
    }
}
