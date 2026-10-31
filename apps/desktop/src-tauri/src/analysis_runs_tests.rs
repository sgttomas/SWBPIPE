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
/// The same pair with invented model rule-pack references, whose transient record carries
/// rule-pack reference diagnostics after the producer's, and its witness.
const TRANSIENT_INPUT_RULE_PACKS: &str = include_str!("../../../../fixtures/persistence/transient_run_input_rule_packs.json");
const WITNESS_RULE_PACKS: &str = include_str!("../../../../fixtures/persistence/recorded_run_witness_rule_packs.json");
const WITNESS_RULE_PACKS_PATH_ENV: &str = "OPENPIPESTRESS_WRITE_RECORDED_RUN_WITNESS_RULE_PACKS";
/// A run of a model with rule-pack references as S2 (#1596) recorded it, before
/// rule-pack resolution (written by `recordedRunParity.test.ts`).
const S2_UNRESOLVED_RUN: &str = include_str!("../../../../fixtures/persistence/s2_unresolved_rule_pack_run.json");

/// An invented 0.8.0 document of `PROJECT` (`name` makes distinct models).
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
            "load_basis_refs": [], "result_refs": result_refs, "diagnostics": [],
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
    assert_eq!(listed, vec![ModelStateSummary { recorded_run_count: 0, ..state.clone() }]);
    drop(connection);
    let outcome = record(&path, request(&model("Invented project"), "Run 1"), T0).unwrap();
    // A re-run (as an interrupted, non-transactional migration would) changes nothing.
    Connection::open(&path).unwrap().execute_batch("PRAGMA user_version = 12;").unwrap();
    let (connection, evidence) = crate::open_project_store(&path).unwrap();
    assert_eq!(evidence.migrations_applied_on_open, vec!["store-v13-analysis-runs-tables"]);
    migrate_v13_analysis_runs(&connection).unwrap();
    assert_eq!(list_runs(&connection, PROJECT).unwrap(), vec![outcome.run.clone()]);
    assert_eq!(model_states::list_states(&connection, PROJECT).unwrap()[0].recorded_run_count, 1);
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
    assert_eq!(outcome.state.recorded_run_count, 1);
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
    assert_eq!(outcome.state.recorded_run_count, 1);
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
fn a_state_with_a_recorded_run_is_not_deleted_and_its_blockers_are_counted() {
    let path = store_with_project("delete");
    let solved = model("Invented project");
    let state = save_state(&path, &solved, "Basis", "000000000000000000000000000000e1", 1_000);
    let first = record(&path, request(&solved, "Run 1"), T0).unwrap();
    let second = record(&path, request(&solved, "Run 2"), T0).unwrap();
    let error = model_states::delete_state_in_store(&path, PROJECT, &state.state_id).unwrap_err();
    // Runs are counted from their columns (W-12): the run list names each verified run.
    assert_eq!(error, format!(
        "MODEL-STATE-DELETE-BLOCKED: state \"Basis\" ({}) is the state of 2 recorded runs; delete them first. Nothing was deleted.",
        state.state_id
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
    let mut walked: Vec<String> = crate::export_disposition_screen::library_ref_sites(document).into_iter().map(|(path, _)| path).collect();
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
        ("admit_analysis_run_comparison", ADMIT_ANALYSIS_RUN_COMPARISON_MAX_ARGS_BYTES),
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
/// Records `input_text` (a TypeScript-built transient record and its manifest) through this
/// module's Record Run path and returns the witness text, with `description`.
fn recorded_run_witness_text(input_text: &str, description: &str) -> String {
    let input: Value = serde_json::from_str(input_text).unwrap();
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
        "description": description,
        "state_id": outcome.state.state_id,
        "state_model_hash": outcome.state.model_hash,
        "created_at": outcome.run.created_at,
        "recording_id": outcome.run.recording_id,
        "run_record_hash": outcome.run.run_record_hash,
        "recorded_record": read.record
    });
    format!("{}\n", serde_json::to_string_pretty(&witness).unwrap())
}

/// Writes `text` to the path in `env` when set (the maintained writer); otherwise compares it
/// with the committed witness.
fn compare_or_write_witness(text: &str, committed: &str, env: &str) {
    if let Ok(target) = std::env::var(env) {
        std::fs::write(target, text).unwrap();
        return;
    }
    assert_eq!(text, committed, "the committed witness is stale; regenerate it with {env}");
}

/// Owner ruling, 2026-10-26, "R-c1 A, 0.8.0 a2" (DEL-02-01 Design `model-document-0.8.0.md`
/// §3.2, §3.3): a store a 0.7.0 build wrote holds a 0.7.0 project, a 0.7.0 state and an
/// ordinary recorded run of that state with intact hashes and bindings. The project and
/// the state are refused by name, with the recorded-runs note; the runs are listed for the
/// project id and the run reads exactly as before (no model-document-version gate). Once
/// the project is re-authored to 0.8.0 under the same id it opens, still lists the run,
/// and the run still reads; a tampered stored result is refused as before.
#[test]
fn a_retired_project_and_state_are_refused_and_their_recorded_run_reads_as_before() {
    let path = scratch_store("a2-retired");
    let mut old = model("Invented project");
    old["schema_version"] = json!("0.7.0");
    {
        let (mut connection, _) = crate::open_project_store(&path).unwrap();
        write_project(&mut connection, PROJECT, &old, BASE);
    }
    // The current save path refuses a 0.7.0 snapshot by name; the state below is the one
    // a 0.7.0 build saved, written through the same insert after its own checks.
    let old_canonical = canonical_json(&old);
    let refused = model_states::checked_new_state(PROJECT, "design_snapshot", old_canonical.clone(), &model_hash(&old), &UnmeteredReadMemory)
        .map(drop)
        .unwrap_err();
    assert!(refused.contains("status=retired_schema") && refused.contains("source_schema_version=0.7.0"), "{refused}");
    assert!(refused.ends_with(crate::model_document_version::RETIRED_PROJECT_RUNS_NOTE), "{refused}");
    let hex = sha256_hex(&old_canonical);
    let snapshot = model_states::CheckedSnapshot { model: old.clone(), model_canonical: old_canonical, model_hash: format!("sha256:{hex}"), model_hash_hex: hex };
    let state = {
        let mut connection = stored_read::open_store(&path).unwrap();
        let transaction = stored_read::write_transaction(&mut connection).unwrap();
        let allocate = || Ok(("000000000000000000000000000000a2".to_string(), 1_000));
        let between_rows = |_: &Transaction<'_>| Ok(());
        let state = insert_new_state(
            &transaction, PROJECT, "design_snapshot", None, &snapshot, &labels_from_value(json!({ "state_name": "Basis 0.7.0" })).unwrap(),
            &StateWriteHooks { allocate: &allocate, between_rows: &between_rows }, &UnmeteredReadMemory,
        )
        .unwrap()
        .into_unreserved()
        .unwrap();
        transaction.commit().unwrap();
        state
    };
    // The run is recorded against that state: the record path binds hashes, not versions.
    let submitted = request(&old, "Run at 0.7.0");
    let result_bytes = submitted.result_json.clone();
    let mut submitted = submitted;
    submitted.state_id = Some(state.state_id.clone());
    let recorded = record(&path, submitted, T0).unwrap();
    assert_eq!(recorded.state_outcome, "existing");
    let recording_id = recorded.run.recording_id.clone();

    // The project and the state are refused by name, with the note; nothing is rewritten.
    let note = crate::model_document_version::RETIRED_PROJECT_RUNS_NOTE;
    let error = crate::open_project_in_store(path.clone(), Some(PROJECT)).map(drop).unwrap_err();
    assert!(error.starts_with("Model document refused for editing: status=retired_schema; source_schema_version=0.7.0; target_schema_version=0.8.0; MODEL_SCHEMA_VERSION_RETIRED: model document 0.7.0 is retired; re-author it as model document 0.8.0"), "{error}");
    assert!(error.ends_with(&format!(" {note}")), "{error}");
    for purpose in [model_states::StateReadPurpose::Open, model_states::StateReadPurpose::Compare] {
        let error = model_states::read_state_for_in_store_with(&path, PROJECT, &state.state_id, purpose, &UnmeteredReadMemory).map(drop).unwrap_err();
        assert!(error.starts_with("Model document refused for editing: status=retired_schema; source_schema_version=0.7.0;"), "{purpose:?}: {error}");
        assert!(error.ends_with(&format!(" {note}")), "{purpose:?}: {error}");
    }
    let (connection, _) = crate::open_project_store(&path).unwrap();
    assert_eq!(crate::load_project(&connection, Some(PROJECT)).unwrap().unwrap().model, old, "the stored project is never rewritten");
    // Its run is listed for the project id and reads exactly as before.
    assert_eq!(list_runs(&connection, PROJECT).unwrap(), vec![recorded.run.clone()]);
    drop(connection);
    let read = read_run_in_store(&path, PROJECT, &recording_id).unwrap();
    assert_eq!(read.result_json, result_bytes);
    assert_eq!(read.state.state_id, state.state_id);

    // Re-authored under the same project id (the version token alone), the project opens,
    // the run is still listed for it and still reads.
    let mut current = old.clone();
    current["schema_version"] = json!(open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION);
    {
        let (mut connection, _) = crate::open_project_store(&path).unwrap();
        write_project(&mut connection, PROJECT, &current, BASE);
    }
    let opened = crate::open_project_in_store(path.clone(), Some(PROJECT)).unwrap().unwrap();
    assert_eq!(opened.model, current);
    let (connection, _) = crate::open_project_store(&path).unwrap();
    assert_eq!(list_runs(&connection, PROJECT).unwrap(), vec![recorded.run.clone()]);
    drop(connection);
    let read_again = read_run_in_store(&path, PROJECT, &recording_id).unwrap();
    assert_eq!((read_again.result_json, read_again.record), (read.result_json, read.record));
    // The state stays refused: it holds the 0.7.0 snapshot.
    assert!(model_states::read_state_for_in_store_with(&path, PROJECT, &state.state_id, model_states::StateReadPurpose::Compare, &UnmeteredReadMemory).is_err());
    // A tampered stored result is refused as before.
    tamper(&path, &format!("UPDATE analysis_runs SET result_json = '{}';", quoted(&result_text(9.0))));
    let error = read_run_in_store(&path, PROJECT, &recording_id).unwrap_err();
    assert!(error.starts_with(&format!("ANALYSIS-RUN-INTEGRITY: recorded run `{recording_id}`: the stored result differs")), "{error}");
}

#[test]
fn the_recorded_run_witness_is_what_the_record_path_emits_for_a_typescript_built_record() {
    let text = recorded_run_witness_text(TRANSIENT_INPUT, "An analysis_run.v0.3 recorded revision emitted by the desktop store's Record Run path (apps/desktop/src-tauri/src/analysis_runs_tests.rs, test the_recorded_run_witness_is_what_the_record_path_emits_for_a_typescript_built_record) from fixtures/persistence/transient_run_input.json (a TypeScript-built transient record of fixtures/results/preview_physics_connected_sparse.json). Regenerate with OPENPIPESTRESS_WRITE_RECORDED_RUN_WITNESS=<path> on that test. apps/desktop/src/services/recordedRunParity.test.ts rebuilds it with the TypeScript builder; tests/test_recorded_run_record.py validates it against schemas/analysis_run.v0.3.schema.json and Python's validator.");
    compare_or_write_witness(&text, WITNESS, WITNESS_PATH_ENV);
}

/// The recorded revision keeps the transient record's rule-pack reference diagnostics
/// unchanged (they are under its record hash) and takes `rule_pack_refs` from the manifest.
#[test]
fn the_rule_pack_witness_keeps_the_transient_diagnostics_under_the_record_hash() {
    let text = recorded_run_witness_text(TRANSIENT_INPUT_RULE_PACKS, "An analysis_run.v0.3 recorded revision emitted by the desktop store's Record Run path (apps/desktop/src-tauri/src/analysis_runs_tests.rs, test the_rule_pack_witness_keeps_the_transient_diagnostics_under_the_record_hash) from fixtures/persistence/transient_run_input_rule_packs.json (a TypeScript-built transient record of fixtures/results/preview_physics_connected_sparse.json whose model carries invented rule_pack_refs; the excluded references are rule-pack reference diagnostics). Regenerate with OPENPIPESTRESS_WRITE_RECORDED_RUN_WITNESS_RULE_PACKS=<path> on that test. apps/desktop/src/services/recordedRunParity.test.ts rebuilds it with the TypeScript builder; tests/test_recorded_run_record.py validates it against schemas/analysis_run.v0.3.schema.json and Python's validator.");
    let input: Value = serde_json::from_str(TRANSIENT_INPUT_RULE_PACKS).unwrap();
    let witness: Value = serde_json::from_str(&text).unwrap();
    let recorded = &witness["recorded_record"]["analysis_run"];
    assert_eq!(recorded["diagnostics"], input["run_record"]["analysis_run"]["diagnostics"]);
    assert_eq!(recorded["rule_pack_refs"], json!([{ "object_type": "RulePack", "ref": "invented-pack%40held@1.0.0" }]));
    compare_or_write_witness(&text, WITNESS_RULE_PACKS, WITNESS_RULE_PACKS_PATH_ENV);
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

/// The 2 MiB pool plus SQLite's working set for one keyed column of `LARGE`
/// bytes (W-12, repair r7): SQLite materializes such a column, so the 2 MiB
/// pool alone refuses the command before SQLite reads it
/// (`sqlite_allocations_for_an_oversized_id_are_admitted_before_sqlite_loads_it`).
/// Under this pool the process-side peak is still checked against the column.
fn working_set_pool() -> crate::solve_jobs::SolveJobRegistry {
    pool_of(SMALL_POOL + stored_read::SQLITE_WORKING_SET_FACTOR * (LARGE as u64 + (64 << 10)))
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
    // The state's runs are counted from their columns, not named (W-12): no name is loaded.
    assert!(error.starts_with("MODEL-STATE-DELETE-BLOCKED: ") && error.len() < 1024 && error.contains("the state of 1 recorded run"), "{error}");
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
    // The list shows the row as unreadable, by its refusal (W-12), the same refusal.
    let (outcome, measured) = crate::ingress::counting::measure(|| {
        list_runs_in_store_with(&path, PROJECT, &memory).map(|listed| unreadable_refusals_of_runs(listed.get()))
    });
    let refusals = outcome.unwrap();
    assert_eq!(refusals.len(), 1);
    refused("run labels", &refusals[0], measured, memory.peak_reserved(), "ANALYSIS-RUN-INTEGRITY");
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
    // A blocked delete naming a child state with a 3 MiB name (runs are counted, W-12).
    let child_model_canonical = canonical_json(&model("Child project"));
    let child = SaveModelStateRequest {
        project_id: PROJECT.to_string(),
        expected_base: BASE.to_string(),
        model_hash: format!("sha256:{}", sha256_hex(&child_model_canonical)),
        model_canonical: child_model_canonical,
        state_kind: "design_snapshot".to_string(),
        parent_state_id: Some(state_id.clone()),
        labels: labels_from_value(json!({ "state_name": "Child" })).unwrap(),
    };
    let allocate = || Ok(("00000000000000000000000000002399".to_string(), 3_000));
    let between_rows = |_: &Transaction<'_>| Ok(());
    let child = save_state_in_store(&path, child, &StateWriteHooks { allocate: &allocate, between_rows: &between_rows }).unwrap().state;
    Connection::open(&path).unwrap().execute("UPDATE model_state_labels SET state_name = ?1 WHERE state_id = ?2", params!["n".repeat(LARGE), child.state_id]).unwrap();
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
    // Runs are counted from their columns, never named from them (W-12).
    assert!(error.contains("the state of 12 recorded runs") && error.matches("runrec:").count() == 0, "{error}");
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

/// The refusals of a run list's unreadable entries (none of their fields is trusted).
fn unreadable_refusals_of_runs(entries: &[ListedRun]) -> Vec<String> {
    entries
        .iter()
        .filter_map(|entry| match entry {
            ListedRun::Unreadable { unreadable } => Some(unreadable.refusal.clone()),
            ListedRun::Readable(_) => None,
        })
        .collect()
}

fn unreadable_refusals_of_states(entries: &[model_states::ListedState]) -> Vec<String> {
    entries
        .iter()
        .filter_map(|entry| match entry {
            model_states::ListedState::Unreadable { unreadable } => Some(unreadable.refusal.clone()),
            model_states::ListedState::Readable(_) => None,
        })
        .collect()
}

/// A list holding one row whose identity or time column holds `LARGE` bytes:
/// the row is listed as unreadable (W-12), by the head of its id and its
/// refusal, before it is loaded and within the reservations, under the 2 MiB
/// pool. `list` gives the list's unreadable refusals and its readable count.
fn assert_oversized_listed(what: &str, column: &str, list: impl FnOnce(&PoolReadMemory<'_>) -> Result<(Vec<String>, usize), String>) {
    let small = working_set_pool();
    let memory = PoolReadMemory::unseeded(&small);
    let (outcome, measured) = crate::ingress::counting::measure(|| list(&memory));
    let (refusals, readable) = outcome.unwrap();
    assert_eq!((refusals.len(), readable), (1, 0), "{what}");
    assert!(refusals[0].contains(&format!("its {column} is longer than")), "{what}: {}", stored_read::shown(&refusals[0]));
    assert_small(what, &refusals[0]);
    assert!(measured.peak_priced < (LARGE as u64) / 4, "{what}: listed before loading (peak {})", measured.peak_priced);
    assert!(measured.peak_priced <= memory.peak_reserved().max(1 << 16), "{what}: peak {} within {}", measured.peak_priced, memory.peak_reserved());
    assert_eq!(small.pool_status().reserved_bytes, 0);
}

/// A row whose identity or time column holds `LARGE` bytes is refused before
/// it is loaded, by the head of its id, within the reservations, under the
/// 2 MiB pool.
fn assert_oversized_refused<T: std::fmt::Debug>(what: &str, column: &str, read: impl FnOnce(&PoolReadMemory<'_>) -> Result<T, String>) {
    let small = working_set_pool();
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
        assert_oversized_listed(&format!("the state list ({column})"), column, |memory| {
            model_states::list_states_in_store_with(&path, PROJECT, memory).map(|listed| {
                let entries = listed.get();
                (unreadable_refusals_of_states(entries), entries.len() - unreadable_refusals_of_states(entries).len())
            })
        });
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
        assert_oversized_listed(&format!("the run list ({column})"), column, |memory| {
            list_runs_in_store_with(&path, PROJECT, memory).map(|listed| {
                let entries = listed.get();
                (unreadable_refusals_of_runs(entries), entries.len() - unreadable_refusals_of_runs(entries).len())
            })
        });
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
    // The state itself and its six verified children named; the runs counted (W-12).
    assert_eq!(error.matches("\" (").count(), 7, "{error}");
    assert!(error.contains("and the state of 6 recorded runs"), "{error}");
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
            "load_basis_refs": [], "result_refs": result_refs, "diagnostics": [],
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

// ---------------------------------------------------------------------------
// Rule-pack reference diagnostics (FR-CMP-002; DEL-14-01 Design §3): the
// record's diagnostics are the result's, then one per model reference the
// manifest leaves out of active_rule_packs, each exactly its code's and
// identity's, at Record Run and at read. All rule-pack data are invented.
// ---------------------------------------------------------------------------

#[test]
fn rule_pack_identities_percent_encode_each_component() {
    // The same vectors as `modelRulePackResolution.test.tsx` and `test_recorded_run_record.py`.
    assert_eq!(rule_pack_identity("rule-pack:b", "2"), "rule-pack:b@2");
    assert_eq!(rule_pack_identity("invented-pack@branch", "1.0.0"), "invented-pack%40branch@1.0.0");
    assert_eq!(rule_pack_identity("invented-pack", "branch@1.0.0"), "invented-pack@branch%401.0.0");
    assert_eq!(rule_pack_identity("pack%40x@y", "1%@2"), "pack%2540x%40y@1%25%402");
    let manifest = json!({ "active_rule_packs": [{ "rule_pack_id": "invented-pack@branch", "version": "1.0.0", "checksum_sha256": "a", "source_notice": "n" }] });
    assert_eq!(rule_pack_refs(&manifest).unwrap(), vec![json!({ "object_type": "RulePack", "ref": "invented-pack%40branch@1.0.0" })]);
}

/// The model's invented references: the first resolved (in the manifest's
/// active_rule_packs), the two whose flattened identities would collide excluded.
fn rule_pack_model() -> Value {
    let mut solved = model("Invented project");
    let reference = |id: &str, version: &str| json!({ "rule_pack_id": id, "name": "Invented", "rule_pack_version": version,
        "checksum": { "algorithm": "sha256", "canonicalization": "JCS", "payload_scope": "rule_pack_payload", "payload_excludes": ["checksums"], "value": "a".repeat(64) },
        "redistribution_status": "invented_non_engineering_example" });
    solved["rule_pack_refs"] = json!([reference("rule-pack:invented", "1.0.0"), reference("invented-pack@branch", "1.0.0"), reference("invented-pack", "branch@1.0.0")]);
    solved
}

fn exclusion_diagnostics() -> Value {
    json!([
        { "source_annotation": rule_pack_ref_diagnostic(RULE_PACK_REF_UNRESOLVED, &rule_pack_identity("invented-pack@branch", "1.0.0")) },
        { "source_annotation": rule_pack_ref_diagnostic(RULE_PACK_REF_UNRESOLVED, &rule_pack_identity("invented-pack", "branch@1.0.0")) }
    ])
}

fn rule_pack_request(solved: &Value, edit: Option<Mutation>) -> RecordRunRequest {
    let mut submitted = request(solved, "Run");
    submitted.run_record["analysis_run"]["diagnostics"] = exclusion_diagnostics();
    if let Some(edit) = edit {
        edit(&mut submitted.run_record);
    }
    reseal(&mut submitted.run_record);
    submitted
}

const RULE_PACK_DIAGNOSTIC_EDITS: [(&str, Mutation); 5] = [
    ("an exclusion dropped", |record| { record["analysis_run"]["diagnostics"].as_array_mut().unwrap().pop(); }),
    ("its message changed", |record| record["analysis_run"]["diagnostics"][0]["source_annotation"]["message"] = json!("Rule pack was applied")),
    ("its identity replaced", |record| record["analysis_run"]["diagnostics"][0]["source_annotation"]["affected_refs"] = json!(["invented-pack@other"])),
    ("an extra exclusion", |record| {
        let extra = json!({ "source_annotation": rule_pack_ref_diagnostic(RULE_PACK_REF_UNRESOLVED, "rule-pack:invented@1.0.0") });
        record["analysis_run"]["diagnostics"].as_array_mut().unwrap().push(extra);
    }),
    ("the exclusions reordered", |record| record["analysis_run"]["diagnostics"].as_array_mut().unwrap().swap(0, 1)),
];

#[test]
fn rule_pack_reference_diagnostics_are_recorded_and_read_unchanged_when_they_name_the_exclusions() {
    let path = store_with_project("rule-pack-diagnostics");
    let solved = rule_pack_model();
    save_state(&path, &solved, "Basis", "00000000000000000000000000002001", 1_000);
    let submitted = rule_pack_request(&solved, None);
    let outcome = record(&path, submitted, T0).unwrap();
    let read = read_run_in_store(&path, PROJECT, &outcome.run.recording_id).unwrap();
    assert_eq!(read.record["analysis_run"]["diagnostics"], exclusion_diagnostics());
    assert_eq!(read.record["analysis_run"]["rule_pack_refs"], json!([{ "object_type": "RulePack", "ref": "rule-pack:invented@1.0.0" }]));
    // The two would share `invented-pack@branch@1.0.0` flattened; encoded they are distinct.
    assert_eq!(read.record["analysis_run"]["diagnostics"][0]["source_annotation"]["affected_refs"], json!(["invented-pack%40branch@1.0.0"]));
    assert_eq!(read.record["analysis_run"]["diagnostics"][1]["source_annotation"]["affected_refs"], json!(["invented-pack@branch%401.0.0"]));
}

#[test]
fn rule_pack_reference_diagnostics_that_do_not_name_the_exclusions_are_refused_at_record_run_and_at_read() {
    for (index, (name, edit)) in RULE_PACK_DIAGNOSTIC_EDITS.into_iter().enumerate() {
        // At Record Run: nothing is written.
        let path = store_with_project(&format!("rule-pack-submit-{index}"));
        let solved = rule_pack_model();
        save_state(&path, &solved, "Basis", "00000000000000000000000000002001", 1_000);
        let error = record(&path, rule_pack_request(&solved, Some(edit)), T0).unwrap_err();
        assert!(error.starts_with("ANALYSIS-RUN-MANIFEST-MISMATCH: the record's rule-pack reference diagnostics are not exactly those"), "{name}: {error}");
        assert!(error.ends_with("nothing was recorded."), "{name}: {error}");
        assert_eq!(row_count(&path, "analysis_runs"), 0, "{name}");
        // At read: a coherently resealed stored record is refused.
        let outcome = record(&path, rule_pack_request(&solved, None), T0).unwrap();
        let mut stored = read_run_in_store(&path, PROJECT, &outcome.run.recording_id).unwrap().record;
        edit(&mut stored);
        reseal(&mut stored);
        let recording_id = forge(&path, &stored);
        let read = read_run_in_store(&path, PROJECT, &recording_id).unwrap_err();
        assert!(read.starts_with(&format!("ANALYSIS-RUN-INTEGRITY: recorded run `{recording_id}`")), "{name}, read: {read}");
        assert!(read.contains("rule-pack reference diagnostics differ from the references its stored input manifest leaves out"), "{name}, read: {read}");
    }
}

#[test]
fn the_records_diagnostics_must_begin_with_the_results() {
    let path = store_with_project("producer-diagnostics");
    let solved = model("Invented project");
    save_state(&path, &solved, "Basis", "00000000000000000000000000002002", 1_000);
    // A result carrying one diagnostic; a record must list it first.
    let diagnostic = json!({ "code": "INVENTED", "severity": "warning", "message": "Invented." });
    let with_result = |listed: bool| {
        let mut submitted = request(&solved, "Run");
        submitted.result_json = result_text(1.25).replace("\"diagnostics\": []", &format!("\"diagnostics\": [{diagnostic}]"));
        submitted.run_record = transient(&submitted.result_json, &submitted.input_manifest_canonical);
        if listed {
            submitted.run_record["analysis_run"]["diagnostics"] = json!([{ "source_annotation": diagnostic.clone() }]);
            reseal(&mut submitted.run_record);
        }
        submitted
    };
    let error = record(&path, with_result(false), T0).unwrap_err();
    assert!(error.starts_with("ANALYSIS-RUN-RESULT-MISMATCH: the record's diagnostics do not begin with the result's"), "{error}");
    assert_eq!(row_count(&path, "analysis_runs"), 0);
    let outcome = record(&path, with_result(true), T0).unwrap();
    let mut stored = read_run_in_store(&path, PROJECT, &outcome.run.recording_id).unwrap().record;
    stored["analysis_run"]["diagnostics"] = json!([]);
    reseal(&mut stored);
    let recording_id = forge(&path, &stored);
    let read = read_run_in_store(&path, PROJECT, &recording_id).unwrap_err();
    assert!(read.contains("the record's diagnostics do not begin with its stored result's"), "{read}");
}

#[test]
fn a_repeated_exclusion_identity_is_refused_at_record_run_and_at_read() {
    // A model naming one excluded (id, version) pair twice (the document validator refuses
    // repeated authored ids; a direct submission or a forged store need not): its two
    // identical exclusion diagnostics are refused, as the TypeScript and Python validators
    // refuse a repeated identity.
    let path = store_with_project("rule-pack-repeat");
    let mut solved = rule_pack_model();
    let repeated = solved["rule_pack_refs"][1].clone();
    solved["rule_pack_refs"].as_array_mut().unwrap().push(repeated);
    save_state(&path, &solved, "Basis", "00000000000000000000000000002003", 1_000);
    let repeat = |record: &mut Value| {
        let first = record["analysis_run"]["diagnostics"][0].clone();
        record["analysis_run"]["diagnostics"].as_array_mut().unwrap().push(first);
    };
    let mut submitted = request(&solved, "Run");
    submitted.run_record["analysis_run"]["diagnostics"] = exclusion_diagnostics();
    repeat(&mut submitted.run_record);
    reseal(&mut submitted.run_record);
    let error = record(&path, submitted, T0).unwrap_err();
    assert!(error.starts_with("ANALYSIS-RUN-MANIFEST-MISMATCH: the record's rule-pack reference diagnostics are not exactly those"), "{error}");
    assert_eq!(row_count(&path, "analysis_runs"), 0);
    // At read: a run recorded for a model without the repeat, its stored record and
    // manifest model forged coherently to repeat the pair, is refused.
    let path = store_with_project("rule-pack-repeat-read");
    let plain = rule_pack_model();
    save_state(&path, &plain, "Basis", "00000000000000000000000000002004", 1_000);
    let outcome = record(&path, rule_pack_request(&plain, None), T0).unwrap();
    let mut stored = read_run_in_store(&path, PROJECT, &outcome.run.recording_id).unwrap().record;
    repeat(&mut stored);
    reseal(&mut stored);
    let recording_id = forge(&path, &stored);
    let read = read_run_in_store(&path, PROJECT, &recording_id).unwrap_err();
    assert!(read.starts_with(&format!("ANALYSIS-RUN-INTEGRITY: recorded run `{recording_id}`")), "{read}");
    assert!(read.contains("rule-pack reference diagnostics differ from the references its stored input manifest leaves out"), "{read}");
}

#[test]
fn diagnostics_difference_refuses_a_repeat_even_when_the_model_repeats_the_pair() {
    // The pure check, with the manifest's model naming the pair twice: the exact two
    // diagnostics are refused (no identity twice), as are one and three.
    let reference = json!({ "rule_pack_id": "invented-pack@branch", "rule_pack_version": "1.0.0" });
    let manifest = json!({ "active_rule_packs": [], "model_basis": { "model_payload": { "rule_pack_refs": [reference.clone(), reference] } } });
    let result = json!({ "diagnostics": [] });
    let one = json!({ "source_annotation": rule_pack_ref_diagnostic(RULE_PACK_REF_UNRESOLVED, &rule_pack_identity("invented-pack@branch", "1.0.0")) });
    for count in [1, 2, 3] {
        let record = json!({ "analysis_run": { "diagnostics": vec![one.clone(); count] } });
        assert_eq!(diagnostics_difference(&record, &result, &manifest).unwrap(), Some(DiagnosticsDifference::RulePacks), "{count}");
    }
    // And a record without a diagnostics list is refused.
    assert_eq!(diagnostics_difference(&json!({ "analysis_run": {} }), &result, &json!({ "active_rule_packs": [] })).unwrap(), Some(DiagnosticsDifference::Producer));
}

#[test]
fn a_model_without_references_excludes_nothing_whatever_the_manifest_holds() {
    // As the TypeScript `manifestExcludedRulePacks` and Python's `manifest_excluded_rule_packs`.
    assert_eq!(manifest_excluded_rule_packs(&json!({ "model_basis": { "model_payload": {} } })).unwrap(), Vec::<(String, String)>::new());
    assert_eq!(manifest_excluded_rule_packs(&json!({ "model_basis": { "model_payload": { "rule_pack_refs": [] } } })).unwrap(), Vec::<(String, String)>::new());
    let referenced = json!({ "model_basis": { "model_payload": { "rule_pack_refs": [{ "rule_pack_id": "x", "rule_pack_version": "1" }] } } });
    assert!(manifest_excluded_rule_packs(&referenced).unwrap_err().starts_with("ANALYSIS-RUN-MANIFEST-INVALID: the input manifest has no active_rule_packs list"));
    let unlisted = json!({ "active_rule_packs": [], "model_basis": { "model_payload": { "rule_pack_refs": null } } });
    assert!(manifest_excluded_rule_packs(&unlisted).unwrap_err().contains("is not a list"));
    let unversioned = json!({ "active_rule_packs": [], "model_basis": { "model_payload": { "rule_pack_refs": [{ "rule_pack_id": "x" }] } } });
    assert!(manifest_excluded_rule_packs(&unversioned).unwrap_err().contains("has no id or version"));
}

// ---------------------------------------------------------------------------
// W-12 (DEL-14-01 Design §2, §3): a run recorded before rule-pack resolution is
// refused by name; the history lists show each unreadable row by its id and
// refusal only, and the other rows as usual; an unreadable run, or an
// unreadable state nothing depends on, may be deleted.
// ---------------------------------------------------------------------------

/// Writes an S2-era store row for the fixture's recorded revision, as S2's Record
/// Run wrote it (its state saved first), and returns its recording id.
fn insert_s2_run(path: &Path, fixture: &Value) -> String {
    let manifest_text = fixture["input_manifest_canonical"].as_str().unwrap();
    let manifest: Value = serde_json::from_str(manifest_text).unwrap();
    let payload = &manifest["model_basis"]["model_payload"];
    let project_id = payload["project"]["id"].as_str().unwrap();
    let state_id = fixture["state_id"].as_str().unwrap();
    let model_canonical = canonical_json(payload);
    let request = SaveModelStateRequest {
        project_id: project_id.to_string(),
        expected_base: BASE.to_string(),
        model_hash: format!("sha256:{}", sha256_hex(&model_canonical)),
        model_canonical,
        state_kind: "post_solve_snapshot".to_string(),
        parent_state_id: None,
        labels: labels_from_value(json!({ "state_name": "S2 snapshot" })).unwrap(),
    };
    let token = state_id.trim_start_matches("state:").to_string();
    let allocate = move || Ok((token.clone(), 500));
    let between_rows = |_: &Transaction<'_>| Ok(());
    save_state_in_store(path, request, &StateWriteHooks { allocate: &allocate, between_rows: &between_rows }).unwrap();
    let record = &fixture["recorded_record"];
    let hash = run_record_hash(record).unwrap();
    let recording_id = recording_id_of(&hash);
    let result_hash = record["analysis_run"]["hashes"].as_array().unwrap().iter().find(|hash| hash["payload_scope"] == "received_result").unwrap()["value"].as_str().unwrap().to_string();
    let connection = Connection::open(path).unwrap();
    connection
        .execute(
            "INSERT INTO analysis_runs (project_id, recording_id, state_id, producer_run_id, created_at, run_record_json, run_record_hash, input_manifest_json, result_json, result_hash)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![project_id, recording_id, state_id, record["analysis_run"]["run_id"].as_str().unwrap(), fixture["created_at"].as_str().unwrap(),
                    serde_json::to_string(record).unwrap(), hash, manifest_text, SOURCE_RESULT, result_hash],
        )
        .unwrap();
    connection
        .execute(
            "INSERT INTO analysis_run_labels (project_id, recording_id, run_name, tags_json, notes_json, assumptions_json, updated_at) VALUES (?1, ?2, 'S2 run', '[]', '[]', '[]', ?3)",
            params![project_id, recording_id, fixture["created_at"].as_str().unwrap()],
        )
        .unwrap();
    recording_id
}

#[test]
fn a_run_recorded_before_rule_pack_resolution_is_refused_by_name_and_the_list_keeps_the_projects_other_runs() {
    let fixture: Value = serde_json::from_str(S2_UNRESOLVED_RUN).unwrap();
    let manifest_text = fixture["input_manifest_canonical"].as_str().unwrap().to_string();
    let manifest: Value = serde_json::from_str(&manifest_text).unwrap();
    assert_eq!(manifest["active_rule_packs"], json!([]));
    let payload = manifest["model_basis"]["model_payload"].clone();
    let project_id = payload["project"]["id"].as_str().unwrap().to_string();
    let path = scratch_store("s2-unresolved");
    let (mut connection, _) = crate::open_project_store(&path).unwrap();
    write_project(&mut connection, &project_id, &payload, BASE);
    drop(connection);
    // Record Run of S2's transient record: refused by name, nothing written.
    let now = || Ok(T0);
    let token = || Ok("000000000000000000000000000052b1".to_string());
    let between_rows = |_: &Transaction<'_>, _: &str| Ok(());
    let hooks = RunWriteHooks { now: &now, state_token: &token, between_rows: &between_rows };
    let submitted = RecordRunRequest {
        project_id: project_id.clone(),
        expected_base: BASE.to_string(),
        run_record: fixture["run_record"].clone(),
        result_json: SOURCE_RESULT.to_string(),
        input_manifest_canonical: manifest_text,
        state_id: None,
        new_state: Some(NewStateRequest { labels: labels_from_value(json!({ "state_name": "After solve" })).unwrap(), parent_state_id: None }),
        run_labels: run_labels("S2 run"),
    };
    let error = record_run_in_store(&path, submitted, &hooks).unwrap_err();
    assert_eq!(error, "ANALYSIS-RUN-RULE-PACKS-NOT-RESOLVED: recorded before rule-pack resolution; re-record this run; nothing was recorded.");
    assert_eq!(row_count(&path, "analysis_runs"), 0);
    // A stored S2-era row: refused by name at read.
    let s2_id = insert_s2_run(&path, &fixture);
    let read = read_run_in_store(&path, &project_id, &s2_id).unwrap_err();
    assert_eq!(read, format!("ANALYSIS-RUN-RULE-PACKS-NOT-RESOLVED: recorded run `{s2_id}`: recorded before rule-pack resolution; re-record this run."));
    // Another run of the project (the no-reference pair) records and reads.
    let input: Value = serde_json::from_str(TRANSIENT_INPUT).unwrap();
    let other_text = input["input_manifest_canonical"].as_str().unwrap().to_string();
    let other = record_run_in_store(
        &path,
        RecordRunRequest {
            project_id: project_id.clone(),
            expected_base: BASE.to_string(),
            run_record: input["run_record"].clone(),
            result_json: SOURCE_RESULT.to_string(),
            input_manifest_canonical: other_text,
            state_id: None,
            new_state: Some(NewStateRequest { labels: labels_from_value(json!({ "state_name": "Plain" })).unwrap(), parent_state_id: None }),
            run_labels: run_labels("Plain run"),
        },
        &hooks,
    )
    .unwrap();
    read_run_in_store(&path, &project_id, &other.run.recording_id).unwrap();
    // The list: both runs, each its verified summary (the list reads no manifest;
    // the S2-era run is refused, by name, when it is read).
    let entries = list_run_entries(&Connection::open(&path).unwrap(), &project_id).unwrap();
    let readable: Vec<&str> = entries.iter().filter_map(|entry| match entry { ListedRun::Readable(run) => Some(run.recording_id.as_str()), _ => None }).collect();
    assert_eq!(readable.len(), 2);
    assert!(readable.contains(&s2_id.as_str()) && readable.contains(&other.run.recording_id.as_str()));
}

/// The unreadable entries of a run list (the row's id, refusal and handle only).
fn unreadable_runs(path: &Path, project_id: &str) -> Vec<stored_read::UnreadableRow> {
    list_run_entries(&Connection::open(path).unwrap(), project_id)
        .unwrap()
        .into_iter()
        .filter_map(|entry| match entry {
            ListedRun::Unreadable { unreadable } => Some(unreadable),
            ListedRun::Readable(_) => None,
        })
        .collect()
}

fn unreadable_states(path: &Path) -> Vec<stored_read::UnreadableRow> {
    model_states::list_state_entries(&Connection::open(path).unwrap(), PROJECT)
        .unwrap()
        .into_iter()
        .filter_map(|entry| match entry {
            model_states::ListedState::Unreadable { unreadable } => Some(unreadable),
            model_states::ListedState::Readable(_) => None,
        })
        .collect()
}

fn readable_states(path: &Path) -> Vec<ModelStateSummary> {
    model_states::list_states(&Connection::open(path).unwrap(), PROJECT).unwrap_or_else(|_| {
        model_states::list_state_entries(&Connection::open(path).unwrap(), PROJECT)
            .unwrap()
            .into_iter()
            .filter_map(|entry| match entry {
                model_states::ListedState::Readable(summary) => Some(summary),
                model_states::ListedState::Unreadable { .. } => None,
            })
            .collect()
    })
}

#[test]
fn a_tampered_run_is_listed_as_unreadable_by_id_refusal_and_handle_only_and_may_be_deleted_by_its_handle() {
    let (path, _) = recorded_fixture("listed-unreadable-run");
    let tampered = id_of(&path);
    let solved = model("Invented project");
    let kept = record(&path, request(&solved, "Kept"), T0 + 5).unwrap().run.recording_id;
    tamper(&path, &format!("UPDATE analysis_runs SET result_hash = 'sha256:tampered' WHERE recording_id = '{tampered}';"));
    let entries = list_run_entries(&Connection::open(&path).unwrap(), PROJECT).unwrap();
    assert_eq!(entries.len(), 2);
    let text = serde_json::to_value(&entries).unwrap();
    let unreadable: Vec<&Value> = text.as_array().unwrap().iter().filter(|entry| entry.get("unreadable").is_some()).collect();
    assert_eq!(unreadable.len(), 1);
    // Only the id, the named refusal, the delete handle and the reference count: no unverified field.
    assert_eq!(unreadable[0].as_object().unwrap().len(), 1);
    assert_eq!(unreadable[0]["unreadable"].as_object().unwrap().keys().collect::<Vec<_>>(), ["handle", "id", "references", "refusal"]);
    assert_eq!(unreadable[0]["unreadable"]["id"], json!(tampered));
    let refusal = unreadable[0]["unreadable"]["refusal"].as_str().unwrap();
    assert!(refusal.starts_with(&format!("ANALYSIS-RUN-INTEGRITY: recorded run `{tampered}`: the row's result hash differs")), "{refusal}");
    assert!(refusal.chars().count() <= stored_read::MAX_MESSAGE_CHARS + 1);
    assert!(text.as_array().unwrap().iter().any(|entry| entry["recording_id"] == json!(kept)));
    assert!(read_run_in_store(&path, PROJECT, &tampered).unwrap_err().starts_with("ANALYSIS-RUN-INTEGRITY"));
    read_run_in_store(&path, PROJECT, &kept).unwrap();
    // By its id it is not deleted (only a verified run is); by its handle it is, with its labels.
    assert!(delete_run_in_store(&path, PROJECT, &tampered).unwrap_err().starts_with("ANALYSIS-RUN-INTEGRITY"));
    let handle = unreadable[0]["unreadable"]["handle"].as_str().unwrap();
    let receipt = delete_unreadable_run_in_store(&path, PROJECT, handle).unwrap();
    assert!(receipt.deleted && receipt.message == format!("Deleted unreadable recorded run `{tampered}`; its record was not read."), "{}", receipt.message);
    assert_eq!(row_count(&path, "analysis_runs"), 1);
    assert_eq!(row_count(&path, "analysis_run_labels"), 1);
    assert_eq!(list_runs(&Connection::open(&path).unwrap(), PROJECT).unwrap().len(), 1);
    // The handle is stale now; another project's handle is not this one's.
    assert!(delete_unreadable_run_in_store(&path, PROJECT, handle).unwrap_err().starts_with("ANALYSIS-RUN-NOT-FOUND"));
    assert!(delete_unreadable_run_in_store(&path, PROJECT, "run-row:1:1:0000000000000000").unwrap_err().starts_with("ANALYSIS-RUN-NOT-FOUND"));
}

#[test]
fn rows_whose_shown_ids_collide_are_each_deleted_by_their_own_handle_and_a_stale_or_wrong_handle_is_refused() {
    // Codex: `x`×120 + "tail" and `x`×120 + "…" list identically; each handle deletes exactly its row.
    let (path, _) = recorded_fixture("colliding-ids");
    let solved = model("Invented project");
    record(&path, request(&solved, "Second"), T0 + 5).unwrap();
    let long = format!("{}tail", "x".repeat(120));
    let short = format!("{}…", "x".repeat(120));
    let connection = Connection::open(&path).unwrap();
    let ids: Vec<String> = connection.prepare("SELECT recording_id FROM analysis_runs ORDER BY rowid").unwrap().query_map([], |row| row.get(0)).unwrap().collect::<Result<_, _>>().unwrap();
    drop(connection);
    tamper(&path, "");
    let connection = Connection::open(&path).unwrap();
    connection.execute_batch("PRAGMA foreign_keys = OFF;").unwrap();
    for (from, to) in [(&ids[0], &long), (&ids[1], &short)] {
        connection.execute("UPDATE analysis_runs SET recording_id = ?1 WHERE recording_id = ?2", params![to, from]).unwrap();
        connection.execute("UPDATE analysis_run_labels SET recording_id = ?1 WHERE recording_id = ?2", params![to, from]).unwrap();
    }
    drop(connection);
    let entries = unreadable_runs(&path, PROJECT);
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].id, entries[1].id, "they list identically");
    assert_ne!(entries[0].handle, entries[1].handle);
    let remaining = |path: &Path| -> Vec<String> {
        Connection::open(path).unwrap().prepare("SELECT recording_id FROM analysis_runs").unwrap().query_map([], |row| row.get(0)).unwrap().collect::<Result<_, _>>().unwrap()
    };
    let long_entry = entries.iter().find(|entry| entry.refusal.contains("tail") || entry.handle.contains(&format!(":{}:", long.len()))).unwrap().clone();
    let short_entry = entries.iter().find(|entry| entry.handle != long_entry.handle).unwrap().clone();
    delete_unreadable_run_in_store(&path, PROJECT, &long_entry.handle).unwrap();
    assert_eq!(remaining(&path), vec![short.clone()], "the long row went, the short one stayed");
    // A wrong digest on the short row's rowid, and the long row's stale handle, are refused.
    let mut wrong = short_entry.handle.clone();
    wrong.replace_range(wrong.len() - 16.., "ffffffffffffffff");
    assert!(delete_unreadable_run_in_store(&path, PROJECT, &wrong).unwrap_err().starts_with("ANALYSIS-RUN-NOT-FOUND"));
    assert!(delete_unreadable_run_in_store(&path, PROJECT, &long_entry.handle).unwrap_err().starts_with("ANALYSIS-RUN-NOT-FOUND"));
    assert!(delete_unreadable_run_in_store(&path, "project:other", &short_entry.handle).unwrap_err().starts_with("ANALYSIS-RUN-NOT-FOUND"));
    delete_unreadable_run_in_store(&path, PROJECT, &short_entry.handle).unwrap();
    assert!(remaining(&path).is_empty());
}

#[test]
fn a_tampered_state_is_listed_as_unreadable_and_is_deleted_by_its_handle_only_when_nothing_names_it() {
    let path = store_with_project("listed-unreadable-state");
    let solved = model("Invented project");
    let basis = save_state(&path, &solved, "Basis", "00000000000000000000000000002701", 1_000);
    let kept = save_state(&path, &model("Kept project"), "Kept", "00000000000000000000000000002702", 2_000);
    let run = record(&path, request(&solved, "Run"), T0).unwrap().run.recording_id;
    tamper(&path, &format!("UPDATE model_states SET created_at = '2026-01-01T00:00:00.000Z' WHERE state_id = '{}';", basis.state_id));
    let entries = model_states::list_state_entries(&Connection::open(&path).unwrap(), PROJECT).unwrap();
    let text = serde_json::to_value(&entries).unwrap();
    let unreadable: Vec<&Value> = text.as_array().unwrap().iter().filter(|entry| entry.get("unreadable").is_some()).collect();
    assert_eq!(unreadable.len(), 1);
    assert_eq!(unreadable[0]["unreadable"].as_object().unwrap().keys().collect::<Vec<_>>(), ["handle", "id", "references", "refusal"]);
    assert_eq!(unreadable[0]["unreadable"]["id"], json!(basis.state_id));
    assert_eq!(unreadable[0]["unreadable"]["references"], json!(1), "its run names it");
    assert!(unreadable[0]["unreadable"]["refusal"].as_str().unwrap().starts_with("MODEL-STATE-INTEGRITY"));
    assert!(text.as_array().unwrap().iter().any(|entry| entry["state_id"] == json!(kept.state_id)));
    assert!(model_states::read_state_in_store(&path, PROJECT, &basis.state_id).unwrap_err().starts_with("MODEL-STATE-INTEGRITY"));
    let handle = unreadable[0]["unreadable"]["handle"].as_str().unwrap().to_string();
    // Blocked while its run is recorded against it; the readable run itself is deleted
    // (its state's readability does not matter), and then the state is.
    let blocked = model_states::delete_unreadable_state_in_store(&path, PROJECT, &handle).unwrap_err();
    assert_eq!(blocked, format!("MODEL-STATE-DELETE-BLOCKED: unreadable state `{}` is the state of 1 recorded run; delete it first. Nothing was deleted.", basis.state_id));
    // The run under the unreadable state is listed as readable, and deleted by its id.
    assert!(list_runs(&Connection::open(&path).unwrap(), PROJECT).unwrap().iter().any(|listed| listed.recording_id == run));
    delete_run_in_store(&path, PROJECT, &run).unwrap();
    assert_eq!(unreadable_states(&path)[0].references, 0);
    let receipt = model_states::delete_unreadable_state_in_store(&path, PROJECT, &handle).unwrap();
    assert!(receipt.deleted && receipt.message.starts_with("Deleted unreadable model state"), "{}", receipt.message);
    assert_eq!(model_states::list_states(&Connection::open(&path).unwrap(), PROJECT).unwrap().len(), 1);
}

#[test]
fn a_readable_state_presents_only_verified_children_and_counts_the_unreadable_rows_that_name_it() {
    // Codex: a child whose parent_state_id is forged is unreadable, and is not the
    // readable parent's child; it is counted, never named.
    let path = store_with_project("forged-parent");
    let parent = save_state(&path, &model("Invented project"), "Parent", "00000000000000000000000000002801", 1_000);
    let child = save_state(&path, &model("Child project"), "Child", "00000000000000000000000000002802", 2_000);
    tamper(&path, &format!("UPDATE model_states SET parent_state_id = '{}' WHERE state_id = '{}';", parent.state_id, child.state_id));
    let listed = readable_states(&path);
    let summary = listed.iter().find(|state| state.state_id == parent.state_id).unwrap();
    assert!(summary.child_state_ids.is_empty(), "{:?}", summary.child_state_ids);
    assert_eq!(summary.unreadable_references, 1);
    assert_eq!(unreadable_states(&path).iter().map(|entry| entry.id.clone()).collect::<Vec<_>>(), vec![child.state_id.clone()]);
    // The parent's delete names no unverified id and counts the unreadable row.
    let blocked = model_states::delete_state_in_store(&path, PROJECT, &parent.state_id).unwrap_err();
    assert_eq!(blocked, format!("MODEL-STATE-DELETE-BLOCKED: state \"Parent\" ({}) is the parent of 1 unreadable saved state row; delete it first. Nothing was deleted.", parent.state_id));
    assert!(!blocked.contains(&child.state_id));
}

#[test]
fn a_child_with_a_3_mib_id_does_not_make_its_parents_list_need_more_than_the_pool() {
    // Codex: a 3 MiB child id under a readable parent made the list need 9.98 MB.
    let path = store_with_project("huge-child-id");
    let parent = save_state(&path, &model("Invented project"), "Parent", "00000000000000000000000000002901", 1_000);
    let child = save_state(&path, &model("Child project"), "Child", "00000000000000000000000000002902", 2_000);
    let large = "c".repeat(3 * 1024 * 1024);
    tamper_with(&path, &format!("UPDATE model_states SET state_id = ?1, parent_state_id = '{}' WHERE state_id = '{}'", parent.state_id, child.state_id), &large);
    let small = working_set_pool();
    let memory = PoolReadMemory::unseeded(&small);
    let (outcome, measured) = crate::ingress::counting::measure(|| {
        model_states::list_states_in_store_with(&path, PROJECT, &memory).map(|listed| {
            let entries = listed.get();
            let parent_summary = entries.iter().find_map(|entry| match entry {
                model_states::ListedState::Readable(summary) if summary.state_id == parent.state_id => Some((summary.child_state_ids.clone(), summary.unreadable_references)),
                _ => None,
            });
            (parent_summary, unreadable_refusals_of_states(entries).len())
        })
    });
    let (parent_summary, unreadable) = outcome.unwrap();
    assert_eq!(parent_summary, Some((Vec::new(), 1)));
    assert_eq!(unreadable, 1);
    assert!(measured.peak_priced < (large.len() as u64) / 4, "the child id is never loaded (peak {})", measured.peak_priced);
    assert_eq!(small.pool_status().reserved_bytes, 0);
}

#[test]
fn a_row_with_a_value_of_the_wrong_type_or_not_utf_8_is_listed_as_unreadable_with_a_handle() {
    // Codex: a run name set to X'FF' (a Blob) hid both runs behind STORE-SQL-ERROR.
    let (path, _) = recorded_fixture("blob-label");
    let first = id_of(&path);
    let solved = model("Invented project");
    let second = record(&path, request(&solved, "Second"), T0 + 5).unwrap().run.recording_id;
    tamper(&path, &format!("UPDATE analysis_run_labels SET run_name = X'FF' WHERE recording_id = '{first}';"));
    let entries = unreadable_runs(&path, PROJECT);
    assert_eq!(entries.len(), 1);
    assert!(entries[0].refusal.starts_with("ANALYSIS-RUN-INTEGRITY") && entries[0].refusal.contains("a stored column cannot be read"), "{}", entries[0].refusal);
    assert!(entries[0].handle.starts_with("run-row:"));
    assert!(list_run_entries(&Connection::open(&path).unwrap(), PROJECT).unwrap().iter().any(|entry| matches!(entry, ListedRun::Readable(run) if run.recording_id == second)));
    assert!(read_run_in_store(&path, PROJECT, &first).unwrap_err().starts_with("ANALYSIS-RUN-INTEGRITY"));
    delete_unreadable_run_in_store(&path, PROJECT, &entries[0].handle).unwrap();
    assert!(unreadable_runs(&path, PROJECT).is_empty());
    // An id that is not UTF-8.
    let third = record(&path, request(&solved, "Third"), T0 + 9).unwrap().run.recording_id;
    tamper(&path, "");
    let connection = Connection::open(&path).unwrap();
    connection.execute_batch("PRAGMA foreign_keys = OFF;").unwrap();
    connection.execute("UPDATE analysis_runs SET recording_id = CAST(X'FFFE41' AS TEXT) WHERE recording_id = ?1", params![third]).unwrap();
    drop(connection);
    let entries = unreadable_runs(&path, PROJECT);
    assert_eq!(entries.len(), 1, "{entries:?}");
    assert!(entries[0].refusal.starts_with("ANALYSIS-RUN-INTEGRITY"), "{}", entries[0].refusal);
    delete_unreadable_run_in_store(&path, PROJECT, &entries[0].handle).unwrap();
    assert_eq!(list_runs(&Connection::open(&path).unwrap(), PROJECT).unwrap().len(), 1);
    // A saved state's label of the wrong type.
    let path = store_with_project("blob-state-label");
    let a = save_state(&path, &model("Invented project"), "A", "00000000000000000000000000003001", 1_000);
    let b = save_state(&path, &model("Other project"), "B", "00000000000000000000000000003002", 2_000);
    tamper(&path, &format!("UPDATE model_state_labels SET state_name = X'FF' WHERE state_id = '{}';", a.state_id));
    let unreadable = unreadable_states(&path);
    assert_eq!(unreadable.len(), 1);
    assert!(unreadable[0].handle.starts_with("state-row:"));
    assert!(readable_states(&path).iter().any(|state| state.state_id == b.state_id));
    model_states::delete_unreadable_state_in_store(&path, PROJECT, &unreadable[0].handle).unwrap();
}

#[test]
fn the_named_refusal_comes_only_after_every_other_check_passes() {
    // A corrupted S2-shaped run keeps its integrity refusal; the intact one is refused by name.
    let fixture: Value = serde_json::from_str(S2_UNRESOLVED_RUN).unwrap();
    let manifest_text = fixture["input_manifest_canonical"].as_str().unwrap().to_string();
    let manifest: Value = serde_json::from_str(&manifest_text).unwrap();
    let payload = manifest["model_basis"]["model_payload"].clone();
    let project_id = payload["project"]["id"].as_str().unwrap().to_string();
    let path = scratch_store("s2-precedence");
    let (mut connection, _) = crate::open_project_store(&path).unwrap();
    write_project(&mut connection, &project_id, &payload, BASE);
    drop(connection);
    let s2_id = insert_s2_run(&path, &fixture);
    assert!(read_run_in_store(&path, &project_id, &s2_id).unwrap_err().starts_with("ANALYSIS-RUN-RULE-PACKS-NOT-RESOLVED"));
    // At Record Run: a corrupt S2-shaped submission gets its own refusal first.
    let now = || Ok(T0);
    let token = || Ok("000000000000000000000000000052c1".to_string());
    let between_rows = |_: &Transaction<'_>, _: &str| Ok(());
    let hooks = RunWriteHooks { now: &now, state_token: &token, between_rows: &between_rows };
    let mut corrupt = fixture["run_record"].clone();
    corrupt["analysis_run"]["run_name"] = json!("edited without a reseal");
    let submitted = |run_record: Value, state_id: Option<String>| RecordRunRequest {
        project_id: project_id.clone(),
        expected_base: BASE.to_string(),
        run_record,
        result_json: SOURCE_RESULT.to_string(),
        input_manifest_canonical: manifest_text.clone(),
        state_id,
        new_state: Some(NewStateRequest { labels: labels_from_value(json!({ "state_name": "After solve" })).unwrap(), parent_state_id: None }),
        run_labels: run_labels("S2 run"),
    };
    let error = record_run_in_store(&path, submitted(corrupt, None), &hooks).unwrap_err();
    assert!(error.starts_with("ANALYSIS-RUN-NOT-TRANSIENT: the record's own checksum does not verify"), "{error}");
    // Naming a state that is not the solved model's: that refusal, not the named one.
    let other = model_states::list_state_entries(&Connection::open(&path).unwrap(), &project_id).unwrap();
    assert_eq!(other.len(), 1);
    let error = record_run_in_store(&path, submitted(fixture["run_record"].clone(), Some("state:absent".to_string())), &hooks).unwrap_err();
    assert!(error.starts_with("ANALYSIS-RUN-STATE-NOT-FOUND"), "{error}");
    // Intact: refused by name.
    let error = record_run_in_store(&path, submitted(fixture["run_record"].clone(), None), &hooks).unwrap_err();
    assert!(error.starts_with("ANALYSIS-RUN-RULE-PACKS-NOT-RESOLVED"), "{error}");
    // Its linked state corrupted: the state's integrity refusal, not the named one.
    tamper(&path, &format!("UPDATE model_states SET created_at = '2026-01-01T00:00:00.000Z' WHERE state_id = '{}';", fixture["state_id"].as_str().unwrap()));
    let read = read_run_in_store(&path, &project_id, &s2_id).unwrap_err();
    assert!(read.starts_with("MODEL-STATE-INTEGRITY"), "{read}");
}

// ---------------------------------------------------------------------------
// Repair r6 (W-12): full-id handles and the delete's re-check, admission
// before per-row collection, per-row classification of column fetches, and
// state summaries that count runs without pricing their ids.
// ---------------------------------------------------------------------------

/// The rowid of the only row of `table`.
fn only_rowid(path: &Path, table: &str) -> i64 {
    Connection::open(path).unwrap().query_row(&format!("SELECT rowid FROM {table}"), [], |row| row.get(0)).unwrap()
}

/// Replaces the row `rowid` of `table` by a copy whose `id_column` is `id`,
/// deleting the old row first and inserting the copy under the same rowid (as a
/// reused rowid would be), its labels row in `labels_table` moved with it.
fn replace_with_same_rowid(path: &Path, table: &str, labels_table: &str, id_column: &str, rowid: i64, id: &str) {
    tamper(path, "");
    let connection = Connection::open(path).unwrap();
    connection.execute_batch("PRAGMA foreign_keys = OFF;").unwrap();
    let columns: Vec<String> =
        connection.prepare(&format!("PRAGMA table_info({table})")).unwrap().query_map([], |row| row.get(1)).unwrap().collect::<Result<_, _>>().unwrap();
    let list = columns.join(", ");
    let old: String = connection.query_row(&format!("SELECT {id_column} FROM {table} WHERE rowid = ?1"), params![rowid], |row| row.get(0)).unwrap();
    connection
        .execute_batch(&format!(
            "CREATE TEMP TABLE replaced AS SELECT rowid AS old_rowid, {list} FROM {table} WHERE rowid = {rowid}; DELETE FROM {table} WHERE rowid = {rowid};"
        ))
        .unwrap();
    connection.execute(&format!("UPDATE replaced SET {id_column} = ?1"), params![id]).unwrap();
    connection.execute_batch(&format!("INSERT INTO {table} (rowid, {list}) SELECT old_rowid, {list} FROM replaced; DROP TABLE replaced;")).unwrap();
    connection.execute(&format!("UPDATE {labels_table} SET {id_column} = ?1 WHERE {id_column} = ?2"), params![id, old]).unwrap();
    assert_eq!(connection.query_row(&format!("SELECT {id_column} FROM {table} WHERE rowid = ?1"), params![rowid], |row| row.get::<_, String>(0)).unwrap(), id);
}

/// The handle digest of a whole id, as the store must compute it.
fn full_id_digest(id: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    Sha256::digest(id).iter().take(8).map(|byte| format!("{byte:02x}")).collect()
}

#[test]
fn a_stale_handle_never_deletes_a_replacement_whose_id_shares_its_length_and_ends() {
    // Codex r6: two ids of one length whose first and last 512 bytes agree, the
    // second taking the first's rowid. A digest of the ends let the first's
    // handle delete the second; the whole id's digest refuses it.
    let first = format!("{}A{}", "e".repeat(600), "e".repeat(600));
    let second = format!("{}B{}", "e".repeat(600), "e".repeat(600));
    // A recorded run (unreadable: its result hash is tampered).
    let (path, _) = recorded_fixture("stale-handle-run");
    let id = id_of(&path);
    tamper(&path, &format!("UPDATE analysis_runs SET result_hash = 'sha256:tampered' WHERE recording_id = '{id}';"));
    let rowid = only_rowid(&path, "analysis_runs");
    replace_with_same_rowid(&path, "analysis_runs", "analysis_run_labels", "recording_id", rowid, &first);
    let listed = unreadable_runs(&path, PROJECT);
    assert_eq!(listed.len(), 1);
    let stale = listed[0].handle.clone();
    assert_eq!(stale, format!("run-row:{rowid}:{}:{}", first.len(), full_id_digest(first.as_bytes())));
    replace_with_same_rowid(&path, "analysis_runs", "analysis_run_labels", "recording_id", rowid, &second);
    let current = unreadable_runs(&path, PROJECT);
    assert_eq!(current.len(), 1);
    assert_eq!(current[0].id, listed[0].id, "they list identically");
    assert_eq!(current[0].handle, format!("run-row:{rowid}:{}:{}", second.len(), full_id_digest(second.as_bytes())));
    assert_ne!(current[0].handle, stale, "the same rowid and length; the digest tells them apart");
    assert!(delete_unreadable_run_in_store(&path, PROJECT, &stale).unwrap_err().starts_with("ANALYSIS-RUN-NOT-FOUND"));
    assert_eq!(row_count(&path, "analysis_runs"), 1, "the replacement was not deleted");
    delete_unreadable_run_in_store(&path, PROJECT, &current[0].handle).unwrap();
    assert_eq!(row_count(&path, "analysis_runs"), 0);
    // A saved state (unreadable: its created_at differs from its record's).
    let path = store_with_project("stale-handle-state");
    let state = save_state(&path, &model("Invented project"), "Basis", "00000000000000000000000000003101", 1_000);
    tamper(&path, &format!("UPDATE model_states SET created_at = '2026-01-01T00:00:00.000Z' WHERE state_id = '{}';", state.state_id));
    let rowid = only_rowid(&path, "model_states");
    replace_with_same_rowid(&path, "model_states", "model_state_labels", "state_id", rowid, &first);
    let listed = unreadable_states(&path);
    assert_eq!(listed.len(), 1);
    let stale = listed[0].handle.clone();
    assert_eq!(stale, format!("state-row:{rowid}:{}:{}", first.len(), full_id_digest(first.as_bytes())));
    replace_with_same_rowid(&path, "model_states", "model_state_labels", "state_id", rowid, &second);
    let current = unreadable_states(&path);
    assert_eq!(current[0].id, listed[0].id);
    assert_ne!(current[0].handle, stale);
    assert!(model_states::delete_unreadable_state_in_store(&path, PROJECT, &stale).unwrap_err().starts_with("MODEL-STATE-NOT-FOUND"));
    assert_eq!(row_count(&path, "model_states"), 1, "the replacement was not deleted");
    model_states::delete_unreadable_state_in_store(&path, PROJECT, &current[0].handle).unwrap();
    assert_eq!(row_count(&path, "model_states"), 0);
}

#[test]
fn a_handle_whose_row_reads_now_is_refused_and_nothing_is_deleted() {
    // A run listed unreadable, then repaired and replaced by a readable row with
    // the same id under the same rowid: its handle still matches, and the
    // delete's re-check refuses it.
    let (path, _) = recorded_fixture("changed-run");
    let id = id_of(&path);
    let hash: String = Connection::open(&path).unwrap().query_row("SELECT result_hash FROM analysis_runs", [], |row| row.get(0)).unwrap();
    tamper(&path, &format!("UPDATE analysis_runs SET result_hash = 'sha256:tampered' WHERE recording_id = '{id}';"));
    let handle = unreadable_runs(&path, PROJECT)[0].handle.clone();
    tamper(&path, &format!("UPDATE analysis_runs SET result_hash = '{hash}' WHERE recording_id = '{id}';"));
    replace_with_same_rowid(&path, "analysis_runs", "analysis_run_labels", "recording_id", only_rowid(&path, "analysis_runs"), &id);
    let error = delete_unreadable_run_in_store(&path, PROJECT, &handle).unwrap_err();
    assert_eq!(error, format!("ANALYSIS-RUN-ROW-CHANGED: recorded run `{id}` reads now: this row changed since it was listed; refresh. Nothing was deleted."));
    assert_eq!(row_count(&path, "analysis_runs"), 1);
    assert_eq!(row_count(&path, "analysis_run_labels"), 1);
    read_run_in_store(&path, PROJECT, &id).unwrap();
    // A replacement with the same id that is also unreadable may be deleted by
    // the handle: it is the row the user asked to delete, as listed.
    tamper(&path, &format!("UPDATE analysis_runs SET result_hash = 'sha256:tampered' WHERE recording_id = '{id}';"));
    delete_unreadable_run_in_store(&path, PROJECT, &handle).unwrap();
    assert_eq!(row_count(&path, "analysis_runs"), 0);
    // A saved state likewise.
    let path = store_with_project("changed-state");
    let state = save_state(&path, &model("Invented project"), "Basis", "00000000000000000000000000003201", 1_000);
    tamper(&path, &format!("UPDATE model_states SET created_at = '2026-01-01T00:00:00.000Z' WHERE state_id = '{}';", state.state_id));
    let handle = unreadable_states(&path)[0].handle.clone();
    tamper(&path, &format!("UPDATE model_states SET created_at = '{}' WHERE state_id = '{}';", state.created_at, state.state_id));
    replace_with_same_rowid(&path, "model_states", "model_state_labels", "state_id", only_rowid(&path, "model_states"), &state.state_id);
    let error = model_states::delete_unreadable_state_in_store(&path, PROJECT, &handle).unwrap_err();
    assert_eq!(
        error,
        format!("MODEL-STATE-ROW-CHANGED: saved state `{}` reads now: this row changed since it was listed; refresh. Nothing was deleted.", state.state_id)
    );
    assert_eq!(row_count(&path, "model_states"), 1);
    model_states::read_state_in_store(&path, PROJECT, &state.state_id).unwrap();
}

#[test]
fn a_large_id_is_hashed_in_chunks_within_the_handles_reservation() {
    // A 3 MiB + 1 byte recording id (oversized, so listed unreadable): its
    // handle digests the whole id, read in 64 KiB chunks, never copied whole
    // into Rust, by the list and by the delete alike (under the 2 MiB pool
    // plus SQLite's working set for the id).
    let (path, _) = recorded_fixture("chunked-handle");
    let id = id_of(&path);
    let large = format!("{}z", "r".repeat(LARGE));
    tamper_with(&path, &format!("UPDATE analysis_runs SET recording_id = ?1 WHERE recording_id = '{id}'"), &large);
    tamper_with(&path, &format!("UPDATE analysis_run_labels SET recording_id = ?1 WHERE recording_id = '{id}'"), &large);
    let rowid = only_rowid(&path, "analysis_runs");
    let small = working_set_pool();
    let memory = PoolReadMemory::unseeded(&small);
    let (outcome, measured) = crate::ingress::counting::measure(|| {
        list_runs_in_store_with(&path, PROJECT, &memory).map(|listed| {
            listed
                .get()
                .iter()
                .filter_map(|entry| match entry {
                    ListedRun::Unreadable { unreadable } => Some(unreadable.handle.clone()),
                    ListedRun::Readable(_) => None,
                })
                .collect::<Vec<_>>()
        })
    });
    let handles = outcome.unwrap();
    assert_eq!(handles, vec![format!("run-row:{rowid}:{}:{}", large.len(), full_id_digest(large.as_bytes()))]);
    assert!(measured.peak_priced < (LARGE as u64) / 4, "the id was never loaded whole (peak {})", measured.peak_priced);
    assert_eq!(small.pool_status().reserved_bytes, 0);
    let memory = PoolReadMemory::unseeded(&small);
    let (deleted, measured) = crate::ingress::counting::measure(|| {
        delete_unreadable_run_in_store_finishing(&path, PROJECT, &handles[0], &memory, |receipt| Ok(receipt.get().deleted))
    });
    assert!(deleted.unwrap());
    assert!(measured.peak_priced < (LARGE as u64) / 4, "the delete hashed the id in chunks (peak {})", measured.peak_priced);
    assert!(memory.peak_reserved() >= stored_read::handle_chunk_bytes(), "the chunk buffer was reserved");
    assert_eq!(small.pool_status().reserved_bytes, 0);
    assert_eq!(row_count(&path, "analysis_runs"), 0);
}

/// `count` run rows (or state rows) of `PROJECT` whose created_at is longer
/// than its bound, each with small stored texts, inserted in one statement.
fn insert_oversized_rows(path: &Path, table: &str, count: i64) {
    tamper(path, "");
    let connection = Connection::open(path).unwrap();
    connection.execute_batch("PRAGMA foreign_keys = OFF;").unwrap();
    let insert = match table {
        "analysis_runs" => "INSERT INTO analysis_runs(project_id, recording_id, state_id, producer_run_id, created_at, run_record_json, run_record_hash,
             input_manifest_json, result_json, result_hash)
             SELECT ?1, 'oversized-run-' || i, 'state:none', 'producer', '2026-01-01T00:00:00.000Z-oversized-' || i, '{}', 'h-' || i, '{}', '{}', 'r-' || i FROM n",
        "model_states" => "INSERT INTO model_states(project_id, state_id, state_kind, created_at, parent_state_id, model_canonical, model_hash, record_json, record_hash)
             SELECT ?1, 'oversized-state-' || i, 'design_snapshot', '2026-01-01T00:00:00.000Z-oversized-' || i, NULL, '{}', 'm-' || i, '{}', 'r-' || i FROM n",
        other => panic!("{other}"),
    };
    connection
        .execute(&format!("WITH RECURSIVE n(i) AS (SELECT 1 UNION ALL SELECT i + 1 FROM n WHERE i < {count}) {insert}"), params![PROJECT])
        .unwrap();
}

#[test]
fn a_hundred_thousand_oversized_rows_are_counted_and_admitted_before_any_entry_is_collected() {
    // Codex r6: the oversized rows were collected before any reservation.
    const ROWS: i64 = 100_000;
    for (table, what) in [("analysis_runs", "recorded-run"), ("model_states", "saved-state")] {
        let (path, _) = recorded_fixture(&format!("many-oversized-{table}"));
        insert_oversized_rows(&path, table, ROWS);
        let small = small_pool();
        let memory = Recording { inner: PoolReadMemory::unseeded(&small), seen: std::cell::RefCell::new(Vec::new()) };
        let (outcome, measured) = crate::ingress::counting::measure(|| match table {
            "analysis_runs" => list_runs_in_store_with(&path, PROJECT, &memory).map(|_| ()),
            _ => model_states::list_states_in_store_with(&path, PROJECT, &memory).map(|_| ()),
        });
        let error = outcome.unwrap_err();
        let admission = format!("the {ROWS} oversized {what} rows' list entries of project `{PROJECT}`");
        assert!(error.starts_with(&format!("STORE-READ-MEMORY-UNAVAILABLE: reading {admission}")), "{table}: {error}");
        let seen: Vec<String> = memory.seen.borrow().iter().map(|(what, _)| what.clone()).collect();
        assert_eq!(seen.len(), 2, "{table}: {seen:?}");
        assert!(seen[0].starts_with("SQLite's working set for the stored history"), "{table}: {seen:?}");
        assert_eq!(seen[1], admission, "{table}: the entries' admission follows only SQLite's working set");
        assert!(measured.peak_priced < 256 * 1024, "{table}: nothing per row was allocated before the admission (peak {})", measured.peak_priced);
        assert_eq!(small.pool_status().reserved_bytes, 0);
    }
    // Admitted, the entries stay within their reservations through the response.
    let (path, _) = recorded_fixture("some-oversized-runs");
    insert_oversized_rows(&path, "analysis_runs", 1_000);
    assert_within_reservation("list_analysis_runs", |memory| list_runs_in_store_with(&path, PROJECT, memory));
    assert_eq!(unreadable_runs(&path, PROJECT).len(), 1_000);
    let (path, _) = recorded_fixture("some-oversized-states");
    insert_oversized_rows(&path, "model_states", 1_000);
    assert_within_reservation("list_model_states", |memory| model_states::list_states_in_store_with(&path, PROJECT, memory));
    assert_eq!(unreadable_states(&path).len(), 1_000);
}

#[test]
fn a_stored_column_of_the_wrong_type_makes_only_its_row_unreadable_or_its_read_refused_by_name() {
    // Codex r6: run_record_json or record_json set to X'FF' hid the whole list
    // behind STORE-SQL-ERROR.
    let (path, _) = recorded_fixture("blob-record");
    let first = id_of(&path);
    let solved = model("Invented project");
    let second = record(&path, request(&solved, "Second"), T0 + 5).unwrap().run.recording_id;
    tamper(&path, &format!("UPDATE analysis_runs SET run_record_json = X'FF' WHERE recording_id = '{first}';"));
    let entries = unreadable_runs(&path, PROJECT);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].id, first);
    assert!(
        entries[0].refusal.starts_with(&format!("ANALYSIS-RUN-INTEGRITY: recorded run `{first}`: a stored column cannot be read")),
        "{}",
        entries[0].refusal
    );
    assert!(list_run_entries(&Connection::open(&path).unwrap(), PROJECT).unwrap().iter().any(|entry| matches!(entry, ListedRun::Readable(run) if run.recording_id == second)));
    assert!(read_run_in_store(&path, PROJECT, &first).unwrap_err().starts_with(&format!("ANALYSIS-RUN-INTEGRITY: recorded run `{first}`: a stored column cannot be read")));
    delete_unreadable_run_in_store(&path, PROJECT, &entries[0].handle).unwrap();
    // A record that is an integer.
    tamper(&path, &format!("UPDATE analysis_runs SET run_record_json = 42 WHERE recording_id = '{second}';"));
    let entries = unreadable_runs(&path, PROJECT);
    assert_eq!(entries.iter().map(|entry| entry.id.clone()).collect::<Vec<_>>(), vec![second.clone()]);
    delete_unreadable_run_in_store(&path, PROJECT, &entries[0].handle).unwrap();
    // A result of the wrong type: the run lists (the list loads no result) and its read is refused by name.
    let third = record(&path, request(&solved, "Third"), T0 + 9).unwrap().run.recording_id;
    tamper(&path, &format!("UPDATE analysis_runs SET result_json = X'FF' WHERE recording_id = '{third}';"));
    assert_eq!(list_runs(&Connection::open(&path).unwrap(), PROJECT).unwrap().len(), 1);
    let error = read_run_in_store(&path, PROJECT, &third).unwrap_err();
    assert!(error.starts_with(&format!("ANALYSIS-RUN-INTEGRITY: recorded run `{third}`: a stored column cannot be read")), "{error}");
    // A saved state's record of the wrong type: its row is unreadable, the other state lists.
    let path = store_with_project("blob-state-record");
    let a = save_state(&path, &model("Invented project"), "A", "00000000000000000000000000003301", 1_000);
    let b = save_state(&path, &model("Other project"), "B", "00000000000000000000000000003302", 2_000);
    tamper(&path, &format!("UPDATE model_states SET record_json = X'FF' WHERE state_id = '{}';", a.state_id));
    let unreadable = unreadable_states(&path);
    assert_eq!(unreadable.iter().map(|entry| entry.id.clone()).collect::<Vec<_>>(), vec![a.state_id.clone()]);
    assert!(unreadable[0].refusal.contains("a stored column cannot be read"), "{}", unreadable[0].refusal);
    assert!(readable_states(&path).iter().any(|state| state.state_id == b.state_id));
    assert!(model_states::read_state_in_store(&path, PROJECT, &a.state_id).unwrap_err().starts_with("MODEL-STATE-INTEGRITY"));
    model_states::delete_unreadable_state_in_store(&path, PROJECT, &unreadable[0].handle).unwrap();
    // A snapshot of the wrong type: the state lists and its read is refused by name.
    tamper(&path, &format!("UPDATE model_states SET model_canonical = X'FF' WHERE state_id = '{}';", b.state_id));
    assert_eq!(readable_states(&path).len(), 1);
    let error = model_states::read_state_in_store(&path, PROJECT, &b.state_id).unwrap_err();
    assert!(error.starts_with(&format!("MODEL-STATE-INTEGRITY: saved state `{}`: a stored column cannot be read", b.state_id)), "{error}");
}

#[test]
fn a_state_with_a_thousand_runs_of_4096_byte_ids_lists_and_reads_under_2_mib() {
    // Codex r6: the state summary still priced the removed run-id lists.
    let (path, _) = recorded_fixture("many-runs-one-state");
    let state_id: String = Connection::open(&path).unwrap().query_row("SELECT state_id FROM analysis_runs", [], |row| row.get(0)).unwrap();
    tamper(&path, "");
    let connection = Connection::open(&path).unwrap();
    connection.execute_batch("PRAGMA foreign_keys = OFF;").unwrap();
    connection
        .execute(
            "WITH RECURSIVE n(i) AS (SELECT 1 UNION ALL SELECT i + 1 FROM n WHERE i < 1000)
             INSERT INTO analysis_runs(project_id, recording_id, state_id, producer_run_id, created_at, run_record_json, run_record_hash,
               input_manifest_json, result_json, result_hash)
             SELECT ?1, printf('%04d', i) || substr(hex(zeroblob(2048)), 5), ?2, 'producer', '2026-01-01T00:00:00.000Z', '{}', 'h-' || i, '{}', '{}', 'r-' || i FROM n",
            params![PROJECT, state_id],
        )
        .unwrap();
    let longest: i64 = connection.query_row("SELECT MAX(octet_length(recording_id)) FROM analysis_runs", [], |row| row.get(0)).unwrap();
    assert_eq!(longest, stored_read::MAX_ID_BYTES);
    drop(connection);
    let small = small_pool();
    let memory = PoolReadMemory::unseeded(&small);
    let listed = model_states::list_states_in_store_with(&path, PROJECT, &memory).unwrap();
    let count = listed.get().iter().find_map(|entry| match entry {
        model_states::ListedState::Readable(summary) if summary.state_id == state_id => Some(summary.recorded_run_count),
        _ => None,
    });
    assert_eq!(count, Some(1_001));
    drop(listed);
    let read = model_states::read_state_in_store_with(&path, PROJECT, &state_id, &memory).unwrap();
    assert_eq!(read.get().state.recorded_run_count, 1_001);
    drop(read);
    drop(memory);
    assert_eq!(small.pool_status().reserved_bytes, 0);
}

/// Runs `body` alone in a child process of this test binary (SQLite's memory
/// status is process-wide, so other tests' statements would move it): the
/// parent re-runs the test `name` with `--exact --test-threads=1`.
fn in_own_process(name: &str, body: impl FnOnce()) {
    const CHILD: &str = "OPENPIPESTRESS_SQLITE_STATUS_CHILD";
    if std::env::var_os(CHILD).is_some() {
        body();
        return;
    }
    let path = format!("{}::{name}", module_path!().split_once("::").map(|(_, rest)| rest).unwrap_or(module_path!()));
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args([path.as_str(), "--exact", "--test-threads=1", "--nocapture"])
        .env(CHILD, "1")
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success() && stdout.contains("1 passed"), "{stdout}\n{}", String::from_utf8_lossy(&output.stderr));
}

/// `f`'s result and SQLite's own allocation peak during it, above what SQLite
/// held when it began (`sqlite3_status64(SQLITE_STATUS_MEMORY_USED)`).
fn sqlite_peak_during<T>(f: impl FnOnce() -> T) -> (T, u64) {
    let (mut current, mut high) = (0i64, 0i64);
    // SAFETY: plain out-pointers to two live i64s; the status call takes no other state.
    unsafe { rusqlite::ffi::sqlite3_status64(rusqlite::ffi::SQLITE_STATUS_MEMORY_USED, &mut current, &mut high, 1) };
    let start = current;
    let out = f();
    // SAFETY: as above.
    unsafe { rusqlite::ffi::sqlite3_status64(rusqlite::ffi::SQLITE_STATUS_MEMORY_USED, &mut current, &mut high, 0) };
    (out, u64::try_from(high - start).unwrap_or(0))
}

/// The working set a store whose largest keyed row holds `bytes` reserves.
fn sqlite_working_set(bytes: u64) -> u64 {
    bytes.saturating_mul(stored_read::SQLITE_WORKING_SET_FACTOR)
}

#[test]
fn sqlite_allocations_for_an_oversized_id_are_admitted_before_sqlite_loads_it() {
    // Codex r7: an 8 MiB id was listed and deleted under a 2 MiB pool while
    // SQLite's own peak grew by 17 MB (list) and 44.7 MB (delete).
    in_own_process("sqlite_allocations_for_an_oversized_id_are_admitted_before_sqlite_loads_it", || {
        const BIG: usize = 8 << 20;
        let big = "q".repeat(BIG);
        let large_pool = pool_of(512 << 20);
        for kind in ["run", "state"] {
            // A row whose 8 MiB id is also its labels row's key.
            let path = match kind {
                "run" => {
                    let (path, _) = recorded_fixture("sqlite-big-run-id");
                    let id = id_of(&path);
                    tamper_with(&path, &format!("UPDATE analysis_runs SET recording_id = ?1 WHERE recording_id = '{id}'"), &big);
                    tamper_with(&path, &format!("UPDATE analysis_run_labels SET recording_id = ?1 WHERE recording_id = '{id}'"), &big);
                    path
                }
                _ => {
                    let path = store_with_project("sqlite-big-state-id");
                    let state = save_state(&path, &model("Invented project"), "Basis", "00000000000000000000000000004001", 1_000);
                    tamper_with(&path, &format!("UPDATE model_states SET state_id = ?1 WHERE state_id = '{}'", state.state_id), &big);
                    tamper_with(&path, &format!("UPDATE model_state_labels SET state_id = ?1 WHERE state_id = '{}'", state.state_id), &big);
                    path
                }
            };
            let row_bytes: i64 = Connection::open(&path)
                .unwrap()
                .query_row(
                    if kind == "run" {
                        "SELECT MAX(octet_length(project_id) + octet_length(recording_id) + octet_length(state_id) + octet_length(producer_run_id)
                         + octet_length(created_at) + octet_length(run_record_hash) + octet_length(result_hash)) FROM analysis_runs"
                    } else {
                        "SELECT MAX(octet_length(project_id) + octet_length(state_id) + octet_length(state_kind) + octet_length(created_at)
                         + COALESCE(octet_length(parent_state_id), 0) + octet_length(model_hash) + octet_length(record_hash)) FROM model_states"
                    },
                    [],
                    |row| row.get(0),
                )
                .unwrap();
            let working_set = sqlite_working_set(row_bytes as u64);
            let list = |memory: &PoolReadMemory<'_>| match kind {
                "run" => list_runs_in_store_with(&path, PROJECT, memory).map(|listed| listed.get().len()),
                _ => model_states::list_states_in_store_with(&path, PROJECT, memory).map(|listed| listed.get().len()),
            };
            let refusal = "STORE-READ-MEMORY-UNAVAILABLE: reading SQLite's working set for the stored history";
            // Under a 2 MiB pool: refused before SQLite loads the id.
            let small = small_pool();
            let memory = PoolReadMemory::unseeded(&small);
            let (outcome, peak) = sqlite_peak_during(|| list(&memory));
            assert!(outcome.as_ref().unwrap_err().starts_with(refusal), "{kind} list: {outcome:?}");
            assert!(peak <= SQLITE_PAGE_CACHE_SLACK, "{kind} list: SQLite never loaded the id; its peak {peak} is within the page-cache residual");
            // Under a large pool: listed, SQLite's peak within the working set.
            let memory = PoolReadMemory::unseeded(&large_pool);
            let (outcome, peak) = sqlite_peak_during(|| list(&memory));
            assert_eq!(outcome.unwrap(), 1, "{kind} list");
            println!("{kind} list: SQLite peak {peak} = {:.2} x the id; working set {working_set}", peak as f64 / BIG as f64);
            assert!(peak <= working_set, "{kind} list: SQLite peak {peak} within the working set {working_set}");
            let handle = match kind {
                "run" => unreadable_runs(&path, PROJECT)[0].handle.clone(),
                _ => unreadable_states(&path)[0].handle.clone(),
            };
            let delete = |memory: &PoolReadMemory<'_>| match kind {
                "run" => delete_unreadable_run_in_store_finishing(&path, PROJECT, &handle, memory, |receipt| Ok(receipt.get().deleted)),
                _ => model_states::delete_unreadable_state_in_store_finishing(&path, PROJECT, &handle, memory, |receipt| Ok(receipt.get().deleted)),
            };
            let memory = PoolReadMemory::unseeded(&small);
            let (outcome, peak) = sqlite_peak_during(|| delete(&memory));
            assert!(outcome.as_ref().unwrap_err().starts_with(refusal), "{kind} delete: {outcome:?}");
            assert!(peak <= SQLITE_PAGE_CACHE_SLACK, "{kind} delete: SQLite never loaded the id; its peak {peak} is within the page-cache residual");
            let table = if kind == "run" { "analysis_runs" } else { "model_states" };
            assert_eq!(row_count(&path, table), 1, "{kind}: nothing was deleted");
            let memory = PoolReadMemory::unseeded(&large_pool);
            let (outcome, peak) = sqlite_peak_during(|| delete(&memory));
            assert!(outcome.unwrap(), "{kind} delete");
            println!("{kind} delete: SQLite peak {peak} = {:.2} x the id; working set {working_set}", peak as f64 / BIG as f64);
            assert!(peak <= working_set, "{kind} delete: SQLite peak {peak} within the working set {working_set}");
            assert_eq!(row_count(&path, table), 0);
            assert_eq!(small.pool_status().reserved_bytes, 0);
            assert_eq!(large_pool.pool_status().reserved_bytes, 0);
        }
        // Normal-size rows: listed and deleted under the 2 MiB pool, SQLite's
        // peak (its page cache) within what the command reserved.
        let (path, _) = recorded_fixture("sqlite-normal-rows");
        let id = id_of(&path);
        tamper(&path, &format!("UPDATE analysis_runs SET result_hash = 'sha256:tampered' WHERE recording_id = '{id}';"));
        let small = small_pool();
        let memory = PoolReadMemory::unseeded(&small);
        let (outcome, peak) = sqlite_peak_during(|| list_runs_in_store_with(&path, PROJECT, &memory).map(|listed| listed.get().len()));
        assert_eq!(outcome.unwrap(), 1);
        assert!(peak <= memory.peak_reserved(), "normal list: SQLite peak {peak} within {}", memory.peak_reserved());
        let memory = PoolReadMemory::unseeded(&small);
        let (outcome, peak) = sqlite_peak_during(|| model_states::list_states_in_store_with(&path, PROJECT, &memory).map(|listed| listed.get().len()));
        assert_eq!(outcome.unwrap(), 1);
        assert!(peak <= memory.peak_reserved(), "normal state list: SQLite peak {peak} within {}", memory.peak_reserved());
        let handle = unreadable_runs(&path, PROJECT)[0].handle.clone();
        let memory = PoolReadMemory::unseeded(&small);
        let (outcome, peak) = sqlite_peak_during(|| delete_unreadable_run_in_store_finishing(&path, PROJECT, &handle, &memory, |receipt| Ok(receipt.get().deleted)));
        assert!(outcome.unwrap());
        assert!(peak <= memory.peak_reserved(), "normal delete: SQLite peak {peak} within {}", memory.peak_reserved());
        assert_eq!(small.pool_status().reserved_bytes, 0);
    });
}

/// SQLite's page cache, which stays within the accepted residual estimate
/// (DEL-14-01 Design §3; Codex r8 measured up to 2.48 MB of cache): an
/// interval's SQLite peak may exceed what the read holds by at most this, and
/// a command that never loads a large value peaks within it.
const SQLITE_PAGE_CACHE_SLACK: u64 = 3 << 20;

/// Runs `command` under `memory` and checks that in every interval between
/// changes of its reservations SQLite's own peak stayed within what it held
/// then (plus the page cache): nothing is loaded by SQLite after the lease that
/// priced it was released. Returns the outcome and SQLite's overall peak, the
/// largest of the intervals' (each reservation change resets SQLite's
/// high-water mark, so no single reading spans the command).
fn within_held_reservations<T>(memory: &PoolReadMemory<'_>, what: &str, command: impl FnOnce() -> T) -> (T, u64) {
    stored_read::release_probe::sqlite_start();
    let outcome = command();
    let intervals = stored_read::release_probe::sqlite_take(memory.held());
    for (index, (sqlite, held)) in intervals.iter().enumerate() {
        assert!(*sqlite <= held + SQLITE_PAGE_CACHE_SLACK, "{what}: interval {index}: SQLite peaked at {sqlite} bytes while {held} were held ({intervals:?})");
    }
    let peak = intervals.iter().map(|(sqlite, _)| *sqlite).max().unwrap_or(0);
    (outcome, peak)
}

#[test]
fn the_interval_probe_reports_a_known_sqlite_allocation_in_any_interval() {
    // Codex r9: the probe returned only the last interval's peak (0 for a 4 MiB allocation).
    in_own_process("the_interval_probe_reports_a_known_sqlite_allocation_in_any_interval", || {
        const KNOWN: u64 = 4 << 20;
        let pool = pool_of(64 << 20);
        let memory = PoolReadMemory::unseeded(&pool);
        let (_, peak) = within_held_reservations(&memory, "the calibration", || {
            let held = memory.reserve("the calibration's allocation", KNOWN).unwrap();
            // SAFETY: a fresh SQLite allocation, freed once, never read or written.
            unsafe {
                let block = rusqlite::ffi::sqlite3_malloc64(KNOWN);
                assert!(!block.is_null());
                rusqlite::ffi::sqlite3_free(block);
            }
            drop(held);
            // Later intervals allocate nothing.
            drop(memory.reserve("a later interval", 1024).unwrap());
        });
        assert!(peak >= KNOWN, "the 4 MiB allocation is reported (peak {peak})");
    });
}

#[test]
fn a_project_save_time_that_is_not_an_integer_is_refused_by_name_without_sqlite_loading_it() {
    // Codex r8: an 8 MiB text in updated_at_unix was materialized by the stale-project
    // guard (10.97 MB in SQLite with 12,488 bytes reserved), then STORE-SQL-ERROR.
    in_own_process("a_project_save_time_that_is_not_an_integer_is_refused_by_name_without_sqlite_loading_it", || {
        const BIG: usize = 8 << 20;
        let path = store_with_project("sqlite-text-save-time");
        let solved = model("Invented project");
        save_state(&path, &solved, "Basis", "00000000000000000000000000004201", 1_000);
        tamper_with(&path, "UPDATE local_projects SET updated_at_unix = ?1", &"t".repeat(BIG));
        let kind: String = Connection::open(&path).unwrap().query_row("SELECT typeof(updated_at_unix) FROM local_projects", [], |row| row.get(0)).unwrap();
        assert_eq!(kind, "text");
        let refusal = format!("PROJECT-BASE-MOVED: project `{PROJECT}` (\"Invented project\", its last save time cannot be read) changed after this session opened or saved it");
        let small = small_pool();
        // Save Model State with a stale base.
        let model_canonical = canonical_json(&model("Other project"));
        let save = SaveModelStateRequest {
            project_id: PROJECT.to_string(),
            expected_base: "sha256:older".to_string(),
            model_hash: format!("sha256:{}", sha256_hex(&model_canonical)),
            model_canonical,
            state_kind: "design_snapshot".to_string(),
            parent_state_id: None,
            labels: labels_from_value(json!({ "state_name": "Other" })).unwrap(),
        };
        let allocate = || Ok(("00000000000000000000000000004202".to_string(), 2_000));
        let between_rows = |_: &Transaction<'_>| Ok(());
        let hooks = StateWriteHooks { allocate: &allocate, between_rows: &between_rows };
        let memory = PoolReadMemory::unseeded(&small);
        let (outcome, peak) = within_held_reservations(&memory, "Save Model State", || model_states::save_state_in_store_with(&path, save, &hooks, &memory).map(|_| ()));
        let error = outcome.unwrap_err();
        assert!(error.starts_with(&refusal), "{error}");
        assert!(peak <= SQLITE_PAGE_CACHE_SLACK, "Save Model State: SQLite never loaded the time; its peak {peak} is within the page-cache residual");
        drop(memory);
        // Record Run with a stale base.
        let mut stale = request(&solved, "Run");
        stale.expected_base = "sha256:older".to_string();
        let now = || Ok(T0);
        let token = || Ok("0000000000000000000000000000e00d".to_string());
        let run_rows = |_: &Transaction<'_>, _: &str| Ok(());
        let run_hooks = RunWriteHooks { now: &now, state_token: &token, between_rows: &run_rows };
        let memory = PoolReadMemory::unseeded(&small);
        let (outcome, peak) = within_held_reservations(&memory, "Record Run", || record_run_in_store_with(&path, stale, &run_hooks, &memory).map(|_| ()));
        let error = outcome.unwrap_err();
        assert!(error.starts_with(&refusal), "{error}");
        assert!(peak <= SQLITE_PAGE_CACHE_SLACK, "Record Run: SQLite never loaded the time; its peak {peak} is within the page-cache residual");
        drop(memory);
        assert_eq!(small.pool_status().reserved_bytes, 0);
        assert_eq!(row_count(&path, "model_states"), 1);
        assert_eq!(row_count(&path, "analysis_runs"), 0);
    });
}

#[test]
fn a_blocked_delete_names_its_children_from_their_verification_without_reloading_a_name() {
    // Codex r8: a blocked parent delete reloaded each child's name after the child's
    // lease was released (8.43 MB in SQLite for an 8 MiB name with 526,200 bytes held).
    in_own_process("a_blocked_delete_names_its_children_from_their_verification_without_reloading_a_name", || {
        const BIG: usize = 8 << 20;
        let path = store_with_project("sqlite-blocker-name");
        let parent = save_state(&path, &model("Invented project"), "Parent", "00000000000000000000000000004301", 1_000);
        let child_model = model("Child project");
        let model_canonical = canonical_json(&child_model);
        let request = SaveModelStateRequest {
            project_id: PROJECT.to_string(),
            expected_base: BASE.to_string(),
            model_hash: format!("sha256:{}", sha256_hex(&model_canonical)),
            model_canonical,
            state_kind: "design_snapshot".to_string(),
            parent_state_id: Some(parent.state_id.clone()),
            labels: labels_from_value(json!({ "state_name": "Child" })).unwrap(),
        };
        let allocate = || Ok(("00000000000000000000000000004302".to_string(), 2_000));
        let between_rows = |_: &Transaction<'_>| Ok(());
        let child = save_state_in_store(&path, request, &StateWriteHooks { allocate: &allocate, between_rows: &between_rows }).unwrap().state;
        tamper_with(&path, &format!("UPDATE model_state_labels SET state_name = ?1 WHERE state_id = '{}'", child.state_id), &"n".repeat(BIG));
        // A pool that admits the list (each child verified within its own lease).
        let pool = pool_of(128 << 20);
        let memory = PoolReadMemory::unseeded(&pool);
        let listed = model_states::list_states_in_store_with(&path, PROJECT, &memory).unwrap();
        let children = listed.get().iter().find_map(|entry| match entry {
            model_states::ListedState::Readable(summary) if summary.state_id == parent.state_id => Some(summary.child_state_ids.clone()),
            _ => None,
        });
        assert_eq!(children, Some(vec![child.state_id.clone()]));
        drop(listed);
        drop(memory);
        let memory = PoolReadMemory::unseeded(&pool);
        let (outcome, _) = within_held_reservations(&memory, "the blocked delete", || model_states::delete_state_in_store_with(&path, PROJECT, &parent.state_id, &memory).map(|_| ()));
        let error = outcome.unwrap_err();
        assert!(error.starts_with("MODEL-STATE-DELETE-BLOCKED: state \"Parent\""), "{}", stored_read::shown(&error));
        assert!(error.contains(&format!("the parent of saved state \"{}…\" ({})", "n".repeat(120), child.state_id)), "{}", stored_read::shown(&error));
        assert!(error.chars().count() <= stored_read::MAX_MESSAGE_CHARS + 1, "the message is capped");
        drop(memory);
        assert_eq!(pool.pool_status().reserved_bytes, 0);
        assert_eq!(row_count(&path, "model_states"), 2);
    });
}

// ---------------------------------------------------------------------------
// Run comparison admission (W-12 S3b; DEL-14-01 Design §4.2, Memory): the
// combined stored size of the two runs' results and input manifests (with the
// session's current run when it is one side) against the per-invocation
// budget, in one read transaction that reserves SQLite's working set first;
// same project only; a missing or unreadable run is refused; every lease is
// released. All data are invented.
// ---------------------------------------------------------------------------

/// A store with two recorded runs of one state: the fixture's and a second
/// recording of the same solve (its own `created_at`), with the stored lengths
/// of each run's result and manifest.
fn two_recorded_runs(label: &str) -> (PathBuf, [String; 2], u64) {
    let (path, _) = recorded_fixture(label);
    let first = id_of(&path);
    let solved = model("Invented project");
    let second = record(&path, request(&solved, "Run 2"), T0 + 5_000).unwrap().run.recording_id;
    let per_run = (result_text(1.25).len() + manifest_text(&solved).len()) as u64;
    (path, [first, second], per_run)
}

fn ids(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}

#[test]
fn a_run_comparison_is_admitted_by_the_combined_stored_size_of_its_inputs() {
    let (path, [first, second], per_run) = two_recorded_runs("compare-runs-admission");
    let combined = 2 * per_run;
    let both = ids(&[&first, &second]);
    // Within the budget (at it exactly): admitted, with the stored lengths in the order named.
    let admitted = admit_run_comparison_in_store_with(&path, PROJECT, &both, None, combined, &UnmeteredReadMemory).unwrap();
    let lengths = RunComparisonInputBytes { result_bytes: result_text(1.25).len() as u64, manifest_bytes: manifest_text(&model("Invented project")).len() as u64 };
    assert_eq!(admitted, AnalysisRunComparisonAdmission { runs: vec![lengths, lengths], current_run_bytes: 0, combined_bytes: combined, budget_bytes: combined });
    // One byte over: refused by name, naming both runs, before either is read.
    let refused = admit_run_comparison_in_store_with(&path, PROJECT, &both, None, combined - 1, &UnmeteredReadMemory).unwrap_err();
    assert!(refused.starts_with(COMPARISON_RUNS_TOO_LARGE), "{refused}");
    assert!(refused.contains(&first) && refused.contains(&second) && refused.contains(&format!("{combined} bytes together")), "{refused}");
    // One recorded run and the session's current run: the current run's bytes count.
    let with_current = admit_run_comparison_in_store_with(&path, PROJECT, &ids(&[&first]), Some(700), per_run + 700, &UnmeteredReadMemory).unwrap();
    assert_eq!((with_current.runs.len(), with_current.current_run_bytes, with_current.combined_bytes), (1, 700, per_run + 700));
    let refused = admit_run_comparison_in_store_with(&path, PROJECT, &ids(&[&first]), Some(701), per_run + 700, &UnmeteredReadMemory).unwrap_err();
    assert!(refused.starts_with(COMPARISON_RUNS_TOO_LARGE) && refused.contains("and the session's current run"), "{refused}");
    // Two sides exactly: two different recorded runs, or one and the current run.
    for (named, current) in [(ids(&[]), Some(1)), (ids(&[&first]), None), (ids(&[&first, &first]), None), (both.clone(), Some(1)), (ids(&[&first, &second, &first]), None)] {
        let error = admit_run_comparison_in_store_with(&path, PROJECT, &named, current, u64::MAX, &UnmeteredReadMemory).unwrap_err();
        assert!(error.starts_with(COMPARISON_RUNS_ARGS), "{named:?} {current:?}: {error}");
    }
    // Same project only: a run of another project, or none, is not found.
    let missing = admit_run_comparison_in_store_with(&path, PROJECT, &ids(&[&first, "runrec:00000000000000000000000000000000"]), None, u64::MAX, &UnmeteredReadMemory).unwrap_err();
    assert!(missing.starts_with("ANALYSIS-RUN-NOT-FOUND"), "{missing}");
    let other_project = admit_run_comparison_in_store_with(&path, "project:another-invented", &both, None, u64::MAX, &UnmeteredReadMemory).unwrap_err();
    assert!(other_project.starts_with("ANALYSIS-RUN-NOT-FOUND"), "{other_project}");
}

#[test]
fn a_run_comparison_refuses_an_unreadable_run_by_its_integrity_refusal() {
    // A record that no longer verifies.
    let (path, [first, second], _) = two_recorded_runs("compare-runs-unreadable");
    tamper(&path, &format!("UPDATE analysis_runs SET run_record_json = replace(run_record_json, 'MECHANICS_SOLVED', 'MECHANICS_SOLVED_X') WHERE recording_id = '{second}';"));
    let error = admit_run_comparison_in_store_with(&path, PROJECT, &ids(&[&first, &second]), None, u64::MAX, &UnmeteredReadMemory).unwrap_err();
    assert!(error.starts_with(&format!("ANALYSIS-RUN-INTEGRITY: recorded run `{second}`")), "{error}");
    // An identity column beyond its bound: refused before it is loaded.
    let (path, [first, second], _) = two_recorded_runs("compare-runs-oversized");
    tamper_with(&path, &format!("UPDATE analysis_runs SET created_at = ?1 WHERE recording_id = '{second}'"), &"z".repeat(LARGE));
    assert_oversized_refused("the run comparison admission", "created_at", |memory| {
        admit_run_comparison_in_store_with(&path, PROJECT, &ids(&[&first, &second]), None, u64::MAX, memory)
    });
    // A result stored with the wrong type is the read's refusal, never the admission's
    // load: the admission reads its length only.
    let (path, [first, second], _) = two_recorded_runs("compare-runs-blob");
    tamper(&path, &format!("UPDATE analysis_runs SET result_json = CAST(result_json AS BLOB) WHERE recording_id = '{second}';"));
    let admitted = admit_run_comparison_in_store_with(&path, PROJECT, &ids(&[&first, &second]), None, u64::MAX, &UnmeteredReadMemory).unwrap();
    assert_eq!(admitted.runs.len(), 2);
    let error = read_run_in_store(&path, PROJECT, &second).unwrap_err();
    assert!(error.starts_with("ANALYSIS-RUN-INTEGRITY"), "{error}");
}

#[test]
fn a_run_comparison_reserves_sqlites_working_set_before_it_reads_a_row() {
    let (path, [first, second], _) = two_recorded_runs("compare-runs-working-set");
    // Another project's run row whose producer run id (a keyed column) holds LARGE bytes:
    // SQLite may materialize it on an index search path, so its working set is reserved first.
    tamper_with(
        &path,
        "INSERT INTO analysis_runs(project_id, recording_id, state_id, producer_run_id, created_at, run_record_json, run_record_hash,
             input_manifest_json, result_json, result_hash)
         VALUES ('project:another-invented', 'runrec:other', 'state:other', ?1, '2026-01-01T00:00:00.000Z', '{}', 'h', '{}', '{}', 'r')",
        &"q".repeat(LARGE),
    );
    let both = ids(&[&first, &second]);
    // Under the 2 MiB pool: refused by name at the working set, its first reservation.
    let small = small_pool();
    let memory = Recording { inner: PoolReadMemory::unseeded(&small), seen: std::cell::RefCell::new(Vec::new()) };
    let error = admit_run_comparison_in_store_with(&path, PROJECT, &both, None, u64::MAX, &memory).unwrap_err();
    assert!(error.starts_with("STORE-READ-MEMORY-UNAVAILABLE: reading SQLite's working set for the stored history"), "{error}");
    let seen: Vec<String> = memory.seen.borrow().iter().map(|(what, _)| what.clone()).collect();
    assert_eq!(seen.len(), 1, "{seen:?}");
    drop(memory);
    assert_eq!(small.pool_status().reserved_bytes, 0);
    // With room for the working set: admitted, the working set reserved before each run's reads.
    let pool = working_set_pool();
    let memory = Recording { inner: PoolReadMemory::unseeded(&pool), seen: std::cell::RefCell::new(Vec::new()) };
    let admitted = admit_run_comparison_in_store_with(&path, PROJECT, &both, None, u64::MAX, &memory).unwrap();
    assert_eq!(admitted.runs.len(), 2);
    let seen: Vec<(String, u64)> = memory.seen.borrow().clone();
    assert!(seen[0].0.starts_with("SQLite's working set for the stored history"), "{seen:?}");
    assert!(seen[1..].iter().all(|(_, held)| *held >= stored_read::SQLITE_WORKING_SET_FACTOR * LARGE as u64), "the working set is held through every later reservation: {seen:?}");
    drop(memory);
    assert_eq!(pool.pool_status().reserved_bytes, 0);
}

#[test]
fn the_run_comparison_command_answers_lengths_only_and_releases_every_lease() {
    let (path, [first, second], per_run) = two_recorded_runs("compare-runs-command");
    let combined = 2 * per_run;
    let pool = registry(ScriptedHost::normal());
    let arguments = json!({ "projectId": PROJECT, "recordingIds": [first, second] });
    let (mut args, lease) = admit_json(&pool, "admit_analysis_run_comparison", ArgsDecoding::Moved, &arguments).unwrap();
    {
        let memory = PoolReadMemory::new(&pool, &lease);
        let response = stored_read::respond_metered(
            &memory,
            "admit_analysis_run_comparison",
            admit_analysis_run_comparison_args(&mut args, || Ok(path.clone()), combined, &memory),
        )
        .unwrap();
        use tauri::ipc::IpcResponse;
        let body: Value = match response.body().unwrap() {
            tauri::ipc::InvokeResponseBody::Json(text) => serde_json::from_str(&text).unwrap(),
            tauri::ipc::InvokeResponseBody::Raw(_) => panic!("raw response"),
        };
        let lengths = json!({ "result_bytes": result_text(1.25).len(), "manifest_bytes": manifest_text(&model("Invented project")).len() });
        assert_eq!(body, json!({ "runs": [lengths, lengths], "current_run_bytes": 0, "combined_bytes": combined, "budget_bytes": combined }));
        assert!(!body.to_string().contains("runrec:"), "no id is echoed");
    }
    drop(lease);
    assert_eq!(pool.pool_status().reserved_bytes, 0);
    // Refused (over the budget, a missing run, an unreadable one): every lease released too.
    tamper(&path, &format!("UPDATE analysis_runs SET run_record_hash = 'tampered' WHERE recording_id = '{second}';"));
    for (budget, arguments, code) in [
        (combined - 1, json!({ "projectId": PROJECT, "recordingIds": [first, "runrec:missing"] }), "ANALYSIS-RUN-NOT-FOUND"),
        (u64::MAX, json!({ "projectId": PROJECT, "recordingIds": [first, second] }), "ANALYSIS-RUN-INTEGRITY"),
        (per_run, json!({ "projectId": PROJECT, "recordingIds": [first], "currentRunBytes": 1 }), COMPARISON_RUNS_TOO_LARGE),
    ] {
        let (mut args, lease) = admit_json(&pool, "admit_analysis_run_comparison", ArgsDecoding::Moved, &arguments).unwrap();
        {
            let memory = PoolReadMemory::new(&pool, &lease);
            match admit_analysis_run_comparison_args(&mut args, || Ok(path.clone()), budget, &memory) {
                Err(CommandError::Command(error)) => assert!(error.starts_with(code), "{code}: {error}"),
                other => panic!("expected {code}, got {other:?}"),
            }
        }
        drop(lease);
        assert_eq!(pool.pool_status().reserved_bytes, 0, "{code}");
    }
}

/// RT-U4: a recorded run whose stored result exceeds the displayable limit is
/// refused by its stored length before it is loaded, and stays unchanged.
#[test]
fn a_recorded_run_above_the_display_limit_is_refused_by_its_stored_length() {
    let path = store_with_project("too-large");
    let solved = model("Invented project");
    save_state(&path, &solved, "Basis", "000000000000000000000000000000c1", 1_000);
    let submitted = request(&solved, "Run");
    let stored_bytes = submitted.result_json.clone();
    let length = stored_bytes.len() as u64;
    let outcome = record(&path, submitted, T0).unwrap();
    let id = outcome.run.recording_id.clone();
    let read = |limit: u64| read_run_in_store_within(&path, PROJECT, &id, &UnmeteredReadMemory, limit).and_then(Metered::into_unreserved);
    // One byte under, and at, the limit: read as before.
    assert_eq!(read(length + 1).unwrap().result_json, stored_bytes);
    assert_eq!(read(length).unwrap().result_json, stored_bytes);
    // One byte over: refused by name with both sizes; nothing is loaded.
    let error = read(length - 1).map(|_| ()).unwrap_err();
    assert_eq!(error, format!(
        "STORED-RESULT-TOO-LARGE: recorded run `{id}`'s stored result is {}, above the desktop's display limit of {}, so it was not read: the per-station result detail of that run is too large for the desktop to display. The recorded run is kept unchanged. A run solved with fewer load cases and combinations at once, or a smaller model, stays within the limit.",
        crate::solve_jobs::display_size(length),
        crate::solve_jobs::display_size(length - 1)
    ));
    // The stored run is unchanged and still reads in full at the product limit.
    assert!(length < crate::solve_jobs::MAX_DISPLAYABLE_RESULT_BYTES);
    assert_eq!(read_run_in_store(&path, PROJECT, &id).unwrap().result_json, stored_bytes);
}
