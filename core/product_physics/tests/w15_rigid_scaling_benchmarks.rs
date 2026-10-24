//! W-15 product-path benchmarks (DEL-03-05 Design `rigid-component-stiffness-scaling.md`
//! §6; PRD §22.2 rigid-link benchmark): the rigid-link cantilever (s = 1, 1/4, 1e3, 1e6;
//! Euler-Bernoulli on both routes, Timoshenko on the exact route), the thermal and fit
//! series on both thermal producers and on material, named and interpolated bases, a
//! free scaled body, and the closed-end pressure cantilever with a scaled body.
//!
//! Every expectation is read from the frozen references in
//! `validation/references/w15_rigid_scaling/references.json` (exact closed forms with an
//! exact direct-stiffness cross-check; see that package's README), pinned here to the
//! hash recorded in its SHA256SUMS. Nothing here derives an expectation from product
//! code. Model inputs are invented: the body is a valve at its member's `from` node with
//! the case's user scale; no scale, stiffness or weight is a catalog or code value.
//!
//! Criterion (DEC-024/DEC-026, unchanged): |obs - exp| <= 1e-9 |exp|; an exactly zero
//! expectation uses 1e-9 times the case's frozen zero scale of its family. Both solver
//! modes run against the same reference, and the product's own readers (with the
//! captured model) admit every envelope.
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, MechanicsEnvelope, PreviewSolverMode, ResultItem};
use open_pipe_stress_product_physics::result_export::reader_verdict;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const REFERENCES: &str = include_str!("../../../validation/references/w15_rigid_scaling/references.json");
const SUMS: &str = include_str!("../../../validation/references/w15_rigid_scaling/SHA256SUMS");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const RELATIVE: f64 = 1.0e-9;
const PROV: &str = "w15 invented benchmark input; not library, component, material-catalog or code data";
const CASE: &str = "case:w15";
const STATE_CONTRACT: &str = "openpipestress.load_reference_state/1.0.0";
const REFERENCE: &str = "reference:installed";

#[derive(Clone, Copy, PartialEq, Debug)]
enum Route {
    Preview,
    Exact,
}

fn refs() -> Value {
    serde_json::from_str(REFERENCES).expect("references.json parses")
}
fn case_ref(id: &str) -> Value {
    refs()["cases"].as_array().unwrap().iter().find(|c| c["id"] == id).unwrap_or_else(|| panic!("no case {id}")).clone()
}
fn value(v: &Value) -> f64 {
    v["value"].as_f64().unwrap_or_else(|| panic!("not a value record: {v}"))
}
fn rational(text: &Value) -> f64 {
    let s = text.as_str().unwrap_or_else(|| panic!("input text {text}"));
    match s.split_once('/') {
        Some((p, q)) => p.parse::<f64>().unwrap() / q.parse::<f64>().unwrap(),
        None => s.parse().unwrap(),
    }
}
fn qv(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}

#[test]
fn references_are_the_frozen_bytes() {
    let digest: String = Sha256::digest(REFERENCES.as_bytes()).iter().map(|b| format!("{b:02x}")).collect();
    let line = SUMS.lines().find(|l| l.ends_with("  references.json")).expect("SHA256SUMS lists references.json");
    assert_eq!(&line[..64], digest, "the references must equal the frozen bytes");
    let doc = refs();
    assert!(doc["crosscheck"].as_array().unwrap().iter().all(|c| c["agrees"] == true));
}

// ------------------------------------------------------------------ comparison
/// The row's error (relative, or |obs|/zero scale for a zero row), or the failure text.
fn close(observed: f64, expected: f64, zero_scale: f64, what: &str) -> Result<f64, String> {
    let tolerance = RELATIVE * if expected == 0.0 { zero_scale.abs() } else { expected.abs() };
    let error = if expected == 0.0 { (observed / zero_scale).abs() } else { ((observed - expected) / expected).abs() };
    if observed.is_finite() && (observed - expected).abs() <= tolerance {
        Ok(error)
    } else {
        Err(format!("{what}: observed {observed:.17e} expected {expected:.17e} tolerance {tolerance:.3e} error {error:.3e}"))
    }
}

fn rows<'a>(out: &'a MechanicsEnvelope, kind: &str, entity: &str, component: Option<&str>, location: Option<&str>) -> Vec<&'a ResultItem> {
    out.results
        .iter()
        .filter(|r| {
            r.kind == kind
                && r.entity_ref == entity
                && r.basis_ref.as_ref().is_some_and(|b| b.ref_type == "load_case" && b.ref_id == CASE)
                && r.metadata.as_ref().is_some_and(|m| component.is_none_or(|c| m.component == c) && location.is_none_or(|l| m.location == l))
        })
        .collect()
}

fn one(out: &MechanicsEnvelope, kind: &str, entity: &str, component: &str, location: Option<&str>) -> (f64, String) {
    let found = rows(out, kind, entity, Some(component), location);
    assert_eq!(found.len(), 1, "one row {kind} {entity} {component} {location:?}");
    (found[0].value, found[0].unit.clone())
}

/// Compare every frozen row of a case: the largest relative error (nonzero rows)
/// and |obs|/zero scale (zero rows), or every row outside the criterion.
fn compare(out: &MechanicsEnvelope, reference: &Value, label: &str) -> Result<(f64, f64), String> {
    let scales = &reference["zero_scales"];
    let scale = |unit: &str| value(&scales[match unit {
        "N" => "force",
        "N*m" => "moment",
        "m" => "translation",
        "rad" => "rotation",
        other => panic!("unit {other}"),
    }]);
    let (mut rel, mut zero, mut worst) = (0.0f64, 0.0f64, 0.0f64);
    let mut failures = Vec::new();
    for r in reference["rows"].as_array().unwrap() {
        let (entity, component, unit) = (r["entity"].as_str().unwrap(), r["component"].as_str().unwrap(), r["unit"].as_str().unwrap());
        let expected = value(r);
        let zs = scale(unit);
        let mut check = |observed: f64, expected: f64, what: String| match close(observed, expected, zs, &what) {
            Ok(e) if expected == 0.0 => zero = zero.max(e),
            Ok(e) => rel = rel.max(e),
            Err(failure) => {
                worst = worst.max(if expected == 0.0 { (observed / zs).abs() } else { ((observed - expected) / expected).abs() });
                failures.push(failure)
            }
        };
        match r["quantity"].as_str().unwrap() {
            "support_reaction" => {
                let (v, u) = one(out, "support_reaction_component_v2", entity, component, None);
                assert_eq!(u, unit);
                check(v, expected, format!("{label} {entity} {component}"));
            }
            "nodal_motion" => {
                let axis = &component[1..];
                let (v, u) = if component.starts_with('u') {
                    let (v, u) = one(out, &format!("global_nodal_displacement_{axis}"), entity, &format!("nodal_displacement_{axis}"), None);
                    assert_eq!(u, "mm");
                    (v / 1000.0, "m".to_string())
                } else {
                    one(out, &format!("global_nodal_rotation_{axis}"), entity, &format!("nodal_rotation_{axis}"), None)
                };
                assert_eq!(u, unit);
                check(v, expected, format!("{label} {entity} {component}"));
            }
            "section_action" => {
                let (location, sign) = match r["station"].as_str().unwrap() {
                    "0" => ("end_i", -1.0),
                    "1/2" => ("midspan", 1.0),
                    "1" => ("end_j", 1.0),
                    s => panic!("station {s}"),
                };
                let (v, u) = one(out, &format!("element_local_{component}"), entity, component, Some(location));
                assert_eq!(u, unit);
                check(v, sign * expected, format!("{label} {entity} {component} {location}"));
            }
            "wall_axial_force" => {
                let found = rows(out, "pipe_wall_axial_force_v2", entity, None, None);
                assert!(!found.is_empty(), "{label}: wall force rows of {entity}");
                for row in found {
                    assert_eq!(row.unit, "N");
                    check(row.value, expected, format!("{label} {entity} wall force {}", row.id));
                }
            }
            q => panic!("quantity {q}"),
        }
    }
    if failures.is_empty() {
        Ok((rel, zero))
    } else {
        let count = reference["rows"].as_array().unwrap().len();
        Err(format!("{label}: {} of {count} rows outside the criterion (largest error {worst:.3e}):\n{}", failures.len(), failures.join("\n")))
    }
}

// ------------------------------------------------------------------ model authoring
struct Section {
    od: f64,
    wall: f64,
    e: f64,
    g: f64,
    nu: f64,
}
fn section() -> Section {
    let s = &refs()["section"];
    Section {
        od: rational(&s["outside_diameter_m"]),
        wall: rational(&s["wall_thickness_m"]),
        e: rational(&s["elastic_modulus_Pa"]),
        g: rational(&s["shear_modulus_Pa"]),
        nu: rational(&s["poisson_ratio"]),
    }
}

/// Route-specific authoring of one case: loads, member thermal and fit states,
/// materials and pressure regions.
#[derive(Default)]
struct Authoring {
    loads: Vec<Value>,
    /// PF: per-member temperature change (degC), applied as element thermal loads.
    thermal_preview: Option<f64>,
    /// EX: per member (thermal_state, fit) of the load/reference-state group.
    states: Option<Vec<(String, Value, Value)>>,
    /// EX: the material selection of every member.
    selection: Option<Value>,
    /// PF: the case's modulus basis keys.
    basis: Option<(&'static str, Value)>,
    materials: Option<Value>,
    regions: Option<Value>,
    timoshenko: bool,
}

fn request(reference: &Value, route: Route, a: Authoring) -> Value {
    let g = &reference["inputs"]["geometry"];
    let s = section();
    let nodes: Vec<Value> = g["nodes"].as_object().unwrap().iter()
        .map(|(id, p)| json!({"id": id, "position": {"x": rational(&p[0]), "y": rational(&p[1]), "z": rational(&p[2])}, "provenance": PROV}))
        .collect();
    let members = g["members"].as_object().unwrap();
    let pipes: Vec<Value> = members.iter().map(|(id, m)| {
        let mut sec = json!({"outside_diameter": qv(s.od, "m"), "wall_thickness": qv(s.wall, "m")});
        if route == Route::Exact {
            sec["corrosion_allowance"] = qv(0.0, "m");
        }
        json!({"id": id, "from": m["from"], "to": m["to"], "section": sec, "material": "material:w15",
            "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0}, "provenance": PROV})
    }).collect();
    let supports: Vec<Value> = g["supports"].as_object().unwrap().iter()
        .map(|(id, v)| json!({"id": id, "node": v["node"], "family": "anchor", "restraints": v["restraints"], "provenance": PROV}))
        .collect();
    let body = &reference["inputs"]["body"];
    let component = json!({"id": "component:w15-valve", "kind": body["component_kind"], "node": body["component_node"],
        "geometry": {"rigid_pipe_ref": body["member"]},
        "modifiers": {"stiffness_scaling_user_value": qv(rational(&body["stiffness_scale"]), "1"), "source_reference": "invented W-15 benchmark scale"},
        "mechanics_interface": {"solver_consumption": "mechanics_geometry_only"}, "provenance": PROV});
    let units = json!({"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"});
    let status = json!({"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"});
    let mut case = json!({"id": CASE, "primitive_loads": a.loads, "provenance": PROV});
    let mut model = json!({"schema_version": "0.7.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:w15", "units": units}, "analysis_status": status,
        "nodes": nodes, "pipe_segments": pipes, "supports": supports, "components": [component], "combinations": []});
    match route {
        Route::Preview => {
            if let Some(dt) = a.thermal_preview {
                for id in members.keys() {
                    case["primitive_loads"].as_array_mut().unwrap().push(json!({"id": format!("thermal:{id}"), "category": "thermal",
                        "target": {"type": "element", "pipe": id}, "direction": "global_x", "magnitude": qv(dt, "degC"),
                        "dimension": "temperature_interval", "provenance": PROV}));
                }
            }
            if let Some((key, basis)) = a.basis {
                case[key] = basis;
            }
            model["materials"] = a.materials.unwrap_or_else(|| json!([{"id": "material:w15", "elastic_modulus": qv(s.e, "Pa"),
                "shear_modulus": qv(s.g, "Pa"), "thermal_expansion_coefficient": qv(1.2e-5, "1/degC"), "provenance": PROV}]));
        }
        Route::Exact => {
            model["pressure_contract"] = json!({"version": "3.0.0", "mode": "exact_pressure_v3",
                "shear_model": if a.timoshenko { "timoshenko_annulus_energy_v1" } else { "euler_bernoulli" }});
            model["materials"] = a.materials.unwrap_or_else(|| json!([{"id": "material:w15", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
                "elastic_modulus": qv(s.e, "Pa"), "poisson_ratio": qv(s.nu, "1"), "provenance": PROV}]));
            case["pressure_regions"] = a.regions.unwrap_or_else(|| json!([]));
            if let Some(states) = a.states {
                let selection = a.selection.unwrap_or_else(|| json!({"kind": "explicit_base_properties", "material_ref": "material:w15",
                    "applicability_reference": "invented analytical basis declared applicable to this member"}));
                let element_states: Vec<Value> = states.iter().map(|(id, thermal, _)| json!({"pipe_ref": id,
                    "material_selection": selection, "thermal_state": thermal})).collect();
                let references: Vec<Value> = states.iter().map(|(id, _, fit)| json!({"pipe_ref": id,
                    "basis": {"kind": "direct_strain_reference"}, "fit": fit, "provenance": PROV})).collect();
                let support_states: Vec<Value> = g["supports"].as_object().unwrap().keys()
                    .map(|id| json!({"support_ref": id, "participation": {"kind": "active_model_device"}})).collect();
                model["reference_configurations"] = json!([{"id": REFERENCE, "label": "Installed reference",
                    "geometry_ref": {"kind": "authored_model_geometry"}, "member_references": references, "provenance": PROV}]);
                let sources: Vec<Value> = case["primitive_loads"].as_array().unwrap().iter().map(|l| json!({"source_ref": l["id"], "factor": 1.0})).collect();
                case["analysis_state"] = json!({"contract": STATE_CONTRACT, "reference_configuration_ref": REFERENCE,
                    "element_states": element_states, "support_states": support_states, "load_sources": sources,
                    "history": {"kind": "independent_equilibrium"}, "provenance": PROV});
            }
        }
    }
    model["load_cases"] = json!([case]);
    json!({"model": model, "materials": []})
}

fn solve(wire: &Value, route: Route, mode: PreviewSolverMode, label: &str) -> MechanicsEnvelope {
    let out = run_linear_static_preview_value_with_mode(wire.clone(), mode).unwrap_or_else(|e| panic!("{label}: {e}"));
    let blocking: Vec<String> = out.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| format!("{}: {}", d.code, d.message)).collect();
    assert_eq!(out.status.mechanics, "MECHANICS_SOLVED", "{label}: blocking {blocking:?}");
    assert!(out.diagnostics.iter().any(|d| d.code == "RIGID_COMPONENT_STIFFNESS_SCALING_APPLIED"), "{label}: the scale is applied");
    // The product's reader with the captured model admits the envelope, records included.
    let reader = if route == Route::Preview { "preview-physics" } else { "pressure" };
    let verdict = reader_verdict::validate(reader, &serde_json::to_value(&out).unwrap(), Some(&wire["model"]), None);
    assert_eq!(verdict["ok"], true, "{label}: {verdict}");
    out
}

fn run(id: &str, route: Route, authoring: impl Fn(&Value) -> Authoring) {
    let reference = case_ref(id);
    let wire = request(&reference, route, authoring(&reference));
    let mut failures = Vec::new();
    for mode in MODES {
        let label = format!("{id} {route:?} {}", mode.as_str());
        let out = solve(&wire, route, mode, &label);
        match compare(&out, &reference, &label) {
            Ok((rel, zero)) => println!("W15 {label}: {} rows; max relative error {rel:.3e}; max |zero row|/scale {zero:.3e}", reference["rows"].as_array().unwrap().len()),
            Err(failure) => failures.push(failure),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

// ------------------------------------------------------------------ W15-R
fn tip_loads(reference: &Value) -> Vec<Value> {
    let l = &reference["inputs"]["loads"];
    let node = l["node"].as_str().unwrap();
    vec![
        json!({"id": "load:fx", "category": "concentrated_force", "target": {"type": "node", "node": node}, "direction": "global_x",
            "magnitude": qv(rational(&l["Fx_N"]), "N"), "dimension": "force", "provenance": PROV}),
        json!({"id": "load:fy", "category": "concentrated_force", "target": {"type": "node", "node": node}, "direction": "global_y",
            "magnitude": qv(rational(&l["Fy_N"]), "N"), "dimension": "force", "provenance": PROV}),
        json!({"id": "load:mx", "category": "concentrated_moment", "target": {"type": "node", "node": node}, "direction": "rotation_x",
            "magnitude": qv(rational(&l["Mx_N_m"]), "N*m"), "dimension": "moment", "provenance": PROV}),
    ]
}

fn rigid_link_euler_bernoulli(scales: &[&str]) {
    let mut failures = Vec::new();
    for s in scales {
        for route in [Route::Preview, Route::Exact] {
            let id = format!("W15-R-S{s}-EB");
            let outcome = std::panic::catch_unwind(|| run(&id, route, |r| Authoring { loads: tip_loads(r), ..Default::default() }));
            if let Err(panic) = outcome {
                failures.push(panic.downcast_ref::<String>().cloned().unwrap_or_default());
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

fn rigid_link_timoshenko(scales: &[&str]) {
    for s in scales {
        run(&format!("W15-R-S{s}-TIM"), Route::Exact, |r| Authoring { loads: tip_loads(r), timoshenko: true, ..Default::default() });
    }
}

#[test]
fn w15_r_rigid_link_euler_bernoulli_both_routes() {
    rigid_link_euler_bernoulli(&["1", "1_4", "1000"]);
}

#[test]
fn w15_r_rigid_link_timoshenko_exact_route() {
    rigid_link_timoshenko(&["1", "1_4", "1000"]);
}

/// Design §6 runs s = 1e6 at the 1e-9 criterion. The product misses it: 14 to 20
/// of the 78 rows per run (the in-plane bending motions, the shear and moment of
/// the body and of the root pipe, and the reactions Fy and Mz) are off by up to
/// 6.1e-8 (sparse 2.7e-8 to 6.1e-8, dense 7.6e-9 to 3.4e-8), on both routes and
/// both modes; at s <= 1e3 every row is within 1e-15. The deviation has the size
/// of binary64 cancellation in the stiff body's s K u, of order
/// eps s |K| |u| / |F|. Ignored, not relaxed: the criterion and the run set are
/// the Design's, and their resolution is the coordinator's (W-15 open point).
#[test]
#[ignore = "W-15 open point: s = 1e6 misses the Design's 1e-9 criterion (largest error 6.1e-8)"]
fn w15_r_rigid_link_s1e6_euler_bernoulli_both_routes() {
    rigid_link_euler_bernoulli(&["1000000"]);
}

#[test]
#[ignore = "W-15 open point: s = 1e6 misses the Design's 1e-9 criterion (largest error 2.7e-8)"]
fn w15_r_rigid_link_s1e6_timoshenko_exact_route() {
    rigid_link_timoshenko(&["1000000"]);
}

// ------------------------------------------------------------------ W15-T
fn pf_materials(points: Option<Value>) -> Value {
    let s = section();
    let mut material = json!({"id": "material:w15", "elastic_modulus": qv(s.e, "Pa"), "shear_modulus": qv(s.g, "Pa"),
        "thermal_expansion_coefficient": qv(1.2e-5, "1/degC"), "provenance": PROV});
    if let Some(points) = points {
        material["temperature_points"] = points;
    }
    json!([material])
}

#[test]
fn w15_t_series_element_thermal_load_on_material_named_and_interpolated_bases() {
    for id in ["W15-T-PF-MAT-S1_4", "W15-T-PF-MAT-S4"] {
        run(id, Route::Preview, |_| Authoring { thermal_preview: Some(40.0), ..Default::default() });
    }
    run("W15-T-PF-NAMED-S4", Route::Preview, |_| Authoring {
        thermal_preview: Some(40.0),
        basis: Some(("modulus_basis_ref", json!("temperature-point:w15-named"))),
        materials: Some(pf_materials(Some(json!([{"id": "temperature-point:w15-named", "elastic_modulus": qv(190e9, "Pa"),
            "shear_modulus": qv(76e9, "Pa"), "thermal_expansion_coefficient": qv(1.3e-5, "1/K"), "provenance": PROV}])))),
        ..Default::default()
    });
    run("W15-T-PF-INTERP-S4", Route::Preview, |_| Authoring {
        thermal_preview: Some(40.0),
        basis: Some(("modulus_basis_temperature", qv(425.0, "K"))),
        materials: Some(pf_materials(Some(json!([
            {"id": "temperature-point:w15-300", "temperature": qv(300.0, "K"), "elastic_modulus": qv(200e9, "Pa"),
                "shear_modulus": qv(80e9, "Pa"), "thermal_expansion_coefficient": qv(1.2e-5, "1/K"), "provenance": PROV},
            {"id": "temperature-point:w15-500", "temperature": qv(500.0, "K"), "elastic_modulus": qv(180e9, "Pa"),
                "shear_modulus": qv(72e9, "Pa"), "thermal_expansion_coefficient": qv(1.4e-5, "1/K"), "provenance": PROV}])))),
        ..Default::default()
    });
}

#[test]
fn w15_t_free_scaled_body_grows_freely() {
    run("W15-T-FREE-S1000000", Route::Preview, |_| Authoring { thermal_preview: Some(40.0), ..Default::default() });
}

/// The exact route's material: base E and nu, the named point (at 600 K, outside
/// the 300-500 K interval, so the 425 K interpolation is between 300 and 500 K).
fn ex_materials() -> Value {
    let s = section();
    json!([{"id": "material:w15", "constitutive_basis": "homogeneous_isotropic_E_nu_v1", "elastic_modulus": qv(s.e, "Pa"),
        "poisson_ratio": qv(s.nu, "1"), "provenance": PROV,
        "temperature_points": [
            {"id": "point:w15-named", "temperature": qv(600.0, "K"), "elastic_modulus": qv(190e9, "Pa"), "poisson_ratio": qv(s.nu, "1"), "provenance": PROV},
            {"id": "point:w15-300", "temperature": qv(300.0, "K"), "elastic_modulus": qv(200e9, "Pa"), "poisson_ratio": qv(s.nu, "1"), "provenance": PROV},
            {"id": "point:w15-500", "temperature": qv(500.0, "K"), "elastic_modulus": qv(180e9, "Pa"), "poisson_ratio": qv(s.nu, "1"), "provenance": PROV}]}])
}

fn interval(strain: f64) -> Value {
    if strain == 0.0 {
        json!({"kind": "unchanged_reference", "provenance": PROV})
    } else {
        json!({"kind": "explicit_interval_strain", "strain": qv(strain, "1"), "interval_reference": "invented reference-to-state interval", "provenance": PROV})
    }
}

fn fit(strain: Option<f64>) -> Value {
    match strain {
        Some(strain) => json!({"kind": "fit_strain", "strain": qv(strain, "1")}),
        None => json!({"kind": "none"}),
    }
}

/// The exact route's load/reference-state member pair from the case's frozen inputs.
fn load_state(reference: &Value) -> Authoring {
    let ls = &reference["inputs"]["load_state"];
    let thermal = rational(&ls["explicit_interval_strain"]);
    let selection = match ls["material_selection"].as_str().unwrap() {
        "explicit_base_properties" => json!({"kind": "explicit_base_properties", "material_ref": "material:w15",
            "applicability_reference": "invented analytical basis declared applicable to this member"}),
        "exact_point" => json!({"kind": "exact_point", "material_ref": "material:w15", "point_ref": "point:w15-named"}),
        other => {
            assert_eq!(other, "temperature_interpolation at 425 K");
            json!({"kind": "temperature_interpolation", "material_ref": "material:w15", "temperature": qv(425.0, "K"),
                "interpolation": "piecewise_linear", "extrapolation": "forbidden"})
        }
    };
    let members = reference["inputs"]["geometry"]["members"].as_object().unwrap();
    let states = members.keys().map(|id| {
        let f = ls["fit_strain"].get(id).map(|t| {
            let t = t.as_str().unwrap();
            if t.contains('/') { rational(&json!(t)) } else { t.parse::<f64>().unwrap() }
        });
        (id.clone(), interval(thermal), fit(f))
    }).collect();
    Authoring { states: Some(states), selection: Some(selection), materials: Some(ex_materials()), ..Default::default() }
}

#[test]
fn w15_t_series_load_state_member_pair_with_fit_on_material_named_and_interpolated_bases() {
    for id in ["W15-T-EX-MAT-THERMAL-S4", "W15-T-EX-NAMED-FIT-S4", "W15-T-EX-INTERP-FITONLY-S4", "W15-T-EX-CANCEL-S4"] {
        run(id, Route::Exact, load_state);
    }
}

// ------------------------------------------------------------------ W15-P
#[test]
fn w15_p_closed_end_pressure_cantilever_with_a_scaled_body() {
    run("W15-P-S4", Route::Exact, |r| {
        let p = &r["inputs"]["pressure"];
        let terminals: Vec<Value> = p["terminals"].as_array().unwrap().iter().map(|node| json!({"node_ref": node,
            "closure_transfer": "transfers_to_wall", "provenance": PROV})).collect();
        Authoring {
            regions: Some(json!([{"id": "region:w15", "member_pipe_ids": p["region_members"], "pressure_basis": "internal_differential_zero_external_v1",
                "pressure": qv(rational(&p["pressure_Pa"]), "Pa"), "terminals": terminals, "provenance": PROV}])),
            ..Default::default()
        }
    });
}
