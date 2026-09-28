//! K4: combinations over retained states (T3 D1 §4.1.1, revised for V1-B1).
//!
//! "`RetainedCombination::form(&[(factor, &RetainedSolve)], p)` … forms
//! `Σ cᵢ·uᵢ` and `Σ cᵢ·fᵢ` as one exact expansion per component: TwoProduct of
//! the binary64 factor and the p-bit state, then TwoSum accumulation. Each is
//! rounded once to p, and actions and reactions are recovered from the combined
//! state at p." Σ cᵢ·fᵢ is the combined exact ledger (`RetainedLedger::combined`,
//! the exact products cᵢ·v of every operand's load terms), so the ledger enters
//! the reactions exactly.
//!
//! "It is formed at p and at 2p from the two precisions' retained states. Its
//! published outputs go through the stop rule exactly as case outputs do, with
//! S\* taken from the combination's own body scale." "A combination whose
//! candidates disagree escalates independently of its operand cases. At the
//! ceiling, it is withheld … (reason `combination_unresolved`), and its operand
//! cases keep their standing."
//!
//! Rules (ROOT's K4 ruling O7): the operands share one stiffness identity and
//! one published layout; the schedule starts at the largest precision at which
//! any operand was selected; an operand state the combination needs and the
//! operand did not solve is solved on demand, charged to the combination, and
//! never changes the operand's values, outcome or evidence.
use super::adaptive::RetainedSolve;
use super::adaptive::{
    classify_rows, solve_more, stop_rule, AttemptRecord, AttemptStop, BudgetScope, CaseLimit,
    InvocationMeter, PrecisionState, Publication, RowClass, Shared, Solved, StageGuard, PRECISIONS,
};
use super::ledger::{LedgerRefusal, RetainedLedger};
use super::recover::{recover, Kind, QuantityId};
use super::wide::multi::{AttemptWork, SupportedWidth, WideContext};
use super::wide::{Wide, WideError};
use super::wide_sum::{ExactWideSum, SumWork};
use std::sync::Arc;

/// Why a combination is not selected.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CombinationReason {
    /// At the ceiling (`combination_unresolved`).
    CombinationUnresolved,
    /// No operand, a nonfinite factor, or an operand not selected.
    OperandUnresolved,
    /// The operands differ in stiffness identity or published layout.
    OperandsDiffer,
    LedgerUnavailable(LedgerRefusal),
    Budget(BudgetScope),
    ExactSumSpan,
    ExponentRange,
    Arithmetic(WideError),
}

/// One combination attempt: the formation at p (and its operand solves).
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CombinationAttempt {
    pub(crate) precision: u32,
    pub(crate) accepted: Option<bool>,
    pub(crate) work: AttemptWork,
    pub(crate) k4_work: SumWork,
    pub(crate) stop_rule_work: u64,
    /// Operand states solved on demand for this precision.
    pub(crate) operand_solves: Vec<AttemptRecord>,
}

/// The combination's evidence (D1 §5 item 1, "for combinations, per
/// combination").
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CombinationEvidence {
    pub(crate) factors: Vec<u64>,
    pub(crate) attempts: Vec<CombinationAttempt>,
    pub(crate) selected_precision: u32,
    pub(crate) verification_precision: u32,
    pub(crate) stop_rule: Vec<(u32, Kind, f64)>,
    pub(crate) floor_ratio_bits: u64,
    pub(crate) body_scales: Vec<(u32, Kind, u64)>,
    pub(crate) absolute_verified: Vec<(QuantityId, u64)>,
    pub(crate) unpublishable: Vec<QuantityId>,
    pub(crate) ledger_encoding: Vec<u8>,
    pub(crate) retained_state_encoding: Vec<u8>,
}

/// A selected combination.
#[derive(Debug, Clone)]
pub(crate) struct RetainedCombination {
    selected: u32,
    publication: Publication,
    evidence: CombinationEvidence,
}

#[allow(dead_code)] // F2a API (F2a reads the outcome's fields)
#[derive(Debug, Clone)]
pub(crate) enum CombinationOutcome {
    Selected(Box<RetainedCombination>),
    Unresolved {
        reason: CombinationReason,
        attempts: Vec<CombinationAttempt>,
    },
}

fn reason_of(stop: &AttemptStop) -> CombinationReason {
    match stop {
        AttemptStop::Budget(scope) => CombinationReason::Budget(*scope),
        AttemptStop::Span => CombinationReason::ExactSumSpan,
        AttemptStop::Exponent => CombinationReason::ExponentRange,
        AttemptStop::Arithmetic(e) => CombinationReason::Arithmetic(*e),
        _ => CombinationReason::OperandUnresolved,
    }
}

/// Σ cᵢ·uᵢ at every DOF (exact, rounded once) and the recovery at p.
#[allow(clippy::too_many_arguments)]
fn form_state<const L: usize, const R: usize>(
    shared: &Shared<L, R>,
    operands: &[&Solved<L>],
    factors: &[f64],
    first: &RetainedSolve,
    ledger: &RetainedLedger,
    guard: StageGuard,
) -> (Result<Solved<L>, AttemptStop>, AttemptWork, SumWork)
where
    Wide<L>: SupportedWidth,
    Wide<R>: SupportedWidth,
{
    let p = shared.p;
    let mut ctx = WideContext::<L>::new(p).expect("supported precision");
    let mut sum = ExactWideSum::new();
    let mut run = || -> Result<Solved<L>, AttemptStop> {
        let lifted: Vec<Wide<L>> = factors
            .iter()
            .map(|&c| Wide::<L>::from_f64(c))
            .collect::<Result<_, _>>()?;
        let n = operands[0].u.len();
        let mut u = Vec::with_capacity(n);
        for j in 0..n {
            sum.clear();
            for (c, operand) in lifted.iter().zip(operands) {
                sum.add_product(&mut ctx, c, &operand.u[j], false)?;
            }
            u.push(sum.round(&mut ctx)?);
        }
        guard.check(&ctx, &sum)?;
        let recovered = recover(
            &mut ctx,
            &mut sum,
            &guard,
            first.source(),
            &first.prep.layout,
            &first.group.structure,
            &shared.k,
            &shared.members,
            &shared.directional,
            ledger,
            &u,
        )?;
        Ok(Solved {
            p,
            u,
            recovered,
            corrections: 0,
            residual_worst: 0.0,
        })
    };
    let result = run();
    let mut work = AttemptWork::default();
    work.record(&ctx);
    (result, work, sum.work())
}

/// The combined state at p (operands' states solved on demand).
fn combined_at(
    p: u32,
    operands: &[(f64, &RetainedSolve)],
    ledger: &RetainedLedger,
    room: &mut u64,
    meter: &mut InvocationMeter,
    attempt: &mut CombinationAttempt,
) -> Result<PrecisionState, AttemptStop> {
    let mut states: Vec<PrecisionState> = Vec::with_capacity(operands.len());
    for (_, operand) in operands {
        match operand.state(p) {
            Some(state) => states.push(state.clone()),
            None => {
                let (result, record) = solve_more(operand, p, *room, meter);
                *room = room.saturating_sub(record.case_charge());
                attempt.operand_solves.push(record);
                states.push(result?);
            }
        }
    }
    let factors: Vec<f64> = operands.iter().map(|o| o.0).collect();
    let first = operands[0].1;
    let guard = StageGuard::rooms(*room, meter);
    macro_rules! form {
        ($variant:ident) => {{
            let PrecisionState::$variant(shared, _) = &states[0] else {
                unreachable!()
            };
            let solved: Vec<&Solved<_>> = states
                .iter()
                .map(|s| match s {
                    PrecisionState::$variant(_, solved) => solved.as_ref(),
                    _ => unreachable!("one precision"),
                })
                .collect();
            let (result, work, sum_work) =
                form_state(shared, &solved, &factors, first, ledger, guard);
            let total = work.limb_multiply_equivalents() + sum_work.limb_multiply_equivalents();
            *room = room.saturating_sub(total);
            meter.charge_work(total);
            attempt.work.merge(&work);
            attempt.k4_work.merge(&sum_work);
            result.map(|s| PrecisionState::$variant(shared.clone(), Arc::new(s)))
        }};
    }
    match p {
        128 => form!(P128),
        256 => form!(P256),
        512 => form!(P512),
        _ => form!(P1024),
    }
}

impl RetainedCombination {
    /// D1's `form(&[(factor, &RetainedSolve)], p)`: the combined state at p.
    #[allow(dead_code)] // F2a API
    pub(crate) fn form(
        operands: &[(f64, &RetainedSolve)],
        p: u32,
        meter: &mut InvocationMeter,
    ) -> Result<PrecisionState, CombinationReason> {
        let ledger = Self::ledger(operands)?;
        let mut room = u64::MAX;
        let mut attempt = CombinationAttempt {
            precision: p,
            accepted: None,
            work: AttemptWork::default(),
            k4_work: SumWork::default(),
            stop_rule_work: 0,
            operand_solves: Vec::new(),
        };
        combined_at(p, operands, &ledger, &mut room, meter, &mut attempt).map_err(|s| reason_of(&s))
    }

    fn ledger(operands: &[(f64, &RetainedSolve)]) -> Result<RetainedLedger, CombinationReason> {
        if operands.is_empty() || operands.iter().any(|o| !o.0.is_finite()) {
            return Err(CombinationReason::OperandUnresolved);
        }
        let first = operands[0].1;
        let identity = first.source().stiffness_encoding();
        for (_, operand) in operands {
            if operand.source().stiffness_encoding() != identity
                || operand.prep.layout != first.prep.layout
            {
                return Err(CombinationReason::OperandsDiffer);
            }
        }
        let parts: Vec<(f64, &super::source::PrimitiveSource)> =
            operands.iter().map(|(c, s)| (*c, s.source())).collect();
        RetainedLedger::combined(&parts).map_err(CombinationReason::LedgerUnavailable)
    }

    /// The combination's own schedule (module documentation).
    #[allow(dead_code)] // F2a API
    pub(crate) fn solve(
        operands: &[(f64, &RetainedSolve)],
        case_limit: CaseLimit,
        meter: &mut InvocationMeter,
    ) -> CombinationOutcome {
        let ledger = match Self::ledger(operands) {
            Ok(l) => l,
            Err(reason) => {
                return CombinationOutcome::Unresolved {
                    reason,
                    attempts: Vec::new(),
                }
            }
        };
        let first = operands[0].1;
        let start = operands
            .iter()
            .map(|o| o.1.selected_precision())
            .max()
            .unwrap_or(128);
        let mut room = case_limit.get();
        let mut attempts: Vec<CombinationAttempt> = Vec::new();
        let mut states: Vec<PrecisionState> = Vec::new();
        let mut c = PRECISIONS.iter().position(|&p| p == start).unwrap_or(0);
        while c < 3 {
            let p = PRECISIONS[c];
            let q = PRECISIONS[c + 1];
            for precision in [p, q] {
                if states.iter().any(|s| s.precision() == precision) {
                    continue;
                }
                let mut attempt = CombinationAttempt {
                    precision,
                    accepted: None,
                    work: AttemptWork::default(),
                    k4_work: SumWork::default(),
                    stop_rule_work: 0,
                    operand_solves: Vec::new(),
                };
                let result =
                    combined_at(precision, operands, &ledger, &mut room, meter, &mut attempt);
                attempts.push(attempt);
                match result {
                    Ok(state) => states.push(state),
                    Err(stop) if stop.escalates() => {
                        // An operand could not be solved at this precision.
                        return CombinationOutcome::Unresolved {
                            reason: CombinationReason::OperandUnresolved,
                            attempts,
                        };
                    }
                    Err(stop) => {
                        return CombinationOutcome::Unresolved {
                            reason: reason_of(&stop),
                            attempts,
                        }
                    }
                }
            }
            let candidate = states.iter().find(|s| s.precision() == p).unwrap().clone();
            let verification = states.iter().find(|s| s.precision() == q).unwrap().clone();
            let decision = stop_state(
                first,
                &candidate,
                &verification,
                StageGuard::rooms(room, meter),
            );
            room = room.saturating_sub(decision.total);
            meter.charge_work(decision.total);
            if let Some(a) = attempts.iter_mut().rev().find(|a| a.precision == p) {
                a.stop_rule_work += decision.total;
                a.work.merge(&decision.work);
                a.k4_work.merge(&decision.sum_work);
                a.accepted = decision.result.as_ref().ok().copied();
            }
            match decision.result {
                Err(stop) => {
                    return CombinationOutcome::Unresolved {
                        reason: reason_of(&stop),
                        attempts,
                    }
                }
                Ok(true) => {
                    let values = candidate.published();
                    let publication =
                        classify_rows(&first.prep.layout, &values, &first.prep.extents);
                    let evidence = CombinationEvidence {
                        factors: operands.iter().map(|o| o.0.to_bits()).collect(),
                        attempts,
                        selected_precision: p,
                        verification_precision: q,
                        stop_rule: decision.summary,
                        floor_ratio_bits: super::adaptive::FLOOR_RATIO_BITS,
                        body_scales: publication.body_scales.clone(),
                        absolute_verified: publication
                            .rows
                            .iter()
                            .filter_map(|r| match r.class {
                                RowClass::AbsoluteVerified { bound_bits } => {
                                    Some((r.id, bound_bits))
                                }
                                _ => None,
                            })
                            .collect(),
                        unpublishable: publication
                            .rows
                            .iter()
                            .filter(|r| r.class == RowClass::Unpublishable)
                            .map(|r| r.id)
                            .collect(),
                        ledger_encoding: ledger.encoding(),
                        retained_state_encoding: candidate.encoding(),
                    };
                    return CombinationOutcome::Selected(Box::new(RetainedCombination {
                        selected: p,
                        publication,
                        evidence,
                    }));
                }
                Ok(false) => c += 1,
            }
        }
        CombinationOutcome::Unresolved {
            reason: CombinationReason::CombinationUnresolved,
            attempts,
        }
    }

    #[allow(dead_code)] // F2a API
    pub(crate) fn publish(&self) -> &Publication {
        &self.publication
    }
    #[allow(dead_code)] // F2a API
    pub(crate) fn evidence(&self) -> &CombinationEvidence {
        &self.evidence
    }
    #[allow(dead_code)] // F2a API
    pub(crate) fn selected_precision(&self) -> u32 {
        self.selected
    }
}

fn stop_state(
    first: &RetainedSolve,
    candidate: &PrecisionState,
    verification: &PrecisionState,
    guard: StageGuard,
) -> super::adaptive::StopDecision {
    let (layout, extents) = (&first.prep.layout, &first.prep.extents);
    match (candidate, verification) {
        (PrecisionState::P128(_, a), PrecisionState::P256(_, b)) => stop_rule(
            layout,
            extents,
            &a.recovered.values,
            &b.recovered.values,
            256,
            guard,
        ),
        (PrecisionState::P256(_, a), PrecisionState::P512(_, b)) => stop_rule(
            layout,
            extents,
            &a.recovered.values,
            &b.recovered.values,
            512,
            guard,
        ),
        (PrecisionState::P512(_, a), PrecisionState::P1024(_, b)) => stop_rule(
            layout,
            extents,
            &a.recovered.values,
            &b.recovered.values,
            1024,
            guard,
        ),
        _ => unreachable!("p with 2p"),
    }
}

#[cfg(test)]
#[path = "../../../tests/retained_k4/combine_tests.rs"]
mod tests;
