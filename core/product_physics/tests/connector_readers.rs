//! T4-U3 (S14): the objective connector's publication under `pressure-2`.
//! A solved v3 connector envelope carries one `contract_evidence.connector`
//! record per connector and the connector rows bound to it, discloses that no
//! joint temperature law is provided, and is admitted by the pressure-2 reader;
//! the shared reader corpus (RE, Python) is generated here.
//!
//! The inputs are the frozen T4-I12 system documents (0.3.0 and 0.4.0), bytes
//! unchanged; B7b re-authors each copy to the current 0.6.0 exact document
//! (`common/current_model.rs`). Under pressure-2 the variable spring hanger's input review rows
//! are model scoped (no load case), so the readers admit them (T4-U2).
//!
//! T4-U5b: the corpus also carries two connectors with explicit elastic tie
//! rods, authored in 0.6.0 from the frozen t4_u5b case descriptions by the
//! shared builder (`common/t4_u5b.rs`): AS-F (a listed constant-effective-area
//! joint with one offset rod) and MO-F (an unpressurized connector with two
//! rods, no region).
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use open_pipe_stress_result_export::semantic_contract;
use serde_json::{json, Value};

#[path = "common/current_model.rs"]
mod current_model;
#[path = "common/t4_u5b.rs"]
mod t4_u5b;

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

/// A frozen system document (bytes unchanged), its copy re-authored to 0.6.0.
fn document(case: &str, key: &str) -> Value {
    let references: Value = serde_json::from_str(REFERENCES).unwrap();
    current_model::reauthor_request(references["cases"][case]["inputs"][key].clone())
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

/// T4-U5b's tied corpus entries: (t4_u5b case, mode, mode name).
const TIED_INPUTS: [(&str, PreviewSolverMode, &str); 2] = [
    ("AS-F", PreviewSolverMode::SparseInteractive, "sparse_interactive"),
    ("MO-F", PreviewSolverMode::DenseScrutiny, "dense_scrutiny"),
];

/// The connector's tie rod 0 of a built t4_u5b request.
fn rod0(request: &mut Value) -> &mut Value {
    &mut request["model"]["components"][0]["objective_connector"]["hardware"]["rods"][0]
}

fn length(value: f64) -> Value {
    json!({"value": value, "unit": "m"})
}

/// Pe = 0 and g_x at noise level: the connector unpressurized, no region and
/// no load, so only the rod's pretension (e_ref = -1e-4 m, k = 1e7 N/m:
/// T about 1 kN) loads the joint.
fn pretension_only(request: &mut Value) {
    let model = &mut request["model"];
    model["components"][0]["objective_connector"]["pressure_model"] = json!({"kind": "unpressurized"});
    model["load_cases"][0]["pressure_regions"] = json!([]);
    model["load_cases"][0]["primitive_loads"] = json!([]);
    let rod = rod0(request);
    rod["stiffness"] = json!({"value": 1.0e7, "unit": "N/m"});
    rod["reference_extension"] = length(-1.0e-4);
}

/// T4-U5b round 4 (s_x on a common operand): derived tied requests whose
/// rods are near-perpendicular or skewed, so the share formed at p and the
/// published binary64 axial_share differ while g_x and Pe are small.
fn derived_tied(name: &str) -> Value {
    let oracle = t4_u5b::oracle();
    match name {
        // Codex's shape: the represented chord (x_j - x_i) + (o_j - o_i) is
        // fl(2.3 - 2) + fl(1e-17 - fl(2.3 - 2)) = 0, so the published share is
        // 0 while the share at p is about 3e-17; Pe = 0, g_x at noise level.
        "MO-F perpendicular 1e-17 skew, pretension only" => {
            let mut request = t4_u5b::request("MO-F", &oracle["cases"]["MO-F"]);
            pretension_only(&mut request);
            let rod = rod0(&mut request);
            rod["offset_i"]["x"] = length(2.3 - 2.0);
            rod["offset_j"]["x"] = length(1.0e-17);
            rod["offset_j"]["y"] = length(0.4);
            request
        }
        // Opus's shape: AS-F-R's rotated connector frame, the rod
        // perpendicular in connector axes (along connector y from the end-j
        // station), so its global direction carries rotation roundoff.
        "AS-F-R rotated perpendicular, pretension only" => {
            let mut request = t4_u5b::request("AS-F-R", &oracle["cases"]["AS-F-R"]);
            pretension_only(&mut request);
            let model = &request["model"];
            let position = |id: &str| -> [f64; 3] {
                let node = model["nodes"].as_array().unwrap().iter().find(|n| n["id"] == id).unwrap();
                ["x", "y", "z"].map(|a| node["position"][a].as_f64().unwrap())
            };
            let connector = &model["components"][0]["objective_connector"];
            let (i, j) = (position(connector["end_i"]["node_ref"].as_str().unwrap()), position(connector["end_j"]["node_ref"].as_str().unwrap()));
            let chord = ((j[0] - i[0]).powi(2) + (j[1] - i[1]).powi(2) + (j[2] - i[2]).powi(2)).sqrt();
            let rod = rod0(&mut request);
            rod["offset_i"]["x"] = length(chord);
            rod["offset_j"]["x"] = length(0.0);
            rod["offset_j"]["y"] = length(0.4);
            request
        }
        // A skewed rod with large offsets: its represented chord cancels
        // metres of offset to centimetres.
        "AS-F skewed rod with large offsets" => {
            let mut request = t4_u5b::request("AS-F", &oracle["cases"]["AS-F"]);
            let rod = rod0(&mut request);
            rod["offset_i"] = json!({"x": length(12.5), "y": length(3.25), "z": length(-1.5)});
            rod["offset_j"] = json!({"x": length(-12.15), "y": length(3.2500001), "z": length(-1.4999)});
            request
        }
        _ => unreachable!("{name}"),
    }
}

const DERIVED_TIED_INPUTS: [(&str, PreviewSolverMode, &str); 3] = [
    ("MO-F perpendicular 1e-17 skew, pretension only", PreviewSolverMode::SparseInteractive, "sparse_interactive"),
    ("AS-F-R rotated perpendicular, pretension only", PreviewSolverMode::DenseScrutiny, "dense_scrutiny"),
    ("AS-F skewed rod with large offsets", PreviewSolverMode::SparseInteractive, "sparse_interactive"),
];

/// The shared connector reader corpus (RE, Python): the 0.3.0
/// system document in sparse mode and the 0.4.0 one in dense mode, each
/// re-authored to 0.6.0, then (T4-U5b) the two tied requests built from
/// t4_u5b, captured entry. Regenerate with `T4_U3_WRITE_CORPUS=1`; otherwise the committed
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
    let oracle = t4_u5b::oracle();
    for (case, mode, name) in TIED_INPUTS {
        let document = t4_u5b::request(case, &oracle["cases"][case]);
        let envelope = run(&document, mode);
        assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{case}");
        semantic_contract::validate_pressure_evidence(&envelope).unwrap_or_else(|e| panic!("{case}: {e}"));
        cases.push(json!({"label": format!("T4-U5b {case} {name}"),
            "document_schema_version": document["model"]["schema_version"], "solver_mode": name, "envelope": envelope}));
    }
    for (name, mode, mode_name) in DERIVED_TIED_INPUTS {
        let document = derived_tied(name);
        let envelope = run(&document, mode);
        assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{name}: {}", envelope["diagnostics"]);
        semantic_contract::validate_pressure_evidence(&envelope).unwrap_or_else(|e| panic!("{name}: {e}"));
        cases.push(json!({"label": format!("T4-U5b {name} {mode_name}"),
            "document_schema_version": document["model"]["schema_version"], "solver_mode": mode_name, "envelope": envelope}));
    }
    let corpus = json!({
        "corpus": "T4-U3 pressure-1 objective connector reader corpus",
        "generator": "core/product_physics/tests/connector_readers.rs::shared_connector_reader_corpus_is_current",
        "inputs": ["validation/references/t4_i12/u3_reference_cases.json U3-SYS-DEMO-CONNECTOR-001 document_v3_0.3.0 and U3-SYS-DEMO-CONNECTOR-002-LR1 document_v3_0.4.0, bytes unchanged; each copy re-authored to model document 0.6.0 (exact_pressure_v3, euler_bernoulli, corrosion allowance 0 m) at test time",
            "validation/references/t4_u5b cases AS-F and MO-F (SHA-bound case descriptions over their t4_u5a base requests), authored as model document 0.6.0 requests with explicit_axial_rods_v1 hardware at test time (core/product_physics/tests/common/t4_u5b.rs)",
            "three requests derived at test time from t4_u5b MO-F, AS-F-R and AS-F (connector_readers.rs derived_tied): near-perpendicular or skewed rods, two with Pe = 0 and pretension only; invented controls, not references"],
        "cases": cases,
    });
    let bytes = serde_json::to_string_pretty(&corpus).unwrap() + "\n";
    if std::env::var_os("T4_U3_WRITE_CORPUS").is_some() {
        std::fs::write(CORPUS_PATH, &bytes).unwrap();
        return;
    }
    assert!(CORPUS == bytes, "the shared connector reader corpus is stale; regenerate it");
}
