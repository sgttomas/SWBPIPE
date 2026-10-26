//! W-16 slice b (DEL-05-02 Design `exact-combinations.md` §6, §8): the
//! pressure-2 reader's combination pass over the shared combination corpus
//! (`fixtures/results/pressure_v3_combination_reader_corpus.json`, written by
//! PP `w16_exact_combinations.rs`). Every envelope is admitted with and without
//! its model; every forgery of an expression, gate, recovery, standing, row,
//! source ref, section basis, withheld or missing entry or diagnostic is
//! refused, by the evidence pass alone or by the model binding.
use open_pipe_stress_result_export::reader_verdict;
use serde_json::{json, Value};

const CORPUS: &str = include_str!("../../../../fixtures/results/pressure_v3_combination_reader_corpus.json");

fn corpus() -> Vec<Value> {
    serde_json::from_str::<Value>(CORPUS).unwrap()["cases"].as_array().unwrap().clone()
}
fn entry(prefix: &str) -> (Value, Value) {
    let case = corpus().into_iter().find(|c| c["label"].as_str().unwrap().starts_with(prefix)).unwrap();
    (case["envelope"].clone(), case["model"].clone())
}
fn read(source: &Value, model: Option<&Value>) -> Result<(), String> {
    let verdict = reader_verdict::validate("pressure", source, model, None);
    if verdict["ok"] == true {
        Ok(())
    } else {
        Err(verdict["error"]["code"].as_str().unwrap().to_string())
    }
}
fn combination_row_index(source: &Value, pred: impl Fn(&Value) -> bool) -> usize {
    source["results"].as_array().unwrap().iter().position(|r| r["basis_ref"]["ref_type"] == "combination" && pred(r)).expect("a combination row")
}
fn refused(source: &Value, model: Option<&Value>, code: &str, what: &str) {
    assert_eq!(read(source, model), Err(code.to_string()), "{what}");
}

#[test]
fn every_corpus_envelope_is_admitted_with_and_without_its_model() {
    let cases = corpus();
    assert_eq!(cases.len(), 4);
    for case in &cases {
        let envelope = &case["envelope"];
        assert!(envelope["contract_evidence"]["exact_combinations"].as_array().is_some_and(|r| !r.is_empty()));
        assert_eq!(read(envelope, None), Ok(()), "{}", case["label"]);
        assert_eq!(read(envelope, Some(&case["model"])), Ok(()), "{}", case["label"]);
        // §5: the standing stays with the primitive cases.
        let standing = reader_verdict::standing(envelope, Some(&case["model"]), None);
        let mut direct = envelope.clone();
        direct["contract_evidence"].as_object_mut().unwrap().remove("exact_combinations");
        direct["results"].as_array_mut().unwrap().retain(|r| r["basis_ref"]["ref_type"] != "combination");
        direct["diagnostics"].as_array_mut().unwrap().retain(|d| !d["id"].as_str().unwrap().starts_with("diagnostic:exact-combination:"));
        let mut model = case["model"].clone();
        model["combinations"] = json!([]);
        let without = reader_verdict::standing(&direct, Some(&model), None);
        assert_eq!(standing["eligible"], without["eligible"], "{}", case["label"]);
        assert_eq!(standing["token"], without["token"], "{}", case["label"]);
    }
}

#[test]
fn forged_rows_are_refused() {
    let (source, model) = entry("W16B line");
    // A value one ulp off.
    let mut forged = source.clone();
    let i = combination_row_index(&source, |r| r["kind"] == "pipe_wall_axial_force_v2");
    let v = forged["results"][i]["value"].as_f64().unwrap();
    forged["results"][i]["value"] = json!(f64::from_bits(v.to_bits() + 1));
    refused(&forged, None, "SOURCE_PHYSICS_COMBINATION_ROW", "a value one ulp off");
    // An omitted row and an extra row.
    let mut forged = source.clone();
    forged["results"].as_array_mut().unwrap().remove(i);
    refused(&forged, None, "SOURCE_PHYSICS_COMBINATION_ROW_COVERAGE", "an omitted row");
    let mut forged = source.clone();
    let mut extra = forged["results"][i].clone();
    extra["id"] = json!("result:combination:forged");
    forged["results"].as_array_mut().unwrap().push(extra);
    refused(&forged, None, "SOURCE_PHYSICS_COMBINATION_ROW_COVERAGE", "an extra row");
    // A forged source ref, a forged metadata basis and a swapped section basis.
    let mut forged = source.clone();
    forged["results"][i]["source_result_refs"][0] = json!("result:forged");
    refused(&forged, None, "SOURCE_PHYSICS_COMBINATION_ROW", "a forged source ref");
    let j = combination_row_index(&source, |r| r["kind"] == "pipe_axial_membrane_stress_v2");
    let mut forged = source.clone();
    let basis = forged["results"][j]["metadata"]["basis"].as_str().unwrap().to_string();
    let swapped = if basis.ends_with("nominal_section") { basis.replace("nominal_section", "reduced_section") } else { basis.replace("reduced_section", "nominal_section") };
    forged["results"][j]["metadata"]["basis"] = json!(swapped);
    refused(&forged, None, "SOURCE_PHYSICS_COMBINATION_ROW", "a swapped section basis");
    let mut forged = source.clone();
    forged["results"][i]["metadata"]["sign_convention"] = json!("certified combination");
    refused(&forged, None, "SOURCE_PHYSICS_COMBINATION_ROW", "a forged sign convention");
    // A combination row without its record.
    let mut forged = source.clone();
    forged["contract_evidence"].as_object_mut().unwrap().remove("exact_combinations");
    refused(&forged, None, "SOURCE_PHYSICS_COMBINATION_EVIDENCE_MISSING", "rows without the record");
    let mut forged = source.clone();
    forged["contract_evidence"]["exact_combinations"] = json!([]);
    refused(&forged, None, "SOURCE_PHYSICS_COMBINATION_EVIDENCE_SHAPE", "an empty record list");
}

#[test]
fn forged_records_are_refused() {
    let (source, model) = entry("W16B line");
    let mutate = |f: &dyn Fn(&mut Value)| {
        let mut forged = source.clone();
        f(&mut forged["contract_evidence"]["exact_combinations"][0]);
        forged
    };
    // The expression: another factor re-derives other rows; with the model, the binding refuses it first.
    let forged = mutate(&|r| r["expression"]["terms"][1]["factor"] = json!(1.25));
    assert!(read(&forged, None).unwrap_err().starts_with("SOURCE_PHYSICS_COMBINATION_"));
    let forged = mutate(&|r| r["expression"]["terms"][1]["factor"] = json!(1.3000000000000003));
    assert!(read(&forged, None).unwrap_err().starts_with("SOURCE_PHYSICS_COMBINATION_"));
    // The gate, the standing, the source-recovery exclusion, an operand's standing.
    refused(&mutate(&|r| r["gate"] = json!({"withheld": true, "reason": "COMBINATION_MODULUS_BASIS_MIXED"})), None, "SOURCE_PHYSICS_COMBINATION_GATE", "a forged gate");
    refused(&mutate(&|r| r["standing"] = json!("certified")), None, "SOURCE_PHYSICS_COMBINATION_STANDING", "a forged standing");
    refused(&mutate(&|r| r["source_recovery"] = json!("attempted")), None, "SOURCE_PHYSICS_COMBINATION_SOURCE_RECOVERY", "a forged exclusion");
    refused(&mutate(&|r| r["operands"][1]["numerical_quality"]["solve_quality"] = json!("sensitive")), None, "SOURCE_PHYSICS_COMBINATION_OPERAND_STANDING", "a forged operand standing");
    refused(&mutate(&|r| r["operands"][0]["modulus_basis_ref"] = json!("point:forged")), None, "SOURCE_PHYSICS_COMBINATION_GATE", "a forged modulus selection");
    // A recovery witness, a row's operand sources, a missing source, a withheld entry.
    refused(&mutate(&|r| r["recoveries"][0]["value"] = json!(1.0)), None, "SOURCE_PHYSICS_COMBINATION_RECOVERY", "a forged witness value");
    refused(&mutate(&|r| {
        let i = r["recoveries"].as_array().unwrap().iter().position(|w| w["sign"] == "negated").unwrap();
        r["recoveries"][i]["sign"] = json!("unchanged");
    }), None, "SOURCE_PHYSICS_COMBINATION_RECOVERY", "a forged witness sign");
    refused(&mutate(&|r| { r["recoveries"].as_array_mut().unwrap().pop(); }), None, "SOURCE_PHYSICS_COMBINATION_RECOVERY", "an omitted witness");
    refused(&mutate(&|r| {
        let i = r["rows"].as_array().unwrap().iter().position(|row| !row["sources"].as_array().unwrap().is_empty()).unwrap();
        r["rows"][i]["sources"] = json!([]);
    }), None, "SOURCE_PHYSICS_COMBINATION_ROW_EVIDENCE", "forged operand sources");
    refused(&mutate(&|r| { r["missing_sources"].as_array_mut().unwrap().pop(); }), None, "SOURCE_PHYSICS_COMBINATION_MISSING_SOURCES", "an omitted missing source");
    refused(&mutate(&|r| r["withheld"] = json!([{"name": "support-action:9:support:a", "reason": "load-case algebra refused"}])), None, "SOURCE_PHYSICS_COMBINATION_WITHHELD", "a forged withheld row");
    refused(&mutate(&|r| { r.as_object_mut().unwrap().insert("certified".into(), json!(true)); }), None, "SOURCE_PHYSICS_COMBINATION_RECORD_SHAPE", "an extra record key");
    // The model binding: another factor, or a model without the combination.
    let mut other = model.clone();
    other["combinations"][0]["terms"][1]["factor"] = json!(1.5);
    refused(&source, Some(&other), "SOURCE_PHYSICS_COMBINATION_MODEL_EXPRESSION", "the model's factor differs");
    let mut fewer = model.clone();
    fewer["combinations"].as_array_mut().unwrap().pop();
    refused(&source, Some(&fewer), "SOURCE_PHYSICS_COMBINATION_MODEL_COVERAGE", "the model lacks a combination");
}

#[test]
fn forged_gates_and_withheld_rows_are_refused() {
    // The connector document: a gated record publishes no row and its gate is re-derived.
    let (source, _) = entry("W16B connector");
    let records = source["contract_evidence"]["exact_combinations"].as_array().unwrap();
    let gated = records.iter().position(|r| r["gate"]["withheld"] == true).unwrap();
    assert_eq!(records[gated]["gate"]["reason"], "COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE");
    let mut forged = source.clone();
    forged["contract_evidence"]["exact_combinations"][gated]["gate"] = json!({"withheld": false, "reason": null});
    refused(&forged, None, "SOURCE_PHYSICS_COMBINATION_GATE", "a gated combination stated as admitted");
    let admitted = records.iter().position(|r| r["gate"]["withheld"] == false).unwrap();
    let mut forged = source.clone();
    forged["contract_evidence"]["exact_combinations"][admitted]["gate"] = json!({"withheld": true, "reason": "COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE"});
    refused(&forged, None, "SOURCE_PHYSICS_COMBINATION_GATE", "an admitted combination stated as gated");
    // Its installed reference removed from the connector record: the gate no longer holds.
    let mut forged = source.clone();
    forged["contract_evidence"]["connector"][0]["q_ref"][0] = json!(0.0);
    assert!(read(&forged, None).is_err());
    // A removed gate diagnostic.
    let mut forged = source.clone();
    forged["diagnostics"].as_array_mut().unwrap().retain(|d| d["code"] != "COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE");
    refused(&forged, None, "SOURCE_PHYSICS_COMBINATION_DIAGNOSTICS", "a removed gate diagnostic");
    // The tie rods: the withheld names and their diagnostic.
    let (source, _) = entry("W16B MO-F");
    let record = &source["contract_evidence"]["exact_combinations"][0];
    assert_eq!(record["withheld"].as_array().unwrap().iter().filter(|w| w["reason"] == "tie-rod row").count(), 2);
    let mut forged = source.clone();
    forged["contract_evidence"]["exact_combinations"][0]["withheld"].as_array_mut().unwrap().pop();
    refused(&forged, None, "SOURCE_PHYSICS_COMBINATION_WITHHELD", "an omitted rod");
    let mut forged = source.clone();
    forged["diagnostics"].as_array_mut().unwrap().retain(|d| d["code"] != "LOAD_COMBINATION_ROW_WITHHELD");
    refused(&forged, None, "SOURCE_PHYSICS_COMBINATION_DIAGNOSTICS", "a removed withheld diagnostic");
    // The load state: a mixed-stiffness gate rests on the per-case materials.
    let (source, model) = entry("W16B load state");
    let mut forged = source.clone();
    let records = forged["contract_evidence"]["exact_combinations"].as_array().unwrap();
    assert_eq!(records[0]["gate"]["reason"], "COMBINATION_RESOLVED_STIFFNESS_MIXED");
    forged["contract_evidence"]["exact_combinations"][0]["gate"] = json!({"withheld": false, "reason": null});
    refused(&forged, None, "SOURCE_PHYSICS_COMBINATION_GATE", "mixed stiffness stated as admitted");
    assert_eq!(read(&source, Some(&model)), Ok(()));
}

/// The pressure-source-1 composite never carries a combination: its direct
/// reader refuses the record by its closed evidence keys.
#[test]
fn a_composite_with_a_combination_record_is_refused() {
    let raw: Value = serde_json::from_str(include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json")).unwrap();
    assert_eq!(raw["producer"]["semantic_contract_id"], "openpipestress.result_semantics/0.3.0/pressure-source-1");
    let request: Value = serde_json::from_str(include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json")).unwrap();
    let mut forged = raw.clone();
    let (line, _) = entry("W16B line");
    forged["contract_evidence"]["exact_combinations"] = line["contract_evidence"]["exact_combinations"].clone();
    let verdict = reader_verdict::validate("pressure-source", &forged, None, Some(&json!({"request": request, "solver_mode": "sparse_interactive"})));
    assert_eq!(verdict["ok"], false, "{verdict}");
}
