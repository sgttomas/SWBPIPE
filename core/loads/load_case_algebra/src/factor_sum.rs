//! The unit factor-sum rule shared by the combination gates.
//!
//! A mechanics combination superposes its case states with factors `f_i`. Where
//! a case state carries a fixed load (a consuming constant-effort support, an
//! installed reference), superposition counts that load `Σf` times, so the
//! combined state is a state of the original system only when `Σf = 1`. The
//! pressure-free preview's `CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE` gate and
//! any other gate that needs `Σf = 1` use [`factor_sum_within_unit_tolerance`].

use open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator;
use std::error::Error;
use std::fmt;

/// The absolute admission bound on `|Σf - 1|`: `64 * f64::EPSILON`, exactly
/// `2^-46` (about `1.42e-14`).
pub const UNIT_FACTOR_SUM_BOUND: f64 = 64.0 * f64::EPSILON;

/// Why the factor sum was not evaluated. Either one refuses the combination.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FactorSumError {
    /// The factor at `index` (in the order supplied) is NaN or infinite.
    NonFiniteFactor { index: usize },
    /// The exact accumulator's carry capacity was exceeded. Its 64 carry bits
    /// beyond the largest binary64 product make this unreachable below 2^64
    /// factors; it is named rather than assumed away.
    AccumulatorOverflow,
}

impl fmt::Display for FactorSumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteFactor { index } => {
                write!(f, "combination factor {index} is not finite")
            }
            Self::AccumulatorOverflow => write!(f, "exact factor-sum accumulator overflow"),
        }
    }
}

impl Error for FactorSumError {}

fn accumulate(accumulator: &mut ExactAccumulator, value: f64) -> Result<(), FactorSumError> {
    // `value` is finite here (checked first, so a non-finite factor is named with its
    // index), and `add` never rounds: it can fail only by exhausting the carry bits.
    accumulator.add(value).map_err(|_| FactorSumError::AccumulatorOverflow)
}

/// Whether the binary64 factors sum to 1 within [`UNIT_FACTOR_SUM_BOUND`].
///
/// **Rule (option C).** `Σf` is the exact dyadic sum of the supplied binary64
/// values, held in `ExactAccumulator` (no rounding, no overflow). The result is
/// `Ok(true)` iff `|Σf - 1| <= 64ε` (ε = `f64::EPSILON`, so the bound is exactly
/// `2^-46`), compared exactly: `Σf - 1 - 64ε <= 0` and `Σf - 1 + 64ε >= 0` are
/// decided by the exact accumulator's sign, so no rounding enters the decision.
/// There is no `Σ|f|` scaling. An empty list sums to 0 and is not admitted. A
/// non-finite factor is `Err(NonFiniteFactor)`; callers refuse it by name.
///
/// **Why C.** The rule this replaces admitted `|fl(Σf) - 1| <= 64ε·max(1, fl(Σ|f|))`.
/// Its hole was the relative scaling, not the tolerance: `2^53` and `-2^53` sum
/// exactly to 0 but scaled the bound to 256, and an accumulation overflowing to
/// infinity compared `inf > inf` as false; both were admitted with the fixed
/// loads scaled by `Σf ≠ 1`. With an exact sum and an absolute bound:
/// - `2^53, -2^53` gives `|0 - 1| = 1` and is refused; exact accumulation cannot
///   overflow;
/// - `0.3 + 0.7` and `1.2 - 0.2` are admitted, as before: each sums exactly to
///   `1 - 2^-54` (about `1 - 5.55e-17`); computed factors such as
///   `fl(1/3) + fl(2/3)` (also `1 - 2^-54`) are admitted too;
/// - no decimal formatting is needed, so no Rust/TS formatting-parity risk;
/// - what it admits scales the fixed loads by at most `1 ± 1.4e-14`, below
///   solve precision.
///
/// Rejected: (A) exact `Σf == 1`, which withholds `0.3 + 0.7` and `1.2 - 0.2`;
/// (B) an exact rational sum of each factor's shortest round-trip decimal `== 1`,
/// which refuses `fl(1/3) + fl(2/3)` (its decimals sum to `0.9999999999999999`)
/// and needs a formatting step.
pub fn factor_sum_within_unit_tolerance(
    factors: impl IntoIterator<Item = f64>,
) -> Result<bool, FactorSumError> {
    let mut sum = ExactAccumulator::new();
    for (index, factor) in factors.into_iter().enumerate() {
        if !factor.is_finite() {
            return Err(FactorSumError::NonFiniteFactor { index });
        }
        accumulate(&mut sum, factor)?;
    }
    accumulate(&mut sum, -1.0)?;
    let mut above = sum.clone();
    accumulate(&mut above, -UNIT_FACTOR_SUM_BOUND)?;
    let mut below = sum;
    accumulate(&mut below, UNIT_FACTOR_SUM_BOUND)?;
    Ok(above.signum() <= 0 && below.signum() >= 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The rule this replaced (main `b7cbcf411d`, `preview_physics.rs` `gate_reason`):
    /// `true` where it admitted.
    fn replaced_rule_admits(factors: &[f64]) -> bool {
        let sum: f64 = factors.iter().sum();
        let abs_sum: f64 = factors.iter().map(|f| f.abs()).sum();
        !((sum - 1.0).abs() > 64.0 * f64::EPSILON * abs_sum.max(1.0))
    }
    fn admits(factors: &[f64]) -> bool {
        factor_sum_within_unit_tolerance(factors.iter().copied()).expect("finite factors")
    }
    /// The exact `Σf - target`, rounded once (exact whenever it is a binary64 value).
    fn exact_offset(factors: &[f64], target: &[f64]) -> f64 {
        let mut sum = ExactAccumulator::new();
        factors.iter().for_each(|f| sum.add(*f).unwrap());
        target.iter().for_each(|t| sum.add(-*t).unwrap());
        sum.round().unwrap()
    }
    const P53: f64 = 9_007_199_254_740_992.0;
    const TINY: f64 = 4.9406564584124654e-324; // 2^-1074, the quantum of a binary64 sum

    #[test]
    fn bound_is_exactly_two_to_minus_46() {
        assert_eq!(UNIT_FACTOR_SUM_BOUND, 2f64.powi(-46));
        assert_eq!(TINY, f64::from_bits(1));
        assert_eq!(P53, 2f64.powi(53));
    }

    #[test]
    fn replaced_rule_holes_are_refused() {
        // 2^53 - 2^53 = 0 exactly; the old bound scaled to 64ε·2^54 = 256.
        let cancel = [P53, -P53];
        assert_eq!(exact_offset(&cancel, &[]), 0.0);
        assert!(replaced_rule_admits(&cancel), "before: admitted");
        assert!(!admits(&cancel), "after: refused");
        // fl(Σf) overflows to inf and `inf > inf` is false.
        let overflow = [f64::MAX, f64::MAX];
        assert_eq!(overflow.iter().sum::<f64>(), f64::INFINITY);
        assert!(replaced_rule_admits(&overflow), "before: admitted");
        assert!(!admits(&overflow), "after: refused");
        // A NaN fold compares false, so the old rule admitted it.
        assert!(replaced_rule_admits(&[f64::NAN, 1.0]), "before: admitted");
        assert!(replaced_rule_admits(&[f64::INFINITY, -f64::INFINITY]), "before: admitted");
    }

    #[test]
    fn non_finite_factors_are_refused_by_name() {
        for (factors, index) in [
            (vec![f64::NAN, 1.0], 0),
            (vec![0.5, f64::INFINITY], 1),
            (vec![1.0, 0.0, f64::NEG_INFINITY], 2),
        ] {
            assert_eq!(
                factor_sum_within_unit_tolerance(factors),
                Err(FactorSumError::NonFiniteFactor { index })
            );
        }
    }

    #[test]
    fn sums_off_by_more_than_the_bound_are_refused() {
        let off = [0.3, 0.7, 1e-12];
        assert!(exact_offset(&off, &[1.0]) > 9.99e-13);
        assert!(!replaced_rule_admits(&off), "before: refused too");
        assert!(!admits(&off), "after: refused");
        for factors in [&[1.0, 0.5][..], &[0.5][..], &[][..], &[2.0, -0.5][..]] {
            assert!(!admits(factors), "{factors:?}");
        }
    }

    #[test]
    fn unit_sums_and_their_binary64_spellings_are_admitted() {
        let one_minus_2_54 = -2f64.powi(-54);
        for (factors, offset) in [
            (&[1.0][..], 0.0),
            (&[0.5, 0.5][..], 0.0),
            (&[0.25, 0.25, 0.5][..], 0.0),
            (&[2.0, -1.0][..], 0.0),
            (&[0.3, 0.7][..], one_minus_2_54),
            (&[1.2, -0.2][..], one_minus_2_54),
            (&[1.0 / 3.0, 2.0 / 3.0][..], one_minus_2_54),
        ] {
            assert_eq!(exact_offset(factors, &[1.0]), offset, "{factors:?}");
            assert!(replaced_rule_admits(factors), "before: admitted {factors:?}");
            assert!(admits(factors), "after: admitted {factors:?}");
        }
    }

    #[test]
    fn the_bound_is_closed_and_the_next_dyadic_is_refused() {
        let bound = UNIT_FACTOR_SUM_BOUND;
        // Σf = 1 + 2^-46 and 1 - 2^-46 exactly: admitted (closed bound).
        let upper = [1.0, bound];
        let lower = [1.0, -bound];
        assert_eq!(exact_offset(&upper, &[1.0, bound]), 0.0);
        assert_eq!(exact_offset(&lower, &[1.0, -bound]), 0.0);
        assert!(admits(&upper) && admits(&lower));
        // One binary64 factor on each bound (1 + 2^-46 and 1 - 2^-46 are binary64 values).
        assert_eq!((1.0 + bound) - 1.0, bound);
        assert_eq!(1.0 - (1.0 - bound), bound);
        assert!(admits(&[1.0 + bound]) && admits(&[1.0 - bound]));
        // The next dyadic beyond each: Σf = 1 ± (2^-46 + 2^-1074), the finest step a
        // sum of binary64 factors can take. Refused.
        let above = [1.0, bound, TINY];
        let below = [1.0, -bound, -TINY];
        assert_eq!(exact_offset(&above, &[1.0, bound]), TINY);
        assert_eq!(exact_offset(&below, &[1.0, -bound]), -TINY);
        assert!(!admits(&above) && !admits(&below));
        // The replaced rule admitted both: fl(Σf) drops the 2^-1074 and its bound
        // grew with Σ|f| > 1.
        assert!(replaced_rule_admits(&above) && replaced_rule_admits(&below), "before: admitted");
        // The next binary64 factor beyond each single-factor bound: refused.
        let next_up = f64::from_bits((1.0 + bound).to_bits() + 1);
        let next_down = f64::from_bits((1.0 - bound).to_bits() - 1);
        assert_eq!(exact_offset(&[next_up], &[1.0, bound]), f64::EPSILON);
        assert_eq!(exact_offset(&[next_down], &[1.0, -bound]), -f64::EPSILON / 2.0);
        assert!(!admits(&[next_up]) && !admits(&[next_down]));
    }
}
