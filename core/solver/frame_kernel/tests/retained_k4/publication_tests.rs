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
        &[(6, f64::from_bits(5), "F-M:tip-force")],
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
        &[(9, f64::from_bits(5), "M-F:tip-moment")],
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
        &[(6, f64::from_bits(5), "ZERO-ROT:tip-force")],
        &[],
    )
}
