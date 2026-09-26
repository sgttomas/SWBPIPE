//! Closed admission of the joined load-reference-source-1 evidence
//! (`openpipestress.result_semantics/0.3.0/load-reference-source-1`, profile
//! `resolved_straight_load_state_source_v1`, receipt policy
//! `LOAD-REFERENCE-SOURCE-1`; CP2_WIRE_ADDENDUM_2 section 5 with the CP4
//! corrections). These checks establish internal source consistency, not
//! solver accuracy, producer origin or model freshness.
//!
//! The order of checks and every error string are shared with the Python
//! reader `core/analysis_runs/load_reference_source.py`; change both together.
//!
//! 1. The load-reference pre-pass (S1-S13 of `crate::load_reference`) in its
//!    joined form: every per-case record, member, support and contribution
//!    check, with a selected case publishing `retained_source_blocks_exact_v1`
//!    and carrying `SOURCE_BLOCK_RECOVERY_SELECTED` instead of NOT_JOINED.
//! 2. The receipt on the received bytes: policy, closed shape, receipt hash,
//!    publication hash (raw only), case order, per-case method, requested
//!    mode and the
//!    per-case physical-evidence hash over domain
//!    `load_reference_source_case_evidence_v1` of
//!    `{exact_case, pressure, load_reference_state}`, as the producer hashes it.
//! 3. The unchanged physics-source-1 validator on a projected copy. The
//!    projection removes or neutralizes only what steps 1-2 have bound (the
//!    records, the resolved-member keys, the two basis constants, the
//!    identity, profile and policy), and re-derives the physics-source-1
//!    hashes of the projected bytes. Every received hash was verified in
//!    step 2 before the projection, so re-derivation cannot hide a change.
//!
//! Numerical eligibility is never granted: the reader cannot re-derive a
//! 0.4.0 resolved case from a captured request (the physics-source-1
//! analogue is `actual_materials`), so a valid envelope is `needs_recompute`.
use crate::load_reference::{prepass, project as project_load_reference, Method};
use crate::source_blocks::domain_hash;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

type Check = Result<(), String>;

pub const CONTRACT_ID: &str = "openpipestress.result_semantics/0.3.0/load-reference-source-1";
pub const PROFILE: &str = "resolved_straight_load_state_source_v1";
pub const TABLE_SHA256: &str = "d1628194a7730f427843b00228dd233cf92b8e7d26f3bc31c660a3ea59e28337";
pub const POLICY: &str = "LOAD-REFERENCE-SOURCE-1";
pub const CASE_EVIDENCE_DOMAIN: &str = "load_reference_source_case_evidence_v1";
const PHYSICS_SOURCE_POLICY: &str = "PHYSICS-SOURCE-1";
const PHYSICS_SOURCE_CASE_DOMAIN: &str = "physics_source_case_evidence_v1";

fn code(name: &str) -> String {
    format!("SOURCE_LOAD_REFERENCE_{name}")
}
fn require(ok: bool, name: &str) -> Check {
    if ok {
        Ok(())
    } else {
        Err(code(name))
    }
}

/// Pinned table bytes: identity, profile and sha256 are checked, never inferred.
pub fn verify_table(bytes: &[u8]) -> Result<Value, String> {
    if format!("{:x}", Sha256::digest(bytes)) != TABLE_SHA256 {
        return Err(code("SOURCE_TABLE_HASH"));
    }
    let table: Value = serde_json::from_slice(bytes).map_err(|_| code("SOURCE_TABLE_HASH"))?;
    if table["semantic_contract_id"] != CONTRACT_ID
        || table["formulation_profile_id"] != PROFILE
        || table["source_block_policy"] != POLICY
    {
        return Err(code("SOURCE_TABLE_IDENTITY"));
    }
    Ok(table)
}

/// Raw joined publication. `Ok(false)`: admitted, never numerically eligible.
pub fn validate_load_reference_source_evidence(source: &Value) -> Result<bool, String> {
    validate(source, true).map(|()| false)
}

/// Retained joined statements without raw rows or diagnostics: the closed
/// pre-pass, the retained receipt and the physics-source-1 transport checks.
pub fn validate_load_reference_source_transport_metadata(source: &Value) -> Check {
    validate(source, false)
}

fn validate(source: &Value, raw: bool) -> Check {
    // J0 the joined identity and profile. The projection below replaces both,
    // so the direct validator checks them itself.
    require(
        source["producer"]["semantic_contract_id"] == CONTRACT_ID
            && source["formulation_basis"]["profile_id"] == PROFILE,
        "JOIN_IDENTITY",
    )?;
    // J1 joined pre-pass (S1-S13).
    prepass(source, raw, Method::Joined)?;
    // J2 receipt on the received bytes.
    receipt(source, raw)?;
    // J3 physics-source-1 on the projected copy.
    let projected = project(source, raw)?;
    let inherited = if raw {
        crate::physics_source::validate(&projected, None).map(|_| ())
    } else {
        crate::physics_source::validate_transport_metadata(&projected)
    };
    inherited.map_err(|e| format!("{}: {e}", code("JOIN_PHYSICS_SOURCE")))
}

fn case_pressure<'a>(pressure: &'a [Value], case_id: &Value) -> Vec<&'a Value> {
    pressure
        .iter()
        .filter(|p| p["load_case_id"] == *case_id)
        .collect()
}

fn receipt(source: &Value, raw: bool) -> Check {
    let receipt = &source["source_block_recovery"];
    let body = &receipt["body"];
    // R1 policy, then R2 the closed physics-source-1 receipt shape with only
    // the policy constant substituted.
    require(body["policy"] == POLICY, "JOIN_RECEIPT_POLICY")?;
    let mut shaped = receipt.clone();
    shaped["body"]["policy"] = json!(PHYSICS_SOURCE_POLICY);
    require(
        crate::physics_source::validate_receipt_shape(&shaped).is_ok(),
        "JOIN_RECEIPT_SHAPE",
    )?;
    // R3-R4 receipt and publication hashes.
    require(
        receipt["receipt_sha256"] == domain_hash("source_blocks_receipt_v1", body)?,
        "JOIN_RECEIPT_HASH",
    )?;
    if raw {
        let mut publication = source.clone();
        if let Some(o) = publication.as_object_mut() {
            o.remove("source_block_recovery");
        }
        require(
            body["publication_sha256"]
                == domain_hash("source_blocks_publication_v1", &publication)?,
            "JOIN_PUBLICATION_HASH",
        )?;
    }
    // R5 one receipt case, exact case and record per case, in case order.
    let evidence = &source["contract_evidence"];
    let cases = body["cases"].as_array().unwrap();
    let exact = evidence["exact_cases"].as_array().unwrap();
    let records = evidence["load_reference_states"].as_array().unwrap();
    let pressure = evidence["pressure"].as_array().unwrap();
    require(
        cases
            .iter()
            .map(|c| &c["basis_ref"]["ref_id"])
            .eq(exact.iter().map(|c| &c["load_case_id"]))
            && records
                .iter()
                .map(|r| &r["load_case_id"])
                .eq(exact.iter().map(|c| &c["load_case_id"])),
        "JOIN_CASE_ORDER",
    )?;
    // R6-R7 per case: selected method, requested mode and physical-evidence hash.
    for ((case, exact), record) in cases.iter().zip(exact).zip(records) {
        require(
            case["selected_method"] == exact["recovery_method"],
            "JOIN_RECOVERY_METHOD",
        )?;
        require(
            case["requested_mode"] == record["solve"]["requested_mode"],
            "JOIN_REQUESTED_MODE",
        )?;
        // The producer hashes `pressure: []` for a selected case; hashing the case's
        // slice is equivalent only because J3 refuses a selected case with a non-empty
        // pressure inventory (physics-source-1 SOURCE_PRESSURE_INVENTORY; note N-3).
        let proof = json!({
            "exact_case": exact,
            "pressure": case_pressure(pressure, &exact["load_case_id"]),
            "load_reference_state": record,
        });
        require(
            case["physical_evidence_sha256"] == domain_hash(CASE_EVIDENCE_DOMAIN, &proof)?,
            "JOIN_PHYSICAL_CASE_HASH",
        )?;
    }
    Ok(())
}

/// The physics-source-1 form of verified joined bytes. Only called after
/// `prepass` and `receipt`, which fix every shape it touches.
fn project(source: &Value, raw: bool) -> Result<Value, String> {
    let mut projected = project_load_reference(source);
    projected["producer"]["semantic_contract_id"] = json!(crate::physics_source::CONTRACT_ID);
    projected["formulation_basis"]["profile_id"] = json!(crate::physics_source::PROFILE);
    let evidence = projected["contract_evidence"].clone();
    let pressure = evidence["pressure"].as_array().unwrap();
    let body = &mut projected["source_block_recovery"]["body"];
    body["policy"] = json!(PHYSICS_SOURCE_POLICY);
    for (case, exact) in body["cases"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .zip(evidence["exact_cases"].as_array().unwrap())
    {
        case["physical_evidence_sha256"] = json!(domain_hash(
            PHYSICS_SOURCE_CASE_DOMAIN,
            &json!({"exact_case": exact, "pressure": case_pressure(pressure, &exact["load_case_id"])}),
        )?);
    }
    if raw {
        let mut publication = projected.clone();
        publication
            .as_object_mut()
            .unwrap()
            .remove("source_block_recovery");
        projected["source_block_recovery"]["body"]["publication_sha256"] =
            json!(domain_hash("source_blocks_publication_v1", &publication)?);
    }
    projected["source_block_recovery"]["receipt_sha256"] = json!(domain_hash(
        "source_blocks_receipt_v1",
        &projected["source_block_recovery"]["body"]
    )?);
    Ok(projected)
}
