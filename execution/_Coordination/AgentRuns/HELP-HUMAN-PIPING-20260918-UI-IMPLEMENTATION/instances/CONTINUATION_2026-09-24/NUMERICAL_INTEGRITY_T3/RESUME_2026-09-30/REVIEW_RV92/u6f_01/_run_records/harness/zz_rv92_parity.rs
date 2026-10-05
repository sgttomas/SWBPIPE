//! RV92 (U6f): the Rust column of the three-language parity table. Reads the
//! probe set ($RV92_PROBES/index.json) and writes one JSON line per probe to
//! $RV92_OUT. Review harness only; not part of the candidate.
use open_pipe_stress_result_export::{derivative as d, semantic_contract as s};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::io::Write as _;
use std::time::Instant;

fn r<T>(x: Result<T, String>, f: impl Fn(T) -> String) -> String {
    match x {
        Ok(v) => format!("ok:{}", f(v)),
        Err(e) => format!("err:{}", e.replace(['\t', '\n'], " ")),
    }
}
fn sha(v: &Value) -> String {
    format!("{:x}", Sha256::digest(serde_json::to_vec(v).unwrap()))
}
fn base_document() -> Value {
    serde_json::from_str(include_str!(
        "../../../../fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json"
    ))
    .unwrap()
}
fn derive(raw: &Value) -> Result<Value, String> {
    let model = raw["model_ref"].as_str().unwrap_or("model:absent");
    let carrier = d::checksum(raw, "attested_headless_producer_carrier", d::reference("test_carrier", "rv92"))?;
    let origin = json!({"origin_id":"rv92","origin_class":"attested_headless_producer","qualification_ref":d::reference("test_fixture","not-authentication"),"authentic_producer_available":true,"received_carrier_checksum":carrier,"original_producer_checksum":carrier,"origin_limit":"RV92 parity","actual_model_ref":d::reference("model_payload",model),"mechanics_run_ref":d::reference("mechanics_run",raw["run_id"].as_str().unwrap_or("run:absent")),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null});
    let mut base = base_document();
    base["result_envelope"]["envelope_id"] = json!("envelope:rv92");
    d::derive_document(base, &json!({"project":{"id":model}}), raw, origin, None)
}

#[test]
fn rv92_parity() {
    let (Ok(dir), Ok(out)) = (std::env::var("RV92_PROBES"), std::env::var("RV92_OUT")) else { return };
    let index: Vec<Value> = serde_json::from_str(&std::fs::read_to_string(format!("{dir}/index.json")).unwrap()).unwrap();
    let mut file = std::fs::File::create(out).unwrap();
    for entry in index {
        let p: Value = serde_json::from_str(&std::fs::read_to_string(format!("{dir}/{}", entry["file"].as_str().unwrap())).unwrap()).unwrap();
        let src = &p["source"];
        let inv = if p["invocation"].is_null() { None } else { Some(&p["invocation"]) };
        let requested: Vec<Value> = p["requested"].as_array().unwrap().clone();
        let id = src["producer"]["semantic_contract_id"].as_str().unwrap_or("").to_string();
        let mut o = serde_json::Map::new();
        o.insert("id".into(), p["id"].clone());
        o.insert("transport".into(), json!(r(s::for_source_metadata(src), |(t, v)| format!("{}|{v}", t["semantic_contract_id"].as_str().unwrap_or("?")))));
        o.insert("raw".into(), json!(r(s::for_source(src), |(t, v)| format!("{}|{v}", t["semantic_contract_id"].as_str().unwrap_or("?")))));
        o.insert("standing".into(), json!(s::numerical_use_standing_with_context(src, &requested, inv)));
        o.insert("fresh".into(), json!(s::is_fresh_identity(&id)));
        o.insert("standing_reason".into(), json!(s::standing_reason(src)));
        let t0 = Instant::now();
        let rows = src["results"].as_array().cloned().unwrap_or_default();
        let binding: Vec<Value> = rows.iter().map(|row| json!(s::rule_binding_refusal(src, row))).collect();
        o.insert("binding_ms".into(), json!(t0.elapsed().as_millis() as u64));
        o.insert("binding".into(), Value::Array(binding));
        // The headline rows (summary result_refs), bound as their rows.
        let headline: Vec<Value> = ["max_displacement", "max_open_formula_stress"].iter().map(|k| {
            let rref = &src["summary"][*k]["result_ref"];
            rows.iter().find(|row| row["id"] == *rref).map(|row| json!(s::rule_binding_refusal(src, row))).unwrap_or(json!("absent"))
        }).collect();
        o.insert("headline_binding".into(), Value::Array(headline));
        o.insert("summary".into(), Value::Array(s::classification_summary(src, inv)));
        let derived = derive(src);
        o.insert("derive".into(), json!(r(derived.clone(), |doc| sha(&doc))));
        if let Ok(doc) = &derived {
            o.insert("validate".into(), json!(r(d::validate_document(doc, src), |_| String::new())));
            let e = &doc["result_envelope"];
            o.insert("derived_receipt_equal".into(), json!(e.get("retained_precision").map(|x| serde_json::to_string(x).unwrap() == serde_json::to_string(&src["retained_precision"]).unwrap())));
            let mut reasons: std::collections::BTreeMap<String, u64> = Default::default();
            for x in e["row_disclosures"].as_array().map(Vec::as_slice).unwrap_or(&[]) {
                *reasons.entry(x["reason_code"].as_str().unwrap_or("?").to_string()).or_default() += 1;
            }
            o.insert("disclosures".into(), json!(reasons));
            o.insert("quantity_rows".into(), json!(e["quantity_results"].as_array().map(|a| a.len())));
            // Receipt mutations on the derivative.
            let mut muts = serde_json::Map::new();
            let mut dropped = doc.clone();
            dropped["result_envelope"].as_object_mut().unwrap().remove("retained_precision");
            muts.insert("dropped".into(), json!(r(d::validate_document(&dropped, src), |_| String::new())));
            let mut zero = doc.clone();
            zero["result_envelope"]["retained_precision"]["receipt_sha256"] = json!("0".repeat(64));
            muts.insert("receipt_sha_zero".into(), json!(r(d::validate_document(&zero, src), |_| String::new())));
            let mut seeded = doc.clone();
            seeded["result_envelope"]["retained_precision"] = json!({"receipt_sha256": "0".repeat(64)});
            muts.insert("seeded_other".into(), json!(r(d::validate_document(&seeded, src), |_| String::new())));
            let mut float = doc.clone();
            if let Some(c) = float["result_envelope"]["retained_precision"]["body"]["work"]["charged"].as_u64() {
                float["result_envelope"]["retained_precision"]["body"]["work"]["charged"] = json!(c as f64);
                muts.insert("charged_as_float".into(), json!(r(d::validate_document(&float, src), |_| String::new())));
            }
            // A class disclosure reason swapped into an ordinary reason, and back.
            if let Some(idx) = e["row_disclosures"].as_array().and_then(|a| a.iter().position(|x| x["reason_code"] == d::RETAINED_ABSOLUTE_VERIFIED)) {
                let mut swapped = doc.clone();
                swapped["result_envelope"]["row_disclosures"][idx]["reason_code"] = json!(d::RETAINED_NOT_COVERED);
                muts.insert("absolute_to_not_covered".into(), json!(r(d::validate_document(&swapped, src), |_| String::new())));
                let mut msg = doc.clone();
                msg["result_envelope"]["row_disclosures"][idx]["message"] = json!("edited");
                muts.insert("absolute_message_edit".into(), json!(r(d::validate_document(&msg, src), |_| String::new())));
            }
            o.insert("derivative_mutations".into(), Value::Object(muts));
            // The headless runner's derivative metadata check (runner/headless lib.rs:617-623).
            let mut meta = e.clone();
            meta["schema_version"] = json!("0.2.0");
            o.insert("headless_metadata".into(), json!(r(s::for_source_metadata(&meta), |(t, _)| t["semantic_contract_id"].as_str().unwrap_or("?").to_string())));
        }
        writeln!(file, "{}", Value::Object(o)).unwrap();
    }
}
