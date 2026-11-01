//! C1 slice 5's persistence: the native Record Run and Save/Create Project
//! (`native_persistence`), against the unchanged submitted paths' actual
//! stores.
//!
//! - **Actual-store byte identity.** For every preview case of the builder's
//!   corpus with a record (`analysis_run_v03_cases.json`: preview-physics,
//!   pressure, pressure-source and retained routes, rule-check aggregates,
//!   rule-pack exclusions), the native command writes exactly the rows the
//!   unchanged submitted path writes for the request the webview sends today
//!   (the TypeScript record, `jsonTextKeepingNegativeZero`'s result text, the
//!   result as serde_json reads `JSON.stringify`'s text): every column of the
//!   run, its labels, the post-solve snapshot and its labels, the project row
//!   (but its two save times) and its search row; the same outcome; and the
//!   session's envelope hash (`persistence_forms.json`, written by the
//!   TypeScript paths). The texts the legacy requests carry are the native
//!   writer's, each first matched to the TypeScript text's digest.
//! - **SQLite's limit.** The record sizes are SQLite's own (it writes the row
//!   at the computed size and refuses it one byte lower); Record Run refuses
//!   at each boundary with the exact lengths and writes nothing, and allocates
//!   no output text; Save Project falls back to the model-only save at the
//!   row's boundary (its envelope hash the TypeScript's for what was stored),
//!   refuses without the result, and never erases a withheld stored run
//!   without the user's confirmed Replace; a row SQLite refuses rolls the whole
//!   transaction back, the snapshot written before it included.
//! - **Binding.** A stale preview, another custody, another mode, a disposed
//!   job, a missing project, a moved base and a missing state are refused,
//!   nothing written; every lease ends with the command.
use super::*;
use crate::analysis_runs::{record_run_in_store, run_labels_from_value, NewStateRequest, RecordRunRequest, RunWriteHooks};
use crate::command_ingress::test_support::headers;
use crate::command_ingress::INGRESS_ARGS_JSON_V1;
use crate::memory_pool::test_host::ScriptedHost;
use crate::native_verdict::tests::gated;
use crate::result_js_text::tests::{forms, preview_cases, source_text};
use crate::solve_jobs::test_support::wait_terminal;
use crate::solve_worker::test_support::Gate;
use rusqlite::Connection;
use std::path::{Path, PathBuf};

const NOW_MILLIS: i64 = 1_793_000_000_000;
const NOW_SECONDS: i64 = 1_793_000_000;

fn frozen_now() -> Result<i64, String> {
    Ok(NOW_MILLIS)
}
fn frozen_token() -> Result<String, String> {
    Ok("0123456789abcdef0123456789abcdef".to_string())
}
fn frozen_seconds() -> Result<i64, String> {
    Ok(NOW_SECONDS)
}
const FROZEN: RunWriteHooks<'static> = RunWriteHooks { now: &frozen_now, state_token: &frozen_token, between_rows: &|_, _| Ok(()) };

/// A fresh directory for this test's stores.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("swbpipe-native-persistence-{name}-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// A native-custody job holding `text`, solved in `mode`.
fn held(registry: &SolveJobRegistry, gate: &Gate, text: &str, mode: &str, custody: ResultCustody) -> String {
    let started = gate.started_count();
    let receipt = registry.start_with_custody(br#"{"project":{"id":"p"}}"#, Some(mode), custody).unwrap();
    gate.wait_started(started + 1);
    gate.release(text);
    let status = wait_terminal(registry, &receipt.job_id);
    assert_eq!(status.state, "completed", "{status:?}");
    receipt.job_id
}

fn claimed(record: &Value, scope: &str) -> String {
    record["analysis_run"]["hashes"].as_array().unwrap().iter().find(|h| h["payload_scope"] == scope).unwrap()["value"].as_str().unwrap().to_string()
}

/// One preview case and what the tests read of it.
struct Case {
    id: String,
    text: String,
    record: Value,
    manifest_text: String,
    manifest: Value,
    mode: String,
    form: Value,
    case: Value,
}

impl Case {
    fn model(&self) -> &Value {
        &self.manifest["model_basis"]["model_payload"]
    }
    fn project_id(&self) -> &str {
        self.manifest["model_basis"]["model_ref"].as_str().unwrap()
    }
    fn project_name(&self) -> String {
        self.model()["project"]["name"].as_str().map_or_else(|| crate::project_name_from_model(self.model()), str::to_string)
    }
    fn expected(&self) -> Value {
        json!({"run_id": self.record["analysis_run"]["run_id"], "record_sha256": claimed(&self.record, "analysis_run_record"), "received_sha256": claimed(&self.record, "received_result")})
    }
    fn inputs(&self, job: &str) -> serde_json::Map<String, Value> {
        let mut args = serde_json::Map::new();
        args.insert("jobId".into(), json!(job));
        args.insert("inputManifest".into(), self.case["input_manifest"].clone());
        args.insert("ruleCheckAggregate".into(), self.case["rule_check_aggregate"].clone());
        args.insert("rulePackExclusions".into(), self.case.get("rule_pack_exclusions").cloned().unwrap_or(json!([])));
        args.insert("expectedRecord".into(), self.expected());
        args
    }
    fn record_args(&self, job: &str, request: Value) -> Value {
        let mut args = self.inputs(job);
        args.insert("request".into(), request);
        Value::Object(args)
    }
    fn save_args(&self, job: &str, mode: &str, expected_base: Value, confirmed: Value) -> Value {
        let save = &self.form["save"];
        let mut args = self.inputs(job);
        for (key, value) in [
            ("mode", json!(mode)),
            ("projectId", self.model()["project"]["id"].clone()),
            // The session sends `model.project.name`; an invented model without one sends the stored default.
            ("projectName", json!(self.project_name())),
            ("model", self.model().clone()),
            ("editorIntents", save["editor_intents"].clone()),
            ("proposal", save["proposal"].clone()),
            ("selectedReviewTarget", save["selected_review_target"].clone()),
            ("modelHash", save["model_hash"].clone()),
            ("confirmedRevision", confirmed),
            ("expectedBase", expected_base),
        ] {
            args.insert(key.into(), value);
        }
        Value::Object(args)
    }
    fn js_text(&self, zero: NegativeZero) -> String {
        let mut out = Vec::new();
        result_js_text::write(&self.text, zero, &mut out).unwrap();
        String::from_utf8(out).unwrap()
    }
}

fn cases() -> Vec<Case> {
    let corpus = preview_cases();
    forms()["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|form| {
            let id = form["id"].as_str().unwrap().to_string();
            let case = corpus[&id].clone();
            let manifest_text = case["input_manifest"]["canonical_bytes"].as_str().unwrap().to_string();
            let manifest: Value = serde_json::from_str(&manifest_text).unwrap();
            Case {
                text: source_text(&case["source"]),
                record: case["expected"]["record"].clone(),
                mode: manifest["solver_basis"]["solver_mode"].as_str().unwrap().to_string(),
                manifest_text,
                manifest,
                form: form.clone(),
                case,
                id,
            }
        })
        .collect()
}

fn digest(text: &str) -> Value {
    let sha: String = Sha256::digest(text.as_bytes()).iter().map(|byte| format!("{byte:02x}")).collect();
    json!({"sha256": sha, "length": text.len()})
}

fn text_of(error: CommandError<String>) -> String {
    match error {
        CommandError::Command(text) | CommandError::Response(text) => text,
        CommandError::Ingress(error) => format!("INGRESS: {error:?}"),
    }
}

fn body(args: &Value) -> InvokeBody {
    InvokeBody::Raw(serde_json::to_vec(args).unwrap())
}

/// The native Record Run's two steps in this thread, its reply as the webview reads it (a JSON body).
fn native_record(registry: &SolveJobRegistry, path: &Path, args: &Value, limit: Option<i32>) -> Result<Value, String> {
    let (raw, lease) = reserve(registry, &body(args), &headers(Some(INGRESS_ARGS_JSON_V1)), RECORD_COMMAND).map_err(text_of)?;
    let path = path.to_path_buf();
    run_record(registry, move || Ok(path), raw, lease, &FROZEN, limit).map(|reply| serde_json::from_str(reply.text()).unwrap()).map_err(text_of)
}

/// The native save's two steps in this thread.
fn native_save(registry: &SolveJobRegistry, path: &Path, args: &Value, limits: &SaveLimits) -> Result<Value, String> {
    let (raw, lease) = reserve(registry, &body(args), &headers(Some(INGRESS_ARGS_JSON_V1)), SAVE_COMMAND).map_err(text_of)?;
    let path = path.to_path_buf();
    run_save(registry, move || Ok(path), raw, lease, limits, &frozen_seconds).map(|reply| serde_json::from_str(reply.text()).unwrap()).map_err(text_of)
}

fn unlimited() -> SaveLimits {
    SaveLimits { sqlite_length: None, withheld_bytes: crate::solve_jobs::MAX_DISPLAYABLE_RESULT_BYTES }
}

/// A stored project with base `base`, as a session that saved it leaves it.
fn stored_project(path: &Path, project_id: &str, model: &Value) {
    let (mut connection, _) = crate::open_project_store(path).unwrap();
    crate::upsert_project(&mut connection, project_id, "Invented project", model, &json!([]), &Value::Null, &Value::Null, &Value::Null, &Value::Null, &Value::Null, &json!({"value": "base"})).unwrap();
}

type Rows = Vec<Vec<(String, rusqlite::types::Value)>>;

/// Every row of `table`, every column but `skip`, in rowid order.
fn dump(path: &Path, table: &str, skip: &[&str]) -> Rows {
    let connection = Connection::open(path).unwrap();
    let mut statement = connection.prepare(&format!("SELECT * FROM {table} ORDER BY rowid")).unwrap();
    let names: Vec<String> = statement.column_names().iter().map(|name| name.to_string()).collect();
    let mut rows = statement.query([]).unwrap();
    let mut out = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let mut cells = Vec::new();
        for (index, name) in names.iter().enumerate() {
            if !skip.contains(&name.as_str()) {
                cells.push((name.clone(), row.get::<_, rusqlite::types::Value>(index).unwrap()));
            }
        }
        out.push(cells);
    }
    out
}

/// The two stores' rows are equal; a difference names its table, row and column.
fn same_rows(a: &Path, b: &Path, table: &str, skip: &[&str], id: &str) {
    let (left, right) = (dump(a, table, skip), dump(b, table, skip));
    assert_eq!(left.len(), right.len(), "{id}: {table} has {} rows natively, {} by the submitted path", right.len(), left.len());
    for (index, (l, r)) in left.iter().zip(&right).enumerate() {
        for ((name, lv), (_, rv)) in l.iter().zip(r) {
            if lv != rv {
                let show = |value: &rusqlite::types::Value| match value {
                    rusqlite::types::Value::Text(text) => format!("text of {} bytes: {}…", text.len(), &text[..text.len().min(160)]),
                    other => format!("{other:?}"),
                };
                panic!("{id}: {table} row {index} column {name} differs:\n submitted: {}\n native:    {}", show(lv), show(rv));
            }
        }
    }
}

const RUN_TABLES: [&str; 4] = ["analysis_runs", "analysis_run_labels", "model_states", "model_state_labels"];

fn run_labels() -> Value {
    json!({"run_name": "Invented run – rev A", "tags": ["tie-in"], "notes": ["Invented note."], "assumptions": []})
}
fn state_labels() -> Value {
    json!({"state_name": "After solve", "tags": [], "notes": [], "assumptions": []})
}

/// The submitted path's Record Run request for `case`, as the webview sends it today.
fn submitted_request(case: &Case, state_id: Option<&str>) -> RecordRunRequest {
    RecordRunRequest {
        project_id: case.project_id().to_string(),
        expected_base: "base".into(),
        run_record: case.record.clone(),
        result_json: case.js_text(NegativeZero::Keep),
        input_manifest_canonical: case.manifest_text.clone(),
        state_id: state_id.map(str::to_string),
        new_state: state_id.is_none().then(|| NewStateRequest { labels: crate::model_states::labels_from_value(state_labels()).unwrap(), parent_state_id: None }),
        run_labels: run_labels_from_value(run_labels()).unwrap(),
    }
}

fn native_request(state_id: Option<&str>) -> Value {
    match state_id {
        Some(state_id) => json!({"expected_base": "base", "state_id": state_id, "new_state": null, "run_labels": run_labels()}),
        None => json!({"expected_base": "base", "state_id": null, "new_state": {"labels": state_labels(), "parent_state_id": null}, "run_labels": run_labels()}),
    }
}

/// Every preview case with a record: the native Record Run's rows and outcome
/// are the submitted path's, for a new post-solve snapshot and then for the
/// existing state; the snapshot is written once.
#[test]
fn record_run_stores_the_submitted_paths_rows_byte_for_byte() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("record");
    let all = cases();
    assert!(all.len() >= 5);
    let mut routes = std::collections::BTreeSet::new();
    for case in &all {
        routes.insert(case.record["analysis_run"]["reproducibility"]["semantic_contract"]["id"].as_str().unwrap().to_string());
        assert_eq!(digest(&case.js_text(NegativeZero::Keep)), case.form["js_text"], "{}: the submitted text is the TypeScript's", case.id);
        let name = case.id.replace(|c: char| !c.is_ascii_alphanumeric(), "-");
        let (submitted, native) = (dir.join(format!("{name}-submitted.sqlite")), dir.join(format!("{name}-native.sqlite")));
        stored_project(&submitted, case.project_id(), case.model());
        stored_project(&native, case.project_id(), case.model());
        let job = held(&registry, &gate, &case.text, &case.mode, ResultCustody::Native);
        let mut state_id: Option<String> = None;
        for _ in 0..2 {
            let legacy = record_run_in_store(&submitted, submitted_request(case, state_id.as_deref()), &FROZEN).unwrap_or_else(|e| panic!("{}: {e}", case.id));
            let reply = native_record(&registry, &native, &case.record_args(&job, native_request(state_id.as_deref())), None).unwrap_or_else(|e| panic!("{}: {e}", case.id));
            assert_eq!(reply, serde_json::to_value(&legacy).unwrap(), "{}: the outcome", case.id);
            for table in RUN_TABLES {
                same_rows(&submitted, &native, table, &[], &case.id);
            }
            state_id = Some(legacy.state.state_id.clone());
        }
        assert_eq!(dump(&native, "model_states", &[]).len(), 1, "{}: one snapshot", case.id);
        assert_eq!(dump(&native, "analysis_runs", &[]).len(), 2, "{}: two runs", case.id);
        assert!(registry.dispose_result(&job).unwrap().released);
    }
    assert!(routes.len() >= 4, "{routes:?}");
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0, "every lease ended with its command");
}

/// Every preview case with a record: the native Save and Create Project write
/// the submitted path's project and search rows (but its save times) for the
/// request the webview sends today, store the session's envelope hash, and
/// reply with the stored project's small fields only.
#[test]
fn save_and_create_store_the_submitted_paths_rows_and_the_sessions_envelope_hash() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("save");
    for case in &cases() {
        let save = &case.form["save"];
        let stringified = case.js_text(NegativeZero::AsZero);
        assert_eq!(digest(&stringified), case.form["stringify_text"], "{}: the sent text is the TypeScript's", case.id);
        let mechanics_result: Value = serde_json::from_str(&stringified).unwrap();
        let name = case.id.replace(|c: char| !c.is_ascii_alphanumeric(), "-");
        let job = held(&registry, &gate, &case.text, &case.mode, ResultCustody::Native);
        for mode in ["save", "create"] {
            let (submitted, native) = (dir.join(format!("{name}-{mode}-submitted.sqlite")), dir.join(format!("{name}-{mode}-native.sqlite")));
            let model = case.model().clone();
            let legacy = if mode == "save" {
                crate::save_project_in_store(
                    submitted.clone(),
                    crate::SaveLocalProjectRequest {
                        project_id: model["project"]["id"].as_str().unwrap().to_string(),
                        project_name: case.project_name(),
                        model: model.clone(),
                        editor_intents: save["editor_intents"].clone(),
                        proposal: save["proposal"].clone(),
                        selected_review_target: save["selected_review_target"].clone(),
                        mechanics_result: mechanics_result.clone(),
                        analysis_run: case.record.clone(),
                        model_hash: save["model_hash"].clone(),
                        project_envelope_hash: save["envelope_hash"].clone(),
                        confirmed_revision: None,
                        expected_base: None,
                    },
                )
            } else {
                crate::create_project_in_store(
                    submitted.clone(),
                    model.clone(),
                    Some(save["editor_intents"].clone()),
                    Some(save["proposal"].clone()),
                    Some(save["selected_review_target"].clone()),
                    Some(mechanics_result.clone()),
                    Some(case.record.clone()),
                    Some(save["model_hash"].clone()),
                    Some(save["envelope_hash"].clone()),
                    None,
                )
            }
            .unwrap_or_else(|e| panic!("{}: {e}", case.id));
            let reply = native_save(&registry, &native, &case.save_args(&job, mode, Value::Null, Value::Null), &unlimited()).unwrap_or_else(|e| panic!("{} {mode}: {e}", case.id));
            same_rows(&submitted, &native, "local_projects", &["created_at_unix", "updated_at_unix"], &case.id);
            same_rows(&submitted, &native, "local_project_fts", &[], &case.id);
            assert_eq!(reply["project_envelope_hash"], save["envelope_hash"], "{}: the session's envelope hash", case.id);
            assert_eq!(reply["stored_model_hash"], save["model_hash"]["value"], "{}", case.id);
            let stored = dump(&native, "local_projects", &[]);
            let column = |name: &str| match &stored[0].iter().find(|(column, _)| column == name).unwrap().1 {
                rusqlite::types::Value::Text(text) => text.len() as u64,
                other => panic!("{other:?}"),
            };
            assert_eq!(reply["result"], json!({"outcome": "saved", "run_id": case.record["analysis_run"]["run_id"], "result_bytes": column("mechanics_result_json"), "record_bytes": column("analysis_run_json")}), "{}", case.id);
            let mut summary = reply["summary"].clone();
            let mut expected = serde_json::to_value(&legacy.summary).unwrap();
            for value in [&mut summary, &mut expected] {
                value.as_object_mut().unwrap().remove("database_path");
            }
            assert_eq!(summary, expected, "{} {mode}: the summary", case.id);
            assert_eq!((&reply["proposal"], &reply["selected_review_target"], &reply["model_hash"]), (&legacy.proposal, &legacy.selected_review_target, &legacy.model_hash));
            for absent in ["model", "mechanics_result", "analysis_run", "editor_intents"] {
                assert!(reply.get(absent).is_none(), "the reply carries no {absent}");
            }
            assert!(serde_json::to_string(&reply).unwrap().len() < 16 * 1024, "the reply is small");
        }
        assert!(registry.dispose_result(&job).unwrap().released);
    }
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0, "every lease ended with its command");
}

/// The bytes SQLite's record of `table`'s single row takes, from its stored
/// values' types and lengths (as `row_record` measures a row about to be written).
fn stored_row_record(path: &Path, table: &str) -> u64 {
    let connection = Connection::open(path).unwrap();
    let mut statement = connection.prepare(&format!("SELECT * FROM {table}")).unwrap();
    let count = statement.column_count();
    let mut rows = statement.query([]).unwrap();
    let row = rows.next().unwrap().unwrap();
    let cells: Vec<Cell> = (0..count)
        .map(|index| match row.get::<_, rusqlite::types::Value>(index).unwrap() {
            rusqlite::types::Value::Null => Cell::Null,
            rusqlite::types::Value::Integer(value) => Cell::Integer(value),
            rusqlite::types::Value::Real(_) => Cell::Real,
            rusqlite::types::Value::Text(text) => Cell::Text(text.len() as u64),
            rusqlite::types::Value::Blob(blob) => Cell::Blob(blob.len() as u64),
        })
        .collect();
    record_size(&cells)
}

fn number_after(text: &str, key: &str) -> u64 {
    let at = text.find(&format!("{key}=")).unwrap_or_else(|| panic!("{key} in {text}")) + key.len() + 1;
    text[at..].chars().take_while(char::is_ascii_digit).collect::<String>().parse().unwrap()
}

fn case_named(id: &str) -> Case {
    cases().into_iter().find(|case| case.id == id).unwrap()
}

/// Record Run at the connection's limit: each boundary refused with the exact
/// lengths, nothing written and no output text allocated (the reservations
/// and the thread's allocations show it); at the row's size the run is
/// written exactly as without a limit.
#[test]
fn record_run_refuses_above_the_connection_limit_with_exact_lengths_and_writes_nothing() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("record-limit");
    let case = case_named("preview:connected");
    let job = held(&registry, &gate, &case.text, &case.mode, ResultCustody::Native);
    let args = case.record_args(&job, native_request(None));
    let reference = dir.join("reference.sqlite");
    stored_project(&reference, case.project_id(), case.model());
    stored_read::native_lease_log::take();
    let (reply, success) = crate::ingress::counting::measure(|| native_record(&registry, &reference, &args, None));
    reply.unwrap();
    let log = stored_read::native_lease_log::take();
    let stored = dump(&reference, "analysis_runs", &[]);
    let text_length = |name: &str| match &stored[0].iter().find(|(column, _)| column == name).unwrap().1 {
        rusqlite::types::Value::Text(text) => text.len() as u64,
        other => panic!("{other:?}"),
    };
    let (result, record) = (text_length("result_json"), text_length("run_record_json"));
    let row = stored_row_record(&reference, "analysis_runs");
    assert!(log.contains(&("the received result's text".to_string(), crate::ingress::allocation_envelope(result).unwrap())), "{log:?}");
    assert!(log.contains(&("the recorded run record".to_string(), crate::ingress::allocation_envelope(record).unwrap())), "{log:?}");
    assert!(record > result, "this case's record ({record}) is longer than its result ({result})");
    let fresh = |name: &str| {
        let path = dir.join(format!("{name}.sqlite"));
        stored_project(&path, case.project_id(), case.model());
        path
    };
    for (limit, refused) in [(result - 1, Some("received result")), (result, Some("recorded run record")), (record, Some("run row")), (row - 1, Some("run row")), (row, None), (row + 1, None)] {
        let path = fresh(&format!("at-{limit}"));
        let before: Vec<Rows> = RUN_TABLES.iter().map(|table| dump(&path, table, &[])).collect();
        stored_read::native_lease_log::take();
        let (outcome, measured) = crate::ingress::counting::measure(|| native_record(&registry, &path, &args, Some(limit as i32)));
        let log = stored_read::native_lease_log::take();
        match (refused, outcome) {
            (Some(what), Err(text)) => {
                assert!(text.starts_with(RUN_RECORD_RESULT_TOO_LARGE), "{text}");
                assert!(text.contains(&format!("'s {what} is ")), "at {limit}: {text}");
                assert_eq!((number_after(&text, "result_bytes"), number_after(&text, "record_bytes"), number_after(&text, "row_bytes"), number_after(&text, "limit_bytes")), (result, record, row, limit), "{text}");
                let after: Vec<Rows> = RUN_TABLES.iter().map(|table| dump(&path, table, &[])).collect();
                assert_eq!(before, after, "at {limit}: nothing was written");
                const OUTPUTS: [&str; 4] = ["the received result's text", "the received result's text writer", "the recorded run record", "SQLite's copies of the run's row"];
                assert!(log.iter().all(|(what, _)| !OUTPUTS.contains(&what.as_str())), "at {limit}: an output was reserved: {log:?}");
                assert!(measured.peak_priced + (result + record) / 2 < success.peak_priced, "at {limit}: {measured:?} against {success:?}");
                if limit == row - 1 {
                    assert!(result <= limit && record <= limit, "the result and the record each fit; their row does not");
                }
            }
            (None, Ok(_)) => {
                for table in RUN_TABLES {
                    same_rows(&reference, &path, table, &[], &format!("at {limit}"));
                }
            }
            (expected, outcome) => panic!("at {limit}: expected {expected:?}, got {outcome:?}"),
        }
    }
    assert!(registry.dispose_result(&job).unwrap().released);
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// Save Project at the connection's limit: at the row's size the result is
/// saved; one byte lower the model-only save (the result and the record each
/// fit, their row does not), its envelope hash the session's for what was
/// stored; below the model-only row nothing is saved.
#[test]
fn save_falls_back_to_the_model_only_save_at_the_rows_limit() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("save-limit");
    let case = case_named("preview:connected");
    let job = held(&registry, &gate, &case.text, &case.mode, ResultCustody::Native);
    let args = case.save_args(&job, "save", Value::Null, Value::Null);
    let reference = dir.join("reference.sqlite");
    native_save(&registry, &reference, &args, &unlimited()).unwrap();
    let row_with = stored_row_record(&reference, "local_projects");
    let at = |limit: u64| {
        let path = dir.join(format!("at-{limit}.sqlite"));
        let outcome = native_save(&registry, &path, &args, &SaveLimits { sqlite_length: Some(limit as i32), withheld_bytes: crate::solve_jobs::MAX_DISPLAYABLE_RESULT_BYTES });
        (path, outcome)
    };
    let (path, saved) = at(row_with);
    assert_eq!(saved.unwrap()["result"]["outcome"], "saved");
    same_rows(&reference, &path, "local_projects", &[], "at the row's size");
    let (path, fallback) = at(row_with - 1);
    let fallback = fallback.unwrap();
    let result = &fallback["result"];
    assert_eq!(result["outcome"], "not_saved", "{fallback}");
    assert_eq!(result["code"], PROJECT_SAVE_RESULT_TOO_LARGE);
    assert_eq!(result["refused"]["what"], "project row");
    assert_eq!((result["row_bytes"].as_u64(), result["limit_bytes"].as_u64()), (Some(row_with), Some(row_with - 1)));
    assert!(result["result_bytes"].as_u64().unwrap() < row_with - 1 && result["record_bytes"].as_u64().unwrap() < row_with - 1, "each fits alone");
    assert!(fallback["summary"]["message"].as_str().unwrap().contains("the result was not saved"), "{fallback}");
    assert_eq!(fallback["project_envelope_hash"], case.form["save"]["model_only_envelope_hash"], "the envelope hash of what was stored");
    let stored = dump(&path, "local_projects", &[]);
    let value = |name: &str| stored[0].iter().find(|(column, _)| column == name).unwrap().1.clone();
    assert_eq!(value("mechanics_result_json"), rusqlite::types::Value::Text("null".into()));
    assert_eq!(value("analysis_run_json"), rusqlite::types::Value::Text("null".into()));
    assert_eq!(value("project_envelope_hash_json"), rusqlite::types::Value::Text(serde_json::to_string(&case.form["save"]["model_only_envelope_hash"]).unwrap()));
    let row_without = stored_row_record(&path, "local_projects");
    let (path, refused) = at(row_without - 1);
    let text = refused.unwrap_err();
    assert!(text.starts_with(PROJECT_SAVE_TOO_LARGE) && text.contains("even without the result"), "{text}");
    assert!(dump(&path, "local_projects", &[]).is_empty() && dump(&path, "local_project_fts", &[]).is_empty(), "nothing was saved");
    assert!(registry.dispose_result(&job).unwrap().released);
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// The model-only save keeps a stored run withheld for its size unless the
/// user confirmed the Replace, as a save with no result does; a stored run
/// that is not withheld is replaced in place, as before.
#[test]
fn the_model_only_save_never_erases_a_withheld_run_without_a_confirmed_replace() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("withheld");
    let case = case_named("preview:connected");
    let job = held(&registry, &gate, &case.text, &case.mode, ResultCustody::Native);
    let path = dir.join("store.sqlite");
    let first = native_save(&registry, &path, &case.save_args(&job, "save", Value::Null, Value::Null), &unlimited()).unwrap();
    let base = first["project_envelope_hash"]["value"].clone();
    let row_with = stored_row_record(&path, "local_projects");
    let squeezed = |withheld_bytes: u64| SaveLimits { sqlite_length: Some((row_with - 1) as i32), withheld_bytes };
    let before = dump(&path, "local_projects", &[]);
    let refused = native_save(&registry, &path, &case.save_args(&job, "save", base.clone(), Value::Null), &squeezed(100)).unwrap_err();
    assert!(refused.starts_with("STORED-RESULT-TOO-LARGE: Not saved."), "{refused}");
    assert_eq!(dump(&path, "local_projects", &[]), before, "the withheld run is kept");
    let revision = {
        let (connection, _) = crate::open_project_store(&path).unwrap();
        let entry = crate::stored_project_entry(&connection, case.model()["project"]["id"].as_str().unwrap()).unwrap().unwrap();
        crate::stored_revision(&connection, &entry).unwrap()
    };
    let replaced = native_save(&registry, &path, &case.save_args(&job, "save", Value::Null, json!(revision)), &squeezed(100)).unwrap();
    assert_eq!(replaced["result"]["outcome"], "not_saved");
    let stored = dump(&path, "local_projects", &[]);
    assert_eq!(stored[0].iter().find(|(column, _)| column == "mechanics_result_json").unwrap().1, rusqlite::types::Value::Text("null".into()));
    // Not withheld: an ordinary save in place replaces the stored run with none.
    let other = dir.join("other.sqlite");
    let first = native_save(&registry, &other, &case.save_args(&job, "save", Value::Null, Value::Null), &unlimited()).unwrap();
    let in_place = native_save(&registry, &other, &case.save_args(&job, "save", first["project_envelope_hash"]["value"].clone(), Value::Null), &squeezed(crate::solve_jobs::MAX_DISPLAYABLE_RESULT_BYTES)).unwrap();
    assert_eq!(in_place["result"]["outcome"], "not_saved");
    assert!(registry.dispose_result(&job).unwrap().released);
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// The measured records are SQLite's own: SQLite writes a row at its computed
/// record size and refuses it one byte lower; the refusal rolls the whole
/// transaction back, the post-solve snapshot written before the run included.
#[test]
fn record_sizes_are_sqlites_and_a_refused_row_rolls_everything_back() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("sqlite-limit");
    let case = case_named("preview:pressure");
    let job = held(&registry, &gate, &case.text, &case.mode, ResultCustody::Native);
    let reference = dir.join("reference.sqlite");
    stored_project(&reference, case.project_id(), case.model());
    native_record(&registry, &reference, &case.record_args(&job, native_request(None)), None).unwrap();
    let row = stored_row_record(&reference, "analysis_runs");
    let stored = dump(&reference, "analysis_runs", &[]);
    let text = |name: &str| match &stored[0].iter().find(|(column, _)| column == name).unwrap().1 {
        rusqlite::types::Value::Text(text) => text.clone(),
        other => panic!("{other:?}"),
    };
    let memory = stored_read::UnmeteredReadMemory;
    let payload = &case.manifest["model_basis"]["model_payload"];
    let payload_canonical = open_pipe_stress_canonical_json::canonical_json(payload);
    let checked = analysis_runs::NativeChecked { payload_hash: format!("sha256:{}", open_pipe_stress_operation_applier::sha256_hex(&payload_canonical)), payload_canonical };
    let mut request = decode_request(native_request(None));
    request.project_id = case.project_id().to_string();
    let (record_json, record_hash, result_json, result_hash, manifest_text, producer) =
        (text("run_record_json"), text("run_record_hash"), text("result_json"), text("result_hash"), text("input_manifest_json"), text("producer_run_id"));
    let texts = NativeRunTexts { record_json: &record_json, record_hash: &record_hash, result_json: &result_json, result_hash: &result_hash, manifest_text: &manifest_text, producer_run_id: &producer };
    for (limit, written) in [(row - 1, false), (row, true)] {
        let path = dir.join(format!("direct-{limit}.sqlite"));
        stored_project(&path, case.project_id(), case.model());
        let mut connection = stored_read::open_store(&path).unwrap();
        connection.set_limit(Limit::SQLITE_LIMIT_LENGTH, limit as i32);
        let transaction = stored_read::write_transaction(&mut connection).unwrap();
        let plan = analysis_runs::plan_native_run(&transaction, &request, &checked, &FROZEN, &memory).unwrap();
        let outcome = analysis_runs::write_native_run(&transaction, &request, plan, &texts, &FROZEN, &memory, |outcome| Ok(outcome.into_unreserved().unwrap()));
        match (written, outcome) {
            (false, Err(text)) => {
                assert!(text.starts_with(RUN_RECORD_RESULT_TOO_LARGE) && text.contains(stored_read::SQLITE_TOO_BIG), "{text}");
                drop(transaction);
                for table in RUN_TABLES {
                    assert!(dump(&path, table, &[]).is_empty(), "{table}: the snapshot and the run were rolled back together");
                }
            }
            (true, Ok(_)) => {
                transaction.commit().unwrap();
                for table in RUN_TABLES {
                    same_rows(&reference, &path, table, &[], "at the row's size");
                }
            }
            (written, outcome) => panic!("at {limit}: written {written}, got {:?}", outcome.map(|_| ())),
        }
    }
    // The project row: SQLite's record size, and the writer's refusal one byte lower.
    let project = dir.join("project.sqlite");
    native_save(&registry, &project, &case.save_args(&job, "save", Value::Null, Value::Null), &unlimited()).unwrap();
    let project_row = stored_row_record(&project, "local_projects");
    let stored = dump(&project, "local_projects", &[]);
    let column = |name: &str| match &stored[0].iter().find(|(column, _)| column == name).unwrap().1 {
        rusqlite::types::Value::Text(text) => text.clone(),
        other => panic!("{other:?}"),
    };
    let names = ["model_json", "editor_intents_json", "proposal_json", "selected_review_target_json", "mechanics_result_json", "analysis_run_json", "model_hash_json", "project_envelope_hash_json"];
    let values: Vec<String> = names.iter().map(|name| column(name)).collect();
    let texts = crate::ProjectTexts {
        model_json: &values[0],
        editor_intents_json: &values[1],
        proposal_json: &values[2],
        selected_review_target_json: &values[3],
        mechanics_result_json: &values[4],
        analysis_run_json: &values[5],
        model_hash_json: &values[6],
        project_envelope_hash_json: &values[7],
    };
    let (id, name) = (column("project_id"), column("project_name"));
    let created = match &stored[0].iter().find(|(column, _)| column == "created_at_unix").unwrap().1 {
        rusqlite::types::Value::Integer(value) => *value,
        other => panic!("{other:?}"),
    };
    for (limit, written) in [(project_row - 1, false), (project_row, true)] {
        let path = dir.join(format!("project-{limit}.sqlite"));
        let (mut connection, _) = crate::open_project_store(&path).unwrap();
        connection.set_limit(Limit::SQLITE_LIMIT_LENGTH, limit as i32);
        let transaction = crate::immediate_transaction(&mut connection).unwrap();
        let outcome = crate::write_project_texts(&transaction, &id, &name, &texts, &texts.search_text(), created);
        assert_eq!(outcome.is_ok(), written, "at {limit}: {outcome:?}");
        if written {
            transaction.commit().unwrap();
            same_rows(&project, &path, "local_projects", &[], "at the project row's size");
        } else {
            assert!(stored_read::sql_error(&outcome.unwrap_err()).starts_with(stored_read::SQLITE_TOO_BIG));
            drop(transaction);
            assert!(dump(&path, "local_projects", &[]).is_empty() && dump(&path, "local_project_fts", &[]).is_empty());
        }
    }
    assert!(registry.dispose_result(&job).unwrap().released);
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

fn decode_request(value: Value) -> NativeRunRequest {
    let mut fields = command_ingress::Fields::of(value, "request").unwrap();
    let run_labels = fields.required_value("run_labels").unwrap();
    NativeRunRequest {
        project_id: String::new(),
        expected_base: fields.required("expected_base").unwrap(),
        state_id: fields.optional("state_id").unwrap(),
        new_state: analysis_runs::new_state_from_value(fields.value_or_null("new_state")).unwrap(),
        run_labels: run_labels_from_value(run_labels).unwrap(),
    }
}

/// The record's binding: a stale preview, another custody, another solver
/// mode, a disposed job, a missing project, a moved base, a missing or
/// unnamed state, a project the save would replace unconfirmed: each refused
/// by name, nothing written, every lease ended.
#[test]
fn persistence_binds_the_held_job_and_the_previewed_record() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("binding");
    let case = case_named("preview:connected");
    let job = held(&registry, &gate, &case.text, &case.mode, ResultCustody::Native);
    let path = dir.join("store.sqlite");
    stored_project(&path, case.project_id(), case.model());
    let unchanged = || -> Vec<Rows> { RUN_TABLES.iter().chain(["local_projects"].iter()).map(|table| dump(&path, table, &[])).collect() };
    let before = unchanged();
    let mut stale = case.record_args(&job, native_request(None));
    stale["expectedRecord"]["record_sha256"] = json!("0".repeat(64));
    let other_mode = if case.mode == "sparse_interactive" { "dense_scrutiny" } else { "sparse_interactive" };
    let webview_job = held(&registry, &gate, &case.text, &case.mode, ResultCustody::Webview);
    let mode_job = held(&registry, &gate, &case.text, other_mode, ResultCustody::Native);
    let disposed_job = held(&registry, &gate, &case.text, &case.mode, ResultCustody::Native);
    assert!(registry.dispose_result(&disposed_job).unwrap().released);
    let mut moved = case.record_args(&job, native_request(None));
    moved["request"]["expected_base"] = json!("another base");
    let mut unnamed = case.record_args(&job, native_request(None));
    unnamed["request"]["new_state"] = Value::Null;
    let refusals: Vec<(Value, &str)> = vec![
        (stale, RECORD_STALE),
        (case.record_args(&webview_job, native_request(None)), CUSTODY),
        (case.record_args(&mode_job, native_request(None)), CUSTODY),
        (case.record_args(&disposed_job, native_request(None)), "INGRESS"),
        (moved, "PROJECT-BASE-MOVED"),
        (case.record_args(&job, native_request(Some("state:missing"))), "ANALYSIS-RUN-STATE-NOT-FOUND"),
        (unnamed, "ANALYSIS-RUN-STATE-REQUIRED"),
    ];
    for (args, code) in refusals {
        let text = native_record(&registry, &path, &args, None).unwrap_err();
        assert!(text.starts_with(code), "{code}: {text}");
        assert_eq!(unchanged(), before, "{code}: nothing was written");
    }
    let missing = dir.join("missing.sqlite");
    let text = native_record(&registry, &missing, &case.record_args(&job, native_request(None)), None).unwrap_err();
    assert!(text.starts_with("ANALYSIS-RUN-PROJECT-MISSING"), "{text}");
    // The save: a stale preview, and a stored project replaced unconfirmed.
    let mut stale_save = case.save_args(&job, "save", Value::Null, Value::Null);
    stale_save["expectedRecord"]["received_sha256"] = json!("1".repeat(64));
    assert!(native_save(&registry, &path, &stale_save, &unlimited()).unwrap_err().starts_with(RECORD_STALE));
    let text = native_save(&registry, &path, &case.save_args(&job, "create", Value::Null, Value::Null), &unlimited()).unwrap_err();
    assert!(text.starts_with("PROJECT-ID-EXISTS"), "{text}");
    let text = native_save(&registry, &path, &case.save_args(&job, "save", json!("another base"), Value::Null), &unlimited()).unwrap_err();
    assert!(text.starts_with("PROJECT-BASE-MOVED"), "{text}");
    assert_eq!(unchanged(), before, "nothing was written");
    for job in [&job, &webview_job, &mode_job] {
        registry.dispose_result(job).unwrap();
    }
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// SQLite's record format: header varints and data, sized as `OP_MakeRecord` sizes them.
#[test]
fn record_size_follows_sqlites_record_format() {
    assert_eq!(record_size(&[]), 1);
    assert_eq!(record_size(&[Cell::Null, Cell::Integer(0), Cell::Integer(1)]), 4);
    assert_eq!(record_size(&[Cell::Integer(2)]), 3);
    assert_eq!(record_size(&[Cell::Integer(1_793_000_000)]), 6);
    assert_eq!(record_size(&[Cell::Text(0)]), 2);
    assert_eq!(record_size(&[Cell::Text(57)]), 2 + 57);
    assert_eq!(record_size(&[Cell::Text(58)]), 3 + 58);
    // A header past 126 bytes takes a two-byte length.
    assert_eq!(record_size(&vec![Cell::Text(100); 64]), 2 + 64 * 2 + 6400);
}
