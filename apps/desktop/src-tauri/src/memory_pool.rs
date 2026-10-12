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
//! Every priced admission is atomic: under one lock it checks capacity, the
//! pool's remaining bytes, a fresh host sample (age, pressure, headroom) and
//! records the reservation, so racing admissions serialize and cannot both
//! consume the same pool bytes or headroom. A [`Lease`] releases exactly once
//! when dropped, including during unwinding.
//!
//! The ordinary solve route has no sound whole-invocation byte bound yet (a
//! reserved owner choice), so its solve stage is unpriced: it runs under an
//! exclusive [`UnpricedSolveSlot`] that reserves no bytes and is admitted only
//! with known capacity, a fresh sample, known headroom and normal pressure. No
//! adaptive concurrency is claimed for unpriced work.

use serde::Serialize;
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};

/// The owner-authorized, qualified per-invocation budget: 12 GiB.
pub(crate) const QUALIFIED_INVOCATION_BUDGET_BYTES: u64 = 12_884_901_888;
/// A host sample older than this cannot admit work.
pub(crate) const HEADROOM_SAMPLE_MAX_AGE: Duration = Duration::from_secs(1);
/// How often a queued unpriced solve re-samples the host while waiting.
pub(crate) const UNPRICED_WAIT_TICK: Duration = Duration::from_millis(200);
/// Jobs holding an admitted input while waiting for the solve slot.
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
    SolveSlotBusy,
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
}

#[derive(Debug, Default)]
struct PoolState {
    capacity_bytes: Option<u64>,
    pool_bytes: u64,
    invocation_budget_bytes: u64,
    above_recommended_acknowledged: bool,
    defaults_applied: bool,
    reserved: [u64; 3],
    unpriced_solve_running: bool,
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
        level => Err(ResourceRefusal::PressureElevated { level }),
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
    pub unpriced_solve_running: bool,
    pub queued_jobs: usize,
    pub running_jobs: usize,
    pub retained_results: usize,
    pub webview_custody_results: usize,
    pub headroom: HeadroomStatus,
    pub ordinary_route_pricing: &'static str,
    pub limits: PoolLimits,
    pub last_refusal: Option<ResourceRefusal>,
}

/// Why a wait for the unpriced solve slot ended without admission.
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
        Self::with_wait_tick(host, UNPRICED_WAIT_TICK)
    }

    pub(crate) fn with_wait_tick(host: Arc<dyn HostMemory>, wait_tick: Duration) -> Arc<Self> {
        let mut state = PoolState::default();
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
    fn admit_bytes_locked(&self, state: &mut PoolState, bytes: u64) -> Result<(), ResourceRefusal> {
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
        // as the new request.
        let required = bytes.saturating_add(reserved);
        if available < required {
            return Err(ResourceRefusal::HeadroomInsufficient { required_bytes: required, available_bytes: available });
        }
        Ok(())
    }

    /// Admits and records `bytes` for `purpose` in one critical section.
    pub(crate) fn acquire(self: &Arc<Self>, bytes: u64, purpose: LeasePurpose) -> Result<Lease, ResourceRefusal> {
        let mut state = self.lock();
        if let Err(refusal) = self.admit_bytes_locked(&mut state, bytes) {
            state.last_refusal = Some(refusal.clone());
            return Err(refusal);
        }
        state.reserved[purpose as usize] += bytes;
        Ok(Lease { pool: Arc::clone(self), bytes, purpose })
    }

    fn release(&self, purpose: LeasePurpose, bytes: u64) {
        let mut state = self.lock();
        let slot = &mut state.reserved[purpose as usize];
        debug_assert!(*slot >= bytes, "a lease releases only what it reserved");
        *slot = slot.saturating_sub(bytes);
        self.notify(&mut state);
    }

    fn admit_unpriced_locked(&self, state: &mut PoolState) -> Result<(), ResourceRefusal> {
        if state.unpriced_solve_running {
            return Err(ResourceRefusal::SolveSlotBusy);
        }
        let sample = self.host.sample();
        state.observe(&sample);
        if sample.capacity_bytes.is_none() {
            return Err(ResourceRefusal::CapacityUnknown);
        }
        check_fresh(&sample)?;
        check_pressure(&sample)?;
        if sample.headroom_bytes.is_none() {
            return Err(ResourceRefusal::HeadroomUnknown);
        }
        state.unpriced_solve_running = true;
        Ok(())
    }

    /// Admits the one unpriced ordinary solve now, or refuses with the reason.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn try_begin_unpriced_solve(self: &Arc<Self>) -> Result<UnpricedSolveSlot, ResourceRefusal> {
        let mut state = self.lock();
        self.admit_unpriced_locked(&mut state)?;
        Ok(UnpricedSolveSlot { pool: Arc::clone(self) })
    }

    /// Waits until the unpriced solve slot is admitted or `cancelled()` holds.
    /// Each refusal while waiting is reported through `on_queued`. Neither
    /// callback runs under the pool lock (callers may take their own locks).
    pub(crate) fn wait_unpriced_solve(
        self: &Arc<Self>,
        cancelled: &dyn Fn() -> bool,
        on_queued: &mut dyn FnMut(&ResourceRefusal),
    ) -> Result<UnpricedSolveSlot, WaitOutcome> {
        loop {
            if cancelled() {
                return Err(WaitOutcome::Cancelled);
            }
            let (verdict, generation) = {
                let mut state = self.lock();
                (self.admit_unpriced_locked(&mut state), state.generation)
            };
            match verdict {
                Ok(()) => return Ok(UnpricedSolveSlot { pool: Arc::clone(self) }),
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
            },
            unpriced_solve_running: state.unpriced_solve_running,
            queued_jobs: jobs.queued,
            running_jobs: jobs.running,
            retained_results: jobs.retained,
            webview_custody_results: jobs.custody,
            headroom: HeadroomStatus {
                available_bytes: sample.headroom_bytes,
                pressure: sample.pressure,
                sample_age_ms: Some(u64::try_from(sample.age().as_millis()).unwrap_or(u64::MAX)),
            },
            ordinary_route_pricing: "unpriced",
            limits: PoolLimits {
                max_queued_jobs: MAX_QUEUED_JOBS,
                max_retained_results: MAX_RETAINED_RESULTS,
                max_webview_custody_results: MAX_WEBVIEW_CUSTODY_RESULTS,
                max_terminal_records: MAX_TERMINAL_RECORDS,
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
    pub(crate) fn unpriced_solve_running(&self) -> bool {
        self.lock().unpriced_solve_running
    }
}

/// Reserved pool bytes; released exactly once on drop (also during unwinding).
/// Not `Clone`: ownership moves input -> job -> worker -> result -> custody.
#[derive(Debug)]
pub(crate) struct Lease {
    pool: Arc<MemoryPool>,
    bytes: u64,
    purpose: LeasePurpose,
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
        if let Err(refusal) = self.pool.admit_bytes_locked(&mut state, delta) {
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
        self.pool.release(self.purpose, delta);
    }

    /// Moves the reserved bytes to another purpose without releasing them.
    pub(crate) fn set_purpose(&mut self, purpose: LeasePurpose) {
        if purpose == self.purpose {
            return;
        }
        let mut state = self.pool.lock();
        state.reserved[self.purpose as usize] -= self.bytes;
        state.reserved[purpose as usize] += self.bytes;
        self.purpose = purpose;
    }
}

impl Drop for Lease {
    fn drop(&mut self) {
        self.pool.release(self.purpose, self.bytes);
    }
}

/// The exclusive unpriced ordinary-solve slot; released on drop.
#[derive(Debug)]
pub(crate) struct UnpricedSolveSlot {
    pool: Arc<MemoryPool>,
}

impl Drop for UnpricedSolveSlot {
    fn drop(&mut self) {
        let mut state = self.pool.lock();
        state.unpriced_solve_running = false;
        self.pool.notify(&mut state);
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
        assert_eq!(unknown.try_begin_unpriced_solve().unwrap_err(), ResourceRefusal::CapacityUnknown);
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
        assert_eq!(pool.try_begin_unpriced_solve().unwrap_err(), ResourceRefusal::HeadroomUnknown);

        host.set(|s| {
            s.headroom_bytes = Some(8 * GIB);
            s.age = Duration::from_secs(5);
        });
        assert!(matches!(pool.acquire(GIB, LeasePurpose::Ingress).unwrap_err(), ResourceRefusal::HeadroomStale { age_ms } if age_ms >= 5000));
        assert!(matches!(pool.try_begin_unpriced_solve().unwrap_err(), ResourceRefusal::HeadroomStale { .. }));

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
        for level in [PressureLevel::Warning, PressureLevel::Critical, PressureLevel::Unknown] {
            host.set(|s| s.pressure = level);
            assert_eq!(pool.acquire(1, LeasePurpose::Ingress).unwrap_err(), ResourceRefusal::PressureElevated { level });
            assert_eq!(pool.try_begin_unpriced_solve().unwrap_err(), ResourceRefusal::PressureElevated { level });
        }
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
            let _slot = unwinding.try_begin_unpriced_solve().unwrap();
            panic!("solve panicked");
        }));
        assert!(panicked.is_err());
        assert_eq!(pool.reserved_total(), 0);
        assert!(!pool.unpriced_solve_running());
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
    fn unpriced_slot_is_exclusive_and_waits_for_release_or_cancel() {
        let host = ScriptedHost::normal();
        let pool = pool(&host);
        let slot = pool.try_begin_unpriced_solve().unwrap();
        assert_eq!(pool.try_begin_unpriced_solve().unwrap_err(), ResourceRefusal::SolveSlotBusy);
        assert_eq!(pool.reserved_total(), 0, "the unpriced slot reserves no bytes");

        let waiter = pool.clone();
        let handle = std::thread::spawn(move || {
            let mut reasons = Vec::new();
            let slot = waiter.wait_unpriced_solve(&|| false, &mut |r| reasons.push(r.clone()));
            (slot.is_ok(), reasons)
        });
        std::thread::sleep(Duration::from_millis(30));
        drop(slot);
        let (admitted, reasons) = handle.join().unwrap();
        assert!(admitted);
        assert!(reasons.iter().all(|r| *r == ResourceRefusal::SolveSlotBusy) && !reasons.is_empty());
        assert!(!pool.unpriced_solve_running(), "the waiter's slot dropped");

        host.set(|s| s.pressure = PressureLevel::Warning);
        let cancel = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let (waiter, flag) = (pool.clone(), cancel.clone());
        let handle = std::thread::spawn(move || {
            let mut last = None;
            let outcome = waiter.wait_unpriced_solve(&|| flag.load(std::sync::atomic::Ordering::SeqCst), &mut |r| last = Some(r.clone()));
            (outcome.map(drop), last)
        });
        std::thread::sleep(Duration::from_millis(30));
        cancel.store(true, std::sync::atomic::Ordering::SeqCst);
        pool.wake_waiters();
        let (outcome, last) = handle.join().unwrap();
        assert_eq!(outcome, Err(WaitOutcome::Cancelled));
        assert_eq!(last, Some(ResourceRefusal::PressureElevated { level: PressureLevel::Warning }));
    }

    #[test]
    fn refusals_and_status_serialize_with_stable_names() {
        let refusal = serde_json::to_value(ResourceRefusal::PressureElevated { level: PressureLevel::Warning }).unwrap();
        assert_eq!(refusal, serde_json::json!({"kind":"pressure_elevated","level":"warning"}));
        assert_eq!(serde_json::to_value(ResourceRefusal::SolveSlotBusy).unwrap(), serde_json::json!({"kind":"solve_slot_busy"}));
        assert_eq!(
            serde_json::to_value(ResourceRefusal::PoolAboveRecommendedUnacknowledged { pool_bytes: 2, recommended_bytes: 1 }).unwrap(),
            serde_json::json!({"kind":"pool_above_recommended_unacknowledged","pool_bytes":2,"recommended_bytes":1})
        );
        let host = ScriptedHost::normal();
        let status = serde_json::to_value(pool(&host).status(JobCounts { queued: 1, running: 1, retained: 0, custody: 2 })).unwrap();
        for key in [
            "capacity_bytes", "recommended_pool_bytes", "pool_bytes", "pool_above_recommended", "invocation_budget_bytes",
            "qualified_invocation_budget_bytes", "reserved_bytes", "reserved_by_purpose", "unpriced_solve_running",
            "queued_jobs", "running_jobs", "retained_results", "webview_custody_results", "headroom",
            "ordinary_route_pricing", "last_refusal",
        ] {
            assert!(status.get(key).is_some(), "{key}");
        }
        assert_eq!(status["ordinary_route_pricing"], "unpriced");
        assert_eq!(status["headroom"]["pressure"], "normal");
        assert_eq!(status["reserved_by_purpose"], serde_json::json!({"ingress":0,"result":0,"custody":0}));
        assert_eq!(status["webview_custody_results"], 2);
    }
}
