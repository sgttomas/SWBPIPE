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

/// `review_mbf_export` as `lib.rs` runs it: the copy's reservation, then the blocking part.
fn review_via_command(registry: &SolveJobRegistry, runs: &MbfRuns, body: &Value) -> (MbfReviewOutcome, String) {
    let entry = runs.enter();
    let (raw, copy) = reserve(registry, &raw_body(body), &headers(Some(INGRESS_ARGS_JSON_V1)), REVIEW_COMMAND).unwrap();
    let (outcome, text) = review_command(registry, runs, entry, raw, copy, |outcome, text| (outcome.clone(), text)).unwrap();
    assert_eq!(serde_json::to_string(&outcome).unwrap(), text);
    (outcome, text)
}

/// `save_mbf_export` as `lib.rs` runs it, with no library store; `chosen` is the chooser's answer.
fn save_via_command(registry: &SolveJobRegistry, runs: &MbfRuns, body: &Value, chosen: Option<PathBuf>, opened: &Opened) -> MbfExportOutcome {
    let entry = runs.enter();
    let (raw, copy) = reserve(registry, &raw_body(body), &headers(Some(INGRESS_ARGS_JSON_V1)), COMMAND).unwrap();
    let (outcome, text) = save_command(
        registry,
        runs,
        entry,
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
    let entry = runs.enter();
    let (raw, copy) = reserve(registry, &raw_body(body), &headers(Some(INGRESS_ARGS_JSON_V1)), COMMAND).unwrap();
    let (outcome, text) = save_command(registry, runs, entry, raw, copy, || Err("the test has no library store".to_string()), choose, |outcome, text| (outcome.clone(), text)).unwrap();
    assert_eq!(serde_json::to_string(&outcome).unwrap(), text);
    outcome
}

/// A cancel that stopped one run before it wrote anything.
const STOPPED_ONE: MbfCancelReceipt = MbfCancelReceipt { cancelled: 1, too_late: 0 };

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
        let body = review_body(&text, settings);
        let (raw, copy) = reserve(&registry, &raw_body(&body), &headers(Some(INGRESS_ARGS_JSON_V1)), REVIEW_COMMAND).unwrap();
        let held_at_response = Cell::new(0);
        let (outcome, response) = review_command(&registry, &runs, runs.enter(), raw, copy, |outcome, text| {
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
        let refused = review_command(&registry, &runs, runs.enter(), raw, copy, |_, text| text).unwrap_err();
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
        let running = scope.spawn(|| CappedWorkerRun::review(&registry, &ledger, &runs, runs.enter()).run(&request));
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
            let running = scope.spawn(|| CappedWorkerRun::review(&registry, &ledger, &runs, runs.enter()).run(&request));
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
        let running = scope.spawn(|| CappedWorkerRun::review(&registry, &ledger, &runs, runs.enter()).run(&request));
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
    let refused = CappedWorkerRun::review(&registry, args.ledger(), &runs, runs.enter()).with_start_wait(Duration::from_millis(30)).run(&request);
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

/// Finding P2-2 (a): the native session reset (`lib.rs` `reset_native_session`, behind
/// `dispose_all_preview_mechanics_results`) cancels a running MBF review and a queued MBF save, with no model check
/// in progress: the review's worker is killed and reaped, the save never starts a worker or opens a chooser, both
/// reply `MBF-EXPORT-CANCELLED`, nothing is written, and the whole ledger bound, every grant and every lease is
/// released. Without the reset's cancel the running review would never end: the test then fails at its deadline.
#[test]
fn the_session_reset_cancels_a_running_and_a_queued_mbf_run_and_releases_everything() {
    let gate = Gate::new("mbf-reset");
    let registry = registry_with(gate.launcher());
    registry.pool().set_max_running_workers(1);
    let runs = MbfRuns::default();
    let sandbox = Sandbox::new("reset");
    let opened = Opened::new(false);
    let review = review_body(&canonical_model(), &settings());
    let save = save_body(&canonical_model(), recorded_default());
    std::thread::scope(|scope| {
        let _guard = CancelOnUnwind(&runs, &registry);
        let reviewing = scope.spawn(|| review_via_command(&registry, &runs, &review));
        gate.wait_started(1);
        let pid = gate.pids()[0];
        let saving = scope.spawn(|| save_via_command(&registry, &runs, &save, Some(sandbox.0.join("swbpipe-model.mbf")), &opened));
        await_queued(&registry, |refusal| matches!(refusal, ResourceRefusal::WorkerLimitReached { .. }));
        let disposal = crate::reset_native_session(&registry, &runs);
        assert_eq!(disposal.abandoned_jobs, 0, "no solve job was in progress");
        let (reviewed, _) = join_within(&registry, &runs, reviewing, "the running review");
        let saved = join_within(&registry, &runs, saving, "the queued save");
        assert_eq!(review_codes(&reviewed), vec![CANCELLED.to_string()]);
        assert_eq!(save_codes(&saved), vec![CANCELLED.to_string()]);
        assert!(!worker_alive(pid), "the review's worker was killed and reaped");
    });
    assert_eq!(gate.started_count(), 1, "the queued save started no worker");
    assert!(!opened.load(Ordering::SeqCst) && sandbox.is_empty(), "nothing was chosen or written");
    assert_eq!(runs.cancel_all(registry.pool()), MbfCancelReceipt::default(), "the reset cleared both slots");
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

/// How a test retires the session from inside a command (on its own thread, at a hook): `cancel_mbf_export`'s body,
/// or the page-load reset (`lib.rs` `reset_native_session`).
#[derive(Clone, Copy, Debug)]
enum Retire {
    Cancel,
    PageLoadReset,
}

fn retire_from_hook(how: Retire, registry: &SolveJobRegistry, runs: &MbfRuns) -> Box<dyn FnOnce()> {
    let (registry, runs) = (registry.clone(), runs.clone());
    Box::new(move || match how {
        Retire::Cancel => drop(runs.cancel_all(registry.pool())),
        Retire::PageLoadReset => drop(crate::reset_native_session(&registry, &runs)),
    })
}

/// Finding P2 at 6ccc68aff (1): a command owns its cancellation from its entry. The review and the save capture the
/// epoch before their reservation (as `lib.rs` does), and the session is retired while each is paused before its
/// admission, or after its admission and before its run registers (the tests' hooks), by `cancel_mbf_export` or by
/// the page-load reset. Resumed, each replies `MBF-EXPORT-CANCELLED`: the review's run is not registered, the save
/// opens no chooser and writes nothing, and everything is released. With the real worker, a command that ignored
/// its entry would complete (and the save open its chooser): the test fails instead of waiting.
#[test]
fn a_command_retired_before_its_admission_or_registration_is_cancelled_and_writes_nothing() {
    let _lock = atomic_save_test_lock();
    let registry = real_worker();
    type Hook = &'static std::thread::LocalKey<std::cell::RefCell<Option<Box<dyn FnOnce()>>>>;
    let points: [(&str, Hook); 2] = [("before admission", &BEFORE_ADMISSION), ("before registration", &BEFORE_REGISTRATION)];
    for how in [Retire::Cancel, Retire::PageLoadReset] {
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

/// Finding P2 at 6ccc68aff (2), native side: a retirement does not reach a command that enters after it. After
/// `cancel_mbf_export` (and after the page-load reset) a review completes and a save writes the recorded files; an
/// entry captured before the retirement is still refused.
#[test]
fn a_command_that_enters_after_a_retirement_is_not_affected_by_it() {
    let _lock = atomic_save_test_lock();
    let registry = real_worker();
    let runs = MbfRuns::default();
    let before = runs.enter();
    for how in [Retire::Cancel, Retire::PageLoadReset] {
        retire_from_hook(how, &registry, &runs)();
        assert_ne!(runs.enter(), before, "{how:?}: the epoch moved on");
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
            let newer = hook_runs.register(RunKind::Save, hook_runs.enter(), &hook_pool);
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
