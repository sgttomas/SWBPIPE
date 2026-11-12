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
//! The pressure-free Timoshenko case is the committed webview-built render
//! input of the product's own Timoshenko result with a realized arc
//! (`fixtures/reports/invented/pf_timoshenko_render_inputs/`), compared like
//! the others; the transformed stand-in of
//! `report_render_pf_timoshenko_tests.rs` is also rendered by both commands.
//! A frame Diagnostics entry that copies a result diagnostic under other
//! prose, or that the host does not derive as a session entry, is refused by
//! both commands (review of f3d02469c, finding 2). Refusals: an unknown job, a
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

/// The committed webview-built render input of the product's own pressure-free
/// Timoshenko result with a realized arc (`pfTimoshenkoIdentity.test.tsx`
/// writes it with the panel's build step, `assembleRenderedReport`).
fn pf_timoshenko_reference() -> Reference {
    let file = read_json("fixtures/reports/invented/pf_timoshenko_render_inputs/pf_timoshenko_arc.json");
    Reference {
        name: "pf_timoshenko_arc".to_string(),
        report: file["report"].clone(),
        result: std::fs::read_to_string(project(file["raw_path"].as_str().unwrap())).unwrap(),
        intake: file["exact_intake_text"].as_str().unwrap().to_string(),
        manifest: file["input_manifest"].clone(),
        mode: file["solver_mode"].as_str().unwrap().to_string(),
        aggregate: file["rule_check_aggregate"].clone(),
    }
}

fn every_reference() -> Vec<Reference> {
    let mut all: Vec<Reference> = SHAPES.iter().map(|shape| request_reference(shape)).collect();
    all.push(substitute_reference());
    all.extend(USER_RELEASE_SHAPES.iter().map(|shape| user_release_reference(shape)));
    all.push(pf_timoshenko_reference());
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
    assert_eq!(summary.len(), 9);
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
/// `report_render_pf_timoshenko_tests.rs`, with a session input manifest: the frame is the desktop request's report with only its
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

/// DEL-08-01 AC-009, item by item, on both routes: the existing command with
/// submitted texts, and the render from custody (which accepts no result
/// text: it reads the held bytes). Each named refusal, with its code.
#[test]
fn each_refusal_ac_009_names_is_made_on_both_routes() {
    use crate::exact_source_binding::STANDING_NOT_ELIGIBLE;
    let reference = request_reference("pressure_2_straight");
    let committed_frame = frame_of(&reference.report, source_diagnostic_count(&reference.result));
    let substitute = substitute_reference();
    let manifest = canonical_json(&reference.manifest);
    let mut moved: Value = serde_json::from_str(&reference.intake).unwrap();
    let x = moved["nodes"][0]["position"]["x"].as_f64().unwrap();
    moved["nodes"][0]["position"]["x"] = json!(x + 0.001);
    let moved = serde_json::to_string(&moved).unwrap();
    let half = |text: &str| text[..text.len() / 2].to_string();
    let today_with = |live: &Live, result: Option<String>, intake: Option<String>| {
        let mut args = json!({"input": bound(&reference.report, &live.job), "inputManifestText": manifest});
        if let Some(result) = result {
            args["exactResultText"] = json!(result);
        }
        if let Some(intake) = intake {
            args["exactIntakeText"] = json!(intake);
        }
        let (mut decoded, mut lease) = admit_json(&live.registry, "render_calculation_report", ArgsDecoding::Moved, &args).unwrap();
        message(crate::render_calculation_report_args(&mut decoded, &live.registry, &mut lease).unwrap_err())
    };
    let not_delivered = |what: &str| crate::exact_source_binding::not_delivered(what).message;
    let live = live_job(&reference);
    // The existing command: substituted, partial and absent result text; the same for the model.
    for (name, result, intake, expected) in [
        ("substituted result text", Some(substitute.result.clone()), Some(reference.intake.clone()), not_delivered("result")),
        ("partial result text", Some(half(&reference.result)), Some(reference.intake.clone()), not_delivered("result")),
        ("absent result text", None, Some(reference.intake.clone()), not_delivered("result")),
        ("substituted model text", Some(reference.result.clone()), Some(moved.clone()), not_delivered("model")),
        ("partial model text", Some(reference.result.clone()), Some(half(&reference.intake)), not_delivered("model")),
        ("absent model text", Some(reference.result.clone()), None, not_delivered("model")),
    ] {
        assert_eq!(today_with(&live, result, intake), expected, "existing command: {name}");
    }
    // The render from custody: substituted and partial model text refuse by the
    // custody's intake digest; an absent one is not a request of the command's form.
    for (name, intake) in [("substituted model text", moved.clone()), ("partial model text", half(&reference.intake))] {
        let mut args = custody_args(&live.job, &committed_frame, &reference);
        args["intakeText"] = json!(intake);
        assert_eq!(message(render_from_custody(&live, &args).unwrap_err()), not_delivered("model"), "render from custody: {name}");
    }
    let mut args = custody_args(&live.job, &committed_frame, &reference);
    args.as_object_mut().unwrap().remove("intakeText");
    let (mut decoded, mut lease) = admit_json(&live.registry, "render_calculation_report_from_custody", ArgsDecoding::Moved, &args).unwrap();
    let absent = crate::render_calculation_report_from_custody_args(&mut decoded, &live.registry, &mut lease).unwrap_err();
    assert!(matches!(absent, CommandError::Ingress(_)), "render from custody: absent model text: {absent:?}");
    // A result text offered to the render from custody is not read: the held bytes are.
    let mut args = custody_args(&live.job, &committed_frame, &reference);
    args["exactResultText"] = json!(substitute.result);
    assert!(render_from_custody(&live, &args).is_ok(), "render from custody: an offered result text is not read");
    // A result no longer held: both commands.
    live.registry.dispose_result(&live.job).unwrap();
    let released = crate::exact_source_binding::custody_released().message;
    assert_eq!(today_with(&live, Some(reference.result.clone()), Some(reference.intake.clone())), released, "existing command: a result no longer held");
    assert_eq!(message(render_from_custody(&live, &custody_args(&live.job, &committed_frame, &reference)).unwrap_err()), released, "render from custody: a result no longer held");
    // An ineligible standing: a pressure-source-1 result with load/reference
    // states in custody (as `report_render_exact_tests`), both commands.
    let source_1 = request_reference("pressure_source_1");
    let raw = std::fs::read_to_string(project("fixtures/product_preview/pressure_source/load_reference_source/n05-sparse_interactive.raw.json")).unwrap();
    let request = read_json("fixtures/product_preview/pressure_source/load_reference_source/n05.request.json");
    let ineligible = Reference { result: raw.clone(), intake: serde_json::to_string(&request["model"]).unwrap(), ..source_1 };
    let live = live_job(&ineligible);
    let standing = format!("{STANDING_NOT_ELIGIBLE}: the native reading of the delivered result is not numerically eligible.");
    let mut input = bound(&ineligible.report, &live.job);
    input["exact_route"]["admission"]["received_bytes_sha256"] = json!(sha256_hex(&raw));
    let args = json!({"input": input, "exactResultText": raw, "exactIntakeText": ineligible.intake, "inputManifestText": canonical_json(&ineligible.manifest)});
    let (mut decoded, mut lease) = admit_json(&live.registry, "render_calculation_report", ArgsDecoding::Moved, &args).unwrap();
    assert_eq!(message(crate::render_calculation_report_args(&mut decoded, &live.registry, &mut lease).unwrap_err()), standing, "existing command: an ineligible standing");
    let frame = frame_of(&ineligible.report, source_diagnostic_count(&std::fs::read_to_string(project("fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json")).unwrap()));
    assert_eq!(message(render_from_custody(&live, &custody_args(&live.job, &frame, &ineligible)).unwrap_err()), standing, "render from custody: an ineligible standing");
}

// ---------------------------------------------------------------------------
// A frame Diagnostics entry that copies a result diagnostic under other prose
// (review of f3d02469c, finding 2). The reviewer's counterexample, executed:
// the stripped straight frame with the result's committed
// LOAD_CATEGORY_PREVIEW_MAPPED entry appended, its remediation and message
// changed, its provenance and other fields kept; and two variants.
// ---------------------------------------------------------------------------

/// The committed straight input's first LOAD_CATEGORY_PREVIEW_MAPPED entry.
fn committed_load_category_entry(report: &Value) -> Value {
    report["report_sections"]["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["code"] == "LOAD_CATEGORY_PREVIEW_MAPPED")
        .cloned()
        .expect("the straight result carries LOAD_CATEGORY_PREVIEW_MAPPED")
}

#[test]
fn a_frame_entry_copying_a_result_diagnostic_under_other_prose_is_refused() {
    let reference = request_reference("pressure_2_straight");
    let live = live_job(&reference);
    let count = source_diagnostic_count(&reference.result);
    let frame = frame_of(&reference.report, count);
    let committed = bound(&reference.report, &live.job);
    let genuine = committed_load_category_entry(&reference.report);
    assert_eq!(genuine["affected_object"]["ref_id"], json!("load:fixture-global_x, case:six-component-load"));
    let session_form = [
        ("class", json!("PROVENANCE_WARNING")),
        ("source", json!({"ref_type": "component", "ref_id": "component:invented"})),
        ("affected_object", json!({"ref_type": "component", "ref_id": "load:fixture-global_x, case:six-component-load"})),
    ];
    let mut session_tag = session_form.to_vec();
    session_tag.push(("diagnostic_origin", json!("session")));
    let variants: [(&str, Vec<(&str, Value)>); 6] = [
        ("the reviewer's counterexample: remediation and message changed", vec![
            ("remediation", json!("Review this load.")),
            ("message", json!("The load category is user selected.")),
        ]),
        ("the message changed only", vec![("message", json!("The load category is user selected."))]),
        ("the remediation and severity changed", vec![("remediation", json!("Review this load.")), ("severity", json!("info"))]),
        ("a session entry's class and reference types", session_form.to_vec()),
        ("a caller tag claiming session origin", session_tag),
        ("a caller tag claiming result origin", vec![("diagnostic_origin", json!("result")), ("message", json!("The load category is user selected."))]),
    ];
    for (name, edits) in variants {
        let mut entry = genuine.clone();
        for (field, value) in &edits {
            entry[*field] = value.clone();
        }
        // The render from custody: the frame with the entry appended.
        let mut forged = frame.clone();
        let diagnostics = forged["report_sections"]["diagnostics"].as_array_mut().unwrap();
        diagnostics.push(entry.clone());
        let index = diagnostics.len() - 1;
        let error = match render_from_custody(&live, &custody_args(&live.job, &forged, &reference)) {
            Err(error) => message(error),
            Ok(rendered) => panic!(
                "{name}: the forged entry was admitted and rendered (forged message printed: {})",
                rendered["html"].as_str().unwrap_or_default().contains("The load category is user selected.")
            ),
        };
        assert_eq!(
            error,
            format!("{FRAME_MEMBER_DERIVED}: the frame carries report_sections.diagnostics[{index}], which the host fills from the result in its custody."),
            "{name}"
        );
        // The existing command: the committed input with the entry appended.
        let mut forged_input = committed.clone();
        forged_input["report_sections"]["diagnostics"].as_array_mut().unwrap().push(entry);
        let error = match render_today(&live, &forged_input, &reference) {
            Err(error) => message(error),
            Ok(rendered) => panic!(
                "{name}: the existing command admitted and rendered the forged entry (forged message printed: {})",
                rendered["html"].as_str().unwrap_or_default().contains("The load category is user selected.")
            ),
        };
        assert_eq!(
            error,
            format!("{}: diagnostics: report diagnostic {count} copies a result diagnostic's code and affected object", crate::exact_source_binding::SECTIONS_MISMATCH),
            "{name}"
        );
    }
    // An invented entry in a result diagnostic's form that copies none: both
    // commands refuse it, as no session entry the host derives.
    let mut invented = genuine.clone();
    invented["code"] = json!("SOLVER_RESULT_VERIFIED");
    invented["affected_object"]["ref_id"] = json!("case:six-component-load");
    let mut forged = frame.clone();
    forged["report_sections"]["diagnostics"].as_array_mut().unwrap().push(invented.clone());
    let mut forged_input = committed.clone();
    forged_input["report_sections"]["diagnostics"].as_array_mut().unwrap().push(invented);
    let expected = format!(
        "{}: diagnostics: report diagnostic {count} is not a session diagnostic the host derives from the bound model",
        crate::exact_source_binding::SECTIONS_MISMATCH
    );
    assert_eq!(message(render_from_custody(&live, &custody_args(&live.job, &forged, &reference)).unwrap_err()), expected);
    assert_eq!(message(render_today(&live, &forged_input, &reference).unwrap_err()), expected);
}

/// The pressure-free Timoshenko stand-in over `model` (as in
/// [`the_pressure_free_timoshenko_fill_renders_as_the_existing_command`]):
/// the reference and its frame, with no Diagnostics entry of its own.
fn pf_stand_in(mut model: Value) -> (Reference, Value) {
    use open_pipe_stress_report_package::pf_timoshenko::REALIZED_ARCS_DIAGNOSTIC;
    use open_pipe_stress_result_export::semantic_contract::{
        preview_physics_timoshenko_contract, PREVIEW_PHYSICS_TIMOSHENKO_ID, PREVIEW_PHYSICS_TIMOSHENKO_PROFILE,
    };
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
    let manifest = open_pipe_stress_report_package::user_release::derive_input_manifest(&model, &source, "sparse_interactive").unwrap();
    let request = crate::report_package_save::test_support::desktop_request_from(&model, &manifest, &source);
    let run = source["run_id"].as_str().unwrap().to_string();
    let reference = Reference {
        name: "pf_timoshenko stand-in".into(),
        report: request["report"].clone(),
        result: serde_json::to_string(&source).unwrap(),
        intake: canonical_json(&model),
        manifest,
        mode: "sparse_interactive".into(),
        aggregate: Value::Null,
    };
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
    (reference, frame)
}

/// A library reference of the material (invented identities).
const LIBRARY_REF: &str = r#"{"library_kind": "material", "library_id": "library:invented", "record_id": "record:invented", "library_version": "sha256:0000000000000000000000000000000000000000000000000000000000000000"}"#;

/// The session entries the desktop builds for the stand-in model with its
/// material's provenance removed and [`LIBRARY_REF`] on it, written here from
/// `renderableReportInput.ts` (`subjectProvenance`, `libraryFindingDiagnostics`)
/// and `lib.rs` `library_ref_findings`, not from the host's derivation.
fn desktop_session_entries(provenance: &Value) -> Vec<Value> {
    let project = "project:precision-connected-fixture";
    let entity = "material:fixture-isotropic";
    let library = |code: &str, message: String| json!({
        "code": code, "class": "PROVENANCE_WARNING", "severity": "warning",
        "source": {"ref_type": "diagnostic_source", "ref_id": "apps/desktop/src-tauri/src/lib.rs check_model_library_refs"},
        "affected_object": {"ref_type": "model_entity", "ref_id": entity},
        "message": message,
        "remediation": "Compare the library reference with the local library store before relying on this report.",
        "provenance": provenance,
    });
    vec![
        json!({
            "code": "MATERIAL_PROVENANCE_MISSING", "class": "PROVENANCE_WARNING", "severity": "warning",
            "source": {"ref_type": "material", "ref_id": entity}, "affected_object": {"ref_type": "material", "ref_id": entity},
            "message": "Material fixture-isotropic has neither a provenance_record nor a provenance note; report provenance is incomplete for this material.",
            "remediation": "Enter material source/provenance before relying on this report.",
            "provenance": provenance,
        }),
        library("MODEL-LIBRARY-REF-UNRESOLVED", format!("{entity} names material library `library:invented`, which the local store does not hold for project `{project}`. The model's values stand. A project saved under a new id reads its references as unresolved until the library is imported into that project.")),
        library("MODEL-LIBRARY-REF-UNRESOLVED", format!("{entity} names record `record:invented` of material library `library:invented`, which the stored library does not hold. The model's values stand.")),
        library("MODEL-LIBRARY-REF-VERSION-CHANGED", format!("The stored record `record:invented` of material library `library:invented` has changed since {entity} recorded it. The model's values and recorded version stand; re-apply from the library to adopt the stored record.")),
    ]
}

/// The session entries the desktop appends are admitted by both commands when
/// the host derives them from the bound model: a material without provenance
/// and each library store finding on a referenced entity. Each derived kind's
/// texts are also read from the desktop builder's source, so a change there
/// that the host's mirror does not follow fails here, not in a user's render.
#[test]
fn the_session_entries_the_host_derives_are_admitted_after_the_results() {
    let builder = std::fs::read_to_string(project("apps/desktop/src/features/report/renderableReportInput.ts")).unwrap();
    for fragment in [
        "has neither a provenance_record nor a component.provenance note; report provenance is incomplete for this component.",
        "Enter component source/provenance before relying on this report.",
        "is missing support provenance or hanger.source_reference.",
        "Enter spring-hanger source/provenance before relying on this report.",
        "has neither a provenance_record nor a provenance note; report provenance is incomplete for this ${subject.kind}.",
        "Enter ${subject.kind} source/provenance before relying on this report.",
        "Compare the library reference with the local library store before relying on this report.",
        "class: \"PROVENANCE_WARNING\"",
    ] {
        assert!(builder.contains(fragment), "the desktop builder no longer writes {fragment:?}");
    }
    let findings = std::fs::read_to_string(project("apps/desktop/src/features/library/libraryRefFindings.ts")).unwrap();
    assert!(findings.contains(&format!("LIBRARY_REF_CHECK_SOURCE = \"{}\"", crate::report_session_diagnostics::LIBRARY_REF_CHECK_SOURCE)));

    let mut model = crate::report_package_save::test_support::model();
    model["materials"][0].as_object_mut().unwrap().remove("provenance");
    model["materials"][0]["library_ref"] = serde_json::from_str(LIBRARY_REF).unwrap();
    let (reference, mut frame) = pf_stand_in(model);
    let live = live_job(&reference);
    let provenance = frame["calculation_report"]["provenance"].clone();
    // The material's entry and one store finding for its one reference site
    // (the store check yields one finding per reference).
    let every = desktop_session_entries(&provenance);
    let session = vec![every[0].clone(), every[3].clone()];
    frame["report_sections"]["diagnostics"] = Value::Array(session.clone());
    let args = custody_args(&live.job, &frame, &reference);
    let filled = fill_only(&live, &args).unwrap_or_else(|error| panic!("{}", message(error)));
    let diagnostics = filled["report_sections"]["diagnostics"].as_array().unwrap();
    let count = diagnostics.len() - session.len();
    assert_eq!(&diagnostics[count..], session.as_slice());
    let today = outcome_text(render_today(&live, &filled, &reference));
    assert!(today.is_ok(), "{today:?}");
    assert_eq!(outcome_text(render_from_custody(&live, &args)), today);
    // A library finding with a message the store check does not write, and one
    // on an entity without a library reference, are refused by both commands.
    for (name, edit) in [
        ("an altered library message", json!({"message": "The library is verified."})),
        ("another entity", json!({"affected_object": {"ref_type": "model_entity", "ref_id": "support:fixture-root"}})),
    ] {
        let mut forged = frame.clone();
        for (field, value) in edit.as_object().unwrap() {
            forged["report_sections"]["diagnostics"][1][field] = value.clone();
        }
        let from_custody = message(render_from_custody(&live, &custody_args(&live.job, &forged, &reference)).unwrap_err());
        let mut forged_input = filled.clone();
        for (field, value) in edit.as_object().unwrap() {
            forged_input["report_sections"]["diagnostics"][count + 1][field] = value.clone();
        }
        let today = message(render_today(&live, &forged_input, &reference).unwrap_err());
        let expected = format!(
            "{}: diagnostics: report diagnostic {} is not a session diagnostic the host derives from the bound model",
            crate::exact_source_binding::SECTIONS_MISMATCH,
            count + 1
        );
        assert_eq!((from_custody.as_str(), today.as_str()), (expected.as_str(), expected.as_str()), "{name}");
    }
    // R3 and R4 (review of c2cc050ea), on both commands: the reviewer's case, the
    // material's entry twice for one material; and two store findings for one
    // reference site. Each is refused at its third appended entry.
    for (name, entries) in [
        ("the material's entry twice", vec![every[0].clone(), every[3].clone(), every[0].clone()]),
        ("two findings for one reference site", vec![every[0].clone(), every[3].clone(), every[1].clone()]),
    ] {
        let mut forged = frame.clone();
        forged["report_sections"]["diagnostics"] = Value::Array(entries.clone());
        let from_custody = message(render_from_custody(&live, &custody_args(&live.job, &forged, &reference)).unwrap_err());
        let mut forged_input = filled.clone();
        let diagnostics = forged_input["report_sections"]["diagnostics"].as_array_mut().unwrap();
        diagnostics.truncate(count);
        diagnostics.extend(entries);
        let today = message(render_today(&live, &forged_input, &reference).unwrap_err());
        let expected = format!(
            "{}: diagnostics: report diagnostic {} is not a session diagnostic the host derives from the bound model",
            crate::exact_source_binding::SECTIONS_MISMATCH,
            count + 2
        );
        assert_eq!((from_custody.as_str(), today.as_str()), (expected.as_str(), expected.as_str()), "{name}");
    }
}

/// The session binding alone, on `model`, for `appended` entries after a
/// result without diagnostics: `Ok`, or the refused entry's index.
fn session_check(model: &Value, appended: &[Value]) -> Result<(), String> {
    let input = json!({"report_sections": {"diagnostics": appended}});
    crate::report_session_diagnostics::check_session_diagnostics(&input, &json!({"diagnostics": []}), model)
}

/// The desktop's library finding entry for `entity` (texts from
/// `renderableReportInput.ts` `libraryFindingDiagnostics`).
fn library_entry(code: &str, entity: &str, message: String) -> Value {
    json!({
        "code": code, "class": "PROVENANCE_WARNING", "severity": "warning",
        "source": {"ref_type": "diagnostic_source", "ref_id": "apps/desktop/src-tauri/src/lib.rs check_model_library_refs"},
        "affected_object": {"ref_type": "model_entity", "ref_id": entity},
        "message": message,
        "remediation": "Compare the library reference with the local library store before relying on this report.",
    })
}

/// R3 (review of c2cc050ea): multiplicity by counts. One material without
/// provenance admits its entry once, not twice; a component carrying the same
/// library reference at two sites yields two identical findings, which are
/// admitted twice and refused a third time.
#[test]
fn a_session_entry_is_admitted_only_as_often_as_the_model_yields_it() {
    let material = json!({
        "code": "MATERIAL_PROVENANCE_MISSING", "class": "PROVENANCE_WARNING", "severity": "warning",
        "source": {"ref_type": "material", "ref_id": "material:bare"}, "affected_object": {"ref_type": "material", "ref_id": "material:bare"},
        "message": "Material material:bare has neither a provenance_record nor a provenance note; report provenance is incomplete for this material.",
        "remediation": "Enter material source/provenance before relying on this report.",
    });
    let one_material = json!({"project": {"id": "project:invented"}, "materials": [{"id": "material:bare"}]});
    assert_eq!(session_check(&one_material, &[material.clone()]), Ok(()));
    assert_eq!(session_check(&one_material, &[material.clone(), material.clone()]).unwrap_err(), "report diagnostic 1 is not a session diagnostic the host derives from the bound model");
    let reference = json!({"library_kind": "component", "library_id": "library:invented", "record_id": "record:invented"});
    let twice = json!({"project": {"id": "project:invented"}, "components": [
        {"id": "component:twice", "provenance": "noted", "library_ref": reference, "geometry": {"library_ref": reference}}
    ]});
    let finding = library_entry("MODEL-LIBRARY-REF-UNRESOLVED", "component:twice", "component:twice names component library `library:invented`, which the local store does not hold for project `project:invented`. The model's values stand. A project saved under a new id reads its references as unresolved until the library is imported into that project.".into());
    assert_eq!(session_check(&twice, &[finding.clone(), finding.clone()]), Ok(()));
    assert_eq!(session_check(&twice, &[finding.clone(), finding.clone(), finding.clone()]).unwrap_err(), "report diagnostic 2 is not a session diagnostic the host derives from the bound model");
}

/// R4 (review of c2cc050ea): the store check's preconditions, per reference
/// shape, and at most one finding per reference site.
#[test]
fn a_library_finding_is_admitted_only_where_the_store_check_can_make_it() {
    let entity = "material:invented";
    let not_held = |kind: &str, library: &str| library_entry("MODEL-LIBRARY-REF-UNRESOLVED", entity, format!("{entity} names {kind} library `{library}`, which the local store does not hold for project `project:invented`. The model's values stand. A project saved under a new id reads its references as unresolved until the library is imported into that project."));
    let record_not_held = |record: &str| library_entry("MODEL-LIBRARY-REF-UNRESOLVED", entity, format!("{entity} names record `{record}` of material library `library:invented`, which the stored library does not hold. The model's values stand."));
    let changed = |record: &str| library_entry("MODEL-LIBRARY-REF-VERSION-CHANGED", entity, format!("The stored record `{record}` of material library `library:invented` has changed since {entity} recorded it. The model's values and recorded version stand; re-apply from the library to adopt the stored record."));
    let model = |reference: Value| json!({"project": {"id": "project:invented"}, "materials": [{"id": entity, "provenance": "noted", "library_ref": reference}]});
    let version = "sha256:0000000000000000000000000000000000000000000000000000000000000000";
    let refused = Err("report diagnostic 0 is not a session diagnostic the host derives from the bound model".to_string());
    // A record and a version: any one of the three findings, never two.
    let full = model(json!({"library_kind": "material", "library_id": "library:invented", "record_id": "record:invented", "library_version": version}));
    for one in [not_held("material", "library:invented"), record_not_held("record:invented"), changed("record:invented")] {
        assert_eq!(session_check(&full, &[one]), Ok(()));
    }
    assert_eq!(
        session_check(&full, &[changed("record:invented"), not_held("material", "library:invented")]).unwrap_err(),
        "report diagnostic 1 is not a session diagnostic the host derives from the bound model"
    );
    // The reviewer's shape: a version and no record id. The store check returns
    // the record-not-held finding before any version comparison.
    let no_record = model(json!({"library_kind": "material", "library_id": "library:invented", "library_version": version}));
    assert_eq!(session_check(&no_record, &[record_not_held("(none recorded)")]), Ok(()));
    assert_eq!(session_check(&no_record, &[not_held("material", "library:invented")]), Ok(()));
    assert_eq!(session_check(&no_record, &[changed("")]), refused);
    // A blank record id is no record (`library_identity`).
    let blank_record = model(json!({"library_kind": "material", "library_id": "library:invented", "record_id": "  ", "library_version": version}));
    assert_eq!(session_check(&blank_record, &[changed("  ")]), refused);
    // A record and no version: no version comparison.
    let no_version = model(json!({"library_kind": "material", "library_id": "library:invented", "record_id": "record:invented"}));
    assert_eq!(session_check(&no_version, &[record_not_held("record:invented")]), Ok(()));
    assert_eq!(session_check(&no_version, &[changed("record:invented")]), refused);
    // A kind the store does not hold, and a blank library id: only "not held".
    let other_kind = model(json!({"library_kind": "pump", "library_id": "library:invented", "record_id": "record:invented", "library_version": version}));
    assert_eq!(session_check(&other_kind, &[not_held("pump", "library:invented")]), Ok(()));
    let pump_record = library_entry("MODEL-LIBRARY-REF-UNRESOLVED", entity, format!("{entity} names record `record:invented` of pump library `library:invented`, which the stored library does not hold. The model's values stand."));
    assert_eq!(session_check(&other_kind, &[pump_record]), refused);
    let blank_library = model(json!({"library_kind": "material", "library_id": " ", "record_id": "record:invented", "library_version": version}));
    assert_eq!(session_check(&blank_library, &[not_held("material", " ")]), Ok(()));
    let blank_library_record = library_entry("MODEL-LIBRARY-REF-UNRESOLVED", entity, format!("{entity} names record `record:invented` of material library ` `, which the stored library does not hold. The model's values stand."));
    assert_eq!(session_check(&blank_library, &[blank_library_record]), refused);
}

/// The host's derivation of each session kind on an invented model, against
/// the entries the desktop builder writes for it (texts from the builder).
#[test]
fn the_host_derives_each_session_kind_the_desktop_appends() {
    let model = json!({
        "project": {"id": " project:invented "},
        "components": [
            {"id": "component:bare", "label": ""},
            {"id": "component:noted", "provenance": "  note  "},
            {"id": "component:recorded", "provenance_record": {"source_name": " Invented "}},
            {"id": "component:blank", "label": "Blank", "provenance": " \u{feff}", "provenance_record": {"source_name": "  "}},
        ],
        "supports": [
            {"id": "support:hanger", "label": "H-1", "family": " spring_hanger ", "provenance": "noted"},
            {"id": "support:typed", "family": "anchor", "hanger": {"hanger_type": "constant_effort_support", "source_reference": "ref"}, "provenance": "noted"},
            {"id": "support:anchor", "family": "anchor"},
        ],
        "materials": [{"id": "material:bare", "label": "  Steel  "}, {"id": "material:noted", "provenance": {"note": "x"}}],
        "sections": [{"id": "section:bare", "name": " "}, {"id": "section:empty-note", "provenance": {}}],
    });
    let derived = crate::report_session_diagnostics::session_diagnostics(&model);
    let summary: Vec<(String, String, String)> = derived
        .iter()
        .map(|entry| (entry["code"].as_str().unwrap().into(), entry["affected_object"]["ref_id"].as_str().unwrap().into(), entry["message"].as_str().unwrap().into()))
        .collect();
    let line = |code: &str, id: &str, message: &str| (code.to_string(), id.to_string(), message.to_string());
    assert_eq!(summary, vec![
        line("COMPONENT_PROVENANCE_MISSING", "component:bare", "Component component:bare has neither a provenance_record nor a component.provenance note; report provenance is incomplete for this component."),
        line("COMPONENT_PROVENANCE_MISSING", "component:blank", "Component Blank has neither a provenance_record nor a component.provenance note; report provenance is incomplete for this component."),
        line("SPRING_HANGER_PROVENANCE_MISSING", "support:hanger", "Spring hanger H-1 is missing support provenance or hanger.source_reference."),
        line("MATERIAL_PROVENANCE_MISSING", "material:bare", "Material Steel has neither a provenance_record nor a provenance note; report provenance is incomplete for this material."),
        line("SECTION_PROVENANCE_MISSING", "section:bare", "Section section:bare has neither a provenance_record nor a provenance note; report provenance is incomplete for this section."),
        line("SECTION_PROVENANCE_MISSING", "section:empty-note", "Section section:empty-note has neither a provenance_record nor a provenance note; report provenance is incomplete for this section."),
    ]);
}

// ---------------------------------------------------------------------------
// The render from custody's allocations against its lease (review of
// f3d02469c, finding 1): the counting allocator's peak over the whole command
// (raw body, ingress admission, production root, response) and over the
// derivation alone (`derive_binding`: the analysis-run record and the Current
// result document derived to read the admission binding), each against the
// held result text's length.
// ---------------------------------------------------------------------------

/// What the counting allocator measured for one held result.
#[derive(Debug)]
struct Peaks {
    text: u64,
    derivation_requested: u64,
    derivation_priced: u64,
    /// What the derivation asked its `reserve` for (the record builder's fingerprints).
    derivation_reserved: u64,
}

/// The derivation alone, on the held text's one parse: everything before it
/// (custody, the parse, the manifest's parse) is allocated outside the measure.
fn derivation_peaks(registry: &SolveJobRegistry, job: &str, intake: &str, manifest_text: &str, aggregate: Option<&str>) -> Peaks {
    let digests = crate::exact_source_binding::check_custody(&json!({"native_job_id": job}), &|job| registry.custody_binding(job)).unwrap();
    let held = registry.held_result(job).ok().expect("held");
    // A scratch result the reader does not find eligible is still measured: the
    // derivation reads the parsed values only.
    let mut bound = crate::exact_source_binding::read_eligible_source(&digests, held.text(), intake).unwrap_or_else(|refusal| {
        eprintln!("(not eligible, measured on the parsed values: {})", refusal.message);
        let model: Value = serde_json::from_str(intake).unwrap();
        let model_sha256 = open_pipe_stress_result_export::derivative::digest(&model).unwrap();
        crate::exact_source_binding::BoundSource { source: serde_json::from_str(held.text()).unwrap(), model, model_sha256 }
    });
    let manifest = crate::report_package_save::parse_canonical("input_manifest", manifest_text).ok().expect("manifest");
    let manifest_text = manifest_text.to_string();
    let mut reserved = 0u64;
    let (outcome, measured) = crate::ingress::counting::measure(|| {
        let mut reserve = |bytes: u64| -> Result<Option<crate::memory_pool::CommandLease>, CommandError<String>> {
            reserved += bytes;
            Ok(None)
        };
        crate::report_native_fill::derive_binding(&held, &mut bound.source, &bound.model, &bound.model_sha256, manifest_text, &manifest, aggregate, &[], &mut reserve)
            .map(|_| ())
    });
    outcome.unwrap_or_else(|error| panic!("the derivation refused: {}", message(error)));
    Peaks {
        text: held.text().len() as u64,
        derivation_requested: measured.peak_requested,
        derivation_priced: measured.peak_priced,
        derivation_reserved: reserved,
    }
}

/// The whole command from its raw body: `(outcome, priced peak, grown lease
/// less the body's envelope, ingress lease less the envelope)`.
fn command_peak(live: &Live, args: &Value) -> (Result<(), String>, u64, u64, u64) {
    use crate::command_ingress::test_support::headers;
    use crate::command_ingress::{admit_body, INGRESS_ARGS_JSON_V1};
    let raw = serde_json::to_vec(args).unwrap();
    let envelope = crate::ingress::allocation_envelope(raw.len() as u64).unwrap();
    let body = tauri::ipc::InvokeBody::Raw(raw);
    let headers = headers(Some(INGRESS_ARGS_JSON_V1));
    let ((outcome, lease_bytes, ingress), measured) = crate::ingress::counting::measure(|| {
        let (mut decoded, mut lease) = admit_body(&live.registry, &body, &headers, "render_calculation_report_from_custody", ArgsDecoding::Moved).unwrap();
        let ingress = lease.bytes();
        let outcome = crate::command_ingress::respond(crate::render_calculation_report_from_custody_args(&mut decoded, &live.registry, &mut lease))
            .map(|_| ())
            .map_err(message);
        drop(decoded);
        (outcome, lease.bytes(), ingress)
    });
    (outcome, measured.peak_priced, lease_bytes - envelope, ingress - envelope)
}

/// Finding 1 (a), the measurement on every committed reference: the
/// derivation's peak and the whole command's, each as a multiple of the held
/// text (printed; `--nocapture`).
#[test]
#[ignore = "measurement: prints the peaks (finding 1 of the review of f3d02469c); run with --ignored --nocapture"]
fn the_render_from_custody_allocations_are_measured_against_the_held_text() {
    for reference in every_reference() {
        let name = &reference.name;
        let live = live_job(&reference);
        let manifest_text = canonical_json(&reference.manifest);
        let peaks = derivation_peaks(&live.registry, &live.job, &reference.intake, &manifest_text, reference.aggregate.as_str());
        let frame = frame_of(&reference.report, source_diagnostic_count(&reference.result));
        let (outcome, peak, lease, ingress) = command_peak(&live, &custody_args(&live.job, &frame, &reference));
        let growth = crate::report_render_exact::exact_render_growth_bytes(&reference.result, &reference.intake, &manifest_text).unwrap();
        let text = peaks.text as f64;
        eprintln!(
            "{name}: text {} | derivation peak requested {} ({:.2}x) priced {} ({:.2}x) reserved {} | command {} priced peak {} ({:.2}x) lease {} ({:.2}x; ingress {}, render growth {})",
            peaks.text,
            peaks.derivation_requested,
            peaks.derivation_requested as f64 / text,
            peaks.derivation_priced,
            peaks.derivation_priced as f64 / text,
            peaks.derivation_reserved,
            if outcome.is_ok() { "rendered".to_string() } else { format!("refused {:?}", outcome.as_ref().unwrap_err().split(':').next()) },
            peak,
            peak as f64 / text,
            lease,
            lease as f64 / text,
            ingress,
            growth,
        );
    }
}

/// Finding 1 (a) at a rack size, scratch only (never committed data): the
/// derivation's peak on a result the product's own entry wrote, or on the
/// product's solve of the model at this head. Run with
/// `SWBPIPE_FILL_MEASURE_MODEL=<model.json> [SWBPIPE_FILL_MEASURE_RESULT=<result.json>]
/// [SWBPIPE_FILL_MEASURE_MODE=sparse_interactive]` and `--ignored --nocapture`.
#[test]
#[ignore = "scratch measurement: reads a large result named by the environment"]
fn the_derivation_is_measured_on_a_scratch_result() {
    let model: Value = serde_json::from_str(&std::fs::read_to_string(std::env::var("SWBPIPE_FILL_MEASURE_MODEL").expect("SWBPIPE_FILL_MEASURE_MODEL")).unwrap()).unwrap();
    let mode = std::env::var("SWBPIPE_FILL_MEASURE_MODE").unwrap_or_else(|_| "sparse_interactive".to_string());
    // The named result, or the product's own solve of the model at this head
    // (the desktop's native request, `{model, materials: []}`).
    let result = match std::env::var("SWBPIPE_FILL_MEASURE_RESULT") {
        Ok(path) => std::fs::read_to_string(path).unwrap(),
        Err(_) => {
            let solver_mode = if mode == "dense_scrutiny" {
                open_pipe_stress_product_physics::PreviewSolverMode::DenseScrutiny
            } else {
                open_pipe_stress_product_physics::PreviewSolverMode::SparseInteractive
            };
            let envelope = open_pipe_stress_product_physics::run_linear_static_preview_value_with_mode(json!({"model": model, "materials": []}), solver_mode).unwrap();
            serde_json::to_string(&envelope).unwrap()
        }
    };
    let source: Value = serde_json::from_str(&result).unwrap();
    let manifest = open_pipe_stress_report_package::user_release::derive_input_manifest(&model, &source, &mode).unwrap();
    drop(source);
    let reference = Reference {
        name: "scratch".into(),
        report: Value::Null,
        result,
        intake: canonical_json(&model),
        manifest,
        mode,
        aggregate: Value::Null,
    };
    let live = live_job(&reference);
    let manifest_text = canonical_json(&reference.manifest);
    let peaks = derivation_peaks(&live.registry, &live.job, &reference.intake, &manifest_text, None);
    let growth = crate::report_render_exact::exact_render_growth_bytes(&reference.result, &reference.intake, &manifest_text).unwrap();
    let text = peaks.text as f64;
    eprintln!(
        "scratch: text {} | derivation peak requested {} ({:.2}x) priced {} ({:.2}x) reserved {} | render growth {} ({:.2}x)",
        peaks.text,
        peaks.derivation_requested,
        peaks.derivation_requested as f64 / text,
        peaks.derivation_priced,
        peaks.derivation_priced as f64 / text,
        peaks.derivation_reserved,
        growth,
        growth as f64 / text,
    );
    // The derivation's first part alone: the analysis-run record's build and
    // header (the rest of the peak is the Current result document's derivation).
    {
        let held = live.registry.held_result(&live.job).ok().expect("held");
        let mut source: Value = serde_json::from_str(held.text()).unwrap();
        let manifest_sha256 = crate::analysis_run_v03_build::hex(&crate::solve_jobs::sha256_in_place(manifest_text.as_bytes()));
        let model_ref = reference.manifest.pointer("/model_basis/model_ref").and_then(Value::as_str).unwrap().to_string();
        let evidence = crate::analysis_run_v03_build::ManifestArg {
            manifest_ref: crate::analysis_runs::input_manifest_ref(&model_ref, &manifest_sha256),
            manifest_sha256: manifest_sha256.clone(),
            canonical_bytes: manifest_text.clone(),
        };
        let hash = Value::String(manifest_sha256);
        let mut load_basis = Value::Null;
        let ((), measured) = crate::ingress::counting::measure(|| {
            let mut reserve = |_: u64| -> Result<Option<crate::memory_pool::CommandLease>, CommandError<String>> { Ok(None) };
            let built = crate::analysis_run_v03_build::preview_parsed(&held, &mut source, &evidence, &hash, &reference.manifest, None, &[], &mut load_basis, &mut reserve)
                .unwrap_or_else(|error| panic!("{}", message(error)));
            drop(crate::analysis_run_v03_build::header_record(&built));
        });
        eprintln!("scratch record build and header alone: peak requested {} ({:.2}x) priced {} ({:.2}x)", measured.peak_requested, measured.peak_requested as f64 / text, measured.peak_priced, measured.peak_priced as f64 / text);
    }
    // The whole fill (custody to the filled input) on the straight frame.
    let straight = request_reference("pressure_2_straight");
    let frame = frame_of(&straight.report, source_diagnostic_count(&straight.result));
    let args = custody_args(&live.job, &frame, &reference);
    let (filled, measured) = crate::ingress::counting::measure(|| fill_only(&live, &args).map(|_| ()).map_err(message));
    eprintln!("scratch fill_from_custody: {filled:?}; priced peak {} ({:.2}x)", measured.peak_priced, measured.peak_priced as f64 / text);
}

/// Finding 1 (c), the regression: on every committed reference, the
/// derivation's priced peak (its intermediates alive at once) is within the
/// reservation the command takes for it, and the whole command's priced peak
/// is within its grown lease, less the buffered body's envelope.
#[test]
fn the_derivation_stays_within_its_reservation_on_every_committed_reference() {
    use crate::report_native_fill::DERIVATION_RESERVE_FACTOR;
    for reference in every_reference() {
        let name = &reference.name;
        let live = live_job(&reference);
        let manifest_text = canonical_json(&reference.manifest);
        let peaks = derivation_peaks(&live.registry, &live.job, &reference.intake, &manifest_text, reference.aggregate.as_str());
        let reserved = peaks.text * DERIVATION_RESERVE_FACTOR + peaks.derivation_reserved;
        assert!(peaks.derivation_priced <= reserved, "{name}: the derivation's priced peak {} exceeds its reservation {reserved}", peaks.derivation_priced);
        let frame = frame_of(&reference.report, source_diagnostic_count(&reference.result));
        let (outcome, peak, lease, _) = command_peak(&live, &custody_args(&live.job, &frame, &reference));
        assert!(outcome.is_ok() || name.starts_with("user_release_exact/"), "{name}: {outcome:?}");
        assert!(peak <= lease, "{name}: the command's priced peak {peak} exceeds its lease {lease}");
        eprintln!(
            "{name}: text {} derivation priced {} ({:.1}x) reserved {} ({DERIVATION_RESERVE_FACTOR}x); command priced {peak} lease {lease}",
            peaks.text,
            peaks.derivation_priced,
            peaks.derivation_priced as f64 / peaks.text as f64,
            reserved
        );
    }
}

/// Finding 1 (c): a result the budget cannot reserve for is refused before any
/// parse, as the resource shortfall `ingress_exceeds_budget` (not a refusal of
/// the result's standing or binding): no result-sized allocation of the parse
/// or the derivation is made, and the lease is not grown.
#[test]
fn a_result_too_large_to_reserve_for_is_refused_before_the_derivation() {
    use crate::command_ingress::test_support::headers;
    use crate::command_ingress::{admit_body, INGRESS_ARGS_JSON_V1};
    use crate::memory_pool::ResourceRefusal;
    use crate::report_native_fill::{check_fill_size, DERIVATION_RESERVE_FACTOR};
    let reference = request_reference("pressure_2_straight");
    let live = live_job(&reference);
    let frame = frame_of(&reference.report, source_diagnostic_count(&reference.result));
    let args = custody_args(&live.job, &frame, &reference);
    let body = tauri::ipc::InvokeBody::Raw(serde_json::to_vec(&args).unwrap());
    let headers = headers(Some(INGRESS_ARGS_JSON_V1));
    let ingress = admit_body(&live.registry, &body, &headers, "render_calculation_report_from_custody", ArgsDecoding::Moved).unwrap().1.bytes();
    let manifest = canonical_json(&reference.manifest);
    let growth = crate::report_render_exact::exact_render_growth_bytes(&reference.result, &reference.intake, &manifest).unwrap();
    let text = reference.result.len() as u64;
    let required = ingress + growth + text * DERIVATION_RESERVE_FACTOR;
    // A budget one byte short of the ingress lease, the render's growth and the reservation.
    let budget = required - 1;
    let (pool_bytes, _) = live.registry.pool().settings();
    live.registry.configure_pool(pool_bytes, budget, false).unwrap();
    let (mut decoded, mut lease) = admit_body(&live.registry, &body, &headers, "render_calculation_report_from_custody", ArgsDecoding::Moved).unwrap();
    assert_eq!((lease.bytes(), lease.budget_bytes()), (ingress, budget));
    let (outcome, measured) = crate::ingress::counting::measure(|| crate::render_calculation_report_from_custody_args(&mut decoded, &live.registry, &mut lease));
    match outcome {
        Err(CommandError::Ingress(crate::solve_jobs::SolveJobError::ResourceUnavailable { refusal, .. })) => {
            assert_eq!(refusal, ResourceRefusal::IngressExceedsBudget { required_bytes: required, budget_bytes: budget });
        }
        other => panic!("not the resource refusal: {other:?}"),
    }
    assert_eq!(lease.bytes(), ingress, "the lease is not grown");
    // Nothing result-sized: the held text is not parsed (a parse alone is
    // several times the text) and the derivation never starts.
    assert!(measured.peak_requested < text, "a result-sized allocation before the refusal: {measured:?}, text {text}");
    // At exactly the budget the reservation fits.
    assert_eq!(check_fill_size(text, ingress + growth, required), Ok(text * DERIVATION_RESERVE_FACTOR));
}

/// Where the derivation's peak goes (proposal of a one-pass derivative hash,
/// review of f3d02469c): on the committed requests' Current result documents
/// (the webview's, whose hash the native derivation reproduces), the parsed
/// document's live bytes, its canonical text and each envelope member's share
/// of it, a clone's cost, the canonical string's cost and `validate_document`'s
/// peak; each against the held text. Printed; `--ignored --nocapture`.
#[test]
#[ignore = "measurement: the derivation's parts; run with --ignored --nocapture"]
fn the_derivation_s_parts_are_measured_on_the_committed_documents() {
    use open_pipe_stress_result_export::derivative::validate_document;
    for shape in SHAPES {
        let file = read_json(&format!("{REQUESTS}/{shape}.json"));
        let text = std::fs::read_to_string(project(file["raw_path"].as_str().unwrap())).unwrap();
        let t = text.len() as f64;
        let source: Value = serde_json::from_str(&text).unwrap();
        let document_text = serde_json::to_string(&file["request"]["result_documents"][0]).unwrap();
        let canonical = canonical_json(&file["request"]["result_documents"][0]);
        let (document, parsed) = crate::ingress::counting::measure(|| serde_json::from_str::<Value>(&document_text).unwrap());
        let ((), source_parsed) = crate::ingress::counting::measure(|| drop(std::hint::black_box(serde_json::from_str::<Value>(&text).unwrap())));
        let (copy, cloned) = crate::ingress::counting::measure(|| document.clone());
        drop(copy);
        let (outcome, validated) = crate::ingress::counting::measure(|| validate_document(&document, &source));
        outcome.unwrap();
        let ((), canonical_cost) = crate::ingress::counting::measure(|| drop(std::hint::black_box(canonical_json(&document))));
        let envelope = document["result_envelope"].as_object().unwrap();
        let mut members: Vec<(String, usize)> = envelope.iter().map(|(key, value)| (key.clone(), canonical_json(value).len())).collect();
        members.sort_by(|a, b| b.1.cmp(&a.1));
        let shares: Vec<String> = members.iter().take(8).map(|(key, bytes)| format!("{key} {:.1}x", *bytes as f64 / t)).collect();
        eprintln!(
            "{shape}: text {} | source parsed live {:.1}x priced peak {:.1}x | document canonical {:.1}x, parsed live {:.1}x (priced {:.1}x), clone priced {:.1}x, canonical string priced {:.1}x, validate_document priced peak {:.1}x | largest members: {}",
            text.len(),
            source_parsed.peak_requested as f64 / t,
            source_parsed.peak_priced as f64 / t,
            canonical.len() as f64 / t,
            parsed.live_requested as f64 / t,
            parsed.live_priced as f64 / t,
            cloned.peak_priced as f64 / t,
            canonical_cost.peak_priced as f64 / t,
            validated.peak_priced as f64 / t,
            shares.join(", "),
        );
    }
}

/// R1 (review of c2cc050ea): the frame's provenance record with its
/// `source_name` padded to `name_bytes` (the calculation report's and the
/// technical-preview entry's, which the web view writes as the same record).
fn with_source_name(frame: &Value, name_bytes: usize) -> Value {
    let mut frame = frame.clone();
    let name = "N".repeat(name_bytes);
    frame["calculation_report"]["provenance"]["source_name"] = json!(name);
    frame["report_sections"]["limitations"][0]["provenance"]["source_name"] = json!(name);
    frame
}

/// R1: the reviewer's input, a 1 MiB `source_name` on the straight frame, is
/// refused by the frame's bound before custody is read: nothing is replicated.
#[test]
fn a_frame_record_above_the_bound_is_refused_before_it_is_replicated() {
    use crate::report_native_fill::{FRAME_MEMBER_TOO_LARGE, MAX_FRAME_RECORD_BYTES};
    let reference = request_reference("pressure_2_straight");
    let live = live_job(&reference);
    let frame = with_source_name(&frame_of(&reference.report, source_diagnostic_count(&reference.result)), 1 << 20);
    let args = custody_args(&live.job, &frame, &reference);
    let (mut decoded, mut lease) = admit_json(&live.registry, "render_calculation_report_from_custody", ArgsDecoding::Moved, &args).unwrap();
    let admitted = lease.bytes();
    let (outcome, measured) = crate::ingress::counting::measure(|| crate::render_calculation_report_from_custody_args(&mut decoded, &live.registry, &mut lease));
    let error = message(outcome.unwrap_err());
    let record = serde_json::to_string(&frame["calculation_report"]["provenance"]).unwrap().len();
    assert_eq!(
        error,
        format!("{FRAME_MEMBER_TOO_LARGE}: the frame's calculation_report.provenance is {record} bytes; the render from custody admits at most {MAX_FRAME_RECORD_BYTES}, as it copies that member into every result reference, diagnostic or limitation entry.")
    );
    assert_eq!(lease.bytes(), admitted, "the lease is not grown");
    // Nothing replicated: less than four copies of the record (the frame's own
    // decode for the object-form check is the largest allocation).
    assert!(measured.peak_requested < 4 * record as u64, "{measured:?}");
    // The technical-preview entry's record is bounded the same way.
    let mut preview = frame_of(&reference.report, source_diagnostic_count(&reference.result));
    preview["report_sections"]["limitations"][0]["effect"]["note"] = json!("E".repeat(MAX_FRAME_RECORD_BYTES as usize));
    let error = message(render_from_custody(&live, &custody_args(&live.job, &preview, &reference)).unwrap_err());
    assert!(error.starts_with(&format!("{FRAME_MEMBER_TOO_LARGE}: the frame's report_sections.limitations[0].effect is ")), "{error}");
}

/// R1: a record just inside the bound, on the committed reference with the
/// most rows: the fill reserves its replication before it replicates, and the
/// whole command's priced peak stays within its grown lease.
#[test]
fn a_frame_record_at_the_bound_stays_within_its_replication_reservation() {
    use crate::report_native_fill::{largest_replicated_record, replication_reserve_bytes, MAX_FRAME_RECORD_BYTES};
    let reference = every_reference()
        .into_iter()
        .filter(|reference| !reference.name.starts_with("user_release_exact/"))
        .max_by_key(|reference| serde_json::from_str::<Value>(&reference.result).unwrap()["results"].as_array().unwrap().len())
        .unwrap();
    let name = reference.name.clone();
    let live = live_job(&reference);
    let frame = frame_of(&reference.report, source_diagnostic_count(&reference.result));
    let base = largest_replicated_record(&frame);
    let padding = (MAX_FRAME_RECORD_BYTES - base) as usize + frame["calculation_report"]["provenance"]["source_name"].as_str().unwrap().len();
    let padded = with_source_name(&frame, padding);
    let record = largest_replicated_record(&padded);
    assert_eq!(record, MAX_FRAME_RECORD_BYTES, "{name}: the record is at the bound");
    let source: Value = serde_json::from_str(&reference.result).unwrap();
    let (plain_outcome, plain_peak, plain_lease, _) = command_peak(&live, &custody_args(&live.job, &frame, &reference));
    let (outcome, peak, lease, _) = command_peak(&live, &custody_args(&live.job, &padded, &reference));
    assert!(plain_outcome.is_ok() && outcome.is_ok(), "{name}: {plain_outcome:?} {outcome:?}");
    let reserved = replication_reserve_bytes(&source, record) - replication_reserve_bytes(&source, base);
    eprintln!(
        "{name}: record {base} -> {record} bytes; priced peak {plain_peak} -> {peak} (+{}); lease {plain_lease} -> {lease} (+{}); the replication's added reservation {reserved}",
        peak.saturating_sub(plain_peak),
        lease - plain_lease,
    );
    assert!(peak <= lease, "{name}: the command's priced peak {peak} exceeds its lease {lease}");
    // The whole-command peak is the derivation's, which ends before the fill
    // replicates; so the replication is measured where it lives: the priced
    // bytes the filled input keeps, then the render of that input.
    let filled = |frame: &Value| {
        let args = custody_args(&live.job, frame, &reference);
        let (input, kept) = crate::ingress::counting::measure(|| fill_only(&live, &args).unwrap());
        let (outcome, render) = crate::ingress::counting::measure(|| crate::render_calculation_report_value(input.clone()).map(|_| ()));
        outcome.unwrap();
        (kept.live_priced as u64, render.peak_priced)
    };
    let ((plain_kept, plain_render), (kept, render)) = (filled(&frame), filled(&padded));
    let added = kept.saturating_sub(plain_kept) + render.saturating_sub(plain_render);
    eprintln!(
        "{name}: replicas kept in the filled input +{}, render peak +{}, together {added} against {reserved} reserved",
        kept.saturating_sub(plain_kept),
        render.saturating_sub(plain_render)
    );
    assert!(added <= reserved, "{name}: the record's replication added {added} priced bytes, more than the {reserved} reserved for it");
}

/// R5 (review of c2cc050ea): with a pool and invocation budget large enough
/// that the reservation would pass, a held result of about 70 MiB is refused by
/// the command's size limit, the bounded copy's 64 MiB, before the reservation.
#[test]
fn a_held_result_above_64_mib_is_refused_by_size_whatever_the_budget() {
    use crate::memory_pool::test_host::GIB;
    use crate::report_native_fill::{check_fill_size, check_size_limit, FILL_NOT_SERVED, MAX_CUSTODY_RENDER_RESULT_BYTES};
    let straight = request_reference("pressure_2_straight");
    let mut reference = request_reference("pressure_2_straight");
    // The straight result padded with trailing spaces to 70 MiB: valid JSON of that length.
    reference.result.push_str(&" ".repeat((70 << 20) - reference.result.len()));
    let text = reference.result.len() as u64;
    assert!(text > MAX_CUSTODY_RENDER_RESULT_BYTES);
    let host = crate::memory_pool::test_host::ScriptedHost::with(Some(512 * GIB), Some(512 * GIB), crate::memory_pool::PressureLevel::Normal);
    let (registry, gate) = gated(&host);
    registry.configure_pool(256 * GIB, 128 * GIB, true).unwrap();
    let job = held(&registry, &gate, &reference.result, &reference.intake, &reference.mode, ResultCustody::Native);
    let live = Live { registry, _gate: gate, job };
    let frame = frame_of(&straight.report, source_diagnostic_count(&straight.result));
    let args = custody_args(&live.job, &frame, &reference);
    let (mut decoded, mut lease) = admit_json(&live.registry, "render_calculation_report_from_custody", ArgsDecoding::Moved, &args).unwrap();
    let admitted = lease.bytes();
    // The reservation alone would pass under this budget.
    let growth = crate::report_render_exact::exact_render_growth_bytes(&reference.result, &reference.intake, &canonical_json(&reference.manifest)).unwrap();
    assert!(check_fill_size(text, admitted + growth, lease.budget_bytes()).is_ok(), "the budget would hold the reservation");
    let (outcome, measured) = crate::ingress::counting::measure(|| crate::render_calculation_report_from_custody_args(&mut decoded, &live.registry, &mut lease));
    let error = message(outcome.unwrap_err());
    assert_eq!(error, check_size_limit(text).unwrap_err());
    assert!(error.starts_with(&format!("{FILL_NOT_SERVED}: the result in custody is {text} bytes; the render from custody serves a result of at most {MAX_CUSTODY_RENDER_RESULT_BYTES} bytes")), "{error}");
    assert!(error.ends_with("This is a size limit of this command, not a fault of the result."), "{error}");
    assert_eq!(lease.bytes(), admitted, "nothing is reserved");
    assert!(measured.peak_requested < text / 4, "nothing result-sized before the refusal: {measured:?}");
    // At the limit it is not refused.
    assert_eq!(check_size_limit(MAX_CUSTODY_RENDER_RESULT_BYTES), Ok(()));
}

/// R8 (review of c2cc050ea): a pressure-free Timoshenko save request over the
/// stand-in, its model's material without provenance and with [`LIBRARY_REF`],
/// its sections as the desktop writes them (the technical-preview entry, the
/// identity's limitation 0, the result's Diagnostics, then `session`), the
/// section checksum rebound.
fn pf_save_request(session: &[Value]) -> (Value, Value, Value) {
    use open_pipe_stress_result_export::semantic_contract::PREVIEW_PHYSICS_TIMOSHENKO_ID;
    let mut model = crate::report_package_save::test_support::model();
    model["materials"][0].as_object_mut().unwrap().remove("provenance");
    model["materials"][0]["library_ref"] = serde_json::from_str(LIBRARY_REF).unwrap();
    let (reference, _) = pf_stand_in(model.clone());
    let mut model = model;
    model["model_settings"] = json!({"shear_model": "timoshenko_annulus_energy_v1"});
    let source: Value = serde_json::from_str(&reference.result).unwrap();
    let mut request = crate::report_package_save::test_support::desktop_request_from(&model, &reference.manifest, &source);
    let run = source["run_id"].as_str().unwrap().to_string();
    let sections = &mut request["report"]["report_sections"];
    let provenance = sections["provenance_notes"][0].clone();
    let mut technical = sections["limitations"][0].clone();
    technical["limitation_id"] = json!(format!("limitation:{run}:technical-preview"));
    sections["limitations"] = json!([technical.clone(), {
        "limitation_id": format!("limitation:{run}:pf-timoshenko:1"),
        "source": {"ref_type": "semantic_contract", "ref_id": PREVIEW_PHYSICS_TIMOSHENKO_ID},
        "affected_scope": {"ref_type": "analysis_run", "ref_id": run},
        "statement": source["formulation_basis"]["limitations"][0],
        "effect": technical["effect"], "provenance": technical["provenance"]}]);
    let mut diagnostics: Vec<Value> = expected_report_diagnostics(&source).unwrap().into_iter().map(|mut entry| { entry["provenance"] = provenance.clone(); entry }).collect();
    diagnostics.extend(session.iter().map(|entry| { let mut entry = entry.clone(); entry["provenance"] = provenance.clone(); entry }));
    sections["diagnostics"] = Value::Array(diagnostics);
    let checksum = open_pipe_stress_result_export::derivative::digest(&request["report"]["report_sections"]).unwrap();
    request["report"]["calculation_report"]["report_section_refs"][0]["checksum"]["value"] = json!(checksum);
    (request, model, reference.manifest)
}

/// R8: the local-private save of a pressure-free Timoshenko document admits the
/// session entries the host derives from its bound model, and refuses a copy
/// of a result diagnostic under other prose, a repeated entry and two findings
/// for one library reference site.
#[test]
fn the_pf_timoshenko_save_admits_only_the_session_entries_the_host_derives() {
    use crate::report_package_save::test_support::{no_store, save_request_with, software};
    let save = |session: &[Value]| {
        let (request, model, manifest) = pf_save_request(session);
        crate::report_package_save::admit_report_package_in(save_request_with(&request, &model, &manifest), &software(), &mut no_store, &|_: &str| None)
            .map(|admitted| admitted.package.export_blocked)
            .map_err(|error| (error.code, error.message))
    };
    let every = desktop_session_entries(&json!(null));
    let strip = |entry: &Value| { let mut entry = entry.clone(); entry.as_object_mut().unwrap().remove("provenance"); entry };
    let (material, not_held, changed) = (strip(&every[0]), strip(&every[1]), strip(&every[3]));
    assert_eq!(save(&[]), Ok(false), "the request without session entries is admitted unblocked");
    assert_eq!(save(&[material.clone(), changed.clone()]), Ok(false), "the derived session entries are admitted");
    let (request, _, _) = pf_save_request(&[]);
    let results = request["report"]["report_sections"]["diagnostics"].as_array().unwrap().len();
    let refusal = |index: usize, detail: &str| Err((SECTIONS.to_string(), format!("{SECTIONS} at /request/report/report_sections/diagnostics: diagnostics: report diagnostic {index} {detail}")));
    let mut copied = request["report"]["report_sections"]["diagnostics"][0].clone();
    copied.as_object_mut().unwrap().remove("provenance");
    copied["message"] = json!("Invented other prose.");
    assert_eq!(save(&[copied]), refusal(results, "copies a result diagnostic's code and affected object"));
    assert_eq!(save(&[material.clone(), material.clone()]), refusal(results + 1, "is not a session diagnostic the host derives from the bound model"));
    assert_eq!(save(&[changed.clone(), not_held.clone()]), refusal(results + 1, "is not a session diagnostic the host derives from the bound model"));
}
const SECTIONS: &str = crate::exact_source_binding::SECTIONS_MISMATCH;
