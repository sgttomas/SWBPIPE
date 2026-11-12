//! Form A of the derivative binding (DEL-05-02): `derive_document` moves its six
//! per-row arrays into the document instead of copying them, every checksum
//! (the derivative hash among them) is the SHA-256 of canonical text streamed
//! into the hasher instead of held as a string, and `validate_document` hashes
//! the document less its own hash member in place instead of in a copy. The
//! former functions are kept below, verbatim, as the reference; the tests
//! compare the two document by document (the produced value, its canonical
//! bytes and its hash; every refusal's text), and the streamed writer with
//! `canonical_json` on every committed JSON document and on generated
//! adversarial values.
use super::{
    checksum_without, class_disclosure, digest_without, guard_json, is_successor, reference,
    refuse_declared_dimensions, write_canonical_without, CANONICALIZATION, OWN_HASH,
    RETAINED_ABSOLUTE_VERIFIED, RETAINED_NOT_COVERED, RETAINED_PRECISION_RECEIPT_BINDING_MISMATCH,
};
use crate::semantic_contract::{
    canonical_metadata_for, complete_metadata, for_source, retained_row_classes, signature_for,
    PHYSICS_RETAINED_ID, PREVIEW_PHYSICS_RETAINED_ID, RETAINED_PRECISION_DOWNGRADE_FORBIDDEN,
};
use open_pipe_stress_canonical_json::canonical_json;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};

// ------------------------------------------------------------------ the reference
// The former `digest`, `checksum`, `derive_document` and `validate_document`,
// verbatim (the unchanged helpers are imported above). They call one another as
// before; the current functions are reached as `super::`.

pub fn digest(v: &Value) -> Result<String, String> {
    guard_json(v)?;
    Ok(format!(
        "{:x}",
        Sha256::digest(canonical_json(v).as_bytes())
    ))
}
pub fn checksum(v: &Value, scope: &str, payload_ref: Value) -> Result<Value, String> {
    Ok(
        json!({"algorithm":"sha256","canonicalization":CANONICALIZATION,"payload_scope":scope,"payload_ref":payload_ref,"value":digest(v)?}),
    )
}

/// `base` carries qualified run/provenance/boundary facts, not numeric mappings.
/// `origin` is supplied only after the caller's source binding has succeeded.
pub fn derive_document(
    mut base: Value,
    model: &Value,
    source: &Value,
    origin: Value,
    request: Option<&Value>,
) -> Result<Value, String> {
    let (table, version) = for_source(source)?;
    let classes = retained_row_classes(source)?;
    guard_json(model)?;
    guard_json(source)?;
    guard_json(&base)?;
    guard_json(&origin)?;
    if source["status"]["mechanics"] != "MECHANICS_SOLVED" {
        return Err("SOURCE_NOT_SOLVED".into());
    }
    if model["project"]["id"] != source["model_ref"] {
        return Err("SOURCE_MODEL_IDENTITY_MISMATCH".into());
    }
    let rows = source["results"].as_array().ok_or("SOURCE_ROWS_MISSING")?;
    if rows.is_empty() {
        return Err("SOLVED_SOURCE_ROWS_EMPTY".into());
    }
    let mut ids = HashSet::new();
    let mut values = Vec::new();
    let mut reviews = Vec::new();
    let mut disclosures = Vec::new();
    let mut annotations = Vec::new();
    let mut accounting = Vec::new();
    let mut witnesses = Vec::new();
    let e = &mut base["result_envelope"];
    let mechanics_set = e["result_sets"][0].clone();
    e["result_sets"] = json!([mechanics_set]);
    e["schema_version"] = json!(version);
    {
        for key in ["producer", "numerical_quality", "formulation_basis"] {
            e[key] = source[key].clone();
        }
        if matches!(
            source["producer"]["semantic_contract_id"].as_str(),
            Some(
                crate::semantic_contract::PRESSURE_ID
                    | crate::semantic_contract::PRESSURE_SOURCE_ID
                    | crate::semantic_contract::PREVIEW_PHYSICS_ID
                    | crate::semantic_contract::PREVIEW_PHYSICS_TIMOSHENKO_ID
                    | PREVIEW_PHYSICS_RETAINED_ID
                    | PHYSICS_RETAINED_ID
            )
        ) {
            e["contract_evidence"] = source["contract_evidence"].clone();
        }
        // b7-plan MJ8: pressure-source-1's received v3 receipt travels whole
        // (D6 lapsed with T4-U2); it is the only source identity.
        if source["producer"]["semantic_contract_id"] == crate::semantic_contract::PRESSURE_SOURCE_ID {
            e["source_block_recovery"] = source["source_block_recovery"].clone();
        } else if e.get("source_block_recovery").is_some() {
            return Err("SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN".into());
        }
        // D2 4.9.7: the successor's receipt travels with the document, whole.
        // A receipt on any other identity's document is refused by the
        // validate_document call that ends this function.
        if is_successor(source) {
            e["retained_precision"] = source["retained_precision"].clone();
        }
        e["semantic_contract_ref"] = reference(
            "semantic_contract",
            source["producer"]["semantic_contract_id"].as_str().unwrap(),
        );
    }
    e["model_ref"] = reference(
        "model_payload",
        source["model_ref"].as_str().ok_or("MODEL_REF_MISSING")?,
    );
    let origin_ref = reference(
        "source_origin_binding",
        origin["origin_id"].as_str().ok_or("ORIGIN_ID_MISSING")?,
    );
    let run_basis = e["run_ref"].clone();
    let provenance = e["provenance"].clone();
    let unit_system = e["unit_system_ref"].clone();
    let carrier_ref = origin["received_carrier_checksum"]["payload_ref"].clone();
    let authentic = origin["authentic_producer_available"]
        .as_bool()
        .ok_or("ORIGIN_AVAILABILITY_MISSING")?;
    let scope = origin["received_carrier_checksum"]["payload_scope"]
        .as_str()
        .ok_or("ORIGIN_SCOPE_MISSING")?;
    if origin["received_carrier_checksum"]["value"] != digest(source)? {
        return Err("SOURCE_CARRIER_HASH_MISMATCH".into());
    }
    if !authentic && !origin["original_producer_checksum"].is_null() {
        return Err("UNAVAILABLE_PRODUCER_HASH".into());
    }
    refuse_declared_dimensions(scope, rows)?;
    for (index, row) in rows.iter().enumerate() {
        let id = row["id"]
            .as_str()
            .filter(|s| !s.is_empty())
            .ok_or("SOURCE_ID_MISSING")?;
        if !ids.insert(id) {
            return Err("DUPLICATE_SOURCE_ID".into());
        }
        row["value"]
            .as_f64()
            .filter(|x| x.is_finite())
            .ok_or("SOURCE_VALUE_INVALID")?;
        let value = row["value"].clone();
        let unit = row["unit"]
            .as_str()
            .filter(|s| !s.is_empty())
            .ok_or("SOURCE_UNIT_MISSING")?;
        let kind = row["kind"].as_str().ok_or("SOURCE_KIND_MISSING")?;
        let signature = signature_for(source, table, row)?;
        let s = signature.as_ref();
        let category = s.map(|s| s["category"].clone()).unwrap_or(json!("unknown"));
        let dimension = s
            .map(|s| s["derivative_target_dimension"].clone())
            .unwrap_or(Value::Null);
        let math = s
            .map(|s| s["source_physical_semantic_dimension"].clone())
            .unwrap_or(Value::Null);
        let disposition = s
            .map(|s| s["canonical_disposition"].as_str().unwrap())
            .unwrap_or("disclosed");
        let metadata = canonical_metadata_for(source, table, row);
        let family = s.map(|s| s["family"].clone()).unwrap_or(Value::Null);
        let mandatory = matches!(
            family.as_str(),
            Some("force" | "moment" | "section_property")
        );
        let review_missing = disposition == "exported_review" && !complete_metadata(row);
        let physical_missing =
            disposition == "exported_quantity" && mandatory && metadata.is_none();
        let class = class_disclosure(kind, unit, classes.as_ref().and_then(|c| c.get(id)));
        let actual = if review_missing || physical_missing || class.is_some() {
            "disclosed"
        } else {
            disposition
        };
        let observed = json!({"present":row.get("dimension").is_some(),"value":row.get("dimension").cloned().unwrap_or(Value::Null)});
        let path = format!("/results/{index}");
        let row_scope = match scope {
            "received_current_dimension_absent_carrier" => "received_current_dimension_absent_row",
            _ => "raw_source_row",
        };
        let row_hash = checksum(row, row_scope, carrier_ref.clone())?;
        let original = if authentic {
            checksum(
                row,
                "raw_source_row",
                origin["original_producer_checksum"]["payload_ref"].clone(),
            )?
        } else {
            Value::Null
        };
        let annotation_id = format!("source-annotation:{index}");
        let annotation_ref = reference("source_annotation", &annotation_id);
        let obj = reference(
            "preview_entity",
            row["entity_ref"].as_str().ok_or("ENTITY_REF_MISSING")?,
        );
        let basis = row
            .get("basis_ref")
            .filter(|v| !v.is_null())
            .cloned()
            .unwrap_or_else(|| {
                if actual == "exported_review" {
                    reference("source_basis", row["metadata"]["basis"].as_str().unwrap())
                } else {
                    run_basis.clone()
                }
            });
        let mut annotation = json!({"annotation_id":annotation_id,"source_row_index":index,"source_result_id":id,"metadata":row.get("metadata").cloned().unwrap_or(Value::Null),"basis_ref":row.get("basis_ref").cloned().unwrap_or(Value::Null),"source_result_refs":row.get("source_result_refs").cloned().unwrap_or(json!([])),"observed_carrier_dimension":observed,"source_physical_semantic_dimension":math,"derivative_target_dimension":dimension,"source_origin_ref":origin_ref});
        // W-15 slice d (DEL-03-05 Design §5): a body row's record, verbatim.
        let body = crate::component_body::derivative_record(row);
        if let Some(record) = &body {
            annotation[crate::component_body::RECORD] = record.clone();
        }
        annotations.push(annotation);
        let (target, path_target, target_type, target_scope) = if actual == "exported_quantity" {
            let mut target = json!({"result_id":id,"family":family,"object_ref":obj,"basis_ref":basis,"magnitude":value,"unit":unit,"dimension":dimension,"provenance":provenance,"source_kind":kind,"source_row_index":index,"source_annotation_ref":annotation_ref});
            if let Some(md) = metadata {
                target["metadata"] = md;
            }
            if let Some(record) = body {
                target[crate::component_body::RECORD] = record;
            }
            let pointer = format!("/result_envelope/result_sets/0/values/{}", values.len());
            values.push(target.clone());
            (target, pointer, "quantity_result", "derived_quantity_row")
        } else if actual == "exported_review" {
            let target = json!({"evidence_id":id,"source_row_index":index,"source_result_id":id,"source_kind":kind,"evidence_kind":if category=="assembled_load_review"{"assembled_load_review"}else{"user_input_review"},"magnitude":value,"unit":unit,"dimension":dimension,"object_ref":obj,"basis_ref":basis,"location_ref":reference("source_location",row["metadata"]["location"].as_str().unwrap()),"source_annotation_ref":annotation_ref,"provenance":provenance});
            let pointer = format!("/result_envelope/review_evidence/{}", reviews.len());
            reviews.push(target.clone());
            (target, pointer, "review_evidence", "derived_review_row")
        } else {
            let reason = if let Some((code, _)) = &class {
                *code
            } else if review_missing {
                "review_metadata_incomplete"
            } else if physical_missing {
                "physical_metadata_incomplete"
            } else if s.is_none() {
                "unsupported_source_kind"
            } else if category == "basis_record" {
                "basis_annotation_not_quantity"
            } else if category == "diagnostic_relative_ratio" {
                "diagnostic_relative_ratio_non_governing"
            } else if matches!(
                category.as_str(),
                Some("count" | "state" | "flag" | "solver_mode")
            ) {
                "discrete_evidence_not_ratio"
            } else {
                "diagnostic_evidence_not_physical_quantity"
            };
            let target = json!({"source_row_index":index,"source_result_id":id,"source_kind":kind,"source_value":value,"source_unit":unit,"source_dimension_present":observed["present"],"source_dimension":observed["value"],"declared_semantic_dimension":dimension,"source_physical_semantic_dimension":math,"semantic_category":category,"reason_code":reason,"object_ref":obj,"source_field_path":path,"source_annotation_ref":annotation_ref,"received_carrier_row_checksum":row_hash,"original_producer_row_checksum":original,"message":class.as_ref().map(|(_, message)| message.clone()).unwrap_or_else(|| format!("{kind}: {reason}; source value/unit and annotation retained; non-governing evidence"))});
            let pointer = format!("/result_envelope/row_disclosures/{}", disclosures.len());
            disclosures.push(target.clone());
            (target, pointer, "row_disclosure", "derived_disclosure_row")
        };
        let target_ref = reference(target_type, id);
        accounting.push(json!({"source_row_index":index,"source_result_id":id,"source_kind":kind,"source_field_path":path,"received_carrier_row_checksum":row_hash,"original_producer_row_checksum":original,"disposition":actual,"target_ref":target_ref,"target_field_path":path_target}));
        if actual != "disclosed" {
            witnesses.push(json!({"witness_id":format!("value-unit-witness:{index}"),"source_result_ref":reference("received_result",id),"source_field_path":path,"source_quantity":{"value":value,"unit":unit,"observed_dimension":observed},"target_result_ref":target_ref,"target_field_path":path_target,"target_quantity":{"value":value,"unit":unit,"dimension":dimension},"target_quantity_policy":"preserve_received_value_and_unit_with_independent_semantic_dimension","export_unit_policy":"no_numerical_conversion_preserve_received_unit","conversion_performed":false,"unit_system_ref":unit_system,"provenance":provenance,"source_row_index":index,"received_carrier_row_checksum":row_hash,"original_producer_row_checksum":original,"source_physical_semantic_dimension":math,"source_origin_ref":origin_ref,"target_row_checksum":checksum(&target,target_scope,reference("derivative_document",e["envelope_id"].as_str().unwrap()))?}));
        }
    }
    e["result_sets"][0]["values"] = json!(values);
    e["review_evidence"] = json!(reviews);
    e["row_disclosures"] = json!(disclosures);
    e["source_annotations"] = json!(annotations);
    e["row_accounting"] = json!(accounting);
    e["unit_preservation_witnesses"] = json!(witnesses);
    e["unit_witness_policy"] =
        json!("preserve_received_value_and_unit_with_independent_semantic_dimension");
    e["reproducibility"]["model_hash"] = checksum(
        model,
        "model_payload",
        reference("model_payload", source["model_ref"].as_str().unwrap()),
    )?;
    // The authentic producer's carrier, exactly as the origin binds it.
    e["reproducibility"]["raw_source_hashes"] = if authentic {
        json!([origin["original_producer_checksum"].clone()])
    } else {
        json!([])
    };
    e["reproducibility"]["source_origin_bindings"] = json!([origin]);
    e["reproducibility"]["request_hash"] = match request {
        Some(v) => checksum(
            v,
            "request_payload",
            reference(
                "runner_request",
                v["request_id"].as_str().unwrap_or("request"),
            ),
        )?,
        None => Value::Null,
    };
    e["reproducibility"]["derivative_hash_excludes"] =
        json!("result_envelope.reproducibility.derivative_hash");
    e["reproducibility"]
        .as_object_mut()
        .unwrap()
        .remove("derivative_hash");
    base["schema_version"] = json!(version);
    let hash = checksum(
        &base,
        "derivative_document_excludes_own_hash",
        reference(
            "derivative_document",
            base["result_envelope"]["envelope_id"].as_str().unwrap(),
        ),
    )?;
    base["result_envelope"]["reproducibility"]["derivative_hash"] = hash;
    validate_document(&base, source)?;
    Ok(base)
}

/// Independent structural/value/hash accounting check. Strict whole schema is
/// additionally exercised by the version-dispatched schema validation gate.
pub fn validate_document(doc: &Value, source: &Value) -> Result<(), String> {
    guard_json(doc)?;
    guard_json(source)?;
    let (table, version) = for_source(source)?;
    let classes = retained_row_classes(source)?;
    if source["producer"]["semantic_contract_id"] == crate::semantic_contract::PRESSURE_SOURCE_ID {
        if !crate::json_value_eq(&doc["result_envelope"]["source_block_recovery"], &source["source_block_recovery"]) {
            return Err("SOURCE_BLOCK_RECEIPT_BINDING_MISMATCH".into());
        }
    } else if doc["result_envelope"]
        .get("source_block_recovery")
        .is_some()
    {
        return Err("SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN".into());
    }
    // D2 4.9.7: the copied receipt equals the source's, whole; no other
    // identity's document carries one.
    if is_successor(source) {
        if !crate::json_value_eq(&doc["result_envelope"]["retained_precision"], &source["retained_precision"]) {
            return Err(RETAINED_PRECISION_RECEIPT_BINDING_MISMATCH.into());
        }
    } else if doc["result_envelope"].get("retained_precision").is_some() {
        return Err(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN.into());
    }
    {
        for key in ["producer", "numerical_quality", "formulation_basis"] {
            if !crate::json_value_eq(&doc["result_envelope"][key], &source[key]) {
                return Err("SOURCE_METADATA_BINDING_MISMATCH".into());
            }
        }
        let evidence = source.get("contract_evidence").filter(|v| !v.is_null());
        if !match (doc["result_envelope"].get("contract_evidence"), evidence) {
            (Some(copied), Some(evidence)) => crate::json_value_eq(copied, evidence),
            (copied, evidence) => copied == evidence,
        } {
            return Err("SOURCE_CONTRACT_EVIDENCE_BINDING_MISMATCH".into());
        }
        if doc["result_envelope"]["semantic_contract_ref"]
            != reference(
                "semantic_contract",
                source["producer"]["semantic_contract_id"].as_str().unwrap(),
            )
        {
            return Err("SEMANTIC_CONTRACT_BINDING_MISMATCH".into());
        }
    }
    if doc["schema_version"] != version || doc["result_envelope"]["schema_version"] != version {
        return Err("DERIVATIVE_VERSION_MISMATCH".into());
    }
    let e = &doc["result_envelope"];
    let rows = source["results"].as_array().ok_or("SOURCE_ROWS_MISSING")?;
    let accounts = e["row_accounting"].as_array().ok_or("ACCOUNTING_MISSING")?;
    let annotations = e["source_annotations"]
        .as_array()
        .ok_or("ANNOTATIONS_MISSING")?;
    let sets = e["result_sets"].as_array().ok_or("RESULT_SETS_MISSING")?;
    let reviews = e["review_evidence"]
        .as_array()
        .ok_or("REVIEW_EVIDENCE_MISSING")?;
    let disclosures = e["row_disclosures"]
        .as_array()
        .ok_or("DISCLOSURES_MISSING")?;
    let witnesses = e["unit_preservation_witnesses"]
        .as_array()
        .ok_or("WITNESSES_MISSING")?;
    let origins = e["reproducibility"]["source_origin_bindings"]
        .as_array()
        .ok_or("ORIGINS_MISSING")?;
    if origins.len() != 1 {
        return Err("ORIGIN_CARDINALITY".into());
    }
    let origin = &origins[0];
    let envelope_id = e["envelope_id"].as_str().ok_or("ENVELOPE_ID_MISSING")?;
    let origin_ref = reference(
        "source_origin_binding",
        origin["origin_id"].as_str().ok_or("ORIGIN_ID_MISSING")?,
    );
    let model_ref = source["model_ref"].as_str().ok_or("MODEL_REF_MISSING")?;
    if origin["actual_model_ref"] != reference("model_payload", model_ref)
        || e["model_ref"] != reference("model_payload", model_ref)
        || origin["mechanics_run_ref"]
            != reference(
                "mechanics_run",
                source["run_id"].as_str().ok_or("MECHANICS_RUN_MISSING")?,
            )
    {
        return Err("ORIGIN_IDENTITY_MISMATCH".into());
    }
    let authentic = origin["authentic_producer_available"]
        .as_bool()
        .ok_or("ORIGIN_AVAILABILITY_MISSING")?;
    let carrier_scope = origin["received_carrier_checksum"]["payload_scope"]
        .as_str()
        .ok_or("ORIGIN_SCOPE_MISSING")?;
    let row_scope = match carrier_scope {
        "received_current_dimension_absent_carrier" => "received_current_dimension_absent_row",
        "attested_headless_producer_carrier" => "raw_source_row",
        _ => return Err("ORIGIN_SCOPE_INVALID".into()),
    };
    let carrier_ref = origin["received_carrier_checksum"]["payload_ref"].clone();
    if origin["received_carrier_checksum"] != checksum(source, carrier_scope, carrier_ref.clone())?
    {
        return Err("SOURCE_CARRIER_HASH_MISMATCH".into());
    }
    refuse_declared_dimensions(carrier_scope, rows)?;
    if authentic {
        if origin["original_producer_checksum"] != origin["received_carrier_checksum"] {
            return Err("ORIGINAL_PRODUCER_BINDING_MISMATCH".into());
        }
    } else if !origin["original_producer_checksum"].is_null() {
        return Err("UNAVAILABLE_PRODUCER_HASH".into());
    }
    if accounts.len() != rows.len() || annotations.len() != rows.len() {
        return Err("ROW_ACCOUNTING_CARDINALITY".into());
    }
    let mut total = reviews.len() + disclosures.len();
    for set in sets {
        total += set["values"].as_array().ok_or("QUANTITIES_MISSING")?.len();
    }
    if total != rows.len() {
        return Err("TARGET_ACCOUNTING_CARDINALITY".into());
    }
    let mut pointers = HashSet::new();
    let mut source_ids = HashSet::new();
    let mut expected_witnesses = Vec::new();
    for (i, (a, row)) in accounts.iter().zip(rows).enumerate() {
        let id = row["id"].as_str().ok_or("SOURCE_ID_MISSING")?;
        if !source_ids.insert(id)
            || a["source_row_index"] != i
            || a["source_result_id"] != row["id"]
            || a["source_kind"] != row["kind"]
            || a["source_field_path"] != format!("/results/{i}")
        {
            return Err("SOURCE_ACCOUNTING_IDENTITY_MISMATCH".into());
        }
        let path = a["target_field_path"]
            .as_str()
            .ok_or("TARGET_POINTER_MISSING")?;
        if !pointers.insert(path) {
            return Err("TARGET_DUPLICATE".into());
        }
        let target = doc.pointer(path).ok_or("TARGET_POINTER_UNRESOLVED")?;
        let signature = signature_for(source, table, row)?;
        let s = signature.as_ref();
        let md = canonical_metadata_for(source, table, row);
        let family = s.map(|s| s["family"].clone()).unwrap_or(Value::Null);
        let category = s.map(|s| s["category"].clone()).unwrap_or(json!("unknown"));
        let mut disposition = s
            .map(|s| s["canonical_disposition"].as_str().unwrap())
            .unwrap_or("disclosed");
        if (disposition == "exported_review" && !complete_metadata(row))
            || (disposition == "exported_quantity"
                && matches!(
                    family.as_str(),
                    Some("force" | "moment" | "section_property")
                )
                && md.is_none())
        {
            disposition = "disclosed";
        }
        let class = class_disclosure(
            row["kind"].as_str().ok_or("SOURCE_KIND_MISSING")?,
            row["unit"].as_str().ok_or("SOURCE_UNIT_MISSING")?,
            classes.as_ref().and_then(|c| c.get(id)),
        );
        if class.is_some() {
            disposition = "disclosed";
        }
        let (target_type, prefix, target_id, target_scope) = match disposition {
            "exported_quantity" => (
                "quantity_result",
                "/result_envelope/result_sets/0/values/",
                &target["result_id"],
                "derived_quantity_row",
            ),
            "exported_review" => (
                "review_evidence",
                "/result_envelope/review_evidence/",
                &target["evidence_id"],
                "derived_review_row",
            ),
            _ => (
                "row_disclosure",
                "/result_envelope/row_disclosures/",
                &target["source_result_id"],
                "derived_disclosure_row",
            ),
        };
        if a["disposition"] != disposition
            || !path.starts_with(prefix)
            || path[prefix.len()..].parse::<usize>().is_err()
            || target_id != &row["id"]
            || a["target_ref"] != reference(target_type, id)
            || target["source_row_index"] != i
            || target["source_kind"] != row["kind"]
        {
            return Err("TARGET_IDENTITY_OR_DISPOSITION_MISMATCH".into());
        }
        let math = s
            .map(|s| s["source_physical_semantic_dimension"].clone())
            .unwrap_or(Value::Null);
        let dimension = s
            .map(|s| s["derivative_target_dimension"].clone())
            .unwrap_or(Value::Null);
        let observed = json!({"present":row.get("dimension").is_some(),"value":row.get("dimension").cloned().unwrap_or(Value::Null)});
        let ann_id = format!("source-annotation:{i}");
        let ann_ref = reference("source_annotation", &ann_id);
        let ann = &annotations[i];
        if ann["annotation_id"] != ann_id
            || ann["source_row_index"] != i
            || ann["source_result_id"] != row["id"]
            || !crate::json_value_eq(&ann["metadata"], row.get("metadata").unwrap_or(&Value::Null))
            || ann["basis_ref"] != row.get("basis_ref").cloned().unwrap_or(Value::Null)
            || ann["source_result_refs"]
                != row.get("source_result_refs").cloned().unwrap_or(json!([]))
            || ann["observed_carrier_dimension"] != observed
            || ann["source_physical_semantic_dimension"] != math
            || ann["derivative_target_dimension"] != dimension
            || ann["source_origin_ref"] != origin_ref
            || target["source_annotation_ref"] != ann_ref
            || !crate::component_body::derivative_agrees(ann, row)
        {
            return Err("SOURCE_ANNOTATION_MISMATCH".into());
        }
        // W-15 slice d: a derivative quantity carries its source row's record
        // verbatim; a review or disclosure target carries none; no target
        // carries the coverage-only body list.
        if (disposition == "exported_quantity" && !crate::component_body::derivative_agrees(target, row))
            || (disposition != "exported_quantity"
                && (target.get(crate::component_body::RECORD).is_some() || target.get(crate::component_body::COVERAGE_KEY).is_some()))
        {
            return Err("SOURCE_COMPONENT_BODY_MISMATCH".into());
        }
        let row_hash = checksum(row, row_scope, carrier_ref.clone())?;
        let original = if authentic {
            checksum(
                row,
                "raw_source_row",
                origin["original_producer_checksum"]["payload_ref"].clone(),
            )?
        } else {
            Value::Null
        };
        if a["received_carrier_row_checksum"] != row_hash
            || a["original_producer_row_checksum"] != original
        {
            return Err("SOURCE_ROW_HASH_BINDING_MISMATCH".into());
        }
        let (value, unit) = if disposition == "disclosed" {
            (&target["source_value"], &target["source_unit"])
        } else {
            (&target["magnitude"], &target["unit"])
        };
        if !crate::json_value_eq(value, &row["value"])
            || unit != &row["unit"]
            || target["object_ref"]
                != reference(
                    "preview_entity",
                    row["entity_ref"].as_str().ok_or("ENTITY_REF_MISSING")?,
                )
        {
            return Err("SOURCE_TARGET_VALUE_OR_HASH_MISMATCH".into());
        }
        let basis = row
            .get("basis_ref")
            .filter(|v| !v.is_null())
            .cloned()
            .unwrap_or_else(|| {
                if disposition == "exported_review" {
                    reference("source_basis", row["metadata"]["basis"].as_str().unwrap())
                } else {
                    e["run_ref"].clone()
                }
            });
        if disposition != "disclosed"
            && (target["basis_ref"] != basis || target["provenance"] != e["provenance"])
        {
            return Err("TARGET_BASIS_OR_PROVENANCE_MISMATCH".into());
        }
        if disposition == "exported_review"
            && (target["source_result_id"] != row["id"]
                || target["location_ref"]
                    != reference(
                        "source_location",
                        row["metadata"]["location"].as_str().unwrap(),
                    )
                || target["evidence_kind"]
                    != if category == "assembled_load_review" {
                        json!("assembled_load_review")
                    } else {
                        json!("user_input_review")
                    })
        {
            return Err("REVIEW_METADATA_MISMATCH".into());
        }
        if disposition == "disclosed" {
            if target["declared_semantic_dimension"] != dimension
                || target["source_physical_semantic_dimension"] != math
                || target["semantic_category"] != category
                || target["source_dimension_present"] != observed["present"]
                || target["source_dimension"] != observed["value"]
                || target["source_field_path"] != a["source_field_path"]
                || target["received_carrier_row_checksum"] != row_hash
                || target["original_producer_row_checksum"] != original
            {
                return Err("DISCLOSURE_SEMANTICS_MISMATCH".into());
            }
            // D-U6-2: a class disclosure states exactly its class code and
            // message; no other disclosure of a successor claims a class code.
            if classes.is_some() {
                let claimed = target["reason_code"] == RETAINED_ABSOLUTE_VERIFIED
                    || target["reason_code"] == RETAINED_NOT_COVERED;
                let consistent = match &class {
                    Some((code, message)) => {
                        target["reason_code"] == *code && target["message"] == *message
                    }
                    None => !claimed,
                };
                if !consistent {
                    return Err("DISCLOSURE_SEMANTICS_MISMATCH".into());
                }
            }
        } else {
            if target["dimension"] != dimension {
                return Err("TARGET_DIMENSION_MISMATCH".into());
            }
            if disposition == "exported_quantity"
                && (target["family"] != family || target.get("metadata").cloned() != md)
            {
                return Err("PHYSICAL_SEMANTICS_MISMATCH".into());
            }
            expected_witnesses.push((
                i,
                a,
                target,
                target_scope,
                row_hash,
                original,
                observed,
                math,
                dimension,
            ));
        }
    }
    if witnesses.len() != expected_witnesses.len() {
        return Err("WITNESS_CARDINALITY".into());
    }
    for (w, (i, a, target, scope, row_hash, original, observed, math, dimension)) in
        witnesses.iter().zip(expected_witnesses)
    {
        let row = &rows[i];
        if w["witness_id"] != format!("value-unit-witness:{i}")
            || w["source_row_index"] != i
            || w["source_result_ref"] != reference("received_result", row["id"].as_str().unwrap())
            || w["source_field_path"] != a["source_field_path"]
            || !crate::json_value_eq(
                &w["source_quantity"],
                &json!({"value":row["value"],"unit":row["unit"],"observed_dimension":observed}),
            )
            || w["target_result_ref"] != a["target_ref"]
            || w["target_field_path"] != a["target_field_path"]
            || !crate::json_value_eq(
                &w["target_quantity"],
                &json!({"value":row["value"],"unit":row["unit"],"dimension":dimension}),
            )
            || w["received_carrier_row_checksum"] != row_hash
            || w["original_producer_row_checksum"] != original
            || w["source_physical_semantic_dimension"] != math
            || w["source_origin_ref"] != origin_ref
            || w["unit_system_ref"] != e["unit_system_ref"]
            || w["provenance"] != e["provenance"]
            || w["conversion_performed"] != false
            || w["target_quantity_policy"]
                != "preserve_received_value_and_unit_with_independent_semantic_dimension"
            || w["export_unit_policy"] != "no_numerical_conversion_preserve_received_unit"
            || w["target_row_checksum"]
                != checksum(target, scope, reference("derivative_document", envelope_id))?
        {
            return Err("WITNESS_MISMATCH".into());
        }
    }
    let mut payload = doc.clone();
    payload["result_envelope"]["reproducibility"]
        .as_object_mut()
        .ok_or("REPRODUCIBILITY_MISSING")?
        .remove("derivative_hash");
    if e["reproducibility"]["derivative_hash"]
        != checksum(
            &payload,
            "derivative_document_excludes_own_hash",
            reference("derivative_document", envelope_id),
        )?
    {
        return Err("DERIVATIVE_HASH_MISMATCH".into());
    }
    Ok(())
}

// ------------------------------------------------------------------ the corpus

/// The repository root.
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

/// Every JSON/JSONL file under `fixtures/`, `validation/` and `core/`, in path
/// order, skipping build, dependency and hidden trees.
fn corpus_files() -> Vec<PathBuf> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        let mut entries: Vec<_> = std::fs::read_dir(dir).unwrap().map(|e| e.unwrap().path()).collect();
        entries.sort();
        for path in entries {
            let name = path.file_name().unwrap().to_string_lossy().into_owned();
            if path.is_dir() {
                if !(name.starts_with('.') || name == "target" || name == "node_modules" || name == "dist") {
                    walk(&path, out);
                }
            } else if name.ends_with(".json") || name.ends_with(".jsonl") {
                out.push(path);
            }
        }
    }
    let mut out = Vec::new();
    for top in ["fixtures", "validation", "core"] {
        walk(&root().join(top), &mut out);
    }
    out
}

/// A progress line every hundred files, so that a long corpus pass is never silent.
fn progress(test: &str, index: usize, files: usize) {
    if index % 100 == 0 {
        eprintln!("{test}: file {index} of {files}");
    }
}

/// The file's path relative to the repository root.
fn relative(path: &Path) -> String {
    let root = root().canonicalize().unwrap();
    path.canonicalize().unwrap().strip_prefix(&root).unwrap().to_string_lossy().into_owned()
}

/// Each JSON document a file holds (a JSONL file one per non-empty line), with
/// its label; unparsable files and lines are skipped.
fn corpus_documents(path: &Path) -> Vec<(String, Value)> {
    let Ok(text) = std::fs::read_to_string(path) else { return Vec::new() };
    let name = relative(path);
    if path.extension().is_some_and(|e| e == "jsonl") {
        text.lines()
            .enumerate()
            .filter(|(_, l)| !l.trim().is_empty())
            .filter_map(|(i, l)| serde_json::from_str(l).ok().map(|v| (format!("{name}:{}", i + 1), v)))
            .collect()
    } else {
        serde_json::from_str(&text).ok().map(|v| (name, v)).into_iter().collect()
    }
}

/// A solver source: an object with result rows, a producer and a model reference.
fn is_source(v: &Value) -> bool {
    v["results"].is_array() && v["producer"].is_object() && v["model_ref"].is_string()
}

/// A derivative document: an object whose reproducibility carries a derivative hash.
fn is_derivative(v: &Value) -> bool {
    v["result_envelope"]["reproducibility"].get("derivative_hash").is_some()
}

/// The JSON Pointer of every value within `v` (itself first) that satisfies `test`.
fn pointers(v: &Value, test: fn(&Value) -> bool) -> Vec<String> {
    fn walk(v: &Value, at: String, test: fn(&Value) -> bool, out: &mut Vec<String>) {
        if test(v) {
            out.push(at.clone());
        }
        match v {
            Value::Object(map) => {
                for (key, x) in map {
                    walk(x, format!("{at}/{}", key.replace('~', "~0").replace('/', "~1")), test, out);
                }
            }
            Value::Array(items) => {
                for (i, x) in items.iter().enumerate() {
                    walk(x, format!("{at}/{i}"), test, out);
                }
            }
            _ => {}
        }
    }
    let mut out = Vec::new();
    walk(v, String::new(), test, &mut out);
    out
}

/// `f`'s value, or the panic's message (both forms must panic alike).
fn outcome<T>(f: impl FnOnce() -> T) -> Result<T, String> {
    catch_unwind(AssertUnwindSafe(f)).map_err(|p| {
        let text = p.downcast_ref::<String>().cloned().or_else(|| p.downcast_ref::<&str>().map(|s| s.to_string()));
        format!("PANIC: {}", text.unwrap_or_default())
    })
}

/// The base every source is derived on: the committed canonical solve envelope.
fn base() -> Value {
    serde_json::from_str(include_str!("../../../../fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json")).unwrap()
}

/// The two origin forms: an attested headless producer with its original
/// checksum, and a received dimension-absent carrier without one.
fn origin(source: &Value, attested: bool) -> Value {
    let model = source["model_ref"].as_str().unwrap_or("model");
    let run = source["run_id"].as_str().unwrap_or("run");
    if attested {
        let carrier = checksum(source, "attested_headless_producer_carrier", reference("test_carrier", "derivative-identity")).unwrap();
        json!({"origin_id":"derivative-identity-test","origin_class":"attested_headless_producer","qualification_ref":reference("test_fixture","not-authentication"),"authentic_producer_available":true,"received_carrier_checksum":carrier,"original_producer_checksum":carrier,"origin_limit":"Retained producer bytes; derivative identity test only","actual_model_ref":reference("model_payload",model),"mechanics_run_ref":reference("mechanics_run",run),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null})
    } else {
        let carrier = checksum(source, "received_current_dimension_absent_carrier", reference("received_current_carrier", run)).unwrap();
        json!({"origin_id":"source-origin:current-received","origin_class":"received_current_dimension_absent","qualification_ref":reference("current_manifest","derivative-identity"),"authentic_producer_available":false,"received_carrier_checksum":carrier,"original_producer_checksum":null,"origin_limit":"Received bytes; derivative identity test only","actual_model_ref":reference("model_payload",model),"mechanics_run_ref":reference("mechanics_run",run),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null})
    }
}

/// Derives `source` both ways and asserts the same outcome: the same document
/// (value, canonical bytes, derivative hash) or the same refusal.
fn same_derivation(label: &str, base: &Value, model: &Value, source: &Value, origin: &Value, request: Option<&Value>) -> Option<Value> {
    let former = outcome(|| derive_document(base.clone(), model, source, origin.clone(), request));
    let now = outcome(|| super::derive_document(base.clone(), model, source, origin.clone(), request));
    match (former, now) {
        (Ok(Ok(former)), Ok(Ok(now))) => {
            assert!(former == now, "{label}: the derived documents differ");
            assert_eq!(canonical_json(&former), canonical_json(&now), "{label}: canonical bytes");
            let hash = &now["result_envelope"]["reproducibility"]["derivative_hash"];
            assert!(hash["value"].as_str().is_some_and(|h| h.len() == 64), "{label}: {hash}");
            assert_eq!(former["result_envelope"]["reproducibility"]["derivative_hash"], *hash, "{label}: derivative hash");
            Some(now)
        }
        (former, now) => {
            let flat = |r: Result<Result<Value, String>, String>| r.and_then(|r| r.map(|_| "OK".to_string()));
            assert_eq!(flat(former), flat(now), "{label}: the refusals differ");
            None
        }
    }
}

/// Validates both ways and asserts the same verdict (the same refusal text).
fn same_validation(label: &str, doc: &Value, source: &Value) -> Result<(), String> {
    let former = outcome(|| validate_document(doc, source)).and_then(|r| r);
    let now = outcome(|| super::validate_document(doc, source)).and_then(|r| r);
    assert_eq!(former, now, "{label}: the validation verdicts differ");
    now
}

/// Mutations of a derived document that reach the hash check and the refusals
/// around it, each with its name.
fn mutations(doc: &Value) -> Vec<(&'static str, Value)> {
    let mut out = Vec::new();
    let mut m = |name, f: &dyn Fn(&mut Value)| {
        let mut v = doc.clone();
        f(&mut v);
        out.push((name, v));
    };
    m("hash member removed", &|v| {
        v["result_envelope"]["reproducibility"].as_object_mut().unwrap().remove("derivative_hash");
    });
    m("hash value altered", &|v| {
        v["result_envelope"]["reproducibility"]["derivative_hash"]["value"] = json!("0".repeat(64));
    });
    m("hash member null", &|v| v["result_envelope"]["reproducibility"]["derivative_hash"] = Value::Null);
    m("hash member an unsafe number", &|v| {
        v["result_envelope"]["reproducibility"]["derivative_hash"] = json!(9_007_199_254_740_992u64)
    });
    m("member beside the hash", &|v| v["result_envelope"]["reproducibility"]["derivative_hasi"] = json!(-0.0));
    m("supplementary-plane key beside the hash", &|v| {
        v["result_envelope"]["reproducibility"]["\u{1f600}"] = json!("\u{e000}")
    });
    m("hash exclusion restated", &|v| {
        v["result_envelope"]["reproducibility"]["derivative_hash_excludes"] = json!("result_envelope.reproducibility")
    });
    m("hash member at the top level", &|v| v["derivative_hash"] = json!({}));
    m("reproducibility emptied", &|v| v["result_envelope"]["reproducibility"] = json!({}));
    m("reproducibility a string", &|v| v["result_envelope"]["reproducibility"] = json!("x"));
    m("schema version", &|v| v["schema_version"] = json!("0.0.0"));
    m("first row accounting removed", &|v| {
        if let Some(rows) = v["result_envelope"]["row_accounting"].as_array_mut() {
            rows.remove(0);
        }
    });
    m("unsafe number in a value", &|v| v["result_envelope"]["extra"] = json!(1e300));
    m("not an object", &|v| *v = json!([]));
    out
}

/// Every source this crate's and its callers' tests derive (a subset of the
/// corpus, by label), each of which must derive.
const NAMED: [&str; 21] = [
    "fixtures/results/preview_physics_invented_sparse.json#",
    "fixtures/results/preview_physics_connected_sparse.json#",
    "fixtures/results/preview_physics_connected_dense.json#",
    "fixtures/results/preview_physics_timoshenko_arc_sparse.json#",
    "fixtures/results/retained_precision_milestone_successor_sparse_interactive.json#/source",
    "fixtures/results/retained_precision_milestone_successor_dense_scrutiny.json#/source",
    "fixtures/results/retained_precision_exact_successor_sparse_interactive.json#/source",
    "fixtures/results/retained_precision_exact_successor_dense_scrutiny.json#/source",
    "fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json#",
    "fixtures/product_preview/pressure_source/load_reference_source/fields-sparse_interactive.raw.json#",
    "fixtures/reports/invented/exact_report_witness_inputs/pressure_2_connector-sparse_interactive.raw.json#",
    "fixtures/reports/invented/exact_report_witness_inputs/pressure_2_realized_bend-sparse_interactive.raw.json#",
    "fixtures/reports/invented/user_release_exact/pressure_2_straight.json#/source",
    "fixtures/reports/invented/user_release_exact/pressure_source_1.json#/source",
    "fixtures/reports/invented/exact_report_package_requests/pressure_2_connector.json#/request/source_results/0",
    "fixtures/reports/invented/exact_report_package_requests/pressure_2_load_reference_states.json#/request/source_results/0",
    "fixtures/reports/invented/exact_report_package_requests/pressure_2_realized_bend.json#/request/source_results/0",
    "fixtures/reports/invented/exact_report_package_requests/pressure_2_straight.json#/request/source_results/0",
    "fixtures/reports/invented/exact_report_package_requests/pressure_source_1.json#/request/source_results/0",
    "fixtures/results/pressure_v3_straight_reader_corpus.json#/cases/0/envelope",
    "validation/witness/inputs/del1005_export_results_success_input.json#/export_results/source_results/0",
];

/// The floors the corpus must reach, so that no test passes on an empty or
/// shrunken corpus (the counts found are printed).
const MIN_SOURCES: usize = 170;
const MIN_DERIVED: usize = 150;
const MIN_REFUSED: usize = 15;
const MIN_COMMITTED_PAIRS: usize = 30;
const MIN_DOCUMENTS: usize = 1000;
const MIN_IDENTITIES: usize = 4;

/// (a) Every committed source (or, unless `full`, every named one), derived on the
/// committed base in one origin form (attested, or dimension-absent with a
/// request), both ways: the same document and hash, or the same refusal; every
/// derived document validated both ways; and the smallest derived document of
/// each semantic contract, with each mutation, validated both ways to the same
/// refusal. Returns the labels that derived.
fn every_source(attested: bool, full: bool) -> BTreeSet<String> {
    let base = base();
    let request = json!({"request_id":"derivative-identity-request","note":"\u{1f600}\u{e000}"});
    let request = if attested { None } else { Some(&request) };
    let form = if attested { "attested" } else { "dimension-absent" };
    let (mut sources, mut derived, mut refused, mut validations) = (0, 0, 0, 0);
    let mut ok = BTreeSet::new();
    let mut smallest: BTreeMap<String, (usize, String, Value, Value)> = BTreeMap::new();
    let files = corpus_files();
    for (index, path) in files.iter().enumerate() {
        progress(form, index, files.len());
        for (name, document) in corpus_documents(path) {
            for pointer in pointers(&document, is_source) {
                let source = document.pointer(&pointer).unwrap();
                if !full && !NAMED.contains(&format!("{name}#{pointer}").as_str()) {
                    continue;
                }
                let label = format!("{name}#{pointer} ({form})");
                sources += 1;
                let model = json!({"project":{"id":source["model_ref"]}});
                let Some(doc) = same_derivation(&label, &base, &model, source, &origin(source, attested), request) else {
                    refused += 1;
                    continue;
                };
                derived += 1;
                ok.insert(format!("{name}#{pointer}"));
                assert_eq!(same_validation(&label, &doc, source), Ok(()), "{label}");
                validations += 1;
                let rows = source["results"].as_array().unwrap().len();
                let id = source["producer"]["semantic_contract_id"].as_str().unwrap_or("").to_owned();
                if smallest.get(&id).is_none_or(|(n, ..)| rows < *n) {
                    smallest.insert(id, (rows, label, doc, source.clone()));
                }
            }
        }
    }
    for (_, label, doc, source) in smallest.values() {
        for (what, changed) in mutations(doc) {
            let verdict = same_validation(&format!("{label}: {what}"), &changed, source);
            assert!(verdict.is_err(), "{label}: {what} is refused");
            validations += 1;
        }
    }
    eprintln!("{form}: sources {sources}; derivations {derived}; refusals {refused}; validations {validations}; mutated identities {}", smallest.len());
    if full {
        assert!(sources >= MIN_SOURCES && derived >= MIN_DERIVED && refused >= MIN_REFUSED, "{sources} {derived} {refused}");
    } else {
        assert_eq!(sources, NAMED.len(), "every named source is found once");
    }
    assert!(smallest.len() >= MIN_IDENTITIES, "{}", smallest.len());
    ok
}

/// The named sources, in both origin forms (part of the crate's ordinary tests).
#[test]
fn named_sources_derive_and_validate_as_before() {
    for attested in [true, false] {
        let ok = every_source(attested, false);
        for name in NAMED {
            assert!(ok.contains(name), "{name}: a named source derives");
        }
    }
}

/// The whole corpus, attested. Long in a debug build (minutes), so outside the
/// ordinary run: `cargo test --release --lib derivative::reference_tests -- --ignored`.
#[test]
#[ignore = "whole-corpus derivation; run with --release -- --ignored"]
fn every_committed_source_derives_and_validates_as_before_attested() {
    let ok = every_source(true, true);
    for name in NAMED {
        assert!(ok.contains(name), "{name}: a named source derives");
    }
}

/// The whole corpus, dimension-absent with a request; as above.
#[test]
#[ignore = "whole-corpus derivation; run with --release -- --ignored"]
fn every_committed_source_derives_and_validates_as_before_dimension_absent() {
    every_source(false, true);
}

/// The committed derivative documents and their sources: each derivative paired
/// with every corpus source whose carrier digest its origin binds, plus the
/// report package's same-run substitute (its overlay over the straight request).
fn committed_pairs() -> Vec<(String, Value, Value)> {
    let mut by_digest: BTreeMap<String, (PathBuf, String, String)> = BTreeMap::new();
    let mut derivatives = Vec::new();
    let files = corpus_files();
    for (index, path) in files.iter().enumerate() {
        progress("committed pairs", index, files.len());
        for (name, document) in corpus_documents(path) {
            for pointer in pointers(&document, is_source) {
                if let Ok(d) = digest(document.pointer(&pointer).unwrap()) {
                    by_digest.entry(d).or_insert((path.clone(), name.clone(), pointer));
                }
            }
            for pointer in pointers(&document, is_derivative) {
                derivatives.push((path.clone(), name.clone(), pointer));
            }
        }
    }
    let load = |path: &Path, name: &str, pointer: &str| -> Value {
        let documents = corpus_documents(path);
        let (_, document) = documents.into_iter().find(|(n, _)| n == name).unwrap();
        document.pointer(pointer).unwrap().clone()
    };
    let mut out = Vec::new();
    for (path, name, pointer) in derivatives {
        let doc = load(&path, &name, &pointer);
        let carrier = doc["result_envelope"]["reproducibility"]["source_origin_bindings"][0]["received_carrier_checksum"]["value"].as_str().map(str::to_owned);
        let source = match carrier.and_then(|c| by_digest.get(&c).cloned()) {
            Some((spath, sname, spointer)) => load(&spath, &sname, &spointer),
            None => Value::Null,
        };
        out.push((format!("{name}#{pointer}"), doc, source));
    }
    let dir = "fixtures/reports/invented/exact_report_package_requests";
    let overlay: Value = serde_json::from_str(&std::fs::read_to_string(root().join(dir).join("pressure_2_straight_substitute.overlay.json")).unwrap()).unwrap();
    let straight: Value = serde_json::from_str(&std::fs::read_to_string(root().join(dir).join("pressure_2_straight.json")).unwrap()).unwrap();
    let mut request = straight["request"].clone();
    for r in overlay["replacements"].as_array().unwrap() {
        *request.pointer_mut(r["pointer"].as_str().unwrap()).unwrap() = r["value"].clone();
    }
    out.push((format!("{dir}/pressure_2_straight_substitute.overlay.json"), request["result_documents"][0].clone(), request["source_results"][0].clone()));
    out
}

/// (a) Every committed derivative document: its derivative hash computed both
/// ways (the former copy less the member through the canonical string; the
/// streamed writer skipping it in place) is the same, and the same as the
/// committed hash for every document that binds; with its source, it validates
/// both ways to the same verdict, mutated as above, and re-derives both ways
/// (the document as the base, its own origin) to the same document.
#[test]
fn every_committed_derivative_hashes_validates_and_rederives_as_before() {
    let pairs = committed_pairs();
    let (mut paired, mut binding, mut valid) = (0, 0, 0);
    // The smallest document of each source identity is also validated mutated.
    let mut smallest: BTreeMap<String, (usize, &str)> = BTreeMap::new();
    for (label, doc, source) in &pairs {
        let size = doc["result_envelope"]["row_accounting"].as_array().map_or(0, Vec::len);
        let id = source["producer"]["semantic_contract_id"].as_str().unwrap_or("").to_owned();
        if !source.is_null() && smallest.get(&id).is_none_or(|(n, _)| size < *n) {
            smallest.insert(id, (size, label));
        }
    }
    let mutated: BTreeSet<&str> = smallest.values().map(|(_, label)| *label).collect();
    for (index, (label, doc, source)) in pairs.iter().enumerate() {
        eprintln!("committed derivative {index} of {}: {label}", pairs.len());
        let envelope = doc["result_envelope"]["envelope_id"].as_str().unwrap();
        let mut payload = doc.clone();
        payload["result_envelope"]["reproducibility"].as_object_mut().unwrap().remove("derivative_hash");
        let scope = "derivative_document_excludes_own_hash";
        let former = checksum(&payload, scope, reference("derivative_document", envelope));
        let now = checksum_without(doc, &OWN_HASH, scope, reference("derivative_document", envelope));
        assert_eq!(former, now, "{label}: the derivative hash");
        if now.as_ref().is_ok_and(|h| *h == doc["result_envelope"]["reproducibility"]["derivative_hash"]) {
            binding += 1;
        }
        if source.is_null() {
            continue;
        }
        paired += 1;
        if same_validation(label, doc, source).is_ok() {
            valid += 1;
        }
        if mutated.contains(label.as_str()) {
            for (what, changed) in mutations(doc) {
                same_validation(&format!("{label}: {what}"), &changed, source).unwrap_err();
            }
        }
        let origin = doc["result_envelope"]["reproducibility"]["source_origin_bindings"][0].clone();
        let model = json!({"project":{"id":source["model_ref"]}});
        same_derivation(&format!("{label} (re-derived)"), doc, &model, source, &origin, None);
    }
    eprintln!("committed derivatives {}; paired with a source {paired}; hash binds {binding}; valid {valid}; mutated {}", pairs.len(), mutated.len());
    assert!(mutated.len() >= 3, "{}", mutated.len());
    assert!(paired >= MIN_COMMITTED_PAIRS && binding >= MIN_COMMITTED_PAIRS && valid >= MIN_COMMITTED_PAIRS, "{paired} {binding} {valid}");
}

/// (a, b) Every committed JSON document (each JSONL line), whole: the streamed
/// digest is the former one (the same refusal where the guard refuses), and
/// the streamed writer's text, whole and less the derivative hash member, is
/// `canonical_json`'s of the document and of its copy less that member.
#[test]
fn every_committed_document_streams_the_former_canonical_bytes() {
    let (mut documents, mut refused, mut omitted) = (0, 0, 0);
    let files = corpus_files();
    for (index, path) in files.iter().enumerate() {
        progress("documents", index, files.len());
        for (name, document) in corpus_documents(path) {
            documents += 1;
            let former = digest(&document);
            assert_eq!(former, super::digest(&document), "{name}");
            refused += usize::from(former.is_err());
            let mut text = String::new();
            write_canonical_without(&document, &[], &mut text);
            assert!(text == canonical_json(&document), "{name}: the streamed text");
            for pointer in pointers(&document, is_derivative) {
                let doc = document.pointer(&pointer).unwrap();
                let mut copy = doc.clone();
                copy["result_envelope"]["reproducibility"].as_object_mut().unwrap().remove("derivative_hash");
                let mut text = String::new();
                write_canonical_without(doc, &OWN_HASH, &mut text);
                assert!(text == canonical_json(&copy), "{name}#{pointer}: the streamed text less the hash member");
                assert_eq!(digest(&copy), digest_without(doc, &OWN_HASH), "{name}#{pointer}");
                omitted += 1;
            }
        }
    }
    eprintln!("documents {documents}; refused by the guard {refused}; derivatives less their hash {omitted}");
    assert!(documents >= MIN_DOCUMENTS && omitted >= MIN_COMMITTED_PAIRS, "{documents} {omitted}");
}

// ------------------------------------------------------------------ generated values

/// A small deterministic generator (xorshift64*), so that the property test needs
/// no dependency and every run sees the same values.
struct Gen(u64);

impl Gen {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
    fn pick<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        &items[self.below(items.len())]
    }
}

/// Key and string pieces whose UTF-16 order differs from their byte and code point
/// order (a supplementary-plane character against U+E000 to U+FFFF), escapes,
/// controls, the empty string and the path's own keys.
const PIECES: [&str; 22] = [
    "", "a", "b", "A", "10", "2", "é", "\u{e000}", "\u{ffff}", "\u{ff5a}", "\u{fffd}", "\u{10000}", "\u{1f600}",
    "\u{10ffff}", "\"", "\\", "/", "\n\t\r\u{8}\u{c}", "\u{0}\u{1f}\u{7f}", "\u{2028}\u{2029}", "derivative_hash", "reproducibility",
];

/// Numbers of every class the canonical profile admits or the guard refuses:
/// integers at the 2^53 edges, the integer extremes, negative zero, exponents of
/// both notations, subnormals and the largest finite values.
fn numbers() -> Vec<Value> {
    let mut out: Vec<Value> = vec![
        json!(0), json!(-1), json!(1), json!(9_007_199_254_740_991i64), json!(-9_007_199_254_740_991i64),
        json!(9_007_199_254_740_992i64), json!(-9_007_199_254_740_992i64), json!(9_007_199_254_740_993u64),
        json!(i64::MIN), json!(i64::MAX), json!(u64::MAX),
    ];
    for f in [
        0.0, -0.0, 0.1, -0.5, 1.5, 1e-6, 1e-7, 4.5e-9, 1e20, 1e21, 1.23e21, 9007199254740991.0, 9007199254740992.0,
        -9007199254740993.0, 2.0f64.powi(53) - 1.0, 333333333.3333332, 5e-324, -5e-324, 2.2250738585072014e-308,
        2.225073858507201e-308, 1.7976931348623157e308, -1.7976931348623157e308, 1e300, 0.30000000000000004,
    ] {
        out.push(json!(f));
    }
    out
}

fn string(g: &mut Gen) -> String {
    (0..g.below(4)).map(|_| *g.pick(&PIECES)).collect()
}

fn value(g: &mut Gen, depth: usize, numbers: &[Value]) -> Value {
    let kinds = if depth == 0 { 4 } else { 7 };
    match g.below(kinds) {
        0 => g.pick(&[Value::Null, json!(true), json!(false)]).clone(),
        1 => g.pick(numbers).clone(),
        2 => loop {
            // A random finite binary64.
            let f = f64::from_bits(g.next());
            if f.is_finite() {
                break json!(f);
            }
        },
        3 => json!(string(g)),
        4 => Value::Array((0..g.below(4)).map(|_| value(g, depth - 1, numbers)).collect()),
        _ => Value::Object((0..g.below(6)).map(|_| (string(g), value(g, depth - 1, numbers))).collect()),
    }
}

/// `v` as the former code saw it: a copy with the member at `omit` removed
/// (unchanged where the path does not reach an object member).
fn removed(v: &Value, omit: &[&str]) -> Value {
    let mut copy = v.clone();
    let mut at = &mut copy;
    for (i, key) in omit.iter().enumerate() {
        let Some(map) = at.as_object_mut() else { break };
        if i + 1 == omit.len() {
            map.remove(*key);
            break;
        }
        match map.get_mut(*key) {
            Some(next) => at = next,
            None => break,
        }
    }
    copy
}

/// (b) Over generated values (and the same values nested under the derivative
/// hash's path, with a hash member to skip), the streamed writer's text is
/// `canonical_json`'s of the copy less the member, and the streamed digest is the
/// former digest of that copy, refusals included.
#[test]
fn generated_values_stream_the_former_canonical_bytes_and_refusals() {
    let numbers = numbers();
    let mut g = Gen(0x9e37_79b9_7f4a_7c15);
    let paths: [&[&str]; 5] = [&[], &OWN_HASH, &["derivative_hash"], &["a", "\u{1f600}"], &["", "\u{e000}", "derivative_hash"]];
    let (mut cases, mut refusals) = (0, 0);
    for round in 0..4000 {
        let inner = value(&mut g, 4, &numbers);
        let v = if round % 2 == 0 {
            inner
        } else {
            let mut wrapped = json!({"result_envelope":{"reproducibility":{"derivative_hash":value(&mut g, 2, &numbers),"z":inner.clone()},"\u{1f600}":inner.clone()},"\u{e000}":1});
            if round % 4 == 1 {
                wrapped["result_envelope"]["reproducibility"]["\u{10000}"] = inner;
            }
            wrapped
        };
        for omit in paths {
            let copy = removed(&v, omit);
            let mut text = String::new();
            write_canonical_without(&v, omit, &mut text);
            assert_eq!(text, canonical_json(&copy), "{v} less {omit:?}");
            let former = digest(&copy);
            assert_eq!(former, digest_without(&v, omit), "{v} less {omit:?}");
            refusals += usize::from(former.is_err());
            cases += 1;
        }
    }
    eprintln!("generated cases {cases}; refused by the guard {refusals}");
    assert!(refusals > 1000 && cases - refusals > 1000, "both outcomes are exercised: {cases} {refusals}");
}

/// (b) Hand-picked: member order by UTF-16 code units, not bytes or code points.
#[test]
fn members_sort_by_utf16_code_units() {
    let v = json!({"\u{ff5a}":1,"\u{1f600}":2,"\u{e000}":3,"b":4,"a":{"\u{10000}":5,"\u{ffff}":6}});
    let mut text = String::new();
    write_canonical_without(&v, &[], &mut text);
    assert_eq!(text, "{\"a\":{\"\u{10000}\":5,\"\u{ffff}\":6},\"b\":4,\"\u{1f600}\":2,\"\u{e000}\":3,\"\u{ff5a}\":1}");
    assert_eq!(text, canonical_json(&v));
}
