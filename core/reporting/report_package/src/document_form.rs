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
pub const CHECKSUM_SUBJECT_MISMATCH: &str = "REPORT-PACKAGE-CHECKSUM-SUBJECT-MISMATCH";

/// The 0.3.0 schema's two checksum record definitions.
const CHECKSUM_DEFS: [&str; 2] = ["/$defs/Checksum", "/$defs/ScopedChecksum"];
/// The headless producer's run naming (`core/runner/headless/src/lib.rs`,
/// `run_id = "run:headless-preview:{request_id}"`): the run names the request
/// it was produced for.
const HEADLESS_RUN_PREFIX: &str = "run:headless-preview:";

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

/// One checksum record's location with its index segments replaced by `*`.
fn record_kind(path: &str) -> String {
    path.split('/')
        .map(|segment| if !segment.is_empty() && segment.bytes().all(|byte| byte.is_ascii_digit()) { "*" } else { segment })
        .collect::<Vec<_>>()
        .join("/")
}

/// Checksum subject attribution in a result member (DEL-08-01 Design §2.4,
/// "Checksum subjects"). Every checksum record the 0.3.0 schema types
/// (`$defs/Checksum`, `$defs/ScopedChecksum`) must name, in `payload_ref`,
/// the subject its location derives; a record at a location with no
/// derivation is refused. Shared by the producer and the reader, for an
/// admitted (preview-physics-2) document; source-independent.
pub fn check_document_subjects(document: &Value) -> Result<(), String> {
    let fail = |detail: String| Err(format!("{CHECKSUM_SUBJECT_MISMATCH}: {detail}"));
    let marked: Vec<(&str, &str)> = CHECKSUM_DEFS.iter().map(|pointer| (RESULTS_V03, *pointer)).collect();
    let records = match Catalog::results_v03().validate_marking_all(RESULTS_V03, document, &marked) {
        Ok(records) => records.into_iter().flatten().collect::<Vec<_>>(),
        Err(failure) => return fail(failure.to_string()),
    };
    let e = &document["result_envelope"];
    let text = |value: &Value| value.as_str().unwrap_or("").to_string();
    let envelope_id = text(&e["envelope_id"]);
    let run = text(&e["run_ref"]["ref_id"]);
    let origins = e["reproducibility"]["source_origin_bindings"].as_array().map(Vec::as_slice).unwrap_or(&[]);
    let [origin] = origins else {
        return fail("the result member must carry exactly one source origin".to_string());
    };
    // The carrier subject: the received carrier the origin binds.
    let carrier = match origin["origin_class"].as_str() {
        Some("attested_headless_producer") => reference("attested_headless_producer", &envelope_id),
        Some("received_current_dimension_absent") => {
            reference("received_current_carrier", &text(&origin["mechanics_run_ref"]["ref_id"]))
        }
        _ => return fail("the origin class has no derivable carrier subject".to_string()),
    };
    let derivative = reference("derivative_document", &envelope_id);
    let run_subjects = [reference("AnalysisRun", &run), reference("ResultEnvelope", &format!("result-envelope:{run}"))];
    for path in &records {
        let record = document.pointer(path).ok_or_else(|| format!("{CHECKSUM_SUBJECT_MISMATCH}: {path} is not in the member"))?;
        let payload_ref = &record["payload_ref"];
        let kind = record_kind(path);
        let accepted = match kind.as_str() {
            "/result_envelope/reproducibility/derivative_hash"
            | "/result_envelope/unit_preservation_witnesses/*/target_row_checksum" => *payload_ref == derivative,
            "/result_envelope/reproducibility/model_hash" => {
                *payload_ref == reference("model_payload", &text(&e["model_ref"]["ref_id"]))
            }
            "/result_envelope/reproducibility/request_hash" => match (
                origin["origin_class"].as_str(),
                origin["request_run_ref"]["ref_id"].as_str().and_then(|run| run.strip_prefix(HEADLESS_RUN_PREFIX)),
            ) {
                (Some("attested_headless_producer"), Some(request_id)) => {
                    *payload_ref == reference("runner_request", request_id)
                }
                _ => false,
            },
            "/result_envelope/reproducibility/run_hashes/*" => run_subjects.contains(payload_ref),
            "/result_envelope/reproducibility/raw_source_hashes/*"
            | "/result_envelope/reproducibility/source_origin_bindings/*/received_carrier_checksum"
            | "/result_envelope/reproducibility/source_origin_bindings/*/original_producer_checksum"
            | "/result_envelope/row_accounting/*/received_carrier_row_checksum"
            | "/result_envelope/row_accounting/*/original_producer_row_checksum"
            | "/result_envelope/row_disclosures/*/received_carrier_row_checksum"
            | "/result_envelope/row_disclosures/*/original_producer_row_checksum"
            | "/result_envelope/unit_preservation_witnesses/*/received_carrier_row_checksum"
            | "/result_envelope/unit_preservation_witnesses/*/original_producer_row_checksum" => *payload_ref == carrier,
            "/result_envelope/rule_pack_refs/*/checksum" => {
                let owner = path.strip_suffix("/checksum").and_then(|owner| document.pointer(owner));
                owner.is_some_and(|owner| payload_ref["ref_id"] == owner["rule_pack_id"])
            }
            _ => return fail(format!("the checksum record at {path} has no derivable subject")),
        };
        if !accepted {
            return fail(format!("the checksum record at {path} names {payload_ref}, not the subject its location derives"));
        }
    }
    // The raw-source hashes are the authentic producer's carrier checksum, as
    // the origin binds it, or nothing.
    let raw = e["reproducibility"]["raw_source_hashes"].as_array().map(Vec::as_slice).unwrap_or(&[]);
    let expected_raw: &[Value] = if origin["authentic_producer_available"] == true {
        std::slice::from_ref(&origin["original_producer_checksum"])
    } else {
        &[]
    };
    if raw != expected_raw {
        return fail("raw_source_hashes is not the origin's original producer checksum".to_string());
    }
    Ok(())
}

/// Checksum subject attribution in an audit-manifest member, as the writer
/// serializes it (`lib.rs` `audit_manifest_json`): an asset's hash names the
/// asset, and a rule pack's checksum names the pack (its document, under the
/// `rfc8785_jcs` basis). The audit's model and input-manifest hashes name
/// their own payloads; the producer binds those to the model and the input
/// manifest (`bindings`), and the reader binds the model hash value to the
/// result member (`check_audit_equalities`).
pub fn check_audit_subjects(audit: &Value) -> Result<(), String> {
    let fail = |detail: String| Err(format!("{CHECKSUM_SUBJECT_MISMATCH}: {detail}"));
    for asset in audit["assets"].as_array().map(Vec::as_slice).unwrap_or(&[]) {
        let hash = &asset["hash"];
        if !hash.is_null() && hash["payload_ref"] != asset["asset_id"] {
            return fail(format!("the hash of asset {} names another payload", asset["asset_id"]));
        }
    }
    for pack in audit["rule_pack_refs"].as_array().map(Vec::as_slice).unwrap_or(&[]) {
        let checksum = &pack["checksum"];
        if checksum.is_null() {
            continue;
        }
        let id = pack["rule_pack_id"].as_str().unwrap_or("");
        let expected = match checksum["canonicalization"].as_str() {
            Some("rfc8785_jcs") => format!("rule_pack_document:{id}"),
            _ => id.to_string(),
        };
        if checksum["payload_ref"].as_str() != Some(expected.as_str()) {
            return fail(format!("the checksum of rule pack {id} names another payload"));
        }
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
