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
fn read(path: &str) -> Value {
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
fn read_verdict(registry: &SolveJobRegistry, job_id: &str, model: Option<&str>, invocation: Option<&str>) -> Result<String, CommandError<String>> {
    verdict(registry, &body(&args(job_id, model, invocation)), &headers(Some(INGRESS_ARGS_JSON_V1)))
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
fn the_parse_is_priced_by_the_prescan_materialization() {
    let text = read_text(DEMO);
    assert_eq!(source_parse_bytes(&text).unwrap(), ingress::prescan(text.as_bytes()).unwrap().materialization_bytes);
    // A text the prescan refuses is refused as the reader's own parse refuses it.
    assert!(source_parse_bytes("{\"a\": NaN}").unwrap_err().starts_with("WASM-ENGINE-INPUT-JSON-INVALID: source is not valid JSON: "));
}

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
    match verdict(&registry, &body(&args("job-1", None, None)), &headers(None)) {
        Err(CommandError::Ingress(SolveJobError::InvalidRequest { message })) => assert!(message.starts_with("INGRESS-HEADER-REQUIRED"), "{message}"),
        other => panic!("{other:?}"),
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
    let args_lease = command_ingress::lease_bytes(ingress::prescan(&raw).unwrap(), ArgsDecoding::Moved).unwrap();
    let needed = source_parse_bytes(&text).unwrap() + registry.pool().reserved_total() + args_lease;
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

#[test]
fn argument_texts_that_are_not_json_are_refused_as_the_reader_refuses_them() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let job = retained(&registry, &gate, &read_text(DEMO));
    for (model, invocation, label) in [(Some("{"), None, "model"), (None, Some("[1,"), "invocation")] {
        match read_verdict(&registry, &job, model, invocation).unwrap_err() {
            CommandError::Command(text) => assert!(text.starts_with(&format!("WASM-ENGINE-INPUT-JSON-INVALID: {label} is not valid JSON")), "{text}"),
            other => panic!("{other:?}"),
        }
    }
    // A held text that is not JSON: the reader's own refusal text.
    let bad = retained(&registry, &gate, "{\"a\": 1e999}");
    match read_verdict(&registry, &bad, None, None).unwrap_err() {
        CommandError::Command(text) => {
            assert_eq!(text, open_pipe_stress_operation_applier::reader_dispatch_json("{\"a\": 1e999}", None, None).unwrap_err());
        }
        other => panic!("{other:?}"),
    }
    registry.dispose_result(&job).unwrap();
    registry.dispose_result(&bad).unwrap();
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
    let v = read(path);
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
            let request = read(&format!("fixtures/product_preview/{dir}/{name}.request.json"));
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
    for case in read("fixtures/results/pressure_v3_combination_reader_corpus.json")["cases"].as_array().unwrap() {
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
    read("fixtures/results/base_reader_outcomes.json")["entries"]
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
