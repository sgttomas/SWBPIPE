//! T4-U3 (S14): the objective connector's publication under `pressure-1`.
//! A solved v3 connector envelope carries one `contract_evidence.connector`
//! record per connector and the connector rows bound to it, discloses that no
//! joint temperature law is provided, and is admitted by the pressure-1 reader;
//! the shared reader corpus (RE, Python, TypeScript) is generated here.
//!
//! The inputs are the frozen T4-I12 system documents (0.3.0 and 0.4.0), bytes
//! unchanged. Under pressure-1 the variable spring hanger's input review rows
//! are model scoped (no load case), so the readers admit them (T4-U2).
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use open_pipe_stress_result_export::semantic_contract;
use serde_json::{json, Value};

const REFERENCES: &str =
    include_str!("../../../validation/references/t4_i12/u3_reference_cases.json");
const CORPUS: &str =
    include_str!("../../../fixtures/results/pressure_v3_connector_reader_corpus.json");
const CORPUS_PATH: &str = "../../fixtures/results/pressure_v3_connector_reader_corpus.json";
const TEMPERATURE_LAW: &str = "CONNECTOR_TEMPERATURE_LAW_NOT_PROVIDED";
const INPUTS: [(&str, &str, PreviewSolverMode, &str); 2] = [
    ("U3-SYS-DEMO-CONNECTOR-001", "document_v3_0.3.0", PreviewSolverMode::SparseInteractive, "sparse_interactive"),
    ("U3-SYS-DEMO-CONNECTOR-002-LR1", "document_v3_0.4.0", PreviewSolverMode::DenseScrutiny, "dense_scrutiny"),
];

/// A frozen system document, bytes unchanged.
fn document(case: &str, key: &str) -> Value {
    let references: Value = serde_json::from_str(REFERENCES).unwrap();
    references["cases"][case]["inputs"][key].clone()
}

fn run(document: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_value_with_mode(document.clone(), mode).unwrap())
        .unwrap()
}

#[test]
fn connector_envelopes_publish_their_records_and_the_temperature_disclosure() {
    for (case, key, _, _) in INPUTS {
        let document = document(case, key);
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let envelope = run(&document, mode);
            assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{case} {mode:?}");
            semantic_contract::validate_pressure_evidence(&envelope)
                .unwrap_or_else(|e| panic!("{case} {mode:?}: {e}"));
            let records = envelope["contract_evidence"]["connector"].as_array().unwrap();
            assert_eq!(records.len(), 1, "{case}");
            let record = &records[0];
            assert_eq!(record["component_id"], "component:C-150");
            assert_eq!(record["replaced_pipe_id"], "pipe:P-130");
            assert_eq!((record["node_i"].as_str(), record["node_j"].as_str()), (Some("node:N-130"), Some("node:N-140")));
            assert_eq!(record["work_matrix"]["translation_scale_m"], 0.5);
            assert_eq!(record["work_matrix"]["upper_triangle"][0], 800000.0);
            assert_eq!(record["installed_reference_temperature_k"], 293.15);
            let disclosures: Vec<&Value> = envelope["diagnostics"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|d| d["code"] == TEMPERATURE_LAW)
                .collect();
            assert_eq!(disclosures.len(), 1, "{case} {mode:?}");
            assert_eq!(disclosures[0]["severity"], "info");
            assert_eq!(disclosures[0]["affected_refs"], json!(["component:C-150", "pipe:P-130"]));
        }
    }
}

#[test]
fn a_model_without_a_connector_has_no_record_or_disclosure() {
    let mut document = document(INPUTS[0].0, INPUTS[0].1);
    let model = &mut document["model"];
    model["components"].as_array_mut().unwrap().clear();
    let envelope = run(&document, PreviewSolverMode::SparseInteractive);
    assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{}", envelope["diagnostics"]);
    assert_eq!(envelope["contract_evidence"]["connector"], json!([]));
    assert!(!envelope["diagnostics"].to_string().contains(TEMPERATURE_LAW));
    assert!(!envelope["results"].to_string().contains("connector_"));
}

/// The shared connector reader corpus (RE, Python, TypeScript): the 0.3.0
/// system document in sparse mode and the 0.4.0 one in dense mode, captured
/// entry. Regenerate with `T4_U3_WRITE_CORPUS=1`; otherwise the committed
/// bytes must be current.
#[test]
fn shared_connector_reader_corpus_is_current() {
    let mut cases = Vec::new();
    for (case, key, mode, name) in INPUTS {
        let document = document(case, key);
        let envelope = run(&document, mode);
        assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED");
        semantic_contract::validate_pressure_evidence(&envelope).unwrap();
        cases.push(json!({"label": format!("{case} {name}"),
            "document_schema_version": document["model"]["schema_version"], "solver_mode": name, "envelope": envelope}));
    }
    let corpus = json!({
        "corpus": "T4-U3 pressure-1 objective connector reader corpus",
        "generator": "core/product_physics/tests/connector_readers.rs::shared_connector_reader_corpus_is_current",
        "inputs": ["validation/references/t4_i12/u3_reference_cases.json U3-SYS-DEMO-CONNECTOR-001 document_v3_0.3.0 and U3-SYS-DEMO-CONNECTOR-002-LR1 document_v3_0.4.0, bytes unchanged"],
        "cases": cases,
    });
    let bytes = serde_json::to_string_pretty(&corpus).unwrap() + "\n";
    if std::env::var_os("T4_U3_WRITE_CORPUS").is_some() {
        std::fs::write(CORPUS_PATH, &bytes).unwrap();
        return;
    }
    assert!(CORPUS == bytes, "the shared connector reader corpus is stale; regenerate it");
}
