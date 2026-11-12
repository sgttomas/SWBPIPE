//! The DEL-10-05 `export-results` witness basis (DEL-08-01 Design
//! `report-package-current-results.md` §2.7, fixture transition).
//!
//! The generator does three things:
//! - it solves a committed joint-free, pressure-free model once through the
//!   real headless producer;
//! - it builds the version 2.0.0 package request with the shared basis builder
//!   (`report_package/tests/common/package_basis.rs`);
//! - it checks the request end to end: the shared bindings, assembly unblocked,
//!   and the reader's read-back.
//!
//! With `OPENPIPESTRESS_UPDATE_DEL1005_WITNESS=1` it writes these files:
//! - `validation/witness/inputs/del1005_export_results_success_input.json`;
//! - `validation/witness/inputs/del1005_export_results_alias_overlay.json`,
//!   the alias input as leaf replacements over the success input (the two
//!   requests share every other byte, including the result document's rows);
//! - `fixtures/reports/invented/del1005_analysis_record.json`.
//!
//! Otherwise it asserts them byte for byte.
//! `validation/witness/inputs/generate_del1005_export_results_inputs.py`
//! derives the missing-payload input from the success input, and the
//! binding-mismatch and producer-blocked inputs on request (one edit each;
//! `tests/test_headless_runner_contract.py` makes the same edits in memory). The witness is versioned input for archive,
//! integrity and determinism checks. It is not an independent numerical or
//! physical-validity witness.

#[path = "../../../reporting/report_package/tests/common/package_basis.rs"]
mod package_basis;

use open_pipe_stress_headless_runner::{
    run_preview_model_value_with_rule_check, PrivacyContext, ProfessionalBoundary, Provenance, RedistributionStatus,
    Reference, RunnerOperation, RunnerRequest, TbdDecisions,
};
use open_pipe_stress_product_physics::{solver_component_name, solver_component_version};
use open_pipe_stress_report_package::wire::{assemble_wire_request, LinkedSolverIdentity, ReportPackageRequest};
use open_pipe_stress_report_package::read_report_package_container;
use package_basis::{analysis_record_projection, build, Inputs, Profile};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

const UPDATE: &str = "OPENPIPESTRESS_UPDATE_DEL1005_WITNESS";
/// The solved, joint-free, pressure-free preview-physics-2 model the
/// headless admission tests already solve (`preview_physics_admission.rs:74`).
/// The larger demo derivative gives a multi-megabyte document; this one keeps
/// the committed witness small.
const MODEL: &str = "fixtures/model_operations/precision_connected_ui_model.json";
const MODEL_ID: &str = "project:precision-connected-fixture";
const INPUT_MANIFEST: &str = "fixtures/reports/invented/del1005_input_manifest.json";
const ANALYSIS_RECORD: &str = "fixtures/reports/invented/del1005_analysis_record.json";
const SUCCESS_INPUT: &str = "validation/witness/inputs/del1005_export_results_success_input.json";
const ALIAS_OVERLAY: &str = "validation/witness/inputs/del1005_export_results_alias_overlay.json";

fn project(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").join(path)
}

fn read(path: &str) -> Value {
    serde_json::from_slice(&std::fs::read(project(path)).unwrap_or_else(|error| panic!("{path}: {error}")))
        .unwrap_or_else(|error| panic!("{path}: {error}"))
}

fn runner_request(model: &Value, model_ref: &str, operation: RunnerOperation, operation_ref: &str) -> RunnerRequest {
    RunnerRequest {
        request_id: "del1005-export-results-invented".into(),
        operation,
        operation_ref: Reference::new("api_operation", operation_ref),
        project_ref: Reference::new("project", MODEL_ID),
        model_ref: Reference::new("model", model_ref),
        unit_system_ref: Reference::new("unit_system", &format!("{MODEL_ID}:units")),
        load_basis_refs: model["load_cases"]
            .as_array()
            .unwrap()
            .iter()
            .map(|case| Reference::new("load_case", case["id"].as_str().unwrap()))
            .collect(),
        input_manifest_ref: Reference::new("input_manifest", "input-manifest:del1005-invented"),
        requested_outputs: vec!["result_envelope".into(), "audit_manifest".into(), "diagnostics".into()],
        privacy: PrivacyContext::local_first_public_metadata(),
        provenance: Provenance {
            source_name: "Invented DEL-10-05 export-results witness".into(),
            source_location: "core/runner/headless/tests/del1005_package_witness.rs".into(),
            source_license: "project invented".into(),
            contributor: "OpenPipeStress".into(),
            contributor_certification: "invented non-engineering example".into(),
            redistribution_status: RedistributionStatus::InventedNonEngineeringExample,
            review_status: "accepted".into(),
        },
        professional_boundary: ProfessionalBoundary::project_default(),
        tbd_decisions: TbdDecisions::d33_local_cli_policy(),
    }
}

fn linked() -> LinkedSolverIdentity<'static> {
    let name = solver_component_name();
    let version = solver_component_version();
    LinkedSolverIdentity {
        solver_name: name,
        solver_version: version,
        solver_build_ref: Box::leak(format!("{name}@{version}").into_boxed_str()),
    }
}

/// Solve once, then build the package basis for one request model reference.
fn basis(model_ref: &str) -> (Value, Value) {
    let model = read(MODEL);
    let solve = runner_request(&model, model_ref, RunnerOperation::Solve, "solve");
    let output =
        run_preview_model_value_with_rule_check(solve.clone(), json!({"model": model, "materials": []}), None).unwrap();
    let source = serde_json::to_value(output.mechanics_envelope.as_ref().expect("a mechanics carrier")).unwrap();
    assert_eq!(source["status"]["mechanics"], "MECHANICS_SOLVED");
    let document = output.result_envelope_document.expect("the producer derived its document");
    let inputs = Inputs {
        model: &model,
        source: &source,
        producer_document: &document,
        solve_request: &serde_json::to_value(&solve).unwrap(),
        input_manifest: &read(INPUT_MANIFEST),
        input_manifest_id: "input-manifest:del1005-invented",
        solver_name: solver_component_name(),
        solver_version: solver_component_version(),
        profile: Profile::Runner,
        run_override: None,
    };
    let built = build(&inputs).unwrap();
    let export = runner_request(&model, model_ref, RunnerOperation::ExportResults, "export_results");
    (json!({"request": export, "export_results": built.export_results}), built.analysis_record)
}

/// End to end: the shared bindings, unblocked assembly and read-back.
fn assert_succeeds(basis: &Value) {
    let request: ReportPackageRequest = serde_json::from_value(basis["export_results"].clone()).unwrap();
    let outcome = assemble_wire_request(request, &linked()).unwrap();
    assert!(!outcome.export_blocked, "{:?}", outcome.blocking_reasons);
    let read = read_report_package_container(&outcome.container_bytes).unwrap();
    assert!(!read.export_blocked);
    let member = read.members.iter().find(|member| member.role == "result_export").unwrap();
    // The member is the document's JCS form, byte for byte.
    let expected = open_pipe_stress_canonical_json::canonical_json(&basis["export_results"]["result_documents"][0]);
    assert!(member.bytes == expected.as_bytes(), "the result member is not the document's canonical form");
}

/// The leaf replacements that turn `base` into `variant`, as
/// `[{"pointer", "value"}]` in document order. The two must have the same
/// shape: the same object keys and array lengths everywhere.
fn leaf_overlay(base: &Value, variant: &Value, pointer: &str, out: &mut Vec<Value>) {
    match (base, variant) {
        (Value::Object(left), Value::Object(right)) => {
            assert!(left.keys().eq(right.keys()), "{pointer}: the alias changes the object shape");
            for (key, item) in left {
                let escaped = key.replace('~', "~0").replace('/', "~1");
                leaf_overlay(item, &right[key], &format!("{pointer}/{escaped}"), out);
            }
        }
        (Value::Array(left), Value::Array(right)) => {
            assert_eq!(left.len(), right.len(), "{pointer}: the alias changes an array length");
            for (index, (item, other)) in left.iter().zip(right).enumerate() {
                leaf_overlay(item, other, &format!("{pointer}/{index}"), out);
            }
        }
        (left, right) if left == right => {}
        (_, right) => out.push(json!({"pointer": pointer, "value": right})),
    }
}

/// Applies a leaf overlay, as the runner's alias test does.
fn apply_overlay(mut base: Value, overlay: &Value) -> Value {
    for entry in overlay.as_array().unwrap() {
        *base.pointer_mut(entry["pointer"].as_str().unwrap()).unwrap() = entry["value"].clone();
    }
    base
}

fn pretty(value: &Value) -> String {
    serde_json::to_string_pretty(value).unwrap() + "\n"
}

fn write_or_assert(path: &str, text: &str) {
    if std::env::var_os(UPDATE).is_some() {
        std::fs::write(project(path), text).unwrap();
    } else {
        let committed = std::fs::read_to_string(project(path)).unwrap_or_default();
        assert!(committed == text, "{path} is stale; regenerate it with {UPDATE}=1 and the input generator");
    }
}

#[test]
fn del1005_package_witness_basis_is_current_and_succeeds_end_to_end() {
    let (success, record) = basis(MODEL_ID);
    let (alias, alias_record) = basis("invented-model-alias");
    // The two requests differ only in the request alias; the run record is
    // the same.
    assert_eq!(record, alias_record);
    assert_eq!(
        alias["export_results"]["result_documents"][0]["result_envelope"]["reproducibility"]["source_origin_bindings"][0]
            ["request_model_ref"],
        json!({"ref_type": "model", "ref_id": "invented-model-alias"})
    );
    // The record's self-hash follows the committed rule.
    let entry = record["analysis_run"]["hashes"][0].clone();
    assert_eq!(entry["payload_scope"], "analysis_run_record");
    assert_eq!(
        entry["value"].as_str().unwrap(),
        open_pipe_stress_result_export::derivative::digest(&analysis_record_projection(&record)).unwrap()
    );
    let statuses = &success["export_results"]["result_documents"][0]["result_envelope"]["analysis_status"];
    let mut statuses: Vec<&str> = statuses.as_array().unwrap().iter().filter_map(Value::as_str).collect();
    statuses.sort();
    assert_eq!(statuses, ["HUMAN_REVIEW_REQUIRED", "MECHANICS_SOLVED", "RULE_INPUTS_INCOMPLETE"]);
    assert_succeeds(&success);
    assert_succeeds(&alias);
    let mut overlay = Vec::new();
    leaf_overlay(&success, &alias, "", &mut overlay);
    let overlay = Value::Array(overlay);
    assert_eq!(apply_overlay(success.clone(), &overlay), alias, "the overlay reproduces the alias input");
    write_or_assert(SUCCESS_INPUT, &pretty(&success));
    write_or_assert(ALIAS_OVERLAY, &pretty(&overlay));
    write_or_assert(ANALYSIS_RECORD, &pretty(&record));
}

/// The exact-route witness requests the desktop's vitest writes, and the
/// same-run substitute.
fn exact_witness_requests() -> Vec<(String, Value)> {
    let dir = "fixtures/reports/invented/exact_report_package_requests";
    let overlay = read(&format!("{dir}/pressure_2_straight_substitute.overlay.json"));
    let substitute = apply_overlay(read(&format!("{dir}/pressure_2_straight.json"))["request"].clone(), &overlay["replacements"]);
    let mut requests: Vec<(String, Value)> = ["pressure_2_straight", "pressure_2_load_reference_states", "pressure_2_connector", "pressure_2_realized_bend", "pressure_source_1"]
        .into_iter()
        .map(|name| (name.to_string(), read(&format!("{dir}/{name}.json"))["request"].clone()))
        .collect();
    requests.push(("pressure_2_straight_substitute".to_string(), substitute));
    requests
}

/// W-11 (DEL-08-01 Design `exact-route-report-sections.md` §5.1, §5.4): since
/// the owner opened the desktop's exact-route report package (2026-10-26), the
/// shared producer's public root admits pressure-2 and pressure-source-1 under
/// the desktop profile (the desktop's save calls it after its native source
/// binding), and still refuses them under the runner profile
/// (`REPORT-PACKAGE-EXACT-RUNNER-PROFILE`).
#[test]
fn the_shared_root_refuses_the_exact_witness_requests_under_the_runner_profile() {
    for (name, witness) in exact_witness_requests() {
        let solver = &witness["audit_manifest"]["solver_version"];
        let text = |key: &str| solver[key].as_str().unwrap().to_string();
        let (solver_name, solver_version, solver_build_ref) = (text("solver_name"), text("solver_version"), text("solver_build_ref"));
        let identity = LinkedSolverIdentity { solver_name: &solver_name, solver_version: &solver_version, solver_build_ref: &solver_build_ref };
        let mut request = witness.clone();
        request["export_profile_id"] = json!("runner_local_private_report_package_1");
        let request: ReportPackageRequest = serde_json::from_value(request).unwrap();
        let error = assemble_wire_request(request, &identity).map(drop).expect_err("refused");
        assert!(error.starts_with("REPORT-PACKAGE-EXACT-RUNNER-PROFILE"), "{name}: {error}");
    }
}

/// W-11 §5.4: the headless runner stays refused for exact-route results. Its
/// `export-results` command, given the committed runner witness input with
/// each exact witness request as its payload, under the desktop and the
/// runner profile, refuses with `REPORT-PACKAGE-ROUTE-NOT-ADMITTED` and
/// writes no package.
#[test]
fn the_runner_package_path_refuses_the_exact_witness_requests() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let base = read(SUCCESS_INPUT);
    for (name, witness) in exact_witness_requests() {
        for profile in ["desktop_local_private_report_package_1", "runner_local_private_report_package_1"] {
            let mut input = base.clone();
            input["export_results"] = witness.clone();
            input["export_results"]["export_profile_id"] = json!(profile);
            let mut child = Command::new(env!("CARGO_BIN_EXE_openpipestress-runner"))
                .args(["export-results", "--explicit-local-private-intent"])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap();
            child.stdin.take().unwrap().write_all(input.to_string().as_bytes()).unwrap();
            let out = child.wait_with_output().unwrap();
            let stdout = String::from_utf8(out.stdout).unwrap();
            assert_ne!(out.status.code(), Some(0), "{name} {profile}: {stdout}");
            let output: Value = serde_json::from_str(&stdout).unwrap_or_else(|error| panic!("{name} {profile}: {error}"));
            assert!(output.get("report_package").map_or(true, Value::is_null), "{name} {profile}: {stdout}");
            assert!(output.pointer("/payload/report_package").map_or(true, Value::is_null), "{name} {profile}: {stdout}");
            assert!(
                stdout.contains("REPORT-PACKAGE-ROUTE-NOT-ADMITTED: the headless runner packages no exact-route result"),
                "{name} {profile}: {stdout}"
            );
        }
    }
    // Codex review round 2 of #1648: a positional result document, envelope or
    // producer hides the exact identity from the runner's key read; the shared
    // producer's item 0 then refuses it (no identity), and a positional report
    // refuses at its first keyed binding. No package either way.
    let as_array = |value: &Value| Value::Array(value.as_object().unwrap().values().cloned().collect());
    let witness = read("fixtures/reports/invented/exact_report_package_requests/pressure_2_straight.json")["request"].clone();
    for (label, pointer) in [
        ("positional result document", "/result_documents/0"),
        ("positional result envelope", "/result_documents/0/result_envelope"),
        ("positional producer", "/result_documents/0/result_envelope/producer"),
        ("positional report", "/report"),
    ] {
        let mut request = witness.clone();
        let member = request.pointer_mut(pointer).unwrap();
        *member = as_array(member);
        let mut input = base.clone();
        input["export_results"] = request;
        let mut child = Command::new(env!("CARGO_BIN_EXE_openpipestress-runner"))
            .args(["export-results", "--explicit-local-private-intent"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child.stdin.take().unwrap().write_all(input.to_string().as_bytes()).unwrap();
        let out = child.wait_with_output().unwrap();
        let stdout = String::from_utf8(out.stdout).unwrap();
        assert_ne!(out.status.code(), Some(0), "{label}: {stdout}");
        let output: Value = serde_json::from_str(&stdout).unwrap_or_else(|error| panic!("{label}: {error}"));
        assert!(output.pointer("/payload/report_package").map_or(true, Value::is_null), "{label}: {stdout}");
    }
}
