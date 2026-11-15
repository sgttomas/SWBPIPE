//! The combination rules of CLM-001, on values already reduced to one per combination unit.
//!
//! A unit is a periodic cluster: its signed value is the sum of its modes' contributions, and
//! its frequency is the cluster's lowest (CLM-001, Repeated modes). Each rule is applied to one
//! row in one direction. The arithmetic is binary64 in a fixed order, so a repeated run gives the
//! same bits:
//! - SRSS: Σ R_i² in ascending cluster order, then the square root;
//! - CQC: for each i in ascending order, R_i² and then 2·ρ_ij·R_i·R_j for each j > i, with
//!   ρ_ii = 1 and ρ_ij from [`correlation_coefficient`]; a sum below zero is refused, never
//!   repaired;
//! - grouping and ten percent: Σ R_i², then 2·|R_i·R_j| over the coupled pairs in ascending
//!   order;
//! - absolute sum: Σ |R_i|.
//!
//! **Dynamic range.** Every rule with a square root (the modal SRSS, CQC, grouping and ten
//! percent rules, the rigid SRSS and the directional SRSS) is evaluated on its values divided by
//! 2^E, where 2^E ≤ max|R_i| < 2^(E+1), and its root is multiplied back by 2^E. Both scalings are
//! by a power of two: the largest value becomes exact in [1, 2), no square overflows, and the
//! one multiplication back rounds once. So a finite result is never refused because a square
//! overflowed, and values below 1e-154 are not lost because their squares underflowed. Only a
//! value below 2^-1022 times the largest one can lose bits in the division, and its square is
//! below 2^-2044 of the largest square, under the rounding of the sum. A result beyond the
//! binary64 range is refused (`SPECTRUM_COMBINATION_NONFINITE`). The absolute sums and 100-40-40
//! add non-negative terms, so their partial sums never exceed the result.
//!
//! **Cancellation.** Every sum of signed terms (the CQC double sum here, and the cluster sums
//! before it) is ordinary binary64 addition: its rounding error is relative to the size of its
//! terms, not of its result. A CQC result much smaller than the largest unit value carries an
//! absolute error of order √(n²·u)·max|R_i| (u = 2⁻⁵³), and a CQC sum that rounds below zero is
//! refused. Nothing is clamped, floored or fitted.
//!
//! Square roots and the four basic operations are correctly rounded; no library function whose
//! rounding varies by platform is used.
use crate::exact::decompose;
use std::cmp::Ordering;

/// The 40 % factor of 100-40-40 as represented: the binary64 nearest to the exact rule value 2/5,
/// above it by 2⁻⁵⁴ of 2/5 (the DEL-04-10 scope, "The 100-40-40 factor is exactly 2/5").
pub const FORTY_PERCENT: f64 = 0.4;

/// Whether `upper` is within 10 % of `lower`, (upper − lower)/lower ≤ 1/10, decided exactly on
/// the represented binary64 operands: 10·upper ≤ 11·lower as an integer comparison, so rounding
/// never decides the inclusive boundary (the grouping and ten percent methods, CLM-001).
///
/// # Panics
/// If either operand is not positive and finite. Validated metadata never passes one.
pub fn within_ten_percent(lower: f64, upper: f64) -> bool {
    assert!(lower.is_finite() && lower > 0.0 && upper.is_finite() && upper > 0.0, "positive finite frequencies");
    let (ml, el) = decompose(lower);
    let (mu, eu) = decompose(upper);
    // 10·mu·2^eu against 11·ml·2^el; each mantissa product is below 2^57.
    let a = 10u128 * u128::from(mu);
    let b = 11u128 * u128::from(ml);
    let ordering = if eu >= el {
        let shift = eu - el;
        if shift >= 64 {
            Ordering::Greater // a·2^shift ≥ 10·2^64 > b
        } else {
            (a << shift).cmp(&b)
        }
    } else {
        let shift = el - eu;
        if shift >= 64 {
            Ordering::Less
        } else {
            a.cmp(&(b << shift))
        }
    };
    ordering != Ordering::Greater
}

/// The Der Kiureghian correlation coefficient of two units with frequencies `fi` and `fj` (Hz)
/// at damping ratio `zeta`: ρ = 8ζ²(1 + r)r^{3/2} / ((1 − r²)² + 4ζ²r(1 + r)²), r = ω_j/ω_i
/// (CLM-001).
///
/// Multiplying numerator and denominator by r⁻⁴ gives the same expression in 1/r, so ρ is
/// symmetric in the two units. It is evaluated once per pair with s = f_low/f_high ≤ 1:
/// ρ = 8ζ²(1 + s)s^{3/2} / ((1 − s²)² + 4ζ²s(1 + s)²). No intermediate can overflow, whatever the
/// frequencies' ratio. The frequency ratio equals the ratio of circular frequencies in exact
/// arithmetic; it is taken on the published frequencies, the operands the closeness tests also
/// use. 1 − s² is formed as (1 − s)(1 + s), whose first factor is exact for 1/2 ≤ s ≤ 1. Equal
/// frequencies give exactly 1.
///
/// ρ enters the CQC sum as 2ρ·R_i·R_j, at most ρ·(R_i² + R_j²), so its absolute error is what
/// matters there. Where s^{3/2} or ζ² is below the binary64 range, ρ is below 2^-1022 and is
/// formed as 0 or a subnormal; the effect on the sum is below 2^-1022 of its diagonal terms. The
/// denominator is at least (1 − s²)² ≥ 2^-106 for distinct frequencies, so it never underflows.
///
/// # Panics
/// If a frequency is not positive and finite or `zeta` is not positive and finite. Validated
/// requests never pass one (CQC refuses ζ = 0).
pub fn correlation_coefficient(fi: f64, fj: f64, zeta: f64) -> f64 {
    assert!(fi.is_finite() && fi > 0.0 && fj.is_finite() && fj > 0.0, "positive finite frequencies");
    assert!(zeta.is_finite() && zeta > 0.0, "a positive finite damping ratio");
    if fi == fj {
        return 1.0;
    }
    let (low, high) = if fi < fj { (fi, fj) } else { (fj, fi) };
    let s = low / high;
    let z2 = zeta * zeta;
    let one_plus_s = 1.0 + s;
    let numerator = 8.0 * z2 * one_plus_s * (s * s.sqrt());
    let difference = (1.0 - s) * one_plus_s;
    let denominator = difference * difference + 4.0 * z2 * s * (one_plus_s * one_plus_s);
    numerator / denominator
}

/// 2^k exactly, for -1074 ≤ k ≤ 1023 (subnormal below -1022).
fn pow2(k: i32) -> f64 {
    debug_assert!((-1074..=1023).contains(&k));
    if k >= -1022 {
        f64::from_bits(((k + 1023) as u64) << 52)
    } else {
        f64::from_bits(1u64 << (k + 1074))
    }
}

/// E with 2^E ≤ x < 2^(E+1), for a positive finite x.
fn exponent(x: f64) -> i32 {
    let (m, e) = decompose(x);
    e + (63 - m.leading_zeros() as i32)
}

/// x·2^-e, for -1074 ≤ e ≤ 1023. Exact when it scales up (e < 0); rounds once when it scales
/// down.
fn scale_down(x: f64, e: i32) -> f64 {
    if e >= -1023 {
        x * pow2(-e)
    } else {
        // −e > 1023: two exact scalings up.
        x * pow2(1023) * pow2(-e - 1023)
    }
}

/// Evaluates `sum` on the values divided by 2^E (2^E ≤ max|v| < 2^(E+1)), takes the root and
/// multiplies it back by 2^E (module documentation, Dynamic range).
fn scaled_root(values: &[f64], sum: impl Fn(&[f64]) -> f64) -> Result<f64, RuleFailure> {
    if values.iter().any(|v| !v.is_finite()) {
        return Err(RuleFailure::Nonfinite);
    }
    let largest = values.iter().fold(0.0f64, |m, v| m.max(v.abs()));
    if largest == 0.0 {
        return root(sum(values));
    }
    let e = exponent(largest);
    let scaled: Vec<f64> = values.iter().map(|&v| scale_down(v, e)).collect();
    let r = root(sum(&scaled))?;
    finite(r * pow2(e))
}

/// A failure of one rule on one row: the CQC sum fell below zero, or a value left the binary64
/// range.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum RuleFailure {
    NegativeRadicand(f64),
    Nonfinite,
}

fn root(sum: f64) -> Result<f64, RuleFailure> {
    if !sum.is_finite() {
        Err(RuleFailure::Nonfinite)
    } else if sum < 0.0 {
        Err(RuleFailure::NegativeRadicand(sum))
    } else {
        Ok(sum.sqrt())
    }
}

fn finite(value: f64) -> Result<f64, RuleFailure> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(RuleFailure::Nonfinite)
    }
}

fn sum_of_squares(values: &[f64]) -> f64 {
    values.iter().fold(0.0, |s, v| s + v * v)
}

/// The prepared modal rule over the periodic units of one request: what CQC, grouping and the
/// ten percent method need besides the values.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum PreparedModalRule {
    Srss,
    /// ρ_ij for i < j, row-major over the upper triangle.
    Cqc { rho: Vec<Vec<f64>> },
    /// Coupled unit pairs (i < j), in ascending order.
    Pairs { pairs: Vec<(usize, usize)> },
    AbsoluteSum,
}

impl PreparedModalRule {
    /// The modal combination of one row in one direction, from its unit values in ascending
    /// cluster order.
    pub(crate) fn apply(&self, values: &[f64]) -> Result<f64, RuleFailure> {
        match self {
            Self::Srss => scaled_root(values, sum_of_squares),
            Self::Cqc { rho } => scaled_root(values, |v| {
                let mut sum = 0.0;
                for i in 0..v.len() {
                    sum += v[i] * v[i];
                    for j in (i + 1)..v.len() {
                        sum += 2.0 * rho[i][j] * v[i] * v[j];
                    }
                }
                sum
            }),
            Self::Pairs { pairs } => scaled_root(values, |v| {
                let mut sum = sum_of_squares(v);
                for &(i, j) in pairs {
                    sum += 2.0 * (v[i] * v[j]).abs();
                }
                sum
            }),
            Self::AbsoluteSum => finite(values.iter().fold(0.0, |s, v| s + v.abs())),
        }
    }
}

/// The groups of the grouping method over units with ascending frequencies: a group starts at
/// its lowest frequency f_g and takes each next unit with (f − f_g)/f_g ≤ 1/10.
pub(crate) fn groups(frequencies: &[f64]) -> Vec<Vec<usize>> {
    let mut out: Vec<Vec<usize>> = Vec::new();
    let mut lowest = None;
    for (i, &f) in frequencies.iter().enumerate() {
        match (lowest, out.last_mut()) {
            (Some(low), Some(group)) if within_ten_percent(low, f) => group.push(i),
            _ => {
                out.push(vec![i]);
                lowest = Some(f);
            }
        }
    }
    out
}

/// Every pair of the grouping method's groups, in ascending order.
pub(crate) fn group_pairs(groups: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();
    for group in groups {
        for (a, &i) in group.iter().enumerate() {
            for &j in &group[a + 1..] {
                pairs.push((i, j));
            }
        }
    }
    pairs
}

/// Every pair i < j of the ten percent method over units with ascending frequencies:
/// (f_j − f_i)/f_i ≤ 1/10, whether or not the two fall in one group.
pub(crate) fn ten_percent_pairs(frequencies: &[f64]) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();
    for i in 0..frequencies.len() {
        for j in (i + 1)..frequencies.len() {
            if within_ten_percent(frequencies[i], frequencies[j]) {
                pairs.push((i, j));
            }
        }
    }
    pairs
}

/// ρ over units with the given frequencies; the diagonal is 1.
pub(crate) fn correlation_matrix(frequencies: &[f64], zeta: f64) -> Vec<Vec<f64>> {
    let n = frequencies.len();
    let mut rho = vec![vec![1.0; n]; n];
    for i in 0..n {
        for j in (i + 1)..n {
            let value = correlation_coefficient(frequencies[i], frequencies[j], zeta);
            rho[i][j] = value;
            rho[j][i] = value;
        }
    }
    rho
}

/// The periodic part with the rigid response: SRSS √(R_p² + R_r²) or the absolute sum
/// R_p + |R_r| (REQ-001, Missing mass).
pub(crate) fn with_rigid_srss(periodic: f64, rigid: f64) -> Result<f64, RuleFailure> {
    scaled_root(&[periodic, rigid], sum_of_squares)
}

pub(crate) fn with_rigid_absolute_sum(periodic: f64, rigid: f64) -> Result<f64, RuleFailure> {
    finite(periodic + rigid.abs())
}

/// SRSS over directional magnitudes, in X, Y, Z order.
pub(crate) fn directional_srss(magnitudes: &[f64]) -> Result<f64, RuleFailure> {
    scaled_root(magnitudes, sum_of_squares)
}

/// The absolute sum of directional magnitudes, in X, Y, Z order.
pub(crate) fn directional_absolute_sum(magnitudes: &[f64]) -> Result<f64, RuleFailure> {
    finite(magnitudes.iter().sum())
}

/// 100-40-40 over directional magnitudes in X, Y, Z order: the governing direction is the
/// largest magnitude (the first on a tie), and the value is R_a + 0.4·R_b + 0.4·R_c, adding the
/// other directions in X, Y, Z order. Taking the largest full direction as governing is exact:
/// R_a + (2/5)(R_b + R_c) = (3/5)R_a + (2/5)ΣR. Returns the governing index and the value.
pub(crate) fn hundred_forty_forty(magnitudes: &[f64]) -> Result<(usize, f64), RuleFailure> {
    let mut governing = 0;
    for (i, &m) in magnitudes.iter().enumerate() {
        if m > magnitudes[governing] {
            governing = i;
        }
    }
    let mut value = magnitudes[governing];
    for (i, &m) in magnitudes.iter().enumerate() {
        if i != governing {
            value += FORTY_PERCENT * m;
        }
    }
    finite(value).map(|v| (governing, v))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_factor_is_the_nearest_binary64_to_two_fifths() {
        // 0.4 = 7205759403792794·2^-54, and 2/5 = 7205759403792793.6·2^-54.
        assert_eq!(FORTY_PERCENT.to_bits(), 0x3FD9_9999_9999_999A);
        let (m, e) = decompose(FORTY_PERCENT);
        assert_eq!((m, e), (7205759403792794, -54));
        // 5·m = 2·2^54 + 2, so 0.4 − 2/5 = 2^-53/5, which is 2^-54 of 2/5.
        assert_eq!(5 * u128::from(m), 2 * (1u128 << 54) + 2);
    }

    #[test]
    fn the_ten_percent_boundary_is_inclusive_and_exact() {
        assert!(within_ten_percent(10.0, 11.0));
        assert!(!within_ten_percent(10.0, f64::from_bits(11.0f64.to_bits() + 1)));
        assert!(within_ten_percent(10.0, f64::from_bits(11.0f64.to_bits() - 1)));
        // The test is on the represented operands: 10·0.11 against 11·0.1, both m·2^-56.
        let (m1, e1) = decompose(0.1);
        let (m2, e2) = decompose(0.11);
        assert_eq!((e1, e2), (-56, -56));
        let expected = 10 * u128::from(m2) <= 11 * u128::from(m1);
        assert_eq!(within_ten_percent(0.1, 0.11), expected);
        // 0.11 as represented exceeds 11/100 by less than 11/10 of 0.1's excess over 1/10.
        assert!(expected);
        assert!(within_ten_percent(f64::MIN_POSITIVE, f64::MIN_POSITIVE));
        assert!(!within_ten_percent(f64::from_bits(1), 1.0));
        assert!(within_ten_percent(f64::MAX / 1.05, f64::MAX));
    }

    #[test]
    fn the_correlation_is_symmetric_and_one_at_equal_frequencies() {
        let a = correlation_coefficient(18.75, 19.57, 0.05);
        assert_eq!(a.to_bits(), correlation_coefficient(19.57, 18.75, 0.05).to_bits());
        assert_eq!(correlation_coefficient(3.0, 3.0, 0.02), 1.0);
        assert!(a > 0.0 && a < 1.0);
    }

    #[test]
    fn grouping_and_ten_percent_differ_on_a_chain() {
        let f = [100.0, 108.0, 116.0];
        assert_eq!(groups(&f), vec![vec![0, 1], vec![2]]);
        assert_eq!(ten_percent_pairs(&f), vec![(0, 1), (1, 2)]);
    }

    #[test]
    fn hundred_forty_forty_ties_take_the_first_direction() {
        let (g, v) = hundred_forty_forty(&[1.0, 1.0, 0.25]).unwrap();
        assert_eq!(g, 0);
        assert_eq!(v, 1.0 + 0.4 * 1.0 + 0.4 * 0.25);
        assert_eq!(hundred_forty_forty(&[0.8]).unwrap(), (0, 0.8));
    }

    #[test]
    fn a_negative_cqc_sum_is_refused_not_repaired() {
        // ρ above 1 cannot arise from the coefficient; a supplied matrix shows the refusal path.
        let rule = PreparedModalRule::Cqc { rho: vec![vec![1.0, 1.5], vec![1.5, 1.0]] };
        assert!(matches!(rule.apply(&[1.0, -1.0]), Err(RuleFailure::NegativeRadicand(s)) if s < 0.0));
        // Scaled, this sum is 1 + 1 − 3 = −1: still refused.
        assert!(matches!(rule.apply(&[f64::MAX, -f64::MAX]), Err(RuleFailure::NegativeRadicand(_))));
    }

    #[test]
    fn squares_neither_overflow_nor_underflow() {
        // √(2)·1e200 and √(2)·1e-200 are finite; plain squares would overflow or vanish.
        for v in [1e200, 1e-200, 3e-320, f64::MAX / 2.0] {
            let r = PreparedModalRule::Srss.apply(&[v, -v]).unwrap();
            let expected = v * std::f64::consts::SQRT_2;
            assert!(((r - expected) / expected).abs() <= 2.0 * f64::EPSILON || v < 1e-300, "{v}: {r}");
            assert!(r > 0.0);
        }
        // The true value √2·MAX is beyond binary64: refused, not published.
        assert_eq!(PreparedModalRule::Srss.apply(&[f64::MAX, f64::MAX]), Err(RuleFailure::Nonfinite));
        // A tiny subnormal: the scaling up is exact, so a single value is returned exactly.
        let tiny = f64::from_bits(3);
        assert_eq!(PreparedModalRule::Srss.apply(&[tiny]).unwrap(), tiny);
        assert_eq!(PreparedModalRule::Srss.apply(&[0.0, -0.0]).unwrap(), 0.0);
        let (big, small) = (2f64.powi(900), 2f64.powi(-1000));
        assert_eq!(with_rigid_srss(3.0 * big, 4.0 * big).unwrap(), 5.0 * big);
        assert_eq!(directional_srss(&[3.0 * small, 4.0 * small]).unwrap(), 5.0 * small);
    }

    #[test]
    fn the_correlation_stays_finite_at_extreme_ratios() {
        let rho = correlation_coefficient(1e-300, 1e300, 0.05);
        assert!(rho.is_finite() && rho >= 0.0 && rho < 1e-200);
        let rho = correlation_coefficient(1.0, 1.0 + f64::EPSILON, 1e-200);
        assert!(rho.is_finite() && (0.0..=1.0).contains(&rho));
        // Here the exact ρ is 1 − O(1e-29); the binary64 value is within the rounding of the
        // formula's few operations, and may lie on either side of 1. Distinct clusters this
        // close do not arise from a modal result's δ_c chaining; if supplied, a CQC sum that
        // rounds below zero is refused.
        let close = correlation_coefficient(1.0, 1.0 + f64::EPSILON, 0.05);
        assert!((close - 1.0).abs() <= 8.0 * f64::EPSILON, "{close}");
    }
}
