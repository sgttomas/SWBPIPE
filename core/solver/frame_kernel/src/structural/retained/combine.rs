//! K4: combinations (T3 D1 §4.1.1 and §4.1.2, as refined by ROOT's ruling on
//! I12's A1 finding F-1).
//!
//! **A combination is its own solve.** "Its right-hand side is one exact
//! expansion of the combined ledger Σcᵢfᵢ and the combined prescribed coupling,
//! rounded once to p. Its prescribed values are the exact combination, rounded
//! once. It reuses the cached factor of the shared stiffness source, and runs
//! its own schedule and stop rule. The operands' results never change." The
//! combined ledger holds the exact products cᵢ·v of every operand's load terms
//! (`RetainedLedger::combined`); each prescribed value is the exact sum of the
//! products cᵢ·vᵢ, rounded once at p (`CasePrep::combination`). The combination
//! then runs the case schedule (`run_schedule`) over 128, 256 and 512, each
//! verified at 2p, ceiling 1024, with the stop rule and S\* of its own values:
//! "its published outputs go through the stop rule exactly as case outputs do,
//! with S\* taken from the combination's own body scale", and "a combination
//! whose candidates disagree escalates independently of its operand cases. At
//! the ceiling, it is withheld … (reason `combination_unresolved`), and its
//! operand cases keep their standing."
//!
//! The former formation Σcᵢ·uᵢ of the operands' retained states is withdrawn
//! (ROOT, F-1): with one linear K the two agree in exact arithmetic, but Σcᵢuᵢ
//! loses operand loads that cancel below both p's and 2p's resolution, and
//! the stop rule cannot see a loss common to both. No operand state is used,
//! so no operand is solved on demand.
//!
//! Rules: the operands are case solves (not combinations) sharing one
//! stiffness identity (nodes, members, springs, directional springs and the
//! constrained DOF set), one published layout, and the same stations and
//! support groups (ROOT's ruling on RV19-3: a station's fraction and a group's
//! members are not in the layout, which holds only their ids). The combination's shared
//! stages at p come from the operands' caches when any of them built (or
//! failed) them, and are counted in full against the combination's case limit
//! and against the invocation only when built here, as for any case of a
//! group.
use super::adaptive::AttemptRecord;
use super::adaptive::{
    run_core, CaseLimit, CasePrep, CombinationPreparationError, ExecutionOutcome, GroupCache,
    InvocationMeter, PreparedCaseSource, Refusal, RetainedSolve, RunWork, UnresolvedReason,
};
use super::ledger::LedgerRefusal;
use super::work::WorkTotal;
use std::sync::Arc;

/// Why a combination is not selected.
#[derive(Debug, Clone, PartialEq)]
pub enum CombinationReason {
    CountRange(&'static str),
    /// "At the ceiling … `combination_unresolved`".
    CombinationUnresolved,
    /// No operand, or a non-finite factor.
    NoOperands,
    /// An operand is itself a combination.
    NestedCombination,
    /// The operands differ in stiffness identity, published layout, stations
    /// or support groups.
    OperandsDiffer,
    LedgerUnavailable(LedgerRefusal),
    /// Another terminal reason of the schedule (budget, span, exponent).
    Unresolved(UnresolvedReason),
    /// A terminal refusal of the schedule (not expected: the operands' factor
    /// passed).
    Refused(Refusal),
    /// B2-K (KD R-3, decision 5): no operand is a selected solve. A combination
    /// takes its group from a selected operand, so it is never selected without
    /// one. Pre-source, operand validation, after the native checks.
    NoSelectedOperand,
}

/// B2-K (KD §1.2): a combination operand. `Retained` is a selected solve (its
/// cache may be imported); `Prepared` is a case's preparation only. Neither is
/// solved, mutated or re-prepared by a combination (I3).
#[derive(Debug, Clone, Copy)]
pub enum CombinationOperand<'a> {
    Retained(&'a RetainedSolve),
    Prepared(&'a PreparedCaseSource),
}
impl<'a> CombinationOperand<'a> {
    pub(crate) fn prep(&self) -> &'a CasePrep {
        match *self {
            Self::Retained(solve) => &solve.prep,
            Self::Prepared(prepared) => prepared.prep(),
        }
    }
    pub(crate) fn selected(&self) -> Option<&'a RetainedSolve> {
        match *self {
            Self::Retained(solve) => Some(solve),
            Self::Prepared(_) => None,
        }
    }
}

/// A combination's outcome: a selected solve (its own `RetainedSolve`, whose
/// evidence carries the combination's identity and combined ledger), or the
/// reason it is withheld with its attempts.
#[derive(Debug, Clone)]
pub enum CombinationOutcome {
    Selected(Box<RetainedSolve>),
    Unresolved {
        reason: CombinationReason,
        attempts: Vec<AttemptRecord>,
    },
}

/// Additive terminal custody. Source/group/build origin inventories are not supplied.
#[derive(Debug, Clone)]
pub enum RecordedCombination {
    PreSourceRefusal {
        outcome: CombinationOutcome,
        invocation_before: WorkTotal,
        invocation_after: WorkTotal,
    },
    WithRun {
        outcome: CombinationOutcome,
        work: RunWork,
    },
}
impl RecordedCombination {
    pub fn into_legacy(self) -> CombinationOutcome {
        let mut outcome = match self {
            Self::PreSourceRefusal { outcome, .. } | Self::WithRun { outcome, .. } => outcome,
        };
        if let CombinationOutcome::Unresolved {
            reason: CombinationReason::Refused(_),
            attempts,
        } = &mut outcome
        {
            attempts.clear();
        }
        outcome
    }
}

/// Combinations Σ cᵢ·(case i) of selected cases.
pub struct RetainedCombination;

impl RetainedCombination {
    /// The combination's own solve (module documentation).
    pub fn solve(
        operands: &[(f64, &RetainedSolve)],
        case_limit: CaseLimit,
        meter: &mut InvocationMeter,
    ) -> CombinationOutcome {
        Self::solve_recorded(operands, case_limit, meter).into_legacy()
    }

    pub fn solve_recorded(
        operands: &[(f64, &RetainedSolve)],
        case_limit: CaseLimit,
        meter: &mut InvocationMeter,
    ) -> RecordedCombination {
        let sources: Vec<(f64, CombinationOperand<'_>)> = operands
            .iter()
            .map(|&(factor, solve)| (factor, CombinationOperand::Retained(solve)))
            .collect();
        Self::solve_sources_recorded(&sources, case_limit, meter)
    }

    /// B2-K: a combination whose operands may be prepared case sources. Every
    /// all-`Retained` call is byte-identical to `solve` (KD I2).
    pub fn solve_sources(
        operands: &[(f64, CombinationOperand<'_>)],
        case_limit: CaseLimit,
        meter: &mut InvocationMeter,
    ) -> CombinationOutcome {
        Self::solve_sources_recorded(operands, case_limit, meter).into_legacy()
    }

    /// KD §1.4's unrecorded order: `NoOperands`, `NestedCombination`,
    /// `OperandsDiffer`, then `NoSelectedOperand`, then the combined
    /// preparation, then the run on the first selected operand's group with the
    /// selected operands' caches merged in authored order (I4).
    pub fn solve_sources_recorded(
        operands: &[(f64, CombinationOperand<'_>)],
        case_limit: CaseLimit,
        meter: &mut InvocationMeter,
    ) -> RecordedCombination {
        let before = meter.checked_charged();
        let withheld = |reason| RecordedCombination::PreSourceRefusal {
            outcome: CombinationOutcome::Unresolved {
                reason,
                attempts: Vec::new(),
            },
            invocation_before: before,
            invocation_after: before,
        };
        let preps: Vec<(f64, &CasePrep)> =
            operands.iter().map(|(c, o)| (*c, o.prep())).collect();
        if let Err(reason) = validate_preps(&preps) {
            return withheld(reason);
        }
        let Some(first) = operands.iter().find_map(|o| o.1.selected()) else {
            return withheld(CombinationReason::NoSelectedOperand);
        };
        let prep = match prepare_preps(&preps) {
            Ok(prep) => prep,
            Err(reason) => return withheld(reason),
        };
        let mut cache = GroupCache::merged(operands.iter().filter_map(|o| o.1.selected()).map(|s| &s.cache));
        let run = run_core(prep, first.group.clone(), &mut cache, case_limit, meter);
        let outcome = match run.outcome {
            ExecutionOutcome::Selected(solve) => CombinationOutcome::Selected(solve),
            ExecutionOutcome::Unresolved {
                reason: UnresolvedReason::Ceiling,
                attempts,
                ..
            } => CombinationOutcome::Unresolved {
                reason: CombinationReason::CombinationUnresolved,
                attempts,
            },
            ExecutionOutcome::Unresolved {
                reason, attempts, ..
            } => CombinationOutcome::Unresolved {
                reason: CombinationReason::Unresolved(reason),
                attempts,
            },
            ExecutionOutcome::Refused {
                refusal, attempts, ..
            } => CombinationOutcome::Unresolved {
                reason: CombinationReason::Refused(refusal),
                attempts,
            },
        };
        RecordedCombination::WithRun {
            outcome,
            work: run.work,
        }
    }
}

/// KD §1.4 steps 1 to 3 on each operand's `CasePrep`, whatever its kind, in
/// today's order: no operands, nested, operands differ (K4STF bytes, layout,
/// stations, supports).
pub(crate) fn validate_preps(operands: &[(f64, &CasePrep)]) -> Result<(), CombinationReason> {
    if operands.is_empty() || operands.iter().any(|o| !o.0.is_finite()) {
        return Err(CombinationReason::NoOperands);
    }
    if operands.iter().any(|o| !o.1.factors.is_empty()) {
        return Err(CombinationReason::NestedCombination);
    }
    let first = operands[0].1;
    let identity = first.source.stiffness_encoding();
    let (stations, supports) = (first.source.stations(), first.source.supports());
    if operands.iter().any(|(_, o)| {
        o.source.stiffness_encoding() != identity
            || o.layout != first.layout
            || o.source.stations() != stations
            || o.source.supports() != supports
    }) {
        return Err(CombinationReason::OperandsDiffer);
    }
    Ok(())
}

pub(crate) fn prepare_preps(
    preps: &[(f64, &CasePrep)],
) -> Result<Arc<CasePrep>, CombinationReason> {
    match CasePrep::combination(preps) {
        Ok(p) => Ok(Arc::new(p)),
        Err(CombinationPreparationError::Ledger(e)) => {
            return Err(CombinationReason::LedgerUnavailable(e))
        }
        Err(CombinationPreparationError::CountRange(field)) => {
            return Err(CombinationReason::CountRange(field))
        }
    }
}

#[cfg(test)]
#[path = "../../../tests/retained_k4/combine_tests.rs"]
mod tests;
