//! RV88 (U6a review): the slice, by independent derivation. Compiles in both
//! lanes (only APIs present at base). Writes the successor derivative (cand)
//! and the projection's derivative (both lanes) to $RV88_SLICE_OUT for the
//! Python comparator, which uses the receipt's own lists as the class oracle.
use open_pipe_stress_canonical_json::canonical_json;
use open_pipe_stress_result_export::{derivative as d, retained_precision as rp, semantic_contract as s};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const MODES: [(&str, &str); 2] = [
    ("sparse_interactive", "ac6986b0680e0df9d88c33a5bf4635372fc3b83cbb9080da44e6d32dbdca59dc"),
    ("dense_scrutiny", "6cd1d249e5352aaffbd2b7d7349c74a1d0e0572df35500f66be49c3cad95c9b5"),
];

fn fixture(mode: &str, sha: &str) -> (Value, Value) {
    let dir = std::env::var("RV88_FIXTURES").unwrap();
    let bytes = std::fs::read(format!("{dir}/retained_precision_milestone_successor_{mode}.json")).unwrap();
    assert_eq!(format!("{:x}", Sha256::digest(&bytes)), sha);
    let doc: Value = serde_json::from_slice(&bytes).unwrap();
    (doc["source"].clone(), doc["invocation"].clone())
}
fn projection(raw: &Value) -> Value {
    let mut p = raw.clone();
    p.as_object_mut().unwrap().remove("retained_precision");
    p["producer"]["semantic_contract_id"] = json!("openpipestress.result_semantics/0.3.0/preview-physics-1");
    p["formulation_basis"]["profile_id"] = json!("product_preview_mechanics_v1");
    for r in p["results"].as_array_mut().unwrap() {
        r.as_object_mut().unwrap().remove("recovery_method");
    }
    p
}
fn template() -> Value {
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json");
    serde_json::from_str(&std::fs::read_to_string(p).unwrap()).unwrap()
}
fn derive(raw: &Value) -> Result<Value, String> {
    let model = raw["model_ref"].as_str().unwrap();
    let carrier = d::checksum(raw, "attested_headless_producer_carrier", d::reference("test_carrier", "rv88-slice"))?;
    let origin = json!({"origin_id":"rv88-slice","origin_class":"attested_headless_producer","qualification_ref":d::reference("test_fixture","not-authentication"),"authentic_producer_available":true,"received_carrier_checksum":carrier,"original_producer_checksum":carrier,"origin_limit":"RV88 slice","actual_model_ref":d::reference("model_payload",model),"mechanics_run_ref":d::reference("mechanics_run",raw["run_id"].as_str().unwrap()),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null});
    let mut base = template();
    base["result_envelope"]["envelope_id"] = json!("envelope:rv88-slice");
    d::derive_document(base, &json!({"project":{"id":model}}), raw, origin, None)
}

#[test]
fn rv88_slice() {
    let Ok(out) = std::env::var("RV88_SLICE_OUT") else { return };
    let lane = std::env::var("RV88_LANE").unwrap();
    std::fs::create_dir_all(&out).unwrap();
    let mut log = String::new();
    for (mode, sha) in MODES {
        let (src, inv) = fixture(mode, sha);
        let proj = projection(&src);
        // The projection is an existing identity: it must derive and validate in both lanes.
        let pdoc = derive(&proj).unwrap();
        d::validate_document(&pdoc, &proj).unwrap();
        std::fs::write(format!("{out}/{lane}_{mode}_projection.json"), canonical_json(&pdoc)).unwrap();
        log += &format!("{mode}\tprojection_for_source\t{:?}\n", s::for_source(&proj).map(|(t, v)| (t["semantic_contract_id"].clone(), v)));
        log += &format!("{mode}\tsuccessor_for_source\t{:?}\n", s::for_source(&src).map(|(t, v)| (t["semantic_contract_id"].clone(), v)));
        log += &format!("{mode}\tsuccessor_for_source_metadata\t{:?}\n", s::for_source_metadata(&src).map(|(t, v)| (t["semantic_contract_id"].clone(), v)));
        let refs: Vec<Value> = inv["request"]["model"]["load_cases"].as_array().unwrap().iter().map(|c| json!({"ref_type":"load_case","ref_id":c["id"]})).collect();
        log += &format!("{mode}\tstanding_none\t{}\n", s::numerical_use_standing(&src, &refs));
        log += &format!("{mode}\tstanding_inv\t{}\n", s::numerical_use_standing_with_context(&src, &refs, Some(&inv)));
        log += &format!("{mode}\tstanding_inv_norefs\t{}\n", s::numerical_use_standing_with_context(&src, &[], Some(&inv)));
        match derive(&src) {
            Err(e) => log += &format!("{mode}\tderive\terr:{e}\n"),
            Ok(doc) => {
                log += &format!("{mode}\tderive\tok\n");
                log += &format!("{mode}\tvalidate_document\t{:?}\n", d::validate_document(&doc, &src));
                let e = &doc["result_envelope"];
                // Byte-equal receipt: canonical bytes and compact serde bytes.
                log += &format!("{mode}\treceipt_canonical_equal\t{}\n", canonical_json(&e["retained_precision"]) == canonical_json(&src["retained_precision"]));
                log += &format!("{mode}\treceipt_serde_equal\t{}\n", serde_json::to_vec(&e["retained_precision"]).unwrap() == serde_json::to_vec(&src["retained_precision"]).unwrap());
                // Back out: reattach to the raw source; revalidate with and without the invocation.
                let mut back = src.clone();
                back["retained_precision"] = e["retained_precision"].clone();
                let a = rp::validate(&src, Some(&inv)).unwrap();
                let b = rp::validate(&back, Some(&inv)).unwrap();
                log += &format!("{mode}\tback_out_equal\t{}\n", a == b);
                log += &format!("{mode}\tback_out_eligible\t{}\n", b.numerical_eligible);
                // A derivative validated against its own projection (receipt present, base source).
                log += &format!("{mode}\tvalidate_against_projection\t{:?}\n", d::validate_document(&doc, &proj));
                std::fs::write(format!("{out}/{lane}_{mode}_successor.json"), canonical_json(&doc)).unwrap();
                std::fs::write(format!("{out}/{lane}_{mode}_receipt_from_derivative.json"), canonical_json(&e["retained_precision"])).unwrap();
            }
        }
    }
    std::fs::write(format!("{out}/{lane}_slice.tsv"), log).unwrap();
}
