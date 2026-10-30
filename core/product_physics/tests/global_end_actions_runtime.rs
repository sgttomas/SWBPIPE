//! W-7 (PRD FR-SOL-007; DEL-04-01 Design `global-end-actions.md`): member end
//! actions in global coordinates, through the desktop value entry
//! `run_linear_static_preview_value_with_mode`, in both solver modes.
//!
//! Every expectation here is independent of the product's transform: the local
//! axes are recomputed from node coordinates and `y_reference` (straight
//! members) or from the arc geometry (bend end tangents); the balances use only
//! published rows and the authored loads. Sign conventions (from the product's
//! published texts): a member end row is the action on the element at its node
//! (node on element), so the element acts on its node with -F; a support row is
//! the support's action on the pipe. Nodal balance at node n is therefore
//! -sum_incident F + P_external + R_support = 0, and the same for moments
//! (end actions act at the node: no lever arm). Criterion: 1e-9 of the node's
//! force (moment) scale. Model inputs are invented.
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, MechanicsEnvelope, PreviewSolverMode, ResultItem};
use open_pipe_stress_result_export::reader_verdict;
use serde_json::{json, Value};
use std::f64::consts::PI;

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const FORCE: &str = "element_global_end_force_v1";
const MOMENT: &str = "element_global_end_moment_v1";
const BASIS: &str = "rotated_from_published_element_local_end_actions_v1";
const WALL_BASIS: &str = "rotated_from_published_element_local_end_actions_v1;axial_is_pipe_wall_action";
const SIGN: &str = "global end action on the element at its node (node on element), F = R\u{1d40} f; the element acts on its node with \u{2212}F";
const LOCAL: [&str; 6] = [
    "element_local_axial_force",
    "element_local_shear_force_y",
    "element_local_shear_force_z",
    "element_local_torsional_moment",
    "element_local_bending_moment_y",
    "element_local_bending_moment_z",
];
const PROV: &str = "w7 invented test input";

// ------------------------------------------------------------------ vectors
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn unit(a: [f64; 3]) -> [f64; 3] {
    let n = dot(a, a).sqrt();
    [a[0] / n, a[1] / n, a[2] / n]
}
fn norm(a: [f64; 3]) -> f64 {
    dot(a, a).sqrt()
}
/// Independent straight-member axes (rows: local x, y, z in global).
fn straight_axes(from: [f64; 3], to: [f64; 3], y_reference: [f64; 3]) -> [[f64; 3]; 3] {
    let x = unit([to[0] - from[0], to[1] - from[1], to[2] - from[2]]);
    let p = dot(y_reference, x);
    let y = unit([y_reference[0] - p * x[0], y_reference[1] - p * x[1], y_reference[2] - p * x[2]]);
    [x, y, cross(x, y)]
}
fn rotate_t(r: [[f64; 3]; 3], f: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|k| r[0][k] * f[0] + r[1][k] * f[1] + r[2][k] * f[2])
}

// ------------------------------------------------------------------ envelope access
fn solve(wire: &Value, mode: PreviewSolverMode, label: &str) -> MechanicsEnvelope {
    let out = run_linear_static_preview_value_with_mode(wire.clone(), mode).unwrap_or_else(|e| panic!("{label}: {e}"));
    let blocking: Vec<_> = out.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| format!("{}: {}", d.code, d.message)).collect();
    assert_eq!(out.status.mechanics, "MECHANICS_SOLVED", "{label}: {blocking:?}");
    assert!(blocking.is_empty(), "{label}: {blocking:?}");
    out
}
fn in_case(r: &ResultItem, case: &str) -> bool {
    r.basis_ref.as_ref().is_some_and(|b| b.ref_type == "load_case" && b.ref_id == case)
}
fn one<'a>(out: &'a MechanicsEnvelope, case: &str, kind: &str, entity: &str, component: Option<&str>, location: Option<&str>) -> Option<&'a ResultItem> {
    let found: Vec<_> = out
        .results
        .iter()
        .filter(|r| {
            in_case(r, case)
                && r.kind == kind
                && r.entity_ref == entity
                && r.metadata.as_ref().is_some_and(|m| component.is_none_or(|c| m.component == c) && location.is_none_or(|l| m.location == l))
        })
        .collect();
    assert!(found.len() <= 1, "{case} {kind} {entity} {component:?} {location:?}: {} rows", found.len());
    found.first().copied()
}
/// The published local end vector (f, m) of a member end; the axial is the
/// wall action where the member publishes one (EX pressure members).
fn local(out: &MechanicsEnvelope, case: &str, pipe: &str, end: &str) -> ([f64; 3], [f64; 3], bool) {
    let wall = one(out, case, "pipe_wall_endpoint_action_v2", pipe, None, Some(end)).map(|r| r.value);
    let v: [f64; 6] = std::array::from_fn(|k| match (k, wall) {
        (0, Some(w)) => {
            assert!(one(out, case, LOCAL[0], pipe, None, Some(end)).is_none(), "the wall action replaces the local axial");
            w
        }
        _ => one(out, case, LOCAL[k], pipe, None, Some(end)).unwrap_or_else(|| panic!("{case} {pipe} {end} {}", LOCAL[k])).value,
    });
    ([v[0], v[1], v[2]], [v[3], v[4], v[5]], wall.is_some())
}
/// The published global end rows (F, M) of a member end, with their declared semantics checked.
fn global(out: &MechanicsEnvelope, case: &str, pipe: &str, end: &str, wall: bool) -> ([f64; 3], [f64; 3]) {
    let mut v = [0.0; 6];
    for (k, (kind, name, unit)) in [(FORCE, "Fx", "N"), (FORCE, "Fy", "N"), (FORCE, "Fz", "N"), (MOMENT, "Mx", "N*m"), (MOMENT, "My", "N*m"), (MOMENT, "Mz", "N*m")].into_iter().enumerate() {
        let r = one(out, case, kind, pipe, Some(name), Some(end)).unwrap_or_else(|| panic!("{case} {pipe} {end} {name} missing"));
        let m = r.metadata.as_ref().unwrap();
        assert_eq!((r.unit.as_str(), m.coordinate_system.as_str(), m.basis.as_str(), m.sign_convention.as_str()), (unit, "global", if wall { WALL_BASIS } else { BASIS }, SIGN), "{}", r.id);
        assert!(r.source_result_refs.is_empty(), "{}", r.id);
        assert!(r.id.contains("element-global-end:") && r.id.ends_with(&format!(":{end}:{}", name.to_ascii_lowercase())), "{}", r.id);
        v[k] = r.value;
    }
    ([v[0], v[1], v[2]], [v[3], v[4], v[5]])
}
fn support(out: &MechanicsEnvelope, case: &str, id: &str) -> ([f64; 3], [f64; 3]) {
    let v: [f64; 6] = std::array::from_fn(|k| one(out, case, "support_reaction_component_v2", id, Some(["Fx", "Fy", "Fz", "Mx", "My", "Mz"][k]), None).unwrap().value);
    ([v[0], v[1], v[2]], [v[3], v[4], v[5]])
}
fn global_rows(out: &MechanicsEnvelope) -> usize {
    out.results.iter().filter(|r| r.kind == FORCE || r.kind == MOMENT).count()
}
/// |a| <= 1e-9 scale per component.
fn balanced(residual: [f64; 3], scale: f64, what: &str) {
    for (k, r) in residual.iter().enumerate() {
        assert!(r.abs() <= 1e-9 * scale, "{what}[{k}]: residual {r:.3e}, scale {scale:.3e}");
    }
}
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
/// The reader round trip: the product reader of the envelope's route accepts it.
fn read_back(out: &MechanicsEnvelope, reader: &str, wire: &Value, label: &str) -> Value {
    let raw = serde_json::to_value(out).unwrap();
    assert_eq!(reader_verdict::validate(reader, &raw, Some(&wire["model"]), None), json!({"ok": true}), "{label}: {reader} reader");
    raw
}

// ------------------------------------------------------------------ model authoring
fn qv(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}
fn node(id: &str, p: [f64; 3]) -> Value {
    json!({"id": id, "position": {"x": p[0], "y": p[1], "z": p[2]}, "provenance": PROV})
}
fn pipe(id: &str, from: &str, to: &str, yref: [f64; 3], exact: bool) -> Value {
    let mut section = json!({"outside_diameter": qv(0.12, "m"), "wall_thickness": qv(0.01, "m")});
    if exact {
        section["corrosion_allowance"] = qv(0.0, "m");
    }
    json!({"id": id, "from": from, "to": to, "section": section, "material": "material:m",
        "y_reference": {"x": yref[0], "y": yref[1], "z": yref[2]}, "provenance": PROV})
}
fn anchor(id: &str, node: &str) -> Value {
    json!({"id": id, "node": node, "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV})
}
fn nodal(id: &str, node: &str, f: [f64; 3], m: [f64; 3]) -> Vec<Value> {
    let mut out = Vec::new();
    for k in 0..3 {
        out.push(json!({"id": format!("{id}:f{k}"), "category": "concentrated_force", "target": {"type": "node", "node": node},
            "direction": (["global_x", "global_y", "global_z"][k]), "magnitude": qv(f[k], "N"), "dimension": "force", "provenance": PROV}));
        out.push(json!({"id": format!("{id}:m{k}"), "category": "concentrated_moment", "target": {"type": "node", "node": node},
            "direction": (["rotation_x", "rotation_y", "rotation_z"][k]), "magnitude": qv(m[k], "N*m"), "dimension": "moment", "provenance": PROV}));
    }
    out
}
fn thermal(id: &str, pipe: &str, dt: f64) -> Value {
    json!({"id": format!("thermal:{id}:{pipe}"), "category": "thermal", "target": {"type": "element", "pipe": pipe}, "direction": "global_x",
        "magnitude": qv(dt, "degC"), "dimension": "temperature_interval", "provenance": PROV})
}
fn line(id: &str, pipe: &str, dir: &str, w: f64) -> Value {
    json!({"id": id, "category": "distributed_force", "target": {"type": "element", "pipe": pipe}, "direction": dir,
        "magnitude": qv(w, "N/m"), "dimension": "force_per_length", "provenance": PROV})
}
const UNITS: &str = r#"{"length":"m","force":"N","angle":"rad","pressure":"Pa","stress":"Pa","temperature":"degC"}"#;
fn status() -> Value {
    json!({"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"})
}
/// A preview-route (PF) document.
fn preview(nodes: Vec<Value>, pipes: Vec<Value>, supports: Vec<Value>, components: Vec<Value>, cases: Vec<Value>, combinations: Vec<Value>) -> Value {
    json!({"model": {"schema_version": "0.8.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:w7", "units": serde_json::from_str::<Value>(UNITS).unwrap()}, "analysis_status": status(),
        "nodes": nodes, "pipe_segments": pipes, "supports": supports, "components": components,
        "materials": [{"id": "material:m", "elastic_modulus": qv(200e9, "Pa"), "shear_modulus": qv(76.923076923e9, "Pa"),
            "thermal_expansion_coefficient": qv(1.2e-5, "1/degC"), "provenance": PROV}],
        "load_cases": cases, "combinations": combinations}, "materials": []})
}
/// An exact-route (EX, `3.0.0/exact_pressure_v3`, Euler-Bernoulli) document.
fn exact(nodes: Vec<Value>, pipes: Vec<Value>, supports: Vec<Value>, components: Vec<Value>, cases: Vec<Value>) -> Value {
    json!({"model": {"schema_version": "0.8.0", "document_kind": "openpipestress.product_preview.model",
        "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"},
        "project": {"id": "project:w7", "units": serde_json::from_str::<Value>(UNITS).unwrap()}, "analysis_status": status(),
        "nodes": nodes, "pipe_segments": pipes, "supports": supports, "components": components,
        "materials": [{"id": "material:m", "constitutive_basis": "homogeneous_isotropic_E_nu_v1", "elastic_modulus": qv(200e9, "Pa"),
            "poisson_ratio": qv(0.3, "1"), "thermal_expansion_coefficient": qv(1.2e-5, "1/degC"), "temperature_points": [], "provenance": PROV}],
        "load_cases": cases, "combinations": []}, "materials": []})
}

// ------------------------------------------------------------------ 1. 3-D frame
type Member = (&'static str, &'static str, &'static str, [f64; 3]);
const FRAME_NODES: [(&str, [f64; 3]); 5] =
    [("node:n0", [0.0, 0.0, 0.0]), ("node:n1", [2.0, 0.0, 0.0]), ("node:n2", [2.0, 3.0, 0.0]), ("node:n3", [2.0, 3.0, 4.0]), ("node:n4", [4.0, 5.0, 7.0])];
/// Members along x, y and z and one inclined member; anchors at n0 and n4.
const FRAME_MEMBERS: [Member; 4] = [
    ("pipe:n0-n1", "node:n0", "node:n1", [0.0, 1.0, 0.0]),
    ("pipe:n1-n2", "node:n1", "node:n2", [0.0, 0.0, 1.0]),
    ("pipe:n2-n3", "node:n2", "node:n3", [1.0, 0.0, 0.0]),
    ("pipe:n3-n4", "node:n3", "node:n4", [0.0, 0.0, 1.0]),
];
const FRAME_SUPPORTS: [(&str, &str); 2] = [("support:n0", "node:n0"), ("support:n4", "node:n4")];
/// Nodal loads (force, moment) at the free nodes, per case.
fn frame_loads(case: usize) -> [(&'static str, [f64; 3], [f64; 3]); 3] {
    let s = if case == 0 { 1.0 } else { -0.6 };
    [
        ("node:n1", [1200.0 * s, -800.0, 450.0], [300.0, -250.0 * s, 175.0]),
        ("node:n2", [-650.0, 900.0 * s, -1300.0], [-120.0 * s, 410.0, -95.0]),
        ("node:n3", [700.0, 350.0, 1000.0 * s], [220.0, 60.0 * s, -330.0]),
    ]
}
fn position(id: &str) -> [f64; 3] {
    FRAME_NODES.iter().find(|n| n.0 == id).unwrap().1
}
fn frame_case(case: usize, with_member_loads: bool) -> Value {
    let id = ["case:one", "case:two"][case];
    let mut loads: Vec<Value> = frame_loads(case).iter().flat_map(|(n, f, m)| nodal(&format!("load:{id}:{n}"), n, *f, *m)).collect();
    if with_member_loads {
        // Member loads act inside the element: they enter its end actions, not a nodal balance.
        loads.push(thermal(id, "pipe:n1-n2", if case == 0 { 80.0 } else { -35.0 }));
        loads.push(line(&format!("line:{id}"), "pipe:n3-n4", "global_y", if case == 0 { -400.0 } else { 250.0 }));
    }
    json!({"id": id, "primitive_loads": loads, "provenance": PROV})
}
fn frame(combinations: Vec<Value>) -> Value {
    preview(
        FRAME_NODES.iter().map(|(id, p)| node(id, *p)).collect(),
        FRAME_MEMBERS.iter().map(|(id, a, b, y)| pipe(id, a, b, *y, false)).collect(),
        FRAME_SUPPORTS.iter().map(|(id, n)| anchor(id, n)).collect(),
        vec![],
        vec![frame_case(0, true), frame_case(1, true)],
        combinations,
    )
}

/// Nodal balance at every node of a frame from published rows: -sum F + P + R = 0.
#[allow(clippy::type_complexity)]
fn node_balance(
    out: &MechanicsEnvelope,
    case: &str,
    node: &str,
    incident: &[(&str, &str)],
    applied: ([f64; 3], [f64; 3]),
    supports: &[&str],
    label: &str,
) {
    let (mut f, mut m) = (applied.0, applied.1);
    let mut fs = norm(applied.0);
    let mut ms = norm(applied.1);
    for (pipe, end) in incident {
        let wall = local(out, case, pipe, end).2;
        let (gf, gm) = global(out, case, pipe, end, wall);
        f = sub(f, gf);
        m = sub(m, gm);
        fs = fs.max(norm(gf));
        ms = ms.max(norm(gm));
    }
    for s in supports {
        let (rf, rm) = support(out, case, s);
        f = add(f, rf);
        m = add(m, rm);
        fs = fs.max(norm(rf));
        ms = ms.max(norm(rm));
    }
    balanced(f, fs, &format!("{label} {case} {node} force"));
    balanced(m, ms, &format!("{label} {case} {node} moment"));
}

#[test]
fn frame_global_rows_are_the_rotation_of_the_local_rows_and_balance_every_node() {
    let wire = frame(vec![]);
    for mode in MODES {
        let label = format!("frame {mode:?}");
        let out = solve(&wire, mode, &label);
        assert_eq!(global_rows(&out), 2 * 4 * 12, "{label}: twelve rows per member and case");
        for (case_index, case) in ["case:one", "case:two"].into_iter().enumerate() {
            // (a) F = R^T f, M = R^T m with R from coordinates and y_reference.
            for (pipe, a, b, yref) in FRAME_MEMBERS {
                let r = straight_axes(position(a), position(b), yref);
                for end in ["end_i", "end_j"] {
                    let (lf, lm, wall) = local(&out, case, pipe, end);
                    assert!(!wall);
                    let (gf, gm) = global(&out, case, pipe, end, false);
                    for (g, l) in [(gf, lf), (gm, lm)] {
                        let expected = rotate_t(r, l);
                        let scale = norm(l).max(f64::MIN_POSITIVE);
                        for k in 0..3 {
                            assert!((g[k] - expected[k]).abs() <= 1e-12 * scale, "{label} {case} {pipe} {end}[{k}]: {} vs {}", g[k], expected[k]);
                        }
                    }
                }
            }
            // (b) free nodes: -sum F + P = 0; (c) supported nodes: -sum F + R = 0.
            let loads = frame_loads(case_index);
            for (node_id, _) in FRAME_NODES {
                let incident: Vec<(&str, &str)> = FRAME_MEMBERS
                    .iter()
                    .filter_map(|(pipe, a, b, _)| if *a == node_id { Some((*pipe, "end_i")) } else if *b == node_id { Some((*pipe, "end_j")) } else { None })
                    .collect();
                let applied = loads.iter().find(|l| l.0 == node_id).map_or(([0.0; 3], [0.0; 3]), |l| (l.1, l.2));
                let supports: Vec<&str> = FRAME_SUPPORTS.iter().filter(|s| s.1 == node_id).map(|s| s.0).collect();
                node_balance(&out, case, node_id, &incident, applied, &supports, &label);
            }
        }
        read_back(&out, "preview-physics", &wire, &label);
    }
}

// ------------------------------------------------------------------ 2. realized curved bend
/// a (0,0,0) - b (1,0,0) straight; b - c (1.25,0.25,0) a realized 90 degree
/// bend of radius 0.25 m in the global xy plane; c - d (1.25,1.25,0) straight.
/// Anchors at a and d; loads at both bend ends.
fn bend_frame(exact_route: bool, pressure: Option<f64>) -> Value {
    let r = 0.25;
    let bend = json!({"id": "component:bend", "label": "w7 arc", "kind": "bend", "node": "node:c",
        "geometry": {"bend_pipe_ref": "pipe:b-c", "bend_radius": qv(r, "m"), "bend_angle": qv(std::f64::consts::FRAC_PI_2, "rad"),
            "bend_plane_orientation": "global_xy_preview", "bend_geometry_source_reference": "w7 invented"},
        "modifiers": {"flexibility_factor_user_value": {"value": 1.0, "unit": "none"}, "source_reference": "w7 invented"},
        "mechanics_interface": {"solver_consumption": "curved_bend_macro_element", "rule_check_consumption": "user_rule_pack_inputs_only"},
        "provenance": PROV});
    let nodes = vec![node("node:a", [0.0, 0.0, 0.0]), node("node:b", [1.0, 0.0, 0.0]), node("node:c", [1.0 + r, r, 0.0]), node("node:d", [1.0 + r, 1.0 + r, 0.0])];
    let pipes = vec![
        pipe("pipe:a-b", "node:a", "node:b", [0.0, 1.0, 0.0], exact_route),
        pipe("pipe:b-c", "node:b", "node:c", [std::f64::consts::FRAC_1_SQRT_2, -std::f64::consts::FRAC_1_SQRT_2, 0.0], exact_route),
        pipe("pipe:c-d", "node:c", "node:d", [1.0, 0.0, 0.0], exact_route),
    ];
    let supports = vec![anchor("support:a", "node:a"), anchor("support:d", "node:d")];
    let mut loads = nodal("load:b", "node:b", [500.0, -300.0, 800.0], [40.0, -90.0, 65.0]);
    loads.extend(nodal("load:c", "node:c", [-250.0, 700.0, -450.0], [-30.0, 55.0, 120.0]));
    if exact_route {
        let regions = match pressure {
            Some(p) => json!([{"id": "region:w7", "member_pipe_ids": ["pipe:a-b", "pipe:b-c", "pipe:c-d"],
                "pressure_basis": "internal_differential_zero_external_v1", "pressure": qv(p, "Pa"),
                "terminals": [{"node_ref": "node:a", "closure_transfer": "transfers_to_wall", "provenance": PROV},
                    {"node_ref": "node:d", "closure_transfer": "transfers_to_wall", "provenance": PROV}],
                "provenance": PROV}]),
            None => json!([]),
        };
        exact(nodes, pipes, supports, vec![bend], vec![json!({"id": "case:bend", "primitive_loads": loads, "pressure_regions": regions, "provenance": PROV})])
    } else {
        preview(nodes, pipes, supports, vec![bend], vec![json!({"id": "case:bend", "primitive_loads": loads, "provenance": PROV})], vec![])
    }
}

#[test]
fn bend_ends_balance_with_the_neighbouring_straight_members_on_both_routes() {
    for (exact_route, reader) in [(false, "preview-physics"), (true, "pressure")] {
        let wire = bend_frame(exact_route, None);
        for mode in MODES {
            let label = format!("bend exact={exact_route} {mode:?}");
            let out = solve(&wire, mode, &label);
            assert_eq!(global_rows(&out), 3 * 12, "{label}");
            // The balance at b and c couples the bend's end-tangent rotation with
            // the straight members' axes; it does not read the local frames.
            node_balance(&out, "case:bend", "node:b", &[("pipe:a-b", "end_j"), ("pipe:b-c", "end_i")], ([500.0, -300.0, 800.0], [40.0, -90.0, 65.0]), &[], &label);
            node_balance(&out, "case:bend", "node:c", &[("pipe:b-c", "end_j"), ("pipe:c-d", "end_i")], ([-250.0, 700.0, -450.0], [-30.0, 55.0, 120.0]), &[], &label);
            node_balance(&out, "case:bend", "node:a", &[("pipe:a-b", "end_i")], ([0.0; 3], [0.0; 3]), &["support:a"], &label);
            node_balance(&out, "case:bend", "node:d", &[("pipe:c-d", "end_j")], ([0.0; 3], [0.0; 3]), &["support:d"], &label);
            // The bend's end tangents (+x at b, +y at c) carry its local axial rows.
            for (end, tangent) in [("end_i", [1.0, 0.0, 0.0]), ("end_j", [0.0, 1.0, 0.0])] {
                let (lf, _, _) = local(&out, "case:bend", "pipe:b-c", end);
                let (gf, _) = global(&out, "case:bend", "pipe:b-c", end, false);
                assert!((dot(gf, tangent) - lf[0]).abs() <= 1e-12 * norm(lf).max(f64::MIN_POSITIVE), "{label} {end}: axial along the end tangent");
            }
            read_back(&out, reader, &wire, &label);
        }
    }
}

// ------------------------------------------------------------------ 3. EX pressure members
/// Two collinear inclined pressure members A - M - B (direction (1,2,2)/3, 3 m
/// each), anchored at A and B with transferring terminals; a load at M.
fn pressure_line(p: f64) -> Value {
    let nodes = vec![node("node:A", [0.0, 0.0, 0.0]), node("node:M", [1.0, 2.0, 2.0]), node("node:B", [2.0, 4.0, 4.0])];
    let pipes = vec![pipe("pipe:A-M", "node:A", "node:M", [0.0, 0.0, 1.0], true), pipe("pipe:M-B", "node:M", "node:B", [0.0, 0.0, 1.0], true)];
    let supports = vec![anchor("support:A", "node:A"), anchor("support:B", "node:B")];
    let loads = nodal("load:M", "node:M", [400.0, -150.0, 250.0], [35.0, -20.0, 60.0]);
    let region = json!({"id": "region:line", "member_pipe_ids": ["pipe:A-M", "pipe:M-B"], "pressure_basis": "internal_differential_zero_external_v1",
        "pressure": qv(p, "Pa"), "terminals": [{"node_ref": "node:A", "closure_transfer": "transfers_to_wall", "provenance": PROV},
            {"node_ref": "node:B", "closure_transfer": "transfers_to_wall", "provenance": PROV}], "provenance": PROV});
    exact(nodes, pipes, supports, vec![], vec![json!({"id": "case:p", "primitive_loads": loads, "pressure_regions": [region], "provenance": PROV})])
}

#[test]
fn pressure_members_rotate_the_published_wall_action() {
    let wire = pressure_line(2e6);
    let positions = [("node:A", [0.0, 0.0, 0.0]), ("node:M", [1.0, 2.0, 2.0]), ("node:B", [2.0, 4.0, 4.0])];
    let at = |id: &str| positions.iter().find(|p| p.0 == id).unwrap().1;
    for mode in MODES {
        let label = format!("pressure {mode:?}");
        let out = solve(&wire, mode, &label);
        assert_eq!(out.producer.semantic_contract_id, "openpipestress.result_semantics/0.3.0/pressure-2");
        for (pipe, a, b) in [("pipe:A-M", "node:A", "node:M"), ("pipe:M-B", "node:M", "node:B")] {
            let r = straight_axes(at(a), at(b), [0.0, 0.0, 1.0]);
            for end in ["end_i", "end_j"] {
                let (lf, lm, wall) = local(&out, "case:p", pipe, end);
                assert!(wall, "{label}: {pipe} publishes its wall action at {end}");
                let (gf, gm) = global(&out, "case:p", pipe, end, true);
                // The projections on the independent axes are the published rows:
                // the wall action on local x, the shears and moments on y and z.
                for axis in 0..3 {
                    assert!((dot(gf, r[axis]) - lf[axis]).abs() <= 1e-12 * norm(lf), "{label} {pipe} {end} force axis {axis}");
                    assert!((dot(gm, r[axis]) - lm[axis]).abs() <= 1e-12 * norm(lm).max(f64::MIN_POSITIVE), "{label} {pipe} {end} moment axis {axis}");
                }
                // The axial is the wall force, not the elastic or effective force.
                let wall_force = one(&out, "case:p", "pipe_wall_axial_force_v2", pipe, None, Some(end)).unwrap().value;
                assert!((lf[0].abs() - wall_force.abs()).abs() <= 1e-12 * wall_force.abs(), "{label} {pipe} {end}: |wall end action| = |Nw|");
            }
        }
        // Balance checked: at M, an interior node of a straight collinear run, no
        // physical pressure load acts (no cap, no junction remainder), so the wall
        // end actions balance the user load alone. At A and B the transferred end
        // caps act; that balance is not asserted here.
        node_balance(&out, "case:p", "node:M", &[("pipe:A-M", "end_j"), ("pipe:M-B", "end_i")], ([400.0, -150.0, 250.0], [35.0, -20.0, 60.0]), &[], &label);
        read_back(&out, "pressure", &wire, &label);
    }
}

#[test]
fn pressurized_bend_rotates_its_wall_action_along_the_end_tangents() {
    let wire = bend_frame(true, Some(1.5e6));
    for mode in MODES {
        let label = format!("pressurized bend {mode:?}");
        let out = solve(&wire, mode, &label);
        for (end, tangent) in [("end_i", [1.0, 0.0, 0.0]), ("end_j", [0.0, 1.0, 0.0])] {
            let (lf, _, wall) = local(&out, "case:bend", "pipe:b-c", end);
            assert!(wall, "{label} {end}");
            let (gf, _) = global(&out, "case:bend", "pipe:b-c", end, true);
            assert!((dot(gf, tangent) - lf[0]).abs() <= 1e-12 * norm(lf), "{label} {end}: the wall action along the end tangent");
            assert!((norm(gf) - norm(lf)).abs() <= 1e-12 * norm(lf), "{label} {end}");
        }
        read_back(&out, "pressure", &wire, &label);
    }
}

// ------------------------------------------------------------------ 4. PF combination
#[test]
fn combined_global_rows_are_the_factored_sum_of_the_case_rows() {
    let (a, b) = (1.5, -0.7);
    let wire = frame(vec![json!({"id": "combination:w7", "basis": "mechanics",
        "terms": [{"load_case": "case:one", "factor": a}, {"load_case": "case:two", "factor": b}], "provenance": PROV})]);
    for mode in MODES {
        let label = format!("combination {mode:?}");
        let out = solve(&wire, mode, &label);
        let combined: Vec<&ResultItem> = out.results.iter().filter(|r| (r.kind == FORCE || r.kind == MOMENT) && r.basis_ref.as_ref().is_some_and(|b| b.ref_type == "combination")).collect();
        assert_eq!(combined.len(), 4 * 12, "{label}: every member's twelve rows combine");
        for row in combined {
            let m = row.metadata.as_ref().unwrap();
            assert_eq!((m.coordinate_system.as_str(), m.basis.as_str()), ("global", "explicit_user_linear_combination"), "{}", row.id);
            let operand = |case: &str| one(&out, case, &row.kind, &row.entity_ref, Some(&m.component), Some(&m.location)).unwrap().value;
            let (one_value, two_value) = (operand("case:one"), operand("case:two"));
            let expected = a * one_value + b * two_value;
            let scale = (a * one_value).abs().max((b * two_value).abs()).max(f64::MIN_POSITIVE);
            assert!((row.value - expected).abs() <= 1e-9 * scale, "{label} {}: {} vs {expected}", row.id, row.value);
            assert_eq!(row.source_result_refs.len(), 2, "{}", row.id);
        }
        read_back(&out, "preview-physics", &wire, &label);
    }
}

// ------------------------------------------------------------------ 5. W-9 L frame
/// The W9-A model inputs, copied as literals from PR #1548
/// (`validation/references/w9_benchmarks/references.json`, case W9-A): a
/// sharp-corner L frame A(0,0,0) - C(4,0,0) - D(4,3,0), both ends fixed, OD
/// 0.2 m, wall 0.01 m, E = 200 GPa, G = 76923076923 Pa (PF), alpha = 1.2e-5
/// /K and a 100 K rise on both members.
fn w9_l_frame() -> Value {
    let pipe9 = |id: &str, from: &str, to: &str, y: [f64; 3]| {
        json!({"id": id, "from": from, "to": to, "section": {"outside_diameter": qv(0.2, "m"), "wall_thickness": qv(0.01, "m")},
            "material": "material:w9", "y_reference": {"x": y[0], "y": y[1], "z": y[2]}, "provenance": PROV})
    };
    json!({"model": {"schema_version": "0.8.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:w7-w9", "units": serde_json::from_str::<Value>(UNITS).unwrap()}, "analysis_status": status(),
        "nodes": [node("node:a", [0.0, 0.0, 0.0]), node("node:c", [4.0, 0.0, 0.0]), node("node:d", [4.0, 3.0, 0.0])],
        "pipe_segments": [pipe9("pipe:a-c", "node:a", "node:c", [0.0, 1.0, 0.0]), pipe9("pipe:c-d", "node:c", "node:d", [-1.0, 0.0, 0.0])],
        "supports": [anchor("support:a", "node:a"), anchor("support:d", "node:d")], "components": [],
        "materials": [{"id": "material:w9", "elastic_modulus": qv(200000000000.0, "Pa"), "shear_modulus": qv(76923076923.0, "Pa"),
            "thermal_expansion_coefficient": qv(1.2e-5, "1/degC"), "provenance": PROV}],
        "load_cases": [{"id": "case:w9", "primitive_loads": [thermal("w9", "pipe:a-c", 100.0), thermal("w9", "pipe:c-d", 100.0)], "provenance": PROV}],
        "combinations": []}, "materials": []})
}

#[test]
fn w9_l_frame_end_rows_equal_the_frozen_reactions() {
    // Frozen W9-A support reactions (support on structure), exact q*pi from the
    // package's README: R_D = (X, Y, 0; 0, 0, M), R_A = (-X, -Y, 0; 0, 0, M_A).
    let q = |p: f64| p / 405751901443.0 * PI;
    let (x, y, m) = (q(-1113526438008000.0), q(-675053938008000.0), q(-1853250674724000.0));
    let ma = q(1212887112732000.0);
    let reactions = [("support:a", "pipe:a-c", "end_i", [-x, -y, 0.0], [0.0, 0.0, ma]), ("support:d", "pipe:c-d", "end_j", [x, y, 0.0], [0.0, 0.0, m])];
    // The README's decimal statement of the same values.
    for (value, decimal) in [(x, -8621.63914644), (y, -5226.70253640), (m, -14349.0608036), (ma, 9390.95350987)] {
        assert!((value - decimal).abs() <= 1e-9 * decimal.abs(), "{value} vs {decimal}");
    }
    let wire = w9_l_frame();
    for mode in MODES {
        let label = format!("w9 {mode:?}");
        let out = solve(&wire, mode, &label);
        for (support_id, pipe, end, rf, rm) in reactions {
            // The node carries only the member end and the support: -F + R = 0, so
            // the member's global end action equals the support's frozen action.
            let (gf, gm) = global(&out, "case:w9", pipe, end, false);
            let scale = norm(rf).max(norm(rm));
            for k in 0..3 {
                assert!((gf[k] - rf[k]).abs() <= 1e-9 * if rf[k] == 0.0 { scale } else { rf[k].abs() }, "{label} {support_id} F[{k}]: {} vs {}", gf[k], rf[k]);
                assert!((gm[k] - rm[k]).abs() <= 1e-9 * if rm[k] == 0.0 { scale } else { rm[k].abs() }, "{label} {support_id} M[{k}]: {} vs {}", gm[k], rm[k]);
            }
        }
        // The corner C carries no load and no support: the two end rows balance.
        node_balance(&out, "case:w9", "node:c", &[("pipe:a-c", "end_j"), ("pipe:c-d", "end_i")], ([0.0; 3], [0.0; 3]), &[], &label);
        read_back(&out, "preview-physics", &wire, &label);
    }
}

// ------------------------------------------------------------------ 6. source-recovery publication
/// A source-selected EX publication (pressure-source-1) carries no global end
/// row on any of its cases (`global_end_rows_published`) and publishes exactly
/// its committed bytes, qualified, as before W-7.
/// The committed source-selected requests and their committed raw publications.
fn source_fixture(name: &str, mode: PreviewSolverMode) -> (&'static str, &'static str) {
    match (name, mode) {
        ("mixed", PreviewSolverMode::SparseInteractive) => (include_str!("../../../fixtures/product_preview/pressure_source/physics_source/mixed.request.json"), include_str!("../../../fixtures/product_preview/pressure_source/physics_source/mixed-sparse_interactive.raw.json")),
        ("mixed", PreviewSolverMode::DenseScrutiny) => (include_str!("../../../fixtures/product_preview/pressure_source/physics_source/mixed.request.json"), include_str!("../../../fixtures/product_preview/pressure_source/physics_source/mixed-dense_scrutiny.raw.json")),
        ("n05", PreviewSolverMode::SparseInteractive) => (include_str!("../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json"), include_str!("../../../fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json")),
        ("n05", PreviewSolverMode::DenseScrutiny) => (include_str!("../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json"), include_str!("../../../fixtures/product_preview/pressure_source/physics_source/n05-dense_scrutiny.raw.json")),
        _ => unreachable!(),
    }
}

#[test]
fn source_recovery_publications_omit_the_rows_and_keep_their_bytes() {
    for name in ["mixed", "n05"] {
        for mode in MODES {
            let label = format!("{name} {mode:?}");
            let (request_text, raw_text) = source_fixture(name, mode);
            let request: Value = serde_json::from_str(request_text).unwrap();
            let out = run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap();
            assert_eq!(out.producer.semantic_contract_id, "openpipestress.result_semantics/0.3.0/pressure-source-1", "{label}");
            assert_eq!(global_rows(&out), 0, "{label}");
            let raw = serde_json::to_value(&out).unwrap();
            assert_eq!(raw["source_block_recovery"]["body"]["status"], "qualified", "{label}: {}", raw["diagnostics"]);
            let committed: Value = serde_json::from_str(raw_text).unwrap();
            assert!(raw == committed, "{label}: the publication is its committed bytes");
            let verdict = reader_verdict::validate("pressure-source", &raw, None, Some(&json!({"request": request, "solver_mode": mode.as_str()})));
            assert_eq!(verdict["ok"], true, "{label}: {verdict}");
        }
    }
}

// ------------------------------------------------------------------ 7. reader refusals
/// Malformed global rows in a solved envelope are refused by name by both readers.
#[test]
fn readers_refuse_malformed_global_rows_by_name() {
    let cases: [(&str, Value); 2] = [("preview-physics", frame(vec![])), ("pressure", pressure_line(2e6))];
    for (reader, wire) in cases {
        let out = solve(&wire, PreviewSolverMode::SparseInteractive, reader);
        let raw = read_back(&out, reader, &wire, reader);
        let first = raw["results"].as_array().unwrap().iter().position(|r| r["kind"] == FORCE).unwrap();
        let refused = |edit: &dyn Fn(&mut Value), code: &str| {
            let mut bad = raw.clone();
            edit(&mut bad);
            let verdict = reader_verdict::validate(reader, &bad, Some(&wire["model"]), None);
            assert_eq!(verdict["ok"], json!(false), "{reader} {code}: {verdict}");
            assert!(verdict.to_string().contains(code), "{reader}: expected {code}, got {verdict}");
        };
        refused(&|v| v["results"][first]["metadata"]["coordinate_system"] = json!("element_local"), "SOURCE_GLOBAL_END_COORDINATE_SYSTEM");
        refused(&|v| v["results"][first]["metadata"]["location"] = json!("midspan"), "SOURCE_GLOBAL_END_LOCATION");
        refused(&|v| v["results"][first]["metadata"]["sign_convention"] = json!("node on element"), "SOURCE_GLOBAL_END_SIGN_CONVENTION");
        refused(&|v| v["results"][first]["metadata"]["basis"] = json!("recovered_from_local_element_stiffness"), "SOURCE_GLOBAL_END_BASIS");
        refused(&|v| { let x = v["results"][first]["value"].as_f64().unwrap(); v["results"][first]["value"] = json!(x * 1.001 + 1.0); }, "SOURCE_GLOBAL_END_ROTATION_NORM");
        refused(&|v| { v["results"].as_array_mut().unwrap().remove(first); }, "SOURCE_GLOBAL_END_COVERAGE");
        refused(&|v| { let copy = v["results"][first].clone(); let mut copy = copy; copy["id"] = json!("result:w7:duplicate"); v["results"].as_array_mut().unwrap().push(copy); }, "SOURCE_GLOBAL_END_SLOT_DUPLICATE");
    }
}

// Regression from independent W-7 review: a frame admitted by the kernel's
// 1e-12 orientation tolerance must remain readable after global row rotation.
#[test]
fn readers_accept_global_end_rows_from_an_admitted_near_parallel_y_reference() {
    for exact_route in [false, true] {
        let nodes = vec![node("node:a", [0.0, 0.0, 0.0]), node("node:b", [1.0, 2.0, 3.0])];
        let pipes = vec![pipe("pipe:ab", "node:a", "node:b", [1.001, 2.0, 3.0], exact_route)];
        let supports = vec![anchor("support:a", "node:a")];
        let mut case = json!({"id": "case:edge", "primitive_loads":
            nodal("load:edge", "node:b", [-1230.0, -386.0, -579.0], [19.0, -37.0, 43.0]),
            "provenance": PROV});
        let (wire, reader) = if exact_route {
            case["pressure_regions"] = json!([]);
            (exact(nodes, pipes, supports, vec![], vec![case]), "pressure")
        } else {
            (preview(nodes, pipes, supports, vec![], vec![case], vec![]), "preview-physics")
        };
        for mode in MODES {
            let out = solve(&wire, mode, "admitted near-parallel y_reference");
            assert_eq!(global_rows(&out), 12);
            let raw = serde_json::to_value(&out).unwrap();
            // Removing only W-7 rows is a control: all other reader checks pass.
            let mut old = raw.clone();
            old["results"].as_array_mut().unwrap().retain(|r|
                r["kind"] != FORCE && r["kind"] != MOMENT);
            assert_eq!(reader_verdict::validate(reader, &old, Some(&wire["model"]), None),
                json!({"ok": true}));
            assert_eq!(reader_verdict::validate(reader, &raw, Some(&wire["model"]), None),
                json!({"ok": true}), "{reader} {mode:?}: producer output must remain readable");
        }
    }
}
