//! K4: the correctly rounded exact multi-term sum (T3 D1 §4.1.2, "every
//! multi-term sum outside the factorization and the triangular solves is formed
//! exactly and rounded once"; RV12's S1; ROOT's K4 ruling Q2(a)).
//!
//! `ExactWideSum` is a signed wide-integer accumulator in the manner of
//! `ExactAccumulator`: the positive and the negative terms are added, exactly,
//! into two separate unsigned magnitudes held in fixed stack buffers, with a
//! floating anchor (the exponent of bit 0). The value is netted once (one
//! compare, one subtract) and rounded **once**, by K3's
//! `WideContext::from_integer`, to nearest with ties to even decided by every
//! bit of the magnitude. An exact zero is +0.
//!
//! Terms, each added exactly (never rounded):
//! - a `Wide<M>` value (M = 4, 8, 16), read through `parts()`;
//! - a `Wide<M>` value times a small integer and a power of two;
//! - a finite binary64 value;
//! - the exact product of two values of at most the context's precision, as
//!   K3's `two_product` pair (s + e = a·b exactly);
//! - an exact ±integer·2^exponent (the load ledger's netted value);
//! - another accumulator's exact value times a small integer and a power of two.
//!
//! **The span limit (ROOT's K4 ruling O4).** Each magnitude has 128 limbs (8,192
//! bits); 64 bits are kept as carry headroom, so a sum may span at most 8,128
//! bits, from its lowest set bit to the leading bit of its largest term. A term
//! that would exceed it is refused (`SumRefusal::Span`); an anchor outside `i64`
//! or a result outside the `Wide` exponent range is refused
//! (`SumRefusal::Exponent`). Nothing is ever truncated or wrapped. Every nonzero
//! binary64 value and every exact product of two binary64 values has its leading
//! bit in [2^-2148, 2^2048); a K4 term has at most 2,048 significant bits, so any
//! sum whose terms lie in that range spans at most 4,196 + 2,048 = 6,244 bits.
//!
//! No heap allocation: the magnitudes and every scratch buffer are on the stack,
//! and `clear` zeroes only the used prefix so one accumulator is reused.
//!
//! Work (`SumWork`, D1 §4.1.7): one limb-multiply equivalent per limb touched,
//! shifted, multiplied or netted, plus the magnitude length passed to
//! `from_integer` (K3's N3); `two_product` is counted by its own context.
use super::wide::multi::{SupportedWidth, WideContext};
use super::wide::{Wide, WideError};
use super::work::{WorkFault, WorkStatus, WorkTotal};
use std::cmp::Ordering;

/// Limbs of each magnitude.
pub(crate) const SUM_LIMBS: usize = 128;
/// Carry headroom kept below the top of each magnitude (2^64 terms).
const CARRY_BITS: i128 = 64;
/// The widest span, in bits, a sum may hold.
pub(crate) const SPAN_LIMIT_BITS: i128 = SUM_LIMBS as i128 * 64 - CARRY_BITS;
/// Scratch limbs for a scaled term.
const TERM_LIMBS: usize = SUM_LIMBS + 2;

/// A sum that cannot be formed exactly within its limits. Never a truncation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SumRefusal {
    CountRange(&'static str),
    WorkAccounting(WorkFault),
    /// The terms span more than `SPAN_LIMIT_BITS`.
    Span,
    /// The anchor or the rounded result is outside the exponent range.
    Exponent,
    /// A binary64 term is NaN or infinite.
    NonFinite,
    /// Another refusal of the `Wide` arithmetic (for example an operand wider
    /// than the context's precision).
    Wide(WideError),
}

impl From<WideError> for SumRefusal {
    fn from(error: WideError) -> Self {
        match error {
            WideError::CountRange(field) => Self::CountRange(field),
            WideError::WorkAccounting(fault) => Self::WorkAccounting(fault),
            WideError::ExponentRange => Self::Exponent,
            WideError::NonFinite => Self::NonFinite,
            other => Self::Wide(other),
        }
    }
}

/// Deterministic work of an accumulator (limb-multiply equivalents).
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct SumWork {
    status: WorkStatus,
    /// Limbs of the added terms, of their scaling multiplications and of the
    /// carries they propagated.
    pub(crate) term_limbs: u64,
    /// Limbs moved when the anchor was lowered.
    pub(crate) shift_limbs: u64,
    /// Limbs compared and subtracted by the nettings.
    pub(crate) net_limbs: u64,
    /// Magnitude limbs passed to `from_integer` (K3's N3).
    pub(crate) rounded_limbs: u64,
    /// The widest span, in bits, any sum reached (evidence of the headroom).
    pub(crate) max_span_bits: u64,
}

impl From<WorkFault> for SumRefusal {
    fn from(fault: WorkFault) -> Self {
        Self::WorkAccounting(fault)
    }
}

impl SumWork {
    pub fn checked_lme(&self) -> WorkTotal {
        [
            self.term_limbs,
            self.shift_limbs,
            self.net_limbs,
            self.rounded_limbs,
        ]
        .into_iter()
        .fold(WorkTotal::zero().join_status(self.status), |t, n| {
            t.add(WorkTotal::exact_count(n))
        })
    }
    pub fn limb_multiply_equivalents(&self) -> u64 {
        self.checked_lme().legacy_saturated()
    }
    pub(crate) fn merge(&mut self, other: &Self) {
        self.status = self.status.join(other.status);
        macro_rules! merge {
            ($field:ident) => {{
                let t =
                    WorkTotal::exact_count(self.$field).add(WorkTotal::exact_count(other.$field));
                self.status = self.status.join(t.status());
                self.$field = t.legacy_saturated();
            }};
        }
        merge!(term_limbs);
        merge!(shift_limbs);
        merge!(net_limbs);
        merge!(rounded_limbs);
        self.max_span_bits = self.max_span_bits.max(other.max_span_bits);
        self.status = self.status.join(self.checked_lme().status());
    }
    fn delta_since(self, before: Self) -> Self {
        let mut out = Self {
            status: self.status.join(before.status),
            max_span_bits: self.max_span_bits,
            ..Self::default()
        };
        macro_rules! delta {
            ($field:ident) => {{
                let t = WorkTotal::exact_count(self.$field)
                    .remainder(WorkTotal::exact_count(before.$field));
                out.status = out.status.join(t.status());
                out.$field = t.legacy_saturated();
            }};
        }
        delta!(term_limbs);
        delta!(shift_limbs);
        delta!(net_limbs);
        delta!(rounded_limbs);
        out.status = out.status.join(out.checked_lme().status());
        out
    }
    // Include the pending base term in both the component and aggregate reserve.
    fn reserve(&mut self, field: SumCharge, amount: u64, pending: u64) -> Result<(), WorkFault> {
        let component = match field {
            SumCharge::Term => self.term_limbs,
            SumCharge::Shift => self.shift_limbs,
            SumCharge::Net => self.net_limbs,
            SumCharge::Round => self.rounded_limbs,
        };
        let t = WorkTotal::exact_count(component).add(WorkTotal::exact_count(amount));
        let term = WorkTotal::exact_count(self.term_limbs)
            .add(WorkTotal::exact_count(pending))
            .add(WorkTotal::exact_count(
                if matches!(field, SumCharge::Term) {
                    amount
                } else {
                    0
                },
            ));
        let total = self
            .checked_lme()
            .add(WorkTotal::exact_count(amount))
            .add(WorkTotal::exact_count(pending))
            .join_status(t.status())
            .join_status(term.status());
        self.status = self.status.join(total.status());
        total.exact().map(|_| ())
    }
    fn charge(&mut self, field: SumCharge, amount: u64, pending: u64) -> Result<(), WorkFault> {
        self.reserve(field, amount, pending)?;
        let slot = match field {
            SumCharge::Term => &mut self.term_limbs,
            SumCharge::Shift => &mut self.shift_limbs,
            SumCharge::Net => &mut self.net_limbs,
            SumCharge::Round => &mut self.rounded_limbs,
        };
        *slot += amount; // the prospective component and total were checked above
        Ok(())
    }
}
#[derive(Clone, Copy)]
enum SumCharge {
    Term,
    Shift,
    Net,
    Round,
}
impl std::fmt::Debug for SumWork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("SumWork");
        d.field("term_limbs", &self.term_limbs)
            .field("shift_limbs", &self.shift_limbs)
            .field("net_limbs", &self.net_limbs)
            .field("rounded_limbs", &self.rounded_limbs)
            .field("max_span_bits", &self.max_span_bits);
        if !self.status.is_exact() {
            d.field("work_status", &self.status);
        }
        d.finish()
    }
}

type Magnitude = [u64; SUM_LIMBS];

/// The exact signed sum of its terms (module documentation).
#[derive(Clone)]
pub(crate) struct ExactWideSum {
    positive: Magnitude,
    negative: Magnitude,
    /// Exponent of bit 0 of both magnitudes (meaningful when not empty).
    anchor: i128,
    /// Limbs that may be nonzero in either magnitude.
    used: usize,
    /// No nonzero term since the last `clear`.
    empty: bool,
    /// The highest leading-bit exponent of any term.
    high: i128,
    work: SumWork,
    poisoned: bool,
}

impl Default for ExactWideSum {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for ExactWideSum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("ExactWideSum");
        d.field("anchor", &self.anchor)
            .field("used", &self.used)
            .field("empty", &self.empty);
        if self.poisoned {
            d.field("value_poisoned", &true);
        } else {
            d.field("signum", &self.signum_quiet());
        }
        d.finish()
    }
}

fn highest_bit(limbs: &[u64]) -> Option<usize> {
    let top = limbs.iter().rposition(|&l| l != 0)?;
    Some(top * 64 + 63 - limbs[top].leading_zeros() as usize)
}

fn lowest_bit(limbs: &[u64]) -> Option<usize> {
    let low = limbs.iter().position(|&l| l != 0)?;
    Some(low * 64 + limbs[low].trailing_zeros() as usize)
}

/// Bits [shift, shift + 64) of `limbs` (zero beyond them).
fn window(limbs: &[u64], shift: usize) -> u64 {
    let word = shift / 64;
    let bit = shift % 64;
    let low = limbs.get(word).copied().unwrap_or(0);
    if bit == 0 {
        return low;
    }
    let high = limbs.get(word + 1).copied().unwrap_or(0);
    (low >> bit) | (high << (64 - bit))
}

fn compare(a: &[u64], b: &[u64]) -> Ordering {
    for i in (0..a.len()).rev() {
        let order = a[i].cmp(&b[i]);
        if !order.is_eq() {
            return order;
        }
    }
    Ordering::Equal
}

impl ExactWideSum {
    pub(crate) fn new() -> Self {
        Self {
            positive: [0; SUM_LIMBS],
            negative: [0; SUM_LIMBS],
            anchor: 0,
            used: 0,
            empty: true,
            high: 0,
            work: SumWork::default(),
            poisoned: false,
        }
    }

    /// Resets the value to zero (the work counts are kept).
    pub(crate) fn clear(&mut self) {
        if self.poisoned {
            self.reset();
            return;
        }
        self.positive[..self.used].fill(0);
        self.negative[..self.used].fill(0);
        self.used = 0;
        self.empty = true;
        self.anchor = 0;
        self.high = 0;
    }

    /// Resets the value to zero after a refusal (T3 KF3; D1 revision 5a.3
    /// amendment A2; ROOT's ruling 7 on I19's plan): both magnitudes are zeroed
    /// in full, not only the used prefix `clear` zeroes, so a write that a
    /// refusal interrupted (the in-loop refusal of `add_raw`, unreachable under
    /// the span check) cannot reach a later sum. The work counts are kept.
    pub(crate) fn reset(&mut self) {
        self.poisoned = false;
        self.positive.fill(0);
        self.negative.fill(0);
        self.used = 0;
        self.empty = true;
        self.anchor = 0;
        self.high = 0;
    }

    #[cfg(test)]
    pub(crate) fn test_seed_term_work(&mut self, terms: u64) {
        self.work.term_limbs = terms;
    }

    pub(crate) fn work(&self) -> SumWork {
        self.work
    }

    /// Adds ±t·2^lsb, t a little-endian magnitude (any length, possibly with
    /// low zero bits or zero).
    pub(crate) fn ensure_valid(&self) -> Result<(), SumRefusal> {
        let status = if self.poisoned {
            self.work
                .status
                .join(WorkStatus::from_fault(WorkFault::Inconsistent))
        } else {
            self.work.status
        };
        status
            .fault()
            .map_or(Ok(()), |f| Err(SumRefusal::WorkAccounting(f)))
    }
    fn donor(&mut self, donor: &Self) -> Result<(), SumRefusal> {
        let result = donor.ensure_valid();
        if let Err(SumRefusal::WorkAccounting(fault)) = result {
            self.work.status = self.work.status.join(WorkStatus::from_fault(fault));
        }
        result
    }
    fn ready(&mut self) -> Result<(), SumRefusal> {
        if self.poisoned {
            self.work.status = self
                .work
                .status
                .join(WorkStatus::from_fault(WorkFault::Inconsistent));
        }
        self.ensure_valid()
    }
    fn inconsistent(&mut self, mutated: bool) -> SumRefusal {
        self.poisoned |= mutated;
        self.work.status = self
            .work
            .status
            .join(WorkStatus::from_fault(WorkFault::Inconsistent));
        SumRefusal::WorkAccounting(self.work.status.fault().unwrap())
    }
    fn add_raw(&mut self, negative: bool, t: &[u64], lsb: i128) -> Result<(), SumRefusal> {
        self.ready()?;
        super::wide::multi::integer_magnitude_bits(t.len())?;
        let (Some(lo), Some(hi)) = (lowest_bit(t), highest_bit(t)) else {
            return Ok(());
        };
        let term_low = lsb.checked_add(lo as i128).ok_or(SumRefusal::Exponent)?;
        let term_high = lsb.checked_add(hi as i128).ok_or(SumRefusal::Exponent)?;
        let (new_low, new_high) = if self.empty {
            (term_low, term_high)
        } else {
            (self.anchor.min(term_low), self.high.max(term_high))
        };
        let span = new_high
            .checked_sub(new_low)
            .and_then(|v| v.checked_add(1))
            .ok_or(SumRefusal::Span)?;
        if span > SPAN_LIMIT_BITS {
            return Err(SumRefusal::Span);
        }
        let limbs = (hi - lo + 1).div_ceil(64);
        if limbs > TERM_LIMBS {
            return Err(self.inconsistent(false));
        }
        let pending = limbs as u64 + 1;
        self.work.reserve(SumCharge::Term, 0, pending)?;
        if !self.empty && new_low < self.anchor {
            let shift =
                usize::try_from(self.anchor - new_low).map_err(|_| self.inconsistent(false))?;
            self.shift_up(shift, pending)?;
        }
        self.anchor = new_low;
        self.empty = false;
        self.high = new_high;
        self.work.max_span_bits = self.work.max_span_bits.max(span as u64);
        let mut trimmed = [0u64; TERM_LIMBS];
        for (k, limb) in trimmed.iter_mut().enumerate().take(limbs) {
            *limb = window(t, lo + 64 * k);
        }
        let offset =
            usize::try_from(term_low - self.anchor).map_err(|_| self.inconsistent(true))?;
        let bit = (offset % 64) as u32;
        let mut carry = 0u64;
        let mut index = offset / 64;
        for k in 0..=limbs {
            let current = if k < limbs { trimmed[k] } else { 0 };
            let previous = if k == 0 { 0 } else { trimmed[k - 1] };
            let part = if bit == 0 {
                current
            } else {
                (current << bit) | (previous >> (64 - bit))
            };
            if index >= SUM_LIMBS {
                if part != 0 || carry != 0 || trimmed[k.min(limbs)..limbs].iter().any(|&w| w != 0) {
                    return Err(self.inconsistent(true));
                }
                break;
            }
            let target = if negative {
                &mut self.negative
            } else {
                &mut self.positive
            };
            let (s1, c1) = target[index].overflowing_add(part);
            let (s2, c2) = s1.overflowing_add(carry);
            target[index] = s2;
            carry = u64::from(c1) + u64::from(c2);
            index += 1;
        }
        while carry != 0 {
            if index >= SUM_LIMBS {
                return Err(self.inconsistent(true));
            }
            if let Err(fault) = self.work.reserve(SumCharge::Term, 1, pending) {
                self.poisoned = true;
                return Err(fault.into());
            }
            let target = if negative {
                &mut self.negative
            } else {
                &mut self.positive
            };
            let (s, c) = target[index].overflowing_add(carry);
            target[index] = s;
            carry = u64::from(c);
            index += 1;
            self.work.term_limbs += 1; // reserved before mutation, including pending base
        }
        self.used = self.used.max(index.min(SUM_LIMBS));
        self.work.charge(SumCharge::Term, pending, 0)?;
        Ok(())
    }

    fn shift_up(&mut self, shift: usize, pending: u64) -> Result<(), SumRefusal> {
        let words = shift / 64;
        let bits = (shift % 64) as u32;
        let new_used = self
            .used
            .checked_add(words)
            .and_then(|v| v.checked_add(1))
            .ok_or_else(|| self.inconsistent(false))?
            .min(SUM_LIMBS);
        let outside = [&self.positive, &self.negative]
            .into_iter()
            .any(|magnitude| {
                highest_bit(magnitude)
                    .is_some_and(|h| h.checked_add(shift).is_none_or(|v| v >= SUM_LIMBS * 64))
            });
        if outside {
            return Err(self.inconsistent(false));
        }
        self.work
            .reserve(SumCharge::Shift, 2 * new_used as u64, pending)?;
        for magnitude in [&mut self.positive, &mut self.negative] {
            for i in (0..new_used).rev() {
                let high = i
                    .checked_sub(words)
                    .and_then(|j| magnitude.get(j))
                    .copied()
                    .unwrap_or(0);
                let low = if bits != 0 {
                    i.checked_sub(words + 1)
                        .and_then(|j| magnitude.get(j))
                        .copied()
                        .unwrap_or(0)
                        >> (64 - bits)
                } else {
                    0
                };
                magnitude[i] = if bits == 0 {
                    high
                } else {
                    (high << bits) | low
                };
            }
        }
        self.work.shift_limbs += 2 * new_used as u64;
        self.used = new_used;
        Ok(())
    }

    /// Adds ±x exactly.
    pub(crate) fn add_wide<const M: usize>(
        &mut self,
        x: &Wide<M>,
        negate: bool,
    ) -> Result<(), SumRefusal>
    where
        Wide<M>: SupportedWidth,
    {
        self.ready()?;
        if x.is_zero() {
            return Ok(());
        }
        let (negative, exponent, significand) = x.parts();
        let lsb = i128::from(exponent) - (64 * M as i128 - 1);
        self.add_raw(negative != negate, &significand, lsb)
    }

    /// Adds ±x·factor·2^pow2 exactly.
    pub(crate) fn add_wide_scaled<const M: usize>(
        &mut self,
        x: &Wide<M>,
        negate: bool,
        factor: u64,
        pow2: i64,
    ) -> Result<(), SumRefusal>
    where
        Wide<M>: SupportedWidth,
    {
        self.ready()?;
        if x.is_zero() || factor == 0 {
            return Ok(());
        }
        let (negative, exponent, significand) = x.parts();
        self.work.charge(SumCharge::Term, M as u64, 0)?;
        let mut scaled = [0u64; 17];
        let mut carry = 0u128;
        for (k, &limb) in significand.iter().enumerate() {
            let product = u128::from(limb) * u128::from(factor) + carry;
            scaled[k] = product as u64;
            carry = product >> 64;
        }
        scaled[M] = carry as u64;

        let lsb = i128::from(exponent) - (64 * M as i128 - 1) + i128::from(pow2);
        self.add_raw(negative != negate, &scaled[..=M], lsb)
    }

    /// Adds ±x exactly, for a finite binary64 x (NaN and infinities refused).
    pub(crate) fn add_binary64(&mut self, x: f64, negate: bool) -> Result<(), SumRefusal> {
        self.ready()?;
        if !x.is_finite() {
            return Err(SumRefusal::NonFinite);
        }
        if x == 0.0 {
            return Ok(());
        }
        let bits = x.to_bits();
        let biased = ((bits >> 52) & 0x7ff) as i128;
        let fraction = bits & ((1u64 << 52) - 1);
        let (significand, lsb) = if biased == 0 {
            (fraction, -1074)
        } else {
            (fraction | (1u64 << 52), biased - 1075)
        };
        self.add_raw(x.is_sign_negative() != negate, &[significand], lsb)
    }

    /// Adds ±a·b exactly, through `two_product` at the context's precision
    /// (both operands must have at most that many significant bits).
    pub(crate) fn add_product<const M: usize>(
        &mut self,
        context: &mut WideContext<M>,
        a: &Wide<M>,
        b: &Wide<M>,
        negate: bool,
    ) -> Result<(), SumRefusal>
    where
        Wide<M>: SupportedWidth,
    {
        self.ready()?;
        if a.is_zero() || b.is_zero() {
            return Ok(());
        }
        let (s, e) = context.two_product(a, b)?;
        self.add_wide(&s, negate)?;
        let result = self.add_wide(&e, negate);
        if result.is_err() && !s.is_zero() {
            self.poisoned = true;
        }
        result
    }

    /// Adds ±magnitude·2^exponent exactly (a little-endian magnitude of any
    /// length; zero adds nothing).
    pub(crate) fn add_integer(
        &mut self,
        negative: bool,
        magnitude: &[u64],
        exponent: i64,
    ) -> Result<(), SumRefusal> {
        self.add_raw(negative, magnitude, i128::from(exponent))
    }

    /// Adds ±other·factor·2^pow2 exactly.
    pub(crate) fn add_scaled(
        &mut self,
        other: &Self,
        negate: bool,
        factor: u64,
        pow2: i64,
    ) -> Result<(), SumRefusal> {
        self.ready()?;
        self.donor(other)?;
        if other.empty || factor == 0 {
            return Ok(());
        }
        let lsb = other.anchor + i128::from(pow2);
        let mut inserted = false;
        let result = (|| {
            for (negative, magnitude) in [(false, &other.positive), (true, &other.negative)] {
                self.work.charge(SumCharge::Term, other.used as u64, 0)?;
                let mut scaled = [0u64; TERM_LIMBS];
                let mut carry = 0u128;
                for k in 0..other.used {
                    let product = u128::from(magnitude[k]) * u128::from(factor) + carry;
                    scaled[k] = product as u64;
                    carry = product >> 64;
                }
                scaled[other.used] = carry as u64;

                self.add_raw(negative != negate, &scaled[..=other.used], lsb)?;
                inserted |= scaled[..=other.used].iter().any(|&v| v != 0);
            }
            Ok(())
        })();
        if result.is_err() && inserted {
            self.poisoned = true;
        }
        result
    }

    /// Replaces the value by its negation.
    fn negate(&mut self) {
        std::mem::swap(&mut self.positive, &mut self.negative);
    }

    fn signum_quiet(&self) -> i8 {
        if self.empty {
            return 0;
        }
        match compare(&self.positive[..self.used], &self.negative[..self.used]) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }

    /// Sign of the exact value: -1, 0 or 1.
    pub(crate) fn signum(&mut self) -> Result<i8, SumRefusal> {
        self.ready()?;
        self.work.charge(SumCharge::Net, self.used as u64, 0)?;
        Ok(self.signum_quiet())
    }

    pub(crate) fn is_zero(&mut self) -> Result<bool, SumRefusal> {
        Ok(self.signum()? == 0)
    }

    /// Replaces the value by its magnitude.
    pub(crate) fn make_absolute(&mut self) -> Result<(), SumRefusal> {
        if self.signum()? < 0 {
            self.negate();
        }
        Ok(())
    }

    /// Adds ±a·b exactly, the product of two accumulators' exact values (D1
    /// revision 5a.3's exact comparisons of ratios; ROOT's A3-0 ruling Q6): b
    /// is netted once, and each of its limbs scales a (`add_scaled`). The
    /// product is formed exactly or refused (`Span`, `Exponent`), never
    /// truncated; integers only, no allocation.
    pub(crate) fn add_product_of(
        &mut self,
        a: &Self,
        b: &mut Self,
        negate: bool,
    ) -> Result<(), SumRefusal> {
        self.ready()?;
        self.donor(a)?;
        self.donor(b)?;
        b.ready()?;
        if a.empty || b.empty {
            return Ok(());
        }
        let (negative, magnitude, used) = b.net()?;
        let mut inserted = false;
        let result = (|| {
            for (k, &limb) in magnitude[..used].iter().enumerate() {
                if limb == 0 {
                    continue;
                }
                let pow2 =
                    i64::try_from(b.anchor + 64 * k as i128).map_err(|_| SumRefusal::Exponent)?;
                self.add_scaled(a, negate != negative, limb, pow2)?;
                inserted = true;
            }
            Ok(())
        })();
        if result.is_err() && inserted {
            self.poisoned = true;
        }
        result
    }

    /// The netted exact value: (negative, magnitude, used limbs, anchor).
    fn net(&mut self) -> Result<(bool, Magnitude, usize), SumRefusal> {
        self.ready()?;
        let used = self.used;
        self.work.charge(SumCharge::Net, 2 * used as u64, 0)?;
        let (negative, big, small) = match compare(&self.positive[..used], &self.negative[..used]) {
            Ordering::Less => (true, &self.negative, &self.positive),
            _ => (false, &self.positive, &self.negative),
        };
        let mut out = [0u64; SUM_LIMBS];
        let mut borrow = false;
        for i in 0..used {
            let (x, b1) = big[i].overflowing_sub(small[i]);
            let (y, b2) = x.overflowing_sub(u64::from(borrow));
            out[i] = y;
            borrow = b1 || b2;
        }
        if borrow {
            return Err(self.inconsistent(false));
        }
        Ok((negative, out, used))
    }

    /// The exact value rounded once to the context's precision (nearest, ties
    /// to even); an exact zero is +0.
    pub(crate) fn round<const L: usize>(
        &mut self,
        context: &mut WideContext<L>,
    ) -> Result<Wide<L>, SumRefusal>
    where
        Wide<L>: SupportedWidth,
    {
        self.ready()?;
        if self.empty {
            return Ok(context.from_integer(false, &[0], 0)?);
        }
        let (negative, magnitude, used) = self.net()?;
        let anchor = i64::try_from(self.anchor).map_err(|_| SumRefusal::Exponent)?;
        self.work.charge(SumCharge::Round, used as u64, 0)?;
        Ok(context.from_integer(negative, &magnitude[..used.max(1)], anchor)?)
    }
}

#[cfg(test)]
#[path = "../../../tests/retained_k4/wide_sum_tests.rs"]
mod tests;

/// An inherited sum whose operation deltas can only come from this same clone.
pub(crate) struct CloneWork {
    sum: ExactWideSum,
}
impl CloneWork {
    pub(crate) fn new(sum: &ExactWideSum) -> Self {
        Self { sum: sum.clone() }
    }
    pub(crate) fn signum(&mut self) -> (Result<i8, SumRefusal>, SumWork) {
        let before = self.sum.work();
        let result = self.sum.signum();
        (result, self.sum.work().delta_since(before))
    }
    pub(crate) fn round<const L: usize>(
        &mut self,
        ctx: &mut WideContext<L>,
    ) -> (Result<Wide<L>, SumRefusal>, SumWork)
    where
        Wide<L>: SupportedWidth,
    {
        let before = self.sum.work();
        let result = self.sum.round(ctx);
        (result, self.sum.work().delta_since(before))
    }
}
