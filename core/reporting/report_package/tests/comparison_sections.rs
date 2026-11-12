//! The comparison sections of a desktop report package's handoff record
//! (DEL-08-06 REQ-011, AC-009): each names its kind, the comparison it restates
//! and the hash of that comparison's record, refused by name otherwise; a
//! package valid before (no comparison section, or a within-run comparison
//! section without hash references) stays valid. The sections are the
//! reference assembler's, taken from the shared parity vector
//! `fixtures/reports/invented/state_comparison_handoff_comparison_parity.json`.
//! All data invented. Standard claim fence applies (claims taxonomy per
//! DEC-081).

mod common;
#[path = "common/package_basis.rs"]
mod package_basis;

use open_pipe_stress_report_package::bindings::BINDING_MISMATCH;
use open_pipe_stress_report_package::wire::{assemble_wire_request, LinkedSolverIdentity, ReportPackageRequest};
use open_pipe_stress_report_package::ReportPackageContainerOutcome;
use package_basis::{build, Inputs, Profile};
use serde_json::{json, Value};
use std::sync::OnceLock;

const MODEL: &str = include_str!("../../../../fixtures/model_operations/precision_connected_ui_model.json");
const INPUT_MANIFEST: &str = include_str!("../../../../fixtures/reports/invented/del1005_input_manifest.json");
const VECTOR: &str = include_str!("../../../../fixtures/reports/invented/state_comparison_handoff_comparison_parity.json");

fn runner() -> Value {
    common::basis()["export_results"].clone()
}

fn desktop() -> Value {
    static DESKTOP: OnceLock<Value> = OnceLock::new();
    DESKTOP
        .get_or_init(|| {
            let runner = runner();
            let model: Value = serde_json::from_str(MODEL).unwrap();
            let manifest: Value = serde_json::from_str(INPUT_MANIFEST).unwrap();
            let manifest_id = runner["audit_manifest"]["input_manifest_hash"]["payload_ref"].as_str().unwrap().to_string();
            let solver = &runner["audit_manifest"]["solver_version"];
            let inputs = Inputs {
                model: &model,
                source: &runner["source_results"][0],
                producer_document: &runner["result_documents"][0],
                solve_request: &Value::Null,
                input_manifest: &manifest,
                input_manifest_id: &manifest_id,
                solver_name: solver["solver_name"].as_str().unwrap(),
                solver_version: solver["solver_version"].as_str().unwrap(),
                profile: Profile::Desktop,
                run_override: None,
            };
            build(&inputs).unwrap().export_results
        })
        .clone()
}

fn linked() -> LinkedSolverIdentity<'static> {
    static SOLVER: OnceLock<(String, String, String)> = OnceLock::new();
    let (name, version, build) = SOLVER.get_or_init(|| {
        let solver = runner()["audit_manifest"]["solver_version"].clone();
        let text = |key: &str| solver[key].as_str().unwrap().to_string();
        (text("solver_name"), text("solver_version"), text("solver_build_ref"))
    });
    LinkedSolverIdentity { solver_name: name, solver_version: version, solver_build_ref: build }
}

fn assemble(request: &Value) -> Result<ReportPackageContainerOutcome, String> {
    let request: ReportPackageRequest =
        serde_json::from_value(request.clone()).map_err(|error| format!("TYPED-DECODE: {error}"))?;
    assemble_wire_request(request, &linked())
}

#[track_caller]
fn unblocked(request: &Value, case: &str) {
    let outcome = assemble(request).unwrap_or_else(|error| panic!("{case}: refused: {error}"));
    assert!(!outcome.export_blocked, "{case}: {:?}", outcome.blocking_reasons);
}

#[track_caller]
fn refused(request: &Value, field: &str, case: &str) {
    match assemble(request) {
        Ok(outcome) => panic!("{case}: assembled (blocked: {})", outcome.export_blocked),
        Err(error) => {
            assert!(error.starts_with(&format!("{BINDING_MISMATCH}:")), "{case}: expected {BINDING_MISMATCH}, got {error}");
            assert!(error.contains("comparison section") && error.ends_with(field), "{case}: not refused by name ({field}): {error}");
        }
    }
}

/// The reference assembler's comparison sections of both kinds.
fn vector_sections() -> Vec<Value> {
    let vector: Value = serde_json::from_str(VECTOR).unwrap();
    let case = vector["cases"].as_array().unwrap().iter().find(|case| case["name"] == "both_kinds").unwrap();
    case["expected"]["sections"]["comparison_sections"].as_array().unwrap().clone()
}

fn with_sections(sections: Vec<Value>) -> Value {
    let mut request = desktop();
    request["state_comparison_handoff_records"][0]["sections"]["comparison_sections"] = Value::Array(sections);
    request
}

fn of_kind(kind: &str) -> Value {
    vector_sections().into_iter().find(|section| section["section_kind"] == kind).unwrap()
}

#[test]
fn a_package_without_comparison_sections_stays_valid() {
    let request = desktop();
    assert_eq!(request["state_comparison_handoff_records"][0]["sections"]["comparison_sections"], json!([]));
    unblocked(&request, "no comparison section");
}

#[test]
fn saved_state_and_run_comparison_sections_are_admitted() {
    let sections = vector_sections();
    assert_eq!(sections.len(), 4);
    unblocked(&with_sections(sections), "both kinds");
}

#[test]
fn a_within_run_comparison_section_without_hash_references_stays_valid() {
    let mut within_run = of_kind("analysis_run_comparison");
    within_run.as_object_mut().unwrap().remove("hash_refs");
    within_run["comparison_ref"]["ref"] = json!("comparison:invented-within-run");
    unblocked(&with_sections(vec![within_run]), "within-run form");
}

#[test]
fn a_comparison_section_lacking_its_kind_reference_or_hash_is_refused_by_name() {
    type Edit = fn(&mut Value);
    let cases: [(&str, &str, Edit); 11] = [
        ("state", "section_kind", |s| { s.as_object_mut().unwrap().remove("section_kind"); }),
        ("run", "section_kind", |s| s["section_kind"] = json!("model_state_record")),
        ("state", "comparison_ref", |s| { s.as_object_mut().unwrap().remove("comparison_ref"); }),
        ("run", "comparison_ref", |s| s["comparison_ref"]["ref"] = json!(" ")),
        ("state", "comparison_ref", |s| s["comparison_ref"]["object_type"] = json!("AnalysisRunComparison")),
        ("state", "hash_refs", |s| { s.as_object_mut().unwrap().remove("hash_refs"); }),
        ("run", "hash_refs", |s| s["hash_refs"] = json!([])),
        ("state", "hash_refs", |s| s["hash_refs"][0]["algorithm"] = json!("md5")),
        ("run", "hash_refs", |s| s["hash_refs"][0]["value"] = json!("not-a-digest")),
        ("state", "hash_refs", |s| s["hash_refs"][0]["payload_ref"]["ref"] = json!("model-state-comparison:invented-other")),
        ("run", "hash_refs", |s| s["hash_refs"][0]["canonicalization"] = json!("python_json_dumps")),
    ];
    for (kind, field, edit) in cases {
        let mut section = of_kind(if kind == "state" { "model_state_comparison" } else { "analysis_run_comparison" });
        edit(&mut section);
        refused(&with_sections(vec![section]), field, &format!("{kind} {field}"));
    }
}
