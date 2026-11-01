//! C1 slice 6: saved and recorded runs reopened into native custody
//! (`stored_custody`, `analysis_run_v03_build::validate`).
//!
//! - **The TypeScript contexts' findings.** Every case of
//!   `apps/desktop/src/test-support/native-reopen/historical_reopen_cases.json`
//!   (written by `historicalReopenCases.ts` from the unchanged
//!   `buildHistoricalRunContext` and `buildRecordedRunContext` over full
//!   rows; `historicalReopenParity.test.ts` regenerates and compares it) must
//!   give the same findings, in order, the same run name, and the same
//!   envelope payload hash or model hash, natively from the stored values.
use super::*;
use crate::analysis_run_v03_build::validate::{projection_checked_sha256, recorded_run_findings, recorded_run_validation, saved_run_findings, saved_run_validation};
use serde_json::{json, Value};

const REOPEN_CASES: &str = "apps/desktop/src/test-support/native-reopen/historical_reopen_cases.json";
const BUILDER_CASES: &str = "apps/desktop/src/test-support/analysis-run-v03/analysis_run_v03_cases.json";

pub(crate) fn root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}
pub(crate) fn read_text(path: &str) -> String {
    std::fs::read_to_string(root().join(path)).unwrap_or_else(|e| panic!("{path}: {e}"))
}

/// A builder case's held text: a file, a file's top-level member, or a text.
pub(crate) fn source_text(source: &Value) -> String {
    if let Some(text) = source["text"].as_str() {
        return text.to_string();
    }
    let text = read_text(source["path"].as_str().unwrap());
    match source["member"].as_str() {
        None => text,
        Some(member) => crate::result_detail::last_members(&text, [member]).unwrap()[0].unwrap().get().to_string(),
    }
}

/// The builder corpus's cases by id.
pub(crate) fn builder_cases() -> std::collections::HashMap<String, Value> {
    let file: Value = serde_json::from_str(&read_text(BUILDER_CASES)).unwrap();
    file["cases"].as_array().unwrap().iter().map(|case| (case["id"].as_str().unwrap().to_string(), case.clone())).collect()
}

pub(crate) fn reopen_cases() -> Vec<Value> {
    serde_json::from_str::<Value>(&read_text(REOPEN_CASES)).unwrap()["cases"].as_array().unwrap().clone()
}

/// `applyPatches` of the generator.
fn apply_patches(result: &mut Value, record: &mut Value, patches: &[Value]) {
    for patch in patches {
        let target = match patch["on"].as_str().unwrap() {
            "result" => &mut *result,
            "record" => &mut *record,
            other => panic!("patch on {other}"),
        };
        let path = patch["path"].as_array().unwrap();
        let value = patch.get("value").cloned().unwrap_or(Value::Null);
        if path.is_empty() {
            *target = value;
            continue;
        }
        let mut at = target;
        for key in &path[..path.len() - 1] {
            at = match key {
                Value::String(key) => at.get_mut(key.as_str()).unwrap(),
                Value::Number(index) => at.get_mut(index.as_u64().unwrap() as usize).unwrap(),
                _ => unreachable!(),
            };
        }
        let last = &path[path.len() - 1];
        match (patch["op"].as_str().unwrap(), last) {
            ("set", Value::String(key)) => {
                at[key.as_str()] = value;
            }
            ("set", Value::Number(index)) => at[index.as_u64().unwrap() as usize] = value,
            ("delete", Value::String(key)) => {
                at.as_object_mut().unwrap().remove(key.as_str());
            }
            ("append", Value::String(key)) => at[key.as_str()].as_array_mut().unwrap().push(value),
            other => panic!("patch {other:?}"),
        }
    }
}

/// A record resealed as the generator reseals it.
fn reseal(record: &mut Value) {
    let value = projection_checked_sha256(record).unwrap();
    for hash in record["analysis_run"]["hashes"].as_array_mut().unwrap() {
        if hash["payload_scope"] == "analysis_run_record" {
            hash["value"] = json!(value);
        }
    }
}

/// A saved case's stored result and record (the values the store parses from what it stored).
pub(crate) fn saved_run(case: &Value, builder: &std::collections::HashMap<String, Value>) -> (Value, Value) {
    let (mut result, mut record) = match case["base"].as_str() {
        Some(base) => {
            let base = &builder[base];
            (serde_json::from_str::<Value>(&source_text(&base["source"])).unwrap(), base["expected"]["record"].clone())
        }
        None => (case["mechanics_result"].clone(), case["analysis_run"].clone()),
    };
    apply_patches(&mut result, &mut record, case.get("patches").and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]));
    if case["reseal"] == true {
        reseal(&mut record);
    }
    (result, record)
}

#[test]
fn the_native_findings_are_the_typescript_contexts() {
    let builder = builder_cases();
    let cases = reopen_cases();
    assert!(cases.len() >= 150, "{} cases", cases.len());
    let mut failures = Vec::new();
    for case in &cases {
        let id = case["id"].as_str().unwrap();
        let expected = &case["expected"];
        let actual = match case["kind"].as_str().unwrap() {
            "saved" => {
                let (mut result, record) = saved_run(case, &builder);
                let project = &case["project"];
                let saved = validate::SavedProject {
                    model: &project["model"],
                    editor_intents: &project["editor_intents"],
                    proposal: &project["proposal"],
                    selected_review_target: &project["selected_review_target"],
                    model_hash: &project["model_hash"],
                    project_envelope_hash: &project["project_envelope_hash"],
                };
                match saved_run_findings(&saved, &mut result, &record) {
                    None => json!({"absent": true}),
                    Some(mut found) => {
                        saved_run_validation(&mut found, &mut result, &record);
                        json!({"findings": found.findings, "run_id": found.run_id, "envelope_payload_hash": found.envelope_payload_hash, "model_hash": null})
                    }
                }
            }
            _ => {
                let read = &case["read"];
                let context = RecordedRead {
                    state_id: read["state"]["state_id"].as_str().unwrap(),
                    state_project_id: read["state"]["project_id"].as_str().unwrap(),
                    state_model_hash: read["state"]["model_hash"].as_str().unwrap(),
                    run_state_id: read["run"]["state_id"].as_str().unwrap(),
                    producer_run_id: read["run"]["producer_run_id"].as_str().unwrap(),
                    run_record_hash: read["run"]["run_record_hash"].as_str().unwrap(),
                };
                let mut result = serde_json::from_str::<Value>(read["result_json"].as_str().unwrap()).ok();
                let manifest = serde_json::from_str::<Value>(read["input_manifest_canonical"].as_str().unwrap()).ok();
                let record = &read["record"];
                let mut found = recorded_run_findings(&context, result.as_mut(), manifest.as_ref(), record);
                if let Some(result) = result.as_mut() {
                    recorded_run_validation(&mut found, &context, result, manifest.as_ref(), record);
                }
                json!({"findings": found.findings, "run_id": found.run_id, "envelope_payload_hash": null, "model_hash": found.model_hash})
            }
        };
        if &actual != expected {
            failures.push(format!("{id}:\n  expected {expected}\n  actual   {actual}"));
        }
    }
    assert!(failures.is_empty(), "{} of {} cases differ:\n{}", failures.len(), cases.len(), failures.join("\n"));
}

// ---------------------------------------------------------------------------
// The connecting path: native Save / Record Run -> SQLite -> native reopen
// ---------------------------------------------------------------------------

use crate::command_ingress::test_support::headers;
use crate::command_ingress::INGRESS_ARGS_JSON_V1;
use crate::memory_pool::test_host::ScriptedHost;
use crate::native_persistence::tests::{cases as persistence_cases, held, native_record, native_request, native_save, scratch, stored_project, unlimited, Case};
use crate::native_verdict::tests::gated;
use crate::solve_jobs::{ResultCustody, ResultOrigin};
use rusqlite::Connection;
use std::path::Path;

fn body(args: &Value) -> InvokeBody {
    InvokeBody::Raw(serde_json::to_vec(args).unwrap())
}

fn text_of(error: CommandError<String>) -> String {
    match error {
        CommandError::Command(text) | CommandError::Response(text) => text,
        CommandError::Ingress(error) => format!("INGRESS: {error:?}"),
    }
}

/// The project open's two steps in this thread; its reply as the webview reads it (a JSON body).
pub(crate) fn native_open(registry: &SolveJobRegistry, path: &Path, project_id: Option<&str>) -> Result<Value, String> {
    let epoch = registry.reset_epoch();
    let (raw, lease) = reserve(registry, &body(&json!({"projectId": project_id})), &headers(Some(INGRESS_ARGS_JSON_V1)), OPEN_COMMAND).map_err(text_of)?;
    let path = path.to_path_buf();
    run_open(registry, move || Ok(path), raw, lease, epoch).map(reply_json).map_err(text_of)
}

/// The recorded run's reopen, two steps in this thread.
pub(crate) fn native_read(registry: &SolveJobRegistry, path: &Path, project_id: &str, recording_id: &str) -> Result<Value, String> {
    let epoch = registry.reset_epoch();
    let (raw, lease) = reserve(registry, &body(&json!({"projectId": project_id, "recordingId": recording_id})), &headers(Some(INGRESS_ARGS_JSON_V1)), RUN_COMMAND).map_err(text_of)?;
    let path = path.to_path_buf();
    run_read(registry, move || Ok(path), raw, lease, epoch).map(reply_json).map_err(text_of)
}

/// A reply's JSON body (`tauri::ipc::Response` holds it as text).
pub(crate) fn reply_json(reply: tauri::ipc::Response) -> Value {
    use tauri::ipc::IpcResponse;
    let tauri::ipc::InvokeResponseBody::Json(text) = reply.body().unwrap() else { panic!("the reply is a JSON body") };
    serde_json::from_str(&text).unwrap()
}

fn column_text(path: &Path, sql: &str) -> String {
    Connection::open(path).unwrap().query_row(sql, [], |row| row.get(0)).unwrap()
}

fn file_digest(path: &Path) -> String {
    hex(&sha256_in_place(&std::fs::read(path).unwrap()))
}

/// The detail rows command on a handle: the unfiltered first page's total.
fn rows_total(registry: &SolveJobRegistry, handle: &str) -> u64 {
    let args = json!({"jobId": handle, "filter": {}, "offset": 0, "limit": 1});
    let (raw, lease) = crate::result_detail::reserve(registry, &body(&args), &headers(Some(INGRESS_ARGS_JSON_V1)), crate::result_detail::ROWS_COMMAND).unwrap();
    let bytes = crate::result_detail::run_rows(registry.clone(), raw, lease).unwrap();
    assert_eq!(&bytes[..8], crate::result_detail::MAGIC);
    let length = u64::from_le_bytes(bytes[8..16].try_into().unwrap()) as usize;
    let header: Value = serde_json::from_slice(&bytes[16..16 + length]).unwrap();
    header["total"].as_u64().unwrap()
}

/// S on a handle with no model and no standing, as the historical panel reads it.
fn summary_rows(registry: &SolveJobRegistry, handle: &str) -> u64 {
    let args = json!({"jobId": handle, "modelJson": "null", "standing": null});
    let (raw, lease) = crate::result_summary::reserve(registry, &body(&args), &headers(Some(INGRESS_ARGS_JSON_V1))).unwrap();
    let bytes = crate::result_summary::run(registry.clone(), raw, lease).unwrap();
    let length = u64::from_le_bytes(bytes[8..16].try_into().unwrap()) as usize;
    let header: Value = serde_json::from_slice(&bytes[16..16 + length]).unwrap();
    header["row_count"].as_u64().unwrap()
}

/// What every restored handle must hold: the stored texts exactly, its
/// stored origin, no solve digests, readable rows; and every fresh
/// persistence route refuses it.
fn check_restored(registry: &SolveJobRegistry, case: &Case, path: &Path, handle: &str, result_sql: &str, record_sql: &str) {
    let stored_result = column_text(path, result_sql);
    let stored_record = column_text(path, record_sql);
    assert!(handle.starts_with(crate::solve_jobs::RESTORED_HANDLE_PREFIX), "{handle}");
    assert_eq!(registry.held_result(handle).unwrap().text(), stored_result, "{}: the held bytes are the stored result", case.id);
    assert_eq!(registry.restored_record(handle).unwrap().text(), stored_record, "{}: the held record is the stored record", case.id);
    assert!(registry.custody_binding(handle).is_none(), "a restored result has no solve custody");
    match registry.result_origin(handle) {
        Some(ResultOrigin::Restored(source)) => {
            assert_eq!(source.result_sha256, hex(&sha256_in_place(stored_result.as_bytes())));
            assert_eq!(source.record_sha256.as_deref(), Some(hex(&sha256_in_place(stored_record.as_bytes()))).as_deref());
        }
        other => panic!("{other:?}"),
    }
    let rows = serde_json::from_str::<Value>(&stored_result).unwrap()["results"].as_array().map_or(0, Vec::len) as u64;
    assert_eq!(rows_total(registry, handle), rows, "{}: paged rows read the restored bytes", case.id);
    assert_eq!(summary_rows(registry, handle), rows, "{}: S reads the restored bytes", case.id);
    // Fresh persistence refuses it, whatever the request names (the solve's own intake included).
    let before = file_digest(path);
    let refused = native_record(registry, path, &case.record_args(handle, native_request(None)), None).unwrap_err();
    assert!(refused.starts_with(crate::native_persistence::CUSTODY) && refused.contains("restored"), "{refused}");
    let refused = native_save(registry, path, &case.save_args(handle, "save", Value::Null, Value::Null), &unlimited()).unwrap_err();
    assert!(refused.starts_with(crate::native_persistence::CUSTODY) && refused.contains("restored"), "{refused}");
    assert_eq!(file_digest(path), before, "nothing was written");
    // No fresh transient record is previewed over it.
    let args = json!({"jobId": handle, "inputManifest": case.case["input_manifest"], "ruleCheckAggregate": case.case["rule_check_aggregate"], "rulePackExclusions": []});
    let (raw, lease) = crate::analysis_run_v03_build::reserve(registry, &body(&args), &headers(Some(INGRESS_ARGS_JSON_V1))).unwrap();
    let refused = text_of(crate::analysis_run_v03_build::run(registry.clone(), raw, lease).unwrap_err());
    assert!(refused.starts_with(crate::analysis_run_v03_build::PREVIEW_READ_FAILED), "{refused}");
    // The take refuses it; the bounded C2 copy reads it at or under 64 MiB.
    assert!(registry.take_result(handle).is_err());
    assert_eq!(registry.copy_result_within(handle, crate::solve_jobs::MAX_DISPLAYABLE_RESULT_BYTES).unwrap(), stored_result.as_bytes());
    assert!(registry.release_full_copy(handle).unwrap());
    let (copy, _lease) = registry.copy_restored_record_within(handle, crate::solve_jobs::MAX_DISPLAYABLE_RESULT_BYTES).unwrap();
    assert_eq!(copy, stored_record.as_bytes());
}

/// Every persistence case: the native Save stores the run; after the job is
/// gone and the session reset, the native open restores the stored bytes
/// into native custody with the findings the legacy open's values give.
#[test]
fn a_native_save_reopens_into_native_custody_after_a_reset() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("reopen-save");
    for case in &persistence_cases() {
        let name = case.id.replace(|c: char| !c.is_ascii_alphanumeric(), "-");
        let path = dir.join(format!("{name}.sqlite"));
        let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
        let saved = native_save(&registry, &path, &case.save_args(&job, "create", Value::Null, Value::Null), &unlimited()).unwrap_or_else(|e| panic!("{}: {e}", case.id));
        assert_eq!(saved["result"]["outcome"], "saved");
        registry.dispose_all_results();
        assert_eq!(registry.pool().reserved_total(), 0);
        let before = file_digest(&path);
        let project_id = case.model()["project"]["id"].as_str().unwrap();
        let opened = native_open(&registry, &path, Some(project_id)).unwrap_or_else(|e| panic!("{}: {e}", case.id));
        assert_eq!(file_digest(&path), before, "{}: opening wrote nothing", case.id);
        let run = &opened["stored_run"];
        assert_eq!(run["state"], "restored", "{}: {run}", case.id);
        assert_eq!(run["origin"], "restored_saved_run");
        for absent in ["mechanics_result", "analysis_run"] {
            assert!(opened.get(absent).is_none(), "the reply carries no {absent}");
        }
        let text = serde_json::to_string(&opened).unwrap();
        assert!(!text.contains("\"result_refs\":["), "{}: no ResultRef crosses", case.id);
        assert!(run["record_header"]["result_ref_count"].as_u64().is_some());
        let handle = run["handle"].as_str().unwrap();
        check_restored(&registry, case, &path, handle, "SELECT mechanics_result_json FROM local_projects", "SELECT analysis_run_json FROM local_projects");
        // The findings are those of the legacy open's values (whose TypeScript reading is the oracle above).
        let legacy = crate::open_project_in_store(path.clone(), Some(project_id)).unwrap().unwrap();
        let legacy_json = serde_json::to_value(&legacy).unwrap();
        let project = SavedProject {
            model: &legacy_json["model"],
            editor_intents: &legacy_json["editor_intents"],
            proposal: &legacy_json["proposal"],
            selected_review_target: &legacy_json["selected_review_target"],
            model_hash: &legacy_json["model_hash"],
            project_envelope_hash: &legacy_json["project_envelope_hash"],
        };
        let (mut result, record) = (legacy_json["mechanics_result"].clone(), legacy_json["analysis_run"].clone());
        let mut expected = saved_run_findings(&project, &mut result, &record).unwrap();
        saved_run_validation(&mut expected, &mut result, &record);
        assert_eq!(run["findings"], serde_json::to_value(&expected).unwrap(), "{}", case.id);
        assert!(!expected.findings.iter().any(|finding| finding.contains("MISMATCH")), "{}: a fresh save reopens clean: {:?}", case.id, expected.findings);
        for key in ["summary", "model", "editor_intents", "proposal", "selected_review_target", "model_hash", "project_envelope_hash", "model_document_version"] {
            assert_eq!(opened[key], legacy_json[key], "{}: {key} as open_local_project returns it", case.id);
        }
        assert!(registry.dispose_result(handle).unwrap().released);
        assert!(registry.dispose_result(handle).map(|disposal| !disposal.released).unwrap_or(true), "released once");
        assert!(registry.held_result(handle).is_err(), "no disposed handle resurrects");
    }
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0, "every lease ended");
}

/// Every persistence case: the native Record Run stores the run; the native
/// read restores it after every integrity check of `read_analysis_run`.
#[test]
fn a_native_record_run_reopens_into_native_custody_after_a_reset() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("reopen-record");
    for case in &persistence_cases() {
        let name = case.id.replace(|c: char| !c.is_ascii_alphanumeric(), "-");
        let path = dir.join(format!("{name}.sqlite"));
        stored_project(&path, case.project_id(), case.model());
        let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
        let outcome = native_record(&registry, &path, &case.record_args(&job, native_request(None)), None).unwrap_or_else(|e| panic!("{}: {e}", case.id));
        let recording_id = outcome["run"]["recording_id"].as_str().unwrap().to_string();
        registry.dispose_all_results();
        let before = file_digest(&path);
        let read = native_read(&registry, &path, case.project_id(), &recording_id).unwrap_or_else(|e| panic!("{}: {e}", case.id));
        assert_eq!(file_digest(&path), before, "{}: reading wrote nothing", case.id);
        let restored = &read["restored"];
        assert_eq!(restored["origin"], "restored_recorded_run");
        let legacy = crate::analysis_runs::read_run_in_store(&path, case.project_id(), &recording_id).unwrap();
        assert_eq!(read["run"], serde_json::to_value(&legacy.run).unwrap());
        assert_eq!(read["state"], serde_json::to_value(&legacy.state).unwrap());
        assert_eq!(read["input_manifest_canonical"], json!(legacy.input_manifest_canonical));
        assert!(read.get("result_json").is_none() && read.get("record").is_none());
        let handle = restored["handle"].as_str().unwrap();
        let sql = |column: &str| format!("SELECT {column} FROM analysis_runs WHERE recording_id = '{recording_id}'");
        check_restored(&registry, case, &path, handle, &sql("result_json"), &sql("run_record_json"));
        let context = RecordedRead {
            state_id: &legacy.state.state_id,
            state_project_id: &legacy.state.project_id,
            state_model_hash: &legacy.state.model_hash,
            run_state_id: &legacy.run.state_id,
            producer_run_id: &legacy.run.producer_run_id,
            run_record_hash: &legacy.run.run_record_hash,
        };
        let mut result: Value = serde_json::from_str(&legacy.result_json).unwrap();
        let manifest: Value = serde_json::from_str(&legacy.input_manifest_canonical).unwrap();
        let mut expected = recorded_run_findings(&context, Some(&mut result), Some(&manifest), &legacy.record);
        recorded_run_validation(&mut expected, &context, &mut result, Some(&manifest), &legacy.record);
        assert_eq!(restored["findings"], serde_json::to_value(&expected).unwrap(), "{}", case.id);
        assert!(!expected.findings.iter().any(|finding| finding.contains("MISMATCH")), "{}: {:?}", case.id, expected.findings);
        assert!(registry.dispose_result(handle).unwrap().released);
    }
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// The first persistence case with at least one row, saved and recorded natively on one store.
fn stored_case(registry: &SolveJobRegistry, gate: &crate::solve_worker::test_support::Gate, path: &Path) -> (Case, String) {
    let case = persistence_cases().into_iter().next().unwrap();
    let job = held(registry, gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
    let saved = native_save(registry, path, &case.save_args(&job, "create", Value::Null, Value::Null), &unlimited()).unwrap();
    let base = saved["project_envelope_hash"]["value"].clone();
    let mut request = native_request(None);
    request["expected_base"] = base;
    let outcome = native_record(registry, path, &case.record_args(&job, request), None).unwrap();
    registry.dispose_all_results();
    (case, outcome["run"]["recording_id"].as_str().unwrap().to_string())
}

/// #51's withholding stays on the legacy whole-value reads (lowered seam);
/// a run above the old limit is held natively, by its lengths alone never withheld.
#[test]
fn the_old_display_limit_withholds_only_the_legacy_reads() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("reopen-limit");
    let path = dir.join("store.sqlite");
    let (case, recording_id) = stored_case(&registry, &gate, &path);
    let project_id = case.model()["project"]["id"].as_str().unwrap();
    let connection = Connection::open(&path).unwrap();
    let lowered = 16;
    let legacy = crate::load_project_within(&connection, Some(project_id), lowered).unwrap().unwrap();
    assert!(legacy.withheld.is_some() && legacy.mechanics_result.is_null(), "the legacy open withholds above the lowered limit");
    let refused = crate::analysis_runs::read_run_in_store_within(&path, project_id, &recording_id, &crate::stored_read::UnmeteredReadMemory, lowered).unwrap_err();
    assert!(refused.starts_with(crate::solve_jobs::STORED_RESULT_TOO_LARGE_CODE), "{refused}");
    let opened = native_open(&registry, &path, Some(project_id)).unwrap();
    assert_eq!(opened["stored_run"]["state"], "restored");
    assert!(opened["stored_run"]["result_bytes"].as_u64().unwrap() > lowered);
    let read = native_read(&registry, &path, project_id, &recording_id).unwrap();
    assert!(read["restored"]["result_bytes"].as_u64().unwrap() > lowered);
    registry.dispose_all_results();
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// The pool refuses the held bytes' leases: the project opens with its model
/// editable and its run refused by name, nothing loaded, held or written; the
/// recorded run's read is the pool's named refusal. Every lease ends.
#[test]
fn a_pool_refusal_keeps_the_model_editable_and_the_store_untouched() {
    use crate::memory_pool::PressureLevel;
    const GIB: u64 = 1 << 30;
    let host = ScriptedHost::with(Some(256 * GIB), Some(1024 * GIB), PressureLevel::Normal);
    let (registry, gate) = gated(&host);
    registry.configure_pool(192 * GIB, 64 * GIB, false).unwrap();
    let dir = scratch("reopen-pool");
    let path = dir.join("store.sqlite");
    let (case, recording_id) = stored_case(&registry, &gate, &path);
    let project_id = case.model()["project"]["id"].as_str().unwrap();
    let result_bytes = column_text(&path, "SELECT mechanics_result_json FROM local_projects").len() as u64;
    // A budget the arguments fit and the held result's envelope does not.
    let budget = (crate::ingress::allocation_envelope(result_bytes).unwrap() - 1).max(64 * 1024);
    registry.configure_pool(192 * GIB, budget, false).unwrap();
    let before = file_digest(&path);
    let opened = native_open(&registry, &path, Some(project_id)).unwrap();
    let run = &opened["stored_run"];
    assert_eq!(run["state"], "refused", "{run}");
    assert_eq!(run["code"], RESTORE_REFUSED);
    assert!(run["message"].as_str().unwrap().contains(crate::stored_read::STORE_READ_MEMORY_UNAVAILABLE), "{run}");
    assert_eq!(run["result_bytes"].as_u64().unwrap(), result_bytes);
    assert!(opened["model"].is_object() && opened["model_document_version"].is_object(), "the model opens for editing");
    assert_eq!(registry.pool_status().retained_results, 0, "nothing is held");
    let refused = native_read(&registry, &path, project_id, &recording_id).unwrap_err();
    assert!(refused.contains(crate::stored_read::STORE_READ_MEMORY_UNAVAILABLE), "{refused}");
    assert_eq!(file_digest(&path), before, "the stored evidence is untouched");
    registry.configure_pool(192 * GIB, 64 * GIB, false).unwrap();
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0, "every lease ended");
}

/// A reset during the read refuses the registration and drops what was
/// loaded; full retention refuses by name; a stored result that is not JSON
/// fails the open with the legacy open's own message.
#[test]
fn resets_retention_and_corruption_are_refused_with_nothing_held() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("reopen-refusals");
    let path = dir.join("store.sqlite");
    let (case, _) = stored_case(&registry, &gate, &path);
    let project_id = case.model()["project"]["id"].as_str().unwrap().to_string();
    // A reset after the command began.
    let epoch = registry.reset_epoch();
    registry.dispose_all_results();
    let opened = open_project_in_store(&registry, path.clone(), Some(&project_id), epoch).unwrap().unwrap();
    let opened = serde_json::to_value(&opened).unwrap();
    assert_eq!(opened["stored_run"]["state"], "refused");
    assert!(opened["stored_run"]["message"].as_str().unwrap().contains("RESTORED-RESULT-SESSION-RESET"), "{}", opened["stored_run"]);
    assert_eq!(registry.pool().reserved_total(), 0, "nothing loaded stays held");
    // Retention full: four retained solves.
    let jobs: Vec<String> = (0..crate::memory_pool::MAX_RETAINED_RESULTS).map(|_| crate::native_verdict::tests::retained(&registry, &gate, "{}")).collect();
    let opened = native_open(&registry, &path, Some(&project_id)).unwrap();
    assert_eq!(opened["stored_run"]["state"], "refused");
    assert!(opened["stored_run"]["message"].as_str().unwrap().contains("retention_full"), "{}", opened["stored_run"]);
    for job in &jobs {
        registry.dispose_result(job).unwrap();
    }
    assert_eq!(registry.pool().reserved_total(), 0);
    // Corruption: the legacy open's message, the store untouched.
    Connection::open(&path).unwrap().execute("UPDATE local_projects SET mechanics_result_json = '{\"run_id\": tru'", []).unwrap();
    let before = file_digest(&path);
    let legacy = crate::open_project_in_store(path.clone(), Some(&project_id)).unwrap_err();
    let native = native_open(&registry, &path, Some(&project_id)).unwrap_err();
    assert_eq!(native, legacy);
    assert_eq!(file_digest(&path), before);
    assert_eq!(registry.pool().reserved_total(), 0);
    std::fs::remove_dir_all(&dir).unwrap();
}

/// A disposal while a reader holds a share releases the bytes and both leases
/// only when the share drops; a second disposal releases nothing.
#[test]
fn a_restored_handle_is_released_once_by_its_last_holder() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("reopen-lifetime");
    let path = dir.join("store.sqlite");
    let (case, _) = stored_case(&registry, &gate, &path);
    let opened = native_open(&registry, &path, case.model()["project"]["id"].as_str()).unwrap();
    let handle = opened["stored_run"]["handle"].as_str().unwrap().to_string();
    let held_bytes = registry.pool().reserved_total();
    assert!(held_bytes > 0);
    let share = registry.held_result(&handle).unwrap();
    let record = registry.restored_record(&handle).unwrap();
    let disposal = registry.dispose_result(&handle).unwrap();
    assert!(disposal.released);
    assert_eq!(disposal.released_bytes, share.lease_bytes() + record.lease_bytes());
    assert_eq!(registry.pool().reserved_total(), held_bytes, "the readers still hold the bytes");
    drop(share);
    drop(record);
    assert_eq!(registry.pool().reserved_total(), 0);
    assert!(!registry.dispose_result(&handle).unwrap().released);
    std::fs::remove_dir_all(&dir).unwrap();
}

// ---------------------------------------------------------------------------
// The webview's reopen replies, from the actual commands
// ---------------------------------------------------------------------------

const REOPEN_REPLIES: &str = "apps/desktop/src/test-support/native-reopen/native_reopen_replies.json";
const REOPEN_REQUESTS: &str = "apps/desktop/src/test-support/native-reopen/native_reopen_requests.json";
const PERSISTENCE_REPLIES: &str = "apps/desktop/src/test-support/native-replay/native_persistence_replies.json";
const REPLAY_SOURCE: &str = "fixtures/results/pressure_connected_ui_mechanics_sparse.json";
const REPLAY_MODEL: &str = "fixtures/model_operations/exact_pressure_authoring_model.json";
const HANDLE: &str = "$HANDLE";

fn with_handle(value: &Value, handle: &str) -> Value {
    match value {
        Value::String(text) if text == HANDLE => json!(handle),
        Value::Array(items) => Value::Array(items.iter().map(|item| with_handle(item, handle)).collect()),
        Value::Object(map) => Value::Object(map.iter().map(|(key, item)| (key.clone(), with_handle(item, handle))).collect()),
        other => other.clone(),
    }
}

/// A reply with its restored handle written `$HANDLE` and the store's path `$STORE`.
fn webview_value(mut value: Value, handle: &str) -> Value {
    if let Some(summary) = value.get_mut("summary").and_then(Value::as_object_mut) {
        summary.insert("database_path".into(), json!("$STORE"));
    }
    fn walk(value: &mut Value, handle: &str) {
        match value {
            Value::String(text) if text == handle => *text = HANDLE.to_string(),
            Value::Array(items) => items.iter_mut().for_each(|item| walk(item, handle)),
            Value::Object(map) => map.values_mut().for_each(|item| walk(item, handle)),
            _ => {}
        }
    }
    walk(&mut value, handle);
    value
}

/// The session tests' solve (`nativeMechanicsReplayPair("sparse_interactive", {profile:
/// "pressure"})`) saved and recorded natively with the session's own requests
/// (`native_persistence_replies.json`), the session reset, then reopened: the project open's reply,
/// the recorded run's reply, and each request the webview's reopen sends on the restored handle
/// (`native_reopen_requests.json`, written by `nativeReopen.session.test.tsx` with
/// NATIVE_REOPEN_RECORD=1) answered by its actual command. NATIVE_REOPEN_REPLIES_WRITE=1 rewrites
/// `native_reopen_replies.json`.
#[test]
fn the_webviews_reopen_replies_are_the_commands_replies() {
    use crate::native_persistence::tests::{repo_text, stringify_text, with_job};
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("reopen-replies");
    let path = dir.join("store.sqlite");
    let persistence: Value = serde_json::from_str(&repo_text(PERSISTENCE_REPLIES)).unwrap();
    let scenarios: Vec<&Value> = persistence["scenarios"].as_array().unwrap().iter().filter(|scenario| scenario["store"] == "store").collect();
    let save_args = &scenarios[0]["args"];
    let manifest: Value = serde_json::from_str(save_args["inputManifest"]["canonical_bytes"].as_str().unwrap()).unwrap();
    let intake = stringify_text(&repo_text(REPLAY_MODEL));
    let job = held(&registry, &gate, &repo_text(REPLAY_SOURCE), &intake, manifest["solver_basis"]["solver_mode"].as_str().unwrap(), ResultCustody::Native);
    let saved = native_save(&registry, &path, &with_job(save_args, &job), &unlimited()).unwrap();
    assert_eq!(saved["result"]["outcome"], "saved");
    let recorded = native_record(&registry, &path, &with_job(&scenarios[1]["args"], &job), None).unwrap();
    let recording_id = recorded["run"]["recording_id"].as_str().unwrap().to_string();
    // Close: the session's reset releases the solve's custody; nothing of it remains.
    registry.dispose_all_results();
    assert_eq!(registry.pool().reserved_total(), 0);
    let project_id = save_args["projectId"].as_str().unwrap();
    let opened = native_open(&registry, &path, Some(project_id)).unwrap();
    let handle = opened["stored_run"]["handle"].as_str().unwrap().to_string();
    let requests: Value = serde_json::from_str(&std::fs::read_to_string(root().join(REOPEN_REQUESTS)).unwrap_or_else(|_| "{\"requests\":[]}".into())).unwrap();
    let mut answered = Vec::new();
    for request in requests["requests"].as_array().unwrap() {
        let command = request["command"].as_str().unwrap();
        let reply = crate::native_replay_tests::reply(&registry, command, &with_handle(&request["args"], &handle));
        answered.push(json!({"command": command, "args": request["args"], "reply": reply}));
    }
    assert!(registry.dispose_result(&handle).unwrap().released);
    let read = native_read(&registry, &path, project_id, &recording_id).unwrap();
    let read_handle = read["restored"]["handle"].as_str().unwrap().to_string();
    assert!(registry.dispose_result(&read_handle).unwrap().released);
    let document = json!({
        "document_kind": "swbpipe.desktop.native_reopen_replies",
        "note": "Written by stored_custody_tests.rs from the actual commands on a real store (the session's native Save and Record Run, a session reset, then the reopen; the store's path written $STORE, each restored handle $HANDLE); NATIVE_REOPEN_REPLIES_WRITE=1 rewrites it.",
        "source_path": REPLAY_SOURCE,
        "open": {"command": OPEN_COMMAND, "args": {"projectId": project_id}, "reply": {"ok": true, "body": "json", "value": webview_value(opened, &handle)}},
        "read": {"command": RUN_COMMAND, "args": {"projectId": project_id, "recordingId": recording_id}, "reply": {"ok": true, "body": "json", "value": webview_value(read, &read_handle)}},
        "requests": answered,
    });
    let written = format!("{}\n", serde_json::to_string_pretty(&document).unwrap());
    let target = root().join(REOPEN_REPLIES);
    if std::env::var_os("NATIVE_REOPEN_REPLIES_WRITE").is_some() {
        std::fs::write(&target, &written).unwrap();
    }
    assert!(std::fs::read_to_string(&target).unwrap_or_default() == written, "the committed reopen replies are the commands' replies (NATIVE_REOPEN_REPLIES_WRITE=1 rewrites them)");
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}
