//! Operating-case slice 1b readers over the shared corpus
//! (`fixtures/results/preview_physics_difference_basis_corpus.json`, written by
//! `core/product_physics/tests/op1b_difference_basis.rs`): with the model, and
//! without it under both settings of the model-free switch (option C, the shipped
//! default, and option A, behind the switch until its review).
use super::*;
use crate::reader_verdict;

const CORPUS: &str = include_str!("../../../../fixtures/results/preview_physics_difference_basis_corpus.json");

fn corpus() -> Vec<(String, Value, Value)> {
    let corpus: Value = serde_json::from_str(CORPUS).unwrap();
    corpus["documents"].as_array().unwrap().iter().map(|e| (e["name"].as_str().unwrap().to_string(), e["model"].clone(), e["document"].clone())).collect()
}
fn document(name: &str) -> (Value, Value) {
    let (_, model, document) = corpus().into_iter().find(|(n, _, _)| n == name).unwrap_or_else(|| panic!("{name}"));
    (model, document)
}
fn code(verdict: &Value) -> String {
    if verdict["ok"] == true { "ok".into() } else { verdict["error"]["code"].as_str().unwrap_or_default().to_string() }
}
fn read(source: &Value, model: Option<&Value>, mode: ModelFree) -> String {
    TEST_MODE.with(|m| m.set(mode));
    let verdict = reader_verdict::validate("preview-physics", source, model, None);
    TEST_MODE.with(|m| m.set(MODEL_FREE));
    code(&verdict)
}
fn record<'a>(v: &'a mut Value, index: usize) -> &'a mut Value {
    &mut v["contract_evidence"][MEMBER][index]
}

#[test]
fn the_shipped_switch_fails_closed() {
    assert_eq!(MODEL_FREE, ModelFree::Refuse);
}

#[test]
fn every_corpus_document_reads_with_its_model_and_under_option_a_and_is_refused_under_option_c() {
    let documents = corpus();
    assert!(documents.len() >= 9);
    for (name, model, document) in documents {
        assert_eq!(read(&document, Some(&model), ModelFree::Refuse), "ok", "{name}: with the model");
        assert_eq!(read(&document, None, ModelFree::Verify), "ok", "{name}: option A");
        assert_eq!(read(&document, None, ModelFree::Refuse), UNVERIFIED, "{name}: option C");
    }
}

/// Option A refuses by name what it can see inside the document.
#[test]
fn option_a_refuses_tampered_evidence_by_name() {
    let (_, admitted) = document("v1_closing");
    let (_, withheld) = document("v5_sustained_only_load");
    let (_, g2) = document("v5_g2");
    let mut cases: Vec<(&str, Value, &str)> = Vec::new();
    let mut tamper = |what: &'static str, base: &Value, code: &'static str, edit: &dyn Fn(&mut Value)| {
        let mut forged = base.clone();
        edit(&mut forged);
        cases.push((what, forged, code));
    };
    // Absent, incomplete or extra evidence.
    tamper("absent member", &admitted, EVIDENCE, &|v| { v["contract_evidence"].as_object_mut().unwrap().remove(MEMBER); });
    tamper("empty member", &admitted, EVIDENCE, &|v| v["contract_evidence"][MEMBER] = json!([]));
    tamper("missing operand", &admitted, EVIDENCE, &|v| { record(v, 0).as_object_mut().unwrap().remove("subtrahend"); });
    tamper("extra record member", &admitted, EVIDENCE, &|v| record(v, 0)["note"] = json!("x"));
    tamper("another token", &admitted, EVIDENCE, &|v| record(v, 0)["difference_basis"] = json!("sustained"));
    tamper("evidence for a plain subtraction", &admitted, ROW, &|v| {
        let mut extra = record(v, 0).clone();
        extra["combination_id"] = json!("comb:plain");
        v["contract_evidence"][MEMBER].as_array_mut().unwrap().push(extra);
    });
    tamper("a record repeated", &admitted, EVIDENCE, &|v| {
        let again = record(v, 0).clone();
        v["contract_evidence"][MEMBER].as_array_mut().unwrap().push(again);
    });
    // Bindings.
    tamper("unknown combination", &admitted, BINDING, &|v| record(v, 0)["combination_id"] = json!("comb:none"));
    tamper("unknown case", &admitted, BINDING, &|v| record(v, 0)["subtrahend"]["load_case_id"] = json!("case:none"));
    tamper("the minuend's case named twice", &admitted, BINDING, &|v| record(v, 0)["subtrahend"]["load_case_id"] = json!("case:O"));
    tamper("cases swapped", &admitted, BINDING, &|v| {
        let r = record(v, 0).clone();
        record(v, 0)["minuend"] = r["subtrahend"].clone();
        record(v, 0)["subtrahend"] = r["minuend"].clone();
    });
    // Counts, duplicates and values.
    tamper("a zero count", &admitted, EVIDENCE, &|v| record(v, 0)["subtrahend"]["primitive_contents"][0]["count"] = json!(0));
    tamper("a fractional count", &admitted, EVIDENCE, &|v| record(v, 0)["subtrahend"]["primitive_contents"][0]["count"] = json!(1.5));
    tamper("a float count", &admitted, EVIDENCE, &|v| record(v, 0)["subtrahend"]["primitive_contents"][0]["count"] = json!(1.0));
    tamper("a duplicated entry", &admitted, EVIDENCE, &|v| {
        let list = record(v, 0)["minuend"]["primitive_contents"].as_array_mut().unwrap();
        let first = list[0].clone();
        list.insert(0, first);
    });
    tamper("entries out of order", &admitted, EVIDENCE, &|v| record(v, 0)["minuend"]["primitive_contents"].as_array_mut().unwrap().reverse());
    tamper("a non-finite value", &admitted, EVIDENCE, &|v| record(v, 0)["subtrahend"]["primitive_contents"][0]["magnitude"]["value_bits"] = json!(format!("{:016x}", f64::INFINITY.to_bits())));
    tamper("a NaN value", &admitted, EVIDENCE, &|v| record(v, 0)["subtrahend"]["primitive_contents"][0]["magnitude"]["value_bits"] = json!(format!("{:016x}", f64::NAN.to_bits())));
    tamper("upper-case bits", &admitted, EVIDENCE, &|v| record(v, 0)["subtrahend"]["primitive_contents"][0]["magnitude"]["value_bits"] = json!(format!("{:016X}", 0.1f64.to_bits())));
    tamper("short bits", &admitted, EVIDENCE, &|v| record(v, 0)["subtrahend"]["primitive_contents"][0]["magnitude"]["value_bits"] = json!("4098"));
    tamper("a number for the bits", &admitted, EVIDENCE, &|v| record(v, 0)["subtrahend"]["primitive_contents"][0]["magnitude"]["value_bits"] = json!(1536.0));
    tamper("an unknown target type", &admitted, EVIDENCE, &|v| record(v, 0)["subtrahend"]["primitive_contents"][0]["target"] = json!({"type": "pipe", "pipe": "pipe:a-b"}));
    // The containment comparison: one bit of the subtrahend's value, unit, target or
    // direction, or its multiplicity, makes the stated admission underivable.
    tamper("one value bit", &admitted, GATE_DERIVATION, &|v| {
        let e = &mut record(v, 0)["subtrahend"]["primitive_contents"][0]["magnitude"]["value_bits"];
        let bits = u64::from_str_radix(e.as_str().unwrap(), 16).unwrap() + 1;
        *e = json!(format!("{bits:016x}"));
    });
    tamper("a unit", &admitted, GATE_DERIVATION, &|v| record(v, 0)["subtrahend"]["primitive_contents"][0]["magnitude"]["unit"] = json!("kN"));
    tamper("a target", &admitted, GATE_DERIVATION, &|v| record(v, 0)["subtrahend"]["primitive_contents"][0]["target"]["node"] = json!("node:a"));
    tamper("a direction", &admitted, GATE_DERIVATION, &|v| record(v, 0)["subtrahend"]["primitive_contents"][0]["direction"] = json!("global_y"));
    tamper("a multiplicity", &admitted, GATE_DERIVATION, &|v| record(v, 0)["subtrahend"]["primitive_contents"][0]["count"] = json!(2));
    // Forged gates and reasons.
    // (Refused first by slice 1a's presence rule: an admitted combination of a nonlinear
    // model states its operands; this module's own verdict is checked below.)
    tamper("a withheld gate made admitted", &withheld, "SOURCE_PREVIEW_PHYSICS_COMBINATION_OPERANDS_PRESENCE", &|v| {
        for g in v["contract_evidence"]["combination_gates"].as_array_mut().unwrap() {
            if g["combination_id"] == "comb:o-s" {
                *g = json!({"combination_id": "comb:o-s", "withheld": false, "reason": null});
            }
        }
        v["diagnostics"].as_array_mut().unwrap().retain(|d| d["code"] != SUSTAINED_NOT_IN_OPERATING);
    });
    tamper("G-1's reason restated as G-2's", &withheld, GATE_DERIVATION, &|v| {
        for g in v["contract_evidence"]["combination_gates"].as_array_mut().unwrap() {
            if g["combination_id"] == "comb:o-s" {
                g["reason"] = json!(MODULUS_MIXED);
            }
        }
    });
    tamper("withheld evidence made contained", &withheld, GATE_DERIVATION, &|v| {
        record(v, 0)["subtrahend"]["primitive_contents"].as_array_mut().unwrap().retain(|e| e["category"] != "weight");
    });
    tamper("G-2's bases made equal", &g2, GATE_DERIVATION, &|v| {
        let rows = v["results"].as_array_mut().unwrap();
        let labels: Vec<Value> = rows.iter().filter(|r| r["kind"] == MODULUS_RECORD && r["basis_ref"]["ref_id"] == "comb:o-s").map(|r| r["metadata"]["basis"].clone()).collect();
        for r in rows.iter_mut().filter(|r| r["kind"] == MODULUS_RECORD && r["basis_ref"]["ref_id"] == "comb:o-s") {
            r["metadata"]["basis"] = labels[1].clone();
        }
    });
    tamper("a G-1 reason without its record", &withheld, EVIDENCE, &|v| { v["contract_evidence"].as_object_mut().unwrap().remove(MEMBER); });
    for (what, forged, expected) in &cases {
        assert_eq!(read(forged, None, ModelFree::Verify), *expected, "{what}");
    }
    let (_, forged, _) = cases.iter().find(|(what, _, _)| *what == "a withheld gate made admitted").unwrap();
    assert_eq!(check_with(forged, None, ModelFree::Verify).unwrap_err(), GATE_DERIVATION);
    assert!(cases.len() >= 30);
}

/// The scope option A states: it checks the record's internal derivation, not its
/// truth to the absent model. A coherent forgery (the evidence and the gate altered
/// together) passes without the model and is refused with it.
#[test]
fn a_coherent_forgery_is_not_authenticated_without_the_model() {
    let (model, mut forged) = document("v5_sustained_only_load");
    record(&mut forged, 0)["subtrahend"]["primitive_contents"].as_array_mut().unwrap().retain(|e| e["category"] != "weight");
    // Without the model it remains a withheld, internally inconsistent claim: the gate is
    // still G-1's, so it is refused.
    assert_eq!(read(&forged, None, ModelFree::Verify), GATE_DERIVATION);
    // An admitted declared difference whose evidence states a different but still
    // contained subtrahend: internally consistent, so admitted without the model.
    let (model_a, mut coherent) = document("v1_closing");
    record(&mut coherent, 0)["subtrahend"]["primitive_contents"] = json!([]);
    assert_eq!(read(&coherent, None, ModelFree::Verify), "ok", "option A cannot see a coherent forgery");
    assert_eq!(read(&coherent, Some(&model_a), ModelFree::Verify), MODEL, "the model can");
    let _ = model;
}

// ------------------------------------------------------------------ review r1
/// `v1_closing` with its plain twin `comb:plain` (same operands) declared too: a second
/// record for the same two cases, and the twin's rows stating the declared sign.
fn declare_plain_twin(v: &mut Value) {
    let mut twin = record(v, 0).clone();
    twin["combination_id"] = json!("comb:plain");
    v["contract_evidence"][MEMBER].as_array_mut().unwrap().push(twin);
    for row in v["results"].as_array_mut().unwrap() {
        if row["basis_ref"]["ref_id"] == "comb:plain" && row["kind"] != MODULUS_RECORD && row["metadata"]["sign_convention"].is_string() {
            row["metadata"]["sign_convention"] = json!(SIGN);
        }
    }
}
fn model_declaring_plain_twin(model: &Value) -> Value {
    let mut model = model.clone();
    for c in model["combinations"].as_array_mut().unwrap() {
        if c["id"] == "comb:plain" {
            c["difference_basis"] = json!("operating_minus_sustained");
        }
    }
    model
}

/// Review r1 P1: one producer resolution gives one multiset per case, so a case id that
/// recurs across records must carry byte-identical contents, in both readings.
#[test]
fn a_case_recurring_across_records_carries_identical_contents() {
    let (model, base) = document("v1_closing");
    let model = model_declaring_plain_twin(&model);
    let mut consistent = base.clone();
    declare_plain_twin(&mut consistent);
    assert_eq!(read(&consistent, None, ModelFree::Verify), "ok");
    assert_eq!(read(&consistent, Some(&model), ModelFree::Verify), "ok");
    assert_eq!(read(&consistent, None, ModelFree::Refuse), UNVERIFIED);
    // The twin's sustained multiset emptied: the same case bound to two multisets.
    let mut contradictory = consistent.clone();
    record(&mut contradictory, 1)["subtrahend"]["primitive_contents"] = json!([]);
    assert_eq!(read(&contradictory, None, ModelFree::Verify), BINDING);
    assert_eq!(read(&contradictory, Some(&model), ModelFree::Verify), BINDING, "checked first, under the same code");
    // A recurring minuend differing in one count, too.
    let mut minuend = consistent.clone();
    record(&mut minuend, 1)["minuend"]["primitive_contents"][0]["count"] = json!(2);
    assert_eq!(read(&minuend, None, ModelFree::Verify), BINDING);
    assert_eq!(read(&minuend, Some(&model), ModelFree::Verify), BINDING);
}

/// Review r1 P2/P2: the closed shape and the count domain (an integer token from 1 to
/// 2^53 - 1, the checked JSON profile's safe integers) are validated before any
/// comparison with the model's evidence, in both readings, with one validator.
#[test]
fn the_count_domain_and_shape_are_checked_before_the_model_comparison() {
    const MAX: u64 = open_pipe_stress_canonical_json::MAX_SAFE_INTEGER as u64;
    assert_eq!(MAX, (1u64 << 53) - 1);
    let (model, base) = document("v1_closing");
    let with_count = |side: &str, count: Value| {
        let mut v = base.clone();
        record(&mut v, 0)[side]["primitive_contents"][0]["count"] = count;
        v
    };
    for (what, count) in [("zero", json!(0)), ("negative", json!(-1)), ("max + 1", json!(MAX + 1)), ("a float", json!(1.0)),
                          ("a boolean", json!(true)), ("a string", json!("1")), ("2^64 as a float", json!(18446744073709551616.0f64))] {
        assert_eq!(read(&with_count("subtrahend", count), None, ModelFree::Verify), EVIDENCE, "{what}: model-free");
    }
    for (what, count) in [("zero", json!(0)), ("negative", json!(-1)), ("max + 1", json!(MAX + 1)), ("a float", json!(1.0)),
                          ("a boolean", json!(true)), ("a string", json!("1"))] {
        assert_eq!(read(&with_count("subtrahend", count), Some(&model), ModelFree::Verify), EVIDENCE, "{what}: with the model");
    }
    // The maximum is in the domain: both force entries at MAX stay contained.
    let mut max = with_count("subtrahend", json!(MAX));
    record(&mut max, 0)["minuend"]["primitive_contents"][0]["count"] = json!(MAX);
    assert_eq!(read(&max, None, ModelFree::Verify), "ok", "the maximum is admitted model-free");
    assert_eq!(read(&max, Some(&model), ModelFree::Verify), MODEL, "and is not the model's evidence");
    // A subtrahend count of MAX against the minuend's 1 is in the domain and fails G-1.
    assert_eq!(read(&with_count("subtrahend", json!(MAX)), None, ModelFree::Verify), GATE_DERIVATION);
}
