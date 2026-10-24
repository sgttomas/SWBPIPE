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
//! interactive command's ingress lease (`crate::command_ingress`). Result and
//! custody leases are bounded by the pool, a fresh headroom verdict and the
//! retention/custody count limits, not by the budget.
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
//! within the same window (`nativeIngress.ts`). Worker growth and the custody
//! grow at take do not re-sample.

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
        debug_assert_ne!(purpose, LeasePurpose::Worker, "worker grants wait in the queue instead");
        let deadline = Instant::now().checked_add(self.resample_window);
        let mut state = self.lock();
        loop {
            let refusal = match self.verdict_locked(&mut state, bytes, purpose) {
                Ok(()) => return Ok(self.record_locked(&mut state, bytes, purpose)),
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

    fn acquire_locked(self: &Arc<Self>, state: &mut PoolState, bytes: u64, purpose: LeasePurpose) -> Result<Lease, ResourceRefusal> {
        if let Err(refusal) = self.verdict_locked(state, bytes, purpose) {
            state.last_refusal = Some(refusal.clone());
            return Err(refusal);
        }
        Ok(self.record_locked(state, bytes, purpose))
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
    fn record_locked(self: &Arc<Self>, state: &mut PoolState, bytes: u64, purpose: LeasePurpose) -> Lease {
        state.reserved[purpose as usize] += bytes;
        let worker_id = (purpose == LeasePurpose::Worker).then(|| {
            let id = state.next_worker_id;
            state.next_worker_id += 1;
            state.workers.insert(id, WorkerEntry { bytes, probe: None });
            id
        });
        Lease { pool: Arc::clone(self), bytes, purpose, class: AdmissionClass::Adaptive, worker_id }
    }

    /// Admits an interactive command's ingress lease of `bytes` in one
    /// critical section ([`AdmissionClass::Interactive`]): a known capacity,
    /// `bytes` within the per-invocation budget (`ingress_exceeds_budget`),
    /// the pool, and a fresh known headroom covering `bytes` plus the
    /// outstanding unrealized reservations. The pressure state is not
    /// consulted. It is not re-sampled here: most interactive commands run on
    /// the main thread (see the module docs).
    pub(crate) fn acquire_interactive(self: &Arc<Self>, bytes: u64) -> Result<Lease, ResourceRefusal> {
        let mut state = self.lock();
        let verdict = (|| {
            let sample = self.host.sample();
            state.observe(&sample);
            if sample.capacity_bytes.is_none() {
                return Err(ResourceRefusal::CapacityUnknown);
            }
            if bytes > state.invocation_budget_bytes {
                return Err(ResourceRefusal::IngressExceedsBudget {
                    required_bytes: bytes,
                    budget_bytes: state.invocation_budget_bytes,
                });
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
        cancelled: &dyn Fn() -> bool,
        on_queued: &mut dyn FnMut(&ResourceRefusal),
    ) -> Result<Lease, WaitOutcome> {
        loop {
            if cancelled() {
                return Err(WaitOutcome::Cancelled);
            }
            let (verdict, generation) = {
                let mut state = self.lock();
                (self.acquire_locked(&mut state, bytes, purpose), state.generation)
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
}

impl Lease {
    pub(crate) fn bytes(&self) -> u64 {
        self.bytes
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn purpose(&self) -> LeasePurpose {
        self.purpose
    }

    /// Admits only the delta, with the same atomic verdict as `acquire`.
    pub(crate) fn grow_to(&mut self, new_bytes: u64) -> Result<(), ResourceRefusal> {
        if new_bytes <= self.bytes {
            self.shrink_to(new_bytes);
            return Ok(());
        }
        let delta = new_bytes - self.bytes;
        let mut state = self.pool.lock();
        if let Err(refusal) = self.pool.admit_bytes_locked(&mut state, delta, self.class) {
            state.last_refusal = Some(refusal.clone());
            return Err(refusal);
        }
        state.reserved[self.purpose as usize] += delta;
        if let Some(entry) = self.worker_id.and_then(|id| state.workers.get_mut(&id)) {
            entry.bytes = new_bytes;
        }
        self.bytes = new_bytes;
        Ok(())
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
        debug_assert!(purpose != LeasePurpose::Worker && self.purpose != LeasePurpose::Worker);
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
            let lease = waiter.wait_acquire(2 * GIB, LeasePurpose::Worker, &|| false, &mut |r| {
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
            let outcome = waiter.wait_acquire(GIB, LeasePurpose::Worker, &|| flag.load(Ordering::SeqCst), &mut |r| {
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
            let lease = waiter.wait_acquire(GIB, LeasePurpose::Worker, &|| false, &mut |r| {
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
        const WINDOW: Duration = Duration::from_millis(300);
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
            // went on towards reacquiring it, with time left before the
            // deadline (at least the window after `started`): the waiter then
            // reacquires the lock past the deadline, not before.
            if holder_locked_by.get().is_some_and(|at| at < started + WINDOW) {
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
        let each_waiter_sampled = |step: usize| {
            let mut waiters = HashSet::new();
            while waiters.len() < 6 {
                let sampled = samples.recv_timeout(Duration::from_secs(10)).expect("every waiter samples the host");
                if sampled.step == step {
                    waiters.insert(sampled.thread);
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
        let admitted = waiters.handles.drain(..).map(|h| h.join().unwrap()).filter(|ok| *ok).count();
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
}
