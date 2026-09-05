//! Authenticates the full original document before the typed physics projection.
//! Produces operation proposals only; it never applies or accepts a model.
use open_pipe_stress_canonical_json::canonical_json;
use open_pipe_stress_product_physics::{
    self_weight::{generate_self_weight_operations, SelfWeightRequest},
    PreviewModel,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

fn source_hash(model: &Value) -> String {
    format!(
        "sha256:{:x}",
        Sha256::digest(canonical_json(model).as_bytes())
    )
}
fn envelope(hash: Option<&str>, plan: Value, diagnostics: Vec<Value>) -> Value {
    json!({"document_kind":"openpipestress.desktop.self_weight_plan_outcome",
        "status":if plan.is_null(){"blocked"}else{"ready"},"input_model_unchanged":true,
        "source_model_hash":hash,"plan":plan,"diagnostics":diagnostics,
        "mutation_route":"structured_operations_only","requires_user_acceptance":true})
}
fn blocked(hash: Option<&str>, code: &str, message: &str, remediation: &str) -> Value {
    envelope(
        hash,
        Value::Null,
        vec![json!({"code":code,"severity":"blocking","message":message,
        "remediation":remediation,"affected_objects":[]})],
    )
}
/// Authenticate canonical identity, validate explicit generation options and
/// return a complete draft plan or a structured blocking outcome.
pub fn generate_plan(model: &Value, request: &Value) -> Value {
    if !model.is_object() {
        return blocked(
            None,
            "SELF-WEIGHT-MODEL-SHAPE-INVALID",
            "Model must be a JSON object.",
            "Supply the original model document.",
        );
    }
    if !request.is_object() {
        return blocked(
            None,
            "SELF-WEIGHT-REQUEST-SHAPE-INVALID",
            "Request must be a JSON object.",
            "Supply explicit self-weight generation options.",
        );
    }
    let hash = source_hash(model);
    if request.get("source_model_hash").and_then(Value::as_str) != Some(hash.as_str()) {
        return blocked(
            None,
            "SELF-WEIGHT-SOURCE-HASH-MISMATCH",
            "Original model hash is missing, malformed, or stale.",
            "Recompute the canonical original model hash and regenerate the request.",
        );
    }
    let typed_model: PreviewModel = match serde_json::from_value(model.clone()) {
        Ok(value) => value,
        Err(_) => {
            return blocked(
                None,
                "SELF-WEIGHT-MODEL-SHAPE-INVALID",
                "Model does not match the preview model input shape.",
                "Correct the original model fields before generating a plan.",
            )
        }
    };
    let typed_request: SelfWeightRequest = match serde_json::from_value(request.clone()) {
        Ok(value) => value,
        Err(_) => {
            return blocked(
                Some(&hash),
                "SELF-WEIGHT-REQUEST-SHAPE-INVALID",
                "Request contains missing, invalid, or unsupported generation options.",
                "Supply only the defined request and gravity fields with their expected types.",
            )
        }
    };
    match generate_self_weight_operations(&typed_model,&typed_request) {
        Ok(plan)=>match serde_json::to_value(plan) {
            Ok(plan)=>envelope(Some(&hash),plan,vec![]),
            Err(_)=>blocked(Some(&hash),"SELF-WEIGHT-PLAN-INVALID","Generated plan could not be serialized.","Correct the source quantities and regenerate the proposal."),
        },
        Err(diagnostics)=>envelope(Some(&hash),Value::Null,diagnostics.into_iter().map(|d|json!({
            "code":d.code,"severity":"blocking","message":d.message,
            "remediation":"Correct the referenced explicit inputs and regenerate the proposal.","affected_objects":d.affected_refs})).collect()),
    }
}
/// JSON transport shared by native tests and the feature-gated Wasm export.
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub fn generate_self_weight_plan_json(model_json: &str, request_json: &str) -> String {
    let parsed: Result<(Value, Value), _> = serde_json::from_str(model_json)
        .and_then(|model| serde_json::from_str(request_json).map(|request| (model, request)));
    match parsed {
        Ok((model, request)) => generate_plan(&model, &request),
        Err(_) => blocked(
            None,
            "SELF-WEIGHT-JSON-INVALID",
            "Model or request is not valid JSON.",
            "Supply valid JSON model and request objects.",
        ),
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (Value, Value) {
        let model = json!({"schema_version":"invented","document_kind":"preview","project":{"id":"invented"},
            "analysis_status":{"mechanics":"draft","rule_check":"not_run","professional_acceptance":"not_assessed"},
            "extension":{"retained":"full original"},"nodes":[{"id":"a","position":{"x":0,"y":0,"z":0}},{"id":"b","position":{"x":1,"y":0,"z":0}}],
            "pipe_segments":[{"id":"pipe","from":"a","to":"b","material":"invented","provenance":"invented pipe","section":{
                "outside_diameter":{"value":0.1,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"},"material_density":{"value":1000,"unit":"kg/m^3"}}}],"supports":[]});
        let request = json!({"case_id":"self","label":"Invented","pipe_refs":["pipe"],"gravity":{"value":-7,"unit":"m/s^2","axis":"global_z"},"provenance":"explicit invented input","source_model_hash":source_hash(&model)});
        (model, request)
    }
    fn assert_blocked(out: &Value) {
        assert_eq!(out["status"], "blocked");
        assert!(out["plan"].is_null());
        assert_eq!(out["input_model_unchanged"], true);
        assert!(out.get("applied_model").is_none());
        assert!(out.get("receipt").is_none());
        assert!(out.get("model").is_none());
        assert!(!out["diagnostics"].as_array().unwrap().is_empty());
        for d in out["diagnostics"].as_array().unwrap() {
            assert_eq!(d["severity"], "blocking");
            assert!(d["remediation"].is_string());
            assert!(d["affected_objects"].is_array());
        }
    }
    #[test]
    fn ready_exact_module_and_json_transport_parity() {
        let (m, r) = fixture();
        let before = (m.clone(), r.clone());
        let out = generate_plan(&m, &r);
        assert_eq!(out["status"], "ready");
        assert_eq!(out["source_model_hash"], r["source_model_hash"]);
        assert_eq!(out["diagnostics"], json!([]));
        let direct = generate_self_weight_operations(
            &serde_json::from_value(m.clone()).unwrap(),
            &serde_json::from_value(r.clone()).unwrap(),
        )
        .unwrap();
        assert_eq!(out["plan"], serde_json::to_value(direct).unwrap());
        assert_eq!(
            out,
            serde_json::from_str::<Value>(&generate_self_weight_plan_json(
                &m.to_string(),
                &r.to_string()
            ))
            .unwrap()
        );
        assert_eq!(before, (m, r));
    }
    #[test]
    fn full_original_hash_rejects_stale_unknown_model_field() {
        let (mut m, r) = fixture();
        m["extension"]["retained"] = json!("changed");
        let out = generate_plan(&m, &r);
        assert_blocked(&out);
        assert_eq!(
            out["diagnostics"][0]["code"],
            "SELF-WEIGHT-SOURCE-HASH-MISMATCH"
        );
        let mut refreshed = r;
        refreshed["source_model_hash"] = json!(source_hash(&m));
        assert_eq!(generate_plan(&m, &refreshed)["status"], "ready");
    }
    #[test]
    fn rejects_malformed_hash_shapes_and_unknown_options() {
        let (m, r) = fixture();
        for hash in [
            Value::Null,
            json!(7),
            json!("bad"),
            json!(format!("sha256:{}", "0".repeat(64))),
        ] {
            let mut bad = r.clone();
            bad["source_model_hash"] = hash;
            assert_blocked(&generate_plan(&m, &bad));
        }
        let mut missing = r.clone();
        missing.as_object_mut().unwrap().remove("source_model_hash");
        assert_blocked(&generate_plan(&m, &missing));
        for bad in [Value::Null, json!([]), json!({})] {
            assert_blocked(&generate_plan(&bad, &r));
            assert_blocked(&generate_plan(&m, &bad));
        }
        let mut bad_model = m.clone();
        bad_model["nodes"] = json!("bad");
        let mut corrected = r.clone();
        corrected["source_model_hash"] = json!(source_hash(&bad_model));
        let out = generate_plan(&bad_model, &corrected);
        assert_blocked(&out);
        assert!(out["source_model_hash"].is_null());
        for nested in [false, true] {
            let mut bad = r.clone();
            if nested {
                bad["gravity"]["unsupported"] = json!(true);
            } else {
                bad["unsupported"] = json!(true);
            }
            let out = generate_plan(&m, &bad);
            assert_blocked(&out);
            assert_eq!(
                out["diagnostics"][0]["code"],
                "SELF-WEIGHT-REQUEST-SHAPE-INVALID"
            );
        }
        let mut bad = r.clone();
        bad["gravity"]["value"] = json!(0);
        let out = generate_plan(&m, &bad);
        assert_blocked(&out);
        assert_eq!(out["diagnostics"][0]["code"], "SELF_WEIGHT_INPUT_INVALID");
    }
    #[test]
    fn malformed_json_blocks_and_key_order_is_canonical() {
        let (m, r) = fixture();
        for (a, b) in [("{", r.to_string()), (&m.to_string(), "{".into())] {
            assert_blocked(
                &serde_json::from_str::<Value>(&generate_self_weight_plan_json(a, &b)).unwrap(),
            );
        }
        let reversed = format!(
            "{{{}}}",
            m.as_object()
                .unwrap()
                .iter()
                .rev()
                .map(|(k, v)| format!("{}:{}", serde_json::to_string(k).unwrap(), v))
                .collect::<Vec<_>>()
                .join(",")
        );
        let reordered: Value = serde_json::from_str(&reversed).unwrap();
        assert_eq!(source_hash(&m), source_hash(&reordered));
        assert_eq!(
            generate_plan(&m, &r),
            serde_json::from_str::<Value>(&generate_self_weight_plan_json(
                &reversed,
                &r.to_string()
            ))
            .unwrap()
        );
    }
}
