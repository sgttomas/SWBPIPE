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
//!
//! **Storage** (design §5.3.8, Agent 0's option (c)). A sum's limbs grow
//! only through [`reserve_limbs`], which requests max(2·capacity, required,
//! 4) elements (capped at an allotment when one is given) with
//! `Vec::reserve_exact`, so a limb vector's requested capacity is at most
//! max(4, 2·L − 2) limbs for L the largest length it has held, and at most
//! the allotment under one. [`ExactSum::reset`] returns a sum to the fresh
//! state (no limbs, `low`, `pending` and the non-finite part cleared) and
//! keeps its capacity, so a workspace reuses one accumulator across pairs or
//! products. Reading a sum ([`ExactSum::to_wide_with`],
//! [`ExactSum::terms_into`]) normalizes a copy of its limbs in a caller's
//! scratch vector instead of a fresh clone. None of this changes a value:
//! the limbs hold the same integers in the same order at every step.
//!
//! **Allotment hook.** [`ExactSum::try_add_scaled`] and
//! [`ExactSum::try_add_f64`] take a limb allotment and refuse
//! ([`LimbAllotmentExceeded`]) before any growth past it; the formation maps
//! use them (`reformation`). No allotment value is set here.

use crate::dd::{binary_exponent, scale_by_pow2, two_prod, Dd, WideDd};

/// Carries are propagated after this many adds: each add changes a limb by
/// less than 2³², so the limbs stay below 2⁶⁰ in magnitude.
const NORMALIZE_EVERY: u32 = 1 << 28;

/// A limb growth refused before its allocation: the sum would need
/// `required` limbs, more than `allotment`. The sum that refused is to be
/// discarded or [`ExactSum::reset`]; its value is unspecified.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LimbAllotmentExceeded {
    pub required: usize,
    pub allotment: usize,
}

/// Makes room for `required` elements in `v` (module docs, "Storage"): if
/// the capacity is short, requests exactly max(2·capacity, required, 4),
/// capped at `allotment`; refuses before any allocation when `required`
/// exceeds `allotment`.
pub(crate) fn reserve_limbs<T>(
    v: &mut Vec<T>,
    required: usize,
    allotment: Option<usize>,
) -> Result<(), LimbAllotmentExceeded> {
    if let Some(allotment) = allotment {
        if required > allotment {
            return Err(LimbAllotmentExceeded {
                required,
                allotment,
            });
        }
    }
    let capacity = v.capacity();
    if required > capacity {
        let mut target = required.max(capacity.saturating_mul(2)).max(4);
        if let Some(allotment) = allotment {
            target = target.min(allotment);
        }
        v.reserve_exact(target - v.len());
    }
    Ok(())
}

/// Propagates carries so that every limb but the top one is in [0, 2³²)
/// and the top one is small and carries the sign. A top limb of magnitude
/// at least 2³¹ gets one more limb (at most once a pass); with an allotment
/// that growth is checked first, and on a refusal the carries made so far
/// are kept (the integer value is unchanged).
fn carry_limbs(
    limbs: &mut Vec<i64>,
    allotment: Option<usize>,
) -> Result<(), LimbAllotmentExceeded> {
    let mut k = 0;
    while k + 1 < limbs.len() {
        let carry = limbs[k] >> 32;
        limbs[k] -= carry << 32;
        limbs[k + 1] += carry;
        k += 1;
        if k + 1 == limbs.len() && limbs[k].abs() >= 1 << 31 {
            reserve_limbs(limbs, limbs.len() + 1, allotment)?;
            limbs.push(0);
        }
    }
    Ok(())
}

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

    /// Returns the sum to the fresh state of [`ExactSum::new`] (every field:
    /// no limbs, `low` 0, no pending adds, no non-finite part), keeping the
    /// limbs' capacity for reuse.
    pub fn reset(&mut self) {
        self.limbs.clear();
        self.low = 0;
        self.pending = 0;
        self.non_finite = None;
    }

    /// The limb vector's length and requested capacity (resource ledger).
    pub fn limb_storage(&self) -> (usize, usize) {
        (self.limbs.len(), self.limbs.capacity())
    }

    pub fn is_finite(&self) -> bool {
        self.non_finite.is_none()
    }

    /// Adds `value·2^exponent` exactly.
    pub fn add_scaled(&mut self, value: f64, exponent: i32) {
        // Without an allotment no growth refuses.
        let _ = self.add_scaled_within(value, exponent, None);
    }

    /// [`ExactSum::add_scaled`] under a limb allotment: refuses before any
    /// growth of the limbs past `allotment` limbs.
    pub fn try_add_scaled(
        &mut self,
        value: f64,
        exponent: i32,
        allotment: usize,
    ) -> Result<(), LimbAllotmentExceeded> {
        self.add_scaled_within(value, exponent, Some(allotment))
    }

    /// [`ExactSum::add_f64`] under a limb allotment.
    pub fn try_add_f64(
        &mut self,
        value: f64,
        allotment: usize,
    ) -> Result<(), LimbAllotmentExceeded> {
        self.add_scaled_within(value, 0, Some(allotment))
    }

    pub(crate) fn add_scaled_within(
        &mut self,
        value: f64,
        exponent: i32,
        allotment: Option<usize>,
    ) -> Result<(), LimbAllotmentExceeded> {
        if value == 0.0 {
            return Ok(());
        }
        if !value.is_finite() {
            self.non_finite = Some(self.non_finite.map_or(value, |v| v + value));
            return Ok(());
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
        // The limbs' growth, checked and reserved before any change: four
        // limbs for the first add; `grow` more below for a lower part (whose
        // index is then 0, within the four or more limbs already held); or
        // up to index + 4.
        if self.limbs.is_empty() {
            reserve_limbs(&mut self.limbs, 4, allotment)?;
            self.low = limb;
            self.limbs.resize(4, 0);
        } else if limb < self.low {
            let grow = (self.low - limb) as usize;
            let old = self.limbs.len();
            reserve_limbs(&mut self.limbs, old + grow, allotment)?;
            // As `splice(0..0, repeat_n(0, grow))`: the limbs move up by
            // `grow` and zeros fill the bottom.
            self.limbs.resize(old + grow, 0);
            self.limbs.copy_within(0..old, grow);
            self.limbs[..grow].fill(0);
            self.low = limb;
        }
        let index = (limb - self.low) as usize;
        if self.limbs.len() < index + 4 {
            reserve_limbs(&mut self.limbs, index + 4, allotment)?;
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
            self.pending = 0;
            carry_limbs(&mut self.limbs, allotment)?;
        }
        Ok(())
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

    /// R-6/R-7 (DEL-04-03 skew-restraints.md §11.3 D-1 (a)): the exact product of
    /// at most four finite factors times 2^exponent, under an optional limb
    /// allotment: q_ir·k·q_jc·x of a congruence term. Each factor is split into
    /// its exponent and a significand in [1, 2), so every TwoProduct is exact
    /// and no part underflows (no split-range premise is needed here). A
    /// separate entry, so the three-factor path is unchanged.
    pub(crate) fn add_product4_within(
        &mut self,
        factors: &[f64],
        exponent: i32,
        allotment: Option<usize>,
    ) -> Result<(), LimbAllotmentExceeded> {
        debug_assert!(!factors.is_empty() && factors.len() <= 4);
        if factors.contains(&0.0) && factors.iter().all(|f| f.is_finite()) {
            return Ok(());
        }
        if factors.iter().any(|f| !f.is_finite()) {
            let product = factors.iter().product::<f64>();
            self.non_finite = Some(self.non_finite.map_or(product, |v| v + product));
            return Ok(());
        }
        let mut scale = exponent;
        let mut parts: [f64; 8] = [0.0; 8];
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
            let mut next = [0.0; 8];
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
            self.add_scaled_within(part, scale, allotment)?;
        }
        Ok(())
    }

    /// Adds the exact product of the factors (at most three).
    pub fn add_product(&mut self, factors: &[f64]) {
        self.add_product_scaled(factors, 0);
    }

    /// Adds d·x·… for a double-double d (its two words) and at most two
    /// binary64 factors, exactly. The factor list is built on the stack.
    pub fn add_dd_product(&mut self, d: Dd, factors: &[f64]) {
        let count = factors.len() + 1;
        let mut all = [0.0; 3];
        all[0] = d.hi;
        all[1..count].copy_from_slice(factors);
        self.add_product(&all[..count]);
        if d.lo != 0.0 {
            all[0] = d.lo;
            self.add_product(&all[..count]);
        }
    }

    /// Adds Σ terms × the factors (at most two), exactly. The factor list is
    /// built on the stack.
    pub fn add_terms_product(&mut self, terms: &[Term], factors: &[f64]) {
        let count = factors.len() + 1;
        let mut all = [0.0; 3];
        all[1..count].copy_from_slice(factors);
        for &(value, exponent) in terms {
            all[0] = value;
            self.add_product_scaled(&all[..count], exponent);
        }
    }

    /// The exact value as a sign and normalized nonnegative limbs (none for
    /// zero), the limbs copied into `scratch` (its contents replaced, its
    /// capacity reused) and normalized there; returns the sign and `low`.
    fn magnitude_into(&self, scratch: &mut Vec<i64>) -> (bool, i32) {
        scratch.clear();
        // The copy and the one limb a normalization may add.
        let _ = reserve_limbs(scratch, self.limbs.len() + 1, None);
        scratch.extend_from_slice(&self.limbs);
        let _ = carry_limbs(scratch, None);
        let negative = scratch.last().is_some_and(|&top| top < 0);
        if negative {
            for limb in scratch.iter_mut() {
                *limb = -*limb;
            }
            let _ = carry_limbs(scratch, None);
        }
        while scratch.last() == Some(&0) {
            scratch.pop();
        }
        (negative, self.low)
    }

    /// The exact value as nonoverlapping binary64 terms (each nonzero 32-bit
    /// limb), for multiplying an exact sum by further factors.
    pub fn terms(&self) -> Vec<Term> {
        self.terms_with(&mut Vec::new())
    }

    /// [`ExactSum::terms`] with `scratch` for the normalized copy; the
    /// returned vector's capacity is exactly its length.
    pub(crate) fn terms_with(&self, scratch: &mut Vec<i64>) -> Vec<Term> {
        assert!(self.is_finite(), "terms of a non-finite sum");
        let (negative, low) = self.magnitude_into(scratch);
        let count = scratch.iter().filter(|&&limb| limb != 0).count();
        let mut out = Vec::with_capacity(count);
        push_terms(negative, low, scratch, &mut out);
        out
    }

    /// [`ExactSum::terms`] into `out` (its contents replaced, its capacity
    /// reused), with `scratch` for the normalized copy.
    pub(crate) fn terms_into(&self, scratch: &mut Vec<i64>, out: &mut Vec<Term>) {
        assert!(self.is_finite(), "terms of a non-finite sum");
        let (negative, low) = self.magnitude_into(scratch);
        out.clear();
        let count = scratch.iter().filter(|&&limb| limb != 0).count();
        let _ = reserve_limbs(out, count, None);
        push_terms(negative, low, scratch, out);
    }

    /// The sum rounded once: the high word is the correctly rounded binary64
    /// significand, the low word the remainder rounded to odd (module docs).
    pub fn to_wide(&self) -> WideDd {
        self.to_wide_with(&mut Vec::new())
    }

    /// [`ExactSum::to_wide`] with `scratch` for the normalized copy.
    pub(crate) fn to_wide_with(&self, scratch: &mut Vec<i64>) -> WideDd {
        if let Some(value) = self.non_finite {
            return WideDd::from_f64(value);
        }
        let (negative, low) = self.magnitude_into(scratch);
        let limbs: &[i64] = scratch;
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

/// One term per nonzero normalized limb, ascending.
fn push_terms(negative: bool, low: i32, limbs: &[i64], out: &mut Vec<Term>) {
    let sign = if negative { -1.0 } else { 1.0 };
    out.extend(
        limbs
            .iter()
            .enumerate()
            .filter(|(_, &limb)| limb != 0)
            .map(|(k, &limb)| (sign * limb as f64, 32 * (low + k as i32))),
    );
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

    /// The operations every reuse test applies after a reset: a wide span
    /// (lower parts, so the limbs grow below), products, dd and term
    /// products, and a cancellation.
    fn series(sum: &mut ExactSum) {
        sum.add_scaled(1.5, 700);
        sum.add_product(&[p(-300), 3.0, -p(-400)]);
        sum.add_dd_product(Dd { hi: 1.0, lo: p(-70) }, &[-7.0, p(-900)]);
        sum.add_terms_product(&[(5.0, -1200), (-3.0, 40)], &[p(10)]);
        sum.add_scaled(-1.5, 700);
        sum.add_f64(p(-1074));
    }

    /// Option (c)'s reset (design §5.3.8): a sum after a non-trivial use (a
    /// span of about 190 limbs, a negative top, pending adds and a
    /// non-finite part) is reset to exactly the fresh state, keeps its
    /// capacity, and then gives exactly the fresh result.
    #[test]
    fn a_reset_sum_gives_exactly_the_fresh_result() {
        let mut used = ExactSum::new();
        used.add_scaled(1.0, 3000);
        used.add_scaled(3.0, -3000);
        used.add_f64(-p(1000));
        used.add_product(&[-p(900), p(900), 3.0]);
        used.add_f64(f64::INFINITY);
        assert!(used.pending > 0 && !used.is_finite());
        let (length, capacity) = used.limb_storage();
        assert!(length >= 180, "{length}");
        used.reset();
        assert_eq!(used, ExactSum::new());
        assert_eq!(used.limb_storage(), (0, capacity));
        let mut fresh = ExactSum::new();
        series(&mut used);
        series(&mut fresh);
        assert_eq!(used, fresh);
        let mut scratch = vec![-1; 7];
        let wide = used.to_wide_with(&mut scratch);
        assert_eq!(wide, fresh.to_wide());
        assert_eq!(wide.mantissa.hi.to_bits(), fresh.to_wide().mantissa.hi.to_bits());
        assert_eq!(wide.mantissa.lo.to_bits(), fresh.to_wide().mantissa.lo.to_bits());
        let mut terms = vec![(9.0, 9); 3];
        used.terms_into(&mut scratch, &mut terms);
        assert_eq!(terms, fresh.terms());
        assert_eq!(used.terms_with(&mut scratch), fresh.terms());
        // A negative sum read with a scratch vector that held a longer one.
        let mut negative = ExactSum::new();
        negative.add_f64(-3.0);
        negative.add_scaled(-1.0, -2000);
        let mut long = vec![i64::MAX; 300];
        assert_eq!(negative.to_wide_with(&mut long), negative.to_wide());
        assert_eq!(negative.terms_with(&mut long), negative.terms());
    }

    /// The stated capacity policy: a limb vector's requested capacity is at
    /// most max(4, 2·L − 2) for L the largest length it held, and a read's
    /// scratch at most max(4, 2·(L + 1) − 2).
    #[test]
    fn limb_capacity_follows_the_stated_policy() {
        let mut state: u64 = 0x2545_F491_4F6C_DD1D;
        let mut next = move || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        let mut sum = ExactSum::new();
        let mut scratch = Vec::new();
        let mut longest = 0;
        for round in 0..2000 {
            if round % 400 == 0 {
                sum.reset();
            }
            let value = f64::from_bits((1 + next() % 2045) << 52 | (next() & ((1 << 52) - 1)));
            let exponent = (next() % 6001) as i32 - 3000;
            sum.add_scaled(value, exponent);
            let (length, capacity) = sum.limb_storage();
            longest = longest.max(length);
            assert!(capacity <= 4.max(2 * longest - 2), "{length} {capacity}");
            let _ = sum.to_wide_with(&mut scratch);
            assert!(scratch.capacity() <= 4.max(2 * (longest + 1) - 2));
        }
        assert!(longest > 150, "{longest}");
    }

    /// The formation-map hook: an allotment refuses before the growth that
    /// would exceed it (no allocation, the value unchanged), and the
    /// capacity never exceeds an allotment that holds.
    #[test]
    fn a_limb_allotment_refuses_before_growth() {
        let mut sum = ExactSum::new();
        assert_eq!(sum.try_add_f64(1.0, 3), Err(LimbAllotmentExceeded { required: 4, allotment: 3 }));
        assert_eq!(sum.limb_storage(), (0, 0));
        sum.try_add_f64(1.0, 4).unwrap();
        assert_eq!(sum.limb_storage(), (4, 4));
        // 1 occupies limbs −2 to 1; 2⁻²⁰⁰ starts in limb −8: 10 are needed.
        assert_eq!(
            sum.try_add_scaled(1.0, -200, 9),
            Err(LimbAllotmentExceeded { required: 10, allotment: 9 })
        );
        assert_eq!(sum.limb_storage(), (4, 4));
        assert_eq!(sum.to_f64(), 1.0);
        // Within the allotment, the doubled request is capped at it.
        sum.try_add_scaled(1.0, -200, 10).unwrap();
        assert_eq!(sum.limb_storage(), (10, 10));
        let mut unguarded = ExactSum::new();
        unguarded.add_f64(1.0);
        unguarded.add_scaled(1.0, -200);
        assert_eq!(sum, unguarded);
        assert_eq!(unguarded.limb_storage(), (10, 10));
        // Growth above: 2²⁰⁰ starts in limb 4, index 12, so 16 are needed.
        assert_eq!(
            sum.try_add_scaled(1.0, 200, 15),
            Err(LimbAllotmentExceeded { required: 16, allotment: 15 })
        );
        assert_eq!(sum.limb_storage(), (10, 10));
        assert_eq!(sum, unguarded);
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
