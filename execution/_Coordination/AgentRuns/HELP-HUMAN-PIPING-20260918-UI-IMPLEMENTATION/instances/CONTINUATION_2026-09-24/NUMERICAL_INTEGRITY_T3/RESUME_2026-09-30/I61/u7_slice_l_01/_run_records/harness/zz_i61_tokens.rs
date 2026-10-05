//! I61 U7 slice L (scratch lane only): Rust standing tokens on the live successors.
//! Reader: retained_precision::validate; carriers: semantic_contract::numerical_use_standing(_with_context),
//! classification_summary. D-U7-4's forms: Rust reads the invocation argument and the requested refs only.
use open_pipe_stress_result_export::{retained_precision as rp, semantic_contract as s};
use serde_json::{json, Value};

#[test]
fn zz_i61_tokens() {
    let (Ok(live), Ok(out)) = (std::env::var("I61_LIVE"), std::env::var("I61_OUT")) else { return };
    let mut all = serde_json::Map::new();
    for mode in ["sparse_interactive", "dense_scrutiny"] {
        let doc: Value = serde_json::from_str(&std::fs::read_to_string(format!("{live}/u3g2_successor_{mode}.json")).unwrap()).unwrap();
        let (src, inv) = (&doc["source"], &doc["invocation"]);
        let req: Vec<Value> = inv["request"]["model"]["load_cases"].as_array().unwrap().iter().map(|c| json!({"ref_type":"load_case","ref_id":c["id"]})).collect();
        let v = rp::validate(src, Some(inv)).unwrap();
        let v0 = rp::validate(src, None).unwrap();
        let withheld = |i: Option<&Value>| -> Vec<Value> { s::classification_summary(src, i).iter().map(|x| x["withheld"].clone()).collect() };
        all.insert(mode.into(), json!({
            "reader": {"with_invocation": {"numerical_eligible": v.numerical_eligible}, "without_invocation": {"numerical_eligible": v0.numerical_eligible}},
            "token": {
                "with_invocation": s::numerical_use_standing_with_context(src, &req, Some(inv)),
                "without_invocation": s::numerical_use_standing(src, &req),
                "d_u7_4_no_native_capture": s::numerical_use_standing_with_context(src, &req, Some(inv)),
                "d_u7_4_stale_current_model": s::numerical_use_standing_with_context(src, &req, Some(inv)),
            },
            "withheld": {"with_invocation": withheld(Some(inv)), "without_invocation": withheld(None)},
        }));
    }
    std::fs::write(out, serde_json::to_string_pretty(&Value::Object(all)).unwrap()).unwrap();
}
