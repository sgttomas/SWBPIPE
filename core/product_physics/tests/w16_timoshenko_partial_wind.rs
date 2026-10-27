//! W-16t (DEL-05-01 Design `exact-equivalent-static.md` §3 and §5; DEL-05-03
//! Design `section-bases-and-timoshenko.md` §3.4 and §4 item 4): wind marked on
//! a proper fraction of a straight span under `timoshenko_annulus_energy_v1`
//! on the exact pressure route (EX).
//!
//! The oracle is read from the frozen references in
//! `validation/references/w16_timoshenko_partial_wind/references.json`
//! (exact rational continuum, element and closure routes; see that package's
//! README), pinned to the hash recorded in its SHA256SUMS. Nothing here
//! derives an expectation from product code. Model inputs are invented.
//!
//! Coverage (W-16t design §5.1): partial spans at either end and in the
//! interior, a high-phi short member, the symmetric, near-full and exact
//! full-span controls, two extents on one pipe, a subdivided cantilever, a
//! fixed-fixed run and a skew member, their Euler-Bernoulli twins and a
//! mechanics combination, in both solver modes; every positive case passes its
//! numerical checks and the K-D5 M3 judging and coverage rules reconstructed
//! here (C1 group breaches, C2 circular-maximum coverage, C3 displacement
//! groups, C6 missing publications, C8 vacuous passes, C9 conflicting
//! identities); negatives N1-N7.
//!
//! Criterion (DEC-024/DEC-026, unchanged): |obs - exp| <= 1e-9 |exp|; an
//! exactly zero expectation uses 1e-9 times the case's frozen zero scale of its
//! family.
//!
//! Mutants (W-16t design §5.3), each run against this file and the unit tests:
//! the stale Euler-Bernoulli binary64 vector fails the CANT-INTERIOR,
//! -PROXIMAL, -DISTAL and -SHORT tips and the FIXED-FIXED reactions; the stale
//! K-D5 ball leaves Passed (`checks_passed`); a retained refusal fails every
//! positive case.
use open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator;
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, run_linear_static_preview_value_with_retained_direct, PreviewSolverMode,
    RetainedByteLimit,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

#[path = "common/current_model.rs"]
mod current_model;

const REFERENCES: &str = include_str!("../../../validation/references/w16_timoshenko_partial_wind/references.json");
const SUMS: &str = include_str!("../../../validation/references/w16_timoshenko_partial_wind/SHA256SUMS");
const W16C: &str = include_str!("../../../validation/references/w16_exact_equivalent_static/references.json");
const SOURCE_MIXED: &str = include_str!("../../../fixtures/product_preview/pressure_source/physics_source/mixed.request.json");
const ARC_SKETCHES: &str = include_str!("../../../validation/references/t4_i7/u2_document_sketches.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const RELATIVE: f64 = 1.0e-9;
const PROV: &str = "w16t invented benchmark input; not library, component, material-catalog or code data";
const CASE: &str = "case:w16t";
const SEISMIC_CASE: &str = "case:w16t-seismic";
const COMBINATION: &str = "combination:w16t";
const TIMOSHENKO: &str = "timoshenko_annulus_energy_v1";
const EULER_BERNOULLI: &str = "euler_bernoulli";
const RETIRED: &str = "EXACT_EQUIVALENT_STATIC_PARTIAL_SPAN_TIMOSHENKO_UNSUPPORTED";
const RETAINED_LIMIT: RetainedByteLimit = RetainedByteLimit::new(12_884_901_888);
const BASES: [&str; 2] = ["nominal_section", "reduced_section"];

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

#[test]
fn references_are_the_frozen_bytes() {
    let digest: String = Sha256::digest(REFERENCES.as_bytes()).iter().map(|b| format!("{b:02x}")).collect();
    let line = SUMS.lines().find(|l| l.ends_with("  references.json")).expect("SHA256SUMS lists references.json");
    assert_eq!(&line[..64], digest, "the references must equal the frozen bytes");
    let doc = refs();
    assert!(doc["crosscheck"].as_array().unwrap().iter().all(|c| c["agrees"] == true));
    assert!(doc["element_controls"].as_array().unwrap().iter().all(|c| c["agrees"] == true));
    let ids: Vec<&str> = doc["element_controls"].as_array().unwrap().iter().map(|c| c["id"].as_str().unwrap()).collect();
    for e in ["E1", "E2", "E3", "E4", "E5", "E6", "E7", "E8", "E9"] {
        assert!(ids.contains(&e), "element control {e}");
    }
    assert_eq!(doc["cases"].as_array().unwrap().len(), 17);
}

// ------------------------------------------------------------------ authoring
fn qv(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}
fn section() -> Value {
    json!({
        "outside_diameter": qv(input("/section/outside_diameter_m"), "m"),
        "wall_thickness": qv(input("/section/wall_thickness_m"), "m"),
        "mill_tolerance": qv(input("/section/mill_tolerance_m"), "m"),
        "corrosion_allowance": qv(input("/section/corrosion_allowance_m"), "m"),
        "material_density": qv(input("/section/material_density_kg_per_m3"), "kg/m^3"),
        "contents_density": qv(input("/section/contents_density_kg_per_m3"), "kg/m^3"),
        "insulation_thickness": qv(input("/section/insulation_thickness_m"), "m"),
        "insulation_density": qv(input("/section/insulation_density_kg_per_m3"), "kg/m^3")})
}
fn node(id: &str, at: [f64; 3]) -> Value {
    json!({"id": id, "position": {"x": at[0], "y": at[1], "z": at[2]}, "provenance": PROV})
}
fn pipe(id: &str, from: &str, to: &str, y: [f64; 3]) -> Value {
    json!({"id": id, "from": from, "to": to, "section": section(), "material": "material:w16t",
        "y_reference": {"x": y[0], "y": y[1], "z": y[2]}, "provenance": PROV})
}
fn anchor(id: &str, node: &str) -> Value {
    json!({"id": id, "node": node, "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV})
}
/// An EX document over `nodes`, `pipes` and `supports`.
fn document(shear_model: &str, nodes: Vec<Value>, pipes: Vec<Value>, supports: Vec<Value>, cases: Vec<Value>) -> Value {
    let material = json!({"id": "material:w16t", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
        "elastic_modulus": qv(input("/material/elastic_modulus_Pa"), "Pa"), "poisson_ratio": qv(input("/material/poisson_ratio"), "1"), "provenance": PROV});
    let mut cases = cases;
    for case in &mut cases {
        case.as_object_mut().unwrap().entry("pressure_regions").or_insert(json!([]));
    }
    json!({"model": {"schema_version": "0.7.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:w16t", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": nodes, "pipe_segments": pipes, "supports": supports,
        "components": [], "materials": [material], "load_cases": cases, "combinations": [],
        "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": shear_model}}, "materials": []})
}
fn wind(refs: &[&str], spans: &[(String, f64, f64)]) -> Value {
    json!({"wind": {"pressure": qv(input("/wind/pressure_Pa"), "Pa"), "shape_factor": qv(input("/wind/shape_factor"), "1"),
        "direction": "global_z", "exposed_pipe_refs": refs,
        "exposed_spans": spans.iter().map(|(p, a, b)| json!({"pipe_ref": p, "start_fraction": qv(*a, "1"), "end_fraction": qv(*b, "1")})).collect::<Vec<_>>()},
        "provenance": PROV})
}
fn seismic() -> Value {
    json!({"seismic": {"gravity_acceleration": qv(input("/seismic/gravity_acceleration_m_per_s2"), "m/s^2"),
        "g_factor_y": qv(input("/seismic/g_factor_y"), "1")}, "provenance": PROV})
}
fn case(id: &str, generation: Option<Value>) -> Value {
    let mut case = json!({"id": id, "primitive_loads": [], "provenance": PROV});
    if let Some(generation) = generation {
        case["equivalent_static"] = generation;
    }
    case
}
fn marks(request: &Value) -> Vec<(String, f64, f64)> {
    let mut out = Vec::new();
    for (pipe, extents) in request["marks"].as_object().unwrap() {
        for extent in extents.as_array().unwrap() {
            out.push((pipe.clone(), rational(&extent[0]), rational(&extent[1])));
        }
    }
    out
}
/// The geometry of one reference request: nodes, pipes and supports.
fn geometry(request: &Value) -> (Vec<Value>, Vec<Value>, Vec<Value>) {
    let x = [0.0, 1.0, 0.0];
    match request["model"].as_str().unwrap() {
        "cantilever" => {
            let l = rational(&request["length_m"]);
            (vec![node("node:a", [0.0; 3]), node("node:b", [l, 0.0, 0.0])], vec![pipe("pipe:a-b", "node:a", "node:b", x)],
                vec![anchor("support:a", "node:a")])
        }
        "subdivided_cantilever" => {
            let (m, b) = (rational(&request["nodes"]["node:m"]), rational(&request["nodes"]["node:b"]));
            (vec![node("node:a", [0.0; 3]), node("node:m", [m, 0.0, 0.0]), node("node:b", [b, 0.0, 0.0])],
                vec![pipe("pipe:a-m", "node:a", "node:m", x), pipe("pipe:m-b", "node:m", "node:b", x)], vec![anchor("support:a", "node:a")])
        }
        "fixed_fixed" => {
            let (m, c) = (rational(&request["nodes"]["node:m"]), rational(&request["nodes"]["node:c"]));
            (vec![node("node:a", [0.0; 3]), node("node:m", [m, 0.0, 0.0]), node("node:c", [c, 0.0, 0.0])],
                vec![pipe("pipe:a-m", "node:a", "node:m", x), pipe("pipe:m-c", "node:m", "node:c", x)],
                vec![anchor("support:a", "node:a"), anchor("support:c", "node:c")])
        }
        "skew_cantilever" => {
            let b: Vec<f64> = request["node_b"].as_array().unwrap().iter().map(rational).collect();
            let y: Vec<f64> = request["y_reference"].as_array().unwrap().iter().map(rational).collect();
            (vec![node("node:a", [0.0; 3]), node("node:b", [b[0], b[1], b[2]])], vec![pipe("pipe:a-b", "node:a", "node:b", [y[0], y[1], y[2]])],
                vec![anchor("support:a", "node:a")])
        }
        other => panic!("model {other}"),
    }
}
/// The request of one primitive reference case, wind through `exposed_spans`.
fn reference_request(reference: &Value) -> Value {
    let request = &reference["request"];
    let (nodes, pipes, supports) = geometry(request);
    document(reference["shear_model"].as_str().unwrap(), nodes, pipes, supports, vec![case(CASE, Some(wind(&[], &marks(request))))])
}
/// W16T-COMBINED: the interior wind case and a seismic case on the cantilever,
/// combined as 1 * wind + 1/2 * seismic.
fn combination_request(reference: &Value) -> Value {
    let request = &reference["request"];
    let (nodes, pipes, supports) = geometry(request);
    let wind_marks = marks(&request["cases"][CASE]["wind_marks"].as_object().map(|m| json!({"marks": m})).unwrap());
    let mut wire = document(TIMOSHENKO, nodes, pipes, supports,
        vec![case(CASE, Some(wind(&[], &wind_marks))), case(SEISMIC_CASE, Some(seismic()))]);
    let terms: Vec<Value> = request["combination"]["terms"].as_array().unwrap().iter()
        .map(|t| json!({"load_case": t[0], "factor": rational(&t[1])})).collect();
    wire["model"]["combinations"] = json!([{"id": COMBINATION, "basis": "mechanics", "terms": terms, "provenance": PROV}]);
    wire
}

// ------------------------------------------------------------------ running
fn run(wire: &Value, mode: PreviewSolverMode) -> Value {
    let out = run_linear_static_preview_value_with_mode(wire.clone(), mode).unwrap_or_else(|e| panic!("typed boundary rejected the request: {e}"));
    serde_json::to_value(out).unwrap()
}
fn codes(out: &Value) -> Vec<String> {
    out["diagnostics"].as_array().unwrap().iter().map(|d| d["code"].as_str().unwrap().to_string()).collect()
}
fn blocking(out: &Value) -> Vec<String> {
    out["diagnostics"].as_array().unwrap().iter()
        .filter(|d| d["severity"] == "blocking" || d["severity"] == "failure")
        .map(|d| format!("{}: {}", d["code"].as_str().unwrap(), d["message"].as_str().unwrap()))
        .collect()
}
/// N7: no output of this file ever carries the retired code.
fn never_retired(out: &Value, what: &str) {
    assert!(!serde_json::to_string(out).unwrap().contains(RETIRED), "{what}: the retired code {RETIRED} appears");
}
fn solved(wire: &Value, mode: PreviewSolverMode, what: &str) -> Value {
    let out = run(wire, mode);
    never_retired(&out, what);
    assert_eq!(out["status"]["mechanics"], "MECHANICS_SOLVED", "{what}: not solved; blocking {:?}", blocking(&out));
    assert!(blocking(&out).is_empty(), "{what}: blocking {:?}", blocking(&out));
    out
}
fn checks_passed(out: &Value, what: &str) {
    assert_eq!(out["numerical_quality"]["status"], "checks_passed", "{what}: {:?}", codes(out));
    for case in out["numerical_quality"]["cases"].as_array().unwrap() {
        assert_eq!(case["solve_quality"], "checks_passed", "{what}: {case}");
    }
}
fn refused(wire: &Value, code: &str, what: &str) -> Vec<Value> {
    let mut found = Vec::new();
    for mode in MODES {
        let out = run(wire, mode);
        never_retired(&out, what);
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
fn exact_case<'a>(out: &'a Value, case_id: &str) -> &'a Value {
    out["contract_evidence"]["exact_cases"].as_array().unwrap().iter().find(|c| c["load_case_id"] == case_id)
        .unwrap_or_else(|| panic!("no exact case {case_id}"))
}

// ------------------------------------------------------------------ comparison
/// The one row of (basis, kind, entity, component, location, section basis);
/// a missing or repeated identity fails (C6, C9).
fn find<'a>(out: &'a Value, basis_ref: (&str, &str), kind: &str, entity: &str, component: Option<&str>, location: Option<&str>, section: Option<&str>) -> &'a Value {
    let found: Vec<&Value> = rows(out).iter().filter(|row| {
        row["kind"] == kind && row["entity_ref"] == entity
            && row["basis_ref"]["ref_type"] == basis_ref.0 && row["basis_ref"]["ref_id"] == basis_ref.1
            && component.is_none_or(|c| row["metadata"]["component"] == c)
            && location.is_none_or(|l| row["metadata"]["location"] == l)
            && section.is_none_or(|b| row["metadata"]["basis"].as_str().is_some_and(|text| text.ends_with(&format!("section_basis={b}"))))
    }).collect();
    assert_eq!(found.len(), 1, "one row {kind} {entity} {component:?} {location:?} {section:?} on {basis_ref:?}; candidates {:#?}",
        rows(out).iter().filter(|row| row["kind"] == kind && row["entity_ref"] == entity).map(|row| &row["metadata"]).collect::<Vec<_>>());
    found[0]
}
fn station(r: &Value) -> (&'static str, f64) {
    match r["station"].as_str().unwrap() {
        "0" => ("end_i", -1.0),
        "1/4" => ("quarter_1", 1.0),
        "1/2" => ("midspan", 1.0),
        "3/4" => ("quarter_3", 1.0),
        "1" => ("end_j", 1.0),
        s => panic!("station {s}"),
    }
}
/// One published value in the reference's SI unit (displacements are
/// published in mm, station stresses in MPa).
fn observed(out: &Value, basis_ref: (&str, &str), r: &Value) -> f64 {
    let (entity, component, unit) = (r["entity"].as_str().unwrap(), r["component"].as_str().unwrap(), r["unit"].as_str().unwrap());
    match r["quantity"].as_str().unwrap() {
        "support_reaction" => {
            let row = find(out, basis_ref, "support_reaction_component_v2", entity, Some(component), None, None);
            assert_eq!(row["unit"], unit);
            row["value"].as_f64().unwrap()
        }
        "support_reaction_magnitude" => {
            let kind = if component == "force_magnitude" { "support_reaction_force_magnitude_v2" } else { "support_reaction_moment_magnitude_v2" };
            let row = find(out, basis_ref, kind, entity, None, None, None);
            assert_eq!(row["unit"], unit);
            row["value"].as_f64().unwrap()
        }
        "nodal_motion" => {
            let axis = &component[1..];
            if component.starts_with('u') {
                let row = find(out, basis_ref, &format!("global_nodal_displacement_{axis}"), entity, Some(&format!("nodal_displacement_{axis}")), None, None);
                assert_eq!(row["unit"], "mm");
                row["value"].as_f64().unwrap() / 1000.0
            } else {
                let row = find(out, basis_ref, &format!("global_nodal_rotation_{axis}"), entity, Some(&format!("nodal_rotation_{axis}")), None, None);
                assert_eq!(row["unit"], unit);
                row["value"].as_f64().unwrap()
            }
        }
        "section_action" => {
            let (location, sign) = station(r);
            let row = find(out, basis_ref, &format!("element_local_{component}"), entity, Some(component), Some(location), None);
            assert_eq!(row["unit"], unit);
            sign * row["value"].as_f64().unwrap()
        }
        "section_stress" => {
            let (location, _) = station(r);
            let row = find(out, basis_ref, &format!("element_local_{component}"), entity, Some(component), Some(location), r["section_basis"].as_str());
            assert_eq!(row["unit"], "MPa");
            row["value"].as_f64().unwrap() * 1.0e6
        }
        "elastic_maximum" => {
            let row = find(out, basis_ref, "pipe_elastic_normal_stress_maximum_v2", entity, Some(component), None, r["section_basis"].as_str());
            assert_eq!(row["unit"], "Pa");
            row["value"].as_f64().unwrap()
        }
        q => panic!("quantity {q}"),
    }
}
fn family(unit: &str) -> &'static str {
    match unit {
        "N" => "force",
        "N*m" => "moment",
        "m" => "translation",
        "rad" => "rotation",
        "Pa" => "stress",
        other => panic!("unit {other}"),
    }
}
fn label(r: &Value) -> String {
    format!("{} {} {} {} {}", r["quantity"].as_str().unwrap(), r["entity"].as_str().unwrap(), r["station"].as_str().unwrap_or(""),
        r["component"].as_str().unwrap(), r["section_basis"].as_str().unwrap_or(""))
}
/// Every frozen row of `reference` against `out` under `basis_ref`. Returns
/// the row count, the largest relative error (nonzero rows) and the largest
/// |obs|/zero_scale (zero rows), and the (row, observed) pairs.
fn compare(out: &Value, reference: &Value, basis_ref: (&str, &str), what: &str) -> (usize, f64, f64, Vec<(Value, f64)>) {
    let (mut count, mut rel, mut zero, mut pairs) = (0, 0.0f64, 0.0f64, Vec::new());
    for r in reference["rows"].as_array().unwrap() {
        let zs = value(&reference["zero_scales"][family(r["unit"].as_str().unwrap())]);
        let (o, e) = (observed(out, basis_ref, r), value(r));
        let tol = RELATIVE * if e == 0.0 { zs.abs() } else { e.abs() };
        assert!(o.is_finite() && (o - e).abs() <= tol, "{what} {}: observed {o:.17e} expected {e:.17e} tolerance {tol:.3e}", label(r));
        if e == 0.0 { zero = zero.max((o / zs).abs()) } else { rel = rel.max(((o - e) / e).abs()) }
        count += 1;
        pairs.push((r.clone(), o));
    }
    assert!(count > 0, "{what}: a vacuous comparison");
    (count, rel, zero, pairs)
}

// ------------------------------------------------------------------ K-D5 M3 judging and coverage
/// RD(1e-9): the largest binary64 not above 1e-9 (1e-9 itself rounds up).
fn crit() -> f64 {
    1.0e-9_f64.next_down()
}
const FLOOR: f64 = 5.820766091346741e-11; // 2^-34
const GUARD: f64 = 1.5e-297; // above 2^-988
/// C1/C3/C8 on one Passed case: the published groups against the reference.
/// Groups: per (pipe, station) |N|, (V_y, V_z), |T|, (M_y, M_z); per support
/// the force and moment vectors and the two magnitudes; per node the
/// translation and rotation vectors; each stress row and maximum alone.
/// A force or moment group is judged when max(|v|, |t|) >= 2^-34 times its
/// coupled scale (SF = max(force, moment/L_b), SM = max(moment, L_b force));
/// a judged group breaches when sum (v - t)^2 > RD(1e-9)^2 sum v^2. A
/// displacement or rotation group must lie within RD(1e-9) of its body's
/// coupled scale S*_t = max(S_t, L_b S_r) or S*_r = max(S_r, S_t / L_b).
fn m3_judge(pairs: &[(Value, f64)], body_diagonal: f64, what: &str) -> usize {
    use std::collections::BTreeMap;
    let mut groups: BTreeMap<(String, String), (Vec<f64>, Vec<f64>, &'static str)> = BTreeMap::new();
    for (r, o) in pairs {
        let e = value(r);
        let q = r["quantity"].as_str().unwrap();
        let entity = r["entity"].as_str().unwrap().to_string();
        let comp = r["component"].as_str().unwrap();
        let (sub, fam) = match q {
            "section_action" => {
                let st = r["station"].as_str().unwrap();
                let g = match comp {
                    "axial_force" => "N",
                    "shear_force_y" | "shear_force_z" => "V",
                    "torsional_moment" => "T",
                    _ => "M",
                };
                (format!("{st}:{g}"), if g == "N" || g == "V" { "force" } else { "moment" })
            }
            "support_reaction" => (if comp.starts_with('F') { "F".into() } else { "M".into() }, if comp.starts_with('F') { "force" } else { "moment" }),
            "support_reaction_magnitude" => (comp.to_string(), if comp == "force_magnitude" { "force" } else { "moment" }),
            "nodal_motion" => (if comp.starts_with('u') { "u".into() } else { "r".into() }, if comp.starts_with('u') { "translation" } else { "rotation" }),
            "section_stress" | "elastic_maximum" => (label(r), "stress"),
            other => panic!("{other}"),
        };
        let entry = groups.entry((format!("{q}:{entity}"), sub)).or_insert((Vec::new(), Vec::new(), fam));
        entry.0.push(*o);
        entry.1.push(e);
    }
    let norm = |x: &[f64]| x.iter().map(|v| v * v).sum::<f64>().sqrt();
    let top = |fam: &str| groups.values().filter(|g| g.2 == fam).map(|g| norm(&g.0)).fold(0.0, f64::max);
    let (sf, sm) = (top("force"), top("moment"));
    let (st, sr) = (top("translation"), top("rotation"));
    let scale_f = sf.max(sm / body_diagonal);
    let scale_m = sm.max(body_diagonal * sf);
    let (star_t, star_r) = (st.max(body_diagonal * sr), sr.max(st / body_diagonal));
    let mut judged = 0;
    for ((entity, sub), (v, t, fam)) in &groups {
        let d: Vec<f64> = v.iter().zip(t).map(|(a, b)| a - b).collect();
        match *fam {
            "translation" | "rotation" => {
                let star = if *fam == "translation" { star_t } else { star_r };
                assert!(norm(&d) <= crit() * star, "{what}: C3 {entity} {sub}: {} beyond {}", norm(&d), crit() * star);
            }
            _ => {
                let scale = match *fam {
                    "force" => scale_f,
                    "moment" => scale_m,
                    _ => groups.values().filter(|g| g.2 == "stress").map(|g| norm(&g.0)).fold(0.0, f64::max),
                };
                if scale >= GUARD && norm(v).max(norm(t)) >= FLOOR * scale {
                    judged += 1;
                    let e2: f64 = d.iter().map(|x| x * x).sum();
                    let q2: f64 = v.iter().map(|x| x * x).sum();
                    assert!(e2 <= crit() * crit() * q2, "{what}: C1 breach {entity} {sub}: |e| {} vs {}", e2.sqrt(), crit() * q2.sqrt());
                }
            }
        }
    }
    assert!(judged > 0, "{what}: C8, nothing judged");
    judged
}

/// Exact sign of (u - scale |a|)^2 - (scale y)^2 - (scale z)^2 >= 0 with
/// u - scale |a| >= 0, for binary64 u, a, y, z and scale (exact products).
fn covers(u: f64, a: f64, y: f64, z: f64, scale: f64) -> bool {
    fn parts(mut acc: ExactAccumulator) -> Vec<f64> {
        let mut out = Vec::new();
        while !acc.is_zero() {
            let head = acc.round().unwrap();
            acc.add(-head).unwrap();
            out.push(head);
            assert!(out.len() < 16);
        }
        out
    }
    let product = |s: f64| {
        let mut acc = ExactAccumulator::new();
        acc.add_product(scale, s).unwrap();
        parts(acc)
    };
    let mut d = ExactAccumulator::new();
    d.add(u).unwrap();
    d.add_product(-scale, a.abs()).unwrap();
    if d.signum() < 0 {
        return false;
    }
    let d = parts(d);
    let mut total = ExactAccumulator::new();
    for &x in &d {
        for &w in &d {
            total.add_product(x, w).unwrap();
        }
    }
    for s in [y, z] {
        let p = product(s);
        for &x in &p {
            for &w in &p {
                total.add_product(-x, w).unwrap();
            }
        }
    }
    total.signum() >= 0
}

/// C2 and the maximum evidence: each pipe and basis has complete coverage;
/// every published station sample satisfies (U - |sigma_axial|)^2 >=
/// sigma_y^2 + sigma_z^2 exactly, U the evidence upper bound; the reference
/// maximum lies within the evidence enclosure widened by RD(1e-9) of itself.
fn maxima_covered(out: &Value, reference: &Value, what: &str) {
    let case = exact_case(out, CASE);
    for coverage in case["stress_maximum_coverage"].as_array().unwrap() {
        assert_eq!(coverage["complete"], true, "{what}: {coverage}");
    }
    for r in reference["rows"].as_array().unwrap().iter().filter(|r| r["quantity"] == "elastic_maximum") {
        let (pipe, basis) = (r["entity"].as_str().unwrap(), r["section_basis"].as_str().unwrap());
        let evidence = case["pipe_stress_extrema"].as_array().unwrap().iter()
            .find(|e| e["pipe_id"] == pipe && e["section_basis"] == basis)
            .unwrap_or_else(|| panic!("{what}: no maximum evidence for {pipe} {basis}"));
        let (lower, upper) = (evidence["value_lower_pa"].as_f64().unwrap(), evidence["value_upper_pa"].as_f64().unwrap());
        let exact = value(r);
        assert!(lower - crit() * exact <= exact && exact <= upper + crit() * exact,
            "{what}: {pipe} {basis}: the enclosure [{lower}, {upper}] misses the exact maximum {exact}");
        for location in ["end_i", "quarter_1", "midspan", "quarter_3", "end_j"] {
            let sample = |kind: &str, component: &str| -> f64 {
                rows(out).iter().find(|row| row["kind"] == kind && row["entity_ref"] == pipe && row["basis_ref"]["ref_id"] == CASE
                    && row["metadata"]["component"] == component && row["metadata"]["location"] == location
                    && row["metadata"]["basis"].as_str().is_some_and(|b| b.ends_with(&format!("section_basis={basis}"))))
                    .map_or(0.0, |row| row["value"].as_f64().unwrap())
            };
            let a = sample("element_local_axial_normal_stress", "axial_normal_stress");
            let y = sample("element_local_bending_normal_stress_y", "bending_normal_stress_y");
            let z = sample("element_local_bending_normal_stress_z", "bending_normal_stress_z");
            assert!(covers(upper, a, y, z, 1.0e6), "{what}: C2 {pipe} {basis} {location}: U {upper} does not cover ({a}, {y}, {z}) MPa");
        }
    }
}

fn body_diagonal(reference: &Value) -> f64 {
    let request = &reference["request"];
    match request["model"].as_str().unwrap() {
        "cantilever" => rational(&request["length_m"]),
        "subdivided_cantilever" => rational(&request["nodes"]["node:b"]),
        "fixed_fixed" => rational(&request["nodes"]["node:c"]),
        "skew_cantilever" => rational(&request["length_m"]),
        other => panic!("{other}"),
    }
}

// ------------------------------------------------------------------ the oracle
/// Every primitive reference case in both modes: solved, checks passed, every
/// frozen row within the criterion, the M3 judging and coverage rules, and the
/// generation record. Prints the worst errors.
#[test]
fn every_partial_span_case_matches_its_reference() {
    let mut worst = (0.0f64, String::new());
    for reference in refs()["cases"].as_array().unwrap().iter().filter(|c| c["id"] != "W16T-COMBINED") {
        let id = reference["id"].as_str().unwrap();
        let wire = reference_request(reference);
        for mode in MODES {
            let what = format!("{id} {}", mode.as_str());
            let out = solved(&wire, mode, &what);
            checks_passed(&out, &what);
            let (n, rel, zero, pairs) = compare(&out, reference, ("load_case", CASE), &what);
            let judged = m3_judge(&pairs, body_diagonal(reference), &what);
            maxima_covered(&out, reference, &what);
            println!("W16T {what}: {n} rows; {judged} judged groups; max relative error {rel:.3e}; max |zero row|/scale {zero:.3e}");
            if rel > worst.0 {
                worst = (rel, what.clone());
            }
            let ids = exact_case(&out, CASE)["equivalent_static"]["generated_load_ids"].as_array().unwrap().len();
            assert_eq!(ids, marks(&reference["request"]).len(), "{what}: one generated load per extent");
        }
    }
    println!("W16T worst relative error {:.3e} ({})", worst.0, worst.1);
}

/// The near-full proper span keeps its exact fraction in the generated id
/// (the closure's fraction binding starts from it); exactly [0, 1] through
/// `exposed_spans` publishes the same bits as `exposed_pipe_refs`.
#[test]
fn near_full_span_binds_its_fraction_and_the_full_span_is_unchanged() {
    let near = input("/near_full_end_fraction");
    assert!(near < 1.0 && near > 0.999);
    let wire = reference_request(&case_ref("W16T-CANT-NEARFULL"));
    let full_spans = reference_request(&case_ref("W16T-CANT-FULL-SPANS"));
    let mut full_refs = full_spans.clone();
    full_refs["model"]["load_cases"][0]["equivalent_static"] = wind(&["pipe:a-b"], &[]);
    for mode in MODES {
        let out = solved(&wire, mode, "near-full");
        assert_eq!(exact_case(&out, CASE)["equivalent_static"]["generated_load_ids"],
            json!([format!("{CASE}:generated:wind:global-z:element-0:extent-0-{near}")]));
        let a = solved(&full_spans, mode, "[0, 1] spans");
        let b = solved(&full_refs, mode, "[0, 1] refs");
        let values = |out: &Value| rows(out).iter().filter(|r| r["kind"] != "linear_solver_mode_basis")
            .map(|r| (r["kind"].as_str().unwrap().to_string(), r["entity_ref"].as_str().unwrap().to_string(),
                r["metadata"].to_string(), r["value"].as_f64().map(f64::to_bits))).collect::<Vec<_>>();
        assert_eq!(values(&a), values(&b), "exactly [0, 1] keeps the full-span bits");
    }
}

/// Cross-references: the full-span case equals W16C's frozen full-span
/// Timoshenko rows, the interior Euler-Bernoulli twin W16C's partial EB rows,
/// and the subdivided cantilever the interior case at the support and tip.
#[test]
fn the_references_agree_with_w16c_and_with_each_other() {
    let w16c: Value = serde_json::from_str(W16C).unwrap();
    let w16c_case = |id: &str| w16c["cases"].as_array().unwrap().iter().find(|c| c["id"] == id).unwrap().clone();
    let key = |r: &Value| (r["quantity"].clone(), r["entity"].clone(), r["station"].clone(), r["component"].clone(), r["section_basis"].clone());
    for (ours, theirs) in [("W16T-CANT-FULL-SPANS", "W16C-WIND-FULL-EX-TIMOSHENKO"), ("W16T-CANT-INTERIOR-EB", "W16C-WIND-PARTIAL-EX-EB")] {
        let (a, b) = (case_ref(ours), w16c_case(theirs));
        let mut matched = 0;
        for r in b["rows"].as_array().unwrap() {
            let mine = a["rows"].as_array().unwrap().iter().find(|x| key(x) == key(r)).unwrap_or_else(|| panic!("{ours}: no row {}", label(r)));
            assert_eq!(mine["exact"], r["exact"], "{ours} vs {theirs}: {}", label(r));
            matched += 1;
        }
        assert_eq!(matched, b["rows"].as_array().unwrap().len(), "{ours}: every {theirs} row");
        assert!(matched >= 50, "{ours}: {matched}");
    }
    let (interior, subdivided) = (case_ref("W16T-CANT-INTERIOR"), case_ref("W16T-CANT-SUBDIVIDED"));
    for r in interior["rows"].as_array().unwrap().iter().filter(|r| r["quantity"] == "support_reaction" || (r["quantity"] == "nodal_motion" && r["entity"] == "node:b")) {
        let s = subdivided["rows"].as_array().unwrap().iter().find(|x| key(x) == key(r)).unwrap();
        assert_eq!(s["exact"], r["exact"], "subdivided {}", label(r));
    }
}

/// W16T-COMBINED: 1 * the interior wind case + 1/2 * a seismic case, as a
/// mechanics combination (slice b): the combination's rows match the
/// reference's combined rows and the M3 judging rules.
#[test]
fn a_partial_span_timoshenko_case_combines() {
    let reference = case_ref("W16T-COMBINED");
    let wire = combination_request(&reference);
    for mode in MODES {
        let what = format!("W16T-COMBINED {}", mode.as_str());
        let out = solved(&wire, mode, &what);
        checks_passed(&out, &what);
        let (n, rel, _, pairs) = compare(&out, &reference, ("combination", COMBINATION), &what);
        m3_judge(&pairs, 2.5, &what);
        println!("W16T {what}: {n} rows; max relative error {rel:.3e}");
    }
}

// ------------------------------------------------------------------ negatives
fn arc_document(generation: Value, shear_model: &str) -> Value {
    let sketches: Value = serde_json::from_str(ARC_SKETCHES).unwrap();
    let mut document = current_model::reauthor_request(sketches["sketches"]["U2-L-ANCH-PTW-K2"]["0.3.0"]["document"].clone());
    let model = &mut document["model"];
    for pipe in model["pipe_segments"].as_array_mut().unwrap() {
        pipe["section"]["material_density"] = qv(7850.0, "kg/m^3");
    }
    model["pressure_contract"]["shear_model"] = json!(shear_model);
    model["load_cases"] = json!([{"id": CASE, "pressure_regions": [], "primitive_loads": [], "equivalent_static": generation, "provenance": PROV}]);
    document
}
fn interior_cantilever(generation: Value) -> Value {
    let reference = case_ref("W16T-CANT-INTERIOR");
    let (nodes, pipes, supports) = geometry(&reference["request"]);
    document(TIMOSHENKO, nodes, pipes, supports, vec![case(CASE, Some(generation))])
}

/// N1: any extent on a realized arc keeps the arc's own refusal under
/// Timoshenko, [0, 1] included; a straight tangent's partial span on the same
/// document is admitted.
#[test]
fn n1_realized_arcs_keep_their_extent_refusal() {
    for (a, b) in [(0.2, 0.7), (0.0, 1.0)] {
        let wire = arc_document(wind(&[], &[("pipe:BEND".into(), a, b)]), TIMOSHENKO);
        let found = refused(&wire, "LOAD_INPUT_INVALID", &format!("arc [{a}, {b}]"));
        assert!(found.iter().any(|d| d["message"].as_str().unwrap().contains("partial extents are not supported on realized arcs")));
    }
    let sketches: Value = serde_json::from_str(ARC_SKETCHES).unwrap();
    let doc = current_model::reauthor_request(sketches["sketches"]["U2-L-ANCH-PTW-K2"]["0.3.0"]["document"].clone());
    let straight = doc["model"]["pipe_segments"].as_array().unwrap().iter().map(|p| p["id"].as_str().unwrap().to_string())
        .find(|id| id != "pipe:BEND").unwrap();
    let wire = arc_document(wind(&[], &[(straight, 0.2, 0.7)]), TIMOSHENKO);
    for mode in MODES {
        let out = solved(&wire, mode, "arc document tangent partial span");
        checks_passed(&out, "arc document tangent partial span");
    }
}

/// N2: a wind marking on a connector-replaced span stays refused under
/// Timoshenko.
#[test]
fn n2_replaced_spans_stay_refused() {
    let reference = case_ref("W16T-CANT-INTERIOR");
    let (mut nodes, mut pipes, supports) = geometry(&reference["request"]);
    nodes.push(node("node:c", [3.5, 0.0, 0.0]));
    pipes.push(pipe("pipe:b-c", "node:b", "node:c", [0.0, 1.0, 0.0]));
    let mut wire = document(TIMOSHENKO, nodes, pipes, supports, vec![case(CASE, Some(wind(&[], &[("pipe:b-c".into(), 0.2, 0.7)])))]);
    wire["model"]["components"] = json!([{"id": "component:w16t-connector", "label": "Invented expansion joint (objective connector)", "kind": "expansion_joint",
        "node": "node:c", "provenance": PROV,
        "objective_connector": {"version": "1.0.0", "motion_basis": "symmetric_midpoint_small_rotation_v1",
            "end_i": {"node_ref": "node:b", "initial_node_axes_global": [[1, 0, 0], [0, 1, 0], [0, 0, 1]], "offset_local": {"x": 0, "y": 0, "z": 0, "unit": "m"}},
            "end_j": {"node_ref": "node:c", "initial_node_axes_global": [[1, 0, 0], [0, 1, 0], [0, 0, 1]], "offset_local": {"x": 0, "y": 0, "z": 0, "unit": "m"}},
            "connector_axes_global": [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
            "installed_reference_temperature": {"value": 20, "unit": "degC"},
            "temperature_applicability": "fixed_installed_parameters_v1", "reference_state": "stress_free",
            "q_ref": {"translation": {"x": 0, "y": 0, "z": 0, "unit": "m"}, "rotation": {"x": 0, "y": 0, "z": 0, "unit": "rad"}},
            "stiffness": {"version": "1.0.0", "representation": "scaled_work_coefficients_v1",
                "translation_scale": {"value": 0.5, "unit": "m"}, "rotation_scale": {"value": 1, "unit": "rad"}, "coefficient_unit": "N*m",
                "upper_triangle": [800000, 0, 0, 0, 0, 0, 225000, 0, 0, 0, 0, 225000, 0, 0, 0, 620000, 0, 0, 480000, 0, 480000],
                "coordinate_order": ["tx", "ty", "tz", "rx", "ry", "rz"],
                "provenance": {"source_reference": PROV, "measurement_restraints": "invented", "basis_transform_reference": "none"}},
            "topology": {"type": "replaces_span", "span_ref": "pipe:b-c"},
            "calibration": {"kind": "constant_structural_elasticity_v1", "includes_pressure_dependent_tangent": false, "installed_geometry": "authored model geometry"},
            "hardware": {"kind": "untied"}, "pressure_model": {"kind": "unpressurized"},
            "provenance": {"source_reference": PROV, "measurement_restraints": "invented", "basis_transform_reference": "none", "validity_statement": "invented analytical control; no applicability claim"}}}]);
    let found = refused(&wire, "EQUIVALENT_STATIC_WIND_SPAN_REPLACED", "Timoshenko partial span on the replaced span");
    assert!(found[0]["affected_refs"].as_array().unwrap().contains(&json!("pipe:b-c")));
}

/// N3: a load/reference-state document refuses the generation by name, a
/// Timoshenko partial span included, and a case not listed in the group too.
#[test]
fn n3_load_state_documents_stay_refused() {
    let state = json!({"contract": "openpipestress.load_reference_state/1.0.0", "reference_configuration_ref": "reference:installed",
        "element_states": [{"pipe_ref": "pipe:a-b", "material_selection": {"kind": "explicit_base_properties", "material_ref": "material:w16t",
            "applicability_reference": "invented analytical basis declared applicable to this member"},
            "thermal_state": {"kind": "unchanged_reference", "provenance": PROV}}],
        "support_states": [{"support_ref": "support:a", "participation": {"kind": "active_model_device"}}],
        "load_sources": [], "history": {"kind": "independent_equilibrium"}, "provenance": PROV});
    let partial = wind(&[], &[("pipe:a-b".into(), 0.2, 0.7)]);
    let mut first = case(CASE, Some(partial.clone()));
    first["analysis_state"] = state;
    let reference = case_ref("W16T-CANT-INTERIOR");
    let (nodes, pipes, supports) = geometry(&reference["request"]);
    let mut wire = document(TIMOSHENKO, nodes, pipes, supports, vec![first, case("case:unlisted", Some(partial))]);
    wire["model"]["reference_configurations"] = json!([{"id": "reference:installed", "label": "Installed reference", "geometry_ref": {"kind": "authored_model_geometry"},
        "member_references": [{"pipe_ref": "pipe:a-b", "basis": {"kind": "direct_strain_reference"}, "fit": {"kind": "none"}, "provenance": PROV}], "provenance": PROV}]);
    let found = refused(&wire, "EXACT_EQUIVALENT_STATIC_LOAD_STATE_UNSUPPORTED", "load-state Timoshenko partial span");
    assert_eq!(found.len(), 2);
}

/// N4: invalid fractions, overlap and double marking keep
/// `EQUIVALENT_STATIC_INPUT_INVALID` under Timoshenko.
#[test]
fn n4_invalid_extents_stay_refused() {
    for spans in [
        vec![("pipe:a-b".to_string(), 0.7, 0.2)],
        vec![("pipe:a-b".to_string(), 0.5, 0.5)],
        vec![("pipe:a-b".to_string(), -0.1, 0.5)],
        vec![("pipe:a-b".to_string(), 0.5, 1.1)],
        vec![("pipe:a-b".to_string(), 0.0, 0.6), ("pipe:a-b".to_string(), 0.5, 1.0)],
    ] {
        refused(&interior_cantilever(wind(&[], &spans)), "EQUIVALENT_STATIC_INPUT_INVALID", &format!("{spans:?}"));
    }
    refused(&interior_cantilever(wind(&["pipe:a-b"], &[("pipe:a-b".into(), 0.2, 0.7)])), "EQUIVALENT_STATIC_INPUT_INVALID", "double marking");
    // A non-numeric fraction (JSON has no NaN) is rejected at the typed
    // boundary, before generation.
    let mut null = interior_cantilever(wind(&[], &[("pipe:a-b".into(), 0.2, 0.7)]));
    null["model"]["load_cases"][0]["equivalent_static"]["wind"]["exposed_spans"][0]["start_fraction"]["value"] = json!(null);
    assert!(run_linear_static_preview_value_with_mode(null, PreviewSolverMode::DenseScrutiny).is_err());
}

/// N5: the retained sub-route keeps declining generation: the ordinary
/// result is returned unchanged.
#[test]
fn n5_the_retained_sub_route_declines() {
    let wire = reference_request(&case_ref("W16T-CANT-INTERIOR"));
    for mode in MODES {
        let ordinary = run_linear_static_preview_value_with_mode(wire.clone(), mode).unwrap();
        let retained = run_linear_static_preview_value_with_retained_direct(wire.clone(), mode, RETAINED_LIMIT).unwrap();
        assert!(retained.successor().is_none());
        assert_eq!(serde_json::to_vec(retained.envelope()).unwrap(), serde_json::to_vec(&ordinary).unwrap());
        assert_eq!(ordinary.status.mechanics, "MECHANICS_SOLVED");
    }
}

/// N6: generated cases stay out of selected retained-source recovery. On a
/// Timoshenko document where another case selects source recovery
/// (pressure-source-1), the partial-span wind case is an ordinary companion:
/// it is not source-recovered and carries its generation record.
#[test]
fn n6_source_recovery_excludes_the_generating_case() {
    let mut wire: Value = serde_json::from_str(SOURCE_MIXED).unwrap();
    let model = &mut wire["model"];
    model["pressure_contract"]["shear_model"] = json!(TIMOSHENKO);
    let member = model["pipe_segments"][0]["id"].as_str().unwrap().to_string();
    model["load_cases"][1] = json!({"id": CASE, "modulus_basis_ref": "point:compliant", "pressure_regions": [], "primitive_loads": [],
        "equivalent_static": wind(&[], &[(member.clone(), 0.2, 0.7)]), "provenance": PROV});
    for mode in MODES {
        let out = run(&wire, mode);
        never_retired(&out, "source companion");
        assert_eq!(out["status"]["mechanics"], "MECHANICS_SOLVED", "{:?}", blocking(&out));
        assert_eq!(out["producer"]["semantic_contract_id"], "openpipestress.result_semantics/0.3.0/pressure-source-1", "{:#?}", out["diagnostics"]);
        let cases = out["contract_evidence"]["exact_cases"].as_array().unwrap();
        assert_eq!(cases[0]["recovery_method"], "retained_source_blocks_exact_v1");
        assert_ne!(cases[1]["recovery_method"], "retained_source_blocks_exact_v1");
        assert_eq!(cases[1]["equivalent_static"]["generated_load_ids"],
            json!([format!("{CASE}:generated:wind:global-z:element-0:extent-0.2-0.7")]));
    }
}
