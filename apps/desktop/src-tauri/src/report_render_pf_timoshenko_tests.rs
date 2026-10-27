//! The native render's binding of the pressure-free Timoshenko report
//! projection (DEL-05-03 Design `pf-timoshenko.md` §4, "Source binding on PF"):
//! an input with the top-level `pf_timoshenko` section takes the exact route's
//! custody steps on its admission, then the re-derivation of its Diagnostics and
//! its limitation 0 entry from the delivered result. Removing or altering the
//! limitation, or the arc diagnostic's text or references, while keeping the
//! result (its texts resubmitted), is refused; Euler–Bernoulli inputs render as
//! before.
//!
//! The Timoshenko source is a TRANSFORMED STAND-IN until the product publishes
//! one: the committed DEL-10-05 witness source with the identity, the profile
//! and limitation 0 swapped, plus a hand-built arc diagnostic; its model selects
//! `timoshenko_annulus_energy_v1`. Every id is invented.

use crate::command_ingress::test_support::admit_json;
use crate::command_ingress::{ArgsDecoding, CommandError};
use crate::exact_source_binding::{SECTIONS_MISMATCH, STANDING_NOT_ELIGIBLE};
use crate::memory_pool::test_host::ScriptedHost;
use crate::report_package_save::test_support::{desktop_request_from, input_manifest, model, witness_source};
use crate::report_render_exact::PF_TIMOSHENKO_SECTION;
use crate::solve_jobs::test_support::delivered_in_custody;
use crate::solve_jobs::SolveJobRegistry;
use open_pipe_stress_operation_applier::{canonical_json, sha256_hex};
use open_pipe_stress_report_package::bindings::expected_report_diagnostics;
use open_pipe_stress_report_package::pf_timoshenko::REALIZED_ARCS_DIAGNOSTIC;
use open_pipe_stress_result_export::derivative::digest;
use open_pipe_stress_result_export::semantic_contract::{
    preview_physics_timoshenko_contract, PREVIEW_PHYSICS_TIMOSHENKO_ID, PREVIEW_PHYSICS_TIMOSHENKO_PROFILE,
};
use serde_json::{json, Value};
use std::sync::Arc;

const ARC_MESSAGE: &str = "1 straight member with shear deformation, 1 realized arc without: pipe:fixture-span";
const MODE: &str = "sparse_interactive";

struct Case {
    registry: SolveJobRegistry,
    _gate: Arc<crate::solve_worker::test_support::Gate>,
    input: Value,
    result: String,
    intake: String,
    manifest: String,
}

fn timoshenko_source() -> Value {
    let mut source = witness_source();
    source["producer"]["semantic_contract_id"] = json!(PREVIEW_PHYSICS_TIMOSHENKO_ID);
    source["formulation_basis"]["profile_id"] = json!(PREVIEW_PHYSICS_TIMOSHENKO_PROFILE);
    source["formulation_basis"]["limitations"] = preview_physics_timoshenko_contract()["supported_profile_limitations"].clone();
    // The producer's own diagnostic source, from the witness's product diagnostics (a path
    // literal here would read as a desktop reference to the whole product crate).
    let producer = source["diagnostics"][0]["source"].clone();
    source["diagnostics"].as_array_mut().unwrap().push(json!({
        "id": "diagnostic:preview-physics:pf-timoshenko:realized-arcs", "code": REALIZED_ARCS_DIAGNOSTIC,
        "severity": "info", "message": ARC_MESSAGE, "source": producer, "affected_refs": ["pipe:fixture-span"]}));
    source
}

/// A coherent Timoshenko render input over a live custody job: the desktop
/// request's report with the technical-preview and limitation 0 entries, the
/// source's diagnostics and the `pf_timoshenko` section.
fn case(shear_model: &str) -> Case {
    let mut model = model();
    model["model_settings"] = json!({"shear_model": shear_model});
    let manifest = input_manifest(&model, json!([]));
    let source = timoshenko_source();
    let request = desktop_request_from(&model, &manifest, &source);
    let result = serde_json::to_string(&source).unwrap();
    let intake = canonical_json(&model);
    let (registry, gate, job) = delivered_in_custody(ScriptedHost::normal(), intake.as_bytes(), Some(MODE), &result);
    let run = source["run_id"].as_str().unwrap().to_string();
    let document = &request["result_documents"][0]["result_envelope"];
    let mut input = request["report"].clone();
    let sections = &mut input["report_sections"];
    let provenance = sections["provenance_notes"][0].clone();
    let mut technical = sections["limitations"][0].clone();
    technical["limitation_id"] = json!(format!("limitation:{run}:technical-preview"));
    sections["limitations"] = json!([technical.clone(), {
        "limitation_id": format!("limitation:{run}:pf-timoshenko:1"),
        "source": {"ref_type": "semantic_contract", "ref_id": PREVIEW_PHYSICS_TIMOSHENKO_ID},
        "affected_scope": {"ref_type": "analysis_run", "ref_id": run},
        "statement": source["formulation_basis"]["limitations"][0],
        "effect": technical["effect"], "provenance": technical["provenance"]}]);
    sections["diagnostics"] = Value::Array(
        expected_report_diagnostics(&source).unwrap().into_iter().map(|mut entry| { entry["provenance"] = provenance.clone(); entry }).collect());
    let manifest_text = canonical_json(&manifest);
    input[PF_TIMOSHENKO_SECTION] = json!({
        "semantic_contract_id": PREVIEW_PHYSICS_TIMOSHENKO_ID,
        "admission": {
            "semantic_contract_id": PREVIEW_PHYSICS_TIMOSHENKO_ID,
            "run_ref": run,
            "result_envelope_id": document["envelope_id"],
            "received_result_sha256": digest(&source).unwrap(),
            "model_sha256": digest(&model).unwrap(),
            "input_manifest_ref": document["reproducibility"]["audit_manifest_ref"]["ref_id"],
            "input_manifest_sha256": sha256_hex(&manifest_text),
            "derivative_sha256": document["reproducibility"]["derivative_hash"]["value"],
            "native_job_id": job,
            "received_bytes_sha256": sha256_hex(&result),
        }});
    Case { registry, _gate: gate, input, result, intake, manifest: manifest_text }
}

fn render(case: &Case, input: &Value, texts: bool) -> Result<Value, CommandError> {
    let mut args = json!({"input": input});
    if texts {
        args["exactResultText"] = json!(case.result);
        args["exactIntakeText"] = json!(case.intake);
        args["inputManifestText"] = json!(case.manifest);
    }
    let (mut decoded, mut lease) = admit_json(&case.registry, "render_calculation_report", ArgsDecoding::Moved, &args).unwrap();
    crate::render_calculation_report_args(&mut decoded, &case.registry, &mut lease)
}

#[track_caller]
fn refused(case: &Case, input: &Value, texts: bool, code: &str, what: &str) {
    match render(case, input, texts) {
        Ok(rendered) => panic!("{what}: rendered (blocked: {})", rendered["export_blocked"]),
        Err(CommandError::Command(message)) => assert!(message.starts_with(code), "{what}: expected {code}, got {message}"),
        Err(other) => panic!("{what}: {other:?}"),
    }
}

fn arc_index(input: &Value) -> usize {
    input["report_sections"]["diagnostics"].as_array().unwrap().iter().position(|d| d["code"] == REALIZED_ARCS_DIAGNOSTIC).unwrap()
}

#[test]
fn a_coherent_timoshenko_input_binds_and_renders_its_projection() {
    let case = case("timoshenko_annulus_energy_v1");
    let rendered = render(&case, &case.input, true).unwrap_or_else(|error| panic!("{error:?}"));
    assert_eq!(rendered["export_blocked"], json!(false), "{}", rendered["blocking_reasons"]);
    let html = rendered["html"].as_str().unwrap();
    let limitation = open_pipe_stress_report_renderer::escape_html(preview_physics_timoshenko_contract()["supported_profile_limitations"][0].as_str().unwrap());
    assert!(html.contains(&limitation), "the identity's limitation 0 is rendered");
    assert!(html.contains(&open_pipe_stress_report_renderer::escape_html(ARC_MESSAGE)) && html.contains(REALIZED_ARCS_DIAGNOSTIC));
}

#[test]
fn removing_or_altering_the_limitation_keeping_the_result_is_refused() {
    let case = case("timoshenko_annulus_energy_v1");
    let mut removed = case.input.clone();
    removed["report_sections"]["limitations"].as_array_mut().unwrap().pop();
    refused(&case, &removed, true, SECTIONS_MISMATCH, "limitation removed");
    let mut altered = case.input.clone();
    altered["report_sections"]["limitations"][1]["statement"] = json!("Small-displacement, linear-elastic Euler-Bernoulli frame preview; numerical integrity does not establish physical correctness.");
    refused(&case, &altered, true, SECTIONS_MISMATCH, "limitation altered");
    // The section removed with the texts kept, or the entry kept without the section.
    let mut unsectioned = case.input.clone();
    unsectioned.as_object_mut().unwrap().remove(PF_TIMOSHENKO_SECTION);
    refused(&case, &unsectioned, true, SECTIONS_MISMATCH, "section removed, texts kept");
    refused(&case, &unsectioned, false, SECTIONS_MISMATCH, "entry without the section");
    // Both sections.
    let mut both = case.input.clone();
    both["exact_route"] = json!({"semantic_contract_id": PREVIEW_PHYSICS_TIMOSHENKO_ID});
    refused(&case, &both, true, "REPORT-RENDER-ROUTE-NOT-ADMITTED", "with an exact_route");
    // Without the texts the section's binding refuses.
    refused(&case, &case.input, false, "REPORT-EXACT-SOURCE-NOT-DELIVERED", "texts absent");
}

#[test]
fn removing_or_altering_the_arc_diagnostic_keeping_the_result_is_refused() {
    let case = case("timoshenko_annulus_energy_v1");
    let index = arc_index(&case.input);
    let mut removed = case.input.clone();
    removed["report_sections"]["diagnostics"].as_array_mut().unwrap().remove(index);
    refused(&case, &removed, true, SECTIONS_MISMATCH, "diagnostic removed");
    let mut text = case.input.clone();
    text["report_sections"]["diagnostics"][index]["message"] = json!("no realized arcs");
    refused(&case, &text, true, SECTIONS_MISMATCH, "diagnostic text altered");
    let mut refs = case.input.clone();
    refs["report_sections"]["diagnostics"][index]["affected_object"]["ref_id"] = json!("pipe:other");
    refused(&case, &refs, true, SECTIONS_MISMATCH, "diagnostic references altered");
}

#[test]
fn a_crossed_model_is_not_eligible_at_the_native_binding() {
    // The same Timoshenko result delivered from a model that does not select Timoshenko.
    let case = case("euler_bernoulli");
    refused(&case, &case.input, true, STANDING_NOT_ELIGIBLE, "crossed pair");
}
