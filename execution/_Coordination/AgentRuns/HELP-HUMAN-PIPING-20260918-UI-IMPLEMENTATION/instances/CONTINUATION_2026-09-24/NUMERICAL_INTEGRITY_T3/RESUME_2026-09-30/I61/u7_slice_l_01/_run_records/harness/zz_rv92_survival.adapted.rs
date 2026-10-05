//! RV92 (U6f): the receipt's survival across the three languages' carriers.
//! Step "a": derive and validate each milestone; write the derivative.
//! Step "b": take every carrier output's receipt (Rust derivative, Python and
//! TS AnalysisRun records), put it back on the raw source, and revalidate.
use open_pipe_stress_result_export::{derivative as d, retained_precision as rp, semantic_contract as s};
use serde_json::{json, Value};
use std::io::Write as _;

fn derive(raw: &Value) -> Result<Value, String> {
    let model = raw["model_ref"].as_str().unwrap();
    let carrier = d::checksum(raw, "attested_headless_producer_carrier", d::reference("test_carrier", "rv92"))?;
    let origin = json!({"origin_id":"rv92","origin_class":"attested_headless_producer","qualification_ref":d::reference("test_fixture","not-authentication"),"authentic_producer_available":true,"received_carrier_checksum":carrier,"original_producer_checksum":carrier,"origin_limit":"RV92 survival","actual_model_ref":d::reference("model_payload",model),"mechanics_run_ref":d::reference("mechanics_run",raw["run_id"].as_str().unwrap()),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null});
    let mut base: Value = serde_json::from_str(include_str!("../../../../fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json")).unwrap();
    base["result_envelope"]["envelope_id"] = json!("envelope:rv92");
    d::derive_document(base, &json!({"project":{"id":model}}), raw, origin, None)
}
fn milestone(mode: &str) -> (Value, Value) {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../fixtures/results/");
    let doc: Value = serde_json::from_str(&std::fs::read_to_string(format!("{root}retained_precision_milestone_successor_{mode}.json")).unwrap()).unwrap();
    (doc["source"].clone(), doc["invocation"].clone())
}
fn requested(inv: &Value) -> Vec<Value> {
    inv["request"]["model"]["load_cases"].as_array().unwrap().iter().map(|c| json!({"ref_type":"load_case","ref_id":c["id"]})).collect()
}

#[test]
fn rv92_survival() {
    let (Ok(dir), Ok(step)) = (std::env::var("RV92_SURVIVAL"), std::env::var("RV92_STEP")) else { return };
    let mut log = std::fs::OpenOptions::new().create(true).append(true).open(format!("{dir}/rust_step_{step}.jsonl")).unwrap();
    for mode in ["sparse_interactive", "dense_scrutiny"] {
        let (src, inv) = milestone(mode);
        let original = serde_json::to_string(&src["retained_precision"]).unwrap();
        if step == "a" {
            let doc = derive(&src).unwrap();
            d::validate_document(&doc, &src).unwrap();
            std::fs::write(format!("{dir}/rust_derivative_{mode}.json"), serde_json::to_vec(&doc).unwrap()).unwrap();
            writeln!(log, "{}", json!({"mode":mode,"derived":true,"receipt_byte_equal": serde_json::to_string(&doc["result_envelope"]["retained_precision"]).unwrap() == original})).unwrap();
            continue;
        }
        let base_validation = rp::validate(&src, Some(&inv)).unwrap();
        for (carrier, path, pointer) in [
            ("rust_derivative", format!("{dir}/rust_derivative_{mode}.json"), "/result_envelope/retained_precision"),
            ("python_analysis_run", format!("{dir}/py_ar_{mode}.json"), "/analysis_run/retained_precision"),
            ("ts_analysis_run", format!("{dir}/ts_ar_{mode}.json"), "/analysis_run/retained_precision"),
        ] {
            let Ok(text) = std::fs::read_to_string(&path) else {
                writeln!(log, "{}", json!({"mode":mode,"carrier":carrier,"missing":true})).unwrap();
                continue;
            };
            let doc: Value = serde_json::from_str(&text).unwrap();
            let receipt = doc.pointer(pointer).cloned().unwrap_or(Value::Null);
            let mut back = src.clone();
            back["retained_precision"] = receipt.clone();
            let v = rp::validate(&back, Some(&inv));
            let same = v.as_ref().map(|v| v.classifications.len() == base_validation.classifications.len() && v.publication_sha256 == base_validation.publication_sha256 && v.invocation_bound && v.numerical_eligible == base_validation.numerical_eligible);
            writeln!(log, "{}", json!({"mode":mode,"carrier":carrier,
                "receipt_byte_equal": serde_json::to_string(&receipt).unwrap() == original,
                "revalidated": same.clone().unwrap_or(false),
                "error": v.err().map(|e| format!("{}:{}", e.gate, e.code)),
                "standing": s::numerical_use_standing_with_context(&back, &requested(&inv), Some(&inv)),
                "standing_no_invocation": s::numerical_use_standing_with_context(&back, &requested(&inv), None)})).unwrap();
            if carrier == "rust_derivative" {
                // The derivative revalidates against its source after the round trip.
                writeln!(log, "{}", json!({"mode":mode,"carrier":"rust_derivative_validate","result": d::validate_document(&doc, &back).map(|_| "ok".to_string()).unwrap_or_else(|e| e)})).unwrap();
            }
        }
    }
}
