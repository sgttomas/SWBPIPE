//! W-16 slice b (DEL-05-02 Design `exact-combinations.md` §8): model-authored
//! load combinations on the exact pressure route (EX).
//!
//! The oracle of the closed-form cases is read from the frozen references in
//! `validation/references/w16_exact_combinations/references.json` (exact
//! rationals with an independent pi treatment; see that package's README),
//! pinned to the hash in its SHA256SUMS. Nothing here derives an expectation
//! from product code. Where a model has no closed form (connectors, arcs,
//! load/reference states, component bodies, seismic generation), a combined row
//! is checked exactly against its published operand rows: the represented
//! binary64 factors times the operand values, summed exactly and rounded once,
//! by this test's own error-free transformation (TwoProduct by fused
//! multiply-add, TwoSum expansion), never by product code.
//!
//! Coverage (§8): the pressure and axial-force line with A + 1.3B, negative,
//! zero and cancelling factors, a subtraction and range envelopes over A, B and
//! a third case C in every mode, the pressure_free_operand_recovery witnesses
//! and the missing sources; the connector gate with the listed passing and
//! withheld factor sets, a null-space q_ref and a non-finite factor; two tie
//! rods withheld by name; a pressurized realized arc in a non-default case;
//! the two load-state documents; W-15 body rows; a seismic case combined with
//! a pressure case (slice c); sparse/dense and PF parity; every #1573 code on
//! EX; the retained-source exclusion; and byte identity of the case rows.
//!
//! Criterion (DEC-024/DEC-026, unchanged): |obs - exp| <= 1e-9 |exp|; an exactly
//! zero expectation uses 1e-9 times the combination's frozen zero scale of its
//! family.
use open_pipe_stress_product_physics::{
    result_export::reader_verdict, run_linear_static_preview_value_with_mode, run_linear_static_preview_with_mode,
    LinearStaticPreviewRequest, PreviewSolverMode,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

#[path = "common/current_model.rs"]
mod current_model;
#[path = "common/t4_u5b.rs"]
mod t4_u5b;

const REFERENCES: &str = include_str!("../../../validation/references/w16_exact_combinations/references.json");
const SUMS: &str = include_str!("../../../validation/references/w16_exact_combinations/SHA256SUMS");
const CONNECTOR: &str = include_str!("fixtures/silent_zero/stress_free_connector_empty.request.json");
const LOAD_STATE_CONNECTED: &str = include_str!("../../../fixtures/product_preview/pressure/load_reference/connected.request.json");
const ARC_SKETCHES: &str = include_str!("../../../validation/references/t4_i7/u2_document_sketches.json");
const SOURCE_MIXED: &str = include_str!("../../../fixtures/product_preview/pressure_source/physics_source/mixed.request.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const RELATIVE: f64 = 1.0e-9;
const PROV: &str = "w16b invented benchmark input; not library, component, material-catalog or code data";
const EX_SEMANTICS: &str = "openpipestress.result_semantics/0.3.0/pressure-2";
const SOURCE_SEMANTICS: &str = "openpipestress.result_semantics/0.3.0/pressure-source-1";
const WITHHELD: &str = "LOAD_COMBINATION_ROW_WITHHELD";
const MISSING: &str = "LOAD_COMBINATION_SOURCE_RESULT_MISSING";
const STIFFNESS_MIXED: &str = "COMBINATION_RESOLVED_STIFFNESS_MIXED";
const INSTALLED_REFERENCE: &str = "COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE";

// ------------------------------------------------------------------ references
fn refs() -> Value {
    serde_json::from_str(REFERENCES).expect("references.json parses")
}

#[test]
fn references_are_the_frozen_bytes() {
    let digest: String = Sha256::digest(REFERENCES.as_bytes()).iter().map(|b| format!("{b:02x}")).collect();
    let line = SUMS.lines().find(|l| l.ends_with("  references.json")).expect("SHA256SUMS lists references.json");
    assert_eq!(&line[..64], digest, "the references must equal the frozen bytes");
    let doc = refs();
    assert!(doc["crosscheck"].as_array().unwrap().iter().all(|c| c["agrees"] == true));
    assert_eq!(doc["cases"].as_array().unwrap().len(), 3);
    assert_eq!(doc["combinations"].as_array().unwrap().len(), 9);
    assert_eq!(doc["factor_sets"].as_array().unwrap().len(), 10);
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

// ------------------------------------------------------------------ authoring
fn qv(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}
fn node(id: &str, x: f64) -> Value {
    json!({"id": id, "position": {"x": x, "y": 0.0, "z": 0.0}, "provenance": PROV})
}
fn section() -> Value {
    json!({"outside_diameter": qv(input("/section/outside_diameter_m"), "m"), "wall_thickness": qv(input("/section/wall_thickness_m"), "m"),
        "mill_tolerance": qv(input("/section/mill_tolerance_m"), "m"), "corrosion_allowance": qv(input("/section/corrosion_allowance_m"), "m")})
}
fn pipe(id: &str, from: &str, to: &str) -> Value {
    json!({"id": id, "from": from, "to": to, "section": section(), "material": "material:w16b",
        "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0}, "provenance": PROV})
}
fn region(id: &str, member: &str, ends: [&str; 2]) -> Value {
    json!({"id": id, "member_pipe_ids": [member], "pressure_basis": "internal_differential_zero_external_v1",
        "pressure": qv(input("/loads/pressure_Pa"), "Pa"),
        "terminals": ends.iter().map(|n| json!({"node_ref": n, "closure_transfer": "transfers_to_wall", "provenance": PROV})).collect::<Vec<_>>(),
        "provenance": PROV})
}
fn tip_force(case: &str) -> Value {
    json!({"id": format!("load:tip:{case}"), "category": "concentrated_force", "target": {"type": "node", "node": "node:c"}, "direction": "global_x",
        "magnitude": qv(input("/loads/tip_force_x_N"), "N"), "dimension": "force", "provenance": PROV})
}
fn mechanics(id: &str, terms: &[(&str, f64)]) -> Value {
    json!({"id": id, "basis": "mechanics", "terms": terms.iter().map(|(c, f)| json!({"load_case": c, "factor": f})).collect::<Vec<_>>(), "provenance": PROV})
}

/// The reference line (README "Model"), with the reference combinations.
fn line(combinations: Value) -> Value {
    let (l1, l2) = (input("/geometry/L1_m"), input("/geometry/L2_m"));
    let regions_a = json!([region("region:m1", "pipe:m1", ["node:a", "node:b"])]);
    let regions_b = json!([region("region:m2", "pipe:m2", ["node:b", "node:c"])]);
    let regions_c = json!([region("region:m1", "pipe:m1", ["node:a", "node:b"]), region("region:m2", "pipe:m2", ["node:b", "node:c"])]);
    json!({"model": {"schema_version": "0.8.0", "document_kind": "openpipestress.product_preview.model",
        "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"},
        "project": {"id": "project:w16b", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": [node("node:a", 0.0), node("node:b", l1), node("node:c", l1 + l2)],
        "pipe_segments": [pipe("pipe:m1", "node:a", "node:b"), pipe("pipe:m2", "node:b", "node:c")],
        "supports": [{"id": "support:a", "node": "node:a", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV}],
        "components": [],
        "materials": [{"id": "material:w16b", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
            "elastic_modulus": qv(input("/material/elastic_modulus_Pa"), "Pa"), "poisson_ratio": qv(input("/material/poisson_ratio"), "1"), "provenance": PROV}],
        "load_cases": [
            {"id": "case:A", "primitive_loads": [], "pressure_regions": regions_a, "provenance": PROV},
            {"id": "case:B", "primitive_loads": [tip_force("B")], "pressure_regions": regions_b, "provenance": PROV},
            {"id": "case:C", "primitive_loads": [tip_force("C")], "pressure_regions": regions_c, "provenance": PROV},
        ],
        "combinations": combinations}, "materials": []})
}

/// The reference combinations as authored model records.
fn reference_combinations() -> Value {
    let mut out = Vec::new();
    for c in refs()["combinations"].as_array().unwrap() {
        let id = c["id"].as_str().unwrap();
        out.push(match c["basis"].as_str().unwrap() {
            "mechanics" => json!({"id": id, "basis": "mechanics", "provenance": PROV, "terms": c["terms"].as_array().unwrap().iter()
                .map(|t| json!({"load_case": t["load_case"], "factor": f64::from_bits(u64::from_str_radix(t["factor_binary64_hex"].as_str().unwrap(), 16).unwrap())}))
                .collect::<Vec<_>>()}),
            "result_state_subtraction" => json!({"id": id, "basis": "result_state_subtraction", "minuend_id": c["minuend"], "subtrahend_id": c["subtrahend"], "provenance": PROV}),
            _ => json!({"id": id, "basis": "range_envelope", "operand_ids": c["operand_ids"], "mode": c["mode"], "provenance": PROV}),
        });
    }
    json!(out)
}

// ------------------------------------------------------------------ running
/// The typed request: the value entry refuses an unsafe integral float such as
/// 2^53 (CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT), so such a factor is typed input.
fn run_typed(wire: &Value, mode: PreviewSolverMode) -> Value {
    let request: LinearStaticPreviewRequest = serde_json::from_value(wire.clone()).unwrap();
    serde_json::to_value(run_linear_static_preview_with_mode(request, mode)).unwrap()
}
fn solved_typed(wire: &Value, mode: PreviewSolverMode, what: &str) -> Value {
    let out = run_typed(wire, mode);
    assert_eq!(out["status"]["mechanics"], "MECHANICS_SOLVED", "{what}: blocking {:?}", blocking(&out));
    for model in [None, Some(&wire["model"])] {
        let verdict = reader_verdict::validate("pressure", &out, model, None);
        assert_eq!(verdict["ok"], true, "{what}: reader {verdict}");
    }
    out
}
fn run(wire: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_value_with_mode(wire.clone(), mode).unwrap_or_else(|e| panic!("typed boundary: {e}"))).unwrap()
}
fn codes(out: &Value) -> Vec<String> {
    out["diagnostics"].as_array().unwrap().iter().map(|d| d["code"].as_str().unwrap().to_string()).collect()
}
fn blocking(out: &Value) -> Vec<String> {
    out["diagnostics"].as_array().unwrap().iter().filter(|d| d["severity"] == "blocking")
        .map(|d| format!("{}: {}", d["code"].as_str().unwrap(), d["message"].as_str().unwrap())).collect()
}
/// A solved pressure-2 envelope that the product reader admits with and
/// without the model.
fn solved(wire: &Value, mode: PreviewSolverMode, what: &str) -> Value {
    let out = run(wire, mode);
    assert_eq!(out["status"]["mechanics"], "MECHANICS_SOLVED", "{what}: blocking {:?}", blocking(&out));
    assert_eq!(out["producer"]["semantic_contract_id"], EX_SEMANTICS, "{what}");
    for model in [None, Some(&wire["model"])] {
        let verdict = reader_verdict::validate("pressure", &out, model, None);
        assert_eq!(verdict["ok"], true, "{what}: reader {verdict}");
    }
    out
}
fn rows(out: &Value) -> &Vec<Value> {
    out["results"].as_array().unwrap()
}
fn combination_rows<'a>(out: &'a Value, id: &str) -> Vec<&'a Value> {
    rows(out).iter().filter(|r| r["basis_ref"]["ref_type"] == "combination" && r["basis_ref"]["ref_id"] == id).collect()
}
fn record<'a>(out: &'a Value, id: &str) -> &'a Value {
    out["contract_evidence"]["exact_combinations"].as_array().expect("exact_combinations").iter()
        .find(|r| r["combination_id"] == id).unwrap_or_else(|| panic!("no record {id}"))
}
fn with_code<'a>(out: &'a Value, code: &str) -> Vec<&'a Value> {
    out["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == code).collect()
}
fn basis_of(row: &Value) -> Option<&str> {
    row["metadata"]["basis"].as_str().and_then(|b| b.split_once("; section_basis=")).map(|(_, s)| s)
}
/// The one row of `basis_ref` (type, id) matching a reference row's key.
fn find<'a>(out: &'a Value, ref_type: &str, id: &str, r: &Value) -> &'a Value {
    let found: Vec<&Value> = rows(out).iter().filter(|row| {
        row["basis_ref"]["ref_type"] == ref_type && row["basis_ref"]["ref_id"] == id
            && row["kind"] == r["kind"] && row["entity_ref"] == r["entity"]
            && (r["component"].is_null() || row["metadata"]["component"] == r["component"])
            && (r["location"].is_null() || row["metadata"]["location"] == r["location"])
            && basis_of(row) == r["section_basis"].as_str()
    }).collect();
    assert_eq!(found.len(), 1, "one {ref_type} {id} row for {r}");
    found[0]
}
fn observed(row: &Value, r: &Value) -> f64 {
    let value = row["value"].as_f64().unwrap();
    match (row["unit"].as_str().unwrap(), r["unit"].as_str().unwrap()) {
        ("mm", "m") => value / 1000.0,
        (a, b) => {
            assert_eq!(a, b, "{r}");
            value
        }
    }
}
fn agrees(obs: f64, exp: f64, zero_scale: f64) -> bool {
    if exp == 0.0 {
        obs.abs() <= RELATIVE * zero_scale
    } else {
        (obs - exp).abs() <= RELATIVE * exp.abs()
    }
}
fn family(unit: &str) -> &'static str {
    match unit {
        "N" => "force",
        "Pa" => "stress",
        "m" => "translation",
        other => panic!("unit {other}"),
    }
}

// ------------------------------------------------------------------ exact algebra (test-local)
fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let s = a + b;
    let bb = s - a;
    (s, (a - (s - bb)) + (b - bb))
}
/// The exact value of sum(f_i x_i) - c as a nonoverlapping expansion (Shewchuk),
/// from TwoProduct terms; its components are summed only to compare magnitudes.
fn residual(terms: &[(f64, f64)], c: f64) -> Vec<f64> {
    let mut expansion: Vec<f64> = Vec::new();
    let mut grow = |x: f64, expansion: &mut Vec<f64>| {
        let mut q = x;
        let mut out = Vec::new();
        for e in expansion.iter() {
            let (s, err) = two_sum(q, *e);
            if err != 0.0 {
                out.push(err);
            }
            q = s;
        }
        out.push(q);
        *expansion = out;
    };
    for (f, x) in terms {
        let p = f * x;
        let e = f.mul_add(*x, -p);
        grow(p, &mut expansion);
        grow(e, &mut expansion);
    }
    grow(-c, &mut expansion);
    expansion
}
/// `c` is sum(f_i x_i) rounded once to nearest (a tie not decided: within half an ulp).
fn rounded_once(terms: &[(f64, f64)], c: f64) -> bool {
    let r: f64 = residual(terms, c).iter().rev().sum();
    let ulp = if c == 0.0 { f64::from_bits(1) } else { f64::from_bits(c.abs().to_bits() + 1) - c.abs() };
    if c == 0.0 {
        // The product's exact zero is +0.0 and is exact.
        return r == 0.0 && c.to_bits() == 0;
    }
    r.abs() <= ulp / 2.0
}

// ------------------------------------------------------------------ the closed-form line
#[test]
fn the_reference_line_publishes_every_combination_of_its_references() {
    let wire = line(reference_combinations());
    let mut worst: Vec<(String, f64)> = Vec::new();
    let mut by_mode: Vec<Vec<f64>> = Vec::new();
    for mode in MODES {
        let out = solved(&wire, mode, "reference line");
        assert_eq!(out["numerical_quality"]["status"], "checks_passed", "{:?}", codes(&out));
        let mut values = Vec::new();
        // The primitive cases against their closed forms first.
        for case in refs()["cases"].as_array().unwrap() {
            let id = case["id"].as_str().unwrap();
            for r in case["rows"].as_array().unwrap() {
                // A pressure-free operand value is not a row of its case.
                let present = rows(&out).iter().any(|row| row["basis_ref"]["ref_id"] == id && row["kind"] == r["kind"] && row["entity_ref"] == r["entity"]
                    && row["metadata"]["component"] == r["component"] && row["metadata"]["location"] == r["location"] && basis_of(row) == r["section_basis"].as_str());
                if !present {
                    continue;
                }
                let obs = observed(find(&out, "load_case", id, r), r);
                let exp = r["value"].as_f64().unwrap();
                assert!(agrees(obs, exp, 1.0), "{id}: {r} observed {obs}");
            }
        }
        for c in refs()["combinations"].as_array().unwrap() {
            let id = c["id"].as_str().unwrap();
            let rec = record(&out, id);
            assert_eq!(rec["gate"], json!({"withheld": false, "reason": null}), "{id}");
            assert_eq!(rec["standing"], "derived_from_case_results");
            assert_eq!(rec["recoveries"].as_array().unwrap().len() as u64, c["expected_recovery_witnesses"].as_u64().unwrap(), "{id}");
            assert_eq!(rec["missing_sources"].as_array().unwrap().len() as u64, c["expected_missing_sources"].as_u64().unwrap(), "{id}");
            // Every missing source is a row the pressure family replaces: an
            // element axial force or axial normal stress of a pressurized member.
            for missing in rec["missing_sources"].as_array().unwrap() {
                let source = rows(&out).iter().find(|r| r["id"] == missing["source_result_id"]).unwrap();
                assert!(matches!(source["kind"].as_str().unwrap(), "element_local_axial_force" | "element_local_axial_normal_stress"), "{id}: {missing}");
            }
            assert_eq!(with_code(&out, MISSING).iter().filter(|d| d["affected_refs"][0] == id).count(), rec["missing_sources"].as_array().unwrap().len(), "{id}");
            assert!(with_code(&out, WITHHELD).iter().all(|d| d["affected_refs"][0] != id), "{id}: nothing withheld");
            // The recovered operand values: the pressure-free closed forms.
            for witness in rec["recoveries"].as_array().unwrap() {
                let case = witness["load_case_id"].as_str().unwrap();
                let r = refs()["cases"].as_array().unwrap().iter().find(|x| x["id"] == case).unwrap()["rows"].as_array().unwrap().iter()
                    .find(|r| r["kind"] == witness["kind"] && r["entity"] == witness["pipe_id"] && r["component"] == witness["component"]
                        && r["location"] == witness["location"] && r["section_basis"] == witness["section_basis"])
                    .unwrap_or_else(|| panic!("{id}: no reference for witness {witness}"))
                    .clone();
                assert!(agrees(witness["value"].as_f64().unwrap(), r["value"].as_f64().unwrap(), 1.0), "{id}: {witness} against {r}");
            }
            for r in c["rows"].as_array().unwrap() {
                let row = find(&out, "combination", id, r);
                let obs = observed(row, r);
                let exp = r["value"].as_f64().unwrap();
                let scale = c["zero_scales"][family(r["unit"].as_str().unwrap())].as_f64().unwrap();
                assert!(agrees(obs, exp, scale), "{id} {mode:?}: {r} observed {obs}");
                if exp != 0.0 {
                    let label = format!("{id} {} {} {}", r["kind"], r["entity"], r["unit"]);
                    let error = (obs - exp).abs() / exp.abs();
                    if worst.iter().all(|(l, _)| *l != label) {
                        worst.push((label, error));
                    } else if let Some(w) = worst.iter_mut().find(|(l, _)| *l == label) {
                        w.1 = w.1.max(error);
                    }
                }
                values.push(obs);
            }
            // A mechanics row is its operands' exact combination, rounded once.
            if c["basis"] == "mechanics" {
                let factors: Vec<f64> = c["terms"].as_array().unwrap().iter().map(|t| f64::from_bits(u64::from_str_radix(t["factor_binary64_hex"].as_str().unwrap(), 16).unwrap())).collect();
                for (row, evidence) in combination_rows(&out, id).iter().zip(rec["rows"].as_array().unwrap()) {
                    let sources = evidence["sources"].as_array().unwrap();
                    if sources.is_empty() {
                        continue;
                    }
                    let operands: Vec<f64> = sources.iter().map(|s| match s.get("result_id") {
                        Some(rid) => rows(&out).iter().find(|r| r["id"] == *rid).unwrap()["value"].as_f64().unwrap(),
                        None => rec["recoveries"][s["recovery_index"].as_u64().unwrap() as usize]["value"].as_f64().unwrap(),
                    }).collect();
                    let terms: Vec<(f64, f64)> = factors.iter().copied().zip(operands).collect();
                    assert!(rounded_once(&terms, row["value"].as_f64().unwrap()), "{id}: {row} from {terms:?}");
                }
            }
        }
        by_mode.push(values);
        // §5: headlines and maxima cover the primitive cases only.
        for basis in ["nominal_section", "reduced_section"] {
            let headline = &out["summary"][format!("max_open_formula_stress_{basis}")];
            let row = rows(&out).iter().find(|r| r["id"] == headline["result_ref"]).unwrap();
            assert_eq!(row["basis_ref"]["ref_type"], "load_case");
        }
        assert!(rows(&out).iter().all(|r| r["basis_ref"]["ref_type"] != "combination"
            || !matches!(r["kind"].as_str().unwrap(), "pipe_elastic_normal_stress_maximum_v2" | "element_local_normal_stress_extreme" | "element_local_transverse_shear_stress_maximum")));
        assert_eq!(with_code(&out, "EXACT_COMBINATION_DERIVED_FROM_CASE_RESULTS").len(), 9);
        assert_eq!(with_code(&out, "EXACT_COMBINATION_SOURCE_RECOVERY_EXCLUDED").len(), 1);
    }
    // Sparse and dense agree within the criterion on every combined value.
    for (a, b) in by_mode[0].iter().zip(&by_mode[1]) {
        assert!(agrees(*a, *b, 1.0) || (a.abs() < 1e-6 && b.abs() < 1e-6), "{a} vs {b}");
    }
    worst.sort_by(|a, b| b.1.total_cmp(&a.1));
    println!("worst relative errors: {:?}", &worst[..worst.len().min(6)]);
}

/// The combination rows do not move any primitive case row: the same document
/// without its combinations publishes the same case rows, bit for bit, and the
/// same contract evidence but the combination record.
#[test]
fn a_combination_adds_rows_and_moves_no_case_row() {
    for mode in MODES {
        let with = solved(&line(reference_combinations()), mode, "with");
        let without = solved(&line(json!([])), mode, "without");
        let cases = |out: &Value| rows(out).iter().filter(|r| r["basis_ref"]["ref_type"] != "combination").cloned().collect::<Vec<_>>();
        assert_eq!(cases(&with), cases(&without));
        assert!(without["contract_evidence"].get("exact_combinations").is_none());
        let mut evidence = with["contract_evidence"].clone();
        evidence.as_object_mut().unwrap().remove("exact_combinations");
        assert_eq!(evidence, without["contract_evidence"]);
        assert_eq!(with["summary"], without["summary"]);
        assert_eq!(with["numerical_quality"], without["numerical_quality"]);
        assert_eq!(with["formulation_basis"], without["formulation_basis"]);
        // Each operand states its case's standing verbatim.
        for rec in with["contract_evidence"]["exact_combinations"].as_array().unwrap() {
            for operand in rec["operands"].as_array().unwrap() {
                let case = with["numerical_quality"]["cases"].as_array().unwrap().iter().find(|c| c["basis_ref"]["ref_id"] == operand["load_case_id"]).unwrap();
                assert_eq!(&operand["numerical_quality"], case);
            }
        }
    }
}

// ------------------------------------------------------------------ connectors and rods
fn connector_model(q_ref_x: f64, factor_sets: &[(&str, Vec<f64>)]) -> Value {
    let mut wire: Value = serde_json::from_str(CONNECTOR).unwrap();
    let model = &mut wire["model"];
    let oc = &mut model["components"][0]["objective_connector"];
    if q_ref_x != 0.0 {
        oc["reference_state"] = json!("prestressed");
        oc["q_ref"]["translation"]["x"] = json!(q_ref_x);
    }
    let force = |id: &str, value: f64| json!({"id": id, "category": "concentrated_force", "target": {"type": "node", "node": "node:b"},
        "direction": "global_y", "magnitude": qv(value, "N"), "dimension": "force", "provenance": PROV});
    model["load_cases"] = json!([
        {"id": "case:1", "primitive_loads": [force("load:1", 1000.0)], "pressure_regions": [], "provenance": PROV},
        {"id": "case:2", "primitive_loads": [force("load:2", -400.0)], "pressure_regions": [], "provenance": PROV},
        {"id": "case:3", "primitive_loads": [force("load:3", 250.0)], "pressure_regions": [], "provenance": PROV},
    ]);
    model["combinations"] = json!(factor_sets.iter().map(|(id, factors)| {
        let terms: Vec<(&str, f64)> = ["case:1", "case:2", "case:3"].into_iter().zip(factors.iter().copied()).collect();
        mechanics(id, &terms)
    }).collect::<Vec<_>>());
    wire
}

fn factor_sets() -> Vec<(String, Vec<f64>, bool)> {
    refs()["factor_sets"].as_array().unwrap().iter().map(|s| {
        let factors = s["factors_binary64_hex"].as_array().unwrap().iter()
            .map(|h| f64::from_bits(u64::from_str_radix(h.as_str().unwrap(), 16).unwrap())).collect();
        (format!("comb:{}", s["id"].as_str().unwrap()), factors, s["unit_sum_admitted"].as_bool().unwrap())
    }).collect()
}

#[test]
fn a_nonzero_installed_reference_admits_exactly_the_unit_factor_sums() {
    let sets = factor_sets();
    let authored: Vec<(&str, Vec<f64>)> = sets.iter().map(|(id, f, _)| (id.as_str(), f.clone())).collect();
    for (q_ref, gated) in [(0.0, false), (0.001, true)] {
        let wire = connector_model(q_ref, &authored);
        for mode in MODES {
            let out = solved_typed(&wire, mode, "connector");
            for (id, factors, admitted) in &sets {
                let rec = record(&out, id);
                let withheld = gated && !admitted;
                assert_eq!(rec["gate"], json!({"withheld": withheld, "reason": if withheld { json!(INSTALLED_REFERENCE) } else { Value::Null }}),
                    "q_ref {q_ref} {id} {factors:?}");
                assert_eq!(combination_rows(&out, id).is_empty(), withheld, "{id}");
                if !withheld {
                    // The connector's rows combine as signed linear rows.
                    assert!(combination_rows(&out, id).iter().any(|r| r["kind"] == "connector_generalized_force_v1"), "{id}");
                    for (row, evidence) in combination_rows(&out, id).iter().zip(rec["rows"].as_array().unwrap()) {
                        let sources = evidence["sources"].as_array().unwrap();
                        if sources.is_empty() {
                            continue;
                        }
                        let terms: Vec<(f64, f64)> = sources.iter().zip(factors).map(|(s, f)| {
                            (*f, rows(&out).iter().find(|r| r["id"] == s["result_id"]).unwrap()["value"].as_f64().unwrap())
                        }).collect();
                        assert!(rounded_once(&terms, row["value"].as_f64().unwrap()), "{id}: {row}");
                    }
                }
            }
            assert_eq!(with_code(&out, INSTALLED_REFERENCE).len(), if gated { sets.iter().filter(|s| !s.2).count() } else { 0 });
        }
    }
}

/// A q_ref in K's null space (K q_ref = 0) applies no load, but it is still a
/// nonzero installed reference: a non-unit sum is withheld (§4).
#[test]
fn a_null_space_installed_reference_still_gates() {
    let mut wire = connector_model(0.0, &[("comb:unit", vec![0.3, 0.7]), ("comb:two", vec![1.0, 1.0])]);
    let model = &mut wire["model"];
    let oc = &mut model["components"][0]["objective_connector"];
    oc["stiffness"]["upper_triangle"][15] = json!(0);
    oc["q_ref"]["rotation"]["x"] = json!(0.01);
    model["supports"].as_array_mut().unwrap()
        .push(json!({"id": "support:b-rx", "node": "node:b", "family": "anchor", "restraints": ["RX"], "provenance": PROV}));
    for mode in MODES {
        let out = solved(&wire, mode, "null space");
        assert_eq!(record(&out, "comb:unit")["gate"]["withheld"], false);
        assert_eq!(record(&out, "comb:two")["gate"], json!({"withheld": true, "reason": INSTALLED_REFERENCE}));
    }
}

/// A non-finite factor cannot be authored in JSON; through the typed request it
/// is refused before the solve by #1573's LOAD_COMBINATION_FACTOR_INVALID.
#[test]
fn a_non_finite_factor_is_refused() {
    for factor in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let wire = connector_model(0.001, &[("comb:bad", vec![1.0, 0.0])]);
        let mut request: LinearStaticPreviewRequest = serde_json::from_value(wire).unwrap();
        request.model.combinations[0].terms[1].factor = factor;
        for mode in MODES {
            let out = serde_json::to_value(run_linear_static_preview_with_mode(request.clone(), mode)).unwrap();
            assert_eq!(out["status"]["mechanics"], "MODEL_INCOMPLETE", "{factor}");
            assert!(codes(&out).iter().any(|c| c == "LOAD_COMBINATION_FACTOR_INVALID"), "{factor}: {:?}", codes(&out));
        }
    }
}

#[test]
fn two_tie_rods_are_withheld_by_name() {
    let oracle = t4_u5b::oracle();
    let mut wire = t4_u5b::request("MO-F", &oracle["cases"]["MO-F"]);
    let model = &mut wire["model"];
    let mut second = model["load_cases"][0].clone();
    second["id"] = json!("case:second");
    for load in second["primitive_loads"].as_array_mut().unwrap() {
        load["id"] = json!(format!("{}:second", load["id"].as_str().unwrap()));
    }
    model["load_cases"].as_array_mut().unwrap().push(second);
    let first = model["load_cases"][0]["id"].as_str().unwrap().to_string();
    model["combinations"] = json!([mechanics("comb:rods", &[(&first, 1.0), ("case:second", 0.5)])]);
    for mode in MODES {
        let out = solved(&wire, mode, "MO-F");
        let rec = record(&out, "comb:rods");
        let rods: Vec<&str> = rows(&out).iter().filter(|r| r["kind"] == "connector_tie_rod_force_v1" && r["basis_ref"]["ref_id"] == first.as_str())
            .map(|r| r["id"].as_str().unwrap()).collect();
        assert_eq!(rods.len(), 2, "two rods");
        let withheld: Vec<&Value> = rec["withheld"].as_array().unwrap().iter().filter(|w| w["reason"] == "tie-rod row").collect();
        assert_eq!(withheld.iter().map(|w| w["name"].as_str().unwrap()).collect::<Vec<_>>(), rods);
        let diagnostic = with_code(&out, WITHHELD);
        assert_eq!(diagnostic.len(), 1);
        for rod in &rods {
            assert!(diagnostic[0]["message"].as_str().unwrap().contains(&format!("{rod} (tie-rod row)")));
        }
        assert!(combination_rows(&out, "comb:rods").iter().all(|r| r["kind"] != "connector_tie_rod_force_v1"));
        // The connector's other rows combine.
        assert!(combination_rows(&out, "comb:rods").iter().any(|r| r["kind"] == "connector_effective_axial_force_v1"));
    }
}

// ------------------------------------------------------------------ arcs
/// A pressurized realized arc in a non-default case: its toroidal rows pair
/// with zero pressure-free values of the unpressurized default case.
#[test]
fn a_pressurized_arc_in_a_non_default_case_combines_with_its_recovery() {
    let sketches: Value = serde_json::from_str(ARC_SKETCHES).unwrap();
    let mut wire = current_model::reauthor_request(sketches["sketches"]["U2-L-ANCH-PTW-K2"]["0.3.0"]["document"].clone());
    let model = &mut wire["model"];
    let pressurized = model["load_cases"][0].clone();
    let mut unpressurized = pressurized.clone();
    unpressurized["id"] = json!("case:unpressurized");
    unpressurized["pressure_regions"] = json!([]);
    let force = json!({"id": "load:w16b-tip", "category": "concentrated_force", "target": {"type": "node", "node": model["nodes"].as_array().unwrap().last().unwrap()["id"]},
        "direction": "global_z", "magnitude": qv(-500.0, "N"), "dimension": "force", "provenance": PROV});
    unpressurized["primitive_loads"] = json!([force]);
    let pressurized_id = pressurized["id"].as_str().unwrap().to_string();
    model["load_cases"] = json!([unpressurized, pressurized]);
    model["combinations"] = json!([mechanics("comb:arc", &[("case:unpressurized", 1.0), (&pressurized_id, 1.3)])]);
    for mode in MODES {
        let out = solved(&wire, mode, "arc");
        let rec = record(&out, "comb:arc");
        let toroidal: Vec<&Value> = rec["recoveries"].as_array().unwrap().iter()
            .filter(|w| w["kind"].as_str().unwrap().starts_with("pipe_toroidal_")).collect();
        assert!(!toroidal.is_empty(), "the arc's toroidal rows recover in the unpressurized case");
        assert!(toroidal.iter().all(|w| w["load_case_id"] == "case:unpressurized" && w["value"] == 0.0 && w["source_result_id"].is_null()));
        // The combined toroidal hoop is 1.3 times the pressurized case's (plus an exact zero).
        for row in combination_rows(&out, "comb:arc").iter().filter(|r| r["kind"] == "pipe_toroidal_membrane_hoop_stress_v1") {
            let source = rows(&out).iter().find(|r| r["basis_ref"]["ref_id"] == pressurized_id.as_str() && r["kind"] == row["kind"]
                && r["entity_ref"] == row["entity_ref"] && r["metadata"]["component"] == row["metadata"]["component"]
                && r["metadata"]["location"] == row["metadata"]["location"] && basis_of(r) == basis_of(row)).unwrap();
            assert!(rounded_once(&[(1.0, 0.0), (1.3, source["value"].as_f64().unwrap())], row["value"].as_f64().unwrap()), "{row}");
        }
        // The arc's wall force recovers from its end-tangent element force rows.
        assert!(rec["recoveries"].as_array().unwrap().iter().any(|w| w["kind"] == "pipe_wall_axial_force_v2" && w["location"] == "end_i" && w["sign"] == "negated"));
    }
}

// ------------------------------------------------------------------ load/reference states
#[test]
fn load_state_documents_combine_at_equal_stiffness_and_withhold_mixed_stiffness() {
    // Different E: cold and hot select different temperature points.
    let mut mixed: Value = serde_json::from_str(LOAD_STATE_CONNECTED).unwrap();
    mixed["model"]["combinations"] = json!([
        mechanics("comb:cold+hot", &[("case:cold", 1.0), ("case:hot", 1.0)]),
        {"id": "comb:hot-cold", "basis": "result_state_subtraction", "minuend_id": "case:hot", "subtrahend_id": "case:cold", "provenance": PROV},
    ]);
    // Equal stiffness: the hot case selects the cold point; its operating temperature and the
    // prescribed boundary motion stay (right-hand-side contributions to the same K).
    let mut equal = mixed.clone();
    for state in equal["model"]["load_cases"][1]["analysis_state"]["element_states"].as_array_mut().unwrap() {
        state["material_selection"]["point_ref"] = json!("point:cold");
        state["analysis_basis_override"] = json!({"reason": "w16b: properties selected at the installed point to keep one stiffness", "provenance": PROV});
    }
    for mode in MODES {
        let out = solved(&mixed, mode, "mixed stiffness");
        assert_eq!(record(&out, "comb:cold+hot")["gate"], json!({"withheld": true, "reason": STIFFNESS_MIXED}));
        assert!(combination_rows(&out, "comb:cold+hot").is_empty());
        // A subtraction compares states and may span different stiffness.
        assert_eq!(record(&out, "comb:hot-cold")["gate"]["withheld"], false);
        assert!(!combination_rows(&out, "comb:hot-cold").is_empty());
        let out = solved(&equal, mode, "equal stiffness");
        assert_eq!(record(&out, "comb:cold+hot")["gate"]["withheld"], false);
        let rec = record(&out, "comb:cold+hot");
        for (row, evidence) in combination_rows(&out, "comb:cold+hot").iter().zip(rec["rows"].as_array().unwrap()) {
            let sources = evidence["sources"].as_array().unwrap();
            if sources.is_empty() {
                continue;
            }
            let terms: Vec<(f64, f64)> = sources.iter().map(|s| (1.0, rows(&out).iter().find(|r| r["id"] == s["result_id"]).unwrap()["value"].as_f64().unwrap())).collect();
            assert!(rounded_once(&terms, row["value"].as_f64().unwrap()), "{row}");
        }
        // The prescribed root motion combines through both factors: UX = 0.5 mm in each case.
        let ux = combination_rows(&out, "comb:cold+hot").into_iter()
            .find(|r| r["kind"] == "global_nodal_displacement_x" && r["entity_ref"] == "node:root").unwrap();
        assert_eq!(ux["value"], 1.0);
    }
}

// ------------------------------------------------------------------ W-15 bodies and slice c
#[test]
fn component_body_records_cross_a_combination() {
    let mut wire = line(json!([mechanics("comb:K1", &[("case:A", 1.0), ("case:B", 1.3)])]));
    wire["model"]["components"] = json!([{"id": "component:valve", "kind": "valve", "node": "node:b",
        "geometry": {"rigid_pipe_ref": "pipe:m2"},
        "modifiers": {"stiffness_scaling_user_value": {"value": 10.0, "unit": "1"}, "source_reference": "invented W-16b scale"},
        "mechanics_interface": {"solver_consumption": "mechanics_geometry_only"}, "provenance": PROV}]);
    for mode in MODES {
        let out = solved(&wire, mode, "body");
        let body_rows: Vec<&Value> = combination_rows(&out, "comb:K1").into_iter().filter(|r| r["entity_ref"] == "pipe:m2" && r.get("component_body").is_some()).collect();
        assert!(!body_rows.is_empty());
        assert!(body_rows.iter().all(|r| r["component_body"] == json!({"component_ref": "component:valve", "stiffness_scale": 10.0, "section_role": "reference_pipe_section"})));
        // Every combined stress row of the body member carries it; no row of the other member does.
        assert!(combination_rows(&out, "comb:K1").iter().all(|r| r.get("component_body").is_none() || r["entity_ref"] == "pipe:m2"));
    }
}

#[test]
fn a_seismic_case_combines_with_a_pressure_case() {
    let mut wire = line(json!([mechanics("comb:seismic+A", &[("case:A", 1.0), ("case:seismic", 1.0)])]));
    let model = &mut wire["model"];
    for pipe in model["pipe_segments"].as_array_mut().unwrap() {
        pipe["section"]["material_density"] = qv(7850.0, "kg/m^3");
    }
    model["load_cases"].as_array_mut().unwrap().push(json!({"id": "case:seismic", "primitive_loads": [], "pressure_regions": [],
        "equivalent_static": {"seismic": {"gravity_acceleration": qv(9.81, "m/s^2"), "g_factor_y": qv(0.25, "1")}, "provenance": PROV}, "provenance": PROV}));
    for mode in MODES {
        let out = solved(&wire, mode, "seismic");
        let rec = record(&out, "comb:seismic+A");
        assert_eq!(rec["gate"]["withheld"], false);
        let shear = combination_rows(&out, "comb:seismic+A").into_iter()
            .find(|r| r["kind"] == "support_reaction_component_v2" && r["metadata"]["component"] == "Fy").unwrap();
        let seismic = rows(&out).iter().find(|r| r["basis_ref"]["ref_id"] == "case:seismic" && r["kind"] == "support_reaction_component_v2" && r["metadata"]["component"] == "Fy").unwrap();
        assert!(seismic["value"].as_f64().unwrap() != 0.0);
        assert!(rounded_once(&[(1.0, 0.0), (1.0, seismic["value"].as_f64().unwrap())], shear["value"].as_f64().unwrap()));
    }
}

// ------------------------------------------------------------------ parity with PF
/// A pressure-free EX model and its PF twin publish the same combined force
/// rows: the same load-case algebra over equal operands.
#[test]
fn force_rows_of_a_pressure_free_model_agree_with_the_preview_route() {
    let mut ex = line(json!([mechanics("comb:K", &[("case:A", 0.7), ("case:B", -1.3)])]));
    for case in ex["model"]["load_cases"].as_array_mut().unwrap() {
        case["pressure_regions"] = json!([]);
        let id = case["id"].as_str().unwrap().to_string();
        case["primitive_loads"] = json!([tip_force(&id)]);
    }
    let mut pf = ex.clone();
    let model = pf["model"].as_object_mut().unwrap();
    model.remove("pressure_contract");
    for case in pf["model"]["load_cases"].as_array_mut().unwrap() {
        case.as_object_mut().unwrap().remove("pressure_regions");
    }
    for pipe in pf["model"]["pipe_segments"].as_array_mut().unwrap() {
        pipe["section"].as_object_mut().unwrap().remove("corrosion_allowance");
    }
    let (e, nu) = (input("/material/elastic_modulus_Pa"), input("/material/poisson_ratio"));
    pf["model"]["materials"] = json!([{"id": "material:w16b", "elastic_modulus": qv(e, "Pa"), "shear_modulus": qv(e / (2.0 * (1.0 + nu)), "Pa"), "provenance": PROV}]);
    for mode in MODES {
        let exact = solved(&ex, mode, "pressure-free exact");
        let preview = run(&pf, mode);
        assert_eq!(preview["status"]["mechanics"], "MECHANICS_SOLVED", "{:?}", blocking(&preview));
        let force_kinds = ["element_local_axial_force", "element_local_shear_force_y", "element_local_shear_force_z", "element_local_bending_moment_y",
            "element_local_bending_moment_z", "element_local_torsional_moment", "support_reaction_component_v2"];
        let mut compared = 0;
        for row in combination_rows(&exact, "comb:K").into_iter().filter(|r| force_kinds.contains(&r["kind"].as_str().unwrap())) {
            let twin = combination_rows(&preview, "comb:K").into_iter().find(|r| r["kind"] == row["kind"] && r["entity_ref"] == row["entity_ref"]
                && r["metadata"]["component"] == row["metadata"]["component"] && r["metadata"]["location"] == row["metadata"]["location"])
                .unwrap_or_else(|| panic!("no PF twin of {row}"));
            let (a, b) = (row["value"].as_f64().unwrap(), twin["value"].as_f64().unwrap());
            assert!(agrees(a, b, 1.0e3), "{row} vs {twin}");
            compared += 1;
        }
        assert!(compared > 20, "{compared}");
    }
}

// ------------------------------------------------------------------ #1573 codes and the source-recovery exclusion
#[test]
fn every_1573_code_applies_on_the_exact_route() {
    for (combination, code) in [
        (json!({"id": "comb:x", "basis": "mechanics", "terms": [{"load_case": "case:missing", "factor": 1.0}], "provenance": PROV}), "LOAD_COMBINATION_LOAD_CASE_UNKNOWN"),
        (json!({"id": "comb:x", "basis": "mechanics", "terms": [{"load_case": "case:A", "factor": 1.0}, {"load_case": "case:A", "factor": 2.0}], "provenance": PROV}), "LOAD_COMBINATION_DUPLICATE_TERM"),
        (json!({"id": "comb:x", "basis": "mechanics", "terms": [], "provenance": PROV}), "LOAD_COMBINATION_TERMS_EMPTY"),
        (json!({"id": "comb:x", "basis": "range_envelope", "operand_ids": [], "mode": "max", "provenance": PROV}), "LOAD_COMBINATION_OPERANDS_EMPTY"),
        (json!({"id": "comb:x", "basis": "result_state_subtraction", "minuend_id": "case:A", "subtrahend_id": "case:A", "provenance": PROV}), "LOAD_COMBINATION_DUPLICATE_TERM"),
        (json!({"id": "comb:x", "basis": "range_envelope", "operand_ids": ["case:A"], "mode": "median", "provenance": PROV}), "LOAD_COMBINATION_RANGE_MODE_UNKNOWN"),
        (json!({"id": "comb:x", "basis": "result_state_subtraction", "minuend_id": "case:A", "provenance": PROV}), "LOAD_COMBINATION_SHAPE_INVALID"),
        (json!({"id": "comb:x", "basis": "unknown", "provenance": PROV}), "LOAD_COMBINATION_BASIS_UNSUPPORTED"),
    ] {
        let wire = line(json!([combination]));
        for mode in MODES {
            let out = run(&wire, mode);
            assert_eq!(out["status"]["mechanics"], "MODEL_INCOMPLETE", "{code}: {:?}", codes(&out));
            assert!(rows(&out).is_empty());
            assert!(codes(&out).iter().any(|c| c == code), "{code}: {:?}", codes(&out));
            assert!(!codes(&out).iter().any(|c| c == "EXACT_PRESSURE_FAMILY_NOT_ADMITTED"), "{code}: the family refusal is gone");
        }
    }
}

/// #1573's closed combination record: an undeclared key is refused at the value
/// entry, before any route, on EX as on PF.
#[test]
fn an_unknown_combination_key_is_refused_on_the_exact_route() {
    let wire = line(json!([{"id": "comb:x", "basis": "mechanics", "terms": [{"load_case": "case:A", "factor": 1.0}], "provenance": PROV, "weight": 1.0}]));
    for mode in MODES {
        let error = run_linear_static_preview_value_with_mode(wire.clone(), mode).err().expect("refused");
        assert!(error.starts_with("UNKNOWN_MODEL_FIELD") && error.contains("combination `comb:x`"), "{error}");
    }
}

/// §7: a document with a combination attempts no retained-source recovery for
/// any case: the source-selected fixture publishes pressure-2 with the
/// exclusion stated, and pressure-source-1 without the combination.
#[test]
fn a_combination_excludes_retained_source_recovery() {
    let without: Value = serde_json::from_str(SOURCE_MIXED).unwrap();
    let mut with = without.clone();
    let cases: Vec<String> = with["model"]["load_cases"].as_array().unwrap().iter().map(|c| c["id"].as_str().unwrap().to_string()).collect();
    with["model"]["combinations"] = json!([{"id": "comb:range", "basis": "range_envelope", "operand_ids": cases, "mode": "max_abs", "provenance": PROV}]);
    for mode in MODES {
        assert_eq!(run(&without, mode)["producer"]["semantic_contract_id"], SOURCE_SEMANTICS);
        let out = solved(&with, mode, "source exclusion");
        assert!(out.get("source_block_recovery").is_none());
        let stated = with_code(&out, "EXACT_COMBINATION_SOURCE_RECOVERY_EXCLUDED");
        assert_eq!(stated.len(), 1);
        assert_eq!(stated[0]["affected_refs"], json!(["comb:range"]));
        assert!(record(&out, "comb:range")["source_recovery"].as_str().unwrap().starts_with("excluded:"));
        // An envelope's modulus-basis records are published, as on PF.
        assert_eq!(combination_rows(&out, "comb:range").iter().filter(|r| r["kind"] == "combination_modulus_basis_record").count(), cases.len());
    }
}

/// §5: combination rows never enter the numerical-eligibility verdict, which
/// stays with the primitive cases.
#[test]
fn combinations_leave_the_numerical_standing_unchanged() {
    for mode in MODES {
        let with = solved(&line(reference_combinations()), mode, "with");
        let without = solved(&line(json!([])), mode, "without");
        let standing = |out: &Value, wire: &Value| reader_verdict::standing(out, Some(&wire["model"]), None);
        let (a, b) = (standing(&with, &line(reference_combinations())), standing(&without, &line(json!([]))));
        assert_eq!(a["eligible"], b["eligible"], "{a} {b}");
        assert_eq!(a["findings"], b["findings"], "{a} {b}");
        assert_eq!(a["token"], b["token"], "{a} {b}");
    }
}

// ------------------------------------------------------------------ the shared reader corpus
const CORPUS_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../fixtures/results/pressure_v3_combination_reader_corpus.json");

/// The shared combination reader corpus (RE native and wasm readers, the
/// Python verifier): one envelope per combination family, with its model.
/// Regenerate with `W16B_WRITE_CORPUS=1`; otherwise the committed bytes must
/// be current.
#[test]
fn shared_combination_reader_corpus_is_current() {
    let line = line(json!([
        mechanics("comb:K1", &[("case:A", 1.0), ("case:B", 1.3)]),
        {"id": "comb:S1", "basis": "result_state_subtraction", "minuend_id": "case:C", "subtrahend_id": "case:A", "provenance": PROV},
        {"id": "comb:E-max_abs", "basis": "range_envelope", "operand_ids": ["case:C", "case:A", "case:B"], "mode": "max_abs", "provenance": PROV},
    ]));
    let connector = connector_model(0.001, &[("comb:unit", vec![0.3, 0.7]), ("comb:two", vec![1.0, 1.0]), ("comb:boundary", vec![1.0, 2f64.powi(-46)])]);
    let rods = {
        let oracle = t4_u5b::oracle();
        let mut wire = t4_u5b::request("MO-F", &oracle["cases"]["MO-F"]);
        let model = &mut wire["model"];
        let mut second = model["load_cases"][0].clone();
        second["id"] = json!("case:second");
        for load in second["primitive_loads"].as_array_mut().unwrap() {
            load["id"] = json!(format!("{}:second", load["id"].as_str().unwrap()));
        }
        model["load_cases"].as_array_mut().unwrap().push(second);
        let first = model["load_cases"][0]["id"].as_str().unwrap().to_string();
        model["combinations"] = json!([mechanics("comb:rods", &[(&first, 1.0), ("case:second", 0.5)])]);
        wire
    };
    let mut load_state: Value = serde_json::from_str(LOAD_STATE_CONNECTED).unwrap();
    load_state["model"]["combinations"] = json!([
        mechanics("comb:cold+hot", &[("case:cold", 1.0), ("case:hot", 1.0)]),
        {"id": "comb:hot-cold", "basis": "result_state_subtraction", "minuend_id": "case:hot", "subtrahend_id": "case:cold", "provenance": PROV},
    ]);
    let celsius = celsius_line();
    let mut cases = Vec::new();
    for (label, wire, mode) in [
        ("W16B line: mechanics, subtraction and envelope", line, PreviewSolverMode::SparseInteractive),
        ("W16B Celsius modulus temperature: the record states kelvin", celsius, PreviewSolverMode::SparseInteractive),
        ("W16B connector with an installed reference: admitted and gated sums", connector, PreviewSolverMode::SparseInteractive),
        ("W16B MO-F: two tie rods withheld by name", rods, PreviewSolverMode::DenseScrutiny),
        ("W16B load state: mixed stiffness gated, subtraction published", load_state, PreviewSolverMode::SparseInteractive),
    ] {
        let envelope = solved(&wire, mode, label);
        // No witness value is -0.0 (a negated zero is +0.0), so a value-preserving
        // re-serialization (JCS, JSON.stringify) reads the same record.
        for record in envelope["contract_evidence"]["exact_combinations"].as_array().unwrap() {
            for witness in record["recoveries"].as_array().unwrap() {
                let value = witness["value"].as_f64().unwrap();
                assert!(!(value == 0.0 && value.is_sign_negative()), "{label}: {witness}");
            }
        }
        cases.push(json!({"label": label, "solver_mode": mode.as_str(), "model": wire["model"], "envelope": envelope}));
    }
    let corpus = json!({
        "corpus": "W-16 slice b pressure-2 combination reader corpus",
        "generator": "core/product_physics/tests/w16_exact_combinations.rs::shared_combination_reader_corpus_is_current",
        "cases": cases,
    });
    let bytes = serde_json::to_string_pretty(&corpus).unwrap() + "\n";
    if std::env::var_os("W16B_WRITE_CORPUS").is_some() {
        std::fs::write(CORPUS_PATH, &bytes).unwrap();
        return;
    }
    // The committed corpus is read at compile time, so the numerical CI's read
    // index binds it to this test (no runtime read).
    let committed = include_str!("../../../fixtures/results/pressure_v3_combination_reader_corpus.json");
    assert!(committed == bytes, "the shared pressure-2 combination reader corpus is stale; regenerate it with W16B_WRITE_CORPUS=1");
}

/// Review r1 (1): the line with temperature-dependent properties, each case
/// selecting them at an authored 100 degC; one mechanics combination.
fn celsius_line() -> Value {
    let mut wire = line(json!([mechanics("comb:K1", &[("case:A", 1.0), ("case:B", 1.3)])]));
    let point = |id: &str, celsius: f64| json!({"id": id, "temperature": {"value": celsius, "unit": "degC"},
        "elastic_modulus": qv(input("/material/elastic_modulus_Pa"), "Pa"), "poisson_ratio": qv(input("/material/poisson_ratio"), "1"),
        "provenance": PROV});
    wire["model"]["materials"][0]["temperature_points"] = json!([point("point:cold", 0.0), point("point:hot", 200.0)]);
    for case in wire["model"]["load_cases"].as_array_mut().unwrap() {
        case["modulus_basis_temperature"] = json!({"value": 100.0, "unit": "degC"});
    }
    wire
}

/// Review r1 (1): an authored Celsius selection temperature is recorded as the
/// producer normalized it (kelvin, by the units authority), which the readers
/// re-derive from the captured quantity.
#[test]
fn a_celsius_selection_temperature_is_recorded_in_kelvin() {
    for mode in MODES {
        let out = solved(&celsius_line(), mode, "celsius");
        let record = record(&out, "comb:K1");
        assert_eq!(record["gate"]["withheld"], false);
        for operand in record["operands"].as_array().unwrap() {
            assert_eq!(operand["modulus_basis_temperature"], json!({"value": 100.0 + 273.15, "unit": "K"}));
        }
    }
}

/// §3: ids rich in delimiters (and in multi-byte characters) pair by their
/// structured fields; the combined ids are length-prefixed by bytes and distinct.
#[test]
fn delimiter_rich_ids_pair_by_structured_fields() {
    let text = line(json!([mechanics("comb:é+1:2", &[("case:A:α", 1.0), ("case:B:1:1", 1.3)])])).to_string()
        .replace("\"case:A\"", "\"case:A:α\"").replace("\"case:B\"", "\"case:B:1:1\"").replace("pipe:m1", "pipe:m:1").replace("pipe:m2", "pipe:m1:");
    let wire: Value = serde_json::from_str(&text).unwrap();
    for mode in MODES {
        let out = solved(&wire, mode, "delimiter-rich ids");
        let combined = combination_rows(&out, "comb:é+1:2");
        assert!(!combined.is_empty());
        let ids: std::collections::HashSet<&str> = combined.iter().map(|r| r["id"].as_str().unwrap()).collect();
        assert_eq!(ids.len(), combined.len());
        assert!(combined.iter().all(|r| r["id"].as_str().unwrap().starts_with("result:combination:11:comb:é+1:2")), "byte length of the id");
        // Both members' wall forces combined, each from its own operand rows.
        for member in ["pipe:m:1", "pipe:m1:"] {
            assert!(combined.iter().any(|r| r["kind"] == "pipe_wall_axial_force_v2" && r["entity_ref"] == member), "{member}");
        }
    }
}
