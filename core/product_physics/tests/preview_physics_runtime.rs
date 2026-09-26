//! T0R preview-physics-1 runtime evidence. Every expectation is read from the
//! frozen hand-statics references: a byte-identical copy of
//! DEFAULT_ROUTE_DESIGN/_run_records/references.stdout.txt, kept in this crate's
//! fixtures so the numerical CI path policy selects this suite, and pinned to
//! the frozen SHA-256 recorded in DEFAULT_ROUTE_DESIGN/_run_records/SHA256SUMS;
//! nothing here derives an expectation from product code. Every model goes
//! through the captured value entry the desktop uses. Criterion:
//! |observed - expected| <= 1e-9 * max(|expected|, zero scale). No new tolerance.
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};
use std::collections::HashSet;

const PREVIEW: &str = "openpipestress.result_semantics/0.3.0/preview-physics-1";
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const SQRT1_2: f64 = std::f64::consts::FRAC_1_SQRT_2;

const FROZEN_REFERENCES: &str = include_str!("fixtures/preview_physics/references.stdout.txt");
/// DEFAULT_ROUTE_DESIGN/_run_records/SHA256SUMS, `_run_records/references.stdout.txt`.
const FROZEN_REFERENCES_SHA256: &str = "1d1bdeed37c78ff11bb9bd913dd7b23eaeab3beddbbfd5f3b56f4ed448cba33d";
fn refs() -> Value {
    serde_json::from_str(FROZEN_REFERENCES).unwrap()
}

#[test]
fn reference_copy_is_the_frozen_bytes() {
    use sha2::{Digest, Sha256};
    let digest = Sha256::digest(FROZEN_REFERENCES.as_bytes());
    let hex: String = digest.iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(hex, FROZEN_REFERENCES_SHA256, "the crate copy must stay byte-identical to the frozen references");
}
fn r2() -> Value {
    refs()["revision_2"].clone()
}
fn num(v: &Value) -> f64 {
    v.as_f64().unwrap_or_else(|| panic!("not a number: {v}"))
}
/// A zero expectation with no zero scale recorded in the frozen references.
const NO_FROZEN_SCALE: f64 = f64::NAN;
/// The frozen criterion (ROOT ruling on R1 N-2): a nonzero expectation uses
/// pure relative 1e-9. Only an exactly-zero expectation uses a scale, and only
/// one recorded in the frozen references; without one it must be exactly zero.
fn close(observed: f64, expected: f64, zero_scale: f64, what: &str) {
    let tolerance = if expected != 0.0 {
        1e-9 * expected.abs()
    } else if zero_scale.is_nan() {
        0.0
    } else {
        1e-9 * zero_scale
    };
    assert!(
        observed.is_finite() && (observed - expected).abs() <= tolerance,
        "{what}: observed {observed:.17e} expected {expected:.17e} (zero scale {zero_scale})"
    );
}
fn close6(observed: [f64; 6], expected: &[f64], fscale: f64, mscale: f64, what: &str) {
    for k in 0..6 {
        close(observed[k], expected[k], if k < 3 { fscale } else { mscale }, &format!("{what}[{k}]"));
    }
}
fn vec3(v: &Value) -> Vec<f64> {
    v.as_array().unwrap().iter().map(num).collect()
}

// ------------------------------------------------------------------ model builders
fn base(nodes: Value, pipes: Value, supports: Value, components: Value, cases: Value, combinations: Value) -> Value {
    json!({"model":{"schema_version":"0.2.0","document_kind":"openpipestress.product_preview.model",
        "project":{"id":"project:t0r","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","stress":"Pa","temperature":"degC"}},
        "analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},
        "nodes":nodes,"pipe_segments":pipes,"supports":supports,"components":components,
        "materials":[{"id":"material:m","elastic_modulus":{"value":200e9,"unit":"Pa"},"shear_modulus":{"value":76.923076923e9,"unit":"Pa"},
            "thermal_expansion_coefficient":{"value":1.2e-5,"unit":"1/degC"},
            "temperature_points":[{"id":"point:hot","temperature":{"value":300.0,"unit":"degC"},"elastic_modulus":{"value":180e9,"unit":"Pa"},"shear_modulus":{"value":69.2e9,"unit":"Pa"}}],
            "provenance":"t0r_invented_test_values"}],
        "load_cases":cases,"combinations":combinations},"materials":[]})
}
fn node(id: &str, x: f64, y: f64, z: f64) -> Value {
    json!({"id":id,"position":{"x":x,"y":y,"z":z},"provenance":"t0r"})
}
fn pipe_od(id: &str, from: &str, to: &str, yref: [f64; 3], od: f64, wall: f64) -> Value {
    json!({"id":id,"from":from,"to":to,"section":{"outside_diameter":{"value":od,"unit":"m"},"wall_thickness":{"value":wall,"unit":"m"}},"material":"material:m","y_reference":{"x":yref[0],"y":yref[1],"z":yref[2]},"provenance":"t0r"})
}
fn pipe(id: &str, from: &str, to: &str, yref: [f64; 3]) -> Value {
    pipe_od(id, from, to, yref, 0.12, 0.01)
}
fn force(id: &str, node: &str, dir: &str, v: f64) -> Value {
    json!({"id":id,"category":"concentrated_force","target":{"type":"node","node":node},"direction":dir,"magnitude":{"value":v,"unit":"N"},"dimension":"force","provenance":"t0r"})
}
fn moment(id: &str, node: &str, dir: &str, v: f64) -> Value {
    json!({"id":id,"category":"concentrated_moment","target":{"type":"node","node":node},"direction":dir,"magnitude":{"value":v,"unit":"N*m"},"dimension":"moment","provenance":"t0r"})
}
fn line(id: &str, pipe: &str, dir: &str, w: f64) -> Value {
    json!({"id":id,"category":"distributed_force","target":{"type":"element","pipe":pipe},"direction":dir,"magnitude":{"value":w,"unit":"N/m"},"dimension":"force_per_length","provenance":"t0r"})
}
fn support(id: &str, node: &str, restraints: &[&str]) -> Value {
    json!({"id":id,"node":node,"family":"anchor","restraints":restraints,"provenance":"t0r"})
}
fn anchor(node: &str) -> Value {
    support("support:a", node, &["UX", "UY", "UZ", "RX", "RY", "RZ"])
}
fn spring(id: &str, node: &str, dof: &str, k: f64) -> Value {
    json!({"id":id,"node":node,"family":"spring","restraints":[],"stiffness":{"dof":dof,"value":{"value":k,"unit":"N/m"}},"provenance":"t0r"})
}
fn case(id: &str, loads: Value) -> Value {
    json!({"id":id,"primitive_loads":loads,"provenance":"t0r"})
}
fn mechanics(id: &str, terms: &[(&str, f64)]) -> Value {
    json!({"id":id,"basis":"mechanics","terms":terms.iter().map(|(c,f)| json!({"load_case":c,"factor":f})).collect::<Vec<_>>(),"provenance":"t0r"})
}
fn subtraction(id: &str, a: &str, b: &str) -> Value {
    json!({"id":id,"basis":"result_state_subtraction","minuend_id":a,"subtrahend_id":b,"provenance":"t0r"})
}
fn range(id: &str, operands: &[&str], mode: &str) -> Value {
    json!({"id":id,"basis":"range_envelope","operand_ids":operands,"mode":mode,"provenance":"t0r"})
}
/// C1 1 m cantilever along +x, anchor at a, tip b.
fn cantilever(extra_supports: Vec<Value>, cases: Value, combinations: Value) -> Value {
    let mut supports = vec![anchor("node:a")];
    supports.extend(extra_supports);
    base(
        json!([node("node:a", 0.0, 0.0, 0.0), node("node:b", 1.0, 0.0, 0.0)]),
        json!([pipe("pipe:a-b", "node:a", "node:b", [0.0, 1.0, 0.0])]),
        json!(supports),
        json!([]),
        cases,
        combinations,
    )
}

// ------------------------------------------------------------------ envelope access
fn solve(wire: &Value, mode: PreviewSolverMode) -> Value {
    let envelope = run_linear_static_preview_value_with_mode(wire.clone(), mode).expect("captured entry");
    let v = serde_json::to_value(envelope).unwrap();
    assert_eq!(v["producer"]["semantic_contract_id"], PREVIEW);
    v
}
fn solved(wire: &Value, mode: PreviewSolverMode) -> Value {
    let v = solve(wire, mode);
    assert_eq!(v["status"]["mechanics"], "MECHANICS_SOLVED", "{}", v["diagnostics"]);
    assert_contract(&v);
    v
}
fn results(v: &Value) -> &Vec<Value> {
    v["results"].as_array().unwrap()
}
fn basis(row: &Value) -> (&str, &str) {
    (row["basis_ref"]["ref_type"].as_str().unwrap_or(""), row["basis_ref"]["ref_id"].as_str().unwrap_or(""))
}
fn support_action(v: &Value, support: &str, ref_type: &str, ref_id: &str) -> Option<[f64; 6]> {
    let mut out = [f64::NAN; 6];
    for (k, c) in ["Fx", "Fy", "Fz", "Mx", "My", "Mz"].iter().enumerate() {
        let row = results(v).iter().find(|r| {
            r["kind"] == "support_reaction_component_v2" && r["entity_ref"] == support && r["metadata"]["component"] == *c && basis(r) == (ref_type, ref_id)
        })?;
        out[k] = num(&row["value"]);
    }
    Some(out)
}
fn case_action(v: &Value, support: &str, case: &str) -> [f64; 6] {
    support_action(v, support, "load_case", case).unwrap_or_else(|| panic!("no signed action for {support} in {case}"))
}
fn magnitude(v: &Value, support: &str, which: &str, ref_type: &str, ref_id: &str) -> f64 {
    let kind = format!("support_reaction_{which}_magnitude_v2");
    num(&results(v).iter().find(|r| r["kind"] == kind.as_str() && r["entity_ref"] == support && basis(r) == (ref_type, ref_id)).unwrap_or_else(|| panic!("no {kind} for {support}"))["value"])
}
fn maximum(v: &Value, pipe: &str, case: &str) -> Option<f64> {
    results(v)
        .iter()
        .find(|r| r["kind"] == "pipe_elastic_normal_stress_maximum_v2" && r["entity_ref"] == pipe && basis(r) == ("load_case", case))
        .map(|r| num(&r["value"]))
}
fn row_value(v: &Value, kind: &str, entity: &str, component: Option<&str>, location: Option<&str>, ref_type: &str, ref_id: &str) -> f64 {
    let row = results(v)
        .iter()
        .find(|r| {
            r["kind"] == kind
                && r["entity_ref"] == entity
                && component.is_none_or(|c| r["metadata"]["component"] == c)
                && location.is_none_or(|l| r["metadata"]["location"] == l)
                && basis(r) == (ref_type, ref_id)
        })
        .unwrap_or_else(|| panic!("missing {kind} {entity} {component:?} {location:?} {ref_type}:{ref_id}"));
    num(&row["value"])
}
fn intensified(v: &Value, component: &str, location: &str, case: &str) -> Vec<f64> {
    results(v)
        .iter()
        .filter(|r| r["kind"] == "component_equal_factor_intensified_bending_stress_v1" && r["entity_ref"] == component && r["metadata"]["location"] == location && basis(r) == ("load_case", case))
        .map(|r| num(&r["value"]))
        .collect()
}
fn intensified_for(v: &Value, pipe: &str, location: &str, case: &str) -> Vec<f64> {
    let ids: HashSet<String> = v["contract_evidence"]["preview_cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["load_case_id"] == case)
        .flat_map(|c| c["intensified_measures"].as_array().unwrap().clone())
        .filter(|m| m["pipe_id"] == pipe && m["location"] == location)
        .map(|m| m["result_id"].as_str().unwrap().to_string())
        .collect();
    results(v).iter().filter(|r| ids.contains(r["id"].as_str().unwrap())).map(|r| num(&r["value"])).collect()
}
fn codes(v: &Value) -> Vec<&str> {
    v["diagnostics"].as_array().unwrap().iter().map(|d| d["code"].as_str().unwrap()).collect()
}
fn diag<'a>(v: &'a Value, code: &str) -> Vec<&'a Value> {
    v["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == code).collect()
}
fn gate(v: &Value, combination: &str) -> Value {
    v["contract_evidence"]["combination_gates"].as_array().unwrap().iter().find(|g| g["combination_id"] == combination).unwrap().clone()
}
fn combination_rows<'a>(v: &'a Value, combination: &str) -> Vec<&'a Value> {
    results(v).iter().filter(|r| basis(r) == ("combination", combination)).collect()
}

/// S1 §9 checks that every rendered envelope must satisfy (producer side).
fn assert_contract(v: &Value) {
    let retired = ["reaction_resultant", "open_formula_stress_summary", "component_user_stress_multiplier_review"];
    let ids: HashSet<&str> = results(v).iter().map(|r| r["id"].as_str().unwrap()).collect();
    assert_eq!(ids.len(), results(v).len(), "duplicate row ids");
    for row in results(v) {
        assert!(!retired.contains(&row["kind"].as_str().unwrap()), "retired kind {}", row["id"]);
    }
    for d in v["diagnostics"].as_array().unwrap() {
        assert!(!["COMPONENT_STRESS_MULTIPLIER_APPLIED", "COMBINATION_STRESS_SUMMARY_SKIPPED"].contains(&d["code"].as_str().unwrap()), "retired code {d}");
        for r in d["affected_refs"].as_array().into_iter().flatten() {
            let r = r.as_str().unwrap();
            assert!(!r.starts_with("result:") || ids.contains(r), "dangling ref {r} in {}", d["id"]);
        }
    }
    for key in ["max_displacement", "max_open_formula_stress"] {
        if let Some(r) = v["summary"][key]["result_ref"].as_str() {
            assert!(ids.contains(r), "dangling summary ref {r}");
        }
    }
    let evidence = &v["contract_evidence"];
    assert_eq!(evidence.as_object().unwrap().len(), 2);
    let cases = evidence["preview_cases"].as_array().unwrap();
    let mut all_complete = true;
    for c in cases {
        let keys: HashSet<&str> = c.as_object().unwrap().keys().map(String::as_str).collect();
        assert_eq!(keys, HashSet::from(["load_case_id", "pipe_stress_extrema", "stress_maximum_coverage", "support_attribution", "intensified_measures"]));
        let case = c["load_case_id"].as_str().unwrap();
        let members: HashSet<&str> = results(v)
            .iter()
            .filter(|r| r["kind"] == "element_local_axial_force" && basis(r) == ("load_case", case))
            .map(|r| r["entity_ref"].as_str().unwrap())
            .collect();
        let mut covered = HashSet::new();
        for e in c["pipe_stress_extrema"].as_array().unwrap() {
            let row = results(v).iter().find(|r| r["id"] == e["result_id"]).expect("extrema row");
            let (lo, hi, val) = (num(&e["value_lower_pa"]), num(&e["value_upper_pa"]), num(&row["value"]));
            assert!(lo <= val && val <= hi && val == lo + 0.5 * (hi - lo));
            assert!(covered.insert(e["pipe_id"].as_str().unwrap()));
        }
        let cov = &c["stress_maximum_coverage"];
        for key in ["unavailable_pipe_ids", "outside_domain_pipe_ids"] {
            for p in cov[key].as_array().unwrap() {
                assert!(covered.insert(p.as_str().unwrap()));
            }
        }
        assert_eq!(covered, members, "coverage partitions members in {case}");
        let complete = cov["complete"].as_bool().unwrap();
        assert_eq!(complete, cov["unavailable_pipe_ids"].as_array().unwrap().is_empty() && cov["outside_domain_pipe_ids"].as_array().unwrap().is_empty());
        all_complete &= complete;
        let att = &c["support_attribution"];
        for s in att["attributed_support_ids"].as_array().unwrap() {
            let s = s.as_str().unwrap();
            let a = case_action(v, s, case);
            close(magnitude(v, s, "force", "load_case", case), a[0].hypot(a[1]).hypot(a[2]), 1e-300, "force magnitude");
            close(magnitude(v, s, "moment", "load_case", case), a[3].hypot(a[4]).hypot(a[5]), 1e-300, "moment magnitude");
        }
        for w in att["withheld"].as_array().unwrap() {
            let s = w["support_id"].as_str().unwrap();
            assert!(!results(v).iter().any(|r| r["entity_ref"] == s && basis(r) == ("load_case", case) && r["kind"].as_str().unwrap().starts_with("support_reaction_")), "withheld {s} has rows");
        }
        for m in c["intensified_measures"].as_array().unwrap() {
            let row = results(v).iter().find(|r| r["id"] == m["result_id"]).expect("intensified row");
            let expected = num(&m["sif"]) * (num(&m["bending_moment_y_n_m"]).hypot(num(&m["bending_moment_z_n_m"])) / num(&m["section_modulus_m3"]));
            assert_eq!(num(&row["value"]), expected);
        }
    }
    let has_max = results(v).iter().any(|r| r["kind"] == "pipe_elastic_normal_stress_maximum_v2");
    assert_eq!(v["summary"]["max_open_formula_stress"].is_null(), !(all_complete && has_max && !cases.is_empty()), "headline presence");
    for r in results(v).iter().filter(|r| r["basis_ref"]["ref_type"] == "combination") {
        assert_ne!(r["kind"], "pipe_elastic_normal_stress_maximum_v2");
        assert_ne!(r["kind"], "component_equal_factor_intensified_bending_stress_v1");
        assert_eq!(gate(v, r["basis_ref"]["ref_id"].as_str().unwrap())["withheld"], false);
    }
}

// ------------------------------------------------------------------ M14 / M33
#[test]
fn m33_headline_is_all_case_maximum_with_first_case_control() {
    let refs = refs();
    let g = &refs["REF-M33-G"];
    for mode in MODES {
        for reverse in [false, true] {
            let mut cases = vec![
                case("case:A", json!([force("fa-y", "node:b", "global_y", 1000.0), force("fa-z", "node:b", "global_z", 1000.0)])),
                case("case:B", json!([force("fb-y", "node:b", "global_y", 3000.0)])),
            ];
            if reverse {
                cases.reverse();
            }
            let v = solved(&cantilever(vec![], json!(cases), json!([])), mode);
            close(maximum(&v, "pipe:a-b", "case:A").unwrap(), num(&g["case_A_max_Pa"]), 0.0, "case A max");
            close(maximum(&v, "pipe:a-b", "case:B").unwrap(), num(&g["case_B_max_Pa"]), 0.0, "case B max");
            close(maximum(&v, "pipe:a-b", "case:A").unwrap(), num(&refs["REF-M14-A"]["circular_normal_max_Pa"]), 0.0, "REF-M14-A");
            let h = &v["summary"]["max_open_formula_stress"];
            close(num(&h["value"]), num(&g["headline_Pa"]), 0.0, "stress headline");
            assert_eq!(h["unit"], "Pa");
            assert_eq!(h["location_ref"], "pipe:a-b");
            assert!(h["result_ref"].as_str().unwrap().contains("case:B"));
            assert_ne!(num(&h["value"]), num(&g["negative_control_first_case_headline_Pa"]));
            assert_ne!(num(&h["value"]), num(&refs["REF-M14-A"]["negative_control_abs_sum_Pa"]));
            close(num(&v["summary"]["max_displacement"]["value"]), num(&g["displacement_headline_mm"]), 0.0, "displacement headline");
            assert_eq!(v["summary"]["max_displacement"]["location_ref"], "node:b");
            assert!(codes(&v).iter().all(|c| *c != "PREVIEW_HEADLINE_SCOPE_LOAD_CASES"));
        }
    }
}

#[test]
fn m14_s1_rotated_and_pure_torque() {
    let refs = refs();
    let s1 = &refs["REF-M14-S1"];
    let scale = num(&refs["revision_2"]["zero_scales_rev1"]["REF-M14-S1.pure_torque_only_normal_max_Pa"]);
    let pi = std::f64::consts::PI;
    let model = |loads: Value| {
        base(json!([node("node:a", 0.0, 0.0, 0.0), node("node:b", 1.0, 0.0, 0.0)]), json!([pipe_od("pipe:a-b", "node:a", "node:b", [0.0, 1.0, 0.0], 0.10, 0.01)]), json!([anchor("node:a")]), json!([]), json!([case("case:s1", loads)]), json!([]))
    };
    for mode in MODES {
        let v = solved(&model(json!([force("x", "node:b", "global_x", 1800.0 * pi), force("y", "node:b", "global_y", 73.8 * pi), force("z", "node:b", "global_z", 55.35 * pi)])), mode);
        let m = maximum(&v, "pipe:a-b", "case:s1").unwrap();
        close(m, num(&s1["circular_normal_max_Pa"]), 0.0, "S1");
        assert!((m - num(&s1["negative_control_abs_sum_Pa"])).abs() > 1e-6 * m);
        let v = solved(&model(json!([force("x", "node:b", "global_x", 1800.0 * pi), force("z", "node:b", "global_z", 92.25 * pi)])), mode);
        close(maximum(&v, "pipe:a-b", "case:s1").unwrap(), num(&s1["rotated_circular_normal_max_Pa"]), 0.0, "S1 rotated");
        let v = solved(&model(json!([moment("t", "node:b", "rotation_x", 221.4 * pi)])), mode);
        close(maximum(&v, "pipe:a-b", "case:s1").unwrap(), 0.0, scale, "pure torque normal max");
    }
}

#[test]
fn m14_x1_enclosure_and_partial_restraint_rows() {
    let refs = refs();
    let x1 = &refs["REF-M14-X1"];
    let sup = &refs["revision_2"]["REF-M14-X1-SUPPORTS"];
    let wire = base(
        json!([node("node:i", 0.0, 0.0, 0.0), node("node:j", 1.0, 0.0, 0.0)]),
        json!([pipe_od("pipe:x1", "node:i", "node:j", [0.0, 1.0, 0.0], 0.10, 0.01)]),
        json!([support("support:pin", "node:i", &["UX", "UY", "UZ", "RX"]), support("support:roller", "node:j", &["UY", "UZ"])]),
        json!([]),
        json!([case("case:x1", json!([line("w", "pipe:x1", "global_z", num(&x1["w_z_N_per_m"])), moment("c", "node:j", "rotation_z", num(&x1["end_couple_Nm"]))]))]),
        json!([]),
    );
    for mode in MODES {
        let v = solved(&wire, mode);
        let e = &v["contract_evidence"]["preview_cases"][0]["pipe_stress_extrema"][0];
        let star = num(&x1["circular_normal_max_Pa"]);
        assert!(num(&e["value_lower_pa"]) <= star * (1.0 + 1e-9) && star * (1.0 - 1e-9) <= num(&e["value_upper_pa"]), "enclosure contains f*");
        close(maximum(&v, "pipe:x1", "case:x1").unwrap(), star, 0.0, "X1 maximum");
        assert!(maximum(&v, "pipe:x1", "case:x1").unwrap() > num(&x1["negative_control_old_eight_sign_candidates_Pa"]) * (1.0 + 1e-6));
        let zs = num(&sup["zero_scale_N_or_Nm"]);
        close6(case_action(&v, "support:pin", "case:x1"), &vec3(&sup["pin_i_support_on_pipe"]), zs, zs, "pin");
        close6(case_action(&v, "support:roller", "case:x1"), &vec3(&sup["roller_j_support_on_pipe"]), zs, zs, "roller");
    }
}

#[test]
fn m14_thermal_axial_force_enters_the_maximum() {
    let th = r2()["REF-M14-TH"].clone();
    let wire = base(
        json!([node("node:a", 0.0, 0.0, 0.0), node("node:b", 1.0, 0.0, 0.0)]),
        json!([pipe("pipe:a-b", "node:a", "node:b", [0.0, 1.0, 0.0])]),
        json!([anchor("node:a"), support("support:b", "node:b", &["UX", "UY", "UZ", "RX", "RY", "RZ"])]),
        json!([]),
        json!([case("case:th", json!([{"id":"t","category":"thermal","target":{"type":"element","pipe":"pipe:a-b"},"direction":"global_x","magnitude":{"value":100.0,"unit":"degC"},"dimension":"temperature_interval","provenance":"t0r"}]))]),
        json!([]),
    );
    for mode in MODES {
        let v = solved(&wire, mode);
        close(maximum(&v, "pipe:a-b", "case:th").unwrap(), num(&th["circular_normal_max_Pa"]), 0.0, "thermal maximum");
        let (fs, ms) = (num(&th["zero_scale_N"]), num(&th["zero_scale_Nm"]));
        close6(case_action(&v, "support:a", "case:th"), &vec3(&th["anchor_a_support_on_pipe"]), fs, ms, "anchor a");
        close6(case_action(&v, "support:b", "case:th"), &vec3(&th["anchor_b_support_on_pipe"]), fs, ms, "anchor b");
    }
}

// ------------------------------------------------------------------ M05
#[test]
fn m05_pure_torque_publishes_the_moment() {
    let refs = refs();
    let t = &refs["REF-M05-T"];
    let scale = num(&refs["revision_2"]["zero_scales_rev1"]["REF-M05-T.force_components_N"]);
    let mscale = num(&refs["revision_2"]["zero_scales_rev1"]["REF-M05-T.moment_y_z_Nm"]);
    for mode in MODES {
        let v = solved(&cantilever(vec![], json!([case("case:t", json!([moment("t", "node:b", "rotation_x", 500.0)]))]), json!([])), mode);
        let a = case_action(&v, "support:a", "case:t");
        let expected: Vec<f64> = vec3(&t["anchor_F_N"]).into_iter().chain(vec3(&t["anchor_M_Nm"])).collect();
        close6(a, &expected, scale, mscale, "torque anchor");
        close(magnitude(&v, "support:a", "force", "load_case", "case:t"), 0.0, scale, "force magnitude");
        close(magnitude(&v, "support:a", "moment", "load_case", "case:t"), num(&t["moment_magnitude_Nm"]), 0.0, "moment magnitude");
    }
}

fn r1_loads(q: &dyn Fn([f64; 3]) -> [f64; 3], tip: &str, pipe: &str, force_scale: f64) -> Value {
    let f = q([10.0, -20.0, 30.0]);
    let c = q([4.0, 5.0, -6.0]);
    let w = q([0.0, -3.0, 0.0]);
    let mut loads = Vec::new();
    for (k, axis) in ["x", "y", "z"].iter().enumerate() {
        loads.push(json!({"id":format!("f{axis}"),"category":"concentrated_force","target":{"type":"node","node":tip},"direction":format!("global_{axis}"),"magnitude":{"value":f[k]*force_scale,"unit":if force_scale == 1.0 {"N"} else {"kN"}},"dimension":"force","provenance":"t0r"}));
        loads.push(json!({"id":format!("m{axis}"),"category":"concentrated_moment","target":{"type":"node","node":tip},"direction":format!("rotation_{axis}"),"magnitude":{"value":c[k]*force_scale,"unit":if force_scale == 1.0 {"N*m"} else {"kN*m"}},"dimension":"moment","provenance":"t0r"}));
        loads.push(line(&format!("w{axis}"), pipe, &format!("global_{axis}"), w[k]));
    }
    json!(loads)
}

#[test]
fn m05_r1_rotated_translated_and_unit_variants() {
    let refs = refs();
    let r1 = &refs["REF-M05-R1"];
    let scale = num(&refs["revision_2"]["zero_scales_rev1"]["REF-M05-R1.zero_components"]);
    let identity = |v: [f64; 3]| v;
    let perm = |v: [f64; 3]| [v[2], v[0], v[1]];
    let (c30, s30) = (30f64.to_radians().cos(), 30f64.to_radians().sin());
    let rot = move |v: [f64; 3]| [c30 * v[0] - s30 * v[1], s30 * v[0] + c30 * v[1], v[2]];
    let cases: Vec<(&str, Box<dyn Fn([f64; 3]) -> [f64; 3]>, [f64; 3], &Value)> = vec![
        ("unrotated", Box::new(identity), [0.0; 3], &r1["with_uniform_0_-3_0_N_per_m"]),
        ("perm", Box::new(perm), [0.0; 3], &r1["rotated_perm_xyz_to_yzx"]),
        ("rot30", Box::new(rot), [0.0; 3], &r1["rotated_rot_z_30deg"]),
    ];
    for mode in MODES {
        for (label, q, origin, expected) in &cases {
            let tip = q([2.0, 0.0, 0.0]);
            let yref = q([0.0, 0.0, 1.0]);
            let wire = base(
                json!([node("node:a", origin[0], origin[1], origin[2]), node("node:b", tip[0], tip[1], tip[2])]),
                json!([pipe("pipe:a-b", "node:a", "node:b", yref)]),
                json!([anchor("node:a")]),
                json!([]),
                json!([case("case:r1", r1_loads(q.as_ref(), "node:b", "pipe:a-b", 1.0))]),
                json!([]),
            );
            let v = solved(&wire, mode);
            let e: Vec<f64> = vec3(&expected["anchor_F_N"]).into_iter().chain(vec3(&expected["anchor_M_Nm"])).collect();
            close6(case_action(&v, "support:a", "case:r1"), &e, scale, scale, label);
        }
        // Translated: moments about the attachment node.
        let t = &r2()["REF-M05-R1-TRANSLATED"];
        let o = vec3(&t["anchor_node"]);
        let wire = base(
            json!([node("node:a", o[0], o[1], o[2]), node("node:b", o[0] + 2.0, o[1], o[2])]),
            json!([pipe("pipe:a-b", "node:a", "node:b", [0.0, 0.0, 1.0])]),
            json!([anchor("node:a")]),
            json!([]),
            json!([case("case:r1", r1_loads(&identity, "node:b", "pipe:a-b", 1.0))]),
            json!([]),
        );
        let v = solved(&wire, mode);
        let e: Vec<f64> = vec3(&t["anchor_F_N"]).into_iter().chain(vec3(&t["anchor_M_about_node_Nm"])).collect();
        let a = case_action(&v, "support:a", "case:r1");
        close6(a, &e, scale, scale, "translated");
        assert!((a[3] - num(&t["negative_control_M_about_global_origin_Nm"][0])).abs() > 1.0);
    }
    // mm and kN authoring gives identical N and N*m rows.
    let mut wire = base(
        json!([node("node:a", 0.0, 0.0, 0.0), node("node:b", 2000.0, 0.0, 0.0)]),
        json!([pipe_od("pipe:a-b", "node:a", "node:b", [0.0, 0.0, 1.0], 120.0, 10.0)]),
        json!([anchor("node:a")]),
        json!([]),
        json!([case("case:r1", r1_loads(&identity, "node:b", "pipe:a-b", 0.001))]),
        json!([]),
    );
    wire["model"]["project"]["units"]["length"] = json!("mm");
    wire["model"]["project"]["units"]["force"] = json!("kN");
    for p in wire["model"]["pipe_segments"].as_array_mut().unwrap() {
        p["section"]["outside_diameter"]["unit"] = json!("mm");
        p["section"]["wall_thickness"]["unit"] = json!("mm");
    }
    for load in wire["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap() {
        if load["dimension"] == "force_per_length" {
            load["magnitude"]["value"] = json!(num(&load["magnitude"]["value"]) / 1000.0);
            load["magnitude"]["unit"] = json!("N/mm");
        }
    }
    let v = solve(&wire, PreviewSolverMode::SparseInteractive);
    assert_eq!(v["status"]["mechanics"], "MECHANICS_SOLVED", "{}", v["diagnostics"]);
    let e: Vec<f64> = vec3(&r1["with_uniform_0_-3_0_N_per_m"]["anchor_F_N"]).into_iter().chain(vec3(&r1["with_uniform_0_-3_0_N_per_m"]["anchor_M_Nm"])).collect();
    close6(case_action(&v, "support:a", "case:r1"), &e, scale, scale, "mm/kN");
}

#[test]
fn m05_springs_guides_and_gap_attribution() {
    let r = r2();
    let refs = refs();
    for mode in MODES {
        // REF-M05-SPRING
        let s = &refs["REF-M05-SPRING"];
        let zs = num(&r["zero_scales_rev1"]["REF-M05-SPRING.zero_components"]);
        let v = solved(&cantilever(vec![spring("support:k", "node:b", "UY", 1e6)], json!([case("case:s", json!([force("f", "node:b", "global_y", 1000.0)]))]), json!([])), mode);
        close6(case_action(&v, "support:k", "case:s"), &[0.0, num(&s["spring_on_pipe_Fy_N"]), 0.0, 0.0, 0.0, 0.0], zs, zs, "spring");
        let e: Vec<f64> = vec3(&s["anchor_F_N"]).into_iter().chain(vec3(&s["anchor_M_Nm"])).collect();
        close6(case_action(&v, "support:a", "case:s"), &e, zs, zs, "spring anchor");
        // REF-M05-SPRING2
        let s2 = &r["REF-M05-SPRING2"];
        let zs = num(&s2["zero_scale_N"]);
        let v = solved(&cantilever(vec![spring("support:k", "node:b", "UY", 1e6), support("support:guide", "node:b", &["UX"])], json!([case("case:s", json!([force("fx", "node:b", "global_x", 2000.0), force("fy", "node:b", "global_y", 1000.0)]))]), json!([])), mode);
        close6(case_action(&v, "support:k", "case:s"), &vec3(&s2["spring_on_pipe"]), zs, NO_FROZEN_SCALE, "spring2 spring");
        close6(case_action(&v, "support:guide", "case:s"), &vec3(&s2["guide_on_pipe"]), zs, NO_FROZEN_SCALE, "spring2 guide");
        let e: Vec<f64> = vec3(&s2["anchor"]["F"]).into_iter().chain(vec3(&s2["anchor"]["M"])).collect();
        close6(case_action(&v, "support:a", "case:s"), &e, zs, NO_FROZEN_SCALE, "spring2 anchor");
        // REF-M05-SPRING-GAP (N-1 positive control)
        let sg = &r["REF-M05-SPRING-GAP"];
        let mut gap = sg["gap_support_json"].clone();
        gap["id"] = json!("support:gap");
        gap["node"] = json!("node:b");
    gap["provenance"] = json!("t0r");
        let v = solved(&cantilever(vec![spring("support:k", "node:b", "UY", 1e6), gap], json!([case("case:s", json!([force("f", "node:b", "global_y", 350.0)]))]), json!([])), mode);
        close6(case_action(&v, "support:k", "case:s"), &[0.0, num(&sg["spring_on_pipe_Fy_N"]), 0.0, 0.0, 0.0, 0.0], NO_FROZEN_SCALE, NO_FROZEN_SCALE, "gap spring");
        close6(case_action(&v, "support:gap", "case:s"), &[0.0; 6], num(&sg["gap_on_pipe"]["zero_scale"]), NO_FROZEN_SCALE, "inactive gap");
        let a = case_action(&v, "support:a", "case:s");
        close(a[1], num(&sg["anchor_Fy_N"]), NO_FROZEN_SCALE, "gap anchor Fy");
        close(a[5], num(&sg["anchor_Mz_Nm"]), NO_FROZEN_SCALE, "gap anchor Mz");
        assert!(diag(&v, "SUPPORT_ACTION_ATTRIBUTION_WITHHELD").is_empty());
    }
}

#[test]
fn m05_ambiguous_attribution_is_withheld_not_zero_filled() {
    let at = r2()["REF-ATTR"].clone();
    let mut one_way = at["one_way_support_json"].clone();
    one_way["id"] = json!("support:oneway");
    one_way["node"] = json!("node:b");
    one_way["provenance"] = json!("t0r");
    let wire = cantilever(vec![support("support:guide", "node:b", &["UY"]), one_way], json!([case("case:attr", json!([force("f", "node:b", "global_y", -1000.0)]))]), json!([]));
    for mode in MODES {
        let v = solve(&wire, mode);
        if v["status"]["mechanics"] != "MECHANICS_SOLVED" {
            // A targeted refusal is acceptable (REF-ATTR); it must not publish results.
            assert!(results(&v).is_empty());
            println!("REF-ATTR: refused before solving: {:?}", codes(&v));
            continue;
        }
        assert_contract(&v);
        let zs = num(&at["anchor_on_pipe"]["zero_scale"]);
        close6(case_action(&v, "support:a", "case:attr"), &[0.0; 6], zs, zs, "attr anchor");
        let withheld = v["contract_evidence"]["preview_cases"][0]["support_attribution"]["withheld"].as_array().unwrap().clone();
        let ids: HashSet<&str> = withheld.iter().map(|w| w["support_id"].as_str().unwrap()).collect();
        assert_eq!(ids, HashSet::from(["support:guide", "support:oneway"]));
        assert!(withheld.iter().all(|w| w["reason"] == "SUPPORT_ACTION_ATTRIBUTION_WITHHELD"));
        assert!(support_action(&v, "support:guide", "load_case", "case:attr").is_none());
        assert!(support_action(&v, "support:oneway", "load_case", "case:attr").is_none());
        assert!(!results(&v).iter().any(|r| r["kind"] == "nonlinear_support_final_reaction" && r["entity_ref"] == "support:oneway"));
        assert_eq!(diag(&v, "SUPPORT_ACTION_ATTRIBUTION_WITHHELD").len(), 2);
    }
}

#[test]
fn m05_two_nonlinear_devices_on_one_dof_are_withheld_or_refused() {
    let gap = |id: &str, gap: f64| json!({"id":id,"node":"node:b","family":"nonlinear","restraints":[],"provenance":"t0r",
        "nonlinear":{"behavior":"gap","dof":"UY","initial_state":"inactive","closes_when":"positive_displacement","gap":{"value":gap,"unit":"m"}}});
    let wire = cantilever(vec![gap("support:g1", 0.0001), gap("support:g2", 0.0002)], json!([case("case:nn", json!([force("f", "node:b", "global_y", 5000.0)]))]), json!([]));
    for mode in MODES {
        let v = solve(&wire, mode);
        if v["status"]["mechanics"] != "MECHANICS_SOLVED" {
            assert!(results(&v).is_empty());
            println!("nonlinear+nonlinear: refused before solving: {:?}", codes(&v));
            continue;
        }
        assert_contract(&v);
        let withheld: HashSet<String> = v["contract_evidence"]["preview_cases"][0]["support_attribution"]["withheld"].as_array().unwrap().iter().map(|w| w["support_id"].as_str().unwrap().to_string()).collect();
        assert_eq!(withheld, HashSet::from(["support:g1".to_string(), "support:g2".to_string()]));
        assert!(!results(&v).iter().any(|r| r["kind"] == "nonlinear_support_final_reaction"));
        println!("nonlinear+nonlinear: solved and withheld");
    }
}

#[test]
fn m05_combinations_recompute_magnitudes_and_envelope_components() {
    let refs = refs();
    let c = &refs["REF-M05-COMB"];
    let c2 = &refs["revision_2"]["REF-M05-COMB-2"];
    let zs = num(&refs["revision_2"]["zero_scales_rev1"]["REF-M05-COMB.zero_components"]);
    let wire = cantilever(
        vec![],
        json!([
            case("case:A1", json!([force("a1", "node:b", "global_y", 1000.0)])),
            case("case:T", json!([moment("t", "node:b", "rotation_x", 500.0)])),
            case("case:A2", json!([force("a2", "node:b", "global_z", 1000.0)])),
        ]),
        json!([
            mechanics("comb:lin", &[("case:A1", 2.0), ("case:T", -1.0)]),
            subtraction("comb:sub", "case:A1", "case:T"),
            range("comb:env-t", &["case:A1", "case:T"], "max"),
            range("comb:env-a2", &["case:A1", "case:A2"], "max"),
            mechanics("comb:plus", &[("case:A1", 1.0), ("case:A2", 1.0)]),
            mechanics("comb:minus", &[("case:A1", 1.0), ("case:A2", -1.0)]),
        ]),
    );
    for mode in MODES {
        let v = solved(&wire, mode);
        let lin = &c["mechanics_2A1_minus_T"];
        let e: Vec<f64> = vec3(&lin["F"]).into_iter().chain(vec3(&lin["M"])).collect();
        close6(support_action(&v, "support:a", "combination", "comb:lin").unwrap(), &e, zs, zs, "2A1-T");
        close(magnitude(&v, "support:a", "moment", "combination", "comb:lin"), num(&lin["moment_magnitude_Nm"]), 0.0, "2A1-T |M|");
        assert!((magnitude(&v, "support:a", "moment", "combination", "comb:lin") - num(&c["negative_control_magnitude_algebra_moment_Nm"])).abs() > 1.0);
        let sub = &c["subtraction_A1_minus_T"];
        let e: Vec<f64> = vec3(&sub["F"]).into_iter().chain(vec3(&sub["M"])).collect();
        close6(support_action(&v, "support:a", "combination", "comb:sub").unwrap(), &e, zs, zs, "A1-T");
        close(magnitude(&v, "support:a", "moment", "combination", "comb:sub"), num(&sub["moment_magnitude_Nm"]), 0.0, "A1-T |M|");
        let sub_row = combination_rows(&v, "comb:sub").into_iter().find(|r| r["kind"] == "support_reaction_component_v2").unwrap();
        assert!(sub_row["metadata"]["sign_convention"].as_str().unwrap().contains("not an equilibrium state"));
        close(magnitude(&v, "support:a", "moment", "combination", "comb:env-t"), num(&c2["moment_envelope_A1_T"]["envelope_max_moment_magnitude_Nm"]), 0.0, "envelope A1/T");
        close(magnitude(&v, "support:a", "moment", "combination", "comb:env-a2"), num(&c2["moment_envelope_A1_A2"]["envelope_max_moment_magnitude_Nm"]), 0.0, "envelope A1/A2");
        close(magnitude(&v, "support:a", "force", "combination", "comb:env-t"), num(&c["envelope_max_force_magnitude_N"]), 0.0, "envelope force");
        let disp = |comb: &str| num(&combination_rows(&v, comb).into_iter().find(|r| r["kind"] == "displacement_magnitude" && r["entity_ref"] == "node:b").unwrap()["value"]);
        close(disp("comb:plus"), num(&c2["A1_plus_A2_tip_displacement_magnitude_mm"]), 0.0, "A1+A2 |u|");
        close(disp("comb:minus"), num(&c2["A1_minus_A2_tip_displacement_magnitude_mm"]), 0.0, "A1-A2 |u|");
        let pa = &c2["A1_plus_A2_anchor"];
        let e: Vec<f64> = vec3(&pa["F"]).into_iter().chain(vec3(&pa["M"])).collect();
        close6(support_action(&v, "support:a", "combination", "comb:plus").unwrap(), &e, NO_FROZEN_SCALE, NO_FROZEN_SCALE, "A1+A2 anchor");
        assert_eq!(diag(&v, "PREVIEW_HEADLINE_SCOPE_LOAD_CASES").len(), 1);
        assert_eq!(diag(&v, "COMBINATION_STRESS_MAXIMUM_UNAVAILABLE").len(), 6);
        assert!(v["summary"]["max_displacement"]["result_ref"].as_str().unwrap().contains("loadcase") || v["summary"]["max_displacement"]["result_ref"] == "result:disp:node-b");
    }
}

#[test]
fn constant_effort_counted_once_and_gated_when_factors_do_not_sum_to_one() {
    let ce = r2()["REF-CE"].clone();
    let ce_support = |restraints: &[&str]| {
        json!({"id":"support:ce","node":"node:b","family":"constant_effort_support","restraints":restraints,
            "hanger":{"hanger_type":"constant_effort_support","constant_load":{"value":375.0,"unit":"N"},"travel_range":{"value":0.05,"unit":"m"},"source_reference":"t0r_invented"},"provenance":"t0r"})
    };
    let cases = json!([case("case:W1", json!([force("w1", "node:b", "global_y", -1000.0)])), case("case:W2", json!([force("w2", "node:b", "global_y", -500.0)]))]);
    let combos = json!([mechanics("comb:half", &[("case:W1", 0.5), ("case:W2", 0.5)]), mechanics("comb:over", &[("case:W1", 1.0), ("case:W2", 0.5)]), subtraction("comb:sub", "case:W1", "case:W2")]);
    for mode in MODES {
        let v = solved(&cantilever(vec![ce_support(&["UY"])], cases.clone(), combos.clone()), mode);
        for (case_id, key) in [("case:W1", "case_W1_tip_Fy_-1000"), ("case:W2", "case_W2_tip_Fy_-500")] {
            close6(case_action(&v, "support:ce", case_id), &vec3(&ce[key]["ce_support_on_pipe"]), NO_FROZEN_SCALE, NO_FROZEN_SCALE, "CE action");
            let e: Vec<f64> = vec3(&ce[key]["anchor_F"]).into_iter().chain(vec3(&ce[key]["anchor_M"])).collect();
            close6(case_action(&v, "support:a", case_id), &e, NO_FROZEN_SCALE, NO_FROZEN_SCALE, "CE anchor");
        }
        let half = &ce["comb_0.5W1_0.5W2_sum_factors_1"];
        close(support_action(&v, "support:a", "combination", "comb:half").unwrap()[1], num(&half["anchor_Fy_N"]), 0.0, "half anchor");
        close(row_value(&v, "constant_effort_support_applied_load", "support:ce", None, None, "combination", "comb:half"), num(&half["ce_applied_N"]), 0.0, "half CE applied");
        assert_eq!(gate(&v, "comb:over")["reason"], "CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE");
        assert!(combination_rows(&v, "comb:over").is_empty());
        let sub = &ce["subtraction_W1_minus_W2"];
        close(support_action(&v, "support:a", "combination", "comb:sub").unwrap()[1], num(&sub["anchor_Fy_N"]), 0.0, "sub anchor");
        close(row_value(&v, "constant_effort_support_applied_load", "support:ce", None, None, "combination", "comb:sub"), 0.0, num(&sub["ce_applied_N"]["zero_scale"]), "sub CE applied");
        // Non-consuming variant.
        let v = solved(&cantilever(vec![ce_support(&[])], json!([case("case:W1", json!([force("w1", "node:b", "global_y", -1000.0)]))]), json!([])), mode);
        let nc = &ce["non_consuming_variant"];
        assert!(support_action(&v, "support:ce", "load_case", "case:W1").is_none());
        assert_eq!(v["contract_evidence"]["preview_cases"][0]["support_attribution"]["withheld"][0], json!({"support_id":"support:ce","reason":"CONSTANT_EFFORT_NOT_CONSUMED"}));
        let e: Vec<f64> = vec3(&nc["anchor_F_W1"]).into_iter().chain(vec3(&nc["anchor_M_W1"])).collect();
        close6(case_action(&v, "support:a", "case:W1"), &e, NO_FROZEN_SCALE, NO_FROZEN_SCALE, "non-consuming anchor");
        assert_eq!(diag(&v, "CONSTANT_EFFORT_NOT_CONSUMED").len(), 1);
    }
}

#[test]
fn nonlinear_mechanics_combinations_are_withheld_subtraction_kept() {
    let nl = r2()["REF-NL-C3"].clone();
    let mut stop = nl["gap_support_json"].clone();
    stop["id"] = json!("support:stop");
    stop["node"] = json!("node:b");
    stop["provenance"] = json!("t0r");
    let wire = cantilever(
        vec![stop],
        json!([case("case:P1", json!([force("p1", "node:b", "global_y", 800.0)])), case("case:P2", json!([force("p2", "node:b", "global_y", 800.0)]))]),
        json!([mechanics("comb:sum", &[("case:P1", 1.0), ("case:P2", 1.0)]), subtraction("comb:diff", "case:P1", "case:P2")]),
    );
    for mode in MODES {
        let v = solved(&wire, mode);
        assert_eq!(gate(&v, "comb:sum"), json!({"combination_id":"comb:sum","withheld":true,"reason":"NONLINEAR_COMBINATION_REQUIRES_SOLVE"}));
        assert!(combination_rows(&v, "comb:sum").is_empty(), "no superposed row may penetrate the stop");
        assert_eq!(diag(&v, "NONLINEAR_COMBINATION_REQUIRES_SOLVE").len(), 1);
        for case_id in ["case:P1", "case:P2"] {
            close(row_value(&v, "global_nodal_displacement_y", "node:b", None, None, "load_case", case_id), num(&nl["case_tip_uy_m"]) * 1000.0, 0.0, "case uy");
            close(case_action(&v, "support:stop", case_id)[1], 0.0, num(&nl["case_stop_force_N"]["zero_scale"]), "stop force");
        }
        assert_eq!(gate(&v, "comb:diff")["withheld"], false);
        assert!(!combination_rows(&v, "comb:diff").is_empty());
        assert!(!combination_rows(&v, "comb:diff").iter().any(|r| r["kind"].as_str().unwrap().starts_with("nonlinear")));
    }
}

#[test]
fn mixed_modulus_mechanics_combination_is_withheld() {
    let mut cases = json!([case("case:cold", json!([force("c", "node:b", "global_y", 1000.0)])), case("case:hot", json!([force("h", "node:b", "global_y", 1000.0)]))]);
    cases[1]["modulus_basis_ref"] = json!("point:hot");
    let v = solved(&cantilever(vec![], cases, json!([mechanics("comb:mix", &[("case:cold", 1.0), ("case:hot", 1.0)]), subtraction("comb:sub", "case:hot", "case:cold")])), PreviewSolverMode::SparseInteractive);
    assert_eq!(gate(&v, "comb:mix")["reason"], "COMBINATION_MODULUS_BASIS_MIXED");
    assert!(combination_rows(&v, "comb:mix").is_empty());
    assert!(!combination_rows(&v, "comb:sub").is_empty());
}

// ------------------------------------------------------------------ M08 and intensification
fn l_model(marker_modifiers: Value, cases: Value, combinations: Value) -> Value {
    let comp = json!({"id":"component:bend","label":"t0r marker","kind":"bend","node":"node:b",
        "geometry":{"bend_radius":{"value":0.2,"unit":"m"},"bend_angle":{"value":1.5707963268,"unit":"rad"},"bend_plane_orientation":"global_xy_preview","bend_geometry_source_reference":"t0r"},
        "modifiers":marker_modifiers,
        "mechanics_interface":{"solver_consumption":"mechanics_geometry_only","rule_check_consumption":"user_rule_pack_inputs_only"},"provenance":"t0r"});
    base(
        json!([node("node:a", 0.0, 0.0, 0.0), node("node:b", 1.0, 0.0, 0.0), node("node:c", 1.0, 1.0, 0.0)]),
        json!([pipe("pipe:a-b", "node:a", "node:b", [0.0, 1.0, 0.0]), pipe("pipe:b-c", "node:b", "node:c", [1.0, 0.0, 0.0])]),
        json!([anchor("node:a")]),
        json!([comp]),
        cases,
        combinations,
    )
}
fn sif(k: Option<f64>) -> Value {
    let mut m = json!({"sif_user_value":{"value":1.15,"unit":"none"},"source_reference":"t0r_invented"});
    if let Some(k) = k {
        m["flexibility_factor_user_value"] = json!({"value":k,"unit":"none"});
    }
    m
}

#[test]
fn m08_marker_intensified_measure_matches_references_and_never_combines() {
    let r = r2();
    let il = &r["REF-I-L"];
    let m08 = &refs()["REF-M08-L"];
    let cases = json!([
        case("case:down", json!([force("down", "node:c", "global_z", -1000.0)])),
        case("case:up", json!([force("up", "node:c", "global_z", 1000.0)])),
        case("case:L2", json!([force("l2x", "node:c", "global_x", 1000.0), force("l2z", "node:c", "global_z", 1000.0)])),
    ]);
    let combos = json!([mechanics("comb:updown", &[("case:up", 1.0), ("case:down", 1.0)])]);
    for mode in MODES {
        let mut previous: Option<Vec<f64>> = None;
        for k in [Some(1.08), Some(2.16), None] {
            let v = solved(&l_model(sif(k), cases.clone(), combos.clone()), mode);
            let l1 = &il["L1_load_c_Fz_1000"];
            let bc = intensified_for(&v, "pipe:b-c", "end_i", "case:up");
            let ab = intensified_for(&v, "pipe:a-b", "end_j", "case:up");
            assert_eq!((bc.len(), ab.len()), (1, 1));
            close(bc[0], num(&l1["pipe_bc_end_i_Pa"]), 0.0, "L1 b-c");
            close(ab[0], 0.0, num(&l1["pipe_ab_end_j_Pa"]["zero_scale"]), "L1 a-b torsion only");
            let l2 = &il["L2_load_c_F_1000_0_1000"];
            let bc2 = intensified_for(&v, "pipe:b-c", "end_i", "case:L2")[0];
            let ab2 = intensified_for(&v, "pipe:a-b", "end_j", "case:L2")[0];
            close(bc2, num(&l2["pipe_bc_end_i_Pa"]), 0.0, "L2 b-c");
            close(ab2, num(&l2["pipe_ab_end_j_Pa"]), 0.0, "L2 a-b");
            for control in ["negative_control_abs_sum_bc_Pa", "negative_control_with_k_bc_Pa"] {
                assert!((bc2 - num(&l2[control])).abs() > 1.0, "{control}");
            }
            for control in ["negative_control_axial_added_unintensified_ab_Pa", "negative_control_axial_added_intensified_ab_Pa"] {
                assert!((ab2 - num(&l2[control])).abs() > 1.0, "{control}");
            }
            let values = vec![bc[0], ab[0], bc2, ab2];
            if let Some(p) = &previous {
                assert_eq!(p, &values, "k must not change the intensified measure");
            }
            previous = Some(values);
            // Never combined; the up+down combination is zero everywhere.
            assert!(combination_rows(&v, "comb:updown").iter().all(|r| r["kind"] != "component_equal_factor_intensified_bending_stress_v1"));
            assert_eq!(diag(&v, "COMBINATION_INTENSIFIED_STRESS_UNAVAILABLE").len(), 1);
            let zs = &r["zero_scales_rev1"]["REF-M08-L.combination_zero_rows"];
            for row in combination_rows(&v, "comb:updown") {
                let scale = match row["unit"].as_str().unwrap() { "N" => num(&zs["force_N"]), "N*m" => num(&zs["moment_Nm"]), "MPa" => num(&zs["stress_Pa"]) / 1e6, _ => 1.0 };
                close(num(&row["value"]), 0.0, scale, &format!("up+down {}", row["id"]));
            }
            // Per-case maximum and headline under the tie rule.
            close(maximum(&v, "pipe:a-b", "case:up").unwrap(), num(&m08["per_case_circular_normal_max_Pa"]), 0.0, "M08 a-b max");
            assert_eq!(v["summary"]["component_stress_modifier_count"], json!(6));
            assert_eq!(diag(&v, "COMPONENT_EQUAL_FACTOR_INTENSIFICATION_APPLIED").len(), 6);
        }
    }
    // No SIF: no row and an info diagnostic, no default factor.
    let v = solved(&l_model(json!({"flexibility_factor_user_value":{"value":1.08,"unit":"none"},"source_reference":"t0r"}), json!([case("case:up", json!([force("up", "node:c", "global_z", 1000.0)]))]), json!([])), PreviewSolverMode::SparseInteractive);
    assert!(intensified(&v, "component:bend", "end_i", "case:up").is_empty() && intensified(&v, "component:bend", "end_j", "case:up").is_empty());
    assert_eq!(diag(&v, "COMPONENT_INTENSIFICATION_SIF_NOT_SUPPLIED").len(), 1);
    assert_eq!(v["summary"]["component_stress_modifier_count"], json!(0));
}

#[test]
fn m08_headline_tie_rule() {
    let m08 = refs()["REF-M08-L"].clone();
    for mode in MODES {
        let v = solved(&l_model(sif(Some(1.08)), json!([case("case:up", json!([force("up", "node:c", "global_z", 1000.0)]))]), json!([])), mode);
        let ab = maximum(&v, "pipe:a-b", "case:up").unwrap();
        let bc = maximum(&v, "pipe:b-c", "case:up").unwrap();
        let h = &v["summary"]["max_open_formula_stress"];
        close(num(&h["value"]), num(&m08["headline_Pa"]), 0.0, "M08 headline");
        if ab.to_bits() == bc.to_bits() {
            assert_eq!(h["location_ref"], "pipe:a-b", "identity tie-break");
        } else {
            close(ab, num(&m08["headline_Pa"]), 0.0, "tie a-b");
            close(bc, num(&m08["headline_Pa"]), 0.0, "tie b-c");
            println!("REF-M08-L ({mode:?}): tie not exercised ({ab:e} vs {bc:e})");
        }
    }
}

#[test]
fn m08_branch_intensified_sides_and_coverage() {
    let it = r2()["REF-I-T"].clone();
    let tee = json!({"id":"component:tee","label":"t0r tee","kind":"branch","node":"node:b",
        "geometry":{"branch_header_pipe_ref":"pipe:a-b","branch_branch_pipe_ref":"pipe:b-c","branch_geometry_source_reference":"t0r"},
        "modifiers":{"branch_header_sif_user_value":{"value":1.3,"unit":"none"},"branch_branch_sif_user_value":{"value":2.0,"unit":"none"},"source_reference":"t0r_invented"},
        "mechanics_interface":{"solver_consumption":"mechanics_geometry_only","rule_check_consumption":"user_rule_pack_inputs_only"},"provenance":"t0r"});
    let wire = base(
        json!([node("node:a", 0.0, 0.0, 0.0), node("node:b", 1.0, 0.0, 0.0), node("node:c", 1.0, 1.0, 0.0), node("node:e", 2.0, 0.0, 0.0)]),
        json!([pipe("pipe:a-b", "node:a", "node:b", [0.0, 1.0, 0.0]), pipe("pipe:b-c", "node:b", "node:c", [1.0, 0.0, 0.0]), pipe("pipe:b-e", "node:b", "node:e", [0.0, 1.0, 0.0])]),
        json!([anchor("node:a")]),
        json!([tee]),
        json!([case("case:t", json!([force("cx", "node:c", "global_x", 800.0), force("cz", "node:c", "global_z", 1000.0), force("ez", "node:e", "global_z", 500.0)]))]),
        json!([]),
    );
    for mode in MODES {
        let v = solved(&wire, mode);
        let header = intensified_for(&v, "pipe:a-b", "end_j", "case:t");
        let branch = intensified_for(&v, "pipe:b-c", "end_i", "case:t");
        assert_eq!((header.len(), branch.len()), (1, 1));
        close(header[0], num(&it["header_ab_end_j_Pa"]), 0.0, "header");
        close(branch[0], num(&it["branch_bc_end_i_Pa"]), 0.0, "branch");
        assert!((header[0] - num(&it["negative_control_header_equals_unreferenced_be_Pa"])).abs() > 1.0);
        assert!(intensified_for(&v, "pipe:b-e", "end_i", "case:t").is_empty());
        let cov = diag(&v, "COMPONENT_INTENSIFIED_COVERAGE_INCOMPLETE");
        assert_eq!(cov.len(), 1);
        assert!(cov[0]["affected_refs"].as_array().unwrap().contains(&json!("pipe:b-e")));
    }
}

// ------------------------------------------------------------------ arcs
fn arc_model(k: f64, yref_bc: [f64; 3], sif_value: Option<f64>) -> Value {
    let mut modifiers = json!({"flexibility_factor_user_value":{"value":k,"unit":"none"},"source_reference":"t0r_invented"});
    if let Some(s) = sif_value {
        modifiers["sif_user_value"] = json!({"value":s,"unit":"none"});
    }
    let comp = json!({"id":"component:bend","label":"t0r arc","kind":"bend","node":"node:c",
        "geometry":{"bend_pipe_ref":"pipe:b-c","bend_radius":{"value":0.2,"unit":"m"},"bend_angle":{"value":std::f64::consts::FRAC_PI_2,"unit":"rad"},"bend_plane_orientation":"global_xy_preview","bend_geometry_source_reference":"t0r"},
        "modifiers":modifiers,
        "mechanics_interface":{"solver_consumption":"curved_bend_macro_element","rule_check_consumption":"user_rule_pack_inputs_only"},"provenance":"t0r"});
    base(
        json!([node("node:a", 0.0, 0.0, 0.0), node("node:b", 1.0, 0.0, 0.0), node("node:c", 1.2, 0.2, 0.0), node("node:d", 1.2, 1.2, 0.0)]),
        json!([pipe("pipe:a-b", "node:a", "node:b", [0.0, 1.0, 0.0]), pipe("pipe:b-c", "node:b", "node:c", yref_bc), pipe("pipe:c-d", "node:c", "node:d", [1.0, 0.0, 0.0])]),
        json!([anchor("node:a")]),
        json!([comp]),
        json!([case("case:z", json!([force("z", "node:d", "global_z", 1000.0)]))]),
        json!([]),
    )
}

#[test]
fn b1_arc_signed_rows_frames_and_withheld_maximum() {
    let r = r2();
    let b1 = &r["REF-B1-SIGNED"];
    let (fs, ms) = (num(&b1["zero_scale"]["force_N"]), num(&b1["zero_scale"]["moment_Nm"]));
    let yref = [SQRT1_2, -SQRT1_2, 0.0];
    for mode in MODES {
        for k in [2.0, 4.0] {
            let v = solved(&arc_model(k, yref, Some(1.3)), mode);
            let names = [("axial_force", "N"), ("shear_force_y", "Vy"), ("shear_force_z", "Vz"), ("torsional_moment", "T"), ("bending_moment_y", "My"), ("bending_moment_z", "Mz")];
            let kinds = ["element_local_axial_force", "element_local_shear_force_y", "element_local_shear_force_z", "element_local_torsional_moment", "element_local_bending_moment_y", "element_local_bending_moment_z"];
            for station in ["quarter_1", "midspan", "quarter_3"] {
                for (idx, (component, key)) in names.iter().enumerate() {
                    let value = row_value(&v, kinds[idx], "pipe:b-c", Some(component), Some(station), "load_case", "case:z");
                    close(value, num(&b1["stations"][station][key]), if idx < 3 { fs } else { ms }, &format!("{station} {key}"));
                }
            }
            for (location, key) in [("end_i", "product_end_i_rows_node_on_element"), ("end_j", "product_end_j_rows_node_on_element")] {
                let expected = vec3(&b1["chord_frame"][key]);
                for (idx, (component, _)) in names.iter().enumerate() {
                    let row = results(&v).iter().find(|r| r["kind"] == kinds[idx] && r["entity_ref"] == "pipe:b-c" && r["metadata"]["location"] == location && r["metadata"]["component"] == *component).unwrap();
                    assert_eq!(row["metadata"]["coordinate_system"], "arc_chord_frame");
                    close(num(&row["value"]), expected[idx], if idx < 3 { fs } else { ms }, &format!("chord {location} {component}"));
                }
            }
            let es = &b1["endpoint_stress_rows_tangent_frame"];
            let stress = |kind: &str, location: &str| {
                let row = results(&v).iter().find(|r| r["kind"] == kind && r["entity_ref"] == "pipe:b-c" && r["metadata"]["location"] == location).unwrap();
                assert_eq!(row["metadata"]["basis"], "nominal_straight_beam_formula_on_arc_resultants");
                num(&row["value"]) * 1e6
            };
            let zs = num(&es["zero_scale_Pa"]);
            close(stress("element_local_bending_normal_stress_y", "end_i"), num(&es["end_i_bending_normal_y_Pa"]), zs, "end_i sigma_y");
            close(stress("element_local_torsional_shear_stress", "end_i"), num(&es["end_i_torsional_shear_Pa"]), zs, "end_i tau");
            close(stress("element_local_bending_normal_stress_y", "end_j"), num(&es["end_j_bending_normal_y_Pa"]), zs, "end_j sigma_y");
            assert!(maximum(&v, "pipe:b-c", "case:z").is_none(), "no arc maximum");
            assert!(v["summary"]["max_open_formula_stress"].is_null(), "no headline with an arc");
            assert_eq!(v["contract_evidence"]["preview_cases"][0]["stress_maximum_coverage"]["outside_domain_pipe_ids"], json!(["pipe:b-c"]));
            assert_eq!(diag(&v, "PREVIEW_STRESS_HEADLINE_WITHHELD").len(), 1);
            assert!(!results(&v).iter().any(|r| r["kind"] == "component_equal_factor_intensified_bending_stress_v1"), "NOTE-1");
            assert_eq!(diag(&v, "COMPONENT_STRESS_INTENSIFICATION_NOT_APPLIED").len(), 1);
            assert!(diag(&v, "CURVED_BEND_TANGENT_DISCONTINUITY").is_empty(), "consistent arc has no warning");
            let anchor = &refs()["REF-B1"]["anchor"];
            let e: Vec<f64> = vec3(&anchor["F"]).into_iter().chain(vec3(&anchor["M"])).collect();
            close6(case_action(&v, "support:a", "case:z"), &e, fs, ms, "B1 anchor");
        }
    }
    // Kinked arc: warning at both ends, solve still published.
    let kinked = r["REF-B1-TANGENCY"]["cases"]["kinked_y_ref_0_0_1"].clone();
    let v = solved(&arc_model(2.0, [0.0, 0.0, 1.0], None), PreviewSolverMode::SparseInteractive);
    let warnings = diag(&v, "CURVED_BEND_TANGENT_DISCONTINUITY");
    assert_eq!(warnings.len(), 2, "{warnings:?}");
    for (w, key) in warnings.iter().zip(["angle_at_b_deg", "angle_at_c_deg"]) {
        let text = w["message"].as_str().unwrap();
        let degrees: f64 = text.split(" rad (").nth(1).unwrap().split(' ').next().unwrap().parse().unwrap();
        close(degrees, num(&kinked[key]), NO_FROZEN_SCALE, "kink angle");
    }
}

#[test]
fn b2_indeterminate_arc_resultants_depend_on_k() {
    let b2 = r2()["REF-B2"].clone();
    let comp = |k: f64| json!({"id":"component:bend","label":"t0r arc","kind":"bend","node":"node:c",
        "geometry":{"bend_pipe_ref":"pipe:b-c","bend_radius":{"value":0.2,"unit":"m"},"bend_angle":{"value":std::f64::consts::FRAC_PI_2,"unit":"rad"},"bend_plane_orientation":"global_xy_preview","bend_geometry_source_reference":"t0r"},
        "modifiers":{"flexibility_factor_user_value":{"value":k,"unit":"none"},"source_reference":"t0r_invented"},
        "mechanics_interface":{"solver_consumption":"curved_bend_macro_element","rule_check_consumption":"user_rule_pack_inputs_only"},"provenance":"t0r"});
    for (k, key) in [(1.0, "k1"), (2.0, "k2"), (4.0, "k4")] {
        let wire = base(
            json!([node("node:b", 1.0, 0.0, 0.0), node("node:c", 1.2, 0.2, 0.0)]),
            json!([pipe("pipe:b-c", "node:b", "node:c", [SQRT1_2, -SQRT1_2, 0.0])]),
            json!([anchor("node:b"), support("support:roller", "node:c", &["UX"])]),
            json!([comp(k)]),
            json!([case("case:y", json!([force("y", "node:c", "global_y", 1000.0)]))]),
            json!([]),
        );
        let v = solve(&wire, PreviewSolverMode::SparseInteractive);
        if v["status"]["mechanics"] != "MECHANICS_SOLVED" {
            println!("REF-B2 k={k}: a lone arc span is refused: {:?}", codes(&v));
            assert!(results(&v).is_empty());
            continue;
        }
        assert_contract(&v);
        println!("REF-B2 k={k}: a lone arc span is accepted");
        let expected = &b2[key];
        close(case_action(&v, "support:roller", "case:y")[0], num(&expected["roller_on_pipe_Fx_N"]), NO_FROZEN_SCALE, "roller");
        close6(case_action(&v, "support:a", "case:y"), &vec3(&expected["anchor_on_pipe"]), NO_FROZEN_SCALE, NO_FROZEN_SCALE, "B2 anchor");
        let rows = vec3(&expected["product_end_i_rows_node_on_element"]);
        let kinds = ["element_local_axial_force", "element_local_shear_force_y", "element_local_shear_force_z", "element_local_torsional_moment", "element_local_bending_moment_y", "element_local_bending_moment_z"];
        for (idx, kind) in kinds.iter().enumerate() {
            close(row_value(&v, kind, "pipe:b-c", None, Some("end_i"), "load_case", "case:y"), rows[idx], NO_FROZEN_SCALE, &format!("B2 end_i {kind}"));
        }
    }
}

// ------------------------------------------------------------------ ties, review and completeness
#[test]
fn m33_displacement_tie_rule() {
    let tie = r2()["REF-M33-TIE"].clone();
    for mode in MODES {
        let mut locations = Vec::new();
        for reverse in [false, true] {
            let mut nodes = vec![node("node:o", 0.0, 0.0, 0.0), node("node:p", 1.0, 0.0, 0.0), node("node:q", -1.0, 0.0, 0.0)];
            let mut pipes = vec![pipe("pipe:o-p", "node:o", "node:p", [0.0, 1.0, 0.0]), pipe("pipe:o-q", "node:o", "node:q", [0.0, 1.0, 0.0])];
            if reverse {
                nodes.swap(1, 2);
                pipes.reverse();
            }
            let v = solved(&base(json!(nodes), json!(pipes), json!([anchor("node:o")]), json!([]), json!([case("case:tie", json!([force("p", "node:p", "global_z", 1000.0), force("q", "node:q", "global_z", 1000.0)]))]), json!([])), mode);
            let p = row_value(&v, "displacement_magnitude", "node:p", None, None, "load_case", "case:tie");
            let q = row_value(&v, "displacement_magnitude", "node:q", None, None, "load_case", "case:tie");
            close(num(&v["summary"]["max_displacement"]["value"]), num(&tie["max_displacement_mm"]), 0.0, "tie value");
            locations.push((p.to_bits() == q.to_bits(), v["summary"]["max_displacement"]["location_ref"].clone(), p, q));
        }
        if locations.iter().all(|l| l.0) {
            assert_eq!(locations[0].1, json!("node:p"));
            assert_eq!(locations[1].1, json!("node:p"));
        } else {
            for l in &locations {
                close(l.2, num(&tie["max_displacement_mm"]), 0.0, "p");
                close(l.3, num(&tie["max_displacement_mm"]), 0.0, "q");
            }
            println!("REF-M33-TIE ({mode:?}): tie not exercised {locations:?}");
        }
    }
}

#[test]
fn high_displacement_review_uses_the_all_case_headline() {
    // A long slender cantilever: case 1 is small, case 2 exceeds the 5 mm review value.
    let wire = base(
        json!([node("node:a", 0.0, 0.0, 0.0), node("node:b", 3.0, 0.0, 0.0)]),
        json!([pipe_od("pipe:a-b", "node:a", "node:b", [0.0, 1.0, 0.0], 0.06, 0.005)]),
        json!([anchor("node:a")]),
        json!([]),
        json!([case("case:small", json!([force("s", "node:b", "global_y", 1.0)])), case("case:large", json!([force("l", "node:b", "global_y", 5000.0)]))]),
        json!([]),
    );
    let v = solved(&wire, PreviewSolverMode::SparseInteractive);
    let review = diag(&v, "HIGH_DISPLACEMENT_REVIEW");
    assert_eq!(review.len(), 1);
    assert_eq!(review[0]["affected_refs"][0], v["summary"]["max_displacement"]["result_ref"]);
    assert!(v["summary"]["max_displacement"]["result_ref"].as_str().unwrap().contains("case-large"));
}

#[test]
fn imposed_displacement_on_a_support_is_refused_with_a_targeted_diagnostic() {
    let load = json!({"id":"load:imp","category":"imposed_displacement","target":{"type":"support","support":"support:a","dof":"UY"},
        "direction":"UY","magnitude":{"value":-0.006,"unit":"m"},"dimension":"displacement","provenance":"t0r"});
    let wire = cantilever(vec![], json!([case("case:imp", json!([load, force("f", "node:b", "global_y", 100.0)]))]), json!([]));
    let v = solve(&wire, PreviewSolverMode::SparseInteractive);
    assert_eq!(v["status"]["mechanics"], "MODEL_INCOMPLETE");
    assert!(results(&v).is_empty(), "no envelope may solve without the load");
    let d = diag(&v, "IMPOSED_DISPLACEMENT_REQUIRES_LOAD_STATE_MODEL");
    assert_eq!(d.len(), 1);
    assert_eq!(d[0]["severity"], "blocking");
    assert_eq!(d[0]["affected_refs"], json!(["load:imp", "case:imp", "support:a"]));
    assert_eq!(v["contract_evidence"], json!({"preview_cases": [], "combination_gates": []}));
    assert!(v["summary"]["max_open_formula_stress"].is_null());
}

#[test]
fn blocked_envelope_carries_empty_preview_evidence() {
    let mut wire = cantilever(vec![], json!([case("case:x", json!([force("f", "node:b", "global_y", 100.0)]))]), json!([]));
    wire["model"]["supports"] = json!([]);
    let v = solve(&wire, PreviewSolverMode::SparseInteractive);
    assert_eq!(v["status"]["mechanics"], "MODEL_INCOMPLETE");
    assert_eq!(v["contract_evidence"], json!({"preview_cases": [], "combination_gates": []}));
    assert_eq!(v["formulation_basis"]["profile_id"], "product_preview_mechanics_v1");
    assert_eq!(v["formulation_basis"]["limitations"].as_array().unwrap().len(), 7);
}

fn invented_demo_without_pressure() -> Value {
    let mut model: Value = serde_json::from_str(include_str!("../../../fixtures/product_preview/invented_preview_model.json")).unwrap();
    // The demo's legacy nonzero pressure is refused on this route; drop it.
    for c in model["load_cases"].as_array_mut().unwrap() {
        c["primitive_loads"].as_array_mut().unwrap().retain(|l| l["category"] != "pressure" && l["dimension"] != "pressure");
    }
    model
}

/// M07 containment (ROOT ruling on R1 N-1). Evidence, from hand statics rather
/// than product output: joint C-150 couples its two nodes, 2.2 m apart, only
/// through relative lateral springs k = 900000 N/m. A relative lateral
/// displacement then produces equal and opposite forces 2.2 m apart with no
/// balancing couple, so the element is not in moment equilibrium; in the
/// invented demo's L-100 that unbalanced couple is k*du*L = 900000 * 3.3254e-4 m
/// * 2.2 m = 658.44 N*m about Y. The ordinary route therefore refuses the solve.
#[test]
fn joint_element_without_moment_coupling_is_refused() {
    let model = invented_demo_without_pressure();
    for mode in MODES {
        let v = solve(&json!({"model": model.clone(), "materials": []}), mode);
        assert_eq!(v["status"]["mechanics"], "MODEL_INCOMPLETE");
        assert!(results(&v).is_empty());
        let d = diag(&v, "JOINT_ELEMENT_EQUILIBRIUM_UNQUALIFIED");
        assert_eq!(d.len(), 1);
        assert_eq!(d[0]["severity"], "blocking");
        assert_eq!(d[0]["affected_refs"], json!(["component:C-150", "pipe:P-130"]));
        assert_eq!(v["contract_evidence"], json!({"preview_cases": [], "combination_gates": []}));
    }
    // Boundaries: axial/rotational-only stiffness, or a zero lateral value, is not refused.
    let joint = |lateral: f64| json!({"id":"component:joint","label":"t0r joint","kind":"expansion_joint","node":"node:b",
        "geometry":{"expansion_joint_pipe_ref":"pipe:b-c","effective_area":{"value":0.01,"unit":"m^2"},"expansion_joint_source_reference":"t0r"},
        "modifiers":{"axial_stiffness_user_value":{"value":3.2e6,"unit":"N/m"},"lateral_stiffness_user_value":{"value":lateral,"unit":"N/m"},
            "angular_stiffness_user_value":{"value":4.8e5,"unit":"N*m/rad"},"torsional_stiffness_user_value":{"value":6.2e5,"unit":"N*m/rad"},"source_reference":"t0r_invented"},
        "mechanics_interface":{"solver_consumption":"mechanics_geometry_and_user_flexibility","rule_check_consumption":"user_rule_pack_inputs_only"},"provenance":"t0r"});
    let wire = |lateral: f64| base(
        json!([node("node:a", 0.0, 0.0, 0.0), node("node:b", 1.0, 0.0, 0.0), node("node:c", 1.5, 0.0, 0.0)]),
        json!([pipe("pipe:a-b", "node:a", "node:b", [0.0, 1.0, 0.0]), pipe("pipe:b-c", "node:b", "node:c", [0.0, 1.0, 0.0])]),
        json!([anchor("node:a"), support("support:c", "node:c", &["UX", "UY", "UZ", "RX", "RY", "RZ"])]),
        json!([joint(lateral)]),
        json!([case("case:j", json!([force("f", "node:b", "global_y", 100.0)]))]),
        json!([]),
    );
    let refused = solve(&wire(9e5), PreviewSolverMode::SparseInteractive);
    assert_eq!(diag(&refused, "JOINT_ELEMENT_EQUILIBRIUM_UNQUALIFIED").len(), 1);
    let axial_rotational = solve(&wire(0.0), PreviewSolverMode::SparseInteractive);
    assert!(diag(&axial_rotational, "JOINT_ELEMENT_EQUILIBRIUM_UNQUALIFIED").is_empty(), "{}", axial_rotational["diagnostics"]);
    // Zero length: coincident joint nodes are never refused by this rule.
    let mut zero = wire(9e5);
    zero["model"]["nodes"][2]["position"]["x"] = json!(1.0);
    let zero = solve(&zero, PreviewSolverMode::SparseInteractive);
    assert!(diag(&zero, "JOINT_ELEMENT_EQUILIBRIUM_UNQUALIFIED").is_empty());
}

/// R1 SF-1: a case that blocks after earlier cases pushed row-naming
/// diagnostics must still give a blocked envelope that satisfies S1 §9.
#[test]
fn blocked_envelope_after_a_later_case_fails_keeps_its_reason_and_no_dangling_refs() {
    let gap = |id: &str, g: f64| json!({"id":id,"node":"node:c","family":"nonlinear","restraints":[],"provenance":"t0r",
        "nonlinear":{"behavior":"gap","dof":"UZ","initial_state":"inactive","closes_when":"negative_displacement","gap":{"value":g,"unit":"m"}}});
    let mut wire = l_model(sif(Some(1.08)), json!([
        case("case:small", json!([force("s", "node:c", "global_z", -1.0)])),
        case("case:large", json!([force("l", "node:c", "global_z", -50000.0)])),
    ]), json!([]));
    wire["model"]["supports"].as_array_mut().unwrap().extend([gap("support:g1", 0.0001), gap("support:g2", 0.0002)]);
    for mode in MODES {
        let v = solve(&wire, mode);
        assert_eq!(v["status"]["mechanics"], "MODEL_INCOMPLETE");
        assert!(results(&v).is_empty());
        assert_eq!(v["contract_evidence"], json!({"preview_cases": [], "combination_gates": []}));
        let all = v["diagnostics"].as_array().unwrap();
        assert!(all.iter().any(|d| d["severity"] == "blocking"), "blocking reason kept");
        for d in all {
            assert!(!["COMPONENT_STRESS_MULTIPLIER_APPLIED", "COMBINATION_STRESS_SUMMARY_SKIPPED"].contains(&d["code"].as_str().unwrap()));
            assert!(d["affected_refs"].as_array().into_iter().flatten().all(|r| !r.as_str().unwrap().starts_with("result:")), "{d}");
        }
    }
}

#[test]
fn invented_demo_model_renders_completely() {
    // The derived demo: legacy pressure removed and the refused joint C-150 removed.
    let model: Value = serde_json::from_str(include_str!("fixtures/preview_physics_invented_model.json")).unwrap();
    for mode in MODES {
        let v = solved(&json!({"model": model.clone(), "materials": []}), mode);
        // Non-result reference classes are accepted (F-1 positive control).
        let refs: Vec<&str> = v["diagnostics"].as_array().unwrap().iter().flat_map(|d| d["affected_refs"].as_array().into_iter().flatten()).filter_map(|r| r.as_str()).collect();
        assert!(refs.contains(&"hanger") || refs.iter().any(|r| r.contains("DEC-") || r.starts_with("invented_")), "{refs:?}");
        assert_eq!(gate(&v, "combination:C-OPER-ALT")["reason"], "NONLINEAR_COMBINATION_REQUIRES_SOLVE");
        assert!(combination_rows(&v, "combination:C-OPER-ALT").is_empty());
    }
}
