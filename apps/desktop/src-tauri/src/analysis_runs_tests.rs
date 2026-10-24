//! Store tests of recorded analysis runs (W-12 S2; DEL-14-01 Design §3 and §6).
use super::*;
use crate::command_ingress::test_support::{admit_json, check_decode_bound, headers, registry};
use crate::command_ingress::{admit_body_within, INGRESS_ARGS_JSON_V1};
use crate::memory_pool::test_host::ScriptedHost;
use crate::model_states::{labels_from_value, save_state_in_store, SaveModelStateRequest};
use crate::solve_jobs::SolveJobError;
use crate::stored_read::{self, Metered};
use serde::Serialize;
use std::cell::Cell;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::ipc::InvokeBody;

const PROJECT: &str = "project:w12-invented-runs";
const BASE: &str = "sha256:base-1";
const RUN: &str = "run:preview-linear-static-001";
/// 2026-10-27T11:30:45.123Z.
const T0: i64 = 1_793_100_645_123;

/// The TypeScript-built transient record of a real producer result
/// (`fixtures/persistence/transient_run_input.json`, written by
/// `recordedRunParity.test.ts`) and the witness this module's Rust path records from it.
const TRANSIENT_INPUT: &str = include_str!("../../../../fixtures/persistence/transient_run_input.json");
const SOURCE_RESULT: &str = include_str!("../../../../fixtures/results/preview_physics_connected_sparse.json");
const WITNESS: &str = include_str!("../../../../fixtures/persistence/recorded_run_witness.json");
const WITNESS_PATH_ENV: &str = "OPENPIPESTRESS_WRITE_RECORDED_RUN_WITNESS";

/// An invented 0.7.0 document of `PROJECT` (`name` makes distinct models).
fn model(name: &str) -> Value {
    json!({
        "schema_version": open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION,
        "document_kind": "openpipestress.product_preview.model",
        "project": { "id": PROJECT, "name": name, "units": { "length": "m" } },
        "nodes": [{ "id": "node:N-100", "coordinates": [0, 0, 0] }],
        "pipe_segments": [], "supports": [], "materials": [], "sections": [], "components": [],
        "load_cases": [], "combinations": []
    })
}

fn model_hash(model: &Value) -> String {
    format!("sha256:{}", sha256_hex(&canonical_json(model)))
}

/// An input manifest of `model` as `inputManifestService.ts` builds it (its canonical text).
fn manifest_text(model: &Value) -> String {
    canonical_json(&json!({
        "schema_version": "1.0.0",
        "document_kind": "openpipestress.current_session_input_manifest",
        "model_basis": { "model_ref": PROJECT, "model_payload": model },
        "unit_basis": { "project_units": { "length": "m" } },
        "solver_basis": { "solver_name": "invented-solver", "solver_version": "0.0.1", "solver_build_ref": "invented-solver@0.0.1",
                          "solver_mode": "sparse_interactive", "settings": { "sparse_evidence_lane": true } },
        "load_basis": { "load_cases": [], "combinations": [] },
        "active_rule_packs": [{ "rule_pack_id": "rule-pack:invented", "version": "1.0.0", "checksum_sha256": "a".repeat(64), "source_notice": "invented" }],
        "external_assets": [],
        "identity_policy": { "canonicalization": "rfc8785_jcs", "hash_algorithm": "sha256", "canonical_bytes_scope": "entire_input_manifest_object" },
        "replay_boundary": { "included_as_package_member": false, "portable_replay_claimed": false, "current_session_ref_hash_integrity_only": true }
    }))
}

/// A received result's text, deliberately not canonical (whitespace, a
/// negative zero): it is stored as given.
fn result_text(value: f64) -> String {
    format!(
        "{{ \"run_id\": \"{RUN}\", \"model_ref\": \"{PROJECT}\", \"results\": [ {{ \"id\": \"result:disp:N-100\", \"value\": {value}, \"offset\": -0 }} ], \"diagnostics\": [] }}"
    )
}

/// A transient record whose bindings Rust checks: its own checksum and exact
/// checksum inventory, the received result's checked hash and its rows'
/// checksums, and the members the manifest fixes (the record's semantics are
/// the TypeScript and Python validators' and are not re-derived here).
fn transient(result: &str, manifest: &str) -> Value {
    let manifest_hash = sha256_hex(manifest);
    let manifest_ref = json!({ "object_type": "InputManifest", "ref": format!("input-manifest:project-w12-invented-runs:{manifest_hash}") });
    let rows: Value = serde_json::from_str(result).unwrap();
    let result_refs: Vec<Value> = rows["results"].as_array().unwrap().iter().enumerate().map(|(index, row)| {
        let id = row["id"].as_str().unwrap();
        json!({ "result_ref": { "object_type": "Result", "ref": id }, "source_row_index": index,
                "hash_refs": [{ "algorithm": "sha256", "canonicalization": CHECKED_PROFILE_V1, "payload_ref": { "object_type": "Result", "ref": id },
                                "payload_scope": "result_row", "value": checked_sha256(row).unwrap() }] })
    }).collect();
    let mut record = json!({
        "schema_version": "0.3.0",
        "run_contract_status": { "record_contract": "strict_analysis_run_v0_3", "result_binding": "received_mechanics_result",
                                 "external_validation_boundary": "reference_only_not_determined_by_software" },
        "analysis_run": {
            "run_id": RUN, "run_name": format!("{RUN} analysis record"), "run_kind": "mechanics_solve", "created_at": null,
            "model_state_ref": { "object_type": "ModelState", "ref": format!("state:{PROJECT}:preview") },
            "solver_version": { "solver_name": "invented-solver", "solver_version": "0.0.1",
                                "build_ref": { "object_type": "ExternalReference", "ref": "invented-solver@0.0.1" } },
            "settings_ref": { "object_type": "SolverSettings", "ref": format!("solver-settings:input-manifest:project-w12-invented-runs:{manifest_hash}:{manifest_hash}") },
            "unit_system_ref": { "object_type": "UnitSystem", "ref": format!("unit-system:{PROJECT}:{manifest_hash}") },
            "load_basis_refs": [], "result_refs": result_refs,
            "rule_pack_refs": [], "library_refs": [],
            "analysis_status": ["HUMAN_REVIEW_REQUIRED", "MECHANICS_SOLVED", "RULE_INPUTS_INCOMPLETE"],
            "hashes": [
                { "algorithm": "sha256", "canonicalization": CHECKED_PROFILE_V1, "payload_ref": { "object_type": "AnalysisRun", "ref": RUN },
                  "payload_scope": "analysis_run_record", "value": "" },
                { "algorithm": "sha256", "canonicalization": CHECKED_PROFILE_V1, "payload_ref": { "object_type": "ResultEnvelope", "ref": format!("result-envelope:{RUN}") },
                  "payload_scope": "received_result", "value": sha256_hex(&canonical_json_checked_v1_text(result).unwrap()) }
            ],
            "reproducibility": {
                "input_manifest_refs": [manifest_ref.clone()],
                "input_manifest_hashes": [{ "algorithm": "sha256", "canonicalization": "rfc8785_jcs", "payload_ref": manifest_ref,
                                            "payload_scope": "input_manifest", "value": manifest_hash }],
                "determinism_notes": ["created_at_unavailable", "model_state_ref_and_solver_settings_unit_basis_bound_by_input_manifest"]
            },
            "scale": 0.1
        }
    });
    reseal(&mut record);
    record
}

/// Recomputes a record's own checksum value after an edit (a coherent reseal).
fn reseal(record: &mut Value) {
    let hash = run_record_hash(record).unwrap();
    for entry in record["analysis_run"]["hashes"].as_array_mut().unwrap() {
        if entry["payload_scope"] == "analysis_run_record" {
            entry["value"] = json!(hash);
        }
    }
}

fn scratch_store(label: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "swbpipe-runs-{label}-{}-{}",
        std::process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    ));
    std::fs::create_dir_all(&root).unwrap();
    root.join(crate::PROJECT_STORE_FILE)
}

fn write_project(connection: &mut Connection, project_id: &str, model: &Value, base: &str) {
    crate::upsert_project(
        connection, project_id, "Invented project", model, &json!([]), &Value::Null, &Value::Null,
        &Value::Null, &json!({ "transient": "slot" }), &Value::Null, &json!({ "value": base }),
    )
    .unwrap();
}

fn store_with_project(label: &str) -> PathBuf {
    let path = scratch_store(label);
    let (mut connection, _) = crate::open_project_store(&path).unwrap();
    write_project(&mut connection, PROJECT, &model("Invented project"), BASE);
    path
}

fn save_state(path: &Path, model: &Value, name: &str, token: &'static str, millis: i64) -> ModelStateSummary {
    let model_canonical = canonical_json(model);
    let request = SaveModelStateRequest {
        project_id: PROJECT.to_string(),
        expected_base: BASE.to_string(),
        model_hash: format!("sha256:{}", sha256_hex(&model_canonical)),
        model_canonical,
        state_kind: "design_snapshot".to_string(),
        parent_state_id: None,
        labels: labels_from_value(json!({ "state_name": name })).unwrap(),
    };
    let allocate = move || Ok((token.to_string(), millis));
    let between_rows = |_: &Transaction<'_>| Ok(());
    save_state_in_store(path, request, &StateWriteHooks { allocate: &allocate, between_rows: &between_rows })
        .unwrap()
        .state
}

fn run_labels(name: &str) -> RunLabels {
    run_labels_from_value(json!({
        "run_name": name, "tags": ["tie-in", "révision-B"], "notes": ["Δp case – invented."],
        "assumptions": [{ "assumption_id": "assumption:R-1", "kind": "assumption", "status": "unresolved",
                          "statement": "Solved with the vendor's nozzle stiffness pending.", "affected_refs": ["node:N-100"] }]
    }))
    .unwrap()
}

fn request(model: &Value, name: &str) -> RecordRunRequest {
    let manifest = manifest_text(model);
    let result = result_text(1.25);
    RecordRunRequest {
        project_id: PROJECT.to_string(),
        expected_base: BASE.to_string(),
        run_record: transient(&result, &manifest),
        result_json: result,
        input_manifest_canonical: manifest,
        state_id: None,
        new_state: None,
        run_labels: run_labels(name),
    }
}

fn with_new_state(mut request: RecordRunRequest, name: &str) -> RecordRunRequest {
    request.new_state = Some(NewStateRequest {
        labels: labels_from_value(json!({ "state_name": name })).unwrap(),
        parent_state_id: None,
    });
    request
}

fn record(path: &Path, request: RecordRunRequest, millis: i64) -> Result<RecordRunOutcome, String> {
    let now = move || Ok(millis);
    let token = || Ok("0000000000000000000000000000beef".to_string());
    let between_rows = |_: &Transaction<'_>, _: &str| Ok(());
    record_run_in_store(path, request, &RunWriteHooks { now: &now, state_token: &token, between_rows: &between_rows })
}

fn row_count(path: &Path, table: &str) -> i64 {
    Connection::open(path)
        .unwrap()
        .query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |row| row.get(0))
        .unwrap()
}

fn project_row(path: &Path) -> (String, String) {
    Connection::open(path)
        .unwrap()
        .query_row(
            "SELECT analysis_run_json, project_envelope_hash_json FROM local_projects WHERE project_id = ?1",
            params![PROJECT],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .unwrap()
}

#[test]
fn v13_migrates_a_v12_store_with_states_and_is_safe_to_run_twice() {
    let path = store_with_project("v12");
    let state = save_state(&path, &model("Invented project"), "Basis", "000000000000000000000000000000a1", 1_000);
    // A v12 store: S1's tables with states, none of v13's objects.
    Connection::open(&path)
        .unwrap()
        .execute_batch(
            "DROP TRIGGER analysis_runs_no_replace; DROP TRIGGER analysis_runs_immutable; DROP TABLE analysis_run_labels;
             DROP INDEX analysis_runs_by_state; DROP TABLE analysis_runs; PRAGMA user_version = 12;",
        )
        .unwrap();
    let (connection, evidence) = crate::open_project_store(&path).unwrap();
    assert_eq!(evidence.store_schema_version_before_open, 12);
    assert_eq!(evidence.store_schema_version, 13);
    assert_eq!(evidence.migrations_applied_on_open, vec!["store-v13-analysis-runs-tables"]);
    let listed = model_states::list_states(&connection, PROJECT).unwrap();
    assert_eq!(listed, vec![ModelStateSummary { recorded_run_ids: vec![], ..state.clone() }]);
    drop(connection);
    let outcome = record(&path, request(&model("Invented project"), "Run 1"), T0).unwrap();
    // A re-run (as an interrupted, non-transactional migration would) changes nothing.
    Connection::open(&path).unwrap().execute_batch("PRAGMA user_version = 12;").unwrap();
    let (connection, evidence) = crate::open_project_store(&path).unwrap();
    assert_eq!(evidence.migrations_applied_on_open, vec!["store-v13-analysis-runs-tables"]);
    migrate_v13_analysis_runs(&connection).unwrap();
    assert_eq!(list_runs(&connection, PROJECT).unwrap(), vec![outcome.run.clone()]);
    assert_eq!(model_states::list_states(&connection, PROJECT).unwrap()[0].recorded_run_ids, vec![outcome.run.recording_id.clone()]);
    let error = connection.execute("UPDATE analysis_runs SET created_at = created_at", []).unwrap_err().to_string();
    assert!(error.contains("ANALYSIS-RUN-IMMUTABLE"), "the trigger survives the re-run: {error}");
    drop(connection);
    // A store from a newer build is refused by name.
    Connection::open(&path).unwrap().execute_batch("PRAGMA user_version = 14;").unwrap();
    let error = crate::open_project_store(&path).map(|_| ()).unwrap_err();
    assert!(error.starts_with("STORE-SCHEMA-NEWER: the local project store is at store schema v14, newer than this build's v13"), "{error}");
}

#[test]
fn a_run_is_recorded_against_the_state_of_its_model_listed_and_read() {
    let path = store_with_project("create");
    let solved = model("Invented project");
    let state = save_state(&path, &solved, "Basis", "000000000000000000000000000000b1", 1_000);
    let before = project_row(&path);
    let submitted = request(&solved, "Run – rev A");
    let transient_record = submitted.run_record.clone();
    let result_bytes = submitted.result_json.clone();
    let manifest = submitted.input_manifest_canonical.clone();
    let outcome = record(&path, submitted, T0).unwrap();
    assert_eq!(outcome.state_outcome, "existing");
    assert_eq!(outcome.state.state_id, state.state_id);
    assert_eq!(outcome.state.recorded_run_ids, vec![outcome.run.recording_id.clone()]);
    let run = &outcome.run;
    assert_eq!(run.created_at, "2026-10-27T11:30:45.123Z");
    assert_eq!(run.labels_updated_at, run.created_at);
    assert_eq!(run.producer_run_id, RUN, "the producer's run id is kept as data");
    assert_eq!(run.recording_id, format!("runrec:{}", &run.run_record_hash[..32]));
    assert_eq!(run.run_record_hash.len(), 64);
    assert_eq!(run.labels, run_labels("Run – rev A"));
    assert_eq!(outcome.message, format!(
        "Recorded run \"Run – rev A\" ({}) against model state \"Basis\" ({}). Renaming, tagging or adding notes does not change the recorded run.",
        run.recording_id, state.state_id
    ));
    assert_eq!(project_row(&path), before, "the current-run slot and the envelope hash are untouched");
    let (connection, _) = crate::open_project_store(&path).unwrap();
    assert_eq!(list_runs(&connection, PROJECT).unwrap(), vec![run.clone()]);
    let read = read_run_in_store(&path, PROJECT, &run.recording_id).unwrap();
    assert_eq!(read.result_json, result_bytes, "the received-result bytes are stored as submitted");
    assert_eq!(read.input_manifest_canonical, manifest);
    assert_eq!(read.state.state_id, state.state_id);
    let recorded = &read.record["analysis_run"];
    assert_eq!(recorded["created_at"], "2026-10-27T11:30:45.123Z");
    assert_eq!(recorded["model_state_ref"], json!({ "object_type": "ModelState", "ref": state.state_id }));
    assert_eq!(recorded["rule_pack_refs"], json!([{ "object_type": "RulePack", "ref": "rule-pack:invented@1.0.0" }]));
    assert_eq!(recorded["library_refs"], json!([]));
    assert_eq!(recorded["reproducibility"]["determinism_notes"], json!(["model_state_ref_and_solver_settings_unit_basis_bound_by_input_manifest"]));
    assert_eq!(recorded["hashes"][0]["value"], json!(run.run_record_hash));
    assert_eq!(recorded["hashes"][1], transient_record["analysis_run"]["hashes"][1], "the received-result checksum is unchanged");
    assert_eq!(run_record_hash(&read.record).unwrap(), run.run_record_hash);
    // Every other member is the transient record's.
    let mut expected = transient_record.clone();
    for key in ["created_at", "model_state_ref", "rule_pack_refs", "library_refs", "hashes", "reproducibility"] {
        expected["analysis_run"][key] = recorded[key].clone();
    }
    assert_eq!(read.record, expected);
    assert_eq!(recorded["reproducibility"]["input_manifest_hashes"], transient_record["analysis_run"]["reproducibility"]["input_manifest_hashes"]);
}

#[test]
fn a_run_of_an_unsaved_model_needs_a_post_solve_snapshot_written_with_it() {
    let path = store_with_project("snapshot");
    let solved = model("Invented project – solved");
    let error = record(&path, request(&solved, "Run"), T0).unwrap_err();
    assert!(error.starts_with("ANALYSIS-RUN-STATE-REQUIRED: project `project:w12-invented-runs` has no saved state of the solved model"), "{error}");
    assert_eq!(row_count(&path, "model_states"), 0);
    let outcome = record(&path, with_new_state(request(&solved, "Run"), "After solve"), T0).unwrap();
    assert_eq!(outcome.state_outcome, "created");
    assert_eq!(outcome.state.state_kind, POST_SOLVE_SNAPSHOT);
    assert_eq!(outcome.state.state_id, "state:0000000000000000000000000000beef");
    assert_eq!(outcome.state.labels.state_name, "After solve");
    assert_eq!(outcome.state.model_hash, model_hash(&solved));
    assert_eq!(outcome.state.recorded_run_ids, vec![outcome.run.recording_id.clone()]);
    assert!(outcome.message.contains("against the new post-solve snapshot \"After solve\""), "{}", outcome.message);
    let opened = model_states::read_state_in_store(&path, PROJECT, &outcome.state.state_id).unwrap();
    assert_eq!(opened.model, solved, "the snapshot is the solved model, from the manifest");
    // A second recording of that model is tied to the snapshot; a name given for
    // a new snapshot is not used.
    let again = record(&path, with_new_state(request(&solved, "Run 2"), "Unused"), T0).unwrap();
    assert_eq!(again.state_outcome, "existing");
    assert_eq!(again.state.state_id, outcome.state.state_id);
    assert!(again.message.contains("the post-solve snapshot name was not used"), "{}", again.message);
    assert_eq!(row_count(&path, "model_states"), 1);
}

#[test]
fn the_state_its_labels_the_run_and_its_labels_are_one_transaction() {
    for stage in ["state", "run"] {
        let path = store_with_project(&format!("rollback-{stage}"));
        let reached = Cell::new(false);
        let now = || Ok(T0);
        let token = || Ok("0000000000000000000000000000f00d".to_string());
        let between_rows = |transaction: &Transaction<'_>, at: &str| {
            if at != stage {
                return Ok(());
            }
            let written: i64 = transaction.query_row("SELECT COUNT(*) FROM model_states", [], |row| row.get(0)).unwrap();
            assert_eq!(written, 1, "the snapshot row is written inside the transaction");
            reached.set(true);
            Err("INJECTED-FAILURE".to_string())
        };
        let error = record_run_in_store(
            &path,
            with_new_state(request(&model("Solved"), "Run"), "After solve"),
            &RunWriteHooks { now: &now, state_token: &token, between_rows: &between_rows },
        )
        .unwrap_err();
        assert_eq!(error, "INJECTED-FAILURE");
        assert!(reached.get(), "{stage}");
        for table in ["model_states", "model_state_labels", "analysis_runs", "analysis_run_labels"] {
            assert_eq!(row_count(&path, table), 0, "{stage}: {table} rolled back");
        }
    }
}

#[test]
fn a_recorded_run_is_immutable_and_insert_or_replace_cannot_overwrite_it() {
    let path = store_with_project("immutable");
    let solved = model("Invented project");
    save_state(&path, &solved, "Basis", "000000000000000000000000000000c1", 1_000);
    let outcome = record(&path, request(&solved, "Run"), T0).unwrap();
    let connection = Connection::open(&path).unwrap();
    connection.execute_batch("PRAGMA foreign_keys = ON;").unwrap();
    for statement in [
        "UPDATE analysis_runs SET result_json = '{}'",
        "UPDATE analysis_runs SET run_record_hash = run_record_hash",
        "UPDATE analysis_runs SET created_at = '1970-01-01T00:00:00.000Z'",
    ] {
        let error = connection.execute(statement, []).unwrap_err().to_string();
        assert!(error.contains("ANALYSIS-RUN-IMMUTABLE"), "{statement}: {error}");
    }
    let before: String = connection.query_row("SELECT run_record_json FROM analysis_runs", [], |row| row.get(0)).unwrap();
    for verb in ["INSERT OR REPLACE INTO", "REPLACE INTO", "INSERT INTO"] {
        // Same recording id, other content.
        let error = connection
            .execute(
                &format!("{verb} analysis_runs SELECT project_id, recording_id, state_id, producer_run_id, created_at, '{{}}', 'other', input_manifest_json, result_json, result_hash FROM analysis_runs"),
                [],
            )
            .unwrap_err()
            .to_string();
        assert!(error.contains("ANALYSIS-RUN-IMMUTABLE"), "{verb}, same recording id: {error}");
        // Another recording id, the same record hash (the UNIQUE key REPLACE would resolve by deleting).
        let error = connection
            .execute(
                &format!("{verb} analysis_runs SELECT project_id, 'runrec:other', state_id, producer_run_id, created_at, run_record_json, run_record_hash, input_manifest_json, result_json, result_hash FROM analysis_runs"),
                [],
            )
            .unwrap_err()
            .to_string();
        assert!(error.contains("ANALYSIS-RUN-IMMUTABLE"), "{verb}, same record hash: {error}");
    }
    let after: String = connection.query_row("SELECT run_record_json FROM analysis_runs", [], |row| row.get(0)).unwrap();
    assert_eq!(after, before);
    assert_eq!(row_count(&path, "analysis_run_labels"), 1, "the labels are not cascade-deleted");
    // UNIQUE refuses on its own: a duplicate record hash is never written over.
    connection.execute_batch("DROP TRIGGER analysis_runs_no_replace;").unwrap();
    let error = connection
        .execute("INSERT INTO analysis_runs SELECT project_id, 'runrec:other', state_id, producer_run_id, created_at, run_record_json, run_record_hash, input_manifest_json, result_json, result_hash FROM analysis_runs", [])
        .unwrap_err()
        .to_string();
    assert!(error.contains("UNIQUE constraint failed: analysis_runs.project_id, analysis_runs.run_record_hash"), "{error}");
    assert!(read_run_in_store(&path, PROJECT, &outcome.run.recording_id).is_ok());
}

#[test]
fn a_label_edit_changes_only_the_labels_and_leaves_the_record_hash() {
    let path = store_with_project("labels");
    let solved = model("Invented project");
    save_state(&path, &solved, "Basis", "000000000000000000000000000000d1", 1_000);
    let outcome = record(&path, request(&solved, "Run"), T0).unwrap();
    let stored = || -> (String, String, String, String) {
        Connection::open(&path)
            .unwrap()
            .query_row("SELECT run_record_json, run_record_hash, result_json, input_manifest_json FROM analysis_runs", [], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
            })
            .unwrap()
    };
    let before = stored();
    let renamed = run_labels_from_value(json!({ "run_name": "Run – renamed", "tags": ["für-review"] })).unwrap();
    let edited = edit_run_labels_in_store(&path, PROJECT, &outcome.run.recording_id, renamed.clone(), Some(&outcome.run.labels_updated_at), 5).unwrap();
    assert_eq!(edited.labels, renamed);
    assert_eq!(edited.run_record_hash, outcome.run.run_record_hash);
    assert_eq!(edited.labels_updated_at, "2026-10-27T11:30:45.124Z", "strictly later than the last write");
    assert_eq!(stored(), before);
    let error = edit_run_labels_in_store(&path, PROJECT, &outcome.run.recording_id, run_labels("Other"), Some(&outcome.run.labels_updated_at), T0 + 9_000).unwrap_err();
    assert!(error.starts_with("ANALYSIS-RUN-LABELS-MOVED"), "{error}");
    for (labels, expected) in [
        (json!({ "tags": [] }), "a recorded run needs a name"),
        (json!({ "run_name": "n", "state_name": "s" }), "unknown member `state_name`"),
        (json!({ "run_name": "n", "assumptions": [{ "assumption_id": "a:1", "kind": "assumption", "status": "rejected", "statement": "s" }] }), "needs a resolution note"),
    ] {
        let error = run_labels_from_value(labels).unwrap_err();
        assert!(error.starts_with("ANALYSIS-RUN-LABELS-INVALID: ") && error.contains(expected), "{error}");
    }
}

#[test]
fn a_state_with_a_recorded_run_is_not_deleted_and_its_blockers_are_named() {
    let path = store_with_project("delete");
    let solved = model("Invented project");
    let state = save_state(&path, &solved, "Basis", "000000000000000000000000000000e1", 1_000);
    let first = record(&path, request(&solved, "Run 1"), T0).unwrap();
    let second = record(&path, request(&solved, "Run 2"), T0).unwrap();
    let error = model_states::delete_state_in_store(&path, PROJECT, &state.state_id).unwrap_err();
    assert_eq!(error, format!(
        "MODEL-STATE-DELETE-BLOCKED: state \"Basis\" ({}) is the state of recorded runs \"Run 1\" ({}), \"Run 2\" ({}); delete them first. Nothing was deleted.",
        state.state_id, first.run.recording_id, second.run.recording_id
    ));
    // The foreign key refuses it on its own.
    let connection = Connection::open(&path).unwrap();
    connection.execute_batch("PRAGMA foreign_keys = ON;").unwrap();
    let error = connection
        .execute("DELETE FROM model_states WHERE state_id = ?1", params![state.state_id])
        .unwrap_err()
        .to_string();
    assert!(error.contains("FOREIGN KEY constraint failed"), "{error}");
    drop(connection);
    let receipt = delete_run_in_store(&path, PROJECT, &first.run.recording_id).unwrap();
    assert!(receipt.deleted);
    assert_eq!(receipt.message, format!("Deleted recorded run \"Run 1\" ({}).", first.run.recording_id));
    assert!(read_run_in_store(&path, PROJECT, &first.run.recording_id).unwrap_err().starts_with("ANALYSIS-RUN-NOT-FOUND"));
    assert_eq!(row_count(&path, "analysis_run_labels"), 1, "its labels went with it");
    delete_run_in_store(&path, PROJECT, &second.run.recording_id).unwrap();
    assert!(model_states::delete_state_in_store(&path, PROJECT, &state.state_id).unwrap().deleted);
}

#[test]
fn deleting_the_project_row_deletes_its_runs_child_states_and_states_in_one_statement() {
    let path = store_with_project("cascade");
    let parent_model = model("Invented project");
    let parent = save_state(&path, &parent_model, "Parent", "000000000000000000000000000000f1", 1_000);
    let child_model = model("Invented project – child");
    let mut child_request = with_new_state(request(&child_model, "Child run"), "Child");
    child_request.new_state.as_mut().unwrap().parent_state_id = Some(parent.state_id.clone());
    let child = record(&path, child_request, T0).unwrap();
    assert_eq!(child.state.parent_state_id.as_deref(), Some(parent.state_id.as_str()));
    record(&path, request(&parent_model, "Parent run"), T0).unwrap();
    // Another project's history is untouched.
    let (mut connection, _) = crate::open_project_store(&path).unwrap();
    write_project(&mut connection, "project:other", &json!({ "project": { "id": "project:other" } }), "sha256:other");
    for table in ["model_states", "model_state_labels", "analysis_runs", "analysis_run_labels"] {
        assert_eq!(row_count(&path, table), 2, "{table}");
    }
    connection.execute("DELETE FROM local_projects WHERE project_id = ?1", params![PROJECT]).unwrap();
    for table in ["model_states", "model_state_labels", "analysis_runs", "analysis_run_labels"] {
        assert_eq!(row_count(&path, table), 0, "{table}: the cascade removed it");
    }
    assert!(crate::load_project(&connection, Some("project:other")).unwrap().is_some());
}

#[test]
fn identical_solves_recorded_from_two_connections_with_the_clock_frozen_stay_distinct() {
    let path = store_with_project("concurrent");
    let solved = model("Invented project");
    save_state(&path, &solved, "Basis", "00000000000000000000000000000a01", 1_000);
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
    let handles: Vec<_> = (0..2)
        .map(|index| {
            let (path, barrier, solved) = (path.clone(), barrier.clone(), solved.clone());
            std::thread::spawn(move || {
                let request = request(&solved, &format!("Window {index}"));
                barrier.wait();
                record(&path, request, T0).unwrap()
            })
        })
        .collect();
    let mut runs: Vec<AnalysisRunSummary> = handles.into_iter().map(|handle| handle.join().unwrap().run).collect();
    runs.sort_by(|left, right| left.created_at.cmp(&right.created_at));
    assert_eq!(runs[0].created_at, "2026-10-27T11:30:45.123Z");
    assert_eq!(runs[1].created_at, "2026-10-27T11:30:45.124Z", "one millisecond after the latest");
    assert_eq!(runs[0].producer_run_id, runs[1].producer_run_id);
    assert_eq!(runs[0].result_hash, runs[1].result_hash, "identical solves");
    assert_ne!(runs[0].run_record_hash, runs[1].run_record_hash);
    assert_ne!(runs[0].recording_id, runs[1].recording_id);
    assert_eq!(row_count(&path, "analysis_runs"), 2);
    // A clock behind the latest recording still moves forward.
    let later = record(&path, request(&solved, "Behind"), T0 - 60_000).unwrap();
    assert_eq!(later.run.created_at, "2026-10-27T11:30:45.125Z");
}

#[test]
fn the_manifest_and_result_bind_the_record_to_its_state() {
    let path = store_with_project("binding");
    let solved = model("Invented project");
    let other = model("Invented project – other");
    let state = save_state(&path, &solved, "Basis", "00000000000000000000000000000b01", 1_000);
    let other_state = save_state(&path, &other, "Other", "00000000000000000000000000000b02", 2_000);
    // Good: the state named is the one the manifest's model hashes to.
    let mut good = request(&solved, "Good");
    good.state_id = Some(state.state_id.clone());
    assert_eq!(record(&path, good, T0).unwrap().state.state_id, state.state_id);
    // Wrong state.
    let mut wrong = request(&solved, "Wrong");
    wrong.state_id = Some(other_state.state_id.clone());
    let error = record(&path, wrong, T0).unwrap_err();
    assert!(error.starts_with(&format!("ANALYSIS-RUN-STATE-MISMATCH: the solved model hashes to {}, not state \"Other\"", state.model_hash)), "{error}");
    let mut absent = request(&solved, "Absent");
    absent.state_id = Some("state:absent".to_string());
    assert!(record(&path, absent, T0).unwrap_err().starts_with("ANALYSIS-RUN-STATE-NOT-FOUND"));
    // Tampered manifests: another model, or the same one not canonical.
    let mut swapped = request(&solved, "Swapped");
    swapped.input_manifest_canonical = manifest_text(&other);
    let error = record(&path, swapped, T0).unwrap_err();
    assert!(error.starts_with("ANALYSIS-RUN-MANIFEST-MISMATCH: the input manifest hashes to"), "{error}");
    let mut pretty = request(&solved, "Pretty");
    pretty.input_manifest_canonical = serde_json::to_string_pretty(&serde_json::from_str::<Value>(&pretty.input_manifest_canonical).unwrap()).unwrap();
    let error = record(&path, pretty, T0).unwrap_err();
    assert!(error.starts_with("ANALYSIS-RUN-MANIFEST-MISMATCH: the input manifest is not its own canonical JCS text"), "{error}");
    // A record that names a manifest of another project.
    let mut foreign = request(&solved, "Foreign");
    let mut foreign_model = solved.clone();
    foreign_model["project"]["id"] = json!("project:other");
    foreign.input_manifest_canonical = manifest_text(&foreign_model);
    foreign.run_record = transient(&foreign.result_json, &foreign.input_manifest_canonical);
    assert!(record(&path, foreign, T0).unwrap_err().starts_with("ANALYSIS-RUN-MANIFEST-MISMATCH: the input manifest is not of project"));
    // Another result than the record's.
    let mut result = request(&solved, "Result");
    result.result_json = result_text(1.5);
    let error = record(&path, result, T0).unwrap_err();
    assert!(error.starts_with("ANALYSIS-RUN-RESULT-MISMATCH: the result hashes to"), "{error}");
    // A record whose own checksum does not verify, and an already recorded one.
    let mut edited = request(&solved, "Edited");
    edited.run_record["analysis_run"]["run_name"] = json!("edited");
    assert!(record(&path, edited, T0).unwrap_err().starts_with("ANALYSIS-RUN-NOT-TRANSIENT: the record's own checksum does not verify"));
    let read = read_run_in_store(&path, PROJECT, &list_runs(&crate::open_project_store(&path).unwrap().0, PROJECT).unwrap()[0].recording_id).unwrap();
    let mut again = request(&solved, "Again");
    again.run_record = read.record;
    assert!(record(&path, again, T0).unwrap_err().starts_with("ANALYSIS-RUN-NOT-TRANSIENT: the record is not a solve's transient record"));
    assert_eq!(row_count(&path, "analysis_runs"), 1);
}

#[test]
fn foreign_keys_refuse_a_run_of_another_project_or_state() {
    let path = store_with_project("fk");
    let solved = model("Invented project");
    let state = save_state(&path, &solved, "Basis", "00000000000000000000000000000c01", 1_000);
    let outcome = record(&path, request(&solved, "Run"), T0).unwrap();
    let (connection, _) = crate::open_project_store(&path).unwrap();
    connection.execute("INSERT INTO local_projects (project_id, project_name, model_json, created_at_unix, updated_at_unix) VALUES ('project:other', 'o', '{}', 1, 1)", []).unwrap();
    let insert = |project: &str, state_id: &str| {
        connection
            .execute(
                "INSERT INTO analysis_runs VALUES (?1, 'runrec:x', ?2, 'run:x', 't', '{}', 'h', '{}', '{}', 'r')",
                params![project, state_id],
            )
            .unwrap_err()
            .to_string()
    };
    assert!(insert("project:other", &state.state_id).contains("FOREIGN KEY constraint failed"), "another project's state");
    assert!(insert(PROJECT, "state:absent").contains("FOREIGN KEY constraint failed"));
    assert!(insert("project:absent", &state.state_id).contains("FOREIGN KEY constraint failed"));
    let error = connection
        .execute("INSERT INTO analysis_run_labels VALUES ('project:other', ?1, 'n', '[]', '[]', '[]', 't')", params![outcome.run.recording_id])
        .unwrap_err()
        .to_string();
    assert!(error.contains("FOREIGN KEY constraint failed"), "{error}");
    // The API refuses a project without a stored row by name.
    let path = scratch_store("fk-missing");
    drop(crate::open_project_store(&path).unwrap());
    let error = record(&path, request(&solved, "Run"), T0).unwrap_err();
    assert!(error.starts_with("ANALYSIS-RUN-PROJECT-MISSING: project `project:w12-invented-runs` has no stored local project"), "{error}");
}

#[test]
fn a_stale_window_records_nothing() {
    let path = store_with_project("stale");
    let solved = model("Invented project");
    save_state(&path, &solved, "Basis", "00000000000000000000000000000d01", 1_000);
    let mut stale = with_new_state(request(&model("Unsaved"), "Run"), "After solve");
    stale.expected_base = "sha256:base-0".to_string();
    let error = record(&path, stale, T0).unwrap_err();
    assert!(error.starts_with("PROJECT-BASE-MOVED: project `project:w12-invented-runs`"), "{error}");
    assert_eq!(row_count(&path, "analysis_runs"), 0);
    assert_eq!(row_count(&path, "model_states"), 1, "no post-solve snapshot either");
}

#[test]
fn replacing_the_project_row_keeps_its_runs_and_the_prompt_counts_them() {
    let path = store_with_project("replace");
    let solved = model("Invented project");
    save_state(&path, &solved, "Basis", "00000000000000000000000000000e01", 1_000);
    record(&path, request(&solved, "One"), T0).unwrap();
    record(&path, with_new_state(request(&model("Two"), "Two"), "Two"), T0).unwrap();
    let (mut connection, _) = crate::open_project_store(&path).unwrap();
    let entry = crate::stored_project_entry(&connection, PROJECT).unwrap().unwrap();
    let revision = crate::stored_revision(&connection, &entry).unwrap();
    let transaction = crate::immediate_transaction(&mut connection).unwrap();
    crate::guarded_write_project_rows(
        &transaction, Some(&revision), None, PROJECT, "Replaced", &model("A different model"), &json!([]),
        &Value::Null, &Value::Null, &Value::Null, &Value::Null, &Value::Null, &json!({ "value": "sha256:base-3" }),
    )
    .unwrap();
    transaction.commit().unwrap();
    assert_eq!(list_runs(&connection, PROJECT).unwrap().len(), 2, "the replacement keeps the runs");
    assert_eq!(recorded_run_count(&connection, PROJECT).unwrap(), 2);
    assert_eq!(model_states::saved_state_count(&connection, PROJECT).unwrap(), 2);
    assert_eq!(recorded_run_count(&connection, "project:absent").unwrap(), 0);
}

/// Edits rows outside the store's own paths (triggers dropped, foreign keys off).
fn tamper(path: &Path, statements: &str) {
    Connection::open(path)
        .unwrap()
        .execute_batch(&format!(
            "PRAGMA foreign_keys = OFF; DROP TRIGGER IF EXISTS analysis_runs_immutable; DROP TRIGGER IF EXISTS analysis_runs_no_replace;
             DROP TRIGGER IF EXISTS model_states_immutable; DROP TRIGGER IF EXISTS model_states_no_replace; {statements}"
        ))
        .unwrap();
}

fn quoted(text: &str) -> String {
    text.replace('\'', "''")
}

#[test]
fn each_identity_column_must_agree_with_the_verified_record() {
    let cases: [(&str, &str); 7] = [
        ("record hash", "UPDATE analysis_runs SET run_record_hash = (CASE WHEN substr(run_record_hash, 1, 1) = '0' THEN '1' ELSE '0' END) || substr(run_record_hash, 2);"),
        ("recording id", "UPDATE analysis_runs SET recording_id = 'runrec:renamed'; UPDATE analysis_run_labels SET recording_id = 'runrec:renamed';"),
        ("state id", "UPDATE analysis_runs SET state_id = (SELECT state_id FROM model_states WHERE state_id <> analysis_runs.state_id);"),
        ("producer run id", "UPDATE analysis_runs SET producer_run_id = 'run:other';"),
        ("created_at", "UPDATE analysis_runs SET created_at = '2000-01-01T00:00:00.000Z';"),
        ("result hash", "UPDATE analysis_runs SET result_hash = (CASE WHEN substr(result_hash, 1, 1) = '0' THEN '1' ELSE '0' END) || substr(result_hash, 2);"),
        (
            "project",
            "INSERT INTO local_projects (project_id, project_name, model_json, created_at_unix, updated_at_unix) VALUES ('project:other', 'o', '{}', 1, 1);
             UPDATE model_states SET project_id = 'project:other'; UPDATE model_state_labels SET project_id = 'project:other';
             UPDATE analysis_runs SET project_id = 'project:other'; UPDATE analysis_run_labels SET project_id = 'project:other';",
        ),
    ];
    for (index, (field, statements)) in cases.into_iter().enumerate() {
        let path = store_with_project(&format!("identity-{index}"));
        let solved = model("Invented project");
        save_state(&path, &solved, "Basis", "00000000000000000000000000000f01", 1_000);
        save_state(&path, &model("Other"), "Other", "00000000000000000000000000000f02", 2_000);
        let outcome = record(&path, request(&solved, "Run"), T0).unwrap();
        assert!(read_run_in_store(&path, PROJECT, &outcome.run.recording_id).is_ok());
        tamper(&path, statements);
        let (project, recording_id) = match field {
            "recording id" => (PROJECT, "runrec:renamed".to_string()),
            "project" => ("project:other", outcome.run.recording_id.clone()),
            _ => (PROJECT, outcome.run.recording_id.clone()),
        };
        let expected = format!("ANALYSIS-RUN-INTEGRITY: recorded run `{recording_id}`: the row's {field} differs from its verified record");
        let (connection, _) = crate::open_project_store(&path).unwrap();
        let listed = list_runs(&connection, project).unwrap_err();
        assert!(listed.starts_with(&expected), "list, {field}: {listed}");
        let read = read_run_in_store(&path, project, &recording_id).unwrap_err();
        assert!(read.starts_with(&expected), "read, {field}: {read}");
    }
}

#[test]
fn a_read_checks_the_stored_result_manifest_and_model_payload() {
    let solved = model("Invented project");
    let fresh = |label: &str| {
        let path = store_with_project(label);
        save_state(&path, &solved, "Basis", "00000000000000000000000000000f11", 1_000);
        let state_b = save_state(&path, &model("Other"), "Other", "00000000000000000000000000000f12", 2_000);
        let outcome = record(&path, request(&solved, "Run"), T0).unwrap();
        (path, outcome, state_b)
    };
    // The received-result bytes.
    let (path, outcome, _) = fresh("read-result");
    tamper(&path, &format!("UPDATE analysis_runs SET result_json = '{}';", quoted(&result_text(9.0))));
    let error = read_run_in_store(&path, PROJECT, &outcome.run.recording_id).unwrap_err();
    assert!(error.starts_with(&format!("ANALYSIS-RUN-INTEGRITY: recorded run `{}`: the stored result differs", outcome.run.recording_id)), "{error}");
    // The manifest: another one, and the same one not canonical.
    let (path, outcome, _) = fresh("read-manifest");
    tamper(&path, &format!("UPDATE analysis_runs SET input_manifest_json = '{}';", quoted(&manifest_text(&model("Other")))));
    let error = read_run_in_store(&path, PROJECT, &outcome.run.recording_id).unwrap_err();
    assert!(error.contains("the stored input manifest differs from its verified record"), "{error}");
    // A coherent forgery: the record rebuilt (with its hash and recording id) to
    // name another state, over the original manifest. The record and its columns
    // agree; the manifest's model payload is not that state's model.
    let (path, outcome, state_b) = fresh("read-payload");
    let read = read_run_in_store(&path, PROJECT, &outcome.run.recording_id).unwrap();
    let manifest: Value = serde_json::from_str(&read.input_manifest_canonical).unwrap();
    let transient_record = request(&solved, "Run").run_record;
    let (forged, forged_hash) = recorded_revision(&UnmeteredReadMemory, &transient_record, &state_b.state_id, &outcome.run.created_at, &manifest).unwrap();
    let forged_id = recording_id_of(&forged_hash);
    tamper(
        &path,
        &format!(
            "UPDATE analysis_runs SET state_id = '{}', run_record_json = '{}', run_record_hash = '{forged_hash}', recording_id = '{forged_id}';
             UPDATE analysis_run_labels SET recording_id = '{forged_id}';",
            state_b.state_id,
            quoted(&serde_json::to_string(&forged).unwrap())
        ),
    );
    let (connection, _) = crate::open_project_store(&path).unwrap();
    assert_eq!(list_runs(&connection, PROJECT).unwrap()[0].state_id, state_b.state_id, "the forgery is coherent");
    let error = read_run_in_store(&path, PROJECT, &forged_id).unwrap_err();
    assert!(error.contains("the stored input manifest's model payload is not state \"Other\""), "{error}");
}

#[test]
fn rule_pack_and_library_refs_follow_the_manifest() {
    let manifest = json!({ "active_rule_packs": [
        { "rule_pack_id": "rule-pack:b", "version": "2", "checksum_sha256": "b", "source_notice": "n" },
        { "rule_pack_id": "rule-pack:a", "version": "1", "checksum_sha256": "a", "source_notice": "n" }] });
    assert_eq!(rule_pack_refs(&manifest).unwrap(), vec![
        json!({ "object_type": "RulePack", "ref": "rule-pack:b@2" }),
        json!({ "object_type": "RulePack", "ref": "rule-pack:a@1" })
    ], "the manifest's order (the model's reference order)");
    assert!(rule_pack_refs(&json!({})).unwrap_err().starts_with("ANALYSIS-RUN-MANIFEST-INVALID"));
    let payload = json!({
        "materials": [
            { "id": "material:a", "provenance_record": { "library_ref": { "library_kind": "material", "library_id": "invented-lib", "library_version": "sha256:aa", "record_id": "steel" } } },
            { "id": "material:b", "provenance_record": { "library_ref": { "library_kind": "material", "library_id": "invented-lib", "library_version": "sha256:aa", "record_id": "steel" } } }
        ],
        "components": [{ "id": "component:c", "nested": { "provenance_record": { "library_ref": { "library_kind": "component", "library_id": "valves" } } } }],
        "library_ref": { "library_kind": "material", "library_id": "outside-a-provenance-record" }
    });
    assert_eq!(library_refs(&payload), vec![
        json!({ "object_type": "Library", "ref": "library:component/valves" }),
        json!({ "object_type": "Library", "ref": "library:material/invented-lib@sha256:aa#steel" }),
        json!({ "object_type": "Library", "ref": "library:material/outside-a-provenance-record" })
    ], "every library_ref the one walk finds, once each, in code-point order");
    // Rust's White_Space trim: U+3000, U+0085 and U+2003 go; U+FEFF and U+001F stay.
    let spaced = json!({ "a": { "library_ref": { "library_kind": "\u{3000}k\u{85}", "library_id": "\u{FEFF}id\u{2003}", "record_id": "\u{1F}" } } });
    assert_eq!(library_refs(&spaced), vec![json!({ "object_type": "Library", "ref": "library:k/\u{FEFF}id#\u{1F}" })]);
}

/// The recorded revision's `library_refs` come from the model document's one
/// walk, pinned to the shared site table as the native save's walk
/// (`library_ref_walk_matches_the_shared_site_table`) and the webview's
/// `libraryRefSites` are: every site with a table identity, normalized as the
/// table normalizes it, and no other; a site whose identity is null is omitted.
#[test]
fn library_refs_follow_the_shared_site_table() {
    let table: Value = serde_json::from_str(include_str!("../../../../fixtures/model_document/library_ref_sites.json")).unwrap();
    let document = &table["document"];
    // serde_json's maps keep keys sorted, so the walk visits them in key order.
    let mut walked: Vec<String> = crate::model_document_save::library_ref_sites(document).into_iter().map(|(path, _)| path).collect();
    let mut tabled: Vec<String> = table["sites"].as_array().unwrap().iter().map(|site| site["path"].as_str().unwrap().to_string()).collect();
    walked.sort();
    tabled.sort();
    assert_eq!(walked, tabled, "the one walk finds every site of the table, and no other");
    let expected: std::collections::BTreeSet<String> = table["sites"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|site| !site["identity"].is_null())
        .map(|site| {
            let identity = site["identity"].as_array().unwrap();
            let pointer = format!("/{}", site["path"].as_str().unwrap().replace('[', ".").replace(']', "").replace('.', "/"));
            let version = document.pointer(&pointer).unwrap().get("library_version").and_then(Value::as_str);
            format!(
                "library:{}/{}{}{}",
                identity[0].as_str().unwrap(),
                identity[1].as_str().unwrap(),
                version.map(|version| format!("@{version}")).unwrap_or_default(),
                identity[2].as_str().map(|record| format!("#{record}")).unwrap_or_default()
            )
        })
        .collect();
    let refs = library_refs(document);
    assert_eq!(refs, expected.iter().map(|text| json!({ "object_type": "Library", "ref": text })).collect::<Vec<_>>());
    assert_eq!(
        refs.iter().map(|reference| reference["ref"].as_str().unwrap()).collect::<Vec<_>>(),
        vec![
            "library:component/complib.invented.sites#comp.inner",
            "library:hanger/hanglib.invented.sites",
            "library:material/matlib.invented.root#mat.root",
            "library:material/matlib.invented.sites#mat.padded",
            "library:material/matlib.invented.sites@sha256:recorded#mat.top",
            "library:section/seclib.invented.sites",
        ],
        "trimmed identities; blank record ids dropped; null identities omitted"
    );
}

#[test]
fn each_command_is_admitted_with_its_own_budget() {
    let registry = registry(ScriptedHost::normal());
    for (command, budget) in [
        ("list_analysis_runs", LIST_ANALYSIS_RUNS_MAX_ARGS_BYTES),
        ("read_analysis_run", READ_ANALYSIS_RUN_MAX_ARGS_BYTES),
        ("delete_analysis_run", DELETE_ANALYSIS_RUN_MAX_ARGS_BYTES),
        ("edit_analysis_run_labels", EDIT_ANALYSIS_RUN_LABELS_MAX_ARGS_BYTES),
    ] {
        let body = |length: usize| InvokeBody::Raw(format!("{{\"projectId\":\"{}\"}}", "p".repeat(length)).into_bytes());
        assert!(admit_body_within(&registry, &body(budget - 16), &headers(Some(INGRESS_ARGS_JSON_V1)), command, ArgsDecoding::Moved, budget).is_ok());
        match admit_body_within(&registry, &body(budget - 15), &headers(Some(INGRESS_ARGS_JSON_V1)), command, ArgsDecoding::Moved, budget) {
            Err(SolveJobError::InvalidRequest { message }) => assert_eq!(
                message,
                format!("INGRESS-ARGS-TOO-LARGE: `{command}` takes at most {budget} bytes of arguments; {} were sent.", budget + 1)
            ),
            other => panic!("{command}: {other:?}"),
        }
    }
    // record_analysis_run's result, manifest and record decode within its lease (moved, not copied).
    let submitted = request(&model("Invented project"), "Run");
    let body = serde_json::to_vec(&json!({ "request": {
        "project_id": PROJECT, "expected_base": BASE, "run_record": submitted.run_record, "result_json": submitted.result_json,
        "input_manifest_canonical": submitted.input_manifest_canonical, "state_id": null,
        "new_state": { "labels": { "state_name": "After solve" }, "parent_state_id": null },
        "run_labels": { "run_name": "Run", "tags": ["a"], "notes": ["n"], "assumptions": [] }
    } }))
    .unwrap();
    let (decoded, _, _) = check_decode_bound(&body, ArgsDecoding::Moved, |args| {
        args.decode_with("request", RecordRunRequest::from_value)
            .map(|request| (request.result_json.len(), request.new_state.map(|state| state.labels.state_name)))
    });
    assert_eq!(decoded.unwrap(), (submitted.result_json.len(), Some("After solve".to_string())));
}

/// The recorded-run witness (`fixtures/persistence/recorded_run_witness.json`):
/// the TypeScript-built transient record of a real producer result
/// (`transient_run_input.json`), recorded by this module's path. The vitest
/// parity test rebuilds the revision with the TypeScript builder and the pytest
/// validates it against `analysis_run.v0.3` and Python's validator. With
/// `OPENPIPESTRESS_WRITE_RECORDED_RUN_WITNESS=<path>` the test writes the witness
/// there instead of comparing (the maintained writer).
#[test]
fn the_recorded_run_witness_is_what_the_record_path_emits_for_a_typescript_built_record() {
    let input: Value = serde_json::from_str(TRANSIENT_INPUT).unwrap();
    let manifest_text = input["input_manifest_canonical"].as_str().unwrap().to_string();
    let manifest: Value = serde_json::from_str(&manifest_text).unwrap();
    let solved = manifest["model_basis"]["model_payload"].clone();
    let project_id = solved["project"]["id"].as_str().unwrap().to_string();
    let path = scratch_store("witness");
    let (mut connection, _) = crate::open_project_store(&path).unwrap();
    write_project(&mut connection, &project_id, &solved, BASE);
    drop(connection);
    let now = || Ok(T0);
    let token = || Ok("0123456789abcdef0123456789abcdef".to_string());
    let between_rows = |_: &Transaction<'_>, _: &str| Ok(());
    let outcome = record_run_in_store(
        &path,
        RecordRunRequest {
            project_id: project_id.clone(),
            expected_base: BASE.to_string(),
            run_record: input["run_record"].clone(),
            result_json: SOURCE_RESULT.to_string(),
            input_manifest_canonical: manifest_text,
            state_id: None,
            new_state: Some(NewStateRequest { labels: labels_from_value(json!({ "state_name": "After solve" })).unwrap(), parent_state_id: None }),
            run_labels: run_labels("Witness run"),
        },
        &RunWriteHooks { now: &now, state_token: &token, between_rows: &between_rows },
    )
    .unwrap();
    let read = read_run_in_store(&path, &project_id, &outcome.run.recording_id).unwrap();
    assert_eq!(read.result_json, SOURCE_RESULT, "the producer's bytes are stored as given");
    let witness = json!({
        "description": "An analysis_run.v0.3 recorded revision emitted by the desktop store's Record Run path (apps/desktop/src-tauri/src/analysis_runs_tests.rs, test the_recorded_run_witness_is_what_the_record_path_emits_for_a_typescript_built_record) from fixtures/persistence/transient_run_input.json (a TypeScript-built transient record of fixtures/results/preview_physics_connected_sparse.json). Regenerate with OPENPIPESTRESS_WRITE_RECORDED_RUN_WITNESS=<path> on that test. apps/desktop/src/services/recordedRunParity.test.ts rebuilds it with the TypeScript builder; tests/test_recorded_run_record.py validates it against schemas/analysis_run.v0.3.schema.json and Python's validator.",
        "state_id": outcome.state.state_id,
        "state_model_hash": outcome.state.model_hash,
        "created_at": outcome.run.created_at,
        "recording_id": outcome.run.recording_id,
        "run_record_hash": outcome.run.run_record_hash,
        "recorded_record": read.record
    });
    let text = format!("{}\n", serde_json::to_string_pretty(&witness).unwrap());
    if let Ok(target) = std::env::var(WITNESS_PATH_ENV) {
        std::fs::write(target, &text).unwrap();
        return;
    }
    assert_eq!(text, WITNESS, "the committed witness is stale; regenerate it with {WITNESS_PATH_ENV}");
}

// ---------------------------------------------------------------------------
// Repair r1: exact checksum inventories and manifest-fixed members, at Record
// Run and at the stored-record checks.
// ---------------------------------------------------------------------------

type Mutation = fn(&mut Value);

/// Rewrites the stored run to `record` with its hash and recording id (and the
/// labels' recording id) made to agree: a coherent forgery the record's own
/// hash cannot see.
fn forge(path: &Path, record: &Value) -> String {
    let hash = run_record_hash(record).unwrap();
    let recording_id = recording_id_of(&hash);
    Connection::open(path)
        .unwrap()
        .execute_batch(&format!(
            "PRAGMA foreign_keys = OFF; DROP TRIGGER IF EXISTS analysis_runs_immutable; DROP TRIGGER IF EXISTS analysis_runs_no_replace;
             UPDATE analysis_runs SET run_record_json = '{}', run_record_hash = '{hash}', recording_id = '{recording_id}';
             UPDATE analysis_run_labels SET recording_id = '{recording_id}';",
            serde_json::to_string(record).unwrap().replace('\'', "''")
        ))
        .unwrap();
    recording_id
}

fn recorded_fixture(label: &str) -> (PathBuf, Value) {
    let path = store_with_project(label);
    let solved = model("Invented project");
    save_state(&path, &solved, "Basis", "00000000000000000000000000001001", 1_000);
    let outcome = record(&path, request(&solved, "Run"), T0).unwrap();
    let stored = read_run_in_store(&path, PROJECT, &outcome.run.recording_id).unwrap().record;
    (path, stored)
}

#[test]
fn every_checksum_descriptor_is_exact_at_record_run_and_in_the_stored_record() {
    let mutations: [(&str, Mutation); 12] = [
        ("record algorithm", |record| record["analysis_run"]["hashes"][0]["algorithm"] = json!("sha512")),
        ("record profile", |record| record["analysis_run"]["hashes"][0]["canonicalization"] = json!("rfc8785_jcs")),
        ("record scope", |record| record["analysis_run"]["hashes"][0]["payload_scope"] = json!("audit_manifest")),
        ("record ref", |record| record["analysis_run"]["hashes"][0]["payload_ref"]["ref"] = json!("run:other")),
        ("record extra field", |record| record["analysis_run"]["hashes"][0]["note"] = json!("x")),
        ("received algorithm", |record| { record["analysis_run"]["hashes"][1]["algorithm"] = json!("sha512"); reseal(record) }),
        ("received ref", |record| { record["analysis_run"]["hashes"][1]["payload_ref"]["object_type"] = json!("Result"); reseal(record) }),
        ("duplicate entry", |record| { let copy = record["analysis_run"]["hashes"][1].clone(); record["analysis_run"]["hashes"].as_array_mut().unwrap().push(copy); reseal(record) }),
        ("extra entry", |record| {
            record["analysis_run"]["hashes"].as_array_mut().unwrap().push(json!({ "algorithm": "sha256", "canonicalization": "NONE",
                "payload_ref": { "object_type": "AuditManifest", "ref": "x" }, "payload_scope": "audit_manifest", "value": "00" }));
            reseal(record)
        }),
        ("missing entry", |record| { record["analysis_run"]["hashes"].as_array_mut().unwrap().remove(1); reseal(record) }),
        ("manifest checksum algorithm", |record| { record["analysis_run"]["reproducibility"]["input_manifest_hashes"][0]["algorithm"] = json!("sha512"); reseal(record) }),
        ("row checksum algorithm", |record| { record["analysis_run"]["result_refs"][0]["hash_refs"][0]["algorithm"] = json!("sha512"); reseal(record) }),
    ];
    for (index, (name, mutation)) in mutations.into_iter().enumerate() {
        // At Record Run: the transient record is refused and nothing is written.
        let path = store_with_project(&format!("descriptor-submit-{index}"));
        let solved = model("Invented project");
        save_state(&path, &solved, "Basis", "00000000000000000000000000001101", 1_000);
        let mut submitted = request(&solved, "Run");
        mutation(&mut submitted.run_record);
        let error = record(&path, submitted, T0).unwrap_err();
        assert!(
            ["ANALYSIS-RUN-CHECKSUM-MISMATCH", "ANALYSIS-RUN-NOT-TRANSIENT", "ANALYSIS-RUN-MANIFEST-MISMATCH", "ANALYSIS-RUN-RESULT-MISMATCH", "ANALYSIS-RUN-RECORD-CHECKSUM-DUPLICATE"]
                .iter()
                .any(|code| error.contains(code)),
            "{name}, at Record Run: {error}"
        );
        assert_eq!(row_count(&path, "analysis_runs"), 0, "{name}");
        let only_the_descriptor_moves = matches!(name, "record algorithm" | "record profile" | "record extra field");
        if only_the_descriptor_moves {
            // The record digest leaves its own checksum out: only the exact inventory sees these.
            assert!(error.starts_with("ANALYSIS-RUN-CHECKSUM-MISMATCH: the record's checksum descriptors are not exactly the v0.3 ones"), "{name}: {error}");
        }
        // In the store: a coherently resealed stored record is refused by the read
        // (and by the list, which verifies every record but does not load results).
        let (path, mut stored) = recorded_fixture(&format!("descriptor-store-{index}"));
        mutation(&mut stored);
        let recording_id = forge(&path, &stored);
        let read = read_run_in_store(&path, PROJECT, &recording_id).unwrap_err();
        assert!(read.starts_with(&format!("ANALYSIS-RUN-INTEGRITY: recorded run `{recording_id}`")), "{name}, read: {read}");
        if only_the_descriptor_moves {
            assert!(read.contains("the row's checksum inventory differs from its verified record"), "{name}: {read}");
        }
        let (connection, _) = crate::open_project_store(&path).unwrap();
        let listed = list_runs(&connection, PROJECT);
        if name == "row checksum algorithm" {
            assert!(read.contains("the stored result's row checksums differ"), "{read}");
        } else {
            let listed = listed.unwrap_err();
            assert!(listed.starts_with("ANALYSIS-RUN-INTEGRITY") || listed.starts_with("ANALYSIS-RUN-RECORD-CHECKSUM-DUPLICATE"), "{name}, list: {listed}");
        }
    }
}

#[test]
fn every_member_the_manifest_fixes_is_derived_from_the_manifest_at_record_run_and_read() {
    let mutations: [(&str, Mutation); 9] = [
        ("settings ref", |record| record["analysis_run"]["settings_ref"]["ref"] = json!("solver-settings:other")),
        ("solver version", |record| record["analysis_run"]["solver_version"]["build_ref"]["ref"] = json!("other-solver@9")),
        ("solver version", |record| record["analysis_run"]["solver_version"]["solver_version"] = json!("9.9.9")),
        ("unit-system ref", |record| record["analysis_run"]["unit_system_ref"]["ref"] = json!(format!("unit-system:{PROJECT}:other"))),
        ("load basis", |record| record["analysis_run"]["load_basis_refs"].as_array_mut().unwrap().push(json!({ "object_type": "LoadCase", "ref": "LC-ABSENT" }))),
        ("rule-pack refs", |record| record["analysis_run"]["rule_pack_refs"] = json!([{ "object_type": "RulePack", "ref": "rule-pack:other@1" }])),
        ("library refs", |record| record["analysis_run"]["library_refs"] = json!([{ "object_type": "Library", "ref": "library:material/other" }])),
        ("input manifest ref", |record| {
            let other = json!({ "object_type": "InputManifest", "ref": "input-manifest:other" });
            record["analysis_run"]["reproducibility"]["input_manifest_refs"] = json!([other.clone()]);
            record["analysis_run"]["reproducibility"]["input_manifest_hashes"][0]["payload_ref"] = other;
        }),
        ("solver version", |record| { record["analysis_run"]["solver_version"].as_object_mut().unwrap().insert("extra".into(), json!(1)); }),
    ];
    for (index, (name, mutation)) in mutations.into_iter().enumerate() {
        let path = store_with_project(&format!("members-submit-{index}"));
        let solved = model("Invented project");
        save_state(&path, &solved, "Basis", "00000000000000000000000000001201", 1_000);
        let mut submitted = request(&solved, "Run");
        mutation(&mut submitted.run_record);
        reseal(&mut submitted.run_record);
        let error = record(&path, submitted, T0).unwrap_err();
        assert!(error.starts_with(&format!("ANALYSIS-RUN-MANIFEST-MISMATCH: the record's {name} is not the input manifest's")), "{name}, at Record Run: {error}");
        assert_eq!(row_count(&path, "analysis_runs"), 0);
        let (path, mut stored) = recorded_fixture(&format!("members-store-{index}"));
        mutation(&mut stored);
        reseal(&mut stored);
        let recording_id = forge(&path, &stored);
        let read = read_run_in_store(&path, PROJECT, &recording_id).unwrap_err();
        assert!(read.starts_with(&format!("ANALYSIS-RUN-INTEGRITY: recorded run `{recording_id}`")), "{name}, read: {read}");
        assert!(
            read.contains(&format!("the record's {name} differs from its stored input manifest"))
                || read.contains("the row's checksum inventory differs")
                || read.contains("the row's project differs"),
            "{name}, read: {read}"
        );
    }
}

// ---------------------------------------------------------------------------
// Repairs r1 and r2: reads take memory-pool leases sized from the stored
// lengths before they load anything, refuse invalid stored JSON before parsing
// it, and hold their reservations until the response is serialized.
// ---------------------------------------------------------------------------

const LARGE: usize = 3 << 20;
/// A pool (and budget) that admits ordinary reads of these stores but is well
/// below the large columns' priced need (each test first reads the same kind of
/// row without its large column under it): Codex r2's 2 MiB probe.
const SMALL_POOL: u64 = 2 << 20;

fn small_pool() -> crate::solve_jobs::SolveJobRegistry {
    pool_of(SMALL_POOL)
}

fn pool_of(bytes: u64) -> crate::solve_jobs::SolveJobRegistry {
    let pool = registry(ScriptedHost::normal());
    pool.configure_pool(bytes, bytes, false).unwrap();
    pool
}

/// A refused read loads nothing large: its priced allocation peak stays far
/// below the large column, and the pool holds nothing afterwards.
fn assert_refused_unloaded<T: std::fmt::Debug>(
    pool: &crate::solve_jobs::SolveJobRegistry,
    what: &str,
    read: impl FnOnce(&PoolReadMemory<'_>) -> Result<T, String>,
) {
    let memory = PoolReadMemory::unseeded(pool);
    let (outcome, measured) = crate::ingress::counting::measure(|| read(&memory));
    let error = outcome.unwrap_err();
    assert!(error.starts_with(&format!("STORE-READ-MEMORY-UNAVAILABLE: reading {what}")), "{error}");
    assert!(measured.peak_priced < (LARGE as u64) / 4, "{what}: the large column was not loaded (peak {})", measured.peak_priced);
    assert_eq!(pool.pool_status().reserved_bytes, 0, "{what}: every lease was released");
}

/// The same read admitted by `pool` (the baseline that makes a refusal mean the
/// large column was priced, not the read's fixed overhead).
fn assert_admitted<T>(pool: &crate::solve_jobs::SolveJobRegistry, what: &str, read: impl FnOnce(&PoolReadMemory<'_>) -> Result<T, String>) {
    let memory = PoolReadMemory::unseeded(pool);
    if let Err(error) = read(&memory) {
        panic!("{what}: the baseline read is refused: {error}");
    }
    drop(memory);
    assert_eq!(pool.pool_status().reserved_bytes, 0);
}

/// An admitted read, through its serialized response: the pool still holds the
/// read's reservation when the value is handed to serialization, the priced
/// allocation peak of the read and the serialization stays within the most the
/// read reserved, and every lease is released once the response exists.
/// Returns the response's length.
fn assert_within_reservation<T: Serialize>(what: &str, read: impl FnOnce(&PoolReadMemory<'_>) -> Result<Metered<T>, String>) -> u64 {
    let pool = registry(ScriptedHost::normal());
    let memory = PoolReadMemory::unseeded(&pool);
    let (outcome, measured) = crate::ingress::counting::measure(|| {
        let metered = read(&memory)?;
        let length = stored_read::serialized_length(metered.get())?;
        let held = pool.pool_status().reserved_bytes;
        let response = stored_read::respond_metered::<T, String>(&memory, what, Ok(metered)).map_err(|error| format!("{error:?}"))?;
        Ok::<_, String>((held, length, response))
    });
    let (held, length, response) = outcome.unwrap();
    drop(response);
    assert!(held >= length, "{what}: the read's reservation ({held}) is held when its {length}-byte response is serialized");
    assert!(
        measured.peak_priced <= memory.peak_reserved(),
        "{what}: priced peak {} exceeds the reservation {}",
        measured.peak_priced,
        memory.peak_reserved()
    );
    assert_eq!(pool.pool_status().reserved_bytes, 0, "{what}: every lease was released after the response");
    length
}

#[test]
fn read_model_state_is_refused_before_loading_a_large_snapshot_the_pool_does_not_admit() {
    let path = store_with_project("memory-state");
    let ordinary = save_state(&path, &model("Invented project"), "Ordinary", "00000000000000000000000000001300", 500);
    let large = model(&"x".repeat(LARGE));
    let state = save_state(&path, &large, "Large", "00000000000000000000000000001301", 1_000);
    let length: i64 = Connection::open(&path)
        .unwrap()
        .query_row("SELECT octet_length(model_canonical) FROM model_states WHERE state_id = ?1", params![state.state_id], |row| row.get(0))
        .unwrap();
    assert!(length as usize > LARGE);
    let small = small_pool();
    assert_admitted(&small, "an ordinary state", |memory| model_states::read_state_in_store_with(&path, PROJECT, &ordinary.state_id, memory));
    assert_refused_unloaded(&small, &format!("saved state `{}`", state.state_id), |memory| {
        model_states::read_state_in_store_with(&path, PROJECT, &state.state_id, memory)
    });
    let response = assert_within_reservation("read_model_state", |memory| {
        model_states::read_state_in_store_with(&path, PROJECT, &state.state_id, memory)
    });
    assert!(response as usize > LARGE);
    assert_eq!(model_states::read_state_in_store(&path, PROJECT, &state.state_id).unwrap().model, large);
}

#[test]
fn the_run_read_is_refused_before_loading_a_large_result_the_pool_does_not_admit() {
    let path = store_with_project("memory-run");
    let solved = model("Invented project");
    save_state(&path, &solved, "Basis", "00000000000000000000000000001401", 1_000);
    let manifest = manifest_text(&solved);
    let result = format!(
        "{{\"run_id\":\"{RUN}\",\"model_ref\":\"{PROJECT}\",\"results\":[{{\"id\":\"result:disp:N-100\",\"value\":1.5,\"padding\":\"{}\"}}],\"diagnostics\":[]}}",
        "p".repeat(LARGE)
    );
    let ordinary = record(&path, request(&solved, "Ordinary result"), T0).unwrap();
    let mut submitted = request(&solved, "Large result");
    submitted.run_record = transient(&result, &manifest);
    submitted.result_json = result.clone();
    let outcome = record(&path, submitted, T0).unwrap();
    let small = small_pool();
    assert_admitted(&small, "an ordinary run", |memory| read_run_in_store_with(&path, PROJECT, &ordinary.run.recording_id, memory));
    assert_refused_unloaded(&small, &format!("recorded run `{}`", outcome.run.recording_id), |memory| {
        read_run_in_store_with(&path, PROJECT, &outcome.run.recording_id, memory)
    });
    let response = assert_within_reservation("read_analysis_run", |memory| {
        read_run_in_store_with(&path, PROJECT, &outcome.run.recording_id, memory)
    });
    assert!(response as usize > LARGE);
    assert_eq!(read_run_in_store(&path, PROJECT, &outcome.run.recording_id).unwrap().result_json, result);
}

#[test]
fn the_lists_and_label_edits_reserve_their_largest_record_before_loading_it() {
    let path = store_with_project("memory-list");
    let solved = model("Invented project");
    save_state(&path, &solved, "Basis", "00000000000000000000000000001501", 1_000);
    let ordinary = record(&path, request(&solved, "Ordinary record"), T0).unwrap();
    {
        assert_admitted(&small_pool(), "an ordinary list", |memory| list_runs_in_store_with(&path, PROJECT, memory));
        assert_admitted(&small_pool(), "an ordinary edit", |memory| {
            edit_run_labels_in_store_with(&path, PROJECT, &ordinary.run.recording_id, run_labels("Renamed"), None, T0, memory)
        });
    }
    let mut submitted = request(&solved, "Large record");
    submitted.run_record["analysis_run"]["padding"] = json!("r".repeat(LARGE));
    reseal(&mut submitted.run_record);
    let outcome = record(&path, submitted, T0).unwrap();
    let small = small_pool();
    let (connection, _) = crate::open_project_store(&path).unwrap();
    assert_refused_unloaded(&small, &format!("the recorded runs of project `{PROJECT}`"), |memory| list_runs_in_store_with(&path, PROJECT, memory));
    assert_within_reservation("list_analysis_runs", |memory| list_runs_in_store_with(&path, PROJECT, memory));
    assert_eq!(list_runs(&connection, PROJECT).unwrap().len(), 2);
    assert_refused_unloaded(&small, &format!("recorded run `{}`", outcome.run.recording_id), |memory| {
        edit_run_labels_in_store_with(&path, PROJECT, &outcome.run.recording_id, run_labels("Renamed"), None, T0, memory)
    });
    assert_refused_unloaded(&small, &format!("recorded run `{}`", outcome.run.recording_id), |memory| {
        delete_run_in_store_with(&path, PROJECT, &outcome.run.recording_id, memory)
    });
    assert_eq!(row_count(&path, "analysis_runs"), 2, "nothing was deleted");
    assert_within_reservation("edit_analysis_run_labels", |memory| {
        edit_run_labels_in_store_with(&path, PROJECT, &outcome.run.recording_id, run_labels("Renamed"), None, T0 + 1, memory)
    });
}

/// Rewrites a stored label column outside the store's paths (labels rows are
/// editable; this bypasses the write-time bound, as a hand edit could).
fn set_label(path: &Path, table: &str, column: &str, value: &str) {
    tamper(path, &format!("UPDATE {table} SET {column} = '{}';", quoted(value)));
}

#[test]
fn large_stored_labels_are_priced_before_they_are_loaded() {
    // A run's name: read, list, edit and delete, and the state summary that names its runs.
    let path = store_with_project("memory-labels-run");
    let solved = model("Invented project");
    let state = save_state(&path, &solved, "Basis", "00000000000000000000000000001601", 1_000);
    let outcome = record(&path, request(&solved, "Run"), T0).unwrap();
    let id = outcome.run.recording_id.clone();
    let small = small_pool();
    assert_admitted(&small, "the run before", |memory| read_run_in_store_with(&path, PROJECT, &id, memory));
    assert_admitted(&small, "the runs before", |memory| list_runs_in_store_with(&path, PROJECT, memory));
    assert_admitted(&small, "the states before", |memory| model_states::list_states_in_store_with(&path, PROJECT, memory));
    set_label(&path, "analysis_run_labels", "run_name", &"n".repeat(LARGE));
    assert_refused_unloaded(&small, &format!("recorded run `{id}`"), |memory| read_run_in_store_with(&path, PROJECT, &id, memory));
    assert_refused_unloaded(&small, &format!("the recorded runs of project `{PROJECT}`"), |memory| list_runs_in_store_with(&path, PROJECT, memory));
    assert_refused_unloaded(&small, &format!("recorded run `{id}`"), |memory| delete_run_in_store_with(&path, PROJECT, &id, memory));
    // The state summary loads its runs' ids, not their names (repair r4): the
    // state list is admitted, and the blocked delete quotes the name's head only.
    assert_admitted(&small, "the states after", |memory| model_states::list_states_in_store_with(&path, PROJECT, memory));
    let memory = PoolReadMemory::unseeded(&small);
    let (outcome, measured) = crate::ingress::counting::measure(|| model_states::delete_state_in_store_with(&path, PROJECT, &state.state_id, &memory).map(|_| ()));
    let error = outcome.unwrap_err();
    assert!(error.starts_with("MODEL-STATE-DELETE-BLOCKED: ") && error.len() < 1024 && error.contains("…"), "{error}");
    assert!(measured.peak_priced <= memory.peak_reserved(), "peak {} within {}", measured.peak_priced, memory.peak_reserved());
    assert!(assert_within_reservation("read_analysis_run", |memory| read_run_in_store_with(&path, PROJECT, &id, memory)) as usize > LARGE);
    assert!(assert_within_reservation("list_analysis_runs", |memory| list_runs_in_store_with(&path, PROJECT, memory)) as usize > LARGE);
    assert_within_reservation("list_model_states", |memory| model_states::list_states_in_store_with(&path, PROJECT, memory));

    // A state's labels: its read, list, label edit and delete.
    let path = store_with_project("memory-labels-state");
    let state = save_state(&path, &model("Invented project"), "Basis", "00000000000000000000000000001602", 1_000);
    {
        assert_admitted(&small, "the state before", |memory| model_states::read_state_in_store_with(&path, PROJECT, &state.state_id, memory));
        assert_admitted(&small, "the states before", |memory| model_states::list_states_in_store_with(&path, PROJECT, memory));
    }
    let notes: Vec<String> = (0..1024).map(|index| format!("{index}:{}", "t".repeat(LARGE / 1024))).collect();
    set_label(&path, "model_state_labels", "notes_json", &serde_json::to_string(&notes).unwrap());
    let what = format!("saved state `{}`", state.state_id);
    assert_refused_unloaded(&small, &what, |memory| model_states::read_state_in_store_with(&path, PROJECT, &state.state_id, memory));
    assert_refused_unloaded(&small, &format!("the saved states of project `{PROJECT}`"), |memory| model_states::list_states_in_store_with(&path, PROJECT, memory));
    assert_refused_unloaded(&small, &what, |memory| {
        model_states::edit_labels_in_store_with(&path, PROJECT, &state.state_id, labels_from_value(json!({ "state_name": "Renamed" })).unwrap(), None, T0, memory)
    });
    assert_refused_unloaded(&small, &what, |memory| model_states::delete_state_in_store_with(&path, PROJECT, &state.state_id, memory));
    // 1,024 label items each gain a provenance object in the emitted record.
    assert!(assert_within_reservation("read_model_state", |memory| model_states::read_state_in_store_with(&path, PROJECT, &state.state_id, memory)) as usize > LARGE);
    assert_within_reservation("list_model_states", |memory| model_states::list_states_in_store_with(&path, PROJECT, memory));
}

#[test]
fn labels_are_bounded_when_they_are_written() {
    for (labels, expected) in [
        (json!({ "state_name": "n", "notes": ["x".repeat(200 * 1024)] }), "bytes of JSON; at most 196608 are kept"),
        (json!({ "state_name": "n", "tags": (0..257).map(|index| format!("t{index}")).collect::<Vec<_>>() }), "more than the 256 kept"),
    ] {
        let error = model_states::labels_from_value(labels.clone()).unwrap_err();
        assert!(error.starts_with("MODEL-STATE-LABELS-INVALID: ") && error.contains(expected), "{error}");
        let mut run = labels;
        let name = run.as_object_mut().unwrap().remove("state_name").unwrap();
        run["run_name"] = name;
        let error = run_labels_from_value(run).unwrap_err();
        assert!(error.starts_with("ANALYSIS-RUN-LABELS-INVALID: ") && error.contains(expected), "{error}");
    }
    let ok = json!({ "state_name": "n", "notes": ["x".repeat(150 * 1024)], "tags": (0..200).map(|index| format!("t{index}")).collect::<Vec<_>>() });
    assert!(model_states::labels_from_value(ok).is_ok());
}

/// A stored text the prescan refuses: an array of 524,288 integers, then a
/// syntax error (about 1 MiB), as Codex r2's probe.
fn invalid_json() -> String {
    format!("[{}x", "1,".repeat(524_288))
}

#[test]
fn prescan_invalid_stored_json_is_refused_before_it_is_parsed() {
    let invalid = invalid_json();
    let bound = (invalid.len() as u64) * 16;
    let refused = |what: &str, error: &str, measured: crate::ingress::counting::Measured, reserved: u64, code: &str| {
        assert!(error.starts_with(code) && error.contains("the stored JSON is invalid"), "{what}: {error}");
        assert!(measured.peak_priced < bound, "{what}: refused before parsing (peak {} against the text's {} bytes)", measured.peak_priced, invalid.len());
        assert!(measured.peak_priced <= reserved, "{what}: peak {} within the reservation {reserved}", measured.peak_priced);
    };
    let pool = registry(ScriptedHost::normal());
    // A run record.
    let (path, _) = recorded_fixture("invalid-run-record");
    tamper(&path, &format!("UPDATE analysis_runs SET run_record_json = '{invalid}';"));
    let id: String = Connection::open(&path).unwrap().query_row("SELECT recording_id FROM analysis_runs", [], |row| row.get(0)).unwrap();
    let memory = PoolReadMemory::unseeded(&pool);
    let (outcome, measured) = crate::ingress::counting::measure(|| read_run_in_store_with(&path, PROJECT, &id, &memory).map(|_| ()));
    refused("run record", &outcome.unwrap_err(), measured, memory.peak_reserved(), "ANALYSIS-RUN-INTEGRITY");
    // A run's result.
    let (path, _) = recorded_fixture("invalid-run-result");
    tamper(&path, &format!("UPDATE analysis_runs SET result_json = '{invalid}';"));
    let memory = PoolReadMemory::unseeded(&pool);
    let (outcome, measured) = crate::ingress::counting::measure(|| read_run_in_store_with(&path, PROJECT, &id_of(&path), &memory).map(|_| ()));
    refused("run result", &outcome.unwrap_err(), measured, memory.peak_reserved(), "ANALYSIS-RUN-INTEGRITY");
    // A run's label list.
    let (path, _) = recorded_fixture("invalid-run-labels");
    tamper(&path, &format!("UPDATE analysis_run_labels SET tags_json = '{invalid}';"));
    let memory = PoolReadMemory::unseeded(&pool);
    let (outcome, measured) = crate::ingress::counting::measure(|| list_runs_in_store_with(&path, PROJECT, &memory).map(|_| ()));
    refused("run labels", &outcome.unwrap_err(), measured, memory.peak_reserved(), "ANALYSIS-RUN-INTEGRITY");
    // A state's record and its snapshot.
    for (column, label) in [("record_json", "state record"), ("model_canonical", "state snapshot")] {
        let path = store_with_project(&format!("invalid-{column}"));
        let state = save_state(&path, &model("Invented project"), "Basis", "00000000000000000000000000001701", 1_000);
        tamper(&path, &format!("UPDATE model_states SET {column} = '{invalid}';"));
        let memory = PoolReadMemory::unseeded(&pool);
        let (outcome, measured) = crate::ingress::counting::measure(|| model_states::read_state_in_store_with(&path, PROJECT, &state.state_id, &memory).map(|_| ()));
        refused(label, &outcome.unwrap_err(), measured, memory.peak_reserved(), "MODEL-STATE-INTEGRITY");
    }
    assert_eq!(pool.pool_status().reserved_bytes, 0);
}

fn id_of(path: &Path) -> String {
    Connection::open(path).unwrap().query_row("SELECT recording_id FROM analysis_runs", [], |row| row.get(0)).unwrap()
}

/// Replaces the stored state's record with a coherently resealed one holding a
/// large extra member (its hash, descriptor and column made to agree).
fn reseal_large_state_record(path: &Path) {
    let record_json: String = Connection::open(path).unwrap().query_row("SELECT record_json FROM model_states", [], |row| row.get(0)).unwrap();
    let mut record: Value = serde_json::from_str(&record_json).unwrap();
    record["padding"] = json!("s".repeat(LARGE));
    let hash = model_states::record_hash_hex(&record, &UnmeteredReadMemory, "a resealed record").unwrap();
    record["hashes"][1]["value"] = json!(hash);
    tamper(
        path,
        &format!("UPDATE model_states SET record_json = '{}', record_hash = '{hash}';", quoted(&serde_json::to_string(&record).unwrap())),
    );
}

#[test]
fn a_run_read_prices_its_states_large_record() {
    let (path, _) = recorded_fixture("memory-state-record");
    let id = id_of(&path);
    let state_id: String = Connection::open(&path).unwrap().query_row("SELECT state_id FROM model_states", [], |row| row.get(0)).unwrap();
    let small = small_pool();
    assert_admitted(&small, "the run before", |memory| read_run_in_store_with(&path, PROJECT, &id, memory));
    assert_admitted(&small, "the state's edit before", |memory| {
        model_states::edit_labels_in_store_with(&path, PROJECT, &state_id, labels_from_value(json!({ "state_name": "Before" })).unwrap(), None, T0, memory)
    });
    reseal_large_state_record(&path);
    // The pool admits the run's own columns but not the state's record: the
    // refusal names the state, and its record was not loaded.
    assert_refused_unloaded(&small, &format!("saved state `{state_id}`"), |memory| read_run_in_store_with(&path, PROJECT, &id, memory));
    assert_within_reservation("read_analysis_run", |memory| read_run_in_store_with(&path, PROJECT, &id, memory));
    // The state's own label edit and delete are metered too.
    let what = format!("saved state `{state_id}`");
    assert_refused_unloaded(&small, &what, |memory| {
        model_states::edit_labels_in_store_with(&path, PROJECT, &state_id, labels_from_value(json!({ "state_name": "Renamed" })).unwrap(), None, T0, memory)
    });
    assert_refused_unloaded(&small, &what, |memory| model_states::delete_state_in_store_with(&path, PROJECT, &state_id, memory));
    assert_within_reservation("edit_model_state_labels", |memory| {
        model_states::edit_labels_in_store_with(&path, PROJECT, &state_id, labels_from_value(json!({ "state_name": "Renamed" })).unwrap(), None, T0 + 10, memory)
    });
}

#[test]
fn a_reads_reservation_is_held_until_its_response_is_serialized() {
    let (path, _) = recorded_fixture("memory-held");
    let id = id_of(&path);
    let pool = registry(ScriptedHost::normal());
    let memory = PoolReadMemory::unseeded(&pool);
    let metered = read_run_in_store_with(&path, PROJECT, &id, &memory).unwrap();
    let held = pool.pool_status().reserved_bytes;
    assert!(held > 0, "the returned read still holds its reservation");
    let length = stored_read::serialized_length(metered.get()).unwrap();
    let response = stored_read::respond_metered::<_, String>(&memory, "read_analysis_run", Ok(metered)).unwrap();
    assert_eq!(pool.pool_status().reserved_bytes, 0, "released once the response exists");
    assert!(memory.peak_reserved() >= held + crate::ingress::allocation_envelope(length).unwrap(), "the response buffer was reserved on top of the held read");
    drop(response);
    // A response buffer the pool does not admit is refused by name, before it is serialized.
    let small = small_pool();
    let memory = PoolReadMemory::unseeded(&small);
    let Err(refused) = stored_read::respond_metered::<_, String>(&memory, "read_analysis_run", Ok(Metered::new("r".repeat(LARGE)))) else {
        panic!("the response buffer was admitted");
    };
    assert!(matches!(&refused, CommandError::Command(message) if message.starts_with("STORE-READ-MEMORY-UNAVAILABLE: reading the response to read_analysis_run")), "{refused:?}");
}

#[test]
fn record_run_prices_its_extra_parse_before_it_parses() {
    let path = store_with_project("memory-record-run");
    let solved = model("Invented project");
    save_state(&path, &solved, "Basis", "00000000000000000000000000001801", 1_000);
    let manifest = manifest_text(&solved);
    let result = format!(
        "{{\"run_id\":\"{RUN}\",\"model_ref\":\"{PROJECT}\",\"results\":[{{\"id\":\"result:disp:N-100\",\"value\":1.5,\"padding\":\"{}\"}}],\"diagnostics\":[]}}",
        "p".repeat(LARGE)
    );
    let mut submitted = request(&solved, "Large result");
    submitted.run_record = transient(&result, &manifest);
    submitted.result_json = result;
    let small = small_pool();
    let now = || Ok(T0);
    let token = || Ok("0000000000000000000000000000beef".to_string());
    let between_rows = |_: &Transaction<'_>, _: &str| Ok(());
    let hooks = RunWriteHooks { now: &now, state_token: &token, between_rows: &between_rows };
    let error = record_run_in_store_with(&path, submitted, &hooks, &PoolReadMemory::unseeded(&small)).unwrap_err();
    assert!(error.starts_with("STORE-READ-MEMORY-UNAVAILABLE: reading the run to record"), "{error}");
    assert_eq!(row_count(&path, "analysis_runs"), 0);
    let mut invalid = request(&solved, "Invalid result");
    invalid.result_json = invalid_json();
    let error = record_run_in_store_with(&path, invalid, &hooks, &PoolReadMemory::unseeded(&small)).unwrap_err();
    assert!(error.starts_with("ANALYSIS-RUN-RESULT-MISMATCH: the result: the stored JSON is invalid"), "{error}");
}

// ---------------------------------------------------------------------------
// Repair r3: hash and canonical texts are reserved at their exact (escaped)
// lengths, a metered read measures and loads in one snapshot, a value read from
// the store keeps its reservations until it is serialized, and the labels bound
// is the serialized labels object's.
// ---------------------------------------------------------------------------

/// A deterministic xorshift generator for the adversarial values.
struct Xorshift(u64);

impl Xorshift {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, bound: usize) -> usize {
        (self.next() % bound as u64) as usize
    }
}

/// Characters every serializer here escapes or encodes differently: controls,
/// DEL, Latin-1, the JSON short escapes, quotes and backslashes, line and
/// paragraph separators, a BOM, the last BMP character and non-BMP ones.
const ADVERSARIAL: [char; 22] = [
    '\u{0}', '\u{1}', '\u{8}', '\t', '\n', '\u{b}', '\u{c}', '\r', '\u{1f}', ' ', '"', '\\', '/', 'a', '\u{7f}', '\u{80}', '\u{e9}',
    '\u{2028}', '\u{feff}', '\u{ffff}', '\u{1f600}', '\u{10ffff}',
];

fn adversarial_string(random: &mut Xorshift) -> String {
    (0..random.below(24)).map(|_| ADVERSARIAL[random.below(ADVERSARIAL.len())]).collect()
}

/// A random value at most `depth` deep: strings and keys of adversarial
/// characters, extreme integers, and (when `floats`) floats whose canonical forms
/// differ from their serde_json forms.
fn adversarial_value(random: &mut Xorshift, depth: usize, floats: bool) -> Value {
    let integers = [json!(0), json!(-1), json!(i64::MIN), json!(i64::MAX), json!(u64::MAX), json!(9_007_199_254_740_993u64)];
    let doubles = [json!(1e20), json!(1e21), json!(1e-7), json!(-0.0), json!(5e-324), json!(1.7976931348623157e308), json!(0.1), json!(123456789.125)];
    match random.below(if depth == 0 { 4 } else { 6 }) {
        0 => json!(adversarial_string(random)),
        1 => integers[random.below(integers.len())].clone(),
        2 if floats => doubles[random.below(doubles.len())].clone(),
        2 => json!(null),
        3 => json!(random.below(2) == 0),
        4 => Value::Array((0..random.below(5)).map(|_| adversarial_value(random, depth - 1, floats)).collect()),
        _ => Value::Object((0..random.below(5)).map(|_| (adversarial_string(random), adversarial_value(random, depth - 1, floats))).collect()),
    }
}

#[test]
fn the_exact_length_counters_agree_with_the_texts_over_adversarial_values() {
    let mut random = Xorshift(0x9e37_79b9_7f4a_7c15);
    for _ in 0..2_000 {
        // The record hash's sorted-compact text holds no floats.
        let value = adversarial_value(&mut random, 4, false);
        let text = model_states::sorted_compact_json(&value).unwrap();
        assert_eq!(model_states::sorted_compact_length(&value).unwrap(), text.len() as u64, "{value}");
        let value = adversarial_value(&mut random, 4, true);
        let text = canonical_json(&value);
        assert_eq!(stored_read::jcs_length(&value), text.len() as u64, "{value}");
        assert_eq!(stored_read::serialized_length(&value).unwrap(), serde_json::to_string(&value).unwrap().len() as u64);
    }
    // Every character escapes to at most six bytes in the record hash's text.
    let all: String = ADVERSARIAL.iter().collect();
    assert_eq!(model_states::sorted_compact_length(&json!(all)).unwrap(), model_states::sorted_compact_json(&json!(all)).unwrap().len() as u64);
    assert_eq!(model_states::sorted_compact_length(&json!("\u{7f}".repeat(1000))).unwrap(), 6_002);
    // The labels object a label edit carries, as the webview serializes it.
    let labels = model_states::StateLabels {
        state_name: all.clone(),
        tags: vec!["t\u{0}".to_string()],
        notes: vec![all.clone(), "\u{1f600}".to_string()],
        assumptions: vec![json!({ "assumption_id": "A-1", "statement": all, "status": "unresolved" })],
    };
    let carried = json!({ "state_name": labels.state_name, "tags": labels.tags, "notes": labels.notes, "assumptions": labels.assumptions });
    assert_eq!(model_states::labels_json_length("state_name", &labels), serde_json::to_string(&carried).unwrap().len() as u64);
}

/// Codex r3's first probe: a normally saved state whose model holds a 3 MiB
/// assumption of U+007F, which the record hash's text escapes to six bytes a
/// character.
fn escaped_assumption_model() -> Value {
    let mut model = model("Invented project");
    model["unresolved_assumptions"] = json!([{ "assumption_id": "A-escaped", "kind": "assumption", "statement": "\u{7f}".repeat(LARGE), "status": "unresolved" }]);
    model
}

#[test]
fn a_record_hash_text_is_reserved_at_its_escaped_length() {
    let path = store_with_project("memory-escaped-hash");
    let state = save_state(&path, &escaped_assumption_model(), "Escaped", "00000000000000000000000000001901", 1_000);
    let escaped = (LARGE as u64) * 6;
    // The list (Codex's probe) and the read: the priced peak stays within the
    // reservation, which covered the escaped text.
    let mut reserved = 0;
    assert_within_reservation("list_model_states", |memory| {
        let listed = model_states::list_states_in_store_with(&path, PROJECT, memory);
        reserved = memory.peak_reserved();
        listed
    });
    assert!(reserved >= escaped, "the list reserved {reserved}, below the {escaped}-byte escaped hash text");
    assert_within_reservation("read_model_state", |memory| model_states::read_state_in_store_with(&path, PROJECT, &state.state_id, memory));
    // The label edit and the delete verify the record too.
    assert_within_reservation("edit_model_state_labels", |memory| {
        model_states::edit_labels_in_store_with(&path, PROJECT, &state.state_id, labels_from_value(json!({ "state_name": "Renamed" })).unwrap(), None, T0, memory)
    });
    // A run recorded against it (Record Run verifies the state) and its read.
    let solved = escaped_assumption_model();
    let pool = registry(ScriptedHost::normal());
    let memory = PoolReadMemory::unseeded(&pool);
    let now = || Ok(T0);
    let token = || Ok("0000000000000000000000000000c0de".to_string());
    let between_rows = |_: &Transaction<'_>, _: &str| Ok(());
    let hooks = RunWriteHooks { now: &now, state_token: &token, between_rows: &between_rows };
    let outcome = record_run_in_store_with(&path, request(&solved, "Escaped run"), &hooks, &memory).unwrap();
    assert!(memory.peak_reserved() >= escaped, "Record Run reserved the state's escaped hash text");
    let id = outcome.get().run.recording_id.clone();
    drop(outcome);
    assert_within_reservation("read_analysis_run", |memory| read_run_in_store_with(&path, PROJECT, &id, memory));
    // Saving it again (the existing state is verified) and saving a new one
    // (its record is hashed) reserve the escaped text too.
    let model_canonical = canonical_json(&escaped_assumption_model());
    let save = |solved: &Value, token: &'static str| {
        let model_canonical = canonical_json(solved);
        let request = SaveModelStateRequest {
            project_id: PROJECT.to_string(),
            expected_base: BASE.to_string(),
            model_hash: format!("sha256:{}", sha256_hex(&model_canonical)),
            model_canonical,
            state_kind: "design_snapshot".to_string(),
            parent_state_id: None,
            labels: labels_from_value(json!({ "state_name": "Again" })).unwrap(),
        };
        let allocate = move || Ok((token.to_string(), T0));
        let between_rows = |_: &Transaction<'_>| Ok(());
        let memory = PoolReadMemory::unseeded(&pool);
        let outcome = model_states::save_state_in_store_with(&path, request, &StateWriteHooks { allocate: &allocate, between_rows: &between_rows }, &memory).unwrap();
        (outcome.get().outcome, memory.peak_reserved())
    };
    assert!(model_canonical.len() > LARGE);
    let (outcome, reserved) = save(&escaped_assumption_model(), "00000000000000000000000000001902");
    assert_eq!(outcome, "already_saved");
    assert!(reserved >= escaped);
    let mut other = escaped_assumption_model();
    other["project"]["name"] = json!("Other");
    let (outcome, reserved) = save(&other, "00000000000000000000000000001903");
    assert_eq!(outcome, "saved");
    assert!(reserved >= escaped);
    assert_eq!(pool.pool_status().reserved_bytes, 0);
}

/// A pool memory that runs `between` once, at its first reservation: after a
/// read's measurement and before its loads.
struct Interleaved<'a> {
    inner: PoolReadMemory<'a>,
    between: std::cell::RefCell<Option<Box<dyn FnOnce() + 'a>>>,
}

impl ReadMemory for Interleaved<'_> {
    fn reserve(&self, what: &str, bytes: u64) -> Result<stored_read::ReadReservation, String> {
        if let Some(between) = self.between.borrow_mut().take() {
            between();
        }
        self.inner.reserve(what, bytes)
    }
}

/// Codex r3's second probe: a valid run with a 3 MiB record (or a state with a
/// 3 MiB escaped record) is written by another connection between a list's
/// measurement and its loads. The list loads the snapshot it measured.
#[test]
fn a_list_loads_the_snapshot_it_measured() {
    let path = store_with_project("memory-interleaved");
    let solved = model("Invented project");
    save_state(&path, &solved, "Basis", "00000000000000000000000000002001", 1_000);
    record(&path, request(&solved, "Ordinary"), T0).unwrap();
    let small = small_pool();
    // Another connection, on another thread (so the counting allocator measures
    // the list alone), commits a large run while the list's snapshot is open.
    let writer = |path: PathBuf| {
        std::thread::spawn(move || {
            let solved = model("Invented project");
            let mut submitted = request(&solved, "Large, written between");
            submitted.run_record["analysis_run"]["padding"] = json!("r".repeat(LARGE));
            reseal(&mut submitted.run_record);
            record(&path, submitted, T0 + 5).unwrap();
        })
        .join()
        .unwrap()
    };
    let memory = Interleaved { inner: PoolReadMemory::unseeded(&small), between: std::cell::RefCell::new(Some(Box::new(|| writer(path.clone())))) };
    let (listed, measured) = crate::ingress::counting::measure(|| list_runs_in_store_with(&path, PROJECT, &memory));
    let listed = listed.unwrap();
    assert_eq!(listed.get().len(), 1, "the list holds the rows it measured");
    assert!(measured.peak_priced <= memory.inner.peak_reserved(), "peak {} within the reservation {}", measured.peak_priced, memory.inner.peak_reserved());
    assert!(measured.peak_priced < (LARGE as u64) / 4, "the large record written after the measurement was not loaded");
    drop(listed);
    assert_eq!(small.pool_status().reserved_bytes, 0);
    assert_eq!(list_runs(&Connection::open(&path).unwrap(), PROJECT).unwrap().len(), 2, "the run was written");
    // The state list, with a state whose record escapes to 18 MiB.
    let writer = |path: PathBuf| {
        std::thread::spawn(move || {
            save_state(&path, &escaped_assumption_model(), "Escaped, written between", "00000000000000000000000000002002", 2_000);
        })
        .join()
        .unwrap()
    };
    let memory = Interleaved { inner: PoolReadMemory::unseeded(&small), between: std::cell::RefCell::new(Some(Box::new(|| writer(path.clone())))) };
    let (listed, measured) = crate::ingress::counting::measure(|| model_states::list_states_in_store_with(&path, PROJECT, &memory));
    assert_eq!(listed.unwrap().get().len(), 1);
    assert!(measured.peak_priced <= memory.inner.peak_reserved());
    assert_eq!(small.pool_status().reserved_bytes, 0);
    assert_eq!(model_states::list_states(&Connection::open(&path).unwrap(), PROJECT).unwrap().len(), 2);
}

/// A pool memory that records, at each reservation, what the command already holds.
struct Recording<'a> {
    inner: PoolReadMemory<'a>,
    seen: std::cell::RefCell<Vec<(String, u64)>>,
}

impl ReadMemory for Recording<'_> {
    fn reserve(&self, what: &str, bytes: u64) -> Result<stored_read::ReadReservation, String> {
        self.seen.borrow_mut().push((what.to_string(), self.inner.held()));
        self.inner.reserve(what, bytes)
    }
}

/// Codex r3's third probe: a state with a stored 3 MiB name (beyond the write
/// bound, as a hand edit could leave it). The label edit's current summary,
/// which holds the name, keeps its reservation while the record is loaded.
#[test]
fn a_label_edit_holds_the_current_summary_while_it_loads_the_record() {
    let path = store_with_project("memory-held-name");
    let state = save_state(&path, &model("Invented project"), "Basis", "00000000000000000000000000002101", 1_000);
    set_label(&path, "model_state_labels", "state_name", &"n".repeat(LARGE));
    let pool = registry(ScriptedHost::normal());
    let memory = Recording { inner: PoolReadMemory::unseeded(&pool), seen: std::cell::RefCell::new(Vec::new()) };
    let edited = model_states::edit_labels_in_store_with(&path, PROJECT, &state.state_id, labels_from_value(json!({ "state_name": "Renamed" })).unwrap(), None, T0, &memory)
        .unwrap();
    assert_eq!(edited.get().labels.state_name, "Renamed");
    let record_load = format!("saved state `{}`'s record, for the label check", state.state_id);
    let seen = memory.seen.borrow();
    let held = seen.iter().find(|(what, _)| *what == record_load).map(|(_, held)| *held).expect("the record load was reserved");
    assert!(held >= LARGE as u64, "the current summary's {LARGE}-byte name was held ({held}) when the record was reserved");
    drop(seen);
    drop(edited);
    assert_eq!(pool.pool_status().reserved_bytes, 0);
}

/// Delete receipts name what they deleted, its name cut to 120 characters
/// (repair r4); they hold the deleted summary's reservations until they are
/// serialized.
#[test]
fn delete_receipts_are_held_until_they_are_serialized() {
    // A state with a stored 3 MiB name.
    let path = store_with_project("memory-delete-state");
    let state = save_state(&path, &model("Invented project"), "Basis", "00000000000000000000000000002201", 1_000);
    set_label(&path, "model_state_labels", "state_name", &"n".repeat(LARGE));
    let pool = registry(ScriptedHost::normal());
    let memory = PoolReadMemory::unseeded(&pool);
    let receipt = model_states::delete_state_in_store_with(&path, PROJECT, &state.state_id, &memory).unwrap();
    let length = stored_read::serialized_length(receipt.get()).unwrap();
    assert!(length < 1024 && receipt.get().message.contains(&format!("{}…", "n".repeat(120))), "{}", receipt.get().message);
    assert!(receipt.reserved_bytes() >= LARGE as u64, "the receipt holds the deleted summary's reservations ({})", receipt.reserved_bytes());
    assert!(receipt.into_unreserved().is_err(), "a metered receipt does not leave its reservations");
    assert_eq!(pool.pool_status().reserved_bytes, 0);
    // Through serialization (a fresh state: the first was deleted).
    let path = store_with_project("memory-delete-state-2");
    let state = save_state(&path, &model("Invented project"), "Basis", "00000000000000000000000000002202", 1_000);
    set_label(&path, "model_state_labels", "state_name", &"n".repeat(LARGE));
    assert!(assert_within_reservation("delete_model_state", |memory| model_states::delete_state_in_store_with(&path, PROJECT, &state.state_id, memory)) < 1024);
    // A run with a stored 3 MiB name.
    let (path, _) = recorded_fixture("memory-delete-run");
    let id = id_of(&path);
    set_label(&path, "analysis_run_labels", "run_name", &"n".repeat(LARGE));
    assert!(assert_within_reservation("delete_analysis_run", |memory| delete_run_in_store_with(&path, PROJECT, &id, memory)) < 1024);
    assert_eq!(row_count(&path, "analysis_runs"), 0);
}

#[test]
fn the_labels_bound_is_the_serialized_labels_objects_and_every_kept_label_can_be_edited() {
    // Codex r3's fourth probe: 196,602 NULs pass a byte count but serialize to
    // six bytes each.
    let nul_name = "\u{0}".repeat(196_602);
    let error = model_states::labels_from_value(json!({ "state_name": nul_name })).unwrap_err();
    assert!(error.starts_with("MODEL-STATE-LABELS-INVALID: the labels hold 1179667 bytes of JSON; at most 196608 are kept"), "{error}");
    let error = run_labels_from_value(json!({ "run_name": nul_name })).unwrap_err();
    assert!(error.starts_with("ANALYSIS-RUN-LABELS-INVALID: the labels hold"), "{error}");
    // Through Save Model State's and Record Run's decoding too.
    let error = SaveModelStateRequest::from_value(json!({
        "project_id": PROJECT, "expected_base": BASE, "model_canonical": "{}", "model_hash": "sha256:x",
        "state_kind": "design_snapshot", "labels": { "state_name": nul_name }
    }))
    .unwrap_err();
    assert!(error.to_string().contains("bytes of JSON; at most 196608 are kept"), "{error}");
    // The largest kept labels of NULs, carried back to each label edit with a
    // project id at the stated editable length, fit the edit's argument budget.
    let fill = |name_key: &str| {
        let mut count = 32_000usize;
        loop {
            let labels = json!({ name_key: "\u{0}".repeat(count + 1), "tags": [], "notes": [], "assumptions": [] });
            if serde_json::to_string(&labels).unwrap().len() as u64 > model_states::MAX_LABELS_JSON_BYTES {
                return json!({ name_key: "\u{0}".repeat(count), "tags": [], "notes": [], "assumptions": [] });
            }
            count += 1;
        }
    };
    // A project id at the stated editable length (its JSON adds two quotes).
    let project = "p".repeat(model_states::EDITABLE_PROJECT_ID_BYTES - 2);
    let registry = registry(ScriptedHost::normal());
    let state_labels = fill("state_name");
    assert!(model_states::labels_from_value(state_labels.clone()).is_ok());
    let body = serde_json::to_vec(&json!({ "projectId": project, "stateId": format!("state:{}", "0".repeat(32)), "labels": state_labels, "expectedUpdatedAt": "2026-10-24T00:00:00.000Z" })).unwrap();
    let (mut args, _lease) = admit_body_within(
        &registry,
        &InvokeBody::Raw(body),
        &headers(Some(INGRESS_ARGS_JSON_V1)),
        "edit_model_state_labels",
        ArgsDecoding::Moved,
        model_states::EDIT_MODEL_STATE_LABELS_MAX_ARGS_BYTES,
    )
    .unwrap();
    assert!(args.decode_with("labels", |value| labels_from_value(value).map_err(serde::de::Error::custom)).is_ok());
    let run_labels = fill("run_name");
    assert!(run_labels_from_value(run_labels.clone()).is_ok());
    let body = serde_json::to_vec(&json!({ "projectId": project, "recordingId": format!("runrec:{}", "0".repeat(32)), "labels": run_labels, "expectedUpdatedAt": "2026-10-24T00:00:00.000Z" })).unwrap();
    assert!(admit_body_within(
        &registry,
        &InvokeBody::Raw(body),
        &headers(Some(INGRESS_ARGS_JSON_V1)),
        "edit_analysis_run_labels",
        ArgsDecoding::Moved,
        EDIT_ANALYSIS_RUN_LABELS_MAX_ARGS_BYTES,
    )
    .is_ok());
}

// ---------------------------------------------------------------------------
// Repair r4: a mutating command serializes its outcome before it commits;
// messages quote at most 120 characters of a stored name; the project guard
// prices what it reads; the invocation total counts the argument lease.
// ---------------------------------------------------------------------------

/// A pool memory that refuses every reservation whose description starts with
/// `refused` (a response the pool does not admit).
struct RefusedAt<'a> {
    inner: PoolReadMemory<'a>,
    refused: String,
}

impl ReadMemory for RefusedAt<'_> {
    fn reserve(&self, what: &str, bytes: u64) -> Result<stored_read::ReadReservation, String> {
        if what.starts_with(&self.refused) {
            return Err(format!("STORE-READ-MEMORY-UNAVAILABLE: reading {what} needs {bytes} bytes of the memory pool, which refused them (test); nothing more was read."));
        }
        self.inner.reserve(what, bytes)
    }
}

/// Runs a mutating command's argument handler with its response refused: the
/// refusal is reported, by name.
fn refused_response(command: &'static str, args: Value, run: impl FnOnce(&mut Args, &dyn ReadMemory) -> Result<stored_read::Serialized, CommandError>) {
    let pool = registry(ScriptedHost::normal());
    let (mut args, _lease) = admit_json(&pool, command, ArgsDecoding::Moved, &args).unwrap();
    let memory = RefusedAt { inner: PoolReadMemory::unseeded(&pool), refused: format!("the response to {command}") };
    match run(&mut args, &memory) {
        Err(CommandError::Command(message)) => {
            assert!(message.starts_with(&format!("STORE-READ-MEMORY-UNAVAILABLE: reading the response to {command}")), "{message}")
        }
        other => panic!("{command}: the refused response was not reported: {other:?}"),
    }
}

fn labels_row(path: &Path, table: &str, column: &str) -> String {
    Connection::open(path).unwrap().query_row(&format!("SELECT {column} FROM {table}"), [], |row| row.get(0)).unwrap()
}

#[test]
fn a_mutation_whose_response_is_refused_writes_nothing() {
    let (path, _) = recorded_fixture("r4-rollback");
    let store = || Ok(path.clone());
    let state_id: String = Connection::open(&path).unwrap().query_row("SELECT state_id FROM model_states", [], |row| row.get(0)).unwrap();
    let id = id_of(&path);
    let (states, runs) = (row_count(&path, "model_states"), row_count(&path, "analysis_runs"));
    // Save Model State: no new state.
    let other = model("Another invented model");
    let canonical = canonical_json(&other);
    refused_response(
        "save_model_state",
        json!({ "request": { "project_id": PROJECT, "expected_base": BASE, "model_canonical": canonical,
                             "model_hash": format!("sha256:{}", sha256_hex(&canonical)), "state_kind": "design_snapshot",
                             "labels": { "state_name": "Refused" } } }),
        |args, memory| model_states::save_model_state_args(args, store, memory),
    );
    assert_eq!(row_count(&path, "model_states"), states, "Save wrote nothing");
    // Record Run: no new run (and no post-solve snapshot).
    let submitted = with_new_state(request(&other, "Refused run"), "Refused snapshot");
    refused_response(
        "record_analysis_run",
        json!({ "request": { "project_id": PROJECT, "expected_base": BASE, "run_record": submitted.run_record,
                             "result_json": submitted.result_json, "input_manifest_canonical": submitted.input_manifest_canonical,
                             "new_state": { "labels": { "state_name": "Refused snapshot" } }, "run_labels": { "run_name": "Refused run" } } }),
        |args, memory| record_analysis_run_args(args, store, memory),
    );
    assert_eq!((row_count(&path, "model_states"), row_count(&path, "analysis_runs")), (states, runs), "Record Run wrote nothing");
    // Both label edits: the labels are unchanged.
    let (state_name, run_name) = (labels_row(&path, "model_state_labels", "state_name"), labels_row(&path, "analysis_run_labels", "run_name"));
    refused_response(
        "edit_model_state_labels",
        json!({ "projectId": PROJECT, "stateId": state_id, "labels": { "state_name": "Refused rename" }, "expectedUpdatedAt": null }),
        |args, memory| model_states::edit_model_state_labels_args(args, store, memory),
    );
    refused_response(
        "edit_analysis_run_labels",
        json!({ "projectId": PROJECT, "recordingId": id, "labels": { "run_name": "Refused rename" }, "expectedUpdatedAt": null }),
        |args, memory| edit_analysis_run_labels_args(args, store, memory),
    );
    assert_eq!(labels_row(&path, "model_state_labels", "state_name"), state_name);
    assert_eq!(labels_row(&path, "analysis_run_labels", "run_name"), run_name);
    // Both deletes: the rows stay (the run first: the state is its run's).
    refused_response("delete_analysis_run", json!({ "projectId": PROJECT, "recordingId": id }), |args, memory| {
        delete_analysis_run_args(args, store, memory)
    });
    assert_eq!(row_count(&path, "analysis_runs"), runs, "the run was not deleted");
    delete_run_in_store(&path, PROJECT, &id).unwrap();
    refused_response("delete_model_state", json!({ "projectId": PROJECT, "stateId": state_id }), |args, memory| {
        model_states::delete_model_state_args(args, store, memory)
    });
    assert_eq!(row_count(&path, "model_states"), states, "the state was not deleted");
    // Admitted, each commits after its response exists.
    let pool = registry(ScriptedHost::normal());
    let (mut args, _lease) = admit_json(&pool, "delete_model_state", ArgsDecoding::Moved, &json!({ "projectId": PROJECT, "stateId": state_id })).unwrap();
    let response = model_states::delete_model_state_args(&mut args, store, &PoolReadMemory::unseeded(&pool)).unwrap();
    assert!(response.text().contains("\"deleted\":true"), "{}", response.text());
    assert_eq!(row_count(&path, "model_states"), states - 1);
}

/// Writes a stored text column with a bound parameter (a NUL cannot be written
/// as an SQL literal).
fn set_column(path: &Path, table: &str, column: &str, value: &str) {
    Connection::open(path).unwrap().execute(&format!("UPDATE {table} SET {column} = ?1"), params![value]).unwrap();
}

/// A refusal measured within its reservation, its text small (the error is
/// serialized after every lease is released).
fn assert_small_refusal(what: &str, code: &str, refuse: impl FnOnce(&PoolReadMemory<'_>) -> Result<(), String>) {
    let pool = registry(ScriptedHost::normal());
    let memory = PoolReadMemory::unseeded(&pool);
    let (outcome, measured) = crate::ingress::counting::measure(|| refuse(&memory));
    let error = outcome.unwrap_err();
    assert!(error.starts_with(code), "{what}: {error}");
    let serialized = serde_json::to_string(&error).unwrap().len();
    assert!(serialized < 2048 && error.contains('…'), "{what}: the refusal serializes to {serialized} bytes");
    assert!(measured.peak_priced <= memory.peak_reserved(), "{what}: peak {} within the reservation {}", measured.peak_priced, memory.peak_reserved());
    assert_eq!(pool.pool_status().reserved_bytes, 0);
}

#[test]
fn refusals_quote_stored_names_cut_to_a_small_bound() {
    // Codex r4: a blocked delete naming one 3 MiB run name; stale-label
    // refusals quoting a 3 MiB NUL name (six bytes a character serialized).
    let (path, _) = recorded_fixture("r4-refusals");
    let state_id: String = Connection::open(&path).unwrap().query_row("SELECT state_id FROM model_states", [], |row| row.get(0)).unwrap();
    let id = id_of(&path);
    set_column(&path, "analysis_run_labels", "run_name", &"n".repeat(LARGE));
    assert_small_refusal("a blocked delete", "MODEL-STATE-DELETE-BLOCKED: ", |memory| {
        model_states::delete_state_in_store_with(&path, PROJECT, &state_id, memory).map(|_| ())
    });
    let nul = "\u{0}".repeat(LARGE);
    set_column(&path, "model_state_labels", "state_name", &nul);
    set_column(&path, "analysis_run_labels", "run_name", &nul);
    assert_small_refusal("a stale state edit", "MODEL-STATE-LABELS-MOVED: ", |memory| {
        model_states::edit_labels_in_store_with(&path, PROJECT, &state_id, labels_from_value(json!({ "state_name": "New" })).unwrap(), Some("stale"), T0, memory)
            .map(|_| ())
    });
    assert_small_refusal("a stale run edit", "ANALYSIS-RUN-LABELS-MOVED: ", |memory| {
        edit_run_labels_in_store_with(&path, PROJECT, &id, run_labels("New"), Some("stale"), T0, memory).map(|_| ())
    });
    // Many blockers: ten are named, the rest counted.
    let path = store_with_project("r4-blockers");
    let solved = model("Invented project");
    let state = save_state(&path, &solved, "Basis", "00000000000000000000000000002301", 1_000);
    for index in 0..12 {
        record(&path, request(&solved, &format!("Run {index}")), T0 + index).unwrap();
    }
    let error = model_states::delete_state_in_store(&path, PROJECT, &state.state_id).unwrap_err();
    assert!(error.contains(" and 2 more") && error.matches("runrec:").count() == 10, "{error}");
    // The cut itself.
    assert_eq!(stored_read::shown(&"é".repeat(120)), "é".repeat(120));
    assert_eq!(stored_read::shown(&"é".repeat(121)), format!("{}…", "é".repeat(120)));
    let head = "\u{1f600}".repeat(200);
    assert_eq!(stored_read::shown_head(&head.as_bytes()[..stored_read::SHOWN_COLUMN_BYTES], head.len() as i64), format!("{}…", "\u{1f600}".repeat(120)));
}

#[test]
fn the_project_guard_prices_what_it_reads_and_quotes_the_name_only_when_it_refuses() {
    // Codex r4: a 3 MiB stored project name.
    let path = store_with_project("r4-project-name");
    set_column(&path, "local_projects", "project_name", &"p".repeat(LARGE));
    let solved = model("Invented project");
    let model_canonical = canonical_json(&solved);
    let save = |expected_base: &str| SaveModelStateRequest {
        project_id: PROJECT.to_string(),
        expected_base: expected_base.to_string(),
        model_hash: format!("sha256:{}", sha256_hex(&model_canonical)),
        model_canonical: model_canonical.clone(),
        state_kind: "design_snapshot".to_string(),
        parent_state_id: None,
        labels: labels_from_value(json!({ "state_name": "Basis" })).unwrap(),
    };
    let allocate = || Ok(("00000000000000000000000000002401".to_string(), 1_000));
    let between_rows = |_: &Transaction<'_>| Ok(());
    let hooks = StateWriteHooks { allocate: &allocate, between_rows: &between_rows };
    // Saved under the 2 MiB pool, within its reservations (the name is never loaded).
    let small = small_pool();
    let memory = PoolReadMemory::unseeded(&small);
    let (outcome, measured) = crate::ingress::counting::measure(|| model_states::save_state_in_store_with(&path, save(BASE), &hooks, &memory).map(|_| ()));
    outcome.unwrap();
    assert!(measured.peak_priced <= memory.peak_reserved(), "save: peak {} within {}", measured.peak_priced, memory.peak_reserved());
    assert!(measured.peak_priced < (LARGE as u64) / 4);
    // Record Run against it, likewise.
    let memory = PoolReadMemory::unseeded(&small);
    let now = || Ok(T0);
    let token = || Ok("0000000000000000000000000000d00d".to_string());
    let run_rows = |_: &Transaction<'_>, _: &str| Ok(());
    let run_hooks = RunWriteHooks { now: &now, state_token: &token, between_rows: &run_rows };
    let (outcome, measured) = crate::ingress::counting::measure(|| record_run_in_store_with(&path, request(&solved, "Run"), &run_hooks, &memory).map(|_| ()));
    outcome.unwrap();
    assert!(measured.peak_priced <= memory.peak_reserved(), "Record Run: peak {} within {}", measured.peak_priced, memory.peak_reserved());
    assert!(measured.peak_priced < (LARGE as u64) / 4);
    // A stale base is refused quoting the name's head only.
    assert_small_refusal("a stale save", "PROJECT-BASE-MOVED: ", |memory| {
        let mut stale = save("sha256:older");
        stale.model_canonical = canonical_json(&model("Other"));
        stale.model_hash = format!("sha256:{}", sha256_hex(&stale.model_canonical));
        model_states::save_state_in_store_with(&path, stale, &hooks, memory).map(|_| ())
    });
}

#[test]
fn the_invocation_total_counts_the_argument_lease() {
    // Codex r4: a command's argument lease and its reservations together.
    let path = store_with_project("r4-invocation");
    let solved = model("Invented project");
    save_state(&path, &solved, "Basis", "00000000000000000000000000002501", 1_000);
    let now = || Ok(T0);
    let token = || Ok("0000000000000000000000000000f00d".to_string());
    let between_rows = |_: &Transaction<'_>, _: &str| Ok(());
    let hooks = RunWriteHooks { now: &now, state_token: &token, between_rows: &between_rows };
    // What a Record Run reserves, alone.
    let pool = registry(ScriptedHost::normal());
    let probe = store_with_project("r4-invocation-probe");
    save_state(&probe, &solved, "Basis", "00000000000000000000000000002502", 1_000);
    let memory = PoolReadMemory::unseeded(&pool);
    record_run_in_store_with(&probe, request(&solved, "Probe"), &hooks, &memory).unwrap();
    let alone = memory.peak_reserved();
    // A budget that admits the reservations alone, and an argument lease that
    // leaves less than them.
    let budget = alone * 2;
    let pool = registry(ScriptedHost::normal());
    pool.configure_pool(budget * 4, budget, false).unwrap();
    let lease = pool.acquire_interactive_ingress(budget - alone / 2).unwrap();
    let error = record_run_in_store_with(&path, request(&solved, "Over"), &hooks, &PoolReadMemory::new(&pool, &lease)).unwrap_err();
    assert!(error.starts_with("STORE-READ-MEMORY-UNAVAILABLE: ") && error.contains("above the per-invocation budget"), "{error}");
    assert_eq!(row_count(&path, "analysis_runs"), 0, "nothing was recorded");
    drop(lease);
    // Without the argument lease's share the same run is admitted.
    let lease = pool.acquire_interactive_ingress(1024).unwrap();
    record_run_in_store_with(&path, request(&solved, "Within"), &hooks, &PoolReadMemory::new(&pool, &lease)).unwrap();
    assert_eq!(row_count(&path, "analysis_runs"), 1);
}

// ---------------------------------------------------------------------------
// Repair r5: every message is bounded (serde and SQLite errors described, never
// quoted; every stored value cut); every loaded identity and time column is
// measured and bounded before it is loaded; serialized responses and parsed
// values keep their reservations; one blocker cap.
// ---------------------------------------------------------------------------

fn assert_small(what: &str, error: &str) {
    let serialized = serde_json::to_string(error).unwrap().len();
    assert!(serialized < 2048, "{what}: the error serializes to {serialized} bytes: {}", stored_read::shown(error));
}

/// Drops the store's triggers (as `tamper` does) and runs `statement` with one
/// bound text parameter.
fn tamper_with(path: &Path, statement: &str, value: &str) {
    tamper(path, "");
    let connection = Connection::open(path).unwrap();
    connection.execute_batch("PRAGMA foreign_keys = OFF;").unwrap();
    connection.execute(statement, params![value]).unwrap();
}

#[test]
fn stored_decoding_and_sqlite_errors_are_described_not_quoted() {
    // Codex r5: a tags_json holding a 3 MiB NUL string (a string, not a list).
    let nul_string = serde_json::to_string(&"\u{0}".repeat(LARGE)).unwrap();
    let (path, _) = recorded_fixture("r5-typed-errors");
    let state_id: String = Connection::open(&path).unwrap().query_row("SELECT state_id FROM model_states", [], |row| row.get(0)).unwrap();
    set_column(&path, "model_state_labels", "tags_json", &nul_string);
    let error = model_states::read_state_in_store(&path, PROJECT, &state_id).unwrap_err();
    assert!(error.starts_with("MODEL-STATE-INTEGRITY: a stored label list is malformed (a value of another type or shape)"), "{}", stored_read::shown(&error));
    assert_small("a state's typed label error", &error);
    set_column(&path, "analysis_run_labels", "tags_json", &nul_string);
    let error = list_runs(&Connection::open(&path).unwrap(), PROJECT).unwrap_err();
    assert!(error.starts_with("ANALYSIS-RUN-INTEGRITY: a stored run label list is malformed"), "{}", stored_read::shown(&error));
    assert_small("a run's typed label error", &error);

    // Codex r5: a long recording id with no labels row (within the id bound),
    // and one past the bound.
    let (path, _) = recorded_fixture("r5-missing-labels");
    let copy = |recording_id: &str| {
        tamper_with(
            &path,
            "INSERT INTO analysis_runs (project_id, recording_id, state_id, producer_run_id, created_at, run_record_json, run_record_hash, input_manifest_json, result_json, result_hash)
             SELECT project_id, ?1, state_id, producer_run_id, created_at, run_record_json, run_record_hash || 'x', input_manifest_json, result_json, result_hash FROM analysis_runs LIMIT 1",
            recording_id,
        )
    };
    copy(&"\u{0}".repeat(4000));
    let error = list_runs(&Connection::open(&path).unwrap(), PROJECT).unwrap_err();
    assert!(error.contains("has no labels row"), "{}", stored_read::shown(&error));
    assert_small("a missing labels row", &error);
    tamper(&path, "DELETE FROM analysis_runs WHERE length(recording_id) = 0 OR recording_id NOT LIKE 'runrec:%';");
    copy(&"r".repeat(LARGE));
    let error = list_runs(&Connection::open(&path).unwrap(), PROJECT).unwrap_err();
    assert!(error.contains("its recording_id is longer than the 4096 bytes kept"), "{}", stored_read::shown(&error));
    assert_small("an oversized recording id", &error);

    // Codex r5: a stored trigger whose message is 3 MiB.
    let (path, _) = recorded_fixture("r5-trigger");
    let id = id_of(&path);
    Connection::open(&path)
        .unwrap()
        .execute_batch(&format!("CREATE TRIGGER invented_noise BEFORE UPDATE ON analysis_run_labels BEGIN SELECT RAISE(ABORT, '{}'); END;", "t".repeat(LARGE)))
        .unwrap();
    let error = edit_run_labels_in_store(&path, PROJECT, &id, run_labels("Renamed"), None, T0).unwrap_err();
    assert!(error.starts_with("STORE-SQL-ERROR: SQLite ConstraintViolation") && error.contains("its message is not shown"), "{}", stored_read::shown(&error));
    assert_small("a trigger's message", &error);
    // A short SQLite message is kept (the store's own trigger codes, constraints).
    assert!(stored_read::sql_error(&rusqlite::Error::SqliteFailure(rusqlite::ffi::Error::new(19), Some("MODEL-STATE-IMMUTABLE".to_string())))
        .ends_with(": MODEL-STATE-IMMUTABLE"));
    // Every message a command returns is capped as a last net.
    assert_eq!(stored_read::capped("e".repeat(5000)).chars().count(), stored_read::MAX_MESSAGE_CHARS + 1);
}

/// A row whose identity or time column holds `LARGE` bytes is refused before
/// it is loaded, by the head of its id, within the reservations, under the
/// 2 MiB pool.
fn assert_oversized_refused<T: std::fmt::Debug>(what: &str, column: &str, read: impl FnOnce(&PoolReadMemory<'_>) -> Result<T, String>) {
    let small = small_pool();
    let memory = PoolReadMemory::unseeded(&small);
    let (outcome, measured) = crate::ingress::counting::measure(|| read(&memory));
    let error = outcome.unwrap_err();
    assert!(error.contains(&format!("its {column} is longer than")), "{what}: {}", stored_read::shown(&error));
    assert_small(what, &error);
    assert!(measured.peak_priced < (LARGE as u64) / 4, "{what}: refused before loading (peak {})", measured.peak_priced);
    assert!(measured.peak_priced <= memory.peak_reserved().max(1 << 16), "{what}: peak {} within {}", measured.peak_priced, memory.peak_reserved());
    assert_eq!(small.pool_status().reserved_bytes, 0);
}

#[test]
fn every_identity_and_time_column_is_bounded_before_it_is_loaded() {
    let large = "z".repeat(LARGE);
    // A state's columns, through the list and the read.
    for (column, table, set) in [
        ("state_id", "model_states", "state_id"),
        ("state_kind", "model_states", "state_kind"),
        ("created_at", "model_states", "created_at"),
        ("parent_state_id", "model_states", "parent_state_id"),
        ("model_hash", "model_states", "model_hash"),
        ("record_hash", "model_states", "record_hash"),
        ("labels_updated_at", "model_state_labels", "updated_at"),
    ] {
        let path = store_with_project(&format!("r5-state-{column}"));
        let state = save_state(&path, &model("Invented project"), "Basis", "00000000000000000000000000002601", 1_000);
        tamper_with(&path, &format!("UPDATE {table} SET {set} = ?1"), &large);
        assert_oversized_refused(&format!("the state list ({column})"), column, |memory| model_states::list_states_in_store_with(&path, PROJECT, memory));
        if column != "state_id" {
            assert_oversized_refused(&format!("the state read ({column})"), column, |memory| {
                model_states::read_state_in_store_with(&path, PROJECT, &state.state_id, memory)
            });
        }
    }
    // A run's columns, through the list and the read.
    for (column, table, set) in [
        ("recording_id", "analysis_runs", "recording_id"),
        ("state_id", "analysis_runs", "state_id"),
        ("created_at", "analysis_runs", "created_at"),
        ("run_record_hash", "analysis_runs", "run_record_hash"),
        ("result_hash", "analysis_runs", "result_hash"),
        ("labels_updated_at", "analysis_run_labels", "updated_at"),
    ] {
        let (path, _) = recorded_fixture(&format!("r5-run-{column}"));
        let id = id_of(&path);
        tamper_with(&path, &format!("UPDATE {table} SET {set} = ?1"), &large);
        assert_oversized_refused(&format!("the run list ({column})"), column, |memory| list_runs_in_store_with(&path, PROJECT, memory));
        if column != "recording_id" {
            assert_oversized_refused(&format!("the run read ({column})"), column, |memory| read_run_in_store_with(&path, PROJECT, &id, memory));
        }
    }
    // The latest created_at Record Run reads: refused before it is loaded.
    let (path, _) = recorded_fixture("r5-latest-created-at");
    tamper_with(&path, "UPDATE analysis_runs SET created_at = ?1", &large);
    let solved = model("Invented project");
    let error = record(&path, request(&solved, "After"), T0 + 99).unwrap_err();
    assert!(error.contains("its created_at is longer than the 32 bytes kept"), "{}", stored_read::shown(&error));
    assert_small("the latest created_at", &error);
    // The existing state Save finds by its model hash: its id is loaded only
    // within its bound.
    let path = store_with_project("r5-existing-state-id");
    save_state(&path, &model("Invented project"), "Basis", "00000000000000000000000000002602", 1_000);
    tamper_with(&path, "UPDATE model_states SET state_id = ?1", &large);
    let model_canonical = canonical_json(&model("Invented project"));
    let again = SaveModelStateRequest {
        project_id: PROJECT.to_string(),
        expected_base: BASE.to_string(),
        model_hash: format!("sha256:{}", sha256_hex(&model_canonical)),
        model_canonical,
        state_kind: "design_snapshot".to_string(),
        parent_state_id: None,
        labels: labels_from_value(json!({ "state_name": "Again" })).unwrap(),
    };
    let allocate = || Ok(("00000000000000000000000000002603".to_string(), 2_000));
    let between_rows = |_: &Transaction<'_>| Ok(());
    let error = save_state_in_store(&path, again, &StateWriteHooks { allocate: &allocate, between_rows: &between_rows }).unwrap_err();
    assert!(error.contains("its state_id is longer than the 4096 bytes kept"), "{}", stored_read::shown(&error));
    assert_small("the existing state's id", &error);
}

#[test]
fn a_serialized_response_and_a_parsed_value_keep_their_reservations() {
    // Codex r5: recording against a state with a large stored name; the
    // response (3 MiB) stays reserved across the commit until it is handed on.
    let path = store_with_project("r5-held-across-commit");
    let solved = model("Invented project");
    let state = save_state(&path, &solved, "Basis", "00000000000000000000000000002701", 1_000);
    set_column(&path, "model_state_labels", "state_name", &"n".repeat(LARGE));
    let pool = registry(ScriptedHost::normal());
    let memory = PoolReadMemory::unseeded(&pool);
    let now = || Ok(T0);
    let token = || Ok("0000000000000000000000000000abba".to_string());
    let between_rows = |_: &Transaction<'_>, _: &str| Ok(());
    let hooks = RunWriteHooks { now: &now, state_token: &token, between_rows: &between_rows };
    let held_at_finish = Cell::new(0u64);
    let serialized = record_run_in_store_finishing(&path, request(&solved, "Run"), &hooks, &memory, |outcome| {
        let serialized = stored_read::serialize_metered(&memory, "record_analysis_run", outcome)?;
        held_at_finish.set(pool.pool_status().reserved_bytes);
        Ok(serialized)
    })
    .unwrap();
    let length = serialized.text().len() as u64;
    assert!(length > LARGE as u64 && serialized.text().contains(&state.state_id));
    assert!(held_at_finish.get() >= length, "the response was reserved when the commit began ({})", held_at_finish.get());
    let after_commit = pool.pool_status().reserved_bytes;
    assert!(after_commit >= length && serialized.reserved_bytes() >= length, "the response stays reserved after the commit ({after_commit})");
    assert_eq!(row_count(&path, "analysis_runs"), 1, "committed");
    let response = stored_read::respond_serialized::<String>(Ok(serialized)).unwrap();
    assert_eq!(pool.pool_status().reserved_bytes, 0, "released once handed to Tauri");
    drop(response);
}

#[test]
fn the_project_guard_refuses_a_stored_hash_the_prescan_refuses() {
    // Codex r5's minor finding, kept as the new behaviour: a stored envelope
    // hash holding serde_json's private RawValue key. The former guard parsed
    // it and admitted the matching base; the prescan refuses the key, so the
    // stored hash reads as none and the save asks (PROJECT-BASE-MOVED).
    let path = store_with_project("r5-raw-value-key");
    let text = format!(r#"{{"value":"{BASE}","$serde_json::private::RawValue":"ignored"}}"#);
    set_column(&path, "local_projects", "project_envelope_hash_json", &text);
    assert_eq!(crate::stored_envelope_hash_value(&Connection::open(&path).unwrap(), PROJECT).unwrap(), BASE, "the former guard's reading");
    let model_canonical = canonical_json(&model("Invented project"));
    let request = SaveModelStateRequest {
        project_id: PROJECT.to_string(),
        expected_base: BASE.to_string(),
        model_hash: format!("sha256:{}", sha256_hex(&model_canonical)),
        model_canonical,
        state_kind: "design_snapshot".to_string(),
        parent_state_id: None,
        labels: labels_from_value(json!({ "state_name": "Basis" })).unwrap(),
    };
    let allocate = || Ok(("00000000000000000000000000002801".to_string(), 1_000));
    let between_rows = |_: &Transaction<'_>| Ok(());
    let error = save_state_in_store(&path, request, &StateWriteHooks { allocate: &allocate, between_rows: &between_rows }).unwrap_err();
    assert!(error.starts_with("PROJECT-BASE-MOVED: "), "{error}");
    // An ordinary stored hash row is read as before.
    set_column(&path, "local_projects", "project_envelope_hash_json", &format!(r#"{{"algorithm":"sha256","value":"{BASE}"}}"#));
    save_state(&path, &model("Invented project"), "Basis", "00000000000000000000000000002802", 1_000);
}

#[test]
fn one_cap_names_at_most_ten_blockers_across_children_and_runs() {
    let path = store_with_project("r5-blockers");
    let solved = model("Invented project");
    let parent = save_state(&path, &solved, "Basis", "00000000000000000000000000002901", 1_000);
    for index in 0..6 {
        let child = model(&format!("Child {index}"));
        let model_canonical = canonical_json(&child);
        let request = SaveModelStateRequest {
            project_id: PROJECT.to_string(),
            expected_base: BASE.to_string(),
            model_hash: format!("sha256:{}", sha256_hex(&model_canonical)),
            model_canonical,
            state_kind: "design_snapshot".to_string(),
            parent_state_id: Some(parent.state_id.clone()),
            labels: labels_from_value(json!({ "state_name": format!("Child {index}") })).unwrap(),
        };
        let token = format!("{:032}", 2910 + index);
        let allocate = move || Ok((token.clone(), 2_000 + index as i64));
        let between_rows = |_: &Transaction<'_>| Ok(());
        save_state_in_store(&path, request, &StateWriteHooks { allocate: &allocate, between_rows: &between_rows }).unwrap();
        record(&path, request_against(&solved, &format!("Run {index}")), T0 + index as i64).unwrap();
    }
    let error = model_states::delete_state_in_store(&path, PROJECT, &parent.state_id).unwrap_err();
    // The state itself, then ten blockers in all: six children and four runs.
    assert_eq!(error.matches("\" (").count(), 11, "{error}");
    assert!(error.contains("recorded runs ") && error.contains(" and 2 more"), "{error}");
}

fn request_against(model: &Value, name: &str) -> RecordRunRequest {
    request(model, name)
}


// ---------------------------------------------------------------------------
// Repair r6: nothing parsed outlives its lease (drop order); the status copy is
// reserved as retained; ids the app does not bound are measured, not refused;
// every message's values are cut; stored times never panic.
// ---------------------------------------------------------------------------

/// Runs `command` (its outcome dropped inside the measurement) and checks, at
/// every release of a reservation, that what is still live is within what is
/// still held, with 1 MiB for the command's small unpriced allocations: a
/// value that outlives its reservation shows as live bytes above it.
fn assert_nothing_outlives_its_lease<R>(what: &str, command: impl FnOnce() -> R) {
    const SLACK: i64 = 1 << 20;
    let (releases, _) = crate::ingress::counting::measure(|| {
        stored_read::release_probe::start();
        drop(command());
        stored_read::release_probe::take()
    });
    assert!(!releases.is_empty(), "{what}: nothing was reserved");
    for (index, (live, held)) in releases.iter().enumerate() {
        assert!(*live <= *held as i64 + SLACK, "{what}: release {index} left {live} bytes live with {held} held");
    }
}

#[test]
fn nothing_parsed_outlives_its_lease() {
    // Codex r6: an envelope hash holding 100,000 empty strings, read by the
    // project guard.
    let path = store_with_project("r6-drop-order");
    let padding = vec![""; 100_000];
    set_column(&path, "local_projects", "project_envelope_hash_json", &serde_json::to_string(&json!({ "algorithm": "sha256", "value": BASE, "padding": padding })).unwrap());
    let pool = registry(ScriptedHost::normal());
    let model_canonical = canonical_json(&model("Invented project"));
    let save = |token: &'static str, model_canonical: String| {
        let request = SaveModelStateRequest {
            project_id: PROJECT.to_string(),
            expected_base: BASE.to_string(),
            model_hash: format!("sha256:{}", sha256_hex(&model_canonical)),
            model_canonical,
            state_kind: "design_snapshot".to_string(),
            parent_state_id: None,
            labels: labels_from_value(json!({ "state_name": "Basis" })).unwrap(),
        };
        let allocate = move || Ok((token.to_string(), 1_000));
        let between_rows = |_: &Transaction<'_>| Ok(());
        let memory = PoolReadMemory::unseeded(&pool);
        model_states::save_state_in_store_with(&path, request, &StateWriteHooks { allocate: &allocate, between_rows: &between_rows }, &memory)
    };
    assert_nothing_outlives_its_lease("save, the project guard", || save("00000000000000000000000000003001", model_canonical.clone()).unwrap());
    // The same probe over the other commands, with large stored rows.
    let (path, _) = recorded_fixture("r6-drop-order-sweep");
    let id = id_of(&path);
    let state_id: String = Connection::open(&path).unwrap().query_row("SELECT state_id FROM model_states", [], |row| row.get(0)).unwrap();
    set_column(&path, "model_state_labels", "notes_json", &serde_json::to_string(&vec!["n".repeat(4096); 600]).unwrap());
    set_column(&path, "analysis_run_labels", "notes_json", &serde_json::to_string(&vec!["n".repeat(4096); 600]).unwrap());
    let memory = PoolReadMemory::unseeded(&pool);
    assert_nothing_outlives_its_lease("list states", || model_states::list_states_in_store_with(&path, PROJECT, &memory).unwrap());
    assert_nothing_outlives_its_lease("read state", || model_states::read_state_in_store_with(&path, PROJECT, &state_id, &memory).unwrap());
    assert_nothing_outlives_its_lease("list runs", || list_runs_in_store_with(&path, PROJECT, &memory).unwrap());
    assert_nothing_outlives_its_lease("read run", || read_run_in_store_with(&path, PROJECT, &id, &memory).unwrap());
    assert_nothing_outlives_its_lease("edit run", || edit_run_labels_in_store_with(&path, PROJECT, &id, run_labels("Renamed"), None, T0 + 1, &memory).unwrap());
    assert_nothing_outlives_its_lease("edit state", || {
        model_states::edit_labels_in_store_with(&path, PROJECT, &state_id, labels_from_value(json!({ "state_name": "Renamed" })).unwrap(), None, T0 + 1, &memory).unwrap()
    });
    assert_nothing_outlives_its_lease("delete run", || delete_run_in_store_with(&path, PROJECT, &id, &memory).unwrap());
    assert_nothing_outlives_its_lease("delete state", || model_states::delete_state_in_store_with(&path, PROJECT, &state_id, &memory).unwrap());
    assert_eq!(pool.pool_status().reserved_bytes, 0);
}

/// Rewrites one recorded run's record coherently (its hash, recording id and
/// labels row follow it).
fn forge_one(path: &Path, recording_id: &str, record: &Value) -> String {
    let hash = run_record_hash(record).unwrap();
    let forged = recording_id_of(&hash);
    tamper(path, "");
    let connection = Connection::open(path).unwrap();
    connection.execute_batch("PRAGMA foreign_keys = OFF;").unwrap();
    connection
        .execute(
            "UPDATE analysis_runs SET run_record_json = ?1, run_record_hash = ?2, recording_id = ?3 WHERE recording_id = ?4",
            params![serde_json::to_string(record).unwrap(), hash, forged, recording_id],
        )
        .unwrap();
    connection.execute("UPDATE analysis_run_labels SET recording_id = ?1 WHERE recording_id = ?2", params![forged, recording_id]).unwrap();
    forged
}

#[test]
fn a_status_list_is_reserved_as_it_is_retained_and_record_run_keeps_v0_3s_words() {
    // Record Run accepts v0.3's status words, each at most once.
    let path = store_with_project("r6-status-record");
    let solved = model("Invented project");
    save_state(&path, &solved, "Basis", "00000000000000000000000000003101", 1_000);
    for status in [json!(vec![""; 1000]), json!(["HUMAN_REVIEW_REQUIRED", "HUMAN_REVIEW_REQUIRED"]), json!(["HUMAN_REVIEW_REQUIRED", "INVENTED"])] {
        let mut submitted = request(&solved, "Status");
        submitted.run_record["analysis_run"]["analysis_status"] = status;
        reseal(&mut submitted.run_record);
        let error = record(&path, submitted, T0).unwrap_err();
        assert!(error.starts_with("ANALYSIS-RUN-NOT-TRANSIENT: the record's analysis status is not a list of v0.3's status words"), "{error}");
    }
    assert_eq!(row_count(&path, "analysis_runs"), 0);
    // Codex r6: 128 runs whose stored records (forged past Record Run) hold
    // 1,000 empty status strings each: the list reserves the retained copies.
    for index in 0..128 {
        let outcome = record(&path, request(&solved, &format!("Run {index}")), T0 + index).unwrap();
        let mut stored = read_run_in_store(&path, PROJECT, &outcome.run.recording_id).unwrap().record;
        stored["analysis_run"]["analysis_status"] = json!(vec![""; 1000]);
        reseal(&mut stored);
        forge_one(&path, &outcome.run.recording_id, &stored);
    }
    assert_within_reservation("list_analysis_runs", |memory| list_runs_in_store_with(&path, PROJECT, memory));
    let pool = registry(ScriptedHost::normal());
    let memory = PoolReadMemory::unseeded(&pool);
    assert_nothing_outlives_its_lease("the list of 128 runs", || list_runs_in_store_with(&path, PROJECT, &memory).unwrap());
}

#[test]
fn long_project_ids_are_measured_not_refused() {
    // Codex r6: S1-era data with a project id past 4 KiB (Save Local Project
    // keeps any). Every history command still works on it.
    for length in [4_097usize, 64 * 1024] {
        let project = format!("project:{}", "p".repeat(length - 8));
        let path = scratch_store(&format!("r6-long-project-{length}"));
        let solved = model_in(&project, "Invented project");
        {
            let (mut connection, _) = crate::open_project_store(&path).unwrap();
            write_project(&mut connection, &project, &solved, BASE);
        }
        let model_canonical = canonical_json(&solved);
        let save = |token: &'static str| {
            let request = SaveModelStateRequest {
                project_id: project.clone(),
                expected_base: BASE.to_string(),
                model_hash: format!("sha256:{}", sha256_hex(&model_canonical)),
                model_canonical: model_canonical.clone(),
                state_kind: "design_snapshot".to_string(),
                parent_state_id: None,
                labels: labels_from_value(json!({ "state_name": "Basis" })).unwrap(),
            };
            let allocate = move || Ok((token.to_string(), 1_000));
            let between_rows = |_: &Transaction<'_>| Ok(());
            save_state_in_store(&path, request, &StateWriteHooks { allocate: &allocate, between_rows: &between_rows })
        };
        let state = save("00000000000000000000000000003201").unwrap().state;
        let run = record(&path, request_in(&project, &solved, "Run"), T0).unwrap().run;
        let pool = registry(ScriptedHost::normal());
        let memory = PoolReadMemory::unseeded(&pool);
        assert_eq!(model_states::list_states_in_store_with(&path, &project, &memory).unwrap().get().len(), 1, "{length}: list states");
        assert_eq!(list_runs_in_store_with(&path, &project, &memory).unwrap().get().len(), 1, "{length}: list runs");
        model_states::read_state_in_store_with(&path, &project, &state.state_id, &memory).unwrap();
        read_run_in_store_with(&path, &project, &run.recording_id, &memory).unwrap();
        // The label edits and the duplicate save, through their commands.
        let (mut args, lease) = admit_json(&pool, "edit_model_state_labels", ArgsDecoding::Moved,
            &json!({ "projectId": project, "stateId": state.state_id, "labels": { "state_name": "Renamed" }, "expectedUpdatedAt": null })).unwrap();
        model_states::edit_model_state_labels_args(&mut args, || Ok(path.clone()), &memory).unwrap();
        drop((args, lease));
        let (mut args, lease) = admit_json(&pool, "edit_analysis_run_labels", ArgsDecoding::Moved,
            &json!({ "projectId": project, "recordingId": run.recording_id, "labels": { "run_name": "Renamed" }, "expectedUpdatedAt": null })).unwrap();
        edit_analysis_run_labels_args(&mut args, || Ok(path.clone()), &memory).unwrap();
        drop((args, lease));
        let again = save("00000000000000000000000000003202").unwrap();
        assert_eq!((again.outcome, again.state.state_id.as_str()), ("already_saved", state.state_id.as_str()), "{length}: duplicate save");
        // A refusal quoting the id stays small (Codex r6: ANALYSIS-RUN-STATE-REQUIRED).
        let other = model_in(&project, "Unsaved model");
        let error = record(&path, request_in(&project, &other, "Unsaved"), T0 + 1).unwrap_err();
        assert!(error.starts_with("ANALYSIS-RUN-STATE-REQUIRED: "), "{}", stored_read::shown(&error));
        assert_small("a refusal quoting a long project id", &error);
        let error = model_states::delete_state_in_store(&path, &project, &state.state_id).unwrap_err();
        assert_small("a blocked delete of a long project id", &error);
        assert_eq!(pool.pool_status().reserved_bytes, 0);
    }
}

#[test]
fn a_stored_time_with_a_multi_byte_character_is_refused_not_a_panic() {
    // Codex r6: 24 bytes, the last character two bytes wide.
    let text = format!("{}é", "x".repeat(22));
    assert_eq!(text.len(), 24);
    assert_eq!(parse_iso_millis(&text), None);
    assert_eq!(parse_iso_millis("2026-10-24T00:00:00.000Z"), Some(1_792_800_000_000));
    // Through Record Run's latest created_at, and a label edit's updated_at.
    let (path, _) = recorded_fixture("r6-time-utf8");
    tamper_with(&path, "UPDATE analysis_runs SET created_at = ?1", &text);
    let error = record(&path, request(&model("Invented project"), "After"), T0 + 5).unwrap_err();
    assert!(error.contains("is not a recording time"), "{error}");
    let (path, _) = recorded_fixture("r6-time-utf8-labels");
    let id = id_of(&path);
    set_column(&path, "analysis_run_labels", "updated_at", &text);
    edit_run_labels_in_store(&path, PROJECT, &id, run_labels("Renamed"), None, T0 + 6).unwrap();
}

fn model_in(project: &str, name: &str) -> Value {
    json!({
        "schema_version": open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION,
        "document_kind": "openpipestress.product_preview.model",
        "project": { "id": project, "name": name, "units": { "length": "m" } },
        "nodes": [{ "id": "node:N-100", "coordinates": [0, 0, 0] }],
        "pipe_segments": [], "supports": [], "materials": [], "sections": [], "components": [],
        "load_cases": [], "combinations": []
    })
}

fn manifest_text_in(project: &str, model: &Value) -> String {
    canonical_json(&json!({
        "schema_version": "1.0.0",
        "document_kind": "openpipestress.current_session_input_manifest",
        "model_basis": { "model_ref": project, "model_payload": model },
        "unit_basis": { "project_units": { "length": "m" } },
        "solver_basis": { "solver_name": "invented-solver", "solver_version": "0.0.1", "solver_build_ref": "invented-solver@0.0.1",
                          "solver_mode": "sparse_interactive", "settings": { "sparse_evidence_lane": true } },
        "load_basis": { "load_cases": [], "combinations": [] },
        "active_rule_packs": [{ "rule_pack_id": "rule-pack:invented", "version": "1.0.0", "checksum_sha256": "a".repeat(64), "source_notice": "invented" }],
        "external_assets": [],
        "identity_policy": { "canonicalization": "rfc8785_jcs", "hash_algorithm": "sha256", "canonical_bytes_scope": "entire_input_manifest_object" },
        "replay_boundary": { "included_as_package_member": false, "portable_replay_claimed": false, "current_session_ref_hash_integrity_only": true }
    }))
}

fn result_text_in(project: &str, value: f64) -> String {
    let project_json = serde_json::to_string(project).unwrap();
    format!(
        "{{ \"run_id\": \"{RUN}\", \"model_ref\": {project_json}, \"results\": [ {{ \"id\": \"result:disp:N-100\", \"value\": {value}, \"offset\": -0 }} ], \"diagnostics\": [] }}"
    )
}

fn transient_in(project: &str, result: &str, manifest: &str) -> Value {
    let manifest_hash = sha256_hex(manifest);
    let manifest_ref = json!({ "object_type": "InputManifest", "ref": format!("input-manifest:{}:{manifest_hash}", safe_token(project)) });
    let rows: Value = serde_json::from_str(result).unwrap();
    let result_refs: Vec<Value> = rows["results"].as_array().unwrap().iter().enumerate().map(|(index, row)| {
        let id = row["id"].as_str().unwrap();
        json!({ "result_ref": { "object_type": "Result", "ref": id }, "source_row_index": index,
                "hash_refs": [{ "algorithm": "sha256", "canonicalization": CHECKED_PROFILE_V1, "payload_ref": { "object_type": "Result", "ref": id },
                                "payload_scope": "result_row", "value": checked_sha256(row).unwrap() }] })
    }).collect();
    let mut record = json!({
        "schema_version": "0.3.0",
        "run_contract_status": { "record_contract": "strict_analysis_run_v0_3", "result_binding": "received_mechanics_result",
                                 "external_validation_boundary": "reference_only_not_determined_by_software" },
        "analysis_run": {
            "run_id": RUN, "run_name": format!("{RUN} analysis record"), "run_kind": "mechanics_solve", "created_at": null,
            "model_state_ref": { "object_type": "ModelState", "ref": format!("state:{project}:preview") },
            "solver_version": { "solver_name": "invented-solver", "solver_version": "0.0.1",
                                "build_ref": { "object_type": "ExternalReference", "ref": "invented-solver@0.0.1" } },
            "settings_ref": { "object_type": "SolverSettings", "ref": format!("solver-settings:input-manifest:{}:{manifest_hash}:{manifest_hash}", safe_token(project)) },
            "unit_system_ref": { "object_type": "UnitSystem", "ref": format!("unit-system:{project}:{manifest_hash}") },
            "load_basis_refs": [], "result_refs": result_refs,
            "rule_pack_refs": [], "library_refs": [],
            "analysis_status": ["HUMAN_REVIEW_REQUIRED", "MECHANICS_SOLVED", "RULE_INPUTS_INCOMPLETE"],
            "hashes": [
                { "algorithm": "sha256", "canonicalization": CHECKED_PROFILE_V1, "payload_ref": { "object_type": "AnalysisRun", "ref": RUN },
                  "payload_scope": "analysis_run_record", "value": "" },
                { "algorithm": "sha256", "canonicalization": CHECKED_PROFILE_V1, "payload_ref": { "object_type": "ResultEnvelope", "ref": format!("result-envelope:{RUN}") },
                  "payload_scope": "received_result", "value": sha256_hex(&canonical_json_checked_v1_text(result).unwrap()) }
            ],
            "reproducibility": {
                "input_manifest_refs": [manifest_ref.clone()],
                "input_manifest_hashes": [{ "algorithm": "sha256", "canonicalization": "rfc8785_jcs", "payload_ref": manifest_ref,
                                            "payload_scope": "input_manifest", "value": manifest_hash }],
                "determinism_notes": ["created_at_unavailable", "model_state_ref_and_solver_settings_unit_basis_bound_by_input_manifest"]
            },
            "scale": 0.1
        }
    });
    reseal(&mut record);
    record
}

fn request_in(project: &str, model: &Value, name: &str) -> RecordRunRequest {
    let manifest = manifest_text_in(project, model);
    let result = result_text_in(project, 1.25);
    RecordRunRequest {
        project_id: project.to_string(),
        expected_base: BASE.to_string(),
        run_record: transient_in(project, &result, &manifest),
        result_json: result,
        input_manifest_canonical: manifest,
        state_id: None,
        new_state: None,
        run_labels: run_labels(name),
    }
}

// ---------------------------------------------------------------------------
// Repair r7: the history commands reach a project id of any length up to their
// 256 KiB argument budget (the app keeps any project id); a longer request is
// refused by name at ingress.
// ---------------------------------------------------------------------------

/// Admits `args` for `command` within `budget`, as the command does.
fn admit_command(pool: &crate::solve_jobs::SolveJobRegistry, command: &'static str, budget: usize, args: &Value) -> Result<(Args, crate::memory_pool::Lease), SolveJobError> {
    admit_body_within(pool, &InvokeBody::Raw(serde_json::to_vec(args).unwrap()), &headers(Some(INGRESS_ARGS_JSON_V1)), command, ArgsDecoding::Moved, budget)
}

fn serialized_value(serialized: stored_read::Serialized) -> Value {
    serde_json::from_str(serialized.text()).unwrap()
}

#[test]
fn the_history_commands_reach_long_project_ids_written_by_the_real_writers() {
    for length in [4_097usize, 64 * 1024] {
        let project = format!("project:{}", "p".repeat(length - 8));
        let path = scratch_store(&format!("r7-long-project-{length}"));
        let store = || Ok(path.clone());
        let pool = registry(ScriptedHost::normal());
        // The project, through Create Local Project's decoder.
        let mut solved = crate::load_preview_model().unwrap();
        solved["project"]["id"] = json!(project);
        let (mut args, lease) = admit_json(&pool, "create_local_project", ArgsDecoding::Moved,
            &json!({ "model": solved, "projectEnvelopeHash": { "value": BASE }, "confirmedRevision": null })).unwrap();
        crate::create_local_project_args(&mut args, store).unwrap();
        drop((args, lease));
        // A state, through Save Model State's decoder.
        let canonical = canonical_json(&solved);
        let memory = PoolReadMemory::unseeded(&pool);
        let (mut args, lease) = admit_json(&pool, "save_model_state", ArgsDecoding::Moved, &json!({ "request": {
            "project_id": project, "expected_base": BASE, "model_canonical": canonical, "model_hash": format!("sha256:{}", sha256_hex(&canonical)),
            "state_kind": "design_snapshot", "labels": { "state_name": "Basis" } } })).unwrap();
        let saved = serialized_value(model_states::save_model_state_args(&mut args, store, &memory).unwrap());
        drop((args, lease));
        let state_id = saved["state"]["state_id"].as_str().unwrap().to_string();
        // A run, through Record Run's decoder.
        let submitted = request_in(&project, &solved, "Run");
        let (mut args, lease) = admit_json(&pool, "record_analysis_run", ArgsDecoding::Moved, &json!({ "request": {
            "project_id": project, "expected_base": BASE, "run_record": submitted.run_record, "result_json": submitted.result_json,
            "input_manifest_canonical": submitted.input_manifest_canonical, "state_id": state_id, "run_labels": { "run_name": "Run" } } })).unwrap();
        let recorded = serialized_value(record_analysis_run_args(&mut args, store, &memory).unwrap());
        drop((args, lease));
        let recording_id = recorded["run"]["recording_id"].as_str().unwrap().to_string();
        // The six history commands, admitted within their budgets and decoded by their handlers.
        let (mut args, lease) = admit_command(&pool, "list_model_states", model_states::LIST_MODEL_STATES_MAX_ARGS_BYTES, &json!({ "projectId": project })).unwrap();
        assert_eq!(model_states::list_model_states_args(&mut args, store, &memory).unwrap().get().len(), 1, "{length}: list_model_states");
        drop((args, lease));
        let (mut args, lease) = admit_command(&pool, "read_model_state", model_states::READ_MODEL_STATE_MAX_ARGS_BYTES, &json!({ "projectId": project, "stateId": state_id })).unwrap();
        assert_eq!(model_states::read_model_state_args(&mut args, store, &memory).unwrap().get().state.state_id, state_id, "{length}: read_model_state");
        drop((args, lease));
        let (mut args, lease) = admit_command(&pool, "list_analysis_runs", LIST_ANALYSIS_RUNS_MAX_ARGS_BYTES, &json!({ "projectId": project })).unwrap();
        assert_eq!(list_analysis_runs_args(&mut args, store, &memory).unwrap().get().len(), 1, "{length}: list_analysis_runs");
        drop((args, lease));
        let (mut args, lease) = admit_command(&pool, "read_analysis_run", READ_ANALYSIS_RUN_MAX_ARGS_BYTES, &json!({ "projectId": project, "recordingId": recording_id })).unwrap();
        assert_eq!(read_analysis_run_args(&mut args, store, &memory).unwrap().get().run.recording_id, recording_id, "{length}: read_analysis_run");
        drop((args, lease));
        let (mut args, lease) = admit_command(&pool, "delete_analysis_run", DELETE_ANALYSIS_RUN_MAX_ARGS_BYTES, &json!({ "projectId": project, "recordingId": recording_id })).unwrap();
        assert_eq!(serialized_value(delete_analysis_run_args(&mut args, store, &memory).unwrap())["deleted"], json!(true), "{length}: delete_analysis_run");
        drop((args, lease));
        let (mut args, lease) = admit_command(&pool, "delete_model_state", model_states::DELETE_MODEL_STATE_MAX_ARGS_BYTES, &json!({ "projectId": project, "stateId": state_id })).unwrap();
        assert_eq!(serialized_value(model_states::delete_model_state_args(&mut args, store, &memory).unwrap())["deleted"], json!(true), "{length}: delete_model_state");
        drop((args, lease));
        assert_eq!((row_count(&path, "analysis_runs"), row_count(&path, "model_states")), (0, 0));
        drop(memory);
        assert_eq!(pool.pool_status().reserved_bytes, 0);
    }
    // A project id past the 256 KiB budget is refused by name at ingress, before any lease.
    let pool = registry(ScriptedHost::normal());
    let project = "p".repeat(model_states::HISTORY_COMMAND_MAX_ARGS_BYTES + 1);
    for (command, budget) in [
        ("list_model_states", model_states::LIST_MODEL_STATES_MAX_ARGS_BYTES),
        ("read_model_state", model_states::READ_MODEL_STATE_MAX_ARGS_BYTES),
        ("delete_model_state", model_states::DELETE_MODEL_STATE_MAX_ARGS_BYTES),
        ("list_analysis_runs", LIST_ANALYSIS_RUNS_MAX_ARGS_BYTES),
        ("read_analysis_run", READ_ANALYSIS_RUN_MAX_ARGS_BYTES),
        ("delete_analysis_run", DELETE_ANALYSIS_RUN_MAX_ARGS_BYTES),
    ] {
        assert_eq!(budget, 256 * 1024, "{command}");
        match admit_command(&pool, command, budget, &json!({ "projectId": project, "stateId": "state:x", "recordingId": "runrec:x" })) {
            Err(SolveJobError::InvalidRequest { message }) => {
                assert!(message.starts_with(&format!("INGRESS-ARGS-TOO-LARGE: `{command}` takes at most 262144 bytes of arguments")), "{message}")
            }
            other => panic!("{command}: {:?}", other.map(|_| ())),
        }
    }
    assert_eq!(pool.pool_status().reserved_bytes, 0);
}
