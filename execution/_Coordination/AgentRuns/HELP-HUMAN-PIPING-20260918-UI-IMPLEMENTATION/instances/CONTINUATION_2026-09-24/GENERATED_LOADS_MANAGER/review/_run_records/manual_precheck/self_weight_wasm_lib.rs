//! Authenticates the full original document before the typed physics projection.
//! Produces operation proposals only; it never applies or accepts a model.
use open_pipe_stress_canonical_json::canonical_json;
use open_pipe_stress_product_physics::{
    self_weight::{generate_self_weight_operations, inspect_applied_self_weight, AppliedSelfWeightState,
        SelfWeightOperationDraft, SelfWeightOperationPlan, SelfWeightRequest},
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
    if request.get("mode").is_some() {
        return refresh_plan(model, &typed_model, request, &hash);
    }
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

fn refresh_plan(model: &Value, typed: &PreviewModel, request: &Value, hash: &str) -> Value {
    let shape_error = || blocked(Some(hash), "SELF-WEIGHT-REQUEST-SHAPE-INVALID",
        "Refresh requires mode, case_id, source_model_hash and an explicit manual_overrides policy.",
        "Use refresh mode and choose block or preserve for modified generated loads.");
    let Some(object) = request.as_object() else { return shape_error(); };
    if object.len() != 4 || object.keys().any(|key| !["mode", "case_id", "source_model_hash", "manual_overrides"].contains(&key.as_str()))
        || request["mode"] != "refresh" || !matches!(request["manual_overrides"].as_str(), Some("block" | "preserve")) {
        return shape_error();
    }
    let Some(case_id) = request["case_id"].as_str().filter(|id| !id.trim().is_empty()) else { return shape_error(); };
    let cases: Vec<_> = model["load_cases"].as_array().into_iter().flatten().filter(|case| case["id"].as_str() == Some(case_id)).collect();
    if cases.len() != 1 {
        return blocked(Some(hash), "SELF-WEIGHT-REFRESH-CASE-INVALID", "Refresh case must resolve exactly once.", "Choose an existing generated self-weight case.");
    }
    let Some(current) = cases[0]["primitive_loads"].as_array() else {
        return blocked(Some(hash), "SELF-WEIGHT-REFRESH-CASE-INVALID", "Refresh case requires a primitive_loads array.", "Repair the stored load case before refreshing.");
    };
    let mut updated = current.clone();
    let mut evidence = Vec::new();
    let mut diagnostics = Vec::new();
    let mut changed = false;
    for check in inspect_applied_self_weight(typed, Some(hash)).into_iter().filter(|check| check.case_id == case_id) {
        let Some(original) = current.get(check.primitive_index) else {
            return blocked(Some(hash), "SELF-WEIGHT-REFRESH-IDENTITY-INVALID", "Generated-load inspection no longer matches the original document.", "Reload the original model and regenerate the plan.");
        };
        evidence.push(json!({"case_id":case_id,"primitive_id":original["id"],"state":check.state,
            "reason_codes":check.diagnostics.iter().map(|d| d.code.as_str()).collect::<Vec<_>>(),
            "manual_overrides":request["manual_overrides"]}));
        match check.state {
            AppliedSelfWeightState::Fresh | AppliedSelfWeightState::ManualOverride => {},
            AppliedSelfWeightState::Stale => {
                let Some(replacement) = check.replacement else {
                    return blocked(Some(hash), "SELF-WEIGHT-REFRESH-UNAVAILABLE", "Current source inputs cannot produce a complete replacement.", "Resolve the source diagnostics before refreshing.");
                };
                if replacement["id"] != original["id"] || replacement["category"] != original["category"]
                    || replacement["target"] != original["target"] || replacement["direction"] != original["direction"]
                    || replacement["dimension"] != original["dimension"] {
                    return blocked(Some(hash), "SELF-WEIGHT-REFRESH-IDENTITY-INVALID", "Refresh would change primitive identity or physical scope.", "Review the modified primitive through the normal authoring route.");
                }
                updated[check.primitive_index]["magnitude"] = replacement["magnitude"].clone();
                updated[check.primitive_index]["provenance"] = replacement["provenance"].clone();
                changed = true;
            },
            AppliedSelfWeightState::Modified if request["manual_overrides"] == "preserve" => {
                let original_generation: Value = match original["provenance"].as_str().and_then(|text| serde_json::from_str::<Value>(text).ok()) {
                    Some(value) if value.is_object() => value,
                    _ => return blocked(Some(hash), "SELF-WEIGHT-LINEAGE-INVALID", "Modified generated load has no valid original generation record.", "Review the invalid lineage before choosing a manual load."),
                };
                updated[check.primitive_index]["provenance"] = Value::String(json!({
                    "method":"manual_override_of_generated_self_weight/v1", "original_generation_provenance":original_generation,
                    "decision":"preserve_modified_generated_load", "source_model_hash":hash
                }).to_string());
                changed = true;
            },
            AppliedSelfWeightState::Modified | AppliedSelfWeightState::Invalid => {
                diagnostics.extend(check.diagnostics.into_iter().map(|d| json!({"code":d.code,"severity":"blocking",
                    "message":d.message,"remediation":"Resolve invalid inputs, or explicitly choose preserve for a valid modified generated load.","affected_objects":d.affected_refs})));
                if diagnostics.is_empty() {
                    diagnostics.push(json!({"code":"SELF-WEIGHT-REFRESH-BLOCKED","severity":"blocking","message":"Generated load requires explicit reconciliation.","remediation":"Review its source or manual override before refreshing.","affected_objects":[original["id"]]}));
                }
            },
        }
    }
    if !diagnostics.is_empty() { return envelope(Some(hash), Value::Null, diagnostics); }
    if !changed {
        return blocked(Some(hash), "SELF-WEIGHT-REFRESH-NOT-NEEDED", "No stale or modified generated load requires refresh in this case.", "Keep current generated loads and explicitly detached manual loads unchanged.");
    }
    let plan = SelfWeightOperationPlan { source_model_hash: hash.into(), source_evidence: evidence.clone(), scope_label:"selected_pipe_mass_only".into(),
        changes: vec![SelfWeightOperationDraft { object_type:"Load".into(), target_ref:case_id.into(), operation_kind:"modify".into(),
            change_kind:"update_load".into(), field_label:"Refresh generated self-weight".into(), field_path:"generated_self_weight".into(),
            before:canonical_json(&Value::Array(current.clone())), after:canonical_json(&Value::Array(updated)), unit:"none".into(), dimension:"dimensionless".into(),
            source_note:json!({"source_model_hash":hash,"manual_overrides":request["manual_overrides"],"source_evidence":evidence}).to_string() }] };
    match serde_json::to_value(plan) {
        Ok(plan) => envelope(Some(hash), plan, vec![]),
        Err(_) => blocked(Some(hash), "SELF-WEIGHT-PLAN-INVALID", "Refresh plan could not be serialized.", "Resolve the source inputs before retrying."),
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

    #[test]
    fn refresh_request_retains_strict_shape_and_original_hash_guards() {
        let (model, _) = fixture();
        let request = json!({"mode":"refresh","case_id":"self","source_model_hash":source_hash(&model),"manual_overrides":"block"});
        let before = model.clone();
        let missing_case = generate_plan(&model, &request);
        assert_blocked(&missing_case);
        assert_eq!(missing_case["diagnostics"][0]["code"], "SELF-WEIGHT-REFRESH-CASE-INVALID");
        for mode in 0..6 {
            let mut invalid = request.clone();
            match mode {
                0 => { invalid.as_object_mut().unwrap().remove("manual_overrides"); },
                1 => invalid["manual_overrides"] = json!("replace"),
                2 => invalid["mode"] = json!("inspect"),
                3 => invalid["case_id"] = json!(7),
                4 => invalid["case_id"] = json!(""),
                _ => invalid["gravity"] = json!({"value":9.81,"unit":"m/s^2","axis":"global_z"}),
            }
            let outcome = generate_plan(&model, &invalid);
            assert_blocked(&outcome);
            assert_eq!(outcome["diagnostics"][0]["code"], "SELF-WEIGHT-REQUEST-SHAPE-INVALID");
        }
        let mut stale = request;
        stale["source_model_hash"] = json!(format!("sha256:{}", "0".repeat(64)));
        let outcome = generate_plan(&model, &stale);
        assert_blocked(&outcome);
        assert_eq!(outcome["diagnostics"][0]["code"], "SELF-WEIGHT-SOURCE-HASH-MISMATCH");
        assert_eq!(model, before);
    }
}
