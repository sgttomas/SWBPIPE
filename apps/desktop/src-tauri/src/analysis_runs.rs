//! Analysis runs recorded against model states in the desktop store (W-12 S2;
//! DEL-14-01 Design `desktop-states-runs-comparison.md` §3; FR-CMP-002 and
//! FR-MOD-006's run level).
//!
//! A solve's transient `analysis_run.v0.3` record (the current-run slot, with
//! the synthetic `state:<model ref>:preview` ref and `created_at: null`) is
//! unchanged. **Record Run** writes a new immutable revision of it, as v0.3
//! permits (`changes_create_new_immutable_record_revision`,
//! `analysis_run_record_sha256`), into `analysis_runs` (store v13):
//! - `created_at` is allocated inside the `BEGIN IMMEDIATE` transaction: the
//!   later of the current UTC time (millisecond ISO) and one millisecond after
//!   the project's latest recorded `created_at`, so every recording has its own
//!   revision hash;
//! - `model_state_ref` is `{ModelState, <state id>}`;
//! - `rule_pack_refs` are the input manifest's `active_rule_packs`, and
//!   `library_refs` the solved model's `provenance_record.library_ref`s;
//! - the `created_at_unavailable` determinism note is dropped;
//! - the record hash is recomputed here (`openpipestress_jcs_ijson_v1`, bare
//!   64-hex); the received-result bytes are stored as submitted, never
//!   rewritten.
//!
//! The binding to the state is checked, not asserted: the submitted input
//! manifest must be its own canonical JCS text, hash to the record's
//! `input_manifest_hashes[0]`, and its `model_basis.model_payload` must hash to
//! the state's model hash. The manifest is stored with the run so the binding
//! stays checkable. When the project holds no state with that model hash, the
//! user names a new `post_solve_snapshot`: the state, its labels, the run and
//! the run's labels are then written in one transaction.
//!
//! The run's name, tags, notes and run-level assumptions are its labels
//! (`analysis_run_labels`), outside the record hash (HELP_HUMAN, 2026-10-23).
//! `analysis_run.v0.3` has no assumption slot, so run-level assumptions are
//! shown with the run and are not in the record.
//!
//! Record Run never touches the project row (its current-run slot), so the
//! project envelope hash is unchanged. Nothing here approves, accepts,
//! certifies or issues anything (F-PIP-2, F-PIP-3).

use crate::command_ingress::{self, Args, ArgsDecoding, CommandError};
use crate::model_states::{
    self, find_summary_with, insert_new_state, labels_from_value_in, now_unix_millis, parse_iso_millis,
    sql, state_description, utc_iso_millis, LabelVocabulary, ModelStateSummary, StateLabels,
    StateWriteHooks,
};
use crate::solve_jobs::SolveJobRegistry;
use crate::stored_read::{self, Metered, PoolReadMemory, ReadMemory, Serialized, UnmeteredReadMemory};
#[cfg(test)]
use open_pipe_stress_canonical_json::canonical_json_checked_v1_text;
use open_pipe_stress_canonical_json::CHECKED_PROFILE_V1;
#[cfg(test)]
use open_pipe_stress_operation_applier::canonical_json;
use open_pipe_stress_operation_applier::sha256_hex;
use rusqlite::{params, Connection, OptionalExtension, Transaction};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use tauri::AppHandle;

/// Store v13: the run and run-label tables, the per-state index and the
/// immutability triggers, as design §3 gives them. Every statement is
/// `IF NOT EXISTS` (the store's migrations are not transactional).
/// `analysis_runs_immutable` refuses any `UPDATE`; `analysis_runs_no_replace`
/// refuses an `INSERT` that meets an existing run of the project on either key
/// (`recording_id` or `run_record_hash`), before conflict resolution, so
/// `INSERT OR REPLACE` and `REPLACE` can neither overwrite a run nor
/// cascade-delete its labels. A state's runs block its deletion through the
/// `NO ACTION` foreign key; deleting the project row cascades to its runs and
/// its states in one statement.
pub(crate) const STORE_V13_ANALYSIS_RUNS_SQL: &str = "
CREATE TABLE IF NOT EXISTS analysis_runs(
  project_id TEXT NOT NULL, recording_id TEXT NOT NULL, state_id TEXT NOT NULL,
  producer_run_id TEXT NOT NULL, created_at TEXT NOT NULL,
  run_record_json TEXT NOT NULL, run_record_hash TEXT NOT NULL,
  input_manifest_json TEXT NOT NULL, result_json TEXT NOT NULL, result_hash TEXT NOT NULL,
  PRIMARY KEY(project_id, recording_id), UNIQUE(project_id, run_record_hash),
  FOREIGN KEY(project_id, state_id) REFERENCES model_states(project_id, state_id),
  FOREIGN KEY(project_id) REFERENCES local_projects(project_id) ON DELETE CASCADE);
CREATE INDEX IF NOT EXISTS analysis_runs_by_state ON analysis_runs(project_id, state_id);
CREATE TABLE IF NOT EXISTS analysis_run_labels(
  project_id TEXT NOT NULL, recording_id TEXT NOT NULL, run_name TEXT NOT NULL,
  tags_json TEXT NOT NULL, notes_json TEXT NOT NULL, assumptions_json TEXT NOT NULL,
  updated_at TEXT NOT NULL, PRIMARY KEY(project_id, recording_id),
  FOREIGN KEY(project_id, recording_id) REFERENCES analysis_runs(project_id, recording_id) ON DELETE CASCADE);
CREATE TRIGGER IF NOT EXISTS analysis_runs_immutable BEFORE UPDATE ON analysis_runs
  BEGIN SELECT RAISE(ABORT, 'ANALYSIS-RUN-IMMUTABLE'); END;
CREATE TRIGGER IF NOT EXISTS analysis_runs_no_replace BEFORE INSERT ON analysis_runs
  WHEN EXISTS (SELECT 1 FROM analysis_runs WHERE project_id = NEW.project_id
    AND (recording_id = NEW.recording_id OR run_record_hash = NEW.run_record_hash))
  BEGIN SELECT RAISE(ABORT, 'ANALYSIS-RUN-IMMUTABLE'); END;
";

pub(crate) fn migrate_v13_analysis_runs(connection: &Connection) -> Result<(), String> {
    connection
        .execute_batch(STORE_V13_ANALYSIS_RUNS_SQL)
        .map_err(|error| stored_read::sql_error(&error))
}

/// Ingress budgets (raw argument bytes) of the small run commands.
/// `record_analysis_run` carries the result and the manifest and is bounded
/// by the per-invocation budget through its lease, as `save_local_project` is.
/// The history commands share one budget with the state commands (repair r7;
/// `model_states::HISTORY_COMMAND_MAX_ARGS_BYTES`).
pub(crate) const LIST_ANALYSIS_RUNS_MAX_ARGS_BYTES: usize = model_states::HISTORY_COMMAND_MAX_ARGS_BYTES;
pub(crate) const READ_ANALYSIS_RUN_MAX_ARGS_BYTES: usize = model_states::HISTORY_COMMAND_MAX_ARGS_BYTES;
pub(crate) const DELETE_ANALYSIS_RUN_MAX_ARGS_BYTES: usize = model_states::HISTORY_COMMAND_MAX_ARGS_BYTES;
pub(crate) const EDIT_ANALYSIS_RUN_LABELS_MAX_ARGS_BYTES: usize = model_states::HISTORY_COMMAND_MAX_ARGS_BYTES;

/// The state kind of the snapshot Record Run creates when no state holds the
/// solved model.
pub(crate) const POST_SOLVE_SNAPSHOT: &str = "post_solve_snapshot";

/// The record's determinism note that a recorded revision drops (its
/// `created_at` is set).
const CREATED_AT_UNAVAILABLE: &str = "created_at_unavailable";

pub(crate) const RUN_LABEL_VOCABULARY: LabelVocabulary = LabelVocabulary {
    name_key: "run_name",
    needs_name: "a recorded run needs a name",
    code: "ANALYSIS-RUN-LABELS-INVALID",
};

/// The sentence the History panel and the outcomes show where run labels change.
pub(crate) const LABELS_DO_NOT_CHANGE_RUN: &str =
    "Renaming, tagging or adding notes does not change the recorded run.";

/// A recorded run's editable labels: its name, tags, notes and run-level
/// assumptions (in the model's `unresolved_assumptions` vocabulary).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct RunLabels {
    pub(crate) run_name: String,
    pub(crate) tags: Vec<String>,
    pub(crate) notes: Vec<String>,
    pub(crate) assumptions: Vec<Value>,
}

pub(crate) fn run_labels_from_value(value: Value) -> Result<RunLabels, String> {
    let labels = labels_from_value_in(value, RUN_LABEL_VOCABULARY)?;
    Ok(RunLabels {
        run_name: labels.state_name,
        tags: labels.tags,
        notes: labels.notes,
        assumptions: labels.assumptions,
    })
}

// ---------------------------------------------------------------------------
// The v0.3 hashes and the recorded revision.
// ---------------------------------------------------------------------------

/// SHA-256 (bare hex) of a value's `openpipestress_jcs_ijson_v1` canonical text,
/// as the TypeScript `canonicalSha256HexCheckedV1` and Python's
/// `canonical_sha256_checked_v1` compute it (unmetered, for tests; the store
/// paths use [`stored_read::checked_sha256`], which reserves its texts first).
#[cfg(test)]
pub(crate) fn checked_sha256(value: &Value) -> Result<String, String> {
    let text = serde_json::to_string(value).map_err(|error| error.to_string())?;
    Ok(sha256_hex(&canonical_json_checked_v1_text(&text)?))
}

/// The record without its `analysis_run_record` checksum (v0.3's
/// `analysis_record_projection`); a duplicated checksum is refused.
fn record_projection(record: &Value) -> Result<Value, String> {
    let mut projected = record.clone();
    let hashes = projected
        .pointer_mut("/analysis_run/hashes")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| "ANALYSIS-RUN-RECORD-INVALID: the record has no hashes.".to_string())?;
    let claims = hashes
        .iter()
        .filter(|hash| hash.get("payload_scope").and_then(Value::as_str) == Some("analysis_run_record"))
        .count();
    if claims > 1 {
        return Err("ANALYSIS-RUN-RECORD-CHECKSUM-DUPLICATE".to_string());
    }
    hashes.retain(|hash| hash.get("payload_scope").and_then(Value::as_str) != Some("analysis_run_record"));
    Ok(projected)
}

/// The `analysis_run_record` hash of a v0.3 record (unmetered, for tests).
#[cfg(test)]
pub(crate) fn run_record_hash(record: &Value) -> Result<String, String> {
    run_record_hash_within(record, &UnmeteredReadMemory, "a record")
}

/// The `analysis_run_record` hash of a v0.3 record, its texts reserved in
/// `memory` at their exact lengths before they are built (the projection is a
/// clone of the record, priced by the caller with the record's parse).
pub(crate) fn run_record_hash_within(record: &Value, memory: &dyn ReadMemory, what: &str) -> Result<String, String> {
    stored_read::checked_sha256(memory, what, &record_projection(record)?)
}

/// The record's one checksum entry of `scope`, or `None` (absent or repeated).
fn single_hash<'a>(record: &'a Value, scope: &str) -> Option<&'a Value> {
    let hashes = record.pointer("/analysis_run/hashes")?.as_array()?;
    let mut found = hashes
        .iter()
        .filter(|hash| hash.get("payload_scope").and_then(Value::as_str) == Some(scope));
    let first = found.next()?;
    found.next().is_none().then_some(first)
}

fn single_manifest_hash(record: &Value) -> Option<&Value> {
    let hashes = record
        .pointer("/analysis_run/reproducibility/input_manifest_hashes")?
        .as_array()?;
    (hashes.len() == 1).then(|| &hashes[0])
}

fn reference(object_type: &str, reference: &str) -> Value {
    json!({ "object_type": object_type, "ref": reference })
}

/// A rule pack's identity as `rule_pack_refs` and the rule-pack reference
/// diagnostics name it: `<rule_pack_id>@<version>`, each component with `%`
/// (as `%25`) and then `@` (as `%40`) percent-encoded, so distinct (id,
/// version) pairs never share a string. An id and version without `%` or `@`
/// are unchanged. TypeScript (`rulePackIdentity`) and Python
/// (`rule_pack_identity`) encode the same.
pub(crate) fn rule_pack_identity(rule_pack_id: &str, version: &str) -> String {
    let component = |text: &str| text.replace('%', "%25").replace('@', "%40");
    format!("{}@{}", component(rule_pack_id), component(version))
}

pub(crate) const RULE_PACK_REF_UNRESOLVED: &str = "MODEL-RULE-PACK-REF-UNRESOLVED";
pub(crate) const RULE_PACK_REF_CHECKSUM_MISMATCH: &str = "MODEL-RULE-PACK-REF-CHECKSUM-MISMATCH";

/// The run record's diagnostic for a model rule-pack reference the solve left
/// out of the manifest's `active_rule_packs`, determined by its code and
/// identity alone (`rulePackRefDiagnostic` in TypeScript, Python's
/// `rule_pack_ref_diagnostic`): the producer's diagnostic shape, the identity
/// shown in the message to at most 160 code points.
pub(crate) fn rule_pack_ref_diagnostic(code: &str, identity: &str) -> Value {
    let shown: String = if identity.chars().count() > 160 {
        identity.chars().take(160).chain(std::iter::once('\u{2026}')).collect()
    } else {
        identity.to_string()
    };
    let reason = if code == RULE_PACK_REF_CHECKSUM_MISMATCH {
        "the stored pack's checksum was not the reference's at solve time"
    } else {
        "the local rule-pack store held no such pack for this project at solve time"
    };
    json!({
        "id": format!("diagnostic:rule-pack-ref:{identity}"),
        "code": code,
        "severity": "warning",
        "message": format!("Rule pack {shown} is referenced by the model, but {reason}; it is not among this run's rule-pack references."),
        "source": "desktop/solve-rule-pack-resolution",
        "affected_refs": [identity]
    })
}

/// The model's rule-pack references (`model_basis.model_payload.rule_pack_refs`)
/// the manifest's `active_rule_packs` leaves out, as (id, version) pairs in the
/// model's order.
pub(crate) fn manifest_excluded_rule_packs(manifest: &Value) -> Result<Vec<(String, String)>, String> {
    let invalid = |detail: &str| format!("ANALYSIS-RUN-MANIFEST-INVALID: {detail}.");
    let pair = |item: &Value, version_key: &str| match (item.get("rule_pack_id").and_then(Value::as_str), item.get(version_key).and_then(Value::as_str)) {
        (Some(id), Some(version)) => Some((id.to_string(), version.to_string())),
        _ => None,
    };
    // A model without references excludes nothing, whatever the manifest holds.
    let references = match manifest.pointer("/model_basis/model_payload/rule_pack_refs") {
        None => return Ok(Vec::new()),
        Some(value) => value.as_array().ok_or_else(|| invalid("the model's rule_pack_refs is not a list"))?,
    };
    if references.is_empty() {
        return Ok(Vec::new());
    }
    let active = manifest
        .get("active_rule_packs")
        .and_then(Value::as_array)
        .ok_or_else(|| invalid("the input manifest has no active_rule_packs list"))?
        .iter()
        .map(|pack| pair(pack, "version").ok_or_else(|| invalid("an active rule pack has no id or version")))
        .collect::<Result<Vec<_>, _>>()?;
    let mut excluded = Vec::new();
    for reference in references {
        let pair = pair(reference, "rule_pack_version").ok_or_else(|| invalid("a model rule-pack reference has no id or version"))?;
        if !active.contains(&pair) {
            excluded.push(pair);
        }
    }
    Ok(excluded)
}

/// Which of the record's `diagnostics` its result and stored manifest do not
/// fix, if any: the received result's diagnostics come first, each as
/// `{source_annotation: <diagnostic>}`; then exactly one rule-pack reference
/// diagnostic per reference the manifest leaves out of `active_rule_packs`, in
/// the model's order, each exactly the one its code and identity determine, no
/// identity twice. A record without a `diagnostics` list differs.
/// `Err` when the manifest's references cannot be read. The comparison clones
/// nothing from the stored texts; the expected rule-pack diagnostics are built
/// from the manifest's identities (residual: their size is the identities',
/// within the manifest's reserved parse).
pub(crate) fn diagnostics_difference(record: &Value, result: &Value, manifest: &Value) -> Result<Option<DiagnosticsDifference>, String> {
    let empty = Vec::new();
    let produced = match result.get("diagnostics") {
        None => &empty,
        Some(value) => match value.as_array() {
            Some(items) => items,
            None => return Ok(Some(DiagnosticsDifference::Producer)),
        },
    };
    // The list is required (v0.3; the TypeScript and Python validators refuse a record without it).
    let Some(diagnostics) = record.pointer("/analysis_run/diagnostics").and_then(Value::as_array) else {
        return Ok(Some(DiagnosticsDifference::Producer));
    };
    let wraps = |item: &Value, diagnostic: &Value| item.as_object().is_some_and(|object| object.len() == 1 && object.get("source_annotation") == Some(diagnostic));
    if diagnostics.len() < produced.len() || !diagnostics.iter().zip(produced).all(|(item, diagnostic)| wraps(item, diagnostic)) {
        return Ok(Some(DiagnosticsDifference::Producer));
    }
    let excluded = manifest_excluded_rule_packs(manifest)?;
    let appended = &diagnostics[produced.len()..];
    // A run recorded before rule-pack resolution (S2, #1596): the model has
    // references, the manifest resolved none, and no exclusion is named.
    let none_resolved = manifest.get("active_rule_packs").and_then(Value::as_array).is_some_and(Vec::is_empty);
    if appended.is_empty() && !excluded.is_empty() && none_resolved {
        return Ok(Some(DiagnosticsDifference::NotResolved));
    }
    if appended.len() != excluded.len() {
        return Ok(Some(DiagnosticsDifference::RulePacks));
    }
    // No identity twice, as the TypeScript and Python validators refuse a repeat
    // (a model naming one (id, version) pair twice included).
    let mut seen = std::collections::HashSet::new();
    for (item, (id, version)) in appended.iter().zip(&excluded) {
        if !seen.insert((id, version)) {
            return Ok(Some(DiagnosticsDifference::RulePacks));
        }
        let identity = rule_pack_identity(id, version);
        let code = item.pointer("/source_annotation/code").and_then(Value::as_str);
        let accepted = matches!(code, Some(RULE_PACK_REF_UNRESOLVED | RULE_PACK_REF_CHECKSUM_MISMATCH))
            && wraps(item, &rule_pack_ref_diagnostic(code.unwrap_or_default(), &identity));
        if !accepted {
            return Ok(Some(DiagnosticsDifference::RulePacks));
        }
    }
    Ok(None)
}

/// What [`diagnostics_difference`] found.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DiagnosticsDifference {
    /// The received result's diagnostics are not the record's first ones.
    Producer,
    /// The rest are not exactly the stored manifest's excluded references' diagnostics.
    RulePacks,
    /// The run was recorded before rule-pack resolution: the model has
    /// references, the manifest's `active_rule_packs` is empty and no exclusion
    /// diagnostic is present ([`RULE_PACKS_NOT_RESOLVED`]). Refused by name.
    NotResolved,
}

/// The refusal of a run recorded before rule-pack resolution (W-12; DEL-14-01
/// Design §3). Not a lenient reading: the run is refused, by name, so the user
/// knows to re-record it.
pub(crate) const RULE_PACKS_NOT_RESOLVED: &str = "ANALYSIS-RUN-RULE-PACKS-NOT-RESOLVED";
pub(crate) const RULE_PACKS_NOT_RESOLVED_DETAIL: &str = "recorded before rule-pack resolution; re-record this run";

/// The recorded revision's `rule_pack_refs`: the manifest's `active_rule_packs`
/// (what the solve used), in the manifest's order, as
/// `{RulePack, rule_pack_identity(<rule_pack_id>, <version>)}`.
pub(crate) fn rule_pack_refs(manifest: &Value) -> Result<Vec<Value>, String> {
    let packs = manifest
        .get("active_rule_packs")
        .and_then(Value::as_array)
        .ok_or_else(|| "ANALYSIS-RUN-MANIFEST-INVALID: the input manifest has no active_rule_packs list.".to_string())?;
    packs
        .iter()
        .map(|pack| {
            let id = pack.get("rule_pack_id").and_then(Value::as_str);
            let version = pack.get("version").and_then(Value::as_str);
            match (id, version) {
                (Some(id), Some(version)) if !id.is_empty() && !version.is_empty() => {
                    Ok(reference("RulePack", &rule_pack_identity(id, version)))
                }
                _ => Err("ANALYSIS-RUN-MANIFEST-INVALID: an active rule pack has no id or version.".to_string()),
            }
        })
        .collect()
}

/// The recorded revision's `library_refs`: every `library_ref` site of the
/// solved model that the model document's one walk finds
/// (`export_disposition_screen::library_ref_sites`, pinned to
/// `fixtures/model_document/library_ref_sites.json`), with its identity
/// normalized as that walk normalizes it (`library_ref_identity`: Rust's
/// White_Space trim). Each is written as
/// `{Library, "library:<kind>/<id>[@<version>][#<record id>]"}`, with
/// `library_version` verbatim when it is text (the store check compares it
/// verbatim). The list is deduplicated and sorted by code point. A site whose
/// identity is null in the table (its kind or id blank or not text) names no
/// library and is omitted; the state's hashed model payload still holds it, and
/// the open-time store check reports it as unresolved.
pub(crate) fn library_refs(model_payload: &Value) -> Vec<Value> {
    let found: BTreeSet<String> = crate::export_disposition_screen::library_ref_sites(model_payload)
        .into_iter()
        .filter_map(|(_, reference)| {
            let (kind, id, record_id) = crate::export_disposition_screen::library_ref_identity(reference)?;
            let mut text = format!("library:{kind}/{id}");
            if let Some(version) = reference.get("library_version").and_then(Value::as_str) {
                text.push('@');
                text.push_str(version);
            }
            if let Some(record_id) = record_id {
                text.push('#');
                text.push_str(record_id);
            }
            Some(text)
        })
        .collect();
    found.iter().map(|text| reference("Library", text)).collect()
}

/// The recorded revision of a transient record (design §3), and its hash. The
/// received-result checksum and every other member are unchanged.
pub(crate) fn recorded_revision(
    memory: &dyn ReadMemory,
    transient: &Value,
    state_id: &str,
    created_at: &str,
    manifest: &Value,
) -> Result<(Value, String), String> {
    let model_payload = manifest
        .pointer("/model_basis/model_payload")
        .ok_or_else(|| "ANALYSIS-RUN-MANIFEST-INVALID: the input manifest has no model payload.".to_string())?;
    let mut record = transient.clone();
    let run = record
        .get_mut("analysis_run")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| "ANALYSIS-RUN-RECORD-INVALID: the record has no analysis_run.".to_string())?;
    run.insert("created_at".to_string(), json!(created_at));
    run.insert("model_state_ref".to_string(), reference("ModelState", state_id));
    run.insert("rule_pack_refs".to_string(), Value::Array(rule_pack_refs(manifest)?));
    run.insert("library_refs".to_string(), Value::Array(library_refs(model_payload)));
    if let Some(notes) = run
        .get_mut("reproducibility")
        .and_then(|reproducibility| reproducibility.get_mut("determinism_notes"))
        .and_then(Value::as_array_mut)
    {
        notes.retain(|note| note.as_str() != Some(CREATED_AT_UNAVAILABLE));
    }
    let hash = run_record_hash_within(&record, memory, "the recorded revision")?;
    let claim = record
        .pointer_mut("/analysis_run/hashes")
        .and_then(Value::as_array_mut)
        .and_then(|hashes| {
            hashes
                .iter_mut()
                .find(|hash| hash.get("payload_scope").and_then(Value::as_str) == Some("analysis_run_record"))
        })
        .ok_or_else(|| "ANALYSIS-RUN-RECORD-INVALID: the record has no analysis_run_record checksum.".to_string())?;
    claim["value"] = json!(hash);
    Ok((record, hash))
}

/// `runrec:` and the first 32 hex characters of the record hash.
pub(crate) fn recording_id_of(record_hash: &str) -> String {
    // Checked slicing (repair r6): the hash is hex, but nothing here assumes it.
    format!("runrec:{}", record_hash.get(..32).unwrap_or(record_hash))
}

// ---------------------------------------------------------------------------
// The members a record's checksums and its input manifest fix (repair r1).
// ---------------------------------------------------------------------------

/// `inputManifestService.ts`'s `safeToken`: every run of characters outside
/// `[A-Za-z0-9._-]` becomes `-`, leading and trailing `-` are trimmed, and an
/// empty token is `current-session`.
pub(crate) fn safe_token(value: &str) -> String {
    let mut token = String::new();
    let mut in_run = false;
    for character in value.chars() {
        if character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-') {
            token.push(character);
            in_run = false;
        } else if !in_run {
            token.push('-');
            in_run = true;
        }
    }
    let trimmed = token.trim_matches('-');
    if trimmed.is_empty() { "current-session".to_string() } else { trimmed.to_string() }
}

/// The input manifest's ref, as the session builds it.
pub(crate) fn input_manifest_ref(model_ref: &str, manifest_hash: &str) -> Value {
    reference("InputManifest", &format!("input-manifest:{}:{manifest_hash}", safe_token(model_ref)))
}

fn checksum(canonicalization: &str, payload_ref: Value, scope: &str, value: &str) -> Value {
    json!({ "algorithm": "sha256", "canonicalization": canonicalization, "payload_ref": payload_ref, "payload_scope": scope, "value": value })
}

/// A v0.3 record's whole `hashes` inventory, exactly as the builders write it:
/// the record checksum, then the received-result checksum.
pub(crate) fn expected_hashes(run_id: &str, record_hash: &str, result_hash: &str) -> Value {
    json!([
        checksum(CHECKED_PROFILE_V1, reference("AnalysisRun", run_id), "analysis_run_record", record_hash),
        checksum(CHECKED_PROFILE_V1, reference("ResultEnvelope", &format!("result-envelope:{run_id}")), "received_result", result_hash)
    ])
}

/// The record members a project and its input manifest's hash fix without the
/// manifest itself: the manifest's ref and checksum, the settings ref and the
/// unit-system ref (`buildAnalysisRunV03`).
fn manifest_hash_members(model_ref: &str, manifest_hash: &str) -> Vec<(&'static str, &'static str, Value)> {
    let manifest_ref = input_manifest_ref(model_ref, manifest_hash);
    let settings = format!("solver-settings:{}:{manifest_hash}", manifest_ref["ref"].as_str().unwrap_or(""));
    vec![
        ("input manifest ref", "/reproducibility/input_manifest_refs", json!([manifest_ref.clone()])),
        ("input manifest checksum", "/reproducibility/input_manifest_hashes", json!([checksum("rfc8785_jcs", manifest_ref, "input_manifest", manifest_hash)])),
        ("settings ref", "/settings_ref", reference("SolverSettings", &settings)),
        ("unit-system ref", "/unit_system_ref", reference("UnitSystem", &format!("unit-system:{model_ref}:{manifest_hash}"))),
    ]
}

/// Every record member the stored input manifest fixes (never the record's own
/// values): the members of [`manifest_hash_members`], the solver identity and
/// build ref, the load basis (the manifest's load cases, then its
/// combinations), and the rule-pack and library references (`recorded`) or
/// none (a transient record).
pub(crate) fn manifest_members(
    manifest: &Value,
    manifest_hash: &str,
    recorded: bool,
) -> Result<Vec<(&'static str, &'static str, Value)>, String> {
    let invalid = |detail: &str| format!("ANALYSIS-RUN-MANIFEST-INVALID: the input manifest has no {detail}.");
    let model_ref = manifest.pointer("/model_basis/model_ref").and_then(Value::as_str).ok_or_else(|| invalid("model ref"))?;
    let solver = manifest.get("solver_basis").ok_or_else(|| invalid("solver basis"))?;
    let text = |key: &str| solver.get(key).and_then(Value::as_str).ok_or_else(|| invalid(&format!("solver_basis.{key}")));
    let mut load_basis = Vec::new();
    for (key, object_type) in [("load_cases", "LoadCase"), ("combinations", "Combination")] {
        let entries = manifest
            .pointer(&format!("/load_basis/{key}"))
            .and_then(Value::as_array)
            .ok_or_else(|| invalid(&format!("load_basis.{key}")))?;
        for entry in entries {
            let id = entry.get("id").and_then(Value::as_str).ok_or_else(|| invalid(&format!("id in load_basis.{key}")))?;
            load_basis.push(reference(object_type, id));
        }
    }
    let mut members = manifest_hash_members(model_ref, manifest_hash);
    members.push((
        "solver version",
        "/solver_version",
        json!({ "solver_name": text("solver_name")?, "solver_version": text("solver_version")?,
                "build_ref": reference("ExternalReference", text("solver_build_ref")?) }),
    ));
    members.push(("load basis", "/load_basis_refs", Value::Array(load_basis)));
    let payload = manifest.pointer("/model_basis/model_payload").ok_or_else(|| invalid("model payload"))?;
    let (rule_packs, libraries) = if recorded {
        (Value::Array(rule_pack_refs(manifest)?), Value::Array(library_refs(payload)))
    } else {
        (json!([]), json!([]))
    };
    members.push(("rule-pack refs", "/rule_pack_refs", rule_packs));
    members.push(("library refs", "/library_refs", libraries));
    Ok(members)
}

/// The first of `members` the record's `analysis_run` does not hold exactly
/// (structural equality: member order inside an object is not significant,
/// array order is, and an extra or missing member is a difference).
fn first_difference(record: &Value, members: &[(&'static str, &'static str, Value)]) -> Option<&'static str> {
    let run = record.get("analysis_run")?;
    members
        .iter()
        .find(|(_, pointer, expected)| run.pointer(pointer) != Some(expected))
        .map(|(name, _, _)| *name)
}

/// The record's `result_refs` against the result's rows: one per row, in order,
/// each naming its row and carrying exactly the row's checked checksum
/// descriptor. `None` when they agree.
fn result_rows_difference(record: &Value, result: &Value, memory: &dyn ReadMemory) -> Result<Option<String>, String> {
    let rows = result.get("results").and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
    let refs = record.pointer("/analysis_run/result_refs").and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
    if rows.len() != refs.len() {
        return Ok(Some(format!("the record has {} result refs for {} result rows", refs.len(), rows.len())));
    }
    for (index, (row, row_ref)) in rows.iter().zip(refs).enumerate() {
        let id = row.get("id").and_then(Value::as_str).unwrap_or("");
        let result_ref = reference("Result", id);
        let row_hash = stored_read::checked_sha256(memory, &format!("result row `{}`", stored_read::shown(id)), row)?;
        let expected = json!([checksum(CHECKED_PROFILE_V1, result_ref.clone(), "result_row", &row_hash)]);
        if row_ref.get("result_ref") != Some(&result_ref)
            || row_ref.get("source_row_index") != Some(&json!(index))
            || row_ref.get("hash_refs") != Some(&expected)
        {
            return Ok(Some(format!("result ref {index} does not carry result row `{}`'s checksum", stored_read::shown(id))));
        }
    }
    Ok(None)
}

// ---------------------------------------------------------------------------
// The binding checks of Record Run (before any store access).
// ---------------------------------------------------------------------------

/// `analysis_run.v0.3`'s analysis status words (its schema's enum).
const ANALYSIS_STATUS_WORDS: [&str; 6] =
    ["MODEL_INCOMPLETE", "MECHANICS_SOLVED", "RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED", "HUMAN_REVIEW_REQUIRED"];

fn refused(code: &str, detail: impl std::fmt::Display) -> String {
    format!("{code}: {detail}; nothing was recorded.")
}

/// The fields of a result Record Run reads (unknown members are skipped, not
/// materialized).
#[derive(Deserialize)]
struct ResultHead {
    run_id: String,
    model_ref: String,
}

/// What the checks establish about a submission.
pub(crate) struct CheckedSubmission {
    pub(crate) manifest: Value,
    /// The JCS text of the manifest's model payload and its `sha256:<hex>`.
    pub(crate) payload_canonical: String,
    pub(crate) payload_hash: String,
    pub(crate) producer_run_id: String,
    pub(crate) result_hash_hex: String,
    /// The record was made before rule-pack resolution ([`RULE_PACKS_NOT_RESOLVED`]):
    /// refused once every other check of Record Run passes, so a corrupt
    /// submission gets its own refusal first (W-12).
    pub(crate) rule_packs_not_resolved: bool,
}

/// The binding of a submission, checked, not asserted:
/// - the input manifest is its own canonical JCS text and its SHA-256 is the
///   record's `input_manifest_hashes[0]`, whose ref is `input_manifest_refs[0]`;
/// - the manifest and its model payload are the project's;
/// - the record is a transient v0.3 record (`created_at` null, the synthetic
///   `state:<project>:preview` ref) whose own checksum verifies;
/// - the result is the record's received result (its checked SHA-256 and ref)
///   and its run and project.
///
/// Its working memory is reserved in `memory` before it is used: the manifest's
/// and the result's parses at the prescan's bound (a text the prescan refuses is
/// refused before it is parsed), and every canonical text (the manifest's, the
/// model payload's, the record's and the result's checked texts, each row's) at
/// its exact length. The submission holds the manifest's and the payload text's
/// reservations. Residual: the record's hash projection is a clone of the
/// submitted record, within the request's argument lease.
pub(crate) fn checked_submission(
    project_id: &str,
    transient: &Value,
    result_json: &str,
    manifest_text: &str,
    memory: &dyn ReadMemory,
) -> Result<Metered<CheckedSubmission>, String> {
    const MANIFEST: &str = "ANALYSIS-RUN-MANIFEST-MISMATCH";
    const RESULT: &str = "ANALYSIS-RUN-RESULT-MISMATCH";
    let result_parse = stored_read::parse_bytes(result_json).map_err(|detail| refused(RESULT, format_args!("the result: {detail}")))?;
    let manifest_parse =
        stored_read::parse_bytes(manifest_text).map_err(|detail| refused(MANIFEST, format_args!("the input manifest: {detail}")))?;
    let manifest_reservation = memory.reserve("the run to record's input manifest", manifest_parse)?;
    let manifest: Value = serde_json::from_str(manifest_text)
        .map_err(|error| refused(MANIFEST, format_args!("the input manifest is not JSON ({error})", error = stored_read::json_error(&error))))?;
    if !stored_read::is_jcs_text(memory, "the run to record's canonical input manifest", &manifest, manifest_text)? {
        return Err(refused(MANIFEST, "the input manifest is not its own canonical JCS text"));
    }
    let manifest_hash_hex = sha256_hex(manifest_text);
    let claimed = single_manifest_hash(transient);
    if claimed.and_then(|hash| hash.get("value")).and_then(Value::as_str) != Some(manifest_hash_hex.as_str()) {
        return Err(refused(
            MANIFEST,
            format_args!("the input manifest hashes to {manifest_hash_hex}, not the record's input_manifest_hashes[0]"),
        ));
    }
    let refs = transient.pointer("/analysis_run/reproducibility/input_manifest_refs").and_then(Value::as_array);
    if refs.map(Vec::len) != Some(1)
        || claimed.and_then(|hash| hash.get("payload_ref")) != refs.and_then(|refs| refs.first())
    {
        return Err(refused(MANIFEST, "the record's input manifest ref and hash name different manifests"));
    }
    let model_payload = manifest
        .pointer("/model_basis/model_payload")
        .ok_or_else(|| refused(MANIFEST, "the input manifest has no model payload"))?;
    let manifest_project = manifest.pointer("/model_basis/model_ref").and_then(Value::as_str);
    if manifest_project != Some(project_id) || crate::project_id_from_model(model_payload) != project_id {
        return Err(refused(MANIFEST, format_args!("the input manifest is not of project `{project_id}`", project_id = stored_read::shown(&project_id))));
    }
    let payload_canonical = stored_read::jcs_text(memory, "the run to record's model payload", model_payload)?;
    let payload_hash = format!("sha256:{}", sha256_hex(payload_canonical.get()));

    const RECORD: &str = "ANALYSIS-RUN-NOT-TRANSIENT";
    if transient.get("schema_version").and_then(Value::as_str) != Some("0.3.0")
        || transient.pointer("/run_contract_status/record_contract").and_then(Value::as_str)
            != Some("strict_analysis_run_v0_3")
    {
        return Err(refused(RECORD, "the record is not a current (0.3.0) analysis-run record"));
    }
    let synthetic = reference("ModelState", &format!("state:{project_id}:preview"));
    if transient.pointer("/analysis_run/created_at") != Some(&Value::Null)
        || transient.pointer("/analysis_run/model_state_ref") != Some(&synthetic)
    {
        return Err(refused(RECORD, "the record is not a solve's transient record (an already recorded run is not recorded again)"));
    }
    // What a run's summary retains of its status (repair r6): v0.3's status
    // words (the schema's enum), each at most once, as the app produces them.
    // The schema itself leaves the list's length open; Record Run does not.
    let status = transient.pointer("/analysis_run/analysis_status").and_then(Value::as_array);
    let status_accepted = status.is_some_and(|items| {
        items.len() <= ANALYSIS_STATUS_WORDS.len()
            && items.iter().enumerate().all(|(index, item)| {
                item.as_str().is_some_and(|word| ANALYSIS_STATUS_WORDS.contains(&word)) && !items[..index].contains(item)
            })
    });
    if !status_accepted {
        return Err(refused(RECORD, "the record's analysis status is not a list of v0.3's status words, each at most once"));
    }
    let run_id = transient.pointer("/analysis_run/run_id").and_then(Value::as_str).unwrap_or("");
    let claim = single_hash(transient, "analysis_run_record");
    let recomputed = run_record_hash_within(transient, memory, "the run to record's record").map_err(|error| {
        if error.starts_with(stored_read::STORE_READ_MEMORY_UNAVAILABLE) { error } else { refused(RECORD, error) }
    })?;
    if claim.and_then(|hash| hash.get("value")).and_then(Value::as_str) != Some(recomputed.as_str()) {
        return Err(refused(RECORD, format_args!("the record's own checksum does not verify (it hashes to {recomputed})")));
    }
    // The members the manifest fixes, from the manifest (never from the record).
    if let Some(name) = first_difference(transient, &manifest_members(&manifest, &manifest_hash_hex, false).map_err(|error| refused(MANIFEST, error))?) {
        return Err(refused(MANIFEST, format_args!("the record's {name} is not the input manifest's")));
    }

    let head: ResultHead = serde_json::from_str(result_json)
        .map_err(|error| refused(RESULT, format_args!("the result is not a mechanics result ({error})", error = stored_read::json_error(&error))))?;
    let result_reservation = memory.reserve("the run to record's result", result_parse)?;
    let result: Value = serde_json::from_str(result_json)
        .map_err(|error| refused(RESULT, format_args!("the result is not JSON ({error})", error = stored_read::json_error(&error))))?;
    let result_hash_hex = stored_read::checked_sha256_text(memory, "the run to record's result", result_json, &result).map_err(|error| {
        if error.starts_with(stored_read::STORE_READ_MEMORY_UNAVAILABLE) { error } else { refused(RESULT, error) }
    })?;
    let received = single_hash(transient, "received_result");
    if received.and_then(|hash| hash.get("value")).and_then(Value::as_str) != Some(result_hash_hex.as_str())
        || received.and_then(|hash| hash.get("payload_ref"))
            != Some(&reference("ResultEnvelope", &format!("result-envelope:{}", head.run_id)))
    {
        return Err(refused(RESULT, format_args!("the result hashes to {result_hash_hex}, not the record's received result")));
    }
    if head.run_id != run_id || head.model_ref != project_id {
        return Err(refused(RESULT, "the result is not the record's run of this project"));
    }
    // The whole checksum inventory, exactly: algorithm, profile, scope, payload
    // ref and value of both checksums, nothing else (v0.3; S1's exact descriptors).
    if transient.pointer("/analysis_run/hashes") != Some(&expected_hashes(run_id, &recomputed, &result_hash_hex)) {
        return Err(refused("ANALYSIS-RUN-CHECKSUM-MISMATCH", "the record's checksum descriptors are not exactly the v0.3 ones"));
    }
    if let Some(detail) = result_rows_difference(transient, &result, memory).map_err(|error| {
        if error.starts_with(stored_read::STORE_READ_MEMORY_UNAVAILABLE) { error } else { refused(RESULT, error) }
    })? {
        return Err(refused("ANALYSIS-RUN-CHECKSUM-MISMATCH", detail));
    }
    // The record's diagnostics: the result's, then the rule-pack reference
    // diagnostics of exactly the references the manifest leaves out.
    let difference = diagnostics_difference(transient, &result, &manifest).map_err(|error| refused(MANIFEST, error))?;
    match difference {
        Some(DiagnosticsDifference::Producer) => return Err(refused(RESULT, "the record's diagnostics do not begin with the result's")),
        Some(DiagnosticsDifference::RulePacks) => {
            return Err(refused(
                MANIFEST,
                "the record's rule-pack reference diagnostics are not exactly those of the references the input manifest leaves out of active_rule_packs",
            ))
        }
        Some(DiagnosticsDifference::NotResolved) | None => {}
    }
    let rule_packs_not_resolved = difference == Some(DiagnosticsDifference::NotResolved);
    drop((result, result_reservation));
    Ok(payload_canonical.holding(manifest_reservation).map(|payload_canonical| CheckedSubmission {
        manifest,
        payload_canonical,
        payload_hash,
        producer_run_id: run_id.to_string(),
        result_hash_hex,
        rule_packs_not_resolved,
    }))
}

// ---------------------------------------------------------------------------
// Store operations.
// ---------------------------------------------------------------------------

/// One recorded run as the History panel lists it (columns, labels and the
/// verified record's status; never the result or the manifest).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct AnalysisRunSummary {
    pub(crate) project_id: String,
    pub(crate) recording_id: String,
    pub(crate) state_id: String,
    pub(crate) producer_run_id: String,
    pub(crate) created_at: String,
    pub(crate) run_record_hash: String,
    pub(crate) result_hash: String,
    #[serde(flatten)]
    pub(crate) labels: RunLabels,
    pub(crate) labels_updated_at: String,
    pub(crate) analysis_status: Vec<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct RecordRunOutcome {
    pub(crate) run: AnalysisRunSummary,
    pub(crate) state: ModelStateSummary,
    /// `existing`: the run is tied to the project's state with the solved
    /// model's hash; `created`: a new post-solve snapshot was written with it.
    pub(crate) state_outcome: &'static str,
    pub(crate) message: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct AnalysisRunRead {
    pub(crate) run: AnalysisRunSummary,
    pub(crate) state: ModelStateSummary,
    /// The recorded `analysis_run.v0.3` record (labels are not in it).
    pub(crate) record: Value,
    /// The received result, as stored (the submitted bytes).
    pub(crate) result_json: String,
    /// The input manifest's canonical JCS text.
    pub(crate) input_manifest_canonical: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct AnalysisRunDeleteReceipt {
    pub(crate) project_id: String,
    pub(crate) recording_id: String,
    pub(crate) deleted: bool,
    pub(crate) message: String,
}

/// A post-solve snapshot named in a Record Run request.
#[derive(Debug)]
pub(crate) struct NewStateRequest {
    pub(crate) labels: StateLabels,
    pub(crate) parent_state_id: Option<String>,
}

/// The `record_analysis_run` request.
#[derive(Debug)]
pub(crate) struct RecordRunRequest {
    pub(crate) project_id: String,
    /// The stored envelope hash value the session last opened or saved.
    pub(crate) expected_base: String,
    /// The solve's transient record.
    pub(crate) run_record: Value,
    /// The received result's JSON text (stored as given).
    pub(crate) result_json: String,
    /// The input manifest's canonical JCS text, kept by the session since the solve.
    pub(crate) input_manifest_canonical: String,
    /// The state the session expects the run to be tied to, when it named one.
    pub(crate) state_id: Option<String>,
    /// The post-solve snapshot to write when the project holds no state with
    /// the solved model's hash.
    pub(crate) new_state: Option<NewStateRequest>,
    pub(crate) run_labels: RunLabels,
}

impl RecordRunRequest {
    pub(crate) fn from_value(value: Value) -> Result<Self, command_ingress::ArgsError> {
        let mut fields = command_ingress::Fields::of(value, "struct RecordRunRequest")?;
        let run_labels = fields.required_value("run_labels")?;
        let new_state = match fields.value_or_null("new_state") {
            Value::Null => None,
            Value::Object(mut map) => {
                let labels = map.remove("labels").unwrap_or(Value::Null);
                let parent_state_id = match map.remove("parent_state_id") {
                    None | Some(Value::Null) => None,
                    Some(Value::String(parent)) => Some(parent),
                    Some(_) => return Err(serde::de::Error::custom("`new_state.parent_state_id` is not a string")),
                };
                Some(NewStateRequest {
                    labels: model_states::labels_from_value(labels).map_err(serde::de::Error::custom)?,
                    parent_state_id,
                })
            }
            _ => return Err(serde::de::Error::custom("`new_state` is not an object")),
        };
        Ok(Self {
            project_id: fields.required("project_id")?,
            expected_base: fields.required("expected_base")?,
            run_record: fields.required_value("run_record")?,
            result_json: fields.required("result_json")?,
            input_manifest_canonical: fields.required("input_manifest_canonical")?,
            state_id: fields.optional("state_id")?,
            new_state,
            run_labels: run_labels_from_value(run_labels).map_err(serde::de::Error::custom)?,
        })
    }
}

/// The clock, the post-solve snapshot's id token, and hooks between rows
/// (tests freeze the clock and inject failures; `stage` is `state` between the
/// snapshot's two rows, `run` between the run's two rows).
pub(crate) struct RunWriteHooks<'a> {
    pub(crate) now: &'a dyn Fn() -> Result<i64, String>,
    pub(crate) state_token: &'a dyn Fn() -> Result<String, String>,
    pub(crate) between_rows: &'a dyn Fn(&Transaction<'_>, &str) -> Result<(), String>,
}

pub(crate) const PRODUCTION_RUN_HOOKS: RunWriteHooks<'static> = RunWriteHooks {
    now: &now_unix_millis,
    state_token: &model_states::random_token,
    between_rows: &|_, _| Ok(()),
};

/// Decodes a run's labels row: each stored list is prescanned (invalid JSON is
/// refused before it is parsed) and its parse reserved before parsing; the
/// returned reservation holds the decoded labels.
fn decode_run_labels(
    recording_id: &str,
    name: String,
    tags: &str,
    notes: &str,
    assumptions: &str,
    memory: &dyn ReadMemory,
) -> Result<Metered<RunLabels>, String> {
    let mut price = 0u64;
    for text in [tags, notes, assumptions] {
        price = price.saturating_add(
            stored_read::parse_bytes(text)
                .map_err(|detail| integrity_of(recording_id, format_args!("a stored label list: {detail}")))?
                .saturating_mul(2),
        );
    }
    let reservation = memory.reserve(&format!("recorded run `{recording_id}`'s labels", recording_id = stored_read::shown(&recording_id)), price)?;
    let parse = |text: &str| {
        serde_json::from_str::<Value>(text)
            .map_err(|error| format!("ANALYSIS-RUN-INTEGRITY: a stored run label is not JSON ({error}).", error = stored_read::json_error(&error)))
    };
    let strings = |value: Value| -> Result<Vec<String>, String> {
        serde_json::from_value(value)
            .map_err(|error| format!("ANALYSIS-RUN-INTEGRITY: a stored run label list is malformed ({error}).", error = stored_read::json_error(&error)))
    };
    let labels = RunLabels {
        run_name: name,
        tags: strings(parse(tags)?)?,
        notes: strings(parse(notes)?)?,
        assumptions: serde_json::from_value(parse(assumptions)?)
            .map_err(|error| format!("ANALYSIS-RUN-INTEGRITY: the stored run assumptions are malformed ({error}).", error = stored_read::json_error(&error)))?,
    };
    Ok(Metered::new(labels).holding(reservation))
}

fn integrity_of(recording_id: &str, detail: impl std::fmt::Display) -> String {
    format!("ANALYSIS-RUN-INTEGRITY: recorded run `{}`: {detail}; it is listed as unreadable and is not read.", stored_read::shown(recording_id))
}

/// The row's identity columns, which the record duplicates.
struct RunColumns<'a> {
    project_id: &'a str,
    recording_id: &'a str,
    state_id: &'a str,
    producer_run_id: &'a str,
    created_at: &'a str,
    run_record_hash: &'a str,
    result_hash: &'a str,
}

/// Parses the stored record, checks its hash against the `run_record_hash`
/// column and its own checksum, and checks every identity column the record
/// duplicates against it: the record hash, the recording id (`runrec:` and the
/// hash's first 32 hex), the state id (`model_state_ref`), the producer run id,
/// `created_at`, the result hash (`received_result`) and the project (the
/// record's unit-system ref names the project and the manifest hash). Any
/// difference is `ANALYSIS-RUN-INTEGRITY`, naming the run.
fn verified_run_record(columns: &RunColumns<'_>, record_json: &str, memory: &dyn ReadMemory) -> Result<Value, String> {
    let id = columns.recording_id;
    let record: Value = serde_json::from_str(record_json)
        .map_err(|error| integrity_of(id, format_args!("the stored record is not JSON ({error})", error = stored_read::json_error(&error))))?;
    let recomputed = run_record_hash_within(&record, memory, &format!("recorded run `{}`'s record hash", stored_read::shown(id))).map_err(|error| {
        if error.starts_with(stored_read::STORE_READ_MEMORY_UNAVAILABLE) { error } else { integrity_of(id, error) }
    })?;
    let null = Value::Null;
    let run = record.get("analysis_run").unwrap_or(&null);
    let text = |pointer: &str| run.pointer(pointer).and_then(Value::as_str);
    let claim = single_hash(&record, "analysis_run_record");
    let received = single_hash(&record, "received_result");
    let manifest_hash = single_manifest_hash(&record)
        .and_then(|hash| hash.get("value"))
        .and_then(Value::as_str)
        .unwrap_or("");
    let run_id = text("/run_id").unwrap_or("");
    let checks: [(&str, bool); 9] = [
        (
            "record hash",
            recomputed == columns.run_record_hash
                && claim.and_then(|hash| hash.get("value")).and_then(Value::as_str) == Some(recomputed.as_str()),
        ),
        ("recording id", columns.recording_id == recording_id_of(&recomputed)),
        (
            "state id",
            run.get("model_state_ref") == Some(&reference("ModelState", columns.state_id)),
        ),
        ("producer run id", run_id == columns.producer_run_id),
        ("created_at", text("/created_at") == Some(columns.created_at)),
        (
            "result hash",
            received.and_then(|hash| hash.get("value")).and_then(Value::as_str) == Some(columns.result_hash),
        ),
        (
            "project",
            text("/unit_system_ref/ref") == Some(format!("unit-system:{}:{manifest_hash}", columns.project_id).as_str()),
        ),
        (
            "schema version",
            record.get("schema_version").and_then(Value::as_str) == Some("0.3.0"),
        ),
        // The record digest leaves its own checksum out, so the whole inventory
        // must be exactly the descriptors the verified fields give, and the
        // manifest's ref, checksum, settings ref and unit-system ref those its
        // project and hash give.
        (
            "checksum inventory",
            run.get("hashes") == Some(&expected_hashes(run_id, &recomputed, columns.result_hash))
                && first_difference(&record, &manifest_hash_members(columns.project_id, manifest_hash)).is_none(),
        ),
    ];
    if let Some((name, _)) = checks.iter().find(|(_, holds)| !holds) {
        return Err(integrity_of(id, format_args!("the row's {name} differs from its verified record")));
    }
    Ok(record)
}

const SUMMARY_COLUMNS: &str = "r.project_id, r.recording_id, r.state_id, r.producer_run_id, r.created_at, r.run_record_hash, r.result_hash,
    l.run_name, l.tags_json, l.notes_json, l.assumptions_json, l.updated_at";

type SummaryRow = (
    String,
    String,
    String,
    String,
    String,
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

/// A verified run: its summary (holding its decoded labels' reservation) and
/// its record (holding its parse's), metered apart so a list drops each record
/// with its reservation once it is verified.
type VerifiedRun = (Metered<AnalysisRunSummary>, Metered<Value>);

fn summary_from_row(connection: &Connection, row: SummaryRow, memory: &dyn ReadMemory) -> Result<VerifiedRun, String> {
    let (project_id, recording_id, state_id, producer_run_id, created_at, run_record_hash, result_hash, name, tags, notes, assumptions, updated_at) =
        row;
    let (Some(name), Some(tags), Some(notes), Some(assumptions), Some(updated_at)) = (name, tags, notes, assumptions, updated_at)
    else {
        return Err(format!("ANALYSIS-RUN-INTEGRITY: recorded run `{recording_id}` has no labels row.", recording_id = stored_read::shown(&recording_id)));
    };
    let record_json: String = connection
        .query_row(
            "SELECT run_record_json FROM analysis_runs WHERE project_id = ?1 AND recording_id = ?2",
            params![project_id, recording_id],
            |row| row.get(0),
        )
        // A record of the wrong type or not UTF-8 is the row's own failure (W-12).
        .map_err(|error| stored_read::column_fetch_error(error, |detail| integrity_of(&recording_id, detail)))?;
    // The record's parse and its hash projection (a clone), priced before
    // parsing and held while the record is; the hash's texts and checked parse
    // are reserved at their exact lengths when they are made. A record the
    // prescan refuses is refused before it is parsed.
    let record_parse = stored_read::parse_bytes(&record_json)
        .map_err(|detail| integrity_of(&recording_id, format_args!("the stored record: {detail}")))?;
    let record_reservation = memory.reserve(&format!("recorded run `{recording_id}`'s record", recording_id = stored_read::shown(&recording_id)), record_parse.saturating_mul(2))?;
    let record = verified_run_record(
        &RunColumns {
            project_id: &project_id,
            recording_id: &recording_id,
            state_id: &state_id,
            producer_run_id: &producer_run_id,
            created_at: &created_at,
            run_record_hash: &run_record_hash,
            result_hash: &result_hash,
        },
        &record_json,
        memory,
    )?;
    drop(record_json);
    // The status list outlives the record: its retained copy is reserved (repair r6).
    let status = stored_read::bounded_status(record.pointer("/analysis_run/analysis_status")).map_err(|detail| integrity_of(&recording_id, detail))?;
    let status_reservation = memory.reserve(
        &format!("recorded run `{}`'s analysis status", stored_read::shown(&recording_id)),
        stored_read::status_bytes(&status),
    )?;
    let analysis_status = serde_json::from_value(status)
        .map_err(|error| integrity_of(&recording_id, format_args!("its analysis status is malformed ({error})", error = stored_read::json_error(&error))))?;
    let labels = decode_run_labels(&recording_id, name, &tags, &notes, &assumptions, memory)?;
    drop((tags, notes, assumptions));
    Ok((
        labels.map(|labels| AnalysisRunSummary {
            labels,
            analysis_status,
            project_id,
            recording_id,
            state_id,
            producer_run_id,
            created_at,
            run_record_hash,
            result_hash,
            labels_updated_at: updated_at,
        })
        .holding(status_reservation),
        Metered::new(record).holding(record_reservation),
    ))
}

/// The identity and time columns a run summary loads, each with the bytes the
/// store keeps in it (repair r5).
pub(crate) const RUN_IDENTITY: [stored_read::BoundedColumn; 8] = [
    // The app bounds neither a project id nor a producer's run id: measured
    // and priced, never refused by size (repair r6).
    ("r.project_id", "project_id", stored_read::MEASURED_ONLY),
    ("r.recording_id", "recording_id", stored_read::MAX_ID_BYTES),
    ("r.state_id", "state_id", stored_read::MAX_ID_BYTES),
    ("r.producer_run_id", "producer_run_id", stored_read::MEASURED_ONLY),
    ("r.created_at", "created_at", stored_read::MAX_TIME_BYTES),
    ("r.run_record_hash", "run_record_hash", stored_read::MAX_HASH_BYTES),
    ("r.result_hash", "result_hash", stored_read::MAX_HASH_BYTES),
    ("l.updated_at", "labels_updated_at", stored_read::MAX_TIME_BYTES),
];

/// The byte lengths a run summary loads, read before any of it is loaded: the
/// record, the labels row, and the identity and time columns.
fn run_summary_lengths_sql() -> String {
    format!(
        "octet_length(r.run_record_json) AS record_length,
         COALESCE(octet_length(l.run_name) + octet_length(l.tags_json) + octet_length(l.notes_json) + octet_length(l.assumptions_json) + octet_length(l.updated_at), 0) AS labels_length,
         {} AS identity_length",
        stored_read::columns_bytes(&RUN_IDENTITY)
    )
}

/// Refuses, before anything of it is loaded, the first run row of
/// `project_id` (or the run `recording_id`) whose identity or time column holds
/// more than its bound (repair r5): an integrity refusal naming the row by the
/// head of its id.
fn refuse_oversized_runs(connection: &Connection, project_id: &str, recording_id: Option<&str>) -> Result<(), String> {
    let oversized = stored_read::oversized_column(&RUN_IDENTITY);
    let found: Option<(Vec<u8>, i64, String)> = connection
        .query_row(
            &format!(
                "SELECT {}, {oversized} FROM analysis_runs r LEFT JOIN analysis_run_labels l ON l.project_id = r.project_id AND l.recording_id = r.recording_id
                 WHERE r.project_id = ?1 AND (?2 IS NULL OR r.recording_id = ?2) AND {oversized} IS NOT NULL LIMIT 1",
                stored_read::shown_column("r.recording_id")
            ),
            params![project_id, recording_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .optional()
        .map_err(sql)?;
    match found {
        None => Ok(()),
        Some((head, length, column)) => Err(oversized_run_refusal(&stored_read::shown_head(&head, length), &column)),
    }
}

/// Loading run summaries: the largest record (records are verified one at a
/// time), every labels row, every row's identity and time columns as measured
/// (each within its bound, [`refuse_oversized_runs`]), and each row's
/// structure. (The record's hash texts are reserved at their exact lengths when
/// they are built.)
fn run_summary_load_bytes(record: u64, labels: u64, identity: u64, rows: u64) -> u64 {
    stored_read::total(&[
        stored_read::loaded_text_bytes(record),
        stored_read::loaded_text_bytes(labels),
        stored_read::string_list_bytes(rows.saturating_mul(RUN_IDENTITY.len() as u64), identity),
        rows.saturating_mul(1024),
    ])
}

/// A recorded run's summary and record within `memory`, on `transaction` (a
/// read transaction or the writer's, so the lengths measured are those
/// loaded): the summary's lengths are read first and its loading reserved
/// before any of it is loaded; the record's and the labels' parses are reserved
/// before parsing. The returned run holds its reservations.
pub(crate) fn find_run_with(
    connection: &Connection,
    project_id: &str,
    recording_id: &str,
    memory: &dyn ReadMemory,
) -> Result<Option<Metered<(AnalysisRunSummary, Value)>>, String> {
    let lengths: Option<(Option<i64>, Option<i64>, Option<i64>)> = connection
        .query_row(
            &format!(
                "SELECT {} FROM analysis_runs r LEFT JOIN analysis_run_labels l
                 ON l.project_id = r.project_id AND l.recording_id = r.recording_id WHERE r.project_id = ?1 AND r.recording_id = ?2",
                run_summary_lengths_sql()
            ),
            params![project_id, recording_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .optional()
        .map_err(sql)?;
    let Some((record, labels, identity)) = lengths else { return Ok(None) };
    refuse_oversized_runs(connection, project_id, Some(recording_id))?;
    let length = |value: Option<i64>| u64::try_from(value.unwrap_or(0)).unwrap_or(0);
    let load = memory.reserve(
        &format!("recorded run `{}`", stored_read::shown(recording_id)),
        stored_read::total(&[stored_read::READ_OVERHEAD_BYTES, run_summary_load_bytes(length(record), length(labels), length(identity), 1)]),
    )?;
    let row = connection
        .query_row(
            &format!(
                "SELECT {SUMMARY_COLUMNS} FROM analysis_runs r LEFT JOIN analysis_run_labels l
                 ON l.project_id = r.project_id AND l.recording_id = r.recording_id WHERE r.project_id = ?1 AND r.recording_id = ?2"
            ),
            params![project_id, recording_id],
            summary_row,
        )
        .optional()
        .map_err(|error| {
            // A stored value of the wrong type or not UTF-8 is the row's own failure (W-12).
            if stored_read::is_row_value_error(&error) {
                let detail = sql(error);
                integrity_of(recording_id, format_args!("a stored column cannot be read ({detail})"))
            } else {
                sql(error)
            }
        })?;
    let Some(row) = row else { return Ok(None) };
    let (summary, record) = summary_from_row(connection, row, memory)?;
    Ok(Some(summary.join(record, |summary, record| (summary, record)).holding(load)))
}

/// One entry of a project's recorded-run list: a verified run's summary, or a
/// row that cannot be read, shown only by its id and its named refusal (W-12;
/// DEL-14-01 Design §3). One unreadable row never hides the others.
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub(crate) enum ListedRun {
    Readable(AnalysisRunSummary),
    Unreadable { unreadable: stored_read::UnreadableRow },
}

/// The project's recorded runs, newest first, unmetered, on its own read
/// transaction (tests; the command uses [`list_runs_in_store_with`]): the
/// summaries when every row is readable, else the first unreadable row's
/// refusal (the refusal its list entry carries). [`list_run_entries`] gives
/// the entries themselves.
#[cfg(test)]
pub(crate) fn list_runs(connection: &Connection, project_id: &str) -> Result<Vec<AnalysisRunSummary>, String> {
    list_run_entries(connection, project_id)?
        .into_iter()
        .map(|entry| match entry {
            ListedRun::Readable(summary) => Ok(summary),
            ListedRun::Unreadable { unreadable } => Err(unreadable.refusal),
        })
        .collect()
}

/// The project's recorded-run list entries, unmetered (tests).
#[cfg(test)]
pub(crate) fn list_run_entries(connection: &Connection, project_id: &str) -> Result<Vec<ListedRun>, String> {
    let transaction = connection.unchecked_transaction().map_err(sql)?;
    list_runs_with(&transaction, project_id, &UnmeteredReadMemory)?.into_unreserved()
}

/// [`list_runs_with`] on its own read transaction of the store at `path`.
pub(crate) fn list_runs_in_store_with(
    path: &Path,
    project_id: &str,
    memory: &dyn ReadMemory,
) -> Result<Metered<Vec<ListedRun>>, String> {
    let mut connection = stored_read::open_store(path)?;
    let transaction = stored_read::read_transaction(&mut connection)?;
    let _sqlite = stored_read::admit_sqlite_working_set(&transaction, memory)?;
    list_runs_with(&transaction, project_id, memory)
}

/// Why a run list shows a decoded row as unreadable: a stored value of the
/// wrong type or not UTF-8 (its detail; the refusal names the row by its id's
/// head), or the row's own integrity refusal.
enum RunRowRefusal {
    Column(String),
    Integrity(String),
}

impl RunRowRefusal {
    fn refusal(self, id: &str) -> String {
        match self {
            Self::Column(detail) => integrity_of(id, format_args!("a stored column cannot be read ({detail})")),
            Self::Integrity(refusal) => refusal,
        }
    }
}

/// The run list's check of one decoded row (W-12, DEL-14-01 Design §3), the
/// same for the list and for a delete by handle's re-check: its verified run,
/// or why the list shows it unreadable. Memory refusals and database errors
/// are the caller's (`Err`).
fn checked_run_row(connection: &Connection, row: rusqlite::Result<SummaryRow>, memory: &dyn ReadMemory) -> Result<Result<VerifiedRun, RunRowRefusal>, String> {
    match row {
        Err(error) if stored_read::is_row_value_error(&error) => Ok(Err(RunRowRefusal::Column(sql(error)))),
        Err(error) => Err(sql(error)),
        Ok(row) => match summary_from_row(connection, row, memory) {
            Ok(verified) => Ok(Ok(verified)),
            Err(error) if error.starts_with("ANALYSIS-RUN-INTEGRITY") => Ok(Err(RunRowRefusal::Integrity(error))),
            Err(error) => Err(error),
        },
    }
}

/// The refusal of a run row whose `column` holds more than its bound.
fn oversized_run_refusal(id: &str, column: &str) -> String {
    integrity_of(id, format_args!("its {column} is longer than the {} bytes kept", stored_read::column_bound(&RUN_IDENTITY, column)))
}

/// The unreadable entry of the run row `rowid`: its id's head, `refusal(id)`,
/// and its delete handle ([`stored_read::row_handle`]); its caller holds its
/// reservation ([`stored_read::UNREADABLE_ROW_BYTES`]), made before this reads
/// anything. Only the id's head is loaded; the id is hashed in chunks.
fn unreadable_run_row(
    connection: &Connection,
    rowid: i64,
    refusal: impl FnOnce(&str) -> String,
    memory: &dyn ReadMemory,
) -> Result<stored_read::UnreadableRow, String> {
    let (head, head_length): (Vec<u8>, i64) = connection
        .query_row(
            &format!("SELECT {} FROM analysis_runs WHERE rowid = ?1", stored_read::shown_column("recording_id")),
            params![rowid],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(sql)?;
    let id = stored_read::shown_head(&head, head_length);
    let handle = stored_read::row_handle(connection, "analysis_runs", "recording_id", RUN_ROW_HANDLE, rowid, memory)?;
    let refusal = refusal(&id);
    Ok(stored_read::UnreadableRow::new(&id, refusal, handle, 0))
}

/// [`unreadable_run_row`] within its own reservation, made first.
pub(crate) fn unreadable_run_entry(
    connection: &Connection,
    rowid: i64,
    refusal: impl FnOnce(&str) -> String,
    memory: &dyn ReadMemory,
) -> Result<Metered<stored_read::UnreadableRow>, String> {
    let reservation = memory.reserve("an unreadable recorded run's list entry", stored_read::UNREADABLE_ROW_BYTES)?;
    Ok(Metered::new(unreadable_run_row(connection, rowid, refusal, memory)?).holding(reservation))
}

/// The kind of a recorded-run row's delete handle.
pub(crate) const RUN_ROW_HANDLE: &str = "run-row";

/// The run rows of `project_id` whose identity or time column holds more than
/// its bound, as unreadable entries (repair r5's refusal, now per row). They
/// are counted first and every entry, with its list slot, is reserved at its
/// bound before any is collected (W-12): nothing per row is allocated before
/// the list is admitted. Only each id's head and the column named are loaded;
/// each id is hashed in chunks.
fn oversized_run_entries(connection: &Connection, project_id: &str, memory: &dyn ReadMemory) -> Result<Metered<Vec<stored_read::UnreadableRow>>, String> {
    let oversized = stored_read::oversized_column(&RUN_IDENTITY);
    let from = format!(
        "FROM analysis_runs r LEFT JOIN analysis_run_labels l ON l.project_id = r.project_id AND l.recording_id = r.recording_id
         WHERE r.project_id = ?1 AND {oversized} IS NOT NULL"
    );
    let count: i64 = connection.query_row(&format!("SELECT COUNT(*) {from}"), params![project_id], |row| row.get(0)).map_err(sql)?;
    let count = usize::try_from(count).unwrap_or(0);
    let admitted = memory.reserve(
        &format!("the {count} oversized recorded-run rows' list entries of project `{}`", stored_read::shown(project_id)),
        (count as u64).saturating_mul(stored_read::UNREADABLE_ROW_BYTES.saturating_add(std::mem::size_of::<ListedRun>() as u64)),
    )?;
    let mut entries = Vec::with_capacity(count);
    let mut statement = connection.prepare(&format!("SELECT r.rowid, {oversized} {from} ORDER BY r.rowid")).map_err(sql)?;
    let mut rows = statement.query(params![project_id]).map_err(sql)?;
    while entries.len() < count {
        let Some(row) = rows.next().map_err(sql)? else { break };
        let rowid: i64 = row.get(0).map_err(sql)?;
        let column: String = row.get(1).map_err(sql)?;
        entries.push(unreadable_run_row(connection, rowid, |id| oversized_run_refusal(id, &column), memory)?);
    }
    Ok(Metered::new(entries).holding(admitted))
}

/// [`list_run_entries`] within `memory`: before anything is loaded, the
/// summaries' lengths are read and their loading reserved (the largest record,
/// as records are loaded, verified and dropped one at a time, and every labels
/// row, which the list keeps); each record's parse is reserved after it is
/// loaded and released once it is verified, and each row's decoded labels are
/// held with the list until its response is serialized. A row that cannot be
/// read (an oversized column, a stored value of the wrong type or not UTF-8, a
/// record or labels row that does not verify) is listed by its id and refusal
/// only, each such entry reserved at its bound
/// ([`stored_read::UNREADABLE_ROW_BYTES`]) before it is made (the oversized
/// rows counted and reserved together first); its unverified fields are not
/// returned. Memory refusals and SQLite errors still refuse the list.
pub(crate) fn list_runs_with(
    transaction: &Transaction<'_>,
    project_id: &str,
    memory: &dyn ReadMemory,
) -> Result<Metered<Vec<ListedRun>>, String> {
    let connection: &Connection = transaction;
    let oversized = stored_read::oversized_column(&RUN_IDENTITY);
    let (record, labels, identity, rows): (Option<i64>, Option<i64>, Option<i64>, i64) = connection
        .query_row(
            &format!(
                "SELECT MAX(record_length), SUM(labels_length), SUM(identity_length), COUNT(*) FROM (SELECT {}
                 FROM analysis_runs r LEFT JOIN analysis_run_labels l ON l.project_id = r.project_id AND l.recording_id = r.recording_id
                 WHERE r.project_id = ?1 AND {oversized} IS NULL)",
                run_summary_lengths_sql()
            ),
            params![project_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        )
        .map_err(sql)?;
    let oversized_entries = oversized_run_entries(connection, project_id, memory)?;
    let length = |value: Option<i64>| u64::try_from(value.unwrap_or(0)).unwrap_or(0);
    let load = memory.reserve(
        &format!("the recorded runs of project `{}`", stored_read::shown(project_id)),
        stored_read::total(&[
            stored_read::READ_OVERHEAD_BYTES,
            run_summary_load_bytes(length(record), length(labels), length(identity), length(Some(rows))),
        ]),
    )?;
    // Each row decoded on its own: a stored value of the wrong type or not
    // UTF-8 makes that row unreadable, not the list (W-12).
    let rows: Vec<(i64, rusqlite::Result<SummaryRow>)> = {
        let mut statement = connection
            .prepare(&format!(
                "SELECT {SUMMARY_COLUMNS}, r.rowid FROM analysis_runs r LEFT JOIN analysis_run_labels l
                 ON l.project_id = r.project_id AND l.recording_id = r.recording_id WHERE r.project_id = ?1 AND {oversized} IS NULL
                 ORDER BY r.created_at DESC, r.recording_id"
            ))
            .map_err(sql)?;
        let mut cursor = statement.query(params![project_id]).map_err(sql)?;
        let mut rows = Vec::new();
        while let Some(row) = cursor.next().map_err(sql)? {
            let rowid: i64 = row.get(12).map_err(sql)?;
            rows.push((rowid, summary_row(row)));
        }
        rows
    };
    let mut listed = Metered::new(Vec::with_capacity(rows.len() + oversized_entries.get().len())).holding(load);
    for (rowid, row) in rows {
        let entry = match checked_run_row(connection, row, memory)? {
            Ok((summary, record)) => {
                drop(record);
                summary.map(ListedRun::Readable)
            }
            Err(refusal) => unreadable_run_entry(connection, rowid, |id| refusal.refusal(id), memory)?.map(|unreadable| ListedRun::Unreadable { unreadable }),
        };
        listed = listed.join(entry, |mut list, entry| {
            list.push(entry);
            list
        });
    }
    Ok(listed.join(oversized_entries, |mut list, entries| {
        list.extend(entries.into_iter().map(|unreadable| ListedRun::Unreadable { unreadable }));
        list
    }))
}

/// Whether the run row `rowid` of `project_id` is still shown unreadable by
/// the list's own check (a delete by handle's re-check, inside its write
/// transaction; W-12, DEL-14-01 Design §3): an oversized column, or
/// [`checked_run_row`] refusing it. The row is loaded only as the list loads
/// it: its oversized columns never, the rest within the summary's reservation.
fn run_row_unreadable(connection: &Connection, project_id: &str, rowid: i64, memory: &dyn ReadMemory) -> Result<bool, String> {
    let oversized = stored_read::oversized_column(&RUN_IDENTITY);
    let joined = "FROM analysis_runs r LEFT JOIN analysis_run_labels l ON l.project_id = r.project_id AND l.recording_id = r.recording_id
         WHERE r.rowid = ?1 AND r.project_id = ?2";
    let column: Option<String> = connection.query_row(&format!("SELECT {oversized} {joined}"), params![rowid, project_id], |row| row.get(0)).map_err(sql)?;
    if column.is_some() {
        return Ok(true);
    }
    let (record, labels, identity): (Option<i64>, Option<i64>, Option<i64>) = connection
        .query_row(&format!("SELECT {} {joined}", run_summary_lengths_sql()), params![rowid, project_id], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
        .map_err(sql)?;
    let length = |value: Option<i64>| u64::try_from(value.unwrap_or(0)).unwrap_or(0);
    let _load = memory.reserve(
        "an unreadable recorded run's re-check",
        stored_read::total(&[stored_read::READ_OVERHEAD_BYTES, run_summary_load_bytes(length(record), length(labels), length(identity), 1)]),
    )?;
    let row = connection.query_row(&format!("SELECT {SUMMARY_COLUMNS} {joined}"), params![rowid, project_id], |row| Ok(summary_row(row))).map_err(sql)?;
    Ok(checked_run_row(connection, row, memory)?.is_err())
}

/// How many recorded runs the project id holds (the replacement prompt's count).
pub(crate) fn recorded_run_count(connection: &Connection, project_id: &str) -> Result<i64, String> {
    connection
        .query_row("SELECT COUNT(*) FROM analysis_runs WHERE project_id = ?1", params![project_id], |row| row.get(0))
        .map_err(sql)
}

/// `"<name>" (<recording id>)` as a message quotes them ([`stored_read::shown`]:
/// at most 120 characters each), so a message naming a run is small.
pub(crate) fn run_description(run: &AnalysisRunSummary) -> String {
    format!("\"{}\" ({})", stored_read::shown(&run.labels.run_name), stored_read::shown(&run.recording_id))
}

/// The recording time: the later of `now` and one millisecond after the
/// project's latest recorded `created_at` (read inside the write transaction).
fn allocate_created_at(transaction: &Transaction<'_>, project_id: &str, now: i64) -> Result<String, String> {
    // Every run's created_at within its bound before the latest is loaded (repair r5).
    let oversized: Option<(Vec<u8>, i64)> = transaction
        .query_row(
            &format!(
                "SELECT {} FROM analysis_runs WHERE project_id = ?1 AND octet_length(created_at) > {} LIMIT 1",
                stored_read::shown_column("recording_id"),
                stored_read::MAX_TIME_BYTES
            ),
            params![project_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()
        .map_err(sql)?;
    if let Some((head, length)) = oversized {
        return Err(integrity_of(
            &stored_read::shown_head(&head, length),
            format_args!("its created_at is longer than the {} bytes kept", stored_read::MAX_TIME_BYTES),
        ));
    }
    let latest: Option<String> = transaction
        .query_row(
            "SELECT MAX(created_at) FROM analysis_runs WHERE project_id = ?1",
            params![project_id],
            |row| row.get(0),
        )
        .map_err(|error| stored_read::column_fetch_error(error, |detail| format!("ANALYSIS-RUN-INTEGRITY: a recorded run's created_at: {detail}.")))?;
    let after_latest = match latest {
        None => i64::MIN,
        Some(text) => parse_iso_millis(&text)
            .ok_or_else(|| format!("ANALYSIS-RUN-INTEGRITY: a recorded run's created_at `{text}` is not a recording time.", text = stored_read::shown(&text)))?
            .saturating_add(1),
    };
    Ok(utc_iso_millis(now.max(after_latest)))
}

fn write_run_labels(
    transaction: &Transaction<'_>,
    project_id: &str,
    recording_id: &str,
    labels: &RunLabels,
    updated_at: &str,
    insert: bool,
) -> Result<(), String> {
    let text = |result: serde_json::Result<String>| result.map_err(|error| stored_read::json_error(&error));
    let tags = text(serde_json::to_string(&labels.tags))?;
    let notes = text(serde_json::to_string(&labels.notes))?;
    let assumptions = text(serde_json::to_string(&labels.assumptions))?;
    let statement = if insert {
        "INSERT INTO analysis_run_labels (project_id, recording_id, run_name, tags_json, notes_json, assumptions_json, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"
    } else {
        "UPDATE analysis_run_labels SET run_name = ?3, tags_json = ?4, notes_json = ?5, assumptions_json = ?6, updated_at = ?7
         WHERE project_id = ?1 AND recording_id = ?2"
    };
    let changed = transaction
        .execute(
            statement,
            params![project_id, recording_id, labels.run_name, tags, notes, assumptions, updated_at],
        )
        .map_err(sql)?;
    if changed != 1 {
        return Err(format!("ANALYSIS-RUN-INTEGRITY: the labels of recorded run `{recording_id}` were not written.", recording_id = stored_read::shown(&recording_id)));
    }
    Ok(())
}

/// `record_analysis_run` on a given store: the binding checks, then (in one
/// `BEGIN IMMEDIATE` transaction with the project row's stale-window check) the
/// state (found by the solved model's hash, or a new post-solve snapshot and
/// its labels), the recording time, the recorded revision, the run row and its
/// labels row.
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn record_run_in_store(
    path: &Path,
    request: RecordRunRequest,
    hooks: &RunWriteHooks<'_>,
) -> Result<RecordRunOutcome, String> {
    record_run_in_store_with(path, request, hooks, &UnmeteredReadMemory)?.into_unreserved()
}

/// [`record_run_in_store_finishing`], keeping the outcome (tests).
pub(crate) fn record_run_in_store_with(
    path: &Path,
    request: RecordRunRequest,
    hooks: &RunWriteHooks<'_>,
    memory: &dyn ReadMemory,
) -> Result<Metered<RecordRunOutcome>, String> {
    record_run_in_store_finishing(path, request, hooks, memory, Ok)
}

/// [`record_run_in_store`] with its working memory reserved in `memory` before
/// it is used: the checks' parses and canonical texts ([`checked_submission`]),
/// the post-solve snapshot's checks and record, the recorded revision's hash
/// and serialized text, and every summary it reads (the state, the run). The
/// outcome, holding the run's and the state's summaries' reservations, goes to
/// `finish` (the command serializes it) before the transaction commits, so a
/// refusal there records nothing (repair r4). Residual: the transient record's
/// clones (its hash projection and the recorded revision) are working memory
/// within the request's argument lease.
pub(crate) fn record_run_in_store_finishing<R>(
    path: &Path,
    request: RecordRunRequest,
    hooks: &RunWriteHooks<'_>,
    memory: &dyn ReadMemory,
    finish: impl FnOnce(Metered<RecordRunOutcome>) -> Result<R, String>,
) -> Result<R, String> {
    let project_id = request.project_id.as_str();
    let submission = checked_submission(
        project_id,
        &request.run_record,
        &request.result_json,
        &request.input_manifest_canonical,
        memory,
    )?;
    let mut connection = stored_read::open_store(path)?;
    let transaction = stored_read::write_transaction(&mut connection)?;
    let _sqlite = stored_read::admit_sqlite_working_set(&transaction, memory)?;
    model_states::require_project_base(
        &transaction,
        project_id,
        &request.expected_base,
        format!(
            "ANALYSIS-RUN-PROJECT-MISSING: project `{}` has no stored local project; save the local project first. Nothing was recorded.",
            stored_read::shown(project_id)
        ),
        memory,
    )?;
    let (state, state_outcome) = match &request.state_id {
        Some(state_id) => {
            let state = find_summary_with(&transaction, project_id, state_id, memory)?.ok_or_else(|| {
                refused("ANALYSIS-RUN-STATE-NOT-FOUND", format_args!("project `{project_id}` has no saved state `{state_id}`", project_id = stored_read::shown(&project_id), state_id = stored_read::shown(&state_id)))
            })?;
            if state.get().model_hash != submission.get().payload_hash {
                return Err(refused(
                    "ANALYSIS-RUN-STATE-MISMATCH",
                    format_args!(
                        "the solved model hashes to {}, not state {}'s {}",
                        submission.get().payload_hash,
                        state_description(state.get()),
                        stored_read::shown(&state.get().model_hash)
                    ),
                ));
            }
            (state, "existing")
        }
        None => match model_states::state_with_model_hash(&transaction, project_id, &submission.get().payload_hash, memory)? {
            Some(state) => (state, "existing"),
            None => {
                let Some(new_state) = &request.new_state else {
                    return Err(refused(
                        "ANALYSIS-RUN-STATE-REQUIRED",
                        format_args!(
                            "project `{}` has no saved state of the solved model ({}); name a post-solve snapshot to record the run against",
                            stored_read::shown(project_id),
                            stored_read::shown(&submission.get().payload_hash)
                        ),
                    ));
                };
                // The snapshot keeps its own copy of the payload's canonical text.
                let copy = memory.reserve(
                    "the post-solve snapshot",
                    stored_read::exact_text_bytes(submission.get().payload_canonical.len() as u64),
                )?;
                let snapshot = model_states::checked_new_state(
                    project_id,
                    POST_SOLVE_SNAPSHOT,
                    submission.get().payload_canonical.clone(),
                    &submission.get().payload_hash,
                    memory,
                )?
                .holding(copy);
                let allocate = || Ok(((hooks.state_token)()?, (hooks.now)()?));
                let between_rows = |transaction: &Transaction<'_>| (hooks.between_rows)(transaction, "state");
                let state = insert_new_state(
                    &transaction,
                    project_id,
                    POST_SOLVE_SNAPSHOT,
                    new_state.parent_state_id.as_deref(),
                    snapshot.get(),
                    &new_state.labels,
                    &StateWriteHooks { allocate: &allocate, between_rows: &between_rows },
                    memory,
                )?;
                drop(snapshot);
                (state, "created")
            }
        },
    };
    // Every other check has passed (W-12: the named refusal never masks corruption).
    if submission.get().rule_packs_not_resolved {
        return Err(refused(RULE_PACKS_NOT_RESOLVED, RULE_PACKS_NOT_RESOLVED_DETAIL));
    }
    let created_at = allocate_created_at(&transaction, project_id, (hooks.now)()?)?;
    let state_id = state.get().state_id.clone();
    drop(state);
    let (record, record_hash) = recorded_revision(memory, &request.run_record, &state_id, &created_at, &submission.get().manifest)?;
    let recording_id = recording_id_of(&record_hash);
    let record_json = stored_read::json_text(memory, "the recorded revision", &record)?;
    drop(record);
    transaction
        .execute(
            "INSERT INTO analysis_runs (project_id, recording_id, state_id, producer_run_id, created_at, run_record_json, run_record_hash, input_manifest_json, result_json, result_hash)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                project_id,
                recording_id,
                state_id,
                submission.get().producer_run_id,
                created_at,
                record_json.get(),
                record_hash,
                request.input_manifest_canonical,
                request.result_json,
                submission.get().result_hash_hex
            ],
        )
        .map_err(sql)?;
    drop((record_json, submission));
    (hooks.between_rows)(&transaction, "run")?;
    write_run_labels(&transaction, project_id, &recording_id, &request.run_labels, &created_at, true)?;
    let run = find_run_with(&transaction, project_id, &recording_id, memory)?
        .ok_or_else(|| format!("ANALYSIS-RUN-INTEGRITY: recorded run `{recording_id}` was not written.", recording_id = stored_read::shown(&recording_id)))?
        .map(|(run, _record)| run);
    // The state as it now stands (its recorded runs include this one).
    let state = find_summary_with(&transaction, project_id, &state_id, memory)?
        .ok_or_else(|| format!("MODEL-STATE-INTEGRITY: state `{state_id}` vanished.", state_id = stored_read::shown(&state_id)))?;
    let unused = if state_outcome == "existing" && request.state_id.is_none() && request.new_state.is_some() {
        " The solved model was already saved as that state, so the post-solve snapshot name was not used."
    } else {
        ""
    };
    let finished = finish(run.join(state, |run, state| {
        let against = if state_outcome == "created" {
            format!("the new post-solve snapshot {}", state_description(&state))
        } else {
            format!("model state {}", state_description(&state))
        };
        let message = format!("Recorded run {} against {against}.{unused} {LABELS_DO_NOT_CHANGE_RUN}", run_description(&run));
        RecordRunOutcome { run, state, state_outcome, message }
    }))?;
    transaction.commit().map_err(sql)?;
    Ok(finished)
}

/// `read_analysis_run` on a given store: the record, the stored result and the
/// stored manifest, after the bytes are checked against the record and the
/// state: the record (its hash, checksum descriptors and identity columns), the
/// result's checked hash and its rows' checksums, the manifest's canonical form,
/// its hash against the record, its project, every record member it fixes
/// (solver, settings, units, load basis, rule-pack and library references), and
/// its model payload's hash against the state's model hash.
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn read_run_in_store(path: &Path, project_id: &str, recording_id: &str) -> Result<AnalysisRunRead, String> {
    read_run_in_store_with(path, project_id, recording_id, &UnmeteredReadMemory)?.into_unreserved()
}

/// [`read_run_in_store`] within `memory` (`stored_read`), in one read
/// transaction: the result's and manifest's byte lengths are read first,
/// without loading them, and their loading is reserved before either is loaded
/// (the run's and the state's summaries reserve their own, labels included);
/// each parse is reserved after its text is loaded and before it is parsed, a
/// stored text the prescan refuses is refused before it is parsed, and every
/// canonical text is reserved at its exact length before it is built. The
/// returned read holds its reservations until its response is serialized
/// ([`stored_read::respond_metered`]). A refusal is
/// `STORE-READ-MEMORY-UNAVAILABLE` and loads nothing more.
pub(crate) fn read_run_in_store_with(
    path: &Path,
    project_id: &str,
    recording_id: &str,
    memory: &dyn ReadMemory,
) -> Result<Metered<AnalysisRunRead>, String> {
    let mut connection = stored_read::open_store(path)?;
    let transaction = stored_read::read_transaction(&mut connection)?;
    let _sqlite = stored_read::admit_sqlite_working_set(&transaction, memory)?;
    let not_found = || format!("ANALYSIS-RUN-NOT-FOUND: project `{project_id}` has no recorded run `{recording_id}`.", project_id = stored_read::shown(&project_id), recording_id = stored_read::shown(&recording_id));
    let lengths: (i64, i64) = transaction
        .query_row(
            "SELECT octet_length(r.result_json), octet_length(r.input_manifest_json) FROM analysis_runs r WHERE r.project_id = ?1 AND r.recording_id = ?2",
            params![project_id, recording_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()
        .map_err(sql)?
        .ok_or_else(not_found)?;
    let length = |value: i64| u64::try_from(value).unwrap_or(0);
    let (result_length, manifest_length) = (length(lengths.0), length(lengths.1));
    let what = format!("recorded run `{recording_id}`", recording_id = stored_read::shown(&recording_id));
    // Loaded here: the result and the manifest (the summaries reserve their
    // own; the canonical texts are reserved when they are built).
    let load = memory.reserve(
        &what,
        stored_read::total(&[
            stored_read::READ_OVERHEAD_BYTES,
            stored_read::loaded_text_bytes(result_length),
            stored_read::loaded_text_bytes(manifest_length),
        ]),
    )?;
    let mut read = Metered::new(()).holding(load);
    let found = find_run_with(&transaction, project_id, recording_id, memory)?.ok_or_else(not_found)?;
    let (run, record) = found.get();
    let (result_json, manifest_text): (String, String) = transaction
        .query_row(
            "SELECT result_json, input_manifest_json FROM analysis_runs WHERE project_id = ?1 AND recording_id = ?2",
            params![project_id, recording_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|error| stored_read::column_fetch_error(error, |detail| integrity_of(recording_id, detail)))?;
    // Parsed: the result (its checked parse, made by the canonicalizer, is
    // reserved with its canonical text) and the manifest. A stored text the
    // prescan refuses is refused before it is parsed.
    let refuse = |field: &str| integrity_of(recording_id, format_args!("the stored {field} differs from its verified record"));
    let memory_or = |field: &'static str| {
        move |error: String| if error.starts_with(stored_read::STORE_READ_MEMORY_UNAVAILABLE) { error } else { refuse(field) }
    };
    let result_parse = stored_read::parse_bytes(&result_json)
        .map_err(|detail| integrity_of(recording_id, format_args!("the stored result: {detail}")))?;
    let manifest_parse = stored_read::parse_bytes(&manifest_text)
        .map_err(|detail| integrity_of(recording_id, format_args!("the stored input manifest: {detail}")))?;
    let parse = memory.reserve(&what, stored_read::total(&[result_parse, manifest_parse]))?;
    let result: Value = serde_json::from_str(&result_json).map_err(|_| refuse("result"))?;
    let result_hash = stored_read::checked_sha256_text(memory, &format!("{what}'s result"), &result_json, &result).map_err(memory_or("result"))?;
    if result_hash != run.result_hash {
        return Err(refuse("result"));
    }
    if result_rows_difference(record, &result, memory).map_err(memory_or("result"))?.is_some() {
        return Err(refuse("result's row checksums"));
    }
    let manifest: Value = serde_json::from_str(&manifest_text).map_err(|_| refuse("input manifest"))?;
    let manifest_hash = sha256_hex(&manifest_text);
    let claimed_manifest_hash = single_manifest_hash(record).and_then(|hash| hash.get("value")).and_then(Value::as_str);
    if !stored_read::is_jcs_text(memory, &format!("{what}'s canonical input manifest"), &manifest, &manifest_text)?
        || claimed_manifest_hash != Some(manifest_hash.as_str())
    {
        return Err(refuse("input manifest"));
    }
    if manifest.pointer("/model_basis/model_ref").and_then(Value::as_str) != Some(project_id) {
        return Err(refuse("input manifest's project"));
    }
    let members = manifest_members(&manifest, &manifest_hash, true).map_err(|_| refuse("input manifest"))?;
    if let Some(name) = first_difference(record, &members) {
        return Err(integrity_of(recording_id, format_args!("the record's {name} differs from its stored input manifest")));
    }
    // Both parses are within `parse`'s reservation.
    let difference = diagnostics_difference(record, &result, &manifest).map_err(|_| refuse("input manifest"))?;
    match difference {
        Some(DiagnosticsDifference::Producer) => return Err(integrity_of(recording_id, "the record's diagnostics do not begin with its stored result's")),
        Some(DiagnosticsDifference::RulePacks) => {
            return Err(integrity_of(
                recording_id,
                "the record's rule-pack reference diagnostics differ from the references its stored input manifest leaves out",
            ))
        }
        Some(DiagnosticsDifference::NotResolved) | None => {}
    }
    // Refused by name only once every other check (below: the linked state and
    // its model hash) has passed, so corruption keeps its integrity refusal (W-12).
    let rule_packs_not_resolved = difference == Some(DiagnosticsDifference::NotResolved);
    drop(result);
    let state = model_states::find_summary_with(&transaction, project_id, &run.state_id, memory)?.ok_or_else(|| {
        integrity_of(recording_id, format_args!("its state `{}` is not saved", stored_read::shown(&run.state_id)))
    })?;
    let payload_hash = match manifest.pointer("/model_basis/model_payload") {
        Some(payload) => Some(format!("sha256:{}", stored_read::jcs_sha256(memory, &format!("{what}'s model payload"), payload)?)),
        None => None,
    };
    if payload_hash.as_deref() != Some(state.get().model_hash.as_str()) {
        return Err(integrity_of(
            recording_id,
            format_args!("the stored input manifest's model payload is not state {}'s model", state_description(state.get())),
        ));
    }
    if rule_packs_not_resolved {
        return Err(format!("{RULE_PACKS_NOT_RESOLVED}: recorded run `{}`: {RULE_PACKS_NOT_RESOLVED_DETAIL}.", stored_read::shown(recording_id)));
    }
    drop(manifest);
    read.hold(parse);
    Ok(read.join(found, |(), found| found).join(state, |(run, record), state| AnalysisRunRead {
        run,
        state,
        record,
        result_json,
        input_manifest_canonical: manifest_text,
    }))
}

/// `edit_analysis_run_labels` on a given store. Only the labels row changes, so
/// the record and its hash are untouched. `expected_updated_at`, when given,
/// must be the labels' current `updated_at` (`ANALYSIS-RUN-LABELS-MOVED`).
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn edit_run_labels_in_store(
    path: &Path,
    project_id: &str,
    recording_id: &str,
    labels: RunLabels,
    expected_updated_at: Option<&str>,
    now_millis: i64,
) -> Result<AnalysisRunSummary, String> {
    edit_run_labels_in_store_with(path, project_id, recording_id, labels, expected_updated_at, now_millis, &UnmeteredReadMemory)?
        .into_unreserved()
}

/// [`edit_run_labels_in_store`] within `memory`: the summaries it reads (one at
/// a time; records and labels) are reserved before they are loaded.
pub(crate) fn edit_run_labels_in_store_with(
    path: &Path,
    project_id: &str,
    recording_id: &str,
    labels: RunLabels,
    expected_updated_at: Option<&str>,
    now_millis: i64,
    memory: &dyn ReadMemory,
) -> Result<Metered<AnalysisRunSummary>, String> {
    edit_run_labels_in_store_finishing(path, project_id, recording_id, labels, expected_updated_at, now_millis, memory, Ok)
}

/// [`edit_run_labels_in_store_with`], the edited summary going to `finish` (the
/// command serializes it) before the transaction commits (repair r4).
#[allow(clippy::too_many_arguments)]
pub(crate) fn edit_run_labels_in_store_finishing<R>(
    path: &Path,
    project_id: &str,
    recording_id: &str,
    labels: RunLabels,
    expected_updated_at: Option<&str>,
    now_millis: i64,
    memory: &dyn ReadMemory,
    finish: impl FnOnce(Metered<AnalysisRunSummary>) -> Result<R, String>,
) -> Result<R, String> {
    let mut connection = stored_read::open_store(path)?;
    let transaction = stored_read::write_transaction(&mut connection)?;
    let _sqlite = stored_read::admit_sqlite_working_set(&transaction, memory)?;
    let current = find_run_with(&transaction, project_id, recording_id, memory)?
        .ok_or_else(|| format!("ANALYSIS-RUN-NOT-FOUND: project `{project_id}` has no recorded run `{recording_id}`.", project_id = stored_read::shown(&project_id), recording_id = stored_read::shown(&recording_id)))?;
    let (run, _) = current.get();
    if let Some(expected) = expected_updated_at {
        if expected != run.labels_updated_at {
            return Err(format!(
                "ANALYSIS-RUN-LABELS-MOVED: the labels of recorded run {} changed after this window read them; nothing was written.",
                run_description(run)
            ));
        }
    }
    let previous = parse_iso_millis(&run.labels_updated_at).unwrap_or(i64::MIN);
    let updated_at = utc_iso_millis(now_millis.max(previous.saturating_add(1)));
    write_run_labels(&transaction, project_id, recording_id, &labels, &updated_at, false)?;
    drop(current);
    let summary = find_run_with(&transaction, project_id, recording_id, memory)?
        .ok_or_else(|| format!("ANALYSIS-RUN-INTEGRITY: recorded run `{recording_id}` vanished.", recording_id = stored_read::shown(&recording_id)))?
        .map(|(summary, _)| summary);
    let finished = finish(summary)?;
    transaction.commit().map_err(sql)?;
    Ok(finished)
}

/// `delete_analysis_run` on a given store: the whole immutable row goes, with
/// its labels row (no other row depends on a run).
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn delete_run_in_store(path: &Path, project_id: &str, recording_id: &str) -> Result<AnalysisRunDeleteReceipt, String> {
    delete_run_in_store_with(path, project_id, recording_id, &UnmeteredReadMemory)?.into_unreserved()
}

/// [`delete_run_in_store_finishing`], keeping the receipt (tests).
pub(crate) fn delete_run_in_store_with(
    path: &Path,
    project_id: &str,
    recording_id: &str,
    memory: &dyn ReadMemory,
) -> Result<Metered<AnalysisRunDeleteReceipt>, String> {
    delete_run_in_store_finishing(path, project_id, recording_id, memory, Ok)
}

/// [`delete_run_in_store`] within `memory`, on its `BEGIN IMMEDIATE`
/// transaction: the run it verifies is priced before it is loaded. The
/// receipt, holding the run's reservations, goes to `finish` (the command
/// serializes it) before the transaction commits, so a refusal there deletes
/// nothing (repair r4).
pub(crate) fn delete_run_in_store_finishing<R>(
    path: &Path,
    project_id: &str,
    recording_id: &str,
    memory: &dyn ReadMemory,
    finish: impl FnOnce(Metered<AnalysisRunDeleteReceipt>) -> Result<R, String>,
) -> Result<R, String> {
    let mut connection = stored_read::open_store(path)?;
    let transaction = stored_read::write_transaction(&mut connection)?;
    let _sqlite = stored_read::admit_sqlite_working_set(&transaction, memory)?;
    let not_found = || format!("ANALYSIS-RUN-NOT-FOUND: project `{project_id}` has no recorded run `{recording_id}`.", project_id = stored_read::shown(&project_id), recording_id = stored_read::shown(&recording_id));
    let found = find_run_with(&transaction, project_id, recording_id, memory)?.ok_or_else(not_found)?;
    let deleted = transaction
        .execute(
            "DELETE FROM analysis_runs WHERE project_id = ?1 AND recording_id = ?2",
            params![project_id, recording_id],
        )
        .map_err(sql)?;
    let finished = finish(found.map(|(run, _record)| AnalysisRunDeleteReceipt {
        project_id: project_id.to_string(),
        recording_id: recording_id.to_string(),
        deleted: deleted == 1,
        message: format!("Deleted recorded run {}.", run_description(&run)),
    }))?;
    transaction.commit().map_err(sql)?;
    Ok(finished)
}

/// Deletes the run row a list's unreadable entry names by its `handle`
/// ([`stored_read::row_handle`]; W-12, DEL-14-01 Design §3), on a
/// `BEGIN IMMEDIATE` transaction: the row of the project with that rowid whose
/// whole stored id still has the handle's length and digest (hashed in chunks
/// inside the transaction), exactly that row and its labels. A handle that
/// names no such row, a stale one included, is not found. Inside the same
/// transaction the row is re-checked by the list's own check
/// ([`run_row_unreadable`]): a row that reads now (a replacement with the same
/// id, or a repair) is refused by name (`ANALYSIS-RUN-ROW-CHANGED`) and
/// nothing is deleted. Nothing of the row is trusted (deleting removes data);
/// it is loaded only as the list loads it, within `memory`.
pub(crate) fn delete_unreadable_run_in_store_finishing<R>(
    path: &Path,
    project_id: &str,
    handle: &str,
    memory: &dyn ReadMemory,
    finish: impl FnOnce(Metered<AnalysisRunDeleteReceipt>) -> Result<R, String>,
) -> Result<R, String> {
    let not_found = || {
        format!(
            "ANALYSIS-RUN-NOT-FOUND: project `{}` has no unreadable recorded run with handle `{}`.",
            stored_read::shown(project_id),
            stored_read::shown(handle)
        )
    };
    let rowid = stored_read::handle_rowid(RUN_ROW_HANDLE, handle).ok_or_else(not_found)?;
    let mut connection = stored_read::open_store(path)?;
    let transaction = stored_read::write_transaction(&mut connection)?;
    let _sqlite = stored_read::admit_sqlite_working_set(&transaction, memory)?;
    let found: Option<(Vec<u8>, i64)> = transaction
        .query_row(
            &format!("SELECT {} FROM analysis_runs WHERE rowid = ?1 AND project_id = ?2", stored_read::shown_column("recording_id")),
            params![rowid, project_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()
        .map_err(sql)?;
    let Some((head, head_length)) = found else { return Err(not_found()) };
    if stored_read::row_handle(&transaction, "analysis_runs", "recording_id", RUN_ROW_HANDLE, rowid, memory)? != handle {
        return Err(not_found());
    }
    let id = stored_read::shown_head(&head, head_length);
    if !run_row_unreadable(&transaction, project_id, rowid, memory)? {
        return Err(format!(
            "ANALYSIS-RUN-ROW-CHANGED: recorded run `{id}` reads now: this row changed since it was listed; refresh. Nothing was deleted."
        ));
    }
    let deleted = transaction
        .execute("DELETE FROM analysis_runs WHERE rowid = ?1 AND project_id = ?2", params![rowid, project_id])
        .map_err(sql)?;
    let finished = finish(Metered::new(AnalysisRunDeleteReceipt {
        project_id: project_id.to_string(),
        recording_id: id.clone(),
        deleted: deleted == 1,
        message: format!("Deleted unreadable recorded run `{id}`; its record was not read."),
    }))?;
    transaction.commit().map_err(sql)?;
    Ok(finished)
}

/// [`delete_unreadable_run_in_store_finishing`], keeping the receipt (tests).
#[cfg(test)]
pub(crate) fn delete_unreadable_run_in_store(path: &Path, project_id: &str, handle: &str) -> Result<AnalysisRunDeleteReceipt, String> {
    delete_unreadable_run_in_store_finishing(path, project_id, handle, &stored_read::UnmeteredReadMemory, Ok)?.into_unreserved()
}

// ---------------------------------------------------------------------------
// Commands (each admitted through `command_ingress` with its own budget).
// ---------------------------------------------------------------------------

/// `{request}` (snake_case fields, see `RecordRunRequest`).
pub(crate) fn record_analysis_run_args(
    args: &mut Args,
    store: impl FnOnce() -> Result<PathBuf, String>,
    memory: &dyn ReadMemory,
) -> Result<Serialized, CommandError> {
    let request = args.decode_with("request", RecordRunRequest::from_value)?;
    let finish = |outcome| stored_read::serialize_metered(memory, "record_analysis_run", outcome);
    store()
        .and_then(|path| record_run_in_store_finishing(&path, request, &PRODUCTION_RUN_HOOKS, memory, finish))
        .map_err(|error| CommandError::Command(stored_read::capped(error)))
}

/// `{projectId}`.
pub(crate) fn list_analysis_runs_args(
    args: &mut Args,
    store: impl FnOnce() -> Result<PathBuf, String>,
    memory: &dyn ReadMemory,
) -> Result<Metered<Vec<ListedRun>>, CommandError> {
    let project_id: String = args.decode("projectId")?;
    store()
        .and_then(|path| list_runs_in_store_with(&path, &project_id, memory))
        .map_err(|error| CommandError::Command(stored_read::capped(error)))
}

/// `{projectId, recordingId}`.
pub(crate) fn read_analysis_run_args(
    args: &mut Args,
    store: impl FnOnce() -> Result<PathBuf, String>,
    memory: &dyn ReadMemory,
) -> Result<Metered<AnalysisRunRead>, CommandError> {
    let project_id: String = args.decode("projectId")?;
    let recording_id: String = args.decode("recordingId")?;
    store()
        .and_then(|path| read_run_in_store_with(&path, &project_id, &recording_id, memory))
        .map_err(|error| CommandError::Command(stored_read::capped(error)))
}

/// `{projectId, recordingId, labels, expectedUpdatedAt?}`.
pub(crate) fn edit_analysis_run_labels_args(
    args: &mut Args,
    store: impl FnOnce() -> Result<PathBuf, String>,
    memory: &dyn ReadMemory,
) -> Result<Serialized, CommandError> {
    let project_id: String = args.decode("projectId")?;
    let recording_id: String = args.decode("recordingId")?;
    let labels = args.decode_with("labels", |value| run_labels_from_value(value).map_err(serde::de::Error::custom))?;
    let expected: Option<String> = args.decode("expectedUpdatedAt")?;
    let finish = |summary| stored_read::serialize_metered(memory, "edit_analysis_run_labels", summary);
    store()
        .and_then(|path| {
            edit_run_labels_in_store_finishing(&path, &project_id, &recording_id, labels, expected.as_deref(), now_unix_millis()?, memory, finish)
        })
        .map_err(|error| CommandError::Command(stored_read::capped(error)))
}

/// `{projectId, recordingId}` or `{projectId, unreadableHandle}`.
pub(crate) fn delete_analysis_run_args(
    args: &mut Args,
    store: impl FnOnce() -> Result<PathBuf, String>,
    memory: &dyn ReadMemory,
) -> Result<Serialized, CommandError> {
    let project_id: String = args.decode("projectId")?;
    // A readable run by its verified recording id, or a listed unreadable row by its handle.
    let recording_id: Option<String> = args.decode("recordingId")?;
    let handle: Option<String> = args.decode("unreadableHandle")?;
    let finish = |receipt| stored_read::serialize_metered(memory, "delete_analysis_run", receipt);
    store()
        .and_then(|path| match (recording_id, handle) {
            (Some(recording_id), None) => delete_run_in_store_finishing(&path, &project_id, &recording_id, memory, finish),
            (None, Some(handle)) => delete_unreadable_run_in_store_finishing(&path, &project_id, &handle, memory, finish),
            _ => Err("ANALYSIS-RUN-DELETE-ARGS: name the run by recordingId or a listed unreadable row by unreadableHandle, not both.".to_string()),
        })
        .map_err(|error| CommandError::Command(stored_read::capped(error)))
}

#[tauri::command]
pub fn record_analysis_run(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    // Save Local Project's memory guard: the per-invocation budget through the lease.
    let (mut args, lease) = command_ingress::admit(&registry, &invoke, "record_analysis_run", ArgsDecoding::Moved)?;
    let memory = PoolReadMemory::new(&registry, &lease);
    stored_read::respond_serialized(record_analysis_run_args(&mut args, || crate::app_store_path(&app), &memory))
}

#[tauri::command]
pub fn list_analysis_runs(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, lease) = command_ingress::admit_within(
        &registry,
        &invoke,
        "list_analysis_runs",
        ArgsDecoding::Moved,
        LIST_ANALYSIS_RUNS_MAX_ARGS_BYTES,
    )?;
    let memory = PoolReadMemory::new(&registry, &lease);
    stored_read::respond_metered(&memory, "list_analysis_runs", list_analysis_runs_args(&mut args, || crate::app_store_path(&app), &memory))
}

#[tauri::command]
pub fn read_analysis_run(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, lease) = command_ingress::admit_within(
        &registry,
        &invoke,
        "read_analysis_run",
        ArgsDecoding::Moved,
        READ_ANALYSIS_RUN_MAX_ARGS_BYTES,
    )?;
    let memory = PoolReadMemory::new(&registry, &lease);
    stored_read::respond_metered(&memory, "read_analysis_run", read_analysis_run_args(&mut args, || crate::app_store_path(&app), &memory))
}

#[tauri::command]
pub fn edit_analysis_run_labels(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, lease) = command_ingress::admit_within(
        &registry,
        &invoke,
        "edit_analysis_run_labels",
        ArgsDecoding::Moved,
        EDIT_ANALYSIS_RUN_LABELS_MAX_ARGS_BYTES,
    )?;
    let memory = PoolReadMemory::new(&registry, &lease);
    stored_read::respond_serialized(edit_analysis_run_labels_args(&mut args, || crate::app_store_path(&app), &memory))
}

#[tauri::command]
pub fn delete_analysis_run(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, lease) = command_ingress::admit_within(
        &registry,
        &invoke,
        "delete_analysis_run",
        ArgsDecoding::Moved,
        DELETE_ANALYSIS_RUN_MAX_ARGS_BYTES,
    )?;
    let memory = PoolReadMemory::new(&registry, &lease);
    stored_read::respond_serialized(delete_analysis_run_args(&mut args, || crate::app_store_path(&app), &memory))
}

#[cfg(test)]
#[path = "analysis_runs_tests.rs"]
mod tests;
