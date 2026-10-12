//! Host-owned aggregate memory ledger (DEL-04-07 `memory-budget-decision.md`,
//! owner-adopted 2026-10-11).
//!
//! Policy (fixed by the owner): a nominal pool of floor(3 * physical / 4)
//! bytes; a qualified per-invocation default of 12 GiB; a lower user budget is
//! allowed and a larger one is refused as unqualified; the budget never exceeds
//! the pool; a pool above the recommended fraction needs an explicit
//! acknowledgement and never exceeds physical capacity. Acknowledgement never
//! bypasses headroom, pressure or qualification refusals.
//!
//! What the per-invocation budget bounds: each ordinary solve's worker grant
//! (owner decision 2026-10-12, "(c) Capped worker process"). The solve runs in
//! a local child process whose allocator is capped at its grant; the grant
//! starts at the worker's process reserve plus the request's ingress bound and
//! grows on demand, each step an atomic admission with a fresh headroom
//! verdict, never above the budget (`crate::solve_worker`). Result and custody
//! leases are bounded by the pool, a fresh headroom verdict and the
//! retention/custody count limits, not by the budget.
//!
//! Every priced admission is atomic: under one lock it checks capacity, the
//! pool's remaining bytes, a fresh host sample (age, pressure, headroom) and
//! records the reservation, so racing admissions serialize and cannot both
//! consume the same pool bytes or headroom. A [`Lease`] releases exactly once
//! when dropped, including during unwinding.
//!
//! Worker growth is the one admission whose headroom verdict credits measured
//! residency: each worker lease's last reported physical footprint (at most
//! its bytes) is already resident or compressed, so it is not also required
//! from the free queue. Every other admission double counts all reservations.

use serde::Serialize;
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};

/// The owner-authorized, qualified per-invocation budget: 12 GiB. It caps each
/// ordinary solve's worker grant; it does not bound result retention or custody.
pub(crate) const QUALIFIED_INVOCATION_BUDGET_BYTES: u64 = 12_884_901_888;
/// A host sample older than this cannot admit work.
pub(crate) const HEADROOM_SAMPLE_MAX_AGE: Duration = Duration::from_secs(1);
/// How often a queued job re-samples the host while waiting for its grant.
pub(crate) const ADMISSION_WAIT_TICK: Duration = Duration::from_millis(200);
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
    BudgetUnqualified { requested_bytes: u64, qualified_bytes: u64 },
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
    /// A growth step of a running worker's grant was refused; the job ended.
    WorkerGrowthRefused { granted_bytes: u64, requested_bytes: u64, cause: Box<ResourceRefusal> },
    /// The worker's measured physical footprint, with the process reserve,
    /// needs more than the per-invocation budget (at a growth exchange), or
    /// its lifetime-maximum footprint exceeded the budget (at the result).
    WorkerFootprintExceeded { grant_bytes: u64, footprint_bytes: u64 },
    /// Running solve workers are at the process-count bound; the job waits.
    WorkerLimitReached { running: usize, limit: usize },
}

/// What a lease's bytes cover.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LeasePurpose {
    /// The buffered raw body and its native materialization while queued/solving.
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
    /// Live worker leases (one per worker process not yet reaped).
    worker_leases: usize,
    /// The bound on live worker leases ([`max_running_workers`]).
    max_running_workers: usize,
    /// Sum over worker leases of min(last reported footprint, bytes).
    worker_footprint_credit: u64,
    /// Bumped on every release or wake so a waiter does not miss a change.
    generation: u64,
    last_refusal: Option<ResourceRefusal>,
}

impl PoolState {
    fn reserved_total(&self) -> u64 {
        self.reserved.iter().fold(0u64, |sum, v| sum.saturating_add(*v))
    }

    /// Records a known capacity; the first one applies the default policy.
    fn observe(&mut self, sample: &HostSample) {
        let Some(capacity) = sample.capacity_bytes else { return };
        self.capacity_bytes = Some(capacity);
        if !self.defaults_applied {
            self.defaults_applied = true;
            self.pool_bytes = recommended_pool_bytes(capacity).unwrap_or(0);
            self.invocation_budget_bytes = QUALIFIED_INVOCATION_BUDGET_BYTES.min(self.pool_bytes);
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
    pub qualified_invocation_budget_bytes: u64,
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

pub(crate) struct MemoryPool {
    host: Arc<dyn HostMemory>,
    state: Mutex<PoolState>,
    changed: Condvar,
    wait_tick: Duration,
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
        Self::with_wait_tick(host, ADMISSION_WAIT_TICK)
    }

    pub(crate) fn with_wait_tick(host: Arc<dyn HostMemory>, wait_tick: Duration) -> Arc<Self> {
        let mut state = PoolState { max_running_workers: max_running_workers(), ..PoolState::default() };
        state.observe(&host.sample());
        Arc::new(Self { host, state: Mutex::new(state), changed: Condvar::new(), wait_tick })
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
    /// `resident_credit` (worker growth only) is the reserved bytes already
    /// measured resident, which the free queue need not cover again.
    fn admit_bytes_locked(&self, state: &mut PoolState, bytes: u64, resident_credit: u64) -> Result<(), ResourceRefusal> {
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
        check_pressure(&sample)?;
        let available = sample.headroom_bytes.ok_or(ResourceRefusal::HeadroomUnknown)?;
        // Deliberately conservative double counting: bytes already reserved may
        // not yet be resident, so the host's headroom must cover them as well
        // as the new request (less any reserved bytes measured resident).
        let required = bytes.saturating_add(reserved.saturating_sub(resident_credit));
        if available < required {
            return Err(ResourceRefusal::HeadroomInsufficient { required_bytes: required, available_bytes: available });
        }
        Ok(())
    }

    /// Admits and records `bytes` for `purpose` in one critical section.
    pub(crate) fn acquire(self: &Arc<Self>, bytes: u64, purpose: LeasePurpose) -> Result<Lease, ResourceRefusal> {
        let mut state = self.lock();
        self.acquire_locked(&mut state, bytes, purpose)
    }

    fn acquire_locked(self: &Arc<Self>, state: &mut PoolState, bytes: u64, purpose: LeasePurpose) -> Result<Lease, ResourceRefusal> {
        let limit = state.max_running_workers;
        let verdict = if purpose == LeasePurpose::Worker && state.worker_leases >= limit {
            Err(ResourceRefusal::WorkerLimitReached { running: state.worker_leases, limit })
        } else {
            self.admit_bytes_locked(state, bytes, 0)
        };
        if let Err(refusal) = verdict {
            state.last_refusal = Some(refusal.clone());
            return Err(refusal);
        }
        state.reserved[purpose as usize] += bytes;
        if purpose == LeasePurpose::Worker {
            state.worker_leases += 1;
        }
        Ok(Lease { pool: Arc::clone(self), bytes, purpose, footprint_credit: 0 })
    }

    /// Releases `bytes` of `purpose` (and `credit` of the worker footprint
    /// credit); `ends_worker` when a worker lease is dropped.
    fn release(&self, purpose: LeasePurpose, bytes: u64, credit: u64, ends_worker: bool) {
        let held = {
            let mut state = self.lock();
            let slot = &mut state.reserved[purpose as usize];
            let held = *slot;
            *slot = slot.saturating_sub(bytes);
            state.worker_footprint_credit = state.worker_footprint_credit.saturating_sub(credit);
            if ends_worker {
                state.worker_leases = state.worker_leases.saturating_sub(1);
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
            if invocation_budget_bytes > QUALIFIED_INVOCATION_BUDGET_BYTES {
                return Err(ResourceRefusal::BudgetUnqualified {
                    requested_bytes: invocation_budget_bytes,
                    qualified_bytes: QUALIFIED_INVOCATION_BUDGET_BYTES,
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
            qualified_invocation_budget_bytes: QUALIFIED_INVOCATION_BUDGET_BYTES,
            reserved_bytes: state.reserved_total(),
            reserved_by_purpose: ReservedByPurpose {
                ingress: state.reserved[LeasePurpose::Ingress as usize],
                result: state.reserved[LeasePurpose::Result as usize],
                custody: state.reserved[LeasePurpose::Custody as usize],
                worker: state.reserved[LeasePurpose::Worker as usize],
            },
            running_workers: state.worker_leases,
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
        self.lock().worker_leases
    }
}

/// Reserved pool bytes; released exactly once on drop (also during unwinding).
/// Not `Clone`: ownership moves input -> job -> worker -> result -> custody.
#[derive(Debug)]
pub(crate) struct Lease {
    pool: Arc<MemoryPool>,
    bytes: u64,
    purpose: LeasePurpose,
    /// Worker leases: min(last reported footprint, bytes), counted in the
    /// pool's `worker_footprint_credit`.
    footprint_credit: u64,
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
        if let Err(refusal) = self.pool.admit_bytes_locked(&mut state, delta, 0) {
            state.last_refusal = Some(refusal.clone());
            return Err(refusal);
        }
        state.reserved[self.purpose as usize] += delta;
        self.bytes = new_bytes;
        Ok(())
    }

    /// A running worker's growth step: records its measured physical
    /// footprint (credited up to the lease's bytes as already resident across
    /// all worker leases), then admits the delta with the same atomic verdict
    /// as `acquire`, the headroom requirement being the delta plus every
    /// reservation's unrealized bytes. A refusal keeps the existing grant.
    pub(crate) fn grow_worker_to(&mut self, new_bytes: u64, footprint_bytes: u64) -> Result<(), ResourceRefusal> {
        debug_assert_eq!(self.purpose, LeasePurpose::Worker);
        let mut state = self.pool.lock();
        let credit = footprint_bytes.min(self.bytes);
        state.worker_footprint_credit = state.worker_footprint_credit.saturating_sub(self.footprint_credit).saturating_add(credit);
        self.footprint_credit = credit;
        if new_bytes <= self.bytes {
            return Ok(());
        }
        let delta = new_bytes - self.bytes;
        let resident = state.worker_footprint_credit;
        if let Err(refusal) = self.pool.admit_bytes_locked(&mut state, delta, resident) {
            state.last_refusal = Some(refusal.clone());
            return Err(refusal);
        }
        state.reserved[self.purpose as usize] += delta;
        self.bytes = new_bytes;
        Ok(())
    }

    /// Releases down to `new_bytes`; never fails.
    pub(crate) fn shrink_to(&mut self, new_bytes: u64) {
        if new_bytes >= self.bytes {
            return;
        }
        let delta = self.bytes - new_bytes;
        self.bytes = new_bytes;
        let credit = self.footprint_credit.min(new_bytes);
        let credit_released = self.footprint_credit - credit;
        self.footprint_credit = credit;
        self.pool.release(self.purpose, delta, credit_released, false);
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
        self.pool.release(self.purpose, self.bytes, self.footprint_credit, self.purpose == LeasePurpose::Worker);
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
    use super::test_host::{ScriptedHost, GIB};
    use super::*;
    use std::sync::Barrier;

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
    fn defaults_follow_capacity_and_qualified_budget() {
        for (capacity, pool_bytes, budget) in [
            (16 * GIB, 12 * GIB, QUALIFIED_INVOCATION_BUDGET_BYTES),
            (8 * GIB, 6 * GIB, 6 * GIB),
            (64 * GIB, 48 * GIB, QUALIFIED_INVOCATION_BUDGET_BYTES),
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
        assert_eq!(unknown.settings(), (12 * GIB, QUALIFIED_INVOCATION_BUDGET_BYTES));
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
            worker.grow_worker_to(2 * GIB, GIB / 2).unwrap();
            panic!("solve panicked");
        }));
        assert!(panicked.is_err());
        assert_eq!(pool.reserved_total(), 0);
        assert_eq!(pool.running_workers(), 0);
        assert_eq!(credit(&pool), 0);
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
            pool.configure(14 * GIB, QUALIFIED_INVOCATION_BUDGET_BYTES + 1, true).unwrap_err(),
            ResourceRefusal::BudgetUnqualified {
                requested_bytes: QUALIFIED_INVOCATION_BUDGET_BYTES + 1,
                qualified_bytes: QUALIFIED_INVOCATION_BUDGET_BYTES
            }
        );
        assert_eq!(
            pool.configure(4 * GIB, 5 * GIB, false).unwrap_err(),
            ResourceRefusal::BudgetExceedsPool { budget_bytes: 5 * GIB, pool_bytes: 4 * GIB }
        );
        assert_eq!(pool.settings(), (12 * GIB, QUALIFIED_INVOCATION_BUDGET_BYTES), "refusals change nothing");
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
        let waiter = pool.clone();
        let handle = std::thread::spawn(move || {
            let mut reasons = Vec::new();
            let lease = waiter.wait_acquire(2 * GIB, LeasePurpose::Worker, &|| false, &mut |r| reasons.push(r.clone()));
            (lease.map(|l| l.bytes()), reasons)
        });
        std::thread::sleep(Duration::from_millis(30));
        assert_eq!(pool.reserved_total(), 3 * GIB, "the waiter reserved nothing while refused");
        drop(held);
        let (admitted, reasons) = handle.join().unwrap();
        assert_eq!(admitted, Ok(2 * GIB));
        assert!(!reasons.is_empty() && reasons.iter().all(|r| matches!(r, ResourceRefusal::PoolExhausted { .. })));
        assert_eq!((pool.reserved_total(), pool.running_workers()), (0, 0), "the waiter's lease dropped");

        host.set(|s| s.pressure = PressureLevel::Warning);
        let cancel = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let (waiter, flag) = (pool.clone(), cancel.clone());
        let handle = std::thread::spawn(move || {
            let mut last = None;
            let outcome = waiter.wait_acquire(GIB, LeasePurpose::Worker, &|| flag.load(std::sync::atomic::Ordering::SeqCst), &mut |r| last = Some(r.clone()));
            (outcome.map(drop), last)
        });
        std::thread::sleep(Duration::from_millis(30));
        cancel.store(true, std::sync::atomic::Ordering::SeqCst);
        pool.wake_waiters();
        let (outcome, last) = handle.join().unwrap();
        assert_eq!(outcome, Err(WaitOutcome::Cancelled));
        assert_eq!(last, Some(ResourceRefusal::PressureElevated { level: PressureLevel::Warning }));
        assert_eq!(pool.reserved_total(), 0);
    }

    fn credit(pool: &MemoryPool) -> u64 {
        pool.lock().worker_footprint_credit
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
        let waiter = pool.clone();
        let handle = std::thread::spawn(move || {
            let mut reasons = Vec::new();
            let lease = waiter.wait_acquire(GIB, LeasePurpose::Worker, &|| false, &mut |r| reasons.push(r.clone()));
            (lease.map(|l| l.bytes()), reasons)
        });
        std::thread::sleep(Duration::from_millis(30));
        drop(a);
        let (admitted, reasons) = handle.join().unwrap();
        assert_eq!(admitted, Ok(GIB));
        assert!(!reasons.is_empty() && reasons.iter().all(|r| *r == ResourceRefusal::WorkerLimitReached { running: 2, limit: 2 }));
        assert_eq!(
            serde_json::to_value(ResourceRefusal::WorkerLimitReached { running: 2, limit: 2 }).unwrap(),
            serde_json::json!({"kind":"worker_limit_reached","running":2,"limit":2})
        );
    }

    /// Worker growth credits each worker lease's measured footprint (at most
    /// its bytes) against the double-counted reservations; other admissions
    /// never do.
    #[test]
    fn worker_growth_credits_measured_footprint_only_up_to_each_lease() {
        let host = ScriptedHost::with(Some(64 * GIB), Some(10 * GIB), PressureLevel::Normal);
        let pool = pool(&host);
        pool.set_max_running_workers(4);
        let mut a = pool.acquire(4 * GIB, LeasePurpose::Worker).unwrap();
        let mut b = pool.acquire(4 * GIB, LeasePurpose::Worker).unwrap();
        assert_eq!(pool.status(JobCounts::default()).running_workers, 2);
        // Without credit: 2 GiB + 8 GiB reserved = 10 GiB fits; 3 GiB does not.
        assert_eq!(
            a.grow_worker_to(7 * GIB, 0).unwrap_err(),
            ResourceRefusal::HeadroomInsufficient { required_bytes: 11 * GIB, available_bytes: 10 * GIB }
        );
        // a reports 3 GiB resident: 3 GiB + (8 - 3) GiB = 8 GiB fits.
        a.grow_worker_to(7 * GIB, 3 * GIB).unwrap();
        assert_eq!(a.bytes(), 7 * GIB);
        // A footprint above the lease credits only the lease's bytes: b reports
        // 9 GiB of a 4 GiB lease; required = 4 + (11 - 3 - 4) = 8 GiB.
        b.grow_worker_to(8 * GIB, 9 * GIB).unwrap();
        assert_eq!(credit(&pool), 7 * GIB);
        // A non-worker admission gets no credit: 15 GiB reserved > 10 GiB headroom.
        assert!(matches!(pool.acquire(1, LeasePurpose::Result).unwrap_err(), ResourceRefusal::HeadroomInsufficient { .. }));
        // A refused growth keeps the grant and the recorded credit.
        assert!(a.grow_worker_to(20 * GIB, 7 * GIB).is_err());
        assert_eq!((a.bytes(), credit(&pool)), (7 * GIB, 11 * GIB));
        a.shrink_to(GIB);
        assert_eq!(credit(&pool), 5 * GIB, "credit never exceeds the lease");
        drop(a);
        assert_eq!((credit(&pool), pool.running_workers()), (4 * GIB, 1));
        drop(b);
        assert_eq!((pool.reserved_total(), credit(&pool), pool.running_workers()), (0, 0, 0));
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
            "qualified_invocation_budget_bytes", "reserved_bytes", "reserved_by_purpose", "running_workers",
            "queued_jobs", "running_jobs", "retained_results", "webview_custody_results", "headroom",
            "ordinary_route_pricing", "last_refusal",
        ] {
            assert!(status.get(key).is_some(), "{key}");
        }
        assert_eq!(status["ordinary_route_pricing"], "worker_capped");
        assert!(status.get("unpriced_solve_running").is_none());
        assert_eq!(status["headroom"]["pressure"], "normal");
        assert_eq!(status["reserved_by_purpose"], serde_json::json!({"ingress":0,"result":0,"custody":0,"worker":0}));
        assert_eq!(status["running_workers"], 0);
        assert_eq!(status["webview_custody_results"], 2);
    }
}
