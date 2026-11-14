//! DEL-04-07 REQ-008, part 1 (evidence only): the whole product call's heap in
//! sparse mode, the twin of NI's loop-level `tests/sparse_loop_memory.rs`.
//!
//! This binary asserts no memory bound. For each model document it measures,
//! with a counting global allocator, the call
//! `run_linear_static_preview_value_with_mode(request, SparseInteractive)`:
//! its peak live heap (increment over the live heap just before the call, which
//! holds the parsed request the call consumes) and its live heap after the call
//! (the returned envelope, less the request it consumed). That is the evidence
//! for what lies outside the loop: the model build, the basis stiffness, every
//! case and combination, recovery and the result rows. The figures are requested
//! bytes (`Layout::size`) and, on macOS, `malloc_size` of each block; thread
//! stacks and other memory not taken through the global allocator are not seen.
//!
//! Counters: this entry takes the ordinary route on the calling thread (it passes no
//! retained entry, so the reserved-stack thread is never spawned), so the figures are the
//! measuring thread's own tally of requested bytes (fields prefixed `thread_`), which the
//! self-tests check exactly. The process-wide counters (prefix `process_`) are diagnostics;
//! every record says so in its `counters` field. The call's peaks and counters are read
//! before any report is formed, and the report's own allocations are counted apart.
//!
//! Models: the manifest written by NI's `tests/sparse_loop_memory/generate_models.py`,
//! in the directory named by `SWBPIPE_SPARSE_MEMORY_MODELS`; each file's SHA-256 is
//! checked against the manifest. One JSON line per model goes to the file named
//! by `SWBPIPE_SPARSE_MEMORY_OUT` (else stdout). Run in release, one test at a time:
//! `cargo test --release --test sparse_admission_memory_twin -- --test-threads 1`.
//! While the file named by `SWBPIPE_TIMING_WINDOW_FILE` exists, no model is solved.
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::alloc::{GlobalAlloc, Layout, System};
use std::collections::BTreeMap;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
use std::sync::Mutex;

/// A safety net, not a measurement.
const CAP_BYTES: usize = 16 << 30;

static LIVE_REQ: AtomicUsize = AtomicUsize::new(0);
static LIVE_RND: AtomicUsize = AtomicUsize::new(0);
static PEAK_REQ: AtomicUsize = AtomicUsize::new(0);
static PEAK_RND: AtomicUsize = AtomicUsize::new(0);
static PEAK_MOVE: AtomicUsize = AtomicUsize::new(0);
static ALLOCS: AtomicUsize = AtomicUsize::new(0);
static DEALLOCS: AtomicUsize = AtomicUsize::new(0);
static REALLOCS: AtomicUsize = AtomicUsize::new(0);
static MOVED_REALLOCS: AtomicUsize = AtomicUsize::new(0);
static SERIAL: Mutex<()> = Mutex::new(());

#[cfg(target_os = "macos")]
extern "C" {
    fn malloc_size(ptr: *const std::ffi::c_void) -> usize;
}

fn rounded(ptr: *mut u8, requested: usize) -> usize {
    #[cfg(target_os = "macos")]
    {
        let _ = requested;
        unsafe { malloc_size(ptr as *const std::ffi::c_void) }
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = ptr;
        requested
    }
}

// The calling thread's own requested live bytes and its peak: the measuring
// thread's share, isolated from every other thread's allocations. Const-initialised and
// without a destructor, so the allocator may read it at any time.
thread_local! {
    static THREAD_LIVE: std::cell::Cell<isize> = const { std::cell::Cell::new(0) };
    static THREAD_PEAK: std::cell::Cell<isize> = const { std::cell::Cell::new(0) };
}

fn thread_add(delta: isize) {
    let _ = THREAD_LIVE.try_with(|live| {
        let now = live.get() + delta;
        live.set(now);
        let _ = THREAD_PEAK.try_with(|peak| {
            if now > peak.get() {
                peak.set(now);
            }
        });
    });
}

fn thread_live() -> isize {
    THREAD_LIVE.try_with(|live| live.get()).unwrap_or(0)
}

fn thread_peak() -> isize {
    THREAD_PEAK.try_with(|peak| peak.get()).unwrap_or(0)
}

fn grow(requested: usize, rnd: usize) {
    let live = LIVE_REQ.fetch_add(requested, SeqCst) + requested;
    if live > CAP_BYTES {
        std::process::abort();
    }
    PEAK_REQ.fetch_max(live, SeqCst);
    PEAK_MOVE.fetch_max(live, SeqCst);
    let live_rnd = LIVE_RND.fetch_add(rnd, SeqCst) + rnd;
    PEAK_RND.fetch_max(live_rnd, SeqCst);
    thread_add(requested as isize);
}

fn shrink(requested: usize, rnd: usize) {
    LIVE_REQ.fetch_sub(requested, SeqCst);
    LIVE_RND.fetch_sub(rnd, SeqCst);
    thread_add(-(requested as isize));
}

struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() {
            ALLOCS.fetch_add(1, SeqCst);
            grow(layout.size(), rounded(ptr, layout.size()));
        }
        ptr
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc_zeroed(layout) };
        if !ptr.is_null() {
            ALLOCS.fetch_add(1, SeqCst);
            grow(layout.size(), rounded(ptr, layout.size()));
        }
        ptr
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let rnd = rounded(ptr, layout.size());
        unsafe { System.dealloc(ptr, layout) };
        DEALLOCS.fetch_add(1, SeqCst);
        shrink(layout.size(), rnd);
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let old_rnd = rounded(ptr, layout.size());
        let new_ptr = unsafe { System.realloc(ptr, layout, new_size) };
        if new_ptr.is_null() {
            return new_ptr;
        }
        REALLOCS.fetch_add(1, SeqCst);
        if new_ptr != ptr {
            MOVED_REALLOCS.fetch_add(1, SeqCst);
            PEAK_MOVE.fetch_max(LIVE_REQ.load(SeqCst) + new_size, SeqCst);
        }
        let new_rnd = rounded(new_ptr, new_size);
        shrink(layout.size(), old_rnd);
        grow(new_size, new_rnd);
        new_ptr
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

#[derive(Clone, Copy)]
struct Snapshot {
    live_req: usize,
    live_rnd: usize,
    allocs: usize,
    deallocs: usize,
    reallocs: usize,
    moved: usize,
    thread_live: isize,
}

fn snapshot() -> Snapshot {
    Snapshot {
        live_req: LIVE_REQ.load(SeqCst),
        live_rnd: LIVE_RND.load(SeqCst),
        allocs: ALLOCS.load(SeqCst),
        deallocs: DEALLOCS.load(SeqCst),
        reallocs: REALLOCS.load(SeqCst),
        moved: MOVED_REALLOCS.load(SeqCst),
        thread_live: thread_live(),
    }
}

fn reset_peaks() -> Snapshot {
    let now = snapshot();
    PEAK_REQ.store(now.live_req, SeqCst);
    PEAK_MOVE.store(now.live_req, SeqCst);
    PEAK_RND.store(now.live_rnd, SeqCst);
    let _ = THREAD_PEAK.try_with(|peak| peak.set(thread_live()));
    snapshot()
}

/// The call's end: every counter and peak, read at once and without allocating, before any
/// report is formed.
#[derive(Clone, Copy)]
struct Endpoint {
    at: Snapshot,
    peak_req: usize,
    peak_rnd: usize,
    peak_move: usize,
    thread_peak: isize,
}

fn endpoint() -> Endpoint {
    Endpoint {
        thread_peak: thread_peak(),
        peak_req: PEAK_REQ.load(SeqCst),
        peak_rnd: PEAK_RND.load(SeqCst),
        peak_move: PEAK_MOVE.load(SeqCst),
        at: snapshot(),
    }
}

/// What the field prefixes of each record name, carried in every record.
fn counters() -> Value {
    json!({
        "thread_": "the measuring thread's own tally of requested bytes (Layout::size); exactly self-tested; authoritative",
        "process_": "process-wide counters shared by every thread: diagnostics only, not exactly self-tested (the self-test bounds the requested peak within 64 KiB); rounded bytes are malloc_size of each block, the moving-realloc peak adds each moved block's new size to the live total, and event counts include any thread's events",
    })
}

fn wait_for_timing_window() {
    let Some(path) = std::env::var_os("SWBPIPE_TIMING_WINDOW_FILE") else { return };
    while std::path::Path::new(&path).exists() {
        eprintln!("timing window open: waiting 60 s");
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}

/// The JSON-lines output: the file named by `SWBPIPE_SPARSE_MEMORY_OUT`, created once per
/// run (else stdout).
static OUT: Mutex<Option<std::fs::File>> = Mutex::new(None);

fn open_out() {
    if let Some(path) = std::env::var_os("SWBPIPE_SPARSE_MEMORY_OUT") {
        *OUT.lock().unwrap_or_else(|e| e.into_inner()) = Some(std::fs::File::create(path).unwrap());
    }
}

fn emit(line: &str) {
    match OUT.lock().unwrap_or_else(|e| e.into_inner()).as_mut() {
        Some(file) => writeln!(file, "{line}").unwrap(),
        None => println!("{line}"),
    }
}

/// The envelope's facts this measurement reports (read after the boundary).
fn envelope_facts(envelope: &Value) -> Value {
    let results = envelope["results"].as_array().map_or(&[][..], |r| r.as_slice());
    let iterations = results
        .iter()
        .filter(|row| row["kind"] == "nonlinear_support_active_set_iteration_count")
        .map(|row| json!({"entity_ref": row["entity_ref"], "value": row["value"]}))
        .collect::<Vec<_>>();
    let converged = results
        .iter()
        .filter(|row| row["kind"] == "nonlinear_support_active_set_converged_flag")
        .map(|row| json!({"entity_ref": row["entity_ref"], "value": row["value"]}))
        .collect::<Vec<_>>();
    let mut codes = BTreeMap::new();
    let mut blocking = Vec::new();
    for diagnostic in envelope["diagnostics"].as_array().map_or(&[][..], |d| d.as_slice()) {
        let code = diagnostic["code"].as_str().unwrap_or("").to_string();
        if diagnostic["severity"] == "blocking" {
            blocking.push(json!({"code": code, "message": diagnostic["message"]}));
        }
        *codes.entry(code).or_insert(0usize) += 1;
    }
    let case_rows = results.iter().fold(BTreeMap::new(), |mut by_case: BTreeMap<String, usize>, row| {
        let case = row["basis_ref"]["ref_id"].as_str().unwrap_or("").to_string();
        *by_case.entry(case).or_insert(0) += 1;
        by_case
    });
    json!({
        "mechanics": envelope["status"]["mechanics"],
        "numerical_quality": envelope["numerical_quality"]["status"],
        "result_rows": results.len(),
        "rows_by_basis_ref": case_rows,
        "iteration_count_rows": iterations,
        "converged_flag_rows": converged,
        "blocking": blocking,
        "diagnostic_codes": codes,
    })
}

fn measure(name: &str, family: &str, sha: &str, bytes: Vec<u8>) {
    wait_for_timing_window();
    // The parsed request's size: the increment across the parse, while the file buffer is
    // still held on both sides.
    let before_parse = thread_live();
    let request: Value = serde_json::from_slice(&bytes).unwrap();
    let after_parse = thread_live();
    drop(bytes);
    let start = reset_peaks();
    let request_live = after_parse - before_parse;
    let started = std::time::Instant::now();
    let outcome = run_linear_static_preview_value_with_mode(request, PreviewSolverMode::SparseInteractive);
    // The end is read before anything is formatted.
    let end = endpoint();
    let seconds = started.elapsed().as_secs_f64();
    // The report's values are formed after the end; their bytes are counted apart
    // (`thread_report_values_live`) and excluded from the residual below.
    let report_start = thread_live();
    let boundary = json!({
        "thread_peak_requested": end.thread_peak - start.thread_live,
        "thread_live_requested_after_call": end.at.thread_live - start.thread_live,
        "process_peak_requested": end.peak_req - start.live_req,
        "process_peak_rounded": end.peak_rnd - start.live_rnd,
        "process_peak_requested_moving_realloc": end.peak_move - start.live_req,
        "process_live_requested_after_call": end.at.live_req as i128 - start.live_req as i128,
        "process_live_rounded_after_call": end.at.live_rnd as i128 - start.live_rnd as i128,
        "process_allocations": end.at.allocs - start.allocs,
        "process_deallocations": end.at.deallocs - start.deallocs,
        "process_reallocations": end.at.reallocs - start.reallocs,
        "process_moved_reallocations": end.at.moved - start.moved,
    });
    // The facts are read from a serialized copy, after the boundary.
    let (facts, envelope_error) = match &outcome {
        Ok(envelope) => (envelope_facts(&serde_json::to_value(envelope).unwrap()), Value::Null),
        Err(error) => (Value::Null, json!(error)),
    };
    let report_live = thread_live() - report_start;
    // Release: nothing but the drop lies between the two readings.
    let held = thread_live();
    drop(outcome);
    let after_drop = thread_live();
    let line = json!({
        "level": "product",
        "model": name,
        "family": family,
        "sha256": sha,
        "mode": "sparse_interactive",
        "counters": counters(),
        "thread_request_value_live_requested": request_live,
        "call": boundary,
        "thread_envelope_live_requested": held - after_drop,
        "thread_report_values_live": report_live,
        "thread_live_after_envelope_drop_vs_call_start_excluding_report_values":
            after_drop - start.thread_live - report_live,
        "envelope": facts,
        "entry_error": envelope_error,
        "call_seconds": (seconds * 1000.0).round() / 1000.0,
    });
    emit(&line.to_string());
    eprintln!("{name}: peak {} B", line["call"]["thread_peak_requested"]);
}

fn models(levels: &[&str]) -> Vec<(String, String, String, std::path::PathBuf)> {
    let Some(dir) = std::env::var_os("SWBPIPE_SPARSE_MEMORY_MODELS").map(std::path::PathBuf::from) else {
        return Vec::new();
    };
    let manifest: Value = serde_json::from_slice(&std::fs::read(dir.join("manifest.json")).unwrap()).unwrap();
    let only = std::env::var("SWBPIPE_SPARSE_MEMORY_ONLY").ok();
    let mut out = Vec::new();
    for entry in manifest["models"].as_array().unwrap() {
        let entry_levels = entry["levels"].as_array().unwrap();
        if !entry_levels.iter().any(|l| levels.contains(&l.as_str().unwrap())) {
            continue;
        }
        let name = entry["name"].as_str().unwrap().to_string();
        if only.as_deref().is_some_and(|only| !only.split(',').any(|o| o == name)) {
            continue;
        }
        let path = dir.join(entry["file"].as_str().unwrap());
        let sha = format!("{:x}", Sha256::digest(std::fs::read(&path).unwrap()));
        assert_eq!(sha, entry["sha256"].as_str().unwrap(), "{name}: the file's SHA-256 differs from the manifest's");
        out.push((name, entry["family"].as_str().unwrap().to_string(), sha, path));
    }
    out
}

#[test]
fn the_counter_counts_a_known_allocation() {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let start = reset_peaks();
    let block = vec![0u8; 1 << 20];
    let held = snapshot();
    drop(block);
    let end = endpoint();
    // Exact on this thread's own tally, which no other thread touches, read as the
    // measurement reads it (`endpoint`).
    assert_eq!(end.thread_peak - start.thread_live, 1 << 20);
    assert_eq!(held.thread_live - start.thread_live, 1 << 20);
    assert_eq!(end.at.thread_live, start.thread_live);
    // The global counters, which the harness's other threads share: bounded within 64 KiB.
    let peak = end.peak_req as i128 - start.live_req as i128;
    assert!(((1 << 20) - (64 << 10)..(1 << 20) + (64 << 10)).contains(&peak), "peak {peak}");
}

#[test]
fn the_thread_tally_counts_a_reallocation() {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let start = reset_peaks();
    let mut block: Vec<u8> = Vec::with_capacity(1 << 19);
    block.reserve_exact(1 << 20);
    let grown = snapshot();
    let capacity = block.capacity();
    drop(block);
    let end = endpoint();
    // A reallocation leaves the new size live and the old one released; the peak is the new size.
    assert_eq!(grown.thread_live - start.thread_live, capacity as isize);
    assert_eq!(end.thread_peak - start.thread_live, capacity as isize);
    assert_eq!(end.at.thread_live, start.thread_live);
    assert!(grown.reallocs > start.reallocs);
}

/// Every manifest model marked `product` or `refusal`, through the headless
/// entry in sparse mode; a refusal's peak is the product guard's.
#[test]
fn whole_call_peaks_over_the_model_family() {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    if std::env::var_os("SWBPIPE_SPARSE_MEMORY_MODELS").is_none() {
        eprintln!("SWBPIPE_SPARSE_MEMORY_MODELS is not set: no model is measured");
        return;
    }
    open_out();
    for (name, family, sha, path) in models(&["product", "refusal"]) {
        measure(&name, &family, &sha, std::fs::read(path).unwrap());
    }
}
