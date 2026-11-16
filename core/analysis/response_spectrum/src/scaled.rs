//! Positive reals carried as `m·2^e`, a binary64 significand `m` in [1, 2) and an integer
//! exponent `e`, so that the evaluation's products, quotients, positive sums and exponentials
//! never overflow or underflow, whatever the magnitudes of the represented inputs.
//!
//! Every operation below is binary64 arithmetic on significands in [1, 4) or (1/2, 2), which are
//! normal numbers far from either end of the format, so each rounds once with relative error at
//! most u = 2⁻⁵³ (IEEE 754 round to nearest). Splitting a value into `m` and `e`, and scaling by a
//! power of two inside the normal range, are exact. The only place a value can leave the binary64
//! range is [`Scaled::to_f64`], where the caller certifies the published value
//! ([`crate::table`]).
use std::cmp::Ordering;

/// A positive real `m·2^e`, `1 ≤ m < 2`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Scaled {
    m: f64,
    e: i32,
}

const FRACTION: u64 = (1 << 52) - 1;

/// `2^k` as a binary64, exactly, for `-1022 ≤ k ≤ 1023`.
fn pow2(k: i32) -> f64 {
    debug_assert!((-1022..=1023).contains(&k));
    f64::from_bits(((k + 1023) as u64) << 52)
}

/// The exact split of a positive normal finite binary64.
fn split_normal(x: f64) -> (f64, i32) {
    let bits = x.to_bits();
    let biased = ((bits >> 52) & 0x7ff) as i32;
    debug_assert!(x > 0.0 && (1..0x7ff).contains(&biased), "{x} is not positive, normal and finite");
    (f64::from_bits((bits & FRACTION) | (1023 << 52)), biased - 1023)
}

impl Scaled {
    /// The exact representation of a positive finite binary64, subnormals included.
    pub(crate) fn new(x: f64) -> Self {
        assert!(x.is_finite() && x > 0.0, "{x} is not positive and finite");
        if x < f64::MIN_POSITIVE {
            // A subnormal times 2^64 is a normal number, exactly.
            let (m, e) = split_normal(x * pow2(64));
            Self { m, e: e - 64 }
        } else {
            let (m, e) = split_normal(x);
            Self { m, e }
        }
    }

    /// `m·2^e` for a positive normal significand `m` from one rounded operation; the split is exact.
    fn renormalized(m: f64, e: i32) -> Self {
        let (m, k) = split_normal(m);
        Self { m, e: e + k }
    }

    /// The product, rounded once.
    pub(crate) fn mul(self, other: Self) -> Self {
        Self::renormalized(self.m * other.m, self.e + other.e)
    }

    /// The quotient, rounded once.
    pub(crate) fn div(self, other: Self) -> Self {
        Self::renormalized(self.m / other.m, self.e - other.e)
    }

    /// The sum of two positive values, rounded once. When the smaller is below 2⁻¹⁰⁰⁰ of the larger
    /// it is dropped, a relative error below 2⁻⁹⁹⁹; otherwise it is aligned exactly.
    pub(crate) fn add(self, other: Self) -> Self {
        let (hi, lo) = if self.e >= other.e { (self, other) } else { (other, self) };
        let shift = lo.e - hi.e;
        if shift < -1000 {
            return hi;
        }
        Self::renormalized(hi.m + lo.m * pow2(shift), hi.e)
    }

    /// The significand and exponent.
    pub(crate) fn parts(self) -> (f64, i32) {
        (self.m, self.e)
    }

    /// The nearest binary64: exact for `-1022 ≤ e ≤ 1023`; rounded once into the subnormal range
    /// (absolute error at most 2⁻¹⁰⁷⁵) for smaller `e`, and zero for `e ≤ -1076`; infinite for
    /// `e > 1023`.
    pub(crate) fn to_f64(self) -> f64 {
        match self.e {
            e if e > 1023 => f64::INFINITY,
            e if e >= -1022 => self.m * pow2(e),
            e if e <= -1076 => 0.0,
            // m·2^-1000 is normal and exact; only the second product rounds.
            e => (self.m * pow2(-1000)) * pow2(e + 1000),
        }
    }

    /// Whether this value is at most `other`.
    pub(crate) fn le(self, other: Self) -> bool {
        (self.e, self.m) <= (other.e, other.m)
    }

    /// The value's ordering against `2^k`.
    pub(crate) fn cmp_pow2(self, k: i32) -> Ordering {
        self.e.cmp(&k).then(self.m.partial_cmp(&1.0).expect("finite"))
    }
}

/// fdlibm's split of ln 2: `LN2_HI` has 32 significant bits, so `k·LN2_HI` is exact for `|k| < 2^21`,
/// and `LN2_HI + LN2_LO`, as exact binary64 values, differs from ln 2 by 1.17·10⁻²⁶ (below 2⁻⁸⁶;
/// computed at 80 digits for this argument).
pub(crate) const LN2_HI: f64 = f64::from_bits(0x3fe6_2e42_fee0_0000); // 6.93147180369123816490e-01
pub(crate) const LN2_LO: f64 = f64::from_bits(0x3dea_39ef_3579_3c76); // 1.90821492927058770002e-10

/// eᶻ as a [`Scaled`], for finite `|z| ≤ 1500`, by the standard reduction z = k·ln 2 + r with
/// integer k = round(z/ln 2): r = (z − k·LN2_HI) − k·LN2_LO, where k·LN2_HI is exact and its
/// difference from z is exact (Sterbenz), so r carries one rounding of at most u·|r| and the
/// product k·LN2_LO's rounding and the split's error, together below 3·10⁻²³ absolutely for
/// |k| ≤ 2200. Then eᶻ = e^r·2^k with |r| ≤ 0.35, and e^r
/// is the platform exponential (the libm premise of [`crate::table`]).
pub(crate) fn exp_scaled(z: f64) -> Scaled {
    assert!(z.is_finite() && z.abs() <= 1500.0, "exponent {z} outside the reduction's range");
    let k = (z / std::f64::consts::LN_2).round();
    let r = (z - k * LN2_HI) - k * LN2_LO;
    Scaled::renormalized(r.exp(), k as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splitting_and_recombining_is_exact_across_the_range() {
        for x in [1.0, 1.5, f64::MAX, f64::MIN_POSITIVE, f64::from_bits(1), f64::from_bits(0x000f_ffff_ffff_ffff), 1e-310, 3e300] {
            let s = Scaled::new(x);
            assert!((1.0..2.0).contains(&s.m), "{x}: {s:?}");
            assert_eq!(s.to_f64().to_bits(), x.to_bits(), "{x}");
        }
        assert_eq!(Scaled::new(f64::from_bits(1)).parts(), (1.0, -1074));
    }

    #[test]
    fn rounding_into_the_subnormal_range_is_single_and_to_nearest() {
        // 1.5·2^-1074 is a tie between 2^-1074 and 2^-1073: ties to even gives 2^-1073.
        assert_eq!(Scaled { m: 1.5, e: -1074 }.to_f64(), f64::from_bits(2));
        // Just above 2^-1075 rounds up to the smallest subnormal; 2^-1075 itself ties to zero.
        assert_eq!(Scaled { m: 1.0f64.next_up(), e: -1075 }.to_f64(), f64::from_bits(1));
        assert_eq!(Scaled { m: 1.0, e: -1075 }.to_f64(), 0.0);
        assert_eq!(Scaled { m: 1.999, e: -1076 }.to_f64(), 0.0);
        // A value with more bits than the subnormal grid holds: at 2^-1050 the grid keeps 25 bits, so
        // 1 + 2^-52 rounds to 1. (2f64.powi(-1050) is not used: powi underflows through 1/2^1050.)
        assert_eq!(Scaled { m: 1.0f64.next_up(), e: -1050 }.to_f64(), f64::from_bits(1 << 24));
        assert_eq!(Scaled { m: 1.999_999_999_999_999_8, e: 1023 }.to_f64(), 1.999_999_999_999_999_8 * 2f64.powi(1023));
        assert!(Scaled { m: 1.0, e: 1024 }.to_f64().is_infinite());
    }

    #[test]
    fn products_and_sums_far_outside_the_format_stay_exact_in_the_exponent() {
        let tiny = Scaled::new(f64::from_bits(1));
        let huge = Scaled::new(f64::MAX);
        assert_eq!(tiny.mul(tiny).parts(), (1.0, -2148));
        assert_eq!(huge.mul(tiny).to_f64(), f64::MAX * f64::from_bits(1));
        assert_eq!(tiny.div(huge).mul(huge).div(tiny).to_f64(), 1.0);
        assert_eq!(Scaled::new(1.0).add(Scaled::new(1e-300)).to_f64(), 1.0);
        assert_eq!(Scaled::new(1.0).add(tiny.mul(tiny)).parts(), (1.0, 0));
        assert_eq!(Scaled::new(3.0).add(Scaled::new(5.0)).to_f64(), 8.0);
    }

    #[test]
    fn the_exponential_matches_the_platform_inside_the_format_and_continues_outside_it() {
        for z in [-700.0, -1.0, -0.3, 0.0, 0.25, 1.0, 10.0, 700.0] {
            let e = exp_scaled(z).to_f64();
            let r = ((e - z.exp()) / z.exp()).abs();
            assert!(r <= 4.0 * f64::EPSILON, "{z}: {e} against {}", z.exp());
        }
        // e^1000 = 2^1442.695...: beyond binary64, but its significand and exponent are finite.
        let (m, e) = exp_scaled(1000.0).parts();
        assert_eq!(e, 1442);
        assert!((m - 2f64.powf(1000.0 / std::f64::consts::LN_2 - 1442.0)).abs() <= 1e-12);
        assert_eq!(exp_scaled(-1000.0).mul(exp_scaled(1000.0)).to_f64(), 1.0);
    }

    #[test]
    fn the_ln2_split_is_the_fdlibm_split() {
        assert_eq!(LN2_HI, 6.931_471_803_691_238e-1);
        assert_eq!(LN2_LO, 1.908_214_929_270_587_7e-10);
        assert_eq!(LN2_HI.to_bits() & 0x1f_ffff, 0, "LN2_HI has at most 32 significant bits");
        assert_eq!(LN2_HI + LN2_LO, std::f64::consts::LN_2);
    }
}
