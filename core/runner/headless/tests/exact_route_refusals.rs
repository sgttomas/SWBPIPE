//! T4-U0 (exact-route hardening) through the headless runner: an exact
//! document refused for a metadata-only component (A1.1) or a combination
//! (A1.6) is blocked with no export, in both
//! solver modes. The input is PP's committed invented X0 fixture.
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

fn request(model: &Value) -> RunnerRequest {
    RunnerRequest {
        request_id: "t4-u0-exact-route-refusals".into(),
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
        input_manifest_ref: Reference::new("audit_manifest", "t4-u0-input-manifest"),
        requested_outputs: vec![
            "result_envelope".into(),
            "audit_manifest".into(),
            "diagnostics".into(),
        ],
        privacy: PrivacyContext::local_first_public_metadata(),
        provenance: Provenance {
            source_name: "invented T4-U0 refusal fixture".into(),
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

#[test]
fn exact_route_refusals_are_blocked_without_export_in_both_modes() {
    let base: Value = serde_json::from_str(X0).unwrap();
    let mut valve = base.clone();
    valve["model"]["components"] = json!([{"id":"component:valve","kind":"valve",
        "node":"node:fixture-tip","provenance":"invented_u0_control"}]);
    let mut combination = base.clone();
    combination["model"]["combinations"] = json!([{"id":"combination:sum","basis":"mechanics",
        "terms":[{"load_case":"case:closed-pressure","factor":1.0},
            {"load_case":"case:six-component-load","factor":1.0}],
        "provenance":"invented_u0_control"}]);
    for (name, payload, code, refs) in [
        (
            "A1.1",
            valve,
            "EXACT_PRESSURE_COMPOSITION_UNSUPPORTED",
            json!(["component:valve"]),
        ),
        (
            "A1.6",
            combination,
            "EXACT_PRESSURE_COMBINATION_UNSUPPORTED",
            json!(["combination:sum"]),
        ),
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
            assert_eq!(
                raw["contract_evidence"],
                json!({"pressure":[],"connector":[],"exact_cases":[]}),
                "{name} {mode:?}"
            );
            assert!(
                raw["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == code
                    && d["affected_refs"] == refs
                    && d["severity"] == "blocking"),
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
