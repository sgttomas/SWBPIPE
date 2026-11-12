//! `release_timing --supervise PLAN --out DIR [--models DIR] [--write-results]`: the driver
//! supervises its own runs, with no Python, git, cargo or other tool outside the stock system.
//!
//! It is the Python runner's `--run` and `--packet` (`core/solver/performance_harness/runner/
//! release_runner.py` `run_plan`, `packet`, over `k6_runner.py` `launch` and `_watch`), ported:
//! the plan's runs in order, one at a time, within the window budget. Each run re-executes this
//! binary (`std::env::current_exe`) as a child in the driver mode, in a new session and process
//! group, and this process watches it from outside: every 100 ms one `proc_pid_rusage(pid,
//! RUSAGE_INFO_V4)` call gives the child's phys_footprint, lifetime-maximum phys_footprint and
//! resident size, and the whole process group is killed with SIGKILL when the footprint or its
//! lifetime maximum passes the run's footprint budget, when the resident size passes the
//! emergency cap, or at the run's timeout. The exit status and `ru_maxrss` come from `wait4`.
//! Records and the packet have the Python's schema names and keys; where a Python source does
//! not exist here (`/usr/bin/time`, `ps`) the key is kept with null and a named sibling carries
//! this supervisor's reading. Observation only: nothing here asserts a time or memory bound.
//!
//! A run's output files are opened before its driver is spawned, without following a symbolic
//! link and without blocking ([`open_output`]): a destination that is not a regular file, or a new
//! file in a real directory, refuses the run before any process starts. From the spawn on, one
//! owner ([`Spawned`]) holds the driver: every return, error and unwind path kills its process
//! group and reaps it. Nothing between the spawn and the watch loop touches the file system.

use crate::classify::{self, int, jsonl, py_str, sanitize, truthy};
use crate::host;
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicI32, Ordering};
use std::time::{Duration, Instant};

pub const PLAN_SCHEMA: &str = "release-timing-plan-v2";
pub const RECORD_SCHEMA: &str = "release-runner-record-v1";
pub const PACKET_SCHEMA: &str = "release-timing-packet-v1";
pub const DRIVER_SCHEMA: &str = "release-timing-v2";
/// The launch record (`<run_id>.record.json`); the Python's is `k6-runner-record-v1`, written by
/// `k6_runner.launch`, which this supervisor does not use.
pub const LAUNCH_SCHEMA: &str = "release-timing-supervise-launch-v1";
pub const RUNNER: &str = "release_timing --supervise";
const GIB: f64 = 1024.0 * 1024.0 * 1024.0;
/// `rr.BASE_MACHINE_BUDGET_BYTES`: memory_pool.rs DEFAULT_INVOCATION_BUDGET_BYTES.
pub const BASE_MACHINE_BUDGET_BYTES: u64 = 12_884_901_888;
/// `rr.EMERGENCY_RSS_CAP_BYTES`: the base machine's physical memory.
pub const EMERGENCY_RSS_CAP_BYTES: u64 = 16 * 1024 * 1024 * 1024;
/// `k6.POLL_S`.
pub const POLL: Duration = Duration::from_millis(100);
const POLL_S: f64 = 0.1;
/// `k6.MEMORYSTATUS_FLOOR`, `k6.LOAD_HIGH`, `k6.LOAD_RESUME`.
const MEMORYSTATUS_FLOOR: i64 = 80;
const LOAD_HIGH: f64 = 8.0;
const LOAD_RESUME: f64 = 6.0;
const QUIET_STEP_S: u64 = 30;
/// `k6.launch`: `stderr_text[-1500:]`.
const STDERR_TAIL_CHARS: usize = 1500;

/// `rr.CONDITIONS`, restated for this supervisor: the same terms where the method is the same,
/// and what differs said where it differs.
pub const CONDITIONS: [&str; 8] = [
    "Engine stages only. A solve run: the release_timing driver runs, in one process, the stages the desktop solve worker runs for an ordinary solve (read, parse, request, entry, serialize, release_envelope, parse_back) and nothing else; parse_back is a cheaper stand-in for the worker's read-back of the result text, so the driver's time and memory from serialize on are lower bounds on the worker's, not a measured worker peak. A model-checks run (task model_checks, run id model-checks-...) is a process of its own: the stages the desktop's check worker runs for the on-demand model checks (read, parse, model_checks, serialize). The desktop app, its IPC and ingress, the worker process protocol and memory pool, the solve attestation digests and the webview are not included; the driver lists its own limits in each run.",
    "One driver process at a time, each with a per-run timeout. The release_timing binary supervises itself (release_timing --supervise): it re-executes itself as the driver in a new session and process group and watches that child from outside, with no /usr/bin/time and no ps. Every 100 ms it reads the driver phys_footprint, lifetime-maximum phys_footprint and resident size (one proc_pid_rusage RUSAGE_INFO_V4 call); any limit kills the whole process group with SIGKILL. When the driver has exited, before reaping it, the supervisor reads its final lifetime-maximum phys_footprint (the counter /usr/bin/time -l reports as peak memory footprint), then takes its exit status and ru_maxrss from wait4.",
    "The footprint budget is the product per-invocation memory budget on the 16 GiB base machine (12 GiB, memory_pool.rs DEFAULT_INVOCATION_BUDGET_BYTES), enforced as the product enforces it, on phys_footprint: a run whose sampled footprint or lifetime-maximum footprint exceeds it is killed and classified exceeds_base_machine_budget (the desktop worker_footprint_exceeded). Like the product, the sampling can miss a peak shorter than one poll; a completed run whose lifetime-maximum footprint (sampled, the driver reading, or the supervisor reading at exit) exceeds the budget is classified the same way, because the desktop would refuse that result. The rules are the Python runner's; this supervisor also has the driver's reading at its exit on a killed run, where the Python runner's /usr/bin/time is killed with the group, and counts it, so the two can class a boundary case differently. Each record names the readings its classification used (classification_readings).",
    "The resident-size watchdog (proc_pid_rusage ri_resident_size, read where the Python runner reads ps RSS) is an emergency cap only (16 GiB by default, the base machine physical memory). Resident size can run well above phys_footprint (about 1.4 times at 500 linear-ex nodes), so it is not the product measure; a kill there is killed_by_rss_cap unless a footprint reading shows the budget exceeded.",
    "The host is the one described in metadata (memory_bytes, cpu_model); it is not necessarily the 16 GiB base machine, so memory pressure, paging and compression on a 16 GiB machine are not reproduced.",
    "Models are invented rack models from rack_model.py, identified per run by sha256 and checked against the plan sha256 where the plan gives one.",
    "This supervisor takes no stack snapshots: a plan with attribution runs is refused.",
    "Observation only: no time or memory threshold is asserted, and criterion 9 provisional bounds are not evaluated.",
];

// ------------------------------------------------------------------ interruption

static INTERRUPTED: AtomicI32 = AtomicI32::new(0);

extern "C" fn on_signal(signal: libc::c_int) {
    INTERRUPTED.store(signal, Ordering::SeqCst);
}

/// SIGINT, SIGTERM and SIGHUP set a flag that the watch loop reads every poll: it kills the
/// running driver's group and the supervisor exits with 128 + the signal (the Python runner's
/// RV18-3 for SIGINT and SIGTERM; SIGHUP is added because the driver runs in its own session and
/// would outlive a closed terminal).
fn install_signal_handlers() {
    for signal in [libc::SIGINT, libc::SIGTERM, libc::SIGHUP] {
        // SAFETY: the handler only stores to an atomic, which is async-signal-safe.
        unsafe {
            let mut action: libc::sigaction = std::mem::zeroed();
            action.sa_sigaction = on_signal as extern "C" fn(libc::c_int) as usize;
            action.sa_flags = libc::SA_RESTART;
            libc::sigemptyset(&mut action.sa_mask);
            libc::sigaction(signal, &action, std::ptr::null_mut());
        }
    }
}

fn interrupted() -> Option<i32> {
    match INTERRUPTED.load(Ordering::SeqCst) {
        0 => None,
        signal => Some(signal),
    }
}

fn exit_interrupted(signal: i32) -> ! {
    eprintln!("release_timing --supervise: interrupted by signal {signal}; the running driver's group was killed");
    std::process::exit(128 + signal)
}

/// Sleeps `seconds`, waking every poll to honour an interruption.
fn sleep_interruptibly(seconds: u64) {
    let end = Instant::now() + Duration::from_secs(seconds);
    while Instant::now() < end {
        if let Some(signal) = interrupted() {
            exit_interrupted(signal);
        }
        std::thread::sleep(POLL.min(end.saturating_duration_since(Instant::now())));
    }
}

// ------------------------------------------------------------------ plan

/// One run of the plan file (`rr.plan_document`).
#[derive(Clone, Debug)]
pub struct Run {
    /// The Python plan entry's keys (`rr.PLAN_RUN_KEYS`), copied into the run's record.
    pub entry: Map<String, Value>,
    pub run_id: String,
    /// `solve` (the driver's solve invocation, with `--mode`) or `model_checks` (its
    /// `--model-checks` invocation; the plan's mode is not passed).
    pub task: String,
    pub mode: String,
    pub timeout_s: i64,
    pub estimate_s: i64,
    pub footprint_budget_bytes: u64,
    pub rss_cap_bytes: u64,
    pub expect: String,
    pub model: String,
    pub expected_model_sha256: Option<String>,
    pub model_stats: Value,
}

#[derive(Clone, Debug)]
pub struct Plan {
    pub window_budget_s: i64,
    pub quiet_wait_s: i64,
    pub runs: Vec<Run>,
}

pub const PLAN_RUN_KEYS: [&str; 12] = [
    "order",
    "run_id",
    "task",
    "profile",
    "nodes",
    "mode",
    "timeout_s",
    "footprint_budget_bytes",
    "rss_cap_bytes",
    "estimate_s",
    "expect",
    "attribute",
];

pub fn read_plan(text: &str) -> Result<Plan, String> {
    let document: Value = serde_json::from_str(text).map_err(|e| format!("the plan is not JSON: {e}"))?;
    if document.get("schema").and_then(Value::as_str) != Some(PLAN_SCHEMA) {
        return Err(format!("the plan's schema is not {PLAN_SCHEMA}"));
    }
    let integer = |v: &Value, key: &str| v.get(key).and_then(Value::as_i64).ok_or_else(|| format!("plan: {key} must be an integer"));
    let window_budget_s = integer(&document, "window_budget_s")?;
    let quiet_wait_s = integer(&document, "quiet_wait_s")?;
    let mut runs = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for run in document.get("runs").and_then(Value::as_array).ok_or("plan: runs must be a list")? {
        let mut entry = Map::new();
        for key in PLAN_RUN_KEYS {
            entry.insert(key.to_string(), run.get(key).cloned().ok_or_else(|| format!("plan: a run has no {key}"))?);
        }
        let text = |key: &str| run.get(key).and_then(Value::as_str).map(str::to_string).ok_or_else(|| format!("plan: {key} must be a string"));
        let unsigned = |key: &str| run.get(key).and_then(Value::as_u64).ok_or_else(|| format!("plan: {key} must be a non-negative integer"));
        let run_id = text("run_id")?;
        if run.get("attribute").and_then(Value::as_bool) != Some(false) {
            return Err(format!("plan: {run_id} is an attribution run; this supervisor takes no stack snapshots"));
        }
        let model = text("model")?;
        if model.contains('/') || model.contains('\\') || model.starts_with('.') {
            return Err(format!("plan: {run_id}'s model must be a file name in the models folder"));
        }
        let mode = text("mode")?;
        if !matches!(mode.as_str(), "sparse" | "dense") {
            return Err(format!("plan: {run_id}'s mode must be sparse or dense"));
        }
        let task = text("task")?;
        if !matches!(task.as_str(), "solve" | "model_checks") {
            return Err(format!("plan: {run_id}'s task must be solve or model_checks"));
        }
        if !seen.insert(run_id.clone()) {
            return Err(format!("plan: {run_id} appears twice"));
        }
        runs.push(Run {
            entry,
            task,
            mode,
            timeout_s: integer(run, "timeout_s")?,
            estimate_s: integer(run, "estimate_s")?,
            footprint_budget_bytes: unsigned("footprint_budget_bytes")?,
            rss_cap_bytes: unsigned("rss_cap_bytes")?,
            expect: text("expect")?,
            expected_model_sha256: run.get("expected_model_sha256").and_then(Value::as_str).map(str::to_string),
            model_stats: run.get("model_stats").cloned().unwrap_or(Value::Null),
            model,
            run_id,
        });
    }
    Ok(Plan { window_budget_s, quiet_wait_s, runs })
}

// ------------------------------------------------------------------ helpers

pub fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes).iter().map(|b| format!("{b:02x}")).collect()
}

fn sha256_file(path: &Path) -> std::io::Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut digest = Sha256::new();
    let mut block = vec![0u8; 1 << 20];
    loop {
        let n = file.read(&mut block)?;
        if n == 0 {
            break;
        }
        digest.update(&block[..n]);
    }
    Ok(digest.finalize().iter().map(|b| format!("{b:02x}")).collect())
}

/// Python's `round(x, n)` for the times recorded (half away from zero; Python rounds the binary
/// value half to even, which differs only on exact ties).
fn round_to(x: f64, digits: i32) -> f64 {
    let scale = 10f64.powi(digits);
    (x * scale).round() / scale
}

fn basename(path: &str) -> String {
    Path::new(path).file_name().map_or_else(|| path.to_string(), |n| n.to_string_lossy().to_string())
}

/// The last `n` characters of `text` (Python `text[-n:]`).
fn tail_chars(text: &str, n: usize) -> &str {
    let count = text.chars().count();
    if count <= n {
        return text;
    }
    let skip = text.char_indices().nth(count - n).map_or(0, |(i, _)| i);
    &text[skip..]
}

/// Opens `path` for writing, truncated or appended, created if absent, without following a
/// symbolic link (`O_NOFOLLOW`) and without blocking (`O_NONBLOCK`: opening a FIFO that has no
/// reader fails at once instead of waiting for one). Anything that is not a regular file once
/// open (a FIFO with a reader, a device) is refused, so a record destination is a regular file or
/// a new one; nothing it returns can block a write.
pub fn open_output(path: &Path, append: bool) -> std::io::Result<std::fs::File> {
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::io::FromRawFd;
    let name = std::ffi::CString::new(path.as_os_str().as_bytes())?;
    let flags = libc::O_WRONLY
        | libc::O_CREAT
        | libc::O_NOFOLLOW
        | libc::O_NONBLOCK
        | libc::O_CLOEXEC
        | if append { libc::O_APPEND } else { libc::O_TRUNC };
    // SAFETY: a NUL-terminated path; the mode argument is the one `O_CREAT` reads.
    let fd = unsafe { libc::open(name.as_ptr(), flags, 0o644 as libc::c_uint) };
    if fd < 0 {
        let error = std::io::Error::last_os_error();
        let file = basename(&path.to_string_lossy());
        return Err(std::io::Error::new(error.kind(), format!("cannot open the record {file}: {error}")));
    }
    // SAFETY: `fd` was just opened here and is owned by nothing else.
    let file = unsafe { std::fs::File::from_raw_fd(fd) };
    if !file.metadata()?.file_type().is_file() {
        let file = basename(&path.to_string_lossy());
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("the record {file} is not a regular file")));
    }
    Ok(file)
}

fn write_json(path: &Path, value: &Value) -> std::io::Result<()> {
    write_json_to(open_output(path, false)?, value)
}

fn write_json_to(mut file: std::fs::File, value: &Value) -> std::io::Result<()> {
    let mut text = serde_json::to_string_pretty(value).expect("JSON");
    text.push('\n');
    file.write_all(text.as_bytes())
}

/// `k6.append_record`.
fn append_record(out_dir: &Path, record: &Value) -> std::io::Result<()> {
    let mut file = open_output(&out_dir.join("records.jsonl"), true)?;
    writeln!(file, "{record}")
}

/// The output folder is a real directory (not a symbolic link to one): a new record file is
/// created in it.
fn real_directory(dir: &Path) -> std::io::Result<()> {
    if std::fs::symlink_metadata(dir)?.file_type().is_dir() {
        Ok(())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "the output folder is not a directory (or is a symbolic link)"))
    }
}

/// `k6.read_records`.
pub fn read_records(out_dir: &Path) -> Vec<Value> {
    std::fs::read_to_string(out_dir.join("records.jsonl"))
        .map(|text| text.lines().filter(|l| !l.trim().is_empty()).filter_map(|l| serde_json::from_str(l).ok()).collect())
        .unwrap_or_default()
}

fn load() -> Value {
    json!(host::load_average().to_vec())
}

// ------------------------------------------------------------------ identity and metadata

/// `k6.metadata` and `rr.metadata`, from the build's embedded identity and `sysctl`.
pub fn metadata(binary: &Path) -> Map<String, Value> {
    let identity: Value = serde_json::from_str(crate::BUILD_IDENTITY).unwrap_or(Value::Null);
    let (system, release, machine) = host::uname();
    let mut md = Map::new();
    md.insert("os".into(), json!(system));
    md.insert("os_release".into(), json!(release));
    md.insert("machine".into(), json!(machine));
    // No Python here; the key is kept so the two metadata objects have the same keys.
    md.insert("python".into(), Value::Null);
    for key in ["build_profile", "source_commit", "source_tree", "source_dirty", "rustc", "cargo"] {
        md.insert(key.into(), identity.get(key).cloned().unwrap_or(Value::Null));
    }
    // The commit, tree and dirty flag are the build's (embedded by build.rs on every build, from
    // the tree the binary was linked from; source_dirty is true, false, or "unknown" where git
    // could not establish it); the Python runner records its own checkout's at run time.
    md.insert("source_identity_from".into(), json!("build"));
    if system.as_deref() == Some("Darwin") {
        // As the Python records them: strings, from sw_vers and sysctl -n.
        md.insert("os_product_version".into(), json!(host::sysctl_string("kern.osproductversion")));
        md.insert("os_build_version".into(), json!(host::sysctl_string("kern.osversion")));
        md.insert("cpu_model".into(), json!(host::sysctl_string("machdep.cpu.brand_string")));
        for (key, name) in [("cpu_physical", "hw.physicalcpu"), ("cpu_logical", "hw.logicalcpu"), ("memory_bytes", "hw.memsize")] {
            md.insert(key.into(), json!(host::sysctl_int(name).map(|v| v.to_string())));
        }
    } else {
        md.insert("cpu_logical".into(), json!(std::thread::available_parallelism().ok().map(|n| n.get().to_string())));
    }
    md.insert("binary_sha256".into(), json!(sha256_file(binary).ok()));
    md.insert("binary_name".into(), json!(binary.file_name().map(|n| n.to_string_lossy().to_string())));
    md.insert("runner".into(), json!(RUNNER));
    md.insert("base_machine_budget_bytes".into(), json!(BASE_MACHINE_BUDGET_BYTES));
    md.insert("emergency_rss_cap_bytes".into(), json!(EMERGENCY_RSS_CAP_BYTES));
    md.insert(
        "footprint_watchdog".into(),
        json!(cfg!(target_os = "macos").then_some("proc_pid_rusage RUSAGE_INFO_V4")),
    );
    md.insert("driver_schema".into(), json!(DRIVER_SCHEMA));
    md.insert("watchdog_poll_s".into(), json!(POLL_S));
    md
}

// ------------------------------------------------------------------ quiet host

/// `k6.wait_for_quiet_host`: no cargo job and no sweep, memorystatus_level at least 80, and the
/// 1-minute load at most 8; once above 8, wait until it is below 6. Waits in 30 s steps.
fn wait_for_quiet_host(log: &dyn Fn(&str), max_wait_s: u64) -> Result<Map<String, Value>, String> {
    let mut waited = 0u64;
    let mut load_waiting = false;
    loop {
        let busy = host::host_busy();
        let mem = host::memorystatus_level();
        let load1 = host::load_average()[0];
        if load1 > LOAD_HIGH {
            load_waiting = true;
        } else if load_waiting && load1 < LOAD_RESUME {
            load_waiting = false;
        }
        if busy.is_empty() && mem.is_none_or(|m| m >= MEMORYSTATUS_FLOOR) && !load_waiting {
            let mut found = Map::new();
            found.insert("waited_s".into(), json!(waited));
            found.insert("memorystatus_level".into(), json!(mem));
            found.insert("load1_at_start".into(), json!(load1));
            return Ok(found);
        }
        let busy_text = format!("[{}]", busy.iter().map(|b| format!("'{b}'")).collect::<Vec<_>>().join(", "));
        let mem_text = mem.map_or("None".to_string(), |m| m.to_string());
        if waited >= max_wait_s {
            return Err(format!("host not quiet after {waited} s: {busy_text}, memorystatus {mem_text}, load {load1:.2}"));
        }
        log(&format!("host busy ({busy_text}; memorystatus {mem_text}; load {load1:.2}); waiting 30 s"));
        sleep_interruptibly(QUIET_STEP_S);
        waited += QUIET_STEP_S;
    }
}

/// `rr.quiet_host`: waits at most `max_wait_s`, rounded down to whole 30 s steps; a host still
/// busy is recorded, and the run goes ahead.
fn quiet_host(log: &dyn Fn(&str), max_wait_s: f64) -> Value {
    let allowance = (max_wait_s.max(0.0).trunc() as u64 / QUIET_STEP_S) * QUIET_STEP_S;
    match wait_for_quiet_host(log, allowance) {
        Ok(mut found) => {
            found.insert("quiet".into(), json!(true));
            Value::Object(found)
        }
        Err(reason) => json!({"quiet": false, "reason": reason}),
    }
}

// ------------------------------------------------------------------ launch and watch

/// What the footprint watchdog saw (`rr.FootprintSampler`).
#[derive(Default)]
struct Sampler {
    budget_bytes: u64,
    samples: Vec<Value>,
    failed_reads: u64,
    peak_footprint: Option<u64>,
    peak_lifetime_max: Option<u64>,
    kill_reason: Option<String>,
    kill_footprint: Option<u64>,
}

impl Sampler {
    /// `FootprintSampler.__call__` on one reading: true asks for a kill.
    fn poll(&mut self, reading: Option<host::Reading>, now: f64) -> bool {
        let Some(reading) = reading else {
            self.failed_reads += 1;
            return false;
        };
        let (footprint, lifetime) = (reading.phys_footprint, reading.lifetime_max_phys_footprint);
        self.samples.push(json!([now, footprint, lifetime]));
        self.peak_footprint = Some(footprint.max(self.peak_footprint.unwrap_or(0)));
        self.peak_lifetime_max = Some(lifetime.max(self.peak_lifetime_max.unwrap_or(0)));
        let over = footprint.max(lifetime);
        if over > self.budget_bytes {
            self.kill_footprint = Some(over);
            self.kill_reason = Some(format!("footprint_{over}_above_budget_{}", self.budget_bytes));
            return true;
        }
        false
    }

    fn summary(&self) -> Value {
        json!({
            "kind": "phys_footprint", "budget_bytes": self.budget_bytes, "samples": self.samples,
            "sample_columns": ["t_s", "phys_footprint_bytes", "lifetime_max_phys_footprint_bytes"],
            "failed_reads": self.failed_reads, "peak_phys_footprint_bytes": self.peak_footprint,
            "peak_lifetime_max_phys_footprint_bytes": self.peak_lifetime_max,
            "kill_reason": self.kill_reason, "kill_footprint_bytes": self.kill_footprint,
        })
    }
}

/// Whether child `pid` has exited, without reaping it (`k6.exited`: `waitid(WEXITED | WNOHANG |
/// WNOWAIT)`), so that its final rusage can still be read and `wait4` still returns it.
fn exited_unreaped(pid: i32, block: bool) -> bool {
    let mut info = std::mem::MaybeUninit::<libc::siginfo_t>::zeroed();
    let flags = libc::WEXITED | libc::WNOWAIT | if block { 0 } else { libc::WNOHANG };
    loop {
        // SAFETY: `info` is a writable siginfo_t; `pid` is this process's child.
        let result = unsafe { libc::waitid(libc::P_PID, pid as libc::id_t, info.as_mut_ptr(), flags) };
        if result != 0 {
            if std::io::Error::last_os_error().raw_os_error() == Some(libc::EINTR) {
                continue;
            }
            return true; // ECHILD: no such child left to wait for.
        }
        // SAFETY: zeroed is valid and waitid filled it on success.
        return unsafe { info.assume_init() }.si_pid != 0;
    }
}

/// Reaps child `pid` (`wait4(pid, 0)`): (status, ru_maxrss in bytes on macOS).
fn reap(pid: i32) -> (libc::c_int, i64) {
    let mut status: libc::c_int = 0;
    let mut usage = std::mem::MaybeUninit::<libc::rusage>::zeroed();
    loop {
        // SAFETY: `status` and `usage` are writable; `pid` is this process's child.
        let result = unsafe { libc::wait4(pid, &mut status, 0, usage.as_mut_ptr()) };
        if result == -1 && std::io::Error::last_os_error().raw_os_error() == Some(libc::EINTR) {
            continue;
        }
        // SAFETY: zeroed is valid; wait4 filled it on success.
        return (status, unsafe { usage.assume_init() }.ru_maxrss);
    }
}

fn kill_group(pgid: i32) {
    // SAFETY: a signal to this supervisor's own child's process group.
    unsafe { libc::killpg(pgid, libc::SIGKILL) };
}

/// `os.waitstatus_to_exitcode`.
fn exit_code_of(status: libc::c_int) -> i32 {
    if libc::WIFEXITED(status) {
        libc::WEXITSTATUS(status)
    } else if libc::WIFSIGNALED(status) {
        -libc::WTERMSIG(status)
    } else {
        status
    }
}

/// `k6.survivors_of`: processes left in the group after a kill, retried while zombies are reaped.
fn survivors_of(pgid: i32) -> Vec<i32> {
    let mut members = Vec::new();
    for _ in 0..20 {
        members = host::group_members(pgid);
        if members.is_empty() {
            return members;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    members
}

/// The one owner of a spawned driver. From the spawn on, every path out of [`launch`] (a
/// return, an error or an unwind) passes through its drop, which kills the driver's process
/// group and reaps the driver unless the watch has already reaped it; so no failure after the
/// spawn leaves a driver running or unreaped.
struct Spawned {
    pid: i32,
    reaped: bool,
}

impl Spawned {
    /// Kills the driver's process group and reaps the driver, once.
    fn kill_and_reap(&mut self) {
        if !self.reaped {
            kill_group(self.pid);
            reap(self.pid);
            self.reaped = true;
        }
    }

    /// Reaps the exited (or killed) driver: (status, ru_maxrss).
    fn reap(&mut self) -> (libc::c_int, i64) {
        let reaped = reap(self.pid);
        self.reaped = true;
        reaped
    }
}

impl Drop for Spawned {
    fn drop(&mut self) {
        self.kill_and_reap();
    }
}

/// A launch's record and the driver's output, as the pumps collected it (the files hold the same
/// bytes; nothing is read back from them).
pub struct Launched {
    pub record: Map<String, Value>,
    pub stdout: String,
    pub stderr: String,
}

/// One driver process (`k6.launch` with `_watch` and `rr.FootprintSampler`): prepare the output
/// files, spawn, watch, classify, record. Returns the launch record, also written to
/// `<run_id>.record.json`, and the driver's output.
///
/// The three output files (`<run_id>.jsonl`, `.stderr.txt`, `.record.json`) are opened by
/// [`open_output`] before the spawn, so a destination that is not a regular file refuses the run
/// before any process starts and no file operation can block the way to the watch loop. From the
/// spawn on, [`Spawned`] owns the driver. `after_spawn` is called with the driver's pid right
/// after the spawn (the supervisor passes a no-op; a test injects a failure there).
#[allow(clippy::too_many_arguments)]
pub fn launch(
    binary: &Path,
    argv_tail: &[String],
    rss_cap_bytes: u64,
    footprint_budget_bytes: u64,
    timeout_s: i64,
    record_dir: &Path,
    run_id: &str,
    after_spawn: &mut dyn FnMut(i32) -> std::io::Result<()>,
) -> std::io::Result<Launched> {
    use std::os::unix::process::CommandExt;
    let base = record_dir.join(run_id);
    let with_suffix = |suffix: &str| PathBuf::from(format!("{}{suffix}", base.display()));
    let mut argv = vec![basename(&binary.to_string_lossy())];
    argv.extend(argv_tail.iter().map(|a| if Path::new(a).is_absolute() { basename(a) } else { a.clone() }));
    let mut record = Map::new();
    record.insert("schema".into(), json!(LAUNCH_SCHEMA));
    record.insert("run_id".into(), json!(run_id));
    record.insert("system".into(), json!(host::uname().0));
    record.insert("argv".into(), json!(argv));
    record.insert("wrapper".into(), Value::Null);
    record.insert("rss_cap_bytes".into(), json!(rss_cap_bytes));
    record.insert("timeout_s".into(), json!(timeout_s));
    record.insert("poll_s".into(), json!(POLL_S));
    record.insert("rlimit_as_applied".into(), json!(false));
    record.insert("load_before".into(), load());
    record.insert("memorystatus_before".into(), json!(host::memorystatus_level()));

    // Every output, prepared before the spawn: a destination that is not a regular file (or a new
    // file in a real directory) is refused here, before any process starts.
    real_directory(record_dir)?;
    let mut out_file = open_output(&with_suffix(".jsonl"), false)?;
    let mut err_file = open_output(&with_suffix(".stderr.txt"), false)?;
    let record_file = open_output(&with_suffix(".record.json"), false)?;
    let mut command = std::process::Command::new(binary);
    command.args(argv_tail).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped());
    // SAFETY: setsid is async-signal-safe; the child starts a new session and process group
    // (Python `subprocess.Popen(start_new_session=True)`), so its pgid is its pid.
    unsafe {
        command.pre_exec(|| if libc::setsid() == -1 { Err(std::io::Error::last_os_error()) } else { Ok(()) });
    }
    let t0 = Instant::now();
    let mut child = command.spawn()?;
    // From here on, `spawned` owns the driver: any `?`, return or panic below kills its group and
    // reaps it.
    let mut spawned = Spawned { pid: child.id() as i32, reaped: false };
    let pid = spawned.pid;
    after_spawn(pid)?;
    let stdout = child.stdout.take().ok_or_else(|| std::io::Error::other("no stdout pipe"))?;
    let stderr = child.stderr.take().ok_or_else(|| std::io::Error::other("no stderr pipe"))?;
    // `pump_out` and `pump_err`: stdout lines go to `<run_id>.jsonl` (opened above) as they come,
    // and are hashed as written; stderr is kept. The pumps touch no other file.
    let (out_tx, out_rx) = std::sync::mpsc::channel::<(Vec<String>, String)>();
    let (err_tx, err_rx) = std::sync::mpsc::channel::<Vec<u8>>();
    std::thread::Builder::new().spawn(move || {
        let (mut lines, mut digest) = (Vec::new(), Sha256::new());
        let mut reader = BufReader::new(stdout);
        let mut line = Vec::new();
        while reader.read_until(b'\n', &mut line).map(|n| n > 0).unwrap_or(false) {
            let _ = out_file.write_all(&line);
            let _ = out_file.flush();
            digest.update(&line);
            lines.push(String::from_utf8_lossy(&line).to_string());
            line.clear();
        }
        let _ = out_tx.send((lines, digest.finalize().iter().map(|b| format!("{b:02x}")).collect()));
    })?;
    std::thread::Builder::new().spawn(move || {
        let mut bytes = Vec::new();
        let _ = BufReader::new(stderr).read_to_end(&mut bytes);
        let _ = err_tx.send(bytes);
    })?;

    // `k6._watch`: the poll loop.
    let mut sampler = Sampler { budget_bytes: footprint_budget_bytes, ..Sampler::default() };
    let (mut resident_samples, mut polls) = (Vec::<Value>::new(), 0u64);
    let (mut killed, mut timed_out, mut sampler_killed, mut kill_at) = (false, false, false, None::<f64>);
    loop {
        if exited_unreaped(pid, false) {
            break;
        }
        if let Some(signal) = interrupted() {
            // RV18-3: the supervisor is stopping; the driver must not run on unwatched.
            spawned.kill_and_reap();
            exit_interrupted(signal);
        }
        let now = t0.elapsed().as_secs_f64();
        let reading = host::rusage_v4(pid);
        polls += 1;
        if let Some(r) = reading {
            resident_samples.push(json!([round_to(now, 3), r.resident_size]));
        }
        if sampler.poll(reading, round_to(now, 3)) {
            kill_group(pid);
            (sampler_killed, kill_at) = (true, Some(round_to(now, 3)));
            break;
        }
        if reading.is_some_and(|r| r.resident_size > rss_cap_bytes) {
            kill_group(pid);
            (killed, kill_at) = (true, Some(round_to(now, 3)));
            break;
        }
        if now > timeout_s as f64 {
            kill_group(pid);
            (timed_out, kill_at) = (true, Some(round_to(now, 3)));
            break;
        }
        std::thread::sleep(POLL);
    }
    // The child has exited or been killed: wait for its exit without reaping it, read its final
    // lifetime maximum (what /usr/bin/time -l reports as peak memory footprint), then reap it.
    exited_unreaped(pid, true);
    let at_exit = host::rusage_v4(pid);
    let (status, ru_maxrss) = spawned.reap();
    let wall = t0.elapsed().as_secs_f64();
    // `t.join(timeout=5)` for each pump (a pump still blocked after that gives nothing; the
    // Python keeps what its pump had collected so far).
    let deadline = Instant::now() + Duration::from_secs(5);
    let (stdout_lines, stdout_sha256) =
        out_rx.recv_timeout(deadline.saturating_duration_since(Instant::now())).unwrap_or_else(|_| (Vec::new(), sha256_hex(b"")));
    let stderr_bytes = err_rx.recv_timeout(deadline.saturating_duration_since(Instant::now())).unwrap_or_default();
    drop(child);
    let exit_code = exit_code_of(status);
    let stderr_text = String::from_utf8_lossy(&stderr_bytes).to_string();
    err_file.write_all(sanitize(&stderr_text).as_bytes())?;
    let stdout_text = stdout_lines.concat();
    let objects = jsonl(&stdout_text);
    let external = (exit_code < 0 && !(killed || timed_out || sampler_killed)).then_some(-exit_code);
    let (mut class, mut detail) = classify::launch_class(killed, timed_out, exit_code, &objects, &stderr_text, external);
    if sampler_killed {
        (class, detail) = ("killed_by_sampler".to_string(), sampler.kill_reason.clone());
    }
    let survivors = if killed || timed_out || sampler_killed { survivors_of(pid) } else { Vec::new() };
    let resident_max = resident_samples.iter().filter_map(|s| s[1].as_u64()).max();
    // The Python keys stay, null where their source (/usr/bin/time, ps) is absent here; the
    // siblings carry this supervisor's readings.
    let rss = json!({
        "wait4_maxrss_bytes": ru_maxrss,
        "ps_max_kib": null,
        "ps_samples": null,
        "time_max_rss_bytes": null,
        "time_peak_footprint_bytes": null,
        "resident_max_bytes": resident_max,
        "resident_samples": resident_samples.len(),
        "exit_lifetime_max_phys_footprint_bytes": at_exit.map(|r| r.lifetime_max_phys_footprint),
    });
    record.insert("classification".into(), json!(class));
    record.insert("classification_detail".into(), json!(detail));
    record.insert("exit_code".into(), json!(exit_code));
    record.insert("wall_s".into(), json!(round_to(wall, 4)));
    record.insert("binary_pid_found".into(), json!(true));
    record.insert("killed_by_rss_watchdog".into(), json!(killed));
    record.insert("timed_out".into(), json!(timed_out));
    record.insert("kill_after_s".into(), json!(kill_at));
    record.insert("watchdog_last_kib".into(), Value::Null);
    record.insert("watchdog_last_resident_bytes".into(), json!(resident_samples.last().map(|s| s[1].clone())));
    record.insert("watchdog_polls".into(), json!(polls));
    record.insert("watchdog_samples".into(), Value::Null);
    record.insert("resident_samples".into(), json!(resident_samples));
    record.insert("survivors".into(), json!(survivors));
    record.insert("rss".into(), rss);
    // `peak_rss_bytes`: the Python takes /usr/bin/time's maximum resident set size, which is
    // wait4's ru_maxrss of the driver; here that is read directly.
    record.insert("peak_rss_bytes".into(), json!(ru_maxrss));
    record.insert("peak_rss_source".into(), json!("wait4"));
    record.insert("load_after".into(), load());
    record.insert("memorystatus_after".into(), json!(host::memorystatus_level()));
    record.insert(
        "summary".into(),
        objects.iter().find(|o| o.get("kind").and_then(Value::as_str) == Some("summary")).cloned().unwrap_or(Value::Null),
    );
    record.insert("stdout_lines".into(), json!(stdout_lines.len()));
    record.insert("stderr_tail".into(), json!(sanitize(tail_chars(&stderr_text, STDERR_TAIL_CHARS))));
    record.insert("killed_by_sampler".into(), json!(sampler_killed));
    record.insert("sampler".into(), sampler.summary());
    record.insert("stdout_sha256".into(), json!(stdout_sha256));
    write_json_to(record_file, &Value::Object(record.clone()))?;
    Ok(Launched { record, stdout: stdout_text, stderr: sanitize(&stderr_text) })
}

/// `rr.driver_argv` without the binary: a solve run's model and `--mode`, or a model-checks run's
/// model and `--model-checks`, then `--write` when results are kept.
pub fn driver_argv(model: &Path, run: &Run, results_dir: Option<&Path>) -> Vec<String> {
    let mut tail = vec![model.to_string_lossy().to_string()];
    if run.task == "model_checks" {
        tail.push("--model-checks".into());
    } else {
        tail.extend(["--mode".to_string(), run.mode.clone()]);
    }
    if let Some(dir) = results_dir {
        tail.push("--write".into());
        tail.push(dir.join(format!("{}.result.json", run.run_id)).to_string_lossy().to_string());
    }
    tail
}

/// `rr.driver_line`: the first driver object (`DRIVER_SCHEMA`) of the run's stdout.
fn driver_line(stdout: &str) -> Option<Value> {
    jsonl(stdout).into_iter().find(|o| o.get("schema").and_then(Value::as_str) == Some(DRIVER_SCHEMA))
}

// ------------------------------------------------------------------ run

pub struct Options {
    pub plan: Plan,
    pub models_dir: PathBuf,
    pub out_dir: PathBuf,
    pub binary: PathBuf,
    pub write_results: bool,
}

/// The launch keys a run record keeps (`rr.run_plan`'s `launch={...}`), and this supervisor's
/// siblings.
const LAUNCH_KEYS: [&str; 24] = [
    "classification",
    "classification_detail",
    "exit_code",
    "wall_s",
    "rss",
    "peak_rss_bytes",
    "peak_rss_source",
    "killed_by_rss_watchdog",
    "killed_by_sampler",
    "timed_out",
    "kill_after_s",
    "watchdog_last_kib",
    "watchdog_polls",
    "survivors",
    "binary_pid_found",
    "load_before",
    "load_after",
    "memorystatus_before",
    "memorystatus_after",
    "stdout_sha256",
    "stderr_tail",
    "argv",
    // Sibling: the last resident-size sample, where the Python keeps the last ps sample in KiB.
    "watchdog_last_resident_bytes",
    // Sibling: the launch record's own schema, naming this supervisor.
    "schema",
];

/// `rr.run_plan`: the plan's runs in order, one at a time, within the window budget.
pub fn run_plan(options: &Options, log: &dyn Fn(&str)) -> Result<Vec<Value>, String> {
    let start = Instant::now();
    let clock = || start.elapsed().as_secs_f64();
    let out_dir = &options.out_dir;
    let plan = &options.plan;
    let budget_s = plan.window_budget_s as f64;
    std::fs::create_dir_all(out_dir).map_err(|e| format!("cannot create the output folder: {e}"))?;
    real_directory(out_dir).map_err(|e| format!("cannot use the output folder: {e}"))?;
    if std::fs::symlink_metadata(out_dir.join("records.jsonl")).is_ok() {
        return Err("records already exist in the output folder; use a new one".into());
    }
    let mut md = metadata(&options.binary);
    md.insert("budget_s".into(), json!(plan.window_budget_s));
    md.insert("write_results".into(), json!(options.write_results));
    md.insert("quiet_wait_s".into(), json!(plan.quiet_wait_s));
    md.insert("plan".into(), json!(plan.runs.iter().map(|r| r.run_id.clone()).collect::<Vec<_>>()));
    let io = |e: std::io::Error| format!("cannot write the records: {e}");
    write_json(&out_dir.join("metadata.json"), &Value::Object(md)).map_err(io)?;
    let results_dir = options.write_results.then(|| out_dir.join("results"));
    if let Some(dir) = &results_dir {
        std::fs::create_dir_all(dir).map_err(io)?;
    }
    let mut records = Vec::new();
    let mut launched_any = false;
    for run in &plan.runs {
        if let Some(signal) = interrupted() {
            exit_interrupted(signal);
        }
        let elapsed = clock();
        let mut remaining = budget_s - elapsed;
        let mut record = run.entry.clone();
        record.insert("schema".into(), json!(RECORD_SCHEMA));
        record.insert("window_elapsed_before_s".into(), json!(round_to(elapsed, 3)));
        let path = options.models_dir.join(&run.model);
        record.insert("model".into(), json!(run.model));
        if run.estimate_s as f64 > remaining {
            record.insert("classification".into(), json!("skipped_window_budget"));
            record.insert(
                "classification_detail".into(),
                json!(format!("estimate {} s exceeds the remaining {} s", run.estimate_s, remaining.trunc() as i64)),
            );
        } else if !path.exists() {
            record.insert("classification".into(), json!("model_missing"));
            record.insert("classification_detail".into(), json!(run.model));
        } else {
            let sha = sha256_file(&path).map_err(|e| format!("cannot read {}: {e}", run.model))?;
            record.insert("model_sha256".into(), json!(sha));
            record.insert("model_bytes".into(), json!(std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0)));
            let listed = run.expected_model_sha256.as_deref();
            if listed.is_some_and(|expected| expected != sha) {
                record.insert("classification".into(), json!("model_mismatch"));
                record.insert("classification_detail".into(), json!("plan sha256 differs"));
            } else {
                if listed.is_some() {
                    record.insert("model_stats".into(), run.model_stats.clone());
                }
                // Before the first launched run the host may settle for up to quiet_wait_s; before
                // each later run its state is checked once and recorded, without waiting.
                let wait = if launched_any { 0.0 } else { plan.quiet_wait_s as f64 };
                record.insert("host".into(), quiet_host(log, wait.min(remaining - run.estimate_s as f64).max(0.0)));
                launched_any = true;
                remaining = budget_s - clock();
                if run.estimate_s as f64 > remaining || remaining < 1.0 {
                    record.insert("classification".into(), json!("skipped_window_budget"));
                    record.insert(
                        "classification_detail".into(),
                        json!(format!(
                            "estimate {} s exceeds the {} s remaining after the host check",
                            run.estimate_s,
                            remaining.trunc() as i64
                        )),
                    );
                    record.insert("window_elapsed_after_s".into(), json!(round_to(clock(), 3)));
                    let value = Value::Object(record);
                    append_record(out_dir, &value).map_err(io)?;
                    records.push(value);
                    log(&format!("{}: skipped_window_budget after the host check", run.run_id));
                    write_packet(out_dir).map_err(io)?;
                    continue;
                }
                let timeout = run.timeout_s.min(remaining.trunc() as i64);
                record.insert("timeout_applied_s".into(), json!(timeout));
                record.insert("timeout_capped_by_window".into(), json!(timeout < run.timeout_s));
                let tail = driver_argv(&path, run, results_dir.as_deref());
                let Launched { record: launched, stdout, stderr } = launch(
                    &options.binary,
                    &tail,
                    run.rss_cap_bytes,
                    run.footprint_budget_bytes,
                    timeout,
                    out_dir,
                    &run.run_id,
                    &mut |_| Ok(()),
                )
                .map_err(|e| format!("cannot launch {}: {e}", run.run_id))?;
                let launched = Value::Object(launched);
                let driver = driver_line(&stdout);
                let (last_stage, last_memory, stages_finished) = classify::last_progress(Some(stderr.as_str()));
                let memory = Some(&last_memory).filter(|m| !m.is_null());
                record.insert("classification_readings".into(), classify::readings_record(&launched, driver.as_ref(), memory));
                let (classification, detail) = classify::classify(
                    &launched,
                    driver.as_ref(),
                    i128::from(run.rss_cap_bytes),
                    i128::from(run.footprint_budget_bytes),
                    memory,
                );
                let watchdog: Map<String, Value> = launched
                    .get("sampler")
                    .and_then(Value::as_object)
                    .map(|m| m.iter().filter(|(k, _)| k.as_str() != "samples").map(|(k, v)| (k.clone(), v.clone())).collect())
                    .unwrap_or_default();
                record.insert("last_stage".into(), last_stage);
                record.insert("last_stage_memory".into(), last_memory.clone());
                record.insert("stages_finished".into(), json!(stages_finished));
                record.insert("footprint_watchdog".into(), Value::Object(watchdog));
                record.insert("peak_footprint_bytes".into(), json!(classify::peak_footprint(driver.as_ref(), Some(&launched), memory)));
                record.insert("as_expected".into(), json!(classify::as_expected(&run.expect, &classification, detail.as_deref())));
                record.insert("classification".into(), json!(classification));
                record.insert("classification_detail".into(), json!(detail));
                record.insert(
                    "launch".into(),
                    Value::Object(LAUNCH_KEYS.iter().map(|k| (k.to_string(), launched.get(*k).cloned().unwrap_or(Value::Null))).collect()),
                );
                record.insert("driver".into(), driver.unwrap_or(Value::Null));
            }
        }
        record.insert("window_elapsed_after_s".into(), json!(round_to(clock(), 3)));
        let value = Value::Object(record);
        append_record(out_dir, &value).map_err(io)?;
        let wall = value.get("launch").and_then(|l| l.get("wall_s")).and_then(Value::as_f64);
        let detail = value.get("classification_detail").filter(|d| truthy(d)).map(|d| format!(" [{}]", py_str(Some(d))));
        log(&format!(
            "{}: {}{} ({}; window {:.0} s)",
            run.run_id,
            py_str(value.get("classification")),
            detail.unwrap_or_default(),
            wall.map_or("not run".to_string(), |w| format!("wall {w:.1} s")),
            value["window_elapsed_after_s"].as_f64().unwrap_or(0.0),
        ));
        records.push(value);
        write_packet(out_dir).map_err(io)?;
    }
    Ok(records)
}

// ------------------------------------------------------------------ packet

fn stage_memory(driver: Option<&Value>, key: &str) -> Option<i128> {
    let peaks = driver.and_then(|d| d.get("peak_memory")).and_then(Value::as_object)?;
    peaks.values().filter(|m| m.is_object()).filter_map(|m| int(m.get(key))).max()
}

/// `rr.run_conditions`.
pub fn run_conditions(r: &Value) -> String {
    let host = r.get("host").filter(|h| h.is_object());
    let state = match host.and_then(|h| h.get("quiet")) {
        None => "not checked".to_string(),
        Some(quiet) if truthy(quiet) => {
            let h = host.expect("host");
            format!(
                "quiet (memorystatus {}, 1-minute load {:.2}, waited {} s)",
                py_str(h.get("memorystatus_level")),
                h.get("load1_at_start").and_then(Value::as_f64).unwrap_or(0.0),
                py_str(h.get("waited_s"))
            )
        }
        Some(_) => format!("not quiet: {}", py_str(host.and_then(|h| h.get("reason")))),
    };
    let budget = r.get("footprint_budget_bytes").and_then(Value::as_f64).filter(|b| *b != 0.0);
    format!(
        "{}, {} nodes, {} mode, {}; timeout {} s{}, footprint budget {}, emergency RSS cap {:.2} GiB; host {}.",
        py_str(r.get("profile")),
        py_str(r.get("nodes")),
        py_str(r.get("mode")),
        r.get("model").filter(|m| truthy(m)).map_or("no model".to_string(), |m| py_str(Some(m))),
        py_str(r.get("timeout_applied_s")),
        if r.get("timeout_capped_by_window").is_some_and(truthy) { " (capped by the window)" } else { "" },
        budget.map_or("none".to_string(), |b| format!("{:.2} GiB", b / GIB)),
        r.get("rss_cap_bytes").and_then(Value::as_f64).unwrap_or(0.0) / GIB,
        state
    )
}

/// `rr.packet_row`, with this supervisor's readings beside the Python's in `peak_memory_bytes`.
pub fn packet_row(r: &Value) -> Value {
    let null = Value::Null;
    let driver = r.get("driver").filter(|d| d.is_object()).unwrap_or(&null);
    let launch = r.get("launch").filter(|l| l.is_object()).unwrap_or(&null);
    let rss = launch.get("rss").filter(|v| v.is_object()).unwrap_or(&null);
    let watchdog = r.get("footprint_watchdog").filter(|v| v.is_object()).unwrap_or(&null);
    let d = |k: &str| driver.get(k).cloned().unwrap_or(Value::Null);
    let g = |v: &Value, k: &str| v.get(k).cloned().unwrap_or(Value::Null);
    let ps_max_rss = int(rss.get("ps_max_kib")).map(|kib| kib * 1024);
    json!({
        "order": g(r, "order"), "run_id": g(r, "run_id"), "task": g(r, "task"),
        "profile": g(r, "profile"), "nodes": g(r, "nodes"),
        "mode": g(r, "mode"), "route": d("route"), "expect": g(r, "expect"),
        "classification": g(r, "classification"), "classification_detail": g(r, "classification_detail"),
        "as_expected": g(r, "as_expected"), "conditions": run_conditions(r),
        "model": g(r, "model"), "model_sha256": g(r, "model_sha256"), "model_bytes": g(r, "model_bytes"),
        "counts": {"nodes": d("nodes"), "members": d("members"), "supports": d("supports"), "cases": d("cases"),
                   "combinations": d("combinations")},
        "status": d("status"), "blocking_codes": d("blocking_codes"),
        "wall_s": g(launch, "wall_s"), "stage_seconds": d("seconds"),
        "peak_memory_bytes": {
            "time_max_rss": g(rss, "time_max_rss_bytes"),
            "time_peak_footprint": g(rss, "time_peak_footprint_bytes"),
            "ps_max_rss": ps_max_rss.map(|v| json!(v)).unwrap_or(Value::Null),
            "driver_ru_maxrss": stage_memory(Some(driver), "ru_maxrss_bytes").map(|v| json!(v)).unwrap_or(Value::Null),
            "driver_lifetime_max_phys_footprint": d("worker_peak_footprint_bytes_equivalent"),
            "sampled_phys_footprint": g(watchdog, "peak_phys_footprint_bytes"),
            "sampled_lifetime_max_phys_footprint": g(watchdog, "peak_lifetime_max_phys_footprint_bytes"),
            "peak_footprint": g(r, "peak_footprint_bytes"),
            // Siblings: this supervisor's readings where the Python's sources are absent.
            "wait4_maxrss": g(rss, "wait4_maxrss_bytes"),
            "sampled_resident_size": g(rss, "resident_max_bytes"),
            "exit_lifetime_max_phys_footprint": g(rss, "exit_lifetime_max_phys_footprint_bytes"),
        },
        "last_stage": g(r, "last_stage"), "last_stage_memory": g(r, "last_stage_memory"),
        "attribution": r.get("attribute").is_some_and(truthy),
        "timing_use": r.get("timing_use").cloned().unwrap_or(json!("measurement")),
        "snapshots": r.get("attribution").and_then(|a| a.get("snapshots")).cloned().unwrap_or(Value::Null),
        "rows": d("rows"), "output_bytes": d("output_bytes"),
        "output_sha256": d("output_sha256"), "row_classes": d("row_classes"),
        "bases": driver.get("rows_per_basis").and_then(Value::as_object).map_or(0, Map::len),
        "diagnostic_counts": d("diagnostic_counts"),
        "nonlinear_iterations": d("nonlinear_iterations"),
    })
}

/// `rr.packet`.
pub fn packet(out_dir: &Path) -> Value {
    let records: Vec<Value> =
        read_records(out_dir).into_iter().filter(|r| r.get("schema").and_then(Value::as_str) == Some(RECORD_SCHEMA)).collect();
    let md: Value = std::fs::read_to_string(out_dir.join("metadata.json"))
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok())
        .unwrap_or(Value::Null);
    let runs: Vec<Value> = records.iter().filter(|r| r.get("launch").is_some()).map(packet_row).collect();
    let not_run: Vec<Value> = records
        .iter()
        .filter(|r| r.get("launch").is_none())
        .map(|r| {
            Value::Object(
                ["order", "run_id", "classification", "classification_detail"]
                    .iter()
                    .map(|k| (k.to_string(), r.get(*k).cloned().unwrap_or(Value::Null)))
                    .collect(),
            )
        })
        .collect();
    let mut counts = Map::new();
    for r in &records {
        let key = py_str(r.get("classification"));
        let n = counts.get(&key).and_then(Value::as_u64).unwrap_or(0);
        counts.insert(key, json!(n + 1));
    }
    let driver_limits = records
        .iter()
        .filter_map(|r| r.get("driver").and_then(|d| d.get("limits")))
        .find(|limits| truthy(limits))
        .cloned()
        .unwrap_or(Value::Null);
    let mut conditions: Vec<Value> = CONDITIONS.iter().map(|c| json!(c)).collect();
    conditions.push(json!(format!(
        "Window budget {} s; {} run(s) not run (skipped, missing or mismatched model).",
        py_str(md.get("budget_s")),
        not_run.len()
    )));
    let window_elapsed = records
        .iter()
        .map(|r| r.get("window_elapsed_after_s").filter(|v| truthy(v)).and_then(Value::as_f64).unwrap_or(0.0))
        .fold(None::<f64>, |a, b| Some(a.map_or(b, |a| a.max(b))));
    json!({
        "schema": PACKET_SCHEMA, "observation_only": true,
        "no_thresholds": "no time or memory bound is asserted anywhere; criterion 9 provisional bounds are not evaluated",
        "conditions": conditions,
        "metadata": md, "classification_counts": counts, "runs": runs, "not_run": not_run,
        "window_elapsed_s": window_elapsed.map_or(json!(0), |w| json!(w)),
        "driver_limits": driver_limits,
    })
}

pub fn write_packet(out_dir: &Path) -> std::io::Result<Value> {
    let result = packet(out_dir);
    write_json(&out_dir.join("packet.json"), &result)?;
    Ok(result)
}

// ------------------------------------------------------------------ entry

const USAGE: &str = "usage: release_timing --supervise <plan.json> --out <dir> [--models <dir>] [--write-results]";

fn usage(message: &str) -> i32 {
    eprintln!("release_timing --supervise: {message}");
    eprintln!("{USAGE}");
    2
}

/// `release_timing --supervise ...`; `args` are the arguments after the program name. Returns
/// the exit code: 0, 1 when a run's classification is error, model_missing or model_mismatch (as
/// the Python `--run`), 2 on a usage, plan or record-folder error.
pub fn main(args: &[String]) -> i32 {
    let mut args = args.iter();
    let (mut plan_path, mut out_dir, mut models_dir, mut write_results) = (None, None, None, false);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--supervise" => plan_path = args.next().cloned(),
            "--out" => out_dir = args.next().cloned(),
            "--models" => models_dir = args.next().cloned(),
            "--write-results" => write_results = true,
            other => return usage(&format!("unexpected argument {other}")),
        }
    }
    let (Some(plan_path), Some(out_dir)) = (plan_path, out_dir) else {
        return usage("--supervise needs a plan file and --out needs a folder");
    };
    if !cfg!(target_os = "macos") {
        return usage("the supervisor reads proc_pid_rusage, which only macOS has; use the Python runner elsewhere");
    }
    let text = match std::fs::read_to_string(&plan_path) {
        Ok(text) => text,
        Err(error) => return usage(&format!("cannot read the plan: {}", error.kind())),
    };
    let plan = match read_plan(&text) {
        Ok(plan) => plan,
        Err(message) => return usage(&message),
    };
    let models_dir = models_dir.map(PathBuf::from).unwrap_or_else(|| {
        Path::new(&plan_path).parent().filter(|p| !p.as_os_str().is_empty()).map_or_else(|| PathBuf::from("."), Path::to_path_buf)
    });
    let models_dir = std::fs::canonicalize(&models_dir).unwrap_or(models_dir);
    let binary = match std::env::current_exe() {
        Ok(binary) => binary,
        Err(error) => return usage(&format!("cannot find this binary: {error}")),
    };
    install_signal_handlers();
    let options = Options { plan, models_dir, out_dir: PathBuf::from(out_dir), binary, write_results };
    let log = |line: &str| {
        println!("{line}");
        let _ = std::io::stdout().flush();
    };
    match run_plan(&options, &log) {
        Ok(records) => {
            let counts = packet(&options.out_dir)["classification_counts"].clone();
            println!("{}", json!({"packet": "packet.json", "classification_counts": counts}));
            let bad = records.iter().any(|r| {
                matches!(r.get("classification").and_then(Value::as_str), Some("error" | "model_missing" | "model_mismatch"))
            });
            i32::from(bad)
        }
        Err(message) => usage(&message),
    }
}
