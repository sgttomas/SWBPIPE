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
const AXIS_ALLOWANCE_USED: &str = "RIGID_COMPONENT_ECCENTRIC_OFFSET_AXIS_ALLOWANCE_USED";

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

// ------------------------------------------------------------------ Codex numerical scrutiny r1 (P2)
fn v_unit(v: [f64; 3]) -> [f64; 3] {
    let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / n, v[1] / n, v[2] / n]
}

/// The arc's tangent at its `to` node for chord `c` (from the origin), radius r and
/// plane reference y: cos h·ĉ − sin h·n̂ (the arc bows toward +n̂).
fn to_tangent(c: [f64; 3], r: f64, y: [f64; 3]) -> [f64; 3] {
    let len = (c[0] * c[0] + c[1] * c[1] + c[2] * c[2]).sqrt();
    let u = v_unit(c);
    let a = y[0] * u[0] + y[1] * u[1] + y[2] * u[2];
    let n = v_unit([y[0] - a * u[0], y[1] - a * u[1], y[2] - a * u[2]]);
    let h = (len / (2.0 * r)).asin();
    [h.cos() * u[0] - h.sin() * n[0], h.cos() * u[1] - h.sin() * n[1], h.cos() * u[2] - h.sin() * n[2]]
}

/// Codex's probe: a realized arc from the anchored origin to the entered chord end
/// P1, the reducer at P1 with body D = 0.04·t + e along the *true* tangent t (from
/// the true chord), and a 1 m downstream run along t. Every entered coordinate is
/// within 0.5 mm of the true geometry, so the run directions agree within rounding.
fn probe_request(entered: [f64; 3], true_chord: [f64; 3], r: f64, y: [f64; 3], e_mm: [f64; 3], route: Route) -> Value {
    let t = to_tangent(true_chord, r, y);
    let f = [entered[0] + 0.04 * t[0] + e_mm[0] * 1e-3, entered[1] + 0.04 * t[1] + e_mm[1] * 1e-3, entered[2] + 0.04 * t[2] + e_mm[2] * 1e-3];
    let g = [f[0] + t[0], f[1] + t[1], f[2] + t[2]];
    let m = Model {
        length_unit: "m",
        nodes: vec![("node:p0", [0.0, 0.0, 0.0]), ("node:p1", entered), ("node:f", f), ("node:g", g)],
        members: vec![("pipe:arc", "node:p0", "node:p1"), ("body:p1-f", "node:p1", "node:f"), ("pipe:f-g", "node:f", "node:g")],
        components: vec![
            reducer("component:r", "node:p1", "body:p1-f", Some(e_mm), "mechanics_geometry_only"),
            json!({"id": "component:bend", "kind": "bend", "node": "node:p0",
                "geometry": {"bend_pipe_ref": "pipe:arc", "bend_radius": qv(r, "m"), "bend_geometry_source_reference": "invented"},
                "modifiers": {"flexibility_factor_user_value": qv(1.0, "none"), "source_reference": "invented"},
                "mechanics_interface": {"solver_consumption": "curved_bend_macro_element"}, "provenance": PROV}),
        ],
        load_node: "node:g",
        tolerance_mm: None,
    };
    let mut wire = request(&m, route);
    wire["model"]["pipe_segments"][0]["y_reference"] = json!({"x": y[0], "y": y[1], "z": y[2]});
    for i in [1, 2] {
        // Straight members: any reference not parallel to the member.
        let axis = v_unit(if i == 1 { [f[0] - entered[0], f[1] - entered[1], f[2] - entered[2]] } else { t });
        let reference = if axis[2].abs() < 0.9 { [0.0, 0.0, 1.0] } else { [1.0, 0.0, 0.0] };
        wire["model"]["pipe_segments"][i]["y_reference"] = json!({"x": reference[0], "y": reference[1], "z": reference[2]});
    }
    wire
}

/// The probe outcome: never a false `run_directions_disagree`. A bend whose
/// rounding leaves its tangent unresolved is refused by name; otherwise it solves.
fn assert_probe(wire: Value, route: Route, reason: &str) {
    let out = run_linear_static_preview_value_with_mode(wire, PreviewSolverMode::SparseInteractive).unwrap();
    let found = codes(&out, "RIGID_COMPONENT_ECCENTRIC_OFFSET");
    assert!(!found.iter().any(|(_, m)| m.contains("run_directions_disagree")), "{route:?}: a false disagreement: {found:?}");
    assert!(found.iter().any(|(c, m)| c == INVALID && m.contains("reason unresolvable_run_evidence") && m.contains(reason)),
        "{route:?}: expected the named unresolved bend ({reason}), got {found:?}");
}

#[test]
fn codex_probe_one_a_nearly_parallel_plane_reference_is_refused_by_name() {
    let r = 1.0 / (2.0 * 0.1f64.sin());
    let y = [1.0, 0.00075, 0.00075];
    for route in [Route::Preview, Route::Exact] {
        assert_probe(probe_request([1.0, 0.0, 0.0], [1.0, 0.001, 0.001], r, y, [0.0, -7.0, 7.0], route), route, "plane reference is too nearly parallel");
    }
}

#[test]
fn codex_probe_two_a_near_semicircular_bend_is_refused_by_name() {
    let d = v_unit([1.0, 1.0, 1.0]);
    let entered = 0.2 - 3f64.sqrt() * 0.001;
    for route in [Route::Preview, Route::Exact] {
        assert_probe(probe_request([d[0] * entered, d[1] * entered, d[2] * entered], [d[0] * 0.19999, d[1] * 0.19999, d[2] * 0.19999], 0.1,
            [0.0, 0.0, 1.0], [7.0, -7.0, 0.0], route), route, "admits a semicircular arc");
    }
}

/// J2 regression, offset check only (Codex numerical scrutiny r2): a resolved but
/// weak realized bend (β ≈ 0.4158) at one end and a 1 m run (β ≈ 0.0017) at the
/// other. With the equal-weight consensus the weak bend tilted the axis, and the
/// placement check refused a consistent offset (`..._MISMATCH`, 8.086 mm
/// residual). The better-resolved end now governs, and the check accepts the
/// offset on both routes. This geometry cannot show a full solve: the arc's
/// `y_reference` lies within 1.9e-4 of its chord, so the frame kernel refuses the
/// member's local axes (`PIPE_ELEMENT_INPUT_INVALID`). That is a separate check,
/// independent of W-15c (the same refusal with `eccentric_offset` removed), so
/// it is asserted here as the only blocking code. The full solve is
/// `a_weak_bend_does_not_tilt_the_axis_full_solve`; the check's own residual
/// (about 2.2e-15 m) is asserted in the unit test
/// `w15c_eccentric_offset_tests::j2_weak_bend_case_is_realized`.
#[test]
fn a_weak_bend_does_not_tilt_the_axis_offset_check_only() {
    let entered = [-18.19787406218549, 13.058496225274045, 3.349962055095552];
    let true_chord = [entered[0] + 0.001, entered[1] + 0.001, entered[2] + 0.001];
    let y = [-0.8035474210765018, 0.5765244818323195, 0.1480914039682203];
    let e = [0.00819795250341938, -0.005298081948721869, -0.002173454029030151];
    let e_mm = [e[0] * 1000.0, e[1] * 1000.0, e[2] * 1000.0];
    for route in [Route::Preview, Route::Exact] {
        let out = run_linear_static_preview_value_with_mode(probe_request(entered, true_chord, 11.36337291391957, y, e_mm, route),
            PreviewSolverMode::SparseInteractive).unwrap();
        assert!(offset_codes(&out).is_empty(), "{route:?}: the offset check refuses nothing: {:?}", codes(&out, "RIGID_COMPONENT_ECCENTRIC"));
        let blocking: Vec<&str> = out.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| d.code.as_str()).collect();
        assert_eq!(blocking, vec!["PIPE_ELEMENT_INPUT_INVALID"], "{route:?}: only the arc frame's own refusal remains");
    }
}

/// J2 regression through the full solve (Codex numerical scrutiny r3; invented
/// geometry, not engineering data). A 0.15 m radius arc P0 → P1 with
/// `y_reference` (0, 1, 0), entered chord (0.298, 0, 0) m against a true chord
/// (0.299, 0, 0) m, so its far-end tangent has β ≈ 0.08628. The reducer at P1
/// (100 mm OD / 5 mm wall upstream, 80 mm OD / 4 mm wall downstream) has body
/// P1 → F = 0.4·t + e along the true tangent t, declared offset e = (0, 0, 10)
/// mm, and a 1 m straight run F → G along t (β ≈ 0.001732). The run governs,
/// the offset is realized and the model solves on both routes and both solver
/// modes. The equal-weight consensus refused it (`..._MISMATCH`, residual
/// 6.772065 mm).
#[test]
fn a_weak_bend_does_not_tilt_the_axis_full_solve() {
    let m = Model {
        length_unit: "m",
        nodes: vec![("node:p0", [0.0, 0.0, 0.0]), ("node:p1", [0.298, 0.0, 0.0]),
            ("node:f", [0.3306326353347209, -0.3986666666666667, 0.01]), ("node:g", [0.41221422367152327, -1.3953333333333333, 0.01])],
        members: vec![("pipe:arc", "node:p0", "node:p1"), ("body:p1-f", "node:p1", "node:f"), ("pipe:f-g", "node:f", "node:g")],
        components: vec![
            reducer("component:r", "node:p1", "body:p1-f", Some([0.0, 0.0, 10.0]), "mechanics_geometry_only"),
            json!({"id": "component:bend", "kind": "bend", "node": "node:p0",
                "geometry": {"bend_pipe_ref": "pipe:arc", "bend_radius": qv(0.15, "m"), "bend_geometry_source_reference": "invented"},
                "modifiers": {"flexibility_factor_user_value": qv(1.0, "none"), "source_reference": "invented"},
                "mechanics_interface": {"solver_consumption": "curved_bend_macro_element"}, "provenance": PROV}),
        ],
        load_node: "node:g",
        tolerance_mm: None,
    };
    for route in [Route::Preview, Route::Exact] {
        let mut wire = request(&m, route);
        wire["model"]["pipe_segments"][0]["y_reference"] = json!({"x": 0.0, "y": 1.0, "z": 0.0});
        for (i, (od, wall)) in [(0.1, 0.005), (0.08, 0.004), (0.08, 0.004)].into_iter().enumerate() {
            wire["model"]["pipe_segments"][i]["section"]["outside_diameter"] = qv(od, "m");
            wire["model"]["pipe_segments"][i]["section"]["wall_thickness"] = qv(wall, "m");
        }
        wire["model"]["components"][0]["geometry"]["rigid_body_length"] = qv(0.4, "m");
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let out = run_linear_static_preview_value_with_mode(wire.clone(), mode).unwrap();
            let blocking: Vec<String> = out.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| format!("{}: {}", d.code, d.message)).collect();
            assert!(blocking.is_empty(), "{route:?} {mode:?}: no blocking code: {blocking:?}");
            assert_eq!(out.status.mechanics, "MECHANICS_SOLVED", "{route:?} {mode:?}");
            assert_eq!(offset_codes(&out), vec![REALIZED.to_string()], "{route:?} {mode:?}");
            assert!(out.diagnostics.iter().any(|d| d.code == REALIZED && d.affected_refs == vec!["component:r".to_string()]), "{route:?} {mode:?}");
        }
    }
}

/// J3 regression through the solve (Codex numerical scrutiny r3; invented
/// geometry): two 125 mm runs with opposite 0.9765625 mm lateral differences
/// (an exact tie of β), a 750 mm body and a 15.625 mm Z offset. The old
/// consensus axis realized the offset at the tie, while moving the downstream
/// outer node's X by one ulp either way selected one end with a 5.859196 mm
/// placement residual. Under J3 the tie judges each end like both neighbours,
/// on both routes, and placing the reducer at the other end (the ends' names
/// swapped, the offset negated) gives the same outcome and residual. Under rule
/// B (§3.3) the 5.859196 mm residual lies within each end's placement allowance
/// τ + ‖D‖·sin β (β of a 125 mm run, about 10.4 mm of axis allowance), so all
/// of them are accepted with the axis-allowance warning; under the τ-only rule
/// all of them were refused as `..._MISMATCH`.
#[test]
fn a_tie_is_judged_at_each_end_like_its_neighbours() {
    let model = |gx: f64, at_far_end: bool| Model {
        length_unit: "m",
        nodes: vec![("node:n0", [-0.125, -0.00048828125, 0.0]), ("node:n", [0.0, 0.00048828125, 0.0]),
            ("node:f", [0.75, 0.00048828125, 0.015625]), ("node:g", [gx, -0.00048828125, 0.015625])],
        members: vec![("pipe:up", "node:n0", "node:n"), ("body:n-f", "node:n", "node:f"), ("pipe:down", "node:f", "node:g")],
        components: vec![if at_far_end {
            reducer("component:r", "node:f", "body:n-f", Some([0.0, 0.0, -15.625]), "mechanics_geometry_only")
        } else {
            reducer("component:r", "node:n", "body:n-f", Some([0.0, 0.0, 15.625]), "mechanics_geometry_only")
        }],
        load_node: "node:g",
        tolerance_mm: None,
    };
    for route in [Route::Preview, Route::Exact] {
        let mut outcomes = Vec::new();
        for delta in [-1i64, 0, 1] {
            let gx = f64::from_bits((0.875f64.to_bits() as i64 + delta) as u64);
            let (named, swapped) = (solve(&model(gx, false), route), solve(&model(gx, true), route));
            assert_eq!(named.status.mechanics, "MECHANICS_SOLVED", "{route:?} {delta}");
            let found = residuals(&named);
            assert_eq!(found.len(), 2, "{route:?} {delta}: {found:?}");
            assert_eq!(found[0], format!("{REALIZED} -"), "{route:?} {delta}");
            assert!(found[1].starts_with(AXIS_ALLOWANCE_USED) && found[1].contains("5.8591"), "{route:?} {delta}: {found:?}");
            assert_eq!(found, residuals(&swapped), "{route:?} {delta}: the ends' names swapped");
            outcomes.push(found);
        }
        assert!(outcomes[1] == outcomes[0] || outcomes[1] == outcomes[2], "{route:?}: the tie matches a neighbour: {outcomes:?}");
    }
}

/// Each offset diagnostic's code with the residual its message states ("-" when none).
fn residuals(out: &MechanicsEnvelope) -> Vec<String> {
    codes(out, "RIGID_COMPONENT_ECCENTRIC_OFFSET").into_iter()
        .map(|(c, m)| format!("{c} {}", m.split("residual ").nth(1).and_then(|r| r.split(' ').next()).unwrap_or("-"))).collect()
}

/// Rule B through the solve on both routes: solved, the realized info and the
/// axis-allowance warning naming the residual, and results bit-identical to the
/// same model without the declared offset (the node geometry governs).
fn assert_accepted_with_warning(m: &Model, residual_needle: &str) {
    let mut bare = Model { components: m.components.clone(), nodes: m.nodes.clone(), members: m.members.clone(), ..*m };
    bare.components[0]["geometry"].as_object_mut().unwrap().remove("eccentric_offset");
    for route in [Route::Preview, Route::Exact] {
        let out = solve(m, route);
        let blocking: Vec<String> = out.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| format!("{}: {}", d.code, d.message)).collect();
        assert_eq!(out.status.mechanics, "MECHANICS_SOLVED", "{route:?}: {blocking:?}");
        assert_eq!(offset_codes(&out), vec![REALIZED.to_string(), AXIS_ALLOWANCE_USED.to_string()], "{route:?}");
        let warning = out.diagnostics.iter().find(|d| d.code == AXIS_ALLOWANCE_USED).unwrap();
        assert_eq!(warning.severity, "warning", "{route:?}");
        for needle in [residual_needle, "node-proximity tolerance 5e-3 m", "run-axis allowance", "declares eccentric offset", "realize a transverse separation", "the node geometry was used for the solve"] {
            assert!(warning.message.contains(needle), "{route:?}: {needle:?} in {}", warning.message);
        }
        let values = |out: &MechanicsEnvelope| out.results.iter().map(|r| (r.id.clone(), r.value.to_bits())).collect::<Vec<_>>();
        assert_eq!(values(&out), values(&solve(&bare, route)), "{route:?}: the node geometry governs");
    }
}

/// A straight run P0 → P, the reducer at P with body P → F, and a run F → G,
/// all in metres; the offset `e` in mm.
fn three_members(nodes: [[f64; 3]; 4], e: [f64; 3]) -> Model {
    Model {
        length_unit: "m",
        nodes: vec![("node:p0", nodes[0]), ("node:p", nodes[1]), ("node:f", nodes[2]), ("node:g", nodes[3])],
        members: vec![("pipe:up", "node:p0", "node:p"), ("body:p-f", "node:p", "node:f"), ("pipe:down", "node:f", "node:g")],
        components: vec![reducer("component:r", "node:p", "body:p-f", Some(e), "mechanics_geometry_only")],
        load_node: "node:g",
        tolerance_mm: None,
    }
}

/// Rule B regressions (§9; invented geometry, not engineering data): two
/// ordinary eccentric reducers between 0.1 m members, entered to the mm, with
/// the offset equal to the radius difference, both exactly consistent before
/// rounding. Under the τ-only rule both were refused (`..._MISMATCH`, residuals
/// 5.319608879485803 and 5.015477503296432 mm); under rule B both are accepted
/// with the axis-allowance warning.
#[test]
fn rounded_compact_reducers_are_accepted_with_the_axis_allowance_warning() {
    // 400 → 300 mm, 0.4 m body, 30° in plan, 10° slope.
    assert_accepted_with_warning(&three_members(
        [[2.318, 3.973, 1.674], [2.403, 4.023, 1.692], [2.737, 4.215, 1.81], [2.822, 4.265, 1.828]], [-8.0, -4.0, 49.0]), "residual 5.3196");
    // 600 → 400 mm, 0.6 m body, 45° in plan.
    assert_accepted_with_warning(&three_members(
        [[5.018, 2.801, 1.961], [5.088, 2.871, 1.961], [5.442, 3.366, 1.961], [5.512, 3.437, 1.961]], [-71.0, 71.0, 0.0]), "residual 5.0154");
}

/// Rule B at the high end of the band (Codex numerical scrutiny r4): 0.1 m runs
/// and a 0.6 m body at 45° in plan with a 2° slope, whose true body differs
/// from the declared 50 mm offset by 21.7 mm before mm rounding. The residual
/// on the entered nodes lies within the axis allowance, so it is accepted with
/// the warning: rule B gives no certified bound on true placement.
#[test]
fn the_high_end_of_the_axis_allowance_band_is_accepted_with_the_warning() {
    let (a, z) = (45f64.to_radians(), 2f64.to_radians());
    let t = [a.cos() * z.cos(), a.sin() * z.cos(), z.sin()];
    let up = [-a.cos() * z.sin(), -a.sin() * z.sin(), z.cos()];
    let p = [2.0752538118573574, -4.970333308486562, -4.0085098971414315];
    let dev = [-0.013510279544283096, 0.013985911754709911, -0.009631023778643315];
    let far: [f64; 3] = std::array::from_fn(|i| p[i] + 0.6 * t[i] + 0.05 * up[i] + dev[i]);
    let rd = |v: [f64; 3]| v.map(|v| (v * 1000.0).round() / 1000.0);
    let nodes = [rd(std::array::from_fn(|i| p[i] - 0.1 * t[i])), rd(p), rd(far), rd(std::array::from_fn(|i| far[i] + 0.1 * t[i]))];
    assert_accepted_with_warning(&three_members(nodes, up.map(|v| (v * 50.0).round())), "residual ");
}

/// Rule B's boundary from both sides: 0.1 m runs on X (β = asin(√3 mm / 0.1 m))
/// and a 0.6 m body with a declared (0, 0, 50) mm offset against a realized
/// (0, dy, 50) mm. The placement allowance is τ + ‖D‖·sin β ≈ 15.4317 mm: dy =
/// 15.4 mm is accepted with the warning, 15.5 mm is refused as `..._MISMATCH`
/// naming τ, the axis allowance and the residual. A residual below τ is
/// accepted with no warning, as before (also every `assert_accepted` case).
#[test]
fn the_axis_allowance_bound_from_both_sides() {
    let at = |dy: f64| three_members([[-0.1, 0.0, 0.0], [0.0, 0.0, 0.0], [0.6, dy, 0.05], [0.7, dy, 0.05]], [0.0, 0.0, 50.0]);
    assert_accepted_with_warning(&at(0.0154), "residual 1.54");
    assert_refused(&at(0.0155), MISMATCH, "exceeds its placement allowance 1.543");
    assert_refused(&at(0.0155), MISMATCH, "node-proximity tolerance 5e-3 m plus the run-axis allowance 1.043");
    assert_accepted(&at(0.0049), &["component:r"]);
}
