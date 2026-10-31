//! Trusted HOST-DRIVEN protocol session: call bounded `poll` continuously while
//! domain/human work proceeds elsewhere. This is no live application bridge,
//! authorization, scheduler, process owner or Tauri route. No additional threads
//! beyond Pump's two workers. Token uniqueness/unpredictability is the host's duty.
//! The dispatcher authenticates session/workspace/generation/call-specific basis;
//! retain actual domain outcomes outside this object. Historical results are
//! never rebased. QUEUED means accepted by Pump, never peer delivery/application.
//!
//! All allowances are nominal GENERAL reservations, not allocator/OS/RSS caps or
//! retained-route qualification. Caller heap (streams/config/outcomes) must have
//! already been admitted by its owner. Generic blocked I/O cannot be closed here:
//! stop original process/streams externally, retain custody and observe joined
//! workers. Neither EOF nor joined workers proves process/descendant exit. Pump's
//! documented fail-closed reservation retention applies on abandonment.

use super::{
    catalog::{Catalog, Tool},
    protocol::{self, Event, Protocol, ThreadOptions},
    stdio_pump::{self, Pump},
};
use crate::{memory_pool::Lease, solve_jobs::SolveJobRegistry};
use serde_json::{json, Value};
use std::io::{self, Read, Write};
use std::sync::Arc;

/// Catalog/expanded declarations, 1024 retained IDs bounded at 256 bytes,
/// references/containers, trusted basis and disconnect bookkeeping, for v2.
pub const SESSION_BYTES: u64 = 8 * 1024 * 1024;
/// Additional per-frame/control allowance: <=64 KiB outcome/text plus encoded
/// text/wrapper, or <=17 detach events/IDs. Frame materialization is separate.
pub const DERIVED_BYTES: u64 = 2 * 1024 * 1024;
const BASIS_BYTES: usize = 16 * 1024;
/// Catalog v2 permits 16 concurrent calls, plus one turn-completion event.
pub const MAX_EVENTS_PER_POLL: usize = 17;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    Configuration,
    Admission,
    InvalidControl,
    InvalidCompletion,
    Oversized,
    Closed,
    /// A fixed control has not changed protocol state; retain/retry its input.
    /// Polling may instead have deferred one owned batch for its next attempt.
    Busy,
    Transport(stdio_pump::Fault),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Terminal {
    StopRequested,
    Io,
    Protocol,
    Admission,
    Send,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Submission {
    Queued,
    Detached,
}

/// Borrowed inspection only: moving a payload out would sever its backing.
/// Dispatchers retaining copies must admit those copies themselves. Debug never
/// emits payloads, private paths, identities or credentials.
pub struct EventBatch {
    events: Vec<Event>,
    _backing: Arc<Lease>,
    _session: Arc<Lease>,
}
impl EventBatch {
    pub fn events(&self) -> &[Event] {
        &self.events
    }
}
impl std::fmt::Debug for EventBatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("EventBatch")
    }
}
#[derive(Debug)]
pub struct Observation {
    pub io: stdio_pump::Status,
    pub terminal: Option<Terminal>,
    pub events: Option<EventBatch>,
}
/// Partial is custody, not rollback. Keep observing until actual joined workers.
pub enum StartError {
    Refused(Refusal),
    Partial {
        fault: stdio_pump::Fault,
        custody: SessionDriver,
    },
}
impl std::fmt::Debug for StartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Refused(r) => f.debug_tuple("Refused").field(r).finish(),
            Self::Partial { fault, .. } => f.debug_tuple("PartialCustody").field(fault).finish(),
        }
    }
}
pub struct SessionDriver {
    protocol: Protocol,
    basis: Value,
    registry: SolveJobRegistry,
    pump: Pump,
    terminal: Option<Terminal>,
    // One disconnect batch, reported once; no history or redispatch queue.
    lost: Option<EventBatch>,
    // At most one frame batch deferred across a Busy final observation.
    deferred: Option<EventBatch>,
    // Last: destroy retained state before releasing its reservation.
    session: Arc<Lease>,
}
impl SessionDriver {
    pub fn last_rejection(&self) -> Option<protocol::RpcRejection> {
        self.protocol.last_rejection()
    }

    /// Stream destructors must be nonblocking, including constructor refusals;
    /// this inherits Pump's generic-stream contract and no wall-clock guarantee.
    pub fn start<R: Read + Send + 'static, W: Write + Send + 'static>(
        source: R,
        sink: W,
        registry: SolveJobRegistry,
        token: &str,
        proposal_cap: u64,
        error_basis: &Value,
    ) -> Result<Self, StartError> {
        // Before copying caller data, loading catalog or constructing derived JSON.
        if !(16..=128).contains(&token.len())
            || proposal_cap == 0
            || !token
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
            || !bounded_value(error_basis, BASIS_BYTES)
        {
            return Err(StartError::Refused(Refusal::Configuration));
        }
        let session = Arc::new(
            registry
                .acquire_interactive_ingress(SESSION_BYTES)
                .map_err(|_| StartError::Refused(Refusal::Admission))?,
        );
        let catalog = Catalog::load().map_err(|_| StartError::Refused(Refusal::Configuration))?;
        let limits = catalog.limits();
        if limits.result_bytes > 64 * 1024
            || limits.argument_bytes > 64 * 1024
            || limits.pending_calls > (MAX_EVENTS_PER_POLL - 1) as u64
        {
            return Err(StartError::Refused(Refusal::Configuration));
        }
        validate_basis(&catalog, error_basis).map_err(StartError::Refused)?;
        let protocol = Protocol::new(catalog, token.to_owned(), proposal_cap)
            .map_err(|_| StartError::Refused(Refusal::Configuration))?;
        let basis = error_basis.clone();
        let started = Pump::start(source, sink, registry.clone(), limits);
        Self::finish_start(protocol, basis, registry, session, started)
    }
    fn finish_start(
        protocol: Protocol,
        basis: Value,
        registry: SolveJobRegistry,
        session: Arc<Lease>,
        started: Result<Pump, stdio_pump::StartError>,
    ) -> Result<Self, StartError> {
        let (pump, partial) = match started {
            Ok(p) => (p, None),
            Err(stdio_pump::StartError::Refused(f)) => {
                return Err(StartError::Refused(Refusal::Transport(f)))
            }
            Err(stdio_pump::StartError::Partial { fault, custody }) => (custody, Some(fault)),
        };
        let mut driver = Self {
            protocol,
            basis,
            registry,
            pump,
            terminal: None,
            lost: None,
            deferred: None,
            session,
        };
        if let Some(fault) = partial {
            driver.close(Terminal::Io);
            Err(StartError::Partial {
                fault,
                custody: driver,
            })
        } else {
            Ok(driver)
        }
    }
    fn backing(&self) -> Result<Arc<Lease>, Refusal> {
        self.registry
            .acquire_interactive_ingress(DERIVED_BYTES)
            .map(Arc::new)
            .map_err(|_| Refusal::Admission)
    }
    fn close(&mut self, why: Terminal) {
        if self.terminal.is_none() {
            self.terminal = Some(why);
            self.deferred = None;
            self.pump.request_stop();
            let events = self.protocol.disconnect(); // Exactly once.
            self.lost = Some(EventBatch {
                events,
                _backing: self.session.clone(),
                _session: self.session.clone(),
            });
        }
    }
    fn live(&mut self) -> Result<(), Refusal> {
        if self.terminal.is_some() {
            return Err(Refusal::Closed);
        }
        let status = self.pump.observe().map_err(map_fault)?;
        if ended(&status) {
            self.close(Terminal::Io);
            return Err(Refusal::Closed);
        }
        Ok(())
    }
    fn send(&mut self, value: Value, backing: Arc<Lease>) -> Result<Submission, Refusal> {
        // Callers checked liveness before changing protocol state. Do not add a
        // fallible try-lock observation after that transition: ordinary queue
        // contention would otherwise turn a valid control into a disconnect.
        // Enqueue checks stop/writer failure under its bookkeeping lock. A later
        // EOF is still observed by poll; queued never means delivered to a peer.
        match self.pump.try_send(value, backing) {
            Ok(()) => Ok(Submission::Queued),
            Err(f) => {
                self.close(Terminal::Send);
                Err(Refusal::Transport(f))
            }
        }
    }
    pub fn initialize(&mut self, name: &str, version: &str) -> Result<Submission, Refusal> {
        self.live()?;
        if !control(name, 128) || !control(version, 128) {
            return Err(Refusal::InvalidControl);
        }
        let backing = self.backing()?;
        let value = self
            .protocol
            .initialize(name, version)
            .map_err(|_| Refusal::InvalidControl)?;
        self.send(value, backing)
    }
    pub fn start_thread(&mut self, options: &ThreadOptions) -> Result<Submission, Refusal> {
        self.live()?;
        if !control(&options.model, 1024)
            || !control(&options.provider, 1024)
            || !control(&options.cwd, 4096)
            || options.base_instructions.is_empty()
            || options.base_instructions.len() > protocol::MAX_CONTROL_BYTES
            || options.base_instructions.contains('\0')
        {
            return Err(Refusal::InvalidControl);
        }
        let backing = self.backing()?;
        let value = self
            .protocol
            .start_thread(ThreadOptions {
                model: options.model.clone(),
                provider: options.provider.clone(),
                cwd: options.cwd.clone(),
                base_instructions: options.base_instructions.clone(),
            })
            .map_err(|_| Refusal::InvalidControl)?;
        self.send(value, backing)
    }
    pub fn submit_text(&mut self, text: &str) -> Result<Submission, Refusal> {
        self.live()?;
        if text.is_empty() || text.len() > protocol::MAX_TEXT_BYTES || text.contains('\0') {
            return Err(Refusal::InvalidControl);
        }
        let backing = self.backing()?;
        let value = self
            .protocol
            .submit_text(text.to_owned())
            .map_err(|_| Refusal::InvalidControl)?;
        self.send(value, backing)
    }
    pub fn interrupt(&mut self) -> Result<Submission, Refusal> {
        self.live()?;
        let backing = self.backing()?;
        let value = self
            .protocol
            .interrupt()
            .map_err(|_| Refusal::InvalidControl)?;
        self.send(value, backing)
    }
    /// Caller outcome heap must already be admitted. Retain its backing while
    /// consumed AND reserve additional response backing before output encoding.
    /// Invalid outcome/reference preserves the valid pending entry. After schema
    /// acceptance a send refusal consumes the reference: preserve that correlated
    /// outcome externally and treat delivery as unresolved; never retry it.
    /// The driver
    /// never substitutes a newer basis or stores an application record.
    pub fn complete(
        &mut self,
        reference: &str,
        outcome: Value,
        outcome_backing: Arc<Lease>,
    ) -> Result<Submission, Refusal> {
        // Field order keeps backing through Value destruction on EVERY refusal.
        struct Outcome {
            value: Value,
            backing: Arc<Lease>,
        }
        let outcome = Outcome {
            value: outcome,
            backing: outcome_backing,
        };
        self.live()?;
        // A lease reference is custody, not permission to reuse another value's
        // budget. Adequacy for caller heap/capacities remains the trusted owner's
        // obligation; reject an explicitly unpriced value even before schema work.
        if outcome.backing.bytes() < std::mem::size_of::<Value>() as u64 {
            return Err(Refusal::Admission);
        }
        if !control(reference, 160) {
            return Err(Refusal::InvalidCompletion);
        }
        if !bounded_value(&outcome.value, 64 * 1024) {
            return Err(Refusal::Oversized);
        }
        let backing = self.backing()?;
        let result = self
            .protocol
            .complete(reference, outcome.value)
            .map_err(|_| Refusal::InvalidCompletion);
        drop(outcome.backing);
        match result? {
            protocol::Completion::Delivered(v) => self.send(v, backing),
            protocol::Completion::Detached => Ok(Submission::Detached),
        }
    }
    /// Only CURRENT fallback errors use this basis; historical outcomes do not.
    pub fn set_error_basis(&mut self, basis: &Value) -> Result<(), Refusal> {
        self.live()?;
        if !bounded_value(basis, BASIS_BYTES) {
            return Err(Refusal::Oversized);
        }
        let _temporary = self.backing()?;
        let catalog = Catalog::load().map_err(|_| Refusal::Configuration)?;
        validate_basis(&catalog, basis)?;
        self.basis = basis.clone();
        Ok(())
    }
    pub fn stop(&mut self) {
        self.close(Terminal::StopRequested);
    }
    /// At most ONE frame and <=MAX_EVENTS_PER_POLL events. No stream/domain wait. A
    /// terminal observation wins over queued work. Failure can occur after any
    /// observation: the trusted dispatcher must recheck session authority before
    /// execution and preserve actual outcomes, as with every asynchronous bridge.
    pub fn poll(&mut self) -> Result<Observation, Refusal> {
        let initial = self.pump.observe().map_err(map_fault)?;
        if ended(&initial) {
            self.close(Terminal::Io);
        }
        if self.terminal.is_some() {
            return Ok(Observation {
                io: initial,
                terminal: self.terminal,
                events: self.lost.take(),
            });
        }
        if let Some(batch) = self.deferred.take() {
            return Ok(Observation {
                io: initial,
                terminal: None,
                events: Some(batch),
            });
        }
        let mut batch = None;
        if let Some(mut frame) = self.pump.try_receive().map_err(map_fault)? {
            // Grow BEFORE interpretation/derived allocations. One shared lease
            // covers all frame derivatives together; clones are custody only.
            if frame
                .lease
                .grow_to(frame.lease.bytes() + DERIVED_BYTES)
                .is_err()
            {
                self.close(Terminal::Admission);
            } else {
                let backing = Arc::new(frame.lease);
                match self.protocol.handle_frame(frame.value, &self.basis) {
                    Err(_) => self.close(Terminal::Protocol),
                    Ok(actions) => {
                        for reply in actions.outbound {
                            if self.send(reply, backing.clone()).is_err() {
                                break;
                            }
                        }
                        batch = Some(EventBatch {
                            events: actions.events,
                            _backing: backing,
                            _session: self.session.clone(),
                        });
                    }
                }
            }
        }
        // Busy cannot allow newly executable work to escape. Defer ONE leased
        // batch until a fresh terminal check; never process another frame first.
        let status = match self.pump.observe() {
            Ok(s) => s,
            Err(stdio_pump::Fault::Busy) => {
                if self.terminal.is_none() {
                    self.deferred = batch;
                }
                return Err(Refusal::Busy);
            }
            Err(f) => {
                self.close(Terminal::Io);
                return Err(map_fault(f));
            }
        };
        if ended(&status) {
            self.close(Terminal::Io);
        }
        if self.terminal.is_some() {
            batch = self.lost.take();
        }
        Ok(Observation {
            io: status,
            terminal: self.terminal,
            events: batch,
        })
    }
}
fn ended(s: &stdio_pump::Status) -> bool {
    s.stop_requested
        || s.fault.is_some()
        || s.reader_io.is_some()
        || s.writer_io.is_some()
        || matches!(
            s.reader_completion,
            stdio_pump::Completion::Joined | stdio_pump::Completion::Panicked
        )
        || matches!(
            s.writer_completion,
            stdio_pump::Completion::Joined | stdio_pump::Completion::Panicked
        )
}
fn map_fault(f: stdio_pump::Fault) -> Refusal {
    if f == stdio_pump::Fault::Busy {
        Refusal::Busy
    } else {
        Refusal::Transport(f)
    }
}
pub(super) fn control(s: &str, cap: usize) -> bool {
    !s.is_empty() && s.len() <= cap && !s.chars().any(char::is_control)
}
pub(super) fn validate_basis(catalog: &Catalog, basis: &Value) -> Result<(), Refusal> {
    let output = json!({"status":"execution_error","basis":basis,"message":"Tool request could not be admitted.","data":null});
    for tool in Tool::ALL {
        catalog
            .validate_output(tool, &output)
            .map_err(|_| Refusal::Configuration)?;
    }
    Ok(())
}
// Allocation-free, depth/node/size precheck before streaming a caller Value.
// This checks size; caller remains responsible for admission of existing heap.
pub(super) fn bounded_value(v: &Value, cap: usize) -> bool {
    fn walk(v: &Value, depth: usize, nodes: &mut usize, bytes: &mut usize) -> bool {
        if depth > 32 || *nodes == 0 {
            return false;
        }
        *nodes -= 1;
        let size = match v {
            Value::String(s) => s.len(),
            _ => 8,
        };
        let Some(left) = bytes.checked_sub(size) else {
            return false;
        };
        *bytes = left;
        match v {
            Value::Array(a) => a.iter().all(|v| walk(v, depth + 1, nodes, bytes)),
            Value::Object(o) => o.iter().all(|(k, v)| {
                let Some(left) = bytes.checked_sub(k.len()) else {
                    return false;
                };
                *bytes = left;
                walk(v, depth + 1, nodes, bytes)
            }),
            _ => true,
        }
    }
    struct Cap(usize);
    impl Write for Cap {
        fn write(&mut self, b: &[u8]) -> io::Result<usize> {
            self.0 = self
                .0
                .checked_sub(b.len())
                .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?;
            Ok(b.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    walk(v, 0, &mut 8192, &mut cap.saturating_mul(2)) && serde_json::to_writer(Cap(cap), v).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_pool::{test_host::ScriptedHost, MemoryPool};
    use std::collections::VecDeque;
    use std::sync::{Condvar, Mutex};
    use std::time::{Duration, Instant};

    #[derive(Default)]
    struct Input {
        state: Mutex<(VecDeque<u8>, bool, bool)>, // bytes, EOF, fault
        wake: Condvar,
    }
    impl Input {
        fn frame(&self, v: Value) {
            let mut s = self.state.lock().unwrap();
            s.0.extend(serde_json::to_vec(&v).unwrap());
            s.0.push_back(b'\n');
            self.wake.notify_all();
        }
        fn end(&self, fault: bool) {
            let mut s = self.state.lock().unwrap();
            s.1 = true;
            s.2 = fault;
            self.wake.notify_all();
        }
    }
    struct Source(Arc<Input>);
    impl Read for Source {
        fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
            let mut s = self.0.state.lock().unwrap();
            while s.0.is_empty() && !s.1 {
                s = self.0.wake.wait(s).unwrap();
            }
            if s.2 {
                return Err(io::Error::from(io::ErrorKind::Other));
            }
            let n = bytes.len().min(s.0.len());
            for b in &mut bytes[..n] {
                *b = s.0.pop_front().unwrap();
            }
            Ok(n)
        }
    }
    #[derive(Default)]
    struct Output {
        state: Mutex<(Vec<u8>, usize, bool, bool)>, // bytes, flushes, stalled, release/fail
        wake: Condvar,
    }
    struct Sink(Arc<Output>);
    impl Write for Sink {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            let mut s = self.0.state.lock().unwrap();
            while s.2 && !s.3 {
                s = self.0.wake.wait(s).unwrap();
            }
            if s.3 {
                return Err(io::Error::from(io::ErrorKind::Other));
            }
            s.0.extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            self.0.state.lock().unwrap().1 += 1;
            self.0.wake.notify_all();
            Ok(())
        }
    }
    fn basis() -> Value {
        json!({"workspaceHandle":"workspace-1","applicationGeneration":1,"modelRevision":2,"modelHash":null,"stateId":null,"runId":null})
    }
    fn outcome() -> Value {
        json!({"status":"unavailable","basis":basis(),"message":"Unavailable","data":null})
    }
    fn options() -> ThreadOptions {
        ThreadOptions {
            model: "test".into(),
            provider: "test".into(),
            cwd: "/test".into(),
            base_instructions: "test".into(),
        }
    }
    struct Harness {
        driver: Option<SessionDriver>,
        input: Arc<Input>,
        output: Arc<Output>,
        registry: SolveJobRegistry,
    }
    impl Harness {
        fn new() -> Self {
            let registry = SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()));
            let input = Arc::new(Input::default());
            let output = Arc::new(Output::default());
            let driver = SessionDriver::start(
                Source(input.clone()),
                Sink(output.clone()),
                registry.clone(),
                "0123456789abcdef",
                16,
                &basis(),
            )
            .unwrap();
            Self {
                driver: Some(driver),
                input,
                output,
                registry,
            }
        }
        fn d(&mut self) -> &mut SessionDriver {
            self.driver.as_mut().unwrap()
        }
        fn control<T>(
            &mut self,
            mut action: impl FnMut(&mut SessionDriver) -> Result<T, Refusal>,
        ) -> Result<T, Refusal> {
            let deadline = Instant::now() + Duration::from_secs(5);
            loop {
                match action(self.d()) {
                    Err(Refusal::Busy) => {
                        assert!(Instant::now() < deadline, "control contention timeout");
                        std::thread::sleep(Duration::from_millis(1));
                    }
                    result => return result,
                }
            }
        }
        // Bounded fixture wait; yields between observations, never busy-spins.
        fn wait(&mut self, mut done: impl FnMut(&Observation) -> bool) -> Observation {
            let deadline = Instant::now() + Duration::from_secs(5);
            loop {
                match self.d().poll() {
                    Ok(o) if done(&o) => return o,
                    Ok(_) | Err(Refusal::Busy) => {}
                    Err(e) => panic!("unexpected static refusal: {e:?}"),
                }
                assert!(Instant::now() < deadline, "fixture observation timeout");
                std::thread::sleep(Duration::from_millis(1));
            }
        }
        fn event(&mut self) -> EventBatch {
            self.wait(|o| o.events.as_ref().is_some_and(|b| !b.events().is_empty()))
                .events
                .unwrap()
        }
        fn sent(&self, count: usize) -> Vec<Value> {
            let s = self.output.state.lock().unwrap();
            let (s, _) = self
                .output
                .wake
                .wait_timeout_while(s, Duration::from_secs(5), |s| s.1 < count)
                .unwrap();
            assert!(s.1 >= count, "fixture flush timeout");
            s.0.split(|b| *b == b'\n')
                .filter(|b| !b.is_empty())
                .map(|b| serde_json::from_slice(b).unwrap())
                .collect()
        }
        fn ready(&mut self) {
            assert_eq!(
                self.control(|d| d.initialize("test", "1")),
                Ok(Submission::Queued)
            );
            assert_eq!(self.sent(1)[0]["method"], "initialize");
            // Queueing alone did not initialize the protocol.
            assert_eq!(
                self.control(|d| d.start_thread(&options())),
                Err(Refusal::InvalidControl)
            );
            self.input.frame(json!({"id":1,"result":{"codexHome":"/test","platformFamily":"test","platformOs":"test","userAgent":"test"}}));
            assert!(matches!(self.event().events(), [Event::Initialized]));
            assert_eq!(self.sent(2)[1]["method"], "initialized");
            assert_eq!(
                self.control(|d| d.start_thread(&options())),
                Ok(Submission::Queued)
            );
            let sent = self.sent(3);
            assert_eq!(sent[2]["method"], "thread/start");
            assert_eq!(
                sent[2]["params"]["dynamicTools"].as_array().unwrap().len(),
                7
            );
            assert_eq!(sent[2]["params"]["approvalPolicy"], "never");
            assert_eq!(
                self.control(|d| d.submit_text("early")),
                Err(Refusal::InvalidControl)
            );
            self.input.frame(json!({"id":2,"result":{"thread":{"id":"thread-1"},"approvalPolicy":"never","sandbox":{"type":"readOnly","networkAccess":false}}}));
            assert!(matches!(
                self.event().events(),
                [Event::ThreadStarted { .. }]
            ));
        }
        fn active(&mut self) {
            self.ready();
            assert_eq!(
                self.control(|d| d.submit_text("hello")),
                Ok(Submission::Queued)
            );
            self.sent(4);
            self.input.frame(
                json!({"id":3,"result":{"turn":{"id":"turn-1","items":[],"status":"inProgress"}}}),
            );
            assert!(matches!(self.event().events(), [Event::TurnStarted { .. }]));
        }
        fn call(&self, id: &str) {
            self.input.frame(json!({"id":id,"method":"item/tool/call","params":{"threadId":"thread-1","turnId":"turn-1","callId":id,"tool":Tool::Summary.name(),"arguments":{}}}));
        }
        fn complete(&mut self, reference: &str, value: Value) -> Result<Submission, Refusal> {
            let lease = Arc::new(
                self.registry
                    .acquire_interactive_ingress(DERIVED_BYTES)
                    .unwrap(),
            );
            let result = self.control(|d| d.complete(reference, value.clone(), lease.clone()));
            drop(value);
            drop(lease);
            result
        }
        fn finish(&mut self) {
            // Unconditional release paths also run on assertion unwind.
            self.input.end(false);
            {
                let mut s = self.output.state.lock().unwrap();
                s.3 = true;
                self.output.wake.notify_all();
            }
            if let Some(driver) = &mut self.driver {
                driver.stop();
                let deadline = Instant::now() + Duration::from_secs(5);
                loop {
                    if let Ok(o) = driver.poll() {
                        if o.io.reader_completion == stdio_pump::Completion::Joined
                            && o.io.writer_completion == stdio_pump::Completion::Joined
                        {
                            break;
                        }
                    }
                    if Instant::now() >= deadline {
                        break;
                    }
                    std::thread::sleep(Duration::from_millis(1));
                }
            }
        }
    }
    impl Drop for Harness {
        fn drop(&mut self) {
            self.finish();
        }
    }
    fn reference(batch: &EventBatch) -> String {
        match batch.events() {
            [Event::ToolRequested { request_ref, .. }] => request_ref.as_str().to_owned(),
            _ => panic!("expected leased request"),
        }
    }

    #[test]
    fn handshake_and_original_rpc_completion_are_streamed_and_leased() {
        let mut h = Harness::new();
        h.active();
        h.call("original-rpc");
        let batch = h.event();
        let reference = reference(&batch);
        assert_eq!(h.complete(&reference, outcome()), Ok(Submission::Queued));
        let sent = h.sent(5);
        assert_eq!(sent[4]["id"], "original-rpc");
        assert_eq!(sent[4]["result"]["contentItems"][0]["type"], "inputText");
        let actual: Value = serde_json::from_str(
            sent[4]["result"]["contentItems"][0]["text"]
                .as_str()
                .unwrap(),
        )
        .unwrap();
        assert_eq!(actual, outcome());
        h.finish();
        let registry = h.registry.clone();
        drop(h);
        assert!(!batch.events().is_empty());
        assert!(registry.pool().reserved_total() >= SESSION_BYTES + DERIVED_BYTES);
        drop(batch);
        assert_eq!(registry.pool().reserved_total(), 0);
    }
    #[test]
    fn local_refusals_preserve_pending_and_duplicate_is_refused() {
        let mut h = Harness::new();
        h.active();
        h.call("rpc");
        let batch = h.event();
        let r = reference(&batch);
        let unpriced = Arc::new(h.registry.acquire_interactive_ingress(1).unwrap());
        assert_eq!(
            h.control(|d| d.complete(&r, outcome(), unpriced.clone())),
            Err(Refusal::Admission)
        );
        assert_eq!(
            h.complete("foreign:1", outcome()),
            Err(Refusal::InvalidCompletion)
        );
        assert_eq!(
            h.complete(&r, json!({"status":"ok"})),
            Err(Refusal::InvalidCompletion)
        );
        assert_eq!(
            h.complete(&r, Value::String("x".repeat(65537))),
            Err(Refusal::Oversized)
        );
        assert_eq!(
            h.control(|d| d.initialize("bad", "1")),
            Err(Refusal::InvalidControl)
        );
        assert_eq!(
            h.control(|d| d.submit_text(&"x".repeat(65537))),
            Err(Refusal::InvalidControl)
        );
        assert_eq!(h.complete(&r, outcome()), Ok(Submission::Queued));
        h.sent(5);
        assert_eq!(h.complete(&r, outcome()), Err(Refusal::InvalidCompletion));
    }
    #[test]
    fn interruption_detaches_without_cancellation_or_response() {
        let mut h = Harness::new();
        h.active();
        h.call("rpc");
        let r = reference(&h.event());
        assert_eq!(h.control(|d| d.interrupt()), Ok(Submission::Queued));
        assert_eq!(h.sent(5)[4]["method"], "turn/interrupt");
        h.input.frame(json!({"method":"turn/completed","params":{"threadId":"thread-1","turn":{"id":"turn-1","items":[],"status":"interrupted"}}}));
        assert!(matches!(
            h.event().events(),
            [Event::ToolDetached { .. }, Event::TurnCompleted { .. }]
        ));
        assert_eq!(h.complete(&r, outcome()), Ok(Submission::Detached));
        h.input.frame(json!({"id":4,"result":{}}));
        assert!(matches!(h.event().events(), [Event::InterruptAcknowledged]));
        assert_eq!(h.output.state.lock().unwrap().1, 5);
    }
    #[test]
    fn protocol_failure_reports_loss_once_and_stops_dispatch() {
        let mut h = Harness::new();
        h.active();
        h.call("rpc");
        let _requested = h.event();
        h.input.frame(json!({"id":999,"result":{}}));
        let o = h.wait(|o| o.terminal.is_some());
        assert_eq!(o.terminal, Some(Terminal::Protocol));
        assert!(matches!(
            o.events.unwrap().events(),
            [Event::ToolLost { .. }]
        ));
        assert!(h.wait(|_| true).events.is_none());
        assert_eq!(h.control(|d| d.submit_text("again")), Err(Refusal::Closed));
    }
    #[test]
    fn observed_eof_or_read_fault_suppresses_already_queued_calls() {
        for fault in [false, true] {
            let mut h = Harness::new();
            h.active();
            h.call("queued");
            h.input.end(fault);
            // Observe the pump directly in this fixture so the driver's FIRST
            // poll happens after terminal observation with queued input intact.
            let deadline = Instant::now() + Duration::from_secs(5);
            loop {
                if h.d().pump.observe().is_ok_and(|s| s.reader_io.is_some()) {
                    break;
                }
                assert!(Instant::now() < deadline);
                std::thread::sleep(Duration::from_millis(1));
            }
            let o = h.wait(|o| o.terminal.is_some());
            assert!(o.events.unwrap().events().is_empty());
            assert!(h.wait(|_| true).events.is_none());
        }
    }
    #[test]
    fn malformed_frame_fault_and_stop_are_separate_from_worker_completion() {
        for (frame, terminal) in [
            // Framing requires an RPC object before Value materialization.
            (json!([]), Terminal::Io),
            // A valid RPC object with an unsolicited response reaches Protocol.
            (json!({"id":999,"result":{}}), Terminal::Protocol),
        ] {
            let mut h = Harness::new();
            h.input.frame(frame);
            assert_eq!(h.wait(|o| o.terminal.is_some()).terminal, Some(terminal));
            h.finish();
            let o = h.wait(|_| true);
            assert_eq!(o.io.reader_completion, stdio_pump::Completion::Joined);
            assert_eq!(o.io.writer_completion, stdio_pump::Completion::Joined);
        }
        let mut h = Harness::new();
        h.d().stop();
        let o = h.wait(|_| true);
        assert_eq!(o.terminal, Some(Terminal::StopRequested));
        assert!(o.io.stop_requested);
        // Source remains blocked until Harness releases it; stop is not closure.
        assert!(h.registry.pool().reserved_total() >= SESSION_BYTES + stdio_pump::STARTUP_BYTES);
    }
    #[test]
    fn completion_after_observed_eof_reports_loss_without_queuing_a_reply() {
        let mut h = Harness::new();
        h.active();
        h.call("rpc");
        let r = reference(&h.event());
        h.input.end(false);
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            if h.d().pump.observe().is_ok_and(|s| s.reader_io.is_some()) {
                break;
            }
            assert!(Instant::now() < deadline);
            std::thread::sleep(Duration::from_millis(1));
        }
        assert_eq!(h.complete(&r, outcome()), Err(Refusal::Closed));
        let lost = h.event();
        assert!(
            matches!(lost.events(), [Event::ToolLost { request_ref, .. }] if request_ref.as_str() == r)
        );
        assert_eq!(h.output.state.lock().unwrap().1, 4);
        assert!(h.wait(|_| true).events.is_none());
    }
    #[test]
    fn catalog_sized_request_burst_reaches_the_host_without_queue_failure() {
        let mut h = Harness::new();
        h.active();
        let count = Catalog::load().unwrap().limits().pending_calls as usize;
        for id in 0..count {
            h.call(&format!("rpc-{id}"));
        }
        // Let the peer's admitted burst reach the transport before consuming.
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            if h.d()
                .pump
                .observe()
                .is_ok_and(|s| s.fault.is_some() || s.read_queued == count)
            {
                break;
            }
            assert!(Instant::now() < deadline);
            std::thread::sleep(Duration::from_millis(1));
        }
        assert!(h.d().pump.observe().unwrap().fault.is_none());
        for _ in 0..count {
            let batch = h.event();
            assert!(matches!(batch.events(), [Event::ToolRequested { .. }]));
        }
        assert_eq!(h.wait(|_| true).terminal, None);
    }
    #[test]
    fn stalled_writer_queue_fault_keeps_custody_until_release() {
        let mut h = Harness::new();
        h.active();
        h.output.state.lock().unwrap().2 = true;
        // Unknown requests generate fixed protocol error replies, never dispatch.
        let mut terminal = false;
        for id in 100..100 + stdio_pump::QUEUE_CAPACITY + 2 {
            h.input
                .frame(json!({"id":id,"method":"unsupported","params":{}}));
            let o = h.wait(|o| o.terminal.is_some() || o.events.is_some());
            if o.terminal.is_some() {
                terminal = true;
            }
            if terminal {
                break;
            }
        }
        assert!(terminal);
        assert!(h.registry.pool().reserved_total() > SESSION_BYTES + stdio_pump::STARTUP_BYTES);
        h.finish();
        let registry = h.registry.clone();
        drop(h);
        assert_eq!(registry.pool().reserved_total(), 0);
    }
    #[test]
    fn typed_partial_boundary_keeps_real_pump_custody_until_joined() {
        let mut h = Harness::new();
        let SessionDriver {
            protocol,
            basis,
            registry,
            pump,
            session,
            ..
        } = h.driver.take().unwrap();
        // Inject only the typed boundary result, using real owned Pump custody.
        // Actual second-spawn failure/gate behavior is Pump's existing test seam;
        // this test neither forces OS resource exhaustion nor claims a spawn fault.
        let partial = SessionDriver::finish_start(
            protocol,
            basis,
            registry,
            session,
            Err(stdio_pump::StartError::Partial {
                fault: stdio_pump::Fault::SecondSpawn,
                custody: pump,
            }),
        );
        h.driver = Some(match partial {
            Err(StartError::Partial { custody, .. }) => custody,
            _ => panic!("expected typed partial custody"),
        });
        assert_eq!(h.wait(|_| true).terminal, Some(Terminal::Io));
        assert!(h.registry.pool().reserved_total() >= SESSION_BYTES + stdio_pump::STARTUP_BYTES);
        h.finish();
        let o = h.wait(|_| true);
        assert_eq!(o.io.reader_completion, stdio_pump::Completion::Joined);
        assert_eq!(o.io.writer_completion, stdio_pump::Completion::Joined);
    }
    #[test]
    fn queued_reply_and_event_share_grown_frame_custody() {
        let mut h = Harness::new();
        assert_eq!(
            h.control(|d| d.initialize("test", "1")),
            Ok(Submission::Queued)
        );
        h.sent(1);
        h.output.state.lock().unwrap().2 = true;
        h.input.frame(json!({"id":1,"result":{"codexHome":"/test","platformFamily":"test","platformOs":"test","userAgent":"test"}}));
        let batch = h.event();
        assert!(matches!(batch.events(), [Event::Initialized]));
        assert!(batch._backing.bytes() > DERIVED_BYTES);
        // The reply is queued or in flight in a stalled writer; SAME frame lease.
        assert!(Arc::strong_count(&batch._backing) >= 2);
        h.finish();
        let registry = h.registry.clone();
        drop(h);
        assert!(registry.pool().reserved_total() >= SESSION_BYTES + DERIVED_BYTES);
        drop(batch);
        assert_eq!(registry.pool().reserved_total(), 0);
    }
    #[test]
    fn invalid_configuration_has_no_started_custody() {
        let registry = SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()));
        let result = SessionDriver::start(
            io::empty(),
            io::sink(),
            registry.clone(),
            "short",
            16,
            &basis(),
        );
        assert!(matches!(
            result,
            Err(StartError::Refused(Refusal::Configuration))
        ));
        let result = SessionDriver::start(
            io::empty(),
            io::sink(),
            registry.clone(),
            "0123456789abcdef",
            16,
            &json!({}),
        );
        assert!(matches!(
            result,
            Err(StartError::Refused(Refusal::Configuration))
        ));
        assert_eq!(registry.pool().reserved_total(), 0);
        assert!(!bounded_value(
            &Value::String("x".repeat(BASIS_BYTES + 1)),
            BASIS_BYTES
        ));
    }
    #[test]
    fn constructor_catalog_heap_fits_the_admitted_session_allowance() {
        let registry = SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()));
        let input = Arc::new(Input::default());
        let output = Arc::new(Output::default());
        let current_basis = basis();
        let (started, measured) = crate::ingress::counting::measure(|| {
            SessionDriver::start(
                Source(input.clone()),
                Sink(output.clone()),
                registry.clone(),
                "0123456789abcdef",
                16,
                &current_basis,
            )
        });
        let mut h = Harness {
            driver: Some(started.unwrap()),
            input,
            output,
            registry,
        };
        let allowance = h.registry.pool().reserved_total() - (2 * stdio_pump::STACK_BYTES) as u64;
        assert!(
            measured.peak_priced <= allowance,
            "{} > {allowance}",
            measured.peak_priced
        );
        h.finish();
        let registry = h.registry.clone();
        drop(h);
        assert_eq!(registry.pool().reserved_total(), 0);
    }
}
