//! Object form on the native report paths (DEL-08-01 Design
//! `report-package-current-results.md` §2.6; found in Codex review rounds 2
//! and 3 of #1648). serde reads a struct written as a JSON array by field
//! order, while the save's disposition screen, its context checks and the
//! render's step 0 read members by key, so a provenance record written
//! positionally hid a `quarantined`, `rejected` or `protected_suspected`
//! disposition from the save's screen while the renderer read it, and an
//! unblocked package was saved. Both native commands now refuse a struct in
//! positional form at any depth before any keyed check, with its fixed path
//! only (`report_renderer::object_form`). These are the regressions on the
//! open pressure-free route, through the production roots, from the decoded
//! args and from the raw IPC body, with the legitimate-arrays control and the
//! pricing of the check. Invented data only. Standard claim fence applies
//! (claims taxonomy per DEC-081).

use crate::command_ingress::test_support::{admit_json, grown_lease_meets_the_budget, headers, registry};
use crate::command_ingress::{admit_body, respond, ArgsDecoding, CommandError, INGRESS_ARGS_JSON_V1};
use crate::ingress::counting::measure;
use crate::memory_pool::test_host::ScriptedHost;
use crate::report_package_save::test_support::{args_body, desktop_request, model, no_store, save_request, software};
use crate::report_package_save::{admit_report_package, save_report_package_args, ReportPackageSaveCommandError, REQUEST_POSITIONAL};
use crate::report_render_exact::RENDER_INPUT_POSITIONAL;
use open_pipe_stress_report_renderer::decode_renderable_report_input;
use open_pipe_stress_report_renderer::object_form::report_positional_struct_path;
use serde_json::{json, Value};
use tauri::ipc::InvokeBody;

const PROVENANCE_MEMBERS: [&str; 8] = [
    "source_name", "source_location", "source_license", "contributor", "contributor_certification", "redistribution_status",
    "review_status", "privacy_classification",
];

/// The dispositions the save's screen blocks in every context.
const DISPOSITIONS: [(&str, &str); 3] =
    [("review_status", "quarantined"), ("review_status", "rejected"), ("redistribution_status", "protected_suspected")];

/// The pressure-free desktop request (the committed DEL-10-05 witness, by
/// the shared basis builder) with one invented library reference, so that
/// every provenance-bearing collection of the report has an element.
fn request() -> Value {
    let mut request = desktop_request(&model());
    let provenance = request["report"]["report_sections"]["provenance_notes"][0].clone();
    request["report"]["library_references"] = json!([{
        "reference_id": "library-ref:invented-1", "library_kind": "material", "library_id": "invented-library",
        "library_version": "not recorded", "record_id": "invented-record", "entity_refs": ["material:invented"], "provenance": provenance
    }]);
    request
}

/// The provenance records of the report, at every depth the witness has:
/// the calculation report's, its model summary's, a load case's, a
/// result-export, audit and section reference's and a diagnostic's, and the
/// report sections' notes and limitations, and the library reference's.
const PATHS: [&str; 10] = [
    "/calculation_report/provenance",
    "/calculation_report/model_input_summary/provenance",
    "/calculation_report/load_case_summary/0/provenance",
    "/calculation_report/result_export_refs/0/provenance",
    "/calculation_report/audit_manifest_refs/0/provenance",
    "/calculation_report/report_section_refs/0/provenance",
    "/calculation_report/diagnostics/0/provenance",
    "/report_sections/provenance_notes/0",
    "/report_sections/limitations/0/provenance",
    "/library_references/0/provenance",
];

/// `report` with the record at `pointer` given `member` = `value` and, when
/// `positional`, written as its struct's positional form.
fn with_record(report: &mut Value, pointer: &str, member: &str, value: &str, positional: bool) {
    let record = report.pointer_mut(pointer).unwrap_or_else(|| panic!("{pointer}"));
    record[member] = json!(value);
    if positional {
        *record = Value::Array(PROVENANCE_MEMBERS.iter().map(|name| record.get(*name).cloned().unwrap_or(Value::Null)).collect());
    }
}

fn response_text(response: tauri::ipc::Response) -> String {
    use tauri::ipc::IpcResponse;
    match response.body().unwrap() {
        tauri::ipc::InvokeResponseBody::Json(text) => text,
        tauri::ipc::InvokeResponseBody::Raw(_) => panic!("raw response"),
    }
}

/// The production render of `input`, from the decoded args and from the raw
/// body (both must agree): the response text, or the refusal.
fn render_both(input: &Value) -> Result<String, String> {
    let registry = registry(ScriptedHost::normal());
    let args = json!({"input": input});
    let (mut decoded, mut lease) = admit_json(&registry, "render_calculation_report", ArgsDecoding::Moved, &args).unwrap();
    let admitted = lease.bytes();
    let first = respond(crate::render_calculation_report_args(&mut decoded, &registry, &mut lease)).map(response_text);
    if first.is_err() {
        assert_eq!(lease.bytes(), admitted, "a refusal grows no lease");
    }
    drop(lease);
    let body = InvokeBody::Raw(serde_json::to_vec(&args).unwrap());
    let (mut decoded, mut lease) =
        admit_body(&registry, &body, &headers(Some(INGRESS_ARGS_JSON_V1)), "render_calculation_report", ArgsDecoding::Moved).unwrap();
    let second = respond(crate::render_calculation_report_args(&mut decoded, &registry, &mut lease)).map(response_text);
    let text = |outcome: Result<String, CommandError>| match outcome {
        Ok(text) => Ok(text),
        Err(CommandError::Command(message)) => Err(message),
        Err(other) => Err(format!("{other:?}")),
    };
    let (first, second) = (text(first), text(second));
    assert_eq!(first, second, "decoded args and raw body agree");
    first
}

/// The save's admission of `request`, from the decoded request and from the
/// raw body (both must agree): the refusal code, or `admitted`.
fn save_both(request: &Value) -> String {
    let model = model();
    let code = |outcome: Result<crate::report_package_save::AdmittedReportPackage, ReportPackageSaveCommandError>| match outcome {
        Ok(admitted) => {
            assert!(!admitted.package.export_blocked);
            "admitted".to_string()
        }
        Err(error) => error.code,
    };
    let first = code(admit_report_package(save_request(request, &model), &software(), &mut no_store));
    let registry = registry(ScriptedHost::normal());
    let raw = serde_json::to_vec(&args_body(&save_request(request, &model))).unwrap();
    let body = InvokeBody::Raw(raw);
    let (mut decoded, _lease) =
        admit_body(&registry, &body, &headers(Some(INGRESS_ARGS_JSON_V1)), "save_report_package", ArgsDecoding::Moved).unwrap();
    let parsed = save_report_package_args(&mut decoded).unwrap();
    let second = code(admit_report_package(parsed, &software(), &mut no_store));
    assert_eq!(first, second, "decoded and raw body agree");
    first
}

/// The legitimate-arrays control: the genuine request, every array of which
/// is a collection, passes the comparison, renders and saves.
#[test]
fn a_genuine_pressure_free_request_and_render_input_pass_with_every_array_field() {
    let request = request();
    assert_eq!(report_positional_struct_path(&request["report"]), None);
    assert_eq!(open_pipe_stress_report_package::wire::request_positional_struct_path(&request), None);
    let rendered = render_both(&request["report"]).expect("the genuine input renders");
    assert!(rendered.contains("\"export_blocked\":false"), "{}", &rendered[..rendered.len().min(400)]);
    assert_eq!(save_both(&request), "admitted");
    // The committed DEL-10-05 runner witness too.
    let witness: Value = serde_json::from_str(include_str!("../../../../validation/witness/inputs/del1005_export_results_success_input.json")).unwrap();
    assert_eq!(report_positional_struct_path(&witness["export_results"]["report"]), None);
    assert_eq!(open_pipe_stress_report_package::wire::request_positional_struct_path(&witness["export_results"]), None);
}

/// Codex review round 3 of #1648 (blocking), live on the pressure-free route:
/// a `quarantined`, `rejected` or `protected_suspected` provenance record in
/// positional form, at every depth of the report, refuses the render and the
/// save before any keyed check, from the decoded args and the raw body; in
/// object form the save's screen refuses the same record.
#[test]
fn a_positional_provenance_record_refuses_the_render_and_the_save_at_every_depth() {
    let genuine = request();
    for (member, value) in DISPOSITIONS {
        for pointer in PATHS {
            let label = format!("{pointer} {member}={value}");
            let mut positional = genuine.clone();
            with_record(&mut positional["report"], pointer, member, value, true);
            // The renderer's decode reads the positional form.
            assert!(decode_renderable_report_input(positional["report"].clone()).is_ok(), "{label}: decodes");
            let refused = render_both(&positional["report"]).expect_err("the render refuses");
            assert!(refused.starts_with(&format!("{RENDER_INPUT_POSITIONAL}: ")), "{label}: {refused}");
            assert!(!refused.contains(value), "{label}: {refused}");
            assert_eq!(save_both(&positional), REQUEST_POSITIONAL, "{label}");
            let mut object = genuine.clone();
            with_record(&mut object["report"], pointer, member, value, false);
            let code = save_both(&object);
            assert_ne!(code, "admitted", "{label}: object form");
            assert_ne!(code, REQUEST_POSITIONAL, "{label}: object form");
        }
    }
}

/// A positional request, report or typed member refuses the save with the
/// same code; the positional request still decodes and is priced at ingress
/// (`report_package_region_pricing_covers_adversarial_requests`).
#[test]
fn a_positional_request_or_typed_member_refuses_the_save() {
    let genuine = request();
    let members = open_pipe_stress_report_package::wire::REQUEST_MEMBERS;
    let top = Value::Array(members.iter().map(|member| genuine.get(*member).cloned().unwrap_or(Value::Null)).collect());
    assert!(open_pipe_stress_report_package::wire::decode_report_package_request(top.clone()).is_ok());
    assert_eq!(save_both(&top), REQUEST_POSITIONAL, "the request");
    let report_members =
        ["report_title", "calculation_report", "report_sections", "result_rows", "report_context", "user_rule_checks", "exact_route", "library_references"];
    let mut report = genuine.clone();
    report["report"] = Value::Array(report_members.iter().map(|member| genuine["report"].get(*member).cloned().unwrap_or(Value::Null)).collect());
    assert!(decode_renderable_report_input(report["report"].clone()).is_ok());
    assert_eq!(save_both(&report), REQUEST_POSITIONAL, "the report");
    let mut reference = genuine.clone();
    reference["source_model_ref"] = json!([genuine["source_model_ref"]["ref_type"], genuine["source_model_ref"]["ref_id"]]);
    assert_eq!(save_both(&reference), REQUEST_POSITIONAL, "source_model_ref");
}

/// DEL-04-07 pricing: the check's by-reference decode is held only during
/// the check. Whole-command counting-allocator peaks on the pressure-free
/// witness, from the raw body, are within the ingress lease (the render) and
/// the parse lease with its W M provision (the save), less the buffered
/// body's envelope. Measured provisions for this input, not proven bounds.
#[test]
fn the_object_form_check_is_priced_within_the_existing_leases() {
    let request = request();
    // The render.
    let raw = serde_json::to_vec(&json!({"input": request["report"]})).unwrap();
    let envelope = crate::ingress::allocation_envelope(raw.len() as u64).unwrap();
    let body = InvokeBody::Raw(raw);
    let registry = registry(ScriptedHost::normal());
    let ((outcome, lease_bytes), measured) = measure(|| {
        let (mut decoded, mut lease) =
            admit_body(&registry, &body, &headers(Some(INGRESS_ARGS_JSON_V1)), "render_calculation_report", ArgsDecoding::Moved).unwrap();
        let outcome = respond(crate::render_calculation_report_args(&mut decoded, &registry, &mut lease)).map(response_text);
        drop(decoded);
        (outcome, lease.bytes())
    });
    assert!(outcome.is_ok());
    assert!(measured.peak_priced <= lease_bytes - envelope, "render: priced peak {} exceeds its lease {}", measured.peak_priced, lease_bytes - envelope);
    let (_, check) = measure(|| report_positional_struct_path(&request["report"]));
    eprintln!("pressure-free render: priced peak {} lease {}; the check alone {}", measured.peak_priced, lease_bytes - envelope, check.peak_priced);
    // The save, to its admission.
    let model = model();
    let args = save_request(&request, &model);
    let raw = serde_json::to_vec(&args_body(&args)).unwrap();
    let envelope = crate::ingress::allocation_envelope(raw.len() as u64).unwrap();
    let body = InvokeBody::Raw(raw);
    let ((admitted, lease_bytes), measured) = measure(|| {
        let (mut decoded, mut lease) =
            admit_body(&registry, &body, &headers(Some(INGRESS_ARGS_JSON_V1)), "save_report_package", ArgsDecoding::Moved).unwrap();
        let request = save_report_package_args(&mut decoded).unwrap();
        drop(decoded);
        let growth = crate::report_package_save::parse_lease_bytes(&request.request_text, &request.model_text, &request.input_manifest_text).unwrap();
        lease.grow_to(lease.bytes() + growth).unwrap();
        let admitted = admit_report_package(request, &software(), &mut no_store).map(|admitted| !admitted.package.export_blocked);
        (admitted, lease.bytes())
    });
    assert_eq!(admitted.ok(), Some(true));
    assert!(measured.peak_priced <= lease_bytes - envelope, "save: priced peak {} exceeds its lease {}", measured.peak_priced, lease_bytes - envelope);
    let (_, check) = measure(|| open_pipe_stress_report_package::wire::request_positional_struct_path(&request));
    eprintln!("pressure-free save: priced peak {} lease {}; the check alone {}", measured.peak_priced, lease_bytes - envelope, check.peak_priced);
}

/// The report-package save's normal path on the pressure-free witness, through
/// the command's admission and growth (`admit_report_package_save`), stays
/// within the per-invocation budget under the growth check
/// (`grown_lease_meets_the_budget`).
#[test]
fn the_pressure_free_save_grows_within_the_budget() {
    let body = InvokeBody::Raw(serde_json::to_vec(&args_body(&save_request(&request(), &model()))).unwrap());
    let headers = headers(Some(INGRESS_ARGS_JSON_V1));
    let registry = registry(ScriptedHost::normal());
    assert_eq!(registry.pool().settings().1, crate::memory_pool::DEFAULT_INVOCATION_BUDGET_BYTES);
    let ingress = admit_body(&registry, &body, &headers, "save_report_package", ArgsDecoding::Moved).unwrap().1.bytes();
    grown_lease_meets_the_budget(&registry, "pressure-free save", ingress, &|| match crate::admit_report_package_save(&registry, &body, &headers) {
        Ok((_, lease)) => Ok(lease.bytes()),
        Err(CommandError::Ingress(error)) => Err(error),
        Err(other) => panic!("the save failed otherwise: {other:?}"),
    });
}
