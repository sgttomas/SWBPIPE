//! Actual execution witnesses: no injected BuildRef, slot origin, or cache history.
use super::super::adaptive::{solve_cases, BudgetScope, CaseOutcome, UnresolvedReason};
use super::super::combine::{CombinationOutcome, RetainedCombination};
use super::*;

#[allow(dead_code)]
#[path = "models.rs"]
mod models;

fn limit() -> CaseLimit {
    CaseLimit::new(u64::MAX)
}
fn context(batches: &[usize], combinations: &[usize]) -> RecordedInvocation {
    RecordedInvocation::new(
        u64::MAX,
        OriginCapacity::for_calls(batches, combinations).unwrap(),
    )
    .unwrap()
}
fn selected(case: &RecordedCase) -> &RetainedSolve {
    match &case.outcome {
        ExecutionOutcome::Selected(s) => s,
        other => panic!("{other:?}"),
    }
}
fn combo_case(out: RecordedKernelCombination) -> RecordedCase {
    match out {
        RecordedKernelCombination::WithRun { case, .. } => case,
        other => panic!("{other:?}"),
    }
}
fn attempts(out: &ExecutionOutcome) -> &[adaptive::AttemptRecord] {
    match out {
        ExecutionOutcome::Selected(s) => &s.evidence().attempts,
        ExecutionOutcome::Refused { attempts, .. }
        | ExecutionOutcome::Unresolved { attempts, .. } => attempts,
    }
}
fn verify_conservation(ctx: &RecordedInvocation, cases: &[RecordedCase]) {
    for case in cases {
        let run = &ctx.runs()[case.run];
        let mut own = 0u128;
        let mut charged = 0u128;
        let mut increment = 0u128;
        for (i, record) in attempts(&case.outcome).iter().enumerate() {
            own += u128::from(record.checked_own_work().exact().unwrap());
            charged += u128::from(record.checked_case_charge().exact().unwrap());
            increment += u128::from(record.checked_invocation_increment().exact().unwrap());
            let links = run.records[i];
            let shared = &ctx.builds()[links.shared.unwrap()];
            assert_eq!(shared.work, record.checked_shared_work());
            assert_eq!(shared.slot, OriginSlot::solve(record.precision));
            assert_eq!(shared.run == run.id, record.shared_built_here);
            let mut stages = shared.stages.clone();
            if let Some(id) = links.verification_shared {
                let verification = &ctx.builds()[id];
                assert_eq!(verification.work, record.checked_verification_shared_work());
                assert_eq!(
                    verification.run == run.id,
                    record.verification_shared_built_here
                );
                stages.add(&verification.stages).unwrap();
            }
            assert_eq!(format!("{stages:?}"), format!("{:?}", record.shared_stages));
        }
        assert!(own <= charged);
        assert_eq!(charged, u128::from(run.work.case().exact().unwrap()));
        assert_eq!(
            increment,
            u128::from(run.work.invocation_increment().exact().unwrap())
        );
        assert_eq!(
            u128::from(run.work.invocation_before().exact().unwrap()) + increment,
            u128::from(run.work.invocation_after().exact().unwrap())
        );
    }
}

#[test]
fn origin_capacity_is_finite_checked_and_has_no_implicit_growth() {
    assert!(matches!(
        OriginCapacity::for_calls(&[usize::MAX], &[]),
        Err(OriginError::CountRange(_))
    ));
    assert!(matches!(
        OriginCapacity::for_calls(&[], &[usize::MAX, 1]),
        Err(OriginError::CountRange(_))
    ));
    let mut ctx = context(&[], &[]);
    assert!(matches!(
        ctx.solve_cases(&[], limit()),
        Err(OriginError::Capacity)
    ));
    assert!(ctx.calls().is_empty());
    assert_eq!(ctx.meter().charged(), 0);
}

#[test]
fn origins_follow_an_independent_ordered_build_oracle_and_preserve_legacy_debug() {
    let source = models::model("N05").source();
    let sources = [source.clone(), source];
    let mut ctx = context(&[2], &[]);
    let recorded = ctx.solve_cases(&sources, limit()).unwrap();
    // The small 128/256 case has exactly S128, S256 and V256 construction.
    assert_eq!(
        ctx.builds()
            .iter()
            .map(|b| (b.id, b.run, b.physical_record, b.slot))
            .collect::<Vec<_>>(),
        vec![
            (0, 0, 0, OriginSlot::S128),
            (1, 0, 1, OriginSlot::S256),
            (2, 0, 1, OriginSlot::V256)
        ]
    );
    assert_eq!(ctx.runs()[0].cache_before, [None; 7]);
    assert_eq!(
        ctx.runs()[0].cache_after,
        [Some(0), Some(1), None, None, Some(2), None, None]
    );
    assert_eq!(ctx.runs()[1].cache_before, ctx.runs()[0].cache_after);
    assert_eq!(ctx.runs()[0].records, ctx.runs()[1].records);
    assert_eq!(ctx.calls()[0].runs, vec![0, 1]);
    assert_eq!(ctx.sources()[0].identity, ctx.sources()[1].identity);
    assert_ne!(ctx.sources()[0].owner, ctx.sources()[1].owner);
    let mut meter = InvocationMeter::new(u64::MAX);
    let legacy = solve_cases(&sources, limit(), &mut meter);
    for (a, b) in recorded.iter().zip(legacy) {
        assert_eq!(
            format!("{:?}", a.outcome.clone().into_legacy()),
            format!("{b:?}")
        );
    }
    assert_eq!(ctx.meter(), &meter);
    verify_conservation(&ctx, &recorded);
}

#[test]
fn distinct_calls_create_distinct_groups_and_keep_one_meter_chain() {
    let source = models::model("N05").source();
    let mut ctx = context(&[1, 1], &[]);
    let first = ctx
        .solve_cases(std::slice::from_ref(&source), limit())
        .unwrap();
    let second = ctx.solve_cases(&[source], limit()).unwrap();
    assert_eq!(ctx.groups().len(), 2);
    assert_eq!(ctx.groups()[0].call, 0);
    assert_eq!(ctx.groups()[1].call, 1);
    assert_eq!(ctx.builds().len(), 6);
    assert_eq!(
        ctx.calls()[0].invocation_after,
        ctx.calls()[1].invocation_before
    );
    assert_eq!(ctx.runs()[1].cache_before, [None; 7]);
    verify_conservation(&ctx, &first);
    verify_conservation(&ctx, &second);
}

#[test]
fn actual_nonbudget_failure_is_resident_and_budget_failure_is_retried() {
    let source = models::model("SKEW-K1E-300").source();
    let mut ctx = context(&[2], &[]);
    let cases = ctx.solve_cases(&[source.clone(), source], limit()).unwrap();
    assert_eq!(ctx.builds().len(), 3);
    assert!(ctx
        .builds()
        .iter()
        .all(|b| b.state == BuildState::NonbudgetFailure));
    assert_eq!(ctx.runs()[0].records, ctx.runs()[1].records);
    assert_eq!(ctx.runs()[1].work.invocation_increment().exact(), Ok(0));
    assert_eq!(ctx.runs()[0].work.case(), ctx.runs()[1].work.case());
    assert!(matches!(
        cases[1].outcome,
        ExecutionOutcome::Unresolved {
            reason: UnresolvedReason::Ceiling,
            ..
        }
    ));
    verify_conservation(&ctx, &cases);

    let source = models::model("N05").source();
    let mut ctx = context(&[2], &[]);
    let cases = ctx
        .solve_cases(&[source.clone(), source], CaseLimit::new(0))
        .unwrap();
    assert_eq!(ctx.builds().len(), 2);
    assert!(ctx
        .builds()
        .iter()
        .all(|b| b.state == BuildState::BudgetFailure));
    assert!(ctx.runs().iter().all(|r| r.cache_after == [None; 7]));
    assert_eq!(ctx.runs()[0].records[0].shared, Some(0));
    assert_eq!(ctx.runs()[1].records[0].shared, Some(1));
    verify_conservation(&ctx, &cases);
}

#[test]
fn exhaustion_and_group_refusal_keep_their_actual_phase_without_builds() {
    let source = models::model("N05").source();
    let mut ctx =
        RecordedInvocation::new(0, OriginCapacity::for_calls(&[1], &[]).unwrap()).unwrap();
    let cases = ctx.solve_cases(&[source], limit()).unwrap();
    assert!(matches!(
        cases[0].outcome,
        ExecutionOutcome::Unresolved {
            reason: UnresolvedReason::Budget(BudgetScope::Invocation),
            ..
        }
    ));
    assert_eq!(ctx.sources().len(), 1);
    assert!(ctx.groups().is_empty() && ctx.builds().is_empty());
    assert_eq!(ctx.runs()[0].phase, RunPhase::InvocationEntry);
    assert_eq!(ctx.runs()[0].group, None);
    assert_eq!(ctx.runs()[0].work.invocation_increment().exact(), Ok(0));

    let mut model = models::model("N05");
    model.parts.constraints.clear();
    model.parts.supports.clear();
    let mut ctx = context(&[2], &[]);
    let source = model.source();
    let cases = ctx.solve_cases(&[source.clone(), source], limit()).unwrap();
    assert!(matches!(cases[0].outcome, ExecutionOutcome::Refused { .. }));
    assert_eq!(ctx.groups().len(), 1);
    assert!(ctx.builds().is_empty());
    assert!(ctx
        .runs()
        .iter()
        .all(|r| r.phase == RunPhase::GroupPreparation && r.group == Some(0)));
    assert!(matches!(
        ctx.groups()[0].preparation,
        GroupPreparation::Refused(_)
    ));
}

#[test]
fn frozen_selected_snapshots_and_authored_first_occupied_imports_are_actual() {
    let loaded = models::model("SKEW6-K1E-12");
    let mut zero = loaded.clone();
    zero.parts.loads.clear();
    let mut ctx = context(&[2, 1], &[2, 2]);
    let cases = ctx
        .solve_cases(&[zero.source(), loaded.source()], limit())
        .unwrap();
    let other_call = ctx.solve_cases(&[loaded.source()], limit()).unwrap();
    assert_eq!(selected(&cases[0]).selected_precision(), 128);
    assert_eq!(selected(&cases[1]).selected_precision(), 256);
    let early = ctx.runs()[0].cache_after;
    let later = ctx.runs()[1].cache_after;
    assert!(early[2].is_none() && later[2].is_some());
    let operands = [(1.0, selected(&cases[0])), (1.0, selected(&other_call[0]))];
    let combination = combo_case(ctx.solve_combination(&operands, limit()).unwrap());
    let run = &ctx.runs()[combination.run];
    let imports = &ctx.groups()[run.group.unwrap()].imports;
    for slot in OriginSlot::ALL {
        let i = slot.index();
        if let Some(import) = imports[i] {
            let expected_run = if early[i].is_some() { 0 } else { 2 };
            assert_eq!(import.selected_run, expected_run);
            assert_eq!(import.operand_index, usize::from(expected_run == 2));
            assert_eq!(Some(import.build), ctx.runs()[expected_run].cache_after[i]);
            assert_eq!(
                ctx.builds()[import.build].group,
                ctx.runs()[expected_run].group.unwrap()
            );
        }
    }
    assert_eq!(ctx.runs()[0].cache_after, early);
    let reverse = [(1.0, selected(&other_call[0])), (1.0, selected(&cases[0]))];
    let next = combo_case(ctx.solve_combination(&reverse, limit()).unwrap());
    assert_eq!(ctx.runs()[next.run].cache_before, ctx.runs()[2].cache_after);
    assert!(ctx.groups()[ctx.runs()[next.run].group.unwrap()]
        .imports
        .iter()
        .flatten()
        .all(|i| i.operand_index == 0 && i.selected_run == 2));
    verify_conservation(&ctx, &[combination, next]);
}

#[test]
fn pre_source_calls_consume_no_runs_and_foreign_or_legacy_solves_get_no_origin() {
    let source = models::model("N05").source();
    let mut ctx = context(&[1], &[0, 1, 1, 1, 1]);
    let cases = ctx
        .solve_cases(std::slice::from_ref(&source), limit())
        .unwrap();
    let before = ctx.meter().checked_charged();
    assert!(matches!(
        ctx.solve_combination(&[], limit()).unwrap(),
        RecordedKernelCombination::PreSourceRefusal {
            reason: CombinationReason::NoOperands,
            ..
        }
    ));
    assert!(matches!(
        ctx.solve_combination(&[(f64::NAN, selected(&cases[0]))], limit())
            .unwrap(),
        RecordedKernelCombination::PreSourceRefusal {
            reason: CombinationReason::NoOperands,
            ..
        }
    ));
    let mut foreign = context(&[1], &[]);
    let other = foreign
        .solve_cases(std::slice::from_ref(&source), limit())
        .unwrap();
    assert_eq!(
        selected(&cases[0]).prep.identity,
        selected(&other[0]).prep.identity
    );
    assert!(matches!(
        ctx.solve_combination(&[(1.0, selected(&other[0]))], limit())
            .unwrap(),
        RecordedKernelCombination::OriginRefusal {
            error: OriginError::MissingSelectedOrigin { operand: 0 },
            ..
        }
    ));
    let mut meter = InvocationMeter::new(u64::MAX);
    let legacy = solve_cases(&[source], limit(), &mut meter);
    let CaseOutcome::Selected(legacy) = &legacy[0] else {
        panic!()
    };
    assert!(matches!(
        ctx.solve_combination(&[(1.0, legacy)], limit()).unwrap(),
        RecordedKernelCombination::OriginRefusal { .. }
    ));
    assert_eq!(ctx.meter().checked_charged(), before);
    assert_eq!(ctx.runs().len(), 1);
    assert_eq!(ctx.sources().len(), 1);
    assert_eq!(ctx.groups().len(), 1);
    for call in &ctx.calls()[1..] {
        assert!(call.runs.is_empty() && call.sources.is_empty());
        assert_eq!(call.invocation_before, call.invocation_after);
    }
    let cloned = selected(&cases[0]).clone();
    let combination = combo_case(ctx.solve_combination(&[(1.0, &cloned)], limit()).unwrap());
    assert_eq!(combination.run, 1);
    assert_eq!(ctx.calls()[5].runs, vec![1]);
}

#[test]
fn recorded_combinations_preserve_native_bytes_work_and_validation_order() {
    let sources = [models::model("N05").source(), models::model("N06").source()];
    let mut ctx = context(&[2], &[2, 1, 1]);
    let cases = ctx.solve_cases(&sources, limit()).unwrap();
    assert!(matches!(
        ctx.solve_combination(
            &[(1.0, selected(&cases[0])), (1.0, selected(&cases[1]))],
            limit()
        )
        .unwrap(),
        RecordedKernelCombination::PreSourceRefusal {
            reason: CombinationReason::OperandsDiffer,
            ..
        }
    ));
    let recorded = combo_case(
        ctx.solve_combination(&[(1.0, selected(&cases[0]))], limit())
            .unwrap(),
    );
    let mut meter = InvocationMeter::new(u64::MAX);
    let legacy = solve_cases(&sources, limit(), &mut meter);
    let CaseOutcome::Selected(a) = &legacy[0] else {
        panic!()
    };
    let old = RetainedCombination::solve(&[(1.0, a)], limit(), &mut meter);
    let CombinationOutcome::Selected(old) = old else {
        panic!()
    };
    assert_eq!(format!("{:?}", selected(&recorded)), format!("{old:?}"));
    assert_eq!(ctx.meter(), &meter);
    assert_eq!(
        ctx.sources()[2].identity,
        selected(&recorded).evidence().source_encoding
    );
    assert!(ctx.sources()[2].identity.starts_with(b"K4CMB"));
    assert_eq!(
        ctx.sources()[2].combination_ledger.as_ref().unwrap(),
        &selected(&recorded).evidence().ledger_encoding
    );
    assert!(matches!(
        ctx.solve_combination(&[(1.0, selected(&recorded))], limit())
            .unwrap(),
        RecordedKernelCombination::PreSourceRefusal {
            reason: CombinationReason::NestedCombination,
            ..
        }
    ));
}

#[test]
fn combination_local_builds_and_unavailable_ledger_do_not_backfill_selected_operands() {
    let source = models::model("N05").source();
    let mut ctx = context(&[1], &[1, 1]);
    let cases = ctx.solve_cases(&[source], limit()).unwrap();
    let snapshot = ctx.runs()[0].cache_after;
    let operands = [(1.0, selected(&cases[0]))];
    // The maintained, thread-local final-state perturbation makes this own
    // schedule reject. It injects no origin, cache, factor, or work record.
    struct ClearSeed;
    impl Drop for ClearSeed {
        fn drop(&mut self) {
            adaptive::seed::set(Vec::new());
        }
    }
    let seeded = {
        let _clear = ClearSeed;
        adaptive::seed::set(vec![(6, 1.0)]);
        combo_case(ctx.solve_combination(&operands, limit()).unwrap())
    };
    let run = &ctx.runs()[seeded.run];
    let source = &ctx.sources()[run.source];
    assert!(source
        .combination_ledger
        .as_ref()
        .unwrap()
        .starts_with(b"K4LED"));
    assert!(!matches!(seeded.outcome, ExecutionOutcome::Selected(_)));
    assert!(ctx.builds().iter().any(|b| b.group == run.group.unwrap()));
    assert_eq!(ctx.runs()[0].cache_after, snapshot);
    let later = combo_case(ctx.solve_combination(&operands, limit()).unwrap());
    assert_eq!(ctx.runs()[later.run].cache_before, snapshot);
    assert!(ctx.groups()[ctx.runs()[later.run].group.unwrap()]
        .imports
        .iter()
        .flatten()
        .all(|import| import.selected_run == 0));
    verify_conservation(&ctx, &[seeded, later]);
}
