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
//!   (but its two save times), and no search row (store schema v14 retired the
//!   FTS sidecar); the same outcome; and the
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
pub(crate) fn frozen_seconds() -> Result<i64, String> {
    Ok(NOW_SECONDS)
}
pub(crate) const FROZEN: RunWriteHooks<'static> = RunWriteHooks { now: &frozen_now, state_token: &frozen_token, between_rows: &|_, _| Ok(()) };

/// A fresh directory for this test's stores.
pub(crate) fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("swbpipe-native-persistence-{name}-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// A native-custody job started on the model text `intake` (the body the
/// session's start sends), solved in `mode`, holding `text`.
pub(crate) fn held(registry: &SolveJobRegistry, gate: &Gate, text: &str, intake: &str, mode: &str, custody: ResultCustody) -> String {
    let started = gate.started_count();
    let receipt = registry.start_with_custody(intake.as_bytes(), Some(mode), custody).unwrap();
    gate.wait_started(started + 1);
    gate.release(text);
    let status = wait_terminal(registry, &receipt.job_id);
    assert_eq!(status.state, "completed", "{status:?}");
    receipt.job_id
}

pub(crate) fn claimed(record: &Value, scope: &str) -> String {
    record["analysis_run"]["hashes"].as_array().unwrap().iter().find(|h| h["payload_scope"] == scope).unwrap()["value"].as_str().unwrap().to_string()
}

/// One preview case and what the tests read of it.
pub(crate) struct Case {
    pub(crate) id: String,
    pub(crate) text: String,
    /// The model text the session's solve sent (`intake_text`, `checkedJsonText` of the manifest's model).
    pub(crate) intake: String,
    pub(crate) record: Value,
    pub(crate) manifest_text: String,
    pub(crate) manifest: Value,
    pub(crate) mode: String,
    pub(crate) form: Value,
    pub(crate) case: Value,
}

impl Case {
    pub(crate) fn model(&self) -> &Value {
        &self.manifest["model_basis"]["model_payload"]
    }
    pub(crate) fn project_id(&self) -> &str {
        self.manifest["model_basis"]["model_ref"].as_str().unwrap()
    }
    pub(crate) fn project_name(&self) -> String {
        self.model()["project"]["name"].as_str().map_or_else(|| crate::project_name_from_model(self.model()), str::to_string)
    }
    pub(crate) fn expected(&self) -> Value {
        json!({"run_id": self.record["analysis_run"]["run_id"], "record_sha256": claimed(&self.record, "analysis_run_record"), "received_sha256": claimed(&self.record, "received_result")})
    }
    pub(crate) fn inputs(&self, job: &str) -> serde_json::Map<String, Value> {
        let mut args = serde_json::Map::new();
        args.insert("jobId".into(), json!(job));
        args.insert("inputManifest".into(), self.case["input_manifest"].clone());
        args.insert("ruleCheckAggregate".into(), self.case["rule_check_aggregate"].clone());
        args.insert("rulePackExclusions".into(), self.case.get("rule_pack_exclusions").cloned().unwrap_or(json!([])));
        args.insert("expectedRecord".into(), self.expected());
        args.insert("intakeText".into(), json!(self.intake));
        args
    }
    pub(crate) fn record_args(&self, job: &str, request: Value) -> Value {
        let mut args = self.inputs(job);
        args.insert("request".into(), request);
        Value::Object(args)
    }
    pub(crate) fn save_args(&self, job: &str, mode: &str, expected_base: Value, confirmed: Value) -> Value {
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

pub(crate) fn cases() -> Vec<Case> {
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
                intake: case["intake_text"].as_str().unwrap_or_else(|| panic!("{id}: the session's intake text")).to_string(),
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

pub(crate) fn text_of(error: CommandError<String>) -> String {
    match error {
        CommandError::Command(text) | CommandError::Response(text) => text,
        CommandError::Ingress(error) => format!("INGRESS: {error:?}"),
    }
}

pub(crate) fn body(args: &Value) -> InvokeBody {
    InvokeBody::Raw(serde_json::to_vec(args).unwrap())
}

/// The native Record Run's two steps in this thread, its reply as the webview reads it (a JSON body).
pub(crate) fn native_record(registry: &SolveJobRegistry, path: &Path, args: &Value, limit: Option<i32>) -> Result<Value, String> {
    let (raw, lease) = reserve(registry, &body(args), &headers(Some(INGRESS_ARGS_JSON_V1)), RECORD_COMMAND).map_err(text_of)?;
    let path = path.to_path_buf();
    run_record(registry, move || Ok(path), raw, lease, &FROZEN, limit).map(|reply| serde_json::from_str(reply.text()).unwrap()).map_err(text_of)
}

/// The native save's two steps in this thread.
pub(crate) fn native_save(registry: &SolveJobRegistry, path: &Path, args: &Value, limits: &SaveLimits) -> Result<Value, String> {
    let (raw, lease) = reserve(registry, &body(args), &headers(Some(INGRESS_ARGS_JSON_V1)), SAVE_COMMAND).map_err(text_of)?;
    let path = path.to_path_buf();
    run_save(registry, move || Ok(path), raw, lease, limits, &frozen_seconds).map(|reply| serde_json::from_str(reply.text()).unwrap()).map_err(text_of)
}

pub(crate) fn unlimited() -> SaveLimits {
    SaveLimits { sqlite_length: None, withheld_bytes: crate::solve_jobs::MAX_DISPLAYABLE_RESULT_BYTES }
}

/// A stored project with base `base`, as a session that saved it leaves it.
pub(crate) fn stored_project(path: &Path, project_id: &str, model: &Value) {
    let (mut connection, _) = crate::open_project_store(path).unwrap();
    crate::upsert_project(&mut connection, project_id, "Invented project", model, &json!([]), &Value::Null, &Value::Null, &Value::Null, &Value::Null, &Value::Null, &json!({"value": "base"})).unwrap();
}

pub(crate) type Rows = Vec<Vec<(String, rusqlite::types::Value)>>;

/// Every row of `table`, every column but `skip`, in rowid order.
pub(crate) fn dump(path: &Path, table: &str, skip: &[&str]) -> Rows {
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
/// The store has no FTS sidecar table (store schema v14 retired it; a save
/// creates none).
pub(crate) fn no_fts_sidecar(path: &Path, id: &str) {
    let objects = crate::tests::fts_sidecar_objects(&Connection::open(path).unwrap());
    assert!(objects.is_empty(), "{id}: no FTS sidecar, found {objects:?}");
}

pub(crate) fn same_rows(a: &Path, b: &Path, table: &str, skip: &[&str], id: &str) {
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

pub(crate) fn run_labels() -> Value {
    json!({"run_name": "Invented run – rev A", "tags": ["tie-in"], "notes": ["Invented note."], "assumptions": []})
}
pub(crate) fn state_labels() -> Value {
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

pub(crate) fn native_request(state_id: Option<&str>) -> Value {
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
        let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
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
/// the submitted path's project row (but its save times) and no search row for the
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
        let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
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
            no_fts_sidecar(&submitted, &case.id);
            no_fts_sidecar(&native, &case.id);
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
pub(crate) fn stored_row_record(path: &Path, table: &str) -> u64 {
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
    let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
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
                const OUTPUTS: [&str; 3] = ["the received result's text", "the recorded run record", "SQLite's copies of the run's row"];
                assert!(log.iter().all(|(what, _)| !OUTPUTS.contains(&what.as_str())), "at {limit}: an output was reserved: {log:?}");
                assert_eq!(log.iter().filter(|(what, _)| what == "the received result's text writer").count(), 1,
                    "at {limit}: counting scratch is reserved even when the output is refused: {log:?}");
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
    let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
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
    assert!(dump(&path, "local_projects", &[]).is_empty(), "nothing was saved");
    no_fts_sidecar(&path, "refused");
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
    let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
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
    let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
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
    let texts = || NativeRunTexts { record_json: record_json.clone(), record_hash: &record_hash, result_json: result_json.clone(), result_hash: &result_hash, manifest_text: &manifest_text, producer_run_id: &producer, leases: Vec::new() };
    for (limit, written) in [(row - 1, false), (row, true)] {
        let path = dir.join(format!("direct-{limit}.sqlite"));
        stored_project(&path, case.project_id(), case.model());
        let mut connection = stored_read::open_store(&path).unwrap();
        connection.set_limit(Limit::SQLITE_LIMIT_LENGTH, limit as i32);
        let transaction = stored_read::write_transaction(&mut connection).unwrap();
        let plan = analysis_runs::plan_native_run(&transaction, &request, &checked, &FROZEN, &memory).unwrap();
        let outcome = analysis_runs::write_native_run(&transaction, &request, plan, texts(), &FROZEN, &memory, |outcome| Ok(outcome.into_unreserved().unwrap()));
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
        let outcome = crate::upsert_project_texts(&transaction, &id, &name, &texts, created);
        assert_eq!(outcome.is_ok(), written, "at {limit}: {outcome:?}");
        if written {
            transaction.commit().unwrap();
            same_rows(&project, &path, "local_projects", &[], "at the project row's size");
        } else {
            let error = outcome.unwrap_err();
            assert!(stored_read::sql_error(&error).starts_with(stored_read::SQLITE_TOO_BIG));
            let code = project_write_error(&error);
            assert!(code.starts_with(PROJECT_SAVE_TOO_LARGE) && code.contains("nothing was saved"), "{code}");
            assert!(!code.starts_with(PROJECT_SAVE_RESULT_TOO_LARGE), "rollback is not a successful model-only save: {code}");
            drop(transaction);
            assert!(dump(&path, "local_projects", &[]).is_empty());
            no_fts_sidecar(&path, "rolled back");
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
    let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
    let path = dir.join("store.sqlite");
    stored_project(&path, case.project_id(), case.model());
    let unchanged = || -> Vec<Rows> { RUN_TABLES.iter().chain(["local_projects"].iter()).map(|table| dump(&path, table, &[])).collect() };
    let before = unchanged();
    let mut stale = case.record_args(&job, native_request(None));
    stale["expectedRecord"]["record_sha256"] = json!("0".repeat(64));
    let other_mode = if case.mode == "sparse_interactive" { "dense_scrutiny" } else { "sparse_interactive" };
    let webview_job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Webview);
    let mode_job = held(&registry, &gate, &case.text, &case.intake, other_mode, ResultCustody::Native);
    let disposed_job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
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

/// The case's manifest with its model edited and sealed again (canonical
/// bytes, digest and ref), the project id kept, and the record the preview
/// command builds with it: a manifest the builder accepts for the held result.
fn swapped_manifest(registry: &SolveJobRegistry, case: &Case, job: &str, edit: impl FnOnce(&mut Value)) -> (Value, Value) {
    let mut manifest = case.manifest.clone();
    edit(&mut manifest["model_basis"]["model_payload"]);
    assert_eq!(manifest["model_basis"]["model_payload"]["project"]["id"], case.manifest["model_basis"]["model_payload"]["project"]["id"], "the project id is kept");
    let mut canonical_bytes = Vec::new();
    canonical(&manifest, &mut canonical_bytes);
    let canonical_bytes = String::from_utf8(canonical_bytes).unwrap();
    let sha = hex(&Sha256::digest(canonical_bytes.as_bytes()));
    let evidence = json!({"manifest_ref": crate::analysis_runs::input_manifest_ref(case.project_id(), &sha), "manifest_sha256": sha, "canonical_bytes": canonical_bytes});
    let mut args = case.inputs(job);
    args.insert("inputManifest".into(), evidence.clone());
    let (raw, lease) = crate::analysis_run_v03_build::reserve(registry, &body(&Value::Object(args)), &headers(Some(INGRESS_ARGS_JSON_V1))).unwrap();
    let header: Value = serde_json::from_slice(&crate::analysis_run_v03_build::run(registry.clone(), raw, lease).expect("the builder accepts the swapped manifest")).unwrap();
    let record = &header["record"];
    let expected = json!({"run_id": record["analysis_run"]["run_id"], "record_sha256": claimed(record, "analysis_run_record"), "received_sha256": claimed(record, "received_result")});
    (evidence, expected)
}

/// The first number under `value`'s `nodes`, changed by one ulp-scale step.
fn nudge_first_node_number(value: &mut Value) -> bool {
    fn nudge(value: &mut Value) -> bool {
        match value {
            Value::Number(number) => {
                let f = number.as_f64().unwrap();
                *number = serde_json::Number::from_f64(if f == 0.0 { 1e-9 } else { f * (1.0 + 1e-12) }).unwrap();
                true
            }
            Value::Array(items) => items.iter_mut().any(nudge),
            Value::Object(map) => map.values_mut().any(nudge),
            _ => false,
        }
    }
    value.get_mut("nodes").is_some_and(nudge)
}

/// C1 slice 5's intake binding (`bound_intake`), for Record Run and for Save
/// Project: each job is started on the model text the session's start sends
/// (`intake_text`), and the request re-sends it. A request without it, with
/// other bytes (even the same model's), naming a job started on other bytes,
/// or with a manifest whose model differs under the same project id (sealed
/// and accepted by the builder, so nothing else refuses it) is refused
/// `NATIVE-PERSISTENCE-CUSTODY` before anything is written; the job's
/// custody refusals come first and the intake's before the record's
/// identity. The same model's text in another member order or layout binds
/// when the job was started on it (the connected case's session text is not
/// the manifest's canonical text; each case also binds a pretty-printed
/// layout); `-0` and `1.0` read as JavaScript reads them. Every lease ends.
#[test]
fn persistence_binds_the_solves_intake_to_the_manifests_model() {
    assert!(same_js_data(&json!({"a": [0, 1, 1e21], "b": "x"}), &serde_json::from_str(r#"{"b":"x","a":[-0,1.0,1000000000000000000000]}"#).unwrap()));
    assert!(!same_js_data(&json!({"a": 1}), &json!({"a": 1, "b": null})));
    assert!(!same_js_data(&json!([1, 2]), &json!([2, 1])));
    assert!(!same_js_data(&json!({"a": "1"}), &json!({"a": 1})));
    assert!(!same_js_data(&json!(0.1), &serde_json::from_str("0.10000000000000002").unwrap()));
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("intake");
    let mut reordered = false;
    for id in ["preview:connected", "preview:pressure-source"] {
        let case = case_named(id);
        let canonical_payload = {
            let mut out = Vec::new();
            canonical(case.model(), &mut out);
            String::from_utf8(out).unwrap()
        };
        reordered |= case.intake != canonical_payload;
        let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
        let path = dir.join(format!("{}.sqlite", id.replace(':', "-")));
        stored_project(&path, case.project_id(), case.model());
        let unchanged = || -> Vec<Rows> { RUN_TABLES.iter().chain(["local_projects"].iter()).map(|table| dump(&path, table, &[])).collect() };
        let before = unchanged();
        // Another job holding the same result, started on the same model's text in another layout.
        let pretty = serde_json::to_string_pretty(&serde_json::from_str::<Value>(&case.intake).unwrap()).unwrap();
        let other_job = held(&registry, &gate, &case.text, &pretty, &case.mode, ResultCustody::Native);
        let (swapped, swapped_record) = swapped_manifest(&registry, &case, &job, |model| model["project"]["name"] = json!("Swapped model"));
        let (nudged, nudged_record) = swapped_manifest(&registry, &case, &job, |model| assert!(nudge_first_node_number(model), "{id}: a node number"));
        let other_mode = if case.mode == "sparse_interactive" { "dense_scrutiny" } else { "sparse_interactive" };
        let mode_job = held(&registry, &gate, &case.text, &case.intake, other_mode, ResultCustody::Native);
        type Edit = Box<dyn Fn(&mut serde_json::Map<String, Value>)>;
        let swap = |evidence: Value, record: Value| -> Edit {
            Box::new(move |args| {
                args.insert("inputManifest".into(), evidence.clone());
                args.insert("expectedRecord".into(), record.clone());
            })
        };
        let (other_, mode_) = (other_job.clone(), mode_job.clone());
        let intake_ = case.intake.clone();
        let refusals: Vec<(&str, Edit, &str)> = vec![
            ("absent", Box::new(|args| { args.remove("intakeText"); }), "is not in the request"),
            ("null", Box::new(|args| { args.insert("intakeText".into(), Value::Null); }), "is not in the request"),
            ("other bytes of the same model", { let pretty = pretty.clone(); Box::new(move |args| { args.insert("intakeText".into(), json!(pretty)); }) }, "is not the body"),
            ("one byte more", { let text = format!("{intake_} "); Box::new(move |args| { args.insert("intakeText".into(), json!(text)); }) }, "is not the body"),
            ("another job", Box::new(move |args| { args.insert("jobId".into(), json!(other_)); }), "is not the body"),
            ("a swapped model under the same project id", swap(swapped.clone(), swapped_record.clone()), "the input manifest's model is not the model"),
            ("a nudged node", swap(nudged.clone(), nudged_record.clone()), "the input manifest's model is not the model"),
            ("the intake before the record's identity", { let swapped = swapped.clone(); Box::new(move |args| { args.insert("inputManifest".into(), swapped.clone()); }) }, "the input manifest's model is not the model"),
            ("the mode before the intake", Box::new(move |args| { args.insert("jobId".into(), json!(mode_)); args.insert("intakeText".into(), json!("{}")); }), "mode, but the input manifest names"),
        ];
        for (what, edit, text) in &refusals {
            let mut args = case.record_args(&job, native_request(None));
            edit(args.as_object_mut().unwrap());
            let refused = native_record(&registry, &path, &args, None).unwrap_err();
            assert!(refused.starts_with(CUSTODY) && refused.contains(text), "{id}: Record Run, {what}: {refused}");
            let mut args = case.save_args(&job, "save", json!("base"), Value::Null);
            edit(args.as_object_mut().unwrap());
            let refused = native_save(&registry, &path, &args, &unlimited()).unwrap_err();
            assert!(refused.starts_with(CUSTODY) && refused.contains(text), "{id}: Save Project, {what}: {refused}");
            assert_eq!(unchanged(), before, "{id}, {what}: nothing was written");
        }
        // A non-string intake is the arguments' typed refusal.
        let mut args = case.record_args(&job, native_request(None));
        args["intakeText"] = json!(7);
        assert!(native_record(&registry, &path, &args, None).unwrap_err().starts_with("INGRESS"), "{id}: a typed refusal");
        // A disposed job is refused before its intake is read.
        let disposed = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
        assert!(registry.dispose_result(&disposed).unwrap().released);
        let mut args = case.record_args(&disposed, native_request(None));
        args["intakeText"] = Value::Null;
        assert!(native_record(&registry, &path, &args, None).unwrap_err().starts_with("INGRESS"), "{id}: disposed");
        assert_eq!(unchanged(), before, "{id}: nothing was written");
        // The job started on the other layout binds with its own bytes: Record Run, then the save.
        let mut args = case.record_args(&other_job, native_request(None));
        args["intakeText"] = json!(pretty);
        let reply = native_record(&registry, &path, &args, None).unwrap_or_else(|e| panic!("{id}: {e}"));
        assert_eq!(reply["state_outcome"], "created", "{id}: {reply}");
        let mut args = case.save_args(&other_job, "save", json!("base"), Value::Null);
        args["intakeText"] = json!(pretty);
        let saved = native_save(&registry, &path, &args, &unlimited()).unwrap_or_else(|e| panic!("{id}: {e}"));
        assert_eq!(saved["result"]["outcome"], "saved", "{id}: {saved}");
        for job in [&job, &other_job, &mode_job] {
            assert!(registry.dispose_result(job).unwrap().released);
        }
    }
    assert!(reordered, "a session's text that is not the manifest's canonical text binds");
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0, "every lease ended with its command");
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

const REPLIES: &str = "apps/desktop/src/test-support/native-replay/native_persistence_replies.json";
const REPLAY_REQUESTS: &str = "apps/desktop/src/test-support/native-replay/native_replay_requests.json";
/// The session tests' solve (`nativeMechanicsReplayPair("sparse_interactive", {profile: "pressure"})`).
const REPLAY_SOURCE: &str = "fixtures/results/pressure_connected_ui_mechanics_sparse.json";
/// Its model: the session sends `checkedJsonText` of this file's parse (`JSON.stringify`'s text).
const REPLAY_MODEL: &str = "fixtures/model_operations/exact_pressure_authoring_model.json";

/// `JSON.stringify(JSON.parse(text))`: JavaScript's member order and numbers, `-0` written `0`
/// (`checkedJsonText` of a parsed model writes the same).
pub(crate) fn stringify_text(text: &str) -> String {
    let mut out = Vec::new();
    result_js_text::write(text, NegativeZero::AsZero, &mut out).unwrap();
    String::from_utf8(out).unwrap()
}

pub(crate) fn repo_text(path: &str) -> String {
    std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").join(path)).unwrap_or_else(|e| panic!("{path}: {e}"))
}

/// A reply as the webview receives it: the JSON body Tauri delivers parsed,
/// or the serialized rejection; the store's path (a temporary directory) as
/// `$STORE`, the one value that is not the command's own.
fn webview_reply(outcome: Result<Serialized, CommandError<String>>) -> Value {
    match outcome {
        Ok(reply) => {
            let mut value: Value = serde_json::from_str(reply.text()).unwrap();
            if let Some(summary) = value.get_mut("summary").and_then(Value::as_object_mut) {
                summary.insert("database_path".into(), json!("$STORE"));
            }
            json!({"ok": true, "body": "json", "value": value})
        }
        Err(error) => json!({"ok": false, "error": serde_json::to_value(&error).unwrap()}),
    }
}

pub(crate) fn with_job(args: &Value, job: &str) -> Value {
    match args {
        Value::String(text) if text == "$JOB" => json!(job),
        Value::Array(items) => Value::Array(items.iter().map(|item| with_job(item, job)).collect()),
        Value::Object(map) => Value::Object(map.iter().map(|(key, item)| (key.clone(), with_job(item, job))).collect()),
        other => other.clone(),
    }
}

/// The webview's native persistence replies (`native_persistence_replies.json`)
/// are the actual commands' replies to the requests the session sends for the
/// session tests' solve, in order on one store: Save Project, Record Run with a
/// new post-solve snapshot, Record Run against it; and, each on a store of its
/// own, the model-only save at the row's limit and a Record Run above it.
/// NATIVE_PERSISTENCE_REPLIES_WRITE=1 rewrites the file.
#[test]
fn the_webviews_persistence_replies_are_the_commands_replies() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("replies");
    let requests: Value = serde_json::from_str(&repo_text(REPLAY_REQUESTS)).unwrap();
    let preview = requests["requests"]
        .as_array()
        .unwrap()
        .iter()
        .find(|request| request["command"] == crate::analysis_run_v03_build::COMMAND && request["source_path"] == REPLAY_SOURCE && request["args"]["ruleCheckAggregate"].is_null())
        .expect("the session's preview request of the replayed solve is recorded");
    let manifest: Value = serde_json::from_str(preview["args"]["inputManifest"]["canonical_bytes"].as_str().unwrap()).unwrap();
    let model = manifest["model_basis"]["model_payload"].clone();
    let text = repo_text(REPLAY_SOURCE);
    // The body the session's start sends for the replayed model; the TypeScript replay requires the
    // session's `intakeText` to be these exact bytes.
    let intake = stringify_text(&repo_text(REPLAY_MODEL));
    assert!(same_js_data(&serde_json::from_str(&intake).unwrap(), &model), "the replayed model is the manifest's");
    let job = held(&registry, &gate, &text, &intake, manifest["solver_basis"]["solver_mode"].as_str().unwrap(), ResultCustody::Native);
    let header: Value = {
        let args = with_job(&preview["args"], &job);
        let (raw, lease) = crate::analysis_run_v03_build::reserve(&registry, &body(&args), &headers(Some(INGRESS_ARGS_JSON_V1))).unwrap();
        serde_json::from_slice(&crate::analysis_run_v03_build::run(registry.clone(), raw, lease).unwrap()).unwrap()
    };
    let record = &header["record"];
    let mut inputs = preview["args"].as_object().unwrap().clone();
    inputs.insert("expectedRecord".into(), json!({"run_id": record["analysis_run"]["run_id"], "record_sha256": claimed(record, "analysis_run_record"), "received_sha256": claimed(record, "received_result")}));
    inputs.insert("intakeText".into(), json!(intake));
    let model_hash = {
        let mut hasher = Sha256::new();
        canonical(&model, &mut hasher);
        json!({"algorithm": "sha256", "canonicalization": "rfc8785_jcs", "payload_scope": "model_payload", "payload_ref": model["project"]["id"], "value": format!("sha256:{}", hex(&hasher.finalize())), "hash_status": "computed_local_preview"})
    };
    let save_args = || {
        let mut args = inputs.clone();
        for (key, value) in [
            ("mode", json!("save")),
            ("projectId", model["project"]["id"].clone()),
            ("projectName", model["project"]["name"].clone()),
            ("model", model.clone()),
            ("editorIntents", json!([])),
            ("proposal", Value::Null),
            ("selectedReviewTarget", Value::Null),
            ("modelHash", model_hash.clone()),
            ("confirmedRevision", Value::Null),
            ("expectedBase", Value::Null),
        ] {
            args.insert(key.into(), value);
        }
        Value::Object(args)
    };
    let labels = json!({"run_name": "Run – rev A", "tags": ["tie-in"], "notes": ["Invented note."], "assumptions": [{"assumption_id": "assumption:R-1", "kind": "assumption", "status": "unresolved", "statement": "Vendor stiffness pending.", "affected_refs": []}]});
    let record_args = |request: Value| {
        let mut args = inputs.clone();
        args.insert("request".into(), request);
        Value::Object(args)
    };
    let save = |path: &Path, args: &Value, limits: &SaveLimits| {
        let (raw, lease) = reserve(&registry, &body(&with_job(args, &job)), &headers(Some(INGRESS_ARGS_JSON_V1)), SAVE_COMMAND).unwrap();
        let path = path.to_path_buf();
        webview_reply(run_save(&registry, move || Ok(path), raw, lease, limits, &frozen_seconds))
    };
    let record_run = |path: &Path, args: &Value, limit: Option<i32>| {
        let (raw, lease) = reserve(&registry, &body(&with_job(args, &job)), &headers(Some(INGRESS_ARGS_JSON_V1)), RECORD_COMMAND).unwrap();
        let path = path.to_path_buf();
        webview_reply(run_record(&registry, move || Ok(path), raw, lease, &FROZEN, limit))
    };
    let mut scenarios = Vec::new();
    let store = dir.join("store.sqlite");
    let args = save_args();
    let saved = save(&store, &args, &unlimited());
    let base = saved["value"]["project_envelope_hash"]["value"].clone();
    assert_eq!(saved["value"]["result"]["outcome"], "saved", "{saved}");
    scenarios.push(json!({"store": "store", "command": SAVE_COMMAND, "args": args, "reply": saved}));
    let args = record_args(json!({"expected_base": base, "state_id": null, "new_state": {"labels": {"state_name": "After solve", "tags": [], "notes": [], "assumptions": []}, "parent_state_id": null}, "run_labels": labels}));
    let created = record_run(&store, &args, None);
    assert_eq!(created["value"]["state_outcome"], "created", "{created}");
    let state_id = created["value"]["state"]["state_id"].clone();
    scenarios.push(json!({"store": "store", "command": RECORD_COMMAND, "args": args, "reply": created}));
    let mut again = labels.clone();
    again["run_name"] = json!("Again");
    let args = record_args(json!({"expected_base": base, "state_id": state_id, "new_state": null, "run_labels": again}));
    let existing = record_run(&store, &args, None);
    assert_eq!(existing["value"]["state_outcome"], "existing", "{existing}");
    scenarios.push(json!({"store": "store", "command": RECORD_COMMAND, "args": args, "reply": existing}));
    // The model-only save, one byte under the project row with the result.
    let row_with = stored_row_record(&store, "local_projects");
    let small = dir.join("small.sqlite");
    let args = save_args();
    let fallback = save(&small, &args, &SaveLimits { sqlite_length: Some((row_with - 1) as i32), withheld_bytes: crate::solve_jobs::MAX_DISPLAYABLE_RESULT_BYTES });
    assert_eq!(fallback["value"]["result"]["outcome"], "not_saved", "{fallback}");
    scenarios.push(json!({"store": "small", "command": SAVE_COMMAND, "args": args, "reply": fallback, "sqlite_length_limit": row_with - 1}));
    let small_base = fallback["value"]["project_envelope_hash"]["value"].clone();
    let args = record_args(json!({"expected_base": small_base, "state_id": null, "new_state": {"labels": {"state_name": "After solve", "tags": [], "notes": [], "assumptions": []}, "parent_state_id": null}, "run_labels": labels}));
    let refused = record_run(&small, &args, Some((row_with - 1) as i32));
    assert!(refused["error"].as_str().is_some_and(|text| text.starts_with(RUN_RECORD_RESULT_TOO_LARGE)), "{refused}");
    scenarios.push(json!({"store": "small", "command": RECORD_COMMAND, "args": args, "reply": refused, "sqlite_length_limit": row_with - 1}));
    let document = json!({
        "document_kind": "swbpipe.desktop.native_persistence_replies",
        "note": "Written by native_persistence_tests.rs from the actual commands on a fresh store per `store` (clock and state token frozen; the store's path written $STORE); NATIVE_PERSISTENCE_REPLIES_WRITE=1 rewrites it.",
        "source_path": REPLAY_SOURCE,
        "scenarios": scenarios,
    });
    let written = format!("{}\n", serde_json::to_string_pretty(&document).unwrap());
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").join(REPLIES);
    if std::env::var_os("NATIVE_PERSISTENCE_REPLIES_WRITE").is_some() {
        std::fs::write(&path, &written).unwrap();
    }
    assert!(std::fs::read_to_string(&path).unwrap_or_default() == written, "the committed replies are the commands' replies (NATIVE_PERSISTENCE_REPLIES_WRITE=1 rewrites them)");
    assert!(registry.dispose_result(&job).unwrap().released);
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// The commands' executor part checks the header and copies the body only; admission, the parse,
/// the build and the store run in a blocking task, so a held build leaves the async runtime free.
/// The reply is a JSON body (Tauri delivers it parsed), as `record_analysis_run`'s is. Each command
/// writes its own store, so this measures only progress off the executor; overlapping writes to
/// one store are `overlapping_record_runs_on_one_store_commit_whole_or_are_refused_busy`.
#[test]
fn the_persistence_commands_run_off_the_async_executor_and_reply_with_a_json_body() {
    use crate::analysis_run_v03_build::tests::gate;
    use tauri::ipc::{InvokeResponseBody, IpcResponse};
    let host = ScriptedHost::normal();
    let (registry, gate_) = gated(&host);
    let dir = scratch("executor");
    let case = case_named("preview:connected");
    let job = held(&registry, &gate_, &case.text, &case.intake, &case.mode, ResultCustody::Native);
    let commands = crate::native_verdict::tests::runtime_workers() * 2 + 2;
    let paths: Vec<PathBuf> = (0..commands)
        .map(|index| {
            let path = dir.join(format!("store-{index}.sqlite"));
            stored_project(&path, case.project_id(), case.model());
            path
        })
        .collect();
    let held_gate = gate::hold(&registry, "read", false);
    let handles: Vec<_> = paths
        .iter()
        .enumerate()
        .map(|(index, path)| {
            let mut request = native_request(None);
            request["run_labels"]["run_name"] = json!(format!("Run {index}"));
            let (registry, body, path) = (registry.clone(), body(&case.record_args(&job, request)), path.clone());
            tauri::async_runtime::spawn(async move { record_run(&registry, move || Ok(path), &body, &headers(Some(INGRESS_ARGS_JSON_V1))).await })
        })
        .collect();
    assert!(held_gate.0.wait_entered(commands, std::time::Duration::from_secs(60)), "only {} of {commands} commands entered", held_gate.0.entered());
    let (sender, receiver) = std::sync::mpsc::channel();
    let (probe_registry, probe_job) = (registry.clone(), job.clone());
    tauri::async_runtime::spawn(async move {
        let _ = sender.send(probe_registry.status(&probe_job).map(|status| status.state));
    });
    assert_eq!(receiver.recv_timeout(std::time::Duration::from_secs(10)).expect("the runtime ran another task").unwrap(), "completed");
    held_gate.0.open();
    for handle in handles {
        let response = tauri::async_runtime::block_on(handle).unwrap().unwrap();
        match response.body().unwrap() {
            InvokeResponseBody::Json(text) => assert!(serde_json::from_str::<Value>(&text).unwrap()["run"]["recording_id"].as_str().unwrap().starts_with("runrec:")),
            InvokeResponseBody::Raw(_) => panic!("the outcome is a JSON body"),
        }
    }
    drop(held_gate);
    for path in &paths {
        assert_eq!(dump(path, "analysis_runs", &[]).len(), 1, "each recorded once");
        assert_eq!(dump(path, "model_states", &[]).len(), 1, "one snapshot");
    }
    // A malformed body is refused only by the blocking part (the executor neither prescans nor parses).
    let (raw, lease) = reserve(&registry, &InvokeBody::Raw(br#"{"jobId":"#.to_vec()), &headers(Some(INGRESS_ARGS_JSON_V1)), SAVE_COMMAND).unwrap();
    let refused = run_save(&registry, || Ok(PathBuf::from("/nonexistent")), raw, lease, &unlimited(), &frozen_seconds).unwrap_err();
    assert!(matches!(refused, CommandError::Ingress(_)), "{refused:?}");
    assert!(matches!(reserve(&registry, &InvokeBody::Raw(b"{}".to_vec()), &headers(None), RECORD_COMMAND), Err(CommandError::Ingress(_))), "the header is required");
    assert!(registry.dispose_result(&job).unwrap().released);
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// Overlapping Record Runs on one store. `open_project_store` (lib.rs) lets a second writer wait
/// 5 s for the store's write lock and then fail visibly, so the contract asserted is the store's,
/// not that every write succeeds: each command either commits its run completely or is refused
/// with the named busy refusal (`STORE-SQL-ERROR: SQLite DatabaseBusy …`); exactly the committed
/// runs are stored, each whole and readable, nothing partial is left, and the store passes
/// `quick_check`. Whether overlapping writes to one store must all succeed (per-store
/// serialization) is an open product matter. Whether any write is
/// refused in the overlapping phase depends on timing, so a second phase forces one: another
/// connection holds the write lock past the busy timeout, the command is refused as busy, every
/// table is exactly as before, and the same request then commits once the lock is released.
#[test]
fn overlapping_record_runs_on_one_store_commit_whole_or_are_refused_busy() {
    use crate::analysis_run_v03_build::tests::gate;
    use std::collections::BTreeSet;
    use tauri::ipc::{InvokeResponseBody, IpcResponse};
    const BUSY: &str = "STORE-SQL-ERROR: SQLite DatabaseBusy (";
    let host = ScriptedHost::normal();
    let (registry, gate_) = gated(&host);
    let dir = scratch("one-store-writers");
    let case = case_named("preview:connected");
    let job = held(&registry, &gate_, &case.text, &case.intake, &case.mode, ResultCustody::Native);
    let path = dir.join("store.sqlite");
    stored_project(&path, case.project_id(), case.model());
    let commands = crate::native_verdict::tests::runtime_workers() * 2 + 2;
    // Every command is held at the build, then all are released at once onto the one store.
    let held_gate = gate::hold(&registry, "read", false);
    let handles: Vec<_> = (0..commands)
        .map(|index| {
            let mut request = native_request(None);
            request["run_labels"]["run_name"] = json!(format!("Run {index}"));
            let (registry, body, path) = (registry.clone(), body(&case.record_args(&job, request)), path.clone());
            tauri::async_runtime::spawn(async move { record_run(&registry, move || Ok(path), &body, &headers(Some(INGRESS_ARGS_JSON_V1))).await })
        })
        .collect();
    assert!(held_gate.0.wait_entered(commands, std::time::Duration::from_secs(60)), "only {} of {commands} commands entered", held_gate.0.entered());
    held_gate.0.open();
    let (mut committed, mut busy) = (BTreeSet::new(), 0);
    for handle in handles {
        match tauri::async_runtime::block_on(handle).unwrap() {
            Ok(response) => match response.body().unwrap() {
                InvokeResponseBody::Json(text) => {
                    let id = serde_json::from_str::<Value>(&text).unwrap()["run"]["recording_id"].as_str().unwrap().to_string();
                    assert!(id.starts_with("runrec:"), "{id}");
                    assert!(committed.insert(id), "each committed run is its own recording");
                }
                InvokeResponseBody::Raw(_) => panic!("the outcome is a JSON body"),
            },
            Err(CommandError::Command(text)) if text.starts_with(BUSY) => busy += 1,
            Err(other) => panic!("neither committed nor refused as busy: {other:?}"),
        }
    }
    drop(held_gate);
    eprintln!("{commands} overlapping Record Runs on one store: {} committed, {busy} refused as busy", committed.len());
    assert_eq!(committed.len() + busy, commands);
    assert!(!committed.is_empty(), "the first writer to take the lock commits");
    // Exactly the committed runs are stored; a refused write left nothing.
    let connection = Connection::open(&path).unwrap();
    let stored: BTreeSet<String> = connection
        .prepare("SELECT recording_id FROM analysis_runs").unwrap()
        .query_map([], |row| row.get::<_, String>(0)).unwrap()
        .map(Result::unwrap)
        .collect();
    assert_eq!(stored, committed, "the stored runs are exactly the committed ones");
    let orphans: i64 = connection
        .query_row("SELECT COUNT(*) FROM analysis_runs WHERE NOT EXISTS (SELECT 1 FROM model_states s WHERE s.project_id = analysis_runs.project_id AND s.state_id = analysis_runs.state_id)", [], |row| row.get(0))
        .unwrap();
    assert_eq!(orphans, 0, "every stored run has its state");
    assert_eq!(dump(&path, "model_states", &[]).len(), 1, "one snapshot");
    let check: String = connection.query_row("PRAGMA quick_check", [], |row| row.get(0)).unwrap();
    assert_eq!(check, "ok");
    drop(connection);
    // Each committed run is whole: it reads back through every integrity check of the legacy read.
    for id in &committed {
        crate::analysis_runs::read_run_in_store(&path, case.project_id(), id).unwrap_or_else(|e| panic!("{id}: {e}"));
    }
    // A forced refusal: the store's write lock held elsewhere past the 5 s busy timeout.
    let tables = ["local_projects", "model_states", "analysis_runs"];
    let before: Vec<Rows> = tables.iter().map(|table| dump(&path, table, &[])).collect();
    let mut request = native_request(None);
    request["run_labels"]["run_name"] = json!("Run refused, then retried");
    let args = body(&case.record_args(&job, request));
    let blocker = Connection::open(&path).unwrap();
    blocker.execute_batch("BEGIN IMMEDIATE").unwrap();
    let store = || { let path = path.clone(); move || Ok(path) };
    let refused = tauri::async_runtime::block_on(record_run(&registry, store(), &args, &headers(Some(INGRESS_ARGS_JSON_V1))));
    blocker.execute_batch("ROLLBACK").unwrap();
    drop(blocker);
    match refused {
        Err(CommandError::Command(text)) if text.starts_with(BUSY) => {}
        Err(other) => panic!("a held write lock is refused as busy, not {other:?}"),
        Ok(_) => panic!("a write that could not take the lock was reported as success"),
    }
    let after: Vec<Rows> = tables.iter().map(|table| dump(&path, table, &[])).collect();
    assert!(before == after, "the refused write changed the store");
    let connection = Connection::open(&path).unwrap();
    let check: String = connection.query_row("PRAGMA quick_check", [], |row| row.get(0)).unwrap();
    assert_eq!(check, "ok");
    drop(connection);
    // The refusal leaves the request eligible: the same request commits once the lock is free.
    let retried = tauri::async_runtime::block_on(record_run(&registry, store(), &args, &headers(Some(INGRESS_ARGS_JSON_V1)))).unwrap();
    let InvokeResponseBody::Json(text) = retried.body().unwrap() else { panic!("the outcome is a JSON body") };
    let id = serde_json::from_str::<Value>(&text).unwrap()["run"]["recording_id"].as_str().unwrap().to_string();
    assert!(!committed.contains(&id));
    crate::analysis_runs::read_run_in_store(&path, case.project_id(), &id).unwrap();
    assert_eq!(dump(&path, "analysis_runs", &[]).len(), committed.len() + 1);
    assert!(registry.dispose_result(&job).unwrap().released);
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// RT-U4 (1) through the persistence commands: a registered pressure solve
/// (the session's `pressure` validation passed on its held bytes) is recorded
/// and saved without reading the route's reader again, and stores exactly
/// what an unregistered job of the same bytes stores (every run, label and
/// snapshot column; the project row; the replies), which
/// `record_run_stores_the_submitted_paths_rows_byte_for_byte` and
/// `save_and_create_store_the_submitted_paths_rows_and_the_sessions_envelope_hash`
/// bind to the submitted path. With the evidence held, the binding refusals
/// still refuse by name, nothing written: a stale preview, another model
/// text, a webview-custody job and a disposed job.
#[test]
fn a_registered_solve_is_recorded_and_saved_without_its_reader_and_stores_the_same_bytes() {
    use crate::analysis_run_v03_build::tests::{passes, verdict};
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("registered");
    let case = case_named("preview:pressure");
    let mut stored = Vec::new();
    for registered in [false, true] {
        let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
        if registered {
            assert_eq!(verdict(&registry, &job, Some("pressure"), Some(&case.intake)).unwrap(), crate::solve_jobs::READER_PASSED);
            assert!(registry.held_result(&job).unwrap().pressure_reader_passed());
        }
        let (record, save) = (dir.join(format!("record-{registered}.sqlite")), dir.join(format!("save-{registered}.sqlite")));
        stored_project(&record, case.project_id(), case.model());
        if registered {
            let unchanged = || -> Vec<Rows> { RUN_TABLES.iter().chain(["local_projects"].iter()).map(|table| dump(&record, table, &[])).collect() };
            let before = unchanged();
            let mut stale = case.record_args(&job, native_request(None));
            stale["expectedRecord"]["record_sha256"] = json!("0".repeat(64));
            let mut other_intake = case.record_args(&job, native_request(None));
            other_intake["intakeText"] = json!(format!("{} ", case.intake));
            let webview_job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Webview);
            assert_eq!(verdict(&registry, &webview_job, Some("pressure"), Some(&case.intake)).unwrap(), crate::solve_jobs::READER_PASSED);
            let disposed_job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
            assert_eq!(verdict(&registry, &disposed_job, Some("pressure"), Some(&case.intake)).unwrap(), crate::solve_jobs::READER_PASSED);
            assert!(registry.dispose_result(&disposed_job).unwrap().released);
            for (args, code) in [
                (stale, RECORD_STALE),
                (other_intake, CUSTODY),
                (case.record_args(&webview_job, native_request(None)), CUSTODY),
                (case.record_args(&disposed_job, native_request(None)), "INGRESS"),
            ] {
                let text = native_record(&registry, &record, &args, None).unwrap_err();
                assert!(text.starts_with(code), "{code}: {text}");
                assert_eq!(unchanged(), before, "{code}: nothing was written");
            }
            let mut stale_save = case.save_args(&job, "create", Value::Null, Value::Null);
            stale_save["expectedRecord"]["received_sha256"] = json!("1".repeat(64));
            assert!(native_save(&registry, &save, &stale_save, &unlimited()).unwrap_err().starts_with(RECORD_STALE));
            registry.dispose_result(&webview_job).unwrap();
        }
        let (reply, record_work) = passes::measure(|| native_record(&registry, &record, &case.record_args(&job, native_request(None)), None).unwrap());
        let (mut saved, save_work) = passes::measure(|| native_save(&registry, &save, &case.save_args(&job, "create", Value::Null, Value::Null), &unlimited()).unwrap());
        saved["summary"].as_object_mut().unwrap().remove("database_path");
        assert_eq!((record_work.reader_validations, save_work.reader_validations), (u64::from(!registered), u64::from(!registered)), "registered {registered}");
        stored.push((record, save, reply, saved, record_work, save_work));
        assert!(registry.dispose_result(&job).unwrap().released);
    }
    let (unregistered, registered) = (&stored[0], &stored[1]);
    for table in RUN_TABLES {
        same_rows(&unregistered.0, &registered.0, table, &[], &case.id);
    }
    same_rows(&unregistered.1, &registered.1, "local_projects", &[], &case.id);
    no_fts_sidecar(&registered.1, &case.id);
    assert_eq!((&unregistered.2, &unregistered.3), (&registered.2, &registered.3), "the same replies");
    // Only the reader's run is saved.
    assert_eq!(passes::Counts { reader_validations: 1, ..registered.4 }, unregistered.4);
    assert_eq!(passes::Counts { reader_validations: 1, ..registered.5 }, unregistered.5);
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0, "every lease ended with its command");
}

// ---------------------------------------------------------------------------
// RT-U4, C1 persistence lifetimes (DEL-04-05 `c1-native-custody-plan.md`
// slice 5): the bytes and hashes every persistence command stores and
// replies with are those of the implementation before the lifetime changes
// (`PARITY`, generated at c8865f384 by this test with
// `SWBPIPE_PRINT_PERSISTENCE_PARITY=1`, before any of them).
// ---------------------------------------------------------------------------

/// Removes the members whose values name this run's scratch paths or
/// registry handles, which differ between runs of the test.
fn without_run_local(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for key in ["database_path", "handle"] {
                map.remove(key);
            }
            map.values_mut().for_each(without_run_local);
        }
        Value::Array(items) => items.iter_mut().for_each(without_run_local),
        _ => {}
    }
}

fn parity_digest(text: &str) -> String {
    let sha: String = Sha256::digest(text.as_bytes()).iter().map(|byte| format!("{byte:02x}")).collect();
    format!("{sha}:{}", text.len())
}

/// Every column of every row of `table`, as `table[row].column = digest`.
fn parity_rows(path: &Path, table: &str, into: &mut std::collections::BTreeMap<String, String>, prefix: &str) {
    for (index, row) in dump(path, table, &[]).iter().enumerate() {
        for (name, value) in row {
            let shown = match value {
                rusqlite::types::Value::Text(text) => parity_digest(text),
                other => format!("{other:?}"),
            };
            into.insert(format!("{prefix} {table}[{index}].{name}"), shown);
        }
    }
}

fn parity_reply(reply: &Value, into: &mut std::collections::BTreeMap<String, String>, key: String) {
    let mut reply = reply.clone();
    without_run_local(&mut reply);
    into.insert(key, parity_digest(&serde_json::to_string(&reply).unwrap()));
}

/// For every persistence case: Record Run twice (a new post-solve snapshot,
/// then the existing state), each recorded run read back and reopened;
/// Create then Save (repeated saves); the saved project reopened and
/// re-saved from its restored handle. Every stored column (record, result,
/// manifest, hashes, project row, labels, states) and every reply is the
/// pre-change implementation's, byte for byte, but the keys store schema v14
/// removed or re-derived ([`REDERIVED_AT_V14`]).
#[test]
fn p1_stored_bytes_hashes_and_replies_are_the_pre_change_implementations() {
    use crate::stored_custody::tests::{native_open, native_read};
    use crate::stored_save::tests::{native_resave, opened_fields, resave_args};
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("parity");
    let mut found = std::collections::BTreeMap::new();
    for case in &cases() {
        let name = case.id.replace(|c: char| !c.is_ascii_alphanumeric(), "-");
        let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
        let recorded = dir.join(format!("{name}-record.sqlite"));
        stored_project(&recorded, case.project_id(), case.model());
        let mut state_id: Option<String> = None;
        let mut recordings = Vec::new();
        for at in 0..2 {
            let reply = native_record(&registry, &recorded, &case.record_args(&job, native_request(state_id.as_deref())), None).unwrap_or_else(|e| panic!("{}: {e}", case.id));
            parity_reply(&reply, &mut found, format!("{} record#{at} reply", case.id));
            state_id = Some(reply["state"]["state_id"].as_str().unwrap().to_string());
            recordings.push(reply["run"]["recording_id"].as_str().unwrap().to_string());
        }
        for table in RUN_TABLES {
            parity_rows(&recorded, table, &mut found, &format!("{} record", case.id));
        }
        let saved = dir.join(format!("{name}-save.sqlite"));
        let created = native_save(&registry, &saved, &case.save_args(&job, "create", Value::Null, Value::Null), &unlimited()).unwrap_or_else(|e| panic!("{}: {e}", case.id));
        parity_reply(&created, &mut found, format!("{} create reply", case.id));
        let base = created["project_envelope_hash"]["value"].clone();
        let again = native_save(&registry, &saved, &case.save_args(&job, "save", base, Value::Null), &unlimited()).unwrap_or_else(|e| panic!("{}: {e}", case.id));
        parity_reply(&again, &mut found, format!("{} save reply", case.id));
        parity_rows(&saved, "local_projects", &mut found, &format!("{} save", case.id));
        no_fts_sidecar(&saved, &case.id);
        registry.dispose_all_results();
        for (at, recording_id) in recordings.iter().enumerate() {
            let read = native_read(&registry, &recorded, case.project_id(), recording_id).unwrap_or_else(|e| panic!("{}: {e}", case.id));
            parity_reply(&read, &mut found, format!("{} recorded read#{at} reply", case.id));
            let legacy = analysis_runs::read_run_in_store(&recorded, case.project_id(), recording_id).unwrap();
            found.insert(format!("{} recorded read#{at} legacy record", case.id), parity_digest(&serde_json::to_string(&legacy.record).unwrap()));
            registry.dispose_all_results();
        }
        let opened = native_open(&registry, &saved, Some(case.project_id())).unwrap();
        parity_reply(&opened, &mut found, format!("{} reopen reply", case.id));
        let base = opened["project_envelope_hash"]["value"].clone();
        let resaved = native_resave(&registry, &saved, &resave_args(&opened, &opened_fields(&opened), "save", base, Value::Null), &unlimited()).unwrap_or_else(|e| panic!("{}: {e}", case.id));
        parity_reply(&resaved, &mut found, format!("{} resave reply", case.id));
        parity_rows(&saved, "local_projects", &mut found, &format!("{} resave", case.id));
        no_fts_sidecar(&saved, &case.id);
        registry.dispose_all_results();
    }
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0, "every lease ended with its command");
    if std::env::var_os("SWBPIPE_PRINT_PERSISTENCE_PARITY").is_some() {
        let mut out = String::from("const PARITY: &[(&str, &str)] = &[\n");
        for (key, value) in &found {
            out.push_str(&format!("    ({key:?}, {value:?}),\n"));
        }
        out.push_str("];\n");
        std::fs::write(std::env::var("SWBPIPE_PRINT_PERSISTENCE_PARITY").unwrap(), out).unwrap();
        return;
    }
    let expected: std::collections::BTreeMap<String, String> = PARITY.iter().map(|(key, value)| (key.to_string(), value.to_string())).collect();
    assert_eq!(found.len(), expected.len(), "the same stored columns and replies");
    // Provenance ([`REDERIVED`]): of the 644 keys generated at c8865f384 with the code before P1
    // (the P1–P3 parity evidence), only these were re-derived: 24 `preview:pressure` keys at
    // 40567c5b7 after #138's input drift, and the same 24 keys of each of the five preview-physics
    // cases on the #122 composition (main 9e5c69797) after its input drift, and those keys again on
    // the S-K1 composition (main 6cb1c3dc3) after S-K1's: a regression pin for the composed code,
    // not pre-change parity.
    // Store schema v14 removed the 42 search-row keys (seven cases, a save
    // and a re-save, three columns) and re-derived [`REDERIVED_AT_V14`].
    let retired = |key: &str| key.contains(" local_project_fts[");
    assert_eq!(expected.len(), 644 - 42, "the keys generated at c8865f384 but the retired search-row keys");
    assert!(!expected.keys().any(|key| retired(key)), "no search-row key is pinned");
    assert_eq!(REDERIVED.len(), 24 + 5 * 24, "the keys re-derived at 40567c5b7 and on the #122 composition");
    assert_eq!(REDERIVED.iter().collect::<std::collections::BTreeSet<_>>().len(), REDERIVED.len(), "each key re-derived once");
    assert!(REDERIVED.iter().all(|key| expected.contains_key(*key) || retired(key)), "only pinned keys were re-derived");
    assert_eq!(REDERIVED.iter().filter(|key| retired(key)).count(), 12, "six cases' save and re-save search texts, now retired");
    // S6 changes only disclosure and its consequent byte/hash/recording/envelope bindings.
    // These are current regression pins; all earlier re-derived groups retain their historical meaning.
    assert_eq!(REDERIVED_AT_S6.len(), 28, "the explicitly enumerated S6 current-regression keys");
    assert_eq!(REDERIVED_AT_S6.iter().collect::<std::collections::BTreeSet<_>>().len(), REDERIVED_AT_S6.len(), "each S6 key re-derived once");
    assert!(REDERIVED_AT_S6.iter().all(|key| expected.contains_key(*key)), "only existing pins re-derived for S6");
    assert_eq!(REDERIVED_AT_RT_L2.len(), 37 + 28, "the explicitly enumerated RT-L2 current-regression keys");
    assert_eq!(REDERIVED_AT_RT_L2.iter().collect::<std::collections::BTreeSet<_>>().len(), REDERIVED_AT_RT_L2.len(), "each RT-L2 key re-derived once");
    assert!(REDERIVED_AT_RT_L2.iter().all(|key| expected.contains_key(*key) && (key.starts_with("preview:pressure ") || key.starts_with("preview:pressure-source "))), "only the two pressure cases' pins re-derived for RT-L2");
    assert_eq!(REDERIVED_AT_V14.len(), 7 * 4, "each case's create, save, reopen and re-save reply");
    assert_eq!(REDERIVED_AT_V14.iter().collect::<std::collections::BTreeSet<_>>().len(), REDERIVED_AT_V14.len(), "each key re-derived once");
    assert!(REDERIVED_AT_V14.iter().all(|key| expected.contains_key(*key) && [" create reply", " save reply", " reopen reply", " resave reply"].iter().any(|end| key.ends_with(end))), "only the project replies were re-derived at v14");
    assert!(REDERIVED[..24].iter().all(|key| key.starts_with("preview:pressure ")), "only preview:pressure keys were re-derived at 40567c5b7");
    let suffixes = |prefix: &str| -> std::collections::BTreeSet<&str> { REDERIVED[..24].iter().map(|key| key.strip_prefix(prefix).unwrap()).collect() };
    let pressure = suffixes("preview:pressure ");
    for case in ["preview:aggregate-failed ", "preview:aggregate-unrecognized ", "preview:connected ", "preview:retained ", "preview:rule-pack-excluded "] {
        let keys: std::collections::BTreeSet<&str> = REDERIVED[24..].iter().filter_map(|key| key.strip_prefix(case)).collect();
        assert_eq!(keys, pressure, "{case}: the same 24 keys re-derived on the #122 composition");
    }
    for (key, value) in &expected {
        assert_eq!(found.get(key), Some(value), "{key}");
    }
}

/// The peak requested bytes (`ingress::counting`, this thread's Rust
/// allocations; SQLite's own heap is not counted) of each persistence
/// command for `case`, with the result's and record's stored lengths. The
/// commands run once unmeasured first ([`warm_up`]), so a process-wide lazy
/// initialization is never charged to the command measured.
fn command_peaks(case: &Case, registry: &SolveJobRegistry, gate: &Gate, dir: &Path) -> std::collections::BTreeMap<&'static str, u64> {
    let mut peaks = std::collections::BTreeMap::new();
    for pass in ["warm-up", "measured"] {
        persistence_sequence(case, registry, gate, dir, pass, &mut |name, peak| {
            if pass == "measured" {
                peaks.insert(name, peak);
            }
        });
    }
    peaks
}

/// Each persistence command once on fresh stores of `case`, unmeasured: the
/// process-wide lazy state the commands initialize on first use (tables,
/// readers) then exists before a measurement, whichever thread made it.
pub(crate) fn warm_up(case: &Case, registry: &SolveJobRegistry, gate: &Gate, dir: &Path) {
    persistence_sequence(case, registry, gate, dir, "warm-up", &mut |_, _| {});
}

/// Record Run, Create, Save again, the recorded-run read and the restored
/// re-save of `case`, each measured, its peak handed to `peak`; and `R`, `Q`.
fn persistence_sequence(case: &Case, registry: &SolveJobRegistry, gate: &Gate, dir: &Path, pass: &str, peak: &mut dyn FnMut(&'static str, u64)) {
    use crate::ingress::counting::measure;
    use crate::stored_custody::tests::{native_open, native_read};
    use crate::stored_save::tests::{native_resave, opened_fields, resave_args};
    let name = format!("{}-{pass}", case.id.replace(|c: char| !c.is_ascii_alphanumeric(), "-"));
    let job = held(registry, gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
    let recorded = dir.join(format!("{name}-peaks-record.sqlite"));
    stored_project(&recorded, case.project_id(), case.model());
    let args = case.record_args(&job, native_request(None));
    let (reply, measured) = measure(|| native_record(registry, &recorded, &args, None).unwrap());
    peak("record_run", measured.peak_requested);
    let recording_id = reply["run"]["recording_id"].as_str().unwrap().to_string();
    let saved = dir.join(format!("{name}-peaks-save.sqlite"));
    let args = case.save_args(&job, "create", Value::Null, Value::Null);
    let (created, measured) = measure(|| native_save(registry, &saved, &args, &unlimited()).unwrap());
    peak("fresh_create", measured.peak_requested);
    let args = case.save_args(&job, "save", created["project_envelope_hash"]["value"].clone(), Value::Null);
    let (_, measured) = measure(|| native_save(registry, &saved, &args, &unlimited()).unwrap());
    peak("fresh_save_again", measured.peak_requested);
    registry.dispose_all_results();
    let (_, measured) = measure(|| native_read(registry, &recorded, case.project_id(), &recording_id).unwrap());
    peak("recorded_read", measured.peak_requested);
    registry.dispose_all_results();
    let opened = native_open(registry, &saved, Some(case.project_id())).unwrap();
    let args = resave_args(&opened, &opened_fields(&opened), "save", opened["project_envelope_hash"]["value"].clone(), Value::Null);
    let (_, measured) = measure(|| native_resave(registry, &saved, &args, &unlimited()).unwrap());
    peak("restored_resave", measured.peak_requested);
    registry.dispose_all_results();
    let stored = dump(&recorded, "analysis_runs", &[]);
    let length = |column: &str| match &stored[0].iter().find(|(name, _)| name == column).unwrap().1 {
        rusqlite::types::Value::Text(text) => text.len() as u64,
        other => panic!("{other:?}"),
    };
    peak("R", length("result_json"));
    peak("Q", length("run_record_json"));
}

/// Prints each persistence command's peak (the allocation evidence's
/// reference values, `SWBPIPE_PRINT_PERSISTENCE_PEAKS=1`).
#[test]
fn print_persistence_command_peaks() {
    if std::env::var_os("SWBPIPE_PRINT_PERSISTENCE_PEAKS").is_none() {
        return;
    }
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("peaks");
    let mut out = String::new();
    for case in &cases() {
        out.push_str(&format!("{} {:?}\n", case.id, command_peaks(case, &registry, &gate, &dir)));
    }
    std::fs::remove_dir_all(&dir).unwrap();
    std::fs::write(std::env::var("SWBPIPE_PRINT_PERSISTENCE_PEAKS").unwrap(), out).unwrap();
}

/// Record Run in this thread with `hooks` (the tamper and failure tests).
fn native_record_with(registry: &SolveJobRegistry, path: &Path, args: &Value, hooks: &RunWriteHooks<'_>) -> Result<Value, String> {
    let (raw, lease) = reserve(registry, &body(args), &headers(Some(INGRESS_ARGS_JSON_V1)), RECORD_COMMAND).map_err(text_of)?;
    let path = path.to_path_buf();
    run_record(registry, move || Ok(path), raw, lease, hooks, None).map(|reply| serde_json::from_str(reply.text()).unwrap()).map_err(text_of)
}

/// Every table a Record Run can write, and whether the store's run
/// immutability trigger is in place.
pub(crate) fn store_snapshot(path: &Path) -> (Vec<Rows>, bool) {
    let tables = RUN_TABLES.iter().chain(["local_projects"].iter()).map(|table| dump(path, table, &[])).collect();
    let trigger = Connection::open(path).unwrap().query_row("SELECT count(*) FROM sqlite_master WHERE type = 'trigger' AND name = 'analysis_runs_immutable'", [], |row| row.get::<_, i64>(0)).unwrap() == 1;
    (tables, trigger)
}

/// SQL run between the run row and its labels (the `run` hook), with the
/// run's immutability trigger dropped and foreign keys deferred, as a
/// corruption between the INSERT and the readback.
pub(crate) const TAMPER: &[(&str, &str, &str)] = &[
    ("state_id", "UPDATE analysis_runs SET state_id = replace(state_id, '0', '1')", "the row's state id differs from its verified record"),
    ("producer_run_id", "UPDATE analysis_runs SET producer_run_id = producer_run_id || 'x'", "the row's producer run id differs from its verified record"),
    ("created_at", "UPDATE analysis_runs SET created_at = replace(created_at, 'Z', 'z')", "the row's created_at differs from its verified record"),
    ("run_record_hash", "UPDATE analysis_runs SET run_record_hash = upper(run_record_hash)", "the row's record hash differs from its verified record"),
    ("result_hash", "UPDATE analysis_runs SET result_hash = upper(result_hash)", "the row's result hash differs from its verified record"),
    ("recording_id", "UPDATE analysis_runs SET recording_id = recording_id || 'x'", "was not written"),
    ("project_id", "UPDATE analysis_runs SET project_id = project_id || 'x'", "was not written"),
    ("record bytes (a member)", "UPDATE analysis_runs SET run_record_json = replace(run_record_json, '\"schema_version\":\"0.3.0\"', '\"schema_version\":\"0.3.1\"')", "the row's record hash differs from its verified record"),
    ("record bytes (not JSON)", "UPDATE analysis_runs SET run_record_json = run_record_json || 'x'", "the stored record: the stored JSON is invalid (trailing characters at byte"),
    ("labels row", "CREATE TEMP TRIGGER drop_run_labels AFTER INSERT ON analysis_run_labels BEGIN DELETE FROM analysis_run_labels WHERE recording_id = NEW.recording_id; END", "has no labels row"),
];

/// Each tamper of TAMPER after the INSERT, and a failure between any two
/// rows, is refused by the readback with its integrity text, and nothing of
/// the run, its labels or its snapshot is written (the trigger is back).
pub(crate) fn tampered_runs_roll_back(name: &str) {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch(name);
    let case = case_named("preview:pressure");
    let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
    let args = case.record_args(&job, native_request(None));
    let reference = dir.join("reference.sqlite");
    stored_project(&reference, case.project_id(), case.model());
    let recording_id = native_record(&registry, &reference, &args, None).unwrap()["run"]["recording_id"].as_str().unwrap().to_string();
    let held_bytes = registry.pool().reserved_total();
    for (at, (column, sql, expected)) in TAMPER.iter().enumerate() {
        let path = dir.join(format!("tamper-{at}.sqlite"));
        stored_project(&path, case.project_id(), case.model());
        let before = store_snapshot(&path);
        let tamper = |transaction: &Transaction<'_>, stage: &str| -> Result<(), String> {
            if stage == "run" {
                transaction.execute_batch(&format!("PRAGMA defer_foreign_keys = ON; DROP TRIGGER analysis_runs_immutable; {sql};")).map_err(|error| error.to_string())?;
            }
            Ok(())
        };
        let hooks = RunWriteHooks { now: &frozen_now, state_token: &frozen_token, between_rows: &tamper };
        let text = native_record_with(&registry, &path, &args, &hooks).expect_err(column);
        assert!(text.starts_with(&format!("ANALYSIS-RUN-INTEGRITY: recorded run `{recording_id}`")), "{column}: {text}");
        assert!(text.contains(expected), "{column}: {text}");
        assert_eq!(store_snapshot(&path), before, "{column}: every row rolled back, the trigger restored");
        assert_eq!(registry.pool().reserved_total(), held_bytes, "{column}: every lease of the command ended");
    }
    for stage in ["state", "run"] {
        let path = dir.join(format!("fail-{stage}.sqlite"));
        stored_project(&path, case.project_id(), case.model());
        let before = store_snapshot(&path);
        let fail = |_: &Transaction<'_>, at: &str| -> Result<(), String> { if at == stage { Err(format!("INVENTED-FAILURE: between the {at} rows")) } else { Ok(()) } };
        let hooks = RunWriteHooks { now: &frozen_now, state_token: &frozen_token, between_rows: &fail };
        assert_eq!(native_record_with(&registry, &path, &args, &hooks).unwrap_err(), format!("INVENTED-FAILURE: between the {stage} rows"));
        assert_eq!(store_snapshot(&path), before, "{stage}: nothing written");
    }
    assert!(registry.dispose_result(&job).unwrap().released);
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

#[test]
fn p1_a_run_tampered_after_its_insert_is_refused_and_rolled_back() {
    tampered_runs_roll_back("p1-tamper");
}

/// The allocation evidence of one command by phase: each mark's live bytes,
/// the phase's peak and the pool's reserved total, and the command's peak.
pub(crate) fn phases(registry: &SolveJobRegistry, run: impl FnOnce()) -> (Vec<stored_read::phase_probe::Mark>, u64) {
    let pool = registry.clone();
    stored_read::phase_probe::start(move || pool.pool().reserved_total());
    let ((), measured) = crate::ingress::counting::measure(run);
    (stored_read::phase_probe::take(), measured.peak_requested)
}

pub(crate) fn mark<'m>(marks: &'m [stored_read::phase_probe::Mark], at: &str) -> &'m stored_read::phase_probe::Mark {
    marks.iter().find(|mark| mark.at == at).unwrap_or_else(|| panic!("no mark `{at}` in {marks:?}"))
}

/// Writes the marks as a table (the PR's allocation evidence,
/// `SWBPIPE_PRINT_PERSISTENCE_PHASES=<file>`).
pub(crate) fn print_phases(title: &str, marks: &[stored_read::phase_probe::Mark], peak: u64) {
    if let Some(path) = std::env::var_os("SWBPIPE_PRINT_PERSISTENCE_PHASES") {
        use std::io::Write;
        let mut out = std::fs::OpenOptions::new().create(true).append(true).open(path).unwrap();
        writeln!(out, "{title} (command peak {peak} B)").unwrap();
        for mark in marks {
            writeln!(out, "  {:<62} live {:>10}  phase peak {:>10}  pool reserved {:>12}", mark.at, mark.live, mark.peak, mark.reserved).unwrap();
        }
    }
}

/// Each command's peak requested bytes before the change (this thread's Rust
/// allocations, after [`warm_up`]; `command_peaks` of this file run on
/// c8865f384's production code): the `preview:pressure` case.
///
/// Three of them carry the growth of that case's input since they were
/// measured, added block by block and not re-measured, because c8865f384's code
/// cannot read the present input: the case's result gained the pressure
/// elongation statement and its per-case label, and its model gained the
/// `model_settings` member. At each command's peak the present code holds, over
/// the same code on the earlier input: five new strings in a parse of the
/// result (309 B), one copy of the result text (324 B), one copy of the
/// manifest text (91 B) and one parsed copy of the settings member per parsed
/// model (1,465 B). Create and Save again hold all of these with the member
/// parsed twice (+3,654 B); the restored re-save holds the strings, the text
/// and one parsed member (+2,098 B). The meaning is unchanged: no more than
/// c8865f384's code would hold, as far as reading that code establishes which
/// copies it held at its peak. Each peak also holds the store's path, so it
/// moves by a few bytes with the temporary directory's length and the process
/// id's digit count.
const PRE_CHANGE_PEAKS: [(&str, u64); 5] =
    [("record_run", 10_446_522), ("recorded_read", 9_597_918), ("fresh_create", 1_802_331), ("fresh_save_again", 1_801_533), ("restored_resave", 4_216_501)];

/// P1's allocation evidence (`ingress::counting`: allocations, not leases):
/// the result's parse ends before its text is written; the run's texts end at
/// the INSERT; the readback frees the loaded record before hashing it and
/// makes no projection clone, serialized text or checked parse; a save's
/// result and record texts end once the project row is written (it builds no
/// search text since store schema v14); a recorded read loads the record once
/// and parses the result once. Each command's peak is lower than before.
#[test]
fn p1_allocations_end_where_their_last_use_is() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("p1-phases");
    let case = case_named("preview:pressure");
    warm_up(&case, &registry, &gate, &dir);
    let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
    let recorded = dir.join("record.sqlite");
    stored_project(&recorded, case.project_id(), case.model());
    let args = case.record_args(&job, native_request(None));
    let mut reply = Value::Null;
    let (marks, record_peak) = phases(&registry, || reply = native_record(&registry, &recorded, &args, None).unwrap());
    print_phases("Record Run (preview:pressure)", &marks, record_peak);
    let stored = dump(&recorded, "analysis_runs", &[]);
    let length = |column: &str| match &stored[0].iter().find(|(name, _)| name == column).unwrap().1 {
        rusqlite::types::Value::Text(text) => text.len() as i64,
        other => panic!("{other:?}"),
    };
    let (r, q) = (length("result_json"), length("run_record_json"));
    let live = |at: &str| mark(&marks, at).live;
    // (b) The parse (larger than its text) ends before the result's text exists.
    assert!(live("record run: record staged, result parse held") - live("record run: result parse released") > r, "{marks:?}");
    assert!(live("record run: result text written") - live("record run: result parse released") >= r, "the result's text is allocated after the parse ended");
    // (c) Both texts end at the INSERT.
    assert!(live("record run: run row inserted") - live("record run: texts and statement copies released") >= r + q, "{marks:?}");
    // (c) Record Run's readback runs after both texts ended (its record checks
    // are below, on a reader's readback, and P2's in `p2_*`).
    let at = |name: &str| marks.iter().position(|mark| mark.at == name).unwrap_or_else(|| panic!("no mark `{name}`: {marks:?}"));
    assert!(at("record run: texts and statement copies released") < at("record run: run read back and verified"), "{marks:?}");
    let pre = |name: &str| PRE_CHANGE_PEAKS.iter().find(|(key, _)| *key == name).unwrap().1;
    assert!(record_peak + 2 * q as u64 <= pre("record_run"), "Record Run: {record_peak} against {}", pre("record_run"));

    let saved = dir.join("save.sqlite");
    let args = case.save_args(&job, "create", Value::Null, Value::Null);
    let (marks, create_peak) = phases(&registry, || reply = native_save(&registry, &saved, &args, &unlimited()).unwrap());
    print_phases("Create Project (preview:pressure)", &marks, create_peak);
    let stored = dump(&saved, "local_projects", &[]);
    let column = |name: &str| match &stored[0].iter().find(|(column, _)| column == name).unwrap().1 {
        rusqlite::types::Value::Text(text) => text.len() as i64,
        other => panic!("{other:?}"),
    };
    let (saved_r, saved_q) = (column("mechanics_result_json"), column("analysis_run_json"));
    let live = |at: &str| mark(&marks, at).live;
    // (a) The result's and the record's texts end once the project row is
    // written, and no search text is built from them.
    assert!(live("save: project row written") - live("save: result and record texts released") >= saved_r + saved_q, "{marks:?}");
    no_fts_sidecar(&saved, "create");
    assert!(create_peak <= pre("fresh_create"), "Create Project: {create_peak} against {}", pre("fresh_create"));
    let base = reply["project_envelope_hash"]["value"].clone();
    let args = case.save_args(&job, "save", base, Value::Null);
    let (marks, again_peak) = phases(&registry, || reply = native_save(&registry, &saved, &args, &unlimited()).unwrap());
    print_phases("Save Project again (preview:pressure)", &marks, again_peak);
    assert!(again_peak <= pre("fresh_save_again"), "Save Project: {again_peak} against {}", pre("fresh_save_again"));

    registry.dispose_all_results();
    let recording_id = dump(&recorded, "analysis_runs", &[])[0].iter().find(|(name, _)| name == "recording_id").map(|(_, value)| match value {
        rusqlite::types::Value::Text(text) => text.clone(),
        other => panic!("{other:?}"),
    }).unwrap();
    // (c) A reader's readback (the webview's recorded-run read, every list and
    // read verifies a run so) frees the loaded record (Q) once it is parsed,
    // before hashing: its phase peak (text, parse and SQLite's handover) is at
    // least Q above what stays; the hash and the checks then allocate nothing
    // of the record's size (no clone, serialized text, checked parse or
    // canonical text).
    let (marks, legacy_peak) = phases(&registry, || {
        analysis_runs::read_run_in_store_with(&recorded, case.project_id(), &recording_id, &stored_read::UnmeteredReadMemory).unwrap().into_unreserved().unwrap();
    });
    print_phases("A reader's readback: read_analysis_run (preview:pressure)", &marks, legacy_peak);
    let parsed = mark(&marks, "run readback: record parsed, text released");
    assert!(parsed.peak - parsed.live >= q, "{marks:?}");
    assert!(mark(&marks, "run readback: record verified").peak - parsed.live < q / 4, "{marks:?}");
    let (marks, read_peak) = phases(&registry, || reply = crate::stored_custody::tests::native_read(&registry, &recorded, case.project_id(), &recording_id).unwrap());
    print_phases("Recorded-run read (preview:pressure)", &marks, read_peak);
    // (d) One load of the record: the summary's readback is the only one.
    assert_eq!(marks.iter().filter(|mark| mark.at.starts_with("run readback: record parsed")).count(), 1, "{marks:?}");
    mark(&marks, "run readback: record parsed, text kept for custody");
    // The read paths peak at the instant the canonical text buffer doubles inside the result hash
    // check, so where that doubling falls in the text can move these peaks by a few hundred bytes
    // (a serialiser temporary and a level of sorted keys) on a change that copies nothing more.
    assert!(read_peak + q as u64 <= pre("recorded_read"), "recorded read: {read_peak} against {}", pre("recorded_read"));
    registry.dispose_all_results();

    let opened = crate::stored_custody::tests::native_open(&registry, &saved, Some(case.project_id())).unwrap();
    let base = opened["project_envelope_hash"]["value"].clone();
    let args = crate::stored_save::tests::resave_args(&opened, &crate::stored_save::tests::opened_fields(&opened), "save", base, Value::Null);
    let (marks, resave_peak) = phases(&registry, || reply = crate::stored_save::tests::native_resave(&registry, &saved, &args, &unlimited()).unwrap());
    print_phases("Restored re-save (preview:pressure)", &marks, resave_peak);
    assert!(live_drop(&marks, "save: project row written", "save: result and record texts released") >= saved_r, "{marks:?}");
    assert!(resave_peak <= pre("restored_resave"), "re-save: {resave_peak} against {}", pre("restored_resave"));
    registry.dispose_all_results();
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

fn live_drop(marks: &[stored_read::phase_probe::Mark], from: &str, to: &str) -> i64 {
    mark(marks, from).live - mark(marks, to).live
}

/// The Record Run tamper set for P2: P1's (`TAMPER`, every refusal text
/// unchanged), the stored record's bytes changed without changing its parse
/// (which only P2 refuses), and stored as a blob of the same bytes.
const P2_TAMPER: &[(&str, &str, &str)] = &[
    ("record bytes (whitespace)", "UPDATE analysis_runs SET run_record_json = ' ' || run_record_json", "the stored record is not the record written"),
    ("record bytes (a blob)", "UPDATE analysis_runs SET run_record_json = CAST(run_record_json AS BLOB)", "recorded run"),
];

#[test]
fn p2_a_run_tampered_after_its_insert_is_refused_and_rolled_back() {
    // Every P1 tamper: the same refusal texts, every row rolled back.
    tampered_runs_roll_back("p2-tamper");
    // Bytes the readback's parse would accept are still not the bytes written.
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("p2-tamper-bytes");
    let case = case_named("preview:pressure");
    let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
    let args = case.record_args(&job, native_request(None));
    for (at, (column, sql, expected)) in P2_TAMPER.iter().enumerate() {
        let path = dir.join(format!("tamper-{at}.sqlite"));
        stored_project(&path, case.project_id(), case.model());
        let before = store_snapshot(&path);
        let tamper = |transaction: &Transaction<'_>, stage: &str| -> Result<(), String> {
            if stage == "run" {
                transaction.execute_batch(&format!("PRAGMA defer_foreign_keys = ON; DROP TRIGGER analysis_runs_immutable; {sql};")).map_err(|error| error.to_string())?;
            }
            Ok(())
        };
        let hooks = RunWriteHooks { now: &frozen_now, state_token: &frozen_token, between_rows: &tamper };
        let text = native_record_with(&registry, &path, &args, &hooks).expect_err(column);
        assert!(text.starts_with("ANALYSIS-RUN-INTEGRITY: recorded run `") && text.contains(expected), "{column}: {text}");
        assert_eq!(store_snapshot(&path), before, "{column}: every row rolled back");
    }
    assert!(registry.dispose_result(&job).unwrap().released);
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// Review of #133 (blocking finding 1): stored bytes that differ from the
/// staged record get P2's new refusal only when the old readback accepts them
/// in full. With a malformed label list as well (an after-insert labels
/// trigger), the old readback's own label refusal stands, exactly as with the
/// label tamper alone, and every row is rolled back.
#[test]
fn p2_changed_bytes_with_a_malformed_label_list_keep_the_old_label_refusal() {
    const BAD_TAGS: &str = "CREATE TEMP TRIGGER bad_run_tags AFTER INSERT ON analysis_run_labels BEGIN UPDATE analysis_run_labels SET tags_json = '{}' WHERE recording_id = NEW.recording_id; END";
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("p2-bytes-and-labels");
    let case = case_named("preview:pressure");
    let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
    let args = case.record_args(&job, native_request(None));
    let held_bytes = registry.pool().reserved_total();
    let refused = |name: &str, sql: String| -> String {
        let path = dir.join(format!("{name}.sqlite"));
        stored_project(&path, case.project_id(), case.model());
        let before = store_snapshot(&path);
        let tamper = |transaction: &Transaction<'_>, stage: &str| -> Result<(), String> {
            if stage == "run" {
                transaction.execute_batch(&format!("PRAGMA defer_foreign_keys = ON; DROP TRIGGER analysis_runs_immutable; {sql};")).map_err(|error| error.to_string())?;
            }
            Ok(())
        };
        let hooks = RunWriteHooks { now: &frozen_now, state_token: &frozen_token, between_rows: &tamper };
        let text = native_record_with(&registry, &path, &args, &hooks).expect_err(name);
        assert_eq!(store_snapshot(&path), before, "{name}: every row rolled back, the trigger restored");
        assert_eq!(registry.pool().reserved_total(), held_bytes, "{name}: every lease of the command ended");
        text
    };
    let labels_only = refused("labels-only", BAD_TAGS.to_string());
    let combined = refused("bytes-and-labels", format!("UPDATE analysis_runs SET run_record_json = ' ' || run_record_json; {BAD_TAGS}"));
    assert!(labels_only.starts_with("ANALYSIS-RUN-INTEGRITY: ") && labels_only.contains("label list is malformed"), "{labels_only}");
    assert!(!labels_only.contains("not the record written"), "{labels_only}");
    assert_eq!(combined, labels_only, "the old label refusal precedes P2's byte refusal");
    // The whitespace alone, which the old readback accepts, is P2's refusal.
    let bytes_only = refused("bytes-only", "UPDATE analysis_runs SET run_record_json = ' ' || run_record_json".to_string());
    assert!(bytes_only.contains("the stored record is not the record written"), "{bytes_only}");
    assert!(registry.dispose_result(&job).unwrap().released);
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// The texts, request and plan inputs of `case`'s run as a native Record Run
/// stored them in `reference` (the direct writer tests).
fn stored_run_texts(reference: &Path) -> std::collections::BTreeMap<String, String> {
    dump(reference, "analysis_runs", &[])[0]
        .iter()
        .filter_map(|(name, value)| match value {
            rusqlite::types::Value::Text(text) => Some((name.clone(), text.clone())),
            _ => None,
        })
        .collect()
}

/// `write_native_run` on a fresh store of `case`, with `record_json` and
/// `record_hash` as given and `finish`'s outcome; the store's tables after
/// the transaction ended (committed only when the writer succeeded).
fn direct_write(dir: &Path, name: &str, case: &Case, stored: &std::collections::BTreeMap<String, String>, record_json: String, record_hash: &str, finish: impl FnOnce() -> Result<(), String>) -> (Result<(), String>, (Vec<Rows>, bool), (Vec<Rows>, bool)) {
    let memory = stored_read::UnmeteredReadMemory;
    let payload_canonical = open_pipe_stress_canonical_json::canonical_json(&case.manifest["model_basis"]["model_payload"]);
    let checked = analysis_runs::NativeChecked { payload_hash: format!("sha256:{}", open_pipe_stress_operation_applier::sha256_hex(&payload_canonical)), payload_canonical };
    let mut request = decode_request(native_request(None));
    request.project_id = case.project_id().to_string();
    let path = dir.join(format!("{name}.sqlite"));
    stored_project(&path, case.project_id(), case.model());
    let before = store_snapshot(&path);
    let mut connection = stored_read::open_store(&path).unwrap();
    let transaction = stored_read::write_transaction(&mut connection).unwrap();
    let plan = analysis_runs::plan_native_run(&transaction, &request, &checked, &FROZEN, &memory).unwrap();
    let texts = NativeRunTexts {
        record_json,
        record_hash,
        result_json: stored["result_json"].clone(),
        result_hash: &stored["result_hash"],
        manifest_text: &stored["input_manifest_json"],
        producer_run_id: &stored["producer_run_id"],
        leases: Vec::new(),
    };
    let outcome = analysis_runs::write_native_run(&transaction, &request, plan, texts, &FROZEN, &memory, |_| finish());
    if outcome.is_ok() {
        transaction.commit().unwrap();
    } else {
        drop(transaction);
    }
    drop(connection);
    (outcome, before, store_snapshot(&path))
}

/// P2's transaction: a record whose staged checks fail is refused before
/// anything is written, with the readback's own text; a finish (the reply's
/// serialization) that refuses after the rows rolls every row back; a
/// staged text the streamed pass leaves to the full readback (members out of
/// order) is read back as before and stored as given.
#[test]
fn p2_staged_checks_refuse_before_writing_and_every_refusal_rolls_back() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("p2-direct");
    let case = case_named("preview:pressure");
    let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
    let reference = dir.join("reference.sqlite");
    stored_project(&reference, case.project_id(), case.model());
    let reply = native_record(&registry, &reference, &case.record_args(&job, native_request(None)), None).unwrap();
    let recording_id = reply["run"]["recording_id"].as_str().unwrap().to_string();
    let stored = stored_run_texts(&reference);
    let record = stored["run_record_json"].clone();
    // As written by the command: the same rows.
    let (outcome, _, _) = direct_write(&dir, "as-written", &case, &stored, record.clone(), &stored["run_record_hash"], || Ok(()));
    outcome.unwrap();
    for table in RUN_TABLES {
        same_rows(&reference, &dir.join("as-written.sqlite"), table, &[], "as written");
    }
    // A staged record whose hash column would differ: refused before writing.
    let upper = stored["run_record_hash"].to_uppercase();
    let (outcome, before, after) = direct_write(&dir, "staged-refused", &case, &stored, record.clone(), &upper, || panic!("the reply is never made"));
    let text = outcome.unwrap_err();
    let upper_id = analysis_runs::recording_id_of(&upper);
    assert_eq!(text, format!("ANALYSIS-RUN-INTEGRITY: recorded run `{upper_id}`: the row's record hash differs from its verified record; it is listed as unreadable and is not read."));
    assert_eq!(before, after, "nothing written");
    // A finish that refuses after every row: all rolled back.
    let (outcome, before, after) = direct_write(&dir, "finish-refused", &case, &stored, record.clone(), &stored["run_record_hash"], || Err("INVENTED-FINISH-REFUSAL".to_string()));
    assert_eq!(outcome.unwrap_err(), "INVENTED-FINISH-REFUSAL");
    assert_eq!(before, after, "the snapshot, the run and the labels rolled back");
    // Members out of order: the full readback, as before P2, accepts the same record.
    let value: Value = serde_json::from_str(&record).unwrap();
    let reversed = format!("{{{}}}", value.as_object().unwrap().iter().rev().map(|(key, item)| format!("{}:{}", serde_json::to_string(key).unwrap(), serde_json::to_string(item).unwrap())).collect::<Vec<_>>().join(","));
    assert_ne!(reversed, record);
    let (outcome, _, _) = direct_write(&dir, "reordered", &case, &stored, reversed.clone(), &stored["run_record_hash"], || Ok(()));
    outcome.unwrap();
    assert_eq!(stored_run_texts(&dir.join("reordered.sqlite"))["run_record_json"], reversed, "stored as given");
    assert_eq!(stored_run_texts(&dir.join("reordered.sqlite"))["recording_id"], recording_id);
    assert!(registry.dispose_result(&job).unwrap().released);
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// P2's allocation evidence: Record Run's readback loads no record, builds no
/// parse of it and reserves nothing of its size; the staged pass before the
/// INSERT builds no record tree. The command's peak is its build stage.
#[test]
fn p2_record_runs_readback_loads_and_parses_no_record() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("p2-phases");
    let case = case_named("preview:pressure");
    warm_up(&case, &registry, &gate, &dir);
    let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
    let path = dir.join("record.sqlite");
    stored_project(&path, case.project_id(), case.model());
    let args = case.record_args(&job, native_request(None));
    let (marks, peak) = phases(&registry, || {
        native_record(&registry, &path, &args, None).unwrap();
    });
    print_phases("Record Run after P2 (preview:pressure)", &marks, peak);
    let q = stored_run_texts(&path)["run_record_json"].len() as i64;
    assert!(marks.iter().all(|mark| !mark.at.starts_with("run readback: record parsed")), "the record is never loaded or parsed: {marks:?}");
    let staged = mark(&marks, "record run: staged record checked");
    let written = mark(&marks, "record run: result text written");
    assert!(staged.peak - written.live < q / 4, "the staged pass builds no record tree: {marks:?}");
    let released = mark(&marks, "record run: texts and statement copies released");
    let verified = mark(&marks, "record run: run read back and verified");
    mark(&marks, "run readback: stored bytes are the staged record's");
    assert!(verified.peak - released.live < q / 4, "the readback allocates nothing of the record's size: {marks:?}");
    // What the readback reserves is the read's fixed overhead, the labels and
    // identity columns and the status (P1's readback reserved twice the
    // record's parse, about 26 times Q here).
    assert!(verified.reserved - released.reserved < stored_read::READ_OVERHEAD_BYTES + (q as u64) / 4, "nor reserves it: {marks:?}");
    // The command's peak is its build stage (the held result's parse and the
    // record staged from it), no longer its readback.
    assert_eq!(peak as i64, marks.iter().map(|mark| mark.peak).max().unwrap(), "{marks:?}");
    assert_eq!(peak as i64, mark(&marks, "record run: record staged, result parse held").peak, "{marks:?}");
    assert!(registry.dispose_result(&job).unwrap().released);
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}
/// The [`PARITY`] keys re-derived after the pre-change generation, with the
/// same writer (`SWBPIPE_PRINT_PERSISTENCE_PARITY`): the original 644 were
/// generated at c8865f384 with the code before P1 (the P1–P3 parity evidence
/// stays there); these 24 `preview:pressure` keys were re-derived at
/// 40567c5b7 (the C1 custody branch composed with main 06e90a89f) after #138's
/// input drift (its builder-corpus record binds the current pressure-2 table,
/// so the TS-built record and the native one, equal byte for byte, both moved).
/// The next 120, the same 24 keys of each of the five preview-physics cases
/// (`preview:aggregate-failed`, `preview:aggregate-unrecognized`,
/// `preview:connected`, `preview:retained`, `preview:rule-pack-excluded`), were
/// re-derived on the C1 custody branch composed with main 9e5c69797 (#122, RT-A5
/// `minuend_final`) after its input drift: #122 restated the preview-physics
/// tables (f41b9d3e… -> 563db3c7…, b4377e73… -> d0bab350…), and those cases'
/// builder-corpus records bind the current tables, so the TS-built record and the
/// native one (equal byte for byte) moved only by the table digest and the record
/// hashes over it; `preview:pressure` and `preview:pressure-source` did not move.
/// The same keys were re-derived again on S-K1 composed with main 6cb1c3dc3, after its input
/// drift: S-K1 (R-6/R-7) restated the same three tables (563db3c7… -> 16eeb49c…, d0bab350… ->
/// ef59ef33…, f64473f5… -> 8a0abaea…); every value kept its stored length. Of S-K1's 120 keys,
/// the 10 search-row keys (`<case> save|resave local_project_fts[0].model_text`) are retired by
/// v14; 20 are also in [`REDERIVED_AT_V14`] (`<case> create|save|reopen|resave reply` of the five
/// preview-physics cases), each re-derived once on the composed code, and each composed reply
/// with the v13 summary members substituted (`store_schema_version` and its target 13,
/// `migration_status` `…v13…`, no `store-v14-retire-project-fts-sidecar` migration,
/// `fts_indexed: true`) reproduces its S-K1 digest of 8dee927fb exactly: the sum of the two
/// drifts. The other 90 equal S-K1's values and every other key equals main's.
/// They are a regression pin for the composed code, not pre-change parity.
const REDERIVED: &[&str] = &[
    "preview:pressure create reply",
    "preview:pressure record analysis_run_labels[0].recording_id",
    "preview:pressure record analysis_run_labels[1].recording_id",
    "preview:pressure record analysis_runs[0].recording_id",
    "preview:pressure record analysis_runs[0].run_record_hash",
    "preview:pressure record analysis_runs[0].run_record_json",
    "preview:pressure record analysis_runs[1].recording_id",
    "preview:pressure record analysis_runs[1].run_record_hash",
    "preview:pressure record analysis_runs[1].run_record_json",
    "preview:pressure record#0 reply",
    "preview:pressure record#1 reply",
    "preview:pressure recorded read#0 legacy record",
    "preview:pressure recorded read#0 reply",
    "preview:pressure recorded read#1 legacy record",
    "preview:pressure recorded read#1 reply",
    "preview:pressure reopen reply",
    "preview:pressure resave local_project_fts[0].model_text",
    "preview:pressure resave local_projects[0].analysis_run_json",
    "preview:pressure resave local_projects[0].project_envelope_hash_json",
    "preview:pressure resave reply",
    "preview:pressure save local_project_fts[0].model_text",
    "preview:pressure save local_projects[0].analysis_run_json",
    "preview:pressure save local_projects[0].project_envelope_hash_json",
    "preview:pressure save reply",
    // The #122 composition (main 9e5c69797, RT-A5 minuend_final): the five preview-physics cases.
    "preview:aggregate-failed create reply",
    "preview:aggregate-failed record analysis_run_labels[0].recording_id",
    "preview:aggregate-failed record analysis_run_labels[1].recording_id",
    "preview:aggregate-failed record analysis_runs[0].recording_id",
    "preview:aggregate-failed record analysis_runs[0].run_record_hash",
    "preview:aggregate-failed record analysis_runs[0].run_record_json",
    "preview:aggregate-failed record analysis_runs[1].recording_id",
    "preview:aggregate-failed record analysis_runs[1].run_record_hash",
    "preview:aggregate-failed record analysis_runs[1].run_record_json",
    "preview:aggregate-failed record#0 reply",
    "preview:aggregate-failed record#1 reply",
    "preview:aggregate-failed recorded read#0 legacy record",
    "preview:aggregate-failed recorded read#0 reply",
    "preview:aggregate-failed recorded read#1 legacy record",
    "preview:aggregate-failed recorded read#1 reply",
    "preview:aggregate-failed reopen reply",
    "preview:aggregate-failed resave local_project_fts[0].model_text",
    "preview:aggregate-failed resave local_projects[0].analysis_run_json",
    "preview:aggregate-failed resave local_projects[0].project_envelope_hash_json",
    "preview:aggregate-failed resave reply",
    "preview:aggregate-failed save local_project_fts[0].model_text",
    "preview:aggregate-failed save local_projects[0].analysis_run_json",
    "preview:aggregate-failed save local_projects[0].project_envelope_hash_json",
    "preview:aggregate-failed save reply",
    "preview:aggregate-unrecognized create reply",
    "preview:aggregate-unrecognized record analysis_run_labels[0].recording_id",
    "preview:aggregate-unrecognized record analysis_run_labels[1].recording_id",
    "preview:aggregate-unrecognized record analysis_runs[0].recording_id",
    "preview:aggregate-unrecognized record analysis_runs[0].run_record_hash",
    "preview:aggregate-unrecognized record analysis_runs[0].run_record_json",
    "preview:aggregate-unrecognized record analysis_runs[1].recording_id",
    "preview:aggregate-unrecognized record analysis_runs[1].run_record_hash",
    "preview:aggregate-unrecognized record analysis_runs[1].run_record_json",
    "preview:aggregate-unrecognized record#0 reply",
    "preview:aggregate-unrecognized record#1 reply",
    "preview:aggregate-unrecognized recorded read#0 legacy record",
    "preview:aggregate-unrecognized recorded read#0 reply",
    "preview:aggregate-unrecognized recorded read#1 legacy record",
    "preview:aggregate-unrecognized recorded read#1 reply",
    "preview:aggregate-unrecognized reopen reply",
    "preview:aggregate-unrecognized resave local_project_fts[0].model_text",
    "preview:aggregate-unrecognized resave local_projects[0].analysis_run_json",
    "preview:aggregate-unrecognized resave local_projects[0].project_envelope_hash_json",
    "preview:aggregate-unrecognized resave reply",
    "preview:aggregate-unrecognized save local_project_fts[0].model_text",
    "preview:aggregate-unrecognized save local_projects[0].analysis_run_json",
    "preview:aggregate-unrecognized save local_projects[0].project_envelope_hash_json",
    "preview:aggregate-unrecognized save reply",
    "preview:connected create reply",
    "preview:connected record analysis_run_labels[0].recording_id",
    "preview:connected record analysis_run_labels[1].recording_id",
    "preview:connected record analysis_runs[0].recording_id",
    "preview:connected record analysis_runs[0].run_record_hash",
    "preview:connected record analysis_runs[0].run_record_json",
    "preview:connected record analysis_runs[1].recording_id",
    "preview:connected record analysis_runs[1].run_record_hash",
    "preview:connected record analysis_runs[1].run_record_json",
    "preview:connected record#0 reply",
    "preview:connected record#1 reply",
    "preview:connected recorded read#0 legacy record",
    "preview:connected recorded read#0 reply",
    "preview:connected recorded read#1 legacy record",
    "preview:connected recorded read#1 reply",
    "preview:connected reopen reply",
    "preview:connected resave local_project_fts[0].model_text",
    "preview:connected resave local_projects[0].analysis_run_json",
    "preview:connected resave local_projects[0].project_envelope_hash_json",
    "preview:connected resave reply",
    "preview:connected save local_project_fts[0].model_text",
    "preview:connected save local_projects[0].analysis_run_json",
    "preview:connected save local_projects[0].project_envelope_hash_json",
    "preview:connected save reply",
    "preview:retained create reply",
    "preview:retained record analysis_run_labels[0].recording_id",
    "preview:retained record analysis_run_labels[1].recording_id",
    "preview:retained record analysis_runs[0].recording_id",
    "preview:retained record analysis_runs[0].run_record_hash",
    "preview:retained record analysis_runs[0].run_record_json",
    "preview:retained record analysis_runs[1].recording_id",
    "preview:retained record analysis_runs[1].run_record_hash",
    "preview:retained record analysis_runs[1].run_record_json",
    "preview:retained record#0 reply",
    "preview:retained record#1 reply",
    "preview:retained recorded read#0 legacy record",
    "preview:retained recorded read#0 reply",
    "preview:retained recorded read#1 legacy record",
    "preview:retained recorded read#1 reply",
    "preview:retained reopen reply",
    "preview:retained resave local_project_fts[0].model_text",
    "preview:retained resave local_projects[0].analysis_run_json",
    "preview:retained resave local_projects[0].project_envelope_hash_json",
    "preview:retained resave reply",
    "preview:retained save local_project_fts[0].model_text",
    "preview:retained save local_projects[0].analysis_run_json",
    "preview:retained save local_projects[0].project_envelope_hash_json",
    "preview:retained save reply",
    "preview:rule-pack-excluded create reply",
    "preview:rule-pack-excluded record analysis_run_labels[0].recording_id",
    "preview:rule-pack-excluded record analysis_run_labels[1].recording_id",
    "preview:rule-pack-excluded record analysis_runs[0].recording_id",
    "preview:rule-pack-excluded record analysis_runs[0].run_record_hash",
    "preview:rule-pack-excluded record analysis_runs[0].run_record_json",
    "preview:rule-pack-excluded record analysis_runs[1].recording_id",
    "preview:rule-pack-excluded record analysis_runs[1].run_record_hash",
    "preview:rule-pack-excluded record analysis_runs[1].run_record_json",
    "preview:rule-pack-excluded record#0 reply",
    "preview:rule-pack-excluded record#1 reply",
    "preview:rule-pack-excluded recorded read#0 legacy record",
    "preview:rule-pack-excluded recorded read#0 reply",
    "preview:rule-pack-excluded recorded read#1 legacy record",
    "preview:rule-pack-excluded recorded read#1 reply",
    "preview:rule-pack-excluded reopen reply",
    "preview:rule-pack-excluded resave local_project_fts[0].model_text",
    "preview:rule-pack-excluded resave local_projects[0].analysis_run_json",
    "preview:rule-pack-excluded resave local_projects[0].project_envelope_hash_json",
    "preview:rule-pack-excluded resave reply",
    "preview:rule-pack-excluded save local_project_fts[0].model_text",
    "preview:rule-pack-excluded save local_projects[0].analysis_run_json",
    "preview:rule-pack-excluded save local_projects[0].project_envelope_hash_json",
    "preview:rule-pack-excluded save reply",
];
/// The keys store schema v14 (RT-U4, the FTS sidecar retired) re-derived:
/// each persistence case's create, save, reopen and re-save reply, whose
/// project summary now reports store schema v14 (`store_schema_version`,
/// `store_schema_target_version`, `migration_status`,
/// `migrations_applied_on_open`) and `fts_indexed: false`. Each was checked to
/// be its pinned reply but for those members before it was re-derived. No
/// stored column's key moved. The 42 search-row keys (`<case> save|resave
/// local_project_fts[0].project_id|project_name|model_text`, twelve of them in
/// [`REDERIVED`]) were removed: v14 writes no search row.
const REDERIVED_AT_V14: &[&str] = &[
    "preview:aggregate-failed create reply",
    "preview:aggregate-failed reopen reply",
    "preview:aggregate-failed resave reply",
    "preview:aggregate-failed save reply",
    "preview:aggregate-unrecognized create reply",
    "preview:aggregate-unrecognized reopen reply",
    "preview:aggregate-unrecognized resave reply",
    "preview:aggregate-unrecognized save reply",
    "preview:connected create reply",
    "preview:connected reopen reply",
    "preview:connected resave reply",
    "preview:connected save reply",
    "preview:pressure create reply",
    "preview:pressure reopen reply",
    "preview:pressure resave reply",
    "preview:pressure save reply",
    "preview:pressure-source create reply",
    "preview:pressure-source reopen reply",
    "preview:pressure-source resave reply",
    "preview:pressure-source save reply",
    "preview:retained create reply",
    "preview:retained reopen reply",
    "preview:retained resave reply",
    "preview:retained save reply",
    "preview:rule-pack-excluded create reply",
    "preview:rule-pack-excluded reopen reply",
    "preview:rule-pack-excluded resave reply",
    "preview:rule-pack-excluded save reply",
];

/// Current regression bindings for S6's authorized disclosure restatement, re-derived with the
/// existing full-map writer. Exactly these 28 keys move; the other 574 values and all membership stay fixed.
/// Earlier REDERIVED/REDERIVED_AT_V14 declarations and frozen pre-change evidence remain unchanged.
const REDERIVED_AT_S6: &[&str] = &[
    "preview:pressure create reply",
    "preview:pressure record analysis_run_labels[0].recording_id",
    "preview:pressure record analysis_run_labels[1].recording_id",
    "preview:pressure record analysis_runs[0].recording_id",
    "preview:pressure record analysis_runs[0].result_hash",
    "preview:pressure record analysis_runs[0].result_json",
    "preview:pressure record analysis_runs[0].run_record_hash",
    "preview:pressure record analysis_runs[0].run_record_json",
    "preview:pressure record analysis_runs[1].recording_id",
    "preview:pressure record analysis_runs[1].result_hash",
    "preview:pressure record analysis_runs[1].result_json",
    "preview:pressure record analysis_runs[1].run_record_hash",
    "preview:pressure record analysis_runs[1].run_record_json",
    "preview:pressure record#0 reply",
    "preview:pressure record#1 reply",
    "preview:pressure recorded read#0 legacy record",
    "preview:pressure recorded read#0 reply",
    "preview:pressure recorded read#1 legacy record",
    "preview:pressure recorded read#1 reply",
    "preview:pressure reopen reply",
    "preview:pressure resave local_projects[0].analysis_run_json",
    "preview:pressure resave local_projects[0].mechanics_result_json",
    "preview:pressure resave local_projects[0].project_envelope_hash_json",
    "preview:pressure resave reply",
    "preview:pressure save local_projects[0].analysis_run_json",
    "preview:pressure save local_projects[0].mechanics_result_json",
    "preview:pressure save local_projects[0].project_envelope_hash_json",
    "preview:pressure save reply",
];

/// Current regression bindings for RT-L2's pressure elongation (DEL-04-07), re-derived with the
/// existing full-map writer: the `preview:pressure` case's model names its pressurised case and its
/// result is pressure-3 with the inclusion statement and per-case label; the `preview:pressure-source`
/// case's result states that no case includes pressure elongation, with the publication and receipt
/// hashes binding that statement. Exactly these 65 keys move (no number in the stored results moved);
/// the other 537 values and all membership stay fixed. Earlier groups keep their meaning.
const REDERIVED_AT_RT_L2: &[&str] = &[
    "preview:pressure create reply",
    "preview:pressure record analysis_run_labels[0].recording_id",
    "preview:pressure record analysis_run_labels[1].recording_id",
    "preview:pressure record analysis_runs[0].input_manifest_json",
    "preview:pressure record analysis_runs[0].recording_id",
    "preview:pressure record analysis_runs[0].result_hash",
    "preview:pressure record analysis_runs[0].result_json",
    "preview:pressure record analysis_runs[0].run_record_hash",
    "preview:pressure record analysis_runs[0].run_record_json",
    "preview:pressure record analysis_runs[1].input_manifest_json",
    "preview:pressure record analysis_runs[1].recording_id",
    "preview:pressure record analysis_runs[1].result_hash",
    "preview:pressure record analysis_runs[1].result_json",
    "preview:pressure record analysis_runs[1].run_record_hash",
    "preview:pressure record analysis_runs[1].run_record_json",
    "preview:pressure record model_states[0].model_canonical",
    "preview:pressure record model_states[0].model_hash",
    "preview:pressure record model_states[0].record_json",
    "preview:pressure record#0 reply",
    "preview:pressure record#1 reply",
    "preview:pressure recorded read#0 legacy record",
    "preview:pressure recorded read#0 reply",
    "preview:pressure recorded read#1 legacy record",
    "preview:pressure recorded read#1 reply",
    "preview:pressure reopen reply",
    "preview:pressure resave local_projects[0].analysis_run_json",
    "preview:pressure resave local_projects[0].mechanics_result_json",
    "preview:pressure resave local_projects[0].model_hash_json",
    "preview:pressure resave local_projects[0].model_json",
    "preview:pressure resave local_projects[0].project_envelope_hash_json",
    "preview:pressure resave reply",
    "preview:pressure save local_projects[0].analysis_run_json",
    "preview:pressure save local_projects[0].mechanics_result_json",
    "preview:pressure save local_projects[0].model_hash_json",
    "preview:pressure save local_projects[0].model_json",
    "preview:pressure save local_projects[0].project_envelope_hash_json",
    "preview:pressure save reply",
    "preview:pressure-source create reply",
    "preview:pressure-source record analysis_run_labels[0].recording_id",
    "preview:pressure-source record analysis_run_labels[1].recording_id",
    "preview:pressure-source record analysis_runs[0].recording_id",
    "preview:pressure-source record analysis_runs[0].result_hash",
    "preview:pressure-source record analysis_runs[0].result_json",
    "preview:pressure-source record analysis_runs[0].run_record_hash",
    "preview:pressure-source record analysis_runs[0].run_record_json",
    "preview:pressure-source record analysis_runs[1].recording_id",
    "preview:pressure-source record analysis_runs[1].result_hash",
    "preview:pressure-source record analysis_runs[1].result_json",
    "preview:pressure-source record analysis_runs[1].run_record_hash",
    "preview:pressure-source record analysis_runs[1].run_record_json",
    "preview:pressure-source record#0 reply",
    "preview:pressure-source record#1 reply",
    "preview:pressure-source recorded read#0 legacy record",
    "preview:pressure-source recorded read#0 reply",
    "preview:pressure-source recorded read#1 legacy record",
    "preview:pressure-source recorded read#1 reply",
    "preview:pressure-source reopen reply",
    "preview:pressure-source resave local_projects[0].analysis_run_json",
    "preview:pressure-source resave local_projects[0].mechanics_result_json",
    "preview:pressure-source resave local_projects[0].project_envelope_hash_json",
    "preview:pressure-source resave reply",
    "preview:pressure-source save local_projects[0].analysis_run_json",
    "preview:pressure-source save local_projects[0].mechanics_result_json",
    "preview:pressure-source save local_projects[0].project_envelope_hash_json",
    "preview:pressure-source save reply",
];

/// Generated at c8865f384 (module section docs), except the [`REDERIVED`], [`REDERIVED_AT_S6`] and
/// [`REDERIVED_AT_RT_L2`] keys. The 42
/// search-row keys are removed and the [`REDERIVED_AT_V14`] keys re-derived
/// (store schema v14).
const PARITY: &[(&str, &str)] = &[
    ("preview:aggregate-failed create reply", "a24d9545fc005835b0649d7d89c0900d1f49119847e70cc9190f8a8564424a30:3906"),
    ("preview:aggregate-failed record analysis_run_labels[0].assumptions_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:aggregate-failed record analysis_run_labels[0].notes_json", "5df6d4f94dea81443474cd8fd8b70333654763c9dfbea861d2428c19e7d00a73:18"),
    ("preview:aggregate-failed record analysis_run_labels[0].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:aggregate-failed record analysis_run_labels[0].recording_id", "ca02a75d550071abc68589a39a61c872a064a36c1a8264aded7dab5f88a41ff8:39"),
    ("preview:aggregate-failed record analysis_run_labels[0].run_name", "a89023ff6219b358cf8bbb555afdc6d5f4303df5bfe187d902c3622679b8092f:22"),
    ("preview:aggregate-failed record analysis_run_labels[0].tags_json", "c38f1b165048a14cf0612830edb0f61f56be4cb4f1f3d01fbaa14e6655c52c55:10"),
    ("preview:aggregate-failed record analysis_run_labels[0].updated_at", "04199ee21ddec3cb0080ba7ae86be72bbf19e53d97bc7b59ff3f343c25431d0e:24"),
    ("preview:aggregate-failed record analysis_run_labels[1].assumptions_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:aggregate-failed record analysis_run_labels[1].notes_json", "5df6d4f94dea81443474cd8fd8b70333654763c9dfbea861d2428c19e7d00a73:18"),
    ("preview:aggregate-failed record analysis_run_labels[1].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:aggregate-failed record analysis_run_labels[1].recording_id", "3306841e2e3ab25af65a3f8ba8ebbd72383d71a60df7eb3db26c192e38ec21b8:39"),
    ("preview:aggregate-failed record analysis_run_labels[1].run_name", "a89023ff6219b358cf8bbb555afdc6d5f4303df5bfe187d902c3622679b8092f:22"),
    ("preview:aggregate-failed record analysis_run_labels[1].tags_json", "c38f1b165048a14cf0612830edb0f61f56be4cb4f1f3d01fbaa14e6655c52c55:10"),
    ("preview:aggregate-failed record analysis_run_labels[1].updated_at", "1ac8477c538da6e93db66aa8df1bdf6f18a246ad755914704dcf06733e26b2fc:24"),
    ("preview:aggregate-failed record analysis_runs[0].created_at", "04199ee21ddec3cb0080ba7ae86be72bbf19e53d97bc7b59ff3f343c25431d0e:24"),
    ("preview:aggregate-failed record analysis_runs[0].input_manifest_json", "ec2b590838e26dbff151fe097131cd7a9ace800cb409041dbdf68a25a3e2c81e:6295"),
    ("preview:aggregate-failed record analysis_runs[0].producer_run_id", "b3862e6694684d4c24986f49ec8e862f3872f1ae70359b7f55b3a8f340507b70:29"),
    ("preview:aggregate-failed record analysis_runs[0].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:aggregate-failed record analysis_runs[0].recording_id", "ca02a75d550071abc68589a39a61c872a064a36c1a8264aded7dab5f88a41ff8:39"),
    ("preview:aggregate-failed record analysis_runs[0].result_hash", "a8a20c6b217680871c9035cf81cfbcbbb6d84b058908d3f7ee90afdfbc4b360b:64"),
    ("preview:aggregate-failed record analysis_runs[0].result_json", "76cb429be9d4557ca4b08467853a5d0f054cc5152438ecc7b13917621c680431:71477"),
    ("preview:aggregate-failed record analysis_runs[0].run_record_hash", "622635fc737e226f8d23dc91bdc98cb3c141fd87170ca7dd81f0de4cbdf0a55a:64"),
    ("preview:aggregate-failed record analysis_runs[0].run_record_json", "7efff91bdd69c793da25435c5c4287c4a1ca3052eeb7c4e010c7380e569054af:143577"),
    ("preview:aggregate-failed record analysis_runs[0].state_id", "3a3cd6490a1651ab2f9fd668c3dc0d5628c6d0120efb67da96e265d81bb849d8:38"),
    ("preview:aggregate-failed record analysis_runs[1].created_at", "1ac8477c538da6e93db66aa8df1bdf6f18a246ad755914704dcf06733e26b2fc:24"),
    ("preview:aggregate-failed record analysis_runs[1].input_manifest_json", "ec2b590838e26dbff151fe097131cd7a9ace800cb409041dbdf68a25a3e2c81e:6295"),
    ("preview:aggregate-failed record analysis_runs[1].producer_run_id", "b3862e6694684d4c24986f49ec8e862f3872f1ae70359b7f55b3a8f340507b70:29"),
    ("preview:aggregate-failed record analysis_runs[1].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:aggregate-failed record analysis_runs[1].recording_id", "3306841e2e3ab25af65a3f8ba8ebbd72383d71a60df7eb3db26c192e38ec21b8:39"),
    ("preview:aggregate-failed record analysis_runs[1].result_hash", "a8a20c6b217680871c9035cf81cfbcbbb6d84b058908d3f7ee90afdfbc4b360b:64"),
    ("preview:aggregate-failed record analysis_runs[1].result_json", "76cb429be9d4557ca4b08467853a5d0f054cc5152438ecc7b13917621c680431:71477"),
    ("preview:aggregate-failed record analysis_runs[1].run_record_hash", "cdd203602625cecadbf88b930e771315f2b148eae4490944d56bf1e9e86cc36c:64"),
    ("preview:aggregate-failed record analysis_runs[1].run_record_json", "b09032db92f5ab4b295b0cfa8782710463d411f685af91ef24e246d08fb1b40e:143577"),
    ("preview:aggregate-failed record analysis_runs[1].state_id", "3a3cd6490a1651ab2f9fd668c3dc0d5628c6d0120efb67da96e265d81bb849d8:38"),
    ("preview:aggregate-failed record model_state_labels[0].assumptions_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:aggregate-failed record model_state_labels[0].notes_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:aggregate-failed record model_state_labels[0].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:aggregate-failed record model_state_labels[0].state_id", "3a3cd6490a1651ab2f9fd668c3dc0d5628c6d0120efb67da96e265d81bb849d8:38"),
    ("preview:aggregate-failed record model_state_labels[0].state_name", "d081e834436746542ed6bc8d1327bda6c85dea255b2af1f44313fa8fe075de15:11"),
    ("preview:aggregate-failed record model_state_labels[0].tags_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:aggregate-failed record model_state_labels[0].updated_at", "04199ee21ddec3cb0080ba7ae86be72bbf19e53d97bc7b59ff3f343c25431d0e:24"),
    ("preview:aggregate-failed record model_states[0].created_at", "04199ee21ddec3cb0080ba7ae86be72bbf19e53d97bc7b59ff3f343c25431d0e:24"),
    ("preview:aggregate-failed record model_states[0].model_canonical", "4a1babd4a449831aeeea1835dfff057ebae6c91c14675443fde9da061f07ff3c:3584"),
    ("preview:aggregate-failed record model_states[0].model_hash", "b357d5f9e868c425b7b4c4798a8b0ac2c88c46635d806460cbf996d3d1fce416:71"),
    ("preview:aggregate-failed record model_states[0].parent_state_id", "Null"),
    ("preview:aggregate-failed record model_states[0].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:aggregate-failed record model_states[0].record_hash", "c3a5e9ad1fa5ff45041703986f7f5a9f7f88d4c84ff79c9e0628734ad4042f5f:64"),
    ("preview:aggregate-failed record model_states[0].record_json", "471bc60490823530e98857c539d997e949385a9864c5b6206998cc6e4c18f3d7:1718"),
    ("preview:aggregate-failed record model_states[0].state_id", "3a3cd6490a1651ab2f9fd668c3dc0d5628c6d0120efb67da96e265d81bb849d8:38"),
    ("preview:aggregate-failed record model_states[0].state_kind", "37e47861edf4c99a70c0e2513fc6d386a62547cb0a25f064d10785d204386944:19"),
    ("preview:aggregate-failed record#0 reply", "abbc7ca7b1df4a9d0d944c083346cf7025ae1bc9b11e47ca928e343c24088ff7:1539"),
    ("preview:aggregate-failed record#1 reply", "17930fb4e5211b96ad44d2b5303ca1e6cd815e62e36aab6b69da96029fca6b9f:1524"),
    ("preview:aggregate-failed recorded read#0 legacy record", "7efff91bdd69c793da25435c5c4287c4a1ca3052eeb7c4e010c7380e569054af:143577"),
    ("preview:aggregate-failed recorded read#0 reply", "38a3dd170933b01945c5a55e6702bde12a4ff92f5c5fcecf0f2526b49087132c:13071"),
    ("preview:aggregate-failed recorded read#1 legacy record", "b09032db92f5ab4b295b0cfa8782710463d411f685af91ef24e246d08fb1b40e:143577"),
    ("preview:aggregate-failed recorded read#1 reply", "9725c4d34b8c637bc9db95888a7e57dab4f274a5392c91cf4658167338dfeb36:13071"),
    ("preview:aggregate-failed reopen reply", "c6540781df533525b71b96143df65fb964bbf85d9a0d3af9e6877b95e5ecb58a:11449"),
    ("preview:aggregate-failed resave local_projects[0].analysis_run_json", "e18b2af008b5aa2cb15b83dba9e1fbaa98a9eaedc6be783d42f61488e7c4c13b:143591"),
    ("preview:aggregate-failed resave local_projects[0].created_at_unix", "Integer(1793000000)"),
    ("preview:aggregate-failed resave local_projects[0].editor_intents_json", "f21abe4b59cfb3b91e8df7df4f65f7f324e44423901aaf53e2fbfa1a7160642f:125"),
    ("preview:aggregate-failed resave local_projects[0].mechanics_result_json", "5613c1d834bc0125003ddfdafe753ffeed4b065fa1ae09409aa8b468a3b2eea9:71477"),
    ("preview:aggregate-failed resave local_projects[0].model_hash_json", "1e64c7d4d580e54c4b39dfc7592ed08ae5b50ff23277b7063bcbf6b0df7427ba:260"),
    ("preview:aggregate-failed resave local_projects[0].model_json", "4a1babd4a449831aeeea1835dfff057ebae6c91c14675443fde9da061f07ff3c:3584"),
    ("preview:aggregate-failed resave local_projects[0].model_migration_ledger_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:aggregate-failed resave local_projects[0].project_envelope_hash_json", "b631e24e1b38ba7b701c4de79b9516dad64aaec421ef6ad964a654cc2808ed10:341"),
    ("preview:aggregate-failed resave local_projects[0].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:aggregate-failed resave local_projects[0].project_name", "65a4ab50988845fb64d0c0005c5b1af7eadee7b8c9b6935eb905722cc8a07773:45"),
    ("preview:aggregate-failed resave local_projects[0].proposal_json", "0516c8de6da19a643e9e9bf7b789bc81dc099beab278ed3c82cffba1900ced6d:82"),
    ("preview:aggregate-failed resave local_projects[0].selected_review_target_json", "b0b571f28e5724d9095fea8cdc00559629f998bc458f1f7f0225e302ef2e23c8:69"),
    ("preview:aggregate-failed resave local_projects[0].updated_at_unix", "Integer(1793000000)"),
    ("preview:aggregate-failed resave reply", "26d587a773504651a4e466afc69a41ec33f10cdfcc59305f447686f14d6ee292:4240"),
    ("preview:aggregate-failed save local_projects[0].analysis_run_json", "e18b2af008b5aa2cb15b83dba9e1fbaa98a9eaedc6be783d42f61488e7c4c13b:143591"),
    ("preview:aggregate-failed save local_projects[0].created_at_unix", "Integer(1793000000)"),
    ("preview:aggregate-failed save local_projects[0].editor_intents_json", "f21abe4b59cfb3b91e8df7df4f65f7f324e44423901aaf53e2fbfa1a7160642f:125"),
    ("preview:aggregate-failed save local_projects[0].mechanics_result_json", "5613c1d834bc0125003ddfdafe753ffeed4b065fa1ae09409aa8b468a3b2eea9:71477"),
    ("preview:aggregate-failed save local_projects[0].model_hash_json", "1e64c7d4d580e54c4b39dfc7592ed08ae5b50ff23277b7063bcbf6b0df7427ba:260"),
    ("preview:aggregate-failed save local_projects[0].model_json", "4a1babd4a449831aeeea1835dfff057ebae6c91c14675443fde9da061f07ff3c:3584"),
    ("preview:aggregate-failed save local_projects[0].model_migration_ledger_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:aggregate-failed save local_projects[0].project_envelope_hash_json", "b631e24e1b38ba7b701c4de79b9516dad64aaec421ef6ad964a654cc2808ed10:341"),
    ("preview:aggregate-failed save local_projects[0].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:aggregate-failed save local_projects[0].project_name", "65a4ab50988845fb64d0c0005c5b1af7eadee7b8c9b6935eb905722cc8a07773:45"),
    ("preview:aggregate-failed save local_projects[0].proposal_json", "0516c8de6da19a643e9e9bf7b789bc81dc099beab278ed3c82cffba1900ced6d:82"),
    ("preview:aggregate-failed save local_projects[0].selected_review_target_json", "b0b571f28e5724d9095fea8cdc00559629f998bc458f1f7f0225e302ef2e23c8:69"),
    ("preview:aggregate-failed save local_projects[0].updated_at_unix", "Integer(1793000000)"),
    ("preview:aggregate-failed save reply", "d3752909cb3ac1320a597a87ef72e73c15a5abd675c1cc5991adee60655f6b6d:3415"),
    ("preview:aggregate-unrecognized create reply", "12db8b02591081112d8ffafc2d8425666d686ad4d0b1fc0ea72fa5ce8ad183ad:3906"),
    ("preview:aggregate-unrecognized record analysis_run_labels[0].assumptions_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:aggregate-unrecognized record analysis_run_labels[0].notes_json", "5df6d4f94dea81443474cd8fd8b70333654763c9dfbea861d2428c19e7d00a73:18"),
    ("preview:aggregate-unrecognized record analysis_run_labels[0].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:aggregate-unrecognized record analysis_run_labels[0].recording_id", "a5471337cd07ec9d84281e972c47a1ab29097957eeb9bb3782ea4f25db882c16:39"),
    ("preview:aggregate-unrecognized record analysis_run_labels[0].run_name", "a89023ff6219b358cf8bbb555afdc6d5f4303df5bfe187d902c3622679b8092f:22"),
    ("preview:aggregate-unrecognized record analysis_run_labels[0].tags_json", "c38f1b165048a14cf0612830edb0f61f56be4cb4f1f3d01fbaa14e6655c52c55:10"),
    ("preview:aggregate-unrecognized record analysis_run_labels[0].updated_at", "04199ee21ddec3cb0080ba7ae86be72bbf19e53d97bc7b59ff3f343c25431d0e:24"),
    ("preview:aggregate-unrecognized record analysis_run_labels[1].assumptions_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:aggregate-unrecognized record analysis_run_labels[1].notes_json", "5df6d4f94dea81443474cd8fd8b70333654763c9dfbea861d2428c19e7d00a73:18"),
    ("preview:aggregate-unrecognized record analysis_run_labels[1].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:aggregate-unrecognized record analysis_run_labels[1].recording_id", "9684dc7198476df569377bd06f91cb63181459f1288df6e9f8f56e5eac58509e:39"),
    ("preview:aggregate-unrecognized record analysis_run_labels[1].run_name", "a89023ff6219b358cf8bbb555afdc6d5f4303df5bfe187d902c3622679b8092f:22"),
    ("preview:aggregate-unrecognized record analysis_run_labels[1].tags_json", "c38f1b165048a14cf0612830edb0f61f56be4cb4f1f3d01fbaa14e6655c52c55:10"),
    ("preview:aggregate-unrecognized record analysis_run_labels[1].updated_at", "1ac8477c538da6e93db66aa8df1bdf6f18a246ad755914704dcf06733e26b2fc:24"),
    ("preview:aggregate-unrecognized record analysis_runs[0].created_at", "04199ee21ddec3cb0080ba7ae86be72bbf19e53d97bc7b59ff3f343c25431d0e:24"),
    ("preview:aggregate-unrecognized record analysis_runs[0].input_manifest_json", "ec2b590838e26dbff151fe097131cd7a9ace800cb409041dbdf68a25a3e2c81e:6295"),
    ("preview:aggregate-unrecognized record analysis_runs[0].producer_run_id", "b3862e6694684d4c24986f49ec8e862f3872f1ae70359b7f55b3a8f340507b70:29"),
    ("preview:aggregate-unrecognized record analysis_runs[0].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:aggregate-unrecognized record analysis_runs[0].recording_id", "a5471337cd07ec9d84281e972c47a1ab29097957eeb9bb3782ea4f25db882c16:39"),
    ("preview:aggregate-unrecognized record analysis_runs[0].result_hash", "a8a20c6b217680871c9035cf81cfbcbbb6d84b058908d3f7ee90afdfbc4b360b:64"),
    ("preview:aggregate-unrecognized record analysis_runs[0].result_json", "76cb429be9d4557ca4b08467853a5d0f054cc5152438ecc7b13917621c680431:71477"),
    ("preview:aggregate-unrecognized record analysis_runs[0].run_record_hash", "d70d159d960f09291475032f48d8e30d067c1ecd660e7b93eef66b2cfd6572fa:64"),
    ("preview:aggregate-unrecognized record analysis_runs[0].run_record_json", "3c5dd680b6956e7c20924824f3d1ff81f95b0377a40ed44b1fc6cc6a5fc77a6d:143583"),
    ("preview:aggregate-unrecognized record analysis_runs[0].state_id", "3a3cd6490a1651ab2f9fd668c3dc0d5628c6d0120efb67da96e265d81bb849d8:38"),
    ("preview:aggregate-unrecognized record analysis_runs[1].created_at", "1ac8477c538da6e93db66aa8df1bdf6f18a246ad755914704dcf06733e26b2fc:24"),
    ("preview:aggregate-unrecognized record analysis_runs[1].input_manifest_json", "ec2b590838e26dbff151fe097131cd7a9ace800cb409041dbdf68a25a3e2c81e:6295"),
    ("preview:aggregate-unrecognized record analysis_runs[1].producer_run_id", "b3862e6694684d4c24986f49ec8e862f3872f1ae70359b7f55b3a8f340507b70:29"),
    ("preview:aggregate-unrecognized record analysis_runs[1].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:aggregate-unrecognized record analysis_runs[1].recording_id", "9684dc7198476df569377bd06f91cb63181459f1288df6e9f8f56e5eac58509e:39"),
    ("preview:aggregate-unrecognized record analysis_runs[1].result_hash", "a8a20c6b217680871c9035cf81cfbcbbb6d84b058908d3f7ee90afdfbc4b360b:64"),
    ("preview:aggregate-unrecognized record analysis_runs[1].result_json", "76cb429be9d4557ca4b08467853a5d0f054cc5152438ecc7b13917621c680431:71477"),
    ("preview:aggregate-unrecognized record analysis_runs[1].run_record_hash", "9a6e3eed6f4a7d859b68b7c74443197be5246d384d0cb6e9c4bbb166cdf3a6fc:64"),
    ("preview:aggregate-unrecognized record analysis_runs[1].run_record_json", "4c261b31ce5b5805c12083aab1c2268172864f0a1016cee577b550b64c20f862:143583"),
    ("preview:aggregate-unrecognized record analysis_runs[1].state_id", "3a3cd6490a1651ab2f9fd668c3dc0d5628c6d0120efb67da96e265d81bb849d8:38"),
    ("preview:aggregate-unrecognized record model_state_labels[0].assumptions_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:aggregate-unrecognized record model_state_labels[0].notes_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:aggregate-unrecognized record model_state_labels[0].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:aggregate-unrecognized record model_state_labels[0].state_id", "3a3cd6490a1651ab2f9fd668c3dc0d5628c6d0120efb67da96e265d81bb849d8:38"),
    ("preview:aggregate-unrecognized record model_state_labels[0].state_name", "d081e834436746542ed6bc8d1327bda6c85dea255b2af1f44313fa8fe075de15:11"),
    ("preview:aggregate-unrecognized record model_state_labels[0].tags_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:aggregate-unrecognized record model_state_labels[0].updated_at", "04199ee21ddec3cb0080ba7ae86be72bbf19e53d97bc7b59ff3f343c25431d0e:24"),
    ("preview:aggregate-unrecognized record model_states[0].created_at", "04199ee21ddec3cb0080ba7ae86be72bbf19e53d97bc7b59ff3f343c25431d0e:24"),
    ("preview:aggregate-unrecognized record model_states[0].model_canonical", "4a1babd4a449831aeeea1835dfff057ebae6c91c14675443fde9da061f07ff3c:3584"),
    ("preview:aggregate-unrecognized record model_states[0].model_hash", "b357d5f9e868c425b7b4c4798a8b0ac2c88c46635d806460cbf996d3d1fce416:71"),
    ("preview:aggregate-unrecognized record model_states[0].parent_state_id", "Null"),
    ("preview:aggregate-unrecognized record model_states[0].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:aggregate-unrecognized record model_states[0].record_hash", "c3a5e9ad1fa5ff45041703986f7f5a9f7f88d4c84ff79c9e0628734ad4042f5f:64"),
    ("preview:aggregate-unrecognized record model_states[0].record_json", "471bc60490823530e98857c539d997e949385a9864c5b6206998cc6e4c18f3d7:1718"),
    ("preview:aggregate-unrecognized record model_states[0].state_id", "3a3cd6490a1651ab2f9fd668c3dc0d5628c6d0120efb67da96e265d81bb849d8:38"),
    ("preview:aggregate-unrecognized record model_states[0].state_kind", "37e47861edf4c99a70c0e2513fc6d386a62547cb0a25f064d10785d204386944:19"),
    ("preview:aggregate-unrecognized record#0 reply", "591329160eb2a5601b185269a2c389e30c0ff645692c43e3bd0bbf4c5ba25563:1545"),
    ("preview:aggregate-unrecognized record#1 reply", "275e37023f3f4ca83c43fbf1f9ab79bcd15cbf91ce5dd1936ed69be9a0f09684:1530"),
    ("preview:aggregate-unrecognized recorded read#0 legacy record", "3c5dd680b6956e7c20924824f3d1ff81f95b0377a40ed44b1fc6cc6a5fc77a6d:143583"),
    ("preview:aggregate-unrecognized recorded read#0 reply", "1ea29575e6f1e7837afb702f3334a6bc0e85b927a5b41c2faf619ac6e1042906:13083"),
    ("preview:aggregate-unrecognized recorded read#1 legacy record", "4c261b31ce5b5805c12083aab1c2268172864f0a1016cee577b550b64c20f862:143583"),
    ("preview:aggregate-unrecognized recorded read#1 reply", "9e22d5dc79944ff500bc0e92a380ffd3ea338510207b1567a9fc31a8c88c1bb6:13083"),
    ("preview:aggregate-unrecognized reopen reply", "d4a1d76c830ced4f214d0034c35cd2f99a68cb8fa7a1155ce8377949568e0f64:11455"),
    ("preview:aggregate-unrecognized resave local_projects[0].analysis_run_json", "e028b18561b9a976e443c9b619de0da6dda6b9d31f015a8811da7c2827b215ee:143597"),
    ("preview:aggregate-unrecognized resave local_projects[0].created_at_unix", "Integer(1793000000)"),
    ("preview:aggregate-unrecognized resave local_projects[0].editor_intents_json", "f21abe4b59cfb3b91e8df7df4f65f7f324e44423901aaf53e2fbfa1a7160642f:125"),
    ("preview:aggregate-unrecognized resave local_projects[0].mechanics_result_json", "5613c1d834bc0125003ddfdafe753ffeed4b065fa1ae09409aa8b468a3b2eea9:71477"),
    ("preview:aggregate-unrecognized resave local_projects[0].model_hash_json", "1e64c7d4d580e54c4b39dfc7592ed08ae5b50ff23277b7063bcbf6b0df7427ba:260"),
    ("preview:aggregate-unrecognized resave local_projects[0].model_json", "4a1babd4a449831aeeea1835dfff057ebae6c91c14675443fde9da061f07ff3c:3584"),
    ("preview:aggregate-unrecognized resave local_projects[0].model_migration_ledger_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:aggregate-unrecognized resave local_projects[0].project_envelope_hash_json", "4024bec19546138d9ce34446241f9bfbe8a47190908092e0b7500053b6762846:341"),
    ("preview:aggregate-unrecognized resave local_projects[0].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:aggregate-unrecognized resave local_projects[0].project_name", "65a4ab50988845fb64d0c0005c5b1af7eadee7b8c9b6935eb905722cc8a07773:45"),
    ("preview:aggregate-unrecognized resave local_projects[0].proposal_json", "0516c8de6da19a643e9e9bf7b789bc81dc099beab278ed3c82cffba1900ced6d:82"),
    ("preview:aggregate-unrecognized resave local_projects[0].selected_review_target_json", "b0b571f28e5724d9095fea8cdc00559629f998bc458f1f7f0225e302ef2e23c8:69"),
    ("preview:aggregate-unrecognized resave local_projects[0].updated_at_unix", "Integer(1793000000)"),
    ("preview:aggregate-unrecognized resave reply", "c5e05baaa5cecb2443b2caf33bf8c67d683d2a1ae86a35d656ea8df8923e53b6:4240"),
    ("preview:aggregate-unrecognized save local_projects[0].analysis_run_json", "e028b18561b9a976e443c9b619de0da6dda6b9d31f015a8811da7c2827b215ee:143597"),
    ("preview:aggregate-unrecognized save local_projects[0].created_at_unix", "Integer(1793000000)"),
    ("preview:aggregate-unrecognized save local_projects[0].editor_intents_json", "f21abe4b59cfb3b91e8df7df4f65f7f324e44423901aaf53e2fbfa1a7160642f:125"),
    ("preview:aggregate-unrecognized save local_projects[0].mechanics_result_json", "5613c1d834bc0125003ddfdafe753ffeed4b065fa1ae09409aa8b468a3b2eea9:71477"),
    ("preview:aggregate-unrecognized save local_projects[0].model_hash_json", "1e64c7d4d580e54c4b39dfc7592ed08ae5b50ff23277b7063bcbf6b0df7427ba:260"),
    ("preview:aggregate-unrecognized save local_projects[0].model_json", "4a1babd4a449831aeeea1835dfff057ebae6c91c14675443fde9da061f07ff3c:3584"),
    ("preview:aggregate-unrecognized save local_projects[0].model_migration_ledger_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:aggregate-unrecognized save local_projects[0].project_envelope_hash_json", "4024bec19546138d9ce34446241f9bfbe8a47190908092e0b7500053b6762846:341"),
    ("preview:aggregate-unrecognized save local_projects[0].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:aggregate-unrecognized save local_projects[0].project_name", "65a4ab50988845fb64d0c0005c5b1af7eadee7b8c9b6935eb905722cc8a07773:45"),
    ("preview:aggregate-unrecognized save local_projects[0].proposal_json", "0516c8de6da19a643e9e9bf7b789bc81dc099beab278ed3c82cffba1900ced6d:82"),
    ("preview:aggregate-unrecognized save local_projects[0].selected_review_target_json", "b0b571f28e5724d9095fea8cdc00559629f998bc458f1f7f0225e302ef2e23c8:69"),
    ("preview:aggregate-unrecognized save local_projects[0].updated_at_unix", "Integer(1793000000)"),
    ("preview:aggregate-unrecognized save reply", "20cb1d488cac0ad5b891df597217eeef1bd749ca13c4f4fad3fc783070b72675:3415"),
    ("preview:connected create reply", "12db8b02591081112d8ffafc2d8425666d686ad4d0b1fc0ea72fa5ce8ad183ad:3906"),
    ("preview:connected record analysis_run_labels[0].assumptions_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:connected record analysis_run_labels[0].notes_json", "5df6d4f94dea81443474cd8fd8b70333654763c9dfbea861d2428c19e7d00a73:18"),
    ("preview:connected record analysis_run_labels[0].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:connected record analysis_run_labels[0].recording_id", "a5471337cd07ec9d84281e972c47a1ab29097957eeb9bb3782ea4f25db882c16:39"),
    ("preview:connected record analysis_run_labels[0].run_name", "a89023ff6219b358cf8bbb555afdc6d5f4303df5bfe187d902c3622679b8092f:22"),
    ("preview:connected record analysis_run_labels[0].tags_json", "c38f1b165048a14cf0612830edb0f61f56be4cb4f1f3d01fbaa14e6655c52c55:10"),
    ("preview:connected record analysis_run_labels[0].updated_at", "04199ee21ddec3cb0080ba7ae86be72bbf19e53d97bc7b59ff3f343c25431d0e:24"),
    ("preview:connected record analysis_run_labels[1].assumptions_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:connected record analysis_run_labels[1].notes_json", "5df6d4f94dea81443474cd8fd8b70333654763c9dfbea861d2428c19e7d00a73:18"),
    ("preview:connected record analysis_run_labels[1].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:connected record analysis_run_labels[1].recording_id", "9684dc7198476df569377bd06f91cb63181459f1288df6e9f8f56e5eac58509e:39"),
    ("preview:connected record analysis_run_labels[1].run_name", "a89023ff6219b358cf8bbb555afdc6d5f4303df5bfe187d902c3622679b8092f:22"),
    ("preview:connected record analysis_run_labels[1].tags_json", "c38f1b165048a14cf0612830edb0f61f56be4cb4f1f3d01fbaa14e6655c52c55:10"),
    ("preview:connected record analysis_run_labels[1].updated_at", "1ac8477c538da6e93db66aa8df1bdf6f18a246ad755914704dcf06733e26b2fc:24"),
    ("preview:connected record analysis_runs[0].created_at", "04199ee21ddec3cb0080ba7ae86be72bbf19e53d97bc7b59ff3f343c25431d0e:24"),
    ("preview:connected record analysis_runs[0].input_manifest_json", "ec2b590838e26dbff151fe097131cd7a9ace800cb409041dbdf68a25a3e2c81e:6295"),
    ("preview:connected record analysis_runs[0].producer_run_id", "b3862e6694684d4c24986f49ec8e862f3872f1ae70359b7f55b3a8f340507b70:29"),
    ("preview:connected record analysis_runs[0].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:connected record analysis_runs[0].recording_id", "a5471337cd07ec9d84281e972c47a1ab29097957eeb9bb3782ea4f25db882c16:39"),
    ("preview:connected record analysis_runs[0].result_hash", "a8a20c6b217680871c9035cf81cfbcbbb6d84b058908d3f7ee90afdfbc4b360b:64"),
    ("preview:connected record analysis_runs[0].result_json", "76cb429be9d4557ca4b08467853a5d0f054cc5152438ecc7b13917621c680431:71477"),
    ("preview:connected record analysis_runs[0].run_record_hash", "d70d159d960f09291475032f48d8e30d067c1ecd660e7b93eef66b2cfd6572fa:64"),
    ("preview:connected record analysis_runs[0].run_record_json", "3c5dd680b6956e7c20924824f3d1ff81f95b0377a40ed44b1fc6cc6a5fc77a6d:143583"),
    ("preview:connected record analysis_runs[0].state_id", "3a3cd6490a1651ab2f9fd668c3dc0d5628c6d0120efb67da96e265d81bb849d8:38"),
    ("preview:connected record analysis_runs[1].created_at", "1ac8477c538da6e93db66aa8df1bdf6f18a246ad755914704dcf06733e26b2fc:24"),
    ("preview:connected record analysis_runs[1].input_manifest_json", "ec2b590838e26dbff151fe097131cd7a9ace800cb409041dbdf68a25a3e2c81e:6295"),
    ("preview:connected record analysis_runs[1].producer_run_id", "b3862e6694684d4c24986f49ec8e862f3872f1ae70359b7f55b3a8f340507b70:29"),
    ("preview:connected record analysis_runs[1].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:connected record analysis_runs[1].recording_id", "9684dc7198476df569377bd06f91cb63181459f1288df6e9f8f56e5eac58509e:39"),
    ("preview:connected record analysis_runs[1].result_hash", "a8a20c6b217680871c9035cf81cfbcbbb6d84b058908d3f7ee90afdfbc4b360b:64"),
    ("preview:connected record analysis_runs[1].result_json", "76cb429be9d4557ca4b08467853a5d0f054cc5152438ecc7b13917621c680431:71477"),
    ("preview:connected record analysis_runs[1].run_record_hash", "9a6e3eed6f4a7d859b68b7c74443197be5246d384d0cb6e9c4bbb166cdf3a6fc:64"),
    ("preview:connected record analysis_runs[1].run_record_json", "4c261b31ce5b5805c12083aab1c2268172864f0a1016cee577b550b64c20f862:143583"),
    ("preview:connected record analysis_runs[1].state_id", "3a3cd6490a1651ab2f9fd668c3dc0d5628c6d0120efb67da96e265d81bb849d8:38"),
    ("preview:connected record model_state_labels[0].assumptions_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:connected record model_state_labels[0].notes_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:connected record model_state_labels[0].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:connected record model_state_labels[0].state_id", "3a3cd6490a1651ab2f9fd668c3dc0d5628c6d0120efb67da96e265d81bb849d8:38"),
    ("preview:connected record model_state_labels[0].state_name", "d081e834436746542ed6bc8d1327bda6c85dea255b2af1f44313fa8fe075de15:11"),
    ("preview:connected record model_state_labels[0].tags_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:connected record model_state_labels[0].updated_at", "04199ee21ddec3cb0080ba7ae86be72bbf19e53d97bc7b59ff3f343c25431d0e:24"),
    ("preview:connected record model_states[0].created_at", "04199ee21ddec3cb0080ba7ae86be72bbf19e53d97bc7b59ff3f343c25431d0e:24"),
    ("preview:connected record model_states[0].model_canonical", "4a1babd4a449831aeeea1835dfff057ebae6c91c14675443fde9da061f07ff3c:3584"),
    ("preview:connected record model_states[0].model_hash", "b357d5f9e868c425b7b4c4798a8b0ac2c88c46635d806460cbf996d3d1fce416:71"),
    ("preview:connected record model_states[0].parent_state_id", "Null"),
    ("preview:connected record model_states[0].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:connected record model_states[0].record_hash", "c3a5e9ad1fa5ff45041703986f7f5a9f7f88d4c84ff79c9e0628734ad4042f5f:64"),
    ("preview:connected record model_states[0].record_json", "471bc60490823530e98857c539d997e949385a9864c5b6206998cc6e4c18f3d7:1718"),
    ("preview:connected record model_states[0].state_id", "3a3cd6490a1651ab2f9fd668c3dc0d5628c6d0120efb67da96e265d81bb849d8:38"),
    ("preview:connected record model_states[0].state_kind", "37e47861edf4c99a70c0e2513fc6d386a62547cb0a25f064d10785d204386944:19"),
    ("preview:connected record#0 reply", "591329160eb2a5601b185269a2c389e30c0ff645692c43e3bd0bbf4c5ba25563:1545"),
    ("preview:connected record#1 reply", "275e37023f3f4ca83c43fbf1f9ab79bcd15cbf91ce5dd1936ed69be9a0f09684:1530"),
    ("preview:connected recorded read#0 legacy record", "3c5dd680b6956e7c20924824f3d1ff81f95b0377a40ed44b1fc6cc6a5fc77a6d:143583"),
    ("preview:connected recorded read#0 reply", "1ea29575e6f1e7837afb702f3334a6bc0e85b927a5b41c2faf619ac6e1042906:13083"),
    ("preview:connected recorded read#1 legacy record", "4c261b31ce5b5805c12083aab1c2268172864f0a1016cee577b550b64c20f862:143583"),
    ("preview:connected recorded read#1 reply", "9e22d5dc79944ff500bc0e92a380ffd3ea338510207b1567a9fc31a8c88c1bb6:13083"),
    ("preview:connected reopen reply", "d4a1d76c830ced4f214d0034c35cd2f99a68cb8fa7a1155ce8377949568e0f64:11455"),
    ("preview:connected resave local_projects[0].analysis_run_json", "e028b18561b9a976e443c9b619de0da6dda6b9d31f015a8811da7c2827b215ee:143597"),
    ("preview:connected resave local_projects[0].created_at_unix", "Integer(1793000000)"),
    ("preview:connected resave local_projects[0].editor_intents_json", "f21abe4b59cfb3b91e8df7df4f65f7f324e44423901aaf53e2fbfa1a7160642f:125"),
    ("preview:connected resave local_projects[0].mechanics_result_json", "5613c1d834bc0125003ddfdafe753ffeed4b065fa1ae09409aa8b468a3b2eea9:71477"),
    ("preview:connected resave local_projects[0].model_hash_json", "1e64c7d4d580e54c4b39dfc7592ed08ae5b50ff23277b7063bcbf6b0df7427ba:260"),
    ("preview:connected resave local_projects[0].model_json", "4a1babd4a449831aeeea1835dfff057ebae6c91c14675443fde9da061f07ff3c:3584"),
    ("preview:connected resave local_projects[0].model_migration_ledger_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:connected resave local_projects[0].project_envelope_hash_json", "4024bec19546138d9ce34446241f9bfbe8a47190908092e0b7500053b6762846:341"),
    ("preview:connected resave local_projects[0].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:connected resave local_projects[0].project_name", "65a4ab50988845fb64d0c0005c5b1af7eadee7b8c9b6935eb905722cc8a07773:45"),
    ("preview:connected resave local_projects[0].proposal_json", "0516c8de6da19a643e9e9bf7b789bc81dc099beab278ed3c82cffba1900ced6d:82"),
    ("preview:connected resave local_projects[0].selected_review_target_json", "b0b571f28e5724d9095fea8cdc00559629f998bc458f1f7f0225e302ef2e23c8:69"),
    ("preview:connected resave local_projects[0].updated_at_unix", "Integer(1793000000)"),
    ("preview:connected resave reply", "c5e05baaa5cecb2443b2caf33bf8c67d683d2a1ae86a35d656ea8df8923e53b6:4240"),
    ("preview:connected save local_projects[0].analysis_run_json", "e028b18561b9a976e443c9b619de0da6dda6b9d31f015a8811da7c2827b215ee:143597"),
    ("preview:connected save local_projects[0].created_at_unix", "Integer(1793000000)"),
    ("preview:connected save local_projects[0].editor_intents_json", "f21abe4b59cfb3b91e8df7df4f65f7f324e44423901aaf53e2fbfa1a7160642f:125"),
    ("preview:connected save local_projects[0].mechanics_result_json", "5613c1d834bc0125003ddfdafe753ffeed4b065fa1ae09409aa8b468a3b2eea9:71477"),
    ("preview:connected save local_projects[0].model_hash_json", "1e64c7d4d580e54c4b39dfc7592ed08ae5b50ff23277b7063bcbf6b0df7427ba:260"),
    ("preview:connected save local_projects[0].model_json", "4a1babd4a449831aeeea1835dfff057ebae6c91c14675443fde9da061f07ff3c:3584"),
    ("preview:connected save local_projects[0].model_migration_ledger_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:connected save local_projects[0].project_envelope_hash_json", "4024bec19546138d9ce34446241f9bfbe8a47190908092e0b7500053b6762846:341"),
    ("preview:connected save local_projects[0].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:connected save local_projects[0].project_name", "65a4ab50988845fb64d0c0005c5b1af7eadee7b8c9b6935eb905722cc8a07773:45"),
    ("preview:connected save local_projects[0].proposal_json", "0516c8de6da19a643e9e9bf7b789bc81dc099beab278ed3c82cffba1900ced6d:82"),
    ("preview:connected save local_projects[0].selected_review_target_json", "b0b571f28e5724d9095fea8cdc00559629f998bc458f1f7f0225e302ef2e23c8:69"),
    ("preview:connected save local_projects[0].updated_at_unix", "Integer(1793000000)"),
    ("preview:connected save reply", "20cb1d488cac0ad5b891df597217eeef1bd749ca13c4f4fad3fc783070b72675:3415"),
    ("preview:pressure create reply", "c088fcd11dd7039c5cf864016b5c62fa6d571fd6ffa280a7c48c8996e34da3ae:4066"),
    ("preview:pressure record analysis_run_labels[0].assumptions_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:pressure record analysis_run_labels[0].notes_json", "5df6d4f94dea81443474cd8fd8b70333654763c9dfbea861d2428c19e7d00a73:18"),
    ("preview:pressure record analysis_run_labels[0].project_id", "a6246e580c56c6f2e381d6c56e6dc78b5e615a7c342071f9c92fe17c44f0ff84:40"),
    ("preview:pressure record analysis_run_labels[0].recording_id", "07689368b3af7643ebcaf0853ddc0ebc5db3aaef167e41997e62a38f356b578a:39"),
    ("preview:pressure record analysis_run_labels[0].run_name", "a89023ff6219b358cf8bbb555afdc6d5f4303df5bfe187d902c3622679b8092f:22"),
    ("preview:pressure record analysis_run_labels[0].tags_json", "c38f1b165048a14cf0612830edb0f61f56be4cb4f1f3d01fbaa14e6655c52c55:10"),
    ("preview:pressure record analysis_run_labels[0].updated_at", "04199ee21ddec3cb0080ba7ae86be72bbf19e53d97bc7b59ff3f343c25431d0e:24"),
    ("preview:pressure record analysis_run_labels[1].assumptions_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:pressure record analysis_run_labels[1].notes_json", "5df6d4f94dea81443474cd8fd8b70333654763c9dfbea861d2428c19e7d00a73:18"),
    ("preview:pressure record analysis_run_labels[1].project_id", "a6246e580c56c6f2e381d6c56e6dc78b5e615a7c342071f9c92fe17c44f0ff84:40"),
    ("preview:pressure record analysis_run_labels[1].recording_id", "bc3ace16ec81b35a271be3e441847aab6d3743078ab8a0a80d591fda7ffcb68f:39"),
    ("preview:pressure record analysis_run_labels[1].run_name", "a89023ff6219b358cf8bbb555afdc6d5f4303df5bfe187d902c3622679b8092f:22"),
    ("preview:pressure record analysis_run_labels[1].tags_json", "c38f1b165048a14cf0612830edb0f61f56be4cb4f1f3d01fbaa14e6655c52c55:10"),
    ("preview:pressure record analysis_run_labels[1].updated_at", "1ac8477c538da6e93db66aa8df1bdf6f18a246ad755914704dcf06733e26b2fc:24"),
    ("preview:pressure record analysis_runs[0].created_at", "04199ee21ddec3cb0080ba7ae86be72bbf19e53d97bc7b59ff3f343c25431d0e:24"),
    ("preview:pressure record analysis_runs[0].input_manifest_json", "b2085836c1c17e0376de5b7bbfbaf074508b5f3eff0074fe0620a9a56b48bf65:7991"),
    ("preview:pressure record analysis_runs[0].producer_run_id", "b3862e6694684d4c24986f49ec8e862f3872f1ae70359b7f55b3a8f340507b70:29"),
    ("preview:pressure record analysis_runs[0].project_id", "a6246e580c56c6f2e381d6c56e6dc78b5e615a7c342071f9c92fe17c44f0ff84:40"),
    ("preview:pressure record analysis_runs[0].recording_id", "07689368b3af7643ebcaf0853ddc0ebc5db3aaef167e41997e62a38f356b578a:39"),
    ("preview:pressure record analysis_runs[0].result_hash", "c6a14a9e3e828ab68ec7791586ba22b37b6f4701a77a388be4697c4a4da5f2b8:64"),
    ("preview:pressure record analysis_runs[0].result_json", "1dc09edf5eb154fda54c870951c0231247b2758b78e5ecfd6b1547711c8933fb:255029"),
    ("preview:pressure record analysis_runs[0].run_record_hash", "46a5b44317a793b71e08bef0dec7fd5ba50a2e9ec25f4defee2d363ba4bfafd7:64"),
    ("preview:pressure record analysis_runs[0].run_record_json", "200dd548fde6ef7d430661958c03a91f5c8ecd45620f15a364b0bc4d4201d852:520513"),
    ("preview:pressure record analysis_runs[0].state_id", "3a3cd6490a1651ab2f9fd668c3dc0d5628c6d0120efb67da96e265d81bb849d8:38"),
    ("preview:pressure record analysis_runs[1].created_at", "1ac8477c538da6e93db66aa8df1bdf6f18a246ad755914704dcf06733e26b2fc:24"),
    ("preview:pressure record analysis_runs[1].input_manifest_json", "b2085836c1c17e0376de5b7bbfbaf074508b5f3eff0074fe0620a9a56b48bf65:7991"),
    ("preview:pressure record analysis_runs[1].producer_run_id", "b3862e6694684d4c24986f49ec8e862f3872f1ae70359b7f55b3a8f340507b70:29"),
    ("preview:pressure record analysis_runs[1].project_id", "a6246e580c56c6f2e381d6c56e6dc78b5e615a7c342071f9c92fe17c44f0ff84:40"),
    ("preview:pressure record analysis_runs[1].recording_id", "bc3ace16ec81b35a271be3e441847aab6d3743078ab8a0a80d591fda7ffcb68f:39"),
    ("preview:pressure record analysis_runs[1].result_hash", "c6a14a9e3e828ab68ec7791586ba22b37b6f4701a77a388be4697c4a4da5f2b8:64"),
    ("preview:pressure record analysis_runs[1].result_json", "1dc09edf5eb154fda54c870951c0231247b2758b78e5ecfd6b1547711c8933fb:255029"),
    ("preview:pressure record analysis_runs[1].run_record_hash", "4e7ccd8b840145eddbe8a5d67aadff4f215605ac573fcc2a7a63d9020d18e5af:64"),
    ("preview:pressure record analysis_runs[1].run_record_json", "378510ddcce679b6f9bfd220dbbc15a0534a800d422f2c8a108203f734c8d006:520513"),
    ("preview:pressure record analysis_runs[1].state_id", "3a3cd6490a1651ab2f9fd668c3dc0d5628c6d0120efb67da96e265d81bb849d8:38"),
    ("preview:pressure record model_state_labels[0].assumptions_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:pressure record model_state_labels[0].notes_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:pressure record model_state_labels[0].project_id", "a6246e580c56c6f2e381d6c56e6dc78b5e615a7c342071f9c92fe17c44f0ff84:40"),
    ("preview:pressure record model_state_labels[0].state_id", "3a3cd6490a1651ab2f9fd668c3dc0d5628c6d0120efb67da96e265d81bb849d8:38"),
    ("preview:pressure record model_state_labels[0].state_name", "d081e834436746542ed6bc8d1327bda6c85dea255b2af1f44313fa8fe075de15:11"),
    ("preview:pressure record model_state_labels[0].tags_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:pressure record model_state_labels[0].updated_at", "04199ee21ddec3cb0080ba7ae86be72bbf19e53d97bc7b59ff3f343c25431d0e:24"),
    ("preview:pressure record model_states[0].created_at", "04199ee21ddec3cb0080ba7ae86be72bbf19e53d97bc7b59ff3f343c25431d0e:24"),
    ("preview:pressure record model_states[0].model_canonical", "e8c2ceaf7d0ad281ae7634dc40bf4780f20bc6fe929b680f31275f7456b80e84:4535"),
    ("preview:pressure record model_states[0].model_hash", "e2c0a4b0102b76b50df443856a601741f85c9494692421f2c1e4bb5cf81a59cd:71"),
    ("preview:pressure record model_states[0].parent_state_id", "Null"),
    ("preview:pressure record model_states[0].project_id", "a6246e580c56c6f2e381d6c56e6dc78b5e615a7c342071f9c92fe17c44f0ff84:40"),
    ("preview:pressure record model_states[0].record_hash", "e3547e1e4174b6d4140df02d97e7f3dce46d5167f15b6b59c13d83949a38e926:64"),
    ("preview:pressure record model_states[0].record_json", "253ce993c730a0c13ae0aa92f144cc329b0f51bee95907dd4db84511f5ec05e7:1728"),
    ("preview:pressure record model_states[0].state_id", "3a3cd6490a1651ab2f9fd668c3dc0d5628c6d0120efb67da96e265d81bb849d8:38"),
    ("preview:pressure record model_states[0].state_kind", "37e47861edf4c99a70c0e2513fc6d386a62547cb0a25f064d10785d204386944:19"),
    ("preview:pressure record#0 reply", "6beacd61e48daafd75f4152cf5abcb3b444b9f1a8a4986bf1cb8bddb91b8b1af:1555"),
    ("preview:pressure record#1 reply", "22b5854175421169df8a5e7b04cdfb790d57365d2215c6d74a79704616919b09:1540"),
    ("preview:pressure recorded read#0 legacy record", "200dd548fde6ef7d430661958c03a91f5c8ecd45620f15a364b0bc4d4201d852:520513"),
    ("preview:pressure recorded read#0 reply", "514eeda7fa6b9400b91ab1026131619bb01df26ed250c61588043cde152269ac:15056"),
    ("preview:pressure recorded read#1 legacy record", "378510ddcce679b6f9bfd220dbbc15a0534a800d422f2c8a108203f734c8d006:520513"),
    ("preview:pressure recorded read#1 reply", "e9e9c4ecbcc83ebc38a5de335fd3716cd877391660f1dd01e6eed7a4f21ed77e:15056"),
    ("preview:pressure reopen reply", "023df262f487a13095588bdedfc6e8384fc183cd4aeb4d4d5f7b55871f52bd1e:12646"),
    ("preview:pressure resave local_projects[0].analysis_run_json", "0ba4b7ed590a01c6c8431e712e7a7a7c143d6bea275bd7bfcba68d3e0bc7b328:520532"),
    ("preview:pressure resave local_projects[0].created_at_unix", "Integer(1793000000)"),
    ("preview:pressure resave local_projects[0].editor_intents_json", "f21abe4b59cfb3b91e8df7df4f65f7f324e44423901aaf53e2fbfa1a7160642f:125"),
    ("preview:pressure resave local_projects[0].mechanics_result_json", "bb0d841beb2f5963674e17f3d0cb967dd51d0197e2641affc6265be31f51ace3:255007"),
    ("preview:pressure resave local_projects[0].model_hash_json", "8964b294b87213990ad911210c5d96db42857c712104dad1c0fedfae00c85213:265"),
    ("preview:pressure resave local_projects[0].model_json", "e8c2ceaf7d0ad281ae7634dc40bf4780f20bc6fe929b680f31275f7456b80e84:4535"),
    ("preview:pressure resave local_projects[0].model_migration_ledger_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:pressure resave local_projects[0].project_envelope_hash_json", "ce277ae786c2d9e01456638c1bd9481d90d5f80c8e36999514a6595498d5f164:346"),
    ("preview:pressure resave local_projects[0].project_id", "a6246e580c56c6f2e381d6c56e6dc78b5e615a7c342071f9c92fe17c44f0ff84:40"),
    ("preview:pressure resave local_projects[0].project_name", "a3be13fbc758ede01e3aed6c502f013fd0465bc37324e55e04b68b3be9588a79:42"),
    ("preview:pressure resave local_projects[0].proposal_json", "0516c8de6da19a643e9e9bf7b789bc81dc099beab278ed3c82cffba1900ced6d:82"),
    ("preview:pressure resave local_projects[0].selected_review_target_json", "b0b571f28e5724d9095fea8cdc00559629f998bc458f1f7f0225e302ef2e23c8:69"),
    ("preview:pressure resave local_projects[0].updated_at_unix", "Integer(1793000000)"),
    ("preview:pressure resave reply", "dfe41beeb5d5d965f54f86081a89c661234e69cf11aeb6f74e27fa00f4db6871:4405"),
    ("preview:pressure save local_projects[0].analysis_run_json", "0ba4b7ed590a01c6c8431e712e7a7a7c143d6bea275bd7bfcba68d3e0bc7b328:520532"),
    ("preview:pressure save local_projects[0].created_at_unix", "Integer(1793000000)"),
    ("preview:pressure save local_projects[0].editor_intents_json", "f21abe4b59cfb3b91e8df7df4f65f7f324e44423901aaf53e2fbfa1a7160642f:125"),
    ("preview:pressure save local_projects[0].mechanics_result_json", "bb0d841beb2f5963674e17f3d0cb967dd51d0197e2641affc6265be31f51ace3:255007"),
    ("preview:pressure save local_projects[0].model_hash_json", "8964b294b87213990ad911210c5d96db42857c712104dad1c0fedfae00c85213:265"),
    ("preview:pressure save local_projects[0].model_json", "e8c2ceaf7d0ad281ae7634dc40bf4780f20bc6fe929b680f31275f7456b80e84:4535"),
    ("preview:pressure save local_projects[0].model_migration_ledger_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:pressure save local_projects[0].project_envelope_hash_json", "ce277ae786c2d9e01456638c1bd9481d90d5f80c8e36999514a6595498d5f164:346"),
    ("preview:pressure save local_projects[0].project_id", "a6246e580c56c6f2e381d6c56e6dc78b5e615a7c342071f9c92fe17c44f0ff84:40"),
    ("preview:pressure save local_projects[0].project_name", "a3be13fbc758ede01e3aed6c502f013fd0465bc37324e55e04b68b3be9588a79:42"),
    ("preview:pressure save local_projects[0].proposal_json", "0516c8de6da19a643e9e9bf7b789bc81dc099beab278ed3c82cffba1900ced6d:82"),
    ("preview:pressure save local_projects[0].selected_review_target_json", "b0b571f28e5724d9095fea8cdc00559629f998bc458f1f7f0225e302ef2e23c8:69"),
    ("preview:pressure save local_projects[0].updated_at_unix", "Integer(1793000000)"),
    ("preview:pressure save reply", "023d40876b6cda5f5b0ede87d2c59665673db81670a51e12d6027b66e4767e5d:3575"),
    ("preview:pressure-source create reply", "65b753d8ad90ae47bdaf37e1da49b89d893bfe13243c88d90db52dbb5c85de6b:3461"),
    ("preview:pressure-source record analysis_run_labels[0].assumptions_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:pressure-source record analysis_run_labels[0].notes_json", "5df6d4f94dea81443474cd8fd8b70333654763c9dfbea861d2428c19e7d00a73:18"),
    ("preview:pressure-source record analysis_run_labels[0].project_id", "0693e489f130b54637dcd80cc4ded978a783ee61ac6c8c39c32e94620c0ad4ef:28"),
    ("preview:pressure-source record analysis_run_labels[0].recording_id", "f56751e200f4d7075762307a07d50eb51db7fe1233a6abddb9b36090c4ead0c4:39"),
    ("preview:pressure-source record analysis_run_labels[0].run_name", "a89023ff6219b358cf8bbb555afdc6d5f4303df5bfe187d902c3622679b8092f:22"),
    ("preview:pressure-source record analysis_run_labels[0].tags_json", "c38f1b165048a14cf0612830edb0f61f56be4cb4f1f3d01fbaa14e6655c52c55:10"),
    ("preview:pressure-source record analysis_run_labels[0].updated_at", "04199ee21ddec3cb0080ba7ae86be72bbf19e53d97bc7b59ff3f343c25431d0e:24"),
    ("preview:pressure-source record analysis_run_labels[1].assumptions_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:pressure-source record analysis_run_labels[1].notes_json", "5df6d4f94dea81443474cd8fd8b70333654763c9dfbea861d2428c19e7d00a73:18"),
    ("preview:pressure-source record analysis_run_labels[1].project_id", "0693e489f130b54637dcd80cc4ded978a783ee61ac6c8c39c32e94620c0ad4ef:28"),
    ("preview:pressure-source record analysis_run_labels[1].recording_id", "391c6a3626ee5c4cfec09e30d75f66b19474fbe1410903bb0e15d2a961d17372:39"),
    ("preview:pressure-source record analysis_run_labels[1].run_name", "a89023ff6219b358cf8bbb555afdc6d5f4303df5bfe187d902c3622679b8092f:22"),
    ("preview:pressure-source record analysis_run_labels[1].tags_json", "c38f1b165048a14cf0612830edb0f61f56be4cb4f1f3d01fbaa14e6655c52c55:10"),
    ("preview:pressure-source record analysis_run_labels[1].updated_at", "1ac8477c538da6e93db66aa8df1bdf6f18a246ad755914704dcf06733e26b2fc:24"),
    ("preview:pressure-source record analysis_runs[0].created_at", "04199ee21ddec3cb0080ba7ae86be72bbf19e53d97bc7b59ff3f343c25431d0e:24"),
    ("preview:pressure-source record analysis_runs[0].input_manifest_json", "1e30137a32aef60b22cb5760dc68dfbccddfe2a6955614d8c51dc590abb2ed1f:4610"),
    ("preview:pressure-source record analysis_runs[0].producer_run_id", "b3862e6694684d4c24986f49ec8e862f3872f1ae70359b7f55b3a8f340507b70:29"),
    ("preview:pressure-source record analysis_runs[0].project_id", "0693e489f130b54637dcd80cc4ded978a783ee61ac6c8c39c32e94620c0ad4ef:28"),
    ("preview:pressure-source record analysis_runs[0].recording_id", "f56751e200f4d7075762307a07d50eb51db7fe1233a6abddb9b36090c4ead0c4:39"),
    ("preview:pressure-source record analysis_runs[0].result_hash", "5443d51744d5e99a6d3ca353dce47316e42b511fb32a05f42bcf4ee088338d7e:64"),
    ("preview:pressure-source record analysis_runs[0].result_json", "46828f509da211e38c48ae46f6f44a472ca81f609b0ab588903bf5738b044311:187730"),
    ("preview:pressure-source record analysis_runs[0].run_record_hash", "c8d171f10b0d311828a4cc2f0694bdff10ad6f0c1e1ffbdf3b51815159ab2f96:64"),
    ("preview:pressure-source record analysis_runs[0].run_record_json", "ee8a48d1fc5fe1123a5cbbc1f2f6e728f0885f59d6952ef31d346ea7b9e1bbed:304292"),
    ("preview:pressure-source record analysis_runs[0].state_id", "3a3cd6490a1651ab2f9fd668c3dc0d5628c6d0120efb67da96e265d81bb849d8:38"),
    ("preview:pressure-source record analysis_runs[1].created_at", "1ac8477c538da6e93db66aa8df1bdf6f18a246ad755914704dcf06733e26b2fc:24"),
    ("preview:pressure-source record analysis_runs[1].input_manifest_json", "1e30137a32aef60b22cb5760dc68dfbccddfe2a6955614d8c51dc590abb2ed1f:4610"),
    ("preview:pressure-source record analysis_runs[1].producer_run_id", "b3862e6694684d4c24986f49ec8e862f3872f1ae70359b7f55b3a8f340507b70:29"),
    ("preview:pressure-source record analysis_runs[1].project_id", "0693e489f130b54637dcd80cc4ded978a783ee61ac6c8c39c32e94620c0ad4ef:28"),
    ("preview:pressure-source record analysis_runs[1].recording_id", "391c6a3626ee5c4cfec09e30d75f66b19474fbe1410903bb0e15d2a961d17372:39"),
    ("preview:pressure-source record analysis_runs[1].result_hash", "5443d51744d5e99a6d3ca353dce47316e42b511fb32a05f42bcf4ee088338d7e:64"),
    ("preview:pressure-source record analysis_runs[1].result_json", "46828f509da211e38c48ae46f6f44a472ca81f609b0ab588903bf5738b044311:187730"),
    ("preview:pressure-source record analysis_runs[1].run_record_hash", "747a83bbd8343dcdb1cfd998bfd945b86f5c00091247802d08250e0223b78a7c:64"),
    ("preview:pressure-source record analysis_runs[1].run_record_json", "8be72892f528b012e6b892a6c67aec8fd7e8e883b19e0deba73882471e4a4c6e:304292"),
    ("preview:pressure-source record analysis_runs[1].state_id", "3a3cd6490a1651ab2f9fd668c3dc0d5628c6d0120efb67da96e265d81bb849d8:38"),
    ("preview:pressure-source record model_state_labels[0].assumptions_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:pressure-source record model_state_labels[0].notes_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:pressure-source record model_state_labels[0].project_id", "0693e489f130b54637dcd80cc4ded978a783ee61ac6c8c39c32e94620c0ad4ef:28"),
    ("preview:pressure-source record model_state_labels[0].state_id", "3a3cd6490a1651ab2f9fd668c3dc0d5628c6d0120efb67da96e265d81bb849d8:38"),
    ("preview:pressure-source record model_state_labels[0].state_name", "d081e834436746542ed6bc8d1327bda6c85dea255b2af1f44313fa8fe075de15:11"),
    ("preview:pressure-source record model_state_labels[0].tags_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:pressure-source record model_state_labels[0].updated_at", "04199ee21ddec3cb0080ba7ae86be72bbf19e53d97bc7b59ff3f343c25431d0e:24"),
    ("preview:pressure-source record model_states[0].created_at", "04199ee21ddec3cb0080ba7ae86be72bbf19e53d97bc7b59ff3f343c25431d0e:24"),
    ("preview:pressure-source record model_states[0].model_canonical", "ed9e8b65562e1a328ea7293fe8cc7e696ee7e776c4b27810b1f116535a0b1c67:3162"),
    ("preview:pressure-source record model_states[0].model_hash", "834ac26a0718b6d5d402ab6805a3fd4f150e7d0edf45de158f9bfd60bcdf54f3:71"),
    ("preview:pressure-source record model_states[0].parent_state_id", "Null"),
    ("preview:pressure-source record model_states[0].project_id", "0693e489f130b54637dcd80cc4ded978a783ee61ac6c8c39c32e94620c0ad4ef:28"),
    ("preview:pressure-source record model_states[0].record_hash", "918e8d4138338145b701eabe76dfb6270b8520046a6ea9de6dd207dce30cc6ce:64"),
    ("preview:pressure-source record model_states[0].record_json", "6816c48d1c3f81e73437dcf494cc4a94f973e36ce21ddef9dd8e68d293db7435:1704"),
    ("preview:pressure-source record model_states[0].state_id", "3a3cd6490a1651ab2f9fd668c3dc0d5628c6d0120efb67da96e265d81bb849d8:38"),
    ("preview:pressure-source record model_states[0].state_kind", "37e47861edf4c99a70c0e2513fc6d386a62547cb0a25f064d10785d204386944:19"),
    ("preview:pressure-source record#0 reply", "cda000c83fb0a4df78b4236002ae4eb22f1c6137723a7e3aba4dfdf730c84eea:1531"),
    ("preview:pressure-source record#1 reply", "c341e714f12f32f2e62d8b52af4784973d32782d88f00353dee7e7fbe4c2c3c0:1516"),
    ("preview:pressure-source recorded read#0 legacy record", "ee8a48d1fc5fe1123a5cbbc1f2f6e728f0885f59d6952ef31d346ea7b9e1bbed:304292"),
    ("preview:pressure-source recorded read#0 reply", "d2112690309c35f9237d5b93bc9fe987f0a7f805114d3a496b5a1e21e6a55a38:11119"),
    ("preview:pressure-source recorded read#1 legacy record", "8be72892f528b012e6b892a6c67aec8fd7e8e883b19e0deba73882471e4a4c6e:304292"),
    ("preview:pressure-source recorded read#1 reply", "9b1f1660c3c7557d146c691a40042df2c1c11574f4f963c25bd4a7a0ddc0c98e:11119"),
    ("preview:pressure-source reopen reply", "5345fcdbc7d38e5585337cb0e35fe581c3093a00779f8729f85e8875ccbff5ef:10617"),
    ("preview:pressure-source resave local_projects[0].analysis_run_json", "3b85de03c57348999e907a4e3893069d49d60d560335451e4356bf7bec541173:304299"),
    ("preview:pressure-source resave local_projects[0].created_at_unix", "Integer(1793000000)"),
    ("preview:pressure-source resave local_projects[0].editor_intents_json", "f21abe4b59cfb3b91e8df7df4f65f7f324e44423901aaf53e2fbfa1a7160642f:125"),
    ("preview:pressure-source resave local_projects[0].mechanics_result_json", "12b8612f915f9f54f168f6e0d3964d8dec4caa4b536f688938e4c864eef597f6:187730"),
    ("preview:pressure-source resave local_projects[0].model_hash_json", "a0330a79164a3502f0e5b51d30c6af25a815d4a8bb336be84c2bf37dc14bf674:253"),
    ("preview:pressure-source resave local_projects[0].model_json", "ed9e8b65562e1a328ea7293fe8cc7e696ee7e776c4b27810b1f116535a0b1c67:3162"),
    ("preview:pressure-source resave local_projects[0].model_migration_ledger_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:pressure-source resave local_projects[0].project_envelope_hash_json", "c974532810aca14b62077cceacce7a426f7673d9938bf4aded9f638c44bb3839:334"),
    ("preview:pressure-source resave local_projects[0].project_id", "0693e489f130b54637dcd80cc4ded978a783ee61ac6c8c39c32e94620c0ad4ef:28"),
    ("preview:pressure-source resave local_projects[0].project_name", "ed4d498abca0d8e212e2fb959bc79a253ecd89531261e18d45740b2d40ec76a8:43"),
    ("preview:pressure-source resave local_projects[0].proposal_json", "0516c8de6da19a643e9e9bf7b789bc81dc099beab278ed3c82cffba1900ced6d:82"),
    ("preview:pressure-source resave local_projects[0].selected_review_target_json", "ba3aa40eae8d2c0209a00ad34f486243ccf8eefdcb1fedd76203c20e1b607773:60"),
    ("preview:pressure-source resave local_projects[0].updated_at_unix", "Integer(1793000000)"),
    ("preview:pressure-source resave reply", "720527c371ca18e9ba0200f898f8762b37676cf3b933832b6a60756dd0cbd902:3834"),
    ("preview:pressure-source save local_projects[0].analysis_run_json", "3b85de03c57348999e907a4e3893069d49d60d560335451e4356bf7bec541173:304299"),
    ("preview:pressure-source save local_projects[0].created_at_unix", "Integer(1793000000)"),
    ("preview:pressure-source save local_projects[0].editor_intents_json", "f21abe4b59cfb3b91e8df7df4f65f7f324e44423901aaf53e2fbfa1a7160642f:125"),
    ("preview:pressure-source save local_projects[0].mechanics_result_json", "12b8612f915f9f54f168f6e0d3964d8dec4caa4b536f688938e4c864eef597f6:187730"),
    ("preview:pressure-source save local_projects[0].model_hash_json", "a0330a79164a3502f0e5b51d30c6af25a815d4a8bb336be84c2bf37dc14bf674:253"),
    ("preview:pressure-source save local_projects[0].model_json", "ed9e8b65562e1a328ea7293fe8cc7e696ee7e776c4b27810b1f116535a0b1c67:3162"),
    ("preview:pressure-source save local_projects[0].model_migration_ledger_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:pressure-source save local_projects[0].project_envelope_hash_json", "c974532810aca14b62077cceacce7a426f7673d9938bf4aded9f638c44bb3839:334"),
    ("preview:pressure-source save local_projects[0].project_id", "0693e489f130b54637dcd80cc4ded978a783ee61ac6c8c39c32e94620c0ad4ef:28"),
    ("preview:pressure-source save local_projects[0].project_name", "ed4d498abca0d8e212e2fb959bc79a253ecd89531261e18d45740b2d40ec76a8:43"),
    ("preview:pressure-source save local_projects[0].proposal_json", "0516c8de6da19a643e9e9bf7b789bc81dc099beab278ed3c82cffba1900ced6d:82"),
    ("preview:pressure-source save local_projects[0].selected_review_target_json", "ba3aa40eae8d2c0209a00ad34f486243ccf8eefdcb1fedd76203c20e1b607773:60"),
    ("preview:pressure-source save local_projects[0].updated_at_unix", "Integer(1793000000)"),
    ("preview:pressure-source save reply", "1bcc8b842631e14504ae01e7fe0055297552bb568609dfa79088bc031751b7d3:2970"),
    ("preview:retained create reply", "a7f794e0802feb204a81438de1f1507a7508d7a6060c3bca2b7b5aa249f0c5d9:3478"),
    ("preview:retained record analysis_run_labels[0].assumptions_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:retained record analysis_run_labels[0].notes_json", "5df6d4f94dea81443474cd8fd8b70333654763c9dfbea861d2428c19e7d00a73:18"),
    ("preview:retained record analysis_run_labels[0].project_id", "d8225a4b1710080c08117f701329684ab30b1d6fb3889f5aeb9f6fea6ad90c35:44"),
    ("preview:retained record analysis_run_labels[0].recording_id", "97a2876b0cfaf9713ff74a94d0d4f235f361ae105c0abc948843658f4bd6b9b3:39"),
    ("preview:retained record analysis_run_labels[0].run_name", "a89023ff6219b358cf8bbb555afdc6d5f4303df5bfe187d902c3622679b8092f:22"),
    ("preview:retained record analysis_run_labels[0].tags_json", "c38f1b165048a14cf0612830edb0f61f56be4cb4f1f3d01fbaa14e6655c52c55:10"),
    ("preview:retained record analysis_run_labels[0].updated_at", "04199ee21ddec3cb0080ba7ae86be72bbf19e53d97bc7b59ff3f343c25431d0e:24"),
    ("preview:retained record analysis_run_labels[1].assumptions_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:retained record analysis_run_labels[1].notes_json", "5df6d4f94dea81443474cd8fd8b70333654763c9dfbea861d2428c19e7d00a73:18"),
    ("preview:retained record analysis_run_labels[1].project_id", "d8225a4b1710080c08117f701329684ab30b1d6fb3889f5aeb9f6fea6ad90c35:44"),
    ("preview:retained record analysis_run_labels[1].recording_id", "9ae287b7463d5ee4a3e8a4d10a564eb35c85b4115d669217f94e4fe1cda4a586:39"),
    ("preview:retained record analysis_run_labels[1].run_name", "a89023ff6219b358cf8bbb555afdc6d5f4303df5bfe187d902c3622679b8092f:22"),
    ("preview:retained record analysis_run_labels[1].tags_json", "c38f1b165048a14cf0612830edb0f61f56be4cb4f1f3d01fbaa14e6655c52c55:10"),
    ("preview:retained record analysis_run_labels[1].updated_at", "1ac8477c538da6e93db66aa8df1bdf6f18a246ad755914704dcf06733e26b2fc:24"),
    ("preview:retained record analysis_runs[0].created_at", "04199ee21ddec3cb0080ba7ae86be72bbf19e53d97bc7b59ff3f343c25431d0e:24"),
    ("preview:retained record analysis_runs[0].input_manifest_json", "637d7eaa945b1e402c694b59bf1cee426e080e27eff991b3366c687c98dffea9:4585"),
    ("preview:retained record analysis_runs[0].producer_run_id", "b3862e6694684d4c24986f49ec8e862f3872f1ae70359b7f55b3a8f340507b70:29"),
    ("preview:retained record analysis_runs[0].project_id", "d8225a4b1710080c08117f701329684ab30b1d6fb3889f5aeb9f6fea6ad90c35:44"),
    ("preview:retained record analysis_runs[0].recording_id", "97a2876b0cfaf9713ff74a94d0d4f235f361ae105c0abc948843658f4bd6b9b3:39"),
    ("preview:retained record analysis_runs[0].result_hash", "ac84fa79ee1ee4f7b18059c692ed0941d6084521b2adf72f1f3d67385e65dd53:64"),
    ("preview:retained record analysis_runs[0].result_json", "f3777ffbfbabf817e87877157258a443e8609249885cf2293666cca501d26ae9:135586"),
    ("preview:retained record analysis_runs[0].run_record_hash", "167c13ef010bb2a3882ca772631687c32e962f5b0fa15203db50a00a3267c2b3:64"),
    ("preview:retained record analysis_runs[0].run_record_json", "1de3595da31d923797e8ef2b6475522230c404b2cddb5795a2111259366333c4:233161"),
    ("preview:retained record analysis_runs[0].state_id", "3a3cd6490a1651ab2f9fd668c3dc0d5628c6d0120efb67da96e265d81bb849d8:38"),
    ("preview:retained record analysis_runs[1].created_at", "1ac8477c538da6e93db66aa8df1bdf6f18a246ad755914704dcf06733e26b2fc:24"),
    ("preview:retained record analysis_runs[1].input_manifest_json", "637d7eaa945b1e402c694b59bf1cee426e080e27eff991b3366c687c98dffea9:4585"),
    ("preview:retained record analysis_runs[1].producer_run_id", "b3862e6694684d4c24986f49ec8e862f3872f1ae70359b7f55b3a8f340507b70:29"),
    ("preview:retained record analysis_runs[1].project_id", "d8225a4b1710080c08117f701329684ab30b1d6fb3889f5aeb9f6fea6ad90c35:44"),
    ("preview:retained record analysis_runs[1].recording_id", "9ae287b7463d5ee4a3e8a4d10a564eb35c85b4115d669217f94e4fe1cda4a586:39"),
    ("preview:retained record analysis_runs[1].result_hash", "ac84fa79ee1ee4f7b18059c692ed0941d6084521b2adf72f1f3d67385e65dd53:64"),
    ("preview:retained record analysis_runs[1].result_json", "f3777ffbfbabf817e87877157258a443e8609249885cf2293666cca501d26ae9:135586"),
    ("preview:retained record analysis_runs[1].run_record_hash", "4f32a37838f5e74d5b505d4c66de320aae02fcff0bd83396d08e05ae3fab8581:64"),
    ("preview:retained record analysis_runs[1].run_record_json", "423cd118b138fd2a65b182033e8488a5102e4bb52952725421dc4bd4eb420ccc:233161"),
    ("preview:retained record analysis_runs[1].state_id", "3a3cd6490a1651ab2f9fd668c3dc0d5628c6d0120efb67da96e265d81bb849d8:38"),
    ("preview:retained record model_state_labels[0].assumptions_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:retained record model_state_labels[0].notes_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:retained record model_state_labels[0].project_id", "d8225a4b1710080c08117f701329684ab30b1d6fb3889f5aeb9f6fea6ad90c35:44"),
    ("preview:retained record model_state_labels[0].state_id", "3a3cd6490a1651ab2f9fd668c3dc0d5628c6d0120efb67da96e265d81bb849d8:38"),
    ("preview:retained record model_state_labels[0].state_name", "d081e834436746542ed6bc8d1327bda6c85dea255b2af1f44313fa8fe075de15:11"),
    ("preview:retained record model_state_labels[0].tags_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:retained record model_state_labels[0].updated_at", "04199ee21ddec3cb0080ba7ae86be72bbf19e53d97bc7b59ff3f343c25431d0e:24"),
    ("preview:retained record model_states[0].created_at", "04199ee21ddec3cb0080ba7ae86be72bbf19e53d97bc7b59ff3f343c25431d0e:24"),
    ("preview:retained record model_states[0].model_canonical", "af5f9efba3803af453bc5604c5993b8240f37874fb204a5abf03d502750567fb:2732"),
    ("preview:retained record model_states[0].model_hash", "cb1564600ed513a6bdab9fb7f70a3e02ce35995ef627f9ed0d5524bdcac2cecd:71"),
    ("preview:retained record model_states[0].parent_state_id", "Null"),
    ("preview:retained record model_states[0].project_id", "d8225a4b1710080c08117f701329684ab30b1d6fb3889f5aeb9f6fea6ad90c35:44"),
    ("preview:retained record model_states[0].record_hash", "d2474ac78c2c347645a004b7b4d39f34d187ec6bebae7aa8c3815d219bac626d:64"),
    ("preview:retained record model_states[0].record_json", "b063c0e85518de790fe33366f17582a84214eb99d1f06dece21c8c53ffcf569b:1736"),
    ("preview:retained record model_states[0].state_id", "3a3cd6490a1651ab2f9fd668c3dc0d5628c6d0120efb67da96e265d81bb849d8:38"),
    ("preview:retained record model_states[0].state_kind", "37e47861edf4c99a70c0e2513fc6d386a62547cb0a25f064d10785d204386944:19"),
    ("preview:retained record#0 reply", "11629b4325ea9d05e518c0256e23502db11de514b40f99320fc755eea4404cf3:1563"),
    ("preview:retained record#1 reply", "84ffd168a9a01bfcb2e5decaba40b26ee9653f23a15782d83f53c6814a12f8be:1548"),
    ("preview:retained recorded read#0 legacy record", "1de3595da31d923797e8ef2b6475522230c404b2cddb5795a2111259366333c4:233161"),
    ("preview:retained recorded read#0 reply", "9185f1ca775d74630217e410a0988af02030ee7c83dad28823025fe472f877db:11323"),
    ("preview:retained recorded read#1 legacy record", "423cd118b138fd2a65b182033e8488a5102e4bb52952725421dc4bd4eb420ccc:233161"),
    ("preview:retained recorded read#1 reply", "0f7887d34e759b808ed8cf1749926dc2b2f96787e9bd39417b6105db40edcfb4:11323"),
    ("preview:retained reopen reply", "fa8e9d89b89c8fa5ff811931863c51420addf32425c704bf67e9bc2dd3c93d62:10281"),
    ("preview:retained resave local_projects[0].analysis_run_json", "47ce4f74fa2c4d82622ce7ec78bff1e8e6bcd83cc6f572b99e1d760ba127e08d:233184"),
    ("preview:retained resave local_projects[0].created_at_unix", "Integer(1793000000)"),
    ("preview:retained resave local_projects[0].editor_intents_json", "f21abe4b59cfb3b91e8df7df4f65f7f324e44423901aaf53e2fbfa1a7160642f:125"),
    ("preview:retained resave local_projects[0].mechanics_result_json", "f3777ffbfbabf817e87877157258a443e8609249885cf2293666cca501d26ae9:135586"),
    ("preview:retained resave local_projects[0].model_hash_json", "6f891091cd88740cee9560b29a3b323d3fbf5099b2cc6340150c260785e60ade:269"),
    ("preview:retained resave local_projects[0].model_json", "af5f9efba3803af453bc5604c5993b8240f37874fb204a5abf03d502750567fb:2732"),
    ("preview:retained resave local_projects[0].model_migration_ledger_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:retained resave local_projects[0].project_envelope_hash_json", "bba6ba97767bfc5a74f6495aeff91ae55524c7e9c50af1ce0eb426ccc3102e7a:350"),
    ("preview:retained resave local_projects[0].project_id", "d8225a4b1710080c08117f701329684ab30b1d6fb3889f5aeb9f6fea6ad90c35:44"),
    ("preview:retained resave local_projects[0].project_name", "e0cc4a728074e1d2d4743cc8006c9dd07c1736850a300b956f7fe6ebc6ab9c63:24"),
    ("preview:retained resave local_projects[0].proposal_json", "0516c8de6da19a643e9e9bf7b789bc81dc099beab278ed3c82cffba1900ced6d:82"),
    ("preview:retained resave local_projects[0].selected_review_target_json", "b0b571f28e5724d9095fea8cdc00559629f998bc458f1f7f0225e302ef2e23c8:69"),
    ("preview:retained resave local_projects[0].updated_at_unix", "Integer(1793000000)"),
    ("preview:retained resave reply", "220856d5e76cf82c17824c3b1470992d6867757744991079f04582b2b3f1ee2e:3862"),
    ("preview:retained save local_projects[0].analysis_run_json", "47ce4f74fa2c4d82622ce7ec78bff1e8e6bcd83cc6f572b99e1d760ba127e08d:233184"),
    ("preview:retained save local_projects[0].created_at_unix", "Integer(1793000000)"),
    ("preview:retained save local_projects[0].editor_intents_json", "f21abe4b59cfb3b91e8df7df4f65f7f324e44423901aaf53e2fbfa1a7160642f:125"),
    ("preview:retained save local_projects[0].mechanics_result_json", "f3777ffbfbabf817e87877157258a443e8609249885cf2293666cca501d26ae9:135586"),
    ("preview:retained save local_projects[0].model_hash_json", "6f891091cd88740cee9560b29a3b323d3fbf5099b2cc6340150c260785e60ade:269"),
    ("preview:retained save local_projects[0].model_json", "af5f9efba3803af453bc5604c5993b8240f37874fb204a5abf03d502750567fb:2732"),
    ("preview:retained save local_projects[0].model_migration_ledger_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:retained save local_projects[0].project_envelope_hash_json", "bba6ba97767bfc5a74f6495aeff91ae55524c7e9c50af1ce0eb426ccc3102e7a:350"),
    ("preview:retained save local_projects[0].project_id", "d8225a4b1710080c08117f701329684ab30b1d6fb3889f5aeb9f6fea6ad90c35:44"),
    ("preview:retained save local_projects[0].project_name", "e0cc4a728074e1d2d4743cc8006c9dd07c1736850a300b956f7fe6ebc6ab9c63:24"),
    ("preview:retained save local_projects[0].proposal_json", "0516c8de6da19a643e9e9bf7b789bc81dc099beab278ed3c82cffba1900ced6d:82"),
    ("preview:retained save local_projects[0].selected_review_target_json", "b0b571f28e5724d9095fea8cdc00559629f998bc458f1f7f0225e302ef2e23c8:69"),
    ("preview:retained save local_projects[0].updated_at_unix", "Integer(1793000000)"),
    ("preview:retained save reply", "e661bbf03d7e6d73ca96562c1b5c626ed7d4c7940563ad1bb9d7b65880cac6e6:2987"),
    ("preview:rule-pack-excluded create reply", "64203b07cec88b35cec01b5d1a812e9ae6c3417031b0622824ae426a094549da:3906"),
    ("preview:rule-pack-excluded record analysis_run_labels[0].assumptions_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:rule-pack-excluded record analysis_run_labels[0].notes_json", "5df6d4f94dea81443474cd8fd8b70333654763c9dfbea861d2428c19e7d00a73:18"),
    ("preview:rule-pack-excluded record analysis_run_labels[0].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:rule-pack-excluded record analysis_run_labels[0].recording_id", "da8a9b842638ceba38dcf9bbac7fe8ea2e4d4560c344c67cad0c65ee87de94f5:39"),
    ("preview:rule-pack-excluded record analysis_run_labels[0].run_name", "a89023ff6219b358cf8bbb555afdc6d5f4303df5bfe187d902c3622679b8092f:22"),
    ("preview:rule-pack-excluded record analysis_run_labels[0].tags_json", "c38f1b165048a14cf0612830edb0f61f56be4cb4f1f3d01fbaa14e6655c52c55:10"),
    ("preview:rule-pack-excluded record analysis_run_labels[0].updated_at", "04199ee21ddec3cb0080ba7ae86be72bbf19e53d97bc7b59ff3f343c25431d0e:24"),
    ("preview:rule-pack-excluded record analysis_run_labels[1].assumptions_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:rule-pack-excluded record analysis_run_labels[1].notes_json", "5df6d4f94dea81443474cd8fd8b70333654763c9dfbea861d2428c19e7d00a73:18"),
    ("preview:rule-pack-excluded record analysis_run_labels[1].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:rule-pack-excluded record analysis_run_labels[1].recording_id", "3c9822029af07de1774314997c5803cb1897b972e4ea2ea2424024212bf0e75c:39"),
    ("preview:rule-pack-excluded record analysis_run_labels[1].run_name", "a89023ff6219b358cf8bbb555afdc6d5f4303df5bfe187d902c3622679b8092f:22"),
    ("preview:rule-pack-excluded record analysis_run_labels[1].tags_json", "c38f1b165048a14cf0612830edb0f61f56be4cb4f1f3d01fbaa14e6655c52c55:10"),
    ("preview:rule-pack-excluded record analysis_run_labels[1].updated_at", "1ac8477c538da6e93db66aa8df1bdf6f18a246ad755914704dcf06733e26b2fc:24"),
    ("preview:rule-pack-excluded record analysis_runs[0].created_at", "04199ee21ddec3cb0080ba7ae86be72bbf19e53d97bc7b59ff3f343c25431d0e:24"),
    ("preview:rule-pack-excluded record analysis_runs[0].input_manifest_json", "dcd76d60ffaf01c46e92fed2bd5aab5aa2298670617aec581094d4e51e1a4300:7195"),
    ("preview:rule-pack-excluded record analysis_runs[0].producer_run_id", "b3862e6694684d4c24986f49ec8e862f3872f1ae70359b7f55b3a8f340507b70:29"),
    ("preview:rule-pack-excluded record analysis_runs[0].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:rule-pack-excluded record analysis_runs[0].recording_id", "da8a9b842638ceba38dcf9bbac7fe8ea2e4d4560c344c67cad0c65ee87de94f5:39"),
    ("preview:rule-pack-excluded record analysis_runs[0].result_hash", "a8a20c6b217680871c9035cf81cfbcbbb6d84b058908d3f7ee90afdfbc4b360b:64"),
    ("preview:rule-pack-excluded record analysis_runs[0].result_json", "76cb429be9d4557ca4b08467853a5d0f054cc5152438ecc7b13917621c680431:71477"),
    ("preview:rule-pack-excluded record analysis_runs[0].run_record_hash", "ce9048b634f6b5ca1f86fabe3e9ae6ac748427479294a54b2dd1bc794eb3e1c7:64"),
    ("preview:rule-pack-excluded record analysis_runs[0].run_record_json", "f40614b0f010dcc2df75550d7bd7718541b59775d14036c374f8ea65828b7d85:144051"),
    ("preview:rule-pack-excluded record analysis_runs[0].state_id", "3a3cd6490a1651ab2f9fd668c3dc0d5628c6d0120efb67da96e265d81bb849d8:38"),
    ("preview:rule-pack-excluded record analysis_runs[1].created_at", "1ac8477c538da6e93db66aa8df1bdf6f18a246ad755914704dcf06733e26b2fc:24"),
    ("preview:rule-pack-excluded record analysis_runs[1].input_manifest_json", "dcd76d60ffaf01c46e92fed2bd5aab5aa2298670617aec581094d4e51e1a4300:7195"),
    ("preview:rule-pack-excluded record analysis_runs[1].producer_run_id", "b3862e6694684d4c24986f49ec8e862f3872f1ae70359b7f55b3a8f340507b70:29"),
    ("preview:rule-pack-excluded record analysis_runs[1].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:rule-pack-excluded record analysis_runs[1].recording_id", "3c9822029af07de1774314997c5803cb1897b972e4ea2ea2424024212bf0e75c:39"),
    ("preview:rule-pack-excluded record analysis_runs[1].result_hash", "a8a20c6b217680871c9035cf81cfbcbbb6d84b058908d3f7ee90afdfbc4b360b:64"),
    ("preview:rule-pack-excluded record analysis_runs[1].result_json", "76cb429be9d4557ca4b08467853a5d0f054cc5152438ecc7b13917621c680431:71477"),
    ("preview:rule-pack-excluded record analysis_runs[1].run_record_hash", "0e25079aa6f7f3d061e5978e98bf20ad450fb788f1da0547c237a7b46ea7951c:64"),
    ("preview:rule-pack-excluded record analysis_runs[1].run_record_json", "52c3e4172c47d935ff354a8bb512ca237249cd5bf55be96cb0875b46c3739a75:144051"),
    ("preview:rule-pack-excluded record analysis_runs[1].state_id", "3a3cd6490a1651ab2f9fd668c3dc0d5628c6d0120efb67da96e265d81bb849d8:38"),
    ("preview:rule-pack-excluded record model_state_labels[0].assumptions_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:rule-pack-excluded record model_state_labels[0].notes_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:rule-pack-excluded record model_state_labels[0].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:rule-pack-excluded record model_state_labels[0].state_id", "3a3cd6490a1651ab2f9fd668c3dc0d5628c6d0120efb67da96e265d81bb849d8:38"),
    ("preview:rule-pack-excluded record model_state_labels[0].state_name", "d081e834436746542ed6bc8d1327bda6c85dea255b2af1f44313fa8fe075de15:11"),
    ("preview:rule-pack-excluded record model_state_labels[0].tags_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:rule-pack-excluded record model_state_labels[0].updated_at", "04199ee21ddec3cb0080ba7ae86be72bbf19e53d97bc7b59ff3f343c25431d0e:24"),
    ("preview:rule-pack-excluded record model_states[0].created_at", "04199ee21ddec3cb0080ba7ae86be72bbf19e53d97bc7b59ff3f343c25431d0e:24"),
    ("preview:rule-pack-excluded record model_states[0].model_canonical", "54ad48de41e3a1df4998d3761082670f98db0197117f8dc3823f8300e4e984be:4311"),
    ("preview:rule-pack-excluded record model_states[0].model_hash", "94ea0db9a27da1274e56303309309bdbadfdcbba7c3fe775c4066c30f080c486:71"),
    ("preview:rule-pack-excluded record model_states[0].parent_state_id", "Null"),
    ("preview:rule-pack-excluded record model_states[0].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:rule-pack-excluded record model_states[0].record_hash", "c3a5e9ad1fa5ff45041703986f7f5a9f7f88d4c84ff79c9e0628734ad4042f5f:64"),
    ("preview:rule-pack-excluded record model_states[0].record_json", "5563d357b6637ebb564f995fb2c49e3b8d9535ad0cd236e0b0745a5187cb0a5b:1718"),
    ("preview:rule-pack-excluded record model_states[0].state_id", "3a3cd6490a1651ab2f9fd668c3dc0d5628c6d0120efb67da96e265d81bb849d8:38"),
    ("preview:rule-pack-excluded record model_states[0].state_kind", "37e47861edf4c99a70c0e2513fc6d386a62547cb0a25f064d10785d204386944:19"),
    ("preview:rule-pack-excluded record#0 reply", "003035727276a32fad01e94245b6c5f20c7c96a4a62261520d413c18416f1b42:1540"),
    ("preview:rule-pack-excluded record#1 reply", "025482b04f28238925c18b08ce0140157efe4a67d641bc0ad393df2cb8fc2065:1525"),
    ("preview:rule-pack-excluded recorded read#0 legacy record", "f40614b0f010dcc2df75550d7bd7718541b59775d14036c374f8ea65828b7d85:144051"),
    ("preview:rule-pack-excluded recorded read#0 reply", "ef7f96a4edb3fc6ca3d8e63529216521f5f9c9aa0db22f74c2ec2ea6da1b0ad9:14539"),
    ("preview:rule-pack-excluded recorded read#1 legacy record", "52c3e4172c47d935ff354a8bb512ca237249cd5bf55be96cb0875b46c3739a75:144051"),
    ("preview:rule-pack-excluded recorded read#1 reply", "75d53b349933804388b1a986307ef811ce09c42da25b6999b46795b632fba3ae:14539"),
    ("preview:rule-pack-excluded reopen reply", "9fe13034ff161fde47d4485329e5f9d8b415512480927dca58f0706da885621d:12595"),
    ("preview:rule-pack-excluded resave local_projects[0].analysis_run_json", "9a2eb22bc5648ee64af9ddca16030a188486416908c4b1936d28003d184e1c85:144011"),
    ("preview:rule-pack-excluded resave local_projects[0].created_at_unix", "Integer(1793000000)"),
    ("preview:rule-pack-excluded resave local_projects[0].editor_intents_json", "f21abe4b59cfb3b91e8df7df4f65f7f324e44423901aaf53e2fbfa1a7160642f:125"),
    ("preview:rule-pack-excluded resave local_projects[0].mechanics_result_json", "5613c1d834bc0125003ddfdafe753ffeed4b065fa1ae09409aa8b468a3b2eea9:71477"),
    ("preview:rule-pack-excluded resave local_projects[0].model_hash_json", "e54c5d83195550d8ec55989c7e1f545c674d276284da517a606f33766ac85b86:260"),
    ("preview:rule-pack-excluded resave local_projects[0].model_json", "54ad48de41e3a1df4998d3761082670f98db0197117f8dc3823f8300e4e984be:4311"),
    ("preview:rule-pack-excluded resave local_projects[0].model_migration_ledger_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:rule-pack-excluded resave local_projects[0].project_envelope_hash_json", "9060c80312f1dc0f25e6766bb8bd4680038b23db3a15527b9d8451a4bc23b45a:341"),
    ("preview:rule-pack-excluded resave local_projects[0].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:rule-pack-excluded resave local_projects[0].project_name", "65a4ab50988845fb64d0c0005c5b1af7eadee7b8c9b6935eb905722cc8a07773:45"),
    ("preview:rule-pack-excluded resave local_projects[0].proposal_json", "0516c8de6da19a643e9e9bf7b789bc81dc099beab278ed3c82cffba1900ced6d:82"),
    ("preview:rule-pack-excluded resave local_projects[0].selected_review_target_json", "b0b571f28e5724d9095fea8cdc00559629f998bc458f1f7f0225e302ef2e23c8:69"),
    ("preview:rule-pack-excluded resave local_projects[0].updated_at_unix", "Integer(1793000000)"),
    ("preview:rule-pack-excluded resave reply", "78f4fa80f7447d7a5d501650f17aa4c22895b45a2097e29ef07a089eb370b6c1:4240"),
    ("preview:rule-pack-excluded save local_projects[0].analysis_run_json", "9a2eb22bc5648ee64af9ddca16030a188486416908c4b1936d28003d184e1c85:144011"),
    ("preview:rule-pack-excluded save local_projects[0].created_at_unix", "Integer(1793000000)"),
    ("preview:rule-pack-excluded save local_projects[0].editor_intents_json", "f21abe4b59cfb3b91e8df7df4f65f7f324e44423901aaf53e2fbfa1a7160642f:125"),
    ("preview:rule-pack-excluded save local_projects[0].mechanics_result_json", "5613c1d834bc0125003ddfdafe753ffeed4b065fa1ae09409aa8b468a3b2eea9:71477"),
    ("preview:rule-pack-excluded save local_projects[0].model_hash_json", "e54c5d83195550d8ec55989c7e1f545c674d276284da517a606f33766ac85b86:260"),
    ("preview:rule-pack-excluded save local_projects[0].model_json", "54ad48de41e3a1df4998d3761082670f98db0197117f8dc3823f8300e4e984be:4311"),
    ("preview:rule-pack-excluded save local_projects[0].model_migration_ledger_json", "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945:2"),
    ("preview:rule-pack-excluded save local_projects[0].project_envelope_hash_json", "9060c80312f1dc0f25e6766bb8bd4680038b23db3a15527b9d8451a4bc23b45a:341"),
    ("preview:rule-pack-excluded save local_projects[0].project_id", "d1e9c929ebcea16d7d74493a6093d073ecc2601f8f16e7f1daf9b10a1828862e:35"),
    ("preview:rule-pack-excluded save local_projects[0].project_name", "65a4ab50988845fb64d0c0005c5b1af7eadee7b8c9b6935eb905722cc8a07773:45"),
    ("preview:rule-pack-excluded save local_projects[0].proposal_json", "0516c8de6da19a643e9e9bf7b789bc81dc099beab278ed3c82cffba1900ced6d:82"),
    ("preview:rule-pack-excluded save local_projects[0].selected_review_target_json", "b0b571f28e5724d9095fea8cdc00559629f998bc458f1f7f0225e302ef2e23c8:69"),
    ("preview:rule-pack-excluded save local_projects[0].updated_at_unix", "Integer(1793000000)"),
    ("preview:rule-pack-excluded save reply", "6a76c9dcf1c4647faffa967701f80ac4a9d5231517df6a2e63987b780454af2e:3415"),
];

