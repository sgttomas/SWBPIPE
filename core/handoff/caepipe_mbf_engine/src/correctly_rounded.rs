//! Correctly rounded `acos`, `asin`, `atan2`, `sin` and `tan` on binary64: the port of the Python twin's
//! `core/handoff/caepipe_mbf/correctly_rounded.py`, step for step.
//!
//! CPython's `math` takes these five functions from the platform C library, whose last bit differs between platforms.
//! Both twins instead return the binary64 nearest the exact value of the exact binary64 input, ties to even, so they
//! agree on every input, natively and in wasm, by definition of correct rounding.
//!
//! Method (Ziv's strategy), as in the Python module: at a working precision of `w` bits, integers `lo <= hi` with
//! `lo / 2^w <= f(x) <= hi / 2^w` rigorously, every truncation of the binary fixed-point evaluation carrying an explicit
//! error bound in units of `2^-w`. When `lo / 2^w` and `hi / 2^w` round to the same binary64 (`int / int` correctly
//! rounded, subnormals included) that is the result; otherwise the precision doubles. pi by Machin's formula,
//! arctangent by argument halving and its Taylor series, sine and cosine by their Taylor series after reduction by a
//! multiple of pi/2 (pi carried beyond the input's binade), square roots by integer square root. The platform
//! library's value only chooses the first precision; any accepted result is the correctly rounded one, whatever that
//! choice. Special values and domains are answered by the callers in `pymath`, as `math` answers them.

use crate::pystr::PyError;
use crate::rational::ratio_to_f64;
use num_bigint::{BigInt, Sign};
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::cell::RefCell;

/// Bits above the result's 53 in the first evaluation (`_GUARD`).
const GUARD: usize = 24;
/// The precision at which the loop gives up (`_MAX_BITS`); unreachable for binary64 inputs.
const MAX_BITS: usize = 1 << 17;
/// `_PI_SPARE`.
const PI_SPARE: usize = 40;

fn exhausted(name: &str) -> PyError {
    PyError { kind: "ArithmeticError", message: format!("correctly rounded {name} did not converge") }
}

/// `math.frexp(x)[1]` for a finite nonzero x.
fn frexp_exponent(value: f64) -> i64 {
    let bits = value.to_bits();
    let biased = ((bits >> 52) & 0x7ff) as i64;
    if biased == 0 {
        let fraction = bits & ((1u64 << 52) - 1);
        // subnormal: value = fraction 2^-1074, fraction in [2^(b-1), 2^b) gives frexp exponent b - 1074
        (64 - fraction.leading_zeros() as i64) - 1074
    } else {
        biased - 1022
    }
}

/// `x.as_integer_ratio()` as (numerator, k) with denominator 2^k, in lowest terms; (0, 0) for a zero.
fn integer_ratio(x: f64) -> (BigInt, usize) {
    let bits = x.to_bits();
    let negative = bits >> 63 == 1;
    let biased = ((bits >> 52) & 0x7ff) as i64;
    let fraction = bits & ((1u64 << 52) - 1);
    let (mut mantissa, mut exponent) = if biased == 0 { (fraction, -1074i64) } else { (fraction | (1u64 << 52), biased - 1075) };
    if mantissa == 0 {
        return (BigInt::zero(), 0);
    }
    let numerator = if exponent >= 0 {
        BigInt::from(mantissa) << (exponent as usize)
    } else {
        let strip = (mantissa.trailing_zeros() as i64).min(-exponent);
        mantissa >>= strip;
        exponent += strip;
        BigInt::from(mantissa)
    };
    let shift = if exponent < 0 { (-exponent) as usize } else { 0 };
    (if negative { -numerator } else { numerator }, shift)
}

/// Python's `int / int`: correctly rounded, the sign applied to a zero quotient as well; None for OverflowError.
fn true_div(numerator: &BigInt, denominator: &BigInt) -> Option<f64> {
    let negative = (numerator.sign() == Sign::Minus) != (denominator.sign() == Sign::Minus);
    let magnitude = ratio_to_f64(&numerator.abs(), &denominator.abs())?;
    Some(if negative { -magnitude } else { magnitude })
}

/// `_first_bits(hint, relative_extra)`.
fn first_bits(hint: f64, relative_extra: usize) -> usize {
    let exponent = if hint == 0.0 || !hint.is_finite() { -1075 } else { frexp_exponent(hint) };
    53 + GUARD + (-exponent).max(0) as usize + relative_extra
}

/// `_round(lo, hi, bits)`: the binary64 every real in [lo, hi] / 2^bits rounds to, or None.
fn round(lo: &BigInt, hi: &BigInt, bits: usize) -> Option<f64> {
    let scale = BigInt::one() << bits;
    let low = true_div(lo, &scale)?;
    let high = true_div(hi, &scale)?;
    if low == high && (low != 0.0 || !lo.is_negative() || !hi.is_positive()) {
        Some(low)
    } else {
        None
    }
}

// -------------------------------------------------------------------------------------------------------------------
// pi
// -------------------------------------------------------------------------------------------------------------------

thread_local! {
    /// `_PI_CACHE`: (bits, P) with |P - pi 2^bits| < 2, grown on demand.
    static PI_CACHE: RefCell<(usize, BigInt)> = RefCell::new((0, BigInt::zero()));
}

/// `_atan_inverse(n, bits)`: (A, E) with |A - atan(1/n) 2^bits| <= E, for n >= 2.
fn atan_inverse(n: u32, bits: usize) -> (BigInt, BigInt) {
    let mut power = (BigInt::one() << bits) / n;
    let mut total = power.clone();
    let square = BigInt::from(u64::from(n) * u64::from(n));
    let mut k: u64 = 0;
    loop {
        power = &power / &square;
        k += 1;
        let term = &power / (2 * k + 1);
        if term.is_zero() {
            break;
        }
        if k & 1 == 1 {
            total -= term;
        } else {
            total += term;
        }
    }
    (total, BigInt::from(2 * k + 2))
}

/// `_pi_bits(bits)`: (P, 2) with |P - pi 2^bits| <= 2, for bits >= 1.
fn pi_bits(bits: usize) -> (BigInt, BigInt) {
    PI_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if cache.0 < bits + PI_SPARE {
            let work = (2 * cache.0).max(bits + PI_SPARE).max(1400);
            let inner = work + PI_SPARE;
            let (a5, _e5) = atan_inverse(5, inner);
            let (a239, _e239) = atan_inverse(239, inner);
            // 16 e5 + 4 e239 is about 7 inner units, far below 2^PI_SPARE: the shift leaves an error below 2
            let value: BigInt = 16 * a5 - 4 * a239;
            *cache = (work, value >> PI_SPARE);
        }
        (&cache.1 >> (cache.0 - bits), BigInt::from(2))
    })
}

// -------------------------------------------------------------------------------------------------------------------
// arctangent on [0, 1]
// -------------------------------------------------------------------------------------------------------------------

/// `_atan_fixed(t, bits)`: (V, E) with |V - atan(t / 2^bits) 2^bits| <= E, for 0 <= t <= 2^bits.
fn atan_fixed(t: &BigInt, bits: usize) -> (BigInt, BigInt) {
    if t.is_zero() {
        return (BigInt::zero(), BigInt::zero());
    }
    let one = BigInt::one() << bits;
    let sixteenth = &one >> 4usize;
    let mut t = t.clone();
    let mut halvings = 0usize;
    let mut error: u64 = 0;
    while t > sixteenth {
        let root = ((&one << bits) + &t * &t).sqrt();
        t = (&t << bits) / (&one + root);
        error = error.div_ceil(2) + 2;
        halvings += 1;
    }
    let square = (&t * &t) >> bits;
    let mut power = t.clone();
    let mut total = t;
    let mut n: u64 = 1;
    let mut terms: u64 = 0;
    loop {
        power = (&power * &square) >> bits;
        n += 2;
        let term = &power / n;
        if term.is_zero() {
            break;
        }
        terms += 1;
        if terms & 1 == 1 {
            total -= term;
        } else {
            total += term;
        }
    }
    let series_error = 2 * terms + 2 + error;
    (total << halvings, BigInt::from(series_error) << halvings)
}

/// `_atan_interval(t_lo, t_hi, bits)`.
fn atan_interval(t_lo: &BigInt, t_hi: &BigInt, bits: usize) -> (BigInt, BigInt) {
    let (value, error) = atan_fixed(t_lo, bits);
    (&value - &error, value + error + (t_hi - t_lo))
}

/// `_atan_ratio(numerator, denominator, bits)` for positive integers.
fn atan_ratio(numerator: &BigInt, denominator: &BigInt, bits: usize) -> (BigInt, BigInt) {
    if numerator <= denominator {
        let t = (numerator << bits) / denominator;
        let next = &t + 1;
        return atan_interval(&t, &next, bits);
    }
    let (low, high) = atan_ratio(denominator, numerator, bits);
    let (half_pi, pi_error) = pi_bits(bits - 1);
    (&half_pi - &pi_error - high, half_pi + pi_error - low)
}

// -------------------------------------------------------------------------------------------------------------------
// sine and cosine
// -------------------------------------------------------------------------------------------------------------------

/// `_sin_cos_fixed(r, bits)`: (S, C, E), for |r| <= 0.8 2^bits.
fn sin_cos_fixed(r: &BigInt, bits: usize) -> (BigInt, BigInt, BigInt) {
    let magnitude = r.abs();
    let one = BigInt::one() << bits;
    let square = (&magnitude * &magnitude) >> bits;
    let mut term = magnitude.clone();
    let mut sine = magnitude;
    let mut k: u64 = 0;
    loop {
        k += 1;
        term = ((&term * &square) >> bits) / ((2 * k) * (2 * k + 1));
        if term.is_zero() {
            break;
        }
        if k & 1 == 1 {
            sine -= &term;
        } else {
            sine += &term;
        }
    }
    let sine_error = 3 * k + 3;
    let mut term = one.clone();
    let mut cosine = one;
    let mut j: u64 = 0;
    loop {
        j += 1;
        term = ((&term * &square) >> bits) / ((2 * j - 1) * (2 * j));
        if term.is_zero() {
            break;
        }
        if j & 1 == 1 {
            cosine -= &term;
        } else {
            cosine += &term;
        }
    }
    let cosine_error = 3 * j + 3;
    let sine = if r.is_negative() { -sine } else { sine };
    (sine, cosine, BigInt::from(sine_error.max(cosine_error)))
}

/// `_reduce(x, bits)`: (q, R, E) with x = q pi/2 + r, |r| <= 0.79 and |R - r 2^bits| <= E.
fn reduce(x: f64, bits: usize) -> (BigInt, BigInt, BigInt) {
    let (numerator, shift) = integer_ratio(x);
    if x.abs() < 0.78 {
        if shift <= bits {
            return (BigInt::zero(), numerator << (bits - shift), BigInt::zero());
        }
        let denominator = BigInt::one() << shift;
        return (BigInt::zero(), (numerator << bits).div_floor(&denominator), BigInt::one());
    }
    // |x| >= 0.78: shift <= 53 < bits
    let exponent = frexp_exponent(x);
    let spare = exponent.max(0) as usize + 8;
    let wide = bits + spare;
    let (half_pi, _pi_error) = pi_bits(wide - 1);
    let scaled: BigInt = numerator << (wide - shift);
    let doubled: BigInt = &scaled * 2u32 + &half_pi;
    let q = doubled.div_floor(&(&half_pi * 2u32));
    let residue = scaled - &q * &half_pi;
    let reduced = residue.div_floor(&(BigInt::one() << spare));
    (q, reduced, BigInt::from(2))
}

// -------------------------------------------------------------------------------------------------------------------
// public functions (finite inputs; `pymath` answers the special values as `math` does)
// -------------------------------------------------------------------------------------------------------------------

/// `sin(x)` correctly rounded; NaN for a non-finite x, x itself for a zero.
pub fn sin(x: f64) -> Result<f64, PyError> {
    if x == 0.0 || !x.is_finite() {
        return Ok(if x == 0.0 { x } else { f64::NAN });
    }
    let mut bits = first_bits(x.sin(), 0);
    while bits <= MAX_BITS {
        let (q, r, reduction_error) = reduce(x, bits);
        let (s, c, error) = sin_cos_fixed(&r, bits);
        let error = error + reduction_error;
        let value = match q.mod_floor(&BigInt::from(4)).to_u8() {
            Some(0) => s,
            Some(1) => c,
            Some(2) => -s,
            _ => -c,
        };
        if let Some(result) = round(&(&value - &error), &(&value + &error), bits) {
            return Ok(result);
        }
        bits *= 2;
    }
    Err(exhausted("sin"))
}

/// `_quotient_bounds`: (fl(min a/b), fl(max a/b)) over the boxes, or None.
fn quotient_bounds(a_lo: &BigInt, a_hi: &BigInt, b_lo: &BigInt, b_hi: &BigInt) -> Option<(f64, f64)> {
    if !b_lo.is_positive() && !b_hi.is_negative() {
        return None;
    }
    let candidates = [true_div(a_lo, b_lo)?, true_div(a_lo, b_hi)?, true_div(a_hi, b_lo)?, true_div(a_hi, b_hi)?];
    let (mut low, mut high) = (candidates[0], candidates[0]);
    for &candidate in &candidates[1..] {
        if candidate < low {
            low = candidate;
        }
        if candidate > high {
            high = candidate;
        }
    }
    Some((low, high))
}

/// `tan(x)` correctly rounded; NaN for a non-finite x, x itself for a zero.
pub fn tan(x: f64) -> Result<f64, PyError> {
    if x == 0.0 || !x.is_finite() {
        return Ok(if x == 0.0 { x } else { f64::NAN });
    }
    let hint = x.tan();
    let extra = if hint == 0.0 || !hint.is_finite() { 0 } else { frexp_exponent(hint).max(0) as usize };
    let mut bits = first_bits(hint, extra);
    while bits <= MAX_BITS {
        let (q, r, reduction_error) = reduce(x, bits);
        let (s, c, error) = sin_cos_fixed(&r, bits);
        let error = error + reduction_error;
        let bounds = if q.is_odd() {
            // tan(q pi/2 + r) = -cos r / sin r
            quotient_bounds(&(-&c - &error), &(-&c + &error), &(&s - &error), &(&s + &error))
        } else {
            quotient_bounds(&(&s - &error), &(&s + &error), &(&c - &error), &(&c + &error))
        };
        if let Some((low, high)) = bounds {
            if low == high {
                return Ok(low);
            }
        }
        bits *= 2;
    }
    Err(exhausted("tan"))
}

/// `atan2(y, x)` correctly rounded, for finite y != 0 and finite x.
pub fn atan2(y: f64, x: f64) -> Result<f64, PyError> {
    let mut bits = first_bits(y.atan2(x), 0);
    let (y_num, y_shift) = integer_ratio(y.abs());
    let (numerator, denominator) = if x == 0.0 {
        (BigInt::zero(), BigInt::zero())
    } else {
        let (x_num, x_shift) = integer_ratio(x.abs());
        (y_num << x_shift, x_num << y_shift)
    };
    while bits <= MAX_BITS {
        let (lo, hi) = if x == 0.0 {
            let (half_pi, pi_error) = pi_bits(bits - 1);
            (&half_pi - &pi_error, half_pi + pi_error)
        } else {
            let (lo, hi) = atan_ratio(&numerator, &denominator, bits);
            if x < 0.0 {
                let (pi, pi_error) = pi_bits(bits);
                (&pi - &pi_error - hi, pi + pi_error - lo)
            } else {
                (lo, hi)
            }
        };
        let lo = if lo.is_negative() { BigInt::zero() } else { lo };
        if let Some(result) = round(&lo, &hi, bits) {
            return Ok(if y < 0.0 { -result } else { result });
        }
        bits *= 2;
    }
    Err(exhausted("atan2"))
}

/// `_arcsine_parts(a, bits)` for 0 <= a <= 1: ([lo, hi] around phi, small).
fn arcsine_parts(a: f64, bits: usize) -> (BigInt, BigInt, bool) {
    let (numerator, shift) = integer_ratio(a);
    let denominator = BigInt::one() << shift;
    let square_den = &denominator * &denominator;
    let rest = &square_den - &numerator * &numerator;
    let root = ((rest << (2 * bits)) / &square_den).sqrt();
    if 2 * &numerator * &numerator <= square_den {
        let top = &numerator << (2 * bits);
        let t_lo = &top / (&denominator * (&root + 1));
        let t_hi = Integer::div_ceil(&top, &(&denominator * &root));
        let (lo, hi) = atan_interval(&t_lo, &t_hi, bits);
        return (lo, hi, true);
    }
    let t_lo = (&root * &denominator) / &numerator;
    let t_hi = Integer::div_ceil(&((&root + 1) * &denominator), &numerator);
    let (lo, hi) = atan_interval(&t_lo, &t_hi, bits);
    (lo, hi, false)
}

/// `asin(x)` correctly rounded; x itself for a zero, NaN outside [-1, 1].
pub fn asin(x: f64) -> Result<f64, PyError> {
    if x == 0.0 || !(x.abs() <= 1.0) {
        return Ok(if x == 0.0 { x } else { f64::NAN });
    }
    let a = x.abs();
    let mut bits = first_bits(a.asin(), 0);
    while bits <= MAX_BITS {
        let (mut lo, mut hi, small) = arcsine_parts(a, bits);
        if !small {
            let (half_pi, pi_error) = pi_bits(bits - 1);
            (lo, hi) = (&half_pi - &pi_error - hi, half_pi + pi_error - lo);
        }
        if let Some(result) = round(&lo, &hi, bits) {
            return Ok(if x < 0.0 { -result } else { result });
        }
        bits *= 2;
    }
    Err(exhausted("asin"))
}

/// `acos(x)` correctly rounded; +0 at 1, NaN outside [-1, 1].
pub fn acos(x: f64) -> Result<f64, PyError> {
    if x == 1.0 || !(x.abs() <= 1.0) {
        return Ok(if x == 1.0 { 0.0 } else { f64::NAN });
    }
    let a = x.abs();
    let mut bits = first_bits(x.acos(), 0);
    while bits <= MAX_BITS {
        let (mut lo, mut hi, small) = arcsine_parts(a, bits);
        if small {
            let (half_pi, pi_error) = pi_bits(bits - 1);
            (lo, hi) = (&half_pi - &pi_error - hi, half_pi + pi_error - lo);
        }
        if x < 0.0 {
            let (pi, pi_error) = pi_bits(bits);
            (lo, hi) = (&pi - &pi_error - hi, pi + pi_error - lo);
        }
        if let Some(result) = round(&lo, &hi, bits) {
            return Ok(result);
        }
        bits *= 2;
    }
    Err(exhausted("acos"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn helpers_follow_python() {
        assert_eq!(integer_ratio(0.75), (BigInt::from(3), 2));
        assert_eq!(integer_ratio(-6.0), (BigInt::from(-6), 0));
        assert_eq!(integer_ratio(5e-324), (BigInt::one(), 1074));
        assert_eq!(frexp_exponent(1.0), 1);
        assert_eq!(frexp_exponent(5e-324), -1073);
        assert_eq!(frexp_exponent(-0.75), 0);
        assert_eq!(true_div(&BigInt::zero(), &BigInt::from(-5)).map(f64::to_bits), Some((-0.0f64).to_bits()));
        assert_eq!(true_div(&BigInt::from(-1), &(BigInt::one() << 2000usize)).map(f64::to_bits), Some((-0.0f64).to_bits()));
        assert_eq!(true_div(&(BigInt::one() << 1024usize), &BigInt::one()), None);
        // floor semantics of the reduction for a negative input below 0.78
        let (_, r, e) = reduce(-0.1, 10);
        assert_eq!((r, e), (BigInt::from(-103), BigInt::one()));
        let (p, _) = pi_bits(60);
        assert_eq!(p, BigInt::from(3622009729038561421u64));
    }
}
