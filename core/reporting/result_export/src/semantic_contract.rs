//! Exact current producer semantics. No ID substring or unit-only family inference.
use serde_json::Value;
use std::sync::OnceLock;
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
/// The signature of `row` under the table `for_source` selected for `source`.
/// Connector signatures, including the two scalar pressure rows, resolve
/// through the same independently bound ordinary pressure table.
pub fn signature_for(_source: &Value, table: &'static Value, row: &Value) -> Result<Option<Value>, String> {
    Ok(signature_in(table, row)?.cloned())
}
/// Canonical metadata under the vocabulary `for_source` selected for `source`
/// Connector-specific location/basis rules remain independently checked.
pub fn canonical_metadata_for(source: &Value, table: &Value, row: &Value) -> Option<Value> {
    if source["producer"]["semantic_contract_id"] == PRESSURE_ID
        && crate::connector_evidence::is_connector_kind(row["kind"].as_str().unwrap_or_default())
    {
        return crate::connector_evidence::canonical_metadata(row);
    }
    canonical_metadata_in(table, row)
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

/// Dispatch is source-bound; a reserved successor ID never selects a table.
/// `3.0.0/exact_pressure_v3` publishes under the `pressure-2` semantics on the
/// current model document, with one profile (T4-U2a H-1 introduced it as
/// pressure-1; model 0.6.0 renamed it, DEL-04-07 Design `u4b-u5b-implementation.md` §1.2).
pub const PRESSURE_ID: &str = "openpipestress.result_semantics/0.3.0/pressure-2";
pub const PRESSURE_PROFILE: &str = "exact_pressure_v3";
pub const PRESSURE_SOURCE_ID: &str = crate::physics_source::PRESSURE_CONTRACT_ID;
/// Independent current resident source-family table, with no ordinary alias.
/// T4-U6 (D-5, §2.4): both tables carry one stress signature per (recovery
/// token × section basis), each with its `source_basis`.
pub const PRESSURE_SOURCE_TABLE_SHA256: &str = "b5a4b0e585b9ec440d36144860baaf101b628498cfdab9a1de9248c2ed5069e7";
pub const PRESSURE_TABLE_SHA256: &str = "22fe1361f7913c88875d1df3faf027bd9db0d6af7a4bb3c214fade33387c3f95";
/// The pinned counts, re-derived from the tables (T4-U6: pressure-2 106
/// signatures over 59 kinds, T4-U5b: plus the tie-rod force, 107 over 60, T4-U4b:
/// plus 22 section stress, toroidal and directional signatures, 129 over 66;
/// pressure-source-1 88 over 51).
pub const PRESSURE_SIGNATURE_COUNT: u64 = 129;
pub const PRESSURE_KIND_COUNT: u64 = 66;
/// T4-U4b §4.3 (S4): + the straight derived signatures (sigma+, sigma-, tau on both bases), 88 -> 94
/// over 51 -> 53 kinds.
pub const PRESSURE_SOURCE_SIGNATURE_COUNT: u64 = 94;
pub const PRESSURE_SOURCE_KIND_COUNT: u64 = 53;
/// B1 (b7-plan): precision-1 is no longer read. A stored precision-1 result is
/// refused by name (re-run the analysis); no table, route or notice remains.
pub const RETIRED_PRECISION_ID: &str = "openpipestress.result_semantics/0.3.0/precision-1";
pub const SOURCE_PRECISION_1_RETIRED: &str = "SOURCE_PRECISION_1_RETIRED";
/// Model 0.6.0 (DEL-04-07 Design `u4b-u5b-implementation.md` §1.2): the
/// ordinary identities pressure-1 and preview-physics-1 are retired, their
/// tables deleted. A stored result under either is refused by name (re-run).
pub const RETIRED_PRESSURE_1_ID: &str = "openpipestress.result_semantics/0.3.0/pressure-1";
pub const SOURCE_PRESSURE_1_RETIRED: &str = "SOURCE_PRESSURE_1_RETIRED";
pub const RETIRED_PREVIEW_PHYSICS_1_ID: &str = "openpipestress.result_semantics/0.3.0/preview-physics-1";
pub const SOURCE_PREVIEW_PHYSICS_1_RETIRED: &str = "SOURCE_PREVIEW_PHYSICS_1_RETIRED";
pub const PREVIEW_PHYSICS_ID: &str = "openpipestress.result_semantics/0.3.0/preview-physics-2";
pub const PREVIEW_PHYSICS_SHA256: &str =
    "ee8036d59d0aeb255ff132e4dee0d6e7a1e8da8b33e4697c7761c695c3f6e387";
/// U6a (D-U6-6; D2 4.7 S-1, 4.9.6): the F2a preview successor. Its raw and
/// transport statements are checked only by the accepted reader
/// (`crate::retained_precision`), never by this module's ordinary branches, and
/// its standing comes only from that reader's verified receipt (D1 5 item 3).
pub const PREVIEW_PHYSICS_RETAINED_ID: &str = crate::retained_precision::CONTRACT_ID;
pub const PREVIEW_PHYSICS_RETAINED_PROFILE: &str = crate::retained_precision::PROFILE;
pub const PREVIEW_PHYSICS_RETAINED_SHA256: &str =
    "4da08f964605a5080b38c0ea9793731b33720070db001a4c7f1c4f9fbcaeb76f";
/// B3b (B3-D §6.1): the exact successor `<physics-retained>`, whose base is
/// pressure-2 since B7b D4 (a). Like the preview successor, its raw and transport statements are
/// checked only by the accepted reader (`crate::retained_precision`, its exact
/// route), and its standing comes only from that reader's verified receipt.
pub const PHYSICS_RETAINED_ID: &str = crate::retained_precision::EXACT_CONTRACT_ID;
pub const PHYSICS_RETAINED_PROFILE: &str = crate::retained_precision::EXACT_PROFILE;
/// T4-U6: its rows, vocabulary and counts follow the inherited pressure-2 table.
pub const PHYSICS_RETAINED_SHA256: &str =
    "d7bc48c919396081495207df35464d3d805436e288a31f9f23f76d28864d382e";
/// A receipt member or a W1 method-token row offered under any other identity
/// (I66 F-5: the base readers' closed lists do not name `retained_precision`).
pub const RETAINED_PRECISION_DOWNGRADE_FORBIDDEN: &str = "RETAINED_PRECISION_DOWNGRADE_FORBIDDEN";
/// D2 4.9.9 (revision 4, N-5): binding refusals for classified successor rows
/// before S-I. They never demote the envelope's standing.
pub const RULE_QUANTITY_BELOW_VERIFIED_FLOOR: &str = "RULE_QUANTITY_BELOW_VERIFIED_FLOOR";
pub const RULE_QUANTITY_NOT_COVERED: &str = "RULE_QUANTITY_NOT_COVERED";
/// Pinned successor table bytes: identity, profile and sha256 are checked.
pub fn verify_preview_physics_retained_table(bytes: &[u8]) -> Result<Value, String> {
    use sha2::{Digest, Sha256};
    if format!("{:x}", Sha256::digest(bytes)) != PREVIEW_PHYSICS_RETAINED_SHA256 {
        return Err("SOURCE_PREVIEW_PHYSICS_RETAINED_TABLE_HASH".into());
    }
    let table: Value = serde_json::from_slice(bytes)
        .map_err(|_| "SOURCE_PREVIEW_PHYSICS_RETAINED_TABLE_HASH".to_string())?;
    if table["semantic_contract_id"] != PREVIEW_PHYSICS_RETAINED_ID
        || table["formulation_profile_id"] != PREVIEW_PHYSICS_RETAINED_PROFILE
    {
        return Err("SOURCE_PREVIEW_PHYSICS_RETAINED_TABLE_IDENTITY".into());
    }
    Ok(table)
}
pub fn preview_physics_retained_contract() -> &'static Value {
    static CONTRACT: OnceLock<Value> = OnceLock::new();
    CONTRACT.get_or_init(|| {
        verify_preview_physics_retained_table(include_bytes!(
            "../../../../fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json"
        ))
        .expect("pinned preview-physics-retained semantic contract")
    })
}
/// Pinned exact successor table bytes: identity, profile and sha256 are checked.
pub fn verify_physics_retained_table(bytes: &[u8]) -> Result<Value, String> {
    use sha2::{Digest, Sha256};
    if format!("{:x}", Sha256::digest(bytes)) != PHYSICS_RETAINED_SHA256 {
        return Err("SOURCE_PHYSICS_RETAINED_TABLE_HASH".into());
    }
    let table: Value = serde_json::from_slice(bytes)
        .map_err(|_| "SOURCE_PHYSICS_RETAINED_TABLE_HASH".to_string())?;
    if table["semantic_contract_id"] != PHYSICS_RETAINED_ID
        || table["formulation_profile_id"] != PHYSICS_RETAINED_PROFILE
    {
        return Err("SOURCE_PHYSICS_RETAINED_TABLE_IDENTITY".into());
    }
    Ok(table)
}
pub fn physics_retained_contract() -> &'static Value {
    static CONTRACT: OnceLock<Value> = OnceLock::new();
    CONTRACT.get_or_init(|| {
        verify_physics_retained_table(include_bytes!(
            "../../../../fixtures/results/semantic_contract_v0_3_physics_retained_1.json"
        ))
        .expect("pinned physics-retained semantic contract")
    })
}
/// Either successor identity: the preview `<preview-physics-retained>` or the
/// exact `<physics-retained>` (B3b). Only these may carry a receipt.
pub(crate) fn is_retained(source: &Value) -> bool {
    source["producer"]["semantic_contract_id"] == PREVIEW_PHYSICS_RETAINED_ID
        || source["producer"]["semantic_contract_id"] == PHYSICS_RETAINED_ID
}
/// The successor's own pinned table, by identity.
fn retained_contract(source: &Value) -> &'static Value {
    if source["producer"]["semantic_contract_id"] == PHYSICS_RETAINED_ID {
        physics_retained_contract()
    } else {
        preview_physics_retained_contract()
    }
}
/// The reader's first failure as this module's error text: a G7 failure keeps
/// the base validator's own text, every other gate its code.
fn retained_error(error: crate::retained_precision::ValidationError) -> String {
    error.detail.unwrap_or(error.code)
}
/// F-5: no identity but the successor may carry a receipt member.
pub(crate) fn forbid_retained_member(source: &Value) -> Result<(), String> {
    if !is_retained(source) && source.get("retained_precision").is_some() {
        return Err(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN.into());
    }
    Ok(())
}
/// F-5 (C1 G6): no raw row outside the successor carries the W1 method token.
pub(crate) fn forbid_retained_rows(source: &Value) -> Result<(), String> {
    if !is_retained(source)
        && source["results"].as_array().is_some_and(|rows| {
            rows.iter()
                .any(|row| row["recovery_method"] == crate::retained_precision::METHOD)
        })
    {
        return Err(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN.into());
    }
    Ok(())
}
/// Validated successor row classes by result id; `None` for any other identity.
/// Classes come only from a statement the accepted reader validated (G5c).
pub fn retained_row_classes(
    source: &Value,
) -> Result<
    Option<std::collections::HashMap<String, crate::retained_precision::AccuracyClass>>,
    String,
> {
    if !is_retained(source) {
        return Ok(None);
    }
    let validation = crate::retained_precision::validate(source, None).map_err(retained_error)?;
    Ok(Some(
        validation
            .classifications
            .into_iter()
            .map(|row| (row.result_id, row.class))
            .collect(),
    ))
}
/// Current ordinary pressure rows have their own reviewed resident table.
/// The independently cached source table below retains its source-only family.
pub fn verify_pressure_table(bytes: &[u8]) -> Result<Value, String> {
    use sha2::{Digest, Sha256};
    if format!("{:x}", Sha256::digest(bytes)) != PRESSURE_TABLE_SHA256 {
        return Err("SOURCE_PRESSURE_TABLE_HASH".into());
    }
    let table: Value = serde_json::from_slice(bytes)
        .map_err(|_| "SOURCE_PRESSURE_TABLE_HASH".to_string())?;
    if table["semantic_contract_id"] != PRESSURE_ID
        || table["formulation_profile_id"] != PRESSURE_PROFILE
        || table["source_signature_count"] != PRESSURE_SIGNATURE_COUNT
        || table["source_kind_count"] != PRESSURE_KIND_COUNT {
        return Err("SOURCE_PRESSURE_TABLE_IDENTITY".into());
    }
    Ok(table)
}
pub fn pressure_rows_contract() -> &'static Value {
    static CONTRACT: OnceLock<Value> = OnceLock::new();
    CONTRACT.get_or_init(|| {
        verify_pressure_table(include_bytes!(
            "../../../../fixtures/results/semantic_contract_v0_3_pressure_2.json"
        )).expect("pinned ordinary pressure semantic contract")
    })
}
/// T4-U3 (S14): the objective connector row signatures of `pressure-2`:
/// (kind, unit, components, locations).
pub use crate::connector_evidence::KINDS as CONNECTOR_ROW_KINDS;
/// T4-U2: the connector rows' stable basis and the canonical metadata
/// vocabulary they add to the straight-member vocabulary under pressure-2.
pub use crate::connector_evidence::{
    ROW_BASIS as CONNECTOR_ROW_BASIS, VOCABULARY_BASES as CONNECTOR_VOCABULARY_BASES,
    VOCABULARY_COMPONENTS as CONNECTOR_VOCABULARY_COMPONENTS,
    VOCABULARY_COORDINATE_SYSTEMS as CONNECTOR_VOCABULARY_COORDINATE_SYSTEMS,
    VOCABULARY_LOCATIONS as CONNECTOR_VOCABULARY_LOCATIONS,
};
pub fn preview_physics_contract() -> &'static Value {
    static CONTRACT: OnceLock<Value> = OnceLock::new();
    CONTRACT.get_or_init(|| {
        serde_json::from_str(include_str!(
            "../../../../fixtures/results/semantic_contract_v0_3_preview_physics_2.json"
        ))
        .expect("pinned preview-physics semantic contract")
    })
}
/// Verify the independent current source table. Its full reviewed-byte digest
/// binds the 94 signatures, vocabulary, bases and hash vectors.
pub fn verify_pressure_source_table(bytes: &[u8]) -> Result<Value, String> {
    use sha2::{Digest, Sha256};
    if format!("{:x}", Sha256::digest(bytes)) != PRESSURE_SOURCE_TABLE_SHA256 {
        return Err("SOURCE_PRESSURE_SOURCE_TABLE_HASH".into());
    }
    let table: Value = serde_json::from_slice(bytes).map_err(|_| "SOURCE_PRESSURE_SOURCE_TABLE_HASH".to_string())?;
    if table["semantic_contract_id"] != PRESSURE_SOURCE_ID
        || table["formulation_profile_id"] != PRESSURE_PROFILE
        || table["source_block_policy"] != crate::physics_source::SourceContract::PressureV3.policy()
        || table["source_signature_count"] != PRESSURE_SOURCE_SIGNATURE_COUNT
        || table["source_kind_count"] != PRESSURE_SOURCE_KIND_COUNT {
        return Err("SOURCE_PRESSURE_SOURCE_TABLE_IDENTITY".into());
    }
    Ok(table)
}
pub fn pressure_source_contract() -> &'static Value {
    static CONTRACT: OnceLock<Value> = OnceLock::new();
    CONTRACT.get_or_init(|| {
        verify_pressure_source_table(include_bytes!(
            "../../../../fixtures/results/semantic_contract_v0_3_pressure_source_1.json"
        )).expect("pinned current pressure-source semantic contract")
    })
}
#[cfg(test)]
mod b7b_current_table_tests {
    use super::*;

    #[test]
    fn b7b_current_tables_are_independent_and_bind_their_own_bytes() {
        let ordinary = pressure_rows_contract();
        let source = pressure_source_contract();
        // T4-U5b's tie-rod force (107), then T4-U4b's 22 per-basis signatures (129).
        assert_eq!(ordinary["rows"].as_array().unwrap().len(), 129);
        // T4-U4b §4.3: + six straight derived signatures over two kinds.
        assert_eq!(source["rows"].as_array().unwrap().len(), 94);
        assert_eq!(source["source_kind_count"], 53);
        assert!(ordinary.get("inherited_semantic_contract_sha256").is_none());
        assert!(source.get("inherited_semantic_contracts").is_none());
        let scalar: Vec<_> = ordinary["rows"].as_array().unwrap().iter()
            .filter(|row| matches!(row["component"].as_str(), Some("pe_x" | "s_x"))).collect();
        assert_eq!(scalar.len(), 2, "padding creates no duplicate scalar signatures");
        for row in scalar {
            let input = serde_json::json!({"kind": row["kind"], "unit": row["unit"],
                "metadata": {"component": row["component"]}});
            assert!(signature_in(ordinary, &input).unwrap().is_some());
            assert!(signature_in(source, &input).unwrap().is_none(),
                "ordinary joint signatures never widen the source table");
        }
        let mut ordinary_bytes = include_bytes!(
            "../../../../fixtures/results/semantic_contract_v0_3_pressure_2.json"
        ).to_vec();
        ordinary_bytes.push(b' ');
        assert_eq!(verify_pressure_table(&ordinary_bytes).unwrap_err(), "SOURCE_PRESSURE_TABLE_HASH");
        let mut source_bytes = include_bytes!(
            "../../../../fixtures/results/semantic_contract_v0_3_pressure_source_1.json"
        ).to_vec();
        source_bytes.push(b' ');
        assert_eq!(verify_pressure_source_table(&source_bytes).unwrap_err(), "SOURCE_PRESSURE_SOURCE_TABLE_HASH");
    }

    /// T4-U6 (D-5, §2.4): every stress-family signature but the two thin-wall
    /// kinds is one per (recovery token × section basis), with its
    /// `source_basis`; the counts are re-derived from the rows; no
    /// (kind, unit, component) group mixes declared and undeclared bases, so a
    /// declared basis is always matched exactly and never widened.
    #[test]
    fn t4_u6_stress_signatures_are_per_basis_and_counts_are_derived() {
        use std::collections::{BTreeMap, BTreeSet};
        for table in [pressure_rows_contract(), pressure_source_contract(), physics_retained_contract()] {
            let rows = table["rows"].as_array().unwrap();
            let id = &table["semantic_contract_id"];
            assert_eq!(table["source_signature_count"], rows.len(), "{id}");
            let kinds: BTreeSet<_> = rows.iter().map(|r| r["kind"].as_str().unwrap()).collect();
            assert_eq!(table["source_kind_count"], kinds.len(), "{id}");
            assert_eq!(table["source_basis_signature_policy"],
                "where_declared_require_exact_metadata_basis_in_addition_to_kind_unit_component", "{id}");
            let vocabulary = table["canonical_metadata_vocabulary"]["basis"]["enum"].as_array().unwrap();
            let mut groups: BTreeMap<(String, String, String), BTreeSet<bool>> = BTreeMap::new();
            let mut identities = BTreeSet::new();
            for row in rows {
                let key = (row["kind"].to_string(), row["unit"].to_string(), row["component"].to_string());
                groups.entry(key.clone()).or_default().insert(row.get("source_basis").is_some());
                assert!(identities.insert((key, row["source_basis"].to_string())), "{id}: duplicate signature");
                let stress = row["family"] == "stress";
                let kind = row["kind"].as_str().unwrap();
                if stress && !crate::section_basis::is_thin_wall_kind(kind) {
                    let basis = row["source_basis"].as_str().unwrap_or_else(|| panic!("{id}: {kind} without basis"));
                    assert!(crate::section_basis::split_token(basis).is_some(), "{id}: {basis}");
                    assert!(vocabulary.contains(&row["source_basis"]), "{id}: {basis} not in vocabulary");
                } else {
                    assert!(row.get("source_basis").is_none_or(|b| crate::section_basis::split_token(b.as_str().unwrap()).is_none()), "{id}: {kind}");
                }
            }
            assert!(groups.values().all(|g| g.len() == 1), "{id}: a group mixes declared and undeclared bases");
            for kind in crate::section_basis::STRESS_KINDS {
                if !kinds.contains(kind) { continue; }
                for basis in crate::section_basis::BASES {
                    assert!(rows.iter().any(|r| r["kind"] == kind
                        && r["source_basis"].as_str().and_then(crate::section_basis::split_token).map(|t| t.1) == Some(basis)), "{id}: {kind} {basis}");
                }
            }
        }
        // A thin-wall row keeps its one basis-less signature: the readers refuse it on an exact envelope.
        let thin = serde_json::json!({"kind":"pipe_section_pressure_hoop_stress","unit":"MPa","metadata":{"component":"pressure_hoop_stress","basis":"anything"}});
        assert_eq!(signature_in(pressure_rows_contract(), &thin).unwrap().unwrap()["signature_id"], "supported-source-019");
        // A stress row with a bare recovery token matches no signature.
        let bare = serde_json::json!({"kind":"element_local_axial_normal_stress","unit":"MPa",
            "metadata":{"component":"axial_normal_stress","basis":"recovered_from_local_element_stiffness"}});
        assert_eq!(signature_in(pressure_rows_contract(), &bare).unwrap_err(), "SOURCE_BASIS_CONTRADICTION: element_local_axial_normal_stress");
        let named = serde_json::json!({"kind":"element_local_axial_normal_stress","unit":"MPa",
            "metadata":{"component":"axial_normal_stress","basis":"recovered_from_local_element_stiffness; section_basis=reduced_section"}});
        assert_eq!(signature_in(pressure_rows_contract(), &named).unwrap().unwrap()["signature_id"],
            "supported-source-015-local-element-stiffness-reduced-section");
    }
}
/// Header dispatch only; canonical envelopes have no raw rows. This does not
/// validate physical evidence and must never be used alone to qualify raw output.
pub fn for_source_metadata(source: &Value) -> Result<(&'static Value, &'static str), String> {
    if is_retained(source) {
        // Transport checks (G0-G2 plus the base metadata check on the reader's
        // projection); never eligible. The projection drops the receipt.
        crate::retained_precision::validate_transport_metadata(source).map_err(retained_error)?;
        return Ok((retained_contract(source), "0.3.0"));
    }
    forbid_retained_member(source)?;
    match source["schema_version"].as_str() {
        Some(MECHANICS_RESULT_VERSION) => current_header(source),
        Some(RETIRED_MECHANICS_RESULT_VERSION) => Err(MECHANICS_RESULT_VERSION_RETIRED.into()),
        _ => Err("SOURCE_SCHEMA_VERSION_UNSUPPORTED".into()),
    }
}
/// The one current mechanics-result carrier version (MechanicsEnvelope's
/// `schema_version`). Its producer header names the semantic contract.
pub const MECHANICS_RESULT_VERSION: &str = "0.2.0";
/// The producer-less 0.1.0 carrier is retired (b7-plan E2): it has no producer
/// or semantic contract and no current table reads it, so it is refused by
/// name (re-solve the model), never read.
pub const RETIRED_MECHANICS_RESULT_VERSION: &str = "0.1.0";
pub const MECHANICS_RESULT_VERSION_RETIRED: &str = "MECHANICS_RESULT_VERSION_RETIRED";
/// B4/D1 (b): the retired non-composite source-block identity, refused by name.
pub const RETIRED_SOURCE_BLOCKS_ID: &str = "openpipestress.result_semantics/0.3.0/source-blocks-1";
pub const SOURCE_BLOCKS_1_RETIRED: &str = "SOURCE_BLOCKS_1_RETIRED";
/// B7b (A4/A5): the identities of the retired `2.0.0/exact_straight_pressure_v2`
/// contract, refused by name (re-author the model and re-run); none is read.
pub const RETIRED_EXACT_V2_IDS: [&str; 4] = [
    "openpipestress.result_semantics/0.3.0/physics-1",
    "openpipestress.result_semantics/0.3.0/physics-source-1",
    "openpipestress.result_semantics/0.3.0/load-reference-1",
    "openpipestress.result_semantics/0.3.0/load-reference-source-1",
];
pub const SOURCE_EXACT_PRESSURE_V2_RETIRED: &str = "SOURCE_EXACT_PRESSURE_V2_RETIRED";
/// The one result-export version (b7-plan E2). Exports carry the source's
/// producer header as copied members; their other versions are retired.
pub const RESULT_EXPORT_VERSION: &str = "0.3.0";
/// Header dispatch of a result export: the members the derivative copied from
/// its source (producer, numerical quality, formulation basis, contract
/// evidence and receipts) are read as the transported header of a current
/// mechanics result, by the same checks as `for_source_metadata`, and the
/// export's `semantic_contract_ref` must name that producer's contract.
pub fn for_export_metadata(envelope: &Value) -> Result<(&'static Value, &'static str), String> {
    if envelope["schema_version"] != RESULT_EXPORT_VERSION {
        return Err("RESULT_EXPORT_VERSION_UNSUPPORTED".into());
    }
    let mut header = envelope.clone();
    header["schema_version"] = Value::from(MECHANICS_RESULT_VERSION);
    let selected = for_source_metadata(&header)?;
    if envelope["semantic_contract_ref"]
        != serde_json::json!({"ref_type": "semantic_contract", "ref_id": envelope["producer"]["semantic_contract_id"]})
    {
        return Err("SEMANTIC_CONTRACT_BINDING_MISMATCH".into());
    }
    Ok(selected)
}
fn current_header(source: &Value) -> Result<(&'static Value, &'static str), String> {
        {
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
            {
                return Err("SOURCE_PRODUCER_CONTRACT_UNSUPPORTED".into());
            }
            if p["semantic_contract_id"] == RETIRED_PRECISION_ID {
                return Err(format!("{SOURCE_PRECISION_1_RETIRED}: precision-1 results are no longer read; re-run the analysis"));
            }
            if p["semantic_contract_id"] == RETIRED_PRESSURE_1_ID {
                return Err(format!("{SOURCE_PRESSURE_1_RETIRED}: pressure-1 results are no longer read; re-run the analysis"));
            }
            if p["semantic_contract_id"] == RETIRED_PREVIEW_PHYSICS_1_ID {
                return Err(format!("{SOURCE_PREVIEW_PHYSICS_1_RETIRED}: preview-physics-1 results are no longer read; re-run the analysis"));
            }
            if p["semantic_contract_id"] == RETIRED_SOURCE_BLOCKS_ID {
                return Err(format!("{SOURCE_BLOCKS_1_RETIRED}: source-blocks-1 results are no longer read; re-run the analysis"));
            }
            if RETIRED_EXACT_V2_IDS.iter().any(|id| p["semantic_contract_id"] == *id) {
                return Err(format!("{SOURCE_EXACT_PRESSURE_V2_RETIRED}: 2.0.0/exact_straight_pressure_v2 results are no longer read; re-author the model as 0.7.0 with the current exact pressure contract and re-run the analysis"));
            }
            if !matches!(
                p["semantic_contract_id"].as_str(),
                Some(
                    PRESSURE_ID | PRESSURE_SOURCE_ID | PREVIEW_PHYSICS_ID
                )
            ) {
                return Err("SOURCE_PRODUCER_CONTRACT_UNSUPPORTED".into());
            }
            // The load/reference-state group owns no carrier namespace (the
            // retired load-reference routes' guard, kept on the current group).
            if source["contract_evidence"].get("load_reference_states").is_some()
                && source.get("carrier_evidence").is_some()
            {
                return Err("SOURCE_PRODUCER_CONTRACT_UNSUPPORTED".into());
            }
            if p["semantic_contract_id"] != PRESSURE_SOURCE_ID
                && source.get("source_block_recovery").is_some()
            {
                return Err("SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN".into());
            }
            if p["semantic_contract_id"] == PREVIEW_PHYSICS_ID
                && !source.get("contract_evidence").is_some_and(Value::is_object)
            {
                return Err("SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED".into());
            }
            match p["semantic_contract_id"].as_str() {
                Some(PRESSURE_SOURCE_ID) => crate::physics_source::SourceContract::PressureV3.check_received(source)?,
                _ => {}
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
                // T4-U2a: the only profile of pressure-1, on 0.3.0 and 0.4.0.
                Some(PRESSURE_ID | PRESSURE_SOURCE_ID) => PRESSURE_PROFILE,
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
                Some(PRESSURE_ID) => pressure_rows_contract(),
                Some(PRESSURE_SOURCE_ID) => pressure_source_contract(),
                Some(PREVIEW_PHYSICS_ID) => preview_physics_contract(),
                _ => return Err("SOURCE_PRODUCER_CONTRACT_UNSUPPORTED".into()),
            };
            Ok((table, RESULT_EXPORT_VERSION))
        }
}
/// Raw dispatch validates the selected physical contract, never inferring it
/// from a profile name or a numerical/accuracy label.
pub fn for_source(source: &Value) -> Result<(&'static Value, &'static str), String> {
    if is_retained(source) {
        // The accepted reader, G0-G7 without an invocation. Its G7 runs this
        // function's unchanged base branch on its own projection, which drops
        // the receipt and the method token, so this branch is not re-entered.
        crate::retained_precision::validate(source, None).map_err(retained_error)?;
        return Ok((retained_contract(source), "0.3.0"));
    }
    let selected = for_source_metadata(source)?;
    forbid_retained_rows(source)?;
    match source["producer"]["semantic_contract_id"].as_str() {
        Some(PREVIEW_PHYSICS_ID) => validate_preview_physics_evidence(source)?,
        Some(PRESSURE_ID) => validate_pressure_evidence(source)?,
        Some(PRESSURE_SOURCE_ID) => {
            crate::physics_source::validate_in(source, None, crate::physics_source::SourceContract::PressureV3)?;
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
pub use crate::physics_evidence::{validate_pressure_evidence, PressureContract};
pub use crate::preview_physics_evidence::validate_preview_physics_evidence;

/// Identities a fresh solve may publish (DESIGN 5.7, S1 10). One static set in
/// every language, never a route predicate. T1 added its identities on activation.
pub const FRESH_IDENTITIES: &[&str] = &[
    PREVIEW_PHYSICS_ID,
    PRESSURE_SOURCE_ID,
    // T4-U2: pressure-1 (`3.0.0/exact_pressure_v3`) on 0.3.0 and 0.4.0.
    PRESSURE_ID,
    // U6a (D-U6-6; D2 4.7 S-1): membership is not standing. The successor's
    // standing comes only from the accepted reader's verified receipt.
    PREVIEW_PHYSICS_RETAINED_ID,
];
pub fn is_fresh_identity(semantic_contract_id: &str) -> bool {
    FRESH_IDENTITIES.contains(&semantic_contract_id)
}
/// Why a readable source is not Current. Derived from the received bytes only.
/// B1 (precision-1) and B4/D1 (b) (source-blocks-1) retired the last
/// identities with a standing reason: both are refused by name before any
/// reading, so every readable source has none. The seam stays the readers'
/// one standing-reason answer.
pub fn standing_reason(_source: &Value) -> Option<&'static str> {
    None
}
/// Rule binding refusal (DESIGN 5.7 SF-B): a successor row by its validated
/// class. Every binding site calls this helper. (B4/D1 (b) retired the
/// non-composite source-blocks-1 summary refusal with that identity.)
pub fn rule_binding_refusal(envelope: &Value, row: &Value) -> Option<&'static str> {
    if is_retained(envelope) {
        return retained_binding_refusal(envelope, row);
    }
    None
}
/// D2 4.9.9 before S-I: a successor row binds only when its validated class
/// allows it. A headline binds the row its `result_ref` names, so it is
/// refused exactly as that row. A statement the reader refuses yields no
/// validated class for any row; every row is then refused (fail closed).
fn retained_binding_refusal(envelope: &Value, row: &Value) -> Option<&'static str> {
    let Ok(validation) = crate::retained_precision::validate(envelope, None) else {
        return Some(RULE_QUANTITY_NOT_COVERED);
    };
    let id = row["id"].as_str()?;
    validation
        .classifications
        .iter()
        .find(|c| c.result_id == id)
        .and_then(|c| class_binding_refusal(&c.class))
}
/// The binding refusal of one validated class (D2 4.9.9, before S-I).
#[doc(hidden)]
pub fn class_binding_refusal(
    class: &crate::retained_precision::AccuracyClass,
) -> Option<&'static str> {
    use crate::retained_precision::AccuracyClass;
    match class {
        AccuracyClass::AbsoluteVerified { .. } => Some(RULE_QUANTITY_BELOW_VERIFIED_FLOOR),
        AccuracyClass::NotCovered => Some(RULE_QUANTITY_NOT_COVERED),
        _ => None,
    }
}
/// D2 4.9.4: the successor's standing from an accepted-reader validation.
/// `numerically_eligible` needs all of: an invocation-bound validation whose
/// eligibility is set; requested refs equal to the receipt's case order;
/// `MECHANICS_SOLVED`; and every `not_required` case ordinarily eligible by
/// the base rules (C1:160; I66 F-7). `numerical_quality` never contributes
/// for a selected case. The reader's eligibility is on since U7; the other
/// conjuncts still apply.
#[doc(hidden)]
pub fn retained_standing_from(
    validation: &crate::retained_precision::Validation,
    source: &Value,
    requested_basis_refs: &[Value],
) -> &'static str {
    let cases = source["retained_precision"]["body"]["cases"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let expected: Vec<&Value> = cases.iter().map(|case| &case["basis_ref"]).collect();
    if !validation.invocation_bound
        || !validation.numerical_eligible
        || expected.len() != requested_basis_refs.len()
        || expected.iter().zip(requested_basis_refs).any(|(a, b)| *a != b)
        || source["status"]["mechanics"] != "MECHANICS_SOLVED"
        || !not_required_cases_ordinarily_eligible(source, cases)
    {
        return "needs_recompute";
    }
    "numerically_eligible"
}
/// Every case is `selected`, or `not_required` and ordinarily eligible by the
/// base rules (the generic branch below), by its receipt index. Any other
/// status (an `unavailable` case) is not eligible (D2 4.9.4).
fn not_required_cases_ordinarily_eligible(source: &Value, cases: &[Value]) -> bool {
    let quality = source["numerical_quality"]["cases"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let mut ids = std::collections::HashSet::new();
    for key in ["results", "diagnostics"] {
        for item in source[key].as_array().map(Vec::as_slice).unwrap_or(&[]) {
            match item["id"].as_str().filter(|id| !id.is_empty()) {
                Some(id) if ids.insert(id) => {}
                _ => return false,
            }
        }
    }
    cases.iter().enumerate().all(|(index, case)| {
        match case["status"].as_str() {
            Some("selected") => return true,
            Some("not_required") => {}
            _ => return false,
        }
        let Some(q) = quality.get(index) else {
            return false;
        };
        q["basis_ref"] == case["basis_ref"]
            && q["solve_quality"] == "checks_passed"
            && q["structural_status"] == "passive_model_basis"
            && q["model_matrix_fidelity"] == "represented_equations_retained"
            && matches!(
                q["accuracy_evidence"].as_str(),
                Some("not_claimed" | "reference_verified")
            )
            && q["evidence_refs"].as_array().is_some_and(|refs| {
                !refs.is_empty()
                    && refs
                        .iter()
                        .all(|r| r.as_str().is_some_and(|id| ids.contains(id)))
            })
    })
}
fn retained_standing(
    source: &Value,
    requested_basis_refs: &[Value],
    actual_invocation: Option<&Value>,
) -> &'static str {
    match crate::retained_precision::validate(source, actual_invocation) {
        Ok(validation) => retained_standing_from(&validation, source, requested_basis_refs),
        Err(_) => "unsupported",
    }
}
/// D2 4.9.9: per-case counts over the validated G5c classes of a successor
/// (rows of selected cases). `withheld` counts the quantity rows (every class
/// but `non_quantity`) that cannot bind under the envelope's standing: all of
/// them unless the standing with the caller's requested refs is
/// `numerically_eligible` (RV94 S-1, as TS); then the absolute and not-covered
/// rows, since S-I has not landed (`interval_bindable` is 0). Any other
/// identity, or a statement the reader refuses, returns nothing.
pub fn classification_summary(source: &Value, invocation: Option<&Value>, requested_basis_refs: &[Value]) -> Vec<Value> {
    match crate::retained_precision::validate(source, invocation) {
        Ok(validation) => classification_summary_from(&validation, source, requested_basis_refs),
        Err(_) => Vec::new(),
    }
}
/// `classification_summary` over an already validated successor statement.
#[doc(hidden)]
pub fn classification_summary_from(
    validation: &crate::retained_precision::Validation,
    source: &Value,
    requested_basis_refs: &[Value],
) -> Vec<Value> {
    use crate::retained_precision::AccuracyClass;
    // RV94 S-1 (the U7 repair, as TS's summary since e5e1693ceb): `withheld`
    // counts as Current only when the standing with the caller's requested refs
    // is `numerically_eligible`; with no refs, or other refs, it is the
    // not-Current count. The invocation only binds the validation; its own
    // cases are never substituted for the caller's refs (RV94 N-4), so this
    // borrows the caller's slice and builds no list of its own. A caller with no
    // refs (a library reader, or a summary shown without a model) therefore
    // sees every quantity row withheld.
    let current =
        retained_standing_from(validation, source, requested_basis_refs) == "numerically_eligible";
    source["retained_precision"]["body"]["cases"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or(&[])
        .iter()
        .map(|case| {
            let id = &case["basis_ref"]["ref_id"];
            let mut n = [0u64; 5];
            for c in validation
                .classifications
                .iter()
                .filter(|c| &c.basis_ref["ref_id"] == id)
            {
                n[match c.class {
                    AccuracyClass::RelativeVerified => 0,
                    AccuracyClass::AbsoluteVerified { .. } => 1,
                    AccuracyClass::NotCovered => 2,
                    AccuracyClass::InputDerived => 3,
                    AccuracyClass::NonQuantity => 4,
                }] += 1;
            }
            let withheld = if current { n[1] + n[2] } else { n[0] + n[1] + n[2] + n[3] };
            serde_json::json!({"case_id":id,"relative_verified":n[0],"absolute_verified":n[1],
                "interval_bindable":0,"not_covered":n[2],"input_derived":n[3],"non_quantity":n[4],
                "withheld":withheld})
        })
        .collect()
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
    if is_retained(source) {
        // Validated once, with the invocation; G7 includes the base dispatch.
        return retained_standing(source, requested_basis_refs, actual_invocation);
    }
    let Ok((_, version)) = for_source(source) else {
        return "unsupported";
    };
    // T0R (A2 item 10): validate first. (B1 and B4/D1 (b) refuse precision-1
    // and source-blocks-1 by name, so no readable source has a standing reason.)
    if version != "0.3.0" || requested_basis_refs.is_empty() {
        return "needs_recompute";
    }
    if matches!(
        source["producer"]["semantic_contract_id"].as_str(),
        Some(PRESSURE_SOURCE_ID)
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
        let standing = crate::physics_source::validate_in(source, actual_invocation, crate::physics_source::SourceContract::PressureV3);
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

#[cfg(test)]
mod e14_tests {
    use super::*;
    use serde_json::json;

    /// E14: the DEC-050/053 parity observation is retired; no current table
    /// carries its kind, so a row of it has no signature in any table.
    #[test]
    fn e14_no_table_carries_the_retired_parity_kind() {
        let row = json!({"kind":"sparse_live_path_dense_parity_relative_delta","unit":"unitless",
            "entity_ref":"solver:sparse_direct","value":0.0,
            "metadata":{"component":"sparse_live_path","coordinate_system":"reduced_system","location":"load_case",
                "basis":"retired","sign_convention":"retired"}});
        let tables = [preview_physics_retained_contract(), physics_retained_contract(),
            pressure_rows_contract(), preview_physics_contract(), pressure_source_contract()];
        for table in tables {
            assert!(table["rows"].as_array().unwrap().iter().all(|r| r["kind"] != row["kind"]), "{}", table["semantic_contract_id"]);
            assert_eq!(signature_in(table, &row).ok().flatten(), None, "{}", table["semantic_contract_id"]);
        }
        let pressure_source: Value = serde_json::from_str(include_str!(
            "../../../../fixtures/results/semantic_contract_v0_3_pressure_source_1.json"
        )).unwrap();
        assert!(pressure_source["rows"].as_array().unwrap().iter().all(|r| r["kind"] != row["kind"]));
    }
}
