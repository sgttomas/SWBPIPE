//! The desktop's solve-job registry under the host memory pool (DEL-04-07
//! memory-budget decision).
//!
//! Ownership and accounting, in order (owner decision 2026-10-12, "(c)
//! Capped worker process"):
//! 1. Ingress: the raw body is prescanned without allocating
//!    ([`crate::ingress::prescan`]). The worker's starting grant (its process
//!    reserve plus the raw body's envelope and materialization bound, whole
//!    MiB) must fit the per-invocation budget; an ingress lease covers this
//!    process's copy of the raw bytes. This process never materializes the
//!    model.
//! 2. Queue: the raw bytes and their lease move into the job's supervisor
//!    thread, which waits (state `queued`, with the current reason visible to
//!    polls) until its starting grant is admitted by the pool with a fresh
//!    headroom verdict. Ordinary jobs run in parallel within the pool.
//! 3. Solve: the supervisor starts a local worker process
//!    ([`crate::solve_worker`]), sends the request and drops the raw bytes and
//!    their lease. The worker's allocator is capped at its grant, which grows
//!    on demand by atomic admissions up to the budget (`ordinary_worker_capped`);
//!    exceeding it ends the job `refused` with a typed refusal. Never a
//!    fallback to another mode or route.
//! 4. Result: the result lease (the resident envelope of the R result bytes)
//!    is admitted while the worker's grant is still held; the bytes are read
//!    into an exact buffer; the grant is released only after the worker has
//!    exited and been reaped. The result is retained (bounded) until taken. A
//!    start is refused early while retention is already full. A result whose
//!    announced length exceeds [`MAX_DISPLAYABLE_RESULT_BYTES`] is refused on
//!    that length (`RSLT {R}`) before any of this: its worker is killed, no
//!    result lease is admitted and no byte is read, and the job ends `refused`
//!    with [`RESULT_TOO_LARGE_CODE`], its size and the limit. Nothing of it
//!    can be taken, so it never crosses IPC.
//!    A retained result is one [`HeldResult`]: the bytes and their result
//!    lease, shared by `Arc` between the record and the native readers in
//!    flight ([`SolveJobRegistry::held_result`], C1 native custody), so the
//!    bytes and their lease are released together by the last holder.
//! 5. Handoff: taking the result grows its lease to 2R (handoff copy overlap),
//!    moves the bytes out without cloning, and keeps the 2R custody lease until
//!    the webview disposes its copy. A freshly loaded webview holds no copy and
//!    knows none of the previous page's jobs, so
//!    [`SolveJobRegistry::dispose_all_results`] then releases every retained
//!    result and custody lease and abandons every queued or running job: a
//!    running job's worker is killed, a queued job exits without starting one,
//!    and neither publishes a result. It never releases a live job's owners
//!    itself; the supervisor drops them after reaping its worker.
//!
//! Custody digests (W-11, DEL-08-01 Design `exact-route-report-sections.md`
//! §3.6): each job records the SHA-256 of the raw model body it admitted, hashed
//! in place before the body is copied, and of its result bytes, hashed in place
//! in the supervisor thread before publication, with the admitted solver mode.
//! One record field owns them ([`JobDigests`]): 65 bytes of digest metadata
//! (two 32-byte digests and the mode), from admission until the job ends
//! without a result (failed, refused or cancelled), or until its result's
//! custody ends (`dispose_result`, `dispose_all_results`), or until the
//! record is pruned, whichever comes first. No result or model bytes are
//! retained for it and no lease changes. [`SolveJobRegistry::custody_binding`]
//! returns them only while the result is delivered and its custody lease is
//! held. The native render and
//! save of an exact-route report bind resubmitted texts to them.
//!
//! Attestation table (W-18b (r2a), DEL-08-01 Design `report-user-release.md`
//! §2.4): separately from the custody digests, publication of a result whose
//! worker sent an attested `ATST` frame records `{job, solver mode, d_M, d_S}`
//! (the SHA-256 of the checked canonical model and result values, under their
//! user-release domains, [`crate::solve_attestation`]) in a bounded session
//! table of the [`crate::solve_attestation::MAX_ATTESTED_SOLVES`] (32) most recent attested solves, evicting
//! the oldest. It lives inline in the job table (a fixed ring under 8 KiB, no
//! lease), is not pruned with a record and not released with custody or by
//! `dispose_result`; [`SolveJobRegistry::dispose_all_results`] (page load)
//! clears it. It is never persisted and never sent to the webview:
//! [`SolveJobRegistry::holds_attested_solve`] answers only whether an entry
//! with given digests and mode is held, for the user-release route's admission.
//!
//! Locks: the registry lock may be held while taking the pool lock (lease
//! drop/grow) or a worker's control lock (kill); the pool never calls back
//! into the registry under its lock, and the supervisor never takes the
//! registry lock while holding a worker's control lock. Every owner drops
//! before a record shows a terminal state.

use crate::ingress;
use crate::memory_pool::{
    JobCounts, Lease, LeasePurpose, MemoryPool, PoolStatus, ResampleStop, ResourceRefusal, WaitOutcome, MAX_QUEUED_JOBS,
    MAX_RETAINED_RESULTS, MAX_TERMINAL_RECORDS, MAX_WEBVIEW_CUSTODY_RESULTS,
};
use crate::solve_attestation::{AttestationTable, SolveAttestation};
use crate::solve_worker::{
    CurrentExeLauncher, SupervisorHooks, WorkerControl, WorkerLauncher, WorkerOutcome, WorkerTask, WorkerTerms, PROCESS_RESERVE_BYTES,
};
use open_pipe_stress_product_physics::PreviewSolverMode;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Cancelling a running job kills its worker process (preemptive); a queued
/// job exits before starting one.
pub(crate) const SOLVE_JOB_CANCELLATION_SCOPE: &str = "worker_process_terminated";
/// A token-holder's cancellation of a running job: its worker is being killed;
/// the job turns `cancelled` (result discarded) only after the worker is reaped.
const CANCEL_REQUESTED_RUNNING: &str = "cancellation_requested_worker_terminating";
/// A token-holder's cancellation of a queued job (it exits before a worker starts).
const CANCEL_REQUESTED_QUEUED: &str = "cancellation_requested_awaiting_cooperative_checkpoint";
pub(crate) const SOLVER_MODE_HEADER: &str = "x-swbpipe-solver-mode";
/// The start header naming the result's custody ([`ResultCustody`]): absent
/// or `webview` for the full-envelope take, `native` for C1 native custody.
pub(crate) const RESULT_CUSTODY_HEADER: &str = "x-swbpipe-result-custody";
/// A take of a result held in native custody (C1): it never crosses IPC
/// whole; the native result commands read it.
pub(crate) const RESULT_NATIVE_CUSTODY_CODE: &str = "PREVIEW-SOLVE-RESULT-NATIVE-CUSTODY";
/// A C2 consumer's full copy of a held result above
/// [`MAX_DISPLAYABLE_RESULT_BYTES`] (C1 interim, DEL-04-05
/// `c1-native-custody-plan.md` slice 4).
pub(crate) const RESULT_DETAIL_WEBVIEW_LIMIT_CODE: &str = "RESULT-DETAIL-WEBVIEW-LIMIT";
/// A second full copy of a held result while the first is still held.
pub(crate) const RESULT_FULL_COPY_HELD_CODE: &str = "RESULT-DETAIL-FULL-COPY-HELD";

/// Who holds a solve job's published result (C1 native custody, DEL-04-05
/// `c1-native-custody-plan.md`). Chosen at start and fixed for the job.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub(crate) enum ResultCustody {
    /// The full-envelope take (every caller before C1): the result is
    /// bounded by [`MAX_DISPLAYABLE_RESULT_BYTES`] at publication and moves to
    /// the webview whole.
    #[default]
    Webview,
    /// C1: the result stays in this process ([`HeldResult`]) until disposal.
    /// No webview limit applies at publication: the worker's capped grant and
    /// the result lease's pool admission bound it, unchanged. It is never
    /// taken; the native readers, and a C2 consumer's bounded full copy
    /// ([`SolveJobRegistry::copy_result_within`]), read it.
    Native,
}

impl ResultCustody {
    /// The header's value: absent or `webview`, or `native`.
    pub(crate) fn from_wire(value: Option<&str>) -> Option<Self> {
        match value {
            None | Some("webview") => Some(Self::Webview),
            Some("native") => Some(Self::Native),
            Some(_) => None,
        }
    }
}
const ROUTE_PRICING: &str = "ordinary_worker_capped";
const CANCELLED_BEFORE_START: &str = "cancelled_before_solver_start";
const CANCELLED_AT_PUBLICATION: &str = "cancelled_before_result_publication_result_discarded";
/// A live job abandoned by [`SolveJobRegistry::dispose_all_results`] (page reload).
const ABANDONED_AWAITING: &str = "abandoned_by_session_reset_worker_terminating";
const ABANDONED_BEFORE_START: &str = "abandoned_by_session_reset_before_solver_start";
const ABANDONED_AT_PUBLICATION: &str = "abandoned_by_session_reset_result_discarded";

/// The largest solve result the desktop hands to its webview: 64 MiB.
///
/// Basis (RT-U4; DEL-04-05 `Work/criterion-9-measurement.md` §6): an
/// exclusive window on 2026-10-31 on the development host (Apple M5 Max,
/// 128 GiB, not the base machine), native-smoke release build `6c2f02d81`,
/// with injected linear-ex rack results. Each run reached the reader verdict;
/// the webview (WebContent) peak footprint was 3.22 GB at 39.1 MiB (79x),
/// 3.69 GB at 55.1 MiB (64x), 5.04 GB at 79.0 MiB (61x) and 8.04 GB at
/// 140.7 MiB (54x). Interpolated at 64 MiB: about 4.2 GB of WebContent, about
/// 17 s to the verdict (that window's times are possibly contended and are
/// re-taken) and an app peak under 0.5 GB, which beside the OS and
/// the app on a 16 GiB machine leaves roughly 1.8x headroom. That is an
/// engineering margin from one host, not a measurement on the base machine.
/// Results of 0.46 GB and more hung or crashed WebContent in an earlier window.
/// The value is fixed in the product, deliberately not derived from the
/// host's physical memory, and is re-measured after the desktop's result
/// reader fixes. A larger result is refused truthfully
/// ([`RESULT_TOO_LARGE_CODE`]) rather than sent to a webview that cannot
/// hold it.
pub(crate) const MAX_DISPLAYABLE_RESULT_BYTES: u64 = 64 << 20;
/// The code of a job refused because its result exceeds the displayable limit.
pub(crate) const RESULT_TOO_LARGE_CODE: &str = "PREVIEW-SOLVE-RESULT-TOO-LARGE";
/// The code of a stored result (a project's saved run or a recorded run)
/// withheld from the webview because it exceeds [`MAX_DISPLAYABLE_RESULT_BYTES`];
/// it is judged by its stored byte length before it is loaded, and the stored
/// bytes are never changed.
pub(crate) const STORED_RESULT_TOO_LARGE_CODE: &str = "STORED-RESULT-TOO-LARGE";

/// What a person can reduce or split today so a solve publishes less result
/// detail (a solve covers every load case and combination in the model); the
/// same words as the desktop's `SOLVE_RESULT_REDUCTION_OPTIONS` (`memoryPool.ts`).
pub(crate) const SOLVE_RESULT_REDUCTION_OPTIONS: &str = "remove the load cases and combinations this review does not need (Load Cases section); or solve the cases in smaller groups, each in its own copy of the model document (File > Save Model Document…), with the combinations in a copy holding only the cases they use; or split a large model into parts.";

/// A byte count for messages: exact, then in MiB or GiB (two decimals at most),
/// for example `67108865 bytes (64 MiB)`.
pub(crate) fn display_size(bytes: u64) -> String {
    const MIB: f64 = (1u64 << 20) as f64;
    const GIB: f64 = (1u64 << 30) as f64;
    let exact = bytes as f64;
    let (value, unit) = if exact >= GIB { (exact / GIB, "GiB") } else { (exact / MIB, "MiB") };
    let rounded = format!("{value:.2}");
    let rounded = rounded.trim_end_matches('0').trim_end_matches('.');
    format!("{bytes} bytes ({rounded} {unit})")
}

/// A result refused for its size: the bytes the worker announced and the
/// limit they exceed. The result itself was never read.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct ResultTooLarge {
    pub code: &'static str,
    pub result_bytes: u64,
    pub limit_bytes: u64,
}

impl ResultTooLarge {
    fn new(result_bytes: u64, limit_bytes: u64) -> Self {
        Self { code: RESULT_TOO_LARGE_CODE, result_bytes, limit_bytes }
    }

    fn message(&self, job_id: &str) -> String {
        format!(
            "{}: {job_id}'s result is {} bytes, above the {} bytes the desktop can display; it was discarded without being read",
            self.code, self.result_bytes, self.limit_bytes
        )
    }
}

/// Where a cancelled job stopped.
#[derive(Clone, Copy)]
enum CancelPoint {
    BeforeStart,
    AtPublication,
}

/// Starts a job's supervisor thread; on failure the job closure (and everything it owns) is dropped.
pub(crate) type SpawnFn = Arc<dyn Fn(Box<dyn FnOnce() + Send + 'static>) -> std::io::Result<()> + Send + Sync>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum JobState {
    Queued,
    Running,
    Completed,
    Cancelled,
    Failed,
    Refused,
}

impl JobState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Completed => "completed",
            Self::Cancelled => "cancelled",
            Self::Failed => "failed",
            Self::Refused => "refused",
        }
    }

    fn terminal(self) -> bool {
        !matches!(self, Self::Queued | Self::Running)
    }
}

/// The resource terms a job was admitted under.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct JobResource {
    /// The per-invocation budget at admission: the worker's maximum grant.
    pub invocation_budget_bytes: u64,
    pub pool_bytes: u64,
    pub route_pricing: &'static str,
    /// Bytes reserved by the ingress lease at admission (the envelope of this
    /// process's copy of the raw body, held until it is sent to the worker).
    pub ingress_reserved_bytes: u64,
    /// The worker's current grant while it runs; null before and after.
    pub worker_grant_bytes: Option<u64>,
    /// The largest grant the worker reached; null if no worker was admitted.
    pub worker_peak_grant_bytes: Option<u64>,
    /// The largest physical footprint measured for the worker (at growth
    /// exchanges, and its lifetime maximum at the result); null if none was
    /// measured. Above `worker_peak_grant_bytes` it shows the worker briefly
    /// held more than its grant between exchanges.
    pub worker_peak_footprint_bytes: Option<u64>,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct SolveJobStartReceipt {
    pub job_id: String,
    pub backend_cancellation_token: String,
    pub state: &'static str,
    pub cancellation_scope: &'static str,
    pub resource: JobResource,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct SolveJobStatusReport {
    pub job_id: String,
    pub state: &'static str,
    pub queue_reason: Option<ResourceRefusal>,
    pub cancellation_requested: bool,
    pub cancellation_status: String,
    pub cancellation_scope: &'static str,
    pub result_available: bool,
    pub result_bytes: Option<u64>,
    pub delivered: bool,
    pub error_message: Option<String>,
    pub resource_refusal: Option<ResourceRefusal>,
    /// A `refused` job whose result exceeded [`MAX_DISPLAYABLE_RESULT_BYTES`].
    pub result_too_large: Option<ResultTooLarge>,
    pub resource: JobResource,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct SolveJobCancellationReceipt {
    pub job_id: String,
    pub accepted: bool,
    pub cancellation_status: String,
    pub job_state: &'static str,
    pub cancellation_scope: &'static str,
    pub cancellation_success_claimed: bool,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct SolveResultDisposal {
    pub job_id: String,
    /// Whether a retained result or a custody lease was released.
    pub released: bool,
    pub released_bytes: u64,
    pub delivered: bool,
}

/// What [`SolveJobRegistry::dispose_all_results`] released and abandoned.
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub(crate) struct SolveResultsDisposal {
    /// Records whose retained result and/or custody lease was released.
    pub released_results: usize,
    pub released_bytes: u64,
    /// Queued or running jobs newly marked abandoned (cancellation requested);
    /// their leases are released when their workers drop them.
    pub abandoned_jobs: usize,
}

/// The typed error of the job and pool commands.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "code")]
pub(crate) enum SolveJobError {
    #[serde(rename = "RESOURCE_UNAVAILABLE")]
    ResourceUnavailable { refusal: ResourceRefusal, pool: Box<PoolStatus> },
    #[serde(rename = "INVALID_REQUEST")]
    InvalidRequest { message: String },
}

impl SolveJobError {
    fn invalid(message: impl Into<String>) -> Self {
        Self::InvalidRequest { message: message.into() }
    }

    #[cfg(test)]
    pub(crate) fn refusal(&self) -> Option<&ResourceRefusal> {
        match self {
            Self::ResourceUnavailable { refusal, .. } => Some(refusal),
            Self::InvalidRequest { .. } => None,
        }
    }
}

/// A job's custody digests (§3.6): the SHA-256 of the raw model body `start`
/// admitted, of the result bytes the job published, and the admitted mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct CustodyDigests {
    pub intake_sha256: [u8; 32],
    pub result_sha256: [u8; 32],
    pub solver_mode: PreviewSolverMode,
}

/// The SHA-256 of `bytes`, hashed in place: no allocation.
pub(crate) fn sha256_in_place(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}

/// The one owner of a job's digest metadata (§3.6): the intake digest and the
/// mode from admission, the result digest from publication.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct JobDigests {
    intake_sha256: [u8; 32],
    solver_mode: PreviewSolverMode,
    result_sha256: Option<[u8; 32]>,
}

#[derive(Debug)]
struct SolveJobRecord {
    number: u64,
    cancellation_token: String,
    state: JobState,
    cancellation_requested: bool,
    /// Cancellation requested by a session reset rather than by a token holder.
    abandoned: bool,
    cancellation_status: String,
    queue_reason: Option<ResourceRefusal>,
    error_message: Option<String>,
    resource_refusal: Option<ResourceRefusal>,
    /// Set when the job was refused for its result's size (never taken).
    result_too_large: Option<ResultTooLarge>,
    /// The producer's exact bytes and their result lease R while retained
    /// (not yet taken).
    result: Option<Arc<HeldResult>>,
    /// The 2R custody lease after delivery until disposal.
    result_lease: Option<Lease>,
    /// Who holds the result (fixed at start).
    custody: ResultCustody,
    /// A native-custody result's C2 full copy in the webview: the copy's
    /// lease, from the copy until its release or the result's disposal.
    full_copy_lease: Option<Lease>,
    result_bytes: Option<u64>,
    delivered: bool,
    resource: JobResource,
    /// Kill access to the job's worker process while it may be unreaped.
    worker: Option<Arc<WorkerControl>>,
    /// The job's digest metadata; released when the job ends without a
    /// result, or with the result and its custody lease.
    digests: Option<JobDigests>,
}

/// A retained result (C1 native custody, DEL-04-05 `c1-native-custody-plan.md`
/// slice 1): the producer's exact bytes and the result lease R covering
/// them. The record and each native reader in flight share it by `Arc`, so
/// a disposal while a reader holds it releases both only when that reader
/// drops it, and the pool never counts the bytes as released while they are
/// still held. Its row index (slice 3, [`crate::result_detail`]), built on
/// the first detail query with its own lease, lives and is released with it.
#[derive(Debug)]
pub(crate) struct HeldResult {
    text: String,
    lease: Lease,
    detail: Mutex<Option<Arc<crate::result_detail::DetailIndex>>>,
}

impl HeldResult {
    /// The producer's exact bytes.
    pub(crate) fn text(&self) -> &str {
        &self.text
    }

    /// The row index's slot: empty until the first detail query builds it.
    pub(crate) fn detail_slot(&self) -> &Mutex<Option<Arc<crate::result_detail::DetailIndex>>> {
        &self.detail
    }
}

#[derive(Debug, Default)]
struct SolveJobTable {
    next_job_number: u64,
    records: HashMap<String, SolveJobRecord>,
    /// The most recent attested solves (§2.4), apart from the records.
    attested: AttestationTable,
}

impl SolveJobTable {
    fn counts(&self) -> JobCounts {
        let mut counts = JobCounts::default();
        for record in self.records.values() {
            match record.state {
                JobState::Queued => counts.queued += 1,
                JobState::Running => counts.running += 1,
                _ => {}
            }
            if record.result.is_some() {
                counts.retained += 1;
            }
            if record.delivered && record.result_lease.is_some() {
                counts.custody += 1;
            }
            if record.full_copy_lease.is_some() {
                counts.custody += 1;
            }
        }
        counts
    }

    /// Drops the oldest terminal records holding no result and no lease.
    fn prune(&mut self) {
        let terminal = self.records.values().filter(|r| r.state.terminal()).count();
        if terminal <= MAX_TERMINAL_RECORDS {
            return;
        }
        let mut candidates: Vec<(u64, String)> = self
            .records
            .iter()
            .filter(|(_, r)| r.state.terminal() && r.result.is_none() && r.result_lease.is_none() && r.full_copy_lease.is_none())
            .map(|(id, r)| (r.number, id.clone()))
            .collect();
        candidates.sort_unstable();
        for (_, id) in candidates.into_iter().take(terminal - MAX_TERMINAL_RECORDS) {
            self.records.remove(&id);
        }
    }
}

/// An admitted request: the raw bytes, the lease covering this process's
/// copy of them and the worker's terms. Moves into the supervisor thread.
struct AdmittedInput {
    raw: Vec<u8>,
    lease: Lease,
    mode: PreviewSolverMode,
    start_grant: u64,
    max_grant: u64,
}

/// The admitted intake of an on-demand model check (`check_preview_model`):
/// the raw `{model}` args body copied under its intake lease, and the worker's
/// starting grant and cap.
pub(crate) struct ModelChecksInput {
    raw: Vec<u8>,
    lease: Lease,
    start_grant: u64,
    max_grant: u64,
    check: CheckRegistration,
}

/// The cancellation of one on-demand model check, as a solve job's record
/// holds it: a flag the supervisor and the start wait read, and kill access to
/// its worker while it may be unreaped.
#[derive(Debug, Default)]
struct CheckControl {
    state: Mutex<CheckState>,
}

#[derive(Debug, Default)]
struct CheckState {
    cancelled: bool,
    worker: Option<Arc<WorkerControl>>,
}

impl CheckControl {
    fn lock(&self) -> MutexGuard<'_, CheckState> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn cancelled(&self) -> bool {
        self.lock().cancelled
    }

    /// Requests cancellation and kills the worker if one is attached; its
    /// supervisor releases the grant after reaping it, as for a solve.
    fn cancel(&self) {
        let mut state = self.lock();
        state.cancelled = true;
        if let Some(worker) = &state.worker {
            worker.kill();
        }
    }
}

/// The registry's current check (at most one; a newer request supersedes the
/// older): dropping the registration clears the slot if it is still current.
pub(crate) struct CheckRegistration {
    slot: Arc<Mutex<Option<Arc<CheckControl>>>>,
    control: Arc<CheckControl>,
}

impl Drop for CheckRegistration {
    fn drop(&mut self) {
        let mut slot = self.slot.lock().unwrap_or_else(PoisonError::into_inner);
        if slot.as_ref().is_some_and(|current| Arc::ptr_eq(current, &self.control)) {
            *slot = None;
        }
    }
}

/// How long an on-demand model check waits for its worker's starting grant, as
/// a solve start waits in the queue (`MemoryPool::wait_acquire`): the same
/// window as the intake and result leases' headroom re-sampling. A check has
/// no job record to cancel, so the wait ends at this deadline, and the last
/// queue reason stands as the check's refusal.
pub(crate) const MODEL_CHECKS_START_WAIT: Duration = Duration::from_secs(2);

/// How an on-demand model check did not produce its report.
#[derive(Debug)]
pub(crate) enum ModelChecksError {
    /// A typed memory-pool refusal: at intake, at the worker's start grant, or
    /// while it ran (its growth, cap or footprint).
    Unavailable(SolveJobError),
    /// The worker failed (a crash, a protocol violation or its own error).
    Failed(String),
    /// Cancelled by a session reset, a project-session change or a newer
    /// check before it reported; its worker was killed and its grant released.
    Cancelled,
}

/// A completed model check: the report's bytes in their result lease, and the
/// worker's peak grant and peak measured footprint.
#[derive(Debug)]
pub(crate) struct ModelChecksOutput {
    pub report: Vec<u8>,
    pub lease: Lease,
    /// Read by the worker-figure regressions; the command sends the report only.
    #[cfg_attr(not(test), allow(dead_code))]
    pub peak_grant_bytes: u64,
    #[cfg_attr(not(test), allow(dead_code))]
    pub peak_footprint_bytes: u64,
}

/// The model check's supervisor view: its cancellation (as [`JobHooks`] reads
/// a job record's) and the worker's peak grant and footprint.
struct ModelChecksHooks {
    control: Arc<CheckControl>,
    peak_grant: u64,
    peak_footprint: u64,
}

impl SupervisorHooks for ModelChecksHooks {
    fn attach(&mut self, control: Arc<WorkerControl>) -> bool {
        let mut state = self.control.lock();
        state.worker = Some(control);
        !state.cancelled
    }
    fn cancelled(&self) -> bool {
        self.control.cancelled()
    }
    fn grant_changed(&mut self, grant_bytes: u64) {
        self.peak_grant = self.peak_grant.max(grant_bytes);
    }
    fn footprint_observed(&mut self, footprint_bytes: u64) {
        self.peak_footprint = self.peak_footprint.max(footprint_bytes);
    }
}

#[derive(Clone)]
pub(crate) struct SolveJobRegistry {
    jobs: Arc<Mutex<SolveJobTable>>,
    pool: Arc<MemoryPool>,
    launcher: Arc<dyn WorkerLauncher>,
    spawn: SpawnFn,
    /// The worker's process reserve (non-heap footprint allowance).
    reserve: u64,
    /// The on-demand model check in progress, if any.
    checks: Arc<Mutex<Option<Arc<CheckControl>>>>,
    /// The largest result a solve job may publish:
    /// [`MAX_DISPLAYABLE_RESULT_BYTES`], lowered only by tests.
    result_limit: u64,
}

fn thread_spawn(job: Box<dyn FnOnce() + Send + 'static>) -> std::io::Result<()> {
    std::thread::Builder::new().name("swbpipe-solve-job".into()).spawn(job).map(drop)
}

/// The supervisor's view into its job's record.
struct JobHooks<'a> {
    registry: &'a SolveJobRegistry,
    job_id: &'a str,
}

impl SupervisorHooks for JobHooks<'_> {
    fn attach(&mut self, control: Arc<WorkerControl>) -> bool {
        let mut table = self.registry.table();
        match table.records.get_mut(self.job_id) {
            Some(record) => {
                record.worker = Some(control);
                !record.cancellation_requested
            }
            None => false,
        }
    }

    fn cancelled(&self) -> bool {
        self.registry.cancellation_requested(self.job_id)
    }

    fn grant_changed(&mut self, grant_bytes: u64) {
        if let Some(record) = self.registry.table().records.get_mut(self.job_id) {
            record.resource.worker_grant_bytes = Some(grant_bytes);
            record.resource.worker_peak_grant_bytes =
                Some(record.resource.worker_peak_grant_bytes.unwrap_or(0).max(grant_bytes));
        }
    }

    fn footprint_observed(&mut self, footprint_bytes: u64) {
        if let Some(record) = self.registry.table().records.get_mut(self.job_id) {
            record.resource.worker_peak_footprint_bytes =
                Some(record.resource.worker_peak_footprint_bytes.unwrap_or(0).max(footprint_bytes));
        }
    }

    fn result_limit(&self) -> Option<u64> {
        // A native-custody job's result never reaches the webview whole: no
        // displayable limit (the worker's grant and the result lease bound it).
        match self.registry.table().records.get(self.job_id).map(|record| record.custody) {
            Some(ResultCustody::Native) => None,
            _ => Some(self.registry.result_limit),
        }
    }
}

/// Marks a worker's record failed if the worker ends without a terminal
/// state. Declared first in the worker so it drops after every other owner.
struct WorkerExit {
    registry: SolveJobRegistry,
    job_id: String,
}

impl Drop for WorkerExit {
    fn drop(&mut self) {
        let mut table = self.registry.table();
        if let Some(record) = table.records.get_mut(&self.job_id) {
            if !record.state.terminal() {
                record.state = JobState::Failed;
                record.digests = None;
                record.queue_reason = None;
                record.error_message =
                    Some("PREVIEW-SOLVE-WORKER-EXITED: the solve worker ended without publishing an outcome.".into());
            }
        }
    }
}

impl SolveJobRegistry {
    pub(crate) fn new(pool: Arc<MemoryPool>) -> Self {
        Self::with_hooks(pool, Arc::new(CurrentExeLauncher), Arc::new(thread_spawn))
    }

    pub(crate) fn with_hooks(pool: Arc<MemoryPool>, launcher: Arc<dyn WorkerLauncher>, spawn: SpawnFn) -> Self {
        Self {
            jobs: Arc::default(),
            pool,
            launcher,
            spawn,
            reserve: PROCESS_RESERVE_BYTES,
            checks: Arc::default(),
            result_limit: MAX_DISPLAYABLE_RESULT_BYTES,
        }
    }

    /// Test seam: the same registry with a lower result limit, so the limit's
    /// boundary can be exercised without results of [`MAX_DISPLAYABLE_RESULT_BYTES`].
    #[cfg(test)]
    pub(crate) fn with_result_limit(mut self, limit: u64) -> Self {
        self.result_limit = limit;
        self
    }


    #[cfg(test)]
    pub(crate) fn pool(&self) -> &Arc<MemoryPool> {
        &self.pool
    }

    fn table(&self) -> MutexGuard<'_, SolveJobTable> {
        // No code panics while holding this lock; recovering keeps workers'
        // terminal transitions working.
        self.jobs.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn unavailable_locked(&self, table: &SolveJobTable, refusal: ResourceRefusal) -> SolveJobError {
        self.pool.record_refusal(&refusal);
        SolveJobError::ResourceUnavailable { refusal, pool: Box::new(self.pool.status(table.counts())) }
    }

    fn unavailable(&self, refusal: ResourceRefusal) -> SolveJobError {
        let table = self.table();
        self.unavailable_locked(&table, refusal)
    }

    /// A typed resource refusal carrying the pool's status (also recorded as
    /// the pool's last refusal).
    pub(crate) fn refuse(&self, refusal: ResourceRefusal) -> SolveJobError {
        self.unavailable(refusal)
    }

    /// Admits an interactive command's ingress lease
    /// ([`crate::memory_pool::AdmissionClass::Interactive`]).
    pub(crate) fn acquire_interactive_ingress(&self, bytes: u64) -> Result<Lease, SolveJobError> {
        self.pool.acquire_interactive(bytes).map_err(|refusal| self.unavailable(refusal))
    }

    /// Admits a native reader's argument lease (C1 native custody):
    /// interactive, bounded with all its growth by the per-invocation budget
    /// and the 12 GiB default ([`MemoryPool::acquire_interactive_capped`]).
    pub(crate) fn acquire_interactive_ingress_capped(&self, bytes: u64) -> Result<Lease, SolveJobError> {
        self.pool.acquire_interactive_capped(bytes).map_err(|refusal| self.unavailable(refusal))
    }

    /// Admits a native reader's parse of a held result (C1 native custody):
    /// an adaptive lease of `bytes`, re-sampled while refused only for
    /// headroom, as the result and custody leases are, and refused
    /// `ingress_exceeds_budget` above the per-invocation budget or the
    /// 12 GiB default ([`MemoryPool::acquire_result_parse`]).
    pub(crate) fn acquire_result_parse(&self, bytes: u64) -> Result<Lease, SolveJobError> {
        match self.pool.acquire_result_parse(bytes, &|| false) {
            Ok(lease) => Ok(lease),
            Err(ResampleStop::Refused(refusal)) => Err(self.unavailable(refusal)),
            Err(ResampleStop::Cancelled) => unreachable!("a result parse is never cancelled"),
        }
    }

    pub(crate) fn pool_status(&self) -> PoolStatus {
        let counts = self.table().counts();
        self.pool.status(counts)
    }

    pub(crate) fn configure_pool(
        &self,
        pool_bytes: u64,
        invocation_budget_bytes: u64,
        acknowledge_above_recommended: bool,
    ) -> Result<PoolStatus, SolveJobError> {
        match self.pool.configure(pool_bytes, invocation_budget_bytes, acknowledge_above_recommended) {
            Ok(()) => Ok(self.pool_status()),
            Err(refusal) => Err(self.unavailable(refusal)),
        }
    }

    /// Admits a raw model document and starts its job. `body` is the raw
    /// invoke body (UTF-8 JSON of the model document).
    pub(crate) fn start(&self, body: &[u8], solver_mode: Option<&str>) -> Result<SolveJobStartReceipt, SolveJobError> {
        self.start_with_custody(body, solver_mode, ResultCustody::Webview)
    }

    /// [`Self::start`] with the result's custody ([`ResultCustody`]).
    pub(crate) fn start_with_custody(
        &self,
        body: &[u8],
        solver_mode: Option<&str>,
        custody: ResultCustody,
    ) -> Result<SolveJobStartReceipt, SolveJobError> {
        let mode = match solver_mode {
            None => PreviewSolverMode::default(),
            Some(value) => PreviewSolverMode::from_wire(value)
                .ok_or_else(|| SolveJobError::invalid(format!("unsupported preview solver mode: {value}")))?,
        };
        let bound = ingress::prescan(body).map_err(|invalid| {
            self.unavailable(ResourceRefusal::IngressInvalid {
                reason: format!("{} at byte {}", invalid.reason, invalid.offset),
            })
        })?;
        let (pool_bytes, budget) = self.pool.settings();
        // The worker must at least hold its process reserve, the raw bytes and their materialization.
        let start_grant = bound
            .lease_bytes()
            .and_then(|ingress| crate::solve_worker::initial_grant(self.reserve, ingress))
            .unwrap_or(u64::MAX);
        if start_grant > budget {
            return Err(self.unavailable(ResourceRefusal::IngressExceedsBudget { required_bytes: start_grant, budget_bytes: budget }));
        }
        {
            let table = self.table();
            let counts = table.counts();
            if counts.queued >= MAX_QUEUED_JOBS {
                return Err(self.unavailable_locked(&table, ResourceRefusal::QueueFull { queued: counts.queued, limit: MAX_QUEUED_JOBS }));
            }
            // A result could not be retained: refuse before reserving or solving.
            if counts.retained >= MAX_RETAINED_RESULTS {
                return Err(self.unavailable_locked(
                    &table,
                    ResourceRefusal::RetentionFull { retained: counts.retained, limit: MAX_RETAINED_RESULTS },
                ));
            }
        }
        // Hashed in place, before the body is copied for the worker (§3.6).
        let intake_sha256 = sha256_in_place(body);
        // The reservation precedes this process's copy of the raw bytes.
        let ingress_bytes = ingress::allocation_envelope(bound.raw_len).unwrap_or(u64::MAX);
        // No job exists yet, so nothing can cancel this wait.
        let lease = match self.pool.acquire_resampling(ingress_bytes, LeasePurpose::Ingress, &|| false) {
            Ok(lease) => lease,
            Err(ResampleStop::Refused(refusal)) => return Err(self.unavailable(refusal)),
            Err(ResampleStop::Cancelled) => unreachable!("the intake wait is never cancelled"),
        };
        let input = AdmittedInput { raw: body.to_vec(), lease, mode, start_grant, max_grant: budget };
        let resource = JobResource {
            invocation_budget_bytes: budget,
            pool_bytes,
            route_pricing: ROUTE_PRICING,
            ingress_reserved_bytes: ingress_bytes,
            worker_grant_bytes: None,
            worker_peak_grant_bytes: None,
            worker_peak_footprint_bytes: None,
        };

        let (job_id, token) = {
            let mut table = self.table();
            let queued = table.counts().queued;
            if queued >= MAX_QUEUED_JOBS {
                let error = self.unavailable_locked(&table, ResourceRefusal::QueueFull { queued, limit: MAX_QUEUED_JOBS });
                drop(table);
                drop(input);
                return Err(error);
            }
            table.next_job_number += 1;
            let number = table.next_job_number;
            let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or_default();
            let job_id = format!("backend-solve-job-{number}");
            let token = format!("solve-cancel-token-{number}-{nanos:x}");
            table.records.insert(
                job_id.clone(),
                SolveJobRecord {
                    number,
                    cancellation_token: token.clone(),
                    state: JobState::Queued,
                    cancellation_requested: false,
                    abandoned: false,
                    cancellation_status: "not_requested".into(),
                    queue_reason: None,
                    error_message: None,
                    resource_refusal: None,
                    result_too_large: None,
                    result: None,
                    result_lease: None,
                    custody,
                    full_copy_lease: None,
                    result_bytes: None,
                    delivered: false,
                    resource: resource.clone(),
                    worker: None,
                    digests: Some(JobDigests { intake_sha256, solver_mode: mode, result_sha256: None }),
                },
            );
            (job_id, token)
        };

        let worker = {
            let registry = self.clone();
            let job_id = job_id.clone();
            move || registry.run_job(job_id, input)
        };
        let mut state = JobState::Queued;
        if let Err(error) = (self.spawn)(Box::new(worker)) {
            // The closure, its input and its lease have been dropped.
            let mut table = self.table();
            if let Some(record) = table.records.get_mut(&job_id) {
                record.state = JobState::Failed;
                record.digests = None;
                record.error_message = Some(format!("PREVIEW-SOLVE-WORKER-START-FAILED: {error}"));
            }
            table.prune();
            state = JobState::Failed;
        }
        Ok(SolveJobStartReceipt {
            job_id,
            backend_cancellation_token: token,
            state: state.as_str(),
            cancellation_scope: SOLVE_JOB_CANCELLATION_SCOPE,
            resource,
        })
    }

    /// Admits an on-demand model check's raw `{model}` args body as `start`
    /// admits a solve (DEL-04-07 memory-budget decision, "On-demand model
    /// checks"): the prescan, the worker's starting grant (process reserve plus
    /// the intake bound) within the per-invocation budget, and the intake lease
    /// before this process copies the bytes. A new check supersedes the one in
    /// progress: that one is cancelled first, so its grant is released.
    pub(crate) fn admit_model_checks(&self, body: &[u8]) -> Result<ModelChecksInput, SolveJobError> {
        let check = self.register_model_check();
        let bound = ingress::prescan(body).map_err(|invalid| {
            self.unavailable(ResourceRefusal::IngressInvalid { reason: format!("{} at byte {}", invalid.reason, invalid.offset) })
        })?;
        let (_, budget) = self.pool.settings();
        let start_grant = bound
            .lease_bytes()
            .and_then(|ingress| crate::solve_worker::initial_grant(self.reserve, ingress))
            .unwrap_or(u64::MAX);
        if start_grant > budget {
            return Err(self.unavailable(ResourceRefusal::IngressExceedsBudget { required_bytes: start_grant, budget_bytes: budget }));
        }
        let ingress_bytes = ingress::allocation_envelope(bound.raw_len).unwrap_or(u64::MAX);
        let lease = match self.pool.acquire_resampling(ingress_bytes, LeasePurpose::Ingress, &|| false) {
            Ok(lease) => lease,
            Err(ResampleStop::Refused(refusal)) => return Err(self.unavailable(refusal)),
            Err(ResampleStop::Cancelled) => unreachable!("the intake wait is never cancelled"),
        };
        Ok(ModelChecksInput { raw: body.to_vec(), lease, start_grant, max_grant: budget, check })
    }

    /// Makes a new check the current one, cancelling the one it supersedes.
    fn register_model_check(&self) -> CheckRegistration {
        let control = Arc::new(CheckControl::default());
        let previous = self.checks.lock().unwrap_or_else(PoisonError::into_inner).replace(control.clone());
        if let Some(previous) = previous {
            previous.cancel();
            self.pool.wake_waiters();
        }
        CheckRegistration { slot: self.checks.clone(), control }
    }

    /// Cancels the model check in progress, if any (`cancel_model_checks`, on
    /// a project-session change, and session reset): a check still waiting for
    /// its starting grant stops waiting, a running check's worker is killed,
    /// and its supervisor releases the grant after reaping it. Returns whether
    /// a check was in progress.
    pub(crate) fn cancel_model_checks(&self) -> bool {
        let current = self.checks.lock().unwrap_or_else(PoisonError::into_inner).take();
        let Some(current) = current else { return false };
        current.cancel();
        self.pool.wake_waiters();
        true
    }

    /// Runs an admitted model check in a capped worker process, as an ordinary
    /// solve runs: its starting grant is a worker lease taken by the solve's own
    /// queue wait (`wait_acquire`: the worker count limit, the pool and a fresh
    /// headroom verdict at each attempt), for at most [`MODEL_CHECKS_START_WAIT`],
    /// after which the last queue reason stands as its typed refusal; its grant
    /// grows on demand up to the budget the check was admitted under
    /// (`max_grant`, as a queued solve's worker), and
    /// its report's bytes come back in a result lease, which the caller holds
    /// until it has sent them.
    pub(crate) fn run_model_checks(&self, input: ModelChecksInput) -> Result<ModelChecksOutput, ModelChecksError> {
        let ModelChecksInput { raw, lease, start_grant, max_grant, check } = input;
        let control = check.control.clone();
        let deadline = Instant::now() + MODEL_CHECKS_START_WAIT;
        let mut queued: Option<ResourceRefusal> = None;
        // As a solve's worker: bounded by the budget the check was admitted
        // under (`max_grant`), not the budget when it leaves the queue.
        let waited = self.pool.wait_acquire(
            start_grant,
            LeasePurpose::Worker,
            Some(max_grant),
            &|| control.cancelled() || Instant::now() >= deadline,
            &mut |reason| queued = Some(reason.clone()),
        );
        let mut grant = match waited {
            Ok(grant) => grant,
            Err(WaitOutcome::Cancelled) if control.cancelled() => {
                drop((raw, lease));
                return Err(ModelChecksError::Cancelled);
            }
            Err(WaitOutcome::Cancelled) => {
                drop((raw, lease));
                // The first attempt always precedes the deadline, so a reason was recorded.
                let refusal = queued.unwrap_or(ResourceRefusal::HeadroomUnknown);
                return Err(ModelChecksError::Unavailable(self.unavailable(refusal)));
            }
        };
        let terms = WorkerTerms { pool: &self.pool, launcher: &*self.launcher, task: WorkerTask::ModelChecks, max_grant, reserve: self.reserve };
        let mut hooks = ModelChecksHooks { control: control.clone(), peak_grant: grant.bytes(), peak_footprint: 0 };
        // The worker is reaped inside `supervise`, also while it unwinds; only
        // then is the grant released.
        let outcome = catch_unwind(AssertUnwindSafe(|| crate::solve_worker::supervise(&terms, &mut grant, (raw, lease), &mut hooks)));
        control.lock().worker = None;
        drop(grant);
        drop(check);
        if control.cancelled() {
            // A report that arrived as it was cancelled answers a retired
            // request; it and its result lease are released here.
            drop(outcome);
            return Err(ModelChecksError::Cancelled);
        }
        match outcome {
            Err(payload) => Err(ModelChecksError::Failed(format!("MODEL-CHECKS-PANICKED: {}", crate::solve_worker::panic_message(payload.as_ref())))),
            Ok(WorkerOutcome::Completed { result, lease, .. }) => Ok(ModelChecksOutput {
                report: result,
                lease,
                peak_grant_bytes: hooks.peak_grant,
                peak_footprint_bytes: hooks.peak_footprint,
            }),
            Ok(WorkerOutcome::Failed(error)) => Err(ModelChecksError::Failed(error)),
            Ok(WorkerOutcome::Refused(refusal)) => Err(ModelChecksError::Unavailable(self.unavailable(refusal))),
            // The check's hooks set no result limit, so this does not arise.
            Ok(WorkerOutcome::ResultTooLarge { result_bytes, limit_bytes }) => Err(ModelChecksError::Failed(format!(
                "MODEL-CHECKS-REPORT-TOO-LARGE: a report of {result_bytes} bytes above {limit_bytes} bytes"
            ))),
            Ok(WorkerOutcome::Terminated) => Err(ModelChecksError::Failed("MODEL-CHECKS-WORKER-TERMINATED: the check's worker was stopped.".to_string())),
        }
    }

    fn cancellation_requested(&self, job_id: &str) -> bool {
        self.table().records.get(job_id).is_none_or(|r| r.cancellation_requested)
    }

    fn set_queue_reason(&self, job_id: &str, reason: &ResourceRefusal) {
        if let Some(record) = self.table().records.get_mut(job_id) {
            if record.queue_reason.as_ref() != Some(reason) {
                record.queue_reason = Some(reason.clone());
            }
        }
    }

    /// Queued -> running under a grant of `grant_bytes`, unless cancellation was requested.
    fn mark_running(&self, job_id: &str, grant_bytes: u64) -> bool {
        let mut table = self.table();
        match table.records.get_mut(job_id) {
            Some(record) if !record.cancellation_requested => {
                record.state = JobState::Running;
                record.queue_reason = None;
                record.resource.worker_grant_bytes = Some(grant_bytes);
                record.resource.worker_peak_grant_bytes = Some(grant_bytes);
                true
            }
            _ => false,
        }
    }

    /// Records a terminal state; callers have already reaped the worker and
    /// dropped its grant and every other owner.
    fn finish(&self, job_id: &str, edit: impl FnOnce(&mut SolveJobRecord)) {
        let mut table = self.table();
        if let Some(record) = table.records.get_mut(job_id) {
            record.queue_reason = None;
            record.worker = None;
            record.resource.worker_grant_bytes = None;
            edit(record);
            // A job that ends without a result releases its digest metadata.
            if record.result.is_none() && !record.delivered {
                record.digests = None;
            }
        }
        table.prune();
    }

    fn finish_cancelled(&self, job_id: &str, point: CancelPoint, error: Option<String>) {
        self.finish(job_id, |record| {
            let status = match (point, record.abandoned) {
                (CancelPoint::BeforeStart, false) => CANCELLED_BEFORE_START,
                (CancelPoint::AtPublication, false) => CANCELLED_AT_PUBLICATION,
                (CancelPoint::BeforeStart, true) => ABANDONED_BEFORE_START,
                (CancelPoint::AtPublication, true) => ABANDONED_AT_PUBLICATION,
            };
            record.state = JobState::Cancelled;
            record.cancellation_status = status.to_string();
            record.error_message = error;
        });
    }

    fn run_job(self, job_id: String, input: AdmittedInput) {
        let _exit = WorkerExit { registry: self.clone(), job_id: job_id.clone() };
        let AdmittedInput { raw, lease, mode, start_grant, max_grant } = input;
        let mut input = Some((raw, lease));
        // The worker is bounded by the budget its job was admitted under
        // (`max_grant`), not the budget at the time it leaves the queue.
        let grant = self.pool.wait_acquire(
            start_grant,
            LeasePurpose::Worker,
            Some(max_grant),
            &|| self.cancellation_requested(&job_id),
            &mut |reason| self.set_queue_reason(&job_id, reason),
        );
        let mut grant = match grant {
            Ok(grant) => grant,
            Err(WaitOutcome::Cancelled) => {
                drop(input.take());
                self.finish_cancelled(&job_id, CancelPoint::BeforeStart, None);
                return;
            }
        };
        if !self.mark_running(&job_id, grant.bytes()) {
            drop(input.take());
            drop(grant);
            self.finish_cancelled(&job_id, CancelPoint::BeforeStart, None);
            return;
        }
        let Some(input) = input.take() else { return };
        let terms = WorkerTerms { pool: &self.pool, launcher: &*self.launcher, task: WorkerTask::Solve(mode), max_grant, reserve: self.reserve };
        // The worker is reaped inside `supervise`, also while it unwinds; only
        // then is the grant released.
        let outcome = catch_unwind(AssertUnwindSafe(|| {
            // Native smoke builds only: an injected result instead of the worker
            // (RT-U4 desktop measurement; `native_smoke_inject.rs`).
            #[cfg(feature = "native-smoke")]
            if let Some(outcome) = crate::native_smoke_inject::injected_outcome(&self.pool, &input.0, &|| self.cancellation_requested(&job_id)) {
                return outcome;
            }
            let mut hooks = JobHooks { registry: &self, job_id: &job_id };
            crate::solve_worker::supervise(&terms, &mut grant, input, &mut hooks)
        }));
        drop(grant);
        match outcome {
            Err(payload) => {
                let message = crate::solve_worker::panic_message(payload.as_ref());
                drop(payload);
                self.finish(&job_id, |record| {
                    record.state = JobState::Failed;
                    record.error_message = Some(format!("PREVIEW-SOLVE-PANICKED: {message}"));
                });
            }
            Ok(WorkerOutcome::Completed { result, lease, attestation }) => match String::from_utf8(result) {
                Ok(result) => {
                    // Hashed in place, before publication takes the registry lock (§3.6).
                    let result_sha256 = sha256_in_place(result.as_bytes());
                    self.publish(&job_id, result, lease, result_sha256, attestation.map(|a| (mode, a)))
                }
                Err(_) => {
                    drop(lease);
                    self.finish(&job_id, |record| {
                        record.state = JobState::Failed;
                        record.error_message = Some("SOLVE_WORKER_PROTOCOL: the result is not UTF-8".into());
                    });
                }
            },
            Ok(WorkerOutcome::Failed(error)) => {
                if self.cancellation_requested(&job_id) {
                    self.finish_cancelled(&job_id, CancelPoint::AtPublication, Some(error));
                } else {
                    self.finish(&job_id, |record| {
                        record.state = JobState::Failed;
                        record.error_message = Some(error);
                    });
                }
            }
            Ok(WorkerOutcome::Refused(refusal)) => {
                self.pool.record_refusal(&refusal);
                self.finish(&job_id, |record| {
                    record.state = JobState::Refused;
                    record.resource_refusal = Some(refusal);
                });
            }
            Ok(WorkerOutcome::ResultTooLarge { result_bytes, limit_bytes }) => {
                self.finish_too_large(&job_id, ResultTooLarge::new(result_bytes, limit_bytes));
            }
            Ok(WorkerOutcome::Terminated) => self.finish_cancelled(&job_id, CancelPoint::AtPublication, None),
        }
    }

    /// Ends a job refused for its result's size. Its result and result lease
    /// (if any were ever held) have already been dropped; it holds none, so
    /// it can never be taken.
    fn finish_too_large(&self, job_id: &str, refusal: ResultTooLarge) {
        self.finish(job_id, |record| {
            record.state = JobState::Refused;
            record.error_message = Some(refusal.message(job_id));
            record.result_too_large = Some(refusal);
        });
    }

    /// Retains a finished result under its result lease, unless the job was
    /// cancelled or retention is full (both drop it). A retained result's
    /// attestation, if its worker sent one, enters the attestation table.
    fn publish(
        &self,
        job_id: &str,
        result: String,
        lease: Lease,
        result_sha256: [u8; 32],
        attestation: Option<(PreviewSolverMode, SolveAttestation)>,
    ) {
        let mut table = self.table();
        let retained = table.counts().retained;
        let Some(record) = table.records.get_mut(job_id) else { return };
        if record.cancellation_requested {
            drop(table);
            drop(result);
            drop(lease);
            self.finish_cancelled(job_id, CancelPoint::AtPublication, None);
            return;
        }
        // The worker's announced length is refused before it is read; this
        // holds the same bound for any result that reaches publication. A
        // native-custody result has no displayable limit.
        if record.custody == ResultCustody::Webview && result.len() as u64 > self.result_limit {
            drop(table);
            let result_bytes = result.len() as u64;
            drop(result);
            drop(lease);
            self.finish_too_large(job_id, ResultTooLarge::new(result_bytes, self.result_limit));
            return;
        }
        if retained >= MAX_RETAINED_RESULTS {
            drop(table);
            drop(result);
            drop(lease);
            let refusal = ResourceRefusal::RetentionFull { retained, limit: MAX_RETAINED_RESULTS };
            self.pool.record_refusal(&refusal);
            self.finish(job_id, |record| {
                record.state = JobState::Refused;
                record.resource_refusal = Some(refusal);
            });
            return;
        }
        record.state = JobState::Completed;
        record.queue_reason = None;
        record.error_message = None;
        record.worker = None;
        record.resource.worker_grant_bytes = None;
        record.result_bytes = Some(result.len() as u64);
        record.result = Some(Arc::new(HeldResult { text: result, lease, detail: Mutex::new(None) }));
        if let Some(digests) = record.digests.as_mut() {
            digests.result_sha256 = Some(result_sha256);
        }
        let number = record.number;
        if let Some((mode, attestation)) = attestation {
            table.attested.record(number, mode, attestation);
        }
        table.prune();
    }

    /// Whether this session's solver produced, in `solver_mode`, an attested
    /// solve with these digests (`d_M`, `d_S`) among the
    /// [`crate::solve_attestation::MAX_ATTESTED_SOLVES`] most recent since the last session reset. For
    /// the user-release route's native admission (DEL-08-01 Design
    /// `report-user-release.md` §2.5 step 7), which computes the digests with
    /// [`SolveAttestation::of`]. Returns no digest and no job.
    pub(crate) fn holds_attested_solve(&self, solver_mode: PreviewSolverMode, attestation: &SolveAttestation) -> bool {
        self.table().attested.holds(solver_mode, attestation)
    }

    /// Records an attested solve as publication does (route 010's admission
    /// tests, which have no worker).
    #[cfg(test)]
    pub(crate) fn record_attested_solve_for_tests(&self, job_number: u64, solver_mode: PreviewSolverMode, attestation: SolveAttestation) {
        self.table().attested.record(job_number, solver_mode, attestation);
    }

    /// The job's custody digests (§3.6), a copy, only while its result is
    /// delivered and its custody lease is held: none for an unknown job, an
    /// untaken (retained) result, or after disposal or pruning.
    pub(crate) fn custody_binding(&self, job_id: &str) -> Option<CustodyDigests> {
        let table = self.table();
        let record = table.records.get(job_id)?;
        if !(record.delivered && record.result.is_none() && record.result_lease.is_some()) {
            return None;
        }
        let digests = record.digests?;
        Some(CustodyDigests { intake_sha256: digests.intake_sha256, result_sha256: digests.result_sha256?, solver_mode: digests.solver_mode })
    }

    /// The job's state; never returns or clones the result.
    pub(crate) fn status(&self, job_id: &str) -> Result<SolveJobStatusReport, SolveJobError> {
        let table = self.table();
        let record = table.records.get(job_id).ok_or_else(|| SolveJobError::invalid(format!("unknown solve job: {job_id}")))?;
        Ok(SolveJobStatusReport {
            job_id: job_id.to_string(),
            state: record.state.as_str(),
            queue_reason: record.queue_reason.clone(),
            cancellation_requested: record.cancellation_requested,
            cancellation_status: record.cancellation_status.clone(),
            cancellation_scope: SOLVE_JOB_CANCELLATION_SCOPE,
            result_available: record.result.is_some(),
            result_bytes: record.result_bytes,
            delivered: record.delivered,
            error_message: record.error_message.clone(),
            resource_refusal: record.resource_refusal.clone(),
            result_too_large: record.result_too_large,
            resource: record.resource.clone(),
        })
    }

    /// Hands the producer's exact bytes over (moved, not cloned) after growing
    /// the result lease to 2R for the handoff copy; on refusal the result stays.
    /// The growth R is admitted as its own (uncredited ingress) lease outside the job table's lock,
    /// re-sampling while it is refused only with `headroom_insufficient`
    /// (`memory_pool.rs`, Re-sampling), then the record is checked again and the
    /// growth folded into its lease; a record that changed meanwhile (taken,
    /// disposed, reset or custody full) releases the growth.
    pub(crate) fn take_result(&self, job_id: &str) -> Result<Vec<u8>, SolveJobError> {
        let growth = {
            let table = self.table();
            let lease = self.takeable_locked(&table, job_id)?;
            lease.bytes()
        };
        // Held as an ingress lease, which no admission credits as resident,
        // until it is folded into custody: the webview's copy is not written
        // until the take returns, so no admission may count half of it yet.
        let extra = match self.pool.acquire_resampling(growth, LeasePurpose::Ingress, &|| false) {
            Ok(extra) => extra,
            Err(ResampleStop::Refused(refusal)) => return Err(self.unavailable(refusal)),
            Err(ResampleStop::Cancelled) => unreachable!("the take is never cancelled"),
        };
        let mut table = self.table();
        if self.takeable_locked(&table, job_id)?.bytes() != growth {
            return Err(SolveJobError::invalid(format!("PREVIEW-SOLVE-RESULT-UNAVAILABLE: {job_id} changed while its take was admitted")));
        }
        let record = table.records.get_mut(job_id).expect("checked above");
        // Readers take their share only under this lock, so a sole holder
        // here stays the sole holder.
        let held = record.result.take().expect("checked above");
        // The row index, if built, leaves native custody with the bytes; its lease is released here.
        let HeldResult { text, mut lease, detail: _ } = match Arc::try_unwrap(held) {
            Ok(held) => held,
            Err(held) => {
                record.result = Some(held);
                return Err(SolveJobError::invalid(format!("PREVIEW-SOLVE-RESULT-BUSY: {job_id} is being read natively")));
            }
        };
        lease.set_purpose(LeasePurpose::Custody);
        lease.absorb(extra);
        record.result_lease = Some(lease);
        record.delivered = true;
        Ok(text.into_bytes())
    }

    /// C1 native custody (slice 1): a share of a retained result's bytes and
    /// their lease, for a native reader, without copying them. The result
    /// stays retained and takeable; a take while a share is held is refused
    /// `PREVIEW-SOLVE-RESULT-BUSY`, and a disposal while a share is held
    /// releases the bytes and their lease when the share is dropped.
    pub(crate) fn held_result(&self, job_id: &str) -> Result<Arc<HeldResult>, SolveJobError> {
        let table = self.table();
        let record = table.records.get(job_id).ok_or_else(|| SolveJobError::invalid(format!("unknown solve job: {job_id}")))?;
        match &record.result {
            Some(held) => Ok(Arc::clone(held)),
            None if record.delivered => Err(SolveJobError::invalid(format!("PREVIEW-SOLVE-RESULT-ALREADY-DELIVERED: {job_id}"))),
            None => Err(SolveJobError::invalid(format!("PREVIEW-SOLVE-RESULT-UNAVAILABLE: {job_id} is {}", record.state.as_str()))),
        }
    }

    /// The custody a job's result was started under; `None` for an unknown job.
    pub(crate) fn result_custody(&self, job_id: &str) -> Option<ResultCustody> {
        self.table().records.get(job_id).map(|record| record.custody)
    }

    /// C1 interim (DEL-04-05 `c1-native-custody-plan.md` slice 4): a
    /// native-custody result's exact bytes copied whole for a C2 consumer
    /// (reports, comparisons, rule checks, derivative exports, handoffs), only
    /// at or under `limit` (PR #51's [`MAX_DISPLAYABLE_RESULT_BYTES`]). Above
    /// it the copy is refused by name ([`RESULT_DETAIL_WEBVIEW_LIMIT_CODE`],
    /// with the size and the limit) before anything is reserved or copied.
    /// The copy's lease, the size of the result lease R (the webview's copy, as
    /// a take's growth), is admitted before the copy (re-sampling only while
    /// refused for headroom) and held as custody until
    /// [`Self::release_full_copy`] or the result's disposal or session reset;
    /// one copy per job at a time ([`RESULT_FULL_COPY_HELD_CODE`]). The held
    /// result stays in native custody. Webview-custody jobs are taken, not copied.
    pub(crate) fn copy_result_within(&self, job_id: &str, limit: u64) -> Result<Vec<u8>, SolveJobError> {
        let (held, growth) = {
            let table = self.table();
            let held = self.copyable_locked(&table, job_id, limit)?;
            let growth = held.lease.bytes();
            (held, growth)
        };
        let lease = match self.pool.acquire_resampling(growth, LeasePurpose::Ingress, &|| false) {
            Ok(lease) => lease,
            Err(ResampleStop::Refused(refusal)) => return Err(self.unavailable(refusal)),
            Err(ResampleStop::Cancelled) => unreachable!("a full copy is never cancelled"),
        };
        // Copied while this share keeps the bytes alive, outside the registry lock.
        let copy = held.text().as_bytes().to_vec();
        drop(held);
        let mut table = self.table();
        // A record that changed meanwhile (disposed, reset, or another copy) releases the copy.
        self.copyable_locked(&table, job_id, limit)?;
        let record = table.records.get_mut(job_id).expect("checked above");
        let mut lease = lease;
        lease.set_purpose(LeasePurpose::Custody);
        record.full_copy_lease = Some(lease);
        Ok(copy)
    }

    /// The held result of a native-custody job whose full copy may be made now.
    fn copyable_locked(&self, table: &SolveJobTable, job_id: &str, limit: u64) -> Result<Arc<HeldResult>, SolveJobError> {
        let custody = table.counts().custody;
        let record = table.records.get(job_id).ok_or_else(|| SolveJobError::invalid(format!("unknown solve job: {job_id}")))?;
        if record.custody != ResultCustody::Native {
            return Err(SolveJobError::invalid(format!("PREVIEW-SOLVE-RESULT-NOT-NATIVE: {job_id} is a webview-custody result; take it")));
        }
        let Some(held) = &record.result else {
            return Err(SolveJobError::invalid(format!("PREVIEW-SOLVE-RESULT-UNAVAILABLE: {job_id} is {}", record.state.as_str())));
        };
        let bytes = held.text().len() as u64;
        if bytes > limit {
            return Err(SolveJobError::invalid(format!(
                "{RESULT_DETAIL_WEBVIEW_LIMIT_CODE} (result_bytes={bytes}, limit_bytes={limit}): {job_id}'s result is {}, above the {} the webview can hold whole; nothing was copied, and the result stays held natively.",
                display_size(bytes),
                display_size(limit)
            )));
        }
        if record.full_copy_lease.is_some() {
            return Err(SolveJobError::invalid(format!("{RESULT_FULL_COPY_HELD_CODE}: {job_id}'s full copy is already held; release it first.")));
        }
        if custody >= MAX_WEBVIEW_CUSTODY_RESULTS {
            return Err(self.unavailable_locked(table, ResourceRefusal::CustodyFull { delivered: custody, limit: MAX_WEBVIEW_CUSTODY_RESULTS }));
        }
        Ok(Arc::clone(held))
    }

    /// The webview dropped its C2 full copy: releases the copy's lease (the
    /// held result stays). Whether a copy was held.
    pub(crate) fn release_full_copy(&self, job_id: &str) -> Result<bool, SolveJobError> {
        let mut table = self.table();
        let record = table.records.get_mut(job_id).ok_or_else(|| SolveJobError::invalid(format!("unknown solve job: {job_id}")))?;
        let copy = record.full_copy_lease.take();
        let released = copy.is_some();
        drop(copy);
        table.prune();
        Ok(released)
    }

    /// The retained result's lease, if `job_id` can be taken now.
    fn takeable_locked<'t>(&self, table: &'t SolveJobTable, job_id: &str) -> Result<&'t Lease, SolveJobError> {
        let custody = table.counts().custody;
        let record = table.records.get(job_id).ok_or_else(|| SolveJobError::invalid(format!("unknown solve job: {job_id}")))?;
        if let Some(refusal) = record.result_too_large {
            return Err(SolveJobError::invalid(refusal.message(job_id)));
        }
        if record.custody == ResultCustody::Native {
            return Err(SolveJobError::invalid(format!(
                "{RESULT_NATIVE_CUSTODY_CODE}: {job_id} is held natively; read it through the native result commands."
            )));
        }
        if record.result.is_none() {
            let message = if record.delivered {
                format!("PREVIEW-SOLVE-RESULT-ALREADY-DELIVERED: {job_id}")
            } else {
                format!("PREVIEW-SOLVE-RESULT-UNAVAILABLE: {job_id} is {}", record.state.as_str())
            };
            return Err(SolveJobError::invalid(message));
        }
        if custody >= MAX_WEBVIEW_CUSTODY_RESULTS {
            return Err(self.unavailable_locked(
                table,
                ResourceRefusal::CustodyFull { delivered: custody, limit: MAX_WEBVIEW_CUSTODY_RESULTS },
            ));
        }
        Ok(&record.result.as_ref().expect("checked above").lease)
    }

    /// The webview's acknowledgement that it dropped its copy, or disposal of
    /// a retained result never taken. Releases the job's result/custody lease.
    pub(crate) fn dispose_result(&self, job_id: &str) -> Result<SolveResultDisposal, SolveJobError> {
        let mut table = self.table();
        let record = table.records.get_mut(job_id).ok_or_else(|| SolveJobError::invalid(format!("unknown solve job: {job_id}")))?;
        let result = record.result.take();
        let lease = record.result_lease.take();
        let full_copy = record.full_copy_lease.take();
        // The digests are released with custody.
        record.digests = None;
        // A retained result's bytes and lease are released here, or when the
        // last native reader holding them drops its share.
        let released_bytes = lease.as_ref().or(result.as_ref().map(|held| &held.lease)).map_or(0, Lease::bytes)
            + full_copy.as_ref().map_or(0, Lease::bytes);
        let receipt = SolveResultDisposal {
            job_id: job_id.to_string(),
            released: result.is_some() || lease.is_some() || full_copy.is_some(),
            released_bytes,
            delivered: record.delivered,
        };
        drop(result);
        drop(lease);
        drop(full_copy);
        table.prune();
        Ok(receipt)
    }

    /// Session reset, called when a freshly loaded webview provably holds no
    /// result copy and no handle on the previous page's jobs. Releases every
    /// retained-but-untaken result and every webview custody lease, and
    /// abandons every queued or running job: cancellation is requested (status
    /// `abandoned_by_session_reset_*`), so a queued job exits without starting
    /// a worker and a running job's worker is killed instead of filling
    /// retention nobody can take. A live job's owners (input lease, worker
    /// grant) are not released here; its supervisor reaps the worker, drops
    /// them, then marks it cancelled. Publication re-checks cancellation under
    /// this registry lock, so no abandoned job can publish after this returns.
    pub(crate) fn dispose_all_results(&self) -> SolveResultsDisposal {
        let mut table = self.table();
        let mut released = Vec::new();
        let mut released_copies: Vec<Lease> = Vec::new();
        let mut abandoned_jobs = 0;
        for record in table.records.values_mut() {
            if !record.state.terminal() {
                if !record.abandoned {
                    record.abandoned = true;
                    record.cancellation_requested = true;
                    record.cancellation_status = ABANDONED_AWAITING.into();
                    abandoned_jobs += 1;
                }
                if let Some(worker) = &record.worker {
                    worker.kill();
                }
                continue;
            }
            let result = record.result.take();
            let lease = record.result_lease.take();
            record.digests = None;
            // A C2 full copy's lease goes with the page that held the copy.
            if let Some(copy) = record.full_copy_lease.take() {
                released_copies.push(copy);
            }
            if result.is_some() || lease.is_some() {
                released.push((result, lease));
            }
        }
        // The attestation table goes with the session (§2.4).
        table.attested.clear();
        let receipt = SolveResultsDisposal {
            released_results: released.len(),
            released_bytes: released
                .iter()
                .filter_map(|(result, lease)| lease.as_ref().or(result.as_ref().map(|held| &held.lease)))
                .map(Lease::bytes)
                .sum::<u64>()
                + released_copies.iter().map(Lease::bytes).sum::<u64>(),
            abandoned_jobs,
        };
        drop(released);
        drop(released_copies);
        table.prune();
        if abandoned_jobs > 0 {
            self.pool.wake_waiters();
        }
        // An on-demand model check in progress belongs to the page that is gone.
        drop(table);
        self.cancel_model_checks();
        receipt
    }

    pub(crate) fn cancel(&self, job_id: &str, cancellation_token: &str) -> Result<SolveJobCancellationReceipt, String> {
        let mut table = self.table();
        let record = table.records.get_mut(job_id).ok_or_else(|| format!("unknown solve job: {job_id}"))?;
        let receipt = |record: &SolveJobRecord, accepted: bool| SolveJobCancellationReceipt {
            job_id: job_id.to_string(),
            accepted,
            cancellation_status: record.cancellation_status.clone(),
            job_state: record.state.as_str(),
            cancellation_scope: SOLVE_JOB_CANCELLATION_SCOPE,
            cancellation_success_claimed: false,
        };
        if record.cancellation_token != cancellation_token {
            let mut out = receipt(record, false);
            out.cancellation_status = "rejected_invalid_cancellation_token".into();
            return Ok(out);
        }
        if record.state.terminal() {
            let mut out = receipt(record, false);
            out.cancellation_status =
                format!("request_after_terminal_state_{}_no_cancellation_performed", record.state.as_str());
            return Ok(out);
        }
        // Cancellation releases nothing itself: a running worker is killed and
        // its supervisor drops the owners after reaping it. An abandoned job
        // is already cancelling and keeps its abandonment status.
        record.cancellation_requested = true;
        if !record.abandoned {
            let status = if record.state == JobState::Running { CANCEL_REQUESTED_RUNNING } else { CANCEL_REQUESTED_QUEUED };
            record.cancellation_status = status.into();
        }
        if let Some(worker) = &record.worker {
            worker.kill();
        }
        let out = receipt(record, true);
        self.pool.wake_waiters();
        Ok(out)
    }
}


#[cfg(test)]
pub(crate) mod test_support {
    use super::*;
    use crate::memory_pool::test_host::ScriptedHost;
    use crate::solve_worker::test_support::TestLauncher;
    use std::time::{Duration, Instant};

    /// A registry whose workers run the real ordinary solve (this test binary
    /// in worker mode).
    pub(crate) fn real_registry(host: Arc<ScriptedHost>) -> SolveJobRegistry {
        SolveJobRegistry::with_hooks(MemoryPool::with_wait_tick(host, Duration::from_millis(5)), TestLauncher::solve(), Arc::new(thread_spawn))
    }

    pub(crate) fn wait_for(registry: &SolveJobRegistry, job_id: &str, done: impl Fn(&SolveJobStatusReport) -> bool) -> SolveJobStatusReport {
        let deadline = Instant::now() + Duration::from_secs(120);
        loop {
            let status = registry.status(job_id).expect("job known");
            if done(&status) {
                return status;
            }
            assert!(Instant::now() < deadline, "job {job_id} stuck: {status:?}");
            std::thread::sleep(Duration::from_millis(2));
        }
    }

    pub(crate) fn wait_terminal(registry: &SolveJobRegistry, job_id: &str) -> SolveJobStatusReport {
        wait_for(registry, job_id, |s| !matches!(s.state, "queued" | "running"))
    }

    /// W-11 (§3.6): a gated registry job admitted from `intake` under `mode`,
    /// released with `result` and taken into custody, as a delivered result is.
    /// Returns the registry, its gate (kept alive by the caller) and the job id.
    pub(crate) fn delivered_in_custody(
        host: Arc<ScriptedHost>,
        intake: &[u8],
        mode: Option<&str>,
        result: &str,
    ) -> (SolveJobRegistry, Arc<crate::solve_worker::test_support::Gate>, String) {
        let gate = crate::solve_worker::test_support::Gate::new("custody");
        let pool = MemoryPool::with_wait_tick(host, Duration::from_millis(5));
        pool.set_max_running_workers(16);
        let registry = SolveJobRegistry::with_hooks(pool, gate.launcher(), Arc::new(thread_spawn));
        let receipt = registry.start(intake, mode).unwrap_or_else(|e| panic!("{e:?}"));
        gate.wait_started(1);
        gate.release(result);
        let status = wait_terminal(&registry, &receipt.job_id);
        assert_eq!(status.state, "completed", "{status:?}");
        assert_eq!(registry.take_result(&receipt.job_id).unwrap(), result.as_bytes());
        (registry, gate, receipt.job_id)
    }

    /// Runs one job through the real ordinary solve in a worker and returns
    /// its terminal status and the taken bytes (if it completed).
    pub(crate) fn run_job(model: &serde_json::Value, mode: Option<&str>) -> (SolveJobStatusReport, Option<String>) {
        let registry = real_registry(ScriptedHost::normal());
        let body = serde_json::to_vec(model).unwrap();
        let receipt = registry.start(&body, mode).unwrap_or_else(|e| panic!("{e:?}"));
        let status = wait_terminal(&registry, &receipt.job_id);
        let bytes = (status.state == "completed")
            .then(|| String::from_utf8(registry.take_result(&receipt.job_id).unwrap()).unwrap());
        if bytes.is_some() {
            registry.dispose_result(&receipt.job_id).unwrap();
        }
        assert_eq!(registry.pool().reserved_total(), 0);
        assert_eq!(registry.pool().running_workers(), 0);
        (status, bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::*;
    use super::*;
    use crate::memory_pool::test_host::{ScriptedHost, GIB};
    use crate::memory_pool::PressureLevel;
    use crate::solve_worker::test_support::{Gate, TestLauncher};
    use crate::worker_alloc::resident_envelope;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{mpsc, Barrier};
    use std::time::Duration;

    const MODEL: &[u8] = br#"{"project":{"id":"p"},"nodes":[1,2,3]}"#;
    const MIB: u64 = 1 << 20;

    /// The process-count bound is set above what any test runs at once, so
    /// tests do not depend on this host's parallelism (one test sets its own).
    fn registry(host: &Arc<ScriptedHost>, launcher: Arc<dyn WorkerLauncher>) -> SolveJobRegistry {
        let pool = MemoryPool::with_wait_tick(host.clone(), Duration::from_millis(5));
        pool.set_max_running_workers(16);
        SolveJobRegistry::with_hooks(pool, launcher, Arc::new(thread_spawn))
    }

    /// A registry whose workers wait at a cross-process gate.
    fn gated(host: &Arc<ScriptedHost>) -> (SolveJobRegistry, Arc<Gate>) {
        let gate = Gate::new("jobs");
        (registry(host, gate.launcher()), gate)
    }

    /// The worker's starting grant for `MODEL`.
    fn start_grant() -> u64 {
        crate::solve_worker::initial_grant(PROCESS_RESERVE_BYTES, ingress::prescan(MODEL).unwrap().lease_bytes().unwrap()).unwrap()
    }

    /// End to end through the registry: a solve's intake lease, its result
    /// lease and the custody grow at take each wait out a headroom dip shorter
    /// than the re-sampling window; the job completes and its result is taken.
    #[test]
    fn intake_result_and_take_leases_wait_out_a_headroom_dip() {
        let host = ScriptedHost::normal();
        let gate = Gate::new("dip");
        let pool = MemoryPool::with_timing(host.clone(), Duration::from_millis(5), Duration::from_secs(20), Duration::from_millis(5));
        pool.set_max_running_workers(16);
        let registry = SolveJobRegistry::with_hooks(pool, gate.launcher(), Arc::new(thread_spawn));
        let refill_after = |delay_ms: u64| {
            let host = host.clone();
            host.set(|s| s.headroom_bytes = Some(0));
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(delay_ms));
                host.set(|s| s.headroom_bytes = Some(64 * GIB));
            })
        };
        let refill = refill_after(100);
        let receipt = registry.start(MODEL, None).expect("intake admitted once the dip passes");
        refill.join().unwrap();
        gate.wait_started(1);
        let refill = refill_after(100);
        gate.release("{\"r\":1}");
        let status = wait_terminal(&registry, &receipt.job_id);
        refill.join().unwrap();
        assert_eq!(status.state, "completed", "{status:?}");
        assert!(status.result_available);
        assert_eq!(status.resource_refusal, None);
        let result = registry.pool().reserved(LeasePurpose::Result);
        let refill = refill_after(100);
        let bytes = registry.take_result(&receipt.job_id).expect("take admitted once the dip passes");
        refill.join().unwrap();
        assert_eq!(bytes, b"{\"r\":1}");
        assert_eq!((registry.pool().reserved(LeasePurpose::Result), registry.pool().reserved(LeasePurpose::Custody)), (0, 2 * result));
        assert_eq!(registry.pool().status(JobCounts::default()).last_refusal, None, "a passing re-sample leaves no refusal");
    }

    /// Review round 1 (P2): a take's admitted growth, while it waits for the
    /// job table, is not yet the webview's copy, so no admission credits it:
    /// an interactive request must still find the growth in the headroom.
    #[test]
    fn a_pending_take_growth_is_not_credited_as_written() {
        let host = ScriptedHost::normal();
        let gate = Gate::new("take-credit");
        let pool = MemoryPool::with_timing(host.clone(), Duration::from_millis(5), Duration::from_secs(20), Duration::from_millis(5));
        pool.set_max_running_workers(16);
        let registry = SolveJobRegistry::with_hooks(pool, gate.launcher(), Arc::new(thread_spawn));
        let receipt = registry.start(MODEL, None).unwrap();
        gate.wait_started(1);
        gate.release("{\"r\":1}");
        assert_eq!(wait_terminal(&registry, &receipt.job_id).state, "completed");
        let result = registry.pool().reserved(LeasePurpose::Result);
        host.set(|s| s.headroom_bytes = Some(0));
        std::thread::scope(|scope| {
            let take = scope.spawn(|| registry.take_result(&receipt.job_id));
            // The take has read the record and is re-sampling (headroom 0).
            std::thread::sleep(Duration::from_millis(200));
            let table = registry.table();
            host.set(|s| s.headroom_bytes = Some(64 * GIB));
            let started = std::time::Instant::now();
            while registry.pool().reserved_total() != 2 * result {
                assert!(started.elapsed() < Duration::from_secs(10), "the take's growth was admitted");
                std::thread::sleep(Duration::from_millis(5));
            }
            // The take now waits for the table. Reserved: R (result) + R (pending);
            // credited: R (the retained result) only, so a request of X needs X + R.
            let request = MIB;
            host.set(|s| s.headroom_bytes = Some(request + result - 1));
            let refused = registry.pool().acquire_interactive(request).unwrap_err();
            assert_eq!(refused, ResourceRefusal::HeadroomInsufficient { required_bytes: request + result, available_bytes: request + result - 1 });
            host.set(|s| s.headroom_bytes = Some(64 * GIB));
            drop(table);
            assert_eq!(take.join().unwrap().unwrap(), b"{\"r\":1}");
        });
        assert_eq!((registry.pool().reserved(LeasePurpose::Ingress), registry.pool().reserved(LeasePurpose::Custody)), (0, 2 * result));
    }

    /// A pool (and budget) admitting one starting grant at a time.
    fn one_worker_pool(registry: &SolveJobRegistry) {
        let bytes = start_grant() + start_grant() / 2;
        registry.configure_pool(bytes, bytes, false).unwrap();
    }

    fn start_running(registry: &SolveJobRegistry) -> SolveJobStartReceipt {
        let receipt = registry.start(MODEL, None).unwrap();
        wait_for(registry, &receipt.job_id, |s| s.state != "queued");
        receipt
    }

    fn reaped(pid: i32) -> bool {
        // SAFETY: signal 0 only probes; ESRCH means the pid is gone (reaped).
        unsafe { libc::kill(pid, 0) != 0 }
    }

    fn pool_exhausted(status: &SolveJobStatusReport) -> bool {
        matches!(status.queue_reason, Some(ResourceRefusal::PoolExhausted { .. }))
    }

    /// W-11 (DEL-08-01 Design `exact-route-report-sections.md` §3.6, §7 slice
    /// (b)): the custody digests are the SHA-256 of the admitted body and of the
    /// delivered bytes with the admitted mode, available only while the result
    /// is delivered and in custody.
    #[test]
    fn custody_digests_bind_the_admitted_body_the_delivered_bytes_and_the_mode() {
        const RESULT: &str = "{\"result\":\"delivered bytes\"}";
        let host = ScriptedHost::normal();
        let (registry, gate) = gated(&host);
        let receipt = registry.start(MODEL, Some("dense_scrutiny")).unwrap();
        gate.wait_started(1);
        assert_eq!(registry.custody_binding(&receipt.job_id), None, "running: no binding");
        gate.release(RESULT);
        assert_eq!(wait_terminal(&registry, &receipt.job_id).state, "completed");
        assert_eq!(registry.custody_binding(&receipt.job_id), None, "a retained, untaken result has no binding");
        registry.take_result(&receipt.job_id).unwrap();
        let expected = CustodyDigests {
            intake_sha256: Sha256::digest(MODEL).into(),
            result_sha256: Sha256::digest(RESULT.as_bytes()).into(),
            solver_mode: PreviewSolverMode::DenseScrutiny,
        };
        assert_eq!(registry.custody_binding(&receipt.job_id), Some(expected));
        // The default mode is recorded as admitted.
        let second = registry.start(MODEL, None).unwrap();
        gate.wait_started(2);
        gate.release(RESULT);
        assert_eq!(wait_terminal(&registry, &second.job_id).state, "completed");
        registry.take_result(&second.job_id).unwrap();
        assert_eq!(registry.custody_binding(&second.job_id).map(|d| d.solver_mode), Some(PreviewSolverMode::default()));
        // Released with custody: one job by `dispose_result`, the other by `dispose_all_results`.
        registry.dispose_result(&receipt.job_id).unwrap();
        assert_eq!(registry.custody_binding(&receipt.job_id), None);
        assert!(registry.table().records[&receipt.job_id].digests.is_none());
        assert!(registry.custody_binding(&second.job_id).is_some());
        registry.dispose_all_results();
        assert_eq!(registry.custody_binding(&second.job_id), None);
        assert!(registry.table().records[&second.job_id].digests.is_none());
        assert_eq!(registry.custody_binding("backend-solve-job-unknown"), None);
        // A disposed untaken result never had a binding.
        let third = registry.start(MODEL, None).unwrap();
        gate.wait_started(3);
        gate.release(RESULT);
        assert_eq!(wait_terminal(&registry, &third.job_id).state, "completed");
        registry.dispose_result(&third.job_id).unwrap();
        assert_eq!(registry.custody_binding(&third.job_id), None);
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    /// A job that ends without a result releases its digest metadata at once;
    /// a running job holds only its intake digest and mode.
    #[test]
    fn a_job_without_a_result_releases_its_digests() {
        let host = ScriptedHost::normal();
        let (registry, gate) = gated(&host);
        let receipt = registry.start(MODEL, None).unwrap();
        gate.wait_started(1);
        let running = registry.table().records[&receipt.job_id].digests;
        assert_eq!(running.map(|d| (d.intake_sha256, d.result_sha256)), Some((Sha256::digest(MODEL).into(), None)));
        gate.release("err:invented worker failure");
        assert_eq!(wait_terminal(&registry, &receipt.job_id).state, "failed");
        assert!(registry.table().records[&receipt.job_id].digests.is_none());
        assert_eq!(registry.custody_binding(&receipt.job_id), None);
    }

    /// Pruning removes a released record and its digests with it; a record in
    /// custody is never pruned and keeps its binding.
    #[test]
    fn pruning_removes_released_digests_and_keeps_custody() {
        struct EchoTwice(std::sync::atomic::AtomicUsize);
        impl WorkerLauncher for EchoTwice {
            fn command(&self) -> std::io::Result<std::process::Command> {
                if self.0.fetch_add(1, Ordering::SeqCst) >= 2 {
                    return Err(std::io::Error::other("fast failure for a registry-logic test"));
                }
                TestLauncher::scripted("echo").command()
            }
        }
        let host = ScriptedHost::normal();
        let registry = registry(&host, Arc::new(EchoTwice(Default::default())));
        let released = registry.start(MODEL, None).unwrap();
        let kept = registry.start(MODEL, None).unwrap();
        for job in [&released, &kept] {
            assert_eq!(wait_terminal(&registry, &job.job_id).state, "completed");
            registry.take_result(&job.job_id).unwrap();
        }
        // The echo workload returns the body: both digests are the body's.
        let body: [u8; 32] = Sha256::digest(MODEL).into();
        assert_eq!(registry.custody_binding(&kept.job_id).map(|d| (d.intake_sha256, d.result_sha256)), Some((body, body)));
        registry.dispose_result(&released.job_id).unwrap();
        for _ in 0..(MAX_TERMINAL_RECORDS + 10) {
            let receipt = registry.start(MODEL, None).unwrap();
            assert_eq!(wait_terminal(&registry, &receipt.job_id).state, "failed");
        }
        assert!(registry.status(&released.job_id).is_err(), "the released record was pruned");
        assert_eq!(registry.custody_binding(&released.job_id), None);
        assert!(registry.custody_binding(&kept.job_id).is_some(), "a record in custody is never pruned");
        registry.dispose_result(&kept.job_id).unwrap();
    }

    /// The two hashes allocate nothing (counting allocator), and taking a
    /// result hashes nothing: its digests were recorded at publication.
    #[test]
    fn custody_hashes_allocate_nothing_and_take_hashes_nothing() {
        let large = vec![b'x'; 1 << 20];
        let (digest, measured) = crate::ingress::counting::measure(|| sha256_in_place(&large));
        assert_eq!(digest, <[u8; 32]>::from(Sha256::digest(&large)));
        assert_eq!(measured.allocations, 0, "{measured:?}");
        // `take_result` moves the bytes and grows the lease; the binding equals
        // what publication recorded, before and after the take.
        let host = ScriptedHost::normal();
        let (registry, gate) = gated(&host);
        let receipt = registry.start(MODEL, None).unwrap();
        gate.wait_started(1);
        gate.release("{\"r\":1}");
        assert_eq!(wait_terminal(&registry, &receipt.job_id).state, "completed");
        let published = registry.table().records[&receipt.job_id].digests;
        assert!(published.is_some_and(|d| d.result_sha256.is_some()), "recorded at publication");
        registry.take_result(&receipt.job_id).unwrap();
        assert_eq!(registry.custody_binding(&receipt.job_id).map(|d| d.result_sha256), published.and_then(|d| d.result_sha256));
        registry.dispose_result(&receipt.job_id).unwrap();
    }

    /// Owner test: the bundled demo through a real worker in both modes gives
    /// the in-process producer's exact bytes; leases are released in order.
    #[test]
    fn job_runs_the_real_ordinary_solve_in_a_worker_in_both_modes_with_exact_bytes() {
        let model = crate::load_preview_model().unwrap();
        let body = serde_json::to_vec(&model).unwrap();
        let start = crate::solve_worker::initial_grant(PROCESS_RESERVE_BYTES, ingress::prescan(&body).unwrap().lease_bytes().unwrap()).unwrap();
        let registry = real_registry(ScriptedHost::normal());
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let direct = crate::solve_preview_mechanics_with_mode(model.clone(), mode).unwrap();
            let receipt = registry.start(&body, Some(mode.as_str())).unwrap();
            assert_eq!((receipt.state, receipt.resource.route_pricing), ("queued", "ordinary_worker_capped"));
            assert_eq!(receipt.resource.invocation_budget_bytes, crate::memory_pool::DEFAULT_INVOCATION_BUDGET_BYTES);
            assert_eq!(receipt.resource.ingress_reserved_bytes, ingress::allocation_envelope(body.len() as u64).unwrap());
            assert_eq!((receipt.resource.worker_grant_bytes, receipt.resource.worker_peak_grant_bytes), (None, None));
            let status = wait_terminal(&registry, &receipt.job_id);
            assert_eq!(status.state, "completed", "{status:?}");
            assert_eq!(status.result_bytes, Some(direct.len() as u64));
            assert_eq!(status.resource.worker_grant_bytes, None, "released once the worker was reaped");
            let peak = status.resource.worker_peak_grant_bytes.unwrap();
            assert!(peak >= start && peak <= crate::memory_pool::DEFAULT_INVOCATION_BUDGET_BYTES, "{peak}");
            let footprint = status.resource.worker_peak_footprint_bytes.unwrap();
            assert!(footprint > 0 && footprint <= peak, "{footprint}");
            eprintln!(
                "{}: start grant {start}, peak grant {peak}, peak footprint {footprint}, result {} bytes",
                mode.as_str(),
                direct.len()
            );
            assert_eq!(registry.pool().reserved(LeasePurpose::Ingress), 0);
            assert_eq!((registry.pool().reserved(LeasePurpose::Worker), registry.pool().running_workers()), (0, 0));
            let retained = registry.pool().reserved(LeasePurpose::Result);
            assert_eq!(retained, resident_envelope(direct.len() as u64));
            let bytes = registry.take_result(&receipt.job_id).unwrap();
            assert_eq!(bytes, direct.as_bytes(), "{}: the producer's exact bytes", mode.as_str());
            assert_eq!(registry.pool().reserved(LeasePurpose::Custody), 2 * retained);
            let after = registry.status(&receipt.job_id).unwrap();
            assert!(!after.result_available && after.delivered && after.state == "completed");
            registry.dispose_result(&receipt.job_id).unwrap();
            assert_eq!(registry.pool().reserved_total(), 0);
        }
    }

    #[test]
    fn start_refuses_bad_modes_invalid_bodies_budget_overruns_and_full_queues() {
        let host = ScriptedHost::normal();
        let (registry, gate) = gated(&host);
        assert!(matches!(registry.start(MODEL, Some("fastest")).unwrap_err(), SolveJobError::InvalidRequest { .. }));
        let invalid = registry.start(b"{\"a\":", None).unwrap_err();
        assert!(matches!(invalid.refusal(), Some(ResourceRefusal::IngressInvalid { .. })));
        let raw_value = registry.start(br#"{"$serde_json::private::RawValue":"1"}"#, None).unwrap_err();
        assert!(matches!(raw_value.refusal(), Some(ResourceRefusal::IngressInvalid { .. })));
        // A budget below the worker's starting grant refuses it; nothing is reserved.
        for budget in [64, start_grant() - 1] {
            registry.configure_pool(12 * GIB, budget, false).unwrap();
            assert_eq!(
                registry.start(MODEL, None).unwrap_err().refusal(),
                Some(&ResourceRefusal::IngressExceedsBudget { required_bytes: start_grant(), budget_bytes: budget })
            );
        }
        assert_eq!(registry.pool().reserved_total(), 0);
        // One running + MAX_QUEUED_JOBS queued for the pool; the next start refuses queue_full.
        one_worker_pool(&registry);
        let first = registry.start(MODEL, Some("dense_scrutiny")).unwrap();
        gate.wait_started(1);
        let queued: Vec<_> = (0..MAX_QUEUED_JOBS).map(|_| registry.start(MODEL, None).unwrap()).collect();
        for receipt in &queued {
            wait_for(&registry, &receipt.job_id, pool_exhausted);
        }
        let full = registry.start(MODEL, None).unwrap_err();
        assert_eq!(full.refusal(), Some(&ResourceRefusal::QueueFull { queued: MAX_QUEUED_JOBS, limit: MAX_QUEUED_JOBS }));
        for receipt in queued.iter().chain([&first]) {
            registry.cancel(&receipt.job_id, &receipt.backend_cancellation_token).unwrap();
        }
        for receipt in queued.iter().chain([&first]) {
            assert_eq!(wait_terminal(&registry, &receipt.job_id).state, "cancelled");
        }
        assert_eq!(registry.pool().reserved_total(), 0);
        assert_eq!(gate.started_count(), 1, "queued jobs never started a worker");
        let error = serde_json::to_value(full).unwrap();
        assert_eq!(error["code"], "RESOURCE_UNAVAILABLE");
        assert_eq!(error["refusal"]["kind"], "queue_full");
        assert!(error["pool"]["pool_bytes"].is_u64());
        assert_eq!(error["pool"]["ordinary_route_pricing"], "worker_capped");
    }

    #[test]
    fn pressure_warning_queues_then_normal_runs_and_critical_spares_the_running_job() {
        let host = ScriptedHost::normal();
        let (registry, gate) = gated(&host);
        one_worker_pool(&registry);
        // Another reservation fills the pool while this job is admitted.
        let held = registry.pool().acquire(start_grant(), LeasePurpose::Ingress).unwrap();
        let job = registry.start(MODEL, None).unwrap();
        wait_for(&registry, &job.job_id, pool_exhausted);
        host.set(|s| s.pressure = PressureLevel::Warning);
        drop(held);
        let queued = wait_for(&registry, &job.job_id, |s| {
            s.queue_reason == Some(ResourceRefusal::PressureElevated { level: PressureLevel::Warning })
        });
        assert_eq!(queued.state, "queued");
        assert_eq!(gate.started_count(), 0);
        // New admissions refuse at warning pressure; nothing falls back.
        assert_eq!(
            registry.start(MODEL, None).unwrap_err().refusal(),
            Some(&ResourceRefusal::PressureElevated { level: PressureLevel::Warning })
        );
        host.set(|s| s.pressure = PressureLevel::Normal);
        gate.wait_started(1);
        let running = registry.status(&job.job_id).unwrap();
        assert_eq!((running.state, running.queue_reason), ("running", None));
        assert_eq!(running.resource.worker_grant_bytes, Some(start_grant()));
        assert_eq!(registry.pool_status().running_workers, 1);
        assert_eq!(registry.pool_status().reserved_by_purpose.worker, start_grant());
        // Normal -> Critical: new admission refused; the running job is unaffected.
        host.set(|s| s.pressure = PressureLevel::Critical);
        assert_eq!(
            registry.start(MODEL, None).unwrap_err().refusal(),
            Some(&ResourceRefusal::PressureElevated { level: PressureLevel::Critical })
        );
        std::thread::sleep(Duration::from_millis(20));
        assert_eq!(registry.status(&job.job_id).unwrap().state, "running");
        host.set(|s| s.pressure = PressureLevel::Normal);
        gate.release("{\"done\":true}");
        assert_eq!(wait_terminal(&registry, &job.job_id).state, "completed");
        registry.dispose_result(&job.job_id).unwrap();

        // Unknown and stale headroom also keep a job queued.
        let held = registry.pool().acquire(start_grant(), LeasePurpose::Ingress).unwrap();
        let next = registry.start(MODEL, None).unwrap();
        wait_for(&registry, &next.job_id, pool_exhausted);
        host.set(|s| s.headroom_bytes = None);
        drop(held);
        wait_for(&registry, &next.job_id, |s| s.queue_reason == Some(ResourceRefusal::HeadroomUnknown));
        host.set(|s| {
            s.headroom_bytes = Some(64 * GIB);
            s.age = Duration::from_secs(3);
        });
        wait_for(&registry, &next.job_id, |s| matches!(s.queue_reason, Some(ResourceRefusal::HeadroomStale { .. })));
        // An unreadable pressure level is reported as such, not as elevated.
        host.set(|s| {
            s.age = Duration::ZERO;
            s.pressure = PressureLevel::Unknown;
        });
        wait_for(&registry, &next.job_id, |s| s.queue_reason == Some(ResourceRefusal::PressureUnknown));
        assert_eq!(registry.start(MODEL, None).unwrap_err().refusal(), Some(&ResourceRefusal::PressureUnknown));
        assert_eq!(gate.started_count(), 1);
        host.set(|s| s.pressure = PressureLevel::Normal);
        gate.wait_started(2);
        gate.release("{}");
        assert_eq!(wait_terminal(&registry, &next.job_id).state, "completed");
        registry.dispose_result(&next.job_id).unwrap();
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    #[test]
    fn poll_never_returns_the_result_and_take_grows_the_lease_to_2r() {
        const RESULT: &[u8] = b"{\"result\":\"exact bytes\"}";
        let host = ScriptedHost::normal();
        let registry = registry(&host, TestLauncher::scripted("echo"));
        let receipt = registry.start(RESULT, None).unwrap();
        let status = wait_terminal(&registry, &receipt.job_id);
        let report = serde_json::to_value(&status).unwrap();
        assert!(report.get("result").is_none(), "polls never carry the result");
        for key in [
            "job_id", "state", "queue_reason", "cancellation_requested", "cancellation_status", "cancellation_scope",
            "result_available", "result_bytes", "error_message", "resource_refusal", "resource",
        ] {
            assert!(report.get(key).is_some(), "{key}");
        }
        assert_eq!(report["resource"]["route_pricing"], "ordinary_worker_capped");
        assert_eq!(report["cancellation_scope"], "worker_process_terminated");
        assert!(report["resource"]["worker_peak_footprint_bytes"].as_u64().unwrap() > 0, "measured at the result");
        assert_eq!(report["resource"]["worker_grant_bytes"], serde_json::Value::Null);
        assert_eq!(report["resource"]["worker_peak_grant_bytes"], start_grant_for(RESULT));
        assert_eq!(status.result_bytes, Some(24));
        let r = registry.pool().reserved(LeasePurpose::Result);
        assert_eq!(r, 32, "the resident envelope of 24 bytes");
        // Poll repeatedly: the ledger never grows.
        for _ in 0..10 {
            registry.status(&receipt.job_id).unwrap();
        }
        assert_eq!(registry.pool().reserved_total(), r);

        // A refused growth keeps the result retained.
        host.set(|s| s.headroom_bytes = Some(r)); // 2R + outstanding R exceeds R
        let refused = registry.take_result(&receipt.job_id).unwrap_err();
        assert!(matches!(refused.refusal(), Some(ResourceRefusal::HeadroomInsufficient { .. })));
        assert!(registry.status(&receipt.job_id).unwrap().result_available);
        assert_eq!(registry.pool().reserved(LeasePurpose::Result), r);

        host.set(|s| s.headroom_bytes = Some(64 * GIB));
        let bytes = registry.take_result(&receipt.job_id).unwrap();
        assert_eq!(bytes, RESULT);
        assert_eq!((registry.pool().reserved(LeasePurpose::Result), registry.pool().reserved(LeasePurpose::Custody)), (0, 2 * r));
        let delivered = registry.status(&receipt.job_id).unwrap();
        assert!(!delivered.result_available && delivered.delivered);
        assert!(matches!(registry.take_result(&receipt.job_id).unwrap_err(), SolveJobError::InvalidRequest { .. }));
        let disposal = registry.dispose_result(&receipt.job_id).unwrap();
        assert!(disposal.released && disposal.delivered && disposal.released_bytes == 2 * r);
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    fn start_grant_for(body: &[u8]) -> u64 {
        crate::solve_worker::initial_grant(PROCESS_RESERVE_BYTES, ingress::prescan(body).unwrap().lease_bytes().unwrap()).unwrap()
    }

    #[test]
    fn retention_and_custody_bounds_refuse_without_losing_held_results() {
        let host = ScriptedHost::normal();
        let (registry, gate) = gated(&host);
        // Five workers run in parallel while nothing is retained; the last to
        // publish finds retention full (post-solve check).
        let started_jobs: Vec<_> = (0..=MAX_QUEUED_JOBS).map(|_| start_running(&registry).job_id).collect();
        gate.wait_started(MAX_QUEUED_JOBS + 1);
        assert_eq!(registry.pool_status().running_workers, MAX_QUEUED_JOBS + 1);
        for _ in 0..=MAX_QUEUED_JOBS {
            gate.release("{\"r\":1}");
        }
        let (mut jobs, mut refused) = (Vec::new(), Vec::new());
        for job in started_jobs {
            let status = wait_terminal(&registry, &job);
            match status.state {
                "completed" => jobs.push(job),
                _ => refused.push(status),
            }
        }
        assert_eq!((jobs.len(), refused.len()), (MAX_RETAINED_RESULTS, 1), "{refused:?}");
        assert_eq!(refused[0].state, "refused");
        assert_eq!(refused[0].resource_refusal, Some(ResourceRefusal::RetentionFull { retained: 4, limit: 4 }));
        assert_eq!(registry.pool_status().retained_results, 4);
        // With retention full, a start is refused early: no reservation, no worker.
        let reserved = registry.pool().reserved_total();
        let early = registry.start(MODEL, None).unwrap_err();
        assert_eq!(early.refusal(), Some(&ResourceRefusal::RetentionFull { retained: 4, limit: 4 }));
        assert_eq!(registry.pool().reserved_total(), reserved);
        assert_eq!(gate.started_count(), MAX_QUEUED_JOBS + 1, "no worker started");
        // Taking all four fills custody; a fifth take refuses and keeps its result.
        for job in &jobs {
            registry.take_result(job).unwrap();
        }
        let fifth = registry.start(MODEL, None).unwrap();
        gate.wait_started(MAX_QUEUED_JOBS + 2);
        gate.release("{\"r\":1}");
        assert_eq!(wait_terminal(&registry, &fifth.job_id).state, "completed");
        let refusal = registry.take_result(&fifth.job_id).unwrap_err();
        assert_eq!(refusal.refusal(), Some(&ResourceRefusal::CustodyFull { delivered: 4, limit: 4 }));
        assert!(registry.status(&fifth.job_id).unwrap().result_available);
        registry.dispose_result(&jobs[0]).unwrap();
        registry.take_result(&fifth.job_id).unwrap();
        // Disposing a retained-but-untaken result drops it too.
        let untaken = registry.start(MODEL, None).unwrap();
        gate.wait_started(MAX_QUEUED_JOBS + 3);
        gate.release("{\"r\":1}");
        wait_terminal(&registry, &untaken.job_id);
        assert!(registry.dispose_result(&untaken.job_id).unwrap().released);
        assert!(!registry.status(&untaken.job_id).unwrap().result_available);
        for job in jobs.iter().skip(1).chain([&fifth.job_id]) {
            registry.dispose_result(job).unwrap();
        }
        assert_eq!(registry.pool().reserved_total(), 0);
        let status = registry.pool_status();
        assert_eq!((status.retained_results, status.webview_custody_results), (0, 0));
    }

    /// Owner test "dispose_all kills running workers": a session reset kills
    /// the running worker (released only after its reap) and abandons the queue.
    #[test]
    fn dispose_all_releases_retained_and_custody_kills_running_workers_and_abandons_queued_jobs() {
        let host = ScriptedHost::normal();
        let (registry, gate) = gated(&host);
        // Two retained results; one of them taken into custody.
        let mut done = Vec::new();
        for n in 1..=2 {
            let receipt = registry.start(MODEL, None).unwrap();
            gate.wait_started(n);
            gate.release("{\"r\":1}");
            assert_eq!(wait_terminal(&registry, &receipt.job_id).state, "completed");
            done.push(receipt.job_id);
        }
        registry.take_result(&done[0]).unwrap();
        // A running job and a job queued behind it for the pool.
        one_worker_pool(&registry);
        let running = start_running(&registry);
        gate.wait_started(3);
        let pid = gate.pids().into_iter().find(|pid| !reaped(*pid)).expect("the running worker");
        let queued = registry.start(MODEL, None).unwrap();
        wait_for(&registry, &queued.job_id, pool_exhausted);
        let held = registry.pool().reserved(LeasePurpose::Result) + registry.pool().reserved(LeasePurpose::Custody);
        assert!(held > 0);
        assert_eq!(registry.pool().reserved(LeasePurpose::Worker), start_grant());

        let disposal = registry.dispose_all_results();
        assert_eq!(disposal, SolveResultsDisposal { released_results: 2, released_bytes: held, abandoned_jobs: 2 });
        let wire = serde_json::to_value(&disposal).unwrap();
        assert_eq!(wire, serde_json::json!({"released_results": 2, "released_bytes": held, "abandoned_jobs": 2}));
        assert_eq!(registry.pool().reserved(LeasePurpose::Result) + registry.pool().reserved(LeasePurpose::Custody), 0);
        for job in &done {
            let after = registry.status(job).unwrap();
            assert!(!after.result_available && after.state == "completed");
        }
        let status = wait_terminal(&registry, &queued.job_id);
        assert_eq!((status.state, status.cancellation_status.as_str()), ("cancelled", ABANDONED_BEFORE_START));
        assert!(status.cancellation_requested && !status.result_available);
        let status = wait_terminal(&registry, &running.job_id);
        assert_eq!((status.state, status.cancellation_status.as_str()), ("cancelled", ABANDONED_AT_PUBLICATION));
        assert!(!status.result_available && status.result_bytes.is_none());
        assert!(reaped(pid), "the killed worker was reaped before the record turned terminal");
        assert_eq!(registry.pool().reserved_total(), 0);
        assert_eq!(gate.started_count(), 3, "the queued job never started a worker");
        let pool = registry.pool_status();
        assert_eq!((pool.retained_results, pool.webview_custody_results, pool.running_jobs, pool.queued_jobs), (0, 0, 0, 0));
        assert_eq!(pool.running_workers, 0);
        assert_eq!(registry.dispose_all_results(), SolveResultsDisposal { released_results: 0, released_bytes: 0, abandoned_jobs: 0 });

        // The new page's starts are not refused for retention.
        let fresh = registry.start(MODEL, None).unwrap();
        gate.wait_started(4);
        gate.release("{\"r\":3}");
        assert!(wait_terminal(&registry, &fresh.job_id).result_available);
        let retained = registry.pool().reserved(LeasePurpose::Result);
        assert_eq!(
            registry.dispose_all_results(),
            SolveResultsDisposal { released_results: 1, released_bytes: retained, abandoned_jobs: 0 }
        );
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    /// Registry logic with a deferred supervisor thread (no worker is involved).
    #[test]
    fn abandonment_is_not_overwritten_by_a_token_holders_cancel() {
        let host = ScriptedHost::normal();
        let (deferred_tx, deferred) = mpsc::channel::<Box<dyn FnOnce() + Send>>();
        let deferred_tx = Mutex::new(deferred_tx);
        let spawn: SpawnFn = Arc::new(move |job| {
            deferred_tx.lock().unwrap().send(job).unwrap();
            Ok(())
        });
        let registry = SolveJobRegistry::with_hooks(
            MemoryPool::with_wait_tick(host, Duration::from_millis(5)),
            TestLauncher::scripted("echo"),
            spawn,
        );
        let job = registry.start(MODEL, None).unwrap();
        assert_eq!(registry.dispose_all_results().abandoned_jobs, 1);
        let cancel = registry.cancel(&job.job_id, &job.backend_cancellation_token).unwrap();
        assert!(cancel.accepted);
        assert_eq!(cancel.cancellation_status, ABANDONED_AWAITING);
        assert_eq!(registry.dispose_all_results(), SolveResultsDisposal { released_results: 0, released_bytes: 0, abandoned_jobs: 0 });
        std::thread::spawn(deferred.recv().unwrap()).join().unwrap();
        let status = registry.status(&job.job_id).unwrap();
        assert_eq!((status.state, status.cancellation_status.as_str()), ("cancelled", ABANDONED_BEFORE_START));
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    #[test]
    fn dispose_all_keeps_a_full_queue_from_filling_retention() {
        let host = ScriptedHost::normal();
        let (registry, gate) = gated(&host);
        one_worker_pool(&registry);
        // The old page: one running and a full queue, enough to fill retention.
        let first = start_running(&registry);
        gate.wait_started(1);
        let mut old: Vec<_> = vec![first.job_id];
        old.extend((0..MAX_QUEUED_JOBS).map(|_| registry.start(MODEL, None).unwrap().job_id));
        assert_eq!(registry.dispose_all_results().abandoned_jobs, MAX_QUEUED_JOBS + 1);
        for job in &old {
            let status = wait_terminal(&registry, job);
            assert_eq!(status.state, "cancelled", "{status:?}");
            assert!(status.cancellation_status.starts_with("abandoned_by_session_reset"));
        }
        assert_eq!(gate.started_count(), 1, "abandoned queued jobs never started a worker");
        assert_eq!(registry.pool().reserved_total(), 0);
        assert_eq!(registry.pool_status().retained_results, 0);
        // The new page can start and retain as usual.
        let fresh = registry.start(MODEL, None).unwrap();
        gate.wait_started(2);
        gate.release("{\"r\":2}");
        assert_eq!(wait_terminal(&registry, &fresh.job_id).state, "completed");
        registry.dispose_result(&fresh.job_id).unwrap();
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    /// Owner test "result-transfer overlap": the result lease is admitted
    /// while the worker's grant is still held, and the grant is released only
    /// after the worker exits and is reaped.
    #[test]
    fn result_lease_overlaps_the_grant_until_the_worker_is_reaped() {
        let host = ScriptedHost::normal();
        // The worker's exit is held at a gate until the overlap has been inspected.
        let exit = Gate::new("exit");
        let launcher = Arc::new(
            Arc::unwrap_or_clone(TestLauncher::scripted("result:7"))
                .with_env(crate::solve_worker::tests::EXIT_GATE_ENV, exit.dir.display().to_string()),
        );
        let registry = registry(&host, launcher);
        let receipt = registry.start(MODEL, None).unwrap();
        // The worker has sent its result and is still alive (its exit is held).
        let overlap = wait_for(&registry, &receipt.job_id, |_| registry.pool().reserved(LeasePurpose::Result) > 0);
        assert_eq!(overlap.state, "running");
        assert_eq!(registry.pool().reserved(LeasePurpose::Result), 16);
        assert_eq!(registry.pool().reserved(LeasePurpose::Worker), start_grant(), "the grant is still held");
        assert_eq!(registry.pool_status().running_workers, 1);
        std::fs::write(exit.dir.join("exit-released"), b"").unwrap();
        let status = wait_terminal(&registry, &receipt.job_id);
        assert_eq!(status.state, "completed");
        assert_eq!((registry.pool().reserved(LeasePurpose::Worker), registry.pool().reserved(LeasePurpose::Result)), (0, 16));
        assert_eq!(registry.take_result(&receipt.job_id).unwrap(), b"xxxxxxx");
        registry.dispose_result(&receipt.job_id).unwrap();
        // A page-granular result is leased by its resident pages.
        let registry = super::tests::registry(&host, TestLauncher::scripted("result:32769"));
        let receipt = registry.start(MODEL, None).unwrap();
        assert_eq!(wait_terminal(&registry, &receipt.job_id).state, "completed");
        assert_eq!(registry.pool().reserved(LeasePurpose::Result), 48 * 1024);
        registry.dispose_result(&receipt.job_id).unwrap();
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    #[test]
    fn result_lease_refusal_marks_the_job_refused_and_drops_the_result() {
        let host = ScriptedHost::normal();
        let (registry, gate) = gated(&host);
        let receipt = registry.start(MODEL, None).unwrap();
        gate.wait_started(1);
        host.set(|s| s.headroom_bytes = Some(0));
        gate.release("{\"big\":true}");
        let status = wait_terminal(&registry, &receipt.job_id);
        assert_eq!(status.state, "refused");
        assert!(matches!(status.resource_refusal, Some(ResourceRefusal::HeadroomInsufficient { .. })));
        assert!(!status.result_available);
        assert_eq!(registry.pool().reserved_total(), 0);
        assert_eq!(registry.pool().running_workers(), 0);
    }

    /// Owner test "cancellation": the running worker is killed and its grant
    /// released only after the reap; a queued job exits without a worker.
    #[test]
    fn cancellation_kills_the_running_worker_and_releases_after_reap() {
        let host = ScriptedHost::normal();
        let (registry, gate) = gated(&host);
        one_worker_pool(&registry);
        let running = start_running(&registry);
        gate.wait_started(1);
        let pid = gate.pids()[0];
        let queued = registry.start(MODEL, None).unwrap();
        wait_for(&registry, &queued.job_id, pool_exhausted);
        // The running job's raw copy was released once sent; the queued job holds its own.
        assert_eq!(registry.pool().reserved(LeasePurpose::Ingress), queued.resource.ingress_reserved_bytes);
        assert_eq!(registry.pool().reserved(LeasePurpose::Worker), start_grant());

        let cancel = registry.cancel(&queued.job_id, &queued.backend_cancellation_token).unwrap();
        assert!(cancel.accepted && !cancel.cancellation_success_claimed);
        assert_eq!(cancel.cancellation_status, CANCEL_REQUESTED_QUEUED);
        let status = wait_terminal(&registry, &queued.job_id);
        assert_eq!((status.state, status.cancellation_status.as_str()), ("cancelled", CANCELLED_BEFORE_START));
        assert_eq!(registry.pool().reserved(LeasePurpose::Ingress), 0);
        assert_eq!(status.resource.worker_peak_grant_bytes, None, "never admitted");

        let cancel = registry.cancel(&running.job_id, &running.backend_cancellation_token).unwrap();
        assert!(cancel.accepted && !cancel.cancellation_success_claimed);
        assert_eq!(cancel.job_state, "running");
        assert_eq!(cancel.cancellation_status, "cancellation_requested_worker_terminating");
        assert_eq!(cancel.cancellation_scope, "worker_process_terminated");
        let status = wait_terminal(&registry, &running.job_id);
        assert_eq!((status.state, status.cancellation_status.as_str()), ("cancelled", CANCELLED_AT_PUBLICATION));
        assert!(!status.result_available);
        assert!(reaped(pid), "reaped before the record turned terminal");
        assert_eq!(registry.pool().reserved_total(), 0);
        assert_eq!(registry.pool().running_workers(), 0);
        assert_eq!(gate.started_count(), 1);

        // Cancellation after completion is rejected without a success claim.
        let registry = super::tests::registry(&host, TestLauncher::scripted("echo"));
        let done = registry.start(MODEL, None).unwrap();
        wait_terminal(&registry, &done.job_id);
        let late = registry.cancel(&done.job_id, &done.backend_cancellation_token).unwrap();
        assert!(!late.accepted && !late.cancellation_success_claimed);
        assert_eq!(late.cancellation_status, "request_after_terminal_state_completed_no_cancellation_performed");
        let wrong = registry.cancel(&done.job_id, "wrong-token").unwrap();
        assert_eq!(wrong.cancellation_status, "rejected_invalid_cancellation_token");
        assert!(registry.cancel("backend-solve-job-missing", "t").is_err());
        assert!(registry.status("backend-solve-job-missing").is_err());
    }

    /// Owner tests "child crash/panic": each ends the job failed, releases the
    /// grant after the reap and leaves the pool admitting.
    #[test]
    fn worker_panic_crash_and_producer_errors_fail_the_job_and_release() {
        let host = ScriptedHost::normal();
        let (registry, gate) = gated(&host);
        for (n, (outcome, expected)) in [
            ("panic", "PREVIEW-SOLVE-PANICKED: scripted solve panic"),
            ("abort", "SOLVE_WORKER_EXITED: terminated by signal 6"),
            ("err:PREVIEW-SOLVE-REFUSED: scripted", "PREVIEW-SOLVE-REFUSED: scripted"),
        ]
        .into_iter()
        .enumerate()
        {
            let job = registry.start(MODEL, None).unwrap();
            gate.wait_started(n + 1);
            gate.release(outcome);
            let status = wait_terminal(&registry, &job.job_id);
            assert_eq!((status.state, status.error_message.as_deref()), ("failed", Some(expected)));
            assert_eq!(registry.pool().reserved_total(), 0);
            assert_eq!(registry.pool().running_workers(), 0);
        }
        // Protocol violations fail the job too.
        let registry = super::tests::registry(&host, TestLauncher::scripted(format!("oversized_header:{}", 1u64 << 40)));
        let job = registry.start(MODEL, None).unwrap();
        let status = wait_terminal(&registry, &job.job_id);
        assert_eq!(status.state, "failed");
        assert!(status.error_message.unwrap().starts_with("SOLVE_WORKER_PROTOCOL: a result of 1099511627776 bytes"));
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    /// Owner test "cap enforcement" at the job level: refused with a typed
    /// refusal, never a fallback; the grant grew to the budget and no further.
    #[test]
    fn a_solve_needing_more_than_the_budget_is_refused_typed() {
        let host = ScriptedHost::normal();
        let registry = registry(&host, TestLauncher::scripted(format!("alloc_steps:{}:{}", 4 * MIB, 64 * MIB)));
        let budget = PROCESS_RESERVE_BYTES + 24 * MIB;
        registry.configure_pool(12 * GIB, budget, false).unwrap();
        let job = registry.start(MODEL, None).unwrap();
        let status = wait_terminal(&registry, &job.job_id);
        assert_eq!(status.state, "refused", "{status:?}");
        assert!(matches!(status.resource_refusal, Some(ResourceRefusal::WorkerCapExceeded { cap_bytes, .. }) if cap_bytes == budget));
        assert_eq!((status.resource.worker_grant_bytes, status.resource.worker_peak_grant_bytes), (None, Some(budget)));
        assert!(status.error_message.is_none() && !status.result_available);
        assert!(matches!(registry.pool_status().last_refusal, Some(ResourceRefusal::WorkerCapExceeded { .. })));
        assert_eq!(serde_json::to_value(&status).unwrap()["resource_refusal"]["kind"], "worker_cap_exceeded");
        assert_eq!(registry.pool().reserved_total(), 0);
        assert_eq!(registry.pool().running_workers(), 0);
    }

    /// Owner test "races": simultaneous starts for a pool that fits only two
    /// grants never over-reserve; the others queue, then run.
    #[test]
    fn simultaneous_starts_race_for_a_pool_that_fits_only_some() {
        let host = ScriptedHost::normal();
        let (registry, gate) = gated(&host);
        let start = start_grant();
        let pool_bytes = 2 * start + start / 2;
        registry.configure_pool(pool_bytes, 2 * start, false).unwrap();
        let stop = Arc::new(AtomicBool::new(false));
        // The monitor reports when it has seen two workers running, so the
        // workers are not released before it has.
        let (saw_two_tx, saw_two) = mpsc::channel();
        let monitor = {
            let (pool, stop) = (registry.pool().clone(), stop.clone());
            std::thread::spawn(move || {
                let (mut most, mut reported) = (0, false);
                while !stop.load(Ordering::SeqCst) {
                    assert!(pool.reserved_total() <= pool_bytes, "never over-reserved");
                    most = most.max(pool.running_workers());
                    if most >= 2 && !reported {
                        reported = saw_two_tx.send(()).is_ok();
                    }
                    std::thread::sleep(Duration::from_millis(1));
                }
                most
            })
        };
        let jobs = 4;
        let barrier = Arc::new(Barrier::new(jobs));
        let receipts: Vec<_> = (0..jobs)
            .map(|_| {
                let (registry, barrier) = (registry.clone(), barrier.clone());
                std::thread::spawn(move || {
                    barrier.wait();
                    registry.start(MODEL, None).unwrap()
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|h| h.join().unwrap())
            .collect();
        gate.wait_started(2);
        // Every job has either started or been refused its grant at least
        // once (its supervisor recorded the queue reason).
        for receipt in &receipts {
            wait_for(&registry, &receipt.job_id, |s| s.state != "queued" || s.queue_reason.is_some());
        }
        assert_eq!(gate.started_count(), 2, "only two grants fit");
        let states: Vec<_> = receipts.iter().map(|r| registry.status(&r.job_id).unwrap()).collect();
        assert_eq!(states.iter().filter(|s| s.state == "running").count(), 2);
        assert!(states.iter().filter(|s| s.state == "queued").all(pool_exhausted));
        assert_eq!(registry.pool().reserved(LeasePurpose::Worker), 2 * start);
        saw_two.recv_timeout(Duration::from_secs(60)).expect("the monitor sees the two running workers");
        for _ in 0..jobs {
            gate.release("{\"r\":1}");
        }
        for receipt in &receipts {
            assert_eq!(wait_terminal(&registry, &receipt.job_id).state, "completed");
        }
        stop.store(true, Ordering::SeqCst);
        assert_eq!(monitor.join().unwrap(), 2, "two workers ran in parallel, never more");
        assert_eq!(gate.started_count(), jobs);
        for receipt in &receipts {
            registry.dispose_result(&receipt.job_id).unwrap();
        }
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    /// The process-count bound: with one running worker allowed, the next
    /// admitted job waits in the queue (`worker_limit_reached`), then runs.
    #[test]
    fn running_workers_beyond_the_process_bound_wait_in_the_queue() {
        let host = ScriptedHost::normal();
        let (registry, gate) = gated(&host);
        registry.pool().set_max_running_workers(1);
        let first = start_running(&registry);
        gate.wait_started(1);
        let second = registry.start(MODEL, None).unwrap();
        let waiting = wait_for(&registry, &second.job_id, |s| s.queue_reason.is_some());
        assert_eq!(waiting.queue_reason, Some(ResourceRefusal::WorkerLimitReached { running: 1, limit: 1 }));
        assert_eq!(registry.pool_status().limits.max_running_workers, 1);
        std::thread::sleep(Duration::from_millis(30));
        assert_eq!(gate.started_count(), 1);
        gate.release("{\"r\":1}");
        assert_eq!(wait_terminal(&registry, &first.job_id).state, "completed");
        gate.wait_started(2);
        gate.release("{\"r\":2}");
        assert_eq!(wait_terminal(&registry, &second.job_id).state, "completed");
        for job in [&first.job_id, &second.job_id] {
            registry.dispose_result(job).unwrap();
        }
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    /// An over-budget lifetime peak reported with a worker's result propagates
    /// through the job: it is recorded in the job's resource, refuses the
    /// result before anything is released and keeps no lease, and the worker
    /// is killed at the refusal. The worker reports an 81 MiB peak in a valid
    /// result frame without using that memory, so no poll can see it first.
    /// The real kernel lifetime-maximum excursion is covered deterministically
    /// by `solve_worker::tests::the_lifetime_peak_footprint_is_checked_at_the_result`.
    #[test]
    fn an_over_budget_reported_result_peak_refuses_the_job() {
        let host = ScriptedHost::normal();
        let gate = Gate::new("reported-peak");
        let registry = registry(&host, TestLauncher::scripted(format!("reported_peak:{}:{}", 81 * MIB, gate.dir.display())));
        registry.configure_pool(12 * GIB, start_grant(), false).unwrap();
        let job = registry.start(MODEL, None).unwrap();
        let status = wait_terminal(&registry, &job.job_id);
        assert_eq!(status.state, "refused", "{status:?}");
        assert!(matches!(status.resource_refusal, Some(ResourceRefusal::WorkerFootprintExceeded { grant_bytes, footprint_bytes })
            if grant_bytes == start_grant() && footprint_bytes > 80 * MIB));
        assert!(status.resource.worker_peak_footprint_bytes.unwrap() > start_grant());
        assert!(!status.result_available);
        assert_eq!(registry.pool().reserved_total(), 0);
        // Killed by the supervisor at the refusal, not ended by its own 10 s wait.
        let pids = gate.pids();
        assert_eq!(pids.len(), 1, "{pids:?}");
        assert!(reaped(pids[0]), "the refused worker was reaped before the record turned terminal");
        assert!(!gate.dir.join(format!("unkilled-{}", pids[0])).exists(), "the refused worker was not killed");
    }

    struct FailingLauncher;

    impl WorkerLauncher for FailingLauncher {
        fn command(&self) -> std::io::Result<std::process::Command> {
            Err(std::io::Error::other("no worker executable"))
        }
    }

    #[test]
    fn failed_thread_or_worker_starts_release_every_lease() {
        let host = ScriptedHost::normal();
        let pool = MemoryPool::with_wait_tick(host.clone(), Duration::from_millis(5));
        let dropped_before_failure = Arc::new(AtomicBool::new(false));
        let observed = dropped_before_failure.clone();
        let probe_pool = pool.clone();
        let spawn: SpawnFn = Arc::new(move |job| {
            drop(job);
            observed.store(probe_pool.reserved_total() == 0, Ordering::SeqCst);
            Err(std::io::Error::other("no threads"))
        });
        let registry = SolveJobRegistry::with_hooks(pool, Arc::new(FailingLauncher), spawn);
        let receipt = registry.start(MODEL, None).unwrap();
        assert_eq!(receipt.state, "failed");
        assert!(dropped_before_failure.load(Ordering::SeqCst), "the closure's lease dropped with it");
        let status = registry.status(&receipt.job_id).unwrap();
        assert!(status.error_message.unwrap().starts_with("PREVIEW-SOLVE-WORKER-START-FAILED: no threads"));
        assert_eq!(registry.pool().reserved_total(), 0);
        // A worker process that cannot be started fails the job and releases its grant.
        let registry = super::tests::registry(&host, Arc::new(FailingLauncher));
        let receipt = registry.start(MODEL, None).unwrap();
        let status = wait_terminal(&registry, &receipt.job_id);
        assert_eq!(status.state, "failed");
        assert_eq!(status.error_message.as_deref(), Some("PREVIEW-SOLVE-WORKER-START-FAILED: no worker executable"));
        assert_eq!((registry.pool().reserved_total(), registry.pool().running_workers()), (0, 0));
    }

    /// A supervisor panic (here: the launcher) ends the job failed and
    /// releases every lease; the pool keeps admitting.
    #[test]
    fn a_supervisor_panic_fails_the_job_and_releases_its_grant() {
        struct PanickingLauncher;
        impl WorkerLauncher for PanickingLauncher {
            fn command(&self) -> std::io::Result<std::process::Command> {
                panic!("launcher bookkeeping failed");
            }
        }
        let host = ScriptedHost::normal();
        let registry = registry(&host, Arc::new(PanickingLauncher));
        let job = registry.start(MODEL, None).unwrap();
        let status = wait_terminal(&registry, &job.job_id);
        assert_eq!((status.state, status.error_message.as_deref()), ("failed", Some("PREVIEW-SOLVE-PANICKED: launcher bookkeeping failed")));
        assert_eq!(status.resource.worker_grant_bytes, None);
        assert_eq!((registry.pool().reserved_total(), registry.pool().running_workers()), (0, 0));
    }

    /// Registry logic: one real worker result is kept while many failed
    /// records (a launcher failing fast, no worker) are pruned.
    #[test]
    fn terminal_records_are_pruned_beyond_the_limit_but_held_results_are_kept() {
        struct EchoOnce(AtomicBool);
        impl WorkerLauncher for EchoOnce {
            fn command(&self) -> std::io::Result<std::process::Command> {
                if self.0.swap(true, Ordering::SeqCst) {
                    return Err(std::io::Error::other("fast failure for a registry-logic test"));
                }
                TestLauncher::scripted("echo").command()
            }
        }
        let host = ScriptedHost::normal();
        let registry = registry(&host, Arc::new(EchoOnce(AtomicBool::new(false))));
        let kept = registry.start(MODEL, None).unwrap();
        assert_eq!(wait_terminal(&registry, &kept.job_id).state, "completed");
        for _ in 0..(MAX_TERMINAL_RECORDS + 10) {
            let receipt = registry.start(MODEL, None).unwrap();
            assert_eq!(wait_terminal(&registry, &receipt.job_id).state, "failed");
        }
        let table = registry.table();
        assert!(table.records.values().filter(|r| r.state.terminal()).count() <= MAX_TERMINAL_RECORDS);
        assert!(table.records.contains_key(&kept.job_id), "a record holding a result is never pruned");
        drop(table);
        registry.dispose_result(&kept.job_id).unwrap();
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    /// A configured budget above 12 GiB (96 GiB on an injected 1 TiB host)
    /// runs the real ordinary solve in a capped worker to the in-process
    /// producer's exact bytes; the grant grows on demand from the intake size
    /// and nothing is allocated toward the budget.
    #[test]
    fn a_budget_above_twelve_gib_runs_the_real_solve_with_exact_bytes() {
        const TIB: u64 = 1 << 40;
        let registry = real_registry(ScriptedHost::with(Some(TIB), Some(TIB), PressureLevel::Normal));
        let status = registry.configure_pool(768 * GIB, 96 * GIB, false).unwrap();
        assert_eq!((status.invocation_budget_bytes, status.max_invocation_budget_bytes), (96 * GIB, 768 * GIB));
        let model = crate::load_preview_model().unwrap();
        let body = serde_json::to_vec(&model).unwrap();
        let direct = crate::solve_preview_mechanics_with_mode(model, PreviewSolverMode::default()).unwrap();
        let receipt = registry.start(&body, None).unwrap();
        assert_eq!(receipt.resource.invocation_budget_bytes, 96 * GIB);
        let status = wait_terminal(&registry, &receipt.job_id);
        assert_eq!(status.state, "completed", "{status:?}");
        let peak = status.resource.worker_peak_grant_bytes.unwrap();
        // On demand from the intake size: far below the 12 GiB default, let alone the budget.
        assert!(peak < crate::memory_pool::DEFAULT_INVOCATION_BUDGET_BYTES, "{peak}");
        assert_eq!(registry.take_result(&receipt.job_id).unwrap(), direct.as_bytes(), "the producer's exact bytes");
        registry.dispose_result(&receipt.job_id).unwrap();
        assert_eq!((registry.pool().reserved_total(), registry.pool().running_workers()), (0, 0));
    }

    // ---- B11 on-demand model checks (Codex reviews r1 and r2 of #1665) ----

    const FAN_OUT_BASE: &str = include_str!("../../../../core/product_physics/tests/fixtures/model_checks/load_state_fan_out_base.json");

    /// Codex r2's adversarial shape (invented): a load-state model whose one
    /// material carries 1.2 MB of provenance, shared by `pipes` members; the
    /// load-state resolution copies the material into every resolved member.
    fn fan_out_model(pipes: usize) -> serde_json::Value {
        let mut model: serde_json::Value = serde_json::from_str(FAN_OUT_BASE).unwrap();
        model["materials"][0]["provenance"] = serde_json::json!(format!("review_invented_{}", "a".repeat(1_200_000)));
        let replicate = |value: &serde_json::Value| -> serde_json::Value {
            let text = value.to_string();
            serde_json::Value::Array((0..pipes).map(|i| serde_json::from_str(&text.replace("pipe:probe-0", &format!("pipe:probe-{i}"))).unwrap()).collect())
        };
        model["pipe_segments"] = replicate(&model["pipe_segments"][0]);
        let members = replicate(&model["reference_configurations"][0]["member_references"][0]);
        model["reference_configurations"][0]["member_references"] = members;
        let states = replicate(&model["load_cases"][0]["analysis_state"]["element_states"][0]);
        model["load_cases"][0]["analysis_state"]["element_states"] = states;
        model
    }

    fn checks_body(model: &serde_json::Value) -> Vec<u8> {
        serde_json::to_vec(&serde_json::json!({ "model": model })).unwrap()
    }

    /// The checks run in a capped worker process: Codex's 16- and 64-pipe
    /// fan-out shapes complete with the applier's report, their grant grows
    /// on demand and never passes the per-invocation budget, and every lease
    /// is released afterwards.
    #[test]
    fn model_checks_run_in_a_capped_worker_whatever_the_fan_out() {
        let host = ScriptedHost::normal();
        let registry = registry(&host, TestLauncher::solve());
        let (_, budget) = registry.pool().settings();
        for pipes in [1, 16, 64] {
            let model = fan_out_model(pipes);
            let body = checks_body(&model);
            let input = registry.admit_model_checks(&body).unwrap();
            let start_grant = input.start_grant;
            let output = registry.run_model_checks(input).unwrap();
            let report: serde_json::Value = serde_json::from_slice(&output.report).unwrap();
            assert_eq!(report, open_pipe_stress_operation_applier::model_checks(&model), "{pipes} pipes");
            assert!(output.peak_grant_bytes <= budget, "{pipes} pipes: grant {} above the budget {budget}", output.peak_grant_bytes);
            eprintln!(
                "model checks fan-out {pipes} pipes: body {} B, starting grant {start_grant} B, peak grant {} B, peak footprint {} B, report {} B",
                body.len(), output.peak_grant_bytes, output.peak_footprint_bytes, output.report.len()
            );
            drop(output);
            assert_eq!((registry.pool().reserved_total(), registry.pool().running_workers()), (0, 0), "{pipes} pipes");
        }
    }

    /// The per-invocation budget holds at the start and while the check runs
    /// (Codex r2's budget probe): a budget below the starting grant refuses the
    /// intake by type, and a budget the 64-pipe check outgrows ends its worker
    /// with a typed worker refusal; nothing stays reserved.
    #[test]
    fn model_checks_refuse_by_type_at_the_invocation_budget() {
        let host = ScriptedHost::normal();
        let registry = registry(&host, TestLauncher::solve());
        let pool_bytes = registry.pool().settings().0;
        // Codex's probe: the budget set to the demo's admitted ingress amount.
        let demo: serde_json::Value = serde_json::from_str(crate::BUNDLED_DEMO_MODEL).unwrap();
        registry.configure_pool(pool_bytes, 196_752, false).unwrap();
        let error = registry.admit_model_checks(&checks_body(&demo)).err().expect("refused at intake");
        assert!(
            matches!(&error, SolveJobError::ResourceUnavailable { refusal: ResourceRefusal::IngressExceedsBudget { budget_bytes: 196_752, .. }, .. }),
            "{error:?}"
        );
        // A budget just above the 64-pipe check's starting grant: the worker outgrows it.
        let body = checks_body(&fan_out_model(64));
        registry.configure_pool(pool_bytes, 12 * GIB, false).unwrap();
        let start_grant = registry.admit_model_checks(&body).unwrap().start_grant;
        let budget = start_grant + 8 * (1 << 20);
        registry.configure_pool(pool_bytes, budget, false).unwrap();
        let input = registry.admit_model_checks(&body).unwrap();
        match registry.run_model_checks(input) {
            Err(ModelChecksError::Unavailable(SolveJobError::ResourceUnavailable { refusal, .. })) => assert!(
                matches!(refusal, ResourceRefusal::WorkerCapExceeded { .. } | ResourceRefusal::WorkerFootprintExceeded { .. } | ResourceRefusal::WorkerGrowthRefused { .. }),
                "{refusal:?}"
            ),
            other => panic!("expected a typed worker refusal at the budget, got {other:?}"),
        }
        assert_eq!((registry.pool().reserved_total(), registry.pool().running_workers()), (0, 0));
    }

    /// With #1676's budget rule: a check that waits for a worker slot keeps the
    /// budget it was admitted under, as a queued solve does. Lowered to just
    /// above the starting grant while it waits, the 64-pipe check still grows
    /// past the lowered budget to its report, within the admitted one.
    #[test]
    fn a_check_admitted_before_a_budget_change_keeps_its_budget() {
        let host = ScriptedHost::normal();
        let registry = registry(&host, TestLauncher::solve());
        let pool_bytes = registry.pool().settings().0;
        registry.configure_pool(pool_bytes, 12 * GIB, false).unwrap();
        registry.pool().set_max_running_workers(0);
        let input = registry.admit_model_checks(&checks_body(&fan_out_model(64))).unwrap();
        let lowered = input.start_grant + 8 * (1 << 20);
        let waiting = {
            let registry = registry.clone();
            std::thread::spawn(move || registry.run_model_checks(input))
        };
        registry.configure_pool(pool_bytes, lowered, false).unwrap();
        registry.pool().set_max_running_workers(1);
        let output = joined(waiting).expect("the check keeps its admitted budget");
        assert!(lowered < output.peak_grant_bytes && output.peak_grant_bytes <= 12 * GIB, "peak grant {} lowered budget {lowered}", output.peak_grant_bytes);
        drop(output);
        assert_eq!((registry.pool().reserved_total(), registry.pool().running_workers()), (0, 0));
    }

    // --- W-18b (r2a): the attestation table (DEL-08-01 Design `report-user-release.md` §2.4) ---

    use crate::solve_attestation::MAX_ATTESTED_SOLVES;
    use crate::solve_worker::tests::scripted_attestation;

    fn hex(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }

    /// Publication records an attested solve; the table keeps the 32 most
    /// recent, evicting the oldest, outlives custody (`dispose_result`) and
    /// the record's pruning, records nothing for an unattested or cancelled
    /// job, and is cleared by `dispose_all_results`.
    #[test]
    fn the_attestation_table_is_bounded_outlives_custody_and_clears_on_reset() {
        let host = ScriptedHost::normal();
        let (registry, gate) = gated(&host);
        let mode = PreviewSolverMode::SparseInteractive;
        let total = MAX_ATTESTED_SOLVES + 3;
        let mut first_job = String::new();
        for index in 0..total {
            let receipt = registry.start(MODEL, None).unwrap();
            gate.wait_started(index + 1);
            gate.release(&format!("attested:{{\"n\":{index}}}"));
            assert_eq!(wait_terminal(&registry, &receipt.job_id).state, "completed");
            assert_eq!(registry.take_result(&receipt.job_id).unwrap(), format!("{{\"n\":{index}}}").as_bytes());
            // Custody ends; the attestation stays (not released with custody).
            registry.dispose_result(&receipt.job_id).unwrap();
            if index == 0 {
                first_job = receipt.job_id;
            }
        }
        let held = |index: usize| registry.holds_attested_solve(mode, &scripted_attestation(&format!("{{\"n\":{index}}}")));
        assert!((0..3).all(|index| !held(index)), "the three oldest were evicted");
        assert!((3..total).all(held), "the 32 most recent are held after their custody ended");
        assert!(!registry.holds_attested_solve(PreviewSolverMode::DenseScrutiny, &scripted_attestation("{\"n\":5}")), "another mode");
        let numbers = registry.table().attested.job_numbers();
        assert_eq!(numbers.len(), MAX_ATTESTED_SOLVES);
        assert_eq!(numbers.first(), Some(&4), "job numbers start at 1; the first three were evicted");

        // Not pruned with its record: the records age out, the entry stays.
        for _ in 0..(MAX_TERMINAL_RECORDS + 10) {
            let receipt = registry.start(MODEL, None).unwrap();
            gate.release("err:failed");
            assert_eq!(wait_terminal(&registry, &receipt.job_id).state, "failed");
        }
        assert!(registry.status(&first_job).is_err(), "the oldest records were pruned");
        assert!(held(total - 1), "unattested and failed jobs record nothing and evict nothing");
        assert_eq!(registry.table().attested.job_numbers(), numbers);

        // A job cancelled before publication records nothing.
        let receipt = registry.start(MODEL, None).unwrap();
        gate.wait_started(total + MAX_TERMINAL_RECORDS + 11);
        assert!(registry.cancel(&receipt.job_id, &receipt.backend_cancellation_token).unwrap().accepted);
        assert_eq!(wait_terminal(&registry, &receipt.job_id).state, "cancelled");
        assert_eq!(registry.table().attested.job_numbers(), numbers);

        // Session reset clears it.
        registry.dispose_all_results();
        assert!((0..total).all(|index| !held(index)));
        assert!(registry.table().attested.job_numbers().is_empty());
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    /// §2.4 and §6: the solved demo's `d_M` and `d_S` equal the digests that
    /// admission computes over the session model and the received result
    /// after the webview round trip (parse, RFC 8785 text, native parse), in
    /// both modes; another mode, a result with one added annotation and
    /// another model are not held. Nothing the webview receives carries a
    /// digest.
    #[test]
    fn the_solved_demos_attestation_matches_admissions_digests_after_the_round_trip() {
        use open_pipe_stress_canonical_json::canonical_json;
        let registry = real_registry(ScriptedHost::normal());
        let model = crate::load_preview_model().unwrap();
        let body = serde_json::to_vec(&model).unwrap();
        let round_trip = |value: &serde_json::Value| -> serde_json::Value { serde_json::from_str(&canonical_json(value)).unwrap() };
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let receipt = registry.start(&body, Some(mode.as_str())).unwrap();
            let status = wait_terminal(&registry, &receipt.job_id);
            assert_eq!(status.state, "completed", "{status:?}");
            let bytes = registry.take_result(&receipt.job_id).unwrap();
            // The webview parses the producer's bytes and writes the session
            // model and the result into the request's RFC 8785 text; admission parses it.
            let received: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
            let (session_model, source) = (round_trip(&model), round_trip(&received));
            let at_admission = SolveAttestation::of(&session_model, &source).expect("the demo is within the checked profile");
            assert!(registry.holds_attested_solve(mode, &at_admission), "{mode:?}");
            let other = match mode {
                PreviewSolverMode::SparseInteractive => PreviewSolverMode::DenseScrutiny,
                PreviewSolverMode::DenseScrutiny => PreviewSolverMode::SparseInteractive,
            };
            if mode == PreviewSolverMode::SparseInteractive {
                assert!(!registry.holds_attested_solve(other, &at_admission), "mode mismatch");
            }
            // An annotation added only in the request is not attested.
            let mut annotated = source.clone();
            let row = annotated["results"].as_array_mut().and_then(|rows| rows.first_mut()).expect("the demo has result rows");
            row.as_object_mut().unwrap().insert("x_annotation".into(), serde_json::json!("invented"));
            assert!(!registry.holds_attested_solve(mode, &SolveAttestation::of(&session_model, &annotated).unwrap()));
            let mut other_model = session_model.clone();
            other_model.as_object_mut().unwrap().insert("x_other".into(), serde_json::json!(1));
            assert!(!registry.holds_attested_solve(mode, &SolveAttestation::of(&other_model, &source).unwrap()));
            // Never sent to the webview: no receipt, status or disposal carries a digest.
            let disposal = registry.dispose_result(&receipt.job_id).unwrap();
            let sent = [
                serde_json::to_string(&receipt).unwrap(),
                serde_json::to_string(&registry.status(&receipt.job_id).unwrap()).unwrap(),
                serde_json::to_string(&disposal).unwrap(),
                serde_json::to_string(&registry.pool_status()).unwrap(),
            ];
            for digest in [at_admission.model_sha256, at_admission.result_sha256] {
                assert!(sent.iter().all(|text| !text.contains(&hex(&digest))));
            }
            assert!(registry.holds_attested_solve(mode, &at_admission), "not released with custody");
        }
        let reset = registry.dispose_all_results();
        assert_eq!(reset.released_results, 0);
        assert!(registry.table().attested.job_numbers().is_empty());
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    /// The starting grant waits in the solve's queue (`wait_acquire`, the
    /// production entry a solve start uses) for at most the check's window,
    /// then the queue reason stands: with no worker slot free the check is
    /// refused `worker_limit_reached`, after the window, and nothing stays reserved.
    #[test]
    fn model_checks_wait_for_a_worker_slot_then_refuse_by_type() {
        let host = ScriptedHost::normal();
        let registry = registry(&host, TestLauncher::solve());
        registry.pool().set_max_running_workers(0);
        let demo: serde_json::Value = serde_json::from_str(crate::BUNDLED_DEMO_MODEL).unwrap();
        let input = registry.admit_model_checks(&checks_body(&demo)).unwrap();
        let started = std::time::Instant::now();
        match registry.run_model_checks(input) {
            Err(ModelChecksError::Unavailable(SolveJobError::ResourceUnavailable { refusal: ResourceRefusal::WorkerLimitReached { limit: 0, .. }, .. })) => {}
            other => panic!("expected worker_limit_reached after the wait, got {other:?}"),
        }
        assert!(started.elapsed() >= MODEL_CHECKS_START_WAIT, "{:?}", started.elapsed());
        assert_eq!((registry.pool().reserved_total(), registry.pool().running_workers()), (0, 0));
    }

    /// Runs one admitted check on its own thread, as `check_preview_model`'s
    /// blocking task does.
    fn spawn_check(registry: &SolveJobRegistry) -> std::thread::JoinHandle<Result<ModelChecksOutput, ModelChecksError>> {
        let input = registry.admit_model_checks(&checks_body(&serde_json::json!({"project": {"id": "p"}}))).unwrap();
        let registry = registry.clone();
        std::thread::spawn(move || registry.run_model_checks(input))
    }

    /// The check's outcome, failing the test if it has not ended within 30 s.
    fn joined(check: std::thread::JoinHandle<Result<ModelChecksOutput, ModelChecksError>>) -> Result<ModelChecksOutput, ModelChecksError> {
        let deadline = std::time::Instant::now() + Duration::from_secs(30);
        while !check.is_finished() {
            assert!(std::time::Instant::now() < deadline, "the check did not end");
            std::thread::sleep(Duration::from_millis(2));
        }
        check.join().unwrap()
    }

    /// Codex review r3 of #1665: a session reset (page load) cancels a running
    /// check, as it abandons a solve: its worker is killed and, after the reap,
    /// its grant released; nothing stays reserved.
    #[test]
    fn a_session_reset_cancels_a_running_check_and_releases_its_grant() {
        let host = ScriptedHost::normal();
        let (registry, gate) = gated(&host);
        let check = spawn_check(&registry);
        gate.wait_started(1);
        assert_eq!(registry.pool().running_workers(), 1);
        assert!(registry.pool().reserved_total() > 0);
        registry.dispose_all_results();
        assert!(matches!(joined(check), Err(ModelChecksError::Cancelled)));
        assert_eq!((registry.pool().reserved_total(), registry.pool().running_workers()), (0, 0));
        assert!(!registry.cancel_model_checks(), "the slot is clear once the check has ended");
    }

    /// A newer check supersedes the one in progress: admitting it cancels the
    /// older check, whose worker is killed, and the newer one runs and reports.
    #[test]
    fn a_newer_check_cancels_the_one_in_progress() {
        let host = ScriptedHost::normal();
        let (registry, gate) = gated(&host);
        let older = spawn_check(&registry);
        gate.wait_started(1);
        let newer = spawn_check(&registry);
        assert!(matches!(joined(older), Err(ModelChecksError::Cancelled)));
        gate.wait_started(2);
        gate.release("{\"newer\":true}");
        let output = joined(newer).unwrap_or_else(|e| panic!("{e:?}"));
        assert_eq!(output.report, br#"{"newer":true}"#);
        drop(output);
        assert_eq!((registry.pool().reserved_total(), registry.pool().running_workers()), (0, 0));
    }

    /// A solve queued behind a running check (one worker slot) starts once the
    /// project session's change cancels the check (`cancel_model_checks`); a
    /// check still waiting for its starting grant stops waiting at once.
    #[test]
    fn a_solve_queued_behind_a_cancelled_check_starts() {
        let host = ScriptedHost::normal();
        let (registry, gate) = gated(&host);
        registry.pool().set_max_running_workers(1);
        let check = spawn_check(&registry);
        gate.wait_started(1);
        let receipt = registry.start(MODEL, None).unwrap();
        let queued = wait_for(&registry, &receipt.job_id, |s| s.queue_reason.is_some());
        assert_eq!(queued.state, "queued", "{queued:?}");
        // A second check waits for the slot too; it is cancelled before its window ends.
        let waiting = spawn_check(&registry);
        assert!(matches!(joined(check), Err(ModelChecksError::Cancelled)), "superseded");
        let cancelled_at = std::time::Instant::now();
        assert!(registry.cancel_model_checks());
        assert!(matches!(joined(waiting), Err(ModelChecksError::Cancelled)));
        assert!(cancelled_at.elapsed() < MODEL_CHECKS_START_WAIT, "{:?}", cancelled_at.elapsed());
        gate.wait_started(2);
        assert_eq!(wait_for(&registry, &receipt.job_id, |s| s.state == "running").state, "running");
        gate.release("{}");
        assert_eq!(wait_terminal(&registry, &receipt.job_id).state, "completed");
        registry.dispose_all_results();
        assert_eq!((registry.pool().reserved_total(), registry.pool().running_workers()), (0, 0));
    }

    #[test]
    fn sizes_in_messages_are_exact() {
        assert_eq!(display_size(67_108_864), "67108864 bytes (64 MiB)");
        assert_eq!(display_size(67_108_865), "67108865 bytes (64 MiB)");
        assert_eq!(display_size(3_950_000_000), "3950000000 bytes (3.68 GiB)");
        assert_eq!(display_size(0), "0 bytes (0 MiB)");
    }

    /// The displayable-result limit (RT-U4): a fixed 64 MiB product value.
    #[test]
    fn the_displayable_result_limit_is_64_mib() {
        assert_eq!(MAX_DISPLAYABLE_RESULT_BYTES, 67_108_864);
        let host = ScriptedHost::normal();
        let (registry, _gate) = gated(&host);
        assert_eq!(registry.result_limit, MAX_DISPLAYABLE_RESULT_BYTES);
    }

    /// At the seam's limit L: a result of L - 1 and of L bytes publishes and
    /// is taken as before; one of L + 1 bytes ends the job refused with the
    /// code, its size and the limit, holds no result or lease, and its take
    /// refuses with the same code and returns no bytes.
    #[test]
    fn a_result_above_the_limit_is_refused_and_never_taken() {
        const LIMIT: u64 = 16;
        let host = ScriptedHost::normal();
        let (registry, gate) = gated(&host);
        let registry = registry.with_result_limit(LIMIT);
        for (n, length) in [LIMIT - 1, LIMIT].into_iter().enumerate() {
            let receipt = registry.start(MODEL, None).unwrap();
            gate.wait_started(n + 1);
            let result = "x".repeat(length as usize);
            gate.release(&result);
            let status = wait_terminal(&registry, &receipt.job_id);
            assert_eq!((status.state, status.result_bytes, status.result_too_large), ("completed", Some(length), None), "{status:?}");
            assert_eq!(registry.take_result(&receipt.job_id).unwrap(), result.as_bytes());
            registry.dispose_result(&receipt.job_id).unwrap();
        }
        assert_eq!(registry.pool().reserved_total(), 0);

        let receipt = registry.start(MODEL, None).unwrap();
        gate.wait_started(3);
        gate.release(&"x".repeat(LIMIT as usize + 1));
        let status = wait_terminal(&registry, &receipt.job_id);
        assert_eq!(status.state, "refused", "{status:?}");
        assert_eq!(status.result_too_large, Some(ResultTooLarge { code: RESULT_TOO_LARGE_CODE, result_bytes: LIMIT + 1, limit_bytes: LIMIT }));
        let message = format!(
            "PREVIEW-SOLVE-RESULT-TOO-LARGE: {}'s result is 17 bytes, above the 16 bytes the desktop can display; it was discarded without being read",
            receipt.job_id
        );
        assert_eq!(status.error_message.as_deref(), Some(message.as_str()));
        // Not a memory-pool refusal, and no result is published or retained.
        assert_eq!((status.resource_refusal, status.result_available, status.result_bytes, status.delivered), (None, false, None, false));
        assert_eq!(registry.pool().status(JobCounts::default()).last_refusal, None);
        // Its worker was reaped and its grant released; it never held a result lease.
        assert_eq!((registry.pool().reserved_total(), registry.pool().running_workers()), (0, 0));
        assert_eq!(registry.pool_status().retained_results, 0);
        assert_eq!(registry.custody_binding(&receipt.job_id), None);
        assert!(registry.table().records[&receipt.job_id].digests.is_none());
        // The take refuses with the same code, every time, and hands over nothing.
        for _ in 0..2 {
            match registry.take_result(&receipt.job_id) {
                Err(SolveJobError::InvalidRequest { message: refused }) => assert_eq!(refused, message),
                other => panic!("{other:?}"),
            }
        }
        assert_eq!(registry.pool().reserved_total(), 0);
        // Disposal finds nothing to release.
        let disposal = registry.dispose_result(&receipt.job_id).unwrap();
        assert_eq!((disposal.released, disposal.released_bytes, disposal.delivered), (false, 0, false));
        // The status stays the refusal.
        assert_eq!(registry.status(&receipt.job_id).unwrap().result_too_large.map(|r| r.result_bytes), Some(LIMIT + 1));
    }

    /// C1 custody mode (seam limit L): a native-custody result above the webview limit publishes
    /// under its result lease (the worker's grant and the pool bound it, unchanged), is never taken
    /// (`PREVIEW-SOLVE-RESULT-NATIVE-CUSTODY`, the result kept), stays readable by the native
    /// readers, and its C2 full copy is refused by name above L, before anything is reserved; a
    /// webview-custody job keeps the refusal at publication. Disposal releases the bytes once.
    #[test]
    fn native_custody_admits_results_above_the_webview_limit_and_never_hands_them_over() {
        const LIMIT: u64 = 16;
        let host = ScriptedHost::normal();
        let (registry, gate) = gated(&host);
        let registry = registry.with_result_limit(LIMIT);
        // Webview custody (the default): refused at publication, as before.
        let webview = registry.start_with_custody(MODEL, None, ResultCustody::Webview).unwrap();
        gate.wait_started(1);
        gate.release(&"x".repeat(LIMIT as usize + 1));
        assert_eq!(wait_terminal(&registry, &webview.job_id).state, "refused");
        // Native custody: L + 1 bytes publish and are held.
        let native = registry.start_with_custody(MODEL, None, ResultCustody::Native).unwrap();
        gate.wait_started(2);
        let text = "y".repeat(LIMIT as usize + 1);
        gate.release(&text);
        let status = wait_terminal(&registry, &native.job_id);
        assert_eq!((status.state, status.result_bytes, status.result_too_large), ("completed", Some(LIMIT + 1), None), "{status:?}");
        let held_lease = registry.pool().reserved(LeasePurpose::Result);
        assert!(held_lease > 0);
        // Never taken: refused by name, the result still held.
        match registry.take_result(&native.job_id) {
            Err(SolveJobError::InvalidRequest { message }) => assert!(message.starts_with(RESULT_NATIVE_CUSTODY_CODE), "{message}"),
            other => panic!("{other:?}"),
        }
        assert_eq!(registry.held_result(&native.job_id).unwrap().text(), text);
        // The C2 full copy above L: refused by name with the sizes, nothing reserved.
        let before = registry.pool().reserved_total();
        match registry.copy_result_within(&native.job_id, LIMIT) {
            Err(SolveJobError::InvalidRequest { message }) => {
                assert!(message.starts_with(&format!("{RESULT_DETAIL_WEBVIEW_LIMIT_CODE} (result_bytes={}, limit_bytes={LIMIT})", LIMIT + 1)), "{message}")
            }
            other => panic!("{other:?}"),
        }
        assert_eq!(registry.pool().reserved_total(), before);
        // At the limit: the exact bytes, its custody lease held until release, one copy at a time.
        let copy = registry.copy_result_within(&native.job_id, LIMIT + 1).unwrap();
        assert_eq!(copy, text.as_bytes());
        assert_eq!(registry.pool().reserved(LeasePurpose::Custody), held_lease);
        assert_eq!(registry.pool_status().webview_custody_results, 1);
        match registry.copy_result_within(&native.job_id, LIMIT + 1) {
            Err(SolveJobError::InvalidRequest { message }) => assert!(message.starts_with(RESULT_FULL_COPY_HELD_CODE), "{message}"),
            other => panic!("{other:?}"),
        }
        assert!(registry.release_full_copy(&native.job_id).unwrap());
        assert!(!registry.release_full_copy(&native.job_id).unwrap());
        assert_eq!(registry.pool().reserved(LeasePurpose::Custody), 0);
        // A copy still held is released with the result's disposal, once.
        registry.copy_result_within(&native.job_id, LIMIT + 1).unwrap();
        let disposal = registry.dispose_result(&native.job_id).unwrap();
        assert!(disposal.released);
        assert_eq!(disposal.released_bytes, 2 * held_lease);
        assert_eq!(registry.pool().reserved_total(), 0);
        assert!(!registry.dispose_result(&native.job_id).unwrap().released);
        // A webview job is taken, not copied.
        let other = registry.start(MODEL, None).unwrap();
        gate.wait_started(3);
        gate.release("z");
        wait_terminal(&registry, &other.job_id);
        assert!(registry.copy_result_within(&other.job_id, LIMIT).is_err());
        registry.dispose_result(&other.job_id).unwrap();
        // The header's values.
        assert_eq!(ResultCustody::from_wire(None), Some(ResultCustody::Webview));
        assert_eq!(ResultCustody::from_wire(Some("native")), Some(ResultCustody::Native));
        assert_eq!(ResultCustody::from_wire(Some("other")), None);
    }

    /// Session reset releases a native result's full-copy lease with the result.
    #[test]
    fn session_reset_releases_native_results_and_their_full_copies() {
        let host = ScriptedHost::normal();
        let (registry, gate) = gated(&host);
        let native = registry.start_with_custody(MODEL, None, ResultCustody::Native).unwrap();
        gate.wait_started(1);
        gate.release("held");
        wait_terminal(&registry, &native.job_id);
        registry.copy_result_within(&native.job_id, MAX_DISPLAYABLE_RESULT_BYTES).unwrap();
        let reset = registry.dispose_all_results();
        assert_eq!(reset.released_results, 1);
        assert_eq!(registry.pool().reserved_total(), 0);
        assert_eq!(registry.pool_status().webview_custody_results, 0);
    }

    /// A refused result's attestation never enters the attestation table; a
    /// result within the limit still records its own (seam limit 16).
    #[test]
    fn a_refused_results_attestation_is_never_recorded() {
        const LIMIT: u64 = 16;
        let host = ScriptedHost::normal();
        let (registry, gate) = gated(&host);
        let registry = registry.with_result_limit(LIMIT);
        let mode = PreviewSolverMode::SparseInteractive;
        let within = "x".repeat(LIMIT as usize);
        let over = "y".repeat(LIMIT as usize + 1);
        let within_attestation = crate::solve_worker::tests::scripted_attestation(&within);
        let over_attestation = crate::solve_worker::tests::scripted_attestation(&over);

        let refused = registry.start(MODEL, Some(mode.as_str())).unwrap();
        gate.wait_started(1);
        gate.release(&format!("attested:{over}"));
        let status = wait_terminal(&registry, &refused.job_id);
        assert_eq!((status.state, status.result_too_large.map(|r| r.result_bytes)), ("refused", Some(LIMIT + 1)), "{status:?}");
        assert!(!registry.holds_attested_solve(mode, &over_attestation));

        let accepted = registry.start(MODEL, Some(mode.as_str())).unwrap();
        gate.wait_started(2);
        gate.release(&format!("attested:{within}"));
        assert_eq!(wait_terminal(&registry, &accepted.job_id).state, "completed");
        assert!(registry.holds_attested_solve(mode, &within_attestation));
        assert!(!registry.holds_attested_solve(mode, &over_attestation));
        registry.dispose_all_results();
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    /// The product limit, refused on the announced length alone: the worker
    /// announces 64 MiB + 1 bytes and then sends none (it sleeps). Reading
    /// them would never finish; the job instead ends refused at once, its
    /// worker killed and reaped, with no result lease admitted.
    #[test]
    fn the_product_limit_refuses_on_the_announced_length_before_reading() {
        let length = MAX_DISPLAYABLE_RESULT_BYTES + 1;
        // Within the starting grant, so the grant check does not refuse it first.
        assert!(length <= start_grant(), "{length} > {}", start_grant());
        let host = ScriptedHost::normal();
        let registry = registry(&host, TestLauncher::scripted(format!("oversized_header:{length}")));
        let receipt = registry.start(MODEL, None).unwrap();
        let status = wait_terminal(&registry, &receipt.job_id);
        assert_eq!(status.state, "refused", "{status:?}");
        assert_eq!(
            status.result_too_large,
            Some(ResultTooLarge { code: RESULT_TOO_LARGE_CODE, result_bytes: 67_108_865, limit_bytes: 67_108_864 })
        );
        assert!(status.error_message.unwrap().starts_with("PREVIEW-SOLVE-RESULT-TOO-LARGE: "));
        assert_eq!((registry.pool().reserved_total(), registry.pool().running_workers()), (0, 0));
        assert!(matches!(registry.take_result(&receipt.job_id), Err(SolveJobError::InvalidRequest { message }) if message.starts_with("PREVIEW-SOLVE-RESULT-TOO-LARGE: ")));
    }

    /// A result of exactly the product limit is published and taken unchanged.
    #[test]
    fn a_result_of_exactly_the_product_limit_is_published_and_taken() {
        let host = ScriptedHost::normal();
        let registry = registry(&host, TestLauncher::scripted(format!("result:{MAX_DISPLAYABLE_RESULT_BYTES}")));
        let receipt = registry.start(MODEL, None).unwrap();
        let status = wait_terminal(&registry, &receipt.job_id);
        assert_eq!((status.state, status.result_bytes, status.result_too_large), ("completed", Some(MAX_DISPLAYABLE_RESULT_BYTES), None), "{status:?}");
        let taken = registry.take_result(&receipt.job_id).unwrap();
        assert_eq!(taken.len() as u64, MAX_DISPLAYABLE_RESULT_BYTES);
        assert!(taken.iter().all(|byte| *byte == b'x'));
        drop(taken);
        registry.dispose_result(&receipt.job_id).unwrap();
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    /// Publication holds the same bound for a result that reaches it: the
    /// result and its lease are released and the job is refused. (Driven
    /// directly on a job whose worker never started, so nothing else
    /// finishes it.)
    #[test]
    fn publication_refuses_a_result_above_the_limit() {
        struct NoWorker;
        impl WorkerLauncher for NoWorker {
            fn command(&self) -> std::io::Result<std::process::Command> {
                Err(std::io::Error::other("no worker for a publication test"))
            }
        }
        const LIMIT: u64 = 4;
        let host = ScriptedHost::normal();
        let registry = registry(&host, Arc::new(NoWorker)).with_result_limit(LIMIT);
        let receipt = registry.start(MODEL, None).unwrap();
        wait_terminal(&registry, &receipt.job_id);
        assert_eq!(registry.pool().reserved_total(), 0);
        let lease = registry.pool().acquire_interactive(4096).unwrap();
        assert_eq!(registry.pool().reserved_total(), 4096);
        // An attested result: its attestation must not enter the table.
        let attestation = crate::solve_worker::tests::scripted_attestation("12345");
        registry.publish(&receipt.job_id, "12345".to_string(), lease, [0; 32], Some((PreviewSolverMode::SparseInteractive, attestation)));
        assert!(!registry.holds_attested_solve(PreviewSolverMode::SparseInteractive, &attestation));
        let status = registry.status(&receipt.job_id).unwrap();
        assert_eq!((status.state, status.result_too_large.map(|r| (r.result_bytes, r.limit_bytes))), ("refused", Some((5, LIMIT))));
        assert_eq!((status.result_available, status.result_bytes), (false, None));
        assert_eq!(registry.pool().reserved_total(), 0, "the result's lease is released at once");
        assert!(matches!(registry.take_result(&receipt.job_id), Err(SolveJobError::InvalidRequest { message }) if message.starts_with("PREVIEW-SOLVE-RESULT-TOO-LARGE: ")));
    }
}
