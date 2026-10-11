//! T4-U3 repair (T4-RV19 SF-2): the PP path of an admitted objective
//! connector beyond the frozen diagonal, stress-free, offset-free system case,
//! through PP's public entries in both solver modes:
//! - an installed preload (S13's `+BᵀK q_ref` producer) against T4-I12's
//!   frozen U3-PRELOAD-RELIEF values;
//! - a skew-Q, offset, coupled-H, prestressed connector whose far end is
//!   free, so the preload relaxes completely;
//! - the same connector authored in mm and degrees (the PP decode's units);
//! - one document per classifier row and per FK constructor refusal reached
//!   through PP, each with exactly its code and refs.
//!
//! Expected values: frozen T4-I12 round-02 values where they exist (named at
//! each use); otherwise derived here from the mechanics, never from the
//! product (the derivation is stated at each use).
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, run_linear_static_preview_with_mode,
    PreviewSolverMode,
};
use open_pipe_stress_result_export::semantic_contract;
use serde_json::{json, Value};

const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];
const REFERENCES: &str =
    include_str!("../../../validation/references/t4_i12/u3_reference_cases.json");
const COMPONENT: &str = "component:C-JOINT";
const NODE_A: &str = "node:N-A";
const NODE_B: &str = "node:N-B";
const NODE_C: &str = "node:N-C";
const SPAN: &str = "pipe:P-AB";
const CASE: &str = "load:L-PRESTRESS";

fn references() -> Value {
    serde_json::from_str(REFERENCES).unwrap()
}

fn case(id: &str) -> Value {
    references()["cases"][id].clone()
}

/// A frozen exact value ("p/q" or a decimal) as binary64.
fn rational(text: &Value) -> f64 {
    let text = text.as_str().unwrap();
    match text.split_once('/') {
        Some((p, q)) => p.parse::<f64>().unwrap() / q.parse::<f64>().unwrap(),
        None => text.parse::<f64>().unwrap(),
    }
}

fn rationals(values: &Value) -> Vec<f64> {
    values.as_array().unwrap().iter().map(rational).collect()
}

/// Both entries (captured value and typed DTO) in both modes; the entries
/// agree in each mode.
fn run(document: &Value) -> Vec<Value> {
    let mut outputs = Vec::new();
    for mode in MODES {
        let captured = serde_json::to_value(
            run_linear_static_preview_value_with_mode(document.clone(), mode).unwrap(),
        )
        .unwrap();
        let typed = serde_json::to_value(run_linear_static_preview_with_mode(
            serde_json::from_value(document.clone()).unwrap(),
            mode,
        ))
        .unwrap();
        assert_eq!(captured["status"], typed["status"]);
        assert_eq!(captured["results"], typed["results"]);
        outputs.push(captured);
    }
    outputs
}

/// The frozen 001 system document (0.3.0), the base of every document here.
fn demo_001() -> Value {
    case("U3-SYS-DEMO-CONNECTOR-001")["inputs"]["document_v3_0.3.0"].clone()
}

/// The objective connector of a two-node document: 001's connector (its
/// provenance, calibration, hardware and pressure model) with the given
/// frame, offsets, work matrix and installed reference.
struct Joint<'a> {
    axes: [[f64; 3]; 3],
    offsets: ([f64; 3], [f64; 3]),
    length_unit: &'a str,
    translation_scale: f64,
    upper: Vec<f64>,
    q_ref: ([f64; 3], [f64; 3]),
    angle_unit: &'a str,
    reference_state: &'a str,
}

impl Joint<'_> {
    fn value(&self) -> Value {
        let mut connector = demo_001()["model"]["components"][0]["objective_connector"].clone();
        let vector = |v: [f64; 3], unit: &str| json!({"x": v[0], "y": v[1], "z": v[2], "unit": unit});
        connector["end_i"]["node_ref"] = json!(NODE_A);
        connector["end_j"]["node_ref"] = json!(NODE_B);
        connector["end_i"]["offset_local"] = vector(self.offsets.0, self.length_unit);
        connector["end_j"]["offset_local"] = vector(self.offsets.1, self.length_unit);
        connector["connector_axes_global"] = json!(self.axes);
        connector["stiffness"]["translation_scale"] =
            json!({"value": self.translation_scale, "unit": self.length_unit});
        connector["stiffness"]["upper_triangle"] = json!(self.upper);
        connector["q_ref"] = json!({
            "translation": vector(self.q_ref.0, self.length_unit),
            "rotation": vector(self.q_ref.1, self.angle_unit),
        });
        connector["reference_state"] = json!(self.reference_state);
        connector["topology"]["span_ref"] = json!(SPAN);
        connector
    }
}

/// A two-node connector document (v3, 0.3.0): N-A at `x_a` anchored, N-B at
/// `x_b` restrained in `restraints_b` (no support when empty), the replaced
/// straight span P-AB, the connector, and one case with no primitive load
/// (the preload is the only action). The pressure-1 readers require each case
/// to publish at least one straight member, so a fixed-fixed member P-CA from
/// N-C (1 m along −x from N-A, also anchored) is added: with both ends held
/// and no load it carries nothing and changes no value checked here.
fn two_node(x_a: [f64; 3], x_b: [f64; 3], restraints_b: &[&str], connector: Value) -> Value {
    let mut document = demo_001();
    let model = &mut document["model"];
    let node = |id: &str, x: [f64; 3]| json!({"id": id, "position": {"x": x[0], "y": x[1], "z": x[2]}, "provenance": "invented T4-I27 two-node connector control"});
    model["nodes"] = json!([node(NODE_A, x_a), node(NODE_B, x_b), node(NODE_C, [x_a[0] - 1.0, x_a[1], x_a[2]])]);
    let pipe = |id: &str, from: &str, to: &str| {
        let mut pipe = model["pipe_segments"][3].clone();
        pipe["id"] = json!(id);
        pipe["from"] = json!(from);
        pipe["to"] = json!(to);
        pipe["y_reference"] = json!({"x": 0.0, "y": 0.0, "z": 1.0});
        pipe
    };
    let pipes = json!([pipe("pipe:P-CA", NODE_C, NODE_A), pipe(SPAN, NODE_A, NODE_B)]);
    model["pipe_segments"] = pipes;
    let anchor = |id: &str, node: &str| json!({"id": id, "node": node, "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": "invented T4-I27 two-node connector control"});
    let mut supports = vec![anchor("support:S-A", NODE_A), anchor("support:S-C", NODE_C)];
    if !restraints_b.is_empty() {
        supports.push(json!({"id": "support:S-B", "node": NODE_B, "family": "guide", "restraints": restraints_b, "provenance": "invented T4-I27 two-node connector control"}));
    }
    model["supports"] = json!(supports);
    let mut component = model["components"][0].clone();
    component["id"] = json!(COMPONENT);
    component["node"] = json!(NODE_B);
    component["objective_connector"] = connector;
    model["components"] = json!([component]);
    model["load_cases"] = json!([{"id": CASE, "primitive_loads": [], "pressure_regions": [], "provenance": "invented T4-I27 two-node connector control"}]);
    document
}

fn find<'e>(envelope: &'e Value, kind: &str, entity: &str, location: Option<&str>, component: &str) -> f64 {
    let found = envelope["results"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| {
            r["basis_ref"]["ref_id"] == CASE
                && r["kind"] == kind
                && r["entity_ref"] == entity
                && r["metadata"]["component"] == component
                && location.map_or(true, |l| r["metadata"]["location"] == l)
        })
        .collect::<Vec<_>>();
    assert_eq!(found.len(), 1, "{kind} {entity} {location:?} {component}");
    let value = found[0]["value"].as_f64().unwrap();
    match found[0]["unit"].as_str().unwrap() {
        "mm" => value / 1000.0,
        _ => value,
    }
}

/// Every published motion, action and reaction this file checks, in a fixed
/// order: N-B's displacement and rotation (6), the connector's q − q_ref and
/// g (12), its end actions at end_i and end_j (12), and each support's
/// reaction components (6 per support).
fn published(envelope: &Value, supports: &[&str]) -> Vec<f64> {
    let xyz = ["x", "y", "z"];
    let mut out = Vec::new();
    for kind in ["displacement", "rotation"] {
        for axis in xyz {
            out.push(find(envelope, &format!("global_nodal_{kind}_{axis}"), NODE_B, None, &format!("nodal_{kind}_{axis}")));
        }
    }
    for (kind, prefix) in [("translation", "qt"), ("rotation", "qr"), ("force", "gt"), ("moment", "gr")] {
        for axis in xyz {
            out.push(find(envelope, &format!("connector_generalized_{kind}_v1"), COMPONENT, Some("connector_local"), &format!("{prefix}_{axis}")));
        }
    }
    for end in ["end_i", "end_j"] {
        for (kind, prefix) in [("force", "F"), ("moment", "M")] {
            for axis in xyz {
                out.push(find(envelope, &format!("connector_endpoint_{kind}_v1"), COMPONENT, Some(end), &format!("{prefix}{axis}")));
            }
        }
    }
    for &support in supports {
        for component in ["Fx", "Fy", "Fz", "Mx", "My", "Mz"] {
            out.push(find(envelope, "support_reaction_component_v2", support, None, component));
        }
    }
    out
}

/// `published`'s families: (start, length, floor) for N-B's translations and
/// rotations, q − q_ref translations and rotations, g forces and moments, end
/// forces and moments, and reaction forces and moments.
fn families(floors: [f64; 4]) -> Vec<(usize, usize, f64)> {
    let [translation, rotation, force, moment] = floors;
    vec![
        (0, 3, translation), (3, 3, rotation),
        (6, 3, translation), (9, 3, rotation), (12, 3, force), (15, 3, moment),
        (18, 3, force), (21, 3, moment), (24, 3, force), (27, 3, moment),
        (30, 3, force), (33, 3, moment), (36, 3, force), (39, 3, moment),
    ]
}

/// |obs − exp| ≤ 1e-9·max(|exp|, F) per entry, F its family's floor.
fn assert_close(observed: &[f64], expected: &[f64], floors: [f64; 4], label: &str) {
    assert_eq!(observed.len(), expected.len());
    for (start, length, floor) in families(floors).into_iter().filter(|f| f.0 < observed.len()) {
        for k in start..start + length {
            assert!(
                (observed[k] - expected[k]).abs() <= 1e-9 * expected[k].abs().max(floor),
                "{label} [{k}]: {} vs {} (floor {floor})",
                observed[k],
                expected[k]
            );
        }
    }
}

fn solved(envelope: &Value, label: &str) {
    assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{label}: {}", envelope["diagnostics"]);
    semantic_contract::validate_pressure_evidence(envelope).unwrap_or_else(|e| panic!("{label}: {e}"));
}

/// S13 against T4-I12's frozen U3-PRELOAD-RELIEF, sub-case
/// `node_i_anchored_node_j_ux_held` (Q = I, L = 2 m, Ls = 2 m, coupled
/// tx–rx K, q_ref = (1/5, 0, 0, 1/10, 0, 0)): N-B's displacement is the
/// frozen `d`, the connector's q − q_ref is the frozen `q` minus `q_ref`, g is
/// the frozen `g_recovered`, both supports' reactions are the frozen
/// `support_on_element_reactions`, and each end action (node on element)
/// equals its node's reaction (nodal equilibrium with no external load).
/// A negated preload (T4-RV19's M1) gives the opposite rotation.
#[test]
fn installed_preload_through_pp_matches_the_frozen_relief_values() {
    let c = case("U3-PRELOAD-RELIEF");
    let inputs = &c["inputs"];
    let q_ref = rationals(&inputs["q_ref"]);
    let joint = Joint {
        axes: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        offsets: ([0.0; 3], [0.0; 3]),
        length_unit: "m",
        translation_scale: rational(&inputs["translation_scale_Ls_m"]),
        upper: rationals(&inputs["H_upper_triangle_21_N_m"]),
        q_ref: ([q_ref[0], q_ref[1], q_ref[2]], [q_ref[3], q_ref[4], q_ref[5]]),
        angle_unit: "rad",
        reference_state: "prestressed",
    };
    let document = two_node([0.0; 3], [2.0, 0.0, 0.0], &["UX"], joint.value());
    let expected_case = &c["expected"]["node_i_anchored_node_j_ux_held"];
    let d = rationals(&expected_case["d"]);
    let q = rationals(&expected_case["q"]);
    let g = rationals(&expected_case["g_recovered"]);
    let reactions = rationals(&expected_case["support_on_element_reactions"]);
    let mut expected = d[6..12].to_vec();
    expected.extend((0..6).map(|k| q[k] - q_ref[k]));
    expected.extend(&g);
    expected.extend(&reactions);
    expected.extend(&reactions);
    for (index, envelope) in run(&document).iter().enumerate() {
        solved(envelope, "preload relief");
        let observed = published(envelope, &["support:S-A", "support:S-B"]);
        assert_close(&observed, &expected, [0.2, 0.1, 1.0, 1.0], &format!("preload relief mode {index}"));
    }
}

/// The generic connector's inputs (frozen U3-GENERIC-SKEW-OFFSET-PRESTRESS:
/// skew rational Q, both offsets, fully coupled H, Ls = 1/4 m, prestressed
/// q_ref), authored in metres and radians, or in millimetres and degrees.
fn generic(millimetres: bool) -> (Value, Value) {
    let inputs = case("U3-GENERIC-SKEW-OFFSET-PRESTRESS")["inputs"].clone();
    let v3 = |key: &str| -> [f64; 3] { rationals(&inputs[key]).try_into().unwrap() };
    let q_ref = rationals(&inputs["q_ref"]);
    let axes = inputs["Q_row_major_columns_are_axes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| rationals(row).try_into().unwrap())
        .collect::<Vec<[f64; 3]>>();
    let length = |v: [f64; 3]| if millimetres { v.map(|x| x * 1000.0) } else { v };
    let joint = Joint {
        axes: axes.try_into().unwrap(),
        offsets: (length(v3("a_i_global")), length(v3("a_j_global"))),
        length_unit: if millimetres { "mm" } else { "m" },
        translation_scale: rational(&inputs["translation_scale_Ls_m"]) * if millimetres { 1000.0 } else { 1.0 },
        upper: rationals(&inputs["H_upper_triangle_21_N_m"]),
        q_ref: (
            length([q_ref[0], q_ref[1], q_ref[2]]),
            if millimetres {
                [q_ref[3], q_ref[4], q_ref[5]].map(f64::to_degrees)
            } else {
                [q_ref[3], q_ref[4], q_ref[5]]
            },
        ),
        angle_unit: if millimetres { "deg" } else { "rad" },
        reference_state: "prestressed",
    };
    (two_node(v3("x_i"), v3("x_j"), &[], joint.value()), inputs)
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

/// A skew-Q, offset, coupled, prestressed connector with N-B free: the
/// preload relaxes completely. Derived from the mechanics (JR §2's B), not
/// the product: N-B carries only the connector and no load, so g = 0 and
/// q = q_ref (q − q_ref = 0), every end action and reaction is zero, and with
/// N-A fixed, B·d = q_ref gives θ_B = Q·q_r and u_B = Q·q_t − (r/2 − a_j) × θ_B,
/// r = (x_B − x_A) + (a_j − a_i) (Q's columns are the axes; the offsets are
/// global, the node triads being I). A negated preload gives q − q_ref = −2·q_ref.
#[test]
fn a_skew_offset_coupled_preload_relaxes_at_a_free_end() {
    let (document, inputs) = generic(false);
    let v3 = |key: &str| -> [f64; 3] { rationals(&inputs[key]).try_into().unwrap() };
    let q_ref = rationals(&inputs["q_ref"]);
    let axes: Vec<Vec<f64>> = inputs["Q_row_major_columns_are_axes"].as_array().unwrap().iter().map(rationals).collect();
    let apply_q = |v: [f64; 3]| -> [f64; 3] { [0, 1, 2].map(|row| (0..3).map(|c| axes[row][c] * v[c]).sum()) };
    let (x_a, x_b, a_i, a_j) = (v3("x_i"), v3("x_j"), v3("a_i_global"), v3("a_j_global"));
    let r: [f64; 3] = [0, 1, 2].map(|k| (x_b[k] - x_a[k]) + (a_j[k] - a_i[k]));
    let theta = apply_q([q_ref[3], q_ref[4], q_ref[5]]);
    let arm = cross([0, 1, 2].map(|k| 0.5 * r[k] - a_j[k]), theta);
    let u = apply_q([q_ref[0], q_ref[1], q_ref[2]]);
    let mut expected = vec![u[0] - arm[0], u[1] - arm[1], u[2] - arm[2], theta[0], theta[1], theta[2]];
    expected.extend([0.0; 30]);
    // Floors: |q_ref| (motions) and Σ|K||q_ref| (actions, about 250 N and 2.5 N·m).
    let floors = [3e-3, 2e-3, 250.0, 2.5];
    let mut reference = Vec::new();
    for (index, envelope) in run(&document).iter().enumerate() {
        solved(envelope, "free end");
        let observed = published(envelope, &["support:S-A"]);
        assert_close(&observed, &expected, floors, &format!("free end mode {index}"));
        reference.push(observed);
    }
    // The same connector authored in mm (offsets, Ls, q_ref) and degrees
    // (q_ref rotation): the PP decode converts to SI, so every value agrees.
    let (millimetres, _) = generic(true);
    for (index, envelope) in run(&millimetres).iter().enumerate() {
        solved(envelope, "mm/deg");
        assert_close(&published(envelope, &["support:S-A"]), &reference[index], floors, &format!("mm/deg mode {index}"));
    }
}

// ------------------------------------------------------------ refusals

fn blocking(envelope: &Value) -> Vec<&Value> {
    envelope["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|d| d["severity"] == "blocking")
        .collect()
}

/// Refused with exactly `code` (every blocking diagnostic), the refs
/// exactly `refs`, and the message containing `fragment`.
fn assert_refused(document: &Value, code: &str, refs: &[&str], fragment: &str, label: &str) {
    for envelope in run(document) {
        assert_eq!(envelope["status"]["mechanics"], "MODEL_INCOMPLETE", "{label}");
        assert_eq!(envelope["results"], json!([]), "{label}");
        let found = blocking(&envelope);
        assert_eq!(found.len(), 1, "{label}: {found:#?}");
        assert_eq!(found[0]["code"], code, "{label}: {found:#?}");
        assert_eq!(found[0]["affected_refs"], json!(refs), "{label}");
        assert!(found[0]["message"].as_str().unwrap().contains(fragment), "{label}: {}", found[0]["message"]);
    }
}

/// 001's document with its connector value changed by `edit`.
fn demo_with(edit: impl FnOnce(&mut Value)) -> Value {
    let mut document = demo_001();
    edit(&mut document["model"]["components"][0]["objective_connector"]);
    document
}

/// Each classifier row (slot table §4.2) on its own, through PP, with
/// exactly its code and refs [component] (T4-RV19's M2 admitted every
/// hardware kind).
#[test]
fn each_classifier_row_refuses_with_exactly_its_code() {
    const C: &str = "component:C-150";
    let rows: Vec<(&str, Value, &str, &str)> = vec![
        ("basis absent", demo_with(|c| { c.as_object_mut().unwrap().remove("calibration"); }),
            "JOINT_STIFFNESS_BASIS_UNSUPPORTED", "no stiffness calibration basis"),
        ("basis with a pressure-dependent tangent", demo_with(|c| c["calibration"]["includes_pressure_dependent_tangent"] = json!(true)),
            "JOINT_STIFFNESS_BASIS_UNSUPPORTED", "only a constant structural elasticity calibration"),
        ("hardware absent", demo_with(|c| { c.as_object_mut().unwrap().remove("hardware"); }),
            "JOINT_HARDWARE_NOT_DEFINED", "never taken as untied"),
        ("hardware tied", demo_with(|c| c["hardware"]["kind"] = json!("tied")),
            "JOINT_HARDWARE_LAW_UNSUPPORTED", "hardware tied is not supported"),
        ("hardware without a kind", demo_with(|c| c["hardware"] = json!({})),
            "JOINT_HARDWARE_LAW_UNSUPPORTED", "hardware (no kind) is not supported"),
        ("pressure model", demo_with(|c| c["pressure_model"]["kind"] = json!("pressurized")),
            "JOINT_PRESSURE_INTERFACE_UNRESOLVED", "only the unpressurized pressure model"),
        ("both ends one node", demo_with(|c| c["end_j"]["node_ref"] = json!("node:N-130")),
            "OBJECTIVE_CONNECTOR_TOPOLOGY_UNRESOLVED", "both ends name the same node"),
        ("an unknown end node", demo_with(|c| c["end_j"]["node_ref"] = json!("node:N-999")),
            "OBJECTIVE_CONNECTOR_TOPOLOGY_UNRESOLVED", "an end node is not in the model"),
        ("a span that is no pipe", demo_with(|c| c["topology"]["span_ref"] = json!("pipe:P-999")),
            "OBJECTIVE_CONNECTOR_TOPOLOGY_UNRESOLVED", "which is not a pipe of the model"),
        ("a pipe not joining the ends", demo_with(|c| c["topology"]["span_ref"] = json!("pipe:P-120")),
            "OBJECTIVE_CONNECTOR_TOPOLOGY_UNRESOLVED", "does not join the connector's end nodes"),
    ];
    for (label, document, code, fragment) in rows {
        assert_refused(&document, code, &[C], fragment, label);
    }
    // A span already replaced: a second connector on P-130 is refused alone.
    let mut document = demo_001();
    let mut second = document["model"]["components"][0].clone();
    second["id"] = json!("component:C-151");
    document["model"]["components"].as_array_mut().unwrap().push(second);
    assert_refused(&document, "OBJECTIVE_CONNECTOR_TOPOLOGY_UNRESOLVED", &["component:C-151"], "already replaced by component:C-150", "span already replaced");
    // Field decode (refs [component, objective_connector]): an installed
    // temperature at or below absolute zero, as the readers require.
    for (value, unit) in [(-300.0, "degC"), (0.0, "K")] {
        let document = demo_with(|c| c["installed_reference_temperature"] = json!({"value": value, "unit": unit}));
        assert_refused(&document, "OBJECTIVE_CONNECTOR_INPUT_INCOMPLETE", &[C, "objective_connector"], "above absolute zero", &format!("temperature {value} {unit}"));
    }
}

/// The FK constructor's refusals and the stress-free condition reach PP as
/// `OBJECTIVE_CONNECTOR_INPUT_INCOMPLETE`, refs [component,
/// objective_connector], naming the reason; never solved.
#[test]
fn fk_constructor_refusals_reach_pp_by_code() {
    const C: &str = "component:C-150";
    // An indefinite authored H (a negative tx coefficient).
    let indefinite = demo_with(|c| c["stiffness"]["upper_triangle"][0] = json!(-800000.0));
    // Q.x not along r (001's r is +z; this Q's x-axis is +x).
    let misaligned = demo_with(|c| c["connector_axes_global"] = json!([[1, 0, 0], [0, 1, 0], [0, 0, 1]]));
    // stress_free with K q_ref ≠ 0.
    let preloaded = demo_with(|c| c["q_ref"]["translation"]["x"] = json!(0.001));
    // A singular H whose binary64 decode at the non-dyadic Ls = 0.3 m is
    // indefinite (T4-I27's decode_inertia.py; FK's
    // definiteness_reads_h_and_its_binary64_decode_at_a_non_dyadic_ls).
    let rounded = demo_with(|c| {
        let mut upper = [0.0; 21];
        for diagonal in [6, 11, 18, 20] {
            upper[diagonal] = 1.0;
        }
        (upper[0], upper[3], upper[15]) = (1.0, 5.0, 25.0);
        c["stiffness"]["upper_triangle"] = json!(upper);
        c["stiffness"]["translation_scale"]["value"] = json!(0.3);
    });
    for (label, document, fragment) in [
        ("indefinite H", indefinite, "not positive semidefinite"),
        ("misaligned Q", misaligned, "Q.x is not along +r"),
        ("stress_free preloaded", preloaded, "stress_free requires K q_ref = 0 exactly"),
        ("decode indefinite", rounded, "non-dyadic Ls"),
    ] {
        assert_refused(&document, "OBJECTIVE_CONNECTOR_INPUT_INCOMPLETE", &[C, "objective_connector"], fragment, label);
    }
}

/// A replaced span that is not built (its own pipe diagnostic blocks) is
/// also refused by the connector, never silently passed over (T4-RV20 N-6).
#[test]
fn an_unbuilt_replaced_span_is_refused_by_the_connector_too() {
    let mut document = demo_001();
    document["model"]["pipe_segments"][3]["section"]["wall_thickness"] = json!({"value": 0.2, "unit": "m"});
    for envelope in run(&document) {
        assert_eq!(envelope["status"]["mechanics"], "MODEL_INCOMPLETE");
        let found = blocking(&envelope);
        let connector = found
            .iter()
            .find(|d| d["code"] == "OBJECTIVE_CONNECTOR_INPUT_INCOMPLETE")
            .unwrap_or_else(|| panic!("{found:#?}"));
        assert!(connector["message"].as_str().unwrap().contains("replaced span pipe:P-130 is not built"));
        assert!(found.iter().any(|d| d["code"] != "OBJECTIVE_CONNECTOR_INPUT_INCOMPLETE"
            && d["affected_refs"].as_array().unwrap().contains(&json!("pipe:P-130"))), "{found:#?}");
    }
}

/// A realized curved span cannot be replaced: the connector is refused by
/// topology (the bend itself takes its own v3 family refusal).
#[test]
fn a_realized_curved_span_is_not_replaceable() {
    let mut document = demo_001();
    document["model"]["components"].as_array_mut().unwrap().push(json!({
        "id": "component:B-130", "kind": "bend", "node": "node:N-140",
        "geometry": {"bend_pipe_ref": "pipe:P-130", "bend_radius": {"value": 1.0, "unit": "m"},
            "bend_plane_orientation": "invented", "bend_geometry_source_reference": "invented"},
        "modifiers": {"flexibility_factor_user_value": {"value": 1.0, "unit": "none"}, "source_reference": "invented"},
        "mechanics_interface": {"solver_consumption": "curved_bend_macro_element"},
        "provenance": "invented T4-I27 curved-span control"}));
    for envelope in run(&document) {
        assert_eq!(envelope["status"]["mechanics"], "MODEL_INCOMPLETE");
        let found = blocking(&envelope);
        assert!(found.iter().any(|d| d["code"] == "OBJECTIVE_CONNECTOR_TOPOLOGY_UNRESOLVED"
            && d["affected_refs"] == json!(["component:C-150"])
            && d["message"].as_str().unwrap().contains("pipe:P-130 is a realized curved span")), "{found:#?}");
        assert!(found.iter().all(|d| d["code"] == "OBJECTIVE_CONNECTOR_TOPOLOGY_UNRESOLVED"
            || d["affected_refs"].as_array().unwrap().contains(&json!("component:B-130"))), "{found:#?}");
    }
}
