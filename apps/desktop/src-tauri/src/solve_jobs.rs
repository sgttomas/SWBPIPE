//! The desktop's solve-job registry under the host memory pool (DEL-04-07
//! memory-budget decision).
//!
//! Ownership and accounting, in order:
//! 1. Ingress: the raw body is prescanned without allocating
//!    ([`crate::ingress::prescan`]); an ingress lease of the raw body's
//!    allocation envelope plus the materialization bound is
//!    admitted against the per-invocation budget and the pool *before*
//!    `serde_json::from_slice` materializes the model. The per-invocation
//!    budget bounds this ingress lease only: the solve stage reserves nothing,
//!    and result and custody leases are bounded by the pool, a fresh host
//!    headroom verdict and the retention/custody count limits, not by it.
//! 2. Queue: the admitted input and its lease move into the worker, which
//!    waits (state `queued`, with the current reason visible to polls) for the
//!    exclusive unpriced ordinary-solve slot.
//! 3. Solve: the ordinary (non-retained) facade runs under the slot and
//!    `catch_unwind`; its stage is unpriced (`ordinary_unpriced`). The input
//!    lease is released when the solve returns.
//! 4. Result: the result buffer is shrunk to fit and a result lease for its
//!    real allocation is admitted; the result is retained (bounded) until
//!    taken. A start is refused early while retention is already full.
//! 5. Handoff: taking the result grows its lease to 2R (handoff copy overlap),
//!    moves the bytes out without cloning, and keeps the 2R custody lease until
//!    the webview disposes its copy. A freshly loaded webview holds no copy and
//!    knows none of the previous page's jobs, so
//!    [`SolveJobRegistry::dispose_all_results`] then releases every retained
//!    result and custody lease and abandons every queued or running job
//!    (cooperative cancellation: it never publishes a result). It never
//!    releases a live job's owners itself; the worker drops them.
//!
//! Locks: the registry lock may be held while taking the pool lock (lease
//! drop/grow); the pool never calls back into the registry under its lock.
//! Every owner drops before a record shows a terminal state.

use crate::ingress;
use crate::memory_pool::{
    JobCounts, Lease, LeasePurpose, MemoryPool, PoolStatus, ResourceRefusal, WaitOutcome, MAX_QUEUED_JOBS,
    MAX_RETAINED_RESULTS, MAX_TERMINAL_RECORDS, MAX_WEBVIEW_CUSTODY_RESULTS,
};
use open_pipe_stress_product_physics::PreviewSolverMode;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) const SOLVE_JOB_CANCELLATION_SCOPE: &str = "cooperative_checkpoints_not_preemptive";
pub(crate) const SOLVER_MODE_HEADER: &str = "x-swbpipe-solver-mode";
const ROUTE_PRICING: &str = "ordinary_unpriced";
const CANCELLED_BEFORE_START: &str = "cancelled_before_solver_start";
const CANCELLED_AT_PUBLICATION: &str = "cancelled_before_result_publication_result_discarded";
/// A live job abandoned by [`SolveJobRegistry::dispose_all_results`] (page reload).
const ABANDONED_AWAITING: &str = "abandoned_by_session_reset_awaiting_cooperative_checkpoint";
const ABANDONED_BEFORE_START: &str = "abandoned_by_session_reset_before_solver_start";
const ABANDONED_AT_PUBLICATION: &str = "abandoned_by_session_reset_result_discarded";

/// Where a cancelled job stopped.
#[derive(Clone, Copy)]
enum CancelPoint {
    BeforeStart,
    AtPublication,
}

/// The ordinary solve: model document and mode to the producer's exact bytes.
pub(crate) type SolveFn = Arc<dyn Fn(Value, PreviewSolverMode) -> Result<String, String> + Send + Sync>;
/// Starts a worker; on failure the job closure (and everything it owns) is dropped.
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
    /// The per-invocation budget at admission; it capped the ingress lease only.
    pub invocation_budget_bytes: u64,
    pub pool_bytes: u64,
    pub route_pricing: &'static str,
    /// Bytes reserved by the ingress lease at admission (raw body envelope + materialization bound).
    pub ingress_reserved_bytes: u64,
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

/// The bytes a result lease reserves for a result buffer: on macOS the
/// allocator's usable size of the block (the desktop uses the system
/// allocator), elsewhere the allocation envelope of its capacity. A reused
/// cached block can report more than the buffer can make resident; reserving
/// it over-prices, never under-prices.
fn result_allocation_bytes(result: &String) -> u64 {
    let capacity = result.capacity() as u64;
    let envelope = ingress::allocation_envelope(capacity).unwrap_or(u64::MAX);
    #[cfg(target_os = "macos")]
    if capacity > 0 {
        // SAFETY: a non-zero-capacity String owns a live heap block from the
        // global allocator (the system malloc); malloc_size only reads it.
        let usable = unsafe { libc::malloc_size(result.as_ptr() as *const libc::c_void) } as u64;
        // 0 means the block is not malloc's: fall back to the envelope.
        if usable > 0 {
            return usable.max(capacity);
        }
    }
    envelope
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
    /// The producer's exact bytes while retained (not yet taken).
    result: Option<String>,
    /// R while retained; 2R custody after delivery until disposal.
    result_lease: Option<Lease>,
    result_bytes: Option<u64>,
    delivered: bool,
    resource: JobResource,
}

#[derive(Debug, Default)]
struct SolveJobTable {
    next_job_number: u64,
    records: HashMap<String, SolveJobRecord>,
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
            .filter(|(_, r)| r.state.terminal() && r.result.is_none() && r.result_lease.is_none())
            .map(|(id, r)| (r.number, id.clone()))
            .collect();
        candidates.sort_unstable();
        for (_, id) in candidates.into_iter().take(terminal - MAX_TERMINAL_RECORDS) {
            self.records.remove(&id);
        }
    }
}

/// An admitted model and the lease covering it; moves into the worker.
struct AdmittedInput {
    model: Value,
    lease: Lease,
}

#[derive(Clone)]
pub(crate) struct SolveJobRegistry {
    jobs: Arc<Mutex<SolveJobTable>>,
    pool: Arc<MemoryPool>,
    solve: SolveFn,
    spawn: SpawnFn,
}

fn thread_spawn(job: Box<dyn FnOnce() + Send + 'static>) -> std::io::Result<()> {
    std::thread::Builder::new().name("swbpipe-solve-job".into()).spawn(job).map(drop)
}

fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    payload
        .downcast_ref::<&str>()
        .map(|s| s.to_string())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "non-string panic payload".to_string())
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
                record.queue_reason = None;
                record.error_message =
                    Some("PREVIEW-SOLVE-WORKER-EXITED: the solve worker ended without publishing an outcome.".into());
            }
        }
    }
}

impl SolveJobRegistry {
    pub(crate) fn new(pool: Arc<MemoryPool>) -> Self {
        Self::with_hooks(pool, Arc::new(crate::solve_preview_mechanics_with_mode), Arc::new(thread_spawn))
    }

    pub(crate) fn with_hooks(pool: Arc<MemoryPool>, solve: SolveFn, spawn: SpawnFn) -> Self {
        Self { jobs: Arc::default(), pool, solve, spawn }
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
        let required = bound.lease_bytes().unwrap_or(u64::MAX);
        if required > budget {
            return Err(self.unavailable(ResourceRefusal::IngressExceedsBudget { required_bytes: required, budget_bytes: budget }));
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
        // The reservation precedes the native Value.
        let lease = self.pool.acquire(required, LeasePurpose::Ingress).map_err(|refusal| self.unavailable(refusal))?;
        let model: Value = serde_json::from_slice(body)
            .map_err(|error| SolveJobError::invalid(format!("PREVIEW-SOLVE-MODEL-INVALID: {error}")))?;
        let input = AdmittedInput { model, lease };
        let resource = JobResource {
            invocation_budget_bytes: budget,
            pool_bytes,
            route_pricing: ROUTE_PRICING,
            ingress_reserved_bytes: required,
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
                    result: None,
                    result_lease: None,
                    result_bytes: None,
                    delivered: false,
                    resource: resource.clone(),
                },
            );
            (job_id, token)
        };

        let worker = {
            let registry = self.clone();
            let job_id = job_id.clone();
            move || registry.run_worker(job_id, input, mode)
        };
        let mut state = JobState::Queued;
        if let Err(error) = (self.spawn)(Box::new(worker)) {
            // The closure, its input and its lease have been dropped.
            let mut table = self.table();
            if let Some(record) = table.records.get_mut(&job_id) {
                record.state = JobState::Failed;
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

    /// Queued -> running unless cancellation was requested.
    fn mark_running(&self, job_id: &str) -> bool {
        let mut table = self.table();
        match table.records.get_mut(job_id) {
            Some(record) if !record.cancellation_requested => {
                record.state = JobState::Running;
                record.queue_reason = None;
                true
            }
            _ => false,
        }
    }

    /// Records a terminal state; callers have already dropped the worker's owners.
    fn finish(&self, job_id: &str, edit: impl FnOnce(&mut SolveJobRecord)) {
        let mut table = self.table();
        if let Some(record) = table.records.get_mut(job_id) {
            record.queue_reason = None;
            edit(record);
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

    fn run_worker(self, job_id: String, input: AdmittedInput, mode: PreviewSolverMode) {
        let _exit = WorkerExit { registry: self.clone(), job_id: job_id.clone() };
        let mut input = Some(input);
        let slot = match self.pool.wait_unpriced_solve(&|| self.cancellation_requested(&job_id), &mut |reason| {
            self.set_queue_reason(&job_id, reason)
        }) {
            Ok(slot) => slot,
            Err(WaitOutcome::Cancelled) => {
                drop(input.take());
                self.finish_cancelled(&job_id, CancelPoint::BeforeStart, None);
                return;
            }
        };
        if !self.mark_running(&job_id) {
            drop(input.take());
            drop(slot);
            self.finish_cancelled(&job_id, CancelPoint::BeforeStart, None);
            return;
        }
        let Some(AdmittedInput { model, lease }) = input.take() else { return };
        let solve = Arc::clone(&self.solve);
        // The input and its lease are owned by the closure: a panic drops them
        // during unwinding; a return drops the lease as soon as the solve no
        // longer needs the input (it consumed the model).
        let outcome = catch_unwind(AssertUnwindSafe(move || {
            let lease = lease;
            let outcome = solve(model, mode);
            drop(lease);
            outcome
        }));
        match outcome {
            Err(payload) => {
                let message = panic_message(payload.as_ref());
                drop(payload);
                drop(slot);
                self.finish(&job_id, |record| {
                    record.state = JobState::Failed;
                    record.error_message = Some(format!("PREVIEW-SOLVE-PANICKED: {message}"));
                });
            }
            Ok(Err(error)) => {
                drop(slot);
                if self.cancellation_requested(&job_id) {
                    self.finish_cancelled(&job_id, CancelPoint::AtPublication, Some(error));
                } else {
                    self.finish(&job_id, |record| {
                        record.state = JobState::Failed;
                        record.error_message = Some(error);
                    });
                }
            }
            Ok(Ok(result)) => self.publish(&job_id, result, slot),
        }
    }

    fn publish(&self, job_id: &str, mut result: String, slot: crate::memory_pool::UnpricedSolveSlot) {
        if self.cancellation_requested(job_id) {
            drop(result);
            drop(slot);
            self.finish_cancelled(job_id, CancelPoint::AtPublication, None);
            return;
        }
        // Shrink the buffer (up to 2x its length after serialization) to fit,
        // still inside the unpriced solve stage, then price its real
        // allocation before the slot is released.
        result.shrink_to_fit();
        let lease = match self.pool.acquire(result_allocation_bytes(&result), LeasePurpose::Result) {
            Ok(lease) => lease,
            Err(refusal) => {
                drop(result);
                drop(slot);
                self.finish(job_id, |record| {
                    record.state = JobState::Refused;
                    record.resource_refusal = Some(refusal);
                });
                return;
            }
        };
        drop(slot);
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
        record.result_bytes = Some(result.len() as u64);
        record.result = Some(result);
        record.result_lease = Some(lease);
        table.prune();
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
            resource: record.resource.clone(),
        })
    }

    /// Hands the producer's exact bytes over (moved, not cloned) after growing
    /// the result lease to 2R for the handoff copy; on refusal the result stays.
    pub(crate) fn take_result(&self, job_id: &str) -> Result<Vec<u8>, SolveJobError> {
        let mut table = self.table();
        let custody = table.counts().custody;
        let record = table.records.get(job_id).ok_or_else(|| SolveJobError::invalid(format!("unknown solve job: {job_id}")))?;
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
                &table,
                ResourceRefusal::CustodyFull { delivered: custody, limit: MAX_WEBVIEW_CUSTODY_RESULTS },
            ));
        }
        let record = table.records.get_mut(job_id).expect("checked above");
        let lease = record.result_lease.as_mut().expect("a retained result holds its lease");
        let grown = lease.bytes().saturating_mul(2);
        if let Err(refusal) = lease.grow_to(grown) {
            return Err(self.unavailable_locked(&table, refusal));
        }
        lease.set_purpose(LeasePurpose::Custody);
        record.delivered = true;
        let result = record.result.take().expect("checked above");
        Ok(result.into_bytes())
    }

    /// The webview's acknowledgement that it dropped its copy, or disposal of
    /// a retained result never taken. Releases the job's result/custody lease.
    pub(crate) fn dispose_result(&self, job_id: &str) -> Result<SolveResultDisposal, SolveJobError> {
        let mut table = self.table();
        let record = table.records.get_mut(job_id).ok_or_else(|| SolveJobError::invalid(format!("unknown solve job: {job_id}")))?;
        let result = record.result.take();
        let lease = record.result_lease.take();
        let released_bytes = lease.as_ref().map_or(0, Lease::bytes);
        let receipt = SolveResultDisposal {
            job_id: job_id.to_string(),
            released: result.is_some() || lease.is_some(),
            released_bytes,
            delivered: record.delivered,
        };
        drop(result);
        drop(lease);
        table.prune();
        Ok(receipt)
    }

    /// Session reset, called when a freshly loaded webview provably holds no
    /// result copy and no handle on the previous page's jobs. Releases every
    /// retained-but-untaken result and every webview custody lease, and
    /// abandons every queued or running job: cancellation is requested (status
    /// `abandoned_by_session_reset_*`), so a queued job exits without solving
    /// and a running one discards its result at publication instead of filling
    /// retention nobody can take. A live job's owners (input lease, solve slot)
    /// are not released here; its worker drops them, then marks it cancelled.
    /// Publication re-checks cancellation under this registry lock, so no
    /// abandoned job can publish after this returns.
    pub(crate) fn dispose_all_results(&self) -> SolveResultsDisposal {
        let mut table = self.table();
        let mut released = Vec::new();
        let mut abandoned_jobs = 0;
        for record in table.records.values_mut() {
            if !record.state.terminal() {
                if !record.abandoned {
                    record.abandoned = true;
                    record.cancellation_requested = true;
                    record.cancellation_status = ABANDONED_AWAITING.into();
                    abandoned_jobs += 1;
                }
                continue;
            }
            let result = record.result.take();
            let lease = record.result_lease.take();
            if result.is_some() || lease.is_some() {
                released.push((result, lease));
            }
        }
        let receipt = SolveResultsDisposal {
            released_results: released.len(),
            released_bytes: released.iter().filter_map(|(_, lease)| lease.as_ref()).map(Lease::bytes).sum(),
            abandoned_jobs,
        };
        drop(released);
        table.prune();
        if abandoned_jobs > 0 {
            self.pool.wake_waiters();
        }
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
        // Cancellation releases nothing itself; the worker's owners drop. An
        // abandoned job is already cancelling and keeps its abandonment status.
        record.cancellation_requested = true;
        if !record.abandoned {
            record.cancellation_status = "cancellation_requested_awaiting_cooperative_checkpoint".into();
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
    use std::time::{Duration, Instant};

    pub(crate) fn real_registry(host: Arc<ScriptedHost>) -> SolveJobRegistry {
        SolveJobRegistry::new(MemoryPool::with_wait_tick(host, Duration::from_millis(5)))
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

    /// Runs one job through the real ordinary solve and returns its terminal
    /// status and the taken bytes (if it completed).
    pub(crate) fn run_job(model: &Value, mode: Option<&str>) -> (SolveJobStatusReport, Option<String>) {
        let registry = real_registry(ScriptedHost::normal());
        let body = serde_json::to_vec(model).unwrap();
        let receipt = registry.start(&body, mode).unwrap_or_else(|e| panic!("{e:?}"));
        let status = wait_terminal(&registry, &receipt.job_id);
        let bytes = (status.state == "completed")
            .then(|| String::from_utf8(registry.take_result(&receipt.job_id).unwrap()).unwrap());
        if bytes.is_some() {
            registry.dispose_result(&receipt.job_id).unwrap();
            assert_eq!(registry.pool().reserved_total(), 0);
        }
        (status, bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::*;
    use super::*;
    use crate::memory_pool::test_host::{ScriptedHost, GIB};
    use crate::memory_pool::PressureLevel;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::mpsc;
    use std::time::Duration;

    const MODEL: &[u8] = br#"{"project":{"id":"p"},"nodes":[1,2,3]}"#;

    /// A solve hook that blocks until released and reports start/finish.
    struct GatedSolve {
        started: mpsc::Receiver<()>,
        release: mpsc::Sender<Result<String, String>>,
        calls: Arc<AtomicUsize>,
    }

    fn gated_registry(host: &Arc<ScriptedHost>) -> (SolveJobRegistry, GatedSolve) {
        let (started_tx, started) = mpsc::channel();
        let (release, release_rx) = mpsc::channel::<Result<String, String>>();
        let release_rx = Arc::new(Mutex::new(release_rx));
        let calls = Arc::new(AtomicUsize::new(0));
        let counter = calls.clone();
        let solve: SolveFn = Arc::new(move |_model, _mode| {
            counter.fetch_add(1, Ordering::SeqCst);
            started_tx.send(()).unwrap();
            let outcome = release_rx.lock().unwrap().recv().unwrap();
            match outcome {
                Ok(text) if text == "panic" => panic!("scripted solve panic"),
                other => other,
            }
        });
        let pool = MemoryPool::with_wait_tick(host.clone(), Duration::from_millis(5));
        (SolveJobRegistry::with_hooks(pool, solve, Arc::new(thread_spawn)), GatedSolve { started, release, calls })
    }

    fn instant_registry(host: &Arc<ScriptedHost>, result: &'static str) -> SolveJobRegistry {
        let pool = MemoryPool::with_wait_tick(host.clone(), Duration::from_millis(5));
        SolveJobRegistry::with_hooks(pool, Arc::new(move |_, _| Ok(result.to_string())), Arc::new(thread_spawn))
    }

    fn started(gate: &GatedSolve) {
        gate.started.recv_timeout(Duration::from_secs(30)).expect("solve started");
    }

    #[test]
    fn job_runs_the_real_ordinary_solve_end_to_end_with_exact_bytes() {
        let model = crate::load_preview_model().unwrap();
        let direct = crate::solve_preview_mechanics_with_mode(model.clone(), PreviewSolverMode::default()).unwrap();
        let registry = real_registry(ScriptedHost::normal());
        let receipt = registry.start(&serde_json::to_vec(&model).unwrap(), None).unwrap();
        assert_eq!((receipt.state, receipt.resource.route_pricing), ("queued", "ordinary_unpriced"));
        assert_eq!(receipt.resource.invocation_budget_bytes, crate::memory_pool::QUALIFIED_INVOCATION_BUDGET_BYTES);
        assert!(receipt.resource.ingress_reserved_bytes > 0);
        let status = wait_terminal(&registry, &receipt.job_id);
        assert_eq!(status.state, "completed", "{status:?}");
        assert!(status.result_available);
        assert_eq!(status.result_bytes, Some(direct.len() as u64));
        assert_eq!(registry.pool().reserved(LeasePurpose::Ingress), 0, "the input lease is released after the solve");
        let retained = registry.pool().reserved(LeasePurpose::Result);
        assert!(retained >= direct.len() as u64);
        let bytes = registry.take_result(&receipt.job_id).unwrap();
        assert_eq!(bytes, direct.as_bytes(), "the producer's exact bytes");
        assert_eq!(registry.pool().reserved(LeasePurpose::Custody), 2 * retained);
        let after = registry.status(&receipt.job_id).unwrap();
        assert!(!after.result_available && after.delivered && after.state == "completed");
        registry.dispose_result(&receipt.job_id).unwrap();
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    #[test]
    fn start_refuses_json_bodies_bad_modes_budget_overruns_and_full_queues() {
        let host = ScriptedHost::normal();
        let (registry, gate) = gated_registry(&host);
        assert!(matches!(registry.start(MODEL, Some("fastest")).unwrap_err(), SolveJobError::InvalidRequest { .. }));
        let invalid = registry.start(b"{\"a\":", None).unwrap_err();
        assert!(matches!(invalid.refusal(), Some(ResourceRefusal::IngressInvalid { .. })));
        let raw_value = registry.start(br#"{"$serde_json::private::RawValue":"1"}"#, None).unwrap_err();
        assert!(matches!(raw_value.refusal(), Some(ResourceRefusal::IngressInvalid { .. })));
        // A lower budget refuses an ingress it cannot cover; nothing is reserved.
        registry.configure_pool(12 * GIB, 64, false).unwrap();
        assert!(matches!(
            registry.start(MODEL, None).unwrap_err().refusal(),
            Some(ResourceRefusal::IngressExceedsBudget { budget_bytes: 64, .. })
        ));
        assert_eq!(registry.pool().reserved_total(), 0);
        registry.configure_pool(12 * GIB, 12 * GIB, false).unwrap();
        // One running + MAX_QUEUED_JOBS queued; the next start refuses queue_full.
        let first = registry.start(MODEL, Some("dense_scrutiny")).unwrap();
        started(&gate);
        let queued: Vec<_> = (0..MAX_QUEUED_JOBS).map(|_| registry.start(MODEL, None).unwrap()).collect();
        let full = registry.start(MODEL, None).unwrap_err();
        assert_eq!(full.refusal(), Some(&ResourceRefusal::QueueFull { queued: MAX_QUEUED_JOBS, limit: MAX_QUEUED_JOBS }));
        for receipt in queued.iter().chain([&first]) {
            registry.cancel(&receipt.job_id, &receipt.backend_cancellation_token).unwrap();
        }
        gate.release.send(Ok("{}".into())).unwrap();
        for receipt in queued.iter().chain([&first]) {
            assert_eq!(wait_terminal(&registry, &receipt.job_id).state, "cancelled");
        }
        assert_eq!(registry.pool().reserved_total(), 0);
        assert_eq!(gate.calls.load(Ordering::SeqCst), 1, "queued jobs never started the solver");
        let error = serde_json::to_value(full).unwrap();
        assert_eq!(error["code"], "RESOURCE_UNAVAILABLE");
        assert_eq!(error["refusal"]["kind"], "queue_full");
        assert!(error["pool"]["pool_bytes"].is_u64());
    }

    #[test]
    fn pressure_warning_queues_then_normal_runs_and_critical_spares_the_running_job() {
        let host = ScriptedHost::normal();
        let (registry, gate) = gated_registry(&host);
        // Another unpriced solve holds the slot while this job is admitted.
        let held = registry.pool().try_begin_unpriced_solve().unwrap();
        let job = registry.start(MODEL, None).unwrap();
        wait_for(&registry, &job.job_id, |s| s.queue_reason == Some(ResourceRefusal::SolveSlotBusy));
        host.set(|s| s.pressure = PressureLevel::Warning);
        drop(held);
        let queued = wait_for(&registry, &job.job_id, |s| {
            s.queue_reason == Some(ResourceRefusal::PressureElevated { level: PressureLevel::Warning })
        });
        assert_eq!(queued.state, "queued");
        assert_eq!(gate.calls.load(Ordering::SeqCst), 0);
        // New admissions refuse at warning pressure; nothing falls back.
        assert_eq!(
            registry.start(MODEL, None).unwrap_err().refusal(),
            Some(&ResourceRefusal::PressureElevated { level: PressureLevel::Warning })
        );
        host.set(|s| s.pressure = PressureLevel::Normal);
        started(&gate);
        assert_eq!(registry.status(&job.job_id).unwrap().state, "running");
        assert_eq!(registry.status(&job.job_id).unwrap().queue_reason, None);
        // Normal -> Critical: new admission refused; the running job is unaffected.
        host.set(|s| s.pressure = PressureLevel::Critical);
        assert_eq!(
            registry.start(MODEL, None).unwrap_err().refusal(),
            Some(&ResourceRefusal::PressureElevated { level: PressureLevel::Critical })
        );
        std::thread::sleep(Duration::from_millis(20));
        assert_eq!(registry.status(&job.job_id).unwrap().state, "running");
        host.set(|s| s.pressure = PressureLevel::Normal);
        gate.release.send(Ok("{\"done\":true}".into())).unwrap();
        assert_eq!(wait_terminal(&registry, &job.job_id).state, "completed");
        registry.dispose_result(&job.job_id).unwrap();

        // Unknown and stale headroom also keep a job queued.
        let held = registry.pool().try_begin_unpriced_solve().unwrap();
        let next = registry.start(MODEL, None).unwrap();
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
        assert_eq!(gate.calls.load(Ordering::SeqCst), 1);
        host.set(|s| s.pressure = PressureLevel::Normal);
        started(&gate);
        gate.release.send(Ok("{}".into())).unwrap();
        assert_eq!(wait_terminal(&registry, &next.job_id).state, "completed");
        registry.dispose_result(&next.job_id).unwrap();
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    #[test]
    fn poll_never_returns_the_result_and_take_grows_the_lease_to_2r() {
        let host = ScriptedHost::normal();
        let registry = instant_registry(&host, "{\"result\":\"exact bytes\"}");
        let receipt = registry.start(MODEL, None).unwrap();
        let status = wait_terminal(&registry, &receipt.job_id);
        let report = serde_json::to_value(&status).unwrap();
        assert!(report.get("result").is_none(), "polls never carry the result");
        for key in [
            "job_id", "state", "queue_reason", "cancellation_requested", "cancellation_status", "cancellation_scope",
            "result_available", "result_bytes", "error_message", "resource_refusal", "resource",
        ] {
            assert!(report.get(key).is_some(), "{key}");
        }
        assert_eq!(report["resource"]["route_pricing"], "ordinary_unpriced");
        assert_eq!(status.result_bytes, Some(24));
        let r = registry.pool().reserved(LeasePurpose::Result);
        assert!(r >= 24);
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
        assert_eq!(bytes, b"{\"result\":\"exact bytes\"}");
        assert_eq!((registry.pool().reserved(LeasePurpose::Result), registry.pool().reserved(LeasePurpose::Custody)), (0, 2 * r));
        let delivered = registry.status(&receipt.job_id).unwrap();
        assert!(!delivered.result_available && delivered.delivered);
        assert!(matches!(registry.take_result(&receipt.job_id).unwrap_err(), SolveJobError::InvalidRequest { .. }));
        let disposal = registry.dispose_result(&receipt.job_id).unwrap();
        assert!(disposal.released && disposal.delivered && disposal.released_bytes == 2 * r);
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    #[test]
    fn retention_and_custody_bounds_refuse_without_losing_held_results() {
        let host = ScriptedHost::normal();
        let (registry, gate) = gated_registry(&host);
        // One running and MAX_QUEUED_JOBS queued are admitted while nothing is
        // retained; the last to publish finds retention full (post-solve check).
        let first = registry.start(MODEL, None).unwrap();
        started(&gate);
        let mut started_jobs: Vec<_> = vec![first.job_id];
        started_jobs.extend((0..MAX_QUEUED_JOBS).map(|_| registry.start(MODEL, None).unwrap().job_id));
        for _ in 0..=MAX_QUEUED_JOBS {
            gate.release.send(Ok("{\"r\":1}".into())).unwrap();
        }
        let (mut jobs, mut refused) = (Vec::new(), Vec::new());
        for job in started_jobs {
            let status = wait_terminal(&registry, &job);
            match status.state {
                "completed" => jobs.push(job),
                _ => refused.push(status),
            }
        }
        while gate.started.try_recv().is_ok() {} // the queued jobs' start signals
        assert_eq!((jobs.len(), refused.len()), (MAX_RETAINED_RESULTS, 1), "{refused:?}");
        assert_eq!(refused[0].state, "refused");
        assert_eq!(refused[0].resource_refusal, Some(ResourceRefusal::RetentionFull { retained: 4, limit: 4 }));
        assert_eq!(registry.pool_status().retained_results, 4);
        let calls = gate.calls.load(Ordering::SeqCst);
        // With retention full, a start is refused early: no reservation, no solve.
        let reserved = registry.pool().reserved_total();
        let early = registry.start(MODEL, None).unwrap_err();
        assert_eq!(early.refusal(), Some(&ResourceRefusal::RetentionFull { retained: 4, limit: 4 }));
        assert_eq!(registry.pool().reserved_total(), reserved);
        assert_eq!(gate.calls.load(Ordering::SeqCst), calls, "the solver never ran");
        // Taking all four fills custody; a fifth take refuses and keeps its result.
        for job in &jobs {
            registry.take_result(job).unwrap();
        }
        let fifth = registry.start(MODEL, None).unwrap();
        started(&gate);
        gate.release.send(Ok("{\"r\":1}".into())).unwrap();
        assert_eq!(wait_terminal(&registry, &fifth.job_id).state, "completed");
        let refusal = registry.take_result(&fifth.job_id).unwrap_err();
        assert_eq!(refusal.refusal(), Some(&ResourceRefusal::CustodyFull { delivered: 4, limit: 4 }));
        assert!(registry.status(&fifth.job_id).unwrap().result_available);
        registry.dispose_result(&jobs[0]).unwrap();
        registry.take_result(&fifth.job_id).unwrap();
        // Disposing a retained-but-untaken result drops it too.
        let untaken = registry.start(MODEL, None).unwrap();
        started(&gate);
        gate.release.send(Ok("{\"r\":1}".into())).unwrap();
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

    #[test]
    fn dispose_all_releases_retained_and_custody_and_abandons_live_jobs() {
        let host = ScriptedHost::normal();
        let (registry, gate) = gated_registry(&host);
        // Two retained results; one of them taken into custody.
        let mut done = Vec::new();
        for _ in 0..2 {
            let receipt = registry.start(MODEL, None).unwrap();
            started(&gate);
            gate.release.send(Ok("{\"r\":1}".into())).unwrap();
            assert_eq!(wait_terminal(&registry, &receipt.job_id).state, "completed");
            done.push(receipt.job_id);
        }
        registry.take_result(&done[0]).unwrap();
        // A running job and a queued job hold their own ingress leases.
        let running = registry.start(MODEL, None).unwrap();
        started(&gate);
        let queued = registry.start(MODEL, None).unwrap();
        let held = registry.pool().reserved(LeasePurpose::Result) + registry.pool().reserved(LeasePurpose::Custody);
        assert!(held > 0);

        let disposal = registry.dispose_all_results();
        assert_eq!(disposal, SolveResultsDisposal { released_results: 2, released_bytes: held, abandoned_jobs: 2 });
        let wire = serde_json::to_value(&disposal).unwrap();
        assert_eq!(wire, serde_json::json!({"released_results": 2, "released_bytes": held, "abandoned_jobs": 2}));
        assert_eq!(registry.pool().reserved(LeasePurpose::Result) + registry.pool().reserved(LeasePurpose::Custody), 0);
        for job in &done {
            let after = registry.status(job).unwrap();
            assert!(!after.result_available && after.state == "completed");
        }

        // The queued job exits without solving; its lease is gone once it shows cancelled.
        let status = wait_terminal(&registry, &queued.job_id);
        assert_eq!((status.state, status.cancellation_status.as_str()), ("cancelled", ABANDONED_BEFORE_START));
        assert!(status.cancellation_requested && !status.result_available);
        assert_eq!(registry.pool().reserved(LeasePurpose::Ingress), running.resource.ingress_reserved_bytes);

        // The running job keeps its lease and the slot until its worker finishes.
        let status = registry.status(&running.job_id).unwrap();
        assert_eq!((status.state, status.cancellation_status.as_str()), ("running", ABANDONED_AWAITING));
        assert!(status.cancellation_requested);
        std::thread::sleep(Duration::from_millis(20));
        assert_eq!(registry.status(&running.job_id).unwrap().state, "running");
        assert_eq!(registry.pool().reserved(LeasePurpose::Ingress), running.resource.ingress_reserved_bytes);
        assert!(registry.pool().unpriced_solve_running());
        // A token-holder's cancel does not overwrite the abandonment.
        let cancel = registry.cancel(&running.job_id, &running.backend_cancellation_token).unwrap();
        assert_eq!(cancel.cancellation_status, ABANDONED_AWAITING);
        // A second reset counts nothing new.
        assert_eq!(registry.dispose_all_results(), SolveResultsDisposal { released_results: 0, released_bytes: 0, abandoned_jobs: 0 });

        gate.release.send(Ok("{\"r\":2}".into())).unwrap();
        let status = wait_terminal(&registry, &running.job_id);
        assert_eq!((status.state, status.cancellation_status.as_str()), ("cancelled", ABANDONED_AT_PUBLICATION));
        assert!(!status.result_available && status.result_bytes.is_none());
        assert_eq!(registry.pool().reserved_total(), 0);
        assert!(!registry.pool().unpriced_solve_running());
        assert_eq!(gate.calls.load(Ordering::SeqCst), 3, "the queued job never started the solver");
        let pool = registry.pool_status();
        assert_eq!((pool.retained_results, pool.webview_custody_results, pool.running_jobs, pool.queued_jobs), (0, 0, 0, 0));

        // The new page's starts are not refused for retention.
        let fresh = registry.start(MODEL, None).unwrap();
        started(&gate);
        gate.release.send(Ok("{\"r\":3}".into())).unwrap();
        assert!(wait_terminal(&registry, &fresh.job_id).result_available);
        let retained = registry.pool().reserved(LeasePurpose::Result);
        assert_eq!(
            registry.dispose_all_results(),
            SolveResultsDisposal { released_results: 1, released_bytes: retained, abandoned_jobs: 0 }
        );
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    #[test]
    fn dispose_all_keeps_a_full_queue_from_filling_retention() {
        let host = ScriptedHost::normal();
        let (registry, gate) = gated_registry(&host);
        // The old page: one running and a full queue, enough to fill retention.
        let first = registry.start(MODEL, None).unwrap();
        started(&gate);
        let mut old: Vec<_> = vec![first.job_id];
        old.extend((0..MAX_QUEUED_JOBS).map(|_| registry.start(MODEL, None).unwrap().job_id));
        assert_eq!(registry.dispose_all_results().abandoned_jobs, MAX_QUEUED_JOBS + 1);
        gate.release.send(Ok("{\"r\":1}".into())).unwrap();
        for job in &old {
            let status = wait_terminal(&registry, job);
            assert_eq!(status.state, "cancelled", "{status:?}");
            assert!(status.cancellation_status.starts_with("abandoned_by_session_reset"));
        }
        assert_eq!(gate.calls.load(Ordering::SeqCst), 1, "abandoned queued jobs never solved");
        assert_eq!(registry.pool().reserved_total(), 0);
        assert_eq!(registry.pool_status().retained_results, 0);
        // The new page can start and retain as usual.
        let fresh = registry.start(MODEL, None).unwrap();
        started(&gate);
        gate.release.send(Ok("{\"r\":2}".into())).unwrap();
        assert_eq!(wait_terminal(&registry, &fresh.job_id).state, "completed");
        registry.dispose_result(&fresh.job_id).unwrap();
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    #[test]
    fn result_lease_covers_the_shrunk_buffer_not_its_spare_capacity() {
        let host = ScriptedHost::normal();
        let pool = MemoryPool::with_wait_tick(host.clone(), Duration::from_millis(5));
        let solve: SolveFn = Arc::new(|_, _| {
            let mut text = String::with_capacity(1 << 20);
            text.push_str("{\"r\":1}");
            Ok(text)
        });
        let registry = SolveJobRegistry::with_hooks(pool, solve, Arc::new(thread_spawn));
        let receipt = registry.start(MODEL, None).unwrap();
        assert_eq!(wait_terminal(&registry, &receipt.job_id).state, "completed");
        let leased = registry.pool().reserved(LeasePurpose::Result);
        assert!((7..=16).contains(&leased), "lease {leased} covers the 7-byte result's block, not 1 MiB");
        assert_eq!(registry.take_result(&receipt.job_id).unwrap(), b"{\"r\":1}");
        registry.dispose_result(&receipt.job_id).unwrap();
        assert_eq!(registry.pool().reserved_total(), 0);
        // The lease covers the block's real size.
        let text = String::from("x").repeat(32_769);
        let bytes = result_allocation_bytes(&text);
        assert!(bytes >= text.capacity() as u64);
        #[cfg(target_os = "macos")]
        // SAFETY: a live heap block owned by `text`.
        assert_eq!(bytes, unsafe { libc::malloc_size(text.as_ptr() as *const libc::c_void) } as u64);
        assert_eq!(result_allocation_bytes(&String::new()), 0);
    }

    #[test]
    fn result_lease_refusal_marks_the_job_refused_and_drops_the_result() {
        let host = ScriptedHost::normal();
        let (registry, gate) = gated_registry(&host);
        let receipt = registry.start(MODEL, None).unwrap();
        started(&gate);
        host.set(|s| s.headroom_bytes = Some(0));
        gate.release.send(Ok("{\"big\":true}".into())).unwrap();
        let status = wait_terminal(&registry, &receipt.job_id);
        assert_eq!(status.state, "refused");
        assert!(matches!(status.resource_refusal, Some(ResourceRefusal::HeadroomInsufficient { .. })));
        assert!(!status.result_available);
        assert_eq!(registry.pool().reserved_total(), 0);
        assert!(!registry.pool().unpriced_solve_running());
    }

    #[test]
    fn cancellation_releases_leases_only_when_the_worker_drops_them() {
        let host = ScriptedHost::normal();
        let (registry, gate) = gated_registry(&host);
        let running = registry.start(MODEL, None).unwrap();
        started(&gate);
        let queued = registry.start(MODEL, None).unwrap();
        let both = running.resource.ingress_reserved_bytes + queued.resource.ingress_reserved_bytes;
        // The running solve still holds its input lease; the queued job holds its own.
        assert_eq!(registry.pool().reserved(LeasePurpose::Ingress), both);

        // Queued cancellation: once the record shows cancelled, its lease is gone.
        let cancel = registry.cancel(&queued.job_id, &queued.backend_cancellation_token).unwrap();
        assert!(cancel.accepted && !cancel.cancellation_success_claimed);
        let status = wait_terminal(&registry, &queued.job_id);
        assert_eq!((status.state, status.cancellation_status.as_str()), ("cancelled", CANCELLED_BEFORE_START));
        assert_eq!(registry.pool().reserved(LeasePurpose::Ingress), running.resource.ingress_reserved_bytes);

        // Mid-run cancellation: the lease stays held while the worker runs.
        let cancel = registry.cancel(&running.job_id, &running.backend_cancellation_token).unwrap();
        assert_eq!(cancel.job_state, "running");
        std::thread::sleep(Duration::from_millis(20));
        assert_eq!(registry.status(&running.job_id).unwrap().state, "running");
        assert_eq!(registry.pool().reserved(LeasePurpose::Ingress), running.resource.ingress_reserved_bytes);
        assert!(registry.pool().unpriced_solve_running());
        gate.release.send(Ok("{\"discarded\":true}".into())).unwrap();
        let status = wait_terminal(&registry, &running.job_id);
        assert_eq!((status.state, status.cancellation_status.as_str()), ("cancelled", CANCELLED_AT_PUBLICATION));
        assert!(!status.result_available);
        assert_eq!(registry.pool().reserved_total(), 0);
        assert!(!registry.pool().unpriced_solve_running());

        // Cancellation after completion is rejected without a success claim.
        let registry = instant_registry(&host, "{}");
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

    #[test]
    fn a_panicking_solve_releases_slot_and_leases_and_records_failed() {
        let host = ScriptedHost::normal();
        let (registry, gate) = gated_registry(&host);
        let receipt = registry.start(MODEL, None).unwrap();
        started(&gate);
        gate.release.send(Ok("panic".into())).unwrap();
        let status = wait_terminal(&registry, &receipt.job_id);
        assert_eq!(status.state, "failed");
        assert!(status.error_message.as_deref().unwrap().starts_with("PREVIEW-SOLVE-PANICKED: scripted solve panic"));
        assert_eq!(registry.pool().reserved_total(), 0);
        assert!(!registry.pool().unpriced_solve_running());
        // The pool still admits the next job.
        let next = registry.start(MODEL, None).unwrap();
        started(&gate);
        gate.release.send(Err("PREVIEW-SOLVE-REFUSED: scripted".into())).unwrap();
        let failed = wait_terminal(&registry, &next.job_id);
        assert_eq!((failed.state, failed.error_message.as_deref()), ("failed", Some("PREVIEW-SOLVE-REFUSED: scripted")));
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    #[test]
    fn a_failed_worker_start_releases_the_ingress_lease() {
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
        let registry = SolveJobRegistry::with_hooks(pool, Arc::new(|_, _| unreachable!("never runs")), spawn);
        let receipt = registry.start(MODEL, None).unwrap();
        assert_eq!(receipt.state, "failed");
        assert!(dropped_before_failure.load(Ordering::SeqCst), "the closure's lease dropped with it");
        let status = registry.status(&receipt.job_id).unwrap();
        assert_eq!(status.state, "failed");
        assert!(status.error_message.unwrap().starts_with("PREVIEW-SOLVE-WORKER-START-FAILED"));
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    #[test]
    fn terminal_records_are_pruned_beyond_the_limit_but_held_results_are_kept() {
        let host = ScriptedHost::normal();
        let registry = instant_registry(&host, "{}");
        let kept = registry.start(MODEL, None).unwrap();
        wait_terminal(&registry, &kept.job_id);
        for _ in 0..(MAX_TERMINAL_RECORDS + 10) {
            let receipt = registry.start(MODEL, None).unwrap();
            wait_terminal(&registry, &receipt.job_id);
            registry.dispose_result(&receipt.job_id).unwrap();
        }
        let table = registry.table();
        assert!(table.records.values().filter(|r| r.state.terminal()).count() <= MAX_TERMINAL_RECORDS);
        assert!(table.records.contains_key(&kept.job_id), "a record holding a result is never pruned");
        drop(table);
        registry.dispose_result(&kept.job_id).unwrap();
        assert_eq!(registry.pool().reserved_total(), 0);
    }
}
