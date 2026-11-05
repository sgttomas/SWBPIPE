//! C1 slice 4 (DEL-04-05 `c1-native-custody-plan.md`): the webview's native replay replies come from
//! the actual commands. `apps/desktop/src/test-support/native-replay/native_replay_requests.json`
//! lists the requests the webview's tests send (recorded by `test/nativeReplayStore.ts` from the
//! actual session and panels, each with the source bytes the job holds: a fixture path, or a text
//! kept once in `sources` by its digest). Each request is run here
//! through its command's own admission and blocking body, on a native-custody job holding those
//! bytes in a real registry, and its reply is recorded as the webview receives it: the verdict's
//! JSON value (Tauri delivers a JSON body parsed), the byte responses as base64 (an ArrayBuffer),
//! a refusal as its serialized rejection. The replies must equal
//! `native_replay_outcomes.json` byte for byte; `NATIVE_REPLAY_OUTCOMES_WRITE=1` rewrites it.
use crate::command_ingress::test_support::headers;
use crate::command_ingress::{CommandError, INGRESS_ARGS_JSON_V1};
use crate::memory_pool::test_host::ScriptedHost;
use crate::memory_pool::MemoryPool;
use crate::solve_jobs::test_support::wait_terminal;
use crate::solve_jobs::{ResultCustody, SolveJobRegistry};
use crate::solve_worker::test_support::Gate;
use crate::{analysis_run_v03_build, native_verdict, result_detail, result_summary};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use std::time::Duration;
use tauri::ipc::InvokeBody;

const REQUESTS: &str = "apps/desktop/src/test-support/native-replay/native_replay_requests.json";
const OUTCOMES: &str = "apps/desktop/src/test-support/native-replay/native_replay_outcomes.json";
const JOB: &str = "$JOB";

fn root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

pub(crate) fn registry() -> (SolveJobRegistry, Arc<Gate>) {
    let host = ScriptedHost::normal();
    let gate = Gate::new("native-replay");
    let pool = MemoryPool::with_timing(host, Duration::from_millis(5), Duration::from_millis(40), Duration::from_millis(5));
    pool.set_max_running_workers(4);
    let registry = SolveJobRegistry::with_hooks(pool, gate.launcher(), Arc::new(|job: Box<dyn FnOnce() + Send + 'static>| std::thread::Builder::new().spawn(job).map(drop)));
    (registry, gate)
}

/// A native-custody job holding `text`, as a completed desktop solve leaves it.
pub(crate) fn held(registry: &SolveJobRegistry, gate: &Gate, text: &str) -> String {
    let receipt = registry.start_with_custody(br#"{"project":{"id":"p"}}"#, None, ResultCustody::Native).unwrap();
    gate.wait_started(1);
    gate.release(text);
    let status = wait_terminal(registry, &receipt.job_id);
    assert_eq!(status.state, "completed", "{status:?}");
    receipt.job_id
}

/// RT-A4 §10.4: a restored handle holding `text`, read under the registry table `table` names (a
/// request recorded with its handle's `recorded_table`, `nativeReopenEmulation.ts`), as
/// `stored_custody` restores a run recorded before RT-A4.
pub(crate) fn restored(registry: &SolveJobRegistry, text: &str, table: &Value) -> String {
    use crate::analysis_run_v03_validate::RecordedTable;
    use crate::solve_jobs::{HeldResult, RestoredSource};
    let variant = open_pipe_stress_result_export::recorded_tables::variant_of(table).unwrap_or_else(|| panic!("{table} names no registry variant"));
    let lease = registry.acquire_result_parse(text.len() as u64).unwrap();
    let source = RestoredSource { kind: "saved_project_run", project_id: "replay".into(), recording_id: None, result_sha256: "replay".into(), record_sha256: None };
    registry.restore(registry.reset_epoch(), source, HeldResult::restored(text.to_string(), lease).read_under(Some(RecordedTable::of(variant))), None).unwrap()
}

fn substitute(value: &Value, job_id: &str) -> Value {
    match value {
        Value::String(text) if text == JOB => Value::String(job_id.to_string()),
        Value::Array(items) => Value::Array(items.iter().map(|item| substitute(item, job_id)).collect()),
        Value::Object(map) => Value::Object(map.iter().map(|(key, item)| (key.clone(), substitute(item, job_id))).collect()),
        other => other.clone(),
    }
}

/// The reply the webview receives for one request.
pub(crate) fn reply(registry: &SolveJobRegistry, command: &str, args: &Value) -> Value {
    let body = InvokeBody::Raw(serde_json::to_vec(args).unwrap());
    let headers = headers(Some(INGRESS_ARGS_JSON_V1));
    let bytes = |outcome: Result<Vec<u8>, CommandError<String>>| match outcome {
        Ok(bytes) => json!({"ok": true, "body": "raw", "base64": crate::result_summary::tests::base64(&bytes)}),
        Err(error) => json!({"ok": false, "error": serde_json::to_value(&error).unwrap()}),
    };
    let text = |outcome: Result<String, CommandError<String>>| match outcome {
        Ok(text) => {
            let mut value: Value = serde_json::from_str(&text).unwrap();
            if command == native_verdict::COMMAND && args["operation"] == "route_header" {
                assert_eq!(value["job_id"], args["jobId"], "the real reply must name its requested held job");
                // Only the launch-local handle is substituted for TypeScript's
                // replay job. Metadata/refusals remain the actual Rust reply.
                value["job_id"] = json!(JOB);
            }
            json!({"ok": true, "body": "json", "value": value})
        },
        Err(error) => json!({"ok": false, "error": serde_json::to_value(&error).unwrap()}),
    };
    let r = registry.clone();
    match command {
        native_verdict::COMMAND => text(native_verdict::reserve(registry, &body, &headers).and_then(|(raw, lease)| native_verdict::run(r, raw, lease))),
        result_summary::COMMAND => bytes(result_summary::reserve(registry, &body, &headers).and_then(|(raw, lease)| result_summary::run(r, raw, lease))),
        analysis_run_v03_build::COMMAND => bytes(analysis_run_v03_build::reserve(registry, &body, &headers).and_then(|(raw, lease)| analysis_run_v03_build::run(r, raw, lease))),
        result_detail::ROWS_COMMAND => bytes(result_detail::reserve(registry, &body, &headers, result_detail::ROWS_COMMAND).and_then(|(raw, lease)| result_detail::run_rows(r, raw, lease))),
        result_detail::LOOKUP_COMMAND => bytes(result_detail::reserve(registry, &body, &headers, result_detail::LOOKUP_COMMAND).and_then(|(raw, lease)| result_detail::run_lookup(r, raw, lease))),
        result_detail::NAVIGATOR_COMMAND => bytes(result_detail::reserve(registry, &body, &headers, result_detail::NAVIGATOR_COMMAND).and_then(|(raw, lease)| result_detail::run_navigator(r, raw, lease))),
        result_detail::RECOVERY_COMMAND => bytes(result_detail::reserve(registry, &body, &headers, result_detail::RECOVERY_COMMAND).and_then(|(raw, lease)| result_detail::run_recovery(r, raw, lease))),
        // Tauri's raw response body (`semantics_response`): the JSON text's bytes.
        result_detail::SEMANTICS_COMMAND => bytes(result_detail::reserve(registry, &body, &headers, result_detail::SEMANTICS_COMMAND).and_then(|(raw, lease)| result_detail::run_semantics(r, raw, lease)).map(String::into_bytes)),
        other => panic!("not a native result command: {other}"),
    }
}

/// The bytes a request's job holds: a committed fixture file by path, or a text kept once in the
/// document's `sources` by its digest (a source no fixture file is, such as a re-serialized result).
fn source_text(request: &Value, sources: &Value) -> String {
    match (request["source_path"].as_str(), request["source_sha256"].as_str()) {
        (Some(path), _) => std::fs::read_to_string(root().join(path)).unwrap_or_else(|e| panic!("{path}: {e}")),
        (None, Some(sha)) => sources[sha].as_str().unwrap_or_else(|| panic!("source {sha} is neither a path nor a kept text")).to_string(),
        _ => panic!("a request names its source by path or digest"),
    }
}

#[test]
fn every_recorded_webview_request_is_answered_by_its_actual_command() {
    let requests: Value = serde_json::from_str(&std::fs::read_to_string(root().join(REQUESTS)).unwrap()).unwrap();
    // Grouped by the held bytes and, for a restored handle, the table it holds (RT-A4 §10.4).
    let mut by_source: std::collections::BTreeMap<(String, String), (String, Vec<&Value>)> = Default::default();
    for request in requests["requests"].as_array().unwrap() {
        let text = source_text(request, &requests["sources"]);
        let sha: String = Sha256::digest(text.as_bytes()).iter().map(|byte| format!("{byte:02x}")).collect();
        assert_eq!(request["source_sha256"].as_str(), Some(sha.as_str()), "the recorded source digest is the source's");
        let table = request.get("recorded_table").map(Value::to_string).unwrap_or_default();
        by_source.entry((sha, table)).or_insert_with(|| (text, Vec::new())).1.push(request);
    }
    let mut outcomes = Vec::new();
    for ((sha, table), (text, list)) in &by_source {
        let (registry, gate) = registry();
        let job_id = if table.is_empty() { held(&registry, &gate, text) } else { restored(&registry, text, &serde_json::from_str(table).unwrap()) };
        for request in list {
            let command = request["command"].as_str().unwrap();
            let args = substitute(&request["args"], &job_id);
            outcomes.push(json!({"key": request["key"], "source_sha256": sha, "command": command, "reply": reply(&registry, command, &args)}));
        }
        // The held result is untouched by every reply and released once.
        assert!(registry.dispose_result(&job_id).unwrap().released);
        assert_eq!(registry.pool().reserved_total(), 0, "every lease ended with its reply and the disposal");
    }
    outcomes.sort_by(|a, b| a["key"].as_str().cmp(&b["key"].as_str()));
    let document = json!({"document_kind": "swbpipe.desktop.native_replay_outcomes", "note": "Written by native_replay_tests.rs from the actual commands; NATIVE_REPLAY_OUTCOMES_WRITE=1 rewrites it.", "outcomes": outcomes});
    let written = format!("{}\n", serde_json::to_string_pretty(&document).unwrap());
    let path = root().join(OUTCOMES);
    if std::env::var_os("NATIVE_REPLAY_OUTCOMES_WRITE").is_some() {
        std::fs::write(&path, &written).unwrap();
    }
    assert_eq!(std::fs::read_to_string(&path).unwrap_or_default(), written, "the committed replies are the commands' replies (NATIVE_REPLAY_OUTCOMES_WRITE=1 rewrites them)");
}
