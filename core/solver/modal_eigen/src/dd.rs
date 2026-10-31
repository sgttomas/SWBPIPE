//! Double-double arithmetic from binary64 error-free transformations.
//!
//! A [`Dd`] is an unevaluated sum `hi + lo` of two binary64 values with
//! `|lo| <= ulp(hi) / 2`, giving about a 106-bit significand (design §3.4,
//! check 1 (ii)). The building blocks are Knuth's TwoSum and the TwoProduct
//! formed with a fused multiply-add (`f64::mul_add`, correctly rounded on every
//! platform). Every operation is a fixed sequence of correctly rounded IEEE
//! binary64 operations, so on round-to-nearest the results are bit-identical
//! across runs and platforms.
//!
//! The add, multiply, divide and square-root algorithms are the standard
//! double-double ones (Dekker 1971; Hida, Li and Bailey 2001). Relative error
//! bounds are a small multiple of 2⁻¹⁰⁶ for finite operands away from the
//! binary64 overflow and underflow ranges; non-finite operands are not
//! supported and give non-finite results.

/// An unevaluated sum `hi + lo` of two binary64 values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dd {
    pub hi: f64,
    pub lo: f64,
}

/// TwoSum: `s + e == a + b` exactly, with `s = fl(a + b)`.
#[inline]
pub fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let s = a + b;
    let bb = s - a;
    let e = (a - (s - bb)) + (b - bb);
    (s, e)
}

/// FastTwoSum, valid when `|a| >= |b|` or `a == 0`.
#[inline]
fn quick_two_sum(a: f64, b: f64) -> (f64, f64) {
    let s = a + b;
    let e = b - (s - a);
    (s, e)
}

/// TwoProduct: `p + e == a * b` exactly, with `p = fl(a * b)`, barring
/// underflow of `e`.
#[inline]
pub fn two_prod(a: f64, b: f64) -> (f64, f64) {
    let p = a * b;
    let e = a.mul_add(b, -p);
    (p, e)
}

impl Dd {
    pub const ZERO: Dd = Dd { hi: 0.0, lo: 0.0 };

    pub fn from_f64(value: f64) -> Dd {
        Dd { hi: value, lo: 0.0 }
    }

    /// The correctly rounded binary64 value of `hi + lo`: a single IEEE
    /// addition rounds the exact sum once.
    pub fn to_f64(self) -> f64 {
        self.hi + self.lo
    }

    /// The exact product of two binary64 values.
    pub fn product(a: f64, b: f64) -> Dd {
        let (hi, lo) = two_prod(a, b);
        Dd { hi, lo }
    }

    pub fn add(self, other: Dd) -> Dd {
        let (s1, s2) = two_sum(self.hi, other.hi);
        let (t1, t2) = two_sum(self.lo, other.lo);
        let (s1, s2) = quick_two_sum(s1, s2 + t1);
        let (hi, lo) = quick_two_sum(s1, s2 + t2);
        Dd { hi, lo }
    }

    pub fn add_f64(self, b: f64) -> Dd {
        let (s1, s2) = two_sum(self.hi, b);
        let (hi, lo) = quick_two_sum(s1, s2 + self.lo);
        Dd { hi, lo }
    }

    pub fn neg(self) -> Dd {
        Dd {
            hi: -self.hi,
            lo: -self.lo,
        }
    }

    pub fn sub(self, other: Dd) -> Dd {
        self.add(other.neg())
    }

    pub fn abs(self) -> Dd {
        if self.hi < 0.0 || (self.hi == 0.0 && self.lo < 0.0) {
            self.neg()
        } else {
            self
        }
    }

    pub fn mul(self, other: Dd) -> Dd {
        let (p1, p2) = two_prod(self.hi, other.hi);
        let p2 = p2 + (self.hi * other.lo + self.lo * other.hi);
        let (hi, lo) = quick_two_sum(p1, p2);
        Dd { hi, lo }
    }

    pub fn mul_f64(self, b: f64) -> Dd {
        let (p1, p2) = two_prod(self.hi, b);
        let (hi, lo) = quick_two_sum(p1, p2 + self.lo * b);
        Dd { hi, lo }
    }

    /// Long division with three binary64 quotient digits.
    pub fn div(self, other: Dd) -> Dd {
        let q1 = self.hi / other.hi;
        let r = self.sub(other.mul_f64(q1));
        let q2 = r.hi / other.hi;
        let r = r.sub(other.mul_f64(q2));
        let q3 = r.hi / other.hi;
        let (q1, q2) = quick_two_sum(q1, q2);
        Dd { hi: q1, lo: q2 }.add_f64(q3)
    }

    /// Square root by one Newton step from the binary64 root. Zero gives
    /// zero; a negative value gives NaN.
    pub fn sqrt(self) -> Dd {
        if self.hi == 0.0 {
            return Dd::ZERO;
        }
        if self.hi < 0.0 {
            return Dd {
                hi: f64::NAN,
                lo: f64::NAN,
            };
        }
        let x = self.hi.sqrt();
        let r = self.sub(Dd::product(x, x));
        let (hi, lo) = quick_two_sum(x, r.hi / (2.0 * x));
        Dd { hi, lo }
    }
}

impl Dd {
    /// π in double-double: hi = fl(π) and lo = fl(π − hi).
    pub fn pi() -> Dd {
        Dd {
            hi: f64::from_bits(0x4009_21fb_5444_2d18),
            lo: f64::from_bits(0x3ca1_a626_3314_5c07),
        }
    }

    /// The value times 2^k, exact barring overflow and underflow.
    pub fn mul_pow2(self, k: i32) -> Dd {
        let scale = 2f64.powi(k);
        Dd {
            hi: self.hi * scale,
            lo: self.lo * scale,
        }
    }

    /// Σ terms in the order given, each added in double-double: exact when
    /// the terms are nonoverlapping and span at most 106 bits.
    pub fn from_terms(terms: &[f64]) -> Dd {
        terms.iter().fold(Dd::ZERO, |sum, &t| sum.add_f64(t))
    }

    pub fn is_zero(self) -> bool {
        self.hi == 0.0 && self.lo == 0.0
    }
}

/// The largest series term kept is above 2^-110 of the reduced argument.
const ATAN_SERIES_STOP: f64 = 7.703719777548943e-34; // 2^-110

/// atan t for 0 ≤ t ≤ 1, in double-double. While t > 1/8 the half-angle
/// step t ← t/(1 + √(1 + t²)) is applied (at most three times on [0, 1]),
/// then the series t − t³/3 + t⁵/5 − … is summed until a term falls below
/// 2⁻¹¹⁰·t (at most 19 terms for t ≤ 1/8), and the result is scaled back by
/// 2^k. Each step is a few double-double roundings, so the relative error is
/// a small multiple of 2⁻¹⁰⁶.
pub fn atan_unit(t: Dd) -> Dd {
    assert!(
        t.hi >= 0.0 && t.hi <= 1.0,
        "atan_unit argument outside [0, 1]"
    );
    let one = Dd::from_f64(1.0);
    let mut t = t;
    let mut halvings = 0;
    while t.hi > 0.125 {
        t = t.div(one.add(one.add(t.mul(t)).sqrt()));
        halvings += 1;
    }
    if t.is_zero() {
        return Dd::ZERO;
    }
    let t2 = t.mul(t);
    let mut power = t;
    let mut terms = Vec::new();
    for n in 0..40u32 {
        let term = power.div(Dd::from_f64(f64::from(2 * n + 1)));
        if n > 0 && term.hi.abs() < ATAN_SERIES_STOP * t.hi {
            break;
        }
        terms.push(if n % 2 == 0 { term } else { term.neg() });
        power = power.mul(t2);
    }
    // Smallest first.
    let sum = terms.iter().rev().fold(Dd::ZERO, |s, &x| s.add(x));
    sum.mul_pow2(halvings)
}

/// A dot-product accumulator: each binary64 product enters exactly and the
/// running sum is kept in double-double, in the order the terms are added.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DdAccumulator {
    sum: Dd,
}

impl Default for DdAccumulator {
    fn default() -> Self {
        Self::new()
    }
}

impl DdAccumulator {
    pub fn new() -> Self {
        Self { sum: Dd::ZERO }
    }

    pub fn add_product(&mut self, a: f64, b: f64) {
        self.sum = self.sum.add(Dd::product(a, b));
    }

    pub fn add_f64(&mut self, a: f64) {
        self.sum = self.sum.add_f64(a);
    }

    pub fn add_dd(&mut self, a: Dd) {
        self.sum = self.sum.add(a);
    }

    pub fn value(&self) -> Dd {
        self.sum
    }
}

/// `Σ a_i b_i` in double-double, terms in ascending index order.
pub fn dot(a: &[f64], b: &[f64]) -> Dd {
    assert_eq!(a.len(), b.len(), "dot product operands differ in length");
    let mut acc = DdAccumulator::new();
    for (&x, &y) in a.iter().zip(b) {
        acc.add_product(x, y);
    }
    acc.value()
}

/// e with 2^e ≤ |x| < 2^(e+1), for x finite and nonzero (subnormals
/// included).
pub fn binary_exponent(x: f64) -> i32 {
    let bits = x.abs().to_bits();
    let biased = (bits >> 52) as i32;
    if biased == 0 {
        // Subnormal: x = mantissa·2⁻¹⁰⁷⁴.
        63 - bits.leading_zeros() as i32 - 1074
    } else {
        biased - 1023
    }
}

/// x·2^k, applied in steps of at most 2^±1022 that each stay normal while
/// the result does; exact when the result is normal.
pub fn scale_by_pow2(mut x: f64, mut k: i32) -> f64 {
    let pow2 = |e: i32| f64::from_bits(((e + 1023) as u64) << 52);
    while k > 1022 {
        x *= pow2(1022);
        k -= 1022;
    }
    while k < -1022 {
        x *= pow2(-1022);
        k += 1022;
    }
    x * pow2(k)
}

/// A double-double with a separate exponent: `mantissa·2^exponent`, the
/// mantissa's high word in [1, 2) in magnitude (or zero). Products,
/// quotients and sums of binary64 values keep about 106 bits whatever their
/// magnitudes, so no intermediate underflows or overflows; only the final
/// [`WideDd::to_f64`] meets the binary64 range. Used where weighted terms
/// span more than the binary64 exponent range (design §3.4, the range
/// premise of the formed quantities).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WideDd {
    pub mantissa: Dd,
    pub exponent: i32,
}

/// Exponent differences beyond this drop the smaller addend: it lies below
/// 2⁻¹⁰⁰⁰ of the larger, far under the double-double resolution.
const WIDE_ALIGNMENT_LIMIT: i32 = 1000;

impl WideDd {
    pub const ZERO: WideDd = WideDd {
        mantissa: Dd::ZERO,
        exponent: 0,
    };

    /// A non-finite value is carried with exponent 0, so it reaches
    /// [`WideDd::to_f64`] unchanged.
    fn non_finite(hi: f64) -> WideDd {
        WideDd {
            mantissa: Dd { hi, lo: hi },
            exponent: 0,
        }
    }

    pub fn is_finite(self) -> bool {
        self.mantissa.hi.is_finite() && self.mantissa.lo.is_finite()
    }

    /// Brings the high word into [1, 2) by a power of two applied with
    /// [`scale_by_pow2`] (bounded steps), so a value far below or above the
    /// binary64 range, a cancellation down to a subnormal high word
    /// included, is rescaled exactly.
    fn normalize(value: Dd, exponent: i32) -> WideDd {
        if !(value.hi.is_finite() && value.lo.is_finite()) {
            return Self::non_finite(value.hi + value.lo);
        }
        if value.hi == 0.0 {
            return Self::ZERO;
        }
        let shift = binary_exponent(value.hi);
        WideDd {
            mantissa: Dd {
                hi: scale_by_pow2(value.hi, -shift),
                lo: scale_by_pow2(value.lo, -shift),
            },
            exponent: exponent + shift,
        }
    }

    /// Exact for every finite x (a subnormal included); a non-finite x is
    /// carried to [`WideDd::to_f64`].
    pub fn from_f64(x: f64) -> WideDd {
        if !x.is_finite() {
            return Self::non_finite(x);
        }
        Self::normalize(Dd::from_f64(x), 0)
    }

    /// Exact for a finite double-double; a non-finite one is carried.
    pub fn from_dd(value: Dd) -> WideDd {
        Self::normalize(value, 0)
    }

    pub fn is_zero(self) -> bool {
        self.mantissa.is_zero()
    }

    /// Finite and greater than zero, however small.
    pub fn is_positive(self) -> bool {
        self.is_finite() && self.mantissa.hi > 0.0
    }

    pub fn mul(self, other: WideDd) -> WideDd {
        Self::normalize(
            self.mantissa.mul(other.mantissa),
            self.exponent + other.exponent,
        )
    }

    pub fn mul_f64(self, b: f64) -> WideDd {
        self.mul(Self::from_f64(b))
    }

    pub fn div(self, other: WideDd) -> WideDd {
        Self::normalize(
            self.mantissa.div(other.mantissa),
            self.exponent - other.exponent,
        )
    }

    /// A non-finite operand is propagated before any alignment, so it is
    /// never dropped as the smaller addend.
    pub fn add(self, other: WideDd) -> WideDd {
        if !(self.is_finite() && other.is_finite()) {
            return Self::non_finite(
                self.mantissa.hi + self.mantissa.lo + other.mantissa.hi + other.mantissa.lo,
            );
        }
        if other.is_zero() {
            return self;
        }
        if self.is_zero() {
            return other;
        }
        let (big, small) = if self.exponent >= other.exponent {
            (self, other)
        } else {
            (other, self)
        };
        let shift = small.exponent - big.exponent;
        if shift < -WIDE_ALIGNMENT_LIMIT {
            return big;
        }
        let aligned = Dd {
            hi: scale_by_pow2(small.mantissa.hi, shift),
            lo: scale_by_pow2(small.mantissa.lo, shift),
        };
        Self::normalize(big.mantissa.add(aligned), big.exponent)
    }

    pub fn sub(self, other: WideDd) -> WideDd {
        self.add(other.neg())
    }

    pub fn neg(self) -> WideDd {
        WideDd {
            mantissa: self.mantissa.neg(),
            exponent: self.exponent,
        }
    }

    pub fn abs(self) -> WideDd {
        WideDd {
            mantissa: self.mantissa.abs(),
            exponent: self.exponent,
        }
    }

    /// √x: the exponent is made even (the mantissa doubled when it is
    /// odd) and halved. Zero gives zero; a negative value gives NaN.
    pub fn sqrt(self) -> WideDd {
        if !self.is_finite() || self.is_zero() {
            return self;
        }
        if self.mantissa.hi < 0.0 {
            return Self::non_finite(f64::NAN);
        }
        let odd = self.exponent.rem_euclid(2);
        Self::normalize(
            self.mantissa.mul_pow2(odd).sqrt(),
            (self.exponent - odd) / 2,
        )
    }

    /// The binary64 value when it may be published (design §3.2, admission
    /// range): zero only for an exact zero, otherwise a normal number.
    /// `None` for a value that overflows, underflows to a subnormal or to
    /// zero, or is not finite.
    pub fn to_normal_or_zero(self) -> Option<f64> {
        if self.is_finite() && self.is_zero() {
            return Some(0.0);
        }
        let value = self.to_f64();
        value.is_normal().then_some(value)
    }

    /// The binary64 value, rounded once to the destination's spacing, ties
    /// to even. In the normal range the mantissa is rounded to 53 bits and
    /// scaled exactly; below it the value is rounded directly to a multiple
    /// of 2⁻¹⁰⁷⁴ (a subnormal, or zero). It is ±∞ only when the value
    /// rounds beyond the largest finite number.
    pub fn to_f64(self) -> f64 {
        if !self.is_finite() {
            return self.mantissa.hi;
        }
        if self.is_zero() {
            return 0.0;
        }
        if self.exponent >= -1022 {
            return scale_by_pow2(self.mantissa.to_f64(), self.exponent);
        }
        // N = |value|/2⁻¹⁰⁷⁴ < 2⁵², as hi + lo.
        let sign = self.mantissa.hi.signum();
        let k = self.exponent + 1074;
        if k < -2 {
            // Below a quarter of 2⁻¹⁰⁷⁴ (the mantissa is below 2).
            return sign * 0.0;
        }
        let hi = scale_by_pow2(self.mantissa.hi.abs(), k);
        let lo = scale_by_pow2(self.mantissa.lo * sign, k);
        // hi < 2⁵², so ulp(hi) ≤ ½: the fraction is a multiple of it, ½ is
        // too, and |lo| ≤ ulp(hi)/2 ≤ ¼. So lo decides only an exact half,
        // and a negative lo with a zero fraction still rounds to the floor.
        let floor = hi.floor();
        let fraction = hi - floor;
        let up = if fraction != 0.5 {
            fraction > 0.5
        } else if lo != 0.0 {
            lo > 0.0
        } else {
            floor % 2.0 == 1.0
        };
        let units = if up { floor + 1.0 } else { floor };
        sign * scale_by_pow2(units, -1074)
    }
}

/// max(a, b), NaN if either is NaN: a reduction that must not discard a
/// NaN (design §3.2, admission range).
pub fn max_propagating_nan(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else {
        a.max(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The exact value of a double-double whose components are integers
    /// below 2^127 in magnitude.
    fn exact_integer(value: Dd) -> i128 {
        assert_eq!(value.hi.fract(), 0.0);
        assert_eq!(value.lo.fract(), 0.0);
        value.hi as i128 + value.lo as i128
    }

    fn two_pow(e: i32) -> f64 {
        2.0f64.powi(e)
    }

    #[test]
    fn two_sum_is_error_free() {
        // 2^53 + 1 is not representable: fl gives 2^53 (ties to even) and the
        // error term carries the 1.
        let (s, e) = two_sum(two_pow(53), 1.0);
        assert_eq!(s, two_pow(53));
        assert_eq!(e, 1.0);
        let (s, e) = two_sum(1.0, two_pow(-60));
        assert_eq!(s, 1.0);
        assert_eq!(e, two_pow(-60));
    }

    #[test]
    fn two_prod_is_error_free() {
        // (2^27 + 1)^2 = 2^54 + 2^28 + 1 needs 55 bits.
        let a = two_pow(27) + 1.0;
        let (p, e) = two_prod(a, a);
        assert_eq!(p as i128 + e as i128, (1i128 << 54) + (1i128 << 28) + 1);
        // (2^53 - 1)(2^53 - 3) = 2^106 - 2^55 + 3.
        let x = two_pow(53) - 1.0;
        let y = two_pow(53) - 3.0;
        let product = Dd::product(x, y);
        assert_eq!(exact_integer(product), (1i128 << 106) - (1i128 << 55) + 3);
    }

    #[test]
    fn add_and_sub_keep_both_parts_exactly() {
        let big = Dd::from_f64(two_pow(80));
        let sum = big.add_f64(3.0).add_f64(two_pow(-10));
        assert_eq!(sum.hi, two_pow(80));
        assert_eq!(sum.lo, 3.0 + two_pow(-10));
        let back = sum.sub(big);
        assert_eq!(back.to_f64(), 3.0 + two_pow(-10));
        // (2^60 + 7) - (2^60 + 5) = 2, with both operands needing two words.
        let a = Dd::from_f64(two_pow(60)).add_f64(7.0);
        let b = Dd::from_f64(two_pow(60)).add_f64(5.0);
        assert_eq!(exact_integer(a), (1i128 << 60) + 7);
        assert_eq!(a.sub(b).to_f64(), 2.0);
        assert_eq!(b.sub(a).to_f64(), -2.0);
    }

    #[test]
    fn mul_carries_the_cross_terms() {
        // a = 2^60 + 1 needs two words (hi 2^60, lo 1). a(2^20 + 1) =
        // 2^80 + 2^60 + 2^20 + 1 is 81 bits, inside the 106-bit significand,
        // and the lo word enters only through the cross term, so it is exact.
        let a = Dd::from_f64(two_pow(60)).add_f64(1.0);
        assert_eq!((a.hi, a.lo), (two_pow(60), 1.0));
        let b = Dd::from_f64(two_pow(20) + 1.0);
        let expected = (1i128 << 80) + (1i128 << 60) + (1i128 << 20) + 1;
        assert_eq!(exact_integer(a.mul(b)), expected);
        assert_eq!(exact_integer(a.mul_f64(two_pow(20) + 1.0)), expected);
        // (2^60 + 1)(2^60 - 1) = 2^120 - 1 needs 120 bits; the lo*lo term is
        // dropped by design, so the error is 1, a relative 2^-120, within the
        // few-units-of-2^-106 bound.
        let c = Dd::from_f64(two_pow(60)).add_f64(-1.0);
        let error = exact_integer(a.mul(c)) - ((1i128 << 120) - 1);
        assert!(error.abs() <= 1i128 << 16, "{error}");
    }

    #[test]
    fn div_and_sqrt_recover_exact_rationals() {
        // (2^80 - 1) / (2^40 - 1) = 2^40 + 1 exactly.
        let num = Dd::from_f64(two_pow(80)).add_f64(-1.0);
        let den = Dd::from_f64(two_pow(40)).add_f64(-1.0);
        assert_eq!(exact_integer(num.div(den)), (1i128 << 40) + 1);
        // 1/3 is not finite in binary; the quotient times 3 must return 1 to
        // within the double-double rounding, a few units of 2^-106.
        let third = Dd::from_f64(1.0).div(Dd::from_f64(3.0));
        let residual = third.mul_f64(3.0).sub(Dd::from_f64(1.0)).to_f64();
        assert!(residual.abs() <= 4.0 * two_pow(-106), "{residual}");
        // sqrt((2^40 + 1)^2) = 2^40 + 1: the square is 81 bits.
        let root = Dd::product(two_pow(40) + 1.0, two_pow(40) + 1.0).sqrt();
        assert_eq!(exact_integer(root), (1i128 << 40) + 1);
        assert_eq!(Dd::from_f64(two_pow(-60)).sqrt().to_f64(), two_pow(-30));
        assert_eq!(Dd::ZERO.sqrt(), Dd::ZERO);
        assert!(Dd::from_f64(-1.0).sqrt().hi.is_nan());
    }

    #[test]
    fn dot_survives_cancellation_that_binary64_loses() {
        // Exact value: 2^60 + 1 - 2^60 + 2^-70 = 1 + 2^-70. Binary64 summation
        // in this order gives 2^-70 (the 1 is absorbed by 2^60).
        let a = [two_pow(30), 1.0, -two_pow(30), two_pow(-35)];
        let b = [two_pow(30), 1.0, two_pow(30), two_pow(-35)];
        let naive: f64 = a.iter().zip(&b).map(|(x, y)| x * y).sum();
        assert_eq!(naive, two_pow(-70));
        let value = dot(&a, &b);
        assert_eq!(value.hi, 1.0);
        assert_eq!(value.lo, two_pow(-70));
        // Integer products whose binary64 rounding differs: Σ (2^27+1)^2 - 2^54.
        let x = two_pow(27) + 1.0;
        let mut acc = DdAccumulator::new();
        acc.add_product(x, x);
        acc.add_f64(-two_pow(54));
        assert_eq!(acc.value().to_f64(), two_pow(28) + 1.0);
    }

    #[test]
    fn to_f64_rounds_once_to_nearest() {
        // 1 + 2^-53 + 2^-80 lies above the midpoint between 1 and 1 + 2^-52,
        // so the correctly rounded value is 1 + 2^-52. Rounding 1 + 2^-53
        // first would give 1 (ties to even).
        let value = Dd::from_f64(1.0)
            .add_f64(two_pow(-53))
            .add_f64(two_pow(-80));
        assert_eq!(value.to_f64(), 1.0 + two_pow(-52));
        assert_eq!(Dd::from_f64(1.0).add_f64(two_pow(-53)).to_f64(), 1.0);
    }

    #[test]
    fn pi_and_atan() {
        // Machin: π = 16 atan(1/5) − 4 atan(1/239), an independent route to π.
        let fifth = Dd::from_f64(1.0).div(Dd::from_f64(5.0));
        let small = Dd::from_f64(1.0).div(Dd::from_f64(239.0));
        let machin = atan_unit(fifth)
            .mul_pow2(4)
            .sub(atan_unit(small).mul_pow2(2));
        let pi = Dd::pi();
        assert_eq!(pi.hi, std::f64::consts::PI);
        assert!(machin.sub(pi).to_f64().abs() <= 8.0 * two_pow(-106) * pi.hi);
        // atan 1 = π/4 through three half-angle steps.
        let quarter = atan_unit(Dd::from_f64(1.0)).mul_pow2(2);
        assert!(quarter.sub(pi).to_f64().abs() <= 8.0 * two_pow(-106) * pi.hi);
        // atan(2 − √3) = π/12.
        let three = Dd::from_f64(3.0).sqrt();
        let twelfth = atan_unit(Dd::from_f64(2.0).sub(three)).mul_f64(12.0);
        assert!(twelfth.sub(pi).to_f64().abs() <= 16.0 * two_pow(-106) * pi.hi);
        assert_eq!(atan_unit(Dd::ZERO), Dd::ZERO);
    }

    #[test]
    fn abs_and_neg() {
        let a = Dd::from_f64(-two_pow(60)).add_f64(1.0);
        assert_eq!(exact_integer(a.abs()), (1i128 << 60) - 1);
        assert_eq!(exact_integer(a.neg()), (1i128 << 60) - 1);
    }

    fn w(x: f64) -> WideDd {
        WideDd::from_f64(x)
    }

    /// Review round 3: A = W(2¹⁰⁰⁰) + W(1 + 2⁻⁵²) and B = W(2¹⁰⁰⁰) + W(1)
    /// differ by exactly 2⁻⁵². The difference's high word is the subnormal
    /// 2⁻¹⁰⁵², which the unbounded rescaling by 2¹⁰⁵² turned into ∞.
    #[test]
    fn wide_normalize_rescales_a_subnormal_high_word() {
        let a = w(two_pow(1000)).add(w(1.0 + two_pow(-52)));
        let b = w(two_pow(1000)).add(w(1.0));
        assert_eq!(a.sub(b).to_f64(), two_pow(-52));
    }

    /// Review round 3: (1 + 2⁻⁵⁴)·2⁻¹⁰⁷⁵ lies just above half of 2⁻¹⁰⁷⁴, so
    /// it rounds to 2⁻¹⁰⁷⁴; rounding the mantissa to 53 bits first gave
    /// exactly the half, which rounds to even, 0.
    #[test]
    fn wide_to_f64_rounds_once_into_the_subnormals() {
        let value = WideDd {
            mantissa: Dd {
                hi: 1.0,
                lo: two_pow(-54),
            },
            exponent: -1075,
        };
        assert_eq!(value.to_f64(), f64::from_bits(1));
        assert_eq!(value.neg().to_f64(), -f64::from_bits(1));
        // Exact halves round to even.
        let half = WideDd {
            mantissa: Dd::from_f64(1.0),
            exponent: -1075,
        };
        assert_eq!(half.to_f64(), 0.0);
        let three_halves = WideDd {
            mantissa: Dd::from_f64(1.5),
            exponent: -1074,
        };
        assert_eq!(three_halves.to_f64(), f64::from_bits(2));
    }

    #[test]
    fn wide_add_propagates_a_non_finite_operand() {
        // A NaN more than 2¹⁰⁰⁰ below the other addend's exponent (2²⁰⁰⁰
        // here) was dropped as negligible, giving ∞.
        let nan = w(f64::NAN);
        let huge = w(two_pow(1000)).mul(w(two_pow(1000)));
        assert!(huge.add(nan).to_f64().is_nan());
        assert!(nan.add(huge).to_f64().is_nan());
        let tiny = w(two_pow(-1000)).mul(w(two_pow(-1000)));
        assert!(nan.add(tiny).to_f64().is_nan());
        assert_eq!(w(two_pow(-1000)).add(w(f64::INFINITY)).to_f64(), f64::INFINITY);
        assert!(w(f64::INFINITY).add(w(f64::NEG_INFINITY)).to_f64().is_nan());
        // √ and its range.
        assert_eq!(w(two_pow(-1000)).mul(w(two_pow(-1000))).sqrt().to_f64(), two_pow(-1000));
        assert_eq!(w(8.0).sqrt().mul(w(2.0).sqrt()).to_f64(), 4.0);
        assert!(w(-1.0).sqrt().to_f64().is_nan());
        assert!(max_propagating_nan(1.0, f64::NAN).is_nan());
        assert_eq!(max_propagating_nan(1.0, 2.0), 2.0);
    }

    /// (integer significand, exponent) of a finite nonzero binary64.
    fn decompose(x: f64) -> (u128, i32) {
        let bits = x.abs().to_bits();
        let field = (bits >> 52) as i32;
        let fraction = (bits & ((1u64 << 52) - 1)) as u128;
        if field == 0 {
            (fraction, -1074)
        } else {
            (fraction | (1u128 << 52), field - 1075)
        }
    }

    /// m·2^e (m > 0) rounded once to binary64, ties to even, with
    /// subnormals and overflow, in integer arithmetic.
    fn round_exact(m: u128, e: i32) -> f64 {
        let bits = 128 - m.leading_zeros() as i32;
        let lead = e + bits - 1;
        // The unit of the destination: 53 significant bits in the normal
        // range, 2⁻¹⁰⁷⁴ below it.
        let unit = if lead >= -1022 { lead - 52 } else { -1074 };
        let shift = unit - e;
        let mut q = if shift <= 0 {
            m << (-shift) as u32
        } else if shift >= 128 {
            0
        } else {
            let q = m >> shift as u32;
            let rem = m & ((1u128 << shift as u32) - 1);
            let half = 1u128 << (shift - 1) as u32;
            if rem > half || (rem == half && q % 2 == 1) {
                q + 1
            } else {
                q
            }
        };
        let mut unit = unit;
        if q == 1u128 << 53 {
            q >>= 1;
            unit += 1;
        }
        if unit + 52 > 1023 {
            return f64::INFINITY;
        }
        scale_by_pow2(q as f64, unit)
    }

    /// WideDd products of two binary64 values are exact (TwoProduct of
    /// 53-bit significands), so to_f64 must equal the exact product rounded
    /// once. Deterministic pseudo-random operands, most with products near
    /// and below the subnormal range, some normal, overflowing or with
    /// subnormal operands.
    #[test]
    fn wide_products_round_like_exact_arithmetic() {
        let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        let mut checked = [0usize; 4];
        for i in 0..200_000 {
            let operand = |exponent_field: u64, r: u64| {
                let sign = (r >> 63) << 63;
                f64::from_bits(sign | (exponent_field << 52) | (r & ((1u64 << 52) - 1)))
            };
            let (fa, fb) = match i % 4 {
                // Products from about 2⁻¹¹⁰⁰ to 2⁻¹⁰⁰⁰ (subnormal and zero).
                0 | 1 => {
                    let ea = 400 + next() % 200;
                    let spread = next() % 100;
                    (ea, 2 * 1023 - 1100 - ea + spread)
                }
                // A subnormal operand.
                2 => (0, 900 + next() % 400),
                // Anywhere, overflow included.
                _ => {
                    let ea = 1 + next() % 2046;
                    (ea, 1 + next() % 2046)
                }
            };
            let (ra, rb) = (next(), next());
            let (a, b) = (operand(fa, ra), operand(fb, rb));
            if a == 0.0 || b == 0.0 {
                continue;
            }
            let (ma, ea) = decompose(a);
            let (mb, eb) = decompose(b);
            let sign = if (a < 0.0) != (b < 0.0) { -1.0 } else { 1.0 };
            let expected = sign * round_exact(ma * mb, ea + eb);
            let actual = w(a).mul(w(b)).to_f64();
            assert_eq!(
                actual.to_bits(),
                expected.to_bits(),
                "{a:e} × {b:e}: {actual:e} vs {expected:e}"
            );
            let class = if expected == 0.0 {
                0
            } else if expected.is_infinite() {
                3
            } else if expected.is_normal() {
                2
            } else {
                1
            };
            checked[class] += 1;
        }
        // Zero, subnormal, normal and overflowing results all occur.
        assert!(checked.iter().all(|&c| c > 1000), "{checked:?}");
    }
}
