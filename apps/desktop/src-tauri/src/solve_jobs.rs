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
//!    start is refused early while retention is already full.
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
use crate::solve_worker::{
    CurrentExeLauncher, SupervisorHooks, WorkerControl, WorkerLauncher, WorkerOutcome, WorkerTerms, PROCESS_RESERVE_BYTES,
};
use open_pipe_stress_product_physics::PreviewSolverMode;
use serde::Serialize;
use std::collections::HashMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::{SystemTime, UNIX_EPOCH};

/// Cancelling a running job kills its worker process (preemptive); a queued
/// job exits before starting one.
pub(crate) const SOLVE_JOB_CANCELLATION_SCOPE: &str = "worker_process_terminated";
/// A token-holder's cancellation of a running job: its worker is being killed;
/// the job turns `cancelled` (result discarded) only after the worker is reaped.
const CANCEL_REQUESTED_RUNNING: &str = "cancellation_requested_worker_terminating";
/// A token-holder's cancellation of a queued job (it exits before a worker starts).
const CANCEL_REQUESTED_QUEUED: &str = "cancellation_requested_awaiting_cooperative_checkpoint";
pub(crate) const SOLVER_MODE_HEADER: &str = "x-swbpipe-solver-mode";
const ROUTE_PRICING: &str = "ordinary_worker_capped";
const CANCELLED_BEFORE_START: &str = "cancelled_before_solver_start";
const CANCELLED_AT_PUBLICATION: &str = "cancelled_before_result_publication_result_discarded";
/// A live job abandoned by [`SolveJobRegistry::dispose_all_results`] (page reload).
const ABANDONED_AWAITING: &str = "abandoned_by_session_reset_worker_terminating";
const ABANDONED_BEFORE_START: &str = "abandoned_by_session_reset_before_solver_start";
const ABANDONED_AT_PUBLICATION: &str = "abandoned_by_session_reset_result_discarded";

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
    /// Kill access to the job's worker process while it may be unreaped.
    worker: Option<Arc<WorkerControl>>,
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

/// An admitted request: the raw bytes, the lease covering this process's
/// copy of them and the worker's terms. Moves into the supervisor thread.
struct AdmittedInput {
    raw: Vec<u8>,
    lease: Lease,
    mode: PreviewSolverMode,
    start_grant: u64,
    max_grant: u64,
}

#[derive(Clone)]
pub(crate) struct SolveJobRegistry {
    jobs: Arc<Mutex<SolveJobTable>>,
    pool: Arc<MemoryPool>,
    launcher: Arc<dyn WorkerLauncher>,
    spawn: SpawnFn,
    /// The worker's process reserve (non-heap footprint allowance).
    reserve: u64,
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
        Self::with_hooks(pool, Arc::new(CurrentExeLauncher), Arc::new(thread_spawn))
    }

    pub(crate) fn with_hooks(pool: Arc<MemoryPool>, launcher: Arc<dyn WorkerLauncher>, spawn: SpawnFn) -> Self {
        Self { jobs: Arc::default(), pool, launcher, spawn, reserve: PROCESS_RESERVE_BYTES }
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
                    result: None,
                    result_lease: None,
                    result_bytes: None,
                    delivered: false,
                    resource: resource.clone(),
                    worker: None,
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
        let grant = self.pool.wait_acquire(
            start_grant,
            LeasePurpose::Worker,
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
        let terms = WorkerTerms { pool: &self.pool, launcher: &*self.launcher, mode, max_grant, reserve: self.reserve };
        // The worker is reaped inside `supervise`, also while it unwinds; only
        // then is the grant released.
        let outcome = catch_unwind(AssertUnwindSafe(|| {
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
            Ok(WorkerOutcome::Completed { result, lease }) => match String::from_utf8(result) {
                Ok(result) => self.publish(&job_id, result, lease),
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
            Ok(WorkerOutcome::Terminated) => self.finish_cancelled(&job_id, CancelPoint::AtPublication, None),
        }
    }

    /// Retains a finished result under its result lease, unless the job was
    /// cancelled or retention is full (both drop it).
    fn publish(&self, job_id: &str, result: String, lease: Lease) {
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
        record.worker = None;
        record.resource.worker_grant_bytes = None;
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
    /// `abandoned_by_session_reset_*`), so a queued job exits without starting
    /// a worker and a running job's worker is killed instead of filling
    /// retention nobody can take. A live job's owners (input lease, worker
    /// grant) are not released here; its supervisor reaps the worker, drops
    /// them, then marks it cancelled. Publication re-checks cancellation under
    /// this registry lock, so no abandoned job can publish after this returns.
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
                if let Some(worker) = &record.worker {
                    worker.kill();
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

    /// End to end through the registry: a solve's intake lease and its result
    /// lease each wait out a free-queue dip shorter than the re-sampling window
    /// and the job completes.
    #[test]
    fn intake_and_result_leases_wait_out_a_headroom_dip() {
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
}
