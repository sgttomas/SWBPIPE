//! Explicitly started, portable stdio pump; no protocol, session or process authority.
//!
//! The coordinator must stop/close its owned process/streams before expecting
//! completion: generic safe-std Read/Write cannot interrupt blocked I/O. EOF,
//! terminal I/O, stop requests and joined threads are separate facts. None proves
//! process/descendant exit, parent-crash cleanup or absence of escaped descendants.
//! No operation is retried after an unknown application outcome. Queue endpoints
//! are not independently cloneable/disconnectable: Pump is the sole consumer and
//! producer owner. Abandonment stops the shared queues and retains custody.

use super::{catalog::Limits, framing};
use crate::memory_pool::Lease;
use crate::solve_jobs::SolveJobRegistry;
use serde_json::Value;
use std::io::{self, Read, Write};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, TryLockError};
use std::thread::{self, JoinHandle};

// Catalog v2 permits 16 concurrent calls; leave room for their control/status
// notifications without requiring the coordinator to run between each frame.
pub const QUEUE_CAPACITY: usize = 32;
pub const STACK_BYTES: usize = 512 * 1024;
// Both requested stacks, plus a deliberately separate 128 KiB allowance for
// thread/Arc/Mutex/Condvar control allocations, queue slots, closure captures,
// serializer scratch and allocator overhead. No serialized frame copy is made.
// Reader buffers and each Value's backing are separately admitted. This is a
// portable reservation rule, not an RSS/OS-stack witness or retained-route price.
pub const STARTUP_BYTES: u64 = (2 * STACK_BYTES + 128 * 1024) as u64;

/// Static diagnostics only: never store io::Error, serde errors or input text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    Admission,
    ReaderSetup,
    StreamLayout,
    FirstSpawn,
    SecondSpawn,
    Read,
    Write,
    SerializeOrCap,
    ReadQueueFull,
    WriteQueueFull,
    Closed,
    Busy,
    Abandoned,
    WorkerPanic,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IoEnd {
    Eof,
    StopRequested,
    StartAborted,
    Failed(Fault),
}

/// I/O observations may precede stream destruction and thread completion.
/// Only `Joined`/`Panicked` confirm worker completion, never process completion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Status {
    pub stop_requested: bool,
    pub fault: Option<Fault>,
    pub reader_io: Option<IoEnd>,
    pub writer_io: Option<IoEnd>,
    pub reader_completion: Completion,
    pub writer_completion: Completion,
    pub read_queued: usize,
    pub write_queued: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Completion {
    NotSpawned,
    Unconfirmed,
    Joined,
    Panicked,
}

// Declaration order matters: destroy the Value before its backing reference.
// The caller must retain enough admitted backing for ALL derived protocol
// state/events/Values, including while another frame shares this Arc. A lease
// reference is custody, not a fresh allocation budget for each clone.
struct Outbound {
    value: Value,
    _backing: Arc<Lease>,
}

struct Queue<T> {
    slots: [Option<T>; QUEUE_CAPACITY],
    head: usize,
    len: usize,
}
impl<T> Queue<T> {
    fn new() -> Self {
        Self {
            slots: std::array::from_fn(|_| None),
            head: 0,
            len: 0,
        }
    }
    fn push(&mut self, item: T) -> Result<(), T> {
        if self.len == QUEUE_CAPACITY {
            return Err(item);
        }
        self.slots[(self.head + self.len) % QUEUE_CAPACITY] = Some(item);
        self.len += 1;
        Ok(())
    }
    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let item = self.slots[self.head].take();
        self.head = (self.head + 1) % QUEUE_CAPACITY;
        self.len -= 1;
        item
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Gate {
    Waiting,
    Run,
    Abort,
}
struct State {
    gate: Gate,
    stop: bool,
    fault: Option<Fault>,
    reader_io: Option<IoEnd>,
    writer_io: Option<IoEnd>,
    reads: Queue<framing::AdmittedFrame>,
    writes: Queue<Outbound>,
}
struct Shared {
    state: Mutex<State>,
    wake: Condvar,
}
impl Shared {
    fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(|e| e.into_inner())
    }
    fn try_lock(&self) -> Result<MutexGuard<'_, State>, Fault> {
        match self.state.try_lock() {
            Ok(state) => Ok(state),
            Err(TryLockError::Poisoned(e)) => Ok(e.into_inner()),
            Err(TryLockError::WouldBlock) => Err(Fault::Busy),
        }
    }
    fn wait<'a>(&self, state: MutexGuard<'a, State>) -> MutexGuard<'a, State> {
        self.wake.wait(state).unwrap_or_else(|e| e.into_inner())
    }
    fn start(&self) -> bool {
        let mut state = self.lock();
        while state.gate == Gate::Waiting {
            state = self.wait(state);
        }
        state.gate == Gate::Run
    }
    fn fail(&self, state: &mut State, fault: Fault) {
        state.fault.get_or_insert(fault);
        state.stop = true;
        self.wake.notify_all();
    }
}

// The reservation is LAST: shared state/queued frames/handles are destroyed
// first. It is not released merely because terminal I/O was reported.
struct Custody {
    shared: Arc<Shared>,
    reader: Option<JoinHandle<()>>,
    writer: Option<JoinHandle<()>>,
    reader_completion: Completion,
    writer_completion: Completion,
    _startup: Lease,
}
pub struct Pump {
    custody: Option<Custody>,
}

/// Partial is explicitly NOT a started pump or confirmed rollback. Its gate is
/// aborted; keep observing/joining it, or Drop deliberately retains accounting.
pub enum StartError {
    Refused(Fault),
    Partial { fault: Fault, custody: Pump },
}
impl std::fmt::Debug for StartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Refused(fault) => f.debug_tuple("Refused").field(fault).finish(),
            Self::Partial { fault, .. } => f.debug_tuple("PartialCustody").field(fault).finish(),
        }
    }
}

impl Pump {
    /// Admit before queue/state allocation or thread spawn. No I/O until both
    /// spawns succeed. There are no waits for worker exit in this constructor.
    /// OS spawn/admission/short bookkeeping locks have no wall-clock guarantee.
    /// Caller-provided streams must have nonblocking destructors, including on
    /// admission/spawn failure; arbitrary user Drop cannot be bounded by std.
    pub fn start<R: Read + Send + 'static, W: Write + Send + 'static>(
        source: R,
        sink: W,
        registry: SolveJobRegistry,
        limits: Limits,
    ) -> Result<Self, StartError> {
        Self::start_inner(source, sink, registry, limits, false)
    }

    fn start_inner<R: Read + Send + 'static, W: Write + Send + 'static>(
        source: R,
        sink: W,
        registry: SolveJobRegistry,
        limits: Limits,
        fail_second_spawn: bool,
    ) -> Result<Self, StartError> {
        // Bound inline closure captures too. Existing stream heap storage must
        // already be backed by its owner; this pump does not copy that storage.
        if std::mem::size_of::<R>().saturating_add(std::mem::size_of::<W>()) > 8192 {
            return Err(StartError::Refused(Fault::StreamLayout));
        }
        let startup = registry
            .acquire_interactive_ingress(STARTUP_BYTES)
            .map_err(|_| StartError::Refused(Fault::Admission))?;
        let reader = framing::Reader::new(source, registry, limits)
            .map_err(|_| StartError::Refused(Fault::ReaderSetup))?;
        let shared = Arc::new(Shared {
            state: Mutex::new(State {
                gate: Gate::Waiting,
                stop: false,
                fault: None,
                reader_io: None,
                writer_io: None,
                reads: Queue::new(),
                writes: Queue::new(),
            }),
            wake: Condvar::new(),
        });
        let read_shared = Arc::clone(&shared);
        let read_handle = thread::Builder::new()
            .stack_size(STACK_BYTES)
            .spawn(move || read_worker(reader, read_shared))
            .map_err(|_| StartError::Refused(Fault::FirstSpawn))?;
        let mut pump = Self {
            custody: Some(Custody {
                shared: Arc::clone(&shared),
                reader: Some(read_handle),
                writer: None,
                reader_completion: Completion::Unconfirmed,
                writer_completion: Completion::NotSpawned,
                _startup: startup,
            }),
        };
        // Deterministic private test seam, never resource-exhaustion testing.
        let write_shared = Arc::clone(&shared);
        let writer = if fail_second_spawn {
            Err(io::Error::from(io::ErrorKind::Other))
        } else {
            thread::Builder::new()
                .stack_size(STACK_BYTES)
                .spawn(move || write_worker(sink, write_shared))
        };
        match writer {
            Ok(handle) => {
                let custody = pump.custody.as_mut().unwrap();
                custody.writer = Some(handle);
                custody.writer_completion = Completion::Unconfirmed;
                shared.lock().gate = Gate::Run;
                shared.wake.notify_all();
                Ok(pump)
            }
            Err(_) => {
                let mut state = shared.lock();
                state.gate = Gate::Abort;
                shared.fail(&mut state, Fault::SecondSpawn);
                drop(state);
                Err(StartError::Partial {
                    fault: Fault::SecondSpawn,
                    custody: pump,
                })
            }
        }
    }

    /// No stream/condition-variable wait; joins only workers std reports finished.
    /// This is not a hard deadline guarantee for OS/TLS cleanup or bookkeeping.
    /// Busy means a short queue bookkeeping lock is held; it is not an exit.
    pub fn observe(&mut self) -> Result<Status, Fault> {
        let custody = self.custody.as_mut().unwrap();
        reap(custody);
        let state = custody.shared.try_lock()?;
        Ok(Status {
            stop_requested: state.stop,
            fault: state.fault,
            reader_io: state.reader_io,
            writer_io: state.writer_io,
            reader_completion: custody.reader_completion,
            writer_completion: custody.writer_completion,
            read_queued: state.reads.len,
            write_queued: state.writes.len,
        })
    }

    /// Drain one frame at a time, even after a fault. Keep its lease through
    /// interpretation and through all derived live values. Empty is not EOF.
    pub fn try_receive(&self) -> Result<Option<framing::AdmittedFrame>, Fault> {
        Ok(self
            .custody
            .as_ref()
            .unwrap()
            .shared
            .try_lock()?
            .reads
            .pop())
    }

    /// Moves one already-admitted Value; no cloning and no serialization buffer.
    /// Cap includes newline. Backing adequacy is the coordinator's obligation.
    /// Success means queued, not delivered or applied. A refusal consumes/drops
    /// the unsent value; never retry an unknown outcome.
    pub fn try_send(&self, value: Value, backing: Arc<Lease>) -> Result<(), Fault> {
        let frame = Outbound {
            value,
            _backing: backing,
        };
        let shared = &self.custody.as_ref().unwrap().shared;
        if serde_json::to_writer(&mut ByteCap(0), &frame.value).is_err() {
            let mut state = shared.lock();
            shared.fail(&mut state, Fault::SerializeOrCap);
            return Err(Fault::SerializeOrCap);
        }
        let mut state = shared.lock();
        if state.stop || state.writer_io.is_some() {
            return Err(Fault::Closed);
        }
        if state.writes.push(frame).is_err() {
            shared.fail(&mut state, Fault::WriteQueueFull);
            return Err(Fault::WriteQueueFull);
        }
        shared.wake.notify_all();
        Ok(())
    }

    /// Request only; does not close or interrupt a blocked Read/Write. Pending
    /// queues remain owned for bounded drain/drop; they are never resubmitted.
    pub fn request_stop(&self) {
        let shared = &self.custody.as_ref().unwrap().shared;
        shared.lock().stop = true;
        shared.wake.notify_all();
    }
}

fn reap(custody: &mut Custody) {
    for (handle, completion) in [
        (&mut custody.reader, &mut custody.reader_completion),
        (&mut custody.writer, &mut custody.writer_completion),
    ] {
        if handle.as_ref().is_some_and(JoinHandle::is_finished) {
            *completion = Completion::Joined;
            if handle.take().unwrap().join().is_err() {
                *completion = Completion::Panicked;
                let mut state = custody.shared.lock();
                custody.shared.fail(&mut state, Fault::WorkerPanic);
            }
        }
    }
}

impl Drop for Pump {
    fn drop(&mut self) {
        let Some(mut custody) = self.custody.take() else {
            return;
        };
        {
            let mut state = custody.shared.lock();
            if state.gate == Gate::Waiting {
                state.gate = Gate::Abort;
            }
            state.stop = true;
            custody.shared.wake.notify_all();
        }
        reap(&mut custody);
        if custody.reader.is_some() || custody.writer.is_some() {
            let mut state = custody.shared.lock();
            custody.shared.fail(&mut state, Fault::Abandoned);
            drop(state);
            // Deliberate fail-closed retention: no reaper/global initializer,
            // detached-success claim, or release of live stack accounting. This
            // leaks custody AND its reservation even if workers later finish.
            // The coordinator must retain Pump and observe completion instead.
            std::mem::forget(custody);
        }
    }
}

struct ByteCap(usize);
impl Write for ByteCap {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0 = self
            .0
            .checked_add(bytes.len())
            .filter(|n| *n < framing::MAX_FRAME_BYTES)
            .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?;
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn read_worker<R: Read>(mut reader: framing::Reader<R>, shared: Arc<Shared>) {
    let end = if !shared.start() {
        IoEnd::StartAborted
    } else {
        loop {
            if shared.lock().stop {
                break IoEnd::StopRequested;
            }
            match reader.next() {
                Ok(Some(frame)) => {
                    let mut state = shared.lock();
                    if state.stop {
                        break IoEnd::StopRequested;
                    }
                    if state.reads.push(frame).is_err() {
                        shared.fail(&mut state, Fault::ReadQueueFull);
                        break IoEnd::Failed(Fault::ReadQueueFull);
                    }
                }
                Ok(None) => break IoEnd::Eof,
                Err(_) => {
                    let mut state = shared.lock();
                    shared.fail(&mut state, Fault::Read);
                    break IoEnd::Failed(Fault::Read);
                }
            }
        }
    };
    shared.lock().reader_io = Some(end);
    shared.wake.notify_all();
    // reader (and its source) may still be in Drop after this observation.
}

fn write_worker<W: Write>(mut sink: W, shared: Arc<Shared>) {
    let end = if !shared.start() {
        IoEnd::StartAborted
    } else {
        loop {
            let frame = {
                let mut state = shared.lock();
                while !state.stop && state.writes.len == 0 {
                    state = shared.wait(state);
                }
                if state.stop {
                    break IoEnd::StopRequested;
                }
                state.writes.pop().unwrap()
            };
            // Keep frame's backing through serialization, newline AND flush.
            if serde_json::to_writer(&mut sink, &frame.value).is_err()
                || sink.write_all(b"\n").is_err()
                || sink.flush().is_err()
            {
                let mut state = shared.lock();
                shared.fail(&mut state, Fault::Write);
                break IoEnd::Failed(Fault::Write);
            }
        }
    };
    shared.lock().writer_io = Some(end);
    shared.wake.notify_all();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_pool::{test_host::ScriptedHost, MemoryPool};
    use std::io::Cursor;
    use std::time::{Duration, Instant};

    fn registry() -> SolveJobRegistry {
        SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()))
    }
    fn limits() -> Limits {
        super::super::catalog::Catalog::load().unwrap().limits()
    }
    fn backing(registry: &SolveJobRegistry) -> Arc<Lease> {
        Arc::new(
            registry
                .acquire_interactive_ingress(4 * framing::MAX_FRAME_BYTES as u64)
                .unwrap(),
        )
    }

    // Every stalled fixture has an unconditional release path in Harness::drop,
    // even on assertion failure. No test relies on an orphaned blocking thread.
    #[derive(Default)]
    struct Stall {
        flags: Mutex<(bool, bool)>, // entered, released
        wake: Condvar,
    }
    impl Stall {
        fn block(&self) {
            let mut flags = self.flags.lock().unwrap_or_else(|e| e.into_inner());
            flags.0 = true;
            self.wake.notify_all();
            while !flags.1 {
                flags = self.wake.wait(flags).unwrap_or_else(|e| e.into_inner());
            }
        }
        fn release(&self) {
            self.flags.lock().unwrap_or_else(|e| e.into_inner()).1 = true;
            self.wake.notify_all();
        }
        fn entered(&self) {
            let flags = self.flags.lock().unwrap_or_else(|e| e.into_inner());
            let (flags, _) = self
                .wake
                .wait_timeout_while(flags, Duration::from_secs(5), |f| !f.0)
                .unwrap();
            let entered = flags.0;
            drop(flags);
            assert!(entered, "fixture did not enter controlled stall");
        }
    }
    struct BlockRead(Arc<Stall>);
    impl Read for BlockRead {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            self.0.block();
            Ok(0)
        }
    }
    struct Release(Arc<Stall>);
    impl Drop for Release {
        fn drop(&mut self) {
            self.0.release();
        }
    }
    struct BlockDrop(Arc<Stall>, thread::ThreadId);
    impl BlockDrop {
        fn new(stall: Arc<Stall>) -> Self {
            Self(stall, thread::current().id())
        }
    }
    impl Read for BlockDrop {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Ok(0)
        }
    }
    impl Drop for BlockDrop {
        fn drop(&mut self) {
            // Never stall destruction on the constructor's error path.
            if thread::current().id() != self.1 {
                self.0.block();
            }
        }
    }
    struct BlockWrite(Arc<Stall>);
    impl Write for BlockWrite {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.0.block();
            Ok(bytes.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    struct BadRead;
    impl Read for BadRead {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::from(io::ErrorKind::Other))
        }
    }
    struct BadWrite;
    impl Write for BadWrite {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Err(io::Error::from(io::ErrorKind::Other))
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[derive(Clone, Default)]
    struct Capture(
        Arc<Mutex<(Vec<u8>, usize)>>,
        Option<std::sync::mpsc::SyncSender<()>>,
    );
    impl Write for Capture {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.0.lock().unwrap().0.extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            self.0.lock().unwrap().1 += 1;
            if let Some(done) = &self.1 {
                let _ = done.try_send(());
            }
            Ok(())
        }
    }

    struct Harness {
        pump: Pump,
        release: Option<Arc<Stall>>,
    }
    impl Harness {
        fn new(pump: Pump, release: Option<Arc<Stall>>) -> Self {
            Self { pump, release }
        }
        fn wait(&self, predicate: impl Fn(&State) -> bool) {
            let shared = &self.pump.custody.as_ref().unwrap().shared;
            let deadline = Instant::now() + Duration::from_secs(5);
            let mut state = shared.lock();
            while !predicate(&state) {
                let remaining = deadline.saturating_duration_since(Instant::now());
                assert!(!remaining.is_zero(), "worker observation not reached");
                state = shared.wake.wait_timeout(state, remaining).unwrap().0;
            }
        }
        fn status(&self) -> Status {
            let c = self.pump.custody.as_ref().unwrap();
            let s = c.shared.lock();
            Status {
                stop_requested: s.stop,
                fault: s.fault,
                reader_io: s.reader_io,
                writer_io: s.writer_io,
                reader_completion: c.reader_completion,
                writer_completion: c.writer_completion,
                read_queued: s.reads.len,
                write_queued: s.writes.len,
            }
        }
        fn finish(&mut self) {
            if let Some(release) = &self.release {
                release.release();
            }
            let custody = self.pump.custody.as_mut().unwrap();
            custody.shared.lock().stop = true;
            custody.shared.wake.notify_all();
            // Test fixtures are finite/released. Production never uses blocking join.
            for (handle, completion) in [
                (&mut custody.reader, &mut custody.reader_completion),
                (&mut custody.writer, &mut custody.writer_completion),
            ] {
                if let Some(handle) = handle.take() {
                    *completion = if handle.join().is_ok() {
                        Completion::Joined
                    } else {
                        Completion::Panicked
                    };
                }
            }
        }
    }
    impl Drop for Harness {
        fn drop(&mut self) {
            self.finish();
        }
    }

    #[test]
    fn frame_delivery_keeps_lease_after_workers_and_pump_are_gone() {
        let registry = registry();
        let pump = Pump::start(
            Cursor::new(b"{\"id\":1}\n"),
            io::sink(),
            registry.clone(),
            limits(),
        )
        .unwrap();
        let mut harness = Harness::new(pump, None);
        harness.wait(|s| s.reader_io == Some(IoEnd::Eof));
        let frame = harness.pump.try_receive().unwrap().unwrap();
        assert_eq!(frame.value["id"], 1);
        harness.finish();
        let bytes = frame.lease.bytes();
        assert_eq!(registry.pool().reserved_total(), STARTUP_BYTES + bytes);
        drop(harness);
        assert_eq!(registry.pool().reserved_total(), bytes);
        drop(frame);
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    #[test]
    fn eof_is_not_joined_completion_or_process_evidence() {
        let registry = registry();
        let stall = Arc::new(Stall::default());
        let _release_on_failure = Release(stall.clone());
        let pump = Pump::start(
            BlockDrop::new(stall.clone()),
            io::sink(),
            registry.clone(),
            limits(),
        )
        .unwrap();
        let mut harness = Harness::new(pump, Some(stall.clone()));
        stall.entered();
        let status = harness.status();
        assert_eq!(status.reader_io, Some(IoEnd::Eof));
        assert_eq!(status.reader_completion, Completion::Unconfirmed);
        assert!(registry.pool().reserved_total() >= STARTUP_BYTES);
        harness.finish();
        assert_eq!(
            harness.pump.observe().unwrap().reader_completion,
            Completion::Joined
        );
        drop(harness);
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    #[test]
    fn read_queue_exhaustion_is_terminal_and_retains_queued_frames() {
        let registry = registry();
        let raw = "{}\n".repeat(QUEUE_CAPACITY + 1);
        let pump = Pump::start(Cursor::new(raw), io::sink(), registry.clone(), limits()).unwrap();
        let mut harness = Harness::new(pump, None);
        harness.wait(|s| s.reader_io.is_some());
        harness.finish();
        let status = harness.status();
        assert_eq!(status.fault, Some(Fault::ReadQueueFull));
        assert_eq!(status.read_queued, QUEUE_CAPACITY);
        for _ in 0..QUEUE_CAPACITY {
            drop(harness.pump.try_receive().unwrap().unwrap());
        }
        assert!(harness.pump.try_receive().unwrap().is_none());
        drop(harness);
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    #[test]
    fn constructor_heap_fits_control_and_reader_reservations() {
        let registry = registry();
        let stall = Arc::new(Stall::default());
        let _release_on_failure = Release(stall.clone());
        // The caller constructs Limits; catalog loading is not Pump's heap.
        let caps = limits();
        let (started, measured) = crate::ingress::counting::measure(|| {
            Pump::start(BlockRead(stall.clone()), io::sink(), registry.clone(), caps)
        });
        let pump = started.unwrap();
        let mut harness = Harness::new(pump, Some(stall.clone()));
        stall.entered();
        let heap_allowance = registry.pool().reserved_total() - (2 * STACK_BYTES) as u64;
        assert!(
            measured.peak_priced <= heap_allowance,
            "{} > {heap_allowance}",
            measured.peak_priced
        );
        harness.finish();
        drop(harness);
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    #[test]
    fn writer_streams_one_frame_with_newline_and_flush_then_releases_backing() {
        let registry = registry();
        let (done, flushed) = std::sync::mpsc::sync_channel(1);
        let capture = Capture(Default::default(), Some(done));
        let pump = Pump::start(io::empty(), capture.clone(), registry.clone(), limits()).unwrap();
        let mut harness = Harness::new(pump, None);
        let lease = backing(&registry);
        harness
            .pump
            .try_send(Value::Bool(true), lease.clone())
            .unwrap();
        flushed.recv_timeout(Duration::from_secs(5)).unwrap();
        harness.finish();
        let output = capture.0.lock().unwrap();
        assert_eq!(output.0, b"true\n");
        assert_eq!(output.1, 1);
        assert_eq!(Arc::strong_count(&lease), 1);
        drop(output);
        drop(harness);
        drop(lease);
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    #[test]
    fn outbound_cap_is_checked_before_enqueue_or_write() {
        let registry = registry();
        let capture = Capture::default();
        let pump = Pump::start(io::empty(), capture.clone(), registry.clone(), limits()).unwrap();
        let mut harness = Harness::new(pump, None);
        let lease = backing(&registry);
        let value = Value::String("x".repeat(framing::MAX_FRAME_BYTES));
        assert_eq!(
            harness.pump.try_send(value, lease),
            Err(Fault::SerializeOrCap)
        );
        harness.finish();
        assert_eq!(harness.pump.observe().unwrap().write_queued, 0);
        assert!(capture.0.lock().unwrap().0.is_empty());
        drop(harness);
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    #[test]
    fn stalled_writer_retains_inflight_and_full_queue_backing_until_release() {
        let registry = registry();
        let stall = Arc::new(Stall::default());
        let _release_on_failure = Release(stall.clone());
        let pump = Pump::start(
            io::empty(),
            BlockWrite(stall.clone()),
            registry.clone(),
            limits(),
        )
        .unwrap();
        let mut harness = Harness::new(pump, Some(stall.clone()));
        let lease = backing(&registry);
        harness.pump.try_send(Value::Null, lease.clone()).unwrap();
        stall.entered();
        for _ in 0..QUEUE_CAPACITY {
            harness.pump.try_send(Value::Null, lease.clone()).unwrap();
        }
        assert_eq!(Arc::strong_count(&lease), QUEUE_CAPACITY + 2);
        assert_eq!(
            harness.pump.try_send(Value::Null, lease.clone()),
            Err(Fault::WriteQueueFull)
        );
        let status = harness.status();
        assert_eq!(status.writer_io, None);
        assert_eq!(status.writer_completion, Completion::Unconfirmed);
        assert!(status.stop_requested);
        assert!(registry.pool().reserved_total() >= STARTUP_BYTES + lease.bytes());
        harness.finish();
        assert_eq!(Arc::strong_count(&lease), QUEUE_CAPACITY + 1);
        drop(harness);
        assert_eq!(Arc::strong_count(&lease), 1);
        drop(lease);
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    #[test]
    fn stop_request_does_not_complete_stalled_read_or_release_reservation() {
        let registry = registry();
        let stall = Arc::new(Stall::default());
        let _release_on_failure = Release(stall.clone());
        let pump = Pump::start(
            BlockRead(stall.clone()),
            io::sink(),
            registry.clone(),
            limits(),
        )
        .unwrap();
        let mut harness = Harness::new(pump, Some(stall.clone()));
        stall.entered();
        harness.pump.request_stop();
        let status = harness.status();
        assert!(status.stop_requested);
        assert_eq!(status.reader_io, None);
        assert_eq!(status.reader_completion, Completion::Unconfirmed);
        assert!(registry.pool().reserved_total() > STARTUP_BYTES);
        harness.finish();
        drop(harness);
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    #[test]
    fn second_spawn_failure_returns_unconfirmed_custody_and_gate_prevents_io() {
        let registry = registry();
        let stall = Arc::new(Stall::default());
        let _release_on_failure = Release(stall.clone());
        let capture = Capture::default();
        let result = Pump::start_inner(
            BlockDrop::new(stall.clone()),
            capture.clone(),
            registry.clone(),
            limits(),
            true,
        );
        // Construct guard before any assertion involving the stalled worker.
        let pump = match result {
            Err(StartError::Partial { custody, .. }) => custody,
            _ => {
                stall.release();
                panic!("expected partial custody");
            }
        };
        let mut harness = Harness::new(pump, Some(stall.clone()));
        stall.entered();
        let status = harness.status();
        assert_eq!(status.reader_io, Some(IoEnd::StartAborted));
        assert_eq!(status.reader_completion, Completion::Unconfirmed);
        assert_eq!(status.writer_completion, Completion::NotSpawned);
        assert_eq!(status.fault, Some(Fault::SecondSpawn));
        assert!(capture.0.lock().unwrap().0.is_empty());
        assert!(registry.pool().reserved_total() >= STARTUP_BYTES);
        harness.finish();
        drop(harness);
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    #[test]
    fn read_and_write_errors_are_static_terminal_faults() {
        let registry = registry();
        let pump = Pump::start(BadRead, io::sink(), registry.clone(), limits()).unwrap();
        let mut harness = Harness::new(pump, None);
        harness.wait(|s| s.reader_io.is_some());
        harness.finish();
        assert_eq!(harness.pump.observe().unwrap().fault, Some(Fault::Read));
        drop(harness);
        let pump = Pump::start(io::empty(), BadWrite, registry.clone(), limits()).unwrap();
        let mut harness = Harness::new(pump, None);
        harness
            .pump
            .try_send(Value::Null, backing(&registry))
            .unwrap();
        harness.wait(|s| s.writer_io.is_some());
        harness.finish();
        assert_eq!(harness.pump.observe().unwrap().fault, Some(Fault::Write));
        drop(harness);
        assert_eq!(registry.pool().reserved_total(), 0);
    }
}
