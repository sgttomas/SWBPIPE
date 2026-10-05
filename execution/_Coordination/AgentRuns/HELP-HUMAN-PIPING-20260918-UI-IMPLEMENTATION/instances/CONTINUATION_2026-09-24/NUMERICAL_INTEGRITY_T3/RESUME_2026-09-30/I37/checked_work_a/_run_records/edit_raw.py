from pathlib import Path
p=Path('projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/wide_sum.rs');s=p.read_text()
s=s.replace('use std::cmp::Ordering;','use std::cmp::Ordering;\nuse super::work::{WorkFault, WorkStatus, WorkTotal};')
s=s.replace('pub(crate) enum SumRefusal {','pub(crate) enum SumRefusal {\n    WorkAccounting(WorkFault),').replace('WideError::ExponentRange =>','WideError::WorkAccounting(fault) => Self::WorkAccounting(fault),\n            WideError::ExponentRange =>')
s=s.replace('#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]\npub struct SumWork {','#[derive(Clone, Copy, Default, PartialEq, Eq)]\npub struct SumWork {\n    status: WorkStatus,')
a=s.index('impl SumWork {');b=s.index('\ntype Magnitude',a)
s=s[:a]+'''impl From<WorkFault> for SumRefusal {
    fn from(fault: WorkFault) -> Self { Self::WorkAccounting(fault) }
}

impl SumWork {
    pub fn checked_lme(&self) -> WorkTotal {
        [self.term_limbs, self.shift_limbs, self.net_limbs, self.rounded_limbs]
            .into_iter().fold(WorkTotal::zero().join_status(self.status), |t, n| t.add(WorkTotal::exact_count(n)))
    }
    pub fn limb_multiply_equivalents(&self) -> u64 { self.checked_lme().legacy_saturated() }
    pub(crate) fn merge(&mut self, other: &Self) {
        self.status = self.status.join(other.status);
        macro_rules! merge { ($field:ident) => {{
            let t = WorkTotal::exact_count(self.$field).add(WorkTotal::exact_count(other.$field));
            self.status = self.status.join(t.status());
            self.$field = t.legacy_saturated();
        }}; }
        merge!(term_limbs); merge!(shift_limbs); merge!(net_limbs); merge!(rounded_limbs);
        self.max_span_bits = self.max_span_bits.max(other.max_span_bits);
        self.status = self.status.join(self.checked_lme().status());
    }
    fn delta_since(self, before: Self) -> Self {
        let mut out = Self { status: self.status.join(before.status), max_span_bits: self.max_span_bits, ..Self::default() };
        macro_rules! delta { ($field:ident) => {{
            let t = WorkTotal::exact_count(self.$field).remainder(WorkTotal::exact_count(before.$field));
            out.status = out.status.join(t.status()); out.$field = t.legacy_saturated();
        }}; }
        delta!(term_limbs); delta!(shift_limbs); delta!(net_limbs); delta!(rounded_limbs);
        out.status = out.status.join(out.checked_lme().status());
        out
    }
    // Include the pending base term in both the component and aggregate reserve.
    fn reserve(&mut self, field: SumCharge, amount: u64, pending: u64) -> Result<(), WorkFault> {
        let component = match field { SumCharge::Term => self.term_limbs, SumCharge::Shift => self.shift_limbs, SumCharge::Net => self.net_limbs, SumCharge::Round => self.rounded_limbs };
        let t = WorkTotal::exact_count(component).add(WorkTotal::exact_count(amount));
        let term = WorkTotal::exact_count(self.term_limbs).add(WorkTotal::exact_count(pending))
            .add(WorkTotal::exact_count(if matches!(field, SumCharge::Term) { amount } else { 0 }));
        let total = self.checked_lme().add(WorkTotal::exact_count(amount)).add(WorkTotal::exact_count(pending))
            .join_status(t.status()).join_status(term.status());
        self.status = self.status.join(total.status());
        total.exact().map(|_| ())
    }
    fn charge(&mut self, field: SumCharge, amount: u64, pending: u64) -> Result<(), WorkFault> {
        self.reserve(field, amount, pending)?;
        let slot = match field { SumCharge::Term => &mut self.term_limbs, SumCharge::Shift => &mut self.shift_limbs, SumCharge::Net => &mut self.net_limbs, SumCharge::Round => &mut self.rounded_limbs };
        *slot += amount; // the prospective component and total were checked above
        Ok(())
    }
}
#[derive(Clone, Copy)]
enum SumCharge { Term, Shift, Net, Round }
impl std::fmt::Debug for SumWork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("SumWork");
        d.field("term_limbs", &self.term_limbs).field("shift_limbs", &self.shift_limbs)
            .field("net_limbs", &self.net_limbs).field("rounded_limbs", &self.rounded_limbs)
            .field("max_span_bits", &self.max_span_bits);
        if !self.status.is_exact() { d.field("work_status", &self.status); }
        d.finish()
    }
}
''' +s[b:]
s=s.replace('    work: SumWork,','    work: SumWork,\n    poisoned: bool,',1).replace('            work: SumWork::default(),','            work: SumWork::default(),\n            poisoned: false,',1)
a=s.index('        f.debug_struct("ExactWideSum")');b=s.index('\n    }',a)
s=s[:a]+'''        let mut d = f.debug_struct("ExactWideSum");
        d.field("anchor", &self.anchor).field("used", &self.used).field("empty", &self.empty);
        if self.poisoned { d.field("value_poisoned", &true); }
        else { d.field("signum", &self.signum_quiet()); }
        d.finish()'''+s[b:]
s=s.replace('    pub(crate) fn clear(&mut self) {','    pub(crate) fn clear(&mut self) {\n        if self.poisoned { self.reset(); return; }')
s=s.replace('    pub(crate) fn reset(&mut self) {','    pub(crate) fn reset(&mut self) {\n        self.poisoned = false;')
a=s.index('    fn add_raw(');b=s.index('    /// Adds ±x exactly.',a)
s=s[:a]+'''    fn ensure_valid(&self) -> Result<(), SumRefusal> {
        let status = if self.poisoned { self.work.status.join(WorkStatus::from_fault(WorkFault::Inconsistent)) } else { self.work.status };
        status.fault().map_or(Ok(()), |f| Err(SumRefusal::WorkAccounting(f)))
    }
    fn ready(&mut self) -> Result<(), SumRefusal> {
        if self.poisoned { self.work.status = self.work.status.join(WorkStatus::from_fault(WorkFault::Inconsistent)); }
        self.ensure_valid()
    }
    fn inconsistent(&mut self, mutated: bool) -> SumRefusal {
        self.poisoned |= mutated;
        self.work.status = self.work.status.join(WorkStatus::from_fault(WorkFault::Inconsistent));
        SumRefusal::WorkAccounting(self.work.status.fault().unwrap())
    }
    fn add_raw(&mut self, negative: bool, t: &[u64], lsb: i128) -> Result<(), SumRefusal> {
        self.ready()?;
        let (Some(lo), Some(hi)) = (lowest_bit(t), highest_bit(t)) else { return Ok(()); };
        let term_low = lsb.checked_add(lo as i128).ok_or(SumRefusal::Exponent)?;
        let term_high = lsb.checked_add(hi as i128).ok_or(SumRefusal::Exponent)?;
        let (new_low, new_high) = if self.empty { (term_low, term_high) } else { (self.anchor.min(term_low), self.high.max(term_high)) };
        let span = new_high.checked_sub(new_low).and_then(|v| v.checked_add(1)).ok_or(SumRefusal::Span)?;
        if span > SPAN_LIMIT_BITS { return Err(SumRefusal::Span); }
        let limbs = (hi - lo + 1).div_ceil(64);
        if limbs > TERM_LIMBS { return Err(self.inconsistent(false)); }
        let pending = limbs as u64 + 1;
        self.work.reserve(SumCharge::Term, 0, pending)?;
        if !self.empty && new_low < self.anchor {
            let shift = usize::try_from(self.anchor - new_low).map_err(|_| self.inconsistent(false))?;
            self.shift_up(shift, pending)?;
        }
        self.anchor = new_low;
        self.empty = false;
        self.high = new_high;
        self.work.max_span_bits = self.work.max_span_bits.max(span as u64);
        let mut trimmed = [0u64; TERM_LIMBS];
        for (k, limb) in trimmed.iter_mut().enumerate().take(limbs) { *limb = window(t, lo + 64 * k); }
        let offset = usize::try_from(term_low - self.anchor).map_err(|_| self.inconsistent(true))?;
        let bit = (offset % 64) as u32;
        let mut carry = 0u64;
        let mut index = offset / 64;
        for k in 0..=limbs {
            let current = if k < limbs { trimmed[k] } else { 0 };
            let previous = if k == 0 { 0 } else { trimmed[k - 1] };
            let part = if bit == 0 { current } else { (current << bit) | (previous >> (64 - bit)) };
            if index >= SUM_LIMBS {
                if part != 0 || carry != 0 || trimmed[k.min(limbs)..limbs].iter().any(|&w| w != 0) { return Err(self.inconsistent(true)); }
                break;
            }
            let target = if negative { &mut self.negative } else { &mut self.positive };
            let (s1, c1) = target[index].overflowing_add(part);
            let (s2, c2) = s1.overflowing_add(carry);
            target[index] = s2;
            carry = u64::from(c1) + u64::from(c2);
            index += 1;
        }
        while carry != 0 {
            if index >= SUM_LIMBS { return Err(self.inconsistent(true)); }
            if let Err(fault) = self.work.reserve(SumCharge::Term, 1, pending) { self.poisoned = true; return Err(fault.into()); }
            let target = if negative { &mut self.negative } else { &mut self.positive };
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
        let new_used = self.used.checked_add(words).and_then(|v| v.checked_add(1)).ok_or_else(|| self.inconsistent(false))?.min(SUM_LIMBS);
        for magnitude in [&self.positive, &self.negative] {
            if highest_bit(magnitude).is_some_and(|h| h.checked_add(shift).is_none_or(|v| v >= SUM_LIMBS * 64)) { return Err(self.inconsistent(false)); }
        }
        self.work.reserve(SumCharge::Shift, 2 * new_used as u64, pending)?;
        for magnitude in [&mut self.positive, &mut self.negative] {
            for i in (0..new_used).rev() {
                let high = i.checked_sub(words).and_then(|j| magnitude.get(j)).copied().unwrap_or(0);
                let low = if bits != 0 { i.checked_sub(words + 1).and_then(|j| magnitude.get(j)).copied().unwrap_or(0) >> (64 - bits) } else { 0 };
                magnitude[i] = if bits == 0 { high } else { (high << bits) | low };
            }
        }
        self.work.shift_limbs += 2 * new_used as u64;
        self.used = new_used;
        Ok(())
    }

'''+s[b:]
s=s.replace('        if x.is_zero() {','        self.ready()?;\n        if x.is_zero() {').replace('        if x.is_zero() || factor == 0 {','        self.ready()?;\n        if x.is_zero() || factor == 0 {')
s=s.replace('        let mut scaled = [0u64; 17];','        self.work.charge(SumCharge::Term, M as u64, 0)?;\n        let mut scaled = [0u64; 17];').replace('        self.work.term_limbs += M as u64;','')
s=s.replace('        if !x.is_finite() {','        self.ready()?;\n        if !x.is_finite() {')
s=s.replace('        if a.is_zero() || b.is_zero() {','        self.ready()?;\n        if a.is_zero() || b.is_zero() {')
s=s.replace('        self.add_wide(&e, negate)','        let result = self.add_wide(&e, negate);\n        if result.is_err() && !s.is_zero() { self.poisoned = true; }\n        result')
s=s.replace('        if other.empty || factor == 0 {','        self.ready()?;\n        other.ensure_valid()?;\n        if other.empty || factor == 0 {')
s=s.replace('        for (negative, magnitude) in [(false, &other.positive), (true, &other.negative)] {','        let mut inserted = false;\n        let result = (|| {\n        for (negative, magnitude) in [(false, &other.positive), (true, &other.negative)] {\n            self.work.charge(SumCharge::Term, other.used as u64, 0)?;')
s=s.replace('            self.work.term_limbs += other.used as u64;','').replace('            self.add_raw(negative != negate, &scaled[..=other.used], lsb)?;\n        }\n        Ok(())','            self.add_raw(negative != negate, &scaled[..=other.used], lsb)?;\n            inserted |= scaled[..=other.used].iter().any(|&v| v != 0);\n        }\n        Ok(())\n        })();\n        if result.is_err() && inserted { self.poisoned = true; }\n        result')
s=s.replace('pub(crate) fn negate','fn negate')
s=s.replace('pub(crate) fn signum(&mut self) -> i8 {\n        self.work.net_limbs += self.used as u64;\n        self.signum_quiet()','pub(crate) fn signum(&mut self) -> Result<i8, SumRefusal> {\n        self.ready()?;\n        self.work.charge(SumCharge::Net, self.used as u64, 0)?;\n        Ok(self.signum_quiet())')
s=s.replace('pub(crate) fn is_zero(&mut self) -> bool {\n        self.signum() == 0','pub(crate) fn is_zero(&mut self) -> Result<bool, SumRefusal> {\n        Ok(self.signum()? == 0)')
s=s.replace('pub(crate) fn make_absolute(&mut self) {\n        if self.signum() < 0 {\n            self.negate();\n        }','pub(crate) fn make_absolute(&mut self) -> Result<(), SumRefusal> {\n        if self.signum()? < 0 {\n            self.negate();\n        }\n        Ok(())')
s=s.replace('        if a.empty || b.empty {','        self.ready()?;\n        a.ensure_valid()?;\n        b.ready()?;\n        if a.empty || b.empty {').replace('let (negative, magnitude, used) = b.net();','let (negative, magnitude, used) = b.net()?;')
s=s.replace('        for (k, &limb) in magnitude[..used].iter().enumerate() {','        let mut inserted = false;\n        let result = (|| {\n        for (k, &limb) in magnitude[..used].iter().enumerate() {').replace('            self.add_scaled(a, negate != negative, limb, pow2)?;\n        }\n        Ok(())','            self.add_scaled(a, negate != negative, limb, pow2)?;\n            inserted = true;\n        }\n        Ok(())\n        })();\n        if result.is_err() && inserted { self.poisoned = true; }\n        result')
s=s.replace('fn net(&mut self) -> (bool, Magnitude, usize) {','fn net(&mut self) -> Result<(bool, Magnitude, usize), SumRefusal> {\n        self.ready()?;').replace('        self.work.net_limbs += 2 * used as u64;','        self.work.charge(SumCharge::Net, 2 * used as u64, 0)?;').replace('        debug_assert!(!borrow);\n        (negative, out, used)','        if borrow { return Err(self.inconsistent(false)); }\n        Ok((negative, out, used))')
s=s.replace('        if self.empty {\n            return Ok(context.from_integer','        self.ready()?;\n        if self.empty {\n            return Ok(context.from_integer').replace('let (negative, magnitude, used) = self.net();','let (negative, magnitude, used) = self.net()?;').replace('        self.work.rounded_limbs += used as u64;','        self.work.charge(SumCharge::Round, used as u64, 0)?;')
s+='''
/// An inherited sum whose operation deltas can only come from this same clone.
pub(crate) struct CloneWork { sum: ExactWideSum }
impl CloneWork {
    pub(crate) fn new(sum: &ExactWideSum) -> Self { Self { sum: sum.clone() } }
    pub(crate) fn signum(&mut self) -> (Result<i8, SumRefusal>, SumWork) {
        let before = self.sum.work(); let result = self.sum.signum();
        (result, self.sum.work().delta_since(before))
    }
    pub(crate) fn round<const L: usize>(&mut self, ctx: &mut WideContext<L>) -> (Result<Wide<L>, SumRefusal>, SumWork) where Wide<L>: SupportedWidth {
        let before = self.sum.work(); let result = self.sum.round(ctx);
        (result, self.sum.work().delta_since(before))
    }
}
'''
p.write_text(s)
