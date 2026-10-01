//! A1 v2 authoring checkpoint. Expected mathematics is independent of solver
//! observations: C17 is the frozen source equilibrium; the small fixtures state
//! exact dyadic inputs. Absolute-work ledgers and new F/M-source truth are
//! separately owned verification prerequisites, not generated from these counts.
use super::super::source::{Constraint, NodalLoad, SourceParts, Spring, StraightMember};
use super::*;

fn w(x: f64) -> Wide<4> {
    Wide::<4>::from_f64(x).unwrap()
}

fn pow2(e: i32) -> f64 {
    assert!((-1022..=1023).contains(&e));
    f64::from_bits(((e + 1023) as u64) << 52)
}

fn meta(kind: Kind) -> QuantityMeta {
    QuantityMeta {
        id: QuantityId::Displacement(Dof::from_global(if kind == Kind::Rotation { 3 } else { 0 })),
        kind,
        body: 0,
        input_derived: false,
    }
}

fn report(p: u32) -> VerificationReport<4> {
    VerificationReport {
        precision: p,
        q_w: (p / 2 * 3 + 64).min(1024),
        resolution: vec![[0.0; 2]],
        e_rows: vec![None],
        w: vec![None],
        a_s: vec![None],
        charge: vec![None],
        w_plus: vec![None],
        r_hat: Vec::new(),
        delta: Vec::new(),
        blocks: Vec::new(),
        norms: Vec::new(),
        theta: Vec::new(),
        bodies: Vec::new(),
        g_max: 0,
        g_violation: None,
        uc_missing: None,
        shift_factorizations: 0,
    }
}

fn meter() -> CertificateMeter {
    CertificateMeter::new(StageGuard::unlimited())
}

fn exact_sum(terms: &[(f64, i64)]) -> ExactWideSum {
    let mut s = ExactWideSum::new();
    for &(x, shift) in terms {
        s.add_wide_scaled(&w(x), false, 1, shift).unwrap();
    }
    s
}

fn equal_sum(actual: &ExactWideSum, expected: &ExactWideSum) {
    let mut d = ExactWideSum::new();
    d.add_scaled(actual, false, 1, 0).unwrap();
    d.add_scaled(expected, true, 1, 0).unwrap();
    assert_eq!(d.signum(), 0);
}

fn row(x: f64, class: RowClass) -> PublishedRow {
    PublishedRow {
        id: QuantityId::Displacement(Dof::from_global(0)),
        kind: Kind::Translation,
        body: 0,
        value: Binary64Outcome::Normal(x),
        class,
    }
}

#[test]
fn h_uses_final_binary64_and_every_error_term() {
    let h = f64::from_bits(1);
    let mut r = report(256);
    r.w_plus[0] = Some(Wide::<4>::ZERO);
    let v = w(f64::from_bits(5)).mul_pow2(-2).unwrap();
    let got = publication_h(0, &meta(Kind::Rotation), h, &v, &r, &mut meter()).unwrap();
    equal_sum(&got, &exact_sum(&[(h, -2)]));

    for p in [256, 512, 1024] {
        let mut r = report(p);
        r.e_rows[0] = Some(w(1.0));
        r.w[0] = Some(w(2.0));
        r.charge[0] = Some(w(4.0));
        let got = publication_h(0, &meta(Kind::Force), 1.0, &w(0.0), &r, &mut meter()).unwrap();
        // |1-0| + 69*2^-P + 2 + 2*2^-P + 4.
        equal_sum(&got, &exact_sum(&[(7.0, 0), (71.0, -i64::from(p))]));
        r.w_plus[0] = Some(w(2.0));
        let magnitude = QuantityMeta {
            id: QuantityId::DisplacementMagnitude(0),
            ..meta(Kind::Translation)
        };
        let got = publication_h(0, &magnitude, 1.0, &w(1.0), &r, &mut meter()).unwrap();
        equal_sum(&got, &exact_sum(&[(2.0, 0), (1.0, 1 - i64::from(p))]));
    }
}

#[test]
fn missing_or_negative_error_is_not_zero() {
    let mut r = report(256);
    assert!(matches!(
        publication_h(0, &meta(Kind::Translation), 0.0, &w(0.0), &r, &mut meter()),
        Err(AttemptStop::PublicationCertificate {
            issue: CertificateIssue::MissingField,
            ..
        })
    ));
    r.w_plus[0] = Some(w(-1.0));
    assert!(matches!(
        publication_h(0, &meta(Kind::Translation), 0.0, &w(0.0), &r, &mut meter()),
        Err(AttemptStop::PublicationCertificate {
            issue: CertificateIssue::NegativeField,
            ..
        })
    ));
}

#[test]
fn bare_bound_including_zero_has_no_observed_zero_exemption() {
    let absolute = row(
        0.0,
        RowClass::AbsoluteVerified {
            bound_bits: 1.0f64.to_bits(),
        },
    );
    for (h, pass) in [(next_down(1.0), true), (1.0, true), (next_up(1.0), false)] {
        let verdict =
            publication_predicate(&exact_sum(&[(h, 0)]), &absolute, pow2(64), &mut meter())
                .unwrap();
        assert_eq!(verdict.is_none(), pass);
    }
    let zero = row(0.0, RowClass::AbsoluteVerified { bound_bits: 0 });
    assert_eq!(
        publication_predicate(&exact_sum(&[]), &zero, 0.0, &mut meter()).unwrap(),
        None
    );
    assert_eq!(
        publication_predicate(
            &exact_sum(&[(f64::from_bits(1), 0)]),
            &zero,
            0.0,
            &mut meter()
        )
        .unwrap(),
        Some(PublicationPredicate::AbsoluteBound)
    );
    let between = exact_sum(&[(1.0, 0), (1.0, -23)]);
    assert_eq!(
        publication_predicate(&between, &absolute, pow2(64), &mut meter()).unwrap(),
        Some(PublicationPredicate::AbsoluteBound)
    );
}

#[test]
fn relative_admission_keeps_public_and_sharper_predicates() {
    let relative = row(1.0, RowClass::RelativeVerified);
    // Public passes 2^-40, but the much sharper allowance at S=1 does not.
    assert_eq!(
        publication_predicate(&exact_sum(&[(1.0, -40)]), &relative, 1.0, &mut meter()).unwrap(),
        Some(PublicationPredicate::SharperExact)
    );
    // The public promise remains a separate exact decimal comparison.
    assert_eq!(
        publication_predicate(&exact_sum(&[(1.0, -29)]), &relative, pow2(34), &mut meter())
            .unwrap(),
        Some(PublicationPredicate::PublicRelative)
    );
    assert!(
        publication_predicate(&exact_sum(&[(1.0, -65)]), &relative, 1.0, &mut meter())
            .unwrap()
            .is_none()
    );
    assert!(sharper_binary64(1.0, f64::INFINITY).is_err());
}

#[test]
fn upward_radius_is_not_nearest_or_underflowed_zero() {
    let h = f64::from_bits(1);
    for (terms, expected) in [
        (vec![], 0.0),
        (vec![(h, -1)], h),
        (vec![(h, 0)], h),
        (vec![(1.0, 0), (h, 0)], f64::from_bits(1.0f64.to_bits() + 1)),
        (vec![(f64::MAX, 0)], f64::MAX),
    ] {
        assert_eq!(
            meter().round_up(&exact_sum(&terms)).unwrap().to_bits(),
            expected.to_bits()
        );
    }
    assert!(meter()
        .round_up(&exact_sum(&[(f64::MAX, 0), (1.0, 0)]))
        .unwrap()
        .is_infinite());
}

#[test]
fn metered_bound_matches_pinned_pure_formula_at_branches() {
    let h = f64::from_bits(1);
    let boundary = pow2(-988);
    for scale in [
        0.0,
        h,
        next_down(boundary),
        boundary,
        next_up(boundary),
        1.0,
    ] {
        for x in [0.0, h, scale] {
            assert_eq!(
                meter().row_bound(x, scale).unwrap().to_bits(),
                row_bound(x, scale).to_bits()
            );
        }
    }
    assert_eq!(meter().row_bound(h, h).unwrap().to_bits(), 3);
}

#[test]
fn clone_counter_delta_never_bills_inherited_work() {
    // Fixed independent arithmetic on counters; this is NOT the yet-required
    // independent absolute operation ledger for the real conversion path.
    let before = SumWork {
        term_limbs: 31,
        shift_limbs: 17,
        net_limbs: 9,
        rounded_limbs: 5,
        max_span_bits: 1075,
    };
    let after = SumWork {
        term_limbs: 38,
        shift_limbs: 19,
        net_limbs: 12,
        rounded_limbs: 9,
        max_span_bits: 1075,
    };
    let delta = sum_work_delta(after, before);
    assert_eq!(delta.limb_multiply_equivalents(), 16);
    assert_eq!(delta.max_span_bits, 1075);
}

#[test]
fn budget_stop_collects_new_conversion_work_without_a_radius() {
    let mut m = CertificateMeter::new(StageGuard::with_case_room(0));
    let result = m.round_up(&exact_sum(&[(1.0, 0), (f64::from_bits(1), 0)]));
    assert!(matches!(
        result,
        Err(AttemptStop::Budget(BudgetScope::Case))
    ));
    assert!(m.total() > 0);
    // Absolute ledger/stop-position assertions are intentionally not invented
    // from m.total(); RV28-N1's independent frozen ledger is still required.
}

#[test]
fn certificate_errors_are_terminal_and_budget_precedence_is_unchanged() {
    let stop = certificate_error(Some(0), CertificateIssue::MissingField);
    assert!(!stop.escalates());
    assert_eq!(
        terminal(&stop),
        Ok(UnresolvedReason::PublicationCertificate {
            index: Some(0),
            issue: CertificateIssue::MissingField,
        })
    );
    let g = StageGuard {
        base: 0,
        case_room: 0,
        invocation_room: 0,
    };
    assert_eq!(g.test(1), Err(AttemptStop::Budget(BudgetScope::Case)));
}

fn two_node_source(
    length: f64,
    area: f64,
    torsion: f64,
    free: &[usize],
    loads: &[(usize, f64, &str)],
    springs: &[(usize, f64)],
) -> PrimitiveSource {
    let mut parts = SourceParts {
        nodes: vec![[0.0; 3], [length, 0.0, 0.0]],
        members: vec![StraightMember {
            id: 1,
            node_i: 0,
            node_j: 1,
            elastic_modulus: 1.0,
            shear_modulus: 1.0,
            area,
            second_moment_y: 1.0,
            second_moment_z: 1.0,
            torsion_constant: torsion,
            y_reference: [0.0, 1.0, 0.0],
        }],
        ..SourceParts::default()
    };
    parts.constraints = (0..12)
        .filter(|g| !free.contains(g))
        .map(|g| Constraint {
            dof: Dof::from_global(g),
            value: 0.0,
        })
        .collect();
    parts.loads = loads
        .iter()
        .map(|&(g, value, id)| NodalLoad {
            dof: Dof::from_global(g),
            value,
            source_id: id.into(),
        })
        .collect();
    parts.springs = springs
        .iter()
        .enumerate()
        .map(|(i, &(g, stiffness))| Spring {
            id: i as u32 + 1,
            dof: Dof::from_global(g),
            stiffness,
        })
        .collect();
    PrimitiveSource::new(parts).unwrap()
}

fn c17() -> PrimitiveSource {
    // Immutable C17 primitive bits and exact independent spring equilibrium.
    // R/I22/c_01/C17_INPUT.json, frozen oracle TRUTH 1772d703e032a715...
    two_node_source(
        pow2(100),
        pow2(176),
        pow2(102),
        &[0, 6, 9],
        &[
            (0, pow2(-900), "C17:axial-F-ground"),
            (0, f64::from_bits(9), "C17:axial-tail-ground"),
            (6, -pow2(-900), "C17:axial-minus-F-other"),
            (9, f64::from_bits(5), "C17:tip-Mx"),
        ],
        &[(0, pow2(-33))],
    )
}

fn outcome_attempts(outcome: &CaseOutcome) -> &[AttemptRecord] {
    match outcome {
        CaseOutcome::Selected(s) => &s.evidence().attempts,
        CaseOutcome::Unresolved { attempts, .. } => attempts,
        CaseOutcome::Refused { refusal, .. } => panic!("unexpected source refusal: {refusal:?}"),
    }
}

#[test]
fn c17_cannot_publish_the_false_p128_interval() {
    let mut invocation = InvocationMeter::new(u64::MAX);
    let out = solve_case(c17(), CaseLimit::new(u64::MAX), &mut invocation);
    let attempts = outcome_attempts(&out);
    assert!(matches!(
        attempts[0].outcome,
        AttemptOutcome::Rejected(AttemptReason::PublicationEnclosure {
            predicate: PublicationPredicate::AbsoluteBound,
            ..
        })
    ));
    assert!(attempts.iter().all(|a| a.precision <= 1024));
    if let CaseOutcome::Selected(s) = out {
        assert_ne!(s.selected_precision(), 128);
        // Sum of applied axial terms is 9h; spring k=2^-33.
        // The exact truth 9*2^-1041 has binary64 bits 0x0000001200000000.
        let exact = f64::from_bits(0x0000_0012_0000_0000);
        for id in [
            QuantityId::Displacement(Dof::from_global(0)),
            QuantityId::DisplacementMagnitude(0),
        ] {
            let row = s.publish().rows.iter().find(|r| r.id == id).unwrap();
            let x = row.value.value().expect("finite ground translation");
            let mut error = ExactWideSum::new();
            error.add_binary64(x, false).unwrap();
            error.add_binary64(exact, true).unwrap();
            error.make_absolute();
            if let RowClass::AbsoluteVerified { bound_bits } = row.class {
                let mut margin = ExactWideSum::new();
                margin
                    .add_binary64(f64::from_bits(bound_bits), false)
                    .unwrap();
                margin.add_scaled(&error, true, 1, 0).unwrap();
                assert!(margin.signum() >= 0, "bare bound, no qualified slack");
            } else {
                let mut margin = ExactWideSum::new();
                margin.add_binary64(x.abs(), false).unwrap();
                margin.add_scaled(&error, true, 1_000_000_000, 0).unwrap();
                assert!(margin.signum() >= 0);
            }
        }
    }
}

#[test]
fn a_selected_zero_source_owns_paired_radii_and_clones_them() {
    let source = two_node_source(1.0, 1.0, 1.0, &[6], &[], &[]);
    let mut invocation = InvocationMeter::new(u64::MAX);
    let out = solve_case(source, CaseLimit::new(u64::MAX), &mut invocation);
    let CaseOutcome::Selected(solve) = out else {
        panic!("zero source must remain selectable")
    };
    assert_eq!(solve.evidence().policy, "M03-INTEGRITY-MP-v2");
    let clone = solve.clone();
    assert_eq!(solve.publication_radius_bits, clone.publication_radius_bits);
    assert_ne!(
        solve.publication_radius_bits.as_ptr(),
        clone.publication_radius_bits.as_ptr()
    );
    for (i, m) in solve.prep.layout.iter().enumerate() {
        let radius = solve
            .publication_radius(i, *m, &solve.prep.identity, solve.selected)
            .unwrap();
        if m.input_derived {
            assert!(radius.is_none());
        } else {
            assert_eq!(radius.unwrap().bits, 0);
        }
    }
    let mut corrupt = clone;
    corrupt.publication_radius_bits[0] = 0;
    assert_eq!(
        corrupt.publication_radius(
            0,
            corrupt.prep.layout[0],
            &corrupt.prep.identity,
            corrupt.selected
        ),
        Err(CertificateIssue::RadiusClassMismatch)
    );
}

// The three new raw source proposals may be embedded, but their independently
// frozen all-row truth has not yet been supplied. No result assertion or
// automatic test run is attached to these constructors.
#[allow(dead_code)]
fn proposed_raw_force_control() -> PrimitiveSource {
    two_node_source(
        pow2(100),
        pow2(100),
        1.0,
        &[6],
        &[(6, f64::from_bits(5), "EXTRA-FM-01:load")],
        &[(6, 1.0); 3],
    )
}
#[allow(dead_code)]
fn proposed_raw_moment_control() -> PrimitiveSource {
    two_node_source(
        pow2(-100),
        1.0,
        pow2(-100),
        &[9],
        &[(9, f64::from_bits(5), "EXTRA-MF-01:load")],
        &[(9, 1.0); 3],
    )
}
#[allow(dead_code)]
fn proposed_free_zero_rotation_control() -> PrimitiveSource {
    two_node_source(
        pow2(100),
        pow2(102),
        pow2(102),
        &[6, 9],
        &[(6, f64::from_bits(5), "EXTRA-ZR-01:load")],
        &[],
    )
}

// PC40--43: independent RV29 candidate_01 ledger, frozen before counters:
// LEDGER.json SHA256 89e088c16e3a2f6edd0d927922f8a4f25984466015611a7dabe6871bf01ba15d.
// These are standalone algebra paths; no full-case price is inferred.
fn ledger_h_route(
    guard: StageGuard,
    b: f64,
) -> (Result<Option<f64>, AttemptStop>, CertificateMeter) {
    let mut m = CertificateMeter::new(guard);
    let mut r = report(256);
    r.w_plus[0] = Some(w(f64::from_bits(1)));
    let result = (|| {
        let h = publication_h(0, &meta(Kind::Translation), 1.0, &w(0.0), &r, &mut m)?;
        let value = row(
            1.0,
            RowClass::AbsoluteVerified {
                bound_bits: b.to_bits(),
            },
        );
        if publication_predicate(&h, &value, b * pow2(64), &mut m)?.is_some() {
            return Ok(None);
        }
        m.round_up(&h).map(Some)
    })();
    (result, m)
}

fn sum_tuple(s: SumWork) -> (u64, u64, u64, u64) {
    (s.term_limbs, s.shift_limbs, s.net_limbs, s.rounded_limbs)
}

#[test]
fn pc40_independent_absolute_work_ledgers_success_and_numeric_rejection() {
    let (result, m) = ledger_h_route(StageGuard::unlimited(), 2.0);
    assert_eq!(result, Ok(Some(f64::from_bits(1.0f64.to_bits() + 1))));
    assert_eq!(m.total(), 18_103); // H44 + predicate115 + RU17_944.
    assert_eq!(sum_tuple(m.sums), (250, 188, 138, 21));
    assert_eq!(lme(&m.ctx), 17_506);
    // RV29 additive source derivation e08b726fb695752b...: the b=2
    // comparison retains uncancelled term extrema +1 and -1074.
    // Aggregate span is 1-(-1074)+1=1076; H and RU alone remain 1075.
    assert_eq!(m.sums.max_span_bits, 1076);
    let (result, m) = ledger_h_route(StageGuard::unlimited(), 1.0);
    assert_eq!(result, Ok(None));
    assert_eq!(m.total(), 159); // Rejection precedes RU, not an omitted radius.
    assert_eq!(m.sums.max_span_bits, 1075); // b=1 has high exponent 0.
    let mut m = meter();
    assert_eq!(
        m.row_bound(f64::from_bits(1), f64::from_bits(1))
            .unwrap()
            .to_bits(),
        3
    );
    assert_eq!(m.total(), 17_563);
    assert_eq!(sum_tuple(m.sums), (32, 6, 15, 4));
    assert_eq!(lme(&m.ctx), 17_506);
    assert_eq!(m.sums.max_span_bits, 2);
}

#[test]
fn pc41_independent_successful_checkpoints_and_both_budget_scopes() {
    let checkpoints = [44, 159, 178, 267, 269, 307, 17_749, 17_868, 17_984, 18_103];
    for (i, &c) in checkpoints.iter().enumerate() {
        for scope in [BudgetScope::Case, BudgetScope::Invocation] {
            let guard = StageGuard {
                base: 0,
                case_room: if scope == BudgetScope::Case {
                    c - 1
                } else {
                    u64::MAX
                },
                invocation_room: if scope == BudgetScope::Invocation {
                    c - 1
                } else {
                    u64::MAX
                },
            };
            let (result, m) = ledger_h_route(guard, 2.0);
            assert_eq!(result, Err(AttemptStop::Budget(scope)));
            assert_eq!(m.total(), c);
        }
        let (result, m) = ledger_h_route(StageGuard::with_case_room(c), 2.0);
        if let Some(&next) = checkpoints.get(i + 1) {
            assert_eq!(result, Err(AttemptStop::Budget(BudgetScope::Case)));
            assert_eq!(m.total(), next); // Equality admitted c, next check stopped.
        } else {
            assert!(result.unwrap().is_some());
            assert_eq!(m.total(), c);
        }
    }
    // Duplicate zero-cost final checks share the last checkpoint; they cannot
    // be uniquely selected by a budget. No test claims a distinct late stop.
    let bound_checkpoints = [6, 8, 46, 48, 86, 17_528, 17_542, 17_563];
    for c in bound_checkpoints {
        let mut m = CertificateMeter::new(StageGuard::with_case_room(c - 1));
        assert_eq!(
            m.row_bound(f64::from_bits(1), f64::from_bits(1)),
            Err(AttemptStop::Budget(BudgetScope::Case))
        );
        assert_eq!(m.total(), c);
    }
}

#[test]
fn pc42_independent_terminal_work_and_precedence() {
    let mut r = report(256);
    r.w_plus[0] = Some(w(1.0).mul_pow2(-8128).unwrap());
    for room in [0, u64::MAX] {
        let mut m = CertificateMeter::new(StageGuard::with_case_room(room));
        assert!(matches!(
            publication_h(0, &meta(Kind::Translation), 1.0, &w(0.0), &r, &mut m),
            Err(AttemptStop::Span)
        ));
        assert_eq!(m.total(), 4);
    }
    for (room, expected, total) in [
        (1, AttemptStop::Budget(BudgetScope::Case), 2),
        (3, AttemptStop::Budget(BudgetScope::Case), 4),
        (4, AttemptStop::Exponent, 42),
        (u64::MAX, AttemptStop::Exponent, 42),
    ] {
        let mut m = CertificateMeter::new(StageGuard::with_case_room(room));
        let mut num = ExactWideSum::new();
        let result = (|| {
            let formed = num
                .add_integer(false, &[1], (1i64 << 62) + 1)
                .map_err(AttemptStop::from);
            m.collect(&num, formed)?;
            m.round_up(&num)
        })();
        assert_eq!(result, Err(expected));
        assert_eq!(m.total(), total);
    }
}

#[test]
fn pc09_12_both_independent_rounding_directions_and_decimal_boundary() {
    // Frozen RV28 algebra: x=S=1, A_exact=A_f64+h,
    // A_f64=4297064449*2^-85. H between them must fail the binary64 predicate.
    let af64 = exact_sum(&[(4_297_064_449.0, -85)]);
    let between = exact_sum(&[(4_297_064_449.0, -85), (f64::from_bits(1), -1)]);
    let one = row(1.0, RowClass::RelativeVerified);
    assert_eq!(
        publication_predicate(&af64, &one, 1.0, &mut meter()).unwrap(),
        None
    );
    assert_eq!(
        publication_predicate(&between, &one, 1.0, &mut meter()).unwrap(),
        Some(PublicationPredicate::SharperBinary64)
    );
    // Frozen opposite direction: x=S=3*2^-1022,
    // A_exact=21481127939*2^-1107, A_f64=3h.
    // This isolates predicate arithmetic, not a claim of reachable relative class.
    let x = 3.0 * pow2(-1022);
    let tiny = row(x, RowClass::RelativeVerified);
    let exact = exact_sum(&[(21_481_127_939.0, -1107)]);
    let between = exact_sum(&[(47_250_931_715.0, -1108)]);
    assert_eq!(sharper_binary64(x, x).unwrap().to_bits(), 3);
    assert_eq!(
        publication_predicate(&exact, &tiny, x, &mut meter()).unwrap(),
        None
    );
    assert_eq!(
        publication_predicate(&between, &tiny, x, &mut meter()).unwrap(),
        Some(PublicationPredicate::SharperExact)
    );
    // Exact decimal equality uses a dyadic H with x=10^9 H; no float 1e-9.
    let x = 1_000_000_000.0 * pow2(-100);
    let relative = row(x, RowClass::RelativeVerified);
    assert_eq!(
        publication_predicate(
            &exact_sum(&[(1.0, -100)]),
            &relative,
            pow2(-36),
            &mut meter()
        )
        .unwrap(),
        None
    );
    assert_eq!(
        publication_predicate(
            &exact_sum(&[(1.0, -100), (1.0, -154)]),
            &relative,
            pow2(-36),
            &mut meter()
        )
        .unwrap(),
        Some(PublicationPredicate::PublicRelative)
    );
}

#[test]
fn pc13_16_thresholds_membership_and_original_operand_coupling() {
    let s = pow2(-988);
    assert_eq!(row_bound(0.0, next_down(s)).to_bits(), (1u64 << 22) + 1);
    assert_eq!(row_bound(0.0, s).to_bits(), 1u64 << 22);
    assert_eq!(row_bound(0.0, next_up(s)).to_bits(), (1u64 << 22) + 1);
    let t = pow2(-34);
    assert!(matches!(
        classify(next_down(t), 1.0),
        RowClass::AbsoluteVerified { .. }
    ));
    assert_eq!(classify(t, 1.0), RowClass::RelativeVerified);
    assert_eq!(classify(next_up(t), 1.0), RowClass::RelativeVerified);
    assert!(matches!(
        classify(1.0, pow2(-989)),
        RowClass::AbsoluteVerified { .. }
    ));
    let h = f64::from_bits(1);
    for (raw, length, slot) in [
        ([0.0, h, 0.0, 0.0], pow2(100), 0),
        ([h, 0.0, 0.0, 0.0], pow2(-100), 1),
        ([0.0, 0.0, h, 0.0], pow2(100), 3),
        ([0.0, 0.0, 0.0, h], pow2(-100), 2),
    ] {
        assert_eq!(coupled_scales(raw, length)[slot], pow2(-974));
    }
    assert_eq!(
        coupled_scales([2.0, 3.0, 5.0, 7.0], 0.0),
        [2.0, 3.0, 5.0, 7.0]
    );
    let mut layout = vec![meta(Kind::Translation), meta(Kind::Rotation)];
    layout[0].input_derived = true;
    let out = classify_rows(
        &layout,
        &[Binary64Outcome::Normal(1e100), Binary64Outcome::Normal(h)],
        &[1.0],
    );
    assert_eq!(out.body_scales[0].2, 1); // prescribed dominant value excluded
    layout[0].input_derived = false;
    let out = classify_rows(
        &layout,
        &[
            Binary64Outcome::Overflow { negative: false },
            Binary64Outcome::Normal(h),
        ],
        &[1.0],
    );
    assert_eq!(out.body_scales[0].2, 1); // O9 absent value excluded
    assert_eq!(out.rows[0].class, RowClass::Unpublishable);
    assert!(out.rows[0].value.value().is_none());
}

#[test]
fn pc17_20_exact_prescription_and_magnitude_term() {
    // Exact cancellation exposes a tail below the retained p value's spacing;
    // the prescribed publisher must not replace this with a p-rounded source.
    let terms = [(1.0, 1.0), (1.0, pow2(-100)), (-1.0, 1.0)];
    assert_eq!(exact_publication(&terms).value().unwrap(), pow2(-100));
    assert!(matches!(
        exact_publication(&[(f64::from_bits(1), 0.5)]),
        Binary64Outcome::Underflow { .. }
    ));
    assert!(matches!(
        exact_publication(&[(f64::MAX, 2.0)]),
        Binary64Outcome::Overflow { .. }
    ));
    let magnitude = QuantityMeta {
        id: QuantityId::DisplacementMagnitude(0),
        ..meta(Kind::Translation)
    };
    let mut r = report(256);
    r.w_plus[0] = Some(w(pow2(-200))); // prescribed component discrepancy retained
    let h = publication_h(0, &magnitude, 5.0, &w(5.0), &r, &mut meter()).unwrap();
    equal_sum(&h, &exact_sum(&[(1.0, -200), (5.0, -255)]));
}

struct PairFixture {
    prep: Arc<CasePrep>,
    group: Arc<GroupPrep>,
    cache: GroupCache,
    candidate: PrecisionState,
    verification: PrecisionState,
    report: BoundVerification,
    attempts: Vec<AttemptRecord>,
}

fn zero_pair(p: u32) -> PairFixture {
    let source = two_node_source(1.0, 1.0, 1.0, &[6], &[], &[]);
    let group = Arc::new(prepare_group(&source).unwrap());
    let prep = Arc::new(CasePrep::new(source).unwrap());
    let mut cache = GroupCache::default();
    let mut budget = CaseBudget {
        limit: u64::MAX,
        used: 0,
    };
    let mut invocation = InvocationMeter::new(u64::MAX);
    let (candidate, c) =
        solve_precision(p, &prep, &group, &mut cache, &mut budget, &mut invocation);
    let candidate = candidate.unwrap();
    let (verification, mut v) = solve_precision(
        p * 2,
        &prep,
        &group,
        &mut cache,
        &mut budget,
        &mut invocation,
    );
    let verification = verification.unwrap();
    v.role = AttemptRole::Verification;
    let report = verify_precision(
        &verification,
        &prep,
        &group,
        &mut cache,
        &mut budget,
        &mut invocation,
        &mut v,
    )
    .unwrap();
    PairFixture {
        prep,
        group,
        cache,
        candidate,
        verification,
        report,
        attempts: vec![c, v],
    }
}

fn assert_issue(result: Result<(), AttemptStop>, issue: CertificateIssue) {
    assert!(matches!(result,Err(AttemptStop::PublicationCertificate {issue:i,..}) if i==issue));
}

#[test]
fn pc25_30_pair_shape_precision_fields_and_nonfinite_metadata() {
    let mut f = zero_pair(128);
    validate_publication_pair(&f.prep, &f.candidate, &f.verification, &f.report).unwrap();
    let other = Arc::new(CasePrep::new(f.prep.source.clone()).unwrap());
    assert_issue(
        validate_publication_pair(&other, &f.candidate, &f.verification, &f.report),
        CertificateIssue::PairIdentity,
    );
    let second = zero_pair(128);
    assert_issue(
        validate_publication_pair(&f.prep, &f.candidate, &second.verification, &f.report),
        CertificateIssue::PairIdentity,
    );
    if let VerificationState::V256(r) = &mut f.report.report {
        Arc::make_mut(r).precision = 512;
    }
    assert_issue(
        validate_publication_pair(&f.prep, &f.candidate, &f.verification, &f.report),
        CertificateIssue::Precision,
    );
    if let VerificationState::V256(r) = &mut f.report.report {
        Arc::make_mut(r).precision = 256;
        Arc::make_mut(r).w_plus.pop();
    }
    assert_issue(
        validate_publication_pair(&f.prep, &f.candidate, &f.verification, &f.report),
        CertificateIssue::Shape,
    );
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0, -0.0] {
        assert!(meter().row_bound(0.0, bad).is_err());
        assert!(sharper_binary64(1.0, bad).is_err());
    }
    let f = zero_pair(128);
    if let VerificationState::V256(r) = &f.report.report {
        let mut bad_prep = CasePrep::new(f.prep.source.clone()).unwrap();
        bad_prep.layout[0].body = 99;
        assert_issue(
            report_shape(
                &bad_prep,
                bad_prep.layout.len(),
                bad_prep.layout.len(),
                128,
                256,
                r,
            ),
            CertificateIssue::RowIdentity,
        );
        let mut bad_prep = CasePrep::new(f.prep.source.clone()).unwrap();
        bad_prep.prescribed.clear();
        assert_issue(
            report_shape(
                &bad_prep,
                bad_prep.layout.len(),
                bad_prep.layout.len(),
                128,
                256,
                r,
            ),
            CertificateIssue::MissingField,
        );
    }
}

#[test]
fn pc21_24_floor_precision_kind_and_original_coupling() {
    let f = zero_pair(128);
    assert!(matches!(
        certify_publication(
            &f.prep,
            &f.candidate,
            &f.verification,
            &f.report,
            Some(&[[0.0, 0.0]]),
            StageGuard::unlimited()
        )
        .result,
        Err(AttemptStop::PublicationCertificate {
            issue: CertificateIssue::Shape,
            ..
        })
    ));
    let f = zero_pair(512);
    assert!(matches!(
        certify_publication(
            &f.prep,
            &f.candidate,
            &f.verification,
            &f.report,
            None,
            StageGuard::unlimited()
        )
        .result,
        Err(AttemptStop::PublicationCertificate {
            issue: CertificateIssue::Shape,
            ..
        })
    ));
    for floor in [
        [0.0, 0.0],
        [f64::from_bits(1), f64::from_bits(1)],
        [8.0, 16.0],
    ] {
        let spent = certify_publication(
            &f.prep,
            &f.candidate,
            &f.verification,
            &f.report,
            Some(&[floor]),
            StageGuard::unlimited(),
        );
        let Ok(PublicationDecision::Accepted(d)) = spent.result else {
            panic!("zero source with valid p512 floor");
        };
        assert_eq!(d.publication.body_scales[0].2, 0);
        assert_eq!(d.publication.body_scales[1].2, 0);
        assert_eq!(d.publication.body_scales[2].2, floor[0].to_bits());
        assert_eq!(d.publication.body_scales[3].2, floor[1].to_bits());
    }
    let metas = [meta(Kind::Force), meta(Kind::Moment)];
    let values = [Binary64Outcome::Normal(2.0), Binary64Outcome::Normal(1.0)];
    let pub_ = classify_rows_floored(&metas, &values, &[2.0], Some(&[[8.0, 0.0]]));
    assert_eq!(pub_.body_scales[2].2, 8.0f64.to_bits());
    assert_eq!(pub_.body_scales[3].2, 4.0f64.to_bits()); // not L*floor_force
}

#[test]
fn pc36_39_final_draft_move_drop_and_radius_identity() {
    let mut f = zero_pair(128);
    let decision = compare_states(
        &f.prep.layout,
        &f.prep.extents,
        &f.candidate,
        &f.verification,
        &f.report.report,
        StageGuard::unlimited(),
    );
    assert_eq!(decision.result, Ok(true));
    let references = Arc::strong_count(&f.prep);
    let spent = certify_publication(
        &f.prep,
        &f.candidate,
        &f.verification,
        &f.report,
        None,
        StageGuard::unlimited(),
    );
    let Ok(PublicationDecision::Accepted(draft)) = spent.result else {
        panic!("zero certificate");
    };
    assert_eq!(Arc::strong_count(&f.prep), references + 1);
    let expected = draft.publication.clone();
    let radii = draft.radii.clone();
    f.attempts[0].outcome = AttemptOutcome::Accepted;
    f.attempts[1].outcome = AttemptOutcome::Verified;
    let out = finish_selected(
        f.prep.clone(),
        f.group,
        f.cache,
        vec![f.candidate.clone(), f.verification.clone()],
        &f.candidate,
        256,
        f.attempts,
        &decision,
        &f.report.report,
        draft,
        vec![BodyGeometry::Restrained],
    );
    let CaseOutcome::Selected(mut solve) = out else {
        panic!("final move");
    };
    assert_eq!(solve.publication, expected);
    assert_eq!(solve.publication_radius_bits, radii);
    let i = solve
        .prep
        .layout
        .iter()
        .position(|m| !m.input_derived)
        .unwrap();
    let meta = solve.prep.layout[i];
    assert_eq!(
        solve.publication_radius(i, meta, b"wrong", 128),
        Err(CertificateIssue::PairIdentity)
    );
    assert_eq!(
        solve.publication_radius(i, meta, &solve.prep.identity, 256),
        Err(CertificateIssue::Precision)
    );
    for bad in [
        ABSENT_RADIUS_BITS,
        0x8000000000000000,
        0xbff0000000000000,
        0x7ff8000000000001,
    ] {
        solve.publication_radius_bits[i] = bad;
        assert!(solve
            .publication_radius(i, meta, &solve.prep.identity, 128)
            .is_err());
    }
    let mut f = zero_pair(128);
    if let VerificationState::V256(r) = &mut f.report.report {
        let index = f.prep.layout.iter().position(|m| !m.input_derived).unwrap();
        Arc::make_mut(r).w_plus[index] = Some(w(1.0));
    }
    let references = Arc::strong_count(&f.prep);
    let spent = certify_publication(
        &f.prep,
        &f.candidate,
        &f.verification,
        &f.report,
        None,
        StageGuard::unlimited(),
    );
    assert!(matches!(
        spent.result,
        Ok(PublicationDecision::Rejected { .. })
    ));
    drop(spent);
    assert_eq!(Arc::strong_count(&f.prep), references);
    let spent = certify_publication(
        &f.prep,
        &f.candidate,
        &f.verification,
        &f.report,
        None,
        StageGuard::with_case_room(0),
    );
    assert!(matches!(spent.result, Err(AttemptStop::Budget(_))));
    drop(spent);
    assert_eq!(Arc::strong_count(&f.prep), references);
}

#[test]
fn pc44_46_replay_and_fresh_combination_keep_owned_radii_and_policy() {
    use super::super::combine::{CombinationOutcome, RetainedCombination};
    let source = two_node_source(1.0, 1.0, 1.0, &[6], &[], &[]);
    let mut meter = InvocationMeter::new(u64::MAX);
    let results = solve_cases(
        &[source.clone(), source],
        CaseLimit::new(u64::MAX),
        &mut meter,
    );
    let (CaseOutcome::Selected(a), CaseOutcome::Selected(b)) = (&results[0], &results[1]) else {
        panic!("zero cases");
    };
    assert_eq!(a.publication, b.publication);
    assert_eq!(a.publication_radius_bits, b.publication_radius_bits);
    let outcome =
        RetainedCombination::solve(&[(1.0, a), (2.0, b)], CaseLimit::new(u64::MAX), &mut meter);
    let CombinationOutcome::Selected(c) = outcome else {
        panic!("zero combination");
    };
    assert_eq!(c.evidence.policy, POLICY);
    assert_eq!(a.publication, c.publication);
    assert_eq!(a.publication_radius_bits, c.publication_radius_bits);
    assert_ne!(
        a.publication_radius_bits.as_ptr(),
        c.publication_radius_bits.as_ptr()
    );
    assert_ne!(a.prep.identity, c.prep.identity);
}

// Independent extra-source equilibrium, frozen before output:
// FM/MF each has total scalar stiffness4 and three separate k=1 springs.
// This function maps every closed output key; no expected value comes from solve.
fn extra_truth(id: usize, row: &PublishedRow) -> Wide<4> {
    use super::super::recover::End;
    use super::super::source::Component;
    let q = w(f64::from_bits(5)).mul_pow2(-2).unwrap();
    let mode = if id == 1 {
        Component::Rx
    } else {
        Component::Ux
    };
    let action = if id == 2 { w(f64::from_bits(5)) } else { q };
    match row.id {
        QuantityId::Displacement(d) if d.node == 1 && d.component == mode => q,
        QuantityId::DisplacementMagnitude(1) if id != 1 => q,
        QuantityId::EndAction { end, component, .. } if component == mode => {
            if end == End::I {
                action.neg()
            } else {
                action
            }
        }
        QuantityId::SpringAction { component, .. } if id != 2 && component == mode => q.neg(),
        QuantityId::Reaction(d) if d.node == 0 && d.component == mode => action.neg(),
        _ => Wide::<4>::ZERO,
    }
}

#[test]
fn independent_fixed_extra_source_rows_never_escape_their_bare_claim() {
    let sources = [
        proposed_raw_force_control(),
        proposed_raw_moment_control(),
        proposed_free_zero_rotation_control(),
    ];
    for (id, source) in sources.into_iter().enumerate() {
        let expected_load = ["EXTRA-FM-01:load", "EXTRA-MF-01:load", "EXTRA-ZR-01:load"][id];
        assert_eq!(source.loads()[0].source_id, expected_load);
        let mut invocation = InvocationMeter::new(u64::MAX);
        let out = solve_case(source, CaseLimit::new(u64::MAX), &mut invocation);
        match out {
            CaseOutcome::Selected(s) => {
                assert_eq!(s.publication.rows.len(), if id == 2 { 36 } else { 40 });
                for row in &s.publication.rows {
                    let truth = extra_truth(id, row);
                    let x = row
                        .value
                        .value()
                        .expect("all frozen extra truths are value-bearing");
                    let mut error = ExactWideSum::new();
                    error.add_binary64(x, false).unwrap();
                    error.add_wide(&truth, true).unwrap();
                    error.make_absolute();
                    let scale = s.publication.body_scales[row.body as usize * 4 + row.kind.index()];
                    if row.class == RowClass::InputDerived {
                        assert_eq!(error.signum(), 0);
                    } else {
                        assert_eq!(
                            publication_predicate(
                                &error,
                                row,
                                f64::from_bits(scale.2),
                                &mut meter()
                            )
                            .unwrap(),
                            None
                        );
                    }
                }
            }
            CaseOutcome::Unresolved {
                reason: UnresolvedReason::Ceiling,
                attempts,
                ..
            } => {
                assert!(attempts.iter().any(|a| matches!(
                    a.outcome,
                    AttemptOutcome::Rejected(AttemptReason::PublicationEnclosure { .. })
                )));
                assert!(attempts.iter().all(|a| a.precision <= 1024));
                // Named nonselection satisfies no numerical-accuracy claim.
            }
            other => panic!("unexpected source/terminal standing: {other:?}"),
        }
    }
}

#[test]
fn pc01_03_complete_closed_kernel_row_error_families() {
    use super::super::recover::End;
    use super::super::source::Component;
    let rows = [
        (
            QuantityId::Displacement(Dof::from_global(0)),
            Kind::Translation,
        ),
        (
            QuantityId::Displacement(Dof::from_global(3)),
            Kind::Rotation,
        ),
        (QuantityId::DisplacementMagnitude(0), Kind::Translation),
        (
            QuantityId::EndAction {
                member: 1,
                end: End::I,
                component: Component::Ux,
            },
            Kind::Force,
        ),
        (
            QuantityId::EndAction {
                member: 1,
                end: End::J,
                component: Component::Rx,
            },
            Kind::Moment,
        ),
        (
            QuantityId::StationAction {
                station: 1,
                component: Component::Uy,
            },
            Kind::Force,
        ),
        (
            QuantityId::StationAction {
                station: 1,
                component: Component::Ry,
            },
            Kind::Moment,
        ),
        (
            QuantityId::SpringAction {
                spring: 1,
                component: Component::Uz,
            },
            Kind::Force,
        ),
        (
            QuantityId::SpringAction {
                spring: 1,
                component: Component::Rz,
            },
            Kind::Moment,
        ),
        (
            QuantityId::DirectionalSpringAction {
                spring: 1,
                component: Component::Ux,
            },
            Kind::Force,
        ),
        (
            QuantityId::DirectionalSpringAction {
                spring: 1,
                component: Component::Rx,
            },
            Kind::Moment,
        ),
        (QuantityId::Reaction(Dof::from_global(0)), Kind::Force),
        (QuantityId::Reaction(Dof::from_global(3)), Kind::Moment),
        (QuantityId::SupportForceMagnitude(1), Kind::Force),
        (QuantityId::SupportMomentMagnitude(1), Kind::Moment),
    ];
    for precision in [256, 512, 1024] {
        let mut r = report(precision);
        r.e_rows[0] = Some(w(1.0));
        r.w[0] = Some(w(2.0));
        r.charge[0] = Some(w(4.0));
        r.w_plus[0] = Some(w(2.0));
        for (id, kind) in rows {
            let meta = QuantityMeta {
                id,
                kind,
                body: 0,
                input_derived: false,
            };
            let actual = publication_h(0, &meta, 1.0, &w(1.0), &r, &mut meter()).unwrap();
            let expected = match kind {
                Kind::Force | Kind::Moment => exact_sum(&[(6.0, 0), (71.0, -i64::from(precision))]),
                _ if matches!(id, QuantityId::DisplacementMagnitude(_)) => {
                    exact_sum(&[(2.0, 0), (1.0, 1 - i64::from(precision))])
                }
                _ => exact_sum(&[(2.0, 0)]),
            };
            equal_sum(&actual, &expected);
        }
    }
}
