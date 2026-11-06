//! Hanger sizing in the capped worker (DEL-04-11 Design `hanger-sizing.md` §13, increment 3c part 2): the blocking
//! parts of the native commands `size_hangers` and `cancel_hanger_sizing` (`lib.rs` registers them), worker task 4
//! ([`WorkerTask::HangerSizing`], `open_pipe_stress_hanger_sizing::desktop::size_and_read`) and its one-current run.
//!
//! The path is the model checks' capped-worker request: one request, one bounded reply, returned by the command. No
//! result is held after the reply is built: there is no job record, held-result registry, custody, persistence or
//! analysis-run record, and the sizing never writes the model.
//!
//! **The request** is an `args-json-v1` raw body `{runId: {page, sequence}, inputText}`. `inputText` is the JSON text
//! of exactly the headless input (§8.3.2), `{"request": {"model", "materials"}, "sizing", "library"}`, which the worker
//! receives byte for byte. The reply is the entry's own JSON text, `{"document", "evidence": {"document_sha256",
//! "verdict"}}` (3c part 1), with reproduction `not_checked`.
//!
//! **Accounting** (as MBF task 3's, `mbf_worker.rs`):
//!
//! 1. **Raw admission.** The raw body is copied under a capped interactive lease ([`reserve`]) that seeds the
//!    command's `CommandLedger` (its budget the one that lease was admitted under, capped at 12 GiB: the *captured*
//!    invocation budget), and the arguments' bound grows it before they are parsed (`command_ingress::admit_raw`:
//!    the prescan, then the priced lease, then the parse). Nothing model-sized is decoded in this process: the
//!    input text stays a string until the worker admits it.
//! 2. **The bound.** The worker's maximum grant is what the captured budget leaves after the command's live leases,
//!    the parent's copy of the request and the largest result lease ([`RESULT_LEASE_BYTES`], the envelope of a reply
//!    at [`REPLY_LIMIT_BYTES`]). The three are reserved in the ledger as one `LedgerBound` before any of them is
//!    admitted, and refused by name above the budget (`ingress_exceeds_budget`), as is a starting grant above the
//!    maximum.
//! 3. **The queue wait** for the starting grant (the process reserve plus the request's resident envelope, whole MiB):
//!    a worker lease taken by the solve's own queue wait for at most [`START_WAIT`] (the model checks' window,
//!    `MODEL_CHECKS_START_WAIT`), ended early by cancellation or supersession, after which the last queue reason
//!    stands as a typed refusal.
//! 4. **The request copy**, admitted before it is made and handed to the supervisor, which drops it once sent.
//! 5. **Supervision** (`solve_worker::supervise`, unchanged) of task 4 with [`SizingHooks`]. The worker is reaped
//!    inside `supervise`; only then is the grant released.
//! 6. **The reply** comes back in a result lease admitted while the grant was held; the bound shrinks to that lease
//!    and both stay counted until the command's response is built ([`ReplyHold`]).
//!
//! **The reply limit.** [`SizingHooks::result_limit`] is [`REPLY_LIMIT_BYTES`], the desktop's 64 MiB displayable
//! limit (`MAX_DISPLAYABLE_RESULT_BYTES`). It bounds the complete reply, document and evidence together, on the
//! length the worker announces, before the result lease is admitted or any byte of the reply is read: a longer reply
//! ends the run `HANGER-SIZING-REPLY-TOO-LARGE` with its worker killed and reaped. No custody route exists for a larger
//! reply.
//!
//! **Cancellation and supersession** ([`HangerSizingRuns`]): one current run, in a slot of its own (not the model
//! check's, not the MBF export's). Every request carries its run id, `{page, sequence}`: a random token made once per
//! page load and a sequence that only rises in that page. For each page the host keeps the latest sequence it started
//! and the latest it cancelled through:
//!
//! - a start supersedes the current run, queued or running (its wait ends, or its worker is killed and reaped before
//!   its grant is released), unless its sequence is below the page's latest start (an older delivery, refused
//!   `HANGER-SIZING-REQUEST-STALE`: it starts and supersedes nothing) or at or below the page's latest cancel (refused
//!   `HANGER-SIZING-CANCELLED` before any worker);
//! - a repeated delivery of the page's latest start (Tauri's IPC may deliver one invoke twice: its transport resends a
//!   command through `postMessage` when its `fetch` fails, without cancelling the first delivery) runs again and
//!   supersedes the first; the reply is the same for the same bytes, and only one reply reaches the page;
//! - `cancel_hanger_sizing {runId}` cancels through that sequence of its page: the current run if it is of that page
//!   and not newer, and every start of that page not yet registered. A repeated or older cancel is a no-op whose
//!   reply says `duplicate`; it can never reach a newer run of its page. It is sent when a project session is
//!   retired and when the person cancels, as Tauri's JSON arguments (the form that survives the `postMessage`
//!   fallback);
//! - the session reset (`SolveJobRegistry::dispose_all_results`) cancels the current run beside the model check.
//!
//! The host keeps at most [`RUN_PAGES_MAX`] page marks, least recently used first out. Residual: page tokens are not
//! ordered against each other, so a start delivered late from a page that is gone (after a reload) supersedes the
//! current page's run, as a late session reset retires a later page's solves; the webview's generation guard
//! (`hangerSizingService.ts`) never shows such a run's reply. Ids are not authenticated and authorize nothing: a
//! cancel only stops work.

use crate::command_ingress::{self, ArgsDecoding, CommandError};
use crate::ingress::allocation_envelope;
use crate::memory_pool::{CommandLease, CommandLedger, Lease, LeasePurpose, LedgerBound, MemoryPool, ResourceRefusal, WaitOutcome};
use crate::solve_jobs::{SolveJobError, SolveJobRegistry, MAX_DISPLAYABLE_RESULT_BYTES, MODEL_CHECKS_START_WAIT};
use crate::solve_worker::{initial_grant, panic_message, supervise, SupervisorHooks, WorkerControl, WorkerOutcome, WorkerTask, WorkerTerms};
use crate::stored_read::NativeLeaseMemory;
use crate::worker_alloc::resident_envelope;
use open_pipe_stress_hanger_sizing::desktop::{SizeAndReadError, INPUT_NOT_UTF8};
use open_pipe_stress_hanger_sizing::publish::{INPUT_TOO_LARGE, MAX_INPUT_BYTES, RESULT_NOT_PUBLISHABLE};
use serde::Serialize;
use serde_json::Value;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};
use tauri::http::HeaderMap;
use tauri::ipc::InvokeBody;

/// `size_hangers`'s command name.
pub(crate) const COMMAND: &str = "size_hangers";

/// The request member that carries the run id.
pub(crate) const RUN_ID: &str = "runId";

/// The request member that carries the headless input's JSON text.
pub(crate) const INPUT_TEXT: &str = "inputText";

/// How long task 4 waits for its worker's starting grant: the model checks' window, reused explicitly.
pub(crate) const START_WAIT: Duration = MODEL_CHECKS_START_WAIT;

/// The largest reply task 4 may hand over, document and evidence together: the desktop's displayable limit.
pub(crate) const REPLY_LIMIT_BYTES: u64 = MAX_DISPLAYABLE_RESULT_BYTES;

/// The largest result lease the supervisor admits for task 4: the resident envelope of a reply at the limit.
pub(crate) const RESULT_LEASE_BYTES: u64 = resident_envelope(REPLY_LIMIT_BYTES);

/// The most page marks the host keeps (least recently used first out).
pub(crate) const RUN_PAGES_MAX: usize = 64;

/// A page token's length: 16 random bytes as lowercase hex.
const PAGE_TOKEN_HEX_LEN: usize = 32;

/// The largest sequence a page sends (JavaScript's largest safe integer).
const RUN_SEQUENCE_MAX: u64 = (1 << 53) - 1;

pub(crate) const CANCELLED: &str = "HANGER-SIZING-CANCELLED";
pub(crate) const SUPERSEDED: &str = "HANGER-SIZING-SUPERSEDED";
pub(crate) const REQUEST_STALE: &str = "HANGER-SIZING-REQUEST-STALE";
pub(crate) const RUN_ID_INVALID: &str = "HANGER-SIZING-RUN-ID-INVALID";
pub(crate) const ARGS_INVALID: &str = "HANGER-SIZING-ARGS-INVALID";
pub(crate) const REPLY_TOO_LARGE: &str = "HANGER-SIZING-REPLY-TOO-LARGE";
pub(crate) const WORKER_FAILED: &str = "HANGER-SIZING-WORKER-FAILED";

/// Task 4's own failure codes (`size_and_read`'s typed refusals): a failure under any other name is not task 4's.
const TASK_FAILURE_CODES: [&str; 3] = [INPUT_TOO_LARGE, INPUT_NOT_UTF8, RESULT_NOT_PUBLISHABLE];

// -- Run ids --------------------------------------------------------------------------------------------------------

/// One request's id: its page's token (made once per page load) and its sequence in that page (from 1, rising by one
/// per logical request; a re-delivery reuses it).
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct RunId {
    pub(crate) page: String,
    pub(crate) sequence: u64,
}

impl RunId {
    /// Reads an id: an object of exactly `page` (32 lowercase hex digits) and `sequence` (an integer from 1 to
    /// 2^53 - 1). Anything else, or none, refuses `HANGER-SIZING-RUN-ID-INVALID`.
    pub(crate) fn read(value: Option<Value>) -> Result<Self, String> {
        let refused = |what: &str| format!("{RUN_ID_INVALID}: the request {what}. Nothing was started or cancelled.");
        let mut map = match value {
            None => return Err(refused("carries no run id")),
            Some(Value::Object(map)) => map,
            Some(_) => return Err(refused("carries a malformed run id")),
        };
        let page = match map.remove("page") {
            Some(Value::String(page))
                if page.len() == PAGE_TOKEN_HEX_LEN && page.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) =>
            {
                page
            }
            _ => return Err(refused("carries a malformed run id")),
        };
        let sequence = map.remove("sequence").and_then(|value| value.as_u64()).filter(|sequence| (1..=RUN_SEQUENCE_MAX).contains(sequence));
        match sequence {
            Some(sequence) if map.is_empty() => Ok(RunId { page, sequence }),
            _ => Err(refused("carries a malformed run id")),
        }
    }
}

// -- Cancellation ---------------------------------------------------------------------------------------------------

/// Why a run stopped before it replied.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Stop {
    /// A cancel of its page, a project-session change or the session reset.
    Cancelled,
    /// A newer request (or a repeated delivery of the same one) became current.
    Superseded,
}

/// One run's cancellation: the reason it stopped, read by the wait and the supervisor, and kill access to its worker
/// while unreaped.
#[derive(Debug, Default)]
struct RunControl {
    state: Mutex<RunState>,
}

#[derive(Debug, Default)]
struct RunState {
    stop: Option<Stop>,
    worker: Option<Arc<WorkerControl>>,
}

impl RunControl {
    fn lock(&self) -> MutexGuard<'_, RunState> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn stopped(&self) -> Option<Stop> {
        self.lock().stop
    }

    /// Stops the run (the first reason stands) and kills its worker if one is attached; its supervisor reaps it, then
    /// the run releases the grant. Whether this call stopped it.
    fn stop(&self, why: Stop) -> bool {
        let mut state = self.lock();
        if state.stop.is_some() {
            return false;
        }
        state.stop = Some(why);
        if let Some(worker) = &state.worker {
            worker.kill();
        }
        true
    }
}

/// One page's marks: the latest sequence started and the latest cancelled through.
#[derive(Debug)]
struct PageMark {
    page: String,
    started: u64,
    cancelled: u64,
}

#[derive(Debug, Default)]
struct RunsState {
    current: Option<(RunId, Arc<RunControl>)>,
    /// Least recently used first; at most [`RUN_PAGES_MAX`].
    pages: Vec<PageMark>,
}

impl RunsState {
    fn mark(&mut self, page: &str) -> &mut PageMark {
        let mark = match self.pages.iter().position(|mark| mark.page == page) {
            Some(index) => self.pages.remove(index),
            None => {
                if self.pages.len() >= RUN_PAGES_MAX {
                    self.pages.remove(0);
                }
                PageMark { page: page.to_string(), started: 0, cancelled: 0 }
            }
        };
        self.pages.push(mark);
        let last = self.pages.len() - 1;
        &mut self.pages[last]
    }
}

/// `cancel_hanger_sizing`'s reply.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct HangerSizingCancelReceipt {
    /// This cancel stopped the current run.
    pub(crate) cancelled: bool,
    /// This id, or a later one of its page, had been cancelled already: nothing changed.
    pub(crate) duplicate: bool,
    pub(crate) run_id: RunId,
}

/// The hanger sizing's current run (the registry's own slot, `SolveJobRegistry::hanger_sizing_runs`) and each page's
/// marks (module docs, "Cancellation and supersession").
#[derive(Clone, Debug, Default)]
pub(crate) struct HangerSizingRuns {
    state: Arc<Mutex<RunsState>>,
}

impl HangerSizingRuns {
    fn lock(&self) -> MutexGuard<'_, RunsState> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Makes `id` the current run, superseding the one in progress, or refuses it: cancelled when its page has been
    /// cancelled through it, stale when its page has started a later one.
    pub(crate) fn register(&self, id: &RunId, pool: &MemoryPool) -> Result<Registration, SizingFailure> {
        let mut state = self.lock();
        let mark = state.mark(&id.page);
        if id.sequence <= mark.cancelled {
            return Err(SizingFailure::Stopped(Stop::Cancelled));
        }
        if id.sequence < mark.started {
            return Err(SizingFailure::Stale);
        }
        mark.started = id.sequence;
        let control = Arc::new(RunControl::default());
        let previous = state.current.replace((id.clone(), Arc::clone(&control)));
        drop(state);
        if previous.is_some_and(|(_, previous)| previous.stop(Stop::Superseded)) {
            pool.wake_waiters();
        }
        Ok(Registration { runs: self.clone(), control })
    }

    /// `cancel_hanger_sizing`: cancels `id`'s page through its sequence. A sequence no greater than the page's last
    /// cancel is a duplicate and changes nothing.
    pub(crate) fn cancel(&self, id: RunId, pool: &MemoryPool) -> HangerSizingCancelReceipt {
        let mut state = self.lock();
        let mark = state.mark(&id.page);
        if id.sequence <= mark.cancelled {
            return HangerSizingCancelReceipt { cancelled: false, duplicate: true, run_id: id };
        }
        mark.cancelled = id.sequence;
        let reached = state.current.as_ref().is_some_and(|(current, _)| current.page == id.page && current.sequence <= id.sequence);
        let current = if reached { state.current.take() } else { None };
        drop(state);
        let cancelled = current.is_some_and(|(_, control)| control.stop(Stop::Cancelled));
        if cancelled {
            pool.wake_waiters();
        }
        HangerSizingCancelReceipt { cancelled, duplicate: false, run_id: id }
    }

    /// The session reset's cancel (`SolveJobRegistry::dispose_all_results`): the current run, whichever page it is
    /// of. Whether one was stopped.
    pub(crate) fn cancel_current(&self, pool: &MemoryPool) -> bool {
        let current = self.lock().current.take();
        let cancelled = current.is_some_and(|(_, control)| control.stop(Stop::Cancelled));
        if cancelled {
            pool.wake_waiters();
        }
        cancelled
    }

    /// The current run's id (tests).
    #[cfg(test)]
    pub(crate) fn current_id(&self) -> Option<RunId> {
        self.lock().current.as_ref().map(|(id, _)| id.clone())
    }
}

/// A run's place as the current one: dropping it (as the run ends) clears the slot if it is still current.
pub(crate) struct Registration {
    runs: HangerSizingRuns,
    control: Arc<RunControl>,
}

impl Drop for Registration {
    fn drop(&mut self) {
        let mut state = self.runs.lock();
        if state.current.as_ref().is_some_and(|(_, current)| Arc::ptr_eq(current, &self.control)) {
            state.current = None;
        }
    }
}

// -- The run --------------------------------------------------------------------------------------------------------

/// How a run did not reply.
#[derive(Debug)]
pub(crate) enum SizingFailure {
    Stopped(Stop),
    /// An older delivery of its page: nothing was started or superseded.
    Stale,
    /// A typed memory-pool refusal (the bound, the grant, its growth or footprint, the request copy, the result lease).
    Memory(SolveJobError),
    /// Task 4's own refusal, `<code>: <message>`.
    Task(String),
    /// The announced reply exceeded [`REPLY_LIMIT_BYTES`]; it was not read and the worker was killed and reaped.
    ReplyTooLarge { reply_bytes: u64, limit_bytes: u64 },
    /// Any other worker failure: an exit, a panic, a protocol violation, an attested or non-UTF-8 reply.
    Worker(String),
}

impl SizingFailure {
    /// The command's error: a memory refusal as the typed ingress error, everything else as `<code>: <message>`.
    pub(crate) fn into_command_error(self) -> CommandError<String> {
        let text = match self {
            SizingFailure::Memory(error) => return CommandError::Ingress(error),
            SizingFailure::Stopped(Stop::Cancelled) => format!(
                "{CANCELLED}: the sizing was cancelled before it replied (a cancel, a project-session change or a session reset); its worker was stopped and nothing was published."
            ),
            SizingFailure::Stopped(Stop::Superseded) => {
                format!("{SUPERSEDED}: a newer sizing request replaced this one before it replied; its worker was stopped.")
            }
            SizingFailure::Stale => format!("{REQUEST_STALE}: this page has sent a newer sizing request; this delivery started and replaced nothing."),
            SizingFailure::Task(text) => text,
            SizingFailure::ReplyTooLarge { reply_bytes, limit_bytes } => format!(
                "{REPLY_TOO_LARGE}: the sizing reply of {reply_bytes} bytes is above the desktop's {limit_bytes}-byte limit for one reply; it was not read and its worker was stopped. Size fewer candidates or operating cases in one request."
            ),
            SizingFailure::Worker(detail) => format!("{WORKER_FAILED}: {detail}"),
        };
        CommandError::Command(text)
    }
}

/// A reply's memory until the command's response is built: the worker's result lease and the command's ledger
/// bound, shrunk to it. The lease is released first, then the ledger's charge.
#[derive(Debug)]
pub(crate) struct ReplyHold {
    lease: Lease,
    _bound: LedgerBound,
}

impl ReplyHold {
    #[cfg(test)]
    pub(crate) fn lease_bytes(&self) -> u64 {
        self.lease.bytes()
    }
}

/// A completed run: the reply text and its memory.
#[derive(Debug)]
pub(crate) struct SizingReply {
    pub(crate) reply: String,
    pub(crate) hold: ReplyHold,
}

/// The supervisor's view of a run: its cancellation and the reply limit.
struct SizingHooks<'c> {
    control: &'c RunControl,
}

impl SupervisorHooks for SizingHooks<'_> {
    fn attach(&mut self, worker: Arc<WorkerControl>) -> bool {
        let mut state = self.control.lock();
        state.worker = Some(worker);
        state.stop.is_none()
    }

    fn cancelled(&self) -> bool {
        self.control.stopped().is_some()
    }

    fn grant_changed(&mut self, _grant_bytes: u64) {}

    fn footprint_observed(&mut self, _footprint_bytes: u64) {}

    /// The 64 MiB reply limit, on the announced length before any result lease or read.
    fn result_limit(&self) -> Option<u64> {
        Some(REPLY_LIMIT_BYTES)
    }
}

fn refused(registry: &SolveJobRegistry, refusal: ResourceRefusal) -> SizingFailure {
    SizingFailure::Memory(registry.refuse(refusal))
}

/// A worker failure: task 4's own by its code, anything else as a worker failure.
fn worker_failure(text: String) -> SizingFailure {
    match text.split_once(": ") {
        Some((code, _)) if TASK_FAILURE_CODES.contains(&code) => SizingFailure::Task(text),
        _ => SizingFailure::Worker(text),
    }
}

/// Task 4 on `input` in the capped worker, within `ledger` (module docs, "Accounting"), for the run `registration`
/// made current.
pub(crate) fn run(
    registry: &SolveJobRegistry,
    ledger: &Arc<CommandLedger>,
    registration: &Registration,
    input: String,
    start_wait: Duration,
) -> Result<SizingReply, SizingFailure> {
    let control = Arc::clone(&registration.control);
    if let Some(why) = control.stopped() {
        return Err(SizingFailure::Stopped(why));
    }
    // The entry's own input bound, applied before a worker is started for it (the worker applies it again).
    if input.len() > MAX_INPUT_BYTES {
        let error = SizeAndReadError::TooLarge { bytes: input.len() };
        return Err(SizingFailure::Task(format!("{}: {}", error.code(), error.message())));
    }
    // 1. The bound: the worker's maximum grant is what the captured budget leaves after the command's live leases,
    // the request copy and the largest result lease; all three are reserved in the ledger before any is admitted.
    let raw_len = input.len() as u64;
    let reserve = registry.process_reserve();
    let request_bytes = allocation_envelope(raw_len).unwrap_or(u64::MAX);
    let start = initial_grant(reserve, resident_envelope(raw_len)).unwrap_or(u64::MAX);
    let budget = ledger.budget_bytes();
    let fixed = ledger.total_bytes().saturating_add(request_bytes).saturating_add(RESULT_LEASE_BYTES);
    let max_grant = budget.saturating_sub(fixed);
    let over_budget = |required_bytes: u64| refused(registry, ResourceRefusal::IngressExceedsBudget { required_bytes, budget_bytes: budget });
    if start > max_grant {
        return Err(over_budget(fixed.saturating_add(start)));
    }
    let mut bound = ledger.bound(request_bytes + max_grant + RESULT_LEASE_BYTES).map_err(|over| over_budget(over.required_bytes))?;
    // 2. The queue wait for the starting grant, bounded, ended by cancellation or a newer run.
    let deadline = Instant::now() + start_wait;
    let mut queued: Option<ResourceRefusal> = None;
    let waited = registry.pool().wait_acquire(
        start,
        LeasePurpose::Worker,
        Some(max_grant),
        &|| control.stopped().is_some() || Instant::now() >= deadline,
        &mut |reason| queued = Some(reason.clone()),
    );
    let mut grant = match waited {
        Ok(grant) => grant,
        Err(WaitOutcome::Cancelled) => match control.stopped() {
            Some(why) => return Err(SizingFailure::Stopped(why)),
            // The first attempt always precedes the deadline, so a reason was recorded.
            None => return Err(refused(registry, queued.unwrap_or(ResourceRefusal::HeadroomUnknown))),
        },
    };
    // 3. The request copy, admitted before it is made; the input text ends here.
    let copy = match registry.acquire_interactive_ingress_capped(request_bytes) {
        Ok(copy) => copy,
        Err(error) => return Err(SizingFailure::Memory(error)),
    };
    let raw = input.as_bytes().to_vec();
    drop(input);
    // 4. Supervision; the worker is reaped inside `supervise` (also while it unwinds), then the grant released.
    let terms = WorkerTerms { pool: registry.pool(), launcher: registry.worker_launcher(), task: WorkerTask::HangerSizing, max_grant, reserve };
    let mut hooks = SizingHooks { control: &control };
    let outcome = catch_unwind(AssertUnwindSafe(|| supervise(&terms, &mut grant, (raw, copy), &mut hooks)));
    control.lock().worker = None;
    drop(grant);
    if let Some(why) = control.stopped() {
        // A reply that arrived as the run was stopped answers a retired request; it and its lease end here.
        drop(outcome);
        return Err(SizingFailure::Stopped(why));
    }
    match outcome {
        Err(payload) => Err(SizingFailure::Worker(format!("the sizing worker's supervisor panicked: {}", panic_message(payload.as_ref())))),
        Ok(WorkerOutcome::Completed { attestation: Some(_), .. }) => Err(SizingFailure::Worker("task 4 attests nothing, but its reply was attested".to_string())),
        Ok(WorkerOutcome::Completed { result, lease, attestation: None }) => {
            // 5. The reply's memory: the grant and the request copy have ended; the bound now covers the result lease.
            // A lease above the bound's share would leave the ledger under-counting it: refused, in every build.
            if lease.bytes() > RESULT_LEASE_BYTES {
                return Err(SizingFailure::Worker(format!(
                    "the result lease of {} bytes exceeds the {RESULT_LEASE_BYTES} bytes its bound holds",
                    lease.bytes()
                )));
            }
            bound.shrink_to(lease.bytes());
            let reply = String::from_utf8(result).map_err(|_| SizingFailure::Worker("the reply is not UTF-8".to_string()))?;
            Ok(SizingReply { reply, hold: ReplyHold { lease, _bound: bound } })
        }
        Ok(WorkerOutcome::Failed(text)) => Err(worker_failure(text)),
        Ok(WorkerOutcome::Refused(refusal)) => Err(refused(registry, refusal)),
        Ok(WorkerOutcome::ResultTooLarge { result_bytes, limit_bytes }) => Err(SizingFailure::ReplyTooLarge { reply_bytes: result_bytes, limit_bytes }),
        Ok(WorkerOutcome::Terminated) => Err(SizingFailure::Worker("the sizing worker was stopped".to_string())),
    }
}

// -- The commands' blocking parts -----------------------------------------------------------------------------------

/// The executor's part of `size_hangers` (C1's): the header check, the raw body's copy reserved as a capped
/// interactive lease before it is made, and the copy.
pub(crate) fn reserve(registry: &SolveJobRegistry, body: &InvokeBody, headers: &HeaderMap) -> Result<(Vec<u8>, Lease), SolveJobError> {
    let raw = command_ingress::raw_args_body(body, headers, COMMAND)?;
    let lease = registry.acquire_interactive_ingress_capped(allocation_envelope(raw.len() as u64).unwrap_or(u64::MAX))?;
    Ok((raw.to_vec(), lease))
}

/// The command's memory and arguments: `copy` seeds its ledger, and the arguments' bound grows it before they are
/// parsed (`command_ingress::admit_raw`); the raw copy ends here.
fn admit(registry: &SolveJobRegistry, raw: Vec<u8>, copy: Lease) -> Result<(command_ingress::Args, CommandLease), SolveJobError> {
    let (memory, copy) = NativeLeaseMemory::seeded(registry, copy);
    let copy_bytes = copy.bytes();
    let (args, lease) = command_ingress::admit_raw(registry, &raw, COMMAND, ArgsDecoding::Moved, |bytes| {
        let mut lease = copy;
        memory.grow(&mut lease, copy_bytes.saturating_add(bytes))?;
        Ok(lease)
    })?;
    drop(raw);
    Ok((args, lease))
}

/// `size_hangers` after [`reserve`]: admission, the run id and input text, registration as the current run, task 4
/// in the capped worker, and `respond` with the reply text while the command's lease and the reply's are held.
pub(crate) fn size_command<R>(
    registry: &SolveJobRegistry,
    raw: Vec<u8>,
    copy: Lease,
    start_wait: Duration,
    respond: impl FnOnce(String) -> R,
) -> Result<R, CommandError<String>> {
    let (mut args, lease) = admit(registry, raw, copy)?;
    let id = RunId::read(args.opt_value(RUN_ID)).map_err(CommandError::Command)?;
    let input = match args.opt_value(INPUT_TEXT) {
        Some(Value::String(text)) => text,
        _ => {
            return Err(CommandError::Command(format!(
                "{ARGS_INVALID}: `{INPUT_TEXT}` must be the hanger-sizing input's JSON text. Nothing was started."
            )))
        }
    };
    drop(args);
    let registration = registry.hanger_sizing_runs().register(&id, registry.pool()).map_err(SizingFailure::into_command_error)?;
    let outcome = run(registry, lease.ledger(), &registration, input, start_wait);
    drop(registration);
    match outcome {
        Ok(SizingReply { reply, hold }) => {
            let response = respond(reply);
            drop(hold);
            drop(lease);
            Ok(response)
        }
        Err(failure) => Err(failure.into_command_error()),
    }
}

/// `cancel_hanger_sizing`: `{runId}` as Tauri's JSON arguments (the one form that reaches the host unchanged on both
/// of Tauri's transports, as `cancel_mbf_export`'s), then the keyed cancel. A missing or malformed id is refused
/// `HANGER-SIZING-RUN-ID-INVALID` and cancels nothing. Tauri has parsed the arguments before the command runs; the id
/// is read from at most 32 hex digits and one integer.
pub(crate) fn cancel_command(registry: &SolveJobRegistry, run_id: Option<Value>) -> Result<HangerSizingCancelReceipt, String> {
    RunId::read(run_id).map(|id| registry.hanger_sizing_runs().cancel(id, registry.pool()))
}

/// A command whose blocking part did not complete.
pub(crate) fn supervisor_failed() -> CommandError<String> {
    CommandError::Command(format!("{WORKER_FAILED}: the hanger sizing's supervisor did not complete."))
}

#[cfg(test)]
#[path = "hanger_sizing_job_tests.rs"]
mod tests;
