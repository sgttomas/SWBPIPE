//! DEL-05-01 VER-005 and AC-007: static-equivalent seismic generation with a
//! rigid-family component's force, against the frozen independent references
//! in `validation/references/seismic_component_weight/` (exact rational
//! arithmetic, two solution methods, no product code; pinned to its
//! SHA256SUMS). Nothing here derives an expectation from product code, and no
//! expected value is changed here.
//!
//! The product solves each reference document on its route (PF and EX) and
//! model (CANT and FIXED) in both solver modes, and is compared row by row:
//! every nodal motion, every support reaction and every member end action,
//! and every generated load and load resultant through the published rows'
//! equilibrium (see `observed`). Every row of a reference case is compared.
//! The admitted positive case (the valve at C, W = 2350 N, offset centre of
//! gravity), the not-consumed flange (TWO), the unequal gravities (G10 and
//! the one-ulp boundary), the case with both components not consumed and the
//! zero-weight document (which keeps its distributed load) are references.
//! The labelled controls of the package (today's rule, the double count, the
//! omitted moment, the fixed-mass inertia, k times the weight case, keyed on
//! mapped, and the zero-weight limit) are each solved document against which
//! the product must disagree: each is the regression it names.
//!
//! Criterion (DEL-05-01 AC-001's analytic tolerance): |obs - exp| <= 1e-9
//! |exp|; an exactly zero expectation uses 1e-9 times the case's largest
//! reference magnitude, over its compared rows, in the same row family as its
//! absolute floor. The families are the package's own
//! (`generators/scw_reference.py` `family`): load per length (N/m), force
//! (N), moment (N m), translation (m) and rotation (rad).
//!
//! The model-check warning of REQ-008 is not built (DEL-05-01 "Not met"); the
//! package's `expected_model_check_warning` members are not compared here.
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

const REFERENCES: &str = include_str!("../../../validation/references/seismic_component_weight/references.json");
const SUMS: &str = include_str!("../../../validation/references/seismic_component_weight/SHA256SUMS");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const RELATIVE: f64 = 1.0e-9;
const PROV: &str = "seismic component weight invented benchmark input; not library, component, material-catalog or code data";
const RANGE_INVALID: &str = "EQUIVALENT_STATIC_COMPONENT_RANGE_INVALID";

// ------------------------------------------------------------------ references
fn refs() -> &'static Value {
    use std::sync::OnceLock;
    static DOC: OnceLock<Value> = OnceLock::new();
    DOC.get_or_init(|| serde_json::from_str(REFERENCES).expect("references.json parses"))
}
fn reference(id: &str) -> &'static Value {
    refs()["cases"].as_array().unwrap().iter().find(|c| c["id"] == id).unwrap_or_else(|| panic!("no case {id}"))
}
fn rational(text: &str) -> f64 {
    match text.split_once('/') {
        Some((p, q)) => p.parse::<f64>().unwrap() / q.parse::<f64>().unwrap(),
        None => text.parse().unwrap(),
    }
}
/// An input's nearest binary64: its exact rational, divided once (the
/// numerator and denominator of every input here are exact in binary64).
fn input(name: &str) -> f64 {
    rational(refs()["inputs"][name]["exact"].as_str().unwrap_or_else(|| panic!("input {name}")))
}
fn input3(name: &str) -> [f64; 3] {
    let v = refs()["inputs"][name]["exact"].as_array().unwrap();
    [0, 1, 2].map(|i| rational(v[i].as_str().unwrap()))
}

#[test]
fn references_are_the_frozen_bytes() {
    let digest: String = Sha256::digest(REFERENCES.as_bytes()).iter().map(|b| format!("{b:02x}")).collect();
    let line = SUMS.lines().find(|l| l.ends_with("  references.json")).expect("SHA256SUMS lists references.json");
    assert_eq!(&line[..64], digest, "the references must equal the frozen bytes");
    assert_eq!(refs()["cases"].as_array().unwrap().len(), 72);
    // The binary64 inputs used here are those the package's binary64 check
    // repeated the build with (every input replaced by its nearest binary64).
    assert_eq!(refs()["binary64_input_check"]["discrete_outcome_changes"], 0);
}

// ------------------------------------------------------------------ authoring
#[derive(Clone, Copy, PartialEq, Debug)]
enum Route {
    Pf,
    Ex,
}
#[derive(Clone, Copy, PartialEq, Debug)]
enum Model {
    Cant,
    Fixed,
}
/// What the document declares about its two components.
#[derive(Clone, Copy, PartialEq, Debug)]
struct Components {
    valve_weight: f64,
    valve_consumed: bool,
    flange: bool,
}
const ONE_MAPPED: Components = Components { valve_weight: 2350.0, valve_consumed: true, flange: false };
const TWO_MAPPED: Components = Components { valve_weight: 2350.0, valve_consumed: true, flange: true };
const NONE_CONSUMED: Components = Components { valve_weight: 2350.0, valve_consumed: false, flange: true };
const ZERO_WEIGHT: Components = Components { valve_weight: 0.0, valve_consumed: true, flange: false };

fn qv(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}

fn section(route: Route) -> Value {
    let mut s = json!({"outside_diameter": qv(input("D"), "m"), "wall_thickness": qv(input("t"), "m"),
        "mill_tolerance": qv(input("c_m"), "m"), "material_density": qv(input("rho_m"), "kg/m^3"),
        "contents_density": qv(input("rho_c"), "kg/m^3"), "insulation_thickness": qv(input("t_i"), "m"),
        "insulation_density": qv(input("rho_i"), "kg/m^3")});
    if route == Route::Ex {
        s["corrosion_allowance"] = qv(input("c_a"), "m");
    }
    s
}

fn component(id: &str, kind: &str, node: &str, pipe: &str, weight: f64, cog: [f64; 3], consumed: bool, scale: Option<f64>) -> Value {
    let mut c = json!({"id": id, "kind": kind, "node": node,
        "geometry": {"rigid_pipe_ref": pipe, "weight": qv(weight, "N"), "center_of_gravity": {"x": cog[0], "y": cog[1], "z": cog[2], "unit": "m"}},
        "mechanics_interface": {"solver_consumption": if consumed { "mechanics_geometry_only" } else { "not_solver_consumed" }},
        "provenance": PROV});
    if let Some(scale) = scale {
        c["modifiers"] = json!({"stiffness_scaling_user_value": qv(scale, "1"), "source_reference": "invented seismic component weight scale"});
    }
    c
}

/// The reference's three members A - B - C - D along +X, with its supports
/// and components, and `cases`.
fn document(route: Route, model: Model, components: Components, cases: Vec<Value>) -> Value {
    let x = [0.0, input("L_ab"), input("L_ab") + input("L_bc"), input("L_ab") + input("L_bc") + input("L_cd")];
    let nodes: Vec<Value> = ["A", "B", "C", "D"].iter().zip(x)
        .map(|(id, x)| json!({"id": id, "position": {"x": x, "y": 0.0, "z": 0.0}, "provenance": PROV})).collect();
    let pipes: Vec<Value> = [("AB", "A", "B"), ("BC", "B", "C"), ("CD", "C", "D")].iter()
        .map(|(id, from, to)| json!({"id": id, "from": from, "to": to, "section": section(route), "material": "material:scw",
            "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0}, "provenance": PROV})).collect();
    let anchor = |id: &str, node: &str| json!({"id": id, "node": node, "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV});
    let mut supports = vec![anchor("support:A", "A")];
    if model == Model::Fixed {
        supports.push(anchor("support:D", "D"));
    }
    let mut parts = vec![component("component:valve", "valve", "C", "BC", components.valve_weight, input3("r_v"),
        components.valve_consumed, Some(input("s_v")))];
    if components.flange {
        parts.push(component("component:flange", "flange", "B", "AB", input("W_f"), input3("r_f"), false, None));
    }
    let e = input("E");
    let material = match route {
        Route::Pf => json!({"id": "material:scw", "elastic_modulus": qv(e, "Pa"), "shear_modulus": qv(e / (2.0 * (1.0 + input("nu"))), "Pa"), "provenance": PROV}),
        Route::Ex => json!({"id": "material:scw", "constitutive_basis": "homogeneous_isotropic_E_nu_v1", "elastic_modulus": qv(e, "Pa"),
            "poisson_ratio": qv(input("nu"), "1"), "provenance": PROV}),
    };
    let mut cases = cases;
    if route == Route::Ex {
        for case in &mut cases {
            case.as_object_mut().unwrap().entry("pressure_regions").or_insert(json!([]));
        }
    }
    let mut model = json!({"schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:scw", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": nodes, "pipe_segments": pipes, "supports": supports, "components": parts, "materials": [material],
        "load_cases": cases, "combinations": []});
    if route == Route::Ex {
        model["pressure_contract"] = json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"});
    }
    json!({"model": model, "materials": []})
}

/// A seismic case with the reference case's authored gravity and its nonzero
/// g-factors (an absent factor and a zero one give the same loads; the zero
/// factor's own rule is tested below).
fn seismic_case(id: &str, gravity: f64, factors: [f64; 3]) -> Value {
    let mut seismic = json!({"gravity_acceleration": qv(gravity, "m/s^2")});
    for (axis, k) in ["x", "y", "z"].iter().zip(factors) {
        if k != 0.0 {
            seismic[format!("g_factor_{axis}")] = qv(k, "1");
        }
    }
    json!({"id": id, "primitive_loads": [], "equivalent_static": {"seismic": seismic, "provenance": PROV}, "provenance": PROV})
}
fn case_of(reference: &Value, id: &str) -> Value {
    let factors = reference["g_factors"].as_array().unwrap();
    seismic_case(id, reference["gravity_authored"]["value"].as_f64().unwrap(), [0, 1, 2].map(|i| factors[i]["value"].as_f64().unwrap()))
}

// ------------------------------------------------------------------ running
fn run(wire: &Value, mode: PreviewSolverMode) -> Value {
    let out = run_linear_static_preview_value_with_mode(wire.clone(), mode).unwrap_or_else(|e| panic!("typed boundary rejected the request: {e}"));
    serde_json::to_value(out).unwrap()
}
fn codes(out: &Value) -> Vec<String> {
    out["diagnostics"].as_array().unwrap().iter().map(|d| d["code"].as_str().unwrap().to_string()).collect()
}
fn solved(wire: &Value, mode: PreviewSolverMode, what: &str) -> Value {
    let out = run(wire, mode);
    let blocking: Vec<_> = out["diagnostics"].as_array().unwrap().iter()
        .filter(|d| d["severity"] == "blocking" || d["severity"] == "failure").collect();
    assert_eq!(out["status"]["mechanics"], "MECHANICS_SOLVED", "{what}: blocking {blocking:?}");
    assert!(blocking.is_empty(), "{what}: blocking {blocking:?}");
    // Every case passes its numerical integrity checks. The component's
    // nodal terms enter the certified ledger as the rule's stored values,
    // each bound to its operands where the ledger is built.
    assert_eq!(out["numerical_quality"]["status"], "checks_passed", "{what}: {:?}", codes(&out));
    out
}
fn refused(wire: &Value, code: &str, what: &str) -> Vec<Value> {
    let mut found = Vec::new();
    for mode in MODES {
        let out = run(wire, mode);
        assert_eq!(out["status"]["mechanics"], "MODEL_INCOMPLETE", "{what}: {:?}", codes(&out));
        assert!(out["results"].as_array().unwrap().is_empty(), "{what}: a refused document publishes no rows");
        found = out["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == code).cloned().collect();
        assert!(!found.is_empty(), "{what}: expected {code}, got {:?}", codes(&out));
        assert!(found.iter().all(|d| d["severity"] == "blocking"), "{what}");
    }
    found
}

/// The model's node positions along +X (the document's own, `document`).
fn node_x(node: &str) -> f64 {
    let x = [0.0, input("L_ab"), input("L_ab") + input("L_bc"), input("L_ab") + input("L_bc") + input("L_cd")];
    x[["A", "B", "C", "D"].iter().position(|n| *n == node).unwrap_or_else(|| panic!("node {node}"))]
}
const MEMBERS: [(&str, &str, &str); 3] = [("AB", "A", "B"), ("BC", "B", "C"), ("CD", "C", "D")];

fn rows_of<'a>(out: &'a Value, case: &str, kind: &str, entity: &str, component: &str, location: Option<&str>) -> Vec<&'a Value> {
    out["results"].as_array().unwrap().iter().filter(|row| {
        row["kind"] == kind && row["entity_ref"] == entity && row["basis_ref"]["ref_id"] == case
            && row["metadata"]["component"] == component && location.is_none_or(|l| row["metadata"]["location"] == l)
    }).collect()
}
fn one(out: &Value, case: &str, kind: &str, entity: &str, component: &str, location: Option<&str>) -> f64 {
    let found = rows_of(out, case, kind, entity, component, location);
    assert_eq!(found.len(), 1, "one row {kind} {entity} {component} {location:?} on {case}");
    found[0]["value"].as_f64().unwrap()
}

const END_COMPONENTS: [(&str, &str); 6] = [("Fx", "axial_force"), ("Fy", "shear_force_y"), ("Fz", "shear_force_z"),
    ("Mx", "torsional_moment"), ("My", "bending_moment_y"), ("Mz", "bending_moment_z")];

/// The force the node exerts on `member` at `end` ("i" or "j"), component
/// `c` (Fx … Mz): the product's local end action (local axes coincide with
/// global here).
fn end_action(out: &Value, case: &str, member: &str, end: &str, c: &str) -> f64 {
    let name = END_COMPONENTS.iter().find(|(r, _)| *r == c).unwrap().1;
    one(out, case, &format!("element_local_{name}"), member, name, Some(if end == "i" { "end_i" } else { "end_j" }))
}
/// The model's supports in this result: A, and D on the fixed-ended model.
fn supports(out: &Value, case: &str) -> Vec<&'static str> {
    ["A", "D"].into_iter().filter(|n| !rows_of(out, case, "support_reaction_component_v2", &format!("support:{n}"), "Fx", None).is_empty()).collect()
}

/// The product's value for one reference row, in the reference's SI unit and
/// convention: `disp` in m (published in mm) and rad; `reaction` as the force
/// the support exerts on the structure (the product's support reaction);
/// `end` as the force the node exerts on the member, which is the product's
/// local end action at end_i and end_j (local axes coincide with global
/// here). The solved envelope publishes no row of the generated loads or of
/// their resultant, so those are read through the published rows'
/// equilibrium, which the package's cross-checks also hold:
/// - `load/<member>/q<a>`, the member's load per length: its own equilibrium,
///   q = -(e_i + e_j) / L, from its two end actions;
/// - `load/<node>/<F|M><a>`, a nodal wrench at a free node: the node's
///   equilibrium, the sum of the end actions of the members meeting there;
/// - `resultant/F<a>` and `resultant/M<a>` (moments about the origin):
///   global equilibrium, minus the sum of the support reactions and, for
///   moments, of r_s × R_s with r_s the support's position.
/// Every row of the package has an observable: `None` is not returned.
fn observed(out: &Value, case: &str, row: &str) -> Option<f64> {
    let parts: Vec<&str> = row.split('/').collect();
    let axis = |c: &str| ["x", "y", "z"].iter().position(|a| c.ends_with(a)).unwrap();
    let reaction = |node: &str, c: &str| one(out, case, "support_reaction_component_v2", &format!("support:{node}"), c, None);
    match parts[0] {
        "disp" => {
            let (node, c) = (parts[1], parts[2]);
            let axis = &c[1..];
            Some(if c.starts_with('u') {
                one(out, case, &format!("global_nodal_displacement_{axis}"), node, &format!("nodal_displacement_{axis}"), None) / 1000.0
            } else {
                one(out, case, &format!("global_nodal_rotation_{axis}"), node, &format!("nodal_rotation_{axis}"), None)
            })
        }
        "reaction" => Some(reaction(parts[1], parts[2])),
        "end" => Some(end_action(out, case, parts[1], parts[2], parts[3])),
        "load" if parts[2].starts_with('q') => {
            let member = parts[1];
            let (_, from, to) = *MEMBERS.iter().find(|(m, _, _)| *m == member).unwrap();
            let f = format!("F{}", &parts[2][1..]);
            Some(-(end_action(out, case, member, "i", &f) + end_action(out, case, member, "j", &f)) / (node_x(to) - node_x(from)))
        }
        "load" => {
            let (node, c) = (parts[1], parts[2]);
            assert!(!supports(out, case).contains(&node), "{row}: a nodal wrench at a supported node");
            Some(MEMBERS.iter().map(|(m, from, to)| {
                (if *from == node { end_action(out, case, m, "i", c) } else { 0.0 })
                    + (if *to == node { end_action(out, case, m, "j", c) } else { 0.0 })
            }).sum())
        }
        "resultant" => {
            let c = parts[1];
            let i = axis(c);
            Some(-supports(out, case).iter().map(|s| {
                if c.starts_with('F') {
                    reaction(s, c)
                } else {
                    // r_s = (x_s, 0, 0): (r × R) = (0, -x R_z, x R_y).
                    let x = node_x(s);
                    reaction(s, c) + [0.0, -x * reaction(s, "Fz"), x * reaction(s, "Fy")][i]
                }
            }).sum::<f64>())
        }
        other => panic!("row family {other}"),
    }
}

/// The package's row family (`generators/scw_reference.py` `family`).
fn family(row: &str) -> &'static str {
    let head = row.split('/').next().unwrap();
    let last = row.rsplit('/').next().unwrap();
    match head {
        "load" if last.starts_with('q') => "load per length",
        "load" | "resultant" | "reaction" | "end" if last.starts_with('F') => "force",
        "load" | "resultant" | "reaction" | "end" => "moment",
        _ if last.starts_with('u') => "translation",
        _ => "rotation",
    }
}

/// Every comparable row of `reference` against the product's case `case`:
/// the rows outside the criterion, and the count compared. Each family's
/// floor is built over the rows compared.
fn measure(out: &Value, reference: &Value, case: &str) -> (usize, Vec<String>) {
    let compared: Vec<(&str, f64, f64)> = reference["rows"].as_array().unwrap().iter()
        .filter_map(|r| {
            let row = r["row"].as_str().unwrap();
            observed(out, case, row).map(|o| (row, o, r["value"].as_f64().unwrap()))
        })
        .collect();
    let mut floor: BTreeMap<&str, f64> = BTreeMap::new();
    for &(row, _, e) in &compared {
        let entry = floor.entry(family(row)).or_insert(0.0);
        *entry = entry.max(e.abs());
    }
    let mut failures = Vec::new();
    for &(row, o, e) in &compared {
        let tolerance = RELATIVE * if e == 0.0 { floor[family(row)] } else { e.abs() };
        if !(o.is_finite() && (o - e).abs() <= tolerance) {
            failures.push(format!("{row}: observed {o:.17e} expected {e:.17e} tolerance {tolerance:.3e}"));
        }
    }
    (compared.len(), failures)
}

fn route_tag(route: Route) -> &'static str {
    if route == Route::Pf { "PF" } else { "EX" }
}
fn model_tag(model: Model) -> &'static str {
    if model == Model::Cant { "CANT" } else { "FIXED" }
}

/// The reference cases of one document, keyed by product case id.
struct Document {
    components: Components,
    names: &'static [&'static str],
    cant_only: bool,
}
const DOCUMENTS: [Document; 5] = [
    Document { components: ONE_MAPPED, names: &["SEISMIC-X", "SEISMIC-Y", "SEISMIC-Z", "SEISMIC-XYZ", "SEISMIC-XYZ-G10"], cant_only: false },
    Document { components: TWO_MAPPED, names: &["TWO-SEISMIC-XYZ"], cant_only: false },
    Document { components: ONE_MAPPED, names: &["BND-BITS-SEISMIC-XYZ", "BND-ULP-SEISMIC-XYZ"], cant_only: true },
    Document { components: NONE_CONSUMED, names: &["BND-NONE-SEISMIC-XYZ-G10"], cant_only: true },
    Document { components: ZERO_WEIGHT, names: &["CTRL-ZERO-AS-NOT-APPLIED"], cant_only: true },
];

/// Solve every reference document on `route` and `model` in `mode`: product
/// case id -> (reference id, envelope).
fn solve_all(route: Route, model: Model, mode: PreviewSolverMode) -> Vec<(String, Value, &'static Document)> {
    let mut solved_cases = Vec::new();
    for doc in &DOCUMENTS {
        if doc.cant_only && model == Model::Fixed {
            continue;
        }
        let ids: Vec<String> = doc.names.iter().map(|n| format!("SCW-{}-{}-{n}", model_tag(model), route_tag(route))).collect();
        let cases = ids.iter().map(|id| case_of(reference(id), &format!("case:{id}"))).collect();
        let wire = document(route, model, doc.components, cases);
        let out = solved(&wire, mode, &format!("{:?}", doc.names));
        for id in ids {
            solved_cases.push((id, out.clone(), doc));
        }
    }
    solved_cases
}

// ------------------------------------------------------------------ the oracle
/// Every seismic reference document a person can make, on both routes, both
/// models and both solver modes: every row of each of the package's 32
/// seismic reference cases (every nodal motion, support reaction, member end
/// action, generated load and load resultant) within the criterion: 84 rows a
/// case on the cantilever, 81 for the cases with no replacing component and
/// the zero-weight fallback (BND-NONE, CTRL-ZERO-AS-NOT-APPLIED), 90 on the
/// fixed-ended model. The package's self-weight reference cases are not
/// solved here, and its controls and limits are read as controls
/// ([`the_product_disagrees_with_every_control`]).
#[test]
fn every_seismic_reference_case_is_reproduced() {
    let mut worst = 0usize;
    let mut compared = std::collections::BTreeSet::new();
    for route in [Route::Pf, Route::Ex] {
        for model in [Model::Cant, Model::Fixed] {
            for mode in MODES {
                for (id, out, _) in solve_all(route, model, mode) {
                    let r = reference(&id);
                    assert_eq!(r["role"], "reference", "{id}");
                    let (n, failures) = measure(&out, r, &format!("case:{id}"));
                    assert_eq!(n, r["rows"].as_array().unwrap().len(), "{id}: every row is compared");
                    assert!(failures.is_empty(), "{id} {}: {} rows outside the criterion:\n{}", mode.as_str(), failures.len(), failures.join("\n"));
                    worst = worst.max(n);
                    compared.insert(id);
                }
            }
        }
    }
    assert!(worst > 0);
    let package: std::collections::BTreeSet<String> = refs()["cases"].as_array().unwrap().iter()
        .filter(|c| c["role"] == "reference" && c["load_kind"] == "seismic")
        .map(|c| c["id"].as_str().unwrap().to_string()).collect();
    assert_eq!(compared.len(), 32);
    assert_eq!(compared, package, "every seismic reference case of the package is compared");
}

/// Each labelled control is a wrong rule, and the product must disagree
/// with it on its reference's document: a product that regressed to the
/// control's rule fails here. The zero-weight document is also compared with
/// the zero-weight limit, which a product applying an entered zero weight
/// would reproduce.
#[test]
fn the_product_disagrees_with_every_control() {
    let controls: [(&str, &str); 7] = [
        ("CTRL-A-TODAY", "SEISMIC-XYZ"),
        ("CTRL-B-DOUBLE", "SEISMIC-XYZ"),
        ("CTRL-C-NO-MOMENT", "SEISMIC-XYZ"),
        ("CTRL-G-RESCALED", "SEISMIC-XYZ-G10"),
        ("CTRL-K-TIMES-WEIGHT-CASE", "SEISMIC-XYZ-G10"),
        ("CTRL-TWO-KEYED-ON-MAPPED", "TWO-SEISMIC-XYZ"),
        ("BND-ZERO-SEISMIC-XYZ-G10", "CTRL-ZERO-AS-NOT-APPLIED"),
    ];
    for route in [Route::Pf, Route::Ex] {
        for model in [Model::Cant, Model::Fixed] {
            for mode in MODES {
                let solved_cases = solve_all(route, model, mode);
                for (control, document_case) in controls {
                    let control_id = format!("SCW-{}-{}-{control}", model_tag(model), route_tag(route));
                    let Some(control_ref) = refs()["cases"].as_array().unwrap().iter().find(|c| c["id"] == control_id.as_str()) else {
                        continue;
                    };
                    let reference_id = format!("SCW-{}-{}-{document_case}", model_tag(model), route_tag(route));
                    let (_, out, _) = solved_cases.iter().find(|(id, _, _)| *id == reference_id).unwrap();
                    let (n, failures) = measure(out, control_ref, &format!("case:{reference_id}"));
                    assert_eq!(n, control_ref["rows"].as_array().unwrap().len(), "{control_id}: every row is compared");
                    assert!(!failures.is_empty(), "{control_id} {}: the product reproduces the control", mode.as_str());
                }
            }
        }
    }
}

// ------------------------------------------------------------------ the rule's edges
/// A zero g-factor keeps the replacement: entering k_x = 0 beside k_y gives
/// exactly the SEISMIC-Y reference (BC carries no x load and C an x force of
/// zero), not a member load.
#[test]
fn a_zero_g_factor_keeps_the_component_replacement() {
    for route in [Route::Pf, Route::Ex] {
        let id = format!("SCW-CANT-{}-SEISMIC-Y", route_tag(route));
        let mut case = case_of(reference(&id), "case:zero-x");
        case["equivalent_static"]["seismic"]["g_factor_x"] = qv(0.0, "1");
        let wire = document(route, Model::Cant, ONE_MAPPED, vec![case]);
        for mode in MODES {
            let out = solved(&wire, mode, "zero k_x");
            let (n, failures) = measure(&out, reference(&id), "case:zero-x");
            assert!(n == reference(&id)["rows"].as_array().unwrap().len() && failures.is_empty(), "{id}: {n} rows, {failures:?}");
        }
    }
}

/// VER-005: generation is the same with and without an applied weight case
/// in the document (the selection is independent of any earlier selection
/// or applied weight case), bit for bit. The weight case is the self-weight
/// plan's own, applied unmodified: the product inspects each of its
/// primitives as a fresh application of the plan (`inspect_applied_self_weight`),
/// the pipes' distributed weight and the valve's wrench at C.
#[test]
fn generation_is_independent_of_an_applied_weight_case() {
    use open_pipe_stress_product_physics::self_weight::{generate_self_weight_operations, inspect_applied_self_weight, AppliedSelfWeightState};
    for route in [Route::Pf, Route::Ex] {
        let id = format!("SCW-CANT-{}-SEISMIC-XYZ", route_tag(route));
        let alone = document(route, Model::Cant, ONE_MAPPED, vec![case_of(reference(&id), "case:s")]);
        let model: open_pipe_stress_product_physics::PreviewModel = serde_json::from_value(alone["model"].clone()).unwrap();
        let request = serde_json::from_value(json!({"case_id": "case:weight", "label": "invented", "pipe_refs": ["AB", "BC", "CD"],
            "gravity": {"value": -9.79, "unit": "m/s^2", "axis": "global_z"}, "provenance": PROV,
            "source_model_hash": format!("sha256:{}", "a".repeat(64))})).unwrap();
        let plan = generate_self_weight_operations(&model, &request).expect("the plan");
        let mut weight: Value = serde_json::from_str(&plan.changes[0].after).unwrap();
        weight["primitive_loads"] = plan.changes[1..].iter().map(|c| serde_json::from_str::<Value>(&c.after).unwrap()).collect::<Vec<_>>().into();
        let beside = document(route, Model::Cant, ONE_MAPPED, vec![weight.clone(), case_of(reference(&id), "case:s")]);
        let typed: open_pipe_stress_product_physics::PreviewModel = serde_json::from_value(beside["model"].clone()).unwrap();
        let states = inspect_applied_self_weight(&typed, None);
        assert_eq!(states.len(), weight["primitive_loads"].as_array().unwrap().len(), "every primitive is the plan's");
        assert!(states.len() >= 3, "AB, CD and the valve's wrench: {}", states.len());
        assert!(states.iter().all(|s| s.case_id == "case:weight" && s.state == AppliedSelfWeightState::Fresh), "{:?}", states.iter().map(|s| &s.state).collect::<Vec<_>>());
        for mode in MODES {
            // A document with several cases prefixes each row id with its case.
            let rows = |out: &Value| out["results"].as_array().unwrap().iter()
                .filter(|r| r["basis_ref"]["ref_id"] == "case:s")
                .map(|r| (r["id"].as_str().unwrap().replace("result:loadcase:case-s:", "result:"), r["value"].as_f64().unwrap().to_bits()))
                .collect::<Vec<_>>();
            let (a, b) = (solved(&alone, mode, "alone"), solved(&beside, mode, "beside a weight case"));
            assert!(!rows(&a).is_empty());
            assert_eq!(rows(&a), rows(&b), "{id} {}", mode.as_str());
        }
    }
}

/// A rigid-family mapping conflict on a member blocks by name.
#[test]
fn a_mapping_conflict_is_refused_by_name() {
    for route in [Route::Pf, Route::Ex] {
        let id = format!("SCW-CANT-{}-SEISMIC-XYZ", route_tag(route));
        let mut wire = document(route, Model::Cant, ONE_MAPPED, vec![case_of(reference(&id), "case:s")]);
        let second = component("component:second", "valve", "C", "BC", 100.0, [0.0, 0.0, 0.1], true, Some(2.0));
        wire["model"]["components"].as_array_mut().unwrap().push(second);
        refused(&wire, "RIGID_COMPONENT_MAPPING_CONFLICT", "two components map BC");
    }
}

/// Issue 452 decision 8: a member replaced by a component force has its mass
/// inputs skipped for the formation of that load. A missing material density
/// on BC does not block the case while the valve replaces BC; with the
/// valve's weight entered as zero, BC keeps its distributed load by
/// fallback, and the same missing density blocks the case by name.
#[test]
fn a_replaced_member_s_mass_is_not_read_and_a_fallback_member_s_is() {
    for route in [Route::Pf, Route::Ex] {
        let id = format!("SCW-CANT-{}-SEISMIC-XYZ", route_tag(route));
        let without_density = |components: Components| {
            let mut wire = document(route, Model::Cant, components, vec![case_of(reference(&id), "case:s")]);
            wire["model"]["pipe_segments"][1]["section"].as_object_mut().unwrap().remove("material_density");
            wire
        };
        for mode in MODES {
            let out = solved(&without_density(ONE_MAPPED), mode, "replaced BC without density");
            let (n, failures) = measure(&out, reference(&id), "case:s");
            assert!(n == reference(&id)["rows"].as_array().unwrap().len() && failures.is_empty(), "{id}: {n} rows, {failures:?}");
        }
        let found = refused(&without_density(ZERO_WEIGHT), "EQUIVALENT_STATIC_INPUT_MISSING", "fallback BC without density");
        assert!(found[0]["affected_refs"].as_array().unwrap().contains(&json!("BC")), "{found:?}");
    }
}

/// The definition's range check: a force (k ≠ 0) or a moment (k ≠ 0 and its
/// coordinate ≠ 0) that is zero or nonfinite after rounding blocks the case
/// by name, with no rows. The checked JSON boundary refuses an unsafe
/// integral float, so no document reaches an overflow; the overflow, the
/// admitted finite nonzero subnormal force and the centre-of-gravity
/// coordinate that normalizes to zero are `seismic_component` unit tests.
#[test]
fn an_out_of_range_component_term_blocks_the_case_by_name() {
    // (what, W in N, the offset's y in m, k_x, the refused part)
    let edits: [(&str, f64, f64, f64, &str); 2] = [
        ("force underflow", 1.0e-300, 0.12, 1.0e-30, "global-x:force"),
        ("moment underflow", 1.0e-300, 1.0e-30, 0.3, "global-x:moment-z"),
    ];
    for route in [Route::Pf, Route::Ex] {
        for (what, weight, y, k, part) in edits {
            let mut wire = document(route, Model::Cant, ONE_MAPPED, vec![seismic_case("case:s", 9.79, [k, 0.0, 0.0])]);
            wire["model"]["components"][0]["geometry"]["weight"] = qv(weight, "N");
            wire["model"]["components"][0]["geometry"]["center_of_gravity"]["y"] = json!(y);
            let found = refused(&wire, RANGE_INVALID, what);
            assert_eq!(found.len(), 1, "{what}: {found:?}");
            assert!(found[0]["id"].as_str().unwrap().ends_with(part), "{what}: {found:?}");
            assert_eq!(found[0]["affected_refs"], json!(["case:s", "component:valve"]), "{what}");
        }
    }
}

/// The two-case order witness (issue 450, answer 16): formation stays inside
/// each case's solve, at the position where generation is formed, so the
/// stopping point and the diagnostic order are those of the case loop: the
/// first refused case in document order is named, by its own diagnostic, and
/// the run stops there. W = 1e-300 N with the offset's x and y 1e-30 m: the
/// z moment of an x force (k_x = 0.3) and of a y force (k_y = 0.35) both round
/// to zero; a case whose only g-factor is zero forms its (zero) terms
/// without a range check and passes.
#[test]
fn a_refusal_stops_the_case_loop_at_its_own_case() {
    let case = |id: &str| {
        let mut case = seismic_case(id, 9.79, match id {
            "case:x" => [0.3, 0.0, 0.0],
            "case:y" => [0.0, 0.35, 0.0],
            _ => [0.0; 3],
        });
        if id == "case:zero" {
            case["equivalent_static"]["seismic"]["g_factor_z"] = qv(0.0, "1");
        }
        case
    };
    for route in [Route::Pf, Route::Ex] {
        for (order, expected) in [(["case:x", "case:y"], "case:x"), (["case:y", "case:x"], "case:y"), (["case:zero", "case:y"], "case:y")] {
            let mut wire = document(route, Model::Cant, ONE_MAPPED, order.iter().map(|id| case(id)).collect());
            wire["model"]["components"][0]["geometry"]["weight"] = qv(1.0e-300, "N");
            wire["model"]["components"][0]["geometry"]["center_of_gravity"]["x"] = json!(1.0e-30);
            wire["model"]["components"][0]["geometry"]["center_of_gravity"]["y"] = json!(1.0e-30);
            for mode in MODES {
                let out = run(&wire, mode);
                assert_eq!(out["status"]["mechanics"], "MODEL_INCOMPLETE", "{order:?}");
                let named: Vec<&Value> = out["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == RANGE_INVALID).collect();
                assert_eq!(named.len(), 1, "{route:?} {order:?} {}: {named:?}", mode.as_str());
                assert_eq!(named[0]["affected_refs"][0], expected, "{route:?} {order:?} {}", mode.as_str());
                assert!(named[0]["id"].as_str().unwrap().ends_with("moment-z"), "{named:?}");
            }
        }
    }
}

// ------------------------------------------------------------------ the record (issue 450 item 4)
const EXACT_MASS_METHOD: &str = "source_od_nominal_wall_pressure_bore_areas/v1";

fn exact_case<'a>(out: &'a Value, case: &str) -> &'a Value {
    out["contract_evidence"]["exact_cases"].as_array().unwrap().iter().find(|c| c["load_case_id"] == case).unwrap()
}
fn from_bits(text: &Value) -> f64 {
    f64::from_bits(u64::from_str_radix(text.as_str().unwrap(), 16).unwrap())
}

/// On the exact route every component case records, per entered axis, the
/// component, node, axis, factor, weight, offset, force and moments; the
/// force and moments equal the reference's `load/C/*` rows (within the
/// criterion, the reference being exact), and every load id is generated.
#[test]
fn the_exact_record_carries_each_component_force_and_reproduces_the_reference_loads() {
    let id = "SCW-CANT-EX-SEISMIC-XYZ";
    let r = reference(id);
    let wire = document(Route::Ex, Model::Cant, ONE_MAPPED, vec![case_of(r, "case:s")]);
    for mode in MODES {
        let out = solved(&wire, mode, id);
        let record = &exact_case(&out, "case:s")["equivalent_static"];
        assert_eq!(record["mass_method"], EXACT_MASS_METHOD);
        let entries = record["component_forces"].as_array().unwrap();
        assert_eq!(entries.len(), 3, "one entry per entered axis");
        let ids: Vec<&Value> = record["generated_load_ids"].as_array().unwrap().iter().collect();
        let row = |name: &str| r["rows"].as_array().unwrap().iter().find(|x| x["row"] == name).unwrap()["value"].as_f64().unwrap();
        let mut moment = [0.0f64; 3];
        for (axis, entry) in ["global_x", "global_y", "global_z"].iter().zip(entries) {
            assert_eq!(entry["axis"], *axis);
            assert_eq!(entry["component_ref"], "component:valve");
            assert_eq!(entry["node_ref"], "C");
            assert_eq!(from_bits(&entry["weight_N"]), 2350.0);
            assert!(ids.contains(&&entry["force"]["load_id"]));
            let letter = &axis[7..];
            let f = from_bits(&entry["force"]["value_N"]);
            let e = row(&format!("load/C/F{letter}"));
            assert!((f - e).abs() <= RELATIVE * e.abs(), "{axis}: {f} vs {e}");
            assert_eq!(entry["moments"].as_array().unwrap().len(), 2, "both transverse coordinates are nonzero");
            for m in entry["moments"].as_array().unwrap() {
                assert!(ids.contains(&&m["load_id"]));
                let i = ["global_x", "global_y", "global_z"].iter().position(|a| m["axis"] == *a).unwrap();
                moment[i] += from_bits(&m["value_N_m"]);
            }
        }
        for (i, letter) in ["x", "y", "z"].iter().enumerate() {
            let e = row(&format!("load/C/M{letter}"));
            assert!((moment[i] - e).abs() <= RELATIVE * e.abs(), "M{letter}: {} vs {e}", moment[i]);
        }
    }
}

/// Every other case's record has exactly today's keys: without a component
/// item there is no `component_forces` (not consumed, zero weight). The test
/// compares keys, not bytes; that the rest of the record is unchanged rests on
/// the producer's construction (`exact_equivalent_static_record` adds only
/// that key, and only when a component item is formed).
#[test]
fn a_case_without_a_component_item_keeps_its_earlier_record() {
    for (components, name) in [(NONE_CONSUMED, "BND-NONE-SEISMIC-XYZ-G10"), (ZERO_WEIGHT, "CTRL-ZERO-AS-NOT-APPLIED")] {
        let id = format!("SCW-CANT-EX-{name}");
        let wire = document(Route::Ex, Model::Cant, components, vec![case_of(reference(&id), "case:s")]);
        let out = solved(&wire, PreviewSolverMode::SparseInteractive, &id);
        let record = &exact_case(&out, "case:s")["equivalent_static"];
        let keys: Vec<&String> = record.as_object().unwrap().keys().collect();
        assert_eq!(keys, ["generated_load_ids", "mass_method"], "{id}");
    }
}

/// The package readers check the closed shape and the inventory and claim
/// nothing about the model; the model-bound reader recomputes the entries
/// from the model and compares bits, refusing a changed value, a missing or
/// extra entry and a missing record. A result saved before the unit (no
/// `component_forces` on a case whose model forms one; issue 450 decision 3)
/// is admitted by the package reader and refused where the model-bound check
/// runs, and its standing says recompute.
#[test]
fn the_readers_bind_the_component_record() {
    use open_pipe_stress_result_export::{reader_verdict, semantic_contract};
    let id = "SCW-CANT-EX-SEISMIC-XYZ";
    let wire = document(Route::Ex, Model::Cant, ONE_MAPPED, vec![case_of(reference(id), "case:s")]);
    let model = &wire["model"];
    let out = solved(&wire, PreviewSolverMode::SparseInteractive, id);
    semantic_contract::validate_pressure_evidence(&out).expect("the package reader admits the producer's record");
    assert_eq!(reader_verdict::validate("pressure", &out, Some(model), None)["ok"], true, "the model-bound reader admits it");
    let case = out["contract_evidence"]["exact_cases"].as_array().unwrap().iter().position(|c| c["load_case_id"] == "case:s").unwrap();
    let forge = |edit: &dyn Fn(&mut Value)| {
        let mut forged = out.clone();
        edit(&mut forged["contract_evidence"]["exact_cases"][case]["equivalent_static"]);
        forged
    };
    // Shape and inventory: refused by the package reader itself.
    type Edit = Box<dyn Fn(&mut Value)>;
    let shape: Vec<(&str, Edit)> = vec![
        ("extra entry key", Box::new(|r| r["component_forces"][0]["extra"] = json!(1))),
        ("decimal value", Box::new(|r| r["component_forces"][0]["force"]["value_N"] = json!("705"))),
        ("uppercase hex", Box::new(|r| { let v = r["component_forces"][0]["weight_N"].as_str().unwrap().to_uppercase(); r["component_forces"][0]["weight_N"] = json!(v) })),
        ("moment about its own axis", Box::new(|r| r["component_forces"][0]["moments"][0]["axis"] = json!("global_x"))),
        ("id outside the inventory", Box::new(|r| r["component_forces"][0]["force"]["load_id"] = json!("case:s:generated:seismic:global-x:other"))),
        ("id twice", Box::new(|r| { let m = r["component_forces"][0]["moments"][0]["load_id"].clone(); r["component_forces"][0]["moments"][1]["load_id"] = m })),
        ("empty list", Box::new(|r| r["component_forces"] = json!([]))),
        // Round 1 finding 3: each id is the one the entry's own fields form.
        ("moment id of another moment axis", Box::new(|r| {
            let id = r["component_forces"][0]["force"]["load_id"].as_str().unwrap().replace(":force", ":moment:x");
            r["component_forces"][0]["moments"][0]["load_id"] = json!(id);
            r["generated_load_ids"].as_array_mut().unwrap().push(json!(id));
        })),
        ("force id of another component", Box::new(|r| {
            let id = "case:s:generated:seismic:global-x:component:16:component:flange:force";
            r["component_forces"][0]["force"]["load_id"] = json!(id);
            r["generated_load_ids"].as_array_mut().unwrap().push(json!(id));
        })),
    ];
    for (what, edit) in &shape {
        let error = semantic_contract::validate_pressure_evidence(&forge(edit.as_ref())).expect_err(what);
        assert!(error.contains("EQUIVALENT_STATIC"), "{what}: {error}");
    }
    // Values: admitted by the package reader, refused with the model.
    let values: Vec<(&str, Edit, &str)> = vec![
        ("force one bit off", Box::new(|r| {
            let v = from_bits(&r["component_forces"][0]["force"]["value_N"]);
            r["component_forces"][0]["force"]["value_N"] = json!(format!("{:016x}", v.to_bits() + 1));
        }), "EQUIVALENT_STATIC_COMPONENT_MODEL_MISMATCH"),
        ("entry missing", Box::new(|r| { r["component_forces"].as_array_mut().unwrap().pop(); }), "EQUIVALENT_STATIC_COMPONENT_MODEL_MISMATCH"),
        ("entries reordered", Box::new(|r| r["component_forces"].as_array_mut().unwrap().swap(0, 1)), "EQUIVALENT_STATIC_COMPONENT_MODEL_MISMATCH"),
        ("record saved before the unit", Box::new(|r| { r.as_object_mut().unwrap().remove("component_forces"); }), "EQUIVALENT_STATIC_COMPONENT_RECORD_MISSING"),
    ];
    for (what, edit, code) in &values {
        let forged = forge(edit.as_ref());
        semantic_contract::validate_pressure_evidence(&forged).unwrap_or_else(|e| panic!("{what}: the package reader claims nothing about the model: {e}"));
        let bound = reader_verdict::validate("pressure", &forged, Some(model), None);
        assert_eq!(bound["ok"], false, "{what}: {bound}");
        assert!(bound.to_string().contains(code), "{what}: expected {code}, got {bound}");
        let standing = &reader_verdict::dispatch(&forged, Some(model), None)["standing"];
        assert_eq!(standing["status"], "needs_recompute", "{what}: {standing}");
        assert_eq!(standing["eligible"], false, "{what}");
    }
    // An extra entry: a model whose valve does not replace (zero weight) and a record that says it does.
    let zero = document(Route::Ex, Model::Cant, ZERO_WEIGHT, vec![case_of(reference(id), "case:s")]);
    let bound = reader_verdict::validate("pressure", &out, Some(&zero["model"]), None);
    assert_eq!(bound["ok"], false, "{bound}");
    assert_eq!(bound["error"]["code"], "SOURCE_PHYSICS_EQUIVALENT_STATIC_COMPONENT_MODEL_MISMATCH", "{bound}");
}

/// Issue 450 round 1 finding 3. The model-bound check compares every solved
/// case, so a case that generates no component force has an empty expected
/// list: a wind-only case and a case without generation are each refused when
/// their record carries a well-formed component entry. The package readers
/// refuse the reviewer's forgery (a wind-only case's entry naming one of its
/// wind primitives, with no moments) by the id's form alone. A model whose
/// required component force rounds to zero is one the product refuses, so no
/// solved record of it exists, and the model-bound check refuses it.
#[test]
fn a_component_record_on_a_case_that_generates_none_is_refused() {
    use open_pipe_stress_result_export::{reader_verdict, semantic_contract};
    let id = "SCW-CANT-EX-SEISMIC-XYZ";
    let wind = json!({"id": "case:w", "primitive_loads": [], "provenance": PROV, "equivalent_static": {"wind": {
        "pressure": qv(500.0, "Pa"), "shape_factor": qv(0.7, "1"), "direction": "global_z", "exposed_pipe_refs": ["AB"]},
        "provenance": PROV}});
    let plain = json!({"id": "case:p", "provenance": PROV, "primitive_loads": [{"id": "load:p", "category": "concentrated_force",
        "target": {"type": "node", "node": "C"}, "direction": "global_z", "magnitude": qv(-10.0, "N"), "dimension": "force", "provenance": PROV}]});
    let wire = document(Route::Ex, Model::Cant, ONE_MAPPED, vec![case_of(reference(id), "case:s"), wind, plain]);
    let model = &wire["model"];
    let out = solved(&wire, PreviewSolverMode::SparseInteractive, id);
    semantic_contract::validate_pressure_evidence(&out).expect("the package reader admits the producer's records");
    assert_eq!(reader_verdict::validate("pressure", &out, Some(model), None)["ok"], true);
    let exact = |out: &Value, case: &str| out["contract_evidence"]["exact_cases"].as_array().unwrap().iter().position(|c| c["load_case_id"] == case).unwrap();
    let (s, w, p) = (exact(&out, "case:s"), exact(&out, "case:w"), exact(&out, "case:p"));
    let wind_ids = out["contract_evidence"]["exact_cases"][w]["equivalent_static"]["generated_load_ids"].clone();
    assert!(wind_ids.as_array().is_some_and(|ids| !ids.is_empty()), "the wind case generates");
    assert!(out["contract_evidence"]["exact_cases"][w]["equivalent_static"].get("component_forces").is_none());
    assert!(out["contract_evidence"]["exact_cases"][p].get("equivalent_static").is_none());
    // The seismic case's x entry, re-identified for another case.
    let entry_for = |case: &str| {
        let text = out["contract_evidence"]["exact_cases"][s]["equivalent_static"]["component_forces"][0].to_string();
        serde_json::from_str::<Value>(&text.replace("case:s:generated:", &format!("{case}:generated:"))).unwrap()
    };
    let ids_of = |entry: &Value| {
        let mut ids = vec![entry["force"]["load_id"].clone()];
        ids.extend(entry["moments"].as_array().unwrap().iter().map(|m| m["load_id"].clone()));
        ids
    };
    // The reviewer's forgery: refused by the package reader by the id's form.
    let mut forged = out.clone();
    let record = &mut forged["contract_evidence"]["exact_cases"][w]["equivalent_static"];
    let mut entry = entry_for("case:w");
    entry["force"]["load_id"] = wind_ids[0].clone();
    entry["moments"] = json!([]);
    record["component_forces"] = json!([entry]);
    let error = semantic_contract::validate_pressure_evidence(&forged).expect_err("a wind id is not a component id");
    assert!(error.contains("EQUIVALENT_STATIC_COMPONENT_LOAD_IDS"), "{error}");
    // A well-formed entry on the wind-only case: the package reader admits it, the model-bound check refuses it.
    let mut forged = out.clone();
    let record = &mut forged["contract_evidence"]["exact_cases"][w]["equivalent_static"];
    let entry = entry_for("case:w");
    record["generated_load_ids"].as_array_mut().unwrap().extend(ids_of(&entry));
    record["component_forces"] = json!([entry]);
    semantic_contract::validate_pressure_evidence(&forged).unwrap_or_else(|e| panic!("well-formed: {e}"));
    let bound = reader_verdict::validate("pressure", &forged, Some(model), None);
    assert_eq!(bound["ok"], false, "{bound}");
    assert!(bound.to_string().contains("EQUIVALENT_STATIC_COMPONENT_MODEL_MISMATCH"), "{bound}");
    // The same on the case without generation, given a record of its own.
    let mut forged = out.clone();
    let entry = entry_for("case:p");
    let mut ids = vec![json!("case:p:generated:seismic:global-x:element-0")];
    ids.extend(ids_of(&entry));
    forged["contract_evidence"]["exact_cases"][p]["equivalent_static"] = json!({"mass_method": "source_od_nominal_wall_pressure_bore_areas/v1",
        "generated_load_ids": ids, "component_forces": [entry]});
    let bound = reader_verdict::validate("pressure", &forged, Some(model), None);
    assert_eq!(bound["ok"], false, "{bound}");
    assert_eq!(bound["error"]["code"], "SOURCE_PHYSICS_EQUIVALENT_STATIC_COMPONENT_MODEL_MISMATCH", "{bound}");
    // A model whose required force rounds to zero (k = 0.3, W the smallest positive binary64 in N).
    let mut range = model.clone();
    range["components"][0]["geometry"]["weight"] = qv(f64::from_bits(1), "N");
    assert_eq!(0.3 * f64::from_bits(1), 0.0);
    let bound = reader_verdict::validate("pressure", &out, Some(&range), None);
    assert_eq!(bound["ok"], false, "{bound}");
    assert!(bound.to_string().contains("EQUIVALENT_STATIC_COMPONENT_RANGE"), "{bound}");
}

/// Issue 450 round 1 finding 2: at the document boundary a centre-of-gravity
/// coordinate of 1e-321 mm converts to 0 m (it is below half the smallest
/// positive binary64 value, about 2.5e-324 m) in the solve's ordinary unit
/// conversion, before generation reads it. On an eligible component, on one
/// that falls back (zero weight) and on a `not_solver_consumed` one, the case
/// solves on both routes and in both modes; no recorded moment uses that
/// coordinate (none is zero); and the model-bound reader, which reads the
/// offset after the same conversion, admits the record. The self-weight
/// plan, which reads the entered value (REQ-015's block), refuses the same
/// document with SELF_WEIGHT_RANGE_INVALID.
#[test]
fn a_coordinate_that_converts_to_zero_forms_no_moment_and_the_reader_agrees() {
    use open_pipe_stress_result_export::{reader_verdict, semantic_contract};
    let id = "SCW-CANT-EX-SEISMIC-XYZ";
    let tiny = 1.0e-321;
    assert_eq!(tiny / 1000.0, 0.0, "the coordinate converts to zero metres");
    let variants = [
        ("eligible", ONE_MAPPED, Some([1usize, 2, 1])),
        ("falls back", ZERO_WEIGHT, None),
        ("not consumed", Components { valve_weight: 2350.0, valve_consumed: false, flange: false }, None),
    ];
    for (what, components, moments) in variants {
        let wire = |route| {
            let mut wire = document(route, Model::Cant, components, vec![case_of(reference(id), "case:s")]);
            wire["model"]["components"][0]["geometry"]["center_of_gravity"] = json!({"x": -175.0, "y": tiny, "z": 125.0, "unit": "mm"});
            wire
        };
        for route in [Route::Pf, Route::Ex] {
            for mode in MODES {
                let out = solved(&wire(route), mode, what);
                if route == Route::Pf {
                    continue;
                }
                let record = &out["contract_evidence"]["exact_cases"].as_array().unwrap().iter()
                    .find(|c| c["load_case_id"] == "case:s").unwrap()["equivalent_static"];
                match moments {
                    Some(counts) => {
                        let entries = record["component_forces"].as_array().unwrap();
                        let got: Vec<usize> = entries.iter().map(|e| e["moments"].as_array().unwrap().len()).collect();
                        // r_y converts to 0: the x force's moment about z and the
                        // z force's moment about x are not formed.
                        assert_eq!(got, counts, "{what} {mode:?}: {entries:?}");
                        for e in entries {
                            for m in e["moments"].as_array().unwrap() {
                                assert_ne!(from_bits(&m["value_N_m"]), 0.0, "{what}: {m}");
                            }
                            assert_eq!(e["center_of_gravity_m"][1], json!("0000000000000000"), "{what}");
                        }
                    }
                    None => assert!(record.get("component_forces").is_none(), "{what} {mode:?}: {record}"),
                }
                semantic_contract::validate_pressure_evidence(&out).unwrap_or_else(|e| panic!("{what}: {e}"));
                let model = wire(route)["model"].clone();
                assert_eq!(reader_verdict::validate("pressure", &out, Some(&model), None)["ok"], true, "{what} {mode:?}: the model-bound reader admits it");
            }
        }
    }
    // The self-weight plan reads the entered value, and its own block fires.
    let mut wire = document(Route::Pf, Model::Cant, ONE_MAPPED, vec![]);
    wire["model"]["components"][0]["geometry"]["center_of_gravity"] = json!({"x": -175.0, "y": tiny, "z": 125.0, "unit": "mm"});
    let model: open_pipe_stress_product_physics::PreviewModel = serde_json::from_value(wire["model"].clone()).unwrap();
    let request = serde_json::from_value(json!({"case_id": "case:weight", "label": "invented", "pipe_refs": ["AB", "BC", "CD"],
        "gravity": {"value": -9.79, "unit": "m/s^2", "axis": "global_z"}, "provenance": PROV,
        "source_model_hash": format!("sha256:{}", "a".repeat(64))})).unwrap();
    let refused = open_pipe_stress_product_physics::self_weight::generate_self_weight_operations(&model, &request).expect_err("the plan's block fires");
    assert!(!refused.is_empty() && refused.iter().all(|d| d.code == "SELF_WEIGHT_RANGE_INVALID"), "{refused:?}");
}

// ------------------------------------------------------------------ the shared record corpus
const RECORD_CORPUS_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../fixtures/results/seismic_component_record_corpus.json");

/// The shared component record corpus (issue 450 round 2 item 2): solved
/// exact-route envelopes with their models, read by
/// `tests/test_seismic_component_record_reader.py`, which runs the same
/// forgeries through the Rust model-bound reader (the runner's `read
/// pressure`) and its Python mirror (`tests/verifiers/reader_verdict.py`).
/// The first holds a seismic case, a wind-only case and a case without
/// generation; the second a centre-of-gravity coordinate that converts to
/// zero metres. Regenerate with `SCW_WRITE_CORPUS=1`; otherwise the committed
/// bytes must be current.
#[test]
fn shared_component_record_corpus_is_current() {
    let id = "SCW-CANT-EX-SEISMIC-XYZ";
    let wind = json!({"id": "case:w", "primitive_loads": [], "provenance": PROV, "equivalent_static": {"wind": {
        "pressure": qv(500.0, "Pa"), "shape_factor": qv(0.7, "1"), "direction": "global_z", "exposed_pipe_refs": ["AB"]},
        "provenance": PROV}});
    let plain = json!({"id": "case:p", "provenance": PROV, "primitive_loads": [{"id": "load:p", "category": "concentrated_force",
        "target": {"type": "node", "node": "C"}, "direction": "global_z", "magnitude": qv(-10.0, "N"), "dimension": "force", "provenance": PROV}]});
    let three = document(Route::Ex, Model::Cant, ONE_MAPPED, vec![case_of(reference(id), "case:s"), wind, plain]);
    let mut tiny = document(Route::Ex, Model::Cant, ONE_MAPPED, vec![case_of(reference(id), "case:s")]);
    tiny["model"]["components"][0]["geometry"]["center_of_gravity"] = json!({"x": -175.0, "y": 1.0e-321, "z": 125.0, "unit": "mm"});
    let mut cases = Vec::new();
    for (label, wire) in [
        ("seismic, wind-only and plain cases", three),
        ("a centre-of-gravity coordinate that converts to zero metres", tiny),
    ] {
        let envelope = solved(&wire, PreviewSolverMode::SparseInteractive, label);
        cases.push(json!({"label": label, "model": wire["model"], "envelope": envelope}));
    }
    let corpus = json!({
        "corpus": "DEL-05-01 seismic component record reader corpus",
        "generator": "core/product_physics/tests/seismic_component_weight.rs::shared_component_record_corpus_is_current",
        "cases": cases,
    });
    let bytes = serde_json::to_string_pretty(&corpus).unwrap() + "\n";
    if std::env::var_os("SCW_WRITE_CORPUS").is_some() {
        std::fs::write(RECORD_CORPUS_PATH, &bytes).unwrap();
        return;
    }
    // Read at compile time, so the numerical CI's read index binds it to this test.
    let committed = include_str!("../../../fixtures/results/seismic_component_record_corpus.json");
    assert!(committed == bytes, "the shared component record corpus is stale; regenerate it with SCW_WRITE_CORPUS=1");
}

// ------------------------------------------------------------------ the rule's value
fn witness_nodal(id: &str, dof: &str, value: f64) -> Value {
    let (category, unit, dimension) = if dof.starts_with("rotation") { ("concentrated_moment", "N*m", "moment") } else { ("concentrated_force", "N", "force") };
    json!({"id": id, "category": category, "target": {"type": "node", "node": "C"}, "direction": dof,
           "magnitude": qv(value, unit), "dimension": dimension, "provenance": PROV})
}
/// One member B - C anchored at B, the valve at C mapped to BC (whose
/// distributed load it replaces, so the component's terms are the case's
/// only seismic load), and the one case `case`.
fn witness(route: Route, weight: f64, cog: [f64; 3], case: Value) -> Value {
    let mut doc = document(route, Model::Cant, Components { valve_weight: weight, valve_consumed: true, flange: false }, vec![case]);
    let m = &mut doc["model"];
    let keep = |v: &mut Value, ids: &[&str]| v.as_array_mut().unwrap().retain(|x| ids.contains(&x["id"].as_str().unwrap()));
    keep(&mut m["nodes"], &["B", "C"]);
    keep(&mut m["pipe_segments"], &["BC"]);
    m["supports"] = json!([{"id": "support:B", "node": "B", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV}]);
    m["components"][0]["geometry"]["center_of_gravity"] = json!({"x": cog[0], "y": cog[1], "z": cog[2], "unit": "m"});
    doc
}
/// What the comparison reads: every published row (its id and value bits)
/// and the published standing.
fn published(out: &Value) -> (Vec<(String, u64)>, Value) {
    assert_eq!(out["status"]["mechanics"], "MECHANICS_SOLVED", "{:?}", codes(out));
    let rows = out["results"].as_array().unwrap().iter()
        .map(|r| (r["id"].as_str().unwrap().to_string(), r["value"].as_f64().unwrap().to_bits())).collect();
    (rows, out["numerical_quality"].clone())
}

/// The reviewer's witness of issue 450 (round 1, finding 1). The rule's
/// value is the once-rounded product: F = fl(k W) and each moment fl(F r_j)
/// from that rounded force. The product publishes the response to exactly
/// that value: the generated load is an exact stored term of the certified
/// ledger (as the self-weight plan's component wrench is), bound to its
/// operands where the ledger is built, not a product the certificate
/// re-forms from its operands. So, on both routes and in both solver modes,
/// a case whose component load is generated publishes the same rows, bit for
/// bit, and the same standing as the case with the rule's value entered as a
/// plain primitive, with and without an entered load that cancels it
/// exactly. Before the repair the certificate's exact product was
/// published: with the cancelling load the force witness was demoted to
/// `sensitive` (formation_check at C:UX against a zero scale) and the moment
/// witness published a support moment of 2.78e-17 N*m where the rule's net
/// is 0; without it the moment witness's support moment was -0.3, not the
/// rule's -fl(3 * 0.1) = -0.30000000000000004.
/// - Force: k = 0.1, W = 10 N, r = 0; fl(k W) = 1.0 (the exact product of
///   the binary64 operands is 1 + 5.55e-17); the cancelling load is -1 N
///   at C global_x.
/// - Moment: k = 0.25, W = 12 N, r = (0, 0, 0.1) m; F = 3 exactly, My =
///   fl(3 * 0.1) = 0.30000000000000004 (the exact product is
///   0.30000000000000001665); the cancelling load is -fl(3 * 0.1) at C
///   rotation_y.
#[test]
fn a_generated_component_load_publishes_the_rule_s_value() {
    let id = "case:w";
    let generated = |k: f64, loads: Vec<Value>| {
        let mut case = seismic_case(id, 9.79, [k, 0.0, 0.0]);
        case["primitive_loads"] = Value::Array(loads);
        case
    };
    let entered = |loads: Vec<Value>| json!({"id": id, "primitive_loads": loads, "provenance": PROV});
    let my = 3.0 * 0.1_f64;
    assert_eq!(my.to_bits(), 0.30000000000000004_f64.to_bits());
    // (what, weight, cog, k, the rule's value entered, the cancelling load)
    let witnesses: [(&str, f64, [f64; 3], f64, Vec<(&str, f64)>, (&str, f64)); 2] = [
        ("force", 10.0, [0.0, 0.0, 0.0], 0.1, vec![("global_x", 0.1 * 10.0)], ("global_x", -1.0)),
        ("moment", 12.0, [0.0, 0.0, 0.1], 0.25, vec![("global_x", 3.0), ("rotation_y", my)], ("rotation_y", -my)),
    ];
    for (what, weight, cog, k, rule, cancel) in witnesses {
        let rule_loads = |tag: &str| rule.iter().enumerate().map(|(i, (dof, v))| witness_nodal(&format!("load:{tag}:rule:{i}"), dof, *v)).collect::<Vec<_>>();
        let cancelling = |tag: &str| witness_nodal(&format!("load:{tag}:cancel"), cancel.0, cancel.1);
        for route in [Route::Pf, Route::Ex] {
            for mode in MODES {
                let solve = |case: Value| published(&run(&witness(route, weight, cog, case), mode));
                let a = solve(generated(k, vec![cancelling("a")]));
                let b = solve(generated(k, vec![]));
                let c = solve(entered([rule_loads("c"), vec![cancelling("c")]].concat()));
                let d = solve(entered(rule_loads("d")));
                assert_eq!(a, c, "{what} {route:?} {mode:?}: generated and cancelled, against the rule's value entered and cancelled");
                assert_eq!(b, d, "{what} {route:?} {mode:?}: generated, against the rule's value entered");
                assert_eq!(a.1["status"], "checks_passed", "{what} {route:?} {mode:?}");
                assert_eq!(b.1["status"], "checks_passed", "{what} {route:?} {mode:?}");
            }
        }
    }
}
