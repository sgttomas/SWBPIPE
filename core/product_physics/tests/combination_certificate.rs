//! RT-A3 §4: the combination certificate (DEL-05-02 Work `combination-certificate.md`;
//! `operating-case.md` §4, Agent 0's eligibility decision (b) of 2026-11-04).
//!
//! The expectations are the frozen independent references of
//! `validation/references/combination_certificate/` (exact rationals, independent π;
//! no product code): the operand rows each model publishes, the verdict classes the
//! hand analysis decides (cancellation by publication gaps, exactly-zero differences
//! below the floor, D-6's equal-pressure statement, gap operands not assessed) and
//! op1a V-2's confirmed nonlinear subtraction. B_D is re-formed here from the
//! published rows and operand bounds by this test's own error-free transformations
//! (TwoSum, FMA), never by product code. The enclosure of the exact intended values
//! (| |v_D| − |D| | <= B_D) is checked in exact rationals by the Python consumer
//! (`tests/test_combination_certificate.py`) over the shared corpus this test writes.
//! Every model runs in both solver modes.
use open_pipe_stress_product_physics::{result_export::reader_verdict, run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

#[path = "common/current_model.rs"]
mod current_model;

const PROV: &str = "rt-a3 combination certificate invented benchmark input; not library, component, material-catalog or code data";

fn qv(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}
fn force(id: &str, direction: &str, f: f64) -> Value {
    json!({"id": id, "category": "concentrated_force", "target": {"type": "node", "node": "node:b"}, "direction": direction,
        "magnitude": qv(f, "N"), "dimension": "force", "provenance": PROV})
}
fn subtraction(id: &str, a: &str, b: &str) -> Value {
    json!({"id": id, "basis": "result_state_subtraction", "minuend_id": a, "subtrahend_id": b, "provenance": PROV})
}

/// The cantilever: node:a anchored at the origin, node:b at (L, 0, 0); tip forces (Fx, Fy).
fn cantilever(exact: bool, cases: &[(&str, f64, f64)], combinations: Value) -> Value {
    let l = 0.3;
    let terminals: Vec<Value> = ["node:a", "node:b"].iter().map(|n| json!({"node_ref": n, "closure_transfer": "transfers_to_wall", "provenance": PROV})).collect();
    let region = json!([{"id": "region:m", "member_pipe_ids": ["pipe:m"], "pressure_basis": "internal_differential_zero_external_v1",
        "pressure": qv(2.0e6, "Pa"), "terminals": terminals,
        "provenance": PROV}]);
    let load_cases: Vec<Value> = cases.iter().map(|(id, fx, fy)| {
        let mut case = json!({"id": id, "primitive_loads": [force(&format!("{id}:fx"), "global_x", *fx), force(&format!("{id}:fy"), "global_y", *fy)], "provenance": PROV});
        if exact { case["pressure_regions"] = region.clone(); }
        case
    }).collect();
    let mut section = json!({"outside_diameter": qv(0.2, "m"), "wall_thickness": qv(0.01, "m")});
    let mut material = json!({"id": "material:m", "elastic_modulus": qv(2.0e11, "Pa"), "shear_modulus": qv(2.0e11 / 2.6, "Pa"), "provenance": PROV});
    let mut model = json!({"schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:cert", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": [{"id": "node:a", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": PROV}, {"id": "node:b", "position": {"x": l, "y": 0.0, "z": 0.0}, "provenance": PROV}],
        "pipe_segments": [{"id": "pipe:m", "from": "node:a", "to": "node:b", "section": section.clone(), "material": "material:m", "y_reference": {"x": 0.0, "y": 0.0, "z": 1.0}, "provenance": PROV}],
        "supports": [{"id": "support:a", "node": "node:a", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV}],
        "components": [], "materials": [], "load_cases": load_cases, "combinations": combinations});
    if exact {
        section["mill_tolerance"] = qv(1.0 / 800.0, "m");
        section["corrosion_allowance"] = qv(1.0 / 500.0, "m");
        model["pipe_segments"][0]["section"] = section;
        model["pressure_contract"] = json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"});
        material = json!({"id": "material:m", "constitutive_basis": "homogeneous_isotropic_E_nu_v1", "elastic_modulus": qv(2.0e11, "Pa"), "poisson_ratio": qv(0.3, "1"), "provenance": PROV});
    }
    model["materials"] = json!([material]);
    json!({"model": model, "materials": []})
}

fn reference_cantilever(exact: bool) -> Value {
    let p = |e: i32| 2f64.powi(e);
    cantilever(exact,
        &[("case:S", 1000.0, 600.0), ("case:S2", 1000.0, 600.0), ("case:O2", 2000.0, 1200.0), ("case:Oc", 1000.0, 600.0 + p(-16)),
          ("case:Oa", 1000.0 + p(-10), 600.0), ("case:Q", -400.0, 200.0)],
        json!([subtraction("comb:double", "case:O2", "case:S"), subtraction("comb:cancel", "case:Oc", "case:S"), subtraction("comb:same", "case:S2", "case:S"),
            subtraction("comb:axial", "case:Oa", "case:S"),
            {"id": "comb:sum", "basis": "mechanics", "terms": [{"load_case": "case:S", "factor": 1.0}, {"load_case": "case:Q", "factor": 1.3}], "provenance": PROV},
            {"id": "comb:range", "basis": "range_envelope", "operand_ids": ["case:S", "case:O2"], "mode": "max", "provenance": PROV}]))
}

fn solve(wire: &Value, mode: PreviewSolverMode) -> Value {
    let envelope = run_linear_static_preview_value_with_mode(wire.clone(), mode).expect("captured entry");
    let v = serde_json::to_value(envelope).unwrap();
    assert_eq!(v["status"]["mechanics"], "MECHANICS_SOLVED", "{:#}", v["diagnostics"]);
    v
}

const W16_CORPUS: &str = include_str!("../../../fixtures/results/pressure_v3_combination_reader_corpus.json");
const OP1A_CORPUS: &str = include_str!("../../../fixtures/results/preview_physics_nonlinear_operands_corpus.json");

/// A frozen corpus model (its inputs only; the documents are regenerated by their writers).
fn corpus_model(corpus: &str, list: &str, field: &str, name: &str) -> Value {
    let doc: Value = serde_json::from_str(corpus).unwrap();
    let entry = doc[list].as_array().unwrap().iter().find(|e| e[field] == name).unwrap_or_else(|| panic!("no corpus model {name}"));
    json!({"model": entry["model"], "materials": []})
}

/// The W-16b line with K1 = A + 1.3B, S1 = C − A, the range envelope, and K4 = C − A − B.
fn w16_line() -> Value {
    let mut wire = corpus_model(W16_CORPUS, "cases", "label", "W16B line: mechanics, subtraction and envelope");
    let k4 = json!({"id": "comb:K4", "basis": "mechanics", "provenance": PROV,
        "terms": [{"load_case": "case:C", "factor": 1.0}, {"load_case": "case:A", "factor": -1.0}, {"load_case": "case:B", "factor": -1.0}]});
    wire["model"]["combinations"].as_array_mut().unwrap().push(k4);
    wire
}


const REFERENCES: &str = include_str!("../../../validation/references/combination_certificate/references.json");
const SUMS: &str = include_str!("../../../validation/references/combination_certificate/SHA256SUMS");
const CORPUS: &str = include_str!("../../../fixtures/results/combination_certificate_corpus.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const FLOOR: f64 = 1.0 / 17_179_869_184.0; // 2^-34
const WITHIN: &str = "COMBINATION_ROWS_WITHIN_STATED_BOUND";
const NOT_ASSESSED: &str = "COMBINATION_ROWS_NOT_ASSESSED";

fn refs() -> Value {
    serde_json::from_str(REFERENCES).unwrap()
}

#[test]
fn references_are_the_frozen_bytes() {
    let digest: String = Sha256::digest(REFERENCES.as_bytes()).iter().map(|b| format!("{b:02x}")).collect();
    let line = SUMS.lines().find(|l| l.ends_with("  references.json")).expect("SHA256SUMS lists references.json");
    assert_eq!(&line[..64], digest);
}

fn certificate(v: &Value) -> &Value {
    &v["contract_evidence"]["combination_certificate"]
}
fn record<'a>(v: &'a Value, id: &str) -> &'a Value {
    certificate(v)["combinations"].as_array().unwrap().iter().find(|r| r["combination_id"] == id).unwrap_or_else(|| panic!("no record {id}"))
}
fn group<'a>(record: &'a Value, entity: &str, location: &str, kind: &str) -> Vec<&'a Value> {
    record["groups"].as_array().unwrap().iter().filter(|g| g["entity"] == entity && g["location"] == location && g["group"] == kind).collect()
}
fn operand_group<'a>(v: &'a Value, case: &str, entity: &str, location: &str, kind: &str) -> &'a Value {
    certificate(v)["operand_cases"].as_array().unwrap().iter().find(|c| c["load_case_id"] == case).unwrap()["groups"].as_array().unwrap()
        .iter().find(|g| g["entity"] == entity && g["location"] == location && g["group"] == kind).unwrap()
}
fn row<'a>(v: &'a Value, id: &str) -> &'a Value {
    v["results"].as_array().unwrap().iter().find(|r| r["id"] == id).unwrap_or_else(|| panic!("no row {id}"))
}
fn f(v: &Value) -> f64 {
    v.as_f64().unwrap_or(f64::INFINITY)
}

// ------------------------------------------------------------------ this test's own upward arithmetic
fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let s = a + b;
    let bb = s - a;
    (s, (a - (s - bb)) + (b - bb))
}
fn ru_add(a: f64, b: f64) -> f64 {
    let (s, e) = two_sum(a, b);
    if !s.is_finite() { f64::INFINITY } else if e == 0.0 { s } else { s.next_up() }
}
fn ru_mul(a: f64, b: f64) -> f64 {
    let p = a * b;
    if !p.is_finite() { f64::INFINITY } else if a.mul_add(b, -p) == 0.0 { p } else { p.next_up() }
}
fn ru_sqrt(x: f64) -> f64 {
    let r = x.sqrt();
    if r.mul_add(r, -x) == 0.0 { r } else { r.next_up() }
}
fn ru_norm(values: &[f64]) -> f64 {
    ru_sqrt(values.iter().fold(0.0, |s, v| ru_add(s, ru_mul(v.abs(), v.abs()))))
}

/// B_D of a subtraction group from its published rows and the operands' published
/// bounds: TwoSum gives each component's exact rounding error.
fn subtraction_bound(v: &Value, minuend: &str, subtrahend: &str, g: &Value) -> f64 {
    let (entity, location, kind) = (g["entity"].as_str().unwrap(), g["location"].as_str().unwrap(), g["group"].as_str().unwrap());
    let (o, s) = (operand_group(v, minuend, entity, location, kind), operand_group(v, subtrahend, entity, location, kind));
    let mut errors = Vec::new();
    for id in g["rows"].as_array().unwrap() {
        let r = row(v, id.as_str().unwrap());
        let sources = r["source_result_refs"].as_array().unwrap();
        let (a, b) = (f(&row(v, sources[0].as_str().unwrap())["value"]), f(&row(v, sources[1].as_str().unwrap())["value"]));
        let (s, e) = two_sum(a, -b);
        assert_eq!(s.to_bits(), f(&r["value"]).to_bits(), "a subtraction row is the rounded difference");
        errors.push(-e);
    }
    ru_add(ru_add(f(&o["bound"]), f(&s["bound"])), ru_norm(&errors))
}

fn verdict_of(g: &Value) -> (String, Value) {
    (g["verdict"].as_str().unwrap().to_string(), g["reason"].clone())
}

// ------------------------------------------------------------------ the cantilever
fn check_cantilever(exact: bool, v: &Value, what: &str) {
    let route = if exact { "cantilever_exact" } else { "cantilever_preview" };
    let reference = refs();
    for c in reference[route]["combinations"].as_array().unwrap() {
        let id = c["combination_id"].as_str().unwrap();
        let rec = record(v, id);
        assert_eq!(rec["assessment"], "assessed", "{what} {id}");
        for expected in c["groups"].as_array().unwrap() {
            let (entity, location, kind) = (expected["entity"].as_str().unwrap(), expected["location"].as_str().unwrap(), expected["group"].as_str().unwrap());
            let found = group(rec, entity, location, kind);
            assert!(!found.is_empty(), "{what} {id}: a derived group {entity} {location} {kind}");
            // The operands publish exactly the reference's binary64 rows.
            if let Some(published) = expected.get("operand_published") {
                let (m, s) = (c["minuend"].as_str().unwrap(), c["subtrahend"].as_str().unwrap());
                for (case, values) in [(m, &published[0]), (s, &published[1])] {
                    let og = operand_group(v, case, entity, location, kind);
                    let set = og["rows"].as_array().unwrap().iter().find(|r| !r.is_null()).unwrap();
                    for (id, want) in set.as_array().unwrap().iter().zip(values.as_array().unwrap()) {
                        let got = f(&row(v, id.as_str().unwrap())["value"]);
                        if f(want) == 0.0 {
                            // An exactly-zero intended row may publish noise within its group's bound.
                            assert!(got.abs() <= f(&og["bound"]), "{what} {case} {id}: {got:e} within B_g");
                        } else {
                            assert_eq!(got.abs().to_bits(), f(want).abs().to_bits(), "{what} {case} {id}");
                        }
                    }
                }
            }
            for g in &found {
                if c["basis"] == "result_state_subtraction" {
                    let bound = subtraction_bound(v, c["minuend"].as_str().unwrap(), c["subtrahend"].as_str().unwrap(), g);
                    assert_eq!(bound.to_bits(), f(&g["bound"]).to_bits(), "{what} {id} {entity} {location} {kind}: B_D");
                }
                match expected["expected"]["verdict"].as_str() {
                    Some("not_assessed") => {
                        // Every intended difference is zero: below the floor when B_D < 2^-34 S*.
                        assert!(f(&g["bound"]) < FLOOR * f(&g["floor_scale"]), "{what} {id} {location} {kind}: the floor condition");
                        assert_eq!(verdict_of(g), ("not_assessed".into(), json!("below_floor")), "{what} {id} {location} {kind}");
                    }
                    Some("not_established") => {
                        assert!(f(&g["floor_scale"]) <= expected["expected"]["floor_scale_upper"].as_str().unwrap().parse::<f64>().unwrap(), "{what} {id}: S* within the reference's bound");
                        assert_eq!(verdict_of(g), ("not_established".into(), json!("bound_exceeds_criterion")), "{what} {id} {location} {kind}");
                    }
                    _ => {}
                }
                if let Some(d6) = expected.get("equal_pressure") {
                    let named = &g["equal_pressure"]["elastic_group"];
                    if d6["elastic_group"].is_null() {
                        assert!(named.is_null() && g["equal_pressure"].is_object(), "{what} {id} {location}: D-6 applies, no elastic row at the end");
                    } else {
                        for k in ["entity", "location", "group"] {
                            assert_eq!(named[k], d6["elastic_group"][k], "{what} {id} {location}: D-6's elastic group");
                        }
                        let elastic = group(rec, entity, named["location"].as_str().unwrap(), "axial");
                        let elastic = elastic.iter().find(|e| e["row_set"] == named["row_set"]).expect("the named elastic group is formed");
                        if id == "comb:axial" {
                            // D-6's demonstration: the wall group fails on P + F's publication
                            // gaps, while the elastic group establishes D(N_w) = D(N_el) = 2^-10 N.
                            assert_eq!(elastic["verdict"], "established", "{what} {id}: the elastic group establishes D(N_w)");
                            assert_eq!(f(&elastic["q"]), 2f64.powi(-10));
                        }
                    }
                } else {
                    assert!(g["equal_pressure"].is_null(), "{what} {id} {location} {kind}: D-6 only on the equal-pressure subtraction's wall groups");
                }
            }
        }
    }
    // The mechanics sum: B_D = RU(B_S + RU(1.3 B_Q) + RU‖e‖), e from an exact two-product sum.
    let rec = record(v, "comb:sum");
    let factor = 1.3_f64;
    for g in rec["groups"].as_array().unwrap().iter().filter(|g| !g["group"].as_str().unwrap().ends_with("magnitude")) {
        let (entity, location, kind) = (g["entity"].as_str().unwrap(), g["location"].as_str().unwrap(), g["group"].as_str().unwrap());
        let (bs, bq) = (f(&operand_group(v, "case:S", entity, location, kind)["bound"]), f(&operand_group(v, "case:Q", entity, location, kind)["bound"]));
        let mut errors = Vec::new();
        for id in g["rows"].as_array().unwrap() {
            let r = row(v, id.as_str().unwrap());
            let sources = r["source_result_refs"].as_array().unwrap();
            let (a, b) = (f(&row(v, sources[0].as_str().unwrap())["value"]), f(&row(v, sources[1].as_str().unwrap())["value"]));
            // a + factor*b = (a + p) + pe exactly, p = fl(factor*b), pe its FMA error.
            let p = factor * b;
            let pe = factor.mul_add(b, -p);
            let (s, se) = two_sum(a, p);
            let d = f(&r["value"]);
            // e = d - (s + se + pe), formed by two more TwoSums; the published row is the
            // once-rounded exact sum, so the error is below one ulp and this tail is exact.
            let (t, te) = two_sum(d, -s);
            let e = t - se - pe + te;
            assert_eq!((t - se - pe).to_bits(), (t - (se + pe)).to_bits(), "{what}: an exact small tail");
            errors.push(e);
        }
        let bound = ru_add(ru_add(bs, ru_mul(factor, bq)), ru_norm(&errors));
        assert_eq!(bound.to_bits(), f(&g["bound"]).to_bits(), "{what} comb:sum {entity} {location} {kind}");
    }
    assert_eq!(record(v, "comb:range")["assessment"], "not_assessed");
    assert_eq!(record(v, "comb:range")["reason"], "range-envelope certification is deferred");
}

#[test]
fn cantilever_certificate_matches_the_references() {
    for exact in [true, false] {
        let mut wire = reference_cantilever(exact);
        current_model::reauthor_model(&mut wire["model"]);
        for mode in MODES {
            let v = solve(&wire, mode);
            let what = format!("{} {mode:?}", if exact { "EX" } else { "PF" });
            check_cantilever(exact, &v, &what);
            read(&v, &wire, exact, &what);
        }
    }
}

/// The readers admit the document with and without the model.
fn read(v: &Value, wire: &Value, exact: bool, what: &str) {
    for model in [None, Some(&wire["model"])] {
        let verdict = reader_verdict::validate(if exact { "pressure" } else { "preview-physics" }, v, model, None);
        assert_eq!(verdict["ok"], true, "{what}: reader {verdict}");
    }
}

/// Agent 0's decision (b): established derived groups carry their own evidence and do
/// not count toward numerical eligibility. On the PF cancellation document, with one
/// combination whose groups are established, not established by cancellation and not
/// assessed, the document's numerical quality and the reader's standing are those of
/// the same model without combinations, and the diagnostic states the covered scope.
#[test]
fn eligibility_witness_one_cancellation_row_leaves_the_document_standing_unchanged() {
    let mut wire = cantilever(false, &[("case:S", 1000.0, 600.0), ("case:Oc", 1000.0, 600.0 + 2f64.powi(-16))], json!([subtraction("comb:cancel", "case:Oc", "case:S")]));
    current_model::reauthor_model(&mut wire["model"]);
    let mut bare = wire.clone();
    bare["model"]["combinations"] = json!([]);
    for mode in MODES {
        let (v, plain) = (solve(&wire, mode), solve(&bare, mode));
        assert_eq!(v["numerical_quality"], plain["numerical_quality"], "{mode:?}: the document's numerical quality is unchanged");
        let (with, without) = (reader_verdict::standing(&v, Some(&wire["model"]), None), reader_verdict::standing(&plain, Some(&bare["model"]), None));
        assert_eq!((with["eligible"].clone(), with["token"].clone()), (without["eligible"].clone(), without["token"].clone()), "{mode:?}: the reader's standing");
        let rec = record(&v, "comb:cancel");
        let verdicts: Vec<(String, Value)> = rec["groups"].as_array().unwrap().iter().map(verdict_of).collect();
        for want in [("established".to_string(), Value::Null), ("not_established".into(), json!("bound_exceeds_criterion")), ("not_assessed".into(), json!("below_floor"))] {
            assert!(verdicts.contains(&want), "{mode:?}: {want:?} present");
        }
        let notice = v["diagnostics"].as_array().unwrap().iter().find(|d| d["code"] == WITHIN).unwrap();
        let count = |w: &str| verdicts.iter().filter(|(v, _)| v == w).count();
        let message = notice["message"].as_str().unwrap();
        for part in [format!("{} derived row group(s) are established within their stated bound", count("established")), format!("{} are not established", count("not_established")),
            format!("{} are not assessed (below the floor)", count("not_assessed")), format!("{} of its rows are covered by no group and remain inspection results", rec["uncovered_row_count"]),
            "does not establish the meaning of \"expansion\" or engineering acceptance".into(), "do not count toward the document's numerical eligibility".into()] {
            assert!(message.contains(&part), "{mode:?}: {part} in {message}");
        }
        assert!(!message.contains("certified"), "R5: never \"certified\"");
        read(&v, &wire, false, &format!("witness {mode:?}"));
    }
}

#[test]
fn pf_confirmed_nonlinear_subtraction_is_assessed_and_gap_operands_are_not() {
    let reference = refs();
    for mode in MODES {
        let v2 = corpus_model(OP1A_CORPUS, "documents", "name", "v2_anchor_rise");
        let v = solve(&v2, mode);
        let rec = record(&v, "comb:lift");
        assert_eq!(rec["assessment"], "assessed", "{mode:?}: confirmed operands");
        for expected in reference["op1a"]["v2"]["groups"].as_array().unwrap() {
            let found = group(rec, expected["entity"].as_str().unwrap(), "node", expected["group"].as_str().unwrap());
            assert_eq!(found.len(), 1, "{mode:?} {expected}");
            assert_eq!(subtraction_bound(&v, "case:O-LIFT", "case:S", found[0]).to_bits(), f(&found[0]["bound"]).to_bits(), "{mode:?}: B_D");
            assert_eq!(found[0]["verdict"], "established", "{mode:?} {expected}");
        }
        read(&v, &v2, false, &format!("V-2 {mode:?}"));
        let v1 = corpus_model(OP1A_CORPUS, "documents", "name", "v1_gap_closes");
        let v = solve(&v1, mode);
        let rec = record(&v, "comb:o-s");
        let want = &reference["op1a"]["v1"]["expected"];
        assert_eq!((rec["assessment"].clone(), rec["reason"].clone()), (want["assessment"].clone(), want["reason"].clone()), "{mode:?}: gap operands");
        assert!(rec["groups"].as_array().unwrap().is_empty() && certificate(&v)["operand_cases"].as_array().unwrap().is_empty());
        let notice = v["diagnostics"].as_array().unwrap().iter().find(|d| d["code"] == NOT_ASSESSED && d["affected_refs"][0] == "comb:o-s").expect("named");
        assert!(notice["message"].as_str().unwrap().contains("carry no certified rows"));
        read(&v, &v1, false, &format!("V-1 {mode:?}"));
    }
}

/// Byte identity: a document without a combination publishes no member and no notice.
#[test]
fn no_combination_no_member() {
    let mut wire = cantilever(true, &[("case:S", 1000.0, 600.0)], json!([]));
    current_model::reauthor_model(&mut wire["model"]);
    let v = solve(&wire, PreviewSolverMode::SparseInteractive);
    assert!(certificate(&v).is_null());
    assert!(!v["diagnostics"].as_array().unwrap().iter().any(|d| d["id"].as_str().unwrap().starts_with("diagnostic:combination-certificate:")));
}

/// Premise: the operand bounds are each operand
/// case's own. A subtrahend held at the minuend's final support status is a different
/// system from its load case, so the combination is not assessed and binds no B_D,
/// whatever its gate says (on this base G-1 also withholds it at the solve).
fn held_cantilever() -> Value {
    let mut held = subtraction("comb:held", "case:O2", "case:S");
    held["difference_basis"] = json!("operating_minus_sustained");
    held["subtrahend_support_state"] = json!("minuend_final");
    let mut own = subtraction("comb:own", "case:O2", "case:S");
    own["difference_basis"] = json!("operating_minus_sustained");
    own["subtrahend_support_state"] = json!("own");
    let mut wire = cantilever(false, &[("case:S", 1000.0, 600.0), ("case:O2", 2000.0, 1200.0)], json!([subtraction("comb:double", "case:O2", "case:S"), held, own]));
    current_model::reauthor_model(&mut wire["model"]);
    wire
}

#[test]
fn a_held_subtrahend_is_not_assessed_and_binds_no_bound() {
    // RT-A5 (DEL-05-02 `operating-case.md` §7.5) removes S1's interim withholding of
    // `minuend_final`: the held combination reaches the solve, its gates (here G-1: its
    // subtrahend's loads are not in the minuend's) withhold it, and its record is not
    // assessed by the held premise, whatever its gate says: no group, no bound, no row. A
    // declared difference basis with its own subtrahend (`comb:own`) is gated the same way,
    // so its record is not assessed, withheld by its gate. The producer's own premise
    // (`combination_certificate::held_reason`, unit-tested there) and the readers' (a
    // synthetic envelope in the shared forgeries) refuse any held subtrahend assessed.
    let wire = held_cantilever();
    for mode in MODES {
        let v = solve(&wire, mode);
        let ids: Vec<&str> = certificate(&v)["combinations"].as_array().unwrap().iter().map(|r| r["combination_id"].as_str().unwrap()).collect();
        assert_eq!(ids, ["comb:double", "comb:held", "comb:own"], "{mode:?}: a record for each gated combination");
        // Withheld at its gate, it publishes only its modulus-basis records, as `comb:own` does.
        assert!(v["results"].as_array().unwrap().iter().all(|r| r["basis_ref"]["ref_id"] != "comb:held" || r["kind"] == "combination_modulus_basis_record"));
        let held = record(&v, "comb:held");
        let reason = "its subtrahend is held at another case's support state (subtrahend_support_state minuend_final), a different system from its load case; the operand bounds are each operand case's own";
        assert_eq!((&held["subtrahend_support_state"], &held["assessment"], &held["reason"]), (&json!("minuend_final"), &json!("not_assessed"), &json!(reason)), "{mode:?}");
        assert_eq!((&held["groups"], &held["uncovered_row_count"]), (&json!([]), &json!(0)), "{mode:?}");
        let own = record(&v, "comb:own");
        assert_eq!((&own["subtrahend_support_state"], &own["assessment"], &own["reason"]), (&json!("own"), &json!("not_assessed"), &json!("withheld by its gate")), "{mode:?}");
        assert_eq!(own["groups"], json!([]), "{mode:?}");
        assert_eq!(record(&v, "comb:double")["subtrahend_support_state"], Value::Null);
        assert_eq!(record(&v, "comb:double")["assessment"], "assessed");
        read(&v, &wire, false, &format!("held {mode:?}"));
    }
}

/// A combination sharing its id with a load case (PP `rt_a4_combinations.rs` V7-a): `S` = 3·W2 is
/// both a load case and a combination, and `X` = S − W2 reads the load case. The ambiguous id is
/// outside the roster's population, and its maximum notice names only `S`.
fn shared_id_cantilever() -> Value {
    let mut wire = cantilever(false, &[("S", 1000.0, -1000.0), ("W2", 500.0, -500.0)], json!([
        {"id": "S", "basis": "mechanics", "terms": [{"load_case": "W2", "factor": 3.0}], "provenance": PROV},
        subtraction("X", "S", "W2")]));
    current_model::reauthor_model(&mut wire["model"]);
    wire
}

// ------------------------------------------------------------------ the shared reader corpus
fn corpus_documents() -> Vec<Value> {
    let mut out = Vec::new();
    let mut ex = reference_cantilever(true);
    current_model::reauthor_model(&mut ex["model"]);
    let mut pf = reference_cantilever(false);
    current_model::reauthor_model(&mut pf["model"]);
    for (name, reader, wire) in [("ex_cantilever", "pressure", ex), ("pf_cantilever", "preview-physics", pf), ("w16_line", "pressure", w16_line()),
        ("op1a_v2", "preview-physics", corpus_model(OP1A_CORPUS, "documents", "name", "v2_anchor_rise")),
        ("op1a_v1", "preview-physics", corpus_model(OP1A_CORPUS, "documents", "name", "v1_gap_closes")),
        ("pf_held", "preview-physics", held_cantilever()), ("pf_shared_id", "preview-physics", shared_id_cantilever())] {
        let document = solve(&wire, PreviewSolverMode::SparseInteractive);
        out.push(json!({"name": name, "reader": reader, "mode": "sparse_interactive", "model": wire["model"], "document": document}));
    }
    out
}

/// `CERT_WRITE_CORPUS=<path>` writes the shared corpus; the committed corpus is this
/// test's own output (sparse mode).
#[test]
fn committed_reader_corpus_is_the_producer_output() {
    let documents = corpus_documents();
    let corpus = json!({"description": "RT-A3 §4 combination-certificate reader corpus (DEL-05-02 Work combination-certificate.md §3): actual producer output, sparse mode, of core/product_physics/tests/combination_certificate.rs's reference models. Invented test values; not library, component, material-catalog or code data.",
        "generator": "core/product_physics/tests/combination_certificate.rs (CERT_WRITE_CORPUS=<path>)", "documents": documents});
    if let Ok(path) = std::env::var("CERT_WRITE_CORPUS") {
        std::fs::write(path, serde_json::to_string_pretty(&corpus).unwrap() + "\n").unwrap();
        return;
    }
    let committed: Value = serde_json::from_str(CORPUS).unwrap();
    assert_eq!(committed, corpus, "regenerate with CERT_WRITE_CORPUS");
}
