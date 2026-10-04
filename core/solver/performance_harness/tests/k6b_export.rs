//! K6b A0: the public path to K4's W1a method is sufficient (T3 K6b brief,
//! Scope 1; ROOT's rulings on K6b's A0 and V-K's Q8).
//!
//! Every exported name is imported from `structural::retained_api`, outside
//! FK, so an item missing from the facade fails this test's build. One invented
//! case is then built from `SourceParts`, solved through `solve_case`, and a
//! published row and an attempt's `StageWork` are read.
//!
//! Invented inputs, stated here: a cantilever of two 1 m members along x;
//! E = 2e11, G = 8e10, A = 0.01, Iy = Iz = 8e-6, J = 1.6e-5 (the harness's
//! invented section); node 0 fully restrained; a tip force Fy = -1000 at node 2.
//! No time or memory bound is asserted.

#[allow(unused_imports)]
use open_pipe_stress_frame_kernel::structural::retained_api::{
    absolute_bound, body_extent, classify, classify_rows, classify_rows_floored, coupled_scales,
    e_hat, intensified_k, layout, phi_512, resolution_hats, reverse_cuthill_mckee, solve_case,
    solve_cases, stress_scale, threshold, AttemptOutcome, AttemptReason, AttemptRecord,
    AttemptRole, AttemptStop, AttemptWork, Binary64Outcome, BodyGeometry, BudgetScope, CaseLimit,
    CaseOutcome, CertificateIssue, CombinationOutcome, CombinationReason, Component, Constraint,
    DirectionalSpring, Dof, End, GateTest, InvocationMeter, Kind, LedgerRefusal, MemberProperty,
    NodalLoad, PrimitiveSource, Publication, PublicationPredicate, PublishedRow, QuantityId,
    QuantityMeta, Refusal, RetainedCombination, RetainedEvidence, RetainedSolve, RowClass,
    SourceError, SourceParts, Spring, SpringKind, StageWork, Station, StorageCounts,
    StraightMember, SumWork, SupportGroup, UnresolvedReason, VerificationSummary, WideError,
    WidthWork, FLOOR_RATIO_BITS, K_SQRT2_BITS, K_TWO_SQRT2_BITS, METHOD_TOKEN, PHI_SCALE_BITS,
    POLICY, PRECISIONS, RCOND_LABEL,
};

const E: f64 = 2.0e11;
const G: f64 = 8.0e10;
const AREA: f64 = 0.01;
const I: f64 = 8.0e-6;
const J: f64 = 1.6e-5;
const TIP_FORCE: f64 = -1000.0;

fn member(id: u32, node_i: u32, node_j: u32) -> StraightMember {
    StraightMember {
        id,
        node_i,
        node_j,
        elastic_modulus: E,
        shear_modulus: G,
        area: AREA,
        second_moment_y: I,
        second_moment_z: I,
        torsion_constant: J,
        y_reference: [0.0, 1.0, 0.0],
    }
}

fn cantilever() -> PrimitiveSource {
    let parts = SourceParts {
        nodes: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
        members: vec![member(1, 0, 1), member(2, 1, 2)],
        constraints: Component::ALL
            .iter()
            .map(|&component| Constraint {
                dof: Dof { node: 0, component },
                value: 0.0,
            })
            .collect(),
        loads: vec![NodalLoad {
            dof: Dof {
                node: 2,
                component: Component::Uy,
            },
            value: TIP_FORCE,
            source_id: "tip".to_string(),
        }],
        stations: vec![Station {
            id: 1,
            member: 2,
            fraction: 0.5,
        }],
        ..SourceParts::default()
    };
    PrimitiveSource::new(parts).expect("the invented cantilever is a valid source")
}

#[test]
fn a_source_built_outside_fk_is_solved_and_read_through_the_public_path() {
    let source = cantilever();
    assert_eq!(source.dof_count(), 18);
    assert_eq!(source.free_dofs().len(), 12);
    let tip_uy = Dof {
        node: 2,
        component: Component::Uy,
    };
    assert_eq!(Dof::from_global(tip_uy.global()), tip_uy);
    assert_eq!(Component::from_index(Component::Uy.index()), Component::Uy);

    let mut meter = InvocationMeter::new(u64::MAX);
    let outcome = solve_case(source, CaseLimit::new(u64::MAX), &mut meter);
    let solve: Box<RetainedSolve> = match outcome {
        CaseOutcome::Selected(solve) => solve,
        other => panic!("the invented cantilever is not selected: {other:?}"),
    };
    assert!(PRECISIONS.contains(&solve.selected_precision()));
    assert!(meter.charged() > 0 && !meter.exhausted());

    // A published row: the tip deflection, against the exact Euler–Bernoulli
    // value P·L³/(3EI) = -1/600 m (a sanity check at 1e-9 relative, not the
    // row's claim).
    let publication: &Publication = solve.publish();
    let row: &PublishedRow = publication
        .rows
        .iter()
        .find(|r| r.id == QuantityId::Displacement(tip_uy))
        .expect("the tip deflection is published");
    assert_eq!(row.kind, Kind::Translation);
    assert!(matches!(
        row.class,
        RowClass::RelativeVerified | RowClass::AbsoluteVerified { .. }
    ));
    let value = row.value.value().expect("a binary64 value");
    let exact = -1.0 / 600.0;
    assert!((value - exact).abs() <= 1e-9 * exact.abs(), "{value}");

    // An attempt's work by stage and its storage counts.
    let evidence: &RetainedEvidence = solve.evidence();
    assert_eq!(evidence.method, METHOD_TOKEN);
    assert_eq!(evidence.policy, POLICY);
    assert_eq!(POLICY, "M03-INTEGRITY-MP-v2");
    let first: &AttemptRecord = &evidence.attempts[0];
    assert_eq!(first.precision, PRECISIONS[0]);
    let own: &StageWork = &first.stages;
    let shared: &StageWork = &first.shared_stages;
    assert!(own.solve > 0 && own.recovery > 0);
    assert!(shared.formation > 0 && shared.factor > 0);
    let storage: StorageCounts = first.storage;
    assert_eq!(storage.limbs_per_entry, 4);
    assert!(storage.pattern_entries > 0 && storage.profile_entries > 0);
    assert!(first.work.limb_multiply_equivalents() > 0);
    assert!(first.k4_work.limb_multiply_equivalents() > 0);
}

#[test]
fn publication_failure_reasons_are_typed_at_the_public_boundary() {
    let predicate = PublicationPredicate::AbsoluteBound;
    let reason = AttemptReason::PublicationEnclosure {
        quantity: QuantityId::Displacement(Dof::from_global(0)),
        body: 0,
        kind: Kind::Translation,
        predicate,
    };
    assert!(matches!(reason, AttemptReason::PublicationEnclosure { .. }));
    let stop = AttemptStop::PublicationCertificate {
        index: Some(0),
        issue: CertificateIssue::MissingField,
    };
    assert!(matches!(stop, AttemptStop::PublicationCertificate { .. }));
    let unresolved = UnresolvedReason::PublicationCertificate {
        index: None,
        issue: CertificateIssue::PairIdentity,
    };
    assert!(matches!(
        unresolved,
        UnresolvedReason::PublicationCertificate { .. }
    ));
}

#[test]
fn checked_work_public_consumers_refuse_unavailable_stage_records() {
    use open_pipe_stress_frame_kernel::structural::retained_api::WorkFault;
    use open_pipe_stress_solver_performance_harness::k6::w1::staged::{
        attempts_of, prefix_limits, segments, stages_complete, stages_equal_totals,
        validate_attempts, work_by_precision, work_closes,
    };
    let mut meter = InvocationMeter::new(u64::MAX);
    let outcome = solve_case(cantilever(), CaseLimit::new(u64::MAX), &mut meter);
    let mut attempts = attempts_of(&outcome).to_vec();
    validate_attempts(&attempts).unwrap();
    let mut unavailable = StageWork::default();
    unavailable.formation = u64::MAX;
    unavailable.assembly = 1;
    assert_eq!(
        unavailable.checked_total().exact(),
        Err(WorkFault::Overflow)
    );
    assert_eq!(
        attempts[0].stages.merge(&unavailable),
        Err(WorkFault::Overflow)
    );
    assert_eq!(validate_attempts(&attempts), Err(WorkFault::Overflow));
    assert!(work_by_precision(&attempts).is_err());
    assert!(segments(&attempts).is_err());
    assert!(prefix_limits(&attempts).is_err());
    assert!(!stages_equal_totals(&attempts));
    assert!(!stages_complete(&attempts[0]));
    assert!(!work_closes(&attempts, meter.checked_charged()));
}
