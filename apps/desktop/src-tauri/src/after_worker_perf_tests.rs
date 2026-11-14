//! Way H's after-worker harness (DEL-04-05): **the native sequence, replayed in
//! order**. An attribution instrument, not a check: it times and counts, call
//! by call, the native reader calls the desktop makes between the worker's
//! `job_done` and first paint, on a result the real worker publishes.
//!
//! What it drives, in the web's order and with the web's arguments
//! (`previewService.ts` `readNativeFreshResult` and `validateNativeReaders`,
//! `nativeResult.ts`, `workspaceSession.ts` after the terminal poll,
//! `NativeResultsPanel.tsx`'s first page), each command through its
//! executor reservation and then its blocking part in this thread, as
//! `native_verdict_tests.rs` and `native_replay_tests.rs` call them:
//! 1. the route header (`read_preview_mechanics_job_verdict`, `route_header`);
//! 2. the registration (the same command with the route's `reader` and the
//!    captured invocation's model text);
//! 3. the standing (the dispatch, with the session model's text);
//! 4. the summary (`read_preview_mechanics_job_summary`, with that standing);
//!    then, only where the web does, the standing and summary again (an
//!    envelope refusal after the readers passed), the dispatch again (a
//!    changed key; the web's cache answers it otherwise) and the source
//!    recovery;
//! 5. run_preview (`read_preview_mechanics_job_analysis_run_preview`, with the
//!    input manifest the session builds);
//! 6. the first rows page (`query_preview_mechanics_job_rows`, the panel's
//!    default filter, offset 0, the default page size) and its row semantics
//!    (`read_preview_mechanics_job_row_semantics`).
//!
//! **Acceptance is the web's own code.** With [`REPLIES_ENV`] set, each native
//! reply is kept byte for byte, after its call's clock stops, and
//! `apps/desktop/src/test-support/after-worker/afterWorkerReplies.test.ts` feeds
//! the kept replies to the web's exported decoders in the web's order
//! (`decodeNativeRouteHeader`, `decodeNativeValidation`, `decodeNativeVerdict`,
//! `decodeResultSummary`, `requireRouteHeaderAgreement`, `decodeSourceRecovery`,
//! `decodeAnalysisRunPreview`, `decodeRowsPage`, `decodeRowSemantics`). The
//! checks in this file are structural pre-checks that steer the sequence (the
//! route, the standing, the producer and the first page's ordinals); they are
//! not the web's acceptance and claim none. A resource refusal ends the
//! sequence where the web's `ResourceUnavailableError` ends it, and is
//! recorded with its `required_bytes`. Nothing asserts a time or a size: a
//! refusal (for example at 1,000 nodes) is an observed baseline, not a
//! requirement.
//!
//! **What it omits.** IPC transfer, the web's bookkeeping and caches, React
//! commits, rendering and the two animation frames, and the panel's
//! concurrent kind-presence lookups (`useNativeKindPresence`'s two
//! `lookup_preview_mechanics_job_rows` calls beside the first page). So it is
//! not first paint and not release criterion 9's clock. The pool is a
//! [`ScriptedHost`] reporting this host's physical memory as capacity and
//! headroom: it removes real headroom refusals and isolates the fixed 12 GiB
//! cap; it does not reproduce admission on the base machine.
//!
//! **The model texts.** The web's texts are made by the web's own
//! transformations, through checked Rust equivalents, from the document File >
//! Open Model Document delivers: `model_document_file::read_model_document_bytes`
//! (the open path's parse and checks), serde_json's text of that document (the
//! IPC reply the webview `JSON.parse`s), then `result_js_text`, which writes
//! `JSON.stringify` (`NegativeZero::AsZero`) or `jsonTextKeepingNegativeZero`
//! (`NegativeZero::Keep`) of a JavaScript parse of a text and is byte-checked
//! against the TypeScript paths (`result_js_text_tests.rs`). The start body
//! and the registration's text go through `checkedJsonText` (`JSON.stringify`,
//! a negative zero written `0`); the standing's and the summary's through
//! `jsonTextKeepingNegativeZero` of the session model (`-0` kept). Whether the
//! two are equal is recorded per run, as an observation for that document.
//!
//! **What it records.** One JSON-lines record per run, appended to the file
//! [`RECORD_ENV`] names (outside the repository): the head, the document and
//! its sha256, the result text's bytes and sha256, the solve's wall time, per
//! call its edges, wall time, reply bytes and outcome, the per-phase lines of
//! the native phase recorder (`native_verdict::diagnostic_phase`, emitted by
//! `--features native-smoke` on the header, registration, standing and summary
//! calls only), the full parses those lines show and each parse lease's M
//! (`source_admit` bytes), M as the prescan prices it for the leases no phase
//! line shows, the footprints at call edges, the allocation figures with
//! their basis (attribution runs only), the two model texts' equality, and
//! the host's load at start and end.
//!
//! **Two kinds of run, never mixed.** A timing run (counting off) and an
//! attribution run ([`COUNTING_ENV`] = `1`): the per-thread allocation counter
//! (`ingress::counting`, fed by `worker_alloc`'s `CappedAllocator`) is armed
//! across the whole sequence and read with phase marks at each call's edges,
//! so a parse that survives between calls (the detail index on the held
//! share) stays in the running figure. It counts this thread's requested
//! bytes only, from zero at the sequence's start: what existed before (the
//! held result text, the registry, the model texts) is outside it, and what
//! other threads allocate (the registry's supervisor) is not seen.
//!
//! **How to run** (an exact filter, `--ignored`, a release build with the
//! feature; the parent re-runs this binary on [`child_entry`] so that the
//! phase lines on stderr are captured, and the worker is the same binary):
//! `cargo test --release --features native-smoke --lib
//! after_worker_perf_tests::replay_rack -- --exact --ignored --nocapture`
//! with [`NODES_ENV`] (and optionally [`PROFILE_ENV`], default `linear-ex`),
//! or `after_worker_perf_tests::replay_document` with [`MODEL_ENV`] naming a
//! model document; [`RECORD_ENV`] names the output file in both.
use crate::command_ingress::test_support::headers;
use crate::command_ingress::{CommandError, INGRESS_ARGS_JSON_V1};
use crate::memory_pool::test_host::ScriptedHost;
use crate::memory_pool::{PressureLevel, ResourceRefusal};
use crate::model_document_file::{read_model_document_bytes, ModelDocumentOpenOutcome};
use crate::result_js_text::{self, NegativeZero};
use crate::solve_jobs::{ResultCustody, SolveJobError, SolveJobRegistry};
use crate::{analysis_run_v03_build, analysis_runs, ingress, native_verdict, result_detail, result_summary, worker_alloc};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use tauri::ipc::InvokeBody;

/// Rack size (nodes) for [`replay_rack`].
pub(crate) const NODES_ENV: &str = "SWBPIPE_AFTER_WORKER_RACK_NODES";
/// Rack profile for [`replay_rack`] (default `linear-ex`).
pub(crate) const PROFILE_ENV: &str = "SWBPIPE_AFTER_WORKER_RACK_PROFILE";
/// A model document's path for [`replay_document`].
pub(crate) const MODEL_ENV: &str = "SWBPIPE_AFTER_WORKER_MODEL";
/// The JSON-lines file each run appends its record to (outside the repository).
pub(crate) const RECORD_ENV: &str = "SWBPIPE_AFTER_WORKER_RECORD";
/// `1`: an attribution run (allocation counting on); otherwise a timing run.
pub(crate) const COUNTING_ENV: &str = "SWBPIPE_AFTER_WORKER_COUNTING";
/// A directory (outside the repository) under which each run keeps its native replies, one
/// subdirectory per run, for the web's own decoders to judge (`afterWorkerReplies.test.ts`).
pub(crate) const REPLIES_ENV: &str = "SWBPIPE_AFTER_WORKER_REPLIES";
/// Set by the parent on the child run: which source the child reads.
const CHILD_ENV: &str = "SWBPIPE_AFTER_WORKER_CHILD";
/// The native phase recorder's switch (`native_smoke::reader_diagnostic_call`).
const DIAGNOSTICS_ENV: &str = "SWBPIPE_NATIVE_SMOKE_READER_DIAGNOSTICS";
const CHILD_ENTRY: &str = "after_worker_perf_tests::child_entry";
/// The child's report line on stderr.
const REPORT_MARK: &str = "[after-worker] report:";
/// The native phase recorder's line (`native_smoke::reader_diagnostic_event`).
const PHASE_MARK: &str = "[native-smoke] reader-phase:";
/// The session's solver mode (`resultsSessionState.ts`) and the result custody of a desktop solve.
const SOLVER_MODE: &str = "sparse_interactive";
/// `DEFAULT_RESULT_PAGE_SIZE` (`ResultsPanel.tsx`).
const FIRST_PAGE_ROWS: u64 = 50;
/// `PRESSURE_SOURCE_CONTRACT_ID` (`numericalResultQuality.ts`).
const PRESSURE_SOURCE_CONTRACT_ID: &str = "openpipestress.result_semantics/0.3.0/pressure-source-1";
const RECORD_SCHEMA: &str = "swbpipe.after_worker_harness.run/1";

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

// ---------------------------------------------------------------------------
// The tests: two parents (by source) and the child they run
// ---------------------------------------------------------------------------

/// The native sequence, replayed in order, on a rack from
/// `core/solver/performance_harness/rack/rack_model.py` of [`NODES_ENV`] nodes.
#[test]
#[ignore = "attribution instrument: run by exact filter with SWBPIPE_AFTER_WORKER_RACK_NODES and SWBPIPE_AFTER_WORKER_RECORD"]
fn replay_rack() {
    parent("rack");
}

/// The native sequence, replayed in order, on the model document [`MODEL_ENV`] names.
#[test]
#[ignore = "attribution instrument: run by exact filter with SWBPIPE_AFTER_WORKER_MODEL and SWBPIPE_AFTER_WORKER_RECORD"]
fn replay_document() {
    parent("document");
}

/// The run itself, in a fresh process the parent starts (and captures).
#[test]
#[ignore = "started only by replay_rack or replay_document"]
fn child_entry() {
    let Some(kind) = std::env::var_os(CHILD_ENV) else { return };
    let report = child(kind.to_str().expect("a source kind"));
    eprintln!("{REPORT_MARK}{report}");
}

// ---------------------------------------------------------------------------
// The parent: start the child, capture its stderr, write the record
// ---------------------------------------------------------------------------

fn parent(kind: &str) {
    assert!(cfg!(feature = "native-smoke"), "build with --features native-smoke: the phase lines come from its recorder");
    let record_path = PathBuf::from(std::env::var_os(RECORD_ENV).unwrap_or_else(|| panic!("{RECORD_ENV} must name the record file")));
    let directory = record_path.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new("."));
    let directory = directory.canonicalize().unwrap_or_else(|e| panic!("the record's directory: {e}"));
    let repository = root().canonicalize().unwrap();
    assert!(!directory.starts_with(&repository), "the record must be written outside the repository");
    let load_start = host_load();
    let started = Instant::now();
    let output = Command::new(std::env::current_exe().unwrap())
        .args([CHILD_ENTRY, "--exact", "--ignored", "--nocapture", "--test-threads=1", "--quiet"])
        .env(CHILD_ENV, kind)
        .env(DIAGNOSTICS_ENV, "1")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .expect("the child run starts");
    let wall_ms = started.elapsed().as_secs_f64() * 1000.0;
    let load_end = host_load();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let mut events = Vec::new();
    let mut report = None;
    let mut other_lines = 0usize;
    for line in stderr.lines() {
        if let Some(event) = line.strip_prefix(PHASE_MARK) {
            events.push(serde_json::from_str::<Value>(event).expect("a phase line is JSON"));
        } else if let Some(text) = line.strip_prefix(REPORT_MARK) {
            assert!(report.is_none(), "one report per run");
            report = Some(serde_json::from_str::<Value>(text).expect("the report is JSON"));
        } else {
            other_lines += 1;
        }
    }
    let Some(mut report) = report.filter(|_| output.status.success()) else {
        let tail: Vec<&str> = stderr.lines().rev().take(40).collect::<Vec<_>>().into_iter().rev().collect();
        panic!("the child run failed ({}):\n{}", output.status, tail.join("\n"));
    };
    let phases = phases_by_call(&events);
    attach_phases(&mut report, &phases);
    let record = json!({
        "schema": RECORD_SCHEMA,
        "name": "the native sequence, replayed in order",
        "head": git(&["rev-parse", "HEAD"]),
        "worktree_clean": git(&["status", "--porcelain", "--untracked-files=no"]).map(|s| s.is_empty()),
        "build": {
            "profile": if cfg!(debug_assertions) { "debug_assertions on (not a release build)" } else { "release (debug_assertions off)" },
            "instrumentation": "cfg(test) with --features native-smoke: the reader phase recorder emits its lines; the allocator's counting hook is compiled in and armed only on attribution runs",
        },
        "run_kind": report["run_kind"].clone(),
        "host_load": {"start": load_start, "end": load_end},
        "child_wall_ms": wall_ms,
        "phase_lines": events.len(),
        "phase_overflow": events.iter().any(|e| e.get("overflow").is_some()),
        "other_stderr_lines": other_lines,
        "run": report,
    });
    let mut file = std::fs::OpenOptions::new().create(true).append(true).open(&record_path).expect("the record file opens");
    writeln!(file, "{record}").unwrap();
    println!("after-worker: {}", one_line(&record));
}

/// A short human line for the run (the record holds everything).
fn one_line(record: &Value) -> String {
    let run = &record["run"];
    let calls: Vec<String> = run["calls"].as_array().map(|calls| {
        calls.iter().map(|c| format!("{} {:.0} ms {}", c["call"].as_str().unwrap_or("?"), c["wall_ms"].as_f64().unwrap_or(0.0), c["outcome"].as_str().unwrap_or("?"))).collect()
    }).unwrap_or_default();
    format!("{} {} text {} bytes; outcome {}; {}", run["source"]["kind"], run["source"]["name"], run["result"]["text_bytes"], run["outcome"], calls.join("; "))
}

/// The phase lines grouped by native call, each phase's enter and exit paired.
fn phases_by_call(events: &[Value]) -> std::collections::BTreeMap<u64, Vec<Value>> {
    let mut calls: std::collections::BTreeMap<u64, Vec<Value>> = Default::default();
    let mut open: std::collections::BTreeMap<(u64, String), (f64, u64)> = Default::default();
    for event in events {
        let (Some(call), Some(phase), Some(edge), Some(ms)) = (event["call"].as_u64(), event["phase"].as_str(), event["edge"].as_str(), event["mono_ms"].as_f64()) else { continue };
        let bytes = event["bytes"].as_u64().unwrap_or(0);
        match edge {
            "enter" => {
                open.insert((call, phase.to_string()), (ms, bytes));
            }
            _ => {
                let (entered, bytes) = open.remove(&(call, phase.to_string())).unwrap_or((ms, bytes));
                calls.entry(call).or_default().push(json!({"phase": phase, "status": event["status"], "bytes": bytes, "enter_ms": entered, "duration_ms": ms - entered}));
            }
        }
    }
    calls
}

/// Each diagnosed call's phases, full parses and parse leases' M, from its phase lines.
fn attach_phases(report: &mut Value, phases: &std::collections::BTreeMap<u64, Vec<Value>>) {
    let Some(calls) = report["calls"].as_array_mut() else { return };
    for call in calls {
        let Some(native) = call["native_call"].as_u64() else { continue };
        let list = phases.get(&native).cloned().unwrap_or_default();
        let parses = list.iter().filter(|p| matches!(p["phase"].as_str(), Some("source_parse" | "summary_health"))).count();
        let leases: Vec<u64> = list.iter().filter(|p| p["phase"] == "source_admit").filter_map(|p| p["bytes"].as_u64()).collect();
        call["phases"] = Value::Array(list);
        call["full_parses"] = json!({"count": parses, "basis": "phase lines: source_parse, and summary_health's own parse of the text"});
        call["parse_leases_m"] = json!({"bytes": leases, "basis": "phase lines: source_admit bytes"});
    }
}

fn git(args: &[&str]) -> Option<String> {
    let output = Command::new("git").arg("-C").arg(root()).args(args).stderr(Stdio::null()).output().ok()?;
    output.status.success().then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn sysctl(name: &str) -> Option<String> {
    let output = Command::new("/usr/sbin/sysctl").args(["-n", name]).stderr(Stdio::null()).output().ok()?;
    output.status.success().then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// The host's load averages and memory status level (percent free), read through sysctl.
fn host_load() -> Value {
    json!({"loadavg": sysctl("vm.loadavg"), "memorystatus_level": sysctl("kern.memorystatus_level").and_then(|s| s.parse::<u64>().ok())})
}

// ---------------------------------------------------------------------------
// The child: the solve, the texts, the sequence
// ---------------------------------------------------------------------------

fn hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes).iter().map(|b| format!("{b:02x}")).collect()
}

/// `result_js_text` of `text`, as a string.
fn js_text(text: &str, zero: NegativeZero) -> String {
    let mut out = Vec::new();
    result_js_text::write(text, zero, &mut out).expect("a JSON text");
    String::from_utf8(out).expect("UTF-8")
}

/// The model document's bytes and how they were obtained.
fn source(kind: &str) -> (String, Vec<u8>, Value) {
    match kind {
        "rack" => {
            let nodes: u64 = std::env::var(NODES_ENV).unwrap_or_else(|_| panic!("{NODES_ENV} must name the rack size")).parse().expect("a node count");
            let profile = std::env::var(PROFILE_ENV).unwrap_or_else(|_| "linear-ex".into());
            let generator = root().join("core/solver/performance_harness/rack/rack_model.py");
            let output = Command::new("python3").arg(&generator).args(["--profile", &profile, "--nodes", &nodes.to_string()]).stderr(Stdio::null()).output().expect("the rack generator runs");
            assert!(output.status.success(), "the rack generator failed for {profile} at {nodes} nodes");
            (format!("{profile}-n{nodes:04}"), output.stdout, json!({"kind": "rack", "generator": "core/solver/performance_harness/rack/rack_model.py", "profile": profile, "nodes": nodes}))
        }
        "document" => {
            let path = PathBuf::from(std::env::var_os(MODEL_ENV).unwrap_or_else(|| panic!("{MODEL_ENV} must name a model document")));
            let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("the model document: {e}"));
            let shown = path.canonicalize().ok().and_then(|p| p.strip_prefix(root().canonicalize().unwrap()).ok().map(|p| p.display().to_string()));
            let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
            (name, bytes, json!({"kind": "document", "repository_path": shown}))
        }
        other => panic!("unknown source kind {other}"),
    }
}

/// The texts the web sends, from the document File > Open Model Document delivers (module docs).
struct WebTexts {
    /// `checkedJsonText(capture.invocation.request.model)`: the start body.
    body: String,
    /// `jsonTextKeepingNegativeZero(capture.invocation.request.model)`: the registration's model text.
    registration: String,
    /// `jsonTextKeepingNegativeZero(solveModel)`: the standing's and the summary's model text.
    standing: String,
    /// `jsonTextKeepingNegativeZero(capture.invocation)`: the pressure-source registration's text.
    invocation: String,
    /// The session model as the manifest's `JSON.stringify` reads it (`asJsonText`).
    model: Value,
}

fn web_texts(name: &str, bytes: &[u8]) -> WebTexts {
    let document = match read_model_document_bytes(name.to_string(), bytes) {
        ModelDocumentOpenOutcome::Opened { document, .. } => document,
        ModelDocumentOpenOutcome::Refused { diagnostics, .. } => panic!("the open path refused the document: {:?}", diagnostics.iter().take(5).collect::<Vec<_>>()),
        ModelDocumentOpenOutcome::Cancelled => unreachable!("no chooser"),
    };
    // The IPC reply's document member: serde_json's text of the opened value.
    let delivered = serde_json::to_string(&document).unwrap();
    drop(document);
    let body = js_text(&delivered, NegativeZero::AsZero);
    let registration = js_text(&body, NegativeZero::Keep);
    let standing = js_text(&delivered, NegativeZero::Keep);
    let captured = format!(r#"{{"request":{{"model":{body},"materials":[]}},"solver_mode":"{SOLVER_MODE}"}}"#);
    let invocation = js_text(&js_text(&captured, NegativeZero::AsZero), NegativeZero::Keep);
    let model = serde_json::from_str(&body).unwrap();
    WebTexts { body, registration, standing, invocation, model }
}

/// The two model texts compared, for this document.
fn texts_compared(texts: &WebTexts) -> Value {
    let (a, b) = (texts.registration.as_bytes(), texts.standing.as_bytes());
    let first_difference = a.iter().zip(b).position(|(x, y)| x != y).or_else(|| (a.len() != b.len()).then(|| a.len().min(b.len())));
    json!({
        "registration_equals_standing": a == b,
        "registration": {"bytes": a.len(), "sha256": hex(a), "made_by": "checkedJsonText then jsonTextKeepingNegativeZero of its parse"},
        "standing": {"bytes": b.len(), "sha256": hex(b), "made_by": "jsonTextKeepingNegativeZero of the session model"},
        "first_difference_at": first_difference,
        "start_body_equals_registration": texts.body == texts.registration,
        "basis": "an observation for this document, not a general identity",
    })
}

fn host_memory_bytes() -> u64 {
    sysctl("hw.memsize").and_then(|s| s.parse().ok()).expect("hw.memsize")
}

fn wait_terminal(registry: &SolveJobRegistry, job_id: &str) -> crate::solve_jobs::SolveJobStatusReport {
    let deadline = Instant::now() + Duration::from_secs(3600);
    loop {
        let status = registry.status(job_id).expect("job known");
        if !matches!(status.state, "queued" | "running") {
            return status;
        }
        assert!(Instant::now() < deadline, "job {job_id} did not end within an hour");
        std::thread::sleep(Duration::from_millis(20));
    }
}

fn child(kind: &str) -> Value {
    let counting = std::env::var(COUNTING_ENV).as_deref() == Ok("1");
    let (name, bytes, mut source) = source(kind);
    source["name"] = json!(name);
    source["document_bytes"] = json!(bytes.len());
    source["document_sha256"] = json!(hex(&bytes));
    let texts = web_texts(&name, &bytes);
    drop(bytes);
    let mut report = json!({
        "run_kind": if counting { "attribution (allocation counting on)" } else { "timing (allocation counting off)" },
        "source": source,
        "model_texts": texts_compared(&texts),
        "omitted": ["IPC transfer", "web bookkeeping and caches", "React commits and rendering", "the two animation frames to first paint", "the panel's concurrent kind-presence lookups"],
        "host_pool": "ScriptedHost reporting this host's physical memory as capacity and headroom: real headroom refusals are removed; the fixed 12 GiB cap applies",
    });
    let memory = host_memory_bytes();
    let registry = crate::solve_jobs::test_support::real_registry(ScriptedHost::with(Some(memory), Some(memory), PressureLevel::Normal));
    report["host_pool_capacity_bytes"] = json!(memory);
    let solve_started = Instant::now();
    let receipt = match registry.start_with_custody(texts.body.as_bytes(), Some(SOLVER_MODE), ResultCustody::Native) {
        Ok(receipt) => receipt,
        Err(error) => {
            report["outcome"] = json!("start_refused");
            report["start_error"] = serde_json::to_value(&error).unwrap_or(Value::Null);
            return report;
        }
    };
    let status = wait_terminal(&registry, &receipt.job_id);
    report["solve"] = json!({"wall_ms": solve_started.elapsed().as_secs_f64() * 1000.0, "status": serde_json::to_value(&status).unwrap()});
    if status.state != "completed" {
        report["outcome"] = json!(format!("solve_{}", status.state));
        return report;
    }
    let job = receipt.job_id.clone();
    let held = registry.held_result(&job).expect("the completed native job holds its result");
    let text_bytes = held.text().len();
    let before = json!({
        "phys_footprint": worker_alloc::phys_footprint(),
        "lifetime_max_footprint": worker_alloc::lifetime_max_footprint(),
        "held_text_bytes": text_bytes,
        "pool_reserved_bytes": registry.pool().reserved_total(),
    });
    drop(held);
    // The kept replies: one subdirectory per run, named in the record (never the base directory).
    let replies = std::env::var_os(REPLIES_ENV).map(|base| {
        let base = PathBuf::from(base);
        assert!(!base.canonicalize().expect("the replies directory exists").starts_with(root().canonicalize().unwrap()), "replies are kept outside the repository");
        let stamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        let name = format!("{}-{}-{stamp}", report["source"]["name"].as_str().unwrap_or("run"), if counting { "attribution" } else { "timing" });
        std::fs::create_dir(base.join(&name)).expect("the run's replies directory is new");
        report["replies"] = json!({"run_directory": name});
        Replies { directory: base.join(name), entries: Vec::new() }
    });
    if replies.is_none() {
        report["replies"] = json!({"run_directory": null, "web_acceptance": "not judged: the replies were not kept"});
    }
    let mut sequence = Sequence { registry: &registry, job: &job, texts: &texts, origin: Instant::now(), counting, calls: Vec::new(), diagnosed: 0, replies };
    let (outcome, allocation) = if counting {
        let (outcome, measured) = ingress::counting::measure(|| sequence.run());
        (outcome, json!({
            "whole_sequence": measured,
            "basis": "ingress::counting over the whole sequence in this thread: requested bytes from zero at the sequence's start, read with phase marks at each call's edges (live_*: running live bytes; peak_during: peak since the call's start mark; peak_between: peak since the previous call's end mark). Counting starts at ingress::counting::measure around the sequence: not seen is what existed before it (the held result text, the registry, the model texts), and other threads' allocations (the registry's supervisor and poll threads; no reader command on this path spawns a thread). Inside the count but outside every call's own marks: each call's reply write and structural pre-check (calls[].allocation.post_call), and the input manifest built before run_preview (in run_preview's peak_between).",
        }))
    } else {
        (sequence.run(), Value::Null)
    };
    let sequence_ms = sequence.origin.elapsed().as_secs_f64() * 1000.0;
    let calls = std::mem::take(&mut sequence.calls);
    if let Some(replies) = sequence.replies.take() {
        let manifest = json!({"schema": "swbpipe.after_worker_harness.replies/1", "head": git(&["rev-parse", "HEAD"]), "job_id": job, "source": report["source"].clone(), "run_kind": report["run_kind"].clone(), "entries": replies.entries});
        std::fs::write(replies.directory.join("replies.json"), format!("{manifest}\n")).expect("the replies manifest is written");
        report["replies"]["entries"] = json!(manifest["entries"].as_array().map_or(0, Vec::len));
        report["replies"]["web_acceptance"] = json!("pre-checked");
        report["replies"]["web_acceptance_basis"] = json!("pre-checked until afterWorkerReplies.test.ts's verdict over this run directory is attached; only a passing verdict makes it web-accepted");
    }
    let held = registry.held_result(&job).expect("the result is still held");
    let text = held.text();
    let m = ingress::prescan_allowances(text.as_bytes()).map(|(bound, _)| bound.materialization_bytes).ok();
    report["result"] = json!({"text_bytes": text.len(), "text_sha256": hex(text.as_bytes()), "parse_lease_m_by_prescan": m, "m_over_text": m.map(|m| m as f64 / text.len() as f64)});
    drop(held);
    report["outcome"] = outcome;
    report["calls"] = Value::Array(calls);
    report["sequence_wall_ms"] = json!(sequence_ms);
    report["before_sequence"] = before;
    report["allocation"] = allocation;
    report["footprint_basis"] = json!("phys_footprint at call edges is a point reading, not a per-call peak; lifetime_max_footprint is this process's maximum since it started (cumulative), not the call's; the worker is another process and is in neither");
    report["timing_basis"] = json!("calls[].wall_ms is the native call alone; calls[].post_call_ms (the reply write and the pre-check) is outside it and inside sequence_wall_ms");
    report["capture_overhead"] = json!({
        "inside each call's own marks": "the request's Value and its raw body (about twice args_bytes; the webview's ingress hands the command the same body) and the HeaderMap the command reads",
        "post_call (its own mark)": "the reply write (no copy) and the structural pre-check's transient; replies are written and dropped before the next call, never held",
        "held across calls": "the records and the replies manifest entries (kilobytes), and the pre-checks' small decoded results (the route, standing, producer, omitted members, first-page ordinals)",
        "between calls, in the next call's peak_between": "the input manifest built before run_preview (a canonical copy of the model, a few MB)",
    });
    report["parse_lease_basis"] = json!("M for the header, registration, standing and summary leases is their source_admit phase line; run_preview's parse, the detail index's route parse and the semantics' working lease have no phase line: their M is result.parse_lease_m_by_prescan (the same prescan bound of the same text), the semantics' lease adding its rows and working allowance");
    report["full_parse_basis"] = json!("counted from phase lines where they exist; from source, run_preview parses the text once, the first rows query once (the detail index's route table, kept on the share afterwards) and the row semantics once");
    report["after_sequence"] = json!({"phys_footprint": worker_alloc::phys_footprint(), "lifetime_max_footprint": worker_alloc::lifetime_max_footprint(), "pool_reserved_bytes": registry.pool().reserved_total()});
    let _ = registry.dispose_result(&job);
    report
}

/// A refusal the web reads as `ResourceUnavailableError` (the session records it and stops).
fn resource_refusal(error: &CommandError<String>) -> Option<&ResourceRefusal> {
    match error {
        CommandError::Ingress(SolveJobError::ResourceUnavailable { refusal, .. }) => Some(refusal),
        _ => None,
    }
}

struct Sequence<'a> {
    registry: &'a SolveJobRegistry,
    job: &'a str,
    texts: &'a WebTexts,
    origin: Instant,
    counting: bool,
    calls: Vec<Value>,
    diagnosed: u64,
    /// Where each reply is kept for the web's decoders (`afterWorkerReplies.test.ts`).
    replies: Option<Replies>,
}

/// The kept replies of one run: its directory and, per reply, its file and request.
struct Replies {
    directory: PathBuf,
    entries: Vec<Value>,
}

/// A native reply as the webview receives it: its bytes, and how the web reads them
/// (`json`: Tauri delivers the parsed JSON value; `bin`: an ArrayBuffer; `txt`: UTF-8 text).
trait Reply {
    const KIND: &'static str;
    fn bytes(&self) -> &[u8];
}
impl Reply for String {
    const KIND: &'static str = "json";
    fn bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}
impl Reply for Vec<u8> {
    const KIND: &'static str = "bin";
    fn bytes(&self) -> &[u8] {
        self
    }
}
/// The row semantics' JSON text, which Tauri sends as a raw body (`semantics_response`).
struct SemanticsText(String);
impl Reply for SemanticsText {
    const KIND: &'static str = "txt";
    fn bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

/// A call's reply as the web reads it: `Ok` with the decoded parts the sequence uses next.
type Accepted = Result<Value, String>;

impl Sequence<'_> {
    fn ms(&self) -> f64 {
        self.origin.elapsed().as_secs_f64() * 1000.0
    }

    /// One native call: its reservation and blocking part in this thread, its
    /// edges, footprints and (attribution runs) phase marks; then, outside its
    /// clock and marks, the kept reply and the structural pre-check. `Err` ends
    /// the sequence with that outcome.
    fn call<T: Reply>(
        &mut self,
        name: &str,
        diagnosed: bool,
        args: Value,
        run: impl FnOnce(&SolveJobRegistry, &InvokeBody) -> Result<T, CommandError<String>>,
        accept: impl FnOnce(&T) -> Accepted,
    ) -> Result<Value, Value> {
        let body = InvokeBody::Raw(serde_json::to_vec(&args).unwrap());
        let args_bytes = match &body {
            InvokeBody::Raw(raw) => raw.len(),
            _ => 0,
        };
        let native_call = diagnosed.then(|| {
            self.diagnosed += 1;
            self.diagnosed
        });
        let start_mark = self.counting.then(ingress::counting::phase_mark);
        let (foot_start, max_start) = (worker_alloc::phys_footprint(), worker_alloc::lifetime_max_footprint());
        let start_ms = self.ms();
        let started = Instant::now();
        let outcome = run(self.registry, &body);
        let wall_ms = started.elapsed().as_secs_f64() * 1000.0;
        let end_ms = self.ms();
        let (foot_end, max_end) = (worker_alloc::phys_footprint(), worker_alloc::lifetime_max_footprint());
        let end_mark = self.counting.then(ingress::counting::phase_mark);
        let mut record = json!({
            "call": name, "native_call": native_call, "args_bytes": args_bytes,
            "start_ms": start_ms, "end_ms": end_ms, "wall_ms": wall_ms,
            "footprint": {"phys_start": foot_start, "phys_end": foot_end, "lifetime_max_start": max_start, "lifetime_max_end": max_end},
        });
        // After the call's clock and marks: the reply is kept (when a replies directory is set) and
        // pre-checked. Both run under the counter on attribution runs; their own mark below keeps
        // them out of this call's peak_during and out of the next call's peak_between.
        let post_started = Instant::now();
        let result = match outcome {
            Ok(reply) => {
                record["reply_bytes"] = json!(reply.bytes().len());
                self.keep(&mut record, name, T::KIND, reply.bytes(), &body);
                match accept(&reply) {
                    Ok(decoded) => {
                        record["outcome"] = json!("pre_check_passed");
                        Ok(decoded)
                    }
                    Err(why) => {
                        record["outcome"] = json!("pre_check_failed");
                        record["why"] = json!(why);
                        Err(json!({"ended_at": name, "kind": "pre_check_failed", "why": why}))
                    }
                }
            }
            Err(error) => {
                let refusal = resource_refusal(&error).map(|r| serde_json::to_value(r).unwrap());
                let required = match resource_refusal(&error) {
                    Some(ResourceRefusal::IngressExceedsBudget { required_bytes, budget_bytes }) => Some((*required_bytes, *budget_bytes)),
                    _ => None,
                };
                record["outcome"] = json!(if refusal.is_some() { "resource_refusal" } else { "command_error" });
                record["error"] = serde_json::to_value(&error).unwrap_or(Value::Null);
                // The rejection as the webview receives it: the error's serialized value.
                let rejection = serde_json::to_vec(&error).unwrap_or_default();
                self.keep(&mut record, name, "error", &rejection, &body);
                if let Some((required, budget)) = required {
                    record["required_bytes"] = json!(required);
                    record["budget_bytes"] = json!(budget);
                }
                Err(json!({"ended_at": name, "kind": record["outcome"], "refusal": refusal, "required_bytes": required.map(|r| r.0), "budget_bytes": required.map(|r| r.1), "error": record["error"]}))
            }
        };
        record["post_call_ms"] = json!(post_started.elapsed().as_secs_f64() * 1000.0);
        let post_mark = self.counting.then(ingress::counting::phase_mark);
        if let (Some((live_start, peak_between)), Some((live_end, peak_during)), Some((live_post, peak_post))) = (start_mark, end_mark, post_mark) {
            record["allocation"] = json!({
                "live_start": live_start, "live_end": live_end, "peak_during": peak_during, "peak_between": peak_between,
                "post_call": {"live_after": live_post, "peak": peak_post, "basis": "the reply write and the structural pre-check, after the call's end mark: their own peak, not the call's"},
            });
        }
        self.calls.push(record);
        result
    }

    /// Keeps one reply (or rejection) and the request bytes it answered, bound to its run and its
    /// position in the call order, for the web's decoders; nothing is kept past this call.
    fn keep(&mut self, record: &mut Value, name: &str, kind: &str, bytes: &[u8], body: &InvokeBody) {
        let Some(replies) = self.replies.as_mut() else { return };
        let seq = replies.entries.len() + 1;
        let file = format!("{seq:02}-{name}.{kind}");
        let args_file = format!("{seq:02}-{name}.args.json");
        std::fs::write(replies.directory.join(&file), bytes).expect("the reply is written");
        let InvokeBody::Raw(raw) = body else { unreachable!("the harness sends raw bodies") };
        std::fs::write(replies.directory.join(&args_file), raw).expect("the request is written");
        replies.entries.push(json!({"seq": seq, "call": name, "file": file, "kind": kind, "bytes": bytes.len(), "args_file": args_file}));
        record["reply_file"] = json!(file);
    }

    fn verdict(&mut self, name: &str, args: Value, accept: impl FnOnce(&String) -> Accepted) -> Result<Value, Value> {
        self.call(name, true, args, |registry, body| {
            let (raw, lease) = native_verdict::reserve(registry, body, &headers(Some(INGRESS_ARGS_JSON_V1)))?;
            native_verdict::run(registry.clone(), raw, lease)
        }, accept)
    }

    fn summary(&mut self, standing: &Value) -> Result<Value, Value> {
        let args = json!({"jobId": self.job, "modelJson": self.texts.standing, "standing": {"eligible": standing["eligible"], "findings": standing["findings"]}});
        self.call("summary", true, args, |registry, body| {
            let (raw, lease) = result_summary::reserve(registry, body, &headers(Some(INGRESS_ARGS_JSON_V1)))?;
            result_summary::run(registry.clone(), raw, lease)
        }, |reply| accept_summary(reply))
    }

    /// The standing's request (`nativeStandingInputs`): the session model's text, and the captured
    /// invocation's text only for a registered pressure-source statement.
    fn standing_args(&self, route: &str, registered: bool) -> Value {
        let mut args = json!({"jobId": self.job, "modelJson": self.texts.standing});
        if route == "pressure_source" && registered {
            args["invocationJson"] = json!(self.texts.invocation);
        }
        args
    }

    fn run(&mut self) -> Value {
        match self.replay() {
            Ok(done) => done,
            Err(ended) => ended,
        }
    }

    /// The web's order (`readNativeFreshResult`, then the session, then the panel's first page).
    fn replay(&mut self) -> Result<Value, Value> {
        let job = self.job.to_string();
        // 1. header
        let header = self.verdict("header", json!({"jobId": job, "operation": "route_header"}), |reply| accept_route_header(reply, &job))?;
        let route = header["route"].as_str().unwrap_or_default().to_string();
        // 2. registration (`validateNativeReaders`); a non-resource refusal is its code, not an end.
        let reader = match route.as_str() {
            "pressure_source" => Some(json!({"jobId": job, "reader": "pressure-source", "invocationJson": self.texts.invocation})),
            "preview_physics" => Some(json!({"jobId": job, "reader": "preview-physics", "modelJson": self.texts.registration})),
            "pressure" => Some(json!({"jobId": job, "reader": "pressure", "modelJson": self.texts.registration})),
            _ => None,
        };
        let readers: Option<String> = match reader {
            None => None,
            Some(args) => match self.verdict("registration", args, accept_validation) {
                Ok(outcome) => outcome["code"].as_str().map(str::to_string),
                Err(ended) if ended["kind"] == "command_error" => Some("UNCODED_REFUSAL".into()),
                Err(ended) => return Err(ended),
            },
        };
        // 3. standing, 4. summary
        let mut standing_args = self.standing_args(&route, readers.is_none());
        let mut verdict = self.verdict("standing", standing_args.clone(), accept_dispatch)?;
        let mut summary = self.summary(&verdict["standing"])?;
        let envelope_refusal = envelope_refusal(&summary["envelope"], &route, &self.texts.model);
        let unregistered = readers.clone().or(envelope_refusal);
        if unregistered.is_some() && readers.is_none() {
            standing_args = self.standing_args(&route, false);
            verdict = self.verdict("standing_again", standing_args.clone(), accept_dispatch)?;
            summary = self.summary(&verdict["standing"])?;
        }
        // The dispatch read again with the final inputs: the web's cache answers an unchanged key.
        let final_args = self.standing_args(&route, unregistered.is_none());
        if final_args != standing_args {
            verdict = self.verdict("dispatch_again", final_args, accept_dispatch)?;
        }
        if let Err(why) = route_header_agreement(&header, &verdict) {
            return Err(json!({"ended_at": "header_agreement", "kind": "pre_check_failed", "why": why}));
        }
        // Source recovery, only where the web reads it.
        let omitted = summary["omitted_members"].as_array().is_some_and(|m| m.iter().any(|m| m == "source_block_recovery"));
        if route == "pressure_source" || omitted {
            let args = json!({"jobId": job, "offset": 0, "limit": 1, "raw": false});
            self.call("source_recovery", false, args, |registry, body| {
                let (raw, lease) = result_detail::reserve(registry, body, &headers(Some(INGRESS_ARGS_JSON_V1)), result_detail::RECOVERY_COMMAND)?;
                result_detail::run_recovery(registry.clone(), raw, lease)
            }, |reply| accept_detail(reply, "swbpipe.desktop.result_source_recovery").map(|_| Value::Null))?;
        }
        // 5. run_preview, with the session's input manifest (`buildCurrentSessionInputManifest`).
        let manifest = input_manifest(&self.texts.model, &summary["envelope"]["producer"]);
        let args = json!({"jobId": job, "inputManifest": manifest, "ruleCheckAggregate": null, "rulePackExclusions": []});
        let preview = self.call("run_preview", false, args, |registry, body| {
            let (raw, lease) = analysis_run_v03_build::reserve(registry, body, &headers(Some(INGRESS_ARGS_JSON_V1)))?;
            analysis_run_v03_build::run(registry.clone(), raw, lease)
        }, |reply| accept_run_preview(reply))?;
        // 6. the first rows page and its row semantics (`NativeResultsPanel`).
        let filter = json!({"text": "", "family": "all", "frame": "all", "case": "all"});
        let rows = self.call("rows_page", false, json!({"jobId": job, "filter": filter, "offset": 0, "limit": FIRST_PAGE_ROWS}), |registry, body| {
            let (raw, lease) = result_detail::reserve(registry, body, &headers(Some(INGRESS_ARGS_JSON_V1)), result_detail::ROWS_COMMAND)?;
            result_detail::run_rows(registry.clone(), raw, lease)
        }, |reply| accept_rows(reply))?;
        let ordinals = rows["ordinals"].clone();
        let returned = ordinals.as_array().map_or(0, Vec::len);
        if returned > 0 {
            let count = returned;
            self.call("row_semantics", false, json!({"jobId": job, "ordinals": ordinals}), |registry, body| {
                let (raw, lease) = result_detail::reserve(registry, body, &headers(Some(INGRESS_ARGS_JSON_V1)), result_detail::SEMANTICS_COMMAND)?;
                result_detail::run_semantics(registry.clone(), raw, lease).map(SemanticsText)
            }, |reply| accept_semantics(&reply.0, count))?;
        }
        Ok(json!({
            "ended_at": "first_page_ready",
            "kind": "first_page_ready",
            "acceptance": "structural pre-checks passed; the web's acceptance is the vitest verdict over the kept replies",
            "route": route,
            "registered": unregistered.is_none(),
            "unregistered_reason": unregistered,
            "standing": verdict["standing"],
            "run_preview": preview,
            "first_page": {"total": rows["total"], "returned": returned},
        }))
    }
}

// ---------------------------------------------------------------------------
// Structural pre-checks (not the web's acceptance: that is afterWorkerReplies.test.ts)
// ---------------------------------------------------------------------------

fn object<'v>(value: &'v Value, what: &str) -> Result<&'v serde_json::Map<String, Value>, String> {
    value.as_object().ok_or_else(|| format!("{what} is not an object"))
}

/// Structural pre-check after `decodeNativeRouteHeader` (which judges the kept reply).
fn accept_route_header(reply: &str, job: &str) -> Accepted {
    let value: Value = serde_json::from_str(reply).map_err(|e| e.to_string())?;
    let map = object(&value, "the route header")?;
    let keys = ["document_kind", "version", "job_id", "header_status", "route", "binding", "refusal"];
    if map.len() != keys.len() || !keys.iter().all(|k| map.contains_key(*k)) || value["document_kind"] != "swbpipe.desktop.result_route_header" || value["version"] != 1 || value["job_id"] != job {
        return Err("not this job's distinct route header".into());
    }
    match value["header_status"].as_str() {
        Some("recognized") => {
            let binding = &value["binding"];
            let sha_ok = binding["sha256"].as_str().is_some_and(|s| s.len() == 64 && s.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)));
            if !value["route"].is_string() || !binding["id"].is_string() || !sha_ok || !value["refusal"].is_null() {
                return Err("inconsistent recognized route/binding".into());
            }
        }
        Some("refused") => {
            if value["route"] != "unsupported" || !value["binding"].is_null() || !value["refusal"]["code"].is_string() {
                return Err("inconsistent refused route header".into());
            }
        }
        _ => return Err("unknown route-header status".into()),
    }
    Ok(value)
}

/// Structural pre-check after `decodeNativeValidation` (which judges the kept reply), then
/// `validateNativeReaders`' reading: `{code}` null when it passed.
fn accept_validation(reply: &String) -> Accepted {
    let value: Value = serde_json::from_str(reply).map_err(|e| e.to_string())?;
    match value["ok"].as_bool() {
        Some(true) => Ok(json!({"code": null})),
        Some(false) => value["error"]["code"].as_str().map(|code| json!({"code": code})).ok_or_else(|| "a refusal without a code".into()),
        None => Err("not a reader validation outcome".into()),
    }
}

/// Structural pre-check after `decodeNativeVerdict` (which judges the kept reply).
fn accept_dispatch(reply: &String) -> Accepted {
    let value: Value = serde_json::from_str(reply).map_err(|e| e.to_string())?;
    let standing = &value["standing"];
    if !value["ok"].is_boolean() || !value["route"].is_string() || !standing.is_object() {
        return Err("not a dispatch outcome".into());
    }
    if !standing["eligible"].is_boolean() || !standing["findings"].is_array() {
        return Err("the standing is not readable".into());
    }
    Ok(json!({"ok": value["ok"], "route": value["route"], "binding": value["binding"], "error": value["error"], "standing": standing}))
}

/// Structural pre-check after `requireRouteHeaderAgreement` (which judges the kept replies).
fn route_header_agreement(header: &Value, verdict: &Value) -> Result<(), String> {
    let binding = if header["binding"].is_null() { verdict["binding"].is_null() } else { header["binding"]["id"] == verdict["binding"]["id"] && header["binding"]["sha256"] == verdict["binding"]["sha256"] };
    let refusal = if header["refusal"].is_null() { verdict["error"].is_null() } else { header["refusal"]["code"] == verdict["error"]["code"] && header["refusal"]["detail"] == verdict["error"]["detail"] };
    if header["route"] != verdict["route"] || (header["header_status"] == "recognized") != (verdict["ok"] == true) || !binding || !refusal {
        return Err("route header differs from the compulsory full verdict".into());
    }
    Ok(())
}

/// `nativeEnvelopeRefusal` (its model-reference and pressure-source identity checks; the
/// retained routes are named by route).
fn envelope_refusal(envelope: &Value, route: &str, model: &Value) -> Option<String> {
    if envelope["model_ref"] != model["project"]["id"] {
        return Some("CAPTURE_MISMATCH".into());
    }
    if route != "pressure_source" && envelope["producer"]["semantic_contract_id"] == PRESSURE_SOURCE_CONTRACT_ID {
        return Some(if route == "unsupported" { "SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED" } else { "SOURCE_BLOCKS_CONTRACT" }.into());
    }
    if route.starts_with("retained_") {
        return Some("RETAINED_PRECISION_NATIVE_REGISTRATION_UNAVAILABLE".into());
    }
    None
}

fn le_u64(bytes: &[u8], at: usize) -> Option<u64> {
    bytes.get(at..at + 8).map(|b| u64::from_le_bytes(b.try_into().unwrap()))
}

/// A structural pre-check of S's framing and the header members the sequence reads; the web's
/// `decodeResultSummary` (planes, descriptors and values) judges the kept reply.
fn accept_summary(reply: &[u8]) -> Accepted {
    // Only the members the sequence reads are deserialized; serde_json skips every other member
    // (the tables, the values, the envelope's carried members) without building a tree, so the
    // pre-check holds nothing of S's size after it returns (Agent 0's condition 5).
    #[derive(serde::Deserialize)]
    struct Head {
        document_kind: String,
        version: u64,
        planes_bytes: u64,
        planes: Vec<serde::de::IgnoredAny>,
        envelope: Envelope,
        omitted_members: Vec<String>,
    }
    #[derive(serde::Deserialize)]
    struct Envelope {
        #[serde(default)]
        model_ref: Value,
        #[serde(default)]
        producer: Option<Producer>,
    }
    #[derive(serde::Deserialize)]
    struct Producer {
        component_name: Option<String>,
        component_version: Option<String>,
        semantic_contract_id: Option<String>,
    }
    if reply.len() < 16 || &reply[..8] != b"SWBSUMv1" {
        return Err("not a result summary".into());
    }
    let length = le_u64(reply, 8).unwrap() as usize;
    let head = 16usize.checked_add(length).filter(|&h| h <= reply.len()).ok_or("the header runs past the end")?;
    let header: Head = serde_json::from_slice(&reply[16..head]).map_err(|e| format!("the header is not the summary's: {e}"))?;
    let start = head + (8 - head % 8) % 8;
    if start > reply.len() || reply[head..start].iter().any(|&b| b != 0) {
        return Err("the padding is not zero".into());
    }
    if header.planes_bytes != (reply.len() - start) as u64 {
        return Err("the planes are not the response's remaining bytes".into());
    }
    if header.document_kind != "swbpipe.desktop.result_summary" || header.version != 1 {
        return Err("another document kind or version".into());
    }
    let _ = header.planes.len();
    let producer = header.envelope.producer.map(|p| json!({"component_name": p.component_name, "component_version": p.component_version, "semantic_contract_id": p.semantic_contract_id}));
    Ok(json!({"envelope": {"model_ref": header.envelope.model_ref, "producer": producer}, "omitted_members": header.omitted_members}))
}

/// Structural pre-check after `decodeAnalysisRunPreview` (which judges the kept reply).
fn accept_run_preview(reply: &[u8]) -> Accepted {
    let header: Value = serde_json::from_slice(reply).map_err(|_| "the response is not UTF-8 JSON".to_string())?;
    if header["document_kind"] != "swbpipe.desktop.analysis_run_preview" || header["version"] != 1 {
        return Err("not an analysis-run preview of this version".into());
    }
    let run = object(&header["record"]["analysis_run"], "record.analysis_run")?;
    for member in ["diagnostics", "result_refs", "contract_evidence", "source_block_recovery", "retained_precision"] {
        if run.contains_key(member) {
            return Err(format!("the header carries record.analysis_run.{member}"));
        }
    }
    for scope in ["analysis_run_record", "received_result"] {
        let claims = run.get("hashes").and_then(Value::as_array).map_or(0, |h| h.iter().filter(|h| h["payload_scope"] == scope).count());
        if claims != 1 {
            return Err(format!("no single {scope} checksum"));
        }
    }
    for member in ["record_bytes", "result_ref_count", "producer_diagnostic_count"] {
        if !header[member].is_u64() {
            return Err(format!("{member} is not a count"));
        }
    }
    Ok(json!({"response_bytes": reply.len(), "record_bytes": header["record_bytes"], "route": header["route"], "result_ref_count": header["result_ref_count"], "rule_check_status": header["rule_check_status"]}))
}

/// Structural pre-check after `decodeDetailResponse`: the header and its texts' lengths.
fn accept_detail(reply: &[u8], kind: &str) -> Result<(Value, usize), String> {
    if reply.len() < 16 || &reply[..8] != b"SWBDETv1" {
        return Err("not a result detail response".into());
    }
    let length = le_u64(reply, 8).unwrap() as usize;
    let mut at = 16usize.checked_add(length).filter(|&h| h <= reply.len()).ok_or("the header runs past the end")?;
    let header: Value = serde_json::from_slice(&reply[16..at]).map_err(|e| format!("the header is not JSON: {e}"))?;
    if header["document_kind"] != kind || header["version"] != 1 {
        return Err("another document kind or version".into());
    }
    let lengths = header["lengths"].as_array().ok_or("no text lengths")?;
    for (i, length) in lengths.iter().enumerate() {
        let end = at.checked_add(length.as_u64().ok_or(format!("text {i}'s length is not a count"))? as usize).filter(|&e| e <= reply.len()).ok_or(format!("text {i} runs past the end"))?;
        std::str::from_utf8(&reply[at..end]).map_err(|_| format!("text {i} is not UTF-8"))?;
        at = end;
    }
    if at != reply.len() {
        return Err("bytes follow the last text".into());
    }
    let texts = lengths.len();
    Ok((header, texts))
}

/// Structural pre-check after `decodeRowsPage` (which judges the kept reply).
fn accept_rows(reply: &[u8]) -> Accepted {
    let (header, texts) = accept_detail(reply, "swbpipe.desktop.result_detail_rows")?;
    let ordinals = header["ordinals"].as_array().ok_or("no ordinals")?;
    if ordinals.len() != texts || header["returned"].as_u64() != Some(texts as u64) || !header["truncated"].is_boolean() {
        return Err("ordinals and texts differ in number".into());
    }
    if ordinals.windows(2).any(|w| !(w[1].as_u64() > w[0].as_u64())) {
        return Err("ordinals do not ascend".into());
    }
    let total = header["total"].as_u64().ok_or("total is not a count")?;
    if texts as u64 > total {
        return Err("more rows than the total".into());
    }
    Ok(json!({"total": total, "ordinals": header["ordinals"]}))
}

/// Structural pre-check after `decodeRowSemantics` (which judges the kept reply).
fn accept_semantics(reply: &String, count: usize) -> Accepted {
    let outcomes: Value = serde_json::from_str(reply).map_err(|_| "the semantics are not JSON".to_string())?;
    let list = outcomes.as_array().filter(|l| l.len() == count).ok_or("not one outcome per row")?;
    if list.iter().any(|o| !o["ok"].is_boolean()) {
        return Err("an outcome without `ok`".into());
    }
    Ok(json!({"rows": count, "ok": list.iter().filter(|o| o["ok"] == true).count()}))
}

/// `buildCurrentSessionInputManifest` for a model with no rule-pack references and no external
/// assets (`NO_RULE_PACK_RESOLUTION`), with the solver basis `workspaceSession.ts` gives it; its
/// canonical bytes are RFC 8785 (`canonical_json`, the wasm engine's `canonicalJsonString` over
/// `JSON.stringify`'s text), which the command checks again (`verify_manifest`).
fn input_manifest(model: &Value, producer: &Value) -> Value {
    let name = producer["component_name"].as_str().unwrap_or_default();
    let version = producer["component_version"].as_str().unwrap_or_default();
    assert!(model["rule_pack_refs"].as_array().is_none_or(Vec::is_empty), "this harness builds the manifest of a model without rule-pack references");
    let manifest = json!({
        "schema_version": "1.0.0",
        "document_kind": "openpipestress.current_session_input_manifest",
        "model_basis": {"model_ref": model["project"]["id"], "model_payload": model},
        "unit_basis": {"project_units": model["project"]["units"]},
        "solver_basis": {
            "solver_name": name, "solver_version": version, "solver_build_ref": format!("{name}@{version}"), "solver_mode": SOLVER_MODE,
            "settings": {"nonlinear_iteration_policy": "DEC-046-CV-B-product-preview-active-set-count-v1", "sparse_evidence_lane": SOLVER_MODE == "sparse_interactive"},
        },
        "load_basis": {"load_cases": model["load_cases"], "combinations": if model["combinations"].is_null() { json!([]) } else { model["combinations"].clone() }},
        "active_rule_packs": [],
        "external_assets": [],
        "identity_policy": {"canonicalization": "rfc8785_jcs", "hash_algorithm": "sha256", "canonical_bytes_scope": "entire_input_manifest_object"},
        "replay_boundary": {"included_as_package_member": false, "portable_replay_claimed": false, "current_session_ref_hash_integrity_only": true},
    });
    let canonical = open_pipe_stress_canonical_json::canonical_json(&manifest);
    let sha256 = hex(canonical.as_bytes());
    let model_ref = model["project"]["id"].as_str().unwrap_or_default();
    json!({"manifest_ref": analysis_runs::input_manifest_ref(model_ref, &sha256), "manifest_sha256": sha256, "canonical_bytes": canonical})
}

/// The harness's own pieces, on small inputs (run with the instrument, by its filter).
#[test]
#[ignore = "attribution instrument: run by exact filter"]
fn the_web_texts_keep_or_drop_negative_zero_as_the_web_does() {
    let document = r#"{"b":[-0.0,1.0,2.5e-7],"a":{"10":1,"2":-0,"x":1e21}}"#;
    assert_eq!(js_text(document, NegativeZero::AsZero), r#"{"b":[0,1,2.5e-7],"a":{"2":0,"10":1,"x":1e+21}}"#);
    assert_eq!(js_text(document, NegativeZero::Keep), r#"{"b":[-0,1,2.5e-7],"a":{"2":-0,"10":1,"x":1e+21}}"#);
    let phases = phases_by_call(&[
        json!({"layer": "native", "call": 1, "phase": "source_admit", "edge": "enter", "status": "pending", "mono_ms": 1.0, "bytes": 80}),
        json!({"layer": "native", "call": 1, "phase": "source_admit", "edge": "exit", "status": "ok", "mono_ms": 3.5, "bytes": 80}),
    ]);
    assert_eq!(phases[&1][0]["duration_ms"], 2.5);
}
