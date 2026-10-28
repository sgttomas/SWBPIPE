//! Exact rationals with Python `fractions.Fraction` semantics for what the importer computes, and Python's exact
//! conversions between rationals, integers and binary64 (`float(Fraction)` and `float(int)` round once, half to even).

use crate::pystr::{decimal_value, PyError};
use num_bigint::{BigInt, Sign};
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::cmp::Ordering;

/// Python 3.11+'s default `sys.get_int_max_str_digits()`.
pub const INT_MAX_STR_DIGITS: usize = 4300;

/// A reduced fraction n/d with d > 0.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Q {
    n: BigInt,
    d: BigInt,
}

/// Python's `int(text)` of a run of decimal digits (any Unicode `\d`), with its digit limit.
pub fn py_int_digits(text: &str) -> Result<BigInt, PyError> {
    let count = text.chars().count();
    if count > INT_MAX_STR_DIGITS {
        return Err(PyError::value(format!(
            "Exceeds the limit ({INT_MAX_STR_DIGITS} digits) for integer string conversion: value has {count} digits; \
             use sys.set_int_max_str_digits() to increase the limit"
        )));
    }
    let mut out = BigInt::zero();
    let ten = BigInt::from(10u8);
    for character in text.chars() {
        let digit = decimal_value(character).ok_or_else(|| PyError::value(format!("invalid digit {character:?}")))?;
        out = out * &ten + BigInt::from(digit);
    }
    Ok(out)
}

pub fn pow10(exponent: u64) -> BigInt {
    num_traits::pow(BigInt::from(10u8), exponent as usize)
}

impl Q {
    pub fn new(n: BigInt, d: BigInt) -> Q {
        assert!(!d.is_zero(), "Fraction(_, 0)");
        let (mut n, mut d) = if d.is_negative() { (-n, -d) } else { (n, d) };
        let g = n.gcd(&d);
        if !g.is_one() && !g.is_zero() {
            n /= &g;
            d /= &g;
        }
        Q { n, d }
    }
    pub fn zero() -> Q {
        Q { n: BigInt::zero(), d: BigInt::one() }
    }
    pub fn from_int(value: i64) -> Q {
        Q { n: BigInt::from(value), d: BigInt::one() }
    }
    pub fn from_bigint(value: BigInt) -> Q {
        Q { n: value, d: BigInt::one() }
    }
    pub fn numer(&self) -> BigInt {
        self.n.clone()
    }
    pub fn numer_ref(&self) -> &BigInt {
        &self.n
    }
    pub fn denom_ref(&self) -> &BigInt {
        &self.d
    }
    pub fn is_zero(&self) -> bool {
        self.n.is_zero()
    }
    pub fn is_integer(&self) -> bool {
        self.d.is_one()
    }
    pub fn is_negative(&self) -> bool {
        self.n.is_negative()
    }
    pub fn abs(&self) -> Q {
        Q { n: self.n.abs(), d: self.d.clone() }
    }
    pub fn neg(&self) -> Q {
        Q { n: -self.n.clone(), d: self.d.clone() }
    }
    pub fn add(&self, other: &Q) -> Q {
        Q::new(&self.n * &other.d + &other.n * &self.d, &self.d * &other.d)
    }
    pub fn sub(&self, other: &Q) -> Q {
        Q::new(&self.n * &other.d - &other.n * &self.d, &self.d * &other.d)
    }
    pub fn mul(&self, other: &Q) -> Q {
        Q::new(&self.n * &other.n, &self.d * &other.d)
    }
    pub fn div(&self, other: &Q) -> Q {
        Q::new(&self.n * &other.d, &self.d * &other.n)
    }
    pub fn mul_int(&self, value: i64) -> Q {
        Q::new(&self.n * BigInt::from(value), self.d.clone())
    }
    /// 2^k for any integer k.
    pub fn pow2(k: i64) -> Q {
        if k >= 0 {
            Q { n: BigInt::one() << (k as usize), d: BigInt::one() }
        } else {
            Q { n: BigInt::one(), d: BigInt::one() << ((-k) as usize) }
        }
    }
    /// 10^k for any integer k.
    pub fn pow10(k: i64) -> Q {
        if k >= 0 {
            Q { n: pow10(k as u64), d: BigInt::one() }
        } else {
            Q { n: BigInt::one(), d: pow10((-k) as u64) }
        }
    }
    /// `Fraction(x)` for a finite binary64 x (exact); None for inf or NaN.
    pub fn from_f64(value: f64) -> Option<Q> {
        if !value.is_finite() {
            return None;
        }
        if value == 0.0 {
            return Some(Q::zero());
        }
        let bits = value.to_bits();
        let negative = bits >> 63 == 1;
        let exponent = ((bits >> 52) & 0x7ff) as i64;
        let fraction = bits & ((1u64 << 52) - 1);
        let (mantissa, power) = if exponent == 0 { (fraction, -1074) } else { (fraction | (1u64 << 52), exponent - 1075) };
        let magnitude = Q::from_bigint(BigInt::from(mantissa)).mul(&Q::pow2(power));
        Some(if negative { magnitude.neg() } else { magnitude })
    }
    /// Python's `int.bit_length()` of numerator and denominator.
    pub fn numer_bits(&self) -> u64 {
        self.n.bits()
    }
    pub fn denom_bits(&self) -> u64 {
        self.d.bits()
    }
    /// `float(self)`: n / d rounded once to binary64, half to even; Err (Python's OverflowError) when it rounds
    /// beyond the largest finite binary64.
    pub fn to_f64(&self) -> Result<f64, PyError> {
        ratio_to_f64(&self.n, &self.d).ok_or_else(|| PyError::overflow("integer division result too large for a float"))
    }
}

impl PartialOrd for Q {
    fn partial_cmp(&self, other: &Q) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Q {
    fn cmp(&self, other: &Q) -> Ordering {
        (&self.n * &other.d).cmp(&(&other.n * &self.d))
    }
}

/// `float(int)` (Python's OverflowError as None).
pub fn bigint_to_f64(value: &BigInt) -> Option<f64> {
    ratio_to_f64(value, &BigInt::one())
}

/// The binary64 nearest n/d (d > 0), ties to even, subnormals on their own lattice; None beyond the finite range.
pub fn ratio_to_f64(n: &BigInt, d: &BigInt) -> Option<f64> {
    if n.is_zero() {
        return Some(0.0);
    }
    let negative = n.sign() == Sign::Minus;
    let a = n.abs();
    let b = d.clone();
    // Choose e so that a / (b 2^e) lies in [2^52, 2^54), then fix it to [2^52, 2^53).
    let mut e: i64 = a.bits() as i64 - b.bits() as i64 - 53;
    loop {
        let q = scaled_floor(&a, &b, e);
        if q.bits() > 53 {
            e += 1;
        } else if q.bits() < 53 {
            e -= 1;
        } else {
            break;
        }
    }
    if e < -1074 {
        e = -1074;
    }
    let (q, exact) = scaled_divmod(&a, &b, e);
    let mut q = q.to_u64().expect("53 bits");
    // Round half to even on the remainder.
    match exact {
        RemainderClass::Zero => {}
        RemainderClass::Below => {}
        RemainderClass::Half => {
            if q & 1 == 1 {
                q += 1;
            }
        }
        RemainderClass::Above => q += 1,
    }
    if q == 1u64 << 53 {
        q >>= 1;
        e += 1;
    }
    // value = q 2^e with q < 2^53.
    if q >= 1u64 << 52 {
        let biased = e + 52 + 1023;
        if biased >= 0x7ff {
            return None;
        }
        let bits = ((biased as u64) << 52) | (q - (1u64 << 52));
        let value = f64::from_bits(bits);
        return Some(if negative { -value } else { value });
    }
    // Subnormal (e == -1074) or zero.
    let value = f64::from_bits(q);
    Some(if negative { -value } else { value })
}

enum RemainderClass {
    Zero,
    Below,
    Half,
    Above,
}

/// floor(a / (b 2^e)).
fn scaled_floor(a: &BigInt, b: &BigInt, e: i64) -> BigInt {
    if e >= 0 {
        a / (b << (e as usize))
    } else {
        (a << ((-e) as usize)) / b
    }
}

fn scaled_divmod(a: &BigInt, b: &BigInt, e: i64) -> (BigInt, RemainderClass) {
    let (numerator, denominator) = if e >= 0 { (a.clone(), b << (e as usize)) } else { (a << ((-e) as usize), b.clone()) };
    let (q, r) = numerator.div_rem(&denominator);
    let class = if r.is_zero() {
        RemainderClass::Zero
    } else {
        match (&r << 1usize).cmp(&denominator) {
            Ordering::Less => RemainderClass::Below,
            Ordering::Equal => RemainderClass::Half,
            Ordering::Greater => RemainderClass::Above,
        }
    };
    (q, class)
}

/// The parts of a decimal numeral the importer reads: sign, integer digits, fraction digits and exponent digits
/// (each a run of Python `\d` characters), as `Fraction(text)` reads them.
pub struct DecimalParts<'a> {
    pub negative: bool,
    pub whole: &'a str,
    pub fraction: Option<&'a str>,
    pub exponent: Option<(bool, &'a str)>,
}

/// `Fraction(text)` for `[+-]?(digits)?(.digits?)?([eE][+-]?digits)?` with at least one mantissa digit, as the
/// importer only ever passes such text. Huge exponents with a non-zero mantissa return `Huge` instead of computing
/// a power of ten Python would also compute only to refuse the value as outside binary64.
pub enum Parsed {
    Exact(Q),
    /// A non-zero value whose decimal exponent puts it far outside binary64 (the caller refuses it as overflowing
    /// or underflowing, as Python does after computing it).
    Huge,
}

pub fn split_decimal(text: &str) -> Option<DecimalParts<'_>> {
    let (negative, rest) = match text.chars().next()? {
        '+' => (false, &text[1..]),
        '-' => (true, &text[1..]),
        _ => (false, text),
    };
    let (mantissa, exponent) = match rest.find(['e', 'E']) {
        Some(at) => (&rest[..at], Some(&rest[at + 1..])),
        None => (rest, None),
    };
    let (whole, fraction) = match mantissa.find('.') {
        Some(at) => (&mantissa[..at], Some(&mantissa[at + 1..])),
        None => (mantissa, None),
    };
    let exponent = match exponent {
        None => None,
        Some(text) => {
            let (minus, digits) = match text.chars().next()? {
                '+' => (false, &text[1..]),
                '-' => (true, &text[1..]),
                _ => (false, text),
            };
            if digits.is_empty() {
                return None;
            }
            Some((minus, digits))
        }
    };
    Some(DecimalParts { negative, whole, fraction, exponent })
}

/// `Fraction(text)` of a decimal numeral (see [`Parsed`]); Err for Python's int() digit limit. A value far outside
/// binary64 is `Huge`: only real() reads this form, which refuses such a value whatever its exact digits.
pub fn parse_fraction(text: &str) -> Result<Parsed, PyError> {
    parse_fraction_with(text, true)
}

fn parse_fraction_with(text: &str, bounded: bool) -> Result<Parsed, PyError> {
    let parts = split_decimal(text).ok_or_else(|| PyError::value(format!("Invalid literal for Fraction: {text:?}")))?;
    let mut numerator = if parts.whole.is_empty() { BigInt::zero() } else { py_int_digits(parts.whole)? };
    let mut scale: i64 = 0;
    if let Some(fraction) = parts.fraction {
        if !fraction.is_empty() {
            let value = py_int_digits(fraction)?;
            let length = fraction.chars().count() as u64;
            numerator = numerator * pow10(length) + value;
            scale = -(length as i64);
        }
    }
    let exponent = match parts.exponent {
        None => BigInt::zero(),
        Some((minus, digits)) => {
            let value = py_int_digits(digits)?;
            if minus {
                -value
            } else {
                value
            }
        }
    };
    if numerator.is_zero() {
        return Ok(Parsed::Exact(Q::zero()));
    }
    let total = exponent + BigInt::from(scale);
    let digits = numerator.to_string().len() as i64;
    match total.to_i64() {
        // A signed range test: `abs()` overflows on `i64::MIN`, which an untrusted exponent can reach (review r1).
        Some(power) if (-100_000..=100_000).contains(&power) => {}
        _ => return Ok(Parsed::Huge),
    }
    let power = total.to_i64().expect("bounded");
    if bounded && (power + digits > 400 || power + digits < -400) {
        return Ok(Parsed::Huge);
    }
    let magnitude = Q::from_bigint(numerator).mul(&Q::pow10(power));
    Ok(Parsed::Exact(if parts.negative { magnitude.neg() } else { magnitude }))
}

/// `Fraction(text) == 0` for a decimal numeral.
pub fn fraction_is_zero(text: &str) -> Result<bool, PyError> {
    Ok(match parse_fraction(text)? {
        Parsed::Exact(value) => value.is_zero(),
        Parsed::Huge => false,
    })
}

/// `Fraction(text)` exactly, whatever its magnitude (English feet, and numerals real() has admitted). A decimal
/// exponent beyond 10^5 is not computed: no such numeral reaches these uses, which have no exponent or a bounded one.
pub fn fraction_of(text: &str) -> Result<Q, PyError> {
    match parse_fraction_with(text, false)? {
        Parsed::Exact(value) => Ok(value),
        Parsed::Huge => Err(PyError::value("a decimal exponent beyond 10^5 is not computed")),
    }
}
