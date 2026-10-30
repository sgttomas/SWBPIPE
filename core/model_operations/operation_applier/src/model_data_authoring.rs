//! Authoring of the model-data slots (W-13 design `model-document-0.7.0.md`
//! §3, §6): lines, unresolved assumptions, constraints, equipment interfaces,
//! rule-pack references, and the `tags`, `notes` and `provenance_record`
//! annotations on existing records.
//!
//! Every operation here is identity-preserving: records are created, updated
//! and deleted one at a time (no whole-array replacement), and an update never
//! changes the record's id (FR-MOD-003). Clearing an annotation removes the
//! member; the `TBD` sentinel is never written to these slots. Record and
//! annotation *shapes* are judged by the one document validator
//! (`document_validator`), which the common applier runs on every candidate
//! result; this module checks only what the validator cannot see (the
//! operation's target, identity, staleness and payload JSON type).
//!
//! Nothing here evaluates a constraint or an equipment interface (OD-1): the
//! records are data slots only. Rule content never enters the document; a
//! rule-pack reference carries the pack's identity and stamped checksum.

use crate::geometry_operations::{GeometryEdit, GeometryError};
use crate::rich_authoring::{RichEdit, RichError};
use crate::{canonical_json, is_valid_schema_id, DiffPreviewRow};
use serde_json::{Map, Value};
use std::collections::BTreeSet;

/// The top-level model-data collections. Reference and attachment scans do
/// not read them: naming an entity here never blocks that entity's edit or
/// deletion (§3.3, §6); a missing subject is the validator's warning.
pub(crate) const MODEL_DATA_COLLECTIONS: [&str; 5] = [
    "lines",
    "unresolved_assumptions",
    "constraints",
    "equipment_interfaces",
    "rule_pack_refs",
];

/// Free-text and provenance annotations: never entity references.
pub(crate) const ANNOTATION_MEMBERS: [&str; 3] = ["tags", "notes", "provenance_record"];

/// The object types that take `tags` and `notes` (§3.1).
const ANNOTATED_TYPES: [&str; 13] = [
    "Project",
    "Node",
    "Element",
    "Section",
    "Material",
    "Support",
    "Component",
    "Load",
    "Combination",
    "Line",
    "Assumption",
    "Constraint",
    "EquipmentInterface",
];

/// The object types that take `provenance_record` (§3.5).
const PROVENANCE_RECORD_TYPES: [&str; 4] = ["Material", "Section", "Component", "Support"];

/// One model-data record family.
struct Family {
    object_type: &'static str,
    collection: &'static str,
    id_key: &'static str,
    create: &'static str,
    update: Option<&'static str>,
    delete: &'static str,
    /// Whether the id is one the product authors (checked against the
    /// applier's id pattern and every other id in the document). A rule-pack
    /// id is the pack's own identity and is only required to be non-blank.
    authored_id: bool,
}

const FAMILIES: [Family; 5] = [
    Family {
        object_type: "Line",
        collection: "lines",
        id_key: "id",
        create: "create_line",
        update: Some("update_line"),
        delete: "delete_line",
        authored_id: true,
    },
    Family {
        object_type: "Assumption",
        collection: "unresolved_assumptions",
        id_key: "assumption_id",
        create: "create_assumption",
        update: Some("update_assumption"),
        delete: "delete_assumption",
        authored_id: true,
    },
    Family {
        object_type: "Constraint",
        collection: "constraints",
        id_key: "constraint_id",
        create: "create_constraint",
        update: Some("update_constraint"),
        delete: "delete_constraint",
        authored_id: true,
    },
    Family {
        object_type: "EquipmentInterface",
        collection: "equipment_interfaces",
        id_key: "id",
        create: "create_equipment_interface",
        update: Some("update_equipment_interface"),
        delete: "delete_equipment_interface",
        authored_id: true,
    },
    Family {
        object_type: "RulePackRef",
        collection: "rule_pack_refs",
        id_key: "rule_pack_id",
        create: "add_rule_pack_ref",
        update: None,
        delete: "remove_rule_pack_ref",
        authored_id: false,
    },
];

#[derive(Clone, Copy, PartialEq)]
enum Action {
    Create,
    Update,
    Delete,
}

fn family_for_change(change_kind: &str) -> Option<(&'static Family, Action)> {
    FAMILIES.iter().find_map(|family| {
        if family.create == change_kind {
            Some((family, Action::Create))
        } else if family.update == Some(change_kind) {
            Some((family, Action::Update))
        } else if family.delete == change_kind {
            Some((family, Action::Delete))
        } else {
            None
        }
    })
}

fn family_for_type(object_type: &str) -> Option<&'static Family> {
    FAMILIES.iter().find(|family| family.object_type == object_type)
}

/// The operation kind a model-data change kind belongs to, or `None` when the
/// change kind is not one of this module's.
pub(crate) fn operation_kind_for(change_kind: &str) -> Option<&'static str> {
    family_for_change(change_kind).map(|(_, action)| match action {
        Action::Create => "create",
        Action::Update => "modify",
        Action::Delete => "delete",
    })
}

/// The collection a model-data object type addresses.
pub(crate) fn collection_for(object_type: &str) -> Option<&'static str> {
    family_for_type(object_type).map(|family| family.collection)
}

/// True for an annotation edit: (any annotated type, `tags` | `notes`) or
/// (Material | Section | Component | Support, `provenance_record`).
pub(crate) fn is_annotation(object_type: &str, field_path: &str) -> bool {
    match field_path {
        "tags" | "notes" => ANNOTATED_TYPES.contains(&object_type),
        "provenance_record" => PROVENANCE_RECORD_TYPES.contains(&object_type),
        _ => false,
    }
}

/// An annotation travels on its family's existing change kind (§6).
pub(crate) fn annotation_change_kind(object_type: &str) -> &'static str {
    match object_type {
        "Load" => "update_load",
        "Support" => "update_support",
        _ => "set_field",
    }
}

fn id_matches(record: &Value, key: &str, id: &str) -> bool {
    record.get(key).and_then(Value::as_str) == Some(id)
}

/// The record an annotation or rich edit targets: the project singleton, a
/// model-data record by its id key, or an entity of an existing collection.
pub(crate) fn target<'a>(model: &'a Value, object_type: &str, target_ref: &str) -> Option<&'a Value> {
    if object_type == "Project" {
        return model.get("project").filter(|project| id_matches(project, "id", target_ref));
    }
    let (collection, key) = match family_for_type(object_type) {
        Some(family) => (family.collection, family.id_key),
        None => (crate::collection_for(object_type)?, "id"),
    };
    model.get(collection)?.as_array()?.iter().find(|record| id_matches(record, key, target_ref))
}

/// [`target`], mutably.
pub(crate) fn target_mut<'a>(model: &'a mut Value, object_type: &str, target_ref: &str) -> Option<&'a mut Value> {
    if object_type == "Project" {
        return model.get_mut("project").filter(|project| id_matches(project, "id", target_ref));
    }
    let (collection, key) = match family_for_type(object_type) {
        Some(family) => (family.collection, family.id_key),
        None => (crate::collection_for(object_type)?, "id"),
    };
    model.get_mut(collection)?.as_array_mut()?.iter_mut().find(|record| id_matches(record, key, target_ref))
}

fn rich_err(code: &'static str, message: impl Into<String>) -> RichError {
    RichError { code, message: message.into() }
}

/// An annotation edit, resolved for the common rich-authoring write path.
///
/// - `before` is the canonical JSON of the current member, or `not_present`.
/// - `after` is JSON text: an array of strings for `tags`, a string for
///   `notes`, an object for `provenance_record`; only `null` clears (removes)
///   the member. An empty tag list is stored as authored, so every inverse
///   edit restores the document exactly.
///
/// Tag form (trimmed, unique, at most 64 characters), non-blank notes and the
/// provenance record's members and enums are the document validator's
/// findings on the candidate result.
#[allow(clippy::too_many_arguments)]
pub(crate) fn resolve_annotation(
    model: &Value,
    object_type: &str,
    target_ref: &str,
    field_path: &str,
    _before: &str,
    after: &str,
    unit: &str,
    dimension: &str,
) -> Result<Option<RichEdit>, RichError> {
    if !is_annotation(object_type, field_path) {
        return Ok(None);
    }
    if unit != "none" || dimension != "dimensionless" {
        return Err(rich_err("OP-ANNOTATION-UNIT-INVALID", format!("`{field_path}` edits carry unit none and dimension dimensionless.")));
    }
    let entity = target(model, object_type, target_ref).ok_or_else(|| {
        rich_err("OP-TARGET-NOT-FOUND", format!("{object_type} `{target_ref}` was not found in the current model."))
    })?;
    let current = entity.get(field_path);
    let current_display = current.map(canonical_json).unwrap_or_else(|| "not_present".into());
    let parsed: Value = serde_json::from_str(after).map_err(|_| {
        rich_err("OP-ANNOTATION-PAYLOAD-INVALID", format!("The new `{field_path}` must be JSON text (`null` clears it)."))
    })?;
    let next = match (field_path, parsed) {
        (_, Value::Null) => None,
        ("tags", Value::Array(items)) => {
            if !items.iter().all(Value::is_string) {
                return Err(rich_err("OP-ANNOTATION-PAYLOAD-INVALID", "Tags are an array of strings."));
            }
            Some(Value::Array(items))
        }
        ("notes", Value::String(text)) => Some(Value::String(text)),
        ("provenance_record", Value::Object(record)) => Some(Value::Object(record)),
        (_, other) => {
            let expected = match field_path {
                "tags" => "an array of strings",
                "notes" => "a string",
                _ => "an object",
            };
            return Err(rich_err(
                "OP-ANNOTATION-PAYLOAD-INVALID",
                format!("The new `{field_path}` must be {expected} or null to clear it, not {}.", json_kind(&other)),
            ));
        }
    };
    if next.as_ref() == current || (next.is_none() && current.is_none()) {
        return Err(rich_err("OP-ANNOTATION-UNCHANGED", format!("The new `{field_path}` equals the current one; nothing to apply.")));
    }
    Ok(Some(RichEdit {
        writes: vec![(vec![field_path.to_string()], next)],
        current_display,
        warnings: Vec::new(),
    }))
}

fn json_kind(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "a boolean",
        Value::Number(_) => "a number",
        Value::String(_) => "a string",
        Value::Array(_) => "an array",
        Value::Object(_) => "an object",
    }
}

fn data_err(code: &'static str, message: impl Into<String>, refs: Vec<String>) -> GeometryError {
    GeometryError { code, message: message.into(), affected_refs: refs }
}

/// True for a change kind this module resolves as a record operation.
pub(crate) fn is_record_change_kind(change_kind: &str) -> bool {
    family_for_change(change_kind).is_some()
}

/// Every id in the document an authored id must not repeat: entity `id`s at
/// any depth plus the assumption and constraint ids.
fn document_ids(model: &Value) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    crate::geometry_operations::global_ids(model, &mut ids);
    for (collection, key) in [("unresolved_assumptions", "assumption_id"), ("constraints", "constraint_id")] {
        for record in model.get(collection).and_then(Value::as_array).into_iter().flatten() {
            if let Some(id) = record.get(key).and_then(Value::as_str) {
                ids.insert(id.to_string());
            }
        }
    }
    ids
}

fn text_at<'a>(intent: &'a Value, pointer: &str) -> &'a str {
    intent.pointer(pointer).and_then(Value::as_str).unwrap_or("")
}

/// Resolve one record operation: create, update or delete a line,
/// assumption, constraint or equipment interface, or add or remove a
/// rule-pack reference.
///
/// Wire: `target.object_type` is the family's type and `target.ref` the
/// record's id; `field_path` is the collection; unit `none`, dimension
/// `dimensionless`. Create: `before` `not_present`, `after` the record's
/// JSON; `field_path` `<collection>` appends, and `<collection>[<index>]`
/// inserts at that index (0 ..= length), which is how a delete's inverse
/// puts the record back where it was. Update: `before` the current record's
/// canonical JSON, `after` the new record's JSON with the same id; the record
/// keeps its index. Delete: `before` the current record's canonical JSON,
/// `after` `not_present`. A collection left empty by a delete is removed
/// (absence means none), so every operation has an exact inverse.
pub(crate) fn resolve_record(model: &Value, intent: &Value) -> Result<GeometryEdit, GeometryError> {
    let change_kind = text_at(intent, "/change/change_kind");
    let Some((family, action)) = family_for_change(change_kind) else {
        return Err(data_err("OP-KIND-UNSUPPORTED", format!("`{change_kind}` is not a model-data record operation."), vec![]));
    };
    let object_type = text_at(intent, "/target/object_type");
    let target_ref = text_at(intent, "/target/ref");
    let field_path = text_at(intent, "/change/field_path");
    let before = text_at(intent, "/change/before");
    let after = text_at(intent, "/change/after");
    let refs = vec![target_ref.to_string()];
    // `<collection>[<index>]` places a created record (a delete's inverse).
    let insert_at = field_path
        .strip_prefix(family.collection)
        .and_then(|rest| rest.strip_prefix('['))
        .and_then(|rest| rest.strip_suffix(']'))
        .filter(|digits| !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit()))
        .and_then(|digits| digits.parse::<usize>().ok());
    let path_ok = field_path == family.collection || (action == Action::Create && insert_at.is_some());
    if object_type != family.object_type
        || !path_ok
        || text_at(intent, "/change/unit") != "none"
        || text_at(intent, "/change/dimension") != "dimensionless"
    {
        return Err(data_err(
            "OP-DATA-SHAPE-INVALID",
            format!(
                "{change_kind} requires object type {}, field_path {} (or {}[<index>] to place a created record), unit none and dimension dimensionless.",
                family.object_type, family.collection, family.collection
            ),
            refs,
        ));
    }
    if target_ref.trim().is_empty() {
        return Err(data_err("OP-DATA-SHAPE-INVALID", format!("{change_kind} requires the record's id as target.ref."), refs));
    }
    if model.get("document_kind").and_then(Value::as_str) != Some("openpipestress.product_preview.model")
        || model.get("schema_version").and_then(Value::as_str) != Some(open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION)
    {
        return Err(data_err(
            "OP-DATA-DOCUMENT-INVALID",
            format!(
                "Model-data records are authored on the current product_preview.model document {}.",
                open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION
            ),
            refs,
        ));
    }
    let records: &[Value] = match model.get(family.collection) {
        None => &[],
        Some(Value::Array(items)) => items,
        Some(_) => {
            return Err(data_err("OP-DATA-COLLECTION-INVALID", format!("`{}` is not an array; repair the document first.", family.collection), refs));
        }
    };
    let matching: Vec<usize> = records
        .iter()
        .enumerate()
        .filter(|(_, record)| id_matches(record, family.id_key, target_ref))
        .map(|(index, _)| index)
        .collect();

    let parse_record = |label: &str| -> Result<Map<String, Value>, GeometryError> {
        match serde_json::from_str::<Value>(after) {
            Ok(Value::Object(record)) => Ok(record),
            _ => Err(data_err("OP-DATA-PAYLOAD-INVALID", format!("The {label} record must be a JSON object."), vec![target_ref.to_string()])),
        }
    };

    let mut candidate = model.clone();
    let root = candidate.as_object_mut().ok_or_else(|| data_err("OP-DATA-DOCUMENT-INVALID", "The model must be an object.", vec![]))?;
    match action {
        Action::Create => {
            if !matching.is_empty() {
                return Err(data_err("OP-DATA-ID-COLLISION", format!("{} `{target_ref}` already exists.", family.object_type), refs));
            }
            if before != "not_present" {
                return Err(data_err("OP-STALE-BEFORE-VALUE", format!("A new {} has before-value `not_present`, not `{before}`.", family.object_type), refs));
            }
            let record = parse_record("new")?;
            if record.get(family.id_key).and_then(Value::as_str) != Some(target_ref) {
                return Err(data_err("OP-DATA-ID-MISMATCH", format!("The new record's `{}` must equal target.ref `{target_ref}`.", family.id_key), refs));
            }
            if family.authored_id {
                if !is_valid_schema_id(target_ref) {
                    return Err(data_err("OP-DATA-ID-INVALID", format!("`{target_ref}` does not match the id pattern ^[A-Za-z][A-Za-z0-9_.:-]*$."), refs));
                }
                if document_ids(model).contains(target_ref) {
                    return Err(data_err("OP-DATA-ID-COLLISION", format!("`{target_ref}` is already an id in this document; ids are unique."), refs));
                }
            }
            let list = root
                .entry(family.collection)
                .or_insert_with(|| Value::Array(Vec::new()))
                .as_array_mut()
                .expect("checked array");
            match insert_at {
                Some(index) if index > list.len() => {
                    return Err(data_err(
                        "OP-DATA-INDEX-INVALID",
                        format!("{} has {} records; a created record is placed at 0 to {}.", family.collection, list.len(), list.len()),
                        refs,
                    ));
                }
                Some(index) => list.insert(index, Value::Object(record)),
                None => list.push(Value::Object(record)),
            }
        }
        Action::Update | Action::Delete => {
            let [index] = matching[..] else {
                return Err(data_err(
                    "OP-TARGET-NOT-FOUND",
                    format!("{} edits require exactly one `{target_ref}` in {}.", family.object_type, family.collection),
                    refs,
                ));
            };
            let current = canonical_json(&records[index]);
            if before != current {
                return Err(data_err(
                    "OP-STALE-BEFORE-VALUE",
                    format!("Intent before-value no longer matches the current {} `{target_ref}`.", family.object_type),
                    refs,
                ));
            }
            let list = root.get_mut(family.collection).and_then(Value::as_array_mut).expect("checked array");
            if action == Action::Update {
                let record = parse_record("updated")?;
                if record.get(family.id_key).and_then(Value::as_str) != Some(target_ref) {
                    return Err(data_err(
                        "OP-DATA-ID-CHANGE-REFUSED",
                        format!("An update never changes `{}`; the record keeps `{target_ref}` (FR-MOD-003).", family.id_key),
                        refs,
                    ));
                }
                let record = Value::Object(record);
                if record == records[index] {
                    return Err(data_err("OP-DATA-UNCHANGED", format!("The updated {} equals the current one; nothing to apply.", family.object_type), refs));
                }
                list[index] = record;
            } else {
                if after != "not_present" {
                    return Err(data_err("OP-DELETE-AFTER-VALUE-INVALID", "Deletions use after-value `not_present`.", refs));
                }
                list.remove(index);
                if list.is_empty() {
                    root.remove(family.collection);
                }
            }
        }
    }
    Ok(GeometryEdit {
        model: candidate,
        diff_preview: vec![DiffPreviewRow {
            entity_ref: target_ref.to_string(),
            object_type: family.object_type.to_string(),
            field_path: field_path.to_string(),
            before: before.to_string(),
            after: after.to_string(),
            unit: "none".into(),
            dimension: "dimensionless".into(),
            change_kind: change_kind.to_string(),
        }],
    })
}

fn line_row(before: &Value, after: &Value) -> DiffPreviewRow {
    DiffPreviewRow {
        entity_ref: before.get("id").and_then(Value::as_str).unwrap_or("").to_string(),
        object_type: "Line".into(),
        field_path: "lines".into(),
        before: canonical_json(before),
        after: canonical_json(after),
        unit: "none".into(),
        dimension: "dimensionless".into(),
        change_kind: "update_line".into(),
    }
}

fn names_pipe(line: &Value, pipe_id: &str) -> bool {
    line.get("pipe_refs")
        .and_then(Value::as_array)
        .is_some_and(|refs| refs.iter().any(|item| item.as_str() == Some(pipe_id)))
}

/// Deleting a pipe removes it from its line in the same operation (§3.2);
/// a line may be left with no pipes. Returns the diff rows of the lines it
/// changes, computed against `model` (the document before deletion).
pub(crate) fn remove_pipe_from_lines(model: &mut Value, pipe_id: &str) -> Vec<DiffPreviewRow> {
    let mut rows = Vec::new();
    for line in model.get_mut("lines").and_then(Value::as_array_mut).into_iter().flatten() {
        if !names_pipe(line, pipe_id) {
            continue;
        }
        let before = line.clone();
        if let Some(refs) = line.get_mut("pipe_refs").and_then(Value::as_array_mut) {
            refs.retain(|item| item.as_str() != Some(pipe_id));
        }
        rows.push(line_row(&before, line));
    }
    rows
}

/// A pipe created from `source_pipe` (split, bend arc) joins the source's
/// line, immediately after the source (§3.2). A copied pipe joins no line and
/// never calls this. Returns the diff row of the line it changes.
pub(crate) fn join_source_line(model: &mut Value, source_pipe: &str, new_pipe: &str) -> Option<DiffPreviewRow> {
    let line = model
        .get_mut("lines")
        .and_then(Value::as_array_mut)?
        .iter_mut()
        .find(|line| names_pipe(line, source_pipe))?;
    let before = line.clone();
    let refs = line.get_mut("pipe_refs").and_then(Value::as_array_mut)?;
    let at = refs.iter().position(|item| item.as_str() == Some(source_pipe))?;
    refs.insert(at + 1, Value::String(new_pipe.to_string()));
    Some(line_row(&before, line))
}
