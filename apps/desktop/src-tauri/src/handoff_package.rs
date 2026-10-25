//! File > Save Handoff… (DEL-15-01 Design `canonical-model-export.md` §3,
//! W-6b; FR-HAND-001/002, PRD §16.1).
//!
//! The command writes a directory package `<name>.swbhandoff/` holding the
//! W-6a canonical model document (`model/model.json`) and a
//! `schemas/handoff_package.schema.json` instance (`handoff_package.json`,
//! compact JCS) whose `checksums` bind the model member by SHA-256; the model
//! content is referenced, never embedded in the manifest. Publication and the
//! verifier are `handoff_package_publish`.
//!
//! Before the chooser is shown, this module
//! - re-checks the route evidence of the governed route
//!   `DOTH-HANDOFF-PACKAGE-008` (local-private, explicit intent, lossless);
//! - admits the model text exactly as W-6a does
//!   (`model_document_save::admit_model_document_text`: the open cap, the
//!   canonical-text and hash check, the open path's document checks, the
//!   native shape check and the disposition screening of every
//!   `provenance_record`, `rule_pack_refs[]` entry and resolved `library_ref`;
//!   `protected_suspected`, `quarantined` and `rejected` refuse, while
//!   `private_only` and `unknown` pass and are recorded as they are);
//! - requires the basis state the webview names to be a saved state of the
//!   project (W-12 S1's store, read with `list_model_states`' query) whose
//!   model hash is the admitted text's SHA-256;
//! - builds every manifest field from its source (design §3 table): nothing
//!   is defaulted to look complete; what is not known is `TBD` or a named
//!   warning, and a unit that cannot be derived refuses
//!   `HANDOFF-UNIT-UNDERIVABLE`.
//!
//! Nothing is sent anywhere (F-PIP-1). The manifest's professional boundary is
//! the schema's fixed statement: it records review and reproducibility context
//! and claims nothing about acceptance, compliance or public use (F-PIP-2,
//! F-PIP-3).

use crate::command_ingress::Args;
use crate::handoff_package_publish::{self as publish, PublishHooks, Published};
use crate::model_document_save::{
    self, library_identity, matching_library_records, own_or_provenance, truthy_str, LibraryLookup,
    ModelDocumentSaveDiagnostic as Diagnostic, ModelDocumentSaveLocalFirst, ModelDocumentSaveScreening,
};
use crate::model_states::ModelStateSummary;
use crate::solve_jobs::SolveJobError;
use open_pipe_stress_operation_applier::{canonical_json, sha256_hex, validate_model_document};
use open_pipe_stress_units::{unit_by_symbol, Dimension, UnitId};
use serde::Serialize;
use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// The governed local-first route of this save.
pub const HANDOFF_PACKAGE_ROUTE_ID: &str = "DOTH-HANDOFF-PACKAGE-008";

pub const UNIT_UNDERIVABLE: &str = "HANDOFF-UNIT-UNDERIVABLE";
pub const BASIS_STATE_MISSING: &str = "HANDOFF-BASIS-STATE-MISSING";
pub const BASIS_STATE_MISMATCH: &str = "HANDOFF-BASIS-STATE-MISMATCH";
pub const STATE_STORE_UNAVAILABLE: &str = "HANDOFF-STATE-STORE-UNAVAILABLE";
pub const RULE_PACK_STORE_UNAVAILABLE: &str = "HANDOFF-RULE-PACK-STORE-UNAVAILABLE";
pub const ID_UNMAPPABLE: &str = "HANDOFF-ID-UNMAPPABLE";
pub const LABEL_INVALID: &str = "HANDOFF-LABEL-INVALID";
pub const PICKER_PATH_INVALID: &str = "HANDOFF-PICKER-PATH-INVALID";
pub const WORKER_FAILED: &str = "HANDOFF-WORKER-FAILED";
pub const DISPOSITION_MALFORMED: &str = "HANDOFF-DISPOSITION-MALFORMED";
/// The store check's codes (W-13 §3.5; `check_model_library_refs`, the Rule
/// Pack Manager's reference list).
pub const LIBRARY_REF_VERSION_CHANGED: &str = crate::MODEL_LIBRARY_REF_VERSION_CHANGED;
pub const RULE_PACK_REF_UNRESOLVED: &str = "MODEL-RULE-PACK-REF-UNRESOLVED";
/// The stored pack is not the one the reference names (#1602's code; `rulePackRefDiagnostics.ts`).
pub const RULE_PACK_REF_CHECKSUM_MISMATCH: &str = "MODEL-RULE-PACK-REF-CHECKSUM-MISMATCH";

/// The unit catalogue every unit identifier is taken from (`get_unit_catalog`).
pub const UNIT_SYSTEM_REF: &str = "unit-system:dec-018-si-dual-display";
const MAX_LABEL_CHARS: usize = 200;
const SOFTWARE_ATTESTATION: &str = "software-generated reference record; no professional claim";
const USER_ATTESTATION: &str = "user-entered project data; no professional claim";
const USER_LICENSE: &str = "user_private_project_data";
const LOCAL_USER: &str = "local user";

/// `{canonicalText, claimedModelHash, screening, localFirstEvidence, basisStateId, packageLabel}`.
#[derive(Debug)]
pub struct HandoffSaveRequest {
    pub canonical_text: String,
    pub claimed_model_hash: String,
    pub screening: ModelDocumentSaveScreening,
    pub local_first: ModelDocumentSaveLocalFirst,
    pub basis_state_id: String,
    pub package_label: String,
}

/// Decodes the command's raw-body arguments (strings and structs of scalars, moved).
pub(crate) fn save_handoff_package_args(args: &mut Args) -> Result<HandoffSaveRequest, SolveJobError> {
    Ok(HandoffSaveRequest {
        canonical_text: args.decode("canonicalText")?,
        claimed_model_hash: args.decode("claimedModelHash")?,
        screening: args.decode("screening")?,
        local_first: args.decode("localFirstEvidence")?,
        basis_state_id: args.decode("basisStateId")?,
        package_label: args.decode("packageLabel")?,
    })
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum HandoffSaveOutcome {
    /// The user dismissed the chooser. Not an error; nothing was written.
    Cancelled,
    Saved {
        package_name: String,
        /// `sha256:<hex>` of `model/model.json`: the session's model hash.
        model_hash: String,
        /// `sha256:<hex>`: the manifest's export hash.
        export_hash: String,
        basis_state_id: String,
        replaced_existing: bool,
        durability: String,
        /// Non-blocking findings (store findings, leftover staging folders).
        warnings: Vec<Diagnostic>,
    },
    Refused {
        diagnostics: Vec<Diagnostic>,
    },
}

impl HandoffSaveOutcome {
    pub fn refused_one(code: &str, message: impl Into<String>) -> Self {
        Self::Refused { diagnostics: vec![diagnostic(code, "", message)] }
    }
}

fn diagnostic(code: &str, path: &str, message: impl Into<String>) -> Diagnostic {
    Diagnostic { code: code.to_string(), path: path.to_string(), message: message.into() }
}

/// The route evidence re-check (`model_document_save`'s rule, this route).
pub fn validate_handoff_evidence(screening: &ModelDocumentSaveScreening, local_first: &ModelDocumentSaveLocalFirst) -> Result<(), Diagnostic> {
    model_document_save::validate_local_private_route_evidence(HANDOFF_PACKAGE_ROUTE_ID, screening, local_first)
}

/// A project's saved state by project id and state id (production: the metered
/// `model_states::list_states_in_store_with`), or `None` when it has none.
pub type StateLookup<'a> = dyn FnMut(&str, &str) -> Result<Option<ModelStateSummary>, String> + 'a;
/// The stored rule-pack document with a `rule_pack_id` under the model's project
/// (at most one), as the solve's rule-pack resolution opens it (#1602).
pub type RulePackLookup<'a> = dyn FnMut(&str, &str) -> Result<Vec<Value>, String> + 'a;

// ---------------------------------------------------------------------------
// Vocabulary helpers.
// ---------------------------------------------------------------------------

fn reference(object_type: &str, reference: &str) -> Value {
    json!({ "object_type": object_type, "ref": reference })
}

/// An entity id as the handoff `Reference` holds it (design §3 "Reference
/// mapping"): `{Model, "<project id>#<entity id>"}`.
fn model_qualified(project_id: &str, entity_id: &str) -> Value {
    reference("Model", &format!("{project_id}#{entity_id}"))
}

/// The handoff schema's `Id`: `^[A-Za-z][A-Za-z0-9_.:-]*$`.
fn is_handoff_id(text: &str) -> bool {
    let mut characters = text.chars();
    characters.next().is_some_and(|first| first.is_ascii_alphabetic())
        && characters.all(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '.' | ':' | '-'))
}

/// A non-blank string member, as written.
fn text<'a>(object: Option<&'a Map<String, Value>>, key: &str) -> Option<&'a str> {
    object?.get(key).and_then(Value::as_str).filter(|value| !value.trim().is_empty())
}

const REDISTRIBUTION_STATUSES: [&str; 7] = [
    "public_permissive",
    "private_only",
    "unknown",
    "protected_suspected",
    "commercial_tool_reference_only",
    "invented_non_engineering_example",
    "TBD",
];

/// A source disposition in the handoff `RedistributionStatus` vocabulary;
/// anything else (or nothing) is `TBD`.
fn handoff_redistribution(value: Option<&str>) -> &str {
    match value {
        Some(value) if REDISTRIBUTION_STATUSES.contains(&value) => value,
        _ => "TBD",
    }
}

/// A source-data `review_status` (W-13 / library import vocabulary) as a
/// handoff `ReviewClassification`, by fixed table. `accepted` records the
/// source review the record states; it is never professional acceptance.
fn review_classification(review_status: Option<&str>) -> &'static str {
    match review_status {
        Some("pending") => "unreviewed",
        Some("accepted") => "human_reviewed_reference_only",
        Some("quarantined" | "rejected") => "quarantined",
        _ => "TBD",
    }
}

/// A rule pack's `metadata.lifecycle_status` as a `ReviewClassification`, by
/// fixed table.
fn lifecycle_review_classification(lifecycle_status: Option<&str>) -> &'static str {
    match lifecycle_status {
        Some("draft") => "unreviewed",
        Some("review_pending") => "human_review_required",
        Some("accepted_private" | "accepted_public_example") => "human_reviewed_reference_only",
        Some("rejected" | "quarantined") => "quarantined",
        _ => "TBD",
    }
}

/// The privacy classification of a referenced record (review round 3,
/// should-fix 2): the least permissive of what its redistribution disposition
/// allows (public metadata only for `public_permissive`, an invented public
/// example only for `invented_non_engineering_example`) and of every
/// `privacy_class` / `privacy_classification` the covered objects state.
/// A privacy value outside the schemas' public and invented-example values is
/// private, so nothing a covered object marks private, protected, redacted,
/// unknown or malformed is labelled public.
fn reference_privacy(redistribution: &str, privacy_values: &[&str], private: &'static str) -> &'static str {
    // Explicit allow-lists from the schemas' `privacy_class` and
    // `privacy_classification` enums (review round 4, note 3); every other
    // value, malformed ones included, is private.
    const PUBLIC: [&str; 5] = ["public", "public_metadata", "public_permissive", "public_permissive_reviewed", "public_schema_only"];
    const INVENTED_EXAMPLE: [&str; 2] = ["invented_public_example", "public_invented_example"];
    fn rank(value: &str) -> u8 {
        if PUBLIC.contains(&value) {
            0
        } else if INVENTED_EXAMPLE.contains(&value) {
            1
        } else {
            2
        }
    }
    let allowed = match redistribution {
        "public_permissive" => 0,
        "invented_non_engineering_example" => 1,
        _ => 2,
    };
    match privacy_values.iter().map(|value| rank(value)).fold(allowed, u8::max) {
        0 => "public_metadata",
        1 => "invented_public_example",
        _ => private,
    }
}

/// Every `privacy_class` and `privacy_classification` string at any depth of `value`.
/// A present member whose value is not a string (`null`, a boolean, a number,
/// an array, an object) is recorded as the empty string, which no allow-list
/// holds, so it counts as private (review round 5, should-fix 3).
fn privacy_values<'a>(value: &'a Value, out: &mut Vec<&'a str>) {
    match value {
        Value::Object(object) => {
            for (key, child) in object {
                if key == "privacy_class" || key == "privacy_classification" {
                    out.push(child.as_str().unwrap_or(""));
                }
                privacy_values(child, out);
            }
        }
        Value::Array(items) => items.iter().for_each(|item| privacy_values(item, out)),
        _ => {}
    }
}

/// Redistribution dispositions from most to least permissive; anything
/// unlisted ranks with `TBD`.
const REDISTRIBUTION_ORDER: [&str; 7] = [
    "public_permissive",
    "invented_non_engineering_example",
    "commercial_tool_reference_only",
    "unknown",
    "TBD",
    "private_only",
    "protected_suspected",
];
/// Review classifications from most to least settled.
const REVIEW_ORDER: [&str; 6] = ["human_reviewed_reference_only", "machine_checked", "human_review_required", "unreviewed", "TBD", "quarantined"];

fn rank(order: &[&str], value: &str) -> usize {
    order.iter().position(|item| *item == value).unwrap_or_else(|| order.iter().position(|item| *item == "TBD").unwrap())
}

/// The least permissive of the redistribution dispositions a record includes
/// (design §3 `redistribution_classification`).
fn least_permissive<'a>(values: impl IntoIterator<Item = &'a str>) -> &'a str {
    values.into_iter().max_by_key(|value| rank(&REDISTRIBUTION_ORDER, value)).unwrap_or("TBD")
}

/// The least settled of the review classifications a record includes.
fn least_reviewed<'a>(values: impl IntoIterator<Item = &'a str>) -> &'a str {
    values.into_iter().max_by_key(|value| rank(&REVIEW_ORDER, value)).unwrap_or("TBD")
}

/// The disposition members the structural screen judges, with the values that
/// block: W-6a's policy (`redistribution_status` `protected_suspected` or
/// `rejected`, `review_status` `quarantined` or `rejected`), the privacy and
/// classification members' `protected_suspected` (the webview's rule), and a
/// rule pack's `lifecycle_status` `quarantined` or `rejected`.
const SCREENED_MEMBERS: [(&str, &[&str]); 6] = [
    ("redistribution_status", &["protected_suspected", "rejected"]),
    ("review_status", &["quarantined", "rejected"]),
    ("privacy_class", &["protected_suspected", "quarantined", "rejected"]),
    ("privacy_classification", &["protected_suspected", "quarantined", "rejected"]),
    ("classification", &["protected_suspected", "quarantined", "rejected"]),
    ("lifecycle_status", &["quarantined", "rejected"]),
];

/// One RFC 6901 reference token.
fn pointer_token(key: &str) -> String {
    key.replace('~', "~0").replace('/', "~1")
}

/// The structural screen of one document resolved from the store (review
/// round 2, finding 2): every object at every depth (a wrapper, its records,
/// array elements, every `provenance` and `provenance_record` beneath them) is
/// judged by its own [`SCREENED_MEMBERS`]; nothing is inherited and no field
/// hides another object's. Returned as `<JSON pointer> <value>`, in document
/// order; empty when nothing blocks. The whole document is screened before
/// any of it is mapped into the manifest.
pub(crate) fn screen_document(document: &Value) -> Vec<String> {
    fn walk(value: &Value, pointer: &str, found: &mut Vec<String>) {
        match value {
            Value::Object(object) => {
                for (key, child) in object {
                    let at = format!("{pointer}/{}", pointer_token(key));
                    if let (Some((_, blocking)), Some(status)) = (SCREENED_MEMBERS.iter().find(|(member, _)| member == key), child.as_str()) {
                        if blocking.contains(&status) {
                            found.push(format!("{at} {status}"));
                        }
                    }
                    walk(child, &at, found);
                }
            }
            Value::Array(items) => {
                for (index, item) in items.iter().enumerate() {
                    walk(item, &format!("{pointer}/{index}"), found);
                }
            }
            _ => {}
        }
    }
    let mut found = Vec::new();
    walk(document, "", &mut found);
    found
}

/// The disposition members of a stored document whose value is an array or an
/// object (review round 6 note 4, narrowed in round 6b by the owner's
/// decision): either could hide a blocking value, so the save refuses them as
/// malformed rather than guess. A rule pack's `classification` object is
/// exempt, since its own members are screened in turn. `null`, booleans and
/// numbers are not refused (the library import stores them); they count as
/// no public disposition and, for privacy members, as private. Returned as
/// `<JSON pointer> is <JSON value kind>`.
pub(crate) fn malformed_dispositions(document: &Value) -> Vec<String> {
    fn kind(value: &Value) -> &'static str {
        match value {
            Value::Null => "null",
            Value::Bool(_) => "a boolean",
            Value::Number(_) => "a number",
            Value::String(_) => "a string",
            Value::Array(_) => "an array",
            Value::Object(_) => "an object",
        }
    }
    fn walk(value: &Value, pointer: &str, found: &mut Vec<String>) {
        match value {
            Value::Object(object) => {
                for (key, child) in object {
                    let at = format!("{pointer}/{}", pointer_token(key));
                    let screened = SCREENED_MEMBERS.iter().any(|(member, _)| member == key);
                    if screened && (child.is_array() || child.is_object()) && !(key == "classification" && child.is_object()) {
                        found.push(format!("{at} is {}", kind(child)));
                    }
                    walk(child, &at, found);
                }
            }
            Value::Array(items) => {
                for (index, item) in items.iter().enumerate() {
                    walk(item, &format!("{pointer}/{index}"), found);
                }
            }
            _ => {}
        }
    }
    let mut found = Vec::new();
    walk(document, "", &mut found);
    found
}

fn malformed_record(path: &str, record: &str, what: &str, found: &[String]) -> Diagnostic {
    diagnostic(
        DISPOSITION_MALFORMED,
        path,
        format!("{record} ({what}) has disposition members that are lists or objects, not text ({}), so its disposition cannot be judged; nothing was saved. Correct the stored record first.", found.join(", ")),
    )
}

fn blocked_record(path: &str, record: &str, what: &str, found: &[String]) -> Diagnostic {
    diagnostic(
        model_document_save::PROTECTED_CONTENT_BLOCKED,
        path,
        format!("{record} ({what}: {}) blocks the save; request human review of the record's disposition first.", found.join(", ")),
    )
}

/// Every string `member` at any depth of `value`.
fn members_at_any_depth<'a>(value: &'a Value, member: &str, out: &mut Vec<&'a str>) {
    match value {
        Value::Object(object) => {
            for (key, child) in object {
                if key == member {
                    // A present member that is not text (null, a boolean, a
                    // number; arrays and objects are refused before) counts as
                    // the empty string, which maps to no public disposition
                    // (review round 6b).
                    out.push(child.as_str().unwrap_or(""));
                }
                members_at_any_depth(child, member, out);
            }
        }
        Value::Array(items) => items.iter().for_each(|item| members_at_any_depth(item, member, out)),
        _ => {}
    }
}

/// The dispositions of the objects a reference covers (review round 2,
/// finding 4): each covered object's own (or else its provenance's) and every
/// one at any depth beneath it, so a record is never classified by its
/// wrapper's. Returned as (redistribution dispositions, review classifications).
fn covered_dispositions<'a>(covered: &[&'a Map<String, Value>]) -> (Vec<&'a str>, Vec<&'static str>) {
    let mut redistribution = Vec::new();
    let mut review = Vec::new();
    for object in covered {
        redistribution.push(handoff_redistribution(own_or_provenance(object, "redistribution_status")));
        review.push(review_classification(own_or_provenance(object, "review_status")));
        let mut found = Vec::new();
        let mut reviews = Vec::new();
        for (key, child) in object.iter() {
            match key.as_str() {
                "redistribution_status" => found.push(child.as_str().unwrap_or("")),
                "review_status" => reviews.push(child.as_str().unwrap_or("")),
                _ => {}
            }
            members_at_any_depth(child, "redistribution_status", &mut found);
            members_at_any_depth(child, "review_status", &mut reviews);
        }
        redistribution.extend(found.into_iter().map(|status| handoff_redistribution(Some(status))));
        review.extend(reviews.into_iter().map(|status| review_classification(Some(status))));
    }
    (redistribution, review)
}

#[allow(clippy::too_many_arguments)]
fn provenance_record(
    source_name: &str,
    source_location: &str,
    source_license: &str,
    contributor: &str,
    attestation: &str,
    redistribution: &str,
    review: &str,
    privacy: &str,
) -> Value {
    json!({
        "source_name": source_name,
        "source_location": source_location,
        "source_license": source_license,
        "contributor": contributor,
        "contributor_attestation": attestation,
        "redistribution_status": redistribution,
        "review_classification": review,
        "privacy_classification": privacy
    })
}

/// A record this producer derived from the user's private model document.
fn derived_provenance(source_location: &str) -> Value {
    provenance_record(
        "SWBPIPE desktop",
        source_location,
        USER_LICENSE,
        LOCAL_USER,
        SOFTWARE_ATTESTATION,
        "private_only",
        "unreviewed",
        "private_project_data",
    )
}

/// A stored library's or rule pack's provenance mapped to the eight handoff
/// fields: W-13's `contributor_certification` is `contributor_attestation`;
/// a missing field is `TBD`.
fn mapped_provenance(source: Option<&Map<String, Value>>, fallback_name: &str, privacy: &str) -> Value {
    provenance_record(
        text(source, "source_name").unwrap_or(fallback_name),
        text(source, "source_location").unwrap_or("TBD"),
        text(source, "source_license").unwrap_or("TBD"),
        text(source, "contributor").unwrap_or("TBD"),
        text(source, "contributor_certification").or_else(|| text(source, "contributor_attestation")).unwrap_or("TBD"),
        handoff_redistribution(text(source, "redistribution_status")),
        review_classification(text(source, "review_status")),
        privacy,
    )
}

fn checksum(object_type: &str, reference_id: &str, scope: &str, canonicalization: &str, value: &str) -> Value {
    json!({
        "algorithm": "sha256",
        "canonicalization": canonicalization,
        "payload_ref": reference(object_type, reference_id),
        "payload_scope": scope,
        "value": value
    })
}

/// A diagnostic class from its code family, by fixed table.
fn diagnostic_class(code: &str) -> &'static str {
    if code.starts_with("MODEL-ASSUMPTION-") {
        "ASSUMPTION_WARNING"
    } else if code.starts_with("MODEL-PROVENANCE-") || code.starts_with("MODEL-LIBRARY-REF-") || code.starts_with("MODEL-RULE-PACK-REF-") {
        "PROVENANCE_WARNING"
    } else if code.contains("UNIT") {
        "UNIT_WARNING"
    } else if code.contains("PRIVACY") || code.contains("PROTECTED") {
        "PRIVACY_WARNING"
    } else if code.contains("HASH") {
        "HASH_WARNING"
    } else {
        "TBD"
    }
}

const DIAGNOSTIC_CLASSES: [&str; 14] = [
    "SOLVE_BLOCKING",
    "RULE_CHECK_BLOCKING",
    "PROVENANCE_WARNING",
    "ASSUMPTION_WARNING",
    "NONLINEAR_WARNING",
    "IP_BOUNDARY_WARNING",
    "UNIT_WARNING",
    "LOCAL_HANDOFF_WARNING",
    "EXPORT_BLOCKING",
    "TARGET_MAPPING_WARNING",
    "UNSUPPORTED_BEHAVIOR_WARNING",
    "PRIVACY_WARNING",
    "HASH_WARNING",
    "TBD",
];

fn diagnostic_remediation(code: &str) -> &'static str {
    match diagnostic_class(code) {
        "ASSUMPTION_WARNING" => "Update or resolve the record so that it names entities the model still has.",
        "PROVENANCE_WARNING" => "Review the named source record or reference in the model and the local store before relying on this package.",
        "UNIT_WARNING" => "Review the named units in the model.",
        _ => "Review the finding in the model document before relying on this package.",
    }
}

fn handoff_severity(severity: &str) -> &'static str {
    match severity {
        "blocking" | "error" => "blocking",
        "warning" => "warning",
        _ => "info",
    }
}

// ---------------------------------------------------------------------------
// The prepared handoff: every manifest part except the package identity.
// ---------------------------------------------------------------------------

/// The saved state the package is based on.
#[derive(Debug, Clone, PartialEq)]
pub struct BasisState {
    pub state_id: String,
    pub state_kind: String,
    /// Bare hex of the state's record hash (sorted-compact JSON, W-12 S1).
    pub record_hash_hex: String,
}

/// An admitted handoff: the model bytes and every manifest part derived from
/// them and the local store, before the package is named.
#[derive(Debug, Clone)]
pub struct PreparedHandoff {
    pub model_text: String,
    pub model_hex: String,
    pub project_id: String,
    pub package_label: String,
    pub suggested_package_name: String,
    pub basis: BasisState,
    pub units_manifest: Value,
    pub entity_ids: Value,
    pub library_refs: Vec<Value>,
    pub rule_pack_refs: Vec<Value>,
    pub unresolved_assumptions: Vec<Value>,
    pub warnings: Vec<Value>,
    pub diagnostics: Vec<Value>,
    /// The store findings, for the status line.
    pub save_warnings: Vec<Diagnostic>,
}

fn admit_label(label: &str) -> Result<String, Vec<Diagnostic>> {
    let trimmed = label.trim();
    if trimmed.is_empty() || trimmed.chars().count() > MAX_LABEL_CHARS || trimmed.chars().any(char::is_control) {
        return Err(vec![diagnostic(LABEL_INVALID, "", format!("The package label must be 1 to {MAX_LABEL_CHARS} characters of text."))]);
    }
    Ok(trimmed.to_string())
}

/// The saved state `state_id` of `project_id` in the store at `path`, read
/// within `memory` with the state list's metered reader (W-12 S2); the list's
/// reservations are released once the one summary is taken from it.
pub(crate) fn stored_state(
    path: &Path,
    project_id: &str,
    state_id: &str,
    memory: &dyn crate::stored_read::ReadMemory,
) -> Result<Option<ModelStateSummary>, String> {
    use crate::model_states::ListedState;
    let listed = crate::model_states::list_states_in_store_with(path, project_id, memory)?;
    for entry in listed.get() {
        match entry {
            ListedState::Readable(summary) if summary.state_id == state_id => return Ok(Some(summary.clone())),
            // The basis state's row cannot be read (W-12 #1602 lists such a
            // row by its id and refusal): the handoff refuses with that
            // refusal, never as a missing state.
            ListedState::Unreadable { unreadable } if unreadable.id == state_id => return Err(unreadable.refusal.clone()),
            _ => {}
        }
    }
    Ok(None)
}

fn basis_state(states: &mut StateLookup<'_>, project_id: &str, state_id: &str, model_hex: &str) -> Result<BasisState, Vec<Diagnostic>> {
    let found = states(project_id, state_id).map_err(|error| {
        vec![diagnostic(STATE_STORE_UNAVAILABLE, "", format!("The saved states of `{project_id}` could not be read ({error}); nothing was saved."))]
    })?;
    let Some(state) = found.filter(|state| state.state_id == state_id) else {
        return Err(vec![diagnostic(
            BASIS_STATE_MISSING,
            "",
            format!("Project `{project_id}` has no saved state `{state_id}`; the handoff needs its saved basis state."),
        )]);
    };
    if state.model_hash != format!("sha256:{model_hex}") {
        return Err(vec![diagnostic(
            BASIS_STATE_MISMATCH,
            "",
            format!("State `{state_id}` holds model {}, not this model (sha256:{model_hex}); nothing was saved.", state.model_hash),
        )]);
    }
    Ok(BasisState { state_id: state.state_id, state_kind: state.state_kind, record_hash_hex: state.record_hash })
}

/// One `project.units` entry as its catalogue unit.
fn catalogue_unit(units: Option<&Map<String, Value>>, key: &str, dimension: Dimension) -> Result<UnitId, Diagnostic> {
    let path = format!("project.units.{key}");
    let symbol = units
        .and_then(|units| units.get(key))
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|symbol| !symbol.is_empty())
        .ok_or_else(|| diagnostic(UNIT_UNDERIVABLE, &path, format!("project.units has no `{key}` unit, so the handoff's units cannot be derived; set it in the project units.")))?;
    unit_by_symbol(symbol, dimension).map_err(|_| {
        diagnostic(UNIT_UNDERIVABLE, &path, format!("`{symbol}` is not a {} unit of the unit catalogue ({UNIT_SYSTEM_REF}).", dimension.as_str()))
    })
}

/// The units derivation table (design §3): identifiers from the unit
/// catalogue; `moment` when present, else the catalogue's `<force>*<length>`
/// unit, else a refusal.
pub fn units_manifest(document: &Value) -> Result<Value, Vec<Diagnostic>> {
    let units = document.pointer("/project/units").and_then(Value::as_object);
    let mut refusals = Vec::new();
    let mut take = |key: &str, dimension: Dimension| match catalogue_unit(units, key, dimension) {
        Ok(unit) => Some(unit),
        Err(refusal) => {
            refusals.push(refusal);
            None
        }
    };
    let length = take("length", Dimension::Length);
    let angle = take("angle", Dimension::Angle);
    let force = take("force", Dimension::Force);
    let stress = take("stress", Dimension::Stress);
    let temperature = take("temperature", Dimension::Temperature);
    let moment_present = units.and_then(|units| units.get("moment")).is_some();
    let moment = if moment_present {
        take("moment", Dimension::Moment)
    } else {
        match (force, length) {
            (Some(force), Some(length)) => {
                let composed = format!("{}*{}", force.definition().symbol, length.definition().symbol);
                match unit_by_symbol(&composed, Dimension::Moment) {
                    Ok(unit) => Some(unit),
                    Err(_) => {
                        refusals.push(diagnostic(
                            UNIT_UNDERIVABLE,
                            "project.units.moment",
                            format!("project.units has no `moment` unit and the unit catalogue has no `{composed}` moment unit for the force and length pair; set the moment unit."),
                        ));
                        None
                    }
                }
            }
            _ => None,
        }
    };
    let (Some(length), Some(angle), Some(force), Some(stress), Some(temperature), Some(moment)) = (length, angle, force, stress, temperature, moment)
    else {
        return Err(refusals);
    };
    let mut block = json!({
        "unit_system_ref": reference("UnitSystem", UNIT_SYSTEM_REF),
        "units_schema": "schemas/units.schema.yaml",
        "dimension_basis": "schemas/units.schema.yaml",
        "coordinate_unit": length.catalog_id(),
        "displacement_unit": length.catalog_id(),
        "rotation_unit": angle.catalog_id(),
        "force_unit": force.catalog_id(),
        "moment_unit": moment.catalog_id(),
        "stress_unit": stress.catalog_id(),
        "temperature_unit": temperature.catalog_id(),
        "provenance": derived_provenance("model document project.units, identifiers from the unit catalogue")
    });
    let hash = sha256_hex(&canonical_json(&block));
    block["hash_refs"] = json!([checksum("UnitSystem", UNIT_SYSTEM_REF, "units_manifest", "JCS", &hash)]);
    Ok(block)
}

fn members<'a>(document: &'a Value, member: &str) -> impl Iterator<Item = (usize, &'a Map<String, Value>)> {
    document.get(member).and_then(Value::as_array).into_iter().flatten().enumerate().filter_map(|(index, item)| item.as_object().map(|item| (index, item)))
}

/// `entity_ids` (design §3): sorted ids in the `EntityIdManifest` slots, and
/// the families without a slot as model-qualified `mapping_id_refs`.
pub fn entity_ids(document: &Value, project_id: &str, state_id: &str) -> Result<Value, Vec<Diagnostic>> {
    let mut refusals = Vec::new();
    let mut slot = |families: &[&str]| -> Vec<String> {
        let mut ids = BTreeSet::new();
        for family in families {
            for (index, item) in members(document, family) {
                let Some(id) = item.get("id").and_then(Value::as_str) else { continue };
                if is_handoff_id(id) {
                    ids.insert(id.to_string());
                } else {
                    refusals.push(diagnostic(
                        ID_UNMAPPABLE,
                        &format!("{family}[{index}].id"),
                        format!("`{id}` is not a handoff identifier ([A-Za-z][A-Za-z0-9_.:-]*); rename the entity before saving a handoff."),
                    ));
                }
            }
        }
        ids.into_iter().collect()
    };
    let node_ids = slot(&["nodes"]);
    let component_ids = slot(&["components"]);
    let element_ids = slot(&["pipe_segments"]);
    let load_case_ids = slot(&["load_cases", "combinations"]);
    if !refusals.is_empty() {
        return Err(refusals);
    }
    let mut mapped = BTreeSet::new();
    for (family, key) in [
        ("supports", "id"),
        ("materials", "id"),
        ("sections", "id"),
        ("lines", "id"),
        ("constraints", "constraint_id"),
        ("equipment_interfaces", "id"),
        ("reference_configurations", "id"),
    ] {
        for (_, item) in members(document, family) {
            if let Some(id) = item.get(key).and_then(Value::as_str) {
                mapped.insert(format!("{project_id}#{id}"));
            }
        }
    }
    Ok(json!({
        "node_ids": node_ids,
        "component_ids": component_ids,
        "element_ids": element_ids,
        "load_case_ids": load_case_ids,
        "model_state_ids": [state_id],
        "station_ids": [],
        "result_ids": [],
        "analysis_run_ids": [],
        "mapping_id_refs": mapped.into_iter().map(|entry| reference("Model", &entry)).collect::<Vec<_>>()
    }))
}

/// `unresolved_assumptions` (design §3): the document's `unresolved` records;
/// `kind: open_issue` folds into the statement prefix "Open issue: "; `tags`,
/// `notes` and `resolution_note` are dropped. `affected_refs`: the project id
/// is `{Project}`, an assumption id `{Assumption}`, any other entity id
/// (a deleted subject included) model-qualified; none names the project.
pub fn unresolved_assumptions(document: &Value, project_id: &str) -> Result<Vec<Value>, Vec<Diagnostic>> {
    let assumption_ids: BTreeSet<&str> =
        members(document, "unresolved_assumptions").filter_map(|(_, record)| record.get("assumption_id").and_then(Value::as_str)).collect();
    let mut records = Vec::new();
    let mut refusals = Vec::new();
    for (index, record) in members(document, "unresolved_assumptions") {
        if record.get("status").and_then(Value::as_str) != Some("unresolved") {
            continue;
        }
        let id = record.get("assumption_id").and_then(Value::as_str).unwrap_or("");
        if !is_handoff_id(id) {
            refusals.push(diagnostic(
                ID_UNMAPPABLE,
                &format!("unresolved_assumptions[{index}].assumption_id"),
                format!("`{id}` is not a handoff identifier ([A-Za-z][A-Za-z0-9_.:-]*); rename the record before saving a handoff."),
            ));
            continue;
        }
        let statement = record.get("statement").and_then(Value::as_str).unwrap_or("");
        let statement = if record.get("kind").and_then(Value::as_str) == Some("open_issue") { format!("Open issue: {statement}") } else { statement.to_string() };
        let mut affected: Vec<Value> = record
            .get("affected_refs")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .map(|entity| {
                if entity == project_id {
                    reference("Project", project_id)
                } else if assumption_ids.contains(entity) {
                    reference("Assumption", entity)
                } else {
                    model_qualified(project_id, entity)
                }
            })
            .collect();
        if affected.is_empty() {
            affected.push(reference("Project", project_id));
        }
        records.push(json!({
            "assumption_id": id,
            "statement": statement,
            "status": "unresolved",
            "affected_refs": affected,
            "provenance": provenance_record(
                "SWBPIPE model document unresolved_assumptions",
                &format!("model document unresolved_assumptions/{id}"),
                USER_LICENSE,
                LOCAL_USER,
                USER_ATTESTATION,
                "private_only",
                "unreviewed",
                "private_project_data"
            )
        }));
    }
    if refusals.is_empty() { Ok(records) } else { Err(refusals) }
}

/// One `provenance_record.library_ref` of the document.
struct LibraryReference {
    kind: String,
    id: String,
    version: Option<String>,
    record_id: Option<String>,
    path: String,
}

fn collect_library_refs(value: &Value, path: &str, out: &mut Vec<LibraryReference>) {
    match value {
        Value::Array(items) => {
            for (index, item) in items.iter().enumerate() {
                collect_library_refs(item, &format!("{path}[{index}]"), out);
            }
        }
        Value::Object(object) => {
            for (key, child) in object {
                let at = if path.is_empty() { key.clone() } else { format!("{path}.{key}") };
                if key == "provenance_record" {
                    if let Some(reference) = child.get("library_ref").and_then(Value::as_object) {
                        // Identities as the store's open path normalizes them (W-6a).
                        let identity = |key: &str| truthy_str(reference.get(key)).and_then(library_identity).map(str::to_string);
                        if let (Some(kind), Some(id)) = (identity("library_kind"), identity("library_id")) {
                            out.push(LibraryReference {
                                kind,
                                id,
                                version: truthy_str(reference.get("library_version")).map(str::to_string),
                                record_id: identity("record_id"),
                                path: format!("{at}.library_ref"),
                            });
                        }
                    }
                }
                collect_library_refs(child, &at, out);
            }
        }
        _ => {}
    }
}

/// `library_refs` (design §3): one per distinct `provenance_record.library_ref`
/// (kind, id, recorded version), resolved against the local store with W-6a's
/// identity and record-matching rules. A recorded version that differs from a
/// matching stored record is `MODEL-LIBRARY-REF-VERSION-CHANGED` (W-13 §3.5).
fn library_refs(document: &Value, project_id: &str, lookup: &mut LibraryLookup<'_>) -> Result<(Vec<Value>, Vec<Diagnostic>), Vec<Diagnostic>> {
    let mut found = Vec::new();
    collect_library_refs(document, "", &mut found);
    let mut distinct: BTreeMap<(String, String, Option<String>), &LibraryReference> = BTreeMap::new();
    for reference in &found {
        distinct.entry((reference.kind.clone(), reference.id.clone(), reference.version.clone())).or_insert(reference);
    }
    let mut stored_libraries: BTreeMap<(String, String), Option<Value>> = BTreeMap::new();
    for reference in &found {
        let key = (reference.kind.clone(), reference.id.clone());
        if stored_libraries.contains_key(&key) {
            continue;
        }
        let stored = lookup(project_id, &reference.kind, &reference.id).map_err(|error| {
            vec![diagnostic(
                model_document_save::LIBRARY_STORE_UNAVAILABLE,
                &reference.path,
                format!("The local library store could not be read to resolve {}:{} ({error}); nothing was saved.", reference.kind, reference.id),
            )]
        })?;
        stored_libraries.insert(key, stored);
    }
    // Every stored library document is screened whole before anything of it
    // is mapped (review round 2, finding 2).
    let mut refusals = Vec::new();
    for ((kind, id), stored) in &stored_libraries {
        let Some(stored) = stored else { continue };
        let found_blocking = screen_document(stored);
        let found_malformed = malformed_dispositions(stored);
        let path = found.iter().find(|reference| &reference.kind == kind && &reference.id == id).map(|reference| reference.path.as_str()).unwrap_or("");
        if !found_blocking.is_empty() {
            refusals.push(blocked_record(path, &format!("{kind}:{id}"), "stored library", &found_blocking));
        }
        if !found_malformed.is_empty() {
            refusals.push(malformed_record(path, &format!("{kind}:{id}"), "stored library", &found_malformed));
        }
    }
    if !refusals.is_empty() {
        return Err(refusals);
    }
    let mut warnings = Vec::new();
    for reference in &found {
        let (Some(recorded), Some(record_id)) = (&reference.version, &reference.record_id) else { continue };
        let Some(Some(stored)) = stored_libraries.get(&(reference.kind.clone(), reference.id.clone())) else { continue };
        let changed = matching_library_records(&reference.kind, stored, record_id)
            .into_iter()
            .map(|record| crate::library_record_version(&Value::Object(record.clone())))
            .find(|stored_version| stored_version != recorded);
        if let Some(stored_version) = changed {
            warnings.push(diagnostic(
                LIBRARY_REF_VERSION_CHANGED,
                &reference.path,
                format!("The stored record `{record_id}` of {} library `{}` is {stored_version}, not the recorded {recorded}; the model's values and recorded version stand.", reference.kind, reference.id),
            ));
        }
    }
    let mut refs = Vec::new();
    for ((kind, id, version), _) in distinct {
        let stored = stored_libraries.get(&(kind.clone(), id.clone())).cloned().flatten();
        let library = stored.as_ref().and_then(|stored| stored.get(format!("{kind}_library"))).and_then(Value::as_object);
        let provenance_source = library.and_then(|library| library.get("provenance")).and_then(Value::as_object);
        let metadata = json!({
            "library_kind": kind,
            "library_id": id,
            "library_version": version,
            "document_sha256": stored.as_ref().map(|stored| sha256_hex(&canonical_json(stored)))
        });
        // The objects the reference covers: the library wrapper and the
        // records its references name (every record, for a reference that
        // names none). Its classification is the least permissive of theirs.
        let mut covered: Vec<&Map<String, Value>> = library.into_iter().collect();
        if let Some(stored) = &stored {
            for reference in found.iter().filter(|reference| reference.kind == kind && reference.id == id && reference.version == version) {
                match &reference.record_id {
                    Some(record_id) => covered.extend(matching_library_records(&kind, stored, record_id)),
                    None => covered.extend(
                        crate::library_records_key(&kind)
                            .and_then(|(records_key, _)| stored.get(records_key))
                            .and_then(Value::as_array)
                            .into_iter()
                            .flatten()
                            .filter_map(Value::as_object),
                    ),
                }
            }
        }
        let (redistributions, reviews) = covered_dispositions(&covered);
        let redistribution = least_permissive(redistributions);
        let review = least_reviewed(reviews);
        let mut privacies = Vec::new();
        for object in &covered {
            for (key, child) in object.iter() {
                if key == "privacy_class" || key == "privacy_classification" {
                    privacies.push(child.as_str().unwrap_or(""));
                }
                privacy_values(child, &mut privacies);
            }
        }
        let privacy = reference_privacy(redistribution, &privacies, "private_library_data");
        let notice = match library {
            Some(library) => text(Some(library), "source_notice").or_else(|| text(provenance_source, "source_name")).unwrap_or("TBD").to_string(),
            None => "not held by the local library store".to_string(),
        };
        let (library_kind, source_notice) = match kind.as_str() {
            "material" | "section" | "component" => (kind.as_str(), notice),
            // The store's hanger kind has no handoff enum value (an open
            // contract gap of the handoff schema, design §3).
            "hanger" => ("TBD", format!("hanger library: {notice}")),
            _ => ("TBD", notice),
        };
        let unresolved_name = format!("{kind} library {id} (not held by the local library store)");
        refs.push(json!({
            "library_ref": reference("Library", &id),
            "library_kind": library_kind,
            "version": version.as_deref().unwrap_or("TBD"),
            "checksum": checksum("Library", &id, "library_reference_metadata", "JCS", &sha256_hex(&canonical_json(&metadata))),
            "source_notice": source_notice,
            "redistribution_status": if library.is_some() { redistribution } else { "unknown" },
            "review_classification": review,
            "privacy_classification": privacy,
            "private_payload_redacted": true,
            "provenance": mapped_provenance(provenance_source, if library.is_some() { "TBD" } else { unresolved_name.as_str() }, privacy)
        }));
    }
    if refusals.is_empty() { Ok((refs, warnings)) } else { Err(refusals) }
}

/// Why a model rule-pack reference is excluded, or `None` when it resolves:
/// the solve's resolution (#1602; `resolveModelRulePacks` in
/// `src/services/modelRulePackResolution.ts`), rule for rule, so that a handoff
/// warns of exactly the references a run of the same model excludes.
/// `stored` is the model's project's pack with the reference's id, if any.
/// - Unresolved: no stored pack, or its `metadata.rule_pack_id` is not the
///   reference's, or it has no `metadata.rule_pack_version` or
///   `metadata.source_notice` (non-blank text).
/// - Checksum mismatch: its stamped `checksums.rule_pack_checksum.value` is not
///   64 lowercase hex digits or not the reference's `checksum.value`, the
///   document's validation does not report its checksum as `match` (as the open
///   command's envelope reports it), or its version is not the reference's.
///
/// TypeScript twin and shared vectors: `fixtures/handoff/rule_pack_ref_resolution_vectors.json`.
pub(crate) fn rule_pack_ref_exclusion(reference: &Map<String, Value>, stored: Option<&Value>) -> Option<&'static str> {
    let Some(document) = stored else { return Some(RULE_PACK_REF_UNRESOLVED) };
    let metadata = document.get("metadata").and_then(Value::as_object);
    let stamped = document
        .get("checksums")
        .and_then(Value::as_object)
        .and_then(|checksums| checksums.get("rule_pack_checksum"))
        .and_then(Value::as_object)
        .and_then(|checksum| text(Some(checksum), "value"));
    let id = text(metadata, "rule_pack_id");
    let version = text(metadata, "rule_pack_version");
    if id != reference.get("rule_pack_id").and_then(Value::as_str) || version.is_none() || text(metadata, "source_notice").is_none() {
        return Some(RULE_PACK_REF_UNRESOLVED);
    }
    let recomputed_matches = open_pipe_stress_rule_pack_document::validate_rule_pack_document(document, false).checksum.match_status == "match";
    let sha256_hex = |value: &str| value.len() == 64 && value.bytes().all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'));
    let referenced = reference.get("checksum").and_then(|checksum| checksum.get("value")).and_then(Value::as_str);
    if !stamped.is_some_and(sha256_hex) || stamped != referenced || !recomputed_matches || version != reference.get("rule_pack_version").and_then(Value::as_str) {
        return Some(RULE_PACK_REF_CHECKSUM_MISMATCH);
    }
    None
}

/// `rule_pack_refs` (design §3): the document's references, each with a
/// checksum over its reference metadata `{rule_pack_id, version,
/// stamped_checksum_value, basis, grammar_version}`; the grammar version is the
/// stored pack's whose stamped checksum is the reference's (null, with a
/// `MODEL-RULE-PACK-REF-UNRESOLVED` warning, when the store holds none).
fn rule_pack_refs(document: &Value, project_id: &str, lookup: &mut RulePackLookup<'_>) -> Result<(Vec<Value>, Vec<Diagnostic>), Vec<Diagnostic>> {
    let mut refs = Vec::new();
    let mut warnings = Vec::new();
    let mut refusals = Vec::new();
    for (index, entry) in members(document, "rule_pack_refs") {
        let path = format!("rule_pack_refs[{index}]");
        let id = entry.get("rule_pack_id").and_then(Value::as_str).unwrap_or("");
        if !is_handoff_id(id) {
            refusals.push(diagnostic(
                ID_UNMAPPABLE,
                &format!("{path}.rule_pack_id"),
                format!("`{id}` is not a handoff identifier ([A-Za-z][A-Za-z0-9_.:-]*)."),
            ));
            continue;
        }
        let version = entry.get("rule_pack_version").and_then(Value::as_str).unwrap_or("");
        let stamped = entry.get("checksum").and_then(|checksum| checksum.get("value")).and_then(Value::as_str).unwrap_or("");
        // A store read error refuses the save. This is stricter than the solve's
        // resolution (#1602), which records the reference as unresolved; both fail
        // closed, and the handoff keeps its refusal.
        let stored = lookup(project_id, id).map_err(|error| {
            vec![diagnostic(RULE_PACK_STORE_UNAVAILABLE, &path, format!("The local rule-pack store could not be read to resolve `{id}` ({error}); nothing was saved."))]
        })?;
        // Resolved only as the solve resolves it (#1602, `rule_pack_ref_exclusion`):
        // among other checks, the stored stamp is the reference's and the pack's
        // checksum, recomputed from its stored document, matches it (review
        // round 1, finding 5: an edited draft keeps its old stamp).
        let exclusion = rule_pack_ref_exclusion(entry, stored.first());
        let pack = if exclusion.is_none() { stored.into_iter().next() } else { None };
        match exclusion {
            Some(RULE_PACK_REF_CHECKSUM_MISMATCH) => warnings.push(diagnostic(
                RULE_PACK_REF_CHECKSUM_MISMATCH,
                &path,
                format!("The local store's rule pack `{id}` for this project is not the referenced one: its stamped checksum, its own recomputed checksum or its version is not the reference's ({stamped}, version {version}); its grammar version, lifecycle and provenance are not known here."),
            )),
            Some(_) => warnings.push(diagnostic(
                RULE_PACK_REF_UNRESOLVED,
                &path,
                format!("The local store holds no rule pack `{id}` for this project with a matching id, a version and a source notice; its grammar version, lifecycle and provenance are not known here."),
            )),
            None => {}
        }
        // A resolved pack is screened whole before anything of it is mapped
        // (review round 2, finding 2).
        if let Some(pack) = &pack {
            let found = screen_document(pack);
            let malformed = malformed_dispositions(pack);
            if !found.is_empty() {
                refusals.push(blocked_record(&path, &format!("rule_pack:{id}"), "stored rule pack", &found));
            }
            if !malformed.is_empty() {
                refusals.push(malformed_record(&path, &format!("rule_pack:{id}"), "stored rule pack", &malformed));
            }
            if !found.is_empty() || !malformed.is_empty() {
                continue;
            }
        }
        let pack_metadata = pack.as_ref().and_then(|pack| pack.get("metadata")).and_then(Value::as_object);
        let metadata = json!({
            "rule_pack_id": id,
            "version": version,
            "stamped_checksum_value": stamped,
            "basis": open_pipe_stress_rule_pack_document::CHECKSUM_BASIS,
            "grammar_version": pack.as_ref().and_then(|pack| pack.get("grammar_version")).cloned().unwrap_or(Value::Null)
        });
        let pack_provenance = pack.as_ref().and_then(|pack| pack.get("provenance")).and_then(Value::as_object);
        let pack_classification = pack.as_ref().and_then(|pack| pack.get("classification")).and_then(Value::as_object);
        // The least permissive disposition the reference covers: its own,
        // the pack's classification and provenance, and every object's at any
        // depth of the pack (review round 2, finding 4).
        let mut dispositions = vec![handoff_redistribution(entry.get("redistribution_status").and_then(Value::as_str))];
        let mut reviews = Vec::new();
        if let Some(pack) = &pack {
            dispositions.push(handoff_redistribution(text(pack_classification, "redistribution_status")));
            dispositions.push(handoff_redistribution(text(pack_provenance, "redistribution_status")));
            reviews.push(lifecycle_review_classification(text(pack_metadata, "lifecycle_status")));
            reviews.push(review_classification(text(pack_provenance, "review_status")));
            let mut found = Vec::new();
            members_at_any_depth(pack, "redistribution_status", &mut found);
            dispositions.extend(found.into_iter().map(|status| handoff_redistribution(Some(status))));
            let mut found = Vec::new();
            members_at_any_depth(pack, "review_status", &mut found);
            reviews.extend(found.into_iter().map(|status| review_classification(Some(status))));
        }
        let redistribution = least_permissive(dispositions);
        let review = if pack.is_some() { least_reviewed(reviews) } else { "TBD" };
        let entry_value = Value::Object(entry.clone());
        let mut privacies = Vec::new();
        privacy_values(&entry_value, &mut privacies);
        if let Some(pack) = &pack {
            privacy_values(pack, &mut privacies);
        }
        let privacy = reference_privacy(redistribution, &privacies, "private_rule_pack_data");
        let source_notice = text(Some(entry), "source_notice").or_else(|| text(pack_metadata, "source_notice")).unwrap_or("TBD");
        let unresolved_name = format!("rule pack {id} (not held by the local rule-pack store)");
        refs.push(json!({
            "rule_pack_id": id,
            "version": if version.trim().is_empty() { "TBD" } else { version },
            "checksum": checksum("RulePack", id, "rule_pack_reference_metadata", "JCS", &sha256_hex(&canonical_json(&metadata))),
            "source_notice": source_notice,
            "redistribution_status": redistribution,
            "review_classification": review,
            "privacy_classification": privacy,
            "private_payload_redacted": true,
            "provenance": mapped_provenance(
                pack_provenance,
                if pack.is_some() { "TBD" } else { unresolved_name.as_str() },
                privacy
            )
        }));
    }
    if refusals.is_empty() { Ok((refs, warnings)) } else { Err(refusals) }
}

/// A finding as a handoff `Diagnostic`.
#[allow(clippy::too_many_arguments)]
fn handoff_diagnostic(project_id: &str, code: &str, class: &str, severity: &str, source_label: &str, affected: Value, message: &str, remediation: &str) -> Value {
    json!({
        "code": if code.trim().is_empty() { "TBD" } else { code },
        "class": class,
        "severity": severity,
        "source": { "object_type": "Model", "ref": project_id, "label": source_label },
        "affected_object": affected,
        "message": if message.trim().is_empty() { "TBD" } else { message },
        "remediation": remediation,
        "provenance": derived_provenance(&format!("model document {source_label}"))
    })
}

fn path_affected(project_id: &str, entity: Option<&str>, path: &str) -> Value {
    let mut affected = match entity {
        Some(entity) if !entity.is_empty() => model_qualified(project_id, entity),
        _ => reference("Model", project_id),
    };
    if !path.is_empty() {
        affected["label"] = json!(path);
    }
    affected
}

/// `warnings`: the document validator's non-blocking findings and the store
/// findings. `diagnostics`: the document's own (solve-independent) diagnostics.
fn warnings_and_diagnostics(document: &Value, project_id: &str, store: &[Diagnostic]) -> (Vec<Value>, Vec<Value>) {
    let mut warnings: Vec<Value> = validate_model_document(document)
        .iter()
        .filter(|finding| !finding.is_blocking())
        .map(|finding| {
            handoff_diagnostic(
                project_id,
                finding.code,
                diagnostic_class(finding.code),
                handoff_severity(finding.severity),
                "document validator",
                path_affected(project_id, finding.affected_refs.first().map(String::as_str), &finding.path),
                &finding.message,
                diagnostic_remediation(finding.code),
            )
        })
        .collect();
    warnings.extend(store.iter().map(|finding| {
        handoff_diagnostic(
            project_id,
            &finding.code,
            diagnostic_class(&finding.code),
            "warning",
            "local store check",
            path_affected(project_id, None, &finding.path),
            &finding.message,
            diagnostic_remediation(&finding.code),
        )
    }));
    let diagnostics = members(document, "diagnostics")
        .map(|(index, item)| {
            let code = item.get("code").and_then(Value::as_str).unwrap_or("");
            let own_class = item.get("class").or_else(|| item.get("diagnostic_class")).and_then(Value::as_str).filter(|class| DIAGNOSTIC_CLASSES.contains(class));
            let entity = item.get("affected_refs").and_then(Value::as_array).and_then(|refs| refs.first()).and_then(Value::as_str);
            let mut affected = path_affected(project_id, entity, &format!("diagnostics[{index}]"));
            if let Some(id) = item.get("id").and_then(Value::as_str).filter(|id| !id.is_empty()) {
                affected["label"] = json!(id);
            }
            let remediation = item.get("remediation").and_then(Value::as_str).filter(|text| !text.trim().is_empty()).unwrap_or(diagnostic_remediation(code));
            handoff_diagnostic(
                project_id,
                code,
                own_class.unwrap_or(diagnostic_class(code)),
                handoff_severity(item.get("severity").and_then(Value::as_str).unwrap_or("")),
                "diagnostics",
                affected,
                item.get("message").and_then(Value::as_str).unwrap_or(""),
                remediation,
            )
        })
        .collect();
    (warnings, diagnostics)
}

/// `<project id without "project:">.swbhandoff`, restricted as W-6a's file name.
pub fn suggested_package_name(document: &Value) -> String {
    let file = model_document_save::suggested_model_document_file_name(document);
    let stem = file.strip_suffix(".json").unwrap_or(&file);
    format!("{stem}.{}", publish::PACKAGE_EXTENSION)
}

/// The admission and every store-derived manifest part (see the module docs).
#[allow(clippy::too_many_arguments)]
pub fn prepare_handoff(
    canonical_text: String,
    claimed_model_hash: &str,
    evidence_payload_sha256: &str,
    basis_state_id: &str,
    package_label: &str,
    states: &mut StateLookup<'_>,
    libraries: &mut LibraryLookup<'_>,
    rule_packs: &mut RulePackLookup<'_>,
) -> Result<PreparedHandoff, Vec<Diagnostic>> {
    let package_label = admit_label(package_label)?;
    // One store read per library for both the screening and the references.
    let mut cache: BTreeMap<(String, String, String), Option<Value>> = BTreeMap::new();
    let mut cached = |project: &str, kind: &str, id: &str| -> Result<Option<Value>, String> {
        let key = (project.to_string(), kind.to_string(), id.to_string());
        if let Some(found) = cache.get(&key) {
            return Ok(found.clone());
        }
        let found = libraries(project, kind, id)?;
        cache.insert(key, found.clone());
        Ok(found)
    };
    let admitted = model_document_save::admit_model_document_text(canonical_text, claimed_model_hash, evidence_payload_sha256, &mut cached)?;
    let document: Value = serde_json::from_str(&admitted.text).expect("the admitted text is JSON");
    let model_hex = sha256_hex(&admitted.text);
    let project_id = document.pointer("/project/id").and_then(Value::as_str).unwrap_or("").to_string();
    let lookup_project = project_id.trim().to_string();
    let basis = basis_state(states, &lookup_project, basis_state_id, &model_hex)?;
    let mut refusals = Vec::new();
    let units = units_manifest(&document).map_err(|found| refusals.extend(found)).ok();
    let ids = entity_ids(&document, &project_id, &basis.state_id).map_err(|found| refusals.extend(found)).ok();
    let assumptions = unresolved_assumptions(&document, &project_id).map_err(|found| refusals.extend(found)).ok();
    let packs = rule_pack_refs(&document, &lookup_project, rule_packs).map_err(|found| refusals.extend(found)).ok();
    if !refusals.is_empty() {
        return Err(refusals);
    }
    let (library_refs, mut store) = library_refs(&document, &lookup_project, &mut cached)?;
    let (rule_pack_refs, pack_warnings) = packs.expect("no refusal");
    // W-6a's unresolved-reference warnings, then the version and rule-pack findings.
    let mut save_warnings = admitted.warnings;
    save_warnings.append(&mut store);
    save_warnings.extend(pack_warnings);
    let (warnings, diagnostics) = warnings_and_diagnostics(&document, &project_id, &save_warnings);
    Ok(PreparedHandoff {
        model_text: admitted.text,
        model_hex,
        project_id,
        package_label,
        suggested_package_name: suggested_package_name(&document),
        basis,
        units_manifest: units.expect("no refusal"),
        entity_ids: ids.expect("no refusal"),
        library_refs,
        rule_pack_refs,
        unresolved_assumptions: assumptions.expect("no refusal"),
        warnings,
        diagnostics,
        save_warnings,
    })
}

/// The package's name and creation time, known once the chooser answers.
pub struct PackageStamp<'a> {
    pub package_name: &'a str,
    /// Local ISO 8601 time.
    pub created_at: &'a str,
}

/// `handoff:<project token>:<model hash prefix>` (the `manifest:` id shares it).
fn package_token(project_id: &str, model_hex: &str) -> String {
    let id = project_id.strip_prefix("project:").unwrap_or(project_id);
    let token: String = id
        .chars()
        .map(|character| if character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-') { character } else { '-' })
        .take(96)
        .collect();
    let token = token.trim_matches(|character| matches!(character, '.' | '-' | '_'));
    format!("{}:{}", if token.is_empty() { "model" } else { token }, &model_hex[..12])
}

/// The manifest without `checksums` (design §3 table).
pub fn handoff_manifest(prepared: &PreparedHandoff, stamp: &PackageStamp<'_>) -> Map<String, Value> {
    let project_id = &prepared.project_id;
    let token = package_token(project_id, &prepared.model_hex);
    let handoff_id = format!("handoff:{token}");
    let model_hash = checksum("Model", project_id, "model_payload", "JCS", &prepared.model_hex);
    let state_id = &prepared.basis.state_id;
    let product = provenance_record(
        "SWBPIPE desktop",
        stamp.package_name,
        USER_LICENSE,
        LOCAL_USER,
        SOFTWARE_ATTESTATION,
        "private_only",
        "unreviewed",
        "private_project_data",
    );
    let included = || prepared.library_refs.iter().chain(&prepared.rule_pack_refs);
    let included_redistribution = least_permissive(
        std::iter::once("private_only").chain(included().filter_map(|item| item.get("redistribution_status").and_then(Value::as_str))),
    );
    let included_review =
        least_reviewed(std::iter::once("unreviewed").chain(included().filter_map(|item| item.get("review_classification").and_then(Value::as_str))));
    let manifest = json!({
        "package_identity": {
            "handoff_package_id": handoff_id,
            "manifest_id": format!("manifest:{token}"),
            "package_schema_version": publish::HANDOFF_SCHEMA_VERSION,
            "created_at": stamp.created_at,
            "created_by": format!("SWBPIPE desktop {}", env!("CARGO_PKG_VERSION")),
            "package_label": prepared.package_label,
            "package_kind": "canonical_handoff_manifest"
        },
        "model_basis": {
            "model_ref": reference("Model", project_id),
            "model_schema": "schemas/model.schema.yaml",
            "model_kind": "canonical_domain_model",
            "basis_state_ref": reference("ModelState", state_id),
            "source_refs": [],
            "hash_refs": [model_hash.clone()],
            "provenance": derived_provenance(&format!("{} {}", stamp.package_name, publish::MODEL_MEMBER))
        },
        "model_hash": model_hash,
        "units_manifest": prepared.units_manifest,
        "entity_ids": prepared.entity_ids,
        "model_state_refs": [{
            "model_state_ref": reference("ModelState", state_id),
            "model_state_schema": "schemas/model_state.schema.json",
            "state_kind": prepared.basis.state_kind,
            "hash_refs": [
                checksum("Model", project_id, "model_payload", "JCS", &prepared.model_hex),
                checksum("ModelState", state_id, "model_state_record", "deterministic_sorted_compact_json_payload_hash", &prepared.basis.record_hash_hex)
            ],
            "privacy_classification": "private_project_data",
            "provenance": provenance_record(
                "SWBPIPE desktop model state store",
                &format!("local model state {state_id}"),
                USER_LICENSE,
                LOCAL_USER,
                SOFTWARE_ATTESTATION,
                "private_only",
                "unreviewed",
                "private_project_data"
            )
        }],
        // Owner decision for v0.1 (reports-and-export manager, W-6b brief; design
        // §3 and §6): the package builds on saved states and references no runs.
        // Recorded runs (W-12 S2) are v0.3 records, while `AnalysisRunRef`'s
        // binding const names `schemas/analysis_run.schema.json`, so referencing
        // them needs an `AnalysisRunRef` contract edit first.
        "analysis_run_refs": [],
        "result_export_refs": [],
        "library_refs": prepared.library_refs,
        "rule_pack_refs": prepared.rule_pack_refs,
        "target_mapping_metadata": {
            "mapping_schema_status": "reserved_metadata_only",
            "target_system_kind": "TBD",
            "target_mapping_refs": [],
            "field_mapping_summary": "No target mapping: the target-mapping taxonomy is DEL-15-02's, and this package claims nothing target-specific.",
            "unsupported_behavior_refs": [],
            "approximate_behavior_refs": [],
            "detailed_taxonomy_owner": "DEL-15-02",
            "provenance": derived_provenance("handoff target mapping reservation (DEL-15-02)")
        },
        "unsupported_behavior_flags": [
            {
                "flag_id": "flag:target-format-not-selected",
                "behavior_label": "target_format_not_selected",
                "status": "requires_human_review",
                "target_ref": reference("TargetMapping", "TBD"),
                "affected_refs": [reference("HandoffPackage", &handoff_id)],
                "human_review_required": true,
                "provenance": derived_provenance("handoff target mapping reservation (DEL-15-02)")
            },
            {
                "flag_id": "flag:target-mapping-taxonomy-pending",
                "behavior_label": "target_mapping_taxonomy_pending",
                "status": "not_implemented",
                "target_ref": reference("TargetMapping", "reserved_for_DEL-15-02"),
                "affected_refs": [reference("HandoffPackage", &handoff_id)],
                "human_review_required": true,
                "provenance": derived_provenance("handoff target mapping reservation (DEL-15-02)")
            }
        ],
        "unresolved_assumptions": prepared.unresolved_assumptions,
        "warnings": prepared.warnings,
        "diagnostics": prepared.diagnostics,
        "privacy": {
            "classification": "private_project_data",
            "local_only": true,
            "telemetry_allowed": false,
            // The model is bound by hash in `checksums`, never embedded; the
            // references carry identities and metadata only.
            "private_payload_embedded": false,
            // Admission refused every protected_suspected, quarantined or rejected disposition.
            "protected_payload_embedded": false,
            "commercial_tool_payload_embedded": false,
            "redaction_refs": []
        },
        // The least permissive disposition of every included record (design
        // §3), the manifest's own provenance (private_only, unreviewed) among them.
        "redistribution_classification": included_redistribution,
        "review_classification": included_review,
        "provenance": product,
        "professional_boundary": {
            "human_review_required": true,
            "supports_downstream_modeling": true,
            "supports_downstream_review": true,
            "software_makes_compliance_claim": false,
            "software_makes_certification_claim": false,
            "software_makes_sealing_claim": false,
            "software_makes_approval_claim": false,
            "software_makes_authentication_claim": false,
            "software_creates_professional_reliance_record": false
        }
    });
    match manifest {
        Value::Object(map) => map,
        _ => unreachable!("a JSON object literal"),
    }
}

// ---------------------------------------------------------------------------
// The binding rule set: the producer's own rules, applied by the verifier.
// ---------------------------------------------------------------------------

/// The handoff `library_kind` of a store kind (hanger has none: `TBD`).
fn handoff_library_kind(kind: &str) -> &str {
    match kind {
        "material" | "section" | "component" => kind,
        _ => "TBD",
    }
}

/// What a manifest must say about the model member it binds (review round 1,
/// finding 3; the subject rule of #1597): every checksum's `payload_ref` names
/// the subject it hashes, by type and id; the model member's project is the
/// manifest's model; the package ids, `entity_ids`, assumptions and reference
/// identities are the ones this producer derives from that model; the basis
/// state references agree. `verify_handoff_package` applies these to every
/// package, the producer's own staged output included; the Python twin
/// (`tests/verifiers/handoff_package.py`) states the same rules. Returns the
/// findings (empty when bound).
pub fn manifest_binding_findings(manifest: &Map<String, Value>, model: &Value, model_hex: &str) -> Vec<String> {
    fn expect(findings: &mut Vec<String>, what: &str, found: Option<&Value>, wanted: &Value) {
        if found != Some(wanted) {
            findings.push(format!("{what} is {}, not {wanted}", found.map(Value::to_string).unwrap_or_else(|| "absent".to_string())));
        }
    }
    let mut findings = Vec::new();
    let Some(project_id) = model.pointer("/project/id").and_then(Value::as_str).filter(|id| !id.is_empty()) else {
        return vec!["the model member has no project id".to_string()];
    };
    let get = |pointer: &str| Value::Object(manifest.clone()).pointer(pointer).cloned();
    let model_ref = reference("Model", project_id);
    let model_checksum = checksum("Model", project_id, "model_payload", "JCS", model_hex);
    expect(&mut findings, "model_basis.model_ref", get("/model_basis/model_ref").as_ref(), &model_ref);
    expect(&mut findings, "model_hash", manifest.get("model_hash"), &model_checksum);
    expect(&mut findings, "model_basis.hash_refs", get("/model_basis/hash_refs").as_ref(), &json!([model_checksum.clone()]));
    let token = package_token(project_id, model_hex);
    expect(&mut findings, "package_identity.handoff_package_id", get("/package_identity/handoff_package_id").as_ref(), &json!(format!("handoff:{token}")));
    expect(&mut findings, "package_identity.manifest_id", get("/package_identity/manifest_id").as_ref(), &json!(format!("manifest:{token}")));
    // The basis state.
    let basis = get("/model_basis/basis_state_ref");
    let state_id = basis
        .as_ref()
        .filter(|basis| basis.get("object_type").and_then(Value::as_str) == Some("ModelState") && basis.as_object().is_some_and(|basis| basis.len() == 2))
        .and_then(|basis| basis.get("ref"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    if state_id.is_empty() {
        findings.push("model_basis.basis_state_ref is not a ModelState reference".to_string());
    }
    let states = manifest.get("model_state_refs").and_then(Value::as_array).cloned().unwrap_or_default();
    if states.len() != 1 {
        findings.push(format!("model_state_refs holds {} references, not the one basis state", states.len()));
    }
    if let Some(state) = states.first() {
        expect(&mut findings, "model_state_refs[0].model_state_ref", state.get("model_state_ref"), &reference("ModelState", &state_id));
        let hashes = state.get("hash_refs").and_then(Value::as_array).cloned().unwrap_or_default();
        if hashes.len() != 2 {
            findings.push(format!("model_state_refs[0].hash_refs holds {} checksums, not the model's and the state record's", hashes.len()));
        }
        expect(&mut findings, "model_state_refs[0].hash_refs[0]", hashes.first(), &model_checksum);
        if let Some(record) = hashes.get(1) {
            expect(&mut findings, "model_state_refs[0].hash_refs[1].payload_ref", record.get("payload_ref"), &reference("ModelState", &state_id));
            expect(&mut findings, "model_state_refs[0].hash_refs[1].payload_scope", record.get("payload_scope"), &json!("model_state_record"));
        }
    }
    // The inventories derived from the model member.
    match entity_ids(model, project_id, &state_id) {
        Ok(derived) => expect(&mut findings, "entity_ids", manifest.get("entity_ids"), &derived),
        Err(_) => findings.push("the model member's entity ids are not handoff identifiers".to_string()),
    }
    match unresolved_assumptions(model, project_id) {
        Ok(derived) => expect(&mut findings, "unresolved_assumptions", manifest.get("unresolved_assumptions"), &Value::Array(derived)),
        Err(_) => findings.push("the model member's assumption ids are not handoff identifiers".to_string()),
    }
    // The units manifest hashes its own block.
    if let Some(Value::Object(units)) = manifest.get("units_manifest") {
        let mut block = units.clone();
        block.remove("hash_refs");
        let system = units.get("unit_system_ref").and_then(|unit| unit.get("ref")).and_then(Value::as_str).unwrap_or("");
        let wanted = json!([checksum("UnitSystem", system, "units_manifest", "JCS", &sha256_hex(&canonical_json(&Value::Object(block))))]);
        expect(&mut findings, "units_manifest.hash_refs", units.get("hash_refs"), &wanted);
        expect(&mut findings, "units_manifest.unit_system_ref", units.get("unit_system_ref"), &reference("UnitSystem", UNIT_SYSTEM_REF));
    } else {
        findings.push("units_manifest is not an object".to_string());
    }
    // Library references: one per distinct model reference, each checksum's subject its own library.
    let mut found = Vec::new();
    collect_library_refs(model, "", &mut found);
    let distinct: BTreeSet<(String, String, Option<String>)> = found.into_iter().map(|item| (item.kind, item.id, item.version)).collect();
    let libraries = manifest.get("library_refs").and_then(Value::as_array).cloned().unwrap_or_default();
    if libraries.len() != distinct.len() {
        findings.push(format!("library_refs holds {} references; the model member names {}", libraries.len(), distinct.len()));
    }
    for (index, ((kind, id, version), entry)) in distinct.iter().zip(&libraries).enumerate() {
        expect(&mut findings, &format!("library_refs[{index}].library_ref"), entry.get("library_ref"), &reference("Library", id));
        expect(&mut findings, &format!("library_refs[{index}].library_kind"), entry.get("library_kind"), &json!(handoff_library_kind(kind)));
        expect(&mut findings, &format!("library_refs[{index}].version"), entry.get("version"), &json!(version.as_deref().unwrap_or("TBD")));
        expect(&mut findings, &format!("library_refs[{index}].checksum.payload_ref"), entry.pointer("/checksum/payload_ref"), &reference("Library", id));
        expect(&mut findings, &format!("library_refs[{index}].checksum.payload_scope"), entry.pointer("/checksum/payload_scope"), &json!("library_reference_metadata"));
    }
    // Rule-pack references: the model's, in order, each checksum's subject its own pack.
    let documented: Vec<(String, String)> = members(model, "rule_pack_refs")
        .map(|(_, entry)| {
            let field = |key: &str| entry.get(key).and_then(Value::as_str).unwrap_or("").to_string();
            let version = field("rule_pack_version");
            (field("rule_pack_id"), if version.trim().is_empty() { "TBD".to_string() } else { version })
        })
        .collect();
    let packs = manifest.get("rule_pack_refs").and_then(Value::as_array).cloned().unwrap_or_default();
    if packs.len() != documented.len() {
        findings.push(format!("rule_pack_refs holds {} references; the model member names {}", packs.len(), documented.len()));
    }
    for (index, ((id, version), entry)) in documented.iter().zip(&packs).enumerate() {
        expect(&mut findings, &format!("rule_pack_refs[{index}].rule_pack_id"), entry.get("rule_pack_id"), &json!(id));
        expect(&mut findings, &format!("rule_pack_refs[{index}].version"), entry.get("version"), &json!(version));
        expect(&mut findings, &format!("rule_pack_refs[{index}].checksum.payload_ref"), entry.pointer("/checksum/payload_ref"), &reference("RulePack", id));
        expect(&mut findings, &format!("rule_pack_refs[{index}].checksum.payload_scope"), entry.pointer("/checksum/payload_scope"), &json!("rule_pack_reference_metadata"));
    }
    findings
}

/// The local time now as ISO 8601 with its UTC offset (`2026-10-24T09:30:00.125-06:00`).
pub fn local_iso_now() -> String {
    let millis = crate::model_states::now_unix_millis().unwrap_or(0);
    let offset_seconds = local_offset_seconds(millis / 1000);
    let local = crate::model_states::utc_iso_millis(millis + offset_seconds * 1000);
    let sign = if offset_seconds < 0 { '-' } else { '+' };
    let magnitude = offset_seconds.abs();
    format!("{}{sign}{:02}:{:02}", local.trim_end_matches('Z'), magnitude / 3600, (magnitude / 60) % 60)
}

#[cfg(unix)]
fn local_offset_seconds(unix_seconds: i64) -> i64 {
    let time = unix_seconds as libc::time_t;
    // SAFETY: `tm` is plain data that `localtime_r` fills; both pointers are valid.
    let mut tm: libc::tm = unsafe { std::mem::zeroed() };
    if unsafe { libc::localtime_r(&time, &mut tm) }.is_null() {
        return 0;
    }
    tm.tm_gmtoff as i64
}

#[cfg(not(unix))]
fn local_offset_seconds(_: i64) -> i64 {
    0
}

/// The basis state, read again from the store immediately before publication
/// (review round 2, note 7): it must still be the state the manifest was
/// prepared from, with the same kind and record hash, and hold this model.
/// The offline verifier cannot check this; the save does, once.
pub fn confirm_basis_state(prepared: &PreparedHandoff, states: &mut StateLookup<'_>) -> Result<(), Vec<Diagnostic>> {
    let state_id = &prepared.basis.state_id;
    let current = basis_state(states, &prepared.project_id.trim().to_string(), state_id, &prepared.model_hex)?;
    if current != prepared.basis {
        return Err(vec![diagnostic(
            BASIS_STATE_MISMATCH,
            "",
            format!("Saved state `{state_id}` changed while the package was prepared (its record is no longer the one referenced); nothing was saved. Save again."),
        )]);
    }
    Ok(())
}

/// Names, seals and publishes a prepared handoff at `destination`.
pub fn write_handoff_package(destination: &Path, prepared: PreparedHandoff, created_at: &str, hooks: &PublishHooks<'_>) -> HandoffSaveOutcome {
    let package_name = destination.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default();
    let manifest = handoff_manifest(&prepared, &PackageStamp { package_name: &package_name, created_at });
    let (manifest_text, export_hex) = publish::seal_manifest(manifest, &prepared.model_hex);
    match publish::publish_package(destination, prepared.model_text.as_bytes(), manifest_text.as_bytes(), hooks) {
        Ok(Published { package_name, replaced_existing, durability, warnings }) => {
            let mut all = prepared.save_warnings;
            all.extend(warnings);
            HandoffSaveOutcome::Saved {
                package_name,
                model_hash: format!("sha256:{}", prepared.model_hex),
                export_hash: format!("sha256:{export_hex}"),
                basis_state_id: prepared.basis.state_id,
                replaced_existing,
                durability,
                warnings: all,
            }
        }
        Err(refusal) => HandoffSaveOutcome::Refused { diagnostics: vec![refusal] },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command_ingress::test_support::{admit_json, check_decode_bound, registry};
    use crate::command_ingress::ArgsDecoding;
    use crate::handoff_package_publish::{verify_handoff_package, MANIFEST_MEMBER, MODEL_DIRECTORY, MODEL_FILE, PRODUCTION_HOOKS};
    use crate::memory_pool::test_host::ScriptedHost;
    use crate::model_states::{labels_from_value, save_state_in_store, SaveModelStateRequest, StateWriteHooks};
    use rusqlite::{Connection, Transaction};
    use std::fs;
    use std::io;
    use std::path::PathBuf;

    const DEMO: &str = include_str!("../../../../fixtures/product_preview/invented_demo_model.json");
    const EVERY_SLOT: &str = include_str!("../../../../fixtures/model_document/invented_every_slot_model.json");
    const DEMO_RULE_PACK: &str = include_str!("../../../../fixtures/product_preview/invented_demo_rule_pack.json");
    const WITNESS_MODEL: &str = include_str!("../../../../fixtures/handoff/invented_witness.swbhandoff/model/model.json");
    const WITNESS_MANIFEST: &str = include_str!("../../../../fixtures/handoff/invented_witness.swbhandoff/handoff_package.json");
    const WITNESS_ENV: &str = "SWBPIPE_WRITE_HANDOFF_WITNESS";
    const BINARY64_MODEL: &str = include_str!("../../../../fixtures/handoff/invented_binary64_witness.swbhandoff/model/model.json");
    const BINARY64_MANIFEST: &str = include_str!("../../../../fixtures/handoff/invented_binary64_witness.swbhandoff/handoff_package.json");
    const BINARY64_ENV: &str = "SWBPIPE_WRITE_HANDOFF_BINARY64_WITNESS";
    const VECTORS: &str = include_str!("../../../../fixtures/handoff/canonical_text_vectors.json");
    const VECTORS_ENV: &str = "SWBPIPE_WRITE_HANDOFF_CANONICAL_VECTORS";
    const BASE: &str = "invented-envelope-base";
    const CREATED_AT: &str = "2026-10-24T12:00:00.000-06:00";

    fn hash_of(text: &str) -> String {
        format!("sha256:{}", sha256_hex(text))
    }

    fn sandbox(label: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "swbpipe-handoff-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
        ));
        fs::create_dir_all(&path).unwrap();
        path
    }

    /// The every-slot witness model with the units a handoff needs (stress) and
    /// one authored diagnostic, so every manifest list is populated.
    fn witness_model() -> Value {
        let mut model: Value = serde_json::from_str(EVERY_SLOT).unwrap();
        model["project"]["units"]["stress"] = json!("MPa");
        model["diagnostics"] = json!([{
            "id": "diagnostic:invented-review-note",
            "code": "INVENTED-REVIEW-NOTE",
            "severity": "info",
            "message": "Invented note carried by the model document.",
            "affected_refs": ["node:N-1"]
        }]);
        model
    }

    /// An invented public material library holding the witness's referenced record.
    fn material_library() -> Value {
        let provenance = json!({
            "source_name": "Invented handbook", "source_location": "invented://handbook", "source_license": "invented-permissive",
            "contributor": "Invented contributor", "contributor_certification": "invented-attestation",
            "redistribution_status": "public_permissive", "review_status": "accepted"
        });
        json!({
            "schema_version": "0.1.0",
            "material_library": { "library_id": "library:invented-materials", "name": "Invented materials", "provenance": provenance },
            "material_records": [{ "material_id": "steel", "name": "Invented steel", "provenance": provenance }]
        })
    }

    struct Store {
        root: PathBuf,
        path: PathBuf,
    }

    impl Drop for Store {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    /// A store holding the model's project row and, optionally, its library and rule pack.
    fn store(label: &str, model: &Value, with_references: bool) -> Store {
        let root = sandbox(label);
        let path = root.join(crate::PROJECT_STORE_FILE);
        let (mut connection, _) = crate::open_project_store(&path).unwrap();
        let project_id = model["project"]["id"].as_str().unwrap();
        crate::upsert_project(&mut connection, project_id, "Invented project", model, &json!([]), &Value::Null, &Value::Null, &Value::Null, &Value::Null, &Value::Null, &json!({ "value": BASE })).unwrap();
        if with_references {
            crate::upsert_library(&connection, project_id, "material", "library:invented-materials", &material_library()).unwrap();
            let pack: Value = serde_json::from_str(DEMO_RULE_PACK).unwrap();
            crate::upsert_rule_pack(&connection, project_id, "invented_demo_rule_pack", &pack).unwrap();
        }
        Store { root, path }
    }

    /// Saves `model` as a state through W-12 S1's store (`save_model_state`'s function).
    fn save_state(store: &Store, model: &Value, kind: &str, token: &'static str) -> crate::model_states::ModelStateSaveOutcome {
        let canonical = canonical_json(model);
        let request = SaveModelStateRequest {
            project_id: model["project"]["id"].as_str().unwrap().to_string(),
            expected_base: BASE.to_string(),
            model_hash: hash_of(&canonical),
            model_canonical: canonical,
            state_kind: kind.to_string(),
            parent_state_id: None,
            labels: labels_from_value(json!({ "state_name": "Handoff 2026-10-24", "tags": [], "notes": [], "assumptions": [] })).unwrap(),
        };
        let allocate = move || Ok((token.to_string(), 1_793_200_000_000_i64));
        let between_rows = |_: &Transaction<'_>| Ok(());
        save_state_in_store(&store.path, request, &StateWriteHooks { allocate: &allocate, between_rows: &between_rows }).unwrap()
    }

    /// `prepare_handoff` with the store's lookups, as the command wires them.
    fn prepare(store: &Store, model: &Value, state_id: &str) -> Result<PreparedHandoff, Vec<Diagnostic>> {
        let connection = Connection::open(&store.path).unwrap();
        let text = canonical_json(model);
        let hash = hash_of(&text);
        let mut states = |project: &str, state: &str| stored_state(&store.path, project, state, &crate::stored_read::UnmeteredReadMemory);
        let mut libraries = |project: &str, kind: &str, id: &str| crate::load_library(&connection, project, kind, id).map(|row| row.map(|row| row.document));
        let mut packs = |project: &str, id: &str| crate::stored_rule_packs_with_id(&connection, project, id);
        prepare_handoff(text, &hash, &hash, state_id, "Invented handoff", &mut states, &mut libraries, &mut packs)
    }

    fn codes(result: &Result<PreparedHandoff, Vec<Diagnostic>>) -> Vec<(String, String)> {
        match result {
            Ok(_) => vec![("prepared".to_string(), String::new())],
            Err(diagnostics) => diagnostics.iter().map(|item| (item.code.clone(), item.path.clone())).collect(),
        }
    }

    fn manifest_of(prepared: &PreparedHandoff) -> Value {
        let (text, _) = publish::seal_manifest(handoff_manifest(prepared, &PackageStamp { package_name: "invented.swbhandoff", created_at: CREATED_AT }), &prepared.model_hex);
        serde_json::from_str(&text).unwrap()
    }

    fn write(destination: &Path, prepared: PreparedHandoff) -> HandoffSaveOutcome {
        write_handoff_package(destination, prepared, CREATED_AT, &PRODUCTION_HOOKS)
    }

    // --- Basis state -------------------------------------------------------

    /// A new `handoff_basis` state written through S1's store is the basis; a
    /// state saved earlier with the same model hash is reused as it is (S1
    /// answers `already_saved`); a missing or other-model state refuses.
    #[test]
    fn basis_state_creation_reuse_and_refusals() {
        let model: Value = serde_json::from_str(DEMO).unwrap();
        let store = store("basis", &model, false);
        let created = save_state(&store, &model, "handoff_basis", "aaaa0000aaaa0000aaaa0000aaaa0000");
        assert_eq!(created.outcome, "saved");
        let prepared = prepare(&store, &model, &created.state.state_id).unwrap();
        assert_eq!(prepared.basis.state_kind, "handoff_basis");
        assert_eq!(prepared.basis.record_hash_hex, created.state.record_hash);
        let manifest = manifest_of(&prepared);
        let refs = &manifest["handoff_package_manifest"]["model_state_refs"];
        assert_eq!(refs.as_array().unwrap().len(), 1);
        assert_eq!(refs[0]["model_state_ref"], json!({"object_type": "ModelState", "ref": created.state.state_id}));
        assert_eq!(refs[0]["hash_refs"][1]["value"], json!(created.state.record_hash));
        assert_eq!(manifest["handoff_package_manifest"]["entity_ids"]["model_state_ids"], json!([created.state.state_id]));
        // Reuse: the same model again is the same saved state (nothing new is written).
        let again = save_state(&store, &model, "handoff_basis", "bbbb0000bbbb0000bbbb0000bbbb0000");
        assert_eq!((again.outcome, again.state.state_id.as_str()), ("already_saved", created.state.state_id.as_str()));
        assert_eq!(crate::model_states::saved_state_count(&Connection::open(&store.path).unwrap(), "project:invented-demo-loop-01").unwrap(), 1);
        // A state of the project with another model is not this handoff's basis.
        let mut edited = model.clone();
        edited["project"]["name"] = json!("Invented edited demo");
        let other = save_state(&store, &edited, "design_snapshot", "cccc0000cccc0000cccc0000cccc0000");
        assert_eq!(codes(&prepare(&store, &model, &other.state.state_id)), vec![(BASIS_STATE_MISMATCH.to_string(), String::new())]);
        // An earlier design snapshot of exactly this model is a valid basis, kept as it is.
        let prepared = prepare(&store, &edited, &other.state.state_id).unwrap();
        assert_eq!(prepared.basis.state_kind, "design_snapshot");
        assert_eq!(codes(&prepare(&store, &model, "state:absent")), vec![(BASIS_STATE_MISSING.to_string(), String::new())]);
    }

    /// Order (design §3): the state is committed first; a publication that
    /// fails leaves it, harmless and reusable by the next save.
    #[test]
    fn a_failed_publication_leaves_a_reusable_state() {
        let model: Value = serde_json::from_str(DEMO).unwrap();
        let store = store("order", &model, false);
        let state = save_state(&store, &model, "handoff_basis", "dddd0000dddd0000dddd0000dddd0000").state;
        let target = store.root.join("invented.swbhandoff");
        let fail = || -> io::Result<()> { Err(io::Error::other("invented failure before the commit")) };
        let hooks = PublishHooks { before_commit: &fail, ..PRODUCTION_HOOKS };
        let outcome = write_handoff_package(&target, prepare(&store, &model, &state.state_id).unwrap(), CREATED_AT, &hooks);
        assert!(matches!(&outcome, HandoffSaveOutcome::Refused { diagnostics } if diagnostics[0].code == publish::PUBLISH_FAILED), "{outcome:?}");
        assert!(!target.exists());
        let listed = crate::model_states::list_states(&Connection::open(&store.path).unwrap(), "project:invented-demo-loop-01").unwrap();
        assert_eq!(listed.iter().map(|item| item.state_id.as_str()).collect::<Vec<_>>(), vec![state.state_id.as_str()]);
        let outcome = write(&target, prepare(&store, &model, &state.state_id).unwrap());
        assert!(matches!(outcome, HandoffSaveOutcome::Saved { ref basis_state_id, .. } if basis_state_id == &state.state_id), "{outcome:?}");
        assert!(verify_handoff_package(&target).is_ok());
    }

    // --- Disposition refusals ----------------------------------------------

    #[test]
    fn quarantined_and_rejected_dispositions_refuse_and_private_ones_pass_as_they_are() {
        let base = witness_model();
        for (path, member, value) in [
            ("materials[0].provenance_record", "review_status", "quarantined"),
            ("sections[0].provenance_record", "review_status", "rejected"),
            ("supports[0].provenance_record", "redistribution_status", "protected_suspected"),
        ] {
            let mut model = base.clone();
            let family = path.split('[').next().unwrap();
            model[family][0]["provenance_record"][member] = json!(value);
            let store = store("disposition", &model, true);
            let state = save_state(&store, &model, "handoff_basis", "eeee0000eeee0000eeee0000eeee0000").state;
            let refused = prepare(&store, &model, &state.state_id).unwrap_err();
            assert_eq!(refused.iter().map(|item| (item.code.as_str(), item.path.as_str())).collect::<Vec<_>>(), vec![(model_document_save::PROTECTED_CONTENT_BLOCKED, path)], "{path} {value}");
            assert!(refused[0].message.contains(model[family][0]["id"].as_str().unwrap()), "{}", refused[0].message);
        }
        // A stored library quarantined in the store refuses, naming the library.
        let model = base.clone();
        let store_quarantined = store("disposition-store", &model, true);
        let mut library = material_library();
        library["material_library"]["review_status"] = json!("quarantined");
        crate::upsert_library(&Connection::open(&store_quarantined.path).unwrap(), "project:slots", "material", "library:invented-materials", &library).unwrap();
        let state = save_state(&store_quarantined, &model, "handoff_basis", "ffff0000ffff0000ffff0000ffff0000").state;
        let refused = prepare(&store_quarantined, &model, &state.state_id).unwrap_err();
        assert_eq!(refused[0].code, model_document_save::PROTECTED_CONTENT_BLOCKED);
        assert!(refused[0].message.contains("material:library:invented-materials"), "{}", refused[0].message);
        // private_only and unknown pass; the manifest records them as they are.
        let mut model = base.clone();
        model["materials"][0]["provenance_record"]["redistribution_status"] = json!("private_only");
        model["rule_pack_refs"][0]["redistribution_status"] = json!("unknown");
        let store = store("disposition-private", &model, true);
        let mut library = material_library();
        library["material_library"]["provenance"]["redistribution_status"] = json!("private_only");
        crate::upsert_library(&Connection::open(&store.path).unwrap(), "project:slots", "material", "library:invented-materials", &library).unwrap();
        let state = save_state(&store, &model, "handoff_basis", "abab0000abab0000abab0000abab0000").state;
        let manifest = manifest_of(&prepare(&store, &model, &state.state_id).unwrap());
        let manifest = &manifest["handoff_package_manifest"];
        assert_eq!(manifest["library_refs"][0]["redistribution_status"], json!("private_only"));
        assert_eq!(manifest["library_refs"][0]["privacy_classification"], json!("private_library_data"));
        assert_eq!(manifest["rule_pack_refs"][0]["redistribution_status"], json!("unknown"));
        assert_eq!(manifest["rule_pack_refs"][0]["privacy_classification"], json!("private_rule_pack_data"));
        assert_eq!(manifest["redistribution_classification"], json!("private_only"));
        assert_eq!(manifest["privacy"]["private_payload_embedded"], json!(false));
    }

    // --- Adapters ----------------------------------------------------------

    #[test]
    fn units_follow_the_derivation_table_or_refuse_by_name() {
        let demo: Value = serde_json::from_str(DEMO).unwrap();
        let units = units_manifest(&demo).unwrap();
        for (field, id) in [
            ("coordinate_unit", "unit:meter"),
            ("displacement_unit", "unit:meter"),
            ("rotation_unit", "unit:radian"),
            ("force_unit", "unit:newton"),
            ("moment_unit", "unit:newton_meter"),
            ("stress_unit", "unit:megapascal"),
        ] {
            assert_eq!(units[field], json!(id), "{field}");
        }
        assert_eq!(units["temperature_unit"], json!("unit:degree_celsius"));
        assert_eq!(units["unit_system_ref"], json!({"object_type": "UnitSystem", "ref": UNIT_SYSTEM_REF}));
        let mut unhashed = units.clone();
        unhashed.as_object_mut().unwrap().remove("hash_refs");
        assert_eq!(units["hash_refs"][0]["value"], json!(sha256_hex(&canonical_json(&unhashed))));
        // An explicit moment unit wins; kN with mm composes to no catalogue unit and refuses.
        let mut explicit = demo.clone();
        explicit["project"]["units"]["moment"] = json!("kN*m");
        assert_eq!(units_manifest(&explicit).unwrap()["moment_unit"], json!("unit:kilonewton_meter"));
        let mut composed = demo.clone();
        composed["project"]["units"]["force"] = json!("kN");
        composed["project"]["units"]["length"] = json!("mm");
        assert_eq!(units_manifest(&composed).unwrap_err().iter().map(|item| (item.code.as_str(), item.path.as_str())).collect::<Vec<_>>(),
            vec![(UNIT_UNDERIVABLE, "project.units.moment")]);
        // A missing or unknown unit refuses, naming each.
        let mut missing = demo.clone();
        missing["project"]["units"].as_object_mut().unwrap().remove("stress");
        missing["project"]["units"]["angle"] = json!("gradian-invented");
        assert_eq!(units_manifest(&missing).unwrap_err().iter().map(|item| item.path.as_str()).collect::<Vec<_>>(),
            vec!["project.units.angle", "project.units.stress"]);
    }

    #[test]
    fn entity_ids_assumptions_and_references_map_as_the_design_says() {
        let model = witness_model();
        let ids = entity_ids(&model, "project:slots", "state:x").unwrap();
        assert_eq!(ids["node_ids"], json!(["node:N-1", "node:N-2", "node:N-3"]));
        assert_eq!(ids["element_ids"], json!(["pipe:P-1", "pipe:P-2"]));
        assert_eq!(ids["load_case_ids"], json!(["combination:C-OP", "load:L-1"]));
        assert_eq!(ids["component_ids"], json!(["component:C-1"]));
        let mapped: Vec<&str> = ids["mapping_id_refs"].as_array().unwrap().iter().map(|item| item["ref"].as_str().unwrap()).collect();
        for entity in ["support:S-1", "material:steel", "section:std", "line:L-100", "constraint:K-001", "equipment-interface:E-001"] {
            assert!(mapped.contains(&format!("project:slots#{entity}").as_str()), "{entity}");
        }
        let mut renamed = model.clone();
        renamed["nodes"][0]["id"] = json!("node N 1");
        assert_eq!(entity_ids(&renamed, "project:slots", "state:x").unwrap_err()[0].path, "nodes[0].id");
        let mut model = model;
        model["unresolved_assumptions"][0]["affected_refs"] = json!(["support:S-1", "project:slots", "assumption:A-002", "support:S-gone"]);
        model["unresolved_assumptions"][1]["status"] = json!("unresolved");
        let records = unresolved_assumptions(&model, "project:slots").unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0]["affected_refs"], json!([
            {"object_type": "Model", "ref": "project:slots#support:S-1"},
            {"object_type": "Project", "ref": "project:slots"},
            {"object_type": "Assumption", "ref": "assumption:A-002"},
            {"object_type": "Model", "ref": "project:slots#support:S-gone"}
        ]));
        assert_eq!(records[1]["statement"], json!("Open issue: Invented open issue."));
        assert_eq!(records[1]["affected_refs"], json!([{"object_type": "Project", "ref": "project:slots"}]));
        assert!(records[1].get("resolution_note").is_none() && records[0].get("tags").is_none());
    }

    #[test]
    fn library_and_rule_pack_references_resolve_against_the_store() {
        let model = witness_model();
        let store = store("references", &model, true);
        let state = save_state(&store, &model, "handoff_basis", "acac0000acac0000acac0000acac0000").state;
        let prepared = prepare(&store, &model, &state.state_id).unwrap();
        let library = &prepared.library_refs[0];
        assert_eq!(prepared.library_refs.len(), 1);
        assert_eq!((library["library_kind"].as_str(), library["version"].as_str()), (Some("material"), Some("1.0.0")));
        let metadata = json!({
            "library_kind": "material", "library_id": "library:invented-materials", "library_version": "1.0.0",
            "document_sha256": sha256_hex(&canonical_json(&material_library()))
        });
        assert_eq!(library["checksum"]["value"], json!(sha256_hex(&canonical_json(&metadata))));
        assert_eq!(library["checksum"]["payload_scope"], json!("library_reference_metadata"));
        assert_eq!(library["provenance"]["contributor_attestation"], json!("invented-attestation"));
        assert_eq!(library["review_classification"], json!("human_reviewed_reference_only"));
        // The recorded 1.0.0 is not the stored record's digest: a named warning, never an update.
        assert!(prepared.save_warnings.iter().any(|item| item.code == LIBRARY_REF_VERSION_CHANGED), "{:?}", prepared.save_warnings);
        let pack = &prepared.rule_pack_refs[0];
        let stored: Value = serde_json::from_str(DEMO_RULE_PACK).unwrap();
        let metadata = json!({
            "rule_pack_id": "invented_demo_rule_pack", "version": "0.2.0",
            "stamped_checksum_value": stored["checksums"]["rule_pack_checksum"]["value"],
            "basis": open_pipe_stress_rule_pack_document::CHECKSUM_BASIS, "grammar_version": stored["grammar_version"]
        });
        assert_eq!(pack["checksum"]["value"], json!(sha256_hex(&canonical_json(&metadata))));
        assert_eq!(pack["review_classification"], json!("human_reviewed_reference_only"));
        assert_eq!(pack["privacy_classification"], json!("invented_public_example"));
        // Without the store's records: TBD and named warnings, never invented values.
        let bare = store_without_references(&model);
        let prepared = prepare(&bare, &model, &save_state(&bare, &model, "handoff_basis", "adad0000adad0000adad0000adad0000").state.state_id).unwrap();
        assert_eq!(prepared.library_refs[0]["redistribution_status"], json!("unknown"));
        assert_eq!(prepared.rule_pack_refs[0]["review_classification"], json!("TBD"));
        let found: Vec<&str> = prepared.save_warnings.iter().map(|item| item.code.as_str()).collect();
        assert!(found.contains(&model_document_save::LIBRARY_REF_UNRESOLVED) && found.contains(&RULE_PACK_REF_UNRESOLVED), "{found:?}");
        // A hanger library maps to TBD with a hanger source notice.
        let mut hanger = model.clone();
        hanger["materials"][0]["provenance_record"]["library_ref"] = json!({"library_kind": "hanger", "library_id": "invented.hangers", "record_id": "invented.variable"});
        let bare = store_without_references(&hanger);
        let prepared = prepare(&bare, &hanger, &save_state(&bare, &hanger, "handoff_basis", "aeae0000aeae0000aeae0000aeae0000").state.state_id).unwrap();
        assert_eq!(prepared.library_refs[0]["library_kind"], json!("TBD"));
        assert!(prepared.library_refs[0]["source_notice"].as_str().unwrap().starts_with("hanger library"));
        assert_eq!(prepared.library_refs[0]["version"], json!("TBD"));
    }

    fn store_without_references(model: &Value) -> Store {
        store("bare", model, false)
    }

    // --- Publication, verifier, tamper witnesses, failure injections ----------

    fn prepared_demo(label: &str) -> (Store, PreparedHandoff) {
        let model: Value = serde_json::from_str(DEMO).unwrap();
        let store = store(label, &model, false);
        let state = save_state(&store, &model, "handoff_basis", "afaf0000afaf0000afaf0000afaf0000").state;
        let prepared = prepare(&store, &model, &state.state_id).unwrap();
        (store, prepared)
    }

    fn package_bytes(target: &Path) -> (Vec<u8>, Vec<u8>) {
        (fs::read(target.join(MODEL_DIRECTORY).join(MODEL_FILE)).unwrap(), fs::read(target.join(MANIFEST_MEMBER)).unwrap())
    }

    fn staging_left(parent: &Path) -> Vec<String> {
        fs::read_dir(parent).unwrap().filter_map(Result::ok).map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| name.contains(".staging-")).collect()
    }

    #[test]
    fn a_saved_package_verifies_and_one_changed_byte_in_either_member_fails() {
        let (store, prepared) = prepared_demo("tamper");
        let model_text = prepared.model_text.clone();
        let target = store.root.join("invented.swbhandoff");
        let HandoffSaveOutcome::Saved { model_hash, export_hash, replaced_existing, .. } = write(&target, prepared) else { panic!("not saved") };
        assert!(!replaced_existing);
        let verified = verify_handoff_package(&target).unwrap();
        assert_eq!((verified.model_hash.as_str(), verified.export_hash.as_str()), (model_hash.as_str(), export_hash.as_str()));
        let (model_bytes, manifest_bytes) = package_bytes(&target);
        assert_eq!(model_bytes, model_text.as_bytes(), "model/model.json is the W-6a bytes");
        assert_eq!(model_hash, hash_of(&model_text));
        for (member, original) in [(target.join(MODEL_DIRECTORY).join(MODEL_FILE), model_bytes), (target.join(MANIFEST_MEMBER), manifest_bytes)] {
            let text = String::from_utf8(original.clone()).unwrap();
            // Every byte of each checksum value, and an even spread of the rest.
            let mut positions: BTreeSet<usize> = (0..original.len()).step_by((original.len() / 300).max(1)).collect();
            positions.insert(original.len() - 1);
            for value in [&model_hash[7..], &export_hash[7..]] {
                if let Some(at) = text.find(value) {
                    positions.extend(at..at + value.len());
                }
            }
            for position in positions {
                let mut tampered = original.clone();
                tampered[position] ^= 0x01;
                fs::write(&member, &tampered).unwrap();
                assert!(verify_handoff_package(&target).is_err(), "{} byte {position} changed and still verifies", member.display());
            }
            fs::write(&member, &original).unwrap();
        }
        assert!(verify_handoff_package(&target).is_ok());
    }

    #[test]
    fn failures_between_the_members_or_before_the_commit_leave_the_previous_package() {
        let (store, prepared) = prepared_demo("inject");
        let target = store.root.join("invented.swbhandoff");
        let model: Value = serde_json::from_str(DEMO).unwrap();
        assert!(matches!(write(&target, prepared), HandoffSaveOutcome::Saved { .. }));
        let previous = package_bytes(&target);
        let mut edited = model.clone();
        edited["project"]["description"] = json!("Invented second revision.");
        let state = save_state(&store, &edited, "handoff_basis", "baba0000baba0000baba0000baba0000").state;
        let fail = || -> io::Result<()> { Err(io::Error::other("invented injected failure")) };
        let unsupported = |_: &Path, _: &Path| -> io::Result<()> { Err(io::Error::from(io::ErrorKind::Unsupported)) };
        for (label, hooks, code) in [
            ("between members", PublishHooks { after_model_member: &fail, ..PRODUCTION_HOOKS }, publish::PUBLISH_FAILED),
            ("before the swap", PublishHooks { before_commit: &fail, ..PRODUCTION_HOOKS }, publish::PUBLISH_FAILED),
            ("swap unsupported", PublishHooks { swap: Some(&unsupported), ..PRODUCTION_HOOKS }, publish::REPLACE_UNSUPPORTED),
        ] {
            let outcome = write_handoff_package(&target, prepare(&store, &edited, &state.state_id).unwrap(), CREATED_AT, &hooks);
            let HandoffSaveOutcome::Refused { diagnostics } = outcome else { panic!("{label}: {outcome:?}") };
            assert_eq!(diagnostics[0].code, code, "{label}");
            assert_eq!(package_bytes(&target), previous, "{label}: the previous package changed");
            assert!(verify_handoff_package(&target).is_ok(), "{label}");
            assert!(staging_left(&store.root).is_empty(), "{label}: {:?}", staging_left(&store.root));
        }
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn an_existing_package_is_replaced_by_one_swap_and_the_old_one_removed() {
        let (store, prepared) = prepared_demo("swap");
        let target = store.root.join("invented.swbhandoff");
        assert!(matches!(write(&target, prepared), HandoffSaveOutcome::Saved { .. }));
        let previous = package_bytes(&target);
        // Finder metadata in the old package does not stop the replacement.
        fs::write(target.join(".DS_Store"), b"invented").unwrap();
        let model: Value = serde_json::from_str(DEMO).unwrap();
        let mut edited = model.clone();
        edited["project"]["description"] = json!("Invented replacement.");
        let state = save_state(&store, &edited, "handoff_basis", "bcbc0000bcbc0000bcbc0000bcbc0000").state;
        let outcome = write(&target, prepare(&store, &edited, &state.state_id).unwrap());
        let HandoffSaveOutcome::Saved { replaced_existing, warnings, model_hash, .. } = outcome else { panic!("{outcome:?}") };
        assert!(replaced_existing);
        assert!(warnings.is_empty(), "{warnings:?}");
        assert_ne!(package_bytes(&target), previous);
        assert_eq!(verify_handoff_package(&target).unwrap().model_hash, model_hash);
        assert!(staging_left(&store.root).is_empty());
        assert!(!target.join(".DS_Store").exists());
    }

    #[test]
    fn leftover_staging_is_reported_and_other_destinations_are_never_replaced() {
        let (store, prepared) = prepared_demo("leftover");
        let target = store.root.join("invented.swbhandoff");
        let leftover = store.root.join("invented.swbhandoff.staging-0123456789abcdef");
        fs::create_dir(&leftover).unwrap();
        fs::write(leftover.join("partial"), b"invented").unwrap();
        let outcome = write(&target, prepared);
        let HandoffSaveOutcome::Saved { warnings, .. } = outcome else { panic!("{outcome:?}") };
        assert_eq!(warnings.iter().map(|item| item.code.as_str()).collect::<Vec<_>>(), vec![publish::STAGING_LEFTOVER]);
        assert!(warnings[0].message.contains("invented.swbhandoff.staging-0123456789abcdef"));
        assert!(leftover.join("partial").exists(), "a leftover is ignored, not deleted");
        // A folder that is not a handoff package is never replaced.
        let (store, prepared) = prepared_demo("not-package");
        let folder = store.root.join("folder.swbhandoff");
        fs::create_dir(&folder).unwrap();
        fs::write(folder.join("notes.txt"), b"invented user notes").unwrap();
        let outcome = write(&folder, prepared);
        assert!(matches!(&outcome, HandoffSaveOutcome::Refused { diagnostics } if diagnostics[0].code == publish::DESTINATION_NOT_PACKAGE), "{outcome:?}");
        assert_eq!(fs::read(folder.join("notes.txt")).unwrap(), b"invented user notes");
        let (store, prepared) = prepared_demo("extension");
        let outcome = write(&store.root.join("invented.json"), prepared);
        assert!(matches!(&outcome, HandoffSaveOutcome::Refused { diagnostics } if diagnostics[0].code == publish::DESTINATION_INVALID), "{outcome:?}");
    }

    // --- The witness ----------------------------------------------------------

    /// The fully populated invented witness package
    /// (`fixtures/handoff/invented_witness.swbhandoff/`): the producer's exact
    /// output for the witness model and store, which pytest validates against
    /// the schemas and `workflow.py` and verifies with the Python twin.
    /// Regenerate after an intended change with
    /// `SWBPIPE_WRITE_HANDOFF_WITNESS=<package folder> cargo test witness_package`.
    #[test]
    fn witness_package_is_the_producers_output() {
        let model = witness_model();
        let store = store("witness", &model, true);
        let state = save_state(&store, &model, "handoff_basis", "0123456789abcdef0123456789abcdef").state;
        let prepared = prepare(&store, &model, &state.state_id).unwrap();
        let manifest = handoff_manifest(&prepared, &PackageStamp { package_name: "invented_witness.swbhandoff", created_at: CREATED_AT });
        for list in ["library_refs", "rule_pack_refs", "unresolved_assumptions", "warnings", "diagnostics", "unsupported_behavior_flags"] {
            assert!(!manifest[list].as_array().unwrap().is_empty(), "{list} is empty");
        }
        assert!(!manifest["entity_ids"]["mapping_id_refs"].as_array().unwrap().is_empty());
        let (text, _) = publish::seal_manifest(manifest, &prepared.model_hex);
        if let Ok(folder) = std::env::var(WITNESS_ENV) {
            let folder = PathBuf::from(folder);
            fs::create_dir_all(folder.join(MODEL_DIRECTORY)).unwrap();
            fs::write(folder.join(MODEL_DIRECTORY).join(MODEL_FILE), &prepared.model_text).unwrap();
            fs::write(folder.join(MANIFEST_MEMBER), &text).unwrap();
            return;
        }
        assert_eq!(WITNESS_MODEL, prepared.model_text, "the witness model differs; regenerate it");
        assert_eq!(WITNESS_MANIFEST, text, "the witness manifest differs from the producer's output; regenerate it");
        let committed = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures/handoff/invented_witness.swbhandoff");
        assert!(verify_handoff_package(&committed).is_ok());
    }

    /// Review round 1, finding 6: a produced package whose model holds a number
    /// outside the checked profile (an elastic modulus of 1e20 Pa, which W-6a
    /// admits and the session hashes), committed so that the Python twin is
    /// checked against the producer's own bytes. Regenerate with
    /// `SWBPIPE_WRITE_HANDOFF_BINARY64_WITNESS=<package folder> cargo test binary64_witness`.
    #[test]
    fn binary64_witness_package_is_the_producers_output() {
        let mut model: Value = serde_json::from_str(DEMO).unwrap();
        model["materials"][0]["elastic_modulus"]["value"] = json!(1e20);
        let store = store("binary64-witness", &model, false);
        let state = save_state(&store, &model, "handoff_basis", "fedcba9876543210fedcba9876543210").state;
        let prepared = prepare(&store, &model, &state.state_id).unwrap();
        assert!(prepared.model_text.contains("100000000000000000000"), "the JCS text of 1e20");
        let manifest = handoff_manifest(&prepared, &PackageStamp { package_name: "invented_binary64_witness.swbhandoff", created_at: CREATED_AT });
        let (text, _) = publish::seal_manifest(manifest, &prepared.model_hex);
        if let Ok(folder) = std::env::var(BINARY64_ENV) {
            let folder = PathBuf::from(folder);
            fs::create_dir_all(folder.join(MODEL_DIRECTORY)).unwrap();
            fs::write(folder.join(MODEL_DIRECTORY).join(MODEL_FILE), &prepared.model_text).unwrap();
            fs::write(folder.join(MANIFEST_MEMBER), &text).unwrap();
            return;
        }
        assert_eq!(BINARY64_MODEL, prepared.model_text, "the binary64 witness model differs; regenerate it");
        assert_eq!(BINARY64_MANIFEST, text, "the binary64 witness manifest differs; regenerate it");
        let committed = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures/handoff/invented_binary64_witness.swbhandoff");
        assert!(verify_handoff_package(&committed).is_ok());
    }

    /// The cross-language vector set (finding 6): W-6a's unchecked canonical
    /// text (`canonical_json`, the session's model-hash serializer) of numbers
    /// and strings at the profile's edges, pinned here and checked by the Python
    /// twin (`tests/test_handoff_package_witness.py`). Regenerate with
    /// `SWBPIPE_WRITE_HANDOFF_CANONICAL_VECTORS=<path> cargo test canonical_text_vectors`.
    #[test]
    fn canonical_text_vectors_are_w6a_serializer_output() {
        let inputs = [
            "1e20", "100000000000000000000", "1e21", "123456789012345678901234567890", "1e-7", "1e-6", "0.000001", "-0", "-0.0",
            "0.1", "1.0", "1.5e300", "5e-324", "1.7976931348623157e308", "2.2250738585072014e-308", "9007199254740991",
            "9007199254740992", "9007199254740993", "18446744073709551615", "18446744073709551616", "-9223372036854775808",
            "-9223372036854775809", "4.35", "0.30000000000000004", "1e+22", "-1.2e-05",
            "\"é中😀\"", "\"\\u0000\\u001f\\u007f\\u2028\\\"\\\\/\"", "\"\\ud83d\\ude00\"",
            "{\"b\":1,\"a\":[true,false,null],\"😀\":2,\"\\uffff\":3,\"é\":4}",
            "{\"value\":200000000000.0,\"unit\":\"Pa\",\"nested\":{\"z\":1.2e-05,\"y\":[1e20,-0.0,3]}}",
        ];
        let vectors: Vec<Value> = inputs
            .iter()
            .map(|input| {
                let text = canonical_json(&serde_json::from_str::<Value>(input).unwrap());
                json!({ "input_json": input, "canonical_text": text, "sha256": sha256_hex(&text) })
            })
            .collect();
        let table = json!({
            "description": "Invented cross-language vectors (DEL-15-01 W-6b review round 1): the unchecked RFC 8785 text of W-6a's serializer (open_pipe_stress_operation_applier::canonical_json) for each input, which the Python handoff verifier twin must reproduce. Generated by handoff_package.rs canonical_text_vectors_are_w6a_serializer_output.",
            "vectors": vectors,
        });
        if let Ok(path) = std::env::var(VECTORS_ENV) {
            fs::write(path, format!("{}\n", serde_json::to_string_pretty(&table).unwrap())).unwrap();
            return;
        }
        let pinned: Value = serde_json::from_str(VECTORS).unwrap();
        assert_eq!(pinned, table, "the pinned vectors differ from the serializer's output; regenerate them");
    }

    // --- Command surface ------------------------------------------------------

    fn screening(payload: &str) -> ModelDocumentSaveScreening {
        ModelDocumentSaveScreening {
            route_id: HANDOFF_PACKAGE_ROUTE_ID.to_string(),
            export_context: "local_private".to_string(),
            explicit_local_private_intent: true,
            blocked: false,
            materialization_withheld: false,
            lossless_required: true,
            exact_payload_match: true,
            blocking_count: 0,
            payload_sha256: payload.to_string(),
        }
    }

    fn local_first() -> ModelDocumentSaveLocalFirst {
        ModelDocumentSaveLocalFirst {
            route_id: HANDOFF_PACKAGE_ROUTE_ID.to_string(),
            export_context: "local_private".to_string(),
            storage_context: "local_private".to_string(),
            action: "include_metadata_only".to_string(),
            reason_code: "PRIVATE_LOCAL_METADATA_ALLOWED".to_string(),
            blocked: false,
            metadata_only: true,
            explicit_local_private_intent: true,
        }
    }

    #[test]
    fn route_evidence_is_rechecked_for_this_route() {
        assert!(validate_handoff_evidence(&screening("sha256:x"), &local_first()).is_ok());
        let mut model_route = screening("sha256:x");
        model_route.route_id = model_document_save::MODEL_DOCUMENT_SAVE_ROUTE_ID.to_string();
        assert_eq!(validate_handoff_evidence(&model_route, &local_first()).unwrap_err().code, model_document_save::EVIDENCE_INVALID);
        let mut no_intent = local_first();
        no_intent.explicit_local_private_intent = false;
        assert_eq!(validate_handoff_evidence(&screening("sha256:x"), &no_intent).unwrap_err().code, model_document_save::EVIDENCE_INVALID);
        let mut lossy = screening("sha256:x");
        lossy.lossless_required = false;
        assert!(validate_handoff_evidence(&lossy, &local_first()).is_err());
    }

    #[test]
    fn arguments_decode_through_the_ingress_lease_and_outcomes_serialize() {
        let text = canonical_json(&serde_json::from_str::<Value>(DEMO).unwrap());
        let body = json!({
            "canonicalText": text, "claimedModelHash": hash_of(&text),
            "screening": serde_json::to_value(screening(&hash_of(&text))).unwrap(),
            "localFirstEvidence": serde_json::to_value(local_first()).unwrap(),
            "basisStateId": "state:x", "packageLabel": "Invented"
        });
        let registry = registry(ScriptedHost::normal());
        let (mut args, _lease) = admit_json(&registry, "save_handoff_package", ArgsDecoding::Moved, &body).unwrap();
        let request = save_handoff_package_args(&mut args).unwrap();
        assert_eq!((request.canonical_text.as_str(), request.basis_state_id.as_str(), request.package_label.as_str()), (text.as_str(), "state:x", "Invented"));
        let raw = serde_json::to_vec(&body).unwrap();
        let _ = check_decode_bound(&raw, ArgsDecoding::Moved, |args| save_handoff_package_args(args).map(drop));
        assert_eq!(serde_json::to_value(HandoffSaveOutcome::Cancelled).unwrap(), json!({"outcome": "cancelled"}));
        assert_eq!(
            serde_json::to_value(HandoffSaveOutcome::refused_one(LABEL_INVALID, "m")).unwrap(),
            json!({"outcome": "refused", "diagnostics": [{"code": LABEL_INVALID, "path": "", "message": "m"}]})
        );
        assert_eq!(suggested_package_name(&json!({"project": {"id": "project:invented-demo-loop-01"}})), "invented-demo-loop-01.swbhandoff");
        assert!(local_iso_now().len() >= "2026-10-24T00:00:00.000+00:00".len());
    }

    // --- Review round 1 regressions (Codex, scratch/rep-w6b-review/regressions.patch) ---

    /// Where the review regressions write their packages: a fresh scratch folder
    /// (the reviewer kept them under `W6B_REVIEW_OUTPUT`).
    fn review_output() -> PathBuf {
        match std::env::var("W6B_REVIEW_OUTPUT") {
            Ok(path) => {
                let path = PathBuf::from(path);
                fs::create_dir_all(&path).unwrap();
                path
            }
            Err(_) => sandbox("review"),
        }
    }
    fn review_emit(name: &str, prepared: PreparedHandoff) {
        let target = review_output().join(format!("{name}.swbhandoff"));
        let out = write(&target, prepared);
        assert!(matches!(&out, HandoffSaveOutcome::Saved { .. }), "{name}: {out:?}");
    }
    #[test]
    fn review_resolved_rule_pack_dispositions_must_refuse() {
        let mut escaped = Vec::new();
        for (tag, pointer, status) in [
            ("lifecycle-quarantined", "/metadata/lifecycle_status", "quarantined"),
            ("lifecycle-rejected", "/metadata/lifecycle_status", "rejected"),
            ("provenance-protected", "/provenance/redistribution_status", "protected_suspected"),
            ("provenance-quarantined", "/provenance/review_status", "quarantined"),
            ("provenance-rejected", "/provenance/review_status", "rejected"),
        ] {
            let mut model = witness_model();
            let mut pack: Value = serde_json::from_str(DEMO_RULE_PACK).unwrap();
            *pack.pointer_mut(pointer).unwrap() = json!(status);
            let computed = open_pipe_stress_rule_pack_document::compute_rule_pack_checksum(&pack).unwrap();
            pack["checksums"]["rule_pack_checksum"]["value"] = json!(computed.value);
            model["rule_pack_refs"][0]["checksum"]["value"] = json!(computed.value);
            let store = store(tag, &model, true);
            crate::upsert_rule_pack(&Connection::open(&store.path).unwrap(), "project:slots", "invented_demo_rule_pack", &pack).unwrap();
            let state = save_state(&store, &model, "handoff_basis", "12340000123400001234000012340000").state;
            match prepare(&store, &model, &state.state_id) {
                Ok(prepared) => {
                    println!("ESCAPED {tag}: {}", prepared.rule_pack_refs[0]);
                    review_emit(tag, prepared);
                    escaped.push(tag);
                },
                Err(diagnostics) => {
                    println!("REFUSED {tag}: {diagnostics:?}");
                    assert_eq!((diagnostics[0].code.as_str(), diagnostics[0].path.as_str()), (model_document_save::PROTECTED_CONTENT_BLOCKED, "rule_pack_refs[0]"), "{tag}");
                    assert!(diagnostics[0].message.contains("rule_pack:invented_demo_rule_pack"), "{tag}: {}", diagnostics[0].message);
                }
            }
        }
        assert!(escaped.is_empty(), "stored rule-pack dispositions escaped: {escaped:?}");
    }
    #[test]
    fn review_resolved_library_provenance_must_refuse() {
        let model = witness_model();
        let store = store("library-conflict", &model, true);
        let mut library = material_library();
        library["material_library"]["redistribution_status"] = json!("public_permissive");
        library["material_library"]["provenance"]["redistribution_status"] = json!("protected_suspected");
        crate::upsert_library(&Connection::open(&store.path).unwrap(), "project:slots", "material", "library:invented-materials", &library).unwrap();
        let state = save_state(&store, &model, "handoff_basis", "12340000123400001234000012340000").state;
        let result = prepare(&store, &model, &state.state_id);
        if let Ok(prepared) = result {
            println!("ESCAPED library provenance: {}", prepared.library_refs[0]);
            review_emit("library-conflict", prepared);
            panic!("protected library provenance was serialized into a successful package");
        }
        let refused = result.unwrap_err();
        assert_eq!((refused[0].code.as_str(), refused[0].path.as_str()), (model_document_save::PROTECTED_CONTENT_BLOCKED, "materials[0].provenance_record.library_ref"));
        assert!(refused[0].message.contains("material:library:invented-materials") && refused[0].message.contains("/material_library/provenance/redistribution_status protected_suspected"), "{}", refused[0].message);
    }
    #[test]
    fn review_produce_real_packages_and_unsafe_binary64() {
        let demo: Value = serde_json::from_str(DEMO).unwrap();
        let milestone: Value = serde_json::from_str(include_str!("../../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.desktop_model.json")).unwrap();
        for (tag, mut model, refs) in [
            ("demo", demo.clone(), false),
            ("milestone", milestone.clone(), false),
            ("library-applied", witness_model(), true),
            ("refs-assumptions", witness_model(), true),
            ("unsafe-binary64", demo.clone(), false),
        ] {
            if tag == "library-applied" { model["rule_pack_refs"] = json!([]); }
            if tag == "unsafe-binary64" { model["materials"][0]["elastic_modulus"]["value"] = json!(1e20); }
            let store = store(tag, &model, refs);
            let state = save_state(&store, &model, "handoff_basis", "12340000123400001234000012340000").state;
            match prepare(&store, &model, &state.state_id) {
                Ok(prepared) => review_emit(tag, prepared),
                Err(errors) => {
                    fs::write(review_output().join(format!("{tag}-refusal.json")), serde_json::to_vec(&errors).unwrap()).unwrap();
                    println!("PRODUCER REFUSAL {tag}: {errors:?}");
                    assert_eq!(tag, "milestone", "unexpected refusal");
                }
            }
        }
    }
    #[test]
    fn review_destination_changed_to_nonpackage_must_not_be_swapped() {
        let (store, first) = prepared_demo("target-race");
        let target = store.root.join("target.swbhandoff");
        assert!(matches!(write(&target, first), HandoffSaveOutcome::Saved { .. }));
        let before = || {
            fs::rename(&target, store.root.join("retained-old.swbhandoff"))?;
            fs::create_dir(&target)?;
            fs::write(target.join("unrelated.txt"), b"unrelated user data")
        };
        let hooks = PublishHooks { before_commit: &before, ..PRODUCTION_HOOKS };
        let (_, second) = prepared_demo("target-race-new");
        let outcome = write_handoff_package(&target, second, CREATED_AT, &hooks);
        println!("RACE OUTCOME {outcome:?}; target still unrelated: {}", target.join("unrelated.txt").exists());
        assert!(matches!(&outcome, HandoffSaveOutcome::Refused { diagnostics } if diagnostics[0].code == publish::DESTINATION_CHANGED), "a concurrently introduced non-package was replaced: {outcome:?}");
        assert!(target.join("unrelated.txt").exists());
        assert_eq!(fs::read(target.join("unrelated.txt")).unwrap(), b"unrelated user data");
        assert!(verify_handoff_package(&store.root.join("retained-old.swbhandoff")).is_ok(), "the previous package is untouched");
        assert!(staging_left(&store.root).is_empty(), "{:?}", staging_left(&store.root));
    }
    #[test]
    fn review_verifier_checks_subject_identity() {
        let (store, prepared) = prepared_demo("forged-subject");
        let mut manifest = handoff_manifest(&prepared, &PackageStamp { package_name: "forged.swbhandoff", created_at: CREATED_AT });
        manifest.get_mut("model_hash").unwrap()["payload_ref"] = reference("RulePack", "rule-pack:unrelated");
        let (text, _) = publish::seal_manifest(manifest, &prepared.model_hex);
        let target = store.root.join("forged.swbhandoff");
        let result = publish::publish_package(&target, prepared.model_text.as_bytes(), text.as_bytes(), &PRODUCTION_HOOKS);
        println!("FORGED SUBJECT PUBLISH {result:?}");
        let keep = review_output().join("forged-subject.swbhandoff");
        if result.is_ok() {
            fs::create_dir_all(keep.join("model")).unwrap();
            fs::write(keep.join("model/model.json"), &prepared.model_text).unwrap();
            fs::write(keep.join("handoff_package.json"), &text).unwrap();
        }
        assert!(result.is_err(), "model checksum claiming to hash a RulePack must not verify");
    }
    #[test]
    fn review_grammar_must_come_from_verified_stamped_pack() {
        let model = witness_model();
        let store = store("stale-stamp", &model, true);
        let mut pack: Value = serde_json::from_str(DEMO_RULE_PACK).unwrap();
        pack["grammar_version"] = json!("9.9.9");
        assert_ne!(open_pipe_stress_rule_pack_document::compute_rule_pack_checksum(&pack).unwrap().value, pack["checksums"]["rule_pack_checksum"]["value"].as_str().unwrap());
        // The normal store command keeps editable drafts, including invalid ones.
        crate::save_local_rule_pack_value(|| Ok(store.path.clone()), "project:slots".to_string(), pack).unwrap();
        let state = save_state(&store, &model, "handoff_basis", "12340000123400001234000012340000").state;
        let prepared = prepare(&store, &model, &state.state_id).unwrap();
        println!("STALE GRAMMAR RESOLUTION warnings: {:?}; rule ref {}", prepared.save_warnings, prepared.rule_pack_refs[0]);
        review_emit("stale-grammar", prepared);
        let mut packs = |project: &str, id: &str| crate::stored_rule_packs_with_id(&Connection::open(&store.path).unwrap(), project, id);
        let (_, warnings) = rule_pack_refs(&model, "project:slots", &mut packs).unwrap();
        // aligned with #1602's exclusion codes (export-diagnostics PR)
        assert!(warnings.iter().any(|w| w.code == RULE_PACK_REF_CHECKSUM_MISMATCH), "invalid stamped checksum was treated as a resolved pack");
    }
    /// W-6b resolves a model rule-pack reference as the solve does (#1602): every
    /// shared vector gives the outcome TypeScript's `resolveModelRulePacks` gives
    /// (`src/services/rulePackRefResolutionParity.test.ts`), through the store
    /// lookup the save command wires and through `rule_pack_ref_exclusion` alone.
    #[test]
    fn rule_pack_ref_resolution_matches_the_solve_on_the_shared_vectors() {
        let vectors: Value = serde_json::from_str(include_str!("../../../../fixtures/handoff/rule_pack_ref_resolution_vectors.json")).unwrap();
        let base: Value = serde_json::from_str(DEMO_RULE_PACK).unwrap();
        let project = vectors["model_project_id"].as_str().unwrap();
        let cases = vectors["vectors"].as_array().unwrap();
        assert!(cases.len() >= 11);
        for case in cases {
            let name = case["name"].as_str().unwrap();
            let reference = case["reference"].as_object().unwrap();
            let rule_pack_id = reference["rule_pack_id"].as_str().unwrap();
            let root = sandbox("rule-pack-resolution");
            let (connection, _) = crate::open_project_store(&root.join(crate::PROJECT_STORE_FILE)).unwrap();
            if let Some(stored_under) = case["stored_under"].as_str() {
                let mut document = base.clone();
                for edit in case["document_edits"].as_array().unwrap() {
                    let pointer = edit["pointer"].as_str().unwrap();
                    let (parent, key) = pointer.rsplit_once('/').unwrap();
                    let object = document.pointer_mut(parent).unwrap().as_object_mut().unwrap();
                    if edit["delete"] == json!(true) { object.remove(key); } else { object.insert(key.to_string(), edit["value"].clone()); }
                }
                document["checksums"]["rule_pack_checksum"]["value"] = case["stamp"].clone();
                let validation = open_pipe_stress_rule_pack_document::validate_rule_pack_document(&document, false);
                assert_eq!(json!(validation.checksum.match_status), case["match_status"], "{name}: the vector's match_status is the validator's");
                crate::upsert_rule_pack(&connection, stored_under, rule_pack_id, &document).unwrap();
            }
            let expected = case["expected"].as_str().unwrap();
            let held = crate::stored_rule_packs_with_id(&connection, project, rule_pack_id).unwrap();
            assert!(held.len() <= 1, "{name}");
            assert_eq!(rule_pack_ref_exclusion(reference, held.first()).unwrap_or("resolved"), expected, "{name}");
            let model = json!({ "rule_pack_refs": [Value::Object(reference.clone())] });
            let mut lookup = |project: &str, id: &str| crate::stored_rule_packs_with_id(&connection, project, id);
            let (refs, warnings) = rule_pack_refs(&model, project, &mut lookup).unwrap();
            assert_eq!(refs.len(), 1, "{name}: an excluded reference stays listed");
            let codes: Vec<&str> = warnings.iter().map(|warning| warning.code.as_str()).collect();
            assert_eq!(codes, if expected == "resolved" { vec![] } else { vec![expected] }, "{name}");
            drop(connection);
            fs::remove_dir_all(&root).unwrap();
        }
    }
    #[test]
    fn review_publication_failures_and_symlinks() {
        use std::os::unix::fs::symlink;
        let (store, prepared) = prepared_demo("more-failures");
        let target = store.root.join("target.swbhandoff");
        assert!(matches!(write(&target, prepared), HandoffSaveOutcome::Saved { .. }));
        let original = package_bytes(&target);
        let token_fail = || -> io::Result<String> { Err(io::Error::other("token failure")) };
        let swap_fail = |_: &Path, _: &Path| -> io::Result<()> { Err(io::Error::other("swap failure")) };
        for hooks in [PublishHooks { token: &token_fail, ..PRODUCTION_HOOKS }, PublishHooks { swap: Some(&swap_fail), ..PRODUCTION_HOOKS }] {
            let (_, prepared) = prepared_demo("more-failures-payload");
            assert!(matches!(write_handoff_package(&target, prepared, CREATED_AT, &hooks), HandoffSaveOutcome::Refused { .. }));
            assert_eq!(package_bytes(&target), original);
        }
        for stage in ["manifest-write", "sync-model-dir", "verify"] {
            let token = || Ok(format!("review-{stage}"));
            let staging = store.root.join(format!("target.swbhandoff.staging-review-{stage}"));
            let alter = || match stage {
                "manifest-write" => fs::create_dir(staging.join(MANIFEST_MEMBER)),
                "sync-model-dir" => fs::rename(staging.join(MODEL_DIRECTORY), staging.join("renamed-model")),
                _ => fs::write(staging.join(MODEL_DIRECTORY).join(MODEL_FILE), b"{}"),
            };
            let hooks = PublishHooks { token: &token, after_model_member: &alter, ..PRODUCTION_HOOKS };
            let (_, prepared) = prepared_demo("more-failures-payload");
            assert!(matches!(write_handoff_package(&target, prepared, CREATED_AT, &hooks), HandoffSaveOutcome::Refused { .. }));
            assert_eq!(package_bytes(&target), original, "{stage}");
        }
        let link = store.root.join("linked.swbhandoff");
        symlink(&target, &link).unwrap();
        let (_, prepared) = prepared_demo("symlink-payload");
        assert!(matches!(write(&link, prepared), HandoffSaveOutcome::Refused { .. }));
        let linked_parent = store.root.join("linked-parent");
        symlink(&store.root, &linked_parent).unwrap();
        let (_, prepared) = prepared_demo("symlink-parent-payload");
        assert!(matches!(write(&linked_parent.join("target.swbhandoff"), prepared), HandoffSaveOutcome::Refused { .. }));
        assert_eq!(package_bytes(&target), original);
    }
    #[test]
    fn review_concurrent_saves_are_complete() {
        use std::sync::Barrier;
        for replacing in [false, true] {
            let (store, first) = prepared_demo("concurrent-root");
            let target = store.root.join("target.swbhandoff");
            if replacing { assert!(matches!(write(&target, first), HandoffSaveOutcome::Saved { .. })); }
            let (_left, l) = prepared_demo("concurrent-left");
            let (_right, r) = prepared_demo("concurrent-right");
            let barrier = Barrier::new(2);
            let outcomes = std::thread::scope(|s| {
                let target_ref = &target; let barrier_ref = &barrier;
                let a = s.spawn(move || { let before = || { barrier_ref.wait(); Ok(()) }; write_handoff_package(target_ref, l, CREATED_AT, &PublishHooks { before_commit: &before, ..PRODUCTION_HOOKS }) });
                let b = s.spawn(move || { let before = || { barrier_ref.wait(); Ok(()) }; write_handoff_package(target_ref, r, CREATED_AT, &PublishHooks { before_commit: &before, ..PRODUCTION_HOOKS }) });
                (a.join().unwrap(), b.join().unwrap())
            });
            println!("CONCURRENT replacing={replacing}: {outcomes:?}");
            let saved = [&outcomes.0, &outcomes.1].into_iter().filter(|o| matches!(o,HandoffSaveOutcome::Saved { .. })).count();
            // Review round 1, finding 4: a replacement whose destination another save
            // replaced first is refused by name (both may land when they serialize).
            if replacing { assert!(saved >= 1, "{outcomes:?}") } else { assert_eq!(saved, 1) }
            for outcome in [&outcomes.0, &outcomes.1] {
                if let HandoffSaveOutcome::Refused { diagnostics } = outcome {
                    let wanted = if replacing { publish::DESTINATION_CHANGED } else { publish::DESTINATION_APPEARED };
                    assert_eq!(diagnostics[0].code, wanted, "{outcomes:?}");
                }
            }
            assert!(verify_handoff_package(&target).is_ok());
            assert!(staging_left(&store.root).is_empty());
        }
    }
    #[test]
    fn review_member_substitution_must_not_keep_old_model_identity() {
        let (store, prepared) = prepared_demo("substitution");
        let mut other: Value = serde_json::from_str(DEMO).unwrap();
        other["project"]["id"] = json!("project:unrelated-model");
        other["nodes"][0]["id"] = json!("node:unrelated-node");
        let text = canonical_json(&other); let model_hex = sha256_hex(&text);
        let mut manifest = handoff_manifest(&prepared, &PackageStamp { package_name: "substituted.swbhandoff", created_at: CREATED_AT });
        manifest.get_mut("model_hash").unwrap()["value"] = json!(model_hex);
        let (manifest_text, _) = publish::seal_manifest(manifest, &model_hex);
        let target = review_output().join("substituted-model.swbhandoff");
        let result = publish::publish_package(&target, text.as_bytes(), manifest_text.as_bytes(), &PRODUCTION_HOOKS);
        println!("SUBSTITUTED MEMBER {result:?}; old project {}, model project {}", prepared.project_id, other["project"]["id"]);
        assert!(result.is_err(), "a re-inventoried different model must not verify against the previous model identity, state refs and entity ids");
        drop(store);
    }

    /// Review round 1, finding 4, the residual window: a destination replaced
    /// after the last identity check and before the swap is detected in what the
    /// swap moved out; the swap is undone, the other folder keeps its name and
    /// contents, and nothing of it is moved away for deletion.
    #[cfg(target_os = "macos")]
    #[test]
    fn a_destination_replaced_inside_the_swap_window_is_swapped_back_untouched() {
        let (store, first) = prepared_demo("swap-window");
        let target = store.root.join("target.swbhandoff");
        assert!(matches!(write(&target, first), HandoffSaveOutcome::Saved { .. }));
        let moved = store.root.join("moved-old.swbhandoff");
        let racing_swap = |staging: &Path, destination: &Path| -> io::Result<()> {
            fs::rename(destination, &moved)?;
            fs::create_dir(destination)?;
            fs::write(destination.join("unrelated.txt"), b"unrelated user data")?;
            publish::platform_swap(staging, destination)
        };
        // Only the first swap races; the undo is the real swap.
        let raced = std::cell::Cell::new(false);
        let swap = |staging: &Path, destination: &Path| -> io::Result<()> {
            if raced.replace(true) { publish::platform_swap(staging, destination) } else { racing_swap(staging, destination) }
        };
        let (_, second) = prepared_demo("swap-window-new");
        let outcome = write_handoff_package(&target, second, CREATED_AT, &PublishHooks { swap: Some(&swap), ..PRODUCTION_HOOKS });
        assert!(matches!(&outcome, HandoffSaveOutcome::Refused { diagnostics } if diagnostics[0].code == publish::DESTINATION_CHANGED), "{outcome:?}");
        assert_eq!(fs::read(target.join("unrelated.txt")).unwrap(), b"unrelated user data");
        assert_eq!(fs::read_dir(&target).unwrap().count(), 1);
        assert!(verify_handoff_package(&moved).is_ok(), "the previous package is untouched");
        assert!(staging_left(&store.root).is_empty(), "{:?}", staging_left(&store.root));
    }

#[test]
fn round2_resolved_library_record_provenance_must_refuse() {
    let mut escapes = Vec::new();
    for (tag, field, status) in [("protected", "redistribution_status", "protected_suspected"), ("quarantined", "review_status", "quarantined"), ("rejected", "review_status", "rejected")] {
        let model = witness_model();
        let store = store(&format!("round2-record-{tag}"), &model, true);
        let mut library = material_library();
        library["material_records"][0]["redistribution_status"] = json!("public_permissive");
        library["material_records"][0]["review_status"] = json!("accepted");
        library["material_records"][0]["provenance"][field] = json!(status);
        crate::upsert_library(&Connection::open(&store.path).unwrap(), "project:slots", "material", "library:invented-materials", &library).unwrap();
        let state = save_state(&store, &model, "handoff_basis", "12340000123400001234000012340000").state;
        let result = prepare(&store, &model, &state.state_id);
        if let Ok(prepared) = result {
            let target = review_output().join(format!("record-{tag}.swbhandoff"));
            let outcome = write(&target, prepared);
            println!("ESCAPED RECORD {tag}: {outcome:?}");
            if matches!(outcome, HandoffSaveOutcome::Saved { .. }) { escapes.push(tag); }
        }
    }
    assert!(escapes.is_empty(), "blocked record provenances escaped: {escapes:?}");
}
#[test]
fn round2_cleanup_must_not_follow_model_directory_symlink() {
    use std::os::unix::fs::symlink;
    let (store, prepared) = prepared_demo("round2-cleanup");
    let target = store.root.join("target.swbhandoff");
    let unrelated = store.root.join("unrelated");
    fs::create_dir(&unrelated).unwrap();
    fs::write(unrelated.join("model.json"), b"unrelated user data").unwrap();
    let token = || Ok("round2-cleanup".to_string());
    let staging = store.root.join("target.swbhandoff.staging-round2-cleanup");
    let fail = || {
        fs::rename(staging.join("model"), store.root.join("retained-model"))?;
        symlink(&unrelated, staging.join("model"))?;
        Err(io::Error::other("interrupted publication"))
    };
    let outcome = write_handoff_package(&target, prepared, CREATED_AT, &PublishHooks {token: &token, before_commit: &fail, ..PRODUCTION_HOOKS});
    println!("CLEANUP {outcome:?}; unrelated survives={}", unrelated.join("model.json").exists());
    assert!(unrelated.join("model.json").exists(), "cleanup followed an intermediate symlink and deleted non-package data");
}
#[test]
fn round2_private_record_must_not_be_classified_public_by_its_library_wrapper() {
    let model=witness_model();let store=store("round2-private-record",&model,true);
    let mut library=material_library();
    library["material_records"][0]["provenance"]["redistribution_status"]=json!("private_only");
    crate::upsert_library(&Connection::open(&store.path).unwrap(),"project:slots","material","library:invented-materials",&library).unwrap();
    let state=save_state(&store,&model,"handoff_basis","12340000123400001234000012340000").state;
    let prepared=prepare(&store,&model,&state.state_id).unwrap();
    let reference=prepared.library_refs[0].clone();
    let outcome=write(&review_output().join("private-record.swbhandoff"),prepared);
    println!("PRIVATE RECORD CLASSIFICATION {reference}; outcome {outcome:?}");
    assert_eq!(reference["redistribution_status"],json!("private_only"));
}
#[test]
fn round2_nested_rule_pack_provenance_must_refuse() {
    let mut model=witness_model();let mut pack:Value=serde_json::from_str(DEMO_RULE_PACK).unwrap();
    let mut changed=0;
    fn mutate(v:&mut Value,changed:&mut usize) {
        match v {
            Value::Object(m)=>{
                for (k,w) in m {
                    if k=="provenance" && w.is_object() {
                        w["redistribution_status"]=json!("protected_suspected");*changed+=1;
                    } else {mutate(w,changed);}
                }
            },Value::Array(xs)=>for x in xs {mutate(x,changed)},_=>{}
        }
    }
    for (k,v) in pack.as_object_mut().unwrap() {if k!="provenance" {mutate(v,&mut changed)}}
    assert!(changed>0);
    let hash=open_pipe_stress_rule_pack_document::compute_rule_pack_checksum(&pack).unwrap().value;
    pack["checksums"]["rule_pack_checksum"]["value"]=json!(hash);
    model["rule_pack_refs"][0]["checksum"]["value"]=json!(hash);
    let store=store("round2-pack-nested",&model,true);
    crate::upsert_rule_pack(&Connection::open(&store.path).unwrap(),"project:slots","invented_demo_rule_pack",&pack).unwrap();
    let state=save_state(&store,&model,"handoff_basis","12340000123400001234000012340000").state;
    match prepare(&store,&model,&state.state_id) {
        Err(d)=>assert!(d.iter().any(|d|d.code==model_document_save::PROTECTED_CONTENT_BLOCKED)),
        Ok(prepared)=>{let outcome=write(&review_output().join("nested-pack.swbhandoff"),prepared);println!("ESCAPED {changed} NESTED RULE PROVENANCES {outcome:?}");assert!(!matches!(outcome,HandoffSaveOutcome::Saved{..}),"nested rule source disposition escaped")}
    }
}

#[test]
fn round2_layout_only_directory_must_not_be_treated_as_verified_handoff_package() {
    let (store, prepared)=prepared_demo("round2-layout-only");
    let target=store.root.join("target.swbhandoff");fs::create_dir_all(target.join("model")).unwrap();
    fs::write(target.join("handoff_package.json"),b"not a handoff manifest").unwrap();
    fs::write(target.join("model/model.json"),b"unrelated user data").unwrap();
    assert!(verify_handoff_package(&target).is_err());
    let outcome=write(&target,prepared);println!("LAYOUT ONLY OUTCOME {outcome:?}");
    assert!(matches!(outcome,HandoffSaveOutcome::Refused{..}),"an existing non-package with the expected filenames was deleted");
    assert_eq!(fs::read(target.join("model/model.json")).unwrap(),b"unrelated user data");
}
#[test]
fn round2_preexisting_symlinks_refuse_and_hardlinked_members_keep_other_links() {
    use std::os::unix::fs::symlink;
    for (tag,member) in [("target",""),("model","model"),("manifest","handoff_package.json"),("member","model/model.json")] {
        let (store,prepared)=prepared_demo(&format!("round2-link-{tag}"));let target=store.root.join("target.swbhandoff");
        assert!(matches!(write(&target,prepared),HandoffSaveOutcome::Saved{..}));
        let entry=if member.is_empty() { target.clone() } else { target.join(member) };let saved=store.root.join("saved-entry");fs::rename(&entry,&saved).unwrap();symlink(&saved,&entry).unwrap();
        let (_other,next)=prepared_demo("round2-link-next");let outcome=write(&target,next);
        assert!(matches!(outcome,HandoffSaveOutcome::Refused{..}),"{tag}: {outcome:?}");assert!(saved.exists());
    }
    let (store,first)=prepared_demo("round2-hardlink");let target=store.root.join("target.swbhandoff");
    assert!(matches!(write(&target,first),HandoffSaveOutcome::Saved{..}));
    let kept=store.root.join("kept-model.json");fs::hard_link(target.join("model/model.json"),&kept).unwrap();let original=fs::read(&kept).unwrap();
    let (_other,next)=prepared_demo("round2-hardlink-next");assert!(matches!(write(&target,next),HandoffSaveOutcome::Saved{..}));assert_eq!(fs::read(&kept).unwrap(),original);
}
#[test]
fn round2_concurrent_saves_stress() { for _ in 0..30 { review_concurrent_saves_are_complete(); } }

#[test]
fn round2_rollback_failure_leaves_both_directories() {
    let (store,first)=prepared_demo("round2-rollback");let target=store.root.join("target.swbhandoff");
    assert!(matches!(write(&target,first),HandoffSaveOutcome::Saved{..}));
    let count=std::cell::Cell::new(0);
    let swap=|staging:&Path,destination:&Path| -> io::Result<()> {
        let n=count.get();count.set(n+1);
        if n>0 {return Err(io::Error::other("injected swap-back failure"));}
        fs::rename(destination,store.root.join("retained-old"))?;fs::create_dir(destination)?;fs::write(destination.join("unrelated.txt"),b"unrelated user data")?;
        publish::platform_swap(staging,destination)
    };
    let token=||Ok("round2-rollback".to_string());
    let (_other,next)=prepared_demo("round2-rollback-next");let result=write_handoff_package(&target,next,CREATED_AT,&PublishHooks{swap:Some(&swap),token:&token,..PRODUCTION_HOOKS});
    assert!(matches!(result,HandoffSaveOutcome::Refused{..}));
    let staging=store.root.join("target.swbhandoff.staging-round2-rollback");
    assert_eq!(fs::read(staging.join("unrelated.txt")).unwrap(),b"unrelated user data");
    assert!(verify_handoff_package(&target).is_ok());
    assert!(verify_handoff_package(&store.root.join("retained-old")).is_ok());
}
#[test]
fn round2_ingress_larger_and_malformed_arguments_stay_within_the_lease() {
    for n in [4096,1024*1024,4*1024*1024] {
        let text="a".repeat(n);
        let body=json!({"canonicalText":text,"claimedModelHash":hash_of(&text),"screening":serde_json::to_value(screening(&hash_of(&text))).unwrap(),"localFirstEvidence":serde_json::to_value(local_first()).unwrap(),"basisStateId":"state:x","packageLabel":"Invented"});
        for invalid in [false,true] {
            let mut args=body.clone();if invalid {args["screening"]["blocking_count"]=json!("s".repeat(n));}
            let raw=serde_json::to_vec(&args).unwrap();
            let (result,_,_)=check_decode_bound(&raw,ArgsDecoding::Moved,|args|save_handoff_package_args(args).map(drop));
            assert_eq!(result.is_err(),invalid);
        }
    }
}
#[test]
fn round2_nested_model_provenance_must_not_be_hidden_by_own_fields() {
    let mut model=witness_model();
    model["materials"][0]["provenance_record"]["redistribution_status"]=json!("public_permissive");
    model["materials"][0]["provenance_record"]["provenance"]=json!({"redistribution_status":"protected_suspected","review_status":"quarantined"});
    let store=store("round2-model-provenance",&model,true);
    let state=save_state(&store,&model,"handoff_basis","12340000123400001234000012340000").state;
    let result=prepare(&store,&model,&state.state_id);
    assert!(result.is_err(),"nested model provenance was hidden by own public disposition");
}


    // -----------------------------------------------------------------------
    // Review round 2, finding 2: the structural screen, by property.
    // -----------------------------------------------------------------------

    /// A deterministic xorshift generator (the cases are reproducible by seed).
    struct Xorshift(u64);

    impl Xorshift {
        fn next(&mut self) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0
        }

        fn below(&mut self, bound: usize) -> usize {
            (self.next() % bound as u64) as usize
        }
    }

    /// Every object's JSON pointer in `value`, and every array's.
    fn containers(value: &Value, pointer: &str, objects: &mut Vec<String>, arrays: &mut Vec<String>) {
        match value {
            Value::Object(object) => {
                objects.push(pointer.to_string());
                for (key, child) in object {
                    containers(child, &format!("{pointer}/{}", pointer_token(key)), objects, arrays);
                }
            }
            Value::Array(items) => {
                arrays.push(pointer.to_string());
                for (index, item) in items.iter().enumerate() {
                    containers(item, &format!("{pointer}/{index}"), objects, arrays);
                }
            }
            _ => {}
        }
    }

    /// Plants one blocking disposition at a random object of `document`, in one
    /// of five positions (the object's own member, a new sibling object, its
    /// nested `provenance`, a new array element, a nested `provenance_record`'s
    /// provenance); returns the pointer and value the screen must name.
    fn plant(document: &mut Value, random: &mut Xorshift, serial: usize) -> String {
        let (member, blocking) = SCREENED_MEMBERS[random.below(SCREENED_MEMBERS.len())];
        let status = blocking[random.below(blocking.len())];
        let (mut objects, mut arrays) = (Vec::new(), Vec::new());
        containers(document, "", &mut objects, &mut arrays);
        let at = objects[random.below(objects.len())].clone();
        let object = document.pointer_mut(&at).unwrap().as_object_mut().unwrap();
        let planted = match random.below(5) {
            0 => {
                object.insert(member.to_string(), json!(status));
                format!("{at}/{member}")
            }
            1 => {
                let key = format!("planted_sibling_{serial}");
                object.insert(key.clone(), json!({ member: status }));
                format!("{at}/{key}/{member}")
            }
            2 => match object.get_mut("provenance").and_then(Value::as_object_mut) {
                Some(provenance) => {
                    provenance.insert(member.to_string(), json!(status));
                    format!("{at}/provenance/{member}")
                }
                None => {
                    let key = format!("planted_{serial}");
                    object.insert(key.clone(), json!({ "provenance": { member: status } }));
                    format!("{at}/{key}/provenance/{member}")
                }
            },
            3 if !arrays.is_empty() => {
                let array = arrays[random.below(arrays.len())].clone();
                let items = document.pointer_mut(&array).unwrap().as_array_mut().unwrap();
                items.push(json!({ "name": "planted", member: status }));
                format!("{array}/{}/{member}", items.len() - 1)
            }
            _ => {
                let key = format!("planted_record_{serial}");
                object.insert(key.clone(), json!({ "provenance_record": { "provenance": { member: status } } }));
                format!("{at}/{key}/provenance_record/provenance/{member}")
            }
        };
        format!("{planted} {status}")
    }

    #[test]
    fn blocking_dispositions_planted_anywhere_in_a_stored_document_are_found_by_pointer() {
        let pack: Value = serde_json::from_str(DEMO_RULE_PACK).unwrap();
        assert!(screen_document(&material_library()).is_empty() && screen_document(&pack).is_empty(), "the bases block nothing");
        let mut random = Xorshift(0x5eed_0002_6b0d_2026);
        for case in 0..4000 {
            let mut document = if case % 2 == 0 { material_library() } else { pack.clone() };
            let mut wanted: Vec<String> = (0..1 + random.below(3)).map(|serial| plant(&mut document, &mut random, serial)).collect();
            // A later plant can overwrite an earlier one's member.
            wanted.retain(|entry| {
                let (pointer, status) = entry.rsplit_once(' ').unwrap();
                document.pointer(pointer).and_then(Value::as_str) == Some(status)
            });
            wanted.sort();
            wanted.dedup();
            let found = screen_document(&document);
            for entry in &wanted {
                assert!(found.contains(entry), "case {case}: {entry} was not found in {found:?}\n{document}");
            }
            assert_eq!(found.len(), wanted.len(), "case {case}: {found:?} against {wanted:?}");
        }
    }

    #[test]
    fn blocking_dispositions_planted_anywhere_in_a_stored_library_or_rule_pack_refuse_the_save() {
        let mut random = Xorshift(0x5eed_0002_6b0d_e2e0);
        let mut packs_resolved = 0;
        for case in 0..48 {
            let mut model = witness_model();
            let library_case = case % 2 == 0;
            let (mut document, what) = if library_case { (material_library(), "stored library") } else { (serde_json::from_str(DEMO_RULE_PACK).unwrap(), "stored rule pack") };
            let planted = plant(&mut document, &mut random, 0);
            let store = store(&format!("round2-planted-{case}"), &model, true);
            if library_case {
                crate::upsert_library(&Connection::open(&store.path).unwrap(), "project:slots", "material", "library:invented-materials", &document).unwrap();
            } else {
                // The planted pack still resolves when its document hashes to the stamp.
                let Ok(hash) = open_pipe_stress_rule_pack_document::compute_rule_pack_checksum(&document).map(|checksum| checksum.value) else { continue };
                document["checksums"]["rule_pack_checksum"]["value"] = json!(hash);
                model["rule_pack_refs"][0]["checksum"]["value"] = json!(hash);
                crate::upsert_rule_pack(&Connection::open(&store.path).unwrap(), "project:slots", "invented_demo_rule_pack", &document).unwrap();
                packs_resolved += 1;
            }
            let state = save_state(&store, &model, "handoff_basis", "12340000123400001234000012340000").state;
            match prepare(&store, &model, &state.state_id) {
                Err(refusals) => assert!(
                    refusals.iter().any(|refusal| refusal.code == model_document_save::PROTECTED_CONTENT_BLOCKED
                        && (refusal.message.contains(&planted) || refusal.message.contains(planted.rsplit_once(' ').unwrap().1))),
                    "case {case} ({what}, {planted}): {refusals:?}"
                ),
                Ok(_) => panic!("case {case}: {planted} in the {what} was not refused"),
            }
        }
        assert!(packs_resolved >= 12, "only {packs_resolved} planted packs resolved");
    }

    /// Review round 2, note 7: the basis state is read again before publication.
    #[test]
    fn the_basis_state_is_read_again_from_the_store_before_publication() {
        let (store, prepared) = prepared_demo("round2-basis-recheck");
        let project = prepared.project_id.clone();
        let mut states = |project: &str, state: &str| stored_state(&store.path, project, state, &crate::stored_read::UnmeteredReadMemory);
        assert!(confirm_basis_state(&prepared, &mut states).is_ok());
        // A record hash that is no longer the stored one is refused.
        let mut stale = prepared.clone();
        stale.basis.record_hash_hex = "0".repeat(64);
        let refused = confirm_basis_state(&stale, &mut states).unwrap_err();
        assert_eq!(refused[0].code, BASIS_STATE_MISMATCH, "{refused:?}");
        // A state deleted since preparation is refused.
        crate::model_states::delete_state_in_store(&store.path, &project, &prepared.basis.state_id).unwrap();
        let refused = confirm_basis_state(&prepared, &mut states).unwrap_err();
        assert_eq!(refused[0].code, BASIS_STATE_MISSING, "{refused:?}");
    }

    /// Review round 2, should-fix 5: both verifiers give every maintained
    /// differential case the pinned verdict, on the same bytes
    /// (`tests/test_handoff_verifier_parity.py` documents the cases and edits).
    #[test]
    fn verifier_parity_cases_match_the_python_twin() {
        const CASES: &str = include_str!("../../../../fixtures/handoff/verifier_parity_cases.json");
        fn resolve<'a>(document: &'a mut Value, pointer: &str) -> (&'a mut Value, String) {
            let (parent, last) = pointer.rsplit_once('/').expect("a pointer");
            let last = last.replace("~1", "/").replace("~0", "~");
            (document.pointer_mut(parent).unwrap_or_else(|| panic!("no parent of {pointer}")), last)
        }
        fn apply(document: &mut Value, edits: &[Value], model_hex: &str, project_id: &str) {
            for edit in edits {
                let edit = edit.as_array().unwrap();
                let (pointer, op) = (edit[0].as_str().unwrap(), edit[1].as_str().unwrap());
                let units_hash = (op == "units_hash").then(|| {
                    let mut block = document["handoff_package_manifest"]["units_manifest"].as_object().unwrap().clone();
                    block.remove("hash_refs");
                    sha256_hex(&canonical_json(&Value::Object(block)))
                });
                let (parent, key) = resolve(document, pointer);
                let slot = match parent {
                    Value::Array(items) => &mut items[key.parse::<usize>().unwrap()],
                    Value::Object(object) => object.entry(key).or_insert(Value::Null),
                    other => panic!("{pointer}: {other} is not a container"),
                };
                *slot = match op {
                    "set" => edit[2].clone(),
                    "append_x" => json!(format!("{}x", slot.as_str().unwrap())),
                    "negate" => json!(!slot.as_bool().unwrap()),
                    "double" => {
                        let mut items = slot.as_array().unwrap().clone();
                        items.extend(items.clone());
                        Value::Array(items)
                    }
                    "model_hex" => json!(model_hex),
                    "handoff_id" => json!(format!("handoff:{}", package_token(project_id, model_hex))),
                    "manifest_id" => json!(format!("manifest:{}", package_token(project_id, model_hex))),
                    "units_hash" => json!(units_hash.unwrap()),
                    other => panic!("unknown edit {other}"),
                };
            }
        }
        let cases: Value = serde_json::from_str(CASES).unwrap();
        let cases = cases["cases"].as_array().unwrap();
        assert!(cases.len() > 2900, "{} cases", cases.len());
        let root = sandbox("parity");
        let mut disagreements = Vec::new();
        for case in cases {
            let case = case.as_array().unwrap();
            let (name, edits, model_edits, seal, digest, accepted) =
                (case[0].as_str().unwrap(), case[1].as_array().unwrap(), case[2].as_array().unwrap(), case[3].as_bool().unwrap(), case[4].as_str().unwrap(), case[5].as_bool().unwrap());
            let mut model: Value = serde_json::from_str(WITNESS_MODEL).unwrap();
            apply(&mut model, model_edits, "", "");
            let model_text = canonical_json(&model);
            let model_hex = sha256_hex(&model_text);
            let project_id = model["project"]["id"].as_str().unwrap_or("").to_string();
            let mut package: Value = serde_json::from_str(WITNESS_MANIFEST).unwrap();
            apply(&mut package, edits, &model_hex, &project_id);
            if seal {
                let manifest = package["handoff_package_manifest"].as_object().unwrap();
                let model_ref = manifest["model_hash"].get("payload_ref").cloned().unwrap_or(Value::Null);
                let package_id = manifest["package_identity"]["handoff_package_id"].clone();
                let mut inventory = publish::checksum_inventory(&model_ref, package_id.as_str().unwrap_or(""), &model_hex, &publish::export_hash_hex(manifest));
                inventory[1]["payload_ref"]["ref"] = package_id;
                package["handoff_package_manifest"]["checksums"] = inventory;
            }
            let manifest_text = canonical_json(&package);
            let built = &sha256_hex(&format!("{model_text}\n{manifest_text}"))[..16];
            let target = root.join(format!("{name}.swbhandoff"));
            fs::create_dir_all(target.join(MODEL_DIRECTORY)).unwrap();
            fs::write(target.join(MODEL_DIRECTORY).join(MODEL_FILE), &model_text).unwrap();
            fs::write(target.join(MANIFEST_MEMBER), &manifest_text).unwrap();
            let verdict = verify_handoff_package(&target).is_ok();
            if built != digest || verdict != accepted {
                disagreements.push(format!("{name}: bytes {built} (pinned {digest}), verdict {verdict} (pinned {accepted})"));
            }
        }
        let _ = fs::remove_dir_all(&root);
        assert!(disagreements.is_empty(), "{} of {} cases disagree with the Python twin: {:?}", disagreements.len(), cases.len(), &disagreements[..disagreements.len().min(20)]);
    }
#[test]
fn round3_new_destination_must_not_publish_substituted_staging() {
    let mut escaped = Vec::new();
    for mode in ["folder", "symlink", "model-file"] {
        let (store, prepared) = prepared_demo(&format!("round3-stage-{mode}"));
        let target = store.root.join("target.swbhandoff");
        let staging = store.root.join("target.swbhandoff.staging-round3-stage");
        let retained = store.root.join("retained-stage");
        let token = || Ok("round3-stage".to_string());
        let before = || {
            if mode == "model-file" {
                fs::rename(staging.join("model"), store.root.join("retained-model"))?;
                fs::write(staging.join("model"), b"unrelated file")?;
            } else {
                fs::rename(&staging, &retained)?;
                if mode == "symlink" {
                    std::os::unix::fs::symlink(&retained, &staging)?;
                } else {
                    fs::create_dir(&staging)?;
                    fs::write(staging.join("unrelated.txt"), b"unrelated folder")?;
                }
            }
            Ok(())
        };
        let outcome = write_handoff_package(&target, prepared, CREATED_AT, &PublishHooks { token: &token, before_commit: &before, ..PRODUCTION_HOOKS });
        println!("STAGE_SUBSTITUTION {mode}: {outcome:?}; verifies={:?}", verify_handoff_package(&target));
        if matches!(outcome, HandoffSaveOutcome::Saved {..}) { escaped.push(mode); }
    }
    assert!(escaped.is_empty(), "substituted staging published as Saved: {escaped:?}");
}

#[test]
fn round3_private_library_privacy_must_not_become_public_metadata() {
    let model = witness_model();
    let store = store("round3-private-privacy", &model, true);
    let mut library: Value = serde_json::from_str(include_str!("../../../../fixtures/material/invented_material_library_valid.json")).unwrap();
    fn reviewed_public(v: &mut Value) {
        match v {
            Value::Object(m) => for (k,v) in m.iter_mut() { if k == "redistribution_status" { *v=json!("public_permissive"); } else if k == "review_status" { *v=json!("accepted"); } else { reviewed_public(v); } },
            Value::Array(a) => for v in a {reviewed_public(v)}, _ => {}
        }
    }
    reviewed_public(&mut library);
    library["material_library"]["library_id"] = json!("library:invented-materials");
    library["material_records"][0]["material_id"] = json!("steel");
    library["material_records"][0]["privacy_class"] = json!("private_user_data");
    if let Ok(evidence) = std::env::var("W6B_REVIEW_EVIDENCE") { fs::write(PathBuf::from(evidence).join("private-library-input.json"), serde_json::to_vec_pretty(&library).unwrap()).unwrap(); }
    crate::upsert_library(&Connection::open(&store.path).unwrap(), "project:slots", "material", "library:invented-materials", &library).unwrap();
    let state = save_state(&store, &model, "handoff_basis", "12340000123400001234000012340000").state;
    let prepared = prepare(&store, &model, &state.state_id).unwrap();
    println!("PRIVATE_LIBRARY_REFERENCE {}", prepared.library_refs[0]);
    assert_eq!(prepared.library_refs[0]["privacy_classification"], json!("private_library_data"));
}

#[test]
fn round3_private_pack_privacy_must_not_become_public_metadata() {
    let mut model = witness_model();
    let store = store("round3-private-pack", &model, true);
    let mut pack: Value = serde_json::from_str(DEMO_RULE_PACK).unwrap();
    pack["classification"]["privacy_class"] = json!("private_user_data");
    let hash = open_pipe_stress_rule_pack_document::compute_rule_pack_checksum(&pack).unwrap().value;
    pack["checksums"]["rule_pack_checksum"]["value"] = json!(hash);
    model["rule_pack_refs"][0]["checksum"]["value"] = json!(hash);
    crate::upsert_rule_pack(&Connection::open(&store.path).unwrap(), "project:slots", "invented_demo_rule_pack", &pack).unwrap();
    let state = save_state(&store, &model, "handoff_basis", "12340000123400001234000012340000").state;
    let prepared = prepare(&store, &model, &state.state_id).unwrap();
    println!("PRIVATE_PACK_REFERENCE {}", prepared.rule_pack_refs[0]);
    assert_eq!(prepared.rule_pack_refs[0]["privacy_classification"], json!("private_rule_pack_data"));
}

#[test]
fn round3_replacement_must_check_what_it_published_before_deleting_old_package() {
    let (store, mut first) = prepared_demo("round3-swap-source");
    first.package_label = "The previous package".to_string();
    let target = store.root.join("target.swbhandoff");
    assert!(matches!(write(&target, first), HandoffSaveOutcome::Saved {..}));
    let old = package_bytes(&target);
    let retained = store.root.join("retained-stage");
    let swap = |staging: &Path, destination: &Path| {
        fs::rename(staging, &retained)?;
        fs::create_dir(staging)?;
        fs::write(staging.join("unrelated.txt"), b"substituted source")?;
        publish::platform_swap(staging, destination)
    };
    let (_other, next) = prepared_demo("round3-swap-source-next");
    let outcome = write_handoff_package(&target, next, CREATED_AT, &PublishHooks {swap: Some(&swap), ..PRODUCTION_HOOKS});
    println!("SWAP_SOURCE_SUBSTITUTION {outcome:?}; verifies={:?}; staging_left={:?}; old_manifest_length={}", verify_handoff_package(&target), staging_left(&store.root), old.0.len());
    assert!(matches!(outcome, HandoffSaveOutcome::Refused {..}), "a substituted source replaced a valid target and the previous package was removed");
}


    /// Review round 3, finding 1: a staging source substituted inside the
    /// commit itself (after every pre-commit check) is caught by verifying what
    /// was published; a new name is moved back, a replacement swapped back, and
    /// the previous package is never deleted. Three shapes: another folder, a
    /// symlink to the real staging folder, `model/` replaced by a file.
    #[test]
    fn a_source_substituted_inside_the_commit_is_refused_and_the_previous_package_kept() {
        fn substitute(mode: &str, staging: &Path, retained: &Path) -> io::Result<()> {
            if mode == "model-file" {
                fs::rename(staging.join("model"), retained)?;
                return fs::write(staging.join("model"), b"unrelated file");
            }
            fs::rename(staging, retained)?;
            if mode == "symlink" {
                std::os::unix::fs::symlink(retained, staging)
            } else {
                fs::create_dir(staging)?;
                fs::write(staging.join("unrelated.txt"), b"unrelated folder")
            }
        }
        for mode in ["folder", "symlink", "model-file"] {
            // A new name.
            let (store, prepared) = prepared_demo(&format!("round3-commit-new-{mode}"));
            let target = store.root.join("target.swbhandoff");
            let retained = store.root.join("retained");
            let rename = |staging: &Path, destination: &Path| {
                substitute(mode, staging, &retained)?;
                fs::rename(staging, destination)
            };
            let outcome = write_handoff_package(&target, prepared, CREATED_AT, &PublishHooks { rename_new: Some(&rename), ..PRODUCTION_HOOKS });
            assert!(matches!(&outcome, HandoffSaveOutcome::Refused { diagnostics } if diagnostics[0].code == publish::PUBLISH_UNVERIFIED), "new {mode}: {outcome:?}");
            // Nothing but the verified previous package is ever moved or
            // deleted (review round 4): the substitute is left where it is.
            assert!(fs::symlink_metadata(&target).is_ok() && fs::symlink_metadata(&retained).is_ok(), "new {mode}: something was moved or deleted");

            // A replacement.
            let (store, first) = prepared_demo(&format!("round3-commit-swap-{mode}"));
            let target = store.root.join("target.swbhandoff");
            assert!(matches!(write(&target, first), HandoffSaveOutcome::Saved { .. }));
            let old = package_bytes(&target);
            let retained = store.root.join("retained");
            let calls = std::cell::Cell::new(0);
            let swap = |staging: &Path, destination: &Path| {
                calls.set(calls.get() + 1);
                if calls.get() == 1 {
                    substitute(mode, staging, &retained)?;
                }
                publish::platform_swap(staging, destination)
            };
            let (_other, next) = prepared_demo(&format!("round3-commit-swap-next-{mode}"));
            let outcome = write_handoff_package(&target, next, CREATED_AT, &PublishHooks { swap: Some(&swap), ..PRODUCTION_HOOKS });
            if mode == "model-file" {
                // This save's own folder is under the name: the swap is undone.
                assert!(matches!(&outcome, HandoffSaveOutcome::Refused { diagnostics } if diagnostics[0].code == publish::PUBLISH_UNVERIFIED), "swap {mode}: {outcome:?}");
                assert_eq!(calls.get(), 2, "swap {mode}: the swap was not undone");
                assert_eq!(package_bytes(&target), old, "swap {mode}: the previous package was not restored unchanged");
                assert!(verify_handoff_package(&target).is_ok(), "swap {mode}");
            } else {
                // A folder this save does not own is under the name: nothing is
                // moved back (review round 5), and the previous package is left
                // unchanged under the staging name the refusal gives.
                assert!(matches!(&outcome, HandoffSaveOutcome::Refused { diagnostics } if diagnostics[0].code == publish::ROLLBACK_WITHHELD), "swap {mode}: {outcome:?}");
                assert_eq!(calls.get(), 1, "swap {mode}: a folder this save does not own was moved back");
                let left = staging_left(&store.root);
                assert_eq!(left.len(), 1, "swap {mode}: {left:?}");
                assert_eq!(package_bytes(&store.root.join(&left[0])), old, "swap {mode}: the previous package changed");
            }
        }
    }


#[test]
fn round2_differential_verifier_results() {
    // A reviewer's harness: runs only when W6B_DIFF_ROOT names a folder of packages.
    let Ok(root) = std::env::var("W6B_DIFF_ROOT") else { return };
    let root = PathBuf::from(root);
    let mut results = BTreeMap::new();
    for entry in fs::read_dir(&root).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            let result = verify_handoff_package(&path);
            results.insert(path.file_name().unwrap().to_string_lossy().to_string(), json!({"ok":result.is_ok(), "result":format!("{result:?}")}));
        }
    }
    fs::write(root.join("rust-results.json"), serde_json::to_vec_pretty(&results).unwrap()).unwrap();
}


#[test]
fn round3_array_nesting_unexpected_keys_and_escaped_pointer_screen() {
    let doc = json!({"~unexpected/key": [[[{"nonsense": {"provenance_record": {"privacy_classification": "protected_suspected"}}}]]], "legal": {"privacy_class": "private_user_data", "redistribution_status": "unknown", "review_status": "pending"}});
    assert_eq!(screen_document(&doc), vec!["/~0unexpected~1key/0/0/0/nonsense/provenance_record/privacy_classification protected_suspected"]);
    for key in ["review_classification", "redistribution_classification", "reviewStatus", "redistributionStatus"] {
        println!("ALTERNATE_KEY {key}: {:?}", screen_document(&json!({key:"quarantined"})));
    }
}


#[test]
fn round3_intermediate_symlink_and_moved_parent_use_the_original_handle() {
    let (store, prepared) = prepared_demo("round3-parent");
    let original = store.root.join("original"); fs::create_dir_all(original.join("inside")).unwrap();
    let link = store.root.join("link"); std::os::unix::fs::symlink(&original, &link).unwrap();
    let target = link.join("inside/target.swbhandoff");
    let moved = store.root.join("moved");
    let before = || { fs::rename(&original, &moved)?; fs::create_dir_all(original.join("inside")) };
    let outcome = write_handoff_package(&target, prepared, CREATED_AT, &PublishHooks { before_commit: &before, ..PRODUCTION_HOOKS });
    assert!(matches!(outcome, HandoffSaveOutcome::Saved {..}), "{outcome:?}");
    assert!(verify_handoff_package(&moved.join("inside/target.swbhandoff")).is_ok());
    assert!(!original.join("inside/target.swbhandoff").exists());
}


#[test]
fn round3_external_bytes_verifier_results() {
    // A reviewer's harness: runs only when W6B_DIFF_ROOT names a folder of packages.
    let Ok(root) = std::env::var("W6B_DIFF_ROOT") else { return };
    let root = PathBuf::from(root);
    let mut results = BTreeMap::new();
    for entry in fs::read_dir(&root).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            let result = verify_handoff_package(&path);
            results.insert(path.file_name().unwrap().to_string_lossy().to_string(), json!({"ok":result.is_ok(),"result":format!("{result:?}")}));
        }
    }
    fs::write(root.join("rust-results.json"), serde_json::to_vec_pretty(&results).unwrap()).unwrap();
}


#[test]
fn round3_legitimate_stored_library_documents_do_not_over_refuse() {
    for (kind, document) in [
        ("material", include_str!("../../../../fixtures/material/invented_material_library_valid.json")),
        ("section", include_str!("../../../../fixtures/component/invented_section_library_valid.json")),
        ("component", include_str!("../../../../fixtures/component/invented_component_library_valid.json")),
        ("hanger", include_str!("../../../../fixtures/hanger/invented_hanger_library_valid.json")),
    ] {
        let model = witness_model();
        let store = store(&format!("round3-legitimate-{kind}"), &model, false);
        let document: Value = serde_json::from_str(document).unwrap();
        let outcome = crate::save_local_library_value(|| Ok(store.path.clone()), "project:slots".to_string(), kind.to_string(), document);
        let saved = outcome.unwrap();
        assert!(saved.stored, "{kind}: {}", saved.message);
        let row = crate::load_library(&Connection::open(&store.path).unwrap(), "project:slots", kind, &saved.library_id).unwrap().unwrap();
        assert!(screen_document(&row.document).is_empty(), "{kind}: {:?}", screen_document(&row.document));
        assert!(malformed_dispositions(&row.document).is_empty(), "{kind}: {:?}", malformed_dispositions(&row.document));
    }
    let pack: Value = serde_json::from_str(DEMO_RULE_PACK).unwrap();
    assert!(screen_document(&pack).is_empty());
    assert!(malformed_dispositions(&pack).is_empty(), "{:?}", malformed_dispositions(&pack));
}

#[test]
fn round4_reference_privacy_uses_exact_known_public_values() {
    let mut failures = Vec::new();
    for value in ["TBD", "unknown", "private_user_data", "private_project_data", "redacted", "commercial_tool_metadata", "private_invented_example", "publicity_sensitive", "public_private"] {
        let got = reference_privacy("public_permissive", &[value], "private_library_data");
        if got != "private_library_data" { failures.push((value, got)); }
    }
    assert!(failures.is_empty(), "unknown/private values promoted: {failures:?}");
}

#[test]
fn round4_every_library_kind_propagates_nested_private_values() {
    for (kind, fixture) in [
        ("material", include_str!("../../../../fixtures/material/invented_material_library_valid.json")),
        ("section", include_str!("../../../../fixtures/component/invented_section_library_valid.json")),
        ("component", include_str!("../../../../fixtures/component/invented_component_library_valid.json")),
        ("hanger", include_str!("../../../../fixtures/hanger/invented_hanger_library_valid.json"))
    ] {
        for key in ["privacy_class", "privacy_classification"] {
            for private in ["private_user_data", "TBD", "redacted", "unknown", "commercial_tool_metadata"] {
                let mut model=witness_model();
                let reference=&mut model["materials"][0]["provenance_record"]["library_ref"];
                reference["library_kind"]=json!(kind);
                reference.as_object_mut().unwrap().remove("record_id");
                let mut lib: Value=serde_json::from_str(fixture).unwrap();
                fn public(v:&mut Value) { match v { Value::Object(m)=>for (k,v) in m {if k=="redistribution_status"{*v=json!("public_permissive")}else{public(v)}},Value::Array(a)=>for v in a{public(v)},_=>{}} }
                public(&mut lib);
                lib[format!("{kind}_library")]["extra_privacy"]=json!({"array":[[{key:private}]]});
                let mut lookup=|_:&str,_:&str,_:&str|Ok(Some(lib.clone()));
                let (refs,_)=library_refs(&model,"project:slots",&mut lookup).unwrap();
                assert_eq!(refs.len(),1,"{kind}");
                assert_eq!(refs[0]["privacy_classification"],json!("private_library_data"),"{kind}:{key}:{private}");
                assert_eq!(refs[0]["provenance"]["privacy_classification"],json!("private_library_data"));
            }
        }
    }
}

#[test]
fn round4_rule_reference_and_pack_both_propagate_private_values() {
    for where_ in ["reference","pack"] {
        for key in ["privacy_class", "privacy_classification"] {
            for private in ["private_user_data", "TBD", "redacted", "unknown", "commercial_tool_metadata"] {
                let mut model=witness_model();
                let mut pack: Value=serde_json::from_str(DEMO_RULE_PACK).unwrap();
                if where_=="pack" {pack["extra_privacy"]=json!({"array":[[{key:private}]]});}
                else {model["rule_pack_refs"][0]["extra_privacy"]=json!({"array":[[{key:private}]]});}
                let hash=open_pipe_stress_rule_pack_document::compute_rule_pack_checksum(&pack).unwrap().value;
                pack["checksums"]["rule_pack_checksum"]["value"]=json!(hash);
                model["rule_pack_refs"][0]["checksum"]["value"]=json!(hash);
                let mut lookup=|_:&str,_:&str|Ok(vec![pack.clone()]);
                let (refs,_)=rule_pack_refs(&model,"project:slots",&mut lookup).unwrap();
                assert_eq!(refs[0]["privacy_classification"],json!("private_rule_pack_data"),"{where_}:{key}:{private}");
                assert_eq!(refs[0]["provenance"]["privacy_classification"],json!("private_rule_pack_data"));
            }
        }
    }
}

#[test]
fn round4_panic_inside_commit_does_not_poison_following_saves() {
    let (store,prepared)=prepared_demo("round4-panic");
    let target=store.root.join("target.swbhandoff");
    let rename=|_:&Path,_:&Path| -> io::Result<()> {panic!("review injected panic before rename")};
    let out=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||write_handoff_package(&target,prepared,CREATED_AT,&PublishHooks{rename_new:Some(&rename),..PRODUCTION_HOOKS})));
    assert!(out.is_err());
    let (_other,next)=prepared_demo("round4-panic-next");
    assert!(matches!(write(&target,next),HandoffSaveOutcome::Saved{..}));
    assert!(verify_handoff_package(&target).is_ok());
    let old_bytes=package_bytes(&target);
    let (_third,next)=prepared_demo("round4-panic-after-swap");
    let token=||Ok("round4-panic-after-swap".to_string());
    let swap=|a:&Path,b:&Path| -> io::Result<()> {publish::platform_swap(a,b)?;panic!("review injected panic after swap")};
    let out=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||write_handoff_package(&target,next,CREATED_AT,&PublishHooks{token:&token,swap:Some(&swap),..PRODUCTION_HOOKS})));
    assert!(out.is_err());
    let kept=store.root.join("target.swbhandoff.staging-round4-panic-after-swap");
    assert_eq!(package_bytes(&kept),old_bytes);
    assert!(verify_handoff_package(&target).is_ok());
    let (_fourth,next)=prepared_demo("round4-panic-recovered");
    assert!(matches!(write(&target,next),HandoffSaveOutcome::Saved{..}));
    assert_eq!(package_bytes(&kept),old_bytes);
}

#[test]
fn round4_changed_member_after_identity_check_is_not_the_written_package() {
    let (store,first)=prepared_demo("round4-read-gap");
    let target=store.root.join("target.swbhandoff");
    assert!(matches!(write(&target,first),HandoffSaveOutcome::Saved{..}));
    let old=package_bytes(&target);
    let (_other,mut next)=prepared_demo("round4-read-gap-next");
    next.package_label="Intended package".to_string();
    let mut forged=handoff_manifest(&next,&PackageStamp{package_name:"target.swbhandoff",created_at:CREATED_AT});
    forged.get_mut("package_identity").unwrap()["package_label"]=json!("Substituted during post-publication verification");
    let (forged_text,_)=publish::seal_manifest(forged,&next.model_hex);
    let target2=target.clone();
    publish::R4_AFTER_ID.with(|hook| *hook.borrow_mut()=Some(Box::new(move||{
        let replacement=target2.join("replacement.json");
        fs::write(&replacement,forged_text).unwrap();
        fs::rename(&replacement,target2.join("handoff_package.json")).unwrap();
    })));
    let out=write(&target,next);
    let actual=package_bytes(&target);
    println!("READ_GAP {out:?}; verifies={:?}; old_retained={}",verify_handoff_package(&target),!staging_left(&store.root).is_empty());
    assert!(matches!(out,HandoffSaveOutcome::Refused{..}),"Saved a member substituted after the identity check; deleted previous {}-byte manifest; published {} bytes",old.0.len(),actual.0.len());
}

#[test]
fn round4_fifo_substitution_stalls_unrelated_destinations_until_a_writer_arrives() {
    use std::sync::mpsc;
    use std::time::Duration;
    let (store,prepared)=prepared_demo("round4-fifo");
    let target=store.root.join("target.swbhandoff");
    let (_other,other)=prepared_demo("round4-fifo-unrelated");
    let unrelated=store.root.join("unrelated.swbhandoff");
    let (ready_tx,ready_rx)=mpsc::channel();
    let (result_tx,result_rx)=mpsc::channel();
    let (attempt_tx,attempt_rx)=mpsc::channel();
    let target2=target.clone();
    let helper=std::thread::spawn(move|| {
        ready_rx.recv().unwrap();
        let save=std::thread::spawn(move||{
            let before=||{attempt_tx.send(()).unwrap();Ok(())};
            let out=write_handoff_package(&unrelated,other,CREATED_AT,&PublishHooks{before_commit:&before,..PRODUCTION_HOOKS});
            result_tx.send(out).unwrap();
        });
        attempt_rx.recv().unwrap();
        let first=result_rx.recv_timeout(Duration::from_millis(250));
        let blocked=first.is_err();
        // Release the blocked read-only FIFO open, so this test leaves no hung thread.
        // (Non-blocking: once the save no longer blocks in the open, no reader holds the FIFO.)
        use std::os::unix::fs::OpenOptionsExt;
        drop(fs::OpenOptions::new().write(true).custom_flags(libc::O_NONBLOCK).open(target2.join("model/model.json")));
        let out=match first { Ok(out)=>out, Err(_)=>result_rx.recv_timeout(Duration::from_secs(10)).unwrap() };
        save.join().unwrap();
        assert!(matches!(out,HandoffSaveOutcome::Saved{..}));
        blocked
    });
    let target3=target.clone();
    publish::R4_AFTER_ID.with(|hook| *hook.borrow_mut()=Some(Box::new(move||{
        publish::R4_BEFORE_OPEN.with(|next| *next.borrow_mut()=Some(Box::new(move||{
            let member=target3.join("model/model.json");
            fs::rename(&member,target3.join("model/retained.json")).unwrap();
            use std::os::unix::ffi::OsStrExt;
            let name=std::ffi::CString::new(member.as_os_str().as_bytes()).unwrap();
            assert_eq!(unsafe{libc::mkfifo(name.as_ptr(),0o600)},0);
            ready_tx.send(()).unwrap();
        })));
    })));
    let out=write(&target,prepared);
    println!("FIFO_SUBSTITUTION {out:?}");
    assert!(matches!(out,HandoffSaveOutcome::Refused{..}));
    assert!(!helper.join().unwrap(),"opening an untrusted FIFO blocked a different destination behind the process-global commit mutex");
}

#[test]
fn round4_member_replaced_during_verification_must_not_return_saved_and_delete_old() {
    let (store,first)=prepared_demo("round4-inconsistent-read");
    let target=store.root.join("target.swbhandoff");
    assert!(matches!(write(&target,first),HandoffSaveOutcome::Saved{..}));
    let old=package_bytes(&target);
    let (_other,next)=prepared_demo("round4-inconsistent-read-next");
    let target2=target.clone();
    publish::R4_AFTER_ID.with(|hook| *hook.borrow_mut()=Some(Box::new(move||{
        // This first callback runs before reading the model. Its callback runs
        // before reading the manifest, after the model bytes have been read.
        publish::R4_BEFORE_OPEN.with(|next| *next.borrow_mut()=Some(Box::new(move||{
            publish::R4_BEFORE_OPEN.with(|next| *next.borrow_mut()=Some(Box::new(move||{
                let replacement=target2.join("model/replacement.json");
                fs::write(&replacement,b"{}").unwrap();
                fs::rename(&replacement,target2.join("model/model.json")).unwrap();
            })));
        })));
    })));
    let out=write(&target,next);
    let verified=verify_handoff_package(&target);
    println!("INCONSISTENT_READ {out:?}; verifies={verified:?}; old_retained={}",!staging_left(&store.root).is_empty());
    assert!(matches!(out,HandoffSaveOutcome::Refused{..}),"Saved an invalid package and removed previous {}-byte manifest",old.0.len());
}


    /// Review round 4, should-fix 2: commits are serialized per destination
    /// only; a save of another name commits while one is held inside its commit.
    #[test]
    fn a_commit_held_open_does_not_stall_a_save_into_another_folder() {
        use std::sync::mpsc;
        use std::time::Duration;
        let (store, held) = prepared_demo("round4-keyed-held");
        let (_other, free) = prepared_demo("round4-keyed-free");
        // Commits serialize per parent folder (review round 5): the free save
        // goes to another folder.
        let held_target = store.root.join("held.swbhandoff");
        let other_folder = store.root.join("other-folder");
        fs::create_dir(&other_folder).unwrap();
        let free_target = other_folder.join("free.swbhandoff");
        let (entered_tx, entered_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel::<()>();
        let holder = std::thread::spawn(move || {
            let rename = |staging: &Path, destination: &Path| {
                entered_tx.send(()).unwrap();
                let _ = release_rx.recv_timeout(Duration::from_secs(10));
                fs::rename(staging, destination)
            };
            write_handoff_package(&held_target, held, CREATED_AT, &PublishHooks { rename_new: Some(&rename), ..PRODUCTION_HOOKS })
        });
        entered_rx.recv_timeout(Duration::from_secs(10)).unwrap();
        let (done_tx, done_rx) = mpsc::channel();
        let saver = std::thread::spawn(move || done_tx.send(write(&free_target, free)).unwrap());
        let while_held = done_rx.recv_timeout(Duration::from_secs(5));
        release_tx.send(()).unwrap();
        let held = holder.join().unwrap();
        let independent = while_held.is_ok();
        let unrelated = match while_held {
            Ok(outcome) => outcome,
            Err(_) => done_rx.recv_timeout(Duration::from_secs(10)).unwrap(),
        };
        saver.join().unwrap();
        assert!(independent, "a save of another destination waited for a held commit");
        assert!(matches!(held, HandoffSaveOutcome::Saved { .. }), "{held:?}");
        assert!(matches!(unrelated, HandoffSaveOutcome::Saved { .. }), "{unrelated:?}");
    }

#[test]
fn round5_rollback_does_not_move_a_known_unrelated_folder() {
    let mut failures = Vec::new();
    for substitute in ["target", "staging"] {
        let (store, first) = prepared_demo(&format!("round5-rollback-{substitute}"));
        let target = store.root.join("target.swbhandoff");
        assert!(matches!(write(&target, first), HandoffSaveOutcome::Saved { .. }));
        let old = package_bytes(&target);
        let (_other, next) = prepared_demo("round5-rollback-next");
        let staging = store.root.join("target.swbhandoff.staging-round5");
        let retained = store.root.join("retained-by-other-process");
        let changed = if substitute == "target" { target.clone() } else { staging.clone() };
        let changed2 = changed.clone();
        let target2 = target.clone();
        let retained2 = retained.clone();
        publish::R4_AFTER_ID.with(|hook| *hook.borrow_mut() = Some(Box::new(move || {
            fs::rename(&changed2, &retained2).unwrap();
            fs::create_dir(&changed2).unwrap();
            fs::write(changed2.join("unrelated.txt"), b"do not move this folder").unwrap();
            if substitute == "staging" {
                fs::write(target2.join(MANIFEST_MEMBER), b"{}").unwrap();
            }
        })));
        let token = || Ok("round5".to_string());
        let out = write_handoff_package(&target, next, CREATED_AT, &PublishHooks { token: &token, ..PRODUCTION_HOOKS });
        let unchanged_name = changed.join("unrelated.txt").exists();
        let old_at_staging = staging.join(MANIFEST_MEMBER).exists() && package_bytes(&staging) == old;
        println!("ROUND5_ROLLBACK {substitute}: {out:?}; unrelated_at_original_name={unchanged_name}; previous_at_staging={old_at_staging}; target_unrelated={}", target.join("unrelated.txt").exists());
        assert!(matches!(out, HandoffSaveOutcome::Refused { .. }));
        if !unchanged_name { failures.push(substitute); }
    }
    assert!(failures.is_empty(), "rollback knowingly moved unrelated folders after verification detected substitution: {failures:?}");
}

#[test]
fn round5_case_alias_saves_do_not_leave_committed_packages_in_staging() {
    use std::sync::mpsc;
    use std::time::Duration;
    let (store, first) = prepared_demo("round5-case-alias");
    let target = store.root.join("target.swbhandoff");
    let alias = store.root.join("TARGET.swbhandoff");
    assert!(matches!(write(&target, first), HandoffSaveOutcome::Saved { .. }));
    use std::os::unix::fs::MetadataExt;
    assert_eq!(fs::metadata(&target).unwrap().ino(), fs::metadata(&alias).unwrap().ino(), "probe requires a case-insensitive volume");
    let (_sa, a) = prepared_demo("round5-case-a");
    let (_sb, b) = prepared_demo("round5-case-b");
    let (a_entered_tx, a_entered_rx) = mpsc::channel();
    let (b_entered_tx, b_entered_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel();
    let (a_swapped_tx, a_swapped_rx) = mpsc::channel();
    let (b_swapped_tx, b_swapped_rx) = mpsc::channel();
    let (a_done_tx, a_done_rx) = mpsc::channel();
    let (b_staged_tx, b_staged_rx) = mpsc::channel();
    let aa = std::thread::spawn(move || {
        let calls = std::cell::Cell::new(0);
        let swap = |s: &Path, t: &Path| {
            calls.set(calls.get() + 1);
            if calls.get() == 1 {
                a_entered_tx.send(()).unwrap();
                release_rx.recv_timeout(Duration::from_secs(10)).unwrap();
                publish::platform_swap(s, t)?;
                a_swapped_tx.send(()).unwrap();
                let _ = b_swapped_rx.recv_timeout(Duration::from_secs(1));
                Ok(())
            } else { publish::platform_swap(s, t) }
        };
        write_handoff_package(&target, a, CREATED_AT, &PublishHooks { swap: Some(&swap), ..PRODUCTION_HOOKS })
    });
    a_entered_rx.recv_timeout(Duration::from_secs(10)).unwrap();
    let bb = std::thread::spawn(move || {
        let swap = |s: &Path, t: &Path| {
            b_entered_tx.send(()).unwrap();
            a_swapped_rx.recv_timeout(Duration::from_secs(10)).unwrap();
            publish::platform_swap(s, t)?;
            b_swapped_tx.send(()).unwrap();
            a_done_rx.recv_timeout(Duration::from_secs(10)).unwrap();
            Ok(())
        };
        let before = || { b_staged_tx.send(()).unwrap(); Ok(()) };
        write_handoff_package(&alias, b, CREATED_AT, &PublishHooks { before_commit: &before, swap: Some(&swap), ..PRODUCTION_HOOKS })
    });
    b_staged_rx.recv_timeout(Duration::from_secs(10)).unwrap();
    let overlap = b_entered_rx.recv_timeout(Duration::from_millis(300)).is_ok();
    release_tx.send(()).unwrap();
    let a = aa.join().unwrap();
    let _ = a_done_tx.send(());
    let b = bb.join().unwrap();
    let leftovers = staging_left(&store.root);
    println!("ROUND5_ALIAS overlap={overlap}; A={a:?}; B={b:?}; leftovers={leftovers:?}");
    assert!(!overlap && leftovers.is_empty(), "saves within the app interleaved through case aliases of the same destination");
}

#[test]
fn round5_member_substitution_matrix_refuses_and_keeps_previous() {
    let mut checked = 0;
    for member in ["model/model.json", "handoff_package.json"] {
        for mode in ["file", "symlink", "fifo", "directory", "hardlink", "rewrite"] {
            for replacing in [false, true] {
                let (store, first) = prepared_demo("round5-members");
                let target = store.root.join("target.swbhandoff");
                let old = if replacing {
                    assert!(matches!(write(&target, first), HandoffSaveOutcome::Saved { .. }));
                    Some(package_bytes(&target))
                } else { None };
                let outside = store.root.join("outside.json");
                fs::write(&outside, b"outside must survive").unwrap();
                let (_next_store, next) = prepared_demo("round5-members-next");
                let target2 = target.clone();
                let outside2 = outside.clone();
                publish::R4_AFTER_ID.with(|hook| *hook.borrow_mut() = Some(Box::new(move || {
                    let substitute = move || {
                        let file = target2.join(member);
                        let bytes = fs::read(&file).unwrap();
                        if mode == "rewrite" { fs::write(&file, b"{}").unwrap(); return; }
                        fs::rename(&file, file.with_extension("retained")).unwrap();
                        match mode {
                            "file" => fs::write(&file, bytes).unwrap(),
                            "symlink" => std::os::unix::fs::symlink(&outside2, &file).unwrap(),
                            "hardlink" => fs::hard_link(&outside2, &file).unwrap(),
                            "directory" => fs::create_dir(&file).unwrap(),
                            "fifo" => {
                                use std::os::unix::ffi::OsStrExt;
                                let file = std::ffi::CString::new(file.as_os_str().as_bytes()).unwrap();
                                assert_eq!(unsafe { libc::mkfifo(file.as_ptr(), 0o600) }, 0);
                            }
                            _ => unreachable!(),
                        }
                    };
                    publish::R4_BEFORE_OPEN.with(|hook| *hook.borrow_mut() = Some(Box::new(move || {
                        if member == "handoff_package.json" {
                            publish::R4_BEFORE_OPEN.with(|hook| *hook.borrow_mut() = Some(Box::new(substitute)));
                        } else { substitute(); }
                    })));
                })));
                let out = write(&target, next);
                assert!(matches!(out, HandoffSaveOutcome::Refused { .. }), "{member}/{mode}/{replacing}: {out:?}");
                if let Some(old) = old { assert_eq!(package_bytes(&target), old, "{member}/{mode}"); }
                assert_eq!(fs::read(&outside).unwrap(), b"outside must survive");
                checked += 1;
            }
        }
    }
    println!("ROUND5_MEMBER_SUBSTITUTIONS {checked} cases passed");
}

#[test]
fn round5_nonstring_privacy_classifications_are_not_public() {
    let mut failures = Vec::new();
    for value in [Value::Null, json!(false), json!(0), json!([]), json!({}), json!(["private_user_data"])] {
        let mut model = witness_model();
        let reference = &mut model["materials"][0]["provenance_record"]["library_ref"];
        reference.as_object_mut().unwrap().remove("record_id");
        let mut lib: Value = serde_json::from_str(include_str!("../../../../fixtures/material/invented_material_library_valid.json")).unwrap();
        fn public(v: &mut Value) {
            match v {
                Value::Object(m) => for (k,v) in m { if k == "redistribution_status" { *v=json!("public_permissive"); } else { public(v); } },
                Value::Array(a) => for v in a { public(v); },
                _ => {},
            }
        }
        public(&mut lib);
        lib["material_library"]["privacy_class"] = value.clone();
        reference["library_id"] = lib["material_library"]["library_id"].clone();
        let store = store("round5-nonstrings", &model, false);
        let saved = crate::save_local_library_value(|| Ok(store.path.clone()), "project:slots".to_string(), "material".to_string(), lib.clone()).unwrap();
        assert!(saved.stored, "normal private import refused {value}: {}", saved.message);
        let row = crate::load_library(&Connection::open(&store.path).unwrap(), "project:slots", "material", &saved.library_id).unwrap().unwrap();
        let mut lookup = |_: &str, _: &str, _: &str| Ok(Some(row.document.clone()));
        // Since review round 6 (note 4) the save refuses such a document as
        // malformed; the classifier itself still counts the value as private.
        match library_refs(&model, "project:slots", &mut lookup) {
            Err(refusals) if refusals.iter().any(|refusal| refusal.code == DISPOSITION_MALFORMED) => {}
            Err(refusals) => panic!("{value}: {refusals:?}"),
            Ok((refs, _)) => {
                let classification = refs[0]["privacy_classification"].clone();
                println!("ROUND5_NONSTRING_PRIVACY {value} => {classification}");
                if classification != json!("private_library_data") { failures.push(value.clone()); }
            }
        }
        let mut privacies = Vec::new();
        privacy_values(&lib, &mut privacies);
        if reference_privacy("public_permissive", &privacies, "private_library_data") != "private_library_data" { failures.push(value); }
        if let Ok(out) = std::env::var("W6B_REVIEW_EVIDENCE") {
            fs::write(PathBuf::from(out).join("nonstring-privacy-library.json"), serde_json::to_vec_pretty(&lib).unwrap()).unwrap();
        }
    }
    assert!(failures.is_empty(), "malformed non-string classifications were silently omitted: {failures:?}");
}

#[test]
fn round5_unsupported_new_name_refuses_with_its_named_diagnostic() {
    let (store, prepared) = prepared_demo("round5-no-exclusive-rename");
    let target = store.root.join("target.swbhandoff");
    let unsupported = |_: &Path, _: &Path| Err(io::Error::from_raw_os_error(libc::ENOTSUP));
    let out = write_handoff_package(&target, prepared, CREATED_AT, &PublishHooks { rename_new: Some(&unsupported), ..PRODUCTION_HOOKS });
    assert!(matches!(&out, HandoffSaveOutcome::Refused { diagnostics } if diagnostics[0].code == "HANDOFF-NEW-NAME-UNSUPPORTED"), "{out:?}");
    assert!(!target.exists());
    assert!(staging_left(&store.root).is_empty());
}

// Characterization only: a same-inode rewrite after the model's final read,
// while the manifest is still being opened. This is separate from regression
// assertions so it does not encode Saved as the required behavior.
#[test]
fn review5_residual_rewrite_after_the_models_read() {
    let (store, first) = prepared_demo("review5-residual");
    let target = store.root.join("target.swbhandoff");
    assert!(matches!(write(&target, first), HandoffSaveOutcome::Saved { .. }));
    let (_next_store, next) = prepared_demo("review5-residual-next");
    let target2 = target.clone();
    publish::R4_AFTER_ID.with(|hook| *hook.borrow_mut() = Some(Box::new(move || {
        publish::R4_BEFORE_OPEN.with(|hook| *hook.borrow_mut() = Some(Box::new(move || {
            publish::R4_BEFORE_OPEN.with(|hook| *hook.borrow_mut() = Some(Box::new(move || {
                fs::write(target2.join(publish::MODEL_MEMBER), b"{}").unwrap();
            })));
        })));
    })));
    let out = write(&target, next);
    let verification = verify_handoff_package(&target);
    println!("REVIEW5_RESIDUAL outcome={out:?}; verifier_afterwards={verification:?}; leftovers={:?}", staging_left(&store.root));
}

#[test]
fn review5_residual_swap_can_move_a_nonempty_substitute() {
    let root = sandbox("review5-residual-swap");
    let staging = root.join("staging");
    let target = root.join("target");
    fs::create_dir(&staging).unwrap();
    fs::create_dir(&target).unwrap();
    fs::write(staging.join("written.txt"), b"new").unwrap();
    fs::write(target.join("unrelated.txt"), b"non-empty substituted folder").unwrap();
    publish::platform_swap(&staging, &target).unwrap();
    assert!(staging.join("unrelated.txt").exists());
    println!("REVIEW5_RESIDUAL_SWAP a nonempty unrelated target folder was moved to staging by RENAME_SWAP");
    fs::remove_dir_all(&root).unwrap();
}


    /// Review round 5, finding 4: a same-inode rewrite of an already-read
    /// member while the other is read is caught by re-checking every member's
    /// descriptor (identity, size, change time) after the last read.
    #[test]
    fn a_member_rewritten_after_its_read_but_before_the_last_check_is_refused() {
        for rewritten in [publish::MODEL_MEMBER] {
            let (store, first) = prepared_demo("round5-ctime");
            let target = store.root.join("target.swbhandoff");
            assert!(matches!(write(&target, first), HandoffSaveOutcome::Saved { .. }));
            let old = package_bytes(&target);
            let (_next_store, next) = prepared_demo("round5-ctime-next");
            let member = target.join(rewritten);
            // The rewrite lands after the model's read, before the manifest's open
            // (same inode, same length, so only the change time shows it).
            publish::R4_AFTER_ID.with(|hook| *hook.borrow_mut() = Some(Box::new(move || {
                publish::R4_BEFORE_OPEN.with(|hook| *hook.borrow_mut() = Some(Box::new(move || {
                    publish::R4_BEFORE_OPEN.with(|hook| *hook.borrow_mut() = Some(Box::new(move || {
                        let bytes = fs::read(&member).unwrap();
                        let mut file = fs::OpenOptions::new().write(true).open(&member).unwrap();
                        io::Write::write_all(&mut file, &bytes).unwrap();
                    })));
                })));
            })));
            let out = write(&target, next);
            assert!(matches!(&out, HandoffSaveOutcome::Refused { diagnostics } if diagnostics[0].code == publish::PUBLISH_UNVERIFIED), "{rewritten}: {out:?}");
            assert_eq!(package_bytes(&target), old, "{rewritten}: the previous package was not restored");
        }
    }

    // ---- Independent review round 6 (Claude): probes and regressions ----------

    /// Every path under `root` (not following links) whose inode is `ino`.
    fn round6_paths_with_inode(root: &Path, ino: u64) -> Vec<PathBuf> {
        use std::os::unix::fs::MetadataExt;
        let mut found = Vec::new();
        let mut stack = vec![root.to_path_buf()];
        while let Some(dir) = stack.pop() {
            for entry in fs::read_dir(&dir).unwrap().filter_map(Result::ok) {
                let meta = fs::symlink_metadata(entry.path()).unwrap();
                if meta.ino() == ino { found.push(entry.path()); }
                if meta.is_dir() { stack.push(entry.path()); }
            }
        }
        found
    }

    /// Finding R6-1 (should-fix): the previous package is removed by identity
    /// only. A member rewritten in place after admission verified it (same
    /// inode; the change time moves) is deleted by a save that reports `Saved`.
    #[test]
    fn round6_previous_package_rewritten_after_admission_is_not_deleted() {
        use std::io::{Seek, SeekFrom, Write as _};
        use std::os::unix::fs::MetadataExt;
        let mut deleted = Vec::new();
        for (member, mode) in [(MANIFEST_MEMBER, "same-size"), (MANIFEST_MEMBER, "append"), (publish::MODEL_MEMBER, "same-size")] {
            let (store, first) = prepared_demo(&format!("round6-old-{mode}"));
            let target = store.root.join("target.swbhandoff");
            assert!(matches!(write(&target, first), HandoffSaveOutcome::Saved { .. }));
            let file = target.join(member);
            let ino = fs::metadata(&file).unwrap().ino();
            let (_next_store, next) = prepared_demo(&format!("round6-old-{mode}-next"));
            let file2 = file.clone();
            // After admission verified the previous package, before the swap.
            let edit = move || -> io::Result<()> {
                let mut handle = fs::OpenOptions::new().write(true).open(&file2)?;
                if mode == "append" { handle.seek(SeekFrom::End(0))?; handle.write_all(b" user edit")?; } else { handle.write_all(b"[")?; }
                handle.sync_all()
            };
            let out = write_handoff_package(&target, next, CREATED_AT, &PublishHooks { before_commit: &edit, ..PRODUCTION_HOOKS });
            let survivors = round6_paths_with_inode(&store.root, ino);
            println!("ROUND6_OLD_REWRITE {member}/{mode}: saved={}; edited file survives at {survivors:?}", matches!(out, HandoffSaveOutcome::Saved { .. }));
            if survivors.is_empty() { deleted.push(format!("{member}/{mode}")); }
        }
        assert!(deleted.is_empty(), "the previous package's member, rewritten after it was verified, was deleted: {deleted:?}");
    }

    /// Finding R6-2 (should-fix, docs): a write through a shared memory mapping
    /// does not move the change time on APFS until msync, so the final
    /// descriptor re-check (identity, size, change time) does not see it.
    #[test]
    fn round6_mapped_write_after_the_models_read_is_not_reported_saved() {
        use std::os::fd::AsRawFd;
        use std::os::unix::fs::MetadataExt;
        let (store, first) = prepared_demo("round6-mmap");
        let target = store.root.join("target.swbhandoff");
        assert!(matches!(write(&target, first), HandoffSaveOutcome::Saved { .. }));
        let (_next_store, next) = prepared_demo("round6-mmap-next");
        let member = target.join(publish::MODEL_MEMBER);
        let mapping: std::rc::Rc<std::cell::Cell<(usize, usize)>> = Default::default();
        let ctimes: std::rc::Rc<std::cell::Cell<((i64, i64), (i64, i64))>> = Default::default();
        let (mapping2, ctimes2) = (mapping.clone(), ctimes.clone());
        publish::R4_AFTER_ID.with(|hook| *hook.borrow_mut() = Some(Box::new(move || {
            publish::R4_BEFORE_OPEN.with(|hook| *hook.borrow_mut() = Some(Box::new(move || {
                publish::R4_BEFORE_OPEN.with(|hook| *hook.borrow_mut() = Some(Box::new(move || {
                    // After the model's read, before the manifest's open.
                    let file = fs::OpenOptions::new().read(true).write(true).open(&member).unwrap();
                    let before = file.metadata().unwrap();
                    let len = before.len() as usize;
                    // SAFETY: a valid descriptor and length; unmapped below.
                    let ptr = unsafe { libc::mmap(std::ptr::null_mut(), len, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED, file.as_raw_fd(), 0) };
                    assert_ne!(ptr, libc::MAP_FAILED);
                    unsafe { *(ptr as *mut u8) = b'[' };
                    let after = file.metadata().unwrap();
                    ctimes2.set(((before.ctime(), before.ctime_nsec()), (after.ctime(), after.ctime_nsec())));
                    mapping2.set((ptr as usize, len));
                })));
            })));
        })));
        let out = write(&target, next);
        let on_disk = fs::read(target.join(publish::MODEL_MEMBER)).unwrap();
        let verified = verify_handoff_package(&target);
        let (ptr, len) = mapping.get();
        if ptr != 0 { unsafe { libc::munmap(ptr as *mut libc::c_void, len) }; }
        let (before, after) = ctimes.get();
        println!("ROUND6_MMAP outcome_saved={}; ctime_before={before:?} ctime_after_mapped_write={after:?}; first_byte_on_disk={:?}; verifier_afterwards={verified:?}", matches!(out, HandoffSaveOutcome::Saved { .. }), on_disk.first().map(|b| *b as char));
        assert!(!matches!(out, HandoffSaveOutcome::Saved { .. }), "Saved was reported while model/model.json held bytes other than the ones written (a mapped write after its read)");
    }

    /// Probe (f): blocking dispositions planted deep in a stored library (also
    /// in a record the model does not reference) and in a stored rule pack.
    #[test]
    fn round6_deep_blocking_dispositions_refuse() {
        let mut escaped = Vec::new();
        for (tag, place) in [("lib-deep", 0), ("lib-unreferenced", 1), ("pack-deep", 2)] {
            let mut model = witness_model();
            let mut library = material_library();
            let mut pack: Value = serde_json::from_str(DEMO_RULE_PACK).unwrap();
            match place {
                0 => library["material_records"][0]["notes"] = json!([{ "deep": { "items": [1, { "review_status": "rejected" }] } }]),
                1 => library["material_records"].as_array_mut().unwrap().push(json!({ "material_id": "other", "name": "Other", "x": [{ "provenance": { "redistribution_status": "protected_suspected" } }] })),
                _ => {
                    pack["metadata"]["invented_extension"] = json!({ "a": [[{ "lifecycle_status": "quarantined" }]] });
                    let hash = open_pipe_stress_rule_pack_document::compute_rule_pack_checksum(&pack).unwrap().value;
                    pack["checksums"]["rule_pack_checksum"]["value"] = json!(hash);
                    model["rule_pack_refs"][0]["checksum"]["value"] = json!(hash);
                }
            }
            let store = store(&format!("round6-{tag}"), &model, true);
            let connection = Connection::open(&store.path).unwrap();
            crate::upsert_library(&connection, "project:slots", "material", "library:invented-materials", &library).unwrap();
            crate::upsert_rule_pack(&connection, "project:slots", "invented_demo_rule_pack", &pack).unwrap();
            let state = save_state(&store, &model, "handoff_basis", "6e6e00006e6e00006e6e00006e6e0000").state;
            let result = prepare(&store, &model, &state.state_id);
            println!("ROUND6_DEEP {tag}: {:?}", codes(&result));
            if !matches!(&result, Err(d) if d.iter().any(|d| d.code == model_document_save::PROTECTED_CONTENT_BLOCKED)) { escaped.push(tag); }
        }
        assert!(escaped.is_empty(), "deep blocking dispositions escaped: {escaped:?}");
    }

    /// Probe (f): a private record, or a private value deep in a record's
    /// provenance, under a public wrapper is never labelled public; a
    /// reference naming no record covers every record.
    #[test]
    fn round6_private_record_under_public_wrapper_is_private() {
        let mut public = Vec::new();
        for tag in ["nested-provenance", "no-record-id"] {
            let mut model = witness_model();
            let mut library = material_library();
            library["material_library"]["privacy_class"] = json!("public");
            library["material_records"][0]["privacy_class"] = json!("public");
            if tag == "nested-provenance" {
                library["material_records"][0]["provenance"]["source_detail"] = json!({ "privacy_classification": "private_user_data" });
            } else {
                model["materials"][0]["provenance_record"]["library_ref"].as_object_mut().unwrap().remove("record_id");
                let provenance = library["material_records"][0]["provenance"].clone();
                library["material_records"].as_array_mut().unwrap().push(json!({ "material_id": "other", "name": "Other", "privacy_class": "private_user_data", "provenance": provenance }));
            }
            let store = store(&format!("round6-private-{tag}"), &model, true);
            crate::upsert_library(&Connection::open(&store.path).unwrap(), "project:slots", "material", "library:invented-materials", &library).unwrap();
            let state = save_state(&store, &model, "handoff_basis", "6e6e00006e6e00006e6e00006e6e0000").state;
            let prepared = prepare(&store, &model, &state.state_id).unwrap();
            let reference = prepared.library_refs[0].clone();
            println!("ROUND6_PRIVATE {tag}: privacy={} provenance.privacy={}", reference["privacy_classification"], reference["provenance"]["privacy_classification"]);
            if reference["privacy_classification"] != json!("private_library_data") || reference["provenance"]["privacy_classification"] != json!("private_library_data") { public.push(tag); }
        }
        assert!(public.is_empty(), "private records labelled public: {public:?}");
    }

    /// Note 4 (review round 6): a disposition value that is not a string, here
    /// an array holding a blocking value, is refused as malformed by pointer.
    #[test]
    fn round6_nonstring_blocking_value_is_refused_as_malformed() {
        // Arrays and objects refuse (round 6b); null, booleans and numbers are
        // saved, never as a public disposition.
        for (value, refused) in [
            (json!(["protected_suspected"]), true),
            (json!({ "status": "rejected" }), true),
            (Value::Null, false),
            (json!(false), false),
            (json!(0), false),
        ] {
            let model = witness_model();
            let mut library = material_library();
            library["material_records"][0]["provenance"]["redistribution_status"] = value.clone();
            library["material_records"][0]["redistribution_status"] = value.clone();
            let store = store("round6-nonstring-blocking", &model, true);
            crate::upsert_library(&Connection::open(&store.path).unwrap(), "project:slots", "material", "library:invented-materials", &library).unwrap();
            let state = save_state(&store, &model, "handoff_basis", "6e6e00006e6e00006e6e00006e6e0000").state;
            let result = prepare(&store, &model, &state.state_id);
            if refused {
                assert!(
                    matches!(&result, Err(d) if d.iter().any(|d| d.code == DISPOSITION_MALFORMED && d.message.contains("/material_records/0/provenance/redistribution_status"))),
                    "{value}: {:?}",
                    codes(&result)
                );
            } else {
                let prepared = result.unwrap_or_else(|refusals| panic!("{value}: {refusals:?}"));
                let reference = &prepared.library_refs[0];
                assert_ne!(reference["redistribution_status"], json!("public_permissive"), "{value}");
                assert_ne!(reference["privacy_classification"], json!("public_metadata"), "{value}");
            }
        }
    }

    /// Probe (f): checksum subject binding. Each forged subject, resealed so
    /// the inventory is self-consistent, is refused by the Rust verifier; the
    /// bytes are written for the Python twin (W6B_R6_FORGED).
    #[test]
    fn round6_forged_checksum_subjects_are_refused() {
        let model = witness_model();
        let store = store("round6-forge", &model, true);
        let state = save_state(&store, &model, "handoff_basis", "6e6e00006e6e00006e6e00006e6e0000").state;
        let prepared = prepare(&store, &model, &state.state_id).unwrap();
        let model_bytes = prepared.model_text.clone().into_bytes();
        let base = handoff_manifest(&prepared, &PackageStamp { package_name: "invented.swbhandoff", created_at: CREATED_AT });
        let (good, _) = publish::seal_manifest(base.clone(), &prepared.model_hex);
        assert!(publish::verify_bytes(&model_bytes, good.as_bytes()).is_ok());
        let forgeries: [(&str, &str, Value); 7] = [
            ("library-subject", "/library_refs/0/checksum/payload_ref/ref", json!("library:other")),
            ("rule-pack-subject", "/rule_pack_refs/0/checksum/payload_ref/ref", json!("other_pack")),
            ("units-subject", "/units_manifest/hash_refs/0/payload_ref/ref", json!("unit-system:other")),
            ("model-basis-subject", "/model_basis/hash_refs/0/payload_ref/ref", json!("project:other")),
            ("state-record-subject", "/model_state_refs/0/hash_refs/1/payload_ref/ref", json!("state:other")),
            ("model-hash-scope", "/model_hash/payload_scope", json!("handoff_package_manifest")),
            ("library-scope", "/library_refs/0/checksum/payload_scope", json!("model_payload")),
        ];
        let out_root = std::env::var("W6B_R6_FORGED").ok().map(PathBuf::from);
        let mut accepted = Vec::new();
        for (tag, pointer, value) in forgeries {
            let mut manifest = Value::Object(base.clone());
            *manifest.pointer_mut(pointer).unwrap_or_else(|| panic!("{tag}: {pointer} absent")) = value;
            let Value::Object(manifest) = manifest else { unreachable!() };
            let (text, _) = publish::seal_manifest(manifest, &prepared.model_hex);
            let verdict = publish::verify_bytes(&model_bytes, text.as_bytes());
            println!("ROUND6_FORGED {tag}: rust_ok={}", verdict.is_ok());
            if verdict.is_ok() { accepted.push(tag); }
            if let Some(root) = &out_root {
                let dir = root.join(format!("{tag}.swbhandoff"));
                fs::create_dir_all(dir.join(MODEL_DIRECTORY)).unwrap();
                fs::write(dir.join(MODEL_DIRECTORY).join(MODEL_FILE), &model_bytes).unwrap();
                fs::write(dir.join(MANIFEST_MEMBER), text.as_bytes()).unwrap();
            }
        }
        if let Some(root) = &out_root {
            let dir = root.join("control.swbhandoff");
            fs::create_dir_all(dir.join(MODEL_DIRECTORY)).unwrap();
            fs::write(dir.join(MODEL_DIRECTORY).join(MODEL_FILE), &model_bytes).unwrap();
            fs::write(dir.join(MANIFEST_MEMBER), good.as_bytes()).unwrap();
        }
        assert!(accepted.is_empty(), "forged subjects accepted: {accepted:?}");
    }

    /// Review round 6, R6-1: the previous package edited in place after the
    /// swap (under the staging name, before its removal) is kept, not deleted;
    /// the new package, already confirmed, is saved with the warning.
    #[test]
    fn a_previous_package_edited_after_the_swap_is_kept_not_deleted() {
        use std::io::Write as _;
        let (store, first) = prepared_demo("round6-after-swap");
        let target = store.root.join("target.swbhandoff");
        assert!(matches!(write(&target, first), HandoffSaveOutcome::Saved { .. }));
        let (_next_store, next) = prepared_demo("round6-after-swap-next");
        let staging = store.root.join("target.swbhandoff.staging-round6-after-swap");
        let edited = staging.join(MANIFEST_MEMBER);
        let edited2 = edited.clone();
        publish::R4_AFTER_ID.with(|hook| *hook.borrow_mut() = Some(Box::new(move || {
            let mut handle = fs::OpenOptions::new().write(true).open(&edited2).unwrap();
            handle.write_all(b"[").unwrap();
        })));
        let token = || Ok("round6-after-swap".to_string());
        let out = write_handoff_package(&target, next, CREATED_AT, &PublishHooks { token: &token, ..PRODUCTION_HOOKS });
        assert!(matches!(&out, HandoffSaveOutcome::Saved { warnings, .. } if warnings.iter().any(|w| w.code == publish::OLD_PACKAGE_LEFT)), "{out:?}");
        assert_eq!(fs::read(&edited).unwrap().first(), Some(&b'['), "the edited previous package was deleted");
        assert!(verify_handoff_package(&target).is_ok());
    }

    // ---- Independent review round 6b (Claude): strict-screen probes ----------

    /// Characterization: which non-string disposition values the library
    /// import stores, and which of those the handoff save now refuses.
    #[test]
    fn round6b_import_accepted_nonstring_dispositions_vs_handoff_refusal() {
        let mut accepted_then_refused = Vec::new();
        let mut cases = 0;
        for (kind, document, wrapper, records) in [
            ("material", include_str!("../../../../fixtures/material/invented_material_library_valid.json"), "material_library", "material_records"),
            ("section", include_str!("../../../../fixtures/component/invented_section_library_valid.json"), "section_library", "section_records"),
            ("component", include_str!("../../../../fixtures/component/invented_component_library_valid.json"), "component_library", "component_records"),
            ("hanger", include_str!("../../../../fixtures/hanger/invented_hanger_library_valid.json"), "hanger_library", "hanger_records"),
        ] {
            for key in ["redistribution_status", "review_status", "privacy_class", "privacy_classification", "classification", "lifecycle_status"] {
                for place in ["wrapper", "wrapper-provenance", "record", "record-provenance"] {
                    let mut doc: Value = serde_json::from_str(document).unwrap();
                    let target = match place {
                        "wrapper" => doc.get_mut(wrapper),
                        "wrapper-provenance" => doc.get_mut(wrapper).and_then(|w| w.get_mut("provenance")),
                        "record" => doc.get_mut(records).and_then(|r| r.get_mut(0)),
                        _ => doc.get_mut(records).and_then(|r| r.get_mut(0)).and_then(|r| r.get_mut("provenance")),
                    };
                    let Some(Value::Object(target)) = target else { continue };
                    target.insert(key.to_string(), Value::Null);
                    cases += 1;
                    let model = witness_model();
                    let store = store(&format!("round6b-{kind}-{key}-{place}"), &model, false);
                    let Ok(saved) = crate::save_local_library_value(|| Ok(store.path.clone()), "project:slots".to_string(), kind.to_string(), doc) else { continue };
                    if !saved.stored { continue }
                    let row = crate::load_library(&Connection::open(&store.path).unwrap(), "project:slots", kind, &saved.library_id).unwrap().unwrap();
                    if !malformed_dispositions(&row.document).is_empty() || !screen_document(&row.document).is_empty() { accepted_then_refused.push(format!("{kind}/{place}/{key}=null")); }
                }
            }
        }
        println!("ROUND6B_IMPORT_VS_HANDOFF {cases} cases; import stores but handoff refuses: {} {accepted_then_refused:?}", accepted_then_refused.len());
        // Round 6b (owner's decision): what the import stores, the handoff screen admits.
        assert!(cases > 0 && accepted_then_refused.is_empty(), "import-accepted libraries refused: {accepted_then_refused:?}");
    }

    /// Probe: the strict screen on stored rule packs, the `classification`
    /// exemption included.
    #[test]
    fn round6b_rule_pack_malformed_and_classification_exemption() {
        let mut wrong = Vec::new();
        // Since round 6b only arrays and objects refuse as malformed; a null or
        // boolean member is saved (here `saved`), never as public.
        let cases: [(&str, fn(&mut Value), &str); 5] = [
            ("lifecycle-null", |p| p["metadata"]["lifecycle_status"] = Value::Null, "saved"),
            ("classification-array", |p| p["classification"] = json!(["protected_suspected"]), DISPOSITION_MALFORMED),
            ("classification-privacy-null", |p| p["classification"]["privacy_class"] = Value::Null, "saved"),
            ("classification-protected", |p| p["classification"]["redistribution_status"] = json!("protected_suspected"), model_document_save::PROTECTED_CONTENT_BLOCKED),
            ("nested-classification-object", |p| p["metadata"]["x"] = json!({ "classification": { "review_status": false } }), "saved"),
        ];
        for (tag, edit, code) in cases {
            let mut model = witness_model();
            let mut pack: Value = serde_json::from_str(DEMO_RULE_PACK).unwrap();
            edit(&mut pack);
            let Ok(hash) = open_pipe_stress_rule_pack_document::compute_rule_pack_checksum(&pack).map(|c| c.value) else { wrong.push(format!("{tag}: checksum")); continue };
            pack["checksums"]["rule_pack_checksum"]["value"] = json!(hash);
            model["rule_pack_refs"][0]["checksum"]["value"] = json!(hash);
            let store = store(&format!("round6b-pack-{tag}"), &model, true);
            crate::upsert_rule_pack(&Connection::open(&store.path).unwrap(), "project:slots", "invented_demo_rule_pack", &pack).unwrap();
            let state = save_state(&store, &model, "handoff_basis", "6b6b00006b6b00006b6b00006b6b0000").state;
            let result = prepare(&store, &model, &state.state_id);
            println!("ROUND6B_PACK {tag}: {:?}", codes(&result));
            let as_expected = match &result {
                Err(d) => d.iter().any(|d| d.code == code),
                Ok(prepared) => {
                    let reference = &prepared.rule_pack_refs[0];
                    code == "saved"
                        && reference["privacy_classification"] != json!("public_metadata")
                        && (tag != "classification-privacy-null" || reference["privacy_classification"] == json!("private_rule_pack_data"))
                        && (tag == "classification-privacy-null" || reference["review_classification"] == json!("TBD"))
                }
            };
            if !as_expected { wrong.push(tag.to_string()); }
        }
        assert!(wrong.is_empty(), "rule-pack strict screen: {wrong:?}");
    }

    /// Pins the pre-swap re-read (R6-1): an edit of the previous package after
    /// admission, before the swap, refuses `HANDOFF-DESTINATION-CHANGED`; the
    /// edited package stays under its name and no staging folder is left.
    #[test]
    fn round6b_previous_package_edited_before_the_swap_refuses_and_stays() {
        use std::io::Write as _;
        let (store, first) = prepared_demo("round6b-preswap");
        let target = store.root.join("target.swbhandoff");
        assert!(matches!(write(&target, first), HandoffSaveOutcome::Saved { .. }));
        let (_next_store, next) = prepared_demo("round6b-preswap-next");
        let member = target.join(MANIFEST_MEMBER);
        let member2 = member.clone();
        let edit = move || -> io::Result<()> { fs::OpenOptions::new().write(true).open(&member2)?.write_all(b"[") };
        let out = write_handoff_package(&target, next, CREATED_AT, &PublishHooks { before_commit: &edit, ..PRODUCTION_HOOKS });
        assert!(matches!(&out, HandoffSaveOutcome::Refused { diagnostics } if diagnostics[0].code == publish::DESTINATION_CHANGED), "{out:?}");
        assert_eq!(fs::read(&member).unwrap().first(), Some(&b'['));
        assert!(staging_left(&store.root).is_empty(), "{:?}", staging_left(&store.root));
    }

    /// Pins the verifier's byte re-read (R6-2): a mapped write to the model
    /// member after its read, while the manifest is read, fails verification.
    #[test]
    fn round6b_verifier_refuses_a_mapped_write_during_its_reads() {
        use std::os::fd::AsRawFd;
        let (store, first) = prepared_demo("round6b-verifier-mmap");
        let target = store.root.join("target.swbhandoff");
        assert!(matches!(write(&target, first), HandoffSaveOutcome::Saved { .. }));
        let member = target.join(publish::MODEL_MEMBER);
        let mapping: std::rc::Rc<std::cell::Cell<(usize, usize)>> = Default::default();
        let mapping2 = mapping.clone();
        publish::R4_BEFORE_OPEN.with(|hook| *hook.borrow_mut() = Some(Box::new(move || {
            publish::R4_BEFORE_OPEN.with(|hook| *hook.borrow_mut() = Some(Box::new(move || {
                let file = fs::OpenOptions::new().read(true).write(true).open(&member).unwrap();
                let len = file.metadata().unwrap().len() as usize;
                // SAFETY: a valid descriptor and length; unmapped below.
                let ptr = unsafe { libc::mmap(std::ptr::null_mut(), len, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED, file.as_raw_fd(), 0) };
                assert_ne!(ptr, libc::MAP_FAILED);
                unsafe { *(ptr as *mut u8) = b'[' };
                mapping2.set((ptr as usize, len));
            })));
        })));
        let verdict = verify_handoff_package(&target);
        let (ptr, len) = mapping.get();
        if ptr != 0 { unsafe { libc::munmap(ptr as *mut libc::c_void, len) }; }
        assert!(verdict.is_err(), "verified bytes no longer on disk: {verdict:?}");
    }

    /// #1602 merge: a basis state whose stored row cannot be read refuses the
    /// handoff with the row's refusal (`HANDOFF-STATE-STORE-UNAVAILABLE`), not
    /// as a missing state; the other rows still list.
    #[test]
    fn an_unreadable_basis_state_row_refuses_by_its_refusal() {
        let model: Value = serde_json::from_str(DEMO).unwrap();
        let store = store("merge-1602-unreadable", &model, false);
        let state = save_state(&store, &model, "handoff_basis", "16020000160200001602000016020000").state;
        assert!(prepare(&store, &model, &state.state_id).is_ok());
        // Tampered as the W-12 tests tamper a row (the immutability trigger dropped first).
        Connection::open(&store.path)
            .unwrap()
            .execute_batch("DROP TRIGGER IF EXISTS model_states_immutable; UPDATE model_states SET record_json = replace(record_json, 'HUMAN_REVIEW_REQUIRED', 'MECHANICS_SOLVED');")
            .unwrap();
        let result = prepare(&store, &model, &state.state_id);
        assert!(matches!(&result, Err(d) if d.iter().any(|d| d.code == STATE_STORE_UNAVAILABLE)), "{:?}", codes(&result));
    }
}
