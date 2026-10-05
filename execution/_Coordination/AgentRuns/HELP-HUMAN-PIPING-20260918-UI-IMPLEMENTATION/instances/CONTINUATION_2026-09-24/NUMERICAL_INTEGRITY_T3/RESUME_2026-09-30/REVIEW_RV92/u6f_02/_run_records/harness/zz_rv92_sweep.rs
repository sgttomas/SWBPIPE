//! RV92 (U6f): existing-behaviour sweep, Rust carriers. Base-compatible APIs
//! only, so the same file runs in the base and candidate lanes. Review harness.
use open_pipe_stress_result_export::{derivative as d, semantic_contract as s};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::io::Write as _;

fn r<T>(x: Result<T, String>, f: impl Fn(T) -> String) -> String {
    match x {
        Ok(v) => format!("ok:{}", f(v)),
        Err(e) => format!("err:{}", e.replace(['\t', '\n'], " ")),
    }
}
fn sha(v: &Value) -> String {
    format!("{:x}", Sha256::digest(serde_json::to_vec(v).unwrap()))
}
fn derive(raw: &Value) -> Result<Value, String> {
    let model = raw["model_ref"].as_str().unwrap_or("model:absent");
    let carrier = d::checksum(raw, "attested_headless_producer_carrier", d::reference("test_carrier", "rv92"))?;
    let origin = json!({"origin_id":"rv92","origin_class":"attested_headless_producer","qualification_ref":d::reference("test_fixture","not-authentication"),"authentic_producer_available":true,"received_carrier_checksum":carrier,"original_producer_checksum":carrier,"origin_limit":"RV92 sweep","actual_model_ref":d::reference("model_payload",model),"mechanics_run_ref":d::reference("mechanics_run",raw["run_id"].as_str().unwrap_or("run:absent")),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null});
    let mut base: Value = serde_json::from_str(include_str!("../../../../fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json")).unwrap();
    base["result_envelope"]["envelope_id"] = json!("envelope:rv92");
    d::derive_document(base, &json!({"project":{"id":model}}), raw, origin, None)
}

#[test]
fn rv92_sweep() {
    let (Ok(dir), Ok(out)) = (std::env::var("RV92_SWEEP"), std::env::var("RV92_OUT")) else { return };
    let index: Vec<Value> = serde_json::from_str(&std::fs::read_to_string(format!("{dir}/index.json")).unwrap()).unwrap();
    let mut file = std::fs::File::create(out).unwrap();
    for entry in index {
        let group = entry["group"].as_str().unwrap();
        if !matches!(group, "raw" | "raw_injected" | "results_doc") {
            continue;
        }
        let p: Value = serde_json::from_str(&std::fs::read_to_string(format!("{dir}/{}", entry["file"].as_str().unwrap())).unwrap()).unwrap();
        let src = &p["source"];
        let mut o = serde_json::Map::new();
        o.insert("id".into(), p["id"].clone());
        if group == "results_doc" {
            let mut meta = src["result_envelope"].clone();
            meta["schema_version"] = json!("0.2.0");
            o.insert("headless_metadata".into(), json!(r(s::for_source_metadata(&meta), |(t, _)| t["semantic_contract_id"].as_str().unwrap_or("?").to_string())));
            writeln!(file, "{}", Value::Object(o)).unwrap();
            continue;
        }
        let inv = if p["invocation"].is_null() { None } else { Some(&p["invocation"]) };
        let requested: Vec<Value> = p["requested"].as_array().unwrap().clone();
        let id = src["producer"]["semantic_contract_id"].as_str().unwrap_or("").to_string();
        o.insert("transport".into(), json!(r(s::for_source_metadata(src), |(t, v)| format!("{}|{v}", t["semantic_contract_id"].as_str().unwrap_or("?")))));
        o.insert("raw".into(), json!(r(s::for_source(src), |(t, v)| format!("{}|{v}", t["semantic_contract_id"].as_str().unwrap_or("?")))));
        o.insert("standing_nq".into(), json!(s::numerical_use_standing(src, &requested)));
        o.insert("standing_empty".into(), json!(s::numerical_use_standing(src, &[])));
        if let Some(inv) = inv {
            o.insert("standing_inv".into(), json!(s::numerical_use_standing_with_context(src, &requested, Some(inv))));
        }
        o.insert("fresh".into(), json!(s::is_fresh_identity(&id)));
        o.insert("standing_reason".into(), json!(s::standing_reason(src)));
        let rows = src["results"].as_array().cloned().unwrap_or_default();
        let binding: Vec<Value> = rows.iter().map(|row| json!(s::rule_binding_refusal(src, row))).collect();
        o.insert("binding".into(), Value::Array(binding));
        let derived = derive(src);
        o.insert("derive".into(), json!(r(derived.clone(), |doc| sha(&doc))));
        if let Ok(doc) = &derived {
            o.insert("validate".into(), json!(r(d::validate_document(doc, src), |_| String::new())));
        }
        writeln!(file, "{}", Value::Object(o)).unwrap();
    }
}
