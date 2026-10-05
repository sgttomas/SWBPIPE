//! RV88 (U6a review): canonical round trip of a derivative, then validate_document
//! against the raw source. Both lanes; base APIs only. Writes to $RV88_RT_OUT.
use open_pipe_stress_canonical_json::canonical_json;
use open_pipe_stress_result_export::derivative as d;
use serde_json::{json, Value};
use std::fmt::Write as _;

fn derive(raw: &Value) -> Result<Value, String> {
    let model = raw["model_ref"].as_str().unwrap();
    let carrier = d::checksum(raw, "attested_headless_producer_carrier", d::reference("test_carrier", "rv88"))?;
    let origin = json!({"origin_id":"rv88","origin_class":"attested_headless_producer","qualification_ref":d::reference("test_fixture","not-authentication"),"authentic_producer_available":true,"received_carrier_checksum":carrier,"original_producer_checksum":carrier,"origin_limit":"RV88","actual_model_ref":d::reference("model_payload",model),"mechanics_run_ref":d::reference("mechanics_run",raw["run_id"].as_str().unwrap()),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null});
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json");
    let mut base: Value = serde_json::from_str(&std::fs::read_to_string(p).unwrap()).unwrap();
    base["result_envelope"]["envelope_id"] = json!("envelope:rv88");
    d::derive_document(base, &json!({"project":{"id":model}}), raw, origin, None)
}

#[test]
fn rv88_roundtrip() {
    let Ok(out) = std::env::var("RV88_RT_OUT") else { return };
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures/results");
    let mut log = String::new();
    for name in ["preview_physics_connected_sparse.json", "preview_physics_connected_dense.json", "physics_connected_mechanics_sparse.json"] {
        let raw: Value = serde_json::from_str(&std::fs::read_to_string(root.join(name)).unwrap()).unwrap();
        match derive(&raw) {
            Ok(doc) => {
                let rt: Value = serde_json::from_str(&canonical_json(&doc)).unwrap();
                writeln!(log, "{name}\tin_memory={:?}\tcanonical_roundtrip={:?}", d::validate_document(&doc, &raw), d::validate_document(&rt, &raw)).unwrap();
            }
            Err(e) => writeln!(log, "{name}\tderive_err={e}").unwrap(),
        }
    }
    std::fs::write(out, log).unwrap();
}
