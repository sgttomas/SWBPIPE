//! T4-U2 phase 2: pressure-1 (`3.0.0/exact_pressure_v3`) through the headless
//! runner's export chain, in both solver modes: every document of the shared
//! arc reader corpus (realized arcs on 0.3.0 and 0.4.0, the admitted kink, an
//! arc beside a connector, a replaced-span-only model) and the frozen T4-I12
//! connector system documents (with their variable spring hanger) produce a
//! pressure-1 export document that the exporter's independent check admits,
//! with its contract evidence copied whole.
//!
//! With `HEADLESS_PRESSURE_OUTPUT_DIR` set, each case writes
//! `<name>.raw.json` and `<name>.document.json` there for the Python schema
//! and AnalysisRun consumer `tests/test_pressure_headless_artifacts.py`.
use open_pipe_stress_headless_runner::{
    run_preview_model_value_with_mode, PrivacyContext, ProfessionalBoundary, Provenance,
    RedistributionStatus, Reference, RunnerOperation, RunnerRequest, TbdDecisions,
};
use open_pipe_stress_product_physics::PreviewSolverMode;
use open_pipe_stress_result_export::{derivative, semantic_contract as s};
use serde_json::Value;

const ARC_CORPUS: &str =
    include_str!("../../../../fixtures/results/pressure_v3_arc_reader_corpus.json");
const I12: &str = include_str!("../../../../validation/references/t4_i12/u3_reference_cases.json");
const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];

fn request(model: &Value) -> RunnerRequest {
    RunnerRequest {
        request_id: "t4-u2-pressure-export".into(),
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
        input_manifest_ref: Reference::new("audit_manifest", "t4-u2-input-manifest"),
        requested_outputs: vec!["result_envelope".into(), "audit_manifest".into(), "diagnostics".into()],
        privacy: PrivacyContext::local_first_public_metadata(),
        provenance: Provenance {
            source_name: "invented T4-U2 pressure-1 export cases".into(),
            source_location: "fixtures/results; validation/references/t4_i12".into(),
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

/// (artifact stem, solve payload).
fn inputs() -> Vec<(String, Value)> {
    let corpus: Value = serde_json::from_str(ARC_CORPUS).unwrap();
    let mut out: Vec<(String, Value)> = corpus["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|case| {
            let label = case["label"].as_str().unwrap();
            let stem = label.rsplit_once(' ').unwrap().0;
            (format!("arc-{}", stem.replace([' ', '.'], "-").to_lowercase()), case["document"].clone())
        })
        .collect();
    let references: Value = serde_json::from_str(I12).unwrap();
    for (case, key) in [("U3-SYS-DEMO-CONNECTOR-001", "document_v3_0.3.0"), ("U3-SYS-DEMO-CONNECTOR-002-LR1", "document_v3_0.4.0")] {
        out.push((format!("connector-{}", case.to_lowercase()), references["cases"][case]["inputs"][key].clone()));
    }
    out
}

#[test]
fn pressure_1_documents_export_in_both_modes() {
    let folder = std::env::var_os("HEADLESS_PRESSURE_OUTPUT_DIR").map(std::path::PathBuf::from);
    let mut exported = 0;
    for (stem, payload) in inputs() {
        for mode in MODES {
            let name = format!("{stem}-{}", mode.as_str());
            let output =
                run_preview_model_value_with_mode(request(&payload["model"]), payload.clone(), mode)
                    .unwrap();
            let raw = serde_json::to_value(output.mechanics_envelope.as_ref().unwrap()).unwrap();
            assert_eq!(raw["producer"]["semantic_contract_id"], s::PRESSURE_ID, "{name}");
            let document = output
                .result_envelope_document
                .as_ref()
                .unwrap_or_else(|| panic!("{name}: {:?}", output.canonical_export_unavailability));
            derivative::validate_document(document, &raw).unwrap_or_else(|e| panic!("{name}: {e}"));
            let envelope = &document["result_envelope"];
            assert_eq!(envelope["semantic_contract_ref"]["ref_id"], s::PRESSURE_ID, "{name}");
            assert_eq!(envelope["contract_evidence"], raw["contract_evidence"], "{name}");
            assert_eq!(envelope["row_accounting"].as_array().unwrap().len(), raw["results"].as_array().unwrap().len());
            // Every pressure-family row is an exported quantity.
            for account in envelope["row_accounting"].as_array().unwrap() {
                let kind = account["source_kind"].as_str().unwrap();
                if kind.starts_with("pipe_") || kind.starts_with("connector_") {
                    assert_eq!(account["disposition"], "exported_quantity", "{name}: {kind}");
                }
            }
            if let Some(folder) = &folder {
                std::fs::create_dir_all(folder).unwrap();
                for (suffix, value) in [("raw", &raw), ("document", document)] {
                    std::fs::write(folder.join(format!("{name}.{suffix}.json")), serde_json::to_vec_pretty(value).unwrap()).unwrap();
                }
            }
            exported += 1;
        }
    }
    assert_eq!(exported, 14);
}
