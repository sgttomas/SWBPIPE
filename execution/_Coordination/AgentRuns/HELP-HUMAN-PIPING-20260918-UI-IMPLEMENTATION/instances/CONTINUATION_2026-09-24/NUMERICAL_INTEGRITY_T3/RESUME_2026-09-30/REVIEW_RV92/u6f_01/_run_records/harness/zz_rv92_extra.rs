//! RV92 (U6f): two extra checks. (1) A non-successor derivative whose
//! disclosure claims a successor class code: does validate_document refuse it?
//! (2) The cost of one accepted-reader validation and of binding every row.
use open_pipe_stress_result_export::{derivative as d, retained_precision as rp, semantic_contract as s};
use serde_json::{json, Value};
use std::io::Write as _;
use std::time::Instant;

fn derive(raw: &Value) -> Result<Value, String> {
    let model = raw["model_ref"].as_str().unwrap();
    let carrier = d::checksum(raw, "attested_headless_producer_carrier", d::reference("test_carrier", "rv92"))?;
    let origin = json!({"origin_id":"rv92","origin_class":"attested_headless_producer","qualification_ref":d::reference("test_fixture","not-authentication"),"authentic_producer_available":true,"received_carrier_checksum":carrier,"original_producer_checksum":carrier,"origin_limit":"RV92 extra","actual_model_ref":d::reference("model_payload",model),"mechanics_run_ref":d::reference("mechanics_run",raw["run_id"].as_str().unwrap()),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null});
    let mut base: Value = serde_json::from_str(include_str!("../../../../fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json")).unwrap();
    base["result_envelope"]["envelope_id"] = json!("envelope:rv92");
    d::derive_document(base, &json!({"project":{"id":model}}), raw, origin, None)
}

#[test]
fn rv92_extra() {
    let Ok(out) = std::env::var("RV92_EXTRA_OUT") else { return };
    let mut log = std::fs::File::create(out).unwrap();
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../fixtures/results/");
    let pp1: Value = serde_json::from_str(&std::fs::read_to_string(format!("{root}preview_physics_invented_sparse.json")).unwrap()).unwrap();
    let doc = derive(&pp1).unwrap();
    d::validate_document(&doc, &pp1).unwrap();
    for code in [d::RETAINED_ABSOLUTE_VERIFIED, d::RETAINED_NOT_COVERED] {
        let mut claimed = doc.clone();
        claimed["result_envelope"]["row_disclosures"][0]["reason_code"] = json!(code);
        let verdict = d::validate_document(&claimed, &pp1).map(|_| "ok".to_string()).unwrap_or_else(|e| e);
        writeln!(log, "{}", json!({"check":"non_successor_class_code","code":code,"validate_document":verdict})).unwrap();
        let mut other = doc.clone();
        other["result_envelope"]["row_disclosures"][0]["reason_code"] = json!("review_metadata_incomplete");
        let verdict = d::validate_document(&other, &pp1).map(|_| "ok".to_string()).unwrap_or_else(|e| e);
        writeln!(log, "{}", json!({"check":"non_successor_other_ordinary_code","validate_document":verdict})).unwrap();
    }
    let ms: Value = serde_json::from_str(&std::fs::read_to_string(format!("{root}retained_precision_milestone_successor_sparse_interactive.json")).unwrap()).unwrap();
    let (src, inv) = (&ms["source"], &ms["invocation"]);
    let t = Instant::now();
    for _ in 0..10 { rp::validate(src, Some(inv)).unwrap(); }
    let per = t.elapsed().as_secs_f64() * 100.0;
    let t = Instant::now();
    let rows = src["results"].as_array().unwrap();
    let n = rows.iter().filter(|row| s::rule_binding_refusal(src, row).is_some()).count();
    let bind = t.elapsed().as_secs_f64() * 1000.0;
    let t = Instant::now();
    let doc = derive(src).unwrap();
    d::validate_document(&doc, src).unwrap();
    let derive_ms = t.elapsed().as_secs_f64() * 1000.0;
    writeln!(log, "{}", json!({"check":"cost","profile": if cfg!(debug_assertions) {"debug"} else {"release"},"validate_ms":per,"binding_all_rows_ms":bind,"rows":rows.len(),"refused":n,"derive_and_validate_ms":derive_ms})).unwrap();
}
