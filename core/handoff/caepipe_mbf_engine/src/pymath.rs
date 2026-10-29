//! CPython's own floating-point algorithms the importer calls: `math.hypot` (`vector_norm`), `sum()` of floats
//! (Neumaier compensated, from the int 0 start), `math.atan2`'s special cases, `math.radians`/`degrees`, and the
//! importer's exact `correct_norm3`. The importer takes acos, asin, atan2, sin and tan from its `correctly_rounded`
//! module, ported in `correctly_rounded`; these wrappers add `math`'s special values and domain errors.

use crate::pystr::PyError;
use crate::rational::{ratio_to_f64, Q};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, ToPrimitive};
use std::cmp::Ordering;

/// `frexp(x)`'s exponent for a finite positive x: x = m 2^e with m in [0.5, 1).
fn frexp_exponent(value: f64) -> i32 {
    let bits = value.to_bits();
    let biased = ((bits >> 52) & 0x7ff) as i32;
    if biased == 0 {
        // subnormal: scale into the normal range first
        let scaled = value * f64::from_bits(0x4350000000000000); // 2^54
        frexp_exponent(scaled) - 54
    } else {
        biased - 1022
    }
}

fn ldexp_one(exponent: i32) -> f64 {
    // 2^exponent for exponent in [-1074, 1023]
    if exponent >= -1022 {
        f64::from_bits(((exponent + 1023) as u64) << 52)
    } else {
        f64::from_bits(1u64 << (exponent + 1074))
    }
}

fn dl_mul(x: f64, y: f64) -> (f64, f64) {
    let z = x * y;
    (z, x.mul_add(y, -z))
}

fn dl_fast_sum(a: f64, b: f64) -> (f64, f64) {
    let x = a + b;
    let z = x - a;
    (x, b - z)
}

fn vector_norm(vec: &mut [f64], max: f64, found_nan: bool) -> f64 {
    if max.is_infinite() {
        return max;
    }
    if found_nan {
        return f64::NAN;
    }
    if max == 0.0 || vec.len() <= 1 {
        return max;
    }
    let max_e = frexp_exponent(max);
    if max_e < -1023 {
        for value in vec.iter_mut() {
            *value /= f64::MIN_POSITIVE;
        }
        return f64::MIN_POSITIVE * vector_norm(vec, max / f64::MIN_POSITIVE, found_nan);
    }
    let scale = ldexp_one(-max_e);
    let (mut csum, mut frac1, mut frac2) = (1.0f64, 0.0f64, 0.0f64);
    for &value in vec.iter() {
        let x = value * scale;
        let (hi, lo) = dl_mul(x, x);
        let (sum_hi, sum_lo) = dl_fast_sum(csum, hi);
        csum = sum_hi;
        frac1 += lo;
        frac2 += sum_lo;
    }
    let mut h = (csum - 1.0 + (frac1 + frac2)).sqrt();
    let (hi, lo) = dl_mul(-h, h);
    let (sum_hi, sum_lo) = dl_fast_sum(csum, hi);
    csum = sum_hi;
    frac1 += lo;
    frac2 += sum_lo;
    let x = csum - 1.0 + (frac1 + frac2);
    h += x / (2.0 * h);
    h / scale
}

/// `math.hypot(*values)`.
pub fn hypot(values: &[f64]) -> f64 {
    let mut coordinates: Vec<f64> = values.iter().map(|value| value.abs()).collect();
    let mut max = 0.0f64;
    let mut found_nan = false;
    for &value in &coordinates {
        found_nan |= value.is_nan();
        if value > max {
            max = value;
        }
    }
    vector_norm(&mut coordinates, max, found_nan)
}

/// `sum(values)` for a non-empty sequence of floats (start 0).
pub fn sum(values: &[f64]) -> f64 {
    let Some((&first, rest)) = values.split_first() else { return 0.0 };
    let mut total = 0.0 + first;
    let mut compensation = 0.0f64;
    for &value in rest {
        let t = total + value;
        if total.abs() >= value.abs() {
            compensation += (total - t) + value;
        } else {
            compensation += (value - t) + total;
        }
        total = t;
    }
    if compensation != 0.0 && compensation.is_finite() {
        total += compensation;
    }
    total
}

/// `correctly_rounded.atan2(y, x)`: CPython's `m_atan2` special cases, then the correctly rounded value.
pub fn atan2(y: f64, x: f64) -> Result<f64, PyError> {
    use std::f64::consts::PI;
    if x.is_nan() || y.is_nan() {
        return Ok(f64::NAN);
    }
    if y.is_infinite() {
        if x.is_infinite() {
            return Ok(if x.is_sign_positive() { (0.25 * PI).copysign(y) } else { (0.75 * PI).copysign(y) });
        }
        return Ok((0.5 * PI).copysign(y));
    }
    if x.is_infinite() || y == 0.0 {
        return Ok(if x.is_sign_positive() { 0.0f64.copysign(y) } else { PI.copysign(y) });
    }
    crate::correctly_rounded::atan2(y, x)
}

/// `math.radians(x)` and `math.degrees(x)`.
pub fn radians(value: f64) -> f64 {
    value * (std::f64::consts::PI / 180.0)
}
pub fn degrees(value: f64) -> f64 {
    value * (180.0 / std::f64::consts::PI)
}

fn domain(result: f64, input: f64) -> Result<f64, PyError> {
    if result.is_nan() && !input.is_nan() {
        Err(PyError::value("math domain error"))
    } else {
        Ok(result)
    }
}

/// `correctly_rounded.acos`, `.asin`, `.tan`, `.sin`, with `math`'s domain errors.
pub fn acos(value: f64) -> Result<f64, PyError> {
    domain(crate::correctly_rounded::acos(value)?, value)
}
pub fn asin(value: f64) -> Result<f64, PyError> {
    domain(crate::correctly_rounded::asin(value)?, value)
}
pub fn tan(value: f64) -> Result<f64, PyError> {
    if value.is_infinite() {
        return Err(PyError::value("math domain error"));
    }
    domain(crate::correctly_rounded::tan(value)?, value)
}
pub fn sin(value: f64) -> Result<f64, PyError> {
    if value.is_infinite() {
        return Err(PyError::value("math domain error"));
    }
    domain(crate::correctly_rounded::sin(value)?, value)
}

/// `min(a, b)` and `max(a, b)` as Python evaluates them: the first unless the second compares below (above) it.
pub fn py_min(a: f64, b: f64) -> f64 {
    if b < a {
        b
    } else {
        a
    }
}
pub fn py_max(a: f64, b: f64) -> f64 {
    if b > a {
        b
    } else {
        a
    }
}
/// `max(values)` over a non-empty sequence.
pub fn py_max_all(values: &[f64]) -> f64 {
    let mut best = values[0];
    for &value in &values[1..] {
        if value > best {
            best = value;
        }
    }
    best
}

/// The importer's `correct_norm3`: RN(sqrt(a^2 + b^2 + c^2)), one rounding of the exact norm.
pub fn correct_norm3(a: f64, b: f64, c: f64) -> f64 {
    if a.is_infinite() || b.is_infinite() || c.is_infinite() {
        return f64::INFINITY;
    }
    if a.is_nan() || b.is_nan() || c.is_nan() {
        return f64::NAN;
    }
    let mut square = Q::zero();
    for value in [a, b, c] {
        let exact = Q::from_f64(value).expect("finite");
        square = square.add(&exact.mul(&exact));
    }
    if square.is_zero() {
        return 0.0;
    }
    let bits = square.numer_bits() as i64 - square.denom_bits() as i64;
    let mut k = (105 - bits).div_euclid(2);
    let lower = BigInt::one() << 52usize;
    let upper = BigInt::one() << 53usize;
    let isqrt_floor = |scaled: &Q| -> BigInt { scaled.numer_ref().div_floor(scaled.denom_ref()).sqrt() };
    let mut scaled;
    let mut root;
    loop {
        scaled = square.mul(&Q::pow2(2 * k));
        root = isqrt_floor(&scaled);
        if root >= upper {
            k -= 1;
        } else if root < lower {
            k += 1;
        } else {
            break;
        }
    }
    if k > 1074 {
        k = 1074;
        scaled = square.mul(&Q::pow2(2 * k));
        root = isqrt_floor(&scaled);
    }
    let midpoint = Q::from_bigint(&root * &root + &root).add(&Q::new(BigInt::one(), BigInt::from(4)));
    let odd = root.is_odd();
    if scaled > midpoint || (scaled.cmp(&midpoint) == Ordering::Equal && odd) {
        root += 1;
    }
    // ldexp(root, -k): exact on the lattice 2^-k, or OverflowError (inf)
    let denominator = BigInt::one() << (k.max(0) as usize);
    let numerator = if k < 0 { &root << ((-k) as usize) } else { root.clone() };
    let _ = root.to_u64();
    ratio_to_f64(&numerator, &denominator).unwrap_or(f64::INFINITY)
}
