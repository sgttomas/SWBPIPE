//! `release_timing`: the DEL-04-05 release timing driver (RT-U4, release
//! target exit criterion 9). Observation only: it asserts no time or memory
//! bound.
//!
//! It runs, in one process, the engine stages the desktop's capped solve
//! worker runs for an ordinary solve, in the worker's order and with the same
//! calls:
//!
//! - `read`: the model document's bytes (the worker reads them from its
//!   control pipe; here `std::fs::read`);
//! - `parse`: `serde_json::from_slice::<Value>` of the bytes, then the bytes
//!   are released (`solve_worker.rs` `ordinary_solve`);
//! - `request`: the `{"model": model, "materials": []}` request, built by
//!   moving the model in (`ingress.rs` `native_request`);
//! - `entry`: `run_linear_static_preview_value_with_mode(request, mode)`
//!   (`lib.rs` `solve_preview_mechanics_with_mode`);
//! - `serialize`: `serde_json::to_string` of the envelope, the worker's exact
//!   result bytes;
//! - `release_envelope`: the envelope's drop (in the worker, at the end of the
//!   `to_string` statement);
//! - `parse_back`: `serde_json::from_str::<Value>` of the result text and its
//!   drop (`solve_worker.rs` `result_digest`; the streamed digest itself is in
//!   the desktop crate and is not reproduced).
//!
//! After each stage it reads `getrusage` `ru_maxrss` and, on macOS, the
//! process's `phys_footprint` and lifetime-maximum `phys_footprint`
//! (`proc_pid_rusage`, as `worker_alloc.rs` reads them; the lifetime maximum
//! is what the worker reports in its `RSLT` frame and the desktop records as
//! the job's `worker_peak_footprint_bytes`).
//!
//! After each stage it also writes one progress line to stderr,
//! `release-timing-stage {"repeat", "stage", "seconds", "ru_maxrss_bytes",
//! "phys_footprint_bytes", "lifetime_max_phys_footprint_bytes"}`, so that a run
//! killed from outside still shows the last stage it finished.
//!
//! Usage: `release_timing <document.json> [--mode sparse|dense] [--write <path>] [--repeats N]`.
//! The input is a model document (what the desktop opens and sends) or a
//! request `{"model": ..., "materials": [...]}`, whose model is used. Prints
//! one JSON line (schema `release-timing-v1`). Exit 0 when the entry returned
//! an envelope (solved or blocked), 3 on an entry error (still one JSON line),
//! 2 on a usage or file error.

use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, MechanicsEnvelope, PreviewSolverMode};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::io::Write;
use std::time::Instant;

const SCHEMA: &str = "release-timing-v1";
const STATIONS: [&str; 5] = ["end_i", "quarter_1", "midspan", "quarter_3", "end_j"];
const STAGES: [&str; 7] = ["read", "parse", "request", "entry", "serialize", "release_envelope", "parse_back"];
const ENTRY_ERROR_EXIT: i32 = 3;
const USAGE_EXIT: i32 = 2;

const LIMITS: [&str; 10] = [
    "Engine stages only: the desktop solve worker's ordinary solve (solve_worker.rs ordinary_solve and attested_solve, lib.rs solve_preview_mechanics_with_mode, ingress.rs native_request) run in one process. The app, its IPC and ingress prescan, the memory pool's leases and growth exchanges, the worker's process start and control frames, and the webview are not exercised.",
    "The solve attestation's two streamed canonical digests (d_M of the model before the solve, d_S of the parsed-back result) live in the desktop crate and are not reproduced: the worker spends that time and this driver does not. parse_back times serde_json::from_str of the result text and the value's drop only.",
    "The worker's counting, capped global allocator (worker_alloc.rs CappedAllocator over System) is not installed; this binary allocates through System directly.",
    "The document is read from a file (std::fs::read) instead of the worker's control pipe; in the worker the parent also holds the raw body and the result bytes, in its own process.",
    "ru_maxrss_bytes and lifetime_max_phys_footprint_bytes are process high-water marks: with more than one repeat, a later repeat's peaks include every earlier repeat's.",
    "worker_peak_footprint_bytes_equivalent is this process's lifetime-maximum phys_footprint (proc_pid_rusage RUSAGE_INFO_V4) read after parse_back, as the worker reads it for its RSLT frame; the desktop records the larger of that report and the parent's reading of the same counter for the worker's pid.",
    "Publication statistics (rows, kinds, classes, bases, diagnostics) are computed from the envelope between entry and serialize, outside every timed stage; output_sha256 and --write happen after serialize, also untimed.",
    "Row bytes per kind and per class are each row's serde_json::to_writer length; they exclude the results array's separators and the envelope's other members, so they sum to less than output_bytes.",
    "Row classes: station = metadata.location in end_i/quarter_1/midspan/quarter_3/end_j; node = metadata.location node; governing = metadata.location governing_station; other = any other location or no metadata.",
    "Observation only: no time or memory bound is asserted.",
];

// ------------------------------------------------------------------ memory

/// This process's `ru_maxrss` in bytes (bytes on macOS, KiB elsewhere).
fn ru_maxrss_bytes() -> Option<u64> {
    #[cfg(unix)]
    {
        let mut usage = std::mem::MaybeUninit::<libc::rusage>::zeroed();
        // SAFETY: `usage` is a writable rusage the call fills.
        if unsafe { libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr()) } != 0 {
            return None;
        }
        // SAFETY: zeroed is a valid value and the call filled it on success.
        let maxrss = u64::try_from(unsafe { usage.assume_init() }.ru_maxrss).ok()?;
        if cfg!(target_os = "macos") {
            Some(maxrss)
        } else {
            Some(maxrss.saturating_mul(1024))
        }
    }
    #[cfg(not(unix))]
    {
        None
    }
}

/// `(phys_footprint, lifetime_max_phys_footprint)` of this process, as
/// `worker_alloc.rs` `pid_phys_footprint` and `pid_lifetime_max_footprint`
/// read them (`None` off macOS or on a failed read).
fn footprints() -> (Option<u64>, Option<u64>) {
    #[cfg(target_os = "macos")]
    {
        // SAFETY: getpid has no preconditions.
        let pid = unsafe { libc::getpid() };
        let mut v0 = std::mem::MaybeUninit::<libc::rusage_info_v0>::zeroed();
        // SAFETY: `v0` is a writable rusage_info_v0, the struct RUSAGE_INFO_V0 fills.
        let current = (unsafe { libc::proc_pid_rusage(pid, libc::RUSAGE_INFO_V0, v0.as_mut_ptr().cast()) } == 0)
            // SAFETY: zeroed is a valid value and the kernel filled it on success.
            .then(|| unsafe { v0.assume_init() }.ri_phys_footprint);
        let mut v4 = std::mem::MaybeUninit::<libc::rusage_info_v4>::zeroed();
        // SAFETY: `v4` is a writable rusage_info_v4, the struct RUSAGE_INFO_V4 fills.
        let lifetime = (unsafe { libc::proc_pid_rusage(pid, libc::RUSAGE_INFO_V4, v4.as_mut_ptr().cast()) } == 0)
            // SAFETY: as above.
            .then(|| unsafe { v4.assume_init() }.ri_lifetime_max_phys_footprint);
        (current, lifetime)
    }
    #[cfg(not(target_os = "macos"))]
    {
        (None, None)
    }
}

fn memory() -> Value {
    let (current, lifetime) = footprints();
    json!({
        "ru_maxrss_bytes": ru_maxrss_bytes(),
        "phys_footprint_bytes": current,
        "lifetime_max_phys_footprint_bytes": lifetime,
    })
}

// ------------------------------------------------------------------ stages

/// The prefix of the progress line written to stderr after each stage.
const PROGRESS_PREFIX: &str = "release-timing-stage ";

/// One repeat's stage times and memory readings, in stage order.
#[derive(Default)]
struct Timeline {
    repeat: usize,
    seconds: Map<String, Value>,
    memory: Map<String, Value>,
    total: f64,
}

impl Timeline {
    fn new(repeat: usize) -> Self {
        Timeline { repeat, ..Timeline::default() }
    }

    /// Runs and times one stage, reads memory after it, and writes one progress line to stderr
    /// (`release-timing-stage {json}`: repeat, stage, seconds and the three memory readings), so
    /// a run killed later still shows the last stage it finished. Stdout is unchanged.
    fn stage<T>(&mut self, name: &str, work: impl FnOnce() -> T) -> T {
        let started = Instant::now();
        let out = work();
        let elapsed = started.elapsed().as_secs_f64();
        self.total += elapsed;
        let reading = memory();
        let mut progress = json!({"repeat": self.repeat, "stage": name, "seconds": elapsed});
        if let (Value::Object(line), Value::Object(readings)) = (&mut progress, &reading) {
            line.extend(readings.clone());
        }
        eprintln!("{PROGRESS_PREFIX}{progress}");
        self.seconds.insert(name.to_string(), json!(elapsed));
        self.memory.insert(name.to_string(), reading);
        out
    }

    fn seconds(&self) -> Value {
        let mut seconds = self.seconds.clone();
        seconds.insert("total".to_string(), json!(self.total));
        Value::Object(seconds)
    }

    fn peak(&self) -> Value {
        self.memory
            .get("parse_back")
            .and_then(|m| m.get("lifetime_max_phys_footprint_bytes"))
            .cloned()
            .unwrap_or(Value::Null)
    }
}

/// `ingress.rs` `native_request`: the model moves in, no deep copy.
fn native_request(model: Value) -> Value {
    let mut request = Map::new();
    request.insert("model".to_string(), model);
    request.insert("materials".to_string(), Value::Array(Vec::new()));
    Value::Object(request)
}

/// A request `{"model": {...}, "materials": [...]}` (only those keys) gives its
/// model; anything else is taken as the model document itself.
fn take_model(parsed: Value) -> (Value, &'static str, usize) {
    match parsed {
        Value::Object(mut object)
            if object.get("model").is_some_and(Value::is_object)
                && object.keys().all(|key| key == "model" || key == "materials") =>
        {
            let ignored = object.get("materials").and_then(Value::as_array).map_or(0, Vec::len);
            let model = object.remove("model").unwrap_or(Value::Null);
            (model, "request", ignored)
        }
        other => (other, "model_document", 0),
    }
}

fn model_counts(model: &Value) -> Value {
    let count = |key: &str| model.get(key).and_then(Value::as_array).map(Vec::len);
    json!({
        "nodes": count("nodes"),
        "members": count("pipe_segments"),
        "supports": count("supports"),
        "cases": count("load_cases"),
        "combinations": count("combinations"),
    })
}

// ------------------------------------------------------------------ statistics

/// An `io::Write` that only counts bytes.
struct Counter(u64);

impl Write for Counter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0 += buf.len() as u64;
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[derive(Default)]
struct Tally {
    rows: u64,
    bytes: u64,
}

impl Tally {
    fn add(&mut self, bytes: u64) {
        self.rows += 1;
        self.bytes += bytes;
    }
    fn value(&self) -> Value {
        json!({"rows": self.rows, "bytes": self.bytes})
    }
}

fn tallies<K: AsRef<str>>(map: &BTreeMap<K, Tally>) -> Value {
    Value::Object(map.iter().map(|(k, t)| (k.as_ref().to_string(), t.value())).collect())
}

fn publication(envelope: &MechanicsEnvelope) -> Map<String, Value> {
    let mut kinds: BTreeMap<&str, Tally> = BTreeMap::new();
    let mut classes: BTreeMap<&str, Tally> = ["station", "node", "governing", "other"].into_iter().map(|c| (c, Tally::default())).collect();
    let mut per_basis: BTreeMap<String, u64> = BTreeMap::new();
    let mut without_basis = 0u64;
    let mut iterations: BTreeMap<String, f64> = BTreeMap::new();
    for row in &envelope.results {
        let mut counter = Counter(0);
        let bytes = serde_json::to_writer(&mut counter, row).map(|()| counter.0).unwrap_or(0);
        kinds.entry(row.kind.as_str()).or_default().add(bytes);
        let class = match row.metadata.as_ref().map(|m| m.location.as_str()) {
            Some(location) if STATIONS.contains(&location) => "station",
            Some("node") => "node",
            Some("governing_station") => "governing",
            _ => "other",
        };
        classes.entry(class).or_default().add(bytes);
        match &row.basis_ref {
            Some(basis) => {
                *per_basis.entry(format!("{}:{}", basis.ref_type, basis.ref_id)).or_default() += 1;
                if row.kind == "nonlinear_support_active_set_iteration_count" {
                    iterations.insert(format!("{}:{}", basis.ref_type, basis.ref_id), row.value);
                }
            }
            None => without_basis += 1,
        }
    }
    let mut diagnostic_counts: BTreeMap<String, u64> = BTreeMap::new();
    let mut blocking_codes: Vec<&str> = Vec::new();
    for d in &envelope.diagnostics {
        *diagnostic_counts.entry(format!("{}:{}", d.severity, d.code)).or_default() += 1;
        if d.severity == "blocking" && !blocking_codes.contains(&d.code.as_str()) {
            blocking_codes.push(&d.code);
        }
    }
    let first_blocking: Vec<&str> =
        envelope.diagnostics.iter().filter(|d| d.severity == "blocking").take(3).map(|d| d.message.as_str()).collect();
    let mut out = Map::new();
    out.insert("status".into(), json!(envelope.status.mechanics));
    out.insert("blocking_codes".into(), json!(blocking_codes));
    out.insert("first_blocking_messages".into(), json!(first_blocking));
    out.insert("diagnostic_counts".into(), json!(diagnostic_counts));
    out.insert("rows".into(), json!(envelope.results.len()));
    out.insert("rows_per_basis".into(), json!(per_basis));
    out.insert("rows_without_basis".into(), json!(without_basis));
    out.insert("row_kinds".into(), tallies(&kinds));
    out.insert("row_classes".into(), tallies(&classes));
    out.insert("nonlinear_iterations".into(), json!(iterations));
    out
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes).iter().map(|b| format!("{b:02x}")).collect()
}

// ------------------------------------------------------------------ driver

struct Args {
    path: String,
    mode: PreviewSolverMode,
    write: Option<String>,
    repeats: usize,
}

fn usage(message: &str) -> ! {
    eprintln!("release_timing: {message}");
    eprintln!("usage: release_timing <document.json> [--mode sparse|dense] [--write <path>] [--repeats N]");
    std::process::exit(USAGE_EXIT)
}

fn parse_args() -> Args {
    let mut args = std::env::args().skip(1);
    let (mut path, mut mode, mut write, mut repeats) = (None, PreviewSolverMode::SparseInteractive, None, 1usize);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--mode" => {
                mode = match args.next().as_deref() {
                    Some("sparse" | "sparse_interactive") => PreviewSolverMode::SparseInteractive,
                    Some("dense" | "dense_scrutiny") => PreviewSolverMode::DenseScrutiny,
                    other => usage(&format!("unknown mode {other:?}")),
                }
            }
            "--write" => write = Some(args.next().unwrap_or_else(|| usage("--write needs a path"))),
            "--repeats" => {
                repeats = args
                    .next()
                    .and_then(|n| n.parse().ok())
                    .filter(|n| *n >= 1)
                    .unwrap_or_else(|| usage("--repeats needs a positive integer"))
            }
            other if other.starts_with("--") => usage(&format!("unknown option {other}")),
            other if path.is_none() => path = Some(other.to_string()),
            other => usage(&format!("unexpected argument {other}")),
        }
    }
    Args { path: path.unwrap_or_else(|| usage("a document path is required")), mode, write, repeats }
}

fn emit(out: &Map<String, Value>) {
    println!("{}", Value::Object(out.clone()));
}

fn main() {
    let args = parse_args();
    let mut out = Map::new();
    out.insert("schema".into(), json!(SCHEMA));
    out.insert("driver".into(), json!({"name": "release_timing", "version": env!("CARGO_PKG_VERSION")}));
    out.insert("mode".into(), json!(args.mode.as_str()));
    out.insert("repeats_requested".into(), json!(args.repeats));
    out.insert("stages".into(), json!(STAGES));
    out.insert("limits".into(), json!(LIMITS));
    out.insert("memory_at_start".into(), memory());

    let mut repeats: Vec<Value> = Vec::new();
    let mut first_sha: Option<String> = None;
    let mut identical = true;
    for repeat in 0..args.repeats {
        let mut t = Timeline::new(repeat);
        let raw = match t.stage("read", || std::fs::read(&args.path)) {
            Ok(raw) => raw,
            Err(error) => {
                out.insert("status".into(), json!("driver_error"));
                out.insert("driver_error".into(), json!(format!("cannot read the document: {}", error.kind())));
                emit(&out);
                std::process::exit(USAGE_EXIT);
            }
        };
        if repeat == 0 {
            out.insert("document".into(), json!({"sha256": sha256_hex(&raw), "bytes": raw.len()}));
        }
        let parsed = t.stage("parse", || {
            let parsed = serde_json::from_slice::<Value>(&raw);
            drop(raw);
            parsed
        });
        let parsed = match parsed {
            Ok(parsed) => parsed,
            Err(error) => {
                return entry_error(out, &t, repeats, "parse", format!("PREVIEW-SOLVE-MODEL-INVALID: {error}"));
            }
        };
        let (model, input_kind, ignored) = take_model(parsed);
        if repeat == 0 {
            if let Some(Value::Object(document)) = out.get_mut("document") {
                document.insert("input_kind".into(), json!(input_kind));
                document.insert("request_materials_ignored".into(), json!(ignored));
            }
            out.insert("route".into(), json!(if model.get("pressure_contract").is_some() { "EX" } else { "PF" }));
            if let Value::Object(counts) = model_counts(&model) {
                out.extend(counts);
            }
        }
        let request = t.stage("request", || native_request(model));
        let envelope = match t.stage("entry", || run_linear_static_preview_value_with_mode(request, args.mode)) {
            Ok(envelope) => envelope,
            Err(message) => return entry_error(out, &t, repeats, "entry", message),
        };
        if repeat == 0 {
            out.extend(publication(&envelope));
        }
        let text = match t.stage("serialize", || serde_json::to_string(&envelope)) {
            Ok(text) => text,
            Err(error) => return entry_error(out, &t, repeats, "serialize", error.to_string()),
        };
        t.stage("release_envelope", || drop(envelope));
        let parse_back_ok = t.stage("parse_back", || serde_json::from_str::<Value>(&text).map(drop).is_ok());
        let sha = sha256_hex(text.as_bytes());
        identical &= first_sha.as_ref().is_none_or(|first| *first == sha);
        if repeat == 0 {
            if let Some(path) = &args.write {
                if let Err(error) = std::fs::write(path, text.as_bytes()) {
                    out.insert("status".into(), json!("driver_error"));
                    out.insert("driver_error".into(), json!(format!("cannot write the result: {}", error.kind())));
                    emit(&out);
                    std::process::exit(USAGE_EXIT);
                }
                out.insert("written".into(), json!(true));
            }
            out.insert("output_bytes".into(), json!(text.len()));
            out.insert("output_sha256".into(), json!(sha));
            out.insert("parse_back_ok".into(), json!(parse_back_ok));
            out.insert("seconds".into(), t.seconds());
            out.insert("peak_memory".into(), Value::Object(t.memory.clone()));
            out.insert("worker_peak_footprint_bytes_equivalent".into(), t.peak());
            first_sha = Some(sha.clone());
        }
        drop(text);
        repeats.push(json!({
            "repeat": repeat,
            "seconds": t.seconds(),
            "peak_memory": Value::Object(t.memory.clone()),
            "worker_peak_footprint_bytes_equivalent": t.peak(),
            "output_sha256": sha,
            "parse_back_ok": parse_back_ok,
        }));
    }
    out.insert("outputs_identical_across_repeats".into(), json!(identical));
    out.insert("repeats".into(), Value::Array(repeats));
    emit(&out);
}

/// An entry error (the worker's `FAIL` frame): one JSON line, exit 3.
fn entry_error(mut out: Map<String, Value>, t: &Timeline, mut repeats: Vec<Value>, stage: &str, message: String) {
    let repeat = repeats.len();
    out.insert("status".into(), json!("entry_error"));
    out.insert("entry_error".into(), json!({"stage": stage, "message": message, "repeat": repeat}));
    if repeat == 0 {
        out.insert("seconds".into(), t.seconds());
        out.insert("peak_memory".into(), Value::Object(t.memory.clone()));
    }
    repeats.push(json!({"repeat": repeat, "seconds": t.seconds(), "peak_memory": Value::Object(t.memory.clone())}));
    out.insert("repeats".into(), Value::Array(repeats));
    emit(&out);
    std::process::exit(ENTRY_ERROR_EXIT)
}
