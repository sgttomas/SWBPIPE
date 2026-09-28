//! K4 tests of `retained/adaptive.rs`: the schedule and the stop rule (the
//! brief's G), budgets, work and determinism (L), and the bounded refinement
//! (F's cap, through the schedule's internals).
//!
//! S\*-dependent assertions (to be updated mechanically if D1 revision 5a.3
//! changes S\*): `SD-G1` SKEW6-K1E-12 is rejected at 128 by the stop rule;
//! `SD-G2` ZERO-TORSION-345's structural-zero forces are accepted at 128
//! through the coupled S\*; `SD-G3` ALL-ZERO-BODY's all-zero body agrees
//! exactly (S\* = 0) at 128; `SD-G4` REACTIONS-ONLY is rejected at 128 by a
//! reaction; `SD-G5` the predicate's boundary tests (S\* from the verification
//! values); `SD-G6` the every-kind stop-rule control; `SD-L1` the golden work
//! counts (the stop rule's work depends on S\*'s form).
use super::super::recover::QuantityId;
use super::super::source::{Component, Dof};
use super::*;

#[allow(dead_code)]
#[path = "models.rs"]
mod models;
#[allow(dead_code)]
#[path = "support.rs"]
mod support;

fn unlimited() -> (CaseLimit, InvocationMeter) {
    (CaseLimit::new(u64::MAX), InvocationMeter::new(u64::MAX))
}

fn outcome(name: &str) -> CaseOutcome {
    let (limit, mut meter) = unlimited();
    solve_case(models::model(name).source(), limit, &mut meter)
}

fn selected(name: &str) -> Box<RetainedSolve> {
    match outcome(name) {
        CaseOutcome::Selected(s) => s,
        other => panic!("{name}: {other:?}"),
    }
}

fn plan(solve: &RetainedSolve) -> Vec<(u32, AttemptRole, AttemptOutcome)> {
    solve
        .evidence()
        .attempts
        .iter()
        .map(|a| (a.precision, a.role, a.outcome.clone()))
        .collect()
}

fn within(name: &str) -> Box<RetainedSolve> {
    let m = models::model(name);
    let solve = selected(name);
    let (worst, at, compared) = models::compare(&m.source(), &solve.publish().rows, &m.expect);
    assert!(
        compared >= 30 && worst <= 1.0,
        "{name}: {worst} at {at} ({compared})"
    );
    solve
}

use AttemptOutcome::{Accepted, Rejected, Verified};
use AttemptRole::{Candidate, Verification, VerificationThenCandidate};

// ---------------------------------------------------------------- G: the schedule

#[test]
fn n05_and_n06_are_accepted_at_128_against_256() {
    for name in ["N05", "N06"] {
        let solve = within(name);
        assert_eq!(
            plan(&solve),
            vec![(128, Candidate, Accepted), (256, Verification, Verified)],
            "{name}"
        );
        assert_eq!(solve.evidence().verification_precision, 256);
    }
}

#[test]
fn k_1e_minus_28_escalates_past_128_and_is_accepted_at_256_against_512() {
    // The plan expected a stop-rule rejection at 128; K4's condition screen
    // stops the 128 attempt first (rcond ≤ 2^-127), which also escalates.
    let solve = within("SKEW-K1E-28");
    assert_eq!(
        plan(&solve),
        vec![
            (
                128,
                Candidate,
                AttemptOutcome::Failed(AttemptReason::Stop(AttemptStop::Condition))
            ),
            (256, Candidate, Accepted),
            (512, Verification, Verified),
        ]
    );
}

#[test]
fn the_six_member_skew_run_is_rejected_at_128_and_its_verification_is_reused() {
    // SD-G1. The 256 verification of the rejected 128 candidate becomes the
    // next candidate (no second 256 solve): at most four solves in all.
    let solve = within("SKEW6-K1E-12");
    let attempts = plan(&solve);
    assert!(matches!(
        attempts[0],
        (128, Candidate, Rejected(AttemptReason::StopRule { .. }))
    ));
    assert_eq!(
        attempts[1..],
        [
            (256, VerificationThenCandidate, Accepted),
            (512, Verification, Verified)
        ]
    );
    let four = selected("SKEW-K1E-60");
    assert_eq!(four.evidence().attempts.len(), 4);
    assert_eq!(
        four.evidence()
            .attempts
            .iter()
            .map(|a| a.precision)
            .collect::<Vec<_>>(),
        vec![128, 256, 512, 1024]
    );
    assert_eq!(four.evidence().attempts[2].role, VerificationThenCandidate);
}

#[test]
fn a_structural_zero_kind_is_accepted_through_the_coupled_scale() {
    // SD-G2: every force of ZERO-TORSION-345 is an exact zero in truth; the
    // coupled S*(force) = S*(moment)/L_b keeps the rule finite.
    let solve = within("ZERO-TORSION-345");
    assert_eq!(solve.selected_precision(), 128);
}

#[test]
fn an_all_zero_body_agrees_exactly() {
    // SD-G3: body 1 is unloaded and unmoved: every value is an exact +0.0 at
    // 128 and 256, S* = 0, and its rows are AbsoluteVerified with bound 0.
    let solve = within("ALL-ZERO-BODY");
    assert_eq!(solve.selected_precision(), 128);
    let zero_body: Vec<&PublishedRow> = solve
        .publish()
        .rows
        .iter()
        .filter(|r| r.body == 1)
        .collect();
    assert!(zero_body.len() > 20);
    for r in zero_body {
        assert_eq!(r.value.value().map(f64::to_bits), Some(0), "{:?}", r.id);
        if r.class != RowClass::InputDerived {
            assert_eq!(r.class, RowClass::AbsoluteVerified { bound_bits: 0 });
        }
    }
    assert!(solve
        .evidence()
        .stop_rule
        .iter()
        .filter(|s| s.0 == 1)
        .all(|s| s.2 == 0.0));
}

#[test]
fn the_reactions_only_control_is_rejected_at_128_by_a_reaction() {
    // SD-G4 (K4-M16's control): only a reaction disagrees at 128.
    let solve = within("REACTIONS-ONLY");
    let attempts = plan(&solve);
    assert!(matches!(
        attempts[0],
        (
            128,
            Candidate,
            Rejected(AttemptReason::StopRule {
                quantity: QuantityId::Reaction(_),
                kind: Kind::Force,
                ..
            })
        )
    ));
    assert_eq!(solve.selected_precision(), 256);
}

#[test]
fn a_ceiling_case_is_unresolved_with_its_attempts() {
    match outcome("SKEW-K1E-300") {
        CaseOutcome::Unresolved {
            reason: UnresolvedReason::Ceiling,
            attempts,
            geometry,
        } => {
            assert_eq!(geometry, vec![BodyGeometry::Restrained]);
            assert_eq!(
                attempts.iter().map(|a| a.precision).collect::<Vec<_>>(),
                vec![128, 256, 512]
            );
            for a in &attempts {
                assert!(matches!(
                    a.outcome,
                    AttemptOutcome::Failed(AttemptReason::Stop(AttemptStop::Pivot { .. }))
                ));
                assert!(a.shared_work > 0);
            }
        }
        other => panic!("{other:?}"),
    }
}

// ---------------------------------------------------------------- G: the predicate, exactly

fn one_quantity() -> Vec<QuantityMeta> {
    vec![QuantityMeta {
        id: QuantityId::Displacement(Dof {
            node: 0,
            component: Component::Ux,
        }),
        kind: Kind::Translation,
        body: 0,
        input_derived: false,
    }]
}

fn decide(candidate: &[Wide<4>], verification: &[Wide<4>], layout: &[QuantityMeta]) -> bool {
    stop_rule(
        layout,
        &[0.0],
        candidate,
        verification,
        256,
        StageGuard::unlimited(),
    )
    .result
    .unwrap()
}

#[test]
fn the_predicate_is_decided_exactly_at_its_boundary() {
    // SD-G5. q_2p = 1 is the only quantity, so S* = 1 and the bound is 2^-64.
    let layout = one_quantity();
    let w = |neg: bool, e: i64, mag: &[u64]| support::w_value::<4>(neg, e, mag);
    let one = support::lift::<4>(1.0);
    // 1 ± 2^-64 are accepted; one 256-bit ulp further is not.
    let above = w(false, 0, &[1, 1]); // 1 + 2^-64
    let above_ulp = w(false, 0, &[1, 0, 1 << 63, 1 << 63]); // 1 + 2^-64 + 2^-255
    let below = w(false, -1, &[u64::MAX]); // 1 − 2^-64
    let below_ulp = w(false, -1, &[u64::MAX, u64::MAX, u64::MAX, u64::MAX - 1]); // 1 − 2^-64 − 2^-256
    assert!(decide(&[above], &[one], &layout));
    assert!(!decide(&[above_ulp], &[one], &layout));
    assert!(decide(&[below], &[one], &layout));
    assert!(!decide(&[below_ulp], &[one], &layout));
    // K4-M14: the magnitude is |q_2p|, never |q_p|. q_2p = 1 and
    // q_p = 1 + 2^-64 + 2^-128: |q_p − q_2p| exceeds 2^-64·1 but not
    // 2^-64·|q_p|, so only the correct rule rejects it.
    let candidate = w(false, 0, &[1, 1, 1]);
    assert!(!decide(&[candidate], &[one], &layout));
}

#[test]
fn every_published_quantity_takes_part_in_the_stop_rule() {
    // SD-G6. N06's 128 and 256 states agree; perturbing any one published
    // quantity of the 128 state by 2^-60 of its body's S* makes the rule
    // reject at exactly that quantity, for every quantity of every kind.
    let solve = selected("N06");
    let layout = &solve.prep.layout;
    let (Some(PrecisionState::P128(a)), Some(PrecisionState::P256(b))) =
        (solve.state(128), solve.state(256))
    else {
        panic!()
    };
    let extents = &solve.prep.extents;
    let base = stop_rule(
        layout,
        extents,
        &a.recovered.values,
        &b.recovered.values,
        256,
        StageGuard::unlimited(),
    );
    assert_eq!(base.result, Ok(true));
    let mut c = WideContext::<4>::new(256).unwrap();
    let scales = scales_at(&mut c, layout, &b.recovered.values, extents).unwrap();
    let mut types = std::collections::HashSet::new();
    let mut kinds = std::collections::BTreeSet::new();
    for (index, meta) in layout.iter().enumerate() {
        let s = scales[meta.body as usize][meta.kind.index()];
        let magnitude = if b.recovered.values[index].abs().cmp_value(&s) == CmpOrdering::Greater {
            b.recovered.values[index].abs()
        } else {
            s
        };
        if magnitude.is_zero() {
            continue;
        }
        let mut perturbed = a.recovered.values.clone();
        let delta: Wide<4> = magnitude.mul_pow2(-60).unwrap();
        perturbed[index] = c.add(&perturbed[index], &delta).unwrap();
        let d = stop_rule(
            layout,
            extents,
            &perturbed,
            &b.recovered.values,
            256,
            StageGuard::unlimited(),
        );
        assert_eq!(
            (d.result, d.first_failure),
            (Ok(false), Some(index)),
            "{:?}",
            meta.id
        );
        types.insert(std::mem::discriminant(&meta.id));
        kinds.insert(meta.kind.index());
    }
    // Displacements, magnitudes, end and station actions, spring actions,
    // reactions and support magnitudes, of all four kinds, were perturbed.
    assert_eq!(types.len(), 8);
    assert_eq!(kinds.len(), 4);
}

// ---------------------------------------------------------------- L: budgets and work

fn charge(solve_attempts: &[AttemptRecord]) -> u64 {
    solve_attempts
        .iter()
        .map(|a| {
            a.work.limb_multiply_equivalents()
                + a.k4_work.limb_multiply_equivalents()
                + a.shared_work
        })
        .sum()
}

#[test]
fn a_case_limit_is_exhausted_exactly_where_its_work_ends() {
    for name in ["N05", "SKEW-K1E-28"] {
        let source = models::model(name).source();
        let (limit, mut meter) = unlimited();
        let CaseOutcome::Selected(solve) = solve_case(source.clone(), limit, &mut meter) else {
            panic!()
        };
        let needed = charge(&solve.evidence().attempts);
        assert_eq!(
            meter.charged(),
            needed,
            "{name}: a lone case charges the invocation its full work"
        );
        let mut meter = InvocationMeter::new(u64::MAX);
        assert!(matches!(
            solve_case(source.clone(), CaseLimit::new(needed), &mut meter),
            CaseOutcome::Selected(_)
        ));
        let mut meter = InvocationMeter::new(u64::MAX);
        match solve_case(source.clone(), CaseLimit::new(needed - 1), &mut meter) {
            CaseOutcome::Unresolved {
                reason: UnresolvedReason::Budget(BudgetScope::Case),
                attempts,
                ..
            } => assert!(!attempts.is_empty() || needed - 1 == 0),
            other => panic!("{name}: {other:?}"),
        }
        let mut meter = InvocationMeter::new(needed - 1);
        assert!(matches!(
            solve_case(source.clone(), CaseLimit::new(u64::MAX), &mut meter),
            CaseOutcome::Unresolved {
                reason: UnresolvedReason::Budget(BudgetScope::Invocation),
                ..
            }
        ));
        // An exhausted invocation refuses further cases before any work.
        let before = meter.charged();
        assert!(matches!(
            solve_case(source, CaseLimit::new(u64::MAX), &mut meter),
            CaseOutcome::Unresolved {
                reason: UnresolvedReason::Budget(BudgetScope::Invocation),
                ..
            }
        ));
        assert_eq!(meter.charged(), before);
    }
}

#[test]
fn failed_and_verification_attempts_are_charged() {
    let solve = selected("SKEW-K1E-28");
    let attempts = &solve.evidence().attempts;
    assert!(matches!(attempts[0].outcome, AttemptOutcome::Failed(_)));
    for a in attempts {
        assert!(a.shared_work > 0, "{a:?}");
        assert!(a.shared_built_here);
    }
    // The failed factor's work is its formation, assembly and factor stages.
    let s = &attempts[0].shared_stages;
    assert!(s.formation > 0 && s.assembly > 0 && s.factor > 0);
    assert_eq!(attempts[0].work.limb_multiply_equivalents(), 0);
    // The verification solve carries its own stages, and the stop rule is
    // charged to the candidate.
    assert!(attempts[2].work.limb_multiply_equivalents() > 0);
    assert!(attempts[1].stop_rule_work > 0 && attempts[2].stop_rule_work == 0);
}

#[test]
fn a_sums_rounding_charges_the_limbs_it_rounds() {
    // from_integer's input length is charged: a one-limb net against a net
    // spanning 1,075 bits (at least 17 limbs).
    let rounded = |terms: &[f64]| {
        let mut sum = super::super::wide_sum::ExactWideSum::new();
        for &t in terms {
            sum.add_binary64(t, false).unwrap();
        }
        let mut c = WideContext::<4>::new(128).unwrap();
        let before = sum.work().rounded_limbs;
        let _ = sum.round(&mut c).unwrap();
        sum.work().rounded_limbs - before
    };
    let narrow = rounded(&[1.0, 2.0]);
    let wide = rounded(&[1.0, f64::from_bits(1)]);
    assert!(narrow <= 2, "{narrow}");
    assert!((17..=20).contains(&wide), "{wide}");
}

/// (precision, own work, K4 sum work, shared work, stop-rule work, stages).
fn work_table(name: &str) -> Vec<(u32, u64, u64, u64, u64, StageWork, StageWork)> {
    selected(name)
        .evidence()
        .attempts
        .iter()
        .map(|a| {
            (
                a.precision,
                a.work.limb_multiply_equivalents(),
                a.k4_work.limb_multiply_equivalents(),
                a.shared_work,
                a.stop_rule_work,
                a.stages.clone(),
                a.shared_stages.clone(),
            )
        })
        .collect()
}

#[test]
fn golden_work_counts() {
    // SD-L1 (K4-M17/M18/M19; ROOT's O5: TWO-SPAN's p + 64 residual is
    // evidenced by its correction and its refinement and residual-formation
    // work). Per attempt: (p, own context work, K4 sum work, shared work,
    // stop-rule work, refinement, residual formation), in limb-multiply
    // equivalents.
    let golden: [(&str, &[(u32, u64, u64, u64, u64, u64, u64)]); 4] = [
        (
            "N05",
            &[
                (128, 119406, 823, 175768, 80007, 23252, 28227),
                (256, 40472, 371, 240395, 0, 23841, 92691),
            ],
        ),
        (
            "N06",
            &[
                (128, 136912, 828, 175761, 80009, 40772, 28227),
                (256, 57978, 394, 240385, 0, 41365, 92691),
            ],
        ),
        (
            "TWO-SPAN",
            &[
                (128, 179484, 2081, 214495, 128670, 37577, 57274),
                (256, 52934, 1549, 345578, 0, 38808, 187120),
            ],
        ),
        (
            "SKEW6-K1E-12",
            &[
                (128, 163112, 4892, 1258935, 2712, 81078, 200498),
                (256, 465728, 13456, 1685244, 293232, 100032, 619826),
                (512, 439660, 13237, 5669481, 0, 164905, 2164328),
            ],
        ),
    ];
    for (name, rows) in golden {
        let got: Vec<(u32, u64, u64, u64, u64, u64, u64)> = work_table(name)
            .into_iter()
            .map(|(p, own, k4, shared, stop, stages, shared_stages)| {
                (
                    p,
                    own,
                    k4,
                    shared,
                    stop,
                    stages.refinement,
                    shared_stages.residual_formation,
                )
            })
            .collect();
        assert_eq!(got, rows, "{name}");
    }
}

// ---------------------------------------------------------------- L: determinism and factor reuse

fn fingerprint(
    solve: &RetainedSolve,
) -> (
    Vec<u8>,
    Vec<u8>,
    Vec<u8>,
    Vec<(u32, AttemptOutcome, u64)>,
    Vec<PublishedRow>,
) {
    let e = solve.evidence();
    (
        e.source_encoding.clone(),
        e.ledger_encoding.clone(),
        e.retained_state_encoding.clone(),
        e.attempts
            .iter()
            .map(|a| {
                (
                    a.precision,
                    a.outcome.clone(),
                    a.work.limb_multiply_equivalents(),
                )
            })
            .collect(),
        solve.publish().rows.clone(),
    )
}

#[test]
fn runs_and_list_permutations_are_deterministic() {
    for name in ["N09-B", "SKEW6-K1E-12", "DIRECTIONAL-SPAN", "PRESCRIBED"] {
        let m = models::model(name);
        let a = fingerprint(&selected(name));
        assert!(a == fingerprint(&selected(name)), "{name}: run to run");
        let mut parts = m.parts.clone();
        parts.members.reverse();
        parts.springs.reverse();
        parts.directional_springs.reverse();
        parts.constraints.reverse();
        parts.loads.reverse();
        parts.stations.reverse();
        let (limit, mut meter) = unlimited();
        let CaseOutcome::Selected(b) = solve_case(
            super::super::source::PrimitiveSource::new(parts).unwrap(),
            limit,
            &mut meter,
        ) else {
            panic!()
        };
        assert!(a == fingerprint(&b), "{name}: permuted lists");
    }
}

#[test]
fn factor_reuse_is_bit_identical_to_separate_solves() {
    // Two cases on one stiffness identity (SKEW-K1E-28 and its axial variant).
    let names = ["SKEW-K1E-28", "SKEW-K1E-28-AXIAL"];
    let sources: Vec<_> = names.iter().map(|n| models::model(n).source()).collect();
    let (limit, mut meter) = unlimited();
    let together = solve_cases(&sources, limit, &mut meter);
    let separate_charge: u64 = names
        .iter()
        .map(|n| {
            let (limit, mut meter) = unlimited();
            let _ = solve_case(models::model(n).source(), limit, &mut meter);
            meter.charged()
        })
        .sum();
    assert!(
        meter.charged() < separate_charge,
        "shared stages are charged once"
    );
    for (outcome, name) in together.iter().zip(names) {
        let CaseOutcome::Selected(t) = outcome else {
            panic!()
        };
        let s = selected(name);
        assert!(fingerprint(t) == fingerprint(&s), "{name}");
        // Per-case budget decisions equal a separate solve's: each attempt
        // counts the shared work in full.
        let shared = |x: &RetainedSolve| {
            x.evidence()
                .attempts
                .iter()
                .map(|a| a.shared_work)
                .collect::<Vec<_>>()
        };
        assert_eq!(shared(t), shared(&s));
    }
    let CaseOutcome::Selected(second) = &together[1] else {
        panic!()
    };
    assert!(second
        .evidence()
        .attempts
        .iter()
        .all(|a| !a.shared_built_here));
}

// ---------------------------------------------------------------- L: the source scan

/// Strips comments, string and character literals (a small Rust lexer).
fn code_only(src: &str) -> String {
    let b: Vec<char> = src.chars().collect();
    let mut out = String::with_capacity(src.len());
    let mut i = 0;
    while i < b.len() {
        let c = b[i];
        if c == '/' && i + 1 < b.len() && b[i + 1] == '/' {
            while i < b.len() && b[i] != '\n' {
                i += 1;
            }
            continue;
        }
        if c == '/' && i + 1 < b.len() && b[i + 1] == '*' {
            i += 2;
            while i + 1 < b.len() && !(b[i] == '*' && b[i + 1] == '/') {
                i += 1;
            }
            i += 2;
            continue;
        }
        if c == '"' {
            i += 1;
            while i < b.len() && b[i] != '"' {
                if b[i] == '\\' {
                    i += 1;
                }
                i += 1;
            }
            i += 1;
            out.push_str("\"\"");
            continue;
        }
        if c == '\'' && i + 2 < b.len() && (b[i + 2] == '\'' || b[i + 1] == '\\') {
            let mut j = i + 1;
            if b[j] == '\\' {
                j += 1;
            }
            while j < b.len() && b[j] != '\'' {
                j += 1;
            }
            i = j + 1;
            out.push_str("' '");
            continue;
        }
        out.push(c);
        i += 1;
    }
    out
}

const FORBIDDEN: [&str; 26] = [
    "powi", "powf", "exp", "exp2", "exp_m1", "ln", "ln_1p", "log", "log2", "log10", "sin", "cos",
    "tan", "asin", "acos", "atan", "atan2", "sinh", "cosh", "tanh", "asinh", "acosh", "atanh",
    "hypot", "cbrt", "mul_add",
];

fn forbidden_calls(code: &str) -> Vec<String> {
    let mut out = Vec::new();
    let chars: Vec<char> = code.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i].is_alphabetic() || chars[i] == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            let called = chars[i..].iter().find(|c| !c.is_whitespace()) == Some(&'(');
            if called && FORBIDDEN.contains(&word.as_str()) {
                out.push(word);
            }
        } else {
            i += 1;
        }
    }
    out
}

#[test]
fn k4s_files_call_no_binary64_transcendental_or_fused_function() {
    // The lexer's self-control: names in comments and strings are ignored,
    // calls in code are found.
    let control = "// x.powi(2)\n/* y.sin() */ let s = \"z.exp()\"; let c = '('; let a = b.hypot(c); f.mul_add(1.0, 2.0);";
    assert_eq!(
        forbidden_calls(&code_only(control)),
        vec!["hypot", "mul_add"]
    );
    let files = [
        include_str!("../../src/structural/retained/wide_sum.rs"),
        include_str!("../../src/structural/retained/source.rs"),
        include_str!("../../src/structural/retained/ledger.rs"),
        include_str!("../../src/structural/retained/assemble.rs"),
        include_str!("../../src/structural/retained/factor.rs"),
        include_str!("../../src/structural/retained/recover.rs"),
        include_str!("../../src/structural/retained/adaptive.rs"),
        include_str!("../../src/structural/retained/combine.rs"),
    ];
    for (k, text) in files.iter().enumerate() {
        let code = code_only(text);
        let found = forbidden_calls(&code);
        assert!(found.is_empty(), "file {k}: {found:?}");
        assert!(code.len() > 1000);
    }
}

// ---------------------------------------------------------------- F: the refinement's cap

/// Solves `name` at 128 (residual at 192) with every k_q diagonal scaled by
/// 1 + 2^-e (a perturbed residual basis: refinement then converges to the
/// perturbed system at a rate of about κ·2^-e per correction).
fn perturbed_solve(name: &str, e: i64) -> Result<u8, AttemptStop> {
    // (The unperturbed factor at p is kept: only the residual's K changes.)
    let source = models::model(name).source();
    let prep = CasePrep::new(source.clone()).unwrap();
    let group = prepare_group(&source).unwrap();
    let mut shared = build_shared::<4, 4>(128, 192, &source, &group, StageGuard::unlimited())
        .result
        .unwrap();
    let mut c = WideContext::<4>::new(192).unwrap();
    for (r, col, index) in group.structure.entries() {
        if r == col {
            let v = shared.k_q[index];
            let delta = v.mul_pow2(-e).unwrap();
            shared.k_q[index] = c.add(&v, &delta).unwrap();
        }
    }
    solve_case_at(&shared, &prep, &group, StageGuard::unlimited())
        .result
        .map(|solved| solved.corrections)
}

#[test]
fn refinement_stops_after_three_corrections_and_the_attempt_escalates() {
    // N09-B's residual basis perturbed on the diagonal by 2^-e: e = 32 needs
    // exactly three corrections (accepted); e = 28 would need a fourth, so the
    // attempt stops with the residual gate, an escalating stop; e = 120 needs
    // none and e = 80 one.
    assert_eq!(perturbed_solve("N09-B", 120), Ok(0));
    assert_eq!(perturbed_solve("N09-B", 80), Ok(1));
    assert_eq!(perturbed_solve("N09-B", 48), Ok(2));
    assert_eq!(perturbed_solve("N09-B", 32), Ok(3));
    let stop = perturbed_solve("N09-B", 28).unwrap_err();
    assert!(matches!(stop, AttemptStop::ResidualGate { .. }));
    assert!(stop.escalates());
}

// ---------------------------------------------------------------- O8: the method, bit for bit

#[test]
fn retained_states_equal_the_generators_emulation_of_the_method_bit_for_bit() {
    // ROOT's ruling O8: the generator mirrors formation, assembly, the RCM
    // order, equilibration, the p-factor, the solves, the p + 64 residual and
    // the recovery of Q for N05 and N06 at 128 and 256 and one skew member at
    // 128; the retained-state encodings' sha256 must match.
    let mut checked = 0;
    for line in include_str!("o8_states.txt").lines() {
        let f: Vec<&str> = line.split_whitespace().collect();
        let (name, p, sha, corrections) = (f[1], f[2].parse::<u32>().unwrap(), f[3], f[4]);
        let solve = selected(name);
        let state = solve
            .state(p)
            .unwrap_or_else(|| panic!("{name} has no state at {p}"));
        assert_eq!(support::sha256_hex(&state.encoding()), sha, "{name} at {p}");
        assert_eq!(
            state.corrections().to_string(),
            corrections,
            "{name} at {p}"
        );
        checked += 1;
    }
    assert_eq!(checked, 5);
}
