//! C1 slice 1: the native verdict on a retained result (`native_verdict`).
//!
//! The differential (plan §4, leg 3): every file-backed statement of the
//! shared base reader corpus (`result_export/tests/base_reader_outcomes.rs`,
//! `corpus()`: the 52 base statements with their models and invocations,
//! the two demo results among them) is published as a real job's result and
//! read through the command. Its outcome text must be the committed native
//! outcome's text (`fixtures/results/base_reader_outcomes.json`), byte for
//! byte; the wasm engine's text is checked against the same committed
//! outcome by `baseReaderWasmEquivalence.test.ts`. The ignored leg reads all
//! 561 corpus inputs from result_export's `BASE_READER_INPUTS_OUT` output.
use super::*;
use crate::command_ingress::test_support::headers;
use crate::command_ingress::INGRESS_ARGS_JSON_V1;
use crate::memory_pool::test_host::ScriptedHost;
use crate::memory_pool::{MemoryPool, PressureLevel, ResourceRefusal};
use crate::solve_jobs::test_support::wait_terminal;
use crate::solve_jobs::SolveJobError;
use crate::solve_worker::test_support::Gate;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

const MODEL: &[u8] = br#"{"project":{"id":"p"},"nodes":[1,2,3]}"#;

fn root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}
fn read_text(path: &str) -> String {
    std::fs::read_to_string(root().join(path)).unwrap_or_else(|e| panic!("{path}: {e}"))
}
fn read_json(path: &str) -> Value {
    serde_json::from_str(&read_text(path)).unwrap()
}

/// A registry whose jobs' workers return the text their gate releases. The
/// re-sampling window is short, so a headroom refusal stands quickly.
fn gated(host: &Arc<ScriptedHost>) -> (SolveJobRegistry, Arc<Gate>) {
    let gate = Gate::new("verdict");
    let pool = MemoryPool::with_timing(host.clone(), Duration::from_millis(5), Duration::from_millis(40), Duration::from_millis(5));
    pool.set_max_running_workers(16);
    let registry = SolveJobRegistry::with_hooks(pool, gate.launcher(), Arc::new(|job: Box<dyn FnOnce() + Send + 'static>| std::thread::Builder::new().spawn(job).map(drop)));
    (registry, gate)
}

/// A completed job retaining `text` as its result.
fn retained(registry: &SolveJobRegistry, gate: &Gate, text: &str) -> String {
    let started = gate.started_count();
    let receipt = registry.start(MODEL, None).unwrap_or_else(|e| panic!("{e:?}"));
    gate.wait_started(started + 1);
    gate.release(text);
    let status = wait_terminal(registry, &receipt.job_id);
    assert_eq!(status.state, "completed", "{status:?}");
    receipt.job_id
}

fn body(args: &Value) -> InvokeBody {
    InvokeBody::Raw(serde_json::to_vec(args).unwrap())
}
fn args(job_id: &str, model: Option<&str>, invocation: Option<&str>) -> Value {
    json!({"jobId": job_id, "modelJson": model, "invocationJson": invocation})
}
/// The command's two steps in this thread: the executor's reservation, then the blocking part.
fn read_verdict(registry: &SolveJobRegistry, job_id: &str, model: Option<&str>, invocation: Option<&str>) -> Result<String, CommandError<String>> {
    let (raw, lease) = reserve(registry, &body(&args(job_id, model, invocation)), &headers(Some(INGRESS_ARGS_JSON_V1)))?;
    run(registry.clone(), raw, lease)
}

/// A test gate in [`run`], per registry (by its pool's address, unique
/// across the tests' registries, unlike job ids) and stage: `admit` as the
/// blocking part starts, before the arguments' admission; `read` after the
/// leases are admitted and before the reader runs. It counts each entry and
/// waits while it is closed.
pub(crate) mod gate {
    use std::sync::{Arc, Condvar, Mutex};
    use std::time::{Duration, Instant};

    pub(crate) struct Gate {
        key: String,
        state: Mutex<(bool, usize)>,
        changed: Condvar,
    }
    static GATES: Mutex<Vec<Arc<Gate>>> = Mutex::new(Vec::new());

    /// Registers a gate for `job`, open or closed; it is removed (and opened) on drop.
    pub(crate) struct Held(pub Arc<Gate>);
    impl Drop for Held {
        fn drop(&mut self) {
            self.0.open();
            GATES.lock().unwrap().retain(|g| !Arc::ptr_eq(g, &self.0));
        }
    }
    pub(crate) fn key(registry: &crate::solve_jobs::SolveJobRegistry, stage: &str) -> String {
        format!("{:p}:{stage}", Arc::as_ptr(registry.pool()))
    }
    pub(crate) fn hold(registry: &crate::solve_jobs::SolveJobRegistry, stage: &str, open: bool) -> Held {
        let gate = Arc::new(Gate { key: key(registry, stage), state: Mutex::new((open, 0)), changed: Condvar::new() });
        GATES.lock().unwrap().push(gate.clone());
        Held(gate)
    }
    impl Gate {
        pub(crate) fn open(&self) {
            self.state.lock().unwrap().0 = true;
            self.changed.notify_all();
        }
        pub(crate) fn entered(&self) -> usize {
            self.state.lock().unwrap().1
        }
        /// Whether `count` reads have entered within `limit`.
        pub(crate) fn wait_entered(&self, count: usize, limit: Duration) -> bool {
            let deadline = Instant::now() + limit;
            let mut state = self.state.lock().unwrap();
            while state.1 < count {
                let now = Instant::now();
                if now >= deadline {
                    return false;
                }
                state = self.changed.wait_timeout(state, deadline - now).unwrap().0;
            }
            true
        }
    }
    pub(crate) fn pass(key: String) {
        let gate = GATES.lock().unwrap().iter().find(|g| g.key == key).cloned();
        let Some(gate) = gate else { return };
        let mut state = gate.state.lock().unwrap();
        state.1 += 1;
        gate.changed.notify_all();
        while !state.0 {
            state = gate.changed.wait(state).unwrap();
        }
    }
}
fn invalid_message(error: CommandError<String>) -> String {
    match error {
        CommandError::Ingress(SolveJobError::InvalidRequest { message }) => message,
        other => panic!("not an invalid request: {other:?}"),
    }
}
fn refusal(error: CommandError<String>) -> ResourceRefusal {
    match error {
        CommandError::Ingress(SolveJobError::ResourceUnavailable { refusal, .. }) => refusal,
        other => panic!("not a resource refusal: {other:?}"),
    }
}

const DEMO: &str = "fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json";

#[test]
fn the_verdict_reads_a_retained_result_which_stays_retained_and_takeable() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let text = read_text(DEMO);
    let job = retained(&registry, &gate, &text);
    let held = registry.pool().reserved_total();
    let got = read_verdict(&registry, &job, None, None).unwrap();
    assert_eq!(got, open_pipe_stress_operation_applier::reader_dispatch_json(&text, None, None).unwrap());
    // The verdict's own leases are released; the result is still retained and takeable.
    assert_eq!(registry.pool().reserved_total(), held);
    assert!(registry.status(&job).unwrap().result_available);
    assert_eq!(registry.take_result(&job).unwrap(), text.as_bytes());
    // A delivered result is no longer held natively.
    let message = invalid_message(read_verdict(&registry, &job, None, None).unwrap_err());
    assert!(message.starts_with("PREVIEW-SOLVE-RESULT-ALREADY-DELIVERED"), "{message}");
    registry.dispose_result(&job).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

#[test]
fn the_verdict_needs_a_known_job_holding_its_result() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let message = invalid_message(read_verdict(&registry, "job-404", None, None).unwrap_err());
    assert!(message.starts_with("unknown solve job"), "{message}");
    // A running job holds no result yet.
    let receipt = registry.start(MODEL, None).unwrap();
    gate.wait_started(1);
    let message = invalid_message(read_verdict(&registry, &receipt.job_id, None, None).unwrap_err());
    assert!(message.starts_with("PREVIEW-SOLVE-RESULT-UNAVAILABLE"), "{message}");
    gate.release("{}");
    wait_terminal(&registry, &receipt.job_id);
    registry.dispose_result(&receipt.job_id).unwrap();
    // The args are admitted as every raw-body command's.
    match reserve(&registry, &body(&args("job-1", None, None)), &headers(None)) {
        Err(CommandError::Ingress(SolveJobError::InvalidRequest { message })) => assert!(message.starts_with("INGRESS-HEADER-REQUIRED"), "{message}"),
        Err(other) => panic!("{other:?}"),
        Ok(_) => panic!("reserved without the ingress header"),
    }
}

/// The parse's lease is exactly the prescan's materialization: admitted when
/// the headroom covers it with the outstanding reservations, refused by name
/// one byte below, and refused under elevated pressure (adaptive).
#[test]
fn the_parse_is_admitted_through_the_pool_and_refused_by_name_when_it_does_not_fit() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let text = read_text(DEMO);
    let job = retained(&registry, &gate, &text);
    let request = args(&job, None, None);
    let raw = serde_json::to_vec(&request).unwrap();
    // The raw body's copy, then the args' admission bound (`reserve`, `run`).
    let args_lease = ingress::allocation_envelope(raw.len() as u64).unwrap()
        + command_ingress::lease_bytes(ingress::prescan(&raw).unwrap(), ArgsDecoding::Moved).unwrap();
    let needed = ingress::prescan(text.as_bytes()).unwrap().materialization_bytes + registry.pool().reserved_total() + args_lease;
    host.set(|s| s.headroom_bytes = Some(needed - 1));
    match refusal(read_verdict(&registry, &job, None, None).unwrap_err()) {
        ResourceRefusal::HeadroomInsufficient { required_bytes, available_bytes } => {
            assert_eq!((required_bytes, available_bytes), (needed, needed - 1));
        }
        other => panic!("{other:?}"),
    }
    host.set(|s| s.headroom_bytes = Some(needed));
    assert!(read_verdict(&registry, &job, None, None).is_ok());
    host.set(|s| s.pressure = PressureLevel::Warning);
    assert!(matches!(refusal(read_verdict(&registry, &job, None, None).unwrap_err()), ResourceRefusal::PressureElevated { .. }));
    // Nothing the verdict admitted is left behind.
    host.set(|s| s.pressure = PressureLevel::Normal);
    registry.dispose_result(&job).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// A native reader's share keeps the bytes and their lease together: the take
/// is refused busy while it is held, and a disposal while it is held releases
/// the bytes' lease only when the share is dropped.
#[test]
fn a_reader_share_holds_the_bytes_and_their_lease_together() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let job = retained(&registry, &gate, "{\"held\":true}");
    let result_lease = registry.pool().reserved_total();
    assert!(result_lease > 0);
    let share = registry.held_result(&job).unwrap();
    let message = match registry.take_result(&job).unwrap_err() {
        SolveJobError::InvalidRequest { message } => message,
        other => panic!("{other:?}"),
    };
    assert!(message.starts_with("PREVIEW-SOLVE-RESULT-BUSY"), "{message}");
    // The refused take released its growth and left the result retained.
    assert_eq!(registry.pool().reserved_total(), result_lease);
    assert!(registry.status(&job).unwrap().result_available);
    drop(share);
    assert_eq!(registry.take_result(&job).unwrap(), b"{\"held\":true}");
    registry.dispose_result(&job).unwrap();

    let job = retained(&registry, &gate, "{\"held\":2}");
    let share = registry.held_result(&job).unwrap();
    let receipt = registry.dispose_result(&job).unwrap();
    assert!(receipt.released);
    assert_eq!(receipt.released_bytes, result_lease);
    assert_eq!(registry.pool().reserved_total(), result_lease, "the share still holds the bytes");
    assert!(!registry.status(&job).unwrap().result_available);
    assert_eq!(share.text(), "{\"held\":2}");
    drop(share);
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// One corpus input: its id, the statement text, and its model and invocation texts.
struct Input {
    id: String,
    source: String,
    model: Option<String>,
    invocation: Option<String>,
}

fn text(value: &Value) -> String {
    serde_json::to_string(value).unwrap()
}
fn model_at(path: &str) -> Value {
    let v = read_json(path);
    if v.get("model").is_some() { v["model"].clone() } else { v }
}

/// The corpus's file-backed statements, as `base_reader_outcomes.rs`
/// `build_corpus` builds them (its first 52 entries), each statement as its
/// file's own text.
fn file_backed_corpus() -> Vec<Input> {
    let mut inputs = Vec::new();
    let results: [(&str, Option<&str>); 17] = [
        ("preview_physics_connected_sparse", Some("fixtures/model_operations/precision_connected_ui_model.json")),
        ("preview_physics_connected_dense", Some("fixtures/model_operations/precision_connected_ui_model.json")),
        ("preview_physics_invented_sparse", Some("core/product_physics/tests/fixtures/preview_physics_invented_model.json")),
        ("preview_physics_invented_dense", Some("core/product_physics/tests/fixtures/preview_physics_invented_model.json")),
        ("preview_physics_unicode_ids_sparse", Some("core/product_physics/tests/fixtures/preview_physics_unicode_ids_model.json")),
        ("pressure_connected_mechanics_sparse", Some("core/product_physics/tests/fixtures/exact_pressure_connected_request.json")),
        ("pressure_connected_mechanics_dense", Some("core/product_physics/tests/fixtures/exact_pressure_connected_request.json")),
        ("pressure_connected_ui_mechanics_sparse", Some("fixtures/model_operations/exact_pressure_authoring_model.json")),
        ("pressure_connected_ui_mechanics_dense", Some("fixtures/model_operations/exact_pressure_authoring_model.json")),
        ("pressure_thermal_ui_mechanics_sparse", Some("fixtures/model_operations/physics_thermal_ui_model.json")),
        ("pressure_thermal_ui_mechanics_dense", Some("fixtures/model_operations/physics_thermal_ui_model.json")),
        ("pressure_u4b_arc_sparse", Some("core/product_physics/tests/fixtures/u4b_l90_p_model.json")),
        ("preview_physics_u4b_arc_sparse", Some("core/product_physics/tests/fixtures/u4b_z_preview_rev_model.json")),
        ("../product_preview/invented_mechanics_result_preview_physics_1_sparse", None),
        ("../product_preview/invented_mechanics_result_preview_physics_1_dense", None),
        ("../product_preview/invented_demo_result_preview_physics_1_sparse", None),
        ("../product_preview/invented_demo_result_preview_physics_1_dense", None),
    ];
    for (name, model) in results {
        inputs.push(Input {
            id: format!("results/{}", name.trim_start_matches(['.', '/'])),
            source: read_text(&format!("fixtures/results/{name}.json")),
            model: model.map(|path| text(&model_at(path))),
            invocation: None,
        });
    }
    let modes = ["sparse_interactive", "dense_scrutiny"];
    for (dir, names, invoked) in [
        ("pressure_source/physics_source", &["n05", "n06", "fields", "mixed", "mixed_units", "n05_units", "n05_unicode"][..], true),
        ("pressure/load_reference", &["connected", "pressure"][..], false),
        ("pressure_source/load_reference_source", &["n05", "n06", "fields", "mixed", "eigen_motion"][..], true),
    ] {
        for name in names {
            let request = read_json(&format!("fixtures/product_preview/{dir}/{name}.request.json"));
            for mode in modes {
                inputs.push(Input {
                    id: format!("{dir}/{name}-{mode}"),
                    source: read_text(&format!("fixtures/product_preview/{dir}/{name}-{mode}.raw.json")),
                    model: Some(text(&request["model"])),
                    invocation: invoked.then(|| text(&json!({"request": request, "solver_mode": mode}))),
                });
            }
        }
    }
    let fallback = text(&model_at("core/reporting/result_export/tests/fixtures/load_reference_fallback_uz.request.json"));
    for mode in modes {
        inputs.push(Input {
            id: format!("load_reference_fallback_uz-{mode}"),
            source: read_text(&format!("core/reporting/result_export/tests/fixtures/load_reference_fallback_uz-{mode}.raw.json")),
            model: Some(fallback.clone()),
            invocation: None,
        });
    }
    for case in read_json("fixtures/results/pressure_v3_combination_reader_corpus.json")["cases"].as_array().unwrap() {
        inputs.push(Input {
            id: format!("combination/{}", case["label"].as_str().unwrap()),
            source: text(&case["envelope"]),
            model: Some(text(&case["model"])),
            invocation: None,
        });
    }
    inputs
}

/// The committed native outcomes by id.
fn committed() -> BTreeMap<String, Value> {
    read_json("fixtures/results/base_reader_outcomes.json")["entries"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| (entry["id"].as_str().unwrap().to_string(), entry.clone()))
        .collect()
}

/// Each input read through the command from a retained job; returns the
/// count checked. A statement the readers cannot parse must be refused with
/// the reader's parse refusal, as the committed outcome records.
fn check_inputs(inputs: &[Input]) -> usize {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let outcomes = committed();
    let mut misses = Vec::new();
    for input in inputs {
        let entry = outcomes.get(&input.id).unwrap_or_else(|| panic!("{} is not a committed corpus id", input.id));
        let job = retained(&registry, &gate, &input.source);
        let got = read_verdict(&registry, &job, input.model.as_deref(), input.invocation.as_deref());
        match (got, entry.get("dispatch")) {
            (Ok(got), Some(dispatch)) => {
                // The committed outcome as the readers write it: serde_json's
                // compact text of the same value, keys in order.
                if got != text(dispatch) {
                    misses.push(format!("{}: {got} != {}", input.id, text(dispatch)));
                }
            }
            (Err(CommandError::Command(refused)), None) if entry["parse"] == "JSON_PARSE_REJECTED" && refused.starts_with(INPUT_JSON_INVALID) => {}
            (other, _) => misses.push(format!("{}: {other:?} against {entry}", input.id)),
        }
        registry.dispose_result(&job).unwrap();
    }
    assert_eq!(registry.pool().reserved_total(), 0);
    assert!(misses.is_empty(), "{} verdicts differ:\n{}", misses.len(), misses.join("\n"));
    inputs.len()
}

#[test]
fn native_verdicts_are_the_committed_reader_outcomes_on_the_file_backed_corpus() {
    let inputs = file_backed_corpus();
    // The corpus order: these are its first 52 entries.
    let ids: Vec<String> = committed().keys().cloned().collect();
    assert_eq!(inputs.len(), 52);
    assert!(inputs.iter().all(|input| ids.contains(&input.id)));
    assert_eq!(check_inputs(&inputs), 52);
}

/// All 561 corpus inputs, from `inputs.jsonl` written by
/// `BASE_READER_INPUTS_OUT=<dir> cargo test --test base_reader_outcomes` in
/// `core/reporting/result_export`. Run with
/// `BASE_READER_INPUTS_OUT=<dir> cargo test native_verdict -- --ignored`.
#[test]
#[ignore = "needs result_export's BASE_READER_INPUTS_OUT inputs"]
fn native_verdicts_are_the_committed_reader_outcomes_on_the_whole_corpus() {
    let dir = std::env::var("BASE_READER_INPUTS_OUT").expect("BASE_READER_INPUTS_OUT");
    let lines = std::fs::read_to_string(std::path::Path::new(&dir).join("inputs.jsonl")).unwrap();
    let inputs: Vec<Input> = lines
        .lines()
        .map(|line| {
            let v: Value = serde_json::from_str(line).unwrap();
            let opt = |key: &str| v[key].as_str().map(String::from);
            Input { id: v["id"].as_str().unwrap().into(), source: v["source_json"].as_str().unwrap().into(), model: opt("model_json"), invocation: opt("invocation_json") }
        })
        .collect();
    assert_eq!(inputs.len(), committed().len());
    assert_eq!(check_inputs(&inputs), 561);
}

/// The worker threads of the runtime the command's futures run on (Tauri's),
/// as that runtime reports them: an environment that configures more workers
/// (`TOKIO_WORKER_THREADS`) raises the reads the executor tests hold with it.
fn runtime_workers() -> usize {
    let workers = tauri::async_runtime::handle().inner().metrics().num_workers();
    // Each held read occupies one of the blocking pool's 512 threads.
    assert!(workers > 0 && workers * 2 + 2 <= 512, "{workers} runtime workers");
    workers
}

/// Review F1 and R1: the arguments' admission and the read run in a blocking
/// task, never on the async runtime's workers. At each stage, more commands
/// than the runtime has workers are held inside it at once (at admission with
/// a large model text each), and another async command still completes.
/// Run on the executor instead, at most one held command per worker enters.
#[test]
fn admission_and_reads_run_off_the_async_executor() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let text = read_text(DEMO);
    let job = retained(&registry, &gate, &text);
    // A model text of 2 MiB: admission prescans and parses it.
    let model = format!("{{\"pad\":\"{}\"}}", "x".repeat(2 << 20));
    let expected = open_pipe_stress_operation_applier::reader_dispatch_json(&text, Some(&model), None).unwrap();
    let commands = runtime_workers() * 2 + 2;
    for stage in ["admit", "read"] {
        let held = gate::hold(&registry, stage, false);
        let handles: Vec<_> = (0..commands)
            .map(|_| {
                let (registry, body) = (registry.clone(), body(&args(&job, Some(&model), None)));
                tauri::async_runtime::spawn(async move { verdict(&registry, &body, &headers(Some(INGRESS_ARGS_JSON_V1))).await })
            })
            .collect();
        assert!(held.0.wait_entered(commands, Duration::from_secs(60)), "{stage}: only {} of {commands} commands entered", held.0.entered());
        // Another command, on the same runtime, while every one is held.
        let (sender, receiver) = std::sync::mpsc::channel();
        let (probe_registry, probe_job) = (registry.clone(), job.clone());
        tauri::async_runtime::spawn(async move {
            let _ = sender.send(probe_registry.status(&probe_job).map(|status| status.state));
        });
        let probe = receiver.recv_timeout(Duration::from_secs(10));
        assert_eq!(probe.unwrap_or_else(|_| panic!("{stage}: the runtime ran no other task")).unwrap(), "completed");
        held.0.open();
        for handle in handles {
            assert_eq!(tauri::async_runtime::block_on(handle).unwrap().unwrap(), expected, "{stage}");
        }
        drop(held);
    }
    registry.dispose_result(&job).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// Review F2: a source, model or invocation text that is not JSON is refused
/// with the reader's own text, byte for byte: the text
/// `operation_applier::reader_dispatch_json` returns (the wasm engine's, which
/// `baseReaderWasmEquivalence.test.ts` pins to the same literals). The
/// source is checked first, then the model, then the invocation, as the
/// reader parses them.
pub(crate) const NOT_JSON_CASES: [(&str, &str, &str, &str); 9] = [
    // (where, the bad text, the model's text when the bad text is the source, the refusal)
    ("model", "{", "", "WASM-ENGINE-INPUT-JSON-INVALID: model is not valid JSON: EOF while parsing an object at line 1 column 1"),
    ("invocation", "[1,", "", "WASM-ENGINE-INPUT-JSON-INVALID: invocation is not valid JSON: EOF while parsing a value at line 1 column 3"),
    ("model", "\"\\ud800\"", "", "WASM-ENGINE-INPUT-JSON-INVALID: model is not valid JSON: unexpected end of hex escape at line 1 column 8"),
    ("model", "[1e999]", "", "WASM-ENGINE-INPUT-JSON-INVALID: model is not valid JSON: number out of range at line 1 column 6"),
    ("invocation", "{\"a\":1} x", "", "WASM-ENGINE-INPUT-JSON-INVALID: invocation is not valid JSON: trailing characters at line 1 column 9"),
    ("source", "{\"a\": NaN}", "{", "WASM-ENGINE-INPUT-JSON-INVALID: source is not valid JSON: expected value at line 1 column 7"),
    ("source", "{\"a\": 1e999}", "{", "WASM-ENGINE-INPUT-JSON-INVALID: source is not valid JSON: number out of range at line 1 column 11"),
    ("source", "", "", "WASM-ENGINE-INPUT-JSON-INVALID: source is not valid JSON: recursion limit exceeded at line 1 column 128"),
    // A model the prescan admits and serde_json refuses, before a refused invocation.
    ("invocation+model", "[1,", "[1e999]", "WASM-ENGINE-INPUT-JSON-INVALID: model is not valid JSON: number out of range at line 1 column 6"),
];

/// A text nested 128 deep, beyond serde_json's recursion limit.
fn deep() -> String {
    "[".repeat(128) + &"]".repeat(128)
}

#[test]
fn texts_that_are_not_json_are_refused_with_the_readers_exact_text() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let good = read_text(DEMO);
    let good_job = retained(&registry, &gate, &good);
    for (place, bad, model, refusal) in NOT_JSON_CASES {
        let deep_text = deep();
        let bad = if bad.is_empty() { deep_text.as_str() } else { bad };
        let (source, job, model, invocation) = match place {
            "source" => {
                let job = retained(&registry, &gate, bad);
                (bad.to_string(), job, (!model.is_empty()).then_some(model), None)
            }
            "model" => (good.clone(), good_job.clone(), Some(bad), None),
            "invocation+model" => (good.clone(), good_job.clone(), Some(model), Some(bad)),
            _ => (good.clone(), good_job.clone(), None, Some(bad)),
        };
        let reader = open_pipe_stress_operation_applier::reader_dispatch_json(&source, model, invocation).unwrap_err();
        match read_verdict(&registry, &job, model, invocation).unwrap_err() {
            CommandError::Command(text) => {
                assert_eq!(text, reader, "{place} {bad}");
                assert_eq!(text, refusal, "{place} {bad}");
            }
            other => panic!("{place} {bad}: {other:?}"),
        }
        if job != good_job {
            registry.dispose_result(&job).unwrap();
        }
    }
    registry.dispose_result(&good_job).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// Review F3: the one exception. A text with serde_json's private `RawValue`
/// token as an object key is refused here, in the held result and in the
/// caller's model and invocation alike, by the prescan's own text; serde_json
/// without `raw_value` (the wasm engine's) reads it as an ordinary key
/// (`reader_parse_error` finds nothing, and `baseReaderWasmEquivalence.test.ts`
/// reads these texts through the wasm engine).
pub(crate) const RAW_VALUE_TEXT: &str = "{\"$serde_json::private::RawValue\":1}";

#[test]
fn a_raw_value_token_key_is_the_one_refused_exception() {
    assert_eq!(reader_parse_error("model", RAW_VALUE_TEXT), None);
    let reason = ingress::prescan(RAW_VALUE_TEXT.as_bytes()).unwrap_err();
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let good = read_text(DEMO);
    let good_job = retained(&registry, &gate, &good);
    let raw_job = retained(&registry, &gate, RAW_VALUE_TEXT);
    for (label, job, model, invocation) in [
        ("source", &raw_job, None, None),
        ("model", &good_job, Some(RAW_VALUE_TEXT), None),
        ("invocation", &good_job, None, Some(RAW_VALUE_TEXT)),
    ] {
        let want = format!("WASM-ENGINE-INPUT-JSON-INVALID: {label} is not valid JSON: {} at byte {}", reason.reason, reason.offset);
        match read_verdict(&registry, job, model, invocation).unwrap_err() {
            CommandError::Command(text) => assert_eq!(text, want),
            other => panic!("{label}: {other:?}"),
        }
    }
    assert_eq!(
        format!("{} at byte {}", reason.reason, reason.offset),
        "reserved RawValue token key at byte 33",
        "the prescan's reason, as the desktop shows it"
    );
    registry.dispose_result(&good_job).unwrap();
    registry.dispose_result(&raw_job).unwrap();
}

/// Ruling B2 (a): the parse's lease is sized from the held bytes by the
/// allocation-free prescan and admitted before the parse allocates anything,
/// so a refusal commits no memory and never reaches the reader.
#[test]
fn a_refused_parse_commits_no_memory_and_never_reaches_the_reader() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let text = read_text(DEMO);
    let job = retained(&registry, &gate, &text);
    let materialization = ingress::prescan(text.as_bytes()).unwrap().materialization_bytes;
    let held = gate::hold(&registry, "read", true);
    let (raw, lease) = reserve(&registry, &body(&args(&job, None, None)), &headers(Some(INGRESS_ARGS_JSON_V1))).unwrap();
    host.set(|s| s.headroom_bytes = Some(materialization / 2));
    let handle = registry.clone();
    let (outcome, measured) = ingress::counting::measure(|| run(handle, raw, lease));
    assert!(matches!(refusal(outcome.unwrap_err()), ResourceRefusal::HeadroomInsufficient { .. }));
    assert_eq!(held.0.entered(), 0, "the reader was reached");
    // Only the refusal and its pool status were allocated: nothing near the parse.
    assert!(measured.peak_priced < 64 * 1024 && measured.peak_priced * 100 < materialization, "{measured:?} against {materialization}");
    host.set(|s| s.headroom_bytes = Some(64 * crate::memory_pool::test_host::GIB));
    assert!(read_verdict(&registry, &job, None, None).is_ok());
    assert_eq!(held.0.entered(), 1);
    drop(held);
    registry.dispose_result(&job).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// Ruling B2 (b): each parse lease is capped by the per-invocation budget and
/// never above the owner-adopted 12 GiB default, whatever budget is
/// configured: above it the lease is refused `ingress_exceeds_budget` (not
/// re-sampled), at it the lease is admitted.
#[test]
fn the_parse_lease_is_capped_at_the_12_gib_default() {
    use crate::memory_pool::test_host::GIB;
    use crate::memory_pool::DEFAULT_INVOCATION_BUDGET_BYTES;
    assert_eq!(DEFAULT_INVOCATION_BUDGET_BYTES, 12 * GIB);
    let host = ScriptedHost::with(Some(256 * GIB), Some(1024 * GIB), PressureLevel::Normal);
    let registry = SolveJobRegistry::new(MemoryPool::with_timing(host, Duration::from_millis(5), Duration::from_millis(40), Duration::from_millis(5)));
    let cap = |registry: &SolveJobRegistry, bytes: u64| match registry.acquire_result_parse(bytes) {
        Ok(lease) => Ok(lease.bytes()),
        Err(SolveJobError::ResourceUnavailable { refusal, .. }) => Err(refusal),
        Err(other) => panic!("{other:?}"),
    };
    let above = |bytes: u64, budget: u64| Err(ResourceRefusal::IngressExceedsBudget { required_bytes: bytes, budget_bytes: budget });
    // The default budget.
    assert_eq!(cap(&registry, 12 * GIB), Ok(12 * GIB));
    assert_eq!(cap(&registry, 12 * GIB + 1), above(12 * GIB + 1, 12 * GIB));
    // A larger configured budget does not raise the cap.
    registry.configure_pool(192 * GIB, 64 * GIB, false).unwrap();
    assert_eq!(cap(&registry, 12 * GIB + 1), above(12 * GIB + 1, 12 * GIB));
    assert_eq!(cap(&registry, 12 * GIB), Ok(12 * GIB));
    // A smaller one lowers it.
    registry.configure_pool(192 * GIB, 4 * GIB, false).unwrap();
    assert_eq!(cap(&registry, 4 * GIB + 1), above(4 * GIB + 1, 4 * GIB));
    assert_eq!(cap(&registry, 4 * GIB), Ok(4 * GIB));
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// Review R2: the diagnostic parse that finds the reader's refusal is priced
/// and admitted before it allocates. A held source with an escaped string
/// decoding to 2 MiB needs a scratch of that order to be read; with a malformed model
/// and too little headroom for it, the refusal comes first and nothing
/// proportional to either text is allocated. With headroom, the reader's
/// exact refusal, in its order, follows. The priced bounds cover what the
/// diagnostic parse allocates, for an admitted and a refused text.
#[test]
fn diagnostics_are_admitted_before_they_allocate() {
    let escaped = "\\n".repeat(2 << 20);
    let source = format!("{{\"s\":\"{escaped}\"}}");
    let (_, allowances) = ingress::prescan_allowances(source.as_bytes()).unwrap();
    let (_, measured) = ingress::counting::measure(|| reader_parse_error("source", &source));
    assert!(measured.peak_priced >= 2 << 20 && measured.peak_priced <= allowances, "{measured:?} against {allowances}");
    let refused = format!("{{\"s\":\"{escaped}");
    let (error, measured) = ingress::counting::measure(|| reader_parse_error("model", &refused));
    assert!(error.is_some());
    let bound = ingress::length_allowances(refused.len() as u64).unwrap();
    assert!(measured.peak_priced >= 2 << 20 && measured.peak_priced <= bound, "{measured:?} against {bound}");

    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let job = retained(&registry, &gate, &source);
    let (raw, lease) = reserve(&registry, &body(&args(&job, Some("{"), None)), &headers(Some(INGRESS_ARGS_JSON_V1))).unwrap();
    host.set(|s| s.headroom_bytes = Some(allowances / 2));
    let handle = registry.clone();
    let (outcome, measured) = ingress::counting::measure(|| run(handle, raw, lease));
    assert!(matches!(refusal(outcome.unwrap_err()), ResourceRefusal::HeadroomInsufficient { .. }));
    assert!(measured.peak_priced < 64 * 1024, "{measured:?}");
    host.set(|s| s.headroom_bytes = Some(64 * crate::memory_pool::test_host::GIB));
    match read_verdict(&registry, &job, Some("{"), None).unwrap_err() {
        CommandError::Command(text) => {
            assert_eq!(text, "WASM-ENGINE-INPUT-JSON-INVALID: model is not valid JSON: EOF while parsing an object at line 1 column 1");
            assert_eq!(text, open_pipe_stress_operation_applier::reader_dispatch_json(&source, Some("{"), None).unwrap_err());
        }
        other => panic!("{other:?}"),
    }
    registry.dispose_result(&job).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// Review R3: the args lease, at admission and at every growth, is capped by
/// the per-invocation budget and never above the 12 GiB default, under the
/// pool lock, as the parse lease is. Checked on the pool method and on the
/// lease the command's own reservation takes.
#[test]
fn the_args_lease_is_capped_at_the_12_gib_default() {
    use crate::memory_pool::test_host::GIB;
    let host = ScriptedHost::with(Some(256 * GIB), Some(1024 * GIB), PressureLevel::Normal);
    let registry = SolveJobRegistry::new(MemoryPool::with_timing(host, Duration::from_millis(5), Duration::from_millis(40), Duration::from_millis(5)));
    let admitted = |bytes: u64| match registry.acquire_interactive_ingress_capped(bytes) {
        Ok(lease) => Ok(lease.bytes()),
        Err(SolveJobError::ResourceUnavailable { refusal, .. }) => Err(refusal),
        Err(other) => panic!("{other:?}"),
    };
    let above = |bytes: u64, budget: u64| Err(ResourceRefusal::IngressExceedsBudget { required_bytes: bytes, budget_bytes: budget });
    let grown = |bytes: u64| {
        let (_, mut lease) = reserve(&registry, &body(&args("job-1", None, None)), &headers(Some(INGRESS_ARGS_JSON_V1))).unwrap();
        lease.grow_to(bytes).map(|()| lease.bytes())
    };
    for (budget, cap) in [(None, 12 * GIB), (Some(64 * GIB), 12 * GIB), (Some(4 * GIB), 4 * GIB)] {
        if let Some(budget) = budget {
            registry.configure_pool(192 * GIB, budget, false).unwrap();
        }
        assert_eq!(admitted(cap), Ok(cap), "{budget:?}");
        assert_eq!(admitted(cap + 1), above(cap + 1, cap), "{budget:?}");
        assert_eq!(grown(cap), Ok(cap), "{budget:?}");
        assert_eq!(grown(cap + 1).map_err(Some), above(cap + 1, cap).map_err(Some), "{budget:?}");
    }
    // The ordinary interactive lease keeps the configured budget (64 GiB here).
    registry.configure_pool(192 * GIB, 64 * GIB, false).unwrap();
    assert!(registry.acquire_interactive_ingress(13 * GIB).is_ok());
    assert_eq!(registry.pool().reserved_total(), 0);
}
