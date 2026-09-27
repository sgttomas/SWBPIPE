//! Per-case exact load ledger (S11 containment, sections 4.2 and 4.3).
//!
//! Producers push each load contribution, term by term, into one
//! `LoadLedger`. `finish` gives the `AssembledForce`: each DOF's correctly
//! rounded net of its terms, through `exact_sum`. The force vector of a typed
//! solve can be built only here: `AssembledForce` has no public constructor,
//! no `From`, `Default`, `Clone` or `Deserialize`, and no `&mut` access.
//! `ReducedForce` is built only by the kernel's typed reduction functions.
use crate::exact_sum::{ExactAccumulator, SumError};
use std::fmt;

/// One contribution as the producer formed it (S11 section 4.2 granularity).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ForceTermKind {
    /// A rounded formed term, pushed as its binary64 value.
    Term(f64),
    /// A load-proportional product, pushed exactly as `a * b`.
    Product(f64, f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForceTerm {
    pub source: String,
    pub dof: usize,
    pub kind: ForceTermKind,
}

impl ForceTerm {
    /// Adds this term (or its negation) exactly to an accumulator.
    pub fn accumulate(
        &self,
        accumulator: &mut ExactAccumulator,
        negate: bool,
    ) -> Result<(), SumError> {
        match self.kind {
            ForceTermKind::Term(value) => accumulator.add(if negate { -value } else { value }),
            ForceTermKind::Product(a, b) => accumulator.add_product(if negate { -a } else { a }, b),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LedgerError {
    EmptySource { index: usize },
    DofOutOfRange { dof: usize, size: usize },
    Sum { dof: usize, error: SumError },
}

impl fmt::Display for LedgerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySource { index } => write!(f, "load ledger term {index} has no source"),
            Self::DofOutOfRange { dof, size } => {
                write!(f, "load ledger DOF {dof} is outside a {size}-DOF system")
            }
            Self::Sum { dof, error } => write!(f, "load ledger DOF {dof}: {error}"),
        }
    }
}

impl std::error::Error for LedgerError {}

/// Evidence recorded by `finish`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LedgerEvidence {
    /// DOFs whose nonzero exact net rounded to +0.0 (a load below 2^-1075
    /// in SI units has been lost; range handling stays W2's).
    pub underflowed_dofs: Vec<usize>,
}

#[derive(Debug, Default)]
pub struct LoadLedger {
    terms: Vec<ForceTerm>,
}

impl LoadLedger {
    pub fn new() -> Self {
        Self::default()
    }

    /// Pushes one rounded formed term.
    pub fn push(&mut self, source: impl Into<String>, dof: usize, value: f64) {
        self.terms.push(ForceTerm {
            source: source.into(),
            dof,
            kind: ForceTermKind::Term(value),
        });
    }

    /// Pushes one load-proportional product, kept exact.
    pub fn push_product(&mut self, source: impl Into<String>, dof: usize, a: f64, b: f64) {
        self.terms.push(ForceTerm {
            source: source.into(),
            dof,
            kind: ForceTermKind::Product(a, b),
        });
    }

    pub fn terms(&self) -> &[ForceTerm] {
        &self.terms
    }

    /// Each DOF's correctly rounded net, rounded once. +0.0 for a zero net.
    pub fn finish(self, size: usize) -> Result<AssembledForce, LedgerError> {
        let mut by_dof = vec![Vec::new(); size];
        for (index, term) in self.terms.iter().enumerate() {
            if term.source.is_empty() {
                return Err(LedgerError::EmptySource { index });
            }
            if term.dof >= size {
                return Err(LedgerError::DofOutOfRange {
                    dof: term.dof,
                    size,
                });
            }
            by_dof[term.dof].push(index);
        }
        let mut values = Vec::with_capacity(size);
        let mut evidence = LedgerEvidence::default();
        for (dof, indices) in by_dof.iter().enumerate() {
            let mut accumulator = ExactAccumulator::new();
            for &index in indices {
                self.terms[index]
                    .accumulate(&mut accumulator, false)
                    .map_err(|error| LedgerError::Sum { dof, error })?;
            }
            let value = accumulator
                .round()
                .map_err(|error| LedgerError::Sum { dof, error })?;
            if value == 0.0 && !accumulator.is_zero() {
                evidence.underflowed_dofs.push(dof);
            }
            values.push(value);
        }
        Ok(AssembledForce {
            values,
            terms: self.terms,
            by_dof,
            evidence,
        })
    }
}

/// A case force vector built only by `LoadLedger::finish`.
#[derive(Debug)]
pub struct AssembledForce {
    values: Vec<f64>,
    terms: Vec<ForceTerm>,
    by_dof: Vec<Vec<usize>>,
    evidence: LedgerEvidence,
}

impl AssembledForce {
    pub fn values(&self) -> &[f64] {
        &self.values
    }

    pub fn get(&self, dof: usize) -> Option<f64> {
        self.values.get(dof).copied()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn terms(&self) -> &[ForceTerm] {
        &self.terms
    }

    /// The terms pushed for one DOF, in push order.
    pub fn terms_for(&self, dof: usize) -> impl Iterator<Item = &ForceTerm> + '_ {
        self.by_dof
            .get(dof)
            .into_iter()
            .flatten()
            .map(move |&index| &self.terms[index])
    }

    pub fn evidence(&self) -> &LedgerEvidence {
        &self.evidence
    }

    /// Adds this DOF's terms (or their negation) exactly.
    pub fn accumulate_dof(
        &self,
        dof: usize,
        accumulator: &mut ExactAccumulator,
        negate: bool,
    ) -> Result<(), SumError> {
        for term in self.terms_for(dof) {
            term.accumulate(accumulator, negate)?;
        }
        Ok(())
    }
}

/// A reduced right-hand side built only by the kernel's typed reductions.
#[derive(Debug, PartialEq)]
pub struct ReducedForce {
    values: Vec<f64>,
}

impl ReducedForce {
    pub(crate) fn from_exact_rows(values: Vec<f64>) -> Self {
        Self { values }
    }

    pub fn values(&self) -> &[f64] {
        &self.values
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finish_rounds_each_dof_once_and_records_underflow() {
        let mut ledger = LoadLedger::new();
        ledger.push("load:a", 0, 1e8);
        ledger.push("load:b", 0, 0.3);
        ledger.push("load:c", 0, -1e8);
        ledger.push_product("thermal:t", 1, 1.35, 4.1e7);
        ledger.push("nodal:n", 1, 1.3);
        ledger.push_product("thermal:t", 1, -1.35, 4.1e7);
        let p = 2.0_f64.powi(-540);
        ledger.push_product("tiny:t", 2, -p, p);
        let force = ledger.finish(4).unwrap();
        // Hand-derived: G terms cancel exactly, so the nets are 0.3 and 1.3.
        assert_eq!(force.values()[0], 0.3);
        assert_eq!(force.values()[1], 1.3);
        assert_eq!(force.values()[2].to_bits(), 0);
        assert_eq!(force.values()[3].to_bits(), 0);
        assert_eq!(force.evidence().underflowed_dofs, vec![2]);
        assert_eq!(force.terms_for(1).count(), 3);
        assert_eq!(force.terms().len(), 7);
        // Precondition: a binary64 fold of DOF 0 in push order is not 0.3.
        assert_ne!(1e8 + 0.3 - 1e8, 0.3);
    }

    #[test]
    fn finish_rejects_bad_terms() {
        let mut ledger = LoadLedger::new();
        ledger.push("load:a", 5, 1.0);
        assert_eq!(
            ledger.finish(3).unwrap_err(),
            LedgerError::DofOutOfRange { dof: 5, size: 3 }
        );
        let mut ledger = LoadLedger::new();
        ledger.push("", 0, 1.0);
        assert_eq!(
            ledger.finish(1).unwrap_err(),
            LedgerError::EmptySource { index: 0 }
        );
        let mut ledger = LoadLedger::new();
        ledger.push("load:a", 0, f64::NAN);
        assert_eq!(
            ledger.finish(1).unwrap_err(),
            LedgerError::Sum {
                dof: 0,
                error: SumError::NonFinite
            }
        );
    }
}
