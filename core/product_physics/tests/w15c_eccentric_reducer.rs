//! W-15 slice c (DEL-03-05 Design `eccentric-reducer-offset.md` §3.2-§3.4, §12):
//! the eccentric offset check through the public solve, on both routes. Every
//! model is invented: a cantilever from an anchor at N0 through a reducer body
//! to a free tip, sections and material chosen for the benchmark family, no
//! catalog, vendor or code value. Option A of R-c1 (from the component's node)
//! is used throughout; the convention-only tests are in `w15c_offset_convention.rs`.
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, MechanicsEnvelope, PreviewSolverMode};
use serde_json::{json, Value};

const PROV: &str = "w15c invented check input; not library, component, material-catalog or code data";
const INVALID: &str = "RIGID_COMPONENT_ECCENTRIC_OFFSET_INVALID";
const BELOW: &str = "RIGID_COMPONENT_ECCENTRIC_OFFSET_BELOW_PRECISION";
const NOT_TRANSVERSE: &str = "RIGID_COMPONENT_ECCENTRIC_OFFSET_NOT_TRANSVERSE";
const MISMATCH: &str = "RIGID_COMPONENT_ECCENTRIC_OFFSET_MISMATCH";
const REALIZED: &str = "RIGID_COMPONENT_ECCENTRIC_OFFSET_REALIZED";

#[derive(Clone, Copy, PartialEq, Debug)]
enum Route {
    Preview,
    Exact,
}

fn qv(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}

/// A model in `length_unit` with nodes, straight members (from, to) and the
/// given components; anchored at the first node; a 10 kN X force at `load_node`.
struct Model {
    length_unit: &'static str,
    nodes: Vec<(&'static str, [f64; 3])>,
    members: Vec<(&'static str, &'static str, &'static str)>,
    components: Vec<Value>,
    load_node: &'static str,
    tolerance_mm: Option<f64>,
}

fn request(m: &Model, route: Route) -> Value {
    let nodes: Vec<Value> = m.nodes.iter().map(|(id, p)| json!({"id": id, "position": {"x": p[0], "y": p[1], "z": p[2]}, "provenance": PROV})).collect();
    let pipes: Vec<Value> = m.members.iter().map(|(id, from, to)| {
        let mut sec = json!({"outside_diameter": qv(0.2, "m"), "wall_thickness": qv(0.01, "m")});
        if route == Route::Exact {
            sec["corrosion_allowance"] = qv(0.0, "m");
        }
        json!({"id": id, "from": from, "to": to, "section": sec, "material": "material:w15c",
            "y_reference": {"x": 0.0, "y": 0.0, "z": 1.0}, "provenance": PROV})
    }).collect();
    let units = json!({"length": m.length_unit, "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"});
    let status = json!({"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"});
    let load = json!({"id": "load:fx", "category": "concentrated_force", "target": {"type": "node", "node": m.load_node}, "direction": "global_x",
        "magnitude": qv(10000.0, "N"), "dimension": "force", "provenance": PROV});
    let mut case = json!({"id": "case:w15c", "primitive_loads": [load], "provenance": PROV});
    let mut model = json!({"schema_version": "0.7.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:w15c", "units": units}, "analysis_status": status, "nodes": nodes, "pipe_segments": pipes,
        "supports": [{"id": "support:n0", "node": m.nodes[0].0, "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV}],
        "components": m.components, "combinations": []});
    if let Some(t) = m.tolerance_mm {
        model["model_settings"] = json!({"node_proximity_tolerance": qv(t, "mm")});
    }
    match route {
        Route::Preview => {
            model["materials"] = json!([{"id": "material:w15c", "elastic_modulus": qv(200e9, "Pa"), "shear_modulus": qv(80e9, "Pa"),
                "thermal_expansion_coefficient": qv(1.2e-5, "1/degC"), "provenance": PROV}]);
        }
        Route::Exact => {
            model["pressure_contract"] = json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"});
            model["materials"] = json!([{"id": "material:w15c", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
                "elastic_modulus": qv(200e9, "Pa"), "poisson_ratio": qv(0.25, "1"), "provenance": PROV}]);
            case["pressure_regions"] = json!([]);
        }
    }
    model["load_cases"] = json!([case]);
    json!({"model": model, "materials": []})
}

/// A reducer at `node` mapping `member`, with `offset` in mm (absent when None).
fn reducer(id: &str, node: &str, member: &str, offset: Option<[f64; 3]>, consumption: &str) -> Value {
    let mut geometry = json!({"rigid_pipe_ref": member, "rigid_body_length": qv(120.0, "mm")});
    if let Some(e) = offset {
        geometry["eccentric_offset"] = json!({"x": e[0], "y": e[1], "z": e[2], "unit": "mm"});
    }
    json!({"id": id, "kind": "reducer", "node": node, "geometry": geometry,
        "modifiers": {"stiffness_scaling_user_value": qv(1000.0, "1"), "source_reference": "invented W-15c benchmark scale"},
        "mechanics_interface": {"solver_consumption": consumption}, "provenance": PROV})
}

fn solve(m: &Model, route: Route) -> MechanicsEnvelope {
    run_linear_static_preview_value_with_mode(request(m, route), PreviewSolverMode::SparseInteractive)
        .unwrap_or_else(|e| panic!("typed boundary rejected the request: {e}"))
}

fn codes(out: &MechanicsEnvelope, prefix: &str) -> Vec<(String, String)> {
    out.diagnostics.iter().filter(|d| d.code.starts_with(prefix)).map(|d| (d.code.clone(), d.message.clone())).collect()
}

fn offset_codes(out: &MechanicsEnvelope) -> Vec<String> {
    codes(out, "RIGID_COMPONENT_ECCENTRIC_OFFSET").into_iter().map(|(c, _)| c).collect()
}

/// Solved on both routes with exactly one realized info for each named reducer.
fn assert_accepted(m: &Model, reducers: &[&str]) {
    for route in [Route::Preview, Route::Exact] {
        let out = solve(m, route);
        let blocking: Vec<String> = out.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| format!("{}: {}", d.code, d.message)).collect();
        assert_eq!(out.status.mechanics, "MECHANICS_SOLVED", "{route:?}: {blocking:?}");
        assert_eq!(offset_codes(&out), vec![REALIZED.to_string(); reducers.len()], "{route:?}");
        for r in reducers {
            assert!(out.diagnostics.iter().any(|d| d.code == REALIZED && d.affected_refs == vec![r.to_string()]), "{route:?}: {r}");
        }
    }
}

/// Refused on both routes by `code`, whose message contains `needle`; nothing solved.
fn assert_refused(m: &Model, code: &str, needle: &str) {
    for route in [Route::Preview, Route::Exact] {
        let out = solve(m, route);
        assert_ne!(out.status.mechanics, "MECHANICS_SOLVED", "{route:?}");
        let found = codes(&out, code);
        assert!(found.iter().any(|(c, msg)| c == code && msg.contains(needle)), "{route:?}: expected {code} with {needle:?}, got {:?}", codes(&out, "RIGID_COMPONENT_ECCENTRIC"));
        assert!(!offset_codes(&out).contains(&REALIZED.to_string()), "{route:?}: no realized info on a refused model");
    }
}

/// The §7.1 cantilever in metres: N0 → A on X, the reducer body A → B with
/// chord (0.12, b_y, 0), the downstream pipe B → C on X.
fn cantilever(b_y: f64, offset: Option<[f64; 3]>) -> Model {
    Model {
        length_unit: "m",
        nodes: vec![("node:n0", [0.0, 0.0, 0.0]), ("node:a", [1.0, 0.0, 0.0]), ("node:b", [1.12, b_y, 0.0]), ("node:c", [2.12, b_y, 0.0])],
        members: vec![("pipe:n0-a", "node:n0", "node:a"), ("body:a-b", "node:a", "node:b"), ("pipe:b-c", "node:b", "node:c")],
        components: vec![reducer("component:r", "node:a", "body:a-b", offset, "mechanics_geometry_only")],
        load_node: "node:c",
        tolerance_mm: None,
    }
}

#[test]
fn the_benchmark_geometry_is_accepted_and_realized() {
    assert_accepted(&cantilever(-0.05, Some([0.0, -50.0, 0.0])), &["component:r"]);
}

/// P2: 47 mm declared against 50 mm geometry passes placement (residual 3 mm),
/// and the actual 50 mm governs: the results are bit-identical to the same
/// model without the offset member.
#[test]
fn a_declared_offset_within_tolerance_does_not_change_the_solve() {
    for route in [Route::Preview, Route::Exact] {
        let declared = solve(&cantilever(-0.05, Some([0.0, -47.0, 0.0])), route);
        let undeclared = solve(&cantilever(-0.05, None), route);
        assert_eq!(declared.status.mechanics, "MECHANICS_SOLVED", "{route:?}");
        let info = codes(&declared, REALIZED);
        assert_eq!(info.len(), 1, "{route:?}");
        assert!(info[0].1.contains("(0e0, -4.7e-2, 0e0) m") && info[0].1.contains("-5e-2"), "{route:?}: {}", info[0].1);
        let values = |out: &MechanicsEnvelope| out.results.iter().map(|r| (r.id.clone(), r.value.to_bits())).collect::<Vec<_>>();
        assert_eq!(values(&declared), values(&undeclared), "{route:?}: the node geometry governs");
    }
}

#[test]
fn an_explicit_zero_offset_on_collinear_nodes_is_accepted() {
    assert_accepted(&cantilever(0.0, Some([0.0, 0.0, 0.0])), &["component:r"]);
}

#[test]
fn an_explicit_zero_offset_on_a_slanted_chord_is_a_mismatch() {
    assert_refused(&cantilever(-0.05, Some([0.0, 0.0, 0.0])), MISMATCH, "residual 5e-2 m");
}

#[test]
fn mismatches_are_refused_by_name() {
    assert_refused(&cantilever(-0.05, Some([0.0, 50.0, 0.0])), MISMATCH, "residual 1e-1 m");
    assert_refused(&cantilever(-0.05, Some([0.0, -40.0, 0.0])), MISMATCH, "residual 1");
    let mut tight = cantilever(-0.05, Some([0.0, -47.0, 0.0]));
    tight.tolerance_mm = Some(1.0);
    assert_refused(&tight, MISMATCH, "tolerance 1e-3 m");
}

/// The wrong transverse axis: the geometry offsets in Y, the declaration in Z.
#[test]
fn an_offset_on_the_wrong_axis_is_a_mismatch() {
    assert_refused(&cantilever(-0.05, Some([0.0, 0.0, -50.0])), MISMATCH, "eccentric offset");
}

#[test]
fn the_collinear_offset_is_refused_as_not_transverse() {
    assert_refused(&cantilever(0.0, Some([5.0, 0.0, 0.0])), NOT_TRANSVERSE, "axial component");
}

#[test]
fn the_transversality_bound_from_both_sides() {
    assert_accepted(&cantilever(-0.05, Some([1.00, -50.0, 0.0])), &["component:r"]);
    assert_refused(&cantilever(-0.05, Some([1.01, -50.0, 0.0])), NOT_TRANSVERSE, "axial component");
}

#[test]
fn the_precision_bound_from_both_sides() {
    assert_refused(&cantilever(-0.0008, Some([0.0, -0.8, 0.0])), BELOW, "declare a zero offset");
    assert_accepted(&cantilever(-0.0009, Some([0.0, -0.9, 0.0])), &["component:r"]);
}

/// P2 decision: the 30° run entered in mm, rounded; axial share 0.003015789.
#[test]
fn the_thirty_degree_run_entered_in_millimetres_is_accepted() {
    let model = Model {
        length_unit: "mm",
        nodes: vec![("node:n0", [0.0, 0.0, 0.0]), ("node:a", [866.0, 500.0, 0.0]), ("node:b", [945.0, 603.0, 0.0]), ("node:c", [1811.0, 1103.0, 0.0])],
        members: vec![("pipe:n0-a", "node:n0", "node:a"), ("body:a-b", "node:a", "node:b"), ("pipe:b-c", "node:b", "node:c")],
        components: vec![reducer("component:r", "node:a", "body:a-b", Some([-25.0, 43.0, 0.0]), "mechanics_geometry_only")],
        load_node: "node:c",
        tolerance_mm: None,
    };
    assert_accepted(&model, &["component:r"]);
}

/// P3 decision: two parallel 100 mm spans rounded to (87, 50) and (86, 50) mm.
#[test]
fn two_rounded_hundred_millimetre_spans_are_accepted() {
    let model = Model {
        length_unit: "mm",
        nodes: vec![("node:n0", [0.0, 0.0, 0.0]), ("node:a", [87.0, 50.0, 0.0]), ("node:b", [166.0, 153.0, 0.0]), ("node:c", [252.0, 203.0, 0.0])],
        members: vec![("pipe:n0-a", "node:n0", "node:a"), ("body:a-b", "node:a", "node:b"), ("pipe:b-c", "node:b", "node:c")],
        components: vec![reducer("component:r", "node:a", "body:a-b", Some([-25.0, 43.0, 0.0]), "mechanics_geometry_only")],
        load_node: "node:c",
        tolerance_mm: None,
    };
    assert_accepted(&model, &["component:r"]);
}

/// The far run rotated by θ (rad) about Z.
fn rotated(theta: f64) -> Model {
    let mut m = cantilever(-0.05, Some([0.0, -50.0, 0.0]));
    m.nodes[3].1 = [1.12 + theta.cos(), -0.05 + theta.sin(), 0.0];
    m
}

#[test]
fn the_agreement_bound_from_both_sides() {
    assert_accepted(&rotated(-4.46e-3), &["component:r"]);
    assert_refused(&rotated(-4.47e-3), INVALID, "reason run_directions_disagree");
}

/// Codex r2: two successive reducer chords (0.12, −0.05, 0) after an X pipe,
/// the first undeclared and the second with e = 0. An undeclared reducer is
/// not run evidence, so the second cannot take the first's slanted chord as
/// its axis.
#[test]
fn an_undeclared_neighbouring_reducer_is_not_run_evidence() {
    let model = Model {
        length_unit: "m",
        nodes: vec![("node:n0", [0.0, 0.0, 0.0]), ("node:a", [1.0, 0.0, 0.0]), ("node:b", [1.12, -0.05, 0.0]), ("node:c", [1.24, -0.10, 0.0])],
        members: vec![("pipe:n0-a", "node:n0", "node:a"), ("body:a-b", "node:a", "node:b"), ("body:b-c", "node:b", "node:c")],
        components: vec![
            reducer("component:r1", "node:a", "body:a-b", None, "mechanics_geometry_only"),
            reducer("component:r2", "node:b", "body:b-c", Some([0.0, 0.0, 0.0]), "mechanics_geometry_only"),
        ],
        load_node: "node:c",
        tolerance_mm: None,
    };
    assert_refused(&model, INVALID, "reason unsupported_run_evidence");
}

#[test]
fn a_neighbouring_reducer_with_any_offset_state_is_unsupported() {
    for neighbour in [None, Some([0.0, 0.0, 0.0])] {
        let model = Model {
            length_unit: "m",
            nodes: vec![("node:n0", [0.0, 0.0, 0.0]), ("node:a", [1.0, 0.0, 0.0]), ("node:b", [1.12, 0.0, 0.0]), ("node:c", [1.24, 0.0, 0.0]), ("node:d", [2.24, 0.0, 0.0])],
            members: vec![("pipe:n0-a", "node:n0", "node:a"), ("body:a-b", "node:a", "node:b"), ("body:b-c", "node:b", "node:c"), ("pipe:c-d", "node:c", "node:d")],
            components: vec![
                reducer("component:r1", "node:a", "body:a-b", neighbour, "mechanics_geometry_only"),
                reducer("component:r2", "node:b", "body:b-c", Some([0.0, 0.0, 0.0]), "mechanics_geometry_only"),
            ],
            load_node: "node:d",
            tolerance_mm: None,
        };
        assert_refused(&model, INVALID, "the body of another reducer");
    }
}

/// The first pass looks for the slot on any component: a valve carrying it in a
/// model with no reducer is refused, not skipped.
#[test]
fn an_offset_on_a_non_reducer_is_refused_even_without_a_reducer() {
    let mut model = cantilever(0.0, None);
    model.components = vec![json!({"id": "component:v", "kind": "valve", "node": "node:a",
        "geometry": {"rigid_pipe_ref": "body:a-b", "eccentric_offset": {"x": 0.0, "y": -50.0, "z": 0.0, "unit": "mm"}},
        "mechanics_interface": {"solver_consumption": "not_solver_consumed"}, "provenance": PROV})];
    assert_refused(&model, INVALID, "reason not_reducer");
}

#[test]
fn an_unchecked_mapping_is_refused() {
    let mut model = cantilever(-0.05, Some([0.0, -50.0, 0.0]));
    model.components = vec![reducer("component:r", "node:a", "pipe:b-c", Some([0.0, -50.0, 0.0]), "mechanics_geometry_only")];
    assert_refused(&model, INVALID, "reason mapping_invalid");
}

#[test]
fn a_tee_at_the_node_is_ambiguous() {
    let mut model = cantilever(-0.05, Some([0.0, -50.0, 0.0]));
    model.nodes.push(("node:t", [1.0, 1.0, 0.0]));
    model.members.push(("pipe:a-t", "node:a", "node:t"));
    assert_refused(&model, INVALID, "reason ambiguous_run");
}

#[test]
fn a_standalone_body_has_no_run() {
    let model = Model {
        length_unit: "m",
        nodes: vec![("node:a", [1.0, 0.0, 0.0]), ("node:b", [1.12, -0.05, 0.0])],
        members: vec![("body:a-b", "node:a", "node:b")],
        components: vec![reducer("component:r", "node:a", "body:a-b", Some([0.0, -50.0, 0.0]), "mechanics_geometry_only")],
        load_node: "node:b",
        tolerance_mm: None,
    };
    assert_refused(&model, INVALID, "reason standalone_body");
}

#[test]
fn a_run_member_shorter_than_the_rounding_allowance_gives_no_direction() {
    let mut model = cantilever(-0.05, Some([0.0, -50.0, 0.0]));
    model.nodes[0].1 = [0.9985, 0.0, 0.0]; // P1 is 1.5 mm long, below √3 mm
    model.nodes[3].1 = [1.1215, -0.05, 0.0]; // and so is the downstream pipe
    assert_refused(&model, INVALID, "reason unresolvable_run_evidence");
}

/// P6: `not_solver_consumed` is honoured: the offset is not read, no offset
/// diagnostic is stated, and the solve is the one without the slot.
#[test]
fn not_solver_consumed_is_honoured() {
    let mut honoured = cantilever(-0.05, None);
    honoured.components = vec![reducer("component:r", "node:a", "body:a-b", Some([0.0, 50.0, 0.0]), "not_solver_consumed")];
    let mut plain = cantilever(-0.05, None);
    plain.components = vec![reducer("component:r", "node:a", "body:a-b", None, "not_solver_consumed")];
    for route in [Route::Preview, Route::Exact] {
        let (a, b) = (solve(&honoured, route), solve(&plain, route));
        assert_eq!(a.status.mechanics, "MECHANICS_SOLVED", "{route:?}");
        assert!(offset_codes(&a).is_empty(), "{route:?}");
        let values = |out: &MechanicsEnvelope| out.results.iter().map(|r| r.value.to_bits()).collect::<Vec<_>>();
        assert_eq!(values(&a), values(&b), "{route:?}");
    }
}

/// An absent offset is not checked and states nothing.
#[test]
fn an_absent_offset_is_not_checked() {
    for route in [Route::Preview, Route::Exact] {
        let out = solve(&cantilever(-0.05, None), route);
        assert_eq!(out.status.mechanics, "MECHANICS_SOLVED", "{route:?}");
        assert!(offset_codes(&out).is_empty(), "{route:?}");
    }
}
