//! W-9 product-path solver benchmarks (PRD section 22.2): a sharp-corner L
//! frame under a uniform temperature rise, a simply supported pipe under its
//! own metal weight, and the L frame under an imposed anchor settlement.
//!
//! Every expectation is read from the frozen references in
//! `validation/references/w9_benchmarks/references.json` (exact closed forms
//! with an exact direct-stiffness cross-check; see that package's README),
//! pinned here to the hash recorded in its SHA256SUMS. Nothing here derives an
//! expectation from product code. Model inputs are invented.
//!
//! Routes. W9-A and W9-B run on the default preview route (PF) and on the
//! exact route (`3.0.0/exact_pressure_v3`, Euler-Bernoulli, load/reference-state
//! group, `pressure_regions: []`). W9-C runs only on the exact route: an imposed
//! support motion exists only as `analysis_state.support_states[].boundary_motion`.
//! W9-B's load is the product's own self-weight generation applied as drafted.
//!
//! Criterion (DEC-024/DEC-026, unchanged): |obs - exp| <= 1e-9 |exp|; an
//! exactly zero expectation uses 1e-9 times the case's frozen zero scale of
//! its family. Both solver modes run against the same reference.
use open_pipe_stress_product_physics::self_weight::{generate_self_weight_operations, SelfWeightGravity, SelfWeightRequest};
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, MechanicsEnvelope, PreviewModel, PreviewSolverMode};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const REFERENCES: &str = include_str!("../../../validation/references/w9_benchmarks/references.json");
const SUMS: &str = include_str!("../../../validation/references/w9_benchmarks/SHA256SUMS");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const RELATIVE: f64 = 1.0e-9;
const PROV: &str = "w9 invented benchmark input; not library, component, material-catalog or code data";
const CASE: &str = "case:w9";
const PF_SEMANTICS: &str = "openpipestress.result_semantics/0.3.0/preview-physics-2";
const EX_SEMANTICS: &str = "openpipestress.result_semantics/0.3.0/pressure-2";
const STATE_CONTRACT: &str = "openpipestress.load_reference_state/1.0.0";
const REFERENCE: &str = "reference:installed";

#[derive(Clone, Copy, PartialEq)]
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
}

// ------------------------------------------------------------------ comparison
fn close(observed: f64, expected: f64, zero_scale: f64, what: &str) -> f64 {
    let tolerance = RELATIVE * if expected == 0.0 { zero_scale.abs() } else { expected.abs() };
    assert!(
        observed.is_finite() && (observed - expected).abs() <= tolerance,
        "{what}: observed {observed:.17e} expected {expected:.17e} tolerance {tolerance:.3e}"
    );
    if expected == 0.0 {
        (observed / zero_scale).abs()
    } else {
        ((observed - expected) / expected).abs()
    }
}

fn row(out: &MechanicsEnvelope, kind: &str, entity: &str, component: &str, location: Option<&str>) -> (f64, String) {
    let found: Vec<_> = out
        .results
        .iter()
        .filter(|r| {
            r.kind == kind
                && r.entity_ref == entity
                && r.basis_ref.as_ref().is_some_and(|b| b.ref_type == "load_case" && b.ref_id == CASE)
                && r.metadata.as_ref().is_some_and(|m| m.component == component && location.is_none_or(|l| m.location == l))
        })
        .collect();
    assert_eq!(found.len(), 1, "one row {kind} {entity} {component} {location:?}");
    (found[0].value, found[0].unit.clone())
}

/// Compare every frozen row of a case; returns the largest relative error
/// (nonzero rows) and the largest |obs|/zero_scale (zero rows).
fn compare(out: &MechanicsEnvelope, reference: &Value, label: &str) -> (f64, f64) {
    let scales = &reference["zero_scales"];
    let scale = |unit: &str| {
        let family = match unit {
            "N" => "force",
            "N*m" => "moment",
            "m" => "translation",
            "rad" => "rotation",
            other => panic!("unit {other}"),
        };
        exact_rational(&scales[family])
    };
    let (mut rel, mut zero) = (0.0f64, 0.0f64);
    let mut count = 0;
    for r in reference["rows"].as_array().unwrap() {
        let (entity, component, unit) = (r["entity"].as_str().unwrap(), r["component"].as_str().unwrap(), r["unit"].as_str().unwrap());
        let expected = exact_rational(r);
        let zs = scale(unit);
        let mut check = |observed: f64, expected: f64, what: String| {
            let e = close(observed, expected, zs, &what);
            if expected == 0.0 {
                zero = zero.max(e)
            } else {
                rel = rel.max(e)
            }
            count += 1;
        };
        match r["quantity"].as_str().unwrap() {
            "support_reaction" => {
                let (v, u) = row(out, "support_reaction_component_v2", entity, component, None);
                assert_eq!(u, unit);
                check(v, expected, format!("{label} {entity} {component}"));
            }
            "nodal_motion" => {
                let axis = &component[1..];
                let (v, u) = if component.starts_with('u') {
                    let (v, u) = row(out, &format!("global_nodal_displacement_{axis}"), entity, &format!("nodal_displacement_{axis}"), None);
                    assert_eq!(u, "mm");
                    (v / 1000.0, "m".to_string())
                } else {
                    row(out, &format!("global_nodal_rotation_{axis}"), entity, &format!("nodal_rotation_{axis}"), None)
                };
                assert_eq!(u, unit);
                check(v, expected, format!("{label} {entity} {component}"));
            }
            "section_action" => {
                let kind = format!("element_local_{component}");
                let station = r["station"].as_str().unwrap();
                let (location, sign) = match station {
                    "0" => ("end_i", -1.0),
                    "1/4" => ("quarter_1", 1.0),
                    "1/2" => ("midspan", 1.0),
                    "3/4" => ("quarter_3", 1.0),
                    "1" => ("end_j", 1.0),
                    s => panic!("station {s}"),
                };
                let (v, u) = row(out, &kind, entity, component, Some(location));
                assert_eq!(u, unit);
                check(v, sign * expected, format!("{label} {entity} {component} {location}"));
            }
            q => panic!("quantity {q}"),
        }
    }
    assert_eq!(count, reference["rows"].as_array().unwrap().len());
    (rel, zero)
}

// ------------------------------------------------------------------ model authoring
struct Frame {
    nodes: Vec<(String, [f64; 3])>,
    pipes: Vec<(String, String, String, [f64; 3])>,
    supports: Vec<(String, String, Vec<String>)>,
}

fn frame(reference: &Value) -> Frame {
    let g = &reference["inputs"]["geometry"];
    let vec3 = |v: &Value| -> [f64; 3] {
        let a = v.as_array().unwrap();
        [a[0].as_f64().unwrap(), a[1].as_f64().unwrap(), a[2].as_f64().unwrap()]
    };
    Frame {
        nodes: g["nodes"].as_object().unwrap().iter().map(|(k, v)| (k.clone(), vec3(v))).collect(),
        pipes: g["pipes"]
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.clone(), v["from"].as_str().unwrap().into(), v["to"].as_str().unwrap().into(), vec3(&v["y_reference"])))
            .collect(),
        supports: g["supports"]
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.clone(), v["node"].as_str().unwrap().into(), v["restraints"].as_array().unwrap().iter().map(|r| r.as_str().unwrap().into()).collect()))
            .collect(),
    }
}

struct Section {
    od: f64,
    wall: f64,
    e: f64,
    g: f64,
    nu: f64,
}
fn section(reference: &Value) -> Section {
    let s = &reference["inputs"]["section"];
    Section {
        od: input_f64(&s["outside_diameter_m"]),
        wall: input_f64(&s["wall_thickness_m"]),
        e: input_f64(&s["elastic_modulus_Pa"]),
        g: input_f64(&s["shear_modulus_Pa_preview_route"]),
        nu: input_f64(&s["poisson_ratio_exact_route"]),
    }
}

fn qv(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}

/// The request for either route. `loads` are primitive loads; `thermal`
/// (alpha 1/K, deltaT K) and `motion` (support, dof, m) apply per route.
fn request(reference: &Value, route: Route, density: Option<f64>, loads: Vec<Value>, thermal: Option<(f64, f64)>, motion: Option<(&str, &str, f64)>) -> Value {
    let f = frame(reference);
    let s = section(reference);
    let nodes: Vec<Value> = f.nodes.iter().map(|(id, p)| json!({"id": id, "position": {"x": p[0], "y": p[1], "z": p[2]}, "provenance": PROV})).collect();
    let pipes: Vec<Value> = f
        .pipes
        .iter()
        .map(|(id, from, to, y)| {
            let mut sec = json!({"outside_diameter": qv(s.od, "m"), "wall_thickness": qv(s.wall, "m")});
            if route == Route::Exact {
                sec["corrosion_allowance"] = qv(0.0, "m");
            }
            if let Some(rho) = density {
                sec["material_density"] = qv(rho, "kg/m^3");
            }
            json!({"id": id, "from": from, "to": to, "section": sec, "material": "material:w9",
                "y_reference": {"x": y[0], "y": y[1], "z": y[2]}, "provenance": PROV})
        })
        .collect();
    let supports: Vec<Value> = f.supports.iter().map(|(id, node, r)| json!({"id": id, "node": node, "family": "anchor", "restraints": r, "provenance": PROV})).collect();
    let units = json!({"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"});
    let status = json!({"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"});
    match route {
        Route::Preview => {
            assert!(motion.is_none(), "the preview route has no support-motion input");
            let mut primitive = loads;
            if let Some((alpha, dt)) = thermal {
                assert_eq!(alpha, 1.2e-5, "the PF material carries alpha");
                for (id, ..) in &f.pipes {
                    primitive.push(json!({"id": format!("thermal:{id}"), "category": "thermal", "target": {"type": "element", "pipe": id},
                        "direction": "global_x", "magnitude": qv(dt, "degC"), "dimension": "temperature_interval", "provenance": PROV}));
                }
            }
            json!({"model": {"schema_version": "0.7.0", "document_kind": "openpipestress.product_preview.model",
                "project": {"id": "project:w9", "units": units}, "analysis_status": status,
                "nodes": nodes, "pipe_segments": pipes, "supports": supports, "components": [],
                "materials": [{"id": "material:w9", "elastic_modulus": qv(s.e, "Pa"), "shear_modulus": qv(s.g, "Pa"),
                    "thermal_expansion_coefficient": qv(1.2e-5, "1/degC"), "provenance": PROV}],
                "load_cases": [{"id": CASE, "primitive_loads": primitive, "provenance": PROV}], "combinations": []},
                "materials": []})
        }
        Route::Exact => {
            let element_states: Vec<Value> = f
                .pipes
                .iter()
                .map(|(id, ..)| {
                    let thermal_state = match thermal {
                        Some((alpha, dt)) => json!({"kind": "constant_alpha_interval", "coefficient": qv(alpha, "1/K"),
                            "temperature_change": qv(dt, "K"), "coefficient_meaning": "engineering_interval", "provenance": PROV}),
                        None => json!({"kind": "unchanged_reference", "provenance": PROV}),
                    };
                    json!({"pipe_ref": id, "material_selection": {"kind": "explicit_base_properties", "material_ref": "material:w9",
                        "applicability_reference": "invented analytical basis declared applicable to this member"}, "thermal_state": thermal_state})
                })
                .collect();
            let support_states: Vec<Value> = f
                .supports
                .iter()
                .map(|(id, ..)| {
                    let mut state = json!({"support_ref": id, "participation": {"kind": "active_model_device"}});
                    if let Some((sup, dof, value)) = motion {
                        if sup == id {
                            state["boundary_motion"] = json!([{"dof": dof, "value": qv(value, "m"), "meaning": "absolute_reference_displacement"}]);
                        }
                    }
                    state
                })
                .collect();
            let members: Vec<Value> = f.pipes.iter().map(|(id, ..)| json!({"pipe_ref": id, "basis": {"kind": "direct_strain_reference"}, "fit": {"kind": "none"}, "provenance": PROV})).collect();
            json!({"model": {"schema_version": "0.7.0", "document_kind": "openpipestress.product_preview.model",
                "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"},
                "project": {"id": "project:w9", "units": units}, "analysis_status": status,
                "nodes": nodes, "pipe_segments": pipes, "supports": supports, "components": [],
                "materials": [{"id": "material:w9", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
                    "elastic_modulus": qv(s.e, "Pa"), "poisson_ratio": qv(s.nu, "1"), "provenance": PROV}],
                "reference_configurations": [{"id": REFERENCE, "label": "Installed reference", "geometry_ref": {"kind": "authored_model_geometry"},
                    "member_references": members, "provenance": PROV}],
                "load_cases": [{"id": CASE, "pressure_regions": [], "primitive_loads": loads,
                    "analysis_state": {"contract": STATE_CONTRACT, "reference_configuration_ref": REFERENCE,
                        "element_states": element_states, "support_states": support_states, "load_sources": [],
                        "history": {"kind": "independent_equilibrium"}, "provenance": PROV},
                    "provenance": PROV}],
                "combinations": []},
                "materials": []})
        }
    }
}

/// W9-B's load: the product's self-weight generation on the request's own
/// model, applied exactly as drafted (no load is authored by hand).
fn with_self_weight(mut wire: Value, reference: &Value) -> (Value, f64) {
    let w = &reference["inputs"]["weight"];
    let gravity = input_f64(&w["gravity_m_s2"]) * input_f64(&w["gravity_sign"]);
    let mut source = wire["model"].clone();
    let existing = source["load_cases"].as_array().unwrap()[0].clone();
    source["load_cases"] = json!([]);
    let digest: String = Sha256::digest(serde_json::to_string(&source).unwrap().as_bytes()).iter().map(|b| format!("{b:02x}")).collect();
    let typed: PreviewModel = serde_json::from_value(source).expect("model parses");
    let request = SelfWeightRequest {
        case_id: CASE.into(),
        label: "W9 metal self weight".into(),
        pipe_refs: frame(reference).pipes.iter().map(|p| p.0.clone()).collect(),
        gravity: SelfWeightGravity { value: gravity, unit: "m/s^2".into(), axis: w["gravity_axis"].as_str().unwrap().into() },
        provenance: PROV.into(),
        source_model_hash: format!("sha256:{digest}"),
    };
    let plan = generate_self_weight_operations(&typed, &request).unwrap_or_else(|d| panic!("self-weight generation refused: {d:?}"));
    let loads: Vec<Value> = plan.changes[1..].iter().map(|c| serde_json::from_str(&c.after).unwrap()).collect();
    assert_eq!(loads.len(), 2);
    let intensity = loads[0]["magnitude"]["value"].as_f64().unwrap();
    assert!(loads.iter().all(|l| l["magnitude"]["value"].as_f64() == Some(intensity) && l["category"] == "distributed_force"));
    let mut case = existing;
    case["primitive_loads"] = json!(loads);
    if let Some(state) = case.get_mut("analysis_state") {
        // A load-state case applies exactly the stored primitives it lists as
        // sources; an unlisted primitive is recorded as excluded.
        state["load_sources"] = json!(loads.iter().map(|l| json!({"source_ref": l["id"], "factor": 1.0})).collect::<Vec<_>>());
    } else {
        let drafted: Value = serde_json::from_str(&plan.changes[0].after).unwrap();
        for key in ["label", "kind", "status"] {
            case[key] = drafted[key].clone();
        }
    }
    wire["model"]["load_cases"] = json!([case]);
    (wire, intensity)
}

fn solved(wire: &Value, route: Route, mode: PreviewSolverMode, label: &str) -> MechanicsEnvelope {
    let out = run_linear_static_preview_value_with_mode(wire.clone(), mode).unwrap_or_else(|e| panic!("{label}: typed boundary rejected the request: {e}"));
    let blocking: Vec<String> = out.diagnostics.iter().filter(|d| d.severity == "blocking" || d.severity == "failure").map(|d| format!("{}: {}", d.code, d.message)).collect();
    assert_eq!(out.status.mechanics, "MECHANICS_SOLVED", "{label}: not solved; blocking {blocking:?}");
    assert!(blocking.is_empty(), "{label}: blocking diagnostics {blocking:?}");
    assert_eq!(out.producer.semantic_contract_id, if route == Route::Preview { PF_SEMANTICS } else { EX_SEMANTICS }, "{label}");
    if route == Route::Exact {
        // Every authored primitive participates: none is recorded as excluded.
        let records = out.contract_evidence.as_ref().expect("contract evidence")["load_reference_states"].as_array().expect("load_reference_states").clone();
        assert_eq!(records.len(), 1, "{label}");
        assert_eq!(records[0]["excluded_sources"], json!([]), "{label}: excluded sources");
    }
    out
}

fn run_case(id: &str, route: Route, build: impl Fn(&Value) -> Value) {
    let reference = case_ref(id);
    let wire = build(&reference);
    for mode in MODES {
        let label = format!("{id} {} {}", if route == Route::Preview { "PF" } else { "EX" }, mode.as_str());
        let out = solved(&wire, route, mode, &label);
        let (rel, zero) = compare(&out, &reference, &label);
        println!("W9 {label}: {} rows; max relative error {rel:.3e}; max |zero row|/scale {zero:.3e}", reference["rows"].as_array().unwrap().len());
    }
}

fn thermal_inputs(reference: &Value) -> (f64, f64) {
    let t = &reference["inputs"]["thermal"];
    (input_f64(&t["alpha_per_K"]), input_f64(&t["delta_T_K"]))
}

// ------------------------------------------------------------------ W9-A
#[test]
fn w9_a_l_frame_thermal_preview_route() {
    run_case("W9-A-LFRAME-THERMAL", Route::Preview, |r| request(r, Route::Preview, None, vec![], Some(thermal_inputs(r)), None));
}

#[test]
fn w9_a_l_frame_thermal_exact_route() {
    run_case("W9-A-LFRAME-THERMAL", Route::Exact, |r| request(r, Route::Exact, None, vec![], Some(thermal_inputs(r)), None));
}

// ------------------------------------------------------------------ W9-B
fn self_weight_case(route: Route) {
    let id = "W9-B-SIMPLE-BEAM-SELF-WEIGHT";
    let reference = case_ref(id);
    let rho = input_f64(&reference["inputs"]["weight"]["material_density_kg_m3"]);
    let (wire, intensity) = with_self_weight(request(&reference, route, Some(rho), vec![], None, None), &reference);
    // The generated intensity is the frozen w (gravity along -Y).
    close(intensity, -exact_rational(&reference["key_values"]["w_N_per_m"]), 0.0, "generated self-weight intensity");
    run_case(id, route, |_| wire.clone());
}

#[test]
fn w9_b_simple_beam_self_weight_preview_route() {
    self_weight_case(Route::Preview);
}

#[test]
fn w9_b_simple_beam_self_weight_exact_route() {
    self_weight_case(Route::Exact);
}

// ------------------------------------------------------------------ W9-C
#[test]
fn w9_c_l_frame_anchor_settlement_exact_route() {
    run_case("W9-C-LFRAME-ANCHOR-SETTLEMENT", Route::Exact, |r| {
        let m = &r["inputs"]["support_motion"];
        assert_eq!((input_f64(&m["UX_m"]), input_f64(&m["RZ_rad"])), (0.0, 0.0));
        request(r, Route::Exact, None, vec![], None, Some((m["support"].as_str().unwrap(), "UY", input_f64(&m["UY_m"]))))
    });
}

