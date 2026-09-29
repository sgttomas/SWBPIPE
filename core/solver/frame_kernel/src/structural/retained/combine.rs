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
    run_schedule, CaseLimit, CaseOutcome, CasePrep, GroupCache, InvocationMeter, Refusal,
    RetainedSolve, UnresolvedReason,
};
use super::ledger::LedgerRefusal;
use std::sync::Arc;

/// Why a combination is not selected.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CombinationReason {
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
}

/// A combination's outcome: a selected solve (its own `RetainedSolve`, whose
/// evidence carries the combination's identity and combined ledger), or the
/// reason it is withheld with its attempts.
#[allow(dead_code)] // F2a API (F2a reads the outcome's fields)
#[derive(Debug, Clone)]
pub(crate) enum CombinationOutcome {
    Selected(Box<RetainedSolve>),
    Unresolved {
        reason: CombinationReason,
        attempts: Vec<AttemptRecord>,
    },
}

/// Combinations Σ cᵢ·(case i) of selected cases.
pub(crate) struct RetainedCombination;

impl RetainedCombination {
    /// The combination's own solve (module documentation).
    #[allow(dead_code)] // F2a API
    pub(crate) fn solve(
        operands: &[(f64, &RetainedSolve)],
        case_limit: CaseLimit,
        meter: &mut InvocationMeter,
    ) -> CombinationOutcome {
        let withheld = |reason| CombinationOutcome::Unresolved {
            reason,
            attempts: Vec::new(),
        };
        if operands.is_empty() || operands.iter().any(|o| !o.0.is_finite()) {
            return withheld(CombinationReason::NoOperands);
        }
        if operands.iter().any(|o| !o.1.prep.factors.is_empty()) {
            return withheld(CombinationReason::NestedCombination);
        }
        let first = operands[0].1;
        let identity = first.prep.source.stiffness_encoding();
        let (stations, supports) = (first.prep.source.stations(), first.prep.source.supports());
        if operands.iter().any(|(_, o)| {
            o.prep.source.stiffness_encoding() != identity
                || o.prep.layout != first.prep.layout
                || o.prep.source.stations() != stations
                || o.prep.source.supports() != supports
        }) {
            return withheld(CombinationReason::OperandsDiffer);
        }
        let preps: Vec<(f64, &CasePrep)> = operands
            .iter()
            .map(|(c, o)| (*c, o.prep.as_ref()))
            .collect();
        let prep = match CasePrep::combination(&preps) {
            Ok(p) => Arc::new(p),
            Err(e) => return withheld(CombinationReason::LedgerUnavailable(e)),
        };
        let mut cache = GroupCache::merged(operands.iter().map(|o| &o.1.cache));
        match run_schedule(prep, first.group.clone(), &mut cache, case_limit, meter) {
            CaseOutcome::Selected(solve) => CombinationOutcome::Selected(solve),
            CaseOutcome::Unresolved {
                reason: UnresolvedReason::Ceiling,
                attempts,
                ..
            } => CombinationOutcome::Unresolved {
                reason: CombinationReason::CombinationUnresolved,
                attempts,
            },
            CaseOutcome::Unresolved {
                reason, attempts, ..
            } => CombinationOutcome::Unresolved {
                reason: CombinationReason::Unresolved(reason),
                attempts,
            },
            CaseOutcome::Refused { refusal, .. } => withheld(CombinationReason::Refused(refusal)),
        }
    }
}

#[cfg(test)]
#[path = "../../../tests/retained_k4/combine_tests.rs"]
mod tests;
