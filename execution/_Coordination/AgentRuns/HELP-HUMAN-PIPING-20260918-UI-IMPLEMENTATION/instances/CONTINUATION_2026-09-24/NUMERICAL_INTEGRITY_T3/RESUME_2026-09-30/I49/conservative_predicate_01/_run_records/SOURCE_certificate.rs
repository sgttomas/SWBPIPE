//! Two bounded, private certificate operations. No product caller or work tariff.
#![allow(dead_code)] // Isolated arithmetic slice; integration is separately gated.

use super::super::adaptive::{next_up, AttemptStop};
use super::super::wide::multi::{AttemptWork, WideContext};
use super::super::wide::Wide;
use super::super::wide_sum::{ExactWideSum, SumWork};
use super::super::work::{WorkFault, WorkStatus};
use super::{step, Toward};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum HelperError {
    Arithmetic(AttemptStop),
    InvalidSmallBoundInput,
    Binary64Range,
    Invariant,
}

/// The original numerical result and all spent work, including failed prefixes.
/// Fields are private: only `result` can expose a success, after status checking.
#[derive(Debug)]
pub(crate) struct EntrySpent<T> {
    result: Result<T, HelperError>,
    work: AttemptWork,
    sum_work: SumWork,
    comparisons: u8,
    f64_operations: u8,
}

impl<T> EntrySpent<T> {
    fn new(result: Result<T, HelperError>) -> Self {
        Self {
            result,
            work: AttemptWork::default(),
            sum_work: SumWork::default(),
            comparisons: 0,
            f64_operations: 0,
        }
    }

    pub(crate) fn status(&self) -> WorkStatus {
        let status = self
            .work
            .checked_lme()
            .add(self.sum_work.checked_lme())
            .status();
        match &self.result {
            Err(HelperError::Arithmetic(AttemptStop::WorkAccounting(fault))) => {
                status.join(WorkStatus::from_fault(*fault))
            }
            Err(HelperError::Invariant) => {
                status.join(WorkStatus::from_fault(WorkFault::Inconsistent))
            }
            _ => status,
        }
    }

    /// A numerical error and non-exact work remain independently observable.
    pub(crate) fn result(&self) -> Result<&T, HelperError> {
        match &self.result {
            Err(error) => Err(error.clone()),
            Ok(value) => match self.status().fault() {
                Some(fault) => Err(HelperError::Arithmetic(AttemptStop::WorkAccounting(fault))),
                None => Ok(value),
            },
        }
    }

    pub(crate) fn work(&self) -> AttemptWork {
        self.work
    }
    pub(crate) fn sum_work(&self) -> SumWork {
        self.sum_work
    }
    pub(crate) fn comparisons(&self) -> u8 {
        self.comparisons
    }

    /// Rounded binary64 arithmetic only: two divisions and two scale-back
    /// multiplications. Bitwise abs/next_up, predicates and integer search are
    /// not charged as floating-point arithmetic or assigned a facade tariff.
    pub(crate) fn f64_operations(&self) -> u8 {
        self.f64_operations
    }
}

/// Tight p1024 sqrt endpoint, using one nearest sqrt and the exact q*q-a side.
pub(crate) fn sqrt_endpoint(a: &Wide<16>, toward: Toward) -> EntrySpent<Wide<16>> {
    match WideContext::<16>::new(1024) {
        Ok(ctx) => sqrt_owned(a, toward, ctx, ExactWideSum::new()),
        Err(error) => EntrySpent::new(Err(HelperError::Arithmetic(error.into()))),
    }
}

// Ownership keeps collection outside the fallible numeric closure. Tests can
// seed these same local owners to witness side/step failures without a runtime
// injection mechanism, a global flag, or a Result-only alternate implementation.
fn sqrt_owned(
    a: &Wide<16>,
    toward: Toward,
    mut ctx: WideContext<16>,
    mut sum: ExactWideSum,
) -> EntrySpent<Wide<16>> {
    let result = (|| -> Result<Wide<16>, AttemptStop> {
        let q = ctx.sqrt(a)?;
        if q.is_zero() {
            return Ok(Wide::<16>::ZERO);
        }
        sum.add_product(&mut ctx, &q, &q, false)?;
        sum.add_wide(a, true)?;
        let side = sum.signum()?;
        sum.clear();
        let wrong = match toward {
            Toward::Up => side < 0,
            Toward::Down => side > 0,
        };
        if wrong {
            step(&mut ctx, &mut sum, &q, toward)
        } else {
            Ok(q)
        }
    })()
    .map_err(HelperError::Arithmetic);
    let mut spent = EntrySpent::new(result);
    spent.work.record(&ctx);
    spent.sum_work.merge(&sum.work());
    spent
}

/// Exact sign of b0+r+2^-1074-candidate. The caller owns collection on failure.
fn bound_comparison(
    sum: &mut ExactWideSum,
    b0: f64,
    r: f64,
    candidate: f64,
) -> Result<i8, AttemptStop> {
    sum.add_binary64(b0, false)?;
    sum.add_binary64(r, false)?;
    sum.add_binary64(f64::from_bits(1), false)?;
    sum.add_binary64(candidate, true)?;
    Ok(sum.signum()?)
}

impl EntrySpent<f64> {
    fn compare(&mut self, b0: f64, r: f64, bits: u64) -> Result<i8, HelperError> {
        self.compare_owned(b0, r, bits, ExactWideSum::new())
    }

    fn compare_owned(
        &mut self,
        b0: f64,
        r: f64,
        bits: u64,
        mut sum: ExactWideSum,
    ) -> Result<i8, HelperError> {
        if self.comparisons >= 64 {
            return Err(HelperError::Invariant);
        }
        self.comparisons += 1;
        let result = bound_comparison(&mut sum, b0, r, f64::from_bits(bits));
        self.sum_work.merge(&sum.work());
        result.map_err(HelperError::Arithmetic)
    }
}

/// The existing small-scale row expression rounded upward once to binary64.
/// This entry admits only finite value and finite 0 < scale < 2^-988.
pub(crate) fn small_row_bound(value: f64, scale: f64) -> EntrySpent<f64> {
    let mut spent = EntrySpent::new(Err(HelperError::InvalidSmallBoundInput));
    if !value.is_finite()
        || !scale.is_finite()
        || scale <= 0.0
        || scale >= f64::from_bits(0x0230_0000_0000_0000)
    {
        return spent;
    }

    // Exactly the separately rounded operations of absolute_bound/row_bound.
    // Keep each RN64 operation separate and preserve the actual scale-back
    // predicate; these intermediates are part of the existing bit contract.
    let two64 = 18_446_744_073_709_551_616.0_f64;
    let b_nearest = scale / two64;
    let b_back = b_nearest * two64;
    let b0 = if b_back < scale {
        next_up(b_nearest)
    } else {
        b_nearest
    };
    let q = value.abs();
    let two53 = 9_007_199_254_740_992.0_f64;
    let r_nearest = q / two53;
    let r_back = r_nearest * two53;
    let r = if r_back < q {
        next_up(r_nearest)
    } else {
        r_nearest
    };
    spent.f64_operations = 4;

    spent.result = (|| {
        const MAX: u64 = 0x7fef_ffff_ffff_ffff;
        if spent.compare(b0, r, MAX)? > 0 {
            return Err(HelperError::Binary64Range);
        }
        // h>0 proves zero is strictly below the sum; MAX reaches it. Positive
        // finite encodings are monotone and MAX < 2^63. Maintain lo < sum <= hi.
        let (mut lo, mut hi) = (0u64, MAX);
        for _ in 0..63 {
            if hi - lo == 1 {
                return Ok(f64::from_bits(hi));
            }
            let mid = lo + (hi - lo) / 2;
            if spent.compare(b0, r, mid)? > 0 {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        if hi - lo == 1 {
            Ok(f64::from_bits(hi))
        } else {
            Err(HelperError::Invariant)
        }
    })();
    spent
}

#[cfg(test)]
#[path = "../../../../tests/retained_k4/certificate_arithmetic_tests.rs"]
mod tests;
