//! W-16 slice a, stage 1 (DEL-04-03 Design `pf-imposed-displacement.md` §5):
//! imposed displacement on the pressure-free route (PF).
//!
//! The nozzle movement is read from the frozen references in
//! `validation/references/w16_imposed_motion/references.json` (a six-redundant
//! force method in exact rationals with an independent direct-stiffness
//! cross-check; see that package's README), pinned to the hash recorded in its
//! SHA256SUMS. Nothing here derives an expectation from product code. Model
//! inputs are invented.
//!
//! - The oracle: motion alone, thermal alone and both together, on PF (support
//!   primitive loads) and on the exact route (EX, `boundary_motion`, Euler-
//!   Bernoulli), both solver modes; then PF against EX row for row.
//! - Every refusal of the design's §3, the EX refusal, the retained sub-route's
//!   decline, and the allowed cases.
//! - Linear combination, subtraction and range envelope with motion cases.
//!
//! Criterion (DEC-024/DEC-026, unchanged): |obs - exp| <= 1e-9 |exp|; an
//! exactly zero expectation uses 1e-9 times the case's frozen zero scale of its
//! family.
use open_pipe_stress_product_physics::self_weight::{generate_self_weight_operations, SelfWeightGravity, SelfWeightRequest};
use open_pipe_stress_product_physics::{
    run_linear_static_preview, run_linear_static_preview_value_with_mode,
    run_linear_static_preview_value_with_retained_direct, LinearStaticPreviewRequest, PreviewModel, PreviewSolverMode,
    RetainedByteLimit,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const REFERENCES: &str = include_str!("../../../validation/references/w16_imposed_motion/references.json");
const SUMS: &str = include_str!("../../../validation/references/w16_imposed_motion/SHA256SUMS");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const RELATIVE: f64 = 1.0e-9;
const PROV: &str = "w16 invented benchmark input; not library, component, material-catalog or code data";
const CASE: &str = "case:w16";
const PF_SEMANTICS: &str = "openpipestress.result_semantics/0.3.0/preview-physics-2";
const EX_SEMANTICS: &str = "openpipestress.result_semantics/0.3.0/pressure-2";
const STATE_CONTRACT: &str = "openpipestress.load_reference_state/1.0.0";
const REFERENCE: &str = "reference:installed";
const MOTION: &str = "W16-MOTION";
const THERMAL: &str = "W16-THERMAL";
const BOTH: &str = "W16-MOTION-THERMAL";
const RETAINED_LIMIT: RetainedByteLimit = RetainedByteLimit::new(12_884_901_888);

#[derive(Clone, Copy, PartialEq, Debug)]
enum Route {
    Preview,
    Exact,
}

// ------------------------------------------------------------------ references
fn refs() -> Value {
    serde_json::from_str(REFERENCES).expect("references.json parses")
}
fn case_ref(id: &str) -> Value {
    refs()["cases"].as_array().unwrap().iter().find(|c| c["id"] == id).unwrap_or_else(|| panic!("no case {id}")).clone()
}
fn exact_rational(v: &Value) -> f64 {
    v["value"].as_f64().unwrap_or_else(|| panic!("not a value record: {v}"))
}
fn input_f64(text: &Value) -> f64 {
    let s = text.as_str().unwrap_or_else(|| panic!("input text {text}"));
    match s.split_once('/') {
        Some((p, q)) => p.parse::<f64>().unwrap() / q.parse::<f64>().unwrap(),
        None => s.parse().unwrap(),
    }
}

#[test]
fn references_are_the_frozen_bytes() {
    let digest: String = Sha256::digest(REFERENCES.as_bytes()).iter().map(|b| format!("{b:02x}")).collect();
    let line = SUMS.lines().find(|l| l.ends_with("  references.json")).expect("SHA256SUMS lists references.json");
    assert_eq!(&line[..64], digest, "the references must equal the frozen bytes");
    let doc = refs();
    assert!(doc["crosscheck"].as_array().unwrap().iter().all(|c| c["agrees"] == true));
    assert_eq!(doc["cases"].as_array().unwrap().len(), 3);
}

// ------------------------------------------------------------------ comparison
fn tolerance(expected: f64, zero_scale: f64) -> f64 {
    RELATIVE * if expected == 0.0 { zero_scale.abs() } else { expected.abs() }
}
fn close(observed: f64, expected: f64, zero_scale: f64, what: &str) {
    let tol = tolerance(expected, zero_scale);
    assert!(
        observed.is_finite() && (observed - expected).abs() <= tol,
        "{what}: observed {observed:.17e} expected {expected:.17e} tolerance {tol:.3e}"
    );
}

fn rows(out: &Value) -> &Vec<Value> {
    out["results"].as_array().expect("results")
}

/// One published row: (value, unit) in the reference's SI unit (displacements
/// are published in mm). `basis` is ("load_case" | "combination", id).
fn observed(out: &Value, basis: (&str, &str), r: &Value) -> f64 {
    let (entity, component, unit) = (r["entity"].as_str().unwrap(), r["component"].as_str().unwrap(), r["unit"].as_str().unwrap());
    let find = |kind: &str, component: &str, location: Option<&str>| -> (f64, String) {
        let found: Vec<&Value> = rows(out)
            .iter()
            .filter(|row| {
                row["kind"] == kind
                    && row["entity_ref"] == entity
                    && row["basis_ref"]["ref_type"] == basis.0
                    && row["basis_ref"]["ref_id"] == basis.1
                    && row["metadata"]["component"] == component
                    && location.is_none_or(|l| row["metadata"]["location"] == l)
            })
            .collect();
        assert_eq!(found.len(), 1, "one row {kind} {entity} {component} {location:?} on {basis:?}");
        (found[0]["value"].as_f64().unwrap(), found[0]["unit"].as_str().unwrap().to_string())
    };
    match r["quantity"].as_str().unwrap() {
        "support_reaction" => {
            let (v, u) = find("support_reaction_component_v2", component, None);
            assert_eq!(u, unit);
            v
        }
        "nodal_motion" => {
            let axis = &component[1..];
            if component.starts_with('u') {
                let (v, u) = find(&format!("global_nodal_displacement_{axis}"), &format!("nodal_displacement_{axis}"), None);
                assert_eq!(u, "mm");
                v / 1000.0
            } else {
                let (v, u) = find(&format!("global_nodal_rotation_{axis}"), &format!("nodal_rotation_{axis}"), None);
                assert_eq!(u, unit);
                v
            }
        }
        "section_action" => {
            let (location, sign) = match r["station"].as_str().unwrap() {
                "0" => ("end_i", -1.0),
                "1/4" => ("quarter_1", 1.0),
                "1/2" => ("midspan", 1.0),
                "3/4" => ("quarter_3", 1.0),
                "1" => ("end_j", 1.0),
                s => panic!("station {s}"),
            };
            let (v, u) = find(&format!("element_local_{component}"), component, Some(location));
            assert_eq!(u, unit);
            sign * v
        }
        q => panic!("quantity {q}"),
    }
}

fn zero_scale(reference: &Value, unit: &str) -> f64 {
    let family = match unit {
        "N" => "force",
        "N*m" => "moment",
        "m" => "translation",
        "rad" => "rotation",
        other => panic!("unit {other}"),
    };
    exact_rational(&reference["zero_scales"][family])
}

fn label(r: &Value) -> String {
    format!("{} {} {} {}", r["quantity"].as_str().unwrap(), r["entity"].as_str().unwrap(), r["station"].as_str().unwrap_or(""), r["component"].as_str().unwrap())
}

/// Every frozen row of `reference`, with `expected` mapping a row to its
/// expectation (the frozen value by default). Returns the row count, the
/// largest relative error (nonzero rows) and the largest |obs|/zero_scale
/// (zero rows).
fn compare_with(out: &Value, reference: &Value, basis: (&str, &str), what: &str, expected: impl Fn(&Value) -> f64) -> (usize, f64, f64) {
    let (mut count, mut rel, mut zero) = (0, 0.0f64, 0.0f64);
    for r in reference["rows"].as_array().unwrap() {
        let zs = zero_scale(reference, r["unit"].as_str().unwrap());
        let (o, e) = (observed(out, basis, r), expected(r));
        close(o, e, zs, &format!("{what} {}", label(r)));
        if e == 0.0 { zero = zero.max((o / zs).abs()) } else { rel = rel.max(((o - e) / e).abs()) }
        count += 1;
    }
    (count, rel, zero)
}
fn compare(out: &Value, reference: &Value, basis: (&str, &str), what: &str) -> (usize, f64, f64) {
    compare_with(out, reference, basis, what, exact_rational)
}

// ------------------------------------------------------------------ model authoring
struct Inputs {
    nodes: Vec<(String, [f64; 3])>,
    pipes: Vec<(String, String, String, [f64; 3])>,
    supports: Vec<(String, String, Vec<String>)>,
    od: f64,
    wall: f64,
    e: f64,
    g: f64,
    nu: f64,
    /// (dof, value, unit) at `support:d`.
    motion: Vec<(String, f64, String)>,
    alpha: f64,
    delta_t: f64,
}

fn inputs() -> Inputs {
    let reference = case_ref(BOTH);
    let i = &reference["inputs"];
    let g = &i["geometry"];
    let s = &i["section"];
    let vec3 = |v: &Value| -> [f64; 3] {
        let a = v.as_array().unwrap();
        [a[0].as_f64().unwrap(), a[1].as_f64().unwrap(), a[2].as_f64().unwrap()]
    };
    Inputs {
        nodes: g["nodes"].as_object().unwrap().iter().map(|(k, v)| (k.clone(), vec3(v))).collect(),
        pipes: g["pipes"].as_object().unwrap().iter()
            .map(|(k, v)| (k.clone(), v["from"].as_str().unwrap().into(), v["to"].as_str().unwrap().into(), vec3(&v["y_reference"])))
            .collect(),
        supports: g["supports"].as_object().unwrap().iter()
            .map(|(k, v)| (k.clone(), v["node"].as_str().unwrap().into(), v["restraints"].as_array().unwrap().iter().map(|r| r.as_str().unwrap().into()).collect()))
            .collect(),
        od: input_f64(&s["outside_diameter_m"]),
        wall: input_f64(&s["wall_thickness_m"]),
        e: input_f64(&s["elastic_modulus_Pa"]),
        g: input_f64(&s["shear_modulus_Pa_preview_route"]),
        nu: input_f64(&s["poisson_ratio_exact_route"]),
        motion: i["support_motion"]["dofs"].as_array().unwrap().iter()
            .map(|d| (d["dof"].as_str().unwrap().into(), input_f64(&d["value"]), d["unit"].as_str().unwrap().into()))
            .collect(),
        alpha: input_f64(&i["thermal"]["alpha_per_K"]),
        delta_t: input_f64(&i["thermal"]["delta_T_K"]),
    }
}

fn qv(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}

fn dimension(dof: &str) -> &'static str {
    if dof.starts_with('U') {
        "displacement"
    } else {
        "rotation"
    }
}

/// One support-targeted primitive (the desktop's wire shape).
fn motion_load(id: &str, support: &str, dof: &str, value: f64, unit: &str) -> Value {
    json!({"id": id, "category": "imposed_displacement", "target": {"type": "support", "support": support, "dof": dof},
        "direction": dof, "magnitude": qv(value, unit), "dimension": dimension(dof), "provenance": PROV})
}
fn motion_loads(i: &Inputs) -> Vec<Value> {
    i.motion.iter().map(|(dof, v, u)| motion_load(&format!("motion:{dof}"), "support:d", dof, *v, u)).collect()
}
fn thermal_loads(i: &Inputs) -> Vec<Value> {
    i.pipes.iter().map(|(id, ..)| json!({"id": format!("thermal:{id}"), "category": "thermal", "target": {"type": "element", "pipe": id},
        "direction": "global_x", "magnitude": qv(i.delta_t, "degC"), "dimension": "temperature_interval", "provenance": PROV})).collect()
}
fn force_load(id: &str, node: &str, direction: &str, value: f64) -> Value {
    json!({"id": id, "category": "concentrated_force", "target": {"type": "node", "node": node}, "direction": direction,
        "magnitude": qv(value, "N"), "dimension": "force", "provenance": PROV})
}
fn case(id: &str, loads: Vec<Value>) -> Value {
    json!({"id": id, "primitive_loads": loads, "provenance": PROV})
}

fn pf_document(i: &Inputs, cases: Vec<Value>, combinations: Vec<Value>, density: Option<f64>) -> Value {
    let nodes: Vec<Value> = i.nodes.iter().map(|(id, p)| json!({"id": id, "position": {"x": p[0], "y": p[1], "z": p[2]}, "provenance": PROV})).collect();
    let pipes: Vec<Value> = i.pipes.iter().map(|(id, from, to, y)| {
        let mut section = json!({"outside_diameter": qv(i.od, "m"), "wall_thickness": qv(i.wall, "m")});
        if let Some(rho) = density {
            section["material_density"] = qv(rho, "kg/m^3");
        }
        json!({"id": id, "from": from, "to": to, "section": section, "material": "material:w16",
            "y_reference": {"x": y[0], "y": y[1], "z": y[2]}, "provenance": PROV})
    }).collect();
    let supports: Vec<Value> = i.supports.iter().map(|(id, node, r)| json!({"id": id, "node": node, "family": "anchor", "restraints": r, "provenance": PROV})).collect();
    json!({"model": {"schema_version": "0.8.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:w16", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": nodes, "pipe_segments": pipes, "supports": supports, "components": [],
        "materials": [{"id": "material:w16", "elastic_modulus": qv(i.e, "Pa"), "shear_modulus": qv(i.g, "Pa"),
            "thermal_expansion_coefficient": qv(i.alpha, "1/degC"), "provenance": PROV}],
        "load_cases": cases, "combinations": combinations},
        "materials": []})
}

/// The PF request of one reference case.
fn pf_request(i: &Inputs, motion: bool, thermal: bool) -> Value {
    let mut loads = Vec::new();
    if motion {
        loads.extend(motion_loads(i));
    }
    if thermal {
        loads.extend(thermal_loads(i));
    }
    pf_document(i, vec![case(CASE, loads)], vec![], None)
}

/// The EX request of one reference case: `3.0.0/exact_pressure_v3` with
/// `shear_model: euler_bernoulli` and the load/reference-state group; the
/// motion is `boundary_motion` on `support:d`. `loads` are stored primitives
/// (listed as sources unless `unlisted`).
fn ex_request(i: &Inputs, motion: bool, thermal: bool, loads: Vec<Value>, unlisted: bool) -> Value {
    let mut wire = pf_document(i, vec![], vec![], None);
    let m = &mut wire["model"];
    m["pressure_contract"] = json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"});
    for pipe in m["pipe_segments"].as_array_mut().unwrap() {
        pipe["section"]["corrosion_allowance"] = qv(0.0, "m");
    }
    m["materials"] = json!([{"id": "material:w16", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
        "elastic_modulus": qv(i.e, "Pa"), "poisson_ratio": qv(i.nu, "1"), "provenance": PROV}]);
    let element_states: Vec<Value> = i.pipes.iter().map(|(id, ..)| {
        let thermal_state = if thermal {
            json!({"kind": "constant_alpha_interval", "coefficient": qv(i.alpha, "1/K"), "temperature_change": qv(i.delta_t, "K"),
                "coefficient_meaning": "engineering_interval", "provenance": PROV})
        } else {
            json!({"kind": "unchanged_reference", "provenance": PROV})
        };
        json!({"pipe_ref": id, "material_selection": {"kind": "explicit_base_properties", "material_ref": "material:w16",
            "applicability_reference": "invented analytical basis declared applicable to this member"}, "thermal_state": thermal_state})
    }).collect();
    let support_states: Vec<Value> = i.supports.iter().map(|(id, ..)| {
        let mut state = json!({"support_ref": id, "participation": {"kind": "active_model_device"}});
        if motion && id == "support:d" {
            state["boundary_motion"] = json!(i.motion.iter().map(|(dof, v, u)| json!({"dof": dof, "value": qv(*v, u), "meaning": "absolute_reference_displacement"})).collect::<Vec<_>>());
        }
        state
    }).collect();
    let members: Vec<Value> = i.pipes.iter().map(|(id, ..)| json!({"pipe_ref": id, "basis": {"kind": "direct_strain_reference"}, "fit": {"kind": "none"}, "provenance": PROV})).collect();
    m["reference_configurations"] = json!([{"id": REFERENCE, "label": "Installed reference", "geometry_ref": {"kind": "authored_model_geometry"},
        "member_references": members, "provenance": PROV}]);
    let sources: Vec<Value> = if unlisted { vec![] } else { loads.iter().map(|l| json!({"source_ref": l["id"], "factor": 1.0})).collect() };
    m["load_cases"] = json!([{"id": CASE, "pressure_regions": [], "primitive_loads": loads,
        "analysis_state": {"contract": STATE_CONTRACT, "reference_configuration_ref": REFERENCE,
            "element_states": element_states, "support_states": support_states, "load_sources": sources,
            "history": {"kind": "independent_equilibrium"}, "provenance": PROV},
        "provenance": PROV}]);
    wire
}

// ------------------------------------------------------------------ running
fn run(wire: &Value, mode: PreviewSolverMode) -> Value {
    let out = run_linear_static_preview_value_with_mode(wire.clone(), mode).unwrap_or_else(|e| panic!("typed boundary rejected the request: {e}"));
    serde_json::to_value(out).unwrap()
}
fn blocking(out: &Value) -> Vec<String> {
    out["diagnostics"].as_array().unwrap().iter()
        .filter(|d| d["severity"] == "blocking" || d["severity"] == "failure")
        .map(|d| format!("{}: {}", d["code"].as_str().unwrap(), d["message"].as_str().unwrap()))
        .collect()
}
fn codes(out: &Value) -> Vec<String> {
    out["diagnostics"].as_array().unwrap().iter().map(|d| d["code"].as_str().unwrap().to_string()).collect()
}
fn solved(wire: &Value, route: Route, mode: PreviewSolverMode, what: &str) -> Value {
    let out = run(wire, mode);
    assert_eq!(out["status"]["mechanics"], "MECHANICS_SOLVED", "{what}: not solved; blocking {:?}", blocking(&out));
    assert!(blocking(&out).is_empty(), "{what}: blocking {:?}", blocking(&out));
    assert_eq!(out["producer"]["semantic_contract_id"], if route == Route::Preview { PF_SEMANTICS } else { EX_SEMANTICS }, "{what}");
    assert!(!codes(&out).iter().any(|c| c.contains("IMPOSED_DISPLACEMENT")), "{what}: an imposed-displacement diagnostic on an admitted case");
    out
}
/// A refused request: the whole document is withheld (`results: []`) with the
/// named blocking code. Returns the matching diagnostics.
fn refused(wire: &Value, code: &str, what: &str) -> Vec<Value> {
    let mut found = Vec::new();
    for mode in MODES {
        let out = run(wire, mode);
        assert_eq!(out["status"]["mechanics"], "MODEL_INCOMPLETE", "{what}");
        assert!(rows(&out).is_empty(), "{what}: a refused document publishes no rows");
        found = out["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == code).cloned().collect();
        assert!(!found.is_empty(), "{what}: expected {code}, got {:?}", codes(&out));
        assert!(found.iter().all(|d| d["severity"] == "blocking"), "{what}");
    }
    found
}

fn route_name(route: Route) -> &'static str {
    if route == Route::Preview { "PF" } else { "EX" }
}

fn oracle_case(id: &str, route: Route) -> Vec<Value> {
    let i = inputs();
    let reference = case_ref(id);
    let (motion, thermal) = (id != THERMAL, id != MOTION);
    let wire = match route {
        Route::Preview => pf_request(&i, motion, thermal),
        Route::Exact => ex_request(&i, motion, thermal, vec![], false),
    };
    MODES.iter().map(|&mode| {
        let what = format!("{id} {} {}", route_name(route), mode.as_str());
        let out = solved(&wire, route, mode, &what);
        let (n, rel, zero) = compare(&out, &reference, ("load_case", CASE), &what);
        println!("W16 {what}: {n} rows; max relative error {rel:.3e}; max |zero row|/scale {zero:.3e}");
        out
    }).collect()
}

// ------------------------------------------------------------------ oracle and parity
#[test]
fn nozzle_movement_on_the_pressure_free_route() {
    oracle_case(MOTION, Route::Preview);
}

#[test]
fn thermal_alone_on_the_pressure_free_route() {
    oracle_case(THERMAL, Route::Preview);
}

#[test]
fn nozzle_movement_and_thermal_together_on_the_pressure_free_route() {
    oracle_case(BOTH, Route::Preview);
}

/// PF against EX `boundary_motion` for the same model and motion, in both
/// modes. EX is held to the same oracle first; parity then compares the two
/// routes' published rows directly.
#[test]
fn route_parity_with_exact_boundary_motion() {
    for id in [MOTION, THERMAL, BOTH] {
        let reference = case_ref(id);
        let pf = oracle_case(id, Route::Preview);
        let ex = oracle_case(id, Route::Exact);
        for (k, mode) in MODES.iter().enumerate() {
            let (n, rel, zero) = compare_with(&pf[k], &reference, ("load_case", CASE), &format!("{id} parity {}", mode.as_str()),
                |r| observed(&ex[k], ("load_case", CASE), r));
            println!("W16 {id} PF-EX parity {}: {n} rows; max relative difference {rel:.3e}; max |zero row|/scale {zero:.3e}", mode.as_str());
        }
    }
}

// ------------------------------------------------------------------ refusals (design §3)
fn with_supports(mut wire: Value, extra: Vec<Value>) -> Value {
    wire["model"]["supports"].as_array_mut().unwrap().extend(extra);
    wire
}
fn one_case(loads: Vec<Value>) -> Value {
    pf_document(&inputs(), vec![case(CASE, loads)], vec![], None)
}

#[test]
fn unknown_support_is_refused() {
    let d = refused(&one_case(vec![motion_load("m", "support:zz", "UX", 0.001, "m")]), "IMPOSED_DISPLACEMENT_SUPPORT_UNKNOWN", "unknown");
    assert_eq!(d[0]["affected_refs"], json!(["m", CASE, "support:zz"]));
}

#[test]
fn a_non_rigid_target_is_refused() {
    let spring = json!({"id": "support:s", "node": "node:c", "family": "spring", "restraints": ["UZ"],
        "stiffness": {"dof": "UZ", "value": qv(1.0e6, "N/m")}, "provenance": PROV});
    let hanger = json!({"id": "support:s", "node": "node:c", "family": "variable_spring_hanger", "restraints": ["UZ"],
        "hanger": {"hanger_type": "variable_spring_hanger", "stiffness": {"dof": "UZ", "value": qv(1.0e5, "N/m")}, "installed_load": qv(100.0, "N")}, "provenance": PROV});
    let constant = json!({"id": "support:s", "node": "node:c", "family": "constant_effort_support", "restraints": ["UZ"],
        "hanger": {"hanger_type": "constant_effort_support", "constant_load": qv(375.0, "N"), "travel_range": qv(0.05, "m"), "source_reference": "w16 invented"}, "provenance": PROV});
    // A nonlinear support's own DOF moves its ground (stage 2, R-b;
    // `w16_nonlinear_motion_tests`); its refusals are tested there.
    for (name, support) in [("spring", spring), ("variable spring hanger", hanger), ("constant effort", constant)] {
        let wire = with_supports(one_case(vec![motion_load("m", "support:s", "UZ", 0.001, "m")]), vec![support]);
        refused(&wire, "IMPOSED_DISPLACEMENT_SUPPORT_NOT_RIGID", name);
    }
}

#[test]
fn an_unrestrained_dof_is_refused() {
    let guide = json!({"id": "support:g", "node": "node:c", "family": "guide", "restraints": ["UZ"], "provenance": PROV});
    let wire = with_supports(one_case(vec![motion_load("m", "support:g", "UX", 0.001, "m")]), vec![guide]);
    refused(&wire, "IMPOSED_DISPLACEMENT_DOF_UNRESTRAINED", "guide UX");
}

#[test]
fn a_dof_mismatch_or_invalid_token_is_refused() {
    let mut mismatch = motion_load("m", "support:d", "UX", 0.001, "m");
    mismatch["direction"] = json!("UY");
    refused(&one_case(vec![mismatch]), "IMPOSED_DISPLACEMENT_DOF_MISMATCH", "UX against UY");
    let mut invalid = motion_load("m", "support:d", "UX", 0.001, "m");
    invalid["target"]["dof"] = json!("UQ");
    invalid["direction"] = json!("UQ");
    refused(&one_case(vec![invalid]), "IMPOSED_DISPLACEMENT_DOF_MISMATCH", "UQ");
    // Accepted spellings of one DOF agree.
    let mut spelled = motion_load("m", "support:d", "UX", 0.001, "m");
    spelled["direction"] = json!("ux");
    solved(&one_case(vec![spelled]), Route::Preview, PreviewSolverMode::SparseInteractive, "UX and ux");
}

#[test]
fn a_duplicate_global_dof_is_refused_even_with_equal_values() {
    let wire = one_case(vec![motion_load("m1", "support:d", "UX", 0.001, "m"), motion_load("m2", "support:d", "UX", 0.001, "m")]);
    let d = refused(&wire, "IMPOSED_DISPLACEMENT_DOF_DUPLICATE", "duplicate");
    assert_eq!(d.len(), 1);
    assert_eq!(d[0]["affected_refs"], json!(["m2", CASE, "support:d"]));
    // Two cases may each prescribe the DOF.
    let two = pf_document(&inputs(), vec![case("case:one", vec![motion_load("m1", "support:d", "UX", 0.001, "m")]),
        case("case:two", vec![motion_load("m2", "support:d", "UX", 0.001, "m")])], vec![], None);
    solved(&two, Route::Preview, PreviewSolverMode::SparseInteractive, "one per case");
}

#[test]
fn a_coincident_rigid_restraint_is_refused_before_the_repeated_dof_refusal() {
    let stop = json!({"id": "support:stop", "node": "node:d", "family": "line_stop", "restraints": ["UX"], "provenance": PROV});
    let wire = with_supports(one_case(vec![motion_load("m", "support:d", "UX", 0.001, "m")]), vec![stop.clone()]);
    for mode in MODES {
        let out = run(&wire, mode);
        assert!(codes(&out).contains(&"IMPOSED_DISPLACEMENT_COINCIDENT_RESTRAINT".to_string()), "{:?}", codes(&out));
        assert!(!codes(&out).contains(&"SUPPORT_INPUT_INVALID".to_string()), "the named check runs first: {:?}", codes(&out));
        assert!(rows(&out).is_empty());
    }
    // Without motion, the generic refusal of the repeated rigid DOF is unchanged.
    let plain = with_supports(one_case(vec![force_load("f", "node:c", "global_z", 100.0)]), vec![stop]);
    refused(&plain, "SUPPORT_INPUT_INVALID", "repeated rigid DOF without motion");
}

/// Stage 2, R-a (DEL-04-03 Design `pf-nonlinear-imposed-motion.md`): motion
/// of a rigid anchor in a model with nonlinear supports solves; the retired
/// `IMPOSED_DISPLACEMENT_WITH_NONLINEAR_SUPPORTS` is gone. An authored zero
/// motion is the no-motion path, row for row. The gap case keeps its recovery
/// basis branch (R1).
#[test]
fn rigid_motion_in_a_model_with_nonlinear_supports_solves() {
    let gap = json!({"id": "support:gap", "node": "node:c", "family": "nonlinear", "restraints": [],
        "nonlinear": {"behavior": "gap", "closes_when": "positive_displacement", "dof": "UZ", "gap": qv(0.0004, "m"), "initial_state": "inactive"}, "provenance": PROV});
    let zero = with_supports(one_case(vec![motion_load("m", "support:d", "UX", 0.0, "m"), force_load("f", "node:c", "global_z", 100.0)]), vec![gap.clone()]);
    let plain = with_supports(one_case(vec![force_load("f", "node:c", "global_z", 100.0)]), vec![gap.clone()]);
    let moved = with_supports(one_case(vec![motion_load("m", "support:d", "UX", 0.001, "m"), force_load("f", "node:c", "global_z", 100.0)]), vec![gap]);
    for mode in MODES {
        let (a, b) = (run(&plain, mode), run(&zero, mode));
        assert_eq!(a["status"]["mechanics"], "MECHANICS_SOLVED", "{:?}", blocking(&a));
        assert_eq!(a["results"], b["results"], "{mode:?}: an authored zero motion keeps every row");
        assert_eq!(a["diagnostics"], b["diagnostics"], "{mode:?}");
        let out = run(&moved, mode);
        assert_eq!(out["status"]["mechanics"], "MECHANICS_SOLVED", "{:?}", blocking(&out));
        assert!(!codes(&out).iter().any(|c| c == "IMPOSED_DISPLACEMENT_WITH_NONLINEAR_SUPPORTS"));
        assert!(codes(&out).contains(&"NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED".to_string()), "{:?}", codes(&out));
        let ux = rows(&out).iter().find(|r| r["id"] == "result:disp:node-d:ux").unwrap_or_else(|| panic!("{:?}", rows(&out).iter().map(|r| r["id"].clone()).collect::<Vec<_>>()));
        assert_eq!(ux["value"].as_f64().unwrap(), 0.001 * 1000.0, "{mode:?}: the moved anchor publishes g");
    }
}

#[test]
fn an_invalid_dimension_unit_or_magnitude_is_refused_by_name() {
    let mut rotation = motion_load("m", "support:d", "UX", 0.001, "m");
    rotation["dimension"] = json!("rotation");
    refused(&one_case(vec![rotation]), "IMPOSED_DISPLACEMENT_DIMENSION_INVALID", "UX as rotation");
    let mut translation = motion_load("m", "support:d", "RY", 0.001, "rad");
    translation["dimension"] = json!("displacement");
    translation["magnitude"]["unit"] = json!("m");
    refused(&one_case(vec![translation]), "IMPOSED_DISPLACEMENT_DIMENSION_INVALID", "RY as displacement");
    // An incompatible unit: the named check runs ahead of the generic unit checks.
    for (dof, unit) in [("UX", "N"), ("UX", "rad"), ("RY", "m")] {
        let wire = one_case(vec![motion_load("m", "support:d", dof, 0.001, unit)]);
        refused(&wire, "IMPOSED_DISPLACEMENT_DIMENSION_INVALID", &format!("{dof} in {unit}"));
        let out = run(&wire, PreviewSolverMode::SparseInteractive);
        assert!(!codes(&out).contains(&"UNIT_INPUT_INVALID".to_string()), "{:?}", codes(&out));
    }
    // A non-finite magnitude (not representable in JSON) through the typed entry.
    for value in [f64::INFINITY, f64::NAN] {
        let mut request: LinearStaticPreviewRequest = serde_json::from_value(one_case(vec![motion_load("m", "support:d", "UX", 0.001, "m")])).unwrap();
        request.model.load_cases[0].primitive_loads[0].magnitude.value = value;
        let out = serde_json::to_value(run_linear_static_preview(request)).unwrap();
        assert_eq!(out["status"]["mechanics"], "MODEL_INCOMPLETE");
        assert!(rows(&out).is_empty());
        assert!(codes(&out).contains(&"IMPOSED_DISPLACEMENT_DIMENSION_INVALID".to_string()), "{value}: {:?}", codes(&out));
    }
}

#[test]
fn a_support_target_needs_the_imposed_displacement_category_and_back() {
    let mut force = motion_load("m", "support:d", "UX", 100.0, "N");
    force["category"] = json!("concentrated_force");
    force["dimension"] = json!("force");
    let d = refused(&one_case(vec![force]), "IMPOSED_DISPLACEMENT_TARGET_INVALID", "a force on a support target");
    assert_eq!(d[0]["affected_refs"], json!(["m", CASE, "support:d"]));
    let mut on_node = motion_load("m", "support:d", "UX", 0.001, "m");
    on_node["target"] = json!({"type": "node", "node": "node:c"});
    refused(&one_case(vec![on_node]), "IMPOSED_DISPLACEMENT_TARGET_INVALID", "imposed displacement on a node");
    let mut on_pipe = motion_load("m", "support:d", "UX", 0.001, "m");
    on_pipe["target"] = json!({"type": "element", "pipe": "pipe:a-c"});
    refused(&one_case(vec![on_pipe]), "IMPOSED_DISPLACEMENT_TARGET_INVALID", "imposed displacement on an element");
    // The closed target refuses a support target without its DOF, or with another key.
    for target in [json!({"type": "support", "support": "support:d"}), json!({"type": "support", "support": "support:d", "dof": "UX", "node": "node:d"})] {
        let mut load = motion_load("m", "support:d", "UX", 0.001, "m");
        load["target"] = target;
        assert!(run_linear_static_preview_value_with_mode(one_case(vec![load]), PreviewSolverMode::SparseInteractive).is_err());
    }
}

#[test]
fn one_invalid_case_withholds_the_whole_document() {
    let i = inputs();
    let wire = pf_document(&i, vec![case("case:valid", motion_loads(&i)), case("case:invalid", vec![motion_load("m", "support:zz", "UX", 0.001, "m")])], vec![], None);
    let d = refused(&wire, "IMPOSED_DISPLACEMENT_SUPPORT_UNKNOWN", "mixed document");
    assert_eq!(d.len(), 1);
    assert_eq!(d[0]["affected_refs"][1], "case:invalid");
    let out = run(&wire, PreviewSolverMode::SparseInteractive);
    assert_eq!(out["contract_evidence"], json!({"preview_cases": [], "combination_gates": []}));
    assert!(out["summary"]["max_displacement"].is_null());
}

/// EX refuses every support-targeted primitive by name, without the
/// load/reference-state group and with it, listed or not: the scan reads the
/// authored primitives before any effective-case selection.
#[test]
fn the_exact_route_refuses_support_targeted_primitives() {
    let i = inputs();
    let load = || vec![motion_load("m", "support:d", "UX", 0.001, "m")];
    // Without the group: the pressure-2 document shape.
    let mut plain = ex_request(&i, false, false, load(), false);
    let m = plain["model"].as_object_mut().unwrap();
    m.remove("reference_configurations");
    m["load_cases"][0].as_object_mut().unwrap().remove("analysis_state");
    for (name, wire) in [("no group", plain), ("group, listed", ex_request(&i, false, false, load(), false)),
        ("group, unlisted", ex_request(&i, false, false, load(), true))] {
        let d = refused(&wire, "EXACT_IMPOSED_DISPLACEMENT_PRIMITIVE_UNSUPPORTED", name);
        assert_eq!(d.len(), 1, "{name}");
        assert!(d[0]["message"].as_str().unwrap().contains("boundary_motion"), "{name}");
        assert_eq!(d[0]["affected_refs"], json!(["m", CASE, "support:d"]), "{name}");
        let out = run(&wire, PreviewSolverMode::SparseInteractive);
        assert!(!codes(&out).iter().any(|c| c.starts_with("IMPOSED_DISPLACEMENT_")), "{name}: PF codes on EX {:?}", codes(&out));
    }
}

/// The retained sub-route declines a support target at D1.7 (its census reads
/// the variant); the published result is the ordinary PF result, byte for byte.
#[test]
fn retained_admission_declines_and_the_ordinary_result_is_returned() {
    let wire = pf_request(&inputs(), true, false);
    for mode in MODES {
        let ordinary = run_linear_static_preview_value_with_mode(wire.clone(), mode).unwrap();
        let retained = run_linear_static_preview_value_with_retained_direct(wire.clone(), mode, RETAINED_LIMIT).unwrap();
        assert!(retained.admission().is_some_and(|report| report.census_complete()));
        assert!(retained.successor().is_none());
        assert_eq!(serde_json::to_vec(retained.envelope()).unwrap(), serde_json::to_vec(&ordinary).unwrap());
        assert_eq!(ordinary.status.mechanics, "MECHANICS_SOLVED");
    }
}

// ------------------------------------------------------------------ allowed cases
/// A spring and a rigid component share the moved node; neither is a
/// coincident rigid restraint. The structure's response is the oracle's.
#[test]
fn a_spring_or_a_rigid_component_at_the_moved_node_is_allowed() {
    let i = inputs();
    let reference = case_ref(MOTION);
    let spring = json!({"id": "support:spring", "node": "node:d", "family": "spring", "restraints": ["UX"],
        "stiffness": {"dof": "UX", "value": qv(2.0e6, "N/m")}, "provenance": PROV});
    let valve = json!({"id": "component:valve", "label": "w16 invented rigid valve marker", "kind": "valve", "node": "node:d",
        "geometry": {"rigid_pipe_ref": "pipe:c-d", "rigid_body_length": qv(0.5, "m"), "end_a_size": qv(0.25, "m"), "end_b_size": qv(0.25, "m"),
            "weight": qv(0.0, "N"), "center_of_gravity": {"x": 0.0, "y": 0.0, "z": 0.0, "unit": "m"},
            "connection_end_a_reference": "w16 invented end a", "connection_end_b_reference": "w16 invented end b",
            "stiffness_behavior_reference": "w16 invented rigid behaviour", "rigid_component_source_reference": "w16 invented rigid component geometry"},
        "modifiers": {"stiffness_scaling_user_value": qv(10.0, "none"), "source_reference": "w16 invented modifier"},
        "provenance": PROV});
    let mut with_spring = with_supports(pf_request(&i, true, false), vec![spring]);
    let mut with_valve = pf_request(&i, true, false);
    with_valve["model"]["components"] = json!([valve]);
    for (name, wire) in [("spring", &mut with_spring), ("rigid component", &mut with_valve)] {
        for mode in MODES {
            let out = solved(wire, Route::Preview, mode, name);
            for r in reference["rows"].as_array().unwrap().iter().filter(|r| r["quantity"] != "support_reaction" || r["entity"] == "support:a") {
                close(observed(&out, ("load_case", CASE), r), exact_rational(r), zero_scale(&reference, r["unit"].as_str().unwrap()), &format!("{name} {}", label(r)));
            }
        }
    }
}

/// The nozzle movement split over two supports at the moved node, on different
/// DOFs: all six DOFs held, four moved.
#[test]
fn motion_on_a_different_dof_of_the_same_node_is_allowed() {
    let i = inputs();
    let reference = case_ref(MOTION);
    let mut wire = pf_request(&i, false, false);
    let supports = wire["model"]["supports"].as_array_mut().unwrap();
    let d = supports.iter_mut().find(|s| s["id"] == "support:d").unwrap();
    d["restraints"] = json!(["UX", "UY", "RX", "RY", "RZ"]);
    supports.push(json!({"id": "support:d-z", "node": "node:d", "family": "vertical_support", "restraints": ["UZ"], "provenance": PROV}));
    let loads: Vec<Value> = i.motion.iter().map(|(dof, v, u)| motion_load(&format!("motion:{dof}"), if dof == "UZ" { "support:d-z" } else { "support:d" }, dof, *v, u)).collect();
    wire["model"]["load_cases"][0]["primitive_loads"] = json!(loads);
    for mode in MODES {
        let out = solved(&wire, Route::Preview, mode, "split supports");
        for r in reference["rows"].as_array().unwrap() {
            let zs = zero_scale(&reference, r["unit"].as_str().unwrap());
            let mut row = r.clone();
            if r["entity"] == "support:d" && r["component"] == "Fz" {
                row["entity"] = json!("support:d-z");
            }
            close(observed(&out, ("load_case", CASE), &row), exact_rational(r), zs, &format!("split {}", label(&row)));
        }
    }
}

fn subtract_documents(with: &Value, without: &Value, reference: &Value, what: &str) {
    compare_with(with, reference, ("load_case", CASE), what, |r| exact_rational(r) + observed(without, ("load_case", CASE), r));
}

/// An unrelated constant-effort support (a fixed nodal force; no nonlinear
/// solve), and motion with ordinary forces and generated weight: motion
/// superposes on each.
#[test]
fn motion_superposes_on_constant_effort_forces_and_generated_weight() {
    let i = inputs();
    let reference = case_ref(MOTION);
    let constant = json!({"id": "support:ce", "node": "node:c", "family": "constant_effort_support", "restraints": ["UZ"],
        "hanger": {"hanger_type": "constant_effort_support", "constant_load": qv(375.0, "N"), "travel_range": qv(0.05, "m"), "source_reference": "w16 invented"}, "provenance": PROV});
    let forces = || vec![force_load("f:c:x", "node:c", "global_x", 1500.0), force_load("f:c:z", "node:c", "global_z", -800.0)];
    for mode in MODES {
        // Constant effort.
        let with = with_supports(pf_request(&i, true, false), vec![constant.clone()]);
        let without = with_supports(one_case(forces()), vec![constant.clone()]);
        let mut with_forces = with.clone();
        with_forces["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().extend(forces());
        let a = solved(&with_forces, Route::Preview, mode, "constant effort with motion");
        let b = solved(&without, Route::Preview, mode, "constant effort without motion");
        subtract_documents(&a, &b, &reference, "constant effort");
        // Generated weight, with the ordinary forces.
        let (with, _) = with_self_weight(pf_document(&i, vec![case(CASE, motion_loads(&i).into_iter().chain(forces()).collect())], vec![], Some(7850.0)));
        let (without, _) = with_self_weight(pf_document(&i, vec![case(CASE, forces())], vec![], Some(7850.0)));
        let a = solved(&with, Route::Preview, mode, "weight with motion");
        let b = solved(&without, Route::Preview, mode, "weight without motion");
        subtract_documents(&a, &b, &reference, "generated weight");
    }
}

/// The product's own self-weight generation on the document's model, applied
/// as drafted (gravity -9.80665 m/s^2 along global Z), kept with the case's
/// existing primitives.
fn with_self_weight(mut wire: Value) -> (Value, usize) {
    let mut source = wire["model"].clone();
    let existing = source["load_cases"][0].clone();
    source["load_cases"] = json!([]);
    let digest: String = Sha256::digest(serde_json::to_string(&source).unwrap().as_bytes()).iter().map(|b| format!("{b:02x}")).collect();
    let typed: PreviewModel = serde_json::from_value(source).expect("model parses");
    let request = SelfWeightRequest {
        case_id: CASE.into(),
        label: "W16 metal self weight".into(),
        pipe_refs: inputs().pipes.iter().map(|p| p.0.clone()).collect(),
        gravity: SelfWeightGravity { value: -9.80665, unit: "m/s^2".into(), axis: "global_z".into() },
        provenance: PROV.into(),
        source_model_hash: format!("sha256:{digest}"),
    };
    let plan = generate_self_weight_operations(&typed, &request).unwrap_or_else(|d| panic!("self-weight generation refused: {d:?}"));
    let generated: Vec<Value> = plan.changes[1..].iter().map(|c| serde_json::from_str(&c.after).unwrap()).collect();
    let drafted: Value = serde_json::from_str(&plan.changes[0].after).unwrap();
    let mut case = existing;
    for key in ["label", "kind", "status"] {
        case[key] = drafted[key].clone();
    }
    let n = generated.len();
    case["primitive_loads"].as_array_mut().unwrap().extend(generated);
    wire["model"]["load_cases"] = json!([case]);
    (wire, n)
}

/// Non-SI inputs: translations in mm and in, the rotation in deg.
#[test]
fn non_si_translation_and_rotation_inputs_are_normalized() {
    let i = inputs();
    let reference = case_ref(MOTION);
    let loads: Vec<Value> = i.motion.iter().map(|(dof, v, _)| match dof.as_str() {
        "UX" => motion_load("motion:UX", "support:d", dof, v * 1000.0, "mm"),
        "UY" => motion_load("motion:UY", "support:d", dof, v * 1000.0, "mm"),
        "UZ" => motion_load("motion:UZ", "support:d", dof, v / 0.0254, "in"),
        _ => motion_load(&format!("motion:{dof}"), "support:d", dof, v.to_degrees(), "deg"),
    }).collect();
    let wire = one_case(loads);
    for mode in MODES {
        let out = solved(&wire, Route::Preview, mode, "non-SI");
        compare(&out, &reference, ("load_case", CASE), &format!("non-SI {}", mode.as_str()));
    }
}

// ------------------------------------------------------------------ combinations
/// Linear combination, subtraction (a labelled state difference) and range
/// envelope (a row envelope, not one equilibrium state) with motion cases.
#[test]
fn combinations_take_motion_cases() {
    let i = inputs();
    let (motion, thermal, both) = (case_ref(MOTION), case_ref(THERMAL), case_ref(BOTH));
    // Primitive-load ids are unique within the model.
    let ids = |prefix: &str, loads: Vec<Value>| -> Vec<Value> {
        loads.into_iter().map(|mut l| { l["id"] = json!(format!("{prefix}:{}", l["id"].as_str().unwrap())); l }).collect()
    };
    let mut both_loads = ids("both", motion_loads(&i));
    both_loads.extend(ids("both", thermal_loads(&i)));
    let wire = pf_document(&i, vec![case("case:motion", ids("motion", motion_loads(&i))), case("case:thermal", ids("thermal", thermal_loads(&i))), case("case:both", both_loads)],
        vec![json!({"id": "comb:lin", "basis": "mechanics", "terms": [{"load_case": "case:motion", "factor": 1.0}, {"load_case": "case:thermal", "factor": 1.0}], "provenance": PROV}),
             json!({"id": "comb:scaled", "basis": "mechanics", "terms": [{"load_case": "case:motion", "factor": 2.0}, {"load_case": "case:thermal", "factor": -0.5}], "provenance": PROV}),
             json!({"id": "comb:sub", "basis": "result_state_subtraction", "minuend_id": "case:both", "subtrahend_id": "case:thermal", "provenance": PROV}),
             json!({"id": "comb:env", "basis": "range_envelope", "operand_ids": ["case:motion", "case:thermal"], "mode": "max", "provenance": PROV})],
        None);
    for mode in MODES {
        let out = solved(&wire, Route::Preview, mode, "combinations");
        for gate in out["contract_evidence"]["combination_gates"].as_array().unwrap() {
            assert_eq!(gate["withheld"], false, "{gate}");
        }
        assert!(!codes(&out).contains(&"LOAD_COMBINATION_ROW_WITHHELD".to_string()), "{:?}", codes(&out));
        for (case_id, reference) in [("case:motion", &motion), ("case:thermal", &thermal), ("case:both", &both)] {
            compare(&out, reference, ("load_case", case_id), case_id);
        }
        compare(&out, &both, ("combination", "comb:lin"), "motion + thermal");
        compare_with(&out, &both, ("combination", "comb:scaled"), "2 motion - 0.5 thermal", |r| {
            let pick = |c: &Value| exact_rational(c["rows"].as_array().unwrap().iter().find(|x| label(x) == label(r)).unwrap());
            2.0 * pick(&motion) - 0.5 * pick(&thermal)
        });
        compare(&out, &motion, ("combination", "comb:sub"), "both - thermal");
        let sub = rows(&out).iter().find(|r| r["basis_ref"]["ref_id"] == "comb:sub" && r["kind"] == "support_reaction_component_v2").unwrap();
        assert!(sub["metadata"]["sign_convention"].as_str().unwrap().contains("not an equilibrium state"));
        compare_with(&out, &both, ("combination", "comb:env"), "envelope max", |r| {
            let pick = |c: &Value| exact_rational(c["rows"].as_array().unwrap().iter().find(|x| label(x) == label(r)).unwrap());
            // The envelope selects among published rows; an end_i row publishes -section(0).
            if r["station"] == "0" { pick(&motion).min(pick(&thermal)) } else { pick(&motion).max(pick(&thermal)) }
        });
    }
}
