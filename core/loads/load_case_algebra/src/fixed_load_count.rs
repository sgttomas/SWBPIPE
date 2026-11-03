//! The fixed-load count κ of a combination (DEL-05-02 Work
//! `absolute-sum-srss-combinations.md` §5.1).
//!
//! A consuming constant-effort support's force, and a connector's or tie rod's
//! nonzero installed reference, act in every case state, so each case carries
//! every fixed load once. κ counts how many times a combination carries it:
//! - a load case: 1;
//! - `mechanics`: Σ fᵢκᵢ, exact;
//! - `result_state_subtraction`: κₘ − κₛ;
//! - `range_envelope`: the operands' common κ when all are equal exactly,
//!   otherwise undefined;
//! - `absolute_sum`, `srss`: 0 when every operand's κ is exactly 0, otherwise
//!   undefined.
//!
//! A combination with an undefined operand has an undefined κ.
//!
//! **Representation.** κ is an exact dyadic rational: a sign, an unsigned
//! integer significand of at most [`FIXED_LOAD_COUNT_MAX_BITS`] bits (held odd,
//! its trailing zeros moved into the exponent) and a binary exponent in `i64`.
//! Every operation (a product by a binary64 factor, a sum, a difference, a
//! comparison) is exact. A result needing more significand bits, or an exponent
//! outside `i64`, makes κ undefined as [`Undefined::Unrepresentable`]; no
//! residual is discarded. It is not held in `ExactAccumulator`, whose quantum
//! 2⁻²¹⁴⁸ cannot hold nested products such as (2⁻¹⁰⁷⁴)³.
//!
//! **Decisions.** The gates read κ through [`mechanics_fixed_load_gate`] (the
//! unit factor-sum rule with κᵢ in place of 1: |κ − 1| ≤ 2⁻⁴⁶, compared exactly)
//! and [`unsigned_fixed_load_gate`] (every operand's κ exactly 0). An undefined κ
//! never passes. Each gate's caller supplies its own fixed-load source and maps
//! the [`FixedLoadWithholding`] to that source's code.

use std::cmp::Ordering;

/// The most significand bits an exact κ may need (§5.1).
pub const FIXED_LOAD_COUNT_MAX_BITS: u64 = 4096;

const LIMB_BITS: u64 = 64;

/// An exact dyadic rational (−1)^negative × significand × 2^exponent, normalized:
/// zero is the empty significand with exponent 0 and no sign; otherwise the
/// significand is odd and has at most [`FIXED_LOAD_COUNT_MAX_BITS`] bits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dyadic {
    negative: bool,
    /// Little-endian 64-bit limbs, no high zero limb.
    significand: Vec<u64>,
    exponent: i64,
}

/// Why a κ is undefined.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Undefined {
    /// The exact result needs more than [`FIXED_LOAD_COUNT_MAX_BITS`] significand
    /// bits (or an exponent outside `i64`). The combination whose own κ this is
    /// is named (`COMBINATION_FIXED_LOAD_COUNT_UNREPRESENTABLE`).
    Unrepresentable,
    /// A range envelope whose operands' κ are not all equal.
    EnvelopeMixed,
    /// An absolute sum or SRSS with an operand whose κ is not exactly 0.
    UnsignedNonzero,
    /// An operand's κ is undefined.
    Operand,
}

/// A combination's fixed-load count.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FixedLoadCount {
    Defined(Dyadic),
    Undefined(Undefined),
}

/// Why a gated use withholds a combination; the caller names it with its
/// fixed-load source's code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixedLoadWithholding {
    /// `mechanics`: |κ − 1| > 2⁻⁴⁶, or its own κ is unrepresentable although its
    /// operands' are defined (the unit rule cannot be decided).
    UnitFactorSum,
    /// `mechanics` naming an operand whose κ is undefined.
    OperandUndefined,
    /// `absolute_sum` or `srss` with an operand whose κ is not exactly 0, or is
    /// undefined.
    UnsignedOperand,
}

fn bit_length(limbs: &[u64]) -> u64 {
    match limbs.last() {
        None => 0,
        Some(top) => (limbs.len() as u64 - 1) * LIMB_BITS + (LIMB_BITS - top.leading_zeros() as u64),
    }
}

fn trim(limbs: &mut Vec<u64>) {
    while limbs.last() == Some(&0) {
        limbs.pop();
    }
}

fn compare(a: &[u64], b: &[u64]) -> Ordering {
    a.len().cmp(&b.len()).then_with(|| a.iter().rev().cmp(b.iter().rev()))
}

fn shift_left(limbs: &[u64], bits: u64) -> Vec<u64> {
    if limbs.is_empty() {
        return Vec::new();
    }
    let whole = (bits / LIMB_BITS) as usize;
    let part = (bits % LIMB_BITS) as u32;
    let mut out = vec![0u64; whole];
    if part == 0 {
        out.extend_from_slice(limbs);
    } else {
        let mut carry = 0u64;
        for &limb in limbs {
            out.push((limb << part) | carry);
            carry = limb >> (LIMB_BITS as u32 - part);
        }
        out.push(carry);
    }
    trim(&mut out);
    out
}

fn add_magnitudes(a: &[u64], b: &[u64]) -> Vec<u64> {
    let (long, short) = if a.len() >= b.len() { (a, b) } else { (b, a) };
    let mut out = Vec::with_capacity(long.len() + 1);
    let mut carry = false;
    for (index, &limb) in long.iter().enumerate() {
        let (sum, c1) = limb.overflowing_add(*short.get(index).unwrap_or(&0));
        let (sum, c2) = sum.overflowing_add(carry as u64);
        out.push(sum);
        carry = c1 || c2;
    }
    if carry {
        out.push(1);
    }
    out
}

/// `a − b` for `a ≥ b`.
fn subtract_magnitudes(a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut out = Vec::with_capacity(a.len());
    let mut borrow = false;
    for (index, &limb) in a.iter().enumerate() {
        let (difference, b1) = limb.overflowing_sub(*b.get(index).unwrap_or(&0));
        let (difference, b2) = difference.overflowing_sub(borrow as u64);
        out.push(difference);
        borrow = b1 || b2;
    }
    debug_assert!(!borrow);
    trim(&mut out);
    out
}

fn multiply_by(limbs: &[u64], factor: u64) -> Vec<u64> {
    let mut out = Vec::with_capacity(limbs.len() + 1);
    let mut carry = 0u128;
    for &limb in limbs {
        let product = limb as u128 * factor as u128 + carry;
        out.push(product as u64);
        carry = product >> 64;
    }
    out.push(carry as u64);
    trim(&mut out);
    out
}

impl Dyadic {
    /// Exact zero.
    pub fn zero() -> Self {
        Dyadic { negative: false, significand: Vec::new(), exponent: 0 }
    }

    /// Exact one.
    pub fn one() -> Self {
        Dyadic { negative: false, significand: vec![1], exponent: 0 }
    }

    /// Normalizes a signed magnitude × 2^exponent; `None` when it needs more than
    /// [`FIXED_LOAD_COUNT_MAX_BITS`] significand bits or its exponent leaves `i64`.
    fn normalized(negative: bool, mut significand: Vec<u64>, exponent: i64) -> Option<Self> {
        trim(&mut significand);
        if significand.is_empty() {
            return Some(Self::zero());
        }
        let zeros = {
            let whole = significand.iter().take_while(|&&limb| limb == 0).count();
            whole as u64 * LIMB_BITS + significand[whole].trailing_zeros() as u64
        };
        let exponent = exponent.checked_add(i64::try_from(zeros).ok()?)?;
        let significand = if zeros == 0 { significand } else { shift_right(&significand, zeros) };
        (bit_length(&significand) <= FIXED_LOAD_COUNT_MAX_BITS).then_some(Dyadic { negative, significand, exponent })
    }

    /// The exact value of a finite binary64 number; `None` for NaN or an infinity.
    pub fn from_f64(value: f64) -> Option<Self> {
        if !value.is_finite() {
            return None;
        }
        let bits = value.to_bits();
        let negative = bits >> 63 == 1;
        let biased = ((bits >> 52) & 0x7ff) as i64;
        let fraction = bits & ((1u64 << 52) - 1);
        let (mantissa, exponent) = if biased == 0 { (fraction, -1074) } else { (fraction | (1u64 << 52), biased - 1075) };
        Self::normalized(negative && mantissa != 0, vec![mantissa], exponent)
    }

    pub fn is_zero(&self) -> bool {
        self.significand.is_empty()
    }

    /// The significand's bit length (0 for zero).
    pub fn significand_bits(&self) -> u64 {
        bit_length(&self.significand)
    }

    /// The exact product by a binary64 factor; `None` when unrepresentable or the
    /// factor is not finite.
    pub fn times_f64(&self, factor: f64) -> Option<Self> {
        let factor = Self::from_f64(factor)?;
        if self.is_zero() || factor.is_zero() {
            return Some(Self::zero());
        }
        // A normalized binary64 significand fits in one limb.
        let product = multiply_by(&self.significand, factor.significand[0]);
        Self::normalized(self.negative != factor.negative, product, self.exponent.checked_add(factor.exponent)?)
    }

    /// The exact sum; `None` when unrepresentable.
    pub fn plus(&self, other: &Self) -> Option<Self> {
        if self.is_zero() {
            return Some(other.clone());
        }
        if other.is_zero() {
            return Some(self.clone());
        }
        let exponent = self.exponent.min(other.exponent);
        // Both significands are odd, so an operand shifted by d > 0 bits is a
        // multiple of 2^d added to an odd number: the sum is odd at `exponent` and
        // at least 2^d − 2^4096 in magnitude, so it needs at least d bits. A shift
        // beyond the bound is therefore refused before it is formed.
        let shift = |value: &Self| -> Option<Vec<u64>> {
            let by = u64::try_from(value.exponent.checked_sub(exponent)?).ok()?;
            (by <= FIXED_LOAD_COUNT_MAX_BITS + 1).then(|| shift_left(&value.significand, by))
        };
        let a = shift(self)?;
        let b = shift(other)?;
        let (negative, magnitude) = if self.negative == other.negative {
            (self.negative, add_magnitudes(&a, &b))
        } else {
            match compare(&a, &b) {
                Ordering::Equal => return Some(Self::zero()),
                Ordering::Greater => (self.negative, subtract_magnitudes(&a, &b)),
                Ordering::Less => (other.negative, subtract_magnitudes(&b, &a)),
            }
        };
        Self::normalized(negative, magnitude, exponent)
    }

    /// The exact negation.
    pub fn negated(&self) -> Self {
        Dyadic { negative: !self.negative && !self.is_zero(), ..self.clone() }
    }

    /// The exact difference; `None` when unrepresentable.
    pub fn minus(&self, other: &Self) -> Option<Self> {
        self.plus(&other.negated())
    }

    /// The exact sign: −1, 0 or 1.
    pub fn signum(&self) -> i32 {
        if self.is_zero() {
            0
        } else if self.negative {
            -1
        } else {
            1
        }
    }

    /// The exact comparison of magnitudes |self| and |other|.
    pub fn compare_magnitude(&self, other: &Self) -> Ordering {
        if self.is_zero() || other.is_zero() {
            return self.significand.len().min(1).cmp(&other.significand.len().min(1));
        }
        // Compare the top bit positions first, then the aligned significands.
        let top = |v: &Self| v.exponent as i128 + bit_length(&v.significand) as i128;
        match top(self).cmp(&top(other)) {
            Ordering::Equal => {}
            unequal => return unequal,
        }
        let exponent = self.exponent.min(other.exponent);
        // Equal tops, so each shift is below the other's bit length: bounded.
        let a = shift_left(&self.significand, (self.exponent - exponent) as u64);
        let b = shift_left(&other.significand, (other.exponent - exponent) as u64);
        compare(&a, &b)
    }
}

fn shift_right(limbs: &[u64], bits: u64) -> Vec<u64> {
    let whole = (bits / LIMB_BITS) as usize;
    let part = (bits % LIMB_BITS) as u32;
    let rest = &limbs[whole.min(limbs.len())..];
    let mut out: Vec<u64> = if part == 0 {
        rest.to_vec()
    } else {
        rest.iter()
            .enumerate()
            .map(|(index, &limb)| (limb >> part) | rest.get(index + 1).map_or(0, |next| next << (LIMB_BITS as u32 - part)))
            .collect()
    };
    trim(&mut out);
    out
}

impl FixedLoadCount {
    /// A load case: 1.
    pub fn load_case() -> Self {
        FixedLoadCount::Defined(Dyadic::one())
    }

    /// The defined value, if any.
    pub fn defined(&self) -> Option<&Dyadic> {
        match self {
            FixedLoadCount::Defined(value) => Some(value),
            FixedLoadCount::Undefined(_) => None,
        }
    }

    /// Defined and exactly 0.
    pub fn is_exact_zero(&self) -> bool {
        self.defined().is_some_and(Dyadic::is_zero)
    }

    /// Whether this κ is undefined because its own exact result is unrepresentable
    /// (an operand's undefined κ is reported at that operand).
    pub fn is_unrepresentable(&self) -> bool {
        *self == FixedLoadCount::Undefined(Undefined::Unrepresentable)
    }

    fn of(value: Option<Dyadic>) -> Self {
        value.map_or(FixedLoadCount::Undefined(Undefined::Unrepresentable), FixedLoadCount::Defined)
    }

    /// `mechanics`: Σ fᵢκᵢ over (factor, operand κ), exact. A non-finite factor
    /// (refused by validation first) is unrepresentable.
    pub fn mechanics<'a>(terms: impl IntoIterator<Item = (f64, &'a FixedLoadCount)>) -> Self {
        let mut sum = Dyadic::zero();
        let mut overflowed = false;
        for (factor, operand) in terms {
            let Some(value) = operand.defined() else {
                return FixedLoadCount::Undefined(Undefined::Operand);
            };
            if overflowed {
                continue;
            }
            match value.times_f64(factor).and_then(|product| sum.plus(&product)) {
                Some(next) => sum = next,
                // Keep reading: an undefined operand later still names the operand.
                None => overflowed = true,
            }
        }
        if overflowed {
            FixedLoadCount::Undefined(Undefined::Unrepresentable)
        } else {
            FixedLoadCount::Defined(sum)
        }
    }

    /// `result_state_subtraction`: κₘ − κₛ.
    pub fn subtraction(minuend: &FixedLoadCount, subtrahend: &FixedLoadCount) -> Self {
        match (minuend.defined(), subtrahend.defined()) {
            (Some(m), Some(s)) => Self::of(m.minus(s)),
            _ => FixedLoadCount::Undefined(Undefined::Operand),
        }
    }

    /// `range_envelope`: the operands' common κ when all are equal exactly.
    pub fn envelope<'a>(operands: impl IntoIterator<Item = &'a FixedLoadCount>) -> Self {
        let mut common: Option<&Dyadic> = None;
        let mut mixed = false;
        for operand in operands {
            let Some(value) = operand.defined() else {
                return FixedLoadCount::Undefined(Undefined::Operand);
            };
            match common {
                None => common = Some(value),
                Some(first) if first != value => mixed = true,
                Some(_) => {}
            }
        }
        match (mixed, common) {
            (false, Some(value)) => FixedLoadCount::Defined(value.clone()),
            // Validation refuses an empty envelope; it carries no fixed load.
            (false, None) => FixedLoadCount::Defined(Dyadic::zero()),
            (true, _) => FixedLoadCount::Undefined(Undefined::EnvelopeMixed),
        }
    }

    /// `absolute_sum` and `srss`: 0 when every operand's κ is exactly 0.
    pub fn unsigned<'a>(operands: impl IntoIterator<Item = &'a FixedLoadCount>) -> Self {
        let mut nonzero = false;
        for operand in operands {
            match operand.defined() {
                None => return FixedLoadCount::Undefined(Undefined::Operand),
                Some(value) if !value.is_zero() => nonzero = true,
                Some(_) => {}
            }
        }
        if nonzero {
            FixedLoadCount::Undefined(Undefined::UnsignedNonzero)
        } else {
            FixedLoadCount::Defined(Dyadic::zero())
        }
    }
}

/// The unit rule's bound, 2⁻⁴⁶ (`UNIT_FACTOR_SUM_BOUND`), exactly.
fn unit_bound() -> Dyadic {
    Dyadic { negative: false, significand: vec![1], exponent: -46 }
}

/// Whether a defined κ satisfies the unit rule |κ − 1| ≤ 2⁻⁴⁶, compared exactly.
pub fn within_unit_factor_sum(kappa: &Dyadic) -> bool {
    match kappa.minus(&Dyadic::one()) {
        Some(offset) => offset.compare_magnitude(&unit_bound()) != Ordering::Greater,
        // κ − 1 is not representable. Then κ's exponent e and 0 differ by more than
        // 4,097, or the exact difference is wider than 4,096 bits, and in each case
        // κ < 1/2 or κ ≥ 2⁴⁰⁹⁶ (κ's own significand has at most 4,096 bits), so the
        // rule fails. It is still decided by exact comparison with the two bounds,
        // which never needs a representation wider than κ's.
        None => between_unit_bounds(kappa),
    }
}

/// 1 − 2⁻⁴⁶ ≤ κ ≤ 1 + 2⁻⁴⁶, by exact magnitude comparison with the two bounds.
fn between_unit_bounds(kappa: &Dyadic) -> bool {
    if kappa.signum() <= 0 {
        return false;
    }
    let one = Dyadic::one();
    let bound = unit_bound();
    let (Some(upper), Some(lower)) = (one.plus(&bound), one.minus(&bound)) else {
        return false;
    };
    kappa.compare_magnitude(&upper) != Ordering::Greater && kappa.compare_magnitude(&lower) != Ordering::Less
}

/// The fixed-load gate of a `mechanics` combination with a fixed-load source
/// (§5.1): `OperandUndefined` when an operand's κ is undefined, `UnitFactorSum`
/// when its own κ is undefined or |κ − 1| > 2⁻⁴⁶; `None` admits.
pub fn mechanics_fixed_load_gate<'a>(own: &FixedLoadCount, operands: impl IntoIterator<Item = &'a FixedLoadCount>) -> Option<FixedLoadWithholding> {
    if operands.into_iter().any(|operand| operand.defined().is_none()) {
        return Some(FixedLoadWithholding::OperandUndefined);
    }
    match own.defined() {
        Some(kappa) if within_unit_factor_sum(kappa) => None,
        _ => Some(FixedLoadWithholding::UnitFactorSum),
    }
}

/// The fixed-load gate of an `absolute_sum` or `srss` combination with a fixed-load
/// source (§5.1): `UnsignedOperand` unless every operand's κ is exactly 0 (never a
/// tolerance; an undefined κ never passes).
pub fn unsigned_fixed_load_gate<'a>(operands: impl IntoIterator<Item = &'a FixedLoadCount>) -> Option<FixedLoadWithholding> {
    if operands.into_iter().all(FixedLoadCount::is_exact_zero) {
        None
    } else {
        Some(FixedLoadWithholding::UnsignedOperand)
    }
}

/// Decision OQ-1 (§5.2): in a model with nonlinear supports a `mechanics`
/// combination is admitted exactly when one term names a load case with factor
/// exactly 1, it has at least one other term, and every other term names a
/// combination whose κ is defined and exactly 0. `terms` are (names a load case,
/// factor, the operand's κ) in authored order.
pub fn solved_state_plus_increments<'a>(terms: impl IntoIterator<Item = (bool, f64, &'a FixedLoadCount)>) -> bool {
    let mut base = 0usize;
    let mut others = 0usize;
    for (load_case, factor, kappa) in terms {
        if load_case {
            // Exactly 1 (a −0.0 or 1 ± ulp is not 1); a second load case fails.
            if factor != 1.0 {
                return false;
            }
            base += 1;
        } else if kappa.is_exact_zero() {
            others += 1;
        } else {
            return false;
        }
    }
    base == 1 && others >= 1
}
