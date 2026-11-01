//! C1 slice 6 (DEL-04-05 `c1-native-custody-plan.md`): saved and recorded
//! runs reopen into native custody. The stored result text and its stored
//! v0.3 record (with its `ResultRef`s) stay in this process as a restored
//! [`HeldResult`]; the webview receives a handle, the historical findings, the
//! record's header and the project's or run's small members, and reads rows
//! through the same native S, pages, semantics, navigator and CSV commands as
//! a fresh solve.
//!
//! - **Commands.** `open_local_project_in_native_custody {projectId?}` (as
//!   `open_local_project`: the project, or the most recently saved one) and
//!   `read_analysis_run_in_native_custody {projectId, recordingId}` (as
//!   `read_analysis_run`), raw args bodies under `command_ingress`. Each reply
//!   is a JSON body (Tauri delivers it parsed): never the result, the record
//!   or a `ResultRef`. The project reply is the stored project's members other
//!   than its run, exactly as `open_local_project` returns them, plus
//!   `stored_run` (`none`, `restored` or `refused`); the run reply is the
//!   verified summaries, the stored manifest's canonical text and `restored`.
//! - **Origin.** A restored handle (`restored-stored-result-N`) is registered
//!   with [`ResultOrigin::Restored`] and the stored row it came from, with the
//!   SHA-256 of the exact texts held. It has no intake or publication digest,
//!   attestation, worker grant, timestamp or job id of a solve; nothing here
//!   invents one. Fresh persistence, the fresh record preview and exact-route
//!   custody refuse it (`native_persistence`, `analysis_run_v03_build`,
//!   `solve_jobs::custody_binding`). Its standing is historical: the findings
//!   are `buildHistoricalRunContext`'s or `buildRecordedRunContext`'s
//!   (`analysis_run_v03_build::validate`), and nothing grants it Current.
//! - **Nothing is written.** Opening reads; the stored bytes, records, model
//!   versions, hashes and schema are never rewritten, migrated or upgraded.
//!   The store is opened as `open_local_project` and `read_analysis_run` open
//!   it (unchanged), and every read runs in one read transaction.
//! - **Refusals.** The project's model document is refused exactly as
//!   `open_local_project` refuses it (retired version, malformed slots), and a
//!   stored column that is not JSON fails the open with serde_json's own
//!   message, as before. The 64 MiB webview limit (#51) does not withhold a
//!   run held natively; the pool's refusal of the held bytes' leases, of a
//!   parse, or full retention does, by name (`STORED-RESULT-RESTORE-REFUSED`),
//!   with the model opened for editing and the stored run untouched. A
//!   recorded run keeps every integrity refusal of `read_analysis_run`.
//! - **Memory** (plan §8; no new budget or rule). Every lease is admitted
//!   before what it covers is allocated, each capped at min(per-invocation
//!   budget, 12 GiB) (`acquire_result_parse`):
//!   - the stored lengths are measured (`octet_length`, no value loaded)
//!     first; the held result's and record's leases (their texts' envelopes)
//!     and SQLite's copies of both are admitted before either is loaded;
//!   - each parse (result, record, manifest) is the prescan's materialization
//!     M, admitted before the parse; the findings' passes stream the
//!     canonical texts into SHA-256 and build no copy;
//!   - the reply is leased at its exact length before it is serialized.
//!
//!   The held leases become result leases and live with the bytes in the
//!   registry until disposal or session reset (exactly once, or when the
//!   last native reader drops its share). A restoration begun before a
//!   session reset is refused at registration and drops what it loaded.
//!   Unpriced, for slice 7: SQLite's page cache, journal and FTS state, the
//!   project's small columns (loaded as `open_local_project` loads them), the
//!   validator's working memory (one row's values at a time, the duplicate-id
//!   fingerprints within the fixed allowance), and Tauri's delivery copies of
//!   the reply after it returns.
use crate::analysis_run_v03_build::validate::{self, HistoricalFindings, RecordedRead, SavedProject};
use crate::analysis_runs::{self, NativeRunLoad};
use crate::command_ingress::{self, ArgsDecoding, CommandError};
use crate::ingress;
use crate::memory_pool::Lease;
use crate::solve_jobs::{sha256_in_place, HeldResult, RestoredSource, SolveJobError, SolveJobRegistry};
use crate::stored_read::{self, NativeLeaseMemory};
use rusqlite::{params, OptionalExtension};
use serde::Serialize;
use serde_json::Value;
use std::path::PathBuf;
use tauri::http::HeaderMap;
use tauri::ipc::InvokeBody;

pub(crate) const OPEN_COMMAND: &str = "open_local_project_in_native_custody";
pub(crate) const RUN_COMMAND: &str = "read_analysis_run_in_native_custody";
/// A stored run the pool or retention did not admit into native custody.
pub(crate) const RESTORE_REFUSED: &str = "STORED-RESULT-RESTORE-REFUSED";
/// A command that ended without an outcome (a panic in its task).
pub(crate) const REOPEN_FAILED: &str = "STORED-RESULT-REOPEN-FAILED";
/// The most bytes of arguments either command takes.
pub(crate) const MAX_ARGS_BYTES: usize = 64 * 1024;
/// The largest record header a reply carries: the record's members other
/// than its rows are model-scale; a stored record whose other members exceed
/// this is shown without its header (`record_header_bytes`).
pub(crate) const RECORD_HEADER_LIMIT_BYTES: u64 = 4 << 20;
/// The record members a header leaves out: its rows, its diagnostics (the
/// producer's grow with the result) and the copied evidence.
/// The source the record's rule-pack reference diagnostics name (`analysis_runs::rule_pack_ref_diagnostic`).
const RULE_PACK_REF_DIAGNOSTIC_SOURCE: &str = "desktop/solve-rule-pack-resolution";
const ROW_SCALE_MEMBERS: [&str; 5] = ["result_refs", "diagnostics", "contract_evidence", "source_block_recovery", "retained_precision"];

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn envelope(bytes: u64) -> u64 {
    ingress::allocation_envelope(bytes).unwrap_or(u64::MAX)
}

/// The pool's refusal of a lease for `what`, as the native reads name it.
fn pool_refusal(what: &str, bytes: u64, error: SolveJobError) -> String {
    match error {
        SolveJobError::ResourceUnavailable { refusal, .. } => format!(
            "{}: {what} needs {bytes} bytes of the memory pool, which refused them ({}); nothing more was read.",
            stored_read::STORE_READ_MEMORY_UNAVAILABLE,
            serde_json::to_string(&refusal).unwrap_or_else(|_| "refused".to_string())
        ),
        SolveJobError::InvalidRequest { message } => format!("{}: {what}: {message}", stored_read::STORE_READ_MEMORY_UNAVAILABLE),
    }
}

/// A lease of `bytes` for `what`, admitted before what it covers is allocated.
fn lease(registry: &SolveJobRegistry, what: &str, bytes: u64) -> Result<Lease, String> {
    registry.acquire_result_parse(bytes).map_err(|error| pool_refusal(what, bytes, error))
}

/// A stored text parsed under its materialization's lease, admitted first.
/// `Err(Ok(message))`: the text is not JSON (serde_json's message, as the
/// stores report it); `Err(Err(refusal))`: the pool refused the parse.
fn parse_leased(registry: &SolveJobRegistry, what: &str, text: &str) -> Result<(Value, Lease), Result<String, String>> {
    let bound = match ingress::prescan(text.as_bytes()) {
        Ok(bound) => bound,
        // The prescan refuses without allocating; serde_json's message comes
        // from a parse that builds no tree.
        Err(_) => return Err(Ok(serde_json::from_str::<serde::de::IgnoredAny>(text).err().map_or_else(|| "invalid JSON".to_string(), |error| error.to_string()))),
    };
    let reading = lease(registry, &format!("parsing {what}"), bound.materialization_bytes).map_err(Err)?;
    match serde_json::from_str::<Value>(text) {
        Ok(value) => Ok((value, reading)),
        Err(error) => Err(Ok(error.to_string())),
    }
}

/// The stored record's header: the envelope with its `analysis_run` members
/// other than [`ROW_SCALE_MEMBERS`], and their counts, measured before it is
/// built; `Err(bytes)` above [`RECORD_HEADER_LIMIT_BYTES`]. A projection of
/// the stored record as stored, never a rebuilt one.
pub(crate) fn record_header(record: &Value) -> Result<Value, u64> {
    struct Counter(u64);
    impl std::io::Write for Counter {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            self.0 += bytes.len() as u64;
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let measure = |value: &Value| {
        let mut counter = Counter(0);
        serde_json::to_writer(&mut counter, value).expect("a Value serializes");
        counter.0
    };
    let Value::Object(map) = record else {
        let bytes = measure(record);
        if bytes > RECORD_HEADER_LIMIT_BYTES {
            return Err(bytes);
        }
        return Ok(serde_json::json!({"record": record, "rule_pack_ref_diagnostics": [], "result_ref_count": null, "diagnostic_count": null, "carried_members": []}));
    };
    let mut bytes = 0u64;
    for (key, value) in map {
        match (key.as_str(), value) {
            ("analysis_run", Value::Object(run)) => {
                for (member, value) in run {
                    if !ROW_SCALE_MEMBERS.contains(&member.as_str()) {
                        bytes = bytes.saturating_add(measure(value)).saturating_add(member.len() as u64 + 4);
                    }
                }
            }
            _ => bytes = bytes.saturating_add(measure(value)).saturating_add(key.len() as u64 + 4),
        }
    }
    if bytes > RECORD_HEADER_LIMIT_BYTES {
        return Err(bytes);
    }
    let mut header = serde_json::Map::new();
    for (key, value) in map {
        match (key.as_str(), value) {
            ("analysis_run", Value::Object(run)) => {
                let mut kept = serde_json::Map::new();
                for (member, value) in run {
                    if !ROW_SCALE_MEMBERS.contains(&member.as_str()) {
                        kept.insert(member.clone(), value.clone());
                    }
                }
                header.insert(key.clone(), Value::Object(kept));
            }
            _ => {
                header.insert(key.clone(), value.clone());
            }
        }
    }
    let run = map.get("analysis_run").and_then(Value::as_object);
    let count = |member: &str| run.and_then(|run| run.get(member)).and_then(Value::as_array).map(Vec::len);
    let carried: Vec<&str> = ["contract_evidence", "source_block_recovery", "retained_precision"].into_iter().filter(|member| run.is_some_and(|run| run.contains_key(*member))).collect();
    // The record's rule-pack reference diagnostics (model-scale: one per excluded reference), as the
    // fresh preview header carries them, for the run's rule-pack references.
    let rule_pack_diagnostics: Vec<Value> = run
        .and_then(|run| run.get("diagnostics"))
        .and_then(Value::as_array)
        .map(|items| items.iter().filter(|item| item.pointer("/source_annotation/source").and_then(Value::as_str) == Some(RULE_PACK_REF_DIAGNOSTIC_SOURCE)).cloned().collect())
        .unwrap_or_default();
    let mut wrapper = serde_json::Map::new();
    wrapper.insert("record".into(), Value::Object(header));
    wrapper.insert("rule_pack_ref_diagnostics".into(), Value::Array(rule_pack_diagnostics));
    wrapper.insert("result_ref_count".into(), count("result_refs").map_or(Value::Null, Value::from));
    wrapper.insert("diagnostic_count".into(), count("diagnostics").map_or(Value::Null, Value::from));
    wrapper.insert("carried_members".into(), Value::from(carried));
    Ok(Value::Object(wrapper))
}

/// What a reopened run tells the webview, besides its findings.
#[derive(Debug, Serialize)]
pub(crate) struct RestoredRun {
    /// The handle the native result commands read (`restored-stored-result-N`).
    pub(crate) handle: String,
    /// `restored_saved_run` or `restored_recorded_run`: never a solve.
    pub(crate) origin: &'static str,
    pub(crate) source: RestoredSource,
    pub(crate) result_bytes: u64,
    pub(crate) record_bytes: u64,
    /// Whether the stored result / record is not JSON `null`.
    pub(crate) result_present: bool,
    pub(crate) record_present: bool,
    pub(crate) findings: HistoricalFindings,
    /// [`record_header`]'s `{record, result_ref_count, diagnostic_count,
    /// carried_members}`, or null with `record_header_bytes` above the limit.
    pub(crate) record_header: Value,
    pub(crate) record_header_bytes: Option<u64>,
}

/// The project's stored run, as the open found it.
#[derive(Debug, Serialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub(crate) enum StoredRun {
    /// Neither a result nor a record is stored.
    None,
    Restored(RestoredRun),
    /// Not admitted into native custody; nothing of it was loaded or changed.
    Refused { code: &'static str, message: String, result_bytes: u64, analysis_run_bytes: u64 },
}

/// `open_local_project_in_native_custody`'s reply: `open_local_project`'s
/// envelope without `mechanics_result` and `analysis_run`, and the run.
#[derive(Debug, Serialize)]
pub(crate) struct NativeProjectEnvelope {
    pub(crate) summary: crate::LocalProjectSummary,
    pub(crate) model: Value,
    pub(crate) editor_intents: Value,
    pub(crate) proposal: Value,
    pub(crate) selected_review_target: Value,
    pub(crate) model_hash: Value,
    pub(crate) project_envelope_hash: Value,
    pub(crate) model_document_version: Value,
    pub(crate) stored_run: StoredRun,
}

/// `read_analysis_run_in_native_custody`'s reply.
#[derive(Debug, Serialize)]
pub(crate) struct NativeRunRead {
    pub(crate) run: analysis_runs::AnalysisRunSummary,
    pub(crate) state: crate::model_states::ModelStateSummary,
    pub(crate) input_manifest_canonical: String,
    pub(crate) restored: RestoredRun,
}

/// A stored text known to be JSON `null` from its length and (only when 4
/// bytes long) its text.
fn null_text(length: i64, small: &Option<String>) -> bool {
    length == 4 && small.as_deref() == Some("null")
}

type ProjectRow = (String, String, String, String, String, String, String, String, i64, i64, Option<String>, Option<String>);

/// The project's small columns, as `open_local_project` loads them, and the
/// run's lengths; the run's texts only when they are 4 bytes (to tell `null`).
const PROJECT_COLUMNS: &str = "project_id, project_name, model_json, editor_intents_json, proposal_json, selected_review_target_json, \
    model_hash_json, project_envelope_hash_json, octet_length(mechanics_result_json), octet_length(analysis_run_json), \
    CASE WHEN octet_length(mechanics_result_json) = 4 THEN mechanics_result_json END, \
    CASE WHEN octet_length(analysis_run_json) = 4 THEN analysis_run_json END";

fn project_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ProjectRow> {
    Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?, row.get(6)?, row.get(7)?, row.get(8)?, row.get(9)?, row.get(10)?, row.get(11)?))
}

/// The held texts and leases of a run being restored.
struct Loaded {
    result: String,
    record: String,
    result_lease: Lease,
    record_lease: Lease,
}

/// The blocking part of the project open (module docs). `epoch` is the
/// session's reset epoch when the command began.
pub(crate) fn open_project_in_store(registry: &SolveJobRegistry, path: PathBuf, project_id: Option<&str>, epoch: u64) -> Result<Option<NativeProjectEnvelope>, String> {
    let (mut connection, migration) = crate::open_project_store(&path)?;
    let transaction = stored_read::read_transaction(&mut connection)?;
    let row = match project_id {
        Some(id) => transaction.query_row(&format!("SELECT {PROJECT_COLUMNS} FROM local_projects WHERE project_id = ?1"), params![id], project_row),
        None => transaction.query_row(&format!("SELECT {PROJECT_COLUMNS} FROM local_projects ORDER BY updated_at_unix DESC, project_id ASC LIMIT 1"), [], project_row),
    }
    .optional()
    .map_err(|error| error.to_string())?;
    let Some((id, name, model_json, editor_json, proposal_json, target_json, model_hash_json, envelope_json, result_length, record_length, small_result, small_record)) = row else {
        return Ok(None);
    };
    let parse = |text: &str| serde_json::from_str::<Value>(text).map_err(|error| error.to_string());
    let model = parse(&model_json)?;
    let editor_intents = parse(&editor_json)?;
    let proposal = parse(&proposal_json)?;
    let selected_review_target = parse(&target_json)?;
    let lengths = (u64::try_from(result_length).unwrap_or(0), u64::try_from(record_length).unwrap_or(0));
    let run_stored = !(null_text(result_length, &small_result) && null_text(record_length, &small_record));
    // The run, admitted and loaded before the remaining columns are parsed
    // (the order `open_local_project` reports a column that is not JSON in).
    let loaded: Result<Option<Loaded>, String> = if run_stored {
        (|| {
            let what = format!("project `{}`'s stored run", stored_read::shown(&id));
            let result_lease = lease(registry, &format!("{what}'s result ({} bytes)", lengths.0), envelope(lengths.0))?;
            let record_lease = lease(registry, &format!("{what}'s record ({} bytes)", lengths.1), envelope(lengths.1))?;
            let sqlite = lease(registry, &format!("SQLite's copies of {what}"), envelope(lengths.0).saturating_add(envelope(lengths.1)).saturating_mul(2))?;
            let (result, record): (String, String) = transaction
                .query_row("SELECT mechanics_result_json, analysis_run_json FROM local_projects WHERE project_id = ?1", params![id], |row| Ok((row.get(0)?, row.get(1)?)))
                .map_err(|error| error.to_string())?;
            drop(sqlite);
            Ok(Some(Loaded { result, record, result_lease, record_lease }))
        })()
    } else {
        Ok(None)
    };
    // A refused restoration keeps the model editable: the open goes on without the run.
    let (loaded, mut refusal) = match loaded {
        Ok(loaded) => (loaded, None),
        Err(message) => (None, Some(message)),
    };
    // Each stored run text that is not JSON fails the open, as `open_local_project` fails it.
    let mut parsed: Option<(Value, Lease, Value, Lease)> = None;
    if let Some(run) = &loaded {
        match parse_leased(registry, "the stored result", &run.result) {
            Err(Ok(message)) => return Err(message),
            Err(Err(pool)) => refusal = Some(pool),
            Ok((result, result_parse)) => match parse_leased(registry, "the stored record", &run.record) {
                Err(Ok(message)) => return Err(message),
                Err(Err(pool)) => refusal = Some(pool),
                Ok((record, record_parse)) => parsed = Some((result, result_parse, record, record_parse)),
            },
        }
    }
    let model_hash = parse(&model_hash_json)?;
    let project_envelope_hash = parse(&envelope_json)?;
    // DEL-02-05 CLM-003 and W-13 §5.3, as `open_local_project`.
    let status = crate::evaluate_model_document(&model);
    if status.is_refused() {
        return Err(crate::refuse_model_document(&status));
    }
    crate::refuse_malformed_model_slots(&model)?;
    let model_document_version = serde_json::to_value(status).map_err(|error| error.to_string())?;
    drop(transaction);
    let summary_of = |result: &Value, record: &Value| {
        crate::project_summary(
            id.clone(),
            name.clone(),
            &model,
            path.clone(),
            &migration,
            &editor_intents,
            &proposal,
            &selected_review_target,
            result,
            record,
            &model_hash,
            &project_envelope_hash,
            "Opened local SQLite project snapshot.".to_string(),
        )
    };
    let (summary, stored_run) = match (loaded, parsed) {
        (Some(run), Some((mut result, result_parse, record, record_parse))) => {
            let summary = summary_of(&result, &record);
            let project = SavedProject {
                model: &model,
                editor_intents: &editor_intents,
                proposal: &proposal,
                selected_review_target: &selected_review_target,
                model_hash: &model_hash,
                project_envelope_hash: &project_envelope_hash,
            };
            let result_present = !result.is_null();
            let record_present = !record.is_null();
            let header = record_header(&record);
            let mut found = validate::saved_run_findings(&project, &mut result, &record).expect("a stored run holds a result or a record");
            validate::saved_run_validation(&mut found, &mut result, &record);
            drop((result, result_parse, record, record_parse));
            let source = RestoredSource {
                kind: "saved_project_run",
                project_id: id.clone(),
                recording_id: None,
                result_sha256: hex(&sha256_in_place(run.result.as_bytes())),
                record_sha256: Some(hex(&sha256_in_place(run.record.as_bytes()))),
            };
            let (result_bytes, record_bytes) = (run.result.len() as u64, run.record.len() as u64);
            match registry.restore(epoch, source.clone(), HeldResult::restored(run.result, run.result_lease), Some(HeldResult::restored(run.record, run.record_lease))) {
                Ok(handle) => {
                    let (record_header, record_header_bytes) = match header {
                        Ok(header) => (header, None),
                        Err(bytes) => (Value::Null, Some(bytes)),
                    };
                    (
                        summary,
                        StoredRun::Restored(RestoredRun {
                            handle,
                            origin: "restored_saved_run",
                            source,
                            result_bytes,
                            record_bytes,
                            result_present,
                            record_present,
                            findings: found,
                            record_header,
                            record_header_bytes,
                        }),
                    )
                }
                Err(error) => (summary_of(&Value::Null, &Value::Null), refused(&id, lengths, &restore_refusal(error))),
            }
        }
        (None, _) if !run_stored => (summary_of(&Value::Null, &Value::Null), StoredRun::None),
        _ => (summary_of(&Value::Null, &Value::Null), refused(&id, lengths, refusal.as_deref().unwrap_or("not admitted"))),
    };
    Ok(Some(NativeProjectEnvelope {
        summary,
        model,
        editor_intents,
        proposal,
        selected_review_target,
        model_hash,
        project_envelope_hash,
        model_document_version,
        stored_run,
    }))
}

fn restore_refusal(error: SolveJobError) -> String {
    match error {
        SolveJobError::ResourceUnavailable { refusal, .. } => format!("the memory pool refused to hold it ({})", serde_json::to_string(&refusal).unwrap_or_else(|_| "refused".into())),
        SolveJobError::InvalidRequest { message } => message,
    }
}

fn refused(project_id: &str, (result_bytes, analysis_run_bytes): (u64, u64), why: &str) -> StoredRun {
    StoredRun::Refused {
        code: RESTORE_REFUSED,
        message: stored_read::capped(format!(
            "{RESTORE_REFUSED}: project `{}`'s stored run (result {}, analysis-run record {}) was not opened: {why} The stored run is kept unchanged and the model is open for editing; saving the project in place is refused until a new solve replaces the run (model edits can be kept with File > Save Model Document…).",
            stored_read::shown(project_id),
            crate::solve_jobs::display_size(result_bytes),
            crate::solve_jobs::display_size(analysis_run_bytes),
        )),
        result_bytes,
        analysis_run_bytes,
    }
}

/// The recorded run's load into native custody (`NativeRunLoad`).
struct RunLoad<'r> {
    registry: &'r SolveJobRegistry,
    leases: Option<(Lease, Lease)>,
    record_text: Option<String>,
}

impl NativeRunLoad for RunLoad<'_> {
    fn admit(&mut self, result_bytes: u64, record_bytes: u64) -> Result<(), String> {
        let result = lease(self.registry, &format!("the recorded run's result ({result_bytes} bytes)"), envelope(result_bytes))?;
        let record = lease(self.registry, &format!("the recorded run's record ({record_bytes} bytes)"), envelope(record_bytes))?;
        self.leases = Some((result, record));
        Ok(())
    }

    fn record_text(&mut self, text: String) {
        self.record_text = Some(text);
    }
}

/// The blocking part of the recorded run's reopen (module docs): the store's
/// verified read (every integrity refusal of `read_analysis_run`, without
/// its webview limit), then the findings, then the handle.
pub(crate) fn read_run_in_store(registry: &SolveJobRegistry, path: PathBuf, project_id: &str, recording_id: &str, epoch: u64) -> Result<NativeRunRead, String> {
    let memory = NativeLeaseMemory::new(registry);
    let mut load = RunLoad { registry, leases: None, record_text: None };
    let read = analysis_runs::read_run_in_store_core(&path, project_id, recording_id, &memory, u64::MAX, Some(&mut load))?;
    let (read, guard) = read.into_custody();
    let analysis_runs::AnalysisRunRead { run, state, record, result_json, input_manifest_canonical } = read;
    let (Some((result_lease, record_lease)), Some(record_text)) = (load.leases, load.record_text) else {
        return Err(format!("{REOPEN_FAILED}: the recorded run's texts were not loaded."));
    };
    let parsed_result = match parse_leased(registry, "the recorded run's result", &result_json) {
        Ok(parsed) => Some(parsed),
        Err(Ok(_)) => None,
        Err(Err(refusal)) => return Err(refusal),
    };
    let parsed_manifest = match parse_leased(registry, "the recorded run's input manifest", &input_manifest_canonical) {
        Ok(parsed) => Some(parsed),
        Err(Ok(_)) => None,
        Err(Err(refusal)) => return Err(refusal),
    };
    let context = RecordedRead {
        state_id: &state.state_id,
        state_project_id: &state.project_id,
        state_model_hash: &state.model_hash,
        run_state_id: &run.state_id,
        producer_run_id: &run.producer_run_id,
        run_record_hash: &run.run_record_hash,
    };
    let manifest = parsed_manifest.as_ref().map(|(manifest, _)| manifest);
    let (mut result_value, result_parse) = match parsed_result {
        Some((value, parse)) => (Some(value), Some(parse)),
        None => (None, None),
    };
    let mut found = validate::recorded_run_findings(&context, result_value.as_mut(), manifest, &record);
    if let Some(result) = result_value.as_mut() {
        validate::recorded_run_validation(&mut found, &context, result, manifest, &record);
    }
    let header = record_header(&record);
    let result_present = result_value.as_ref().is_some_and(|value| !value.is_null()) || result_value.is_none();
    drop((result_value, result_parse, parsed_manifest, record));
    drop(guard);
    let source = RestoredSource {
        kind: "recorded_run",
        project_id: run.project_id.clone(),
        recording_id: Some(run.recording_id.clone()),
        result_sha256: hex(&sha256_in_place(result_json.as_bytes())),
        record_sha256: Some(hex(&sha256_in_place(record_text.as_bytes()))),
    };
    let (result_bytes, record_bytes) = (result_json.len() as u64, record_text.len() as u64);
    let handle = registry
        .restore(epoch, source.clone(), HeldResult::restored(result_json, result_lease), Some(HeldResult::restored(record_text, record_lease)))
        .map_err(|error| format!("{RESTORE_REFUSED}: recorded run `{}` was not opened: {}", stored_read::shown(recording_id), restore_refusal(error)))?;
    let (record_header, record_header_bytes) = match header {
        Ok(header) => (header, None),
        Err(bytes) => (Value::Null, Some(bytes)),
    };
    Ok(NativeRunRead {
        run,
        state,
        input_manifest_canonical,
        restored: RestoredRun {
            handle,
            origin: "restored_recorded_run",
            source,
            result_bytes,
            record_bytes,
            result_present,
            record_present: true,
            findings: found,
            record_header,
            record_header_bytes,
        },
    })
}

/// The reply as a JSON body, its exact length leased before it is built.
fn reply<T: Serialize>(registry: &SolveJobRegistry, value: &T) -> Result<(String, Lease), String> {
    struct Counter(u64);
    impl std::io::Write for Counter {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            self.0 += bytes.len() as u64;
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut counter = Counter(0);
    serde_json::to_writer(&mut counter, value).map_err(|error| error.to_string())?;
    let held = lease(registry, "the reply", envelope(counter.0))?;
    let mut text = Vec::with_capacity(usize::try_from(counter.0).unwrap_or(usize::MAX));
    serde_json::to_writer(&mut text, value).map_err(|error| error.to_string())?;
    Ok((String::from_utf8(text).expect("serde_json writes UTF-8"), held))
}

/// The executor's part: the header check, the small copy's reservation and
/// the copy.
fn reserve(registry: &SolveJobRegistry, body: &InvokeBody, headers: &HeaderMap, command: &'static str) -> Result<(Vec<u8>, Lease), CommandError<String>> {
    let raw = command_ingress::raw_args_body(body, headers, command)?;
    if raw.len() > MAX_ARGS_BYTES {
        return Err(CommandError::Command(format!("INGRESS-ARGS-TOO-LARGE: `{command}` takes at most {MAX_ARGS_BYTES} bytes of arguments; {} were sent.", raw.len())));
    }
    let lease = registry.acquire_interactive_ingress_capped(envelope(raw.len() as u64))?;
    Ok((raw.to_vec(), lease))
}

fn admit(registry: &SolveJobRegistry, raw: &[u8], copy: Lease, command: &'static str) -> Result<(command_ingress::Args, Lease), CommandError<String>> {
    let copy_bytes = copy.bytes();
    let (args, lease) = command_ingress::admit_raw(registry, raw, command, ArgsDecoding::Moved, |bytes| {
        let mut lease = copy;
        lease.grow_to(copy_bytes.saturating_add(bytes)).map_err(|refusal| registry.refuse(refusal))?;
        Ok(lease)
    })?;
    Ok((args, lease))
}

/// `open_local_project_in_native_custody`, after the executor's part.
pub(crate) fn run_open(registry: &SolveJobRegistry, store: impl FnOnce() -> Result<PathBuf, String>, raw: Vec<u8>, copy: Lease, epoch: u64) -> Result<tauri::ipc::Response, CommandError<String>> {
    let (mut args, args_lease) = admit(registry, &raw, copy, OPEN_COMMAND)?;
    drop(raw);
    let project_id: Option<String> = args.decode("projectId")?;
    drop(args);
    let path = store().map_err(CommandError::Command)?;
    let opened = open_project_in_store(registry, path, project_id.as_deref(), epoch).map_err(|error| CommandError::Command(stored_read::capped(error)))?;
    let (text, held) = reply(registry, &opened).map_err(CommandError::Command)?;
    let response = tauri::ipc::Response::new(text);
    drop((held, args_lease));
    Ok(response)
}

/// `read_analysis_run_in_native_custody`, after the executor's part.
pub(crate) fn run_read(registry: &SolveJobRegistry, store: impl FnOnce() -> Result<PathBuf, String>, raw: Vec<u8>, copy: Lease, epoch: u64) -> Result<tauri::ipc::Response, CommandError<String>> {
    let (mut args, args_lease) = admit(registry, &raw, copy, RUN_COMMAND)?;
    drop(raw);
    let project_id: String = args.decode("projectId")?;
    let recording_id: String = args.decode("recordingId")?;
    drop(args);
    let path = store().map_err(CommandError::Command)?;
    let read = read_run_in_store(registry, path, &project_id, &recording_id, epoch).map_err(|error| CommandError::Command(stored_read::capped(error)))?;
    let (text, held) = reply(registry, &read).map_err(CommandError::Command)?;
    let response = tauri::ipc::Response::new(text);
    drop((held, args_lease));
    Ok(response)
}

/// The command entry: the executor's part, then the blocking part.
pub(crate) async fn open_project(
    registry: &SolveJobRegistry,
    store: impl FnOnce() -> Result<PathBuf, String> + Send + 'static,
    body: &InvokeBody,
    headers: &HeaderMap,
) -> Result<tauri::ipc::Response, CommandError<String>> {
    let epoch = registry.reset_epoch();
    let (raw, lease) = reserve(registry, body, headers, OPEN_COMMAND)?;
    let registry = registry.clone();
    tauri::async_runtime::spawn_blocking(move || run_open(&registry, store, raw, lease, epoch))
        .await
        .unwrap_or_else(|_| Err(CommandError::Command(format!("{REOPEN_FAILED}: the open ended without an outcome."))))
}

/// The command entry for a recorded run.
pub(crate) async fn read_run(
    registry: &SolveJobRegistry,
    store: impl FnOnce() -> Result<PathBuf, String> + Send + 'static,
    body: &InvokeBody,
    headers: &HeaderMap,
) -> Result<tauri::ipc::Response, CommandError<String>> {
    let epoch = registry.reset_epoch();
    let (raw, lease) = reserve(registry, body, headers, RUN_COMMAND)?;
    let registry = registry.clone();
    tauri::async_runtime::spawn_blocking(move || run_read(&registry, store, raw, lease, epoch))
        .await
        .unwrap_or_else(|_| Err(CommandError::Command(format!("{REOPEN_FAILED}: the read ended without an outcome."))))
}

#[cfg(test)]
#[path = "stored_custody_tests.rs"]
mod tests;
