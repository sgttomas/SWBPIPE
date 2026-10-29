//! A bounded protocol model, not the desktop worker or a solver.
//! Times and headroom are injected; Windows process custody is a separate concern.

use std::time::Duration;

pub const WINDOW: Duration = Duration::from_secs(2);
pub const TICK: Duration = Duration::from_millis(50);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Running,
    Waiting,
    Parked,
    Cancelled,
    Refused,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Admission {
    Preferred,
    Minimal,
    Retry,
    Expired,
    MissingQuiescence,
    Cancelled,
}

/// One supervisor and one cooperating fixture mutator. `reserved` includes the
/// fixture's entire bounded transition allowance before a stop is requested.
/// Calling `request` and `ack` never waits for the mutator.
pub struct Supervisor {
    pub state: State,
    pub generation: u64,
    pub reserved: u64,
    deadline: Duration,
}
impl Supervisor {
    pub fn new(reserved: u64) -> Self {
        Self {
            state: State::Running,
            generation: 0,
            reserved,
            deadline: Duration::ZERO,
        }
    }
    pub fn request(&mut self, now: Duration) -> u64 {
        assert_eq!(self.state, State::Running);
        self.generation = self.generation.checked_add(1).unwrap();
        self.deadline = now.checked_add(WINDOW).unwrap();
        self.state = State::Waiting;
        self.generation
    }
    /// An event is only a request. Only the current generation's cooperative
    /// ack, produced at a safe point, can establish this fixture's parked state.
    pub fn ack(&mut self, generation: u64) -> bool {
        if self.state != State::Waiting || self.generation != generation {
            return false;
        }
        self.state = State::Parked;
        true
    }
    pub fn cancel(&mut self) {
        self.state = State::Cancelled;
    }
    /// A fresh scripted sample is supplied for each call under the caller's
    /// pool lock. The deadline is fixed at request, never restarted by ack.
    pub fn attempt(
        &mut self,
        now: Duration,
        headroom: u64,
        preferred: u64,
        minimal: u64,
    ) -> Admission {
        if self.state == State::Cancelled {
            return Admission::Cancelled;
        }
        assert!(matches!(self.state, State::Waiting | State::Parked));
        if now >= self.deadline {
            let outcome = if self.state == State::Parked {
                Admission::Expired
            } else {
                Admission::MissingQuiescence
            };
            self.state = State::Refused;
            return outcome;
        }
        if self.state != State::Parked {
            return Admission::Retry;
        }
        assert!(preferred >= minimal);
        let (outcome, added) = if headroom >= preferred {
            (Admission::Preferred, preferred)
        } else if headroom >= minimal {
            (Admission::Minimal, minimal)
        } else {
            return Admission::Retry;
        };
        self.reserved = self.reserved.checked_add(added).unwrap();
        self.state = State::Running;
        outcome
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn ms(n: u64) -> Duration {
        Duration::from_millis(n)
    }
    #[test]
    fn current_ack_preferred_and_stale_ack() {
        let mut s = Supervisor::new(4);
        let g = s.request(ms(0));
        assert!(!s.ack(g + 1));
        assert_eq!(s.attempt(ms(1), 100, 8, 3), Admission::Retry);
        assert!(s.ack(g));
        assert_eq!(s.attempt(ms(2), 8, 8, 3), Admission::Preferred);
        assert_eq!(s.reserved, 12);
        let h = s.request(ms(3));
        assert!(!s.ack(g));
        assert!(s.ack(h));
        assert_eq!(s.attempt(ms(4), 3, 8, 3), Admission::Minimal);
    }
    #[test]
    fn transient_samples_and_release_wakeup_resample() {
        let mut s = Supervisor::new(4);
        let g = s.request(ms(0));
        assert!(s.ack(g));
        assert_eq!(s.attempt(ms(0), 0, 8, 3), Admission::Retry);
        assert_eq!(s.attempt(TICK, 2, 8, 3), Admission::Retry);
        // A release wakes the supervisor before the next tick; it samples anew.
        assert_eq!(s.attempt(ms(60), 3, 8, 3), Admission::Minimal);
    }
    #[test]
    fn no_ack_or_late_ack_cannot_extend_the_absolute_window() {
        let mut s = Supervisor::new(4);
        let g = s.request(ms(0));
        assert_eq!(s.attempt(ms(1999), 0, 8, 3), Admission::Retry);
        assert!(s.ack(g));
        assert_eq!(s.attempt(ms(2000), 100, 8, 3), Admission::Expired);
        assert!(!s.ack(g));
        let mut no_ack = Supervisor::new(4);
        no_ack.request(ms(0));
        assert_eq!(
            no_ack.attempt(ms(2000), 100, 8, 3),
            Admission::MissingQuiescence
        );
    }
    #[test]
    fn cancellation_dominates_late_ack_and_resume() {
        for parked in [false, true] {
            let mut s = Supervisor::new(4);
            let g = s.request(ms(0));
            if parked {
                assert!(s.ack(g));
            }
            s.cancel();
            assert!(!s.ack(g));
            assert_eq!(s.attempt(ms(1), u64::MAX, 8, 3), Admission::Cancelled);
            assert_eq!(s.reserved, 4);
        }
    }
}

#[cfg(all(test, windows, target_arch = "x86_64"))]
mod windows_accounting;

#[cfg(all(test, windows, target_arch = "x86_64"))]
mod native_fixture;
