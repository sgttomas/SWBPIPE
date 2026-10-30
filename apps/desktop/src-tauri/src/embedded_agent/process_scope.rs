//! Observe only the original retained supplier. No ancestry snapshots, recursive
//! tracking or signal authority. The inherited no-fork policy covers startup
//! before this observer is armed; any observed fork permanently defeats proof.
use std::{
    os::fd::{AsRawFd, FromRawFd, OwnedFd},
    process::Child,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Invalid {
    Observation,
    Fork,
    Identity,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Status {
    pub exit_observed: bool,
    pub exit_status_raw: Option<i32>,
    pub invalid: Option<Invalid>,
}
impl Status {
    pub fn certified_exit(self) -> bool {
        self.exit_observed && self.invalid.is_none()
    }
    fn observe(&mut self, expected: usize, event: &libc::kevent) {
        if event.ident != expected {
            self.invalid.get_or_insert(Invalid::Identity);
        }
        if event.flags & libc::EV_ERROR != 0 {
            self.invalid.get_or_insert(Invalid::Observation);
        }
        if event.fflags & libc::NOTE_FORK != 0 {
            self.invalid.get_or_insert(Invalid::Fork);
        }
        if event.fflags & libc::NOTE_EXIT != 0 {
            self.exit_observed = true;
            self.exit_status_raw = Some(event.data as i32);
        }
    }
}
pub(super) struct Observer {
    fd: Option<OwnedFd>,
    original: usize,
    status: Status,
}
impl Observer {
    pub fn new(original: &Child) -> Self {
        let mut this = Self {
            fd: None,
            original: original.id() as usize,
            status: Status {
                exit_observed: false,
                exit_status_raw: None,
                invalid: None,
            },
        };
        // SAFETY: SDK public kqueue/kevent ABI; owned fd, stack event/timeouts.
        // The caller retains this actual unreaped Child through final group ops.
        let raw = unsafe { libc::kqueue() };
        if raw < 0 {
            this.status.invalid = Some(Invalid::Observation);
            return this;
        }
        let fd = unsafe { OwnedFd::from_raw_fd(raw) };
        if unsafe { libc::fcntl(raw, libc::F_SETFD, libc::FD_CLOEXEC) } < 0 {
            this.status.invalid = Some(Invalid::Observation);
            return this;
        }
        let event = libc::kevent {
            ident: this.original,
            filter: libc::EVFILT_PROC,
            flags: libc::EV_ADD | libc::EV_ENABLE | libc::EV_CLEAR,
            fflags: libc::NOTE_EXIT | libc::NOTE_EXITSTATUS | libc::NOTE_FORK,
            data: 0,
            udata: std::ptr::null_mut(),
        };
        if unsafe { libc::kevent(raw, &event, 1, std::ptr::null_mut(), 0, std::ptr::null()) } < 0 {
            this.status.invalid = Some(Invalid::Observation);
            return this;
        }
        this.fd = Some(fd);
        this.poll();
        this
    }
    pub fn poll(&mut self) -> Status {
        if let Some(fd) = &self.fd {
            let timeout = libc::timespec {
                tv_sec: 0,
                tv_nsec: 0,
            };
            for _ in 0..16 {
                let mut event = libc::kevent {
                    ident: 0,
                    filter: 0,
                    flags: 0,
                    fflags: 0,
                    data: 0,
                    udata: std::ptr::null_mut(),
                };
                let count = unsafe {
                    libc::kevent(fd.as_raw_fd(), std::ptr::null(), 0, &mut event, 1, &timeout)
                };
                if count == 0 {
                    break;
                }
                if count < 0 {
                    self.status.invalid.get_or_insert(Invalid::Observation);
                    break;
                }
                self.status.observe(self.original, &event);
            }
        }
        self.status
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn event(flags: u32) -> libc::kevent {
        libc::kevent {
            ident: 42,
            filter: libc::EVFILT_PROC,
            flags: 0,
            fflags: flags,
            data: 0,
            udata: std::ptr::null_mut(),
        }
    }
    #[test]
    fn fork_and_observation_failure_can_never_be_cleared_by_later_exit() {
        for reason in [Invalid::Fork, Invalid::Observation, Invalid::Identity] {
            let mut status = Status {
                exit_observed: false,
                exit_status_raw: None,
                invalid: Some(reason),
            };
            status.observe(42, &event(libc::NOTE_EXIT));
            assert!(!status.certified_exit());
            assert_eq!(status.invalid, Some(reason));
        }
    }
    #[test]
    fn native_fork_flag_invalidates_even_an_exit_in_the_same_event() {
        let mut status = Status {
            exit_observed: false,
            exit_status_raw: None,
            invalid: None,
        };
        status.observe(42, &event(libc::NOTE_FORK | libc::NOTE_EXIT));
        assert_eq!(status.invalid, Some(Invalid::Fork));
        assert!(!status.certified_exit());
    }
    #[test]
    fn original_exit_is_distinct_from_liveness_and_queue_observation() {
        let mut status = Status {
            exit_observed: false,
            exit_status_raw: None,
            invalid: None,
        };
        assert!(!status.certified_exit());
        status.observe(42, &event(libc::NOTE_EXIT));
        assert!(status.certified_exit());
        assert_eq!(status.exit_status_raw, Some(0));
    }
}
