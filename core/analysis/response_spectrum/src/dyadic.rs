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
    fn top(&self) -> Option<i64> {
        (!self.is_zero()).then(|| self.exp + self.mag.bits() as i64 - 1)
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

/// An enclosure of √x for an enclosure x with 0 < x.lo (or x exactly 0), at `p` bits.
pub(crate) fn sqrt_interval(x: &Interval, p: u64) -> Interval {
    Interval { lo: sqrt(&x.lo, p).lo, hi: sqrt(&x.hi, p).hi }
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
