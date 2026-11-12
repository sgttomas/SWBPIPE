//! The render from custody (`report_native_fill`), proved against the reports
//! the webview built. For every committed exact-route request
//! (`fixtures/reports/invented/exact_report_package_requests/`, the substitute
//! overlay included) and every committed user-released exact request
//! (`fixtures/reports/invented/user_release_exact/`), the committed report
//! input, stripped of the members the host fills, is the frame; the raw result
//! is held in native custody under a test registry job admitted from the
//! committed intake text; the host fills the frame from the held result and
//! requires
//! - the filled input canonically equal to the committed input (its
//!   `native_job_id` set to the live job's, as the existing binding tests do),
//!   the first difference named otherwise;
//! - the same render: the new command's response equal to the existing
//!   command's for the committed input with its texts resubmitted (HTML,
//!   `sha256_hex`, the print view and every other member), or the same refusal.
//!
//! No pressure-free Timoshenko report input is committed; its fill is proved
//! against the existing command on the transformed stand-in of
//! `report_render_pf_timoshenko_tests.rs`. Refusals: an unknown job, a
//! webview-custody job, a model text that is not the custody's, each filled
//! member smuggled in a frame, a result diagnostic or an extra limitation in
//! a frame, rule-pack exclusions, a result the fill does not serve, an exact
//! render under another export context and a withheld render. Every id is
//! invented.

use crate::command_ingress::test_support::admit_json;
use crate::command_ingress::{ArgsDecoding, CommandError};
use crate::exact_source_binding::{CUSTODY_RELEASED, SOURCE_NOT_DELIVERED};
use crate::memory_pool::test_host::ScriptedHost;
use crate::native_persistence::tests::held;
use crate::native_verdict::tests::gated;
use crate::report_native_fill::{FILL_NOT_SERVED, FRAME_MEMBER_DERIVED};
use crate::report_render_exact::{PF_TIMOSHENKO_SECTION, RENDER_EXACT_CONTEXT_NOT_LOCAL_PRIVATE};
use crate::solve_jobs::{ResultCustody, SolveJobRegistry};
use crate::solve_worker::test_support::Gate;
use open_pipe_stress_operation_applier::{canonical_json, sha256_hex};
use open_pipe_stress_report_package::bindings::{expected_report_diagnostics, RESULT_DIAGNOSTIC_REMEDIATION};
use open_pipe_stress_report_package::user_release::first_difference;
use serde_json::{json, Value};
use std::path::Path;
use std::sync::Arc;

const REQUESTS: &str = "fixtures/reports/invented/exact_report_package_requests";
const SHAPES: [&str; 5] = [
    "pressure_2_straight",
    "pressure_2_load_reference_states",
    "pressure_2_connector",
    "pressure_2_realized_bend",
    "pressure_source_1",
];
const USER_RELEASE_SHAPES: [&str; 2] = ["pressure_2_straight", "pressure_source_1"];

fn project(path: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").join(path)
}

fn read_json(path: &str) -> Value {
    serde_json::from_str(&std::fs::read_to_string(project(path)).unwrap()).unwrap()
}

/// One committed reference: the report input the webview built, its raw
/// result text, the intake text, the input manifest, the mode and the
/// session's rule-check aggregate.
struct Reference {
    name: String,
    report: Value,
    result: String,
    intake: String,
    manifest: Value,
    mode: String,
    aggregate: Value,
}

fn request_reference(shape: &str) -> Reference {
    let file = read_json(&format!("{REQUESTS}/{shape}.json"));
    Reference {
        name: shape.to_string(),
        report: file["request"]["report"].clone(),
        result: std::fs::read_to_string(project(file["raw_path"].as_str().unwrap())).unwrap(),
        intake: file["exact_intake_text"].as_str().unwrap().to_string(),
        manifest: file["input_manifest"].clone(),
        mode: file["solver_mode"].as_str().unwrap().to_string(),
        aggregate: file["request"]["rule_check_aggregate"].clone(),
    }
}

/// The substitute: the straight request with the overlay's leaf replacements;
/// its held result is its own (substituted) source result, so its received
/// bytes digest is that text's.
fn substitute_reference() -> Reference {
    let overlay = read_json(&format!("{REQUESTS}/pressure_2_straight_substitute.overlay.json"));
    assert_eq!(overlay["base"], "pressure_2_straight.json");
    let file = read_json(&format!("{REQUESTS}/pressure_2_straight.json"));
    let mut request = file["request"].clone();
    for entry in overlay["replacements"].as_array().unwrap() {
        *request.pointer_mut(entry["pointer"].as_str().unwrap()).unwrap() = entry["value"].clone();
    }
    let result = serde_json::to_string(&request["source_results"][0]).unwrap();
    let mut report = request["report"].clone();
    report["exact_route"]["admission"]["received_bytes_sha256"] = json!(sha256_hex(&result));
    Reference {
        name: "pressure_2_straight_substitute".to_string(),
        report,
        result,
        intake: file["exact_intake_text"].as_str().unwrap().to_string(),
        manifest: file["input_manifest"].clone(),
        mode: file["solver_mode"].as_str().unwrap().to_string(),
        aggregate: request["rule_check_aggregate"].clone(),
    }
}

fn user_release_reference(shape: &str) -> Reference {
    let file = read_json(&format!("fixtures/reports/invented/user_release_exact/{shape}.json"));
    Reference {
        name: format!("user_release_exact/{shape}"),
        report: file["request"]["report"].clone(),
        result: file["exact_result_text"].as_str().unwrap().to_string(),
        intake: file["exact_intake_text"].as_str().unwrap().to_string(),
        manifest: file["input_manifest"].clone(),
        mode: file["solver_mode"].as_str().unwrap().to_string(),
        aggregate: file["request"]["rule_check_aggregate"].clone(),
    }
}

fn every_reference() -> Vec<Reference> {
    let mut all: Vec<Reference> = SHAPES.iter().map(|shape| request_reference(shape)).collect();
    all.push(substitute_reference());
    all.extend(USER_RELEASE_SHAPES.iter().map(|shape| user_release_reference(shape)));
    all
}

/// A native-custody job holding the reference's raw result.
struct Live {
    registry: SolveJobRegistry,
    _gate: Arc<Gate>,
    job: String,
}

fn live_job(reference: &Reference) -> Live {
    let (registry, gate) = gated(&ScriptedHost::normal());
    let job = held(&registry, &gate, &reference.result, &reference.intake, &reference.mode, ResultCustody::Native);
    Live { registry, _gate: gate, job }
}

/// The section key the report carries.
fn section_key(report: &Value) -> &'static str {
    if report.get(PF_TIMOSHENKO_SECTION).is_some() { PF_TIMOSHENKO_SECTION } else { "exact_route" }
}

/// The committed report bound to the live job.
fn bound(report: &Value, job: &str) -> Value {
    let mut report = report.clone();
    let key = section_key(&report);
    report[key]["admission"]["native_job_id"] = json!(job);
    report
}

/// The frame: `report` without the members the host fills, its first
/// `result_diagnostics` Diagnostics (the result's) and every limitation entry
/// after the technical-preview entry.
fn frame_of(report: &Value, result_diagnostics: usize) -> Value {
    let mut frame = report.clone();
    let root = frame.as_object_mut().unwrap();
    for key in ["result_rows", "model_input_tables", "exact_route", PF_TIMOSHENKO_SECTION] {
        root.remove(key);
    }
    let calculation = frame["calculation_report"].as_object_mut().unwrap();
    calculation.remove("result_export_refs").expect("result_export_refs");
    calculation.remove("report_section_refs").expect("report_section_refs");
    frame["calculation_report"]["model_input_summary"].as_object_mut().unwrap().remove("unit_display_summary").expect("unit_display_summary");
    frame["report_context"].as_object_mut().unwrap().remove("result_coordinate_systems").expect("result_coordinate_systems");
    let diagnostics = frame["report_sections"]["diagnostics"].as_array_mut().unwrap();
    let removed: Vec<Value> = diagnostics.drain(..result_diagnostics).collect();
    assert!(removed.iter().all(|entry| entry["remediation"] == RESULT_DIAGNOSTIC_REMEDIATION));
    frame["report_sections"]["limitations"].as_array_mut().unwrap().truncate(1);
    frame
}

fn source_diagnostic_count(result: &str) -> usize {
    let source: Value = serde_json::from_str(result).unwrap();
    source["diagnostics"].as_array().unwrap().len()
}

fn custody_args(job: &str, frame: &Value, reference: &Reference) -> Value {
    json!({
        "jobId": job,
        "frame": frame,
        "intakeText": reference.intake,
        "inputManifestText": canonical_json(&reference.manifest),
        "ruleCheckAggregate": reference.aggregate,
        "rulePackExclusions": [],
    })
}

/// The fill alone (the command's steps before the admission).
fn fill_only(live: &Live, args: &Value) -> Result<Value, CommandError> {
    let (mut decoded, mut lease) = admit_json(&live.registry, "render_calculation_report_from_custody", ArgsDecoding::Moved, args).unwrap();
    let request = crate::report_native_fill::CustodyRenderRequest::decode(&mut decoded)?;
    crate::report_native_fill::fill_from_custody(request, &live.registry, &mut lease).map(|filled| filled.input)
}

/// The new command through its production root.
fn render_from_custody(live: &Live, args: &Value) -> Result<Value, CommandError> {
    let (mut decoded, mut lease) = admit_json(&live.registry, "render_calculation_report_from_custody", ArgsDecoding::Moved, args).unwrap();
    crate::render_calculation_report_from_custody_args(&mut decoded, &live.registry, &mut lease)
}

/// The existing command through its production root, the texts resubmitted.
fn render_today(live: &Live, input: &Value, reference: &Reference) -> Result<Value, CommandError> {
    let args = json!({
        "input": input,
        "exactResultText": reference.result,
        "exactIntakeText": reference.intake,
        "inputManifestText": canonical_json(&reference.manifest),
    });
    let (mut decoded, mut lease) = admit_json(&live.registry, "render_calculation_report", ArgsDecoding::Moved, &args).unwrap();
    crate::render_calculation_report_args(&mut decoded, &live.registry, &mut lease)
}

fn message(error: CommandError) -> String {
    match error {
        CommandError::Command(message) => message,
        other => format!("{other:?}"),
    }
}

fn outcome_text(outcome: Result<Value, CommandError>) -> Result<String, String> {
    outcome.map(|value| serde_json::to_string(&value).unwrap()).map_err(message)
}

/// Every committed request: the fill equals the committed input, and the new
/// command renders what the existing command renders for it.
#[test]
fn the_fill_from_custody_equals_every_committed_report_input_and_renders_alike() {
    let mut summary = Vec::new();
    for reference in every_reference() {
        let name = &reference.name;
        let live = live_job(&reference);
        let committed = bound(&reference.report, &live.job);
        let frame = frame_of(&reference.report, source_diagnostic_count(&reference.result));
        let args = custody_args(&live.job, &frame, &reference);
        let filled = fill_only(&live, &args).unwrap_or_else(|error| panic!("{name}: the fill refused: {}", message(error)));
        if let Some(pointer) = first_difference(&committed, &filled) {
            panic!("{name}: the filled input differs from the committed input at {pointer}: committed {:?}, filled {:?}", committed.pointer(&pointer), filled.pointer(&pointer));
        }
        assert_eq!(canonical_json(&filled), canonical_json(&committed), "{name}");
        let today = outcome_text(render_today(&live, &committed, &reference));
        let from_custody = outcome_text(render_from_custody(&live, &args));
        assert_eq!(from_custody, today, "{name}: the render from custody is not the existing render");
        match &today {
            Ok(text) => {
                let rendered: Value = serde_json::from_str(text).unwrap();
                assert_eq!(rendered["export_blocked"], json!(false), "{name}: {}", rendered["blocking_reasons"]);
                assert_eq!(rendered["sha256_hex"].as_str().unwrap(), sha256_hex(rendered["html"].as_str().unwrap()), "{name}");
                summary.push(format!("{name}: equal; rendered {}", rendered["sha256_hex"]));
            }
            Err(refusal) => {
                // The user-released inputs: the render command refuses them, as today
                // (their export context is not local_private).
                assert!(name.starts_with("user_release_exact/"), "{name}: {refusal}");
                assert!(
                    refusal.starts_with(RENDER_EXACT_CONTEXT_NOT_LOCAL_PRIVATE) || refusal.starts_with(crate::RENDER_USER_RELEASE_PACKAGE_ONLY),
                    "{name}: {refusal}"
                );
                summary.push(format!("{name}: equal; refused as today ({})", refusal.split(':').next().unwrap_or_default()));
            }
        }
    }
    assert_eq!(summary.len(), 8);
    eprintln!("{}", summary.join("\n"));
}

/// The pressure-elongation reference (pull request 217): the fill carries the
/// pressure-3 identity, each pressurised case's elongation label in the
/// evidence-derived rows and the fixed elongation statements in the
/// limitations exactly as the committed input does (above), and the statements
/// it carries are among the closed table's.
#[test]
fn the_fill_carries_the_pressure_3_identity_and_elongation_statements() {
    let reference = request_reference("pressure_2_straight");
    let live = live_job(&reference);
    let frame = frame_of(&reference.report, source_diagnostic_count(&reference.result));
    let filled = fill_only(&live, &custody_args(&live.job, &frame, &reference)).unwrap();
    assert_eq!(filled["exact_route"]["semantic_contract_id"], json!(open_pipe_stress_result_export::semantic_contract::PRESSURE_ID));
    let statements: Vec<&str> =
        filled["report_sections"]["limitations"].as_array().unwrap().iter().skip(1).filter_map(|entry| entry["statement"].as_str()).collect();
    assert!(statements.iter().any(|statement| statement.to_lowercase().contains("elongation")), "{statements:#?}");
}

#[test]
fn an_unknown_job_and_a_webview_custody_job_are_refused() {
    let reference = request_reference("pressure_2_straight");
    let live = live_job(&reference);
    let frame = frame_of(&reference.report, source_diagnostic_count(&reference.result));
    let error = message(render_from_custody(&live, &custody_args("backend-solve-job-unknown", &frame, &reference)).unwrap_err());
    assert!(error.starts_with(CUSTODY_RELEASED), "{error}");
    // A taken (webview-custody) result: the custody digests answer, the held text does not exist.
    let (registry, gate, job) = crate::solve_jobs::test_support::delivered_in_custody(
        ScriptedHost::normal(),
        reference.intake.as_bytes(),
        Some(&reference.mode),
        &reference.result,
    );
    let taken = Live { registry, _gate: gate, job };
    let error = message(render_from_custody(&taken, &custody_args(&taken.job, &frame, &reference)).unwrap_err());
    assert!(error.contains("PREVIEW-SOLVE-RESULT-ALREADY-DELIVERED"), "{error}");
}

#[test]
fn a_model_text_that_is_not_the_custodys_is_refused() {
    let reference = request_reference("pressure_2_straight");
    let live = live_job(&reference);
    let frame = frame_of(&reference.report, source_diagnostic_count(&reference.result));
    let mut args = custody_args(&live.job, &frame, &reference);
    args["intakeText"] = json!(format!("{} ", reference.intake));
    let error = message(render_from_custody(&live, &args).unwrap_err());
    assert_eq!(error, crate::exact_source_binding::not_delivered("model").message);
    assert!(error.starts_with(SOURCE_NOT_DELIVERED));
}

/// The frame rule: a member the host fills is refused by its path, never
/// overwritten; so is a result diagnostic or a second limitation entry.
#[test]
fn a_frame_that_carries_a_filled_member_is_refused_by_name() {
    let reference = request_reference("pressure_2_straight");
    let live = live_job(&reference);
    let frame = frame_of(&reference.report, source_diagnostic_count(&reference.result));
    let committed = bound(&reference.report, &live.job);
    let cases: [(&str, &[&str]); 8] = [
        ("result_rows", &["result_rows"]),
        ("model_input_tables", &["model_input_tables"]),
        ("exact_route", &["exact_route"]),
        (PF_TIMOSHENKO_SECTION, &[PF_TIMOSHENKO_SECTION]),
        ("calculation_report.result_export_refs", &["calculation_report", "result_export_refs"]),
        ("calculation_report.report_section_refs", &["calculation_report", "report_section_refs"]),
        ("calculation_report.model_input_summary.unit_display_summary", &["calculation_report", "model_input_summary", "unit_display_summary"]),
        ("report_context.result_coordinate_systems", &["report_context", "result_coordinate_systems"]),
    ];
    for (name, path) in cases {
        for value in [committed.pointer(&format!("/{}", path.join("/"))).cloned().unwrap_or(json!({})), Value::Null] {
            let mut smuggled = frame.clone();
            let (last, parents) = path.split_last().unwrap();
            let parent = parents.iter().fold(&mut smuggled, |at, key| &mut at[*key]);
            parent[*last] = value;
            let error = message(render_from_custody(&live, &custody_args(&live.job, &smuggled, &reference)).unwrap_err());
            assert_eq!(error, format!("{FRAME_MEMBER_DERIVED}: the frame carries {name}, which the host fills from the result in its custody."));
        }
    }
    let mut diagnostic = frame.clone();
    diagnostic["report_sections"]["diagnostics"].as_array_mut().unwrap().insert(0, committed["report_sections"]["diagnostics"][0].clone());
    let error = message(render_from_custody(&live, &custody_args(&live.job, &diagnostic, &reference)).unwrap_err());
    assert!(error.starts_with(FRAME_MEMBER_DERIVED) && error.contains("report_sections.diagnostics[0]"), "{error}");
    let mut limitation = frame.clone();
    limitation["report_sections"]["limitations"].as_array_mut().unwrap().push(committed["report_sections"]["limitations"][1].clone());
    let error = message(render_from_custody(&live, &custody_args(&live.job, &limitation, &reference)).unwrap_err());
    assert!(error.starts_with(FRAME_MEMBER_DERIVED) && error.contains("report_sections.limitations[1]"), "{error}");
}

#[test]
fn rule_pack_exclusions_and_an_unserved_result_are_refused() {
    let reference = request_reference("pressure_2_straight");
    let live = live_job(&reference);
    let frame = frame_of(&reference.report, source_diagnostic_count(&reference.result));
    let mut args = custody_args(&live.job, &frame, &reference);
    args["rulePackExclusions"] = json!([{"rule_pack_id": "invented-pack", "rule_pack_version": "1.0.0", "code": "RULE_PACK_REF_UNRESOLVED"}]);
    let error = message(render_from_custody(&live, &args).unwrap_err());
    assert!(error.starts_with(FILL_NOT_SERVED), "{error}");
    // A pressure-free Euler-Bernoulli result: the generic route is not served.
    let model = crate::report_package_save::test_support::model();
    let source = crate::report_package_save::test_support::witness_source();
    let generic = Reference {
        name: "preview-physics".into(),
        report: Value::Null,
        result: serde_json::to_string(&source).unwrap(),
        intake: canonical_json(&model),
        manifest: crate::report_package_save::test_support::input_manifest(&model, json!([])),
        mode: "sparse_interactive".into(),
        aggregate: Value::Null,
    };
    let generic_live = live_job(&generic);
    let error = message(render_from_custody(&generic_live, &custody_args(&generic_live.job, &frame, &generic)).unwrap_err());
    assert!(error.starts_with(FILL_NOT_SERVED), "{error}");
}

/// A refused render and a withheld render behave as today: the exact context
/// check refuses another export context with the same message, and a frame
/// whose provenance withholds the content renders the existing command's
/// metadata-only outcome.
#[test]
fn refused_and_withheld_renders_behave_as_today() {
    let reference = request_reference("pressure_2_straight");
    let live = live_job(&reference);
    let frame = frame_of(&reference.report, source_diagnostic_count(&reference.result));
    let mut public = frame.clone();
    public["report_context"]["export_context"] = json!("public_report");
    let mut committed_public = bound(&reference.report, &live.job);
    committed_public["report_context"]["export_context"] = json!("public_report");
    let today = outcome_text(render_today(&live, &committed_public, &reference));
    let from_custody = outcome_text(render_from_custody(&live, &custody_args(&live.job, &public, &reference)));
    assert!(today.as_ref().is_err_and(|error| error.starts_with(RENDER_EXACT_CONTEXT_NOT_LOCAL_PRIVATE)), "{today:?}");
    assert_eq!(from_custody, today);

    let mut withheld = frame.clone();
    let mut note = withheld["report_sections"]["provenance_notes"][0].clone();
    note["review_status"] = json!("quarantined");
    withheld["report_sections"]["provenance_notes"].as_array_mut().unwrap().push(note);
    let args = custody_args(&live.job, &withheld, &reference);
    let filled = fill_only(&live, &args).unwrap();
    let today = outcome_text(render_today(&live, &filled, &reference)).unwrap();
    let from_custody = outcome_text(render_from_custody(&live, &args)).unwrap();
    assert_eq!(from_custody, today);
    let rendered: Value = serde_json::from_str(&today).unwrap();
    assert_eq!((rendered["content_withheld"].clone(), rendered["export_blocked"].clone(), rendered["html"].clone()), (json!(true), json!(true), json!("")));
}

/// The pressure-free Timoshenko fill, on the transformed stand-in of
/// `report_render_pf_timoshenko_tests.rs` (no webview-built Timoshenko input
/// is committed), with a session input manifest: the frame is the desktop request's report with only its
/// technical-preview limitation and no result diagnostic; the new command
/// renders what the existing command renders for the host's filled input with
/// the texts resubmitted, and that render carries the identity's limitation 0
/// and the arc diagnostic.
#[test]
fn the_pressure_free_timoshenko_fill_renders_as_the_existing_command() {
    use open_pipe_stress_report_package::pf_timoshenko::REALIZED_ARCS_DIAGNOSTIC;
    use open_pipe_stress_result_export::semantic_contract::{
        preview_physics_timoshenko_contract, PREVIEW_PHYSICS_TIMOSHENKO_ID, PREVIEW_PHYSICS_TIMOSHENKO_PROFILE,
    };
    let mut model = crate::report_package_save::test_support::model();
    model["model_settings"] = json!({"shear_model": "timoshenko_annulus_energy_v1"});
    let mut source = crate::report_package_save::test_support::witness_source();
    source["producer"]["semantic_contract_id"] = json!(PREVIEW_PHYSICS_TIMOSHENKO_ID);
    source["formulation_basis"]["profile_id"] = json!(PREVIEW_PHYSICS_TIMOSHENKO_PROFILE);
    source["formulation_basis"]["limitations"] = preview_physics_timoshenko_contract()["supported_profile_limitations"].clone();
    let producer = source["diagnostics"][0]["source"].clone();
    source["diagnostics"].as_array_mut().unwrap().push(json!({
        "id": "diagnostic:preview-physics:pf-timoshenko:realized-arcs", "code": REALIZED_ARCS_DIAGNOSTIC,
        "severity": "info", "message": "1 straight member with shear deformation, 1 realized arc without: pipe:fixture-span",
        "source": producer, "affected_refs": ["pipe:fixture-span"]}));
    // The session's input manifest for this solve (the Rust mirror of
    // `buildCurrentSessionInputManifest`): the record builder reads its full
    // solver basis, which the committed invented manifest does not carry.
    let manifest = open_pipe_stress_report_package::user_release::derive_input_manifest(&model, &source, "sparse_interactive").unwrap();
    let request = crate::report_package_save::test_support::desktop_request_from(&model, &manifest, &source);
    let reference = Reference {
        name: "pf_timoshenko stand-in".into(),
        report: request["report"].clone(),
        result: serde_json::to_string(&source).unwrap(),
        intake: canonical_json(&model),
        manifest,
        mode: "sparse_interactive".into(),
        aggregate: Value::Null,
    };
    let live = live_job(&reference);
    let run = source["run_id"].as_str().unwrap();
    let mut frame = reference.report.clone();
    for key in ["result_rows", "model_input_tables", "exact_route", PF_TIMOSHENKO_SECTION] {
        frame.as_object_mut().unwrap().remove(key);
    }
    frame["calculation_report"].as_object_mut().unwrap().remove("result_export_refs");
    frame["calculation_report"].as_object_mut().unwrap().remove("report_section_refs");
    frame["calculation_report"]["model_input_summary"].as_object_mut().unwrap().remove("unit_display_summary");
    frame["report_context"].as_object_mut().unwrap().remove("result_coordinate_systems");
    let mut technical = frame["report_sections"]["limitations"][0].clone();
    technical["limitation_id"] = json!(format!("limitation:{run}:technical-preview"));
    frame["report_sections"]["limitations"] = json!([technical]);
    frame["report_sections"]["diagnostics"] = json!([]);
    let args = custody_args(&live.job, &frame, &reference);
    let filled = fill_only(&live, &args).unwrap_or_else(|error| panic!("{}", message(error)));
    assert_eq!(filled[PF_TIMOSHENKO_SECTION]["semantic_contract_id"], json!(PREVIEW_PHYSICS_TIMOSHENKO_ID));
    assert_eq!(filled[PF_TIMOSHENKO_SECTION]["admission"]["native_job_id"], json!(live.job));
    // The result diagnostics carry the session provenance the frame's
    // calculation report carries (on the desktop the same object as the first
    // provenance note; the stand-in's package basis writes two).
    let provenance = filled["calculation_report"]["provenance"].clone();
    let expected: Vec<Value> = expected_report_diagnostics(&source).unwrap().into_iter().map(|mut entry| { entry["provenance"] = provenance.clone(); entry }).collect();
    assert_eq!(filled["report_sections"]["diagnostics"], Value::Array(expected));
    let today = outcome_text(render_today(&live, &filled, &reference)).unwrap();
    let from_custody = outcome_text(render_from_custody(&live, &args)).unwrap();
    assert_eq!(from_custody, today);
    let rendered: Value = serde_json::from_str(&today).unwrap();
    assert_eq!(rendered["export_blocked"], json!(false), "{}", rendered["blocking_reasons"]);
    let html = rendered["html"].as_str().unwrap();
    let limitation = open_pipe_stress_report_renderer::escape_html(preview_physics_timoshenko_contract()["supported_profile_limitations"][0].as_str().unwrap());
    assert!(html.contains(&limitation) && html.contains(REALIZED_ARCS_DIAGNOSTIC));
}
