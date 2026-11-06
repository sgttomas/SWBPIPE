//! Hanger sizing in the capped worker (DEL-04-11 Design `hanger-sizing.md` §13, increment 3c part 2): the blocking
//! parts of the native commands `size_hangers` and `cancel_hanger_sizing` (`lib.rs` registers them), worker task 4
//! ([`WorkerTask::HangerSizing`], `open_pipe_stress_hanger_sizing::desktop::size_and_read`) and its one-current run.
//!
//! The path is the model checks' capped-worker request: one request, one bounded reply, returned by the command. No
//! result is held after the reply is built: there is no job record, held-result registry, custody, persistence or
//! analysis-run record, and the sizing never writes the model.
//!
//! **The request** is an `args-json-v1` raw body `{expectedEpoch, inputText}`. `expectedEpoch` is the request's
//! dispatch ticket (below). `inputText` is the JSON text of exactly the headless input (§8.3.2),
//! `{"request": {"model", "materials"}, "sizing", "library"}`, which the worker receives byte for byte. The reply is the
//! entry's own JSON text, `{"document", "evidence": {"document_sha256", "verdict"}}` (3c part 1), with reproduction
//! `not_checked`.
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
//!    `MODEL_CHECKS_START_WAIT`), ended early by a retirement, after which the last queue reason stands as a typed
//!    refusal.
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
//! **Retirement and the dispatch ticket** ([`HangerSizingRuns`], MBF's design in `mbf_worker.rs` applied to the
//! sizing's own slot: not the model check's, not the MBF export's). The sizing keeps a *retirement epoch*. A
//! retirement moves it on by one and retires everything before it: the current run (its wait ends, or its worker is
//! killed and reaped before its grant is released; it replies `HANGER-SIZING-CANCELLED`) and every start that has
//! not yet registered (it is refused when it does). Two things retire:
//!
//! - `cancel_hanger_sizing {retirementId}` ([`cancel_command`], [`HangerSizingRuns::retire_keyed`]), which replies the
//!   epoch after its bump. The webview (`hangerSizingService.ts`) sends one before every start, so a newer request
//!   retires the one in progress, and one when the person cancels and when the form's project session ends (its
//!   unmount);
//! - the session reset (`SolveJobRegistry::dispose_all_results`, [`HangerSizingRuns::cancel_current`]).
//!
//! Every start carries a ticket, `expectedEpoch`: the epoch its own retirement's reply acknowledged, taken by the
//! webview before it sends, never an epoch read when the command's task happens to run (Tauri 2 extracts an async
//! command's arguments inside the task it spawns, so a request sent before a retirement may begin after it). The host
//! checks the ticket twice ([`HangerSizingRuns::enter`] on the admitted arguments, before the input is read or any
//! worker starts, and [`HangerSizingRuns::register`] under the same lock as every retirement): a missing, malformed
//! or future ticket is refused `HANGER-SIZING-TICKET-INVALID`; a ticket a retirement has passed, or one a run has
//! already registered under, is refused `HANGER-SIZING-REQUEST-STALE`. A refused start starts no worker, replaces and
//! cancels nothing, and writes nothing. One ticket admits one run, and only a retirement makes a new one, so a start
//! that is delayed anywhere between its send and its registration (in the transport, in Tauri's task, in admission)
//! can never become current after a later retirement, nor replace or cancel the run that later retirement made room
//! for.
//!
//! **Idempotent retirement.** Each logical retirement carries its id ([`RetirementId`], `{page, sequence}`: a random
//! token made once per page load and a counter that only rises in that page; a re-delivery reuses it). The host keeps,
//! for every page token it has applied, the last sequence it applied, and applies a retirement only when its sequence
//! is above that one. A repeated or older delivery is a no-op: it cancels nothing, moves nothing on, and replies
//! `duplicate: true` with the current epoch. An applied token is never forgotten: the host keeps at most
//! [`RETIREMENT_PAGES_MAX`] of them, and a retirement from a new token beyond them is applied once and makes the
//! sizing exhausted instead of evicting a token, whose delayed duplicate would otherwise be applied again.
//!
//! **Exhaustion** (terminal until the process restarts): a retirement finds the epoch at [`EPOCH_MAX`] (2^53 - 1, so
//! every ticket stays a safe JavaScript integer; it moves on by checked addition and never wraps), or comes from a new
//! page token with [`RETIREMENT_PAGES_MAX`] kept (in practice the 65th page load in one process that sizes or
//! cancels). That retirement still cancels the current run; from then on every start is refused
//! `HANGER-SIZING-EXHAUSTED`, at its entry or when it registers, and the retirement replies say `exhausted`.
//!
//! **The wire forms.** `size_hangers` takes only a raw body (`INGRESS-RAW-BODY-REQUIRED` otherwise).
//! `cancel_hanger_sizing` takes Tauri's own JSON arguments, the one form that reaches the host unchanged on both of
//! Tauri 2.11.1's transports. Tauri's IPC script posts an invoke over its custom protocol and, when that `fetch`
//! fails, resends it through `postMessage` without cancelling the first delivery; the fallback `JSON.stringify`s the
//! payload, so a raw `Uint8Array` arrives as a JSON array of numbers, which `size_hangers` refuses. So when the
//! custom-protocol request fails after the host has received it, the page sees the fallback's ingress refusal while
//! the first delivery's worker keeps running; the next retirement (the next start's, the person's cancel, the form's
//! unmount or the session reset) retires it, since the keyed cancel is JSON and survives the fallback. Nothing here
//! claims the fallback is otherwise safe: a start sent after the transport has fallen back is refused, never run.
//!
//! **Residual.** Page tokens are not ordered against each other. A retirement from another page that has not been
//! applied before (its first delivery: an old page's unmount cancel or start retirement delivered late, after a
//! reload) or a late session reset still applies, and so cancels the current page's run (which replies
//! `HANGER-SIZING-CANCELLED`; the person sizes again). It cannot start anything: a retirement only retires, and the
//! epoch it makes is acknowledged only to the page that sent it. Ids and tickets are not authenticated and authorize
//! nothing: any code in the webview could cancel sizings, or exhaust the sizing until restart, but gains nothing else.

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

/// The request member that carries the dispatch ticket: the retirement epoch the start's own retirement acknowledged.
pub(crate) const EXPECTED_EPOCH: &str = "expectedEpoch";

/// The request member that carries the headless input's JSON text.
pub(crate) const INPUT_TEXT: &str = "inputText";

/// How long task 4 waits for its worker's starting grant: the model checks' window, reused explicitly.
pub(crate) const START_WAIT: Duration = MODEL_CHECKS_START_WAIT;

/// The largest reply task 4 may hand over, document and evidence together: the desktop's displayable limit.
pub(crate) const REPLY_LIMIT_BYTES: u64 = MAX_DISPLAYABLE_RESULT_BYTES;

/// The largest result lease the supervisor admits for task 4: the resident envelope of a reply at the limit.
pub(crate) const RESULT_LEASE_BYTES: u64 = resident_envelope(REPLY_LIMIT_BYTES);

/// The most page tokens the host keeps (one per page load that retires): a retirement from a new token beyond them
/// makes the sizing exhausted, since no applied token may be forgotten.
pub(crate) const RETIREMENT_PAGES_MAX: usize = 64;

/// The largest retirement epoch (JavaScript's largest safe integer, so every ticket round-trips through the webview).
pub(crate) const EPOCH_MAX: u64 = (1 << 53) - 1;

/// A page token's length: 16 random bytes as lowercase hex.
const PAGE_TOKEN_HEX_LEN: usize = 32;

/// The largest sequence a page sends (JavaScript's largest safe integer).
const RETIREMENT_SEQUENCE_MAX: u64 = (1 << 53) - 1;

pub(crate) const CANCELLED: &str = "HANGER-SIZING-CANCELLED";
pub(crate) const REQUEST_STALE: &str = "HANGER-SIZING-REQUEST-STALE";
pub(crate) const TICKET_INVALID: &str = "HANGER-SIZING-TICKET-INVALID";
pub(crate) const EXHAUSTED: &str = "HANGER-SIZING-EXHAUSTED";
pub(crate) const RETIREMENT_ID_INVALID: &str = "HANGER-SIZING-RETIREMENT-ID-INVALID";
pub(crate) const ARGS_INVALID: &str = "HANGER-SIZING-ARGS-INVALID";
pub(crate) const REPLY_TOO_LARGE: &str = "HANGER-SIZING-REPLY-TOO-LARGE";
pub(crate) const WORKER_FAILED: &str = "HANGER-SIZING-WORKER-FAILED";

/// Task 4's own failure codes (`size_and_read`'s typed refusals): a failure under any other name is not task 4's.
const TASK_FAILURE_CODES: [&str; 3] = [INPUT_TOO_LARGE, INPUT_NOT_UTF8, RESULT_NOT_PUBLISHABLE];

// -- Retirement ids -------------------------------------------------------------------------------------------------

/// One logical retirement's id: its page's token (made once per page load) and its sequence in that page (from 1,
/// rising by one per logical retirement; a re-delivery reuses it).
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct RetirementId {
    pub(crate) page: String,
    pub(crate) sequence: u64,
}

impl RetirementId {
    /// Reads an id: an object of exactly `page` (32 lowercase hex digits) and `sequence` (an integer from 1 to
    /// 2^53 - 1). Anything else, or none, refuses `HANGER-SIZING-RETIREMENT-ID-INVALID`.
    pub(crate) fn read(value: Option<Value>) -> Result<Self, String> {
        let refused = |what: &str| format!("{RETIREMENT_ID_INVALID}: the cancel {what}. Nothing was retired.");
        let mut map = match value {
            None => return Err(refused("carries no retirement id")),
            Some(Value::Object(map)) => map,
            Some(_) => return Err(refused("carries a malformed retirement id")),
        };
        let page = match map.remove("page") {
            Some(Value::String(page))
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

// -- Cancellation ---------------------------------------------------------------------------------------------------

/// One run's cancellation: whether a retirement stopped it, read by the wait and the supervisor, and kill access to
/// its worker while unreaped.
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

    /// Stops the run and kills its worker if one is attached; its supervisor reaps it, then the run releases the
    /// grant. Whether this call stopped it.
    fn cancel(&self) -> bool {
        let mut state = self.lock();
        if state.cancelled {
            return false;
        }
        state.cancelled = true;
        if let Some(worker) = &state.worker {
            worker.kill();
        }
        true
    }
}

#[derive(Debug, Default)]
struct RunsState {
    /// Moved on by one by every applied retirement (checked; never past [`EPOCH_MAX`]).
    epoch: u64,
    /// A run has registered under the ticket `epoch`: a second start with it is stale.
    ticket_used: bool,
    /// Terminal: every start is refused `HANGER-SIZING-EXHAUSTED` (module docs, "Exhaustion").
    exhausted: bool,
    /// Every page token a keyed retirement was applied from, with the last sequence applied from it. Never shortened;
    /// at most [`RETIREMENT_PAGES_MAX`].
    applied: Vec<(String, u64)>,
    /// The current run and the ticket it registered under.
    current: Option<(u64, Arc<RunControl>)>,
}

/// `cancel_hanger_sizing`'s reply: what the retirement did and the epoch after it, which the next start's request
/// carries as its ticket ([`EXPECTED_EPOCH`]).
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct HangerSizingRetirement {
    /// This retirement stopped the current run.
    pub(crate) cancelled: bool,
    pub(crate) epoch: u64,
    /// The keyed retirement this replies to, echoed.
    pub(crate) retirement_id: RetirementId,
    /// This id, or a later one of its page, had been applied already: nothing was cancelled or moved on, and `epoch`
    /// is the current one. Only present in the reply when true.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub(crate) duplicate: bool,
    /// The sizing can be retired no further and refuses every start (`HANGER-SIZING-EXHAUSTED`); `epoch` is no
    /// ticket. Only present in the reply when true.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub(crate) exhausted: bool,
}

/// A start's entry: the ticket its request carried, current when the command read it ([`HangerSizingRuns::enter`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SizingEntry {
    epoch: u64,
}

/// The hanger sizing's current run (the registry's own slot, `SolveJobRegistry::hanger_sizing_runs`), its retirement
/// epoch and the applied retirement ids (module docs, "Retirement and the dispatch ticket").
#[derive(Clone, Debug, Default)]
pub(crate) struct HangerSizingRuns {
    state: Arc<Mutex<RunsState>>,
}

// A test's hook, run on the command's thread after its entry and just before its run registers (the
// delayed-registration test).
#[cfg(test)]
thread_local! {
    pub(crate) static BEFORE_REGISTRATION: std::cell::RefCell<Option<Box<dyn FnOnce()>>> = std::cell::RefCell::new(None);
}

impl HangerSizingRuns {
    fn lock(&self) -> MutexGuard<'_, RunsState> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// A start's entry, from its request's ticket (the [`EXPECTED_EPOCH`] member of its admitted arguments, read
    /// before its input). Refused when the sizing is exhausted; when the ticket is missing, not a non-negative integer,
    /// or above the current epoch (`HANGER-SIZING-TICKET-INVALID`); and when it is below the current epoch or a run has
    /// already registered under it (`HANGER-SIZING-REQUEST-STALE`).
    pub(crate) fn enter(&self, ticket: Option<Value>) -> Result<SizingEntry, SizingFailure> {
        let expected = match ticket {
            None => return Err(SizingFailure::TicketInvalid("carries no ticket")),
            Some(value) => value.as_u64().ok_or(SizingFailure::TicketInvalid("carries a malformed ticket"))?,
        };
        let state = self.lock();
        if state.exhausted {
            return Err(SizingFailure::Exhausted);
        }
        if expected > state.epoch {
            return Err(SizingFailure::TicketInvalid("names a retirement epoch the host has not reached"));
        }
        if expected < state.epoch || state.ticket_used {
            return Err(SizingFailure::Stale);
        }
        Ok(SizingEntry { epoch: expected })
    }

    /// Makes the run of `entry` the current one, under the same lock as every retirement: refused, with nothing
    /// replaced or cancelled, when the sizing is exhausted, a retirement has passed its ticket since its entry, or a run
    /// has registered under the ticket already.
    pub(crate) fn register(&self, entry: SizingEntry, pool: &MemoryPool) -> Result<Registration, SizingFailure> {
        #[cfg(test)]
        if let Some(hook) = BEFORE_REGISTRATION.with(|hook| hook.borrow_mut().take()) {
            hook();
        }
        let mut state = self.lock();
        if state.exhausted {
            return Err(SizingFailure::Exhausted);
        }
        if entry.epoch != state.epoch || state.ticket_used {
            return Err(SizingFailure::Stale);
        }
        state.ticket_used = true;
        let control = Arc::new(RunControl::default());
        // Every retirement takes the current run, and one ticket admits one run: there is none to replace. Should one
        // ever be left, it is cancelled rather than left running beside the new one.
        let previous = state.current.replace((entry.epoch, Arc::clone(&control)));
        drop(state);
        if previous.is_some_and(|(_, previous)| previous.cancel()) {
            pool.wake_waiters();
        }
        Ok(Registration { runs: self.clone(), control })
    }

    /// `cancel_hanger_sizing`'s retirement, applied once per id (module docs, "Idempotent retirement"). A sequence no
    /// greater than the last applied from its page is a duplicate: nothing is cancelled or moved on, and the reply says
    /// `duplicate` with the current epoch. Otherwise the sequence is recorded and the retirement applied; a new page
    /// token with [`RETIREMENT_PAGES_MAX`] kept is not recorded, and makes the sizing exhausted as it is applied.
    pub(crate) fn retire_keyed(&self, id: RetirementId, pool: &MemoryPool) -> HangerSizingRetirement {
        let mut state = self.lock();
        match state.applied.iter().position(|(page, _)| *page == id.page) {
            Some(known) if id.sequence <= state.applied[known].1 => {
                let (epoch, exhausted) = (state.epoch, state.exhausted);
                return HangerSizingRetirement { cancelled: false, epoch, retirement_id: id, duplicate: true, exhausted };
            }
            Some(known) => state.applied[known].1 = id.sequence,
            None if state.applied.len() < RETIREMENT_PAGES_MAX => state.applied.push((id.page.clone(), id.sequence)),
            None => state.exhausted = true,
        }
        let (cancelled, epoch, exhausted) = Self::retire_locked(state, pool);
        HangerSizingRetirement { cancelled, epoch, retirement_id: id, duplicate: false, exhausted }
    }

    /// The session reset's retirement (`SolveJobRegistry::dispose_all_results`): as an applied keyed one, unkeyed.
    /// Whether it stopped the current run.
    pub(crate) fn cancel_current(&self, pool: &MemoryPool) -> bool {
        Self::retire_locked(self.lock(), pool).0
    }

    /// Moves the epoch on by one (checked: at [`EPOCH_MAX`] the sizing becomes exhausted and the epoch stays), which
    /// retires every start that has not yet registered, and cancels the current run. Whether it stopped one, and the
    /// epoch and exhaustion after it, read under the same lock.
    fn retire_locked(mut state: MutexGuard<'_, RunsState>, pool: &MemoryPool) -> (bool, u64, bool) {
        if !state.exhausted {
            match state.epoch.checked_add(1).filter(|next| *next <= EPOCH_MAX) {
                Some(next) => {
                    state.epoch = next;
                    state.ticket_used = false;
                }
                None => state.exhausted = true,
            }
        }
        let (epoch, exhausted) = (state.epoch, state.exhausted);
        let current = state.current.take();
        drop(state);
        let cancelled = current.is_some_and(|(_, control)| control.cancel());
        if cancelled {
            pool.wake_waiters();
        }
        (cancelled, epoch, exhausted)
    }

    /// The ticket the current run registered under (tests).
    #[cfg(test)]
    pub(crate) fn current_ticket(&self) -> Option<u64> {
        self.lock().current.as_ref().map(|(epoch, _)| *epoch)
    }

    /// The current retirement epoch (tests; production learns it from `cancel_hanger_sizing`'s reply).
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
    pub(crate) fn entry_now(&self) -> SizingEntry {
        SizingEntry { epoch: self.epoch() }
    }

    /// The number of page tokens kept (tests).
    #[cfg(test)]
    pub(crate) fn applied_pages(&self) -> usize {
        self.lock().applied.len()
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
    /// A retirement stopped it (a newer request's, the person's cancel, the form's project session ending, the session
    /// reset).
    Cancelled,
    /// Its ticket has been passed by a retirement, or used by a run already: nothing was started, replaced or cancelled.
    Stale,
    /// Its ticket is missing, malformed or ahead of the host (what it is): nothing was started.
    TicketInvalid(&'static str),
    /// The sizing can be retired no further in this process: nothing was started.
    Exhausted,
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
            SizingFailure::Cancelled => format!(
                "{CANCELLED}: the sizing was retired before it replied (a newer request, a cancel, a project-session change or a session reset); its worker was stopped and nothing was published."
            ),
            SizingFailure::Stale => format!(
                "{REQUEST_STALE}: a retirement has passed this request's ticket, or a run has used it already; this delivery started, replaced and cancelled nothing."
            ),
            SizingFailure::TicketInvalid(what) => format!("{TICKET_INVALID}: the request {what}. Nothing was started."),
            SizingFailure::Exhausted => format!(
                "{EXHAUSTED}: hanger sizing cannot be retired any further in this process; restart the application to size again. Nothing was started."
            ),
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
        !state.cancelled
    }

    fn cancelled(&self) -> bool {
        self.control.cancelled()
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
    if control.cancelled() {
        return Err(SizingFailure::Cancelled);
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
    // 2. The queue wait for the starting grant, bounded, ended by a retirement.
    let deadline = Instant::now() + start_wait;
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
        Err(WaitOutcome::Cancelled) if control.cancelled() => return Err(SizingFailure::Cancelled),
        // The first attempt always precedes the deadline, so a reason was recorded.
        Err(WaitOutcome::Cancelled) => return Err(refused(registry, queued.unwrap_or(ResourceRefusal::HeadroomUnknown))),
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
    if control.cancelled() {
        // A reply that arrived as the run was stopped answers a retired request; it and its lease end here.
        drop(outcome);
        return Err(SizingFailure::Cancelled);
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

/// `size_hangers` after [`reserve`]: admission, the ticket ([`HangerSizingRuns::enter`], before the input is read),
/// the input text, registration as the current run (the ticket checked again), task 4 in the capped worker, and
/// `respond` with the reply text while the command's lease and the reply's are held.
pub(crate) fn size_command<R>(
    registry: &SolveJobRegistry,
    raw: Vec<u8>,
    copy: Lease,
    start_wait: Duration,
    respond: impl FnOnce(String) -> R,
) -> Result<R, CommandError<String>> {
    let (mut args, lease) = admit(registry, raw, copy)?;
    let runs = registry.hanger_sizing_runs();
    let entry = runs.enter(args.opt_value(EXPECTED_EPOCH)).map_err(SizingFailure::into_command_error)?;
    let input = match args.opt_value(INPUT_TEXT) {
        Some(Value::String(text)) => text,
        _ => {
            return Err(CommandError::Command(format!(
                "{ARGS_INVALID}: `{INPUT_TEXT}` must be the hanger-sizing input's JSON text. Nothing was started."
            )))
        }
    };
    drop(args);
    let registration = runs.register(entry, registry.pool()).map_err(SizingFailure::into_command_error)?;
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

/// `cancel_hanger_sizing`: `{retirementId}` as Tauri's JSON arguments (the one form that reaches the host unchanged on
/// both of Tauri's transports, as `cancel_mbf_export`'s; module docs, "The wire forms"), then the keyed retirement
/// ([`HangerSizingRuns::retire_keyed`]). A missing or malformed id is refused `HANGER-SIZING-RETIREMENT-ID-INVALID`
/// and retires nothing. Tauri has parsed the arguments before the command runs; the id is read from at most 32 hex
/// digits and one integer.
pub(crate) fn cancel_command(registry: &SolveJobRegistry, retirement_id: Option<Value>) -> Result<HangerSizingRetirement, String> {
    RetirementId::read(retirement_id).map(|id| registry.hanger_sizing_runs().retire_keyed(id, registry.pool()))
}

/// A command whose blocking part did not complete.
pub(crate) fn supervisor_failed() -> CommandError<String> {
    CommandError::Command(format!("{WORKER_FAILED}: the hanger sizing's supervisor did not complete."))
}

#[cfg(test)]
#[path = "hanger_sizing_job_tests.rs"]
mod tests;
