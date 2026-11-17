//! Exact dyadic numbers and outward-rounded enclosures (unit S4).
//!
//! A [`Dyadic`] is ±m·2^e with an unsigned big integer m and an i64 exponent, so every finite
//! binary64 is one exactly and sums and products of dyadics are exact: nothing overflows or
//! underflows. Quotients and square roots are not dyadic in general; they are enclosed by an
//! [`Interval`] whose ends are rounded outward (down for the lower end, up for the upper end) at
//! a working precision of p significant bits. Every enclosure produced here contains the exact
//! value it encloses; that is the only property the certificate relies on.
use crate::bigint::BigUint;
use crate::exact::decompose;
use std::cmp::Ordering;

/// ±mag·2^exp, normalized: the magnitude is odd, or the value is zero, with an empty magnitude,
/// positive sign and exponent 0. Equal values are therefore equal representations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Dyadic {
    negative: bool,
    mag: BigUint,
    exp: i64,
}

impl Dyadic {
    pub(crate) fn zero() -> Self {
        Self { negative: false, mag: BigUint::zero(), exp: 0 }
    }

    fn new(negative: bool, mag: BigUint, exp: i64) -> Self {
        if mag.is_zero() {
            return Self::zero();
        }
        let z = mag.trailing_zeros();
        if z == 0 {
            Self { negative, mag, exp }
        } else {
            Self { negative, mag: mag.shr(z), exp: exp + z as i64 }
        }
    }

    pub(crate) fn from_int(v: i64) -> Self {
        Self::new(v < 0, BigUint::from_u128(u128::from(v.unsigned_abs())), 0)
    }

    /// The exact value of a finite binary64.
    pub(crate) fn from_f64(x: f64) -> Self {
        assert!(x.is_finite(), "a finite binary64");
        if x == 0.0 {
            return Self::zero();
        }
        let (m, e) = decompose(x.abs());
        Self::new(x < 0.0, BigUint::from_u128(u128::from(m)), i64::from(e))
    }

    pub(crate) fn is_zero(&self) -> bool {
        self.mag.is_zero()
    }

    pub(crate) fn is_negative(&self) -> bool {
        self.negative
    }

    pub(crate) fn abs(&self) -> Self {
        Self::new(false, self.mag.clone(), self.exp)
    }

    pub(crate) fn neg(&self) -> Self {
        Self::new(!self.negative, self.mag.clone(), self.exp)
    }

    /// Both magnitudes over the smaller exponent.
    fn aligned(a: &Self, b: &Self) -> (BigUint, BigUint, i64) {
        let e = a.exp.min(b.exp);
        (a.mag.shl((a.exp - e) as u64), b.mag.shl((b.exp - e) as u64), e)
    }

    pub(crate) fn add(&self, other: &Self) -> Self {
        if self.is_zero() {
            return other.clone();
        }
        if other.is_zero() {
            return self.clone();
        }
        let (a, b, e) = Self::aligned(self, other);
        if self.negative == other.negative {
            Self::new(self.negative, a.add(&b), e)
        } else {
            match a.cmp(&b) {
                Ordering::Equal => Self::zero(),
                Ordering::Greater => Self::new(self.negative, a.sub(&b), e),
                Ordering::Less => Self::new(other.negative, b.sub(&a), e),
            }
        }
    }

    pub(crate) fn sub(&self, other: &Self) -> Self {
        self.add(&other.neg())
    }

    pub(crate) fn mul(&self, other: &Self) -> Self {
        Self::new(self.negative != other.negative, self.mag.mul(&other.mag), self.exp + other.exp)
    }

    /// self·2^k, exactly.
    pub(crate) fn scale2(&self, k: i64) -> Self {
        Self::new(self.negative, self.mag.clone(), self.exp + k)
    }

    pub(crate) fn mul_int(&self, k: i64) -> Self {
        self.mul(&Self::from_int(k))
    }

    pub(crate) fn cmp(&self, other: &Self) -> Ordering {
        let d = self.sub(other);
        if d.is_zero() {
            Ordering::Equal
        } else if d.negative {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }

    /// The exponent of the leading bit: 2^top ≤ |self| < 2^(top+1). `None` for zero.
    pub(crate) fn top(&self) -> Option<i64> {
        (!self.is_zero()).then(|| self.exp + self.mag.bits() as i64 - 1)
    }

    /// Rounds the signed value to at most `p` significant bits, down (`up` false) or up.
    pub(crate) fn round_outward(&self, p: u64, up: bool) -> Self {
        let bits = self.mag.bits();
        if bits <= p {
            return self.clone();
        }
        let shift = bits - p;
        let mut mag = self.mag.shr(shift);
        // Rounding the magnitude up moves a positive value up and a negative value down.
        if self.mag.any_below(shift) && (up != self.negative) {
            mag = mag.add(&BigUint::one());
        }
        Self::new(self.negative, mag, self.exp + shift as i64)
    }

    /// (negative, mantissa, exponent) when the magnitude fits 64 bits.
    pub(crate) fn small(&self) -> Option<(bool, u64, i64)> {
        (self.mag.bits() <= 64).then(|| (self.negative, self.mag.low_u64(), self.exp))
    }

    /// The binary64 nearest to the exact value (ties to even), ±∞ beyond the binary64 range.
    pub(crate) fn to_f64(&self) -> f64 {
        let Some(top) = self.top() else { return 0.0 };
        let sign = if self.negative { -1.0 } else { 1.0 };
        if top > 1023 {
            return sign * f64::INFINITY;
        }
        // The lowest bit kept: 53 bits for a normal result, the 2^-1074 grid for a subnormal one.
        let lowest = if top >= -1022 { top - 52 } else { -1074 };
        let shift = lowest - self.exp;
        let mut q = if shift > 0 {
            let q = self.mag.shr(shift as u64);
            let half = self.mag.bit(shift as u64 - 1);
            let rest = self.mag.any_below(shift as u64 - 1);
            if half && (rest || q.bit(0)) {
                q.add(&BigUint::one())
            } else {
                q
            }
        } else {
            self.mag.shl((-shift) as u64)
        };
        let mut exp = lowest;
        if q.bits() > 53 {
            q = q.shr(1);
            exp += 1;
        }
        let q = q.low_u64();
        if q == 0 {
            return sign * 0.0;
        }
        // q·2^exp with q < 2^53: normal when q ≥ 2^52 (or the grid is the normal one).
        if exp + 52 > 1023 {
            return sign * f64::INFINITY;
        }
        let bits = if q >= 1 << 52 {
            ((exp + 52 + 1023) as u64) << 52 | (q - (1 << 52))
        } else {
            debug_assert_eq!(exp, -1074);
            q
        };
        sign * f64::from_bits(bits)
    }

    /// The exact midpoint of two dyadics.
    pub(crate) fn midpoint(&self, other: &Self) -> Self {
        self.add(other).scale2(-1)
    }
}

/// An exact fixed-point accumulator of signed products m·2^e (m < 2^128) over a stated exponent
/// range, without allocation per addend: the CQC inner sums (`rules`).
pub(crate) struct ExactAccumulator {
    base: i64,
    pos: Vec<u64>,
    neg: Vec<u64>,
}

impl ExactAccumulator {
    /// Covers addends m·2^e with e ≥ `base` and m·2^e < 2^`top`, plus 64 bits of carry headroom
    /// (up to 2^64 addends).
    pub(crate) fn new(base: i64, top: i64) -> Self {
        let limbs = ((top - base).max(0) as usize) / 64 + 3;
        Self { base, pos: vec![0; limbs], neg: vec![0; limbs] }
    }

    pub(crate) fn clear(&mut self) {
        self.pos.iter_mut().for_each(|l| *l = 0);
        self.neg.iter_mut().for_each(|l| *l = 0);
    }

    pub(crate) fn add(&mut self, negative: bool, m: u128, e: i64) {
        if m == 0 {
            return;
        }
        let shift = (e - self.base) as u64;
        let (limb, bit) = ((shift / 64) as usize, shift % 64);
        let (m0, m1) = (m as u64, (m >> 64) as u64);
        let parts = if bit == 0 { [m0, m1, 0] } else { [m0 << bit, (m1 << bit) | (m0 >> (64 - bit)), m1 >> (64 - bit)] };
        let target = if negative { &mut self.neg } else { &mut self.pos };
        let mut carry = 0u64;
        let mut i = limb;
        for part in parts {
            let (s1, c1) = target[i].overflowing_add(part);
            let (s2, c2) = s1.overflowing_add(carry);
            target[i] = s2;
            carry = u64::from(c1) + u64::from(c2);
            i += 1;
        }
        while carry > 0 {
            let (s, c) = target[i].overflowing_add(carry);
            target[i] = s;
            carry = u64::from(c);
            i += 1;
        }
    }

    /// The exact sum.
    pub(crate) fn value(&self) -> Dyadic {
        let pos = BigUint::from_limbs(&self.pos);
        let neg = BigUint::from_limbs(&self.neg);
        match pos.cmp(&neg) {
            Ordering::Equal => Dyadic::zero(),
            Ordering::Greater => Dyadic::new(false, pos.sub(&neg), self.base),
            Ordering::Less => Dyadic::new(true, neg.sub(&pos), self.base),
        }
    }
}

/// A closed enclosure [lo, hi] of an exact real.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Interval {
    pub(crate) lo: Dyadic,
    pub(crate) hi: Dyadic,
}

impl Interval {
    pub(crate) fn exact(x: Dyadic) -> Self {
        Self { lo: x.clone(), hi: x }
    }

    pub(crate) fn is_exact_zero(&self) -> bool {
        self.lo.is_zero() && self.hi.is_zero()
    }

    pub(crate) fn add(&self, other: &Self) -> Self {
        Self { lo: self.lo.add(&other.lo), hi: self.hi.add(&other.hi) }
    }

    /// The enclosure times an exact dyadic.
    pub(crate) fn mul_exact(&self, k: &Dyadic) -> Self {
        let (a, b) = (self.lo.mul(k), self.hi.mul(k));
        if k.is_negative() {
            Self { lo: b, hi: a }
        } else {
            Self { lo: a, hi: b }
        }
    }

}

/// The relative half-width of a binary64-guided enclosure: 2^-50, several roundings of a
/// correctly rounded binary64 operation. The enclosure is checked exactly before it is used.
const GUIDE_WIDTH: i64 = -50;

/// [c(1 − 2^-50), c(1 + 2^-50)] around a positive dyadic candidate c.
fn guided(c: &Dyadic) -> Interval {
    let delta = c.scale2(GUIDE_WIDTH);
    Interval { lo: c.sub(&delta), hi: c.add(&delta) }
}

/// The binary64 nearest x·2^-k, for a positive x whose leading bit is at 2^k or 2^(k+1).
fn scaled_f64(x: &Dyadic, k: i64) -> f64 {
    x.scale2(-k).to_f64()
}

/// An enclosure of √x for an exact dyadic x ≥ 0 with a relative width of about 2^-49: a
/// candidate from the binary64 square root of x scaled exactly by an even power of two, widened
/// by 2^-50 and checked exactly (lo² ≤ x ≤ hi²). When the check fails, the exact integer square
/// root at 60 bits encloses it instead.
pub(crate) fn sqrt_guided(x: &Dyadic) -> Interval {
    assert!(!x.is_negative(), "the square root of a negative value");
    let Some(top) = x.top() else { return Interval::exact(Dyadic::zero()) };
    let k = top.div_euclid(2) * 2; // x·2^-k in [1, 4)
    let candidate = Dyadic::from_f64(scaled_f64(x, k).sqrt()).scale2(k / 2);
    let e = guided(&candidate);
    if e.lo.mul(&e.lo).cmp(x) != Ordering::Greater && e.hi.mul(&e.hi).cmp(x) != Ordering::Less {
        e
    } else {
        sqrt(x, 60)
    }
}

/// An enclosure of n/d for exact dyadics, d ≠ 0, with a relative width of about 2^-49: a binary64
/// candidate from both operands scaled exactly by powers of two, widened by 2^-50 and checked
/// exactly (lo·|d| ≤ |n| ≤ hi·|d|). When the check fails, binary long division at 60 bits encloses
/// it instead.
pub(crate) fn div_guided(n: &Dyadic, d: &Dyadic) -> Interval {
    assert!(!d.is_zero(), "division by zero");
    let (Some(tn), Some(td)) = (n.top(), d.top()) else { return Interval::exact(Dyadic::zero()) };
    let (na, da) = (n.abs(), d.abs());
    let candidate = Dyadic::from_f64(scaled_f64(&na, tn) / scaled_f64(&da, td)).scale2(tn - td);
    let e = guided(&candidate);
    let ok = e.lo.mul(&da).cmp(&na) != Ordering::Greater && e.hi.mul(&da).cmp(&na) != Ordering::Less;
    let e = if ok { e } else { div(&na, &da, 60) };
    if n.negative != d.negative {
        Interval { lo: e.hi.neg(), hi: e.lo.neg() }
    } else {
        e
    }
}

/// An enclosure of √x for an enclosure x of a non-negative value, binary64-guided (relative width
/// about 2^-49 at each end).
pub(crate) fn sqrt_interval_guided(x: &Interval) -> Interval {
    Interval { lo: sqrt_guided(&x.lo).lo, hi: sqrt_guided(&x.hi).hi }
}

/// An enclosure of `n / d` for exact dyadics with `d ≠ 0`, at `p` bits.
pub(crate) fn div(n: &Dyadic, d: &Dyadic, p: u64) -> Interval {
    assert!(!d.is_zero(), "division by zero");
    if n.is_zero() {
        return Interval::exact(Dyadic::zero());
    }
    let negative = n.negative != d.negative;
    // Scale the numerator so the integer quotient has at least p + 1 bits.
    let k = (p as i64 + 1 + d.mag.bits() as i64 - n.mag.bits() as i64).max(0);
    let (q, r) = n.mag.shl(k as u64).divrem(&d.mag);
    let exp = n.exp - d.exp - k;
    let low = Dyadic::new(false, q.clone(), exp);
    let high = if r.is_zero() { low.clone() } else { Dyadic::new(false, q.add(&BigUint::one()), exp) };
    if negative {
        Interval { lo: high.neg(), hi: low.neg() }
    } else {
        Interval { lo: low, hi: high }
    }
}

/// An enclosure of √x for an exact dyadic x ≥ 0, at `p` bits.
pub(crate) fn sqrt(x: &Dyadic, p: u64) -> Interval {
    assert!(!x.is_negative(), "the square root of a negative value");
    if x.is_zero() {
        return Interval::exact(Dyadic::zero());
    }
    // m·2^e with e even and m of at least 2p + 2 bits: √x = √m·2^(e/2).
    let mut k = (2 * p as i64 + 2 - x.mag.bits() as i64).max(0);
    if (x.exp - k).rem_euclid(2) != 0 {
        k += 1;
    }
    let m = x.mag.shl(k as u64);
    let e = (x.exp - k) / 2;
    let s = m.isqrt();
    let exact = s.mul(&s) == m;
    let low = Dyadic::new(false, s.clone(), e);
    let high = if exact { low.clone() } else { Dyadic::new(false, s.add(&BigUint::one()), e) };
    Interval { lo: low, hi: high }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(x: f64) -> Dyadic {
        Dyadic::from_f64(x)
    }

    #[test]
    fn binary64_round_trips_exactly() {
        for x in [0.0, 1.0, -1.0, 0.1, -0.3, 1e300, -1e-300, f64::MAX, f64::MIN_POSITIVE, f64::from_bits(1), 5e-324, 2.5e-310] {
            assert_eq!(d(x).to_f64().to_bits(), x.to_bits(), "{x}");
        }
    }

    #[test]
    fn sums_are_exact_and_rounded_once() {
        let s = d(1e16).add(&d(1.0)).add(&d(-1e16));
        assert_eq!(s.to_f64(), 1.0);
        let s = d(1e308).add(&d(1e308)).add(&d(-1e308));
        assert_eq!(s.to_f64(), 1e308);
        assert_eq!(d(1e308).add(&d(1e308)).to_f64(), f64::INFINITY);
        // 1 + 2^-53 is a tie and rounds to even (1); 1 + 2^-53 + 2^-80 rounds up.
        let tie = d(1.0).add(&d(2f64.powi(-53)));
        assert_eq!(tie.to_f64(), 1.0);
        assert_eq!(tie.add(&d(2f64.powi(-80))).to_f64(), 1.0 + f64::EPSILON);
        // Half the smallest subnormal is a tie with zero and rounds to even (0); 1.5 quanta
        // rounds to 2.
        assert_eq!(d(5e-324).scale2(-1).to_f64(), 0.0);
        assert_eq!(d(5e-324).mul_int(3).scale2(-1).to_f64().to_bits(), 2);
    }

    #[test]
    fn guided_enclosures_contain_the_value_across_the_range() {
        for x in [2.0, 3.0, 1e-300, 1e300, 5e-324, f64::MAX, 0.1] {
            let v = d(x).mul(&d(x)).add(&d(x));
            let r = sqrt_guided(&v);
            assert!(r.lo.mul(&r.lo).cmp(&v) != Ordering::Greater && r.hi.mul(&r.hi).cmp(&v) != Ordering::Less, "{x}");
            let q = div_guided(&v, &d(-3.0));
            assert!(q.lo.mul_int(-3).cmp(&v) != Ordering::Less && q.hi.mul_int(-3).cmp(&v) != Ordering::Greater, "{x}");
        }
        // A perfect square's enclosure is centred on its exact root.
        let s = sqrt_guided(&d(0.75).mul(&d(0.75)));
        assert_eq!(s.lo.midpoint(&s.hi).to_f64(), 0.75);
        // Far beyond binary64: 2^5000.
        let big = Dyadic::from_int(1).scale2(5000);
        assert_eq!(sqrt_guided(&big).lo.midpoint(&sqrt_guided(&big).hi), Dyadic::from_int(1).scale2(2500));
    }

    #[test]
    fn the_accumulator_is_exact() {
        let mut acc = ExactAccumulator::new(-1200, 1200);
        acc.add(false, u128::MAX, 1000);
        acc.add(true, u128::MAX, 1000);
        acc.add(false, 3, -1200);
        acc.add(true, 1, -1199);
        assert_eq!(acc.value(), Dyadic::from_int(1).scale2(-1200));
        acc.clear();
        acc.add(false, 1 << 100, 0);
        acc.add(false, 1 << 100, 0);
        assert_eq!(acc.value(), Dyadic::from_int(1).scale2(101));
    }

    #[test]
    fn quotients_and_roots_are_enclosed() {
        let third = div(&Dyadic::from_int(1), &Dyadic::from_int(3), 60);
        assert!(third.lo.mul_int(3).cmp(&Dyadic::from_int(1)) == Ordering::Less);
        assert!(third.hi.mul_int(3).cmp(&Dyadic::from_int(1)) == Ordering::Greater);
        let exact = div(&Dyadic::from_int(6), &Dyadic::from_int(3), 60);
        assert_eq!(exact, Interval::exact(Dyadic::from_int(2)));
        let root2 = sqrt(&Dyadic::from_int(2), 80);
        assert!(root2.lo.mul(&root2.lo).cmp(&Dyadic::from_int(2)) == Ordering::Less);
        assert!(root2.hi.mul(&root2.hi).cmp(&Dyadic::from_int(2)) == Ordering::Greater);
        assert_eq!(sqrt(&d(f64::from_bits(1)), 60), Interval::exact(Dyadic::from_int(1).scale2(-537)));
        assert_eq!(sqrt(&Dyadic::from_int(9).scale2(-3), 60).lo.to_f64(), (9.0f64 / 8.0).sqrt());
        let r = div(&Dyadic::from_int(-1), &Dyadic::from_int(3), 60);
        assert!(r.lo.cmp(&r.hi) == Ordering::Less && r.hi.is_negative());
    }
}
