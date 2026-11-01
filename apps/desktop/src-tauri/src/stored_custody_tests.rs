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
