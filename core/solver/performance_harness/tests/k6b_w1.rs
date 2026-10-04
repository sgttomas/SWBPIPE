//! K6b: the W1 mode's library side (T3 K6b plan §6; ROOT's rulings on I16's
//! plan). Debug tests stay at 100 members or fewer. No time or memory bound is
//! asserted anywhere.
//!
//! - Determinism of outcomes, publications, evidence and work across two runs.
//! - The staged `w1_solve` equals K4's own `solve_case` and `solve_cases`.
//! - R1's unchanged predicate at 10 and 100 members, on every R1 row of the
//!   twelve RF-LARGE frames (`K4T/r1_large.txt`, its sha256 asserted).
//! - W1's counts against every attempt's storage; the work closure; the work by
//!   precision; the prefixes; the estimate against a hand derivation.
//! - A build that stops partway (ROOT's ruling on the K6B-S3 stop): since T3
//!   KF3 it stages the partial work of the stage it stopped in, so its stages
//!   equal its charged total and nothing is unstaged (ROOT's ruling "KF3: main
//!   merged; K6b's parity restored in KF3").

mod k6b_support;

use k6b_support::{r1_rows, sha256_hex, R1_LARGE, R1_LARGE_SHA256};
use open_pipe_stress_frame_kernel::structural::retained_api::{
    solve_case, solve_cases, AttemptOutcome, AttemptReason, AttemptRecord, AttemptRole,
    AttemptStop, BudgetScope, CaseLimit, CaseOutcome, InvocationMeter, RetainedSolve, StageWork,
    UnresolvedReason,
};
use open_pipe_stress_solver_performance_harness::k6::counts::parse_counts_line;
use open_pipe_stress_solver_performance_harness::k6::models::model;
use open_pipe_stress_solver_performance_harness::k6::staged::{NoObserver, Stage};
use open_pipe_stress_solver_performance_harness::k6::w1::adapter::source;
use open_pipe_stress_solver_performance_harness::k6::w1::counts::{
    compute, limbs_per_entry, W1Counts, W1Estimate,
};
use open_pipe_stress_solver_performance_harness::k6::w1::envelope::{KernelPhase, PhaseId};
use open_pipe_stress_solver_performance_harness::k6::w1::h_envelope::{
    original_k6b_pair, HComposedEstimate, ReferenceHProfile,
};
use open_pipe_stress_solver_performance_harness::k6::w1::rows::{r1_passes, r1_values};
use open_pipe_stress_solver_performance_harness::k6::w1::staged::{
    attempts_of, builds_completed, own_total, prefix_limits, prefix_matches, segments,
    shared_total, stage_sum, stages_complete, stages_equal_totals, unstaged, w1_solve,
    work_by_precision, work_closes, W1Limits, W1Solve,
};

fn rf(size: usize) -> Vec<String> {
    let mut ids = Vec::new();
    for family in ["CHAIN", "TREE", "CONT"] {
        for orientation in ["AX", "ROT"] {
            ids.push(format!("RF-LARGE-{family}-n{size:05}-{orientation}"));
        }
    }
    ids
}

fn run(id: &str, limits: W1Limits) -> W1Solve {
    let m = model(id).expect("model");
    let src = source(&m).expect("source");
    w1_solve(src, limits, Stage::W1Solve, &mut NoObserver)
}

fn selected(solve: &W1Solve) -> &RetainedSolve {
    match &solve.outcome {
        CaseOutcome::Selected(s) => s,
        other => panic!("not selected: {other:?}"),
    }
}

#[test]
fn outcomes_publications_evidence_and_work_are_deterministic() {
    for id in rf(10) {
        let (a, b) = (run(&id, W1Limits::default()), run(&id, W1Limits::default()));
        let (sa, sb) = (selected(&a), selected(&b));
        assert_eq!(sa.publish(), sb.publish(), "{id}");
        assert_eq!(sa.evidence(), sb.evidence(), "{id}");
        assert_eq!(a.charged, b.charged, "{id}");
    }
}

#[test]
fn the_staged_solve_equals_k4s_own_entries() {
    let id = "RF-LARGE-TREE-n00010-ROT";
    let staged = run(id, W1Limits::default());
    let m = model(id).unwrap();
    let mut meter = InvocationMeter::new(u64::MAX);
    let direct = solve_case(source(&m).unwrap(), CaseLimit::new(u64::MAX), &mut meter);
    let mut meter_all = InvocationMeter::new(u64::MAX);
    let all = solve_cases(
        &[source(&m).unwrap()],
        CaseLimit::new(u64::MAX),
        &mut meter_all,
    );
    assert_eq!(all.len(), 1);
    for other in [&direct, &all[0]] {
        let CaseOutcome::Selected(o) = other else {
            panic!("{id}: not selected")
        };
        assert_eq!(o.publish(), selected(&staged).publish());
        assert_eq!(o.evidence(), selected(&staged).evidence());
    }
    assert_eq!(meter.charged(), staged.charged.exact().unwrap());
    assert_eq!(meter_all.charged(), staged.charged.exact().unwrap());
}

/// Every R1 row of `id` under the unchanged predicate; W1's counts against the
/// attempts' storage; the closure identities.
fn check_r1_and_counts(id: &str) {
    let m = model(id).unwrap();
    let solve = run(id, W1Limits::default());
    let s = selected(&solve);
    assert_eq!(s.selected_precision(), 128, "{id}");
    let obs = r1_values(&m, s.publish());
    let rows = r1_rows(id);
    assert!(rows.len() > 100, "{id}: {} R1 rows", rows.len());
    for row in &rows {
        let o = *obs
            .get(&row.key)
            .unwrap_or_else(|| panic!("{id}: no published value for R1's {}", row.key));
        assert!(
            r1_passes(o, row.exp, row.scale),
            "{id} {}: obs {o:e}, exp {:e}, scale {:e}",
            row.key,
            row.exp,
            row.scale
        );
    }
    let (w1, error) = compute(&m);
    assert!(w1.source_ok && error.is_none(), "{id}");
    let attempts = attempts_of(&solve.outcome);
    for a in attempts {
        assert_eq!(a.storage.profile_entries, w1.profile_entries, "{id}");
        assert_eq!(a.storage.pattern_entries, w1.pattern_entries, "{id}");
        assert_eq!(
            a.storage.limbs_per_entry,
            limbs_per_entry(a.precision),
            "{id}"
        );
    }
    assert!(stages_equal_totals(attempts), "{id}");
    assert!(work_closes(attempts, solve.charged), "{id}");
    assert_eq!(w1.rows, s.publish().rows.len(), "{id}");
}

#[test]
fn r1_large_txt_is_the_pinned_file() {
    assert_eq!(sha256_hex(R1_LARGE.as_bytes()), R1_LARGE_SHA256);
}

#[test]
fn r1_predicate_and_counts_at_10_members() {
    for id in rf(10) {
        check_r1_and_counts(&id);
    }
}

#[test]
fn r1_predicate_and_counts_at_100_members_chain() {
    for id in ["RF-LARGE-CHAIN-n00100-AX", "RF-LARGE-CHAIN-n00100-ROT"] {
        check_r1_and_counts(id);
    }
}

#[test]
fn r1_predicate_and_counts_at_100_members_tree() {
    for id in ["RF-LARGE-TREE-n00100-AX", "RF-LARGE-TREE-n00100-ROT"] {
        check_r1_and_counts(id);
    }
}

#[test]
fn r1_predicate_and_counts_at_100_members_cont() {
    for id in ["RF-LARGE-CONT-n00100-AX", "RF-LARGE-CONT-n00100-ROT"] {
        check_r1_and_counts(id);
    }
}

fn stage_array(w: &StageWork) -> [u64; 19] {
    [
        w.formation,
        w.assembly,
        w.residual_formation,
        w.factor,
        w.condition,
        w.rhs,
        w.solve,
        w.refinement,
        w.recovery,
        w.stop_rule,
        w.bounded_gate,
        w.scale,
        w.estimate,
        w.charge,
        w.bound,
        w.shift,
        w.bounded_formation,
        w.wide_formation,
        w.uc,
    ]
}

#[test]
fn work_by_precision_files_each_attempt_under_its_own_precision() {
    let solve = run("RF-LARGE-CHAIN-n00010-ROT", W1Limits::default());
    let attempts: &[AttemptRecord] = attempts_of(&solve.outcome);
    let by = work_by_precision(attempts).unwrap();
    assert_eq!(
        by.iter().map(|w| w.precision).collect::<Vec<_>>(),
        vec![128, 256]
    );
    for w in &by {
        let mine: Vec<&AttemptRecord> = attempts
            .iter()
            .filter(|a| a.precision == w.precision)
            .collect();
        assert_eq!(w.attempts, mine.len());
        let mut own = [0u64; 19];
        let mut shared = [0u64; 19];
        for a in &mine {
            for (k, v) in stage_array(&a.stages).iter().enumerate() {
                own[k] += v;
            }
            for (k, v) in stage_array(&a.shared_stages).iter().enumerate() {
                shared[k] += v;
            }
        }
        assert_eq!(stage_array(&w.own), own, "{}", w.precision);
        assert_eq!(stage_array(&w.shared), shared, "{}", w.precision);
        // Completed builds: the charged totals equal the stage sums.
        assert_eq!(w.own_total, own.iter().sum::<u64>(), "{}", w.precision);
        assert_eq!(
            w.shared_total,
            shared.iter().sum::<u64>(),
            "{}",
            w.precision
        );
    }
    // The verification pass and the stop rule are where K4 files them: the
    // 256 verification carries the pass (and the shift), the 128 candidate the
    // stop rule.
    assert!(by[1].own.scale > 0 && by[1].own.shift > 0);
    assert!(by[0].own.stop_rule > 0 && by[1].own.stop_rule == 0);
    // Each attempt's own stages add up to its own work.
    for a in attempts {
        assert!(builds_completed(a));
        assert_eq!(stage_sum(&a.stages), own_total(a));
        assert_eq!(unstaged(a), (0, 0));
    }
}

/// ROOT's ruling on the K6B-S3 stop: a build that stops partway. A case limit
/// inside the 256 verification's `uc` stage stops its shared build with
/// `Budget(Case)` on the error path row 247 took with `Span`. Since T3 KF3 the
/// build stages its partial `uc` work in `uc`, so the stages equal the charged
/// totals and nothing is unstaged; one short side now fails the check (ROOT's
/// ruling "KF3: main merged; K6b's parity restored in KF3"; RV22's C-N2).
#[test]
fn a_build_that_stops_partway_is_checked_against_its_charged_total() {
    let id = "RF-LARGE-CHAIN-n00010-AX";
    let full = run(id, W1Limits::default());
    let full_attempts = attempts_of(&full.outcome);
    let v = &full_attempts[1];
    assert_eq!((v.precision, v.role), (256, AttemptRole::Verification));
    assert!(v.verification_shared_built_here && v.shared_stages.uc > 1);
    // The verification's shared build starts where solve_256 ends and runs
    // bounded_formation, wide_formation, then uc.
    let segs = segments(full_attempts).unwrap();
    assert_eq!(segs[1].label, "solve_256");
    let into_uc = segs[1].end
        + v.shared_stages.bounded_formation
        + v.shared_stages.wide_formation
        + v.shared_stages.uc / 2;
    let stopped = run(
        id,
        W1Limits {
            case: into_uc,
            invocation: u64::MAX,
        },
    );
    assert!(
        matches!(
            stopped.outcome,
            CaseOutcome::Unresolved {
                reason: UnresolvedReason::Budget(BudgetScope::Case),
                ..
            }
        ),
        "{:?}",
        stopped.outcome
    );
    let attempts = attempts_of(&stopped.outcome);
    assert_eq!(attempts.len(), 2);
    let (c, a) = (&attempts[0], &attempts[1]);
    assert_eq!(
        c.outcome,
        AttemptOutcome::Rejected(AttemptReason::VerificationFailed)
    );
    assert_eq!(
        a.outcome,
        AttemptOutcome::Failed(AttemptReason::Stop(AttemptStop::Budget(BudgetScope::Case)))
    );
    assert!(builds_completed(c) && !builds_completed(a));
    // The stopped build: its `uc` stage holds the partial uc work (T3 KF3),
    // and nothing charged is unstaged, on either attempt.
    assert!(a.shared_stages.uc > 0 && a.shared_stages.uc < v.shared_stages.uc);
    assert_eq!(
        a.shared_stages.wide_formation,
        v.shared_stages.wide_formation
    );
    assert_eq!((unstaged(c), unstaged(a)), ((0, 0), (0, 0)));
    assert!(stages_complete(c) && stages_complete(a));
    assert_eq!(stage_sum(&a.shared_stages), shared_total(a));
    assert_eq!(stage_sum(&a.stages), own_total(a));
    assert!(stages_equal_totals(attempts));
    assert!(work_closes(attempts, stopped.charged));
    // The work by precision reports the charged totals, equal to the stages.
    let by = work_by_precision(attempts).unwrap();
    assert_eq!(by[1].precision, 256);
    assert_eq!(by[1].shared_total, shared_total(a));
    assert_eq!(by[1].shared_total, stage_sum(&by[1].shared));
    assert_eq!(by[1].own_total, own_total(a));
    // A stage sum above its charged total fails the check.
    let mut over = attempts.to_vec();
    over[1].shared_stages.uc += 1;
    assert!(!stages_equal_totals(&over));
    // One short side, which the check accepted on a stopped build before KF3,
    // now fails, on either side.
    let mut shared_short = attempts.to_vec();
    shared_short[1].shared_stages.uc -= 1;
    assert!(!stages_equal_totals(&shared_short));
    let mut own_short = attempts.to_vec();
    own_short[1].stages.rhs -= 1;
    assert!(!stages_equal_totals(&own_short));
    // A completed build is held to the equality too.
    let mut short = full_attempts.to_vec();
    short[0].stages.rhs -= 1;
    assert!(!stages_equal_totals(&short));
}

/// T3 KF3: a case limit at every segment boundary of the full call, and
/// halfway into every segment, stops a build somewhere; every attempt of
/// every stopped run has nothing unstaged and its stages equal its totals.
#[test]
fn every_stopped_build_leaves_nothing_unstaged() {
    let id = "RF-LARGE-CHAIN-n00010-AX";
    let full = run(id, W1Limits::default());
    let segs = segments(attempts_of(&full.outcome)).unwrap();
    let mut limits = Vec::new();
    let mut start = 0;
    for s in &segs {
        limits.push(start + (s.end - start) / 2);
        limits.push(s.end);
        start = s.end;
    }
    let mut stopped_builds = 0;
    for limit in limits {
        let solve = run(
            id,
            W1Limits {
                case: limit,
                invocation: u64::MAX,
            },
        );
        let attempts = attempts_of(&solve.outcome);
        for a in attempts {
            assert_eq!(unstaged(a), (0, 0), "limit {limit}: {:?}", a.outcome);
            assert!(stages_complete(a), "limit {limit}");
            stopped_builds += usize::from(!builds_completed(a));
        }
        assert!(stages_equal_totals(attempts), "limit {limit}");
        assert!(work_closes(attempts, solve.charged), "limit {limit}");
    }
    assert!(stopped_builds > 0);
}

/// RV22 C-N1: stop after the 128 shared build, ten LME into the own solve.
/// KF3 records partial own work exactly; "short" here means less work than
/// the completed own solve, never permission for an unrecorded stage remainder.
#[test]
fn into_solve_128_records_partial_own_work_and_complete_shared_work() {
    let id = "RF-LARGE-CHAIN-n00010-AX";
    let full = run(id, W1Limits::default());
    let original = &attempts_of(&full.outcome)[0];
    let stopped = run(
        id,
        W1Limits {
            case: shared_total(original) + 10,
            invocation: u64::MAX,
        },
    );
    assert!(matches!(
        stopped.outcome,
        CaseOutcome::Unresolved {
            reason: UnresolvedReason::Budget(BudgetScope::Case),
            ..
        }
    ));
    let attempts = attempts_of(&stopped.outcome);
    assert_eq!(attempts.len(), 1);
    let a = &attempts[0];
    assert_eq!(a.precision, 128);
    assert_eq!(shared_total(a), shared_total(original));
    assert!(own_total(a) > 0 && own_total(a) < own_total(original));
    assert_eq!(unstaged(a), (0, 0));
    assert!(stages_equal_totals(attempts));
    assert!(work_closes(attempts, stopped.charged));
    assert!(a.stages.rhs > 0);
    let mut under = attempts.to_vec();
    under[0].stages.rhs -= 1;
    assert!(!stages_equal_totals(&under));
}

/// RV22-1: a candidate stopped in its stop rule (a case limit at the end of
/// the 256 verification) had every build complete, and K4 files the stop
/// rule's partial work in its stages. So it is held to equality: RV22's probe,
/// 1 LME under-recorded on either side, fails.
#[test]
fn a_stop_in_the_stop_rule_is_held_to_equality() {
    let id = "RF-LARGE-CHAIN-n00010-AX";
    let full = run(id, W1Limits::default());
    let segs = segments(attempts_of(&full.outcome)).unwrap();
    assert_eq!(segs[2].label, "verify_256");
    let stopped = run(
        id,
        W1Limits {
            case: segs[2].end,
            invocation: u64::MAX,
        },
    );
    let attempts = attempts_of(&stopped.outcome);
    let c = &attempts[0];
    assert_eq!(c.role, AttemptRole::Candidate);
    assert_eq!(
        c.outcome,
        AttemptOutcome::Failed(AttemptReason::Stop(AttemptStop::Budget(BudgetScope::Case)))
    );
    assert!(c.stop_rule_work > 0 && c.stages.stop_rule == c.stop_rule_work);
    assert!(builds_completed(c) && stages_complete(c));
    assert!(stages_equal_totals(attempts));
    assert!(work_closes(attempts, stopped.charged));
    let mut own_short = attempts.to_vec();
    own_short[0].stages.rhs -= 1;
    assert!(!stages_equal_totals(&own_short));
    let mut shared_short = attempts.to_vec();
    shared_short[0].shared_stages.factor -= 1;
    assert!(!stages_equal_totals(&shared_short));
}

#[test]
fn prefixes_stop_on_the_case_budget_after_each_segment() {
    let id = "RF-LARGE-CHAIN-n00010-AX";
    let full = run(id, W1Limits::default());
    let full_attempts = attempts_of(&full.outcome).to_vec();
    let segs = segments(&full_attempts).unwrap();
    assert_eq!(
        segs.iter().map(|s| s.label.as_str()).collect::<Vec<_>>(),
        vec!["solve_128", "solve_256", "verify_256", "decide_128"]
    );
    assert_eq!(segs.last().unwrap().end, full.charged.exact().unwrap());
    let limits = prefix_limits(&full_attempts).unwrap();
    assert_eq!(limits.len(), 3);
    // Each prefix limit is its segment's exact end (b_j), not a neighbour of it.
    assert_eq!(
        limits.iter().map(|l| l.1).collect::<Vec<_>>(),
        segs[..3].iter().map(|s| s.end).collect::<Vec<_>>()
    );
    for (j, (_, limit)) in limits.iter().enumerate() {
        let prefix = run(
            id,
            W1Limits {
                case: *limit,
                invocation: u64::MAX,
            },
        );
        assert!(
            matches!(
                prefix.outcome,
                CaseOutcome::Unresolved {
                    reason: UnresolvedReason::Budget(BudgetScope::Case),
                    ..
                }
            ),
            "prefix {}: {:?}",
            j + 1,
            prefix.outcome
        );
        let own = segments(attempts_of(&prefix.outcome)).unwrap();
        assert!(own.len() > j, "prefix {}", j + 1);
        assert_eq!(own[..=j], segs[..=j], "prefix {}", j + 1);
        // The binary's `w1_prefix_segments` (RV22-N3): true for prefix j + 1,
        // false one segment further (that segment was cut short).
        assert!(prefix_matches(j + 1, &full_attempts, &prefix.outcome));
        assert!(!prefix_matches(j + 2, &full_attempts, &prefix.outcome));
        assert!(
            prefix.charged.exact().unwrap() > *limit,
            "the next segment starts before it stops"
        );
    }
    // With the full call's own work as the limit, the call completes.
    let exact = run(
        id,
        W1Limits {
            case: full.charged.exact().unwrap(),
            invocation: u64::MAX,
        },
    );
    assert_eq!(selected(&exact).publish(), selected(&full).publish());
    // A completed call is not a budget-truncated prefix.
    assert!(!prefix_matches(1, &full_attempts, &full.outcome));
}

// Independent source-reviewed H19/RV30 numeric fixtures. All fields are literal
// outputs of the accepted equations; no duplicate estimator or evidence reader.
fn hand(id: &str) -> (W1Estimate, PhaseId) {
    match id {
        "RF-LARGE-CHAIN-n00010-AX" => (
            W1Estimate {
                fixed: 87693,
                shared: [223968, 261328, 445104, 663408],
                state: [18776, 18776, 31224, 56120],
                verify: [169536, 297568, 369312],
                solve: [1910144, 1945856, 2032960, 2064320],
                pass: [157254, 256966, 456390],
                decide: 19888711,
                max: 22744300,
                sel128: 20739628,
            },
            PhaseId {
                phase: KernelPhase::StopRule,
                precision: Some(1024),
            },
        ),
        "RF-LARGE-CHAIN-n00010-ROT" => (
            W1Estimate {
                fixed: 87696,
                shared: [223968, 261328, 445104, 663408],
                state: [18776, 18776, 31224, 56120],
                verify: [169536, 297568, 369312],
                solve: [1910144, 1945856, 2032960, 2064320],
                pass: [157254, 256966, 456390],
                decide: 19888711,
                max: 22744303,
                sel128: 20739631,
            },
            PhaseId {
                phase: KernelPhase::StopRule,
                precision: Some(1024),
            },
        ),
        "RF-LARGE-CHAIN-n00100-AX" => (
            W1Estimate {
                fixed: 681261,
                shared: [2217648, 2580448, 4397184, 6579648],
                state: [178616, 178616, 297624, 535640],
                verify: [1672896, 2941408, 3635232],
                solve: [18667584, 19014336, 19875200, 20209920],
                pass: [1580334, 2586286, 4598190],
                decide: 20330161,
                max: 48275158,
                sel128: 28522486,
            },
            PhaseId {
                phase: KernelPhase::StopRule,
                precision: Some(1024),
            },
        ),
        "RF-LARGE-CHAIN-n00100-ROT" => (
            W1Estimate {
                fixed: 681264,
                shared: [2217648, 2580448, 4397184, 6579648],
                state: [178616, 178616, 297624, 535640],
                verify: [1672896, 2941408, 3635232],
                solve: [18667584, 19014336, 19875200, 20209920],
                pass: [1580334, 2586286, 4598190],
                decide: 20330161,
                max: 48275161,
                sel128: 28522489,
            },
            PhaseId {
                phase: KernelPhase::StopRule,
                precision: Some(1024),
            },
        ),
        "RF-LARGE-CHAIN-n01000-AX" => (
            W1Estimate {
                fixed: 6415541,
                shared: [22154448, 25771648, 43917984, 65742048],
                state: [1777016, 1777016, 2961624, 5330840],
                verify: [16706496, 29379808, 36294432],
                solve: [74014656, 77471808, 85992448, 89205120],
                pass: [15694398, 25684926, 45665982],
                decide: 25608917,
                max: 347434501,
                sel128: 152074133,
            },
            PhaseId {
                phase: KernelPhase::Fallback,
                precision: Some(1024),
            },
        ),
        "RF-LARGE-CHAIN-n01000-ROT" => (
            W1Estimate {
                fixed: 6415544,
                shared: [22154448, 25771648, 43917984, 65742048],
                state: [1777016, 1777016, 2961624, 5330840],
                verify: [16706496, 29379808, 36294432],
                solve: [74014656, 77471808, 85992448, 89205120],
                pass: [15694398, 25684926, 45665982],
                decide: 25608917,
                max: 347434504,
                sel128: 152074136,
            },
            PhaseId {
                phase: KernelPhase::Fallback,
                precision: Some(1024),
            },
        ),
        "RF-LARGE-CHAIN-n10000-AX" => (
            W1Estimate {
                fixed: 63701085,
                shared: [221522448, 257683648, 439125984, 657366048],
                state: [17761016, 17761016, 29601624, 53282840],
                verify: [167042496, 293763808, 362886432],
                solve: [534661824, 569222976, 653882624, 684957312],
                pass: [156146910, 255524446, 454279518],
                decide: 83516477,
                max: 3266456237,
                sel128: 1314694845,
            },
            PhaseId {
                phase: KernelPhase::Fallback,
                precision: Some(1024),
            },
        ),
        "RF-LARGE-CHAIN-n10000-ROT" => (
            W1Estimate {
                fixed: 63701088,
                shared: [221522448, 257683648, 439125984, 657366048],
                state: [17761016, 17761016, 29601624, 53282840],
                verify: [167042496, 293763808, 362886432],
                solve: [534661824, 569222976, 653882624, 684957312],
                pass: [156146910, 255524446, 454279518],
                decide: 83516477,
                max: 3266456240,
                sel128: 1314694848,
            },
            PhaseId {
                phase: KernelPhase::Fallback,
                precision: Some(1024),
            },
        ),
        "RF-LARGE-TREE-n00010-AX" => (
            W1Estimate {
                fixed: 86162,
                shared: [223968, 261328, 445104, 663408],
                state: [18776, 18776, 31224, 56120],
                verify: [169536, 297568, 369312],
                solve: [1910144, 1945856, 2032960, 2064320],
                pass: [157254, 256966, 456390],
                decide: 19888711,
                max: 22742769,
                sel128: 20738097,
            },
            PhaseId {
                phase: KernelPhase::StopRule,
                precision: Some(1024),
            },
        ),
        "RF-LARGE-TREE-n00010-ROT" => (
            W1Estimate {
                fixed: 92213,
                shared: [223968, 261328, 445104, 663408],
                state: [18776, 18776, 31224, 56120],
                verify: [169536, 297568, 369312],
                solve: [1910144, 1945856, 2032960, 2064320],
                pass: [157254, 256966, 456390],
                decide: 19888711,
                max: 22748820,
                sel128: 20744148,
            },
            PhaseId {
                phase: KernelPhase::StopRule,
                precision: Some(1024),
            },
        ),
        "RF-LARGE-TREE-n00100-AX" => (
            W1Estimate {
                fixed: 709388,
                shared: [2217648, 2580448, 4397184, 6579648],
                state: [178616, 178616, 297624, 535640],
                verify: [1672896, 2941408, 3635232],
                solve: [18667584, 19014336, 19875200, 20209920],
                pass: [1580334, 2586286, 4598190],
                decide: 20330161,
                max: 48303285,
                sel128: 28550613,
            },
            PhaseId {
                phase: KernelPhase::StopRule,
                precision: Some(1024),
            },
        ),
        "RF-LARGE-TREE-n00100-ROT" => (
            W1Estimate {
                fixed: 775775,
                shared: [2217648, 2580448, 4397184, 6579648],
                state: [178616, 178616, 297624, 535640],
                verify: [1672896, 2941408, 3635232],
                solve: [18667584, 19014336, 19875200, 20209920],
                pass: [1580334, 2586286, 4598190],
                decide: 20330161,
                max: 48369672,
                sel128: 28617000,
            },
            PhaseId {
                phase: KernelPhase::StopRule,
                precision: Some(1024),
            },
        ),
        "RF-LARGE-TREE-n01000-AX" => (
            W1Estimate {
                fixed: 6736336,
                shared: [22154448, 25771648, 43917984, 65742048],
                state: [1777016, 1777016, 2961624, 5330840],
                verify: [16706496, 29379808, 36294432],
                solve: [74014656, 77471808, 85992448, 89205120],
                pass: [15694398, 25684926, 45665982],
                decide: 25608917,
                max: 347755296,
                sel128: 152394928,
            },
            PhaseId {
                phase: KernelPhase::Fallback,
                precision: Some(1024),
            },
        ),
        "RF-LARGE-TREE-n01000-ROT" => (
            W1Estimate {
                fixed: 7395287,
                shared: [22154448, 25771648, 43917984, 65742048],
                state: [1777016, 1777016, 2961624, 5330840],
                verify: [16706496, 29379808, 36294432],
                solve: [74014656, 77471808, 85992448, 89205120],
                pass: [15694398, 25684926, 45665982],
                decide: 25608917,
                max: 348414247,
                sel128: 153053879,
            },
            PhaseId {
                phase: KernelPhase::Fallback,
                precision: Some(1024),
            },
        ),
        "RF-LARGE-TREE-n10000-AX" => (
            W1Estimate {
                fixed: 67025226,
                shared: [221522448, 257683648, 439125984, 657366048],
                state: [17761016, 17761016, 29601624, 53282840],
                verify: [167042496, 293763808, 362886432],
                solve: [534661824, 569222976, 653882624, 684957312],
                pass: [156146910, 255524446, 454279518],
                decide: 83516477,
                max: 3269780378,
                sel128: 1318018986,
            },
            PhaseId {
                phase: KernelPhase::Fallback,
                precision: Some(1024),
            },
        ),
        "RF-LARGE-TREE-n10000-ROT" => (
            W1Estimate {
                fixed: 73551665,
                shared: [221522448, 257683648, 439125984, 657366048],
                state: [17761016, 17761016, 29601624, 53282840],
                verify: [167042496, 293763808, 362886432],
                solve: [534661824, 569222976, 653882624, 684957312],
                pass: [156146910, 255524446, 454279518],
                decide: 83516477,
                max: 3276306817,
                sel128: 1324545425,
            },
            PhaseId {
                phase: KernelPhase::Fallback,
                precision: Some(1024),
            },
        ),
        "RF-LARGE-CONT-n00010-AX" => (
            W1Estimate {
                fixed: 89630,
                shared: [210312, 247672, 422904, 624120],
                state: [19496, 19496, 32424, 58280],
                verify: [169536, 297568, 369312],
                solve: [1899584, 1935296, 2019520, 2045120],
                pass: [128442, 209626, 371994],
                decide: 19892326,
                max: 22674492,
                sel128: 20722188,
            },
            PhaseId {
                phase: KernelPhase::StopRule,
                precision: Some(1024),
            },
        ),
        "RF-LARGE-CONT-n00010-ROT" => (
            W1Estimate {
                fixed: 94885,
                shared: [210312, 247672, 422904, 624120],
                state: [19496, 19496, 32424, 58280],
                verify: [169536, 297568, 369312],
                solve: [1899584, 1935296, 2019520, 2045120],
                pass: [128442, 209626, 371994],
                decide: 19892326,
                max: 22679747,
                sel128: 20727443,
            },
            PhaseId {
                phase: KernelPhase::StopRule,
                precision: Some(1024),
            },
        ),
        "RF-LARGE-CONT-n00100-AX" => (
            W1Estimate {
                fixed: 741181,
                shared: [2073312, 2436112, 4162224, 6163440],
                state: [185816, 185816, 309624, 557240],
                verify: [1672896, 2941408, 3635232],
                solve: [15231936, 15578688, 16394368, 16638720],
                pass: [1252086, 2046006, 3633846],
                decide: 20366311,
                max: 47492060,
                sel128: 28348508,
            },
            PhaseId {
                phase: KernelPhase::StopRule,
                precision: Some(1024),
            },
        ),
        "RF-LARGE-CONT-n00100-ROT" => (
            W1Estimate {
                fixed: 781719,
                shared: [2073312, 2436112, 4162224, 6163440],
                state: [185816, 185816, 309624, 557240],
                verify: [1672896, 2941408, 3635232],
                solve: [15231936, 15578688, 16394368, 16638720],
                pass: [1252086, 2046006, 3633846],
                decide: 20366311,
                max: 47532598,
                sel128: 28389046,
            },
            PhaseId {
                phase: KernelPhase::StopRule,
                precision: Some(1024),
            },
        ),
        "RF-LARGE-CONT-n01000-AX" => (
            W1Estimate {
                fixed: 7095484,
                shared: [20703312, 24320512, 41555424, 61556640],
                state: [1849016, 1849016, 3081624, 5546840],
                verify: [16706496, 29379808, 36294432],
                solve: [72958656, 76415808, 84648448, 87285120],
                pass: [12642126, 20665806, 36713166],
                decide: 25970417,
                max: 337224204,
                sel128: 148939804,
            },
            PhaseId {
                phase: KernelPhase::Fallback,
                precision: Some(1024),
            },
        ),
        "RF-LARGE-CONT-n01000-ROT" => (
            W1Estimate {
                fixed: 7508841,
                shared: [20703312, 24320512, 41555424, 61556640],
                state: [1849016, 1849016, 3081624, 5546840],
                verify: [16706496, 29379808, 36294432],
                solve: [72958656, 76415808, 84648448, 87285120],
                pass: [12642126, 20665806, 36713166],
                decide: 25970417,
                max: 337637561,
                sel128: 149353161,
            },
            PhaseId {
                phase: KernelPhase::Fallback,
                precision: Some(1024),
            },
        ),
        "RF-LARGE-CONT-n10000-AX" => (
            W1Estimate {
                fixed: 75890191,
                shared: [207003312, 243164512, 415487424, 615488640],
                state: [18481016, 18481016, 30801624, 55442840],
                verify: [167042496, 293763808, 362886432],
                solve: [524101824, 558662976, 640442624, 665757312],
                pass: [125608638, 205307326, 364704702],
                decide: 102860117,
                max: 3169691103,
                sel128: 1288725679,
            },
            PhaseId {
                phase: KernelPhase::Fallback,
                precision: Some(1024),
            },
        ),
        "RF-LARGE-CONT-n10000-ROT" => (
            W1Estimate {
                fixed: 80006511,
                shared: [207003312, 243164512, 415487424, 615488640],
                state: [18481016, 18481016, 30801624, 55442840],
                verify: [167042496, 293763808, 362886432],
                solve: [524101824, 558662976, 640442624, 665757312],
                pass: [125608638, 205307326, 364704702],
                decide: 102860117,
                max: 3173807423,
                sel128: 1292841999,
            },
            PhaseId {
                phase: KernelPhase::Fallback,
                precision: Some(1024),
            },
        ),
        "DEC053:invented-cantilever-chain-8" => (
            W1Estimate {
                fixed: 66872,
                shared: [179664, 209792, 357280, 531936],
                state: [15224, 15224, 25304, 45464],
                verify: [136128, 238816, 296736],
                solve: [1890752, 1919552, 1990144, 2016128],
                pass: [126654, 206910, 367422],
                decide: 19878901,
                max: 22172389,
                sel128: 20560069,
            },
            PhaseId {
                phase: KernelPhase::StopRule,
                precision: Some(1024),
            },
        ),
        "DEC053:invented-cantilever-chain-24" => (
            W1Estimate {
                fixed: 175617,
                shared: [534096, 622080, 1059872, 1583712],
                state: [43640, 43640, 72664, 130712],
                verify: [403392, 708832, 877344],
                solve: [7426496, 7510592, 7719424, 7800704],
                pass: [380670, 622718, 1106814],
                decide: 19957381,
                max: 26717502,
                sel128: 21947934,
            },
            PhaseId {
                phase: KernelPhase::StopRule,
                precision: Some(1024),
            },
        ),
        "DEC053:invented-cantilever-chain-48" => (
            W1Estimate {
                fixed: 335745,
                shared: [1065744, 1240512, 2113760, 3161376],
                state: [86264, 86264, 143704, 258584],
                verify: [804288, 1413856, 1748256],
                solve: [14833344, 15000384, 15415552, 15577728],
                pass: [760158, 1243870, 2211294],
                decide: 20075101,
                max: 33527574,
                sel128: 24025206,
            },
            PhaseId {
                phase: KernelPhase::StopRule,
                precision: Some(1024),
            },
        ),
        "DEC053:invented-grid-frame-4x3" => (
            W1Estimate {
                fixed: 114584,
                shared: [335832, 391592, 668680, 1000008],
                state: [28328, 28328, 47144, 84776],
                verify: [276096, 485920, 592224],
                solve: [1927904, 1980896, 2100448, 2127584],
                pass: [192066, 315330, 561858],
                decide: 19923982,
                max: 24297262,
                sel128: 21205246,
            },
            PhaseId {
                phase: KernelPhase::StopRule,
                precision: Some(1024),
            },
        ),
        "DEC053:invented-grid-frame-6x8" => (
            W1Estimate {
                fixed: 465109,
                shared: [1811232, 2068624, 3522288, 5400240],
                state: [126248, 126248, 210344, 378536],
                verify: [1311744, 2311840, 2800608],
                solve: [7749344, 7993568, 8547808, 8679392],
                pass: [1342530, 2216322, 3963906],
                decide: 20229256,
                max: 42171589,
                sel128: 26608133,
            },
            PhaseId {
                phase: KernelPhase::StopRule,
                precision: Some(1024),
            },
        ),
        "DEC053:invented-grid-frame-7x8" => (
            W1Estimate {
                fixed: 577485,
                shared: [2171352, 2474920, 4213032, 6475176],
                state: [148808, 148808, 247944, 446216],
                verify: [1550208, 2732320, 3308640],
                solve: [15021024, 15309024, 15968608, 16135776],
                pass: [1657962, 2738538, 4899690],
                decide: 20300308,
                max: 46486441,
                sel128: 27935545,
            },
            PhaseId {
                phase: KernelPhase::StopRule,
                precision: Some(1024),
            },
        ),
        "DEC053:invented-grid-frame-5x5" => (
            W1Estimate {
                fixed: 238253,
                shared: [839760, 967168, 1648800, 2502624],
                state: [63224, 63224, 105304, 189464],
                verify: [643008, 1132768, 1375008],
                solve: [3877824, 3998784, 4272640, 4336512],
                pass: [561822, 925726, 1653534],
                decide: 20032573,
                max: 30508450,
                sel128: 23082882,
            },
            PhaseId {
                phase: KernelPhase::StopRule,
                precision: Some(1024),
            },
        ),
        "DEC053:invented-cantilever-chain-32" => (
            W1Estimate {
                fixed: 221057,
                shared: [711312, 828224, 1411168, 2109600],
                state: [57848, 57848, 96344, 173336],
                verify: [537024, 943840, 1167648],
                solve: [7504064, 7615808, 7890688, 7993472],
                pass: [503070, 822942, 1462686],
                decide: 19996621,
                max: 28967302,
                sel128: 22628326,
            },
            PhaseId {
                phase: KernelPhase::StopRule,
                precision: Some(1024),
            },
        ),
        "DEC053:invented-grid-frame-5x6" => (
            W1Estimate {
                fixed: 301501,
                shared: [1045272, 1200616, 2045928, 3115368],
                state: [76712, 76712, 127784, 229928],
                verify: [786432, 1385632, 1680864],
                solve: [7527200, 7674656, 8012320, 8097824],
                pass: [728562, 1201266, 2146674],
                decide: 20074450,
                max: 33019063,
                sel128: 23852231,
            },
            PhaseId {
                phase: KernelPhase::StopRule,
                precision: Some(1024),
            },
        ),
        _ => panic!("no reviewed reference fixture for {id}"),
    }
}
fn assert_estimate_is_hand(c: &W1Counts, id: &str) -> HComposedEstimate {
    let e =
        original_k6b_pair(id, c, &ReferenceHProfile::source40129_rust1971_aarch64_v1()).unwrap();
    let (expected, phase) = hand(id);
    assert_eq!(e.legacy, expected, "every corrected field: {id}");
    assert_eq!(e.dominant_kernel, Some(phase), "phase owner: {id}");
    e
}

#[test]
fn the_estimate_equals_a_hand_derivation() {
    let m = model("RF-LARGE-CHAIN-n00010-AX").unwrap();
    let (c, _) = compute(&m);
    assert_eq!(
        [
            c.nodes,
            c.members,
            c.stations,
            c.constraints,
            c.loads,
            c.dofs,
            c.free_dofs,
            c.bodies,
            c.pattern_entries,
            c.profile_entries,
            c.half_bandwidth,
            c.blocks,
            c.rows,
            c.source_encoding_len
        ],
        [11, 10, 10, 6, 6, 66, 60, 1, 1116, 534, 11, 1, 263, 1512]
    );
    assert!(c.source_ok);
    let e = assert_estimate_is_hand(&c, &m.id);
    // Independent persistent owner arithmetic at p128: Arc, members, pattern,
    // residual coefficients, skyline and per-free-row factor data, no b term.
    assert_eq!(
        e.legacy.shared[0],
        720 + 10 * 7888 + 1116 * (48 + 48) + 10 * 248 + 534 * 48 + 60 * (48 + 104)
    );
    assert_eq!(e.legacy.state[0], 104 + (66 + 6 * 10 + 263) * 48);
    assert_eq!(
        e.dominant_kernel,
        Some(PhaseId {
            phase: KernelPhase::StopRule,
            precision: Some(1024)
        })
    );
}

const COUNTS: &str = include_str!("../observations/k6b/counts.jsonl");

fn number(line: &str, key: &str) -> u128 {
    let pattern = format!("\"{key}\":");
    let start = line.find(&pattern).expect(key) + pattern.len();
    line[start..]
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .expect(key)
}

/// RV22-3: every committed counts line carries this code's E_max and
/// E_sel128 for the explicit immutable OriginalK6bPair. Actual runner/binary
/// contexts rebind their paths and use one newly composed result per process.
#[test]
fn the_committed_counts_carry_this_codes_estimate() {
    let mut lines = 0;
    for line in COUNTS.lines().filter(|l| !l.trim().is_empty()) {
        let (id, counts, _) = parse_counts_line(line).expect("a counts line");
        let w1 = counts.w1.expect("W1 counts");
        let e = assert_estimate_is_hand(&w1, &id).legacy;
        assert_eq!(e.max, number(line, "estimate_adm_bytes_w1a"), "{id}");
        assert_eq!(e.sel128, number(line, "estimate_w1_sel128_bytes"), "{id}");
        assert_eq!(e.fixed, number(line, "estimate_w1_fixed_bytes"), "{id}");
        assert_eq!(e.decide, number(line, "estimate_w1_decide_bytes"), "{id}");
        for (name, values) in [("shared", e.shared), ("state", e.state), ("solve", e.solve)] {
            for (p, value) in [128, 256, 512, 1024].into_iter().zip(values) {
                assert_eq!(
                    value,
                    number(line, &format!("estimate_w1_{name}_{p}")),
                    "{id}"
                );
            }
        }
        for (name, values) in [("verify", e.verify), ("pass", e.pass)] {
            for (p, value) in [256, 512, 1024].into_iter().zip(values) {
                assert_eq!(
                    value,
                    number(line, &format!("estimate_w1_{name}_{p}")),
                    "{id}"
                );
            }
        }
        lines += 1;
    }
    assert_eq!(lines, 33);
}

/// RV22-3: at 1,000 and 10,000 members (from the committed counts; nothing is
/// solved), the solve-phase terms set E_sel128, and at 10,000 members a solve
/// or pass term sets E_max; the estimate equals the hand derivation there.
#[test]
fn the_solve_and_pass_terms_bind_on_the_large_models() {
    for line in COUNTS.lines().filter(|l| !l.trim().is_empty()) {
        let (id, counts, _) = parse_counts_line(line).expect("a counts line");
        if !(id.contains("-n01000-") || id.contains("-n10000-")) {
            continue;
        }
        let h = assert_estimate_is_hand(&counts.w1.expect("W1 counts"), &id);
        assert_eq!(
            h.selected128_dominant_kernel,
            Some(PhaseId {
                phase: KernelPhase::Fallback,
                precision: Some(256)
            }),
            "{id}"
        );
        if id.contains("-n10000-") {
            assert!(
                matches!(
                    h.dominant_kernel.unwrap().phase,
                    KernelPhase::Fallback | KernelPhase::Shift
                ),
                "{id}: {}",
                format!("{:?}", h.dominant_kernel)
            );
        }
    }
}
