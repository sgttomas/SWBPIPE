//! The one model-document validator (W-13 design `model-document-0.7.0.md`
//! §3, §5.3): a manual check of the model-data slots that carry no physics —
//! tags, notes, lines, unresolved assumptions, constraints, equipment
//! interfaces, rule-pack references and structured provenance records.
//!
//! It runs wherever a document enters or changes: native file open, the
//! webview open path (through the wasm export), project load and every
//! applier apply. It reads only; it never repairs, defaults or normalizes a
//! document. A slot is checked when it is present, whatever the document's
//! version string says; absence means none.
//!
//! Blocking findings refuse the document (or the operation whose result
//! carries them). Warnings name a condition the user should see and never
//! block: an assumption, constraint or equipment interface that names an
//! entity the document no longer has, and a provenance or rule-pack
//! disposition that the library-import policy quarantines.
//!
//! Physics validity and admission remain the solver's; the record shapes are
//! the schema's (`schemas/model.schema.yaml`). No JSON Schema library is used.
//! Nothing here evaluates a constraint or an interface (OD-1): the records are
//! data slots only.

use serde::Serialize;
use serde_json::{Map, Value};
use std::collections::{BTreeMap, BTreeSet};

pub const SEVERITY_BLOCKING: &str = "blocking";
pub const SEVERITY_WARNING: &str = "warning";

/// A slot's JSON type, a missing required member, a blank string or an
/// unknown member on one of the new records.
pub const SLOT_SHAPE: &str = "MODEL-SLOT-SHAPE";
/// Two records of one new collection share an id.
pub const SLOT_ID_DUPLICATE: &str = "MODEL-SLOT-ID-DUPLICATE";
/// A reference list (`affected_refs`, `target_refs`) names one id twice.
pub const SLOT_REF_DUPLICATE: &str = "MODEL-SLOT-REF-DUPLICATE";
pub const TAGS_INVALID: &str = "MODEL-TAGS-INVALID";
pub const NOTES_INVALID: &str = "MODEL-NOTES-INVALID";
/// `tags`, `notes` or `provenance_record` where the format does not take it.
pub const ANNOTATION_NOT_ALLOWED: &str = "MODEL-ANNOTATION-NOT-ALLOWED";
pub const LINE_PIPE_MISSING: &str = "MODEL-LINE-PIPE-MISSING";
pub const LINE_PIPE_DUPLICATE: &str = "MODEL-LINE-PIPE-DUPLICATE";
pub const LINE_PIPE_SHARED: &str = "MODEL-LINE-PIPE-SHARED";
pub const ASSUMPTION_KIND_INVALID: &str = "MODEL-ASSUMPTION-KIND-INVALID";
pub const ASSUMPTION_STATUS_INVALID: &str = "MODEL-ASSUMPTION-STATUS-INVALID";
pub const ASSUMPTION_RESOLUTION_NOTE_REQUIRED: &str = "MODEL-ASSUMPTION-RESOLUTION-NOTE-REQUIRED";
pub const CONSTRAINT_KIND_INVALID: &str = "MODEL-CONSTRAINT-KIND-INVALID";
pub const CONSTRAINT_STATE_INVALID: &str = "MODEL-CONSTRAINT-STATE-INVALID";
pub const RULE_PACK_REF_INVALID: &str = "MODEL-RULE-PACK-REF-INVALID";
pub const PROVENANCE_RECORD_INVALID: &str = "MODEL-PROVENANCE-RECORD-INVALID";
/// Warning: a reference names an entity the document does not have.
pub const ASSUMPTION_SUBJECT_MISSING: &str = "MODEL-ASSUMPTION-SUBJECT-MISSING";
/// Warning: a `protected_suspected` or `quarantined` disposition.
pub const PROVENANCE_QUARANTINE: &str = "MODEL-PROVENANCE-QUARANTINE";

/// The validator's report envelope kind (the wasm export's output).
pub const VALIDATION_DOCUMENT_KIND: &str = "openpipestress.desktop.model_document_validation";

/// Maximum characters in one tag (design §3.1).
pub const MAX_TAG_CHARS: usize = 64;

/// The existing records that take `tags` and `notes` (§3.1), with whether
/// they also take `provenance_record` (§3.5).
const ANNOTATED_COLLECTIONS: [(&str, bool); 8] = [
    ("nodes", false),
    ("pipe_segments", false),
    ("sections", true),
    ("materials", true),
    ("supports", true),
    ("components", true),
    ("load_cases", false),
    ("combinations", false),
];

/// `model_state.schema.json` `AssumptionRecord` vocabulary (§3.3).
const ASSUMPTION_KINDS: [&str; 2] = ["assumption", "open_issue"];
const ASSUMPTION_STATUSES: [&str; 3] = ["unresolved", "resolved", "rejected"];

/// `schemas/constraint.schema.json` `ConstraintKind` minus `TBD`,
/// `equipment_interface` (that is `equipment_interfaces`) and
/// `missing_required_data` (that is an open issue) — §3.6.
pub const CONSTRAINT_KINDS: [&str; 9] = [
    "connectivity",
    "clearance",
    "no_go_volume",
    "support_zone",
    "route_conflict",
    "slope",
    "drain",
    "vent",
    "access",
];
/// `ConstraintRecord.state`'s enum without `TBD`: these slots never carry
/// the TBD sentinel (design §6).
pub const CONSTRAINT_STATES: [&str; 5] = [
    "proposed",
    "active",
    "inactive",
    "superseded",
    "blocked_by_missing_data",
];

/// Provenance-record dispositions (§3.5), the library import's vocabulary.
pub const PROVENANCE_REDISTRIBUTION_STATUSES: [&str; 4] =
    ["public_permissive", "private_only", "unknown", "protected_suspected"];
pub const PROVENANCE_REVIEW_STATUSES: [&str; 4] = ["pending", "accepted", "rejected", "quarantined"];
/// `library_import_document::LibraryKind` tokens.
pub const LIBRARY_KINDS: [&str; 4] = ["material", "section", "component", "hanger"];
/// `library_import_document::REQUIRED_PROVENANCE_FIELDS`; only `source_name`
/// is required on a private model (FR-MOD-005 "can record").
pub const PROVENANCE_RECORD_FIELDS: [&str; 7] = [
    "source_name",
    "source_location",
    "source_license",
    "contributor",
    "contributor_certification",
    "redistribution_status",
    "review_status",
];

/// Rule-pack reference dispositions: §3.4's four, plus the rule-pack
/// lifecycle's `invented_non_engineering_example`, which the bundled invented
/// demo pack carries (`classification.redistribution_status`).
pub const RULE_PACK_REDISTRIBUTION_STATUSES: [&str; 5] = [
    "private_only",
    "public_permissive",
    "unknown",
    "protected_suspected",
    "invented_non_engineering_example",
];
/// A rule-pack reference's checksum: the members common to the pack's stored
/// stamped record (`checksums.rule_pack_checksum`) and the computed
/// `ComputedRulePackChecksum` (`core/rules/rule_pack_document/src/lib.rs`).
pub const RULE_PACK_CHECKSUM_MEMBERS: [&str; 5] =
    ["algorithm", "canonicalization", "payload_scope", "payload_excludes", "value"];
pub const RULE_PACK_CHECKSUM_ALGORITHM: &str = "sha256";
pub const RULE_PACK_CHECKSUM_CANONICALIZATION: &str = "JCS";
pub const RULE_PACK_CHECKSUM_PAYLOAD_SCOPE: &str = "rule_pack_payload";
pub const RULE_PACK_CHECKSUM_PAYLOAD_EXCLUDES: [&str; 1] = ["checksums"];

/// One finding: a named code, its severity, the JSON path of the offending
/// member (`""` for the whole document) and the entity ids it concerns.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DocumentDiagnostic {
    pub code: &'static str,
    pub severity: &'static str,
    pub path: String,
    pub message: String,
    pub affected_refs: Vec<String>,
}

impl DocumentDiagnostic {
    pub fn is_blocking(&self) -> bool {
        self.severity == SEVERITY_BLOCKING
    }
}

/// True when any finding blocks.
pub fn has_blocking(diagnostics: &[DocumentDiagnostic]) -> bool {
    diagnostics.iter().any(DocumentDiagnostic::is_blocking)
}

/// Validate a model document's model-data slots. Returns every finding in
/// document order; an empty list means the slots are well formed. A value
/// that is not an object has no slots to check and returns no findings (the
/// callers' own shape checks name it).
pub fn validate_model_document(model: &Value) -> Vec<DocumentDiagnostic> {
    let Some(root) = model.as_object() else {
        return Vec::new();
    };
    let mut checker = Validator { found: Vec::new() };
    checker.document(root);
    checker.found
}

/// The validator's report as JSON text (the wasm export's body):
/// `{document_kind, blocking, diagnostics}`.
pub fn validation_report(model: &Value) -> Value {
    let diagnostics = validate_model_document(model);
    serde_json::json!({
        "document_kind": VALIDATION_DOCUMENT_KIND,
        "blocking": has_blocking(&diagnostics),
        "diagnostics": diagnostics,
    })
}

struct Validator {
    found: Vec<DocumentDiagnostic>,
}

/// The entity ids a reference may name, by kind.
#[derive(Default)]
struct Entities {
    any: BTreeSet<String>,
    nodes: BTreeSet<String>,
    pipes: BTreeSet<String>,
}

fn entity_id<'a>(record: &'a Value, key: &str) -> Option<&'a str> {
    record.get(key).and_then(Value::as_str).filter(|id| !id.trim().is_empty())
}

fn array<'a>(root: &'a Map<String, Value>, member: &str) -> &'a [Value] {
    root.get(member).and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[])
}

fn entities(root: &Map<String, Value>) -> Entities {
    let mut found = Entities::default();
    if let Some(id) = root.get("project").and_then(|project| entity_id(project, "id")) {
        found.any.insert(id.to_string());
    }
    for (collection, _) in ANNOTATED_COLLECTIONS {
        for record in array(root, collection) {
            if let Some(id) = entity_id(record, "id") {
                found.any.insert(id.to_string());
                match collection {
                    "nodes" => {
                        found.nodes.insert(id.to_string());
                    }
                    "pipe_segments" => {
                        found.pipes.insert(id.to_string());
                    }
                    _ => {}
                }
            }
            if collection == "load_cases" {
                for primitive in record.get("primitive_loads").and_then(Value::as_array).into_iter().flatten() {
                    if let Some(id) = entity_id(primitive, "id") {
                        found.any.insert(id.to_string());
                    }
                }
            }
        }
    }
    for (collection, key) in [
        ("lines", "id"),
        ("unresolved_assumptions", "assumption_id"),
        ("constraints", "constraint_id"),
        ("equipment_interfaces", "id"),
    ] {
        for record in array(root, collection) {
            if let Some(id) = entity_id(record, key) {
                found.any.insert(id.to_string());
            }
        }
    }
    found
}

fn kind_name(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

fn record_ref(record: &Map<String, Value>, key: &str) -> Vec<String> {
    record
        .get(key)
        .and_then(Value::as_str)
        .filter(|id| !id.trim().is_empty())
        .map(|id| vec![id.to_string()])
        .unwrap_or_default()
}

impl Validator {
    fn push(&mut self, code: &'static str, severity: &'static str, path: &str, message: String, affected_refs: Vec<String>) {
        self.found.push(DocumentDiagnostic { code, severity, path: path.to_string(), message, affected_refs });
    }

    fn block(&mut self, code: &'static str, path: &str, message: String, affected_refs: Vec<String>) {
        self.push(code, SEVERITY_BLOCKING, path, message, affected_refs);
    }

    fn warn(&mut self, code: &'static str, path: &str, message: String, affected_refs: Vec<String>) {
        self.push(code, SEVERITY_WARNING, path, message, affected_refs);
    }

    fn document(&mut self, root: &Map<String, Value>) {
        let known = entities(root);

        // §3.1: tags and notes on the project and the existing records;
        // provenance_record only on materials, sections, components, supports.
        if let Some(project) = root.get("project").and_then(Value::as_object) {
            let refs = record_ref(project, "id");
            self.annotations(project, "project", &refs);
            self.not_allowed(project, "project", &["provenance_record"], &refs);
        }
        for (collection, takes_provenance_record) in ANNOTATED_COLLECTIONS {
            for (index, record) in array(root, collection).iter().enumerate() {
                let Some(record) = record.as_object() else { continue };
                let at = format!("{collection}[{index}]");
                let refs = record_ref(record, "id");
                self.annotations(record, &at, &refs);
                if takes_provenance_record {
                    if let Some(value) = record.get("provenance_record") {
                        self.provenance_record(value, &format!("{at}.provenance_record"), &refs);
                    }
                } else {
                    self.not_allowed(record, &at, &["provenance_record"], &refs);
                }
            }
        }

        // §3: never on the load/reference-state group or the pressure contract
        // (closed solve inputs).
        if let Some(group) = root.get("reference_configurations") {
            self.scan_forbidden(group, "reference_configurations");
        }
        if let Some(contract) = root.get("pressure_contract") {
            self.scan_forbidden(contract, "pressure_contract");
        }
        for (index, material) in array(root, "materials").iter().enumerate() {
            if let Some(laws) = material.get("expansion_laws") {
                self.scan_forbidden(laws, &format!("materials[{index}].expansion_laws"));
            }
        }
        for (index, case) in array(root, "load_cases").iter().enumerate() {
            if let Some(state) = case.get("analysis_state") {
                self.scan_forbidden(state, &format!("load_cases[{index}].analysis_state"));
            }
        }

        self.lines(root, &known);
        self.assumptions(root, &known);
        self.constraints(root, &known);
        self.equipment_interfaces(root, &known);
        self.rule_pack_refs(root);
        self.model_settings(root);
    }

    // ---- shared member checks ------------------------------------------------

    fn annotations(&mut self, record: &Map<String, Value>, at: &str, refs: &[String]) {
        if let Some(tags) = record.get("tags") {
            self.tags(tags, &format!("{at}.tags"), refs);
        }
        if let Some(notes) = record.get("notes") {
            let path = format!("{at}.notes");
            match notes.as_str() {
                Some(text) if !text.trim().is_empty() => {}
                Some(_) => self.block(NOTES_INVALID, &path, format!("{path} is blank; notes are one non-empty string (clearing notes removes the member)."), refs.to_vec()),
                None => self.block(NOTES_INVALID, &path, format!("{path} is {}; notes are one non-empty string.", kind_name(notes)), refs.to_vec()),
            }
        }
    }

    fn tags(&mut self, tags: &Value, path: &str, refs: &[String]) {
        let Some(items) = tags.as_array() else {
            self.block(TAGS_INVALID, path, format!("{path} is {}; tags are an array of strings.", kind_name(tags)), refs.to_vec());
            return;
        };
        let mut seen = BTreeSet::new();
        for (index, item) in items.iter().enumerate() {
            let at = format!("{path}[{index}]");
            let Some(tag) = item.as_str() else {
                self.block(TAGS_INVALID, &at, format!("{at} is {}; a tag is a string.", kind_name(item)), refs.to_vec());
                continue;
            };
            if tag.trim().is_empty() {
                self.block(TAGS_INVALID, &at, format!("{at} is blank; a tag is a non-empty string."), refs.to_vec());
            } else if tag.trim() != tag {
                self.block(TAGS_INVALID, &at, format!("{at} `{tag}` has leading or trailing whitespace; tags are trimmed."), refs.to_vec());
            } else if tag.chars().count() > MAX_TAG_CHARS {
                self.block(TAGS_INVALID, &at, format!("{at} has {} characters; a tag has at most {MAX_TAG_CHARS}.", tag.chars().count()), refs.to_vec());
            } else if !seen.insert(tag) {
                self.block(TAGS_INVALID, &at, format!("{at} `{tag}` repeats a tag; tags are unique."), refs.to_vec());
            }
        }
    }

    fn not_allowed(&mut self, record: &Map<String, Value>, at: &str, members: &[&str], refs: &[String]) {
        for member in members {
            if record.contains_key(*member) {
                let path = format!("{at}.{member}");
                self.block(ANNOTATION_NOT_ALLOWED, &path, format!("{path} is not part of the model format: `{member}` is not accepted on this record."), refs.to_vec());
            }
        }
    }

    fn scan_forbidden(&mut self, value: &Value, path: &str) {
        match value {
            Value::Object(map) => {
                for member in ["tags", "notes", "provenance_record"] {
                    if map.contains_key(member) {
                        let at = format!("{path}.{member}");
                        self.block(ANNOTATION_NOT_ALLOWED, &at, format!("{at}: `{member}` is not accepted on the load/reference-state group or the pressure contract, which are closed solve inputs."), record_ref(map, "id"));
                    }
                }
                for (key, item) in map {
                    self.scan_forbidden(item, &format!("{path}.{key}"));
                }
            }
            Value::Array(items) => {
                for (index, item) in items.iter().enumerate() {
                    self.scan_forbidden(item, &format!("{path}[{index}]"));
                }
            }
            _ => {}
        }
    }

    /// The new top-level collection at `member`: absent, or an array of
    /// objects. Returns the records with their paths.
    fn collection<'a>(&mut self, root: &'a Map<String, Value>, member: &str) -> Vec<(String, &'a Map<String, Value>)> {
        let Some(value) = root.get(member) else { return Vec::new() };
        let Some(items) = value.as_array() else {
            self.block(SLOT_SHAPE, member, format!("{member} is {}; array required.", kind_name(value)), Vec::new());
            return Vec::new();
        };
        let mut records = Vec::new();
        for (index, item) in items.iter().enumerate() {
            let at = format!("{member}[{index}]");
            match item.as_object() {
                Some(record) => records.push((at, record)),
                None => self.block(SLOT_SHAPE, &at, format!("{at} is {}; object required.", kind_name(item)), Vec::new()),
            }
        }
        records
    }

    /// Closed record: only `allowed` members; every `required` member present.
    fn members(&mut self, record: &Map<String, Value>, at: &str, required: &[&str], allowed: &[&str], refs: &[String]) {
        for key in record.keys() {
            if !allowed.contains(&key.as_str()) {
                let path = format!("{at}.{key}");
                self.block(SLOT_SHAPE, &path, format!("{path} is not a member of this record."), refs.to_vec());
            }
        }
        for key in required {
            if !record.contains_key(*key) {
                let path = format!("{at}.{key}");
                self.block(SLOT_SHAPE, &path, format!("{path} is missing (required)."), refs.to_vec());
            }
        }
    }

    /// A present member that must be a non-blank string; returns it.
    fn text<'a>(&mut self, record: &'a Map<String, Value>, at: &str, key: &str, code: &'static str, refs: &[String]) -> Option<&'a str> {
        let value = record.get(key)?;
        let path = format!("{at}.{key}");
        match value.as_str() {
            Some(text) if !text.trim().is_empty() => Some(text),
            Some(_) => {
                self.block(code, &path, format!("{path} is blank; a non-empty string is required."), refs.to_vec());
                None
            }
            None => {
                self.block(code, &path, format!("{path} is {}; string required.", kind_name(value)), refs.to_vec());
                None
            }
        }
    }

    fn texts(&mut self, record: &Map<String, Value>, at: &str, keys: &[&str], code: &'static str, refs: &[String]) {
        for key in keys {
            self.text(record, at, key, code, refs);
        }
    }

    /// A present enum member; returns it when admitted.
    fn token<'a>(&mut self, record: &'a Map<String, Value>, at: &str, key: &str, admitted: &[&str], code: &'static str, refs: &[String]) -> Option<&'a str> {
        let value = record.get(key)?;
        let path = format!("{at}.{key}");
        match value.as_str() {
            Some(token) if admitted.contains(&token) => Some(token),
            _ => {
                self.block(code, &path, format!("{path} is {}; one of {} required.", describe(value), admitted.join(", ")), refs.to_vec());
                None
            }
        }
    }

    /// Unique ids within one new collection.
    fn unique_ids(&mut self, records: &[(String, &Map<String, Value>)], key: &str) {
        let mut first: BTreeMap<&str, &str> = BTreeMap::new();
        for (at, record) in records {
            let Some(id) = record.get(key).and_then(Value::as_str).filter(|id| !id.trim().is_empty()) else { continue };
            if let Some(earlier) = first.get(id) {
                let path = format!("{at}.{key}");
                self.block(SLOT_ID_DUPLICATE, &path, format!("{path} `{id}` repeats the id of {earlier}; ids are unique within the collection."), vec![id.to_string()]);
            } else {
                first.insert(id, at);
            }
        }
    }

    /// An entity-reference list: an array of unique non-blank strings. Each
    /// id the document lacks is a missing-subject warning, never a block.
    fn subject_refs(&mut self, record: &Map<String, Value>, at: &str, key: &str, known: &Entities, owner: &[String]) {
        let Some(value) = record.get(key) else { return };
        let path = format!("{at}.{key}");
        let Some(items) = value.as_array() else {
            self.block(SLOT_SHAPE, &path, format!("{path} is {}; array of entity ids required.", kind_name(value)), owner.to_vec());
            return;
        };
        let mut seen = BTreeSet::new();
        for (index, item) in items.iter().enumerate() {
            let item_at = format!("{path}[{index}]");
            match item.as_str() {
                Some(id) if !id.trim().is_empty() => {
                    if !seen.insert(id) {
                        self.block(SLOT_REF_DUPLICATE, &item_at, format!("{item_at} `{id}` is named twice."), owner.to_vec());
                    } else if !known.any.contains(id) {
                        self.missing_subject(&item_at, id, "entity", owner);
                    }
                }
                _ => self.block(SLOT_SHAPE, &item_at, format!("{item_at} is {}; a non-empty entity id is required.", describe(item)), owner.to_vec()),
            }
        }
    }

    fn missing_subject(&mut self, path: &str, id: &str, kind: &str, owner: &[String]) {
        let mut affected = owner.to_vec();
        affected.push(id.to_string());
        self.warn(ASSUMPTION_SUBJECT_MISSING, path, format!("{path} names {kind} `{id}`, which this document does not have; the record is kept as authored."), affected);
    }

    // ---- §3.2 lines ----------------------------------------------------------

    fn lines(&mut self, root: &Map<String, Value>, known: &Entities) {
        let records = self.collection(root, "lines");
        self.unique_ids(&records, "id");
        let mut owner_of: BTreeMap<String, String> = BTreeMap::new();
        for (at, record) in &records {
            let refs = record_ref(record, "id");
            self.members(record, at, &["id", "pipe_refs"], &["id", "label", "pipe_refs", "tags", "notes", "provenance"], &refs);
            self.texts(record, at, &["id", "label", "provenance"], SLOT_SHAPE, &refs);
            self.annotations(record, at, &refs);
            let Some(value) = record.get("pipe_refs") else { continue };
            let path = format!("{at}.pipe_refs");
            let Some(items) = value.as_array() else {
                self.block(SLOT_SHAPE, &path, format!("{path} is {}; array of pipe ids required.", kind_name(value)), refs.clone());
                continue;
            };
            let line_id = refs.first().cloned().unwrap_or_else(|| at.clone());
            let mut seen = BTreeSet::new();
            for (index, item) in items.iter().enumerate() {
                let item_at = format!("{path}[{index}]");
                let Some(pipe) = item.as_str().filter(|id| !id.trim().is_empty()) else {
                    self.block(SLOT_SHAPE, &item_at, format!("{item_at} is {}; a non-empty pipe id is required.", describe(item)), refs.clone());
                    continue;
                };
                if !seen.insert(pipe) {
                    self.block(LINE_PIPE_DUPLICATE, &item_at, format!("{item_at} `{pipe}` is named twice in the line."), vec![line_id.clone(), pipe.to_string()]);
                    continue;
                }
                if !known.pipes.contains(pipe) {
                    self.block(LINE_PIPE_MISSING, &item_at, format!("{item_at} `{pipe}` is not a pipe segment of this document; a line groups existing pipes."), vec![line_id.clone(), pipe.to_string()]);
                    continue;
                }
                match owner_of.get(pipe) {
                    Some(other) => self.block(LINE_PIPE_SHARED, &item_at, format!("{item_at} `{pipe}` already belongs to line `{other}`; a pipe belongs to at most one line."), vec![other.clone(), line_id.clone(), pipe.to_string()]),
                    None => {
                        owner_of.insert(pipe.to_string(), line_id.clone());
                    }
                }
            }
        }
    }

    // ---- §3.3 unresolved assumptions -----------------------------------------

    fn assumptions(&mut self, root: &Map<String, Value>, known: &Entities) {
        let records = self.collection(root, "unresolved_assumptions");
        self.unique_ids(&records, "assumption_id");
        for (at, record) in &records {
            let refs = record_ref(record, "assumption_id");
            self.members(
                record,
                at,
                &["assumption_id", "kind", "statement", "status"],
                &["assumption_id", "kind", "statement", "status", "affected_refs", "resolution_note", "tags", "notes"],
                &refs,
            );
            self.texts(record, at, &["assumption_id", "statement"], SLOT_SHAPE, &refs);
            self.token(record, at, "kind", &ASSUMPTION_KINDS, ASSUMPTION_KIND_INVALID, &refs);
            let status = self.token(record, at, "status", &ASSUMPTION_STATUSES, ASSUMPTION_STATUS_INVALID, &refs);
            let note = record.get("resolution_note");
            if note.is_some() {
                self.text(record, at, "resolution_note", SLOT_SHAPE, &refs);
            } else if let Some(status @ ("resolved" | "rejected")) = status {
                let path = format!("{at}.resolution_note");
                self.block(ASSUMPTION_RESOLUTION_NOTE_REQUIRED, &path, format!("{path} is missing; a {status} record states how it was {status}."), refs.clone());
            }
            self.subject_refs(record, at, "affected_refs", known, &refs);
            self.annotations(record, at, &refs);
        }
    }

    // ---- §3.6 constraints and equipment interfaces ---------------------------

    fn constraints(&mut self, root: &Map<String, Value>, known: &Entities) {
        let records = self.collection(root, "constraints");
        self.unique_ids(&records, "constraint_id");
        for (at, record) in &records {
            let refs = record_ref(record, "constraint_id");
            self.members(
                record,
                at,
                &["constraint_id", "name", "constraint_kind", "state"],
                &["constraint_id", "name", "constraint_kind", "state", "target_refs", "statement", "tags", "notes", "provenance"],
                &refs,
            );
            self.texts(record, at, &["constraint_id", "name", "statement", "provenance"], SLOT_SHAPE, &refs);
            self.token(record, at, "constraint_kind", &CONSTRAINT_KINDS, CONSTRAINT_KIND_INVALID, &refs);
            self.token(record, at, "state", &CONSTRAINT_STATES, CONSTRAINT_STATE_INVALID, &refs);
            self.subject_refs(record, at, "target_refs", known, &refs);
            self.annotations(record, at, &refs);
        }
    }

    fn equipment_interfaces(&mut self, root: &Map<String, Value>, known: &Entities) {
        let records = self.collection(root, "equipment_interfaces");
        self.unique_ids(&records, "id");
        for (at, record) in &records {
            let refs = record_ref(record, "id");
            self.members(
                record,
                at,
                &["id", "node_ref"],
                &["id", "label", "node_ref", "equipment_label", "tags", "notes", "provenance"],
                &refs,
            );
            self.texts(record, at, &["id", "label", "equipment_label", "provenance"], SLOT_SHAPE, &refs);
            if let Some(node) = self.text(record, at, "node_ref", SLOT_SHAPE, &refs) {
                if !known.nodes.contains(node) {
                    self.missing_subject(&format!("{at}.node_ref"), node, "node", &refs);
                }
            }
            self.annotations(record, at, &refs);
        }
    }

    // ---- §3.4 rule-pack references -------------------------------------------

    fn rule_pack_refs(&mut self, root: &Map<String, Value>) {
        let records = self.collection(root, "rule_pack_refs");
        self.unique_ids(&records, "rule_pack_id");
        for (at, record) in &records {
            let refs = record_ref(record, "rule_pack_id");
            self.members(
                record,
                at,
                &["rule_pack_id", "name", "rule_pack_version", "checksum", "redistribution_status"],
                &["rule_pack_id", "name", "rule_pack_version", "checksum", "redistribution_status", "source_notice"],
                &refs,
            );
            self.text(record, at, "rule_pack_id", RULE_PACK_REF_INVALID, &refs);
            self.texts(record, at, &["name", "rule_pack_version", "source_notice"], RULE_PACK_REF_INVALID, &refs);
            let status = self.token(record, at, "redistribution_status", &RULE_PACK_REDISTRIBUTION_STATUSES, RULE_PACK_REF_INVALID, &refs);
            if status == Some("protected_suspected") {
                let path = format!("{at}.redistribution_status");
                self.warn(PROVENANCE_QUARANTINE, &path, format!("{path} is protected_suspected: the library-import policy quarantines this disposition; request human review before any export."), refs.clone());
            }
            if let Some(checksum) = record.get("checksum") {
                self.rule_pack_checksum(checksum, &format!("{at}.checksum"), &refs);
            }
        }
    }

    /// The common subset of the pack's stored stamped record
    /// (`checksums.rule_pack_checksum`) and `ComputedRulePackChecksum`:
    /// closed, every member required. A reference's identity is
    /// `rule_pack_id` + `rule_pack_version` + `checksum.value`.
    fn rule_pack_checksum(&mut self, value: &Value, at: &str, refs: &[String]) {
        let code = RULE_PACK_REF_INVALID;
        let Some(checksum) = value.as_object() else {
            self.block(code, at, format!("{at} is {}; the pack's checksum record (an object) is required.", kind_name(value)), refs.to_vec());
            return;
        };
        for key in checksum.keys() {
            if !RULE_PACK_CHECKSUM_MEMBERS.contains(&key.as_str()) {
                let path = format!("{at}.{key}");
                self.block(code, &path, format!("{path} is not a member of the rule-pack checksum reference ({}).", RULE_PACK_CHECKSUM_MEMBERS.join(", ")), refs.to_vec());
            }
        }
        let fixed = |key: &str, expected: &str, this: &mut Self| {
            let path = format!("{at}.{key}");
            match checksum.get(key) {
                Some(Value::String(found)) if found == expected => {}
                Some(found) => this.block(code, &path, format!("{path} is {}; `{expected}` required.", describe(found)), refs.to_vec()),
                None => this.block(code, &path, format!("{path} is missing; `{expected}` required."), refs.to_vec()),
            }
        };
        fixed("algorithm", RULE_PACK_CHECKSUM_ALGORITHM, self);
        fixed("canonicalization", RULE_PACK_CHECKSUM_CANONICALIZATION, self);
        fixed("payload_scope", RULE_PACK_CHECKSUM_PAYLOAD_SCOPE, self);
        let excludes_path = format!("{at}.payload_excludes");
        let excludes_ok = checksum.get("payload_excludes").and_then(Value::as_array).is_some_and(|items| {
            items.len() == RULE_PACK_CHECKSUM_PAYLOAD_EXCLUDES.len()
                && items.iter().zip(RULE_PACK_CHECKSUM_PAYLOAD_EXCLUDES).all(|(item, expected)| item.as_str() == Some(expected))
        });
        if !excludes_ok {
            self.block(code, &excludes_path, format!("{excludes_path} must be [\"{}\"].", RULE_PACK_CHECKSUM_PAYLOAD_EXCLUDES.join("\", \"")), refs.to_vec());
        }
        let value_path = format!("{at}.value");
        if !checksum.get("value").and_then(Value::as_str).is_some_and(is_sha256_hex) {
            self.block(code, &value_path, format!("{value_path} must be 64 lowercase hexadecimal digits (SHA-256)."), refs.to_vec());
        }
    }

    // ---- §3.8 model settings ------------------------------------------------

    /// `model_settings`: a closed record. `node_proximity_tolerance` is a length quantity
    /// `{value, unit}`: finite, greater than zero, in a catalog length unit; `shear_model`
    /// (model document 0.8.0) is one of the two names and never beside a pressure contract.
    /// The value checks run before the unit check, as in product_physics' accessor, and give
    /// its codes and messages.
    fn model_settings(&mut self, root: &Map<String, Value>) {
        use open_pipe_stress_product_physics::model_settings as ms;
        let Some(value) = root.get("model_settings") else { return };
        let Some(settings) = value.as_object() else {
            self.block(SLOT_SHAPE, "model_settings", format!("model_settings is {}; object required.", kind_name(value)), Vec::new());
            return;
        };
        self.members(settings, "model_settings", &[], &["node_proximity_tolerance", "shear_model"], &[]);
        // Model document 0.8.0: `shear_model`, one of the two names, never beside a
        // pressure contract; `timoshenko_annulus_energy_v1` is schema-valid here and is
        // refused at solve and at authoring.
        if let Some(value) = settings.get("shear_model") {
            let has_contract = root.get("pressure_contract").is_some_and(|contract| !contract.is_null());
            if let Some(error) = ms::shear_model_document_refusal(ms::ShearModelSetting::of(value), has_contract) {
                self.block(error.code, ms::SHEAR_MODEL_PATH, error.message, Vec::new());
            }
        }
        let Some(tolerance) = settings.get("node_proximity_tolerance") else { return };
        let at = ms::NODE_PROXIMITY_TOLERANCE_PATH;
        let Some(quantity) = tolerance.as_object() else {
            self.block(SLOT_SHAPE, at, format!("{at} is {}; a length quantity {{value, unit}} is required.", kind_name(tolerance)), Vec::new());
            return;
        };
        self.members(quantity, at, &["value", "unit"], &["value", "unit"], &[]);
        let number = match quantity.get("value") {
            Some(Value::Number(number)) => number.as_f64(),
            Some(other) => {
                let path = format!("{at}.value");
                self.block(SLOT_SHAPE, &path, format!("{path} is {}; number required.", kind_name(other)), Vec::new());
                None
            }
            None => None,
        };
        let unit = match quantity.get("unit") {
            Some(Value::String(unit)) => Some(unit.as_str()),
            Some(other) => {
                let path = format!("{at}.unit");
                self.block(SLOT_SHAPE, &path, format!("{path} is {}; string required.", kind_name(other)), Vec::new());
                None
            }
            None => None,
        };
        let mut admitted = true;
        if let Some(number) = number {
            if !number.is_finite() {
                admitted = false;
                self.block(ms::MODEL_SETTING_TOLERANCE_NOT_FINITE, &format!("{at}.value"), ms::tolerance_not_finite_message(number), Vec::new());
            } else if number <= 0.0 {
                admitted = false;
                self.block(ms::MODEL_SETTING_TOLERANCE_NOT_POSITIVE, &format!("{at}.value"), ms::tolerance_not_positive_message(number), Vec::new());
            }
        }
        if let Some(unit) = unit {
            if !ms::is_catalog_length_unit(unit) {
                admitted = false;
                self.block(ms::MODEL_SETTING_TOLERANCE_UNIT_NOT_LENGTH, &format!("{at}.unit"), ms::tolerance_unit_not_length_message(unit), Vec::new());
            }
        }
        // A value and unit each admitted can still be zero or non-finite in metres; the
        // accessor's own conversion decides, with its code and message.
        if let (true, Some(number), Some(unit)) = (admitted, number, unit) {
            let quantity = open_pipe_stress_product_physics::Quantity { value: number, unit: unit.to_owned() };
            if let Err(error) = ms::node_proximity_tolerance_m(&quantity) {
                self.block(error.code, at, error.message, Vec::new());
            }
        }
    }

    // ---- §3.5 structured provenance ------------------------------------------

    fn provenance_record(&mut self, value: &Value, at: &str, refs: &[String]) {
        let code = PROVENANCE_RECORD_INVALID;
        let Some(record) = value.as_object() else {
            self.block(code, at, format!("{at} is {}; object required.", kind_name(value)), refs.to_vec());
            return;
        };
        let allowed = [
            "source_name",
            "source_location",
            "source_license",
            "contributor",
            "contributor_certification",
            "redistribution_status",
            "review_status",
            "reviewed_by",
            "reviewed_on",
            "verification_note",
            "library_ref",
        ];
        for key in record.keys() {
            if !allowed.contains(&key.as_str()) {
                let path = format!("{at}.{key}");
                self.block(code, &path, format!("{path} is not a member of the provenance record."), refs.to_vec());
            }
        }
        if !record.contains_key("source_name") {
            let path = format!("{at}.source_name");
            self.block(code, &path, format!("{path} is missing; a provenance record names its source."), refs.to_vec());
        }
        self.texts(
            record,
            at,
            &["source_name", "source_location", "source_license", "contributor", "contributor_certification", "reviewed_by", "verification_note"],
            code,
            refs,
        );
        let redistribution = self.token(record, at, "redistribution_status", &PROVENANCE_REDISTRIBUTION_STATUSES, code, refs);
        let review = self.token(record, at, "review_status", &PROVENANCE_REVIEW_STATUSES, code, refs);
        if let Some(date) = self.text(record, at, "reviewed_on", code, refs) {
            if !is_calendar_date(date) {
                let path = format!("{at}.reviewed_on");
                self.block(code, &path, format!("{path} `{date}` is not a YYYY-MM-DD calendar date."), refs.to_vec());
            }
        }
        if let Some(library) = record.get("library_ref") {
            let path = format!("{at}.library_ref");
            match library.as_object() {
                None => self.block(code, &path, format!("{path} is {}; object required.", kind_name(library)), refs.to_vec()),
                Some(library) => {
                    for key in library.keys() {
                        if !["library_kind", "library_id", "library_version", "record_id"].contains(&key.as_str()) {
                            let member = format!("{path}.{key}");
                            self.block(code, &member, format!("{member} is not a member of the library reference."), refs.to_vec());
                        }
                    }
                    for key in ["library_kind", "library_id"] {
                        if !library.contains_key(key) {
                            let member = format!("{path}.{key}");
                            self.block(code, &member, format!("{member} is missing (required)."), refs.to_vec());
                        }
                    }
                    self.token(library, &path, "library_kind", &LIBRARY_KINDS, code, refs);
                    self.texts(library, &path, &["library_id", "library_version", "record_id"], code, refs);
                }
            }
        }
        let quarantined = match (redistribution, review) {
            (Some("protected_suspected"), _) => Some(("redistribution_status", "protected_suspected")),
            (_, Some("quarantined")) => Some(("review_status", "quarantined")),
            _ => None,
        };
        if let Some((member, token)) = quarantined {
            let path = format!("{at}.{member}");
            self.warn(PROVENANCE_QUARANTINE, &path, format!("{path} is {token}: the library-import policy quarantines this record's data; request human review before any export."), refs.to_vec());
        }
    }
}

fn describe(value: &Value) -> String {
    match value {
        Value::String(text) => format!("`{text}`"),
        other => kind_name(other).to_string(),
    }
}

fn is_sha256_hex(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}

fn is_calendar_date(text: &str) -> bool {
    let bytes = text.as_bytes();
    if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
        return false;
    }
    let digits = |range: std::ops::Range<usize>| -> Option<u32> {
        let part = &text[range];
        part.bytes().all(|byte| byte.is_ascii_digit()).then(|| part.parse().ok()).flatten()
    };
    let (Some(year), Some(month), Some(day)) = (digits(0..4), digits(5..7), digits(8..10)) else {
        return false;
    };
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => return false,
    };
    (1..=days).contains(&day)
}

#[cfg(test)]
#[path = "document_validator_tests.rs"]
mod tests;
