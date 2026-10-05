//! I66 U7 slice F, the oracle-diff control (Rust; scratch lanes only, never committed):
//! the lane's reader and carriers on every shared input. Per input: the reader's outcome
//! (pass, or gate and code) and classifications digest, invocation_bound,
//! numerical_eligible, the carrier token (numerical_use_standing_with_context), the
//! classification summary, the header-only transport and, for inputs marked `binding`,
//! every row's binding refusal; for milestone inputs also the derivative's bytes and its
//! validation. Inputs: I66_U7F_INPUTS; output: I66_U7F_OUT.
use open_pipe_stress_result_export::{derivative as d, retained_precision as rp, semantic_contract as s};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

fn digest(text: &str) -> String {
    format!("{:x}", Sha256::digest(text.as_bytes()))[..16].to_string()
}
fn derive(raw: &Value) -> Result<Value, String> {
    let mut base: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json"
    ))
    .unwrap();
    let model = raw["model_ref"].as_str().unwrap();
    let carrier = d::checksum(raw, "attested_headless_producer_carrier", d::reference("test_carrier", "preview-physics-retained"))?;
    let origin = json!({"origin_id":"retained-precision-carrier-test","origin_class":"attested_headless_producer","qualification_ref":d::reference("test_fixture","not-authentication"),"authentic_producer_available":true,"received_carrier_checksum":carrier,"original_producer_checksum":carrier,"origin_limit":"Pinned producer test bytes; carrier contract test only","actual_model_ref":d::reference("model_payload",model),"mechanics_run_ref":d::reference("mechanics_run",raw["run_id"].as_str().unwrap()),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null});
    base["result_envelope"]["envelope_id"] = json!("envelope:retained-precision-carrier-test");
    d::derive_document(base, &json!({"project":{"id":model}}), raw, origin, None)
}

#[test]
#[ignore]
fn zz_i66_u7r_dump() {
    let inputs: Vec<Value> = serde_json::from_str(&std::fs::read_to_string(std::env::var("I66_U7F_INPUTS").unwrap()).unwrap()).unwrap();
    let mut out = serde_json::Map::new();
    for record in &inputs {
        let (source, invocation) = (&record["source"], record["invocation"].as_object().map(|_| &record["invocation"]));
        let requested: Vec<Value> = record["requested"].as_array().unwrap().clone();
        let mut row = serde_json::Map::new();
        match rp::validate(source, invocation) {
            Ok(v) => {
                row.insert("reader".into(), json!("pass"));
                row.insert("classes".into(), json!(digest(&format!("{:?}", v.classifications))));
                row.insert("invocation_bound".into(), json!(v.invocation_bound));
                row.insert("numerical_eligible".into(), json!(v.numerical_eligible));
            }
            Err(e) => {
                row.insert("reader".into(), json!(format!("{}:{}", e.gate, e.code)));
            }
        }
        row.insert("carrier_token".into(), json!(s::numerical_use_standing_with_context(source, &requested, invocation)));
        let own: Vec<Value> = invocation.and_then(|i| i["request"]["model"]["load_cases"].as_array()).map(|cs| cs.iter().map(|c| json!({"ref_type":"load_case","ref_id":c["id"]})).collect()).unwrap_or_default();
        row.insert("token_with_own_cases".into(), json!(s::numerical_use_standing_with_context(source, &own, invocation)));
        row.insert("summary".into(), json!(s::classification_summary(source, invocation, &requested)));
        row.insert(
            "transport".into(),
            match s::for_source_metadata(source) {
                Ok((table, version)) => json!(["ok", table["semantic_contract_id"], version]),
                Err(e) => json!(["err", e]),
            },
        );
        if record["binding"] == true {
            let refusals: Vec<Option<&str>> = source["results"].as_array().map_or(vec![], |rows| rows.iter().map(|r| s::rule_binding_refusal(source, r)).collect());
            row.insert("binding".into(), json!(digest(&format!("{refusals:?}"))));
        }
        if record["milestone"].is_string() {
            match derive(source) {
                Ok(doc) => {
                    row.insert("derivative".into(), json!(digest(&String::from_utf8(serde_json::to_vec(&doc).unwrap()).unwrap())));
                    row.insert("derivative_validate".into(), json!(d::validate_document(&doc, source).map_err(|e| e)));
                }
                Err(e) => {
                    row.insert("derivative".into(), json!(["err", e]));
                }
            }
        }
        out.insert(record["key"].as_str().unwrap().to_string(), Value::Object(row));
    }
    std::fs::write(std::env::var("I66_U7F_OUT").unwrap(), serde_json::to_vec_pretty(&Value::Object(out)).unwrap()).unwrap();
}
