//! Slice K3: `Wide<L>` at L = 4, 8 and 16, the rounded conversion to binary64
//! at every width, and the arithmetic K4 needs (T3 D1 §4.1.1, §4.11).
//!
//! K3a's `Wide<2>` code in `wide.rs` is not changed or called here: K-D5 keeps
//! using it. This module adds, beside it:
//! - the generic core (free functions over the width L), used at L = 4, 8
//!   and 16 through `WideContext<L>`, and instantiated at L = 2 only by the
//!   tests, which check it bitwise against K3a's `WideArith`;
//! - the value API of `Wide<4>`, `Wide<8>` and `Wide<16>` (lift, parts,
//!   comparisons, `Debug`), with K3a's method names;
//! - `Wide::to_binary64` and `Wide::widen`, for every width, L = 2 included;
//! - `WideContext<L>`: + − × ÷ √, correctly rounded narrowing, the integer
//!   constructor, TwoSum and TwoProduct, each rounded to nearest, ties to
//!   even, at a runtime precision 2 ≤ p ≤ 64L, with a per-width work count;
//! - `AttemptWork`: the counts of one attempt across widths, and their cost in
//!   limb-multiply equivalents (D1 §4.1.7).
//!
//! # Value model
//!
//! As in `wide.rs`, with 127 replaced by 64L − 1: a value is
//! (−1)^negative · m · 2^(exponent − (64L − 1)), m the 64L-bit significand in
//! little-endian limbs, normalized (bit 64L − 1 set) unless zero. A zero has
//! m = 0 and exponent 0 and keeps its sign. Exponents lie in ±2^62; a result
//! outside is `WideError::ExponentRange`, never a wrapped value.
//!
//! # Arithmetic (every operation rounded once, from the exact result)
//!
//! Only integer operations are used, so results are bitwise reproducible on
//! every platform. Every intermediate lives on the stack: the 2L-limb
//! buffers are `[u64; 2L]` through `CoreWidth::Double`, and no operation
//! allocates.
//! - **Addition** aligns the smaller operand in a 2L-limb window below the
//!   larger one. A nonzero part shifted out of the window (only when the
//!   exponent gap exceeds 64L) is kept as a sticky fraction f ∈ (0, 1), and a
//!   subtraction takes (big − small − 1) + (1 − f), exactly as K3a does. The
//!   result then has at least 128L − 1 ≥ p + 1 significant bits, so the round
//!   bit is a bit of it and f lies below it.
//! - **Multiplication** forms the exact 2L-limb schoolbook product.
//! - **Division** is bit-serial restoring division: floor(a·2^(64L+1)/b), with
//!   64L + 1 or 64L + 2 bits, and the remainder as the sticky bit.
//! - **Square root** is bit-serial, two radicand bits per root bit: 64L + 1 or
//!   64L + 2 root bits, and the remainder as the sticky bit.
//! - **Rounding** keeps the top p bits of the exact magnitude, and rounds to
//!   nearest with ties to even on the round bit and everything below it.
//! - Signs of zero follow IEEE 754 under round-to-nearest, as in K3a.
//!
//! # Conversion to binary64 (`Wide::to_binary64`, every width)
//!
//! The exact value is rounded once, to nearest with ties to even, at the
//! binary64 quantum of its binade: 2^(e − 52) for e ≥ −1022, and 2^−1074
//! below (the subnormal quantum). There is no intermediate rounding to 53
//! bits, so a subnormal is never double-rounded. The outcome (D1 §4.1.1, §5
//! item 7; ROOT's K3 ruling Q5, the convention of K2b's publication rule):
//! - `Normal(x)`: the result is a normal binary64 value, or the value is an
//!   exact ±0, which keeps its sign;
//! - `Subnormal { value, relative_precision }`: the result is subnormal. The
//!   absolute error is at most 2^−1075, and `relative_precision` is
//!   2^−1075/|value|, rounded upward (integer arithmetic);
//! - `Underflow { negative }`: a nonzero value whose rounding is ±0;
//! - `Overflow { negative }`: a value whose rounding is ±∞ (from the midpoint
//!   2^1024 − 2^970 upward).
//!
//! Underflow and overflow carry no value: nothing is returned silently.
//!
//! # K4's arithmetic (ROOT's K3 rulings Q3, Q4)
//!
//! - `widen` is exact (a compile-time M ≥ L); `WideContext::round` is the
//!   correctly rounded narrowing (or re-rounding) of a value of any width.
//! - `WideContext::from_integer` rounds ±magnitude·2^exponent once, for a
//!   little-endian magnitude of any length (for example `ExactAccumulator`'s
//!   68 limbs at quantum 2^−2148). A zero magnitude gives +0 whatever the sign
//!   flag (D1 §4.1.2, "an exact zero is +0.0").
//! - `two_sum` and `two_product` return (s, e) with s = fl_p(a ∘ b) and
//!   s + e = a ∘ b exactly, |e| ≤ ulp_p(s)/2. Both operands must have at most
//!   p significant bits (`WideError::OperandPrecision` otherwise). TwoSum is
//!   Knuth's six-operation form at p; TwoProduct takes the exact product minus
//!   its rounding. An exact result gives e = +0.
//!
//! # Work (D1 §4.1.7; ROOT's K3 ruling Q9)
//!
//! `WideContext` counts its operations by kind, saturating. `AttemptWork`
//! adds contexts of every width within one attempt and states their cost in
//! limb-multiply equivalents, a deterministic function of (kind, L) taken from
//! the algorithm's step count (not a timing):
//!
//! | Kind | Cost at width L |
//! |---|---|
//! | add, sub, round (narrowing, the integer constructor) | 2L |
//! | mul | L² |
//! | div | (L + 1)(64L + 2) |
//! | sqrt | (L + 2)(64L + 2) |
//! | two_sum | 12L |
//! | two_product | L² + 4L |
//!
//! The conversion to binary64 and widening are value methods and are not
//! counted. K3a's `WorkCounter` at L = 2 is unchanged.

use super::{checked_exponent, Wide, WideError, EXPONENT_LIMIT, MIN_PRECISION};
use std::cmp::Ordering;
use std::fmt;

// ---------------------------------------------------------------------------
// Widths
// ---------------------------------------------------------------------------

/// Widths the generic core runs at: L = 2 (tests only), 4, 8 and 16.
pub(crate) trait CoreWidth {
    /// The 2L-limb stack buffer of products, quotients and remainders.
    type Double: Copy + AsRef<[u64]> + AsMut<[u64]>;
    const DOUBLE_ZERO: Self::Double;
}

/// Widths with the value API and an arithmetic context: L = 4, 8 and 16.
pub(crate) trait SupportedWidth: CoreWidth {
    /// Index of the width in `AttemptWork`.
    const SLOT: usize;
}

macro_rules! core_width {
    ($limbs:literal, $double:literal) => {
        impl CoreWidth for Wide<$limbs> {
            type Double = [u64; $double];
            const DOUBLE_ZERO: [u64; $double] = [0; $double];
        }
    };
}
core_width!(2, 4);
core_width!(4, 8);
core_width!(8, 16);
core_width!(16, 32);

impl SupportedWidth for Wide<4> {
    const SLOT: usize = 0;
}
impl SupportedWidth for Wide<8> {
    const SLOT: usize = 1;
}
impl SupportedWidth for Wide<16> {
    const SLOT: usize = 2;
}

/// Limb counts of the `AttemptWork` slots.
const SLOT_LIMBS: [usize; 3] = [4, 8, 16];

// ---------------------------------------------------------------------------
// Limb-slice helpers (little-endian u64 limbs)
// ---------------------------------------------------------------------------

fn limbs_zero(a: &[u64]) -> bool {
    a.iter().all(|&x| x == 0)
}

/// Index of the highest set bit.
fn highest_bit(a: &[u64]) -> Option<usize> {
    let word = (0..a.len()).rev().find(|&i| a[i] != 0)?;
    Some(word * 64 + 63 - a[word].leading_zeros() as usize)
}

/// Bit i (false outside the slice).
fn bit_at(a: &[u64], i: i128) -> bool {
    if i < 0 || i >= 64 * a.len() as i128 {
        return false;
    }
    let i = i as usize;
    (a[i / 64] >> (i % 64)) & 1 == 1
}

/// Any set bit strictly below index n.
fn any_below(a: &[u64], n: i128) -> bool {
    if n <= 0 {
        return false;
    }
    if n >= 64 * a.len() as i128 {
        return !limbs_zero(a);
    }
    let n = n as usize;
    let full = n / 64;
    if !limbs_zero(&a[..full]) {
        return true;
    }
    let rem = n % 64;
    rem != 0 && a[full] & ((1u64 << rem) - 1) != 0
}

/// Bits [start, start + 64) as one word (zero outside the slice).
fn word_at(a: &[u64], start: i128) -> u64 {
    let word = start.div_euclid(64);
    let shift = start.rem_euclid(64) as u32;
    let get = |k: i128| -> u64 {
        if k >= 0 && k < a.len() as i128 {
            a[k as usize]
        } else {
            0
        }
    };
    if shift == 0 {
        get(word)
    } else {
        (get(word) >> shift) | (get(word + 1) << (64 - shift))
    }
}

/// out = floor(src · 2^shift), limb by limb. Returns true when a set bit of
/// src was shifted out below bit 0. The caller ensures nothing leaves the top.
fn place(src: &[u64], shift: i128, out: &mut [u64]) -> bool {
    for (i, o) in out.iter_mut().enumerate() {
        *o = word_at(src, 64 * i as i128 - shift);
    }
    debug_assert!(!any_below_top(src, shift, out.len()));
    any_below(src, -shift)
}

/// True when src · 2^shift has a set bit at or above 64·limbs.
fn any_below_top(src: &[u64], shift: i128, limbs: usize) -> bool {
    match highest_bit(src) {
        Some(h) => h as i128 + shift >= 64 * limbs as i128,
        None => false,
    }
}

fn set_bit(a: &mut [u64], i: usize) {
    a[i / 64] |= 1u64 << (i % 64);
}

/// Clears every bit below index n.
fn clear_below(a: &mut [u64], n: usize) {
    for (i, x) in a.iter_mut().enumerate() {
        let lo = 64 * i;
        if lo + 64 <= n {
            *x = 0;
        } else if lo < n {
            *x &= !((1u64 << (n - lo)) - 1);
        }
    }
}

/// Clears every bit at or above index n.
fn clear_from(a: &mut [u64], n: usize) {
    for (i, x) in a.iter_mut().enumerate() {
        let lo = 64 * i;
        if lo >= n {
            *x = 0;
        } else if lo + 64 > n {
            *x &= (1u64 << (n - lo)) - 1;
        }
    }
}

/// a += b (same length); returns the carry out of the top limb.
fn add_in_place(a: &mut [u64], b: &[u64]) -> bool {
    let mut carry = false;
    for (x, &y) in a.iter_mut().zip(b) {
        let (s, c1) = x.overflowing_add(y);
        let (s, c2) = s.overflowing_add(u64::from(carry));
        *x = s;
        carry = c1 || c2;
    }
    carry
}

/// a −= b (same length), modulo 2^(64·len).
fn sub_in_place(a: &mut [u64], b: &[u64]) {
    let mut borrow = false;
    for (x, &y) in a.iter_mut().zip(b) {
        let (d, b1) = x.overflowing_sub(y);
        let (d, b2) = d.overflowing_sub(u64::from(borrow));
        *x = d;
        borrow = b1 || b2;
    }
}

/// a −= 1, with a ≥ 1.
fn decrement(a: &mut [u64]) {
    for x in a.iter_mut() {
        let (d, borrow) = x.overflowing_sub(1);
        *x = d;
        if !borrow {
            return;
        }
    }
}

/// a += 2^pos; returns the carry out of the top limb.
fn add_bit(a: &mut [u64], pos: usize) -> bool {
    let mut word = pos / 64;
    let mut add = 1u64 << (pos % 64);
    while word < a.len() {
        let (s, carry) = a[word].overflowing_add(add);
        a[word] = s;
        if !carry {
            return false;
        }
        add = 1;
        word += 1;
    }
    true
}

/// Numerical order of two equal-length magnitudes.
fn cmp_limbs(a: &[u64], b: &[u64]) -> Ordering {
    for i in (0..a.len()).rev() {
        let order = a[i].cmp(&b[i]);
        if order != Ordering::Equal {
            return order;
        }
    }
    Ordering::Equal
}

/// a <<= n for 1 ≤ n ≤ 63; returns the bits shifted out of the top.
fn shl_small(a: &mut [u64], n: u32) -> u64 {
    let mut carry = 0u64;
    for x in a.iter_mut() {
        let next = *x >> (64 - n);
        *x = (*x << n) | carry;
        carry = next;
    }
    carry
}

/// a >>= 1; returns the bit shifted out.
fn shr1(a: &mut [u64]) -> bool {
    let mut carry = 0u64;
    for x in a.iter_mut().rev() {
        let next = *x & 1;
        *x = (*x >> 1) | (carry << 63);
        carry = next;
    }
    carry == 1
}

/// out = a · b, the full schoolbook product (out has a.len() + b.len() limbs).
fn mul_limbs(a: &[u64], b: &[u64], out: &mut [u64]) {
    for o in out.iter_mut() {
        *o = 0;
    }
    for (i, &x) in a.iter().enumerate() {
        let mut carry = 0u64;
        for (j, &y) in b.iter().enumerate() {
            let t = u128::from(x) * u128::from(y) + u128::from(out[i + j]) + u128::from(carry);
            out[i + j] = t as u64;
            carry = (t >> 64) as u64;
        }
        out[i + b.len()] = carry;
    }
}

// ---------------------------------------------------------------------------
// The generic core
// ---------------------------------------------------------------------------

fn signed_zero<const L: usize>(negative: bool) -> Wide<L> {
    Wide {
        negative,
        exponent: 0,
        significand: [0; L],
    }
}

fn negated<const L: usize>(a: &Wide<L>) -> Wide<L> {
    Wide {
        negative: !a.negative,
        ..*a
    }
}

fn is_zero_value<const L: usize>(a: &Wide<L>) -> bool {
    limbs_zero(&a.significand)
}

/// True when the value has at most p significant bits.
fn fits<const L: usize>(a: &Wide<L>, p: u32) -> bool {
    let width = 64 * L as i128;
    i128::from(p) >= width || !any_below(&a.significand, width - i128::from(p))
}

/// Magnitude order of two nonzero normalized values.
fn cmp_mag<const L: usize>(a: &Wide<L>, b: &Wide<L>) -> Ordering {
    a.exponent
        .cmp(&b.exponent)
        .then_with(|| cmp_limbs(&a.significand, &b.significand))
}

/// Exponent of the unit bit of the significand: value = m · 2^(unit).
fn unit_exponent<const L: usize>(a: &Wide<L>) -> i128 {
    i128::from(a.exponent) - (64 * L as i128 - 1)
}

/// Rounds (−1)^negative · (mag + f) · 2^scale to p bits, to nearest with ties
/// to even, where f ∈ (0, 1) exactly when `sticky`. When `sticky` is set, mag
/// must carry at least p + 1 significant bits (the round bit is a bit of mag).
/// Returns the rounded value, the index in mag of the kept least significant
/// bit, and whether the magnitude was rounded up.
fn round_detail<const L: usize>(
    negative: bool,
    mag: &[u64],
    sticky: bool,
    scale: i128,
    p: u32,
) -> Result<(Wide<L>, usize, bool), WideError> {
    let Some(h) = highest_bit(mag) else {
        debug_assert!(!sticky);
        return Ok((signed_zero(negative), 0, false));
    };
    let width = 64 * L;
    let count = (h + 1).min(p as usize);
    let lsb = h + 1 - count;
    debug_assert!(!sticky || lsb > 0);
    let mut significand = [0u64; L];
    for (i, limb) in significand.iter_mut().enumerate() {
        *limb = word_at(mag, h as i128 + 1 - width as i128 + 64 * i as i128);
    }
    clear_below(&mut significand, width - count);
    let round = bit_at(mag, lsb as i128 - 1);
    let rest = sticky || any_below(mag, lsb as i128 - 1);
    let odd = bit_at(&significand, (width - count) as i128);
    let mut exponent = scale + h as i128;
    let up = round && (rest || odd);
    if up && add_bit(&mut significand, width - count) {
        significand = [0; L];
        significand[L - 1] = 1 << 63;
        exponent += 1;
    }
    let exponent = checked_exponent(exponent)?;
    Ok((
        Wide {
            negative,
            exponent,
            significand,
        },
        lsb,
        up,
    ))
}

fn round_limbs<const L: usize>(
    negative: bool,
    mag: &[u64],
    sticky: bool,
    scale: i128,
    p: u32,
) -> Result<Wide<L>, WideError> {
    Ok(round_detail::<L>(negative, mag, sticky, scale, p)?.0)
}

/// A value of any width rounded to p bits at width L (a zero keeps its sign).
fn round_from<const L: usize, const M: usize>(x: &Wide<M>, p: u32) -> Result<Wide<L>, WideError> {
    round_limbs::<L>(x.negative, &x.significand, false, unit_exponent(x), p)
}

/// ±magnitude · 2^exponent rounded once to p bits; +0 for a zero magnitude.
fn from_integer_rounded<const L: usize>(
    negative: bool,
    magnitude: &[u64],
    exponent: i64,
    p: u32,
) -> Result<Wide<L>, WideError> {
    if limbs_zero(magnitude) {
        return Ok(signed_zero(false));
    }
    round_limbs::<L>(negative, magnitude, false, i128::from(exponent), p)
}

fn add_rounded<const L: usize>(a: &Wide<L>, b: &Wide<L>, p: u32) -> Result<Wide<L>, WideError>
where
    Wide<L>: CoreWidth,
{
    match (is_zero_value(a), is_zero_value(b)) {
        (true, true) => return Ok(signed_zero(a.negative && b.negative)),
        (true, false) => return round_from::<L, L>(b, p),
        (false, true) => return round_from::<L, L>(a, p),
        (false, false) => {}
    }
    let (big, small) = if cmp_mag(a, b) == Ordering::Less {
        (b, a)
    } else {
        (a, b)
    };
    let width = 64 * L as i128;
    let gap = i128::from(big.exponent) - i128::from(small.exponent);
    // big · 2^(64L) in the window; small aligned below it, with the sticky
    // fraction for bits shifted out of the window.
    let mut window = <Wide<L> as CoreWidth>::DOUBLE_ZERO;
    window.as_mut()[L..].copy_from_slice(&big.significand);
    let mut aligned = <Wide<L> as CoreWidth>::DOUBLE_ZERO;
    let sticky = place(&small.significand, width - gap, aligned.as_mut());
    let scale = unit_exponent(big) - width;
    if big.negative == small.negative {
        if add_in_place(window.as_mut(), aligned.as_ref()) {
            let lost = shr1(window.as_mut());
            window.as_mut()[2 * L - 1] |= 1 << 63;
            round_limbs::<L>(big.negative, window.as_ref(), sticky || lost, scale + 1, p)
        } else {
            round_limbs::<L>(big.negative, window.as_ref(), sticky, scale, p)
        }
    } else {
        // big − (small + f) = (big − small − 1) + (1 − f), 1 − f ∈ (0, 1).
        sub_in_place(window.as_mut(), aligned.as_ref());
        if sticky {
            decrement(window.as_mut());
        } else if limbs_zero(window.as_ref()) {
            return Ok(signed_zero(false));
        }
        round_limbs::<L>(big.negative, window.as_ref(), sticky, scale, p)
    }
}

fn mul_rounded<const L: usize>(a: &Wide<L>, b: &Wide<L>, p: u32) -> Result<Wide<L>, WideError>
where
    Wide<L>: CoreWidth,
{
    let negative = a.negative != b.negative;
    if is_zero_value(a) || is_zero_value(b) {
        return Ok(signed_zero(negative));
    }
    let mut product = <Wide<L> as CoreWidth>::DOUBLE_ZERO;
    mul_limbs(&a.significand, &b.significand, product.as_mut());
    let scale = unit_exponent(a) + unit_exponent(b);
    round_limbs::<L>(negative, product.as_ref(), false, scale, p)
}

fn div_rounded<const L: usize>(a: &Wide<L>, b: &Wide<L>, p: u32) -> Result<Wide<L>, WideError>
where
    Wide<L>: CoreWidth,
{
    if is_zero_value(b) {
        return Err(WideError::DivisionByZero);
    }
    let negative = a.negative != b.negative;
    if is_zero_value(a) {
        return Ok(signed_zero(negative));
    }
    // floor(A · 2^(64L+1) / B) for normalized A, B: 64L + 1 or 64L + 2 bits.
    let top = 64 * L + 1;
    let mut quotient = <Wide<L> as CoreWidth>::DOUBLE_ZERO;
    let mut remainder = a.significand;
    if cmp_limbs(&remainder, &b.significand) != Ordering::Less {
        sub_in_place(&mut remainder, &b.significand);
        set_bit(quotient.as_mut(), top);
    }
    for k in 1..=top {
        let carry = shl_small(&mut remainder, 1) != 0;
        if carry || cmp_limbs(&remainder, &b.significand) != Ordering::Less {
            sub_in_place(&mut remainder, &b.significand);
            set_bit(quotient.as_mut(), top - k);
        }
    }
    let sticky = !limbs_zero(&remainder);
    let scale = i128::from(a.exponent) - i128::from(b.exponent) - top as i128;
    round_limbs::<L>(negative, quotient.as_ref(), sticky, scale, p)
}

fn sqrt_rounded<const L: usize>(a: &Wide<L>, p: u32) -> Result<Wide<L>, WideError>
where
    Wide<L>: CoreWidth,
{
    if is_zero_value(a) {
        return Ok(*a);
    }
    if a.negative {
        return Err(WideError::NegativeSqrt);
    }
    // a = m · 2^e; radicand m · 2^shift with e − shift even and 128L + 2 or
    // 128L + 3 bits, so the root has 64L + 1 or 64L + 2 bits.
    let e = unit_exponent(a);
    let base = 64 * L as i128 + 2;
    let shift = if (e - base).rem_euclid(2) == 0 {
        base
    } else {
        base + 1
    };
    let radicand_bit = |j: i128| -> u64 { u64::from(bit_at(&a.significand, j - shift)) };
    let used = L + 1;
    let mut remainder = <Wide<L> as CoreWidth>::DOUBLE_ZERO;
    let mut root = <Wide<L> as CoreWidth>::DOUBLE_ZERO;
    let mut trial = <Wide<L> as CoreWidth>::DOUBLE_ZERO;
    for i in (0..64 * L as i128 + 2).rev() {
        let rem = &mut remainder.as_mut()[..used];
        shl_small(rem, 2);
        rem[0] |= (radicand_bit(2 * i + 1) << 1) | radicand_bit(2 * i);
        let t = &mut trial.as_mut()[..used];
        t.copy_from_slice(&root.as_ref()[..used]);
        shl_small(t, 2);
        t[0] |= 1;
        let r = &mut root.as_mut()[..used];
        shl_small(r, 1);
        if cmp_limbs(rem, t) != Ordering::Less {
            sub_in_place(rem, t);
            r[0] |= 1;
        }
    }
    let sticky = !limbs_zero(remainder.as_ref());
    let scale = (e - shift).div_euclid(2);
    round_limbs::<L>(false, &root.as_ref()[..used], sticky, scale, p)
}

/// Knuth's TwoSum at p: (s, e) with s = fl_p(a + b) and s + e = a + b.
fn two_sum_rounded<const L: usize>(
    a: &Wide<L>,
    b: &Wide<L>,
    p: u32,
) -> Result<(Wide<L>, Wide<L>), WideError>
where
    Wide<L>: CoreWidth,
{
    if !fits(a, p) || !fits(b, p) {
        return Err(WideError::OperandPrecision);
    }
    let s = add_rounded(a, b, p)?;
    let bv = add_rounded(&s, &negated(a), p)?;
    let av = add_rounded(&s, &negated(&bv), p)?;
    let da = add_rounded(a, &negated(&av), p)?;
    let db = add_rounded(b, &negated(&bv), p)?;
    let e = add_rounded(&da, &db, p)?;
    Ok((s, e))
}

/// TwoProduct at p: (s, e) with s = fl_p(a · b) and s + e = a · b, formed as
/// the exact product minus its rounding.
fn two_product_rounded<const L: usize>(
    a: &Wide<L>,
    b: &Wide<L>,
    p: u32,
) -> Result<(Wide<L>, Wide<L>), WideError>
where
    Wide<L>: CoreWidth,
{
    if !fits(a, p) || !fits(b, p) {
        return Err(WideError::OperandPrecision);
    }
    let negative = a.negative != b.negative;
    if is_zero_value(a) || is_zero_value(b) {
        return Ok((signed_zero(negative), signed_zero(false)));
    }
    let mut product = <Wide<L> as CoreWidth>::DOUBLE_ZERO;
    mul_limbs(&a.significand, &b.significand, product.as_mut());
    let scale = unit_exponent(a) + unit_exponent(b);
    let (s, lsb, up) = round_detail::<L>(negative, product.as_ref(), false, scale, p)?;
    // The bits below the kept least significant bit; the residual is those
    // bits, or (when rounded up) 2^lsb minus them with the opposite sign.
    let mut residual = product;
    clear_from(residual.as_mut(), lsb);
    let residual_negative = if up {
        let low = residual;
        residual = <Wide<L> as CoreWidth>::DOUBLE_ZERO;
        set_bit(residual.as_mut(), lsb);
        sub_in_place(residual.as_mut(), low.as_ref());
        !negative
    } else {
        negative
    };
    if limbs_zero(residual.as_ref()) {
        return Ok((s, signed_zero(false)));
    }
    let e = round_limbs::<L>(residual_negative, residual.as_ref(), false, scale, p)?;
    Ok((s, e))
}

// ---------------------------------------------------------------------------
// Conversion to binary64 and widening (every width, L = 2 included)
// ---------------------------------------------------------------------------

/// The correctly rounded binary64 value of a `Wide`, with its outcome (module
/// documentation).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Binary64Outcome {
    /// A normal result, or an exact ±0 (sign kept). Rounded to nearest.
    Normal(f64),
    /// A subnormal result: absolute error at most 2^−1075, and
    /// `relative_precision` = 2^−1075/|value| rounded upward.
    Subnormal { value: f64, relative_precision: f64 },
    /// A nonzero value whose rounding is ±0. No value is returned.
    Underflow { negative: bool },
    /// A value whose rounding is ±∞. No value is returned.
    Overflow { negative: bool },
}

impl Binary64Outcome {
    /// The binary64 value, or None for underflow and overflow.
    pub fn value(&self) -> Option<f64> {
        match *self {
            Self::Normal(v) | Self::Subnormal { value: v, .. } => Some(v),
            Self::Underflow { .. } | Self::Overflow { .. } => None,
        }
    }
}

/// fl↑(1/(2k)) = 2^−1075/(k · 2^−1074), rounded upward, for 1 ≤ k < 2^52;
/// integer arithmetic only.
fn half_quantum_over(k: u64) -> f64 {
    debug_assert!(k >= 1 && k < 1 << 52);
    let d = 2 * u128::from(k);
    // d ∈ [2^(t−1), 2^t), so 1/d ∈ (2^−t, 2^−(t−1)].
    let t = 128 - d.leading_zeros() as i64;
    let (exponent, significand) = if d.is_power_of_two() {
        (-(t - 1), 1u128 << 52)
    } else {
        let numerator = 1u128 << (52 + t);
        let m = numerator.div_ceil(d);
        if m == 1u128 << 53 {
            (-(t - 1), 1u128 << 52)
        } else {
            (-t, m)
        }
    };
    let biased = (exponent + 1023) as u64;
    f64::from_bits((biased << 52) | (significand as u64 & ((1u64 << 52) - 1)))
}

impl<const L: usize> Wide<L> {
    /// The value rounded once to binary64, with its outcome (module
    /// documentation).
    #[allow(dead_code)] // K4 API (publication of retained quantities)
    pub(crate) fn to_binary64(&self) -> Binary64Outcome {
        let negative = self.negative;
        let sign = u64::from(negative) << 63;
        if is_zero_value(self) {
            return Binary64Outcome::Normal(f64::from_bits(sign));
        }
        let e = i128::from(self.exponent);
        if e > 1023 {
            return Binary64Outcome::Overflow { negative };
        }
        let mut quantum = (e - 52).max(-1074);
        // Index in the significand of the bit of weight 2^quantum.
        let index = quantum - unit_exponent(self);
        let significand = &self.significand[..];
        let mut kept = word_at(significand, index);
        let round = bit_at(significand, index - 1);
        let rest = any_below(significand, index - 1);
        if round && (rest || kept & 1 == 1) {
            kept += 1;
        }
        if kept == 1 << 53 {
            kept = 1 << 52;
            quantum += 1;
        }
        if kept == 0 {
            return Binary64Outcome::Underflow { negative };
        }
        if kept >= 1 << 52 {
            let leading = quantum + 52;
            if leading > 1023 {
                return Binary64Outcome::Overflow { negative };
            }
            let biased = (leading + 1023) as u64;
            Binary64Outcome::Normal(f64::from_bits(sign | (biased << 52) | (kept - (1 << 52))))
        } else {
            debug_assert_eq!(quantum, -1074);
            Binary64Outcome::Subnormal {
                value: f64::from_bits(sign | kept),
                relative_precision: half_quantum_over(kept),
            }
        }
    }

    /// The same value at a width M ≥ L (exact).
    #[allow(dead_code)] // K4 API (the stop rule and the p + 64 residual across widths)
    pub(crate) fn widen<const M: usize>(&self) -> Wide<M> {
        const { assert!(M >= L) };
        let mut significand = [0u64; M];
        significand[M - L..].copy_from_slice(&self.significand);
        Wide {
            negative: self.negative,
            exponent: self.exponent,
            significand,
        }
    }
}

// ---------------------------------------------------------------------------
// The value API of Wide<4>, Wide<8> and Wide<16> (K3a's names)
// ---------------------------------------------------------------------------

impl<const L: usize> Wide<L>
where
    Wide<L>: SupportedWidth,
{
    #[allow(dead_code)] // K4 API
    pub(crate) const ZERO: Self = Self {
        negative: false,
        exponent: 0,
        significand: [0; L],
    };
    #[allow(dead_code)] // K4 API
    pub(crate) const ONE: Self = {
        let mut significand = [0u64; L];
        significand[L - 1] = 1 << 63;
        Self {
            negative: false,
            exponent: 0,
            significand,
        }
    };

    /// Validated construction from raw parts (value model above).
    #[allow(dead_code)] // K4 API
    pub(crate) fn from_parts(
        negative: bool,
        exponent: i64,
        significand: [u64; L],
    ) -> Result<Self, WideError> {
        if limbs_zero(&significand) {
            return if exponent == 0 {
                Ok(Self {
                    negative,
                    exponent,
                    significand,
                })
            } else {
                Err(WideError::NotNormalized)
            };
        }
        if significand[L - 1] >> 63 == 0 {
            return Err(WideError::NotNormalized);
        }
        if !(-EXPONENT_LIMIT..=EXPONENT_LIMIT).contains(&exponent) {
            return Err(WideError::ExponentRange);
        }
        Ok(Self {
            negative,
            exponent,
            significand,
        })
    }

    /// (negative, exponent of the leading bit, little-endian significand):
    /// the canonical limbs (D1 §4.1.8's retained-state digest).
    #[allow(dead_code)] // K4 API
    pub(crate) fn parts(&self) -> (bool, i64, [u64; L]) {
        (self.negative, self.exponent, self.significand)
    }

    /// The exact value of a finite binary64 number, including subnormals and
    /// the sign of zero. NaN and infinities are refused.
    #[allow(dead_code)] // K4 API
    pub(crate) fn from_f64(value: f64) -> Result<Self, WideError> {
        if !value.is_finite() {
            return Err(WideError::NonFinite);
        }
        let bits = value.to_bits();
        let negative = bits >> 63 != 0;
        let biased = ((bits >> 52) & 0x7ff) as i64;
        let fraction = bits & ((1u64 << 52) - 1);
        let (integer, lsb) = match (biased, fraction) {
            (0, 0) => return Ok(signed_zero(negative)),
            (0, f) => (f, -1074),
            (b, f) => (f | (1u64 << 52), b - 1075),
        };
        let lz = integer.leading_zeros();
        let mut significand = [0u64; L];
        significand[L - 1] = integer << lz;
        Ok(Self {
            negative,
            exponent: lsb + 63 - i64::from(lz),
            significand,
        })
    }

    #[allow(dead_code)] // K4 API
    pub(crate) fn is_zero(&self) -> bool {
        is_zero_value(self)
    }

    #[allow(dead_code)] // K4 API
    pub(crate) fn is_sign_negative(&self) -> bool {
        self.negative
    }

    /// Exponent of the leading bit (0 for zero).
    #[allow(dead_code)] // K4 API
    pub(crate) fn exponent(&self) -> i64 {
        self.exponent
    }

    #[allow(dead_code)] // K4 API
    pub(crate) fn neg(&self) -> Self {
        negated(self)
    }

    #[allow(dead_code)] // K4 API
    pub(crate) fn abs(&self) -> Self {
        Self {
            negative: false,
            ..*self
        }
    }

    /// True when the value has at most p significant bits.
    #[allow(dead_code)] // K4 API
    pub(crate) fn fits_precision(&self, p: u32) -> bool {
        fits(self, p)
    }

    /// Exact multiplication by 2^k.
    #[allow(dead_code)] // K4 API
    pub(crate) fn mul_pow2(&self, k: i64) -> Result<Self, WideError> {
        if is_zero_value(self) {
            return Ok(*self);
        }
        let exponent = checked_exponent(i128::from(self.exponent) + i128::from(k))?;
        Ok(Self { exponent, ..*self })
    }

    /// Numerical comparison (+0 and −0 are equal).
    #[allow(dead_code)] // K4 API
    pub(crate) fn cmp_value(&self, other: &Self) -> Ordering {
        match (is_zero_value(self), is_zero_value(other)) {
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
                (false, false) => cmp_mag(self, other),
                (true, true) => cmp_mag(other, self),
            },
        }
    }
}

/// `Z+`/`Z-`, or the sign, the 16L hex digits of the significand and `p` with
/// the exponent (the scheme of K3a's `Wide<2>`).
impl<const L: usize> fmt::Debug for Wide<L>
where
    Wide<L>: SupportedWidth,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = if self.negative { '-' } else { '+' };
        if is_zero_value(self) {
            return write!(f, "Z{sign}");
        }
        write!(f, "{sign}")?;
        for limb in self.significand.iter().rev() {
            write!(f, "{limb:016x}")?;
        }
        write!(f, "p{}", self.exponent)
    }
}

// ---------------------------------------------------------------------------
// Work: counts by kind and width, and their cost
// ---------------------------------------------------------------------------

/// Operation kinds of a `WideContext`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OpKind {
    Add,
    Sub,
    Mul,
    Div,
    Sqrt,
    /// Narrowing, re-rounding and the integer constructor.
    Round,
    TwoSum,
    TwoProduct,
}

impl OpKind {
    pub(crate) const ALL: [OpKind; 8] = [
        OpKind::Add,
        OpKind::Sub,
        OpKind::Mul,
        OpKind::Div,
        OpKind::Sqrt,
        OpKind::Round,
        OpKind::TwoSum,
        OpKind::TwoProduct,
    ];
}

/// Cost of one operation of `kind` at width L, in limb-multiply equivalents
/// (the table in the module documentation).
pub(crate) fn limb_multiply_cost(kind: OpKind, limbs: usize) -> u64 {
    let l = limbs as u64;
    let steps = 64 * l + 2;
    match kind {
        OpKind::Add | OpKind::Sub | OpKind::Round => 2 * l,
        OpKind::Mul => l * l,
        OpKind::Div => (l + 1) * steps,
        OpKind::Sqrt => (l + 2) * steps,
        OpKind::TwoSum => 12 * l,
        OpKind::TwoProduct => l * l + 4 * l,
    }
}

/// Operations of one width, by kind. Saturating; never wraps.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct WidthWork {
    pub(crate) add: u64,
    pub(crate) sub: u64,
    pub(crate) mul: u64,
    pub(crate) div: u64,
    pub(crate) sqrt: u64,
    pub(crate) round: u64,
    pub(crate) two_sum: u64,
    pub(crate) two_product: u64,
}

impl WidthWork {
    /// The count of one kind.
    pub(crate) fn count(&self, kind: OpKind) -> u64 {
        match kind {
            OpKind::Add => self.add,
            OpKind::Sub => self.sub,
            OpKind::Mul => self.mul,
            OpKind::Div => self.div,
            OpKind::Sqrt => self.sqrt,
            OpKind::Round => self.round,
            OpKind::TwoSum => self.two_sum,
            OpKind::TwoProduct => self.two_product,
        }
    }

    fn count_mut(&mut self, kind: OpKind) -> &mut u64 {
        match kind {
            OpKind::Add => &mut self.add,
            OpKind::Sub => &mut self.sub,
            OpKind::Mul => &mut self.mul,
            OpKind::Div => &mut self.div,
            OpKind::Sqrt => &mut self.sqrt,
            OpKind::Round => &mut self.round,
            OpKind::TwoSum => &mut self.two_sum,
            OpKind::TwoProduct => &mut self.two_product,
        }
    }

    fn charge(&mut self, kind: OpKind) {
        let count = self.count_mut(kind);
        *count = count.saturating_add(1);
    }

    fn merge(&mut self, other: &Self) {
        for kind in OpKind::ALL {
            let count = self.count_mut(kind);
            *count = count.saturating_add(other.count(kind));
        }
    }

    /// All operations counted.
    #[allow(dead_code)] // K4 API (budgets)
    pub(crate) fn operations(&self) -> u64 {
        OpKind::ALL
            .iter()
            .fold(0u64, |sum, &kind| sum.saturating_add(self.count(kind)))
    }

    /// The counts' cost at width `limbs`, in limb-multiply equivalents.
    pub(crate) fn limb_multiply_equivalents(&self, limbs: usize) -> u64 {
        OpKind::ALL.iter().fold(0u64, |sum, &kind| {
            sum.saturating_add(
                self.count(kind)
                    .saturating_mul(limb_multiply_cost(kind, limbs)),
            )
        })
    }
}

/// The work of one attempt across widths (L = 4, 8, 16). Saturating.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AttemptWork {
    widths: [WidthWork; 3],
}

impl AttemptWork {
    /// Adds a context's counts to its width. Record each context once.
    #[allow(dead_code)] // K4 API (budgets)
    pub(crate) fn record<const L: usize>(&mut self, context: &WideContext<L>)
    where
        Wide<L>: SupportedWidth,
    {
        self.widths[<Wide<L> as SupportedWidth>::SLOT].merge(&context.work);
    }

    /// Adds another attempt's counts, width by width.
    #[allow(dead_code)] // K4 API (budgets)
    pub(crate) fn merge(&mut self, other: &Self) {
        for (mine, theirs) in self.widths.iter_mut().zip(&other.widths) {
            mine.merge(theirs);
        }
    }

    /// The counts of width L.
    #[allow(dead_code)] // K4 API (budgets)
    pub(crate) fn width<const L: usize>(&self) -> WidthWork
    where
        Wide<L>: SupportedWidth,
    {
        self.widths[<Wide<L> as SupportedWidth>::SLOT]
    }

    /// The whole attempt's cost in limb-multiply equivalents.
    pub fn limb_multiply_equivalents(&self) -> u64 {
        self.widths
            .iter()
            .zip(SLOT_LIMBS)
            .fold(0u64, |sum, (work, limbs)| {
                sum.saturating_add(work.limb_multiply_equivalents(limbs))
            })
    }
}

// ---------------------------------------------------------------------------
// The arithmetic context of L = 4, 8 and 16
// ---------------------------------------------------------------------------

/// Correctly rounded `Wide<L>` arithmetic at a runtime precision 2 ≤ p ≤ 64L,
/// with its work count.
#[derive(Debug, Clone)]
pub(crate) struct WideContext<const L: usize> {
    precision: u32,
    work: WidthWork,
}

impl<const L: usize> WideContext<L>
where
    Wide<L>: SupportedWidth,
{
    #[allow(dead_code)] // K4 API
    pub(crate) fn new(precision: u32) -> Result<Self, WideError> {
        if !(MIN_PRECISION..=(64 * L) as u32).contains(&precision) {
            return Err(WideError::InvalidPrecision(precision));
        }
        Ok(Self {
            precision,
            work: WidthWork::default(),
        })
    }

    #[allow(dead_code)] // K4 API
    pub(crate) fn precision(&self) -> u32 {
        self.precision
    }

    #[allow(dead_code)] // K4 API (budgets)
    pub(crate) fn work(&self) -> WidthWork {
        self.work
    }

    #[allow(dead_code)] // K4 API
    pub(crate) fn add(&mut self, a: &Wide<L>, b: &Wide<L>) -> Result<Wide<L>, WideError> {
        self.work.charge(OpKind::Add);
        add_rounded(a, b, self.precision)
    }

    #[allow(dead_code)] // K4 API
    pub(crate) fn sub(&mut self, a: &Wide<L>, b: &Wide<L>) -> Result<Wide<L>, WideError> {
        self.work.charge(OpKind::Sub);
        add_rounded(a, &negated(b), self.precision)
    }

    #[allow(dead_code)] // K4 API
    pub(crate) fn mul(&mut self, a: &Wide<L>, b: &Wide<L>) -> Result<Wide<L>, WideError> {
        self.work.charge(OpKind::Mul);
        mul_rounded(a, b, self.precision)
    }

    #[allow(dead_code)] // K4 API
    pub(crate) fn div(&mut self, a: &Wide<L>, b: &Wide<L>) -> Result<Wide<L>, WideError> {
        self.work.charge(OpKind::Div);
        div_rounded(a, b, self.precision)
    }

    #[allow(dead_code)] // K4 API
    pub(crate) fn sqrt(&mut self, a: &Wide<L>) -> Result<Wide<L>, WideError> {
        self.work.charge(OpKind::Sqrt);
        sqrt_rounded(a, self.precision)
    }

    /// A value of any width M rounded once to this width and precision:
    /// narrowing (M > L), re-rounding (M = L) or widening with rounding. A
    /// zero keeps its sign.
    #[allow(dead_code)] // K4 API
    pub(crate) fn round<const M: usize>(&mut self, x: &Wide<M>) -> Result<Wide<L>, WideError> {
        self.work.charge(OpKind::Round);
        round_from::<L, M>(x, self.precision)
    }

    /// ±magnitude · 2^exponent (little-endian limbs, any length) rounded once;
    /// +0 for a zero magnitude, whatever `negative`.
    #[allow(dead_code)] // K4 API (the ledger projection, D1 §4.1.2 item 5)
    pub(crate) fn from_integer(
        &mut self,
        negative: bool,
        magnitude: &[u64],
        exponent: i64,
    ) -> Result<Wide<L>, WideError> {
        self.work.charge(OpKind::Round);
        from_integer_rounded::<L>(negative, magnitude, exponent, self.precision)
    }

    /// (s, e): s = fl_p(a + b), s + e = a + b exactly. Operands of at most p
    /// significant bits.
    #[allow(dead_code)] // K4 API (exact assembly and combinations)
    pub(crate) fn two_sum(
        &mut self,
        a: &Wide<L>,
        b: &Wide<L>,
    ) -> Result<(Wide<L>, Wide<L>), WideError> {
        self.work.charge(OpKind::TwoSum);
        two_sum_rounded(a, b, self.precision)
    }

    /// (s, e): s = fl_p(a · b), s + e = a · b exactly. Operands of at most p
    /// significant bits.
    #[allow(dead_code)] // K4 API (exact assembly and combinations)
    pub(crate) fn two_product(
        &mut self,
        a: &Wide<L>,
        b: &Wide<L>,
    ) -> Result<(Wide<L>, Wide<L>), WideError> {
        self.work.charge(OpKind::TwoProduct);
        two_product_rounded(a, b, self.precision)
    }
}

#[cfg(test)]
#[path = "../../../../tests/retained_wide_k3/k3_tests.rs"]
mod tests;
