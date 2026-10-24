//! Source-independent checks of a package's DEL-08-04 0.3.0 result member
//! (DEL-08-01 Design `report-package-current-results.md` §2.3 items 0-3 and
//! the reader). The producer and the reader share them. They are form,
//! header-dispatch, self-hash and equality checks. They are not signatures,
//! source authentication or row accounting, which need the source and stay
//! with the producer (`bindings`). Standard claim fence applies (claims
//! taxonomy per DEC-081).

use crate::schema_eval::{Catalog, RESULTS_V03};
use open_pipe_stress_result_export::derivative::{checksum, guard_json, reference};
use open_pipe_stress_result_export::semantic_contract::{for_export_metadata, PREVIEW_PHYSICS_ID, RESULT_EXPORT_VERSION};
use serde_json::Value;

/// The semantic contracts a report package admits (§2.3 item 0): the
/// pressure-free route only. Exact-route results stay refused under owner
/// ruling 10A until W-11.
pub const ADMITTED_SEMANTIC_CONTRACTS: [&str; 1] = [PREVIEW_PHYSICS_ID];

pub const ROUTE_NOT_ADMITTED: &str = "REPORT-PACKAGE-ROUTE-NOT-ADMITTED";
pub const RESULT_FORM_INVALID: &str = "REPORT-PACKAGE-RESULT-FORM-INVALID";
pub const RESULT_HEADER_INVALID: &str = "REPORT-PACKAGE-RESULT-HEADER-INVALID";
pub const DERIVATIVE_HASH_MISMATCH: &str = "REPORT-PACKAGE-DERIVATIVE-HASH-MISMATCH";
pub const AUDIT_BINDING_MISMATCH: &str = "REPORT-PACKAGE-AUDIT-BINDING-MISMATCH";

/// The 0.3.0 schema's provenance record definition.
const PROVENANCE_DEF: &str = "/$defs/Provenance";

const RULE_TOKENS: [&str; 3] = ["RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED"];

/// The document's producer semantic-contract id, if it has one.
pub fn producer_contract_id(document: &Value) -> Option<&str> {
    document.pointer("/result_envelope/producer/semantic_contract_id").and_then(Value::as_str)
}

/// Item 0, the route preflight: an id outside the admitted set, an unknown id
/// or a missing one is refused by the route code, before any form check.
pub fn check_route_admitted(document: &Value) -> Result<(), String> {
    match producer_contract_id(document) {
        Some(id) if ADMITTED_SEMANTIC_CONTRACTS.contains(&id) => Ok(()),
        Some(id) => Err(format!("{ROUTE_NOT_ADMITTED}: the report package admits only preview-physics-2; found {id}")),
        None => Err(format!("{ROUTE_NOT_ADMITTED}: the result member names no producer semantic contract")),
    }
}

/// Item 1, the complete 0.3.0 contract: I-JSON values, the schema closure and
/// the invariants the schema cannot state (carried over from
/// `result_export::validate_result_envelope`).
pub fn validate_result_export_form(document: &Value) -> Result<(), String> {
    let fail = |detail: String| Err(format!("{RESULT_FORM_INVALID}: {detail}"));
    if let Err(detail) = guard_json(document) {
        return fail(detail);
    }
    // The schema pass also returns where the schema types a provenance record
    // (`$defs/Provenance`), so the completeness invariant below reads exactly
    // those records and never a member that merely has that name inside
    // unconstrained raw metadata.
    let provenance_records = match Catalog::results_v03().validate_marking(RESULTS_V03, document, RESULTS_V03, PROVENANCE_DEF) {
        Ok(records) => records,
        Err(failure) => return fail(failure.to_string()),
    };
    if document["schema_version"] != RESULT_EXPORT_VERSION
        || document["result_envelope"]["schema_version"] != RESULT_EXPORT_VERSION
        || document["deliverable_id"] != "DEL-08-04"
    {
        return fail("schema versions must both be 0.3.0 and the deliverable DEL-08-04".to_string());
    }
    let envelope = &document["result_envelope"];
    let statuses: Vec<&str> = envelope["analysis_status"]
        .as_array()
        .map(|items| items.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();
    if !statuses.contains(&"MECHANICS_SOLVED") || !statuses.contains(&"HUMAN_REVIEW_REQUIRED") {
        return fail("analysis_status must hold MECHANICS_SOLVED and HUMAN_REVIEW_REQUIRED".to_string());
    }
    if statuses.iter().filter(|status| RULE_TOKENS.contains(status)).count() > 1 {
        return fail("analysis_status holds more than one rule token".to_string());
    }
    for path in &provenance_records {
        let complete = document.pointer(path).and_then(Value::as_object).is_some_and(|record| {
            record.values().all(|field| field.as_str().is_none_or(|text| !text.trim().is_empty()))
        });
        if !complete {
            return fail(format!("provenance at {path} is incomplete"));
        }
    }
    for key in ["solver_name", "solver_version", "solver_build_ref"] {
        if envelope["solver_version"][key].as_str().is_none_or(|text| text.trim().is_empty()) {
            return fail(format!("solver_version.{key} is empty"));
        }
    }
    Ok(())
}

/// Item 2, header dispatch: the transported producer header is read as a
/// current mechanics-result header and `semantic_contract_ref` names it.
pub fn check_transported_header(document: &Value) -> Result<&'static Value, String> {
    for_export_metadata(&document["result_envelope"])
        .map(|(table, _)| table)
        .map_err(|detail| format!("{RESULT_HEADER_INVALID}: {detail}"))
}

/// Item 3, the self-hash: the derivative hash over the document without its own
/// hash, as `result_export::derivative` computes it.
pub fn check_self_hash(document: &Value) -> Result<(), String> {
    let envelope_id = document
        .pointer("/result_envelope/envelope_id")
        .and_then(Value::as_str)
        .ok_or_else(|| format!("{DERIVATIVE_HASH_MISMATCH}: the result member has no envelope_id"))?;
    let mut payload = document.clone();
    let removed = payload
        .pointer_mut("/result_envelope/reproducibility")
        .and_then(Value::as_object_mut)
        .and_then(|reproducibility| reproducibility.remove("derivative_hash"));
    if removed.is_none() {
        return Err(format!("{DERIVATIVE_HASH_MISMATCH}: the result member has no derivative hash"));
    }
    let expected = checksum(
        &payload,
        "derivative_document_excludes_own_hash",
        reference("derivative_document", envelope_id),
    )
    .map_err(|detail| format!("{DERIVATIVE_HASH_MISMATCH}: {detail}"))?;
    if document["result_envelope"]["reproducibility"]["derivative_hash"] != expected {
        return Err(format!("{DERIVATIVE_HASH_MISMATCH}: the derivative hash does not verify"));
    }
    Ok(())
}

/// Items 1-3, applied by the reader to every result member of every package.
pub fn check_result_member_form(document: &Value) -> Result<(), String> {
    validate_result_export_form(document)?;
    check_transported_header(document)?;
    check_self_hash(document)
}

/// The unblocked-package equalities shared by the writer and the reader: the
/// audit member's id names the result member's audit reference, and the model
/// hash values are equal.
pub fn check_audit_equalities(document: &Value, manifest_id: &str, model_hash_value: Option<&str>) -> Result<(), String> {
    let reproducibility = &document["result_envelope"]["reproducibility"];
    if reproducibility["audit_manifest_ref"]["ref_id"].as_str() != Some(manifest_id) {
        return Err(format!(
            "{AUDIT_BINDING_MISMATCH}: the audit manifest id does not equal the result member's audit_manifest_ref"
        ));
    }
    if model_hash_value.is_none() || reproducibility["model_hash"]["value"].as_str() != model_hash_value {
        return Err(format!("{AUDIT_BINDING_MISMATCH}: the audit model hash does not equal the result member's"));
    }
    Ok(())
}
