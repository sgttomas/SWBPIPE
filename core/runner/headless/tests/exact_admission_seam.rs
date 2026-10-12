//! T4-U2a through the headless runner: a v3 (`3.0.0/exact_pressure_v3`)
//! document with a family the seam does not admit (a valve; a geometry-only
//! bend, D-2) is blocked by name with no export, in both solver modes; a
//! straight v3 document, and since T4-U2 phase 1 one whose region member is a
//! realized bend, solves under `pressure-1`; since T4-U2 phase 2 both produce
//! a pressure-1 export document that the exporter's own check admits.
//! The input is PP's committed invented X0 fixture with only the contract
//! identity changed.
use open_pipe_stress_headless_runner::{
    run_preview_model_value_with_mode, PrivacyContext, ProfessionalBoundary, Provenance,
    RedistributionStatus, Reference, RunnerOperation, RunnerRequest, TbdDecisions,
};
use open_pipe_stress_product_physics::PreviewSolverMode;
use serde_json::{json, Value};

const X0: &str =
    include_str!("../../../product_physics/tests/fixtures/exact_pressure_connected_request.json");
const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];
const PRESSURE_ID: &str = "openpipestress.result_semantics/0.3.0/pressure-1";

fn request(model: &Value) -> RunnerRequest {
    RunnerRequest {
        request_id: "t4-u2a-exact-admission-seam".into(),
        operation: RunnerOperation::Solve,
        operation_ref: Reference::new("api_operation", "ops.solve.job"),
        project_ref: Reference::new("project", model["project"]["id"].as_str().unwrap()),
        model_ref: Reference::new("model", model["project"]["id"].as_str().unwrap()),
        unit_system_ref: Reference::new("unit_system", "invented-si"),
        load_basis_refs: model["load_cases"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| Reference::new("load_case", c["id"].as_str().unwrap()))
            .collect(),
        input_manifest_ref: Reference::new("audit_manifest", "t4-u2a-input-manifest"),
        requested_outputs: vec![
            "result_envelope".into(),
            "audit_manifest".into(),
            "diagnostics".into(),
        ],
        privacy: PrivacyContext::local_first_public_metadata(),
        provenance: Provenance {
            source_name: "invented T4-U2a seam fixture".into(),
            source_location: "core/product_physics/tests/fixtures".into(),
            source_license: "project invented".into(),
            contributor: "OpenPipeStress".into(),
            contributor_certification: "invented non-engineering example".into(),
            redistribution_status: RedistributionStatus::InventedNonEngineeringExample,
            review_status: "pending".into(),
        },
        professional_boundary: ProfessionalBoundary::project_default(),
        tbd_decisions: TbdDecisions::d33_local_cli_policy(),
    }
}

fn v3() -> Value {
    let mut document: Value = serde_json::from_str(X0).unwrap();
    document["model"]["pressure_contract"] = json!({"version":"3.0.0","mode":"exact_pressure_v3"});
    document
}

/// The v3 X0 document with a bend over its region member.
fn bend(consumption: &str) -> Value {
    let mut document = v3();
    document["model"]["components"] = json!([{"id":"component:bend","kind":"bend","node":"node:fixture-tip",
        "geometry":{"bend_pipe_ref":"pipe:fixture-span","bend_radius":{"value":1.0,"unit":"m"},
            "bend_plane_orientation":"invented","bend_geometry_source_reference":"invented"},
        "modifiers":{"flexibility_factor_user_value":{"value":1.0,"unit":"none"},"source_reference":"invented"},
        "mechanics_interface":{"solver_consumption":consumption},
        "provenance":"invented_u2a_control"}]);
    document
}

/// The runner's pressure-1 export document: produced, bound to pressure-1 and
/// admitted by the exporter's independent accounting check.
fn assert_pressure_1_document(output: &open_pipe_stress_headless_runner::PreviewRunnerOutput, raw: &Value, label: &str) {
    assert!(output.canonical_export_unavailability.is_none(), "{label}: {:?}", output.canonical_export_unavailability);
    let document = output.result_envelope_document.as_ref().expect("an export document");
    assert_eq!(document["result_envelope"]["semantic_contract_ref"]["ref_id"], PRESSURE_ID, "{label}");
    assert_eq!(document["result_envelope"]["contract_evidence"], raw["contract_evidence"], "{label}");
    open_pipe_stress_result_export::derivative::validate_document(document, raw)
        .unwrap_or_else(|e| panic!("{label}: {e}"));
}

/// T4-U2: a realized bend on the region member is admitted and solves through
/// the runner in both modes, published Passed with its arc pressure rows, and
/// exported under pressure-1 (phase 2).
#[test]
fn a_v3_document_with_a_realized_bend_solves_and_exports_under_pressure_1() {
    let payload = bend("curved_bend_macro_element");
    for mode in MODES {
        let output =
            run_preview_model_value_with_mode(request(&payload["model"]), payload.clone(), mode)
                .unwrap();
        let raw = serde_json::to_value(output.mechanics_envelope.as_ref().unwrap()).unwrap();
        assert_eq!(raw["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}: {:#?}", raw["diagnostics"]);
        assert_eq!(raw["producer"]["semantic_contract_id"], PRESSURE_ID, "{mode:?}");
        assert!(raw["results"].as_array().unwrap().iter().any(|row| row["entity_ref"] == "pipe:fixture-span"
            && row["kind"] == "pipe_wall_axial_force_v2"));
        assert!(raw["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == "NUMERICAL_INTEGRITY_CHECKS_PASSED"));
        assert_pressure_1_document(&output, &raw, &format!("{mode:?}"));
    }
}

#[test]
fn families_not_yet_admitted_are_blocked_by_name_without_export_in_both_modes() {
    let mut valve = v3();
    valve["model"]["components"] = json!([{"id":"component:valve","kind":"valve",
        "node":"node:fixture-tip","provenance":"invented_u2a_control"}]);
    for (name, payload, id, family) in [
        ("valve", valve, "component:valve", "valve"),
        ("geometry-only bend", bend("mechanics_geometry_only"), "component:bend", "geometry-only bend"),
    ] {
        for mode in MODES {
            let output = run_preview_model_value_with_mode(
                request(&payload["model"]),
                payload.clone(),
                mode,
            )
            .unwrap();
            let raw = serde_json::to_value(output.mechanics_envelope.as_ref().unwrap()).unwrap();
            assert_eq!(raw["status"]["mechanics"], "MODEL_INCOMPLETE", "{name} {mode:?}");
            assert_eq!(raw["results"], json!([]), "{name} {mode:?}");
            assert_eq!(raw["producer"]["semantic_contract_id"], PRESSURE_ID, "{name} {mode:?}");
            assert!(
                raw["diagnostics"].as_array().unwrap().iter().any(|d| d["code"]
                    == "EXACT_PRESSURE_FAMILY_NOT_ADMITTED"
                    && d["affected_refs"] == json!([id])
                    && d["severity"] == "blocking"
                    && d["message"].as_str().unwrap().starts_with(family)),
                "{name} {mode:?}: {:#?}",
                raw["diagnostics"]
            );
            assert!(output.result_envelope_document.is_none(), "{name} {mode:?}");
            assert!(output.qualified_preview_evidence.is_none(), "{name} {mode:?}");
            assert_eq!(
                output.canonical_export_unavailability.as_deref(),
                Some("SOURCE_NOT_SOLVED"),
                "{name} {mode:?}"
            );
        }
    }
}

#[test]
fn a_straight_v3_document_solves_under_pressure_1_in_both_modes() {
    let payload = v3();
    for mode in MODES {
        let output =
            run_preview_model_value_with_mode(request(&payload["model"]), payload.clone(), mode)
                .unwrap();
        let raw = serde_json::to_value(output.mechanics_envelope.as_ref().unwrap()).unwrap();
        assert_eq!(raw["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}");
        assert_eq!(raw["producer"]["semantic_contract_id"], PRESSURE_ID, "{mode:?}");
        open_pipe_stress_result_export::semantic_contract::validate_pressure_evidence(&raw)
            .unwrap();
        // T4-U2 phase 2: the results 0.3 export schema has its pressure-1 branch.
        assert_pressure_1_document(&output, &raw, &format!("{mode:?}"));
    }
}
