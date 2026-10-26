//! Shared W-15c benchmark binding (DEL-03-05 Design `eccentric-reducer-offset.md` §7):
//! reading the frozen references, authoring a case's request on either route, solving
//! and comparing every row. Used by `w15c_eccentric_reducer_benchmarks.rs` and
//! `w15c_offset_convention.rs`.
#![allow(dead_code)]
use open_pipe_stress_product_physics::result_export::reader_verdict;
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, MechanicsEnvelope, PreviewSolverMode, ResultItem};
use serde_json::{json, Value};

pub const REFERENCES: &str = include_str!("../../../../validation/references/w15c_eccentric_reducer/references.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const RELATIVE: f64 = 1.0e-9;
const PROV: &str = "w15c invented benchmark input; not library, component, material-catalog or code data";
const CASE: &str = "case:w15c";
const STATE_CONTRACT: &str = "openpipestress.load_reference_state/1.0.0";
const REFERENCE: &str = "reference:installed";

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Route {
    Preview,
    Exact,
}

pub fn refs() -> Value {
    serde_json::from_str(REFERENCES).expect("references.json parses")
}
pub fn case_ref(id: &str) -> Value {
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

fn close(observed: f64, expected: f64, zero_scale: f64, what: &str) -> (f64, Option<String>) {
    let tolerance = RELATIVE * if expected == 0.0 { zero_scale.abs() } else { expected.abs() };
    let error = if expected == 0.0 { (observed / zero_scale).abs() } else { ((observed - expected) / expected).abs() };
    let within = observed.is_finite() && (observed - expected).abs() <= tolerance;
    (if observed.is_finite() { error } else { f64::INFINITY },
     (!within).then(|| format!("{what}: observed {observed:.17e} expected {expected:.17e} tolerance {tolerance:.3e} error {error:.3e}")))
}

fn rows<'a>(out: &'a MechanicsEnvelope, kind: &str, entity: &str, component: &str, location: Option<&str>) -> Vec<&'a ResultItem> {
    out.results
        .iter()
        .filter(|r| {
            r.kind == kind
                && r.entity_ref == entity
                && r.basis_ref.as_ref().is_some_and(|b| b.ref_type == "load_case" && b.ref_id == CASE)
                && r.metadata.as_ref().is_some_and(|m| m.component == component && location.is_none_or(|l| m.location == l))
        })
        .collect()
}

fn one(out: &MechanicsEnvelope, kind: &str, entity: &str, component: &str, location: Option<&str>) -> (f64, String) {
    let found = rows(out, kind, entity, component, location);
    assert_eq!(found.len(), 1, "one row {kind} {entity} {component} {location:?}");
    (found[0].value, found[0].unit.clone())
}

/// Every frozen row of a case: (largest relative error, largest |zero row|/scale, failures).
pub fn measure(out: &MechanicsEnvelope, reference: &Value, label: &str) -> (f64, f64, Vec<String>) {
    let scales = &reference["zero_scales"];
    let scale = |unit: &str| value(&scales[match unit {
        "N" => "force",
        "N*m" => "moment",
        "m" => "translation",
        "rad" => "rotation",
        other => panic!("unit {other}"),
    }]);
    let (mut rel, mut zero) = (0.0f64, 0.0f64);
    let mut failures = Vec::new();
    for r in reference["rows"].as_array().unwrap() {
        let (entity, component, unit) = (r["entity"].as_str().unwrap(), r["component"].as_str().unwrap(), r["unit"].as_str().unwrap());
        let expected = value(r);
        let zs = scale(unit);
        let mut check = |observed: f64, expected: f64, what: String| {
            let (e, failure) = close(observed, expected, zs, &what);
            if expected == 0.0 { zero = zero.max(e) } else { rel = rel.max(e) }
            failures.extend(failure);
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
            q => panic!("quantity {q}"),
        }
    }
    (rel, zero, failures)
}

/// Route-specific authoring of one case.
#[derive(Default)]
pub struct Authoring {
    pub loads: Vec<Value>,
    /// The free thermal strain on every member: PF element thermal loads, EX the
    /// load/reference-state member pair.
    pub thermal: bool,
    pub timoshenko: bool,
}

pub fn request(reference: &Value, route: Route, a: Authoring) -> Value {
    let g = &reference["inputs"]["geometry"];
    let doc = refs();
    let sections = &doc["sections"];
    let material = &doc["material"];
    let nodes: Vec<Value> = g["nodes"].as_object().unwrap().iter()
        .map(|(id, p)| json!({"id": id, "position": {"x": rational(&p[0]), "y": rational(&p[1]), "z": rational(&p[2])}, "provenance": PROV}))
        .collect();
    let members = g["members"].as_object().unwrap();
    let pipes: Vec<Value> = members.iter().map(|(id, m)| {
        let s = &sections[m["section"].as_str().unwrap()];
        let (od, idia) = (rational(&s["outside_diameter_m"]), rational(&s["inside_diameter_m"]));
        let mut sec = json!({"outside_diameter": qv(od, "m"), "wall_thickness": qv((od - idia) / 2.0, "m")});
        if route == Route::Exact {
            sec["corrosion_allowance"] = qv(0.0, "m");
        }
        let y = &m["y_reference"];
        json!({"id": id, "from": m["from"], "to": m["to"], "section": sec, "material": "material:w15c",
            "y_reference": {"x": y[0], "y": y[1], "z": y[2]}, "provenance": PROV})
    }).collect();
    let supports: Vec<Value> = g["supports"].as_object().unwrap().iter()
        .map(|(id, v)| json!({"id": id, "node": v["node"], "family": "anchor", "restraints": v["restraints"], "provenance": PROV}))
        .collect();
    let r = &reference["inputs"]["reducer"];
    let e = r["eccentric_offset_mm"].as_array().unwrap();
    let component = json!({"id": "component:w15c-reducer", "kind": r["component_kind"], "node": r["component_node"],
        "geometry": {"rigid_pipe_ref": r["member"], "rigid_body_length": qv(rational(&r["rigid_body_length_m"]), "m"),
            "eccentric_offset": {"x": rational(&e[0]), "y": rational(&e[1]), "z": rational(&e[2]), "unit": "mm"}},
        "modifiers": {"stiffness_scaling_user_value": qv(rational(&r["stiffness_scale"]), "1"), "source_reference": "invented W-15c benchmark scale"},
        "mechanics_interface": {"solver_consumption": "mechanics_geometry_only"}, "provenance": PROV});
    let units = json!({"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"});
    let status = json!({"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"});
    let mut case = json!({"id": CASE, "primitive_loads": a.loads, "provenance": PROV});
    let mut model = json!({"schema_version": "0.7.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:w15c", "units": units}, "analysis_status": status,
        "nodes": nodes, "pipe_segments": pipes, "supports": supports, "components": [component], "combinations": []});
    let (e_pa, g_pa, nu) = (rational(&material["elastic_modulus_Pa"]), rational(&material["shear_modulus_Pa"]), rational(&material["poisson_ratio"]));
    let alpha = rational(&material["thermal_expansion_per_K"]);
    match route {
        Route::Preview => {
            if a.thermal {
                let dt = rational(&reference["inputs"]["loads"]["temperature_change_K"]);
                for id in members.keys() {
                    case["primitive_loads"].as_array_mut().unwrap().push(json!({"id": format!("thermal:{id}"), "category": "thermal",
                        "target": {"type": "element", "pipe": id}, "direction": "global_x", "magnitude": qv(dt, "degC"),
                        "dimension": "temperature_interval", "provenance": PROV}));
                }
            }
            model["materials"] = json!([{"id": "material:w15c", "elastic_modulus": qv(e_pa, "Pa"), "shear_modulus": qv(g_pa, "Pa"),
                "thermal_expansion_coefficient": qv(alpha, "1/degC"), "provenance": PROV}]);
        }
        Route::Exact => {
            model["pressure_contract"] = json!({"version": "3.0.0", "mode": "exact_pressure_v3",
                "shear_model": if a.timoshenko { "timoshenko_annulus_energy_v1" } else { "euler_bernoulli" }});
            model["materials"] = json!([{"id": "material:w15c", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
                "elastic_modulus": qv(e_pa, "Pa"), "poisson_ratio": qv(nu, "1"), "provenance": PROV}]);
            case["pressure_regions"] = json!([]);
            if a.thermal {
                let strain = rational(&reference["inputs"]["loads"]["free_strain"]);
                let selection = json!({"kind": "explicit_base_properties", "material_ref": "material:w15c",
                    "applicability_reference": "invented analytical basis declared applicable to this member"});
                let element_states: Vec<Value> = members.keys().map(|id| json!({"pipe_ref": id, "material_selection": selection,
                    "thermal_state": {"kind": "explicit_interval_strain", "strain": qv(strain, "1"),
                        "interval_reference": "invented reference-to-state interval", "provenance": PROV}})).collect();
                let references: Vec<Value> = members.keys().map(|id| json!({"pipe_ref": id,
                    "basis": {"kind": "direct_strain_reference"}, "fit": {"kind": "none"}, "provenance": PROV})).collect();
                let support_states: Vec<Value> = g["supports"].as_object().unwrap().keys()
                    .map(|id| json!({"support_ref": id, "participation": {"kind": "active_model_device"}})).collect();
                model["reference_configurations"] = json!([{"id": REFERENCE, "label": "Installed reference",
                    "geometry_ref": {"kind": "authored_model_geometry"}, "member_references": references, "provenance": PROV}]);
                case["analysis_state"] = json!({"contract": STATE_CONTRACT, "reference_configuration_ref": REFERENCE,
                    "element_states": element_states, "support_states": support_states, "load_sources": [],
                    "history": {"kind": "independent_equilibrium"}, "provenance": PROV});
            }
        }
    }
    model["load_cases"] = json!([case]);
    json!({"model": model, "materials": []})
}

pub fn solve(wire: &Value, route: Route, mode: PreviewSolverMode, label: &str) -> MechanicsEnvelope {
    let out = run_linear_static_preview_value_with_mode(wire.clone(), mode).unwrap_or_else(|e| panic!("{label}: {e}"));
    let blocking: Vec<String> = out.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| format!("{}: {}", d.code, d.message)).collect();
    assert_eq!(out.status.mechanics, "MECHANICS_SOLVED", "{label}: blocking {blocking:?}");
    let realized: Vec<_> = out.diagnostics.iter().filter(|d| d.code == "RIGID_COMPONENT_ECCENTRIC_OFFSET_REALIZED").collect();
    assert_eq!(realized.len(), 1, "{label}: one realized-offset info");
    assert!(out.diagnostics.iter().any(|d| d.code == "RIGID_COMPONENT_STIFFNESS_SCALING_APPLIED"), "{label}: the scale is applied");
    let reader = if route == Route::Preview { "preview-physics" } else { "pressure" };
    let verdict = reader_verdict::validate(reader, &serde_json::to_value(&out).unwrap(), Some(&wire["model"]), None);
    assert_eq!(verdict["ok"], true, "{label}: {verdict}");
    out
}

pub fn run(id: &str, route: Route, authoring: impl Fn(&Value) -> Authoring) {
    let reference = case_ref(id);
    let wire = request(&reference, route, authoring(&reference));
    let mut failures = Vec::new();
    for mode in MODES {
        let label = format!("{id} {route:?} {}", mode.as_str());
        let out = solve(&wire, route, mode, &label);
        let (rel, zero, fails) = measure(&out, &reference, &label);
        let count = reference["rows"].as_array().unwrap().len();
        println!("W15C {label}: {count} rows; max relative error {rel:.3e}; max |zero row|/scale {zero:.3e}");
        if !fails.is_empty() {
            failures.push(format!("{label}: {} of {count} rows outside the criterion:\n{}", fails.len(), fails.join("\n")));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

pub fn tip_force(reference: &Value) -> Vec<Value> {
    let l = &reference["inputs"]["loads"];
    let node = l["node"].as_str().unwrap();
    let f = l["force_N"].as_array().unwrap();
    ["x", "y", "z"].iter().zip(f).filter(|(_, v)| rational(v) != 0.0).map(|(axis, v)| json!({"id": format!("load:f{axis}"),
        "category": "concentrated_force", "target": {"type": "node", "node": node}, "direction": format!("global_{axis}"),
        "magnitude": qv(rational(v), "N"), "dimension": "force", "provenance": PROV})).collect()
}

