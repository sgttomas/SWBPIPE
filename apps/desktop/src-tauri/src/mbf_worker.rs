//! File > Export MBF… in the capped worker (DEL-17-04 Design `desktop-mbf-export-import.md` §3.7 and §3.9.5, slice
//! 4b.2a): [`CappedWorkerRun`], the only production [`MbfWriterRun`], and the blocking parts of the native commands
//! `review_mbf_export` and `save_mbf_export` (`lib.rs` registers them).
//!
//! A command (C1's form, as `native_persistence`'s): the raw `args-json-v1` body is copied under a capped interactive
//! lease ([`reserve`]); on the blocking executor that lease seeds the command's ledger (`memory_pool::CommandLedger`,
//! through `stored_read::NativeLeaseMemory`; DEL-14-01 r4: its live leases together within min(the budget that lease
//! was admitted under, 12 GiB)), and the arguments' bound grows it before they are parsed
//! (`command_ingress::admit_raw`). The pipeline of `mbf_export` then runs within that ledger.
//!
//! The run of task 3 (§3.9.5), in order:
//!
//! 1. **Bound.** The worker's maximum grant is the rest of the command's budget after its live leases, the parent's
//!    copy of the request and the largest result lease (the 64 MiB output ceiling's envelope). Those three are
//!    reserved in the ledger as one [`LedgerBound`] before any of them is admitted, and refused by name above the
//!    budget (`ingress_exceeds_budget`), as is a starting grant above that maximum. The grant is admitted under that
//!    maximum, so neither it nor its growth can pass it; the result lease is no larger than the bound's share for it.
//! 2. **Queue wait.** The starting grant (the process reserve plus the request's resident envelope, whole MiB) is a
//!    worker lease taken by the solve's own queue wait (`MemoryPool::wait_acquire`: the worker count, the pool and a
//!    fresh headroom verdict at each attempt) for at most [`MBF_START_WAIT`], after which the last queue reason
//!    stands as the refusal. Cancellation or a newer review ends the wait.
//! 3. **The request copy**, admitted (a capped interactive lease) before it is made, handed to the supervisor, which
//!    drops it once sent.
//! 4. **Supervision** (`solve_worker::supervise`, unchanged) of task 3 ([`WorkerTask::MbfRender`]) with this run's
//!    hooks: cancellation, and `result_limit` at the 64 MiB output ceiling, so a longer announced result ends the run
//!    `MBF-EXPORT-OUTPUT-TOO-LARGE` before any result lease is admitted or any of its bytes is read. The worker is
//!    reaped inside `supervise`; only then is the grant released.
//! 5. **Result custody.** A completed result's lease stays counted, with the bound shrunk to it, until the command's
//!    response is built ([`WorkerResultHold`]). The envelope is then read by `mbf_export::decode_worker_result`.
//!
//! Failures are named (`run_failure` in `mbf_export`): a refused grant, growth, footprint or request copy is a memory
//! refusal (`MBF-EXPORT-MEMORY-UNAVAILABLE`, or the review's `MBF-REVIEW-MEMORY-UNAVAILABLE`); task 3's own failure
//! keeps its code; any other worker failure (an exit, a panic, a protocol violation, an attested result) is
//! `MBF-EXPORT-WORKER-PROTOCOL`; cancellation is `MBF-EXPORT-CANCELLED`.
//!
//! **Cancellation** ([`MbfRuns`]). A newer review or save cancels the older ones of its kind, whose wait ends or whose
//! worker is killed and reaped before its grant is released. A *retirement* cancels every run in progress and moves
//! the retirement epoch on. Production retires only through `cancel_mbf_export` ([`cancel_command`], a keyed
//! retirement, [`MbfRuns::retire_keyed`]): the frontend sends one when its page's workspace mounts (page load
//! included, so an old page's queued or running run is retired then), when the project session is retired, and
//! (4b.2b) when the dialog closes. The performance session reset (`dispose_all_preview_mechanics_results`) does not
//! touch the MBF export.
//!
//! **Idempotent retirement.** Tauri's IPC may deliver one invoke twice: its transport script resends a command
//! through `postMessage` when its `fetch` fails, and does not cancel or deduplicate the original delivery. So every
//! logical retirement carries its own id ([`RetirementId`], `{page, sequence}`: a random token made once per page
//! load and a counter that only rises in that page; a re-delivery of the same logical retirement reuses its id).
//! The host keeps, for every page token it has applied, the last sequence it applied, and applies a retirement only
//! when its sequence is above that one. A duplicate or older delivery is a no-op: it cancels nothing, moves nothing
//! on, and its reply says so (`duplicate: true`, with the current epoch). An applied token is never forgotten: the
//! host keeps at most [`RETIREMENT_PAGES_MAX`] of them, and a new token beyond them makes the export exhausted
//! (below) instead of evicting one, whose delayed delivery would otherwise be applied again. A missing or malformed id
//! is refused `MBF-EXPORT-RETIREMENT-ID-INVALID` and applies nothing. The frontend sends the id as Tauri's own JSON
//! arguments, the one form that survives the transport's `postMessage` fallback; the host also admits it as a raw
//! body, and the two forms share this one keyed path ([`cancel_command`]).
//!
//! **The dispatch ticket.** A run belongs to the retirement epoch its request carries, not to the moment its task
//! happens to run (Tauri 2 extracts an async command's arguments inside the task it spawns, so a request sent before
//! a retirement may begin after it). `cancel_mbf_export` replies the epoch after its bump ([`MbfRetirement`]); the
//! frontend's barrier (`mbfExportCancel.ts`) records the latest acknowledged one and every review and save request
//! carries it as `expectedEpoch` ([`EXPECTED_EPOCH`]), captured once per logical start and kept through every
//! re-send. The command reads that ticket from its admitted arguments before it decodes the model
//! ([`MbfRuns::enter`]): a ticket below the current epoch was issued before a retirement and refuses
//! `MBF-EXPORT-CANCELLED`; a missing, malformed or future ticket refuses `MBF-EXPORT-EPOCH-INVALID`. The ticket is
//! compared again when the run registers (a retirement in between cancels it from the start), and every registered
//! run stays tracked until its command ends, so a later retirement reaches it wherever it is. A refused ticket starts
//! no worker, opens no chooser and writes nothing. A new page's keyed mount retirement (its workspace's first
//! `cancel_mbf_export`, under its own page token) moves the epoch on as any applied retirement does, so an older page,
//! which has not acknowledged that retirement, cannot start anything. The ticket only binds a request to a retirement:
//! it authorizes nothing, and every other check of the review and the save applies unchanged.
//!
//! **Exhaustion.** The export enters a terminal, fail-closed state in either of two cases, and only a restart of the
//! process leaves it:
//!
//! - *Epoch overflow*: a retirement finds the epoch at `u64::MAX`, which it cannot move on without wrapping (each
//!   applied retirement moves it by one, so this takes 2^64 of them; saturating alone would leave the tickets at
//!   `u64::MAX` current). The epoch stays at `u64::MAX`.
//! - *Token capacity*: a retirement comes from a new page token once [`RETIREMENT_PAGES_MAX`] (64) are kept. The
//!   epoch stays where it was (it is not moved on, and is far below `u64::MAX`). Each page load makes one token, so in
//!   practice the 65th distinct page load in one process (a webview reload counts) exhausts the MBF export until the
//!   app restarts: an availability limit, chosen over forgetting a token, whose delayed duplicate would otherwise be
//!   applied again.
//!
//! In both, the retirement that exhausts it cancels every run in progress as any retirement does, its reply says so
//! (`exhausted: true`, with the epoch as it is), and from then on every request is refused
//! `MBF-EXPORT-EPOCH-EXHAUSTED` at its entry and a command that had already entered is cancelled when it registers.
//!
//! **Ids are not authenticated.** A page token is random but not a secret, and the host does not check who sends it:
//! any code running in the webview can send `cancel_mbf_export` with any id. Such code could cancel MBF runs, or use
//! up the 64 tokens and exhaust the export until restart (a denial of service), but it gains no save authority: a
//! retirement only cancels and refuses, and a token or ticket authorizes nothing. This is not a security boundary;
//! the webview's code is trusted as for every other command.
//!
//! **The cancel/commit boundary** (a save only). The save checks its cancellation again before it opens the chooser,
//! and once the chooser returns a path it commits ([`MbfWriterRun::commit`]) under the run's lock before the first
//! write. A cancel that wins the lock first refuses the save `MBF-EXPORT-CANCELLED` with nothing written; a cancel
//! after the commit is too late (its receipt says so), and the save's transactional write (4a: every destination
//! checked and backed up first, undone on failure) completes or undoes itself and reports its own outcome. A
//! committed save stays tracked until it completes, superseded or not: every cancel meanwhile is reported too late.

use crate::command_ingress::{self, Args, ArgsDecoding, CommandError, NoCommandError};
use crate::ingress::allocation_envelope;
use crate::mbf_export::{
    self, MbfExportOutcome, MbfReviewOutcome, MbfWriterRun, WorkerRequest, WriterOutput, WriterRunFailure, ENGINE_FAILED, MODEL_JSON_INVALID,
    OUTPUT_TOO_LARGE, SETTINGS_INVALID, WORKER_PROTOCOL, WORKER_REQUEST_INVALID,
};
use serde::Serialize;
use crate::memory_pool::{CommandLease, CommandLedger, Lease, LeasePurpose, LedgerBound, MemoryPool, ResourceRefusal, WaitOutcome};
use crate::solve_jobs::{SolveJobError, SolveJobRegistry};
use crate::solve_worker::{initial_grant, panic_message, supervise, SupervisorHooks, WorkerControl, WorkerOutcome, WorkerTask, WorkerTerms};
use crate::stored_read::NativeLeaseMemory;
use crate::worker_alloc::resident_envelope;
use open_pipe_stress_caepipe_mbf as mbf;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};
use tauri::http::HeaderMap;
use tauri::ipc::InvokeBody;

/// How long task 3 waits for its worker's starting grant: the model checks' window (§3.9.5, "the 2 s wait for a
/// slot").
pub(crate) const MBF_START_WAIT: Duration = crate::solve_jobs::MODEL_CHECKS_START_WAIT;

/// The largest result lease the supervisor admits for task 3: the resident envelope of a result at the output ceiling
/// (`result_limit`), which bounds every announced length it admits.
pub(crate) const RESULT_LEASE_BYTES: u64 = resident_envelope(mbf::MBF_EXPORT_OUTPUT_MAX_BYTES);

/// Task 3's own failure codes (`worker_render`'s `Err`): a failure under any other name is not task 3's.
const TASK_FAILURE_CODES: [&str; 6] = [WORKER_REQUEST_INVALID, SETTINGS_INVALID, MODEL_JSON_INVALID, ENGINE_FAILED, WORKER_PROTOCOL, OUTPUT_TOO_LARGE];

// -- Cancellation ---------------------------------------------------------------------------------------------------

/// One run's cancellation: a flag the wait, the supervisor and the save read, kill access to its worker while unreaped,
/// and the save's commit, after which a cancel no longer applies.
#[derive(Debug, Default)]
struct RunControl {
    state: Mutex<RunState>,
}

#[derive(Debug, Default)]
struct RunState {
    cancelled: bool,
    committed: bool,
    worker: Option<Arc<WorkerControl>>,
}

/// What one cancel did to one run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CancelEffect {
    /// This cancel stopped it: it replies `MBF-EXPORT-CANCELLED` and writes nothing.
    Stopped,
    /// An earlier cancel had stopped it already; it is still ending.
    AlreadyStopped,
    /// A save past its commit: its write is not interrupted.
    TooLate,
}

impl RunControl {
    /// A run whose command entered before a retirement it registers after: cancelled from the start.
    fn retired() -> Self {
        RunControl { state: Mutex::new(RunState { cancelled: true, ..RunState::default() }) }
    }

    fn lock(&self) -> MutexGuard<'_, RunState> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn cancelled(&self) -> bool {
        self.lock().cancelled
    }

    /// Requests cancellation and kills the worker if one is attached; its supervisor reaps it, then the run releases
    /// the grant. A save that has already committed its write is too late, and nothing changes.
    fn cancel(&self) -> CancelEffect {
        let mut state = self.lock();
        if state.committed {
            return CancelEffect::TooLate;
        }
        if state.cancelled {
            return CancelEffect::AlreadyStopped;
        }
        state.cancelled = true;
        if let Some(worker) = &state.worker {
            worker.kill();
        }
        CancelEffect::Stopped
    }

    /// The cancel/commit boundary: commits unless a cancel came first (under the same lock as [`Self::cancel`]).
    fn commit(&self) -> bool {
        let mut state = self.lock();
        if !state.cancelled {
            state.committed = true;
        }
        state.committed
    }
}

/// What a retirement (`cancel_mbf_export`, the session reset) did: the runs it stopped before they wrote anything, and
/// the saves it reached after they had committed their write, which complete (or undo themselves) and report their
/// own outcome. A run an earlier cancel already stopped is in neither. Both zero: nothing it could stop was in
/// progress.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub(crate) struct MbfCancelReceipt {
    /// Stopped: each replies `MBF-EXPORT-CANCELLED`, and nothing was written.
    pub(crate) cancelled: u32,
    /// Too late: a save past its commit, whose write was not interrupted.
    pub(crate) too_late: u32,
}

/// `cancel_mbf_export`'s reply: what the retirement did, and the retirement epoch after it, which every later start's
/// request carries as its ticket ([`EXPECTED_EPOCH`]).
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct MbfRetirement {
    #[serde(flatten)]
    pub(crate) receipt: MbfCancelReceipt,
    pub(crate) epoch: u64,
    /// The epoch could move on no further: the export is closed for good (`MBF-EXPORT-EPOCH-EXHAUSTED`). Only
    /// present in the reply when true.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub(crate) exhausted: bool,
    /// The keyed retirement this replies to, echoed (`cancel_mbf_export`'s reply always has it).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) retirement_id: Option<RetirementId>,
    /// This id, or a later one of its page, had been applied already: nothing was cancelled or moved on, and `epoch`
    /// is the current one. Only present in the reply when true.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub(crate) duplicate: bool,
}

/// `cancel_mbf_export`'s command name.
pub(crate) const CANCEL_COMMAND: &str = "cancel_mbf_export";

/// The request member that carries a retirement's id.
pub(crate) const RETIREMENT_ID: &str = "retirementId";

/// The most page tokens the host keeps (one per page load in this process): a retirement from a new token beyond them
/// makes the export exhausted, since no applied token may be forgotten.
pub(crate) const RETIREMENT_PAGES_MAX: usize = 64;

/// A page token's length: 16 random bytes as lowercase hex.
const PAGE_TOKEN_HEX_LEN: usize = 32;

/// The largest sequence a page sends (JavaScript's largest safe integer).
const RETIREMENT_SEQUENCE_MAX: u64 = (1 << 53) - 1;

/// The largest `cancel_mbf_export` arguments the host admits.
const CANCEL_ARGS_MAX_BYTES: usize = 4 << 10;

/// One logical retirement's id: its page's token (made once per page load) and its sequence in that page (from 1,
/// rising by one per logical retirement; a re-delivery reuses it).
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct RetirementId {
    pub(crate) page: String,
    pub(crate) sequence: u64,
}

impl RetirementId {
    /// Reads the id from the [`RETIREMENT_ID`] member: an object of exactly `page` (32 lowercase hex digits) and
    /// `sequence` (an integer from 1 to 2^53 - 1). Anything else, or none, refuses `MBF-EXPORT-RETIREMENT-ID-INVALID`.
    pub(crate) fn read(value: Option<serde_json::Value>) -> Result<Self, EntryRefusal> {
        let refused = |what: &str| EntryRefusal {
            code: mbf_export::RETIREMENT_ID_INVALID,
            message: format!("The export was not retired: its request {what}. Nothing was cancelled."),
        };
        let mut map = match value {
            None => return Err(refused("carries no retirement id")),
            Some(serde_json::Value::Object(map)) => map,
            Some(_) => return Err(refused("carries a malformed retirement id")),
        };
        let page = match map.remove("page") {
            Some(serde_json::Value::String(page))
                if page.len() == PAGE_TOKEN_HEX_LEN && page.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) =>
            {
                page
            }
            _ => return Err(refused("carries a malformed retirement id")),
        };
        let sequence = map.remove("sequence").and_then(|value| value.as_u64()).filter(|sequence| (1..=RETIREMENT_SEQUENCE_MAX).contains(sequence));
        match sequence {
            Some(sequence) if map.is_empty() => Ok(RetirementId { page, sequence }),
            _ => Err(refused("carries a malformed retirement id")),
        }
    }
}

/// The request member that carries the dispatch ticket: the retirement epoch the frontend last saw acknowledged.
pub(crate) const EXPECTED_EPOCH: &str = "expectedEpoch";

/// A request refused at its entry by its ticket: no run registers, no worker starts, nothing is chosen or written.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct EntryRefusal {
    pub(crate) code: &'static str,
    pub(crate) message: String,
}

/// The two kinds of run: a newer one of either kind supersedes the older ones of its kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RunKind {
    Review,
    Save,
}

/// A command's entry into the MBF export: the retirement epoch its request's ticket names, accepted as current when
/// the command read it ([`MbfRuns::enter`]). A run whose entry is older than the current epoch when it registers was
/// retired in between, and is cancelled from the start: no worker, no chooser, no write.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct MbfEntry {
    epoch: u64,
}

#[derive(Debug, Default)]
struct Tracked {
    /// Bumped by every retirement ([`MbfRuns::cancel_all`]), never past `u64::MAX`.
    epoch: u64,
    /// A retirement found the epoch at `u64::MAX`, or came from a new page token with [`RETIREMENT_PAGES_MAX`] kept:
    /// terminal, every start is refused (`MBF-EXPORT-EPOCH-EXHAUSTED`).
    exhausted: bool,
    /// Every page token a keyed retirement was applied from, with the last sequence applied from it. Never shortened.
    applied: Vec<(String, u64)>,
    /// Every registered run until it ends (its [`Registration`] drops): cancelled and committed ones included, so a
    /// committed save stays tracked, and is reported too late, until its write completes.
    runs: Vec<(RunKind, Arc<RunControl>)>,
}

/// The MBF export's runs in progress (managed app state) and its retirement epoch. A newer review or save cancels the
/// older ones of its kind (a committed save is too late and completes); a retirement cancels all of them and every
/// command that entered before it.
#[derive(Clone, Debug, Default)]
pub(crate) struct MbfRuns {
    tracked: Arc<Mutex<Tracked>>,
}

// A test's hook, run on the command's thread just before its run registers (the retirement-before-registration test).
#[cfg(test)]
thread_local! {
    pub(crate) static BEFORE_REGISTRATION: std::cell::RefCell<Option<Box<dyn FnOnce()>>> = std::cell::RefCell::new(None);
}

impl MbfRuns {
    fn lock(&self) -> MutexGuard<'_, Tracked> {
        self.tracked.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// The command's entry, from its request's ticket (the [`EXPECTED_EPOCH`] member of its admitted arguments, read
    /// before the model is decoded). The ticket must be a non-negative integer no greater than the current epoch: a
    /// missing, malformed or future one is a protocol error (`MBF-EXPORT-EPOCH-INVALID`), and one below it was issued
    /// before a retirement since, and refuses `MBF-EXPORT-CANCELLED`. Once the epoch is exhausted every request is
    /// refused `MBF-EXPORT-EPOCH-EXHAUSTED`, whatever its ticket.
    pub(crate) fn enter(&self, ticket: Option<serde_json::Value>) -> Result<MbfEntry, EntryRefusal> {
        let protocol = |what: &str| EntryRefusal {
            code: mbf_export::EPOCH_INVALID,
            message: format!("The export did not start: its request {what}. Nothing was written."),
        };
        let expected = match ticket {
            None => return Err(protocol("carries no retirement epoch")),
            Some(value) => value.as_u64().ok_or_else(|| protocol("carries a malformed retirement epoch"))?,
        };
        let (current, exhausted) = {
            let tracked = self.lock();
            (tracked.epoch, tracked.exhausted)
        };
        if exhausted {
            return Err(EntryRefusal {
                code: mbf_export::EPOCH_EXHAUSTED,
                message: "The export did not start: the export cannot be retired any further in this session. Nothing was written.".to_string(),
            });
        }
        if expected > current {
            return Err(protocol("names a retirement epoch the host has not reached"));
        }
        if expected < current {
            return Err(EntryRefusal { code: mbf_export::CANCELLED, message: mbf_export::CANCELLED_MESSAGE.to_string() });
        }
        Ok(MbfEntry { epoch: expected })
    }

    /// The current retirement epoch (tests; production learns it from `cancel_mbf_export`'s reply).
    #[cfg(test)]
    pub(crate) fn epoch(&self) -> u64 {
        self.lock().epoch
    }

    /// Moves the epoch to `epoch` (the exhaustion boundary's test).
    #[cfg(test)]
    pub(crate) fn seed_epoch_for_tests(&self, epoch: u64) {
        self.lock().epoch = epoch;
    }

    /// An entry at the current epoch (tests that register a run directly).
    #[cfg(test)]
    pub(crate) fn entry_now(&self) -> MbfEntry {
        MbfEntry { epoch: self.epoch() }
    }

    /// Registers a new run of `kind`, cancelling the older ones of its kind and waking the pool's waiters so a
    /// waiting run sees it at once. A run whose command entered before the latest retirement, or that registers once
    /// the epoch is exhausted, is not registered: it is cancelled from the start, and supersedes nothing.
    fn register(&self, kind: RunKind, entry: MbfEntry, pool: &MemoryPool) -> Registration {
        #[cfg(test)]
        if let Some(hook) = BEFORE_REGISTRATION.with(|hook| hook.borrow_mut().take()) {
            hook();
        }
        let mut tracked = self.lock();
        if entry.epoch < tracked.epoch || tracked.exhausted {
            return Registration { runs: self.clone(), control: Arc::new(RunControl::retired()) };
        }
        let control = Arc::new(RunControl::default());
        let stopped = tracked.runs.iter().filter(|(of, _)| *of == kind).filter(|(_, run)| run.cancel() == CancelEffect::Stopped).count();
        tracked.runs.push((kind, Arc::clone(&control)));
        drop(tracked);
        if stopped > 0 {
            pool.wake_waiters();
        }
        Registration { runs: self.clone(), control }
    }

    /// An unkeyed retirement's receipt (tests: production retires only by [`Self::retire_keyed`]).
    #[cfg(test)]
    pub(crate) fn cancel_all(&self, pool: &MemoryPool) -> MbfCancelReceipt {
        self.retire(pool).receipt
    }

    /// An unkeyed retirement ([`Self::retire_locked`]; tests).
    #[cfg(test)]
    pub(crate) fn retire(&self, pool: &MemoryPool) -> MbfRetirement {
        Self::retire_locked(self.lock(), pool)
    }

    /// `cancel_mbf_export`'s retirement, applied once per id (module docs, "Idempotent retirement"). A sequence no
    /// greater than the last applied from its page is a duplicate: no run is cancelled, the epoch does not move, and the
    /// reply says `duplicate` with the current epoch (and `exhausted` when so). Otherwise the sequence is recorded and
    /// the retirement applied ([`Self::retire_locked`]); a new page token with [`RETIREMENT_PAGES_MAX`] kept is not
    /// recorded, and makes the export exhausted (terminal) as it is applied. The id is echoed in the reply.
    pub(crate) fn retire_keyed(&self, id: RetirementId, pool: &MemoryPool) -> MbfRetirement {
        let mut tracked = self.lock();
        match tracked.applied.iter().position(|(page, _)| *page == id.page) {
            Some(known) if id.sequence <= tracked.applied[known].1 => {
                let (epoch, exhausted) = (tracked.epoch, tracked.exhausted);
                return MbfRetirement { receipt: MbfCancelReceipt::default(), epoch, exhausted, retirement_id: Some(id), duplicate: true };
            }
            Some(known) => tracked.applied[known].1 = id.sequence,
            None if tracked.applied.len() < RETIREMENT_PAGES_MAX => tracked.applied.push((id.page.clone(), id.sequence)),
            None => tracked.exhausted = true,
        }
        MbfRetirement { retirement_id: Some(id), ..Self::retire_locked(tracked, pool) }
    }

    /// Retires every run in progress and every command that has entered but not yet registered (the epoch moves on):
    /// a waiting run stops waiting, a running worker is killed and its grant released after the reap, a save before
    /// its commit refuses with nothing written. A save past its commit is counted too late, stays tracked and
    /// completes. A request whose ticket is the epoch after it is not affected. The reply names the epoch after it,
    /// read under the same lock. At `u64::MAX` the epoch does not wrap: the export becomes exhausted (terminal), and
    /// once exhausted the epoch moves no more; the reply says so.
    fn retire_locked(mut tracked: MutexGuard<'_, Tracked>, pool: &MemoryPool) -> MbfRetirement {
        if !tracked.exhausted {
            match tracked.epoch.checked_add(1) {
                Some(next) => tracked.epoch = next,
                None => tracked.exhausted = true,
            }
        }
        let (epoch, exhausted) = (tracked.epoch, tracked.exhausted);
        let mut receipt = MbfCancelReceipt::default();
        for (_, run) in &tracked.runs {
            match run.cancel() {
                CancelEffect::Stopped => receipt.cancelled += 1,
                CancelEffect::TooLate => receipt.too_late += 1,
                CancelEffect::AlreadyStopped => {}
            }
        }
        drop(tracked);
        if receipt.cancelled > 0 {
            pool.wake_waiters();
        }
        MbfRetirement { receipt, epoch, exhausted, retirement_id: None, duplicate: false }
    }
}

/// A run's place among the tracked runs: dropping it (as the run's command ends) stops tracking it.
struct Registration {
    runs: MbfRuns,
    control: Arc<RunControl>,
}

impl Drop for Registration {
    fn drop(&mut self) {
        self.runs.lock().runs.retain(|(_, run)| !Arc::ptr_eq(run, &self.control));
    }
}

// -- The run --------------------------------------------------------------------------------------------------------

/// A completed result's memory until the command's response is built: the worker's result lease and the command's
/// ledger bound, shrunk to it. The lease is released first, then the ledger's charge.
#[derive(Debug)]
pub(crate) struct WorkerResultHold {
    _lease: Lease,
    _bound: LedgerBound,
}

impl WorkerResultHold {
    #[cfg(test)]
    pub(crate) fn lease_bytes(&self) -> u64 {
        self._lease.bytes()
    }
}

/// The supervisor's view of a run: its cancellation and the output ceiling.
struct MbfHooks<'c> {
    control: &'c RunControl,
}

impl SupervisorHooks for MbfHooks<'_> {
    fn attach(&mut self, worker: Arc<WorkerControl>) -> bool {
        let mut state = self.control.lock();
        state.worker = Some(worker);
        !state.cancelled
    }

    fn cancelled(&self) -> bool {
        self.control.cancelled()
    }

    fn grant_changed(&mut self, _grant_bytes: u64) {}

    fn footprint_observed(&mut self, _footprint_bytes: u64) {}

    /// The 64 MiB output ceiling (§3.7), checked on the announced length before any result lease or read.
    fn result_limit(&self) -> Option<u64> {
        Some(mbf::MBF_EXPORT_OUTPUT_MAX_BYTES)
    }
}

/// Task 3 in the capped worker process (see the module docs), within the command's ledger.
pub(crate) struct CappedWorkerRun<'r> {
    registry: &'r SolveJobRegistry,
    ledger: Arc<CommandLedger>,
    registration: Registration,
    start_wait: Duration,
}

impl<'r> CappedWorkerRun<'r> {
    /// The review's run, for a command that entered at `entry`: it becomes `runs`' review in progress, superseding
    /// older ones, unless a retirement came after `entry`.
    pub(crate) fn review(registry: &'r SolveJobRegistry, ledger: &Arc<CommandLedger>, runs: &MbfRuns, entry: MbfEntry) -> Self {
        Self::new(registry, ledger, runs.register(RunKind::Review, entry, registry.pool()))
    }

    /// The save's run, for a command that entered at `entry`: it becomes `runs`' save in progress, as the review's.
    pub(crate) fn save(registry: &'r SolveJobRegistry, ledger: &Arc<CommandLedger>, runs: &MbfRuns, entry: MbfEntry) -> Self {
        Self::new(registry, ledger, runs.register(RunKind::Save, entry, registry.pool()))
    }

    fn new(registry: &'r SolveJobRegistry, ledger: &Arc<CommandLedger>, registration: Registration) -> Self {
        CappedWorkerRun { registry, ledger: Arc::clone(ledger), registration, start_wait: MBF_START_WAIT }
    }

    /// The same run with a shorter start wait (tests).
    #[cfg(test)]
    pub(crate) fn with_start_wait(mut self, start_wait: Duration) -> Self {
        self.start_wait = start_wait;
        self
    }

    /// A typed refusal (recorded as the pool's last) as a memory failure.
    fn refused(&self, refusal: ResourceRefusal) -> WriterRunFailure {
        match self.registry.refuse(refusal) {
            SolveJobError::ResourceUnavailable { refusal, .. } => WriterRunFailure::Memory(format!(
                "The MBF worker's memory was refused ({}).",
                serde_json::to_string(&refusal).unwrap_or_else(|_| "refused".to_string())
            )),
            SolveJobError::InvalidRequest { message } => WriterRunFailure::Memory(message),
        }
    }

    fn over_budget(&self, required_bytes: u64, budget_bytes: u64) -> WriterRunFailure {
        self.refused(ResourceRefusal::IngressExceedsBudget { required_bytes, budget_bytes })
    }
}

/// A worker failure: task 3's own by its code, anything else (an exit, a panic, a protocol violation) as a protocol
/// failure.
fn worker_failure(text: String) -> WriterRunFailure {
    match text.split_once(": ") {
        Some((code, _)) if TASK_FAILURE_CODES.contains(&code) => WriterRunFailure::Failed(text),
        _ => WriterRunFailure::Protocol(text),
    }
}

// A test's hook, run on the saving thread just after a save commits (the cancel-after-commit test).
#[cfg(test)]
thread_local! {
    pub(crate) static AFTER_COMMIT: std::cell::RefCell<Option<Box<dyn FnOnce()>>> = std::cell::RefCell::new(None);
}

impl MbfWriterRun for CappedWorkerRun<'_> {
    fn cancelled(&self) -> bool {
        self.registration.control.cancelled()
    }

    fn commit(&mut self) -> Result<(), WriterRunFailure> {
        if !self.registration.control.commit() {
            return Err(WriterRunFailure::Cancelled);
        }
        #[cfg(test)]
        if let Some(hook) = AFTER_COMMIT.with(|hook| hook.borrow_mut().take()) {
            hook();
        }
        Ok(())
    }

    fn run(&mut self, request: &WorkerRequest<'_>) -> Result<WriterOutput, WriterRunFailure> {
        let registry = self.registry;
        let control = Arc::clone(&self.registration.control);
        if control.cancelled() {
            return Err(WriterRunFailure::Cancelled);
        }
        // 1. The bound: the worker's maximum grant is what the command's budget leaves after its live leases, the
        // request copy and the largest result lease; all three are reserved in the ledger before any is admitted.
        let raw_len = request.encoded_len();
        let reserve = registry.process_reserve();
        let request_bytes = allocation_envelope(raw_len).unwrap_or(u64::MAX);
        let start = initial_grant(reserve, resident_envelope(raw_len)).unwrap_or(u64::MAX);
        let budget = self.ledger.budget_bytes();
        let fixed = self.ledger.total_bytes().saturating_add(request_bytes).saturating_add(RESULT_LEASE_BYTES);
        let max_grant = budget.saturating_sub(fixed);
        if start > max_grant {
            return Err(self.over_budget(fixed.saturating_add(start), budget));
        }
        let mut bound = self
            .ledger
            .bound(request_bytes + max_grant + RESULT_LEASE_BYTES)
            .map_err(|over| self.over_budget(over.required_bytes, over.budget_bytes))?;
        // 2. The queue wait for the starting grant, bounded, ended by cancellation or a newer run.
        let deadline = Instant::now() + self.start_wait;
        let mut queued: Option<ResourceRefusal> = None;
        let waited = registry.pool().wait_acquire(
            start,
            LeasePurpose::Worker,
            Some(max_grant),
            &|| control.cancelled() || Instant::now() >= deadline,
            &mut |reason| queued = Some(reason.clone()),
        );
        let mut grant = match waited {
            Ok(grant) => grant,
            Err(WaitOutcome::Cancelled) if control.cancelled() => return Err(WriterRunFailure::Cancelled),
            // The first attempt always precedes the deadline, so a reason was recorded.
            Err(WaitOutcome::Cancelled) => return Err(self.refused(queued.unwrap_or(ResourceRefusal::HeadroomUnknown))),
        };
        // 3. The request copy, admitted before it is made.
        let copy = match registry.acquire_interactive_ingress_capped(request_bytes) {
            Ok(copy) => copy,
            Err(SolveJobError::ResourceUnavailable { refusal, .. }) => return Err(self.refused(refusal)),
            Err(SolveJobError::InvalidRequest { message }) => return Err(WriterRunFailure::Memory(message)),
        };
        let raw = request.encode();
        // 4. Supervision; the worker is reaped inside `supervise` (also while it unwinds), then the grant released.
        let terms = WorkerTerms { pool: registry.pool(), launcher: registry.worker_launcher(), task: WorkerTask::MbfRender, max_grant, reserve };
        let mut hooks = MbfHooks { control: &control };
        let outcome = catch_unwind(AssertUnwindSafe(|| supervise(&terms, &mut grant, (raw, copy), &mut hooks)));
        control.lock().worker = None;
        drop(grant);
        if control.cancelled() {
            // A result that arrived as the run was cancelled answers a retired request; it and its lease end here.
            drop(outcome);
            return Err(WriterRunFailure::Cancelled);
        }
        match outcome {
            Err(payload) => Err(WriterRunFailure::Protocol(format!("the MBF worker's supervisor panicked: {}", panic_message(payload.as_ref())))),
            Ok(WorkerOutcome::Completed { attestation: Some(_), .. }) => Err(WriterRunFailure::Protocol("task 3 attests nothing, but its result was attested".to_string())),
            Ok(WorkerOutcome::Completed { result, lease, attestation: None }) => {
                // 5. Result custody: the grant and the request copy have ended; the bound now covers the result lease.
                // A lease above the bound's share would leave the ledger under-counting it: refused, in every build.
                if lease.bytes() > RESULT_LEASE_BYTES {
                    return Err(WriterRunFailure::Protocol(format!(
                        "the result lease of {} bytes exceeds the {RESULT_LEASE_BYTES} bytes its bound holds",
                        lease.bytes()
                    )));
                }
                bound.shrink_to(lease.bytes());
                let text = String::from_utf8(result).map_err(|_| WriterRunFailure::Protocol("the result is not UTF-8".to_string()))?;
                Ok(WriterOutput { text, custody: Some(WorkerResultHold { _lease: lease, _bound: bound }) })
            }
            Ok(WorkerOutcome::Failed(text)) => Err(worker_failure(text)),
            Ok(WorkerOutcome::Refused(refusal)) => Err(self.refused(refusal)),
            Ok(WorkerOutcome::ResultTooLarge { .. }) => Err(WriterRunFailure::OutputTooLarge),
            Ok(WorkerOutcome::Terminated) => Err(WriterRunFailure::Protocol("the MBF worker was stopped".to_string())),
        }
    }
}

// -- The commands' blocking parts -----------------------------------------------------------------------------------

/// The executor's part of either command (C1's): the header check, the raw body's copy reserved as a capped
/// interactive lease before it is made, and the copy.
pub(crate) fn reserve(registry: &SolveJobRegistry, body: &InvokeBody, headers: &HeaderMap, command: &'static str) -> Result<(Vec<u8>, Lease), SolveJobError> {
    let raw = command_ingress::raw_args_body(body, headers, command)?;
    let lease = registry.acquire_interactive_ingress_capped(allocation_envelope(raw.len() as u64).unwrap_or(u64::MAX))?;
    Ok((raw.to_vec(), lease))
}

// A test's hook, run on the command's thread just before its admission (the retirement-before-admission test).
#[cfg(test)]
thread_local! {
    pub(crate) static BEFORE_ADMISSION: std::cell::RefCell<Option<Box<dyn FnOnce()>>> = std::cell::RefCell::new(None);
}

/// The command's memory and arguments: `copy` seeds its ledger, and the arguments' bound grows it before they are
/// parsed (`command_ingress::admit_raw`); the raw copy ends here.
fn admit<'r>(registry: &'r SolveJobRegistry, raw: Vec<u8>, copy: Lease, command: &'static str) -> Result<(NativeLeaseMemory<'r>, Args, CommandLease), SolveJobError> {
    #[cfg(test)]
    if let Some(hook) = BEFORE_ADMISSION.with(|hook| hook.borrow_mut().take()) {
        hook();
    }
    let (memory, copy) = NativeLeaseMemory::seeded(registry, copy);
    let copy_bytes = copy.bytes();
    let (args, lease) = command_ingress::admit_raw(registry, &raw, command, ArgsDecoding::Moved, |bytes| {
        let mut lease = copy;
        memory.grow(&mut lease, copy_bytes.saturating_add(bytes))?;
        Ok(lease)
    })?;
    drop(raw);
    Ok((memory, args, lease))
}

/// `review_mbf_export` after [`reserve`]: admission, the request's ticket ([`MbfRuns::enter`], before the model is
/// decoded), the review in the capped worker, and `respond` with the outcome and its text while the command's leases
/// and the worker's result lease are held.
pub(crate) fn review_command<R>(
    registry: &SolveJobRegistry,
    runs: &MbfRuns,
    raw: Vec<u8>,
    copy: Lease,
    respond: impl FnOnce(&MbfReviewOutcome, String) -> R,
) -> Result<R, SolveJobError> {
    let (memory, mut args, lease) = admit(registry, raw, copy, mbf_export::REVIEW_COMMAND)?;
    let entry = match runs.enter(args.opt_value(EXPECTED_EPOCH)) {
        Ok(entry) => entry,
        Err(refusal) => {
            drop(args);
            let response = mbf_export::review_mbf_export_refused(&memory, refusal.code, &refusal.message, respond);
            drop(lease);
            return Ok(response);
        }
    };
    let request = mbf_export::review_mbf_export_args(&mut args)?;
    drop(args);
    let response = mbf_export::review_mbf_export_admitted(request, &memory, &lease, runs, entry, respond);
    drop(lease);
    Ok(response)
}

/// `save_mbf_export` after [`reserve`]: admission, the request's ticket as the review's, the save (§3.6) with task 3
/// in the capped worker, the chooser `choose` and the store `open_store`, and `respond` as for the review.
pub(crate) fn save_command<R>(
    registry: &SolveJobRegistry,
    runs: &MbfRuns,
    raw: Vec<u8>,
    copy: Lease,
    open_store: impl FnMut() -> Result<rusqlite::Connection, String>,
    choose: impl FnOnce(&str) -> Result<Option<PathBuf>, String>,
    respond: impl FnOnce(&MbfExportOutcome, String) -> R,
) -> Result<R, SolveJobError> {
    let (memory, mut args, lease) = admit(registry, raw, copy, mbf_export::COMMAND)?;
    let entry = match runs.enter(args.opt_value(EXPECTED_EPOCH)) {
        Ok(entry) => entry,
        Err(refusal) => {
            drop(args);
            let response = mbf_export::save_mbf_export_refused(&memory, refusal.code, &refusal.message, respond);
            drop(lease);
            return Ok(response);
        }
    };
    let request = mbf_export::save_mbf_export_args(&mut args)?;
    drop(args);
    let response = mbf_export::save_mbf_export_admitted(request, open_store, &memory, &lease, runs, entry, choose, respond);
    drop(lease);
    Ok(response)
}

/// `cancel_mbf_export` (`lib.rs` registers it): `{retirementId}` in either of two forms, then the keyed retirement
/// ([`MbfRuns::retire_keyed`]). A missing or malformed id is the command's error, `MBF-EXPORT-RETIREMENT-ID-INVALID:
/// …`, and retires nothing.
///
/// - **The raw form**: an `args-json-v1` raw body with the ingress header, admitted as every C1 command's arguments
///   (`command_ingress::admit_body_within`): at most [`CANCEL_ARGS_MAX_BYTES`] raw bytes, refused
///   `INGRESS-ARGS-TOO-LARGE` before the prescan and before any lease, then the prescan, an interactive ingress lease
///   and the parse.
/// - **The JSON form** ([`cancel_json_args`]): Tauri's own JSON arguments, `{"retirementId": {page, sequence}}`, with
///   no ingress header. This is what the frontend sends (`mbfExportCancel.ts`), because it is the one form that reaches
///   the host unchanged on both of Tauri 2.11.1's transports: the custom protocol posts it as `application/json`, and
///   its `postMessage` fallback (taken for the rest of the page once one `fetch` fails, and for the failed invoke
///   itself) `JSON.stringify`s it inside its message. A raw body does not survive the fallback: its `Uint8Array` is
///   stringified into a JSON array of numbers, which arrives as a JSON body and is refused
///   `INGRESS-RAW-BODY-REQUIRED`, so a raw cancel could not stop a save after the transport fell back.
///
/// What bounds the JSON form: Tauri parses it into a `serde_json::Value` before the command runs, unpriced, and
/// neither of its transports limits a request's size first (the custom protocol reads the whole body and parses it as
/// JSON; the fallback parses the whole message string), so no check here bounds Tauri's parse or its allocations,
/// exactly as for every JSON-argument command (`cancel_preview_mechanics_job`). The command's own check bounds what
/// follows the parse: the arguments' canonical serialization is measured without allocating a serialized buffer
/// (a rejection allocates only serde_json's small boxed I/O error), and refused
/// `INGRESS-ARGS-TOO-LARGE` above [`CANCEL_ARGS_MAX_BYTES`] before anything is read from them; the id is then copied
/// out of at most that many bytes. Like the JSON-argument commands, the form takes no ingress lease: the `Value` it
/// would price already exists. Starts (`review_mbf_export`, `save_mbf_export`) stay raw-body only.
///
/// Both forms then read the id from the same member by [`RetirementId::read`] and retire by the one keyed path, so the
/// id's validation, its once-only application (a delivery in one form and a re-delivery in the other are one logical
/// retirement), the reply and exhaustion are the same.
pub(crate) fn cancel_command(registry: &SolveJobRegistry, runs: &MbfRuns, body: &InvokeBody, headers: &HeaderMap) -> Result<MbfRetirement, CommandError<String>> {
    let (id, lease) = match body {
        InvokeBody::Json(args) if headers.get(command_ingress::INGRESS_HEADER).is_none() => (RetirementId::read(cancel_json_args(args)?), None),
        _ => {
            let (mut args, lease) = command_ingress::admit_body_within(registry, body, headers, CANCEL_COMMAND, ArgsDecoding::Moved, CANCEL_ARGS_MAX_BYTES)?;
            (RetirementId::read(args.opt_value(RETIREMENT_ID)), Some(lease))
        }
    };
    let reply = id.map(|id| runs.retire_keyed(id, registry.pool())).map_err(|refusal| CommandError::Command(format!("{}: {}", refusal.code, refusal.message)));
    drop(lease);
    reply
}

/// The JSON form's arguments ([`cancel_command`]): one object whose canonical serialization is at most
/// [`CANCEL_ARGS_MAX_BYTES`] (measured by a counting writer that stops at the bound without allocating a serialized
/// buffer; a rejection allocates only serde_json's small boxed I/O error), and its
/// [`RETIREMENT_ID`] member, copied out of it. A larger body is refused `INGRESS-ARGS-TOO-LARGE` and anything but an
/// object `INGRESS-ARGS-NOT-OBJECT`, as the raw form's are; other members are ignored, as the raw form's are.
fn cancel_json_args(args: &serde_json::Value) -> Result<Option<serde_json::Value>, SolveJobError> {
    struct Bounded(usize);
    impl std::io::Write for Bounded {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            self.0 = self.0.saturating_add(bytes.len());
            if self.0 > CANCEL_ARGS_MAX_BYTES {
                return Err(std::io::ErrorKind::FileTooLarge.into());
            }
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    if serde_json::to_writer(&mut Bounded(0), args).is_err() {
        return Err(SolveJobError::InvalidRequest {
            message: format!("INGRESS-ARGS-TOO-LARGE: `{CANCEL_COMMAND}` takes at most {CANCEL_ARGS_MAX_BYTES} bytes of arguments."),
        });
    }
    let serde_json::Value::Object(map) = args else {
        return Err(SolveJobError::InvalidRequest { message: format!("INGRESS-ARGS-NOT-OBJECT: `{CANCEL_COMMAND}`'s arguments must be one JSON object.") });
    };
    Ok(map.get(RETIREMENT_ID).cloned())
}

/// A command whose blocking part did not complete: a fixed refusal of the given code, within the argument lease's
/// error allowance.
pub(crate) fn worker_failed(save: bool) -> tauri::ipc::Response {
    let message = "The MBF export's worker did not complete. Nothing was written.";
    let text = if save {
        serde_json::to_string(&MbfExportOutcome::refused_one(WORKER_PROTOCOL, message))
    } else {
        serde_json::to_string(&MbfReviewOutcome::Refused { diagnostics: vec![crate::export_disposition_screen::diagnostic(WORKER_PROTOCOL, "", message)] })
    };
    tauri::ipc::Response::new(text.unwrap_or_default())
}

/// The commands' ingress error type (the outcome itself is the reply's JSON body).
pub(crate) type MbfCommandError = CommandError<NoCommandError>;

#[cfg(test)]
#[path = "mbf_worker_tests.rs"]
mod tests;
