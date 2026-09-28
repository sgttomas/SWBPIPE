//! K4 tests of `retained/combine.rs` (the brief's I): a combination is its own
//! solve (ROOT's ruling on I12's F-1).
//!
//! S\*-dependent assertions (to be updated mechanically if D1 revision 5a.3
//! changes S\*): `SD-I1` the stop rule rejects 2·SKEW6 − SKEW6 at 128;
//! `SD-I2` the cancelled combination SKEW6 − SKEW6 is selected at 128 with
//! every row an exact +0.0 classified `AbsoluteVerified` with bound 0.
use super::super::adaptive::{
    solve_case, solve_cases, AttemptOutcome, AttemptReason, AttemptRole, AttemptStop, BudgetScope,
    CaseLimit, CaseOutcome, InvocationMeter, PrecisionState, RetainedSolve, RowClass,
    UnresolvedReason,
};
use super::super::recover::publish_value;
use super::super::source::{PrimitiveSource, SourceParts};
use super::super::wide::multi::Binary64Outcome;
use super::super::wide_sum::ExactWideSum;
use super::*;

#[allow(dead_code)]
#[path = "models.rs"]
mod models;
#[allow(dead_code)]
#[path = "support.rs"]
mod support;

fn unlimited() -> CaseLimit {
    CaseLimit::new(u64::MAX)
}

fn selected_source(source: PrimitiveSource) -> Box<RetainedSolve> {
    let mut meter = InvocationMeter::new(u64::MAX);
    match solve_case(source, unlimited(), &mut meter) {
        CaseOutcome::Selected(s) => s,
        other => panic!("{other:?}"),
    }
}

fn selected(name: &str) -> Box<RetainedSolve> {
    selected_source(models::model(name).source())
}

fn combine(operands: &[(f64, &RetainedSolve)]) -> CombinationOutcome {
    let mut meter = InvocationMeter::new(u64::MAX);
    RetainedCombination::solve(operands, unlimited(), &mut meter)
}

fn combined(operands: &[(f64, &RetainedSolve)]) -> Box<RetainedSolve> {
    match combine(operands) {
        CombinationOutcome::Selected(s) => s,
        other => panic!("{other:?}"),
    }
}

fn bits(solve: &RetainedSolve) -> Vec<(String, u64)> {
    solve
        .publish()
        .rows
        .iter()
        .map(|r| {
            let b = match r.value {
                Binary64Outcome::Normal(x) => x.to_bits(),
                ref other => panic!("{other:?}"),
            };
            (models::key(&r.id), b)
        })
        .collect()
}

/// The combination equals the solve of its net case bit for bit: published
/// rows, selected precision, the retained state and the ledger.
fn assert_equals_net(combination: &RetainedSolve, net: &RetainedSolve) {
    assert_eq!(bits(combination), bits(net));
    assert_eq!(combination.selected_precision(), net.selected_precision());
    let (c, n) = (combination.evidence(), net.evidence());
    assert_eq!(c.retained_state_encoding, n.retained_state_encoding);
    assert_eq!(c.ledger_encoding, n.ledger_encoding);
    assert!(c.source_encoding.starts_with(b"K4CMB\x01"));
    assert_eq!(
        c.attempts
            .iter()
            .map(|a| (a.precision, a.outcome.clone()))
            .collect::<Vec<_>>(),
        n.attempts
            .iter()
            .map(|a| (a.precision, a.outcome.clone()))
            .collect::<Vec<_>>()
    );
}

fn combo(name: &str) -> models::Combo {
    models::combos()
        .into_iter()
        .find(|c| c.name == name)
        .unwrap()
}

/// The generator's combinations, their operands solved as cases, combined, and
/// compared with the net case and the exact truth.
fn check_combo(name: &str, net: &str) -> (Box<RetainedSolve>, Vec<Box<RetainedSolve>>) {
    let c = combo(name);
    let operands: Vec<(f64, Box<RetainedSolve>)> =
        c.operands.iter().map(|(f, n)| (*f, selected(n))).collect();
    let refs: Vec<(f64, &RetainedSolve)> = operands.iter().map(|(f, s)| (*f, s.as_ref())).collect();
    let combination = combined(&refs);
    let net_solve = selected(net);
    assert_equals_net(&combination, &net_solve);
    let source = operands[0].1.source().clone();
    let (worst, at, compared) = models::compare(&source, &combination.publish().rows, &c.expect);
    assert!(
        compared >= 30 && worst <= 1.0,
        "{name}: {worst} at {at} ({compared})"
    );
    (combination, operands.into_iter().map(|o| o.1).collect())
}

#[test]
fn b1_c_the_cancelled_1e80_terms_leave_the_1e_minus_8_load_exactly() {
    let (combination, operands) = check_combo("B1-C", "B1-C-NET");
    assert_eq!(combination.selected_precision(), 128);
    assert_eq!(operands.len(), 3);
    // D14's discrimination: the same terms folded at p = 128 lose B.
    let mut c = support::ctx::<4>(128);
    let (a, b) = (support::lift::<4>(1e80), support::lift::<4>(1e-8));
    let partial = c.add(&a, &b).unwrap();
    let folded = c.add(&partial, &a.neg()).unwrap();
    assert!(folded.is_zero());
}

/// Σcᵢ·(published quantity at 128), formed exactly from the operands' 128
/// states and rounded once at 128: the withdrawn formation.
fn sum_of_states_at_128(operands: &[(f64, &RetainedSolve)]) -> Vec<f64> {
    let mut c = support::ctx::<4>(128);
    let values: Vec<&Vec<_>> = operands
        .iter()
        .map(|(_, s)| match s.state(128) {
            Some(PrecisionState::P128(solved)) => &solved.recovered.values,
            other => panic!("{other:?}"),
        })
        .collect();
    (0..values[0].len())
        .map(|k| {
            let mut sum = ExactWideSum::new();
            for ((factor, _), v) in operands.iter().zip(&values) {
                sum.add_product(&mut c, &support::lift::<4>(*factor), &v[k], false)
                    .unwrap();
            }
            publish_value(&sum.round(&mut c).unwrap()).value().unwrap()
        })
        .collect()
}

#[test]
fn b1_e_is_caught_the_combination_reproduces_the_truth_that_a_sum_of_states_loses() {
    let (combination, operands) = check_combo("B1-E", "B1-E-NET");
    let refs: Vec<(f64, &RetainedSolve)> = combo("B1-E")
        .operands
        .iter()
        .map(|o| o.0)
        .zip(operands.iter().map(|s| s.as_ref()))
        .collect();
    // The withdrawn Σcᵢuᵢ at 128 loses ε/P = 1e-45 below 128 bits: some row
    // differs from the truth (the combination's value) by more than 1e-9.
    let withdrawn = sum_of_states_at_128(&refs);
    let truth: Vec<f64> = combination
        .publish()
        .rows
        .iter()
        .map(|r| r.value.value().unwrap())
        .collect();
    let scale = truth.iter().fold(0.0f64, |m, v| m.max(v.abs()));
    assert!(scale > 0.0);
    let worst = withdrawn
        .iter()
        .zip(&truth)
        .map(|(w, t)| (w - t).abs() / scale)
        .fold(0.0f64, f64::max);
    assert!(worst > 1e-9, "{worst}");
}

#[test]
fn ceiling_operands_2_to_the_minus_1060_apart_give_the_net_cases_truth() {
    // F-1's control: CEIL-A − CEIL-B, whose loads differ by 2^-60 on 2^1000.
    let (combination, operands) = check_combo("CEILING", "CEIL-NET");
    let rows = &combination.publish().rows;
    let spring = rows
        .iter()
        .find(|r| models::key(&r.id) == "spr.3.3")
        .unwrap();
    assert_eq!(spring.value.value(), Some(-(2f64.powi(-60))));
    assert!(rows.iter().filter(|r| r.value.value() != Some(0.0)).count() > 10);
    // The withdrawn formation publishes zeros.
    let refs = [(1.0, operands[0].as_ref()), (-1.0, operands[1].as_ref())];
    assert!(sum_of_states_at_128(&refs).iter().all(|&v| v == 0.0));
}

#[test]
fn a_combination_runs_its_own_stop_rule() {
    // D15's control. SD-I1: 2·SKEW6 − SKEW6 (net: SKEW6) is rejected at 128 by
    // its own stop rule and accepted at 256 against 512.
    let a = selected("SKEW6-K1E-12");
    let combination = combined(&[(2.0, a.as_ref()), (-1.0, a.as_ref())]);
    let attempts = &combination.evidence().attempts;
    assert!(matches!(
        attempts[0].outcome,
        AttemptOutcome::Rejected(AttemptReason::StopRule { .. })
    ));
    assert_eq!(attempts[0].precision, 128);
    assert_eq!(combination.selected_precision(), 256);
    assert_eq!(bits(&combination), bits(&a));
}

#[test]
fn a_combination_escalates_independently_of_its_operands() {
    // SD-I2: the operands are selected at 256; their difference cancels
    // exactly and is selected at 128, every row an exact +0.0.
    let a = selected("SKEW6-K1E-12");
    assert_eq!(a.selected_precision(), 256);
    let combination = combined(&[(1.0, a.as_ref()), (-1.0, a.as_ref())]);
    assert_eq!(combination.selected_precision(), 128);
    for row in &combination.publish().rows {
        assert_eq!(row.value, Binary64Outcome::Normal(0.0));
        assert_eq!(row.value.value().unwrap().to_bits(), 0);
        if row.class != RowClass::InputDerived {
            assert_eq!(row.class, RowClass::AbsoluteVerified { bound_bits: 0 });
        }
    }
}

#[test]
fn prescribed_values_combine_exactly_and_round_once() {
    // PRESCRIBED + PRESCRIBED equals the case with doubled prescribed values;
    // 0.5·PRESCRIBED equals the case with halved ones (both exact in binary64).
    let base = models::model("PRESCRIBED");
    let scaled = |factor: f64| {
        let mut parts: SourceParts = base.parts.clone();
        for c in &mut parts.constraints {
            c.value *= factor;
        }
        selected_source(PrimitiveSource::new(parts).unwrap())
    };
    let a = selected("PRESCRIBED");
    assert_equals_net(
        &combined(&[(1.0, a.as_ref()), (1.0, a.as_ref())]),
        &scaled(2.0),
    );
    assert_equals_net(&combined(&[(0.5, a.as_ref())]), &scaled(0.5));
}

#[test]
fn a_combination_reuses_its_operands_cached_factor_stops_and_builds() {
    // The operands escalate past 128 on the condition screen; the combination
    // meets the cached stop at 128 and the cached build at 256, and charges
    // the invocation only its own stages.
    let sources = [
        models::model("SKEW-K1E-28-AXIAL").source(),
        models::model("SKEW-K1E-28").source(),
    ];
    let mut meter = InvocationMeter::new(u64::MAX);
    let outcomes = solve_cases(&sources, unlimited(), &mut meter);
    let operands: Vec<&RetainedSolve> = outcomes
        .iter()
        .map(|o| match o {
            CaseOutcome::Selected(s) => s.as_ref(),
            other => panic!("{other:?}"),
        })
        .collect();
    assert!(operands.iter().all(|s| s.selected_precision() == 256));
    let before = meter.charged();
    let combination = match RetainedCombination::solve(
        &[(1.0, operands[0]), (1.0, operands[1])],
        unlimited(),
        &mut meter,
    ) {
        CombinationOutcome::Selected(s) => s,
        other => panic!("{other:?}"),
    };
    let attempts = &combination.evidence().attempts;
    assert_eq!(
        attempts[0].outcome,
        AttemptOutcome::Failed(AttemptReason::Stop(AttemptStop::Condition))
    );
    assert!(attempts.iter().all(|a| !a.shared_built_here));
    assert!(attempts.iter().all(|a| a.shared_work > 0));
    let own: u64 = attempts
        .iter()
        .map(|a| a.work.limb_multiply_equivalents() + a.k4_work.limb_multiply_equivalents())
        .sum();
    assert_eq!(meter.charged() - before, own);
    assert_eq!(combination.selected_precision(), 256);
    assert_eq!(attempts.last().unwrap().role, AttemptRole::Verification);
}

#[test]
fn invalid_combinations_are_withheld_with_their_reason() {
    let a = selected("N05");
    let b = selected("N06");
    let reason = |o: CombinationOutcome| match o {
        CombinationOutcome::Unresolved { reason, attempts } => {
            assert!(attempts.is_empty());
            reason
        }
        other => panic!("{other:?}"),
    };
    assert_eq!(reason(combine(&[])), CombinationReason::NoOperands);
    assert_eq!(
        reason(combine(&[(f64::NAN, a.as_ref())])),
        CombinationReason::NoOperands
    );
    let n09 = selected("N09-B");
    assert_eq!(
        reason(combine(&[(1.0, a.as_ref()), (1.0, n09.as_ref())])),
        CombinationReason::OperandsDiffer
    );
    let c = combined(&[(1.0, a.as_ref()), (1.0, a.as_ref())]);
    assert_eq!(
        reason(combine(&[(1.0, c.as_ref()), (1.0, a.as_ref())])),
        CombinationReason::NestedCombination
    );
    // N05 and N06 differ in their springs (N06's root spring): no shared K.
    assert_eq!(
        reason(combine(&[(1.0, a.as_ref()), (1.0, b.as_ref())])),
        CombinationReason::OperandsDiffer
    );
}

#[test]
fn a_combination_out_of_budget_is_withheld_and_its_operands_keep_their_standing() {
    let a = selected("SKEW6-K1E-12");
    let before = (bits(&a), a.evidence().clone());
    let mut meter = InvocationMeter::new(u64::MAX);
    match RetainedCombination::solve(
        &[(1.0, a.as_ref()), (2.0, a.as_ref())],
        CaseLimit::new(1),
        &mut meter,
    ) {
        CombinationOutcome::Unresolved {
            reason: CombinationReason::Unresolved(UnresolvedReason::Budget(BudgetScope::Case)),
            attempts,
        } => assert!(!attempts.is_empty()),
        other => panic!("{other:?}"),
    }
    assert_eq!((bits(&a), a.evidence().clone()), before);
}
