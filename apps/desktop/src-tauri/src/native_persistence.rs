//! C1 native custody, slice 5's persistence (DEL-04-05
//! `c1-native-custody-plan.md` slice 5; DEL-14-01 Design
//! `desktop-states-runs-comparison.md` §3): Record Run and Save or Create
//! Project of a native-custody solve, from the job handle and small arguments.
//! The webview sends no result, no record and no envelope hash; nothing
//! row-scale comes back.
//!
//! - **Today's stored forms, from the held bytes.** Record Run's
//!   `result_json` is `jsonTextKeepingNegativeZero(JSON.parse(bytes))`
//!   ([`result_js_text`], JavaScript's member order and numbers, negative
//!   zeros kept) with its checked SHA-256 (the builder's received-result
//!   checksum); its `run_record_json` is the recorded revision of the
//!   transient record ([`Revision`]: `recorded_revision`'s members, written
//!   in the record's stored form) and its hash. Save Project's
//!   `mechanics_result_json` is `serde_json::to_string` of the value parsed
//!   from `JSON.stringify` of the parse (the builder's stored writer over the
//!   held value), its `analysis_run_json` the transient record's stored bytes
//!   ([`record_bytes`]), and its envelope hash the session's
//!   `computeProjectEnvelopeHash` (RFC 8785 over `{model, editor_intents,
//!   proposal, selected_review_target, mechanics_result, analysis_run,
//!   model_hash}`) computed here over the values stored, streamed, with the
//!   evidence object the session writes. The held publication bytes are
//!   never rewritten.
//! - **Binding.** The job must be a native-custody solve still held
//!   (`custody_binding`), its held bytes must hash to its publication digest,
//!   and the session's manifest must name the job's solver mode; the model
//!   text the job was sent, re-sent with the request, must hash to the intake
//!   digest the registry recorded at admission, and its parse must be the
//!   manifest's model ([`bound_intake`]), so the manifest's model cannot be
//!   swapped under the same project id; the record
//!   rebuilt from the held result and the session's manifest, rule-check
//!   aggregate and exclusions must be the record the session previewed (its
//!   run id and both checksums, `NATIVE-PERSISTENCE-RECORD-STALE`). Record
//!   Run then applies the store's own checks to that record
//!   (`analysis_runs::checked_native_record`) and its transaction
//!   (`plan_native_run`, `write_native_run`); Save Project applies the
//!   project store's replacement guard, its withheld-run guard and the model
//!   document checks, unchanged.
//! - **SQLite's limit, exactly, before allocation and before writes.** Inside
//!   the write transaction the writing connection's `SQLITE_LIMIT_LENGTH` is
//!   read (rusqlite `limits`), and every value bound and every record SQLite
//!   builds is measured from the texts' exact counted lengths (each stored
//!   form is first written to a counter): the table row's record (SQLite's
//!   record format: header varints and data), each index key's record
//!   (bounded with an eight-byte rowid), and for a project the search row's
//!   value and its content record. A result and a record that each fit can
//!   still make a row that does not.
//!   - Record Run above the limit: [`RUN_RECORD_RESULT_TOO_LARGE`] with the
//!     exact lengths, nothing written (the plan writes nothing first).
//!   - Save Project above the limit with the result: the model-only save the
//!     plan approved (`mechanics_result_json` and `analysis_run_json`
//!     `null`, the envelope hash over what was stored), its reply saying the
//!     result was not saved ([`PROJECT_SAVE_RESULT_TOO_LARGE`]); never over a
//!     stored run withheld for its size without the user's confirmed Replace
//!     (`refuse_dropping_withheld_run`, unchanged). Above it without the
//!     result: [`PROJECT_SAVE_TOO_LARGE`], nothing written. No field is cut.
//!   - A row SQLite still refuses as too big rolls the transaction back and
//!     is refused by name (a backstop).
//! - **Leases** (no new budget or rule; each a pool lease capped at the
//!   per-invocation budget and the 12 GiB default): the raw body's copy and
//!   the decoded arguments' lease (the re-sent intake text included), the
//!   intake's parse (admitted at its priced parse and ended once compared,
//!   before the held result is parsed), the held result's parse M and the builder's
//!   working and fingerprint leases (as the preview's), the JS-text writer's
//!   member-list scratch, each output text's lease at its exact length
//!   (result, record, search text, the small slots), and SQLite's statement
//!   copies (it copies every bound text) and records, all held together while
//!   the rows are written; the parse M and the builder's leases end before the
//!   statements run. The response is leased at its exact length until it is
//!   handed to Tauri. The store's own reads lease through
//!   [`NativeLeaseMemory`].
//! - **Residuals, unpriced** (slice 7's measurement): the builder's and
//!   readers' own working memory (as the preview's); SQLite's page cache
//!   and journal, and FTS5's in-memory index of the search text it tokenizes
//!   (as the submitted save's); Tauri's delivery copies of the small reply.
use crate::analysis_run_v03_build::{
    self as build, header_record, record_bytes, write_canonical_record, Built, Count, Revision, Sink, Style, Writer,
};
use crate::analysis_runs::{self, NativeRunRequest, NativeRunTexts, RunWriteHooks};
use crate::command_ingress::{self, ArgsDecoding, CommandError};
use crate::ingress;
use crate::memory_pool::Lease;
use crate::result_js_text::{self, NegativeZero};
use crate::solve_jobs::{sha256_in_place, CustodyDigests, ResultCustody, SolveJobRegistry};
use crate::stored_read::{self, NativeLeaseMemory, ReadMemory, Serialized};
use rusqlite::limits::Limit;
use rusqlite::{params, OptionalExtension, Transaction};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use tauri::http::HeaderMap;
use tauri::ipc::InvokeBody;

pub(crate) const RECORD_COMMAND: &str = "record_preview_mechanics_job_analysis_run";
pub(crate) const SAVE_COMMAND: &str = "save_preview_mechanics_job_local_project";
/// The blocking body ended without an outcome (it panicked).
pub(crate) const PERSISTENCE_FAILED: &str = "NATIVE-PERSISTENCE-FAILED";
/// The job is not a native-custody solve still held, or its bytes, mode or
/// manifest do not bind.
pub(crate) const CUSTODY: &str = "NATIVE-PERSISTENCE-CUSTODY";
/// The record rebuilt now is not the record the session previewed.
pub(crate) const RECORD_STALE: &str = "NATIVE-PERSISTENCE-RECORD-STALE";
pub(crate) use crate::analysis_runs::RUN_RECORD_RESULT_TOO_LARGE;
/// A project saved without its result, which did not fit the store.
pub(crate) const PROJECT_SAVE_RESULT_TOO_LARGE: &str = "PROJECT-SAVE-RESULT-TOO-LARGE";
/// A project whose fields other than the result do not fit the store.
pub(crate) const PROJECT_SAVE_TOO_LARGE: &str = "PROJECT-SAVE-TOO-LARGE";

// ---------------------------------------------------------------------------
// SQLite's record format and length limit
// ---------------------------------------------------------------------------

/// A value as SQLite stores it in a record: its storage class and length.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Cell {
    Null,
    Integer(i64),
    Real,
    Text(u64),
    Blob(u64),
}

/// `sqlite3VarintLen`.
fn varint_len(mut value: u64) -> u64 {
    let mut length = 1;
    loop {
        value >>= 7;
        if value == 0 || length == 9 {
            return length;
        }
        length += 1;
    }
}

/// `sqlite3VdbeSerialType` (schema format 4): the serial type and data size.
fn serial(cell: Cell) -> (u64, u64) {
    match cell {
        Cell::Null => (0, 0),
        Cell::Integer(value) => {
            let magnitude = if value < 0 { !(value as u64) } else { value as u64 };
            if magnitude <= 127 {
                if value == 0 || value == 1 { (8 + value as u64, 0) } else { (1, 1) }
            } else if magnitude <= 32_767 {
                (2, 2)
            } else if magnitude <= 8_388_607 {
                (3, 3)
            } else if magnitude <= 2_147_483_647 {
                (4, 4)
            } else if magnitude <= 0x7fff_ffff_ffff {
                (5, 6)
            } else {
                (6, 8)
            }
        }
        Cell::Real => (7, 8),
        Cell::Text(length) => (length.saturating_mul(2).saturating_add(13), length),
        Cell::Blob(length) => (length.saturating_mul(2).saturating_add(12), length),
    }
}

/// The bytes of the record SQLite's `OP_MakeRecord` builds for these cells
/// (its header, sized as SQLite sizes it, and its data), which SQLite refuses
/// as too big above `SQLITE_LIMIT_LENGTH`.
pub(crate) fn record_size(cells: &[Cell]) -> u64 {
    let (mut header, mut data) = (0u64, 0u64);
    for cell in cells {
        let (serial_type, size) = serial(*cell);
        header = header.saturating_add(varint_len(serial_type));
        data = data.saturating_add(size);
    }
    if header <= 126 {
        header += 1;
    } else {
        let width = varint_len(header);
        header += width;
        if width < varint_len(header) {
            header += 1;
        }
    }
    header.saturating_add(data)
}

/// The writing connection's `SQLITE_LIMIT_LENGTH`.
pub(crate) fn length_limit(transaction: &Transaction<'_>) -> u64 {
    u64::try_from(transaction.limit(Limit::SQLITE_LIMIT_LENGTH)).unwrap_or(0)
}

/// A column of a row about to be written: its new value, or its stored value
/// kept by an upsert's update.
enum Column {
    Is(Cell),
    Kept,
}

/// A table's columns in their stored order, with their declared defaults.
fn table_columns(transaction: &Transaction<'_>, table: &str) -> Result<Vec<(String, Option<String>)>, String> {
    let mut statement = transaction.prepare(&format!("PRAGMA table_info({table})")).map_err(|error| stored_read::sql_error(&error))?;
    let columns = statement
        .query_map([], |row| Ok((row.get::<_, String>(1)?, row.get::<_, Option<String>>(4)?)))
        .map_err(|error| stored_read::sql_error(&error))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| stored_read::sql_error(&error))?;
    Ok(columns)
}

/// A declared default as the cell an insert stores.
fn default_cell(default: Option<&str>) -> Cell {
    match default.map(str::trim) {
        None => Cell::Null,
        Some(text) if text.eq_ignore_ascii_case("null") => Cell::Null,
        Some(text) => match text.parse::<i64>() {
            Ok(value) => Cell::Integer(value),
            Err(_) => Cell::Text(text.trim_matches('\'').len() as u64),
        },
    }
}

/// A stored value's cell, from its type and length (read from the record
/// header; an integer's value is read).
fn stored_cell(transaction: &Transaction<'_>, table: &str, column: &str, key_column: &str, key: &str) -> Result<Cell, String> {
    if !column.bytes().all(|byte| byte.is_ascii_alphanumeric() || byte == b'_') {
        return Err(format!("STORE-SQL-ERROR: column `{}` cannot be measured", stored_read::shown(column)));
    }
    let (kind, length, integer): (String, Option<i64>, Option<i64>) = transaction
        .query_row(
            &format!("SELECT typeof({column}), octet_length({column}), CASE WHEN typeof({column}) = 'integer' THEN {column} END FROM {table} WHERE {key_column} = ?1"),
            params![key],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .map_err(|error| stored_read::sql_error(&error))?;
    let length = u64::try_from(length.unwrap_or(0)).unwrap_or(0);
    Ok(match kind.as_str() {
        "null" => Cell::Null,
        "integer" => Cell::Integer(integer.unwrap_or(0)),
        "real" => Cell::Real,
        "blob" => Cell::Blob(length),
        _ => Cell::Text(length),
    })
}

/// The record of the row `columns` names (by column name; any other column
/// takes its stored value when `existing` names the row, else its default).
fn row_record(
    transaction: &Transaction<'_>,
    table: &str,
    columns: &[(&str, Column)],
    existing: Option<(&str, &str)>,
) -> Result<u64, String> {
    let mut cells = Vec::new();
    for (name, default) in table_columns(transaction, table)? {
        let given = columns.iter().find(|(column, _)| *column == name).map(|(_, value)| value);
        let cell = match (given, existing) {
            (Some(Column::Is(cell)), _) => *cell,
            (Some(Column::Kept) | None, Some((key_column, key))) => stored_cell(transaction, table, &name, key_column, key)?,
            (Some(Column::Kept) | None, None) => default_cell(default.as_deref()),
        };
        cells.push(cell);
    }
    Ok(record_size(&cells))
}

/// One length SQLite bounds: a bound value or a record.
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub(crate) struct Measured {
    pub(crate) what: String,
    pub(crate) bytes: u64,
}

fn measured(what: &str, bytes: u64) -> Measured {
    Measured { what: what.to_string(), bytes }
}

/// The first of `lengths` above `limit`.
fn first_over(lengths: &[Measured], limit: u64) -> Option<&Measured> {
    lengths.iter().find(|length| length.bytes > limit)
}

/// An index's key record for a row: its columns' cells and the rowid, bounded
/// above with an eight-byte rowid.
fn index_record(cells: &[Cell]) -> u64 {
    let mut all = cells.to_vec();
    all.push(Cell::Integer(i64::MAX));
    record_size(&all)
}

// ---------------------------------------------------------------------------
// Admission and binding
// ---------------------------------------------------------------------------

/// The record the session previewed (its native analysis-run header).
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ExpectedRecord {
    pub(crate) run_id: String,
    pub(crate) record_sha256: String,
    pub(crate) received_sha256: String,
}

/// The executor's part: the header check, the copy's reservation and the copy.
pub(crate) fn reserve(registry: &SolveJobRegistry, body: &InvokeBody, headers: &HeaderMap, command: &'static str) -> Result<(Vec<u8>, Lease), CommandError<String>> {
    let raw = command_ingress::raw_args_body(body, headers, command)?;
    let lease = registry.acquire_interactive_ingress_capped(ingress::allocation_envelope(raw.len() as u64).unwrap_or(u64::MAX))?;
    Ok((raw.to_vec(), lease))
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// The job is a native-custody solve still held, its held bytes are its
/// publication's (their SHA-256 is the digest recorded at publication), and
/// the session's manifest names the job's solver mode. Returns the job's
/// custody digests (for [`bound_intake`]).
fn bound_custody(registry: &SolveJobRegistry, prepared: &build::Prepared, manifest: &Value) -> Result<CustodyDigests, String> {
    let job = stored_read::shown(&prepared.job_id).into_owned();
    if registry.result_custody(&prepared.job_id) != Some(ResultCustody::Native) {
        return Err(format!("{CUSTODY}: job `{job}` is not a native-custody solve; nothing was written."));
    }
    let digests = registry
        .custody_binding(&prepared.job_id)
        .ok_or_else(|| format!("{CUSTODY}: job `{job}`'s result is no longer held; nothing was written."))?;
    if <[u8; 32]>::from(Sha256::digest(prepared.held.text().as_bytes())) != digests.result_sha256 {
        return Err(format!("{CUSTODY}: job `{job}`'s held bytes are not its publication's; nothing was written."));
    }
    let mode = manifest.pointer("/solver_basis/solver_mode").and_then(Value::as_str);
    if mode != Some(digests.solver_mode.as_str()) {
        return Err(format!(
            "{CUSTODY}: job `{job}` was solved in {} mode, but the input manifest names {}; nothing was written.",
            digests.solver_mode.as_str(),
            stored_read::shown(mode.unwrap_or("no solver mode"))
        ));
    }
    Ok(digests)
}

/// The solve's intake binds the manifest's model. `intake` is the model text
/// the session's `startPreviewMechanicsJob` sent for this job
/// (`deliveredIntakeText`: `checkedJsonText` of the captured model), re-sent
/// with the request; the registry kept only its digest. In order, each a
/// [`CUSTODY`] refusal before anything is written:
/// 1. the text is sent;
/// 2. its exact bytes hash, in place and without allocating, to the intake
///    digest the registry recorded when it admitted the job's body;
/// 3. its parse, priced as an argument's (`text_parse_bytes`) and admitted
///    first as an interactive lease capped as the arguments' was (a refusal
///    is the pool's typed refusal), reads it;
/// 4. the parsed model is the manifest's `model_basis.model_payload` as JSON
///    data with JavaScript's numbers ([`same_js_data`]), so a manifest whose
///    model differs under the same project id is refused, while the session's
///    member order and number texts are not compared.
///
/// The parse and its lease end here, before the held result is parsed.
fn bound_intake(registry: &SolveJobRegistry, job_id: &str, digests: &CustodyDigests, intake: Option<String>, manifest: &Value) -> Result<(), CommandError<String>> {
    let job = stored_read::shown(job_id).into_owned();
    let refuse = |what: &str| CommandError::Command(format!("{CUSTODY}: {what}; nothing was written."));
    let intake = intake.ok_or_else(|| refuse(&format!("the model text job `{job}` was sent is not in the request")))?;
    if sha256_in_place(intake.as_bytes()) != digests.intake_sha256 {
        return Err(refuse(&format!("the model text in the request is not the body job `{job}` admitted")));
    }
    let priced = command_ingress::text_parse_bytes(&intake, ArgsDecoding::Moved).map_err(|_| refuse(&format!("job `{job}`'s model text is not JSON this app reads")))?;
    let parsing = registry.acquire_interactive_ingress_capped(priced)?;
    let model: Value = serde_json::from_str(&intake).map_err(|_| refuse(&format!("job `{job}`'s model text is not JSON this app reads")))?;
    drop(intake);
    let payload = manifest.pointer("/model_basis/model_payload").ok_or_else(|| refuse("the input manifest has no model payload"))?;
    let same = same_js_data(&model, payload);
    drop(model);
    drop(parsing);
    if !same {
        return Err(refuse(&format!("the input manifest's model is not the model job `{job}` solved")));
    }
    Ok(())
}

/// JSON data equality as JavaScript reads both texts: scalars by value with
/// numbers as binary64 (`-0` equals `0`, `1.0` equals `1`), arrays by
/// position, objects by their member names, in any order. Exactly the
/// equality of the two values' RFC 8785 texts (`canonical_json`, which the
/// manifest's identity uses), computed without writing them. The recursion is
/// bounded by serde_json's nesting limit.
pub(crate) fn same_js_data(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Null, Value::Null) => true,
        (Value::Bool(x), Value::Bool(y)) => x == y,
        (Value::Number(x), Value::Number(y)) => x.as_f64() == y.as_f64(),
        (Value::String(x), Value::String(y)) => x == y,
        (Value::Array(x), Value::Array(y)) => x.len() == y.len() && x.iter().zip(y).all(|(x, y)| same_js_data(x, y)),
        (Value::Object(x), Value::Object(y)) => x.len() == y.len() && x.iter().all(|(key, x)| y.get(key).is_some_and(|y| same_js_data(x, y))),
        _ => false,
    }
}

/// The rebuilt record is the record the session previewed.
fn expect_record(built: &Built<'_>, expected: &ExpectedRecord) -> Result<(), String> {
    if built.run_id != expected.run_id || built.record_sha256 != expected.record_sha256 || built.received_sha256 != expected.received_sha256 {
        return Err(format!(
            "{RECORD_STALE}: the record built now from the held result and these inputs (run `{}`, record {}, received result {}) is not the record the session previewed (run `{}`, record {}, received result {}); nothing was written.",
            stored_read::shown(built.run_id),
            built.record_sha256,
            built.received_sha256,
            stored_read::shown(&expected.run_id),
            stored_read::shown(&expected.record_sha256),
            stored_read::shown(&expected.received_sha256)
        ));
    }
    Ok(())
}

/// The command's refusal as the webview reads it: a typed ingress refusal
/// stays typed; every other refusal is its text, cut as the store's are.
fn refusal(error: String) -> CommandError<String> {
    CommandError::Command(stored_read::capped(error))
}

/// A text of exactly `length` bytes, its lease admitted first.
fn leased_text(memory: &dyn ReadMemory, what: &str, length: u64, write: impl FnOnce(&mut Vec<u8>) -> Result<(), String>) -> Result<(String, stored_read::ReadReservation), String> {
    let reservation = memory.reserve(what, stored_read::exact_text_bytes(length))?;
    let mut out = Vec::with_capacity(usize::try_from(length).map_err(|_| "STORE-READ: a length does not fit this platform".to_string())?);
    write(&mut out)?;
    if out.len() as u64 != length {
        return Err(format!("STORE-READ-LENGTH-MISCOUNTED: {what} is {} bytes, counted {length}.", out.len()));
    }
    let text = String::from_utf8(out).map_err(|_| format!("STORE-READ: {what} is not UTF-8"))?;
    Ok((text, reservation))
}

/// `serde_json::to_string(value)`, leased at its exact length.
fn leased_json(memory: &dyn ReadMemory, what: &str, value: &Value) -> Result<(String, stored_read::ReadReservation), String> {
    let length = stored_read::serialized_length(value)?;
    leased_text(memory, what, length, |out| serde_json::to_writer(out, value).map_err(|error| stored_read::json_error(&error)))
}

/// The held result's parse and the builder's leases, dropped together with
/// the record built on them, before any statement runs.
struct Building {
    _working: Lease,
    _scratch: Lease,
    _reading: Lease,
}

// ---------------------------------------------------------------------------
// Record Run
// ---------------------------------------------------------------------------

/// The command: [`reserve`] on the caller's executor, then [`run_record`] in
/// a blocking task. The reply is the store's outcome as a JSON body.
pub(crate) async fn record_run(
    registry: &SolveJobRegistry,
    store: impl FnOnce() -> Result<PathBuf, String> + Send + 'static,
    body: &InvokeBody,
    headers: &HeaderMap,
) -> Result<tauri::ipc::Response, CommandError<String>> {
    let (raw, lease) = reserve(registry, body, headers, RECORD_COMMAND)?;
    let registry = registry.clone();
    // The response is handed over inside the task, its lease released after it.
    tauri::async_runtime::spawn_blocking(move || stored_read::respond_serialized(run_record(&registry, store, raw, lease, &analysis_runs::PRODUCTION_RUN_HOOKS, None)))
        .await
        .unwrap_or_else(|_| Err(CommandError::Command(PERSISTENCE_FAILED.into())))
}

/// The record the session previewed and the request's members (snake_case,
/// `request`, as `record_analysis_run`'s without the record, result and
/// manifest): its base, its state or new snapshot and its labels. The project
/// is the held result's, set once the record is built.
fn decode_record(args: &mut command_ingress::Args) -> Result<(ExpectedRecord, Option<String>, NativeRunRequest), CommandError<String>> {
    let expected: ExpectedRecord = args.decode("expectedRecord")?;
    let intake: Option<String> = args.decode("intakeText")?;
    let request = args.decode_with("request", |value| {
        let mut fields = command_ingress::Fields::of(value, "struct NativeRunRequest")?;
        let run_labels = fields.required_value("run_labels")?;
        Ok(NativeRunRequest {
            project_id: String::new(),
            expected_base: fields.required("expected_base")?,
            state_id: fields.optional("state_id")?,
            new_state: analysis_runs::new_state_from_value(fields.value_or_null("new_state"))?,
            run_labels: analysis_runs::run_labels_from_value(run_labels).map_err(serde::de::Error::custom)?,
        })
    })?;
    Ok((expected, intake, request))
}

/// Record Run's texts and plan, made while the record and the held result's
/// parse are alive; everything here is owned, so they can end first.
struct StagedRun {
    plan: analysis_runs::NativeRunPlan,
    checked: stored_read::Metered<analysis_runs::NativeChecked>,
    record_json: String,
    record_hash: String,
    result_json: String,
    received: String,
    run_id: String,
    /// The texts' leases, held with the texts.
    leases: Vec<stored_read::ReadReservation>,
    /// What SQLite copies and builds for the row: the bound texts and the row's record.
    statement_bytes: u64,
}

/// The blocking part of Record Run (module docs). `limit` lowers the writing
/// connection's `SQLITE_LIMIT_LENGTH` (tests); production passes `None`.
/// Order: the arguments and the manifest, the job's custody, the intake's
/// binding, the build and the record's identity, the store's record checks, then in one write
/// transaction the plan, the exact lengths against the connection's limit,
/// the texts (the parse and the record end here), and the writes; the
/// response is serialized before the commit.
pub(crate) fn run_record(
    registry: &SolveJobRegistry,
    store: impl FnOnce() -> Result<PathBuf, String>,
    raw: Vec<u8>,
    copy: Lease,
    hooks: &RunWriteHooks<'_>,
    limit: Option<i32>,
) -> Result<Serialized, CommandError<String>> {
    let memory = NativeLeaseMemory::new(registry);
    let (prepared, (expected, intake, mut request)) = build::admit_for(registry, raw, copy, RECORD_COMMAND, decode_record)?;
    let mut values = vec![&prepared.evidence.manifest_ref];
    values.extend(prepared.manifest.as_ref());
    let working = build::working_lease(registry, &values)?;
    build::verify_manifest(&prepared.evidence, prepared.manifest.as_ref()).map_err(CommandError::Command)?;
    let manifest = prepared.manifest.as_ref().expect("verified");
    let digests = bound_custody(registry, &prepared, manifest).map_err(refusal)?;
    bound_intake(registry, &prepared.job_id, &digests, intake, manifest)?;
    let manifest_text = prepared.evidence.canonical_bytes.as_str();
    let held_text = prepared.held.text();
    let mut connection: Option<rusqlite::Connection> = None;
    let transaction: Transaction<'_>;
    let sqlite: stored_read::ReadReservation;
    let staged = {
        let (mut source, reading) = build::parse_source(registry, held_text)?;
        let scratch = build::working_lease(registry, &[&source])?;
        let mut load_basis = Value::Null;
        let mut reserve = |bytes: u64| registry.acquire_result_parse(bytes).map(Some).map_err(CommandError::from);
        let built = build::preview_parsed(
            &mut source,
            &prepared.evidence,
            &prepared.manifest_sha256,
            manifest,
            prepared.aggregate.as_ref(),
            &prepared.exclusions,
            &mut load_basis,
            &mut reserve,
        )?;
        expect_record(&built, &expected).map_err(refusal)?;
        request.project_id = build::model_ref(&built).to_string();
        let header = header_record(&built);
        let checked = analysis_runs::checked_native_record(&request.project_id, &header, &built.record_sha256, &built.received_sha256, manifest, manifest_text, &memory)
            .map_err(refusal)?;
        drop(header);
        // The received result's text: its exact length first (nothing allocated).
        let result_length = result_js_text::length(held_text, NegativeZero::Keep).map_err(refusal)?;
        let path = store().map_err(refusal)?;
        let opened = connection.insert(stored_read::open_store(&path).map_err(refusal)?);
        if let Some(limit) = limit {
            opened.set_limit(Limit::SQLITE_LIMIT_LENGTH, limit);
        }
        transaction = stored_read::write_transaction(opened).map_err(refusal)?;
        sqlite = stored_read::admit_sqlite_working_set(&transaction, &memory).map_err(refusal)?;
        let staged = stage_record(&transaction, &built, checked, manifest, manifest_text, held_text, result_length, &request, hooks, &memory).map_err(refusal)?;
        // The record, the held result's parse and the builder's leases end here.
        drop(built);
        drop(source);
        drop(Building { _working: working, _scratch: scratch, _reading: reading });
        staged
    };
    let StagedRun { plan, checked, record_json, record_hash, result_json, received, run_id, leases, statement_bytes } = staged;
    // SQLite copies each bound text and builds the row's record.
    let statement = memory.reserve("SQLite's copies of the run's row", statement_bytes).map_err(refusal)?;
    let texts = NativeRunTexts {
        record_json: &record_json,
        record_hash: &record_hash,
        result_json: &result_json,
        result_hash: &received,
        manifest_text,
        producer_run_id: &run_id,
    };
    let finish = |outcome| stored_read::serialize_metered(&memory, RECORD_COMMAND, outcome);
    let serialized = analysis_runs::write_native_run(&transaction, &request, plan, &texts, hooks, &memory, finish).map_err(refusal)?;
    drop((statement, result_json, record_json, leases, checked));
    transaction.commit().map_err(|error| refusal(stored_read::sql_error(&error)))?;
    drop(sqlite);
    Ok(serialized)
}

/// Record Run's steps inside its transaction while the record is alive: the
/// plan, the recorded revision, the exact lengths against the connection's
/// limit (refused with nothing written and nothing of their size allocated),
/// then the texts at their exact lengths.
#[allow(clippy::too_many_arguments)]
fn stage_record(
    transaction: &Transaction<'_>,
    built: &Built<'_>,
    checked: stored_read::Metered<analysis_runs::NativeChecked>,
    manifest: &Value,
    manifest_text: &str,
    held_text: &str,
    result_length: u64,
    request: &NativeRunRequest,
    hooks: &RunWriteHooks<'_>,
    memory: &dyn ReadMemory,
) -> Result<StagedRun, String> {
    let project_id = request.project_id.as_str();
    let plan = analysis_runs::plan_native_run(transaction, request, checked.get(), hooks, memory)?;
    let payload = manifest.pointer("/model_basis/model_payload").ok_or_else(|| "ANALYSIS-RUN-MANIFEST-INVALID: the input manifest has no model payload.".to_string())?;
    let rule_pack_refs = analysis_runs::rule_pack_refs(manifest)?;
    let library_refs = analysis_runs::library_refs(payload);
    let revision = Revision { created_at: &plan.created_at, state_id: &plan.state_id, rule_pack_refs: &rule_pack_refs, library_refs: &library_refs, record_sha256: String::new() }.sealed(built);
    let record_length = revision.stored_length(built);
    let recording_id = analysis_runs::recording_id_of(&revision.record_sha256);
    let limit_bytes = length_limit(transaction);
    let small = |value: &str| Cell::Text(value.len() as u64);
    let row = row_record(
        transaction,
        "analysis_runs",
        &[
            ("project_id", Column::Is(small(project_id))),
            ("recording_id", Column::Is(small(&recording_id))),
            ("state_id", Column::Is(small(&plan.state_id))),
            ("producer_run_id", Column::Is(small(built.run_id))),
            ("created_at", Column::Is(small(&plan.created_at))),
            ("run_record_json", Column::Is(Cell::Text(record_length))),
            ("run_record_hash", Column::Is(small(&revision.record_sha256))),
            ("input_manifest_json", Column::Is(small(manifest_text))),
            ("result_json", Column::Is(Cell::Text(result_length))),
            ("result_hash", Column::Is(small(&built.received_sha256))),
        ],
        None,
    )?;
    let lengths = [
        measured("received result", result_length),
        measured("recorded run record", record_length),
        measured("input manifest", manifest_text.len() as u64),
        measured("project id", project_id.len() as u64),
        measured("producer run id", built.run_id.len() as u64),
        measured("run row", row),
        measured("run index key", index_record(&[small(project_id), small(&recording_id)])),
        measured("run state index key", index_record(&[small(project_id), small(&plan.state_id)])),
    ];
    if let Some(over) = first_over(&lengths, limit_bytes) {
        return Err(format!(
            "{RUN_RECORD_RESULT_TOO_LARGE} (result_bytes={result_length}, record_bytes={record_length}, row_bytes={row}, limit_bytes={limit_bytes}): run `{}`'s {} is {} bytes, above this store's SQLite length limit of {limit_bytes} bytes, so the run was not recorded; nothing was written. The current solve stays open.",
            stored_read::shown(built.run_id),
            over.what,
            over.bytes
        ));
    }
    // The texts, each at its exact length, its lease admitted first.
    let js_scratch = memory.reserve("the received result's text writer", result_js_text::scratch_bytes(built.source))?;
    let (result_json, result_lease) =
        leased_text(memory, "the received result's text", result_length, |out| result_js_text::write(held_text, NegativeZero::Keep, out))?;
    drop(js_scratch);
    let (record_json, record_lease) = revision
        .stored_bytes(built, record_length, |bytes| memory.reserve("the recorded run record", bytes).map_err(CommandError::Command))
        .map_err(command_text)?;
    let record_json = String::from_utf8(record_json).map_err(|_| "STORE-READ: the recorded run record is not UTF-8".to_string())?;
    let bound = [result_length, record_length, manifest_text.len() as u64, project_id.len() as u64].iter().fold(0u64, |sum, length| sum.saturating_add(*length));
    let record_hash = revision.record_sha256.clone();
    drop(revision);
    Ok(StagedRun {
        plan,
        checked,
        record_json,
        record_hash,
        result_json,
        received: built.received_sha256.clone(),
        run_id: built.run_id.to_string(),
        leases: vec![result_lease, record_lease],
        statement_bytes: bound.saturating_add(row),
    })
}

fn command_text(error: CommandError<String>) -> String {
    match error {
        CommandError::Command(text) | CommandError::Response(text) => text,
        CommandError::Ingress(error) => format!("{}: {error:?}", stored_read::STORE_READ_MEMORY_UNAVAILABLE),
    }
}

// ---------------------------------------------------------------------------
// Save and Create Project
// ---------------------------------------------------------------------------

/// The save's members other than the record's inputs: the project's
/// non-row-scale fields, as `save_local_project` and `create_local_project`
/// take them.
pub(crate) struct SaveRequest {
    /// `create` (the project id and name from the model, as
    /// `create_local_project`) or `save` (as the session sends them).
    pub(crate) create: bool,
    pub(crate) project_id: Option<String>,
    pub(crate) project_name: Option<String>,
    pub(crate) model: Value,
    pub(crate) editor_intents: Option<Value>,
    pub(crate) proposal: Option<Value>,
    pub(crate) selected_review_target: Option<Value>,
    pub(crate) model_hash: Value,
    pub(crate) confirmed_revision: Option<String>,
    pub(crate) expected_base: Option<String>,
    pub(crate) expected: ExpectedRecord,
    /// The model text the job was sent ([`bound_intake`]).
    pub(crate) intake: Option<String>,
}

fn decode_save(args: &mut command_ingress::Args) -> Result<SaveRequest, CommandError<String>> {
    let mode: String = args.decode("mode")?;
    let create = match mode.as_str() {
        "create" => true,
        "save" => false,
        _ => return Err(CommandError::Command("INGRESS-ARGS-INVALID: `mode` is `create` or `save`.".into())),
    };
    Ok(SaveRequest {
        create,
        project_id: args.decode("projectId")?,
        project_name: args.decode("projectName")?,
        model: args.value("model")?,
        editor_intents: args.opt_value("editorIntents"),
        proposal: args.opt_value("proposal"),
        selected_review_target: args.opt_value("selectedReviewTarget"),
        model_hash: args.opt_value("modelHash").unwrap_or(Value::Null),
        confirmed_revision: args.decode("confirmedRevision")?,
        expected_base: args.decode("expectedBase")?,
        expected: args.decode("expectedRecord")?,
        intake: args.decode("intakeText")?,
    })
}

/// What happened to the result in a native save.
#[derive(Debug, Serialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub(crate) enum SavedResult {
    /// Stored with the project, in today's forms.
    Saved { run_id: String, result_bytes: u64, record_bytes: u64 },
    /// Not stored (the model-only save): the stored project holds no result.
    NotSaved { code: &'static str, message: String, result_bytes: u64, record_bytes: u64, row_bytes: u64, limit_bytes: u64, refused: Measured },
}

/// The native save's reply: the stored project's small fields and hashes, and
/// what happened to the result. No model, result or record.
#[derive(Debug, Serialize)]
pub(crate) struct NativeProjectSave {
    pub(crate) summary: crate::LocalProjectSummary,
    pub(crate) proposal: Value,
    pub(crate) selected_review_target: Value,
    pub(crate) model_hash: Value,
    pub(crate) project_envelope_hash: Value,
    pub(crate) model_document_version: Value,
    /// `sha256:<hex>` of the stored model's RFC 8785 text (`computeModelHash`'s value).
    pub(crate) stored_model_hash: String,
    pub(crate) result: SavedResult,
}

/// The command: [`reserve`] on the caller's executor, then [`run_save`] in a
/// blocking task. The reply is [`NativeProjectSave`] as a JSON body.
pub(crate) async fn save_project(
    registry: &SolveJobRegistry,
    store: impl FnOnce() -> Result<PathBuf, String> + Send + 'static,
    body: &InvokeBody,
    headers: &HeaderMap,
) -> Result<tauri::ipc::Response, CommandError<String>> {
    let (raw, lease) = reserve(registry, body, headers, SAVE_COMMAND)?;
    let registry = registry.clone();
    tauri::async_runtime::spawn_blocking(move || stored_read::respond_serialized(run_save(&registry, store, raw, lease, &SaveLimits::PRODUCTION, &crate::now_unix_seconds)))
        .await
        .unwrap_or_else(|_| Err(CommandError::Command(PERSISTENCE_FAILED.into())))
}

/// The limits a save runs under: the writing connection's
/// `SQLITE_LIMIT_LENGTH` as it is, or lowered (tests), and the stored-run
/// size above which a stored run is withheld (`refuse_dropping_withheld_run`).
pub(crate) struct SaveLimits {
    pub(crate) sqlite_length: Option<i32>,
    pub(crate) withheld_bytes: u64,
}

impl SaveLimits {
    pub(crate) const PRODUCTION: SaveLimits = SaveLimits { sqlite_length: None, withheld_bytes: crate::solve_jobs::MAX_DISPLAYABLE_RESULT_BYTES };
}

/// What Save Project made while the record and the held result's parse were
/// alive; everything here is owned.
struct StagedSave {
    saved: bool,
    refused_with: Option<Measured>,
    row_with_result: u64,
    lengths: ProjectLengths,
    limit_bytes: u64,
    now: i64,
    envelope: Value,
    stored_model_hash: String,
    result_json: String,
    record_json: String,
    leases: Vec<stored_read::ReadReservation>,
    run_id: String,
    result_length: u64,
    record_length: u64,
}

/// The blocking part of Save or Create Project (module docs); `now` is the
/// row's time (unix seconds). Order: the arguments and the manifest, the
/// job's custody, the intake's binding, the build and the record's identity,
/// the model document
/// checks, then in one write transaction the replacement guard, the exact
/// lengths against the connection's limit (the model-only save, or the
/// refusal), the withheld-run guard, the envelope hash and the result's texts
/// (the parse and the record end here), the other texts and the writes; the
/// response is serialized before the commit.
pub(crate) fn run_save(
    registry: &SolveJobRegistry,
    store: impl FnOnce() -> Result<PathBuf, String>,
    raw: Vec<u8>,
    copy: Lease,
    limits: &SaveLimits,
    now: &dyn Fn() -> Result<i64, String>,
) -> Result<Serialized, CommandError<String>> {
    let memory = NativeLeaseMemory::new(registry);
    let (prepared, request) = build::admit_for(registry, raw, copy, SAVE_COMMAND, decode_save)?;
    let mut values = vec![&prepared.evidence.manifest_ref];
    values.extend(prepared.manifest.as_ref());
    let working = build::working_lease(registry, &values)?;
    build::verify_manifest(&prepared.evidence, prepared.manifest.as_ref()).map_err(CommandError::Command)?;
    let manifest = prepared.manifest.as_ref().expect("verified");
    let digests = bound_custody(registry, &prepared, manifest).map_err(refusal)?;
    let SaveRequest { create, project_id, project_name, model, editor_intents, proposal, selected_review_target, model_hash, confirmed_revision, expected_base, expected, intake } = request;
    bound_intake(registry, &prepared.job_id, &digests, intake, manifest)?;
    let (project_id, project_name, expected_base) = if create {
        (crate::project_id_from_model(&model), crate::project_name_from_model(&model), None)
    } else {
        let missing = |what: &str| refusal(format!("INGRESS-ARGS-INVALID: a save names its project's {what}."));
        (project_id.ok_or_else(|| missing("id"))?, project_name.ok_or_else(|| missing("name"))?, expected_base)
    };
    let editor_intents = crate::normalized_editor_intents(editor_intents);
    let proposal = crate::normalized_proposal(proposal);
    let selected_review_target = crate::normalized_selected_review_target(selected_review_target);
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
    let (path, migration, document_version, staged) = {
        let (mut source, reading) = build::parse_source(registry, prepared.held.text())?;
        let scratch = build::working_lease(registry, &[&source])?;
        let mut load_basis = Value::Null;
        let mut reserve = |bytes: u64| registry.acquire_result_parse(bytes).map(Some).map_err(CommandError::from);
        let built = build::preview_parsed(
            &mut source,
            &prepared.evidence,
            &prepared.manifest_sha256,
            manifest,
            prepared.aggregate.as_ref(),
            &prepared.exclusions,
            &mut load_basis,
            &mut reserve,
        )?;
        expect_record(&built, &expected).map_err(refusal)?;
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
        let staged = stage_save(&transaction, &built, &fields, confirmed_revision.as_deref(), limits, now, &memory).map_err(refusal)?;
        // The record, the held result's parse and the builder's leases end here.
        drop(built);
        drop(source);
        drop(Building { _working: working, _scratch: scratch, _reading: reading });
        (path, migration, document_version, staged)
    };
    finish_save(&transaction, &fields, staged, create, path, &migration, document_version, &memory).map_err(refusal).and_then(|serialized| {
        transaction.commit().map_err(|error| refusal(error.to_string()))?;
        drop(sqlite);
        Ok(serialized)
    })
}

/// The project's fields a save stores besides the result and the record.
struct ProjectFields<'f> {
    project_id: &'f str,
    project_name: &'f str,
    model: &'f Value,
    editor_intents: &'f Value,
    proposal: &'f Value,
    selected_review_target: &'f Value,
    model_hash: &'f Value,
    small: &'f [(&'static str, u64)],
}

/// RFC 8785 text of `value` streamed into `out`.
fn canonical<S: Sink>(value: &Value, out: &mut S) {
    Writer::new(out, Style::Canonical).value(value);
}

/// `computeProjectEnvelopeHash`'s value over the stored values (module docs),
/// streamed: the payload's members in RFC 8785's order.
fn envelope_payload_sha256(run: Option<&Built<'_>>, model: &Value, editor_intents: &Value, proposal: &Value, selected_review_target: &Value, model_hash: &Value) -> String {
    let mut hasher = Sha256::new();
    hasher.put(b"{\"analysis_run\":");
    match run {
        Some(built) => write_canonical_record(built, &mut hasher),
        None => hasher.put(b"null"),
    }
    hasher.put(b",\"editor_intents\":");
    canonical(editor_intents, &mut hasher);
    hasher.put(b",\"mechanics_result\":");
    match run {
        Some(built) => canonical(built.source, &mut hasher),
        None => hasher.put(b"null"),
    }
    hasher.put(b",\"model\":");
    canonical(model, &mut hasher);
    hasher.put(b",\"model_hash\":");
    canonical(model_hash, &mut hasher);
    hasher.put(b",\"proposal\":");
    canonical(proposal, &mut hasher);
    hasher.put(b",\"selected_review_target\":");
    canonical(selected_review_target, &mut hasher);
    hasher.put(b"}");
    hex(&hasher.finalize())
}

/// The project row's measured lengths for given result and record lengths.
struct ProjectLengths {
    values: Vec<Measured>,
    row: u64,
}

fn project_lengths(
    transaction: &Transaction<'_>,
    fields: &ProjectFields<'_>,
    result_length: u64,
    record_length: u64,
    envelope_length: u64,
    now: i64,
    exists: bool,
) -> Result<ProjectLengths, String> {
    let (project_id, project_name) = (fields.project_id, fields.project_name);
    let length = |name: &str| fields.small.iter().find(|(key, _)| *key == name).map_or(0, |(_, bytes)| *bytes);
    let search = 4 + length("model_json") + length("editor_intents_json") + length("proposal_json") + length("selected_review_target_json") + record_length;
    let text = Cell::Text;
    let row = row_record(
        transaction,
        "local_projects",
        &[
            ("project_id", Column::Is(text(project_id.len() as u64))),
            ("project_name", Column::Is(text(project_name.len() as u64))),
            ("model_json", Column::Is(text(length("model_json")))),
            ("editor_intents_json", Column::Is(text(length("editor_intents_json")))),
            ("proposal_json", Column::Is(text(length("proposal_json")))),
            ("selected_review_target_json", Column::Is(text(length("selected_review_target_json")))),
            ("mechanics_result_json", Column::Is(text(result_length))),
            ("analysis_run_json", Column::Is(text(record_length))),
            ("model_hash_json", Column::Is(text(length("model_hash_json")))),
            ("project_envelope_hash_json", Column::Is(text(envelope_length))),
            ("created_at_unix", if exists { Column::Kept } else { Column::Is(Cell::Integer(now)) }),
            ("updated_at_unix", Column::Is(Cell::Integer(now))),
        ],
        exists.then_some(("project_id", project_id)),
    )?;
    // The search row's content record: its rowid alias (stored null), the id, the name and the text.
    let content = record_size(&[Cell::Null, text(project_id.len() as u64), text(project_name.len() as u64), text(search)]);
    let values = vec![
        measured("project id", project_id.len() as u64),
        measured("project name", project_name.len() as u64),
        measured("model", length("model_json")),
        measured("editor intents", length("editor_intents_json")),
        measured("proposal", length("proposal_json")),
        measured("selected review target", length("selected_review_target_json")),
        measured("result", result_length),
        measured("analysis-run record", record_length),
        measured("model hash", length("model_hash_json")),
        measured("project envelope hash", envelope_length),
        measured("search text", search),
        measured("project row", row),
        measured("project index key", index_record(&[text(project_id.len() as u64)])),
        measured("search row", content),
    ];
    Ok(ProjectLengths { values, row })
}

/// The bytes the search row being replaced holds (FTS5 reads it to remove its
/// terms): the stored slots it was made of, from their record headers.
fn replaced_search_bytes(transaction: &Transaction<'_>, project_id: &str) -> Result<u64, String> {
    let stored: Option<i64> = transaction
        .query_row(
            "SELECT 4 + COALESCE(octet_length(model_json), 0) + COALESCE(octet_length(editor_intents_json), 0) + COALESCE(octet_length(proposal_json), 0)
                 + COALESCE(octet_length(selected_review_target_json), 0) + COALESCE(octet_length(analysis_run_json), 0) FROM local_projects WHERE project_id = ?1",
            params![project_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(|error| stored_read::sql_error(&error))?;
    Ok(u64::try_from(stored.unwrap_or(0)).unwrap_or(0))
}

/// Save Project's steps inside its transaction while the record is alive:
/// the exact lengths with the result against the connection's limit, else
/// without it (the model-only save, or [`PROJECT_SAVE_TOO_LARGE`] with nothing
/// written); the withheld-run guard for a model-only save; the envelope hash
/// over what will be stored; the result's and record's texts.
fn stage_save(
    transaction: &Transaction<'_>,
    built: &Built<'_>,
    fields: &ProjectFields<'_>,
    confirmed_revision: Option<&str>,
    limits: &SaveLimits,
    now: &dyn Fn() -> Result<i64, String>,
    memory: &dyn ReadMemory,
) -> Result<StagedSave, String> {
    // Exact lengths, nothing allocated: the result's stored form and the record's.
    let mut count = Count::default();
    Writer::new(&mut count, Style::Stored).value(built.source);
    let result_length = count.0;
    let record_length = built.record_bytes;
    let limit_bytes = length_limit(transaction);
    let exists = transaction
        .query_row("SELECT 1 FROM local_projects WHERE project_id = ?1", params![fields.project_id], |_| Ok(()))
        .optional()
        .map_err(|error| stored_read::sql_error(&error))?
        .is_some();
    let now = now()?;
    // The evidence's value has a fixed length, so its text's length is known before the hash.
    let envelope_length = stored_read::serialized_length(&envelope_evidence(fields.model, &"0".repeat(64)))?;
    let with = project_lengths(transaction, fields, result_length, record_length, envelope_length, now, exists)?;
    let refused_with = first_over(&with.values, limit_bytes).cloned();
    let row_with_result = with.row;
    let saved = refused_with.is_none();
    let lengths = if saved { with } else { project_lengths(transaction, fields, 4, 4, envelope_length, now, exists)? };
    if let Some(over) = first_over(&lengths.values, limit_bytes) {
        return Err(format!(
            "{PROJECT_SAVE_TOO_LARGE} ({}_bytes={}, limit_bytes={limit_bytes}): project `{}`'s {} is {} bytes, above this store's SQLite length limit of {limit_bytes} bytes even without the result; nothing was saved.",
            over.what.replace(' ', "_"),
            over.bytes,
            stored_read::shown(fields.project_id),
            over.what,
            over.bytes
        ));
    }
    if !saved {
        // The model-only save never replaces a stored run withheld for its size
        // without the user's confirmed Replace (as a save with no result).
        crate::refuse_dropping_withheld_run(transaction, fields.project_id, confirmed_revision, &Value::Null, limits.withheld_bytes)?;
    }
    let run = saved.then_some(built);
    let envelope = envelope_evidence(
        fields.model,
        &envelope_payload_sha256(run, fields.model, fields.editor_intents, fields.proposal, fields.selected_review_target, fields.model_hash),
    );
    let mut model_hasher = Sha256::new();
    canonical(fields.model, &mut model_hasher);
    let stored_model_hash = format!("sha256:{}", hex(&model_hasher.finalize()));
    let mut leases = Vec::new();
    let (result_json, record_json) = if saved {
        let (result_json, result_lease) = leased_text(memory, "the project's result text", result_length, |out| {
            Writer::new(out, Style::Stored).value(built.source);
            Ok(())
        })?;
        let (record, record_lease) =
            record_bytes(built, record_length, |bytes| memory.reserve("the project's analysis-run record", bytes).map_err(CommandError::Command)).map_err(command_text)?;
        leases.extend([result_lease, record_lease]);
        (result_json, String::from_utf8(record).map_err(|_| "STORE-READ: the analysis-run record is not UTF-8".to_string())?)
    } else {
        ("null".to_string(), "null".to_string())
    };
    Ok(StagedSave {
        saved,
        refused_with,
        row_with_result,
        lengths,
        limit_bytes,
        now,
        envelope,
        stored_model_hash,
        result_json,
        record_json,
        leases,
        run_id: built.run_id.to_string(),
        result_length,
        record_length,
    })
}

/// The session's envelope evidence (`computeProjectEnvelopeHash`'s object)
/// for the payload hash `value` (bare hex).
fn envelope_evidence(model: &Value, value: &str) -> Value {
    json!({
        "algorithm": "sha256",
        "canonicalization": "rfc8785_jcs",
        "payload_scope": "project_envelope_payload",
        "payload_excludes": "storage_summary_and_envelope_hash_carrier_fields",
        "payload_ref": model.pointer("/project/id").cloned().unwrap_or(Value::Null),
        "value": format!("sha256:{value}"),
        "hash_status": "computed_local_preview"
    })
}

/// Save Project after the record ended: the small texts and the search text at
/// their exact lengths, SQLite's copies and records leased, the rows written
/// (a row SQLite still refuses as too big is refused by name, the
/// transaction left to roll back), and the reply serialized at its exact
/// length.
#[allow(clippy::too_many_arguments)]
fn finish_save(
    transaction: &Transaction<'_>,
    fields: &ProjectFields<'_>,
    staged: StagedSave,
    create: bool,
    path: PathBuf,
    migration: &crate::StoreMigrationEvidence,
    document_version: crate::ModelDocumentVersionStatus,
    memory: &dyn ReadMemory,
) -> Result<Serialized, String> {
    let StagedSave { saved, refused_with, row_with_result, lengths, limit_bytes, now, envelope, stored_model_hash, result_json, record_json, leases, run_id, result_length, record_length } = staged;
    let (model_json, model_lease) = leased_json(memory, "the project's model text", fields.model)?;
    let (editor_json, editor_lease) = leased_json(memory, "the project's editor intents text", fields.editor_intents)?;
    let (proposal_json, proposal_lease) = leased_json(memory, "the project's proposal text", fields.proposal)?;
    let (selected_json, selected_lease) = leased_json(memory, "the project's review target text", fields.selected_review_target)?;
    let (model_hash_json, model_hash_lease) = leased_json(memory, "the project's model hash text", fields.model_hash)?;
    let (envelope_json, envelope_lease) = leased_json(memory, "the project's envelope hash text", &envelope)?;
    let texts = crate::ProjectTexts {
        model_json: &model_json,
        editor_intents_json: &editor_json,
        proposal_json: &proposal_json,
        selected_review_target_json: &selected_json,
        mechanics_result_json: &result_json,
        analysis_run_json: &record_json,
        model_hash_json: &model_hash_json,
        project_envelope_hash_json: &envelope_json,
    };
    let search_lease = memory.reserve("the project's search text", stored_read::exact_text_bytes(texts.search_text_length()))?;
    let search_text = texts.search_text();
    // SQLite copies each bound text, builds the rows' records, and reads the
    // search row it replaces.
    let bound = lengths.values.iter().filter(|length| !length.what.ends_with(" row") && !length.what.ends_with(" key")).fold(0u64, |sum, length| sum.saturating_add(length.bytes));
    let records = lengths.values.iter().filter(|length| length.what.ends_with(" row")).fold(0u64, |sum, length| sum.saturating_add(length.bytes));
    let statement = memory.reserve("SQLite's copies of the project's rows", stored_read::total(&[bound, records, replaced_search_bytes(transaction, fields.project_id)?]))?;
    crate::write_project_texts(transaction, fields.project_id, fields.project_name, &texts, &search_text, now).map_err(|error| {
        let text = stored_read::sql_error(&error);
        if text.starts_with(stored_read::SQLITE_TOO_BIG) {
            format!("{PROJECT_SAVE_RESULT_TOO_LARGE}: SQLite refused the project's rows as too big ({text}); the transaction was rolled back and nothing was saved.")
        } else {
            error.to_string()
        }
    })?;
    drop((statement, search_text, search_lease));
    let (stub_result, stub_run) = if saved { (json!({ "run_id": run_id }), json!({ "analysis_run": { "run_id": run_id } })) } else { (Value::Null, Value::Null) };
    let saved_message = if create { "Created local SQLite project snapshot without external file copies." } else { "Saved current project model snapshot without external file copies." };
    let result = match refused_with {
        None => SavedResult::Saved { run_id, result_bytes: result_length, record_bytes: record_length },
        Some(over) => SavedResult::NotSaved {
            code: PROJECT_SAVE_RESULT_TOO_LARGE,
            message: format!(
                "{PROJECT_SAVE_RESULT_TOO_LARGE} (result_bytes={result_length}, record_bytes={record_length}, row_bytes={row_with_result}, limit_bytes={limit_bytes}): the result was not saved. Its {} is {} bytes, above this store's SQLite length limit of {limit_bytes} bytes, so the project was saved with no result (its stored result and analysis-run record are empty). The current solve stays open in this session.",
                over.what, over.bytes
            ),
            result_bytes: result_length,
            record_bytes: record_length,
            row_bytes: row_with_result,
            limit_bytes,
            refused: over,
        },
    };
    let message = match &result {
        SavedResult::Saved { .. } => saved_message.to_string(),
        SavedResult::NotSaved { message, .. } => format!("{saved_message} {message}"),
    };
    let reply = NativeProjectSave {
        summary: crate::project_summary(
            fields.project_id.to_string(),
            fields.project_name.to_string(),
            fields.model,
            path,
            migration,
            fields.editor_intents,
            fields.proposal,
            fields.selected_review_target,
            &stub_result,
            &stub_run,
            fields.model_hash,
            &envelope,
            message,
        ),
        proposal: fields.proposal.clone(),
        selected_review_target: fields.selected_review_target.clone(),
        model_hash: fields.model_hash.clone(),
        project_envelope_hash: envelope,
        model_document_version: serde_json::to_value(document_version).map_err(|error| error.to_string())?,
        stored_model_hash,
        result,
    };
    let serialized = stored_read::serialize_metered(memory, SAVE_COMMAND, stored_read::Metered::new(reply))?;
    drop((model_json, model_lease, editor_json, editor_lease, proposal_json, proposal_lease, selected_json, selected_lease));
    drop((model_hash_json, model_hash_lease, envelope_json, envelope_lease, result_json, record_json, leases));
    Ok(serialized)
}

#[cfg(test)]
#[path = "native_persistence_tests.rs"]
mod tests;
