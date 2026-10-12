//! The capped ordinary-solve worker (owner decision 2026-10-12, "(c) Capped
//! worker process"; DEL-04-07 memory-budget decision).
//!
//! Each ordinary solve runs in a short-lived local child process: this app's
//! own executable started with [`WORKER_FLAG`], with a minimal environment,
//! stdin/stdout as its framed control channel and stderr discarded. No
//! daemon, network, telemetry or persistent service is involved; the child
//! ends with its job.
//!
//! Protocol (all integers little-endian):
//! - Parent -> child, once: the request header `SWRQ`, then u64 mode
//!   (0 sparse_interactive, 1 dense_scrutiny), u64 initial grant, u64 maximum
//!   grant (the per-invocation budget), u64 process reserve, u64 raw length L,
//!   then L raw model bytes (UTF-8 JSON). The child reads all of it before its
//!   cap is armed, so no growth exchange can interleave with the request.
//! - Child -> parent: 20-byte frames ([`crate::worker_alloc::FRAME_BYTES`]):
//!   `GROW {requested_grant, phys_footprint}` (answered `GRNT {grant}` or
//!   `DENY`), `CAPX {needed_grant, max_grant}` or `RFSD {requested, grant}`
//!   (a final refusal, then exit [`WORKER_CAP_EXIT`]), `RSLT {R,
//!   lifetime_max_footprint}` + R bytes (the producer's exact bytes),
//!   `FAIL {n}` + n bytes (n <= 64 KiB).
//!
//! The grant (a [`LeasePurpose::Worker`] lease) starts at the process reserve
//! plus the request's ingress bound, rounded up to 1 MiB, and grows on demand
//! by atomic pool admissions with a fresh headroom verdict, never above the
//! budget; the child's allocator is capped at grant - reserve
//! ([`crate::worker_alloc`]). The measured physical footprint (the larger of
//! the child's report and the parent's own reading) is a growth need as well:
//! a growth step asks for at least footprint + reserve, so memory the count
//! does not see (allocator-retained freed blocks, metadata, stacks) is
//! reserved too. Only a footprint whose reservation would exceed the budget
//! refuses the job (`worker_footprint_exceeded`); a pool or headroom refusal
//! of the step is `worker_growth_refused`.
//!
//! Residual: the footprint is measured at growth exchanges and at the result,
//! not continuously, so between exchanges it can briefly exceed the grant (by
//! memory the count does not see). At the result frame the parent reads the
//! worker's lifetime-maximum footprint (which covers those moments), records
//! it as the job's `worker_peak_footprint_bytes` and refuses the result if it
//! exceeded the budget.
//!
//! A result of R bytes must not exceed the grant; its result lease is
//! acquired while the grant is still held, the bytes are read into an
//! exact-capacity buffer, the child must exit 0, and only after the child is
//! reaped is the grant released. A kill (cancellation, session reset, refusal
//! or protocol violation) targets the child only while it is unreaped.

use crate::memory_pool::{Lease, LeasePurpose, MemoryPool, ResourceRefusal};
use crate::worker_alloc::{
    decode_frame, encode_frame, resident_envelope, FRAME_BYTES, TAG_CAP_EXCEEDED, TAG_DENIED, TAG_ERROR, TAG_GRANTED,
    TAG_GROW, TAG_REFUSED, TAG_RESULT,
};
use open_pipe_stress_product_physics::PreviewSolverMode;
use std::io::{Read, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, ExitStatus, Stdio};
use std::sync::{Arc, Mutex, PoisonError};

pub(crate) use crate::worker_alloc::{WORKER_CAP_EXIT, WORKER_PROTOCOL_EXIT};

/// The argument that starts this executable as a solve worker (checked by
/// `main` before any Tauri or AppKit initialization).
pub const WORKER_FLAG: &str = "--swbpipe-solve-worker";

/// The worker's non-heap footprint allowance: binary image dirty pages, dyld,
/// thread stacks, allocator metadata and freed blocks the allocator keeps.
/// Measured by `process_reserve_covers_a_real_workers_measured_footprint`
/// (idle and post-solve `phys_footprint` of this crate's binary in worker
/// mode) and enforced at every growth exchange.
pub(crate) const PROCESS_RESERVE_BYTES: u64 = 64 << 20;
/// Grants are whole MiB.
pub(crate) const GRANT_GRANULE_BYTES: u64 = 1 << 20;
/// The longest error frame the parent reads; longer messages are truncated by the child.
pub(crate) const MAX_ERROR_FRAME_BYTES: u64 = 64 * 1024;

const REQUEST_TAG: [u8; 4] = *b"SWRQ";
const REQUEST_BYTES: usize = 4 + 5 * 8;

/// The worker's starting grant for an ingress lease bound (the raw body's
/// envelope plus its materialization bound): reserve + bound, rounded up to
/// whole MiB. `None` on overflow.
pub(crate) fn initial_grant(reserve: u64, ingress_bound: u64) -> Option<u64> {
    reserve.checked_add(ingress_bound)?.checked_next_multiple_of(GRANT_GRANULE_BYTES)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Request {
    mode: PreviewSolverMode,
    initial_grant: u64,
    max_grant: u64,
    reserve: u64,
    raw_len: u64,
}

fn mode_code(mode: PreviewSolverMode) -> u64 {
    match mode {
        PreviewSolverMode::SparseInteractive => 0,
        PreviewSolverMode::DenseScrutiny => 1,
    }
}

fn encode_request(request: &Request) -> [u8; REQUEST_BYTES] {
    let mut bytes = [0u8; REQUEST_BYTES];
    bytes[..4].copy_from_slice(&REQUEST_TAG);
    let fields = [mode_code(request.mode), request.initial_grant, request.max_grant, request.reserve, request.raw_len];
    for (index, field) in fields.iter().enumerate() {
        bytes[4 + index * 8..12 + index * 8].copy_from_slice(&field.to_le_bytes());
    }
    bytes
}

fn decode_request(bytes: &[u8; REQUEST_BYTES]) -> Option<Request> {
    if bytes[..4] != REQUEST_TAG {
        return None;
    }
    let field = |index: usize| u64::from_le_bytes(bytes[4 + index * 8..12 + index * 8].try_into().unwrap_or([0; 8]));
    let mode = match field(0) {
        0 => PreviewSolverMode::SparseInteractive,
        1 => PreviewSolverMode::DenseScrutiny,
        _ => return None,
    };
    let request = Request { mode, initial_grant: field(1), max_grant: field(2), reserve: field(3), raw_len: field(4) };
    (request.reserve <= request.initial_grant && request.initial_grant <= request.max_grant).then_some(request)
}

// ---------------------------------------------------------------------------
// Child side
// ---------------------------------------------------------------------------

/// The worker process's entry: reads one request from the control channel,
/// solves it under the capped allocator and writes one terminal frame.
/// Returns the process exit code. A cap refusal never returns: it exits with
/// [`WORKER_CAP_EXIT`] from inside the allocator.
pub fn run_solve_worker() -> i32 {
    #[cfg(unix)]
    {
        let (in_fd, out_fd) = channel_fds();
        let Some((control_in, control_out)) = take_channel(in_fd, out_fd) else {
            return WORKER_PROTOCOL_EXIT;
        };
        crate::worker_alloc::enable(control_in, control_out);
        let mut header = [0u8; REQUEST_BYTES];
        if !crate::worker_alloc::read_request(&mut header) {
            return WORKER_PROTOCOL_EXIT;
        }
        let Some(request) = decode_request(&header) else {
            return WORKER_PROTOCOL_EXIT;
        };
        // A request whose own buffer cannot fit the budget is refused before it is read.
        let needed = resident_envelope(request.raw_len).saturating_add(request.reserve);
        if needed > request.max_grant {
            crate::worker_alloc::refuse_need_above_budget(needed, request.max_grant);
        }
        let Ok(raw_len) = usize::try_from(request.raw_len) else {
            return WORKER_PROTOCOL_EXIT;
        };
        let mut raw = vec![0u8; raw_len];
        if !crate::worker_alloc::read_request(&mut raw) {
            return WORKER_PROTOCOL_EXIT;
        }
        crate::worker_alloc::arm(request.initial_grant, request.max_grant, request.reserve);
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| workload(raw, request.mode)));
        let sent = match outcome {
            Ok(Ok(result)) => crate::worker_alloc::send_frame(
                TAG_RESULT,
                result.len() as u64,
                crate::worker_alloc::lifetime_max_footprint(),
                result.as_bytes(),
            ),
            Ok(Err(error)) => send_error(&error),
            Err(payload) => send_error(&format!("PREVIEW-SOLVE-PANICKED: {}", panic_message(payload.as_ref()))),
        };
        #[cfg(test)]
        tests::exit_delay();
        if sent {
            0
        } else {
            WORKER_PROTOCOL_EXIT
        }
    }
    #[cfg(not(unix))]
    {
        WORKER_PROTOCOL_EXIT
    }
}

pub(crate) fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    payload
        .downcast_ref::<&str>()
        .map(|s| s.to_string())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "non-string panic payload".to_string())
}

fn send_error(message: &str) -> bool {
    let mut end = message.len().min(MAX_ERROR_FRAME_BYTES as usize);
    while !message.is_char_boundary(end) {
        end -= 1;
    }
    let body = &message.as_bytes()[..end];
    crate::worker_alloc::send_frame(TAG_ERROR, body.len() as u64, 0, body)
}

/// The control channel's fds as inherited: stdin and stdout (a test worker
/// started under libtest receives its output channel on another fd).
#[cfg(unix)]
fn channel_fds() -> (i32, i32) {
    #[cfg(test)]
    if let Some(fd) = std::env::var(tests::OUT_FD_ENV).ok().and_then(|v| v.parse().ok()) {
        return (0, fd);
    }
    (0, 1)
}

/// Moves the channel to private close-on-exec fds and points stdin/stdout at
/// /dev/null, so a stray print can never write into the channel.
#[cfg(unix)]
fn take_channel(in_fd: i32, out_fd: i32) -> Option<(i32, i32)> {
    // SAFETY: plain fd syscalls on fds this process owns; the path is NUL-terminated.
    unsafe {
        let control_in = libc::fcntl(in_fd, libc::F_DUPFD_CLOEXEC, 3);
        let control_out = libc::fcntl(out_fd, libc::F_DUPFD_CLOEXEC, 3);
        if control_in < 0 || control_out < 0 {
            return None;
        }
        let null = libc::open(c"/dev/null".as_ptr(), libc::O_RDWR | libc::O_CLOEXEC);
        if null < 0 || libc::dup2(null, 0) < 0 || libc::dup2(null, 1) < 0 {
            return None;
        }
        libc::close(null);
        for fd in [in_fd, out_fd] {
            if fd > 2 {
                libc::close(fd);
            }
        }
        Some((control_in, control_out))
    }
}

fn workload(raw: Vec<u8>, mode: PreviewSolverMode) -> Result<String, String> {
    #[cfg(test)]
    if let Some(workload) = tests::scripted_workload() {
        return tests::run_scripted(&workload, raw, mode);
    }
    ordinary_solve(raw, mode)
}

/// The ordinary solve, unchanged: materialize the model, release the raw
/// bytes, produce the exact serialized bytes.
fn ordinary_solve(raw: Vec<u8>, mode: PreviewSolverMode) -> Result<String, String> {
    let model: serde_json::Value =
        serde_json::from_slice(&raw).map_err(|error| format!("PREVIEW-SOLVE-MODEL-INVALID: {error}"))?;
    drop(raw);
    crate::solve_preview_mechanics_with_mode(model, mode)
}

// ---------------------------------------------------------------------------
// Parent side
// ---------------------------------------------------------------------------

/// Builds the worker command; the caller configures its stdio.
pub(crate) trait WorkerLauncher: Send + Sync {
    fn command(&self) -> std::io::Result<Command>;
}

/// The product launcher: this executable with [`WORKER_FLAG`] and an empty environment.
#[derive(Debug, Default)]
pub(crate) struct CurrentExeLauncher;

impl WorkerLauncher for CurrentExeLauncher {
    fn command(&self) -> std::io::Result<Command> {
        let mut command = Command::new(std::env::current_exe()?);
        command.arg(WORKER_FLAG).env_clear();
        Ok(command)
    }
}

/// Spawns are serialized so no worker inherits another worker's pipe ends
/// while they are briefly open in this process (pipe creation and
/// close-on-exec are not atomic on macOS).
static SPAWN_LOCK: Mutex<()> = Mutex::new(());

/// Kill access to a running worker for cancellation and session reset.
#[derive(Debug)]
pub(crate) struct WorkerControl {
    pid: i32,
    state: Mutex<ControlState>,
}

#[derive(Debug, Default)]
struct ControlState {
    reaped: bool,
    killed: bool,
}

impl WorkerControl {
    /// SIGKILL, only while the worker is unreaped (its pid cannot be reused).
    pub(crate) fn kill(&self) {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if !state.reaped {
            #[cfg(unix)]
            // SAFETY: the pid is our unreaped child (exited or not), so it names no other process.
            unsafe {
                libc::kill(self.pid, libc::SIGKILL);
            }
            state.killed = true;
        }
    }

    fn killed(&self) -> bool {
        self.state.lock().unwrap_or_else(PoisonError::into_inner).killed
    }

    #[cfg(test)]
    pub(crate) fn pid(&self) -> i32 {
        self.pid
    }
}

/// A spawned worker. Dropping it unreaped kills and reaps it (also while unwinding).
struct WorkerProcess {
    child: Child,
    stdin: Option<ChildStdin>,
    stdout: ChildStdout,
    control: Arc<WorkerControl>,
    status: Option<std::io::Result<ExitStatus>>,
}

impl WorkerProcess {
    fn spawn(launcher: &dyn WorkerLauncher) -> std::io::Result<Self> {
        let mut command = launcher.command()?;
        command.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null());
        let mut child = {
            let _serial = SPAWN_LOCK.lock().unwrap_or_else(PoisonError::into_inner);
            command.spawn()?
        };
        let (Some(stdin), Some(stdout), Ok(pid)) = (child.stdin.take(), child.stdout.take(), i32::try_from(child.id())) else {
            let _ = child.kill();
            let _ = child.wait();
            return Err(std::io::Error::other("worker stdio or pid unavailable"));
        };
        let control = Arc::new(WorkerControl { pid, state: Mutex::default() });
        Ok(Self { child, stdin: Some(stdin), stdout, control, status: None })
    }

    /// Waits for the child to exit without reaping it, marks it reaped under
    /// the control lock (so no kill can race the reap), then reaps it.
    fn reap(&mut self) -> &std::io::Result<ExitStatus> {
        if self.status.is_none() {
            drop(self.stdin.take());
            #[cfg(unix)]
            loop {
                // SAFETY: a zeroed siginfo_t is a valid out-parameter; WNOWAIT leaves the child waitable.
                let mut info: libc::siginfo_t = unsafe { std::mem::zeroed() };
                let rc = unsafe {
                    libc::waitid(libc::P_PID, self.control.pid as libc::id_t, &mut info, libc::WEXITED | libc::WNOWAIT)
                };
                if rc == 0 || std::io::Error::last_os_error().kind() != std::io::ErrorKind::Interrupted {
                    break;
                }
            }
            let mut state = self.control.state.lock().unwrap_or_else(PoisonError::into_inner);
            state.reaped = true;
            self.status = Some(self.child.wait());
        }
        self.status.as_ref().expect("set above")
    }
}

impl Drop for WorkerProcess {
    fn drop(&mut self) {
        if self.status.is_none() {
            self.control.kill();
            self.reap();
        }
    }
}

fn describe(status: &std::io::Result<ExitStatus>) -> String {
    match status {
        Err(error) => format!("its exit status could not be read ({error})"),
        Ok(status) => {
            #[cfg(unix)]
            if let Some(signal) = std::os::unix::process::ExitStatusExt::signal(status) {
                return format!("terminated by signal {signal}");
            }
            match status.code() {
                Some(code) => format!("exit code {code}"),
                None => "exit status unknown".to_string(),
            }
        }
    }
}

fn measured_footprint(pid: i32) -> u64 {
    #[cfg(target_os = "macos")]
    {
        crate::worker_alloc::pid_phys_footprint(pid)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = pid;
        0
    }
}

fn measured_lifetime_max_footprint(pid: i32) -> u64 {
    #[cfg(target_os = "macos")]
    {
        crate::worker_alloc::pid_lifetime_max_footprint(pid)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = pid;
        0
    }
}

/// How a worker job ended. The worker has been reaped before this is returned.
#[derive(Debug)]
pub(crate) enum WorkerOutcome {
    /// The producer's exact bytes, in an exact-capacity buffer, and the result
    /// lease acquired for them while the grant was still held.
    Completed { result: Vec<u8>, lease: Lease },
    Failed(String),
    Refused(ResourceRefusal),
    /// Killed on request (cancellation or session reset).
    Terminated,
}

/// The registry's view of a running worker job.
pub(crate) trait SupervisorHooks {
    /// Registers the worker's kill access; false if the job was already
    /// cancelled (the worker is then killed at once).
    fn attach(&mut self, control: Arc<WorkerControl>) -> bool;
    fn cancelled(&self) -> bool;
    /// The worker's grant grew to `grant_bytes`.
    fn grant_changed(&mut self, grant_bytes: u64);
    /// A measured physical footprint of the worker (reported before any
    /// growth it causes; at the result, its lifetime maximum).
    fn footprint_observed(&mut self, footprint_bytes: u64);
}

/// One worker job's fixed terms.
pub(crate) struct WorkerTerms<'a> {
    pub pool: &'a Arc<MemoryPool>,
    pub launcher: &'a dyn WorkerLauncher,
    pub mode: PreviewSolverMode,
    pub max_grant: u64,
    pub reserve: u64,
}

fn protocol(message: impl std::fmt::Display) -> WorkerOutcome {
    WorkerOutcome::Failed(format!("SOLVE_WORKER_PROTOCOL: {message}"))
}

/// Runs one worker under `grant` (already admitted) and serves its frames
/// until it ends. `input` is the raw model bytes and the lease covering this
/// process's copy of them; both are dropped once sent. The worker is reaped
/// before this returns (and while unwinding); the caller releases `grant`
/// afterwards.
pub(crate) fn supervise(
    terms: &WorkerTerms<'_>,
    grant: &mut Lease,
    input: (Vec<u8>, Lease),
    hooks: &mut dyn SupervisorHooks,
) -> WorkerOutcome {
    let mut process = match WorkerProcess::spawn(terms.launcher) {
        Ok(process) => process,
        Err(error) => return WorkerOutcome::Failed(format!("PREVIEW-SOLVE-WORKER-START-FAILED: {error}")),
    };
    let control = Arc::clone(&process.control);
    if !hooks.attach(Arc::clone(&control)) {
        control.kill();
    }
    let (raw, ingress) = input;
    let request = Request {
        mode: terms.mode,
        initial_grant: grant.bytes(),
        max_grant: terms.max_grant,
        reserve: terms.reserve,
        raw_len: raw.len() as u64,
    };
    if let Some(stdin) = process.stdin.as_mut() {
        // A failed write means the worker has already ended; its frames and
        // exit status say why.
        let _ = stdin.write_all(&encode_request(&request)).and_then(|()| stdin.write_all(&raw)).and_then(|()| stdin.flush());
    }
    drop(raw);
    drop(ingress);

    let mut verdict: Option<WorkerOutcome> = None;
    let mut refusal: Option<ResourceRefusal> = None;
    let mut completed: Option<(Vec<u8>, Lease)> = None;
    let mut error: Option<String> = None;
    let mut frame = [0u8; FRAME_BYTES];
    while process.stdout.read_exact(&mut frame).is_ok() {
        let end = |outcome: WorkerOutcome| {
            control.kill();
            Some(outcome)
        };
        match decode_frame(&frame) {
            (TAG_GROW, requested, reported_footprint) => {
                let current = grant.bytes();
                if requested <= current || requested > terms.max_grant {
                    verdict = end(protocol(format_args!(
                        "growth to {requested} bytes is outside ({current}, {}]",
                        terms.max_grant
                    )));
                    break;
                }
                let footprint = reported_footprint.max(measured_footprint(control.pid));
                hooks.footprint_observed(footprint);
                // The footprint is a need: the new grant also reserves it plus the reserve.
                let footprint_need = footprint.saturating_add(terms.reserve);
                let reply = if footprint_need > terms.max_grant {
                    refusal = Some(ResourceRefusal::WorkerFootprintExceeded { grant_bytes: current, footprint_bytes: footprint });
                    encode_frame(TAG_DENIED, 0, 0)
                } else {
                    let target = requested.max(footprint_need);
                    match grant.grow_worker_to(target, footprint) {
                        Ok(()) => {
                            hooks.grant_changed(target);
                            encode_frame(TAG_GRANTED, target, 0)
                        }
                        Err(cause) => {
                            refusal = Some(ResourceRefusal::WorkerGrowthRefused {
                                granted_bytes: current,
                                requested_bytes: target,
                                cause: Box::new(cause),
                            });
                            encode_frame(TAG_DENIED, 0, 0)
                        }
                    }
                };
                if let Some(stdin) = process.stdin.as_mut() {
                    let _ = stdin.write_all(&reply).and_then(|()| stdin.flush());
                }
            }
            (TAG_CAP_EXCEEDED, needed, _) => {
                refusal = Some(ResourceRefusal::WorkerCapExceeded { cap_bytes: terms.max_grant, requested_bytes: needed });
            }
            (TAG_REFUSED, _, _) if refusal.is_some() => {}
            (TAG_RESULT, length, reported_peak) => {
                let current = grant.bytes();
                if length > current {
                    verdict = end(protocol(format_args!(
                        "a result of {length} bytes exceeds the worker's grant of {current} bytes"
                    )));
                    break;
                }
                // Before anything is released: the worker's real high-water
                // footprint. Above the grant it is recorded (the grant covered
                // less than reality between exchanges); above the budget the
                // solve did not fit and its result is refused.
                let peak = reported_peak.max(measured_lifetime_max_footprint(control.pid));
                hooks.footprint_observed(peak);
                if peak > terms.max_grant {
                    verdict = end(WorkerOutcome::Refused(ResourceRefusal::WorkerFootprintExceeded {
                        grant_bytes: current,
                        footprint_bytes: peak,
                    }));
                    break;
                }
                if hooks.cancelled() {
                    control.kill();
                    break;
                }
                // The result lease is admitted while the grant is still held.
                let lease = match terms.pool.acquire(resident_envelope(length), LeasePurpose::Result) {
                    Ok(lease) => lease,
                    Err(refused) => {
                        verdict = end(WorkerOutcome::Refused(refused));
                        break;
                    }
                };
                let Ok(length) = usize::try_from(length) else {
                    verdict = end(protocol("result length"));
                    break;
                };
                let mut result = vec![0u8; length];
                if process.stdout.read_exact(&mut result).is_err() {
                    verdict = end(protocol("the result frame was truncated"));
                    break;
                }
                completed = Some((result, lease));
                break;
            }
            (TAG_ERROR, length, _) => {
                if length > MAX_ERROR_FRAME_BYTES {
                    verdict = end(protocol(format_args!("an error frame of {length} bytes")));
                    break;
                }
                let mut message = vec![0u8; length as usize];
                if process.stdout.read_exact(&mut message).is_err() {
                    verdict = end(protocol("the error frame was truncated"));
                    break;
                }
                error = Some(String::from_utf8_lossy(&message).into_owned());
                break;
            }
            (tag, _, _) => {
                verdict = end(protocol(format_args!("unexpected frame {:?}", String::from_utf8_lossy(&tag))));
                break;
            }
        }
    }
    let status = process.reap();
    if let Some(outcome) = verdict {
        return outcome;
    }
    if let Some((result, lease)) = completed {
        return match status {
            Ok(status) if status.success() => WorkerOutcome::Completed { result, lease },
            _ if control.killed() => WorkerOutcome::Terminated,
            other => WorkerOutcome::Failed(format!("SOLVE_WORKER_EXITED: after its result, {}", describe(other))),
        };
    }
    if let Some(message) = error {
        return WorkerOutcome::Failed(message);
    }
    if let Some(refusal) = refusal {
        return WorkerOutcome::Refused(refusal);
    }
    if control.killed() {
        return WorkerOutcome::Terminated;
    }
    match status {
        Ok(status) if status.code() == Some(WORKER_CAP_EXIT) => protocol("a cap exit without a refusal frame"),
        other => WorkerOutcome::Failed(format!("SOLVE_WORKER_EXITED: {}", describe(other))),
    }
}

#[cfg(test)]
pub(crate) mod test_support {
    //! The test-binary self-spawn launcher: the worker is this test binary run
    //! with an exact filter on [`super::tests::worker_child_entry`] and an
    //! environment selecting the workload. libtest's own stdout lines are sent
    //! to /dev/null and the output channel is passed on fd 3.
    use super::tests::{CHILD_ENV, OUT_FD_ENV, WORKLOAD_ENV};
    use super::*;
    use std::os::unix::process::CommandExt;

    pub(crate) const ENTRY: &str = "solve_worker::tests::worker_child_entry";

    #[derive(Debug, Clone)]
    pub(crate) struct TestLauncher {
        pub workload: Option<String>,
        pub env: Vec<(String, String)>,
    }

    impl TestLauncher {
        /// The real ordinary solve.
        pub(crate) fn solve() -> Arc<Self> {
            Arc::new(Self { workload: None, env: Vec::new() })
        }

        pub(crate) fn scripted(workload: impl Into<String>) -> Arc<Self> {
            Arc::new(Self { workload: Some(workload.into()), env: Vec::new() })
        }

        pub(crate) fn with_env(mut self, key: &str, value: impl Into<String>) -> Self {
            self.env.push((key.to_string(), value.into()));
            self
        }
    }

    impl WorkerLauncher for TestLauncher {
        fn command(&self) -> std::io::Result<Command> {
            let mut command = Command::new(std::env::current_exe()?);
            command.args([ENTRY, "--exact", "--test-threads=1", "--quiet"]).env_clear().env(CHILD_ENV, "1").env(OUT_FD_ENV, "3");
            if let Some(workload) = &self.workload {
                command.env(WORKLOAD_ENV, workload);
            }
            for (key, value) in &self.env {
                command.env(key, value);
            }
            // SAFETY: only dup2 (async-signal-safe, allocation-free) runs after fork;
            // stdio is already in place (stdout = pipe, stderr = /dev/null).
            unsafe {
                command.pre_exec(|| {
                    if libc::dup2(1, 3) < 0 || libc::dup2(2, 1) < 0 {
                        return Err(std::io::Error::last_os_error());
                    }
                    Ok(())
                });
            }
            Ok(command)
        }
    }

    /// A cross-process gate: workers announce themselves with `started-<pid>`
    /// files and each claims one `release-<n>` file (atomically, by rename),
    /// whose content scripts its outcome.
    #[derive(Debug)]
    pub(crate) struct Gate {
        pub dir: std::path::PathBuf,
        released: std::sync::atomic::AtomicUsize,
    }

    impl Gate {
        pub(crate) fn new(name: &str) -> Arc<Self> {
            // A per-process sequence number keeps concurrent tests' gates apart
            // (the clock alone can repeat within its resolution).
            static SEQUENCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
            let sequence = SEQUENCE.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
            let dir = std::env::temp_dir().join(format!("swbpipe-gate-{name}-{}-{sequence}-{nonce:x}", std::process::id()));
            std::fs::create_dir(&dir).unwrap();
            Arc::new(Self { dir, released: Default::default() })
        }

        pub(crate) fn workload(&self) -> String {
            format!("gate:{}", self.dir.display())
        }

        pub(crate) fn launcher(&self) -> Arc<TestLauncher> {
            TestLauncher::scripted(self.workload())
        }

        /// The pids of the workers that have started (from their `started-<pid>` files).
        pub(crate) fn pids(&self) -> Vec<i32> {
            std::fs::read_dir(&self.dir)
                .map(|entries| {
                    entries
                        .filter_map(Result::ok)
                        .filter_map(|e| e.file_name().to_string_lossy().strip_prefix("started-").and_then(|p| p.parse().ok()))
                        .collect()
                })
                .unwrap_or_default()
        }

        pub(crate) fn started_count(&self) -> usize {
            std::fs::read_dir(&self.dir)
                .map(|entries| entries.filter_map(Result::ok).filter(|e| e.file_name().to_string_lossy().starts_with("started-")).count())
                .unwrap_or(0)
        }

        /// Waits until at least `count` workers have started.
        pub(crate) fn wait_started(&self, count: usize) {
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
            while self.started_count() < count {
                assert!(std::time::Instant::now() < deadline, "{count} workers never started");
                std::thread::sleep(std::time::Duration::from_millis(2));
            }
        }

        /// Releases one worker with `outcome` (result text, `panic`, `abort` or `err:<message>`).
        pub(crate) fn release(&self, outcome: &str) {
            let n = self.released.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            let staging = self.dir.join(format!("staging-{n}"));
            std::fs::write(&staging, outcome).unwrap();
            std::fs::rename(staging, self.dir.join(format!("release-{n:06}"))).unwrap();
        }
    }

    impl Drop for Gate {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.dir);
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::test_support::{Gate, TestLauncher};
    use super::*;
    use crate::memory_pool::test_host::{ScriptedHost, GIB};
    use crate::memory_pool::PressureLevel;
    use std::time::{Duration, Instant};

    pub(crate) const CHILD_ENV: &str = "SWBPIPE_TEST_SOLVE_WORKER";
    pub(crate) const OUT_FD_ENV: &str = "SWBPIPE_TEST_WORKER_OUT_FD";
    pub(crate) const WORKLOAD_ENV: &str = "SWBPIPE_TEST_WORKER_WORKLOAD";
    pub(crate) const EXIT_DELAY_ENV: &str = "SWBPIPE_TEST_WORKER_EXIT_DELAY_MS";
    const MIB: u64 = 1 << 20;

    /// The worker entry in the test binary: inert unless started as a worker.
    #[test]
    fn worker_child_entry() {
        if std::env::var_os(CHILD_ENV).is_some() {
            std::process::exit(run_solve_worker());
        }
    }

    pub(super) fn scripted_workload() -> Option<String> {
        std::env::var(WORKLOAD_ENV).ok()
    }

    pub(super) fn exit_delay() {
        if let Some(ms) = std::env::var(EXIT_DELAY_ENV).ok().and_then(|v| v.parse().ok()) {
            std::thread::sleep(Duration::from_millis(ms));
        }
    }

    fn touched(bytes: usize) -> Vec<u8> {
        let mut block = vec![0u8; bytes];
        for page in block.chunks_mut(4096) {
            page[0] = 1;
        }
        block
    }

    fn sleep_forever() -> ! {
        loop {
            std::thread::sleep(Duration::from_secs(1));
        }
    }

    #[cfg(target_os = "macos")]
    fn lifetime_max_footprint() -> u64 {
        let mut info = std::mem::MaybeUninit::<libc::rusage_info_v4>::zeroed();
        // SAFETY: `info` is a writable rusage_info_v4, the struct RUSAGE_INFO_V4 fills.
        let rc = unsafe { libc::proc_pid_rusage(libc::getpid(), libc::RUSAGE_INFO_V4, info.as_mut_ptr().cast()) };
        assert_eq!(rc, 0);
        // SAFETY: filled on success.
        unsafe { info.assume_init() }.ri_lifetime_max_phys_footprint
    }

    /// Child side of the scripted workloads.
    pub(super) fn run_scripted(workload: &str, raw: Vec<u8>, mode: PreviewSolverMode) -> Result<String, String> {
        let (name, args) = workload.split_once(':').unwrap_or((workload, ""));
        let arg = |index: usize| -> u64 { args.split(':').nth(index).and_then(|v| v.parse().ok()).unwrap_or(0) };
        match name {
            "echo" => Ok(String::from_utf8(raw).unwrap()),
            "mode" => Ok(mode.as_str().to_string()),
            "gate" => {
                let dir = std::path::Path::new(args);
                let pid = std::process::id();
                std::fs::write(dir.join(format!("started-{pid}")), b"").unwrap();
                let claimed = dir.join(format!("claimed-{pid}"));
                let outcome = loop {
                    let mut releases: Vec<_> = std::fs::read_dir(dir)
                        .unwrap()
                        .filter_map(Result::ok)
                        .map(|e| e.path())
                        .filter(|p| p.file_name().is_some_and(|n| n.to_string_lossy().starts_with("release-")))
                        .collect();
                    releases.sort();
                    if let Some(found) = releases.into_iter().find(|p| std::fs::rename(p, &claimed).is_ok()) {
                        let _ = found;
                        break std::fs::read_to_string(&claimed).unwrap();
                    }
                    std::thread::sleep(Duration::from_millis(2));
                };
                match outcome.as_str() {
                    "panic" => panic!("scripted solve panic"),
                    // SAFETY: abort has no preconditions.
                    "abort" => unsafe { libc::abort() },
                    other => match other.strip_prefix("err:") {
                        Some(message) => Err(message.to_string()),
                        None => Ok(other.to_string()),
                    },
                }
            }
            // Holds `arg0` bytes (touched), announces itself, waits for a release,
            // then requests `arg1` bytes in one allocation.
            "alloc_gate" => {
                let first = touched(arg(0) as usize);
                let dir = std::path::Path::new(args.splitn(3, ':').nth(2).unwrap());
                std::fs::write(dir.join(format!("started-{}", std::process::id())), b"").unwrap();
                while !dir.join("release-alloc").exists() {
                    std::thread::sleep(Duration::from_millis(2));
                }
                let second = touched(arg(1) as usize);
                Ok(format!("{{\"held\":{},\"second\":{}}}", first.len(), second.len()))
            }
            // Allocates `arg1` bytes in `arg0`-byte touched steps and keeps them.
            "alloc_steps" => {
                let (step, total) = (arg(0) as usize, arg(1) as usize);
                let mut held = Vec::new();
                while held.len() * step < total {
                    held.push(touched(step));
                }
                Ok(format!("{{\"allocated\":{}}}", held.len() * step))
            }
            "panic" => panic!("scripted worker panic"),
            // SAFETY: abort has no preconditions.
            "abort" => unsafe { libc::abort() },
            "exit" => std::process::exit(arg(0) as i32),
            "sleep" => sleep_forever(),
            "result" => Ok("x".repeat(arg(0) as usize)),
            "oversized_header" => {
                crate::worker_alloc::send_frame(TAG_RESULT, arg(0), 0, &[]);
                sleep_forever()
            }
            "oversized_error" => {
                crate::worker_alloc::send_frame(TAG_ERROR, MAX_ERROR_FRAME_BYTES + 1, 0, &[]);
                sleep_forever()
            }
            "bad_frame" => {
                crate::worker_alloc::send_frame(*b"NOPE", 0, 0, &[]);
                sleep_forever()
            }
            "bad_growth" => {
                crate::worker_alloc::send_frame(TAG_GROW, 1, 0, &[]);
                sleep_forever()
            }
            // Makes `arg0` bytes resident outside the counted heap, then
            // requests a growth (by allocating past the heap cap).
            #[cfg(unix)]
            "footprint" => {
                let bytes = arg(0) as usize;
                // SAFETY: an anonymous private mapping of `bytes`, touched page by page.
                unsafe {
                    let map = libc::mmap(
                        std::ptr::null_mut(),
                        bytes,
                        libc::PROT_READ | libc::PROT_WRITE,
                        libc::MAP_PRIVATE | libc::MAP_ANON,
                        -1,
                        0,
                    );
                    assert_ne!(map, libc::MAP_FAILED);
                    for offset in (0..bytes).step_by(4096) {
                        *(map as *mut u8).add(offset) = 1;
                    }
                }
                let block = touched(arg(1) as usize);
                Ok(block.len().to_string())
            }
            // Freed small blocks the allocator keeps resident: `arg0` bytes of
            // 64-byte boxes are allocated and freed, then `arg1` bytes are
            // filled in touched 1 MiB chunks (a size class the freed blocks
            // cannot serve), so the footprint exceeds the counted heap.
            "retained" => {
                let boxes = (arg(0) / 64) as usize;
                let mut small: Vec<Box<[u8; 64]>> = Vec::with_capacity(boxes);
                for _ in 0..boxes {
                    small.push(Box::new([1u8; 64]));
                }
                drop(small);
                let chunks: Vec<Vec<u8>> = (0..arg(1) / MIB).map(|_| touched(MIB as usize)).collect();
                Ok(format!("{{\"filled\":{}}}", chunks.len() as u64 * MIB))
            }
            // Makes `arg0` bytes resident outside the counted heap, releases
            // them, then returns: only the lifetime maximum shows them.
            #[cfg(unix)]
            "transient_mmap" => {
                let bytes = arg(0) as usize;
                // SAFETY: an anonymous private mapping of `bytes`, touched page by page, then unmapped.
                unsafe {
                    let map = libc::mmap(std::ptr::null_mut(), bytes, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_PRIVATE | libc::MAP_ANON, -1, 0);
                    assert_ne!(map, libc::MAP_FAILED);
                    for offset in (0..bytes).step_by(4096) {
                        *(map as *mut u8).add(offset) = 1;
                    }
                    libc::munmap(map, bytes);
                }
                Ok("{}".to_string())
            }
            // Evidence for the process reserve: footprints of a real worker.
            #[cfg(target_os = "macos")]
            "probe" => {
                let idle = crate::worker_alloc::phys_footprint();
                let idle_counted = crate::worker_alloc::peak_counted();
                let model: serde_json::Value = serde_json::from_slice(&raw).unwrap();
                drop(raw);
                let mut lengths = Vec::new();
                for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
                    lengths.push(crate::solve_preview_mechanics_with_mode(model.clone(), mode)?.len());
                }
                drop(model);
                Ok(serde_json::json!({
                    "idle_footprint": idle,
                    "idle_counted": idle_counted,
                    "post_solve_footprint": crate::worker_alloc::phys_footprint(),
                    "lifetime_max_footprint": lifetime_max_footprint(),
                    "peak_counted": crate::worker_alloc::peak_counted(),
                    "grant": crate::worker_alloc::current_grant(),
                    "result_lengths": lengths,
                })
                .to_string())
            }
            other => Err(format!("unknown scripted workload {other}")),
        }
    }

    // --- parent-side harness ---------------------------------------------

    struct Hooks {
        attached: Option<Arc<WorkerControl>>,
        cancelled: Arc<std::sync::atomic::AtomicBool>,
        grants: Arc<Mutex<Vec<u64>>>,
        /// The grant in force when each footprint was observed.
        current: u64,
        observed: Vec<(u64, u64)>,
    }

    impl SupervisorHooks for Hooks {
        fn attach(&mut self, control: Arc<WorkerControl>) -> bool {
            self.attached = Some(control);
            !self.cancelled()
        }
        fn cancelled(&self) -> bool {
            self.cancelled.load(std::sync::atomic::Ordering::SeqCst)
        }
        fn grant_changed(&mut self, grant_bytes: u64) {
            self.grants.lock().unwrap().push(grant_bytes);
            self.current = grant_bytes;
        }
        fn footprint_observed(&mut self, footprint_bytes: u64) {
            self.observed.push((self.current, footprint_bytes));
        }
    }

    struct Run {
        outcome: WorkerOutcome,
        grants: Vec<u64>,
        /// (grant in force, footprint) per observation; the last is the result's lifetime peak.
        observed: Vec<(u64, u64)>,
        pid: Option<i32>,
    }

    fn pool() -> Arc<MemoryPool> {
        MemoryPool::with_wait_tick(ScriptedHost::normal(), Duration::from_millis(5))
    }

    /// Runs one worker to its end under a fresh grant; the grant is dropped
    /// after `supervise` returns (after the reap).
    fn run(pool: &Arc<MemoryPool>, launcher: &dyn WorkerLauncher, body: &[u8], max_grant: u64, reserve: u64) -> Run {
        let start = initial_grant(reserve, crate::ingress::prescan(body).unwrap().lease_bytes().unwrap()).unwrap();
        let mut grant = pool.acquire(start, LeasePurpose::Worker).unwrap();
        let ingress = pool.acquire(crate::ingress::allocation_envelope(body.len() as u64).unwrap(), LeasePurpose::Ingress).unwrap();
        let mut hooks = Hooks { attached: None, cancelled: Default::default(), grants: Default::default(), current: start, observed: Vec::new() };
        let terms = WorkerTerms { pool, launcher, mode: PreviewSolverMode::SparseInteractive, max_grant, reserve };
        let outcome = supervise(&terms, &mut grant, (body.to_vec(), ingress), &mut hooks);
        assert_eq!(pool.reserved(LeasePurpose::Ingress), 0, "the raw copy's lease dropped once sent");
        assert_eq!(pool.running_workers(), 1, "the grant is held until supervise returns");
        drop(grant);
        assert_eq!(pool.running_workers(), 0);
        let grants = hooks.grants.lock().unwrap().clone();
        Run { outcome, grants, observed: hooks.observed, pid: hooks.attached.map(|c| c.pid()) }
    }

    fn assert_reaped(pid: Option<i32>) {
        let pid = pid.expect("a worker was attached");
        // SAFETY: probing with signal 0; ESRCH means no such process (reaped).
        let alive = unsafe { libc::kill(pid, 0) } == 0;
        assert!(!alive, "worker {pid} was not reaped");
    }

    const BODY: &[u8] = br#"{"model":"small"}"#;

    #[test]
    fn worker_echoes_the_request_and_releases_after_reap() {
        let pool = pool();
        let run = run(&pool, &*TestLauncher::scripted("echo"), BODY, 12 * GIB, PROCESS_RESERVE_BYTES);
        match run.outcome {
            WorkerOutcome::Completed { result, lease } => {
                assert_eq!(result, BODY);
                assert_eq!(result.capacity(), BODY.len(), "an exact-capacity buffer");
                assert_eq!(lease.bytes(), resident_envelope(BODY.len() as u64));
                assert_eq!(pool.reserved(LeasePurpose::Result), lease.bytes());
            }
            other => panic!("{other:?}"),
        }
        assert_eq!(pool.reserved_total(), 0);
        assert_reaped(run.pid);
        let mode = run_with_mode(&pool, PreviewSolverMode::DenseScrutiny);
        assert_eq!(mode, "dense_scrutiny");
    }

    fn run_with_mode(pool: &Arc<MemoryPool>, mode: PreviewSolverMode) -> String {
        let launcher = TestLauncher::scripted("mode");
        let mut grant = pool.acquire(PROCESS_RESERVE_BYTES + MIB, LeasePurpose::Worker).unwrap();
        let ingress = pool.acquire(64, LeasePurpose::Ingress).unwrap();
        let mut hooks = Hooks {
            attached: None,
            cancelled: Default::default(),
            grants: Default::default(),
            current: PROCESS_RESERVE_BYTES + MIB,
            observed: Vec::new(),
        };
        let terms = WorkerTerms { pool, launcher: &*launcher, mode, max_grant: 12 * GIB, reserve: PROCESS_RESERVE_BYTES };
        match supervise(&terms, &mut grant, (BODY.to_vec(), ingress), &mut hooks) {
            WorkerOutcome::Completed { result, .. } => String::from_utf8(result).unwrap(),
            other => panic!("{other:?}"),
        }
    }

    /// Owner test "cap enforcement": an allocation beyond the per-invocation
    /// cap ends the worker with a typed refusal before it is made; this test
    /// process is unaffected, the child is reaped and the grant released.
    #[test]
    fn allocation_beyond_the_cap_is_a_typed_refusal_and_never_allocated() {
        let pool = pool();
        let budget = PROCESS_RESERVE_BYTES + 24 * MIB;
        // 4 MiB steps toward 64 MiB: the step crossing 24 MiB of heap is refused.
        let launcher = TestLauncher::scripted(format!("alloc_steps:{}:{}", 4 * MIB, 64 * MIB));
        let run = run(&pool, &*launcher, BODY, budget, PROCESS_RESERVE_BYTES);
        match run.outcome {
            WorkerOutcome::Refused(ResourceRefusal::WorkerCapExceeded { cap_bytes, requested_bytes }) => {
                assert_eq!(cap_bytes, budget);
                assert!(requested_bytes > budget && requested_bytes <= budget + 5 * MIB, "{requested_bytes}");
            }
            other => panic!("{other:?}"),
        }
        assert_eq!(run.grants.last(), Some(&budget), "grown to the budget, never above it");
        assert!(run.grants.windows(2).all(|w| w[0] < w[1]));
        assert_eq!(pool.reserved_total(), 0);
        assert_reaped(run.pid);
        // The same workload within the budget completes.
        let fits = TestLauncher::scripted(format!("alloc_steps:{}:{}", 4 * MIB, 16 * MIB));
        let run = super::tests::run(&pool, &*fits, BODY, budget, PROCESS_RESERVE_BYTES);
        assert!(matches!(&run.outcome, WorkerOutcome::Completed { result, .. } if result == br#"{"allocated":16777216}"#), "{:?}", run.outcome);
    }

    /// A single request above the budget is refused before the allocation.
    #[test]
    fn one_allocation_above_the_budget_is_refused_up_front() {
        let pool = pool();
        let budget = PROCESS_RESERVE_BYTES + 8 * MIB;
        let launcher = TestLauncher::scripted(format!("alloc_steps:{}:{}", 512 * GIB, 512 * GIB));
        let run = run(&pool, &*launcher, BODY, budget, PROCESS_RESERVE_BYTES);
        assert!(
            matches!(run.outcome, WorkerOutcome::Refused(ResourceRefusal::WorkerCapExceeded { cap_bytes, requested_bytes })
                if cap_bytes == budget && requested_bytes >= 512 * GIB),
            "{:?}",
            run.outcome
        );
        assert!(run.grants.is_empty(), "no growth was requested for an impossible need");
        assert_eq!(pool.reserved_total(), 0);
        assert_reaped(run.pid);
    }

    /// Owner test: a growth is granted, then a later growth is refused by the
    /// host provider (elevated pressure): the job ends `worker_growth_refused`.
    #[test]
    fn growth_granted_then_refused_by_the_host_is_typed() {
        let host = ScriptedHost::normal();
        let pool = MemoryPool::with_wait_tick(host.clone(), Duration::from_millis(5));
        let gate = Gate::new("growth");
        let reserve = PROCESS_RESERVE_BYTES;
        let start = initial_grant(reserve, crate::ingress::prescan(BODY).unwrap().lease_bytes().unwrap()).unwrap();
        let first = 2 * MIB; // above the start's heap cap: one growth to 2 x start
        let second = 2 * start; // above the grown heap cap: a second growth
        let launcher = TestLauncher::scripted(format!("alloc_gate:{first}:{second}:{}", gate.dir.display()));
        let (pool2, host2, gate_dir) = (pool.clone(), host.clone(), gate.dir.clone());
        let releaser = std::thread::spawn(move || {
            let deadline = Instant::now() + Duration::from_secs(60);
            while std::fs::read_dir(&gate_dir).unwrap().count() == 0 {
                assert!(Instant::now() < deadline);
                std::thread::sleep(Duration::from_millis(2));
            }
            assert_eq!(pool2.reserved(LeasePurpose::Worker), 2 * start, "the first growth was granted");
            host2.set(|s| s.pressure = PressureLevel::Warning);
            std::fs::write(gate_dir.join("release-alloc"), b"").unwrap();
        });
        let run = run(&pool, &*launcher, BODY, 12 * GIB, reserve);
        releaser.join().unwrap();
        match run.outcome {
            WorkerOutcome::Refused(ResourceRefusal::WorkerGrowthRefused { granted_bytes, requested_bytes, cause }) => {
                assert_eq!(granted_bytes, 2 * start);
                assert!(requested_bytes >= second + reserve && requested_bytes <= 4 * start + 4 * MIB, "{requested_bytes}");
                assert_eq!(*cause, ResourceRefusal::PressureElevated { level: PressureLevel::Warning });
            }
            other => panic!("{other:?}"),
        }
        assert_eq!(run.grants, vec![2 * start]);
        assert_eq!(pool.reserved_total(), 0);
        assert_reaped(run.pid);
    }

    /// Owner tests "child crash/panic": a panic is reported as the producer's
    /// panic error; an abort is a worker exit by signal; neither touches this process.
    #[test]
    fn child_panic_and_abort_end_failed_without_affecting_the_host() {
        let pool = pool();
        let run = run(&pool, &*TestLauncher::scripted("panic"), BODY, 12 * GIB, PROCESS_RESERVE_BYTES);
        assert!(matches!(&run.outcome, WorkerOutcome::Failed(m) if m == "PREVIEW-SOLVE-PANICKED: scripted worker panic"), "{:?}", run.outcome);
        assert_reaped(run.pid);
        let run = super::tests::run(&pool, &*TestLauncher::scripted("abort"), BODY, 12 * GIB, PROCESS_RESERVE_BYTES);
        assert!(matches!(&run.outcome, WorkerOutcome::Failed(m) if m == "SOLVE_WORKER_EXITED: terminated by signal 6"), "{:?}", run.outcome);
        assert_reaped(run.pid);
        let run = super::tests::run(&pool, &*TestLauncher::scripted("exit:3"), BODY, 12 * GIB, PROCESS_RESERVE_BYTES);
        assert!(matches!(&run.outcome, WorkerOutcome::Failed(m) if m == "SOLVE_WORKER_EXITED: exit code 3"), "{:?}", run.outcome);
        let run = super::tests::run(&pool, &*TestLauncher::scripted(format!("exit:{WORKER_CAP_EXIT}")), BODY, 12 * GIB, PROCESS_RESERVE_BYTES);
        assert!(matches!(&run.outcome, WorkerOutcome::Failed(m) if m.starts_with("SOLVE_WORKER_PROTOCOL:")), "{:?}", run.outcome);
        assert_eq!(pool.reserved_total(), 0);
    }

    /// Owner test "IPC result-size bounds": a result header above the grant is
    /// a protocol failure; R bytes are never allocated and the child is killed.
    #[test]
    fn oversized_result_header_fails_without_allocating_it() {
        let pool = pool();
        let run = run(&pool, &*TestLauncher::scripted(format!("oversized_header:{}", 1u64 << 50)), BODY, 12 * GIB, PROCESS_RESERVE_BYTES);
        match &run.outcome {
            WorkerOutcome::Failed(message) => {
                assert!(message.starts_with("SOLVE_WORKER_PROTOCOL: a result of 1125899906842624 bytes exceeds the worker's grant"), "{message}")
            }
            other => panic!("{other:?}"),
        }
        assert_eq!(pool.reserved_total(), 0, "no result lease was taken");
        assert_reaped(run.pid);
        for workload in ["oversized_error", "bad_frame", "bad_growth"] {
            let run = super::tests::run(&pool, &*TestLauncher::scripted(workload), BODY, 12 * GIB, PROCESS_RESERVE_BYTES);
            assert!(matches!(&run.outcome, WorkerOutcome::Failed(m) if m.starts_with("SOLVE_WORKER_PROTOCOL:")), "{workload}: {:?}", run.outcome);
            assert_reaped(run.pid);
        }
        assert_eq!(pool.reserved_total(), 0);
    }

    /// A result within the grant is read into an exact buffer; its lease is
    /// refused (typed) when the host cannot cover it, and the child is killed.
    #[test]
    fn result_frames_within_the_grant_are_leased_or_refused() {
        let host = ScriptedHost::normal();
        let pool = MemoryPool::with_wait_tick(host.clone(), Duration::from_millis(5));
        let run = run(&pool, &*TestLauncher::scripted(format!("result:{}", 3 * MIB)), BODY, 12 * GIB, PROCESS_RESERVE_BYTES);
        match run.outcome {
            WorkerOutcome::Completed { result, lease } => {
                assert_eq!((result.len(), result.capacity()), (3 << 20, 3 << 20));
                assert_eq!(lease.bytes(), 3 * MIB);
            }
            other => panic!("{other:?}"),
        }
        // Headroom covering the grant but not grant + result: the result lease
        // is refused (a 512 KiB result fits the starting heap cap, so no growth).
        let start = initial_grant(PROCESS_RESERVE_BYTES, crate::ingress::prescan(BODY).unwrap().lease_bytes().unwrap()).unwrap();
        host.set(|s| s.headroom_bytes = Some(start + 256 * 1024));
        let run = super::tests::run(&pool, &*TestLauncher::scripted(format!("result:{}", 512 * 1024)), BODY, 12 * GIB, PROCESS_RESERVE_BYTES);
        assert!(matches!(run.outcome, WorkerOutcome::Refused(ResourceRefusal::HeadroomInsufficient { .. })), "{:?}", run.outcome);
        assert_reaped(run.pid);
        assert_eq!(pool.reserved_total(), 0);
    }

    /// Memory resident outside the counted heap (a stand-in for allocator
    /// overhead) is a growth need: the grant grows to cover footprint + reserve
    /// and the job completes; beyond the budget it is refused.
    #[cfg(target_os = "macos")]
    #[test]
    fn memory_outside_the_count_is_a_growth_need_and_refused_only_beyond_the_budget() {
        let pool = pool();
        let reserve = 16 * MIB;
        let start = initial_grant(reserve, crate::ingress::prescan(BODY).unwrap().lease_bytes().unwrap()).unwrap();
        // 48 MiB resident outside the heap, then a 2 MiB allocation past the heap cap.
        let launcher = TestLauncher::scripted(format!("footprint:{}:{}", 48 * MIB, 2 * MIB));
        let run = run(&pool, &*launcher, BODY, 12 * GIB, reserve);
        assert!(matches!(&run.outcome, WorkerOutcome::Completed { result, .. } if result == b"2097152"), "{:?}", run.outcome);
        let (grant_before, footprint) = run.observed[0];
        assert_eq!(grant_before, start);
        assert!(footprint > 48 * MIB && footprint > start, "{footprint}");
        assert!(run.grants[0] >= footprint + reserve, "the grant covers footprint + reserve: {:?}", run.grants);
        assert_reaped(run.pid);
        drop(run);
        // A budget the footprint + reserve does not fit: refused, typed.
        let budget = reserve + 40 * MIB;
        let run = super::tests::run(&pool, &*launcher, BODY, budget, reserve);
        match run.outcome {
            WorkerOutcome::Refused(ResourceRefusal::WorkerFootprintExceeded { grant_bytes, footprint_bytes }) => {
                assert_eq!(grant_bytes, start);
                assert!(footprint_bytes > 48 * MIB && footprint_bytes + reserve > budget, "{footprint_bytes}");
            }
            other => panic!("{other:?}"),
        }
        assert!(run.grants.is_empty());
        assert_eq!(pool.reserved_total(), 0);
        assert_reaped(run.pid);
    }

    /// Owner follow-up: freed small blocks the allocator keeps resident push
    /// the footprint above the grant; that is a growth need (within the
    /// budget the job grows and completes), not a refusal.
    #[cfg(target_os = "macos")]
    #[test]
    fn retained_freed_blocks_above_the_grant_grow_it_and_complete() {
        let pool = pool();
        let reserve = 4 * MIB;
        let launcher = TestLauncher::scripted(format!("retained:{}:{}", 24 * MIB, 40 * MIB));
        let run = run(&pool, &*launcher, BODY, 12 * GIB, reserve);
        assert!(matches!(&run.outcome, WorkerOutcome::Completed { result, .. } if result == br#"{"filled":41943040}"#), "{:?}", run.outcome);
        eprintln!("retained scenario: grants {:?} observed (grant, footprint) {:?}", run.grants, run.observed);
        let above = run.observed.iter().position(|(grant, footprint)| footprint > grant);
        let step = above.expect("the retained blocks pushed the footprint above the grant at a growth exchange");
        assert!(step < run.grants.len(), "observed at a growth exchange, not only at the result");
        assert!(run.grants[step] >= run.observed[step].1 + reserve, "that growth covered footprint + reserve");
        assert_reaped(run.pid);
        drop(run);
        assert_eq!(pool.reserved_total(), 0);
    }

    /// At the result frame the lifetime-maximum footprint is checked before
    /// anything is released: above the grant it is recorded; above the budget
    /// the result is refused.
    #[cfg(target_os = "macos")]
    #[test]
    fn the_lifetime_peak_footprint_is_checked_at_the_result() {
        let pool = pool();
        let reserve = 16 * MIB;
        let start = initial_grant(reserve, crate::ingress::prescan(BODY).unwrap().lease_bytes().unwrap()).unwrap();
        let launcher = TestLauncher::scripted(format!("transient_mmap:{}", 32 * MIB));
        let run = run(&pool, &*launcher, BODY, 12 * GIB, reserve);
        assert!(matches!(run.outcome, WorkerOutcome::Completed { .. }), "{:?}", run.outcome);
        let (grant, peak) = *run.observed.last().unwrap();
        assert!(grant == start && peak > 32 * MIB && peak > grant, "recorded above the grant: {peak}");
        drop(run);
        // The budget equals the starting grant: the peak exceeded it, so the result is refused.
        let run = super::tests::run(&pool, &*launcher, BODY, start, reserve);
        match run.outcome {
            WorkerOutcome::Refused(ResourceRefusal::WorkerFootprintExceeded { grant_bytes, footprint_bytes }) => {
                assert_eq!(grant_bytes, start);
                assert!(footprint_bytes > 32 * MIB, "{footprint_bytes}");
            }
            other => panic!("{other:?}"),
        }
        assert_eq!(pool.reserved_total(), 0, "no result lease was kept");
        assert_reaped(run.pid);
    }

    /// Owner test "cancellation": a kill while the worker runs is classified
    /// as terminated; the grant stays reserved until the child is reaped.
    #[test]
    fn kill_while_running_terminates_and_releases_only_after_reap() {
        let pool = pool();
        let launcher = TestLauncher::scripted("sleep");
        let mut grant = pool.acquire(PROCESS_RESERVE_BYTES + MIB, LeasePurpose::Worker).unwrap();
        let ingress = pool.acquire(64, LeasePurpose::Ingress).unwrap();
        let attached: Arc<Mutex<Option<Arc<WorkerControl>>>> = Arc::default();
        struct KillHooks(Arc<Mutex<Option<Arc<WorkerControl>>>>);
        impl SupervisorHooks for KillHooks {
            fn attach(&mut self, control: Arc<WorkerControl>) -> bool {
                *self.0.lock().unwrap() = Some(control);
                true
            }
            fn cancelled(&self) -> bool {
                false
            }
            fn grant_changed(&mut self, _: u64) {}
            fn footprint_observed(&mut self, _: u64) {}
        }
        let (pool2, attached2) = (pool.clone(), attached.clone());
        let supervisor = std::thread::spawn(move || {
            let terms = WorkerTerms {
                pool: &pool2,
                launcher: &*launcher,
                mode: PreviewSolverMode::SparseInteractive,
                max_grant: 12 * GIB,
                reserve: PROCESS_RESERVE_BYTES,
            };
            let outcome = supervise(&terms, &mut grant, (BODY.to_vec(), ingress), &mut KillHooks(attached2));
            (outcome, grant)
        });
        let deadline = Instant::now() + Duration::from_secs(60);
        let control = loop {
            if let Some(control) = attached.lock().unwrap().clone() {
                break control;
            }
            assert!(Instant::now() < deadline);
            std::thread::sleep(Duration::from_millis(2));
        };
        std::thread::sleep(Duration::from_millis(50));
        assert_eq!(pool.reserved(LeasePurpose::Worker), PROCESS_RESERVE_BYTES + MIB, "held while running");
        control.kill();
        let (outcome, grant) = supervisor.join().unwrap();
        assert!(matches!(outcome, WorkerOutcome::Terminated), "{outcome:?}");
        assert_reaped(Some(control.pid()));
        // Reaped, and the grant still held by its owner until dropped.
        assert_eq!(pool.reserved(LeasePurpose::Worker), PROCESS_RESERVE_BYTES + MIB);
        drop(grant);
        assert_eq!(pool.reserved_total(), 0);
        // A kill after the reap targets nothing.
        control.kill();
    }

    /// Owner test "supervisor panics": a panic in the supervisor unwinds
    /// through the worker's handle, which kills and reaps it; the grant is
    /// still held by its owner until that owner drops it.
    #[test]
    fn a_supervisor_panic_kills_and_reaps_the_worker_before_the_grant_drops() {
        struct PanicHooks(Arc<Mutex<Option<i32>>>);
        impl SupervisorHooks for PanicHooks {
            fn attach(&mut self, control: Arc<WorkerControl>) -> bool {
                *self.0.lock().unwrap() = Some(control.pid());
                true
            }
            fn cancelled(&self) -> bool {
                false
            }
            fn grant_changed(&mut self, _: u64) {
                panic!("supervisor bookkeeping failed");
            }
            fn footprint_observed(&mut self, _: u64) {}
        }
        let pool = pool();
        let launcher = TestLauncher::scripted(format!("alloc_steps:{}:{}", 4 * MIB, 8 * MIB));
        let mut grant = pool.acquire(PROCESS_RESERVE_BYTES + MIB, LeasePurpose::Worker).unwrap();
        let ingress = pool.acquire(64, LeasePurpose::Ingress).unwrap();
        let pid: Arc<Mutex<Option<i32>>> = Arc::default();
        let terms = WorkerTerms {
            pool: &pool,
            launcher: &*launcher,
            mode: PreviewSolverMode::SparseInteractive,
            max_grant: 12 * GIB,
            reserve: PROCESS_RESERVE_BYTES,
        };
        let unwound = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            supervise(&terms, &mut grant, (BODY.to_vec(), ingress), &mut PanicHooks(pid.clone()))
        }));
        assert!(unwound.is_err());
        assert_reaped(*pid.lock().unwrap());
        assert_eq!(pool.reserved(LeasePurpose::Ingress), 0);
        assert_eq!(pool.running_workers(), 1, "the grant outlives the reap");
        drop(grant);
        assert_eq!((pool.reserved_total(), pool.running_workers()), (0, 0));
    }

    /// Evidence for [`PROCESS_RESERVE_BYTES`]: the idle, post-solve and
    /// lifetime-maximum physical footprints of a real worker (this crate's
    /// test binary in worker mode) solving the bundled demo in both modes,
    /// against its counted heap and grant. The reserve must cover the
    /// non-heap footprint with margin.
    #[cfg(target_os = "macos")]
    #[test]
    fn process_reserve_covers_a_real_workers_measured_footprint() {
        let pool = pool();
        let body = serde_json::to_vec(&crate::load_preview_model().unwrap()).unwrap();
        let run = run(&pool, &*TestLauncher::scripted("probe"), &body, 12 * GIB, PROCESS_RESERVE_BYTES);
        let WorkerOutcome::Completed { result, .. } = run.outcome else { panic!("{:?}", run.outcome) };
        let probe: serde_json::Value = serde_json::from_slice(&result).unwrap();
        eprintln!("PROCESS_RESERVE evidence: {probe} grants {:?}", run.grants);
        let field = |key: &str| probe[key].as_u64().unwrap();
        let (idle, post, max, peak, grant) =
            (field("idle_footprint"), field("post_solve_footprint"), field("lifetime_max_footprint"), field("peak_counted"), field("grant"));
        assert!(idle > 0 && post > 0 && max >= post);
        // The non-heap share (footprint beyond the counted heap) stays well inside the reserve.
        assert!(idle * 2 <= PROCESS_RESERVE_BYTES, "idle footprint {idle}");
        assert!(max.saturating_sub(peak) * 2 <= PROCESS_RESERVE_BYTES, "lifetime max {max} vs counted peak {peak}");
        assert!(max <= grant, "the footprint stayed within the grant");
    }
}
