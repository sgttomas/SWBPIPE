//! C1 slice 6's native save continuity (DEL-04-05 `c1-native-custody-plan.md`
//! slice 6): Save or Create Project of a project whose saved run was reopened
//! into native custody (`stored_custody`), from the restored handle and the
//! project's small fields. The stored result and record stay in this process;
//! the webview sends no result, record, model hash or envelope hash, and
//! nothing row-scale comes back.
//!
//! - **What is stored is what the legacy session stored.** Before this path
//!   the session copied the stored run whole into the webview (at or under
//!   64 MiB), `JSON.parse`d it and sent it back in `save_local_project`'s
//!   request, whose ingress serde_json read from `JSON.stringify`'s text. So
//!   the stored result and record are written here as serde_json reads
//!   `JSON.stringify` of their stored values' parse: the held texts' parse is
//!   read with JavaScript's numbers ([`build::js_numbers`]: binary64 values,
//!   a negative zero `0`, JavaScript's number text), then written by the
//!   builder's stored writer ([`build::Style::Stored`], members in serde_json's
//!   order); the envelope hash is computed over those values, as
//!   `computeProjectEnvelopeHash` hashed `JSON.stringify`. Nothing is
//!   rebuilt: the record is
//!   the stored record (its `created_at`, `model_state_ref`, recorded revision
//!   and claims as stored), never a fresh builder's. The held texts, the
//!   stored rows they came from and the store's schema and model versions are
//!   never rewritten or migrated.
//! - **Hashes the host holds.** The model hash evidence written is the stored
//!   project's (as the session sent its historical evidence), read at reopen
//!   and held with the handle (`solve_jobs::RestoredBasis`), written as
//!   JavaScript round-tripped it. The envelope hash is computed here over
//!   what is stored (`computeProjectEnvelopeHash`'s payload, streamed); when
//!   it equals the payload hash of the envelope basis the session holds, that
//!   basis's stored evidence is written instead, as the legacy session
//!   re-saved an unchanged historical project. The session names its basis
//!   (the reopen's, or a previous re-save's from this reply); only a basis
//!   the host established for this handle is accepted.
//! - **Binding** (each [`CUSTODY`], before anything is written). The handle
//!   is a restored saved project run still held (not a solve, not a restored
//!   recorded run: no session path saves a recorded run into a project; not
//!   disposed or reset); the stored source the session names (kind, project,
//!   recording, both SHA-256) is the one the handle holds; the held texts
//!   still hash to that source's SHA-256 (hashed in place); the envelope
//!   basis is one the host established. Fresh persistence, the fresh record
//!   preview and exact-route custody keep refusing the handle; this path
//!   grants it no intake, publication, worker, attestation or Current fact.
//!   Its provenance stays historical: the reply's findings are those the
//!   native reopen of the project as stored gives.
//! - **The save contract, unchanged.** The model document checks, the
//!   replacement guard (`expected_base` / confirmed revision), SQLite's exact
//!   lengths, the model-only save (`PROJECT-SAVE-RESULT-TOO-LARGE`, a
//!   successful save without the result), the withheld-run guard before a
//!   model-only save drops a stored run, the no-save refusal
//!   (`PROJECT-SAVE-TOO-LARGE`, rolled back) and the reply are the fresh native
//!   save's (`native_persistence::stage_save` and `finish_save`, shared).
//! - **Leases** (no new budget or rule; each capped at min(per-invocation
//!   budget, 12 GiB)). The raw body's copy and the decoded arguments; each
//!   held text's parse (the prescan's materialization M, admitted before the
//!   parse); the canonical writer's working lease; each output text at its
//!   exact length; SQLite's statement copies; the reply at its exact length.
//!   The held texts are the handle's, already leased as result leases: this
//!   path shares them and credits them nothing more; a disposal or reset
//!   while it runs releases them when its share drops. The parses and the
//!   working lease end before the statements run.
//! - **Residuals, unpriced** (slice 7): the validator's working memory for
//!   the reply's findings (one row's values, within the fixed allowance),
//!   SQLite's page cache, journal and FTS5 state, and Tauri's delivery copies
//!   of the small reply, as the fresh save's.
use crate::analysis_run_v03_build as build;
use crate::analysis_run_v03_validate::{self as validate, HistoricalFindings, SavedProject};
use crate::command_ingress::{self, ArgsDecoding, CommandError};
use crate::memory_pool::Lease;
use crate::native_persistence::{self as persistence, refusal, same_js_data, ProjectFields, SaveLimits, SaveRun, CUSTODY, PERSISTENCE_FAILED};
use crate::solve_jobs::{sha256_in_place, EnvelopeBasis, HeldResult, ResultOrigin, RestoredSource, SolveJobRegistry};
use crate::stored_read::{self, NativeLeaseMemory, Serialized};
use rusqlite::limits::Limit;
use rusqlite::Transaction;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::http::HeaderMap;
use tauri::ipc::InvokeBody;

pub(crate) const COMMAND: &str = "save_restored_result_local_project";
/// The only restored source a project save re-saves.
const SAVED_PROJECT_RUN: &str = "saved_project_run";

/// The stored source the session names: the restored run's `source` as the
/// reopen returned it.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ExpectedSource {
    kind: String,
    project_id: String,
    recording_id: Option<String>,
    result_sha256: String,
    record_sha256: Option<String>,
}

/// The envelope basis the session holds: the payload hash and the envelope
/// evidence of the project as last opened or re-saved from this handle.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SessionBasis {
    payload_sha256: Option<String>,
    evidence: Value,
}

/// The re-save's members (as `create_local_project`'s and
/// `save_local_project`'s, without the result, record and both hashes).
struct Request {
    job_id: String,
    expected_source: ExpectedSource,
    basis: SessionBasis,
    create: bool,
    project_id: Option<String>,
    project_name: Option<String>,
    model: Value,
    editor_intents: Option<Value>,
    proposal: Option<Value>,
    selected_review_target: Option<Value>,
    confirmed_revision: Option<String>,
    expected_base: Option<String>,
}

fn decode(args: &mut command_ingress::Args) -> Result<Request, CommandError<String>> {
    let mode: String = args.decode("mode")?;
    let create = match mode.as_str() {
        "create" => true,
        "save" => false,
        _ => return Err(CommandError::Command("INGRESS-ARGS-INVALID: `mode` is `create` or `save`.".into())),
    };
    Ok(Request {
        job_id: args.decode("jobId")?,
        expected_source: args.decode("expectedSource")?,
        basis: args.decode("envelopeBasis")?,
        create,
        project_id: args.decode("projectId")?,
        project_name: args.decode("projectName")?,
        model: args.value("model")?,
        editor_intents: args.opt_value("editorIntents"),
        proposal: args.opt_value("proposal"),
        selected_review_target: args.opt_value("selectedReviewTarget"),
        confirmed_revision: args.decode("confirmedRevision")?,
        expected_base: args.decode("expectedBase")?,
    })
}

/// What the reply says of the restored run besides the save's outcome.
#[derive(Debug, Serialize)]
pub(crate) struct RestoredSave {
    /// The handle, still held: the save neither disposes nor requalifies it.
    pub(crate) handle: String,
    pub(crate) origin: &'static str,
    /// The stored source the handle holds (where it was reopened from).
    pub(crate) source: RestoredSource,
    /// The findings of the project as stored (what its native reopen gives),
    /// when the run was saved with it; null after a model-only save.
    pub(crate) findings: Option<HistoricalFindings>,
    /// `sha256:<hex>`: the envelope payload hash of what was stored.
    pub(crate) envelope_payload_hash: String,
    /// Whether the session's basis's stored envelope evidence was written.
    pub(crate) envelope_basis_kept: bool,
}

/// The command: [`persistence::reserve`] on the caller's executor, then
/// [`run_save`] in a blocking task. The reply is
/// `native_persistence::NativeProjectSave` with `restored`, as a JSON body.
pub(crate) async fn save_project(
    registry: &SolveJobRegistry,
    store: impl FnOnce() -> Result<PathBuf, String> + Send + 'static,
    body: &InvokeBody,
    headers: &HeaderMap,
) -> Result<tauri::ipc::Response, CommandError<String>> {
    let (raw, lease) = persistence::reserve(registry, body, headers, COMMAND)?;
    let registry = registry.clone();
    tauri::async_runtime::spawn_blocking(move || stored_read::respond_serialized(run_save(&registry, store, raw, lease, &SaveLimits::PRODUCTION, &crate::now_unix_seconds)))
        .await
        .unwrap_or_else(|_| Err(CommandError::Command(PERSISTENCE_FAILED.into())))
}

/// The handle's binding (module docs): its stored source, its two held texts
/// (shared) and the envelope basis the session names, established by the host.
struct Bound {
    source: RestoredSource,
    result: Arc<HeldResult>,
    record: Arc<HeldResult>,
    model_hash: Value,
    basis: EnvelopeBasis,
}

fn bound_restored(registry: &SolveJobRegistry, request: &Request) -> Result<Bound, String> {
    let handle = stored_read::shown(&request.job_id).into_owned();
    let refuse = |what: String| format!("{CUSTODY}: {what}; nothing was written.");
    let source = match registry.result_origin(&request.job_id) {
        None => return Err(refuse(format!("`{handle}` is not held (disposed, reset or unknown)"))),
        Some(ResultOrigin::Solve) => {
            return Err(refuse(format!("`{handle}` is a solve, not a reopened saved run; a fresh solve is saved from its job with its record inputs")));
        }
        Some(ResultOrigin::Restored(source)) if source.kind != SAVED_PROJECT_RUN => {
            return Err(refuse(format!(
                "`{handle}` holds a restored {}; only a project's reopened saved run is re-saved with a project",
                source.kind.replace('_', " ")
            )));
        }
        Some(ResultOrigin::Restored(source)) => source,
    };
    let expected = &request.expected_source;
    if expected.kind != source.kind
        || expected.project_id != source.project_id
        || expected.recording_id != source.recording_id
        || expected.result_sha256 != source.result_sha256
        || expected.record_sha256 != source.record_sha256
    {
        return Err(refuse(format!(
            "the stored source the session names (project `{}`, result {}, record {}) is not the one `{handle}` holds (project `{}`, result {}, record {})",
            stored_read::shown(&expected.project_id),
            stored_read::shown(&expected.result_sha256),
            stored_read::shown(expected.record_sha256.as_deref().unwrap_or("none")),
            stored_read::shown(&source.project_id),
            source.result_sha256,
            source.record_sha256.as_deref().unwrap_or("none")
        )));
    }
    let gone = |_| refuse(format!("`{handle}`'s stored run is no longer held"));
    let result = registry.held_result(&request.job_id).map_err(gone)?;
    let record = registry.restored_record(&request.job_id).map_err(gone)?;
    if hex(&sha256_in_place(result.text().as_bytes())) != source.result_sha256 || Some(hex(&sha256_in_place(record.text().as_bytes()))) != source.record_sha256 {
        return Err(refuse(format!("`{handle}`'s held texts are not its stored source's")));
    }
    let held = registry.restored_basis(&request.job_id).ok_or_else(|| refuse(format!("`{handle}` holds no stored project basis")))?;
    let named = &request.basis;
    let basis = held
        .envelopes
        .iter()
        .find(|basis| basis.payload_sha256 == named.payload_sha256 && same_js_data(&basis.evidence, &named.evidence))
        .cloned()
        .ok_or_else(|| refuse(format!("the envelope basis the session names is not one established for `{handle}` (its stored project's or a re-save's)")))?;
    Ok(Bound { source, result, record, model_hash: held.model_hash, basis })
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// A small held value as the legacy session sent it back: its parse as
/// JavaScript read it, read again by serde_json.
fn as_sent(mut value: Value) -> Value {
    build::js_numbers(&mut value);
    value
}

/// A held text's parse under its materialization's lease, admitted first.
fn parse(registry: &SolveJobRegistry, what: &str, text: &str) -> Result<(Value, Lease), CommandError<String>> {
    crate::stored_custody::parse_leased(registry, what, text).map_err(|error| match error {
        Ok(message) => refusal(format!("{CUSTODY}: {what} is not JSON this app reads ({message}); nothing was written.")),
        Err(pool) => refusal(pool),
    })
}

/// The blocking part (module docs); `now` is the row's time (unix seconds).
/// Order: the arguments, the handle's binding, the small fields, the held
/// texts' parses, the model document checks, then in one write transaction
/// the replacement guard, the shared stage (exact lengths, the model-only
/// save or the refusal, the withheld-run guard, the envelope hash and the
/// texts), the findings of what is stored (the parses end here), the shared
/// finish (the other texts, the writes, the reply) and the commit; then the
/// committed basis is recorded for the handle.
pub(crate) fn run_save(
    registry: &SolveJobRegistry,
    store: impl FnOnce() -> Result<PathBuf, String>,
    raw: Vec<u8>,
    copy: Lease,
    limits: &SaveLimits,
    now: &dyn Fn() -> Result<i64, String>,
) -> Result<Serialized, CommandError<String>> {
    #[cfg(test)]
    tests::command_gate::pass(tests::command_gate::key(registry, COMMAND));
    let memory = NativeLeaseMemory::new(registry);
    let copy_bytes = copy.bytes();
    let (mut args, args_lease) = command_ingress::admit_raw(registry, &raw, COMMAND, ArgsDecoding::Moved, |bytes| {
        let mut lease = copy;
        lease.grow_to(copy_bytes.saturating_add(bytes)).map_err(|refused| registry.refuse(refused))?;
        Ok(lease)
    })?;
    drop(raw);
    let request = decode(&mut args)?;
    drop(args);
    let bound = bound_restored(registry, &request).map_err(refusal)?;
    #[cfg(test)]
    tests::command_gate::pass(tests::command_gate::key(registry, "stored_save:bound"));
    let Request { job_id, create, project_id, project_name, model, editor_intents, proposal, selected_review_target, confirmed_revision, expected_base, .. } = request;
    let (project_id, project_name, expected_base) = if create {
        (crate::project_id_from_model(&model), crate::project_name_from_model(&model), None)
    } else {
        let missing = |what: &str| refusal(format!("INGRESS-ARGS-INVALID: a save names its project's {what}."));
        (project_id.ok_or_else(|| missing("id"))?, project_name.ok_or_else(|| missing("name"))?, expected_base)
    };
    let editor_intents = crate::normalized_editor_intents(editor_intents);
    let proposal = crate::normalized_proposal(proposal);
    let selected_review_target = crate::normalized_selected_review_target(selected_review_target);
    let model_hash = as_sent(bound.model_hash.clone());
    let basis = EnvelopeBasis { payload_sha256: bound.basis.payload_sha256.clone(), evidence: as_sent(bound.basis.evidence.clone()) };
    let small = [
        ("model_json", stored_read::serialized_length(&model).map_err(refusal)?),
        ("editor_intents_json", stored_read::serialized_length(&editor_intents).map_err(refusal)?),
        ("proposal_json", stored_read::serialized_length(&proposal).map_err(refusal)?),
        ("selected_review_target_json", stored_read::serialized_length(&selected_review_target).map_err(refusal)?),
        ("model_hash_json", stored_read::serialized_length(&model_hash).map_err(refusal)?),
    ];
    let fields = ProjectFields { project_id: &project_id, project_name: &project_name, model: &model, editor_intents: &editor_intents, proposal: &proposal, selected_review_target: &selected_review_target, model_hash: &model_hash, small: &small };
    let mut connection: Option<rusqlite::Connection> = None;
    let transaction: Transaction<'_>;
    let sqlite: stored_read::ReadReservation;
    let (path, migration, document_version, staged, findings) = {
        let (mut result, result_parse) = parse(registry, "the stored result", bound.result.text())?;
        let (mut record, record_parse) = parse(registry, "the stored record", bound.record.text())?;
        // The values as the legacy session sent them: JavaScript's numbers (`JSON.stringify` of
        // `JSON.parse`), so the envelope hash is the one it computed over them, and the stored
        // writer and the findings read what is stored.
        build::js_numbers(&mut result);
        build::js_numbers(&mut record);
        let working = build::working_lease(registry, &[&result, &record, &model, &editor_intents, &proposal, &selected_review_target, &model_hash])?;
        let document_version = crate::check_model_document_for_persist(&model).map_err(refusal)?;
        let path = store().map_err(refusal)?;
        let (opened, migration) = crate::open_project_store(&path).map_err(|error| refusal(stored_read::capped(error)))?;
        let opened = connection.insert(opened);
        if let Some(limit) = limits.sqlite_length {
            opened.set_limit(Limit::SQLITE_LIMIT_LENGTH, limit);
        }
        transaction = crate::immediate_transaction(opened).map_err(refusal)?;
        sqlite = stored_read::admit_sqlite_working_set(&transaction, &memory).map_err(refusal)?;
        crate::refuse_unconfirmed_replacement(&transaction, &project_id, confirmed_revision.as_deref(), expected_base.as_deref()).map_err(refusal)?;
        let staged = persistence::stage_save(&transaction, &SaveRun::Stored { result: &result, record: &record }, &fields, confirmed_revision.as_deref(), limits, now, &memory, Some(&basis))
            .map_err(refusal)?;
        // The findings of the project as stored: its values as serde_json reads the stored texts.
        let findings = if staged.saved {
            let project = SavedProject {
                model: &model,
                editor_intents: &editor_intents,
                proposal: &proposal,
                selected_review_target: &selected_review_target,
                model_hash: &model_hash,
                project_envelope_hash: &staged.envelope,
            };
            let mut found = validate::saved_run_findings(&project, &mut result, &record);
            if let Some(found) = found.as_mut() {
                validate::saved_run_validation(found, &mut result, &record);
            }
            found
        } else {
            None
        };
        // The parses and the working lease end here, before the statements run.
        drop((result, result_parse, record, record_parse, working));
        (path, migration, document_version, staged, findings)
    };
    let restored = RestoredSave {
        handle: job_id.clone(),
        origin: "restored_saved_run",
        source: bound.source.clone(),
        findings,
        envelope_payload_hash: staged.envelope_payload.clone(),
        envelope_basis_kept: basis.payload_sha256.as_deref() == Some(staged.envelope_payload.as_str()),
    };
    let committed = staged.saved.then(|| EnvelopeBasis { payload_sha256: Some(staged.envelope_payload.clone()), evidence: staged.envelope.clone() });
    let serialized = persistence::finish_save(&transaction, &fields, staged, create, path, &migration, document_version, &memory, Some(restored)).map_err(refusal)?;
    transaction.commit().map_err(|error| refusal(error.to_string()))?;
    drop(sqlite);
    // The session may re-save from the basis this save established.
    if let Some(basis) = committed {
        registry.add_restored_envelope_basis(&job_id, basis);
    }
    drop((bound, args_lease));
    Ok(serialized)
}

/// The stored writer's text of `value` (tests and the reply's oracle).
#[cfg(test)]
pub(crate) fn stored_text(value: &Value) -> String {
    let mut out = Vec::new();
    build::Writer::new(&mut out, build::Style::Stored).value(value);
    String::from_utf8(out).unwrap()
}

#[cfg(test)]
#[path = "stored_save_tests.rs"]
pub(crate) mod tests;
