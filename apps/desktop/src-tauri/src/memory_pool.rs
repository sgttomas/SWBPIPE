//! Host-owned aggregate memory ledger (DEL-04-07 `memory-budget-decision.md`,
//! owner-adopted 2026-10-11).
//!
//! Policy (fixed by the owner): a nominal pool of floor(3 * physical / 4)
//! bytes; a per-invocation default of 12 GiB (min(12 GiB, pool)); a lower or
//! larger user budget is allowed up to the pool and never above it; a pool
//! above the recommended fraction needs an explicit acknowledgement and never
//! exceeds physical capacity. Acknowledgement never bypasses headroom or
//! pressure refusals. A budget above 12 GiB needs no separate numerical
//! qualification: the ordinary route runs in the capped worker, whose cap is
//! enforced, and the route, model family and predicates do not change with the
//! budget (the worker's grant arithmetic and protocol are checked up to
//! u64::MAX by `large_budgets_keep_grant_frame_lease_and_envelope_arithmetic_exact`).
//! The retained route (core D1) is not used by the desktop and stays at its
//! registered 12 GiB.
//!
//! What the per-invocation budget bounds: each ordinary solve's worker grant
//! (owner decision 2026-10-12, "(c) Capped worker process"). The solve runs in
//! a local child process whose allocator is capped at its grant; the grant
//! starts at the worker's process reserve plus the request's ingress bound and
//! grows on demand, each step an atomic admission with a fresh headroom
//! verdict, never above the budget (`crate::solve_worker`); and each
//! interactive command's ingress lease (`crate::command_ingress`), including
//! its growth. [`Lease::grow_to`] and [`Lease::grow_resampling_with`] refuse
//! an ingress lease grown above the budget `ingress_exceeds_budget` and a
//! worker grant `worker_cap_exceeded`, under the admission's lock, before
//! the headroom verdict. Result and custody leases are bounded by the pool, a
//! fresh headroom verdict and the retention/custody count limits, not by the
//! budget: a delivered result's 2R custody can exceed it.
//!
//! Every priced admission is atomic: under one lock it checks capacity, the
//! pool's remaining bytes, a fresh host sample (age, pressure, headroom) and
//! records the reservation, so racing admissions serialize and cannot both
//! consume the same pool bytes or headroom. A [`Lease`] releases exactly once
//! when dropped, including during unwinding.
//!
//! Admission classes ([`AdmissionClass`]). Adaptive heavy work (solve intake,
//! worker grants and their growth, results, custody) is refused unless the
//! host's pressure state is normal. Interactive commands (model edits,
//! saves, rule checks, report rendering: `crate::command_ingress`) are not
//! gated by the pressure state: refusing every edit and save at each macOS
//! pressure warning would remove current capability. They still need the
//! aggregate pool reservation, a known capacity, a fresh known headroom
//! covering the request plus the outstanding unrealized reservations (the same
//! rule as every other lease), and their lease must fit the per-invocation
//! budget.
//!
//! Residency credit: a running worker's pages that are already resident were
//! taken from the free queue when they were touched, so the headroom verdict
//! need not require them again. At every admission, under the pool lock, each
//! live worker lease is measured afresh through its [`ResidencyProbe`]
//! (min(resident size, physical footprint) of the worker process; compressed
//! pages are not credited because they occupy no free page) and credited at
//! most its lease bytes; a lease without a probe, a reaped worker or a failed
//! reading is credited 0. Every other reservation is double counted.
//!
//! Re-sampling: the free queue on a loaded Mac swings between about zero and
//! gigabytes within a second while the kernel reports normal pressure (the
//! pageout daemon refills it toward `vm.vm_page_free_target`). A solve's
//! intake lease and a finished solve's result lease
//! ([`MemoryPool::acquire_resampling`], both off the main thread) refused
//! only with `headroom_insufficient` therefore take further fresh samples,
//! every [`HEADROOM_RESAMPLE_TICK`] or sooner on a release, while
//! [`HEADROOM_RESAMPLE_WINDOW`] has not elapsed and the caller is not
//! cancelled, and are admitted at the first sample whose verdict passes.
//! Every verdict is the unchanged atomic rule on a fresh sample; only the
//! moment of sampling moves. Any other refusal, and `headroom_insufficient`
//! once the window has elapsed, stands. Interactive admissions refuse at
//! once (most interactive commands run on the main thread); the desktop
//! re-sends a command refused at admission with `headroom_insufficient`
//! within the same window (`nativeIngress.ts`). A worker's growth step
//! ([`Lease::grow_resampling`]: each attempt tries the doubling and then the
//! minimal need) and the custody grow at take (a separately admitted
//! [`MemoryPool::acquire_resampling`] lease, folded in by [`Lease::absorb`])
//! re-sample in the same way within the same window. Which headroom a sample
//! counts is the host reader's (`host_memory.rs`).

use serde::Serialize;
use std::collections::BTreeMap;
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};

/// The owner-authorized default per-invocation budget: 12 GiB (or the pool,
/// if smaller). A configured budget may be lower or larger, up to the pool. It
/// caps each ordinary solve's worker grant and each interactive command's
/// ingress lease; it does not bound result retention or custody.
pub(crate) const DEFAULT_INVOCATION_BUDGET_BYTES: u64 = 12_884_901_888;
/// A host sample older than this cannot admit work.
pub(crate) const HEADROOM_SAMPLE_MAX_AGE: Duration = Duration::from_secs(1);
/// How often a queued job re-samples the host while waiting for its grant.
pub(crate) const ADMISSION_WAIT_TICK: Duration = Duration::from_millis(200);
/// How long an admission refused only for insufficient headroom keeps taking
/// fresh host samples before the refusal stands (see the module docs).
pub(crate) const HEADROOM_RESAMPLE_WINDOW: Duration = Duration::from_secs(2);
/// The interval between those fresh samples.
pub(crate) const HEADROOM_RESAMPLE_TICK: Duration = Duration::from_millis(50);
/// The default bound on concurrently running solve worker processes, when
/// the host's parallelism cannot be read.
pub(crate) const FALLBACK_MAX_RUNNING_WORKERS: usize = 4;

/// The bound on concurrently running solve worker processes: the host's
/// available parallelism (fallback 4). A process-count safety bound, not a
/// memory policy: memory is bounded by the pool and each worker's grant; this
/// keeps the number of child processes (and their fixed per-process costs)
/// proportionate to the cores that can run them. Jobs beyond it wait in the
/// queue (`worker_limit_reached`), within the unchanged queue bound.
pub(crate) fn max_running_workers() -> usize {
    std::thread::available_parallelism().map_or(FALLBACK_MAX_RUNNING_WORKERS, std::num::NonZeroUsize::get)
}
/// Jobs holding an admitted input while waiting for their worker grant.
pub(crate) const MAX_QUEUED_JOBS: usize = 4;
/// Native results published and not yet taken by the webview.
pub(crate) const MAX_RETAINED_RESULTS: usize = 4;
/// Results delivered to the webview and not yet disposed.
pub(crate) const MAX_WEBVIEW_CUSTODY_RESULTS: usize = 4;
/// Terminal job records kept for polling (records holding no bytes).
pub(crate) const MAX_TERMINAL_RECORDS: usize = 64;

/// floor(3 * capacity / 4) without overflow: (c / 4) * 3 + ((c % 4) * 3) / 4.
pub(crate) fn recommended_pool_bytes(capacity_bytes: u64) -> Option<u64> {
    let quarters = (capacity_bytes / 4).checked_mul(3)?;
    let remainder = (capacity_bytes % 4).checked_mul(3)? / 4;
    quarters.checked_add(remainder)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PressureLevel {
    Normal,
    Warning,
    Critical,
    Unknown,
}

/// One reading of the host. `None` means the reader could not establish the value.
#[derive(Clone, Copy, Debug)]
pub(crate) struct HostSample {
    pub capacity_bytes: Option<u64>,
    pub headroom_bytes: Option<u64>,
    pub pressure: PressureLevel,
    pub taken_at: Instant,
}

impl HostSample {
    fn age(&self) -> Duration {
        Instant::now().saturating_duration_since(self.taken_at)
    }
}

/// A fresh reading of the bytes a running worker already holds resident (and
/// so no longer needs from the free queue); `None` if it cannot be measured or
/// the worker has been reaped. Called under the pool lock; must not take the
/// pool lock.
pub(crate) trait ResidencyProbe: Send + Sync + std::fmt::Debug {
    fn realized_bytes(&self) -> Option<u64>;
}

/// A live worker lease's ledger entry.
#[derive(Debug)]
struct WorkerEntry {
    bytes: u64,
    probe: Option<Arc<dyn ResidencyProbe>>,
}

/// The host's memory reader; the pool samples it inside its critical section.
pub(crate) trait HostMemory: Send + Sync {
    fn sample(&self) -> HostSample;
}

/// A typed resource refusal. Names are stable: the frontend renders them.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub(crate) enum ResourceRefusal {
    CapacityUnknown,
    HeadroomUnknown,
    HeadroomStale { age_ms: u64 },
    HeadroomInsufficient { required_bytes: u64, available_bytes: u64 },
    /// The host's pressure level could not be read.
    PressureUnknown,
    /// Pressure is `warning` or `critical`.
    PressureElevated { level: PressureLevel },
    PoolExhausted { requested_bytes: u64, reserved_bytes: u64, pool_bytes: u64 },
    BudgetExceedsPool { budget_bytes: u64, pool_bytes: u64 },
    PoolAboveCapacity { pool_bytes: u64, capacity_bytes: u64 },
    PoolAboveRecommendedUnacknowledged { pool_bytes: u64, recommended_bytes: u64 },
    IngressExceedsBudget { required_bytes: u64, budget_bytes: u64 },
    IngressInvalid { reason: String },
    QueueFull { queued: usize, limit: usize },
    RetentionFull { retained: usize, limit: usize },
    CustodyFull { delivered: usize, limit: usize },
    /// The solve needed a worker grant above the per-invocation budget.
    WorkerCapExceeded { cap_bytes: u64, requested_bytes: u64 },
    /// A running worker's growth was refused at its minimal need
    /// (`requested_bytes`, after the doubling step was also refused); the job ended.
    WorkerGrowthRefused { granted_bytes: u64, requested_bytes: u64, cause: Box<ResourceRefusal> },
    /// The worker's measured physical footprint (at a growth exchange or a
    /// poll), or its lifetime maximum (at the result), exceeded the
    /// per-invocation budget.
    WorkerFootprintExceeded { grant_bytes: u64, footprint_bytes: u64 },
    /// Running solve workers are at the process-count bound; the job waits.
    WorkerLimitReached { running: usize, limit: usize },
}

/// How an admission treats the host's pressure state (see the module docs).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AdmissionClass {
    /// Heavy adaptive work: refused unless pressure is normal.
    Adaptive,
    /// An interactive command's ingress lease: the pressure state does not
    /// gate it; capacity, pool, headroom freshness and amount, and the
    /// per-invocation budget do.
    Interactive,
}

/// What a lease's bytes cover.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LeasePurpose {
    /// A solve's buffered raw body while queued (its materialization happens
    /// in the worker), or an interactive command's raw body, its
    /// materialization and decoded arguments while the command runs.
    Ingress = 0,
    /// A published native result not yet taken.
    Result = 1,
    /// A delivered result: the handoff copy overlap and the webview's copy until disposed.
    Custody = 2,
    /// A running solve worker's grant (its whole process, capped).
    Worker = 3,
}

#[derive(Debug, Default)]
struct PoolState {
    capacity_bytes: Option<u64>,
    pool_bytes: u64,
    invocation_budget_bytes: u64,
    above_recommended_acknowledged: bool,
    defaults_applied: bool,
    reserved: [u64; 4],
    /// Live worker leases by id (one per worker process not yet reaped).
    workers: BTreeMap<u64, WorkerEntry>,
    next_worker_id: u64,
    /// The bound on live worker leases ([`max_running_workers`]).
    max_running_workers: usize,
    /// Bumped on every release or wake so a waiter does not miss a change.
    generation: u64,
    last_refusal: Option<ResourceRefusal>,
}

impl PoolState {
    fn reserved_total(&self) -> u64 {
        self.reserved.iter().fold(0u64, |sum, v| sum.saturating_add(*v))
    }

    /// Fresh residency credit: the sum over live worker leases of
    /// min(measured realized bytes, lease bytes); unmeasurable leases give 0.
    fn resident_credit(&self) -> u64 {
        self.workers
            .values()
            .map(|entry| entry.probe.as_ref().and_then(|probe| probe.realized_bytes()).unwrap_or(0).min(entry.bytes))
            .fold(0u64, u64::saturating_add)
    }

    /// Records a known capacity; the first one applies the default policy.
    fn observe(&mut self, sample: &HostSample) {
        let Some(capacity) = sample.capacity_bytes else { return };
        self.capacity_bytes = Some(capacity);
        if !self.defaults_applied {
            self.defaults_applied = true;
            self.pool_bytes = recommended_pool_bytes(capacity).unwrap_or(0);
            self.invocation_budget_bytes = DEFAULT_INVOCATION_BUDGET_BYTES.min(self.pool_bytes);
        }
    }
}

fn check_fresh(sample: &HostSample) -> Result<(), ResourceRefusal> {
    let age = sample.age();
    if age > HEADROOM_SAMPLE_MAX_AGE {
        return Err(ResourceRefusal::HeadroomStale { age_ms: u64::try_from(age.as_millis()).unwrap_or(u64::MAX) });
    }
    Ok(())
}

fn check_pressure(sample: &HostSample) -> Result<(), ResourceRefusal> {
    match sample.pressure {
        PressureLevel::Normal => Ok(()),
        PressureLevel::Unknown => Err(ResourceRefusal::PressureUnknown),
        level @ (PressureLevel::Warning | PressureLevel::Critical) => Err(ResourceRefusal::PressureElevated { level }),
    }
}

/// Job counters the registry feeds into a status report.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct JobCounts {
    pub queued: usize,
    pub running: usize,
    pub retained: usize,
    pub custody: usize,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct ReservedByPurpose {
    pub ingress: u64,
    pub result: u64,
    pub custody: u64,
    pub worker: u64,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct HeadroomStatus {
    pub available_bytes: Option<u64>,
    pub pressure: PressureLevel,
    pub sample_age_ms: Option<u64>,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct PoolLimits {
    pub max_queued_jobs: usize,
    pub max_retained_results: usize,
    pub max_webview_custody_results: usize,
    pub max_terminal_records: usize,
    pub max_running_workers: usize,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct PoolStatus {
    pub capacity_bytes: Option<u64>,
    pub recommended_pool_bytes: Option<u64>,
    pub pool_bytes: u64,
    pub pool_above_recommended: bool,
    pub invocation_budget_bytes: u64,
    /// The budget a fresh pool starts with: 12 GiB (the pool, if smaller).
    pub default_invocation_budget_bytes: u64,
    /// The largest budget `configure_memory_pool` accepts: the pool.
    pub max_invocation_budget_bytes: u64,
    pub reserved_bytes: u64,
    pub reserved_by_purpose: ReservedByPurpose,
    pub running_workers: usize,
    pub queued_jobs: usize,
    pub running_jobs: usize,
    pub retained_results: usize,
    pub webview_custody_results: usize,
    pub headroom: HeadroomStatus,
    pub ordinary_route_pricing: &'static str,
    pub limits: PoolLimits,
    pub last_refusal: Option<ResourceRefusal>,
}

/// Why a wait for an admission ended without one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum WaitOutcome {
    Cancelled,
}

/// Why a re-sampling admission ([`MemoryPool::acquire_resampling`]) ended
/// without a lease.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ResampleStop {
    /// The refusal that stands.
    Refused(ResourceRefusal),
    /// The caller was cancelled between attempts.
    Cancelled,
}

pub(crate) struct MemoryPool {
    host: Arc<dyn HostMemory>,
    state: Mutex<PoolState>,
    changed: Condvar,
    wait_tick: Duration,
    resample_window: Duration,
    resample_tick: Duration,
}

impl std::fmt::Debug for MemoryPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemoryPool").field("state", &*self.lock()).finish()
    }
}

impl MemoryPool {
    /// Defaults come from one host sample: a known capacity C gives
    /// pool = floor(3C/4) and budget = min(12 GiB, pool); an unknown capacity
    /// leaves the pool at 0 (every priced admission refuses `capacity_unknown`)
    /// until a later sample establishes it.
    pub(crate) fn new(host: Arc<dyn HostMemory>) -> Arc<Self> {
        Self::with_timing(host, ADMISSION_WAIT_TICK, HEADROOM_RESAMPLE_WINDOW, HEADROOM_RESAMPLE_TICK)
    }

    /// A pool with a queued-job wait tick of `wait_tick` (tests use a short
    /// one); its headroom re-sampling window is ten such ticks.
    #[cfg(test)]
    pub(crate) fn with_wait_tick(host: Arc<dyn HostMemory>, wait_tick: Duration) -> Arc<Self> {
        Self::with_timing(host, wait_tick, wait_tick.saturating_mul(10), wait_tick)
    }

    pub(crate) fn with_timing(host: Arc<dyn HostMemory>, wait_tick: Duration, resample_window: Duration, resample_tick: Duration) -> Arc<Self> {
        let mut state = PoolState { max_running_workers: max_running_workers(), ..PoolState::default() };
        state.observe(&host.sample());
        Arc::new(Self { host, state: Mutex::new(state), changed: Condvar::new(), wait_tick, resample_window, resample_tick })
    }

    fn lock(&self) -> MutexGuard<'_, PoolState> {
        // No code panics while holding this lock; recovering keeps release
        // working during unwinding elsewhere.
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn notify(&self, state: &mut PoolState) {
        state.generation = state.generation.wrapping_add(1);
        self.changed.notify_all();
    }

    /// The current pool and per-invocation budget.
    pub(crate) fn settings(&self) -> (u64, u64) {
        let state = self.lock();
        (state.pool_bytes, state.invocation_budget_bytes)
    }

    pub(crate) fn record_refusal(&self, refusal: &ResourceRefusal) {
        self.lock().last_refusal = Some(refusal.clone());
    }

    /// The atomic verdict for `bytes` more reserved bytes, under the lock.
    fn admit_bytes_locked(&self, state: &mut PoolState, bytes: u64, class: AdmissionClass) -> Result<(), ResourceRefusal> {
        let sample = self.host.sample();
        state.observe(&sample);
        if sample.capacity_bytes.is_none() {
            return Err(ResourceRefusal::CapacityUnknown);
        }
        let reserved = state.reserved_total();
        if reserved.checked_add(bytes).is_none_or(|total| total > state.pool_bytes) {
            return Err(ResourceRefusal::PoolExhausted {
                requested_bytes: bytes,
                reserved_bytes: reserved,
                pool_bytes: state.pool_bytes,
            });
        }
        check_fresh(&sample)?;
        if class == AdmissionClass::Adaptive {
            check_pressure(&sample)?;
        }
        let available = sample.headroom_bytes.ok_or(ResourceRefusal::HeadroomUnknown)?;
        // Deliberately conservative double counting: bytes already reserved may
        // not yet be resident, so the host's headroom must cover them as well
        // as the new request, less what running workers are measured (now) to
        // hold resident already.
        let mut credit = state.resident_credit();
        if class == AdmissionClass::Interactive {
            // An interactive command (an edit or a save) also counts what is
            // already written: a retained result's bytes are resident in this
            // process, and a delivered result's webview copy (half its 2R
            // custody lease) is resident in the webview. Adaptive admissions
            // keep the stricter double counting.
            credit = credit
                .saturating_add(state.reserved[LeasePurpose::Result as usize])
                .saturating_add(state.reserved[LeasePurpose::Custody as usize] / 2);
        }
        let required = bytes.saturating_add(reserved.saturating_sub(credit));
        if available < required {
            return Err(ResourceRefusal::HeadroomInsufficient { required_bytes: required, available_bytes: available });
        }
        Ok(())
    }

    /// Admits and records `bytes` for `purpose` in one critical section,
    /// without re-sampling (product callers use [`Self::acquire_resampling`]).
    #[cfg(test)]
    pub(crate) fn acquire(self: &Arc<Self>, bytes: u64, purpose: LeasePurpose) -> Result<Lease, ResourceRefusal> {
        let mut state = self.lock();
        self.acquire_locked(&mut state, bytes, purpose)
    }

    /// [`Self::acquire`], re-sampling the host while the only refusal is
    /// `headroom_insufficient` (see the module docs). `cancelled` is checked
    /// before every further attempt, outside the pool lock. Not for worker
    /// leases, which wait in the queue instead.
    pub(crate) fn acquire_resampling(
        self: &Arc<Self>,
        bytes: u64,
        purpose: LeasePurpose,
        cancelled: &dyn Fn() -> bool,
    ) -> Result<Lease, ResampleStop> {
        self.acquire_resampling_checked(bytes, purpose, cancelled, &|_| None)
    }

    /// [`Self::acquire_resampling`] with `check`, a standing refusal tested
    /// under the lock before each attempt's verdict.
    fn acquire_resampling_checked(
        self: &Arc<Self>,
        bytes: u64,
        purpose: LeasePurpose,
        cancelled: &dyn Fn() -> bool,
        check: &dyn Fn(&PoolState) -> Option<ResourceRefusal>,
    ) -> Result<Lease, ResampleStop> {
        debug_assert_ne!(purpose, LeasePurpose::Worker, "worker grants wait in the queue instead");
        let deadline = Instant::now().checked_add(self.resample_window);
        let mut state = self.lock();
        loop {
            if let Some(refusal) = check(&state) {
                state.last_refusal = Some(refusal.clone());
                return Err(ResampleStop::Refused(refusal));
            }
            let refusal = match self.verdict_locked(&mut state, bytes, purpose) {
                Ok(()) => {
                    let budget = state.invocation_budget_bytes;
                    return Ok(self.record_locked(&mut state, bytes, purpose, budget));
                }
                Err(refusal) => refusal,
            };
            let remaining = deadline.map_or(Duration::ZERO, |deadline| deadline.saturating_duration_since(Instant::now()));
            if !matches!(refusal, ResourceRefusal::HeadroomInsufficient { .. }) || remaining.is_zero() {
                state.last_refusal = Some(refusal.clone());
                return Err(ResampleStop::Refused(refusal));
            }
            // The lock is released while waiting; a release or a cancellation
            // request (`wake_waiters`) wakes it early.
            drop(self.changed.wait_timeout(state, self.resample_tick.min(remaining)).unwrap_or_else(PoisonError::into_inner));
            if cancelled() {
                return Err(ResampleStop::Cancelled);
            }
            // The deadline bounds when an attempt may start, checked once the
            // lock is held again: past it, the refusal already observed stands.
            state = self.lock();
            if deadline.is_none_or(|deadline| Instant::now() >= deadline) {
                state.last_refusal = Some(refusal.clone());
                return Err(ResampleStop::Refused(refusal));
            }
        }
    }

    /// [`Self::acquire_resampling`] for a native reader's parse of a held
    /// result (C1 native custody, DEL-04-05 `c1-native-custody-plan.md`):
    /// before every attempt, under the pool lock, `bytes` above the
    /// per-invocation budget, and never above the owner-adopted
    /// [`DEFAULT_INVOCATION_BUDGET_BYTES`] whatever budget is configured, is
    /// refused `ingress_exceeds_budget` (it stands; it is not re-sampled).
    /// Otherwise the unchanged adaptive verdict applies.
    pub(crate) fn acquire_result_parse(self: &Arc<Self>, bytes: u64, cancelled: &dyn Fn() -> bool) -> Result<Lease, ResampleStop> {
        self.acquire_resampling_checked(bytes, LeasePurpose::Ingress, cancelled, &|state| {
            let cap = state.invocation_budget_bytes.min(DEFAULT_INVOCATION_BUDGET_BYTES);
            // Before a capacity is known no budget applies; the verdict refuses it.
            (state.capacity_bytes.is_some() && bytes > cap)
                .then_some(ResourceRefusal::IngressExceedsBudget { required_bytes: bytes, budget_bytes: cap })
        })
    }

    fn acquire_locked(self: &Arc<Self>, state: &mut PoolState, bytes: u64, purpose: LeasePurpose) -> Result<Lease, ResourceRefusal> {
        let budget = state.invocation_budget_bytes;
        self.acquire_locked_under(state, bytes, purpose, budget)
    }

    fn acquire_locked_under(
        self: &Arc<Self>,
        state: &mut PoolState,
        bytes: u64,
        purpose: LeasePurpose,
        admitted_budget_bytes: u64,
    ) -> Result<Lease, ResourceRefusal> {
        if let Err(refusal) = self.verdict_locked(state, bytes, purpose) {
            state.last_refusal = Some(refusal.clone());
            return Err(refusal);
        }
        Ok(self.record_locked(state, bytes, purpose, admitted_budget_bytes))
    }

    /// The adaptive verdict for `bytes` more of `purpose` (worker leases are
    /// also bounded by the process count).
    fn verdict_locked(&self, state: &mut PoolState, bytes: u64, purpose: LeasePurpose) -> Result<(), ResourceRefusal> {
        let limit = state.max_running_workers;
        if purpose == LeasePurpose::Worker && state.workers.len() >= limit {
            return Err(ResourceRefusal::WorkerLimitReached { running: state.workers.len(), limit });
        }
        self.admit_bytes_locked(state, bytes, AdmissionClass::Adaptive)
    }

    /// Records an admitted adaptive lease.
    /// `admitted_budget_bytes` is the budget the lease's invocation was
    /// admitted under: the current budget, or a queued job's own snapshot.
    fn record_locked(self: &Arc<Self>, state: &mut PoolState, bytes: u64, purpose: LeasePurpose, admitted_budget_bytes: u64) -> Lease {
        state.reserved[purpose as usize] += bytes;
        let worker_id = (purpose == LeasePurpose::Worker).then(|| {
            let id = state.next_worker_id;
            state.next_worker_id += 1;
            state.workers.insert(id, WorkerEntry { bytes, probe: None });
            id
        });
        Lease { pool: Arc::clone(self), bytes, purpose, class: AdmissionClass::Adaptive, worker_id, admitted_budget_bytes }
    }

    /// Admits an interactive command's ingress lease of `bytes` in one
    /// critical section ([`AdmissionClass::Interactive`]): a known capacity,
    /// `bytes` within the per-invocation budget (`ingress_exceeds_budget`),
    /// the pool, and a fresh known headroom covering `bytes` plus the
    /// outstanding unrealized reservations. The pressure state is not
    /// consulted. It is not re-sampled here: most interactive commands run on
    /// the main thread (see the module docs).
    pub(crate) fn acquire_interactive(self: &Arc<Self>, bytes: u64) -> Result<Lease, ResourceRefusal> {
        self.acquire_interactive_under(bytes, |budget| budget)
    }

    /// [`Self::acquire_interactive`] for a native reader's arguments (C1
    /// native custody, DEL-04-05 `c1-native-custody-plan.md` §8): the lease
    /// and every growth of it ([`Lease::grow_to`], under the pool lock) are
    /// bounded by the per-invocation budget and never above the owner-adopted
    /// [`DEFAULT_INVOCATION_BUDGET_BYTES`], whatever budget is configured.
    pub(crate) fn acquire_interactive_capped(self: &Arc<Self>, bytes: u64) -> Result<Lease, ResourceRefusal> {
        self.acquire_interactive_under(bytes, |budget| budget.min(DEFAULT_INVOCATION_BUDGET_BYTES))
    }

    /// An interactive lease bounded by `cap(budget)`, which the lease keeps
    /// as the budget it was admitted under.
    fn acquire_interactive_under(self: &Arc<Self>, bytes: u64, cap: impl Fn(u64) -> u64) -> Result<Lease, ResourceRefusal> {
        let mut state = self.lock();
        let verdict = (|| {
            let sample = self.host.sample();
            state.observe(&sample);
            if sample.capacity_bytes.is_none() {
                return Err(ResourceRefusal::CapacityUnknown);
            }
            let budget_bytes = cap(state.invocation_budget_bytes);
            if bytes > budget_bytes {
                return Err(ResourceRefusal::IngressExceedsBudget { required_bytes: bytes, budget_bytes });
            }
            self.admit_bytes_locked(&mut state, bytes, AdmissionClass::Interactive)
        })();
        if let Err(refusal) = verdict {
            state.last_refusal = Some(refusal.clone());
            return Err(refusal);
        }
        state.reserved[LeasePurpose::Ingress as usize] += bytes;
        Ok(Lease {
            pool: Arc::clone(self),
            bytes,
            purpose: LeasePurpose::Ingress,
            class: AdmissionClass::Interactive,
            worker_id: None,
            admitted_budget_bytes: cap(state.invocation_budget_bytes),
        })
    }

    /// Releases `bytes` of `purpose`; a worker lease's entry is updated to
    /// `remaining` bytes, or removed when the lease ends.
    fn release(&self, purpose: LeasePurpose, bytes: u64, worker: Option<(u64, Option<u64>)>) {
        let held = {
            let mut state = self.lock();
            let slot = &mut state.reserved[purpose as usize];
            let held = *slot;
            *slot = slot.saturating_sub(bytes);
            match worker {
                Some((id, Some(remaining))) => {
                    if let Some(entry) = state.workers.get_mut(&id) {
                        entry.bytes = remaining;
                    }
                }
                Some((id, None)) => {
                    state.workers.remove(&id);
                }
                None => {}
            }
            self.notify(&mut state);
            held
        };
        // Checked after the lock is released so a failure cannot poison it.
        debug_assert!(held >= bytes, "a lease releases only what it reserved");
    }

    /// Waits until `bytes` for `purpose` are admitted or `cancelled()` holds.
    /// Each refusal while waiting is reported through `on_queued`. Neither
    /// callback runs under the pool lock (callers may take their own locks).
    pub(crate) fn wait_acquire(
        self: &Arc<Self>,
        bytes: u64,
        purpose: LeasePurpose,
        admitted_budget_bytes: Option<u64>,
        cancelled: &dyn Fn() -> bool,
        on_queued: &mut dyn FnMut(&ResourceRefusal),
    ) -> Result<Lease, WaitOutcome> {
        loop {
            if cancelled() {
                return Err(WaitOutcome::Cancelled);
            }
            let (verdict, generation) = {
                let mut state = self.lock();
                let budget = admitted_budget_bytes.unwrap_or(state.invocation_budget_bytes);
                (self.acquire_locked_under(&mut state, bytes, purpose, budget), state.generation)
            };
            match verdict {
                Ok(lease) => return Ok(lease),
                Err(reason) => on_queued(&reason),
            }
            let state = self.lock();
            if state.generation == generation {
                drop(self.changed.wait_timeout(state, self.wait_tick).unwrap_or_else(PoisonError::into_inner));
            }
        }
    }

    /// Wakes waiters to re-check (e.g. after a cancellation request).
    pub(crate) fn wake_waiters(&self) {
        let mut state = self.lock();
        self.notify(&mut state);
    }

    /// Applies the owner's policy to a requested pool and budget.
    pub(crate) fn configure(
        &self,
        pool_bytes: u64,
        invocation_budget_bytes: u64,
        acknowledge_above_recommended: bool,
    ) -> Result<(), ResourceRefusal> {
        let mut state = self.lock();
        let sample = self.host.sample();
        state.observe(&sample);
        let verdict = (|| {
            let capacity = sample.capacity_bytes.ok_or(ResourceRefusal::CapacityUnknown)?;
            if pool_bytes > capacity {
                return Err(ResourceRefusal::PoolAboveCapacity { pool_bytes, capacity_bytes: capacity });
            }
            let recommended = recommended_pool_bytes(capacity).ok_or(ResourceRefusal::CapacityUnknown)?;
            if pool_bytes > recommended && !acknowledge_above_recommended {
                return Err(ResourceRefusal::PoolAboveRecommendedUnacknowledged {
                    pool_bytes,
                    recommended_bytes: recommended,
                });
            }
            if invocation_budget_bytes > pool_bytes {
                return Err(ResourceRefusal::BudgetExceedsPool { budget_bytes: invocation_budget_bytes, pool_bytes });
            }
            Ok(pool_bytes > recommended)
        })();
        match verdict {
            Ok(above_recommended) => {
                // Reducing the pool below current reservations is allowed:
                // existing leases stay until dropped; new admissions refuse.
                state.pool_bytes = pool_bytes;
                state.invocation_budget_bytes = invocation_budget_bytes;
                state.above_recommended_acknowledged = above_recommended;
                state.defaults_applied = true;
                self.notify(&mut state);
                Ok(())
            }
            Err(refusal) => {
                state.last_refusal = Some(refusal.clone());
                Err(refusal)
            }
        }
    }

    /// The ledger, a fresh host sample and the registry's job counters.
    pub(crate) fn status(&self, jobs: JobCounts) -> PoolStatus {
        let mut state = self.lock();
        let sample = self.host.sample();
        state.observe(&sample);
        let recommended = state.capacity_bytes.and_then(recommended_pool_bytes);
        PoolStatus {
            capacity_bytes: state.capacity_bytes,
            recommended_pool_bytes: recommended,
            pool_bytes: state.pool_bytes,
            pool_above_recommended: recommended.is_some_and(|r| state.pool_bytes > r),
            invocation_budget_bytes: state.invocation_budget_bytes,
            default_invocation_budget_bytes: DEFAULT_INVOCATION_BUDGET_BYTES,
            max_invocation_budget_bytes: state.pool_bytes,
            reserved_bytes: state.reserved_total(),
            reserved_by_purpose: ReservedByPurpose {
                ingress: state.reserved[LeasePurpose::Ingress as usize],
                result: state.reserved[LeasePurpose::Result as usize],
                custody: state.reserved[LeasePurpose::Custody as usize],
                worker: state.reserved[LeasePurpose::Worker as usize],
            },
            running_workers: state.workers.len(),
            queued_jobs: jobs.queued,
            running_jobs: jobs.running,
            retained_results: jobs.retained,
            webview_custody_results: jobs.custody,
            headroom: HeadroomStatus {
                available_bytes: sample.headroom_bytes,
                pressure: sample.pressure,
                sample_age_ms: Some(u64::try_from(sample.age().as_millis()).unwrap_or(u64::MAX)),
            },
            ordinary_route_pricing: "worker_capped",
            limits: PoolLimits {
                max_queued_jobs: MAX_QUEUED_JOBS,
                max_retained_results: MAX_RETAINED_RESULTS,
                max_webview_custody_results: MAX_WEBVIEW_CUSTODY_RESULTS,
                max_terminal_records: MAX_TERMINAL_RECORDS,
                max_running_workers: state.max_running_workers,
            },
            last_refusal: state.last_refusal.clone(),
        }
    }

    #[cfg(test)]
    pub(crate) fn reserved(&self, purpose: LeasePurpose) -> u64 {
        self.lock().reserved[purpose as usize]
    }

    #[cfg(test)]
    pub(crate) fn reserved_total(&self) -> u64 {
        self.lock().reserved_total()
    }

    #[cfg(test)]
    pub(crate) fn set_max_running_workers(&self, limit: usize) {
        let mut state = self.lock();
        state.max_running_workers = limit;
        self.notify(&mut state);
    }

    #[cfg(test)]
    pub(crate) fn running_workers(&self) -> usize {
        self.lock().workers.len()
    }
}

/// Reserved pool bytes; released exactly once on drop (also during unwinding).
/// Not `Clone`: ownership moves input -> job -> worker -> result -> custody.
#[derive(Debug)]
pub(crate) struct Lease {
    pool: Arc<MemoryPool>,
    bytes: u64,
    purpose: LeasePurpose,
    /// The class its admission (and any growth) is judged under.
    class: AdmissionClass,
    /// A worker lease's entry in the pool's worker ledger.
    worker_id: Option<u64>,
    /// The per-invocation budget in force when this lease was admitted. Its
    /// growth is bounded by it ([`Self::budget_verdict_locked`]): a budget
    /// changed later applies to new invocations, not to this one.
    admitted_budget_bytes: u64,
}

impl Lease {
    pub(crate) fn bytes(&self) -> u64 {
        self.bytes
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn purpose(&self) -> LeasePurpose {
        self.purpose
    }

    /// Admits only the delta, with the same atomic verdict as `acquire`, after
    /// the per-invocation budget's verdict on the enlarged lease
    /// ([`Self::budget_verdict_locked`]) under the same lock.
    pub(crate) fn grow_to(&mut self, new_bytes: u64) -> Result<(), ResourceRefusal> {
        if new_bytes <= self.bytes {
            self.shrink_to(new_bytes);
            return Ok(());
        }
        let delta = new_bytes - self.bytes;
        let pool = Arc::clone(&self.pool);
        let mut state = pool.lock();
        let verdict = self.budget_verdict_locked(new_bytes).and_then(|()| pool.admit_bytes_locked(&mut state, delta, self.class));
        if let Err(refusal) = verdict {
            state.last_refusal = Some(refusal.clone());
            return Err(refusal);
        }
        self.record_growth_locked(&mut state, new_bytes);
        Ok(())
    }

    /// [`Self::grow_to`] to the first of `targets` (in preference order, each
    /// above the current bytes) that the unchanged atomic verdict admits,
    /// re-sampling while the last target's refusal is `headroom_insufficient`
    /// (see the module docs): every attempt tries every target under one pool
    /// lock. A target above the per-invocation budget is refused like any
    /// other, so a last target above it stands at once and is never
    /// re-sampled. `stop` is called between attempts, outside the pool lock;
    /// `true` ends the wait with [`ResampleStop::Cancelled`] and grows nothing.
    /// Returns the admitted target. Only a refusal that stands is recorded.
    pub(crate) fn grow_resampling(&mut self, targets: &[u64], stop: &mut dyn FnMut() -> bool) -> Result<u64, ResampleStop> {
        debug_assert!(!targets.is_empty() && targets.iter().all(|&target| target > self.bytes));
        self.grow_resampling_with(&mut || targets.to_vec(), &mut || {}, stop)
    }

    /// [`Self::grow_resampling`], calling `waiting` once, outside the pool
    /// lock, when the first attempt has been refused only with
    /// `headroom_insufficient` and the step is about to wait; never when the
    /// first attempt is admitted or its refusal stands. A worker's poll step
    /// stops the child there (`solve_worker.rs`). `targets` is called at every
    /// attempt, under the pool lock, so a step can follow a need measured
    /// between attempts; its last target is the need, and a need already
    /// within the lease ends the step with no growth.
    pub(crate) fn grow_resampling_with(
        &mut self,
        targets: &mut dyn FnMut() -> Vec<u64>,
        waiting: &mut dyn FnMut(),
        stop: &mut dyn FnMut() -> bool,
    ) -> Result<u64, ResampleStop> {
        let pool = Arc::clone(&self.pool);
        let deadline = Instant::now().checked_add(pool.resample_window);
        let mut state = pool.lock();
        let mut first_wait = true;
        loop {
            let targets = targets();
            if targets.last().is_none_or(|&need| need <= self.bytes) {
                return Ok(self.bytes);
            }
            let mut refusal = None;
            for &target in targets.iter().filter(|&&target| target > self.bytes) {
                let verdict = self
                    .budget_verdict_locked(target)
                    .and_then(|()| pool.admit_bytes_locked(&mut state, target - self.bytes, self.class));
                match verdict {
                    Ok(()) => {
                        self.record_growth_locked(&mut state, target);
                        return Ok(target);
                    }
                    Err(refused) => refusal = Some(refused),
                }
            }
            let refusal = refusal.expect("at least one target");
            let remaining = deadline.map_or(Duration::ZERO, |deadline| deadline.saturating_duration_since(Instant::now()));
            if !matches!(refusal, ResourceRefusal::HeadroomInsufficient { .. }) || remaining.is_zero() {
                state.last_refusal = Some(refusal.clone());
                return Err(ResampleStop::Refused(refusal));
            }
            if std::mem::take(&mut first_wait) {
                drop(state);
                waiting();
                state = pool.lock();
            }
            drop(pool.changed.wait_timeout(state, pool.resample_tick.min(remaining)).unwrap_or_else(PoisonError::into_inner));
            if stop() {
                return Err(ResampleStop::Cancelled);
            }
            state = pool.lock();
            if deadline.is_none_or(|deadline| Instant::now() >= deadline) {
                state.last_refusal = Some(refusal.clone());
                return Err(ResampleStop::Refused(refusal));
            }
        }
    }

    /// The per-invocation budget's verdict on this lease grown to `new_bytes`:
    /// the budget in force when the lease was admitted, so a budget changed
    /// while an invocation runs neither stops nor extends it. An ingress lease
    /// above it is refused
    /// `ingress_exceeds_budget` and a worker grant `worker_cap_exceeded`, as at
    /// their admission and the worker's own cap. Result and custody leases are
    /// bounded by the pool, not the budget (see the module docs): the 2R
    /// custody of a result up to its grant can exceed it.
    fn budget_verdict_locked(&self, new_bytes: u64) -> Result<(), ResourceRefusal> {
        let budget_bytes = self.admitted_budget_bytes;
        if new_bytes <= budget_bytes {
            return Ok(());
        }
        match self.purpose {
            LeasePurpose::Ingress => Err(ResourceRefusal::IngressExceedsBudget { required_bytes: new_bytes, budget_bytes }),
            LeasePurpose::Worker => Err(ResourceRefusal::WorkerCapExceeded { cap_bytes: budget_bytes, requested_bytes: new_bytes }),
            LeasePurpose::Result | LeasePurpose::Custody => Ok(()),
        }
    }

    fn record_growth_locked(&mut self, state: &mut PoolState, new_bytes: u64) {
        state.reserved[self.purpose as usize] += new_bytes - self.bytes;
        if let Some(entry) = self.worker_id.and_then(|id| state.workers.get_mut(&id)) {
            entry.bytes = new_bytes;
        }
        self.bytes = new_bytes;
    }

    /// Folds `other` (admitted from the same pool) into this lease: its bytes
    /// move to this lease's purpose without being released or admitted again.
    /// Neither may be a worker lease.
    pub(crate) fn absorb(&mut self, mut other: Lease) {
        assert!(Arc::ptr_eq(&self.pool, &other.pool), "a lease absorbs only a lease of its own pool");
        // Only a lease outside the budget absorbs: an ingress lease or a worker
        // grant grows only through the budget's verdict (`grow_to`).
        assert!(matches!(self.purpose, LeasePurpose::Result | LeasePurpose::Custody), "only a result or custody lease absorbs");
        debug_assert!(self.worker_id.is_none() && other.worker_id.is_none());
        let mut state = self.pool.lock();
        state.reserved[other.purpose as usize] -= other.bytes;
        state.reserved[self.purpose as usize] += other.bytes;
        self.bytes += other.bytes;
        other.bytes = 0;
    }

    /// Attaches (or, with `None`, detaches) the residency probe of the worker
    /// process this lease covers; only worker leases have one.
    pub(crate) fn set_residency_probe(&mut self, probe: Option<Arc<dyn ResidencyProbe>>) {
        let Some(id) = self.worker_id else { return };
        if let Some(entry) = self.pool.lock().workers.get_mut(&id) {
            entry.probe = probe;
        }
    }

    /// Releases down to `new_bytes`; never fails.
    pub(crate) fn shrink_to(&mut self, new_bytes: u64) {
        if new_bytes >= self.bytes {
            return;
        }
        let delta = self.bytes - new_bytes;
        self.bytes = new_bytes;
        self.pool.release(self.purpose, delta, self.worker_id.map(|id| (id, Some(new_bytes))));
    }

    /// Moves the reserved bytes to another purpose without releasing them.
    /// Worker leases never change purpose.
    pub(crate) fn set_purpose(&mut self, purpose: LeasePurpose) {
        if purpose == self.purpose {
            return;
        }
        // Forward only: an ingress lease becomes a result or custody, and a
        // result becomes custody. No lease returns to a budget-bound purpose
        // after growing outside the budget.
        assert!(
            matches!(
                (self.purpose, purpose),
                (LeasePurpose::Ingress, LeasePurpose::Result | LeasePurpose::Custody) | (LeasePurpose::Result, LeasePurpose::Custody)
            ),
            "a lease's purpose moves forward only"
        );
        let mut state = self.pool.lock();
        state.reserved[self.purpose as usize] -= self.bytes;
        state.reserved[purpose as usize] += self.bytes;
        self.purpose = purpose;
    }
}

impl Drop for Lease {
    fn drop(&mut self) {
        self.pool.release(self.purpose, self.bytes, self.worker_id.map(|id| (id, None)));
    }
}

#[cfg(test)]
pub(crate) mod test_host {
    //! Scripted host providers: no test allocates toward host limits.
    use super::*;

    #[derive(Debug)]
    pub(crate) struct Script {
        pub capacity_bytes: Option<u64>,
        pub headroom_bytes: Option<u64>,
        pub pressure: PressureLevel,
        pub age: Duration,
    }

    #[derive(Debug)]
    pub(crate) struct ScriptedHost(pub Mutex<Script>);

    pub(crate) const GIB: u64 = 1 << 30;

    impl ScriptedHost {
        /// A 16 GiB machine with ample headroom at normal pressure.
        pub(crate) fn normal() -> Arc<Self> {
            Self::with(Some(16 * GIB), Some(64 * GIB), PressureLevel::Normal)
        }

        pub(crate) fn with(capacity: Option<u64>, headroom: Option<u64>, pressure: PressureLevel) -> Arc<Self> {
            Arc::new(Self(Mutex::new(Script {
                capacity_bytes: capacity,
                headroom_bytes: headroom,
                pressure,
                age: Duration::ZERO,
            })))
        }

        pub(crate) fn set(&self, edit: impl FnOnce(&mut Script)) {
            edit(&mut self.0.lock().unwrap());
        }
    }

    impl HostMemory for ScriptedHost {
        fn sample(&self) -> HostSample {
            let script = self.0.lock().unwrap();
            let now = Instant::now();
            HostSample {
                capacity_bytes: script.capacity_bytes,
                headroom_bytes: script.headroom_bytes,
                pressure: script.pressure,
                taken_at: now.checked_sub(script.age).unwrap_or(now),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test_host::{Script, ScriptedHost, GIB};
    use super::*;
    use std::collections::HashSet;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::cell::Cell;
    use std::sync::{mpsc, Barrier};
    use std::thread::ThreadId;

    fn pool(host: &Arc<ScriptedHost>) -> Arc<MemoryPool> {
        MemoryPool::with_wait_tick(host.clone(), Duration::from_millis(5))
    }

    #[test]
    fn recommended_pool_is_checked_floor_of_three_quarters() {
        for c in [0u64, 1, 2, 3, 4, 5, 7, 16 * GIB, 16 * GIB + 3, 64 * GIB, u64::MAX / 4, u64::MAX / 3, u64::MAX - 1, u64::MAX] {
            let exact = (c as u128 * 3 / 4) as u64;
            assert_eq!(recommended_pool_bytes(c), Some(exact), "{c}");
        }
        assert_eq!(recommended_pool_bytes(16 * GIB), Some(12 * GIB));
        assert_eq!(recommended_pool_bytes(u64::MAX), Some(13_835_058_055_282_163_711));
    }

    #[test]
    fn defaults_follow_capacity_and_default_budget() {
        for (capacity, pool_bytes, budget) in [
            (16 * GIB, 12 * GIB, DEFAULT_INVOCATION_BUDGET_BYTES),
            (8 * GIB, 6 * GIB, 6 * GIB),
            (64 * GIB, 48 * GIB, DEFAULT_INVOCATION_BUDGET_BYTES),
        ] {
            let host = ScriptedHost::with(Some(capacity), Some(capacity), PressureLevel::Normal);
            let status = pool(&host).status(JobCounts::default());
            assert_eq!((status.pool_bytes, status.invocation_budget_bytes), (pool_bytes, budget));
            assert_eq!(status.recommended_pool_bytes, Some(pool_bytes));
            assert!(!status.pool_above_recommended);
        }
        let host = ScriptedHost::with(None, Some(GIB), PressureLevel::Normal);
        let unknown = pool(&host);
        assert_eq!(unknown.settings(), (0, 0));
        assert_eq!(unknown.acquire(1, LeasePurpose::Ingress).unwrap_err(), ResourceRefusal::CapacityUnknown);
        assert_eq!(unknown.acquire(1, LeasePurpose::Worker).unwrap_err(), ResourceRefusal::CapacityUnknown);
        // A later known capacity applies the defaults once.
        host.set(|s| s.capacity_bytes = Some(16 * GIB));
        assert!(unknown.acquire(1, LeasePurpose::Ingress).is_ok());
        assert_eq!(unknown.settings(), (12 * GIB, DEFAULT_INVOCATION_BUDGET_BYTES));
    }

    #[test]
    fn simultaneous_acquisitions_never_over_reserve_and_release_exactly() {
        let host = ScriptedHost::with(Some(16 * GIB), Some(1 << 50), PressureLevel::Normal);
        let pool = pool(&host);
        pool.configure(10 * GIB, 10 * GIB, false).unwrap();
        let threads = 32;
        let barrier = Arc::new(Barrier::new(threads));
        let request = 3 * GIB; // only three fit in 10 GiB
        let handles: Vec<_> = (0..threads)
            .map(|_| {
                let (pool, barrier) = (pool.clone(), barrier.clone());
                std::thread::spawn(move || {
                    barrier.wait();
                    let lease = pool.acquire(request, LeasePurpose::Ingress);
                    assert!(pool.reserved_total() <= 10 * GIB, "never over-reserved");
                    lease
                })
            })
            .collect();
        let leases: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        let admitted = leases.iter().filter(|l| l.is_ok()).count();
        assert_eq!(admitted, 3);
        assert_eq!(pool.reserved_total(), 3 * request);
        for refusal in leases.iter().filter_map(|l| l.as_ref().err()) {
            assert!(matches!(refusal, ResourceRefusal::PoolExhausted { requested_bytes, pool_bytes, .. }
                if *requested_bytes == request && *pool_bytes == 10 * GIB));
        }
        drop(leases);
        assert_eq!(pool.reserved_total(), 0, "every lease released exactly once");

        // Racing growth of existing leases shares the same verdict.
        let leases: Vec<_> = (0..8).map(|_| pool.acquire(GIB, LeasePurpose::Result).unwrap()).collect();
        let barrier = Arc::new(Barrier::new(8));
        let handles: Vec<_> = leases
            .into_iter()
            .map(|mut lease| {
                let barrier = barrier.clone();
                std::thread::spawn(move || {
                    barrier.wait();
                    let grown = lease.grow_to(2 * GIB).is_ok();
                    (lease, grown)
                })
            })
            .collect();
        let grown: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        assert_eq!(grown.iter().filter(|(_, g)| *g).count(), 2, "8 GiB held, 2 GiB of growth fits");
        assert_eq!(pool.reserved_total(), 10 * GIB);
        drop(grown);
        assert_eq!(pool.reserved_total(), 0);
    }

    #[test]
    fn admission_refuses_unknown_stale_and_insufficient_headroom() {
        let host = ScriptedHost::with(Some(16 * GIB), None, PressureLevel::Normal);
        let pool = pool(&host);
        assert_eq!(pool.acquire(GIB, LeasePurpose::Ingress).unwrap_err(), ResourceRefusal::HeadroomUnknown);
        assert_eq!(pool.acquire(GIB, LeasePurpose::Worker).unwrap_err(), ResourceRefusal::HeadroomUnknown);

        host.set(|s| {
            s.headroom_bytes = Some(8 * GIB);
            s.age = Duration::from_secs(5);
        });
        assert!(matches!(pool.acquire(GIB, LeasePurpose::Ingress).unwrap_err(), ResourceRefusal::HeadroomStale { age_ms } if age_ms >= 5000));
        assert!(matches!(pool.acquire(GIB, LeasePurpose::Worker).unwrap_err(), ResourceRefusal::HeadroomStale { .. }));

        host.set(|s| {
            s.age = Duration::ZERO;
            s.headroom_bytes = Some(3 * GIB);
        });
        let first = pool.acquire(2 * GIB, LeasePurpose::Ingress).unwrap();
        // Outstanding reservations count against headroom (deliberate double counting).
        assert_eq!(
            pool.acquire(2 * GIB, LeasePurpose::Ingress).unwrap_err(),
            ResourceRefusal::HeadroomInsufficient { required_bytes: 4 * GIB, available_bytes: 3 * GIB }
        );
        assert!(pool.acquire(GIB, LeasePurpose::Ingress).is_ok());
        drop(first);
        assert!(matches!(pool.status(JobCounts::default()).last_refusal, Some(ResourceRefusal::HeadroomInsufficient { .. })));
    }

    #[test]
    fn admission_refuses_elevated_or_unknown_pressure() {
        let host = ScriptedHost::normal();
        let pool = pool(&host);
        for level in [PressureLevel::Warning, PressureLevel::Critical] {
            host.set(|s| s.pressure = level);
            assert_eq!(pool.acquire(1, LeasePurpose::Ingress).unwrap_err(), ResourceRefusal::PressureElevated { level });
            assert_eq!(pool.acquire(1, LeasePurpose::Worker).unwrap_err(), ResourceRefusal::PressureElevated { level });
        }
        // An unreadable pressure level is its own refusal, not an elevated level.
        host.set(|s| s.pressure = PressureLevel::Unknown);
        assert_eq!(pool.acquire(1, LeasePurpose::Ingress).unwrap_err(), ResourceRefusal::PressureUnknown);
        assert_eq!(pool.acquire(1, LeasePurpose::Worker).unwrap_err(), ResourceRefusal::PressureUnknown);
        host.set(|s| s.pressure = PressureLevel::Normal);
        let lease = pool.acquire(1, LeasePurpose::Ingress).unwrap();
        // Elevated pressure refuses new admission without touching existing leases.
        host.set(|s| s.pressure = PressureLevel::Critical);
        assert_eq!(pool.reserved_total(), 1);
        drop(lease);
        assert_eq!(pool.reserved_total(), 0);
    }

    #[test]
    fn lease_grow_shrink_purpose_and_unwinding_release_are_exact() {
        let host = ScriptedHost::normal();
        let pool = pool(&host);
        let mut lease = pool.acquire(100, LeasePurpose::Result).unwrap();
        lease.grow_to(200).unwrap();
        assert_eq!((lease.bytes(), pool.reserved(LeasePurpose::Result)), (200, 200));
        lease.set_purpose(LeasePurpose::Custody);
        assert_eq!((pool.reserved(LeasePurpose::Result), pool.reserved(LeasePurpose::Custody)), (0, 200));
        lease.shrink_to(50);
        assert_eq!(pool.reserved(LeasePurpose::Custody), 50);
        assert_eq!(lease.purpose(), LeasePurpose::Custody);
        // A refused growth keeps the existing reservation.
        assert!(matches!(lease.grow_to(13 * GIB).unwrap_err(), ResourceRefusal::PoolExhausted { .. }));
        assert_eq!(lease.bytes(), 50);
        drop(lease);
        assert_eq!(pool.reserved_total(), 0);

        let unwinding = pool.clone();
        let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
            let _lease = unwinding.acquire(GIB, LeasePurpose::Ingress).unwrap();
            let mut worker = unwinding.acquire(GIB, LeasePurpose::Worker).unwrap();
            worker.set_residency_probe(Some(ScriptedProbe::new(Some(GIB / 2))));
            worker.grow_to(2 * GIB).unwrap();
            panic!("solve panicked");
        }));
        assert!(panicked.is_err());
        assert_eq!(pool.reserved_total(), 0);
        assert_eq!(pool.running_workers(), 0);
        assert!(pool.lock().workers.is_empty());
    }

    #[test]
    fn configure_enforces_policy_and_acknowledgement_never_bypasses_refusals() {
        let host = ScriptedHost::with(Some(16 * GIB), Some(64 * GIB), PressureLevel::Normal);
        let pool = pool(&host);
        assert_eq!(
            pool.configure(17 * GIB, GIB, true).unwrap_err(),
            ResourceRefusal::PoolAboveCapacity { pool_bytes: 17 * GIB, capacity_bytes: 16 * GIB }
        );
        assert_eq!(
            pool.configure(14 * GIB, GIB, false).unwrap_err(),
            ResourceRefusal::PoolAboveRecommendedUnacknowledged { pool_bytes: 14 * GIB, recommended_bytes: 12 * GIB }
        );
        assert_eq!(
            pool.configure(4 * GIB, 5 * GIB, false).unwrap_err(),
            ResourceRefusal::BudgetExceedsPool { budget_bytes: 5 * GIB, pool_bytes: 4 * GIB }
        );
        assert_eq!(
            pool.configure(14 * GIB, 14 * GIB + 1, true).unwrap_err(),
            ResourceRefusal::BudgetExceedsPool { budget_bytes: 14 * GIB + 1, pool_bytes: 14 * GIB }
        );
        assert_eq!(pool.settings(), (12 * GIB, DEFAULT_INVOCATION_BUDGET_BYTES), "refusals change nothing");
        // A budget above 12 GiB is accepted up to the pool (one current
        // contract: no separate qualification refusal).
        pool.configure(14 * GIB, 14 * GIB, true).unwrap();
        let status = pool.status(JobCounts::default());
        assert_eq!((status.invocation_budget_bytes, status.max_invocation_budget_bytes), (14 * GIB, 14 * GIB));
        assert_eq!(status.default_invocation_budget_bytes, DEFAULT_INVOCATION_BUDGET_BYTES);
        // A lower budget is allowed; an acknowledged pool above 3/4 is allowed.
        pool.configure(16 * GIB, 2 * GIB, true).unwrap();
        let status = pool.status(JobCounts::default());
        assert!(status.pool_above_recommended);
        assert_eq!((status.pool_bytes, status.invocation_budget_bytes), (16 * GIB, 2 * GIB));
        // Acknowledgement does not bypass headroom or pressure.
        host.set(|s| s.headroom_bytes = Some(GIB));
        assert!(matches!(pool.acquire(15 * GIB, LeasePurpose::Ingress).unwrap_err(), ResourceRefusal::HeadroomInsufficient { .. }));
        host.set(|s| {
            s.headroom_bytes = Some(64 * GIB);
            s.pressure = PressureLevel::Warning;
        });
        assert!(matches!(pool.acquire(GIB, LeasePurpose::Ingress).unwrap_err(), ResourceRefusal::PressureElevated { .. }));
        // Reducing the pool below reservations keeps existing leases; new admissions refuse.
        host.set(|s| s.pressure = PressureLevel::Normal);
        let held = pool.acquire(8 * GIB, LeasePurpose::Ingress).unwrap();
        pool.configure(4 * GIB, 4 * GIB, false).unwrap();
        assert_eq!(pool.reserved_total(), 8 * GIB);
        assert!(matches!(pool.acquire(1, LeasePurpose::Ingress).unwrap_err(), ResourceRefusal::PoolExhausted { .. }));
        drop(held);
        assert!(pool.acquire(1, LeasePurpose::Ingress).is_ok());
        // Unknown capacity refuses configuration.
        host.set(|s| s.capacity_bytes = None);
        assert_eq!(pool.configure(GIB, GIB, false).unwrap_err(), ResourceRefusal::CapacityUnknown);
        // u64 extremes do not overflow.
        host.set(|s| s.capacity_bytes = Some(u64::MAX));
        assert!(matches!(pool.configure(u64::MAX, GIB, false).unwrap_err(), ResourceRefusal::PoolAboveRecommendedUnacknowledged { .. }));
        pool.configure(u64::MAX, GIB, true).unwrap();
        assert!(matches!(pool.acquire(u64::MAX, LeasePurpose::Ingress).unwrap_err(), ResourceRefusal::HeadroomInsufficient { .. }));
    }

    #[test]
    fn wait_acquire_queues_until_admitted_or_cancelled() {
        let host = ScriptedHost::normal();
        let pool = pool(&host);
        pool.set_max_running_workers(4);
        pool.configure(4 * GIB, 4 * GIB, false).unwrap();
        let held = pool.acquire(3 * GIB, LeasePurpose::Worker).unwrap();
        assert_eq!(pool.running_workers(), 1);
        // The waiter reports each refusal; the test acts once it has been refused.
        let (refused_tx, refused) = mpsc::channel();
        let waiter = pool.clone();
        let handle = std::thread::spawn(move || {
            let mut reasons = Vec::new();
            let lease = waiter.wait_acquire(2 * GIB, LeasePurpose::Worker, None, &|| false, &mut |r| {
                reasons.push(r.clone());
                let _ = refused_tx.send(());
            });
            (lease.map(|l| l.bytes()), reasons)
        });
        refused.recv().expect("the waiter is refused while the pool is held");
        assert_eq!(pool.reserved_total(), 3 * GIB, "the waiter reserved nothing while refused");
        drop(held);
        let (admitted, reasons) = handle.join().unwrap();
        assert_eq!(admitted, Ok(2 * GIB));
        assert!(!reasons.is_empty() && reasons.iter().all(|r| matches!(r, ResourceRefusal::PoolExhausted { .. })));
        assert_eq!((pool.reserved_total(), pool.running_workers()), (0, 0), "the waiter's lease dropped");

        host.set(|s| s.pressure = PressureLevel::Warning);
        let cancel = Arc::new(AtomicBool::new(false));
        let (refused_tx, refused) = mpsc::channel();
        let (waiter, flag) = (pool.clone(), cancel.clone());
        let handle = std::thread::spawn(move || {
            let mut last = None;
            let outcome = waiter.wait_acquire(GIB, LeasePurpose::Worker, None, &|| flag.load(Ordering::SeqCst), &mut |r| {
                last = Some(r.clone());
                let _ = refused_tx.send(());
            });
            (outcome.map(drop), last)
        });
        refused.recv().expect("the waiter is refused at warning pressure");
        cancel.store(true, Ordering::SeqCst);
        pool.wake_waiters();
        let (outcome, last) = handle.join().unwrap();
        assert_eq!(outcome, Err(WaitOutcome::Cancelled));
        assert_eq!(last, Some(ResourceRefusal::PressureElevated { level: PressureLevel::Warning }));
        assert_eq!(pool.reserved_total(), 0);
    }

    /// An injectable residency reading.
    #[derive(Debug)]
    struct ScriptedProbe(Mutex<Option<u64>>);

    impl ScriptedProbe {
        fn new(bytes: Option<u64>) -> Arc<Self> {
            Arc::new(Self(Mutex::new(bytes)))
        }
        fn set(&self, bytes: Option<u64>) {
            *self.0.lock().unwrap() = bytes;
        }
    }

    impl ResidencyProbe for ScriptedProbe {
        fn realized_bytes(&self) -> Option<u64> {
            *self.0.lock().unwrap()
        }
    }

    #[test]
    fn running_workers_are_bounded_by_the_process_count_limit() {
        let host = ScriptedHost::normal();
        let pool = pool(&host);
        assert_eq!(pool.status(JobCounts::default()).limits.max_running_workers, max_running_workers());
        assert!(max_running_workers() >= 1);
        pool.set_max_running_workers(2);
        let a = pool.acquire(GIB, LeasePurpose::Worker).unwrap();
        let _b = pool.acquire(GIB, LeasePurpose::Worker).unwrap();
        assert_eq!(
            pool.acquire(GIB, LeasePurpose::Worker).unwrap_err(),
            ResourceRefusal::WorkerLimitReached { running: 2, limit: 2 }
        );
        // Other purposes are not process-bounded.
        drop(pool.acquire(GIB, LeasePurpose::Result).unwrap());
        let (refused_tx, refused) = mpsc::channel();
        let waiter = pool.clone();
        let handle = std::thread::spawn(move || {
            let mut reasons = Vec::new();
            let lease = waiter.wait_acquire(GIB, LeasePurpose::Worker, None, &|| false, &mut |r| {
                reasons.push(r.clone());
                let _ = refused_tx.send(());
            });
            (lease.map(|l| l.bytes()), reasons)
        });
        refused.recv().expect("the waiter is refused at the process-count limit");
        drop(a);
        let (admitted, reasons) = handle.join().unwrap();
        assert_eq!(admitted, Ok(GIB));
        assert!(!reasons.is_empty() && reasons.iter().all(|r| *r == ResourceRefusal::WorkerLimitReached { running: 2, limit: 2 }));
        assert_eq!(
            serde_json::to_value(ResourceRefusal::WorkerLimitReached { running: 2, limit: 2 }).unwrap(),
            serde_json::json!({"kind":"worker_limit_reached","running":2,"limit":2})
        );
    }

    /// Review B1 regression: residency credit is measured afresh at every
    /// admission. After worker A's resident memory drops, a second worker's
    /// growth that a stale credit would admit is refused. The credit never
    /// exceeds a lease's bytes, and an unmeasurable or detached worker gets none.
    #[test]
    fn residency_credit_is_measured_fresh_at_every_admission() {
        let host = ScriptedHost::with(Some(64 * GIB), Some(10 * GIB), PressureLevel::Normal);
        let pool = pool(&host);
        pool.set_max_running_workers(4);
        let (probe_a, probe_b) = (ScriptedProbe::new(Some(3 * GIB)), ScriptedProbe::new(Some(0)));
        let mut a = pool.acquire(4 * GIB, LeasePurpose::Worker).unwrap();
        let mut b = pool.acquire(4 * GIB, LeasePurpose::Worker).unwrap();
        assert_eq!(pool.status(JobCounts::default()).running_workers, 2);
        // Before any probe is attached nothing is credited: 3 + 8 > 10.
        assert_eq!(
            b.grow_to(7 * GIB).unwrap_err(),
            ResourceRefusal::HeadroomInsufficient { required_bytes: 11 * GIB, available_bytes: 10 * GIB }
        );
        a.set_residency_probe(Some(probe_a.clone()));
        b.set_residency_probe(Some(probe_b.clone()));
        // A holds 3 GiB resident: 3 + 8 - 3 = 8 GiB fits.
        b.grow_to(7 * GIB).unwrap();
        // A frees its memory: its credit is gone at the next admission (a stale
        // credit would admit 1 + 11 - 3 = 9 GiB).
        probe_a.set(Some(0));
        assert_eq!(
            b.grow_to(8 * GIB).unwrap_err(),
            ResourceRefusal::HeadroomInsufficient { required_bytes: 12 * GIB, available_bytes: 10 * GIB }
        );
        assert_eq!(b.bytes(), 7 * GIB, "a refused growth keeps the grant");
        // A reading above the lease credits only the lease's bytes: 1 + 11 - 4 = 8.
        probe_a.set(Some(9 * GIB));
        b.grow_to(8 * GIB).unwrap();
        // Every admission path gets the same fresh credit: 1 + 12 - 4 = 9 fits.
        let result = pool.acquire(GIB, LeasePurpose::Result).unwrap();
        // A failed reading credits 0: 1 + 13 > 10.
        probe_a.set(None);
        assert!(matches!(pool.acquire(GIB, LeasePurpose::Result).unwrap_err(), ResourceRefusal::HeadroomInsufficient { .. }));
        // So does a detached probe (a reaped worker).
        probe_a.set(Some(4 * GIB));
        a.set_residency_probe(None);
        assert!(matches!(pool.acquire(GIB, LeasePurpose::Result).unwrap_err(), ResourceRefusal::HeadroomInsufficient { .. }));
        drop((a, b, result));
        assert_eq!((pool.reserved_total(), pool.running_workers()), (0, 0));
        assert!(pool.lock().workers.is_empty());
    }

    /// Interactive headroom counts retained results and half of custody as written.
    #[test]
    fn interactive_admission_credits_written_results_and_custody() {
        // 10 GiB headroom: a 6 GiB result and a 4 GiB custody lease are held.
        let gib = GIB;
        let host = ScriptedHost::with(Some(64 * GIB), Some(10 * GIB), PressureLevel::Normal);
        let pool = pool(&host);
        let _result = pool.acquire(6 * gib, LeasePurpose::Result).unwrap();
        let _custody = pool.acquire(4 * gib, LeasePurpose::Custody).unwrap();
        // Adaptive: 1 GiB more needs 1 + 10 GiB unrealized > 10 GiB available.
        assert!(matches!(pool.acquire(gib, LeasePurpose::Ingress), Err(ResourceRefusal::HeadroomInsufficient { .. })));
        // Interactive: the result (6) and half the custody (2) count as written: 1 + 2 <= 10.
        let lease = pool.acquire_interactive(gib).unwrap();
        drop(lease);
        // Still bounded: 9 GiB more needs 9 + 2 > 10.
        assert!(matches!(pool.acquire_interactive(9 * gib), Err(ResourceRefusal::HeadroomInsufficient { .. })));
        // A result still being read is held as an uncredited ingress lease:
        // with 1 GiB of it pending, 8 GiB more needs 8 + 2 + 1 > 10; once it
        // is written (relabelled Result) the same request fits (8 + 2 <= 10).
        let mut pending = pool.acquire(gib, LeasePurpose::Ingress).unwrap_or_else(|_| {
            host.set(|s| s.headroom_bytes = Some(20 * GIB));
            let lease = pool.acquire(gib, LeasePurpose::Ingress).unwrap();
            host.set(|s| s.headroom_bytes = Some(10 * GIB));
            lease
        });
        assert!(matches!(pool.acquire_interactive(8 * gib), Err(ResourceRefusal::HeadroomInsufficient { .. })));
        pending.set_purpose(LeasePurpose::Result);
        drop(pool.acquire_interactive(8 * gib).unwrap());
    }

    /// The interactive class: pressure does not gate it (warning, critical or
    /// unknown), while capacity, budget, pool and headroom (fresh, known and
    /// covering the outstanding unrealized reservations) do. Adaptive
    /// admissions are unchanged.
    #[test]
    fn interactive_admission_ignores_pressure_but_not_budget_pool_or_headroom() {
        let host = ScriptedHost::with(Some(16 * GIB), Some(8 * GIB), PressureLevel::Normal);
        let pool = pool(&host);
        pool.configure(12 * GIB, 2 * GIB, false).unwrap();
        for level in [PressureLevel::Warning, PressureLevel::Critical, PressureLevel::Unknown] {
            host.set(|s| s.pressure = level);
            let lease = pool.acquire_interactive(GIB).unwrap();
            assert_eq!((lease.purpose(), pool.reserved(LeasePurpose::Ingress)), (LeasePurpose::Ingress, GIB));
            assert!(pool.acquire(1, LeasePurpose::Ingress).is_err(), "adaptive admission still refuses at {level:?}");
            assert!(pool.acquire(1, LeasePurpose::Worker).is_err());
            drop(lease);
        }
        host.set(|s| s.pressure = PressureLevel::Warning);
        assert_eq!(
            pool.acquire_interactive(2 * GIB + 1).unwrap_err(),
            ResourceRefusal::IngressExceedsBudget { required_bytes: 2 * GIB + 1, budget_bytes: 2 * GIB }
        );
        // Outstanding reservations count against headroom (8 GiB): 7 + 2 > 8.
        host.set(|s| s.pressure = PressureLevel::Normal);
        let held = pool.acquire(7 * GIB, LeasePurpose::Ingress).unwrap();
        host.set(|s| s.pressure = PressureLevel::Critical);
        assert_eq!(
            pool.acquire_interactive(2 * GIB).unwrap_err(),
            ResourceRefusal::HeadroomInsufficient { required_bytes: 9 * GIB, available_bytes: 8 * GIB }
        );
        let fits = pool.acquire_interactive(GIB).unwrap();
        // The pool: 7 + 1 held of 12; 5 more would exceed it.
        host.set(|s| s.headroom_bytes = Some(64 * GIB));
        pool.configure(12 * GIB, 12 * GIB, false).unwrap();
        assert!(matches!(pool.acquire_interactive(5 * GIB).unwrap_err(), ResourceRefusal::PoolExhausted { .. }));
        drop((held, fits));
        host.set(|s| s.headroom_bytes = None);
        assert_eq!(pool.acquire_interactive(1).unwrap_err(), ResourceRefusal::HeadroomUnknown);
        host.set(|s| {
            s.headroom_bytes = Some(GIB);
            s.age = Duration::from_secs(5);
        });
        assert!(matches!(pool.acquire_interactive(1).unwrap_err(), ResourceRefusal::HeadroomStale { .. }));
        host.set(|s| s.capacity_bytes = None);
        assert_eq!(pool.acquire_interactive(1).unwrap_err(), ResourceRefusal::CapacityUnknown);
        assert!(matches!(pool.status(JobCounts::default()).last_refusal, Some(ResourceRefusal::CapacityUnknown)));
        assert_eq!(pool.reserved_total(), 0, "every refusal reserved nothing");
    }

    /// The demo result-JSON save of the completeness map's native run
    /// (COMPLETENESS_MAP §5), as measured by an instrumented native build: a
    /// 4,918,811-byte args body whose lease is envelope 8,388,608 +
    /// materialization 8,397,520 + 4,096, while the delivered demo result's
    /// 2R custody lease of 1,146,880 bytes is held (half credited). That run
    /// saw 6.44 MiB of free-queue headroom and was refused "needs 16.56 MiB".
    const DEMO_SAVE_LEASE: u64 = 16_790_224;
    const DEMO_CUSTODY_LEASE: u64 = 1_146_880;
    const DEMO_SAVE_REQUIRED: u64 = 17_363_664;
    const MIB: u64 = 1 << 20;
    const NEVER: &dyn Fn() -> bool = &|| false;

    fn timed_pool<H: HostMemory + 'static>(host: &Arc<H>, window: Duration, tick: Duration) -> Arc<MemoryPool> {
        MemoryPool::with_timing(host.clone(), Duration::from_millis(5), window, tick)
    }

    /// One host sample taken while a test observes its [`ObservedHost`].
    #[derive(Debug)]
    struct Sampled {
        thread: ThreadId,
        at: Instant,
        /// How many [`ObservedHost::set`] edits preceded the sample.
        step: usize,
    }

    #[derive(Debug, Default)]
    struct Observer {
        report: Option<mpsc::Sender<Sampled>>,
        first: Option<Instant>,
        step: usize,
        /// (delay after the first reported sample, headroom from then on).
        refill: Option<(Duration, u64)>,
    }

    /// A scripted host that reports every sample once a test observes it, so
    /// the test acts after an admission attempt has sampled the host (and,
    /// since sampling happens under the pool lock, been refused) instead of
    /// after a fixed sleep. A scheduled refill is applied inside `sample`
    /// itself, to samples taken at least its delay after the first reported
    /// one: no helper thread's scheduling decides when an attempt sees it.
    #[derive(Debug)]
    struct ObservedHost {
        script: Arc<ScriptedHost>,
        observer: Mutex<Observer>,
    }

    impl ObservedHost {
        fn with(capacity: Option<u64>, headroom: Option<u64>, pressure: PressureLevel) -> Arc<Self> {
            Arc::new(Self { script: ScriptedHost::with(capacity, headroom, pressure), observer: Mutex::default() })
        }

        /// Starts reporting samples (call it after the pool's construction sample).
        fn observe(&self, refill: Option<(Duration, u64)>) -> mpsc::Receiver<Sampled> {
            let (report, samples) = mpsc::channel();
            *self.observer.lock().unwrap() = Observer { report: Some(report), refill, ..Observer::default() };
            samples
        }

        /// Edits the script between samples, as one more step.
        fn set(&self, edit: impl FnOnce(&mut Script)) {
            let mut observer = self.observer.lock().unwrap();
            observer.step += 1;
            self.script.set(edit);
        }
    }

    impl HostMemory for ObservedHost {
        fn sample(&self) -> HostSample {
            let mut observer = self.observer.lock().unwrap();
            if let Some(report) = observer.report.clone() {
                let at = Instant::now();
                let first = *observer.first.get_or_insert(at);
                if let Some((delay, headroom)) = observer.refill {
                    if at.saturating_duration_since(first) >= delay {
                        self.script.set(|s| s.headroom_bytes = Some(headroom));
                    }
                }
                let _ = report.send(Sampled { thread: std::thread::current().id(), at, step: observer.step });
            }
            self.script.sample()
        }
    }

    /// A caller stalled for a whole re-sampling window before its first
    /// attempt refuses without waiting, so a test about what follows the wait
    /// repeats such a run (each run keeps every assertion).
    const RUNS_TO_REACH_THE_WAIT: usize = 20;

    /// A rendezvous of `parties` threads that can be aborted, so a failing
    /// coordinator never leaves the other parties blocked.
    struct Rendezvous {
        parties: usize,
        /// (arrived, aborted)
        state: Mutex<(usize, bool)>,
        changed: Condvar,
    }

    impl Rendezvous {
        fn new(parties: usize) -> Arc<Self> {
            Arc::new(Self { parties, state: Mutex::new((0, false)), changed: Condvar::new() })
        }

        /// Waits until every party has arrived or the rendezvous is aborted.
        fn arrive(&self) {
            let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
            state.0 += 1;
            self.changed.notify_all();
            while state.0 < self.parties && !state.1 {
                state = self.changed.wait(state).unwrap_or_else(PoisonError::into_inner);
            }
        }

        fn abort(&self) {
            self.state.lock().unwrap_or_else(PoisonError::into_inner).1 = true;
            self.changed.notify_all();
        }
    }

    /// Re-sampling waiter threads. Dropping it, also while the coordinator
    /// unwinds, aborts their rendezvous, cancels their waits and joins them.
    struct Waiters {
        pool: Arc<MemoryPool>,
        cancel: Arc<AtomicBool>,
        rendezvous: Vec<Arc<Rendezvous>>,
        handles: Vec<std::thread::JoinHandle<bool>>,
    }

    /// Aborts rendezvous if its thread unwinds, so a panicking party never
    /// leaves the others blocked.
    struct AbortOnUnwind(Vec<Arc<Rendezvous>>);

    impl Drop for AbortOnUnwind {
        fn drop(&mut self) {
            if std::thread::panicking() {
                for rendezvous in &self.0 {
                    rendezvous.abort();
                }
            }
        }
    }

    impl Drop for Waiters {
        fn drop(&mut self) {
            self.cancel.store(true, Ordering::SeqCst);
            for rendezvous in &self.rendezvous {
                rendezvous.abort();
            }
            self.pool.wake_waiters();
            for handle in self.handles.drain(..) {
                let _ = handle.join();
            }
        }
    }

    fn refused(stop: ResampleStop) -> ResourceRefusal {
        match stop {
            ResampleStop::Refused(refusal) => refusal,
            ResampleStop::Cancelled => panic!("not cancelled"),
        }
    }

    /// The constructed control: the map's refusal, byte for byte. Interactive
    /// admission refuses at once (the desktop re-sends, `nativeIngress.ts`).
    #[test]
    fn demo_save_refusal_reproduces_the_observed_arithmetic_and_is_immediate() {
        let observed = 6_752_829; // 6.44 MiB
        let host = ScriptedHost::with(Some(128 * GIB), Some(GIB), PressureLevel::Normal);
        let pool = timed_pool(&host, Duration::from_secs(10), Duration::from_millis(10));
        let _custody = pool.acquire(DEMO_CUSTODY_LEASE, LeasePurpose::Custody).unwrap();
        host.set(|s| s.headroom_bytes = Some(observed));
        // The lease is the ingress pricing of the measured body (raw length 4,918,811, materialization 8,397,520).
        let bound = crate::ingress::IngressBound { raw_len: 4_918_811, materialization_bytes: 8_397_520 };
        assert_eq!(crate::command_ingress::lease_bytes(bound, crate::command_ingress::ArgsDecoding::Moved), Some(DEMO_SAVE_LEASE));
        assert_eq!(DEMO_SAVE_REQUIRED, DEMO_SAVE_LEASE + DEMO_CUSTODY_LEASE / 2);
        assert_eq!(format!("{:.2}", DEMO_SAVE_REQUIRED as f64 / MIB as f64), "16.56");
        let started = Instant::now();
        let refusal = pool.acquire_interactive(DEMO_SAVE_LEASE).unwrap_err();
        assert!(started.elapsed() < Duration::from_secs(5), "interactive admission does not wait: {:?}", started.elapsed());
        assert_eq!(refusal, ResourceRefusal::HeadroomInsufficient { required_bytes: DEMO_SAVE_REQUIRED, available_bytes: observed });
        assert_eq!(pool.status(JobCounts::default()).last_refusal, Some(refusal));
        assert_eq!(pool.reserved(LeasePurpose::Ingress), 0, "a refusal reserves nothing");
        host.set(|s| s.headroom_bytes = Some(DEMO_SAVE_REQUIRED - 1));
        assert!(matches!(pool.acquire_interactive(DEMO_SAVE_LEASE), Err(ResourceRefusal::HeadroomInsufficient { .. })));
        host.set(|s| s.headroom_bytes = Some(DEMO_SAVE_REQUIRED));
        let lease = pool.acquire_interactive(DEMO_SAVE_LEASE).unwrap();
        assert_eq!(pool.reserved(LeasePurpose::Ingress), DEMO_SAVE_LEASE);
        drop(lease);
        assert_eq!(pool.reserved(LeasePurpose::Ingress), 0);
    }

    /// With the production window, a dip shorter than it admits the lease at
    /// the first fresh sample that passes, and records no refusal.
    #[test]
    fn a_headroom_dip_within_the_window_admits_a_resampling_lease() {
        let host = ObservedHost::with(Some(128 * GIB), Some(0), PressureLevel::Normal);
        let pool = MemoryPool::new(host.clone());
        // The headroom covers the lease from 300 ms after the first attempt's
        // sample, which is refused and taken after `started`.
        let _samples = host.observe(Some((Duration::from_millis(300), DEMO_SAVE_LEASE)));
        let started = Instant::now();
        let lease = pool.acquire_resampling(DEMO_SAVE_LEASE, LeasePurpose::Ingress, NEVER).expect("admitted once a fresh sample covers it");
        let waited = started.elapsed();
        assert!(waited >= Duration::from_millis(300) && waited < HEADROOM_RESAMPLE_WINDOW, "{waited:?}");
        assert_eq!(lease.bytes(), DEMO_SAVE_LEASE);
        assert_eq!(pool.reserved(LeasePurpose::Ingress), DEMO_SAVE_LEASE);
        assert_eq!(pool.status(JobCounts::default()).last_refusal, None, "a passing re-sample leaves no refusal");
    }

    /// No attempt starts after the deadline: the refusal already observed
    /// stands even if the host would now admit.
    #[test]
    fn resampling_starts_no_attempt_after_the_deadline() {
        for _ in 0..RUNS_TO_REACH_THE_WAIT {
            let host = ObservedHost::with(Some(16 * GIB), Some(0), PressureLevel::Normal);
            let pool = timed_pool(&host, Duration::from_millis(100), Duration::from_secs(60));
            let samples = host.observe(None);
            // The caller's wait has ended and it has not yet reacquired the lock
            // (production checks `cancelled` there). From the window after its
            // first, refused sample its deadline has passed, and the host would
            // now admit.
            let waited_once = Cell::new(false);
            let past_the_deadline = || {
                if !waited_once.replace(true) {
                    let first = samples.recv().expect("the caller has attempted").at;
                    std::thread::sleep((first + Duration::from_millis(100)).saturating_duration_since(Instant::now()));
                    host.set(|s| s.headroom_bytes = Some(GIB));
                }
                false
            };
            let started = Instant::now();
            let refusal = refused(pool.acquire_resampling(MIB, LeasePurpose::Ingress, &past_the_deadline).unwrap_err());
            let waited = started.elapsed();
            assert_eq!(refusal, ResourceRefusal::HeadroomInsufficient { required_bytes: MIB, available_bytes: 0 });
            assert!(waited >= Duration::from_millis(100) && waited < Duration::from_secs(10), "bounded by the window, not the tick: {waited:?}");
            assert_eq!(pool.status(JobCounts::default()).last_refusal, Some(refusal));
            assert_eq!(pool.reserved_total(), 0);
            if waited_once.get() {
                return;
            }
        }
        panic!("no run reached the caller's wait");
    }

    /// A waiter that reacquires the pool lock only after the deadline does not
    /// attempt again, even though the host would now admit.
    #[test]
    fn resampling_starts_no_attempt_after_waiting_past_the_deadline_for_the_lock() {
        const WINDOW: Duration = Duration::from_millis(1200);
        // A run counts only if the handshake below completes this soon after
        // `started`, leaving at least WINDOW - COUNTED_WITHIN (800 ms) before
        // the deadline.
        const COUNTED_WITHIN: Duration = Duration::from_millis(400);
        for _ in 0..RUNS_TO_REACH_THE_WAIT {
            let host = ObservedHost::with(Some(16 * GIB), Some(0), PressureLevel::Normal);
            let pool = timed_pool(&host, WINDOW, Duration::from_millis(20));
            let samples = host.observe(None);
            let (waited_tx, waited) = mpsc::channel();
            let (held_tx, held) = mpsc::channel();
            let holder = {
                let (pool, host) = (pool.clone(), host.clone());
                std::thread::spawn(move || {
                    let first = samples.recv().expect("the waiter attempts").at;
                    // The holder takes the lock only once the waiter's wait has
                    // returned and released it (production checks `cancelled`
                    // between the wait and its reacquisition). A waiter that
                    // refused without waiting dropped the sender.
                    if waited.recv().is_err() {
                        return;
                    }
                    let state = pool.lock();
                    let _ = held_tx.send(());
                    // The waiter's deadline is at most the window after its
                    // first sample; the lock is held, and the host refilled, past it.
                    std::thread::sleep((first + WINDOW + Duration::from_millis(100)).saturating_duration_since(Instant::now()));
                    host.set(|s| s.headroom_bytes = Some(GIB));
                    drop(state);
                })
            };
            // When the waiter's `cancelled` check returns, the holder holds the
            // lock; it was taken after the wait returned.
            let holder_locked_by = Cell::new(None);
            let started = Instant::now();
            let refusal = {
                let holder_locked_by = &holder_locked_by;
                let after_the_holder_locks = move || {
                    if holder_locked_by.get().is_none() {
                        let _ = waited_tx.send(());
                        held.recv().expect("the holder takes the lock");
                        holder_locked_by.set(Some(Instant::now()));
                    }
                    false
                };
                refused(pool.acquire_resampling(MIB, LeasePurpose::Ingress, &after_the_holder_locks).unwrap_err())
            };
            holder.join().unwrap();
            assert_eq!(refusal, ResourceRefusal::HeadroomInsufficient { required_bytes: MIB, available_bytes: 0 });
            assert_eq!(pool.reserved_total(), 0);
            // The run counts only if the holder held the lock, and the waiter
            // went on towards reacquiring it, at least 800 ms before the
            // deadline (which is no earlier than the window after `started`):
            // the waiter then reaches its reacquisition before the deadline,
            // unless it stalls for 800 ms within a few instructions, and
            // reacquires the lock only past it. Later handshakes are repeated.
            if holder_locked_by.get().is_some_and(|at| at < started + COUNTED_WITHIN) {
                return;
            }
        }
        panic!("no run reached the waiter's wait with time left before its deadline");
    }

    /// A cancellation request (`wake_waiters`) ends the wait at once, reserves
    /// nothing and is not a refusal.
    #[test]
    fn resampling_ends_on_cancellation() {
        let host = ObservedHost::with(Some(16 * GIB), Some(0), PressureLevel::Normal);
        let pool = timed_pool(&host, Duration::from_secs(30), Duration::from_secs(30));
        let samples = host.observe(None);
        let flag = Arc::new(AtomicBool::new(false));
        let canceller = {
            let (pool, flag) = (pool.clone(), flag.clone());
            std::thread::spawn(move || {
                // After the first (refused) attempt: the wake, which takes the
                // pool lock, reaches the waiter only once it waits.
                samples.recv().expect("the waiter attempts");
                flag.store(true, Ordering::SeqCst);
                pool.wake_waiters();
            })
        };
        let started = Instant::now();
        let stop = pool.acquire_resampling(MIB, LeasePurpose::Ingress, &|| flag.load(Ordering::SeqCst)).unwrap_err();
        canceller.join().unwrap();
        assert_eq!(stop, ResampleStop::Cancelled);
        assert!(started.elapsed() < Duration::from_secs(10), "woken by the request: {:?}", started.elapsed());
        assert_eq!(pool.reserved_total(), 0);
        // The first, pre-cancellation refusal is not recorded as standing.
        assert_eq!(pool.status(JobCounts::default()).last_refusal, None);
    }

    /// Only `headroom_insufficient` is re-sampled; every other refusal stands
    /// at once, and the adaptive pressure rule is kept.
    #[test]
    fn only_insufficient_headroom_is_resampled() {
        let host = ScriptedHost::with(Some(16 * GIB), Some(64 * GIB), PressureLevel::Normal);
        let pool = timed_pool(&host, Duration::from_secs(30), Duration::from_millis(10));
        let timed = |label: &str| {
            let started = Instant::now();
            let refusal = refused(pool.acquire_resampling(2 * MIB, LeasePurpose::Ingress, NEVER).unwrap_err());
            assert!(started.elapsed() < Duration::from_secs(5), "{label}: {refusal:?} waited {:?}", started.elapsed());
            refusal
        };
        let held = pool.acquire(12 * GIB - MIB, LeasePurpose::Result).unwrap();
        assert!(matches!(timed("pool"), ResourceRefusal::PoolExhausted { .. }));
        drop(held);
        host.set(|s| s.headroom_bytes = None);
        assert_eq!(timed("unknown"), ResourceRefusal::HeadroomUnknown);
        host.set(|s| { s.headroom_bytes = Some(GIB); s.age = Duration::from_secs(3) });
        assert!(matches!(timed("stale"), ResourceRefusal::HeadroomStale { .. }));
        host.set(|s| { s.age = Duration::ZERO; s.pressure = PressureLevel::Warning });
        assert_eq!(timed("pressure"), ResourceRefusal::PressureElevated { level: PressureLevel::Warning });
        host.set(|s| s.pressure = PressureLevel::Unknown);
        assert_eq!(timed("pressure unknown"), ResourceRefusal::PressureUnknown);
        host.set(|s| { s.pressure = PressureLevel::Normal; s.capacity_bytes = None });
        assert_eq!(timed("capacity"), ResourceRefusal::CapacityUnknown);
        assert_eq!(pool.reserved_total(), 0);
    }

    /// A pressure warning that appears while an adaptive lease re-samples ends
    /// the wait with that refusal: the pressure rule still applies.
    #[test]
    fn a_pressure_warning_during_the_window_stands_at_once() {
        let host = ObservedHost::with(Some(16 * GIB), Some(0), PressureLevel::Normal);
        let pool = timed_pool(&host, Duration::from_secs(30), Duration::from_millis(10));
        let samples = host.observe(None);
        let warn = {
            let host = host.clone();
            std::thread::spawn(move || {
                // During the window: after the first attempt was refused for headroom.
                samples.recv().expect("the waiter attempts");
                host.set(|s| { s.pressure = PressureLevel::Warning; s.headroom_bytes = Some(64 * GIB) });
            })
        };
        let started = Instant::now();
        let refusal = refused(pool.acquire_resampling(MIB, LeasePurpose::Ingress, NEVER).unwrap_err());
        warn.join().unwrap();
        assert_eq!(refusal, ResourceRefusal::PressureElevated { level: PressureLevel::Warning });
        assert!(started.elapsed() < Duration::from_secs(10), "{:?}", started.elapsed());
        assert_eq!(pool.reserved_total(), 0);
    }

    /// A release elsewhere wakes a re-sampling admission before its tick.
    #[test]
    fn resampling_admits_after_a_release_frees_headroom() {
        let host = ObservedHost::with(Some(16 * GIB), Some(4 * GIB), PressureLevel::Normal);
        let pool = timed_pool(&host, Duration::from_secs(10), Duration::from_secs(5));
        let held = pool.acquire(3 * GIB, LeasePurpose::Ingress).unwrap();
        let samples = host.observe(None);
        let releaser = std::thread::spawn(move || {
            // After the first attempt was refused: the release, which takes
            // the pool lock, wakes the waiter.
            samples.recv().expect("the waiter attempts");
            drop(held);
        });
        let started = Instant::now();
        let lease = pool.acquire_resampling(2 * GIB, LeasePurpose::Ingress, NEVER).expect("admitted after the release");
        releaser.join().unwrap();
        assert!(started.elapsed() < Duration::from_secs(4), "woken by the release, not the 5 s tick: {:?}", started.elapsed());
        assert_eq!(pool.reserved(LeasePurpose::Ingress), lease.bytes());
        assert_eq!(pool.status(JobCounts::default()).last_refusal, None);
    }

    /// Waiters resuming together after a refusal never over-admit: with
    /// headroom for 2.5 leases, exactly two of six are admitted.
    #[test]
    fn concurrent_waiters_resuming_from_insufficient_headroom_never_over_admit() {
        let chunk = GIB;
        let host = ObservedHost::with(Some(64 * GIB), Some(0), PressureLevel::Normal);
        // The window outlasts the test; the waiters left refused end on cancellation.
        let pool = timed_pool(&host, Duration::from_secs(30), Duration::from_millis(5));
        let samples = host.observe(None);
        let (start, done) = (Rendezvous::new(7), Rendezvous::new(7));
        let mut waiters =
            Waiters { pool: pool.clone(), cancel: Arc::default(), rendezvous: vec![start.clone(), done.clone()], handles: Vec::new() };
        for _ in 0..6 {
            let (pool, start, done, cancel) = (pool.clone(), start.clone(), done.clone(), waiters.cancel.clone());
            waiters.handles.push(std::thread::spawn(move || {
                let _abort = AbortOnUnwind(vec![start.clone(), done.clone()]);
                start.arrive();
                let lease = pool.acquire_resampling(chunk, LeasePurpose::Ingress, &|| cancel.load(Ordering::SeqCst));
                done.arrive(); // every lease is held until all waits end
                lease.is_ok()
            }));
        }
        start.arrive();
        // Waits until each of the six waiters has sampled the host at `step`.
        // A waiter that ends before the coordinator releases it has panicked.
        let each_waiter_sampled = |step: usize| {
            let mut sampled_by = HashSet::new();
            while sampled_by.len() < 6 {
                let sampled = samples.recv_timeout(Duration::from_secs(10)).expect("every waiter samples the host");
                assert!(!waiters.handles.iter().any(std::thread::JoinHandle::is_finished), "a waiter ended early");
                if sampled.step == step {
                    sampled_by.insert(sampled.thread);
                }
            }
        };
        // Every waiter is refused before the headroom changes, and every one
        // attempts again on the new headroom before the rest are cancelled.
        each_waiter_sampled(0);
        host.set(|s| s.headroom_bytes = Some(chunk * 5 / 2));
        each_waiter_sampled(1);
        waiters.cancel.store(true, Ordering::SeqCst);
        pool.wake_waiters();
        done.arrive();
        // Every waiter is joined before any waiter's panic is propagated.
        let joined: Vec<_> = waiters.handles.drain(..).map(std::thread::JoinHandle::join).collect();
        let admitted = joined.into_iter().map(|j| j.unwrap_or_else(|panic| std::panic::resume_unwind(panic))).filter(|ok| *ok).count();
        assert_eq!(admitted, 2);
        assert_eq!(pool.reserved_total(), 0);
    }

    #[test]
    fn refusals_and_status_serialize_with_stable_names() {
        let refusal = serde_json::to_value(ResourceRefusal::PressureElevated { level: PressureLevel::Warning }).unwrap();
        assert_eq!(refusal, serde_json::json!({"kind":"pressure_elevated","level":"warning"}));
        assert_eq!(
            serde_json::to_value(ResourceRefusal::WorkerGrowthRefused {
                granted_bytes: 2,
                requested_bytes: 4,
                cause: Box::new(ResourceRefusal::PressureUnknown)
            })
            .unwrap(),
            serde_json::json!({"kind":"worker_growth_refused","granted_bytes":2,"requested_bytes":4,"cause":{"kind":"pressure_unknown"}})
        );
        assert_eq!(
            serde_json::to_value(ResourceRefusal::WorkerCapExceeded { cap_bytes: 1, requested_bytes: 2 }).unwrap(),
            serde_json::json!({"kind":"worker_cap_exceeded","cap_bytes":1,"requested_bytes":2})
        );
        assert_eq!(
            serde_json::to_value(ResourceRefusal::WorkerFootprintExceeded { grant_bytes: 1, footprint_bytes: 2 }).unwrap(),
            serde_json::json!({"kind":"worker_footprint_exceeded","grant_bytes":1,"footprint_bytes":2})
        );
        assert_eq!(serde_json::to_value(ResourceRefusal::PressureUnknown).unwrap(), serde_json::json!({"kind":"pressure_unknown"}));
        assert_eq!(
            serde_json::to_value(ResourceRefusal::PoolAboveRecommendedUnacknowledged { pool_bytes: 2, recommended_bytes: 1 }).unwrap(),
            serde_json::json!({"kind":"pool_above_recommended_unacknowledged","pool_bytes":2,"recommended_bytes":1})
        );
        let host = ScriptedHost::normal();
        let status = serde_json::to_value(pool(&host).status(JobCounts { queued: 1, running: 1, retained: 0, custody: 2 })).unwrap();
        for key in [
            "capacity_bytes", "recommended_pool_bytes", "pool_bytes", "pool_above_recommended", "invocation_budget_bytes",
            "default_invocation_budget_bytes", "max_invocation_budget_bytes", "reserved_bytes", "reserved_by_purpose", "running_workers",
            "queued_jobs", "running_jobs", "retained_results", "webview_custody_results", "headroom",
            "ordinary_route_pricing", "last_refusal",
        ] {
            assert!(status.get(key).is_some(), "{key}");
        }
        assert_eq!(status["ordinary_route_pricing"], "worker_capped");
        assert!(status.get("unpriced_solve_running").is_none());
        assert!(status.get("qualified_invocation_budget_bytes").is_none(), "one current contract");
        assert_eq!(status["default_invocation_budget_bytes"], DEFAULT_INVOCATION_BUDGET_BYTES);
        assert_eq!(status["max_invocation_budget_bytes"], 12 * GIB);
        assert_eq!(status["headroom"]["pressure"], "normal");
        assert_eq!(status["reserved_by_purpose"], serde_json::json!({"ingress":0,"result":0,"custody":0,"worker":0}));
        assert_eq!(status["running_workers"], 0);
        assert_eq!(status["webview_custody_results"], 2);
    }

    /// Two held leases of 1 GiB (no residency credit) on a host whose headroom
    /// is then scripted; a growth by `delta` needs delta + every reservation.
    fn held_leases(host: &Arc<ObservedHost>, pool: &Arc<MemoryPool>) -> (Lease, Lease) {
        host.set(|s| s.headroom_bytes = Some(100 * GIB));
        let a = pool.acquire(GIB, LeasePurpose::Ingress).unwrap();
        let b = pool.acquire(GIB, LeasePurpose::Result).unwrap();
        host.set(|s| s.headroom_bytes = Some(0));
        (a, b)
    }

    #[test]
    fn a_growth_step_re_samples_and_takes_the_first_target_a_fresh_sample_admits() {
        let host = ObservedHost::with(Some(128 * GIB), Some(0), PressureLevel::Normal);
        let pool = timed_pool(&host, Duration::from_secs(10), Duration::from_millis(10));
        let (mut a, _b) = held_leases(&host, &pool);
        // From 100 ms on, the headroom covers growing `a` by 1 GiB (1 + 2 GiB
        // required) but not by 3 GiB (3 + 2): the minimal target is admitted.
        let _samples = host.observe(Some((Duration::from_millis(100), 3 * GIB)));
        let started = Instant::now();
        let grown = a.grow_resampling(&[4 * GIB, 2 * GIB], &mut || false).expect("admitted after the dip");
        assert!(started.elapsed() >= Duration::from_millis(100), "{:?}", started.elapsed());
        assert_eq!((grown, a.bytes()), (2 * GIB, 2 * GIB));
        assert_eq!(pool.reserved(LeasePurpose::Ingress), 2 * GIB);
        assert_eq!(pool.status(JobCounts::default()).last_refusal, None, "a passing re-sample leaves no refusal");
        // With room for both, the preferred target comes first (the refill ends).
        let _samples = host.observe(None);
        host.set(|s| s.headroom_bytes = Some(100 * GIB));
        assert_eq!(a.grow_resampling(&[4 * GIB, 3 * GIB], &mut || false).unwrap(), 4 * GIB);
        drop(a);
        assert_eq!(pool.reserved(LeasePurpose::Ingress), 0);
    }

    #[test]
    fn a_growth_step_refused_for_the_whole_window_stands_and_grows_nothing() {
        let host = ObservedHost::with(Some(128 * GIB), Some(0), PressureLevel::Normal);
        let pool = timed_pool(&host, Duration::from_millis(150), Duration::from_millis(10));
        let (mut a, _b) = held_leases(&host, &pool);
        let started = Instant::now();
        let refusal = refused(a.grow_resampling(&[4 * GIB, 2 * GIB], &mut || false).unwrap_err());
        let waited = started.elapsed();
        assert!(waited >= Duration::from_millis(150) && waited < Duration::from_secs(10), "{waited:?}");
        assert_eq!(refusal, ResourceRefusal::HeadroomInsufficient { required_bytes: 3 * GIB, available_bytes: 0 }, "the minimal target's refusal");
        assert_eq!(pool.status(JobCounts::default()).last_refusal, Some(refusal));
        assert_eq!((a.bytes(), pool.reserved_total()), (GIB, 2 * GIB));
    }

    #[test]
    fn a_growth_step_refused_otherwise_stands_at_once_and_stop_ends_the_wait() {
        let host = ObservedHost::with(Some(128 * GIB), Some(0), PressureLevel::Normal);
        let pool = timed_pool(&host, Duration::from_secs(60), Duration::from_millis(10));
        let (mut a, _b) = held_leases(&host, &pool);
        host.set(|s| {
            s.headroom_bytes = Some(100 * GIB);
            s.pressure = PressureLevel::Warning;
        });
        let started = Instant::now();
        let refusal = refused(a.grow_resampling(&[2 * GIB], &mut || panic!("no wait")).unwrap_err());
        assert!(started.elapsed() < Duration::from_secs(5));
        assert!(!matches!(refusal, ResourceRefusal::HeadroomInsufficient { .. }), "{refusal:?}");
        host.set(|s| {
            s.headroom_bytes = Some(0);
            s.pressure = PressureLevel::Normal;
        });
        let mut calls = 0;
        let stopped = a.grow_resampling(&[2 * GIB], &mut || {
            calls += 1;
            calls == 3
        });
        assert!(matches!(stopped, Err(ResampleStop::Cancelled)), "{stopped:?}");
        assert_eq!((calls, a.bytes(), pool.reserved_total()), (3, GIB, 2 * GIB));
    }

    /// `waiting` runs once, before the first wait, and only when the step
    /// waits: never for a step admitted at once or refused otherwise.
    #[test]
    fn a_growth_step_calls_waiting_once_and_only_before_it_waits() {
        let host = ObservedHost::with(Some(128 * GIB), Some(0), PressureLevel::Normal);
        let pool = timed_pool(&host, Duration::from_millis(100), Duration::from_millis(5));
        let (mut a, _b) = held_leases(&host, &pool);
        let mut waits = 0;
        let refusal = refused(a.grow_resampling_with(&mut || vec![2 * GIB], &mut || waits += 1, &mut || false).unwrap_err());
        assert!(matches!(refusal, ResourceRefusal::HeadroomInsufficient { .. }), "{refusal:?}");
        assert_eq!(waits, 1, "one call for the whole window");
        host.set(|s| s.pressure = PressureLevel::Warning);
        assert!(a.grow_resampling_with(&mut || vec![2 * GIB], &mut || panic!("no wait"), &mut || panic!("no wait")).is_err());
        host.set(|s| {
            s.pressure = PressureLevel::Normal;
            s.headroom_bytes = Some(100 * GIB);
        });
        assert_eq!(a.grow_resampling_with(&mut || vec![2 * GIB], &mut || panic!("no wait"), &mut || panic!("no wait")), Ok(2 * GIB));
    }

    /// Concurrent growth waiters: when the headroom covers one step, exactly
    /// one is admitted, and the other's next verdict counts it.
    #[test]
    fn concurrent_growth_waiters_never_over_admit() {
        for _ in 0..5 {
            let host = ObservedHost::with(Some(128 * GIB), Some(0), PressureLevel::Normal);
            let pool = timed_pool(&host, Duration::from_millis(400), Duration::from_millis(5));
            let (a, b) = held_leases(&host, &pool);
            // 3 GiB admits one 1 GiB step (1 + 2 required), never the second (1 + 3).
            let _samples = host.observe(Some((Duration::from_millis(50), 3 * GIB)));
            let grow = |mut lease: Lease| {
                std::thread::spawn(move || {
                    let target = lease.bytes() + GIB;
                    let outcome = lease.grow_resampling(&[target], &mut || false).map_err(refused);
                    (outcome, lease)
                })
            };
            let (ha, hb) = (grow(a), grow(b));
            let (ra, la) = ha.join().unwrap();
            let (rb, lb) = hb.join().unwrap();
            assert_eq!(usize::from(ra.is_ok()) + usize::from(rb.is_ok()), 1, "{ra:?} {rb:?}");
            let refusal = ra.err().or(rb.err()).unwrap();
            assert_eq!(refusal, ResourceRefusal::HeadroomInsufficient { required_bytes: 4 * GIB, available_bytes: 3 * GIB });
            assert_eq!(la.bytes() + lb.bytes(), 3 * GIB);
            assert_eq!(pool.reserved_total(), 3 * GIB);
            drop((la, lb));
            assert_eq!(pool.reserved_total(), 0);
        }
    }

    #[test]
    fn absorb_moves_reserved_bytes_without_releasing_or_admitting_them() {
        let host = ScriptedHost::normal();
        let pool = pool(&host);
        let mut result = pool.acquire(GIB, LeasePurpose::Result).unwrap();
        result.set_purpose(LeasePurpose::Custody);
        let extra = pool.acquire(GIB, LeasePurpose::Custody).unwrap();
        assert_eq!(pool.reserved(LeasePurpose::Custody), 2 * GIB);
        host.set(|s| s.headroom_bytes = None);
        result.absorb(extra);
        assert_eq!((result.bytes(), pool.reserved(LeasePurpose::Custody), pool.reserved_total()), (2 * GIB, 2 * GIB, 2 * GIB));
        let ingress = pool.acquire(MIB, LeasePurpose::Ingress);
        assert!(ingress.is_err(), "absorb admitted nothing: an unknown headroom still refuses");
        drop(result);
        assert_eq!(pool.reserved_total(), 0);
    }

    /// Review round 1 (P2): the targets are evaluated at every attempt, so a
    /// need measured between attempts (a stopped worker's footprint) is what an
    /// admitted step covers; a need already within the lease grows nothing.
    #[test]
    fn a_growth_step_follows_a_need_measured_between_attempts() {
        let host = ObservedHost::with(Some(128 * GIB), Some(0), PressureLevel::Normal);
        let pool = timed_pool(&host, Duration::from_secs(10), Duration::from_millis(10));
        let (mut a, _b) = held_leases(&host, &pool);
        let _samples = host.observe(Some((Duration::from_millis(100), 100 * GIB)));
        let need = std::cell::Cell::new(2 * GIB);
        let grown = a
            .grow_resampling_with(&mut || vec![need.get()], &mut || {}, &mut || {
                need.set(5 * GIB);
                false
            })
            .expect("admitted after the dip");
        assert_eq!((grown, a.bytes(), pool.reserved(LeasePurpose::Ingress)), (5 * GIB, 5 * GIB, 5 * GIB));
        assert_eq!(a.grow_resampling_with(&mut || vec![6 * GIB, 4 * GIB], &mut || {}, &mut || false), Ok(5 * GIB), "the need is within the lease");
        assert_eq!(a.bytes(), 5 * GIB);
    }

    /// Codex review r2 of #1665 (the budget probe): with the budget at the
    /// demo's admitted ingress lease (envelope(raw) + M + 4 KiB = 196,752 B,
    /// M = 176,272 B), growing it by 8 M to 1,606,928 B was admitted, because
    /// growth admitted only the delta. It is refused at the budget now.
    #[test]
    fn growing_an_interactive_ingress_lease_above_the_budget_is_refused() {
        const BUDGET: u64 = 196_752;
        const M: u64 = 176_272;
        let host = ScriptedHost::with(Some(64 * GIB), Some(64 * GIB), PressureLevel::Normal);
        let pool = pool(&host);
        pool.configure(48 * GIB, BUDGET, false).unwrap();
        let mut lease = pool.acquire_interactive(BUDGET).unwrap();
        let refusal = lease.grow_to(lease.bytes() + 8 * M).unwrap_err();
        assert_eq!(refusal, ResourceRefusal::IngressExceedsBudget { required_bytes: 1_606_928, budget_bytes: BUDGET });
        assert_eq!((lease.bytes(), pool.reserved(LeasePurpose::Ingress)), (BUDGET, BUDGET), "the lease and the reservation are unchanged");
        assert_eq!(pool.status(JobCounts::default()).last_refusal, Some(refusal));
    }

    /// A lease's growth is bounded by the budget in force when it was admitted:
    /// lowering the budget afterwards does not stop it, and raising it does
    /// not extend it. A lease admitted after the change takes the new budget.
    #[test]
    fn growth_is_bounded_by_the_budget_the_lease_was_admitted_under() {
        let host = ScriptedHost::with(Some(64 * GIB), Some(64 * GIB), PressureLevel::Normal);
        let pool = pool(&host);
        pool.configure(48 * GIB, 4 * MIB, false).unwrap();
        let mut ingress = pool.acquire_interactive(MIB).unwrap();
        let mut worker = pool.acquire(MIB, LeasePurpose::Worker).unwrap();
        pool.configure(48 * GIB, 2 * MIB, false).unwrap();
        ingress.grow_to(4 * MIB).expect("lowering the budget does not stop a running invocation");
        worker.grow_to(4 * MIB).expect("nor a running worker");
        pool.configure(48 * GIB, 8 * MIB, false).unwrap();
        assert_eq!(
            ingress.grow_to(4 * MIB + 1).unwrap_err(),
            ResourceRefusal::IngressExceedsBudget { required_bytes: 4 * MIB + 1, budget_bytes: 4 * MIB },
            "raising the budget does not extend it"
        );
        assert_eq!(
            worker.grow_to(4 * MIB + 1).unwrap_err(),
            ResourceRefusal::WorkerCapExceeded { cap_bytes: 4 * MIB, requested_bytes: 4 * MIB + 1 }
        );
        let mut later = pool.acquire_interactive(MIB).unwrap();
        later.grow_to(8 * MIB).expect("a lease admitted after the change takes the new budget");
    }

    /// A queued job's worker lease is bounded by the budget the job was
    /// admitted under (its `max_grant`), even when the budget changes before
    /// the worker is acquired: lowered, the worker still grows to it; raised,
    /// it is not extended.
    #[test]
    fn a_worker_admitted_after_a_budget_change_keeps_its_jobs_budget() {
        let host = ScriptedHost::with(Some(64 * GIB), Some(64 * GIB), PressureLevel::Normal);
        let pool = pool(&host);
        pool.configure(48 * GIB, 8 * MIB, false).unwrap();
        let job_budget = 8 * MIB;
        pool.configure(48 * GIB, 4 * MIB, false).unwrap();
        let mut worker = pool.wait_acquire(MIB, LeasePurpose::Worker, Some(job_budget), &|| false, &mut |_| {}).unwrap();
        worker.grow_to(6 * MIB).expect("lowered before the worker started: the job's budget still holds");
        worker.grow_to(8 * MIB).unwrap();
        pool.configure(48 * GIB, 16 * MIB, false).unwrap();
        assert_eq!(
            worker.grow_to(8 * MIB + 1).unwrap_err(),
            ResourceRefusal::WorkerCapExceeded { cap_bytes: job_budget, requested_bytes: 8 * MIB + 1 }
        );
    }

    #[test]
    #[should_panic(expected = "only a result or custody lease absorbs")]
    fn an_ingress_lease_cannot_absorb() {
        let host = ScriptedHost::with(Some(64 * GIB), Some(64 * GIB), PressureLevel::Normal);
        let pool = pool(&host);
        pool.configure(48 * GIB, 4 * MIB, false).unwrap();
        let mut a = pool.acquire_interactive(3 * MIB).unwrap();
        a.absorb(pool.acquire(3 * MIB, LeasePurpose::Ingress).unwrap());
    }

    #[test]
    #[should_panic(expected = "a lease's purpose moves forward only")]
    fn a_result_lease_cannot_return_to_ingress() {
        let host = ScriptedHost::with(Some(64 * GIB), Some(64 * GIB), PressureLevel::Normal);
        let pool = pool(&host);
        let mut lease = pool.acquire(MIB, LeasePurpose::Result).unwrap();
        lease.set_purpose(LeasePurpose::Ingress);
    }

    /// `grow_to` admits an ingress lease or a worker grant grown exactly to
    /// the budget and refuses one byte more with the purpose's code, leaving
    /// the lease and the reservation unchanged. Result and custody leases are
    /// bounded by the pool, not the budget.
    #[test]
    fn growth_to_the_budget_is_admitted_and_one_byte_more_is_refused_by_purpose() {
        let host = ScriptedHost::with(Some(64 * GIB), Some(64 * GIB), PressureLevel::Normal);
        let pool = pool(&host);
        let budget = 4 * MIB;
        pool.configure(48 * GIB, budget, false).unwrap();
        let interactive = || pool.acquire_interactive(MIB).unwrap();
        let adaptive = |purpose| pool.acquire(MIB, purpose).unwrap();
        let ingress_over = ResourceRefusal::IngressExceedsBudget { required_bytes: budget + 1, budget_bytes: budget };
        let worker_over = ResourceRefusal::WorkerCapExceeded { cap_bytes: budget, requested_bytes: budget + 1 };
        let cases: [(&dyn Fn() -> Lease, LeasePurpose, ResourceRefusal); 3] = [
            (&interactive, LeasePurpose::Ingress, ingress_over.clone()),
            (&|| adaptive(LeasePurpose::Ingress), LeasePurpose::Ingress, ingress_over),
            (&|| adaptive(LeasePurpose::Worker), LeasePurpose::Worker, worker_over),
        ];
        for (acquire, purpose, over) in cases {
            let mut lease = acquire();
            lease.grow_to(budget).unwrap();
            assert_eq!((lease.bytes(), pool.reserved(purpose)), (budget, budget), "{purpose:?}: grown to the budget");
            assert_eq!(lease.grow_to(budget + 1).unwrap_err(), over, "{purpose:?}");
            assert_eq!((lease.bytes(), pool.reserved(purpose), pool.reserved_total()), (budget, budget, budget), "{purpose:?}: unchanged");
            assert_eq!(pool.status(JobCounts::default()).last_refusal, Some(over));
            if purpose == LeasePurpose::Worker {
                assert_eq!(pool.lock().workers.values().map(|entry| entry.bytes).collect::<Vec<_>>(), vec![budget], "the worker ledger is unchanged");
            }
            drop(lease);
            assert_eq!(pool.reserved_total(), 0);
        }
        for purpose in [LeasePurpose::Result, LeasePurpose::Custody] {
            let mut lease = pool.acquire(MIB, purpose).unwrap();
            lease.grow_to(2 * budget).unwrap();
            assert_eq!((lease.bytes(), pool.reserved(purpose)), (2 * budget, 2 * budget), "{purpose:?}: not bounded by the budget");
        }
    }

    /// The same for a re-sampling growth step: to the budget is admitted, one
    /// byte more is refused at once with the purpose's code even while the
    /// headroom is short (a budget refusal is never re-sampled), and a
    /// preferred target above the budget falls back to a minimal one within it.
    #[test]
    fn a_growth_step_is_bounded_by_the_budget_and_its_refusal_never_waits() {
        let host = ObservedHost::with(Some(128 * GIB), Some(100 * GIB), PressureLevel::Normal);
        let pool = timed_pool(&host, Duration::from_secs(60), Duration::from_millis(10));
        let budget = 4 * GIB;
        pool.configure(96 * GIB, budget, false).unwrap();
        for (purpose, over) in [
            (LeasePurpose::Ingress, ResourceRefusal::IngressExceedsBudget { required_bytes: budget + 1, budget_bytes: budget }),
            (LeasePurpose::Worker, ResourceRefusal::WorkerCapExceeded { cap_bytes: budget, requested_bytes: budget + 1 }),
        ] {
            host.set(|s| s.headroom_bytes = Some(100 * GIB));
            let mut lease = pool.acquire(GIB, purpose).unwrap();
            // A preferred target above the budget; the minimal one within it is admitted.
            assert_eq!(lease.grow_resampling(&[2 * budget, 2 * GIB], &mut || false), Ok(2 * GIB), "{purpose:?}");
            assert_eq!(lease.grow_resampling(&[budget], &mut || panic!("no wait")), Ok(budget), "{purpose:?}: to the budget");
            assert_eq!(pool.reserved(purpose), budget);
            // Headroom that would make any admission wait does not delay a budget refusal.
            host.set(|s| s.headroom_bytes = Some(0));
            let started = Instant::now();
            let refusal = refused(
                lease.grow_resampling_with(&mut || vec![2 * budget, budget + 1], &mut || panic!("no wait"), &mut || panic!("no wait")).unwrap_err(),
            );
            assert!(started.elapsed() < Duration::from_secs(5), "{:?}", started.elapsed());
            assert_eq!(refusal, over, "{purpose:?}: the minimal target's refusal");
            assert_eq!((lease.bytes(), pool.reserved(purpose), pool.reserved_total()), (budget, budget, budget), "{purpose:?}: unchanged");
            assert_eq!(pool.status(JobCounts::default()).last_refusal, Some(over));
            drop(lease);
        }
        assert_eq!(pool.reserved_total(), 0);
    }
}
