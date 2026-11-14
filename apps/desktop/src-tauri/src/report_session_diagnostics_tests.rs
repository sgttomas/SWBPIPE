//! The host's binding of the session's Diagnostics entries
//! (`report_session_diagnostics`) on the routes the desktop uses: the
//! existing render command (`render_calculation_report`, both bound
//! identities) and the local-private package save (`save_report_package`, on
//! exact and pressure-free Timoshenko documents).
//!
//! - An appended entry that copies a result diagnostic's code and affected
//!   object under other prose, in six forms, is refused by the render on the
//!   committed straight exact witness and by the save; an invented entry in a
//!   result diagnostic's form that copies none is refused as no session entry.
//! - On the pressure-free Timoshenko stand-in (a material without provenance
//!   carrying one library reference), the render and the save admit the
//!   session entries the host derives, and refuse an altered library message,
//!   a finding on another entity, a repeated entry and two findings for one
//!   reference site.
//! - The check alone: counts, the store check's preconditions per reference
//!   shape, one finding per site, the first unassignable entry, the derived
//!   kinds, the named limit, and its work, allocation count and priced peak on
//!   models with thousands of sites (review round 3 of c290410c6).
//! - The render and the save of the stand-in with session entries stay within
//!   their leases.
//!
//! Every id is invented.

use crate::command_ingress::test_support::{admit_json, headers};
use crate::command_ingress::{admit_body, ArgsDecoding, CommandError, INGRESS_ARGS_JSON_V1};
use crate::exact_source_binding::SECTIONS_MISMATCH;
use crate::ingress::counting::measure;
use crate::memory_pool::test_host::ScriptedHost;
use crate::report_package_save::test_support::{desktop_request_from, model as invented_model, no_store, save_request_with, software, witness_source};
use crate::report_package_save::{admit_report_package_in, parse_lease_bytes_with_exact, ReportPackageSaveRequest};
use crate::report_render_exact::PF_TIMOSHENKO_SECTION;
use crate::report_session_diagnostics::{
    check_session_diagnostics, diagnostics_text_bytes, session_check_provision_bytes, session_diagnostics, steps, LIBRARY_REF_CHECK_SOURCE,
    SESSION_CHECK_LIMIT,
};
use crate::solve_jobs::test_support::delivered_in_custody;
use crate::solve_jobs::SolveJobRegistry;
use open_pipe_stress_operation_applier::{canonical_json, sha256_hex};
use open_pipe_stress_report_package::bindings::expected_report_diagnostics;
use open_pipe_stress_result_export::derivative::digest;
use serde_json::{json, Value};
use std::path::Path;
use std::sync::Arc;
use tauri::ipc::InvokeBody;

const STRAIGHT: &str = include_str!("../../../../fixtures/reports/invented/exact_report_package_requests/pressure_2_straight.json");
const MODE: &str = "sparse_interactive";
const NOT_DERIVED: &str = "is not a session diagnostic the host derives from the bound model";
const COPIES: &str = "copies a result diagnostic's code and affected object";

fn project(path: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").join(path)
}

/// A live native-custody job holding a delivered result.
struct Live {
    registry: SolveJobRegistry,
    _gate: Arc<crate::solve_worker::test_support::Gate>,
    job: String,
}

fn live(intake: &str, result: &str) -> Live {
    let (registry, gate, job) = delivered_in_custody(ScriptedHost::normal(), intake.as_bytes(), Some(MODE), result);
    Live { registry, _gate: gate, job }
}

/// The existing render command through its production root.
fn render(live: &Live, input: &Value, result: &str, intake: &str, manifest: &str) -> Result<Value, CommandError> {
    let args = json!({"input": input, "exactResultText": result, "exactIntakeText": intake, "inputManifestText": manifest});
    let (mut decoded, mut lease) = admit_json(&live.registry, "render_calculation_report", ArgsDecoding::Moved, &args).unwrap();
    crate::render_calculation_report_args(&mut decoded, &live.registry, &mut lease)
}

fn message(outcome: Result<Value, CommandError>) -> String {
    match outcome {
        Ok(rendered) => panic!("rendered (blocked: {})", rendered["export_blocked"]),
        Err(CommandError::Command(message)) => message,
        Err(other) => format!("{other:?}"),
    }
}

fn refusal(index: usize, detail: &str) -> String {
    format!("{SECTIONS_MISMATCH}: diagnostics: report diagnostic {index} {detail}")
}

/// The save's admission: `Ok(export_blocked)` or the refusal's code and message.
fn save(live: Option<&Live>, args: ReportPackageSaveRequest) -> Result<bool, (String, String)> {
    let custody = |job: &str| live.and_then(|live| live.registry.custody_binding(job));
    admit_report_package_in(args, &software(), &mut no_store, &custody).map(|admitted| admitted.package.export_blocked).map_err(|error| (error.code, error.message))
}

fn save_refusal(index: usize, detail: &str) -> Result<bool, (String, String)> {
    Err((SECTIONS_MISMATCH.to_string(), format!("{SECTIONS_MISMATCH} at /request/report/report_sections/diagnostics: diagnostics: report diagnostic {index} {detail}")))
}

// ---------------------------------------------------------------------------
// The committed straight exact witness.
// ---------------------------------------------------------------------------

struct Exact {
    request: Value,
    model: Value,
    manifest: Value,
    intake: String,
    result: String,
}

fn exact_witness() -> Exact {
    let file: Value = serde_json::from_str(STRAIGHT).unwrap();
    Exact {
        request: file["request"].clone(),
        model: file["model"].clone(),
        manifest: file["input_manifest"].clone(),
        intake: file["exact_intake_text"].as_str().unwrap().to_string(),
        result: std::fs::read_to_string(project(file["raw_path"].as_str().unwrap())).unwrap(),
    }
}

/// The request bound to the live job.
fn exact_bound(w: &Exact, job: &str) -> Value {
    let mut request = w.request.clone();
    request["report"]["exact_route"]["admission"]["native_job_id"] = json!(job);
    request
}

fn exact_save_args(w: &Exact, request: &Value) -> ReportPackageSaveRequest {
    let mut args = save_request_with(request, &w.model, &w.manifest);
    args.exact_result_text = Some(w.result.clone());
    args.exact_intake_text = Some(w.intake.clone());
    args
}

/// The six forms of a result diagnostic's copy under other prose (the
/// reviewer's counterexample first), applied to `base`.
fn copies_of(base: &Value) -> Vec<(&'static str, Value)> {
    let session_form = vec![
        ("class", json!("PROVENANCE_WARNING")),
        ("source", json!({"ref_type": "component", "ref_id": "component:invented"})),
        ("affected_object", json!({"ref_type": "component", "ref_id": base["affected_object"]["ref_id"].clone()})),
    ];
    let mut session_tag = session_form.clone();
    session_tag.push(("diagnostic_origin", json!("session")));
    let forms: Vec<(&str, Vec<(&str, Value)>)> = vec![
        ("the reviewer's counterexample", vec![("remediation", json!("Review this load.")), ("message", json!("The load category is user selected."))]),
        ("the message changed only", vec![("message", json!("The load category is user selected."))]),
        ("the remediation and severity changed", vec![("remediation", json!("Review this load.")), ("severity", json!("info"))]),
        ("a session entry's class and reference types", session_form),
        ("a caller tag claiming session origin", session_tag),
        ("a caller tag claiming result origin", vec![("diagnostic_origin", json!("result")), ("message", json!("The load category is user selected."))]),
    ];
    forms
        .into_iter()
        .map(|(name, edits)| {
            let mut entry = base.clone();
            for (field, value) in edits {
                entry[field] = value;
            }
            (name, entry)
        })
        .collect()
}

/// Finding 2 (review of f3d02469c): the existing render refuses an appended
/// entry copying a result diagnostic under other prose, in each form, and an
/// invented entry in a result diagnostic's form that copies none.
#[test]
fn the_render_refuses_a_copy_of_a_result_diagnostic_under_other_prose() {
    let w = exact_witness();
    let live = live(&w.intake, &w.result);
    let report = exact_bound(&w, &live.job)["report"].clone();
    let manifest = canonical_json(&w.manifest);
    let count = report["report_sections"]["diagnostics"].as_array().unwrap().len();
    let base = report["report_sections"]["diagnostics"].as_array().unwrap().iter().find(|entry| entry["code"] == "LOAD_CATEGORY_PREVIEW_MAPPED").unwrap().clone();
    assert_eq!(base["affected_object"]["ref_id"], json!("load:fixture-global_x, case:six-component-load"));
    let render_with = |entry: Value| {
        let mut input = report.clone();
        input["report_sections"]["diagnostics"].as_array_mut().unwrap().push(entry);
        message(render(&live, &input, &w.result, &w.intake, &manifest))
    };
    for (name, entry) in copies_of(&base) {
        assert_eq!(render_with(entry), refusal(count, COPIES), "{name}");
    }
    let mut invented = base.clone();
    invented["code"] = json!("SOLVER_RESULT_VERIFIED");
    invented["affected_object"]["ref_id"] = json!("case:six-component-load");
    assert_eq!(render_with(invented), refusal(count, NOT_DERIVED));
    assert!(render(&live, &report, &w.result, &w.intake, &manifest).is_ok(), "the genuine input renders");
}

/// R8 (review of c2cc050ea): the local-private save of an exact document
/// refuses an appended entry that copies a result diagnostic under other
/// prose, in the six forms the render refuses, and one that copies none but
/// is not a session entry the host derives; the genuine request is admitted.
#[test]
fn the_save_refuses_a_forged_session_entry_on_an_exact_document() {
    let w = exact_witness();
    let live = live(&w.intake, &w.result);
    let genuine = exact_bound(&w, &live.job);
    let diagnostics = genuine["report"]["report_sections"]["diagnostics"].as_array().unwrap();
    let count = diagnostics.len();
    let base = diagnostics.iter().find(|entry| entry["code"] == "LOAD_CATEGORY_PREVIEW_MAPPED").unwrap().clone();
    let save_with = |entry: Value| {
        let mut request = genuine.clone();
        request["report"]["report_sections"]["diagnostics"].as_array_mut().unwrap().push(entry);
        save(Some(&live), exact_save_args(&w, &request))
    };
    for (name, entry) in copies_of(&base) {
        assert_eq!(save_with(entry), save_refusal(count, COPIES), "{name}");
    }
    let mut invented = base.clone();
    invented["code"] = json!("SOLVER_RESULT_VERIFIED");
    assert_eq!(save_with(invented), save_refusal(count, NOT_DERIVED));
    assert_eq!(save(Some(&live), exact_save_args(&w, &genuine)), Ok(false), "the genuine request is admitted unblocked");
}

// ---------------------------------------------------------------------------
// The pressure-free Timoshenko stand-in (as `report_render_pf_timoshenko_tests.rs`):
// the invented model selecting Timoshenko, its material without provenance and
// with [`LIBRARY_REF`], over the witness source with the identity swapped.
// ---------------------------------------------------------------------------

/// A library reference of the material (invented identities).
const LIBRARY_REF: &str = r#"{"library_kind": "material", "library_id": "library:invented", "record_id": "record:invented", "library_version": "sha256:0000000000000000000000000000000000000000000000000000000000000000"}"#;

fn pf_model() -> Value {
    let mut model = invented_model();
    model["materials"][0].as_object_mut().unwrap().remove("provenance");
    model["materials"][0]["library_ref"] = serde_json::from_str(LIBRARY_REF).unwrap();
    model["model_settings"] = json!({"shear_model": "timoshenko_annulus_energy_v1"});
    model
}

fn pf_source() -> Value {
    use open_pipe_stress_report_package::pf_timoshenko::REALIZED_ARCS_DIAGNOSTIC;
    use open_pipe_stress_result_export::semantic_contract::{preview_physics_timoshenko_contract, PREVIEW_PHYSICS_TIMOSHENKO_ID, PREVIEW_PHYSICS_TIMOSHENKO_PROFILE};
    let mut source = witness_source();
    source["producer"]["semantic_contract_id"] = json!(PREVIEW_PHYSICS_TIMOSHENKO_ID);
    source["formulation_basis"]["profile_id"] = json!(PREVIEW_PHYSICS_TIMOSHENKO_PROFILE);
    source["formulation_basis"]["limitations"] = preview_physics_timoshenko_contract()["supported_profile_limitations"].clone();
    let producer = source["diagnostics"][0]["source"].clone();
    source["diagnostics"].as_array_mut().unwrap().push(json!({
        "id": "diagnostic:preview-physics:pf-timoshenko:realized-arcs", "code": REALIZED_ARCS_DIAGNOSTIC,
        "severity": "info", "message": "1 straight member with shear deformation, 1 realized arc without: pipe:fixture-span",
        "source": producer, "affected_refs": ["pipe:fixture-span"]}));
    source
}

/// The desktop request over the stand-in, its sections as the desktop writes
/// them: the technical-preview entry, the identity's limitation 0, the
/// result's Diagnostics, then `session` (each with the session provenance).
fn pf_request(model: &Value, manifest: &Value, source: &Value, session: &[Value]) -> Value {
    use open_pipe_stress_result_export::semantic_contract::PREVIEW_PHYSICS_TIMOSHENKO_ID;
    let mut request = desktop_request_from(model, manifest, source);
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
    let mut diagnostics = expected_report_diagnostics(source).unwrap();
    diagnostics.extend(session.iter().cloned());
    for entry in &mut diagnostics {
        entry["provenance"] = provenance.clone();
    }
    sections["diagnostics"] = Value::Array(diagnostics);
    request
}

/// The pressure-free save request over the stand-in, its section checksum
/// rebound: `(request, model, manifest)`.
fn pf_save_args(session: &[Value]) -> ReportPackageSaveRequest {
    let (model, source) = (pf_model(), pf_source());
    let manifest = open_pipe_stress_report_package::user_release::derive_input_manifest(&model, &source, MODE).unwrap();
    let mut request = pf_request(&model, &manifest, &source, session);
    let checksum = digest(&request["report"]["report_sections"]).unwrap();
    request["report"]["calculation_report"]["report_section_refs"][0]["checksum"]["value"] = json!(checksum);
    save_request_with(&request, &model, &manifest)
}

/// The pressure-free render over the stand-in: a live job, the input with
/// its bound `pf_timoshenko` section, and the texts it submits.
struct PfRender {
    live: Live,
    input: Value,
    result: String,
    intake: String,
    manifest: String,
}

fn pf_render(session: &[Value]) -> PfRender {
    use open_pipe_stress_result_export::semantic_contract::PREVIEW_PHYSICS_TIMOSHENKO_ID;
    let (model, source) = (pf_model(), pf_source());
    let manifest = crate::report_package_save::test_support::input_manifest(&model, json!([]));
    let request = pf_request(&model, &manifest, &source, session);
    let result = serde_json::to_string(&source).unwrap();
    let intake = canonical_json(&model);
    let live = live(&intake, &result);
    let manifest_text = canonical_json(&manifest);
    let document = &request["result_documents"][0]["result_envelope"];
    let mut input = request["report"].clone();
    input[PF_TIMOSHENKO_SECTION] = json!({
        "semantic_contract_id": PREVIEW_PHYSICS_TIMOSHENKO_ID,
        "admission": {
            "semantic_contract_id": PREVIEW_PHYSICS_TIMOSHENKO_ID,
            "run_ref": source["run_id"],
            "result_envelope_id": document["envelope_id"],
            "received_result_sha256": digest(&source).unwrap(),
            "model_sha256": digest(&model).unwrap(),
            "input_manifest_ref": document["reproducibility"]["audit_manifest_ref"]["ref_id"],
            "input_manifest_sha256": sha256_hex(&manifest_text),
            "derivative_sha256": document["reproducibility"]["derivative_hash"]["value"],
            "native_job_id": live.job,
            "received_bytes_sha256": sha256_hex(&result),
        }});
    // DEL-08-01 REQ-024: the standing record the desktop's builder carries, as the reader
    // derives it from this source and model; the native host re-derives it and requires
    // equality.
    let invocation = json!({"request": {"model": model.clone(), "materials": []}, "solver_mode": MODE});
    let standing = open_pipe_stress_result_export::reader_verdict::standing(&source, Some(&model), Some(&invocation));
    input["numerical_standing"] = open_pipe_stress_report_standing::report_standing::historical_standing_record(&source, &model, &standing)
        .expect("the stand-in's standing record derives");
    PfRender { live, input, result, intake, manifest: manifest_text }
}

fn pf_rendered(case: &PfRender) -> Result<Value, CommandError> {
    render(&case.live, &case.input, &case.result, &case.intake, &case.manifest)
}

/// The session entries the desktop builds for the stand-in model, written
/// here from `renderableReportInput.ts` (`subjectProvenance`,
/// `libraryFindingDiagnostics`) and `lib.rs` `library_ref_findings`, not from
/// the host's derivation: the material's entry, then the three store findings
/// its one reference site may yield (library not held, record not held,
/// record changed).
fn desktop_session_entries() -> Vec<Value> {
    let project = "project:precision-connected-fixture";
    let entity = "material:fixture-isotropic";
    let library = |code: &str, message: String| json!({
        "code": code, "class": "PROVENANCE_WARNING", "severity": "warning",
        "source": {"ref_type": "diagnostic_source", "ref_id": "apps/desktop/src-tauri/src/lib.rs check_model_library_refs"},
        "affected_object": {"ref_type": "model_entity", "ref_id": entity},
        "message": message,
        "remediation": "Compare the library reference with the local library store before relying on this report.",
    });
    vec![
        json!({
            "code": "MATERIAL_PROVENANCE_MISSING", "class": "PROVENANCE_WARNING", "severity": "warning",
            "source": {"ref_type": "material", "ref_id": entity}, "affected_object": {"ref_type": "material", "ref_id": entity},
            "message": "Material fixture-isotropic has neither a provenance_record nor a provenance note; report provenance is incomplete for this material.",
            "remediation": "Enter material source/provenance before relying on this report.",
        }),
        library("MODEL-LIBRARY-REF-UNRESOLVED", format!("{entity} names material library `library:invented`, which the local store does not hold for project `{project}`. The model's values stand. A project saved under a new id reads its references as unresolved until the library is imported into that project.")),
        library("MODEL-LIBRARY-REF-UNRESOLVED", format!("{entity} names record `record:invented` of material library `library:invented`, which the stored library does not hold. The model's values stand.")),
        library("MODEL-LIBRARY-REF-VERSION-CHANGED", format!("The stored record `record:invented` of material library `library:invented` has changed since {entity} recorded it. The model's values and recorded version stand; re-apply from the library to adopt the stored record.")),
    ]
}

/// The session entries the desktop appends are admitted by the existing
/// render when the host derives them from the bound model, and refused when it
/// does not. Each derived kind's texts are also read from the desktop
/// builder's source, so a change there that the host's mirror does not follow
/// fails here, not in a user's render.
#[test]
fn the_pf_timoshenko_render_admits_only_the_session_entries_the_host_derives() {
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
    assert!(findings.contains(&format!("LIBRARY_REF_CHECK_SOURCE = \"{LIBRARY_REF_CHECK_SOURCE}\"")));

    let every = desktop_session_entries();
    let results = expected_report_diagnostics(&pf_source()).unwrap().len();
    // The material's entry and one store finding for its one reference site.
    let genuine = pf_render(&[every[0].clone(), every[3].clone()]);
    let rendered = pf_rendered(&genuine).unwrap_or_else(|error| panic!("{error:?}"));
    assert_eq!(rendered["export_blocked"], json!(false), "{}", rendered["blocking_reasons"]);
    // A library finding with a message the store check does not write, and one
    // on an entity without a library reference.
    for (name, field, value) in [
        ("an altered library message", "message", json!("The library is verified.")),
        ("another entity", "affected_object", json!({"ref_type": "model_entity", "ref_id": "support:fixture-root"})),
    ] {
        let mut forged = every[3].clone();
        forged[field] = value;
        assert_eq!(message(pf_rendered(&pf_render(&[every[0].clone(), forged]))), refusal(results + 1, NOT_DERIVED), "{name}");
    }
    // R3 and R4 (review of c2cc050ea): the material's entry twice for one
    // material, and two store findings for one reference site, each refused at
    // its third appended entry.
    for (name, session) in [
        ("the material's entry twice", [every[0].clone(), every[3].clone(), every[0].clone()]),
        ("two findings for one reference site", [every[0].clone(), every[3].clone(), every[1].clone()]),
    ] {
        assert_eq!(message(pf_rendered(&pf_render(&session))), refusal(results + 2, NOT_DERIVED), "{name}");
    }
    // A copy of a result diagnostic under other prose.
    let mut copied = expected_report_diagnostics(&pf_source()).unwrap()[0].clone();
    copied["message"] = json!("Invented other prose.");
    assert_eq!(message(pf_rendered(&pf_render(&[copied]))), refusal(results, COPIES));
}

/// R8: the local-private save of a pressure-free Timoshenko document admits
/// the session entries the host derives from its bound model, and refuses a
/// copy of a result diagnostic under other prose, a repeated entry and two
/// findings for one library reference site.
#[test]
fn the_pf_timoshenko_save_admits_only_the_session_entries_the_host_derives() {
    let every = desktop_session_entries();
    let (material, not_held, changed) = (every[0].clone(), every[1].clone(), every[3].clone());
    assert_eq!(save(None, pf_save_args(&[])), Ok(false), "the request without session entries is admitted unblocked");
    assert_eq!(save(None, pf_save_args(&[material.clone(), changed.clone()])), Ok(false), "the derived session entries are admitted");
    let results = expected_report_diagnostics(&pf_source()).unwrap();
    let mut copied = results[0].clone();
    copied["message"] = json!("Invented other prose.");
    assert_eq!(save(None, pf_save_args(&[copied])), save_refusal(results.len(), COPIES));
    assert_eq!(save(None, pf_save_args(&[material.clone(), material.clone()])), save_refusal(results.len() + 1, NOT_DERIVED));
    assert_eq!(save(None, pf_save_args(&[changed.clone(), not_held.clone()])), save_refusal(results.len() + 1, NOT_DERIVED));
}

/// The render and the save of the stand-in with its session entries, each
/// through the command's ingress admission: the counting allocator's priced
/// peak is within the command's lease, less the buffered body's envelope
/// (as `report_render_exact_tests.rs`
/// `the_genuine_exact_render_and_save_stay_within_their_leases`).
#[test]
fn the_render_and_the_save_with_session_entries_stay_within_their_leases() {
    let every = desktop_session_entries();
    let session = [every[0].clone(), every[3].clone()];
    let headers = headers(Some(INGRESS_ARGS_JSON_V1));

    let case = pf_render(&session);
    let raw = serde_json::to_vec(&json!({"input": case.input, "exactResultText": case.result, "exactIntakeText": case.intake, "inputManifestText": case.manifest})).unwrap();
    let envelope = crate::ingress::allocation_envelope(raw.len() as u64).unwrap();
    let body = InvokeBody::Raw(raw);
    let ((rendered, lease_bytes), measured) = measure(|| {
        let (mut decoded, mut lease) = admit_body(&case.live.registry, &body, &headers, "render_calculation_report", ArgsDecoding::Moved).unwrap();
        let rendered = crate::render_calculation_report_args(&mut decoded, &case.live.registry, &mut lease).map(|value| value["export_blocked"] == json!(false));
        drop(decoded);
        (rendered, lease.bytes())
    });
    assert_eq!(rendered.map_err(|error| format!("{error:?}")), Ok(true), "the render with session entries is admitted unblocked");
    assert!(measured.peak_priced <= lease_bytes - envelope, "render: priced peak {} exceeds its lease {}", measured.peak_priced, lease_bytes - envelope);

    let args = pf_save_args(&session);
    let raw = serde_json::to_vec(&crate::report_package_save::test_support::args_body(&args)).unwrap();
    let envelope = crate::ingress::allocation_envelope(raw.len() as u64).unwrap();
    let body = InvokeBody::Raw(raw);
    let ((blocked, lease_bytes), measured) = measure(|| {
        let (mut decoded, mut lease) = admit_body(&case.live.registry, &body, &headers, "save_report_package", ArgsDecoding::Moved).unwrap();
        let request = crate::report_package_save::save_report_package_args(&mut decoded).unwrap();
        drop(decoded);
        let growth = crate::report_package_save::ingress_lease_bytes(&request).unwrap();
        lease.grow_to(lease.bytes() + growth).unwrap();
        let blocked = save(None, request);
        (blocked, lease.bytes())
    });
    assert_eq!(blocked, Ok(false), "the save with session entries is admitted unblocked");
    assert!(measured.peak_priced <= lease_bytes - envelope, "save: priced peak {} exceeds its lease {}", measured.peak_priced, lease_bytes - envelope);
}

// ---------------------------------------------------------------------------
// The check alone.
// ---------------------------------------------------------------------------

/// The session binding alone, on `model`, for `appended` entries after a
/// result without diagnostics: `Ok`, or the refusal's detail.
fn session_check(model: &Value, appended: &[Value]) -> Result<(), String> {
    let input = json!({"report_sections": {"diagnostics": appended}});
    check_session_diagnostics(&input, &json!({"diagnostics": []}), model)
}

fn not_derived(index: usize) -> Result<(), String> {
    Err(format!("report diagnostic {index} {NOT_DERIVED}"))
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

fn not_held_on(entity: &str, kind: &str, library: &str, project: &str) -> Value {
    library_entry("MODEL-LIBRARY-REF-UNRESOLVED", entity, format!("{entity} names {kind} library `{library}`, which the local store does not hold for project `{project}`. The model's values stand. A project saved under a new id reads its references as unresolved until the library is imported into that project."))
}

fn record_not_held_on(entity: &str, record: &str, kind: &str, library: &str) -> Value {
    library_entry("MODEL-LIBRARY-REF-UNRESOLVED", entity, format!("{entity} names record `{record}` of {kind} library `{library}`, which the stored library does not hold. The model's values stand."))
}

fn changed_on(entity: &str, record: &str, kind: &str, library: &str) -> Value {
    library_entry("MODEL-LIBRARY-REF-VERSION-CHANGED", entity, format!("The stored record `{record}` of {kind} library `{library}` has changed since {entity} recorded it. The model's values and recorded version stand; re-apply from the library to adopt the stored record."))
}

fn component_entry(id: &str) -> Value {
    json!({
        "code": "COMPONENT_PROVENANCE_MISSING", "class": "PROVENANCE_WARNING", "severity": "warning",
        "source": {"ref_type": "component", "ref_id": id}, "affected_object": {"ref_type": "component", "ref_id": id},
        "message": format!("Component {id} has neither a provenance_record nor a component.provenance note; report provenance is incomplete for this component."),
        "remediation": "Enter component source/provenance before relying on this report.",
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
    assert_eq!(session_check(&one_material, &[material.clone(), material.clone()]), not_derived(1));
    let reference = json!({"library_kind": "component", "library_id": "library:invented", "record_id": "record:invented"});
    let twice = json!({"project": {"id": "project:invented"}, "components": [
        {"id": "component:twice", "provenance": "noted", "library_ref": reference, "geometry": {"library_ref": reference}}
    ]});
    let finding = not_held_on("component:twice", "component", "library:invented", "project:invented");
    assert_eq!(session_check(&twice, &[finding.clone(), finding.clone()]), Ok(()));
    assert_eq!(session_check(&twice, &[finding.clone(), finding.clone(), finding.clone()]), not_derived(2));
}

/// R4 (review of c2cc050ea): the store check's preconditions, per reference
/// shape, and at most one finding per reference site. The refused entry is
/// the first that the entries before it leave no site for, in either order.
#[test]
fn a_library_finding_is_admitted_only_where_the_store_check_can_make_it() {
    let entity = "material:invented";
    let not_held = |kind: &str, library: &str| not_held_on(entity, kind, library, "project:invented");
    let record_not_held = |record: &str| record_not_held_on(entity, record, "material", "library:invented");
    let changed = |record: &str| changed_on(entity, record, "material", "library:invented");
    let model = |reference: Value| json!({"project": {"id": "project:invented"}, "materials": [{"id": entity, "provenance": "noted", "library_ref": reference}]});
    let version = "sha256:0000000000000000000000000000000000000000000000000000000000000000";
    // A record and a version: any one of the three findings, never two.
    let full = model(json!({"library_kind": "material", "library_id": "library:invented", "record_id": "record:invented", "library_version": version}));
    for one in [not_held("material", "library:invented"), record_not_held("record:invented"), changed("record:invented")] {
        assert_eq!(session_check(&full, &[one]), Ok(()));
    }
    assert_eq!(session_check(&full, &[changed("record:invented"), not_held("material", "library:invented")]), not_derived(1));
    assert_eq!(session_check(&full, &[not_held("material", "library:invented"), changed("record:invented")]), not_derived(1));
    assert_eq!(session_check(&full, &[record_not_held("record:invented"), changed("record:invented")]), not_derived(1));
    // The reviewer's shape: a version and no record id. The store check returns
    // the record-not-held finding before any version comparison.
    let no_record = model(json!({"library_kind": "material", "library_id": "library:invented", "library_version": version}));
    assert_eq!(session_check(&no_record, &[record_not_held("(none recorded)")]), Ok(()));
    assert_eq!(session_check(&no_record, &[not_held("material", "library:invented")]), Ok(()));
    assert_eq!(session_check(&no_record, &[changed("")]), not_derived(0));
    // A blank record id is no record (`library_identity`).
    let blank_record = model(json!({"library_kind": "material", "library_id": "library:invented", "record_id": "  ", "library_version": version}));
    assert_eq!(session_check(&blank_record, &[changed("  ")]), not_derived(0));
    // A record and no version: no version comparison.
    let no_version = model(json!({"library_kind": "material", "library_id": "library:invented", "record_id": "record:invented"}));
    assert_eq!(session_check(&no_version, &[record_not_held("record:invented")]), Ok(()));
    assert_eq!(session_check(&no_version, &[changed("record:invented")]), not_derived(0));
    // A kind the store does not hold, and a blank library id: only "not held".
    let other_kind = model(json!({"library_kind": "pump", "library_id": "library:invented", "record_id": "record:invented", "library_version": version}));
    assert_eq!(session_check(&other_kind, &[not_held("pump", "library:invented")]), Ok(()));
    assert_eq!(session_check(&other_kind, &[record_not_held_on(entity, "record:invented", "pump", "library:invented")]), not_derived(0));
    let blank_library = model(json!({"library_kind": "material", "library_id": " ", "record_id": "record:invented", "library_version": version}));
    assert_eq!(session_check(&blank_library, &[not_held("material", " ")]), Ok(()));
    assert_eq!(session_check(&blank_library, &[record_not_held_on(entity, "record:invented", "material", " ")]), not_derived(0));
    // Two sites of one shape: two of the findings, one per site, in any mix.
    let two_sites = json!({"project": {"id": "project:invented"}, "materials": [{"id": entity, "provenance": "noted",
        "library_ref": {"library_kind": "material", "library_id": "library:invented", "record_id": "record:invented", "library_version": version},
        "grade": {"library_ref": {"library_kind": "material", "library_id": "library:invented", "record_id": "record:invented", "library_version": version}}}]});
    assert_eq!(session_check(&two_sites, &[changed("record:invented"), not_held("material", "library:invented")]), Ok(()));
    assert_eq!(session_check(&two_sites, &[record_not_held("record:invented"), changed("record:invented")]), Ok(()));
    assert_eq!(
        session_check(&two_sites, &[changed("record:invented"), not_held("material", "library:invented"), record_not_held("record:invented")]),
        not_derived(2)
    );
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
        "notes": [{"library_ref": {"library_kind": "pump", "library_id": "library:invented"}}],
    });
    let derived = session_diagnostics(&model);
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
        // A reference with no owning id is named by its path; the project id trimmed.
        line("MODEL-LIBRARY-REF-UNRESOLVED", "notes[0].library_ref", "notes[0].library_ref names pump library `library:invented`, which the local store does not hold for project `project:invented`. The model's values stand. A project saved under a new id reads its references as unresolved until the library is imported into that project."),
    ]);
    // Every derived entry is admitted, each once.
    assert_eq!(session_check(&model, &derived), Ok(()));
}

/// A section may carry the same reference twice: in its object-form provenance
/// note and in its provenance record. With both, the store check's two
/// identical findings are admitted. With one cleared, only one is: a second
/// is a finding for a reference the model no longer carries (independent
/// review of 7d6b8ffa2, finding 1). The desktop's filter of its published
/// findings applies the same rule (`libraryRefFindings.ts`
/// `currentLibraryRefFindings`), so the pair it now sends is admitted.
#[test]
fn a_repeated_reference_admits_one_finding_for_each_site_that_remains() {
    let (entity, project) = ("section:refs", "project:invented");
    let reference = json!({"library_kind": "section", "library_id": "seclib.invented.gone", "library_version": "sha256:s", "record_id": "sec.refs"});
    let both = json!({"project": {"id": project}, "sections": [{"id": entity, "provenance": {"note": "Invented", "library_ref": reference},
        "provenance_record": {"source_name": "Invented", "library_ref": reference}}]});
    let finding = not_held_on(entity, "section", "seclib.invented.gone", project);
    assert_eq!(session_check(&both, &[finding.clone(), finding.clone()]), Ok(()), "two sites, two findings");
    assert_eq!(session_check(&both, &[finding.clone(), finding.clone(), finding.clone()]), not_derived(2));
    let cleared = json!({"project": {"id": project}, "sections": [{"id": entity, "provenance": {"note": "Invented", "library_ref": reference}}]});
    assert_eq!(session_check(&cleared, std::slice::from_ref(&finding)), Ok(()), "the pair the desktop's filter now sends");
    assert_eq!(session_check(&cleared, &[finding.clone(), finding]), not_derived(1), "the stale duplicate is still refused");
}

/// Where reference fields spell one finding's text through different splits
/// (a backtick in a library or record id), sites can yield the same text in
/// different positions: an entry there is refused by name, as a limit of the
/// check, not as a fault of the report. The same text alone is still counted.
#[test]
fn a_finding_text_shared_across_reference_shapes_is_refused_by_name_as_a_limit() {
    let entity = "component:spelled";
    let project = "project:invented";
    // Site A: library `L`, record "r` of material library `M"; site B: library
    // "M` of material library `L", record "r". Both record-not-held texts read
    // "... record `r` of material library `M` of material library `L`, ...".
    let site_a = json!({"library_kind": "material", "library_id": "L", "record_id": "r` of material library `M"});
    let site_b = json!({"library_kind": "material", "library_id": "M` of material library `L", "record_id": "r"});
    let model = json!({"project": {"id": project}, "components": [{"id": entity, "provenance": "noted", "library_ref": site_a, "geometry": {"library_ref": site_b}}]});
    let shared = record_not_held_on(entity, "r` of material library `M", "material", "L");
    assert_eq!(shared, record_not_held_on(entity, "r", "material", "M` of material library `L"));
    let not_held_a = not_held_on(entity, "material", "L", project);
    assert_eq!(session_check(&model, &[shared.clone(), shared.clone()]), Ok(()), "two sites yield the shared text");
    assert_eq!(session_check(&model, &[shared.clone(), shared.clone(), shared.clone()]), not_derived(2));
    let limited = session_check(&model, &[not_held_a.clone(), shared.clone()]).unwrap_err();
    assert!(limited.starts_with(&format!("report diagnostic 0: {SESSION_CHECK_LIMIT}: ")), "{limited}");
    assert!(limited.ends_with("this is a limit of the check, not a fault of the report"), "{limited}");
    // An earlier entry no site yields is refused as such, before the limit.
    assert_eq!(session_check(&model, &[component_entry("component:other"), not_held_a, shared]), not_derived(0));
}

/// The check's work, allocation count and priced peak on thousands of
/// sites and entries, identical and distinct, against the work bound below
/// and the provision both routes make before the check
/// ([`session_check_provision_bytes`], the function the routes call). The
/// general matching this replaced compared every entry with every site and
/// kept a site list per entry; its first replacement kept a hash map of about
/// 540 bytes per entry, which exceeded the provision here (2,163,104 priced
/// bytes against 1,977,589 for 4,000 identical entries).
#[test]
fn the_matching_is_bounded_on_thousands_of_sites_and_entries() {
    const SITES: usize = 4000;
    let version = "sha256:0000000000000000000000000000000000000000000000000000000000000000";
    let reference = json!({"library_kind": "material", "library_id": "library:invented", "record_id": "record:invented", "library_version": version});
    let entity = "material:many";
    let identical_components = json!({"project": {"id": "project:invented"}, "components": vec![json!({"id": "component:same"}); SITES]});
    let distinct_components = json!({"project": {"id": "project:invented"},
        "components": (0..SITES).map(|index| json!({"id": format!("component:{index}")})).collect::<Vec<_>>()});
    let identical_references = json!({"project": {"id": "project:invented"},
        "materials": [{"id": entity, "provenance": "noted", "grades": vec![json!({"library_ref": reference}); SITES]}]});
    let third = SITES / 3;
    let mixed: Vec<Value> = std::iter::repeat_n(not_held_on(entity, "material", "library:invented", "project:invented"), third)
        .chain(std::iter::repeat_n(record_not_held_on(entity, "record:invented", "material", "library:invented"), third))
        .chain(std::iter::repeat_n(changed_on(entity, "record:invented", "material", "library:invented"), SITES - 2 * third))
        .collect();
    let cases: Vec<(&str, &Value, Vec<Value>, Result<(), String>)> = vec![
        ("identical entries, all placed", &identical_components, vec![component_entry("component:same"); SITES], Ok(())),
        ("identical entries, one too many", &identical_components, vec![component_entry("component:same"); SITES + 1], not_derived(SITES)),
        ("distinct entries, all placed", &distinct_components, (0..SITES).map(|index| component_entry(&format!("component:{index}"))).collect(), Ok(())),
        ("distinct entries, the last repeated", &distinct_components,
            (0..SITES).map(|index| component_entry(&format!("component:{}", index.min(SITES - 2)))).collect(), not_derived(SITES - 1)),
        ("alternatives at identical sites, all placed", &identical_references, mixed.clone(), Ok(())),
        ("alternatives at identical sites, one too many", &identical_references,
            mixed.iter().cloned().chain([changed_on(entity, "record:invented", "material", "library:invented")]).collect(), not_derived(SITES)),
    ];
    for (name, model, appended, expected) in cases {
        let entries = appended.len() as u64;
        let input = json!({"report_sections": {"diagnostics": appended}});
        let (outcome, measured, work, provision) = measured_check(&input, model);
        assert_eq!(outcome, expected, "{name}");
        let bound = work_bound(SITES as u64, entries, 0);
        eprintln!("{name}: {entries} entries, {SITES} sites: {work} steps (bound {bound}), {} allocations, priced peak {} B, provision {provision} B", measured.allocations, measured.peak_priced);
        assert!(work <= bound, "{name}: {work} steps exceed {bound}");
        // A fixed number of tables, none per site or entry.
        assert!(measured.allocations <= 64, "{name}: {} allocations", measured.allocations);
        assert!(measured.peak_priced <= provision, "{name}: priced peak {} exceeds the provision {provision}", measured.peak_priced);
    }
}

/// The check on `input`'s Diagnostics after a result without diagnostics,
/// measured: its outcome, the counting allocator's measure, its units of work,
/// and the provision both routes make for it before it runs.
fn measured_check(input: &Value, model: &Value) -> (Result<(), String>, crate::ingress::counting::Measured, u64, u64) {
    let source = json!({"diagnostics": []});
    let before = steps();
    let (outcome, measured) = measure(|| check_session_diagnostics(input, &source, model));
    let work = steps() - before;
    let provision = session_check_provision_bytes(canonical_json(model).len() as u64, diagnostics_text_bytes(input));
    (outcome, measured, work, provision)
}

/// The check's work bound (`report_session_diagnostics::place`): for the copy
/// check, one examination per appended entry and one lookup per result
/// diagnostic; for the matching, one examination and one lookup per entry, at
/// most three lookups per site, at most three visits per distinct key (at most
/// one per entry) and one pass over the entries: 3 S + 7 A + R. (The bound it
/// replaces was 3 S + 2 A + 2 A (2 + ceil(log2 A)): 132,000 at 4,000 sites and
/// entries; this one is 40,000 there.)
fn work_bound(sites: u64, entries: u64, results: u64) -> u64 {
    3 * sites + 7 * entries + results
}

/// The case that broke the first provision (property 1 of the Agent 0's
/// decision): thousands of the smallest entries the check indexes (an object
/// with the seven bound fields, every string empty: 150 bytes of compact text
/// with its comma, against at most 48 indexed bytes), identical and distinct;
/// thousands of the smallest entries the copy check indexes (a string code and
/// affected id: 44 bytes against at most 16); and thousands of entries the
/// check refuses before storing anything for them (`{}`, `0`, `""`: two or
/// three bytes each). The model is tiny, so the provision is the Diagnostics
/// text and the fixed part, and 20,000 entries put any storage of more than
/// about three bytes per refused entry above it. Each priced peak is within
/// the provision, by the function the routes call.
#[test]
fn many_minimal_entries_stay_within_the_provision() {
    const ENTRIES: usize = 20_000;
    let empty_session_form = json!({"code": "", "class": "", "severity": "",
        "source": {"ref_type": "", "ref_id": ""}, "affected_object": {"ref_type": "", "ref_id": ""}, "message": "", "remediation": ""});
    assert_eq!(serde_json::to_string(&empty_session_form).unwrap().len() + 1, 150, "the smallest indexed entry with its comma");
    let distinct_session_form = |index: usize| {
        let mut entry = empty_session_form.clone();
        entry["code"] = json!(index.to_string());
        entry
    };
    let copy_form = json!({"code": "", "affected_object": {"ref_id": ""}});
    assert_eq!(serde_json::to_string(&copy_form).unwrap().len() + 1, 44, "the smallest copy-check entry with its comma");
    let model = json!({"project": {"id": "project:invented"}});
    let cases: Vec<(&str, Vec<Value>)> = vec![
        ("the smallest indexed entry, identical", vec![empty_session_form.clone(); ENTRIES]),
        ("the smallest indexed entries, distinct", (0..ENTRIES).map(distinct_session_form).collect()),
        ("the smallest copy-check entry", vec![copy_form; ENTRIES]),
        ("empty objects", vec![json!({}); ENTRIES]),
        ("zeros", vec![json!(0); ENTRIES]),
        ("empty strings", vec![json!(""); ENTRIES]),
    ];
    for (name, appended) in cases {
        let entries = appended.len() as u64;
        let input = json!({"report_sections": {"diagnostics": appended}});
        let (outcome, measured, work, provision) = measured_check(&input, &model);
        // No site yields any of them: the first is refused.
        assert_eq!(outcome, not_derived(0), "{name}");
        let bound = work_bound(0, entries, 0);
        eprintln!("{name}: {entries} entries: {work} steps (bound {bound}), {} allocations, priced peak {} B, provision {provision} B", measured.allocations, measured.peak_priced);
        assert!(work <= bound, "{name}: {work} steps exceed {bound}");
        assert!(measured.peak_priced <= provision, "{name}: priced peak {} exceeds the provision {provision}", measured.peak_priced);
    }
}

/// The render's provision for the check (`report_render_exact::bind_render_admission`):
/// with a forged entry appended to the committed straight witness, a budget
/// that grants the ingress lease, the render's growth and the provision
/// exactly yields the check's refusal; one byte under it yields the pool's
/// refusal, with the lease unchanged by it and no unit of the check's work.
#[test]
fn the_render_grows_its_lease_by_the_provision_before_the_check() {
    let w = exact_witness();
    let live = live(&w.intake, &w.result);
    let mut input = exact_bound(&w, &live.job)["report"].clone();
    let count = input["report_sections"]["diagnostics"].as_array().unwrap().len();
    let mut forged = input["report_sections"]["diagnostics"][0].clone();
    forged["code"] = json!("SOLVER_RESULT_VERIFIED");
    forged["affected_object"]["ref_id"] = json!("case:invented");
    input["report_sections"]["diagnostics"].as_array_mut().unwrap().push(forged);
    let manifest = canonical_json(&w.manifest);
    let args = json!({"input": input, "exactResultText": w.result, "exactIntakeText": w.intake, "inputManifestText": manifest});
    let ingress = admit_json(&live.registry, "render_calculation_report", ArgsDecoding::Moved, &args).unwrap().1.bytes();
    let growth = crate::report_render_exact::exact_render_growth_bytes(&w.result, &w.intake, &manifest).unwrap();
    let provision = session_check_provision_bytes(w.intake.len() as u64, diagnostics_text_bytes(&input));
    let grown = ingress + growth + provision;
    let (pool_bytes, budget) = live.registry.pool().settings();
    assert!(grown <= budget, "the witness fits the default budget");
    let render_under = |budget_bytes: u64| {
        live.registry.configure_pool(pool_bytes, budget_bytes, false).unwrap();
        let (mut decoded, mut lease) = admit_json(&live.registry, "render_calculation_report", ArgsDecoding::Moved, &args).unwrap();
        let before = steps();
        let outcome = crate::render_calculation_report_args(&mut decoded, &live.registry, &mut lease);
        (outcome, lease.bytes(), steps() - before)
    };
    let (outcome, lease_bytes, work) = render_under(grown);
    assert_eq!(message(outcome), refusal(count, NOT_DERIVED), "at the grown lease the check runs and refuses");
    assert_eq!(lease_bytes, grown);
    assert!(work > 0);
    let (outcome, lease_bytes, work) = render_under(grown - 1);
    match outcome {
        Err(CommandError::Ingress(crate::solve_jobs::SolveJobError::ResourceUnavailable { refusal, .. })) => assert_eq!(
            refusal,
            crate::memory_pool::ResourceRefusal::IngressExceedsBudget { required_bytes: grown, budget_bytes: grown - 1 }
        ),
        other => panic!("one byte under: not the pool's refusal: {other:?}"),
    }
    assert_eq!(lease_bytes, ingress + growth, "the provision is not granted");
    assert_eq!(work, 0, "no unit of the check's work after the pool's refusal");
    live.registry.configure_pool(pool_bytes, budget, false).unwrap();
}

/// The save's provision for the check (`report_package_save::session_check_lease_bytes`,
/// added by `ingress_lease_bytes`, by which the save's ingress grows the lease
/// before the request is parsed): the growth with the term less the growth
/// without it (`parse_lease_bytes_with_exact`) is the model text plus the
/// request text plus the fixed part, and a budget one byte under the grown
/// lease is refused by the pool in the save's ingress, before the admission
/// parses anything.
#[test]
fn the_save_grows_its_lease_by_the_provision_before_the_parse() {
    use crate::command_ingress::test_support::grown_lease_meets_the_budget;
    use crate::command_ingress::text_parse_bytes;
    let w = exact_witness();
    let live = live(&w.intake, &w.result);
    let args = exact_save_args(&w, &exact_bound(&w, &live.job));
    let with = crate::report_package_save::ingress_lease_bytes(&args).unwrap();
    let without = parse_lease_bytes_with_exact(&args.request_text, &args.model_text, &args.input_manifest_text, Some(&w.result), Some(&w.intake)).unwrap();
    assert_eq!(
        without,
        crate::report_package_save::parse_lease_bytes(&args.request_text, &args.model_text, &args.input_manifest_text).unwrap()
            + text_parse_bytes(&w.result, ArgsDecoding::Moved).unwrap()
            + text_parse_bytes(&w.intake, ArgsDecoding::Moved).unwrap()
    );
    assert_eq!(with - without, session_check_provision_bytes(args.model_text.len() as u64, args.request_text.len() as u64));
    assert_eq!(with - without, args.model_text.len() as u64 + args.request_text.len() as u64 + (64 << 10));
    let mut raw = crate::report_package_save::test_support::args_body(&args);
    raw["exactResultText"] = json!(w.result);
    raw["exactIntakeText"] = json!(w.intake);
    let body = InvokeBody::Raw(serde_json::to_vec(&raw).unwrap());
    let headers = headers(Some(INGRESS_ARGS_JSON_V1));
    let ingress = admit_body(&live.registry, &body, &headers, "save_report_package", ArgsDecoding::Moved).unwrap().1.bytes();
    let (_, grown) = grown_lease_meets_the_budget(&live.registry, "exact save with the session provision", ingress, &|| {
        match crate::admit_report_package_save(&live.registry, &body, &headers) {
            Ok((_, lease)) => Ok(lease.bytes()),
            Err(CommandError::Ingress(error)) => Err(error),
            Err(other) => panic!("the save's ingress failed otherwise: {other:?}"),
        }
    });
    assert_eq!(grown, ingress + with, "the grown lease carries the provision");
}

/// No amplification from the model (review round 3 of c290410c6): 1,000
/// reference sites under a long project id, which every library-not-held text
/// names, with one appended finding. The derivation it replaces kept every
/// alternative of every site, with the project id copied into each (the
/// reviewer's 1,000 sites: 138,950,000 priced bytes from a 165,462 byte
/// model); this check's priced peak stays within the model's text plus the
/// report's Diagnostics.
#[test]
fn the_session_check_does_not_amplify_a_model_with_many_reference_sites() {
    const SITES: usize = 1000;
    let project = format!("project:{}", "p".repeat(45_000));
    let version = "sha256:0000000000000000000000000000000000000000000000000000000000000000";
    let model = json!({"project": {"id": project}, "materials": (0..SITES).map(|index| json!({
        "id": format!("material:{index}"), "provenance": "noted",
        "library_ref": {"library_kind": "material", "library_id": "library:invented", "record_id": "record:invented", "library_version": version}
    })).collect::<Vec<_>>()});
    let model_text = canonical_json(&model).len() as u64;
    let appended = vec![not_held_on("material:999", "material", "library:invented", &project)];
    let appended_text = serde_json::to_string(&appended).unwrap().len() as u64;
    let input = json!({"report_sections": {"diagnostics": appended}});
    let source = json!({"diagnostics": []});
    let (outcome, measured) = measure(|| check_session_diagnostics(&input, &source, &model));
    assert_eq!(outcome, Ok(()));
    // The bound is the provision both routes make before the check runs, by the same function.
    assert_eq!(diagnostics_text_bytes(&input), appended_text, "the counted Diagnostics text is its compact JSON length");
    let bound = session_check_provision_bytes(model_text, appended_text);
    assert_eq!(bound, model_text + appended_text + (64 << 10));
    eprintln!("{SITES} reference sites: model text {model_text} B, Diagnostics {appended_text} B, priced peak {} B (bound {bound}), {} allocations", measured.peak_priced, measured.allocations);
    assert!(measured.peak_priced <= bound, "priced peak {} exceeds {bound}", measured.peak_priced);
}
