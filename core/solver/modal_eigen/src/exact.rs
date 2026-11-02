//! Exact accumulation of sums of products (design §3.2, admission range:
//! bilinear forms are "accumulated so that no contribution dropped by
//! exponent alignment can be exposed by later cancellation").
//!
//! [`ExactSum`] is a superaccumulator: a signed fixed-point integer in 32-bit
//! limbs with a movable integer exponent, so it holds any finite sum of
//! binary64 values times powers of two exactly, however far apart their
//! magnitudes. A product of binary64 factors enters as its exact expansion:
//! each factor is split into a significand in [1, 2) and an integer
//! exponent, and the significands are multiplied with TwoProduct, whose two
//! parts are exact because the significands never approach the binary64
//! underflow range (a product of three factors is four binary64 parts). The
//! sum is rounded once, at the end, to a [`WideDd`] whose high word is the
//! correctly rounded binary64 significand and whose low word carries the
//! remainder rounded to odd, so that [`WideDd::to_f64`] and the
//! classification of [`WideDd::to_published`] see the exact value's rounding
//! and its side of every midpoint.
//!
//! Every add is O(1) limb operations; carries are propagated every 2²⁸ adds
//! and before a result is read.

use crate::dd::{binary_exponent, scale_by_pow2, two_prod, Dd, WideDd};

/// Carries are propagated after this many adds: each add changes a limb by
/// less than 2³², so the limbs stay below 2⁶⁰ in magnitude.
const NORMALIZE_EVERY: u32 = 1 << 28;

/// An exact sum of binary64 values times powers of two.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExactSum {
    /// `limbs[k]` has weight 2^(32·(low + k)); signed until normalized.
    limbs: Vec<i64>,
    low: i32,
    pending: u32,
    /// The sum of the non-finite terms, if any (∞ − ∞ is NaN).
    non_finite: Option<f64>,
}

/// One exact term of a sum: `value·2^exponent`.
pub type Term = (f64, i32);

impl ExactSum {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_finite(&self) -> bool {
        self.non_finite.is_none()
    }

    /// Adds `value·2^exponent` exactly.
    pub fn add_scaled(&mut self, value: f64, exponent: i32) {
        if value == 0.0 {
            return;
        }
        if !value.is_finite() {
            self.non_finite = Some(self.non_finite.map_or(value, |v| v + value));
            return;
        }
        let bits = value.abs().to_bits();
        let field = (bits >> 52) as i32;
        let fraction = bits & ((1u64 << 52) - 1);
        let (significand, lsb) = if field == 0 {
            (fraction, -1074)
        } else {
            (fraction | (1u64 << 52), field - 1075)
        };
        let lsb = lsb + exponent;
        let limb = lsb.div_euclid(32);
        if self.limbs.is_empty() {
            self.low = limb;
            self.limbs = vec![0; 4];
        } else if limb < self.low {
            let grow = (self.low - limb) as usize;
            self.limbs.splice(0..0, std::iter::repeat_n(0, grow));
            self.low = limb;
        }
        let index = (limb - self.low) as usize;
        if self.limbs.len() < index + 4 {
            self.limbs.resize(index + 4, 0);
        }
        let wide = (significand as u128) << lsb.rem_euclid(32);
        let negative = value < 0.0;
        for (k, chunk) in [
            wide & 0xFFFF_FFFF,
            (wide >> 32) & 0xFFFF_FFFF,
            wide >> 64,
        ]
        .into_iter()
        .enumerate()
        {
            let chunk = chunk as i64;
            if negative {
                self.limbs[index + k] -= chunk;
            } else {
                self.limbs[index + k] += chunk;
            }
        }
        self.pending += 1;
        if self.pending >= NORMALIZE_EVERY {
            self.normalize();
        }
    }

    pub fn add_f64(&mut self, value: f64) {
        self.add_scaled(value, 0);
    }

    /// Adds the exact product of the factors (at most three) times
    /// 2^exponent.
    pub fn add_product_scaled(&mut self, factors: &[f64], exponent: i32) {
        debug_assert!(!factors.is_empty() && factors.len() <= 3);
        if factors.contains(&0.0) && factors.iter().all(|f| f.is_finite()) {
            return;
        }
        if factors.iter().any(|f| !f.is_finite()) {
            let product = factors.iter().product::<f64>();
            self.non_finite = Some(self.non_finite.map_or(product, |v| v + product));
            return;
        }
        let mut scale = exponent;
        let mut parts: [f64; 4] = [0.0; 4];
        let mut count = 0;
        for (i, &factor) in factors.iter().enumerate() {
            let e = binary_exponent(factor);
            scale += e;
            let significand = scale_by_pow2(factor, -e);
            if i == 0 {
                parts[0] = significand;
                count = 1;
                continue;
            }
            // Each part times a significand in [1, 2): TwoProduct is exact,
            // its low part far above the underflow range.
            let mut next = [0.0; 4];
            let mut next_count = 0;
            for &part in &parts[..count] {
                let (hi, lo) = two_prod(part, significand);
                next[next_count] = hi;
                next_count += 1;
                if lo != 0.0 {
                    next[next_count] = lo;
                    next_count += 1;
                }
            }
            parts = next;
            count = next_count;
        }
        for &part in &parts[..count] {
            self.add_scaled(part, scale);
        }
    }

    /// Adds the exact product of the factors (at most three).
    pub fn add_product(&mut self, factors: &[f64]) {
        self.add_product_scaled(factors, 0);
    }

    /// Adds d·x·… for a double-double d (its two words) and at most two
    /// binary64 factors, exactly.
    pub fn add_dd_product(&mut self, d: Dd, factors: &[f64]) {
        let mut hi = vec![d.hi];
        hi.extend_from_slice(factors);
        self.add_product(&hi);
        if d.lo != 0.0 {
            let mut lo = vec![d.lo];
            lo.extend_from_slice(factors);
            self.add_product(&lo);
        }
    }

    /// Adds Σ terms × the factors (at most two), exactly.
    pub fn add_terms_product(&mut self, terms: &[Term], factors: &[f64]) {
        for &(value, exponent) in terms {
            let mut all = vec![value];
            all.extend_from_slice(factors);
            self.add_product_scaled(&all, exponent);
        }
    }

    /// Propagates carries so that every limb but the top one is in
    /// [0, 2³²) and the top one is small and carries the sign.
    fn normalize(&mut self) {
        self.pending = 0;
        let mut k = 0;
        while k + 1 < self.limbs.len() {
            let carry = self.limbs[k] >> 32;
            self.limbs[k] -= carry << 32;
            self.limbs[k + 1] += carry;
            k += 1;
            if k + 1 == self.limbs.len() && self.limbs[k].abs() >= 1 << 31 {
                self.limbs.push(0);
            }
        }
    }

    /// The exact value as a sign and normalized nonnegative limbs (none for
    /// zero).
    fn magnitude(&self) -> (bool, Vec<i64>, i32) {
        let mut copy = self.clone();
        copy.normalize();
        let negative = copy.limbs.last().is_some_and(|&top| top < 0);
        if negative {
            for limb in &mut copy.limbs {
                *limb = -*limb;
            }
            copy.normalize();
        }
        while copy.limbs.last() == Some(&0) {
            copy.limbs.pop();
        }
        (negative, copy.limbs, copy.low)
    }

    /// The exact value as nonoverlapping binary64 terms (each nonzero 32-bit
    /// limb), for multiplying an exact sum by further factors.
    pub fn terms(&self) -> Vec<Term> {
        assert!(self.is_finite(), "terms of a non-finite sum");
        let (negative, limbs, low) = self.magnitude();
        let sign = if negative { -1.0 } else { 1.0 };
        limbs
            .iter()
            .enumerate()
            .filter(|(_, &limb)| limb != 0)
            .map(|(k, &limb)| (sign * limb as f64, 32 * (low + k as i32)))
            .collect()
    }

    /// The sum rounded once: the high word is the correctly rounded binary64
    /// significand, the low word the remainder rounded to odd (module docs).
    pub fn to_wide(&self) -> WideDd {
        if let Some(value) = self.non_finite {
            return WideDd::from_f64(value);
        }
        let (negative, limbs, low) = self.magnitude();
        let Some(top) = limbs.len().checked_sub(1) else {
            return WideDd::ZERO;
        };
        // The top four limbs as a 128-bit window, the top limb at its top;
        // anything below is a sticky bit.
        let mut window: u128 = 0;
        for j in 0..4 {
            let limb = top.checked_sub(j).map_or(0, |k| limbs[k]) as u128;
            window |= limb << (32 * (3 - j));
        }
        let mut sticky = top >= 4 && limbs[..top - 3].iter().any(|&limb| limb != 0);
        let mut lsb = 32 * (low + top as i32 - 3);
        while 128 - window.leading_zeros() > 126 {
            sticky |= window & 1 == 1;
            window >>= 1;
            lsb += 1;
        }
        let length = 128 - window.leading_zeros() as i32;
        let shift = length - 53;
        let half = 1u128 << (shift - 1);
        let mut q = window >> shift;
        let r = window & ((1u128 << shift) - 1);
        if r > half || (r == half && (sticky || q & 1 == 1)) {
            q += 1;
        }
        // The remainder x − hi in units of 2^lsb, |R| ≤ half, plus the
        // sticky part (positive, below one unit).
        let remainder = window as i128 - ((q << shift) as i128);
        let lo_units = round_to_odd(remainder, sticky);
        let sign = if negative { -1.0 } else { 1.0 };
        let mantissa = Dd {
            hi: sign * q as f64,
            lo: sign * scale_by_pow2(lo_units, -shift),
        };
        WideDd::from_dd_scaled(mantissa, lsb + shift)
    }

    pub fn to_f64(&self) -> f64 {
        self.to_wide().to_f64()
    }
}

/// R + s·ε rounded to odd at 53 bits (truncation toward zero, the last
/// kept bit set when inexact), for R an integer with |R| < 2⁷³ and
/// 0 < ε < 1 when `sticky`. When the kept bits reach below the units, an
/// inexact value is represented by its integer part plus ½, which lies
/// strictly between the neighbouring integers like the true value. The
/// result has the true value's sign, lies within one unit of it, and is a
/// power of two above ½ only when exact.
fn round_to_odd(remainder: i128, sticky: bool) -> f64 {
    let negative = remainder < 0;
    let magnitude = remainder.unsigned_abs();
    // The true magnitude's integer part and whether a fraction remains: a
    // sticky part adds to a positive R and takes from a negative one.
    let (integer, inexact) = match (negative, sticky) {
        (true, true) => (magnitude - 1, true),
        (_, sticky) => (magnitude, sticky),
    };
    if integer == 0 && !inexact {
        return 0.0;
    }
    let length = 128 - integer.leading_zeros() as i32;
    let value = if length >= 53 {
        let k = length - 53;
        let mut t = integer >> k;
        if inexact || integer & ((1u128 << k) - 1) != 0 {
            t |= 1;
        }
        scale_by_pow2(t as f64, k)
    } else if inexact {
        // integer < 2⁵², so integer + ½ is exact.
        integer as f64 + 0.5
    } else {
        integer as f64
    };
    if negative {
        -value
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(e: i32) -> f64 {
        scale_by_pow2(1.0, e)
    }

    /// Review round 4, item 2: 1 + 2⁻¹⁰⁰¹ − 1 is 2⁻¹⁰⁰¹, though 2⁻¹⁰⁰¹ lies more
    /// than 2¹⁰⁰⁰ below 1 (WideDd's alignment dropped it), and
    /// 1 + 2⁻⁵⁰⁰ + 2⁻⁷⁰⁰ − 1 − 2⁻⁵⁰⁰ is 2⁻⁷⁰⁰ (a double-double rounds 2⁻⁷⁰⁰
    /// away).
    #[test]
    fn cancellation_exposes_no_dropped_contribution() {
        let mut sum = ExactSum::new();
        for x in [1.0, p(-1001), -1.0] {
            sum.add_f64(x);
        }
        assert_eq!(sum.to_wide().to_f64(), p(-1001));
        let mut sum = ExactSum::new();
        for x in [1.0, p(-500), p(-700), -1.0, -p(-500)] {
            sum.add_f64(x);
        }
        assert_eq!(sum.to_f64(), p(-700));
        // Far beyond the binary64 range, through scaled terms.
        let mut sum = ExactSum::new();
        sum.add_scaled(1.0, 3000);
        sum.add_scaled(3.0, -3000);
        sum.add_scaled(-1.0, 3000);
        let wide = sum.to_wide();
        assert_eq!(wide.exponent, -3000 + 1);
        assert_eq!(wide.mantissa, Dd::from_f64(1.5));
    }

    #[test]
    fn products_are_expanded_exactly() {
        // (1 + 2⁻⁵²)³ = 1 + 3·2⁻⁵² + 3·2⁻¹⁰⁴ + 2⁻¹⁵⁶; minus its leading terms.
        let a = 1.0 + p(-52);
        let mut sum = ExactSum::new();
        sum.add_product(&[a, a, a]);
        sum.add_f64(-1.0);
        sum.add_f64(-3.0 * p(-52));
        sum.add_f64(-3.0 * p(-104));
        assert_eq!(sum.to_f64(), p(-156));
        // Factors whose product is far below binary64: 2⁻⁷⁰⁰·2⁻³⁰⁰·2⁻³⁰⁰.
        let mut sum = ExactSum::new();
        sum.add_product(&[p(-700), p(-300), -p(-300)]);
        let wide = sum.to_wide();
        assert_eq!((wide.mantissa, wide.exponent), (Dd::from_f64(-1.0), -1300));
        // A double-double times a factor.
        let mut sum = ExactSum::new();
        sum.add_dd_product(Dd { hi: 1.0, lo: p(-80) }, &[3.0]);
        sum.add_f64(-3.0);
        assert_eq!(sum.to_f64(), 3.0 * p(-80));
        let mut nan = ExactSum::new();
        nan.add_f64(1.0);
        nan.add_product(&[f64::INFINITY, 0.0]);
        assert!(nan.to_f64().is_nan());
    }

    /// Random sums with cancellation: values, their negatives in another
    /// order, and a known remainder at a far exponent. Thousands of limb
    /// adds and the carry propagation are exercised; the result is exact.
    #[test]
    fn random_cancelling_sums_are_exact() {
        let mut state: u64 = 0x2545_F491_4F6C_DD1D;
        let mut next = move || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        for round in 0..200 {
            let mut values = Vec::new();
            for _ in 0..50 {
                let r = next();
                let field = 1 + (r % 2046);
                values.push(f64::from_bits((field << 52) | (next() & ((1u64 << 52) - 1))));
            }
            let mut sum = ExactSum::new();
            for &v in &values {
                sum.add_f64(v);
            }
            let remainder = p(-1074 + (round % 1000) as i32);
            sum.add_f64(remainder);
            for &v in values.iter().rev() {
                sum.add_f64(-v);
            }
            assert_eq!(sum.to_f64(), remainder, "round {round}");
            assert_eq!(sum.terms().len(), 1);
        }
    }

    /// to_wide's high word is the correctly rounded value and its low word
    /// decides the midpoints: 1 + 2⁻⁵³ (a tie, to even, 1) and
    /// 1 + 2⁻⁵³ + 2⁻²⁰⁰⁰ (just above, 1 + 2⁻⁵²), and the subnormal tie
    /// (1 + 2⁻²⁰⁰⁰)·2⁻¹⁰⁷⁵.
    #[test]
    fn rounding_sees_every_midpoint() {
        let mut tie = ExactSum::new();
        tie.add_f64(1.0);
        tie.add_f64(p(-53));
        assert_eq!(tie.to_f64(), 1.0);
        let mut above = tie.clone();
        above.add_scaled(1.0, -2000);
        assert_eq!(above.to_f64(), 1.0 + p(-52));
        let mut below = tie.clone();
        below.add_scaled(-1.0, -2000);
        assert_eq!(below.to_f64(), 1.0);
        let mut sub = ExactSum::new();
        sub.add_scaled(1.0, -1075);
        assert_eq!(sub.to_f64(), 0.0);
        sub.add_scaled(1.0, -3000);
        assert_eq!(sub.to_f64(), p(-1074));
        // 2⁻¹⁰²² − 2⁻¹⁰⁷⁵: rounds to the smallest normal, classified below.
        let mut edge = ExactSum::new();
        edge.add_f64(f64::MIN_POSITIVE);
        edge.add_scaled(-1.0, -1075);
        let wide = edge.to_wide();
        assert_eq!(wide.to_f64(), f64::MIN_POSITIVE);
        assert_eq!(wide.to_published(), Some((0.0, true)));
    }
}
