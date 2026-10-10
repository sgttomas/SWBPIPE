//! F1b (T3 D1 revision 5a.2 §4.7, W2 at formation in the product; ROOT's F1b
//! rulings of 2026-09-28): product-level tests of force scaling through both
//! public entries (the captured `run_linear_static_preview_value_with_mode`
//! and the historical typed `run_linear_static_preview_with_mode`) where the
//! captured entry admits the request, in both solver modes.
//!
//! Every input is invented. Expected values are exact references rounded once
//! (computed with exact rational arithmetic from the product's own binary64
//! section values) or exact power-of-two relations; the predicate is the
//! unchanged `|obs - exp| <= 1e-9 * |exp|`. The b values and texts are the
//! ones F1b's checkpoint-A2 product runs recorded.
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, run_linear_static_preview_with_mode, Diagnostic,
    LinearStaticPreviewRequest, MechanicsEnvelope, PreviewSolverMode,
};
use serde_json::{json, Value};

const PROV: &str = "invented_t3_f1b_w2_runtime_input_no_library_data";
const ALL: [&str; 6] = ["UX", "UY", "UZ", "RX", "RY", "RZ"];
const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];

fn pow2(exponent: i32) -> f64 {
    assert!((-1022..=1023).contains(&exponent));
    f64::from_bits(((exponent + 1023) as u64) << 52)
}

// ------------------------------------------------------------------ models

fn nodal(id: &str, node: &str, dof: &str, value: f64) -> Value {
    if dof.starts_with('R') {
        json!({"id": id, "category": "concentrated_moment", "target": {"type": "node", "node": node},
               "direction": dof, "magnitude": {"value": value, "unit": "N*m"}, "dimension": "moment", "provenance": PROV})
    } else {
        json!({"id": id, "category": "concentrated_force", "target": {"type": "node", "node": node},
               "direction": format!("global_{}", dof[1..].to_lowercase()),
               "magnitude": {"value": value, "unit": "N"}, "dimension": "force", "provenance": PROV})
    }
}

fn anchor(node: &str, restraints: &[&str]) -> Value {
    json!({"id": format!("anchor:{node}"), "node": node, "family": "anchor", "restraints": restraints,
           "provenance": PROV})
}

/// A straight-frame model: nodes, members (id, from, to), one section and one
/// material for every member, supports and one case of loads.
#[allow(clippy::too_many_arguments)]
fn frame(
    id: &str,
    nodes: &[(&str, [f64; 3])],
    members: &[(&str, &str, &str)],
    (od, wall): (f64, f64),
    (e, g): (f64, f64),
    supports: Vec<Value>,
    loads: Vec<Value>,
) -> Value {
    json!({"model": {
        "schema_version": "0.1.0", "document_kind": "openpipestress.product_preview.model",
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
                            "rule_check": "not_performed_user_rule_inputs_missing",
                            "professional_acceptance": "not_provided"},
        "project": {"id": format!("invented:t3-f1b-w2:{id}"),
                    "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa",
                              "temperature": "degC", "stress": "Pa"}},
        "nodes": nodes.iter().map(|(n, p)| json!({"id": n, "position": {"x": p[0], "y": p[1], "z": p[2]}, "provenance": PROV})).collect::<Vec<_>>(),
        "pipe_segments": members.iter().map(|(m, a, b)| json!({"id": m, "from": a, "to": b, "material": "mat:W2",
            "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0},
            "section": {"outside_diameter": {"value": od, "unit": "m"}, "wall_thickness": {"value": wall, "unit": "m"}},
            "provenance": PROV})).collect::<Vec<_>>(),
        "materials": [{"id": "mat:W2", "elastic_modulus": {"value": e, "unit": "Pa"},
                       "shear_modulus": {"value": g, "unit": "Pa"}, "provenance": PROV}],
        "supports": supports,
        "load_cases": [{"id": "case", "label": id, "kind": "primitive_user_load",
                        "primitive_loads": loads, "provenance": PROV}],
        "combinations": []
    }, "materials": []})
}

/// K2a's product-reach shape: one member N0 -> N1 along x; N0 anchored; N1
/// anchored in every DOF but `free`, which carries the load and, when given, a
/// ground spring.
#[allow(clippy::too_many_arguments)]
fn reach(
    length: f64,
    (od, wall): (f64, f64),
    (e, g): (f64, f64),
    free: &str,
    spring: Option<f64>,
    loads: Vec<Value>,
) -> Value {
    let restrained: Vec<&str> = ALL.iter().copied().filter(|d| *d != free).collect();
    let mut supports = vec![anchor("N0", &ALL), anchor("N1", &restrained)];
    if let Some(k) = spring {
        supports.push(
            json!({"id": "spring:N1:0", "node": "N1", "family": "spring", "restraints": [free],
            "stiffness": {"dof": free, "value": {"value": k, "unit": "N/m"}}, "provenance": PROV}),
        );
    }
    frame(
        "reach",
        &[("N0", [0.0; 3]), ("N1", [length, 0.0, 0.0])],
        &[("M1", "N0", "N1")],
        (od, wall),
        (e, g),
        supports,
        loads,
    )
}

/// K2a's exact-zero member (b = 734), with its own loads and optional spring.
fn exact_zero(spring: Option<f64>, loads: Vec<Value>) -> Value {
    reach(
        1.8189894035458565e-12,
        (1.0e-11, 1.0e-12),
        (6.4e-280, 1.0e-100),
        "UY",
        spring,
        loads,
    )
}

/// PHYS-R4 (`tests/pressure_membrane_range.rs` `fixture`, with its inputs):
/// OD 4e-77 m, wall 1e-77 m, L 1 m, E 1 Pa, nu 0.1. `pressurized`: both ends
/// fixed and p = 4.7e-170 Pa (the public fixture); otherwise a cantilever
/// with tip Fy = tip Mx = f64 bits 0x0031fa182c40c60d (about 1e-307).
fn phys_r4(pressurized: bool) -> Value {
    let (a, b) = ("node:section-a", "node:section-b");
    let fixed = |id: &str, node: &str| {
        json!({"id": id, "node": node, "family": "anchor", "restraints": ALL,
               "provenance": "independent_section_geometry_control"})
    };
    let mut supports = vec![fixed("support:section-a", a)];
    let mut loads = Vec::new();
    let mut regions = Vec::new();
    if pressurized {
        supports.push(fixed("support:section-b", b));
        regions.push(json!({"id": "region:source-section", "member_pipe_ids": ["pipe:source-section"],
            "pressure_basis": "internal_differential_zero_external_v1",
            "pressure": {"value": 4.7e-170, "unit": "Pa"},
            "terminals": [
                {"node_ref": a, "closure_transfer": "transfers_to_wall", "provenance": "explicit_test_closure"},
                {"node_ref": b, "closure_transfer": "transfers_to_wall", "provenance": "explicit_test_closure"}],
            "provenance": "independent_geometry_pressure_reference"}));
    } else {
        let tip = f64::from_bits(0x0031fa182c40c60d);
        loads.push(json!({"id": "load:tip-y", "category": "concentrated_force",
            "target": {"type": "node", "node": b}, "direction": "global_y", "dimension": "force",
            "magnitude": {"value": tip, "unit": "N"}, "provenance": PROV}));
        loads.push(
            json!({"id": "load:tip-torque", "category": "concentrated_moment",
            "target": {"type": "node", "node": b}, "direction": "rotation_x", "dimension": "moment",
            "magnitude": {"value": tip, "unit": "N*m"}, "provenance": PROV}),
        );
    }
    json!({"model": {
        "schema_version": "0.3.0", "document_kind": "openpipestress.product_preview.model",
        "pressure_contract": {"version": "2.0.0", "mode": "exact_straight_pressure_v2"},
        "project": {"id": "project:section-oracle", "units": {"length": "m", "force": "N", "angle": "rad",
            "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
            "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": [{"id": a, "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": PROV},
            {"id": b, "position": {"x": 1.0, "y": 0.0, "z": 0.0}, "provenance": PROV}],
        "pipe_segments": [{"id": "pipe:source-section", "from": a, "to": b, "section": {
            "outside_diameter": {"value": 4e-77, "unit": "m"},
            "wall_thickness": {"value": 1e-77, "unit": "m"}},
            "material": "material:section", "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0},
            "provenance": PROV}],
        "materials": [{"id": "material:section", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
            "elastic_modulus": {"value": 1.0, "unit": "Pa"},
            "poisson_ratio": {"value": 0.1, "unit": "1"}, "provenance": PROV}],
        "supports": supports, "components": [],
        "load_cases": [{"id": "case:source-section", "primitive_loads": loads, "pressure_regions": regions,
            "provenance": PROV}], "combinations": []
    }, "materials": []})
}

/// A skew chain of five 1 m members (x, 0.125x, -0.0625x), anchored at N0,
/// with a tip force in UY and a tip moment about X, scaled by exact powers of
/// two (the RF-RANGE LEF-large construction): lengths and section dimensions
/// by 2^pl, moduli by 2^pm, forces by 2^pf, moments by 2^(pf+pl).
fn scaled_chain(pl: i32, pm: i32, pf: i32) -> Value {
    let s = pow2(pl);
    let names: Vec<String> = (0..=5).map(|i| format!("N{i}")).collect();
    let nodes: Vec<(&str, [f64; 3])> = names
        .iter()
        .enumerate()
        .map(|(i, n)| {
            let x = i as f64;
            (n.as_str(), [x * s, 0.125 * x * s, -0.0625 * x * s])
        })
        .collect();
    let members: Vec<String> = (1..=5).map(|i| format!("M{i}")).collect();
    let pairs: Vec<(&str, &str, &str)> = members
        .iter()
        .enumerate()
        .map(|(i, m)| (m.as_str(), names[i].as_str(), names[i + 1].as_str()))
        .collect();
    frame(
        "lef",
        &nodes,
        &pairs,
        (0.2 * s, 0.01 * s),
        (2.0e11 * pow2(pm), 8.0e10 * pow2(pm)),
        vec![anchor("N0", &ALL)],
        vec![
            nodal("load:tip-y", "N5", "UY", 1250.0 * pow2(pf)),
            nodal("load:tip-rx", "N5", "RX", 40.0 * pow2(pf + pl)),
        ],
    )
}

/// K2b's documented-limitation chain (K2b RETURN §13.3) at product level:
/// N0-N1-N2 along x, 1 m bars, OD 0.2 m, wall 0.01 m, E = 2^440 Pa,
/// G = 2^439 Pa, UX loads of 2^-1010 N at N1 and 1 N at N2; N1 and N2 are free
/// in UX only.
fn limitation_chain() -> Value {
    let free_ux = ["UY", "UZ", "RX", "RY", "RZ"];
    frame(
        "limitation-chain",
        &[
            ("N0", [0.0; 3]),
            ("N1", [1.0, 0.0, 0.0]),
            ("N2", [2.0, 0.0, 0.0]),
        ],
        &[("M1", "N0", "N1"), ("M2", "N1", "N2")],
        (0.2, 0.01),
        (pow2(440), pow2(439)),
        vec![
            anchor("N0", &ALL),
            anchor("N1", &free_ux),
            anchor("N2", &free_ux),
        ],
        vec![
            nodal("load:n1", "N1", "UX", pow2(-1010)),
            nodal("load:n2", "N2", "UX", 1.0),
        ],
    )
}

/// A 2^20 m cantilever along x, E = 2^83 Pa, G = 2^82 Pa, with a 2^1010 N tip
/// load in UY: the root moment, about 2^1030 N*m, lies beyond binary64. The
/// root is anchored (a reaction) or, with `spring`, held in RZ by a 2^120
/// N*m/rad ground spring instead (a spring action).
fn overflowing_root(spring: bool) -> Value {
    let mut supports = if spring {
        vec![anchor("N0", &["UX", "UY", "UZ", "RX", "RY"])]
    } else {
        vec![anchor("N0", &ALL)]
    };
    if spring {
        supports.push(json!({"id": "spring:N0:RZ", "node": "N0", "family": "spring", "restraints": ["RZ"],
            "stiffness": {"dof": "RZ", "value": {"value": pow2(120), "unit": "N*m/rad"}}, "provenance": PROV}));
    }
    frame(
        "overflowing-root",
        &[("N0", [0.0; 3]), ("N1", [pow2(20), 0.0, 0.0])],
        &[("M1", "N0", "N1")],
        (0.2, 0.01),
        (pow2(83), pow2(82)),
        supports,
        vec![nodal("load:y", "N1", "UY", pow2(1010))],
    )
}

/// An axial-only cantilever whose derived second moment is subnormal: OD 1e-77
/// m, wall 2.5e-78 m (I about 4.6e-311 m^4), E = 1e300 Pa, N1 free in UX only.
fn subnormal_second_moment(load: f64) -> Value {
    frame(
        "subnormal-i",
        &[("N0", [0.0; 3]), ("N1", [1.0, 0.0, 0.0])],
        &[("M1", "N0", "N1")],
        (1e-77, 2.5e-78),
        (1e300, 4e299),
        vec![
            anchor("N0", &ALL),
            anchor("N1", &["UY", "UZ", "RX", "RY", "RZ"]),
        ],
        vec![nodal("load:x", "N1", "UX", load)],
    )
}

// --------------------------------------------------- the family models (T0R)

/// The T0R shape (`tests/preview_physics_runtime.rs` `base`): a material with a
/// thermal expansion coefficient, moduli times `modulus_scale`.
fn t0r(
    nodes: Value,
    pipes: Value,
    supports: Value,
    components: Value,
    loads: Value,
    modulus_scale: f64,
) -> Value {
    json!({"model": {"schema_version": "0.2.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:t3-f1b-w2", "units": {"length": "m", "force": "N", "angle": "rad",
            "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
            "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": nodes, "pipe_segments": pipes, "supports": supports, "components": components,
        "materials": [{"id": "material:m", "elastic_modulus": {"value": 200e9 * modulus_scale, "unit": "Pa"},
            "shear_modulus": {"value": 76.923076923e9 * modulus_scale, "unit": "Pa"},
            "thermal_expansion_coefficient": {"value": 1.2e-5, "unit": "1/degC"},
            "provenance": PROV}],
        "load_cases": [{"id": "case:f", "primitive_loads": loads, "provenance": PROV}],
        "combinations": []}, "materials": []})
}

fn tnode(id: &str, x: f64, y: f64) -> Value {
    json!({"id": id, "position": {"x": x, "y": y, "z": 0.0}, "provenance": PROV})
}

fn tpipe(id: &str, from: &str, to: &str, yref: [f64; 3]) -> Value {
    json!({"id": id, "from": from, "to": to, "section": {"outside_diameter": {"value": 0.12, "unit": "m"},
        "wall_thickness": {"value": 0.01, "unit": "m"}}, "material": "material:m",
        "y_reference": {"x": yref[0], "y": yref[1], "z": yref[2]}, "provenance": PROV})
}

fn tforce(node: &str, direction: &str, value: f64) -> Value {
    json!({"id": "f", "category": "concentrated_force", "target": {"type": "node", "node": node},
        "direction": direction, "magnitude": {"value": value, "unit": "N"}, "dimension": "force", "provenance": PROV})
}

/// A 1 m cantilever a-b (T0R's C1) carrying one admission family, with moduli
/// times `modulus_scale` (2^-1000: a range case; 1: its b = 0 twin).
fn cantilever_family(family: &str, modulus_scale: f64) -> Value {
    let nodes = json!([tnode("node:a", 0.0, 0.0), tnode("node:b", 1.0, 0.0)]);
    let pipes = json!([tpipe("pipe:a-b", "node:a", "node:b", [0.0, 1.0, 0.0])]);
    let mut supports = vec![anchor("node:a", &ALL)];
    let mut loads = vec![tforce("node:b", "global_y", 100.0)];
    match family {
        "uniform_element_load" => loads.push(json!({"id": "w", "category": "distributed_force",
            "target": {"type": "element", "pipe": "pipe:a-b"}, "direction": "global_z",
            "magnitude": {"value": -50.0, "unit": "N/m"}, "dimension": "force_per_length", "provenance": PROV})),
        "thermal_or_eigen_load" => {
            supports.push(anchor("node:b", &["UX"]));
            loads.push(json!({"id": "t", "category": "thermal", "target": {"type": "element", "pipe": "pipe:a-b"},
                "direction": "global_z", "magnitude": {"value": 12.5, "unit": "degC"},
                "dimension": "temperature_interval", "provenance": PROV}));
        }
        "constant_effort_support" => supports.push(json!({"id": "support:ce", "node": "node:b",
            "family": "constant_effort_support", "restraints": ["UY"],
            "hanger": {"hanger_type": "constant_effort_support", "constant_load": {"value": 375.0, "unit": "N"},
                       "travel_range": {"value": 0.05, "unit": "m"}, "source_reference": "invented"},
            "provenance": PROV})),
        other => panic!("{other}"),
    }
    t0r(
        nodes,
        pipes,
        json!(supports),
        json!([]),
        json!(loads),
        modulus_scale,
    )
}

/// T0R's arc model (`arc_model`, k = 1) with its z load times `load_scale`.
fn curved_bend(load_scale: f64) -> Value {
    let bend = json!({"id": "component:bend", "label": "invented arc", "kind": "bend", "node": "node:c",
        "geometry": {"bend_pipe_ref": "pipe:b-c", "bend_radius": {"value": 0.2, "unit": "m"},
                     "bend_angle": {"value": std::f64::consts::FRAC_PI_2, "unit": "rad"},
                     "bend_plane_orientation": "global_xy_preview", "bend_geometry_source_reference": "invented"},
        "modifiers": {"flexibility_factor_user_value": {"value": 1.0, "unit": "none"}, "source_reference": "invented"},
        "mechanics_interface": {"solver_consumption": "curved_bend_macro_element",
                                "rule_check_consumption": "user_rule_pack_inputs_only"},
        "provenance": PROV});
    t0r(
        json!([
            tnode("node:a", 0.0, 0.0),
            tnode("node:b", 1.0, 0.0),
            tnode("node:c", 1.2, 0.2),
            tnode("node:d", 1.2, 1.2)
        ]),
        json!([
            tpipe("pipe:a-b", "node:a", "node:b", [0.0, 1.0, 0.0]),
            tpipe("pipe:b-c", "node:b", "node:c", [0.0, 0.0, 1.0]),
            tpipe("pipe:c-d", "node:c", "node:d", [1.0, 0.0, 0.0])
        ]),
        json!([anchor("node:a", &ALL)]),
        json!([bend]),
        json!([tforce("node:d", "global_z", 1000.0 * load_scale)]),
        1.0,
    )
}

/// PHYS-R4's exact-pressure shape with a pressure whose end-cap operand is
/// normal (`p`, section and moduli as given; both ends fixed).
fn exact_pressure(p: f64, (od, wall): (f64, f64), (e, nu): (f64, f64)) -> Value {
    let mut request = phys_r4(true);
    let model = &mut request["model"];
    model["load_cases"][0]["pressure_regions"][0]["pressure"]["value"] = json!(p);
    model["pipe_segments"][0]["section"] = json!({"outside_diameter": {"value": od, "unit": "m"},
        "wall_thickness": {"value": wall, "unit": "m"}});
    model["materials"][0]["elastic_modulus"]["value"] = json!(e);
    model["materials"][0]["poisson_ratio"]["value"] = json!(nu);
    request
}

/// P1's NP-A-N05 (a soft 1e-4 N*m/rad spring at the root's RX; exact-block
/// qualifies and selects its Sensitive case) with a second case, B, on a
/// temperature point whose E is scaled by 2^-1050 (a formation-range basis; G
/// kept), carrying the same 1e-8 N*m torque.
fn mixed_invocation() -> Value {
    let mut request = frame(
        "mixed",
        &[("root", [0.0; 3]), ("tip", [2.0, 0.0, 0.0])],
        &[("pipe", "root", "tip")],
        (0.2, 0.01),
        (2.0e11, 8.0e10),
        vec![
            anchor("root", &["UX", "UY", "UZ", "RY", "RZ"]),
            json!({"id": "soft", "node": "root", "family": "spring", "restraints": ["RX"],
                   "stiffness": {"dof": "RX", "value": {"value": 0.0001, "unit": "N*m/rad"}}, "provenance": PROV}),
        ],
        vec![],
    );
    let model = &mut request["model"];
    model["load_cases"] = json!([
        {"id": "case:A", "label": "A", "kind": "primitive_user_load",
         "primitive_loads": [nodal("torque:a", "tip", "RX", 1e-8)], "provenance": PROV},
        {"id": "case:B", "label": "B", "kind": "primitive_user_load",
         "modulus_basis_ref": "temperature-point:tiny",
         "primitive_loads": [nodal("torque:b", "tip", "RX", 1e-8)], "provenance": PROV}
    ]);
    model["materials"][0]["temperature_points"] = json!([{"id": "temperature-point:tiny",
        "elastic_modulus": {"value": 2.0e11 * pow2(-1000) * pow2(-50), "unit": "Pa"},
        "shear_modulus": {"value": 8.0e10, "unit": "Pa"}, "provenance": PROV}]);
    request
}

// ------------------------------------------------------------------ checks

#[derive(Clone, Copy, PartialEq, Debug)]
enum Entries {
    Both,
    /// The captured entry refuses an integral binary64 value above 2^53 - 1.
    TypedOnly,
}

fn envelopes(
    request: &Value,
    mode: PreviewSolverMode,
    entries: Entries,
) -> Vec<(&'static str, MechanicsEnvelope)> {
    let typed_request: LinearStaticPreviewRequest =
        serde_json::from_value(request.clone()).unwrap();
    let typed = (
        "typed",
        run_linear_static_preview_with_mode(typed_request, mode),
    );
    let captured = run_linear_static_preview_value_with_mode(request.clone(), mode);
    match entries {
        Entries::Both => vec![("captured", captured.unwrap()), typed],
        Entries::TypedOnly => {
            let error = captured
                .err()
                .expect("the captured entry refuses the request");
            assert!(
                error.contains("CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT"),
                "{error}"
            );
            vec![typed]
        }
    }
}

fn integrity<'a>(envelope: &'a MechanicsEnvelope, case: &str) -> &'a Diagnostic {
    let id = format!("diagnostic:numerical-integrity:{case}");
    let found: Vec<_> = envelope.diagnostics.iter().filter(|d| d.id == id).collect();
    assert_eq!(found.len(), 1, "{:?}", envelope.diagnostics);
    found[0]
}

/// W2 refused the invocation's only case: no result, one blocking diagnostic
/// (the case's integrity record, `code`), whose message begins with `prefix`
/// (everything before `global_dof_map=`).
fn assert_w2_refusal(
    request: &Value,
    entries: Entries,
    modes: &[PreviewSolverMode],
    case: &str,
    code: &str,
    prefix: &str,
) {
    for &mode in modes {
        for (entry, envelope) in envelopes(request, mode, entries) {
            let ctx = format!("{entry} {mode:?}");
            assert!(envelope.results.is_empty(), "{ctx}");
            assert_eq!(envelope.status.mechanics, "MODEL_INCOMPLETE", "{ctx}");
            let blocking: Vec<_> = envelope
                .diagnostics
                .iter()
                .filter(|d| d.severity == "blocking")
                .collect();
            assert_eq!(blocking.len(), 1, "{ctx}: {:?}", envelope.diagnostics);
            let record = integrity(&envelope, case);
            assert_eq!(record.severity, "blocking", "{ctx}");
            assert_eq!(record.code, code, "{ctx}");
            assert_eq!(record.affected_refs, vec![case.to_string()], "{ctx}");
            assert!(
                record
                    .message
                    .starts_with(&format!("Load case {case}: {prefix}; global_dof_map=")),
                "{ctx}: {}",
                record.message
            );
        }
    }
}

/// The published row of `kind` at `entity`.
fn row(envelope: &MechanicsEnvelope, entity: &str, kind: &str) -> f64 {
    let rows: Vec<_> = envelope
        .results
        .iter()
        .filter(|r| r.entity_ref == entity && r.kind == kind)
        .collect();
    assert_eq!(rows.len(), 1, "{entity} {kind}");
    rows[0].value
}

fn within(observed: f64, expected: f64) -> bool {
    (observed - expected).abs() <= 1e-9 * expected.abs()
}

// ------------------------------------------------------------------ tests

/// Scope §6, PHYS-R4 (ROOT Q10 as approved): the public fixture is not solved
/// with K2b's census. Its exact-pressure end-cap operand is subnormal at
/// formation, so the census refuses it, by name, with M03's step-1 trigger and
/// no b. Both entries (F1b adds the captured one) and both modes.
#[test]
fn f1b_w2_phys_r4_fixture_is_refused_by_the_census() {
    assert_w2_refusal(
        &phys_r4(true),
        Entries::Both,
        &MODES,
        "case:source-section",
        "NUMERICAL_INTEGRITY_UNRESOLVED",
        "range: subnormal stiffness or load at formation; range_scaling: attempted; step1_trigger=Evaluation(Range(\"arithmetic outside normal range\")); force_scale_exponent=none",
    );
}

/// Scope §6, PHYS-R4 without pressure: published at b = 536 on both entries
/// and in both modes. u_y, theta_z and theta_x at the tip are within 1e-9 of
/// the exact cantilever values from the product's binary64 section
/// (Iz = 1.178097245096172e-307 m^4, J = 2.356194490192344e-307 m^4,
/// G = 0.45454545454545453 Pa), each rounded once: F/(3 E Iz), F/(2 E Iz) and
/// M/(G J) (L = 1 m, E = 1 Pa). u_x is 0 (displacements are never scaled).
/// The step-1 trigger is M03's evaluation, so formation at b = 0 is normal
/// and R-b' computes its bound: the case is `CHECKS_PASSED`. The evidence line
/// appears once, with the mode's subnormal residual-record outcomes; the mode
/// row's observation fields are not observed (OQ5).
#[test]
fn f1b_w2_phys_r4_without_pressure_is_published_at_b_536() {
    let tip = "node:section-b";
    let (uy_m, rz, rx) = (
        f64::from_bits(0x3fd21bb945252404),
        f64::from_bits(0x3fdb2995e7b7b606),
        f64::from_bits(0x3fede0f1b216e1d4),
    );
    for mode in MODES {
        for (entry, envelope) in envelopes(&phys_r4(false), mode, Entries::Both) {
            let ctx = format!("{entry} {mode:?}");
            assert_eq!(envelope.status.mechanics, "MECHANICS_SOLVED", "{ctx}");
            assert!(within(
                row(&envelope, tip, "global_nodal_displacement_y"),
                uy_m * 1000.0
            ));
            assert!(within(row(&envelope, tip, "global_nodal_rotation_z"), rz));
            assert!(within(row(&envelope, tip, "global_nodal_rotation_x"), rx));
            assert_eq!(
                row(&envelope, tip, "global_nodal_displacement_x"),
                0.0,
                "{ctx}"
            );
            let record = integrity(&envelope, "case:source-section");
            assert_eq!(record.code, "NUMERICAL_INTEGRITY_CHECKS_PASSED", "{ctx}");
            assert_eq!(record.message.matches("range_scaling:").count(), 1);
            // The line, with this mode's residual-record outcomes, ends the
            // message: no R-b' sentence follows (the case stays Passed).
            let line = record
                .message
                .split_once(
                    " range_scaling: force_scale_exponent=536; basis=exact power-of-two; record=",
                )
                .map(|(_, tail)| tail)
                .unwrap_or_else(|| panic!("{ctx}: {}", record.message));
            assert!(!line.contains("S11-G"), "{ctx}");
            let basis = envelope
                .results
                .iter()
                .find(|r| r.kind == "linear_solver_mode_basis")
                .and_then(|r| r.metadata.as_ref())
                .map(|m| m.basis.clone())
                .unwrap();
            assert!(
                basis.contains("sparse_entry_count=not_observed; original_profile_entries=not_observed; ordered_profile_entries=not_observed; original_half_bandwidth=not_observed; ordered_half_bandwidth=not_observed; nonpositive_pivots=not_observed; pivot_condition_ratio_proxy=not_observed; max_abs_sparse_residual=not_observed; dense_fallback=false; dense_fallback_message=none"),
                "{ctx}: {basis}"
            );
        }
    }
}

/// Scope §6, K2b's documented limitation (not fixed): the product analogue of
/// the E = 2^440 chain is refused at the rule's b (census span [-1010, 443],
/// b = 312) by name, with M03's trigger and no b (OQ1, c2). The captured
/// entry refuses 2^440 at capture. (F1b's forced-even-b probe records that
/// every even b from 408 to 580 solves it within 1e-9.)
#[test]
fn f1b_w2_limitation_chain_is_refused_by_name_and_not_fixed() {
    assert_w2_refusal(
        &limitation_chain(),
        Entries::TypedOnly,
        &MODES,
        "case",
        "NUMERICAL_INTEGRITY_UNRESOLVED",
        "range: scaled evaluation outside normal range; range_scaling: attempted; step1_trigger=Evaluation(Range(\"radix scaling loses normal range\"))",
    );
}

/// Scope §6, LEF-large (D8) on an analogue of RF-RANGE's construction: the
/// scaled skew chain (pl = 200, pm = 300, pf = 600) is published at b = -702
/// (typed; the captured entry refuses its coordinates), and every published
/// displacement is its base model's times 2^(pf-pm-pl) (translations) or
/// 2^(pf-pm-2pl) (rotations), every force 2^pf and every moment 2^(pf+pl)
/// times the base's, bit for bit (support components and member end actions).
/// Standing is never above the base's: the base is `CHECKS_PASSED`, the
/// scaled case Sensitive by R-b' (Q5(a)).
#[test]
fn f1b_w2_lef_large_analogue_scales_by_exact_powers_of_two() {
    let (pl, pm, pf) = (200, 300, 600);
    let factor = |kind: &str, component: Option<&str>| -> Option<f64> {
        let moment = |c: &str| c.starts_with('M');
        Some(match kind {
            k if k.starts_with("global_nodal_displacement_") => pow2(pf - pm - pl),
            k if k.starts_with("global_nodal_rotation_") => pow2(pf - pm - 2 * pl),
            "support_reaction_component_v2" if moment(component?) => pow2(pf + pl),
            "support_reaction_component_v2" => pow2(pf),
            "element_local_axial_force"
            | "element_local_shear_force_y"
            | "element_local_shear_force_z" => pow2(pf),
            "element_local_torsional_moment"
            | "element_local_bending_moment_y"
            | "element_local_bending_moment_z" => pow2(pf + pl),
            _ => return None,
        })
    };
    for mode in MODES {
        let base = run_linear_static_preview_with_mode(
            serde_json::from_value(scaled_chain(0, 0, 0)).unwrap(),
            mode,
        );
        assert_eq!(base.status.mechanics, "MECHANICS_SOLVED", "{mode:?}");
        assert_eq!(
            integrity(&base, "case").code,
            "NUMERICAL_INTEGRITY_CHECKS_PASSED"
        );
        for (_, scaled) in envelopes(&scaled_chain(pl, pm, pf), mode, Entries::TypedOnly) {
            assert_eq!(scaled.status.mechanics, "MECHANICS_SOLVED", "{mode:?}");
            let record = integrity(&scaled, "case");
            assert_eq!(record.code, "NUMERICAL_INTEGRITY_SENSITIVE", "{mode:?}");
            assert_eq!(record.message.matches("range_scaling:").count(), 1);
            assert!(record.message.contains(
                " range_scaling: force_scale_exponent=-702; basis=exact power-of-two S11-G recovery guard (R-b'): "
            ));
            let mut compared = 0;
            for expected in &base.results {
                let component = expected.metadata.as_ref().map(|m| m.component.as_str());
                let Some(factor) = factor(&expected.kind, component) else {
                    continue;
                };
                let observed = scaled
                    .results
                    .iter()
                    .find(|r| r.id == expected.id)
                    .unwrap_or_else(|| panic!("{mode:?}: {} is published", expected.id));
                assert_eq!(
                    observed.value.to_bits(),
                    (expected.value * factor).to_bits(),
                    "{mode:?} {}: {} against {} x {factor:e}",
                    expected.id,
                    observed.value,
                    expected.value
                );
                compared += 1;
            }
            // 6 nodes x 6 components, N0's 6 support components, and 5
            // members x 5 stations x 6 local actions.
            assert_eq!(compared, 36 + 6 + 150, "{mode:?}");
        }
    }
}

/// Admission (ROOT Q3, OQ7 and OQ13 narrowed): a range case carrying an
/// excluded family is published by the orchestrator at b != 0 and then refused
/// by name, with b; its b = 0 twin solves with no `range_scaling:` text.
/// (The user-stiffness element and non-nodal checks are unreachable in the
/// product and pinned at unit level. The legacy pressure-thrust check, reachable
/// through a zero-valued legacy pressure primitive until U3 refused such
/// primitives before any solve, was removed with the legacy computation.)
#[test]
fn f1b_w2_admission_refuses_each_reachable_family_by_name() {
    let evaluation = "Evaluation(Range(\"arithmetic outside normal range\"))";
    let overflow = "Evaluation(Range(\"product overflow or underflow\"))";
    let zero_term = exact_zero(
        None,
        vec![
            nodal("load:0", "N1", "UY", 2.05e-289),
            nodal("load:zero", "N1", "UY", 0.0),
        ],
    );
    let cases: Vec<(&str, Value, &[PreviewSolverMode], &str, &str, i32)> = vec![
        (
            "uniform_element_load",
            cantilever_family("uniform_element_load", pow2(-1000)),
            &MODES,
            "case:f",
            evaluation,
            516,
        ),
        (
            "thermal_or_eigen_load",
            cantilever_family("thermal_or_eigen_load", pow2(-1000)),
            &MODES,
            "case:f",
            evaluation,
            516,
        ),
        (
            "constant_effort_support",
            cantilever_family("constant_effort_support", pow2(-1000)),
            &MODES,
            "case:f",
            evaluation,
            514,
        ),
        (
            "exact_pressure_operand",
            exact_pressure(1e-150, (4e-77, 1e-77), (1.0, 0.1)),
            &MODES,
            "case:source-section",
            evaluation,
            536,
        ),
        // Re-derived (one row, both modes) after T4-U1 formed the arc angle without libm: the
        // old sparse row (2^-900, b 454) depended on the platform atan2's last bit (T4-RV24).
        (
            "curved_bend_macro_element",
            curved_bend(pow2(-940)),
            &MODES,
            "case:f",
            overflow,
            474,
        ),
        (
            "zero_nodal_load_term",
            zero_term,
            &MODES,
            "case",
            "Formation(NumericalRange { name: \"12EIy/L^3: (12*E)*Iy\" })",
            734,
        ),
    ];
    for (family, request, modes, case, trigger, b) in cases {
        assert_w2_refusal(
            &request,
            Entries::Both,
            modes,
            case,
            "NUMERICAL_INTEGRITY_UNRESOLVED",
            &format!(
                "range: family not admitted under force scaling: {family}; range_scaling: attempted; step1_trigger={trigger}; force_scale_exponent={b}; basis=exact power-of-two"
            ),
        );
    }
    let twins = [
        cantilever_family("uniform_element_load", 1.0),
        cantilever_family("thermal_or_eigen_load", 1.0),
        cantilever_family("constant_effort_support", 1.0),
        exact_pressure(1.0e6, (0.2, 0.01), (2.0e11, 0.3)),
        curved_bend(1.0),
    ];
    for twin in twins {
        for mode in MODES {
            for (entry, envelope) in envelopes(&twin, mode, Entries::Both) {
                assert_eq!(
                    envelope.status.mechanics, "MECHANICS_SOLVED",
                    "{entry} {mode:?}: {:?}",
                    envelope.diagnostics
                );
                assert!(envelope
                    .diagnostics
                    .iter()
                    .all(|d| !d.message.contains("range_scaling")));
            }
        }
    }
}

/// Publication at b != 0 (RV11's F-A2 and F-S, product analogues): a published
/// value that leaves binary64 after unscaling refuses the case by name, with
/// b: never an infinite or flushed value labelled normal. The root moment of
/// `overflowing_root` (about 2^1030 N*m) is a reaction, or a spring action.
#[test]
fn f1b_w2_publication_refuses_a_value_outside_binary64() {
    let trigger = "Evaluation(Range(\"physical residual record overflow\"))";
    for (spring, quantity) in [(false, "reaction@N0:RZ"), (true, "spring_action@N0:RZ")] {
        assert_w2_refusal(
            &overflowing_root(spring),
            Entries::TypedOnly,
            &MODES,
            "case",
            "NUMERICAL_INTEGRITY_UNRESOLVED",
            &format!(
                "range: publication outside binary64; at={quantity}; range_scaling: attempted; step1_trigger={trigger}; force_scale_exponent=-482; basis=exact power-of-two"
            ),
        );
    }
}

/// A subnormal published value carries its precision, and residual-record
/// outcomes are rendered with their fields (RV11-N3): K2a's partial-underflow
/// case (b = 898) publishes its whole evidence line, identically on both
/// entries and in both modes, with S11-G's `NAMED` (6) entries and `more=1`.
#[test]
fn f1b_w2_evidence_line_renders_subnormal_values_and_records() {
    let request = reach(
        9.5367431640625e-07,
        (3.0e-8, 3.0e-9),
        (1.3e-292, 1.0e-200),
        "UY",
        None,
        vec![nodal("load:0", "N1", "UY", 1.0e-307)],
    );
    let line = "range_scaling: force_scale_exponent=898; basis=exact power-of-two; \
        subnormal=reaction@N0:RZ:relative_precision=5.1806537865882194e-11; \
        record=residual_rows.evaluation_allowance@N1:UY:subnormal(relative_precision=0.011111111111111112); \
        record=intended_residual_rows.residual@N1:UY:underflow; \
        record=intended_residual_rows.evaluation_allowance@N1:UY:subnormal(relative_precision=0.003703703703703704); \
        subnormal=reaction@N1:RZ:relative_precision=5.1806537865882194e-11; \
        subnormal=end_action@M1.i:Mz:relative_precision=5.1806537865882194e-11; more=1 ";
    for mode in MODES {
        for (entry, envelope) in envelopes(&request, mode, Entries::Both) {
            let record = integrity(&envelope, "case");
            assert_eq!(
                record.code, "NUMERICAL_INTEGRITY_SENSITIVE",
                "{entry} {mode:?}"
            );
            assert_eq!(record.message.matches(line).count(), 1, "{entry} {mode:?}");
        }
    }
}

/// A non-range failure at the chosen b (RV11-N2; ROOT OQ1 c2 and OQ4): K2a's
/// least-subnormal member in series with a second member whose E is 2^-60
/// times smaller (N0 and N2 anchored, N1 free in UY, a 2^-1021 N load). At
/// the rule's b the soft member's axial stiffness at N1 is absorbed by the
/// assembly, which M03 refuses as today, so the case keeps
/// `append_integrity_failure`'s code and the unscaled error's text, under W2's
/// template with K2a's trigger and without b (the orchestrator returns none).
#[test]
fn f1b_w2_non_range_failure_at_the_chosen_b_keeps_its_code() {
    let length = 1.8189894035458565e-12;
    let mut request = frame(
        "series",
        &[
            ("N0", [0.0; 3]),
            ("N1", [length, 0.0, 0.0]),
            ("N2", [2.0 * length, 0.0, 0.0]),
        ],
        &[("M1", "N0", "N1"), ("M2", "N1", "N2")],
        (1.0e-11, 1.0e-12),
        (9.6e-280, 1.0e-100),
        vec![
            anchor("N0", &ALL),
            anchor("N1", &["UX", "UZ", "RX", "RY", "RZ"]),
            anchor("N2", &ALL),
        ],
        vec![nodal("load:0", "N1", "UY", pow2(-1021))],
    );
    let model = &mut request["model"];
    model["materials"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id": "mat:soft",
        "elastic_modulus": {"value": 9.6e-280 * pow2(-60), "unit": "Pa"},
        "shear_modulus": {"value": 1.0e-100, "unit": "Pa"}, "provenance": PROV}));
    model["pipe_segments"][1]["material"] = json!("mat:soft");
    assert_w2_refusal(
        &request,
        Entries::Both,
        &MODES,
        "case",
        "NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED",
        "structural integrity: NumericallyUnresolved { reason: \"positive diagonal contribution absorbed by assembly; stabilization unresolved\", global_dof: Some(6) }; range_scaling: attempted; step1_trigger=Formation(NumericalRange { name: \"12EIy/L^3: (12*E)*Iy\" })",
    );
}

/// The census on a subnormal derived section value (the routed
/// input-validation finding, documented, not fixed): on the range path it is a
/// refusal by name, with no b; the same section at b = 0 solves unchanged
/// (typed: E = 1e300 Pa is refused at capture).
#[test]
fn f1b_w2_subnormal_derived_section_value_is_refused_on_the_range_path() {
    assert_w2_refusal(
        &subnormal_second_moment(1e-300),
        Entries::TypedOnly,
        &MODES,
        "case",
        "NUMERICAL_INTEGRITY_UNRESOLVED",
        "range: subnormal stiffness or load at formation; range_scaling: attempted; step1_trigger=Evaluation(Range(\"radix scaling loses normal range\")); force_scale_exponent=none",
    );
    for mode in MODES {
        for (_, envelope) in envelopes(&subnormal_second_moment(1.0), mode, Entries::TypedOnly) {
            assert_eq!(envelope.status.mechanics, "MECHANICS_SOLVED", "{mode:?}");
            assert_eq!(
                integrity(&envelope, "case").code,
                "NUMERICAL_INTEGRITY_CHECKS_PASSED"
            );
        }
    }
}

/// The mixed captured invocation (Scope D): case A is P1's NP-A-N05, which
/// exact-block selects; case B's basis leaves the range at formation (main
/// blocks the whole invocation there), so exact-block declines B with zero
/// work (OQ2) and W2 publishes it at b = 524. The invocation finalizes with a
/// consistent, partial receipt: A qualified; B unsupported at source
/// validation, its ordinary attempt following W2's published Sensitive verdict
/// (OQ15). The typed entry publishes both cases and carries no receipt.
#[test]
fn f1b_w2_mixed_captured_invocation_finalizes_with_a_consistent_receipt() {
    for mode in MODES {
        for (entry, envelope) in envelopes(&mixed_invocation(), mode, Entries::Both) {
            let ctx = format!("{entry} {mode:?}");
            assert_eq!(envelope.status.mechanics, "MECHANICS_SOLVED", "{ctx}");
            assert!(envelope
                .diagnostics
                .iter()
                .all(|d| d.severity != "blocking"));
            let a = integrity(&envelope, "case:A");
            let b = integrity(&envelope, "case:B");
            assert_eq!(a.code, "NUMERICAL_INTEGRITY_SENSITIVE", "{ctx}");
            assert_eq!(b.code, "NUMERICAL_INTEGRITY_SENSITIVE", "{ctx}");
            assert!(!a.message.contains("range_scaling"));
            assert!(b
                .message
                .ends_with(" range_scaling: force_scale_exponent=524; basis=exact power-of-two"));
            let receipt = envelope.source_block_recovery.as_ref();
            if entry == "typed" {
                assert!(receipt.is_none(), "{ctx}");
                continue;
            }
            let body = &receipt.expect("captured receipt")["body"];
            assert_eq!(body["status"], "partial", "{ctx}");
            let cases = body["cases"].as_array().unwrap();
            assert_eq!(cases.len(), 2);
            assert_eq!(cases[0]["basis_ref"]["ref_id"], "case:A");
            assert_eq!(cases[0]["outcome"], "qualified", "{ctx}");
            assert_eq!(cases[0]["ordinary_attempt"]["outcome"], "sensitive");
            assert_eq!(cases[1]["basis_ref"]["ref_id"], "case:B");
            assert_eq!(cases[1]["outcome"], "unsupported", "{ctx}");
            assert_eq!(cases[1]["failure"]["code"], "unsupported_family");
            assert_eq!(cases[1]["failure"]["stage"], "source_validation");
            assert_eq!(cases[1]["ordinary_attempt"]["outcome"], "sensitive");
            assert!(envelope
                .diagnostics
                .iter()
                .any(|d| d.id == "diagnostic:source-recovery:case:A:selected"
                    && d.code == "SOURCE_BLOCK_RECOVERY_SELECTED"));
            let decline: Vec<_> = envelope
                .diagnostics
                .iter()
                .filter(|d| d.id == "diagnostic:source-recovery:case:B")
                .collect();
            assert_eq!(decline.len(), 1);
            assert!(decline[0].message.contains(
                "RecoveryFailure { stage: \"range formation\", helper_stage: SourceClosure, error: Unsupported(\"the ordinary stiffness was not formed (range at formation); no retained-source attempt\"), work: WorkReport { charged: 0, rejected: 0, limit: 0 } }"
            ));
        }
    }
}

/// Coexistence with exact-block (Q2(a)), the unselected side: on two of F1b
/// A2's searched candidates (a 1e-310 N and a 2.5e-308 N tip load on an
/// axis-aligned 2 m cantilever; full-mantissa loads, whose exact products lose
/// represented bits) exact-block's failure is main's, and W2 then refuses the
/// case by name. (A2's wider claim, that exact-block never selects a
/// range-triggered case, is refuted by RV17's CX-F and CX-G; the selected side
/// is pinned by
/// `f1b_w2_exact_block_selection_of_a_range_triggered_case_publishes_mains_bytes`.)
#[test]
fn f1b_w2_exact_block_does_not_select_the_searched_coexistence_candidates() {
    let cantilever = |load: f64| {
        frame(
            "coexist",
            &[("N0", [0.0; 3]), ("N1", [2.0, 0.0, 0.0])],
            &[("M1", "N0", "N1")],
            (0.2, 0.01),
            (2.0e11, 8.0e10),
            vec![anchor("N0", &ALL)],
            vec![nodal("load:y", "N1", "UY", load)],
        )
    };
    let trigger = "Evaluation(Range(\"radix scaling loses normal range\"))";
    for (load, prefix) in [
        (
            1e-310,
            format!("range: subnormal stiffness or load at formation; range_scaling: attempted; step1_trigger={trigger}; force_scale_exponent=none"),
        ),
        (
            2.5e-308,
            format!("range: scaled evaluation outside normal range; range_scaling: attempted; step1_trigger={trigger}"),
        ),
    ] {
        assert_w2_refusal(
            &cantilever(load),
            Entries::Both,
            &MODES,
            "case",
            "NUMERICAL_INTEGRITY_UNRESOLVED",
            &prefix,
        );
        for mode in MODES {
            let envelope = run_linear_static_preview_value_with_mode(cantilever(load), mode).unwrap();
            let unavailable: Vec<_> = envelope
                .diagnostics
                .iter()
                .filter(|d| d.code == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE")
                .collect();
            assert_eq!(unavailable.len(), 1, "{load} {mode:?}");
            assert!(unavailable[0]
                .message
                .contains("error: Exact(Arithmetic(Range(\"exact radix loses represented bits\")))"));
        }
    }
}

// ------------------------------------------ RV17's constructions (review fixes)

/// RV17's one-member model (`R/coex/make_coex.py` and `R/probes/make_probe_
/// requests.py` in `T3/REVIEW/_run_records/f1b_review/`), field for field, so
/// that the pinned hashes are the ones RV17 recorded for main: member M1 from
/// N0 to N1 (x = `length`), y reference +y, one material, one case "case".
#[allow(clippy::too_many_arguments)]
fn rv17_member(
    project: &str,
    material: &str,
    prov: &str,
    length: f64,
    (od, wall): (f64, f64),
    (e, g): (f64, f64),
    supports: Vec<Value>,
    loads: Vec<Value>,
) -> Value {
    let label = project.rsplit(':').next().unwrap();
    json!({"model": {
        "schema_version": "0.1.0", "document_kind": "openpipestress.product_preview.model",
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
                            "rule_check": "not_performed_user_rule_inputs_missing",
                            "professional_acceptance": "not_provided"},
        "project": {"id": project,
                    "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa",
                              "temperature": "degC", "stress": "Pa"}},
        "nodes": [{"id": "N0", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": prov},
                  {"id": "N1", "position": {"x": length, "y": 0.0, "z": 0.0}, "provenance": prov}],
        "pipe_segments": [{"id": "M1", "from": "N0", "to": "N1", "material": material,
                           "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0},
                           "section": {"outside_diameter": {"value": od, "unit": "m"},
                                       "wall_thickness": {"value": wall, "unit": "m"}},
                           "provenance": prov}],
        "materials": [{"id": material, "elastic_modulus": {"value": e, "unit": "Pa"},
                       "shear_modulus": {"value": g, "unit": "Pa"}, "provenance": prov}],
        "supports": supports,
        "load_cases": [{"id": "case", "label": label, "kind": "primitive_user_load",
                        "primitive_loads": loads, "provenance": prov}],
        "combinations": []}, "materials": []})
}

fn rv17_anchor(node: &str, restraints: &[&str], prov: &str) -> Value {
    json!({"id": format!("anchor:{node}"), "node": node, "family": "anchor", "restraints": restraints,
           "provenance": prov})
}

fn rv17_force(id: &str, dof: &str, value: f64, prov: &str) -> Value {
    json!({"id": id, "category": "concentrated_force", "target": {"type": "node", "node": "N1"},
           "direction": format!("global_{}", dof[1..].to_lowercase()),
           "magnitude": {"value": value, "unit": "N"}, "dimension": "force", "provenance": prov})
}

const RV17_COEX: &str = "invented_rv17_f1b_review_input_no_library_data";
const RV17_PROBE: &str = "invented_rv17_f1b_review_probe_no_library_data";

/// RV17's CX-F: a 1 m member (OD 0.02 m, wall 0.002 m, E chosen so that EA/L is
/// about 2^16 N/m), free only in N1's UX, with a one-bit-mantissa tip load of
/// 3 * 2^-1016 N. M03's scaled right-hand side is subnormal (the ordinary
/// attempt range-triggers, `Range("division overflow or underflow")`), while
/// every exact product is short and representable, so exact-block selects it.
fn coexistence_cx_f() -> Value {
    let free_ux: Vec<&str> = ALL.iter().copied().filter(|d| *d != "UX").collect();
    rv17_member(
        "invented:rv17-coex:CX-F",
        "mat:C",
        RV17_COEX,
        1.0,
        (0.02, 0.002),
        (579465463.915025, 222871332.2750096),
        vec![
            rv17_anchor("N0", &ALL, RV17_COEX),
            rv17_anchor("N1", &free_ux, RV17_COEX),
        ],
        vec![rv17_force("load:0", "UX", 3.0 * pow2(-1016), RV17_COEX)],
    )
}

/// RV17's probe m1: K2a's partial-underflow member with an open gap on N1 UY,
/// a nonlinear invocation whose basis formation leaves the range.
fn nonlinear_formation_range_m1() -> Value {
    let mut supports = vec![
        rv17_anchor("N0", &ALL, RV17_PROBE),
        rv17_anchor("N1", &["UX", "UZ", "RX", "RY", "RZ"], RV17_PROBE),
    ];
    supports.push(json!({"id": "support:open-gap", "node": "N1", "family": "nonlinear", "restraints": [],
        "nonlinear": {"behavior": "gap", "dof": "UY", "initial_state": "inactive",
                      "closes_when": "positive_displacement", "gap": {"value": 1000.0, "unit": "mm"}},
        "provenance": RV17_PROBE}));
    rv17_member(
        "invented:rv17-probe:m1",
        "mat:P",
        RV17_PROBE,
        9.5367431640625e-07,
        (3.0e-8, 3.0e-9),
        (1.3e-292, 1.0e-200),
        supports,
        vec![rv17_force("load:0", "UY", 1.0e-307, RV17_PROBE)],
    )
}

/// RV17's probe m2a: K2a's exact-zero member (b = 734), free in UY and UZ at
/// N1, each held by its own ground spring (1e-289 and 3e-289 N/m) and loaded.
fn two_springs_m2a() -> Value {
    let spring = |dof: &str, k: f64| {
        json!({"id": format!("spring:N1:{dof}"), "node": "N1", "family": "spring", "restraints": [dof],
               "stiffness": {"dof": dof, "value": {"value": k, "unit": "N/m"}}, "provenance": RV17_PROBE})
    };
    rv17_member(
        "invented:rv17-probe:m2a",
        "mat:P",
        RV17_PROBE,
        1.8189894035458565e-12,
        (1.0e-11, 1.0e-12),
        (6.4e-280, 1.0e-100),
        vec![
            rv17_anchor("N0", &ALL, RV17_PROBE),
            rv17_anchor("N1", &["UX", "RX", "RY", "RZ"], RV17_PROBE),
            spring("UY", 1.0e-289),
            spring("UZ", 3.0e-289),
        ],
        vec![
            rv17_force("load:y", "UY", 2.05e-289, RV17_PROBE),
            rv17_force("load:z", "UZ", 4.1e-289, RV17_PROBE),
        ],
    )
}

/// The sha256 of the serialized `MechanicsEnvelope`: the gate's full-envelope
/// probe's `run.envelope_sha256`.
fn full_envelope_sha256(envelope: &MechanicsEnvelope) -> String {
    use sha2::{Digest, Sha256};
    format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(envelope).unwrap())
    )
}

/// Coexistence with exact-block, the selected side (brief test D; RV17-1;
/// ROOT's re-ruling of A2 (c)): exact-block does select range-triggered cases,
/// and wherever it selects, F1b publishes main's bytes, because the selected
/// arm of the outcome match precedes W2's arm.
///
/// - Captured entry (exact-block's route), both modes: `MECHANICS_SOLVED`, the
///   selection's diagnostic, the receipt, the rejected ordinary attempt kept as
///   an info record with its Range trigger, no `range_scaling` text, and the
///   full-envelope sha256 equal to Mac main's (`e7d930d49`, recorded with the
///   gate's full-envelope probe; RV17 recorded the same). Platform-independent:
///   every magnitude here has one nonzero component (Annex F: hypot(x, +-0) =
///   |x|), there is no thermal load, and exact-block's arithmetic is exact.
///   Re-pinned for the pressure retirement (U3 T2): main's bytes with only the
///   declared formulation-limitation string replaced and the two receipt
///   digests recomputed by the product's digest rule (ROOT's mechanical check).
/// - Typed entry (no capture, so exact-block is not eligible), both modes: W2
///   refuses the case by name (`ScaledEvaluation`, no b under c2), where main
///   refuses it with the ordinary Range text (the C1 kind).
///
/// A re-pin is due if a later slice legitimately changes this model's envelope.
#[test]
fn f1b_w2_exact_block_selection_of_a_range_triggered_case_publishes_mains_bytes() {
    let request = coexistence_cx_f();
    for (mode, main_sha256) in [
        (
            PreviewSolverMode::SparseInteractive,
            "aaabc777a0f2f3bff2392fc442eb2fac48c305345b25164ceb9d7cfbb90ecf9c",
        ),
        (
            PreviewSolverMode::DenseScrutiny,
            "7db0edaf667ef1d38fcdfb72da9df313adf2f716c305a5d22219fc64a5400f7f",
        ),
    ] {
        let envelope = run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap();
        assert_eq!(
            envelope.status.mechanics, "MECHANICS_SOLVED",
            "{mode:?}: {:?}",
            envelope.diagnostics
        );
        let selected: Vec<_> = envelope
            .diagnostics
            .iter()
            .filter(|d| d.id == "diagnostic:source-recovery:case:selected")
            .collect();
        assert_eq!(selected.len(), 1, "{mode:?}");
        assert_eq!(selected[0].code, "SOURCE_BLOCK_RECOVERY_SELECTED");
        assert!(
            envelope.source_block_recovery.is_some(),
            "{mode:?}: the receipt"
        );
        let record = integrity(&envelope, "case");
        assert_eq!(record.code, "NUMERICAL_INTEGRITY_UNRESOLVED", "{mode:?}");
        assert_eq!(record.severity, "info", "{mode:?}");
        assert!(
            record.message.starts_with(
                "Rejected ordinary attempt; a separate retained-source response is selected. Load case case: structural integrity: Range(\"division overflow or underflow\"); global_dof_map="
            ),
            "{mode:?}: {}",
            record.message
        );
        assert!(envelope
            .diagnostics
            .iter()
            .all(|d| !d.message.contains("range_scaling")));
        assert_eq!(full_envelope_sha256(&envelope), main_sha256, "{mode:?}");
    }
    let typed_request: LinearStaticPreviewRequest = serde_json::from_value(request).unwrap();
    for mode in MODES {
        let envelope = run_linear_static_preview_with_mode(typed_request.clone(), mode);
        assert!(envelope.results.is_empty(), "{mode:?}");
        let record = integrity(&envelope, "case");
        assert_eq!(record.severity, "blocking", "{mode:?}");
        assert_eq!(record.code, "NUMERICAL_INTEGRITY_UNRESOLVED", "{mode:?}");
        assert!(
            record.message.starts_with(
                "Load case case: range: scaled evaluation outside normal range; range_scaling: attempted; step1_trigger=Evaluation(Range(\"division overflow or underflow\")); global_dof_map="
            ),
            "{mode:?}: {}",
            record.message
        );
    }
}

/// A nonlinear invocation whose basis formation leaves the range keeps main's
/// blocked envelope in full (Q2: "invocations with a nonlinear support never
/// engage W2 and publish byte for byte as main"; RV17-2): exactly one
/// diagnostic, the invocation's `SOLVER_SYSTEM_BLOCKED` with K2a's name, no
/// result, and the full-envelope sha256 equal to Mac main's on both entries and
/// in both modes. A deferral on this invocation would run the case loop first
/// and add its diagnostics (RV17-M1).
#[test]
fn f1b_w2_nonlinear_formation_range_invocation_is_mains_blocked_envelope() {
    let request = nonlinear_formation_range_m1();
    for mode in MODES {
        for (entry, envelope) in envelopes(&request, mode, Entries::Both) {
            let ctx = format!("{entry} {mode:?}");
            assert_eq!(envelope.status.mechanics, "MODEL_INCOMPLETE", "{ctx}");
            assert!(envelope.results.is_empty(), "{ctx}");
            let diagnostics: Vec<_> = envelope
                .diagnostics
                .iter()
                .map(|d| {
                    (
                        d.id.as_str(),
                        d.code.as_str(),
                        d.severity.as_str(),
                        d.message.as_str(),
                    )
                })
                .collect();
            assert_eq!(
                diagnostics,
                vec![(
                    "diagnostic:physics:solver",
                    "SOLVER_SYSTEM_BLOCKED",
                    "blocking",
                    "range: stiffness formation outside the binary64 normal range at 12EIy/L^3: (12*E)*Iy (zero, subnormal or non-finite from nonzero finite operands)"
                )],
                "{ctx}"
            );
            assert_eq!(
                full_envelope_sha256(&envelope),
                "a29e29f2c447a96cdac952c64a82cb528d974c3f78bc41c7cd2a1a4f46a7bdbb",
                "{ctx}"
            );
        }
    }
}

/// W2 publication with two springs (RV17-3): each spring's action is its own.
/// K2a's exact-zero member, free in UY and UZ at N1 with springs of 1e-289 and
/// 3e-289 N/m, is published at b = 734 on both entries and in both modes. Each
/// spring's published support component is within 1e-9 of -k*u from the
/// published displacement of its own DOF (mm to m), and the two differ.
#[test]
fn f1b_w2_two_spring_publication_gives_each_spring_its_own_action() {
    let request = two_springs_m2a();
    let component = |envelope: &MechanicsEnvelope, support: &str, name: &str| -> f64 {
        let rows: Vec<_> = envelope
            .results
            .iter()
            .filter(|r| {
                r.kind == "support_reaction_component_v2"
                    && r.entity_ref == support
                    && r.metadata.as_ref().map(|m| m.component.as_str()) == Some(name)
            })
            .collect();
        assert_eq!(rows.len(), 1, "{support} {name}");
        rows[0].value
    };
    for mode in MODES {
        for (entry, envelope) in envelopes(&request, mode, Entries::Both) {
            let ctx = format!("{entry} {mode:?}");
            assert_eq!(
                envelope.status.mechanics, "MECHANICS_SOLVED",
                "{ctx}: {:?}",
                envelope.diagnostics
            );
            assert!(
                integrity(&envelope, "case")
                    .message
                    .contains(" range_scaling: force_scale_exponent=734; basis=exact power-of-two"),
                "{ctx}"
            );
            let mut actions = Vec::new();
            for (support, name, kind, k) in [
                (
                    "spring:N1:UY",
                    "Fy",
                    "global_nodal_displacement_y",
                    1.0e-289,
                ),
                (
                    "spring:N1:UZ",
                    "Fz",
                    "global_nodal_displacement_z",
                    3.0e-289,
                ),
            ] {
                let action = component(&envelope, support, name);
                let u = row(&envelope, "N1", kind) / 1000.0;
                let expected = -k * u;
                assert!(
                    expected != 0.0 && within(action, expected),
                    "{ctx}: {support} {action} against {expected}"
                );
                actions.push(action);
            }
            assert!(actions[0] != actions[1], "{ctx}: {actions:?}");
        }
    }
}
