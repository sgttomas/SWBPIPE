//! Read-only boundary metadata and the private authority for composite creation.
//! Ordinary support records carry the metadata; it adds no solver mechanics.
use crate::{canonical_json, sha256_hex};
use open_pipe_stress_units::{canonical_unit, convert_for_dimension, unit_by_symbol, Dimension};
use serde_json::{Map, Value};
use std::collections::{BTreeMap, HashSet};

const DOFS: [&str; 6] = ["UX", "UY", "UZ", "RX", "RY", "RZ"];

#[derive(Debug)]
pub(crate) struct BoundaryError {
    pub(crate) code: &'static str,
    pub(crate) message: String,
}

type Result<T> = std::result::Result<T, BoundaryError>;
fn error(message: impl Into<String>) -> BoundaryError {
    BoundaryError {
        code: "OP-BOUNDARY-INVALID",
        message: message.into(),
    }
}
fn object<'a>(value: &'a Value, allowed: &[&str]) -> Result<&'a Map<String, Value>> {
    let record = value
        .as_object()
        .ok_or_else(|| error("Expected an object"))?;
    if let Some(key) = record.keys().find(|k| !allowed.contains(&k.as_str())) {
        return Err(error(format!("Unknown boundary field {key}")));
    }
    Ok(record)
}
fn text<'a>(value: Option<&'a Value>, name: &str) -> Result<&'a str> {
    value
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| error(format!("{name} must be a nonempty string")))
}
fn collection<'a>(model: &'a Value, key: &str) -> Result<&'a Vec<Value>> {
    model
        .get(key)
        .and_then(Value::as_array)
        .ok_or_else(|| error(format!("Missing {key} collection")))
}
fn group_id(support: &Value) -> Option<&str> {
    support
        .get("boundary_association")?
        .get("boundary_id")?
        .as_str()
}

/// Opaque capability minted only after validating the original base and whole
/// submitted batch. No constructor, mutable fields, serialization or caller flag
/// can supply it. The owning batch runner retains it only for its private replay.
#[derive(Debug)]
pub(crate) struct ValidatedBoundaryBatch {
    // Frozen provenance: current_model is deliberately allowed to evolve under
    // ordinary validated earlier operations, so its hash need not equal base.
    _base_hash: String,
    _batch_hash: String,
    groups: BTreeMap<String, Vec<Value>>,
}

pub(crate) fn validate_association(association: &Value) -> Result<()> {
    let record = object(
        association,
        &[
            "boundary_id",
            "kind",
            "equipment_reference",
            "nozzle_reference",
            "coordinate_system",
        ],
    )?;
    text(record.get("boundary_id"), "boundary_id")?;
    text(record.get("equipment_reference"), "equipment_reference")?;
    match text(record.get("kind"), "kind")? {
        "equipment" if record.contains_key("nozzle_reference") => {
            return Err(error("equipment must omit nozzle_reference"))
        }
        "equipment" => (),
        "equipment_nozzle" => {
            text(record.get("nozzle_reference"), "nozzle_reference")?;
        }
        _ => return Err(error("kind must be equipment or equipment_nozzle")),
    }
    if record.get("coordinate_system").and_then(Value::as_str) != Some("global") {
        return Err(error(
            "Boundary coordinate_system must be explicitly global",
        ));
    }
    Ok(())
}

/// Return canonical member ordinal and occupied DOFs after exact wire checks.
fn validate_member(payload: &Value) -> Result<(usize, Vec<usize>)> {
    let record = object(
        payload,
        &[
            "id",
            "label",
            "node",
            "provenance",
            "family",
            "restraints",
            "stiffness",
            "boundary_association",
        ],
    )?;
    let association = record
        .get("boundary_association")
        .ok_or_else(|| error("Missing boundary association"))?;
    validate_association(association)?;
    let boundary_id = text(association.get("boundary_id"), "boundary_id")?;
    let id = text(record.get("id"), "id")?;
    for key in ["label", "node", "provenance"] {
        text(record.get(key), key)?;
    }
    let restraints = record
        .get("restraints")
        .and_then(Value::as_array)
        .filter(|a| !a.is_empty())
        .ok_or_else(|| error("A boundary member requires explicit nonempty restraints"))?;
    let mut indices = Vec::new();
    for restraint in restraints {
        let dof = restraint
            .as_str()
            .ok_or_else(|| error("Restraints must be DOF strings"))?;
        let index = DOFS
            .iter()
            .position(|d| *d == dof)
            .ok_or_else(|| error("Unsupported boundary DOF"))?;
        if indices.last().is_some_and(|last| *last >= index) {
            return Err(error(
                "Boundary restraints must be unique in UX/UY/UZ/RX/RY/RZ order",
            ));
        }
        indices.push(index);
    }
    match record.get("family").and_then(Value::as_str) {
        Some("anchor") => {
            if record.contains_key("stiffness") || id != format!("{boundary_id}:rigid") {
                return Err(error(
                    "Rigid member requires exact :rigid ID and no stiffness",
                ));
            }
            Ok((0, indices))
        }
        Some("spring") => {
            if indices.len() != 1 {
                return Err(error("A boundary spring requires exactly one DOF"));
            }
            let index = indices[0];
            let dof = DOFS[index];
            if id != format!("{boundary_id}:spring:{dof}") {
                return Err(error("Boundary spring ID must match its group and DOF"));
            }
            let stiffness = object(
                record
                    .get("stiffness")
                    .ok_or_else(|| error("Missing stiffness"))?,
                &["dof", "value"],
            )?;
            if stiffness.get("dof").and_then(Value::as_str) != Some(dof) {
                return Err(error("Stiffness DOF must match restraint"));
            }
            let quantity = object(
                stiffness
                    .get("value")
                    .ok_or_else(|| error("Missing stiffness quantity"))?,
                &["value", "unit"],
            )?;
            let value = quantity
                .get("value")
                .and_then(Value::as_f64)
                .filter(|v| v.is_finite() && *v > 0.0)
                .ok_or_else(|| error("Stiffness must be finite and positive"))?;
            let dim = if index < 3 {
                Dimension::LinearStiffness
            } else {
                Dimension::RotationalStiffness
            };
            let symbol = text(quantity.get("unit"), "stiffness unit")?;
            let from = unit_by_symbol(symbol, dim).map_err(|e| error(e.to_string()))?;
            let to =
                canonical_unit(dim).ok_or_else(|| error("Missing canonical stiffness unit"))?;
            let converted =
                convert_for_dimension(value, dim, from, to).map_err(|e| error(e.to_string()))?;
            if !converted.is_finite() || converted <= 0.0 {
                return Err(error("Converted stiffness must be finite and positive"));
            }
            Ok((index + 1, indices))
        }
        _ => Err(error("Boundary member family must be anchor or spring")),
    }
}

fn validate_references(model: &Value, payload: &Value) -> Result<()> {
    let id = text(payload.get("id"), "id")?;
    if collection(model, "supports")?
        .iter()
        .any(|s| s.get("id").and_then(Value::as_str) == Some(id))
    {
        return Err(error(format!("Support ID {id} already exists")));
    }
    let node = text(payload.get("node"), "node")?;
    if collection(model, "nodes")?
        .iter()
        .filter(|n| n.get("id").and_then(Value::as_str) == Some(node))
        .count()
        != 1
    {
        return Err(error(format!("Boundary node {node} must resolve uniquely")));
    }
    Ok(())
}

pub(crate) fn preflight_batch(base_model: &Value, batch: &Value) -> Result<ValidatedBoundaryBatch> {
    let operations = batch
        .get("operations")
        .and_then(Value::as_array)
        .ok_or_else(|| error("Batch requires an operations array"))?;
    let mut groups: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    let mut ids = HashSet::new();
    for intent in operations {
        if intent
            .pointer("/change/change_kind")
            .and_then(Value::as_str)
            != Some("create_support")
        {
            continue;
        }
        let payload: Value = serde_json::from_str(text(
            intent.pointer("/change/after"),
            "create_support after",
        )?)
        .map_err(|_| error("Support payload must be JSON"))?;
        if payload.get("boundary_association").is_none() {
            continue;
        }
        validate_member(&payload)?;
        // Composite members are the exact accepted dimensionless ordinary intent.
        if intent.get("operation_kind").and_then(Value::as_str) != Some("create")
            || intent
                .pointer("/target/object_type")
                .and_then(Value::as_str)
                != Some("Support")
            || intent.pointer("/target/ref") != payload.get("id")
            || intent.pointer("/change/field_path").and_then(Value::as_str) != Some("supports")
            || intent.pointer("/change/before").and_then(Value::as_str) != Some("not_present")
            || intent.pointer("/change/unit").and_then(Value::as_str) != Some("none")
            || intent.pointer("/change/dimension").and_then(Value::as_str) != Some("dimensionless")
        {
            return Err(error("Boundary member intent must use exact create Support/supports/not_present/none/dimensionless shape"));
        }
        validate_references(base_model, &payload)?;
        let boundary_id = group_id(&payload)
            .expect("validated association")
            .to_string();
        if collection(base_model, "supports")?
            .iter()
            .any(|s| group_id(s) == Some(boundary_id.as_str()))
        {
            return Err(error(format!(
                "Boundary group {boundary_id} exists in the original batch base"
            )));
        }
        if !ids.insert(text(payload.get("id"), "id")?.to_string()) {
            return Err(error("Duplicate boundary member ID"));
        }
        groups.entry(boundary_id).or_default().push(payload);
    }
    for members in groups.values() {
        let mut previous = None;
        let mut occupied = HashSet::new();
        for member in members {
            if member["boundary_association"] != members[0]["boundary_association"]
                || member["node"] != members[0]["node"]
            {
                return Err(error(
                    "Every boundary member must have identical association and node",
                ));
            }
            let (ordinal, dofs) = validate_member(member)?;
            if previous.is_some_and(|p| p >= ordinal) {
                return Err(error(
                    "Boundary members must be rigid first, then springs UX/UY/UZ/RX/RY/RZ",
                ));
            }
            previous = Some(ordinal);
            if dofs.into_iter().any(|dof| !occupied.insert(dof)) {
                return Err(error("Boundary DOF cannot be both rigid and spring"));
            }
        }
    }
    Ok(ValidatedBoundaryBatch {
        _base_hash: sha256_hex(&canonical_json(base_model)),
        _batch_hash: sha256_hex(&canonical_json(batch)),
        groups,
    })
}

pub(crate) fn validate_create(
    current_model: &Value,
    support_payload: &Value,
    context: Option<&ValidatedBoundaryBatch>,
) -> Result<()> {
    if support_payload.get("boundary_association").is_none() {
        return Ok(());
    }
    validate_member(support_payload)?;
    validate_references(current_model, support_payload)?;
    let id = group_id(support_payload).expect("validated association");
    let existing: Vec<&Value> = collection(current_model, "supports")?
        .iter()
        .filter(|s| group_id(s) == Some(id))
        .collect();
    match context {
        None if existing.is_empty() => Ok(()),
        None => Err(error(
            "Single-operation creation cannot append to an existing boundary group",
        )),
        Some(token) => {
            let members = token
                .groups
                .get(id)
                .ok_or_else(|| error("Boundary group is absent from the validated batch"))?;
            let canonical = canonical_json(support_payload);
            let index = members
                .iter()
                .position(|m| canonical_json(m) == canonical)
                .ok_or_else(|| {
                    error("Support payload does not match any exact validated boundary member")
                })?;
            if existing.len() != index
                || existing
                    .iter()
                    .zip(&members[..index])
                    .any(|(actual, expected)| canonical_json(actual) != canonical_json(expected))
            {
                return Err(error(
                    "Current boundary group is not the exact earlier validated member prefix",
                ));
            }
            Ok(())
        }
    }
}

/// Shared public/private run guard; metadata changes outside configuration.before
/// still invalidate queued mechanics edits through the full model-hash claim.
pub(crate) fn requires_model_hash(model: &Value, intent: &Value) -> bool {
    if intent
        .pointer("/target/object_type")
        .and_then(Value::as_str)
        == Some("Support")
    {
        if let Some(target) = intent.pointer("/target/ref").and_then(Value::as_str) {
            if model
                .get("supports")
                .and_then(Value::as_array)
                .is_some_and(|supports| {
                    supports.iter().any(|support| {
                        support.get("id").and_then(Value::as_str) == Some(target)
                            && support.get("boundary_association").is_some()
                    })
                })
            {
                return true;
            }
        }
    }
    intent
        .pointer("/change/change_kind")
        .and_then(Value::as_str)
        == Some("create_support")
        && intent
            .pointer("/change/after")
            .and_then(Value::as_str)
            .and_then(|s| serde_json::from_str::<Value>(s).ok())
            .is_some_and(|payload| payload.get("boundary_association").is_some())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn base() -> Value {
        json!({"nodes":[{"id":"node:a"},{"id":"node:b"}],"supports":[]})
    }
    fn association() -> Value {
        json!({"boundary_id":"boundary:a","kind":"equipment_nozzle","equipment_reference":" Drawing E-42 rev 2 ","nozzle_reference":" N1 ","coordinate_system":"global"})
    }
    fn rigid() -> Value {
        json!({"id":"boundary:a:rigid","label":" Explicit rigid UX,RZ ","node":"node:a","provenance":" Source, unchanged ","family":"anchor","restraints":["UX","RZ"],"boundary_association":association()})
    }
    fn spring(dof: &str) -> Value {
        json!({"id":format!("boundary:a:spring:{dof}"),"label":format!("Spring {dof}"),"node":"node:a","provenance":"source","family":"spring","restraints":[dof],"stiffness":{"dof":dof,"value":{"value":12.5,"unit":if dof.starts_with('U') {"N/m"} else {"N*m/rad"}}},"boundary_association":association()})
    }
    fn intent(payload: &Value) -> Value {
        json!({"operation_kind":"create","operation_status":"proposed","author_type":"user","target":{"object_type":"Support","ref":payload["id"]},"change":{"change_kind":"create_support","field_path":"supports","before":"not_present","after":payload.to_string(),"unit":"none","dimension":"dimensionless"}})
    }
    fn batch(payloads: &[Value]) -> Value {
        json!({"batch_id":"batch:test","operations":payloads.iter().map(intent).collect::<Vec<_>>()})
    }
    fn add(model: &mut Value, payload: &Value) {
        model["supports"]
            .as_array_mut()
            .unwrap()
            .push(payload.clone());
    }

    #[test]
    fn association_exact_shapes_and_source_preservation() {
        let original = association();
        validate_association(&original).unwrap();
        assert_eq!(original["equipment_reference"], " Drawing E-42 rev 2 ");
        assert_eq!(original["nozzle_reference"], " N1 ");
        let mut equipment = original.clone();
        equipment["kind"] = json!("equipment");
        assert!(validate_association(&equipment).is_err());
        equipment
            .as_object_mut()
            .unwrap()
            .remove("nozzle_reference");
        validate_association(&equipment).unwrap();
        for bad in [Value::Null, json!([]), json!("global"), json!({})] {
            assert!(validate_association(&bad).is_err());
        }
        for key in [
            "boundary_id",
            "equipment_reference",
            "nozzle_reference",
            "kind",
            "coordinate_system",
        ] {
            for invalid in [Value::Null, json!(12), json!([]), json!(""), json!(" \t ")] {
                let mut bad = original.clone();
                bad[key] = invalid;
                assert!(validate_association(&bad).is_err(), "{key}: {bad}");
            }
            let mut missing = original.clone();
            missing.as_object_mut().unwrap().remove(key);
            assert!(validate_association(&missing).is_err());
        }
        for (key, value) in [
            ("kind", "nozzle"),
            ("coordinate_system", "local"),
            ("coordinate_system", " GLOBAL "),
            ("unknown", "bad"),
        ] {
            let mut bad = original.clone();
            bad[key] = json!(value);
            assert!(validate_association(&bad).is_err());
        }
    }

    #[test]
    fn single_new_passes_append_and_payload_bypass_reject() {
        let mut model = base();
        let first = rigid();
        validate_create(&model, &first, None).unwrap();
        let mut legacy = first.clone();
        legacy
            .as_object_mut()
            .unwrap()
            .remove("boundary_association");
        validate_create(&model, &legacy, None).unwrap();
        add(&mut model, &first);
        assert!(validate_create(&model, &spring("UY"), None).is_err());
        let mut forged = spring("UY");
        forged["validated_batch_context"] = json!(true);
        assert!(validate_create(&base(), &forged, None).is_err());
        assert!(validate_create(&model, &forged, None).is_err());
        let mut unknown = first.clone();
        unknown["boundary_association"]["allow_existing_group"] = json!(true);
        assert!(validate_create(&base(), &unknown, None).is_err());
        let mut null = first.clone();
        null["boundary_association"] = Value::Null;
        assert!(validate_create(&base(), &null, None).is_err());
    }

    #[test]
    fn exact_token_replay_accepts_second_member_and_preserves_inputs() {
        let base_model = base();
        let payloads = vec![rigid(), spring("UY"), spring("RX")];
        let submitted = batch(&payloads);
        let snapshot = submitted.clone();
        let token = preflight_batch(&base_model, &submitted).unwrap();
        assert_eq!(token._base_hash, sha256_hex(&canonical_json(&base_model)));
        assert_eq!(token._batch_hash, sha256_hex(&canonical_json(&submitted)));
        let mut current = base_model.clone();
        for payload in &payloads {
            validate_create(&current, payload, Some(&token)).unwrap();
            add(&mut current, payload);
        }
        assert_eq!(current["supports"][0], payloads[0]);
        assert_eq!(submitted, snapshot);
        assert_eq!(base_model, base());
        assert!(validate_create(&current, &payloads[2], Some(&token)).is_err());
    }

    #[test]
    fn original_base_collisions_reject_even_after_delete_intent() {
        let mut model = base();
        add(&mut model, &rigid());
        let mut submitted = batch(&[spring("UY")]);
        submitted["operations"].as_array_mut().unwrap().insert(0, json!({"change":{"change_kind":"delete_entity","after":"deleted"},"target":{"object_type":"Support","ref":"boundary:a:rigid"}}));
        assert!(preflight_batch(&model, &submitted).is_err());
        let mut collision = rigid();
        collision
            .as_object_mut()
            .unwrap()
            .remove("boundary_association");
        let mut model = base();
        add(&mut model, &collision);
        assert!(preflight_batch(&model, &batch(&[rigid()])).is_err());
        assert!(validate_create(&model, &rigid(), None).is_err());
    }

    #[test]
    fn batch_rejects_duplicate_metadata_node_order_and_member_identity() {
        let first = rigid();
        let second = spring("UY");
        for payloads in [
            vec![first.clone(), first.clone()],
            vec![second.clone(), first.clone()],
            vec![spring("RZ"), spring("UY")],
            vec![first.clone(), spring("UX")],
        ] {
            assert!(preflight_batch(&base(), &batch(&payloads)).is_err());
        }
        for (key, value) in [
            ("node", json!("node:b")),
            ("id", json!("boundary:a:spring:UZ")),
            (
                "boundary_association",
                json!({"boundary_id":"boundary:a","kind":"equipment","equipment_reference":"other","coordinate_system":"global"}),
            ),
        ] {
            let mut altered = second.clone();
            altered[key] = value;
            assert!(preflight_batch(&base(), &batch(&[first.clone(), altered])).is_err());
        }
        let mut bad = second.clone();
        bad["node"] = json!("missing");
        assert!(preflight_batch(&base(), &batch(&[bad])).is_err());
        let mut ambiguous = base();
        ambiguous["nodes"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id":"node:a"}));
        assert!(validate_create(&ambiguous, &first, None).is_err());
    }

    #[test]
    fn members_require_canonical_mechanics_and_positive_dimensioned_quantities() {
        for (key, value) in [
            ("family", json!("guide")),
            ("restraints", json!([])),
            ("restraints", json!(["RZ", "UX"])),
            ("restraints", json!(["UX", "UX"])),
            ("restraints", json!(["TX"])),
            ("restraints", json!([1])),
            ("stiffness", Value::Null),
            ("properties", json!({})),
            ("hanger", json!({})),
            ("nonlinear", json!({})),
        ] {
            let mut bad = rigid();
            bad[key] = value;
            assert!(validate_create(&base(), &bad, None).is_err(), "{bad}");
        }
        for dof in DOFS {
            let valid = spring(dof);
            validate_create(&base(), &valid, None).unwrap();
            for invalid in [json!(0), json!(-1), json!("12"), Value::Null, json!(1e308)] {
                let mut bad = valid.clone();
                bad["stiffness"]["value"]["value"] = invalid;
                if bad["stiffness"]["value"]["value"] == json!(1e308) {
                    bad["stiffness"]["value"]["unit"] = json!(if dof.starts_with('U') {
                        "kN/m"
                    } else {
                        "kN*m/rad"
                    });
                }
                assert!(validate_create(&base(), &bad, None).is_err(), "{bad}");
            }
            for unit in [
                "Pa",
                "",
                if dof.starts_with('U') {
                    "N*m/rad"
                } else {
                    "N/m"
                },
            ] {
                let mut bad = valid.clone();
                bad["stiffness"]["value"]["unit"] = json!(unit);
                assert!(validate_create(&base(), &bad, None).is_err());
            }
        }
        for (path, value) in [
            ("/stiffness/dof", json!("UZ")),
            ("/restraints", json!(["UY", "UZ"])),
            (
                "/stiffness/value",
                json!({"value":3,"unit":"N/m","dimension":"linear_stiffness"}),
            ),
            (
                "/stiffness",
                json!({"dof":"UY","value":{"value":3,"unit":"N/m"},"mode":"nonlinear"}),
            ),
        ] {
            let mut bad = spring("UY");
            *bad.pointer_mut(path).unwrap() = value;
            assert!(validate_create(&base(), &bad, None).is_err());
        }
    }

    #[test]
    fn token_rejects_payload_drift_wrong_context_missing_and_injected_prefix() {
        let first = rigid();
        let second = spring("UY");
        let token = preflight_batch(&base(), &batch(&[first.clone(), second.clone()])).unwrap();
        assert!(validate_create(&base(), &second, Some(&token)).is_err());
        let empty = preflight_batch(&base(), &batch(&[])).unwrap();
        assert!(validate_create(&base(), &first, Some(&empty)).is_err());
        let mut model = base();
        add(&mut model, &first);
        for key in ["label", "provenance"] {
            let mut altered = second.clone();
            altered[key] = json!("changed");
            assert!(validate_create(&model, &altered, Some(&token)).is_err());
            let mut drifted_model = model.clone();
            drifted_model["supports"][0][key] = json!("changed");
            assert!(validate_create(&drifted_model, &second, Some(&token)).is_err());
        }
        let mut drifted_model = model.clone();
        drifted_model["supports"][0]["boundary_association"]["equipment_reference"] =
            json!("changed");
        assert!(validate_create(&drifted_model, &second, Some(&token)).is_err());
        let mut injected = model.clone();
        add(&mut injected, &spring("RX"));
        assert!(validate_create(&injected, &second, Some(&token)).is_err());
        let mut duplicate = model.clone();
        add(&mut duplicate, &first);
        assert!(validate_create(&duplicate, &second, Some(&token)).is_err());
        assert!(validate_create(&model, &spring("UZ"), Some(&token)).is_err());
    }

    #[test]
    fn batch_member_intent_requires_exact_target_and_shape() {
        let submitted = batch(&[rigid()]);
        for (path, value) in [
            ("/operation_kind", "modify"),
            ("/target/ref", "wrong"),
            ("/target/object_type", "Element"),
            ("/change/field_path", "configuration"),
            ("/change/before", "{}"),
            ("/change/unit", "N/m"),
            ("/change/dimension", "linear_stiffness"),
        ] {
            let mut bad = submitted.clone();
            *bad["operations"][0].pointer_mut(path).unwrap() = json!(value);
            assert!(preflight_batch(&base(), &bad).is_err(), "{path}");
        }
    }

    #[test]
    fn hash_guard_covers_creation_configuration_scalar_delete_and_null_association() {
        assert!(requires_model_hash(&base(), &intent(&rigid())));
        let mut legacy = rigid();
        legacy
            .as_object_mut()
            .unwrap()
            .remove("boundary_association");
        assert!(!requires_model_hash(&base(), &intent(&legacy)));
        let mut model = base();
        add(&mut model, &rigid());
        add(&mut model, &json!({"id":"legacy"}));
        for (kind, field) in [
            ("update_support", "configuration"),
            ("set_field", "label"),
            ("update_support", "restraints"),
            ("delete_entity", "supports"),
        ] {
            let operation = json!({"target":{"object_type":"Support","ref":"boundary:a:rigid"},"change":{"change_kind":kind,"field_path":field}});
            assert!(requires_model_hash(&model, &operation));
            let mut unrelated = operation.clone();
            unrelated["target"]["ref"] = json!("legacy");
            assert!(!requires_model_hash(&model, &unrelated));
            unrelated["target"]["ref"] = json!("absent");
            assert!(!requires_model_hash(&model, &unrelated));
            let mut malformed = model.clone();
            malformed["supports"][0]["boundary_association"] = Value::Null;
            assert!(requires_model_hash(&malformed, &operation));
        }
        assert!(!requires_model_hash(&model, &json!({})));
    }

    #[test]
    fn independent_groups_and_legacy_supports_remain_valid() {
        let mut other = spring("UY");
        other["boundary_association"]["boundary_id"] = json!("boundary:b");
        other["id"] = json!("boundary:b:spring:UY");
        let mut legacy = rigid();
        legacy["id"] = json!("legacy");
        legacy
            .as_object_mut()
            .unwrap()
            .remove("boundary_association");
        let mut model = base();
        add(&mut model, &legacy);
        let token =
            preflight_batch(&model, &batch(&[rigid(), other.clone(), spring("RX")])).unwrap();
        validate_create(&model, &rigid(), Some(&token)).unwrap();
        add(&mut model, &rigid());
        validate_create(&model, &other, Some(&token)).unwrap();
        add(&mut model, &other);
        validate_create(&model, &spring("RX"), Some(&token)).unwrap();
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use serde_json::json;

    fn model() -> Value {
        json!({"schema_version":"0.1.0","document_kind":"openpipestress.product_preview.model","project":{"id":"p","name":"Invented boundary test"},"nodes":[{"id":"node:a"}],"supports":[],"loads":[],"load_cases":[],"pipe_segments":[],"components":[]})
    }
    fn payload(dof: Option<&str>) -> Value {
        let mut value = json!({"id":"boundary:a:rigid","label":"Rigid UX","node":"node:a","provenance":"invented source","family":"anchor","restraints":["UX"],"boundary_association":{"boundary_id":"boundary:a","kind":"equipment_nozzle","equipment_reference":" Drawing E-42 ","nozzle_reference":" N1 ","coordinate_system":"global"}});
        if let Some(dof) = dof {
            value["id"] = json!(format!("boundary:a:spring:{dof}"));
            value["family"] = json!("spring");
            value["restraints"] = json!([dof]);
            value["stiffness"] = json!({"dof":dof,"value":{"value":42,"unit":if dof.starts_with('U') {"N/m"} else {"N*m/rad"}}});
        }
        value
    }
    fn operation(p: &Value) -> Value {
        json!({"operation_id":format!("op:{}",p["id"].as_str().unwrap()),"operation_kind":"create","operation_status":"proposed","author_type":"user","target":{"object_type":"Support","ref":p["id"]},"change":{"change_id":format!("change:{}",p["id"].as_str().unwrap()),"change_kind":"create_support","field_label":"Support","field_path":"supports","before":"not_present","after":p.to_string(),"unit":"none","dimension":"dimensionless","source_note":"Invented boundary test"},"validation":{"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run","diff_preview_status":"not_generated","application_status":"not_applied"},"audit_boundary":{"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,"requires_user_acceptance":true,"mutates_accepted_model_state":false},"professional_boundary":{"human_review_required":true,"software_makes_compliance_claim":false,"software_makes_certification_claim":false,"software_makes_sealing_claim":false,"software_makes_approval_claim":false,"software_makes_authentication_claim":false},"rationale":"Invented global equipment/nozzle boundary member"})
    }
    fn claim(model: &Value) -> Value {
        json!({"algorithm":"sha256","canonicalization":crate::BACKEND_CANONICALIZATION,"payload_scope":"model_payload","value":format!("sha256:{}",sha256_hex(&canonical_json(model)))})
    }
    fn assert_claim_guard(model: &Value, op: &Value) {
        for missing in [None, Some(Value::Null)] {
            for out in [
                crate::validate_operation(model, op, missing.as_ref()),
                crate::apply_operation(model, op, missing.as_ref()),
            ] {
                assert!(out.applied_model.is_none());
                assert!(
                    out.diagnostics
                        .iter()
                        .any(|d| d.code == "OP-BOUNDARY-MODEL-HASH-REQUIRED"),
                    "{:?}",
                    out.diagnostics
                );
            }
        }
        for bad in [
            json!({}),
            json!("forged"),
            json!({"value":"sha256:bad"}),
            claim(&json!({"other":"model"})),
        ] {
            for out in [
                crate::validate_operation(model, op, Some(&bad)),
                crate::apply_operation(model, op, Some(&bad)),
            ] {
                assert!(out.applied_model.is_none());
                assert!(
                    out.diagnostics
                        .iter()
                        .any(|d| d.code.starts_with("OP-CLAIMED-MODEL-HASH")),
                    "{:?}",
                    out.diagnostics
                );
            }
        }
    }
    fn config(support: &Value) -> Value {
        Value::Object(
            [
                "family",
                "restraints",
                "stiffness",
                "hanger",
                "nonlinear",
                "provenance",
            ]
            .iter()
            .filter_map(|key| support.get(key).map(|v| ((*key).into(), v.clone())))
            .collect(),
        )
    }

    #[test]
    fn actual_single_hash_guard_preservation_and_append_rejection() {
        let base = model();
        let mut raw = payload(None);
        raw["label"] = json!(" Preserve label whitespace ");
        raw["provenance"] = json!(" Preserve provenance whitespace ");
        let create = operation(&raw);
        assert_claim_guard(&base, &create);
        let preview = crate::validate_operation(&base, &create, Some(&claim(&base)));
        assert!(preview.applied_model.is_none());
        assert!(
            preview.diagnostics.iter().all(|d| d.severity != "blocking"),
            "{:?}",
            preview.diagnostics
        );
        let applied = crate::apply_operation(&base, &create, Some(&claim(&base)));
        assert_eq!(
            applied
                .applied_model
                .as_ref()
                .map(|m| m["supports"][0].clone()),
            Some(raw),
            "{:?}",
            applied.diagnostics
        );
        let current = applied.applied_model.unwrap();
        let second = operation(&payload(Some("UY")));
        let blocked = crate::apply_operation(&current, &second, Some(&claim(&current)));
        assert!(blocked.applied_model.is_none());
        assert!(blocked
            .diagnostics
            .iter()
            .any(|d| d.code == "OP-BOUNDARY-INVALID"));
        let mut legacy = payload(None);
        legacy
            .as_object_mut()
            .unwrap()
            .remove("boundary_association");
        assert!(crate::apply_operation(&base, &operation(&legacy), None)
            .applied_model
            .is_some());
    }

    #[test]
    fn actual_associated_configuration_scalar_delete_require_hash_and_preserve_metadata() {
        let mut current = model();
        let support = payload(None);
        current["supports"] = json!([support]);
        let mut config_op = operation(&support);
        config_op["operation_kind"] = json!("modify");
        config_op["change"]["change_kind"] = json!("update_support");
        config_op["change"]["field_path"] = json!("configuration");
        config_op["change"]["before"] = json!(canonical_json(&config(&support)));
        let mut changed = config(&support);
        changed["restraints"] = json!(["UX", "UY"]);
        config_op["change"]["after"] = json!(changed.to_string());
        let mut scalar = config_op.clone();
        scalar["change"]["change_kind"] = json!("set_field");
        scalar["change"]["field_path"] = json!("label");
        scalar["change"]["before"] = support["label"].clone();
        scalar["change"]["after"] = json!("Renamed");
        let mut deletion = scalar.clone();
        deletion["operation_kind"] = json!("delete");
        deletion["change"]["change_kind"] = json!("delete_support");
        deletion["change"]["field_path"] = json!("supports");
        deletion["change"]["after"] = json!("not_present");
        for op in [&config_op, &scalar, &deletion] {
            assert_claim_guard(&current, op);
            let out = crate::apply_operation(&current, op, Some(&claim(&current)));
            assert!(out.applied_model.is_some(), "{:?}", out.diagnostics);
            let applied = out.applied_model.unwrap();
            if op == &deletion {
                assert_eq!(applied["supports"], json!([]));
            } else {
                assert_eq!(
                    applied["supports"][0]["boundary_association"],
                    support["boundary_association"]
                );
            }
        }
        let old_claim = claim(&current);
        current["supports"][0]["boundary_association"]["equipment_reference"] =
            json!("changed reference");
        let stale = crate::apply_operation(&current, &config_op, Some(&old_claim));
        assert!(stale.applied_model.is_none());
        assert!(stale
            .diagnostics
            .iter()
            .any(|d| d.code.starts_with("OP-CLAIMED-MODEL-HASH")));
        assert!(
            crate::apply_operation(&current, &config_op, Some(&claim(&current)))
                .applied_model
                .is_some()
        );
        let mut illicit = changed.clone();
        illicit["boundary_association"] = support["boundary_association"].clone();
        config_op["change"]["after"] = json!(illicit.to_string());
        assert!(
            crate::apply_operation(&current, &config_op, Some(&claim(&current)))
                .applied_model
                .is_none()
        );
    }

    #[test]
    fn actual_batch_replay_agent_parity_atomic_failure_and_roundtrip() {
        let base = model();
        let members = [payload(None), payload(Some("UY")), payload(Some("RX"))];
        let batch = json!({"batch_id":"batch:boundary","operations":members.iter().map(operation).collect::<Vec<_>>()});
        let preview = crate::validate_operation_batch(&base, &batch, Some(&claim(&base)));
        assert!(preview["applied_model"].is_null());
        let applied = crate::apply_operation_batch(&base, &batch, Some(&claim(&base)));
        assert_eq!(
            applied["applied_model"]["supports"],
            json!(members),
            "{applied}"
        );
        assert_eq!(
            serde_json::from_str::<Value>(&applied["applied_model"].to_string()).unwrap(),
            applied["applied_model"]
        );
        let mut agent = batch.clone();
        for op in agent["operations"].as_array_mut().unwrap() {
            op["author_type"] = json!("agent");
            op["source"] = json!({"source_ref":"invented offline proposal","source_role":"agent","source_channel":"imported"});
        }
        let agent_applied = crate::apply_operation_batch(&base, &agent, Some(&claim(&base)));
        assert_eq!(
            agent_applied["applied_model"], applied["applied_model"],
            "{agent_applied}"
        );
        assert_eq!(
            agent_applied["applied_model_backend_hash"],
            applied["applied_model_backend_hash"]
        );
        let mut failed = batch.clone();
        let mut bad = members[2].clone();
        bad["node"] = json!("node:missing");
        failed["operations"][2]["change"]["after"] = json!(bad.to_string());
        let rejected = crate::apply_operation_batch(&base, &failed, Some(&claim(&base)));
        assert!(rejected["applied_model"].is_null());
        assert_eq!(base, model());
    }
}
