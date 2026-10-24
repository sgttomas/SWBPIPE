//! Memory admission for reads of stored rows (W-12; DEL-14-01 Design
//! `desktop-states-runs-comparison.md` §2, §3 and §4.2).
//!
//! The history commands (`read_model_state`, `read_analysis_run`, the two
//! lists, the label edits and deletes, Save Model State and Record Run) are
//! admitted with their argument ingress, but what they then read from the store
//! (a snapshot, a received result, an input manifest, a run record, labels) can
//! be MiB. A command therefore prices its work and takes interactive memory-pool
//! leases ([`crate::memory_pool::MemoryPool::acquire_interactive`], the
//! admission Save Local Project's ingress lease uses: the pool and fresh
//! headroom, with the command's live leases together within the per-invocation
//! budget) before it loads, parses or builds anything:
//!
//! 1. **Before loading** ([`loaded_text_bytes`], [`string_list_bytes`]): each
//!    column it loads, labels and relationship lists included, priced from the
//!    columns' `octet_length`s and counts, read without loading the values. The
//!    measurement and every load it prices run in one read transaction
//!    ([`read_transaction`], or the writer's `BEGIN IMMEDIATE`): the metered
//!    readers take a `Transaction`, so a row another connection writes after
//!    the measurement is not loaded under it (WAL snapshot isolation).
//! 2. **Before parsing** ([`parse_bytes`]): each loaded text's
//!    `serde_json::Value` materialization, bounded by the allocation-free ingress
//!    prescan. A text the prescan refuses is refused (as the row's integrity
//!    failure) before any deserialization of it.
//! 3. **Before building a hash or canonical text** ([`jcs_text`],
//!    [`checked_sha256`], `model_states::record_hash_hex`): the text's exact
//!    length is counted first without allocating ([`jcs_length`],
//!    [`serialized_length`], `model_states::sorted_compact_length`), never
//!    estimated from a stored length (escapes expand a character up to six
//!    bytes, a number's canonical form up to five times its stored form), and
//!    the text is reserved at that length before it is built.
//! 4. **The response** ([`respond_metered`]): its exact serialized length is
//!    counted first, a buffer of exactly that capacity is reserved, and the value
//!    is serialized into it while every reservation the value lives under is
//!    still held ([`Metered`], which cannot be taken apart: a value read from
//!    the store leaves its reservations only through serialization).
//!
//! S3's comparison of two stored results (design §4.2) reuses the same pricing.
//! The counting-allocator tests in `analysis_runs_tests.rs` check a command's
//! priced peak, through its serialized response, against what it reserved.

use crate::command_ingress::CommandError;
use crate::ingress::{allocation_envelope, prescan};
use crate::memory_pool::Lease;
use crate::solve_jobs::{SolveJobError, SolveJobRegistry};
use open_pipe_stress_canonical_json::{canonical_json, canonical_json_checked_v1_text, canonical_number};
use open_pipe_stress_operation_applier::sha256_hex;
use rusqlite::{Connection, Transaction, TransactionBehavior};
use serde::Serialize;
use serde_json::Value;
use std::cell::Cell;
use std::rc::Rc;
use tauri::ipc::Response;

/// The refusal of a read the memory pool does not admit.
pub(crate) const STORE_READ_MEMORY_UNAVAILABLE: &str = "STORE-READ-MEMORY-UNAVAILABLE";

/// What a read holds besides its priced columns: the store connection and its
/// statements, the migration evidence, small columns and labels.
pub(crate) const READ_OVERHEAD_BYTES: u64 = 512 * 1024;

/// A reservation; its leases are released when it is dropped.
#[derive(Debug, Default)]
pub(crate) struct ReadReservation {
    /// Held for their release on drop.
    _leases: Vec<Lease>,
    bytes: u64,
    /// The read's running total of held bytes (per-invocation budget), when metered.
    held: Option<Rc<Cell<u64>>>,
}

impl Drop for ReadReservation {
    fn drop(&mut self) {
        if let Some(held) = &self.held {
            held.set(held.get().saturating_sub(self.bytes));
        }
    }
}

/// Where a read's memory comes from.
pub(crate) trait ReadMemory {
    /// Reserves `bytes` for `what` (a description of the stored data), or
    /// refuses by name.
    fn reserve(&self, what: &str, bytes: u64) -> Result<ReadReservation, String>;
}

/// The application's memory pool, through the job registry. Each reservation is
/// one interactive lease; together with the command's argument lease, which the
/// accounting starts from (repair r4), the command's live leases stay within the
/// per-invocation budget.
pub(crate) struct PoolReadMemory<'a> {
    registry: &'a SolveJobRegistry,
    held: Rc<Cell<u64>>,
    /// The most the read held at once (checked against its measured peak by the tests).
    #[cfg_attr(not(test), allow(dead_code))]
    peak: Cell<u64>,
}

impl<'a> PoolReadMemory<'a> {
    /// The memory of a command admitted with `argument_lease`: every later
    /// admission counts the argument lease with the command's reservations.
    pub(crate) fn new(registry: &'a SolveJobRegistry, argument_lease: &Lease) -> Self {
        let argument = argument_lease.bytes();
        Self { registry, held: Rc::new(Cell::new(argument)), peak: Cell::new(argument) }
    }

    /// A command's memory with no argument lease (tests).
    #[cfg(test)]
    pub(crate) fn unseeded(registry: &'a SolveJobRegistry) -> Self {
        Self { registry, held: Rc::new(Cell::new(0)), peak: Cell::new(0) }
    }

    #[cfg(test)]
    pub(crate) fn peak_reserved(&self) -> u64 {
        self.peak.get()
    }

    /// The bytes the command's live reservations hold now.
    #[cfg(test)]
    pub(crate) fn held(&self) -> u64 {
        self.held.get()
    }
}

impl ReadMemory for PoolReadMemory<'_> {
    fn reserve(&self, what: &str, bytes: u64) -> Result<ReadReservation, String> {
        let budget = self.registry.pool_status().invocation_budget_bytes;
        let required = self.held.get().saturating_add(bytes);
        if required > budget {
            return Err(format!(
                "{STORE_READ_MEMORY_UNAVAILABLE}: reading {what} needs {required} bytes in all, above the per-invocation budget of {budget} bytes; nothing more was read."
            ));
        }
        match self.registry.acquire_interactive_ingress(bytes) {
            Ok(lease) => {
                self.held.set(required);
                self.peak.set(self.peak.get().max(required));
                Ok(ReadReservation { _leases: vec![lease], bytes, held: Some(Rc::clone(&self.held)) })
            }
            Err(SolveJobError::ResourceUnavailable { refusal, .. }) => Err(format!(
                "{STORE_READ_MEMORY_UNAVAILABLE}: reading {what} needs {bytes} bytes of the memory pool, which refused them ({}); nothing more was read.",
                serde_json::to_string(&refusal).unwrap_or_else(|_| "refused".to_string())
            )),
            Err(SolveJobError::InvalidRequest { message }) => Err(format!("{STORE_READ_MEMORY_UNAVAILABLE}: reading {what}: {message}")),
        }
    }
}

/// No pool: for a store path whose memory a caller has already priced (a write
/// admitted with its arguments) and for tests that are not about memory.
pub(crate) struct UnmeteredReadMemory;

impl ReadMemory for UnmeteredReadMemory {
    fn reserve(&self, _what: &str, bytes: u64) -> Result<ReadReservation, String> {
        Ok(ReadReservation { _leases: Vec::new(), bytes, held: None })
    }
}

fn envelope(bytes: u64) -> u64 {
    allocation_envelope(bytes).unwrap_or(u64::MAX)
}

/// Loading one stored text column of `length` bytes: SQLite's page cache and
/// the statement's value of it, and the `String` it is copied into.
pub(crate) fn loaded_text_bytes(length: u64) -> u64 {
    envelope(length).saturating_mul(3)
}

/// One text copy of about `length` bytes (a canonical form or a serialization
/// for a hash), built by a growing buffer: the old and new buffers of its last
/// doubling are live together.
pub(crate) fn text_copy_bytes(length: u64) -> u64 {
    envelope(length).saturating_mul(3)
}

/// A text written into a buffer of exactly `length` bytes (its length counted
/// first): the allocation itself.
pub(crate) fn exact_text_bytes(length: u64) -> u64 {
    envelope(length)
}

/// The most characters of a stored name or id a message, receipt or refusal
/// quotes (repair r4): a longer one is cut there with an ellipsis, so every
/// message built from stored data has a small constant bound (at most six
/// serialized bytes a character) and needs no reservation. The structured
/// fields keep the full, metered values.
pub(crate) const MAX_SHOWN_CHARS: usize = 120;

/// `text` as a message quotes it: whole when it has at most
/// [`MAX_SHOWN_CHARS`] characters, else its first ones and `…`.
pub(crate) fn shown(text: &str) -> std::borrow::Cow<'_, str> {
    match text.char_indices().nth(MAX_SHOWN_CHARS) {
        None => std::borrow::Cow::Borrowed(text),
        Some((cut, _)) => std::borrow::Cow::Owned(format!("{}…", &text[..cut])),
    }
}

/// The most characters of any error message a command returns (repair r5): a
/// last net after every source is bounded (each stored value quoted through
/// [`shown`], each serde or SQLite error described by [`json_error`] or
/// [`sql_error`], never by its own text).
pub(crate) const MAX_MESSAGE_CHARS: usize = 1024;

/// `message` cut to [`MAX_MESSAGE_CHARS`] characters, then `…`.
pub(crate) fn capped(message: String) -> String {
    match message.char_indices().nth(MAX_MESSAGE_CHARS) {
        None => message,
        Some((cut, _)) => format!("{}…", &message[..cut]),
    }
}

/// A serde_json error as a message describes it: its category and, for a
/// text, its line and column. Never its own text, which can quote the value.
pub(crate) fn json_error(error: &serde_json::Error) -> String {
    let category = match error.classify() {
        serde_json::error::Category::Io => "an I/O error",
        serde_json::error::Category::Syntax => "a syntax error",
        serde_json::error::Category::Data => "a value of another type or shape",
        serde_json::error::Category::Eof => "an unexpected end",
    };
    if error.line() > 0 {
        format!("{category} at line {} column {}", error.line(), error.column())
    } else {
        category.to_string()
    }
}

/// A SQLite error as a message describes it: SQLite's code and extended
/// code, and its message only when that is at most [`MAX_SHOWN_CHARS`]
/// printable characters (SQLite's own texts, such as a constraint or the
/// store's trigger codes); otherwise a fixed text. rusqlite has already copied
/// the message by the time it is an error; nothing longer is copied again.
pub(crate) fn sql_error(error: &rusqlite::Error) -> String {
    match error {
        rusqlite::Error::SqliteFailure(failure, message) => {
            let shown = message
                .as_deref()
                .filter(|text| text.chars().count() <= MAX_SHOWN_CHARS && !text.chars().any(char::is_control));
            match shown {
                Some(text) => format!("STORE-SQL-ERROR: SQLite {:?} ({}): {text}", failure.code, failure.extended_code),
                None => format!("STORE-SQL-ERROR: SQLite {:?} ({}): its message is not shown", failure.code, failure.extended_code),
            }
        }
        rusqlite::Error::QueryReturnedNoRows => "STORE-SQL-ERROR: the row is not there".to_string(),
        rusqlite::Error::InvalidColumnType(index, _, kind) => format!("STORE-SQL-ERROR: column {index} holds a {kind} value"),
        rusqlite::Error::FromSqlConversionFailure(index, kind, _) => format!("STORE-SQL-ERROR: column {index} ({kind}) could not be read"),
        rusqlite::Error::IntegralValueOutOfRange(index, _) => format!("STORE-SQL-ERROR: column {index} is out of range"),
        rusqlite::Error::Utf8Error(_) => "STORE-SQL-ERROR: a stored text is not UTF-8".to_string(),
        rusqlite::Error::SqlInputError { error, offset, .. } => {
            format!("STORE-SQL-ERROR: SQLite {:?} ({}): the statement is invalid at byte {offset}", error.code, error.extended_code)
        }
        _ => "STORE-SQL-ERROR: the store could not be read or written".to_string(),
    }
}

/// The bytes the store keeps in an identity or time column (repair r5),
/// against what the app writes: ids (`state:` or `runrec:` and 32 hex, a
/// producer's run id, a project id) are short, and no command can name one
/// longer than its 4 KiB argument budget; kinds are words; times are
/// 24-byte ISO texts; hashes are 64 hex, or `sha256:` and 64 hex. A row
/// holding a longer value is refused, before it is loaded, as the row's
/// integrity failure.
pub(crate) const MAX_ID_BYTES: i64 = 4096;
pub(crate) const MAX_KIND_BYTES: i64 = 64;
pub(crate) const MAX_TIME_BYTES: i64 = 32;
pub(crate) const MAX_HASH_BYTES: i64 = 80;

/// The most bytes of a verified record's `analysis_status` a summary copies
/// out of the record, whose reservation it outlives (repair r5): the app writes
/// a few short status words.
pub(crate) const MAX_STATUS_BYTES: u64 = 4096;

/// A record's `analysis_status` (an empty list when it has none), copied only
/// when it serializes to at most [`MAX_STATUS_BYTES`].
pub(crate) fn bounded_status(value: Option<&Value>) -> Result<Value, String> {
    let Some(value) = value else { return Ok(Value::Array(Vec::new())) };
    let length = serialized_length(value)?;
    if length > MAX_STATUS_BYTES {
        return Err(format!("its analysis status holds {length} bytes; at most {MAX_STATUS_BYTES} are kept"));
    }
    Ok(value.clone())
}

/// A loaded identity or time column: its SQL, its name, its bound.
pub(crate) type BoundedColumn = (&'static str, &'static str, i64);

/// SQL naming the first of `columns` whose stored value exceeds its bound, or
/// NULL.
pub(crate) fn oversized_column(columns: &[BoundedColumn]) -> String {
    let arms: String = columns
        .iter()
        .map(|(column, name, max)| format!(" WHEN COALESCE(octet_length({column}), 0) > {max} THEN '{name}'"))
        .collect();
    format!("(CASE{arms} ELSE NULL END)")
}

/// SQL summing the stored bytes of `columns`.
pub(crate) fn columns_bytes(columns: &[BoundedColumn]) -> String {
    let terms: Vec<String> = columns.iter().map(|(column, _, _)| format!("COALESCE(octet_length({column}), 0)")).collect();
    format!("({})", terms.join(" + "))
}

/// The bound of the column `name` in `columns`.
pub(crate) fn column_bound(columns: &[BoundedColumn], name: &str) -> i64 {
    columns.iter().find(|(_, column, _)| *column == name).map(|(_, _, max)| *max).unwrap_or(0)
}

/// The bytes of a stored text column a message reads ([`shown_column`]): enough
/// for [`MAX_SHOWN_CHARS`] characters and one more (UTF-8 takes at most four
/// bytes a character).
pub(crate) const SHOWN_COLUMN_BYTES: usize = (MAX_SHOWN_CHARS + 1) * 4;

/// SQL selecting the head of a stored text `column` for a message, without
/// loading the rest into the process: its first [`SHOWN_COLUMN_BYTES`] bytes
/// and its byte length (read as `(Vec<u8>, i64)`, then [`shown_head`]). Bytes,
/// not characters, as SQLite's character functions stop at a NUL.
pub(crate) fn shown_column(column: &str) -> String {
    format!("substr(CAST({column} AS BLOB), 1, {SHOWN_COLUMN_BYTES}), octet_length({column})")
}

/// A column head read by [`shown_column`], as [`shown`] quotes the whole text.
pub(crate) fn shown_head(head: &[u8], length: i64) -> String {
    let text = String::from_utf8_lossy(head);
    let shown = shown(&text);
    if matches!(shown, std::borrow::Cow::Borrowed(_)) && (head.len() as i64) < length {
        // Only a cut inside the last character can leave it short of the cut.
        format!("{}…", text.trim_end_matches('\u{fffd}'))
    } else {
        shown.into_owned()
    }
}

/// `value`'s serde_json text, written into a buffer of its exact length, which
/// is reserved in `memory` before it is written.
pub(crate) fn json_text<T: Serialize + ?Sized>(memory: &dyn ReadMemory, what: &str, value: &T) -> Result<Metered<String>, String> {
    let length = serialized_length(value)?;
    let reservation = memory.reserve(what, exact_text_bytes(length))?;
    let mut buffer = Vec::with_capacity(usize::try_from(length).map_err(|_| "STORE-READ: a length does not fit this platform".to_string())?);
    serde_json::to_writer(&mut buffer, value).map_err(|error| json_error(&error))?;
    let text = String::from_utf8(buffer).map_err(|_| "STORE-READ: a serialized text is not UTF-8".to_string())?;
    Ok(Metered::new(text).holding(reservation))
}

/// The `serde_json::Value` materialization of a loaded text, bounded by the
/// ingress prescan (which allocates nothing). A text the prescan does not accept
/// as one JSON value is refused here, before any deserialization of it
/// (repair r2): the caller names the row it belongs to.
pub(crate) fn parse_bytes(text: &str) -> Result<u64, String> {
    prescan(text.as_bytes())
        .map(|bound| bound.materialization_bytes)
        .map_err(|invalid| format!("the stored JSON is invalid ({} at byte {})", invalid.reason, invalid.offset))
}

/// Sums byte counts, saturating.
pub(crate) fn total(parts: &[u64]) -> u64 {
    parts.iter().fold(0u64, |sum, part| sum.saturating_add(*part))
}

/// A list of `count` stored strings of `bytes` bytes in all, loaded into a
/// `Vec<String>` (or pairs of them): each string's allocation, the strings'
/// headers in a growing vector, and SQLite's copy of each value.
pub(crate) fn string_list_bytes(count: u64, bytes: u64) -> u64 {
    let strings = bytes.saturating_add(count.saturating_mul(16)).saturating_mul(3);
    total(&[strings, text_copy_bytes(count.saturating_mul(48))])
}

/// A value read or built from stored or submitted data, with the reservations
/// it lives under. It cannot be taken apart: its value is borrowed ([`get`]),
/// transformed or combined with the reservations kept ([`map`], [`join`]), and
/// leaves its reservations only when it is serialized ([`respond_metered`]), or,
/// for an unmetered caller, when it holds no lease ([`into_unreserved`]).
///
/// [`get`]: Metered::get
/// [`map`]: Metered::map
/// [`join`]: Metered::join
/// [`into_unreserved`]: Metered::into_unreserved
#[derive(Debug)]
pub(crate) struct Metered<T> {
    value: T,
    held: Vec<ReadReservation>,
}

impl<T> Metered<T> {
    /// A value that holds nothing read from the store (or whose memory the
    /// caller's argument lease prices).
    pub(crate) fn new(value: T) -> Self {
        Self { value, held: Vec::new() }
    }

    pub(crate) fn holding(mut self, reservation: ReadReservation) -> Self {
        self.held.push(reservation);
        self
    }

    pub(crate) fn hold(&mut self, reservation: ReadReservation) {
        self.held.push(reservation);
    }

    pub(crate) fn get(&self) -> &T {
        &self.value
    }

    /// The rule for `map`, `join` and the commit-last `finish` steps: the
    /// closure moves the value into its result and does not capture or keep it
    /// (or a copy of a stored part of it) elsewhere. The type checks the
    /// result, not the captures, so this is a reviewed rule.
    pub(crate) fn map<U>(self, f: impl FnOnce(T) -> U) -> Metered<U> {
        Metered { value: f(self.value), held: self.held }
    }

    /// One value from two, holding both one's reservations.
    pub(crate) fn join<U, V>(mut self, other: Metered<U>, f: impl FnOnce(T, U) -> V) -> Metered<V> {
        self.held.extend(other.held);
        Metered { value: f(self.value, other.value), held: self.held }
    }

    /// The value of an unmetered read ([`UnmeteredReadMemory`]); a value that
    /// holds a lease is refused rather than outliving it.
    pub(crate) fn into_unreserved(self) -> Result<T, String> {
        if self.held.iter().any(|reservation| !reservation._leases.is_empty()) {
            return Err("STORE-READ-RESERVATION-HELD: a metered value leaves its reservations only when it is serialized.".to_string());
        }
        Ok(self.value)
    }

    /// The bytes this value's reservations hold.
    #[cfg(test)]
    pub(crate) fn reserved_bytes(&self) -> u64 {
        self.held.iter().map(|reservation| reservation.bytes).sum()
    }
}

/// The local store, opened (and migrated) as every command opens it; a
/// refusal's text is capped ([`capped`]): it comes from the shared opener.
pub(crate) fn open_store(path: &std::path::Path) -> Result<Connection, String> {
    crate::open_project_store(path).map(|(connection, _migration)| connection).map_err(capped)
}

/// A write transaction (`BEGIN IMMEDIATE`), its SQLite refusal described by
/// [`sql_error`].
pub(crate) fn write_transaction(connection: &mut Connection) -> Result<Transaction<'_>, String> {
    connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(|error| sql_error(&error))
}

/// A read transaction for a metered read (`BEGIN DEFERRED`): its first
/// statement, the measurement, fixes the snapshot every later load of the read
/// sees (the store is in WAL mode), so the loads are those measured.
pub(crate) fn read_transaction(connection: &mut Connection) -> Result<Transaction<'_>, String> {
    connection.transaction_with_behavior(TransactionBehavior::Deferred).map_err(|error| sql_error(&error))
}

/// Counts the bytes of a serialization without storing them.
struct ByteCount(u64);

impl std::io::Write for ByteCount {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0 = self.0.saturating_add(bytes.len() as u64);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// The exact length of `value`'s JSON serialization, found by a first pass that
/// writes nowhere (serde_json formats into fixed buffers, so the pass does not
/// allocate).
pub(crate) fn serialized_length<T: Serialize + ?Sized>(value: &T) -> Result<u64, String> {
    let mut count = ByteCount(0);
    serde_json::to_writer(&mut count, value).map_err(|error| json_error(&error))?;
    Ok(count.0)
}

/// The exact length of `open_pipe_stress_canonical_json::canonical_json(value)`
/// (RFC 8785), counted without building the text: strings and keys as serde_json
/// escapes them (the canonicalizer's escape table), numbers as the
/// canonicalizer renders them (a number's short text is formatted and dropped),
/// keys in any order (their order does not change the length).
pub(crate) fn jcs_length(value: &Value) -> u64 {
    match value {
        Value::Null | Value::Bool(true) => 4,
        Value::Bool(false) => 5,
        Value::Number(number) => canonical_number(number).len() as u64,
        Value::String(text) => serialized_length(text).unwrap_or(u64::MAX),
        Value::Array(items) => {
            let commas = items.len().saturating_sub(1) as u64;
            items.iter().fold(total(&[2, commas]), |length, item| length.saturating_add(jcs_length(item)))
        }
        Value::Object(map) => {
            let mut length = total(&[2, map.len().saturating_sub(1) as u64]);
            for (key, member) in map {
                length = total(&[length, serialized_length(key).unwrap_or(u64::MAX), 1, jcs_length(member)]);
            }
            length
        }
    }
}

/// Building a text of exactly `length` bytes with a growing `String` (the
/// canonicalizer's): its last doubling's old and new buffers (three times the
/// length), and the escaped copy it makes of each string with its own growth
/// (three times the length again, at most).
pub(crate) fn grown_text_bytes(length: u64) -> u64 {
    text_copy_bytes(length).saturating_mul(2)
}

/// `value`'s RFC 8785 canonical text, reserved at its exact length
/// ([`jcs_length`]) before it is built.
pub(crate) fn jcs_text(memory: &dyn ReadMemory, what: &str, value: &Value) -> Result<Metered<String>, String> {
    let length = jcs_length(value);
    let reservation = memory.reserve(what, grown_text_bytes(length))?;
    let text = canonical_json(value);
    if text.len() as u64 != length {
        return Err(format!("STORE-READ-LENGTH-MISCOUNTED: {what}'s canonical text is {} bytes, counted {length}.", text.len()));
    }
    Ok(Metered::new(text).holding(reservation))
}

/// Whether `text` is `value`'s canonical text. A text of another length is not,
/// and nothing is built; otherwise the canonical text is built within `memory`.
pub(crate) fn is_jcs_text(memory: &dyn ReadMemory, what: &str, value: &Value, text: &str) -> Result<bool, String> {
    if jcs_length(value) != text.len() as u64 {
        return Ok(false);
    }
    Ok(jcs_text(memory, what, value)?.get() == text)
}

/// SHA-256 (bare hex) of `value`'s canonical text, built within `memory`.
pub(crate) fn jcs_sha256(memory: &dyn ReadMemory, what: &str, value: &Value) -> Result<String, String> {
    Ok(sha256_hex(jcs_text(memory, what, value)?.get()))
}

/// SHA-256 (bare hex) of `value`'s `openpipestress_jcs_ijson_v1` checked
/// canonical text, as `canonical_json_checked_v1_text` of its serde_json text
/// gives it: the text (written into a buffer of its exact length), the checked
/// parse of it and the canonical text are each reserved before they are built.
pub(crate) fn checked_sha256(memory: &dyn ReadMemory, what: &str, value: &Value) -> Result<String, String> {
    let text = json_text(memory, what, value)?;
    checked_sha256_text(memory, what, text.get(), value)
}

/// SHA-256 (bare hex) of the checked canonical text of `text`, whose parse is
/// `parsed` (the caller's): the checked parse the canonicalizer makes (at the
/// prescan's bound) and the canonical text (at `parsed`'s exact canonical
/// length; the checked parse rebuilds the same value, or refuses) are reserved
/// before either is made.
pub(crate) fn checked_sha256_text(memory: &dyn ReadMemory, what: &str, text: &str, parsed: &Value) -> Result<String, String> {
    let parse = parse_bytes(text)?;
    let _work = memory.reserve(what, total(&[parse, grown_text_bytes(jcs_length(parsed))]))?;
    // The canonicalizer's refusal can quote the text (a duplicate member's
    // key, a serde error's value): it is cut to its first characters.
    let canonical = canonical_json_checked_v1_text(text).map_err(|error| shown(&error).into_owned())?;
    Ok(sha256_hex(&canonical))
}

/// A command's response, serialized, holding its buffer's reservation until
/// it is handed to Tauri ([`respond_serialized`]; repair r5). A mutating
/// command makes it before it commits ([`serialize_metered`]), so nothing
/// after the commit can fail and the text stays reserved across the commit.
#[derive(Debug)]
pub(crate) struct Serialized {
    text: String,
    held: ReadReservation,
}

impl Serialized {
    #[cfg(test)]
    pub(crate) fn text(&self) -> &str {
        &self.text
    }

    #[cfg(test)]
    pub(crate) fn reserved_bytes(&self) -> u64 {
        self.held.bytes
    }
}

/// Serializes `metered` within `memory`: the exact length is counted first,
/// a buffer of exactly that capacity is reserved, and the value is serialized
/// into it while its own reservations are still held; every reservation is
/// released once the text exists. A refusal names `what`'s response.
pub(crate) fn serialize_metered<T: Serialize>(memory: &dyn ReadMemory, what: &str, metered: Metered<T>) -> Result<Serialized, String> {
    let length = serialized_length(&metered.value)?;
    let length_usize = usize::try_from(length).map_err(|_| "STORE-READ: a length does not fit this platform".to_string())?;
    let buffer_reservation = memory.reserve(&format!("the response to {what}"), envelope(length))?;
    let mut buffer = Vec::with_capacity(length_usize);
    serde_json::to_writer(&mut buffer, &metered.value).map_err(|error| json_error(&error))?;
    let text = String::from_utf8(buffer).map_err(|_| "STORE-READ: a serialized text is not UTF-8".to_string())?;
    drop(metered);
    Ok(Serialized { text, held: buffer_reservation })
}

/// The response of a mutating command, serialized before it committed: the
/// text is handed to Tauri, then its reservation is released.
pub(crate) fn respond_serialized<E>(outcome: Result<Serialized, CommandError<E>>) -> Result<Response, CommandError<E>> {
    let Serialized { text, held } = outcome?;
    let response = Response::new(text);
    drop(held);
    Ok(response)
}

/// Serializes a metered read's response within `memory`: the response's exact
/// length is counted first, a buffer of exactly that capacity is reserved, and
/// the value is serialized into it while the value's own reservations are still
/// held. Every reservation is released only once the serialized text exists.
/// Residual: the `Response` handed to Tauri, and any copy the IPC layer makes of
/// it, outlive the command and are not ours to price (as for every command).
pub(crate) fn respond_metered<T: Serialize, E>(
    memory: &dyn ReadMemory,
    what: &str,
    outcome: Result<Metered<T>, CommandError<E>>,
) -> Result<Response, CommandError<E>>
where
    E: From<String>,
{
    let metered = outcome?;
    let length = serialized_length(&metered.value).map_err(CommandError::Response)?;
    let length_usize = usize::try_from(length).map_err(|_| CommandError::Response("STORE-READ: a length does not fit this platform".to_string()))?;
    let buffer_reservation = memory.reserve(&format!("the response to {what}"), envelope(length)).map_err(|error| CommandError::Command(E::from(error)))?;
    let mut buffer = Vec::with_capacity(length_usize);
    serde_json::to_writer(&mut buffer, &metered.value).map_err(|error| CommandError::Response(json_error(&error)))?;
    let text = String::from_utf8(buffer).map_err(|_| CommandError::Response("STORE-READ: a serialized text is not UTF-8".to_string()))?;
    let response = Response::new(text);
    drop(metered);
    drop(buffer_reservation);
    Ok(response)
}
