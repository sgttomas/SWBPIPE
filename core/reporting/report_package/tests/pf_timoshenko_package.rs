//! The pressure-free Timoshenko report projection at packaging (DEL-05-03
//! Design `pf-timoshenko.md` §4, "Source binding on PF"): a report of a
//! `preview-physics-timoshenko-1` result carries the identity's limitation 0 and
//! the result's diagnostics, the arc diagnostic among them; removing or altering
//! the limitation, or the diagnostic's text or references, while keeping the
//! result, is refused. Euler–Bernoulli reports carry no such entry.
//!
//! The Timoshenko source is a TRANSFORMED STAND-IN until the product publishes
//! one: the committed DEL-10-05 witness source with the identity, the profile and
//! limitation 0 swapped, plus a hand-built arc diagnostic, from which the shared
//! basis builder derives the document. All data invented. Standard claim fence
//! applies (claims taxonomy per DEC-081).

mod common;
#[path = "common/package_basis.rs"]
mod package_basis;

use open_pipe_stress_report_package::bindings::{expected_report_diagnostics, SOURCE_INVALID};
use open_pipe_stress_report_package::document_form::ADMITTED_SEMANTIC_CONTRACTS;
use open_pipe_stress_report_package::pf_timoshenko::{PF_TIMOSHENKO_SECTIONS_MISMATCH, REALIZED_ARCS_DIAGNOSTIC};
use open_pipe_stress_report_package::wire::{assemble_wire_request, LinkedSolverIdentity, ReportPackageRequest};
use open_pipe_stress_report_package::ReportPackageContainerOutcome;
use open_pipe_stress_result_export::semantic_contract::{
    preview_physics_timoshenko_contract, PREVIEW_PHYSICS_ID, PREVIEW_PHYSICS_TIMOSHENKO_ID, PREVIEW_PHYSICS_TIMOSHENKO_PROFILE,
};
use package_basis::{build, Inputs, Profile};
use serde_json::{json, Value};

const MODEL: &str = include_str!("../../../../fixtures/model_operations/precision_connected_ui_model.json");
const INPUT_MANIFEST: &str = include_str!("../../../../fixtures/reports/invented/del1005_input_manifest.json");
const ARC_MESSAGE: &str = "1 straight member with shear deformation, 1 realized arc without: pipe:fixture-span";

fn runner() -> Value {
    common::basis()["export_results"].clone()
}

/// The transformed stand-in: the witness source under the Timoshenko identity, with an arc diagnostic.
fn timoshenko(source: &mut Value) {
    source["producer"]["semantic_contract_id"] = json!(PREVIEW_PHYSICS_TIMOSHENKO_ID);
    source["formulation_basis"]["profile_id"] = json!(PREVIEW_PHYSICS_TIMOSHENKO_PROFILE);
    source["formulation_basis"]["limitations"] = preview_physics_timoshenko_contract()["supported_profile_limitations"].clone();
    source["diagnostics"].as_array_mut().unwrap().push(json!({
        "id": "diagnostic:preview-physics:pf-timoshenko:realized-arcs", "code": REALIZED_ARCS_DIAGNOSTIC,
        "severity": "info", "message": ARC_MESSAGE, "source": "core/product_physics", "affected_refs": ["pipe:fixture-span"]}));
}

/// The desktop basis over `edit_source`'s source, its report carrying the
/// technical-preview entry and the source's diagnostics; with `projection`, the
/// Timoshenko limitation entry too.
fn request(edit_source: impl FnOnce(&mut Value), projection: bool) -> Value {
    let runner = runner();
    let model: Value = serde_json::from_str(MODEL).unwrap();
    let manifest: Value = serde_json::from_str(INPUT_MANIFEST).unwrap();
    let manifest_id = runner["audit_manifest"]["input_manifest_hash"]["payload_ref"].as_str().unwrap().to_string();
    let solver = &runner["audit_manifest"]["solver_version"];
    let mut source = runner["source_results"][0].clone();
    edit_source(&mut source);
    let inputs = Inputs {
        model: &model,
        source: &source,
        producer_document: &runner["result_documents"][0],
        solve_request: &Value::Null,
        input_manifest: &manifest,
        input_manifest_id: &manifest_id,
        solver_name: solver["solver_name"].as_str().unwrap(),
        solver_version: solver["solver_version"].as_str().unwrap(),
        profile: Profile::Desktop,
        run_override: None,
    };
    let mut request = build(&inputs).unwrap().export_results;
    let source = request["source_results"][0].clone();
    let run = source["run_id"].as_str().unwrap().to_string();
    let sections = &mut request["report"]["report_sections"];
    let provenance = sections["provenance_notes"][0].clone();
    let mut technical = sections["limitations"][0].clone();
    technical["limitation_id"] = json!(format!("limitation:{run}:technical-preview"));
    let mut limitations = vec![technical.clone()];
    if projection {
        limitations.push(json!({
            "limitation_id": format!("limitation:{run}:pf-timoshenko:1"),
            "source": {"ref_type": "semantic_contract", "ref_id": PREVIEW_PHYSICS_TIMOSHENKO_ID},
            "affected_scope": {"ref_type": "analysis_run", "ref_id": run},
            "statement": source["formulation_basis"]["limitations"][0],
            "effect": technical["effect"], "provenance": technical["provenance"]}));
    }
    sections["limitations"] = Value::Array(limitations);
    sections["diagnostics"] = Value::Array(
        expected_report_diagnostics(&source).unwrap().into_iter().map(|mut entry| { entry["provenance"] = provenance.clone(); entry }).collect());
    request
}

fn linked() -> LinkedSolverIdentity<'static> {
    let solver = runner()["audit_manifest"]["solver_version"].clone();
    let leak = |key: &str| -> &'static str { Box::leak(solver[key].as_str().unwrap().to_string().into_boxed_str()) };
    LinkedSolverIdentity { solver_name: leak("solver_name"), solver_version: leak("solver_version"), solver_build_ref: leak("solver_build_ref") }
}

fn assemble(request: &Value) -> Result<ReportPackageContainerOutcome, String> {
    let request: ReportPackageRequest = serde_json::from_value(request.clone()).map_err(|error| format!("TYPED-DECODE: {error}"))?;
    assemble_wire_request(request, &linked())
}

#[track_caller]
fn refused(request: &Value, code: &str, case: &str) {
    match assemble(request) {
        Ok(outcome) => panic!("{case}: assembled (blocked: {})", outcome.export_blocked),
        Err(error) => assert!(error.starts_with(&format!("{code}:")), "{case}: expected {code}, got {error}"),
    }
}

fn edited(request: &Value, edit: impl FnOnce(&mut Value)) -> Value {
    let mut request = request.clone();
    edit(&mut request["report"]["report_sections"]);
    request
}

fn arc_index(request: &Value) -> usize {
    request["report"]["report_sections"]["diagnostics"].as_array().unwrap().iter().position(|d| d["code"] == REALIZED_ARCS_DIAGNOSTIC).unwrap()
}

#[test]
fn both_pressure_free_identities_are_admitted() {
    assert!(ADMITTED_SEMANTIC_CONTRACTS.contains(&PREVIEW_PHYSICS_ID));
    assert!(ADMITTED_SEMANTIC_CONTRACTS.contains(&PREVIEW_PHYSICS_TIMOSHENKO_ID));
}

#[test]
fn a_timoshenko_report_with_its_projection_packages() {
    let request = request(timoshenko, true);
    let outcome = assemble(&request).unwrap_or_else(|error| panic!("refused: {error}"));
    assert!(!outcome.export_blocked, "{:?}", outcome.blocking_reasons);
    let diagnostics = &request["report"]["report_sections"]["diagnostics"];
    assert_eq!(diagnostics[arc_index(&request)]["message"], ARC_MESSAGE);
    assert_eq!(diagnostics[arc_index(&request)]["affected_object"]["ref_id"], "pipe:fixture-span");
}

#[test]
fn removing_or_altering_the_limitation_is_refused() {
    let request = request(timoshenko, true);
    let removed = edited(&request, |sections| { sections["limitations"].as_array_mut().unwrap().pop(); });
    refused(&removed, PF_TIMOSHENKO_SECTIONS_MISMATCH, "limitation removed");
    let altered = edited(&request, |sections| {
        sections["limitations"][1]["statement"] = json!("Small-displacement, linear-elastic Euler-Bernoulli frame preview; numerical integrity does not establish physical correctness.");
    });
    refused(&altered, PF_TIMOSHENKO_SECTIONS_MISMATCH, "limitation altered");
    let resourced = edited(&request, |sections| { sections["limitations"][1]["source"]["ref_id"] = json!(PREVIEW_PHYSICS_ID); });
    refused(&resourced, PF_TIMOSHENKO_SECTIONS_MISMATCH, "limitation source altered");
    let extra = edited(&request, |sections| { sections["limitations"][1]["note"] = json!("x"); });
    refused(&extra, PF_TIMOSHENKO_SECTIONS_MISMATCH, "limitation extra member");
    // A report without the projection, for the same Timoshenko result.
    refused(&request_without_projection(), PF_TIMOSHENKO_SECTIONS_MISMATCH, "projection absent");
}

fn request_without_projection() -> Value {
    request(timoshenko, false)
}

#[test]
fn removing_or_altering_the_arc_diagnostic_is_refused() {
    let request = request(timoshenko, true);
    let index = arc_index(&request);
    let removed = edited(&request, |sections| { sections["diagnostics"].as_array_mut().unwrap().remove(index); });
    refused(&removed, PF_TIMOSHENKO_SECTIONS_MISMATCH, "diagnostic removed");
    let text = edited(&request, |sections| { sections["diagnostics"][index]["message"] = json!("no realized arcs"); });
    refused(&text, PF_TIMOSHENKO_SECTIONS_MISMATCH, "diagnostic text altered");
    let refs = edited(&request, |sections| { sections["diagnostics"][index]["affected_object"]["ref_id"] = json!("pipe:other"); });
    refused(&refs, PF_TIMOSHENKO_SECTIONS_MISMATCH, "diagnostic references altered");
}

#[test]
fn euler_bernoulli_reports_carry_no_projection_entry() {
    // The preview-physics-2 witness with the Timoshenko entry added is refused.
    let mut request = request(|_| {}, false);
    let run = request["source_results"][0]["run_id"].as_str().unwrap().to_string();
    let sections = &mut request["report"]["report_sections"];
    let mut entry = sections["limitations"][0].clone();
    entry["limitation_id"] = json!(format!("limitation:{run}:pf-timoshenko:1"));
    sections["limitations"].as_array_mut().unwrap().push(entry);
    refused(&request, PF_TIMOSHENKO_SECTIONS_MISMATCH, "entry on an Euler-Bernoulli report");
    // And the Euler-Bernoulli report itself packages unchanged.
    let plain = request_eb();
    let outcome = assemble(&plain).unwrap_or_else(|error| panic!("refused: {error}"));
    assert!(!outcome.export_blocked);
}

fn request_eb() -> Value {
    request(|_| {}, false)
}

#[test]
fn a_mixed_identity_source_is_refused_at_its_reading() {
    // Timoshenko identity with Euler-Bernoulli limitations: the source reader refuses it.
    let mut request = request(timoshenko, true);
    request["source_results"][0]["formulation_basis"]["limitations"][0] =
        json!("Small-displacement, linear-elastic Euler-Bernoulli frame preview; numerical integrity does not establish physical correctness.");
    refused(&request, SOURCE_INVALID, "mixed source");
}
