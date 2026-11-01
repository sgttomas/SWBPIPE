//! T4-U3 (S14): the objective connector's publication under `pressure-2`.
//! A solved v3 connector envelope carries one `contract_evidence.connector`
//! record per connector and the connector rows bound to it, discloses that no
//! joint temperature law is provided, and is admitted by the pressure-2 reader;
//! the shared reader corpus (RE, Python) is generated here.
//!
//! The inputs are the frozen T4-I12 system documents (0.3.0 and 0.4.0), bytes
//! unchanged; B7b re-authors each copy to the current 0.9.0 exact document
//! (`common/current_model.rs`). Under pressure-2 the variable spring hanger's input review rows
//! are model scoped (no load case), so the readers admit them (T4-U2).
//!
//! T4-U5b: the corpus also carries two connectors with explicit elastic tie
//! rods, authored in 0.9.0 from the frozen t4_u5b case descriptions by the
//! shared builder (`common/t4_u5b.rs`): AS-F (a listed constant-effective-area
//! joint with one offset rod) and MO-F (an unpressurized connector with two
//! rods, no region).
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use open_pipe_stress_result_export::semantic_contract;
use open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator;
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

/// A frozen system document (bytes unchanged), its copy re-authored to 0.9.0.
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
        // Codex's original single-rod control: Q = I, the connector's nodes at
        // (0,0,0) and (1,0,0), one rod with offsets (1,0,0) and (1e-17,1,0),
        // k = 1 N/m, e_ref = -1 m, unpressurized and unloaded (g_x = Pe = 0).
        // The represented chord's x is fl(1 - 0) + fl(1e-17 - 1) = 0, so the
        // published share is exactly 0 while the share at p is about 1e-17;
        // with no other rod the binding's tolerance is a few subnormals.
        "MO-F single-rod control, Q = I, 1e-17 skew" => {
            let mut request = t4_u5b::request("MO-F", &oracle["cases"]["MO-F"]);
            pretension_only(&mut request);
            let model = &mut request["model"];
            for (id, x) in [("node:A", -2.0), ("node:B", 0.0), ("node:C", 1.0), ("node:D", 3.0)] {
                let node = model["nodes"].as_array_mut().unwrap().iter_mut().find(|n| n["id"] == id).unwrap();
                node["position"] = json!({"x": x, "y": 0.0, "z": 0.0});
            }
            model["components"][0]["objective_connector"]["hardware"]["rods"].as_array_mut().unwrap().truncate(1);
            let rod = rod0(&mut request);
            rod["stiffness"] = json!({"value": 1.0, "unit": "N/m"});
            rod["reference_extension"] = length(-1.0);
            rod["offset_i"] = json!({"x": length(1.0), "y": length(0.0), "z": length(0.0)});
            rod["offset_j"] = json!({"x": length(1.0e-17), "y": length(1.0), "z": length(0.0)});
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
    ("MO-F single-rod control, Q = I, 1e-17 skew", PreviewSolverMode::SparseInteractive, "sparse_interactive"),
    ("AS-F-R rotated perpendicular, pretension only", PreviewSolverMode::DenseScrutiny, "dense_scrutiny"),
    ("AS-F skewed rod with large offsets", PreviewSolverMode::SparseInteractive, "sparse_interactive"),
];

/// The shared connector reader corpus (RE, Python): the 0.3.0
/// system document in sparse mode and the 0.4.0 one in dense mode, each
/// re-authored to 0.9.0, then (T4-U5b) the two tied requests built from
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
        "inputs": ["validation/references/t4_i12/u3_reference_cases.json U3-SYS-DEMO-CONNECTOR-001 document_v3_0.3.0 and U3-SYS-DEMO-CONNECTOR-002-LR1 document_v3_0.4.0, bytes unchanged; each copy re-authored to model document 0.9.0 (exact_pressure_v3, euler_bernoulli, corrosion allowance 0 m) at test time",
            "validation/references/t4_u5b cases AS-F and MO-F (SHA-bound case descriptions over their t4_u5a base requests), authored as model document 0.9.0 requests with explicit_axial_rods_v1 hardware at test time (core/product_physics/tests/common/t4_u5b.rs)",
            "three requests derived at test time from t4_u5b MO-F, AS-F-R and AS-F (connector_readers.rs derived_tied): MO-F re-placed as a single-rod Q = I control, AS-F-R with a perpendicular rod, AS-F with a skewed one, the first two unpressurized and unloaded; invented controls, not references"],
        "cases": cases,
    });
    let bytes = serde_json::to_string_pretty(&corpus).unwrap() + "\n";
    if std::env::var_os("T4_U3_WRITE_CORPUS").is_some() {
        std::fs::write(CORPUS_PATH, &bytes).unwrap();
        return;
    }
    assert!(CORPUS == bytes, "the shared connector reader corpus is stale; regenerate it");
}

/// a·b added exactly (TwoProduct; operands here never underflow).
fn add_product(sum: &mut ExactAccumulator, a: f64, b: f64) {
    let p = a * b;
    sum.add(p).unwrap();
    sum.add(a.mul_add(b, -p)).unwrap();
}

/// An exact sum as a double-double (hi, lo): |sum - hi - lo| <= 2^-106 |sum|.
fn double_double(sum: &ExactAccumulator) -> (f64, f64) {
    let hi = sum.round().unwrap();
    let mut rest = sum.clone();
    rest.add(-hi).unwrap();
    (hi, rest.round().unwrap())
}

/// The independent share at p of rod 0: the exact chord of the authored
/// binary64 inputs, x_j - x_i + Q (o_j - o_i) (each product exact), dotted
/// with the connector x axis Q e_x in double-double and divided by its
/// length. It never sees the represented (rounded) chord the product uses.
fn share_at_p(request: &Value) -> f64 {
    let model = &request["model"];
    let connector = &model["components"][0]["objective_connector"];
    let position = |id: &Value| -> [f64; 3] {
        let node = model["nodes"].as_array().unwrap().iter().find(|n| &n["id"] == id).unwrap();
        ["x", "y", "z"].map(|a| node["position"][a].as_f64().unwrap())
    };
    for end in ["end_i", "end_j"] {
        assert!(["x", "y", "z"].iter().all(|a| connector[end]["offset_local"][a].as_f64() == Some(0.0)), "{end}: a station at its node");
    }
    let (xi, xj) = (position(&connector["end_i"]["node_ref"]), position(&connector["end_j"]["node_ref"]));
    let q: Vec<[f64; 3]> = connector["connector_axes_global"].as_array().unwrap().iter()
        .map(|row| [0, 1, 2].map(|c| row[c].as_f64().unwrap())).collect();
    let rod = &connector["hardware"]["rods"][0];
    let offset = |key: &str| ["x", "y", "z"].map(|a| rod[key][a]["value"].as_f64().unwrap());
    let (oi, oj) = (offset("offset_i"), offset("offset_j"));
    let chord: Vec<(f64, f64)> = (0..3).map(|k| {
        let mut sum = ExactAccumulator::new();
        sum.add(xj[k]).unwrap();
        sum.add(-xi[k]).unwrap();
        for a in 0..3 {
            add_product(&mut sum, q[k][a], oj[a]);
            add_product(&mut sum, -q[k][a], oi[a]);
        }
        double_double(&sum)
    }).collect();
    let mut dot = ExactAccumulator::new();
    for k in 0..3 {
        add_product(&mut dot, chord[k].0, q[k][0]);
        add_product(&mut dot, chord[k].1, q[k][0]);
    }
    let length = (chord[0].0.powi(2) + chord[1].0.powi(2) + chord[2].0.powi(2)).sqrt();
    dot.round().unwrap() / length
}

/// Codex round 2 (MINOR): s_x's common operand discriminates. On Codex's
/// single-rod control and Opus's AS-F-R perpendicular rod, the published
/// output passes the native reader (and the Python reader, through the
/// shared corpus), while the independent share at p differs from the
/// published axial_share, and s_x formed with it, RN(g_x - Pe + t*share_at_p),
/// lies outside the binding's tolerance: the native reader refuses it.
#[test]
fn s_x_formed_with_the_share_at_p_fails_the_binding() {
    for (name, mode, _) in &DERIVED_TIED_INPUTS[..2] {
        let request = derived_tied(name);
        let envelope = run(&request, *mode);
        semantic_contract::validate_pressure_evidence(&envelope).unwrap_or_else(|e| panic!("{name}: {e}"));
        let rows = envelope["results"].as_array().unwrap();
        let pick = |kind: &str, component: Option<&str>| rows.iter()
            .filter(|r| r["kind"] == kind && component.is_none_or(|c| r["metadata"]["component"] == c))
            .map(|r| r["value"].as_f64().unwrap()).collect::<Vec<_>>();
        assert!(pick("connector_pressure_force_v1", Some("pe_x")).is_empty(), "{name}: unpressurized, Pe = 0");
        let [g] = pick("connector_generalized_force_v1", Some("gt_x"))[..] else { panic!("{name}: g_x") };
        let [s] = pick("connector_effective_axial_force_v1", Some("s_x"))[..] else { panic!("{name}: s_x") };
        let [t] = pick("connector_tie_rod_force_v1", None)[..] else { panic!("{name}: one rod") };
        let published = envelope["contract_evidence"]["connector"][0]["tie_rods"][0]["axial_share"].as_f64().unwrap();
        let at_p = share_at_p(&request);
        let mut old = ExactAccumulator::new();
        old.add(g).unwrap();
        add_product(&mut old, t, at_p);
        let old = old.round().unwrap();
        let tolerance = 65.0 * f64::EPSILON * (g.abs() + (t * published).abs()) + 3.0 * f64::from_bits(1);
        eprintln!("{name}: g_x={g:e} t={t:e} s_x={s:e} share={published:e} share_at_p={at_p:e} s_x(at p)={old:e} |shift|={:e} tolerance={tolerance:e}", (old - s).abs());
        assert_ne!(at_p, published, "{name}: the share at p differs from the published share");
        assert!((old - s).abs() > tolerance, "{name}: s_x formed at p moves by more than the binding's tolerance");
        let mut tampered = envelope.clone();
        let row = tampered["results"].as_array_mut().unwrap().iter_mut()
            .find(|r| r["kind"] == "connector_effective_axial_force_v1" && r["metadata"]["component"] == "s_x").unwrap();
        row["value"] = json!(old);
        let refusal = semantic_contract::validate_pressure_evidence(&tampered).expect_err(name);
        assert!(refusal.to_string().contains("EFFECTIVE_ROW_BINDING"), "{name}: {refusal}");
    }
}
