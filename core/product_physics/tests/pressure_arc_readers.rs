//! T4-U2 phase 2: realized arcs under `pressure-1` (`3.0.0/exact_pressure_v3`)
//! reach the readers. Generates the shared arc reader corpus (RE, Python,
//! TypeScript) from PP's public captured entry:
//!
//! - the T4-I7 L line (anchored, pressure + thermal + weight, k = 2) on 0.3.0
//!   (sparse) and 0.4.0 (dense), and the admitted kink (θ = 5e-4 rad);
//! - the same L line with an objective connector replacing an added straight
//!   tail, solved without pressure (a case with a connector and a pressure
//!   region is refused, `JOINT_PRESSURE_INTERFACE_UNRESOLVED`): the arc is a
//!   member outside any region, beside a connector record;
//! - a model whose only pipe is a replaced span (T4-RV23 NOTE-4).
//!
//! Regenerate with `T4_U2_WRITE_ARC_CORPUS=1`; otherwise the committed bytes
//! must be current.
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use open_pipe_stress_result_export::semantic_contract;
use serde_json::{json, Value};

const I7_DOCUMENTS: &str =
    include_str!("../../../validation/references/t4_i7/u2_document_sketches.json");
const CORPUS: &str = include_str!("../../../fixtures/results/pressure_v3_arc_reader_corpus.json");
const CORPUS_PATH: &str = "../../fixtures/results/pressure_v3_arc_reader_corpus.json";
const PROVENANCE: &str = "invented T4-U2 phase-2 reader corpus input (from T4-I7's L line); not library, component, catalog or code-rule data";

fn i7(id: &str, version: &str) -> Value {
    let documents: Value = serde_json::from_str(I7_DOCUMENTS).unwrap();
    documents["sketches"][id][version]["document"].clone()
}

fn run(document: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_value_with_mode(document.clone(), mode).unwrap())
        .unwrap()
}

/// An objective connector replacing `span` from `node_i` to `node_j`, whose
/// first connector axis (Q's first column) is the span direction.
fn connector(span: &str, node_i: &str, node_j: &str, axes: Value) -> Value {
    json!({"id": "component:C-1", "label": "Invented expansion joint (objective connector)", "kind": "expansion_joint",
        "node": node_j, "provenance": PROVENANCE,
        "objective_connector": {"version": "1.0.0", "motion_basis": "symmetric_midpoint_small_rotation_v1",
            "end_i": {"node_ref": node_i, "initial_node_axes_global": [[1, 0, 0], [0, 1, 0], [0, 0, 1]], "offset_local": {"x": 0, "y": 0, "z": 0, "unit": "m"}},
            "end_j": {"node_ref": node_j, "initial_node_axes_global": [[1, 0, 0], [0, 1, 0], [0, 0, 1]], "offset_local": {"x": 0, "y": 0, "z": 0, "unit": "m"}},
            "connector_axes_global": axes,
            "installed_reference_temperature": {"value": 20, "unit": "degC"},
            "temperature_applicability": "fixed_installed_parameters_v1", "reference_state": "stress_free",
            "q_ref": {"translation": {"x": 0, "y": 0, "z": 0, "unit": "m"}, "rotation": {"x": 0, "y": 0, "z": 0, "unit": "rad"}},
            "stiffness": {"version": "1.0.0", "representation": "scaled_work_coefficients_v1",
                "translation_scale": {"value": 0.5, "unit": "m"}, "rotation_scale": {"value": 1, "unit": "rad"}, "coefficient_unit": "N*m",
                "upper_triangle": [800000, 0, 0, 0, 0, 0, 225000, 0, 0, 0, 0, 225000, 0, 0, 0, 620000, 0, 0, 480000, 0, 480000],
                "coordinate_order": ["tx", "ty", "tz", "rx", "ry", "rz"],
                "provenance": {"source_reference": PROVENANCE, "measurement_restraints": "invented", "basis_transform_reference": "none"}},
            "topology": {"type": "replaces_span", "span_ref": span},
            "calibration": {"kind": "constant_structural_elasticity_v1", "includes_pressure_dependent_tangent": false, "installed_geometry": "authored model geometry"},
            "hardware": {"kind": "untied"}, "pressure_model": {"kind": "unpressurized"},
            "provenance": {"source_reference": PROVENANCE, "measurement_restraints": "invented", "basis_transform_reference": "none", "validity_statement": "invented analytical control; no applicability claim"}}})
}

/// The L line with an added straight tail D→E (+y) replaced by a connector,
/// anchored at A and E, solved under thermal and weight only.
fn l_line_with_connector() -> Value {
    let mut document = i7("U2-L-ANCH-PTW-K2", "0.3.0");
    let model = &mut document["model"];
    model["project"]["id"] = json!("project:u2-arc-connector");
    model["nodes"].as_array_mut().unwrap().push(json!({"id": "node:E", "position": {"x": 3.25, "y": 4.75, "z": 0.0}, "provenance": PROVENANCE}));
    let mut tail = model["pipe_segments"][2].clone();
    tail["id"] = json!("pipe:S3");
    tail["from"] = json!("node:D");
    tail["to"] = json!("node:E");
    model["pipe_segments"].as_array_mut().unwrap().push(tail);
    model["components"].as_array_mut().unwrap().push(connector(
        "pipe:S3",
        "node:D",
        "node:E",
        json!([[0, -1, 0], [1, 0, 0], [0, 0, 1]]),
    ));
    for support in model["supports"].as_array_mut().unwrap() {
        if support["id"] == "support:D" {
            support["id"] = json!("support:E");
            support["node"] = json!("node:E");
        }
    }
    for case in model["load_cases"].as_array_mut().unwrap() {
        case["pressure_regions"] = json!([]);
    }
    document
}

/// One pipe, replaced by a connector: anchored at A, a lateral force at B.
fn replaced_span_only() -> Value {
    let mut document = i7("U2-L-ANCH-PTW-K2", "0.3.0");
    let model = &mut document["model"];
    model["project"]["id"] = json!("project:u2-replaced-span-only");
    model["nodes"].as_array_mut().unwrap().retain(|n| n["id"] == "node:A" || n["id"] == "node:B");
    model["pipe_segments"].as_array_mut().unwrap().retain(|p| p["id"] == "pipe:S1");
    model["components"] = json!([connector(
        "pipe:S1",
        "node:A",
        "node:B",
        json!([[1, 0, 0], [0, 1, 0], [0, 0, 1]]),
    )]);
    model["supports"].as_array_mut().unwrap().retain(|s| s["id"] == "support:A");
    for case in model["load_cases"].as_array_mut().unwrap() {
        case["pressure_regions"] = json!([]);
        case["primitive_loads"] = json!([{"id": "load:B-Y", "category": "occasional", "target": {"type": "node", "node": "node:B"},
            "direction": "global_y", "magnitude": {"value": 350.0, "unit": "N"}, "dimension": "force", "provenance": PROVENANCE}]);
    }
    document
}

/// (label, document, mode, mode name).
fn inputs() -> Vec<(&'static str, Value, PreviewSolverMode, &'static str)> {
    vec![
        ("U2-L-ANCH-PTW-K2 0.3.0", i7("U2-L-ANCH-PTW-K2", "0.3.0"), PreviewSolverMode::SparseInteractive, "sparse_interactive"),
        ("U2-L-ANCH-PTW-K2 0.4.0", i7("U2-L-ANCH-PTW-K2", "0.4.0"), PreviewSolverMode::DenseScrutiny, "dense_scrutiny"),
        ("U2-L-KINK-ANCH-P-K2 0.3.0", i7("U2-L-KINK-ANCH-P-K2", "0.3.0"), PreviewSolverMode::DenseScrutiny, "dense_scrutiny"),
        ("L line with connector 0.3.0", l_line_with_connector(), PreviewSolverMode::SparseInteractive, "sparse_interactive"),
        ("replaced span only 0.3.0", replaced_span_only(), PreviewSolverMode::SparseInteractive, "sparse_interactive"),
    ]
}

#[test]
fn arc_envelopes_are_admitted_by_the_pressure_1_reader_in_both_modes() {
    for (label, document, _, _) in inputs() {
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let envelope = run(&document, mode);
            assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{label} {mode:?}: {}", envelope["diagnostics"]);
            semantic_contract::validate_pressure_evidence(&envelope)
                .unwrap_or_else(|e| panic!("{label} {mode:?}: {e}"));
            semantic_contract::for_source(&envelope).unwrap_or_else(|e| panic!("{label} {mode:?}: {e}"));
        }
    }
}

/// Every I7 valued case (both versions, both modes) and the frozen T4-I12
/// connector documents with their variable spring hanger (input review rows
/// are model scoped under pressure-1) are admitted.
#[test]
fn every_i7_case_and_the_hanger_documents_are_admitted() {
    let documents: Value = serde_json::from_str(I7_DOCUMENTS).unwrap();
    let mut admitted = 0;
    for (id, sketch) in documents["sketches"].as_object().unwrap() {
        if id.contains("MITRE") {
            continue;
        }
        for version in ["0.3.0", "0.4.0"] {
            for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
                let envelope = run(&sketch[version]["document"], mode);
                semantic_contract::validate_pressure_evidence(&envelope)
                    .unwrap_or_else(|e| panic!("{id} {version} {mode:?}: {e}"));
                admitted += 1;
            }
        }
    }
    assert_eq!(admitted, 79 * 4);
    let references: Value = serde_json::from_str(include_str!(
        "../../../validation/references/t4_i12/u3_reference_cases.json"
    ))
    .unwrap();
    for (case, key) in [("U3-SYS-DEMO-CONNECTOR-001", "document_v3_0.3.0"), ("U3-SYS-DEMO-CONNECTOR-002-LR1", "document_v3_0.4.0")] {
        let document = &references["cases"][case]["inputs"][key];
        assert!(document.to_string().contains("variable_spring_hanger"), "{case}");
        let envelope = run(document, PreviewSolverMode::SparseInteractive);
        assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{case}");
        semantic_contract::validate_pressure_evidence(&envelope).unwrap_or_else(|e| panic!("{case}: {e}"));
    }
}

#[test]
fn shared_arc_reader_corpus_is_current() {
    let mut cases = Vec::new();
    for (label, document, mode, name) in inputs() {
        let envelope = run(&document, mode);
        assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{label}");
        semantic_contract::validate_pressure_evidence(&envelope).unwrap();
        cases.push(json!({"label": format!("{label} {name}"),
            "document_schema_version": document["model"]["schema_version"], "solver_mode": name,
            "document": document, "envelope": envelope}));
    }
    let corpus = json!({
        "corpus": "T4-U2 pressure-1 realized-arc reader corpus",
        "generator": "core/product_physics/tests/pressure_arc_readers.rs::shared_arc_reader_corpus_is_current",
        "inputs": ["validation/references/t4_i7/u2_document_sketches.json U2-L-ANCH-PTW-K2 (0.3.0, 0.4.0) and U2-L-KINK-ANCH-P-K2 (0.3.0); the L line with a connector tail and a replaced-span-only model, both built by the generator from U2-L-ANCH-PTW-K2 0.3.0"],
        "cases": cases,
    });
    let bytes = serde_json::to_string_pretty(&corpus).unwrap() + "\n";
    if std::env::var_os("T4_U2_WRITE_ARC_CORPUS").is_some() {
        std::fs::write(CORPUS_PATH, &bytes).unwrap();
        return;
    }
    assert!(CORPUS == bytes, "the shared arc reader corpus is stale; regenerate it");
}
