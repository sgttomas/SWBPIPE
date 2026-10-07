//! One correctly rounded summation function (S11 containment, section 4.1).
//!
//! `ExactAccumulator` holds a signed fixed-point integer in units of 2^-2148,
//! the quantum of an exact product of two binary64 values. Every finite
//! binary64 operand and every exact product of two finite binary64 operands is
//! an integer multiple of that quantum. The largest exact product occupies at
//! most bit 4195; 64 further carry bits give 4260 bits, held in 68 limbs of 64
//! bits (4352 bits). Positive and negative contributions are kept in separate
//! unsigned magnitudes and subtracted once, at rounding.
//!
//! `round` projects the exact value once to binary64, to nearest with ties to
//! even, at the binary64 quantum of the result's binade, including the
//! subnormal binade (quantum 2^-1074). An exact zero, and a nonzero value that
//! rounds to zero, both give +0.0: `round` never returns -0.0. That +0.0 for a
//! negative underflow is a stated deviation from IEEE 754 (S11 section 4.1.1).
//! A value outside the binary64 range is `NonRepresentable`, never infinite.
//!
//! This does not recover information rounded before the operands were supplied.
const LIMBS: usize = 68;
/// Bit position of 2^0 in the accumulator (the quantum is 2^-2148).
const QUANTUM_EXPONENT: i64 = -2148;
type Magnitude = [u64; LIMBS];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SumError {
    NonFinite,
    AccumulatorOverflow,
    NonRepresentable,
}

impl std::fmt::Display for SumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFinite => write!(f, "exact sum operand is not finite"),
            Self::AccumulatorOverflow => write!(f, "exact sum accumulator overflow"),
            Self::NonRepresentable => write!(f, "exact sum is outside the binary64 range"),
        }
    }
}

impl std::error::Error for SumError {}

/// Exact signed fixed-point accumulator (quantum 2^-2148, 68 limbs).
#[derive(Debug, Clone)]
pub struct ExactAccumulator {
    positive: Magnitude,
    negative: Magnitude,
}

impl Default for ExactAccumulator {
    fn default() -> Self {
        Self::new()
    }
}

/// Integer significand and the accumulator bit position of its least bit.
fn decompose(value: f64) -> (u64, usize) {
    let bits = value.to_bits();
    let exponent = ((bits >> 52) & 0x7ff) as i64;
    let fraction = bits & ((1u64 << 52) - 1);
    let (significand, binary_exponent) = if exponent == 0 {
        (fraction, -1074)
    } else {
        (fraction | (1u64 << 52), exponent - 1075)
    };
    (significand, (binary_exponent - QUANTUM_EXPONENT) as usize)
}

fn add_word(value: &mut Magnitude, mut word: usize, mut add: u64) -> Result<(), SumError> {
    while add != 0 {
        let target = value.get_mut(word).ok_or(SumError::AccumulatorOverflow)?;
        let (next, carry) = target.overflowing_add(add);
        *target = next;
        add = u64::from(carry);
        word += 1;
    }
    Ok(())
}

/// Adds `integer * 2^position` to the magnitude, exactly.
fn add_shifted(value: &mut Magnitude, integer: u128, position: usize) -> Result<(), SumError> {
    let word = position / 64;
    let offset = (position % 64) as u32;
    let low = integer as u64;
    let high = (integer >> 64) as u64;
    let words = if offset == 0 {
        [low, high, 0]
    } else {
        [
            low << offset,
            (low >> (64 - offset)) | (high << offset),
            high >> (64 - offset),
        ]
    };
    for (k, part) in words.into_iter().enumerate() {
        if part != 0 {
            add_word(value, word + k, part)?;
        }
    }
    Ok(())
}

fn compare(a: &Magnitude, b: &Magnitude) -> std::cmp::Ordering {
    for i in (0..LIMBS).rev() {
        let order = a[i].cmp(&b[i]);
        if !order.is_eq() {
            return order;
        }
    }
    std::cmp::Ordering::Equal
}

fn subtract(a: &Magnitude, b: &Magnitude) -> Magnitude {
    let mut out = [0; LIMBS];
    let mut borrow = false;
    for i in 0..LIMBS {
        let (x, b1) = a[i].overflowing_sub(b[i]);
        let (y, b2) = x.overflowing_sub(u64::from(borrow));
        out[i] = y;
        borrow = b1 || b2;
    }
    debug_assert!(!borrow);
    out
}

fn bit(a: &Magnitude, index: i64) -> bool {
    if index < 0 || index >= (LIMBS * 64) as i64 {
        return false;
    }
    let index = index as usize;
    a[index / 64] & (1u64 << (index % 64)) != 0
}

/// Any set bit strictly below `end`.
fn any_below(a: &Magnitude, end: i64) -> bool {
    if end <= 0 {
        return false;
    }
    let end = (end as usize).min(LIMBS * 64);
    let full = end / 64;
    if a[..full].iter().any(|&x| x != 0) {
        return true;
    }
    let rem = end % 64;
    rem != 0 && a[full] & ((1u64 << rem) - 1) != 0
}

/// Bits [shift, shift + 64) as an integer; zero beyond the magnitude.
fn window(a: &Magnitude, shift: i64) -> u64 {
    let mut out = 0u64;
    for k in 0..64 {
        if bit(a, shift + k) {
            out |= 1u64 << k;
        }
    }
    out
}

/// Rounds `magnitude * 2^scale_exponent` (with the given sign) once to binary64.
fn project(a: &Magnitude, negative: bool, scale_exponent: i64) -> Result<f64, SumError> {
    let Some(word) = (0..LIMBS).rev().find(|&i| a[i] != 0) else {
        return Ok(0.0);
    };
    let highest = (word * 64 + 63 - a[word].leading_zeros() as usize) as i64;
    // Value lies in [2^e, 2^(e+1)).
    let e = highest + scale_exponent;
    // Binary64 quantum of that binade, including the subnormal binade.
    let mut quantum = (e - 52).max(-1074);
    // Accumulator bit index that carries the result's quantum.
    let position = quantum - scale_exponent;
    let mut significand = if position <= 0 {
        // Exact: at most 53 significant bits, shifted up.
        window(a, 0) << (-position)
    } else {
        let mut m = window(a, position) & ((1u64 << 54) - 1);
        let round = bit(a, position - 1);
        if round && (any_below(a, position - 1) || m & 1 != 0) {
            m += 1;
        }
        m
    };
    if significand == (1u64 << 53) {
        significand >>= 1;
        quantum += 1;
    }
    if significand == 0 {
        // A nonzero net below half the subnormal quantum: +0.0, never -0.0.
        return Ok(0.0);
    }
    let sign = if negative { 1u64 << 63 } else { 0 };
    let bits = if significand >= (1u64 << 52) {
        let biased = quantum + 52 + 1023;
        if biased >= 0x7ff {
            return Err(SumError::NonRepresentable);
        }
        ((biased as u64) << 52) | (significand - (1u64 << 52))
    } else {
        debug_assert_eq!(quantum, -1074);
        significand
    };
    Ok(f64::from_bits(sign | bits))
}

impl ExactAccumulator {
    pub fn new() -> Self {
        Self {
            positive: [0; LIMBS],
            negative: [0; LIMBS],
        }
    }

    /// Adds `value` exactly, as the integer `value * 2^2148`.
    pub fn add(&mut self, value: f64) -> Result<(), SumError> {
        if !value.is_finite() {
            return Err(SumError::NonFinite);
        }
        let (significand, position) = decompose(value);
        if significand == 0 {
            return Ok(());
        }
        let target = if value.is_sign_negative() {
            &mut self.negative
        } else {
            &mut self.positive
        };
        add_shifted(target, u128::from(significand), position)
    }

    /// Adds the exact product `a * b`: the 106-bit integer product of the
    /// significands, placed at the sum of the operands' exponents.
    pub fn add_product(&mut self, a: f64, b: f64) -> Result<(), SumError> {
        if !a.is_finite() || !b.is_finite() {
            return Err(SumError::NonFinite);
        }
        let (ma, pa) = decompose(a);
        let (mb, pb) = decompose(b);
        if ma == 0 || mb == 0 {
            return Ok(());
        }
        // pa + pb counts the quantum twice; the product's quantum is one 2^-2148.
        let position = pa + pb - (-QUANTUM_EXPONENT) as usize;
        let target = if a.is_sign_negative() != b.is_sign_negative() {
            &mut self.negative
        } else {
            &mut self.positive
        };
        add_shifted(target, u128::from(ma) * u128::from(mb), position)
    }

    /// True when the exact value is zero.
    pub fn is_zero(&self) -> bool {
        compare(&self.positive, &self.negative).is_eq()
    }

    /// Sign of the exact value: -1, 0 or 1.
    pub fn signum(&self) -> i8 {
        match compare(&self.positive, &self.negative) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }

    /// K4 (T3 D1 §4.1.2 item 5; ROOT's K4 ruling Q3): the exact value, netted
    /// once with `compare` and `subtract`, as (negative, magnitude, exponent of
    /// the magnitude's unit bit). The exponent is always -2148 (the quantum);
    /// an exact zero is (false, 0, -2148).
    pub(crate) fn net_parts(&self) -> (bool, Magnitude, i64) {
        match compare(&self.positive, &self.negative) {
            std::cmp::Ordering::Less => (
                true,
                subtract(&self.negative, &self.positive),
                QUANTUM_EXPONENT,
            ),
            _ => (
                false,
                subtract(&self.positive, &self.negative),
                QUANTUM_EXPONENT,
            ),
        }
    }

    /// The exact value, rounded once to binary64 (nearest, ties to even).
    pub fn round(&self) -> Result<f64, SumError> {
        self.round_scaled(0)
    }

    /// The exact value times `2^exponent`, rounded once to binary64. The power
    /// of two is applied before the single rounding, so no double rounding
    /// occurs when the unscaled value would be subnormal (S11 R3-4).
    pub fn round_scaled(&self, exponent: i32) -> Result<f64, SumError> {
        let scale = QUANTUM_EXPONENT + i64::from(exponent);
        match compare(&self.positive, &self.negative) {
            std::cmp::Ordering::Less => {
                project(&subtract(&self.negative, &self.positive), true, scale)
            }
            _ => project(&subtract(&self.positive, &self.negative), false, scale),
        }
    }
}

/// Correctly rounded sum of finite binary64 values; +0.0 for an exact zero.
pub fn exact_rounded_sum(values: impl IntoIterator<Item = f64>) -> Result<f64, SumError> {
    let mut accumulator = ExactAccumulator::new();
    for value in values {
        accumulator.add(value)?;
    }
    accumulator.round()
}

/// Correctly rounded sum of exact products; +0.0 for an exact zero.
pub fn exact_rounded_dot(pairs: impl IntoIterator<Item = (f64, f64)>) -> Result<f64, SumError> {
    let mut accumulator = ExactAccumulator::new();
    for (a, b) in pairs {
        accumulator.add_product(a, b)?;
    }
    accumulator.round()
}

#[cfg(test)]
mod tests;
