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
//! **Cancellation** ([`MbfRuns`]). At most one review is in progress: a newer review cancels the older one, whose
//! wait ends or whose worker is killed and reaped before its grant is released. [`MbfRuns::cancel`] does the same for
//! the review and the save in progress; slice 4b.2b connects it to the dialog's close and the project session's
//! change.

use crate::command_ingress::{self, Args, ArgsDecoding, CommandError, NoCommandError};
use crate::ingress::allocation_envelope;
use crate::mbf_export::{
    self, MbfExportOutcome, MbfReviewOutcome, MbfWriterRun, WorkerRequest, WriterOutput, WriterRunFailure, ENGINE_FAILED, MODEL_JSON_INVALID,
    OUTPUT_TOO_LARGE, SETTINGS_INVALID, WORKER_PROTOCOL, WORKER_REQUEST_INVALID,
};
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

/// One run's cancellation: a flag the wait and the supervisor read, and kill access to its worker while unreaped.
#[derive(Debug, Default)]
struct RunControl {
    state: Mutex<RunState>,
}

#[derive(Debug, Default)]
struct RunState {
    cancelled: bool,
    worker: Option<Arc<WorkerControl>>,
}

impl RunControl {
    fn lock(&self) -> MutexGuard<'_, RunState> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn cancelled(&self) -> bool {
        self.lock().cancelled
    }

    /// Requests cancellation and kills the worker if one is attached; its supervisor reaps it, then the run releases
    /// the grant.
    fn cancel(&self) {
        let mut state = self.lock();
        state.cancelled = true;
        if let Some(worker) = &state.worker {
            worker.kill();
        }
    }
}

type Slot = Arc<Mutex<Option<Arc<RunControl>>>>;

fn lock_slot(slot: &Slot) -> MutexGuard<'_, Option<Arc<RunControl>>> {
    slot.lock().unwrap_or_else(PoisonError::into_inner)
}

/// The MBF export's runs in progress (managed app state): at most one review and one save. A newer one of either
/// kind cancels the older.
#[derive(Clone, Debug, Default)]
pub(crate) struct MbfRuns {
    review: Slot,
    save: Slot,
}

impl MbfRuns {
    /// Makes a new run the current one of `slot`, cancelling the one it supersedes and waking the pool's waiters so a
    /// waiting run sees it at once.
    fn register(slot: &Slot, pool: &MemoryPool) -> Registration {
        let control = Arc::new(RunControl::default());
        let previous = lock_slot(slot).replace(Arc::clone(&control));
        if let Some(previous) = previous {
            previous.cancel();
            pool.wake_waiters();
        }
        Registration { slot: Arc::clone(slot), control }
    }

    /// Cancels the review and the save in progress, if any: a waiting run stops waiting, a running worker is killed
    /// and its grant released after the reap. Returns whether one was in progress. Slice 4b.2b calls it when the
    /// dialog closes and when the project session changes.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn cancel(&self, pool: &MemoryPool) -> bool {
        let current: Vec<_> = [&self.review, &self.save].into_iter().filter_map(|slot| lock_slot(slot).take()).collect();
        for control in &current {
            control.cancel();
        }
        if !current.is_empty() {
            pool.wake_waiters();
        }
        !current.is_empty()
    }
}

/// A run's place in its slot: dropping it clears the slot if the run is still the current one.
struct Registration {
    slot: Slot,
    control: Arc<RunControl>,
}

impl Drop for Registration {
    fn drop(&mut self) {
        let mut slot = lock_slot(&self.slot);
        if slot.as_ref().is_some_and(|current| Arc::ptr_eq(current, &self.control)) {
            *slot = None;
        }
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
    /// The review's run: it becomes `runs`' review in progress, superseding an older one.
    pub(crate) fn review(registry: &'r SolveJobRegistry, ledger: &Arc<CommandLedger>, runs: &MbfRuns) -> Self {
        Self::new(registry, ledger, MbfRuns::register(&runs.review, registry.pool()))
    }

    /// The save's run: it becomes `runs`' save in progress.
    pub(crate) fn save(registry: &'r SolveJobRegistry, ledger: &Arc<CommandLedger>, runs: &MbfRuns) -> Self {
        Self::new(registry, ledger, MbfRuns::register(&runs.save, registry.pool()))
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

impl MbfWriterRun for CappedWorkerRun<'_> {
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
                debug_assert!(lease.bytes() <= RESULT_LEASE_BYTES);
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

/// The command's memory and arguments: `copy` seeds its ledger, and the arguments' bound grows it before they are
/// parsed (`command_ingress::admit_raw`); the raw copy ends here.
fn admit<'r>(registry: &'r SolveJobRegistry, raw: Vec<u8>, copy: Lease, command: &'static str) -> Result<(NativeLeaseMemory<'r>, Args, CommandLease), SolveJobError> {
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

/// `review_mbf_export` after [`reserve`]: admission, the review in the capped worker, and `respond` with the outcome
/// and its text while the command's leases and the worker's result lease are held.
pub(crate) fn review_command<R>(
    registry: &SolveJobRegistry,
    runs: &MbfRuns,
    raw: Vec<u8>,
    copy: Lease,
    respond: impl FnOnce(&MbfReviewOutcome, String) -> R,
) -> Result<R, SolveJobError> {
    let (memory, mut args, lease) = admit(registry, raw, copy, mbf_export::REVIEW_COMMAND)?;
    let request = mbf_export::review_mbf_export_args(&mut args)?;
    drop(args);
    let response = mbf_export::review_mbf_export_admitted(request, &memory, &lease, runs, respond);
    drop(lease);
    Ok(response)
}

/// `save_mbf_export` after [`reserve`]: admission, the save (§3.6) with task 3 in the capped worker, the chooser
/// `choose` and the store `open_store`, and `respond` as for the review.
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
    let request = mbf_export::save_mbf_export_args(&mut args)?;
    drop(args);
    let response = mbf_export::save_mbf_export_admitted(request, open_store, &memory, &lease, runs, choose, respond);
    drop(lease);
    Ok(response)
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
