//! The importer's `twice_atan2_nonnegative`: Python `decimal` arithmetic at 90 significant digits, ROUND_HALF_EVEN,
//! operation for operation, then one rounding to binary64.
//!
//! Each `decimal` operation (+, -, *, /, sqrt, unary minus, abs) is the exact result rounded once to the context's
//! 90 digits, half to even; division and square root are correctly rounded. Values are compared exactly. The
//! exponent range (Emin -10^6, Emax 10^6) is never reached by quotients of binary64 values.

use crate::rational::{pow10, Q};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use std::cmp::Ordering;

const PREC: usize = 90;

/// A decimal value c x 10^e (c signed).
#[derive(Clone, Debug)]
struct D {
    c: BigInt,
    e: i64,
}

fn digit_count(value: &BigInt) -> usize {
    if value.is_zero() {
        1
    } else {
        value.abs().to_string().len()
    }
}

impl D {
    fn int(value: i64) -> D {
        D { c: BigInt::from(value), e: 0 }
    }
    /// `Decimal(x)` for a finite binary64: exact.
    fn from_f64(value: f64) -> D {
        let exact = Q::from_f64(value).expect("finite");
        // denominator is a power of two 2^k: x = n / 2^k = n 5^k / 10^k
        let k = exact.denom_bits() as i64 - 1;
        let five = num_traits::pow(BigInt::from(5u8), k as usize);
        D { c: exact.numer() * five, e: -k }
    }
    fn to_q(&self) -> Q {
        Q::from_bigint(self.c.clone()).mul(&Q::pow10(self.e))
    }
    fn is_zero(&self) -> bool {
        self.c.is_zero()
    }
    fn cmp(&self, other: &D) -> Ordering {
        let shift = self.e.min(other.e);
        let a = &self.c * pow10((self.e - shift) as u64);
        let b = &other.c * pow10((other.e - shift) as u64);
        a.cmp(&b)
    }
    fn abs(&self) -> D {
        D { c: self.c.abs(), e: self.e }
    }
}

/// Round |c| x 10^e (with `sticky` set when the exact value lies strictly above it) to PREC digits, half to even.
fn round(c: BigInt, e: i64, sticky: bool) -> D {
    let negative = c.is_negative();
    let magnitude = c.abs();
    let digits = digit_count(&magnitude);
    if digits <= PREC {
        debug_assert!(!sticky, "an inexact value has more digits than the precision");
        return D { c: if negative { -magnitude } else { magnitude }, e };
    }
    let drop = (digits - PREC) as u64;
    let divisor = pow10(drop);
    let (mut kept, rest) = magnitude.div_rem(&divisor);
    let twice = &rest * BigInt::from(2u8);
    let up = match twice.cmp(&divisor) {
        Ordering::Greater => true,
        Ordering::Less => false,
        Ordering::Equal => sticky || kept.is_odd(),
    };
    if up {
        kept += 1;
    }
    let mut exponent = e + drop as i64;
    if digit_count(&kept) > PREC {
        kept /= 10;
        exponent += 1;
    }
    D { c: if negative { -kept } else { kept }, e: exponent }
}

fn add(a: &D, b: &D) -> D {
    let shift = a.e.min(b.e);
    let sum = &a.c * pow10((a.e - shift) as u64) + &b.c * pow10((b.e - shift) as u64);
    round(sum, shift, false)
}

fn sub(a: &D, b: &D) -> D {
    add(a, &D { c: -b.c.clone(), e: b.e })
}

fn mul(a: &D, b: &D) -> D {
    round(&a.c * &b.c, a.e + b.e, false)
}

fn neg(a: &D) -> D {
    round(-a.c.clone(), a.e, false)
}

fn div(a: &D, b: &D) -> D {
    assert!(!b.is_zero(), "decimal division by zero");
    if a.is_zero() {
        return D { c: BigInt::zero(), e: 0 };
    }
    let negative = a.c.is_negative() != b.c.is_negative();
    let numerator = a.c.abs();
    let denominator = b.c.abs();
    // scale so the quotient has at least PREC + 2 digits
    let extra = (PREC + 2 + digit_count(&denominator)).saturating_sub(digit_count(&numerator)) as u64;
    let (quotient, rest) = (&numerator * pow10(extra)).div_rem(&denominator);
    let exponent = a.e - b.e - extra as i64;
    let signed = if negative { -quotient } else { quotient };
    round_sticky(signed, exponent, !rest.is_zero())
}

/// round() for a truncated value: the sticky bit only matters past the guard digits, which `round` reads.
fn round_sticky(c: BigInt, e: i64, sticky: bool) -> D {
    if !sticky {
        return round(c, e, false);
    }
    // the exact value is |c| + epsilon: append a sticky digit so the half-even tie test sees it
    let negative = c.is_negative();
    let widened = c.abs() * BigInt::from(10u8) + BigInt::one();
    let result = round(widened, e - 1, false);
    if negative {
        D { c: -result.c, e: result.e }
    } else {
        result
    }
}

fn sqrt(a: &D) -> D {
    assert!(!a.c.is_negative(), "decimal sqrt of a negative value");
    if a.is_zero() {
        return D { c: BigInt::zero(), e: a.e / 2 };
    }
    let digits = digit_count(&a.c) as i64;
    let mut t = (2 * (PREC as i64 + 2) - digits).max(0);
    if (a.e - t).rem_euclid(2) != 0 {
        t += 1;
    }
    let scaled = &a.c * pow10(t as u64);
    let root = scaled.sqrt();
    let exact = &root * &root == scaled;
    round_sticky(root, (a.e - t) / 2, !exact)
}

fn scaleb(a: &D, places: i64) -> D {
    D { c: a.c.clone(), e: a.e + places }
}

/// `_decimal_atan(t)` for 0 <= t <= 1.
fn decimal_atan(t: D) -> D {
    let mut t = t;
    let eighth = D { c: BigInt::from(125), e: -3 };
    let one = D::int(1);
    let mut doublings: u32 = 0;
    while t.cmp(&eighth) == Ordering::Greater {
        let inner = sqrt(&add(&one, &mul(&t, &t)));
        t = div(&t, &add(&one, &inner));
        doublings += 1;
    }
    let square = mul(&t, &t);
    let mut term = t.clone();
    let mut total = t.clone();
    let mut n: i64 = 1;
    loop {
        term = mul(&neg(&term), &square);
        n += 2;
        let step = div(&term, &D::int(n));
        if step.is_zero() || step.abs().cmp(&scaleb(&total.abs(), -(PREC as i64) - 4)) == Ordering::Less {
            break;
        }
        total = add(&total, &step);
    }
    mul(&total, &D::int(2i64.pow(doublings)))
}

/// frame_kernel `twice_atan2_nonnegative` as the importer ports it: 2 atan2(y, x) for finite y, x >= 0, not both
/// 0, rounded once to binary64; +0 for y = 0 and fl(pi) for x = 0; None otherwise.
pub fn twice_atan2_nonnegative(y: f64, x: f64) -> Option<f64> {
    if !(y.is_finite() && x.is_finite()) || y < 0.0 || x < 0.0 || (y == 0.0 && x == 0.0) {
        return None;
    }
    if y == 0.0 {
        return Some(0.0);
    }
    if x == 0.0 {
        return Some(std::f64::consts::PI);
    }
    let quotient = div(&D::from_f64(y), &D::from_f64(x));
    let one = D::int(1);
    let angle = if quotient.cmp(&one) != Ordering::Greater {
        decimal_atan(quotient)
    } else {
        let quarter_pi =
            sub(&mul(&D::int(4), &decimal_atan(div(&one, &D::int(5)))), &decimal_atan(div(&one, &D::int(239))));
        sub(&mul(&D::int(2), &quarter_pi), &decimal_atan(div(&one, &quotient)))
    };
    mul(&D::int(2), &angle).to_q().to_f64().ok()
}
