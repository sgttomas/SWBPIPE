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
            .sub(atan_unit(small).mul_pow2(2))
            .mul_pow2(2);
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
}
