//! W-15 slice b product-path benchmarks (DEL-03-05 Design
//! `rigid-component-stiffness-scaling.md` §3 and §6): a rigid-family component's
//! weight at its centre of gravity, as the self-weight generator emits it and as
//! both routes solve it.
//!
//! Every expectation is read from the frozen references in
//! `validation/references/w15_component_weight/references.json` (exact closed forms
//! with an independent second route; see that package's README), pinned here to the
//! hash recorded in its SHA256SUMS. Nothing here derives an expectation from product
//! code. Model inputs are invented: the body is a valve at the free end mapping the
//! tip member, with the user's weight, centre of gravity (global X/Y/Z from the node,
//! the owner's R10 convention) and stiffness scale; nothing is a catalog or code value.
//!
//! Criterion (DEC-024/DEC-026, unchanged): |obs - exp| <= 1e-9 |exp|; an exactly zero
//! expectation uses 1e-9 times the case's frozen zero scale of its family. Both solver
//! modes run against the same reference, and the product's own readers (with the
//! captured model) admit every envelope.
use open_pipe_stress_product_physics::result_export::reader_verdict;
use open_pipe_stress_product_physics::self_weight::{
    component_load_id, generate_self_weight_operations, inspect_applied_self_weight, AppliedSelfWeightState,
    SelfWeightGravity, SelfWeightOperationPlan, SelfWeightRequest, COMPONENT_WEIGHT_METHOD, COMPONENT_WEIGHT_SCOPE,
};
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, MechanicsEnvelope, PreviewModel, PreviewSolverMode, ResultItem};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const REFERENCES: &str = include_str!("../../../validation/references/w15_component_weight/references.json");
const SUMS: &str = include_str!("../../../validation/references/w15_component_weight/SHA256SUMS");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const RELATIVE: f64 = 1.0e-9;
const PROV: &str = "w15b invented benchmark input; not library, component, material-catalog or code data";
const CASE: &str = "case:w15b";
const COMPONENT: &str = "component:w15b-valve";
/// An invented signed acceleration magnitude: the component weight is the user's
/// force, so only the sign and axis of gravity enter its wrench.
const G: f64 = 9.80665;

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
fn close(observed: f64, expected: f64, zero_scale: f64, what: &str) -> Option<String> {
    let tolerance = RELATIVE * if expected == 0.0 { zero_scale.abs() } else { expected.abs() };
    let within = observed.is_finite() && (observed - expected).abs() <= tolerance;
    (!within).then(|| format!("{what}: observed {observed:.17e} expected {expected:.17e} tolerance {tolerance:.3e}"))
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

/// Every frozen row of a solved case; returns the rows outside the criterion.
fn measure(out: &MechanicsEnvelope, reference: &Value, label: &str) -> Vec<String> {
    let scales = &reference["zero_scales"];
    let scale = |unit: &str| value(&scales[match unit {
        "N" => "force",
        "N*m" => "moment",
        "m" => "translation",
        "rad" => "rotation",
        other => panic!("unit {other}"),
    }]);
    let mut failures = Vec::new();
    for r in reference["rows"].as_array().unwrap() {
        let (entity, component, unit) = (r["entity"].as_str().unwrap(), r["component"].as_str().unwrap(), r["unit"].as_str().unwrap());
        let expected = value(r);
        let (observed, what) = match r["quantity"].as_str().unwrap() {
            "support_reaction" => {
                let (v, u) = one(out, "support_reaction_component_v2", entity, component, None);
                assert_eq!(u, unit);
                (v, format!("{label} {entity} {component}"))
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
                (v, format!("{label} {entity} {component}"))
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
                (sign * v, format!("{label} {entity} {component} {location}"))
            }
            q => panic!("quantity {q}"),
        };
        failures.extend(close(observed, expected, scale(unit), &what));
    }
    failures
}

// ------------------------------------------------------------------ model authoring
fn section() -> (f64, f64, f64, f64, f64) {
    let s = &refs()["section"];
    (rational(&s["outside_diameter_m"]), rational(&s["wall_thickness_m"]), rational(&s["elastic_modulus_Pa"]),
        rational(&s["shear_modulus_Pa"]), rational(&s["poisson_ratio"]))
}

/// The case's model without loads: the chain, its anchor and the valve at the
/// free end with its weight, centre of gravity and (when the case has one) scale.
fn model(reference: &Value, route: Route, timoshenko: bool) -> Value {
    let g = &reference["inputs"]["geometry"];
    let (od, wall, e, shear, nu) = section();
    let nodes: Vec<Value> = g["nodes"].as_object().unwrap().iter()
        .map(|(id, p)| json!({"id": id, "position": {"x": rational(&p[0]), "y": rational(&p[1]), "z": rational(&p[2])}, "provenance": PROV}))
        .collect();
    let pipes: Vec<Value> = g["members"].as_object().unwrap().iter().map(|(id, m)| {
        let mut sec = json!({"outside_diameter": qv(od, "m"), "wall_thickness": qv(wall, "m")});
        if route == Route::Exact {
            sec["corrosion_allowance"] = qv(0.0, "m");
        }
        json!({"id": id, "from": m["from"], "to": m["to"], "section": sec, "material": "material:w15b",
            "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0}, "provenance": PROV})
    }).collect();
    let supports: Vec<Value> = g["supports"].as_object().unwrap().iter()
        .map(|(id, v)| json!({"id": id, "node": v["node"], "family": "anchor", "restraints": v["restraints"], "provenance": PROV}))
        .collect();
    let body = &reference["inputs"]["body"];
    let r = body["center_of_gravity_m"].as_array().unwrap();
    let mut component = json!({"id": COMPONENT, "kind": body["component_kind"], "node": body["component_node"],
        "geometry": {"rigid_pipe_ref": body["member"], "weight": qv(rational(&body["weight_N"]), "N"),
            "center_of_gravity": {"x": rational(&r[0]), "y": rational(&r[1]), "z": rational(&r[2]), "unit": "m"}},
        "mechanics_interface": {"solver_consumption": "mechanics_geometry_only"}, "provenance": PROV});
    if !body["stiffness_scale"].is_null() {
        component["modifiers"] = json!({"stiffness_scaling_user_value": qv(rational(&body["stiffness_scale"]), "1"),
            "source_reference": "invented W-15 slice b benchmark scale"});
    }
    let units = json!({"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"});
    let status = json!({"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"});
    let mut model = json!({"schema_version": "0.8.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:w15b", "units": units}, "analysis_status": status,
        "nodes": nodes, "pipe_segments": pipes, "supports": supports, "components": [component], "combinations": [], "load_cases": []});
    match route {
        Route::Preview => {
            model["materials"] = json!([{"id": "material:w15b", "elastic_modulus": qv(e, "Pa"), "shear_modulus": qv(shear, "Pa"), "provenance": PROV}]);
        }
        Route::Exact => {
            model["pressure_contract"] = json!({"version": "3.0.0", "mode": "exact_pressure_v3",
                "shear_model": if timoshenko { "timoshenko_annulus_energy_v1" } else { "euler_bernoulli" }});
            model["materials"] = json!([{"id": "material:w15b", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
                "elastic_modulus": qv(e, "Pa"), "poisson_ratio": qv(nu, "1"), "provenance": PROV}]);
        }
    }
    model
}

fn gravity(axis: &str, sign: i64) -> SelfWeightGravity {
    SelfWeightGravity { value: sign as f64 * G, unit: "m/s^2".into(), axis: axis.into() }
}

fn generate(model: &Value, pipes: &[&str], gravity: SelfWeightGravity) -> SelfWeightOperationPlan {
    let typed: PreviewModel = serde_json::from_value(model.clone()).expect("typed model");
    let digest: String = Sha256::digest(model.to_string().as_bytes()).iter().map(|b| format!("{b:02x}")).collect();
    let request = SelfWeightRequest {
        case_id: CASE.into(),
        label: "W-15 slice b component weight".into(),
        pipe_refs: pipes.iter().map(|p| p.to_string()).collect(),
        gravity,
        provenance: PROV.into(),
        source_model_hash: format!("sha256:{digest}"),
    };
    generate_self_weight_operations(&typed, &request).unwrap_or_else(|d| panic!("self-weight generation refused: {d:?}"))
}

/// The plan's primitives against a frozen W15-B-WRENCH case: the same set, in
/// order (the force, then each nonzero moment in x, y, z), each id deterministic,
/// each value within the criterion, each evidence row naming the method, the
/// component and the omitted member.
fn check_primitives(plan: &SelfWeightOperationPlan, wrench: &Value, node: &str, member: &str, label: &str) {
    assert_eq!(plan.scope_label, COMPONENT_WEIGHT_SCOPE, "{label}");
    assert!(plan.warnings.is_empty(), "{label}: {:?}", plan.warnings);
    let loads: Vec<Value> = plan.changes[1..].iter().map(|c| serde_json::from_str(&c.after).unwrap()).collect();
    let expected = wrench["primitives"].as_array().unwrap();
    assert_eq!(loads.len(), expected.len(), "{label}: primitive count");
    assert_eq!(plan.source_evidence.len(), expected.len(), "{label}: one evidence row per primitive");
    for ((load, p), evidence) in loads.iter().zip(expected).zip(&plan.source_evidence) {
        let (part, axis) = (p["part"].as_str().unwrap(), p["axis"].as_str().unwrap());
        assert_eq!(load["id"], component_load_id(CASE, COMPONENT, part, axis), "{label}");
        assert_eq!(load["target"], json!({"type": "node", "node": node}), "{label}");
        assert_eq!(load["direction"], p["direction"], "{label}");
        let (category, unit, dimension) = if part == "force" { ("weight", "N", "force") } else { ("weight", "N*m", "moment") };
        assert_eq!((load["category"].as_str(), load["magnitude"]["unit"].as_str(), load["dimension"].as_str()),
            (Some(category), Some(unit), Some(dimension)), "{label}");
        let observed = load["magnitude"]["value"].as_f64().unwrap();
        assert_eq!(close(observed, value(p), 1.0, label), None, "{label} {part} {axis}");
        assert_eq!(evidence["method"], COMPONENT_WEIGHT_METHOD, "{label}");
        assert_eq!(evidence["component_id"], COMPONENT, "{label}");
        assert_eq!(evidence["omitted_pipe_id"], member, "{label}");
        assert_eq!((evidence["wrench_part"].as_str(), evidence["wrench_axis"].as_str()), (Some(part), Some(axis)), "{label}");
        // The whole wrench is recorded on every row, zeros included.
        let rows = wrench["rows"].as_array().unwrap();
        for (i, comp) in ["Fx", "Fy", "Fz"].iter().enumerate() {
            let row = rows.iter().find(|r| r["component"] == *comp).unwrap();
            assert_eq!(close(evidence["force_N"][i].as_f64().unwrap(), value(row), value(&wrench["zero_scales"]["force"]), label), None);
        }
        for (i, comp) in ["Mx", "My", "Mz"].iter().enumerate() {
            let row = rows.iter().find(|r| r["component"] == *comp).unwrap();
            assert_eq!(close(evidence["moment_N_m"][i].as_f64().unwrap(), value(row), value(&wrench["zero_scales"]["moment"]), label), None);
        }
    }
}

// ------------------------------------------------------------------ W15-B-WRENCH
#[test]
fn w15b_wrench_every_gravity_axis_and_sign_and_each_offset_axis() {
    let base = case_ref("W15-B-CANT-XYZ-G-Y-S4-EB");
    let mut count = 0;
    for wrench in refs()["cases"].as_array().unwrap().iter().filter(|c| c["family"] == "component_weight_wrench") {
        let inputs = &wrench["inputs"];
        let mut m = model(&base, Route::Preview, false);
        let r = inputs["center_of_gravity_m"].as_array().unwrap();
        m["components"][0]["geometry"]["weight"] = qv(rational(&inputs["weight_N"]), "N");
        m["components"][0]["geometry"]["center_of_gravity"] = json!({"x": rational(&r[0]), "y": rational(&r[1]), "z": rational(&r[2]), "unit": "m"});
        let plan = generate(&m, &["body:c-d"], gravity(inputs["gravity_axis"].as_str().unwrap(), inputs["gravity_sign"].as_i64().unwrap()));
        check_primitives(&plan, wrench, "node:d", "body:c-d", wrench["id"].as_str().unwrap());
        // The same weight in kN with the centre of gravity in mm, and another
        // acceleration magnitude, give the same primitives.
        let mut scaled = m.clone();
        scaled["components"][0]["geometry"]["weight"] = qv(rational(&inputs["weight_N"]) / 1000.0, "kN");
        scaled["components"][0]["geometry"]["center_of_gravity"] = json!({"x": rational(&r[0]) * 1000.0, "y": rational(&r[1]) * 1000.0, "z": rational(&r[2]) * 1000.0, "unit": "mm"});
        let mut g = gravity(inputs["gravity_axis"].as_str().unwrap(), inputs["gravity_sign"].as_i64().unwrap());
        g.value *= 0.25;
        check_primitives(&generate(&scaled, &["body:c-d"], g), wrench, "node:d", "body:c-d", &format!("{} kN/mm", wrench["id"]));
        count += 1;
    }
    assert_eq!(count, 24);
}

// ------------------------------------------------------------------ W15-B-CANT
fn solve(wire: &Value, route: Route, mode: PreviewSolverMode, label: &str, scaled: bool) -> MechanicsEnvelope {
    let out = run_linear_static_preview_value_with_mode(wire.clone(), mode).unwrap_or_else(|e| panic!("{label}: {e}"));
    let blocking: Vec<String> = out.diagnostics.iter().filter(|d| d.severity == "blocking" || d.severity == "failure").map(|d| format!("{}: {}", d.code, d.message)).collect();
    assert_eq!(out.status.mechanics, "MECHANICS_SOLVED", "{label}: blocking {blocking:?}");
    assert!(blocking.is_empty(), "{label}: {blocking:?}");
    assert_eq!(out.diagnostics.iter().any(|d| d.code == "RIGID_COMPONENT_STIFFNESS_SCALING_APPLIED"), scaled, "{label}: scale applied");
    assert!(!out.diagnostics.iter().any(|d| d.code.starts_with("SELF_WEIGHT")), "{label}: generated loads are fresh");
    // The component weight is classified as weight on both routes: no
    // preview category mapping is stated for its primitives.
    assert!(!out.diagnostics.iter().any(|d| d.code == "LOAD_CATEGORY_PREVIEW_MAPPED"), "{label}: {:?}",
        out.diagnostics.iter().filter(|d| d.code == "LOAD_CATEGORY_PREVIEW_MAPPED").collect::<Vec<_>>());
    let reader = if route == Route::Preview { "preview-physics" } else { "pressure" };
    let verdict = reader_verdict::validate(reader, &serde_json::to_value(&out).unwrap(), Some(&wire["model"]), None);
    assert_eq!(verdict["ok"], true, "{label}: {verdict}");
    out
}

fn run(id: &str, route: Route) {
    let reference = case_ref(id);
    let timoshenko = reference["inputs"]["beam_theory"] == "timoshenko_annulus_energy_v1";
    let sw = &reference["inputs"]["self_weight"];
    let (axis, sign) = (sw["gravity_axis"].as_str().unwrap(), sw["gravity_sign"].as_i64().unwrap());
    let mut m = model(&reference, route, timoshenko);
    let selected: Vec<&str> = sw["selected_members"].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect();
    let plan = generate(&m, &selected, gravity(axis, sign));
    // The plan's primitives are the frozen wrench of the same offset and gravity.
    let body = &reference["inputs"]["body"];
    let offset = match body["center_of_gravity_m"].as_array().unwrap().iter().map(|c| c != "0").collect::<Vec<_>>().as_slice() {
        [true, false, false] => "X",
        [false, true, false] => "Y",
        [false, false, true] => "Z",
        _ => "XYZ",
    };
    let wrench = case_ref(&format!("W15-B-WRENCH-{offset}-G{}{}", if sign > 0 { "+" } else { "-" }, axis.trim_start_matches("global_").to_uppercase()));
    check_primitives(&plan, &wrench, body["component_node"].as_str().unwrap(), body["member"].as_str().unwrap(), id);
    let loads: Vec<Value> = plan.changes[1..].iter().map(|c| serde_json::from_str(&c.after).unwrap()).collect();
    let mut case: Value = serde_json::from_str(&plan.changes[0].after).unwrap();
    case["primitive_loads"] = json!(loads);
    if route == Route::Exact {
        case["pressure_regions"] = json!([]);
    }
    m["load_cases"] = json!([case]);
    let typed: PreviewModel = serde_json::from_value(m.clone()).unwrap();
    assert!(inspect_applied_self_weight(&typed, None).iter().all(|s| s.state == AppliedSelfWeightState::Fresh), "{id}: applied loads are fresh");
    let wire = json!({"model": m, "materials": []});
    let scaled = !body["stiffness_scale"].is_null();
    let mut failures = Vec::new();
    for mode in MODES {
        let label = format!("{id} {route:?} {}", mode.as_str());
        let out = solve(&wire, route, mode, &label, scaled);
        let found = measure(&out, &reference, &label);
        println!("W15B {label}: {} rows; {} outside 1e-9", reference["rows"].as_array().unwrap().len(), found.len());
        if !found.is_empty() {
            failures.push(found.join("\n"));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

#[test]
fn w15b_cantilever_every_gravity_axis_and_sign_both_routes() {
    for g in ["-X", "+X", "-Y", "+Y", "-Z", "+Z"] {
        for route in [Route::Preview, Route::Exact] {
            run(&format!("W15-B-CANT-XYZ-G{g}-S4-EB"), route);
        }
    }
}

#[test]
fn w15b_cantilever_each_offset_axis_both_routes() {
    for offset in ["X", "Y", "Z"] {
        for route in [Route::Preview, Route::Exact] {
            run(&format!("W15-B-CANT-{offset}-G-Y-S4-EB"), route);
        }
    }
}

#[test]
fn w15b_cantilever_timoshenko_exact_route() {
    for g in ["-Y", "-Z"] {
        run(&format!("W15-B-CANT-XYZ-G{g}-S4-TIM"), Route::Exact);
    }
}

#[test]
fn w15b_cantilever_unscaled_body_still_carries_its_weight() {
    for route in [Route::Preview, Route::Exact] {
        run("W15-B-CANT-XYZ-G-Y-S1-EB", route);
    }
}
