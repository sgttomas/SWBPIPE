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

use crate::command_ingress::{self, respond, Args, ArgsDecoding, CommandError};
use crate::model_states::{
    self, find_summary, insert_new_state, labels_from_value_in, now_unix_millis, parse_iso_millis,
    sql, state_description, utc_iso_millis, LabelVocabulary, ModelStateSummary, StateLabels,
    StateWriteHooks,
};
use crate::solve_jobs::SolveJobRegistry;
use open_pipe_stress_canonical_json::{canonical_json_checked_v1_text, CHECKED_PROFILE_V1};
use open_pipe_stress_operation_applier::{canonical_json, sha256_hex};
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
        .map_err(|error| error.to_string())
}

/// Ingress budgets (raw argument bytes) of the small run commands.
/// `record_analysis_run` carries the result and the manifest and is bounded
/// by the per-invocation budget through its lease, as `save_local_project` is.
pub(crate) const LIST_ANALYSIS_RUNS_MAX_ARGS_BYTES: usize = 4 * 1024;
pub(crate) const READ_ANALYSIS_RUN_MAX_ARGS_BYTES: usize = 4 * 1024;
pub(crate) const DELETE_ANALYSIS_RUN_MAX_ARGS_BYTES: usize = 4 * 1024;
pub(crate) const EDIT_ANALYSIS_RUN_LABELS_MAX_ARGS_BYTES: usize = 256 * 1024;

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
/// `canonical_sha256_checked_v1` compute it.
pub(crate) fn checked_sha256(value: &Value) -> Result<String, String> {
    let text = serde_json::to_string(value).map_err(|error| error.to_string())?;
    checked_sha256_text(&text)
}

/// [`checked_sha256`] of JSON text (the text is not rewritten).
pub(crate) fn checked_sha256_text(text: &str) -> Result<String, String> {
    let canonical = canonical_json_checked_v1_text(text)?;
    Ok(sha256_hex(&canonical))
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

/// The `analysis_run_record` hash of a v0.3 record.
pub(crate) fn run_record_hash(record: &Value) -> Result<String, String> {
    checked_sha256(&record_projection(record)?)
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

/// The recorded revision's `rule_pack_refs`: the manifest's `active_rule_packs`
/// (what the solve used), in the manifest's order, as
/// `{RulePack, "<rule_pack_id>@<version>"}`.
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
                    Ok(reference("RulePack", &format!("{id}@{version}")))
                }
                _ => Err("ANALYSIS-RUN-MANIFEST-INVALID: an active rule pack has no id or version.".to_string()),
            }
        })
        .collect()
}

/// One `provenance_record.library_ref` as a reference:
/// `library:<kind>/<id>[@<version>][#<record id>]`.
fn library_reference_text(library_ref: &Value) -> Option<String> {
    let kind = library_ref.get("library_kind")?.as_str()?;
    let id = library_ref.get("library_id")?.as_str()?;
    let mut text = format!("library:{kind}/{id}");
    if let Some(version) = library_ref.get("library_version").and_then(Value::as_str) {
        text.push('@');
        text.push_str(version);
    }
    if let Some(record) = library_ref.get("record_id").and_then(Value::as_str) {
        text.push('#');
        text.push_str(record);
    }
    Some(text)
}

fn collect_library_refs(value: &Value, found: &mut BTreeSet<String>) {
    match value {
        Value::Object(map) => {
            for (key, child) in map {
                if key == "provenance_record" {
                    if let Some(text) = child.get("library_ref").and_then(library_reference_text) {
                        found.insert(text);
                    }
                }
                collect_library_refs(child, found);
            }
        }
        Value::Array(items) => items.iter().for_each(|item| collect_library_refs(item, found)),
        _ => {}
    }
}

/// The recorded revision's `library_refs`: every `provenance_record.library_ref`
/// of the solved model (at any depth), once each, sorted by code point, as
/// `{Library, "library:<kind>/<id>[@<version>][#<record id>]"}`.
pub(crate) fn library_refs(model_payload: &Value) -> Vec<Value> {
    let mut found = BTreeSet::new();
    collect_library_refs(model_payload, &mut found);
    found.iter().map(|text| reference("Library", text)).collect()
}

/// The recorded revision of a transient record (design §3), and its hash. The
/// received-result checksum and every other member are unchanged.
pub(crate) fn recorded_revision(
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
    let hash = run_record_hash(&record)?;
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
    format!("runrec:{}", &record_hash[..record_hash.len().min(32)])
}

// ---------------------------------------------------------------------------
// The binding checks of Record Run (before any store access).
// ---------------------------------------------------------------------------

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
}

/// The binding of a submission, checked, not asserted:
/// - the input manifest is its own canonical JCS text and its SHA-256 is the
///   record's `input_manifest_hashes[0]`, whose ref is `input_manifest_refs[0]`;
/// - the manifest and its model payload are the project's;
/// - the record is a transient v0.3 record (`created_at` null, the synthetic
///   `state:<project>:preview` ref) whose own checksum verifies;
/// - the result is the record's received result (its checked SHA-256 and ref)
///   and its run and project.
pub(crate) fn checked_submission(
    project_id: &str,
    transient: &Value,
    result_json: &str,
    manifest_text: &str,
) -> Result<CheckedSubmission, String> {
    const MANIFEST: &str = "ANALYSIS-RUN-MANIFEST-MISMATCH";
    let manifest: Value = serde_json::from_str(manifest_text)
        .map_err(|error| refused(MANIFEST, format_args!("the input manifest is not JSON ({error})")))?;
    if canonical_json(&manifest) != manifest_text {
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
        return Err(refused(MANIFEST, format_args!("the input manifest is not of project `{project_id}`")));
    }
    let payload_canonical = canonical_json(model_payload);
    let payload_hash = format!("sha256:{}", sha256_hex(&payload_canonical));

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
    let run_id = transient.pointer("/analysis_run/run_id").and_then(Value::as_str).unwrap_or("");
    let claim = single_hash(transient, "analysis_run_record");
    let recomputed = run_record_hash(transient).map_err(|error| refused(RECORD, error))?;
    if claim.and_then(|hash| hash.get("value")).and_then(Value::as_str) != Some(recomputed.as_str())
        || claim.and_then(|hash| hash.get("canonicalization")).and_then(Value::as_str) != Some(CHECKED_PROFILE_V1)
        || claim.and_then(|hash| hash.get("payload_ref")) != Some(&reference("AnalysisRun", run_id))
    {
        return Err(refused(RECORD, format_args!("the record's own checksum does not verify (it hashes to {recomputed})")));
    }

    const RESULT: &str = "ANALYSIS-RUN-RESULT-MISMATCH";
    let head: ResultHead = serde_json::from_str(result_json)
        .map_err(|error| refused(RESULT, format_args!("the result is not a mechanics result ({error})")))?;
    let result_hash_hex = checked_sha256_text(result_json).map_err(|error| refused(RESULT, error))?;
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
    Ok(CheckedSubmission {
        manifest,
        payload_canonical,
        payload_hash,
        producer_run_id: run_id.to_string(),
        result_hash_hex,
    })
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

fn decode_run_labels(name: String, tags: &str, notes: &str, assumptions: &str) -> Result<RunLabels, String> {
    let parse = |text: &str| {
        serde_json::from_str::<Value>(text)
            .map_err(|error| format!("ANALYSIS-RUN-INTEGRITY: a stored run label is not JSON ({error})."))
    };
    let strings = |value: Value| -> Result<Vec<String>, String> {
        serde_json::from_value(value)
            .map_err(|error| format!("ANALYSIS-RUN-INTEGRITY: a stored run label list is malformed ({error})."))
    };
    Ok(RunLabels {
        run_name: name,
        tags: strings(parse(tags)?)?,
        notes: strings(parse(notes)?)?,
        assumptions: serde_json::from_value(parse(assumptions)?)
            .map_err(|error| format!("ANALYSIS-RUN-INTEGRITY: the stored run assumptions are malformed ({error})."))?,
    })
}

fn integrity_of(recording_id: &str, detail: impl std::fmt::Display) -> String {
    format!("ANALYSIS-RUN-INTEGRITY: recorded run `{recording_id}`: {detail}; it is neither listed nor read.")
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
fn verified_run_record(columns: &RunColumns<'_>, record_json: &str) -> Result<Value, String> {
    let id = columns.recording_id;
    let record: Value = serde_json::from_str(record_json)
        .map_err(|error| integrity_of(id, format_args!("the stored record is not JSON ({error})")))?;
    let recomputed = run_record_hash(&record).map_err(|error| integrity_of(id, error))?;
    let run = record.get("analysis_run").cloned().unwrap_or(Value::Null);
    let text = |pointer: &str| run.pointer(pointer).and_then(Value::as_str);
    let claim = single_hash(&record, "analysis_run_record");
    let received = single_hash(&record, "received_result");
    let manifest_hash = single_manifest_hash(&record)
        .and_then(|hash| hash.get("value"))
        .and_then(Value::as_str)
        .unwrap_or("");
    let run_id = text("/run_id").unwrap_or("");
    let checks: [(&str, bool); 8] = [
        (
            "record hash",
            recomputed == columns.run_record_hash
                && claim.and_then(|hash| hash.get("value")).and_then(Value::as_str) == Some(recomputed.as_str())
                && claim.and_then(|hash| hash.get("canonicalization")).and_then(Value::as_str)
                    == Some(CHECKED_PROFILE_V1)
                && claim.and_then(|hash| hash.get("payload_ref")) == Some(&reference("AnalysisRun", run_id)),
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
            received.and_then(|hash| hash.get("value")).and_then(Value::as_str) == Some(columns.result_hash)
                && received.and_then(|hash| hash.get("payload_ref"))
                    == Some(&reference("ResultEnvelope", &format!("result-envelope:{run_id}"))),
        ),
        (
            "project",
            text("/unit_system_ref/ref") == Some(format!("unit-system:{}:{manifest_hash}", columns.project_id).as_str()),
        ),
        (
            "schema version",
            record.get("schema_version").and_then(Value::as_str) == Some("0.3.0"),
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

fn summary_from_row(connection: &Connection, row: SummaryRow) -> Result<(AnalysisRunSummary, Value), String> {
    let (project_id, recording_id, state_id, producer_run_id, created_at, run_record_hash, result_hash, name, tags, notes, assumptions, updated_at) =
        row;
    let (Some(name), Some(tags), Some(notes), Some(assumptions), Some(updated_at)) = (name, tags, notes, assumptions, updated_at)
    else {
        return Err(format!("ANALYSIS-RUN-INTEGRITY: recorded run `{recording_id}` has no labels row."));
    };
    let record_json: String = connection
        .query_row(
            "SELECT run_record_json FROM analysis_runs WHERE project_id = ?1 AND recording_id = ?2",
            params![project_id, recording_id],
            |row| row.get(0),
        )
        .map_err(sql)?;
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
    )?;
    let analysis_status = serde_json::from_value(record.pointer("/analysis_run/analysis_status").cloned().unwrap_or(json!([])))
        .map_err(|error| integrity_of(&recording_id, format_args!("its analysis status is malformed ({error})")))?;
    Ok((
        AnalysisRunSummary {
            labels: decode_run_labels(name, &tags, &notes, &assumptions)?,
            analysis_status,
            project_id,
            recording_id,
            state_id,
            producer_run_id,
            created_at,
            run_record_hash,
            result_hash,
            labels_updated_at: updated_at,
        },
        record,
    ))
}

fn find_run(connection: &Connection, project_id: &str, recording_id: &str) -> Result<Option<(AnalysisRunSummary, Value)>, String> {
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
        .map_err(sql)?;
    row.map(|row| summary_from_row(connection, row)).transpose()
}

/// The project's recorded runs, newest first. Each row's record is verified;
/// the result and the manifest are not loaded.
pub(crate) fn list_runs(connection: &Connection, project_id: &str) -> Result<Vec<AnalysisRunSummary>, String> {
    let mut statement = connection
        .prepare(&format!(
            "SELECT {SUMMARY_COLUMNS} FROM analysis_runs r LEFT JOIN analysis_run_labels l
             ON l.project_id = r.project_id AND l.recording_id = r.recording_id WHERE r.project_id = ?1
             ORDER BY r.created_at DESC, r.recording_id"
        ))
        .map_err(sql)?;
    let rows = statement
        .query_map(params![project_id], summary_row)
        .map_err(sql)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(sql)?;
    rows.into_iter()
        .map(|row| summary_from_row(connection, row).map(|(summary, _)| summary))
        .collect()
}

/// How many recorded runs the project id holds (the replacement prompt's count).
pub(crate) fn recorded_run_count(connection: &Connection, project_id: &str) -> Result<i64, String> {
    connection
        .query_row("SELECT COUNT(*) FROM analysis_runs WHERE project_id = ?1", params![project_id], |row| row.get(0))
        .map_err(sql)
}

pub(crate) fn run_description(run: &AnalysisRunSummary) -> String {
    format!("\"{}\" ({})", run.labels.run_name, run.recording_id)
}

/// The recording time: the later of `now` and one millisecond after the
/// project's latest recorded `created_at` (read inside the write transaction).
fn allocate_created_at(transaction: &Transaction<'_>, project_id: &str, now: i64) -> Result<String, String> {
    let latest: Option<String> = transaction
        .query_row(
            "SELECT MAX(created_at) FROM analysis_runs WHERE project_id = ?1",
            params![project_id],
            |row| row.get(0),
        )
        .map_err(sql)?;
    let after_latest = match latest {
        None => i64::MIN,
        Some(text) => parse_iso_millis(&text)
            .ok_or_else(|| format!("ANALYSIS-RUN-INTEGRITY: a recorded run's created_at `{text}` is not a recording time."))?
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
    let text = |result: serde_json::Result<String>| result.map_err(|error| error.to_string());
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
        return Err(format!("ANALYSIS-RUN-INTEGRITY: the labels of recorded run `{recording_id}` were not written."));
    }
    Ok(())
}

/// `record_analysis_run` on a given store: the binding checks, then (in one
/// `BEGIN IMMEDIATE` transaction with the project row's stale-window check) the
/// state (found by the solved model's hash, or a new post-solve snapshot and
/// its labels), the recording time, the recorded revision, the run row and its
/// labels row.
pub(crate) fn record_run_in_store(
    path: &Path,
    request: RecordRunRequest,
    hooks: &RunWriteHooks<'_>,
) -> Result<RecordRunOutcome, String> {
    let project_id = request.project_id.as_str();
    let submission = checked_submission(
        project_id,
        &request.run_record,
        &request.result_json,
        &request.input_manifest_canonical,
    )?;
    let (mut connection, _migration) = crate::open_project_store(path)?;
    let transaction = crate::immediate_transaction(&mut connection)?;
    if crate::stored_project_entry(&transaction, project_id)?.is_none() {
        return Err(format!(
            "ANALYSIS-RUN-PROJECT-MISSING: project `{project_id}` has no stored local project; save the local project first. Nothing was recorded."
        ));
    }
    crate::refuse_unconfirmed_replacement(&transaction, project_id, None, Some(&request.expected_base))?;
    let (state, state_outcome) = match &request.state_id {
        Some(state_id) => {
            let state = find_summary(&transaction, project_id, state_id)?.ok_or_else(|| {
                refused("ANALYSIS-RUN-STATE-NOT-FOUND", format_args!("project `{project_id}` has no saved state `{state_id}`"))
            })?;
            if state.model_hash != submission.payload_hash {
                return Err(refused(
                    "ANALYSIS-RUN-STATE-MISMATCH",
                    format_args!(
                        "the solved model hashes to {}, not state {}'s {}",
                        submission.payload_hash,
                        state_description(&state),
                        state.model_hash
                    ),
                ));
            }
            (state, "existing")
        }
        None => match model_states::state_with_model_hash(&transaction, project_id, &submission.payload_hash)? {
            Some(state) => (state, "existing"),
            None => {
                let Some(new_state) = &request.new_state else {
                    return Err(refused(
                        "ANALYSIS-RUN-STATE-REQUIRED",
                        format_args!(
                            "project `{project_id}` has no saved state of the solved model ({}); name a post-solve snapshot to record the run against",
                            submission.payload_hash
                        ),
                    ));
                };
                let snapshot = model_states::checked_new_state(
                    project_id,
                    POST_SOLVE_SNAPSHOT,
                    submission.payload_canonical.clone(),
                    &submission.payload_hash,
                )?;
                let allocate = || Ok(((hooks.state_token)()?, (hooks.now)()?));
                let between_rows = |transaction: &Transaction<'_>| (hooks.between_rows)(transaction, "state");
                let state = insert_new_state(
                    &transaction,
                    project_id,
                    POST_SOLVE_SNAPSHOT,
                    new_state.parent_state_id.as_deref(),
                    &snapshot,
                    &new_state.labels,
                    &StateWriteHooks { allocate: &allocate, between_rows: &between_rows },
                )?;
                (state, "created")
            }
        },
    };
    let created_at = allocate_created_at(&transaction, project_id, (hooks.now)()?)?;
    let (record, record_hash) = recorded_revision(&request.run_record, &state.state_id, &created_at, &submission.manifest)?;
    let recording_id = recording_id_of(&record_hash);
    let record_json = serde_json::to_string(&record).map_err(|error| error.to_string())?;
    transaction
        .execute(
            "INSERT INTO analysis_runs (project_id, recording_id, state_id, producer_run_id, created_at, run_record_json, run_record_hash, input_manifest_json, result_json, result_hash)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                project_id,
                recording_id,
                state.state_id,
                submission.producer_run_id,
                created_at,
                record_json,
                record_hash,
                request.input_manifest_canonical,
                request.result_json,
                submission.result_hash_hex
            ],
        )
        .map_err(sql)?;
    (hooks.between_rows)(&transaction, "run")?;
    write_run_labels(&transaction, project_id, &recording_id, &request.run_labels, &created_at, true)?;
    let (run, _) = find_run(&transaction, project_id, &recording_id)?
        .ok_or_else(|| format!("ANALYSIS-RUN-INTEGRITY: recorded run `{recording_id}` was not written."))?;
    // The state as it now stands (its recorded runs include this one).
    let state = find_summary(&transaction, project_id, &state.state_id)?
        .ok_or_else(|| format!("MODEL-STATE-INTEGRITY: state `{}` vanished.", state.state_id))?;
    transaction.commit().map_err(sql)?;
    let against = if state_outcome == "created" {
        format!("the new post-solve snapshot {}", state_description(&state))
    } else {
        format!("model state {}", state_description(&state))
    };
    let unused = if state_outcome == "existing" && request.state_id.is_none() && request.new_state.is_some() {
        " The solved model was already saved as that state, so the post-solve snapshot name was not used."
    } else {
        ""
    };
    let message = format!("Recorded run {} against {against}.{unused} {LABELS_DO_NOT_CHANGE_RUN}", run_description(&run));
    Ok(RecordRunOutcome { run, state, state_outcome, message })
}

/// `read_analysis_run` on a given store: the record, the stored result and the
/// stored manifest, after the bytes are checked against the record and the
/// state: the record (its hash and identity columns), the result's checked
/// hash, the manifest's canonical form, its hash against the record, its
/// project, and its model payload's hash against the state's model hash.
pub(crate) fn read_run_in_store(path: &Path, project_id: &str, recording_id: &str) -> Result<AnalysisRunRead, String> {
    let (connection, _migration) = crate::open_project_store(path)?;
    let (run, record) = find_run(&connection, project_id, recording_id)?.ok_or_else(|| {
        format!("ANALYSIS-RUN-NOT-FOUND: project `{project_id}` has no recorded run `{recording_id}`.")
    })?;
    let (result_json, manifest_text): (String, String) = connection
        .query_row(
            "SELECT result_json, input_manifest_json FROM analysis_runs WHERE project_id = ?1 AND recording_id = ?2",
            params![project_id, recording_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(sql)?;
    let refuse = |field: &str| integrity_of(recording_id, format_args!("the stored {field} differs from its verified record"));
    if checked_sha256_text(&result_json).ok().as_deref() != Some(run.result_hash.as_str()) {
        return Err(refuse("result"));
    }
    let manifest: Value = serde_json::from_str(&manifest_text).map_err(|_| refuse("input manifest"))?;
    let manifest_hash = single_manifest_hash(&record).and_then(|hash| hash.get("value")).and_then(Value::as_str);
    if canonical_json(&manifest) != manifest_text || manifest_hash != Some(sha256_hex(&manifest_text).as_str()) {
        return Err(refuse("input manifest"));
    }
    if manifest.pointer("/model_basis/model_ref").and_then(Value::as_str) != Some(project_id) {
        return Err(refuse("input manifest's project"));
    }
    let state = find_summary(&connection, project_id, &run.state_id)?.ok_or_else(|| {
        integrity_of(recording_id, format_args!("its state `{}` is not saved", run.state_id))
    })?;
    let payload_hash = manifest
        .pointer("/model_basis/model_payload")
        .map(|payload| format!("sha256:{}", sha256_hex(&canonical_json(payload))));
    if payload_hash.as_deref() != Some(state.model_hash.as_str()) {
        return Err(integrity_of(
            recording_id,
            format_args!("the stored input manifest's model payload is not state {}'s model", state_description(&state)),
        ));
    }
    Ok(AnalysisRunRead { run, state, record, result_json, input_manifest_canonical: manifest_text })
}

/// `edit_analysis_run_labels` on a given store. Only the labels row changes, so
/// the record and its hash are untouched. `expected_updated_at`, when given,
/// must be the labels' current `updated_at` (`ANALYSIS-RUN-LABELS-MOVED`).
pub(crate) fn edit_run_labels_in_store(
    path: &Path,
    project_id: &str,
    recording_id: &str,
    labels: RunLabels,
    expected_updated_at: Option<&str>,
    now_millis: i64,
) -> Result<AnalysisRunSummary, String> {
    let (mut connection, _migration) = crate::open_project_store(path)?;
    let transaction = crate::immediate_transaction(&mut connection)?;
    let (current, _) = find_run(&transaction, project_id, recording_id)?.ok_or_else(|| {
        format!("ANALYSIS-RUN-NOT-FOUND: project `{project_id}` has no recorded run `{recording_id}`.")
    })?;
    if let Some(expected) = expected_updated_at {
        if expected != current.labels_updated_at {
            return Err(format!(
                "ANALYSIS-RUN-LABELS-MOVED: the labels of recorded run {} changed after this window read them; nothing was written.",
                run_description(&current)
            ));
        }
    }
    let previous = parse_iso_millis(&current.labels_updated_at).unwrap_or(i64::MIN);
    let updated_at = utc_iso_millis(now_millis.max(previous.saturating_add(1)));
    write_run_labels(&transaction, project_id, recording_id, &labels, &updated_at, false)?;
    let (summary, _) = find_run(&transaction, project_id, recording_id)?
        .ok_or_else(|| format!("ANALYSIS-RUN-INTEGRITY: recorded run `{recording_id}` vanished."))?;
    transaction.commit().map_err(sql)?;
    Ok(summary)
}

/// `delete_analysis_run` on a given store: the whole immutable row goes, with
/// its labels row (no other row depends on a run).
pub(crate) fn delete_run_in_store(path: &Path, project_id: &str, recording_id: &str) -> Result<AnalysisRunDeleteReceipt, String> {
    let (mut connection, _migration) = crate::open_project_store(path)?;
    let transaction = crate::immediate_transaction(&mut connection)?;
    let (run, _) = find_run(&transaction, project_id, recording_id)?.ok_or_else(|| {
        format!("ANALYSIS-RUN-NOT-FOUND: project `{project_id}` has no recorded run `{recording_id}`.")
    })?;
    let deleted = transaction
        .execute(
            "DELETE FROM analysis_runs WHERE project_id = ?1 AND recording_id = ?2",
            params![project_id, recording_id],
        )
        .map_err(sql)?;
    transaction.commit().map_err(sql)?;
    Ok(AnalysisRunDeleteReceipt {
        project_id: project_id.to_string(),
        recording_id: recording_id.to_string(),
        deleted: deleted == 1,
        message: format!("Deleted recorded run {}.", run_description(&run)),
    })
}

// ---------------------------------------------------------------------------
// Commands (each admitted through `command_ingress` with its own budget).
// ---------------------------------------------------------------------------

/// `{request}` (snake_case fields, see `RecordRunRequest`).
pub(crate) fn record_analysis_run_args(
    args: &mut Args,
    store: impl FnOnce() -> Result<PathBuf, String>,
) -> Result<RecordRunOutcome, CommandError> {
    let request = args.decode_with("request", RecordRunRequest::from_value)?;
    store()
        .and_then(|path| record_run_in_store(&path, request, &PRODUCTION_RUN_HOOKS))
        .map_err(CommandError::Command)
}

/// `{projectId}`.
pub(crate) fn list_analysis_runs_args(
    args: &mut Args,
    store: impl FnOnce() -> Result<PathBuf, String>,
) -> Result<Vec<AnalysisRunSummary>, CommandError> {
    let project_id: String = args.decode("projectId")?;
    store()
        .and_then(|path| {
            let (connection, _migration) = crate::open_project_store(&path)?;
            list_runs(&connection, &project_id)
        })
        .map_err(CommandError::Command)
}

/// `{projectId, recordingId}`.
pub(crate) fn read_analysis_run_args(
    args: &mut Args,
    store: impl FnOnce() -> Result<PathBuf, String>,
) -> Result<AnalysisRunRead, CommandError> {
    let project_id: String = args.decode("projectId")?;
    let recording_id: String = args.decode("recordingId")?;
    store()
        .and_then(|path| read_run_in_store(&path, &project_id, &recording_id))
        .map_err(CommandError::Command)
}

/// `{projectId, recordingId, labels, expectedUpdatedAt?}`.
pub(crate) fn edit_analysis_run_labels_args(
    args: &mut Args,
    store: impl FnOnce() -> Result<PathBuf, String>,
) -> Result<AnalysisRunSummary, CommandError> {
    let project_id: String = args.decode("projectId")?;
    let recording_id: String = args.decode("recordingId")?;
    let labels = args.decode_with("labels", |value| run_labels_from_value(value).map_err(serde::de::Error::custom))?;
    let expected: Option<String> = args.decode("expectedUpdatedAt")?;
    store()
        .and_then(|path| {
            edit_run_labels_in_store(&path, &project_id, &recording_id, labels, expected.as_deref(), now_unix_millis()?)
        })
        .map_err(CommandError::Command)
}

/// `{projectId, recordingId}`.
pub(crate) fn delete_analysis_run_args(
    args: &mut Args,
    store: impl FnOnce() -> Result<PathBuf, String>,
) -> Result<AnalysisRunDeleteReceipt, CommandError> {
    let project_id: String = args.decode("projectId")?;
    let recording_id: String = args.decode("recordingId")?;
    store()
        .and_then(|path| delete_run_in_store(&path, &project_id, &recording_id))
        .map_err(CommandError::Command)
}

#[tauri::command]
pub fn record_analysis_run(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    // Save Local Project's memory guard: the per-invocation budget through the lease.
    let (mut args, _lease) = command_ingress::admit(&registry, &invoke, "record_analysis_run", ArgsDecoding::Moved)?;
    respond(record_analysis_run_args(&mut args, || crate::app_store_path(&app)))
}

#[tauri::command]
pub fn list_analysis_runs(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, _lease) = command_ingress::admit_within(
        &registry,
        &invoke,
        "list_analysis_runs",
        ArgsDecoding::Moved,
        LIST_ANALYSIS_RUNS_MAX_ARGS_BYTES,
    )?;
    respond(list_analysis_runs_args(&mut args, || crate::app_store_path(&app)))
}

#[tauri::command]
pub fn read_analysis_run(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, _lease) = command_ingress::admit_within(
        &registry,
        &invoke,
        "read_analysis_run",
        ArgsDecoding::Moved,
        READ_ANALYSIS_RUN_MAX_ARGS_BYTES,
    )?;
    respond(read_analysis_run_args(&mut args, || crate::app_store_path(&app)))
}

#[tauri::command]
pub fn edit_analysis_run_labels(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, _lease) = command_ingress::admit_within(
        &registry,
        &invoke,
        "edit_analysis_run_labels",
        ArgsDecoding::Moved,
        EDIT_ANALYSIS_RUN_LABELS_MAX_ARGS_BYTES,
    )?;
    respond(edit_analysis_run_labels_args(&mut args, || crate::app_store_path(&app)))
}

#[tauri::command]
pub fn delete_analysis_run(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, _lease) = command_ingress::admit_within(
        &registry,
        &invoke,
        "delete_analysis_run",
        ArgsDecoding::Moved,
        DELETE_ANALYSIS_RUN_MAX_ARGS_BYTES,
    )?;
    respond(delete_analysis_run_args(&mut args, || crate::app_store_path(&app)))
}

#[cfg(test)]
#[path = "analysis_runs_tests.rs"]
mod tests;
