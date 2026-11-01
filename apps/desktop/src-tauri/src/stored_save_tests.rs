//! C1 slice 6's save continuity (`stored_save`): a project's saved run,
//! reopened into native custody, re-saved from its restored handle.
//!
//! - **The legacy session's rows, byte for byte.** The oracle is the
//!   unchanged submitted path (`save_project_in_store` /
//!   `create_project_in_store`) given what the legacy session sent: the
//!   stored texts `JSON.parse`d and `JSON.stringify`d (`result_js_text`'s
//!   `JSON.stringify` writer, matched to the TypeScript's own texts in
//!   `persistence_forms.json`), read by serde_json; the historical model hash
//!   evidence; and `computeProjectEnvelopeHash` over them (an independent
//!   RFC 8785 writer, `computed_project_envelope_hash`), or the stored
//!   evidence when that payload hash is the historical one. Every
//!   persistence case saved natively, reset, reopened and re-saved (Save in
//!   place unchanged and edited, Create over it with the confirmed revision);
//!   stored texts in another form (JavaScript's member order and negative
//!   zeros, as Record Run writes a result; integral floats, unsorted keys,
//!   escapes, non-ASCII); and every saved case of the TypeScript reopen
//!   oracle (0.7/0.8-era records, unknown bases, broken checksums), whose
//!   findings stay the TypeScript's after the re-save.
//! - **Limits, guards and lifetimes**: SQLite's exact row boundary (saved,
//!   the model-only save, the no-save refusal), the withheld-run guard, the
//!   old 64 MiB display seam lowered, binding refusals with nothing written,
//!   the pool's refusal of a parse, a disposal or reset during a pending
//!   save, concurrent saves, and the actual command off the async executor.
use super::*;
pub(crate) use crate::native_verdict::tests::gate as command_gate;
use crate::command_ingress::test_support::headers;
use crate::command_ingress::INGRESS_ARGS_JSON_V1;
use crate::memory_pool::test_host::ScriptedHost;
use crate::native_persistence::tests::{body, cases as persistence_cases, dump, frozen_seconds, held, native_save, same_rows, scratch, stored_row_record, text_of, unlimited, Case};
use crate::native_verdict::tests::gated;
use crate::result_js_text::{self, NegativeZero};
use crate::solve_jobs::ResultCustody;
use crate::stored_custody::tests::{builder_cases, native_open, native_read, reopen_cases, saved_run};
use rusqlite::Connection;
use serde_json::json;
use std::path::Path;

const SKIP_TIMES: [&str; 2] = ["created_at_unix", "updated_at_unix"];

/// The native re-save's two steps in this thread; its reply as the webview reads it.
pub(crate) fn native_resave(registry: &SolveJobRegistry, path: &Path, args: &Value, limits: &SaveLimits) -> Result<Value, String> {
    let (raw, lease) = persistence::reserve(registry, &body(args), &headers(Some(INGRESS_ARGS_JSON_V1)), COMMAND).map_err(text_of)?;
    let path = path.to_path_buf();
    run_save(registry, move || Ok(path), raw, lease, limits, &frozen_seconds).map(|reply| serde_json::from_str(reply.text()).unwrap()).map_err(text_of)
}

/// The session's project fields after the open (`opened`'s own, as the session adopts them).
fn opened_fields(opened: &Value) -> Value {
    json!({
        "model": opened["model"],
        "editorIntents": if opened["editor_intents"].is_null() { json!([]) } else { opened["editor_intents"].clone() },
        "proposal": opened["proposal"],
        "selectedReviewTarget": opened["selected_review_target"],
    })
}

/// The re-save request the session sends for the reopened project `opened`
/// with its fields `fields` (`opened_fields`, edited or not).
pub(crate) fn resave_args(opened: &Value, fields: &Value, mode: &str, expected_base: Value, confirmed: Value) -> Value {
    let run = &opened["stored_run"];
    let model = &fields["model"];
    json!({
        "jobId": run["handle"],
        "expectedSource": run["source"],
        "envelopeBasis": {"payload_sha256": run["findings"]["envelope_payload_hash"], "evidence": opened["project_envelope_hash"]},
        "mode": mode,
        "projectId": if mode == "save" { model["project"]["id"].clone() } else { Value::Null },
        "projectName": if mode == "save" { json!(model["project"]["name"].as_str().map_or_else(|| crate::project_name_from_model(model), str::to_string)) } else { Value::Null },
        "model": model,
        "editorIntents": fields["editorIntents"],
        "proposal": fields["proposal"],
        "selectedReviewTarget": fields["selectedReviewTarget"],
        "confirmedRevision": confirmed,
        "expectedBase": expected_base,
    })
}

/// `JSON.parse` then `JSON.stringify` of a stored text, read by serde_json:
/// what the legacy session's request carried for it.
fn as_legacy_sent(text: &str) -> Value {
    let mut out = Vec::new();
    result_js_text::write(text, NegativeZero::AsZero, &mut out).unwrap();
    serde_json::from_slice(&out).unwrap()
}

/// The legacy session's re-save of `args` through the unchanged submitted
/// path on the store at `path` (module docs).
fn legacy_resave(path: &Path, stored_result: &str, stored_record: &str, opened: &Value, args: &Value) -> Result<crate::LocalProjectEnvelope, String> {
    let result = as_legacy_sent(stored_result);
    let record = as_legacy_sent(stored_record);
    let model_hash = as_legacy_sent(&serde_json::to_string(&opened["model_hash"]).unwrap());
    let model = args["model"].clone();
    let project_id = crate::project_id_from_model(&model);
    let computed = crate::computed_project_envelope_hash(&project_id, &model, &args["editorIntents"], &args["proposal"], &args["selectedReviewTarget"], &result, &record, &model_hash);
    let envelope = if computed["value"] == opened["stored_run"]["findings"]["envelope_payload_hash"] {
        as_legacy_sent(&serde_json::to_string(&opened["project_envelope_hash"]).unwrap())
    } else {
        computed
    };
    let confirmed = args["confirmedRevision"].as_str().map(str::to_string);
    if args["mode"] == "create" {
        crate::create_project_in_store(path.to_path_buf(), model, Some(args["editorIntents"].clone()), Some(args["proposal"].clone()), Some(args["selectedReviewTarget"].clone()), Some(result), Some(record), Some(model_hash), Some(envelope), confirmed)
    } else {
        crate::save_project_in_store(
            path.to_path_buf(),
            crate::SaveLocalProjectRequest {
                project_id: args["projectId"].as_str().unwrap().to_string(),
                project_name: args["projectName"].as_str().unwrap().to_string(),
                model,
                editor_intents: args["editorIntents"].clone(),
                proposal: args["proposal"].clone(),
                selected_review_target: args["selectedReviewTarget"].clone(),
                mechanics_result: result,
                analysis_run: record,
                model_hash,
                project_envelope_hash: envelope,
                confirmed_revision: confirmed,
                expected_base: args["expectedBase"].as_str().map(str::to_string),
            },
        )
    }
}

fn column(path: &Path, sql: &str) -> String {
    Connection::open(path).unwrap().query_row(sql, [], |row| row.get(0)).unwrap()
}

fn stored_texts(path: &Path, project_id: &str) -> (String, String) {
    let connection = Connection::open(path).unwrap();
    connection
        .query_row("SELECT mechanics_result_json, analysis_run_json FROM local_projects WHERE project_id = ?1", [project_id], |row| Ok((row.get(0)?, row.get(1)?)))
        .unwrap()
}

fn file_digest(path: &Path) -> String {
    hex(&sha256_in_place(&std::fs::read(path).unwrap()))
}

fn revision(path: &Path, project_id: &str) -> String {
    let (connection, _) = crate::open_project_store(path).unwrap();
    let entry = crate::stored_project_entry(&connection, project_id).unwrap().unwrap();
    crate::stored_revision(&connection, &entry).unwrap()
}

/// Re-saves `args` natively on `path` and through the legacy oracle on a copy
/// of it made first; the two stores' project and search rows must be equal
/// (but the save times), the reply must report what was stored, and its
/// findings must be what the native reopen of the stored project gives.
fn resave_matches_legacy(registry: &SolveJobRegistry, path: &Path, opened: &Value, args: &Value, id: &str) -> Value {
    let project_id = crate::project_id_from_model(&args["model"]);
    // The legacy session's copy was of the texts the handle holds.
    let handle = args["jobId"].as_str().unwrap();
    let (stored_result, stored_record) = (registry.held_result(handle).unwrap().text().to_string(), registry.restored_record(handle).unwrap().text().to_string());
    let legacy_path = path.with_extension(format!("legacy-{}.sqlite", args["mode"].as_str().unwrap()));
    std::fs::copy(path, &legacy_path).unwrap();
    let legacy = legacy_resave(&legacy_path, &stored_result, &stored_record, opened, args);
    let native = native_resave(registry, path, args, &unlimited());
    let (legacy, reply) = match (legacy, native) {
        (Ok(legacy), Ok(reply)) => (legacy, reply),
        (Err(legacy), Err(native)) => {
            assert_eq!(native, legacy, "{id}: the same refusal");
            std::fs::remove_file(&legacy_path).unwrap();
            return Value::Null;
        }
        (legacy, native) => panic!("{id}: legacy {:?}, native {:?}", legacy.map(|_| "saved"), native),
    };
    same_rows(&legacy_path, path, "local_projects", &SKIP_TIMES, id);
    same_rows(&legacy_path, path, "local_project_fts", &[], id);
    let legacy_json = serde_json::to_value(&legacy).unwrap();
    assert_eq!(reply["project_envelope_hash"], legacy_json["project_envelope_hash"], "{id}: the envelope hash stored");
    assert_eq!(reply["model_hash"], legacy_json["model_hash"], "{id}: the historical model hash evidence");
    let mut summary = reply["summary"].clone();
    let mut expected = legacy_json["summary"].clone();
    for value in [&mut summary, &mut expected] {
        value.as_object_mut().unwrap().remove("database_path");
    }
    assert_eq!(summary, expected, "{id}: the summary");
    let (written_result, written_record) = stored_texts(path, &project_id);
    assert_eq!(reply["result"], json!({"outcome": "saved", "run_id": legacy.summary.persisted_analysis_run_ref, "result_bytes": written_result.len(), "record_bytes": written_record.len()}), "{id}");
    // Nothing row-scale comes back.
    for absent in ["model", "mechanics_result", "analysis_run", "editor_intents"] {
        assert!(reply.get(absent).is_none(), "{id}: the reply carries no {absent}");
    }
    assert!(serde_json::to_string(&reply).unwrap().len() < 64 * 1024, "{id}: the reply is small");
    let restored = &reply["restored"];
    assert_eq!(restored["handle"], opened["stored_run"]["handle"], "{id}: the handle stays held");
    assert_eq!(restored["source"], opened["stored_run"]["source"], "{id}: its stored source is unchanged");
    assert_eq!(restored["origin"], "restored_saved_run");
    assert_eq!(restored["envelope_basis_kept"], json!(reply["project_envelope_hash"] == opened["project_envelope_hash"] && restored["envelope_payload_hash"] == opened["stored_run"]["findings"]["envelope_payload_hash"]), "{id}");
    // The findings are those of the project as stored, as its native reopen reads it.
    let reopened = native_open(registry, path, Some(&project_id)).unwrap();
    assert_eq!(restored["findings"], reopened["stored_run"]["findings"], "{id}: the reply's findings are the stored project's");
    assert_eq!(restored["envelope_payload_hash"], reopened["stored_run"]["findings"]["envelope_payload_hash"], "{id}");
    assert!(registry.dispose_result(reopened["stored_run"]["handle"].as_str().unwrap()).unwrap().released);
    std::fs::remove_file(&legacy_path).unwrap();
    reply
}

/// The handle still holds exactly the texts it was reopened with.
fn still_held(registry: &SolveJobRegistry, handle: &str, result: &str, record: &str) {
    assert_eq!(registry.held_result(handle).unwrap().text(), result, "the held result is never rewritten");
    assert_eq!(registry.restored_record(handle).unwrap().text(), record, "the held record is never rewritten");
}

/// Every persistence case: the native Save, a reset, the native reopen, then
/// the re-save from the restored handle (unchanged, edited, Create over it,
/// and again from the reply's basis) stores the legacy session's rows.
#[test]
fn a_reopened_saved_run_is_resaved_as_the_legacy_session_stored_it() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("resave");
    for case in &persistence_cases() {
        let name = case.id.replace(|c: char| !c.is_ascii_alphanumeric(), "-");
        let path = dir.join(format!("{name}.sqlite"));
        let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
        native_save(&registry, &path, &case.save_args(&job, "create", Value::Null, Value::Null), &unlimited()).unwrap_or_else(|e| panic!("{}: {e}", case.id));
        registry.dispose_all_results();
        assert_eq!(registry.pool().reserved_total(), 0);
        let project_id = case.project_id().to_string();
        let opened = native_open(&registry, &path, Some(&project_id)).unwrap();
        let handle = opened["stored_run"]["handle"].as_str().unwrap().to_string();
        let (original_result, original_record) = stored_texts(&path, &project_id);
        let held_bytes = registry.pool().reserved_total();
        // Unchanged: the stored envelope evidence is kept, the rows are the legacy session's.
        let fields = opened_fields(&opened);
        let base = opened["project_envelope_hash"]["value"].clone();
        let reply = resave_matches_legacy(&registry, &path, &opened, &resave_args(&opened, &fields, "save", base, Value::Null), &format!("{} unchanged", case.id));
        assert_eq!(reply["restored"]["envelope_basis_kept"], true, "{}", case.id);
        still_held(&registry, &handle, &original_result, &original_record);
        assert_eq!(registry.pool().reserved_total(), held_bytes, "{}: only the handle's leases remain", case.id);
        // Edited (the proposal and the model's name): a computed envelope hash, the historical model hash kept.
        let mut edited = fields.clone();
        edited["proposal"] = json!({"proposal_id": "proposal:invented-edit", "summary": "Invented edit.", "operations": []});
        edited["model"]["project"]["name"] = json!("Invented renamed project – é");
        let after_unchanged = reply["project_envelope_hash"]["value"].clone();
        // The session now holds the reply's basis (the reopen's is still established).
        let mut session_view = opened.clone();
        session_view["project_envelope_hash"] = reply["project_envelope_hash"].clone();
        session_view["stored_run"]["findings"] = reply["restored"]["findings"].clone();
        let reply = resave_matches_legacy(&registry, &path, &session_view, &resave_args(&session_view, &edited, "save", after_unchanged, Value::Null), &format!("{} edited", case.id));
        assert_eq!(reply["restored"]["envelope_basis_kept"], false, "{}", case.id);
        assert!(reply["restored"]["findings"]["findings"].as_array().unwrap().contains(&json!("HISTORICAL_MODEL_HASH_MISMATCH")), "{}: the historical model hash no longer matches the edited model", case.id);
        // Create over it, with the user's confirmed Replace.
        session_view["project_envelope_hash"] = reply["project_envelope_hash"].clone();
        session_view["stored_run"]["findings"] = reply["restored"]["findings"].clone();
        let confirmed = json!(revision(&path, &project_id));
        resave_matches_legacy(&registry, &path, &session_view, &resave_args(&session_view, &edited, "create", Value::Null, confirmed), &format!("{} create", case.id));
        // Again from the reply's basis: the same bytes (the stored writer is idempotent).
        let before = dump(&path, "local_projects", &SKIP_TIMES);
        let base = column(&path, "SELECT json_extract(project_envelope_hash_json, '$.value') FROM local_projects");
        native_resave(&registry, &path, &resave_args(&session_view, &edited, "save", json!(base), Value::Null), &unlimited()).unwrap();
        assert_eq!(dump(&path, "local_projects", &SKIP_TIMES), before, "{}: a second re-save stores the same rows", case.id);
        still_held(&registry, &handle, &original_result, &original_record);
        assert!(registry.dispose_result(&handle).unwrap().released);
        assert_eq!(registry.pool().reserved_total(), 0, "{}: every lease ended", case.id);
    }
    std::fs::remove_dir_all(&dir).unwrap();
}

/// A stored run in other forms than the store's own: a result and record in
/// JavaScript's member order with negative zeros kept (as Record Run writes a
/// result), and in a raw form with integral floats, `-0.0`, unsorted keys,
/// escapes, non-ASCII and integers beyond binary64. The re-save stores them
/// as the legacy session did (normalized), never as copied text; the
/// persistence forms' edge texts read the same through the stored writer.
#[test]
fn stored_texts_in_other_forms_are_resaved_as_the_legacy_session_normalized_them() {
    // The TypeScript's own `JSON.stringify` texts of the edge texts, read by serde_json.
    let forms = crate::result_js_text::tests::forms();
    for edge in forms["edges"].as_array().unwrap() {
        let (Ok(value), Ok(sent)) = (serde_json::from_str::<Value>(edge["text"].as_str().unwrap()), serde_json::from_str::<Value>(edge["stringify_text"].as_str().unwrap())) else {
            continue; // JavaScript reads it (an infinity); serde_json does not, so no restored handle holds it.
        };
        assert_eq!(stored_text(&value), serde_json::to_string(&sent).unwrap(), "edge {}", edge["text"]);
    }
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("resave-forms");
    let probe = r#""edge_probe":{"z":[-0,-0.0,1.0,1e21,1e-7,2.5E-3,9007199254740993,123456789012345678901234567890,"é😀 ","é😀"],"b":1,"a":2,"10":3,"2":4},"#;
    for case in persistence_cases().iter().take(3) {
        for form in ["raw", "javascript"] {
            let name = format!("{}-{form}", case.id.replace(|c: char| !c.is_ascii_alphanumeric(), "-"));
            let path = dir.join(format!("{name}.sqlite"));
            let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
            native_save(&registry, &path, &case.save_args(&job, "create", Value::Null, Value::Null), &unlimited()).unwrap();
            registry.dispose_all_results();
            let project_id = case.project_id().to_string();
            let (result, record) = stored_texts(&path, &project_id);
            let inject = |text: &str| format!("{{{probe}{}", &text[1..]);
            let reform = |text: String| match form {
                "raw" => text,
                _ => {
                    let mut out = Vec::new();
                    result_js_text::write(&text, NegativeZero::Keep, &mut out).unwrap();
                    String::from_utf8(out).unwrap()
                }
            };
            let (result, record) = (reform(inject(&result)), reform(inject(&record)));
            assert!(result.contains("-0"), "{name}: a negative zero is stored");
            Connection::open(&path).unwrap().execute("UPDATE local_projects SET mechanics_result_json = ?1, analysis_run_json = ?2", [&result, &record]).unwrap();
            let opened = native_open(&registry, &path, Some(&project_id)).unwrap();
            assert_eq!(opened["stored_run"]["state"], "restored", "{name}");
            let handle = opened["stored_run"]["handle"].as_str().unwrap().to_string();
            let base = opened["project_envelope_hash"]["value"].clone();
            resave_matches_legacy(&registry, &path, &opened, &resave_args(&opened, &opened_fields(&opened), "save", base, Value::Null), &name);
            let (written, _) = stored_texts(&path, &project_id);
            assert_ne!(written, result, "{name}: the stored text is the normalized form, not the copied text");
            // Negative zeros `0`, integral floats integers, JavaScript's binary64 values and number
            // texts, serde_json's member order.
            assert!(written.contains(r#""edge_probe":{"10":3,"2":4,"a":2,"b":1,"z":[0,0,1,1e+21,1e-7,0.0025,9007199254740992,1.2345678901234568e+29,"#), "{name}: {}", &written[..written.len().min(300)]);
            still_held(&registry, &handle, &result, &record);
            assert!(registry.dispose_result(&handle).unwrap().released);
        }
    }
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// Every saved case of the TypeScript reopen oracle (records of every route,
/// 0.7/0.8-era and unknown load bases, broken checksums and references,
/// retired and unsupported record versions) stored, reopened and re-saved
/// unchanged: the legacy session's rows, and the TypeScript's findings still.
#[test]
fn historical_cases_are_resaved_with_their_findings_unchanged() {
    let host = ScriptedHost::normal();
    let (registry, _gate) = gated(&host);
    let dir = scratch("resave-historical");
    let builder = builder_cases();
    let mut resaved = 0;
    for case in reopen_cases().iter().filter(|case| case["kind"] == "saved") {
        let id = case["id"].as_str().unwrap();
        if case["expected"].get("absent").is_some() {
            continue;
        }
        let (result, record) = saved_run(case, &builder);
        let project = &case["project"];
        let path = dir.join(format!("{}.sqlite", id.replace(|c: char| !c.is_ascii_alphanumeric(), "-")));
        let project_id = crate::project_id_from_model(&project["model"]);
        {
            let (mut connection, _) = crate::open_project_store(&path).unwrap();
            crate::upsert_project(
                &mut connection,
                &project_id,
                &crate::project_name_from_model(&project["model"]),
                &project["model"],
                &project["editor_intents"],
                &project["proposal"],
                &project["selected_review_target"],
                &result,
                &record,
                &project["model_hash"],
                &project["project_envelope_hash"],
            )
            .unwrap();
        }
        let opened = match native_open(&registry, &path, Some(&project_id)) {
            Ok(opened) => opened,
            Err(native) => {
                let legacy = crate::open_project_in_store(path.clone(), Some(&project_id)).err();
                assert_eq!(Some(native), legacy, "{id}: refused as the legacy open refuses");
                continue;
            }
        };
        if opened["stored_run"]["state"] != "restored" {
            continue;
        }
        let expected = &case["expected"];
        assert_eq!(opened["stored_run"]["findings"]["findings"], expected["findings"], "{id}: the reopen's findings are the TypeScript's");
        let handle = opened["stored_run"]["handle"].as_str().unwrap().to_string();
        let base = match opened["project_envelope_hash"]["value"].as_str() {
            Some(value) if !value.is_empty() => json!(value),
            _ => Value::Null,
        };
        let confirmed = if base.is_null() { json!(revision(&path, &project_id)) } else { Value::Null };
        let reply = resave_matches_legacy(&registry, &path, &opened, &resave_args(&opened, &opened_fields(&opened), "save", base, confirmed), id);
        if !reply.is_null() {
            assert_eq!(reply["restored"]["findings"]["findings"], expected["findings"], "{id}: the TypeScript's findings after the re-save");
            resaved += 1;
        }
        assert!(registry.dispose_result(&handle).unwrap().released);
    }
    assert!(resaved >= 100, "{resaved} historical cases re-saved");
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// The first persistence case saved natively, the session reset, the project reopened.
fn reopened(registry: &SolveJobRegistry, gate: &crate::solve_worker::test_support::Gate, path: &Path) -> (Case, Value) {
    reopened_case(registry, gate, path, persistence_cases().into_iter().next().unwrap())
}

fn reopened_case(registry: &SolveJobRegistry, gate: &crate::solve_worker::test_support::Gate, path: &Path, case: Case) -> (Case, Value) {
    let job = held(registry, gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
    native_save(registry, path, &case.save_args(&job, "create", Value::Null, Value::Null), &unlimited()).unwrap();
    registry.dispose_all_results();
    let opened = native_open(registry, path, Some(case.project_id())).unwrap();
    assert_eq!(opened["stored_run"]["state"], "restored");
    (case, opened)
}

fn in_place(opened: &Value) -> Value {
    resave_args(opened, &opened_fields(opened), "save", opened["project_envelope_hash"]["value"].clone(), Value::Null)
}

/// Each binding refusal is `NATIVE-PERSISTENCE-CUSTODY` with nothing written:
/// an unknown, a solve's, a recorded run's or a disposed handle; another
/// stored source (kind, project, either digest); an envelope basis the host
/// did not establish. The destination's guard is the save's own.
#[test]
fn a_handle_that_does_not_bind_is_refused_with_nothing_written() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("resave-binding");
    let path = dir.join("store.sqlite");
    let (case, opened) = reopened(&registry, &gate, &path);
    let args = in_place(&opened);
    let before = file_digest(&path);
    let refused = |args: &Value, why: &str| {
        let text = native_resave(&registry, &path, args, &unlimited()).unwrap_err();
        assert!(text.starts_with(CUSTODY) && text.contains("nothing was written"), "{why}: {text}");
        assert_eq!(file_digest(&path), before, "{why}: nothing was written");
        text
    };
    let mut other = args.clone();
    other["jobId"] = json!("restored-stored-result-999");
    assert!(refused(&other, "unknown").contains("not held"));
    // A solve's handle, whatever the request names.
    let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
    other["jobId"] = json!(job);
    assert!(refused(&other, "solve").contains("is a solve"));
    registry.dispose_result(&job).unwrap();
    for (member, value) in [("kind", json!("recorded_run")), ("project_id", json!("project:other")), ("result_sha256", json!("0".repeat(64))), ("record_sha256", json!("f".repeat(64)))] {
        let mut altered = args.clone();
        altered["expectedSource"][member] = value;
        assert!(refused(&altered, member).contains("is not the one"), "{member}");
    }
    let mut altered = args.clone();
    altered["envelopeBasis"]["evidence"]["hash_status"] = json!("invented");
    assert!(refused(&altered, "evidence").contains("envelope basis"));
    let mut altered = args.clone();
    altered["envelopeBasis"]["payload_sha256"] = json!(format!("sha256:{}", "1".repeat(64)));
    assert!(refused(&altered, "payload").contains("envelope basis"));
    // No model hash or envelope hash the request names is used.
    let mut named = args.clone();
    named["modelHash"] = json!({"value": "sha256:invented"});
    named["projectEnvelopeHash"] = json!({"value": "sha256:invented"});
    let landed = native_resave(&registry, &path, &named, &unlimited()).unwrap();
    assert_eq!(landed["model_hash"], opened["model_hash"]);
    assert_eq!(landed["project_envelope_hash"], opened["project_envelope_hash"]);
    // The destination's guard: a moved base, no base, a stale confirmation.
    let before = file_digest(&path);
    let mut moved = args.clone();
    moved["expectedBase"] = json!("sha256:invented-old-base");
    assert!(native_resave(&registry, &path, &moved, &unlimited()).unwrap_err().starts_with("PROJECT-BASE-MOVED"));
    moved["expectedBase"] = Value::Null;
    assert!(native_resave(&registry, &path, &moved, &unlimited()).unwrap_err().starts_with("PROJECT-ID-EXISTS"));
    moved["confirmedRevision"] = json!("invented-revision");
    assert!(native_resave(&registry, &path, &moved, &unlimited()).unwrap_err().starts_with("PROJECT-BASE-MOVED"));
    assert_eq!(file_digest(&path), before);
    // A recorded run's handle: no session path saves a recorded run into a project.
    let base = opened["project_envelope_hash"]["value"].clone();
    let mut request = crate::native_persistence::tests::native_request(None);
    request["expected_base"] = base;
    let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
    let recorded = crate::native_persistence::tests::native_record(&registry, &path, &case.record_args(&job, request), None).unwrap();
    registry.dispose_result(&job).unwrap();
    let read = native_read(&registry, &path, case.project_id(), recorded["run"]["recording_id"].as_str().unwrap()).unwrap();
    let mut from_record = args.clone();
    from_record["jobId"] = read["restored"]["handle"].clone();
    from_record["expectedSource"] = read["restored"]["source"].clone();
    let before = file_digest(&path);
    assert!(refused_at(&registry, &path, &from_record).contains("restored recorded run"));
    assert_eq!(file_digest(&path), before);
    // Disposed: refused.
    let handle = opened["stored_run"]["handle"].as_str().unwrap();
    registry.dispose_result(handle).unwrap();
    assert!(refused_at(&registry, &path, &args).contains("no longer held"));
    // The fresh routes still refuse a restored handle (`stored_custody_tests::check_restored`); this
    // route grants it no solve custody.
    let reopened_again = native_open(&registry, &path, Some(case.project_id())).unwrap();
    let again = reopened_again["stored_run"]["handle"].as_str().unwrap();
    assert!(registry.custody_binding(again).is_none());
    let refused_fresh = native_save(&registry, &path, &case.save_args(again, "save", Value::Null, Value::Null), &unlimited()).unwrap_err();
    assert!(refused_fresh.starts_with(CUSTODY) && refused_fresh.contains("restored"), "{refused_fresh}");
    registry.dispose_all_results();
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

fn refused_at(registry: &SolveJobRegistry, path: &Path, args: &Value) -> String {
    let text = native_resave(registry, path, args, &unlimited()).unwrap_err();
    assert!(text.starts_with(CUSTODY), "{text}");
    text
}

/// SQLite's exact boundary on the actual store: at the row's size the run is
/// saved; one byte lower (the result and the record each fit, their row does
/// not) the model-only save, its envelope hash over what was stored and no
/// findings, the handle still held; below the model-only row nothing is saved
/// and the transaction rolls back. A model-only save over a stored run
/// withheld for its size needs the confirmed Replace.
#[test]
fn the_resave_keeps_the_saves_limits_and_guards() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("resave-limits");
    let path = dir.join("store.sqlite");
    let (case, opened) = reopened(&registry, &gate, &path);
    let handle = opened["stored_run"]["handle"].as_str().unwrap().to_string();
    let project_id = case.project_id().to_string();
    let reference = dir.join("reference.sqlite");
    std::fs::copy(&path, &reference).unwrap();
    native_resave(&registry, &reference, &in_place(&opened), &unlimited()).unwrap();
    let row_with = stored_row_record(&reference, "local_projects");
    let at = |limit: u64, withheld_bytes: u64, args: &Value| {
        let target = dir.join(format!("at-{limit}-{withheld_bytes}.sqlite"));
        std::fs::copy(&path, &target).unwrap();
        let outcome = native_resave(&registry, &target, args, &SaveLimits { sqlite_length: Some(limit as i32), withheld_bytes });
        (target, outcome)
    };
    let display = crate::solve_jobs::MAX_DISPLAYABLE_RESULT_BYTES;
    let (target, saved) = at(row_with, display, &in_place(&opened));
    assert_eq!(saved.unwrap()["result"]["outcome"], "saved", "SQLite writes the row at its computed size");
    same_rows(&reference, &target, "local_projects", &SKIP_TIMES, "at the row's size");
    let (target, fallback) = at(row_with - 1, display, &in_place(&opened));
    let fallback = fallback.unwrap();
    let result = &fallback["result"];
    assert_eq!(result["outcome"], "not_saved", "{fallback}");
    assert_eq!(result["code"], crate::native_persistence::PROJECT_SAVE_RESULT_TOO_LARGE);
    assert_eq!(result["refused"]["what"], "project row");
    assert!(result["result_bytes"].as_u64().unwrap() < row_with - 1 && result["record_bytes"].as_u64().unwrap() < row_with - 1, "each fits alone");
    assert!(fallback["restored"]["findings"].is_null() && fallback["restored"]["handle"] == json!(handle), "{fallback}");
    let model_hash = &opened["model_hash"];
    let fields = opened_fields(&opened);
    let expected = crate::computed_project_envelope_hash(&project_id, &fields["model"], &fields["editorIntents"], &fields["proposal"], &fields["selectedReviewTarget"], &Value::Null, &Value::Null, model_hash);
    assert_eq!(fallback["project_envelope_hash"], expected, "the envelope hash of what was stored");
    assert_eq!(column(&target, "SELECT mechanics_result_json FROM local_projects"), "null");
    assert_eq!(column(&target, "SELECT analysis_run_json FROM local_projects"), "null");
    let row_without = stored_row_record(&target, "local_projects");
    let (target, refused) = at(row_without - 1, display, &in_place(&opened));
    let text = refused.unwrap_err();
    assert!(text.starts_with(crate::native_persistence::PROJECT_SAVE_TOO_LARGE) && text.contains("even without the result"), "{text}");
    assert_eq!(file_digest(&target), file_digest(&path), "nothing was saved: no row, no search row");
    // The withheld-run guard: the stored run above a lowered withholding size is kept unless confirmed.
    let (target, refused) = at(row_with - 1, 100, &in_place(&opened));
    assert!(refused.unwrap_err().starts_with("STORED-RESULT-TOO-LARGE: Not saved."));
    assert_eq!(file_digest(&target), file_digest(&path), "the withheld run is kept");
    let confirmed = resave_args(&opened, &fields, "save", Value::Null, json!(revision(&path, &project_id)));
    let (_, replaced) = at(row_with - 1, 100, &confirmed);
    assert_eq!(replaced.unwrap()["result"]["outcome"], "not_saved", "the confirmed Replace saves without the run");
    // The handle is untouched by all of it; a model-only save establishes no basis.
    // An unchanged re-save keeps the reopen's basis (nothing new); a model-only save establishes none.
    assert_eq!(registry.restored_basis(&handle).unwrap().envelopes.len(), 1);
    assert!(registry.dispose_result(&handle).unwrap().released);
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// The old 64 MiB display limit lowered: the C2 copy the legacy re-save took
/// is refused, the native restore and re-save complete with the exact bytes.
#[test]
fn a_run_above_the_lowered_display_limit_is_resaved_without_a_webview_copy() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("resave-display");
    let path = dir.join("store.sqlite");
    let (case, opened) = reopened(&registry, &gate, &path);
    let handle = opened["stored_run"]["handle"].as_str().unwrap().to_string();
    let lowered = 16;
    assert!(opened["stored_run"]["result_bytes"].as_u64().unwrap() > lowered && opened["stored_run"]["record_bytes"].as_u64().unwrap() > lowered);
    let copy = registry.copy_result_within(&handle, lowered).unwrap_err();
    assert!(format!("{copy:?}").contains(crate::solve_jobs::RESULT_DETAIL_WEBVIEW_LIMIT_CODE), "the legacy re-save's copy is refused: {copy:?}");
    assert!(registry.copy_restored_record_within(&handle, lowered).is_err());
    let (result, record) = stored_texts(&path, case.project_id());
    let reply = resave_matches_legacy(&registry, &path, &opened, &in_place(&opened), "lowered display limit");
    assert_eq!(reply["result"]["outcome"], "saved");
    // The stored form of a store-written run is itself: the exact bytes.
    assert_eq!(stored_texts(&path, case.project_id()), (result, record));
    // A lowered withholding size does not hold back a save that carries the run.
    let again = native_resave(&registry, &path, &resave_args(&opened, &opened_fields(&opened), "save", reply["project_envelope_hash"]["value"].clone(), Value::Null), &SaveLimits { sqlite_length: None, withheld_bytes: lowered });
    assert_eq!(again.unwrap()["result"]["outcome"], "saved");
    registry.dispose_all_results();
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// The pool refuses a held text's parse (admitted before it): the pool's
/// typed refusal, nothing written, the handle still held; every lease ends.
#[test]
fn a_pool_refusal_of_the_parse_writes_nothing_and_keeps_the_handle() {
    use crate::memory_pool::PressureLevel;
    const GIB: u64 = 1 << 30;
    let host = ScriptedHost::with(Some(256 * GIB), Some(1024 * GIB), PressureLevel::Normal);
    let (registry, gate) = gated(&host);
    registry.configure_pool(192 * GIB, 64 * GIB, false).unwrap();
    let dir = scratch("resave-pool");
    let path = dir.join("store.sqlite");
    let largest = persistence_cases().into_iter().max_by_key(|case| case.text.len()).unwrap();
    let (case, opened) = reopened_case(&registry, &gate, &path, largest);
    let handle = opened["stored_run"]["handle"].as_str().unwrap().to_string();
    let (result, _) = stored_texts(&path, case.project_id());
    let parse = crate::ingress::prescan(result.as_bytes()).unwrap().materialization_bytes;
    let args = in_place(&opened);
    let arguments = crate::ingress::allocation_envelope(serde_json::to_vec(&args).unwrap().len() as u64).unwrap();
    let budget = (parse - 1).max(arguments * 4);
    assert!(budget < parse, "a budget the arguments fit and the parse does not");
    registry.configure_pool(192 * GIB, budget, false).unwrap();
    let held_bytes = registry.pool().reserved_total();
    let before = file_digest(&path);
    let refused = native_resave(&registry, &path, &args, &unlimited()).unwrap_err();
    assert!(refused.starts_with(crate::stored_read::STORE_READ_MEMORY_UNAVAILABLE) && refused.contains("parsing the stored result"), "the pool's named refusal: {refused}");
    assert_eq!(file_digest(&path), before, "nothing was written");
    assert_eq!(registry.pool().reserved_total(), held_bytes, "only the handle's leases remain");
    assert!(registry.held_result(&handle).is_ok(), "the handle is still held");
    registry.configure_pool(192 * GIB, 64 * GIB, false).unwrap();
    registry.dispose_all_results();
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// A disposal or a session reset while a bound re-save is pending: the save
/// completes from its share of the held texts (as an in-flight legacy save
/// completed), and the texts and their leases are released once, when the
/// share drops. Two concurrent re-saves from one base: one lands, the other
/// is the replacement guard's refusal.
#[test]
fn disposal_reset_and_concurrency_during_a_pending_resave() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("resave-lifecycle");
    for ending in ["dispose", "reset"] {
        let path = dir.join(format!("{ending}.sqlite"));
        let (_, opened) = reopened(&registry, &gate, &path);
        let handle = opened["stored_run"]["handle"].as_str().unwrap().to_string();
        let held = command_gate::hold(&registry, "stored_save:bound", false);
        let worker = {
            let (registry, path, args) = (registry.clone(), path.clone(), in_place(&opened));
            std::thread::spawn(move || native_resave(&registry, &path, &args, &unlimited()))
        };
        assert!(held.0.wait_entered(1, std::time::Duration::from_secs(30)));
        let reserved = registry.pool().reserved_total();
        if ending == "dispose" {
            assert!(registry.dispose_result(&handle).unwrap().released);
        } else {
            registry.dispose_all_results();
        }
        assert_eq!(registry.pool().reserved_total(), reserved, "{ending}: the pending save still shares the texts");
        held.0.open();
        let reply = worker.join().unwrap().unwrap();
        assert_eq!(reply["result"]["outcome"], "saved", "{ending}");
        drop(held);
        assert_eq!(registry.pool().reserved_total(), 0, "{ending}: released once, when the share dropped");
        assert!(registry.held_result(&handle).is_err(), "{ending}: no disposed handle resurrects");
    }
    // Concurrent: one write lock; the second sees the first's row.
    let path = dir.join("concurrent.sqlite");
    let (_, opened) = reopened(&registry, &gate, &path);
    let args = in_place(&opened);
    let workers: Vec<_> = (0..2)
        .map(|_| {
            let (registry, path, args) = (registry.clone(), path.clone(), args.clone());
            std::thread::spawn(move || native_resave(&registry, &path, &args, &unlimited()))
        })
        .collect();
    let outcomes: Vec<_> = workers.into_iter().map(|worker| worker.join().unwrap()).collect();
    // Unchanged fields store the same base again, so both may land; a moved base would refuse.
    assert!(outcomes.iter().all(|outcome| outcome.as_ref().map_or_else(|text| text.starts_with("PROJECT-BASE-MOVED"), |reply| reply["result"]["outcome"] == "saved")), "{outcomes:?}");
    let mut edited = opened_fields(&opened);
    edited["proposal"] = json!({"proposal_id": "proposal:invented-race", "summary": "Invented.", "operations": []});
    let edited_args = resave_args(&opened, &edited, "save", opened["project_envelope_hash"]["value"].clone(), Value::Null);
    let base = column(&path, "SELECT json_extract(project_envelope_hash_json, '$.value') FROM local_projects");
    let mut from_base = edited_args.clone();
    from_base["expectedBase"] = json!(base);
    let workers: Vec<_> = (0..2)
        .map(|index| {
            let (registry, path) = (registry.clone(), path.clone());
            let mut args = from_base.clone();
            args["proposal"]["summary"] = json!(format!("Invented {index}."));
            std::thread::spawn(move || native_resave(&registry, &path, &args, &unlimited()))
        })
        .collect();
    let outcomes: Vec<_> = workers.into_iter().map(|worker| worker.join().unwrap()).collect();
    assert_eq!(outcomes.iter().filter(|outcome| outcome.is_ok()).count(), 1, "{outcomes:?}");
    assert!(outcomes.iter().any(|outcome| outcome.as_ref().is_err_and(|text| text.starts_with("PROJECT-BASE-MOVED"))), "{outcomes:?}");
    registry.dispose_all_results();
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// The actual command's futures, more than the runtime's workers, held at
/// the blocking part while another async task completes.
#[test]
fn the_resave_runs_off_the_async_executor() {
    use crate::native_verdict::tests::runtime_workers;
    use std::time::Duration;
    let host = ScriptedHost::normal();
    let (registry, _) = gated(&host);
    let commands = runtime_workers() * 2 + 2;
    let held = command_gate::hold(&registry, COMMAND, false);
    let tasks: Vec<_> = (0..commands)
        .map(|_| {
            let registry = registry.clone();
            tauri::async_runtime::spawn(async move {
                let args = json!({"jobId": "restored-stored-result-1", "mode": "save"});
                let store = || Err("TEST-RESAVE-STORE-UNAVAILABLE".to_string());
                save_project(&registry, store, &body(&args), &headers(Some(INGRESS_ARGS_JSON_V1))).await
            })
        })
        .collect();
    let all_entered = held.0.wait_entered(commands, Duration::from_secs(30));
    let (sender, receiver) = std::sync::mpsc::channel();
    tauri::async_runtime::spawn(async move {
        let _ = sender.send(());
    });
    let probe = receiver.recv_timeout(Duration::from_secs(5));
    held.0.open();
    for task in tasks {
        assert!(tauri::async_runtime::block_on(task).unwrap().is_err());
    }
    assert!(all_entered, "admission occupied the async executor");
    assert!(probe.is_ok(), "no other async task could run");
    drop(held);
    assert_eq!(registry.pool().reserved_total(), 0);
}
