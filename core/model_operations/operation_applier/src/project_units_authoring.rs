//! Editing the project units after creation (DEL-02-02 Design
//! `project-unit-selection.md` §2.5, §2.7, §2.8): `set_project_unit` on one key
//! of `project.units`.
//!
//! `project.units` is the one record of the project's units: a map from
//! dimension key to catalog symbol. The edit changes one string in it and no
//! other byte of the document. No stored value is converted; positions stay in
//! the length unit, which is locked while nodes exist.
//!
//! Wire (operation kind `modify`, the `Project` singleton target as
//! `set_model_setting` uses it):
//! - `target`: object type `Project`, `ref` the document's `project.id`.
//! - `field_path`: `project.units.<key>`, for one of the seven keys.
//! - `before`: the stored symbol, or `not_present` when the key is absent.
//! - `after`: the canonical catalog symbol; there is no remove form.
//! - `unit`: the `after` symbol; `dimension`: the key's dimension.
//!
//! Refusals run in the Design's §2.7 order, so a stale `before` is reported
//! before the length lock:
//! `OP-PROJECT-UNIT-SHAPE-INVALID`, `OP-TARGET-NOT-FOUND`,
//! `OP-PROJECT-UNIT-DOCUMENT-INVALID`, `OP-PROJECT-UNIT-UNKNOWN-KEY`,
//! `OP-PROJECT-UNIT-REMOVAL-REFUSED`, `OP-PROJECT-UNIT-NOT-CANONICAL`,
//! `OP-UNIT-DIMENSION-MISMATCH`, `OP-STALE-BEFORE-VALUE`,
//! `OP-PROJECT-UNIT-UNCHANGED`, `OP-PROJECT-UNIT-LENGTH-LOCKED`.
//!
//! The canonical check is the desktop's (`projectUnitSelection.ts`
//! `canonicalCatalogEntry`), not `unit_by_symbol`, which resolves the legacy
//! alias `C` and falls back across compatible dimensions. A stored
//! legacy `C` is still read and may be replaced; it cannot be chosen.
//!
//! The desktop undoes through its session checkpoint, which restores the prior
//! document exactly; the inverse of a change between two symbols is the same
//! operation with `before` and `after` exchanged.

use crate::geometry_operations::{GeometryEdit, GeometryError};
use crate::{canonical_json, DiffPreviewRow};
use open_pipe_stress_units::{catalog_definitions, ReviewStatus};
use serde_json::{Map, Value};

/// Sets one key of `project.units` to a canonical catalog symbol.
pub(crate) const SET: &str = "set_project_unit";

/// The seven keys a project carries (Design §2.2). Each key's catalog
/// dimension has the key's name.
pub(crate) const KEYS: [&str; 7] = ["length", "force", "moment", "pressure", "stress", "temperature", "angle"];

const FIELD_PREFIX: &str = "project.units.";

/// True for the change kind this module resolves.
pub(crate) fn is_change_kind(change_kind: &str) -> bool {
    change_kind == SET
}

fn refusal(code: &'static str, message: impl Into<String>, refs: &[String]) -> GeometryError {
    GeometryError { code, message: message.into(), affected_refs: refs.to_vec() }
}

/// Whether a catalog entry of dimension `entry_dimension` serves `key`, as the
/// desktop's `unitEntryMatchesDimension` matches it: the same dimension, or for
/// `stress` a pressure entry (the catalog lists Pa and psi as pressure).
fn serves_key(key: &str, entry_dimension: &str) -> bool {
    entry_dimension == key || (key == "stress" && entry_dimension == "pressure")
}

/// The member at `pointer`: `Ok(None)` when absent, `Ok(Some(text))` when a
/// string, `Err(())` when present and not a string.
fn string_member<'a>(intent: &'a Value, pointer: &str) -> Result<Option<&'a str>, ()> {
    match intent.pointer(pointer) {
        None => Ok(None),
        Some(Value::String(text)) => Ok(Some(text)),
        Some(_) => Err(()),
    }
}

/// Resolve one `set_project_unit` to its candidate document and diff row.
pub(crate) fn resolve(model: &Value, intent: &Value) -> Result<GeometryEdit, GeometryError> {
    let target_ref = intent.pointer("/target/ref").and_then(Value::as_str).unwrap_or("");
    let refs = [target_ref.to_string()];

    // 1. Shape: the Project target and every member a string. `after` may be
    // absent or null; that is a removal, refused by name below.
    let shape = |message: String| refusal("OP-PROJECT-UNIT-SHAPE-INVALID", message, &refs);
    let object_type = string_member(intent, "/target/object_type").ok().flatten().unwrap_or("");
    if object_type != "Project" {
        return Err(shape(format!("{SET} targets the Project singleton, not `{object_type}`.")));
    }
    let mut members = Vec::new();
    for (name, pointer) in [
        ("target.ref", "/target/ref"),
        ("change.field_path", "/change/field_path"),
        ("change.before", "/change/before"),
        ("change.unit", "/change/unit"),
        ("change.dimension", "/change/dimension"),
    ] {
        match string_member(intent, pointer) {
            Ok(Some(text)) => members.push(text),
            _ => return Err(shape(format!("{SET} requires `{name}` as a string."))),
        }
    }
    let [_, field_path, before, unit, dimension] = members[..] else { unreachable!("five members") };
    let after = match intent.pointer("/change/after") {
        None | Some(Value::Null) => None,
        Some(Value::String(text)) => Some(text.as_str()),
        Some(_) => return Err(shape(format!("{SET} requires `change.after` as a string."))),
    };
    let removal = matches!(after, None | Some("not_present"));
    if let Some(after) = after.filter(|_| !removal) {
        if unit != after {
            return Err(shape(format!("{SET} carries the chosen symbol as its unit; `change.unit` `{unit}` is not `{after}`.")));
        }
    }

    // 2. The singleton target.
    if model.pointer("/project/id").and_then(Value::as_str) != Some(target_ref) {
        return Err(refusal(
            "OP-TARGET-NOT-FOUND",
            format!("Project `{target_ref}` is not this document's project; project units target the document's project id."),
            &refs,
        ));
    }

    // 3. The current model document, with `project.units` an object when present.
    let version = open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION;
    if model.get("document_kind").and_then(Value::as_str) != Some("openpipestress.product_preview.model")
        || model.get("schema_version").and_then(Value::as_str) != Some(version)
    {
        return Err(refusal(
            "OP-PROJECT-UNIT-DOCUMENT-INVALID",
            format!("Project units are edited on the current product_preview.model document {version}."),
            &refs,
        ));
    }
    let units = match model.pointer("/project/units") {
        None => None,
        Some(Value::Object(units)) => Some(units),
        Some(_) => {
            return Err(refusal(
                "OP-PROJECT-UNIT-DOCUMENT-INVALID",
                "`project.units` is not an object; repair the document first.",
                &refs,
            ));
        }
    };

    // 4. One of the seven keys.
    let Some(key) = field_path.strip_prefix(FIELD_PREFIX).filter(|key| KEYS.contains(key)) else {
        return Err(refusal(
            "OP-PROJECT-UNIT-UNKNOWN-KEY",
            format!("`{field_path}` is not a project unit; the keys are project.units.{}.", KEYS.join(", project.units.")),
            &refs,
        ));
    };

    // 5. No remove form: the applier's preconditions need every key.
    let Some(after) = after.filter(|_| !removal) else {
        return Err(refusal(
            "OP-PROJECT-UNIT-REMOVAL-REFUSED",
            format!("`{field_path}` cannot be removed; choose a {key} unit instead."),
            &refs,
        ));
    };

    // 6-7. An exact accepted catalog symbol, of the key's dimension.
    let entries: Vec<_> = catalog_definitions()
        .filter(|entry| entry.review_status == ReviewStatus::Accepted && entry.symbol == after)
        .collect();
    if entries.is_empty() {
        let hint = if after == "C" { " C is a legacy spelling; choose the catalog symbol degC." } else { "" };
        return Err(refusal(
            "OP-PROJECT-UNIT-NOT-CANONICAL",
            format!("`{after}` is not a catalog unit symbol.{hint}"),
            &refs,
        ));
    }
    if !entries.iter().any(|entry| serves_key(key, entry.dimension.as_str())) {
        let other = entries.iter().map(|entry| entry.dimension.as_str()).collect::<Vec<_>>().join(", ");
        return Err(refusal(
            "OP-UNIT-DIMENSION-MISMATCH",
            format!("`{after}` is a {other} unit; `{field_path}` takes a {key} unit."),
            &refs,
        ));
    }
    if dimension != key {
        return Err(refusal(
            "OP-UNIT-DIMENSION-MISMATCH",
            format!("`{field_path}` carries dimension `{key}`; intent dimension `{dimension}` is not accepted."),
            &refs,
        ));
    }

    // 8. The `before` the change was prepared from.
    let current = units.and_then(|units| units.get(key));
    let current_text = match current {
        None => "not_present".to_string(),
        Some(Value::String(text)) => text.clone(),
        Some(other) => canonical_json(other),
    };
    if before != current_text {
        return Err(refusal(
            "OP-STALE-BEFORE-VALUE",
            format!("Intent before-value `{before}` no longer matches the current `{field_path}` (`{current_text}`)."),
            &refs,
        ));
    }

    // 9. Something to change.
    if current.and_then(Value::as_str) == Some(after) {
        return Err(refusal(
            "OP-PROJECT-UNIT-UNCHANGED",
            format!("`{field_path}` is already {after}; nothing to apply."),
            &refs,
        ));
    }

    // 10. Positions are bare numbers in the length unit: it is fixed once a node exists.
    if key == "length" && model.get("nodes").and_then(Value::as_array).is_some_and(|nodes| !nodes.is_empty()) {
        return Err(refusal(
            "OP-PROJECT-UNIT-LENGTH-LOCKED",
            "The length unit is fixed once nodes exist: node positions are stored in it, and changing it would rescale every position. Enter positions in any length unit instead; they are converted on entry.",
            &refs,
        ));
    }

    let mut candidate = model.clone();
    let project = candidate
        .get_mut("project")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| refusal("OP-PROJECT-UNIT-DOCUMENT-INVALID", "`project` must be an object.", &refs))?;
    project
        .entry("units")
        .or_insert_with(|| Value::Object(Map::new()))
        .as_object_mut()
        .expect("checked object")
        .insert(key.to_string(), Value::String(after.to_string()));
    Ok(GeometryEdit {
        model: candidate,
        diff_preview: vec![DiffPreviewRow {
            entity_ref: target_ref.to_string(),
            object_type: "Project".into(),
            field_path: field_path.to_string(),
            before: current_text,
            after: after.to_string(),
            unit: after.to_string(),
            dimension: key.to_string(),
            change_kind: SET.to_string(),
        }],
    })
}
