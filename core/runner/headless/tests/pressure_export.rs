//! T4-U2 phase 2: pressure-2 (`3.0.0/exact_pressure_v3`) through the headless
//! runner's export chain, in both solver modes: every document of the shared
//! arc reader corpus (realized arcs, B7b: all on the current 0.9.0 document,
//! the admitted kink, an arc beside a connector, a replaced-span-only model)
//! and the frozen T4-I12 connector system documents (with their variable
//! spring hanger; B7b: each frozen 0.3.0/0.4.0 copy re-authored to 0.9.0 by the
//! shared rule in `core/product_physics/tests/common/current_model.rs`), and the L
//! line authored from its corner through the operation applier (T4-U2 phase 3,
//! `fixtures/model_operations/pressure_v3_l_line_authoring.json`) produce a
//! pressure-2 export document that the exporter's independent check admits,
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

#[path = "../../../product_physics/tests/common/current_model.rs"]
mod current_model;

const ARC_CORPUS: &str =
    include_str!("../../../../fixtures/results/pressure_v3_arc_reader_corpus.json");
const I12: &str = include_str!("../../../../validation/references/t4_i12/u3_reference_cases.json");
const AUTHORED: &str =
    include_str!("../../../../fixtures/model_operations/pressure_v3_l_line_authoring.json");
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
        let frozen = references["cases"][case]["inputs"][key].clone();
        out.push((format!("connector-{}", case.to_lowercase()), current_model::reauthor_request(frozen)));
    }
    let authored: Value = serde_json::from_str(AUTHORED).unwrap();
    // The applier authors the current document itself (no re-authoring here).
    assert_eq!(authored["authored_model"]["schema_version"], current_model::CURRENT_MODEL_VERSION);
    assert_eq!(authored["authored_model"]["pressure_contract"], current_model::current_contract());
    out.push(("authored-l-line".into(), serde_json::json!({"model": authored["authored_model"], "materials": []})));
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
    // T4-U4b: the arc corpus's directional-SIF case adds one document per mode.
    assert_eq!(exported, 18);
}

/// Epoch 8 (DEL-04-07 Design desktop-exact-outputs, section 6.2): the plain
/// pressure-2 result-export twins, the desktop's equal-input oracle for an
/// eligible plain result. The connected exact request solves through this
/// export chain to the committed raw `pressure_connected_mechanics_<mode>.json`
/// (equal JSON values and canonical digests; the files' key order and
/// whitespace differ), and the document is derived from that source. The
/// committed `pressure_plain_connected_<mode>.document.json` is checked byte
/// for byte, and rewritten only with `HEADLESS_PLAIN_TWIN_WRITE_FIXTURES=1`.
#[test]
fn exact_pressure_connected_plain_document_twins_in_both_modes() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let read = |relative: &str| -> Value {
        serde_json::from_slice(&std::fs::read(root.join(relative)).unwrap()).unwrap()
    };
    let payload = read("core/product_physics/tests/fixtures/exact_pressure_connected_request.json");
    for (short, mode) in [("sparse", PreviewSolverMode::SparseInteractive), ("dense", PreviewSolverMode::DenseScrutiny)] {
        let output = run_preview_model_value_with_mode(request(&payload["model"]), payload.clone(), mode).unwrap();
        let raw = serde_json::to_value(output.mechanics_envelope.as_ref().unwrap()).unwrap();
        let committed = read(&format!("fixtures/results/pressure_connected_mechanics_{short}.json"));
        assert_eq!(raw, committed, "{short}: the actual producer output is the committed raw");
        assert_eq!(derivative::digest(&raw).unwrap(), derivative::digest(&committed).unwrap(), "{short}");
        assert_eq!(raw["producer"]["semantic_contract_id"], s::PRESSURE_ID, "{short}");
        assert!(raw["contract_evidence"].get("load_reference_states").is_none(), "{short}: a plain result");
        let document = output
            .result_envelope_document
            .as_ref()
            .unwrap_or_else(|| panic!("{short}: {:?}", output.canonical_export_unavailability));
        derivative::validate_document(document, &committed).unwrap_or_else(|e| panic!("{short}: {e}"));
        assert_eq!(document["result_envelope"]["contract_evidence"], committed["contract_evidence"], "{short}");
        // Compact bytes plus a newline, as the load/reference-state document twins are written.
        let text = serde_json::to_string(document).unwrap() + "\n";
        let path = root.join(format!("fixtures/results/pressure_plain_connected_{short}.document.json"));
        if std::env::var("HEADLESS_PLAIN_TWIN_WRITE_FIXTURES").as_deref() == Ok("1") {
            std::fs::write(&path, &text).unwrap();
        }
        assert_eq!(std::fs::read_to_string(&path).unwrap(), text, "{short}: document twin bytes");
        let reread: Value = serde_json::from_str(&text).unwrap();
        derivative::validate_document(&reread, &committed).unwrap();
    }
}
