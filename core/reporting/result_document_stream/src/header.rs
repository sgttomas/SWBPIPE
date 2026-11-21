//! The header-level decisions `derive_document` makes inline, and the header comparisons of
//! `validate_document` that a derived document can still fail.

use open_pipe_stress_result_export::derivative::{checksum, reference};
use open_pipe_stress_result_export::json_value_eq;
use open_pipe_stress_result_export::semantic_contract::{
    PHYSICS_RETAINED_ID, PRESSURE_ID, PRESSURE_SOURCE_ID, PREVIEW_PHYSICS_ID, PREVIEW_PHYSICS_RETAINED_ID,
    PREVIEW_PHYSICS_TIMOSHENKO_ID, RETAINED_PRECISION_DOWNGRADE_FORBIDDEN,
};
use serde_json::{Map, Value};

/// What a source's identity and rows decide of the document's header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct HeaderDecisions {
    /// The envelope carries the source's `contract_evidence`, whole.
    pub(crate) carries_contract_evidence: bool,
    /// The envelope carries the source's `source_block_recovery` receipt, whole (the
    /// pressure-source identity); any other identity's document carries none.
    pub(crate) carries_source_blocks: bool,
    /// A successor identity: its `retained_precision` receipt travels whole, and its rows
    /// carry validated classes.
    pub(crate) is_successor: bool,
    /// The refusal of a dimension-absent carrier whose rows declare a dimension.
    pub(crate) declared_dimensions: Option<&'static str>,
}

/// (d) The header-level decisions: which semantic contracts carry `contract_evidence`,
/// whether the source is a successor, whether it carries the source-block receipt, and
/// whether a dimension-absent carrier is refused for declaring a row dimension.
///
/// Restates `derivative.rs` lines 158 to 168 (the identities that carry
/// `contract_evidence`), `is_successor` (lines 17 to 22), the pressure-source test of
/// lines 173 and 426, and `refuse_declared_dimensions` (lines 110 to 117), all private
/// there. `carrier_scope` is the origin's carrier scope, when it states one.
pub(crate) fn header_decisions(source: &Value, carrier_scope: Option<&str>) -> HeaderDecisions {
    let identity = source["producer"]["semantic_contract_id"].as_str();
    let carries_contract_evidence = matches!(
        identity,
        Some(
            PRESSURE_ID
                | PRESSURE_SOURCE_ID
                | PREVIEW_PHYSICS_ID
                | PREVIEW_PHYSICS_TIMOSHENKO_ID
                | PREVIEW_PHYSICS_RETAINED_ID
                | PHYSICS_RETAINED_ID
        )
    );
    let is_successor = matches!(identity, Some(PREVIEW_PHYSICS_RETAINED_ID | PHYSICS_RETAINED_ID));
    let carries_source_blocks = source["producer"]["semantic_contract_id"] == PRESSURE_SOURCE_ID;
    let declares = || {
        source["results"]
            .as_array()
            .is_some_and(|rows| rows.iter().any(|row| row.get("dimension").is_some()))
    };
    let declared_dimensions = (carrier_scope == Some("received_current_dimension_absent_carrier") && declares())
        .then_some("CARRIER_DIMENSION_PRESENT");
    HeaderDecisions { carries_contract_evidence, carries_source_blocks, is_successor, declared_dimensions }
}

/// What the header comparisons read of the derived envelope.
pub(crate) struct DerivedHeader<'h> {
    /// The envelope's own members (those not copied from the source and not row lists).
    pub(crate) envelope: &'h Map<String, Value>,
    /// The envelope's `contract_evidence` as derived: the source's member where the identity
    /// carries it, else the base's own, if any.
    pub(crate) contract_evidence: Option<&'h Value>,
    /// The one origin binding, as the document states it.
    pub(crate) origin: &'h Value,
    /// The SHA-256 of the source's canonical text, as lowercase hexadecimal.
    pub(crate) source_digest: &'h str,
}

/// The header comparisons `validate_document` makes, in its order (`derivative.rs` lines 426
/// to 532), of a document `derive_document` built. A comparison of a member the derivation
/// copied from the source, or computed itself, holds by construction and is named, not
/// repeated; the ones a base or an origin can still fail are made.
pub(crate) fn validate_header(header: &DerivedHeader, source: &Value, decisions: &HeaderDecisions) -> Result<(), String> {
    let e = header.envelope;
    // 426 to 435: the source-block receipt is the source's, copied whole; on another identity
    // a base that carries one was refused by the derivation.
    // 438 to 444: the successor's receipt is the source's, copied whole.
    if !decisions.is_successor && e.contains_key("retained_precision") {
        return Err(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN.into());
    }
    // 446 to 450: producer, numerical_quality and formulation_basis are copied whole.
    // 451 to 457: contract evidence.
    let evidence = source.get("contract_evidence").filter(|v| !v.is_null());
    if !match (header.contract_evidence, evidence) {
        (Some(copied), Some(evidence)) => json_value_eq(copied, evidence),
        (copied, evidence) => copied == evidence,
    } {
        return Err("SOURCE_CONTRACT_EVIDENCE_BINDING_MISMATCH".into());
    }
    // 458 to 469: the semantic contract reference and both versions are the derivation's own.
    // 470 to 498: the lists and the one origin are the derivation's; the envelope id, the
    // origin id and the model reference were read by it.
    let origin = header.origin;
    let model_ref = source["model_ref"].as_str().ok_or("MODEL_REF_MISSING")?;
    if origin["actual_model_ref"] != reference("model_payload", model_ref)
        || e.get("model_ref").unwrap_or(&Value::Null) != &reference("model_payload", model_ref)
        || origin["mechanics_run_ref"]
            != reference("mechanics_run", source["run_id"].as_str().ok_or("MECHANICS_RUN_MISSING")?)
    {
        return Err("ORIGIN_IDENTITY_MISMATCH".into());
    }
    let authentic = origin["authentic_producer_available"].as_bool().ok_or("ORIGIN_AVAILABILITY_MISSING")?;
    let carrier_scope = origin["received_carrier_checksum"]["payload_scope"].as_str().ok_or("ORIGIN_SCOPE_MISSING")?;
    match carrier_scope {
        "received_current_dimension_absent_carrier" | "attested_headless_producer_carrier" => {}
        _ => return Err("ORIGIN_SCOPE_INVALID".into()),
    }
    let carrier_ref = origin["received_carrier_checksum"]["payload_ref"].clone();
    let mut carrier = checksum(&Value::Null, carrier_scope, carrier_ref)?;
    carrier["value"] = Value::String(header.source_digest.to_owned());
    if origin["received_carrier_checksum"] != carrier {
        return Err("SOURCE_CARRIER_HASH_MISMATCH".into());
    }
    // 525: the declared-dimension refusal is the derivation's, made already.
    if authentic {
        if origin["original_producer_checksum"] != origin["received_carrier_checksum"] {
            return Err("ORIGINAL_PRODUCER_BINDING_MISMATCH".into());
        }
    }
    // 530 to 532: an unavailable producer's hash was refused by the derivation.
    // 533 to 541: the cardinalities hold by construction; the row loop follows.
    Ok(())
}
