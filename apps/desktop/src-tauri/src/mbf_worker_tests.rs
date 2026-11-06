//! The MBF export's connected path (DEL-17-04 slice 4b.2a): the commands' ingress and ledger, task 3 in a real worker
//! process (this test binary run as the worker, `solve_worker::test_support::TestLauncher`), the decode and the reply,
//! against the maintained fixtures `fixtures/caepipe_mbf/invented/desktop_export/`; then §3.9.6's cancel, newer
//! review, budget, ledger, memory, output-ceiling, protocol and save-refusal cases, and the source scan that keeps
//! [`CappedWorkerRun`] the only production route to `worker_render`.

use super::*;
use crate::atomic_report_package_save::test_support::atomic_save_test_lock;
use crate::command_ingress::test_support::{check_decode_bound, headers};
use crate::command_ingress::INGRESS_ARGS_JSON_V1;
use crate::export_disposition_screen::DispositionDiagnostic as Diagnostic;
use crate::mbf_export::{
    decode_worker_result, review_mbf_export_args, save_mbf_export_args, sha256_hex, worker_render, MbfMode, CANCELLED, COMMAND, IDS_SUFFIX,
    MEMORY_UNAVAILABLE, REVIEW_COMMAND, REVIEW_MEMORY_UNAVAILABLE, REVIEW_STALE, REVIEW_TRUNCATED, WRITER_REFUSED,
};
use crate::memory_pool::test_host::ScriptedHost;
use crate::model_document_save::{ModelDocumentSaveLocalFirst, ModelDocumentSaveScreening};
use crate::solve_worker::test_support::{Gate, TestLauncher};
use crate::solve_worker::{WorkerLauncher, PROCESS_RESERVE_BYTES};
use open_pipe_stress_operation_applier::canonical_json;
use serde_json::{json, Value};
use std::cell::Cell;
use std::path::Path;
use std::sync::atomic::{AtomicBool as Opened, Ordering};

const EXPORT_MODEL: &str = include_str!("../../../../fixtures/caepipe_mbf/invented/desktop_export/export_model.json");
const EXPORT_DIGESTS: &str = include_str!("../../../../fixtures/caepipe_mbf/invented/desktop_export/export_digests.json");
const REVIEW_CASES: &str = include_str!("../../../../fixtures/caepipe_mbf/invented/desktop_export/review_cases.json");
const GIB: u64 = 1 << 30;
const MIB: u64 = 1 << 20;

// -- Fixtures and helpers -------------------------------------------------------------------------------------------

fn canonical_model() -> String {
    canonical_json(&serde_json::from_str::<Value>(EXPORT_MODEL).unwrap())
}

/// The model variants the review cases name (`model_edit`), as `mbf_export`'s tests make them.
fn model_for(edit: &str) -> String {
    let mut document: Value = serde_json::from_str(EXPORT_MODEL).unwrap();
    match edit {
        "none" => {}
        "project.units.temperature = degF" => document["project"]["units"]["temperature"] = json!("degF"),
        "components[0].kind = tee" => document["components"][0]["kind"] = json!("tee"),
        "nodes[last].position.z = 1e306" => {
            let last = document["nodes"].as_array().unwrap().len() - 1;
            document["nodes"][last]["position"]["z"] = json!(1e306);
        }
        other => panic!("unknown model edit {other}"),
    }
    canonical_json(&document)
}

fn settings() -> String {
    r#"{"title":"Invented MBF export","referenceTemperature":{"value":20,"unit":"degC"},"maxLayoutRows":null}"#.to_string()
}

fn hash_claim(text: &str) -> String {
    format!("sha256:{}", sha256_hex(text.as_bytes()))
}

fn review_body(text: &str, settings: &str) -> Value {
    json!({"canonicalText": text, "claimedModelHash": hash_claim(text), "settingsJson": settings})
}

fn screening(text: &str) -> ModelDocumentSaveScreening {
    ModelDocumentSaveScreening {
        route_id: crate::mbf_export::MBF_EXPORT_ROUTE_ID.to_string(),
        export_context: "local_private".to_string(),
        explicit_local_private_intent: true,
        blocked: false,
        materialization_withheld: false,
        lossless_required: true,
        exact_payload_match: true,
        blocking_count: 0,
        payload_sha256: hash_claim(text),
    }
}

fn local_first() -> ModelDocumentSaveLocalFirst {
    ModelDocumentSaveLocalFirst {
        route_id: crate::mbf_export::MBF_EXPORT_ROUTE_ID.to_string(),
        export_context: "local_private".to_string(),
        storage_context: "local_private".to_string(),
        action: "include_metadata_only".to_string(),
        reason_code: "PRIVATE_LOCAL_METADATA_ALLOWED".to_string(),
        blocked: false,
        metadata_only: true,
        explicit_local_private_intent: true,
    }
}

/// The recorded digests of the default case (`export_digests.json`).
fn recorded_default() -> (String, String) {
    let recorded: Value = serde_json::from_str(EXPORT_DIGESTS).unwrap();
    let case = &recorded["cases"]["degc-default"];
    assert_eq!(case["settings_json"], settings());
    assert_eq!(case["model_sha256"], sha256_hex(canonical_model().as_bytes()));
    (case["mbf_sha256"].as_str().unwrap().to_string(), case["sidecar_sha256"].as_str().unwrap().to_string())
}

fn save_body(text: &str, reviewed: (String, String)) -> Value {
    json!({
        "canonicalText": text,
        "claimedModelHash": hash_claim(text),
        "settingsJson": settings(),
        "suggestedName": "swbpipe-model.mbf",
        "reviewed": {"mbfSha256": reviewed.0, "sidecarSha256": reviewed.1},
        "screening": serde_json::to_value(screening(text)).unwrap(),
        "localFirstEvidence": serde_json::to_value(local_first()).unwrap(),
    })
}

/// A registry whose workers are this test binary started by `launcher` (the real task 3, or a scripted workload).
fn registry_with(launcher: Arc<dyn WorkerLauncher>) -> SolveJobRegistry {
    SolveJobRegistry::with_hooks(
        MemoryPool::with_wait_tick(ScriptedHost::normal(), Duration::from_millis(5)),
        launcher,
        Arc::new(|job: Box<dyn FnOnce() + Send + 'static>| std::thread::Builder::new().spawn(job).map(drop)),
    )
}

fn real_worker() -> SolveJobRegistry {
    registry_with(TestLauncher::solve())
}

fn raw_body(body: &Value) -> InvokeBody {
    InvokeBody::Raw(serde_json::to_vec(body).unwrap())
}

/// `body` with the dispatch ticket a current frontend start sends: the epoch now, unless `body` already names one
/// (or names `expectedEpoch: "absent"`, which removes it).
fn ticketed(runs: &MbfRuns, body: &Value) -> Value {
    let mut body = body.clone();
    match body.get(EXPECTED_EPOCH) {
        None => body[EXPECTED_EPOCH] = json!(runs.epoch()),
        Some(value) if value.as_str() == Some("absent") => drop(body.as_object_mut().unwrap().remove(EXPECTED_EPOCH)),
        Some(_) => {}
    }
    body
}

/// `review_mbf_export` as `lib.rs` runs it: the copy's reservation, then the blocking part, for `body` with its
/// ticket ([`ticketed`]).
fn review_via_command(registry: &SolveJobRegistry, runs: &MbfRuns, body: &Value) -> (MbfReviewOutcome, String) {
    let body = ticketed(runs, body);
    let (raw, copy) = reserve(registry, &raw_body(&body), &headers(Some(INGRESS_ARGS_JSON_V1)), REVIEW_COMMAND).unwrap();
    let (outcome, text) = review_command(registry, runs, raw, copy, |outcome, text| (outcome.clone(), text)).unwrap();
    assert_eq!(serde_json::to_string(&outcome).unwrap(), text);
    (outcome, text)
}

/// `save_mbf_export` as `lib.rs` runs it, with no library store; `chosen` is the chooser's answer.
fn save_via_command(registry: &SolveJobRegistry, runs: &MbfRuns, body: &Value, chosen: Option<PathBuf>, opened: &Opened) -> MbfExportOutcome {
    let body = ticketed(runs, body);
    let (raw, copy) = reserve(registry, &raw_body(&body), &headers(Some(INGRESS_ARGS_JSON_V1)), COMMAND).unwrap();
    let (outcome, text) = save_command(
        registry,
        runs,
        raw,
        copy,
        || Err("the test has no library store".to_string()),
        |suggested| {
            assert_eq!(suggested, "swbpipe-model.mbf");
            opened.store(true, Ordering::SeqCst);
            Ok(chosen)
        },
        |outcome, text| (outcome.clone(), text),
    )
    .unwrap();
    assert_eq!(serde_json::to_string(&outcome).unwrap(), text);
    outcome
}

/// `save_via_command` with the chooser `choose` (a test that holds the chooser open).
fn save_choosing(registry: &SolveJobRegistry, runs: &MbfRuns, body: &Value, choose: impl FnOnce(&str) -> Result<Option<PathBuf>, String>) -> MbfExportOutcome {
    let body = ticketed(runs, body);
    let (raw, copy) = reserve(registry, &raw_body(&body), &headers(Some(INGRESS_ARGS_JSON_V1)), COMMAND).unwrap();
    let (outcome, text) = save_command(registry, runs, raw, copy, || Err("the test has no library store".to_string()), choose, |outcome, text| (outcome.clone(), text)).unwrap();
    assert_eq!(serde_json::to_string(&outcome).unwrap(), text);
    outcome
}

/// A cancel that stopped one run before it wrote anything.
const STOPPED_ONE: MbfCancelReceipt = MbfCancelReceipt { cancelled: 1, too_late: 0 };

/// Page `n`'s token: 32 lowercase hex digits, as the frontend makes one per page load.
fn page(n: usize) -> String {
    format!("{:032x}", 0xab00_0000_0000_0000_0000_0000_0000_0000u128 + n as u128)
}

fn retirement(page_n: usize, sequence: u64) -> RetirementId {
    RetirementId { page: page(page_n), sequence }
}

/// `cancel_mbf_export` as `lib.rs` runs it: `body` as an `args-json-v1` raw body through the command's ingress.
fn cancel_via_command(registry: &SolveJobRegistry, runs: &MbfRuns, body: &Value) -> Result<MbfRetirement, CommandError<String>> {
    cancel_command(registry, runs, &raw_body(body), &headers(Some(INGRESS_ARGS_JSON_V1)))
}

/// `cancel_mbf_export`'s request for `id`.
fn cancel_body(id: &RetirementId) -> Value {
    json!({ RETIREMENT_ID: { "page": id.page, "sequence": id.sequence } })
}

/// How Tauri 2.11.1 hands `cancel_mbf_export` an invoke (`scripts/ipc-protocol.js`,
/// `scripts/process-ipc-message-fn.js`, `src/ipc/protocol.rs`).
#[derive(Clone, Copy, Debug)]
enum Wire {
    /// The frontend's JSON form (`invoke(cmd, {retirementId})`) on the custom protocol: the payload posted as
    /// `application/json` text, parsed by Tauri into a JSON body; no ingress header.
    JsonCustomProtocol,
    /// The JSON form after the transport fell back to `postMessage`: the message (payload included) through
    /// `JSON.stringify`, parsed by Tauri, its `payload` the JSON body and its `options.headers` the headers.
    JsonFallback,
    /// The raw form (`invoke(cmd, Uint8Array, {headers})`) after the fallback: `JSON.stringify` turns the bytes into an
    /// array of numbers (the replacer's `Array.from(val)`), and the ingress header travels in `options.headers`.
    RawFallback,
}

/// The body and headers `cancel_mbf_export` receives for the arguments `args` sent over `wire`, decoded as Tauri does.
fn tauri_wire(wire: Wire, args: &Value) -> (InvokeBody, tauri::http::HeaderMap) {
    if let Wire::JsonCustomProtocol = wire {
        let posted = serde_json::to_vec(args).unwrap();
        return (InvokeBody::from(serde_json::from_slice::<Value>(&posted).unwrap()), headers(None));
    }
    let (payload, sent_headers) = match wire {
        Wire::RawFallback => (json!(serde_json::to_vec(args).unwrap()), json!({ crate::command_ingress::INGRESS_HEADER: INGRESS_ARGS_JSON_V1 })),
        _ => (args.clone(), json!({})),
    };
    let message = json!({
        "cmd": CANCEL_COMMAND, "callback": 1, "error": 2, "payload": payload,
        "options": { "headers": sent_headers, "customProtocolIpcBlocked": true }, "__TAURI_INVOKE_KEY__": "key",
    })
    .to_string();
    #[derive(serde::Deserialize)]
    struct Options {
        #[serde(default)]
        headers: std::collections::HashMap<String, String>,
    }
    #[derive(serde::Deserialize)]
    struct Message {
        payload: Value,
        options: Option<Options>,
    }
    let message: Message = serde_json::from_str(&message).unwrap();
    let mut received = tauri::http::HeaderMap::new();
    for (name, value) in message.options.map(|options| options.headers).unwrap_or_default() {
        received.insert(tauri::http::HeaderName::from_bytes(name.as_bytes()).unwrap(), tauri::http::HeaderValue::from_str(&value).unwrap());
    }
    (InvokeBody::from(message.payload), received)
}

/// `cancel_mbf_export` for `args` sent over `wire`.
fn cancel_over(wire: Wire, registry: &SolveJobRegistry, runs: &MbfRuns, args: &Value) -> Result<MbfRetirement, CommandError<String>> {
    let (body, received) = tauri_wire(wire, args);
    cancel_command(registry, runs, &body, &received)
}

/// An ingress refusal's message.
fn ingress_message(result: Result<MbfRetirement, CommandError<String>>) -> String {
    match result {
        Err(CommandError::Ingress(SolveJobError::InvalidRequest { message })) => message,
        other => panic!("{other:?}"),
    }
}

/// Whether any tracked run has been cancelled.
fn any_tracked_cancelled(runs: &MbfRuns) -> bool {
    runs.lock().runs.iter().any(|(_, run)| run.cancelled())
}

fn review_codes(outcome: &MbfReviewOutcome) -> Vec<String> {
    match outcome {
        MbfReviewOutcome::Refused { diagnostics } => diagnostics.iter().map(|item| item.code.clone()).collect(),
        MbfReviewOutcome::Reviewed { .. } => Vec::new(),
    }
}

fn save_diagnostics(outcome: &MbfExportOutcome) -> Vec<Diagnostic> {
    match outcome {
        MbfExportOutcome::Refused { diagnostics, .. } => diagnostics.clone(),
        _ => Vec::new(),
    }
}

fn save_codes(outcome: &MbfExportOutcome) -> Vec<String> {
    save_diagnostics(outcome).into_iter().map(|item| item.code).collect()
}

/// Nothing the run took stays reserved, and no worker is left.
fn assert_released(registry: &SolveJobRegistry) {
    assert_eq!(registry.pool_status().reserved_bytes, 0, "every lease is released");
    assert_eq!(registry.pool().running_workers(), 0, "every worker is reaped");
}

struct Sandbox(PathBuf);

impl Sandbox {
    fn new(label: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "swbpipe-mbf-worker-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
        ));
        std::fs::create_dir_all(&path).unwrap();
        Sandbox(path)
    }

    fn is_empty(&self) -> bool {
        std::fs::read_dir(&self.0).unwrap().next().is_none()
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// The envelope the real task 3 forms for the default model (as a gate's scripted outcome).
fn envelope(mode: MbfMode) -> String {
    worker_render(&WorkerRequest { mode, settings_json: &settings(), model_text: &canonical_model() }.encode()).unwrap()
}

/// Waits until the pool records `matches` as its last refusal (a run is in the queue wait).
fn await_queued(registry: &SolveJobRegistry, matches: impl Fn(&ResourceRefusal) -> bool) {
    let deadline = Instant::now() + Duration::from_secs(60);
    while !registry.pool_status().last_refusal.as_ref().is_some_and(&matches) {
        assert!(Instant::now() < deadline, "the run never waited");
        std::thread::sleep(Duration::from_millis(2));
    }
}

/// Cancels `runs` if a test's own thread panics inside a scope, so a run it started ends (its worker killed) and the
/// scope's join returns: a failed assertion fails the test instead of leaving it waiting on its worker.
struct CancelOnUnwind<'a>(&'a MbfRuns, &'a SolveJobRegistry);

impl Drop for CancelOnUnwind<'_> {
    fn drop(&mut self) {
        if std::thread::panicking() {
            self.0.cancel_all(self.1.pool());
        }
    }
}

/// Runs `run` on another thread and fails the test if it has not ended by itself within a minute (cancelling it, so
/// its worker ends and the thread returns).
fn ends_by_itself<T: Send>(registry: &SolveJobRegistry, runs: &MbfRuns, run: impl FnOnce() -> T + Send) -> T {
    std::thread::scope(|scope| {
        let running = scope.spawn(run);
        let deadline = Instant::now() + Duration::from_secs(60);
        while !running.is_finished() {
            if Instant::now() >= deadline {
                runs.cancel_all(registry.pool());
                let _ = running.join();
                panic!("the run did not end by itself");
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        running.join().unwrap()
    })
}

/// Joins `running`, failing the test (after cancelling `runs`, so its worker ends and the join returns) if it has not
/// ended within 30 s: a run nothing cancels fails its test instead of hanging it.
fn join_within<T>(registry: &SolveJobRegistry, runs: &MbfRuns, running: std::thread::ScopedJoinHandle<'_, T>, what: &str) -> T {
    let deadline = Instant::now() + Duration::from_secs(30);
    while !running.is_finished() {
        if Instant::now() >= deadline {
            runs.cancel_all(registry.pool());
            let _ = running.join();
            panic!("{what} was not cancelled");
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    running.join().unwrap()
}

fn worker_alive(pid: i32) -> bool {
    std::process::Command::new("/bin/kill")
        .args(["-0", &pid.to_string()])
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}

// -- The connected path ---------------------------------------------------------------------------------------------

/// Replaces 4b.1's `the_reviews_production_entry_point_refuses_not_connected`. `review_mbf_export` end to end: the
/// raw body's ingress and ledger, task 3 in `review` mode in a real worker process, the decode and the reply. Each
/// review case formed under the production limits (`review_cases.json`) comes back byte for byte, its digests are the
/// recorded ones, the result lease is held while the response is built, and everything is released after.
#[test]
fn the_review_command_runs_task_3_in_the_capped_worker_and_replies_the_recorded_reviews() {
    let registry = real_worker();
    let runs = MbfRuns::default();
    let cases: Value = serde_json::from_str(REVIEW_CASES).unwrap();
    let recorded: Value = serde_json::from_str(EXPORT_DIGESTS).unwrap();
    let mut connected = 0;
    for (name, case) in cases["cases"].as_object().unwrap() {
        let production = case["review_allowance"] == mbf::MBF_REVIEW_MAX_BYTES && case["output_ceiling"] == mbf::MBF_EXPORT_OUTPUT_MAX_BYTES;
        if !production {
            // The truncated and over-ceiling variants use a test bound only the in-process seam can set.
            continue;
        }
        let text = model_for(case["model_edit"].as_str().unwrap());
        assert_eq!(case["model_sha256"], sha256_hex(text.as_bytes()), "{name}");
        let settings = case["settings_json"].as_str().unwrap();
        let body = ticketed(&runs, &review_body(&text, settings));
        let (raw, copy) = reserve(&registry, &raw_body(&body), &headers(Some(INGRESS_ARGS_JSON_V1)), REVIEW_COMMAND).unwrap();
        let held_at_response = Cell::new(0);
        let (outcome, response) = review_command(&registry, &runs, raw, copy, |outcome, text| {
            // Result custody holds through the response's construction.
            held_at_response.set(registry.pool().reserved(LeasePurpose::Result));
            (outcome.clone(), text)
        })
        .unwrap();
        assert_eq!(serde_json::to_string(&outcome).unwrap(), response, "{name}");
        let MbfReviewOutcome::Reviewed { review } = &outcome else { panic!("{name}: {outcome:?}") };
        assert_eq!(review.get(), case["review_text"].as_str().unwrap(), "{name}: the review is the recorded one, byte for byte");
        assert!(held_at_response.get() >= review.get().len() as u64, "{name}: the result lease is held at the response");
        let parsed: Value = serde_json::from_str(review.get()).unwrap();
        if let Some(digests) = recorded["cases"].get(name.as_str()).filter(|digests| digests["status"] == "written") {
            assert_eq!(parsed["mbf_sha256"], digests["mbf_sha256"], "{name}");
            assert_eq!(parsed["sidecar_sha256"], digests["sidecar_sha256"], "{name}");
        }
        assert_released(&registry);
        connected += 1;
    }
    assert_eq!(connected, 6, "every production-limit case ran connected");
    // The review command's arguments decode within their bound.
    let raw = serde_json::to_vec(&review_body(&canonical_model(), &settings())).unwrap();
    let _ = check_decode_bound(&raw, ArgsDecoding::Moved, |args| review_mbf_export_args(args).map(drop));
}

/// Replaces 4a's `the_production_entry_point_refuses_not_connected_and_touches_nothing`. `save_mbf_export` end to end
/// with a real worker: the route evidence, the admission, task 3 in `save` mode, the staleness check against the
/// recorded digests, the chooser and the atomic writes. The files are the recorded digests' and the engine's bytes;
/// the review a save replies with on a refusal comes from the same worker; Cancel writes nothing.
#[test]
fn the_save_command_writes_the_recorded_files_from_the_capped_workers_result() {
    let _lock = atomic_save_test_lock();
    let sandbox = Sandbox::new("save");
    let registry = real_worker();
    let runs = MbfRuns::default();
    let text = canonical_model();
    let recorded = recorded_default();
    let chosen = sandbox.0.join("swbpipe-model.mbf");
    let opened = Opened::new(false);
    let outcome = save_via_command(&registry, &runs, &save_body(&text, recorded.clone()), Some(chosen.clone()), &opened);
    let MbfExportOutcome::Saved { mbf, identity_file, .. } = &outcome else { panic!("{outcome:?}") };
    assert!(opened.load(Ordering::SeqCst));
    let ids_path = sandbox.0.join(format!("swbpipe-model{IDS_SUFFIX}"));
    let (written_mbf, written_ids) = (std::fs::read(&chosen).unwrap(), std::fs::read(&ids_path).unwrap());
    assert_eq!((sha256_hex(&written_mbf), sha256_hex(&written_ids)), recorded, "the files are the recorded digests'");
    assert_eq!((mbf.sha256.clone(), identity_file.sha256.clone()), recorded);
    let engine = envelope(MbfMode::Save);
    let decoded = decode_worker_result(MbfMode::Save, &engine).unwrap();
    assert_eq!((written_mbf.as_slice(), written_ids.as_slice()), (decoded.mbf.as_bytes(), decoded.identity_file.as_bytes()), "the engine's bytes");
    assert_released(&registry);
    // Cancel in the chooser writes nothing.
    let cancel = Sandbox::new("save-cancel");
    let opened = Opened::new(false);
    let outcome = save_via_command(&registry, &runs, &save_body(&text, recorded), None, &opened);
    assert_eq!(outcome, MbfExportOutcome::Cancelled);
    assert!(opened.load(Ordering::SeqCst) && cancel.is_empty());
    assert_released(&registry);
    let raw = serde_json::to_vec(&save_body(&text, recorded_default())).unwrap();
    let _ = check_decode_bound(&raw, ArgsDecoding::Moved, |args| save_mbf_export_args(args).map(drop));
}

/// The commands' ingress is C1's: a raw `args-json-v1` body only, refused typed before any copy, lease or worker.
#[test]
fn the_commands_refuse_a_body_without_the_ingress_header() {
    let gate = Gate::new("mbf-ingress");
    let registry = registry_with(gate.launcher());
    for command in [REVIEW_COMMAND, COMMAND] {
        let body = raw_body(&review_body(&canonical_model(), &settings()));
        let missing = reserve(&registry, &body, &headers(None), command).unwrap_err();
        assert!(matches!(&missing, SolveJobError::InvalidRequest { message } if message.starts_with("INGRESS-HEADER-REQUIRED")), "{missing:?}");
        let json = reserve(&registry, &InvokeBody::Json(json!({})), &headers(Some(INGRESS_ARGS_JSON_V1)), command).unwrap_err();
        assert!(matches!(&json, SolveJobError::InvalidRequest { message } if message.starts_with("INGRESS-RAW-BODY-REQUIRED")), "{json:?}");
        // A body that is not one object is refused after the copy's lease, which is released.
        let (raw, copy) = reserve(&registry, &InvokeBody::Raw(b"[1]".to_vec()), &headers(Some(INGRESS_ARGS_JSON_V1)), command).unwrap();
        let runs = MbfRuns::default();
        let refused = review_command(&registry, &runs, raw, copy, |_, text| text).unwrap_err();
        assert!(matches!(&refused, SolveJobError::InvalidRequest { message } if message.starts_with("INGRESS-ARGS-NOT-OBJECT")), "{refused:?}");
    }
    assert_eq!(gate.started_count(), 0);
    assert_released(&registry);
}

// -- The ledger (DEL-14-01 r4) --------------------------------------------------------------------------------------

/// A command's memory seeded with an argument lease of `bytes` (admitted under the budget in force).
fn seeded(registry: &SolveJobRegistry, bytes: u64) -> (NativeLeaseMemory<'_>, CommandLease) {
    NativeLeaseMemory::seeded(registry, registry.acquire_interactive_ingress_capped(bytes).unwrap())
}

fn review_request(text: &str, settings: &str) -> (String, String) {
    (text.to_string(), settings.to_string())
}

/// While the worker runs, the command's ledger holds its whole budget (the argument lease and the bound for the
/// request copy, the grant up to its maximum and the result lease) and the pool holds no more than the ledger counts;
/// once the worker is reaped the bound is the result lease's alone, and after the response the argument lease's.
#[test]
fn the_grant_request_and_result_are_counted_in_the_commands_ledger() {
    let gate = Gate::new("mbf-ledger");
    let registry = registry_with(gate.launcher());
    let runs = MbfRuns::default();
    let (_memory, args) = seeded(&registry, 3 * MIB);
    let ledger = Arc::clone(args.ledger());
    let (text, settings) = review_request(&canonical_model(), &settings());
    let request = WorkerRequest { mode: MbfMode::Review, settings_json: &settings, model_text: &text };
    let reply = envelope(MbfMode::Review);
    std::thread::scope(|scope| {
        let _guard = CancelOnUnwind(&runs, &registry);
        let running = scope.spawn(|| CappedWorkerRun::review(&registry, &ledger, &runs, runs.entry_now()).run(&request));
        gate.wait_started(1);
        assert_eq!(ledger.total_bytes(), ledger.budget_bytes(), "the bound takes the rest of the captured budget");
        assert!(registry.pool_status().reserved_bytes <= ledger.total_bytes(), "the pool holds no more than the ledger counts");
        gate.release(&reply);
        let output = running.join().unwrap().unwrap();
        let custody = output.custody.as_ref().expect("a worker result is held");
        assert_eq!(output.text, reply);
        assert_eq!(ledger.total_bytes(), args.bytes() + custody.lease_bytes(), "the bound shrank to the result lease");
        assert_eq!(registry.pool_status().reserved_bytes, args.bytes() + custody.lease_bytes());
        assert!(custody.lease_bytes() <= RESULT_LEASE_BYTES);
        drop(output);
        assert_eq!(ledger.total_bytes(), args.bytes());
    });
    drop(args);
    assert_released(&registry);
}

/// The admission's exact boundary: with the command's argument lease, its request copy, the largest result lease
/// and the starting grant one byte above the captured budget, the run is refused by name before any worker starts
/// and the ledger is as before; at the budget the worker starts.
#[test]
fn a_run_one_byte_above_the_budget_is_refused_before_any_worker_and_one_at_it_starts() {
    let gate = Gate::new("mbf-boundary");
    let (text, settings) = review_request(&canonical_model(), &settings());
    let request = WorkerRequest { mode: MbfMode::Review, settings_json: &settings, model_text: &text };
    let argument = 3 * MIB;
    let start = crate::solve_worker::initial_grant(PROCESS_RESERVE_BYTES, resident_envelope(request.encoded_len())).unwrap();
    let required = argument + allocation_envelope(request.encoded_len()).unwrap() + RESULT_LEASE_BYTES + start;
    for (budget, admitted) in [(required - 1, false), (required, true)] {
        let registry = registry_with(gate.launcher());
        registry.configure_pool(12 * GIB, budget, false).unwrap();
        let runs = MbfRuns::default();
        let (_memory, args) = seeded(&registry, argument);
        let ledger = Arc::clone(args.ledger());
        assert_eq!(ledger.budget_bytes(), budget);
        let started = gate.started_count();
        std::thread::scope(|scope| {
            let _guard = CancelOnUnwind(&runs, &registry);
            let running = scope.spawn(|| CappedWorkerRun::review(&registry, &ledger, &runs, runs.entry_now()).run(&request));
            if admitted {
                gate.wait_started(started + 1);
                assert_eq!(ledger.total_bytes(), budget);
                assert_eq!(runs.cancel_all(registry.pool()), STOPPED_ONE);
                assert!(matches!(running.join().unwrap(), Err(WriterRunFailure::Cancelled)));
            } else {
                let Err(WriterRunFailure::Memory(message)) = running.join().unwrap() else { panic!("not refused") };
                let expected = format!(r#"{{"kind":"ingress_exceeds_budget","required_bytes":{required},"budget_bytes":{budget}}}"#);
                assert!(message.contains(&expected), "{message}");
                assert_eq!(gate.started_count(), started, "no worker started");
            }
        });
        assert_eq!(ledger.total_bytes(), argument);
        drop(args);
        assert_released(&registry);
    }
}

/// The budget is captured with the argument lease: raised while a review waits, it does not extend the run's bound.
#[test]
fn a_budget_raised_while_a_review_waits_does_not_extend_it() {
    let gate = Gate::new("mbf-budget");
    let registry = registry_with(gate.launcher());
    registry.configure_pool(12 * GIB, GIB, false).unwrap();
    let runs = MbfRuns::default();
    let (_memory, args) = seeded(&registry, 3 * MIB);
    let ledger = Arc::clone(args.ledger());
    registry.configure_pool(12 * GIB, 4 * GIB, false).unwrap();
    registry.pool().set_max_running_workers(0);
    let (text, settings) = review_request(&canonical_model(), &settings());
    let request = WorkerRequest { mode: MbfMode::Review, settings_json: &settings, model_text: &text };
    std::thread::scope(|scope| {
        let _guard = CancelOnUnwind(&runs, &registry);
        let running = scope.spawn(|| CappedWorkerRun::review(&registry, &ledger, &runs, runs.entry_now()).run(&request));
        await_queued(&registry, |refusal| matches!(refusal, ResourceRefusal::WorkerLimitReached { .. }));
        assert_eq!((ledger.budget_bytes(), ledger.total_bytes()), (GIB, GIB), "the bound is the captured budget's rest");
        assert_eq!(runs.cancel_all(registry.pool()), STOPPED_ONE);
        assert!(matches!(running.join().unwrap(), Err(WriterRunFailure::Cancelled)));
    });
    assert_eq!(ledger.total_bytes(), args.bytes());
    assert_eq!(gate.started_count(), 0);
    drop(args);
    assert_released(&registry);
}

// -- Cancellation and supersession ----------------------------------------------------------------------------------

/// A review waiting for its starting grant stops waiting when cancelled, well before its wait would end, holding
/// nothing it took; a wait that reaches its deadline is refused by the last queue reason, as a memory refusal.
#[test]
fn a_review_waiting_for_its_grant_is_cancelled_or_refused_at_its_deadline() {
    let gate = Gate::new("mbf-queue");
    let registry = registry_with(gate.launcher());
    registry.pool().set_max_running_workers(0);
    let runs = MbfRuns::default();
    let body = review_body(&canonical_model(), &settings());
    std::thread::scope(|scope| {
        let _guard = CancelOnUnwind(&runs, &registry);
        let began = Instant::now();
        let waiting = scope.spawn(|| review_via_command(&registry, &runs, &body));
        await_queued(&registry, |refusal| matches!(refusal, ResourceRefusal::WorkerLimitReached { .. }));
        assert_eq!(runs.cancel_all(registry.pool()), STOPPED_ONE);
        let (outcome, _) = waiting.join().unwrap();
        assert_eq!(review_codes(&outcome), vec![CANCELLED.to_string()]);
        assert!(began.elapsed() < MBF_START_WAIT, "the wait ended at the cancellation");
    });
    assert_eq!(runs.cancel_all(registry.pool()), MbfCancelReceipt::default(), "the slot is clear once the review has ended");
    assert_released(&registry);
    // The deadline: the queue reason stands as the review's memory refusal.
    let (_memory, args) = seeded(&registry, 3 * MIB);
    let (text, settings) = review_request(&canonical_model(), &settings());
    let request = WorkerRequest { mode: MbfMode::Review, settings_json: &settings, model_text: &text };
    let refused = CappedWorkerRun::review(&registry, args.ledger(), &runs, runs.entry_now()).with_start_wait(Duration::from_millis(30)).run(&request);
    let Err(WriterRunFailure::Memory(message)) = refused else { panic!("not refused") };
    assert!(message.contains("worker_limit_reached"), "{message}");
    assert_eq!(args.ledger().total_bytes(), args.bytes());
    drop(args);
    assert_eq!(gate.started_count(), 0);
    assert_released(&registry);
}

/// A newer review cancels the running one: its worker is killed and reaped, its grant released, and it replies
/// `MBF-EXPORT-CANCELLED`; the newer review completes. A cancellation during a save's worker run ends it the same way,
/// before any chooser.
#[test]
fn a_newer_review_kills_the_running_worker_and_completes() {
    let gate = Gate::new("mbf-supersede");
    let registry = registry_with(gate.launcher());
    let runs = MbfRuns::default();
    let body = review_body(&canonical_model(), &settings());
    let reply = envelope(MbfMode::Review);
    std::thread::scope(|scope| {
        let _guard = CancelOnUnwind(&runs, &registry);
        let older = scope.spawn(|| review_via_command(&registry, &runs, &body));
        gate.wait_started(1);
        let first = gate.pids()[0];
        let newer = scope.spawn(|| review_via_command(&registry, &runs, &body));
        let (outcome, _) = older.join().unwrap();
        assert_eq!(review_codes(&outcome), vec![CANCELLED.to_string()]);
        assert!(!worker_alive(first), "the superseded worker was killed and reaped");
        gate.wait_started(2);
        gate.release(&reply);
        let (outcome, _) = newer.join().unwrap();
        let MbfReviewOutcome::Reviewed { review } = outcome else { panic!("{outcome:?}") };
        assert_eq!(review.get(), decode_worker_result(MbfMode::Review, &reply).unwrap().review);
    });
    assert_released(&registry);
    // A save's worker, cancelled: no chooser, nothing written.
    let sandbox = Sandbox::new("save-cancelled");
    let opened = Opened::new(false);
    let save = save_body(&canonical_model(), recorded_default());
    std::thread::scope(|scope| {
        let _guard = CancelOnUnwind(&runs, &registry);
        let saving = scope.spawn(|| save_via_command(&registry, &runs, &save, Some(sandbox.0.join("swbpipe-model.mbf")), &opened));
        gate.wait_started(3);
        assert_eq!(runs.cancel_all(registry.pool()), STOPPED_ONE);
        assert_eq!(save_codes(&saving.join().unwrap()), vec![CANCELLED.to_string()]);
    });
    assert!(!opened.load(Ordering::SeqCst) && sandbox.is_empty());
    assert_released(&registry);
}

/// Finding P2-1 (review at 9c5785a6d): a save superseded while its chooser is open writes nothing. Save A's worker has
/// ended and its chooser is held open when save B registers (B's worker has started); accepting A's chooser then
/// refuses `MBF-EXPORT-CANCELLED` at the commit boundary, with no file written. B completes later and writes the
/// recorded files; every lease, grant and ledger charge is released.
#[test]
fn a_save_superseded_while_its_chooser_is_open_writes_nothing() {
    let _lock = atomic_save_test_lock();
    let gate = Gate::new("mbf-chooser");
    let registry = registry_with(gate.launcher());
    let runs = MbfRuns::default();
    let (older_dir, newer_dir) = (Sandbox::new("superseded"), Sandbox::new("superseding"));
    let body = save_body(&canonical_model(), recorded_default());
    let reply = envelope(MbfMode::Save);
    let newer = std::thread::scope(|scope| {
        let _guard = CancelOnUnwind(&runs, &registry);
        let (opened_tx, opened_rx) = std::sync::mpsc::channel::<()>();
        let (accept_tx, accept_rx) = std::sync::mpsc::channel::<()>();
        let older_path = older_dir.0.join("swbpipe-model.mbf");
        let older = scope.spawn(|| {
            save_choosing(&registry, &runs, &body, move |_| {
                opened_tx.send(()).unwrap();
                let _ = accept_rx.recv_timeout(Duration::from_secs(60));
                Ok(Some(older_path))
            })
        });
        gate.wait_started(1);
        gate.release(&reply);
        opened_rx.recv_timeout(Duration::from_secs(60)).expect("save A's chooser opened");
        let newer_path = newer_dir.0.join("swbpipe-model.mbf");
        let newer = scope.spawn(|| save_choosing(&registry, &runs, &body, move |_| Ok(Some(newer_path))));
        // B registers before its worker starts, so A is superseded by now.
        gate.wait_started(2);
        accept_tx.send(()).unwrap();
        let outcome = older.join().unwrap();
        assert_eq!(save_codes(&outcome), vec![CANCELLED.to_string()], "{outcome:?}");
        assert!(older_dir.is_empty(), "the superseded save wrote nothing");
        gate.release(&reply);
        join_within(&registry, &runs, newer, "save B")
    });
    let MbfExportOutcome::Saved { mbf, identity_file, .. } = &newer else { panic!("{newer:?}") };
    assert_eq!((mbf.sha256.clone(), identity_file.sha256.clone()), recorded_default(), "the late save completes");
    assert!(older_dir.is_empty());
    assert_eq!(runs.cancel_all(registry.pool()), MbfCancelReceipt::default());
    assert_released(&registry);
}

/// Finding P2-1's other side: a cancel after the save's commit is too late. It reaches the save on its own thread
/// just after the commit (the test's `AFTER_COMMIT` hook), is reported `too_late`, and does not interrupt the write:
/// both files are written with the recorded digests, and everything is released after.
#[test]
fn a_cancel_after_the_commit_is_too_late_and_the_save_completes() {
    let _lock = atomic_save_test_lock();
    let sandbox = Sandbox::new("after-commit");
    let registry = real_worker();
    let runs = MbfRuns::default();
    let receipt = Arc::new(Mutex::new(None));
    let (hook_runs, hook_pool, hook_receipt) = (runs.clone(), Arc::clone(registry.pool()), Arc::clone(&receipt));
    AFTER_COMMIT.with(|hook| *hook.borrow_mut() = Some(Box::new(move || *hook_receipt.lock().unwrap() = Some(hook_runs.cancel_all(&hook_pool)))));
    let opened = Opened::new(false);
    let chosen = sandbox.0.join("swbpipe-model.mbf");
    let outcome = save_via_command(&registry, &runs, &save_body(&canonical_model(), recorded_default()), Some(chosen.clone()), &opened);
    AFTER_COMMIT.with(|hook| hook.borrow_mut().take());
    assert_eq!(*receipt.lock().unwrap(), Some(MbfCancelReceipt { cancelled: 0, too_late: 1 }), "the cancel came after the commit");
    let MbfExportOutcome::Saved { mbf, identity_file, .. } = &outcome else { panic!("{outcome:?}") };
    assert_eq!((mbf.sha256.clone(), identity_file.sha256.clone()), recorded_default());
    let ids_path = sandbox.0.join(format!("swbpipe-model{IDS_SUFFIX}"));
    assert_eq!((sha256_hex(&std::fs::read(&chosen).unwrap()), sha256_hex(&std::fs::read(&ids_path).unwrap())), recorded_default());
    assert_eq!(runs.cancel_all(registry.pool()), MbfCancelReceipt::default());
    assert_released(&registry);
}

/// Repair 5 (option a): the performance session reset (`dispose_all_preview_mechanics_results`, whose registry part is
/// `SolveJobRegistry::dispose_all_results`) does not touch the MBF export: a running review is not cancelled, its
/// worker stays alive, and the retirement epoch does not move, so a delayed duplicate of the reset cannot cancel a fresh
/// MBF run. The page's keyed retirement (`cancel_mbf_export`, sent when the new page's workspace mounts) is what
/// retires the old page's runs: it stops the review, its worker is killed and reaped, and everything is released.
#[test]
fn the_session_reset_leaves_the_mbf_export_alone_and_the_keyed_mount_retirement_stops_it() {
    let gate = Gate::new("mbf-reset");
    let registry = registry_with(gate.launcher());
    let runs = MbfRuns::default();
    let review = review_body(&canonical_model(), &settings());
    std::thread::scope(|scope| {
        let _guard = CancelOnUnwind(&runs, &registry);
        let reviewing = scope.spawn(|| review_via_command(&registry, &runs, &review));
        gate.wait_started(1);
        let pid = gate.pids()[0];
        let epoch = runs.epoch();
        for _ in 0..2 {
            let disposal = registry.dispose_all_results();
            assert_eq!(disposal.abandoned_jobs, 0, "no solve job was in progress");
        }
        assert_eq!(runs.epoch(), epoch, "the reset moved no MBF epoch");
        assert!(!any_tracked_cancelled(&runs) && worker_alive(pid), "the reset cancelled no MBF run");
        let mount = cancel_via_command(&registry, &runs, &cancel_body(&retirement(1, 1))).unwrap();
        assert_eq!((mount.receipt, mount.epoch, mount.duplicate), (STOPPED_ONE, epoch + 1, false), "the new page's mount retirement");
        let (reviewed, _) = join_within(&registry, &runs, reviewing, "the running review");
        assert_eq!(review_codes(&reviewed), vec![CANCELLED.to_string()]);
        assert!(!worker_alive(pid), "the review's worker was killed and reaped");
    });
    assert_eq!(runs.cancel_all(registry.pool()), MbfCancelReceipt::default(), "nothing is tracked");
    assert_released(&registry);
}

/// The MBF export and the hanger sizing (DEL-04-11 increment 3c part 2) keep independent retirement slots: the MBF
/// export's in `MbfRuns`, the hanger sizing's in `SolveJobRegistry::hanger_sizing_runs`. `cancel_mbf_export` neither
/// moves the hanger sizing's epoch nor takes its current run, and `cancel_hanger_sizing` neither moves `MbfRuns`'s epoch
/// nor cancels a tracked MBF run; each retirement still retires its own.
#[test]
fn the_mbf_and_hanger_sizing_retirements_each_retire_only_their_own_runs() {
    let registry = real_worker();
    let runs = MbfRuns::default();
    let hanger = registry.hanger_sizing_runs();
    let (mbf_epoch, hanger_epoch) = (runs.epoch(), hanger.epoch());
    let mbf_run = runs.register(RunKind::Review, runs.entry_now(), registry.pool());
    let hanger_run = hanger.register(hanger.entry_now()).unwrap();

    let mbf = cancel_via_command(&registry, &runs, &cancel_body(&retirement(1, 1))).unwrap();
    assert_eq!((mbf.receipt, mbf.epoch), (STOPPED_ONE, mbf_epoch + 1), "cancel_mbf_export retires the MBF run");
    assert_eq!((hanger.epoch(), hanger.current_ticket()), (hanger_epoch, Some(hanger_epoch)), "cancel_mbf_export leaves the hanger sizing alone");
    drop(mbf_run);

    let fresh = runs.register(RunKind::Review, runs.entry_now(), registry.pool());
    let sizing = crate::hanger_sizing_job::cancel_command(&registry, Some(json!({"page": page(1), "sequence": 1}))).unwrap();
    assert_eq!((sizing.cancelled, sizing.epoch, sizing.duplicate), (true, hanger_epoch + 1, false), "cancel_hanger_sizing retires the hanger run");
    assert_eq!(hanger.current_ticket(), None);
    assert_eq!(runs.epoch(), mbf_epoch + 1, "cancel_hanger_sizing moved no MBF epoch");
    assert!(!any_tracked_cancelled(&runs), "cancel_hanger_sizing cancelled no MBF run");
    drop((fresh, hanger_run));
    assert_eq!(runs.cancel_all(registry.pool()), MbfCancelReceipt::default(), "nothing is tracked");
    assert_released(&registry);
}

/// Finding P2-2 (b): `cancel_mbf_export`'s body (`MbfRuns::cancel_all`) with nothing in progress does nothing; with a
/// running review it stops it (`{cancelled: 1, too_late: 0}`), the worker is killed and reaped, nothing is written and
/// everything is released; a second cancel finds nothing.
#[test]
fn cancel_mbf_export_stops_the_current_run_and_is_a_no_op_with_nothing_running() {
    let gate = Gate::new("mbf-cancel-command");
    let registry = registry_with(gate.launcher());
    let runs = MbfRuns::default();
    assert_eq!(runs.cancel_all(registry.pool()), MbfCancelReceipt::default(), "nothing in progress");
    assert_released(&registry);
    let body = review_body(&canonical_model(), &settings());
    std::thread::scope(|scope| {
        let _guard = CancelOnUnwind(&runs, &registry);
        let reviewing = scope.spawn(|| review_via_command(&registry, &runs, &body));
        gate.wait_started(1);
        let pid = gate.pids()[0];
        assert_eq!(runs.cancel_all(registry.pool()), STOPPED_ONE);
        let (outcome, _) = join_within(&registry, &runs, reviewing, "the review");
        assert_eq!(review_codes(&outcome), vec![CANCELLED.to_string()]);
        assert!(!worker_alive(pid));
    });
    assert_eq!(runs.cancel_all(registry.pool()), MbfCancelReceipt::default());
    assert_eq!(serde_json::to_value(STOPPED_ONE).unwrap(), json!({"cancelled": 1, "too_late": 0}), "the command's reply");
    assert_released(&registry);
}

/// How a test retires the session from inside a command (on its own thread, at a hook): the retirement itself
/// (`MbfRuns::cancel_all`), or `cancel_mbf_export` as `lib.rs` runs it, keyed with a new page's first id (the
/// page-load mount retirement).
#[derive(Clone, Copy, Debug)]
enum Retire {
    Cancel,
    PageMount,
}

fn retire_from_hook(how: Retire, registry: &SolveJobRegistry, runs: &MbfRuns) -> Box<dyn FnOnce()> {
    let (registry, runs) = (registry.clone(), runs.clone());
    Box::new(move || match how {
        Retire::Cancel => drop(runs.cancel_all(registry.pool())),
        Retire::PageMount => {
            // A fresh page token each time: a new page load.
            let next = runs.lock().applied.len() + 1;
            let reply = cancel_via_command(&registry, &runs, &cancel_body(&retirement(next, 1))).unwrap();
            assert!(!reply.duplicate && !reply.exhausted);
        }
    })
}

/// Finding P2 at 6ccc68aff (1), the later checkpoints: the review and the save carry the ticket current when their
/// request was built, and the session is retired while each is paused before its admission (so its ticket is stale
/// when it is read), or after the ticket was accepted and before its run registers (the tests' hooks), by
/// the retirement or by `cancel_mbf_export`'s keyed page-mount retirement. Resumed, each replies `MBF-EXPORT-CANCELLED`: the review's run is not registered, the save
/// opens no chooser and writes nothing, and everything is released. With the real worker, a command that ignored
/// its entry would complete (and the save open its chooser): the test fails instead of waiting.
#[test]
fn a_command_retired_before_its_admission_or_registration_is_cancelled_and_writes_nothing() {
    let _lock = atomic_save_test_lock();
    let registry = real_worker();
    type Hook = &'static std::thread::LocalKey<std::cell::RefCell<Option<Box<dyn FnOnce()>>>>;
    let points: [(&str, Hook); 2] = [("before admission", &BEFORE_ADMISSION), ("before registration", &BEFORE_REGISTRATION)];
    for how in [Retire::Cancel, Retire::PageMount] {
        for (point, hook) in points {
            let runs = MbfRuns::default();
            hook.with(|slot| *slot.borrow_mut() = Some(retire_from_hook(how, &registry, &runs)));
            let (reviewed, _) = review_via_command(&registry, &runs, &review_body(&canonical_model(), &settings()));
            assert!(hook.with(|slot| slot.borrow().is_none()), "{how:?} {point}: the hook ran");
            assert_eq!(review_codes(&reviewed), vec![CANCELLED.to_string()], "{how:?} {point}: {reviewed:?}");

            let sandbox = Sandbox::new("retired-entry");
            let opened = Opened::new(false);
            hook.with(|slot| *slot.borrow_mut() = Some(retire_from_hook(how, &registry, &runs)));
            let saved = save_via_command(&registry, &runs, &save_body(&canonical_model(), recorded_default()), Some(sandbox.0.join("swbpipe-model.mbf")), &opened);
            assert!(hook.with(|slot| slot.borrow().is_none()), "{how:?} {point}: the hook ran");
            assert_eq!(save_codes(&saved), vec![CANCELLED.to_string()], "{how:?} {point}: {saved:?}");
            assert!(!opened.load(Ordering::SeqCst), "{how:?} {point}: no chooser");
            assert!(sandbox.is_empty(), "{how:?} {point}: nothing written");
            assert_eq!(runs.cancel_all(registry.pool()), MbfCancelReceipt::default(), "{how:?} {point}: nothing was tracked");
            assert_released(&registry);
        }
    }
}

/// Finding P2 at 6ccc68aff (2), native side; the ticket's fresh start: a retirement does not reach a request whose
/// ticket is the epoch after it. After a retirement (and after `cancel_mbf_export`'s keyed page-mount retirement) a review carrying the
/// current epoch completes and a save writes the recorded files; an entry from before the retirement is still refused
/// when it registers.
#[test]
fn a_command_that_enters_after_a_retirement_is_not_affected_by_it() {
    let _lock = atomic_save_test_lock();
    let registry = real_worker();
    let runs = MbfRuns::default();
    let before = runs.entry_now();
    for how in [Retire::Cancel, Retire::PageMount] {
        retire_from_hook(how, &registry, &runs)();
        assert_ne!(runs.entry_now(), before, "{how:?}: the epoch moved on");
        let (reviewed, _) = review_via_command(&registry, &runs, &review_body(&canonical_model(), &settings()));
        assert!(matches!(reviewed, MbfReviewOutcome::Reviewed { .. }), "{how:?}: {reviewed:?}");
        let sandbox = Sandbox::new("after-retirement");
        let opened = Opened::new(false);
        let saved = save_via_command(&registry, &runs, &save_body(&canonical_model(), recorded_default()), Some(sandbox.0.join("swbpipe-model.mbf")), &opened);
        let MbfExportOutcome::Saved { mbf, identity_file, .. } = &saved else { panic!("{how:?}: {saved:?}") };
        assert_eq!((mbf.sha256.clone(), identity_file.sha256.clone()), recorded_default(), "{how:?}");
    }
    // The entry captured before both retirements registers cancelled from the start.
    let stale = runs.register(RunKind::Review, before, registry.pool());
    assert!(stale.control.cancelled());
    assert_eq!(runs.cancel_all(registry.pool()), MbfCancelReceipt::default(), "a stale entry is not tracked");
    drop(stale);
    assert_released(&registry);
}

/// A launcher that counts the workers it starts.
struct CountingLauncher(Arc<dyn WorkerLauncher>, std::sync::atomic::AtomicUsize);

impl WorkerLauncher for CountingLauncher {
    fn command(&self) -> std::io::Result<std::process::Command> {
        self.1.fetch_add(1, Ordering::SeqCst);
        self.0.command()
    }
}

fn counted_real_worker() -> (SolveJobRegistry, Arc<CountingLauncher>) {
    let launcher = Arc::new(CountingLauncher(TestLauncher::solve(), std::sync::atomic::AtomicUsize::new(0)));
    (registry_with(Arc::clone(&launcher) as Arc<dyn WorkerLauncher>), launcher)
}

/// The escaped start (4b.2a repair, the last repair's open matter 1). A request is built with the ticket a start
/// would have sent then; the session is retired, by the retirement (whose reply names the new epoch) or by
/// `cancel_mbf_export`'s keyed page-mount retirement; only then does the command's task start. Both the review and the save reply
/// `MBF-EXPORT-CANCELLED`: no worker starts, no chooser opens, nothing is written or tracked, and everything is
/// released. A command that took the epoch as its task began (the previous design) would complete instead: with the
/// real worker the review would reply its review and the save open its chooser.
#[test]
fn a_request_sent_before_a_retirement_is_cancelled_whenever_its_task_starts() {
    let _lock = atomic_save_test_lock();
    let (registry, launcher) = counted_real_worker();
    let runs = MbfRuns::default();
    for how in [Retire::Cancel, Retire::PageMount] {
        let sent = runs.epoch();
        let review = ticketed(&runs, &review_body(&canonical_model(), &settings()));
        let save = ticketed(&runs, &save_body(&canonical_model(), recorded_default()));
        assert_eq!((review[EXPECTED_EPOCH].as_u64(), save[EXPECTED_EPOCH].as_u64()), (Some(sent), Some(sent)));
        match how {
            Retire::Cancel => {
                let reply = runs.retire(registry.pool());
                assert_eq!(reply.epoch, sent + 1, "the reply names the epoch after the bump");
                assert_eq!(
                    serde_json::to_value(reply).unwrap(),
                    json!({"cancelled": 0, "too_late": 0, "epoch": sent + 1}),
                    "cancel_mbf_export's reply"
                );
            }
            Retire::PageMount => retire_from_hook(how, &registry, &runs)(),
        }
        assert_eq!(runs.epoch(), sent + 1, "{how:?}: the epoch moved on");
        let (reviewed, _) = review_via_command(&registry, &runs, &review);
        assert_eq!(review_codes(&reviewed), vec![CANCELLED.to_string()], "{how:?}: {reviewed:?}");
        let sandbox = Sandbox::new("escaped-start");
        let opened = Opened::new(false);
        let saved = save_via_command(&registry, &runs, &save, Some(sandbox.0.join("swbpipe-model.mbf")), &opened);
        assert_eq!(save_codes(&saved), vec![CANCELLED.to_string()], "{how:?}: {saved:?}");
        assert!(!opened.load(Ordering::SeqCst), "{how:?}: no chooser");
        assert!(sandbox.is_empty(), "{how:?}: nothing written");
        assert_eq!(launcher.1.load(Ordering::SeqCst), 0, "{how:?}: no worker started");
        assert_eq!(runs.cancel_all(registry.pool()), MbfCancelReceipt::default(), "{how:?}: nothing was tracked");
        assert_released(&registry);
    }
}

/// The ticket's protocol: a request with no `expectedEpoch` (or `null`), a malformed one (a string, a negative, a
/// fraction, above u64) or one above the current epoch is refused `MBF-EXPORT-EPOCH-INVALID`, the review and the
/// save alike, before the model is decoded (a model member of the wrong type is not reached), through the normal
/// ingress: no worker, no chooser, nothing tracked, everything released.
#[test]
fn a_request_without_a_valid_current_ticket_is_a_protocol_refusal() {
    let (registry, launcher) = counted_real_worker();
    let runs = MbfRuns::default();
    runs.cancel_all(registry.pool());
    let current = runs.epoch();
    let tickets = [json!("absent"), Value::Null, json!(current.to_string()), json!(-1), json!(0.5), json!(u64::MAX as f64 * 2.0), json!(current + 1)];
    for ticket in tickets {
        for undecodable in [false, true] {
            let mut review = review_body(&canonical_model(), &settings());
            let mut save = save_body(&canonical_model(), recorded_default());
            review[EXPECTED_EPOCH] = ticket.clone();
            save[EXPECTED_EPOCH] = ticket.clone();
            if undecodable {
                review["canonicalText"] = json!(17);
                save["canonicalText"] = json!(17);
            }
            let (reviewed, _) = review_via_command(&registry, &runs, &review);
            assert_eq!(review_codes(&reviewed), vec![crate::mbf_export::EPOCH_INVALID.to_string()], "{ticket} {undecodable}: {reviewed:?}");
            let opened = Opened::new(false);
            let saved = save_via_command(&registry, &runs, &save, Some(std::env::temp_dir().join("never-written.mbf")), &opened);
            assert_eq!(save_codes(&saved), vec![crate::mbf_export::EPOCH_INVALID.to_string()], "{ticket} {undecodable}: {saved:?}");
            assert!(!opened.load(Ordering::SeqCst), "{ticket}: no chooser");
        }
    }
    assert_eq!(launcher.1.load(Ordering::SeqCst), 0, "no worker started");
    assert_eq!(runs.epoch(), current, "a refused ticket moves nothing on");
    assert_eq!(runs.cancel_all(registry.pool()), MbfCancelReceipt::default(), "nothing was tracked");
    assert_released(&registry);
    // `enter` itself, by value: the current epoch is the only one accepted.
    let current = runs.epoch();
    assert_eq!(runs.enter(Some(json!(current))), Ok(runs.entry_now()));
    assert_eq!(runs.enter(Some(json!(current - 1))).unwrap_err().code, CANCELLED);
    for refused in [None, Some(json!(current + 1)), Some(json!("1")), Some(json!(1.0))] {
        assert_eq!(runs.enter(refused.clone()).unwrap_err().code, crate::mbf_export::EPOCH_INVALID, "{refused:?}");
    }
}

/// Finding P3 at f77172935 (2): the retirement epoch never wraps. Seeded at `u64::MAX - 1`, a first retirement moves
/// it to `u64::MAX` as usual; with a review running at that epoch, a second retirement cannot move it on, so the export
/// becomes exhausted: the epoch stays at `u64::MAX` (no wrap to 0, no overflow panic), the running review is cancelled
/// and its worker reaped, and the reply says `exhausted`. From then on every start is refused
/// `MBF-EXPORT-EPOCH-EXHAUSTED` whatever its ticket (the review and the save alike, no worker, no chooser), a command
/// that entered before the exhaustion is cancelled when it registers, and later retirements stay exhausted.
#[test]
fn an_exhausted_retirement_epoch_does_not_wrap_and_refuses_every_start() {
    use crate::mbf_export::EPOCH_EXHAUSTED;
    let gate = Gate::new("mbf-epoch-exhausted");
    let registry = registry_with(gate.launcher());
    let runs = MbfRuns::default();
    runs.seed_epoch_for_tests(u64::MAX - 1);
    let first = runs.retire(registry.pool());
    assert_eq!((first.epoch, first.exhausted), (u64::MAX, false), "the last ordinary retirement");
    assert_eq!(serde_json::to_value(first).unwrap(), json!({"cancelled": 0, "too_late": 0, "epoch": u64::MAX}), "no exhausted member while false");
    let entered = runs.enter(Some(json!(u64::MAX))).expect("u64::MAX is still a current ticket");
    let body = review_body(&canonical_model(), &settings());
    std::thread::scope(|scope| {
        let _guard = CancelOnUnwind(&runs, &registry);
        let reviewing = scope.spawn(|| review_via_command(&registry, &runs, &body));
        gate.wait_started(1);
        let pid = gate.pids()[0];
        let second = runs.retire(registry.pool());
        assert_eq!(
            second,
            MbfRetirement { receipt: STOPPED_ONE, epoch: u64::MAX, exhausted: true, retirement_id: None, duplicate: false },
            "no wrap; the running review is cancelled"
        );
        assert_eq!(serde_json::to_value(second).unwrap(), json!({"cancelled": 1, "too_late": 0, "epoch": u64::MAX, "exhausted": true}), "the reply");
        let (outcome, _) = join_within(&registry, &runs, reviewing, "the review");
        assert_eq!(review_codes(&outcome), vec![CANCELLED.to_string()]);
        assert!(!worker_alive(pid));
    });
    assert_eq!(runs.epoch(), u64::MAX, "the epoch did not wrap");
    // Every ticket is refused by name now, u64::MAX included.
    for ticket in [json!(u64::MAX), json!(0), json!(u64::MAX - 1)] {
        assert_eq!(runs.enter(Some(ticket.clone())).unwrap_err().code, EPOCH_EXHAUSTED, "{ticket}");
        let mut review = body.clone();
        review[EXPECTED_EPOCH] = ticket.clone();
        let (reviewed, _) = review_via_command(&registry, &runs, &review);
        assert_eq!(review_codes(&reviewed), vec![EPOCH_EXHAUSTED.to_string()], "{ticket}: {reviewed:?}");
        let mut save = save_body(&canonical_model(), recorded_default());
        save[EXPECTED_EPOCH] = ticket.clone();
        let opened = Opened::new(false);
        let saved = save_via_command(&registry, &runs, &save, Some(std::env::temp_dir().join("never-written.mbf")), &opened);
        assert_eq!(save_codes(&saved), vec![EPOCH_EXHAUSTED.to_string()], "{ticket}: {saved:?}");
        assert!(!opened.load(Ordering::SeqCst), "{ticket}: no chooser");
    }
    assert_eq!(gate.pids().len(), 1, "no worker started after the exhaustion");
    // A command that entered at u64::MAX before the exhaustion is cancelled when it registers.
    let late = runs.register(RunKind::Review, entered, registry.pool());
    assert!(late.control.cancelled());
    drop(late);
    // Terminal: a later retirement stays exhausted at u64::MAX and finds nothing tracked.
    assert_eq!(
        runs.retire(registry.pool()),
        MbfRetirement { receipt: MbfCancelReceipt::default(), epoch: u64::MAX, exhausted: true, retirement_id: None, duplicate: false }
    );
    // A keyed retirement once exhausted: terminal still, the epoch unmoved.
    let keyed = runs.retire_keyed(retirement(1, 1), registry.pool());
    assert_eq!((keyed.epoch, keyed.exhausted, keyed.duplicate), (u64::MAX, true, false));
    assert_eq!(runs.enter(Some(json!(u64::MAX))).unwrap_err().code, EPOCH_EXHAUSTED);
    assert_released(&registry);
}

/// Finding P3 at 6ccc68aff (3): a committed save stays tracked until its write completes. Paused just after its
/// commit (the `AFTER_COMMIT` hook), repeated cancels each report it too late; a newer save superseding it leaves
/// it tracked (the next cancel stops the newer one and reports the committed one too late); once the newer one
/// ends, the committed save is still reported too late. The write completes with the recorded digests, and after it
/// nothing is tracked.
#[test]
fn a_committed_save_stays_tracked_through_repeated_cancels_and_supersession() {
    let _lock = atomic_save_test_lock();
    let sandbox = Sandbox::new("committed-tracked");
    let registry = real_worker();
    let runs = MbfRuns::default();
    let receipts = Arc::new(Mutex::new(Vec::new()));
    let (hook_runs, hook_pool, hook_receipts) = (runs.clone(), Arc::clone(registry.pool()), Arc::clone(&receipts));
    AFTER_COMMIT.with(|hook| {
        *hook.borrow_mut() = Some(Box::new(move || {
            let mut receipts = hook_receipts.lock().unwrap();
            receipts.push(hook_runs.cancel_all(&hook_pool));
            receipts.push(hook_runs.cancel_all(&hook_pool));
            let newer = hook_runs.register(RunKind::Save, hook_runs.entry_now(), &hook_pool);
            receipts.push(hook_runs.cancel_all(&hook_pool));
            drop(newer);
            receipts.push(hook_runs.cancel_all(&hook_pool));
        }))
    });
    let opened = Opened::new(false);
    let chosen = sandbox.0.join("swbpipe-model.mbf");
    let outcome = save_via_command(&registry, &runs, &save_body(&canonical_model(), recorded_default()), Some(chosen.clone()), &opened);
    AFTER_COMMIT.with(|hook| hook.borrow_mut().take());
    let too_late = MbfCancelReceipt { cancelled: 0, too_late: 1 };
    assert_eq!(*receipts.lock().unwrap(), vec![too_late, too_late, MbfCancelReceipt { cancelled: 1, too_late: 1 }, too_late]);
    let MbfExportOutcome::Saved { mbf, identity_file, .. } = &outcome else { panic!("{outcome:?}") };
    assert_eq!((mbf.sha256.clone(), identity_file.sha256.clone()), recorded_default());
    assert_eq!(sha256_hex(&std::fs::read(&chosen).unwrap()), recorded_default().0);
    assert_eq!(runs.cancel_all(registry.pool()), MbfCancelReceipt::default(), "the completed save is no longer tracked");
    assert_released(&registry);
}

/// RT-U4's should-fix: the result lease is checked against [`RESULT_LEASE_BYTES`] in every build, which holds only if
/// `resident_envelope(length) <= RESULT_LEASE_BYTES` for every length the supervisor admits (at most the 64 MiB output
/// ceiling). `resident_envelope` is monotone: it is checked at every length up to the size-class limit and one page
/// beyond, and above that it is constant on each page `(kP, (k+1)P]` and steps up by one page, checked at both ends of
/// every page up to the ceiling. So the ceiling's envelope bounds every admissible length's.
#[test]
fn every_admissible_result_length_fits_the_result_lease() {
    use crate::worker_alloc::{LARGE_PAGE, SIZE_CLASS_LIMIT};
    let ceiling = mbf::MBF_EXPORT_OUTPUT_MAX_BYTES;
    assert_eq!(RESULT_LEASE_BYTES, ceiling, "the ceiling is page-aligned, so its envelope is itself");
    let mut previous = 0;
    for length in 0..=SIZE_CLASS_LIMIT + LARGE_PAGE {
        let envelope = resident_envelope(length);
        assert!(envelope >= previous && envelope >= length, "{length}");
        previous = envelope;
    }
    let mut page = SIZE_CLASS_LIMIT / LARGE_PAGE + 1;
    while page * LARGE_PAGE < ceiling {
        let (low, high) = (page * LARGE_PAGE + 1, (page + 1) * LARGE_PAGE);
        assert_eq!(resident_envelope(low), high, "{low}");
        assert_eq!(resident_envelope(high), high, "{high}");
        assert_eq!(resident_envelope(low - 1), high - LARGE_PAGE, "one page lower just below");
        page += 1;
    }
    for length in [0, 1, SIZE_CLASS_LIMIT, SIZE_CLASS_LIMIT + 1, ceiling - LARGE_PAGE, ceiling - 1, ceiling] {
        assert!(resident_envelope(length) <= RESULT_LEASE_BYTES, "{length}");
    }
    assert!(resident_envelope(ceiling + 1) > RESULT_LEASE_BYTES, "a longer result is refused at result_limit first");
}

// -- Memory, the output ceiling and the protocol ---------------------------------------------------------------------

/// A worker that needs more than its capped grant is refused by the pool's name, as the review's and the save's
/// memory refusal, with nothing left held.
#[test]
fn a_worker_above_its_grant_is_a_named_memory_refusal() {
    let registry = registry_with(TestLauncher::scripted(format!("alloc_steps:{}:{}", 16 * MIB, 2 * GIB)));
    registry.configure_pool(12 * GIB, GIB, false).unwrap();
    let runs = MbfRuns::default();
    let (outcome, _) = review_via_command(&registry, &runs, &review_body(&canonical_model(), &settings()));
    assert_eq!(review_codes(&outcome), vec![REVIEW_MEMORY_UNAVAILABLE.to_string()]);
    let MbfReviewOutcome::Refused { diagnostics } = &outcome else { unreachable!() };
    assert!(diagnostics[0].message.contains("The MBF worker's memory was refused"), "{}", diagnostics[0].message);
    assert_released(&registry);
    let opened = Opened::new(false);
    let outcome = save_via_command(&registry, &runs, &save_body(&canonical_model(), recorded_default()), None, &opened);
    assert_eq!(save_codes(&outcome), vec![MEMORY_UNAVAILABLE.to_string()]);
    assert!(!opened.load(Ordering::SeqCst));
    assert_released(&registry);
}

/// A result announced above the 64 MiB output ceiling ends at `result_limit` (the worker's grant is above the
/// ceiling, so the length passes the grant check first): `MBF-EXPORT-OUTPUT-TOO-LARGE` for a save, before any result
/// lease, chooser or write, and the review's memory refusal for a review.
#[test]
fn a_result_above_the_output_ceiling_is_refused_before_any_result_lease() {
    let over = mbf::MBF_EXPORT_OUTPUT_MAX_BYTES + 1;
    let registry = registry_with(TestLauncher::scripted(format!("oversized_header:{over}")));
    let runs = MbfRuns::default();
    let sandbox = Sandbox::new("ceiling");
    let opened = Opened::new(false);
    let body = save_body(&canonical_model(), recorded_default());
    let outcome = ends_by_itself(&registry, &runs, || save_via_command(&registry, &runs, &body, Some(sandbox.0.join("swbpipe-model.mbf")), &opened));
    assert_eq!(save_codes(&outcome), vec![OUTPUT_TOO_LARGE.to_string()]);
    assert!(!opened.load(Ordering::SeqCst) && sandbox.is_empty());
    assert_eq!(registry.pool().reserved(LeasePurpose::Result), 0);
    assert_released(&registry);
    let body = review_body(&canonical_model(), &settings());
    let (outcome, _) = ends_by_itself(&registry, &runs, || review_via_command(&registry, &runs, &body));
    assert_eq!(review_codes(&outcome), vec![REVIEW_MEMORY_UNAVAILABLE.to_string()]);
    assert_released(&registry);
}

/// Tampered or failed worker output is refused by name, before any chooser or write: a result that is not a task 3
/// envelope, the other mode's envelope, an attested result, the supervisor's protocol violations, an exit and a panic
/// are `MBF-EXPORT-WORKER-PROTOCOL`; task 3's own named failure keeps its code, another name does not.
#[test]
fn tampered_worker_output_is_refused_by_name_before_any_chooser() {
    let review_envelope = envelope(MbfMode::Review);
    let scripted: Vec<(String, Option<String>, &str)> = vec![
        ("echo".into(), None, WORKER_PROTOCOL),
        ("result_without_atst".into(), None, WORKER_PROTOCOL),
        ("atst_twice".into(), None, WORKER_PROTOCOL),
        ("attested:{}".into(), None, WORKER_PROTOCOL),
        ("exit:3".into(), None, WORKER_PROTOCOL),
        ("panic".into(), None, WORKER_PROTOCOL),
        ("gate".into(), Some(review_envelope), WORKER_PROTOCOL),
        ("gate".into(), Some(format!("err:{ENGINE_FAILED}: The MBF engine returned no result (KeyError).")), ENGINE_FAILED),
        ("gate".into(), Some("err:SOME-OTHER-CODE: not task 3's".into()), WORKER_PROTOCOL),
    ];
    for (workload, release, code) in scripted {
        let gate = Gate::new("mbf-tamper");
        let launcher = if workload == "gate" { gate.launcher() } else { TestLauncher::scripted(workload.clone()) };
        let registry = registry_with(launcher);
        let runs = MbfRuns::default();
        let sandbox = Sandbox::new("tamper");
        let opened = Opened::new(false);
        let body = save_body(&canonical_model(), recorded_default());
        let outcome = std::thread::scope(|scope| {
            let _guard = CancelOnUnwind(&runs, &registry);
            let saving = scope.spawn(|| save_via_command(&registry, &runs, &body, Some(sandbox.0.join("swbpipe-model.mbf")), &opened));
            if let Some(release) = &release {
                gate.wait_started(1);
                gate.release(release);
            }
            saving.join().unwrap()
        });
        assert_eq!(save_codes(&outcome), vec![code.to_string()], "{workload} {release:?}");
        assert!(!opened.load(Ordering::SeqCst) && sandbox.is_empty(), "{workload}");
        assert_released(&registry);
    }
}

// -- Save refusals ----------------------------------------------------------------------------------------------------

/// A review is no authorization: the save's own worker run decides. Its truncated review refuses
/// `MBF-REVIEW-TRUNCATED` whatever the request's reviewed digests; its writer refusal returns the review; a stale
/// review, and route evidence the save refuses before any worker starts, are refused; none opens a chooser or writes.
#[test]
fn the_saves_own_worker_run_refuses_before_any_chooser() {
    let text = canonical_model();
    let truncated = |limits: crate::mbf_export::RenderLimits| {
        crate::mbf_export::worker_render_limited(&WorkerRequest { mode: MbfMode::Save, settings_json: &settings(), model_text: &text }.encode(), limits).unwrap()
    };
    let full_review = decode_worker_result(MbfMode::Review, &envelope(MbfMode::Review)).unwrap().review.len();
    let save_len = envelope(MbfMode::Save).len() as u64;
    let gated: Vec<(String, &str)> = vec![
        (truncated(crate::mbf_export::RenderLimits::for_tests(full_review - 1, mbf::MBF_EXPORT_OUTPUT_MAX_BYTES)), REVIEW_TRUNCATED),
        (truncated(crate::mbf_export::RenderLimits::for_tests(mbf::MBF_REVIEW_MAX_BYTES, save_len - 1)), OUTPUT_TOO_LARGE),
    ];
    for (release, code) in gated {
        let gate = Gate::new("mbf-save-refusal");
        let registry = registry_with(gate.launcher());
        let runs = MbfRuns::default();
        let opened = Opened::new(false);
        let outcome = std::thread::scope(|scope| {
            let _guard = CancelOnUnwind(&runs, &registry);
            let saving = scope.spawn(|| save_via_command(&registry, &runs, &save_body(&text, recorded_default()), Some(std::env::temp_dir().join("never-written.mbf")), &opened));
            gate.wait_started(1);
            gate.release(&release);
            saving.join().unwrap()
        });
        assert_eq!(save_codes(&outcome), vec![code.to_string()]);
        assert!(matches!(&outcome, MbfExportOutcome::Refused { review: Some(_), .. }), "the refusal carries the save run's review");
        assert!(!opened.load(Ordering::SeqCst));
        assert_released(&registry);
    }
    // The real worker: a writer refusal, and a stale review.
    let registry = real_worker();
    let runs = MbfRuns::default();
    let tee = model_for("components[0].kind = tee");
    let opened = Opened::new(false);
    let outcome = save_via_command(&registry, &runs, &save_body(&tee, recorded_default()), None, &opened);
    assert_eq!(save_codes(&outcome), vec![WRITER_REFUSED.to_string()]);
    let MbfExportOutcome::Refused { review: Some(review), .. } = &outcome else { panic!("{outcome:?}") };
    assert_eq!(serde_json::from_str::<Value>(review.get()).unwrap()["blocking"], true);
    // The ticket binds a request to a retirement and authorizes nothing: a stale review and wrong route evidence
    // carrying the current epoch are still refused.
    assert_eq!(ticketed(&runs, &save_body(&text, recorded_default()))[EXPECTED_EPOCH], json!(runs.epoch()), "the requests carry a current ticket");
    let stale = save_via_command(&registry, &runs, &save_body(&text, ("0".repeat(64), "0".repeat(64))), None, &opened);
    assert_eq!(save_codes(&stale), vec![REVIEW_STALE.to_string()]);
    assert!(!opened.load(Ordering::SeqCst));
    assert_released(&registry);
    // Route evidence for another context: refused before the admission, no worker started.
    let gate = Gate::new("mbf-route");
    let registry = registry_with(gate.launcher());
    let mut body = save_body(&text, recorded_default());
    body["screening"]["export_context"] = json!("public");
    let outcome = save_via_command(&registry, &runs, &body, None, &opened);
    assert_eq!(save_codes(&outcome).len(), 1, "{outcome:?}");
    assert!(!matches!(outcome, MbfExportOutcome::Saved { .. } | MbfExportOutcome::Cancelled));
    assert_eq!(gate.started_count(), 0);
    assert!(!opened.load(Ordering::SeqCst));
    assert_released(&registry);
}

// -- The only production route to the writer (P3-1) -------------------------------------------------------------------

/// The names the scan follows: the writer's entry points, task 3 and the run seam.
const FOLLOWED: [&str; 4] = ["worker_render", "worker_render_limited", "MbfRender", "MbfWriterRun"];

/// Every non-test reference to a followed name, counted by `file: item: name` (item is the enclosing function, or the
/// implementing type for an `impl MbfWriterRun for` item; `-` outside both). An attribute on a statement is not
/// read: `solve_worker.rs`'s test-only scripted-mode match in `workload` counts as one of its references.
#[derive(Default)]
struct References {
    file: String,
    items: Vec<String>,
    test_depth: usize,
    found: std::collections::BTreeMap<String, usize>,
}

fn test_only(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path().is_ident("test")
            || (attr.path().is_ident("cfg") && attr.parse_args::<syn::Ident>().is_ok_and(|ident| ident == "test"))
    })
}

impl References {
    fn record(&mut self, name: &str) {
        if self.test_depth == 0 && FOLLOWED.contains(&name) {
            let item = self.items.last().map_or("-", String::as_str);
            *self.found.entry(format!("{}: {item}: {name}", self.file)).or_default() += 1;
        }
    }

    fn within(&mut self, attrs: &[syn::Attribute], item: Option<String>, visit: impl FnOnce(&mut Self)) {
        let test = test_only(attrs);
        self.test_depth += usize::from(test);
        let named = item.is_some();
        self.items.extend(item);
        visit(self);
        if named {
            self.items.pop();
        }
        self.test_depth -= usize::from(test);
    }

    fn tokens(&mut self, tokens: proc_macro2::TokenStream) {
        for tree in tokens {
            match tree {
                proc_macro2::TokenTree::Ident(ident) => self.record(&ident.to_string()),
                proc_macro2::TokenTree::Group(group) => self.tokens(group.stream()),
                _ => {}
            }
        }
    }
}

impl<'ast> syn::visit::Visit<'ast> for References {
    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
        self.within(&item.attrs, Some(item.sig.ident.to_string()), |this| syn::visit::visit_item_fn(this, item));
    }

    fn visit_impl_item_fn(&mut self, item: &'ast syn::ImplItemFn) {
        self.within(&item.attrs, Some(item.sig.ident.to_string()), |this| syn::visit::visit_impl_item_fn(this, item));
    }

    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        self.within(&item.attrs, None, |this| syn::visit::visit_item_mod(this, item));
    }

    fn visit_item_impl(&mut self, item: &'ast syn::ItemImpl) {
        let implementing = item.trait_.as_ref().filter(|(_, path, _)| path.segments.last().is_some_and(|last| last.ident == "MbfWriterRun"));
        let named = implementing.map(|_| match &*item.self_ty {
            syn::Type::Path(path) => path.path.segments.last().map_or_else(String::new, |last| last.ident.to_string()),
            _ => "?".to_string(),
        });
        self.within(&item.attrs, named, |this| syn::visit::visit_item_impl(this, item));
    }

    fn visit_path(&mut self, path: &'ast syn::Path) {
        if let Some(last) = path.segments.last() {
            self.record(&last.ident.to_string());
        }
        syn::visit::visit_path(self, path);
    }

    fn visit_use_name(&mut self, name: &'ast syn::UseName) {
        self.record(&name.ident.to_string());
    }

    fn visit_use_rename(&mut self, rename: &'ast syn::UseRename) {
        self.record(&rename.ident.to_string());
    }

    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        self.tokens(mac.tokens.clone());
        syn::visit::visit_macro(self, mac);
    }
}

/// The files `file` declares as modules (`mod name;`, with or without `#[path]`), by their paths under `src/`, each
/// with whether that declaration is test-only (`#[cfg(test)]` or `#[test]`).
fn module_files(file: &str, parsed: &syn::File) -> Vec<(String, bool)> {
    let dir = Path::new(file).parent().unwrap_or(Path::new(""));
    let owns_dir = matches!(Path::new(file).file_name().and_then(|name| name.to_str()), Some("lib.rs" | "main.rs" | "mod.rs"));
    let mut files = Vec::new();
    for item in &parsed.items {
        let syn::Item::Mod(module) = item else { continue };
        if module.content.is_some() {
            continue;
        }
        let path = module.attrs.iter().find(|attr| attr.path().is_ident("path")).and_then(|attr| match &attr.meta {
            syn::Meta::NameValue(syn::MetaNameValue { value: syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(path), .. }), .. }) => Some(dir.join(path.value())),
            _ => None,
        });
        let path = path.unwrap_or_else(|| {
            let name = format!("{}.rs", module.ident);
            if owns_dir {
                dir.join(name)
            } else {
                dir.join(Path::new(file).file_stem().unwrap()).join(name)
            }
        });
        files.push((normalized(&path), test_only(&module.attrs)));
    }
    files
}

/// A resolved module path, lexically normalized (`.` dropped, `..` taken back), so two spellings of one file are one
/// key: `#[path = "./helper.rs"]` and `#[path = "helper.rs"]` in the same file name the same module file.
fn normalized(path: &Path) -> String {
    let mut parts: Vec<String> = Vec::new();
    for component in path.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir if parts.last().is_some_and(|last| last != "..") => {
                parts.pop();
            }
            other => parts.push(other.as_os_str().to_string_lossy().into_owned()),
        }
    }
    parts.join("/")
}

/// The non-test references in `files` ((name, text) pairs). A file is skipped only when every declaration of it is a
/// test-only module: production reachability is resolved over all declarations first, so a file also declared by a
/// production `mod` (with or without `#[path]`) is scanned.
fn references(files: &[(String, String)]) -> std::collections::BTreeMap<String, usize> {
    let parsed: Vec<(&String, syn::File)> =
        files.iter().map(|(file, text)| (file, syn::parse_file(text).unwrap_or_else(|error| panic!("{file} does not parse: {error}")))).collect();
    let declared: Vec<(String, bool)> = parsed.iter().flat_map(|(file, parsed)| module_files(file, parsed)).collect();
    let production: std::collections::BTreeSet<&str> = declared.iter().filter(|(_, test)| !test).map(|(path, _)| path.as_str()).collect();
    let test_files: std::collections::BTreeSet<&str> =
        declared.iter().filter(|(path, test)| *test && !production.contains(path.as_str())).map(|(path, _)| path.as_str()).collect();
    let mut found = std::collections::BTreeMap::new();
    for (file, parsed) in &parsed {
        if test_files.contains(file.as_str()) {
            continue;
        }
        let mut scan = References { file: (*file).clone(), ..References::default() };
        syn::visit::Visit::visit_file(&mut scan, parsed);
        for (reference, count) in scan.found {
            *found.entry(reference).or_default() += count;
        }
    }
    found
}

/// The production references the design admits: task 3's workload arm is the only call of `worker_render` (whose
/// only call of the limited form is its own), task 3 is named by its codes, the workload and [`CappedWorkerRun`]'s
/// run alone, the run seam is implemented only by [`CappedWorkerRun`], and named only by `mbf_export`'s pipelines and
/// this module's import.
const ADMITTED: [(&str, usize); 12] = [
    ("mbf_export.rs: review_mbf_export_outcome: MbfWriterRun", 1),
    ("mbf_export.rs: review_mbf_export_with: MbfWriterRun", 1),
    ("mbf_export.rs: save_mbf_export_outcome: MbfWriterRun", 1),
    ("mbf_export.rs: save_mbf_export_with: MbfWriterRun", 1),
    ("mbf_export.rs: worker_render: worker_render_limited", 1),
    ("mbf_worker.rs: -: MbfWriterRun", 1),
    ("mbf_worker.rs: CappedWorkerRun: MbfWriterRun", 1),
    ("mbf_worker.rs: run: MbfRender", 1),
    ("solve_worker.rs: decode_request: MbfRender", 1),
    ("solve_worker.rs: task_code: MbfRender", 1),
    // The workload arm, and the test-only scripted mode's match in the same function.
    ("solve_worker.rs: workload: MbfRender", 2),
    ("solve_worker.rs: workload: worker_render", 1),
];

/// The crate's sources, by file name under `src/`.
fn crate_sources() -> Vec<(String, String)> {
    fn walk(dir: &Path, root: &Path, out: &mut Vec<(String, String)>) {
        let mut entries: Vec<_> = std::fs::read_dir(dir).unwrap().map(|entry| entry.unwrap().path()).collect();
        entries.sort();
        for path in entries {
            if path.is_dir() {
                walk(&path, root, out);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                let name = path.strip_prefix(root).unwrap().to_string_lossy().replace('\\', "/");
                out.push((name, std::fs::read_to_string(&path).unwrap()));
            }
        }
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    walk(&root, &root, &mut files);
    files
}

/// P3-1: in production code, task 3's workload arm is the only caller of `worker_render`, the capped-worker run the
/// only one naming task 3 outside the worker's own codes, and the only implementation of the run seam.
#[test]
fn the_capped_worker_run_is_the_only_production_route_to_the_writer() {
    let files = crate_sources();
    assert!(files.len() > 20 && files.iter().any(|(name, _)| name == "mbf_worker.rs"), "the scan reads the crate's sources");
    assert!(files.iter().any(|(name, _)| name == "mbf_worker_tests.rs"));
    let found = references(&files);
    let admitted: std::collections::BTreeMap<String, usize> =
        ADMITTED.iter().map(|(entry, count)| (entry.to_string(), *count)).collect();
    assert_eq!(found, admitted, "the production references to the MBF writer changed");
}

/// The scan's negative controls: each of these production references is found; the same in test-only code is not.
#[test]
fn the_writer_route_scan_finds_each_other_route() {
    let found = |text: &str| references(&[("invented.rs".to_string(), text.to_string())]);
    let refused = [
        ("fn other(raw: &[u8]) { crate::mbf_export::worker_render(raw); }", "invented.rs: other: worker_render"),
        ("fn other(raw: &[u8]) { worker_render_limited(raw, LIMITS); }", "invented.rs: other: worker_render_limited"),
        ("#[cfg(not(test))] fn other(raw: &[u8]) { worker_render(raw); }", "invented.rs: other: worker_render"),
        ("use crate::mbf_export::worker_render as render;", "invented.rs: -: worker_render"),
        ("fn other() { run!(worker_render); }", "invented.rs: other: worker_render"),
        ("fn other() -> WorkerTask { WorkerTask::MbfRender }", "invented.rs: other: MbfRender"),
        ("struct InProcess; impl MbfWriterRun for InProcess {}", "invented.rs: InProcess: MbfWriterRun"),
        ("mod inner { fn other(raw: &[u8]) { super::worker_render(raw); } }", "invented.rs: other: worker_render"),
    ];
    for (text, expected) in refused {
        assert!(found(text).contains_key(expected), "{text}: {:?}", found(text));
    }
    let admitted = [
        "#[cfg(test)] mod tests { fn t(raw: &[u8]) { worker_render(raw); } }",
        "#[test] fn t() { let task = WorkerTask::MbfRender; }",
        "#[cfg(test)] impl MbfWriterRun for InProcess {}",
        "/// [`worker_render`] in a doc comment\nfn other() {}",
    ];
    for text in admitted {
        assert!(found(text).is_empty(), "{text}: {:?}", found(text));
    }
    // A second call inside an admitted function is counted.
    let twice = found("fn workload(raw: Vec<u8>) { worker_render(&raw); worker_render(&raw); }");
    assert_eq!(twice.get("invented.rs: workload: worker_render"), Some(&2));
    // A file declared as a test-only module is skipped; the same file declared for production is not.
    let call = "fn other(raw: &[u8]) { worker_render(raw); }".to_string();
    for (declaration, skipped) in [
        ("#[cfg(test)]\n#[path = \"helper.rs\"]\nmod helper;", true),
        ("#[path = \"helper.rs\"]\nmod helper;", false),
        ("#[cfg(test)]\nmod helper;", true),
        // Finding P3 at 9c5785a6d: declared both test-only and for production, in either order, the file is reachable
        // from production and is scanned.
        ("#[cfg(test)]\n#[path = \"helper.rs\"]\nmod for_tests;\n#[path = \"helper.rs\"]\nmod shipped;", false),
        ("#[path = \"helper.rs\"]\nmod shipped;\n#[cfg(test)]\nmod helper;", false),
        // Finding P3 at 6ccc68aff: two spellings of one file are one module file, so a production declaration by
        // another spelling still makes it reachable and scanned.
        ("#[cfg(test)]\n#[path = \"helper.rs\"]\nmod for_tests;\n#[path = \"./helper.rs\"]\nmod shipped;", false),
        ("#[cfg(test)]\n#[path = \"./helper.rs\"]\nmod for_tests;\n#[path = \"sub/../helper.rs\"]\nmod shipped;", false),
        ("#[cfg(test)]\n#[path = \"./sub/../helper.rs\"]\nmod helper;", true),
    ] {
        let files = [("lib.rs".to_string(), declaration.to_string()), ("helper.rs".to_string(), call.clone())];
        assert_eq!(references(&files).is_empty(), skipped, "{declaration}");
    }
}

// -- Repair 5: idempotent keyed retirement ------------------------------------------------------------------------

/// Starts a review on another thread in `scope`, waits for its worker, and returns the worker's pid.
fn running_review<'s, 'e: 's>(
    scope: &'s std::thread::Scope<'s, 'e>,
    gate: &Gate,
    started: usize,
    registry: &'e SolveJobRegistry,
    runs: &'e MbfRuns,
    body: &'e Value,
) -> (std::thread::ScopedJoinHandle<'s, (MbfReviewOutcome, String)>, i32) {
    let reviewing = scope.spawn(move || review_via_command(registry, runs, body));
    gate.wait_started(started);
    (reviewing, gate.pids()[started - 1])
}

/// An injected-fault test of the native rule (the reviewer's schedule at 8cecfc1ce): the test hands the host one id
/// twice, as a duplicated delivery would (the JSON-form cancel survives Tauri 2.11.1's `postMessage` fallback, so a
/// cancel's resend is admitted; a raw-body start is not, so a fresh start after the fallback is refused
/// `INGRESS-RAW-BODY-REQUIRED`). `r1` is applied (epoch 2) and acknowledged; a
/// fresh review enters at epoch 2 and runs; then `r1` arrives again, and so does an older id of the same page: each
/// is a no-op whose reply says `duplicate` with the current epoch, cancels nothing and moves nothing on, and the fresh
/// review's worker keeps running. A different page's token advances the epoch and stops the review. A host that
/// ignored the sequence would cancel the fresh review at the second delivery.
#[test]
fn a_duplicate_or_older_retirement_id_is_a_truthful_no_op_and_a_new_page_token_advances() {
    let gate = Gate::new("mbf-keyed-duplicate");
    let registry = registry_with(gate.launcher());
    let runs = MbfRuns::default();
    let (r0, r1) = (retirement(1, 1), retirement(1, 2));
    let first = cancel_via_command(&registry, &runs, &cancel_body(&r0)).unwrap();
    assert_eq!((first.epoch, first.duplicate), (1, false));
    let applied = cancel_via_command(&registry, &runs, &cancel_body(&r1)).unwrap();
    assert_eq!(
        serde_json::to_value(&applied).unwrap(),
        json!({"cancelled": 0, "too_late": 0, "epoch": 2, "retirement_id": {"page": page(1), "sequence": 2}}),
        "the first delivery's reply: applied, the id echoed, no duplicate member"
    );
    let body = review_body(&canonical_model(), &settings());
    std::thread::scope(|scope| {
        let _guard = CancelOnUnwind(&runs, &registry);
        let (reviewing, pid) = running_review(scope, &gate, 1, &registry, &runs, &body);
        for (delayed, what) in [(&r1, "a second delivery of r1"), (&r0, "an older id of the same page")] {
            let reply = cancel_via_command(&registry, &runs, &cancel_body(delayed)).unwrap();
            assert_eq!(
                serde_json::to_value(&reply).unwrap(),
                json!({"cancelled": 0, "too_late": 0, "epoch": 2, "retirement_id": {"page": page(1), "sequence": delayed.sequence}, "duplicate": true}),
                "{what}: a truthful no-op"
            );
            assert_eq!(runs.epoch(), 2, "{what}: the epoch did not move");
            assert!(!any_tracked_cancelled(&runs) && worker_alive(pid), "{what}: the fresh review was not cancelled");
        }
        let other = cancel_via_command(&registry, &runs, &cancel_body(&retirement(2, 1))).unwrap();
        assert_eq!((other.receipt, other.epoch, other.duplicate), (STOPPED_ONE, 3, false), "a different page token advances");
        let (reviewed, _) = join_within(&registry, &runs, reviewing, "the review");
        assert_eq!(review_codes(&reviewed), vec![CANCELLED.to_string()]);
        assert!(!worker_alive(pid));
    });
    // The next sequence of page 1 is applied as usual.
    assert_eq!(runs.retire_keyed(retirement(1, 3), registry.pool()).epoch, 4);
    assert_released(&registry);
}

/// No applied token is forgotten. With [`RETIREMENT_PAGES_MAX`] page tokens applied, a fresh review is acknowledged
/// and runs at the current epoch; a delayed duplicate of the oldest token's id is still a no-op (no advance, no cancel,
/// `duplicate`). A retirement from one more, new token is not recorded and makes the export exhausted (fail closed): it
/// stops the review, the epoch does not move, and every start is refused `MBF-EXPORT-EPOCH-EXHAUSTED`. The delayed
/// duplicate of the oldest token stays a truthful no-op after it.
#[test]
fn the_page_token_map_never_evicts_and_a_new_token_beyond_it_exhausts_the_export() {
    use crate::mbf_export::EPOCH_EXHAUSTED;
    let gate = Gate::new("mbf-keyed-capacity");
    let registry = registry_with(gate.launcher());
    let runs = MbfRuns::default();
    for n in 0..RETIREMENT_PAGES_MAX {
        let reply = runs.retire_keyed(retirement(n, 1), registry.pool());
        assert_eq!((reply.duplicate, reply.exhausted), (false, false), "page {n}");
    }
    let full = RETIREMENT_PAGES_MAX as u64;
    assert_eq!((runs.epoch(), runs.lock().applied.len()), (full, RETIREMENT_PAGES_MAX));
    let body = review_body(&canonical_model(), &settings());
    std::thread::scope(|scope| {
        let _guard = CancelOnUnwind(&runs, &registry);
        let (reviewing, pid) = running_review(scope, &gate, 1, &registry, &runs, &body);
        let delayed = cancel_via_command(&registry, &runs, &cancel_body(&retirement(0, 1))).unwrap();
        assert_eq!((delayed.receipt, delayed.epoch, delayed.duplicate, delayed.exhausted), (MbfCancelReceipt::default(), full, true, false));
        assert!(!any_tracked_cancelled(&runs) && worker_alive(pid), "the delayed duplicate of the oldest token cancelled nothing");
        let beyond = cancel_via_command(&registry, &runs, &cancel_body(&retirement(RETIREMENT_PAGES_MAX, 1))).unwrap();
        assert_eq!((beyond.receipt, beyond.epoch, beyond.duplicate, beyond.exhausted), (STOPPED_ONE, full, false, true), "fail closed");
        let (reviewed, _) = join_within(&registry, &runs, reviewing, "the review");
        assert_eq!(review_codes(&reviewed), vec![CANCELLED.to_string()]);
    });
    assert_eq!(runs.lock().applied.len(), RETIREMENT_PAGES_MAX, "the new token was not recorded, and none was evicted");
    let after = cancel_via_command(&registry, &runs, &cancel_body(&retirement(0, 1))).unwrap();
    assert_eq!((after.receipt, after.epoch, after.duplicate, after.exhausted), (MbfCancelReceipt::default(), full, true, true), "still a no-op");
    assert_eq!(runs.enter(Some(json!(full))).unwrap_err().code, EPOCH_EXHAUSTED, "terminal");
    let (reviewed, _) = review_via_command(&registry, &runs, &body);
    assert_eq!(review_codes(&reviewed), vec![EPOCH_EXHAUSTED.to_string()]);
    assert_eq!(gate.pids().len(), 1, "no worker started after the exhaustion");
    assert_released(&registry);
}

/// `cancel_mbf_export`'s protocol: a missing, `null` or malformed `retirementId` is refused
/// `MBF-EXPORT-RETIREMENT-ID-INVALID` (the command's error) and retires nothing: no run is cancelled, the epoch does
/// not move and no token is recorded. The request goes through the normal raw-body ingress: one without the ingress
/// header, or above the command's argument bound, is an ingress refusal.
#[test]
fn a_missing_or_malformed_retirement_id_is_refused_by_name_and_retires_nothing() {
    use crate::mbf_export::RETIREMENT_ID_INVALID;
    let gate = Gate::new("mbf-keyed-malformed");
    let registry = registry_with(gate.launcher());
    let runs = MbfRuns::default();
    runs.retire_keyed(retirement(0, 1), registry.pool());
    let good = page(1);
    let refused = [
        json!({}),
        json!({ RETIREMENT_ID: null }),
        json!({ RETIREMENT_ID: "abc" }),
        json!({ RETIREMENT_ID: [good, 1] }),
        json!({ RETIREMENT_ID: { "sequence": 1 } }),
        json!({ RETIREMENT_ID: { "page": good } }),
        json!({ RETIREMENT_ID: { "page": good.to_uppercase(), "sequence": 1 } }),
        json!({ RETIREMENT_ID: { "page": &good[1..], "sequence": 1 } }),
        json!({ RETIREMENT_ID: { "page": format!("{good}0"), "sequence": 1 } }),
        json!({ RETIREMENT_ID: { "page": format!("g{}", &good[1..]), "sequence": 1 } }),
        json!({ RETIREMENT_ID: { "page": 17, "sequence": 1 } }),
        json!({ RETIREMENT_ID: { "page": good, "sequence": 0 } }),
        json!({ RETIREMENT_ID: { "page": good, "sequence": -1 } }),
        json!({ RETIREMENT_ID: { "page": good, "sequence": 1.5 } }),
        json!({ RETIREMENT_ID: { "page": good, "sequence": 1.0 } }),
        json!({ RETIREMENT_ID: { "page": good, "sequence": "1" } }),
        json!({ RETIREMENT_ID: { "page": good, "sequence": 1u64 << 53 } }),
        json!({ RETIREMENT_ID: { "page": good, "sequence": 1, "epoch": 3 } }),
    ];
    let body = review_body(&canonical_model(), &settings());
    std::thread::scope(|scope| {
        let _guard = CancelOnUnwind(&runs, &registry);
        let (reviewing, pid) = running_review(scope, &gate, 1, &registry, &runs, &body);
        for request in &refused {
            match cancel_via_command(&registry, &runs, request) {
                Err(CommandError::Command(message)) => assert!(message.starts_with(&format!("{RETIREMENT_ID_INVALID}: ")), "{request}: {message}"),
                other => panic!("{request}: {other:?}"),
            }
            // The JSON form's id is read by the same rule.
            for wire in [Wire::JsonCustomProtocol, Wire::JsonFallback] {
                match cancel_over(wire, &registry, &runs, request) {
                    Err(CommandError::Command(message)) => assert!(message.starts_with(&format!("{RETIREMENT_ID_INVALID}: ")), "{wire:?} {request}: {message}"),
                    other => panic!("{wire:?} {request}: {other:?}"),
                }
            }
        }
        // The JSON form's arguments are one object of at most 4 KiB, as the raw form's are.
        for args in [json!([good, 1]), json!("retirementId"), json!(null), json!(serde_json::to_vec(&cancel_body(&retirement(1, 1))).unwrap())] {
            let message = ingress_message(cancel_over(Wire::JsonFallback, &registry, &runs, &args));
            assert!(message.starts_with("INGRESS-ARGS-NOT-OBJECT: "), "{args}: {message}");
        }
        let oversized_json = json!({ RETIREMENT_ID: { "page": good, "sequence": 1 }, "pad": "x".repeat(4 << 10) });
        for wire in [Wire::JsonCustomProtocol, Wire::JsonFallback] {
            let message = ingress_message(cancel_over(wire, &registry, &runs, &oversized_json));
            assert!(message.starts_with("INGRESS-ARGS-TOO-LARGE: "), "{wire:?}: {message}");
        }
        // The raw form after the fallback (its bytes as a JSON array of numbers, under the ingress header) is a JSON
        // body under the raw form's header: still refused, as before.
        let message = ingress_message(cancel_over(Wire::RawFallback, &registry, &runs, &cancel_body(&retirement(1, 1))));
        assert!(message.starts_with("INGRESS-RAW-BODY-REQUIRED: "), "{message}");
        let unheaded = cancel_command(&registry, &runs, &raw_body(&cancel_body(&retirement(1, 1))), &headers(None));
        assert!(matches!(unheaded, Err(CommandError::Ingress(_))), "{unheaded:?}");
        let oversized = json!({ RETIREMENT_ID: { "page": good, "sequence": 1 }, "pad": "x".repeat(8 << 10) });
        assert!(matches!(cancel_via_command(&registry, &runs, &oversized), Err(CommandError::Ingress(_))));
        assert_eq!(runs.epoch(), 1, "nothing moved on");
        assert_eq!(runs.lock().applied.len(), 1, "no token was recorded");
        assert!(!any_tracked_cancelled(&runs) && worker_alive(pid), "nothing was cancelled");
        assert_eq!(runs.cancel_all(registry.pool()), STOPPED_ONE);
        let _ = join_within(&registry, &runs, reviewing, "the review");
    });
    // The largest sequence is accepted.
    let largest = cancel_via_command(&registry, &runs, &json!({ RETIREMENT_ID: { "page": good, "sequence": (1u64 << 53) - 1 } })).unwrap();
    assert!(!largest.duplicate);
    assert_released(&registry);
}

/// Finding P2 (re-check at d4517a4e5): once Tauri 2.11.1's transport has fallen back to `postMessage` for the page,
/// a retirement must still stop a save that was already admitted. Save A has entered, its worker has ended and its
/// chooser is held open (before its commit) when the transport falls back. The raw form's cancel then arrives as an
/// array of numbers and is refused `INGRESS-RAW-BODY-REQUIRED`, stopping nothing; the JSON form, with the bytes
/// Tauri's fallback actually delivers, is applied: it stops the save, which replies `MBF-EXPORT-CANCELLED` when its
/// chooser returns, writes nothing, and releases its ledger and grant. A host that refused the JSON form would let the
/// save write.
#[test]
fn after_the_transport_falls_back_a_json_retirement_stops_an_admitted_save_before_its_commit() {
    let _lock = atomic_save_test_lock();
    let gate = Gate::new("mbf-fallback");
    let registry = registry_with(gate.launcher());
    let runs = MbfRuns::default();
    let sandbox = Sandbox::new("fallback");
    let body = save_body(&canonical_model(), recorded_default());
    let reply = envelope(MbfMode::Save);
    let (outcome, receipt) = std::thread::scope(|scope| {
        let _guard = CancelOnUnwind(&runs, &registry);
        let (opened_tx, opened_rx) = std::sync::mpsc::channel::<()>();
        let (accept_tx, accept_rx) = std::sync::mpsc::channel::<()>();
        let chosen = sandbox.0.join("swbpipe-model.mbf");
        let saving = scope.spawn(|| {
            save_choosing(&registry, &runs, &body, move |_| {
                opened_tx.send(()).unwrap();
                let _ = accept_rx.recv_timeout(Duration::from_secs(60));
                Ok(Some(chosen))
            })
        });
        gate.wait_started(1);
        gate.release(&reply);
        opened_rx.recv_timeout(Duration::from_secs(60)).expect("the save's chooser opened");
        let epoch = runs.epoch();
        let request = cancel_body(&retirement(1, 1));
        let raw = ingress_message(cancel_over(Wire::RawFallback, &registry, &runs, &request));
        assert!(raw.starts_with("INGRESS-RAW-BODY-REQUIRED: "), "{raw}");
        assert!(!any_tracked_cancelled(&runs) && runs.epoch() == epoch, "the raw form's fallback stopped nothing");
        let receipt = cancel_over(Wire::JsonFallback, &registry, &runs, &request).expect("the JSON form is admitted after the fallback");
        accept_tx.send(()).unwrap();
        (join_within(&registry, &runs, saving, "the save"), receipt)
    });
    assert_eq!((receipt.receipt, receipt.duplicate, receipt.retirement_id), (STOPPED_ONE, false, Some(retirement(1, 1))));
    assert_eq!(save_codes(&outcome), vec![CANCELLED.to_string()], "{outcome:?}");
    assert!(sandbox.is_empty(), "the retired save wrote nothing");
    assert_eq!(runs.cancel_all(registry.pool()), MbfCancelReceipt::default(), "nothing is tracked");
    assert_released(&registry);
}

/// One logical retirement delivered in both forms (the raw form, then the JSON form after the fallback, or the other
/// way round) is applied once: the second delivery's reply says `duplicate` with the current epoch, cancels nothing and
/// moves nothing on, and a fresh review that entered after the first delivery keeps running. The page's next id, in
/// either form, retires it.
#[test]
fn a_retirement_delivered_raw_and_as_json_is_applied_once() {
    let gate = Gate::new("mbf-keyed-forms");
    let registry = registry_with(gate.launcher());
    let runs = MbfRuns::default();
    let body = review_body(&canonical_model(), &settings());
    let first = cancel_via_command(&registry, &runs, &cancel_body(&retirement(1, 1))).unwrap();
    assert_eq!((first.epoch, first.duplicate), (1, false));
    std::thread::scope(|scope| {
        let _guard = CancelOnUnwind(&runs, &registry);
        let (reviewing, pid) = running_review(scope, &gate, 1, &registry, &runs, &body);
        for wire in [Wire::JsonCustomProtocol, Wire::JsonFallback] {
            let again = cancel_over(wire, &registry, &runs, &cancel_body(&retirement(1, 1))).unwrap();
            assert_eq!((again.receipt, again.epoch, again.duplicate), (MbfCancelReceipt::default(), 1, true), "{wire:?}");
        }
        assert!(!any_tracked_cancelled(&runs) && worker_alive(pid), "the duplicate cancelled nothing");
        let next = cancel_over(Wire::JsonFallback, &registry, &runs, &cancel_body(&retirement(1, 2))).unwrap();
        assert_eq!((next.receipt, next.epoch, next.duplicate), (STOPPED_ONE, 2, false));
        let raw_again = cancel_via_command(&registry, &runs, &cancel_body(&retirement(1, 2))).unwrap();
        assert_eq!((raw_again.receipt, raw_again.epoch, raw_again.duplicate), (MbfCancelReceipt::default(), 2, true), "raw after JSON");
        let (reviewed, _) = join_within(&registry, &runs, reviewing, "the review");
        assert_eq!(review_codes(&reviewed), vec![CANCELLED.to_string()]);
    });
    assert_eq!(runs.lock().applied, vec![(page(1), 2)], "one token, its last sequence");
    assert_released(&registry);
}
