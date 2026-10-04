//! Checked custody of instrumented retained work. Invalid amounts are unavailable.
use std::{cell::Cell, fmt};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkFault {
    Overflow = 1,
    Inconsistent = 2,
    Both = 3,
}

impl fmt::Display for WorkFault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Overflow => "overflow",
            Self::Inconsistent => "inconsistent",
            Self::Both => "both",
        })
    }
}
impl std::error::Error for WorkFault {}

#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct WorkStatus(u8);
impl WorkStatus {
    pub fn is_exact(self) -> bool {
        self.0 == 0
    }
    pub fn fault(self) -> Option<WorkFault> {
        match self.0 {
            0 => None,
            1 => Some(WorkFault::Overflow),
            2 => Some(WorkFault::Inconsistent),
            _ => Some(WorkFault::Both),
        }
    }
    pub(crate) fn join(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
    pub(crate) fn from_fault(fault: WorkFault) -> Self {
        Self(fault as u8)
    }
}
impl fmt::Debug for WorkStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.fault() {
            Some(fault) => fmt::Debug::fmt(&fault, f),
            None => f.write_str("Exact"),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct WorkTotal {
    amount: u64,
    status: WorkStatus,
}
impl WorkTotal {
    pub fn status(self) -> WorkStatus {
        self.status
    }
    pub fn exact(self) -> Result<u64, WorkFault> {
        self.status.fault().map_or(Ok(self.amount), Err)
    }
    pub(crate) fn zero() -> Self {
        Self::exact_count(0)
    }
    pub(crate) fn exact_count(amount: u64) -> Self {
        Self {
            amount,
            status: WorkStatus::default(),
        }
    }
    pub(crate) fn join_status(mut self, status: WorkStatus) -> Self {
        self.status = self.status.join(status);
        self
    }
    pub(crate) fn add(self, other: Self) -> Self {
        let status = self.status.join(other.status);
        match self.amount.checked_add(other.amount) {
            Some(amount) => Self { amount, status },
            None => Self {
                amount: u64::MAX,
                status: status.join(WorkStatus::from_fault(WorkFault::Overflow)),
            },
        }
    }
    pub(crate) fn mul(self, factor: u64) -> Self {
        match self.amount.checked_mul(factor) {
            Some(amount) => Self {
                amount,
                status: self.status,
            },
            None => Self {
                amount: u64::MAX,
                status: self
                    .status
                    .join(WorkStatus::from_fault(WorkFault::Overflow)),
            },
        }
    }
    pub(crate) fn remainder(self, subset: Self) -> Self {
        let status = self.status.join(subset.status);
        match self.amount.checked_sub(subset.amount) {
            Some(amount) => Self { amount, status },
            None => Self {
                amount: 0,
                status: status.join(WorkStatus::from_fault(WorkFault::Inconsistent)),
            },
        }
    }
    pub(crate) fn room(self, limit: u64) -> Self {
        Self::exact_count(limit.saturating_sub(self.amount)).join_status(self.status)
    }
    pub(crate) fn legacy_saturated(self) -> u64 {
        self.exact().unwrap_or(u64::MAX)
    }
}
impl fmt::Debug for WorkTotal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.exact() {
            Ok(amount) => fmt::Debug::fmt(&amount, f),
            Err(fault) => f
                .debug_struct("UnavailableWork")
                .field("fault", &fault)
                .finish(),
        }
    }
}
impl std::ops::Add for WorkTotal {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        self.add(rhs)
    }
}
impl std::ops::Sub for WorkTotal {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self.remainder(rhs)
    }
}

pub(crate) struct WorkStream {
    anchor: Cell<u8>,
}
#[derive(Clone, Copy)]
pub(crate) struct WorkSnapshot<'s> {
    stream: &'s WorkStream,
    total: WorkTotal,
}
impl WorkStream {
    pub(crate) fn new() -> Self {
        Self {
            anchor: Cell::new(0),
        }
    }
    pub(crate) fn snapshot(&self, total: WorkTotal) -> Result<WorkSnapshot<'_>, WorkFault> {
        total.exact()?;
        let _ = self.anchor.get();
        Ok(WorkSnapshot {
            stream: self,
            total,
        })
    }
}
impl WorkSnapshot<'_> {
    pub(crate) fn delta_since(self, before: Self) -> WorkTotal {
        let delta = self.total.remainder(before.total);
        if std::ptr::eq(self.stream, before.stream) {
            delta
        } else {
            delta.join_status(WorkStatus::from_fault(WorkFault::Inconsistent))
        }
    }
    pub(crate) fn total(self) -> WorkTotal {
        self.total
    }
}

#[cfg(test)]
#[path = "../../../tests/retained_k4/work_tests.rs"]
mod tests;
