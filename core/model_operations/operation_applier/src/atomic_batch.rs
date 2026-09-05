//! Atomic local proposal replay. Intermediate engine applications are private
//! simulations; only an entirely successful explicit apply returns a model.
use serde_json::{json, Map, Value};
use std::collections::HashSet;

/// Simulate a batch through the same engine used for a single operation.
/// Returns no model and records no acceptance, including on success.
pub fn validate_operation_batch(model: &Value, batch: &Value, claimed: Option<&Value>) -> Value {
    run_batch(model, batch, claimed, false)
}

/// Apply all operations to a private evolving document, or return no document.
/// The caller's input remains immutable and persistence remains a separate act.
pub fn apply_operation_batch(model: &Value, batch: &Value, claimed: Option<&Value>) -> Value {
    run_batch(model, batch, claimed, true)
}

fn hash_evidence(model: &Value) -> Value {
    json!({
        "algorithm":"sha256", "canonicalization":crate::BACKEND_CANONICALIZATION,
        "payload_scope":"model_payload", "payload_ref":"model:local_batch_input",
        "hash_status":"computed_local_preview",
        "value":format!("sha256:{}",crate::sha256_hex(&crate::canonical_json(model)))
    })
}

fn diagnostic(code: &str, message: impl Into<String>, refs: Vec<String>) -> Value {
    json!({"id":format!("diagnostic:batch:{code}"),"code":code,"severity":"blocking",
        "message":message.into(),"remediation":"Refresh the current model hash and provide a complete draft structured batch; review before applying.",
        "affected_refs":refs,"source":"operation_batch_preflight"})
}

fn run_batch(model: &Value, batch: &Value, claimed: Option<&Value>, apply: bool) -> Value {
    let initial_hash = hash_evidence(model);
    let initial_backend_hash = initial_hash["value"].clone();
    let batch_id = batch.get("batch_id").and_then(Value::as_str).unwrap_or("");
    let mut diagnostics = Vec::<Value>::new();
    let mut steps = Vec::<Value>::new();
    let operations = match preflight(batch) {
        Ok(operations) => Some(operations),
        Err(error) => {
            diagnostics.push(diagnostic(
                "OP-BATCH-PREFLIGHT-INVALID",
                error,
                vec![batch_id.into()],
            ));
            None
        }
    };

    // Reuse the authoritative single-engine hash comparison. Missing/null hash
    // is stricter for a batch: there is no unbound import/replay path.
    if let Some(claim) = claimed.filter(|v| !v.is_null()) {
        let shape = exact_object(
            claim,
            &["algorithm", "canonicalization", "payload_scope", "value"],
            &["payload_ref", "hash_status"],
        )
        .and_then(|o| {
            for key in ["payload_ref", "hash_status"] {
                if o.contains_key(key) {
                    string(o, key, false)?;
                }
            }
            if let Some(status) = o.get("hash_status") {
                if status.as_str() != Some("computed_local_preview") {
                    return Err("Unsupported claimed hash_status".into());
                }
            }
            Ok(())
        });
        if let Err(error) = shape {
            diagnostics.push(diagnostic(
                "OP-BATCH-HASH-SHAPE-INVALID",
                error,
                vec![batch_id.into()],
            ));
        }
        let mut checker = crate::Checker::new();
        let mut basis = crate::model_basis_evidence(model, Some(claim));
        crate::check_claimed_model_hash(Some(claim), &mut basis, batch_id, &mut checker);
        diagnostics.extend(
            checker
                .diagnostics
                .iter()
                .map(|d| serde_json::to_value(d).expect("diagnostic serializes")),
        );
    } else {
        diagnostics.push(diagnostic(
            "OP-BATCH-MODEL-HASH-REQUIRED",
            "Atomic batches require complete current model hash evidence",
            vec![batch_id.into()],
        ));
    }

    let mut current = model.clone();
    let mut preflight_passed = diagnostics.is_empty();
    if preflight_passed {
        // Opaque token is constructed from the original base, never JSON input.
        match crate::boundary_association::preflight_batch(model, batch) {
            Err(error) => {
                preflight_passed = false;
                diagnostics.push(diagnostic(error.code, error.message, vec![batch_id.into()]))
            }
            Ok(context) => {
                for (index, intent) in operations.expect("preflight passed").iter().enumerate() {
                    let fresh_claim = hash_evidence(&current);
                    let outcome = crate::apply_operation_in_batch(
                        &current,
                        intent,
                        Some(&fresh_claim),
                        &context,
                    );
                    let success = outcome.applied_model.is_some()
                        && outcome.validation.application_status == "applied_to_session_model"
                        && !outcome.diagnostics.iter().any(|d| d.severity == "blocking");
                    // Never serialize the operation outcome itself: it contains
                    // intermediate model, hash, and local acceptance claims.
                    steps.push(json!({
                        "index":index,"operation_id":outcome.operation_id,"change_id":outcome.change_id,
                        "target_object_type":outcome.target_object_type,"target_ref":outcome.target_ref,
                        "change_kind":outcome.change_kind,
                        "validation": {
                            "schema_validation":outcome.validation.schema_validation,
                            "reference_validation":outcome.validation.reference_validation,
                            "unit_validation":outcome.validation.unit_validation,
                            "before_state_validation":outcome.validation.before_state_validation,
                            "diff_preview_status":outcome.validation.diff_preview_status
                        },
                        "diff_preview":outcome.diff_preview,"diagnostics":outcome.diagnostics,
                        "simulation_status":if success {"validated_on_temporary_state"} else {"blocked"}
                    }));
                    diagnostics.extend(
                        outcome
                            .diagnostics
                            .iter()
                            .map(|d| serde_json::to_value(d).expect("diagnostic serializes")),
                    );
                    if !success {
                        if !outcome.diagnostics.iter().any(|d| d.severity == "blocking") {
                            diagnostics.push(diagnostic(
                                "OP-BATCH-STEP-NOT-APPLIED",
                                "A simulated operation did not produce a valid candidate model",
                                vec![outcome.operation_id],
                            ));
                        }
                        break;
                    }
                    current = outcome.applied_model.expect("success checked");
                }
            }
        }
    }
    let completed = operations.is_some_and(|ops| steps.len() == ops.len())
        && !diagnostics
            .iter()
            .any(|d| d.get("severity").and_then(Value::as_str) == Some("blocking"));
    let applied = apply && completed;
    let final_hash = applied.then(|| hash_evidence(&current)["value"].clone());
    json!({
        "schema_version":crate::OPERATION_APPLIER_VERSION,
        "document_kind":"openpipestress.desktop.operation_batch_outcome",
        "batch_id":batch_id,"mode":if apply {"apply"} else {"validate_only"},
        "application_route":"structured_operation_batch",
        "input_model_unchanged":true,
        "validation":{
            "schema_validation":if preflight_passed && steps.iter().all(|s|s["validation"]["schema_validation"] == "passed") {"passed"} else {"blocked"},
            "batch_validation_status":if completed {"passed"} else {"blocked"},
            "diff_preview_status":if completed {"generated"} else {"blocked_by_validation"},
            "application_status":if applied {"applied_to_session_model"} else if apply {"blocked"} else {"not_applied"}
        },
        "simulation_disposition":if !completed {"rolled_back_no_model_published"} else if apply {"committed_as_one_batch"} else {"validation_only_discarded"},
        "diagnostics":diagnostics,"operation_outcomes":steps,
        "initial_model_hash":initial_hash,"initial_backend_hash":initial_backend_hash,
        "input_backend_hash":initial_backend_hash,
        "batch_hash":format!("sha256:{}",crate::sha256_hex(&crate::canonical_json(batch))),
        "submitted_operations":batch.get("operations").cloned().unwrap_or(Value::Null),
        "submitted_operations_trust":"untrusted_submitted_metadata_not_validation_evidence",
        "submitted_initial_model_hash":claimed.cloned().unwrap_or(Value::Null),
        "applied_model":if applied {Some(current)} else {None},
        "applied_model_backend_hash":final_hash,
        "acceptance":if applied {Some(json!({
            "acceptance_basis":"user_initiated_apply_in_local_session",
            "acceptance_is_professional_approval":false,
            "persistence_status":"session_state_only_not_yet_saved"
        }))} else {None},
        "professional_boundary":{
            "human_review_required":true,"software_makes_compliance_claim":false,
            "software_makes_certification_claim":false,"software_makes_sealing_claim":false,
            "software_makes_approval_claim":false,"software_makes_authentication_claim":false
        },
        "audit_boundary":{
            "mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,
            "requires_user_acceptance":true,"input_model_mutated_in_place":false,
            "applied_model_is_new_document":applied,"agent_runtime_binding":"held_D58",
            "source_identity_verification":"not_performed_asserted_metadata_only"
        }
    })
}

fn exact_object<'a>(
    value: &'a Value,
    required: &[&str],
    optional: &[&str],
) -> Result<&'a Map<String, Value>, String> {
    let object = value
        .as_object()
        .ok_or_else(|| "Expected an object".to_string())?;
    for key in required {
        if !object.contains_key(*key) {
            return Err(format!("Missing required field {key}"));
        }
    }
    for key in object.keys() {
        if !required.contains(&key.as_str()) && !optional.contains(&key.as_str()) {
            return Err(format!("Unknown field {key}"));
        }
    }
    Ok(object)
}
fn string<'a>(
    object: &'a Map<String, Value>,
    field: &str,
    allow_empty: bool,
) -> Result<&'a str, String> {
    object
        .get(field)
        .and_then(Value::as_str)
        .filter(|s| allow_empty || !s.trim().is_empty())
        .ok_or_else(|| {
            format!(
                "{field} must be {}string",
                if allow_empty { "a " } else { "a nonempty " }
            )
        })
}
fn boolean(object: &Map<String, Value>, field: &str, expected: bool) -> Result<(), String> {
    if object.get(field).and_then(Value::as_bool) != Some(expected) {
        Err(format!("{field} must be {expected}"))
    } else {
        Ok(())
    }
}

/// Strict proposal import validation; no submitted validation labels are trusted.
fn preflight(batch: &Value) -> Result<&Vec<Value>, String> {
    let batch = exact_object(batch, &["batch_id", "operations"], &[])?;
    string(batch, "batch_id", false)?;
    let operations = batch["operations"]
        .as_array()
        .filter(|v| !v.is_empty())
        .ok_or_else(|| "operations must be a nonempty array".to_string())?;
    let mut operation_ids = HashSet::new();
    let mut change_ids = HashSet::new();
    for (index, operation) in operations.iter().enumerate() {
        preflight_operation(operation).map_err(|e| format!("operations[{index}]: {e}"))?;
        if !operation_ids.insert(operation["operation_id"].as_str().unwrap()) {
            return Err("Duplicate operation_id".into());
        }
        if !change_ids.insert(operation["change"]["change_id"].as_str().unwrap()) {
            return Err("Duplicate change_id".into());
        }
    }
    Ok(operations)
}
fn preflight_operation(value: &Value) -> Result<(), String> {
    let op = exact_object(
        value,
        &[
            "operation_id",
            "operation_kind",
            "operation_status",
            "author_type",
            "target",
            "change",
            "validation",
            "audit_boundary",
            "professional_boundary",
            "rationale",
        ],
        &["queue_id", "source"],
    )?;
    for key in [
        "operation_id",
        "operation_kind",
        "operation_status",
        "author_type",
    ] {
        string(op, key, false)?;
    }
    if op.contains_key("queue_id") {
        string(op, "queue_id", false)?;
    }
    if op["operation_status"].as_str() != Some("proposed") {
        return Err("operation_status must be proposed".into());
    }
    if !matches!(
        op["operation_kind"].as_str(),
        Some("create" | "connect" | "delete" | "insert" | "modify")
    ) {
        return Err("Unsupported operation_kind".into());
    }
    let author = string(op, "author_type", false)?;
    if !matches!(author, "user" | "agent") {
        return Err("author_type must be user or agent".into());
    }
    string(op, "rationale", author == "user")?;
    match op.get("source") {
        Some(source) => {
            let source = exact_object(
                source,
                &["source_ref", "source_channel", "source_role"],
                &[],
            )?;
            for key in ["source_ref", "source_channel", "source_role"] {
                string(source, key, false)?;
            }
        }
        None if author == "agent" => {
            return Err("Agent proposals require a complete source object".into())
        }
        None => {}
    }
    let target = exact_object(&op["target"], &["object_type", "ref"], &[])?;
    string(target, "ref", false)?;
    if !matches!(
        string(target, "object_type", false)?,
        "Material"
            | "Section"
            | "Node"
            | "Element"
            | "Component"
            | "Support"
            | "Load"
            | "Combination"
    ) {
        return Err("Unsupported target object_type".into());
    }
    let change = exact_object(
        &op["change"],
        &[
            "change_id",
            "change_kind",
            "field_label",
            "field_path",
            "before",
            "after",
            "unit",
            "dimension",
            "source_note",
        ],
        &[],
    )?;
    for key in [
        "change_id",
        "change_kind",
        "field_label",
        "field_path",
        "unit",
        "dimension",
    ] {
        string(change, key, false)?;
    }
    for key in ["before", "after", "source_note"] {
        string(change, key, true)?;
    }
    let validation = exact_object(
        &op["validation"],
        &[
            "schema_validation",
            "constraint_validation",
            "unit_validation",
            "diff_preview_status",
            "application_status",
        ],
        &[],
    )?;
    for key in [
        "schema_validation",
        "constraint_validation",
        "unit_validation",
        "diff_preview_status",
        "application_status",
    ] {
        string(validation, key, false)?;
    }
    let audit = exact_object(
        &op["audit_boundary"],
        &[
            "mutation_route",
            "direct_model_mutation_allowed",
            "requires_user_acceptance",
            "mutates_accepted_model_state",
        ],
        &[],
    )?;
    if audit["mutation_route"].as_str() != Some("structured_operations_only") {
        return Err("Only structured operation routing is accepted".into());
    }
    boolean(audit, "direct_model_mutation_allowed", false)?;
    boolean(audit, "requires_user_acceptance", true)?;
    boolean(audit, "mutates_accepted_model_state", false)?;
    let professional = exact_object(
        &op["professional_boundary"],
        &[
            "human_review_required",
            "software_makes_compliance_claim",
            "software_makes_certification_claim",
            "software_makes_sealing_claim",
            "software_makes_approval_claim",
            "software_makes_authentication_claim",
        ],
        &[],
    )?;
    boolean(professional, "human_review_required", true)?;
    for key in [
        "software_makes_compliance_claim",
        "software_makes_certification_claim",
        "software_makes_sealing_claim",
        "software_makes_approval_claim",
        "software_makes_authentication_claim",
    ] {
        boolean(professional, key, false)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn model() -> Value {
        json!({"schema_version":"0.2.0","project":{"id":"project:batch-fixture","units":{"length":"m","force":"N","pressure":"Pa","temperature":"K","angle":"rad"}},"nodes":[{"id":"node:n","label":"N","position":{"x":0,"y":0,"z":0}}],"pipe_segments":[],"supports":[],"materials":[],"sections":[],"components":[],"load_cases":[],"combinations":[]})
    }
    fn intent(
        id: &str,
        kind: &str,
        path: &str,
        after: Value,
        unit: &str,
        dimension: &str,
    ) -> Value {
        json!({
            "operation_id":format!("op:{id}"),"operation_kind":"create","operation_status":"proposed","author_type":"user",
            "target":{"object_type":"Load","ref":"case:new"},
            "change":{"change_id":format!("change:{id}"),"change_kind":kind,"field_label":path,"field_path":path,"before":"not_present","after":serde_json::to_string(&after).unwrap(),"unit":unit,"dimension":dimension,"source_note":"invented test input"},
            "validation":{"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run","diff_preview_status":"not_generated","application_status":"not_applied"},
            "audit_boundary":{"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,"requires_user_acceptance":true,"mutates_accepted_model_state":false},
            "professional_boundary":{"human_review_required":true,"software_makes_compliance_claim":false,"software_makes_certification_claim":false,"software_makes_sealing_claim":false,"software_makes_approval_claim":false,"software_makes_authentication_claim":false},
            "rationale":"Invented explicit authoring test"
        })
    }
    fn batch() -> Value {
        let case = intent(
            "case",
            "create_load_case",
            "load_cases",
            json!({"id":"case:new","label":"Test case","kind":"operating","status":"draft","provenance":"invented","primitive_loads":[]}),
            "none",
            "dimensionless",
        );
        let load = intent(
            "load",
            "create_primitive_load",
            "primitive_loads",
            json!({"id":"load:new","category":"concentrated_force","target":{"type":"node","node":"node:n"},"direction":"global_y","magnitude":{"value":20,"unit":"N"},"dimension":"force","provenance":"invented"}),
            "N",
            "force",
        );
        json!({"batch_id":"batch:test","operations":[case,load]})
    }
    fn assert_no_nested_engine_claims(outcome: &Value) {
        fn walk(value: &Value) {
            if let Some(object) = value.as_object() {
                for (key, value) in object {
                    assert!(
                        ![
                            "applied_model",
                            "applied_model_backend_hash",
                            "model_basis",
                            "initial_model_hash",
                            "acceptance",
                            "acceptance_basis",
                            "application_status",
                            "professional_boundary",
                            "audit_boundary",
                            "mode",
                            "application_route"
                        ]
                        .contains(&key.as_str()),
                        "forbidden nested key {key}"
                    );
                    walk(value);
                }
            } else if let Some(array) = value.as_array() {
                for value in array {
                    walk(value);
                }
            }
        }
        walk(&outcome["operation_outcomes"]);
    }
    fn assert_unpublished(outcome: &Value) {
        assert!(outcome["applied_model"].is_null());
        assert!(outcome["applied_model_backend_hash"].is_null());
        assert!(outcome["acceptance"].is_null());
        assert_no_nested_engine_claims(outcome);
    }
    #[test]
    fn dependent_create_case_and_load_apply_atomically_and_deterministically() {
        let model = model();
        let original = model.clone();
        let batch = batch();
        let claim = hash_evidence(&model);
        let outcome = apply_operation_batch(&model, &batch, Some(&claim));
        assert_eq!(
            outcome["validation"]["application_status"], "applied_to_session_model",
            "{outcome:#}"
        );
        assert_eq!(
            outcome["applied_model"]["load_cases"][0]["primitive_loads"][0]["id"],
            "load:new"
        );
        assert_eq!(outcome["operation_outcomes"].as_array().unwrap().len(), 2);
        assert_eq!(outcome["initial_model_hash"], claim);
        assert_eq!(outcome["initial_backend_hash"], claim["value"]);
        assert_ne!(outcome["applied_model_backend_hash"], claim["value"]);
        assert_eq!(
            outcome["acceptance"]["acceptance_is_professional_approval"],
            false
        );
        assert_eq!(outcome, apply_operation_batch(&model, &batch, Some(&claim)));
        assert_eq!(model, original);
        assert_no_nested_engine_claims(&outcome);
    }
    #[test]
    fn validation_simulates_dependencies_without_any_application_or_acceptance() {
        let model = model();
        let batch = batch();
        let outcome = validate_operation_batch(&model, &batch, Some(&hash_evidence(&model)));
        assert_eq!(
            outcome["validation"]["batch_validation_status"], "passed",
            "{outcome:#}"
        );
        assert_eq!(outcome["validation"]["application_status"], "not_applied");
        assert_eq!(
            outcome["simulation_disposition"],
            "validation_only_discarded"
        );
        assert_unpublished(&outcome);
    }
    #[test]
    fn late_reference_failure_rolls_back_and_stops_replay() {
        let model = model();
        let original = model.clone();
        let mut batch = batch();
        let mut payload: Value =
            serde_json::from_str(batch["operations"][1]["change"]["after"].as_str().unwrap())
                .unwrap();
        payload["target"]["node"] = json!("node:absent");
        batch["operations"][1]["change"]["after"] = json!(payload.to_string());
        let mut third = batch["operations"][0].clone();
        third["operation_id"] = json!("op:unreached");
        third["change"]["change_id"] = json!("change:unreached");
        batch["operations"].as_array_mut().unwrap().push(third);
        let outcome = apply_operation_batch(&model, &batch, Some(&hash_evidence(&model)));
        assert_eq!(outcome["operation_outcomes"].as_array().unwrap().len(), 2);
        assert_eq!(
            outcome["operation_outcomes"][0]["simulation_status"],
            "validated_on_temporary_state"
        );
        assert_eq!(
            outcome["operation_outcomes"][1]["simulation_status"],
            "blocked"
        );
        assert_eq!(
            outcome["simulation_disposition"],
            "rolled_back_no_model_published"
        );
        assert!(!outcome["diagnostics"].as_array().unwrap().is_empty());
        assert_eq!(model, original);
        assert_unpublished(&outcome);
    }
    #[test]
    fn late_stale_before_discards_earlier_success() {
        let model = model();
        let mut batch = batch();
        batch["operations"][1]["change"]["before"] = json!("already_present");
        let outcome = apply_operation_batch(&model, &batch, Some(&hash_evidence(&model)));
        assert_eq!(outcome["validation"]["batch_validation_status"], "blocked");
        assert_eq!(
            outcome["operation_outcomes"][0]["simulation_status"],
            "validated_on_temporary_state"
        );
        assert_unpublished(&outcome);
    }
    #[test]
    fn mandatory_complete_initial_hash_rejects_null_malformed_mismatch_before_replay() {
        let model = model();
        let batch = batch();
        let good = hash_evidence(&model);
        let mut mismatch = good.clone();
        mismatch["value"] = json!(format!("sha256:{}", "0".repeat(64)));
        let mut malformed = good.clone();
        malformed["algorithm"] = json!("sha512");
        let mut extra = good.clone();
        extra["context"] = json!({"trusted":true});
        for claim in [
            None,
            Some(Value::Null),
            Some(json!({"value":good["value"]})),
            Some(mismatch),
            Some(malformed),
            Some(extra),
        ] {
            let outcome = apply_operation_batch(&model, &batch, claim.as_ref());
            assert!(
                outcome["operation_outcomes"].as_array().unwrap().is_empty(),
                "{outcome:#}"
            );
            assert_unpublished(&outcome);
            assert_eq!(outcome["initial_model_hash"], good);
        }
        let mut minimal = good;
        minimal.as_object_mut().unwrap().remove("payload_ref");
        minimal.as_object_mut().unwrap().remove("hash_status");
        assert_eq!(
            validate_operation_batch(&model, &batch, Some(&minimal))["validation"]
                ["batch_validation_status"],
            "passed"
        );
    }
    #[test]
    fn duplicate_ids_and_empty_batches_are_preflight_failures() {
        let model = model();
        let claim = hash_evidence(&model);
        let batch = batch();
        for field in ["operation_id", "change_id"] {
            let mut duplicate = batch.clone();
            if field == "operation_id" {
                duplicate["operations"][1][field] = duplicate["operations"][0][field].clone();
            } else {
                duplicate["operations"][1]["change"][field] =
                    duplicate["operations"][0]["change"][field].clone();
            }
            let outcome = apply_operation_batch(&model, &duplicate, Some(&claim));
            assert!(outcome["operation_outcomes"].as_array().unwrap().is_empty());
            assert_unpublished(&outcome);
        }
        for malformed in [
            json!({"batch_id":"batch:x","operations":[]}),
            json!({"batch_id":" ","operations":batch["operations"]}),
            json!({"batch_id":"batch:x","operations":batch["operations"],"trusted_context":{}}),
        ] {
            let outcome = apply_operation_batch(&model, &malformed, Some(&claim));
            assert_unpublished(&outcome);
            assert_eq!(outcome["validation"]["schema_validation"], "blocked");
        }
    }
    #[test]
    fn author_metadata_is_preserved_and_user_agent_have_same_semantics() {
        let model = model();
        let claim = hash_evidence(&model);
        let user = batch();
        let mut agent = user.clone();
        for op in agent["operations"].as_array_mut().unwrap() {
            op["author_type"] = json!("agent");
            op["source"] = json!({"source_ref":"import:local","source_channel":"offline_json","source_role":"proposal_author"});
            op["rationale"] = json!("Explicit proposed change from offline file");
        }
        let user_outcome = apply_operation_batch(&model, &user, Some(&claim));
        let agent_outcome = apply_operation_batch(&model, &agent, Some(&claim));
        assert_eq!(
            user_outcome["applied_model"],
            agent_outcome["applied_model"]
        );
        assert!(
            !agent_outcome["applied_model"].is_null(),
            "{agent_outcome:#}"
        );
        assert_eq!(
            user_outcome["operation_outcomes"],
            agent_outcome["operation_outcomes"]
        );
        assert_eq!(agent_outcome["submitted_operations"], agent["operations"]);
        assert_ne!(user_outcome["batch_hash"], agent_outcome["batch_hash"]);
        assert_no_nested_engine_claims(&agent_outcome);
    }
    #[test]
    fn preflight_rejects_each_boundary_flag_missing_agent_attribution_and_bad_source_shape() {
        let model = model();
        let claim = hash_evidence(&model);
        let batch = batch();
        let pointers = [
            "/operation_status",
            "/author_type",
            "/audit_boundary/mutation_route",
            "/audit_boundary/direct_model_mutation_allowed",
            "/audit_boundary/requires_user_acceptance",
            "/audit_boundary/mutates_accepted_model_state",
            "/professional_boundary/human_review_required",
            "/professional_boundary/software_makes_compliance_claim",
            "/professional_boundary/software_makes_certification_claim",
            "/professional_boundary/software_makes_sealing_claim",
            "/professional_boundary/software_makes_approval_claim",
            "/professional_boundary/software_makes_authentication_claim",
        ];
        for pointer in pointers {
            let mut b = batch.clone();
            let slot = b["operations"][1].pointer_mut(pointer).unwrap();
            *slot = if let Some(value) = slot.as_bool() {
                json!(!value)
            } else {
                json!("invalid")
            };
            let outcome = apply_operation_batch(&model, &b, Some(&claim));
            assert!(
                outcome["operation_outcomes"].as_array().unwrap().is_empty(),
                "{pointer}"
            );
            assert_unpublished(&outcome);
        }
        for source in [
            Value::Null,
            json!({"source_ref":"r"}),
            json!({"source_ref":"r","source_channel":"c","source_role":" "}),
            json!({"source_ref":"r","source_channel":"c","source_role":"a","trusted":true}),
        ] {
            for author in ["user", "agent"] {
                let mut b = batch.clone();
                b["operations"][0]["author_type"] = json!(author);
                b["operations"][0]["source"] = source.clone();
                assert!(
                    apply_operation_batch(&model, &b, Some(&claim))["operation_outcomes"]
                        .as_array()
                        .unwrap()
                        .is_empty()
                );
            }
        }
        let mut b = batch.clone();
        b["operations"][0]["author_type"] = json!("agent");
        assert!(preflight(&b).is_err());
        b["operations"][0]["source"] =
            json!({"source_ref":"r","source_channel":"c","source_role":"a"});
        b["operations"][0]["rationale"] = json!(" ");
        assert!(preflight(&b).is_err());
    }
    #[test]
    fn submitted_validation_claims_are_untrusted_and_cannot_override_engine_failure() {
        let model = model();
        let mut batch = batch();
        batch["operations"][1]["change"]["before"] = json!("stale");
        for op in batch["operations"].as_array_mut().unwrap() {
            for value in op["validation"].as_object_mut().unwrap().values_mut() {
                *value = json!("already_approved");
            }
        }
        let outcome = apply_operation_batch(&model, &batch, Some(&hash_evidence(&model)));
        assert_eq!(outcome["submitted_operations"], batch["operations"]);
        assert_eq!(
            outcome["operation_outcomes"][1]["simulation_status"],
            "blocked"
        );
        assert_unpublished(&outcome);
    }
}
