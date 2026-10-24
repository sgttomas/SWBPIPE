//! Named immutable model states in the desktop store (W-12 S1; DEL-14-01
//! Design `desktop-states-runs-comparison.md` §2; FR-CMP-001 and FR-MOD-006's
//! state level).
//!
//! A state is the canonical RFC 8785 JCS text of one model document (the
//! session's W-6a bytes) and its SHA-256, the session's model hash, stored in
//! `model_states` (store v12) with a `model_state.schema.json`
//! `ModelStateRecord`. The rows are immutable (`model_states_immutable`
//! raises `MODEL-STATE-IMMUTABLE` on any `UPDATE`). The state's name, tags,
//! notes and state-level assumptions are its labels (`model_state_labels`):
//! the only editable part, joined into the record when it is read and left out
//! of the record hash (HELP_HUMAN, 2026-10-23: a label edit is not a new
//! state).
//!
//! Rust is the authority for both hashes:
//! - the snapshot: the submitted text must be its own unchecked
//!   `canonical_json` and its SHA-256 must equal the claimed model hash
//!   (`MODEL-STATE-HASH-MISMATCH` otherwise);
//! - the record (`model_state_record`): SHA-256 of the sorted-compact JSON of
//!   the record without `hashes` and without the label fields, with the
//!   serializer pinned to `core/project_persistence/service.py`'s
//!   `canonical_json` (`json.dumps(sort_keys=True, separators=(",", ":"),
//!   ensure_ascii=True)`), checked against Python by
//!   `fixtures/persistence/sorted_compact_json_vectors.json` and the record
//!   witness `fixtures/persistence/model_state_record_witness.json`.
//!
//! A state needs its project's stored row and is written in one
//! `BEGIN IMMEDIATE` transaction with that row's stale-window check: the
//! session's base (the stored envelope hash it last opened or saved) must still
//! be the stored one (`PROJECT-BASE-MOVED`). Recorded runs (`analysis_runs.rs`,
//! store v13) are tied to states; a state's deletion is blocked by its child
//! states and by its recorded runs.
//!
//! The record is a review and reproducibility record. Nothing here approves,
//! accepts, certifies or issues anything (F-PIP-2, F-PIP-3).

use crate::command_ingress::{self, respond, Args, ArgsDecoding, CommandError};
use crate::solve_jobs::SolveJobRegistry;
use crate::stored_read::{self, Metered, PoolReadMemory, ReadMemory, ReadReservation, UnmeteredReadMemory};
use open_pipe_stress_operation_applier::{
    canonical_json, sha256_hex, validate_model_document, DocumentDiagnostic,
};
use rusqlite::{params, Connection, OptionalExtension, Transaction};
use serde::Serialize;
use serde_json::{json, Map, Value};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::AppHandle;

/// Store v12: the state and label tables and the immutability triggers, as
/// design §2 gives them. Every statement is `IF NOT EXISTS`, so the step is safe
/// to re-run (the store's migrations are not transactional).
/// `model_states_immutable` refuses any `UPDATE`. `model_states_no_replace`
/// refuses an `INSERT` that meets an existing row of the project on either key
/// (`state_id` or `model_hash`): a BEFORE INSERT trigger fires before conflict
/// resolution, so `INSERT OR REPLACE` and `REPLACE` can neither overwrite a
/// snapshot nor cascade-delete its labels.
pub(crate) const STORE_V12_MODEL_STATES_SQL: &str = "
CREATE TABLE IF NOT EXISTS model_states(
  project_id TEXT NOT NULL REFERENCES local_projects(project_id) ON DELETE CASCADE,
  state_id TEXT NOT NULL, state_kind TEXT NOT NULL, created_at TEXT NOT NULL,
  parent_state_id TEXT, model_canonical TEXT NOT NULL, model_hash TEXT NOT NULL,
  record_json TEXT NOT NULL, record_hash TEXT NOT NULL,
  PRIMARY KEY(project_id, state_id), UNIQUE(project_id, model_hash),
  FOREIGN KEY(project_id, parent_state_id) REFERENCES model_states(project_id, state_id));
CREATE TABLE IF NOT EXISTS model_state_labels(
  project_id TEXT NOT NULL, state_id TEXT NOT NULL, state_name TEXT NOT NULL,
  tags_json TEXT NOT NULL, notes_json TEXT NOT NULL, assumptions_json TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  PRIMARY KEY(project_id, state_id),
  FOREIGN KEY(project_id, state_id) REFERENCES model_states(project_id, state_id) ON DELETE CASCADE);
CREATE TRIGGER IF NOT EXISTS model_states_immutable BEFORE UPDATE ON model_states
  BEGIN SELECT RAISE(ABORT, 'MODEL-STATE-IMMUTABLE'); END;
CREATE TRIGGER IF NOT EXISTS model_states_no_replace BEFORE INSERT ON model_states
  WHEN EXISTS (SELECT 1 FROM model_states WHERE project_id = NEW.project_id
    AND (state_id = NEW.state_id OR model_hash = NEW.model_hash))
  BEGIN SELECT RAISE(ABORT, 'MODEL-STATE-IMMUTABLE'); END;
";

pub(crate) fn migrate_v12_model_states(connection: &Connection) -> Result<(), String> {
    connection
        .execute_batch(STORE_V12_MODEL_STATES_SQL)
        .map_err(|error| error.to_string())
}

/// `ModelStateRecord.state_kind` without `TBD`; the desktop default is the first.
pub(crate) const STATE_KINDS: [&str; 5] = [
    "design_snapshot",
    "pre_solve_snapshot",
    "post_solve_snapshot",
    "comparison_basis",
    "handoff_basis",
];

/// Ingress budgets (raw argument bytes) of the small model-state commands.
/// `save_model_state` carries the snapshot and is bounded by the
/// per-invocation budget through its lease, as `save_local_project` is.
pub(crate) const LIST_MODEL_STATES_MAX_ARGS_BYTES: usize = 4 * 1024;
pub(crate) const READ_MODEL_STATE_MAX_ARGS_BYTES: usize = 4 * 1024;
pub(crate) const DELETE_MODEL_STATE_MAX_ARGS_BYTES: usize = 4 * 1024;
pub(crate) const EDIT_MODEL_STATE_LABELS_MAX_ARGS_BYTES: usize = 256 * 1024;

/// The longest tag, as for model tags (DEL-02-01 `model-document-0.7.0.md` §3.1).
const MAX_TAG_CHARS: usize = 64;
/// The most tags, notes and assumptions one set of labels keeps (repair r2).
pub(crate) const MAX_LABEL_ITEMS: usize = 256;
/// The most JSON one set of labels keeps (name, tags, notes, assumptions), below
/// the label edit's 256 KiB argument budget (repair r2).
pub(crate) const MAX_LABELS_JSON_BYTES: u64 = 192 * 1024;
const ASSUMPTION_KINDS: [&str; 2] = ["assumption", "open_issue"];
const ASSUMPTION_STATUSES: [&str; 3] = ["unresolved", "resolved", "rejected"];

// ---------------------------------------------------------------------------
// The sorted-compact serializer (pinned to service.py's `canonical_json`).
// ---------------------------------------------------------------------------

/// The record hash refuses numbers it cannot write exactly as Python does.
/// `model_state.schema.json` gives no field of a `ModelStateRecord` a number
/// or integer type, so a hashed record never holds a float: the serializer
/// fails closed instead of approximating Python's `repr(float)` (whose
/// shortest-digit tie-breaking Rust's formatter does not share). Integers
/// within i64/u64 are written exactly in decimal, as Python writes them; any
/// other number (a float, or an integer serde_json could only hold as a
/// float, such as 18446744073709551616) is refused.
pub(crate) const RECORD_NUMBER_UNSUPPORTED: &str = "MODEL-STATE-RECORD-NUMBER-UNSUPPORTED";

/// `json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)`
/// for the values a hashed record may hold: keys sorted by code point, no
/// whitespace, every character outside printable ASCII escaped as lowercase
/// `\uXXXX` (UTF-16 surrogate pairs above U+FFFF), Python's short escapes and
/// i64/u64 integers in decimal. Any other number is refused
/// ([`RECORD_NUMBER_UNSUPPORTED`]). This is not JCS.
pub(crate) fn sorted_compact_json(value: &Value) -> Result<String, String> {
    let mut out = String::new();
    write_sorted_compact(value, &mut out)?;
    Ok(out)
}

fn write_sorted_compact(value: &Value, out: &mut String) -> Result<(), String> {
    match value {
        Value::Null => out.push_str("null"),
        Value::Bool(true) => out.push_str("true"),
        Value::Bool(false) => out.push_str("false"),
        Value::Number(number) => write_exact_integer(number, out)?,
        Value::String(text) => write_ascii_string(text, out),
        Value::Array(items) => {
            out.push('[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                write_sorted_compact(item, out)?;
            }
            out.push(']');
        }
        Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            out.push('{');
            for (index, key) in keys.into_iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                write_ascii_string(key, out);
                out.push(':');
                write_sorted_compact(&map[key], out)?;
            }
            out.push('}');
        }
    }
    Ok(())
}

fn write_exact_integer(number: &serde_json::Number, out: &mut String) -> Result<(), String> {
    if let Some(integer) = number.as_i64() {
        let _ = write!(out, "{integer}");
    } else if let Some(integer) = number.as_u64() {
        let _ = write!(out, "{integer}");
    } else {
        return Err(format!(
            "{RECORD_NUMBER_UNSUPPORTED}: the record holds the number {number}; a model-state record holds no floats or integers outside i64/u64, so its hash is not computed."
        ));
    }
    Ok(())
}

fn write_ascii_string(text: &str, out: &mut String) {
    out.push('"');
    for character in text.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            ' '..='~' => out.push(character),
            _ => {
                let mut units = [0u16; 2];
                for unit in character.encode_utf16(&mut units) {
                    let _ = write!(out, "\\u{unit:04x}");
                }
            }
        }
    }
    out.push('"');
}

/// The `model_state_record` hash: SHA-256 hex of the sorted-compact JSON of the
/// record without `hashes` (the stored record already holds no label field).
pub(crate) fn record_hash_hex(stored_record: &Value) -> Result<String, String> {
    let mut payload = stored_record.clone();
    if let Some(map) = payload.as_object_mut() {
        map.remove("hashes");
    }
    Ok(sha256_hex(&sorted_compact_json(&payload)?))
}

// ---------------------------------------------------------------------------
// Time and identity.
// ---------------------------------------------------------------------------

pub(crate) fn now_unix_millis() -> Result<i64, String> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?;
    i64::try_from(duration.as_millis()).map_err(|error| error.to_string())
}

/// UTC ISO 8601 at millisecond precision (`2026-10-24T09:30:00.125Z`), as
/// recorded runs use.
pub(crate) fn utc_iso_millis(unix_millis: i64) -> String {
    let days = unix_millis.div_euclid(86_400_000);
    let of_day = unix_millis.rem_euclid(86_400_000);
    let (year, month, day) = civil_from_days(days);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{:03}Z",
        of_day / 3_600_000,
        (of_day / 60_000) % 60,
        (of_day / 1_000) % 60,
        of_day % 1_000
    )
}

/// Days since 1970-01-01 to the proleptic Gregorian date (H. Hinnant's
/// `civil_from_days`).
fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    (year, month, day)
}

/// 128 random bits as 32 lowercase hex characters (the `state:` token).
pub(crate) fn random_token() -> Result<String, String> {
    let mut bytes = [0u8; 16];
    getrandom::fill(&mut bytes)
        .map_err(|error| format!("MODEL-STATE-TOKEN-UNAVAILABLE: {error}"))?;
    Ok(bytes.iter().map(|byte| format!("{byte:02x}")).collect())
}

/// `model_state.schema.json` `Id`: `^[A-Za-z][A-Za-z0-9_.:-]*$`.
fn is_record_id(text: &str) -> bool {
    let mut characters = text.chars();
    characters
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic())
        && characters.all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | ':' | '-'))
}

// ---------------------------------------------------------------------------
// Labels.
// ---------------------------------------------------------------------------

/// A state's editable labels: its name, tags, notes and state-level
/// assumptions (records in the model's `unresolved_assumptions` vocabulary).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct StateLabels {
    pub(crate) state_name: String,
    pub(crate) tags: Vec<String>,
    pub(crate) notes: Vec<String>,
    pub(crate) assumptions: Vec<Value>,
}

/// What a set of labels names and how its refusals are coded: a state's labels
/// (`state_name`, `MODEL-STATE-LABELS-INVALID`) or a recorded run's (W-12 S2).
#[derive(Debug, Clone, Copy)]
pub(crate) struct LabelVocabulary {
    pub(crate) name_key: &'static str,
    pub(crate) needs_name: &'static str,
    pub(crate) code: &'static str,
}

pub(crate) const STATE_LABEL_VOCABULARY: LabelVocabulary = LabelVocabulary {
    name_key: "state_name",
    needs_name: "a state needs a name",
    code: "MODEL-STATE-LABELS-INVALID",
};

fn labels_invalid(code: &str, detail: impl std::fmt::Display) -> String {
    format!("{code}: {detail}")
}

fn string_list(map: &mut Map<String, Value>, key: &str, code: &str) -> Result<Vec<String>, String> {
    match map.remove(key) {
        None | Some(Value::Null) => Ok(Vec::new()),
        Some(Value::Array(items)) => items
            .into_iter()
            .map(|item| match item {
                Value::String(text) => Ok(text),
                _ => Err(labels_invalid(code, format_args!(
                    "`{key}` holds a value that is not a string"
                ))),
            })
            .collect(),
        Some(_) => Err(labels_invalid(code, format_args!(
            "`{key}` is not a list of strings"
        ))),
    }
}

/// Decodes and checks labels: a non-blank name; unique, trimmed, non-empty tags
/// of at most 64 characters; non-blank notes; assumptions as in the model
/// (`assumption_id` a record id, `kind` assumption/open_issue, a statement,
/// `status` unresolved/resolved/rejected with a `resolution_note` when resolved
/// or rejected, unique `affected_refs` entity ids). Unknown members are refused.
pub(crate) fn labels_from_value(value: Value) -> Result<StateLabels, String> {
    labels_from_value_in(value, STATE_LABEL_VOCABULARY)
}

/// [`labels_from_value`] for a vocabulary: the name is `vocabulary.name_key`
/// (returned in `state_name`), and refusals carry `vocabulary.code`.
pub(crate) fn labels_from_value_in(
    value: Value,
    vocabulary: LabelVocabulary,
) -> Result<StateLabels, String> {
    let code = vocabulary.code;
    let Value::Object(mut map) = value else {
        return Err(labels_invalid(code, "labels must be an object"));
    };
    let state_name = match map.remove(vocabulary.name_key) {
        Some(Value::String(name)) if !name.trim().is_empty() => name.trim().to_string(),
        _ => return Err(labels_invalid(code, vocabulary.needs_name)),
    };
    let tags = string_list(&mut map, "tags", code)?;
    for (index, tag) in tags.iter().enumerate() {
        if tag.trim() != tag || tag.is_empty() || tag.chars().count() > MAX_TAG_CHARS {
            return Err(labels_invalid(code, format_args!(
                "tag \"{tag}\" must be non-empty, trimmed and at most {MAX_TAG_CHARS} characters"
            )));
        }
        if tags[..index].contains(tag) {
            return Err(labels_invalid(code, format_args!(
                "tag \"{tag}\" is listed twice"
            )));
        }
    }
    let notes = string_list(&mut map, "notes", code)?;
    if notes.iter().any(|note| note.trim().is_empty()) {
        return Err(labels_invalid(code, "a note is blank"));
    }
    let assumptions = match map.remove("assumptions") {
        None | Some(Value::Null) => Vec::new(),
        Some(Value::Array(items)) => items
            .into_iter()
            .map(|item| label_assumption(item, code))
            .collect::<Result<Vec<_>, _>>()?,
        Some(_) => return Err(labels_invalid(code, "`assumptions` is not a list")),
    };
    let mut ids: Vec<&str> = Vec::new();
    for assumption in &assumptions {
        let id = assumption["assumption_id"].as_str().unwrap_or("");
        if ids.contains(&id) {
            return Err(labels_invalid(code, format_args!(
                "assumption id `{id}` is listed twice"
            )));
        }
        ids.push(id);
    }
    if let Some(key) = map.keys().next() {
        return Err(labels_invalid(code, format_args!("unknown member `{key}`")));
    }
    // The write-time bound on labels (repair r2): every path that writes labels
    // (Save Model State, Record Run, the label edits) keeps at most
    // MAX_LABEL_ITEMS tags, notes and assumptions together and at most
    // MAX_LABELS_JSON_BYTES of their JSON, as the label edit's ingress budget
    // does. Reads still price what is stored, whatever its size.
    let items = tags.len() + notes.len() + assumptions.len();
    if items > MAX_LABEL_ITEMS {
        return Err(labels_invalid(code, format_args!("{items} tags, notes and assumptions are more than the {MAX_LABEL_ITEMS} kept")));
    }
    let json_bytes = stored_read::total(&[
        state_name.len() as u64,
        stored_read::serialized_length(&tags).unwrap_or(u64::MAX),
        stored_read::serialized_length(&notes).unwrap_or(u64::MAX),
        stored_read::serialized_length(&assumptions).unwrap_or(u64::MAX),
    ]);
    if json_bytes > MAX_LABELS_JSON_BYTES {
        return Err(labels_invalid(code, format_args!("the labels hold {json_bytes} bytes of JSON; at most {MAX_LABELS_JSON_BYTES} are kept")));
    }
    Ok(StateLabels {
        state_name,
        tags,
        notes,
        assumptions,
    })
}

fn label_assumption(value: Value, code: &str) -> Result<Value, String> {
    let Value::Object(map) = value else {
        return Err(labels_invalid(code, "an assumption must be an object"));
    };
    const ALLOWED: [&str; 6] = [
        "assumption_id",
        "kind",
        "statement",
        "status",
        "affected_refs",
        "resolution_note",
    ];
    if let Some(key) = map.keys().find(|key| !ALLOWED.contains(&key.as_str())) {
        return Err(labels_invalid(code, format_args!(
            "unknown assumption member `{key}`"
        )));
    }
    let text = |key: &str| {
        map.get(key)
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|text| !text.is_empty())
    };
    let id = text("assumption_id")
        .ok_or_else(|| labels_invalid(code, "an assumption needs an `assumption_id`"))?;
    if !is_record_id(id) {
        return Err(labels_invalid(code, format_args!(
            "assumption id `{id}` is not a record id ([A-Za-z][A-Za-z0-9_.:-]*)"
        )));
    }
    let kind = text("kind")
        .filter(|kind| ASSUMPTION_KINDS.contains(kind))
        .ok_or_else(|| {
            labels_invalid(code, format_args!(
                "assumption `{id}`: `kind` must be assumption or open_issue"
            ))
        })?;
    let statement = text("statement")
        .ok_or_else(|| labels_invalid(code, format_args!("assumption `{id}` needs a statement")))?;
    let status = text("status")
        .filter(|status| ASSUMPTION_STATUSES.contains(status))
        .ok_or_else(|| {
            labels_invalid(code, format_args!(
                "assumption `{id}`: `status` must be unresolved, resolved or rejected"
            ))
        })?;
    let mut refs: Vec<String> = Vec::new();
    match map.get("affected_refs") {
        None | Some(Value::Null) => {}
        Some(Value::Array(items)) => {
            for item in items {
                let reference = item
                    .as_str()
                    .map(str::trim)
                    .filter(|text| !text.is_empty())
                    .ok_or_else(|| {
                        labels_invalid(code, format_args!(
                            "assumption `{id}`: an affected ref is not an entity id"
                        ))
                    })?;
                if refs.iter().any(|known| known == reference) {
                    return Err(labels_invalid(code, format_args!(
                        "assumption `{id}` names `{reference}` twice"
                    )));
                }
                refs.push(reference.to_string());
            }
        }
        Some(_) => {
            return Err(labels_invalid(code, format_args!(
                "assumption `{id}`: `affected_refs` is not a list"
            )))
        }
    }
    let note = text("resolution_note");
    if map
        .get("resolution_note")
        .is_some_and(|value| !value.is_null())
        && note.is_none()
    {
        return Err(labels_invalid(code, format_args!(
            "assumption `{id}`: the resolution note is blank"
        )));
    }
    if status != "unresolved" && note.is_none() {
        return Err(labels_invalid(code, format_args!(
            "assumption `{id}` is {status}: it needs a resolution note"
        )));
    }
    let mut record = json!({
        "assumption_id": id, "kind": kind, "statement": statement, "status": status, "affected_refs": refs
    });
    if let Some(note) = note {
        record["resolution_note"] = json!(note);
    }
    Ok(record)
}

// ---------------------------------------------------------------------------
// The record (design §2 mapping table).
// ---------------------------------------------------------------------------

fn reference(object_type: &str, reference: &str) -> Value {
    json!({ "object_type": object_type, "ref": reference })
}

/// An entity id as `Reference` holds it: `{Model, "<project id>#<entity id>"}`
/// (the state schema's `Reference.object_type` has no entity types).
fn model_qualified(project_id: &str, entity_id: &str) -> Value {
    reference("Model", &format!("{project_id}#{entity_id}"))
}

fn provenance(
    source_name: &str,
    source_location: &str,
    contributor: &str,
    certification: &str,
) -> Value {
    json!({
        "source_name": source_name,
        "source_location": source_location,
        "source_license": "user-owned local project data",
        "contributor": contributor,
        "contributor_certification": certification,
        "redistribution_status": "private_only",
        "review_status": "pending",
        "privacy_classification": "private_project_data"
    })
}

fn record_provenance() -> Value {
    provenance(
        "SWBPIPE desktop model state store",
        "apps/desktop/src-tauri/src/model_states.rs",
        "SWBPIPE desktop (software-recorded state)",
        "software-recorded-no-professional-claim",
    )
}

fn user_provenance(source_name: &str, source_location: String) -> Value {
    provenance(
        source_name,
        &source_location,
        "local user",
        "user-entered-no-professional-claim",
    )
}

/// One assumption as `AssumptionRecord`: an open issue's statement is prefixed
/// "Open issue: "; entity ids become model-qualified references, and an empty
/// list names `owner` (the project for a model record, the state for a
/// state-level one). `tags` and `notes` are dropped.
fn assumption_record(
    project_id: &str,
    owner: &Value,
    record: &Value,
    provenance: Value,
) -> Result<Value, String> {
    let field = |key: &str| record.get(key).and_then(Value::as_str);
    let id = field("assumption_id").unwrap_or("");
    let unmappable = |detail: &str| {
        format!("MODEL-STATE-ASSUMPTION-UNMAPPABLE: assumption `{id}` {detail}; correct it in the model before saving a state.")
    };
    if !is_record_id(id) {
        return Err(unmappable("has no record id ([A-Za-z][A-Za-z0-9_.:-]*)"));
    }
    let statement = field("statement")
        .filter(|text| !text.trim().is_empty())
        .ok_or_else(|| unmappable("has no statement"))?;
    let status = field("status")
        .filter(|status| ASSUMPTION_STATUSES.contains(status))
        .ok_or_else(|| unmappable("has no valid status"))?;
    let refs = match record.get("affected_refs") {
        None | Some(Value::Null) => Vec::new(),
        Some(Value::Array(items)) => items
            .iter()
            .map(|item| {
                item.as_str()
                    .map(|entity| model_qualified(project_id, entity))
            })
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| unmappable("has an affected ref that is not an entity id"))?,
        Some(_) => return Err(unmappable("has affected_refs that is not a list")),
    };
    let statement = if field("kind") == Some("open_issue") {
        format!("Open issue: {statement}")
    } else {
        statement.to_string()
    };
    let mut mapped = json!({
        "assumption_id": id,
        "statement": statement,
        "status": status,
        "affected_refs": if refs.is_empty() { vec![owner.clone()] } else { refs },
        "provenance": provenance
    });
    if let Some(note) = field("resolution_note") {
        mapped["resolution_note"] = json!(note);
    }
    Ok(mapped)
}

fn model_assumption_records(project_id: &str, model: &Value) -> Result<Vec<Value>, String> {
    let owner = reference("Project", project_id);
    match model.get("unresolved_assumptions") {
        None | Some(Value::Null) => Ok(Vec::new()),
        Some(Value::Array(records)) => records
            .iter()
            .map(|record| {
                let id = record
                    .get("assumption_id")
                    .and_then(Value::as_str)
                    .unwrap_or("");
                assumption_record(
                    project_id,
                    &owner,
                    record,
                    user_provenance(
                        "SWBPIPE model document unresolved_assumptions",
                        format!("model document unresolved_assumptions/{id}"),
                    ),
                )
            })
            .collect(),
        Some(_) => Err(
            "MODEL-STATE-ASSUMPTION-UNMAPPABLE: the model's unresolved_assumptions is not a list."
                .to_string(),
        ),
    }
}

fn diagnostic_class(code: &str) -> &'static str {
    match code {
        "MODEL-ASSUMPTION-SUBJECT-MISSING" => "ASSUMPTION_WARNING",
        "MODEL-PROVENANCE-QUARANTINE" => "PROVENANCE_WARNING",
        _ => "TBD",
    }
}

fn diagnostic_remediation(code: &str) -> &'static str {
    match code {
        "MODEL-ASSUMPTION-SUBJECT-MISSING" => {
            "Update or resolve the record so that it names entities the model still has."
        }
        "MODEL-PROVENANCE-QUARANTINE" => {
            "Review the named provenance or rule-pack disposition before relying on this state."
        }
        _ => "Review the finding in the model document.",
    }
}

/// A document-validator warning as a `Diagnostic` with model-qualified refs.
fn warning_record(project_id: &str, finding: &DocumentDiagnostic) -> Value {
    let mut affected = match finding.affected_refs.first() {
        Some(entity) => model_qualified(project_id, entity),
        None => reference("Model", project_id),
    };
    if !finding.path.is_empty() {
        affected["label"] = json!(finding.path);
    }
    json!({
        "code": finding.code,
        "class": diagnostic_class(finding.code),
        "severity": finding.severity,
        "source": { "object_type": "Model", "ref": project_id, "label": "model document validator" },
        "affected_object": affected,
        "message": finding.message,
        "remediation": diagnostic_remediation(finding.code),
        "provenance": record_provenance()
    })
}

/// What a new state's immutable record is built from.
pub(crate) struct NewState<'a> {
    pub(crate) project_id: &'a str,
    pub(crate) state_id: &'a str,
    pub(crate) state_kind: &'a str,
    pub(crate) created_at: &'a str,
    pub(crate) parent_state_id: Option<&'a str>,
    pub(crate) model: &'a Value,
    /// The snapshot's SHA-256 (bare hex) of its canonical JCS text.
    pub(crate) model_hash_hex: &'a str,
}

/// The stored record: every `ModelStateRecord` member except the label-derived
/// ones (`state_name`, `tags`, `notes` and the state-level assumptions), with
/// `hashes` holding the snapshot's JCS hash and the record hash. Returns the
/// record and its hash.
pub(crate) fn stored_record(state: &NewState<'_>) -> Result<(Value, String), String> {
    let findings = validate_model_document(state.model);
    let mut analysis_status = vec!["HUMAN_REVIEW_REQUIRED"];
    if findings.iter().any(DocumentDiagnostic::is_blocking) {
        analysis_status.push("MODEL_INCOMPLETE");
    }
    let warnings: Vec<Value> = findings
        .iter()
        .filter(|finding| !finding.is_blocking())
        .map(|finding| warning_record(state.project_id, finding))
        .collect();
    let mut record = json!({
        "state_id": state.state_id,
        "state_kind": state.state_kind,
        "created_at": state.created_at,
        "model_ref": reference("Project", state.project_id),
        "parent_state_refs": state.parent_state_id.map(|parent| vec![reference("ModelState", parent)]).unwrap_or_default(),
        "external_references": [],
        "unresolved_assumptions": model_assumption_records(state.project_id, state.model)?,
        "warnings": warnings,
        "analysis_status": analysis_status,
        "immutability_policy": {
            "snapshot_is_read_only": true,
            "mutation_policy": "changes_create_new_model_state",
            "new_state_required_for_change": true,
            "hash_invalidates_external_acceptance": true
        },
        "professional_boundary": {
            "human_review_required": true,
            "software_makes_compliance_claim": false,
            "software_makes_certification_claim": false,
            "software_makes_sealing_claim": false,
            "software_makes_approval_claim": false,
            "software_makes_authentication_claim": false
        },
        "provenance": record_provenance()
    });
    let hash = record_hash_hex(&record)?;
    record["hashes"] = checksum_descriptors(
        state.project_id,
        state.state_id,
        state.model_hash_hex,
        &hash,
    );
    Ok((record, hash))
}

/// A record's `hashes`: exactly these two descriptors, in this order. The
/// record digest leaves `hashes` out, so verification rebuilds this value from
/// the verified fields and requires the stored one to equal it.
fn checksum_descriptors(
    project_id: &str,
    state_id: &str,
    model_hash_hex: &str,
    record_hash_hex: &str,
) -> Value {
    json!([
        {
            "algorithm": "sha256",
            "canonicalization": "JCS",
            "payload_ref": reference("Model", project_id),
            "payload_scope": "model_payload",
            "value": model_hash_hex
        },
        {
            "algorithm": "sha256",
            "canonicalization": "SORTED_COMPACT_JSON",
            "payload_ref": reference("ModelState", state_id),
            "payload_scope": "model_state_record",
            "value": record_hash_hex
        }
    ])
}

/// The record as read: the stored record with the labels joined in. Tags become
/// `StateTag`s, notes `StateNote`s, and state-level assumptions follow the
/// model's in `unresolved_assumptions` (an empty `affected_refs` names the state).
pub(crate) fn emitted_record(stored: &Value, labels: &StateLabels) -> Result<Value, String> {
    let mut record = stored.clone();
    let project_id = stored
        .pointer("/model_ref/ref")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let state_id = stored
        .get("state_id")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let label_source = format!("model state {state_id} labels");
    let tags: Vec<Value> = labels
        .tags
        .iter()
        .map(|tag| json!({ "tag": tag, "tag_kind": "user_label", "provenance": user_provenance("SWBPIPE model state labels", label_source.clone()) }))
        .collect();
    let notes: Vec<Value> = labels
        .notes
        .iter()
        .enumerate()
        .map(|(index, note)| {
            json!({
                "note_id": format!("{state_id}:note:{}", index + 1),
                "note_type": "design_note",
                "statement": note,
                "visibility": "private",
                "provenance": user_provenance("SWBPIPE model state labels", label_source.clone())
            })
        })
        .collect();
    let owner = reference("ModelState", &state_id);
    let mut assumptions = stored
        .get("unresolved_assumptions")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    for assumption in &labels.assumptions {
        assumptions.push(assumption_record(
            &project_id,
            &owner,
            assumption,
            user_provenance("SWBPIPE model state labels", label_source.clone()),
        )?);
    }
    let map = record
        .as_object_mut()
        .ok_or_else(|| "MODEL-STATE-INTEGRITY: the stored record is not an object.".to_string())?;
    map.insert("state_name".to_string(), json!(labels.state_name));
    map.insert("tags".to_string(), Value::Array(tags));
    map.insert("notes".to_string(), Value::Array(notes));
    map.insert(
        "unresolved_assumptions".to_string(),
        Value::Array(assumptions),
    );
    Ok(record)
}

// ---------------------------------------------------------------------------
// Store operations.
// ---------------------------------------------------------------------------

/// One state as the History panel lists it, read from columns only (never the
/// snapshot or the record).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct ModelStateSummary {
    pub(crate) project_id: String,
    pub(crate) state_id: String,
    pub(crate) state_kind: String,
    pub(crate) created_at: String,
    pub(crate) parent_state_id: Option<String>,
    pub(crate) model_hash: String,
    pub(crate) record_hash: String,
    #[serde(flatten)]
    pub(crate) labels: StateLabels,
    pub(crate) labels_updated_at: String,
    /// The saved states whose parent this state is; they block its deletion.
    pub(crate) child_state_ids: Vec<String>,
    /// The recorded runs tied to this state (W-12 S2, oldest first); they block
    /// its deletion too.
    pub(crate) recorded_run_ids: Vec<String>,
    /// The record's `analysis_status` (from its column). `MODEL_INCOMPLETE`: the
    /// snapshot has blocking document findings, so it is kept (a comparison
    /// basis) but not opened as the working model, as a project would not be.
    pub(crate) analysis_status: Vec<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct ModelStateSaveOutcome {
    /// `saved`, or `already_saved` when the project holds a state with this
    /// model hash (nothing is written; that state is named).
    pub(crate) outcome: &'static str,
    pub(crate) state: ModelStateSummary,
    pub(crate) message: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct ModelStateRead {
    pub(crate) state: ModelStateSummary,
    pub(crate) record: Value,
    /// The snapshot, parsed from its stored canonical text.
    pub(crate) model: Value,
    pub(crate) model_document_version: Value,
}

#[derive(Debug, Serialize)]
pub(crate) struct ModelStateDeleteReceipt {
    pub(crate) project_id: String,
    pub(crate) state_id: String,
    pub(crate) deleted: bool,
    pub(crate) message: String,
}

/// The `save_model_state` request. Decoded by [`SaveModelStateRequest::from_value`],
/// which moves the snapshot text out of the parsed body.
#[derive(Debug)]
pub(crate) struct SaveModelStateRequest {
    pub(crate) project_id: String,
    /// The stored envelope hash value the session last opened or saved.
    pub(crate) expected_base: String,
    /// The snapshot's canonical JCS text, as the session hashed it.
    pub(crate) model_canonical: String,
    /// The session's model hash, `sha256:<hex>`.
    pub(crate) model_hash: String,
    pub(crate) state_kind: String,
    pub(crate) parent_state_id: Option<String>,
    pub(crate) labels: StateLabels,
}

impl SaveModelStateRequest {
    pub(crate) fn from_value(value: Value) -> Result<Self, command_ingress::ArgsError> {
        let mut fields = command_ingress::Fields::of(value, "struct SaveModelStateRequest")?;
        let labels = fields.required_value("labels")?;
        Ok(Self {
            project_id: fields.required("project_id")?,
            expected_base: fields.required("expected_base")?,
            model_canonical: fields.required("model_canonical")?,
            model_hash: fields.required("model_hash")?,
            state_kind: fields
                .optional("state_kind")?
                .unwrap_or_else(|| STATE_KINDS[0].to_string()),
            parent_state_id: fields.optional("parent_state_id")?,
            labels: labels_from_value(labels).map_err(serde::de::Error::custom)?,
        })
    }
}

/// How a new state's id and time are allocated, and a hook between the state
/// row and its labels row (tests inject a failure there).
pub(crate) struct StateWriteHooks<'a> {
    pub(crate) allocate: &'a dyn Fn() -> Result<(String, i64), String>,
    pub(crate) between_rows: &'a dyn Fn(&Transaction<'_>) -> Result<(), String>,
}

fn production_allocate() -> Result<(String, i64), String> {
    Ok((random_token()?, now_unix_millis()?))
}

pub(crate) const PRODUCTION_HOOKS: StateWriteHooks<'static> = StateWriteHooks {
    allocate: &production_allocate,
    between_rows: &|_| Ok(()),
};

pub(crate) fn sql(error: rusqlite::Error) -> String {
    error.to_string()
}

fn hash_mismatch(detail: &str) -> String {
    format!("MODEL-STATE-HASH-MISMATCH: {detail}; nothing was saved.")
}

/// Checks a submitted snapshot: it must be its own unchecked `canonical_json`
/// and hash to the claim. Returns the parsed model and the bare hex.
fn verified_snapshot(model_canonical: &str, claimed: &str) -> Result<(Value, String), String> {
    let model: Value = serde_json::from_str(model_canonical)
        .map_err(|error| hash_mismatch(&format!("the snapshot is not JSON ({error})")))?;
    if canonical_json(&model) != model_canonical {
        return Err(hash_mismatch(
            "the snapshot text is not its own canonical JCS text",
        ));
    }
    let hex = sha256_hex(model_canonical);
    if claimed != format!("sha256:{hex}") {
        return Err(hash_mismatch(&format!(
            "the snapshot hashes to sha256:{hex}, not the claimed {claimed}"
        )));
    }
    Ok((model, hex))
}

/// Decodes a labels row. Each stored label list is prescanned first (a list
/// that is not valid JSON is refused before it is parsed), and its parse and
/// the typed lists it becomes are reserved in `memory` before parsing; the
/// returned reservation holds the decoded labels.
fn decode_labels(
    state_id: &str,
    name: String,
    tags: &str,
    notes: &str,
    assumptions: &str,
    memory: &dyn ReadMemory,
) -> Result<(StateLabels, ReadReservation), String> {
    let mut price = 0u64;
    for text in [tags, notes, assumptions] {
        price = price.saturating_add(
            stored_read::parse_bytes(text)
                .map_err(|detail| integrity_of(state_id, format_args!("a stored label list: {detail}")))?
                .saturating_mul(2),
        );
    }
    let reservation = memory.reserve(&format!("saved state `{state_id}`'s labels"), price)?;
    let parse = |text: &str| {
        serde_json::from_str::<Value>(text).map_err(|error| {
            format!("MODEL-STATE-INTEGRITY: a stored label is not JSON ({error}).")
        })
    };
    let strings = |value: Value| -> Result<Vec<String>, String> {
        serde_json::from_value(value).map_err(|error| {
            format!("MODEL-STATE-INTEGRITY: a stored label list is malformed ({error}).")
        })
    };
    let labels = StateLabels {
        state_name: name,
        tags: strings(parse(tags)?)?,
        notes: strings(parse(notes)?)?,
        assumptions: serde_json::from_value(parse(assumptions)?).map_err(|error| {
            format!("MODEL-STATE-INTEGRITY: the stored assumptions are malformed ({error}).")
        })?,
    };
    Ok((labels, reservation))
}

/// The byte lengths a state summary loads, read with `octet_length` and counts
/// before any of it is loaded: the record, the labels row, the child state ids,
/// and the recorded runs' ids and names.
const SUMMARY_LENGTHS: &str = "octet_length(s.record_json) AS record_length,
    COALESCE(octet_length(l.state_name) + octet_length(l.tags_json) + octet_length(l.notes_json) + octet_length(l.assumptions_json) + octet_length(l.updated_at), 0) AS labels_length,
    (SELECT COUNT(*) FROM model_states c WHERE c.project_id = s.project_id AND c.parent_state_id = s.state_id) AS child_count,
    (SELECT COALESCE(SUM(octet_length(c.state_id)), 0) FROM model_states c WHERE c.project_id = s.project_id AND c.parent_state_id = s.state_id) AS child_bytes,
    (SELECT COUNT(*) FROM analysis_runs r WHERE r.project_id = s.project_id AND r.state_id = s.state_id) AS run_count,
    (SELECT COALESCE(SUM(octet_length(r.recording_id) + COALESCE(octet_length(rl.run_name), 0)), 0) FROM analysis_runs r
       LEFT JOIN analysis_run_labels rl ON rl.project_id = r.project_id AND rl.recording_id = r.recording_id
       WHERE r.project_id = s.project_id AND r.state_id = s.state_id) AS run_bytes";

/// What one or more state summaries load (for a list: the largest record, as
/// records are verified one at a time, and the sums of the rest).
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct SummaryLengths {
    pub(crate) record: u64,
    labels: u64,
    child_count: u64,
    child_bytes: u64,
    run_count: u64,
    run_bytes: u64,
    rows: u64,
}

fn lengths_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<SummaryLengths> {
    let length = |index: usize| -> rusqlite::Result<u64> {
        Ok(u64::try_from(row.get::<_, Option<i64>>(index)?.unwrap_or(0)).unwrap_or(0))
    };
    Ok(SummaryLengths {
        record: length(0)?,
        labels: length(1)?,
        child_count: length(2)?,
        child_bytes: length(3)?,
        run_count: length(4)?,
        run_bytes: length(5)?,
        rows: length(6)?,
    })
}

/// The summary lengths of one state, or `None` when it is not saved.
fn state_summary_lengths(connection: &Connection, project_id: &str, state_id: &str) -> Result<Option<SummaryLengths>, String> {
    connection
        .query_row(
            &format!(
                "SELECT {SUMMARY_LENGTHS}, 1 FROM model_states s LEFT JOIN model_state_labels l
                 ON l.project_id = s.project_id AND l.state_id = s.state_id WHERE s.project_id = ?1 AND s.state_id = ?2"
            ),
            params![project_id, state_id],
            lengths_row,
        )
        .optional()
        .map_err(sql)
}

/// What loading summaries of these lengths needs before any is loaded: the
/// largest record's loading and sorted-compact hash text, every labels row,
/// the id and name lists, and each row's small columns.
pub(crate) fn summary_load_bytes(lengths: &SummaryLengths) -> u64 {
    stored_read::total(&[
        stored_read::loaded_text_bytes(lengths.record),
        stored_read::text_copy_bytes(lengths.record),
        stored_read::loaded_text_bytes(lengths.labels),
        stored_read::string_list_bytes(lengths.child_count, lengths.child_bytes),
        stored_read::string_list_bytes(lengths.run_count, lengths.run_bytes),
        lengths.rows.saturating_mul(4096),
    ])
}

const SUMMARY_COLUMNS: &str = "s.project_id, s.state_id, s.state_kind, s.created_at, s.parent_state_id, s.model_hash, s.record_hash,
    l.state_name, l.tags_json, l.notes_json, l.assumptions_json, l.updated_at";

type SummaryRow = (
    String,
    String,
    String,
    String,
    Option<String>,
    String,
    String,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
);

fn summary_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<SummaryRow> {
    Ok((
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
        row.get(4)?,
        row.get(5)?,
        row.get(6)?,
        row.get(7)?,
        row.get(8)?,
        row.get(9)?,
        row.get(10)?,
        row.get(11)?,
    ))
}

fn children(
    connection: &Connection,
    project_id: &str,
    state_id: &str,
) -> Result<Vec<String>, String> {
    let mut statement = connection
        .prepare("SELECT state_id FROM model_states WHERE project_id = ?1 AND parent_state_id = ?2 ORDER BY created_at, state_id")
        .map_err(sql)?;
    let ids = statement
        .query_map(params![project_id, state_id], |row| row.get::<_, String>(0))
        .map_err(sql)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(sql)?;
    Ok(ids)
}

/// The recorded runs tied to a state, oldest first: `(recording id, run name)`.
pub(crate) fn recorded_runs_of(
    connection: &Connection,
    project_id: &str,
    state_id: &str,
) -> Result<Vec<(String, Option<String>)>, String> {
    let mut statement = connection
        .prepare(
            "SELECT r.recording_id, l.run_name FROM analysis_runs r LEFT JOIN analysis_run_labels l
             ON l.project_id = r.project_id AND l.recording_id = r.recording_id
             WHERE r.project_id = ?1 AND r.state_id = ?2 ORDER BY r.created_at, r.recording_id",
        )
        .map_err(sql)?;
    let runs = statement
        .query_map(params![project_id, state_id], |row| Ok((row.get(0)?, row.get(1)?)))
        .map_err(sql)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(sql)?;
    Ok(runs)
}

/// The value of the stored record's `hashes` entry for `scope`.
fn claimed_hash<'a>(record: &'a Value, scope: &str) -> Option<&'a str> {
    record
        .get("hashes")
        .and_then(Value::as_array)
        .and_then(|hashes| {
            hashes
                .iter()
                .find(|hash| hash.get("payload_scope").and_then(Value::as_str) == Some(scope))
                .and_then(|hash| hash.get("value"))
                .and_then(Value::as_str)
        })
}

fn integrity_of(state_id: &str, detail: impl std::fmt::Display) -> String {
    format!("MODEL-STATE-INTEGRITY: saved state `{state_id}`: {detail}; it is neither listed nor opened.")
}

/// The row's identity columns, which the record duplicates.
struct IdentityColumns<'a> {
    project_id: &'a str,
    state_id: &'a str,
    state_kind: &'a str,
    created_at: &'a str,
    parent_state_id: Option<&'a str>,
    model_hash: &'a str,
    record_hash: &'a str,
}

/// Parses the stored record, checks its hash against the `record_hash` column
/// and its own `hashes`, and checks every identity field the record duplicates
/// (project, state id, kind, `created_at`, parent ref, model hash) against the
/// row's columns. Any difference is `MODEL-STATE-INTEGRITY`. Used wherever a
/// state is listed, summarized or read, so a column edited outside the store
/// is never reported as the state's.
fn verified_record(columns: &IdentityColumns<'_>, record_json: &str) -> Result<Value, String> {
    let id = columns.state_id;
    let record: Value = serde_json::from_str(record_json).map_err(|error| {
        integrity_of(id, format_args!("the stored record is not JSON ({error})"))
    })?;
    let recomputed = record_hash_hex(&record).map_err(|error| integrity_of(id, error))?;
    if recomputed != columns.record_hash
        || claimed_hash(&record, "model_state_record") != Some(recomputed.as_str())
    {
        return Err(integrity_of(
            id,
            format_args!("the stored record hashes to {recomputed}, not its recorded hash"),
        ));
    }
    let field = |pointer: &str| record.pointer(pointer).and_then(Value::as_str);
    let parent_refs = match columns.parent_state_id {
        Some(parent) => json!([reference("ModelState", parent)]),
        None => json!([]),
    };
    let checks: [(&str, bool); 6] = [
        ("state_id", field("/state_id") == Some(columns.state_id)),
        (
            "project",
            record.get("model_ref") == Some(&reference("Project", columns.project_id)),
        ),
        (
            "state_kind",
            field("/state_kind") == Some(columns.state_kind),
        ),
        (
            "created_at",
            field("/created_at") == Some(columns.created_at),
        ),
        (
            "parent ref",
            record.get("parent_state_refs") == Some(&parent_refs),
        ),
        (
            "model hash",
            columns
                .model_hash
                .strip_prefix("sha256:")
                .is_some_and(|hex| claimed_hash(&record, "model_payload") == Some(hex)),
        ),
    ];
    if let Some((name, _)) = checks.iter().find(|(_, holds)| !holds) {
        return Err(integrity_of(
            id,
            format_args!("the row's {name} differs from its verified record"),
        ));
    }
    // The digest does not cover `hashes`: the whole array must be exactly the
    // descriptors the verified fields give (algorithm, canonicalization, scope,
    // payload ref, value), with no duplicate, extra or missing entry.
    let model_hash_hex = columns.model_hash.strip_prefix("sha256:").unwrap_or("");
    let expected = checksum_descriptors(
        columns.project_id,
        columns.state_id,
        model_hash_hex,
        &recomputed,
    );
    if record.get("hashes") != Some(&expected) {
        return Err(integrity_of(
            id,
            "the record's checksum descriptors differ from the expected ones",
        ));
    }
    Ok(record)
}

fn stored_record_json(
    connection: &Connection,
    project_id: &str,
    state_id: &str,
) -> Result<String, String> {
    connection
        .query_row(
            "SELECT record_json FROM model_states WHERE project_id = ?1 AND state_id = ?2",
            params![project_id, state_id],
            |row| row.get(0),
        )
        .map_err(sql)
}

/// One summary from its row. The caller has reserved loading it
/// ([`summary_load_bytes`]); the record's parse is reserved here before parsing
/// and released once it is verified; the returned reservation holds the decoded
/// labels.
fn summary_from_row(
    connection: &Connection,
    row: SummaryRow,
    memory: &dyn ReadMemory,
) -> Result<(ModelStateSummary, ReadReservation), String> {
    let (
        project_id,
        state_id,
        state_kind,
        created_at,
        parent_state_id,
        model_hash,
        record_hash,
        name,
        tags,
        notes,
        assumptions,
        updated_at,
    ) = row;
    let (Some(name), Some(tags), Some(notes), Some(assumptions), Some(updated_at)) =
        (name, tags, notes, assumptions, updated_at)
    else {
        return Err(format!(
            "MODEL-STATE-INTEGRITY: state `{state_id}` has no labels row."
        ));
    };
    let record_json = stored_record_json(connection, &project_id, &state_id)?;
    // The record's parse and the projection its hash clones (priced before parsing).
    let record_parse = stored_read::parse_bytes(&record_json)
        .map_err(|detail| integrity_of(&state_id, format_args!("the stored record: {detail}")))?;
    let parse = memory.reserve(&format!("saved state `{state_id}`'s record"), record_parse.saturating_mul(2))?;
    let record = verified_record(
        &IdentityColumns {
            project_id: &project_id,
            state_id: &state_id,
            state_kind: &state_kind,
            created_at: &created_at,
            parent_state_id: parent_state_id.as_deref(),
            model_hash: &model_hash,
            record_hash: &record_hash,
        },
        &record_json,
    )?;
    drop(record_json);
    let analysis_status: Vec<String> =
        serde_json::from_value(record.get("analysis_status").cloned().unwrap_or(json!([])))
            .map_err(|error| {
                integrity_of(
                    &state_id,
                    format_args!("its analysis status is malformed ({error})"),
                )
            })?;
    drop(record);
    drop(parse);
    let child_state_ids = children(connection, &project_id, &state_id)?;
    let recorded_run_ids = recorded_runs_of(connection, &project_id, &state_id)?
        .into_iter()
        .map(|(recording_id, _)| recording_id)
        .collect();
    let (labels, labels_reservation) = decode_labels(&state_id, name, &tags, &notes, &assumptions, memory)?;
    drop((tags, notes, assumptions));
    Ok((ModelStateSummary {
        analysis_status,
        labels,
        project_id,
        state_id,
        state_kind,
        created_at,
        parent_state_id,
        model_hash,
        record_hash,
        labels_updated_at: updated_at,
        child_state_ids,
        recorded_run_ids,
    }, labels_reservation))
}

pub(crate) fn find_summary(
    connection: &Connection,
    project_id: &str,
    state_id: &str,
) -> Result<Option<ModelStateSummary>, String> {
    Ok(find_summary_with(connection, project_id, state_id, &UnmeteredReadMemory)?.map(|metered| metered.into_parts().0))
}

/// [`find_summary`] within `memory`: the summary's lengths are read first, its
/// loading is reserved before anything of it is loaded, and the returned
/// reservations hold the summary.
pub(crate) fn find_summary_with(
    connection: &Connection,
    project_id: &str,
    state_id: &str,
    memory: &dyn ReadMemory,
) -> Result<Option<Metered<ModelStateSummary>>, String> {
    let Some(lengths) = state_summary_lengths(connection, project_id, state_id)? else {
        return Ok(None);
    };
    let load = memory.reserve(&format!("saved state `{state_id}`"), summary_load_bytes(&lengths))?;
    let row = connection
        .query_row(
            &format!(
                "SELECT {SUMMARY_COLUMNS} FROM model_states s LEFT JOIN model_state_labels l
                 ON l.project_id = s.project_id AND l.state_id = s.state_id WHERE s.project_id = ?1 AND s.state_id = ?2"
            ),
            params![project_id, state_id],
            summary_row,
        )
        .optional()
        .map_err(sql)?;
    let Some(row) = row else { return Ok(None) };
    let (summary, labels) = summary_from_row(connection, row, memory)?;
    Ok(Some(Metered::new(summary).holding(load).holding(labels)))
}

/// The project's states, newest first, from columns only (unmetered; the
/// command uses [`list_states_with`]).
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn list_states(
    connection: &Connection,
    project_id: &str,
) -> Result<Vec<ModelStateSummary>, String> {
    Ok(list_states_with(connection, project_id, &UnmeteredReadMemory)?.into_parts().0)
}

/// [`list_states`] within `memory`: before anything is loaded, the summaries'
/// lengths are read and their loading reserved (the largest record, as records
/// are verified one at a time, and every labels row and id list, which the
/// list keeps); each record's parse is reserved after it is loaded and released
/// once it is verified, and each row's decoded labels are held with the list.
pub(crate) fn list_states_with(
    connection: &Connection,
    project_id: &str,
    memory: &dyn ReadMemory,
) -> Result<Metered<Vec<ModelStateSummary>>, String> {
    let lengths = connection
        .query_row(
            &format!(
                "SELECT MAX(record_length), SUM(labels_length), SUM(child_count), SUM(child_bytes), SUM(run_count), SUM(run_bytes), COUNT(*)
                 FROM (SELECT {SUMMARY_LENGTHS}
                 FROM model_states s LEFT JOIN model_state_labels l ON l.project_id = s.project_id AND l.state_id = s.state_id
                 WHERE s.project_id = ?1)"
            ),
            params![project_id],
            lengths_row,
        )
        .map_err(sql)?;
    let load = memory.reserve(
        &format!("the saved states of project `{project_id}`"),
        stored_read::total(&[stored_read::READ_OVERHEAD_BYTES, summary_load_bytes(&lengths)]),
    )?;
    let mut statement = connection
        .prepare(&format!(
            "SELECT {SUMMARY_COLUMNS} FROM model_states s LEFT JOIN model_state_labels l
             ON l.project_id = s.project_id AND l.state_id = s.state_id WHERE s.project_id = ?1
             ORDER BY s.created_at DESC, s.state_id"
        ))
        .map_err(sql)?;
    let rows = statement
        .query_map(params![project_id], summary_row)
        .map_err(sql)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(sql)?;
    let mut listed = Metered::new(Vec::with_capacity(rows.len())).holding(load);
    for row in rows {
        let (summary, labels) = summary_from_row(connection, row, memory)?;
        listed.value.push(summary);
        listed.hold(labels);
    }
    Ok(listed)
}

/// How many saved states the project id holds (the replacement prompt's count).
pub(crate) fn saved_state_count(connection: &Connection, project_id: &str) -> Result<i64, String> {
    connection
        .query_row(
            "SELECT COUNT(*) FROM model_states WHERE project_id = ?1",
            params![project_id],
            |row| row.get(0),
        )
        .map_err(sql)
}

pub(crate) fn state_description(summary: &ModelStateSummary) -> String {
    format!("\"{}\" ({})", summary.labels.state_name, summary.state_id)
}

/// Model assumption ids a label assumption may not reuse.
fn refuse_assumption_id_collisions(stored: &Value, labels: &StateLabels) -> Result<(), String> {
    let model_ids: Vec<&str> = stored
        .get("unresolved_assumptions")
        .and_then(Value::as_array)
        .map(|records| {
            records
                .iter()
                .filter_map(|record| record.get("assumption_id").and_then(Value::as_str))
                .collect()
        })
        .unwrap_or_default();
    for assumption in &labels.assumptions {
        let id = assumption["assumption_id"].as_str().unwrap_or("");
        if model_ids.contains(&id) {
            return Err(labels_invalid(
                STATE_LABEL_VOCABULARY.code,
                format_args!("assumption id `{id}` is already a model assumption of this state"
            )));
        }
    }
    Ok(())
}

/// A new state's inputs, checked by [`checked_new_state`]: the snapshot is its
/// own canonical JCS text and hashes to the claim, it is its project's, and its
/// document version is current.
pub(crate) struct CheckedSnapshot {
    pub(crate) model: Value,
    pub(crate) model_canonical: String,
    /// `sha256:<hex>`.
    pub(crate) model_hash: String,
    pub(crate) model_hash_hex: String,
}

/// The snapshot checks of a state save (before any store access).
pub(crate) fn checked_new_state(
    project_id: &str,
    state_kind: &str,
    model_canonical: String,
    claimed_hash: &str,
) -> Result<CheckedSnapshot, String> {
    if !STATE_KINDS.contains(&state_kind) {
        return Err(format!(
            "MODEL-STATE-KIND-INVALID: `{state_kind}` is not one of {}.",
            STATE_KINDS.join(", ")
        ));
    }
    let (model, model_hash_hex) = verified_snapshot(&model_canonical, claimed_hash)?;
    let snapshot_project = crate::project_id_from_model(&model);
    if snapshot_project != project_id {
        return Err(format!(
            "MODEL-STATE-PROJECT-MISMATCH: the snapshot is project `{snapshot_project}`, not `{project_id}`; nothing was saved."
        ));
    }
    // One current model-document version. Unlike a project save, a state may
    // hold a model with blocking slot findings: the record says
    // `MODEL_INCOMPLETE` and the state is not opened (design §2). The record
    // mapping refuses, by name, any slot it cannot map.
    crate::check_model_document_version(&model)?;
    Ok(CheckedSnapshot {
        model,
        model_canonical,
        model_hash: claimed_hash.to_string(),
        model_hash_hex,
    })
}

/// The state of `project_id` holding this model hash, if any.
pub(crate) fn state_with_model_hash(
    connection: &Connection,
    project_id: &str,
    model_hash: &str,
) -> Result<Option<ModelStateSummary>, String> {
    let existing: Option<String> = connection
        .query_row(
            "SELECT state_id FROM model_states WHERE project_id = ?1 AND model_hash = ?2",
            params![project_id, model_hash],
            |row| row.get(0),
        )
        .optional()
        .map_err(sql)?;
    match existing {
        None => Ok(None),
        Some(state_id) => find_summary(connection, project_id, &state_id)?
            .map(Some)
            .ok_or_else(|| format!("MODEL-STATE-INTEGRITY: state `{state_id}` vanished.")),
    }
}

/// Writes a new state row and its labels row on the caller's `BEGIN IMMEDIATE`
/// transaction (the caller has checked the project row and its base, and that
/// no state holds this model hash). Shared by Save Model State and by Record
/// Run's post-solve snapshot (W-12 S2), which writes the state, its labels, the
/// run and the run's labels in one transaction.
pub(crate) fn insert_new_state(
    transaction: &Transaction<'_>,
    project_id: &str,
    state_kind: &str,
    parent_state_id: Option<&str>,
    snapshot: &CheckedSnapshot,
    labels: &StateLabels,
    hooks: &StateWriteHooks<'_>,
) -> Result<ModelStateSummary, String> {
    if let Some(parent) = parent_state_id {
        if find_summary(transaction, project_id, parent)?.is_none() {
            return Err(format!(
                "MODEL-STATE-PARENT-MISSING: project `{project_id}` has no saved state `{parent}`; nothing was saved."
            ));
        }
    }
    let (token, now) = (hooks.allocate)()?;
    let state_id = format!("state:{token}");
    let created_at = utc_iso_millis(now);
    let (record, record_hash) = stored_record(&NewState {
        project_id,
        state_id: &state_id,
        state_kind,
        created_at: &created_at,
        parent_state_id,
        model: &snapshot.model,
        model_hash_hex: &snapshot.model_hash_hex,
    })?;
    refuse_assumption_id_collisions(&record, labels)?;
    let record_json = serde_json::to_string(&record).map_err(|error| error.to_string())?;
    transaction
        .execute(
            "INSERT INTO model_states (project_id, state_id, state_kind, created_at, parent_state_id, model_canonical, model_hash, record_json, record_hash)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                project_id,
                state_id,
                state_kind,
                created_at,
                parent_state_id,
                snapshot.model_canonical,
                snapshot.model_hash,
                record_json,
                record_hash
            ],
        )
        .map_err(sql)?;
    (hooks.between_rows)(transaction)?;
    write_labels(transaction, project_id, &state_id, labels, &created_at, true)?;
    find_summary(transaction, project_id, &state_id)?
        .ok_or_else(|| format!("MODEL-STATE-INTEGRITY: state `{state_id}` was not written."))
}

/// `save_model_state` on a given store: the checks, then the state row and its
/// labels row in one `BEGIN IMMEDIATE` transaction with the project row's
/// stale-window check.
pub(crate) fn save_state_in_store(
    path: &Path,
    request: SaveModelStateRequest,
    hooks: &StateWriteHooks<'_>,
) -> Result<ModelStateSaveOutcome, String> {
    let snapshot = checked_new_state(
        &request.project_id,
        &request.state_kind,
        request.model_canonical,
        &request.model_hash,
    )?;
    let (mut connection, _migration) = crate::open_project_store(path)?;
    let transaction = crate::immediate_transaction(&mut connection)?;
    if crate::stored_project_entry(&transaction, &request.project_id)?.is_none() {
        return Err(format!(
            "MODEL-STATE-PROJECT-MISSING: project `{}` has no stored local project; save the local project first.",
            request.project_id
        ));
    }
    crate::refuse_unconfirmed_replacement(
        &transaction,
        &request.project_id,
        None,
        Some(&request.expected_base),
    )?;
    if let Some(state) = state_with_model_hash(&transaction, &request.project_id, &snapshot.model_hash)? {
        let message = format!(
            "This model is already saved as state {}; nothing was written.",
            state_description(&state)
        );
        return Ok(ModelStateSaveOutcome {
            outcome: "already_saved",
            state,
            message,
        });
    }
    let state = insert_new_state(
        &transaction,
        &request.project_id,
        &request.state_kind,
        request.parent_state_id.as_deref(),
        &snapshot,
        &request.labels,
        hooks,
    )?;
    transaction.commit().map_err(sql)?;
    let message = format!(
        "Saved model state {}. Renaming, tagging or adding notes does not change the saved state.",
        state_description(&state)
    );
    Ok(ModelStateSaveOutcome {
        outcome: "saved",
        state,
        message,
    })
}

fn write_labels(
    transaction: &Transaction<'_>,
    project_id: &str,
    state_id: &str,
    labels: &StateLabels,
    updated_at: &str,
    insert: bool,
) -> Result<(), String> {
    let text = |result: serde_json::Result<String>| result.map_err(|error| error.to_string());
    let tags = text(serde_json::to_string(&labels.tags))?;
    let notes = text(serde_json::to_string(&labels.notes))?;
    let assumptions = text(serde_json::to_string(&labels.assumptions))?;
    let statement = if insert {
        "INSERT INTO model_state_labels (project_id, state_id, state_name, tags_json, notes_json, assumptions_json, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"
    } else {
        "UPDATE model_state_labels SET state_name = ?3, tags_json = ?4, notes_json = ?5, assumptions_json = ?6, updated_at = ?7
         WHERE project_id = ?1 AND state_id = ?2"
    };
    let changed = transaction
        .execute(
            statement,
            params![
                project_id,
                state_id,
                labels.state_name,
                tags,
                notes,
                assumptions,
                updated_at
            ],
        )
        .map_err(sql)?;
    if changed != 1 {
        return Err(format!(
            "MODEL-STATE-INTEGRITY: the labels of state `{state_id}` were not written."
        ));
    }
    Ok(())
}

fn integrity(detail: String) -> String {
    format!("MODEL-STATE-INTEGRITY: {detail}; the state is not opened.")
}

/// `read_model_state` on a given store: the state's emitted record (labels
/// joined in) and its snapshot, after the stored bytes are checked against
/// both stored hashes. A snapshot whose version is not current, or whose
/// model-data slots are malformed, is refused by name (as an opened project is).
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn read_state_in_store(
    path: &Path,
    project_id: &str,
    state_id: &str,
) -> Result<ModelStateRead, String> {
    Ok(read_state_in_store_with(path, project_id, state_id, &UnmeteredReadMemory)?.into_parts().0)
}

/// [`read_state_in_store`] within `memory` (`stored_read`): the snapshot's and
/// the record's byte lengths are read first, without loading them, and their
/// loading and hash copies are reserved before either is loaded (the summary
/// reserves its own); their parses are reserved before they are parsed, and a
/// stored text the prescan refuses is refused before it is parsed. The
/// returned reservations hold the read until its response is serialized
/// ([`stored_read::respond_metered`]). A refusal is
/// `STORE-READ-MEMORY-UNAVAILABLE` and loads nothing more.
pub(crate) fn read_state_in_store_with(
    path: &Path,
    project_id: &str,
    state_id: &str,
    memory: &dyn ReadMemory,
) -> Result<Metered<ModelStateRead>, String> {
    let (connection, _migration) = crate::open_project_store(path)?;
    let not_found = || format!("MODEL-STATE-NOT-FOUND: project `{project_id}` has no saved state `{state_id}`.");
    let (model_length, record_length): (i64, i64) = connection
        .query_row(
            "SELECT octet_length(model_canonical), octet_length(record_json) FROM model_states WHERE project_id = ?1 AND state_id = ?2",
            params![project_id, state_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()
        .map_err(sql)?
        .ok_or_else(not_found)?;
    let (model_length, record_length) = (
        u64::try_from(model_length).unwrap_or(0),
        u64::try_from(record_length).unwrap_or(0),
    );
    let what = format!("saved state `{state_id}`");
    // Loaded here: the snapshot and the record (the summary reserves its own
    // load). Copies: the snapshot's canonical text; the record's sorted-compact text.
    let load = memory.reserve(
        &what,
        stored_read::total(&[
            stored_read::READ_OVERHEAD_BYTES,
            stored_read::loaded_text_bytes(model_length),
            stored_read::text_copy_bytes(model_length),
            stored_read::loaded_text_bytes(record_length),
            stored_read::text_copy_bytes(record_length),
        ]),
    )?;
    let mut read = Metered::new(()).holding(load);
    let state = read.absorb(find_summary_with(&connection, project_id, state_id, memory)?.ok_or_else(not_found)?);
    let (model_canonical, record_json): (String, String) = connection
        .query_row(
            "SELECT model_canonical, record_json FROM model_states WHERE project_id = ?1 AND state_id = ?2",
            params![project_id, state_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(sql)?;
    // Parsed: the snapshot, and the validators' indexes of it; the record, its
    // hash projection and the emitted record, whose label items each gain a
    // provenance object.
    let model_parse = stored_read::parse_bytes(&model_canonical)
        .map_err(|detail| integrity(format!("the stored snapshot: {detail}")))?;
    let record_parse = stored_read::parse_bytes(&record_json)
        .map_err(|detail| integrity_of(state_id, format_args!("the stored record: {detail}")))?;
    let label_items = (state.labels.tags.len() + state.labels.notes.len() + state.labels.assumptions.len()) as u64;
    read.hold(memory.reserve(
        &what,
        stored_read::total(&[
            model_parse.saturating_mul(2),
            record_parse.saturating_mul(3),
            label_items.saturating_mul(8 * 1024),
        ]),
    )?);
    let hex = sha256_hex(&model_canonical);
    if state.model_hash != format!("sha256:{hex}") {
        return Err(integrity(format!(
            "the stored snapshot hashes to sha256:{hex}, not the stored {}",
            state.model_hash
        )));
    }
    let model: Value = serde_json::from_str(&model_canonical)
        .map_err(|error| integrity(format!("the stored snapshot is not JSON ({error})")))?;
    if canonical_json(&model) != model_canonical {
        return Err(integrity(
            "the stored snapshot is not canonical JCS text".to_string(),
        ));
    }
    // The summary above verified the record against the row's columns; the
    // record read here is checked again, then against the snapshot.
    let stored = verified_record(
        &IdentityColumns {
            project_id: &state.project_id,
            state_id: &state.state_id,
            state_kind: &state.state_kind,
            created_at: &state.created_at,
            parent_state_id: state.parent_state_id.as_deref(),
            model_hash: &state.model_hash,
            record_hash: &state.record_hash,
        },
        &record_json,
    )?;
    if claimed_hash(&stored, "model_payload") != Some(hex.as_str()) {
        return Err(integrity(
            "the record's model_payload hash differs from the snapshot".to_string(),
        ));
    }
    let snapshot_project = crate::project_id_from_model(&model);
    if snapshot_project != state.project_id {
        return Err(integrity(format!(
            "the snapshot is project `{snapshot_project}`, not the state's project `{}`",
            state.project_id
        )));
    }
    let version = crate::model_document_version::evaluate_model_document(&model);
    if version.is_refused() {
        return Err(crate::refuse_model_document(&version));
    }
    crate::refuse_malformed_model_slots(&model)?;
    let record = emitted_record(&stored, &state.labels)?;
    drop(stored);
    drop(record_json);
    drop(model_canonical);
    Ok(read.map(|()| ModelStateRead {
        state,
        record,
        model,
        model_document_version: serde_json::to_value(version).unwrap_or(Value::Null),
    }))
}

/// `edit_model_state_labels` on a given store. Only the labels row changes, so
/// the record hash and the snapshot are untouched. `expected_updated_at`, when
/// given, must be the labels' current `updated_at` (`MODEL-STATE-LABELS-MOVED`
/// otherwise: another window edited them).
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn edit_labels_in_store(
    path: &Path,
    project_id: &str,
    state_id: &str,
    labels: StateLabels,
    expected_updated_at: Option<&str>,
    now_millis: i64,
) -> Result<ModelStateSummary, String> {
    Ok(edit_labels_in_store_with(path, project_id, state_id, labels, expected_updated_at, now_millis, &UnmeteredReadMemory)?
        .into_parts()
        .0)
}

/// [`edit_labels_in_store`] within `memory`: the summaries it reads and the
/// record it checks the labels against are reserved before they are loaded,
/// and their parses before they are parsed.
pub(crate) fn edit_labels_in_store_with(
    path: &Path,
    project_id: &str,
    state_id: &str,
    labels: StateLabels,
    expected_updated_at: Option<&str>,
    now_millis: i64,
    memory: &dyn ReadMemory,
) -> Result<Metered<ModelStateSummary>, String> {
    let (mut connection, _migration) = crate::open_project_store(path)?;
    let transaction = crate::immediate_transaction(&mut connection)?;
    let not_found = || format!("MODEL-STATE-NOT-FOUND: project `{project_id}` has no saved state `{state_id}`.");
    let current = find_summary_with(&transaction, project_id, state_id, memory)?.ok_or_else(not_found)?.into_parts().0;
    if let Some(expected) = expected_updated_at {
        if expected != current.labels_updated_at {
            return Err(format!(
                "MODEL-STATE-LABELS-MOVED: the labels of state {} changed after this window read them; nothing was written.",
                state_description(&current)
            ));
        }
    }
    let record_length: i64 = transaction
        .query_row(
            "SELECT octet_length(record_json) FROM model_states WHERE project_id = ?1 AND state_id = ?2",
            params![project_id, state_id],
            |row| row.get(0),
        )
        .map_err(sql)?;
    let load = memory.reserve(
        &format!("saved state `{state_id}`'s record"),
        stored_read::loaded_text_bytes(u64::try_from(record_length).unwrap_or(0)),
    )?;
    let record_json: String = transaction
        .query_row(
            "SELECT record_json FROM model_states WHERE project_id = ?1 AND state_id = ?2",
            params![project_id, state_id],
            |row| row.get(0),
        )
        .map_err(sql)?;
    let parse = memory.reserve(
        &format!("saved state `{state_id}`'s record"),
        stored_read::parse_bytes(&record_json).map_err(|detail| integrity_of(state_id, format_args!("the stored record: {detail}")))?,
    )?;
    let stored: Value = serde_json::from_str(&record_json).map_err(|error| error.to_string())?;
    refuse_assumption_id_collisions(&stored, &labels)?;
    drop((stored, record_json, parse, load));
    // Strictly later than the labels' last write, so `expected_updated_at`
    // tells two writes apart.
    let previous = parse_iso_millis(&current.labels_updated_at).unwrap_or(i64::MIN);
    let updated_at = utc_iso_millis(now_millis.max(previous.saturating_add(1)));
    write_labels(
        &transaction,
        project_id,
        state_id,
        &labels,
        &updated_at,
        false,
    )?;
    let summary = find_summary_with(&transaction, project_id, state_id, memory)?
        .ok_or_else(|| format!("MODEL-STATE-INTEGRITY: state `{state_id}` vanished."))?;
    transaction.commit().map_err(sql)?;
    Ok(summary)
}

/// The inverse of [`utc_iso_millis`] for the text it writes.
pub(crate) fn parse_iso_millis(text: &str) -> Option<i64> {
    let bytes = text.as_bytes();
    if bytes.len() != 24 || &text[23..] != "Z" {
        return None;
    }
    let number = |range: std::ops::Range<usize>| text.get(range)?.parse::<i64>().ok();
    let (year, month, day) = (number(0..4)?, number(5..7)?, number(8..10)?);
    let (hour, minute, second, milli) = (
        number(11..13)?,
        number(14..16)?,
        number(17..19)?,
        number(20..23)?,
    );
    // H. Hinnant's `days_from_civil`.
    let y = if month <= 2 { year - 1 } else { year };
    let era = y.div_euclid(400);
    let yoe = y.rem_euclid(400);
    let mp = if month > 2 { month - 3 } else { month + 9 };
    let doy = (153 * mp + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146_097 + doe - 719_468;
    Some(((days * 24 + hour) * 60 + minute) * 60_000 + second * 1_000 + milli)
}

/// `delete_model_state` on a given store: refused while a saved state names it
/// as its parent or a recorded run is tied to it (the blockers are named; the
/// run's foreign key refuses it in SQL as well); its labels row goes with it.
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn delete_state_in_store(
    path: &Path,
    project_id: &str,
    state_id: &str,
) -> Result<ModelStateDeleteReceipt, String> {
    delete_state_in_store_with(path, project_id, state_id, &UnmeteredReadMemory)
}

/// [`delete_state_in_store`] within `memory`: the state's summary, each blocking
/// child's summary (one at a time) and the blockers' names in the refusal are
/// reserved before they are loaded.
pub(crate) fn delete_state_in_store_with(
    path: &Path,
    project_id: &str,
    state_id: &str,
    memory: &dyn ReadMemory,
) -> Result<ModelStateDeleteReceipt, String> {
    let (mut connection, _migration) = crate::open_project_store(path)?;
    let transaction = crate::immediate_transaction(&mut connection)?;
    let metered = find_summary_with(&transaction, project_id, state_id, memory)?.ok_or_else(|| {
        format!("MODEL-STATE-NOT-FOUND: project `{project_id}` has no saved state `{state_id}`.")
    })?;
    let (state, _held) = metered.into_parts();
    // The refusal names every blocker: the children's names and the runs' names.
    let (child_names, child_count): (i64, i64) = transaction
        .query_row(
            "SELECT COALESCE(SUM(octet_length(c.state_id) + COALESCE(octet_length(l.state_name), 0)), 0), COUNT(*) FROM model_states c
             LEFT JOIN model_state_labels l ON l.project_id = c.project_id AND l.state_id = c.state_id
             WHERE c.project_id = ?1 AND c.parent_state_id = ?2",
            params![project_id, state_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(sql)?;
    let length = |value: i64| u64::try_from(value).unwrap_or(0);
    let _names = memory.reserve(
        &format!("the blockers of saved state `{state_id}`"),
        stored_read::total(&[
            stored_read::string_list_bytes(length(child_count), length(child_names)),
            stored_read::text_copy_bytes(length(child_names).saturating_mul(2)),
        ]),
    )?;
    let mut reasons = Vec::new();
    if !state.child_state_ids.is_empty() {
        let blockers = state
            .child_state_ids
            .iter()
            .map(
                |child| match find_summary_with(&transaction, project_id, child, memory) {
                    Ok(Some(summary)) => Ok(state_description(&summary.value)),
                    Ok(None) => Ok(child.clone()),
                    Err(error) if error.starts_with(stored_read::STORE_READ_MEMORY_UNAVAILABLE) => Err(error),
                    Err(_) => Ok(child.clone()),
                },
            )
            .collect::<Result<Vec<_>, String>>()?
            .join(", ");
        reasons.push(format!(
            "the parent of saved state{} {blockers}",
            if state.child_state_ids.len() == 1 { "" } else { "s" }
        ));
    }
    let runs = recorded_runs_of(&transaction, project_id, state_id)?;
    if !runs.is_empty() {
        let blockers = runs
            .iter()
            .map(|(recording_id, name)| match name {
                Some(name) => format!("\"{name}\" ({recording_id})"),
                None => recording_id.clone(),
            })
            .collect::<Vec<_>>()
            .join(", ");
        reasons.push(format!(
            "the state of recorded run{} {blockers}",
            if runs.len() == 1 { "" } else { "s" }
        ));
    }
    if !reasons.is_empty() {
        let count = state.child_state_ids.len() + runs.len();
        return Err(format!(
            "MODEL-STATE-DELETE-BLOCKED: state {} is {}; delete {} first. Nothing was deleted.",
            state_description(&state),
            reasons.join(" and "),
            if count == 1 { "it" } else { "them" }
        ));
    }
    let deleted = transaction
        .execute(
            "DELETE FROM model_states WHERE project_id = ?1 AND state_id = ?2",
            params![project_id, state_id],
        )
        .map_err(sql)?;
    transaction.commit().map_err(sql)?;
    Ok(ModelStateDeleteReceipt {
        project_id: project_id.to_string(),
        state_id: state_id.to_string(),
        deleted: deleted == 1,
        message: format!("Deleted model state {}.", state_description(&state)),
    })
}

// ---------------------------------------------------------------------------
// Commands (each admitted through `command_ingress` with its own budget).
// ---------------------------------------------------------------------------

/// `{request}` (snake_case fields, see `SaveModelStateRequest`).
pub(crate) fn save_model_state_args(
    args: &mut Args,
    store: impl FnOnce() -> Result<PathBuf, String>,
) -> Result<ModelStateSaveOutcome, CommandError> {
    let request = args.decode_with("request", SaveModelStateRequest::from_value)?;
    store()
        .and_then(|path| save_state_in_store(&path, request, &PRODUCTION_HOOKS))
        .map_err(CommandError::Command)
}

/// `{projectId}`.
pub(crate) fn list_model_states_args(
    args: &mut Args,
    store: impl FnOnce() -> Result<PathBuf, String>,
    memory: &dyn ReadMemory,
) -> Result<Metered<Vec<ModelStateSummary>>, CommandError> {
    let project_id: String = args.decode("projectId")?;
    store()
        .and_then(|path| {
            let (connection, _migration) = crate::open_project_store(&path)?;
            list_states_with(&connection, &project_id, memory)
        })
        .map_err(CommandError::Command)
}

/// `{projectId, stateId}`.
pub(crate) fn read_model_state_args(
    args: &mut Args,
    store: impl FnOnce() -> Result<PathBuf, String>,
    memory: &dyn ReadMemory,
) -> Result<Metered<ModelStateRead>, CommandError> {
    let project_id: String = args.decode("projectId")?;
    let state_id: String = args.decode("stateId")?;
    store()
        .and_then(|path| read_state_in_store_with(&path, &project_id, &state_id, memory))
        .map_err(CommandError::Command)
}

/// `{projectId, stateId, labels, expectedUpdatedAt?}`.
pub(crate) fn edit_model_state_labels_args(
    args: &mut Args,
    store: impl FnOnce() -> Result<PathBuf, String>,
    memory: &dyn ReadMemory,
) -> Result<Metered<ModelStateSummary>, CommandError> {
    let project_id: String = args.decode("projectId")?;
    let state_id: String = args.decode("stateId")?;
    let labels = args.decode_with("labels", |value| {
        labels_from_value(value).map_err(serde::de::Error::custom)
    })?;
    let expected: Option<String> = args.decode("expectedUpdatedAt")?;
    store()
        .and_then(|path| {
            edit_labels_in_store_with(
                &path,
                &project_id,
                &state_id,
                labels,
                expected.as_deref(),
                now_unix_millis()?,
                memory,
            )
        })
        .map_err(CommandError::Command)
}

/// `{projectId, stateId}`.
pub(crate) fn delete_model_state_args(
    args: &mut Args,
    store: impl FnOnce() -> Result<PathBuf, String>,
    memory: &dyn ReadMemory,
) -> Result<Metered<ModelStateDeleteReceipt>, CommandError> {
    let project_id: String = args.decode("projectId")?;
    let state_id: String = args.decode("stateId")?;
    store()
        .and_then(|path| delete_state_in_store_with(&path, &project_id, &state_id, memory).map(Metered::new))
        .map_err(CommandError::Command)
}

#[tauri::command]
pub fn save_model_state(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, _lease) =
        command_ingress::admit(&registry, &invoke, "save_model_state", ArgsDecoding::Moved)?;
    respond(save_model_state_args(&mut args, || {
        crate::app_store_path(&app)
    }))
}

#[tauri::command]
pub fn list_model_states(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, _lease) = command_ingress::admit_within(
        &registry,
        &invoke,
        "list_model_states",
        ArgsDecoding::Moved,
        LIST_MODEL_STATES_MAX_ARGS_BYTES,
    )?;
    let memory = PoolReadMemory::new(&registry);
    stored_read::respond_metered(
        &memory,
        "list_model_states",
        list_model_states_args(&mut args, || crate::app_store_path(&app), &memory),
    )
}

#[tauri::command]
pub fn read_model_state(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, _lease) = command_ingress::admit_within(
        &registry,
        &invoke,
        "read_model_state",
        ArgsDecoding::Moved,
        READ_MODEL_STATE_MAX_ARGS_BYTES,
    )?;
    let memory = PoolReadMemory::new(&registry);
    stored_read::respond_metered(
        &memory,
        "read_model_state",
        read_model_state_args(&mut args, || crate::app_store_path(&app), &memory),
    )
}

#[tauri::command]
pub fn edit_model_state_labels(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, _lease) = command_ingress::admit_within(
        &registry,
        &invoke,
        "edit_model_state_labels",
        ArgsDecoding::Moved,
        EDIT_MODEL_STATE_LABELS_MAX_ARGS_BYTES,
    )?;
    let memory = PoolReadMemory::new(&registry);
    stored_read::respond_metered(
        &memory,
        "edit_model_state_labels",
        edit_model_state_labels_args(&mut args, || crate::app_store_path(&app), &memory),
    )
}

#[tauri::command]
pub fn delete_model_state(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, _lease) = command_ingress::admit_within(
        &registry,
        &invoke,
        "delete_model_state",
        ArgsDecoding::Moved,
        DELETE_MODEL_STATE_MAX_ARGS_BYTES,
    )?;
    let memory = PoolReadMemory::new(&registry);
    stored_read::respond_metered(
        &memory,
        "delete_model_state",
        delete_model_state_args(&mut args, || crate::app_store_path(&app), &memory),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command_ingress::test_support::{admit_json, check_decode_bound, headers, registry};
    use crate::command_ingress::{admit_body_within, INGRESS_ARGS_JSON_V1};
    use crate::memory_pool::test_host::ScriptedHost;
    use crate::solve_jobs::SolveJobError;
    use std::cell::Cell;
    use tauri::ipc::InvokeBody;

    const PROJECT: &str = "project:w12-invented-states";
    /// The committed sorted-compact vectors (computed by service.py).
    const VECTORS: &str =
        include_str!("../../../../fixtures/persistence/sorted_compact_json_vectors.json");
    /// The committed record witness (written by this module's witness test).
    const WITNESS: &str =
        include_str!("../../../../fixtures/persistence/model_state_record_witness.json");
    const WITNESS_PATH_ENV: &str = "OPENPIPESTRESS_WRITE_MODEL_STATE_WITNESS";

    /// An invented 0.7.0 document: one node, a model assumption naming a
    /// support it does not have (a validator warning) and a resolved open issue.
    fn model(name: &str) -> Value {
        json!({
            "schema_version": open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION,
            "document_kind": "openpipestress.product_preview.model",
            "project": { "id": PROJECT, "name": name, "units": { "length": "m" } },
            "nodes": [{ "id": "node:N-100", "coordinates": [0, 0, 0] }],
            "pipe_segments": [], "supports": [], "materials": [], "sections": [], "components": [],
            "load_cases": [], "combinations": [],
            "unresolved_assumptions": [
                { "assumption_id": "assumption:A-001", "kind": "assumption", "status": "unresolved",
                  "statement": "Nozzle N-100 treated as an anchor pending vendor loads.", "affected_refs": ["support:S-404"] },
                { "assumption_id": "assumption:OI-002", "kind": "open_issue", "status": "resolved",
                  "statement": "Insulation density – confirm with the owner.", "affected_refs": [],
                  "resolution_note": "Confirmed: 120 kg/m³ (invented)." }
            ]
        })
    }

    fn scratch_store(label: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "swbpipe-states-{label}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&root).unwrap();
        root.join(crate::PROJECT_STORE_FILE)
    }

    /// A store holding the project row for `PROJECT`, with envelope hash `base`.
    fn store_with_project(label: &str, base: &str) -> PathBuf {
        let path = scratch_store(label);
        let (mut connection, _) = crate::open_project_store(&path).unwrap();
        write_project(&mut connection, &model("Invented project"), base);
        path
    }

    fn write_project(connection: &mut Connection, model: &Value, base: &str) {
        crate::upsert_project(
            connection,
            PROJECT,
            "Invented project",
            model,
            &json!([]),
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &json!({ "value": base }),
        )
        .unwrap();
    }

    fn labels(name: &str) -> StateLabels {
        labels_from_value(json!({
            "state_name": name,
            "tags": ["tie-in", "révision-B"],
            "notes": ["Basis for the Δp review – invented."],
            "assumptions": [{ "assumption_id": "assumption:S-1", "kind": "open_issue", "status": "unresolved",
                              "statement": "Vendor drawing pending.", "affected_refs": ["node:N-100"] }]
        }))
        .unwrap()
    }

    fn request(model: &Value, base: &str, name: &str) -> SaveModelStateRequest {
        let model_canonical = canonical_json(model);
        let model_hash = format!("sha256:{}", sha256_hex(&model_canonical));
        SaveModelStateRequest {
            project_id: PROJECT.to_string(),
            expected_base: base.to_string(),
            model_canonical,
            model_hash,
            state_kind: "design_snapshot".to_string(),
            parent_state_id: None,
            labels: labels(name),
        }
    }

    fn save(
        path: &Path,
        request: SaveModelStateRequest,
        token: &'static str,
        millis: i64,
    ) -> Result<ModelStateSaveOutcome, String> {
        let allocate = move || Ok((token.to_string(), millis));
        let between_rows = |_: &Transaction<'_>| Ok(());
        save_state_in_store(
            path,
            request,
            &StateWriteHooks {
                allocate: &allocate,
                between_rows: &between_rows,
            },
        )
    }

    fn row_count(path: &Path, table: &str) -> i64 {
        let connection = Connection::open(path).unwrap();
        connection
            .query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |row| {
                row.get(0)
            })
            .unwrap()
    }

    #[test]
    fn sorted_compact_serializer_matches_the_python_service_vectors() {
        let document: Value = serde_json::from_str(VECTORS).unwrap();
        let vectors = document["vectors"].as_array().unwrap();
        assert!(vectors.len() >= 5);
        for vector in vectors {
            let text = sorted_compact_json(&vector["value"]).unwrap();
            assert_eq!(
                text,
                vector["sorted_compact_json"].as_str().unwrap(),
                "{}",
                vector["name"]
            );
            assert_eq!(
                sha256_hex(&text),
                vector["sha256"].as_str().unwrap(),
                "{}",
                vector["name"]
            );
            assert!(text.is_ascii());
        }
        // Numbers the schema never puts in a record are refused, not approximated.
        let refused = document["refused_numbers"].as_array().unwrap();
        assert!(refused.len() >= 10);
        for entry in refused {
            let value = json!({ "statement": "s", "number": entry["value"].clone() });
            let error = sorted_compact_json(&value).unwrap_err();
            assert!(
                error.starts_with(
                    "MODEL-STATE-RECORD-NUMBER-UNSUPPORTED: the record holds the number "
                ),
                "{}: {error}",
                entry["python_sorted_compact_json"]
            );
        }
        for codex in [
            "999999999999975.2",
            "279836846559720.12",
            "18446744073709551616",
        ] {
            assert!(
                refused
                    .iter()
                    .any(|entry| entry["python_sorted_compact_json"] == codex),
                "{codex}"
            );
        }
    }

    #[test]
    fn a_record_holding_a_float_has_no_hash() {
        let mut record = json!({ "state_id": "state:x", "warnings": [] });
        assert!(record_hash_hex(&record).is_ok());
        record["warnings"] = json!([{ "magnitude": 999999999999975.2 }]);
        assert!(record_hash_hex(&record)
            .unwrap_err()
            .starts_with(RECORD_NUMBER_UNSUPPORTED));
        record["warnings"] = json!([{ "count": 18446744073709551615_u64 }]);
        assert_eq!(
            record_hash_hex(&record).unwrap().len(),
            64,
            "a u64 integer is exact"
        );
    }

    #[test]
    fn utc_times_are_millisecond_iso_and_round_trip() {
        assert_eq!(utc_iso_millis(0), "1970-01-01T00:00:00.000Z");
        assert_eq!(utc_iso_millis(951_782_400_000), "2000-02-29T00:00:00.000Z");
        assert_eq!(
            utc_iso_millis(1_793_100_645_123),
            "2026-10-27T11:30:45.123Z"
        );
        assert_eq!(
            utc_iso_millis(4_102_444_799_999),
            "2099-12-31T23:59:59.999Z"
        );
        for millis in [0, 951_782_400_000, 1_793_100_645_123, 4_102_444_799_999] {
            assert_eq!(parse_iso_millis(&utc_iso_millis(millis)), Some(millis));
        }
    }

    #[test]
    fn v12_migrates_a_v11_store_with_rows_and_is_safe_to_run_twice() {
        let path = scratch_store("v11");
        {
            let mut connection = Connection::open(&path).unwrap();
            connection
                .execute_batch("PRAGMA foreign_keys = ON;")
                .unwrap();
            for migration in crate::store_migrations()
                .into_iter()
                .filter(|migration| migration.version <= 11)
            {
                (migration.apply)(&connection).unwrap();
            }
            connection
                .execute_batch("PRAGMA user_version = 11;")
                .unwrap();
            write_project(&mut connection, &model("Invented project"), "sha256:base-1");
        }
        let (connection, evidence) = crate::open_project_store(&path).unwrap();
        assert_eq!(evidence.store_schema_version_before_open, 11);
        assert_eq!(evidence.store_schema_version, crate::STORE_SCHEMA_TARGET_VERSION);
        assert_eq!(
            evidence.migrations_applied_on_open,
            vec!["store-v12-model-states-tables", "store-v13-analysis-runs-tables"]
        );
        assert!(
            crate::load_project(&connection, Some(PROJECT))
                .unwrap()
                .is_some(),
            "the v11 project row is kept"
        );
        drop(connection);
        save(
            &path,
            request(&model("Invented project"), "sha256:base-1", "Basis"),
            "00000000000000000000000000000001",
            1_000,
        )
        .unwrap();
        // A re-run (as an interrupted, non-transactional migration would) changes nothing.
        let connection = Connection::open(&path).unwrap();
        connection
            .execute_batch("PRAGMA foreign_keys = ON; PRAGMA user_version = 11;")
            .unwrap();
        drop(connection);
        let (connection, evidence) = crate::open_project_store(&path).unwrap();
        assert_eq!(
            evidence.migrations_applied_on_open,
            vec!["store-v12-model-states-tables", "store-v13-analysis-runs-tables"]
        );
        migrate_v12_model_states(&connection).unwrap();
        assert_eq!(list_states(&connection, PROJECT).unwrap().len(), 1);
        assert!(crate::load_project(&connection, Some(PROJECT))
            .unwrap()
            .is_some());
        let error = connection
            .execute("UPDATE model_states SET state_kind = 'handoff_basis'", [])
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("MODEL-STATE-IMMUTABLE"),
            "the trigger survives the re-run: {error}"
        );
    }

    #[test]
    fn a_store_from_a_newer_build_is_refused_by_name_without_writing() {
        let path = scratch_store("newer");
        {
            let connection = Connection::open(&path).unwrap();
            connection
                .execute_batch("CREATE TABLE marker(x); PRAGMA user_version = 14;")
                .unwrap();
        }
        let before = std::fs::read(&path).unwrap();
        let error = crate::open_project_store(&path).map(|_| ()).unwrap_err();
        assert!(error.starts_with("STORE-SCHEMA-NEWER: the local project store is at store schema v14, newer than this build's v13"), "{error}");
        assert_eq!(std::fs::read(&path).unwrap(), before, "not a byte written");
        let connection = Connection::open(&path).unwrap();
        let mode: String = connection
            .query_row("PRAGMA journal_mode", [], |row| row.get(0))
            .unwrap();
        assert_eq!(mode, "delete", "the journal mode is not switched to WAL");
        assert_eq!(crate::store_user_version(&connection).unwrap(), 14);
        assert!(
            !store_has_table(&connection, "local_projects")
                && !store_has_table(&connection, "model_states")
        );
    }

    fn store_has_table(connection: &Connection, name: &str) -> bool {
        connection
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE name = ?1",
                params![name],
                |row| row.get::<_, i64>(0),
            )
            .unwrap()
            > 0
    }

    #[test]
    fn a_state_is_created_listed_and_read_with_its_labels_joined() {
        let path = store_with_project("create", "sha256:base-1");
        let snapshot = model("Invented project");
        let saved = save(
            &path,
            request(&snapshot, "sha256:base-1", "Basis – rev A"),
            "0123456789abcdef0123456789abcdef",
            1_793_100_645_123,
        )
        .unwrap();
        assert_eq!(saved.outcome, "saved");
        assert_eq!(
            saved.state.state_id,
            "state:0123456789abcdef0123456789abcdef"
        );
        assert_eq!(saved.state.created_at, "2026-10-27T11:30:45.123Z");
        assert_eq!(saved.state.labels_updated_at, saved.state.created_at);
        assert!(saved
            .message
            .contains("Renaming, tagging or adding notes does not change the saved state."));
        let (connection, _) = crate::open_project_store(&path).unwrap();
        assert_eq!(
            list_states(&connection, PROJECT).unwrap(),
            vec![saved.state.clone()]
        );
        let read = read_state_in_store(&path, PROJECT, &saved.state.state_id).unwrap();
        // The canonical snapshot round trip gives an equal hash.
        assert_eq!(read.model, snapshot);
        assert_eq!(
            format!("sha256:{}", sha256_hex(&canonical_json(&read.model))),
            saved.state.model_hash
        );
        assert_eq!(read.model_document_version["status"], "current");
        let record = &read.record;
        assert_eq!(record["state_name"], "Basis – rev A");
        assert_eq!(
            record["model_ref"],
            json!({ "object_type": "Project", "ref": PROJECT })
        );
        assert_eq!(record["parent_state_refs"], json!([]));
        assert_eq!(record["analysis_status"], json!(["HUMAN_REVIEW_REQUIRED"]));
        assert_eq!(record["tags"].as_array().unwrap().len(), 2);
        assert_eq!(
            record["notes"][0]["note_id"],
            "state:0123456789abcdef0123456789abcdef:note:1"
        );
        let assumptions = record["unresolved_assumptions"].as_array().unwrap();
        assert_eq!(
            assumptions.len(),
            3,
            "two model assumptions and one state-level assumption"
        );
        assert_eq!(
            assumptions[0]["affected_refs"],
            json!([{ "object_type": "Model", "ref": format!("{PROJECT}#support:S-404") }])
        );
        assert_eq!(
            assumptions[1]["statement"],
            "Open issue: Insulation density – confirm with the owner."
        );
        assert_eq!(
            assumptions[1]["affected_refs"],
            json!([{ "object_type": "Project", "ref": PROJECT }])
        );
        assert_eq!(
            assumptions[2]["affected_refs"],
            json!([{ "object_type": "Model", "ref": format!("{PROJECT}#node:N-100") }])
        );
        let warnings = record["warnings"].as_array().unwrap();
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0]["code"], "MODEL-ASSUMPTION-SUBJECT-MISSING");
        assert_eq!(warnings[0]["class"], "ASSUMPTION_WARNING");
        let hashes = record["hashes"].as_array().unwrap();
        assert_eq!(
            hashes[0]["value"].as_str().unwrap(),
            saved.state.model_hash.trim_start_matches("sha256:")
        );
        assert_eq!(
            hashes[1]["value"].as_str().unwrap(),
            saved.state.record_hash
        );
    }

    #[test]
    fn saving_an_already_saved_model_names_the_existing_state_and_the_unique_hash_holds() {
        let path = store_with_project("unique", "sha256:base-1");
        let snapshot = model("Invented project");
        let first = save(
            &path,
            request(&snapshot, "sha256:base-1", "First"),
            "000000000000000000000000000000a1",
            1_000,
        )
        .unwrap();
        let second = save(
            &path,
            request(&snapshot, "sha256:base-1", "Second"),
            "000000000000000000000000000000a2",
            2_000,
        )
        .unwrap();
        assert_eq!(second.outcome, "already_saved");
        assert_eq!(second.state, first.state);
        assert_eq!(
            second.message,
            format!(
                "This model is already saved as state \"First\" ({}); nothing was written.",
                first.state.state_id
            )
        );
        assert_eq!(row_count(&path, "model_states"), 1);
        let connection = Connection::open(&path).unwrap();
        let duplicate_hash = "INSERT INTO model_states SELECT project_id, 'state:other', state_kind, created_at, NULL, model_canonical, model_hash, record_json, record_hash FROM model_states";
        // The BEFORE INSERT trigger refuses it first...
        let error = connection
            .execute(duplicate_hash, [])
            .unwrap_err()
            .to_string();
        assert!(error.contains("MODEL-STATE-IMMUTABLE"), "{error}");
        // ...and UNIQUE(project_id, model_hash) holds on its own.
        connection
            .execute_batch("DROP TRIGGER model_states_no_replace;")
            .unwrap();
        let error = connection
            .execute(duplicate_hash, [])
            .unwrap_err()
            .to_string();
        assert!(error.contains("UNIQUE constraint failed"), "{error}");
    }

    #[test]
    fn a_snapshot_that_does_not_match_its_claim_is_refused() {
        let path = store_with_project("mismatch", "sha256:base-1");
        let snapshot = model("Invented project");
        let mut wrong_claim = request(&snapshot, "sha256:base-1", "Basis");
        wrong_claim.model_hash = format!("sha256:{}", "0".repeat(64));
        let error = save(
            &path,
            wrong_claim,
            "000000000000000000000000000000b1",
            1_000,
        )
        .unwrap_err();
        assert!(
            error.starts_with("MODEL-STATE-HASH-MISMATCH: the snapshot hashes to sha256:"),
            "{error}"
        );
        // Pretty-printed text hashed honestly is still not the canonical text.
        let mut not_canonical = request(&snapshot, "sha256:base-1", "Basis");
        not_canonical.model_canonical = serde_json::to_string_pretty(&snapshot).unwrap();
        not_canonical.model_hash = format!("sha256:{}", sha256_hex(&not_canonical.model_canonical));
        let error = save(
            &path,
            not_canonical,
            "000000000000000000000000000000b2",
            1_000,
        )
        .unwrap_err();
        assert!(
            error.starts_with(
                "MODEL-STATE-HASH-MISMATCH: the snapshot text is not its own canonical JCS text"
            ),
            "{error}"
        );
        assert_eq!(row_count(&path, "model_states"), 0);
    }

    #[test]
    fn an_update_of_a_state_row_raises_model_state_immutable() {
        let path = store_with_project("immutable", "sha256:base-1");
        save(
            &path,
            request(&model("Invented project"), "sha256:base-1", "Basis"),
            "000000000000000000000000000000c1",
            1_000,
        )
        .unwrap();
        let connection = Connection::open(&path).unwrap();
        for statement in [
            "UPDATE model_states SET model_canonical = '{}'",
            "UPDATE model_states SET record_hash = record_hash",
            "UPDATE model_states SET created_at = '1970-01-01T00:00:00.000Z'",
        ] {
            let error = connection.execute(statement, []).unwrap_err().to_string();
            assert!(
                error.contains("MODEL-STATE-IMMUTABLE"),
                "{statement}: {error}"
            );
        }
    }

    #[test]
    fn a_label_edit_changes_only_the_labels_and_leaves_the_record_hash() {
        let path = store_with_project("labels", "sha256:base-1");
        let saved = save(
            &path,
            request(&model("Invented project"), "sha256:base-1", "Basis"),
            "000000000000000000000000000000d1",
            1_000,
        )
        .unwrap();
        let stored_before: (String, String, String) = Connection::open(&path)
            .unwrap()
            .query_row(
                "SELECT model_canonical, record_json, record_hash FROM model_states",
                [],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .unwrap();
        let new_labels = labels_from_value(json!({ "state_name": "Basis – renamed", "tags": ["für-review"], "notes": [], "assumptions": [] })).unwrap();
        let edited = edit_labels_in_store(
            &path,
            PROJECT,
            &saved.state.state_id,
            new_labels.clone(),
            Some(&saved.state.labels_updated_at),
            500,
        )
        .unwrap();
        assert_eq!(edited.labels, new_labels);
        assert_eq!(edited.record_hash, saved.state.record_hash);
        assert_eq!(
            edited.labels_updated_at, "1970-01-01T00:00:01.001Z",
            "strictly later than the last write"
        );
        let stored_after: (String, String, String) = Connection::open(&path)
            .unwrap()
            .query_row(
                "SELECT model_canonical, record_json, record_hash FROM model_states",
                [],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .unwrap();
        assert_eq!(stored_after, stored_before);
        let read = read_state_in_store(&path, PROJECT, &saved.state.state_id).unwrap();
        assert_eq!(read.record["state_name"], "Basis – renamed");
        assert_eq!(
            read.record["unresolved_assumptions"]
                .as_array()
                .unwrap()
                .len(),
            2
        );
        // A stale window's edit is refused.
        let error = edit_labels_in_store(
            &path,
            PROJECT,
            &saved.state.state_id,
            labels("Other"),
            Some(&saved.state.labels_updated_at),
            9_000,
        )
        .unwrap_err();
        assert!(error.starts_with("MODEL-STATE-LABELS-MOVED"), "{error}");
        // A state-level assumption may not reuse a model assumption id.
        let clash = labels_from_value(json!({ "state_name": "x", "assumptions": [{ "assumption_id": "assumption:A-001", "kind": "assumption", "status": "unresolved", "statement": "s" }] })).unwrap();
        let error = edit_labels_in_store(&path, PROJECT, &saved.state.state_id, clash, None, 9_000)
            .unwrap_err();
        assert!(error.contains("already a model assumption"), "{error}");
    }

    #[test]
    fn labels_are_checked() {
        for (labels, expected) in [
            (json!({ "tags": [] }), "a state needs a name"),
            (json!({ "state_name": "  " }), "a state needs a name"),
            (
                json!({ "state_name": "n", "tags": ["a", "a"] }),
                "listed twice",
            ),
            (json!({ "state_name": "n", "tags": [" a"] }), "trimmed"),
            (
                json!({ "state_name": "n", "notes": [""] }),
                "a note is blank",
            ),
            (
                json!({ "state_name": "n", "colour": "red" }),
                "unknown member `colour`",
            ),
            (
                json!({ "state_name": "n", "assumptions": [{ "assumption_id": "1x", "kind": "assumption", "status": "unresolved", "statement": "s" }] }),
                "not a record id",
            ),
            (
                json!({ "state_name": "n", "assumptions": [{ "assumption_id": "a:1", "kind": "assumption", "status": "resolved", "statement": "s" }] }),
                "needs a resolution note",
            ),
            (
                json!({ "state_name": "n", "assumptions": [{ "assumption_id": "a:1", "kind": "guess", "status": "unresolved", "statement": "s" }] }),
                "`kind` must be",
            ),
        ] {
            let error = labels_from_value(labels).unwrap_err();
            assert!(
                error.starts_with("MODEL-STATE-LABELS-INVALID") && error.contains(expected),
                "{error}"
            );
        }
    }

    #[test]
    fn deleting_a_state_is_blocked_by_its_child_states_which_are_named() {
        let path = store_with_project("delete", "sha256:base-1");
        let parent = save(
            &path,
            request(&model("Invented project"), "sha256:base-1", "Parent"),
            "000000000000000000000000000000e1",
            1_000,
        )
        .unwrap();
        let mut child_request =
            request(&model("Invented project, edited"), "sha256:base-1", "Child");
        child_request.parent_state_id = Some(parent.state.state_id.clone());
        let child = save(
            &path,
            child_request,
            "000000000000000000000000000000e2",
            2_000,
        )
        .unwrap();
        assert_eq!(
            child.state.parent_state_id.as_deref(),
            Some(parent.state.state_id.as_str())
        );
        let read = read_state_in_store(&path, PROJECT, &child.state.state_id).unwrap();
        assert_eq!(
            read.record["parent_state_refs"],
            json!([{ "object_type": "ModelState", "ref": parent.state.state_id }])
        );
        let error = delete_state_in_store(&path, PROJECT, &parent.state.state_id).unwrap_err();
        assert_eq!(
            error,
            format!(
                "MODEL-STATE-DELETE-BLOCKED: state \"Parent\" ({}) is the parent of saved state \"Child\" ({}); delete it first. Nothing was deleted.",
                parent.state.state_id, child.state.state_id
            )
        );
        assert_eq!(row_count(&path, "model_states"), 2);
        // A parent of another project, or one that does not exist, is refused.
        let mut orphan = request(&model("Invented project, third"), "sha256:base-1", "Orphan");
        orphan.parent_state_id = Some("state:absent".to_string());
        assert!(
            save(&path, orphan, "000000000000000000000000000000e3", 3_000)
                .unwrap_err()
                .starts_with("MODEL-STATE-PARENT-MISSING")
        );
        assert!(
            delete_state_in_store(&path, PROJECT, &child.state.state_id)
                .unwrap()
                .deleted
        );
        assert!(
            delete_state_in_store(&path, PROJECT, &parent.state.state_id)
                .unwrap()
                .deleted
        );
        assert_eq!(row_count(&path, "model_states"), 0);
        assert_eq!(
            row_count(&path, "model_state_labels"),
            0,
            "the labels go with their state"
        );
    }

    #[test]
    fn the_state_and_labels_rows_are_one_transaction_that_rolls_back_on_failure() {
        let path = store_with_project("rollback", "sha256:base-1");
        let reached = Cell::new(false);
        let allocate = || Ok(("000000000000000000000000000000f1".to_string(), 1_000));
        let between_rows = |transaction: &Transaction<'_>| {
            let written: i64 = transaction
                .query_row("SELECT COUNT(*) FROM model_states", [], |row| row.get(0))
                .unwrap();
            assert_eq!(
                written, 1,
                "the state row is written inside the transaction"
            );
            reached.set(true);
            Err("INJECTED-FAILURE".to_string())
        };
        let error = save_state_in_store(
            &path,
            request(&model("Invented project"), "sha256:base-1", "Basis"),
            &StateWriteHooks {
                allocate: &allocate,
                between_rows: &between_rows,
            },
        )
        .unwrap_err();
        assert_eq!(error, "INJECTED-FAILURE");
        assert!(reached.get());
        assert_eq!(row_count(&path, "model_states"), 0);
        assert_eq!(row_count(&path, "model_state_labels"), 0);
    }

    #[test]
    fn foreign_keys_refuse_a_state_of_another_or_no_project() {
        let path = store_with_project("fk", "sha256:base-1");
        let saved = save(
            &path,
            request(&model("Invented project"), "sha256:base-1", "Basis"),
            "00000000000000000000000000000101",
            1_000,
        )
        .unwrap();
        let (connection, _) = crate::open_project_store(&path).unwrap();
        let insert = |project: &str, parent: Option<&str>| {
            connection
                .execute(
                    "INSERT INTO model_states VALUES (?1, 'state:x', 'design_snapshot', 't', ?2, '{}', 'sha256:x', '{}', 'h')",
                    params![project, parent],
                )
                .unwrap_err()
                .to_string()
        };
        assert!(insert("project:absent", None).contains("FOREIGN KEY constraint failed"));
        connection.execute("INSERT INTO local_projects (project_id, project_name, model_json, created_at_unix, updated_at_unix) VALUES ('project:other', 'o', '{}', 1, 1)", []).unwrap();
        // The other project cannot take this project's state as its parent.
        assert!(insert("project:other", Some(&saved.state.state_id))
            .contains("FOREIGN KEY constraint failed"));
        let error = connection
            .execute("INSERT INTO model_state_labels VALUES ('project:other', ?1, 'n', '[]', '[]', '[]', 't')", params![saved.state.state_id])
            .unwrap_err()
            .to_string();
        assert!(error.contains("FOREIGN KEY constraint failed"), "{error}");
        // The API refuses a project without a stored row by name.
        let mut stranger = request(&model("Invented project"), "sha256:base-1", "Basis");
        stranger.project_id = "project:absent".to_string();
        assert!(
            save(&path, stranger, "00000000000000000000000000000102", 1_000)
                .unwrap_err()
                .starts_with("MODEL-STATE-PROJECT-MISMATCH")
        );
    }

    #[test]
    fn a_state_needs_its_project_row_and_a_current_base() {
        let path = scratch_store("no-project");
        drop(crate::open_project_store(&path).unwrap());
        let error = save(
            &path,
            request(&model("Invented project"), "sha256:base-1", "Basis"),
            "00000000000000000000000000000201",
            1_000,
        )
        .unwrap_err();
        assert!(error.starts_with("MODEL-STATE-PROJECT-MISSING: project `project:w12-invented-states` has no stored local project"), "{error}");
        // Another window saved the project since this session last saw it.
        let path = store_with_project("stale", "sha256:base-2");
        let error = save(
            &path,
            request(&model("Invented project"), "sha256:base-1", "Basis"),
            "00000000000000000000000000000202",
            1_000,
        )
        .unwrap_err();
        assert!(
            error.starts_with("PROJECT-BASE-MOVED: project `project:w12-invented-states`"),
            "{error}"
        );
        assert_eq!(row_count(&path, "model_states"), 0);
        // A document that is not the current version is refused by name.
        let mut retired = model("Invented project");
        retired["schema_version"] = json!("0.6.0");
        let error = save(
            &path,
            request(&retired, "sha256:base-2", "Basis"),
            "00000000000000000000000000000203",
            1_000,
        )
        .unwrap_err();
        assert!(error.contains("status=retired_schema"), "{error}");
    }

    #[test]
    fn replacing_the_project_row_keeps_its_states_and_the_prompt_counts_them() {
        let path = store_with_project("replace", "sha256:base-1");
        save(
            &path,
            request(&model("Invented project"), "sha256:base-1", "One"),
            "00000000000000000000000000000301",
            1_000,
        )
        .unwrap();
        save(
            &path,
            request(&model("Invented project, two"), "sha256:base-1", "Two"),
            "00000000000000000000000000000302",
            2_000,
        )
        .unwrap();
        let (mut connection, _) = crate::open_project_store(&path).unwrap();
        let entry = crate::stored_project_entry(&connection, PROJECT)
            .unwrap()
            .unwrap();
        let revision = crate::stored_revision(&connection, &entry).unwrap();
        let transaction = crate::immediate_transaction(&mut connection).unwrap();
        let replacement = model("A different model under the same id");
        crate::guarded_write_project_rows(
            &transaction,
            Some(&revision),
            None,
            PROJECT,
            "Replaced",
            &replacement,
            &json!([]),
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &json!({ "value": "sha256:base-3" }),
        )
        .unwrap();
        transaction.commit().unwrap();
        assert_eq!(
            crate::load_project(&connection, Some(PROJECT))
                .unwrap()
                .unwrap()
                .model,
            replacement
        );
        assert_eq!(
            list_states(&connection, PROJECT).unwrap().len(),
            2,
            "the replacement keeps the states"
        );
        assert_eq!(saved_state_count(&connection, PROJECT).unwrap(), 2);
        assert_eq!(saved_state_count(&connection, "project:absent").unwrap(), 0);
        // The project-level cascade removes the states and their labels in one statement.
        connection
            .execute(
                "DELETE FROM local_projects WHERE project_id = ?1",
                params![PROJECT],
            )
            .unwrap();
        assert_eq!(row_count(&path, "model_states"), 0);
        assert_eq!(row_count(&path, "model_state_labels"), 0);
    }

    #[test]
    fn a_state_with_blocking_findings_is_saved_incomplete_and_not_opened() {
        let path = store_with_project("incomplete", "sha256:base-1");
        let mut incomplete = model("Invented project");
        // A resolved record without its resolution note blocks (W-13 §3.3).
        incomplete["unresolved_assumptions"][1]
            .as_object_mut()
            .unwrap()
            .remove("resolution_note");
        let saved = save(
            &path,
            request(&incomplete, "sha256:base-1", "Incomplete basis"),
            "00000000000000000000000000000501",
            1_000,
        )
        .unwrap();
        assert_eq!(saved.outcome, "saved");
        assert_eq!(
            saved.state.analysis_status,
            vec!["HUMAN_REVIEW_REQUIRED", "MODEL_INCOMPLETE"]
        );
        let (connection, _) = crate::open_project_store(&path).unwrap();
        assert_eq!(
            list_states(&connection, PROJECT).unwrap()[0].analysis_status,
            saved.state.analysis_status
        );
        let error = read_state_in_store(&path, PROJECT, &saved.state.state_id).unwrap_err();
        assert!(
            error.starts_with("Model document refused: malformed model-data slots: MODEL-ASSUMPTION-RESOLUTION-NOTE-REQUIRED"),
            "{error}"
        );
        // A complete state lists only the review status.
        let complete = save(
            &path,
            request(&model("Invented project"), "sha256:base-1", "Complete"),
            "00000000000000000000000000000502",
            2_000,
        )
        .unwrap();
        assert_eq!(
            complete.state.analysis_status,
            vec!["HUMAN_REVIEW_REQUIRED"]
        );
    }

    /// Edits rows outside the store's own paths: the triggers dropped and
    /// foreign keys off, as a hand edit of the file could.
    fn tamper(path: &Path, statements: &str) {
        Connection::open(path)
            .unwrap()
            .execute_batch(&format!(
                "PRAGMA foreign_keys = OFF; DROP TRIGGER IF EXISTS model_states_immutable; DROP TRIGGER IF EXISTS model_states_no_replace; {statements}"
            ))
            .unwrap();
    }

    fn expect_identity_refused(path: &Path, project: &str, state_id: &str, field: &str) {
        let expected = format!("MODEL-STATE-INTEGRITY: saved state `{state_id}`: the row's {field} differs from its verified record");
        let (connection, _) = crate::open_project_store(path).unwrap();
        let listed = list_states(&connection, project).unwrap_err();
        assert!(listed.starts_with(&expected), "list, {field}: {listed}");
        let read = read_state_in_store(path, project, state_id).unwrap_err();
        assert!(read.starts_with(&expected), "read, {field}: {read}");
    }

    #[test]
    fn each_identity_column_must_agree_with_the_verified_record() {
        let cases: [(&str, &str); 4] = [
            ("state_kind", "UPDATE model_states SET state_kind = 'handoff_basis';"),
            ("created_at", "UPDATE model_states SET created_at = '2000-01-01T00:00:00.000Z';"),
            ("state_id", "UPDATE model_states SET state_id = 'state:renamed'; UPDATE model_state_labels SET state_id = 'state:renamed';"),
            (
                "project",
                "INSERT INTO local_projects (project_id, project_name, model_json, created_at_unix, updated_at_unix) VALUES ('project:other', 'o', '{}', 1, 1);
                 UPDATE model_states SET project_id = 'project:other'; UPDATE model_state_labels SET project_id = 'project:other';",
            ),
        ];
        for (index, (field, statements)) in cases.into_iter().enumerate() {
            let path = store_with_project(&format!("identity-{index}"), "sha256:base-1");
            let saved = save(
                &path,
                request(&model("Invented project"), "sha256:base-1", "Basis"),
                "00000000000000000000000000000601",
                1_000,
            )
            .unwrap();
            assert!(read_state_in_store(&path, PROJECT, &saved.state.state_id).is_ok());
            tamper(&path, statements);
            let (project, state_id) = match field {
                "state_id" => (PROJECT, "state:renamed".to_string()),
                "project" => ("project:other", saved.state.state_id.clone()),
                _ => (PROJECT, saved.state.state_id.clone()),
            };
            expect_identity_refused(&path, project, &state_id, field);
        }
    }

    #[test]
    fn a_parent_column_or_model_hash_column_that_disagrees_with_the_record_is_refused() {
        let path = store_with_project("identity-parent", "sha256:base-1");
        let first = save(
            &path,
            request(&model("Invented project"), "sha256:base-1", "First"),
            "00000000000000000000000000000701",
            1_000,
        )
        .unwrap();
        let second = save(
            &path,
            request(&model("Invented project, two"), "sha256:base-1", "Second"),
            "00000000000000000000000000000702",
            2_000,
        )
        .unwrap();
        tamper(
            &path,
            &format!(
                "UPDATE model_states SET parent_state_id = '{}' WHERE state_id = '{}';",
                first.state.state_id, second.state.state_id
            ),
        );
        expect_identity_refused(&path, PROJECT, &second.state.state_id, "parent ref");

        // A snapshot and model hash swapped together for another model's: each
        // hashes to the other, but the record still names the original model.
        let path = store_with_project("identity-model", "sha256:base-1");
        let saved = save(
            &path,
            request(&model("Invented project"), "sha256:base-1", "Basis"),
            "00000000000000000000000000000703",
            1_000,
        )
        .unwrap();
        let other = canonical_json(&model("Invented project, swapped"));
        tamper(
            &path,
            &format!(
                "UPDATE model_states SET model_canonical = '{}', model_hash = 'sha256:{}';",
                other.replace('\'', "''"),
                sha256_hex(&other)
            ),
        );
        expect_identity_refused(&path, PROJECT, &saved.state.state_id, "model hash");
    }

    #[test]
    fn checksum_descriptors_must_be_exactly_the_expected_ones() {
        // Each edit changes only `hashes`, which the record digest leaves out.
        type Edit = fn(&mut Value);
        let edits: [(&str, Edit); 9] = [
            ("model payload ref", |hashes| {
                hashes[0]["payload_ref"]["ref"] = json!("project:other")
            }),
            ("model payload ref type", |hashes| {
                hashes[0]["payload_ref"]["object_type"] = json!("Project")
            }),
            ("canonicalization", |hashes| {
                hashes[0]["canonicalization"] = json!("NONE")
            }),
            ("algorithm", |hashes| {
                hashes[1]["algorithm"] = json!("sha512")
            }),
            ("payload scope", |hashes| {
                hashes[0]["payload_scope"] = json!("state_payload_partition")
            }),
            ("record payload ref", |hashes| {
                hashes[1]["payload_ref"]["ref"] = json!("state:other")
            }),
            ("duplicate entry", |hashes| {
                let copy = hashes[1].clone();
                hashes.as_array_mut().unwrap().push(copy);
            }),
            ("extra entry", |hashes| {
                hashes.as_array_mut().unwrap().push(json!({
                "algorithm": "sha256", "canonicalization": "NONE", "payload_ref": { "object_type": "AuditManifest", "ref": "x" },
                "payload_scope": "audit_manifest", "value": "00" }))
            }),
            ("missing entry", |hashes| {
                hashes.as_array_mut().unwrap().remove(0);
            }),
        ];
        for (index, (name, edit)) in edits.into_iter().enumerate() {
            let path = store_with_project(&format!("descriptor-{index}"), "sha256:base-1");
            let saved = save(
                &path,
                request(&model("Invented project"), "sha256:base-1", "Basis"),
                "00000000000000000000000000000b01",
                1_000,
            )
            .unwrap();
            let record_json: String = Connection::open(&path)
                .unwrap()
                .query_row("SELECT record_json FROM model_states", [], |row| row.get(0))
                .unwrap();
            let mut record: Value = serde_json::from_str(&record_json).unwrap();
            edit(&mut record["hashes"]);
            assert_eq!(
                record_hash_hex(&record).unwrap(),
                saved.state.record_hash,
                "{name}: the digest does not see it"
            );
            tamper(
                &path,
                &format!(
                    "UPDATE model_states SET record_json = '{}';",
                    serde_json::to_string(&record).unwrap().replace('\'', "''")
                ),
            );
            // A changed scope, or a missing model entry, already hides the model
            // hash from the identity check, which refuses first.
            let reason = if matches!(name, "payload scope" | "missing entry") {
                "the row's model hash differs from its verified record"
            } else {
                "the record's checksum descriptors differ from the expected ones"
            };
            let expected = format!(
                "MODEL-STATE-INTEGRITY: saved state `{}`: {reason}",
                saved.state.state_id
            );
            let (connection, _) = crate::open_project_store(&path).unwrap();
            let listed = list_states(&connection, PROJECT).unwrap_err();
            assert!(listed.starts_with(&expected), "{name}, list: {listed}");
            let read = read_state_in_store(&path, PROJECT, &saved.state.state_id).unwrap_err();
            assert!(read.starts_with(&expected), "{name}, read: {read}");
        }
    }

    #[test]
    fn insert_or_replace_and_replace_cannot_overwrite_a_state_or_drop_its_labels() {
        let path = store_with_project("replace-row", "sha256:base-1");
        let saved = save(
            &path,
            request(&model("Invented project"), "sha256:base-1", "Basis"),
            "00000000000000000000000000000801",
            1_000,
        )
        .unwrap();
        let (connection, _) = crate::open_project_store(&path).unwrap();
        let before: (String, String) = connection
            .query_row(
                "SELECT model_canonical, record_json FROM model_states",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        for verb in ["INSERT OR REPLACE INTO", "REPLACE INTO", "INSERT INTO"] {
            // Same key, different snapshot.
            let error = connection
                .execute(
                    &format!("{verb} model_states VALUES (?1, ?2, 'design_snapshot', 't', NULL, '{{}}', 'sha256:other', '{{}}', 'h')"),
                    params![PROJECT, saved.state.state_id],
                )
                .unwrap_err()
                .to_string();
            assert!(
                error.contains("MODEL-STATE-IMMUTABLE"),
                "{verb}, same key: {error}"
            );
            // Another id, same model hash (the UNIQUE key REPLACE would resolve by deleting).
            let error = connection
                .execute(
                    &format!("{verb} model_states VALUES (?1, 'state:other', 'design_snapshot', 't', NULL, '{{}}', ?2, '{{}}', 'h')"),
                    params![PROJECT, saved.state.model_hash],
                )
                .unwrap_err()
                .to_string();
            assert!(
                error.contains("MODEL-STATE-IMMUTABLE"),
                "{verb}, same model hash: {error}"
            );
        }
        let after: (String, String) = connection
            .query_row(
                "SELECT model_canonical, record_json FROM model_states",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(after, before);
        assert_eq!(row_count(&path, "model_states"), 1);
        assert_eq!(
            row_count(&path, "model_state_labels"),
            1,
            "the labels are not cascade-deleted"
        );
        assert!(read_state_in_store(&path, PROJECT, &saved.state.state_id).is_ok());
    }

    #[test]
    fn a_deleted_state_is_gone_and_its_model_saves_again_as_a_new_state() {
        let path = store_with_project("deleted-identity", "sha256:base-1");
        let snapshot = model("Invented project");
        let first = save(
            &path,
            request(&snapshot, "sha256:base-1", "First"),
            "00000000000000000000000000000901",
            1_000,
        )
        .unwrap();
        assert!(
            delete_state_in_store(&path, PROJECT, &first.state.state_id)
                .unwrap()
                .deleted
        );
        assert!(read_state_in_store(&path, PROJECT, &first.state.state_id)
            .unwrap_err()
            .starts_with("MODEL-STATE-NOT-FOUND"));
        // Re-saving the deleted state's model makes a new state (UNIQUE allows it
        // after the delete); its id is a fresh random token, never the old one.
        let again = save(
            &path,
            request(&snapshot, "sha256:base-1", "Again"),
            "00000000000000000000000000000902",
            2_000,
        )
        .unwrap();
        assert_eq!(again.outcome, "saved");
        assert_ne!(again.state.state_id, first.state.state_id);
        assert_eq!(again.state.model_hash, first.state.model_hash);
        assert_ne!(
            again.state.record_hash, first.state.record_hash,
            "a new identity and time make a new record"
        );
        // Production tokens are 128 random bits.
        let tokens: std::collections::BTreeSet<String> =
            (0..64).map(|_| random_token().unwrap()).collect();
        assert_eq!(tokens.len(), 64);
        assert!(tokens
            .iter()
            .all(|token| token.len() == 32 && token.bytes().all(|byte| byte.is_ascii_hexdigit())));
    }

    #[test]
    fn two_connections_saving_the_same_model_at_once_save_it_once() {
        let path = store_with_project("concurrent", "sha256:base-1");
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
        let handles: Vec<_> = [
            "00000000000000000000000000000a01",
            "00000000000000000000000000000a02",
        ]
        .into_iter()
        .enumerate()
        .map(|(index, token)| {
            let (path, barrier) = (path.clone(), barrier.clone());
            std::thread::spawn(move || {
                let request = request(
                    &model("Invented project"),
                    "sha256:base-1",
                    &format!("Window {index}"),
                );
                let allocate = move || Ok((token.to_string(), 1_000 + index as i64));
                let between_rows = |_: &Transaction<'_>| Ok(());
                barrier.wait();
                save_state_in_store(
                    &path,
                    request,
                    &StateWriteHooks {
                        allocate: &allocate,
                        between_rows: &between_rows,
                    },
                )
                .unwrap()
            })
        })
        .collect();
        let outcomes: Vec<ModelStateSaveOutcome> = handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect();
        let mut kinds: Vec<&str> = outcomes.iter().map(|outcome| outcome.outcome).collect();
        kinds.sort();
        assert_eq!(kinds, vec!["already_saved", "saved"]);
        assert_eq!(
            outcomes[0].state.state_id, outcomes[1].state.state_id,
            "the second names the first's state"
        );
        assert_eq!(row_count(&path, "model_states"), 1);
        assert_eq!(row_count(&path, "model_state_labels"), 1);
    }

    #[test]
    fn a_tampered_stored_state_is_not_opened() {
        let path = store_with_project("tamper", "sha256:base-1");
        let saved = save(
            &path,
            request(&model("Invented project"), "sha256:base-1", "Basis"),
            "00000000000000000000000000000401",
            1_000,
        )
        .unwrap();
        let connection = Connection::open(&path).unwrap();
        connection.execute_batch("DROP TRIGGER model_states_immutable; UPDATE model_states SET record_json = replace(record_json, 'HUMAN_REVIEW_REQUIRED', 'MECHANICS_SOLVED');").unwrap();
        let error = read_state_in_store(&path, PROJECT, &saved.state.state_id).unwrap_err();
        // The summary read verifies the record first, naming the state.
        assert!(
            error.starts_with(&format!(
                "MODEL-STATE-INTEGRITY: saved state `{}`: the stored record hashes to",
                saved.state.state_id
            )),
            "{error}"
        );
    }

    #[test]
    fn each_command_is_admitted_with_its_own_budget() {
        let registry = registry(ScriptedHost::normal());
        for (command, budget) in [
            ("list_model_states", LIST_MODEL_STATES_MAX_ARGS_BYTES),
            ("read_model_state", READ_MODEL_STATE_MAX_ARGS_BYTES),
            ("delete_model_state", DELETE_MODEL_STATE_MAX_ARGS_BYTES),
            (
                "edit_model_state_labels",
                EDIT_MODEL_STATE_LABELS_MAX_ARGS_BYTES,
            ),
        ] {
            let at_budget = InvokeBody::Raw(
                format!("{{\"projectId\":\"{}\"}}", "p".repeat(budget - 16)).into_bytes(),
            );
            let InvokeBody::Raw(raw) = &at_budget else {
                unreachable!()
            };
            assert_eq!(raw.len(), budget);
            assert!(admit_body_within(
                &registry,
                &at_budget,
                &headers(Some(INGRESS_ARGS_JSON_V1)),
                command,
                ArgsDecoding::Moved,
                budget
            )
            .is_ok());
            let over = InvokeBody::Raw(
                format!("{{\"projectId\":\"{}\"}}", "p".repeat(budget - 15)).into_bytes(),
            );
            match admit_body_within(
                &registry,
                &over,
                &headers(Some(INGRESS_ARGS_JSON_V1)),
                command,
                ArgsDecoding::Moved,
                budget,
            ) {
                Err(SolveJobError::InvalidRequest { message }) => {
                    assert_eq!(message, format!("INGRESS-ARGS-TOO-LARGE: `{command}` takes at most {budget} bytes of arguments; {} were sent.", budget + 1))
                }
                other => panic!("{command}: {other:?}"),
            }
            // A JSON body is still refused: the budget adds to the ingress rules.
            let json_body = InvokeBody::Json(json!({ "projectId": PROJECT }));
            assert!(matches!(
                admit_body_within(
                    &registry,
                    &json_body,
                    &headers(Some(INGRESS_ARGS_JSON_V1)),
                    command,
                    ArgsDecoding::Moved,
                    budget
                ),
                Err(SolveJobError::InvalidRequest { .. })
            ));
        }
        // save_model_state's snapshot decodes within its lease (moved, not copied).
        let snapshot = model("Invented project");
        let body = serde_json::to_vec(&json!({ "request": {
            "project_id": PROJECT, "expected_base": "sha256:base-1", "model_canonical": canonical_json(&snapshot),
            "model_hash": "sha256:x", "labels": { "state_name": "Basis", "tags": ["a"], "notes": ["n"], "assumptions": [] }
        } }))
        .unwrap();
        let (decoded, _, _) = check_decode_bound(&body, ArgsDecoding::Moved, |args| {
            args.decode_with("request", SaveModelStateRequest::from_value)
                .map(|request| request.model_canonical.len())
        });
        assert_eq!(decoded.unwrap(), canonical_json(&snapshot).len());
        let (mut args, _lease) = admit_json(
            &registry,
            "save_model_state",
            ArgsDecoding::Moved,
            &json!({ "request": { "project_id": PROJECT } }),
        )
        .unwrap();
        let error = save_model_state_args(&mut args, || Ok(scratch_store("args"))).unwrap_err();
        assert!(
            matches!(error, CommandError::Ingress(SolveJobError::InvalidRequest { ref message }) if message.contains("`request`")),
            "{error:?}"
        );
    }

    /// The populated record witness (`fixtures/persistence/model_state_record_witness.json`),
    /// produced by the save and read path itself. With
    /// `OPENPIPESTRESS_WRITE_MODEL_STATE_WITNESS=<path>` the test writes the witness
    /// there instead of comparing (the maintained writer).
    #[test]
    fn the_record_witness_is_what_the_save_and_read_path_emits() {
        let path = store_with_project("witness", "sha256:base-1");
        let parent = save(
            &path,
            request(&model("Invented project"), "sha256:base-1", "Basis – rev A"),
            "0123456789abcdef0123456789abcdef",
            1_793_100_645_123,
        )
        .unwrap();
        let mut child_request = request(
            &model("Invented project – rev B «Zürich»"),
            "sha256:base-1",
            "Rev B · für Prüfung",
        );
        child_request.parent_state_id = Some(parent.state.state_id.clone());
        child_request.state_kind = "comparison_basis".to_string();
        let child = save(
            &path,
            child_request,
            "fedcba9876543210fedcba9876543210",
            1_793_100_700_456,
        )
        .unwrap();
        let read = read_state_in_store(&path, PROJECT, &child.state.state_id).unwrap();
        let stored_record_json: String = Connection::open(&path)
            .unwrap()
            .query_row(
                "SELECT record_json FROM model_states WHERE state_id = ?1",
                params![child.state.state_id],
                |row| row.get(0),
            )
            .unwrap();
        // A state with empty labels (a name only) of a model without assumptions.
        let mut bare_model = model("Invented project – bare");
        bare_model
            .as_object_mut()
            .unwrap()
            .remove("unresolved_assumptions");
        let mut bare_request = request(&bare_model, "sha256:base-1", "Bare");
        bare_request.labels = labels_from_value(json!({ "state_name": "Bare" })).unwrap();
        let bare = save(
            &path,
            bare_request,
            "00112233445566778899aabbccddeeff",
            1_793_100_800_789,
        )
        .unwrap();
        let bare_read = read_state_in_store(&path, PROJECT, &bare.state.state_id).unwrap();
        let witness = json!({
            "description": "ModelStateRecords emitted by the desktop store's save and read path (apps/desktop/src-tauri/src/model_states.rs, test the_record_witness_is_what_the_save_and_read_path_emits) for invented models: emitted_record is populated; empty_labels_record has a name only and no assumptions, tags, notes or warnings. Regenerate with OPENPIPESTRESS_WRITE_MODEL_STATE_WITNESS=<path> on that test. tests/test_model_state_record_parity.py validates both against schemas/model_state.schema.json and recomputes record_hash from stored_record_json with core/project_persistence/service.py.",
            "empty_labels_record": bare_read.record,
            "record_hash": child.state.record_hash,
            "label_assumption_ids": child.state.labels.assumptions.iter().map(|assumption| assumption["assumption_id"].clone()).collect::<Vec<_>>(),
            "stored_record_json": stored_record_json,
            "emitted_record": read.record
        });
        let text = format!("{}\n", serde_json::to_string_pretty(&witness).unwrap());
        if let Ok(target) = std::env::var(WITNESS_PATH_ENV) {
            std::fs::write(target, &text).unwrap();
            return;
        }
        assert_eq!(
            text, WITNESS,
            "the committed witness is stale; regenerate it with {WITNESS_PATH_ENV}"
        );
    }
}
