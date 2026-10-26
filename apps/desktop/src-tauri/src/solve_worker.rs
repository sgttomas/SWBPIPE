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
//!   then L raw model bytes (UTF-8 JSON). The child closes every inherited fd
//!   but stdio and its control pair, and reads all of the request before its
//!   cap is armed, so no growth exchange can interleave with it.
//! - 28-byte frames ([`crate::worker_alloc::FRAME_BYTES`]): a 4-byte tag and
//!   three u64 fields (unused ones 0). Child -> parent:
//!   `GROW {requested_grant, needed_grant, phys_footprint}` (requested is the
//!   doubling step, needed = counted heap + reserve; answered by
//!   `GRNT {grant >= needed}` or `DENY`), `CAPX {needed_grant, max_grant}` or
//!   `RFSD {needed_grant, grant}` (a final refusal, then exit
//!   [`WORKER_CAP_EXIT`]), `RSLT {R, lifetime_max_footprint}` + R bytes (the
//!   producer's exact bytes), `FAIL {n}` + n bytes (n <= 64 KiB).
//!
//! The grant (a [`LeasePurpose::Worker`] lease) starts at the process reserve
//! plus the request's ingress bound, rounded up to 1 MiB, and grows on demand
//! by atomic pool admissions with a fresh headroom verdict, never above the
//! budget; the child's allocator is capped at grant - reserve
//! ([`crate::worker_alloc`]). Each attempt of a growth step tries the
//! doubling step, then (if refused) the minimal need, under one pool lock.
//! When the minimal need is refused only with `headroom_insufficient`, the
//! step re-samples within the window the intake and result leases use
//! ([`Lease::grow_resampling`]; owner, 2026-10-26) and is admitted at the
//! first attempt that passes the unchanged rule. A minimal need still refused
//! ends the job (`worker_growth_refused`, whose `granted_bytes` is the lease
//! before the step and `requested_bytes` that minimal need): at once for any
//! other refusal, otherwise once the window has elapsed. Cancellation and
//! session reset (a kill, then a wake of the pool's waiters) end a waiting
//! step at the next attempt.
//!
//! The measured physical footprint (the larger of the child's report and the
//! parent's own reading) is a need as well, so memory the count does not see
//! (allocator-retained freed blocks, metadata, stacks, mappings) is reserved
//! too: at a growth exchange the lease grows to cover footprint + reserve
//! (capped at the budget), and while no frame arrives the supervisor polls
//! the footprint every [`FOOTPRINT_POLL_INTERVAL`] and grows the lease the
//! same way when the footprint exceeds it (a refusal kills the worker:
//! `worker_growth_refused`). A footprint above the budget kills the worker
//! and ends the job `worker_footprint_exceeded`. The lease the pool holds can
//! therefore run ahead of the grant the child knows; the child's heap cap
//! follows only its own grant.
//!
//! While an exchange step waits, the child is blocked in its allocator on the
//! reply. A poll step's child is running: if the step's first attempt is
//! refused only with `headroom_insufficient`, the child is stopped (SIGSTOP)
//! for the wait, so its excess stays at what the poll measured, and continued
//! (SIGCONT) once admitted (Agent 0, 2026-10-26, option (a)). Between attempts
//! the footprint is measured again; above the budget the job ends
//! `worker_footprint_exceeded`. A refused step kills the stopped child; so do
//! cancellation and session reset (SIGKILL ends a stopped process). A frame
//! the child wrote before it was stopped is read only if the step is
//! admitted. The worker runs in its own process group of this session, so
//! signals to the app's group (a terminal's Ctrl-C) no longer reach it.
//! Instead it ends itself when the app dies: before reading its request it
//! checks that the pid the parent passed ([`PARENT_PID_ENV`]) is still its
//! parent and starts a watchdog thread (a raw pthread with a 64 KiB stack,
//! allocating nothing) that `_exit`s once the parent exits, woken at once by
//! a kqueue `NOTE_EXIT` registration on macOS, else by comparing `getppid()`
//! every 100 ms. A stopped worker's watchdog is stopped too; for that case
//! the worker keeps SIGHUP's default action, so the orphaned group's SIGHUP
//! and SIGCONT from the kernel end it.
//! Stopped time counts against no timeout or staleness check of the job (each
//! attempt takes a fresh host sample); only the window bounds the stop.
//!
//! Residual: the footprint is sampled, not continuous. Memory outside the
//! counted heap can exceed the lease (or the budget) for at most one poll
//! interval (100 ms) plus the time to measure and kill, and an excess a poll
//! measured stays unreserved, frozen with the child stopped, for at most the
//! re-sampling window; the counted heap itself never exceeds its cap. The result is also not polled while its
//! bytes are read (the worker has finished computing). At the result frame
//! the parent reads the worker's lifetime-maximum footprint, which covers
//! every moment, records it as the job's `worker_peak_footprint_bytes` and
//! refuses the result if it exceeded the budget.
//!
//! A result of R bytes must not exceed the grant; its result lease is
//! acquired while the grant is still held, the bytes are read into an
//! exact-capacity buffer, the child must exit 0, and only after the child is
//! reaped is the grant released. A kill (cancellation, session reset, refusal
//! or protocol violation), a stop or a continue targets the child only while
//! it is unreaped.

use crate::memory_pool::{Lease, LeasePurpose, MemoryPool, ResampleStop, ResidencyProbe, ResourceRefusal};
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
/// mode) and on the release app binary; a footprint beyond it is not
/// assumed away but reserved (growth exchanges and polling) or refused.
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
        // SIGHUP keeps its default action (terminate) even if the app was
        // started with it ignored (an ignored disposition survives exec): the
        // orphaned-group SIGHUP must end a worker whose app died while it was
        // stopped. Nothing in the worker handles or ignores SIGHUP.
        // SAFETY: setting a default disposition has no preconditions.
        unsafe {
            libc::signal(libc::SIGHUP, libc::SIG_DFL);
        }
        let parent = spawning_parent();
        let (in_fd, out_fd) = channel_fds();
        let Some((control_in, control_out)) = take_channel(in_fd, out_fd) else {
            return WORKER_PROTOCOL_EXIT;
        };
        close_inherited_fds(&[control_in, control_out]);
        // Before the allocator counts anything; the watchdog allocates nothing.
        if !start_parent_watchdog(parent) {
            return WORKER_PROTOCOL_EXIT;
        }
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

/// The environment variable in which the parent passes its pid to the worker.
pub(crate) const PARENT_PID_ENV: &str = "SWBPIPE_SOLVE_WORKER_PARENT_PID";

/// The pid of the process that spawned this worker: the one it passed
/// ([`PARENT_PID_ENV`]), or the current parent if none was passed. A worker
/// whose spawner is no longer its parent (it died before this check) exits.
#[cfg(unix)]
fn spawning_parent() -> libc::pid_t {
    // SAFETY: getppid has no preconditions.
    let current = unsafe { libc::getppid() };
    let passed = std::env::var(PARENT_PID_ENV).ok().and_then(|pid| pid.parse::<libc::pid_t>().ok());
    if passed.is_some_and(|passed| passed != current) {
        // SAFETY: _exit ends the process at once; nothing needs unwinding yet.
        unsafe { libc::_exit(WORKER_PROTOCOL_EXIT) }
    }
    current
}

/// The parent the watchdog watches.
#[cfg(unix)]
static WATCHED_PARENT: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
/// The watchdog's kqueue (-1 when it polls instead).
#[cfg(unix)]
pub(crate) static WATCHDOG_KQUEUE: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1);
/// The watchdog thread's stack (its touched pages count in the footprint).
#[cfg(unix)]
const WATCHDOG_STACK_BYTES: usize = 64 * 1024;
/// How often the fallback watchdog compares the parent pid.
#[cfg(unix)]
const WATCHDOG_POLL_MICROS: libc::useconds_t = 100_000;

/// Starts the parent-death watchdog: one raw thread (no Rust allocation,
/// nothing counted) that `_exit`s the worker once `parent` has exited, so a
/// running worker never outlives the app that started it. On macOS it waits
/// on a kqueue `EVFILT_PROC`/`NOTE_EXIT` registration for the parent (woken
/// at once); where that is unavailable it compares `getppid()` with `parent`
/// every 100 ms. A parent already gone at registration ends the worker here.
/// A stopped worker's threads are stopped too: the orphaned-group SIGHUP
/// covers that case. False if no thread could be started.
#[cfg(unix)]
fn start_parent_watchdog(parent: libc::pid_t) -> bool {
    use std::sync::atomic::Ordering::SeqCst;
    WATCHED_PARENT.store(parent, SeqCst);
    #[cfg(target_os = "macos")]
    // SAFETY: plain kqueue/kevent calls on a kqueue this process owns; the
    // changelist is one initialized kevent and no events are read here.
    unsafe {
        let kq = libc::kqueue();
        if kq >= 0 {
            libc::fcntl(kq, libc::F_SETFD, libc::FD_CLOEXEC);
            let change = libc::kevent {
                ident: parent as libc::uintptr_t,
                filter: libc::EVFILT_PROC,
                flags: libc::EV_ADD | libc::EV_ONESHOT,
                fflags: libc::NOTE_EXIT,
                data: 0,
                udata: std::ptr::null_mut(),
            };
            if libc::kevent(kq, &change, 1, std::ptr::null_mut(), 0, std::ptr::null()) == 0 {
                WATCHDOG_KQUEUE.store(kq, SeqCst);
            } else {
                let gone = std::io::Error::last_os_error().raw_os_error() == Some(libc::ESRCH);
                libc::close(kq);
                if gone {
                    libc::_exit(WORKER_PROTOCOL_EXIT);
                }
            }
        }
    }
    // Registered after the parent may already have died: check once more.
    // SAFETY: getppid has no preconditions.
    if unsafe { libc::getppid() } != parent {
        // SAFETY: as above.
        unsafe { libc::_exit(WORKER_PROTOCOL_EXIT) }
    }
    // SAFETY: an initialized attribute object, a valid entry point that never
    // returns, and an out-parameter for the thread.
    unsafe {
        let mut attr = std::mem::MaybeUninit::<libc::pthread_attr_t>::uninit();
        if libc::pthread_attr_init(attr.as_mut_ptr()) != 0 {
            return false;
        }
        libc::pthread_attr_setstacksize(attr.as_mut_ptr(), WATCHDOG_STACK_BYTES.max(libc::PTHREAD_STACK_MIN));
        let mut thread = std::mem::MaybeUninit::<libc::pthread_t>::uninit();
        let started = libc::pthread_create(thread.as_mut_ptr(), attr.as_ptr(), parent_watchdog, std::ptr::null_mut()) == 0;
        libc::pthread_attr_destroy(attr.as_mut_ptr());
        if started {
            libc::pthread_detach(thread.assume_init());
        }
        started
    }
}

/// The watchdog thread: allocation-free, it ends the process with `_exit`
/// (no unwinding, no destructors) once the parent has exited.
#[cfg(unix)]
extern "C" fn parent_watchdog(_: *mut libc::c_void) -> *mut libc::c_void {
    use std::sync::atomic::Ordering::SeqCst;
    let parent = WATCHED_PARENT.load(SeqCst);
    #[cfg(target_os = "macos")]
    {
        let kq = WATCHDOG_KQUEUE.load(SeqCst);
        if kq >= 0 {
            loop {
                // SAFETY: a zeroed kevent is a valid out-parameter for one event.
                let mut event: libc::kevent = unsafe { std::mem::zeroed() };
                let n = unsafe { libc::kevent(kq, std::ptr::null(), 0, &mut event, 1, std::ptr::null()) };
                if n > 0 {
                    // SAFETY: _exit has no preconditions.
                    unsafe { libc::_exit(WORKER_PROTOCOL_EXIT) }
                }
                if n < 0 && std::io::Error::last_os_error().raw_os_error() != Some(libc::EINTR) {
                    break;
                }
            }
        }
    }
    loop {
        // SAFETY: getppid and usleep have no preconditions.
        unsafe {
            if libc::getppid() != parent {
                libc::_exit(WORKER_PROTOCOL_EXIT);
            }
            libc::usleep(WATCHDOG_POLL_MICROS);
        }
    }
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

/// The highest fd number [`close_inherited_fds`] closes (exclusive), when the
/// soft RLIMIT_NOFILE is larger or unlimited.
#[cfg(unix)]
const MAX_FD_SCAN: libc::rlim_t = 65_536;

/// Closes every fd other than 0-2 and `keep`, before the request is read: a
/// worker holds no descriptor it was not given (another worker's pipe end, a
/// file the app had open). macOS has no closefrom, so every fd below the soft
/// RLIMIT_NOFILE (at most [`MAX_FD_SCAN`]) is closed; EBADF is ignored.
#[cfg(unix)]
fn close_inherited_fds(keep: &[i32]) {
    let mut limit = libc::rlimit { rlim_cur: 0, rlim_max: 0 };
    // SAFETY: `limit` is a valid out-parameter.
    let soft = if unsafe { libc::getrlimit(libc::RLIMIT_NOFILE, &mut limit) } == 0 {
        limit.rlim_cur.min(MAX_FD_SCAN)
    } else {
        MAX_FD_SCAN
    };
    for fd in 3..i32::try_from(soft).unwrap_or(i32::MAX) {
        if !keep.contains(&fd) {
            // SAFETY: closing an fd this process may hold; nothing else uses it yet.
            unsafe {
                libc::close(fd);
            }
        }
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
    /// SIGSTOP was sent for a poll step's wait and no SIGCONT since.
    stopped: bool,
}

impl WorkerControl {
    /// SIGKILL, only while the worker is unreaped (its pid cannot be reused).
    /// A stopped worker is killed too: SIGKILL ends a stopped process.
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

    /// SIGSTOP for a poll step's re-sampling wait, only while the worker is
    /// unreaped and not killed (a kill wins: it is never undone). Returns
    /// whether the worker was stopped.
    fn stop(&self) -> bool {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if state.reaped || state.killed {
            return false;
        }
        #[cfg(unix)]
        // SAFETY: as for `kill`, the pid is our unreaped child.
        {
            state.stopped = unsafe { libc::kill(self.pid, libc::SIGSTOP) } == 0;
        }
        state.stopped
    }

    /// SIGCONT after [`Self::stop`], only while the worker is unreaped and
    /// not killed (a killed worker is ending; continuing it changes nothing).
    fn resume(&self) {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if state.stopped && !state.reaped && !state.killed {
            #[cfg(unix)]
            // SAFETY: as for `kill`, the pid is our unreaped child.
            unsafe {
                libc::kill(self.pid, libc::SIGCONT);
            }
        }
        state.stopped = false;
    }
}

impl ResidencyProbe for WorkerControl {
    /// Measured under the control lock, so a reaped worker (whose pid could be
    /// reused) is never measured.
    fn realized_bytes(&self) -> Option<u64> {
        let state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if state.reaped {
            return None;
        }
        #[cfg(target_os = "macos")]
        {
            crate::worker_alloc::pid_realized_bytes(self.pid)
        }
        #[cfg(not(target_os = "macos"))]
        {
            None
        }
    }
}

#[cfg(test)]
impl WorkerControl {
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
        // Its own process group in this session: if this process dies while
        // the worker is stopped (a poll step's wait), the group is orphaned
        // and the kernel sends it SIGHUP and SIGCONT, which end it.
        #[cfg(unix)]
        std::os::unix::process::CommandExt::process_group(&mut command, 0);
        // The worker's parent-death watchdog watches this pid.
        command.env(PARENT_PID_ENV, std::process::id().to_string());
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

/// How often the supervisor measures a worker's physical footprint while no
/// frame arrives: the bound on how long memory outside the counted heap can
/// exceed the grant (or the budget) unnoticed.
pub(crate) const FOOTPRINT_POLL_INTERVAL: std::time::Duration = std::time::Duration::from_millis(100);

/// The lease growth a measured footprint above the lease asks for between
/// exchanges: (preferred, minimal) = (min(budget, max(minimal, 2 * lease)),
/// min(budget, footprint + reserve)). Saturating, so exact for every budget up
/// to u64::MAX.
pub(crate) fn poll_growth(current: u64, footprint: u64, reserve: u64, max_grant: u64) -> (u64, u64) {
    let minimal = max_grant.min(footprint.saturating_add(reserve));
    let preferred = max_grant.min(minimal.max(current.saturating_mul(2)));
    (preferred, minimal)
}

/// The lease growth a `GROW {requested, needed, footprint}` asks for: the
/// lease also covers the footprint plus the reserve (capped at the budget).
/// Returns (preferred, minimal).
pub(crate) fn exchange_growth(requested: u64, needed: u64, footprint: u64, reserve: u64, max_grant: u64) -> (u64, u64) {
    let footprint_need = max_grant.min(footprint.saturating_add(reserve));
    (requested.max(footprint_need), needed.max(footprint_need))
}

/// One growth step: grows the worker's lease to `preferred` (the doubling
/// step) or, if that is refused, to `minimal` (the need), both tried under one
/// pool lock with a fresh headroom verdict per attempt. While the minimal need
/// is refused only with `headroom_insufficient`, the step re-samples within
/// the pool's window ([`Lease::grow_resampling`]); `waiting`, if given, runs
/// once before the first wait ([`Lease::grow_resampling_with`]) and `stop`
/// between attempts. Returns the new lease bytes, the refusal of the minimal
/// need that stands, or the stop.
fn grow_grant(
    grant: &mut Lease,
    preferred: u64,
    minimal: u64,
    waiting: Option<&mut dyn FnMut()>,
    stop: &mut dyn FnMut() -> bool,
) -> Result<u64, ResampleStop> {
    if minimal <= grant.bytes() {
        return Ok(grant.bytes());
    }
    let both = [preferred, minimal];
    let targets = if preferred > minimal { &both[..] } else { &both[1..] };
    match waiting {
        None => grant.grow_resampling(targets, stop),
        Some(waiting) => grant.grow_resampling_with(targets, waiting, stop),
    }
}

#[cfg(unix)]
enum Readiness {
    Readable,
    Timeout,
}

/// Waits up to `timeout` for the worker's output (a frame, EOF or an error).
#[cfg(unix)]
fn wait_readable(stdout: &ChildStdout, timeout: std::time::Duration) -> Readiness {
    use std::os::fd::AsRawFd;
    let mut descriptor = libc::pollfd { fd: stdout.as_raw_fd(), events: libc::POLLIN, revents: 0 };
    let millis = i32::try_from(timeout.as_millis()).unwrap_or(i32::MAX);
    // SAFETY: one valid pollfd for the duration of the call.
    let rc = unsafe { libc::poll(&mut descriptor, 1, millis) };
    // Ready, EOF/error (revents) or a failed poll: the read decides.
    if rc == 0 {
        Readiness::Timeout
    } else {
        Readiness::Readable
    }
}

/// Runs one worker under `grant` (already admitted) and serves its frames
/// until it ends. `input` is the raw model bytes and the lease covering this
/// process's copy of them; both are dropped once sent. The worker is reaped
/// before this returns (and while unwinding); the caller releases `grant`
/// afterwards.
///
/// The lease (the pool's grant) can run ahead of the grant the child knows:
/// footprint growth (at a growth exchange or a poll) reserves memory outside
/// the counted heap without raising the child's heap cap.
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
    // The pool measures this worker afresh at every admission (0 once reaped).
    grant.set_residency_probe(Some(Arc::clone(&control) as Arc<dyn ResidencyProbe>));
    if !hooks.attach(Arc::clone(&control)) {
        control.kill();
    }
    let (raw, ingress) = input;
    // The grant the child knows (its heap cap is this less the reserve).
    let mut child_grant = grant.bytes();
    let request = Request {
        mode: terms.mode,
        initial_grant: child_grant,
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
    loop {
        let end = |outcome: WorkerOutcome| {
            control.kill();
            Some(outcome)
        };
        #[cfg(unix)]
        if let Readiness::Timeout = wait_readable(&process.stdout, FOOTPRINT_POLL_INTERVAL) {
            // Memory outside the counted heap, between exchanges.
            let footprint = measured_footprint(control.pid);
            if footprint == 0 || refusal.is_some() {
                continue;
            }
            hooks.footprint_observed(footprint);
            let current = grant.bytes();
            if footprint > terms.max_grant {
                verdict = end(WorkerOutcome::Refused(ResourceRefusal::WorkerFootprintExceeded {
                    grant_bytes: current,
                    footprint_bytes: footprint,
                }));
                break;
            }
            if footprint > current {
                let (preferred, minimal) = poll_growth(current, footprint, terms.reserve, terms.max_grant);
                // The child is running: if the first attempt is refused only
                // with `headroom_insufficient`, it is stopped for the
                // re-sampling wait, so the excess stays at what this poll
                // measured, and continued once admitted. A frame it wrote
                // before the stop is read after the step resolves.
                let mut stopped = false;
                let mut exceeded = None;
                let mut observed = footprint;
                let grown = grow_grant(grant, preferred, minimal, Some(&mut || stopped = control.stop()), &mut || {
                    if control.killed() || hooks.cancelled() {
                        return true;
                    }
                    let now = measured_footprint(control.pid);
                    if now > observed {
                        observed = now;
                        hooks.footprint_observed(now);
                    }
                    if now > terms.max_grant {
                        exceeded = Some(now);
                        return true;
                    }
                    false
                });
                match grown {
                    Ok(bytes) => {
                        if stopped {
                            control.resume();
                        }
                        if bytes != current {
                            hooks.grant_changed(bytes);
                        }
                    }
                    // A stopped child is killed (SIGKILL ends it) and reaped below.
                    Err(ResampleStop::Refused(cause)) => {
                        verdict = end(WorkerOutcome::Refused(ResourceRefusal::WorkerGrowthRefused {
                            granted_bytes: current,
                            requested_bytes: minimal,
                            cause: Box::new(cause),
                        }));
                        break;
                    }
                    Err(ResampleStop::Cancelled) => {
                        match exceeded {
                            Some(footprint_bytes) => {
                                verdict = end(WorkerOutcome::Refused(ResourceRefusal::WorkerFootprintExceeded {
                                    grant_bytes: current,
                                    footprint_bytes,
                                }));
                            }
                            // Cancellation or session reset: as for a kill while running.
                            None => control.kill(),
                        }
                        break;
                    }
                }
            }
            continue;
        }
        if process.stdout.read_exact(&mut frame).is_err() {
            break;
        }
        match decode_frame(&frame) {
            (TAG_GROW, requested, needed, reported_footprint) => {
                if needed <= child_grant || needed > requested || requested > terms.max_grant {
                    verdict = end(protocol(format_args!(
                        "growth to {needed}..={requested} bytes from {child_grant} is outside the budget of {}",
                        terms.max_grant
                    )));
                    break;
                }
                let current = grant.bytes();
                let footprint = reported_footprint.max(measured_footprint(control.pid));
                hooks.footprint_observed(footprint);
                let reply = if footprint > terms.max_grant {
                    refusal = Some(ResourceRefusal::WorkerFootprintExceeded { grant_bytes: current, footprint_bytes: footprint });
                    encode_frame(TAG_DENIED, 0, 0, 0)
                } else {
                    // The footprint is a need too: the lease also covers it plus the reserve.
                    let (preferred, minimal) = exchange_growth(requested, needed, footprint, terms.reserve, terms.max_grant);
                    // The child is blocked in its allocator until it reads the
                    // reply, so it allocates nothing while the step re-samples.
                    match grow_grant(grant, preferred, minimal, None, &mut || control.killed() || hooks.cancelled()) {
                        Ok(bytes) => {
                            if bytes != current {
                                hooks.grant_changed(bytes);
                            }
                            // The child's heap grows to the doubling step if the
                            // lease covers it, else to its need.
                            child_grant = if bytes >= preferred { requested } else { needed };
                            encode_frame(TAG_GRANTED, child_grant, 0, 0)
                        }
                        Err(ResampleStop::Refused(cause)) => {
                            refusal = Some(ResourceRefusal::WorkerGrowthRefused {
                                granted_bytes: current,
                                requested_bytes: minimal,
                                cause: Box::new(cause),
                            });
                            encode_frame(TAG_DENIED, 0, 0, 0)
                        }
                        // Cancellation or session reset during the wait: as for a kill while running.
                        Err(ResampleStop::Cancelled) => {
                            control.kill();
                            break;
                        }
                    }
                };
                if let Some(stdin) = process.stdin.as_mut() {
                    let _ = stdin.write_all(&reply).and_then(|()| stdin.flush());
                }
            }
            (TAG_CAP_EXCEEDED, needed, _, _) => {
                refusal = Some(ResourceRefusal::WorkerCapExceeded { cap_bytes: terms.max_grant, requested_bytes: needed });
            }
            (TAG_REFUSED, _, _, _) if refusal.is_some() => {}
            (TAG_RESULT, length, reported_peak, _) => {
                let current = grant.bytes();
                if length > current {
                    verdict = end(protocol(format_args!(
                        "a result of {length} bytes exceeds the worker's grant of {current} bytes"
                    )));
                    break;
                }
                // Before anything is released: the worker's real high-water
                // footprint. Above the grant it is recorded (the grant covered
                // less than reality for at most a poll interval); above the
                // budget the solve did not fit and its result is refused.
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
                // The result lease is admitted while the grant is still held. It
                // is held as an uncredited (ingress) reservation while the bytes
                // are read, and becomes a Result lease, which interactive
                // admissions count as written, only once they are written.
                let mut lease = match terms.pool.acquire_resampling(resident_envelope(length), LeasePurpose::Ingress, &|| hooks.cancelled()) {
                    Ok(lease) => lease,
                    Err(ResampleStop::Refused(refused)) => {
                        verdict = end(WorkerOutcome::Refused(refused));
                        break;
                    }
                    // As for a cancellation seen just before the admission.
                    Err(ResampleStop::Cancelled) => {
                        control.kill();
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
                lease.set_purpose(LeasePurpose::Result);
                completed = Some((result, lease));
                break;
            }
            (TAG_ERROR, length, _, _) => {
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
            (tag, _, _, _) => {
                verdict = end(protocol(format_args!("unexpected frame {:?}", String::from_utf8_lossy(&tag))));
                break;
            }
        }
    }
    let status = process.reap();
    grant.set_residency_probe(None);
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
        /// An extra fd (a copy of /dev/null) the worker inherits, as a leaked descriptor would be.
        pub inherited_fd: Option<i32>,
    }

    impl TestLauncher {
        /// The real ordinary solve.
        pub(crate) fn solve() -> Arc<Self> {
            Arc::new(Self { workload: None, env: Vec::new(), inherited_fd: None })
        }

        pub(crate) fn scripted(workload: impl Into<String>) -> Arc<Self> {
            Arc::new(Self { workload: Some(workload.into()), env: Vec::new(), inherited_fd: None })
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
            let inherited = self.inherited_fd;
            // SAFETY: only dup2 (async-signal-safe, allocation-free) runs after fork;
            // stdio is already in place (stdout = pipe, stderr = /dev/null).
            unsafe {
                command.pre_exec(move || {
                    if libc::dup2(1, 3) < 0 || libc::dup2(2, 1) < 0 {
                        return Err(std::io::Error::last_os_error());
                    }
                    if let Some(fd) = inherited {
                        if libc::dup2(2, fd) < 0 {
                            return Err(std::io::Error::last_os_error());
                        }
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
    /// A directory: the worker's exit is held until `exit-released` exists in it.
    pub(crate) const EXIT_GATE_ENV: &str = "SWBPIPE_TEST_WORKER_EXIT_GATE";
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

    /// Holds the worker's exit, after its terminal frame is sent, until the
    /// test releases it ([`EXIT_GATE_ENV`]) or removes the directory.
    pub(super) fn exit_delay() {
        if let Some(dir) = std::env::var_os(EXIT_GATE_ENV).map(std::path::PathBuf::from) {
            await_release(&dir, "exit-released");
        }
    }

    /// Waits until `dir/name` exists, or `dir` is gone (its test has ended).
    fn await_release(dir: &std::path::Path, name: &str) {
        while dir.is_dir() && !dir.join(name).exists() {
            std::thread::sleep(Duration::from_millis(2));
        }
    }

    /// Maps `total` bytes outside the counted heap (anonymous mappings,
    /// touched) in `step`-byte steps, `interval_ms` apart; returns the bytes mapped.
    #[cfg(unix)]
    fn map_uncounted(step: usize, total: usize, interval_ms: u64) -> usize {
        let mut mapped = 0;
        while mapped < total {
            // SAFETY: an anonymous private mapping of `step` bytes, touched page by page (kept until exit).
            unsafe {
                let map = libc::mmap(std::ptr::null_mut(), step, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_PRIVATE | libc::MAP_ANON, -1, 0);
                assert_ne!(map, libc::MAP_FAILED);
                for offset in (0..step).step_by(4096) {
                    *(map as *mut u8).add(offset) = 1;
                }
            }
            mapped += step;
            std::thread::sleep(Duration::from_millis(interval_ms));
        }
        mapped
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
            // Grows memory outside the counted heap (anonymous mappings, touched)
            // in `arg0`-byte steps to `arg1` bytes, `arg2` ms apart, then holds
            // it for `arg3` ms: only polling sees it.
            #[cfg(unix)]
            "uncounted" => {
                let mapped = map_uncounted(arg(0) as usize, arg(1) as usize, arg(2));
                std::thread::sleep(Duration::from_millis(arg(3)));
                Ok(mapped.to_string())
            }
            // As `uncounted`, but the memory is held until the test creates
            // `hold-released` in the directory `arg3`, not for a fixed time.
            #[cfg(unix)]
            "uncounted_gate" => {
                let mapped = map_uncounted(arg(0) as usize, arg(1) as usize, arg(2));
                await_release(std::path::Path::new(args.splitn(4, ':').nth(3).unwrap()), "hold-released");
                Ok(mapped.to_string())
            }
            // Announces itself (`started-<pid>` in the directory `arg1`), waits
            // for `go` there, makes `arg0` bytes resident outside the counted
            // heap (one touched mapping), then holds them until
            // `hold-released` exists and returns the bytes mapped: only a
            // poll sees them, never a growth exchange.
            #[cfg(unix)]
            "map_go" => {
                let dir = std::path::Path::new(args.splitn(2, ':').nth(1).unwrap());
                std::fs::write(dir.join(format!("started-{}", std::process::id())), b"").unwrap();
                await_release(dir, "go");
                let mapped = map_uncounted(arg(0) as usize, arg(0) as usize, 0);
                await_release(dir, "hold-released");
                Ok(mapped.to_string())
            }
            // The fds open in the worker.
            #[cfg(unix)]
            "fds" => {
                // SAFETY: F_GETFD only probes whether the fd is open.
                let open: Vec<i32> = (0..1024).filter(|fd| unsafe { libc::fcntl(*fd, libc::F_GETFD) } != -1).collect();
                let watchdog = WATCHDOG_KQUEUE.load(std::sync::atomic::Ordering::SeqCst);
                Ok(format!("{{\"open\":{open:?},\"watchdog\":{watchdog}}}"))
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
            // them, then returns: only the lifetime maximum shows them. With a
            // directory (`transient_mmap:<bytes>:<dir>`) the excursion starts
            // only once `excursion-go` exists there and `excursion-done` is
            // created after the unmap, so a test can hold the supervisor (in a
            // hook, after its poll has sampled) until the excursion is over.
            #[cfg(unix)]
            "transient_mmap" => {
                let bytes = arg(0) as usize;
                let gate = args.split_once(':').map(|(_, dir)| std::path::Path::new(dir));
                if let Some(dir) = gate {
                    await_release(dir, "excursion-go");
                }
                // SAFETY: an anonymous private mapping of `bytes`, touched page by page, then unmapped.
                unsafe {
                    let map = libc::mmap(std::ptr::null_mut(), bytes, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_PRIVATE | libc::MAP_ANON, -1, 0);
                    assert_ne!(map, libc::MAP_FAILED);
                    for offset in (0..bytes).step_by(4096) {
                        *(map as *mut u8).add(offset) = 1;
                    }
                    libc::munmap(map, bytes);
                }
                if let Some(dir) = gate {
                    std::fs::write(dir.join("excursion-done"), b"").unwrap();
                }
                Ok("{}".to_string())
            }
            // Announces itself (`started-<pid>` in the directory `arg1`), sends
            // a valid `{}` result whose frame reports a lifetime peak of `arg0`
            // bytes (no such memory is used), then waits for the supervisor's
            // kill. Not killed within 10 s, it leaves `unkilled-<pid>` and
            // exits 1, so a missing kill fails its test rather than hangs it.
            "reported_peak" => {
                let dir = std::path::Path::new(args.splitn(2, ':').nth(1).unwrap());
                let pid = std::process::id();
                std::fs::write(dir.join(format!("started-{pid}")), b"").unwrap();
                crate::worker_alloc::send_frame(TAG_RESULT, 2, arg(0), b"{}");
                std::thread::sleep(Duration::from_secs(10));
                let _ = std::fs::write(dir.join(format!("unkilled-{pid}")), b"");
                std::process::exit(1)
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

    #[derive(Debug, Clone, Copy)]
    enum Event {
        /// A footprint observation and the lease in force at that moment.
        Footprint { grant: u64, footprint: u64 },
        Grant(u64),
    }

    struct Hooks {
        attached: Option<Arc<WorkerControl>>,
        /// When the supervisor attached the spawned worker.
        attached_at: Option<Instant>,
        cancelled: Arc<std::sync::atomic::AtomicBool>,
        grants: Arc<Mutex<Vec<u64>>>,
        /// The grant in force when each footprint was observed.
        current: u64,
        observed: Vec<(u64, u64)>,
        events: Vec<Event>,
        /// A `transient_mmap` gate directory: the first footprint observation
        /// holds the supervisor until the worker's excursion is over.
        excursion_gate: Option<std::path::PathBuf>,
        /// The attached worker's control, for a test thread (cancellation).
        shared: SharedControl,
    }

    type SharedControl = Arc<Mutex<Option<Arc<WorkerControl>>>>;

    impl SupervisorHooks for Hooks {
        fn attach(&mut self, control: Arc<WorkerControl>) -> bool {
            *self.shared.lock().unwrap() = Some(Arc::clone(&control));
            self.attached = Some(control);
            self.attached_at = Some(Instant::now());
            !self.cancelled()
        }
        fn cancelled(&self) -> bool {
            self.cancelled.load(std::sync::atomic::Ordering::SeqCst)
        }
        fn grant_changed(&mut self, grant_bytes: u64) {
            self.grants.lock().unwrap().push(grant_bytes);
            self.current = grant_bytes;
            self.events.push(Event::Grant(grant_bytes));
        }
        fn footprint_observed(&mut self, footprint_bytes: u64) {
            self.observed.push((self.current, footprint_bytes));
            self.events.push(Event::Footprint { grant: self.current, footprint: footprint_bytes });
            // This footprint was sampled before the gate opens, so before the
            // excursion; until the worker has unmapped it the supervisor does
            // not return here to sample again.
            if let Some(dir) = self.excursion_gate.take() {
                std::fs::write(dir.join("excursion-go"), b"").unwrap();
                // A hang guard: the hold ends after 60 s, and then only with the excursion over.
                let deadline = Instant::now() + Duration::from_secs(60);
                while dir.is_dir() && !dir.join("excursion-done").exists() && Instant::now() < deadline {
                    std::thread::sleep(Duration::from_millis(2));
                }
                // Unwinding kills and reaps the worker (its handle's drop) and removes the gate.
                assert!(dir.join("excursion-done").exists(), "the worker's excursion was not over within the hold: {}", dir.display());
            }
        }
    }

    struct Run {
        outcome: WorkerOutcome,
        grants: Vec<u64>,
        /// (grant in force, footprint) per observation; the last is the result's lifetime peak.
        observed: Vec<(u64, u64)>,
        events: Vec<Event>,
        pid: Option<i32>,
        attached_at: Option<Instant>,
    }

    impl Run {
        /// Whether a footprint above the lease was followed at once by a
        /// growth covering it plus the reserve (capped at the budget).
        fn grew_to_cover_a_footprint_above_the_grant(&self, reserve: u64, budget: u64) -> bool {
            self.events.windows(2).any(|pair| {
                matches!(pair, [Event::Footprint { grant, footprint }, Event::Grant(new)]
                    if footprint > grant && *new >= budget.min(footprint + reserve))
            })
        }
    }

    fn pool() -> Arc<MemoryPool> {
        MemoryPool::with_wait_tick(ScriptedHost::normal(), Duration::from_millis(5))
    }

    /// Runs one worker to its end under a fresh grant; the grant is dropped
    /// after `supervise` returns (after the reap).
    fn run(pool: &Arc<MemoryPool>, launcher: &dyn WorkerLauncher, body: &[u8], max_grant: u64, reserve: u64) -> Run {
        run_gated(pool, launcher, body, max_grant, reserve, None)
    }

    /// As [`run`], with a `transient_mmap` gate directory the hooks open.
    fn run_gated(
        pool: &Arc<MemoryPool>,
        launcher: &dyn WorkerLauncher,
        body: &[u8],
        max_grant: u64,
        reserve: u64,
        excursion_gate: Option<std::path::PathBuf>,
    ) -> Run {
        run_hooked(pool, launcher, body, max_grant, reserve, excursion_gate, Default::default(), Default::default())
    }

    /// As [`run_gated`], with the cancellation flag and control slot shared with the caller.
    #[allow(clippy::too_many_arguments)]
    fn run_hooked(
        pool: &Arc<MemoryPool>,
        launcher: &dyn WorkerLauncher,
        body: &[u8],
        max_grant: u64,
        reserve: u64,
        excursion_gate: Option<std::path::PathBuf>,
        cancelled: Arc<std::sync::atomic::AtomicBool>,
        shared: SharedControl,
    ) -> Run {
        let start = initial_grant(reserve, crate::ingress::prescan(body).unwrap().lease_bytes().unwrap()).unwrap();
        let mut grant = pool.acquire(start, LeasePurpose::Worker).unwrap();
        let ingress = pool.acquire(crate::ingress::allocation_envelope(body.len() as u64).unwrap(), LeasePurpose::Ingress).unwrap();
        let mut hooks = Hooks {
            attached: None,
            attached_at: None,
            cancelled,
            grants: Default::default(),
            current: start,
            observed: Vec::new(),
            events: Vec::new(),
            excursion_gate,
            shared,
        };
        let terms = WorkerTerms { pool, launcher, mode: PreviewSolverMode::SparseInteractive, max_grant, reserve };
        let outcome = supervise(&terms, &mut grant, (body.to_vec(), ingress), &mut hooks);
        assert_eq!(pool.reserved(LeasePurpose::Ingress), 0, "the raw copy's lease dropped once sent");
        assert_eq!(pool.running_workers(), 1, "the grant is held until supervise returns");
        drop(grant);
        assert_eq!(pool.running_workers(), 0);
        let grants = hooks.grants.lock().unwrap().clone();
        Run {
            outcome,
            grants,
            observed: hooks.observed,
            events: hooks.events,
            pid: hooks.attached.map(|c| c.pid()),
            attached_at: hooks.attached_at,
        }
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
            attached_at: None,
            cancelled: Default::default(),
            grants: Default::default(),
            current: PROCESS_RESERVE_BYTES + MIB,
            observed: Vec::new(),
            events: Vec::new(),
            excursion_gate: None,
            shared: Default::default(),
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
        // A pool with room for the grant but not grant + result: the result
        // lease is refused (a 512 KiB result fits the starting heap cap, so no growth).
        let start = initial_grant(PROCESS_RESERVE_BYTES, crate::ingress::prescan(BODY).unwrap().lease_bytes().unwrap()).unwrap();
        let ingress = crate::ingress::allocation_envelope(BODY.len() as u64).unwrap();
        let blocker = pool.acquire(12 * GIB - start - ingress - 256 * 1024, LeasePurpose::Custody).unwrap();
        let run = super::tests::run(&pool, &*TestLauncher::scripted(format!("result:{}", 512 * 1024)), BODY, 12 * GIB, PROCESS_RESERVE_BYTES);
        assert!(matches!(run.outcome, WorkerOutcome::Refused(ResourceRefusal::PoolExhausted { .. })), "{:?}", run.outcome);
        assert_reaped(run.pid);
        drop(blocker);
        assert_eq!(pool.reserved_total(), 0);
    }

    /// Memory resident outside the counted heap (a stand-in for allocator
    /// overhead) is a growth need: the grant grows to cover footprint + reserve
    /// and the job completes; a footprint beyond the budget is refused.
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
        assert!(run.grew_to_cover_a_footprint_above_the_grant(reserve, 12 * GIB), "{:?}", run.events);
        assert!(*run.grants.last().unwrap() >= 48 * MIB + reserve);
        assert_reaped(run.pid);
        drop(run);
        // A budget the footprint itself does not fit: refused, typed. A poll
        // during the touch may refuse first, correctly, with a part-touched
        // sample above the budget, so only the budget bounds the footprint.
        let budget = reserve + 24 * MIB;
        let run = super::tests::run(&pool, &*launcher, BODY, budget, reserve);
        match run.outcome {
            WorkerOutcome::Refused(ResourceRefusal::WorkerFootprintExceeded { grant_bytes, footprint_bytes }) => {
                assert!(grant_bytes >= start && grant_bytes <= budget);
                assert!(footprint_bytes > budget, "{footprint_bytes}");
            }
            other => panic!("{other:?}"),
        }
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
        eprintln!("retained scenario: events {:?}", run.events);
        assert!(
            run.grew_to_cover_a_footprint_above_the_grant(reserve, 12 * GIB),
            "the retained blocks pushed the footprint above the grant and a growth covered it: {:?}",
            run.events
        );
        assert_reaped(run.pid);
        drop(run);
        assert_eq!(pool.reserved_total(), 0);
    }

    /// Review S2: memory that grows outside the counted heap without any
    /// growth exchange is found by the supervisor's footprint polling: within
    /// the budget the grant grows to cover it; beyond the budget the worker is
    /// killed and the job refused.
    #[cfg(target_os = "macos")]
    #[test]
    fn polling_detects_uncounted_growth_grows_within_the_budget_and_refuses_beyond_it() {
        let pool = pool();
        let reserve = 16 * MIB;
        let start = initial_grant(reserve, crate::ingress::prescan(BODY).unwrap().lease_bytes().unwrap()).unwrap();
        // The worker holds its memory until a poll has grown the grant to
        // cover it, not for a fixed time a delayed supervisor could miss.
        let gate = Gate::new("uncounted");
        let launcher = TestLauncher::scripted(format!("uncounted_gate:{}:{}:50:{}", 8 * MIB, 48 * MIB, gate.dir.display()));
        let releaser = {
            let (pool, dir) = (pool.clone(), gate.dir.clone());
            std::thread::spawn(move || {
                // A bounded wait (a hang guard): the hold is released in any
                // case. A removed gate (the test has ended) ends the releaser.
                let deadline = Instant::now() + Duration::from_secs(60);
                while pool.reserved(LeasePurpose::Worker) < 48 * MIB && Instant::now() < deadline && dir.is_dir() {
                    std::thread::sleep(Duration::from_millis(2));
                }
                if let Err(error) = std::fs::write(dir.join("hold-released"), b"") {
                    assert!(!dir.is_dir(), "{error}");
                }
            })
        };
        let run = run(&pool, &*launcher, BODY, 12 * GIB, reserve);
        releaser.join().unwrap();
        assert!(matches!(&run.outcome, WorkerOutcome::Completed { result, .. } if result == b"50331648"), "{:?}", run.outcome);
        assert!(run.grew_to_cover_a_footprint_above_the_grant(reserve, 12 * GIB), "{:?}", run.events);
        assert!(*run.grants.last().unwrap() >= 48 * MIB, "the lease covers the uncounted memory: {:?}", run.grants);
        assert_reaped(run.pid);
        drop(run);
        // Beyond the budget: found by a poll, killed, refused.
        let budget = reserve + 24 * MIB;
        let launcher = TestLauncher::scripted(format!("uncounted:{}:{}:50:5000", 8 * MIB, 64 * MIB));
        let run = super::tests::run(&pool, &*launcher, BODY, budget, reserve);
        // From the worker's start, which precedes its 5 s hold: the spawn
        // (serialized with other tests' spawns) is no part of the poll's promptness.
        let started = run.attached_at.expect("a worker was attached");
        match run.outcome {
            WorkerOutcome::Refused(ResourceRefusal::WorkerFootprintExceeded { grant_bytes, footprint_bytes }) => {
                assert!(grant_bytes >= start && grant_bytes <= budget);
                assert!(footprint_bytes > budget, "{footprint_bytes}");
            }
            other => panic!("{other:?}"),
        }
        assert!(started.elapsed() < Duration::from_secs(4), "killed by a poll, not at the end of its 5 s hold");
        assert_eq!(pool.reserved_total(), 0);
        assert_reaped(run.pid);
    }

    /// Review S1: a refused doubling step falls back to the minimal need (one
    /// more atomic admission); only if that is refused does the job end, and
    /// the refusal reports the minimal need.
    #[test]
    fn growth_falls_back_to_the_minimal_need_before_refusing() {
        let start = initial_grant(PROCESS_RESERVE_BYTES, crate::ingress::prescan(BODY).unwrap().lease_bytes().unwrap()).unwrap();
        // Headroom covers the starting grant and a few MiB more, not its doubling.
        let host = ScriptedHost::with(Some(16 * GIB), Some(start + 16 * MIB), PressureLevel::Normal);
        let pool = MemoryPool::with_wait_tick(host, Duration::from_millis(5));
        let launcher = TestLauncher::scripted(format!("alloc_steps:{}:{}", 2 * MIB, 2 * MIB));
        let run = run(&pool, &*launcher, BODY, 12 * GIB, PROCESS_RESERVE_BYTES);
        assert!(matches!(&run.outcome, WorkerOutcome::Completed { .. }), "{:?}", run.outcome);
        // The minimal need: the counted heap (> 2 MiB) plus the reserve, below the doubling.
        assert!(!run.grants.is_empty() && run.grants[0] < 2 * start && run.grants[0] > PROCESS_RESERVE_BYTES + 2 * MIB, "{:?}", run.grants);
        assert_reaped(run.pid);
        drop(run);
        // A pool with 1 MiB to spare: the doubling and the need are both refused.
        let pool = super::tests::pool();
        let ingress = crate::ingress::allocation_envelope(BODY.len() as u64).unwrap();
        let blocker = pool.acquire(12 * GIB - start - ingress - MIB, LeasePurpose::Custody).unwrap();
        let run = super::tests::run(&pool, &*launcher, BODY, 12 * GIB, PROCESS_RESERVE_BYTES);
        match run.outcome {
            WorkerOutcome::Refused(ResourceRefusal::WorkerGrowthRefused { granted_bytes, requested_bytes, cause }) => {
                assert_eq!(granted_bytes, start);
                assert!(requested_bytes > PROCESS_RESERVE_BYTES + 2 * MIB && requested_bytes < 2 * start, "the minimal need: {requested_bytes}");
                assert!(matches!(*cause, ResourceRefusal::PoolExhausted { .. }), "{cause:?}");
            }
            other => panic!("{other:?}"),
        }
        assert_reaped(run.pid);
        drop(blocker);
        assert_eq!(pool.reserved_total(), 0);
    }

    /// Review N1: a worker closes every fd it inherited except stdio and its
    /// control pair before reading the request. The only other fd it holds is
    /// the one it opens itself afterwards: its parent-death watchdog's kqueue
    /// (macOS).
    #[test]
    fn a_worker_holds_no_inherited_fds() {
        let pool = pool();
        let launcher = Arc::new(TestLauncher { inherited_fd: Some(9), ..(*TestLauncher::scripted("fds")).clone() });
        let run = run(&pool, &*launcher, BODY, 12 * GIB, PROCESS_RESERVE_BYTES);
        let WorkerOutcome::Completed { result, .. } = &run.outcome else { panic!("{:?}", run.outcome) };
        let report: serde_json::Value = serde_json::from_slice(result).unwrap();
        let watchdog = report["watchdog"].as_i64().unwrap() as i32;
        let mut open: Vec<i32> = serde_json::from_value(report["open"].clone()).unwrap();
        #[cfg(target_os = "macos")]
        assert!(open.contains(&watchdog), "the watchdog's kqueue: {report}");
        open.retain(|&fd| fd != watchdog);
        assert_eq!(open.len(), 5, "stdio and the control pair only: {open:?}");
        assert!(open.starts_with(&[0, 1, 2]) && !open.contains(&9), "{open:?}");
    }

    /// At the result frame the lifetime-maximum footprint is checked before
    /// anything is released: above the grant it is recorded; above the budget
    /// the result is refused.
    ///
    /// The excursion is one no poll or growth exchange sees: the supervisor's
    /// first poll is held in its hook, after sampling, until the worker has
    /// mapped, touched and unmapped the memory. Ungated, a poll landing during
    /// the touch refused the second run with a part-touched sample (hosted
    /// macos-26-arm64: 28148992 bytes), and one landing before the unmap
    /// refused it with no result check made.
    #[cfg(target_os = "macos")]
    #[test]
    fn the_lifetime_peak_footprint_is_checked_at_the_result() {
        let pool = pool();
        let reserve = 16 * MIB;
        let start = initial_grant(reserve, crate::ingress::prescan(BODY).unwrap().lease_bytes().unwrap()).unwrap();
        let gated = |max_grant: u64| {
            let gate = Gate::new("excursion");
            let launcher = TestLauncher::scripted(format!("transient_mmap:{}:{}", 32 * MIB, gate.dir.display()));
            run_gated(&pool, &*launcher, BODY, max_grant, reserve, Some(gate.dir.clone()))
        };
        // Every observation but the result's lifetime peak preceded or followed the excursion.
        let unseen_until_the_result = |observed: &[(u64, u64)]| {
            observed.len() >= 2 && observed[..observed.len() - 1].iter().all(|&(_, footprint)| footprint < 32 * MIB)
        };
        let run = gated(12 * GIB);
        assert!(matches!(run.outcome, WorkerOutcome::Completed { .. }), "{:?}", run.outcome);
        let (grant, peak) = *run.observed.last().unwrap();
        assert!(peak > 32 * MIB, "{peak}");
        assert!(unseen_until_the_result(&run.observed), "{:?}", run.observed);
        if run.grants.is_empty() {
            assert!(grant == start && peak > grant, "recorded above the grant: {peak}");
        }
        drop(run);
        // The budget equals the starting grant: the peak exceeded it, so the result is refused.
        let run = gated(start);
        match run.outcome {
            WorkerOutcome::Refused(ResourceRefusal::WorkerFootprintExceeded { grant_bytes, footprint_bytes }) => {
                assert_eq!(grant_bytes, start);
                assert!(footprint_bytes > 32 * MIB, "{footprint_bytes}");
                assert_eq!(run.observed.last(), Some(&(start, footprint_bytes)), "the refusal is the last observation");
                assert!(unseen_until_the_result(&run.observed), "{:?}", run.observed);
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

    /// Larger per-invocation budgets (owner policy: any budget up to the
    /// pool) keep the worker's grant arithmetic, request and control frames,
    /// lease growth and envelopes exact, checked and overflow-free at 16, 48
    /// and 96 GiB, 1 TiB and near u64::MAX. Pure arithmetic and pool
    /// bookkeeping on an injected host: nothing is allocated toward these sizes.
    #[test]
    fn large_budgets_keep_grant_frame_lease_and_envelope_arithmetic_exact() {
        use crate::ingress::{allocation_envelope, IngressBound};
        use crate::worker_alloc::growth_request;
        const TIB: u64 = 1 << 40;
        let reserve = PROCESS_RESERVE_BYTES;
        let largest_whole_mib = u64::MAX / GRANT_GRANULE_BYTES * GRANT_GRANULE_BYTES;
        for budget in [16 * GIB, 48 * GIB, 96 * GIB, TIB, largest_whole_mib, u64::MAX] {
            let whole_mib = budget % GRANT_GRANULE_BYTES == 0;
            // Starting grants: reserve + bound rounded up to whole MiB, or None.
            assert_eq!(initial_grant(reserve, budget - reserve), whole_mib.then_some(budget), "{budget}");
            let above = initial_grant(reserve, budget - reserve + 1);
            assert_eq!(above, budget.checked_add(GRANT_GRANULE_BYTES).filter(|_| whole_mib), "{budget}");
            assert_eq!(initial_grant(u64::MAX, 1), None);
            assert_eq!(initial_grant(reserve, u64::MAX - reserve + 1), None);

            // The request header round-trips every field exactly.
            for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
                let request = Request { mode, initial_grant: budget / 2, max_grant: budget, reserve, raw_len: budget - reserve };
                assert_eq!(decode_request(&encode_request(&request)), Some(request));
                let at_budget = Request { initial_grant: budget, ..request };
                assert_eq!(decode_request(&encode_request(&at_budget)), Some(at_budget));
                let invalid = Request { initial_grant: budget, max_grant: budget - 1, ..request };
                assert_eq!(decode_request(&encode_request(&invalid)), None);
            }
            // Control frames carry every u64 exactly.
            for (tag, a, b, c) in [
                (TAG_GROW, budget, budget - 1, u64::MAX),
                (TAG_GRANTED, budget, 0, 0),
                (TAG_CAP_EXCEEDED, budget.saturating_add(1), budget, 0),
                (TAG_RESULT, budget - reserve, budget, 0),
            ] {
                assert_eq!(decode_frame(&encode_frame(tag, a, b, c)), (tag, a, b, c));
            }

            // The child's growth request: needed = counted + reserve, the
            // request capped at the budget, saturating at u64::MAX.
            assert_eq!(growth_request(budget - reserve, reserve, budget / 2, budget, 0), (budget, budget));
            assert_eq!(growth_request(budget / 4, reserve, budget / 2 + 1, budget, 0), (budget, budget / 4 + reserve));
            assert_eq!(growth_request(1, reserve, u64::MAX, budget, u64::MAX), (budget, 1 + reserve));
            assert_eq!(growth_request(u64::MAX, reserve, 0, budget, 0), (budget, u64::MAX));
            let (requested, needed) = growth_request(budget - reserve + 1, reserve, 0, budget, 0);
            assert_eq!(needed, budget.saturating_add(1), "a need above the budget is visible (refused CAPX)");
            assert!(requested <= budget);
            // The parent's poll and exchange growth, capped at the budget.
            assert_eq!(poll_growth(budget / 2, budget - 1, reserve, budget), (budget, budget));
            assert_eq!(poll_growth(budget / 8, budget / 8 + 1, reserve, budget), (budget / 8 * 2, budget / 8 + 1 + reserve));
            assert_eq!(poll_growth(u64::MAX, u64::MAX, reserve, budget), (budget, budget));
            assert_eq!(exchange_growth(budget, budget - 1, u64::MAX, reserve, budget), (budget, budget));
            assert_eq!(exchange_growth(budget / 2, budget / 4, 0, reserve, budget), (budget / 2, (budget / 4).max(reserve)));

            // Lease admission and growth on an injected host of u64::MAX bytes.
            let host = ScriptedHost::with(Some(u64::MAX), Some(u64::MAX), PressureLevel::Normal);
            let pool = MemoryPool::with_wait_tick(host, Duration::from_millis(5));
            pool.configure(u64::MAX, budget, true).unwrap();
            assert_eq!(pool.settings(), (u64::MAX, budget));
            let start = initial_grant(reserve, 4096).unwrap();
            let mut grant = pool.acquire(start, LeasePurpose::Worker).unwrap();
            assert_eq!(grow_grant(&mut grant, budget, budget / 2, None, &mut || false), Ok(budget));
            assert_eq!((grant.bytes(), pool.reserved(LeasePurpose::Worker)), (budget, budget));
            assert_eq!(grow_grant(&mut grant, budget, budget - 1, None, &mut || false), Ok(budget), "a need within the lease is no growth");
            let extra = pool.acquire(u64::MAX - budget, LeasePurpose::Result).unwrap();
            assert_eq!(pool.reserved_total(), u64::MAX);
            assert!(matches!(pool.acquire(1, LeasePurpose::Result).unwrap_err(), ResourceRefusal::PoolExhausted { .. }));
            assert!(matches!(pool.acquire_interactive(1).unwrap_err(), ResourceRefusal::PoolExhausted { .. }));
            drop((grant, extra));
            assert_eq!(pool.reserved_total(), 0);
            pool.configure(budget / 2, budget / 2, true).unwrap();
            assert_eq!(
                pool.configure(budget / 2, budget, true).unwrap_err(),
                ResourceRefusal::BudgetExceedsPool { budget_bytes: budget, pool_bytes: budget / 2 }
            );

            // Envelopes: exact for page multiples, checked (None) or saturating above.
            if budget % (16 * 1024) == 0 {
                assert_eq!(resident_envelope(budget), budget);
            }
            assert!(resident_envelope(budget) >= budget);
            assert_eq!(allocation_envelope(budget), budget.checked_next_power_of_two());
            let bound = IngressBound { raw_len: budget - reserve, materialization_bytes: budget };
            assert_eq!(
                bound.lease_bytes(),
                allocation_envelope(budget - reserve).and_then(|raw| raw.checked_add(budget))
            );
            assert_eq!(
                crate::command_ingress::lease_bytes(bound, crate::command_ingress::ArgsDecoding::Moved),
                bound.lease_bytes().and_then(|l| l.checked_add(crate::command_ingress::ARGS_ERROR_ALLOWANCE_BYTES))
            );
        }
        assert_eq!(resident_envelope(u64::MAX), u64::MAX, "saturates");
        assert_eq!(allocation_envelope(u64::MAX), None);
    }

    // --- growth steps that re-sample (owner, 2026-10-26; Agent 0, option (a)) ---

    use crate::memory_pool::{HostMemory, HostSample, JobCounts};
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering::SeqCst};

    /// A scripted host that counts the samples it reports with zero headroom,
    /// so a test acts once a growth step has been refused and has re-sampled,
    /// not after a fixed sleep.
    struct DipHost {
        script: Arc<ScriptedHost>,
        dry_samples: AtomicUsize,
    }

    impl DipHost {
        fn new() -> Arc<Self> {
            Arc::new(Self { script: ScriptedHost::normal(), dry_samples: AtomicUsize::new(0) })
        }
        fn dry(&self) {
            self.script.set(|s| s.headroom_bytes = Some(0));
        }
        fn refill(&self) {
            self.script.set(|s| s.headroom_bytes = Some(64 * GIB));
        }
        fn dry_samples(&self) -> usize {
            self.dry_samples.load(SeqCst)
        }
    }

    impl HostMemory for DipHost {
        fn sample(&self) -> HostSample {
            let sample = self.script.sample();
            if sample.headroom_bytes == Some(0) {
                self.dry_samples.fetch_add(1, SeqCst);
            }
            sample
        }
    }

    /// A pool on `host` whose growth steps re-sample every 5 ms for `window`.
    fn dip_pool(host: &Arc<DipHost>, window: Duration) -> Arc<MemoryPool> {
        MemoryPool::with_timing(host.clone(), Duration::from_millis(5), window, Duration::from_millis(5))
    }

    fn start_grant(reserve: u64) -> u64 {
        initial_grant(reserve, crate::ingress::prescan(BODY).unwrap().lease_bytes().unwrap()).unwrap()
    }

    fn wait_for<T>(what: &str, mut ready: impl FnMut() -> Option<T>) -> T {
        let deadline = Instant::now() + Duration::from_secs(60);
        loop {
            if let Some(value) = ready() {
                return value;
            }
            assert!(Instant::now() < deadline, "{what}: not within 60 s");
            std::thread::sleep(Duration::from_millis(2));
        }
    }

    /// A worker job supervised on another thread. Dropped unjoined (a failing
    /// test), it cancels and kills the worker and joins, so no worker,
    /// stopped or not, outlives the test.
    struct Background {
        handle: Option<std::thread::JoinHandle<Run>>,
        control: SharedControl,
        cancelled: Arc<AtomicBool>,
    }

    fn run_in_background(pool: &Arc<MemoryPool>, launcher: Arc<TestLauncher>, reserve: u64) -> Background {
        let (control, cancelled) = (SharedControl::default(), Arc::<AtomicBool>::default());
        let (pool, shared, flag) = (pool.clone(), control.clone(), cancelled.clone());
        let handle = std::thread::spawn(move || run_hooked(&pool, &*launcher, BODY, 12 * GIB, reserve, None, flag, shared));
        Background { handle: Some(handle), control, cancelled }
    }

    impl Background {
        fn control(&self) -> Arc<WorkerControl> {
            wait_for("the worker attached", || self.control.lock().unwrap().clone())
        }
        fn join(mut self) -> Run {
            self.handle.take().unwrap().join().unwrap()
        }
    }

    impl Drop for Background {
        fn drop(&mut self) {
            if let Some(handle) = self.handle.take() {
                self.cancelled.store(true, SeqCst);
                if let Some(control) = self.control.lock().unwrap_or_else(PoisonError::into_inner).clone() {
                    control.kill();
                }
                let _ = handle.join();
            }
        }
    }

    /// The BSD process status (`SSTOP`, `SZOMB`, ...) of `pid`, if it exists.
    #[cfg(target_os = "macos")]
    fn process_status(pid: i32) -> Option<u32> {
        let mut info = std::mem::MaybeUninit::<libc::proc_bsdinfo>::zeroed();
        let size = std::mem::size_of::<libc::proc_bsdinfo>() as libc::c_int;
        // SAFETY: `info` is a writable proc_bsdinfo of `size` bytes, the struct PROC_PIDTBSDINFO fills.
        let filled = unsafe { libc::proc_pidinfo(pid, libc::PROC_PIDTBSDINFO, 0, info.as_mut_ptr().cast(), size) };
        // SAFETY: filled on success.
        (filled == size).then(|| unsafe { info.assume_init() }.pbi_status)
    }

    #[cfg(target_os = "macos")]
    fn is_stopped(pid: i32) -> bool {
        process_status(pid) == Some(libc::SSTOP)
    }

    /// Owner ruling 2026-10-26: a growth exchange refused only with
    /// `headroom_insufficient` re-samples while the child waits, blocked, for
    /// its reply, and is admitted at the first fresh sample that passes.
    #[test]
    fn an_exchange_growth_step_waits_out_a_headroom_dip_and_is_admitted() {
        let host = DipHost::new();
        let pool = dip_pool(&host, Duration::from_secs(30));
        let gate = Gate::new("exchange-dip");
        let reserve = PROCESS_RESERVE_BYTES;
        let start = start_grant(reserve);
        let launcher = TestLauncher::scripted(format!("alloc_gate:0:{}:{}", 8 * MIB, gate.dir.display()));
        let background = run_in_background(&pool, launcher, reserve);
        wait_for("the worker started", || (gate.started_count() > 0).then_some(()));
        host.dry();
        std::fs::write(gate.dir.join("release-alloc"), b"").unwrap();
        // Refused and re-sampled: the step is waiting, and has admitted nothing.
        wait_for("the step re-sampled", || (host.dry_samples() >= 2).then_some(()));
        assert_eq!(pool.reserved(LeasePurpose::Worker), start);
        host.refill();
        let run = background.join();
        assert!(matches!(&run.outcome, WorkerOutcome::Completed { result, .. } if result == br#"{"held":0,"second":8388608}"#), "{:?}", run.outcome);
        assert!(!run.grants.is_empty() && run.grants[0] > start + 8 * MIB, "{:?}", run.grants);
        assert_eq!(pool.status(JobCounts::default()).last_refusal, None, "a passing re-sample leaves no refusal");
        let pid = run.pid;
        drop(run);
        assert_eq!(pool.reserved_total(), 0, "the result lease was the last reservation");
        assert_reaped(pid);
    }

    /// A growth exchange still refused with `headroom_insufficient` when the
    /// window has elapsed ends the job `worker_growth_refused`, with the
    /// lease before the step and the minimal need, as a single-sample refusal did.
    #[test]
    fn an_exchange_growth_step_refused_for_the_whole_window_ends_worker_growth_refused() {
        let host = DipHost::new();
        let window = Duration::from_millis(300);
        let pool = dip_pool(&host, window);
        let gate = Gate::new("exchange-refused");
        let reserve = PROCESS_RESERVE_BYTES;
        let start = start_grant(reserve);
        let launcher = TestLauncher::scripted(format!("alloc_gate:0:{}:{}", 8 * MIB, gate.dir.display()));
        let background = run_in_background(&pool, launcher, reserve);
        wait_for("the worker started", || (gate.started_count() > 0).then_some(()));
        host.dry();
        let released = Instant::now();
        std::fs::write(gate.dir.join("release-alloc"), b"").unwrap();
        let run = background.join();
        let waited = released.elapsed();
        match run.outcome {
            WorkerOutcome::Refused(ResourceRefusal::WorkerGrowthRefused { granted_bytes, requested_bytes, cause }) => {
                assert_eq!(granted_bytes, start);
                assert!(requested_bytes > reserve + 8 * MIB && requested_bytes < 2 * start, "the minimal need: {requested_bytes}");
                assert!(matches!(*cause, ResourceRefusal::HeadroomInsufficient { available_bytes: 0, .. }), "{cause:?}");
                assert_eq!(pool.status(JobCounts::default()).last_refusal, Some(*cause), "the refusal that stands is recorded");
            }
            other => panic!("{other:?}"),
        }
        assert!(waited >= window, "re-sampled for the whole window: {waited:?}");
        assert!(host.dry_samples() >= 2);
        assert!(run.grants.is_empty(), "{:?}", run.grants);
        assert_eq!(pool.reserved_total(), 0);
        assert_reaped(run.pid);
    }

    /// Agent 0, option (a): a poll that finds the footprint above the lease,
    /// with the first attempt refused only with `headroom_insufficient`, stops
    /// the running child for the re-sampling wait and continues it once a
    /// fresh sample admits the step; the child then finishes its work.
    #[cfg(target_os = "macos")]
    #[test]
    fn a_poll_growth_step_stops_the_child_waits_out_a_dip_and_continues_it() {
        let host = DipHost::new();
        let pool = dip_pool(&host, Duration::from_secs(30));
        let gate = Gate::new("poll-dip");
        let reserve = PROCESS_RESERVE_BYTES;
        let start = start_grant(reserve);
        let mapped = 96 * MIB;
        let launcher = TestLauncher::scripted(format!("map_go:{mapped}:{}", gate.dir.display()));
        let background = run_in_background(&pool, launcher, reserve);
        let pid = wait_for("the worker started", || gate.pids().first().copied());
        host.dry();
        std::fs::write(gate.dir.join("go"), b"").unwrap();
        wait_for("the child stopped for the step", || (is_stopped(pid) && host.dry_samples() >= 2).then_some(()));
        std::thread::sleep(Duration::from_millis(100));
        assert!(is_stopped(pid), "stopped for the whole wait: {:?}", process_status(pid));
        assert_eq!(pool.reserved(LeasePurpose::Worker), start, "nothing admitted during the dip");
        // Released while stopped: the child can act on it only once continued.
        std::fs::write(gate.dir.join("hold-released"), b"").unwrap();
        std::thread::sleep(Duration::from_millis(50));
        assert!(is_stopped(pid));
        host.refill();
        let run = background.join();
        assert_eq!(run.pid, Some(pid));
        let expected = mapped.to_string();
        assert!(matches!(&run.outcome, WorkerOutcome::Completed { result, .. } if result == expected.as_bytes()), "continued and completed: {:?}", run.outcome);
        let first_grant = run.events.iter().position(|e| matches!(e, Event::Grant(_))).expect("the step was admitted");
        assert!(
            run.events[..first_grant].iter().any(|e| matches!(e, Event::Footprint { grant, footprint } if footprint > grant)),
            "a poll found the footprint above the lease first: {:?}",
            run.events
        );
        assert!(*run.grants.last().unwrap() >= mapped, "the lease covers the mapping: {:?}", run.grants);
        assert_eq!(pool.status(JobCounts::default()).last_refusal, None, "a passing re-sample leaves no refusal");
        let pid = run.pid;
        drop(run);
        assert_eq!(pool.reserved_total(), 0, "the result lease was the last reservation");
        assert_reaped(pid);
    }

    /// A poll step still refused when the window has elapsed ends the job as
    /// a single-sample refusal did: `worker_growth_refused`, the child (stopped
    /// for the wait) killed and reaped.
    #[cfg(target_os = "macos")]
    #[test]
    fn a_poll_growth_step_refused_for_the_whole_window_kills_the_stopped_child() {
        let host = DipHost::new();
        let window = Duration::from_secs(1);
        let pool = dip_pool(&host, window);
        let gate = Gate::new("poll-refused");
        let reserve = PROCESS_RESERVE_BYTES;
        let start = start_grant(reserve);
        let mapped = 96 * MIB;
        let launcher = TestLauncher::scripted(format!("map_go:{mapped}:{}", gate.dir.display()));
        let background = run_in_background(&pool, launcher, reserve);
        let pid = wait_for("the worker started", || gate.pids().first().copied());
        host.dry();
        let go = Instant::now();
        std::fs::write(gate.dir.join("go"), b"").unwrap();
        wait_for("the child stopped for the step", || is_stopped(pid).then_some(()));
        let run = background.join();
        let waited = go.elapsed();
        match run.outcome {
            WorkerOutcome::Refused(ResourceRefusal::WorkerGrowthRefused { granted_bytes, requested_bytes, cause }) => {
                assert_eq!(granted_bytes, start);
                // The minimal need: the polled footprint (above the lease) plus the reserve.
                assert!(requested_bytes > start + reserve && requested_bytes <= mapped + 2 * reserve, "{requested_bytes}");
                assert!(matches!(*cause, ResourceRefusal::HeadroomInsufficient { available_bytes: 0, .. }), "{cause:?}");
                assert_eq!(pool.status(JobCounts::default()).last_refusal, Some(*cause));
            }
            other => panic!("{other:?}"),
        }
        assert!(waited >= window, "re-sampled for the whole window: {waited:?}");
        assert!(run.grants.is_empty(), "{:?}", run.grants);
        assert_eq!(pool.reserved_total(), 0);
        assert_reaped(run.pid);
    }

    /// Cancellation (and session reset, which requests it the same way)
    /// during a stopped wait: the stopped child is killed and reaped, the wait
    /// ends at once rather than at the window, the job ends cancelled
    /// (`Terminated`) and the lease is released after the reap.
    #[cfg(target_os = "macos")]
    #[test]
    fn cancellation_during_a_stopped_wait_kills_and_reaps_the_child() {
        let host = DipHost::new();
        let pool = dip_pool(&host, Duration::from_secs(30));
        let gate = Gate::new("poll-cancel");
        let launcher = TestLauncher::scripted(format!("map_go:{}:{}", 96 * MIB, gate.dir.display()));
        let background = run_in_background(&pool, launcher, PROCESS_RESERVE_BYTES);
        let pid = wait_for("the worker started", || gate.pids().first().copied());
        let control = background.control();
        host.dry();
        std::fs::write(gate.dir.join("go"), b"").unwrap();
        wait_for("the child stopped for the step", || (is_stopped(pid) && host.dry_samples() >= 2).then_some(()));
        // As `SolveJobRegistry::cancel` does: request, kill, wake the waiters.
        let cancelled = Instant::now();
        background.cancelled.store(true, SeqCst);
        control.kill();
        pool.wake_waiters();
        let run = background.join();
        assert!(cancelled.elapsed() < Duration::from_secs(5), "the wait ended promptly: {:?}", cancelled.elapsed());
        assert!(matches!(run.outcome, WorkerOutcome::Terminated), "{:?}", run.outcome);
        assert_reaped(Some(pid));
        assert!(run.grants.is_empty());
        assert_eq!(pool.status(JobCounts::default()).last_refusal, None, "a cancelled step records no refusal");
        assert_eq!(pool.reserved_total(), 0, "released after the reap");
        // Reaped: no stop or continue signals it any more.
        assert!(!control.stop());
        control.resume();
    }

    pub(crate) const PARENT_ENV: &str = "SWBPIPE_TEST_WORKER_PARENT";
    /// Set to `1`, the intermediate parent stops its worker before reporting it.
    const PARENT_STOPS_ENV: &str = "SWBPIPE_TEST_WORKER_PARENT_STOPS";
    const PARENT_ENTRY: &str = "solve_worker::tests::stopped_worker_parent_entry";

    /// The parent-death tests' intermediate parent, inert unless started as
    /// one. With SIGHUP ignored, as `nohup` leaves it and as a worker would
    /// inherit it through exec, it spawns a sleeping worker (a workload that
    /// never touches its channel) through the product's spawn path, stops it
    /// as a poll step does if asked to ([`PARENT_STOPS_ENV`]), reports its pid
    /// and holds it unreaped until killed (at most 60 s).
    #[test]
    fn stopped_worker_parent_entry() {
        let Some(dir) = std::env::var_os(PARENT_ENV).map(std::path::PathBuf::from) else { return };
        // SAFETY: setting a disposition has no preconditions.
        unsafe {
            libc::signal(libc::SIGHUP, libc::SIG_IGN);
        }
        let launcher = TestLauncher::scripted("sleep");
        let mut process = WorkerProcess::spawn(&*launcher).unwrap();
        let request = Request {
            mode: PreviewSolverMode::SparseInteractive,
            initial_grant: PROCESS_RESERVE_BYTES + MIB,
            max_grant: 12 * GIB,
            reserve: PROCESS_RESERVE_BYTES,
            raw_len: BODY.len() as u64,
        };
        let stdin = process.stdin.as_mut().unwrap();
        stdin.write_all(&encode_request(&request)).and_then(|()| stdin.write_all(BODY)).and_then(|()| stdin.flush()).unwrap();
        std::thread::sleep(Duration::from_millis(200));
        if std::env::var_os(PARENT_STOPS_ENV).is_some_and(|v| v == "1") {
            assert!(process.control.stop());
        }
        std::fs::write(dir.join("staging"), process.control.pid.to_string()).unwrap();
        std::fs::rename(dir.join("staging"), dir.join("worker-pid")).unwrap();
        std::thread::sleep(Duration::from_secs(60));
        std::process::exit(0);
    }

    /// Kills the intermediate and, if it still exists, the worker.
    #[cfg(target_os = "macos")]
    struct OrphanGuard {
        parent: std::process::Child,
        worker: Option<i32>,
    }

    #[cfg(target_os = "macos")]
    impl Drop for OrphanGuard {
        fn drop(&mut self) {
            let _ = self.parent.kill();
            let _ = self.parent.wait();
            if let Some(worker) = self.worker.filter(|&worker| process_status(worker).is_some()) {
                // SAFETY: the test's own worker, seen alive just now.
                unsafe {
                    libc::kill(worker, libc::SIGKILL);
                }
            }
        }
    }

    /// Spawns the intermediate parent with a worker, stopped or running, and
    /// kills the intermediate; the worker must be gone within `limit`.
    #[cfg(target_os = "macos")]
    fn worker_ends_when_its_parent_dies(stopped: bool, limit: Duration) {
        let gate = Gate::new("orphan");
        let parent = {
            let _serial = SPAWN_LOCK.lock().unwrap_or_else(PoisonError::into_inner);
            Command::new(std::env::current_exe().unwrap())
                .args([PARENT_ENTRY, "--exact", "--test-threads=1", "--quiet"])
                .env(PARENT_ENV, &gate.dir)
                .env(PARENT_STOPS_ENV, if stopped { "1" } else { "0" })
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .unwrap()
        };
        let mut guard = OrphanGuard { parent, worker: None };
        let pid_file = gate.dir.join("worker-pid");
        let worker: i32 = wait_for("the intermediate reported its worker", || {
            if let Ok(Some(status)) = guard.parent.try_wait() {
                panic!("the intermediate exited first: {status}");
            }
            std::fs::read_to_string(&pid_file).ok()?.parse().ok()
        });
        guard.worker = Some(worker);
        if stopped {
            wait_for("the worker stopped", || is_stopped(worker).then_some(()));
        } else {
            let status = process_status(worker);
            assert!(status.is_some_and(|status| status != libc::SSTOP && status != libc::SZOMB), "a running worker: {status:?}");
        }
        // SAFETY: getpgid only reads.
        let (worker_group, parent_group) = unsafe { (libc::getpgid(worker), libc::getpgid(guard.parent.id() as i32)) };
        assert_eq!(worker_group, worker, "the worker leads its own process group");
        assert_ne!(worker_group, parent_group);
        guard.parent.kill().unwrap();
        guard.parent.wait().unwrap();
        let died = Instant::now();
        while let Some(status) = process_status(worker).filter(|&status| status != libc::SZOMB) {
            assert!(died.elapsed() < limit, "worker {worker} remains {:?} after its parent died (status {status})", died.elapsed());
            std::thread::sleep(Duration::from_millis(10));
        }
        guard.worker = None;
    }

    /// Agent 0's parent-death condition: the worker runs in its own process
    /// group, so when its parent dies while it is stopped, the group is
    /// orphaned and the kernel sends it SIGHUP and SIGCONT, which end it, even
    /// though the parent had SIGHUP ignored (the worker restores the default).
    /// (Once continued, the watchdog would end it too.)
    #[cfg(target_os = "macos")]
    #[test]
    fn a_stopped_worker_ends_when_its_parent_dies() {
        worker_ends_when_its_parent_dies(true, Duration::from_secs(5));
    }

    /// The manager's watchdog decision: a running worker, in a workload that
    /// never touches its channel and outside the app's process group, ends
    /// when its parent dies (kqueue `NOTE_EXIT` on the parent), within 2 s.
    #[cfg(target_os = "macos")]
    #[test]
    fn a_running_worker_ends_when_its_parent_dies() {
        worker_ends_when_its_parent_dies(false, Duration::from_secs(2));
    }
}
