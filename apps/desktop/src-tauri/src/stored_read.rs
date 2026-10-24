//! Memory admission for reads of stored rows (W-12; DEL-14-01 Design
//! `desktop-states-runs-comparison.md` §2, §3 and §4.2).
//!
//! The small history commands (`read_model_state`, `read_analysis_run`, the two
//! lists, the label edits and deletes) are admitted with a few KiB of argument
//! ingress, but what they then read from the store (a snapshot, a received
//! result, an input manifest, a run record) can be MiB. A read therefore prices
//! its work from the stored columns' byte lengths, read with `octet_length` and
//! without loading the values, and takes interactive memory-pool leases
//! ([`crate::memory_pool::MemoryPool::acquire_interactive`], the admission Save
//! Local Project's ingress lease uses: the per-invocation budget, the pool and
//! fresh headroom) before it loads or parses anything:
//!
//! 1. **Before loading** ([`loaded_text_bytes`], [`text_copy_bytes`],
//!    [`response_bytes`]): each column it loads (SQLite's page cache, the
//!    statement's value and the `String` it is copied into), each text copy the
//!    read makes (a canonical or serialized form for a hash), and the response
//!    serialized while the command runs. A refusal loads nothing:
//!    `STORE-READ-MEMORY-UNAVAILABLE`, naming what was to be read.
//! 2. **Before parsing**: each loaded text's `serde_json::Value` materialization,
//!    bounded by the allocation-free ingress prescan ([`parse_bytes`]) times the
//!    number of trees the read holds at once. A refusal drops the loaded text
//!    and parses nothing.
//!
//! Every lease is released when the read returns. S3's comparison of two stored
//! results (design §4.2) reuses the same pricing. The counting-allocator tests
//! in `analysis_runs_tests.rs` and `model_states.rs` check a read's priced peak
//! against what it reserved.

use crate::ingress::{allocation_envelope, prescan};
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

/// The response, serialized while the command runs: the texts it carries, each
/// at most doubled by JSON string escaping, plus slack for its small members,
/// in a growing buffer.
pub(crate) fn response_bytes(lengths: u64) -> u64 {
    text_copy_bytes(lengths.saturating_mul(2).saturating_add(64 * 1024))
}

/// The `serde_json::Value` materialization of a loaded text, bounded by the
/// ingress prescan (which allocates nothing). A text that is not JSON is priced
/// at nothing: its parse fails at the first error and the read refuses it.
pub(crate) fn parse_bytes(text: &str) -> u64 {
    prescan(text.as_bytes()).map_or(0, |bound| bound.materialization_bytes)
}

/// Sums byte counts, saturating.
pub(crate) fn total(parts: &[u64]) -> u64 {
    parts.iter().fold(0u64, |sum, part| sum.saturating_add(*part))
}
