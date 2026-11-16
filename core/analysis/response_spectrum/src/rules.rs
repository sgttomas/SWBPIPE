//! The combination rules of CLM-001, evaluated as enclosures of their exact values on the
//! represented operands (unit S4; Agent 0's route A/E2).
//!
//! **The exact values.** The operands are the supplied binary64 values: each contribution, each
//! rigid response, each frequency and the damping ratio. Every combined magnitude is a fixed
//! formula of them, and S4 publishes a binary64 for it only when an enclosure shows that binary64
//! within 1e-9 relative of the formula's exact value, or when the value is exactly zero
//! ([`certify`]):
//! - a cluster sum is an exact sum of dyadics (every finite binary64 is m·2^e exactly), with no
//!   rounding and no intermediate overflow;
//! - SRSS, grouping and ten percent take the square root of an exact sum of exact squares and
//!   absolute products; the absolute sum is exact;
//! - CQC encloses each ρ_ij from its exact rational factors (below), forms
//!   Q = Σ R_i² + 2·Σ_{i<j} ρ_ij·R_i·R_j with exact products of the enclosure's ends and the exact
//!   cluster sums, and takes the square root only of an enclosure whose lower end is positive
//!   (a positive radicand) or of an exact zero;
//! - the rigid rules, √(R_p² + R_r²) and R_p + |R_r|, and the directional rules, SRSS, the
//!   absolute sum and 100-40-40, are monotone in non-negative arguments, so they carry the
//!   enclosures of their arguments to an enclosure of their result;
//! - 100-40-40 is the exact rule value: max_a (R_a + (2/5)·Σ_{b≠a} R_b) = (3·max + 2·ΣR)/5, with
//!   the division by 5 enclosed.
//!
//! **ρ from exact rational factors.** With a = f_low and b = f_high (r = a/b; ρ is symmetric in the
//! two units, exactly), clearing the denominators of the scope's formula gives
//! ρ = N·√(ab) / (D·b), with N = 8ζ²(b + a)·a·b² and D = (b² − a²)² + 4ζ²·a·b·(b + a)², exact
//! dyadics of the binary64 operands. √(ab) and the division are enclosed outward at the working
//! precision; nothing else rounds. No intermediate can overflow or underflow, since dyadics have
//! unbounded exponents. Equal frequencies give exactly 1.
//!
//! **The enclosures' sizes.** Every enclosure end is an exact dyadic. Its stored integer size
//! depends on the exponent spans of the actual intermediates, including the damping and
//! frequency factors in ρ and the alignment of its terms with the exact cluster products.
//! The working precision follows [`PRECISION_LADDER`]; it does not bound the stored bit span
//! of an accumulated endpoint or the request's memory use. No storage bound is claimed here.
//! An observed 53-bit CQC evaluation at frequencies [2^-1074, f64::MAX], damping 2^-1074 and
//! contributions [1, 1] has a radicand endpoint requiring 5,356 integer bits.
use crate::dyadic::{self, Dyadic, Interval};
use crate::exact::decompose;
use std::cmp::Ordering;

/// The working precisions tried in turn, in significant bits, for the enclosures of ρ; 1,024 is
/// the cap. At 53 the enclosures are binary64-guided and checked exactly; above it they come from
/// exact integer division and square roots. The square roots and quotients of the combined
/// magnitudes themselves are always binary64-guided and checked (relative width about 2^-49):
/// nothing after them cancels, so their width never decides certification. The cap's reason
/// and its measured cost are in the crate README.
pub const PRECISION_LADDER: [u64; 5] = [53, 128, 256, 512, 1024];

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
    let a = 10u128 * u128::from(mu);
    let b = 11u128 * u128::from(ml);
    let ordering = if eu >= el {
        let shift = eu - el;
        if shift >= 64 {
            Ordering::Greater
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

/// An enclosure of the Der Kiureghian coefficient ρ of two units with frequencies `fi` and `fj`
/// (Hz, binary64) at damping ratio `zeta`, at `p` bits (module documentation).
pub(crate) fn correlation_enclosure(fi: f64, fj: f64, zeta: f64, p: u64) -> Interval {
    let (low, high) = if fi <= fj { (fi, fj) } else { (fj, fi) };
    let (a, b, z) = (Dyadic::from_f64(low), Dyadic::from_f64(high), Dyadic::from_f64(zeta));
    let z2 = z.mul(&z);
    let sum = b.add(&a);
    let numerator = z2.mul_int(8).mul(&sum).mul(&a).mul(&b).mul(&b);
    let difference = b.mul(&b).sub(&a.mul(&a));
    let denominator = difference.mul(&difference).add(&z2.mul_int(4).mul(&a).mul(&b).mul(&sum).mul(&sum));
    let divisor = denominator.mul(&b);
    if p <= 53 {
        let root = dyadic::sqrt_guided(&a.mul(&b));
        let lo = dyadic::div_guided(&numerator.mul(&root.lo), &divisor).lo;
        let hi = dyadic::div_guided(&numerator.mul(&root.hi), &divisor).hi;
        return Interval { lo, hi };
    }
    let root = dyadic::sqrt(&a.mul(&b), p);
    let lo = dyadic::div(&numerator.mul(&root.lo), &divisor, p).lo;
    let hi = dyadic::div(&numerator.mul(&root.hi), &divisor, p).hi;
    Interval { lo, hi }
}

/// The Der Kiureghian correlation coefficient of two units with frequencies `fi` and `fj` (Hz) at
/// damping ratio `zeta`: ρ = 8ζ²(1 + r)r^{3/2} / ((1 − r²)² + 4ζ²r(1 + r)²), r = ω_j/ω_i
/// (CLM-001), as the binary64 nearest the midpoint of a 128-bit enclosure of its exact value on
/// the represented operands. It is informational (the combinations use the enclosure itself) and
/// is within one rounding of the exact value.
///
/// # Panics
/// If a frequency or `zeta` is not positive and finite.
pub fn correlation_coefficient(fi: f64, fj: f64, zeta: f64) -> f64 {
    assert!(fi.is_finite() && fi > 0.0 && fj.is_finite() && fj > 0.0, "positive finite frequencies");
    assert!(zeta.is_finite() && zeta > 0.0, "a positive finite damping ratio");
    let e = correlation_enclosure(fi, fj, zeta, PRECISION_LADDER[1]);
    e.lo.midpoint(&e.hi).to_f64()
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

/// The modal rule, prepared over the periodic units of one request.
#[derive(Debug, Clone)]
pub(crate) enum ModalForm {
    Srss,
    Cqc,
    /// Coupled unit pairs (i < j), in ascending order: grouping and ten percent.
    Pairs(Vec<(usize, usize)>),
    AbsoluteSum,
}

/// The ρ enclosures of every unit pair at one precision, row-major over i < j. At the first
/// level the ends are rounded outward to 64 bits, so that the CQC inner sums can be accumulated
/// from 128-bit products without allocation.
pub(crate) struct CorrelationTable {
    rho: Vec<Vec<Interval>>,
    /// (lo, hi) as (mantissa, exponent) when every end fits 64 bits.
    small: Option<Vec<Vec<((u64, i64), (u64, i64))>>>,
}

impl CorrelationTable {
    pub(crate) fn new(frequencies: &[f64], zeta: f64, p: u64) -> Self {
        let n = frequencies.len();
        let rho: Vec<Vec<Interval>> = (0..n)
            .map(|i| {
                ((i + 1)..n)
                    .map(|j| {
                        let e = correlation_enclosure(frequencies[i], frequencies[j], zeta, p);
                        if p <= 64 {
                            Interval { lo: e.lo.round_outward(64, false), hi: e.hi.round_outward(64, true) }
                        } else {
                            e
                        }
                    })
                    .collect()
            })
            .collect();
        let small = rho
            .iter()
            .map(|row| {
                row.iter()
                    .map(|e| match (e.lo.small(), e.hi.small()) {
                        (Some((false, ml, el)), Some((false, mh, eh))) => Some(((ml, el), (mh, eh))),
                        _ => None,
                    })
                    .collect::<Option<Vec<_>>>()
            })
            .collect::<Option<Vec<_>>>();
        Self { rho, small }
    }

    fn get(&self, i: usize, j: usize) -> &Interval {
        &self.rho[i][j - i - 1]
    }

    /// The binary64 nearest each enclosure's midpoint, as a full matrix with a unit diagonal
    /// (informational).
    pub(crate) fn nearest(&self, n: usize) -> Vec<Vec<f64>> {
        let mut m = vec![vec![1.0; n]; n];
        for i in 0..n {
            for j in (i + 1)..n {
                let e = self.get(i, j);
                let v = e.lo.midpoint(&e.hi).to_f64();
                m[i][j] = v;
                m[j][i] = v;
            }
        }
        m
    }
}

/// Why an enclosure could not be published at the working precision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Uncertain {
    /// The CQC radicand's enclosure does not have a positive lower end and is not exactly zero.
    RadicandNotPositive,
    /// The binary64 nearest the enclosure's midpoint is not shown within 1e-9 relative of every
    /// point of the enclosure. `tight` when the enclosure is narrower than a quarter of the
    /// tolerance. Either way it records what this argument could not establish.
    NotWithinTolerance { tight: bool },
}

/// The outcome of certifying one value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Certified {
    Value(f64),
    Uncertain(Uncertain),
    /// The enclosure's lower end rounds beyond the binary64 range.
    Nonfinite,
}

/// The binary64 published for an enclosure [lo, hi] of a non-negative exact value x: exactly zero
/// for an exact zero; otherwise v, the binary64 nearest the midpoint, when lo > 0 and
/// |v − x| ≤ 1e-9·x for every x in [lo, hi]. Since |v − x|/x is largest at an end, that is
/// 10⁹·(v − lo) ≤ lo and 10⁹·(hi − v) ≤ hi, decided exactly.
pub(crate) fn certify(e: &Interval) -> Certified {
    if e.is_exact_zero() {
        return Certified::Value(0.0);
    }
    if e.lo.is_zero() || e.lo.is_negative() {
        return Certified::Uncertain(Uncertain::NotWithinTolerance { tight: false });
    }
    let v = e.lo.midpoint(&e.hi).to_f64();
    if v.is_infinite() {
        return if e.lo.to_f64().is_infinite() {
            Certified::Nonfinite
        } else {
            Certified::Uncertain(Uncertain::NotWithinTolerance { tight: false })
        };
    }
    let vd = Dyadic::from_f64(v);
    let billion = 1_000_000_000;
    let below = vd.sub(&e.lo).mul_int(billion).cmp(&e.lo) != Ordering::Greater;
    let above = e.hi.sub(&vd).mul_int(billion).cmp(&e.hi) != Ordering::Greater;
    if below && above {
        Certified::Value(v)
    } else {
        let tight = e.hi.sub(&e.lo).mul_int(4 * billion).cmp(&e.lo) == Ordering::Less;
        Certified::Uncertain(Uncertain::NotWithinTolerance { tight })
    }
}

/// An enclosure of the modal combination of one row in one direction from its exact cluster sums.
pub(crate) fn modal(form: &ModalForm, table: Option<&CorrelationTable>, values: &[Dyadic])
    -> Result<Interval, Uncertain> {
    let squares = || values.iter().fold(Dyadic::zero(), |s, v| s.add(&v.mul(v)));
    match form {
        ModalForm::Srss => Ok(dyadic::sqrt_interval_guided(&Interval::exact(squares()))),
        ModalForm::AbsoluteSum => Ok(Interval::exact(values.iter().fold(Dyadic::zero(), |s, v| s.add(&v.abs())))),
        ModalForm::Pairs(pairs) => {
            let mut sum = squares();
            for &(i, j) in pairs {
                sum = sum.add(&values[i].mul(&values[j]).abs().scale2(1));
            }
            Ok(dyadic::sqrt_interval_guided(&Interval::exact(sum)))
        }
        ModalForm::Cqc => {
            let table = table.expect("a correlation table under CQC");
            let q = match cqc_fast(table, values) {
                Some(q) => q,
                None => {
                    let mut q = Interval::exact(squares());
                    for i in 0..values.len() {
                        for j in (i + 1)..values.len() {
                            let product = values[i].mul(&values[j]).scale2(1);
                            q = q.add(&table.get(i, j).mul_exact(&product));
                        }
                    }
                    q
                }
            };
            if q.is_exact_zero() {
                Ok(q)
            } else if q.lo.is_zero() || q.lo.is_negative() {
                Err(Uncertain::RadicandNotPositive)
            } else {
                Ok(dyadic::sqrt_interval_guided(&q))
            }
        }
    }
}

/// The CQC radicand's enclosure, exactly as the general path forms it, when every cluster sum and
/// every ρ end fits 64 bits: Q ∈ Σ S_i² + 2·Σ_i S_i·[A_i, B_i], with
/// A_i = Σ_{j>i} S_j·(ρlo_ij if S_j ≥ 0 else ρhi_ij) and B_i the same with lo and hi exchanged,
/// so that S_j·ρ_ij ∈ [S_j·(…), S_j·(…)] term by term. The inner sums are exact fixed-point
/// accumulations of 128-bit products.
fn cqc_fast(table: &CorrelationTable, values: &[Dyadic]) -> Option<Interval> {
    let small_rho = table.small.as_ref()?;
    let s: Vec<(bool, u64, i64)> = values.iter().map(Dyadic::small).collect::<Option<_>>()?;
    let n = values.len();
    if n < 2 {
        return None;
    }
    // The exponent range of every product S_j·ρ end.
    let (mut base, mut top) = (i64::MAX, i64::MIN);
    for (i, row) in small_rho.iter().enumerate() {
        for (k, &((_, el), (_, eh))) in row.iter().enumerate() {
            let (_, m, e) = s[i + 1 + k];
            if m != 0 {
                base = base.min(e + el.min(eh));
                top = top.max(e + el.max(eh) + 128);
            }
        }
    }
    let mut q = Interval::exact(values.iter().fold(Dyadic::zero(), |acc, v| acc.add(&v.mul(v))));
    if base == i64::MAX {
        return Some(q);
    }
    let mut lo_acc = dyadic::ExactAccumulator::new(base, top);
    let mut hi_acc = dyadic::ExactAccumulator::new(base, top);
    for i in 0..n - 1 {
        if values[i].is_zero() {
            continue;
        }
        lo_acc.clear();
        hi_acc.clear();
        for (k, &((ml, el), (mh, eh))) in small_rho[i].iter().enumerate() {
            let (negative, m, e) = s[i + 1 + k];
            let (a, b) = if negative { ((mh, eh), (ml, el)) } else { ((ml, el), (mh, eh)) };
            lo_acc.add(negative, u128::from(m) * u128::from(a.0), e + a.1);
            hi_acc.add(negative, u128::from(m) * u128::from(b.0), e + b.1);
        }
        let w = Interval { lo: lo_acc.value(), hi: hi_acc.value() };
        q = q.add(&w.mul_exact(&values[i].scale2(1)));
    }
    Some(q)
}

/// √(P² + r²) for an enclosure P ≥ 0 and an exact r.
pub(crate) fn with_rigid_srss(periodic: &Interval, rigid: &Dyadic) -> Interval {
    let r2 = rigid.mul(rigid);
    let lo = dyadic::sqrt_guided(&periodic.lo.mul(&periodic.lo).add(&r2)).lo;
    let hi = dyadic::sqrt_guided(&periodic.hi.mul(&periodic.hi).add(&r2)).hi;
    Interval { lo, hi }
}

/// P + |r| for an enclosure P ≥ 0 and an exact r.
pub(crate) fn with_rigid_absolute_sum(periodic: &Interval, rigid: &Dyadic) -> Interval {
    periodic.add(&Interval::exact(rigid.abs()))
}

/// SRSS over directional enclosures (each ≥ 0).
pub(crate) fn directional_srss(parts: &[Interval]) -> Interval {
    let lo = parts.iter().fold(Dyadic::zero(), |s, e| s.add(&e.lo.mul(&e.lo)));
    let hi = parts.iter().fold(Dyadic::zero(), |s, e| s.add(&e.hi.mul(&e.hi)));
    Interval { lo: dyadic::sqrt_guided(&lo).lo, hi: dyadic::sqrt_guided(&hi).hi }
}

/// The absolute sum of directional enclosures (each ≥ 0).
pub(crate) fn directional_absolute_sum(parts: &[Interval]) -> Interval {
    parts.iter().fold(Interval::exact(Dyadic::zero()), |s, e| s.add(e))
}

/// 100-40-40 over directional enclosures (each ≥ 0), with the exact factor 2/5:
/// (3·max + 2·Σ)/5, monotone in every argument.
pub(crate) fn hundred_forty_forty(parts: &[Interval]) -> Interval {
    let bound = |pick: fn(&Interval) -> &Dyadic| {
        let max = parts.iter().map(pick).fold(Dyadic::zero(), |m, v| if v.cmp(&m) == Ordering::Greater { v.clone() } else { m });
        let sum = parts.iter().map(pick).fold(Dyadic::zero(), |s, v| s.add(v));
        max.mul_int(3).add(&sum.mul_int(2))
    };
    let five = Dyadic::from_int(5);
    Interval { lo: dyadic::div_guided(&bound(|e| &e.lo), &five).lo, hi: dyadic::div_guided(&bound(|e| &e.hi), &five).hi }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_ten_percent_boundary_is_inclusive_and_exact() {
        assert!(within_ten_percent(10.0, 11.0));
        assert!(!within_ten_percent(10.0, f64::from_bits(11.0f64.to_bits() + 1)));
        assert!(within_ten_percent(10.0, f64::from_bits(11.0f64.to_bits() - 1)));
        let (m1, e1) = decompose(0.1);
        let (m2, e2) = decompose(0.11);
        assert_eq!((e1, e2), (-56, -56));
        assert_eq!(within_ten_percent(0.1, 0.11), 10 * u128::from(m2) <= 11 * u128::from(m1));
        assert!(within_ten_percent(f64::MIN_POSITIVE, f64::MIN_POSITIVE));
        assert!(!within_ten_percent(f64::from_bits(1), 1.0));
        assert!(within_ten_percent(f64::MAX / 1.05, f64::MAX));
    }

    #[test]
    fn the_correlation_is_exactly_symmetric_and_one_at_equal_frequencies() {
        let a = correlation_enclosure(18.75, 19.57, 0.05, 128);
        assert_eq!(a, correlation_enclosure(19.57, 18.75, 0.05, 128));
        assert_eq!(correlation_enclosure(3.0, 3.0, 0.02, 128), Interval::exact(Dyadic::from_int(1)));
        let rho = correlation_coefficient(18.75, 19.57, 0.05);
        assert!(rho > 0.0 && rho < 1.0);
    }

    #[test]
    fn the_correlation_keeps_its_value_at_extreme_operands() {
        // Review witness: f = 1 and the next binary64 above, ζ = 1e-162; the exact ρ is
        // 8.11296384146066922605e-293 (the reviewer's independent Decimal derivation), where ζ²
        // alone is below the binary64 range.
        let rho = correlation_coefficient(1.0, 1.0 + f64::EPSILON, 1e-162);
        let expected = 8.11296384146066922605e-293;
        assert!(((rho - expected) / expected).abs() < 1e-15, "{rho:e}");
        // ρ ≈ 8ζ²s^{3/2} with s = 1e-600 is about 2e-902, below the binary64 range: the enclosure
        // holds it, and its nearest binary64 is 0.
        let e = correlation_enclosure(1e-300, 1e300, 0.05, 128);
        assert!(!e.lo.is_zero() && !e.lo.is_negative());
        assert_eq!(correlation_coefficient(1e-300, 1e300, 0.05), 0.0);
        // Here the exact ρ is 1 − O(1e-29), so its nearest binary64 is 1; the enclosure is below 1.
        assert_eq!(correlation_coefficient(1.0, 1.0 + f64::EPSILON, 0.05), 1.0);
        let e = correlation_enclosure(1.0, 1.0 + f64::EPSILON, 0.05, 128);
        assert!(e.hi.cmp(&Dyadic::from_int(1)) == Ordering::Less);
    }

    #[test]
    fn grouping_and_ten_percent_differ_on_a_chain() {
        let f = [100.0, 108.0, 116.0];
        assert_eq!(groups(&f), vec![vec![0, 1], vec![2]]);
        assert_eq!(ten_percent_pairs(&f), vec![(0, 1), (1, 2)]);
    }

    #[test]
    fn certification_publishes_exact_zero_and_refuses_what_it_cannot_show() {
        assert_eq!(certify(&Interval::exact(Dyadic::zero())), Certified::Value(0.0));
        assert_eq!(certify(&Interval::exact(Dyadic::from_f64(0.3))), Certified::Value(0.3));
        // An enclosure of 9/5 quanta of 2^-1074: tight, but the nearest binary64 is 2 quanta.
        let q = dyadic::div_guided(&Dyadic::from_f64(5e-324).mul_int(9), &Dyadic::from_int(5));
        assert_eq!(certify(&q), Certified::Uncertain(Uncertain::NotWithinTolerance { tight: true }));
        let straddling = Interval { lo: Dyadic::from_int(-1), hi: Dyadic::from_int(1) };
        assert!(matches!(certify(&straddling), Certified::Uncertain(_)));
        let huge = Interval::exact(Dyadic::from_f64(f64::MAX).mul_int(2));
        assert_eq!(certify(&huge), Certified::Nonfinite);
    }

    #[test]
    fn hundred_forty_forty_uses_the_exact_factor() {
        let parts = [1.25, 0.5, 0.3].map(|v| Interval::exact(Dyadic::from_f64(v)));
        let e = hundred_forty_forty(&parts);
        let numerator = Dyadic::from_f64(1.25).mul_int(5).add(&Dyadic::from_f64(0.5).add(&Dyadic::from_f64(0.3)).mul_int(2));
        assert!(e.lo.mul_int(5).cmp(&numerator) != Ordering::Greater);
        assert!(e.hi.mul_int(5).cmp(&numerator) != Ordering::Less);
        let one = [Interval::exact(Dyadic::from_f64(0.8))];
        let e = hundred_forty_forty(&one);
        assert_eq!(e.lo.midpoint(&e.hi).to_f64(), 0.8);
    }
}
