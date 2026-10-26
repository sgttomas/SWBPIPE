//! The exact-route package witness (W-11 slice (b), DEL-08-01 Design
//! `exact-route-report-sections.md` §5.5 and §7): a `#[cfg(test)]` module of
//! this crate, because the widened set `EXACT_WITNESS_ADMITTED` exists only in
//! the crate's own test build.
//!
//! - Each witness request (written by the desktop's vitest below the output
//!   gate, `fixtures/reports/invented/exact_report_package_requests/`)
//!   assembles through `assemble_wire_request_in` and reads back through
//!   `read_report_package_container_in`, both under the witness set: the set
//!   reaches every item-0 check on both paths through the forwarding graph.
//! - Every production entry point refuses each witness request and the witness
//!   package bytes with `REPORT-PACKAGE-ROUTE-NOT-ADMITTED`.
//! - The runner profile is refused in the producer and the reader
//!   (`REPORT-PACKAGE-EXACT-RUNNER-PROFILE`), with and without `exact_route`.
//! - The same-run substitute passes items 0-9: the shared checks alone cannot
//!   see a coherent substitute; the desktop's native source binding refuses it
//!   (`report_package_save.rs`, `report_render_exact_tests.rs`).
//!
//! Before the owner opens exact-route packages, no production path assembles
//! or reads an exact member. Standard claim fence applies (claims taxonomy per
//! DEC-081).

use crate::admitted::EXACT_WITNESS_ADMITTED;
use crate::bindings::{check_request, RUNNER_PROFILE};
use crate::exact_sections::EXACT_RUNNER_PROFILE;
use crate::reader::{read_report_package_container, read_report_package_container_in};
use crate::wire::{assemble_wire_request, assemble_wire_request_in, audit_manifest, LinkedSolverIdentity, ReportPackageRequest};
use crate::{
    assemble_report_package_container, build_deterministic_zip, sha256_hex, PackageRef, ReportPackageContainerInput,
    ReportPackageContainerOutcome, ROLE_PACKAGE_MANIFEST, ROLE_RESULT_EXPORT,
};
use open_pipe_stress_canonical_json::canonical_json;
use open_pipe_stress_report_renderer::RenderableReportInput;
use serde_json::{json, Value};

const ROUTE_NOT_ADMITTED: &str = "REPORT-PACKAGE-ROUTE-NOT-ADMITTED";

const WITNESSES: [(&str, &str); 3] = [
    ("pressure_2_straight", include_str!("../../../../fixtures/reports/invented/exact_report_package_requests/pressure_2_straight.json")),
    (
        "pressure_2_load_reference_states",
        include_str!("../../../../fixtures/reports/invented/exact_report_package_requests/pressure_2_load_reference_states.json"),
    ),
    ("pressure_source_1", include_str!("../../../../fixtures/reports/invented/exact_report_package_requests/pressure_source_1.json")),
];
const SUBSTITUTE: &str =
    include_str!("../../../../fixtures/reports/invented/exact_report_package_requests/pressure_2_straight_substitute.json");

fn request_of(text: &str) -> Value {
    let witness: Value = serde_json::from_str(text).unwrap();
    witness["request"].clone()
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
    let outcome = assemble_wire_request_in(typed(request), &identity(&solver), EXACT_WITNESS_ADMITTED)
        .unwrap_or_else(|error| panic!("the witness set assembles the request: {error}"));
    assert!(!outcome.export_blocked, "{:?}", outcome.blocking_reasons);
    outcome
}

#[test]
fn each_witness_shape_assembles_and_reads_back_under_the_witness_set() {
    for (name, text) in WITNESSES {
        let request = request_of(text);
        let identity_id = request["result_documents"][0]["result_envelope"]["producer"]["semantic_contract_id"].clone();
        let outcome = witness_package(&request);
        let read = read_report_package_container_in(&outcome.container_bytes, EXACT_WITNESS_ADMITTED)
            .unwrap_or_else(|refusal| panic!("{name}: the witness set reads the package back: {refusal:?}"));
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
        // The rendered members carry the exact block and the composed standing line.
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

#[test]
fn every_production_entry_point_refuses_each_witness() {
    for (name, text) in WITNESSES {
        let request = request_of(text);
        let solver = linked(&request);
        let refused = |outcome: Result<String, String>| {
            let error = outcome.expect_err("refused");
            assert!(error.starts_with(ROUTE_NOT_ADMITTED), "{name}: {error}");
        };
        refused(assemble_wire_request(typed(&request), &identity(&solver)).map(|_| String::new()));
        let typed_request = typed(&request);
        let audit = audit_manifest(typed_request.audit_manifest.clone(), &identity(&solver)).unwrap();
        refused(check_request(&typed_request, &audit, &identity(&solver)).map(|()| String::new()));
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
        let outcome = assemble_report_package_container(&input);
        assert!(outcome.export_blocked, "{name}");
        assert!(outcome.blocking_reasons.iter().any(|reason| reason.contains(ROUTE_NOT_ADMITTED)), "{name}: {:?}", outcome.blocking_reasons);
        // The witness package's bytes, read by the production reader.
        let bytes = witness_package(&request).container_bytes;
        let refusal = read_report_package_container(&bytes).expect_err("the production reader refuses");
        assert!(format!("{refusal:?}").contains(ROUTE_NOT_ADMITTED), "{name}: {refusal:?}");
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
            let error = assemble_wire_request_in(typed(&runner_profile(&request, keep)), &identity(&solver), EXACT_WITNESS_ADMITTED)
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
        let refusal = read_report_package_container_in(&bytes, EXACT_WITNESS_ADMITTED).expect_err("the reader refuses");
        assert!(format!("{refusal:?}").contains(EXACT_RUNNER_PROFILE), "{name}: {refusal:?}");
    }
}

/// §3.5: the shared checks alone cannot see a coherent same-run substitute.
/// It passes items 0-9 under the witness set; the native source binding of
/// the desktop's save is what refuses it.
#[test]
fn the_same_run_substitute_passes_items_0_to_9() {
    let genuine = request_of(WITNESSES[0].1);
    let substitute = request_of(SUBSTITUTE);
    let admission = |request: &Value| request["report"]["exact_route"]["admission"].clone();
    for field in ["native_job_id", "received_bytes_sha256", "run_ref", "semantic_contract_id"] {
        assert_eq!(admission(&substitute)[field], admission(&genuine)[field], "{field}");
    }
    assert_ne!(admission(&substitute)["received_result_sha256"], admission(&genuine)["received_result_sha256"]);
    assert_ne!(canonical_json(&substitute["source_results"][0]), canonical_json(&genuine["source_results"][0]));
    witness_package(&substitute);
}
