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
//!   `solve_jobs::custody_binding`). A saved project run's handle also holds
//!   its stored project's model hash and envelope evidence, the basis a
//!   project save re-saves it with (`stored_save`). Its standing is historical: the findings
//!   are `buildHistoricalRunContext`'s or `buildRecordedRunContext`'s
//!   (`analysis_run_v03_validate`), and nothing grants it Current.
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
//!   before what it covers is allocated, in one running total per command
//!   (`stored_read::NativeLeaseMemory`) started from the argument lease and
//!   kept within min(the per-invocation budget that lease was admitted
//!   under, 12 GiB); each delta is reserved there before the pool admits it
//!   (adaptive, as `acquire_result_parse`), and a reservation above it is
//!   refused by name before its allocation:
//!   - the stored lengths are measured (`octet_length`, no value loaded)
//!     first; the held result's and record's leases (their texts' envelopes)
//!     and SQLite's copies of both are admitted before either is loaded;
//!   - each parse (result, record, manifest) is the prescan's materialization
//!     M, admitted before the parse; the findings' passes stream the
//!     canonical texts into SHA-256 and build no copy;
//!   - a recorded run's record is loaded once and its result parsed once
//!     (RT-U4): the store's verified read hands over the record text it
//!     verified and the result parse it checked, under its own reservations
//!     (`analysis_runs::NativeRunLoad`), rather than loading or parsing either
//!     again;
//!   - the complete record header is capped before allocation, then its text
//!     and parse are leased before allocation; the parse lease lives through
//!     reply serialization;
//!   - the reply is leased at its exact length before it is serialized.
//!
//!   The held leases become result leases and live with the bytes in the
//!   registry until disposal or session reset (exactly once, or when the
//!   last native reader drops its share). They are handed over whole, never
//!   released and admitted again, and leave the command's total only when
//!   the registry establishes the handle (`HeldResult::restored_in_custody`);
//!   every other lease of the command stays counted until it drops. A handle
//!   registered and then not carried by the reply (its lease or
//!   serialization refused, a panic) is disposed ([`Unpublished`]). A
//!   restoration begun before a session reset is refused at registration and
//!   drops what it loaded.
//!   Unpriced, for slice 7: SQLite's page cache, journal and FTS state, the
//!   project's small columns (loaded as `open_local_project` loads them), the
//!   validator's working memory (one row's values at a time, the duplicate-id
//!   fingerprints within the fixed allowance), and Tauri's delivery copies of
//!   the reply after it returns.
use crate::analysis_run_v03_validate::{self as validate, HistoricalFindings, RecordedRead, SavedProject};
use crate::analysis_runs::{self, NativeRunLoad};
use crate::command_ingress::{self, ArgsDecoding, CommandError};
use crate::ingress;
use crate::memory_pool::{CommandLease, Lease};
use crate::solve_jobs::{sha256_in_place, EnvelopeBasis, HeldResult, RestoredBasis, RestoredSource, SolveJobError, SolveJobRegistry};
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

/// A lease of `bytes` for `what` in the command's ledger, admitted before
/// what it covers is allocated; refused by name (above the command's budget,
/// or by the pool) with nothing more read.
fn lease(memory: &NativeLeaseMemory<'_>, what: &str, bytes: u64) -> Result<CommandLease, String> {
    memory.named_lease(what, bytes, "nothing more was read.")
}

/// A stored text parsed under its materialization's lease, admitted first.
/// `Err(Ok(message))`: the text is not JSON (serde_json's message, as the
/// stores report it); `Err(Err(refusal))`: the pool or the command's budget
/// refused the parse.
pub(crate) fn parse_leased(memory: &NativeLeaseMemory<'_>, what: &str, text: &str) -> Result<(Value, CommandLease), Result<String, String>> {
    let bound = match ingress::prescan(text.as_bytes()) {
        Ok(bound) => bound,
        // The prescan refuses without allocating; serde_json's message comes
        // from a parse that builds no tree.
        Err(_) => return Err(Ok(serde_json::from_str::<serde::de::IgnoredAny>(text).err().map_or_else(|| "invalid JSON".to_string(), |error| error.to_string()))),
    };
    let reading = lease(memory, &format!("parsing {what}"), bound.materialization_bytes).map_err(Err)?;
    match serde_json::from_str::<Value>(text) {
        Ok(value) => Ok((value, reading)),
        Err(error) => Err(Ok(error.to_string())),
    }
}

/// A borrowed projection: measuring the complete header (including matching
/// diagnostics and escaped keys) allocates no cloned record or diagnostic list.
struct HeaderRecord<'a>(&'a Value, bool);
impl Serialize for HeaderRecord<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let Value::Object(map) = self.0 else { return self.0.serialize(serializer); };
        let mut out = serializer.serialize_map(None)?;
        for (key, value) in map {
            if self.1 && ROW_SCALE_MEMBERS.contains(&key.as_str()) { continue; }
            if !self.1 && key == "analysis_run" && value.is_object() {
                out.serialize_entry(key, &HeaderRecord(value, true))?;
            } else {
                out.serialize_entry(key, value)?;
            }
        }
        out.end()
    }
}
struct HeaderDiagnostics<'a>(Option<&'a Vec<Value>>);
impl Serialize for HeaderDiagnostics<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut out = serializer.serialize_seq(None)?;
        for item in self.0.into_iter().flatten() {
            if item.pointer("/source_annotation/source").and_then(Value::as_str) == Some(RULE_PACK_REF_DIAGNOSTIC_SOURCE) {
                out.serialize_element(item)?;
            }
        }
        out.end()
    }
}
#[derive(Serialize)]
struct Header<'a> {
    record: HeaderRecord<'a>,
    rule_pack_ref_diagnostics: HeaderDiagnostics<'a>,
    result_ref_count: Option<usize>,
    diagnostic_count: Option<usize>,
    carried_members: Vec<&'static str>,
}

/// The entire projected header is measured before allocation. Above the cap,
/// omit it; otherwise lease both its temporary text and its parsed Value before
/// allocating either. The Value's lease travels with the reply until serialized.
fn record_header(memory: &NativeLeaseMemory<'_>, record: &Value) -> Result<(Value, Option<u64>, Option<CommandLease>), String> {
    let run = record.get("analysis_run").and_then(Value::as_object);
    let items = |key: &str| run.and_then(|run| run.get(key)).and_then(Value::as_array);
    let header = Header {
        record: HeaderRecord(record, false),
        rule_pack_ref_diagnostics: HeaderDiagnostics(items("diagnostics")),
        result_ref_count: items("result_refs").map(Vec::len),
        diagnostic_count: items("diagnostics").map(Vec::len),
        carried_members: ["contract_evidence", "source_block_recovery", "retained_precision"].into_iter()
            .filter(|member| run.is_some_and(|run| run.contains_key(*member))).collect(),
    };
    struct Counter(u64);
    impl std::io::Write for Counter {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            self.0 = self.0.saturating_add(bytes.len() as u64);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
    }
    let mut counter = Counter(0);
    serde_json::to_writer(&mut counter, &header).map_err(|error| error.to_string())?;
    if counter.0 > RECORD_HEADER_LIMIT_BYTES {
        return Ok((Value::Null, Some(counter.0), None));
    }
    let (text, _text_lease) = reply(memory, &header)?;
    let (value, held) = parse_leased(memory, "the restored record header", &text)
        .map_err(|error| match error { Ok(message) | Err(message) => message })?;
    Ok((value, None, Some(held)))
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
    /// RT-A4 §10.4: under the recorded table the findings name
    /// (`recorded_table_read`), the stored statement's recorded dispatch with
    /// no model (`recordedReaderDispatch(result, binding)`: route, binding,
    /// refusal and its standing as recorded), the route a display surface
    /// classifies it under; null otherwise. Display only: no standing.
    pub(crate) recorded_dispatch: Option<Value>,
    /// [`record_header`]'s `{record, result_ref_count, diagnostic_count,
    /// carried_members}`, or null with `record_header_bytes` above the limit.
    pub(crate) record_header: Value,
    pub(crate) record_header_bytes: Option<u64>,
    #[serde(skip)]
    _header_lease: Option<CommandLease>,
    /// The handle until the reply naming it is serialized ([`Unpublished`]).
    #[serde(skip)]
    unpublished: Unpublished,
}

/// A restored handle this command registered and its reply has not yet
/// carried: dropped unpublished (the reply's lease or serialization refused,
/// a panic or unwinding after registration), it disposes the handle, so no
/// handle is left that no webview knows. A reset in between already
/// disposed it; the disposal is then a no-op.
pub(crate) struct Unpublished {
    registry: SolveJobRegistry,
    handle: Option<String>,
}

impl Unpublished {
    fn new(registry: &SolveJobRegistry, handle: &str) -> Self {
        Self { registry: registry.clone(), handle: Some(handle.to_string()) }
    }
}

impl std::fmt::Debug for Unpublished {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Unpublished").field("handle", &self.handle).finish()
    }
}

impl Drop for Unpublished {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            // Unknown (a reset pruned it) or already released: nothing to do.
            let _ = self.registry.dispose_result(&handle);
        }
    }
}

impl RestoredRun {
    /// The reply carrying the handle exists: the handle stays held.
    pub(crate) fn published(&mut self) {
        self.unpublished.handle = None;
    }
}

impl NativeProjectEnvelope {
    pub(crate) fn published(&mut self) {
        if let StoredRun::Restored(run) = &mut self.stored_run {
            run.published();
        }
    }
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
    result_lease: CommandLease,
    record_lease: CommandLease,
}

/// The blocking part of the project open (module docs). `epoch` is the
/// session's reset epoch when the command began.
pub(crate) fn open_project_in_store(memory: &NativeLeaseMemory<'_>, path: PathBuf, project_id: Option<&str>, epoch: u64) -> Result<Option<NativeProjectEnvelope>, String> {
    let registry = memory.registry();
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
            let result_lease = lease(memory, &format!("{what}'s result ({} bytes)", lengths.0), envelope(lengths.0))?;
            let record_lease = lease(memory, &format!("{what}'s record ({} bytes)", lengths.1), envelope(lengths.1))?;
            let sqlite = lease(memory, &format!("SQLite's copies of {what}"), envelope(lengths.0).saturating_add(envelope(lengths.1)).saturating_mul(2))?;
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
    let mut parsed: Option<(Value, CommandLease, Value, CommandLease)> = None;
    let mut header = None;
    if let Some(run) = &loaded {
        match parse_leased(memory, "the stored result", &run.result) {
            Err(Ok(message)) => return Err(message),
            Err(Err(pool)) => refusal = Some(pool),
            Ok((result, result_parse)) => match parse_leased(memory, "the stored record", &run.record) {
                Err(Ok(message)) => return Err(message),
                Err(Err(pool)) => refusal = Some(pool),
                Ok((record, record_parse)) => match record_header(memory, &record) {
                    Ok(projected) => {
                        header = Some(projected);
                        parsed = Some((result, result_parse, record, record_parse));
                    }
                    Err(pool) => refusal = Some(pool),
                },
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
            let (record_header, record_header_bytes, header_lease) = header.expect("an admitted record has an admitted header");
            let mut found = validate::saved_run_findings(&project, &mut result, &record).expect("a stored run holds a result or a record");
            validate::saved_run_validation(&mut found, &mut result, &record);
            let recorded_dispatch = recorded_dispatch_of(&found, &result);
            drop((result, result_parse, record, record_parse));
            let source = RestoredSource {
                kind: "saved_project_run",
                project_id: id.clone(),
                recording_id: None,
                result_sha256: hex(&sha256_in_place(run.result.as_bytes())),
                record_sha256: Some(hex(&sha256_in_place(run.record.as_bytes()))),
            };
            let (result_bytes, record_bytes) = (run.result.len() as u64, run.record.len() as u64);
            // The re-save basis (`stored_save`): the stored model hash evidence and the stored
            // envelope evidence with the payload hash recomputed over the stored values.
            let basis = RestoredBasis { model_hash: model_hash.clone(), envelopes: vec![EnvelopeBasis { payload_sha256: found.envelope_payload_hash.clone(), evidence: project_envelope_hash.clone() }] };
            // The handoff: the leases backing the held texts leave the
            // command's total when the registry establishes the handle.
            match registry.restore_with_basis(
                epoch,
                source.clone(),
                HeldResult::restored_in_custody(run.result, run.result_lease).read_under(found.recorded),
                Some(HeldResult::restored_in_custody(run.record, run.record_lease)),
                Some(basis),
            ) {
                Ok(handle) => {
                    let unpublished = Unpublished::new(registry, &handle);
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
                            recorded_dispatch,
                            record_header,
                            record_header_bytes,
                            _header_lease: header_lease,
                            unpublished,
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

/// RT-A4 §10.4: a renderable reopened statement's recorded dispatch with no
/// model under the table its findings were read under ([`RestoredRun::recorded_dispatch`]).
fn recorded_dispatch_of(found: &HistoricalFindings, result: &Value) -> Option<Value> {
    found.recorded.filter(|_| found.renderable).map(|table| validate::recorded_dispatch(result, table.variant(), None))
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
struct RunLoad<'r, 'm> {
    memory: &'r NativeLeaseMemory<'m>,
    leases: Option<(CommandLease, CommandLease)>,
    record_text: Option<String>,
    /// The read's own parse of the stored result (RT-U4: not parsed again).
    result_value: Option<Value>,
}

impl NativeRunLoad for RunLoad<'_, '_> {
    fn admit(&mut self, result_bytes: u64, record_bytes: u64) -> Result<(), String> {
        let result = lease(self.memory, &format!("the recorded run's result ({result_bytes} bytes)"), envelope(result_bytes))?;
        let record = lease(self.memory, &format!("the recorded run's record ({record_bytes} bytes)"), envelope(record_bytes))?;
        self.leases = Some((result, record));
        Ok(())
    }

    fn record_text(&mut self, text: String) {
        self.record_text = Some(text);
    }

    fn result_value(&mut self, value: Value) {
        self.result_value = Some(value);
    }
}

/// The blocking part of the recorded run's reopen (module docs): the store's
/// verified read (every integrity refusal of `read_analysis_run`, without
/// its webview limit), then the findings, then the handle.
pub(crate) fn read_run_in_store(memory: &NativeLeaseMemory<'_>, path: PathBuf, project_id: &str, recording_id: &str, epoch: u64) -> Result<NativeRunRead, String> {
    let registry = memory.registry();
    let mut load = RunLoad { memory, leases: None, record_text: None, result_value: None };
    let read = analysis_runs::read_run_in_store_core(&path, project_id, recording_id, memory, u64::MAX, Some(&mut load))?;
    // The guard holds the read's reservations, the result's parse included,
    // until the parse is dropped below.
    let (read, guard) = read.into_custody();
    let analysis_runs::AnalysisRunRead { run, state, record, result_json, input_manifest_canonical } = read;
    let (Some((result_lease, record_lease)), Some(record_text), Some(result_value)) = (load.leases, load.record_text, load.result_value) else {
        return Err(format!("{REOPEN_FAILED}: the recorded run's texts were not loaded."));
    };
    #[cfg(test)]
    stored_read::phase_probe::mark("recorded read: verified, record text and result parse kept");
    let parsed_manifest = match parse_leased(memory, "the recorded run's input manifest", &input_manifest_canonical) {
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
    let mut result_value = Some(result_value);
    let mut found = validate::recorded_run_findings(&context, result_value.as_mut(), manifest, &record);
    if let Some(result) = result_value.as_mut() {
        validate::recorded_run_validation(&mut found, &context, result, manifest, &record);
    }
    let (record_header, record_header_bytes, header_lease) = record_header(memory, &record)?;
    let recorded_dispatch = result_value.as_ref().and_then(|result| recorded_dispatch_of(&found, result));
    let result_present = result_value.as_ref().is_some_and(|value| !value.is_null()) || result_value.is_none();
    drop((result_value, parsed_manifest, record));
    drop(guard);
    let source = RestoredSource {
        kind: "recorded_run",
        project_id: run.project_id.clone(),
        recording_id: Some(run.recording_id.clone()),
        result_sha256: hex(&sha256_in_place(result_json.as_bytes())),
        record_sha256: Some(hex(&sha256_in_place(record_text.as_bytes()))),
    };
    let (result_bytes, record_bytes) = (result_json.len() as u64, record_text.len() as u64);
    // The handoff: the leases backing the held texts leave the command's
    // total when the registry establishes the handle.
    let handle = registry
        .restore(epoch, source.clone(), HeldResult::restored_in_custody(result_json, result_lease).read_under(found.recorded), Some(HeldResult::restored_in_custody(record_text, record_lease)))
        .map_err(|error| format!("{RESTORE_REFUSED}: recorded run `{}` was not opened: {}", stored_read::shown(recording_id), restore_refusal(error)))?;
    let unpublished = Unpublished::new(registry, &handle);
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
            recorded_dispatch,
            record_header,
            record_header_bytes,
            _header_lease: header_lease,
            unpublished,
        },
    })
}

/// The reply as a JSON body, its exact length leased before it is built.
fn reply<T: Serialize>(memory: &NativeLeaseMemory<'_>, value: &T) -> Result<(String, CommandLease), String> {
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
    let held = lease(memory, "the reply", envelope(counter.0))?;
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

fn admit(memory: &NativeLeaseMemory<'_>, raw: &[u8], copy: CommandLease, command: &'static str) -> Result<(command_ingress::Args, CommandLease), CommandError<String>> {
    let copy_bytes = copy.bytes();
    let (args, lease) = command_ingress::admit_raw(memory.registry(), raw, command, ArgsDecoding::Moved, |bytes| {
        let mut lease = copy;
        memory.grow(&mut lease, copy_bytes.saturating_add(bytes))?;
        Ok(lease)
    })?;
    Ok((args, lease))
}

/// `open_local_project_in_native_custody`, after the executor's part.
pub(crate) fn run_open(registry: &SolveJobRegistry, store: impl FnOnce() -> Result<PathBuf, String>, raw: Vec<u8>, copy: Lease, epoch: u64) -> Result<tauri::ipc::Response, CommandError<String>> {
    #[cfg(test)]
    tests::command_gate::pass(tests::command_gate::key(registry, OPEN_COMMAND));
    let (memory, copy) = NativeLeaseMemory::seeded(registry, copy);
    let (mut args, args_lease) = admit(&memory, &raw, copy, OPEN_COMMAND)?;
    drop(raw);
    let project_id: Option<String> = args.decode("projectId")?;
    drop(args);
    let path = store().map_err(CommandError::Command)?;
    let mut opened = open_project_in_store(&memory, path, project_id.as_deref(), epoch).map_err(|error| CommandError::Command(stored_read::capped(error)))?;
    // A refused reply drops `opened`, disposing a handle it registered.
    let (text, held) = reply(&memory, &opened).map_err(CommandError::Command)?;
    if let Some(opened) = opened.as_mut() {
        opened.published();
    }
    let response = tauri::ipc::Response::new(text);
    drop((held, args_lease, opened));
    Ok(response)
}

/// `read_analysis_run_in_native_custody`, after the executor's part.
pub(crate) fn run_read(registry: &SolveJobRegistry, store: impl FnOnce() -> Result<PathBuf, String>, raw: Vec<u8>, copy: Lease, epoch: u64) -> Result<tauri::ipc::Response, CommandError<String>> {
    #[cfg(test)]
    tests::command_gate::pass(tests::command_gate::key(registry, RUN_COMMAND));
    let (memory, copy) = NativeLeaseMemory::seeded(registry, copy);
    let (mut args, args_lease) = admit(&memory, &raw, copy, RUN_COMMAND)?;
    drop(raw);
    let project_id: String = args.decode("projectId")?;
    let recording_id: String = args.decode("recordingId")?;
    drop(args);
    let path = store().map_err(CommandError::Command)?;
    let mut read = read_run_in_store(&memory, path, &project_id, &recording_id, epoch).map_err(|error| CommandError::Command(stored_read::capped(error)))?;
    // A refused reply drops `read`, disposing the handle it registered.
    let (text, held) = reply(&memory, &read).map_err(CommandError::Command)?;
    read.restored.published();
    let response = tauri::ipc::Response::new(text);
    drop((held, args_lease, read));
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
    tauri::async_runtime::spawn_blocking(move || open_pipe_stress_result_export::recorded_tables::entry_scope(move || run_open(&registry, store, raw, lease, epoch)))
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
    tauri::async_runtime::spawn_blocking(move || open_pipe_stress_result_export::recorded_tables::entry_scope(move || run_read(&registry, store, raw, lease, epoch)))
        .await
        .unwrap_or_else(|_| Err(CommandError::Command(format!("{REOPEN_FAILED}: the read ended without an outcome."))))
}

#[cfg(test)]
#[path = "stored_custody_tests.rs"]
pub(crate) mod tests;
