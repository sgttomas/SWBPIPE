//! Authoring of operating-condition sets, their pipe assignment and a load
//! case's condition use (model document 0.9.0, slice S3a; DEL-02-01 Work
//! `model-document-0.9.0.md` §11, with §1.2-§1.4 and §10).
//!
//! Every operation is one record or one member, identity-preserving
//! (FR-MOD-003). The shape, quantities, range, provenance and references of a
//! set, a reference and a use are the one document validator's findings on the
//! candidate (§1.3), as for every applier edit; this module checks only what the
//! validator cannot see: the target, identity, staleness, the payload's JSON
//! type, the reference guard on delete and the route admission of a use.
//!
//! Wire (object type `ConditionSet`, collection `condition_sets`, key `id`; the
//! model-data families' wire and codes, `model_data_authoring.rs`):
//! - `create_condition_set` (create): `before` `not_present`, `after` the whole
//!   record as JSON object text, its `id` equal to `target.ref`. `field_path`
//!   `condition_sets` appends; `condition_sets[<index>]` inserts at that index,
//!   which is how a delete's inverse puts the record back where it was.
//! - `update_condition_set` (modify): `before` the current record's canonical
//!   JSON, `after` the whole replacement record; the `id` is unchanged.
//! - `delete_condition_set` (delete): `before` the current record's canonical
//!   JSON, `after` `not_present`. Refused with `OP-CONDITION-SET-IN-USE` while a
//!   pipe's `condition_set_ref` names the set, naming every such pipe. Deleting
//!   the last set leaves `condition_sets` an empty array; only a create writes
//!   the collection when it is absent.
//! - `assign_condition_set` (modify, target `Element`, `field_path`
//!   `condition_set_ref`): `before` `{"condition_set_ref": id}` as canonical
//!   JSON, or `not_present`; `after` `{"condition_set_ref": id}`, or
//!   `not_present` to clear.
//! - `set_load_case_condition_use` and `clear_load_case_condition_use` (modify,
//!   target `Load`, `field_path` `condition_use`): `before` the current member's
//!   canonical JSON or `not_present`; `after` the whole `ConditionUse` object
//!   text (set) or `not_present` (clear).
//!
//! Every operation carries unit `none` and dimension `dimensionless`. The same
//! value as stored, and a clear of an absent member, is `OP-DATA-UNCHANGED`.
//! The product creates no set and writes no value the person did not enter.
//!
//! **Route admission of a use** ([`admit_case_use`], also applied to
//! `create_load_case`'s `condition_use`): on a pressure-free document
//! `temperature`, `added_weight: true` and `wind_exposure: true` are admitted;
//! `contents_density` is refused (`LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED`)
//! and `pressure` is refused
//! (`LOAD_CASE_CONDITION_PRESSURE_WITHOUT_PRESSURE_CONTRACT`). On a document with
//! a `pressure_contract` every use is refused
//! (`LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED`, until S4).
//!
//! **The solve's judgement, classified (SWBPIPE Agent 0, 2026-11-02;
//! [`report_judgement`]).** After an admitted operation, and once the document
//! validator admits the candidate, the applier runs product_physics' public
//! read-only `operating_conditions::preview_operating_conditions` on the
//! candidate and classifies every finding it gives by the one explicit list
//! [`STAGED_FINDINGS`]: a listed finding is a warning with the solve's code and
//! message (authoring is not blocked, so a case's use can be stated before every
//! pipe is assigned, OC-8); every other finding, a parse or unknown-field
//! refusal of the candidate, and any code added later until the list admits it,
//! blocks. A blocking finding the document already carried before the
//! operation, with the same code, message and references, is not reported
//! against it ([`introduced`]): an interim member the operation does not touch
//! does not block an unrelated condition edit, and the solve still refuses it by
//! name. The solve still refuses a case with a staged finding by name; the
//! applier computes no temperature difference of its own.
//!
//! Inverses: create and delete (at the deleted index), update and update with
//! the before record, set and clear or set. The desktop undoes through its
//! session checkpoint, which restores the prior document exactly.

use crate::document_validator;
use crate::geometry_operations::{GeometryEdit, GeometryError};
use crate::{canonical_json, is_valid_schema_id, DiffPreviewRow};
use open_pipe_stress_product_physics::operating_conditions::{self as pp_conditions, resolution as pp_resolution};
use serde_json::{Map, Value};
use std::collections::BTreeSet;

/// Creates one condition set.
pub(crate) const CREATE: &str = "create_condition_set";
/// Replaces one condition set's record, keeping its id.
pub(crate) const UPDATE: &str = "update_condition_set";
/// Deletes one condition set no pipe references.
pub(crate) const DELETE: &str = "delete_condition_set";
/// Sets or clears one pipe's `condition_set_ref`.
pub(crate) const ASSIGN: &str = "assign_condition_set";
/// Sets one load case's `condition_use`.
pub(crate) const SET_USE: &str = "set_load_case_condition_use";
/// Removes one load case's `condition_use`.
pub(crate) const CLEAR_USE: &str = "clear_load_case_condition_use";

/// The record operations' object type and collection.
pub(crate) const OBJECT_TYPE: &str = "ConditionSet";
pub(crate) const COLLECTION: &str = "condition_sets";

/// A set a pipe still references is not deleted.
pub(crate) const CONDITION_SET_IN_USE: &str = "OP-CONDITION-SET-IN-USE";
/// The solve's request parse refuses the candidate document.
pub(crate) const JUDGEMENT_REFUSED: &str = "OP-CONDITION-JUDGEMENT-REFUSED";
/// The solve's code for missing equivalent-static inputs; an empty wind
/// exposure resolved from the sets meets it (§1.5 step 3).
pub(crate) const EQUIVALENT_STATIC_INPUT_MISSING: &str = "EQUIVALENT_STATIC_INPUT_MISSING";

/// One finding of the solve's judgement that is a staged state, reported as a
/// warning.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Staged {
    /// A diagnostic the preview gives, by its code.
    Code(&'static str),
    /// A case whose wind exposure, resolved from the sets, lists no pipe: the
    /// solve's `EQUIVALENT_STATIC_INPUT_MISSING` on the case's generated
    /// `exposed_pipe_refs`. The preview reports the empty list, not the code, so
    /// the applier names it from the preview's case record. The same code from
    /// any other source is not on this list and blocks.
    EmptyGeneratedWindExposure,
}

/// The staged-state findings (§11, SWBPIPE Agent 0, 2026-11-02): exactly these
/// are warnings. Every other finding blocks.
pub(crate) const STAGED_FINDINGS: [Staged; 8] = [
    Staged::Code(pp_resolution::OPERATING_CONDITION_SET_MISSING),
    Staged::Code(pp_resolution::OPERATING_CONDITION_KEY_MISSING),
    Staged::Code(pp_resolution::OPERATING_CONDITION_VALUE_MISSING),
    Staged::Code(pp_resolution::OPERATING_CONDITION_INSTALLATION_TEMPERATURE_MISSING),
    Staged::Code(pp_resolution::LOAD_CASE_CONDITION_TEMPERATURE_CONFLICT),
    Staged::Code(pp_resolution::LOAD_CASE_CONDITION_WIND_EXPOSURE_CONFLICT),
    Staged::Code(pp_resolution::LOAD_CASE_CONDITION_WIND_INPUT_MISSING),
    Staged::EmptyGeneratedWindExposure,
];

fn is_staged_code(code: &str) -> bool {
    STAGED_FINDINGS.iter().any(|staged| matches!(staged, Staged::Code(listed) if *listed == code))
}

/// True for a change kind this module resolves.
pub(crate) fn is_change_kind(change_kind: &str) -> bool {
    operation_kind_for(change_kind).is_some()
}

/// The operation kind of one of this module's change kinds.
pub(crate) fn operation_kind_for(change_kind: &str) -> Option<&'static str> {
    match change_kind {
        CREATE => Some("create"),
        UPDATE | ASSIGN | SET_USE | CLEAR_USE => Some("modify"),
        DELETE => Some("delete"),
        _ => None,
    }
}

fn refusal(code: &'static str, message: impl Into<String>, refs: &[String]) -> GeometryError {
    GeometryError { code, message: message.into(), affected_refs: refs.to_vec() }
}

fn text_at<'a>(intent: &'a Value, pointer: &str) -> &'a str {
    intent.pointer(pointer).and_then(Value::as_str).unwrap_or("")
}

fn id_of(record: &Value) -> Option<&str> {
    record.get("id").and_then(Value::as_str)
}

/// Every id in the document an authored id must not repeat (the model-data
/// rule): entity `id`s at any depth plus the assumption and constraint ids.
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

/// The pipes whose `condition_set_ref` names `set_id`, in `pipe_segments` order.
fn referencing_pipes(model: &Value, set_id: &str) -> Vec<String> {
    model
        .get("pipe_segments")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter(|pipe| pipe.get("condition_set_ref").and_then(Value::as_str) == Some(set_id))
        .map(|pipe| id_of(pipe).unwrap_or("").to_string())
        .collect()
}

fn has_pressure_contract(model: &Value) -> bool {
    model.get("pressure_contract").is_some_and(|contract| !contract.is_null())
}

/// The route admission of a case's `condition_use` (§11). Returns every
/// refusal; empty means admitted. A use that is not an object, an empty use and
/// `false` in a `true`-only member are the validator's findings on the
/// candidate.
pub(crate) fn admit_case_use(model: &Value, case_ref: &str, using: &Value) -> Vec<GeometryError> {
    let refs = [case_ref.to_string()];
    let Some(members) = using.as_object() else {
        return Vec::new();
    };
    if has_pressure_contract(model) {
        return vec![refusal(
            pp_conditions::LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED,
            format!("Load case `{case_ref}` would use operating conditions on a document with a pressure contract; the exact route's condition resolution is not built yet (S4), so the use is not authored."),
            &refs,
        )];
    }
    let key = |member: &str| members.get(member).map(|value| value.as_str().map(str::to_string).unwrap_or_else(|| canonical_json(value)));
    let mut refusals = Vec::new();
    if let Some(key) = key("pressure") {
        refusals.push(refusal(
            pp_resolution::LOAD_CASE_CONDITION_PRESSURE_WITHOUT_PRESSURE_CONTRACT,
            format!("Load case `{case_ref}` would use condition `{key}` for pressure on a pressure-free document, which has no pressure carrier; pressure from conditions needs a pressure contract."),
            &refs,
        ));
    }
    if let Some(key) = key("contents_density") {
        refusals.push(refusal(
            pp_resolution::LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED,
            format!("Load case `{case_ref}` would use condition `{key}` for contents density; the resolution of contents density is not designed yet (RT-A6), so the use is not authored."),
            &refs,
        ));
    }
    refusals
}

/// One finding of the classified judgement.
#[derive(Debug, PartialEq)]
pub(crate) struct JudgedFinding {
    pub code: String,
    pub severity: &'static str,
    pub message: String,
    pub affected_refs: Vec<String>,
}

/// The solve's message for a case whose wind generation lacks inputs
/// (`product_physics` equivalent-static wind generation), for a case whose
/// resolved exposure is empty: the members it would name, in its order.
fn empty_exposure_solve_message(case: Option<&Value>) -> String {
    let wind = case.and_then(|case| case.pointer("/equivalent_static/wind"));
    let absent = |member: &str| wind.and_then(|wind| wind.get(member)).is_none_or(Value::is_null);
    let mut missing: Vec<&str> = ["pressure", "shape_factor", "direction"].into_iter().filter(|member| absent(member)).collect();
    missing.push("exposed_pipe_refs or exposed_spans");
    format!(
        "wind equivalent-static generation requires user-entered {}; no value or marked span is defaulted",
        missing.join(", ")
    )
}

/// Classify the preview's report on `candidate` (§11): the staged findings are
/// warnings with the solve's code and message; everything else blocks.
pub(crate) fn classify(candidate: &Value, preview: &Value) -> Vec<JudgedFinding> {
    let mut found = Vec::new();
    if let Some(text) = preview.get("refusal").and_then(Value::as_str) {
        found.push(JudgedFinding {
            code: JUDGEMENT_REFUSED.to_string(),
            severity: "blocking",
            message: format!("The solve's request parse refuses the candidate document, so its operating conditions cannot be judged: {text}"),
            affected_refs: vec!["model".to_string()],
        });
    }
    for diagnostic in preview.get("diagnostics").and_then(Value::as_array).into_iter().flatten() {
        let code = diagnostic.get("code").and_then(Value::as_str).unwrap_or("").to_string();
        let severity = if is_staged_code(&code) { "warning" } else { "blocking" };
        found.push(JudgedFinding {
            message: diagnostic.get("message").and_then(Value::as_str).unwrap_or("").to_string(),
            affected_refs: diagnostic
                .get("affected_refs")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect(),
            code,
            severity,
        });
    }
    if STAGED_FINDINGS.contains(&Staged::EmptyGeneratedWindExposure) {
        for case in preview.get("cases").and_then(Value::as_array).into_iter().flatten() {
            let empty = case.get("wind_exposed_pipe_refs").and_then(Value::as_array).is_some_and(Vec::is_empty);
            if !empty {
                continue;
            }
            let case_id = case.get("case_id").and_then(Value::as_str).unwrap_or("");
            let record = candidate
                .get("load_cases")
                .and_then(Value::as_array)
                .and_then(|cases| cases.iter().find(|item| id_of(item) == Some(case_id)));
            found.push(JudgedFinding {
                code: EQUIVALENT_STATIC_INPUT_MISSING.to_string(),
                severity: "warning",
                message: format!(
                    "{} (load case `{case_id}` takes its wind exposure from conditions, and no condition set of its pipes marks exposure.wind, so the generated equivalent_static.wind.exposed_pipe_refs is empty)",
                    empty_exposure_solve_message(record)
                ),
                affected_refs: vec![case_id.to_string()],
            });
        }
    }
    found
}

/// The classified judgement of `candidate`, or nothing when the document
/// validator blocks it (the common path then reports the validator's findings,
/// which the preview would repeat).
pub(crate) fn judgement(candidate: &Value) -> Vec<JudgedFinding> {
    if document_validator::has_blocking(&document_validator::validate_model_document(candidate)) {
        return Vec::new();
    }
    classify(candidate, &pp_conditions::preview_operating_conditions(candidate))
}

/// The findings of `candidate`'s judgement this operation introduces: every
/// staged warning, and each blocking finding not already given, with the same
/// code, message and references, by the judgement of the document before the
/// operation. A refusal the document already carries (an interim member the
/// operation does not touch, such as a hydrostatic test case, a skewed support
/// direction or a snubber) is not this operation's: the solve still refuses the
/// document by name, and an unrelated condition edit is not blocked by it. A
/// blocking finding the operation introduces always blocks.
pub(crate) fn introduced(before: Vec<JudgedFinding>, candidate: Vec<JudgedFinding>) -> Vec<JudgedFinding> {
    let mut carried: Vec<(String, String, Vec<String>)> = before
        .into_iter()
        .filter(|finding| finding.severity == "blocking")
        .map(|finding| (finding.code, finding.message, finding.affected_refs))
        .collect();
    candidate
        .into_iter()
        .filter(|finding| {
            if finding.severity != "blocking" {
                return true;
            }
            let key = (finding.code.clone(), finding.message.clone(), finding.affected_refs.clone());
            match carried.iter().position(|item| *item == key) {
                Some(index) => {
                    carried.swap_remove(index);
                    false
                }
                None => true,
            }
        })
        .collect()
}

/// Push the findings of [`judgement`] this operation introduces ([`introduced`])
/// onto the operation's diagnostics.
pub(crate) fn report_judgement(checker: &mut crate::Checker, before: &Value, candidate: &Value) {
    for finding in introduced(judgement(before), judgement(candidate)) {
        let remediation = if finding.severity == "warning" {
            "Authoring is not blocked; the solve refuses the named case until the named input is stated."
        } else {
            "The solve refuses the candidate document for this finding; correct it or the operation and retry."
        };
        checker.push(&finding.code, finding.severity, finding.message, remediation, finding.affected_refs);
    }
}

fn shape_ok(intent: &Value) -> bool {
    text_at(intent, "/change/unit") == "none" && text_at(intent, "/change/dimension") == "dimensionless"
}

fn document_ok(model: &Value) -> bool {
    model.get("document_kind").and_then(Value::as_str) == Some("openpipestress.product_preview.model")
        && model.get("schema_version").and_then(Value::as_str) == Some(open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION)
}

fn row(entity_ref: &str, object_type: &str, field_path: &str, before: String, after: String, change_kind: &str) -> DiffPreviewRow {
    DiffPreviewRow {
        entity_ref: entity_ref.to_string(),
        object_type: object_type.to_string(),
        field_path: field_path.to_string(),
        before,
        after,
        unit: "none".into(),
        dimension: "dimensionless".into(),
        change_kind: change_kind.to_string(),
    }
}

/// Resolve one of this module's operations to its candidate document and diff
/// row, or every refusal it meets.
pub(crate) fn resolve(model: &Value, intent: &Value) -> Result<GeometryEdit, Vec<GeometryError>> {
    let change_kind = text_at(intent, "/change/change_kind");
    let target_ref = text_at(intent, "/target/ref");
    let refs = [target_ref.to_string()];
    if !document_ok(model) {
        return Err(vec![refusal(
            "OP-DATA-DOCUMENT-INVALID",
            format!(
                "Condition sets and uses are authored on the current product_preview.model document {}.",
                open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION
            ),
            &refs,
        )]);
    }
    if !model.is_object() {
        return Err(vec![refusal("OP-DATA-DOCUMENT-INVALID", "The model must be an object.", &refs)]);
    }
    match change_kind {
        CREATE | UPDATE | DELETE => resolve_record(model, intent).map_err(|error| vec![error]),
        ASSIGN => resolve_assignment(model, intent).map_err(|error| vec![error]),
        SET_USE | CLEAR_USE => resolve_use(model, intent),
        _ => Err(vec![refusal("OP-KIND-UNSUPPORTED", format!("`{change_kind}` is not a condition-set operation."), &refs)]),
    }
}

fn resolve_record(model: &Value, intent: &Value) -> Result<GeometryEdit, GeometryError> {
    let change_kind = text_at(intent, "/change/change_kind");
    let object_type = text_at(intent, "/target/object_type");
    let target_ref = text_at(intent, "/target/ref");
    let field_path = text_at(intent, "/change/field_path");
    let before = text_at(intent, "/change/before");
    let after = text_at(intent, "/change/after");
    let refs = [target_ref.to_string()];
    // `condition_sets[<index>]` places a created record (a delete's inverse).
    let insert_at = field_path
        .strip_prefix(COLLECTION)
        .and_then(|rest| rest.strip_prefix('['))
        .and_then(|rest| rest.strip_suffix(']'))
        .filter(|digits| !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit()))
        .and_then(|digits| digits.parse::<usize>().ok());
    let path_ok = field_path == COLLECTION || (change_kind == CREATE && insert_at.is_some());
    if object_type != OBJECT_TYPE || !path_ok || !shape_ok(intent) {
        return Err(refusal(
            "OP-DATA-SHAPE-INVALID",
            format!("{change_kind} requires object type {OBJECT_TYPE}, field_path {COLLECTION} (or {COLLECTION}[<index>] to place a created record), unit none and dimension dimensionless."),
            &refs,
        ));
    }
    if target_ref.trim().is_empty() {
        return Err(refusal("OP-DATA-SHAPE-INVALID", format!("{change_kind} requires the set's id as target.ref."), &refs));
    }
    let records: &[Value] = match model.get(COLLECTION) {
        None => &[],
        Some(Value::Array(items)) => items,
        Some(_) => return Err(refusal("OP-DATA-COLLECTION-INVALID", format!("`{COLLECTION}` is not an array; repair the document first."), &refs)),
    };
    let matching: Vec<usize> =
        records.iter().enumerate().filter(|(_, record)| id_of(record) == Some(target_ref)).map(|(index, _)| index).collect();
    let parse_record = |label: &str| -> Result<Map<String, Value>, GeometryError> {
        match serde_json::from_str::<Value>(after) {
            Ok(Value::Object(record)) => Ok(record),
            _ => Err(refusal("OP-DATA-PAYLOAD-INVALID", format!("The {label} condition set must be a JSON object."), &refs)),
        }
    };

    let mut candidate = model.clone();
    let root = candidate.as_object_mut().expect("checked object");
    let (row_before, row_after) = match change_kind {
        CREATE => {
            if !matching.is_empty() {
                return Err(refusal("OP-DATA-ID-COLLISION", format!("Condition set `{target_ref}` already exists."), &refs));
            }
            if before != "not_present" {
                return Err(refusal("OP-STALE-BEFORE-VALUE", format!("A new condition set has before-value `not_present`, not `{before}`."), &refs));
            }
            let record = parse_record("new")?;
            if record.get("id").and_then(Value::as_str) != Some(target_ref) {
                return Err(refusal("OP-DATA-ID-MISMATCH", format!("The new set's `id` must equal target.ref `{target_ref}`."), &refs));
            }
            if !is_valid_schema_id(target_ref) {
                return Err(refusal("OP-DATA-ID-INVALID", format!("`{target_ref}` does not match the id pattern ^[A-Za-z][A-Za-z0-9_.:-]*$."), &refs));
            }
            if document_ids(model).contains(target_ref) {
                return Err(refusal("OP-DATA-ID-COLLISION", format!("`{target_ref}` is already an id in this document; ids are unique."), &refs));
            }
            let record = Value::Object(record);
            let after_text = canonical_json(&record);
            let list = root.entry(COLLECTION).or_insert_with(|| Value::Array(Vec::new())).as_array_mut().expect("checked array");
            match insert_at {
                Some(index) if index > list.len() => {
                    return Err(refusal(
                        "OP-DATA-INDEX-INVALID",
                        format!("{COLLECTION} has {} records; a created set is placed at 0 to {}.", list.len(), list.len()),
                        &refs,
                    ));
                }
                Some(index) => list.insert(index, record),
                None => list.push(record),
            }
            ("not_present".to_string(), after_text)
        }
        _ => {
            let [index] = matching[..] else {
                return Err(refusal("OP-TARGET-NOT-FOUND", format!("Condition-set edits require exactly one `{target_ref}` in {COLLECTION}."), &refs));
            };
            let current = canonical_json(&records[index]);
            if before != current {
                return Err(refusal(
                    "OP-STALE-BEFORE-VALUE",
                    format!("Intent before-value no longer matches the current condition set `{target_ref}`."),
                    &refs,
                ));
            }
            let list = root.get_mut(COLLECTION).and_then(Value::as_array_mut).expect("checked array");
            if change_kind == UPDATE {
                let record = parse_record("updated")?;
                if record.get("id").and_then(Value::as_str) != Some(target_ref) {
                    return Err(refusal(
                        "OP-DATA-ID-MISMATCH",
                        format!("An update never changes a set's `id`; the record keeps `{target_ref}` (FR-MOD-003)."),
                        &refs,
                    ));
                }
                let record = Value::Object(record);
                if record == records[index] {
                    return Err(refusal("OP-DATA-UNCHANGED", "The updated condition set equals the current one; nothing to apply.", &refs));
                }
                let after_text = canonical_json(&record);
                list[index] = record;
                (current, after_text)
            } else {
                if after != "not_present" {
                    return Err(refusal("OP-DELETE-AFTER-VALUE-INVALID", "Deletions use after-value `not_present`.", &refs));
                }
                let pipes = referencing_pipes(model, target_ref);
                if !pipes.is_empty() {
                    let named = pipes.iter().map(|pipe| format!("`{pipe}`")).collect::<Vec<_>>().join(", ");
                    return Err(GeometryError {
                        code: CONDITION_SET_IN_USE,
                        message: format!("Condition set `{target_ref}` is assigned to pipes {named}; assign those pipes another set or clear their assignment first."),
                        affected_refs: std::iter::once(target_ref.to_string()).chain(pipes).collect(),
                    });
                }
                // An emptied collection stays an empty array (§11).
                list.remove(index);
                (current, "not_present".to_string())
            }
        }
    };
    Ok(GeometryEdit {
        model: candidate,
        diff_preview: vec![row(target_ref, OBJECT_TYPE, field_path, row_before, row_after, change_kind)],
    })
}

fn resolve_assignment(model: &Value, intent: &Value) -> Result<GeometryEdit, GeometryError> {
    let object_type = text_at(intent, "/target/object_type");
    let target_ref = text_at(intent, "/target/ref");
    let field_path = text_at(intent, "/change/field_path");
    let before = text_at(intent, "/change/before");
    let after = text_at(intent, "/change/after");
    let refs = [target_ref.to_string()];
    if object_type != "Element" || field_path != "condition_set_ref" || !shape_ok(intent) {
        return Err(refusal(
            "OP-DATA-SHAPE-INVALID",
            format!("{ASSIGN} requires object type Element, field_path condition_set_ref, unit none and dimension dimensionless."),
            &refs,
        ));
    }
    let pipes = model.get("pipe_segments").and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
    let matching: Vec<usize> = pipes.iter().enumerate().filter(|(_, pipe)| id_of(pipe) == Some(target_ref)).map(|(index, _)| index).collect();
    let [index] = matching[..] else {
        return Err(refusal("OP-TARGET-NOT-FOUND", format!("Pipe `{target_ref}` was not found exactly once in pipe_segments."), &refs));
    };
    let current = pipes[index].get("condition_set_ref");
    let member = |value: &Value| {
        let mut record = Map::new();
        record.insert("condition_set_ref".into(), value.clone());
        canonical_json(&Value::Object(record))
    };
    let current_text = current.map(member).unwrap_or_else(|| "not_present".into());
    if before != current_text {
        return Err(refusal(
            "OP-STALE-BEFORE-VALUE",
            format!("Intent before-value `{before}` no longer matches pipe `{target_ref}`'s condition set (`{current_text}`)."),
            &refs,
        ));
    }
    let next = if after == "not_present" {
        None
    } else {
        let parsed = serde_json::from_str::<Value>(after).ok();
        let reference = parsed
            .as_ref()
            .and_then(Value::as_object)
            .filter(|record| record.len() == 1)
            .and_then(|record| record.get("condition_set_ref"))
            .filter(|value| value.is_string());
        let Some(reference) = reference else {
            return Err(refusal(
                "OP-DATA-PAYLOAD-INVALID",
                format!("{ASSIGN} takes after-value {{\"condition_set_ref\": <set id>}}, or `not_present` to clear."),
                &refs,
            ));
        };
        Some(reference.clone())
    };
    if next.as_ref() == current {
        let message = match &next {
            Some(value) => format!("Pipe `{target_ref}` already carries condition set {value}; nothing to apply."),
            None => format!("Pipe `{target_ref}` carries no condition set; nothing to clear."),
        };
        return Err(refusal("OP-DATA-UNCHANGED", message, &refs));
    }
    let mut candidate = model.clone();
    let pipe = candidate
        .get_mut("pipe_segments")
        .and_then(Value::as_array_mut)
        .and_then(|pipes| pipes.get_mut(index))
        .and_then(Value::as_object_mut)
        .ok_or_else(|| refusal("OP-DATA-PAYLOAD-INVALID", format!("Pipe `{target_ref}` is not an object; repair the document first."), &refs))?;
    let after_text = match &next {
        Some(value) => {
            pipe.insert("condition_set_ref".into(), value.clone());
            member(value)
        }
        None => {
            pipe.remove("condition_set_ref");
            "not_present".to_string()
        }
    };
    Ok(GeometryEdit {
        model: candidate,
        diff_preview: vec![row(target_ref, "Element", "condition_set_ref", current_text, after_text, ASSIGN)],
    })
}

fn resolve_use(model: &Value, intent: &Value) -> Result<GeometryEdit, Vec<GeometryError>> {
    let change_kind = text_at(intent, "/change/change_kind");
    let object_type = text_at(intent, "/target/object_type");
    let target_ref = text_at(intent, "/target/ref");
    let field_path = text_at(intent, "/change/field_path");
    let before = text_at(intent, "/change/before");
    let after = text_at(intent, "/change/after");
    let refs = [target_ref.to_string()];
    let one = |code, message: String| Err(vec![refusal(code, message, &refs)]);
    let clearing = change_kind == CLEAR_USE;
    if object_type != "Load" || field_path != "condition_use" || !shape_ok(intent) || clearing != (after == "not_present") {
        return one(
            "OP-DATA-SHAPE-INVALID",
            format!("{change_kind} requires object type Load, field_path condition_use, unit none and dimension dimensionless; {SET_USE} takes the whole condition use as JSON object text and {CLEAR_USE} takes after-value `not_present`."),
        );
    }
    let cases = model.get("load_cases").and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
    let matching: Vec<usize> = cases.iter().enumerate().filter(|(_, case)| id_of(case) == Some(target_ref)).map(|(index, _)| index).collect();
    let [index] = matching[..] else {
        return one("OP-TARGET-NOT-FOUND", format!("Load case `{target_ref}` was not found exactly once in load_cases."));
    };
    let current = cases[index].get("condition_use");
    let current_text = current.map(canonical_json).unwrap_or_else(|| "not_present".into());
    if before != current_text {
        return one(
            "OP-STALE-BEFORE-VALUE",
            format!("Intent before-value `{before}` no longer matches load case `{target_ref}`'s condition use (`{current_text}`)."),
        );
    }
    let next = if clearing {
        None
    } else {
        let Ok(parsed @ Value::Object(_)) = serde_json::from_str::<Value>(after) else {
            return one("OP-DATA-PAYLOAD-INVALID", format!("{SET_USE} takes the whole condition use as a JSON object."));
        };
        let refusals = admit_case_use(model, target_ref, &parsed);
        if !refusals.is_empty() {
            return Err(refusals);
        }
        Some(parsed)
    };
    if next.as_ref() == current {
        let message = if clearing {
            format!("Load case `{target_ref}` has no condition use; nothing to clear.")
        } else {
            format!("Load case `{target_ref}`'s condition use is already {current_text}; nothing to apply.")
        };
        return one("OP-DATA-UNCHANGED", message);
    }
    let mut candidate = model.clone();
    let Some(case) = candidate
        .get_mut("load_cases")
        .and_then(Value::as_array_mut)
        .and_then(|cases| cases.get_mut(index))
        .and_then(Value::as_object_mut)
    else {
        return one("OP-DATA-PAYLOAD-INVALID", format!("Load case `{target_ref}` is not an object; repair the document first."));
    };
    let after_text = match next {
        Some(value) => {
            let text = canonical_json(&value);
            case.insert("condition_use".into(), value);
            text
        }
        None => {
            case.remove("condition_use");
            "not_present".to_string()
        }
    };
    Ok(GeometryEdit {
        model: candidate,
        diff_preview: vec![row(target_ref, "Load", "condition_use", current_text, after_text, change_kind)],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn preview(diagnostics: Value) -> Value {
        json!({"document_kind": pp_conditions::OPERATING_CONDITION_PREVIEW_DOCUMENT_KIND, "resolved": false, "cases": [], "diagnostics": diagnostics})
    }

    fn diagnostic(code: &str) -> Value {
        json!({"id": format!("diagnostic:{code}"), "code": code, "severity": "blocking", "message": format!("message of {code}"), "affected_refs": ["L1"]})
    }

    /// Exactly the eight staged findings are warnings; the list is the one
    /// constant and nothing else is downgraded.
    #[test]
    fn a_blocking_finding_blocks_unless_the_document_already_carried_it() {
        let finding = |code: &str, severity: &'static str, refs: &[&str]| JudgedFinding {
            code: code.to_string(),
            severity,
            message: format!("{code} message"),
            affected_refs: refs.iter().map(|r| r.to_string()).collect(),
        };
        let before = vec![finding("CARRIED", "blocking", &["HT"]), finding("OPERATING_CONDITION_SET_MISSING", "warning", &["SUS"])];
        let after = vec![
            finding("CARRIED", "blocking", &["HT"]),
            finding("CARRIED", "blocking", &["HT"]),
            finding("CARRIED", "blocking", &["OTHER"]),
            finding("INTRODUCED", "blocking", &["SUS"]),
            finding("OPERATING_CONDITION_SET_MISSING", "warning", &["SUS"]),
        ];
        let kept: Vec<(String, &str, Vec<String>)> =
            introduced(before, after).into_iter().map(|f| (f.code, f.severity, f.affected_refs)).collect();
        assert_eq!(
            kept,
            vec![
                ("CARRIED".to_string(), "blocking", vec!["HT".to_string()]),
                ("CARRIED".to_string(), "blocking", vec!["OTHER".to_string()]),
                ("INTRODUCED".to_string(), "blocking", vec!["SUS".to_string()]),
                ("OPERATING_CONDITION_SET_MISSING".to_string(), "warning", vec!["SUS".to_string()]),
            ],
            "one carried occurrence is consumed; a second occurrence, other references and new codes block; warnings stay"
        );
    }

    #[test]
    fn the_staged_list_is_exactly_the_eight_findings() {
        let codes: Vec<&str> = STAGED_FINDINGS
            .iter()
            .filter_map(|staged| match staged {
                Staged::Code(code) => Some(*code),
                Staged::EmptyGeneratedWindExposure => None,
            })
            .collect();
        assert_eq!(
            codes,
            [
                "OPERATING_CONDITION_SET_MISSING",
                "OPERATING_CONDITION_KEY_MISSING",
                "OPERATING_CONDITION_VALUE_MISSING",
                "OPERATING_CONDITION_INSTALLATION_TEMPERATURE_MISSING",
                "LOAD_CASE_CONDITION_TEMPERATURE_CONFLICT",
                "LOAD_CASE_CONDITION_WIND_EXPOSURE_CONFLICT",
                "LOAD_CASE_CONDITION_WIND_INPUT_MISSING",
            ]
        );
        assert!(STAGED_FINDINGS.contains(&Staged::EmptyGeneratedWindExposure));
        for code in &codes {
            let found = classify(&json!({}), &preview(json!([diagnostic(code)])));
            assert_eq!(found.len(), 1);
            assert_eq!((found[0].code.as_str(), found[0].severity), (*code, "warning"));
            assert_eq!(found[0].message, format!("message of {code}"), "the solve's message, verbatim");
            assert_eq!(found[0].affected_refs, ["L1"]);
        }
    }

    /// A code not on the list blocks: route, admission, document-level, interim
    /// and unknown or future codes, and the same `EQUIVALENT_STATIC_INPUT_MISSING`
    /// code when the preview itself gives it. A warning-severity diagnostic not
    /// on the list blocks too.
    #[test]
    fn a_code_not_on_the_list_blocks() {
        for code in [
            "LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED",
            "LOAD_CASE_CONDITION_PRESSURE_WITHOUT_PRESSURE_CONTRACT",
            "LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED",
            "LOAD_CASE_CONDITION_USE_EMPTY",
            "OPERATING_CONDITION_SET_UNRESOLVED",
            "OPERATING_CONDITION_QUANTITY_INVALID",
            "OPERATING_CONDITION_TEMPERATURE_RANGE_UNSUPPORTED",
            "LOAD_CASE_TEST_CONDITION_UNSUPPORTED",
            "SUPPORT_DIRECTION_SKEW_UNSUPPORTED",
            "EQUIVALENT_STATIC_INPUT_MISSING",
            "PREVIEW_SCHEMA_VERSION_MISSING",
            "OPERATING_CONDITION_SOMETHING_ADDED_LATER",
        ] {
            let mut warning = diagnostic(code);
            warning["severity"] = json!("warning");
            for item in [diagnostic(code), warning] {
                let found = classify(&json!({}), &preview(json!([item])));
                assert_eq!(found.len(), 1);
                assert_eq!((found[0].code.as_str(), found[0].severity), (code, "blocking"), "{code}");
            }
        }
        let refused = classify(&json!({}), &json!({"resolved": false, "refusal": "unknown field `x`", "cases": [], "diagnostics": []}));
        assert_eq!(refused.len(), 1);
        assert_eq!((refused[0].code.as_str(), refused[0].severity), (JUDGEMENT_REFUSED, "blocking"));
        assert!(refused[0].message.ends_with("unknown field `x`"));
    }

    /// A resolved case with an empty generated wind exposure is the derived
    /// staged finding, named by its case; an absent (`null`) exposure is no use.
    #[test]
    fn an_empty_generated_exposure_is_named_by_its_case() {
        let report = json!({"resolved": true, "diagnostics": [], "cases": [
            {"case_id": "W", "generated_primitive_loads": [], "wind_exposed_pipe_refs": []},
            {"case_id": "X", "generated_primitive_loads": [], "wind_exposed_pipe_refs": null},
            {"case_id": "Y", "generated_primitive_loads": [], "wind_exposed_pipe_refs": ["P1"]},
        ]});
        let candidate = json!({"load_cases": [{"id": "W", "equivalent_static": {"wind": {"pressure": {"value": 1.0, "unit": "Pa"}, "direction": "global_x"}}}]});
        let found = classify(&candidate, &report);
        assert_eq!(found.len(), 1);
        assert_eq!((found[0].code.as_str(), found[0].severity), (EQUIVALENT_STATIC_INPUT_MISSING, "warning"));
        assert!(found[0].message.starts_with(
            "wind equivalent-static generation requires user-entered shape_factor, exposed_pipe_refs or exposed_spans; no value or marked span is defaulted"
        ));
        assert!(found[0].message.contains("load case `W`"));
        assert_eq!(found[0].affected_refs, ["W"]);
    }
}
