//! W-16 slice c (DEL-05-01 Design `exact-equivalent-static.md` §5): DEC-068
//! equivalent-static generation on the exact pressure route (EX), beside the
//! pressure-free route (PF).
//!
//! The oracle is read from the frozen references in
//! `validation/references/w16_exact_equivalent_static/references.json`
//! (closed-form cantilever statics and flexure in exact rationals, with a
//! unit-load integration cross-check; see that package's README), pinned to
//! the hash recorded in its SHA256SUMS. Nothing here derives an expectation
//! from product code. Model inputs are invented.
//!
//! Coverage (§5): both solver modes; Euler-Bernoulli and Timoshenko; full-
//! and partial-span wind (Timoshenko partial spans against
//! `w16_timoshenko_partial_wind`); arcs (full span,
//! partial blocked); replaced spans (seismic skips, wind refused); shared
//! sections; non-SI inputs; authored and generated loads mixed; the certified
//! source controls (the raw re-formation with an allowance and with an
//! authored zero, the prefix lookalike and the authored/generated collision);
//! the mixed source-recovery envelope; W-15 body rows (the scale multiplies
//! stiffness, not mass); every PF code on EX, the load-state refusal
//! (including a case not listed in the group and an authored null group), the
//! retained refusal; and the record's absence without generation. The
//! combination of a seismic case with a pressure case waits for slice b.
//!
//! Criterion (DEC-024/DEC-026, unchanged): |obs - exp| <= 1e-9 |exp|; an
//! exactly zero expectation uses 1e-9 times the case's frozen zero scale of its
//! family.
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, run_linear_static_preview_value_with_retained_direct, PreviewSolverMode,
    RetainedByteLimit,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

#[path = "common/current_model.rs"]
mod current_model;

const REFERENCES: &str = include_str!("../../../validation/references/w16_exact_equivalent_static/references.json");
const SUMS: &str = include_str!("../../../validation/references/w16_exact_equivalent_static/SHA256SUMS");
const W16T_REFERENCES: &str = include_str!("../../../validation/references/w16_timoshenko_partial_wind/references.json");
const ARC_SKETCHES: &str = include_str!("../../../validation/references/t4_i7/u2_document_sketches.json");
const SOURCE_MIXED: &str = include_str!("../../../fixtures/product_preview/pressure_source/physics_source/mixed.request.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const RELATIVE: f64 = 1.0e-9;
const PROV: &str = "w16c invented benchmark input; not library, component, material-catalog or code data";
const CASE: &str = "case:w16c";
const PF_SEMANTICS: &str = "openpipestress.result_semantics/0.3.0/preview-physics-2";
const EX_SEMANTICS: &str = "openpipestress.result_semantics/0.3.0/pressure-2";
const SOURCE_SEMANTICS: &str = "openpipestress.result_semantics/0.3.0/pressure-source-1";
const EXACT_MASS_METHOD: &str = "source_od_nominal_wall_pressure_bore_areas/v1";
const TIMOSHENKO: &str = "timoshenko_annulus_energy_v1";
const EULER_BERNOULLI: &str = "euler_bernoulli";
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
fn input(path: &str) -> f64 {
    rational(refs()["inputs"].pointer(path).unwrap_or_else(|| panic!("no input {path}")))
}
fn derived(key: &str) -> f64 {
    value(&refs()["derived"][key])
}

#[test]
fn references_are_the_frozen_bytes() {
    let digest: String = Sha256::digest(REFERENCES.as_bytes()).iter().map(|b| format!("{b:02x}")).collect();
    let line = SUMS.lines().find(|l| l.ends_with("  references.json")).expect("SHA256SUMS lists references.json");
    assert_eq!(&line[..64], digest, "the references must equal the frozen bytes");
    let doc = refs();
    assert!(doc["crosscheck"].as_array().unwrap().iter().all(|c| c["agrees"] == true));
    assert_eq!(doc["cases"].as_array().unwrap().len(), 8);
}

// ------------------------------------------------------------------ authoring
fn qv(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}

/// The reference cantilever's section on `route` (PF omits the allowance).
fn section(route: Route) -> Value {
    let mut section = json!({
        "outside_diameter": qv(input("/section/outside_diameter_m"), "m"),
        "wall_thickness": qv(input("/section/wall_thickness_m"), "m"),
        "mill_tolerance": qv(input("/section/mill_tolerance_m"), "m"),
        "material_density": qv(input("/section/material_density_kg_per_m3"), "kg/m^3"),
        "contents_density": qv(input("/section/contents_density_kg_per_m3"), "kg/m^3"),
        "insulation_thickness": qv(input("/section/insulation_thickness_m"), "m"),
        "insulation_density": qv(input("/section/insulation_density_kg_per_m3"), "kg/m^3")});
    if route == Route::Exact {
        section["corrosion_allowance"] = qv(input("/section/corrosion_allowance_m_exact_route"), "m");
    }
    section
}

fn length() -> f64 {
    input("/geometry/length_m")
}

fn node(id: &str, x: f64) -> Value {
    json!({"id": id, "position": {"x": x, "y": 0.0, "z": 0.0}, "provenance": PROV})
}
fn pipe(id: &str, from: &str, to: &str, route: Route) -> Value {
    json!({"id": id, "from": from, "to": to, "section": section(route), "material": "material:w16c",
        "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0}, "provenance": PROV})
}

/// A document on `route` over `nodes` and `pipes`, anchored at node:a.
fn document(route: Route, shear_model: &str, nodes: Vec<Value>, pipes: Vec<Value>, cases: Vec<Value>) -> Value {
    let e = input("/material/elastic_modulus_Pa");
    let material = match route {
        Route::Preview => json!({"id": "material:w16c", "elastic_modulus": qv(e, "Pa"),
            "shear_modulus": qv(input("/material/shear_modulus_Pa_preview_route"), "Pa"), "provenance": PROV}),
        Route::Exact => json!({"id": "material:w16c", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
            "elastic_modulus": qv(e, "Pa"), "poisson_ratio": qv(input("/material/poisson_ratio_exact_route"), "1"), "provenance": PROV}),
    };
    let mut cases = cases;
    if route == Route::Exact {
        for case in &mut cases {
            case.as_object_mut().unwrap().entry("pressure_regions").or_insert(json!([]));
        }
    }
    let mut model = json!({"schema_version": "0.8.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:w16c", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": nodes, "pipe_segments": pipes,
        "supports": [{"id": "support:a", "node": "node:a", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV}],
        "components": [], "materials": [material], "load_cases": cases, "combinations": []});
    if route == Route::Exact {
        model["pressure_contract"] = json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": shear_model});
    }
    json!({"model": model, "materials": []})
}

/// The reference cantilever A(0) - B(L), one pipe.
fn cantilever(route: Route, shear_model: &str, cases: Vec<Value>) -> Value {
    document(route, shear_model, vec![node("node:a", 0.0), node("node:b", length())],
        vec![pipe("pipe:a-b", "node:a", "node:b", route)], cases)
}

fn seismic() -> Value {
    json!({"seismic": {"gravity_acceleration": qv(input("/seismic/gravity_acceleration_m_per_s2"), "m/s^2"),
        "g_factor_y": qv(input("/seismic/g_factor_y"), "1")}, "provenance": PROV})
}
fn wind(refs: &[&str], spans: Vec<(&str, f64, f64)>) -> Value {
    json!({"wind": {"pressure": qv(input("/wind/pressure_Pa"), "Pa"), "shape_factor": qv(input("/wind/shape_factor"), "1"),
        "direction": "global_z", "exposed_pipe_refs": refs,
        "exposed_spans": spans.iter().map(|(p, a, b)| json!({"pipe_ref": p, "start_fraction": qv(*a, "1"), "end_fraction": qv(*b, "1")})).collect::<Vec<_>>()},
        "provenance": PROV})
}
fn case(id: &str, generation: Option<Value>, loads: Vec<Value>) -> Value {
    let mut case = json!({"id": id, "primitive_loads": loads, "provenance": PROV});
    if let Some(generation) = generation {
        case["equivalent_static"] = generation;
    }
    case
}
fn distributed(id: &str, pipe: &str, direction: &str, value: f64) -> Value {
    json!({"id": id, "category": "distributed_force", "target": {"type": "element", "pipe": pipe}, "direction": direction,
        "magnitude": qv(value, "N/m"), "dimension": "force_per_length", "provenance": PROV})
}

/// The request of one reference case.
fn reference_request(reference: &Value) -> (Route, Value) {
    let route = if reference["route"] == "EX" { Route::Exact } else { Route::Preview };
    let shear_model = reference["shear_model"].as_str().unwrap();
    let generation = if reference["load"] == "seismic" {
        seismic()
    } else {
        let (a, b) = (rational(&reference["fractions"][0]), rational(&reference["fractions"][1]));
        if (a, b) == (0.0, 1.0) { wind(&["pipe:a-b"], vec![]) } else { wind(&[], vec![("pipe:a-b", a, b)]) }
    };
    (route, cantilever(route, shear_model, vec![case(CASE, Some(generation), vec![])]))
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
    if route == Route::Preview {
        assert_eq!(out["producer"]["semantic_contract_id"], PF_SEMANTICS, "{what}");
    }
    out
}
/// Every case of a solved envelope passes its numerical integrity checks: the
/// certified producer re-formed each applied load from its raw source.
fn checks_passed(out: &Value, what: &str) {
    assert_eq!(out["numerical_quality"]["status"], "checks_passed", "{what}: {:?}", codes(out));
    for case in out["numerical_quality"]["cases"].as_array().unwrap() {
        assert_eq!(case["solve_quality"], "checks_passed", "{what}: {case}");
    }
}
/// A refused request: no rows, the named blocking code, in both modes.
fn refused(wire: &Value, code: &str, what: &str) -> Vec<Value> {
    let mut found = Vec::new();
    for mode in MODES {
        let out = run(wire, mode);
        assert_eq!(out["status"]["mechanics"], "MODEL_INCOMPLETE", "{what}: {:?}", codes(&out));
        assert!(rows(&out).is_empty(), "{what}: a refused document publishes no rows");
        found = out["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == code).cloned().collect();
        assert!(!found.is_empty(), "{what}: expected {code}, got {:?}", codes(&out));
        assert!(found.iter().all(|d| d["severity"] == "blocking"), "{what}");
    }
    found
}

fn rows(out: &Value) -> &Vec<Value> {
    out["results"].as_array().expect("results")
}

/// The exact case evidence of `case_id`.
fn exact_case<'a>(out: &'a Value, case_id: &str) -> &'a Value {
    out["contract_evidence"]["exact_cases"].as_array().unwrap().iter().find(|c| c["load_case_id"] == case_id)
        .unwrap_or_else(|| panic!("no exact case {case_id}"))
}

// ------------------------------------------------------------------ comparison
fn find<'a>(out: &'a Value, case_id: &str, kind: &str, entity: &str, component: &str, location: Option<&str>, basis: Option<&str>) -> &'a Value {
    let found: Vec<&Value> = rows(out).iter().filter(|row| {
        row["kind"] == kind && row["entity_ref"] == entity && row["basis_ref"]["ref_id"] == case_id
            && row["metadata"]["component"] == component
            && location.is_none_or(|l| row["metadata"]["location"] == l)
            && basis.is_none_or(|b| row["metadata"]["basis"].as_str().is_some_and(|text| text.ends_with(b)))
    }).collect();
    assert_eq!(found.len(), 1, "one row {kind} {entity} {component} {location:?} {basis:?} on {case_id}; candidates {:#?}",
        rows(out).iter().filter(|row| row["kind"] == kind && row["basis_ref"]["ref_id"] == case_id).map(|row| &row["metadata"]).collect::<Vec<_>>());
    found[0]
}

/// One published value in the reference's SI unit (displacements are
/// published in mm, stresses in MPa).
fn observed(out: &Value, case_id: &str, r: &Value) -> f64 {
    let (entity, component, unit) = (r["entity"].as_str().unwrap(), r["component"].as_str().unwrap(), r["unit"].as_str().unwrap());
    let station = |r: &Value| match r["station"].as_str().unwrap() {
        "0" => ("end_i", -1.0),
        "1/4" => ("quarter_1", 1.0),
        "1/2" => ("midspan", 1.0),
        "3/4" => ("quarter_3", 1.0),
        "1" => ("end_j", 1.0),
        s => panic!("station {s}"),
    };
    match r["quantity"].as_str().unwrap() {
        "support_reaction" => {
            let row = find(out, case_id, "support_reaction_component_v2", entity, component, None, None);
            assert_eq!(row["unit"], unit);
            row["value"].as_f64().unwrap()
        }
        "nodal_motion" => {
            let axis = &component[1..];
            if component.starts_with('u') {
                let row = find(out, case_id, &format!("global_nodal_displacement_{axis}"), entity, &format!("nodal_displacement_{axis}"), None, None);
                assert_eq!(row["unit"], "mm");
                row["value"].as_f64().unwrap() / 1000.0
            } else {
                let row = find(out, case_id, &format!("global_nodal_rotation_{axis}"), entity, &format!("nodal_rotation_{axis}"), None, None);
                assert_eq!(row["unit"], unit);
                row["value"].as_f64().unwrap()
            }
        }
        "section_action" => {
            let (location, sign) = station(r);
            let row = find(out, case_id, &format!("element_local_{component}"), entity, component, Some(location), None);
            assert_eq!(row["unit"], unit);
            sign * row["value"].as_f64().unwrap()
        }
        "section_stress" => {
            // Stress rows follow the j-side section action at every station,
            // end_i included (their sign convention), so no end-action sign.
            let (location, _) = station(r);
            // PF publishes one stress row per station (its single section);
            // EX one per station and named basis.
            let basis = match r["section_basis"].as_str().unwrap() {
                "preview_section" => None,
                b => Some(format!("section_basis={b}")),
            };
            let row = find(out, case_id, &format!("element_local_{component}"), entity, component, Some(location), basis.as_deref());
            assert_eq!(row["unit"], "MPa");
            row["value"].as_f64().unwrap() * 1.0e6
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
        "Pa" => "stress",
        other => panic!("unit {other}"),
    };
    value(&reference["zero_scales"][family])
}

fn label(r: &Value) -> String {
    format!("{} {} {} {} {}", r["quantity"].as_str().unwrap(), r["entity"].as_str().unwrap(), r["station"].as_str().unwrap_or(""),
        r["component"].as_str().unwrap(), r["section_basis"].as_str().unwrap_or(""))
}

/// Every frozen row of `reference` (those `keep` selects) against `out`.
/// Returns the row count, the largest relative error (nonzero rows) and the
/// largest |obs|/zero_scale (zero rows).
fn compare(out: &Value, reference: &Value, case_id: &str, what: &str, keep: impl Fn(&Value) -> bool) -> (usize, f64, f64) {
    let (mut count, mut rel, mut zero) = (0, 0.0f64, 0.0f64);
    for r in reference["rows"].as_array().unwrap().iter().filter(|r| keep(r)) {
        let zs = zero_scale(reference, r["unit"].as_str().unwrap());
        let (o, e) = (observed(out, case_id, r), value(r));
        let tol = RELATIVE * if e == 0.0 { zs.abs() } else { e.abs() };
        assert!(o.is_finite() && (o - e).abs() <= tol, "{what} {}: observed {o:.17e} expected {e:.17e} tolerance {tol:.3e}", label(r));
        if e == 0.0 { zero = zero.max((o / zs).abs()) } else { rel = rel.max(((o - e) / e).abs()) }
        count += 1;
    }
    (count, rel, zero)
}
fn every(_: &Value) -> bool {
    true
}
/// The rows a statically determinate response shares with a reference whose
/// member layout differs: the support and the tip.
fn support_and_tip(r: &Value) -> bool {
    r["quantity"] == "support_reaction" || (r["quantity"] == "nodal_motion" && r["entity"] == "node:b")
}

// ------------------------------------------------------------------ the oracle
/// Every reference case on its route, in both modes. EX publishes the
/// generation record; PF has no exact evidence. Prints the worst errors per
/// route and the PF/EX seismic difference.
#[test]
fn every_reference_case_matches_its_closed_form_on_its_route() {
    let (mut worst_pf, mut worst_ex) = (0.0f64, 0.0f64);
    for reference in refs()["cases"].as_array().unwrap() {
        let id = reference["id"].as_str().unwrap();
        let (route, wire) = reference_request(reference);
        for mode in MODES {
            let what = format!("{id} {}", mode.as_str());
            let out = solved(&wire, route, mode, &what);
            checks_passed(&out, &what);
            let (n, rel, zero) = compare(&out, reference, CASE, &what, every);
            println!("W16C {what}: {n} rows; max relative error {rel:.3e}; max |zero row|/scale {zero:.3e}");
            match route {
                Route::Preview => {
                    worst_pf = worst_pf.max(rel);
                    assert!(out["contract_evidence"].get("exact_cases").is_none() || out["contract_evidence"]["exact_cases"].as_array().unwrap().is_empty());
                }
                Route::Exact => {
                    worst_ex = worst_ex.max(rel);
                    assert_eq!(out["producer"]["semantic_contract_id"], EX_SEMANTICS, "{what}");
                    let record = &exact_case(&out, CASE)["equivalent_static"];
                    assert_eq!(record["mass_method"], EXACT_MASS_METHOD, "{what}");
                    let expected = if reference["load"] == "seismic" {
                        json!([format!("{CASE}:generated:seismic:global-y:element-0")])
                    } else if reference["fractions"] == json!(["0", "1"]) {
                        json!([format!("{CASE}:generated:wind:global-z:element-0")])
                    } else {
                        let (a, b) = (rational(&reference["fractions"][0]), rational(&reference["fractions"][1]));
                        json!([format!("{CASE}:generated:wind:global-z:element-0:extent-{a}-{b}")])
                    };
                    assert_eq!(record["generated_load_ids"], expected, "{what}");
                }
            }
        }
    }
    println!("W16C worst relative error: PF {worst_pf:.3e}; EX {worst_ex:.3e}");
}

/// §2: the routes' seismic masses differ by the documented closed form; the
/// support force difference is -(mu_EX - mu_PF) k_g g L. Wind does not depend
/// on mass, so the generated loads (hence the statically determinate support
/// actions) are equal on PF and EX.
#[test]
fn route_specific_seismic_mass_and_equal_wind_loads() {
    let ex_wire = reference_request(&case_ref("W16C-SEISMIC-EX-EB")).1;
    let pf_wire = reference_request(&case_ref("W16C-SEISMIC-PF")).1;
    let fy = |out: &Value| find(out, CASE, "support_reaction_component_v2", "support:a", "Fy", None, None)["value"].as_f64().unwrap();
    let expected = -derived("mass_difference_ex_minus_pf_kg_per_m") * input("/seismic/g_factor_y") * input("/seismic/gravity_acceleration_m_per_s2") * length();
    for mode in MODES {
        let ex = solved(&ex_wire, Route::Exact, mode, "seismic EX");
        let pf = solved(&pf_wire, Route::Preview, mode, "seismic PF");
        let difference = fy(&ex) - fy(&pf);
        let relative = ((difference - expected) / expected).abs();
        println!("W16C PF/EX seismic support-force difference {}: observed {difference:.12e} N, expected {expected:.12e} N, relative {relative:.3e}", mode.as_str());
        assert!(relative <= 1.0e-6, "the PF/EX difference must be the documented mass difference: {difference} vs {expected}");
        assert!((fy(&ex) - fy(&pf)).abs() > 0.0, "the routes' masses differ");
    }
    for (ex_id, pf_id) in [("W16C-WIND-FULL-EX-EB", "W16C-WIND-FULL-PF"), ("W16C-WIND-PARTIAL-EX-EB", "W16C-WIND-PARTIAL-PF")] {
        let ex_wire = reference_request(&case_ref(ex_id)).1;
        let pf_wire = reference_request(&case_ref(pf_id)).1;
        for mode in MODES {
            let ex = solved(&ex_wire, Route::Exact, mode, ex_id);
            let pf = solved(&pf_wire, Route::Preview, mode, pf_id);
            for component in ["Fz", "My"] {
                let (e, p) = (find(&ex, CASE, "support_reaction_component_v2", "support:a", component, None, None)["value"].as_f64().unwrap(),
                    find(&pf, CASE, "support_reaction_component_v2", "support:a", component, None, None)["value"].as_f64().unwrap());
                assert!(((e - p) / p).abs() <= 1.0e-12, "{ex_id} {component}: EX {e} PF {p}");
            }
        }
    }
}

// ------------------------------------------------------------------ Timoshenko and partial spans
/// §3 (W-16t): a proper fraction of a straight span under the Timoshenko
/// member is admitted and takes the exact Timoshenko vector: this file's own
/// cantilever, solved in both modes, matches the W-16t references
/// (`validation/references/w16_timoshenko_partial_wind/`) for spans at either
/// end, in the interior and near full span; exactly [0, 1] through
/// `exposed_spans` is full span and matches the full-span reference;
/// Euler-Bernoulli admits partial spans (the oracle's partial case).
#[test]
fn timoshenko_partial_span_wind_matches_the_reference() {
    let w16t: Value = serde_json::from_str(W16T_REFERENCES).unwrap();
    let w16t_case = |id: &str| w16t["cases"].as_array().unwrap().iter().find(|c| c["id"] == id).unwrap().clone();
    let components = |r: &Value| ["support_reaction", "nodal_motion", "section_action", "section_stress"].contains(&r["quantity"].as_str().unwrap());
    for (id, a, b) in [("W16T-CANT-INTERIOR", 0.2, 0.7), ("W16T-CANT-PROXIMAL", 0.0, 0.5), ("W16T-CANT-DISTAL", 0.5, 1.0),
        ("W16T-CANT-NEARFULL", 0.0, 1.0 - 2f64.powi(-40))] {
        let reference = w16t_case(id);
        let wire = cantilever(Route::Exact, TIMOSHENKO, vec![case(CASE, Some(wind(&[], vec![("pipe:a-b", a, b)])), vec![])]);
        for mode in MODES {
            let what = format!("Timoshenko [{a}, {b}] {}", mode.as_str());
            let out = solved(&wire, Route::Exact, mode, &what);
            checks_passed(&out, &what);
            assert!(!codes(&out).iter().any(|c| c == "EXACT_EQUIVALENT_STATIC_PARTIAL_SPAN_TIMOSHENKO_UNSUPPORTED"));
            let (n, _, _) = compare(&out, &reference, CASE, &what, components);
            assert_eq!(n, reference["rows"].as_array().unwrap().iter().filter(|r| components(r)).count(), "{what}");
            assert!(n >= 50, "{what}: {n} rows");
        }
    }
    let reference = case_ref("W16C-WIND-FULL-EX-TIMOSHENKO");
    let wire = cantilever(Route::Exact, TIMOSHENKO, vec![case(CASE, Some(wind(&[], vec![("pipe:a-b", 0.0, 1.0)])), vec![])]);
    for mode in MODES {
        let out = solved(&wire, Route::Exact, mode, "Timoshenko exposed_spans [0, 1]");
        checks_passed(&out, "Timoshenko [0, 1]");
        compare(&out, &reference, CASE, "Timoshenko exposed_spans [0, 1]", every);
    }
}

// ------------------------------------------------------------------ arcs
fn arc_document(generation: Value) -> Value {
    let sketches: Value = serde_json::from_str(ARC_SKETCHES).unwrap();
    let mut document = current_model::reauthor_request(sketches["sketches"]["U2-L-ANCH-PTW-K2"]["0.3.0"]["document"].clone());
    let model = &mut document["model"];
    for pipe in model["pipe_segments"].as_array_mut().unwrap() {
        pipe["section"]["material_density"] = qv(7850.0, "kg/m^3");
    }
    model["load_cases"] = json!([{"id": CASE, "pressure_regions": [], "primitive_loads": [], "equivalent_static": generation, "provenance": PROV}]);
    document
}

/// §3: a realized arc takes the full-span load per unit arc length, as an
/// authored distributed load of the same intensity on the arc; any explicit
/// extent on an arc stays blocked, [0, 1] included, under either shear model.
#[test]
fn realized_arcs_take_full_span_generation_and_block_explicit_extents() {
    let generated = arc_document(wind(&["pipe:BEND"], vec![]));
    let od = generated["model"]["pipe_segments"][1]["section"]["outside_diameter"]["value"].as_f64().unwrap();
    let intensity = input("/wind/pressure_Pa") * input("/wind/shape_factor") * (od + 2.0 * 0.0);
    let mut authored = arc_document(json!(null));
    authored["model"]["load_cases"][0].as_object_mut().unwrap().remove("equivalent_static");
    authored["model"]["load_cases"][0]["primitive_loads"] = json!([distributed("load:authored-wind", "pipe:BEND", "global_z", intensity)]);
    for mode in MODES {
        let g = solved(&generated, Route::Exact, mode, "arc wind generated");
        let a = solved(&authored, Route::Exact, mode, "arc wind authored");
        checks_passed(&g, "arc wind");
        assert_eq!(exact_case(&g, CASE)["equivalent_static"]["generated_load_ids"], json!([format!("{CASE}:generated:wind:global-z:element-1")]));
        let values = |out: &Value| rows(out).iter().filter(|r| r["kind"] != "linear_solver_mode_basis")
            .map(|r| (r["id"].as_str().unwrap().to_string(), r["value"].as_f64().unwrap())).collect::<Vec<_>>();
        let (gv, av) = (values(&g), values(&a));
        assert_eq!(gv.len(), av.len());
        for ((gi, gx), (ai, ax)) in gv.iter().zip(&av) {
            assert_eq!(gi, ai);
            assert!((gx - ax).abs() <= 1.0e-12 * ax.abs().max(1.0e-30) || gx == ax, "{gi}: generated {gx} authored {ax}");
        }
        // Seismic on every member, the arc included.
        let seismic_out = solved(&arc_document(seismic()), Route::Exact, mode, "arc seismic");
        checks_passed(&seismic_out, "arc seismic");
        assert_eq!(exact_case(&seismic_out, CASE)["equivalent_static"]["generated_load_ids"].as_array().unwrap().len(), 3);
    }
    for shear_model in [EULER_BERNOULLI, TIMOSHENKO] {
        for (a, b) in [(0.2, 0.7), (0.0, 1.0)] {
            let mut wire = arc_document(wind(&[], vec![("pipe:BEND", a, b)]));
            wire["model"]["pressure_contract"]["shear_model"] = json!(shear_model);
            let found = refused(&wire, "LOAD_INPUT_INVALID", &format!("arc [{a}, {b}] {shear_model}"));
            assert!(found.iter().any(|d| d["message"].as_str().unwrap().contains("partial extents are not supported on realized arcs")));
        }
    }
}

// ------------------------------------------------------------------ replaced spans
fn connector(span: &str, node_i: &str, node_j: &str) -> Value {
    json!({"id": "component:w16c-connector", "label": "Invented expansion joint (objective connector)", "kind": "expansion_joint",
        "node": node_j, "provenance": PROV,
        "objective_connector": {"version": "1.0.0", "motion_basis": "symmetric_midpoint_small_rotation_v1",
            "end_i": {"node_ref": node_i, "initial_node_axes_global": [[1, 0, 0], [0, 1, 0], [0, 0, 1]], "offset_local": {"x": 0, "y": 0, "z": 0, "unit": "m"}},
            "end_j": {"node_ref": node_j, "initial_node_axes_global": [[1, 0, 0], [0, 1, 0], [0, 0, 1]], "offset_local": {"x": 0, "y": 0, "z": 0, "unit": "m"}},
            "connector_axes_global": [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
            "installed_reference_temperature": {"value": 20, "unit": "degC"},
            "temperature_applicability": "fixed_installed_parameters_v1", "reference_state": "stress_free",
            "q_ref": {"translation": {"x": 0, "y": 0, "z": 0, "unit": "m"}, "rotation": {"x": 0, "y": 0, "z": 0, "unit": "rad"}},
            "stiffness": {"version": "1.0.0", "representation": "scaled_work_coefficients_v1",
                "translation_scale": {"value": 0.5, "unit": "m"}, "rotation_scale": {"value": 1, "unit": "rad"}, "coefficient_unit": "N*m",
                "upper_triangle": [800000, 0, 0, 0, 0, 0, 225000, 0, 0, 0, 0, 225000, 0, 0, 0, 620000, 0, 0, 480000, 0, 480000],
                "coordinate_order": ["tx", "ty", "tz", "rx", "ry", "rz"],
                "provenance": {"source_reference": PROV, "measurement_restraints": "invented", "basis_transform_reference": "none"}},
            "topology": {"type": "replaces_span", "span_ref": span},
            "calibration": {"kind": "constant_structural_elasticity_v1", "includes_pressure_dependent_tangent": false, "installed_geometry": "authored model geometry"},
            "hardware": {"kind": "untied"}, "pressure_model": {"kind": "unpressurized"},
            "provenance": {"source_reference": PROV, "measurement_restraints": "invented", "basis_transform_reference": "none", "validity_statement": "invented analytical control; no applicability claim"}}})
}

/// The reference cantilever A - B (L) with a tail B - C (1 m) that an objective
/// connector replaces.
fn with_replaced_tail(cases: Vec<Value>) -> Value {
    let mut wire = document(Route::Exact, EULER_BERNOULLI,
        vec![node("node:a", 0.0), node("node:b", length()), node("node:c", length() + 1.0)],
        vec![pipe("pipe:a-b", "node:a", "node:b", Route::Exact), pipe("pipe:b-c", "node:b", "node:c", Route::Exact)], cases);
    wire["model"]["components"] = json!([connector("pipe:b-c", "node:b", "node:c")]);
    wire
}

/// §3: seismic generation skips the replaced span before its mass is
/// evaluated (the surviving pipe keeps element index 0, and the support
/// carries only A - B's load); a wind marking of the replaced span is refused
/// by name in either form; a stored primitive on it keeps its own refusal.
#[test]
fn replaced_spans_are_skipped_by_seismic_and_refused_for_wind() {
    let reference = case_ref("W16C-SEISMIC-EX-EB");
    let wire = with_replaced_tail(vec![case(CASE, Some(seismic()), vec![])]);
    for mode in MODES {
        let out = solved(&wire, Route::Exact, mode, "replaced tail seismic");
        checks_passed(&out, "replaced tail seismic");
        assert_eq!(exact_case(&out, CASE)["equivalent_static"]["generated_load_ids"], json!([format!("{CASE}:generated:seismic:global-y:element-0")]));
        compare(&out, &reference, CASE, "replaced tail seismic", |r| r["quantity"] == "support_reaction");
    }
    for generation in [wind(&["pipe:b-c"], vec![]), wind(&[], vec![("pipe:b-c", 0.2, 0.7)]), wind(&["pipe:a-b"], vec![("pipe:b-c", 0.0, 1.0)])] {
        let wire = with_replaced_tail(vec![case(CASE, Some(generation), vec![])]);
        let found = refused(&wire, "EQUIVALENT_STATIC_WIND_SPAN_REPLACED", "wind on the replaced span");
        assert!(found[0]["affected_refs"].as_array().unwrap().contains(&json!("pipe:b-c")));
    }
    let stored = with_replaced_tail(vec![case(CASE, Some(seismic()), vec![distributed("load:stored", "pipe:b-c", "global_y", 10.0)])]);
    refused(&stored, "JOINT_REPLACED_SPAN_LOAD_UNOWNED", "stored primitive on the replaced span");
}

// ------------------------------------------------------------------ inputs
/// Shared sections: two members A - M - B bound to one shared section
/// generate per member; the support and tip match the one-pipe reference.
#[test]
fn shared_sections_generate_per_member() {
    for (route, id) in [(Route::Exact, "W16C-SEISMIC-EX-EB"), (Route::Preview, "W16C-SEISMIC-PF")] {
        let reference = case_ref(id);
        let mut first = pipe("pipe:a-m", "node:a", "node:m", route);
        let mut second = pipe("pipe:m-b", "node:m", "node:b", route);
        for p in [&mut first, &mut second] {
            p["section_ref"] = json!("section:w16c");
        }
        let mut wire = document(route, EULER_BERNOULLI, vec![node("node:a", 0.0), node("node:m", length() / 2.0), node("node:b", length())],
            vec![first, second], vec![case(CASE, Some(seismic()), vec![])]);
        wire["model"]["sections"] = json!([{"id": "section:w16c", "name": "w16c shared", "section_type": "pipe",
            "properties": {"outside_diameter": qv(input("/section/outside_diameter_m"), "m"), "wall_thickness": qv(input("/section/wall_thickness_m"), "m")},
            "provenance": {"source": PROV}}]);
        for mode in MODES {
            let out = solved(&wire, route, mode, "shared section");
            checks_passed(&out, "shared section");
            compare(&out, &reference, CASE, "shared section", support_and_tip);
            if route == Route::Exact {
                assert_eq!(exact_case(&out, CASE)["equivalent_static"]["generated_load_ids"].as_array().unwrap().len(), 2);
            }
        }
    }
}

/// Non-SI inputs normalize before generation: the section in mm and in,
/// densities in lb/in^3, the wind pressure in kPa.
#[test]
fn non_si_inputs_match_the_si_references() {
    const LB_PER_IN3: f64 = 0.45359237 / (0.0254 * 0.0254 * 0.0254);
    let convert = |wire: &mut Value| {
        for p in wire["model"]["pipe_segments"].as_array_mut().unwrap() {
            let s = &mut p["section"];
            for key in ["outside_diameter", "wall_thickness", "mill_tolerance", "corrosion_allowance"] {
                if let Some(q) = s.get_mut(key) {
                    *q = qv(q["value"].as_f64().unwrap() * 1000.0, "mm");
                }
            }
            s["insulation_thickness"] = qv(s["insulation_thickness"]["value"].as_f64().unwrap() / 0.0254, "in");
            for key in ["material_density", "contents_density", "insulation_density"] {
                s[key] = qv(s[key]["value"].as_f64().unwrap() / LB_PER_IN3, "lb/in^3");
            }
        }
        if let Some(w) = wire["model"]["load_cases"][0]["equivalent_static"].get_mut("wind") {
            w["pressure"] = qv(w["pressure"]["value"].as_f64().unwrap() / 1000.0, "kPa");
        }
    };
    for id in ["W16C-SEISMIC-EX-EB", "W16C-SEISMIC-PF", "W16C-WIND-PARTIAL-EX-EB", "W16C-WIND-FULL-PF"] {
        let reference = case_ref(id);
        let (route, mut wire) = reference_request(&reference);
        convert(&mut wire);
        for mode in MODES {
            let out = solved(&wire, route, mode, &format!("{id} non-SI"));
            checks_passed(&out, id);
            compare(&out, &reference, CASE, &format!("{id} non-SI"), every);
        }
    }
}

/// Authored and generated loads in one case superpose: an authored uniform
/// load w along +Y adds -wL to the support force of the seismic reference.
#[test]
fn authored_and_generated_loads_mix() {
    let w = 125.0;
    for id in ["W16C-SEISMIC-EX-EB", "W16C-SEISMIC-PF"] {
        let reference = case_ref(id);
        let route = if reference["route"] == "EX" { Route::Exact } else { Route::Preview };
        let wire = cantilever(route, EULER_BERNOULLI, vec![case(CASE, Some(seismic()), vec![distributed("load:authored", "pipe:a-b", "global_y", w)])]);
        for mode in MODES {
            let out = solved(&wire, route, mode, "mixed");
            checks_passed(&out, "mixed");
            compare(&out, &reference, CASE, "mixed", |r| r["quantity"] == "support_reaction" && r["component"] != "Fy" && r["component"] != "Mz");
            let fy = find(&out, CASE, "support_reaction_component_v2", "support:a", "Fy", None, None)["value"].as_f64().unwrap();
            let expected = value(reference["rows"].as_array().unwrap().iter().find(|r| r["quantity"] == "support_reaction" && r["component"] == "Fy").unwrap()) - w * length();
            assert!(((fy - expected) / expected).abs() <= RELATIVE, "{id}: {fy} vs {expected}");
            if route == Route::Exact {
                assert_eq!(exact_case(&out, CASE)["equivalent_static"]["generated_load_ids"], json!([format!("{CASE}:generated:seismic:global-y:element-0")]));
            }
        }
    }
}

// ------------------------------------------------------------------ certified-source controls
/// §2: the certified producer forms the generator's mass. With a nonzero
/// allowance the EX mass differs from the preview basis, so a producer on the
/// wrong basis fails the raw-source re-formation; an authored zero allowance
/// is `Some(0)`. An authored load whose id resembles the generated prefix is
/// authored (its re-formation is its authored intensity) and superposes; an
/// exact collision between an authored and a generated id is refused on both
/// routes.
#[test]
fn certified_source_controls() {
    // Nonzero allowance: the oracle case passes its integrity checks (above);
    // an authored zero allowance does too.
    let mut zero = cantilever(Route::Exact, EULER_BERNOULLI, vec![case(CASE, Some(seismic()), vec![])]);
    zero["model"]["pipe_segments"][0]["section"]["corrosion_allowance"] = qv(0.0, "m");
    for mode in MODES {
        let out = solved(&zero, Route::Exact, mode, "zero allowance");
        checks_passed(&out, "zero allowance");
    }
    // The prefix lookalike: authored, in a generating case.
    let lookalike = format!("{CASE}:generated:seismic:global-y:lookalike");
    let w = 40.0;
    for id in ["W16C-SEISMIC-EX-EB", "W16C-SEISMIC-PF"] {
        let reference = case_ref(id);
        let route = if reference["route"] == "EX" { Route::Exact } else { Route::Preview };
        let wire = cantilever(route, EULER_BERNOULLI, vec![case(CASE, Some(seismic()), vec![distributed(&lookalike, "pipe:a-b", "global_y", w)])]);
        for mode in MODES {
            let out = solved(&wire, route, mode, "lookalike");
            checks_passed(&out, "lookalike");
            let fy = find(&out, CASE, "support_reaction_component_v2", "support:a", "Fy", None, None)["value"].as_f64().unwrap();
            let expected = value(reference["rows"].as_array().unwrap().iter().find(|r| r["quantity"] == "support_reaction" && r["component"] == "Fy").unwrap()) - w * length();
            assert!(((fy - expected) / expected).abs() <= RELATIVE, "{id} lookalike: {fy} vs {expected}");
            if route == Route::Exact {
                assert_eq!(exact_case(&out, CASE)["equivalent_static"]["generated_load_ids"], json!([format!("{CASE}:generated:seismic:global-y:element-0")]),
                    "the lookalike is not a generated primitive");
            }
        }
    }
    // The exact collision.
    let collision = format!("{CASE}:generated:seismic:global-y:element-0");
    for route in [Route::Exact, Route::Preview] {
        let wire = cantilever(route, EULER_BERNOULLI, vec![case(CASE, Some(seismic()), vec![distributed(&collision, "pipe:a-b", "global_y", w)])]);
        let found = refused(&wire, "EQUIVALENT_STATIC_GENERATED_ID_COLLISION", "collision");
        assert!(found[0]["affected_refs"].as_array().unwrap().contains(&json!(collision)));
    }
}

/// §2: a document in which another case selects retained-source recovery
/// publishes pressure-source-1; the passing generating case is an ordinary
/// companion whose record is bound by the companion binding (a failed binding
/// would withhold the join and publish pressure-2).
#[test]
fn a_generating_case_is_an_ordinary_companion_of_a_source_selected_envelope() {
    let mut wire: Value = serde_json::from_str(SOURCE_MIXED).unwrap();
    let model = &mut wire["model"];
    model["pipe_segments"][0]["section"]["material_density"] = qv(7850.0, "kg/m^3");
    // The fixture's ordinary companion solves on its compliant point, where
    // it passes its checks (a Sensitive companion would itself be declined
    // by source recovery and withhold the join); the generating case does
    // the same.
    model["load_cases"][1] = json!({"id": "case:seismic", "modulus_basis_ref": "point:compliant", "pressure_regions": [], "primitive_loads": [],
        "equivalent_static": seismic(), "provenance": PROV});
    for mode in MODES {
        let out = run(&wire, mode);
        assert_eq!(out["status"]["mechanics"], "MECHANICS_SOLVED", "{:?}", blocking(&out));
        assert_eq!(out["producer"]["semantic_contract_id"], SOURCE_SEMANTICS, "{:#?}", out["diagnostics"]);
        let cases = out["contract_evidence"]["exact_cases"].as_array().unwrap();
        assert_eq!(cases[0]["recovery_method"], "retained_source_blocks_exact_v1");
        assert!(cases[0].get("equivalent_static").is_none());
        assert_ne!(cases[1]["recovery_method"], "retained_source_blocks_exact_v1");
        assert_eq!(cases[1]["equivalent_static"], json!({"mass_method": EXACT_MASS_METHOD,
            "generated_load_ids": ["case:seismic:generated:seismic:global-y:element-0"]}));
    }
}

/// W-15 body rows: a valve's DEC-045 scale multiplies its mapped member's
/// stiffness, not its mass. The cantilever's support actions are statically
/// determinate, so a scaled member carries the same generated load as without
/// the valve, while the tip moves less.
#[test]
fn a_rigid_component_scale_multiplies_stiffness_not_mass() {
    let reference = case_ref("W16C-SEISMIC-EX-EB");
    let valve = json!({"id": "component:w16c-valve", "kind": "valve", "node": "node:b", "geometry": {"rigid_pipe_ref": "pipe:a-b"},
        "modifiers": {"stiffness_scaling_user_value": qv(10.0, "1"), "source_reference": "invented W-16c scale"},
        "mechanics_interface": {"solver_consumption": "mechanics_geometry_only"}, "provenance": PROV});
    let mut wire = cantilever(Route::Exact, EULER_BERNOULLI, vec![case(CASE, Some(seismic()), vec![])]);
    wire["model"]["components"] = json!([valve]);
    for mode in MODES {
        let out = solved(&wire, Route::Exact, mode, "scaled valve member");
        checks_passed(&out, "scaled valve member");
        compare(&out, &reference, CASE, "scaled valve member", |r| r["quantity"] == "support_reaction");
        let tip = find(&out, CASE, "global_nodal_displacement_y", "node:b", "nodal_displacement_y", None, None)["value"].as_f64().unwrap() / 1000.0;
        let unscaled = value(&reference["key_values"]["tip_translation_m"]);
        assert!(((tip * 10.0 - unscaled) / unscaled).abs() <= RELATIVE, "the scale divides the flexure: {tip} vs {unscaled}/10");
        assert!(rows(&out).iter().any(|r| r.get("component_body").is_some()), "the mapped member's rows are marked component_body");
    }
}

// ------------------------------------------------------------------ negatives
/// Every PF validation and refusal code for `equivalent_static` applies
/// unchanged on EX (§1).
#[test]
fn every_preview_route_code_applies_unchanged_on_the_exact_route() {
    type Mutation = (&'static str, &'static str, fn(&mut Value));
    let mutations: Vec<Mutation> = vec![
        ("no inputs", "EQUIVALENT_STATIC_INPUT_MISSING", |m| m["load_cases"][0]["equivalent_static"] = json!({"provenance": PROV})),
        ("no g-factor", "EQUIVALENT_STATIC_INPUT_MISSING", |m| { m["load_cases"][0]["equivalent_static"]["seismic"].as_object_mut().unwrap().remove("g_factor_y"); }),
        ("no gravity", "EQUIVALENT_STATIC_INPUT_MISSING", |m| { m["load_cases"][0]["equivalent_static"]["seismic"].as_object_mut().unwrap().remove("gravity_acceleration"); }),
        ("no density", "EQUIVALENT_STATIC_INPUT_MISSING", |m| { m["pipe_segments"][0]["section"].as_object_mut().unwrap().remove("material_density"); }),
        ("insulation pair", "EQUIVALENT_STATIC_INPUT_MISSING", |m| { m["pipe_segments"][0]["section"].as_object_mut().unwrap().remove("insulation_density"); }),
        ("negative contents", "EQUIVALENT_STATIC_INPUT_INVALID", |m| m["pipe_segments"][0]["section"]["contents_density"] = qv(-1.0, "kg/m^3")),
        ("wind inputs", "EQUIVALENT_STATIC_INPUT_MISSING", |m| m["load_cases"][0]["equivalent_static"] = json!({"wind": {"direction": "global_z", "exposed_pipe_refs": ["pipe:a-b"]}, "provenance": PROV})),
        ("wind span fields", "EQUIVALENT_STATIC_INPUT_MISSING", |m| m["load_cases"][0]["equivalent_static"]["wind"]["exposed_spans"] = json!([{"pipe_ref": "pipe:a-b"}])),
        ("wind direction", "EQUIVALENT_STATIC_INPUT_INVALID", |m| m["load_cases"][0]["equivalent_static"]["wind"]["direction"] = json!("rotation_x")),
        ("unknown pipe", "EQUIVALENT_STATIC_INPUT_INVALID", |m| m["load_cases"][0]["equivalent_static"]["wind"]["exposed_pipe_refs"] = json!(["pipe:none"])),
        ("duplicate whole span", "EQUIVALENT_STATIC_INPUT_INVALID", |m| m["load_cases"][0]["equivalent_static"]["wind"]["exposed_pipe_refs"] = json!(["pipe:a-b", "pipe:a-b"])),
        ("double exposure", "EQUIVALENT_STATIC_INPUT_INVALID", |m| m["load_cases"][0]["equivalent_static"]["wind"]["exposed_spans"] = json!([{"pipe_ref": "pipe:a-b", "start_fraction": qv(0.1, "1"), "end_fraction": qv(0.2, "1")}])),
        ("bad fractions", "EQUIVALENT_STATIC_INPUT_INVALID", |m| {
            let w = &mut m["load_cases"][0]["equivalent_static"]["wind"];
            w["exposed_pipe_refs"] = json!([]);
            w["exposed_spans"] = json!([{"pipe_ref": "pipe:a-b", "start_fraction": qv(0.7, "1"), "end_fraction": qv(0.2, "1")}]);
        }),
        ("overlap", "EQUIVALENT_STATIC_INPUT_INVALID", |m| {
            let w = &mut m["load_cases"][0]["equivalent_static"]["wind"];
            w["exposed_pipe_refs"] = json!([]);
            w["exposed_spans"] = json!([{"pipe_ref": "pipe:a-b", "start_fraction": qv(0.1, "1"), "end_fraction": qv(0.5, "1")},
                {"pipe_ref": "pipe:a-b", "start_fraction": qv(0.4, "1"), "end_fraction": qv(0.8, "1")}]);
        }),
    ];
    for (what, code, mutate) in mutations {
        let mut found = Vec::new();
        for route in [Route::Preview, Route::Exact] {
            let generation = json!({"seismic": seismic()["seismic"], "wind": wind(&["pipe:a-b"], vec![])["wind"], "provenance": PROV});
            let mut wire = cantilever(route, EULER_BERNOULLI, vec![case(CASE, Some(generation), vec![])]);
            mutate(&mut wire["model"]);
            let diagnostics = refused(&wire, code, &format!("{what} {route:?}"));
            found.push(diagnostics.iter().map(|d| (d["id"].clone(), d["message"].clone())).collect::<Vec<_>>());
        }
        assert_eq!(found[0], found[1], "{what}: the same diagnostics on both routes");
    }
}

/// The exact cantilever with the load/reference-state group: `case:w16c` in
/// the group with generation, and `case:unlisted` without a state record.
fn load_state_document(listed: bool, unlisted: bool) -> Value {
    let mut cases = vec![];
    let state = json!({"contract": "openpipestress.load_reference_state/1.0.0", "reference_configuration_ref": "reference:installed",
        "element_states": [{"pipe_ref": "pipe:a-b", "material_selection": {"kind": "explicit_base_properties", "material_ref": "material:w16c",
            "applicability_reference": "invented analytical basis declared applicable to this member"},
            "thermal_state": {"kind": "unchanged_reference", "provenance": PROV}}],
        "support_states": [{"support_ref": "support:a", "participation": {"kind": "active_model_device"}}],
        "load_sources": [], "history": {"kind": "independent_equilibrium"}, "provenance": PROV});
    let mut first = case(CASE, listed.then(seismic), vec![]);
    first["analysis_state"] = state;
    cases.push(first);
    if unlisted {
        cases.push(case("case:unlisted", Some(seismic()), vec![]));
    }
    let mut wire = cantilever(Route::Exact, EULER_BERNOULLI, cases);
    wire["model"]["reference_configurations"] = json!([{"id": "reference:installed", "label": "Installed reference", "geometry_ref": {"kind": "authored_model_geometry"},
        "member_references": [{"pipe_ref": "pipe:a-b", "basis": {"kind": "direct_strain_reference"}, "fit": {"kind": "none"}, "provenance": PROV}], "provenance": PROV}]);
    wire
}

/// §3: a load/reference-state document refuses every authored case's
/// generation by name, a case without a state record included, and so does a
/// document whose group is an authored null.
#[test]
fn load_state_documents_refuse_generation_for_every_case() {
    let found = refused(&load_state_document(true, false), "EXACT_EQUIVALENT_STATIC_LOAD_STATE_UNSUPPORTED", "listed case");
    assert_eq!(found.len(), 1);
    assert_eq!(found[0]["affected_refs"], json!([CASE, "equivalent_static"]));
    let found = refused(&load_state_document(false, true), "EXACT_EQUIVALENT_STATIC_LOAD_STATE_UNSUPPORTED", "unlisted case");
    assert_eq!(found[0]["affected_refs"], json!(["case:unlisted", "equivalent_static"]));
    let found = refused(&load_state_document(true, true), "EXACT_EQUIVALENT_STATIC_LOAD_STATE_UNSUPPORTED", "both");
    assert_eq!(found.len(), 2);
    let mut null_group = cantilever(Route::Exact, EULER_BERNOULLI, vec![case(CASE, Some(seismic()), vec![])]);
    null_group["model"]["reference_configurations"] = Value::Null;
    let found = refused(&null_group, "EXACT_EQUIVALENT_STATIC_LOAD_STATE_UNSUPPORTED", "authored null group");
    assert_eq!(found.len(), 1);
}

/// §3: the retained sub-route keeps declining generation; the ordinary
/// result is returned unchanged.
#[test]
fn the_retained_sub_route_declines_generation() {
    let wire = reference_request(&case_ref("W16C-SEISMIC-EX-EB")).1;
    for mode in MODES {
        let ordinary = run_linear_static_preview_value_with_mode(wire.clone(), mode).unwrap();
        let retained = run_linear_static_preview_value_with_retained_direct(wire.clone(), mode, RETAINED_LIMIT).unwrap();
        assert!(retained.successor().is_none());
        assert_eq!(serde_json::to_vec(retained.envelope()).unwrap(), serde_json::to_vec(&ordinary).unwrap());
        assert_eq!(ordinary.status.mechanics, "MECHANICS_SOLVED");
    }
}

/// Byte identity: a case without `equivalent_static` publishes no record.
#[test]
fn a_case_without_generation_publishes_no_record() {
    let wire = cantilever(Route::Exact, EULER_BERNOULLI, vec![
        case(CASE, Some(seismic()), vec![]),
        case("case:plain", None, vec![distributed("load:plain", "pipe:a-b", "global_y", 10.0)])]);
    for mode in MODES {
        let out = solved(&wire, Route::Exact, mode, "two cases");
        assert!(exact_case(&out, CASE).get("equivalent_static").is_some());
        assert!(exact_case(&out, "case:plain").get("equivalent_static").is_none());
    }
}

/// §5's combination of a seismic case with a pressure case runs through slice
/// b's exact combinations, which join with slice b's code.
#[test]
#[ignore = "W-16 slice b (exact combinations) is not in this change; the combination joins with slice b's code"]
fn a_seismic_case_combines_with_a_pressure_case() {}

/// §2: the readers bind the record: the pressure-2 reader and the
/// pressure-source-1 transport reader accept the producer's envelopes, and
/// refuse a record with an extra key, another mass method, no id, a repeated
/// id or an id outside the case's generated namespace, and a record on a
/// selected retained-source case.
#[test]
fn the_readers_bind_the_generation_record() {
    use open_pipe_stress_result_export::{physics_source, semantic_contract};
    let wire = reference_request(&case_ref("W16C-SEISMIC-EX-EB")).1;
    let out = solved(&wire, Route::Exact, PreviewSolverMode::SparseInteractive, "reader base");
    semantic_contract::validate_pressure_evidence(&out).expect("the pressure-2 reader admits the producer's record");
    let id = format!("{CASE}:generated:seismic:global-y:element-0");
    let tamper = |record: Value| {
        let mut forged = out.clone();
        forged["contract_evidence"]["exact_cases"][0]["equivalent_static"] = record;
        forged
    };
    for (what, record) in [
        ("extra key", json!({"mass_method": EXACT_MASS_METHOD, "generated_load_ids": [id], "extra": 1})),
        ("preview mass method", json!({"mass_method": "source_od_effective_wall_areas/v1", "generated_load_ids": [id]})),
        ("no id", json!({"mass_method": EXACT_MASS_METHOD, "generated_load_ids": []})),
        ("repeated id", json!({"mass_method": EXACT_MASS_METHOD, "generated_load_ids": [id, id]})),
        ("foreign id", json!({"mass_method": EXACT_MASS_METHOD, "generated_load_ids": ["case:other:generated:seismic:global-y:element-0"]})),
        ("authored id", json!({"mass_method": EXACT_MASS_METHOD, "generated_load_ids": ["load:authored"]})),
    ] {
        let error = semantic_contract::validate_pressure_evidence(&tamper(record)).expect_err(what);
        assert!(error.contains("EQUIVALENT_STATIC"), "{what}: {error}");
    }
    // pressure-source-1: the companion's record, under the closed source
    // schema and the transport reader.
    let mut source: Value = serde_json::from_str(SOURCE_MIXED).unwrap();
    source["model"]["pipe_segments"][0]["section"]["material_density"] = qv(7850.0, "kg/m^3");
    source["model"]["load_cases"][1] = json!({"id": "case:seismic", "modulus_basis_ref": "point:compliant", "pressure_regions": [], "primitive_loads": [],
        "equivalent_static": seismic(), "provenance": PROV});
    let envelope = run(&source, PreviewSolverMode::SparseInteractive);
    assert_eq!(envelope["producer"]["semantic_contract_id"], SOURCE_SEMANTICS);
    physics_source::validate(&envelope, None).expect("the source reader admits the companion's record");
    let mut forged = envelope.clone();
    forged["contract_evidence"]["exact_cases"][1]["equivalent_static"]["extra"] = json!(1);
    assert!(physics_source::validate(&forged, None).is_err(), "the closed source shape refuses an extra key");
    let mut forged = envelope.clone();
    forged["contract_evidence"]["exact_cases"][0]["equivalent_static"] = forged["contract_evidence"]["exact_cases"][1]["equivalent_static"].clone();
    assert!(physics_source::validate(&forged, None).is_err(), "a selected case never carries a record");
}
