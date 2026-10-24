//! Memory admission for reads of stored rows (W-12; DEL-14-01 Design
//! `desktop-states-runs-comparison.md` §2, §3 and §4.2).
//!
//! The small history commands (`read_model_state`, `read_analysis_run`, the two
//! lists, the label edits and deletes) are admitted with a few KiB of argument
//! ingress, but what they then read from the store (a snapshot, a received
//! result, an input manifest, a run record, labels) can be MiB. A read therefore
//! prices its work from the stored columns' byte lengths, read with
//! `octet_length` and counts without loading the values, and takes interactive
//! memory-pool leases ([`crate::memory_pool::MemoryPool::acquire_interactive`],
//! the admission Save Local Project's ingress lease uses: the pool and fresh
//! headroom, with the read's live leases together within the per-invocation
//! budget) before it loads or parses anything:
//!
//! 1. **Before loading** ([`loaded_text_bytes`], [`text_copy_bytes`],
//!    [`string_list_bytes`]): each column it loads (SQLite's page cache, the
//!    statement's value and the `String` it is copied into), labels and
//!    relationship lists included, and each text copy the read makes (a
//!    canonical or serialized form for a hash). A refusal loads nothing:
//!    `STORE-READ-MEMORY-UNAVAILABLE`, naming what was to be read.
//! 2. **Before parsing** ([`parse_bytes`]): each loaded text's
//!    `serde_json::Value` materialization, bounded by the allocation-free ingress
//!    prescan, times the number of trees the read holds at once. A text the
//!    prescan refuses is refused (as the row's integrity failure) before any
//!    deserialization of it.
//! 3. **The response** ([`respond_metered`]): its exact serialized length is
//!    counted first without allocating, a buffer of exactly that capacity is
//!    reserved, and the value is serialized into it while every reservation the
//!    value lives under is still held ([`Metered`]).
//!
//! S3's comparison of two stored results (design §4.2) reuses the same pricing.
//! The counting-allocator tests in `analysis_runs_tests.rs` check a read's
//! priced peak, through its serialized response, against what it reserved.

use crate::command_ingress::CommandError;
use crate::ingress::{allocation_envelope, prescan};
use serde::Serialize;
use tauri::ipc::Response;
use crate::memory_pool::Lease;
use crate::solve_jobs::{SolveJobError, SolveJobRegistry};
use std::cell::Cell;
use std::rc::Rc;

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
/// one interactive lease; together the read's live reservations stay within the
/// per-invocation budget, as one command's lease does.
pub(crate) struct PoolReadMemory<'a> {
    registry: &'a SolveJobRegistry,
    held: Rc<Cell<u64>>,
    /// The most the read held at once (checked against its measured peak by the tests).
    #[cfg_attr(not(test), allow(dead_code))]
    peak: Cell<u64>,
}

impl<'a> PoolReadMemory<'a> {
    pub(crate) fn new(registry: &'a SolveJobRegistry) -> Self {
        Self { registry, held: Rc::new(Cell::new(0)), peak: Cell::new(0) }
    }

    #[cfg(test)]
    pub(crate) fn peak_reserved(&self) -> u64 {
        self.peak.get()
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

/// A read's value with the reservations it lives under. The reservations are
/// dropped only after the value has been serialized ([`respond_metered`]).
#[derive(Debug)]
pub(crate) struct Metered<T> {
    pub(crate) value: T,
    held: Vec<ReadReservation>,
}

impl<T> Metered<T> {
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

    /// Moves `other`'s reservations here and returns its value.
    pub(crate) fn absorb<U>(&mut self, other: Metered<U>) -> U {
        self.held.extend(other.held);
        other.value
    }

    pub(crate) fn map<U>(self, f: impl FnOnce(T) -> U) -> Metered<U> {
        Metered { value: f(self.value), held: self.held }
    }

    pub(crate) fn into_parts(self) -> (T, Vec<ReadReservation>) {
        (self.value, self.held)
    }
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
pub(crate) fn serialized_length<T: Serialize>(value: &T) -> Result<u64, String> {
    let mut count = ByteCount(0);
    serde_json::to_writer(&mut count, value).map_err(|error| error.to_string())?;
    Ok(count.0)
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
    let length_usize = usize::try_from(length).map_err(|error| CommandError::Response(error.to_string()))?;
    let buffer_reservation = memory.reserve(&format!("the response to {what}"), envelope(length)).map_err(|error| CommandError::Command(E::from(error)))?;
    let mut buffer = Vec::with_capacity(length_usize);
    serde_json::to_writer(&mut buffer, &metered.value).map_err(|error| CommandError::Response(error.to_string()))?;
    let text = String::from_utf8(buffer).map_err(|error| CommandError::Response(error.to_string()))?;
    let response = Response::new(text);
    drop(metered);
    drop(buffer_reservation);
    Ok(response)
}
