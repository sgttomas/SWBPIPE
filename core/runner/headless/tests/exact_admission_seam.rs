//! T4-U2a through the headless runner: a v3 (`3.0.0/exact_pressure_v3`)
//! document with a family the seam does not admit (a tee, since W-15 admits
//! the valve; a geometry-only bend, D-2) is blocked by name with no export, in
//! both solver modes; a
//! straight v3 document, and since T4-U2 phase 1 one whose region member is a
//! realized bend, solves under `pressure-2`; since T4-U2 phase 2 both produce
//! a pressure-2 export document that the exporter's own check admits.
//! The input is PP's committed invented X0 fixture (B7b: already the current
//! 0.8.0 exact document) with the current contract stated explicitly.
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
const PRESSURE_ID: &str = "openpipestress.result_semantics/0.3.0/pressure-2";

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
    document["model"]["pressure_contract"] =
        json!({"version":"3.0.0","mode":"exact_pressure_v3","shear_model":"euler_bernoulli"});
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

/// The runner's pressure-2 export document: produced, bound to pressure-2 and
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
/// exported under pressure-2 (phase 2).
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
    // RT-A3 admits the tee, the earlier sample; `other` stays refused.
    let mut other = v3();
    other["model"]["components"] = json!([{"id":"component:other","kind":"other",
        "node":"node:fixture-tip","provenance":"invented_u2a_control"}]);
    for (name, payload, id, family) in [
        ("other", other, "component:other", "component"),
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
        // T4-U2 phase 2: the results 0.3 export schema has its pressure-2 branch.
        assert_pressure_1_document(&output, &raw, &format!("{mode:?}"));
    }
}

/// b7-plan MJ8 (D6 lapsed with T4-U2's export, #1314): through the runner's
/// export chain a selected straight v3 case publishes pressure-source-1, and its
/// export document carries and binds the received v3 receipt and evidence; a
/// changed, resealed receipt is refused by that binding.
#[test]
fn pressure_source_export_binds_the_v3_receipt_after_u2() {
    use open_pipe_stress_result_export::{derivative, source_blocks};
    const N05: &str = include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json");
    let payload: Value = serde_json::from_str(N05).unwrap();
    for mode in MODES {
        let output =
            run_preview_model_value_with_mode(request(&payload["model"]), payload.clone(), mode)
                .unwrap();
        let raw = serde_json::to_value(output.mechanics_envelope.as_ref().unwrap()).unwrap();
        assert_eq!(raw["producer"]["semantic_contract_id"], "openpipestress.result_semantics/0.3.0/pressure-source-1", "{mode:?}");
        let document = output
            .result_envelope_document
            .as_ref()
            .unwrap_or_else(|| panic!("{mode:?}: {:?}", output.canonical_export_unavailability));
        derivative::validate_document(document, &raw).unwrap_or_else(|e| panic!("{mode:?}: {e}"));
        let envelope = &document["result_envelope"];
        assert_eq!(envelope["semantic_contract_ref"]["ref_id"], raw["producer"]["semantic_contract_id"], "{mode:?}");
        assert_eq!(envelope["source_block_recovery"], raw["source_block_recovery"], "{mode:?}");
        assert_eq!(envelope["contract_evidence"], raw["contract_evidence"], "{mode:?}");
        let mut changed = document.clone();
        let receipt = &mut changed["result_envelope"]["source_block_recovery"];
        receipt["body"]["invocation"]["value"] = json!("0".repeat(64));
        receipt["receipt_sha256"] = json!(source_blocks::domain_hash("source_blocks_receipt_v1", &receipt["body"]).unwrap());
        assert_eq!(derivative::validate_document(&changed, &raw).unwrap_err(), "SOURCE_BLOCK_RECEIPT_BINDING_MISMATCH", "{mode:?}");
    }
}
