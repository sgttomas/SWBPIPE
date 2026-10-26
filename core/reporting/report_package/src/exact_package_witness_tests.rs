//! The exact-route package witness (W-11, DEL-08-01 Design
//! `exact-route-report-sections.md` §5.5 and §7). Since the owner opened
//! exact-route packages (§5.1; owner, 2026-10-26: "My two decisions are to
//! approve your recommendations in both cases. Proceed accordingly."), the
//! production set admits pressure-2 and pressure-source-1, so this module uses
//! the public production roots only; the test-only widened set is removed.
//!
//! - Each witness request (written by the desktop's vitest,
//!   `fixtures/reports/invented/exact_report_package_requests/`) assembles
//!   through `assemble_wire_request` and reads back through
//!   `read_report_package_container`: the production set reaches every item-0
//!   check on both paths through the forwarding graph.
//! - Every other production entry point admits each witness request:
//!   `bindings::check_request` and `assemble_report_package_container`.
//! - Every production entry point still refuses an identity outside the
//!   admitted set (physics-retained-1) with `REPORT-PACKAGE-ROUTE-NOT-ADMITTED`.
//! - The runner profile is refused in the producer and the reader
//!   (`REPORT-PACKAGE-EXACT-RUNNER-PROFILE`), with and without `exact_route`.
//! - The same-run substitute passes items 0-9: the shared checks alone cannot
//!   see a coherent substitute; the desktop's native source binding refuses it
//!   (`report_package_save.rs`, `report_render_exact_tests.rs`).
//!
//! Standard claim fence applies (claims taxonomy per DEC-081).

use crate::bindings::{check_request, RUNNER_PROFILE};
use crate::exact_sections::EXACT_RUNNER_PROFILE;
use crate::reader::read_report_package_container;
use crate::wire::{assemble_wire_request, audit_manifest, decode_report_package_request, LinkedSolverIdentity, ReportPackageRequest, REQUEST_MEMBERS};
use crate::{
    assemble_report_package_container, build_deterministic_zip, sha256_hex, PackageRef, ReportPackageContainerInput,
    ReportPackageContainerOutcome, ROLE_PACKAGE_MANIFEST, ROLE_RESULT_EXPORT,
};
use open_pipe_stress_canonical_json::canonical_json;
use open_pipe_stress_report_renderer::RenderableReportInput;
use open_pipe_stress_result_export::semantic_contract::PHYSICS_RETAINED_ID;
use serde_json::{json, Value};

const ROUTE_NOT_ADMITTED: &str = "REPORT-PACKAGE-ROUTE-NOT-ADMITTED";

/// The five witness shapes of §5.5.
const WITNESSES: [(&str, &str); 5] = [
    ("pressure_2_straight", include_str!("../../../../fixtures/reports/invented/exact_report_package_requests/pressure_2_straight.json")),
    (
        "pressure_2_load_reference_states",
        include_str!("../../../../fixtures/reports/invented/exact_report_package_requests/pressure_2_load_reference_states.json"),
    ),
    ("pressure_2_connector", include_str!("../../../../fixtures/reports/invented/exact_report_package_requests/pressure_2_connector.json")),
    (
        "pressure_2_realized_bend",
        include_str!("../../../../fixtures/reports/invented/exact_report_package_requests/pressure_2_realized_bend.json"),
    ),
    ("pressure_source_1", include_str!("../../../../fixtures/reports/invented/exact_report_package_requests/pressure_source_1.json")),
];
/// The same-run substitute: leaf replacements over the straight witness's request.
const SUBSTITUTE_OVERLAY: &str =
    include_str!("../../../../fixtures/reports/invented/exact_report_package_requests/pressure_2_straight_substitute.overlay.json");

fn request_of(text: &str) -> Value {
    let witness: Value = serde_json::from_str(text).unwrap();
    witness["request"].clone()
}

/// The substitute request: the overlay's replacements applied to the straight
/// witness's request.
fn substitute_request() -> Value {
    let overlay: Value = serde_json::from_str(SUBSTITUTE_OVERLAY).unwrap();
    assert_eq!(overlay["base"], "pressure_2_straight.json");
    let mut request = request_of(WITNESSES[0].1);
    for entry in overlay["replacements"].as_array().unwrap() {
        *request.pointer_mut(entry["pointer"].as_str().unwrap()).unwrap() = entry["value"].clone();
    }
    request
}

fn typed(request: &Value) -> ReportPackageRequest {
    serde_json::from_value(request.clone()).unwrap_or_else(|error| panic!("typed request: {error}"))
}

/// The linked product-physics identity the desktop bridge passes, read from the
/// request's audit solver (its build stamp is `<name>@<version>`).
fn linked(request: &Value) -> (String, String, String) {
    let solver = &request["audit_manifest"]["solver_version"];
    let text = |key: &str| solver[key].as_str().unwrap().to_string();
    (text("solver_name"), text("solver_version"), text("solver_build_ref"))
}

fn identity((name, version, build): &(String, String, String)) -> LinkedSolverIdentity<'_> {
    LinkedSolverIdentity { solver_name: name, solver_version: version, solver_build_ref: build }
}

fn witness_package(request: &Value) -> ReportPackageContainerOutcome {
    let solver = linked(request);
    let outcome = assemble_wire_request(typed(request), &identity(&solver))
        .unwrap_or_else(|error| panic!("the production set assembles the request: {error}"));
    assert!(!outcome.export_blocked, "{:?}", outcome.blocking_reasons);
    outcome
}

#[test]
fn each_witness_shape_assembles_and_reads_back_through_the_production_roots() {
    for (name, text) in WITNESSES {
        let request = request_of(text);
        let identity_id = request["result_documents"][0]["result_envelope"]["producer"]["semantic_contract_id"].clone();
        let outcome = witness_package(&request);
        let read = read_report_package_container(&outcome.container_bytes)
            .unwrap_or_else(|refusal| panic!("{name}: the production reader reads the package back: {refusal:?}"));
        assert!(!read.export_blocked, "{name}");
        assert_eq!(read.package_identity_sha256_hex, outcome.package_identity_sha256_hex, "{name}");
        assert_eq!(read.members.len(), outcome.members.len(), "{name}");
        for (found, written) in read.members.iter().zip(&outcome.members) {
            assert_eq!(
                (&found.role, &found.file_name, &found.sha256_hex, found.byte_length),
                (&written.role, &written.file_name, &written.sha256_hex, written.byte_length),
                "{name}"
            );
        }
        // The result member is the exact document Export Result JSON saves.
        let result = outcome.members.iter().find(|member| member.role == ROLE_RESULT_EXPORT).unwrap();
        let document: Value = serde_json::from_slice(&result.bytes).unwrap();
        assert_eq!(document["result_envelope"]["producer"]["semantic_contract_id"], identity_id, "{name}");
        assert_eq!(canonical_json(&document), canonical_json(&request["result_documents"][0]), "{name}");
        // Both rendered members are emitted; the HTML carries the exact block and
        // the composed standing line.
        let pdf = outcome.members.iter().find(|member| member.file_name.ends_with(".pdf")).expect("a PDF member");
        assert!(pdf.bytes.starts_with(b"%PDF-") && pdf.byte_length > 0, "{name}");
        let html = outcome.members.iter().find(|member| member.file_name.ends_with(".html")).unwrap();
        let html = String::from_utf8(html.bytes.clone()).unwrap();
        assert!(html.contains("Exact-Route Result") && html.contains("Numerical standing: integrity checked"), "{name}");
        assert!(html.contains("Stress Headlines by Section Basis") && html.contains("[nominal section]"), "{name}");
        if name.contains("load_reference") {
            assert!(html.contains("Their published limitations are listed under Limitations."), "{name}");
        }
        assert!(!html.contains("professional acceptance is produced"), "{name}: the omitted sentence");
    }
}

/// The container input of `request` for `assemble_report_package_container`.
fn container_outcome(request: &Value) -> ReportPackageContainerOutcome {
    let solver = linked(request);
    let typed_request = typed(request);
    let audit = audit_manifest(typed_request.audit_manifest.clone(), &identity(&solver)).unwrap();
    let report: RenderableReportInput = serde_json::from_value(request["report"].clone()).unwrap();
    let refs = |value: &Value| PackageRef { ref_type: value["ref_type"].as_str().unwrap().into(), ref_id: value["ref_id"].as_str().unwrap().into() };
    let input = ReportPackageContainerInput {
        package_id: request["package_id"].as_str().unwrap().into(),
        export_profile_id: request["export_profile_id"].as_str().unwrap().into(),
        source_model_ref: refs(&request["source_model_ref"]),
        source_basis_refs: request["source_basis_refs"].as_array().unwrap().iter().map(refs).collect(),
        report: &report,
        audit_manifest: &audit,
        result_documents: &typed_request.result_documents,
        state_comparison_handoff_records: &typed_request.state_comparison_handoff_records,
    };
    assemble_report_package_container(&input)
}

/// W-11 opening (§5.1 item 8): every production entry point admits each
/// witness request; the container assembly yields the same package identity
/// as the wire assembly.
#[test]
fn every_production_entry_point_admits_each_witness() {
    for (name, text) in WITNESSES {
        let request = request_of(text);
        let solver = linked(&request);
        let wire = witness_package(&request);
        let typed_request = typed(&request);
        let audit = audit_manifest(typed_request.audit_manifest.clone(), &identity(&solver)).unwrap();
        check_request(&typed_request, &audit, &identity(&solver)).unwrap_or_else(|error| panic!("{name}: {error}"));
        let outcome = container_outcome(&request);
        assert!(!outcome.export_blocked, "{name}: {:?}", outcome.blocking_reasons);
        assert_eq!(outcome.package_identity_sha256_hex, wire.package_identity_sha256_hex, "{name}");
        read_report_package_container(&wire.container_bytes).unwrap_or_else(|refusal| panic!("{name}: {refusal:?}"));
    }
}

/// An identity outside the admitted set (physics-retained-1) is still refused
/// by name at item 0 by every production producer entry point.
#[test]
fn every_production_entry_point_refuses_an_unadmitted_identity() {
    for (name, text) in WITNESSES {
        let mut request = request_of(text);
        request["result_documents"][0]["result_envelope"]["producer"]["semantic_contract_id"] = json!(PHYSICS_RETAINED_ID);
        let solver = linked(&request);
        let refused = |outcome: Result<String, String>| {
            let error = outcome.expect_err("refused");
            assert!(error.starts_with(ROUTE_NOT_ADMITTED), "{name}: {error}");
            assert!(error.ends_with(&format!("found {PHYSICS_RETAINED_ID}")), "{name}: {error}");
        };
        refused(assemble_wire_request(typed(&request), &identity(&solver)).map(|_| String::new()));
        let typed_request = typed(&request);
        let audit = audit_manifest(typed_request.audit_manifest.clone(), &identity(&solver)).unwrap();
        refused(check_request(&typed_request, &audit, &identity(&solver)).map(|()| String::new()));
        let outcome = container_outcome(&request);
        assert!(outcome.export_blocked, "{name}");
        assert!(outcome.blocking_reasons.iter().any(|reason| reason.contains(ROUTE_NOT_ADMITTED)), "{name}: {:?}", outcome.blocking_reasons);
    }
}

/// `request` under the runner profile, with or without its `exact_route`.
fn runner_profile(request: &Value, keep_exact_route: bool) -> Value {
    let mut request = request.clone();
    request["export_profile_id"] = json!(RUNNER_PROFILE);
    if !keep_exact_route {
        request["report"].as_object_mut().unwrap().remove("exact_route");
    }
    request
}

#[test]
fn the_runner_profile_is_refused_in_the_producer_and_the_reader() {
    for (name, text) in WITNESSES {
        let request = request_of(text);
        let solver = linked(&request);
        for keep in [false, true] {
            let error = assemble_wire_request(typed(&runner_profile(&request, keep)), &identity(&solver))
                .expect_err("the runner profile packages no exact identity");
            assert!(error.starts_with(EXACT_RUNNER_PROFILE), "{name} (exact_route kept: {keep}): {error}");
        }
        // The witness package with its manifest rewritten to the runner profile
        // and its package digests recomputed refuses on read with the same code.
        let mut members = witness_package(&request).members;
        let manifest = members.iter_mut().find(|member| member.role == ROLE_PACKAGE_MANIFEST).unwrap();
        let mut document: Value = serde_json::from_slice(&manifest.bytes).unwrap();
        assert_ne!(document["export_profile_id"], json!(RUNNER_PROFILE));
        document["export_profile_id"] = json!(RUNNER_PROFILE);
        manifest.bytes = canonical_json(&document).into_bytes();
        manifest.sha256_hex = sha256_hex(&manifest.bytes);
        manifest.byte_length = manifest.bytes.len();
        let bytes = build_deterministic_zip(&members);
        let refusal = read_report_package_container(&bytes).expect_err("the reader refuses");
        assert!(format!("{refusal:?}").contains(EXACT_RUNNER_PROFILE), "{name}: {refusal:?}");
    }
}

/// §3.5: the shared checks alone cannot see a coherent same-run substitute.
/// It passes items 0-9; the native source binding of the desktop's save is
/// what refuses it.
#[test]
fn the_same_run_substitute_passes_items_0_to_9() {
    let genuine = request_of(WITNESSES[0].1);
    let substitute = substitute_request();
    let admission = |request: &Value| request["report"]["exact_route"]["admission"].clone();
    for field in ["native_job_id", "received_bytes_sha256", "run_ref", "semantic_contract_id"] {
        assert_eq!(admission(&substitute)[field], admission(&genuine)[field], "{field}");
    }
    assert_ne!(admission(&substitute)["received_result_sha256"], admission(&genuine)["received_result_sha256"]);
    assert_ne!(canonical_json(&substitute["source_results"][0]), canonical_json(&genuine["source_results"][0]));
    witness_package(&substitute);
}

/// Codex r1 finding 2: a row the source does not derive, such as a fabricated
/// numerical-health line, refuses at item 6 through the production roots.
#[test]
fn a_fabricated_report_row_refuses_at_item_6() {
    for (name, text) in WITNESSES {
        let mut request = request_of(text);
        request["report"]["result_rows"].as_array_mut().unwrap().push(json!({
            "row_id": "numerical-health:case:invented",
            "label": "Maximum stress 1 Pa on every member (fabricated)",
            "case_ref": "load_case:case:invented",
            "quantity_display": "",
            "source_ref": "diagnostic:invented"
        }));
        let solver = linked(&request);
        let error = assemble_wire_request(typed(&request), &identity(&solver)).map(drop).expect_err("refused");
        assert!(error.starts_with("REPORT-PACKAGE-REPORT-ROWS-MISMATCH"), "{name}: {error}");
    }
}

/// The object's member values as a JSON array.
fn as_array(value: &Value) -> Value {
    Value::Array(value.as_object().expect("an object").values().cloned().collect())
}

/// Codex review round 2 of #1648: the shared producer reads the report's
/// context, exact section and the document's identity by key from untyped
/// members. A positional request decodes to the same members and assembles
/// the same package; a positional report, report context, exact section or
/// result document refuses, with no package (the report's structs by the
/// shared object-form check, round 3).
#[test]
fn positional_members_refuse_and_a_positional_request_assembles_the_same_package() {
    for (name, text) in WITNESSES {
        let request = request_of(text);
        let solver = linked(&request);
        let top = Value::Array(REQUEST_MEMBERS.iter().map(|member| request.get(*member).cloned().unwrap_or(Value::Null)).collect());
        let decoded = decode_report_package_request(top).expect("the positional request decodes");
        let outcome = assemble_wire_request(decoded, &identity(&solver)).unwrap_or_else(|error| panic!("{name}: {error}"));
        assert_eq!(outcome.package_identity_sha256_hex, witness_package(&request).package_identity_sha256_hex, "{name}");
        // Each refuses, never assembling a package: a positional report
        // struct at the producer's object-form check after item 0
        // (`REPORT-PACKAGE-REQUEST-POSITIONAL`), and a positional result
        // document at item 0's identity, which reads it by key.
        let edits: [(&str, fn(&mut Value)); 4] = [
            ("positional report", |r| r["report"] = as_array(&r["report"])),
            ("positional report_context", |r| r["report"]["report_context"] = as_array(&r["report"]["report_context"])),
            ("positional exact_route", |r| r["report"]["exact_route"] = as_array(&r["report"]["exact_route"])),
            ("positional result document", |r| r["result_documents"][0] = as_array(&r["result_documents"][0])),
        ];
        for (label, edit) in edits {
            let mut variant = request.clone();
            edit(&mut variant);
            let error = assemble_wire_request(typed(&variant), &identity(&solver)).map(drop).expect_err("refused");
            assert!(error.starts_with("REPORT-PACKAGE-"), "{name} {label}: {error}");
        }
    }
}
