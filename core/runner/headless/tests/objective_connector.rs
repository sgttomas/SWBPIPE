//! T4-U3 through the headless runner, in both solver modes, on the frozen
//! T4-I12 system documents (`validation/references/t4_i12/`, bytes
//! unchanged): an admitted unpressurized v3 objective connector solves under
//! `pressure-1` with its connector record, rows and temperature-law
//! disclosure, and is exported under pressure-1 (T4-U2 phase 2: the connector
//! rows are exported quantities with a stable basis); a legacy joint
//! on the same document is refused with `LEGACY_FINITE_CONNECTOR_REAUTHOR_REQUIRED`;
//! a case with a pressure region is refused with `JOINT_PRESSURE_INTERFACE_UNRESOLVED`.
use open_pipe_stress_headless_runner::{
    run_preview_model_value_with_mode, PrivacyContext, ProfessionalBoundary, Provenance,
    RedistributionStatus, Reference, RunnerOperation, RunnerRequest, TbdDecisions,
};
use open_pipe_stress_product_physics::PreviewSolverMode;
use serde_json::{json, Value};
use std::collections::BTreeSet;

const REFERENCES: &str =
    include_str!("../../../../validation/references/t4_i12/u3_reference_cases.json");
const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];
const PRESSURE_ID: &str = "openpipestress.result_semantics/0.3.0/pressure-1";

fn references() -> Value {
    serde_json::from_str(REFERENCES).unwrap()
}

fn request(model: &Value) -> RunnerRequest {
    RunnerRequest {
        request_id: "t4-u3-objective-connector".into(),
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
        input_manifest_ref: Reference::new("audit_manifest", "t4-u3-input-manifest"),
        requested_outputs: vec![
            "result_envelope".into(),
            "audit_manifest".into(),
            "diagnostics".into(),
        ],
        privacy: PrivacyContext::local_first_public_metadata(),
        provenance: Provenance {
            source_name: "invented T4-I12 system documents".into(),
            source_location: "validation/references/t4_i12".into(),
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

/// The runner's raw mechanics envelope, its export unavailability and its
/// export document.
fn run(payload: &Value, mode: PreviewSolverMode) -> (Value, Option<String>, Option<Value>) {
    let output =
        run_preview_model_value_with_mode(request(&payload["model"]), payload.clone(), mode)
            .unwrap();
    let raw = serde_json::to_value(output.mechanics_envelope.as_ref().unwrap()).unwrap();
    (raw, output.canonical_export_unavailability.clone(), output.result_envelope_document.clone())
}

fn blocking_codes(raw: &Value) -> BTreeSet<String> {
    raw["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|d| d["severity"] == "blocking")
        .map(|d| d["code"].as_str().unwrap().to_string())
        .collect()
}

/// A JSON patch of the round-02 variant form (`add` with `/-`, `replace`).
fn patched(base: &Value, operations: &Value) -> Value {
    let mut document = base.clone();
    for op in operations.as_array().unwrap() {
        let path = op["path"].as_str().unwrap();
        let (parent, last) = path.rsplit_once('/').unwrap();
        let target = document.pointer_mut(parent).unwrap();
        match (op["op"].as_str().unwrap(), last) {
            ("add", "-") => target.as_array_mut().unwrap().push(op["value"].clone()),
            ("add" | "replace", key) => target[key] = op["value"].clone(),
            (other, _) => panic!("unsupported patch op {other}"),
        }
    }
    document
}

#[test]
fn an_admitted_v3_connector_solves_and_exports_under_pressure_1_in_both_modes() {
    let references = references();
    for (case, key) in [
        ("U3-SYS-DEMO-CONNECTOR-001", "document_v3_0.3.0"),
        ("U3-SYS-DEMO-CONNECTOR-002-LR1", "document_v3_0.4.0"),
    ] {
        let payload = references["cases"][case]["inputs"][key].clone();
        let cases = payload["model"]["load_cases"].as_array().unwrap().len();
        for mode in MODES {
            let (raw, unavailability, document) = run(&payload, mode);
            assert_eq!(raw["status"]["mechanics"], "MECHANICS_SOLVED", "{case} {mode:?}: {}", raw["diagnostics"]);
            assert_eq!(raw["producer"]["semantic_contract_id"], PRESSURE_ID, "{case} {mode:?}");
            let records = raw["contract_evidence"]["connector"].as_array().unwrap();
            assert_eq!(records.len(), 1, "{case} {mode:?}");
            assert_eq!(records[0]["component_id"], "component:C-150");
            assert_eq!(records[0]["replaced_pipe_id"], "pipe:P-130");
            let connector_rows = raw["results"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|r| r["kind"].as_str().unwrap().starts_with("connector_"))
                .count();
            assert_eq!(connector_rows, 24 * cases, "{case} {mode:?}");
            assert!(!raw["results"].as_array().unwrap().iter().any(|r| r["entity_ref"] == "pipe:P-130"));
            assert!(raw["diagnostics"].as_array().unwrap().iter().any(|d| d["code"]
                == "CONNECTOR_TEMPERATURE_LAW_NOT_PROVIDED"
                && d["severity"] == "info"
                && d["affected_refs"] == json!(["component:C-150", "pipe:P-130"])));
            assert!(blocking_codes(&raw).is_empty(), "{case} {mode:?}");
            // T4-U2: exported under pressure-1, on 0.3.0 and 0.4.0 alike. The
            // spring hanger's input review rows are model scoped (pressure-1),
            // so the reader admits them; the numerical standing gate was never
            // the cause (it read the reader's refusal as "unsupported").
            assert!(unavailability.is_none(), "{case} {mode:?}: {unavailability:?}");
            let document = document.expect("an export document");
            open_pipe_stress_result_export::derivative::validate_document(&document, &raw)
                .unwrap_or_else(|e| panic!("{case} {mode:?}: {e}"));
            let envelope = &document["result_envelope"];
            assert_eq!(envelope["semantic_contract_ref"]["ref_id"], PRESSURE_ID, "{case} {mode:?}");
            // Every connector row is an exported quantity with canonical
            // metadata (components, frame, location, the stable basis).
            let exported: Vec<&Value> = envelope["result_sets"][0]["values"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|v| v["source_kind"].as_str().unwrap().starts_with("connector_"))
                .collect();
            assert_eq!(exported.len(), 24 * cases, "{case} {mode:?}");
            assert!(exported.iter().all(|v| v["metadata"]["basis"]
                == open_pipe_stress_result_export::semantic_contract::CONNECTOR_ROW_BASIS));
        }
    }
}

#[test]
fn a_legacy_joint_on_a_v3_document_is_refused_without_export_in_both_modes() {
    let references = references();
    let mut payload = references["cases"]["U3-SYS-DEMO-CONNECTOR-001"]["inputs"]["document_v3_0.3.0"].clone();
    payload["model"]["components"] = json!([{
        "id": "component:C-150", "kind": "expansion_joint", "node": "node:N-140",
        "geometry": {"expansion_joint_pipe_ref": "pipe:P-130"},
        "modifiers": {
            "axial_stiffness_user_value": {"value": 3200000, "unit": "N/m"},
            "lateral_stiffness_user_value": {"value": 900000, "unit": "N/m"},
            "angular_stiffness_user_value": {"value": 480000, "unit": "N*m/rad"},
            "torsional_stiffness_user_value": {"value": 620000, "unit": "N*m/rad"},
            "source_reference": "invented legacy four-rate joint (T4-U3 control)"
        },
        "mechanics_interface": {"solver_consumption": "mechanics_geometry_and_user_flexibility"},
        "provenance": "invented legacy four-rate joint (T4-U3 control)"
    }]);
    for mode in MODES {
        let (raw, unavailability, document) = run(&payload, mode);
        let exported = document.is_some();
        assert_eq!(raw["status"]["mechanics"], "MODEL_INCOMPLETE", "{mode:?}");
        assert_eq!(raw["results"], json!([]), "{mode:?}");
        assert_eq!(
            blocking_codes(&raw),
            BTreeSet::from(["LEGACY_FINITE_CONNECTOR_REAUTHOR_REQUIRED".to_string()]),
            "{mode:?}: {}",
            raw["diagnostics"]
        );
        assert!(raw["diagnostics"].as_array().unwrap().iter().any(|d| d["code"]
            == "LEGACY_FINITE_CONNECTOR_REAUTHOR_REQUIRED"
            && d["affected_refs"] == json!(["component:C-150", "pipe:P-130"])));
        assert!(!exported, "{mode:?}");
        assert_eq!(unavailability.as_deref(), Some("SOURCE_NOT_SOLVED"), "{mode:?}");
    }
}

#[test]
fn a_pressurized_case_with_a_joint_is_refused_without_export_in_both_modes() {
    let references = references();
    let case = &references["cases"]["U3-SYS-DEMO-CONNECTOR-001"];
    let variant = &case["refusal_variants"]["joint_case_with_pressure_region"];
    let payload = patched(
        &case["inputs"]["document_v3_0.3.0"],
        &variant["document_patch_round_02"]["operations"],
    );
    for mode in MODES {
        let (raw, unavailability, document) = run(&payload, mode);
        let exported = document.is_some();
        assert_eq!(raw["status"]["mechanics"], "MODEL_INCOMPLETE", "{mode:?}");
        assert_eq!(raw["results"], json!([]), "{mode:?}");
        let expected: BTreeSet<String> = variant["expected_round_02"]["blocking_codes_exactly"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| c.as_str().unwrap().to_string())
            .collect();
        assert_eq!(expected, BTreeSet::from(["JOINT_PRESSURE_INTERFACE_UNRESOLVED".to_string()]));
        assert_eq!(blocking_codes(&raw), expected, "{mode:?}: {}", raw["diagnostics"]);
        assert!(!exported, "{mode:?}");
        assert_eq!(unavailability.as_deref(), Some("SOURCE_NOT_SOLVED"), "{mode:?}");
    }
}
