//! Conservative fixture ledger. No committed byte receives residency credit.
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Failure {
    PhysicalHeadroom, CommitHeadroom, SyntheticCommitHeadroom,
    SyntheticCommitOther, SyntheticDecommit, Address, Limit, Unknown,
    Cancelled, Stale, Deadline, OsCommit(u32), OsDecommit(u32),
}
impl Failure {
    pub fn retryable(self) -> bool {
        matches!(self, Self::PhysicalHeadroom | Self::CommitHeadroom | Self::SyntheticCommitHeadroom)
    }
}

#[derive(Debug)]
pub struct Ledger {
    pub physical: usize,
    pub commit: usize,
    pub held: [usize; 2],
    pub actual: [usize; 2],
    pub cancelled: [bool; 2],
    pub generation: [u64; 2],
}
impl Ledger {
    pub fn new(physical: usize, commit: usize) -> Self {
        Self { physical, commit, held: [0; 2], actual: [0; 2], cancelled: [false; 2], generation: [1; 2] }
    }
    /// Targets are cumulative held bytes, not deltas. Caller holds one mutex
    /// across this fresh verdict and its mutation.
    pub fn grant_to(&mut self, worker: usize, generation: u64, preferred: usize, minimum: usize) -> Result<usize, Failure> {
        if worker >= 2 { return Err(Failure::Limit); }
        if generation != self.generation[worker] { return Err(Failure::Stale); }
        if self.cancelled[worker] { return Err(Failure::Cancelled); }
        if minimum > preferred || minimum < self.held[worker] { return Err(Failure::Limit); }
        let other = self.held[1-worker];
        let minimum_total = other.checked_add(minimum).ok_or(Failure::Limit)?;
        let preferred_total = other.checked_add(preferred).ok_or(Failure::Limit)?;
        if minimum_total > self.physical { return Err(Failure::PhysicalHeadroom); }
        if minimum_total > self.commit { return Err(Failure::CommitHeadroom); }
        let chosen = if preferred_total <= self.physical && preferred_total <= self.commit { preferred } else { minimum };
        self.held[worker] = chosen;
        Ok(chosen)
    }
    pub fn record_actual(&mut self, worker: usize, actual: usize) -> Result<(), Failure> {
        if worker >= 2 || actual < self.actual[worker] || actual > self.held[worker] { return Err(Failure::Limit); }
        self.actual[worker] = actual; Ok(())
    }
    pub fn future(&self, worker: usize) -> usize { self.held[worker] - self.actual[worker] }
    pub fn cancel(&mut self, worker: usize) { self.cancelled[worker] = true; }
    /// Reset invalidates late replies; it does not release a live worker.
    pub fn reset(&mut self, worker: usize) -> Result<(), Failure> {
        self.generation[worker] = self.generation[worker].checked_add(1).ok_or(Failure::Limit)?;
        self.cancelled[worker] = true; Ok(())
    }
    /// A kill request, EOF or cancellation is not a release.
    pub fn observed_exit(&mut self, worker: usize) { self.held[worker] = 0; self.actual[worker] = 0; }
}
pub struct Window { end: Instant }
impl Window {
    pub fn new() -> Self { Self { end: Instant::now() + Duration::from_secs(2) } }
    pub fn open(&self) -> bool { Instant::now() < self.end }
    pub fn retry(&self, reason: Failure) -> bool { reason.retryable() && self.open() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    #[test]
    fn ledgers_and_cancellation() {
        let l = Mutex::new(Ledger::new(100, 100));
        let mut l = l.lock().unwrap();
        assert_eq!(l.grant_to(0, 1, 60, 50), Ok(60));
        assert_eq!(l.grant_to(1, 1, 50, 40), Ok(40));
        assert_eq!(l.grant_to(1, 1, 41, 41), Err(Failure::PhysicalHeadroom));
        l.record_actual(0, 20).unwrap(); assert_eq!(l.future(0), 40);
        assert_eq!(l.record_actual(0, 61), Err(Failure::Limit));
        l.physical = 200; l.commit = 100;
        assert_eq!(l.grant_to(1, 1, 41, 41), Err(Failure::CommitHeadroom));
        l.cancel(0); assert_eq!(l.grant_to(0, 1, 61, 61), Err(Failure::Cancelled));
        l.reset(0).unwrap(); assert_eq!(l.grant_to(0, 1, 61, 61), Err(Failure::Stale));
        assert_eq!(l.held[0], 60); assert_eq!(l.actual[0], 20);
        l.observed_exit(0); assert_eq!(l.held, [0, 40]);
        l.physical = usize::MAX; l.commit = usize::MAX;
        assert_eq!(l.grant_to(1, 1, usize::MAX, usize::MAX), Ok(usize::MAX));
        l.cancelled[0] = false;
        assert_eq!(l.grant_to(0, 2, 1, 1), Err(Failure::Limit));
    }
    #[test]
    fn retry_classifier() {
        let w = Window::new();
        for x in [Failure::PhysicalHeadroom, Failure::CommitHeadroom, Failure::SyntheticCommitHeadroom] { assert!(w.retry(x)); }
        for x in [Failure::Address, Failure::Limit, Failure::Unknown, Failure::Cancelled, Failure::Stale, Failure::Deadline, Failure::SyntheticCommitOther, Failure::OsCommit(1455), Failure::OsDecommit(8)] { assert!(!w.retry(x)); }
        std::thread::sleep(Duration::from_millis(2100));
        assert!(!w.retry(Failure::CommitHeadroom));
    }
}
