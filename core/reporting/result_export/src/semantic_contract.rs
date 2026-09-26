//! Exact current producer semantics. No ID substring or unit-only family inference.
use serde_json::Value;
use std::sync::OnceLock;
pub fn contract() -> &'static Value {
    static CONTRACT: OnceLock<Value> = OnceLock::new();
    CONTRACT.get_or_init(|| {
        serde_json::from_str(include_str!(
            "../../../../fixtures/results/semantic_contract_v0_2.json"
        ))
        .expect("pinned semantic contract")
    })
}
pub fn signature(row: &Value) -> Result<Option<&'static Value>, String> {
    signature_in(contract(), row)
}
pub fn signature_in(table: &'static Value, row: &Value) -> Result<Option<&'static Value>, String> {
    let kind = row["kind"].as_str().ok_or("SOURCE_KIND_MISSING")?;
    let known: Vec<_> = table["rows"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|s| s["kind"] == kind)
        .collect();
    if known.is_empty() {
        return Ok(None);
    }
    let units: Vec<_> = known
        .into_iter()
        .filter(|s| s["unit"] == row["unit"])
        .collect();
    if units.is_empty() {
        return Err(format!("SOURCE_UNIT_CONTRADICTION: {kind}"));
    }
    let component = row["metadata"]["component"]
        .as_str()
        .filter(|s| !s.is_empty());
    let variants: Vec<&Value> = units
        .iter()
        .copied()
        .filter(|s| s["component"].is_null() || s["component"].as_str() == component)
        .collect();
    if !variants.is_empty() {
        // Only a table that declares source_basis adds this discriminator.
        // Existing method tables and their historical matching stay unchanged.
        return variants
            .into_iter()
            .find(|s| {
                s.get("source_basis").is_none() || s["source_basis"] == row["metadata"]["basis"]
            })
            .map(Some)
            .ok_or_else(|| format!("SOURCE_BASIS_CONTRADICTION: {kind}"));
    }
    if component.is_some() {
        return Err(format!("SOURCE_COMPONENT_CONTRADICTION: {kind}"));
    }
    // A missing component cannot identify the variant. This representative is
    // used only to classify the incomplete disclosure, never to emit a target.
    Ok(Some(units[0]))
}
pub fn complete_metadata(row: &Value) -> bool {
    [
        "component",
        "coordinate_system",
        "location",
        "basis",
        "sign_convention",
    ]
    .iter()
    .all(|k| row["metadata"][k].as_str().is_some_and(|s| !s.is_empty()))
}
pub fn canonical_metadata(row: &Value) -> Option<Value> {
    canonical_metadata_in(contract(), row)
}
pub fn canonical_metadata_in(table: &Value, row: &Value) -> Option<Value> {
    if !complete_metadata(row) {
        return None;
    }
    let mut projection = serde_json::Map::new();
    for (key, rule) in table["canonical_metadata_vocabulary"].as_object().unwrap() {
        let value = &row["metadata"][key];
        if let Some(allowed) = rule["enum"].as_array() {
            if !allowed.contains(value) {
                return None;
            }
        }
        projection.insert(key.clone(), value.clone());
    }
    Some(Value::Object(projection))
}

/// Dispatch is source-bound; reserved successor IDs never select the precision table.
pub const PHYSICS_SOURCE_ID: &str = "openpipestress.result_semantics/0.3.0/physics-source-1";
pub const PHYSICS_ID: &str = "openpipestress.result_semantics/0.3.0/physics-1";
pub const PRECISION_ID: &str = "openpipestress.result_semantics/0.3.0/precision-1";
/// Resolved load/reference-state method; bound to exactly one formulation profile.
pub const LOAD_REFERENCE_ID: &str = "openpipestress.result_semantics/0.3.0/load-reference-1";
pub const LOAD_REFERENCE_PROFILE: &str = "resolved_straight_load_state_v1";
pub const LOAD_REFERENCE_TABLE_SHA256: &str =
    "44bc41c06f589fab6ce931ac0eaa5344765ff64fd5f880cc2dd69ecb839c4f4d";
/// Joined load/reference-state method (retained-source receipt required);
/// bound to exactly one formulation profile and one receipt policy.
pub const LOAD_REFERENCE_SOURCE_ID: &str = crate::load_reference_source::CONTRACT_ID;
pub const LOAD_REFERENCE_SOURCE_PROFILE: &str = crate::load_reference_source::PROFILE;
pub const LOAD_REFERENCE_SOURCE_TABLE_SHA256: &str = crate::load_reference_source::TABLE_SHA256;
pub const PREVIEW_PHYSICS_ID: &str = "openpipestress.result_semantics/0.3.0/preview-physics-1";
pub const PREVIEW_PHYSICS_SHA256: &str =
    "ae55503d44a4750714a35c423623e38cf4132099134097193024d1635bfbc88a";
pub fn precision_contract() -> &'static Value {
    static CONTRACT: OnceLock<Value> = OnceLock::new();
    CONTRACT.get_or_init(|| {
        serde_json::from_str(include_str!(
            "../../../../fixtures/results/semantic_contract_v0_3_precision_1.json"
        ))
        .expect("pinned precision semantic contract")
    })
}
pub fn physics_contract() -> &'static Value {
    static CONTRACT: OnceLock<Value> = OnceLock::new();
    CONTRACT.get_or_init(|| {
        serde_json::from_str(include_str!(
            "../../../../fixtures/results/semantic_contract_v0_3_physics_1.json"
        ))
        .expect("pinned physics semantic contract")
    })
}
/// Pinned table bytes: identity, profile and sha256 are checked, never inferred.
pub fn verify_load_reference_table(bytes: &[u8]) -> Result<Value, String> {
    use sha2::{Digest, Sha256};
    if format!("{:x}", Sha256::digest(bytes)) != LOAD_REFERENCE_TABLE_SHA256 {
        return Err("SOURCE_LOAD_REFERENCE_TABLE_HASH".into());
    }
    let table: Value = serde_json::from_slice(bytes)
        .map_err(|_| "SOURCE_LOAD_REFERENCE_TABLE_HASH".to_string())?;
    if table["semantic_contract_id"] != LOAD_REFERENCE_ID
        || table["formulation_profile_id"] != LOAD_REFERENCE_PROFILE
    {
        return Err("SOURCE_LOAD_REFERENCE_TABLE_IDENTITY".into());
    }
    Ok(table)
}
pub fn load_reference_contract() -> &'static Value {
    static CONTRACT: OnceLock<Value> = OnceLock::new();
    CONTRACT.get_or_init(|| {
        verify_load_reference_table(include_bytes!(
            "../../../../fixtures/results/semantic_contract_v0_3_load_reference_1.json"
        ))
        .expect("pinned load-reference semantic contract")
    })
}
/// Pinned joined table bytes: identity, profile, policy and sha256 are checked.
pub fn verify_load_reference_source_table(bytes: &[u8]) -> Result<Value, String> {
    crate::load_reference_source::verify_table(bytes)
}
pub fn load_reference_source_contract() -> &'static Value {
    static CONTRACT: OnceLock<Value> = OnceLock::new();
    CONTRACT.get_or_init(|| {
        verify_load_reference_source_table(include_bytes!(
            "../../../../fixtures/results/semantic_contract_v0_3_load_reference_source_1.json"
        ))
        .expect("pinned load-reference-source semantic contract")
    })
}
pub fn preview_physics_contract() -> &'static Value {
    static CONTRACT: OnceLock<Value> = OnceLock::new();
    CONTRACT.get_or_init(|| {
        serde_json::from_str(include_str!(
            "../../../../fixtures/results/semantic_contract_v0_3_preview_physics_1.json"
        ))
        .expect("pinned preview-physics semantic contract")
    })
}
pub fn physics_source_contract() -> &'static Value {
    static CONTRACT: OnceLock<Value> = OnceLock::new();
    CONTRACT.get_or_init(|| {
        serde_json::from_str(include_str!(
            "../../../../fixtures/results/semantic_contract_v0_3_physics_source_1.json"
        ))
        .expect("pinned composite physical/source semantic contract")
    })
}
/// Header dispatch only; canonical envelopes have no raw rows. This does not
/// validate physical evidence and must never be used alone to qualify raw output.
pub fn for_source_metadata(source: &Value) -> Result<(&'static Value, &'static str), String> {
    match source["schema_version"].as_str() {
        Some("0.1.0") => {
            if [
                "producer",
                "numerical_quality",
                "formulation_basis",
                "contract_evidence",
                "source_block_recovery",
            ]
            .iter()
            .any(|k| source.get(k).is_some())
            {
                return Err("LEGACY_SOURCE_METADATA_CONTRADICTION".into());
            }
            Ok((contract(), "0.2.0"))
        }
        Some("0.2.0") => {
            let p = &source["producer"];
            if !exact_keys(
                p,
                &[
                    "component_name",
                    "component_version",
                    "semantic_contract_id",
                ],
            ) || p["component_name"] != "open_pipe_stress_product_physics"
                || p["component_version"] != "0.2.0"
                || !matches!(
                    p["semantic_contract_id"].as_str(),
                    Some(
                        PRECISION_ID
                            | PHYSICS_ID
                            | PHYSICS_SOURCE_ID
                            | LOAD_REFERENCE_ID
                            | LOAD_REFERENCE_SOURCE_ID
                            | PREVIEW_PHYSICS_ID
                            | crate::source_blocks::CONTRACT_ID
                    )
                )
            {
                return Err("SOURCE_PRODUCER_CONTRACT_UNSUPPORTED".into());
            }
            // The load-reference methods own no carrier namespace (same code
            // and position as the Python reader's first dispatch check).
            if (p["semantic_contract_id"] == LOAD_REFERENCE_ID
                || p["semantic_contract_id"] == LOAD_REFERENCE_SOURCE_ID)
                && source.get("carrier_evidence").is_some()
            {
                return Err("SOURCE_PRODUCER_CONTRACT_UNSUPPORTED".into());
            }
            if p["semantic_contract_id"] != crate::source_blocks::CONTRACT_ID
                && p["semantic_contract_id"] != PHYSICS_SOURCE_ID
                && p["semantic_contract_id"] != LOAD_REFERENCE_SOURCE_ID
                && source.get("source_block_recovery").is_some()
            {
                return Err("SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN".into());
            }
            if p["semantic_contract_id"] == PREVIEW_PHYSICS_ID
                && !source.get("contract_evidence").is_some_and(Value::is_object)
            {
                return Err("SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED".into());
            }
            let q = &source["numerical_quality"];
            if !exact_keys(
                q,
                &[
                    "value_representation",
                    "publication_quantization",
                    "integrity_policy",
                    "status",
                    "cases",
                ],
            ) || q["value_representation"] != "finite_binary64"
                || q["publication_quantization"] != "none"
                || q["integrity_policy"] != "M03-INTEGRITY-v1"
                || !quality_status(&q["status"])
                || !q["cases"].is_array()
            {
                return Err("SOURCE_NUMERICAL_QUALITY_INVALID".into());
            }
            for case in q["cases"].as_array().unwrap() {
                if !exact_keys(
                    case,
                    &[
                        "basis_ref",
                        "structural_status",
                        "solve_quality",
                        "model_matrix_fidelity",
                        "accuracy_evidence",
                        "evidence_refs",
                    ],
                ) || !quality_status(&case["solve_quality"])
                    || !matches!(
                        case["structural_status"].as_str(),
                        Some(
                            "passive_model_basis"
                                | "physical_mechanism_witnessed"
                                | "negative_energy_witnessed"
                                | "numerically_unresolved"
                        )
                    )
                    || !matches!(
                        case["model_matrix_fidelity"].as_str(),
                        Some(
                            "represented_equations_retained"
                                | "assembly_loss_detected"
                                | "assembly_uncertainty"
                                | "not_assessed"
                        )
                    )
                    || !matches!(
                        case["accuracy_evidence"].as_str(),
                        Some("not_claimed" | "reference_verified" | "unresolved")
                    )
                    || !case["evidence_refs"]
                        .as_array()
                        .is_some_and(|a| a.iter().all(nonempty_string))
                    || !exact_keys(&case["basis_ref"], &["ref_type", "ref_id"])
                    || !nonempty_string(&case["basis_ref"]["ref_type"])
                    || !nonempty_string(&case["basis_ref"]["ref_id"])
                {
                    return Err("SOURCE_NUMERICAL_CASE_INVALID".into());
                }
            }
            let f = &source["formulation_basis"];
            let profile = match p["semantic_contract_id"].as_str() {
                Some(PHYSICS_ID | PHYSICS_SOURCE_ID) => "exact_straight_pressure_v2",
                // The only profile for load-reference-1; no other contract accepts it.
                Some(LOAD_REFERENCE_ID) => LOAD_REFERENCE_PROFILE,
                // The only profile for load-reference-source-1, likewise exclusive.
                Some(LOAD_REFERENCE_SOURCE_ID) => LOAD_REFERENCE_SOURCE_PROFILE,
                _ => "product_preview_mechanics_v1",
            };
            if !exact_keys(f, &["profile_id", "limitations"])
                || f["profile_id"] != profile
                || !f["limitations"]
                    .as_array()
                    .is_some_and(|a| !a.is_empty() && a.iter().all(nonempty_string))
            {
                return Err("SOURCE_FORMULATION_BASIS_UNSUPPORTED".into());
            }
            // Canonical metadata has no raw rows. Select the explicit table here;
            // raw evidence and source-block invocation checks are separate APIs.
            let table = match p["semantic_contract_id"].as_str() {
                Some(PHYSICS_ID) => physics_contract(),
                Some(PHYSICS_SOURCE_ID) => physics_source_contract(),
                Some(LOAD_REFERENCE_ID) => load_reference_contract(),
                Some(LOAD_REFERENCE_SOURCE_ID) => load_reference_source_contract(),
                Some(crate::source_blocks::CONTRACT_ID) => source_blocks_contract(),
                Some(PRECISION_ID) => precision_contract(),
                Some(PREVIEW_PHYSICS_ID) => preview_physics_contract(),
                _ => return Err("SOURCE_PRODUCER_CONTRACT_UNSUPPORTED".into()),
            };
            Ok((table, "0.3.0"))
        }
        _ => Err("SOURCE_SCHEMA_VERSION_UNSUPPORTED".into()),
    }
}
/// Raw dispatch validates the selected physical contract, never inferring it
/// from a profile name or a numerical/accuracy label.
pub fn for_source(source: &Value) -> Result<(&'static Value, &'static str), String> {
    let selected = for_source_metadata(source)?;
    match source["producer"]["semantic_contract_id"].as_str() {
        Some(PHYSICS_ID) => {
            forbid_load_reference_evidence(source)?;
            validate_physics_evidence(source)?
        }
        Some(PREVIEW_PHYSICS_ID) => validate_preview_physics_evidence(source)?,
        Some(PHYSICS_SOURCE_ID) => {
            forbid_load_reference_evidence(source)?;
            crate::physics_source::validate(source, None)?;
        }
        Some(LOAD_REFERENCE_ID) => validate_load_reference_evidence(source)?,
        Some(LOAD_REFERENCE_SOURCE_ID) => {
            validate_load_reference_source_evidence(source)?;
        }
        Some(crate::source_blocks::CONTRACT_ID) => {
            // Shape/publication validation does not supply an actual invocation
            // or recreate the producer's private arithmetic receipt.
            crate::source_blocks::validate(source, None)?;
        }
        _ => {
            if source
                .get("contract_evidence")
                .is_some_and(|v| !v.is_null())
                || source.get("source_block_recovery").is_some()
                || source.get("carrier_evidence").is_some()
            {
                return Err("SOURCE_EVIDENCE_CONTRACT_UNSUPPORTED".into());
            }
        }
    }
    Ok(selected)
}
pub use crate::load_reference::{
    validate_load_reference_evidence, validate_load_reference_transport_metadata,
};
pub use crate::load_reference_source::{
    validate_load_reference_source_evidence, validate_load_reference_source_transport_metadata,
};
pub use crate::physics_evidence::{
    validate_physics_evidence, validate_transport_metadata as validate_physics_transport_metadata,
};
/// These inputs were already refused by the closed physics namespaces; the
/// shared code only names the load-reference downgrade in both readers.
fn forbid_load_reference_evidence(source: &Value) -> Result<(), String> {
    if source["contract_evidence"]
        .get("load_reference_states")
        .is_some()
    {
        return Err("SOURCE_LOAD_REFERENCE_EVIDENCE_FORBIDDEN".into());
    }
    Ok(())
}
pub use crate::preview_physics_evidence::validate_preview_physics_evidence;

/// Identities a fresh solve may publish (DESIGN 5.7, S1 10). One static set in
/// every language, never a route predicate. T1 added its identities on activation.
pub const FRESH_IDENTITIES: &[&str] = &[
    PREVIEW_PHYSICS_ID,
    crate::source_blocks::CONTRACT_ID,
    PHYSICS_ID,
    PHYSICS_SOURCE_ID,
    // T1 activation (DESIGN 10.3, SF-4): 0.4.0 exact-route identities only.
    LOAD_REFERENCE_ID,
    LOAD_REFERENCE_SOURCE_ID,
];
pub fn is_fresh_identity(semantic_contract_id: &str) -> bool {
    FRESH_IDENTITIES.contains(&semantic_contract_id)
}
pub const PRECISION_1_HISTORICAL_SEMANTICS: &str = "PRECISION_1_HISTORICAL_SEMANTICS";
pub const SOURCE_BLOCKS_ORDINARY_CASE_LEGACY_SEMANTICS: &str =
    "SOURCE_BLOCKS_ORDINARY_CASE_LEGACY_SEMANTICS";
pub const RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE: &str = "RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE";
/// Why a readable source is not Current. Derived from the received bytes only.
pub fn standing_reason(source: &Value) -> Option<&'static str> {
    match source["producer"]["semantic_contract_id"].as_str() {
        Some(PRECISION_ID) => Some(PRECISION_1_HISTORICAL_SEMANTICS),
        Some(crate::source_blocks::CONTRACT_ID)
            if crate::source_blocks::has_ordinary_qualified_case(source) =>
        {
            Some(SOURCE_BLOCKS_ORDINARY_CASE_LEGACY_SEMANTICS)
        }
        _ => None,
    }
}
/// Rule binding refusal (DESIGN 5.7 SF-B). The abs-sum summary of a
/// non-composite source-blocks-1 envelope may not be bound, directly or via
/// its headline reference. Every binding site calls this helper.
pub fn rule_binding_refusal(envelope: &Value, row: &Value) -> Option<&'static str> {
    if envelope["producer"]["semantic_contract_id"] != crate::source_blocks::CONTRACT_ID {
        return None;
    }
    let headline = &envelope["summary"]["max_open_formula_stress"]["result_ref"];
    (row["kind"] == "open_formula_stress_summary"
        || (headline.is_string() && row["id"] == *headline))
        .then_some(RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE)
}

fn quality_status(value: &Value) -> bool {
    matches!(
        value.as_str(),
        Some("not_assessed" | "checks_passed" | "sensitive" | "unresolved" | "failed")
    )
}

/// Numerical eligibility only. The caller must separately bind the actual model,
/// input, build and authentic source. Requested bases must come from that model.
/// Historical authenticity never supplies evidence of the current algorithm.
pub fn numerical_use_standing(source: &Value, requested_basis_refs: &[Value]) -> &'static str {
    numerical_use_standing_with_context(source, requested_basis_refs, None)
}

pub fn numerical_use_standing_with_context(
    source: &Value,
    requested_basis_refs: &[Value],
    actual_invocation: Option<&Value>,
) -> &'static str {
    let Ok((_, version)) = for_source(source) else {
        return "unsupported";
    };
    // T0R (A2 item 10): validate first; then historical precision-1 and mixed
    // ordinary source-blocks-1 are never Current.
    if standing_reason(source).is_some() {
        return "needs_recompute";
    }
    // T1 (declared standing edit, T1_WAVE1_RULINGS.md section 6): admitted joined
    // evidence is never numerically eligible here, because a 0.4.0 resolved case
    // cannot be re-derived from a captured request by a reader. Identical in
    // outcome to the fall-through (its status is never checks_passed).
    if source["producer"]["semantic_contract_id"] == LOAD_REFERENCE_SOURCE_ID {
        return "needs_recompute";
    }
    if version != "0.3.0" || requested_basis_refs.is_empty() {
        return "needs_recompute";
    }
    if matches!(
        source["producer"]["semantic_contract_id"].as_str(),
        Some(crate::source_blocks::CONTRACT_ID | PHYSICS_SOURCE_ID)
    ) {
        let expected = source["source_block_recovery"]["body"]["cases"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| c["basis_ref"].clone())
            .collect::<Vec<_>>();
        if expected != requested_basis_refs {
            return "needs_recompute";
        }
        let standing = if source["producer"]["semantic_contract_id"] == PHYSICS_SOURCE_ID {
            crate::physics_source::validate(source, actual_invocation)
        } else {
            crate::source_blocks::validate(source, actual_invocation)
        };
        return match standing {
            Ok(true) => "numerically_eligible",
            Ok(false) => "needs_recompute",
            Err(_) => "unsupported",
        };
    }
    let q = &source["numerical_quality"];
    // Sensitive backward-error evidence is retained for inspection, not qualified use.
    if q["status"] != "checks_passed" {
        return "needs_recompute";
    }
    let cases = q["cases"].as_array().unwrap();
    if cases.len() != requested_basis_refs.len() {
        return "needs_recompute";
    }
    let mut ids = std::collections::HashSet::new();
    for key in ["results", "diagnostics"] {
        let Some(items) = source[key].as_array() else {
            return "needs_recompute";
        };
        for item in items {
            let Some(id) = item["id"].as_str().filter(|id| !id.is_empty()) else {
                return "needs_recompute";
            };
            if !ids.insert(id) {
                return "needs_recompute";
            }
        }
    }
    for (index, basis) in requested_basis_refs.iter().enumerate() {
        if requested_basis_refs[..index].contains(basis) {
            return "needs_recompute";
        }
        let matching: Vec<_> = cases
            .iter()
            .filter(|case| case["basis_ref"] == *basis)
            .collect();
        if matching.len() != 1 {
            return "needs_recompute";
        }
        let case = matching[0];
        if case["solve_quality"] != "checks_passed"
            || case["structural_status"] != "passive_model_basis"
            || case["model_matrix_fidelity"] != "represented_equations_retained"
            || !matches!(
                case["accuracy_evidence"].as_str(),
                Some("not_claimed" | "reference_verified")
            )
            || !case["evidence_refs"].as_array().is_some_and(|refs| {
                !refs.is_empty()
                    && refs
                        .iter()
                        .all(|r| r.as_str().is_some_and(|id| ids.contains(id)))
            })
        {
            return "needs_recompute";
        }
    }
    "numerically_eligible"
}

fn exact_keys(value: &Value, keys: &[&str]) -> bool {
    value.as_object().is_some_and(|object| {
        object.len() == keys.len() && keys.iter().all(|key| object.contains_key(*key))
    })
}
fn nonempty_string(value: &Value) -> bool {
    value.as_str().is_some_and(|text| !text.is_empty())
}

/// Additive signatures preserve p1 bytes and meanings under a distinct identity.
pub fn source_blocks_contract() -> &'static Value {
    static CONTRACT: OnceLock<Value> = OnceLock::new();
    CONTRACT.get_or_init(|| {
        serde_json::from_str(include_str!(
            "../../../../fixtures/results/semantic_contract_v0_3_source_blocks_1.json"
        ))
        .expect("pinned source-block semantic contract")
    })
}
