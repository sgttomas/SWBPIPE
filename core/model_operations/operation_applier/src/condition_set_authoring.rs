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
//! "The same value" ([`same_value`]) means the same canonical JSON text (RFC
//! 8785, as the stale before-value check and the model hash judge it) and the
//! same exact numbers, so `20`, `20.0` and `2e1` (or `-0` and `0`) are one value
//! whichever lane parsed the document, while numerically different values whose
//! canonical text coincides (an integer and a float beyond 2^53) stay different.
//! The product creates no set and writes no value the person did not enter.
//!
//! **Route admission of a use** ([`admit_case_use`], also applied to
//! `create_load_case`'s `condition_use`): on a pressure-free document
//! `temperature`, `added_weight: true`, `wind_exposure: true` and (RT-A6 H1)
//! `contents_density` are admitted, and `pressure` is refused
//! (`LOAD_CASE_CONDITION_PRESSURE_WITHOUT_PRESSURE_CONTRACT`). On a document with
//! a `pressure_contract` a use stating only `contents_density` is admitted
//! (RT-A6 H1) and every other use is refused
//! (`LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED`, until S4). A contents use is
//! mass input: until self-weight generated from the case's own selection
//! consumes it, the solve refuses it by name
//! (`LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED`), which authoring reports as a
//! staged warning, so a source case's selection can be stated first.
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
//! blocks, with one exception ([`permit_carried`], SWBPIPE Agent 0, 2026-11-02,
//! option (b)): a refusal the document already carried before the operation,
//! unchanged (same code, message and references, matched as multisets),
//! naming only entities of the document before the edit, outside the edit's
//! dependency scope ([`edit_scope`]) and not a route or admission code, is
//! reported as `info` with the solve's code and message, and the edit applies.
//! The solve still refuses it, and every case with a staged finding, by name;
//! nothing records that the document solves. The applier computes no
//! temperature difference of its own.
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
/// are warnings. Every other finding blocks. RT-A6 H1 adds the unconsumed
/// contents use: a case may state its contents selection before the
/// self-weight generated from it exists (the solve still refuses it by name).
pub(crate) const STAGED_FINDINGS: [Staged; 9] = [
    Staged::Code(pp_resolution::OPERATING_CONDITION_SET_MISSING),
    Staged::Code(pp_resolution::OPERATING_CONDITION_KEY_MISSING),
    Staged::Code(pp_resolution::OPERATING_CONDITION_VALUE_MISSING),
    Staged::Code(pp_resolution::OPERATING_CONDITION_INSTALLATION_TEMPERATURE_MISSING),
    Staged::Code(pp_resolution::LOAD_CASE_CONDITION_TEMPERATURE_CONFLICT),
    Staged::Code(pp_resolution::LOAD_CASE_CONDITION_WIND_EXPOSURE_CONFLICT),
    Staged::Code(pp_resolution::LOAD_CASE_CONDITION_WIND_INPUT_MISSING),
    Staged::Code(pp_resolution::LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED),
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

/// The one predicate for "the document has a pressure contract": present and not
/// JSON null (any other value, a retired or invalid version included, counts).
/// Shared by the condition-set admission and `create_support`'s skew admission.
/// This is routing only: a present null is not absence, and the shared document
/// validator refuses a candidate that keeps it (`MODEL-SLOT-SHAPE` at
/// `pressure_contract`, Agent 0 2026-11-05), so no null document is admitted here.
pub(crate) fn has_pressure_contract(model: &Value) -> bool {
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
        // RT-A6 H1: contents density alone is admitted on the exact route.
        if members.keys().all(|member| member == "contents_density") {
            return Vec::new();
        }
        return vec![refusal(
            pp_conditions::LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED,
            format!("Load case `{case_ref}` would use operating conditions other than contents density on a document with a pressure contract; the exact route's condition resolution is not built yet (S4), so the use is not authored."),
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

/// The change kind of `create_load_case`, whose carried `condition_use` is
/// judged here (lib.rs's own kind).
pub(crate) const CREATE_LOAD_CASE: &str = "create_load_case";

/// The route and admission refusals of a case's use (§11): never a permitted
/// carried refusal, wherever they appear.
const ROUTE_ADMISSION_CODES: [&str; 3] = [
    pp_conditions::LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED,
    pp_resolution::LOAD_CASE_CONDITION_PRESSURE_WITHOUT_PRESSURE_CONTRACT,
    // The exact route's refusal of a declared operating comparison (review r1).
    "LOAD_COMBINATION_DIFFERENCE_BASIS_EXACT_UNSUPPORTED",
];

/// A route or admission refusal is never carried: one of
/// [`ROUTE_ADMISSION_CODES`], or any exact-route refusal (`*_EXACT_UNSUPPORTED`).
fn is_route_or_admission(code: &str) -> bool {
    ROUTE_ADMISSION_CODES.contains(&code) || code.ends_with("_EXACT_UNSUPPORTED")
}

/// The operand ids a combination names: its terms' `load_case` and
/// `combination`, its `minuend_id`, `subtrahend_id` and `operand_ids`.
fn combination_operands(combination: &Value) -> Vec<&str> {
    let mut found = Vec::new();
    for term in combination.get("terms").and_then(Value::as_array).into_iter().flatten() {
        for key in ["load_case", "combination"] {
            found.extend(term.get(key).and_then(Value::as_str));
        }
    }
    for key in ["minuend_id", "subtrahend_id"] {
        found.extend(combination.get(key).and_then(Value::as_str));
    }
    found.extend(combination.get("operand_ids").and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str));
    found
}

fn records<'a>(model: &'a Value, collection: &str) -> impl Iterator<Item = &'a Value> {
    model.get(collection).and_then(Value::as_array).into_iter().flatten()
}

fn set_ref_of(pipe: &Value) -> Option<&str> {
    pipe.get("condition_set_ref").and_then(Value::as_str)
}

/// The entity ids a carried refusal may name: every `id` of a record in one of
/// the document's top-level collections, at any depth below it. Document-level
/// members (`project`, `analysis_status`, ...) and the collection names are
/// not entities.
fn entity_ids(model: &Value) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    for value in model.as_object().into_iter().flat_map(Map::values).filter(|value| value.is_array()) {
        crate::geometry_operations::global_ids(value, &mut ids);
    }
    ids
}

/// The edit's transitive dependency scope S (SWBPIPE Agent 0, 2026-11-02,
/// option (b); §11): the entities whose judgement the edit can change. It is
/// computed on the union of the document before the edit and the candidate:
/// - **Set operations** (`create_condition_set`, `update_condition_set`,
///   `delete_condition_set` of set X): X; every pipe whose `condition_set_ref`
///   is X; and every load case with a `condition_use`. Condition resolution
///   reads every pipe's set for any case that uses conditions, so these are the
///   effective cases fed by any set.
/// - **`assign_condition_set`** on pipe P, from set X0 to X1 (either may be
///   absent): P, X0 and X1; every pipe referencing X0 or X1; and every case
///   with a `condition_use`.
/// - **`set_load_case_condition_use` / `clear_load_case_condition_use`** on
///   case L, and **`create_load_case`** of L carrying a use: L; every pipe that
///   carries a set; and every set referenced by any pipe.
///
/// S also holds the operation's target ref and every entity its diff row names:
/// each of these operations' rows names its target (`entity_ref`), and an
/// assignment's row names X0 and X1 (`before`, `after`), already above. Any
/// other change kind is not judged here; given one, S is every entity of both
/// documents, so nothing is carried past it.
pub(crate) fn edit_scope(before: &Value, candidate: &Value, change_kind: &str, target_ref: &str) -> BTreeSet<String> {
    let documents = [before, candidate];
    let pipes = || documents.into_iter().flat_map(|model| records(model, "pipe_segments"));
    let pipes_on = |sets: &BTreeSet<String>| -> Vec<String> {
        pipes()
            .filter(|pipe| set_ref_of(pipe).is_some_and(|set| sets.contains(set)))
            .filter_map(id_of)
            .map(str::to_string)
            .collect()
    };
    let using_cases = || -> Vec<String> {
        documents
            .into_iter()
            .flat_map(|model| records(model, "load_cases"))
            .filter(|case| case.get("condition_use").is_some())
            .filter_map(id_of)
            .map(str::to_string)
            .collect()
    };
    let mut scope = BTreeSet::from([target_ref.to_string()]);
    match change_kind {
        CREATE | UPDATE | DELETE => {
            let sets = BTreeSet::from([target_ref.to_string()]);
            scope.extend(pipes_on(&sets));
            scope.extend(using_cases());
        }
        ASSIGN => {
            let sets: BTreeSet<String> = documents
                .into_iter()
                .flat_map(|model| records(model, "pipe_segments"))
                .filter(|pipe| id_of(pipe) == Some(target_ref))
                .filter_map(set_ref_of)
                .map(str::to_string)
                .collect();
            scope.extend(sets.iter().cloned());
            scope.extend(pipes_on(&sets));
            scope.extend(using_cases());
        }
        SET_USE | CLEAR_USE | CREATE_LOAD_CASE => {
            for pipe in pipes() {
                if let Some(set) = set_ref_of(pipe) {
                    scope.insert(set.to_string());
                    scope.extend(id_of(pipe).map(str::to_string));
                }
            }
        }
        _ => {
            scope.extend(entity_ids(before));
            scope.extend(entity_ids(candidate));
        }
    }
    // Downstream combinations (review r1): every combination naming an entity
    // in scope, directly or through another combination, in either document,
    // is in scope too, to a fixed point.
    loop {
        let added: Vec<String> = documents
            .into_iter()
            .flat_map(|model| records(model, "combinations"))
            .filter(|combination| combination_operands(combination).iter().any(|operand| scope.contains(*operand)))
            .filter_map(id_of)
            .filter(|id| !scope.contains(*id))
            .map(str::to_string)
            .collect();
        if added.is_empty() {
            break;
        }
        scope.extend(added);
    }
    scope
}

/// Whether `finding` names only entities of the document before the edit: at
/// least one affected ref, each an entity id (not `model`, a collection or a
/// document-level member), and not the judgement's parse refusal.
fn attributable(finding: &JudgedFinding, before: &Value, before_ids: &BTreeSet<String>) -> bool {
    finding.code != JUDGEMENT_REFUSED
        && !finding.affected_refs.is_empty()
        && finding.affected_refs.iter().all(|reference| {
            reference != "model" && before.get(reference.as_str()).is_none() && before_ids.contains(reference)
        })
}

/// Classify the candidate's judgement against the document before the edit
/// (SWBPIPE Agent 0, 2026-11-02, option (b)). A non-staged blocking finding of
/// the candidate becomes a permitted carried refusal, reported as `info`, only
/// when it is unchanged (the before-document's judgement gives the same code,
/// message and affected refs, matched as multisets so an extra occurrence is
/// worsened and blocks), attributable ([`attributable`]), outside `scope`
/// ([`edit_scope`]) in every affected ref, and not a route or admission code.
/// Everything else blocks; staged findings keep their classification.
pub(crate) fn permit_carried(
    before: &Value,
    before_judgement: Vec<JudgedFinding>,
    candidate_judgement: Vec<JudgedFinding>,
    scope: &BTreeSet<String>,
) -> Vec<JudgedFinding> {
    let before_ids = entity_ids(before);
    let mut carried: Vec<(String, String, Vec<String>)> = before_judgement
        .into_iter()
        .filter(|finding| finding.severity == "blocking")
        .map(|finding| (finding.code, finding.message, finding.affected_refs))
        .collect();
    candidate_judgement
        .into_iter()
        .map(|mut finding| {
            if finding.severity != "blocking" {
                return finding;
            }
            let key = (finding.code.clone(), finding.message.clone(), finding.affected_refs.clone());
            let unchanged = match carried.iter().position(|item| *item == key) {
                Some(index) => {
                    carried.swap_remove(index);
                    true
                }
                None => false,
            };
            let permitted = unchanged
                && !is_route_or_admission(&finding.code)
                && attributable(&finding, before, &before_ids)
                && finding.affected_refs.iter().all(|reference| !scope.contains(reference));
            if permitted {
                finding.severity = "info";
                finding.message = format!(
                    "{} (The document already carried this refusal before this edit, and it lies outside this edit's dependency scope, so it does not block the edit. The solve still refuses it by name.)",
                    finding.message
                );
            }
            finding
        })
        .collect()
}

/// Push [`judgement`]'s findings on `candidate` onto the operation's
/// diagnostics, with the permitted carried refusals of [`permit_carried`]
/// reported as `info`. `before` is the document the operation edits;
/// `change_kind` and `target_ref` are the operation's, for [`edit_scope`].
pub(crate) fn report_judgement(checker: &mut crate::Checker, before: &Value, change_kind: &str, target_ref: &str, candidate: &Value) {
    let found = judgement(candidate);
    let found = if found.iter().any(|finding| finding.severity == "blocking") {
        permit_carried(before, judgement(before), found, &edit_scope(before, candidate, change_kind, target_ref))
    } else {
        found
    };
    for finding in found {
        let remediation = match finding.severity {
            "warning" => "Authoring is not blocked; the solve refuses the named case until the named input is stated.",
            "info" => "Authoring is not blocked by a refusal the document already carried outside this edit's scope; the solve refuses the document until the named member is resolved.",
            _ => "The solve refuses the candidate document for this finding; correct it or the operation and retry.",
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

/// The no-op judgement of `OP-DATA-UNCHANGED`: equal canonical JSON text and
/// equal exact numbers. Canonical text alone would collapse an integer and a
/// float beyond 2^53 that render alike (`1000000000000000100` and the float
/// `1000000000000000128.0`); exact numbers alone would differ from the hash
/// where an integer and an equal float render differently. Requiring both
/// refuses only an edit that changes neither the value nor the canonical bytes.
fn same_value(left: Option<&Value>, right: Option<&Value>) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(left), Some(right)) => canonical_json(left) == canonical_json(right) && same_numbers(left, right),
        _ => false,
    }
}

/// Exact numeric equality over equal-shaped values (`-0` equals `0`); any
/// other difference of shape is left to the canonical-text comparison.
fn same_numbers(left: &Value, right: &Value) -> bool {
    fn integer(number: &serde_json::Number) -> Option<i128> {
        number.as_i64().map(i128::from).or_else(|| number.as_u64().map(i128::from))
    }
    fn float_is(float: Option<f64>, value: i128) -> bool {
        float.is_some_and(|float| float.fract() == 0.0 && float.abs() < 1.0e38 && float as i128 == value)
    }
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => match (integer(a), integer(b)) {
            (Some(a), Some(b)) => a == b,
            (Some(a), None) => float_is(b.as_f64(), a),
            (None, Some(b)) => float_is(a.as_f64(), b),
            (None, None) => a.as_f64() == b.as_f64(),
        },
        (Value::Array(a), Value::Array(b)) => a.len() == b.len() && a.iter().zip(b).all(|(a, b)| same_numbers(a, b)),
        (Value::Object(a), Value::Object(b)) => {
            a.len() == b.len() && a.iter().all(|(key, value)| b.get(key).is_some_and(|other| same_numbers(value, other)))
        }
        (a, b) => a == b,
    }
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
    let mut row_path = field_path.to_string();
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
                let after_text = canonical_json(&record);
                if after_text == current && same_numbers(&record, &records[index]) {
                    return Err(refusal("OP-DATA-UNCHANGED", "The updated condition set equals the current one; nothing to apply.", &refs));
                }
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
                // An emptied collection stays an empty array (§11). The diff row
                // names the deleted index, `condition_sets[<index>]`, so the
                // inverse create restores the record at its place.
                list.remove(index);
                row_path = format!("{COLLECTION}[{index}]");
                (current, "not_present".to_string())
            }
        }
    };
    Ok(GeometryEdit {
        model: candidate,
        diff_preview: vec![row(target_ref, OBJECT_TYPE, &row_path, row_before, row_after, change_kind)],
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
    if same_value(next.as_ref(), current) {
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
                "LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED",
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

    fn refs_diagnostic(code: &str, refs: Value) -> Value {
        json!({"id": format!("diagnostic:{code}"), "code": code, "severity": "blocking", "message": format!("message of {code}"), "affected_refs": refs})
    }

    /// The document the carried-refusal tests judge against: entities `snub`,
    /// `HT`, `L9`, `P1` and `cs:a`, a project id and a document-level member.
    fn before_document() -> Value {
        json!({
            "project": {"id": "proj"},
            "analysis_status": {"mechanics": "ready_for_preview_diagnostics"},
            "supports": [{"id": "snub"}],
            "load_cases": [{"id": "HT", "primitive_loads": [{"id": "HT:t"}]}, {"id": "L9"}],
            "pipe_segments": [{"id": "P1", "condition_set_ref": "cs:a"}],
            "condition_sets": [{"id": "cs:a"}],
        })
    }

    fn outcome_of(found: &[JudgedFinding]) -> Vec<(String, &'static str, Vec<String>)> {
        found.iter().map(|f| (f.code.clone(), f.severity, f.affected_refs.clone())).collect()
    }

    /// A carried refusal outside the scope is `info` with the solve's message
    /// leading; one occurrence more than before is worsened and blocks, as do a
    /// new code, the same code on other refs, a ref in the scope, and route and
    /// admission codes. Staged findings keep their classification.
    #[test]
    fn a_carried_refusal_outside_the_scope_is_info_and_everything_else_blocks() {
        let before = before_document();
        let snub = || refs_diagnostic("SNUBBER_DYNAMIC_ONLY_UNSUPPORTED", json!(["snub"]));
        let test_case = || refs_diagnostic("LOAD_CASE_TEST_CONDITION_UNSUPPORTED", json!(["HT"]));
        let route = || refs_diagnostic(pp_conditions::LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED, json!(["L9"]));
        let staged = || refs_diagnostic(pp_resolution::OPERATING_CONDITION_SET_MISSING, json!(["L9"]));
        let judged_before = classify(&before, &preview(json!([snub(), test_case(), route(), staged()])));
        let judged_after = classify(
            &before,
            &preview(json!([
                snub(),
                snub(),
                test_case(),
                refs_diagnostic("LOAD_CASE_TEST_CONDITION_UNSUPPORTED", json!(["L9"])),
                route(),
                staged(),
                refs_diagnostic("SOMETHING_NEW", json!(["snub"])),
            ])),
        );
        let scope = BTreeSet::from(["HT".to_string()]);
        let found = permit_carried(&before, judged_before, judged_after, &scope);
        let s = |text: &str| text.to_string();
        assert_eq!(
            outcome_of(&found),
            vec![
                (s("SNUBBER_DYNAMIC_ONLY_UNSUPPORTED"), "info", vec![s("snub")]),
                (s("SNUBBER_DYNAMIC_ONLY_UNSUPPORTED"), "blocking", vec![s("snub")]),
                (s("LOAD_CASE_TEST_CONDITION_UNSUPPORTED"), "blocking", vec![s("HT")]),
                (s("LOAD_CASE_TEST_CONDITION_UNSUPPORTED"), "blocking", vec![s("L9")]),
                (s(pp_conditions::LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED), "blocking", vec![s("L9")]),
                (s(pp_resolution::OPERATING_CONDITION_SET_MISSING), "warning", vec![s("L9")]),
                (s("SOMETHING_NEW"), "blocking", vec![s("snub")]),
            ],
            "carried once outside S: info; worsened, in S, other refs, route code, new code: blocking; staged: warning"
        );
        assert!(found[0].message.starts_with("message of SNUBBER_DYNAMIC_ONLY_UNSUPPORTED (The document already carried this refusal before this edit, and it lies outside this edit's dependency scope"));
        assert!(found[0].message.ends_with("The solve still refuses it by name.)"));
        assert_eq!(found[1].message, "message of SNUBBER_DYNAMIC_ONLY_UNSUPPORTED", "a blocking finding keeps the solve's message");
        assert_eq!(found[5].message, "message of OPERATING_CONDITION_SET_MISSING");
    }

    /// Unattributable and global findings block even when unchanged and outside
    /// the scope: no refs, `model`, a ref that is not an entity id of the
    /// before-document (a collection, a document-level member, the project id,
    /// a condition key, an unknown id), a ref list mixing an entity with such a
    /// ref, and the judgement's parse refusal.
    #[test]
    fn unattributable_and_global_findings_block() {
        let before = before_document();
        let cases = [
            json!([]),
            json!(["model"]),
            json!(["load_cases"]),
            json!(["analysis_status"]),
            json!(["proj"]),
            json!(["OPE1"]),
            json!(["ghost"]),
            json!(["snub", "OPE1"]),
        ];
        let scope = BTreeSet::new();
        for refs in cases {
            let report = preview(json!([refs_diagnostic("SNUBBER_DYNAMIC_ONLY_UNSUPPORTED", refs.clone())]));
            let found = permit_carried(&before, classify(&before, &report), classify(&before, &report), &scope);
            assert_eq!(found.len(), 1);
            assert_eq!(found[0].severity, "blocking", "{refs}");
        }
        // Nested entity ids (a primitive load) are entities.
        let report = preview(json!([refs_diagnostic("SNUBBER_DYNAMIC_ONLY_UNSUPPORTED", json!(["HT:t", "snub"]))]));
        let found = permit_carried(&before, classify(&before, &report), classify(&before, &report), &scope);
        assert_eq!(found[0].severity, "info");
        let refused = json!({"resolved": false, "refusal": "unknown field `x`", "cases": [], "diagnostics": []});
        let found = permit_carried(&before, classify(&before, &refused), classify(&before, &refused), &scope);
        assert_eq!((found[0].code.as_str(), found[0].severity), (JUDGEMENT_REFUSED, "blocking"));
        // An entity of the candidate only is not the before-document's.
        let report = preview(json!([refs_diagnostic("SNUBBER_DYNAMIC_ONLY_UNSUPPORTED", json!(["cs:new"]))]));
        let found = permit_carried(&before, classify(&before, &report), classify(&before, &report), &scope);
        assert_eq!(found[0].severity, "blocking");
    }

    /// The scope of each operation family, on the union of both documents.
    #[test]
    fn downstream_combinations_join_the_scope_and_exact_route_codes_never_carry() {
        let model = json!({
            "load_cases": [{"id": "L"}, {"id": "SUS"}],
            "pipe_segments": [],
            "combinations": [
                {"id": "C", "basis": "result_state_subtraction", "minuend_id": "L", "subtrahend_id": "SUS"},
                {"id": "D", "basis": "srss", "terms": [{"combination": "C", "factor": 1.0}]},
                {"id": "E", "basis": "range_envelope", "operand_ids": ["SUS"]}
            ]
        });
        let scope = edit_scope(&model, &model, SET_USE, "L");
        assert!(scope.contains("C") && scope.contains("D"), "{scope:?}");
        assert!(!scope.contains("E"), "a combination naming only cases outside the scope stays outside: {scope:?}");
        assert!(is_route_or_admission("LOAD_COMBINATION_DIFFERENCE_BASIS_EXACT_UNSUPPORTED"));
        assert!(is_route_or_admission("SOME_FUTURE_EXACT_UNSUPPORTED"));
        assert!(!is_route_or_admission("SNUBBER_DYNAMIC_ONLY_UNSUPPORTED"));
    }

    #[test]
    fn the_edit_scope_follows_the_rule() {
        let doc = |pipes: Value, cases: Value| json!({"pipe_segments": pipes, "load_cases": cases, "condition_sets": [{"id": "X"}, {"id": "Y"}, {"id": "Z"}], "supports": [{"id": "snub"}]});
        let before = doc(
            json!([{"id": "P1", "condition_set_ref": "X"}, {"id": "P2", "condition_set_ref": "Y"}, {"id": "P3"}, {"id": "P4", "condition_set_ref": "Z"}]),
            json!([{"id": "HT", "condition_use": {"temperature": "OPE1"}}, {"id": "SUS"}, {"id": "W"}]),
        );
        let candidate = doc(
            json!([{"id": "P1", "condition_set_ref": "Y"}, {"id": "P2", "condition_set_ref": "Y"}, {"id": "P3"}, {"id": "P4", "condition_set_ref": "Z"}]),
            json!([{"id": "HT", "condition_use": {"temperature": "OPE1"}}, {"id": "SUS"}, {"id": "W", "condition_use": {"wind_exposure": true}}]),
        );
        let set = |items: &[&str]| items.iter().map(|item| item.to_string()).collect::<BTreeSet<String>>();
        for kind in [CREATE, UPDATE, DELETE] {
            assert_eq!(edit_scope(&before, &candidate, kind, "Z"), set(&["Z", "P4", "HT", "W"]), "{kind}");
            assert_eq!(edit_scope(&before, &candidate, kind, "N"), set(&["N", "HT", "W"]), "{kind}: an unreferenced set");
        }
        assert_eq!(edit_scope(&before, &candidate, ASSIGN, "P1"), set(&["P1", "X", "Y", "P2", "HT", "W"]));
        assert_eq!(edit_scope(&before, &candidate, ASSIGN, "P3"), set(&["P3", "HT", "W"]), "no set on either side");
        for kind in [SET_USE, CLEAR_USE, CREATE_LOAD_CASE] {
            assert_eq!(edit_scope(&before, &candidate, kind, "SUS"), set(&["SUS", "P1", "P2", "P4", "X", "Y", "Z"]), "{kind}");
        }
        let all = edit_scope(&before, &candidate, "set_field", "P3");
        assert!(all.contains("snub") && all.contains("HT") && all.contains("X"), "an unknown kind scopes everything");
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
