//! `Wide<L>`: binary floating point with an `i64` exponent and an L-limb
//! significand, rounded to a runtime precision (T3 D1 §4.1.1, §4.11; slice K3a).
//!
//! K3a implements `Wide<2>` only (128-bit significand, p ≤ 128). The rest of
//! the family (L = 4, 8, 16) and the rounded conversion back to binary64 are
//! slice K3.
//!
//! # Value model
//!
//! A value is `(-1)^negative · m · 2^(exponent − 127)` with `m` the 128-bit
//! significand held little-endian in `[u64; 2]`. A nonzero value is
//! normalized (bit 127 of `m` set), so `exponent` is the exponent of its
//! leading bit. Zero has `m = 0` and exponent 0 and keeps its sign. There are
//! no subnormals, infinities or NaNs. `==` is bitwise identity, so +0 and −0
//! differ; `cmp_value` compares numerically.
//!
//! # Arithmetic
//!
//! `WideArith` holds the precision p (2 ≤ p ≤ 128) and the work counter.
//! `add`, `sub`, `mul`, `div` and `sqrt` each return the exact result of the
//! operation on the exact operands (whatever their bit count), rounded once
//! to p bits, to nearest, ties to even. Signs of zero follow IEEE 754 under
//! round-to-nearest: an exact zero sum of opposite-signed operands is +0,
//! (−0) + (−0) = −0, products and quotients carry the xor of the signs, and
//! √(−0) = −0. Division by zero and the square root of a negative nonzero
//! value are refused.
//!
//! Only integer operations are used (`u64`/`u128` limbs, shifts, compares,
//! and bit-serial division and square root), so every result is bitwise
//! reproducible on every platform. Exponent arithmetic is done in `i128` and
//! every result exponent is checked against ±2^62: a result outside that range
//! is `WideError::ExponentRange`, never a wrapped value.
//!
//! # Work counter
//!
//! `WorkCounter` counts every correctly rounded operation by kind (including
//! those inside the arctangent) and every arctangent call. The counts saturate
//! and never wrap. They are the unit of work for the budgets of D1 §4.1.7.
//!
//! # The included-angle arctangent (C1)
//!
//! `WideArith::included_angle(s, c)` returns φ = 2·atan(s/(1 + c)) for s > 0
//! and 1 + c > 0, the open interval 0 < φ < π (D1 §4.3.1, `R5_4_CURVED.md`
//! §2 step 3). It forms t = s/(1 + c), applies the half-angle step
//! t ← t/(1 + √(1 + t²)) while t ≥ 1/20 (compared exactly), sums the
//! arctangent series t − t³/3 + t⁵/5 − … and scales by 2^(k+1). The series
//! stops before the first term n ≥ 1 whose computed magnitude is below
//! 2^(e_t − p − 1) (e_t the exponent of t); its terms are summed smallest
//! first, and t is added last. Every operation is rounded to p. At most 5
//! reductions and at most 15 series terms are used; both limits are enforced
//! (an internal error, unreachable by the proof below).
//!
//! **Proved bound: the accuracy contract (53 ≤ p ≤ 128), u = 2^−p.** For
//! exact inputs, the result φ̂ satisfies |φ̂ − φ| ≤ 23.55·u·φ, hence
//! |φ̂ − φ| ≤ 23.6 ulp_p(φ̂), or the call is refused with `ExponentRange`
//! (when t² or a series power leaves ±2^62, roughly e_t > 2^61 or
//! e_t < −2^62/3; never a wrong value); for `atan_positive` the bound is
//! (1.54 + 4k)·u ≤ 21.54·u relative, with the same refusals. This is the
//! bound K-D5 may cite. Proof, with
//! rnd(x) = x(1 + δ), |δ| ≤ u, and γ_n = nu/(1 − nu):
//! 1. (Lemma A) For t, t̃ > 0 with t̃ = t(1 + η), |η| ≤ 1/2:
//!    |atan t̃ − atan t| ≤ λ(η)·atan t with λ(η) = |η|/(1 − |η|)². By the mean
//!    value theorem the difference is |η|t/(1 + ξ²) with ξ ≥ t(1 − |η|), and
//!    t/(1 + t²) ≤ atan t for t ≥ 0 (the difference has derivative
//!    2t²/(1 + t²)² ≥ 0 and vanishes at 0).
//! 2. Initial step: t̂₀ = T(1 + η₀), T = s/(1 + c), |η₀| ≤ 2u/(1 − u) ≤ γ₂
//!    (two roundings, 1 + c and the quotient).
//! 3. Reduction step: the exact map R(t) = t/(1 + √(1 + t²)) halves atan t.
//!    With a = rnd(t²), b = rnd(1 + a), r = rnd(√b), d = rnd(1 + r),
//!    t' = rnd(t/d): 1 + a = (1 + t²)(1 + δ') with |δ'| ≤ u; r =
//!    √(1 + t²)(1 + ρ) with 1 + ρ ∈ [(1 − u)², (1 + u)²]; 1 + r =
//!    (1 + √(1 + t²))(1 + ρ') with ρ' between 0 and ρ; so t' = R(t)(1 + η)
//!    with |η| ≤ β₄ = 4u/(1 − u)³.
//! 4. Let θ̂_j = atan t̂_j and θ_j = atan(T)/2^j. By Lemma A, θ̂₀ =
//!    θ₀(1 + α₀) with |α₀| ≤ λ(γ₂), and θ̂_{j+1} = (θ̂_j/2)(1 + α_{j+1}) with
//!    |α_{j+1}| ≤ λ(β₄). Since θ₀ < π/2, θ̂₅ < (π/64)(1 + 23u) and
//!    tan θ̂₅ < 0.0492 < 1/20: at most 5 reductions.
//! 5. Series on t = t̂_k < 1/20, a_n = (−1)^n t^(2n+1)/(2n+1): each computed
//!    term is a_n(1 + ε_n) with |ε_n| ≤ γ_{2n+1} (t² rounded once, n products
//!    and one quotient). The stop rule leaves a true tail of magnitude at most
//!    |a_N| < 2^(e_t − p − 1)(1 + γ₃₁) ≤ (u/2)t(1 + γ₃₁). Σ_{n≥1}|a_n| ≤
//!    t³/(3(1 − t²)) < 8.354·10⁻⁴·t, so the term errors add at most
//!    γ₂₉·8.354·10⁻⁴·t and the smallest-first summation of at most 14 terms
//!    (13 roundings) at most γ₁₃(1 + γ₂₉)·8.354·10⁻⁴·t. The final t + tail is
//!    rounded once. With atan t ≥ t(1 − t²/3) ≥ 0.99916·t, the series result
//!    is atan(t)(1 + σ) with |σ| ≤ (1 + 0.0352 + 0.5005)u + O(u²) ≤ 1.54u.
//!    At p ≤ 128 the term n = 15 is always below the stop threshold
//!    (0.05³⁰/31 < 2^−130), so at most 15 terms (n = 0 … 14) are summed.
//! 6. Scaling by 2^(k+1) is exact. So φ̂ = φ(1 + α₀)∏(1 + α_j)(1 + σ) and
//!    |φ̂/φ − 1| ≤ (2 + 4·5 + 1.54)u + O(u²) ≤ 23.55u. With
//!    2^e ≤ φ̂ < 2^(e+1) and ulp_p(φ̂) = 2^(e−p+1), that is at most
//!    23.55·2^p·u·ulp_p(φ̂)(1 + 24u) < 23.6 ulp.
//!
//! **Measured, and the regression tolerance (not a bound).** V1 measured
//! V1's variant (forward summation) at ≤ 2.69 ulp over 13 angles. Over this
//! slice's 3 307 referenced vectors (a 160-digit reference computed two
//! independent ways), the worst error of this implementation is 6.08 ulp at
//! p = 128, on the input the independent review RV2 found
//! (s = +ff4014cc…dddc·2^−128, c = −9c9e9003…981b·2^−131, φ ≈ 1.6473; the
//! worst on I2's own set was 5.41 ulp); V1's variant reaches 7.28 ulp on the
//! same set. `ATAN_REGRESSION_TOLERANCE_TENTH_ULPS` = 6.1 ulp is a regression
//! tolerance for the committed vectors only, raised just enough to cover that
//! input: other inputs may exceed it. The accuracy contract is the proved
//! 23.6 ulp above, which the tests also assert on every vector. Both are far
//! inside K-D5's first-order margin (~10⁻⁹ relative).
//!
//! # The exact binary64 split
//!
//! `Wide::split_binary64` cuts the significand into its bits 127…75, 74…22
//! and 21…0 (53, 53 and 22 bits), each a binary64 value with the value's
//! sign, zero pieces omitted: at most three terms, nonoverlapping, in
//! decreasing magnitude. It is exact (the terms sum exactly to the value)
//! unless the value has set bits below 2^−1074. Those bits are dropped
//! (truncation toward zero) and `truncated_below_min_subnormal` is set; then
//! 0 < |value − Σ terms| < 2^−1074, with the value's sign. For K-D5 (D1
//! §4.3.1 step 2) a coefficient K split this way and taken through
//! `add_product` with u_j therefore misses less than 2^−1074·|u_j|, the
//! per-row allowance. A value of magnitude 2^1024 or more cannot be split and
//! is refused.

use crate::exact_sum::{ExactAccumulator, SumError};
use std::cmp::Ordering;
use std::fmt;

/// Smallest supported precision.
pub(crate) const MIN_PRECISION: u32 = 2;
/// Largest precision of `Wide<2>`.
pub(crate) const MAX_PRECISION: u32 = 128;
/// Smallest precision for which the arctangent's bound is proved.
pub(crate) const ATAN_MIN_PRECISION: u32 = 53;
/// Every exponent (of a leading bit) lies in [−EXPONENT_LIMIT, EXPONENT_LIMIT].
pub(crate) const EXPONENT_LIMIT: i64 = 1 << 62;
/// Half-angle reductions the arctangent may use (proved sufficient).
pub(crate) const ATAN_MAX_REDUCTIONS: u32 = 5;
/// Series terms the arctangent may sum (proved sufficient for p ≤ 128).
pub(crate) const ATAN_MAX_TERMS: u32 = 15;
/// Regression tolerance of the arctangent on the committed vectors only, in
/// tenths of an ulp of p (measured; not a bound; the contract is the proved
/// bound below).
pub(crate) const ATAN_REGRESSION_TOLERANCE_TENTH_ULPS: u32 = 61;
/// Proved worst-case bound of the included angle, in tenths of an ulp of p.
pub(crate) const ATAN_PROVED_BOUND_TENTH_ULPS: u32 = 236;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WideError {
    /// Precision outside [MIN_PRECISION, MAX_PRECISION] (or below
    /// ATAN_MIN_PRECISION for the arctangent).
    InvalidPrecision(u32),
    /// Lift of a NaN or infinity.
    NonFinite,
    /// A result exponent outside ±EXPONENT_LIMIT.
    ExponentRange,
    DivisionByZero,
    NegativeSqrt,
    /// `from_parts` with a significand that is neither zero nor normalized,
    /// or a zero with a nonzero exponent.
    NotNormalized,
    /// Arctangent argument outside its domain (t ≤ 0, s ≤ 0 or 1 + c ≤ 0).
    AngleDomain,
    /// Internal limit of the arctangent reached (unreachable by the proof).
    ArctangentLimit,
    /// Split of a value of magnitude 2^1024 or more.
    SplitOverflow,
    Accumulator(SumError),
}

impl fmt::Display for WideError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPrecision(p) => write!(f, "retained precision {p} is not supported"),
            Self::NonFinite => write!(f, "retained lift of a non-finite binary64 value"),
            Self::ExponentRange => write!(f, "retained exponent outside the supported range"),
            Self::DivisionByZero => write!(f, "retained division by zero"),
            Self::NegativeSqrt => write!(f, "retained square root of a negative value"),
            Self::NotNormalized => write!(f, "retained value is not normalized"),
            Self::AngleDomain => write!(f, "retained arctangent argument outside its domain"),
            Self::ArctangentLimit => write!(f, "retained arctangent internal limit reached"),
            Self::SplitOverflow => write!(f, "retained value too large for a binary64 split"),
            Self::Accumulator(e) => write!(f, "retained split accumulation: {e}"),
        }
    }
}

impl std::error::Error for WideError {}

/// Sign, exponent of the leading bit, and a normalized L-limb significand
/// (little-endian limbs). See the module documentation.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Wide<const L: usize> {
    negative: bool,
    exponent: i64,
    significand: [u64; L],
}

pub(crate) type Wide2 = Wide<2>;

impl fmt::Debug for Wide<2> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = if self.negative { '-' } else { '+' };
        if self.is_zero() {
            write!(f, "Z{sign}")
        } else {
            write!(f, "{sign}{:032x}p{}", self.sig(), self.exponent)
        }
    }
}

/// Binary64 terms whose exact sum is the split value (see the module
/// documentation for when the split is exact).
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Binary64Split {
    terms: [f64; 3],
    len: usize,
    truncated_below_min_subnormal: bool,
}

impl Binary64Split {
    /// The terms, in decreasing magnitude, all with the value's sign.
    pub(crate) fn terms(&self) -> &[f64] {
        &self.terms[..self.len]
    }

    /// True when set bits below 2^−1074 were dropped; then the remainder is
    /// nonzero, below 2^−1074 in magnitude and has the value's sign.
    pub(crate) fn truncated_below_min_subnormal(&self) -> bool {
        self.truncated_below_min_subnormal
    }
}

impl Wide<2> {
    pub(crate) const ZERO: Self = Self {
        negative: false,
        exponent: 0,
        significand: [0, 0],
    };
    pub(crate) const ONE: Self = Self {
        negative: false,
        exponent: 0,
        significand: [0, 1 << 63],
    };

    fn zero(negative: bool) -> Self {
        Self {
            negative,
            ..Self::ZERO
        }
    }

    fn from_sig(negative: bool, exponent: i64, sig: u128) -> Self {
        Self {
            negative,
            exponent,
            significand: [sig as u64, (sig >> 64) as u64],
        }
    }

    fn sig(&self) -> u128 {
        (u128::from(self.significand[1]) << 64) | u128::from(self.significand[0])
    }

    /// Validated construction from raw parts (value model above).
    pub(crate) fn from_parts(
        negative: bool,
        exponent: i64,
        significand: [u64; 2],
    ) -> Result<Self, WideError> {
        let value = Self {
            negative,
            exponent,
            significand,
        };
        let sig = value.sig();
        if sig == 0 {
            return if exponent == 0 {
                Ok(value)
            } else {
                Err(WideError::NotNormalized)
            };
        }
        if sig >> 127 == 0 {
            return Err(WideError::NotNormalized);
        }
        if !(-EXPONENT_LIMIT..=EXPONENT_LIMIT).contains(&exponent) {
            return Err(WideError::ExponentRange);
        }
        Ok(value)
    }

    /// (negative, exponent of the leading bit, little-endian significand).
    pub(crate) fn parts(&self) -> (bool, i64, [u64; 2]) {
        (self.negative, self.exponent, self.significand)
    }

    /// The exact value of a finite binary64 number, including subnormals and
    /// the sign of zero. NaN and infinities are refused.
    pub(crate) fn from_f64(value: f64) -> Result<Self, WideError> {
        if !value.is_finite() {
            return Err(WideError::NonFinite);
        }
        let bits = value.to_bits();
        let negative = bits >> 63 != 0;
        let biased = ((bits >> 52) & 0x7ff) as i64;
        let fraction = bits & ((1u64 << 52) - 1);
        let (integer, lsb) = match (biased, fraction) {
            (0, 0) => return Ok(Self::zero(negative)),
            (0, f) => (f, -1074),
            (b, f) => (f | (1u64 << 52), b - 1075),
        };
        let integer = u128::from(integer);
        let lz = integer.leading_zeros();
        Ok(Self::from_sig(
            negative,
            lsb + 127 - i64::from(lz),
            integer << lz,
        ))
    }

    pub(crate) fn is_zero(&self) -> bool {
        self.sig() == 0
    }

    pub(crate) fn is_sign_negative(&self) -> bool {
        self.negative
    }

    /// Exponent of the leading bit (0 for zero).
    pub(crate) fn exponent(&self) -> i64 {
        self.exponent
    }

    pub(crate) fn neg(&self) -> Self {
        Self {
            negative: !self.negative,
            ..*self
        }
    }

    pub(crate) fn abs(&self) -> Self {
        Self {
            negative: false,
            ..*self
        }
    }

    /// True when the value has at most p significant bits.
    pub(crate) fn fits_precision(&self, p: u32) -> bool {
        p >= 128 || self.sig() & ((1u128 << (128 - p)) - 1) == 0
    }

    /// Exact multiplication by 2^k.
    pub(crate) fn mul_pow2(&self, k: i64) -> Result<Self, WideError> {
        if self.is_zero() {
            return Ok(*self);
        }
        let exponent = checked_exponent(i128::from(self.exponent) + i128::from(k))?;
        Ok(Self { exponent, ..*self })
    }

    /// Numerical comparison (+0 and −0 are equal).
    pub(crate) fn cmp_value(&self, other: &Self) -> Ordering {
        match (self.is_zero(), other.is_zero()) {
            (true, true) => Ordering::Equal,
            (true, false) => {
                if other.negative {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            (false, true) => {
                if self.negative {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
            (false, false) => match (self.negative, other.negative) {
                (false, true) => Ordering::Greater,
                (true, false) => Ordering::Less,
                (false, false) => cmp_magnitude(self, other),
                (true, true) => cmp_magnitude(other, self),
            },
        }
    }

    /// The exact split into at most three binary64 terms (module
    /// documentation).
    pub(crate) fn split_binary64(&self) -> Result<Binary64Split, WideError> {
        let mut split = Binary64Split {
            terms: [0.0; 3],
            len: 0,
            truncated_below_min_subnormal: false,
        };
        if self.is_zero() {
            return Ok(split);
        }
        if self.exponent > 1023 {
            return Err(WideError::SplitOverflow);
        }
        let sig = self.sig();
        // (highest bit, width) of each piece of the significand.
        for (high, width) in [(127u32, 53u32), (74, 53), (21, 22)] {
            let shift = high + 1 - width;
            let mut chunk = ((sig >> shift) as u64) & ((1u64 << width) - 1);
            let mut lsb = self.exponent - 127 + i64::from(shift);
            if chunk == 0 {
                continue;
            }
            if lsb < -1074 {
                let drop = -1074 - lsb;
                if drop >= 64 {
                    split.truncated_below_min_subnormal = true;
                    continue;
                }
                if chunk & ((1u64 << drop) - 1) != 0 {
                    split.truncated_below_min_subnormal = true;
                }
                chunk >>= drop;
                lsb = -1074;
                if chunk == 0 {
                    continue;
                }
            }
            split.terms[split.len] = exact_binary64(self.negative, chunk, lsb);
            split.len += 1;
        }
        Ok(split)
    }

    /// Adds `self · factor` to the accumulator through the exact split, one
    /// `add_product` per term. Returns true when the split was truncated
    /// below 2^−1074, in which case the accumulated value misses less than
    /// 2^−1074·|factor| (K-D5's per-row allowance).
    pub(crate) fn add_product_to(
        &self,
        accumulator: &mut ExactAccumulator,
        factor: f64,
    ) -> Result<bool, WideError> {
        let split = self.split_binary64()?;
        for &term in split.terms() {
            accumulator
                .add_product(term, factor)
                .map_err(WideError::Accumulator)?;
        }
        Ok(split.truncated_below_min_subnormal)
    }
}

/// Binary64 value (−1)^negative · chunk · 2^lsb; exact by construction
/// (chunk < 2^53, lsb ≥ −1074, leading bit at most 2^1023).
fn exact_binary64(negative: bool, chunk: u64, lsb: i64) -> f64 {
    debug_assert!(chunk != 0 && chunk < (1u64 << 53) && lsb >= -1074);
    let top = i64::from(63 - chunk.leading_zeros());
    let leading = lsb + top;
    debug_assert!(leading <= 1023);
    let bits = if leading >= -1022 {
        let m = chunk << (52 - top);
        (((leading + 1023) as u64) << 52) | (m & ((1u64 << 52) - 1))
    } else {
        chunk << (lsb + 1074)
    };
    f64::from_bits(bits | (u64::from(negative) << 63))
}

fn checked_exponent(exponent: i128) -> Result<i64, WideError> {
    if exponent < -i128::from(EXPONENT_LIMIT) || exponent > i128::from(EXPONENT_LIMIT) {
        Err(WideError::ExponentRange)
    } else {
        Ok(exponent as i64)
    }
}

/// Magnitude order of two nonzero normalized values.
fn cmp_magnitude(a: &Wide2, b: &Wide2) -> Ordering {
    a.exponent
        .cmp(&b.exponent)
        .then_with(|| a.sig().cmp(&b.sig()))
}

// ---------------------------------------------------------------------------
// 256-bit unsigned helper (two u128 halves)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct U256 {
    hi: u128,
    lo: u128,
}

impl U256 {
    const ZERO: Self = Self { hi: 0, lo: 0 };
    const ONE: Self = Self { hi: 0, lo: 1 };

    fn is_zero(&self) -> bool {
        self.hi == 0 && self.lo == 0
    }

    fn leading_zeros(&self) -> u32 {
        if self.hi != 0 {
            self.hi.leading_zeros()
        } else {
            128 + self.lo.leading_zeros()
        }
    }

    /// Left shift by n < 256 (bits shifted out are discarded).
    fn shl(&self, n: u32) -> Self {
        match n {
            0 => *self,
            1..=127 => Self {
                hi: (self.hi << n) | (self.lo >> (128 - n)),
                lo: self.lo << n,
            },
            _ => Self {
                hi: self.lo << (n - 128),
                lo: 0,
            },
        }
    }

    /// Right shift by any n; the flag is true when a set bit was shifted out.
    fn shr_sticky(&self, n: i128) -> (Self, bool) {
        if n <= 0 {
            return (*self, false);
        }
        if n >= 256 {
            return (Self::ZERO, !self.is_zero());
        }
        let n = n as u32;
        if n < 128 {
            let lost = self.lo & ((1u128 << n) - 1) != 0;
            (
                Self {
                    hi: self.hi >> n,
                    lo: (self.lo >> n) | (self.hi << (128 - n)),
                },
                lost,
            )
        } else {
            let m = n - 128;
            let lost = self.lo != 0 || (m > 0 && self.hi & ((1u128 << m) - 1) != 0);
            (
                Self {
                    hi: 0,
                    lo: if m == 0 { self.hi } else { self.hi >> m },
                },
                lost,
            )
        }
    }

    fn overflowing_add(&self, other: &Self) -> (Self, bool) {
        let (lo, c1) = self.lo.overflowing_add(other.lo);
        let (hi, c2) = self.hi.overflowing_add(other.hi);
        let (hi, c3) = hi.overflowing_add(u128::from(c1));
        (Self { hi, lo }, c2 || c3)
    }

    /// self − other, with self ≥ other.
    fn sub(&self, other: &Self) -> Self {
        debug_assert!(*self >= *other);
        let (lo, borrow) = self.lo.overflowing_sub(other.lo);
        let hi = self
            .hi
            .wrapping_sub(other.hi)
            .wrapping_sub(u128::from(borrow));
        Self { hi, lo }
    }

    fn or_low(&self, bits: u128) -> Self {
        Self {
            hi: self.hi,
            lo: self.lo | bits,
        }
    }

    fn bit(&self, i: u32) -> bool {
        if i < 128 {
            (self.lo >> i) & 1 == 1
        } else {
            (self.hi >> (i - 128)) & 1 == 1
        }
    }

    /// Any set bit strictly below position n (n ≤ 256).
    fn any_below(&self, n: u32) -> bool {
        match n {
            0 => false,
            1..=127 => self.lo & ((1u128 << n) - 1) != 0,
            128 => self.lo != 0,
            129..=255 => self.lo != 0 || self.hi & ((1u128 << (n - 128)) - 1) != 0,
            _ => !self.is_zero(),
        }
    }

    /// Full 256-bit product of two u128 values.
    fn mul_u128(a: u128, b: u128) -> Self {
        let mask = u128::from(u64::MAX);
        let (a1, a0) = (a >> 64, a & mask);
        let (b1, b0) = (b >> 64, b & mask);
        let p00 = a0 * b0;
        let p01 = a0 * b1;
        let p10 = a1 * b0;
        let p11 = a1 * b1;
        let mid = (p00 >> 64) + (p01 & mask) + (p10 & mask);
        let lo = (p00 & mask) | (mid << 64);
        let hi = p11 + (p01 >> 64) + (p10 >> 64) + (mid >> 64);
        Self { hi, lo }
    }
}

/// Rounds (−1)^negative · (mag + f) · 2^scale to p bits, to nearest, ties to
/// even, where f ∈ (0, 1) exactly when `sticky` is set. When `sticky` is set,
/// `mag` must carry at least p + 1 significant bits, so the round bit is a bit
/// of `mag` and f lies strictly below it.
fn round_pack(
    negative: bool,
    mag: U256,
    sticky: bool,
    scale: i128,
    p: u32,
) -> Result<Wide2, WideError> {
    if mag.is_zero() {
        debug_assert!(!sticky);
        return Ok(Wide2::zero(negative));
    }
    let lz = mag.leading_zeros();
    debug_assert!(!sticky || 256 - lz > p);
    let m = mag.shl(lz);
    let mut exponent = scale - i128::from(lz) + 255;
    // Keep the top p bits (p ≤ 128, so they lie in the high half).
    let drop = 256 - p;
    let keep = m.hi >> (drop - 128);
    let round = m.bit(drop - 1);
    let rest = sticky || m.any_below(drop - 1);
    let mut keep = keep;
    if round && (rest || keep & 1 == 1) {
        let (next, overflow) = keep.overflowing_add(1);
        if overflow || (p < 128 && next >> p != 0) {
            keep = 1u128 << (p - 1);
            exponent += 1;
        } else {
            keep = next;
        }
    }
    let exponent = checked_exponent(exponent)?;
    Ok(Wide2::from_sig(negative, exponent, keep << (128 - p)))
}

/// A value rounded to p bits.
fn round_value(value: &Wide2, p: u32) -> Result<Wide2, WideError> {
    if value.is_zero() {
        return Ok(*value);
    }
    round_pack(
        value.negative,
        U256 {
            hi: value.sig(),
            lo: 0,
        },
        false,
        i128::from(value.exponent) - 255,
        p,
    )
}

fn add_exact_operands(a: &Wide2, b: &Wide2, p: u32) -> Result<Wide2, WideError> {
    match (a.is_zero(), b.is_zero()) {
        (true, true) => return Ok(Wide2::zero(a.negative && b.negative)),
        (true, false) => return round_value(b, p),
        (false, true) => return round_value(a, p),
        (false, false) => {}
    }
    let (big, small) = if cmp_magnitude(a, b) == Ordering::Less {
        (b, a)
    } else {
        (a, b)
    };
    let gap = i128::from(big.exponent) - i128::from(small.exponent);
    let big_mag = U256 {
        hi: big.sig(),
        lo: 0,
    };
    // `small_mag + f` with f ∈ (0, 1) when `sticky`: the exact aligned operand.
    let (small_mag, sticky) = U256 {
        hi: small.sig(),
        lo: 0,
    }
    .shr_sticky(gap);
    let scale = i128::from(big.exponent) - 255;
    if big.negative == small.negative {
        let (sum, carry) = big_mag.overflowing_add(&small_mag);
        if carry {
            let lost = sum.lo & 1 == 1;
            let shifted = U256 {
                hi: (sum.hi >> 1) | (1u128 << 127),
                lo: (sum.lo >> 1) | (sum.hi << 127),
            };
            round_pack(big.negative, shifted, sticky || lost, scale + 1, p)
        } else {
            round_pack(big.negative, sum, sticky, scale, p)
        }
    } else {
        // big − (small_mag + f) = (big − small_mag − 1) + (1 − f), 1 − f ∈ (0, 1).
        let mut diff = big_mag.sub(&small_mag);
        if sticky {
            diff = diff.sub(&U256::ONE);
        }
        if diff.is_zero() && !sticky {
            return Ok(Wide2::zero(false));
        }
        round_pack(big.negative, diff, sticky, scale, p)
    }
}

fn mul_exact_operands(a: &Wide2, b: &Wide2, p: u32) -> Result<Wide2, WideError> {
    let negative = a.negative != b.negative;
    if a.is_zero() || b.is_zero() {
        return Ok(Wide2::zero(negative));
    }
    let product = U256::mul_u128(a.sig(), b.sig());
    let scale = i128::from(a.exponent) + i128::from(b.exponent) - 254;
    round_pack(negative, product, false, scale, p)
}

/// floor(a · 2^129 / b) (129 or 130 bits) and whether the remainder is
/// nonzero, for normalized a, b; bit-serial restoring division.
fn divide_significands(a: u128, b: u128) -> (U256, bool) {
    let mut quotient = U256::ZERO;
    let mut remainder = a;
    if remainder >= b {
        remainder -= b;
        quotient = U256::ONE;
    }
    for _ in 0..129 {
        let carry = remainder >> 127 != 0;
        let doubled = remainder << 1;
        let bit = carry || doubled >= b;
        remainder = if bit {
            doubled.wrapping_sub(b)
        } else {
            doubled
        };
        quotient = quotient.shl(1).or_low(u128::from(bit));
    }
    (quotient, remainder != 0)
}

fn div_exact_operands(a: &Wide2, b: &Wide2, p: u32) -> Result<Wide2, WideError> {
    if b.is_zero() {
        return Err(WideError::DivisionByZero);
    }
    let negative = a.negative != b.negative;
    if a.is_zero() {
        return Ok(Wide2::zero(negative));
    }
    let (quotient, sticky) = divide_significands(a.sig(), b.sig());
    let scale = i128::from(a.exponent) - i128::from(b.exponent) - 129;
    round_pack(negative, quotient, sticky, scale, p)
}

/// floor(√(sig · 2^shift)) and whether the remainder is nonzero; bit-serial
/// (two radicand bits per root bit), 130 root bits.
fn sqrt_significand(sig: u128, shift: u32) -> (U256, bool) {
    let radicand_bit = |j: u32| -> u128 {
        if j >= shift && j - shift < 128 {
            (sig >> (j - shift)) & 1
        } else {
            0
        }
    };
    let mut remainder = U256::ZERO;
    let mut root = U256::ZERO;
    for i in (0..130u32).rev() {
        let pair = (radicand_bit(2 * i + 1) << 1) | radicand_bit(2 * i);
        remainder = remainder.shl(2).or_low(pair);
        let trial = root.shl(2).or_low(1);
        if remainder >= trial {
            remainder = remainder.sub(&trial);
            root = root.shl(1).or_low(1);
        } else {
            root = root.shl(1);
        }
    }
    (root, !remainder.is_zero())
}

fn sqrt_exact_operand(a: &Wide2, p: u32) -> Result<Wide2, WideError> {
    if a.is_zero() {
        return Ok(*a);
    }
    if a.negative {
        return Err(WideError::NegativeSqrt);
    }
    // a = sig · 2^e with e = exponent − 127; radicand sig · 2^shift with
    // e − shift even and 257 to 259 bits, so the root has 129 or 130 bits.
    let e = i128::from(a.exponent) - 127;
    let shift: u32 = if e.rem_euclid(2) == 0 { 130 } else { 131 };
    let (root, sticky) = sqrt_significand(a.sig(), shift);
    let scale = (e - i128::from(shift)) / 2;
    round_pack(false, root, sticky, scale, p)
}

/// t ≥ 1/20, decided exactly (5·t ≥ 1/4).
fn at_least_one_twentieth(t: &Wide2) -> bool {
    if t.is_zero() || t.negative {
        return false;
    }
    // 5 · sig · 2^(e − 127) ≥ 2^−2  ⇔  5 · sig ≥ 2^(125 − e).
    let k = 125 - i128::from(t.exponent);
    if k <= 0 {
        return true;
    }
    if k >= 131 {
        return false;
    }
    U256::mul_u128(t.sig(), 5) >= U256::ONE.shl(k as u32)
}

// ---------------------------------------------------------------------------
// Arithmetic context: precision and work counter
// ---------------------------------------------------------------------------

/// Operations performed, by kind. Saturating; never wraps.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct WorkCounter {
    pub(crate) add: u64,
    pub(crate) sub: u64,
    pub(crate) mul: u64,
    pub(crate) div: u64,
    pub(crate) sqrt: u64,
    pub(crate) atan: u64,
}

impl WorkCounter {
    /// Correctly rounded operations (+ − × ÷ √), including those inside the
    /// arctangent.
    pub(crate) fn rounded_operations(&self) -> u64 {
        self.add
            .saturating_add(self.sub)
            .saturating_add(self.mul)
            .saturating_add(self.div)
            .saturating_add(self.sqrt)
    }
}

/// Correctly rounded `Wide<2>` arithmetic at a runtime precision.
#[derive(Debug, Clone)]
pub(crate) struct WideArith {
    precision: u32,
    work: WorkCounter,
}

impl WideArith {
    pub(crate) fn new(precision: u32) -> Result<Self, WideError> {
        if !(MIN_PRECISION..=MAX_PRECISION).contains(&precision) {
            return Err(WideError::InvalidPrecision(precision));
        }
        Ok(Self {
            precision,
            work: WorkCounter::default(),
        })
    }

    pub(crate) fn precision(&self) -> u32 {
        self.precision
    }

    pub(crate) fn work(&self) -> WorkCounter {
        self.work
    }

    pub(crate) fn add(&mut self, a: &Wide2, b: &Wide2) -> Result<Wide2, WideError> {
        self.work.add = self.work.add.saturating_add(1);
        add_exact_operands(a, b, self.precision)
    }

    pub(crate) fn sub(&mut self, a: &Wide2, b: &Wide2) -> Result<Wide2, WideError> {
        self.work.sub = self.work.sub.saturating_add(1);
        add_exact_operands(a, &b.neg(), self.precision)
    }

    pub(crate) fn mul(&mut self, a: &Wide2, b: &Wide2) -> Result<Wide2, WideError> {
        self.work.mul = self.work.mul.saturating_add(1);
        mul_exact_operands(a, b, self.precision)
    }

    pub(crate) fn div(&mut self, a: &Wide2, b: &Wide2) -> Result<Wide2, WideError> {
        self.work.div = self.work.div.saturating_add(1);
        div_exact_operands(a, b, self.precision)
    }

    pub(crate) fn sqrt(&mut self, a: &Wide2) -> Result<Wide2, WideError> {
        self.work.sqrt = self.work.sqrt.saturating_add(1);
        sqrt_exact_operand(a, self.precision)
    }

    /// The included angle φ = 2·atan(s/(1 + c)) ∈ (0, π) for s > 0 and
    /// 1 + c > 0 (module documentation: algorithm, proved bound, tolerance).
    pub(crate) fn included_angle(&mut self, s: &Wide2, c: &Wide2) -> Result<Wide2, WideError> {
        self.work.atan = self.work.atan.saturating_add(1);
        self.check_atan_precision()?;
        if s.is_zero() || s.negative {
            return Err(WideError::AngleDomain);
        }
        let d = self.add(&Wide2::ONE, c)?;
        if d.is_zero() || d.negative {
            return Err(WideError::AngleDomain);
        }
        let t = self.div(s, &d)?;
        self.atan_of_positive(&t)?.mul_pow2(1)
    }

    /// atan t ∈ (0, π/2) for t > 0 (the same reduction and series).
    pub(crate) fn atan_positive(&mut self, t: &Wide2) -> Result<Wide2, WideError> {
        self.work.atan = self.work.atan.saturating_add(1);
        self.check_atan_precision()?;
        if t.is_zero() || t.negative {
            return Err(WideError::AngleDomain);
        }
        self.atan_of_positive(t)
    }

    fn check_atan_precision(&self) -> Result<(), WideError> {
        if self.precision < ATAN_MIN_PRECISION {
            return Err(WideError::InvalidPrecision(self.precision));
        }
        Ok(())
    }

    fn atan_of_positive(&mut self, t: &Wide2) -> Result<Wide2, WideError> {
        let one = Wide2::ONE;
        let mut t = *t;
        let mut reductions = 0u32;
        while at_least_one_twentieth(&t) {
            if reductions == ATAN_MAX_REDUCTIONS {
                return Err(WideError::ArctangentLimit);
            }
            let t2 = self.mul(&t, &t)?;
            let b = self.add(&one, &t2)?;
            let r = self.sqrt(&b)?;
            let d = self.add(&one, &r)?;
            t = self.div(&t, &d)?;
            reductions += 1;
        }
        let t2 = self.mul(&t, &t)?;
        let threshold = i128::from(t.exponent) - i128::from(self.precision) - 1;
        let mut terms = [Wide2::ZERO; ATAN_MAX_TERMS as usize];
        terms[0] = t;
        let mut count = 1usize;
        let mut power = t;
        for n in 1..=ATAN_MAX_TERMS {
            power = self.mul(&power, &t2)?.neg();
            let divisor = Wide2::from_f64(f64::from(2 * n + 1))?;
            let term = self.div(&power, &divisor)?;
            if term.is_zero() || i128::from(term.exponent) < threshold {
                break;
            }
            if n == ATAN_MAX_TERMS {
                return Err(WideError::ArctangentLimit);
            }
            terms[count] = term;
            count += 1;
        }
        let mut tail = Wide2::ZERO;
        for term in terms[1..count].iter().rev() {
            tail = self.add(&tail, term)?;
        }
        let sum = self.add(&t, &tail)?;
        sum.mul_pow2(i64::from(reductions))
    }
}

#[cfg(test)]
#[path = "../../../tests/retained_wide/wide_tests.rs"]
mod tests;
