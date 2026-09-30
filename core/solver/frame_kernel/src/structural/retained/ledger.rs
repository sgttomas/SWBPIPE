//! K4: the load ledger at precision p (T3 D1 §4.1.2 item 5).
//!
//! "Each DOF's load is the exact sum of its identified contributions, rounded
//! once to p." It is S11's ledger granularity (one term per authored
//! contribution, `FK/load_ledger.rs`) in S11's accumulator
//! (`FK/exact_sum.rs`'s `ExactAccumulator`, one `add` per nodal load), netted
//! once through the accumulator's own compare and subtract
//! (`ExactAccumulator::net_parts`, ROOT's K4 ruling Q3). The projection to p is
//! K3's `from_integer` of that netted integer (stale-design ruling 3).
//!
//! **Wherever the ledger joins another sum** (a prescribed-coupled right-hand
//! side, a residual, a reaction, a combination) it enters that sum exactly
//! (`add_to`), never as a value already rounded to p.
use super::source::{put_i64, put_u32, put_u64, Dof, PrimitiveSource};
#[cfg(test)]
use super::wide::multi::{SupportedWidth, WideContext};
#[cfg(test)]
use super::wide::Wide;
use super::wide_sum::{ExactWideSum, SumRefusal};
use crate::exact_sum::{ExactAccumulator, SumError};

/// One DOF's exact net: (−1)^negative · magnitude · 2^exponent, the magnitude
/// odd (lowest bit set), or empty for an exact zero (canonical form).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LedgerNet {
    pub(crate) negative: bool,
    pub(crate) magnitude: Vec<u64>,
    pub(crate) exponent: i64,
}

impl LedgerNet {
    fn from_accumulator(accumulator: &ExactAccumulator) -> Self {
        let (negative, magnitude, exponent) = accumulator.net_parts();
        let Some(low) = magnitude.iter().position(|&l| l != 0) else {
            return Self {
                negative: false,
                magnitude: Vec::new(),
                exponent: 0,
            };
        };
        let shift = low * 64 + magnitude[low].trailing_zeros() as usize;
        let top = magnitude.iter().rposition(|&l| l != 0).unwrap();
        let bits = top * 64 + 64 - magnitude[top].leading_zeros() as usize - shift;
        let mut out = vec![0u64; bits.div_ceil(64)];
        for (k, limb) in out.iter_mut().enumerate() {
            let at = shift + 64 * k;
            let (word, bit) = (at / 64, at % 64);
            let lo = magnitude.get(word).copied().unwrap_or(0);
            let hi = magnitude.get(word + 1).copied().unwrap_or(0);
            *limb = if bit == 0 {
                lo
            } else {
                (lo >> bit) | (hi << (64 - bit))
            };
        }
        while out.last() == Some(&0) {
            out.pop();
        }
        Self {
            negative,
            magnitude: out,
            exponent: exponent + shift as i64,
        }
    }

    pub(crate) fn is_zero(&self) -> bool {
        self.magnitude.is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedgerRefusal {
    /// The accumulator refused a term (`SumError`).
    Accumulator(SumError),
}

/// The exact per-DOF nets of a case (DOFs with at least one term, ascending).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RetainedLedger {
    entries: Vec<(usize, LedgerNet)>,
    /// Per entry: whether any of its terms is nonzero (D1 revision 5a.3,
    /// R7 §4.1.6.3 item 7a: "a nonzero ledger term", even when the net is 0).
    nonzero: Vec<bool>,
}

impl RetainedLedger {
    /// One `ExactAccumulator` per loaded DOF, one `add` per nodal load.
    pub(crate) fn from_source(source: &PrimitiveSource) -> Result<Self, LedgerRefusal> {
        // V-K seeded fault VK-F13 (§7.3-13): the loads folded at p = 128 in
        // listed order instead of the exact ledger.
        #[cfg(any(test, feature = "mutation-controls"))]
        if super::seeded::active(super::seeded::Fault::F13) {
            let mut entries: Vec<(usize, ExactAccumulator, bool)> = Vec::new();
            for (dof, terms, nonzero) in super::seeded::folded_loads(source) {
                let mut accumulator = ExactAccumulator::new();
                for term in terms {
                    accumulator.add(term).map_err(LedgerRefusal::Accumulator)?;
                }
                entries.push((dof, accumulator, nonzero));
            }
            return Ok(Self::netted(&entries));
        }
        let mut entries: Vec<(usize, ExactAccumulator, bool)> = Vec::new();
        for load in source.loads() {
            let dof = load.dof.global();
            let index = match entries.binary_search_by_key(&dof, |e| e.0) {
                Ok(k) => k,
                Err(k) => {
                    entries.insert(k, (dof, ExactAccumulator::new(), false));
                    k
                }
            };
            entries[index]
                .1
                .add(load.value)
                .map_err(LedgerRefusal::Accumulator)?;
            entries[index].2 |= load.value != 0.0;
        }
        Ok(Self::netted(&entries))
    }

    fn netted(entries: &[(usize, ExactAccumulator, bool)]) -> Self {
        Self {
            entries: entries
                .iter()
                .map(|(dof, acc, _)| (*dof, LedgerNet::from_accumulator(acc)))
                .collect(),
            nonzero: entries.iter().map(|e| e.2).collect(),
        }
    }

    /// A ledger with no terms: the recovery of a correction δ̂ "with the ledger
    /// omitted" (D1 revision 5a.3, R7 §4.1.6.3 item 3).
    pub(crate) fn empty() -> Self {
        Self {
            entries: Vec::new(),
            nonzero: Vec::new(),
        }
    }

    /// Whether a nonzero ledger term acts at the DOF (R7 §4.1.6.3 item 7a's
    /// data flag), whatever its net.
    pub(crate) fn has_nonzero_term(&self, dof: usize) -> bool {
        self.entries
            .binary_search_by_key(&dof, |e| e.0)
            .is_ok_and(|k| self.nonzero[k])
    }

    /// The ledger of a combination Σ cᵢ·(case i): per DOF, the exact products
    /// cᵢ·v of every load term of every operand (`add_product`, exact).
    pub(crate) fn combined(operands: &[(f64, &PrimitiveSource)]) -> Result<Self, LedgerRefusal> {
        let mut entries: Vec<(usize, ExactAccumulator, bool)> = Vec::new();
        for &(factor, source) in operands {
            for load in source.loads() {
                let dof = load.dof.global();
                let index = match entries.binary_search_by_key(&dof, |e| e.0) {
                    Ok(k) => k,
                    Err(k) => {
                        entries.insert(k, (dof, ExactAccumulator::new(), false));
                        k
                    }
                };
                entries[index]
                    .1
                    .add_product(factor, load.value)
                    .map_err(LedgerRefusal::Accumulator)?;
                entries[index].2 |= factor != 0.0 && load.value != 0.0;
            }
        }
        Ok(Self::netted(&entries))
    }

    /// The exact net of a DOF (None for a DOF with no term).
    pub(crate) fn net(&self, dof: usize) -> Option<&LedgerNet> {
        self.entries
            .binary_search_by_key(&dof, |e| e.0)
            .ok()
            .map(|k| &self.entries[k].1)
    }

    /// Adds the DOF's exact net (or its negation) to another sum, exactly.
    pub(crate) fn add_to(
        &self,
        dof: usize,
        sum: &mut ExactWideSum,
        negate: bool,
    ) -> Result<(), SumRefusal> {
        match self.net(dof) {
            Some(net) if !net.is_zero() => {
                sum.add_integer(net.negative != negate, &net.magnitude, net.exponent)
            }
            _ => Ok(()),
        }
    }

    /// The DOF's load at p: its exact net rounded once (+0 for an exact zero or
    /// an unloaded DOF).
    #[cfg(test)]
    pub(crate) fn project<const L: usize>(
        &self,
        dof: usize,
        context: &mut WideContext<L>,
    ) -> Result<Wide<L>, SumRefusal>
    where
        Wide<L>: SupportedWidth,
    {
        match self.net(dof) {
            Some(net) if !net.is_zero() => {
                Ok(context.from_integer(net.negative, &net.magnitude, net.exponent)?)
            }
            _ => Ok(context.from_integer(false, &[0], 0)?),
        }
    }

    /// The canonical byte encoding (ROOT's K4 ruling Q10): loaded DOFs
    /// ascending, each exact net as (sign, exponent, limb count, limbs) with an
    /// odd magnitude; an exactly cancelled DOF is (0, 0, 0).
    pub(crate) fn encoding(&self) -> Vec<u8> {
        let mut out = b"K4LED\x01".to_vec();
        put_u32(&mut out, self.entries.len() as u32);
        for (dof, net) in &self.entries {
            let d = Dof::from_global(*dof);
            put_u32(&mut out, d.node);
            out.push(d.component.index() as u8);
            out.push(u8::from(net.negative));
            put_i64(&mut out, net.exponent);
            put_u32(&mut out, net.magnitude.len() as u32);
            for &limb in &net.magnitude {
                put_u64(&mut out, limb);
            }
        }
        out
    }
}

#[cfg(test)]
#[path = "../../../tests/retained_k4/ledger_tests.rs"]
mod tests;
