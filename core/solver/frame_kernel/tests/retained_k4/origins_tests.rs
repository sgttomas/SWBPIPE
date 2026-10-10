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

// ---------------------------------------------------------------- B2-K custody
// KD §1-§2 with RV115's SF-1 and N-6. N05 and N05-TRANSVERSE share one stiffness
// identity (different loads); SKEW6-K1E-12 selects at 256 and its unloaded copy
// at 128.
use super::super::adaptive::PreparedCaseSource;
fn b2k_ctx(batches: &[usize], combinations: &[usize], prepared: usize) -> RecordedInvocation {
    RecordedInvocation::new(u64::MAX, OriginCapacity::for_invocation(batches, combinations, prepared).unwrap()).unwrap()
}
fn b2k_capacities(ctx: &RecordedInvocation) -> [usize; 6] {
    let s = &ctx.store;
    [s.calls.capacity(), s.sources.capacity(), s.groups.capacity(), s.builds.capacity(), s.runs.capacity(), s.selected.capacity()]
}
/// No registry outgrows its reservation: lengths within, capacities unchanged.
fn b2k_within(ctx: &RecordedInvocation, reserved: [usize; 6]) {
    let s = &ctx.store;
    assert_eq!(b2k_capacities(ctx), reserved, "no reallocation");
    for (len, cap) in [s.calls.len(), s.sources.len(), s.groups.len(), s.builds.len(), s.runs.len(), s.selected.len()].into_iter().zip(reserved) {
        assert!(len <= cap);
    }
}
fn b2k_with_run(out: RecordedKernelCombination) -> RecordedCase {
    combo_case(out)
}
#[test]
fn b2k_k02_for_invocation_registration_and_the_run_capacity_check() {
    // N-6: for_calls(a, b) is for_invocation(a, b, 0), every field and CountRange name.
    let huge = usize::MAX;
    let shapes: [(&[usize], &[usize]); 9] = [(&[], &[]), (&[2], &[2, 1]), (&[1, 1], &[3]), (&[huge, 1], &[]),
        (&[], &[huge, 1]), (&[huge], &[0]), (&[huge / 2], &[]), (&[huge / 5], &[]), (&[huge / 8], &[huge / 8])];
    for (a, b) in shapes {
        assert_eq!(format!("{:?}", OriginCapacity::for_calls(a, b)), format!("{:?}", OriginCapacity::for_invocation(a, b, 0)));
    }
    let names: Vec<_> = shapes.iter().filter_map(|(a, b)| match OriginCapacity::for_calls(a, b) {
        Err(OriginError::CountRange(n)) => Some(n), _ => None }).collect();
    assert_eq!(names, ["case runs", "operands", "runs", "physical records", "builds"]);
    assert_eq!(OriginCapacity::for_invocation(&[1], &[], huge).unwrap_err(), OriginError::CountRange("case ordinal"));
    assert_eq!(OriginCapacity::for_invocation(&[], &[0], huge).unwrap_err(), OriginError::CountRange("record allocation"));
    // The same reservations with no prepared source; a prepared source adds a source slot only.
    assert_eq!(b2k_capacities(&context(&[2], &[2, 1])), b2k_capacities(&b2k_ctx(&[2], &[2, 1], 0)));
    assert_eq!(b2k_capacities(&b2k_ctx(&[2], &[2, 1], 3)), [3, 7, 4, 28, 4, 4]);

    // Registration: one SourceOrigin, owner Case(next ordinal); no Call, no Run, no meter change.
    let (a, b) = (models::model("N05").source(), models::model("N05-TRANSVERSE").source());
    assert_eq!(a.stiffness_encoding(), b.stiffness_encoding());
    let mut ctx = b2k_ctx(&[1], &[2], 1);
    let reserved = b2k_capacities(&ctx);
    let cases = ctx.solve_cases(std::slice::from_ref(&a), limit()).unwrap();
    let charged = ctx.meter().checked_charged();
    let prepared = PreparedCaseSource::new(b.clone()).unwrap();
    let id = ctx.register_prepared_source(&prepared).unwrap();
    assert_eq!(id, 1);
    let origin = &ctx.sources()[id];
    assert_eq!((origin.owner, origin.identity.clone(), origin.stiffness.clone(), origin.combination_ledger.clone()),
        (NativeOwner::Case(1), b.encoding(), b.stiffness_encoding(), None));
    assert_eq!((ctx.calls().len(), ctx.runs().len(), ctx.meter().checked_charged()), (1, 1, charged));
    assert_eq!(ctx.register_prepared_source(&prepared).unwrap_err(), OriginError::Capacity);
    assert_eq!(ctx.sources().len(), 2);
    let mixed = b2k_with_run(ctx.solve_combination_sources(&[(1.0, RecordedOperand::Selected(selected(&cases[0]))),
        (1.0, RecordedOperand::Prepared { source: id, prepared: &prepared })], limit()).unwrap());
    assert!(matches!(mixed.outcome, ExecutionOutcome::Selected(_)));
    assert_eq!(ctx.calls()[1].requested_operands.iter().map(|o| o.source).collect::<Vec<_>>(), [Some(0), Some(1)]);
    b2k_within(&ctx, reserved);

    // SF-1 (i): a batch that would spend the prepared ordinal is refused, with
    // nothing recorded; the declared order then fits exactly.
    let mut ctx = b2k_ctx(&[2], &[2], 1);
    let reserved = b2k_capacities(&ctx);
    assert_eq!(ctx.solve_cases(&[a.clone(), a.clone(), a.clone()], limit()).err(), Some(OriginError::Capacity));
    assert!(ctx.calls().is_empty() && ctx.sources().is_empty() && ctx.runs().is_empty());
    assert_eq!(ctx.meter().charged(), 0);
    let cases = ctx.solve_cases(&[a.clone(), a.clone()], limit()).unwrap();
    let id = ctx.register_prepared_source(&prepared).unwrap();
    b2k_with_run(ctx.solve_combination_sources(&[(1.0, RecordedOperand::Selected(selected(&cases[0]))),
        (1.0, RecordedOperand::Prepared { source: id, prepared: &prepared })], limit()).unwrap());
    b2k_within(&ctx, reserved);
    // SF-1 (ii): registrations that take the batch's ordinals make the batch refuse.
    let mut ctx = b2k_ctx(&[2], &[2], 1);
    let reserved = b2k_capacities(&ctx);
    ctx.register_prepared_source(&prepared).unwrap();
    ctx.register_prepared_source(&prepared).unwrap();
    let sources_before = ctx.sources().len();
    assert_eq!(ctx.solve_cases(&[a.clone(), a.clone()], limit()).err(), Some(OriginError::Capacity));
    assert!(ctx.calls().is_empty() && ctx.runs().is_empty() && ctx.sources().len() == sources_before);
    b2k_within(&ctx, reserved);
    // Dormant under for_calls: batches and combinations interleave freely.
    let mut ctx = context(&[1, 1], &[1, 1]);
    let reserved = b2k_capacities(&ctx);
    let first = ctx.solve_cases(std::slice::from_ref(&a), limit()).unwrap();
    b2k_with_run(ctx.solve_combination(&[(1.0, selected(&first[0]))], limit()).unwrap());
    let second = ctx.solve_cases(std::slice::from_ref(&b), limit()).unwrap();
    b2k_with_run(ctx.solve_combination(&[(1.0, selected(&second[0]))], limit()).unwrap());
    b2k_within(&ctx, reserved);
    assert_eq!(ctx.runs().len(), 4);
}
#[test]
fn b2k_k03_an_unavailable_operand_is_rebuilt_identity_checked_and_custody_refuses_otherwise() {
    let loaded = models::model("SKEW6-K1E-12");
    let mut zero = loaded.clone();
    zero.parts.loads.clear();
    let sources = [zero.source(), loaded.source()];
    let mut probe = context(&[2], &[]);
    probe.solve_cases(&sources, limit()).unwrap();
    let fits = probe.runs()[0].work.case().exact().unwrap();
    // A case limit that the unloaded case's 128-bit selection fits and the
    // loaded case's escalation does not: one batch, A selected, B unavailable.
    let mut ctx = context(&[2], &[2, 2, 2, 2, 2]);
    let cases = ctx.solve_cases(&sources, CaseLimit::new(fits)).unwrap();
    assert!(matches!(cases[0].outcome, ExecutionOutcome::Selected(_)));
    assert!(matches!(cases[1].outcome, ExecutionOutcome::Unresolved { reason: UnresolvedReason::Budget(BudgetScope::Case), .. }),
        "{:?}", cases[1].outcome);
    let b_source = ctx.calls()[0].sources[1];
    let rebuilt = PreparedCaseSource::new(loaded.source()).unwrap();
    assert_eq!(rebuilt.identity(), &ctx.sources()[b_source].identity[..]);
    // The combination takes its own (unlimited) limit and selects.
    let a = selected(&cases[0]);
    let ok = b2k_with_run(ctx.solve_combination_sources(&[(1.0, RecordedOperand::Selected(a)),
        (1.0, RecordedOperand::Prepared { source: b_source, prepared: &rebuilt })], limit()).unwrap());
    let ExecutionOutcome::Selected(combination) = &ok.outcome else { panic!("{:?}", ok.outcome) };
    // Its ledger is the exact combined ledger of A (no loads) and B: B's own nets.
    assert_eq!(combination.evidence().ledger_encoding, rebuilt.prep().ledger.encoding());
    // Refusals at the first offending operand: a rebuild from a changed source,
    // an out-of-range id, and a combination's own source.
    let mut changed = loaded.clone();
    changed.parts.loads[0].value *= 2.0;
    let wrong = PreparedCaseSource::new(changed.source()).unwrap();
    let refusal = |out: Result<RecordedKernelCombination, OriginError>| match out.unwrap() {
        RecordedKernelCombination::OriginRefusal { error, .. } => error, other => panic!("{other:?}") };
    let runs = ctx.runs().len();
    assert_eq!(refusal(ctx.solve_combination_sources(&[(1.0, RecordedOperand::Selected(a)),
        (1.0, RecordedOperand::Prepared { source: b_source, prepared: &wrong })], limit())),
        OriginError::MissingSelectedOrigin { operand: 1 });
    assert_eq!(refusal(ctx.solve_combination_sources(&[(1.0, RecordedOperand::Selected(a)),
        (1.0, RecordedOperand::Prepared { source: 99, prepared: &rebuilt })], limit())),
        OriginError::MissingSelectedOrigin { operand: 1 });
    assert_eq!(ctx.calls().last().unwrap().requested_operands[1].source, None);
    let cmb = ctx.runs()[ok.run].source;
    assert_eq!(refusal(ctx.solve_combination_sources(&[(1.0, RecordedOperand::Prepared { source: cmb, prepared: &rebuilt }),
        (1.0, RecordedOperand::Selected(a))], limit())), OriginError::MissingSelectedOrigin { operand: 0 });
    assert_eq!(ctx.runs().len(), runs, "origin refusals make no Run");
    // A kernel-selected source (a case whose freeze failed) is accepted as Prepared (R-11).
    let rebuilt_a = PreparedCaseSource::new(zero.source()).unwrap();
    let a_source = ctx.calls()[0].sources[0];
    let again = b2k_with_run(ctx.solve_combination_sources(&[(1.0, RecordedOperand::Selected(a)),
        (1.0, RecordedOperand::Prepared { source: a_source, prepared: &rebuilt_a })], limit()).unwrap());
    assert!(matches!(again.outcome, ExecutionOutcome::Selected(_)));
}
#[test]
fn b2k_k04_no_selected_operand_is_refused_after_the_native_checks() {
    let (a, b) = (models::model("N05").source(), models::model("N05-TRANSVERSE").source());
    let other = models::model("N06").source();
    let mut ctx = b2k_ctx(&[1], &[1, 2, 1, 2, 2], 3);
    let cases = ctx.solve_cases(std::slice::from_ref(&a), limit()).unwrap();
    let nested = b2k_with_run(ctx.solve_combination(&[(1.0, selected(&cases[0]))], limit()).unwrap());
    let (pa, pb, po) = (PreparedCaseSource::new(a.clone()).unwrap(), PreparedCaseSource::new(b).unwrap(),
        PreparedCaseSource::new(other).unwrap());
    let ids = [&pa, &pb, &po].map(|p| ctx.register_prepared_source(p).unwrap());
    let charged = ctx.meter().checked_charged();
    let (runs, sources) = (ctx.runs().len(), ctx.sources().len());
    let reason = |out: Result<RecordedKernelCombination, OriginError>| match out.unwrap() {
        RecordedKernelCombination::PreSourceRefusal { stage, reason, .. } => (stage, reason), other => panic!("{other:?}") };
    let (qa, qb, qo) = (RecordedOperand::Prepared { source: ids[0], prepared: &pa },
        RecordedOperand::Prepared { source: ids[1], prepared: &pb }, RecordedOperand::Prepared { source: ids[2], prepared: &po });
    assert_eq!(reason(ctx.solve_combination_sources(&[(f64::NAN, qa)], limit())),
        (CombinationStage::OperandValidation, CombinationReason::NoOperands));
    assert_eq!(reason(ctx.solve_combination_sources(&[(1.0, RecordedOperand::Selected(selected(&nested))), (1.0, qa)], limit())),
        (CombinationStage::OperandValidation, CombinationReason::NestedCombination));
    assert_eq!(reason(ctx.solve_combination_sources(&[(1.0, qa), (1.0, qo)], limit())),
        (CombinationStage::OperandValidation, CombinationReason::OperandsDiffer));
    assert_eq!(reason(ctx.solve_combination_sources(&[(1.0, qa), (1.0, qb)], limit())),
        (CombinationStage::OperandValidation, CombinationReason::NoSelectedOperand));
    assert_eq!((ctx.meter().checked_charged(), ctx.runs().len(), ctx.sources().len()), (charged, runs, sources));
    assert!(matches!(ctx.calls().last().unwrap().result,
        CallResult::PreSourceRefusal { reason: CombinationReason::NoSelectedOperand, .. }));
}
#[test]
fn b2k_k05_a_prepared_first_operand_keeps_authored_order_and_takes_the_group_of_operand_one() {
    let (a, b) = (models::model("N05").source(), models::model("N05-TRANSVERSE").source());
    let mut ctx = b2k_ctx(&[1], &[2, 2], 1);
    let cases = ctx.solve_cases(std::slice::from_ref(&a), limit()).unwrap();
    let pb = PreparedCaseSource::new(b.clone()).unwrap();
    let id = ctx.register_prepared_source(&pb).unwrap();
    let sa = selected(&cases[0]);
    let first = b2k_with_run(ctx.solve_combination_sources(&[(2.0, RecordedOperand::Prepared { source: id, prepared: &pb }),
        (1.0, RecordedOperand::Selected(sa))], limit()).unwrap());
    let ExecutionOutcome::Selected(c) = &first.outcome else { panic!() };
    assert!(Arc::ptr_eq(&c.group, &sa.group), "the group is the selected operand's");
    assert_eq!(c.prep.source.encoding(), b.encoding(), "operand 0 stays the representative");
    let group = &ctx.groups()[ctx.runs()[first.run].group.unwrap()];
    assert!(group.imports.iter().flatten().all(|i| i.operand_index == 1 && i.selected_run == cases[0].run));
    assert!(group.imports.iter().flatten().count() > 0);
    assert_eq!(ctx.calls()[1].requested_operands.iter().map(|o| o.source).collect::<Vec<_>>(), [Some(id), Some(0)]);
    let second = b2k_with_run(ctx.solve_combination_sources(&[(1.0, RecordedOperand::Selected(sa)),
        (2.0, RecordedOperand::Prepared { source: id, prepared: &pb })], limit()).unwrap());
    let ExecutionOutcome::Selected(d) = &second.outcome else { panic!() };
    assert_eq!(c.publish(), d.publish(), "authored order changes identity, not numerics");
    assert_ne!(c.prep.identity, d.prep.identity);
    assert_eq!(c.evidence().ledger_encoding, d.evidence().ledger_encoding);
    // Two selected operands from different Calls have distinct (equal) groups:
    // the combination runs on the first selected operand's, after a prepared first.
    let mut ctx = b2k_ctx(&[1, 1], &[3, 3], 1);
    let first = ctx.solve_cases(std::slice::from_ref(&a), limit()).unwrap();
    let later = ctx.solve_cases(std::slice::from_ref(&a), limit()).unwrap();
    let (s1, s2) = (selected(&first[0]), selected(&later[0]));
    assert!(!Arc::ptr_eq(&s1.group, &s2.group));
    let id = ctx.register_prepared_source(&pb).unwrap();
    for (x, y) in [(s1, s2), (s2, s1)] {
        let out = b2k_with_run(ctx.solve_combination_sources(&[(1.0, RecordedOperand::Prepared { source: id, prepared: &pb }),
            (1.0, RecordedOperand::Selected(x)), (1.0, RecordedOperand::Selected(y))], limit()).unwrap());
        let ExecutionOutcome::Selected(e) = &out.outcome else { panic!() };
        assert!(Arc::ptr_eq(&e.group, &x.group));
        // Imports come from the first selected operand in authored order.
        let (got, want) = (e.cache.test_slot_addresses(), x.cache.test_slot_addresses());
        for i in 0..7 {
            if want[i] != 0 { assert_eq!(got[i], want[i], "slot {i}"); }
        }
        let mut meter = InvocationMeter::new(u64::MAX);
        let CombinationOutcome::Selected(u) = RetainedCombination::solve_sources(&[
            (1.0, super::super::combine::CombinationOperand::Prepared(&pb)),
            (1.0, super::super::combine::CombinationOperand::Retained(x)),
            (1.0, super::super::combine::CombinationOperand::Retained(y))], limit(), &mut meter) else { panic!() };
        assert!(Arc::ptr_eq(&u.group, &x.group));
        let (got, want) = (u.cache.test_slot_addresses(), x.cache.test_slot_addresses());
        for i in 0..7 {
            if want[i] != 0 { assert_eq!(got[i], want[i], "unrecorded slot {i}"); }
        }
    }
}
#[test]
fn b2k_k06_product_owner_is_the_recorded_kind_of_the_run() {
    let a = models::model("N05").source();
    let mut ctx = context(&[1], &[1]);
    let cases = ctx.solve_cases(std::slice::from_ref(&a), limit()).unwrap();
    let combination = b2k_with_run(ctx.solve_combination(&[(1.0, selected(&cases[0]))], limit()).unwrap());
    let (case_run, comb_run) = (cases[0].run, combination.run);
    assert_eq!(ctx.product_owner(case_run, selected(&cases[0])), Some(NativeOwner::Case(0)));
    assert_eq!(ctx.product_owner(comb_run, selected(&combination)), Some(NativeOwner::Combination(0)));
    assert_eq!(ctx.product_owner(case_run, selected(&combination)), None, "another run id");
    assert_eq!(ctx.product_owner(99, selected(&combination)), None);
    // I9: the recorded kind must agree with the prep (factors empty for a case).
    ctx.store.runs[comb_run].owner = NativeOwner::Case(7);
    assert_eq!(ctx.product_owner(comb_run, selected(&combination)), None);
    ctx.store.runs[comb_run].owner = NativeOwner::Combination(0);
    ctx.store.runs[case_run].owner = NativeOwner::Combination(7);
    assert_eq!(ctx.product_owner(case_run, selected(&cases[0])), None);
}
#[test]
fn b2k_c01_c04_mixed_operands_match_all_selected_bits_charge_nothing_and_change_no_operand() {
    let (a, b) = (models::model("N05").source(), models::model("N05-TRANSVERSE").source());
    let mut all = context(&[2], &[2]);
    let both = all.solve_cases(&[a.clone(), b.clone()], limit()).unwrap();
    let full = b2k_with_run(all.solve_combination(&[(1.0, selected(&both[0])), (1.0, selected(&both[1]))], limit()).unwrap());
    let mut mixed = b2k_ctx(&[1], &[2, 2], 1);
    let one = mixed.solve_cases(std::slice::from_ref(&a), limit()).unwrap();
    let pb = PreparedCaseSource::new(b.clone()).unwrap();
    let id = mixed.register_prepared_source(&pb).unwrap();
    let before = (format!("{:?}", selected(&one[0])), format!("{pb:?}"), mixed.store.selected[0].slots);
    let half = b2k_with_run(mixed.solve_combination_sources(&[(1.0, RecordedOperand::Selected(selected(&one[0]))),
        (1.0, RecordedOperand::Prepared { source: id, prepared: &pb })], limit()).unwrap());
    let (f, h) = (selected(&full), selected(&half));
    assert_eq!(f.publish(), h.publish());
    assert_eq!((&f.evidence().ledger_encoding, &f.evidence().retained_state_encoding, &f.evidence().source_encoding),
        (&h.evidence().ledger_encoding, &h.evidence().retained_state_encoding, &h.evidence().source_encoding));
    assert_eq!(mixed.runs().len(), 2, "B has no Run");
    assert!(mixed.meter().charged() < all.meter().charged(), "B is not charged");
    // One prepared source serves a second combination; nothing about an operand changed.
    let again = b2k_with_run(mixed.solve_combination_sources(&[(1.0, RecordedOperand::Selected(selected(&one[0]))),
        (1.0, RecordedOperand::Prepared { source: id, prepared: &pb })], limit()).unwrap());
    assert_eq!(selected(&again).publish(), h.publish());
    assert_eq!(before, (format!("{:?}", selected(&one[0])), format!("{pb:?}"), mixed.store.selected[0].slots));
    assert_eq!(mixed.runs()[again.run].cache_before, mixed.runs()[one[0].run].cache_after, "no reuse of a combination's slots");
}
#[test]
fn b2k_w05_w06_custody_of_terminal_work_and_imports_from_selected_operands_only() {
    let loaded = models::model("SKEW6-K1E-12");
    let mut zero = loaded.clone();
    zero.parts.loads.clear();
    let sources = [zero.source(), loaded.source()];
    // W06: one batch; the loaded case builds 256-bit slots in the shared group,
    // but as a prepared operand it lends none: imports come from operand 0 only.
    let mut ctx = context(&[2], &[2, 2]);
    let cases = ctx.solve_cases(&sources, limit()).unwrap();
    assert_eq!(selected(&cases[1]).selected_precision(), 256);
    let rebuilt = PreparedCaseSource::new(loaded.source()).unwrap();
    let b_source = ctx.calls()[0].sources[1];
    let ops = [(1.0, RecordedOperand::Selected(selected(&cases[0]))), (1.0, RecordedOperand::Prepared { source: b_source, prepared: &rebuilt })];
    let c = b2k_with_run(ctx.solve_combination_sources(&ops, limit()).unwrap());
    let run = &ctx.runs()[c.run];
    assert!(ctx.groups()[run.group.unwrap()].imports.iter().flatten().all(|i| i.operand_index == 0 && i.selected_run == cases[0].run));
    assert_eq!(run.cache_before, ctx.runs()[cases[0].run].cache_after);
    let ExecutionOutcome::Selected(sc) = &c.outcome else { panic!() };
    assert_eq!(sc.selected_precision(), 256);
    assert!(sc.evidence().attempts.iter().any(|x| x.shared_built_here), "the combination builds what operand 0 lacked");
    // Unrecorded, the same: independent calls rebuild and charge.
    let mut meter = InvocationMeter::new(u64::MAX);
    let legacy = RetainedCombination::solve_sources(&[(1.0, super::super::combine::CombinationOperand::Retained(selected(&cases[0]))),
        (1.0, super::super::combine::CombinationOperand::Prepared(&rebuilt))], limit(), &mut meter);
    let CombinationOutcome::Selected(l) = legacy else { panic!() };
    assert_eq!(l.publish(), sc.publish());
    assert!(meter.charged() > 0);
    // W05: a terminal unavailability after nonzero work keeps its attempts in
    // the recorded custody of solve_sources_recorded, and its legacy projection.
    let mut meter = InvocationMeter::new(u64::MAX);
    let recorded = RetainedCombination::solve_sources_recorded(&[(1.0, super::super::combine::CombinationOperand::Retained(selected(&cases[0]))),
        (1.0, super::super::combine::CombinationOperand::Prepared(&rebuilt))], CaseLimit::new(1), &mut meter);
    let super::super::combine::RecordedCombination::WithRun { outcome, work } = recorded.clone() else { panic!() };
    let CombinationOutcome::Unresolved { reason, attempts } = &outcome else { panic!() };
    assert_eq!(reason, &CombinationReason::Unresolved(UnresolvedReason::Budget(BudgetScope::Case)));
    assert!(!attempts.is_empty() && work.case().exact().unwrap() > 0);
    assert!(matches!(recorded.into_legacy(), CombinationOutcome::Unresolved { attempts, .. } if !attempts.is_empty()));
    // A terminal Refused keeps its attempts in custody and loses them only in
    // the legacy projection (unreachable here without a seeded fault: the
    // selected operand's factor already passed).
    let refused = super::super::combine::RecordedCombination::WithRun { outcome: CombinationOutcome::Unresolved {
        reason: CombinationReason::Refused(super::super::adaptive::Refusal::Structure), attempts: attempts.clone() }, work };
    assert!(matches!(&refused, super::super::combine::RecordedCombination::WithRun { outcome: CombinationOutcome::Unresolved { attempts, .. }, .. } if !attempts.is_empty()));
    assert!(matches!(refused.into_legacy(), CombinationOutcome::Unresolved { attempts, .. } if attempts.is_empty()));
}
#[test]
fn b2k_k03_each_i7_check_refuses_on_its_own() {
    // I7's three conditions are redundant in any actual store (a case source
    // never has a combination ledger, and a combination's K4CMB bytes are never
    // a K4SRC). Each is shown live by tampering one recorded fact.
    let (a, b) = (models::model("N05").source(), models::model("N05-TRANSVERSE").source());
    let mut ctx = b2k_ctx(&[1], &[2, 2, 2], 1);
    let cases = ctx.solve_cases(std::slice::from_ref(&a), limit()).unwrap();
    let pb = PreparedCaseSource::new(b).unwrap();
    let id = ctx.register_prepared_source(&pb).unwrap();
    let ops = |ctx: &mut RecordedInvocation| ctx.solve_combination_sources(&[(1.0, RecordedOperand::Selected(selected(&cases[0]))),
        (1.0, RecordedOperand::Prepared { source: id, prepared: &pb })], limit()).unwrap();
    ctx.store.sources[id].owner = NativeOwner::Combination(0);
    assert!(matches!(ops(&mut ctx), RecordedKernelCombination::OriginRefusal { error: OriginError::MissingSelectedOrigin { operand: 1 }, .. }));
    ctx.store.sources[id].owner = NativeOwner::Case(1);
    ctx.store.sources[id].combination_ledger = Some(b"K4LED\x01\x00\x00\x00\x00".to_vec());
    assert!(matches!(ops(&mut ctx), RecordedKernelCombination::OriginRefusal { error: OriginError::MissingSelectedOrigin { operand: 1 }, .. }));
    ctx.store.sources[id].combination_ledger = None;
    assert!(matches!(ops(&mut ctx), RecordedKernelCombination::WithRun { .. }));
}
