//! T3 KF3 tests (mounted in `adaptive.rs`, so they reach the builds and the
//! schedule):
//! - D1 revision 5a.3 amendment A2 (ROOT's ruling on I19's plan): a Uc_c or
//!   S_c whose formation is refused is unavailable for its block, B_c is the
//!   minimum over the bounds formed, and a block with data left with no bound
//!   after a refusal stops the attempt as before. The constructed controls of
//!   `kf3.txt` (GEN's `kf3_models`): KF3-UC-SPAN moves from
//!   `Unresolved(ExactSumSpan)` (main) to selected, honest against GEN's
//!   high-precision expectations with G5a passing; KF3-UC-SPAN-ZERO's refused
//!   block carries no data and needs no bound; with S made unavailable (the
//!   test-only hook) KF3-UC-SPAN stops with the refusal, as before A2.
//! - Partial stage work: every build that records stages (`build_shared`,
//!   `solve_case_at`, `build_verify_shared`, `verify_state`) adds the work it
//!   was charged beyond its recorded stages to the stage in progress when it
//!   stops, so its stages sum to its charged total on every path; and every
//!   attempt's stages sum to its charged totals (K6b's parity item then holds
//!   with equality everywhere).
use super::super::bound::{BlockRefusal, CertifiedBound};
use super::super::verify::{build_verify_shared, hooks, verify_state};
use super::*;
use std::time::Instant;

use super::method_tests::models;
#[allow(dead_code)]
#[path = "support.rs"]
mod support;

const KF3: &str = include_str!("kf3.txt");
const SUMS: &str = include_str!("SHA256SUMS");

fn kf3_model(name: &str) -> models::Model {
    models::parse_models(KF3)
        .into_iter()
        .find(|m| m.name == name)
        .unwrap_or_else(|| panic!("{name}"))
}

/// The lines of `kf3.txt` of the given record kinds (first fields).
fn section(kinds: &[&str]) -> String {
    let mut text: String = KF3
        .lines()
        .filter(|l| {
            kinds
                .iter()
                .any(|k| l.split_whitespace().next() == Some(*k))
        })
        .collect::<Vec<_>>()
        .join("\n");
    text.push('\n');
    text
}

fn gen_outcome(name: &str) -> (String, Vec<String>) {
    let line = KF3
        .lines()
        .find(|l| l.starts_with(&format!("outcome {name} ")))
        .unwrap_or_else(|| panic!("{name}: no GEN outcome"));
    let f: Vec<&str> = line.split_whitespace().collect();
    (
        f[2].to_string(),
        f[3..].iter().map(|t| t.to_string()).collect(),
    )
}

/// GEN's refusal token of block 0 at 256 (`blk` record, U's column).
fn gen_refusal(name: &str) -> String {
    let line = KF3
        .lines()
        .find(|l| l.starts_with(&format!("blk {name} 256 0 ")))
        .unwrap_or_else(|| panic!("{name}: no blk record"));
    line.split_whitespace().nth(7).unwrap().to_string()
}

/// The charged totals of an attempt: its own contexts and sums, and the
/// shared work counted against its case (K6b's `own_total` and
/// `shared_total`).
fn own_total(a: &AttemptRecord) -> u64 {
    a.work.limb_multiply_equivalents() + a.k4_work.limb_multiply_equivalents()
}

/// Every attempt's stages sum to its charged totals, on every path.
pub(super) fn assert_stage_identity(name: &str, attempts: &[AttemptRecord]) {
    for a in attempts {
        assert_eq!(
            a.stages.total(),
            own_total(a),
            "{name} {}: own stages {:?}",
            a.precision,
            a.outcome
        );
        assert_eq!(
            a.shared_stages.total(),
            a.shared_work + a.verification_shared_work,
            "{name} {}: shared stages {:?}",
            a.precision,
            a.outcome
        );
    }
}

fn solve(m: &models::Model) -> CaseOutcome {
    let mut meter = InvocationMeter::new(u64::MAX);
    solve_case(m.source(), CaseLimit::new(u64::MAX), &mut meter)
}

fn layout(m: &models::Model) -> Vec<QuantityMeta> {
    CasePrep::new(m.source()).unwrap().layout
}

/// The test-only hook that removes S_c, cleared on drop.
struct NoShift;

impl NoShift {
    fn new() -> Self {
        hooks::set_no_shift(true);
        NoShift
    }
}

impl Drop for NoShift {
    fn drop(&mut self) {
        hooks::set_no_shift(false);
    }
}

// ---------------------------------------------------------------- the vectors

#[test]
fn kf3_txt_is_the_pinned_file() {
    let line = SUMS
        .lines()
        .find(|l| l.ends_with("  kf3.txt"))
        .expect("kf3.txt in SHA256SUMS");
    assert_eq!(
        line.split_whitespace().next().unwrap(),
        support::sha256_hex(KF3.as_bytes())
    );
}

#[test]
fn kf3_models_equal_the_generators_emulation_at_128_and_256() {
    // E-UNIT at 128 and 256, E-UC and E-CHARGE at 256 (the precisions the
    // schedule uses), bit for bit against GEN.
    let scale = section(&["scale", "g", "abar", "erows", "erow", "ebody"]);
    let bounds = section(&["bnd", "blk", "shf", "shiftcount"]);
    let charge = section(&["chg", "chgrows", "chgblk", "chgbody", "chgcount"]);
    for name in ["KF3-UC-SPAN", "KF3-UC-SPAN-ZERO"] {
        let m = kf3_model(name);
        let prep = CasePrep::new(m.source()).unwrap();
        let group = prepare_group(&prep.source).unwrap();
        let recs = super::scale_tests::records(&scale, name, 128);
        super::scale_tests::scale_at::<4, 4>(128, 192, &prep, &group, &recs);
        let recs = super::scale_tests::records(&scale, name, 256);
        super::scale_tests::scale_at::<4, 8>(256, 320, &prep, &group, &recs);
        let recs = super::scale_tests::records(&bounds, name, 256);
        let (_, shifted) = super::scale_tests::bounds_at::<4, 8>(256, 320, &prep, &group, &recs);
        assert_eq!(shifted, 1, "{name}: S_c formed");
        let f = super::method_tests::verification_in::<4, 8, 8>(
            &charge, "", 256, 320, &m, &prep, &group,
        )
        .unwrap_or_else(|| panic!("{name}: a stop"));
        println!("FIG {name} 256 {f:?}");
        if name == "KF3-UC-SPAN" {
            // B_c = S_c: the shift ran once, its first σ succeeded.
            assert_eq!((f.shifts, f.tries.clone()), (1, vec![1]), "{name}");
            // Measured at A: θ 8.0e-55, C/allowance 2.6e-37, Ŵ/V 1.4e-2.
            assert!(
                f.theta < 1e-50 && f.charge < 1e-30 && f.estimate < 0.25,
                "{name}: {f:?}"
            );
        }
    }
}

// ---------------------------------------------------------------- A2 at W1 level

#[test]
fn kf3_uc_span_moves_from_a_stop_to_selected_honestly() {
    let m = kf3_model("KF3-UC-SPAN");
    let started = Instant::now();
    let out = solve(&m);
    let elapsed = started.elapsed().as_secs_f64();
    println!("KF3-UC-SPAN: debug wall time {elapsed:.1} s");
    let CaseOutcome::Selected(s) = out else {
        panic!("KF3-UC-SPAN: {out:?}")
    };
    let attempts = &s.evidence().attempts;
    let (sel, toks) = gen_outcome("KF3-UC-SPAN");
    assert_eq!(
        (
            s.selected_precision().to_string(),
            super::method_tests::tokens(attempts, &layout(&m))
        ),
        (sel, toks)
    );
    // The verification's Uc_c was refused (so main stops this case with
    // `Unresolved(ExactSumSpan)`), and recorded on the verification attempt
    // only; B = S_c.
    let v = attempts.iter().find(|a| a.precision == 256).unwrap();
    assert_eq!(v.bound_refusals.len(), 1, "{:?}", v.bound_refusals);
    let r: &BlockRefusal = &v.bound_refusals[0];
    assert_eq!((r.block, r.bound), (0, CertifiedBound::Uc));
    assert_eq!(
        super::scale_tests::refusal_token(&r.refusal),
        gen_refusal("KF3-UC-SPAN")
    );
    assert!(attempts
        .iter()
        .filter(|a| a.precision != 256)
        .all(|a| a.bound_refusals.is_empty()));
    let summary = v.verification.as_ref().unwrap();
    assert_eq!(
        (summary.uc_missing, summary.shift_factorizations),
        (None, 1)
    );
    // Honest against GEN's high-precision solves, and G5a.
    models::g5a(&s).unwrap_or_else(|e| panic!("KF3-UC-SPAN: G5a {e}"));
    let (worst, at, compared) =
        models::compare_honest(s.publish(), s.selected_precision(), &m.exact);
    println!("KF3-UC-SPAN: worst {worst:e} at {at}, {compared} compared");
    assert!(
        compared >= s.publish().rows.len() && worst <= 1.0,
        "{worst} at {at}"
    );
    assert_stage_identity("KF3-UC-SPAN", attempts);
}

#[test]
fn kf3_a_refused_block_without_data_needs_no_bound() {
    let m = kf3_model("KF3-UC-SPAN-ZERO");
    let started = Instant::now();
    let out = solve(&m);
    println!(
        "KF3-UC-SPAN-ZERO: debug wall time {:.1} s",
        started.elapsed().as_secs_f64()
    );
    let CaseOutcome::Selected(s) = out else {
        panic!("KF3-UC-SPAN-ZERO: {out:?}")
    };
    let attempts = &s.evidence().attempts;
    let (sel, toks) = gen_outcome("KF3-UC-SPAN-ZERO");
    assert_eq!(
        (
            s.selected_precision().to_string(),
            super::method_tests::tokens(attempts, &layout(&m))
        ),
        (sel, toks)
    );
    let v = attempts.iter().find(|a| a.precision == 256).unwrap();
    assert_eq!(
        v.bound_refusals
            .iter()
            .map(|r| super::scale_tests::refusal_token(&r.refusal))
            .collect::<Vec<_>>(),
        vec![gen_refusal("KF3-UC-SPAN-ZERO")]
    );
    let summary = v.verification.as_ref().unwrap();
    assert_eq!((summary.data_blocks, summary.uc_missing), (0, None));
    models::g5a(&s).unwrap_or_else(|e| panic!("KF3-UC-SPAN-ZERO: G5a {e}"));
    let (worst, at, _) = models::compare_honest(s.publish(), s.selected_precision(), &m.exact);
    assert!(worst <= 1.0, "{worst} at {at}");
    assert_stage_identity("KF3-UC-SPAN-ZERO", attempts);
}

#[test]
fn kf3_with_neither_bound_the_attempt_stops_with_the_refusal_as_before() {
    // W2 (ROOT's ruling 1 on I19's plan): the hook sets every est_c to 0, so no
    // shift runs and the refused block with data has no bound.
    let m = kf3_model("KF3-UC-SPAN");
    let out = {
        let _hook = NoShift::new();
        solve(&m)
    };
    let CaseOutcome::Unresolved {
        reason, attempts, ..
    } = out
    else {
        panic!("{out:?}")
    };
    assert_eq!(reason, UnresolvedReason::ExactSumSpan);
    assert_eq!(
        super::method_tests::tokens(&attempts, &layout(&m)),
        vec!["128:rejected:verification_failed", "256:failed:Span"]
    );
    let v = &attempts[1];
    assert_eq!(
        v.outcome,
        AttemptOutcome::Failed(AttemptReason::Stop(AttemptStop::Span))
    );
    assert_eq!(
        v.bound_refusals
            .iter()
            .map(|r| (
                r.block,
                r.bound,
                super::scale_tests::refusal_token(&r.refusal)
            ))
            .collect::<Vec<_>>(),
        vec![(0, CertifiedBound::Uc, gen_refusal("KF3-UC-SPAN"))]
    );
    // The pass ran through E, the estimate and the charge's norms to its bound
    // stage and stopped there (no shift: est_c = 0); its stages hold its
    // charged work.
    assert!(v.stages.scale > 0 && v.stages.estimate > 0 && v.stages.charge > 0);
    assert_eq!(v.stages.shift, 0);
    assert_stage_identity("KF3-UC-SPAN (no shift)", &attempts);
}

// ---------------------------------------------------------------- partial stage work

/// A build's stages against its charged total, unlimited and with the case
/// room ending at every stage boundary, inside every stage and at 32 even
/// points (every stop lands in some stage).
fn sweep<F>(label: &str, run: F) -> usize
where
    F: Fn(StageGuard) -> (bool, StageWork, u64),
{
    let (ok, stages, total) = run(StageGuard::unlimited());
    assert_eq!(stages.total(), total, "{label}: unlimited ({ok})");
    let parts = [
        stages.formation,
        stages.assembly,
        stages.residual_formation,
        stages.factor,
        stages.condition,
        stages.rhs,
        stages.solve,
        stages.refinement,
        stages.bounded_gate,
        stages.recovery,
        stages.scale,
        stages.estimate,
        stages.charge,
        stages.bound,
        stages.shift,
        stages.bounded_formation,
        stages.wide_formation,
        stages.uc,
    ];
    let mut limits: Vec<u64> = (1..32).map(|k| total / 32 * k).collect();
    let mut at = 0u64;
    for w in parts {
        if w > 0 {
            limits.push(at + w / 2);
            limits.push(at + w);
            at += w;
        }
    }
    let mut stopped = 0;
    for limit in limits {
        let (ok, stages, total) = run(StageGuard::with_case_room(limit));
        assert_eq!(stages.total(), total, "{label}: room {limit} ({ok})");
        stopped += usize::from(!ok);
    }
    stopped
}

fn shared_sweep<const L: usize, const R: usize>(
    name: &str,
    p: u32,
    q: u32,
    m: &models::Model,
) -> usize
where
    Wide<L>: SupportedWidth,
    Wide<R>: SupportedWidth,
{
    let prep = CasePrep::new(m.source()).unwrap();
    let group = prepare_group(&prep.source).unwrap();
    sweep(&format!("{name} {p} build_shared"), |g| {
        let s = build_shared::<L, R>(p, q, &prep.source, &group, g);
        (s.result.is_ok(), s.stages, s.total)
    })
}

fn solve_sweep<const L: usize, const R: usize>(
    name: &str,
    p: u32,
    q: u32,
    m: &models::Model,
) -> usize
where
    Wide<L>: SupportedWidth,
    Wide<R>: SupportedWidth,
{
    let prep = CasePrep::new(m.source()).unwrap();
    let group = prepare_group(&prep.source).unwrap();
    let shared = build_shared::<L, R>(p, q, &prep.source, &group, StageGuard::unlimited())
        .result
        .unwrap();
    sweep(&format!("{name} {p} solve_case_at"), |g| {
        let s = solve_case_at::<L, R>(&shared, &prep, &group, g);
        (s.result.is_ok(), s.stages, s.total)
    })
}

/// The verification's shared build and pass at 256 (verifying 128).
fn verify_sweep(name: &str, m: &models::Model) -> (usize, usize) {
    let prep = CasePrep::new(m.source()).unwrap();
    let group = prepare_group(&prep.source).unwrap();
    let g = StageGuard::unlimited();
    let shared = build_shared::<4, 8>(256, 320, &prep.source, &group, g)
        .result
        .unwrap();
    let state = solve_case_at::<4, 8>(&shared, &prep, &group, g)
        .result
        .unwrap();
    let a = sweep(&format!("{name} 256 build_verify_shared"), |g| {
        let s = build_verify_shared::<4, 8, 8>(&shared, &prep.source, &group, g);
        (s.result.is_ok(), s.stages, s.total)
    });
    let vs = build_verify_shared::<4, 8, 8>(&shared, &prep.source, &group, g)
        .result
        .unwrap();
    let b = sweep(&format!("{name} 256 verify_state"), |g| {
        let s = verify_state::<4, 8, 8>(&shared, &vs, &prep, &group, &state, g);
        (s.result.is_ok(), s.stages, s.total)
    });
    (a, b)
}

#[test]
fn kf3_every_build_stages_its_charged_work_on_every_path() {
    // Budget stops inside every stage of every build, on N05 and TWO-SPAN.
    for name in ["N05", "TWO-SPAN"] {
        let m = models::model(name);
        assert!(shared_sweep::<4, 4>(name, 128, 192, &m) > 0, "{name}");
        assert!(solve_sweep::<4, 4>(name, 128, 192, &m) > 0, "{name}");
        let (a, b) = verify_sweep(name, &m);
        assert!(a > 0 && b > 0, "{name}");
    }
    // The escalating stops: `Condition` in the screen (SKEW-K1E-28 at 128)
    // and `Pivot` in the factor (PIVOT at 128).
    for (name, stop) in [("SKEW-K1E-28", "Condition"), ("PIVOT", "Pivot")] {
        let m = models::model(name);
        let prep = CasePrep::new(m.source()).unwrap();
        let group = prepare_group(&prep.source).unwrap();
        let s = build_shared::<4, 4>(128, 192, &prep.source, &group, StageGuard::unlimited());
        let err = s.result.as_ref().expect_err(name);
        assert!(format!("{err:?}").starts_with(stop), "{name}: {err:?}");
        assert_eq!(s.stages.total(), s.total, "{name}");
        let partial = if stop == "Pivot" {
            s.stages.factor
        } else {
            s.stages.condition
        };
        assert!(
            partial > 0,
            "{name}: the stopped stage holds its partial work"
        );
    }
    // `ResidualGate` after the fallback: N09-B's residual basis perturbed
    // (adaptive_tests' `perturbed_solve`, e = 28).
    let m = models::model("N09-B");
    let prep = CasePrep::new(m.source()).unwrap();
    let group = prepare_group(&prep.source).unwrap();
    let mut shared = build_shared::<4, 4>(128, 192, &prep.source, &group, StageGuard::unlimited())
        .result
        .unwrap();
    let mut c = WideContext::<4>::new(192).unwrap();
    for (r, col, index) in group.structure.entries() {
        if r == col {
            let v = shared.k_q[index];
            let delta = v.mul_pow2(-28).unwrap();
            shared.k_q[index] = c.add(&v, &delta).unwrap();
        }
    }
    let s = solve_case_at::<4, 4>(&shared, &prep, &group, StageGuard::unlimited());
    assert!(matches!(s.result, Err(AttemptStop::ResidualGate { .. })));
    assert_eq!(s.stages.total(), s.total);
    assert!(s.stages.refinement > 0 && s.stages.bounded_gate > 0);
    // `ResolutionScale` in the pass (EHAT-OVERFLOW at 256).
    let all5a3 = models::parse_models(include_str!("models5a3.txt"));
    let m = all5a3.iter().find(|m| m.name == "EHAT-OVERFLOW").unwrap();
    let prep = CasePrep::new(m.source()).unwrap();
    let group = prepare_group(&prep.source).unwrap();
    let g = StageGuard::unlimited();
    let shared = build_shared::<4, 8>(256, 320, &prep.source, &group, g)
        .result
        .unwrap();
    let state = solve_case_at::<4, 8>(&shared, &prep, &group, g)
        .result
        .unwrap();
    let vs = build_verify_shared::<4, 8, 8>(&shared, &prep.source, &group, g)
        .result
        .unwrap();
    let s = verify_state::<4, 8, 8>(&shared, &vs, &prep, &group, &state, g);
    assert!(matches!(s.result, Err(AttemptStop::ResolutionScale { .. })));
    assert_eq!(s.stages.total(), s.total);
    assert!(s.stages.scale > 0);
}

#[test]
fn kf3_a_budget_stop_inside_uc_stages_the_partial_uc_work() {
    // K6b's case (ROOT's ruling on the K6B-S3 stop), on the KF3 model whose
    // `uc` stage ends in a refusal: a case room ending halfway into `uc`.
    let m = kf3_model("KF3-UC-SPAN");
    let prep = CasePrep::new(m.source()).unwrap();
    let group = prepare_group(&prep.source).unwrap();
    let g = StageGuard::unlimited();
    let shared = build_shared::<4, 8>(256, 320, &prep.source, &group, g)
        .result
        .unwrap();
    let full = build_verify_shared::<4, 8, 8>(&shared, &prep.source, &group, g);
    let vs = full.result.as_ref().unwrap();
    assert!(vs.uc[0].refused.is_some() && full.stages.uc > 0);
    assert_eq!(full.stages.total(), full.total);
    let room = full.stages.bounded_formation + full.stages.wide_formation + full.stages.uc / 2;
    let s = build_verify_shared::<4, 8, 8>(
        &shared,
        &prep.source,
        &group,
        StageGuard::with_case_room(room),
    );
    assert!(matches!(
        s.result,
        Err(AttemptStop::Budget(BudgetScope::Case))
    ));
    assert_eq!(s.stages.total(), s.total);
    assert!(s.stages.uc > 0 && s.stages.uc < full.stages.uc);
}

// ---------------------------------------------------------------- RV23-1 and RV23-N1

/// The case work charged before the 256 verification's shared build: 128's
/// and 256's shared builds and own solves (H's `segments`, K6b; RV23's probe).
fn before_verify_shared_256(attempts: &[AttemptRecord]) -> u64 {
    attempts[..2]
        .iter()
        .map(|a| a.shared_work + own_total(a) - a.verification_work - a.stop_rule_work)
        .sum()
}

#[test]
fn kf3_a_budget_stop_after_the_uc_refusal_keeps_it_in_the_evidence() {
    // RV23-1 (ROOT's ruling on RV23's review), the verification's shared
    // build, as RV23's probe: the case limit leaves the 256 verification's
    // shared build 1 or 100 LME short, so the build does all its work, the
    // Uc_c refusal included, and stops at a later guard check.
    let m = kf3_model("KF3-UC-SPAN");
    let out = solve(&m);
    let CaseOutcome::Selected(s) = out else {
        panic!("KF3-UC-SPAN: {out:?}")
    };
    let full = &s.evidence().attempts;
    let v = &full[1];
    assert_eq!((v.precision, v.bound_refusals.len()), (256, 1));
    let start = before_verify_shared_256(full);
    for delta in [1u64, 100] {
        let limit = start + v.verification_shared_work - delta;
        let mut meter = InvocationMeter::new(u64::MAX);
        let out = solve_case(m.source(), CaseLimit::new(limit), &mut meter);
        let CaseOutcome::Unresolved { attempts, .. } = out else {
            panic!("{delta}: {out:?}")
        };
        let a = &attempts[1];
        assert_eq!(
            a.outcome,
            AttemptOutcome::Failed(AttemptReason::Stop(AttemptStop::Budget(BudgetScope::Case))),
            "{delta}"
        );
        // The build did all its work, the refusal included ...
        assert_eq!(
            (a.verification_shared_work, a.shared_stages.uc),
            (v.verification_shared_work, v.shared_stages.uc),
            "{delta}"
        );
        // ... and its refusal reaches the evidence, as on the completed build.
        assert_eq!(a.bound_refusals, v.bound_refusals, "{delta}");
        assert_stage_identity(&format!("KF3-UC-SPAN, {delta} LME short"), &attempts);
    }
}

/// KF3-UC-SPAN with a second body: one member along x, fixed at its first
/// node and loaded at its second (its own block, with data).
fn kf3_with_a_second_body() -> models::Model {
    let text: String = KF3
        .lines()
        .skip_while(|l| *l != "model KF3-UC-SPAN")
        .take_while(|l| *l != "end")
        .filter(|l| !l.starts_with("expect "))
        .map(|l| format!("{l}\n"))
        .collect();
    let member = text.lines().find(|l| l.starts_with("member 1 ")).unwrap();
    let section: Vec<&str> = member.split_whitespace().skip(4).take(6).collect();
    let hex = |x: f64| format!("{:016x}", x.to_bits());
    let mut extra = format!(
        "node {} {} {}\nnode {} {} {}\nmember 391 391 392 {} {} {} {}\n",
        hex(1000.0),
        hex(0.0),
        hex(0.0),
        hex(1017.0),
        hex(0.0),
        hex(0.0),
        section.join(" "),
        hex(0.0),
        hex(1.0),
        hex(0.0)
    );
    for c in 0..6 {
        extra.push_str(&format!("constraint 391 {c} {}\n", hex(0.0)));
    }
    extra.push_str(&format!("load 392 1 {} l3\n", hex(0.0625)));
    models::parse_models(&format!("{text}{extra}end\n"))
        .into_iter()
        .next()
        .unwrap()
}

#[test]
fn kf3_in_verify_state_a_refusal_stop_outranks_uc() {
    // RV23-N1 (ROOT's ruling on RV23's review): 7d's precedence as
    // `verify_state` composes it (ROOT's rulings 1 and 2 on I19's plan). Two
    // blocks with data and no S (the hook: every est_c 0). The chain's Uc_c is
    // refused; the second body's is made missing, t_c ≥ 1 with no refusal
    // (7b's `uc` case). The refusal stops the pass.
    let m = kf3_with_a_second_body();
    let prep = CasePrep::new(m.source()).unwrap();
    let group = prepare_group(&prep.source).unwrap();
    assert_eq!(group.blocks.len(), 2);
    let g = StageGuard::unlimited();
    let shared = build_shared::<4, 8>(256, 320, &prep.source, &group, g)
        .result
        .unwrap();
    let state = solve_case_at::<4, 8>(&shared, &prep, &group, g)
        .result
        .unwrap();
    let mut vs = build_verify_shared::<4, 8, 8>(&shared, &prep.source, &group, g)
        .result
        .unwrap();
    let chain = vs
        .uc
        .iter()
        .position(|b| b.refused.is_some())
        .expect("the chain's Uc_c is refused");
    let other = 1 - chain;
    assert!(vs.uc[other].refused.is_none() && vs.uc[other].uc.is_some());
    let refusal = vs.uc[chain].refused.unwrap();
    vs.uc[other].t = Wide::<4>::ONE;
    vs.uc[other].uc = None;
    let _hook = NoShift::new();
    let r = verify_state::<4, 8, 8>(&shared, &vs, &prep, &group, &state, g);
    assert_eq!(r.result.as_ref().err(), Some(&refusal.stop()));
    // Control: with the chain's bound formed instead (any positive value), the
    // same pass reports `uc` on the second body, so both branches are live.
    vs.uc[chain].refused = None;
    vs.uc[chain].uc = Some(Wide::<4>::ONE.mul_pow2(64).unwrap());
    let r = verify_state::<4, 8, 8>(&shared, &vs, &prep, &group, &state, g);
    assert_eq!(
        r.result.as_ref().map(|rep| rep.uc_missing).ok(),
        Some(Some(other))
    );
}
