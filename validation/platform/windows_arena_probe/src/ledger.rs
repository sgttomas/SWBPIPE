//! Deliberately conservative fixture ledger: no residency credit.
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Failure { Headroom, Address, Limit, Unknown, Cancelled, Commit(u32), Decommit(u32) }

#[derive(Debug)]
pub struct Ledger {
    pub physical: usize,
    pub commit: usize,
    pub held: [usize; 2],
    pub actual: [usize; 2],
    pub cancelled: [bool; 2],
}
impl Ledger {
    pub fn new(physical: usize, commit: usize) -> Self {
        Self { physical, commit, held: [0; 2], actual: [0; 2], cancelled: [false; 2] }
    }
    /// All verdict and reservation mutations happen under the caller's one mutex.
    pub fn grant(&mut self, worker: usize, preferred: usize, minimum: usize) -> Result<usize, Failure> {
        if self.cancelled[worker] { return Err(Failure::Cancelled); }
        if minimum > preferred { return Err(Failure::Limit); }
        let outstanding = self.held.iter().try_fold(0usize, |a, &b| a.checked_add(b)).ok_or(Failure::Limit)?;
        let available = self.physical.min(self.commit).saturating_sub(outstanding);
        let amount = if preferred <= available { preferred } else if minimum <= available { minimum } else { return Err(Failure::Headroom) };
        self.held[worker] = self.held[worker].checked_add(amount).ok_or(Failure::Limit)?;
        Ok(amount)
    }
    pub fn committed(&mut self, worker: usize, amount: usize) -> Result<(), Failure> {
        if self.cancelled[worker] { return Err(Failure::Cancelled); }
        let actual = self.actual[worker].checked_add(amount).ok_or(Failure::Limit)?;
        if actual > self.held[worker] { return Err(Failure::Limit); }
        self.actual[worker] = actual; Ok(())
    }
    /// A kill request, pipe EOF, or cancellation is not a release.
    pub fn observed_exit(&mut self, worker: usize) { self.held[worker] = 0; self.actual[worker] = 0; }
}
pub struct Window { end: Instant }
impl Window {
    pub fn new() -> Self { Self { end: Instant::now() + Duration::from_secs(2) } }
    pub fn retry(&self, reason: Failure) -> bool { reason == Failure::Headroom && Instant::now() < self.end }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ledgers_and_cancellation() {
        let mut l = Ledger::new(100, 70);
        assert_eq!(l.grant(0, 60, 50), Ok(60));
        assert_eq!(l.grant(1, 20, 11), Err(Failure::Headroom));
        assert_eq!(l.grant(1, 20, 10), Ok(10));
        assert_eq!(l.committed(0, 61), Err(Failure::Limit));
        l.cancelled[0] = true;
        assert_eq!(l.committed(0, 1), Err(Failure::Cancelled));
        assert_eq!(l.held[0], 60);
        l.observed_exit(0);
        assert_eq!(l.held, [0, 10]);
        l.physical = 12; l.commit = 100;
        assert_eq!(l.grant(0, 3, 3), Err(Failure::Cancelled));
        l.cancelled[0] = false;
        assert_eq!(l.grant(0, 3, 3), Err(Failure::Headroom));
        assert_eq!(l.grant(0, 2, 2), Ok(2));
    }
    #[test]
    fn retry_classifier() {
        let w = Window::new();
        assert!(w.retry(Failure::Headroom));
        for x in [Failure::Address, Failure::Limit, Failure::Unknown, Failure::Cancelled, Failure::Commit(8), Failure::Decommit(8)] { assert!(!w.retry(x)); }
        std::thread::sleep(Duration::from_millis(2100));
        assert!(!w.retry(Failure::Headroom));
    }
}
