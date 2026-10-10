//! T4-U2a through the headless runner: a v3 (`3.0.0/exact_pressure_v3`)
//! document with a family the seam does not yet admit (a valve; a realized
//! bend on a region member) is blocked by name with no export, in both solver
//! modes; a straight v3 document solves under `pressure-1`, and its export is
//! refused by name (`PRESSURE_1_EXPORT_NOT_AVAILABLE`) until T4-U2. The input
//! is PP's committed invented X0 fixture with only the contract identity
//! changed.
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

#[test]
fn families_not_yet_admitted_are_blocked_by_name_without_export_in_both_modes() {
    let mut valve = v3();
    valve["model"]["components"] = json!([{"id":"component:valve","kind":"valve",
        "node":"node:fixture-tip","provenance":"invented_u2a_control"}]);
    let mut bend = v3();
    bend["model"]["components"] = json!([{"id":"component:bend","kind":"bend","node":"node:fixture-tip",
        "geometry":{"bend_pipe_ref":"pipe:fixture-span","bend_radius":{"value":1.0,"unit":"m"},
            "bend_plane_orientation":"invented","bend_geometry_source_reference":"invented"},
        "modifiers":{"flexibility_factor_user_value":{"value":1.0,"unit":"none"},"source_reference":"invented"},
        "mechanics_interface":{"solver_consumption":"curved_bend_macro_element"},
        "provenance":"invented_u2a_control"}]);
    for (name, payload, id, family) in [
        ("valve", valve, "component:valve", "valve"),
        ("realized bend", bend, "component:bend", "realized curved bend"),
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
        // The results 0.3 export schema has no pressure-1 branch yet (its
        // digests are pinned by the readers and generation manifests; T4-U2
        // owns the export chain), so export is refused by name.
        assert!(output.result_envelope_document.is_none(), "{mode:?}");
        assert_eq!(
            output.canonical_export_unavailability.as_deref(),
            Some(
                "result-envelope production failed structurally: PRESSURE_1_EXPORT_NOT_AVAILABLE: \
                 pressure-1 (3.0.0/exact_pressure_v3) result export is not yet available; it \
                 arrives with T4-U2"
            ),
            "{mode:?}"
        );
    }
}
