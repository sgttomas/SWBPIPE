//! The version 2.0.0 report package over a current result (DEL-08-01 Design
//! `report-package-current-results.md` §2.3 and §4): the shared producer's
//! identity, form, binding, row and rule-check refusals, for the runner and the
//! desktop profiles, and determinism. All data invented; the bases are the
//! committed DEL-10-05 witness and its desktop-profile transformation by the
//! shared basis builder. Standard claim fence applies (claims taxonomy per
//! DEC-081).

mod common;
#[path = "common/package_basis.rs"]
mod package_basis;

use open_pipe_stress_canonical_json::canonical_json;
use open_pipe_stress_report_package::bindings::{
    BINDING_MISMATCH, DESKTOP_PROFILE, REPORT_CONTEXT_NOT_LOCAL_PRIVATE, REPORT_ROWS_MISMATCH,
    RULE_CHECK_BINDING_MISMATCH, RULE_PACK_CHECKSUM_BASIS_UNSUPPORTED, SOLVER_IDENTITY_MISMATCH, SOURCE_INVALID,
};
use open_pipe_stress_report_package::document_form::{
    CHECKSUM_SUBJECT_MISMATCH, DERIVATIVE_HASH_MISMATCH, RESULT_FORM_INVALID, ROUTE_NOT_ADMITTED,
};
use open_pipe_stress_report_package::wire::{assemble_wire_request, LinkedSolverIdentity, ReportPackageRequest};
use open_pipe_stress_report_package::{
    read_report_package_container, ReportPackageContainerOutcome, PACKAGE_MANIFEST_SCHEMA_VERSION, ROLE_REPORT_HTML,
    ROLE_REPORT_PDF, ROLE_RESULT_EXPORT,
};
use open_pipe_stress_result_export::semantic_contract::{
    PHYSICS_RETAINED_ID, PRESSURE_ID, PRESSURE_SOURCE_ID, PREVIEW_PHYSICS_RETAINED_ID,
};
use package_basis::{build, Inputs, Profile};
use serde_json::{json, Value};
use std::sync::OnceLock;

const MODEL: &str = include_str!("../../../../fixtures/model_operations/precision_connected_ui_model.json");
const INPUT_MANIFEST: &str = include_str!("../../../../fixtures/reports/invented/del1005_input_manifest.json");
const OTHER_HEX: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const RULE_PACK_HEX: &str = "abababababababababababababababababababababababababababababababab";

fn runner() -> Value {
    common::basis()["export_results"].clone()
}

fn desktop_with(run_override: Option<&str>) -> Value {
    desktop_from(run_override, |_| {})
}

/// The desktop-profile basis, derived by the shared builder from the
/// committed source after `edit_source`.
fn desktop_from(run_override: Option<&str>, edit_source: impl FnOnce(&mut Value)) -> Value {
    let manifest: Value = serde_json::from_str(INPUT_MANIFEST).unwrap();
    let manifest_id = runner()["audit_manifest"]["input_manifest_hash"]["payload_ref"].as_str().unwrap().to_string();
    desktop_basis(run_override, edit_source, &manifest, &manifest_id)
}

/// As [`desktop_from`], for the input manifest `manifest` named `manifest_id`.
fn desktop_basis(run_override: Option<&str>, edit_source: impl FnOnce(&mut Value), manifest: &Value, manifest_id: &str) -> Value {
    let runner = runner();
    let model: Value = serde_json::from_str(MODEL).unwrap();
    let solver = &runner["audit_manifest"]["solver_version"];
    let mut source = runner["source_results"][0].clone();
    edit_source(&mut source);
    let inputs = Inputs {
        model: &model,
        source: &source,
        producer_document: &runner["result_documents"][0],
        solve_request: &Value::Null,
        input_manifest: manifest,
        input_manifest_id: manifest_id,
        solver_name: solver["solver_name"].as_str().unwrap(),
        solver_version: solver["solver_version"].as_str().unwrap(),
        profile: Profile::Desktop,
        run_override,
    };
    build(&inputs).unwrap().export_results
}

fn desktop() -> Value {
    static DESKTOP: OnceLock<Value> = OnceLock::new();
    DESKTOP.get_or_init(|| desktop_with(None)).clone()
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

fn unblocked(request: &Value) -> ReportPackageContainerOutcome {
    let outcome = assemble(request).unwrap_or_else(|error| panic!("refused: {error}"));
    assert!(!outcome.export_blocked, "{:?}", outcome.blocking_reasons);
    outcome
}

#[track_caller]
fn refused(request: &Value, code: &str, case: &str) -> String {
    match assemble(request) {
        Ok(outcome) => panic!("{case}: assembled (blocked: {})", outcome.export_blocked),
        Err(error) => {
            assert!(error.starts_with(&format!("{code}:")), "{case}: expected {code}, got {error}");
            error
        }
    }
}

/// The request with its result document edited and its derivative hash
/// recomputed, so only the check the edit targets can refuse it.
fn with_document(request: &Value, edit: impl FnOnce(&mut Value)) -> Value {
    let mut request = request.clone();
    edit(&mut request["result_documents"][0]);
    request["result_documents"][0] = common::rehash(request["result_documents"][0].take());
    request
}

fn with(request: &Value, edit: impl FnOnce(&mut Value)) -> Value {
    let mut request = request.clone();
    edit(&mut request);
    request
}

fn at<'a>(value: &'a mut Value, pointer: &str) -> &'a mut Value {
    value.pointer_mut(pointer).unwrap_or_else(|| panic!("no {pointer}"))
}

fn member<'a>(outcome: &'a ReportPackageContainerOutcome, role: &str) -> &'a [u8] {
    &outcome.members.iter().find(|member| member.role == role).unwrap().bytes
}

// ---------------------------------------------------------------------------
// Success and determinism.
// ---------------------------------------------------------------------------

#[test]
fn current_result_packages_assemble_read_back_and_are_deterministic() {
    for (profile, request) in [("runner", runner()), ("desktop", desktop())] {
        let first = unblocked(&request);
        let second = unblocked(&request);
        assert_eq!(first.container_bytes, second.container_bytes, "{profile}: not deterministic");
        let read = read_report_package_container(&first.container_bytes).unwrap_or_else(|error| panic!("{profile}: {error}"));
        assert_eq!(read.schema_version, PACKAGE_MANIFEST_SCHEMA_VERSION);
        assert_eq!(PACKAGE_MANIFEST_SCHEMA_VERSION, "2.0.0");
        assert!(!read.export_blocked);
        assert_eq!(read.export_profile_id, request["export_profile_id"].as_str().unwrap());
        let result = read.members.iter().find(|member| member.role == ROLE_RESULT_EXPORT).unwrap();
        assert!(
            result.bytes == canonical_json(&request["result_documents"][0]).into_bytes(),
            "{profile}: the result member is not the document's JCS form"
        );
    }
}

#[test]
fn the_desktop_basis_carries_the_received_current_origin() {
    let request = desktop();
    assert_eq!(request["export_profile_id"], DESKTOP_PROFILE);
    let e = &request["result_documents"][0]["result_envelope"];
    assert_eq!(e["reproducibility"]["source_origin_bindings"][0]["origin_class"], "received_current_dimension_absent");
    assert_eq!(e["run_ref"]["ref_id"], request["source_results"][0]["run_id"]);
}

// ---------------------------------------------------------------------------
// Identity, version and form.
// ---------------------------------------------------------------------------

#[test]
fn non_admitted_identities_are_refused_by_the_preflight_before_any_form_error() {
    let ids: [Option<&str>; 6] = [
        Some(PRESSURE_ID),
        Some(PRESSURE_SOURCE_ID),
        Some(PREVIEW_PHYSICS_RETAINED_ID),
        Some(PHYSICS_RETAINED_ID),
        Some("openpipestress.result_semantics/0.3.0/invented-unknown"),
        None,
    ];
    for request in [runner(), desktop()] {
        for id in ids {
            let edited = with_document(&request, |document| {
                let producer = at(document, "/result_envelope/producer").as_object_mut().unwrap();
                match id {
                    Some(id) => {
                        producer.insert("semantic_contract_id".into(), json!(id));
                    }
                    None => {
                        producer.remove("semantic_contract_id");
                    }
                }
                // A form defect too: the route code still comes first.
                document["result_envelope"].as_object_mut().unwrap().remove("result_sets");
            });
            refused(&edited, ROUTE_NOT_ADMITTED, &format!("{id:?}"));
        }
    }
}

#[test]
fn a_semantic_contract_ref_other_than_the_producers_is_refused() {
    let edited = with_document(&runner(), |document| {
        *at(document, "/result_envelope/semantic_contract_ref/ref_id") = json!(PRESSURE_ID);
    });
    let error = assemble(&edited).unwrap_err();
    assert!(error.starts_with("REPORT-PACKAGE-RESULT-"), "{error}");
}

#[test]
fn mixed_result_schema_versions_are_refused() {
    for pointer in ["/schema_version", "/result_envelope/schema_version"] {
        let edited = with_document(&runner(), |document| *at(document, pointer) = json!("0.1.0"));
        refused(&edited, RESULT_FORM_INVALID, pointer);
    }
}

#[test]
fn form_mutations_with_a_recomputed_hash_are_refused_by_the_form_check() {
    type Edit = fn(&mut Value);
    let cases: [(&str, Edit); 12] = [
        ("compliance claim", |d| *at(d, "/result_envelope/professional_boundary/software_makes_compliance_claim") = json!(true)),
        ("human review", |d| *at(d, "/result_envelope/professional_boundary/human_review_required") = json!(false)),
        ("ordering", |d| *at(d, "/result_envelope/reproducibility/deterministic_ordering") = json!(false)),
        ("export_format_status", |d| {
            d.as_object_mut().unwrap().remove("export_format_status");
        }),
        ("objectives", |d| {
            d.as_object_mut().unwrap().remove("objectives");
        }),
        ("result_sets", |d| {
            d["result_envelope"].as_object_mut().unwrap().remove("result_sets");
        }),
        ("row_accounting", |d| {
            d["result_envelope"].as_object_mut().unwrap().remove("row_accounting");
        }),
        ("run_ref", |d| {
            d["result_envelope"].as_object_mut().unwrap().remove("run_ref");
        }),
        ("unknown status", |d| at(d, "/result_envelope/analysis_status").as_array_mut().unwrap().push(json!("INVENTED_STATUS"))),
        ("two rule tokens", |d| at(d, "/result_envelope/analysis_status").as_array_mut().unwrap().push(json!("USER_RULE_CHECKED"))),
        ("incomplete provenance", |d| *at(d, "/result_envelope/provenance/source_name") = json!(" ")),
        ("header only", |d| {
            let e = &d["result_envelope"];
            let header = json!({
                "schema_version": e["schema_version"],
                "envelope_id": e["envelope_id"],
                "producer": e["producer"],
                "semantic_contract_ref": e["semantic_contract_ref"],
                "reproducibility": {"derivative_hash": e["reproducibility"]["derivative_hash"]}
            });
            d["result_envelope"] = header;
        }),
    ];
    for request in [runner(), desktop()] {
        for (case, edit) in cases {
            refused(&with_document(&request, edit), RESULT_FORM_INVALID, case);
        }
    }
}

#[test]
fn tampered_documents_are_refused() {
    // A value edit with a recomputed hash: the source accounting refuses it.
    let rehashed = with_document(&runner(), |document| {
        let magnitude = at(document, "/result_envelope/result_sets/0/values/0/magnitude");
        *magnitude = json!(magnitude.as_f64().unwrap() + 1.0);
    });
    refused(&rehashed, SOURCE_INVALID, "rehashed value edit");
    // The same edit without the hash: the self-hash refuses it.
    let stale = with(&runner(), |request| {
        let magnitude = at(request, "/result_documents/0/result_envelope/result_sets/0/values/0/magnitude");
        *magnitude = json!(magnitude.as_f64().unwrap() + 1.0);
    });
    refused(&stale, DERIVATIVE_HASH_MISMATCH, "stale hash");
}

// ---------------------------------------------------------------------------
// Cross-member substitution (§2.3 item 5).
// ---------------------------------------------------------------------------

type RequestEdit = fn(&mut Value);

fn binding_cases() -> Vec<(&'static str, RequestEdit, &'static str)> {
    const SUMMARY: &str = "/report/calculation_report/model_input_summary";
    let _ = SUMMARY;
    vec![
        // Model.
        ("source_model_ref", |r| *at(r, "/source_model_ref/ref_id") = json!("invented-other"), BINDING_MISMATCH),
        ("sections model_ref", |r| *at(r, "/report/report_sections/model_ref/ref_id") = json!("invented-other"), BINDING_MISMATCH),
        ("summary model_ref", |r| *at(r, "/report/calculation_report/model_input_summary/model_ref/ref_id") = json!("invented-other"), BINDING_MISMATCH),
        ("summary project_ref", |r| *at(r, "/report/calculation_report/model_input_summary/project_ref/ref_id") = json!("invented-other"), BINDING_MISMATCH),
        ("audit model payload", |r| *at(r, "/audit_manifest/model_hash/payload_ref") = json!("invented-other"), BINDING_MISMATCH),
        ("audit model value", |r| *at(r, "/audit_manifest/model_hash/value") = json!(OTHER_HEX), BINDING_MISMATCH),
        ("audit from another model", |r| {
            *at(r, "/audit_manifest/model_hash/payload_ref") = json!("invented-other");
            *at(r, "/audit_manifest/model_hash/value") = json!(OTHER_HEX);
        }, BINDING_MISMATCH),
        ("audit model basis", |r| *at(r, "/audit_manifest/model_hash/canonicalization") = json!("project_local_deterministic_json"), BINDING_MISMATCH),
        ("summary model hash value", |r| *at(r, "/report/calculation_report/model_input_summary/model_hash/value") = json!(OTHER_HEX), BINDING_MISMATCH),
        ("summary model hash reference", |r| *at(r, "/report/calculation_report/model_input_summary/model_hash/payload_ref/ref_type") = json!("model_payload"), BINDING_MISMATCH),
        ("summary model hash basis", |r| *at(r, "/report/calculation_report/model_input_summary/model_hash/canonicalization") = json!("project_local_deterministic_json"), BINDING_MISMATCH),
        ("swapped manifest_id", |r| *at(r, "/audit_manifest/manifest_id") = json!("audit-manifest:invented-other"), BINDING_MISMATCH),
        // Run.
        ("source_basis_refs run", |r| *at(r, "/source_basis_refs/0/ref_id") = json!("run:invented-other"), BINDING_MISMATCH),
        ("source_basis_refs load dropped", |r| {
            at(r, "/source_basis_refs").as_array_mut().unwrap().pop();
        }, BINDING_MISMATCH),
        ("sections run_ref", |r| *at(r, "/report/report_sections/run_ref/ref_id") = json!("run:invented-other"), BINDING_MISMATCH),
        ("report run-record ref", |r| *at(r, "/report/calculation_report/audit_manifest_refs/0/ref/ref_id") = json!("run:invented-other"), BINDING_MISMATCH),
        ("report run-record checksum", |r| *at(r, "/report/calculation_report/audit_manifest_refs/0/checksum/value") = json!(OTHER_HEX), BINDING_MISMATCH),
        ("report received checksum", |r| *at(r, "/report/calculation_report/audit_manifest_refs/1/checksum/value") = json!(OTHER_HEX), BINDING_MISMATCH),
        ("report received checksum basis", |r| *at(r, "/report/calculation_report/audit_manifest_refs/1/checksum/canonicalization") = json!("jcs_compatible"), BINDING_MISMATCH),
        ("result_export_refs row checksum", |r| *at(r, "/report/calculation_report/result_export_refs/0/checksum/value") = json!(OTHER_HEX), BINDING_MISMATCH),
        ("result_export_refs row dropped", |r| {
            at(r, "/report/calculation_report/result_export_refs").as_array_mut().unwrap().pop();
        }, BINDING_MISMATCH),
        ("source carrier from another run", |r| *at(r, "/source_results/0/run_id") = json!("run:invented-other"), SOURCE_INVALID),
        // Handoff.
        ("two handoff records", |r| {
            let record = r["state_comparison_handoff_records"][0].clone();
            at(r, "/state_comparison_handoff_records").as_array_mut().unwrap().push(record);
        }, BINDING_MISMATCH),
        ("handoff section_set_id", |r| *at(r, "/state_comparison_handoff_records/0/section_set_id") = json!("invented-session:run:other"), BINDING_MISMATCH),
        ("handoff state ref", |r| *at(r, "/state_comparison_handoff_records/0/sections/state_run_sections/0/state_ref/ref") = json!("state:invented-other"), BINDING_MISMATCH),
        ("handoff state section id", |r| *at(r, "/state_comparison_handoff_records/0/sections/state_run_sections/0/section_id") = json!("state-run:invented-other"), BINDING_MISMATCH),
        ("handoff state hash of another model", |r| *at(r, "/state_comparison_handoff_records/0/sections/state_run_sections/0/hash_refs/0/value") = json!(OTHER_HEX), BINDING_MISMATCH),
        ("handoff run state ref", |r| *at(r, "/state_comparison_handoff_records/0/sections/state_run_sections/1/state_ref/ref") = json!("state:invented-other"), BINDING_MISMATCH),
        ("handoff run section id", |r| *at(r, "/state_comparison_handoff_records/0/sections/state_run_sections/1/section_id") = json!("state-run:invented-other"), BINDING_MISMATCH),
        ("handoff run ref", |r| *at(r, "/state_comparison_handoff_records/0/sections/state_run_sections/1/run_ref/ref") = json!("run:invented-other"), BINDING_MISMATCH),
        ("handoff another run's record hash", |r| *at(r, "/state_comparison_handoff_records/0/sections/state_run_sections/1/hash_refs/0/payload_ref/ref") = json!("run:invented-other"), BINDING_MISMATCH),
        ("handoff record hash value", |r| *at(r, "/state_comparison_handoff_records/0/sections/state_run_sections/1/hash_refs/0/value") = json!(OTHER_HEX), BINDING_MISMATCH),
        ("handoff hash dropped", |r| {
            at(r, "/state_comparison_handoff_records/0/sections/state_run_sections/1/hash_refs").as_array_mut().unwrap().pop();
        }, BINDING_MISMATCH),
        ("handoff row hash", |r| *at(r, "/state_comparison_handoff_records/0/sections/state_run_sections/1/result_refs/0/hash_refs/0/value") = json!(OTHER_HEX), BINDING_MISMATCH),
        ("handoff load dropped", |r| {
            at(r, "/state_comparison_handoff_records/0/sections/state_run_sections/1/load_basis_refs").as_array_mut().unwrap().pop();
        }, BINDING_MISMATCH),
        ("handoff duplicated current section", |r| {
            let section = r["state_comparison_handoff_records"][0]["sections"]["state_run_sections"][1].clone();
            at(r, "/state_comparison_handoff_records/0/sections/state_run_sections").as_array_mut().unwrap().push(section);
        }, BINDING_MISMATCH),
        ("handoff missing current section", |r| {
            at(r, "/state_comparison_handoff_records/0/sections/state_run_sections").as_array_mut().unwrap().remove(0);
        }, BINDING_MISMATCH),
        ("handoff input manifest ref", |r| *at(r, "/state_comparison_handoff_records/0/sections/state_run_sections/1/reproducibility/input_manifest_refs/0/ref") = json!("input-manifest:invented-other"), BINDING_MISMATCH),
        ("handoff input manifest hash", |r| *at(r, "/state_comparison_handoff_records/0/sections/state_run_sections/1/reproducibility/input_manifest_hashes/0/value") = json!(OTHER_HEX), BINDING_MISMATCH),
        // Solver.
        ("report context solver version", |r| *at(r, "/report/report_context/solver_version") = json!("0.0.1"), SOLVER_IDENTITY_MISMATCH),
        ("report context solver_build_ref", |r| *at(r, "/report/report_context/solver_build_ref") = json!("invented-build"), BINDING_MISMATCH),
        ("audit solver_build_ref", |r| *at(r, "/audit_manifest/solver_version/solver_build_ref") = json!("invented-build"), SOLVER_IDENTITY_MISMATCH),
        // Input manifest.
        ("audit input manifest value", |r| *at(r, "/audit_manifest/input_manifest_hash/value") = json!(OTHER_HEX), BINDING_MISMATCH),
        ("summary input manifest ref", |r| *at(r, "/report/calculation_report/model_input_summary/input_manifest_ref/ref_id") = json!("input-manifest:invented-other"), BINDING_MISMATCH),
        // Units and display policy.
        ("audit unit system", |r| *at(r, "/audit_manifest/unit_system_ref") = json!("invented-other:units"), BINDING_MISMATCH),
        ("summary unit_system_ref", |r| *at(r, "/report/calculation_report/model_input_summary/unit_system_ref/ref_id") = json!("unit-system:invented-other"), BINDING_MISMATCH),
        ("conversion_performed", |r| *at(r, "/report/calculation_report/model_input_summary/unit_display_summary/conversion_performed") = json!(true), BINDING_MISMATCH),
        ("storage_convention", |r| *at(r, "/report/calculation_report/model_input_summary/unit_display_summary/storage_convention") = json!("converted_to_si"), BINDING_MISMATCH),
        ("quantity_display_policy", |r| *at(r, "/report/calculation_report/model_input_summary/unit_display_summary/quantity_display_policy") = json!("display converted values"), BINDING_MISMATCH),
        ("result_units", |r| *at(r, "/report/calculation_report/model_input_summary/unit_display_summary/result_units") = json!([]), BINDING_MISMATCH),
        // Status.
        ("solve rule status", |r| *at(r, "/solve_rule_check_status") = json!("USER_RULE_CHECKED"), BINDING_MISMATCH),
        ("report status", |r| {
            at(r, "/report/calculation_report/analysis_status").as_array_mut().unwrap().retain(|status| status != "RULE_INPUTS_INCOMPLETE");
        }, BINDING_MISMATCH),
        ("handoff status", |r| {
            at(r, "/state_comparison_handoff_records/0/sections/state_run_sections/1/analysis_status").as_array_mut().unwrap().pop();
        }, BINDING_MISMATCH),
        // Profile and context.
        ("profile", |r| {
            let other = if r["export_profile_id"] == DESKTOP_PROFILE { "runner_local_private_report_package_1" } else { DESKTOP_PROFILE };
            r["export_profile_id"] = json!(other);
        }, BINDING_MISMATCH),
        ("public report context", |r| *at(r, "/report/report_context/export_context") = json!("public_report"), REPORT_CONTEXT_NOT_LOCAL_PRIVATE),
    ]
}

#[test]
fn each_cross_member_substitution_is_refused() {
    for (profile, request) in [("runner", runner()), ("desktop", desktop())] {
        for (case, edit, code) in binding_cases() {
            refused(&with(&request, edit), code, &format!("{profile}: {case}"));
        }
    }
}

#[test]
fn document_side_substitutions_are_refused() {
    type Edit = fn(&mut Value);
    let cases: [(&str, Edit); 4] = [
        ("document solver_build_ref", |d| *at(d, "/result_envelope/solver_version/solver_build_ref") = json!("invented-build")),
        ("document unit system", |d| *at(d, "/result_envelope/unit_system_ref/ref_id") = json!("invented-other:units")),
        ("document audit reference", |d| *at(d, "/result_envelope/reproducibility/audit_manifest_ref/ref_id") = json!("audit-manifest:invented-other")),
        ("another run's record hash", |d| *at(d, "/result_envelope/reproducibility/run_hashes/0/value") = json!(OTHER_HEX)),
    ];
    for (profile, request) in [("runner", runner()), ("desktop", desktop())] {
        for (case, edit) in cases {
            let error = assemble(&with_document(&request, edit)).expect_err(case);
            assert!(
                [BINDING_MISMATCH, SOLVER_IDENTITY_MISMATCH, SOURCE_INVALID].iter().any(|code| error.starts_with(code)),
                "{profile}: {case}: {error}"
            );
        }
    }
}

#[test]
fn a_coherent_desktop_run_relabelling_is_refused() {
    let relabelled = desktop_with(Some("run:invented-relabelled"));
    assert_ne!(
        relabelled["result_documents"][0]["result_envelope"]["run_ref"]["ref_id"],
        relabelled["source_results"][0]["run_id"]
    );
    let error = assemble(&relabelled).unwrap_err();
    assert!(error.starts_with(BINDING_MISMATCH) || error.starts_with(SOURCE_INVALID), "{error}");
}

#[test]
fn a_runner_run_other_than_the_request_run_is_refused() {
    let edited = with_document(&runner(), |document| {
        *at(document, "/result_envelope/reproducibility/source_origin_bindings/0/request_run_ref/ref_id") =
            json!("run:invented-other");
    });
    let error = assemble(&edited).unwrap_err();
    // The request hash's subject derives from the request run, so the subject
    // check may refuse it first.
    assert!(
        [BINDING_MISMATCH, SOURCE_INVALID, CHECKSUM_SUBJECT_MISMATCH].iter().any(|code| error.starts_with(code)),
        "{error}"
    );
}

#[test]
fn a_desktop_origin_naming_another_input_manifest_is_refused() {
    let edited = with_document(&desktop(), |document| {
        *at(document, "/result_envelope/reproducibility/source_origin_bindings/0/qualification_ref/ref_id") =
            json!("input-manifest:invented-other");
    });
    let error = assemble(&edited).unwrap_err();
    assert!(error.starts_with(BINDING_MISMATCH) || error.starts_with(SOURCE_INVALID), "{error}");
}

/// Codex W-18a (b2) round 2 (assembled before this binding): a desktop
/// request that keeps its audit `manifest_id` and the document's matching
/// `audit_manifest_ref`, while the audited input-manifest hash and its payload
/// reference, the report's and the handoff records' references and the
/// origin's qualification name another input manifest, with the derivative
/// hash recomputed, is refused. The runner names its audit manifest by its
/// run, so the rule is the desktop's (§2.4).
#[test]
fn a_desktop_audit_manifest_id_other_than_its_input_manifest_reference_is_refused() {
    let old = desktop();
    let mut manifest: Value = serde_json::from_str(INPUT_MANIFEST).unwrap();
    manifest["fixture_label"] = json!("Invented other input manifest; test fixture only");
    let mut moved = desktop_basis(None, |_| {}, &manifest, "input-manifest:invented-other");
    let audit_ref = "/result_envelope/reproducibility/audit_manifest_ref";
    moved["audit_manifest"]["manifest_id"] = old["audit_manifest"]["manifest_id"].clone();
    *at(&mut moved["result_documents"][0], audit_ref) = old["result_documents"][0].pointer(audit_ref).unwrap().clone();
    moved["result_documents"][0] = common::rehash(moved["result_documents"][0].take());
    assert_eq!(moved["audit_manifest"]["input_manifest_hash"]["payload_ref"], "input-manifest:invented-other");
    let error = refused(&moved, BINDING_MISMATCH, "desktop: old audit id, moved input manifest");
    assert!(error.contains("desktop audit manifest id"), "{error}");
    // The runner's audit id is its run's, not its input manifest's.
    let runner = runner();
    assert_ne!(runner["audit_manifest"]["manifest_id"], runner["audit_manifest"]["input_manifest_hash"]["payload_ref"]);
    unblocked(&runner);
}

// ---------------------------------------------------------------------------
// Displayed content (§2.3 item 6).
// ---------------------------------------------------------------------------

#[test]
fn displayed_rows_other_than_the_source_projection_are_refused() {
    type Edit = fn(&mut Vec<Value>);
    let cases: [(&str, Edit); 4] = [
        ("replaced value", |rows| rows[0]["quantity_display"] = json!("1 m")),
        ("reordered", |rows| rows.swap(0, 1)),
        ("dropped", |rows| {
            rows.pop();
        }),
        ("added", |rows| {
            let row = rows[0].clone();
            rows.push(row);
        }),
    ];
    for request in [runner(), desktop()] {
        for (case, edit) in cases {
            let edited = with(&request, |r| edit(at(r, "/report/result_rows").as_array_mut().unwrap()));
            refused(&edited, REPORT_ROWS_MISMATCH, case);
        }
    }
}

#[test]
fn a_comparison_section_with_its_own_runs_is_accepted() {
    for request in [runner(), desktop()] {
        let edited = with(&request, |r| {
            at(r, "/state_comparison_handoff_records/0/sections/comparison_sections").as_array_mut().unwrap().push(json!({
                "section_id": "comparison:invented-left-right",
                "section_kind": "analysis_run_comparison",
                "comparison_ref": {"object_type": "AnalysisRunComparison", "ref": "invented-left-right"},
                "run_context": {
                    "left": {"run_ref": {"object_type": "AnalysisRun", "ref": "run:invented-left"}},
                    "right": {"run_ref": {"object_type": "AnalysisRun", "ref": "run:invented-right"}}
                },
                "manual_mappings": [],
                "unmatched_classifications": [],
                "tolerance_profile_refs": [],
                "unit_normalized_deltas": [],
                "settings_deltas": [],
                "diagnostics": [],
                "privacy_classification": "invented_public_example",
                "review_state": "pending"
            }));
        });
        unblocked(&edited);
    }
}

// ---------------------------------------------------------------------------
// Rule check (§2.3 item 7) and the W-19 headline.
// ---------------------------------------------------------------------------

/// A rule-check package: one failed and one incomplete check under the
/// document's own `RULE_INPUTS_INCOMPLETE` token, with the given checksum.
fn with_rule_check(request: &Value, checksum: Value) -> Value {
    let algorithm = checksum["algorithm"].clone();
    let report_basis = checksum["canonicalization"].as_str().unwrap_or("").to_string();
    let (audit_basis, audit_ref) = match report_basis.as_str() {
        "JCS" => ("rule_pack_payload_jcs", "invented-rule-pack".to_string()),
        _ => ("rfc8785_jcs", "rule_pack_document:invented-rule-pack".to_string()),
    };
    with(request, |r| {
        r["rule_check_aggregate"] = json!("RULE_INPUTS_INCOMPLETE");
        r["report"]["user_rule_checks"] = json!({
            "rule_pack_id": "invented-rule-pack",
            "grammar_version": "1.0.0",
            "aggregate_status": "RULE_INPUTS_INCOMPLETE",
            "checks": [
                {"check_id": "invented-check-a", "status": "USER_RULE_FAILED", "missing_inputs": []},
                {"check_id": "invented-check-b", "status": "RULE_INPUTS_INCOMPLETE", "missing_inputs": ["invented_input_x"]}
            ]
        });
        r["report"]["calculation_report"]["rule_pack_refs"] = json!([{
            "rule_pack_id": "invented-rule-pack",
            "version": "0.1.0",
            "checksum": checksum,
            "source_notice": "Invented non-code example only.",
            "redistribution_status": "invented_non_engineering_example",
            "completeness_status": "not_checked",
            "private_payload_redacted": true
        }]);
        r["audit_manifest"]["rule_pack_refs"] = json!([{
            "rule_pack_id": "invented-rule-pack",
            "rule_pack_version": "0.1.0",
            "source_notice": "Invented non-code example only.",
            "redistribution_status": "invented_non_engineering_example",
            "checksum": {
                "algorithm": algorithm,
                "canonicalization": audit_basis,
                "payload_kind": "rule_pack_reference",
                "payload_ref": audit_ref,
                "value": RULE_PACK_HEX
            },
            "private_payload_redacted": true
        }]);
    })
}

fn rule_pack_checksum(canonicalization: &str, payload_type: &str) -> Value {
    json!({
        "algorithm": "sha256",
        "canonicalization": canonicalization,
        "payload_ref": {"ref_type": payload_type, "ref_id": "invented-rule-pack"},
        "value": RULE_PACK_HEX
    })
}

/// The PDF's shown strings (`(...) Tj`), unescaped and joined by spaces, so
/// wrapped lines read as one text.
fn pdf_text(pdf: &[u8]) -> Vec<u8> {
    let mut text = Vec::new();
    let mut index = 0;
    while index < pdf.len() {
        if pdf[index] == b'(' {
            let mut shown = Vec::new();
            index += 1;
            while index < pdf.len() && pdf[index] != b')' {
                if pdf[index] == b'\\' && index + 1 < pdf.len() {
                    index += 1;
                }
                shown.push(pdf[index]);
                index += 1;
            }
            if pdf[index..].starts_with(b") Tj") {
                if !text.is_empty() {
                    text.push(b' ');
                }
                text.extend(shown);
            }
        }
        index += 1;
    }
    text
}

#[test]
fn supported_rule_pack_checksum_pairs_assemble_and_headline_incomplete() {
    let headline = "INCOMPLETE \u{2014} 1 check(s) failed; 1 check(s) blocked";
    for request in [runner(), desktop()] {
        for (basis, payload) in [("JCS", "rule_pack"), ("rfc8785_jcs", "rule_pack_document")] {
            let outcome = unblocked(&with_rule_check(&request, rule_pack_checksum(basis, payload)));
            let html = String::from_utf8(member(&outcome, ROLE_REPORT_HTML).to_vec()).unwrap();
            assert!(html.contains(headline), "{basis}: the HTML lacks the INCOMPLETE headline");
            // The PDF text is WinAnsi (the em dash is byte 0x97) and wraps.
            let text = pdf_text(member(&outcome, ROLE_REPORT_PDF));
            let needle = b"INCOMPLETE \x97 1 check(s) failed; 1 check(s) blocked";
            assert!(text.windows(needle.len()).any(|window| window == needle), "{basis}: the PDF lacks the INCOMPLETE headline");
        }
    }
}

#[test]
fn rule_check_mismatches_are_refused() {
    for request in [runner(), desktop()] {
        let checked = with_rule_check(&request, rule_pack_checksum("JCS", "rule_pack"));
        let cases: [(&str, Value, &str); 4] = [
            ("aggregate without run", with(&checked, |r| r["report"]["user_rule_checks"] = Value::Null), RULE_CHECK_BINDING_MISMATCH),
            ("run without aggregate", with(&checked, |r| r["rule_check_aggregate"] = Value::Null), RULE_CHECK_BINDING_MISMATCH),
            ("pack id mismatch", with(&checked, |r| r["report"]["user_rule_checks"]["rule_pack_id"] = json!("invented-other-pack")), RULE_CHECK_BINDING_MISMATCH),
            ("aggregate differs from the document token", with(&checked, |r| {
                r["rule_check_aggregate"] = json!("USER_RULE_FAILED");
                r["report"]["user_rule_checks"]["aggregate_status"] = json!("USER_RULE_FAILED");
            }), BINDING_MISMATCH),
        ];
        for (case, edited, code) in cases {
            refused(&edited, code, case);
        }
        for (basis, payload) in [
            ("jcs_compatible", "rule_pack"),
            ("JCS", "rule_pack_document"),
            ("rfc8785_jcs", "rule_pack"),
            ("project_local_deterministic_json", "rule_pack"),
            ("none", "rule_pack"),
        ] {
            refused(
                &with_rule_check(&request, rule_pack_checksum(basis, payload)),
                RULE_PACK_CHECKSUM_BASIS_UNSUPPORTED,
                &format!("{basis}/{payload}"),
            );
        }
        let mut other_algorithm = rule_pack_checksum("JCS", "rule_pack");
        other_algorithm["algorithm"] = json!("sha512");
        let edited = with(&with_rule_check(&request, rule_pack_checksum("JCS", "rule_pack")), |r| {
            r["report"]["calculation_report"]["rule_pack_refs"][0]["checksum"] = other_algorithm;
        });
        refused(&edited, RULE_PACK_CHECKSUM_BASIS_UNSUPPORTED, "sha512");
        let unprojected = with(&checked, |r| r["audit_manifest"]["rule_pack_refs"][0]["checksum"]["value"] = json!(OTHER_HEX));
        refused(&unprojected, RULE_CHECK_BINDING_MISMATCH, "audit checksum value");
    }
}

#[test]
fn checksum_payload_id_must_name_the_participating_rule_pack() {
    for request in [runner(), desktop()] {
        for (basis, payload_type) in [("JCS", "rule_pack"), ("rfc8785_jcs", "rule_pack_document")] {
            let mut checksum = rule_pack_checksum(basis, payload_type);
            checksum["payload_ref"]["ref_id"] = json!("invented-other-rule-pack");
            refused(&with_rule_check(&request, checksum), RULE_CHECK_BINDING_MISMATCH,
                "a different rule pack's checksum must not be relabelled by projection");
        }
    }
}

#[test]
fn arbitrary_annotation_metadata_is_not_a_typed_provenance_record() {
    let request = with_document(&runner(), |d| {
        d["result_envelope"]["source_annotations"][0]["metadata"] =
            json!({"provenance": {"optional_note": ""}});
    });
    open_pipe_stress_report_package::document_form::check_result_member_form(
        &request["result_documents"][0]).expect("schema-unconstrained raw annotation metadata must be preserved");
}

/// B2 on a real derived document: a source row's raw metadata carrying a
/// member named `provenance` derives a source-valid desktop document, which
/// assembles unblocked and reads back.
#[test]
fn raw_metadata_named_provenance_in_a_derived_desktop_document_is_admitted() {
    let request = desktop_from(None, |source| {
        let metadata = source["results"][0]["metadata"].as_object_mut().expect("the first row has metadata");
        metadata.insert("provenance".into(), json!({"optional_note": ""}));
    });
    let annotated = request["result_documents"][0]["result_envelope"]["source_annotations"]
        .as_array()
        .unwrap()
        .iter()
        .any(|annotation| annotation["metadata"]["provenance"] == json!({"optional_note": ""}));
    assert!(annotated, "the derived document carries the raw metadata");
    let outcome = unblocked(&request);
    read_report_package_container(&outcome.container_bytes).expect("the reader admits it too");
}

#[test]
fn route_preflight_precedes_audit_conversion() {
    for id in [Some("invented-unadmitted-contract"), None] {
        let mut request = with_document(&runner(), |d| {
            let producer = d["result_envelope"]["producer"].as_object_mut().unwrap();
            if let Some(id) = id { producer.insert("semantic_contract_id".into(), json!(id)); }
            else { producer.remove("semantic_contract_id"); }
        });
        request["audit_manifest"]["model_hash"]["canonicalization"] = json!("unsupported-basis");
        refused(&request, ROUTE_NOT_ADMITTED, "item 0 must run before audit conversion");
    }
}

#[test]
fn round2_audit_asset_checksum_must_name_its_asset() {
    let mut request = runner();
    request["audit_manifest"]["assets"] = json!([{
        "asset_id":"invented-asset-A", "payload_kind":"binary_asset", "privacy_class":"public_metadata",
        "hash":{"algorithm":"sha256", "canonicalization":"none", "payload_kind":"binary_asset",
            "payload_ref":"invented-asset-A", "value":RULE_PACK_HEX}, "provenance":"Invented public fixture"
    }]);
    let control=unblocked(&request);
    read_report_package_container(&control.container_bytes).unwrap();
    request["audit_manifest"]["assets"][0]["hash"]["payload_ref"]=json!("invented-asset-B");
    let result=assemble(&request);
    assert!(result.as_ref().map_or(true, |r|r.export_blocked), "another asset's hash assembled unblocked");
}

#[test]
fn round2_report_section_checksum_must_name_its_reference() {
    let mut request=runner();
    unblocked(&request);
    request["report"]["calculation_report"]["report_section_refs"][0]["checksum"]["payload_ref"]["ref_id"] = json!("invented-other-section");
    let result=assemble(&request);
    assert!(result.as_ref().map_or(true, |r|r.export_blocked), "another section's checksum assembled unblocked");
}

#[test]
fn round2_raw_source_hash_must_equal_the_bound_original_producer_checksum() {
    let request=with_document(&runner(), |d| {
        d["result_envelope"]["reproducibility"]["raw_source_hashes"][0]["payload_ref"]["ref_id"] = json!("invented-other-producer");
    });
    let result=assemble(&request);
    assert!(result.as_ref().map_or(true, |r|r.export_blocked), "unbound raw-source hash assembled unblocked");
}

#[test]
fn round2_marking_collects_all_successful_any_of_branches() {
    use open_pipe_stress_report_package::schema_eval::Catalog;
    let c=Catalog::new(vec![("inline.json".into(),json!({
        "$defs":{"Provenance":{"type":"object","required":["source_name"]}},
        "properties":{"record":{"anyOf":[true,{"$ref":"#/$defs/Provenance"}]}}
    }))]);
    let marks=c.validate_marking("inline.json",&json!({"record":{"source_name":" "}}),"inline.json","/$defs/Provenance").unwrap();
    assert_eq!(marks,["/record"]);
}

#[test]
fn round2_marking_collects_all_matching_contains_items() {
    use open_pipe_stress_report_package::schema_eval::Catalog;
    let c=Catalog::new(vec![("inline.json".into(),json!({
        "$defs":{"Provenance":{"type":"object","required":["source_name"]}},
        "contains":{"$ref":"#/$defs/Provenance"}
    }))]);
    let marks=c.validate_marking("inline.json",&json!([{"source_name":"ok"},{"source_name":" "}]),"inline.json","/$defs/Provenance").unwrap();
    assert_eq!(marks,["/0","/1"]);
}

#[test]
fn round2_numeric_bounds_preserve_large_integer_order() {
    use open_pipe_stress_report_package::schema_eval::Catalog;
    let c=Catalog::new(vec![("inline.json".into(),json!({"exclusiveMinimum":9007199254740992u64}))]);
    assert!(c.validate("inline.json", &json!(9007199254740993u64)).is_ok());
}

#[test]
fn round2_carrier_checksum_subjects_cannot_be_coherently_relabelled() {
    fn edit(value:&mut Value) {
        match value {
            Value::Object(map)=> {
                if map.get("payload_ref").is_some_and(|r|r["ref_type"]=="attested_headless_producer") {
                    map.get_mut("payload_ref").unwrap()["ref_id"]=json!("invented-other-carrier");
                }
                for item in map.values_mut() {edit(item);}
            },
            Value::Array(items)=>for item in items {edit(item);},
            _=>{},
        }
    }
    let request=with_document(&runner(),edit);
    let result=assemble(&request);
    assert!(result.as_ref().map_or(true, |r|r.export_blocked), "359 carrier checksum subjects relabelled without refusal");
}

// ---------------------------------------------------------------------------
// Checksum subjects (DEL-08-01 Design §2.4, "Checksum subjects"): one refusal
// per record kind, generated from the request's own payload references.
// ---------------------------------------------------------------------------

/// Every `payload_ref` location in `value`, as JSON pointers.
fn payload_ref_locations(value: &Value, pointer: &str, out: &mut Vec<String>) {
    match value {
        Value::Object(members) => {
            for (key, member) in members {
                let at = format!("{pointer}/{}", key.replace('~', "~0").replace('/', "~1"));
                if key == "payload_ref" {
                    out.push(at.clone());
                }
                payload_ref_locations(member, &at, out);
            }
        }
        Value::Array(items) => {
            for (index, item) in items.iter().enumerate() {
                payload_ref_locations(item, &format!("{pointer}/{index}"), out);
            }
        }
        _ => {}
    }
}

fn record_kind(pointer: &str) -> String {
    pointer
        .split('/')
        .map(|segment| if !segment.is_empty() && segment.bytes().all(|byte| byte.is_ascii_digit()) { "*" } else { segment })
        .collect::<Vec<_>>()
        .join("/")
}

/// A request carrying every record kind: the basis plus an audit asset and a
/// rule-check run (the report's and the audit's rule-pack records).
fn every_kind(request: &Value) -> Value {
    let mut request = with_rule_check(request, rule_pack_checksum("JCS", "rule_pack"));
    request["audit_manifest"]["assets"] = json!([{
        "asset_id": "invented-asset-A", "payload_kind": "binary_asset", "privacy_class": "public_metadata",
        "hash": {"algorithm": "sha256", "canonicalization": "none", "payload_kind": "binary_asset",
            "payload_ref": "invented-asset-A", "value": RULE_PACK_HEX},
        "provenance": "Invented public fixture"
    }]);
    // The result member's own rule-pack reference, under the document basis.
    request = with_document(&request, |document| {
        document["result_envelope"]["rule_pack_refs"] = json!([{
            "rule_pack_id": "invented-rule-pack", "version": "0.1.0",
            "checksum": rule_pack_checksum("rfc8785_jcs", "rule_pack_document"),
            "source_notice": "Invented non-code example only.",
            "redistribution_status": "invented_non_engineering_example",
            "completeness_status": "not_checked", "private_payload_redacted": true
        }]);
    });
    request
}

/// The record kinds the round-2 review inventoried (640 references, 23
/// shapes), plus the asset and rule-pack records a rule-check package adds.
const RECORD_KINDS: [&str; 27] = [
    "/audit_manifest/assets/*/hash/payload_ref",
    "/audit_manifest/input_manifest_hash/payload_ref",
    "/audit_manifest/model_hash/payload_ref",
    "/audit_manifest/rule_pack_refs/*/checksum/payload_ref",
    "/report/calculation_report/audit_manifest_refs/*/checksum/payload_ref",
    "/report/calculation_report/model_input_summary/model_hash/payload_ref",
    "/report/calculation_report/report_section_refs/*/checksum/payload_ref",
    "/report/calculation_report/result_export_refs/*/checksum/payload_ref",
    "/report/calculation_report/rule_pack_refs/*/checksum/payload_ref",
    "/result_documents/*/result_envelope/reproducibility/derivative_hash/payload_ref",
    "/result_documents/*/result_envelope/reproducibility/model_hash/payload_ref",
    "/result_documents/*/result_envelope/reproducibility/raw_source_hashes/*/payload_ref",
    "/result_documents/*/result_envelope/reproducibility/request_hash/payload_ref",
    "/result_documents/*/result_envelope/reproducibility/run_hashes/*/payload_ref",
    "/result_documents/*/result_envelope/reproducibility/source_origin_bindings/*/original_producer_checksum/payload_ref",
    "/result_documents/*/result_envelope/reproducibility/source_origin_bindings/*/received_carrier_checksum/payload_ref",
    "/result_documents/*/result_envelope/rule_pack_refs/*/checksum/payload_ref",
    "/result_documents/*/result_envelope/row_accounting/*/original_producer_row_checksum/payload_ref",
    "/result_documents/*/result_envelope/row_accounting/*/received_carrier_row_checksum/payload_ref",
    "/result_documents/*/result_envelope/row_disclosures/*/original_producer_row_checksum/payload_ref",
    "/result_documents/*/result_envelope/row_disclosures/*/received_carrier_row_checksum/payload_ref",
    "/result_documents/*/result_envelope/unit_preservation_witnesses/*/original_producer_row_checksum/payload_ref",
    "/result_documents/*/result_envelope/unit_preservation_witnesses/*/received_carrier_row_checksum/payload_ref",
    "/result_documents/*/result_envelope/unit_preservation_witnesses/*/target_row_checksum/payload_ref",
    "/state_comparison_handoff_records/*/sections/state_run_sections/*/hash_refs/*/payload_ref",
    "/state_comparison_handoff_records/*/sections/state_run_sections/*/reproducibility/input_manifest_hashes/*/payload_ref",
    "/state_comparison_handoff_records/*/sections/state_run_sections/*/result_refs/*/hash_refs/*/payload_ref",
];

#[test]
fn every_checksum_record_kind_refuses_another_subject() {
    for (profile, request) in [("runner", runner()), ("desktop", desktop())] {
        let request = every_kind(&request);
        unblocked(&request);
        let mut locations = Vec::new();
        payload_ref_locations(&request, "", &mut locations);
        let mut kinds = std::collections::BTreeMap::new();
        for location in locations {
            kinds.entry(record_kind(&location)).or_insert(location);
        }
        for kind in RECORD_KINDS {
            // The desktop document has no request hash or raw-source hash,
            // and no producer-row checksums (its producer is not available).
            let desktop_absent = kind.contains("request_hash")
                || kind.contains("raw_source_hashes")
                || kind.contains("original_producer");
            if profile == "runner" || !desktop_absent {
                assert!(kinds.contains_key(kind), "{profile}: no {kind} record to probe");
            }
        }
        for (kind, location) in &kinds {
            // The type alone, the id alone, and both. An audit reference is a
            // string whose type is the record's `payload_kind`.
            for change in ["type", "id", "both"] {
                let mut edited = request.clone();
                let record_pointer = location.strip_suffix("/payload_ref").unwrap().to_string();
                let subject = edited.pointer_mut(location).unwrap();
                let is_string = subject.is_string();
                let (type_key, id_key) = if subject.get("object_type").is_some() { ("object_type", "ref") } else { ("ref_type", "ref_id") };
                if change != "type" {
                    match subject {
                        Value::String(_) => *subject = json!("invented-other-subject"),
                        _ => subject[id_key] = json!("invented-other-subject"),
                    }
                }
                if change != "id" {
                    if is_string {
                        let record = edited.pointer_mut(&record_pointer).unwrap();
                        let other = if record["payload_kind"] == "external_artifact" { "binary_asset" } else { "external_artifact" };
                        record["payload_kind"] = json!(other);
                    } else {
                        let subject = edited.pointer_mut(location).unwrap();
                        let other = if subject[type_key] == "model_payload" { "rule_pack" } else { "model_payload" };
                        subject[type_key] = json!(other);
                    }
                }
                // Re-hash an edited result document, except its own
                // derivative hash's subject, which the re-hash would restore.
                if location.starts_with("/result_documents/0/") && !kind.ends_with("/derivative_hash/payload_ref") {
                    edited["result_documents"][0] = common::rehash(edited["result_documents"][0].take());
                }
                let outcome = assemble(&edited);
                assert!(
                    outcome.as_ref().map_or(true, |outcome| outcome.export_blocked),
                    "{profile}: a {change} change at {location} assembled unblocked"
                );
            }
        }
    }
}

#[test]
fn a_document_rule_pack_checksum_names_its_pack() {
    use open_pipe_stress_report_package::document_form::check_document_subjects;
    let mut document = runner()["result_documents"][0].clone();
    check_document_subjects(&document).unwrap();
    document["result_envelope"]["rule_pack_refs"] = json!([{
        "rule_pack_id": "invented-rule-pack", "version": "0.1.0",
        "checksum": rule_pack_checksum("JCS", "rule_pack"),
        "source_notice": "Invented non-code example only.",
        "redistribution_status": "invented_non_engineering_example",
        "completeness_status": "not_checked", "private_payload_redacted": true
    }]);
    let document = common::rehash(document);
    check_document_subjects(&document).unwrap();
    let mut other = document.clone();
    other["result_envelope"]["rule_pack_refs"][0]["checksum"]["payload_ref"]["ref_id"] = json!("invented-other-pack");
    let error = check_document_subjects(&common::rehash(other)).unwrap_err();
    assert!(error.starts_with(CHECKSUM_SUBJECT_MISMATCH), "{error}");
}

#[test]
fn round3_document_rule_pack_checksum_must_name_the_pack_type() {
    for basis in ["JCS", "rfc8785_jcs"] {
        let edited = with_document(&runner(), |d| {
            let mut checksum = rule_pack_checksum(basis, "model_payload");
            checksum["payload_ref"]["ref_id"] = json!("invented-rule-pack");
            d["result_envelope"]["rule_pack_refs"] = json!([{
                "rule_pack_id":"invented-rule-pack", "version":"0.1.0", "checksum":checksum,
                "source_notice":"Invented non-code example only.",
                "redistribution_status":"invented_non_engineering_example", "completeness_status":"not_checked", "private_payload_redacted":true
            }]);
        });
        let result = assemble(&edited);
        assert!(result.as_ref().map_or(true, |r|r.export_blocked),
            "{basis}: a model_payload checksum was exported as a rule-pack checksum");
    }
}

#[test]
fn round3_rendered_rule_pack_checksum_must_name_the_pack_type() {
    let mut request = with_rule_check(&runner(), rule_pack_checksum("JCS", "rule_pack"));
    request["report"]["calculation_report"]["rule_pack_refs"][0]["checksum"]["payload_ref"]["ref_type"] = json!("model_payload");
    let report: open_pipe_stress_report_generator::CalculationReport =
        serde_json::from_value(request["report"]["calculation_report"].clone()).unwrap();
    assert!(open_pipe_stress_report_generator::validate_report(&report).is_blocked(),
        "a model_payload checksum was rendered as a rule-pack checksum");
}
