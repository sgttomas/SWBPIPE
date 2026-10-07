//! K4 tests of `retained/factor.rs` (the brief's F): geometry first, the
//! ordering, the pivot, negative-energy and condition screens. The bounded
//! refinement's tests use the schedule's internals and sit in
//! `adaptive_tests.rs`.
use super::super::adaptive::{
    solve_case, AttemptOutcome, AttemptReason, AttemptRole, AttemptStop, CaseLimit, CaseOutcome,
    InvocationMeter, Refusal, StageGuard,
};
use super::super::assemble::Structure;
use super::super::source::{
    Component, Constraint, DirectionalSpring, Dof, NodalLoad, PrimitiveSource, SourceParts,
    SpringKind, StraightMember,
};
use super::super::wide::multi::SupportedWidth;
use super::super::wide::Wide;
use super::super::wide_sum::ExactWideSum;
use super::*;

#[allow(dead_code)]
#[path = "models.rs"]
mod models;
#[allow(dead_code)]
#[path = "support.rs"]
mod support;
use support::ctx;

fn solve(source: PrimitiveSource) -> CaseOutcome {
    let mut meter = InvocationMeter::new(u64::MAX);
    solve_case(source, CaseLimit::new(u64::MAX), &mut meter)
}

// ---------------------------------------------------------------- ordering

#[test]
fn the_rcm_port_orders_sparse_directs_small_graphs_as_sparse_direct_does() {
    // sparse_direct/src/lib.rs's own RCM tests (their graphs and orders).
    let cases: [(Vec<Vec<usize>>, Vec<usize>); 4] = [
        (
            vec![vec![1], vec![0, 2], vec![1, 3], vec![2]],
            vec![3, 2, 1, 0],
        ),
        (
            vec![vec![1, 2, 3], vec![0], vec![0], vec![0]],
            vec![3, 2, 0, 1],
        ),
        (
            vec![vec![1], vec![0], vec![], vec![4], vec![3]],
            vec![4, 3, 2, 1, 0],
        ),
        (
            vec![vec![1, 1], vec![2], vec![], vec![2, 2]],
            vec![3, 2, 1, 0],
        ),
    ];
    for (adjacency, order) in cases {
        assert_eq!(reverse_cuthill_mckee(&adjacency), order, "{adjacency:?}");
    }
    // sparse_direct's scrambled path 0-2-4-1-3: the RCM order walks the path.
    let adjacency = vec![vec![2], vec![4, 3], vec![0, 4], vec![1], vec![2, 1]];
    let order = reverse_cuthill_mckee(&adjacency);
    let mut rank = [0usize; 5];
    for (k, &node) in order.iter().enumerate() {
        rank[node] = k;
    }
    for (a, list) in adjacency.iter().enumerate() {
        for &b in list {
            assert_eq!(rank[a].abs_diff(rank[b]), 1, "{order:?}");
        }
    }
}

// ---------------------------------------------------------------- constructed matrices

/// Nodes (k, 0, 0), members k–k+1, every DOF but UX restrained: the free–free
/// pattern is a path over the UX DOFs (a carrier for constructed values).
fn path_source(n: usize) -> PrimitiveSource {
    let member = |id: u32| StraightMember {
        id,
        node_i: id - 1,
        node_j: id,
        elastic_modulus: 1.0,
        shear_modulus: 1.0,
        area: 1.0,
        second_moment_y: 1.0,
        second_moment_z: 1.0,
        torsion_constant: 1.0,
        y_reference: [0.0, 1.0, 0.0],
    };
    let mut parts = SourceParts {
        nodes: (0..n).map(|k| [k as f64, 0.0, 0.0]).collect(),
        members: (1..n as u32).map(member).collect(),
        ..Default::default()
    };
    for node in 0..n as u32 {
        for c in 1..6 {
            parts.constraints.push(Constraint {
                dof: Dof {
                    node,
                    component: Component::from_index(c),
                },
                value: 0.0,
            });
        }
    }
    PrimitiveSource::new(parts).unwrap()
}

/// K on the path's free pattern with K(i, j) = value(rank i, rank j) in the
/// RCM order (every other stored entry zero).
fn constructed<const L: usize>(
    source: &PrimitiveSource,
    value: impl Fn(usize, usize) -> f64,
) -> (Structure, Ordering, Vec<Wide<L>>)
where
    Wide<L>: SupportedWidth,
{
    let structure = Structure::new(source).unwrap();
    let ordering = order_free(source, &structure);
    let mut k = vec![Wide::<L>::ZERO; structure.entry_count()];
    for (r, c, index) in structure.entries() {
        let (a, b) = (ordering.position[r], ordering.position[c]);
        if a != usize::MAX && b != usize::MAX {
            k[index] = support::lift::<L>(value(ordering.rank[a], ordering.rank[b]));
        }
    }
    (structure, ordering, k)
}

/// K = L D Lᵀ in the RCM order with D = I and L unit lower bidiagonal with
/// subdiagonal −2: K(0,0) = 1, K(i,i) = 5, K(i,i−1) = −2. Every pivot is
/// exactly 1 at any p, and κ(K) grows like 4^n.
fn bidiagonal_product(i: usize, j: usize) -> f64 {
    match (i, j) {
        (0, 0) => 1.0,
        _ if i == j => 5.0,
        _ if i.abs_diff(j) == 1 => -2.0,
        _ => 0.0,
    }
}

#[test]
fn a_constructed_ill_conditioned_matrix_whose_pivots_pass_escalates_on_rcond_at_128_only() {
    let source = path_source(70);
    let guard = StageGuard::unlimited();
    let mut sum = ExactWideSum::new();
    // p = 128: every pivot passes, the condition screen escalates.
    let (structure, ordering, k) = constructed::<4>(&source, bidiagonal_product);
    assert_eq!(ordering.free.len(), 70);
    let mut c128 = ctx::<4>(128);
    let f = factor(&mut c128, &mut sum, &guard, &structure, &k, &ordering).unwrap();
    assert_eq!(f.screens.len(), 70);
    assert_eq!(
        f.condition(&mut c128, &mut sum, &structure, &k, &ordering),
        Err(AttemptStop::Condition)
    );
    // p = 256: the same matrix passes, with rcond far below 2^-127.
    let mut c256 = ctx::<4>(256);
    let f = factor(&mut c256, &mut sum, &guard, &structure, &k, &ordering).unwrap();
    let rcond = f
        .condition(&mut c256, &mut sum, &structure, &k, &ordering)
        .unwrap()
        .to_binary64()
        .value()
        .unwrap();
    assert!(rcond > 0.0 && rcond < support::pow2(-127), "{rcond:e}");
    // A well-conditioned control (n = 20) passes at 128.
    let small = path_source(20);
    let (structure, ordering, k) = constructed::<4>(&small, bidiagonal_product);
    let f = factor(&mut c128, &mut sum, &guard, &structure, &k, &ordering).unwrap();
    assert!(f
        .condition(&mut c128, &mut sum, &structure, &k, &ordering)
        .is_ok());
}

#[test]
fn a_constructed_indefinite_pair_is_a_negative_energy_witness_and_its_definite_neighbour_is_not() {
    let source = path_source(2);
    let guard = StageGuard::unlimited();
    let mut sum = ExactWideSum::new();
    let mut c = ctx::<4>(128);
    // K11 = K22 = 1, K12 = 2: E = 1 + 1 − 4 < 0.
    let (structure, ordering, k) = constructed::<4>(&source, |i, j| if i == j { 1.0 } else { 2.0 });
    assert_eq!(
        negative_pair(&mut sum, &structure, &k, &ordering, 128),
        Ok(Some((0, 6)))
    );
    assert_eq!(
        factor(&mut c, &mut sum, &guard, &structure, &k, &ordering).err(),
        Some(AttemptStop::NegativeEnergy { i: 0, j: 6 })
    );
    // K12 = 0.75: definite, no witness, the factor passes.
    let (structure, ordering, k) =
        constructed::<4>(&source, |i, j| if i == j { 1.0 } else { 0.75 });
    assert_eq!(
        negative_pair(&mut sum, &structure, &k, &ordering, 128),
        Ok(None)
    );
    assert!(factor(&mut c, &mut sum, &guard, &structure, &k, &ordering).is_ok());
    // K12 = 1 exactly (E = 0, singular): a failed pivot, never a witness.
    let (structure, ordering, k) = constructed::<4>(&source, |i, j| if i == j { 1.0 } else { 1.0 });
    assert_eq!(
        negative_pair(&mut sum, &structure, &k, &ordering, 128),
        Ok(None)
    );
    assert!(matches!(
        factor(&mut c, &mut sum, &guard, &structure, &k, &ordering),
        Err(AttemptStop::Pivot { .. })
    ));
}

// ---------------------------------------------------------------- the pivot screen through the schedule

#[test]
fn a_pivot_that_fails_at_128_escalates_and_passes_at_256() {
    let m = models::model("PIVOT");
    let source = m.source();
    let CaseOutcome::Selected(solve) = solve(source.clone()) else {
        panic!("PIVOT not selected")
    };
    assert_eq!(solve.selected_precision(), 256);
    let attempts = &solve.evidence().attempts;
    assert!(matches!(
        attempts[0].outcome,
        AttemptOutcome::Failed(AttemptReason::Stop(AttemptStop::Pivot { .. }))
    ));
    assert_eq!(attempts[0].precision, 128);
    assert_eq!(
        (attempts[1].precision, &attempts[1].outcome),
        (256, &AttemptOutcome::Accepted)
    );
    assert_eq!(
        (attempts[2].precision, &attempts[2].outcome),
        (512, &AttemptOutcome::Verified)
    );
    let (worst, at, compared) = models::compare(&source, &solve.publish().rows, &m.expect);
    assert!(
        compared > 20 && worst <= 1.0,
        "{worst} at {at} ({compared})"
    );
}

// ---------------------------------------------------------------- geometry first

#[test]
fn witnessed_mechanisms_are_refused_before_any_attempt_and_the_rx_companion_solves() {
    for name in ["N02", "N03-RZ", "N04"] {
        match solve(models::model(name).source()) {
            CaseOutcome::Refused {
                refusal: Refusal::MechanismWitnessed { .. },
                geometry,
            } => assert!(geometry.is_empty(), "{name}"),
            other => panic!("{name}: {other:?}"),
        }
        // Geometry first decides without a factor.
        assert!(matches!(
            geometry_first(&models::model(name).source()),
            Err(GeometryRefusal::MechanismWitnessed { .. })
        ));
    }
    match solve(models::model("N03-RX").source()) {
        CaseOutcome::Selected(solve) => {
            assert_eq!(solve.selected_precision(), 128);
            assert_eq!(solve.evidence().geometry, vec![BodyGeometry::Restrained]);
        }
        other => panic!("N03-RX: {other:?}"),
    }
}

#[test]
fn the_span_decision_is_exact() {
    // det = 2^-104 exactly; a binary64 rule-of-Sarrus evaluation gives 0.
    let e = f64::EPSILON;
    let (a, b, c) = ([1.0, 1.0, 1.0], [1.0, 1.0 + e, 1.0], [1.0, 1.0, 1.0 + e]);
    let sarrus = a[0] * b[1] * c[2] + a[1] * b[2] * c[0] + a[2] * b[0] * c[1]
        - a[0] * b[2] * c[1]
        - a[1] * b[0] * c[2]
        - a[2] * b[1] * c[0];
    assert_eq!(sarrus, 0.0);
    assert!(spans_space(&[a, b, c]));
    assert!(!spans_space(&[
        [1.0, 1.0, 1.0],
        [2.0, 2.0, 2.0],
        [1.0, 2.0, 3.0]
    ]));
    assert!(!spans_space(&[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]]));
    assert!(spans_space(&[
        [1.0, 1.0, 0.0],
        [1.0, -1.0, 0.0],
        [0.0, 0.0, 1.0]
    ]));
}

/// A cantilever whose root is restrained in UZ and every rotation, and
/// grounded in the plane by directional translation springs.
fn directional_root(directions: &[[f64; 3]]) -> PrimitiveSource {
    let mut parts = SourceParts {
        nodes: vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
        members: vec![StraightMember {
            id: 1,
            node_i: 0,
            node_j: 1,
            elastic_modulus: 200e9,
            shear_modulus: 80e9,
            area: 6e-3,
            second_moment_y: 2.7e-5,
            second_moment_z: 2.7e-5,
            torsion_constant: 5.4e-5,
            y_reference: [0.0, 0.0, 1.0],
        }],
        ..Default::default()
    };
    for c in 2..6 {
        parts.constraints.push(Constraint {
            dof: Dof {
                node: 0,
                component: Component::from_index(c),
            },
            value: 0.0,
        });
    }
    for (k, d) in directions.iter().enumerate() {
        parts.directional_springs.push(DirectionalSpring {
            id: k as u32 + 1,
            node: 0,
            kind: SpringKind::Translation,
            direction: *d,
            stiffness: 1e6,
        });
    }
    parts.loads.push(NodalLoad {
        dof: Dof {
            node: 1,
            component: Component::Uy,
        },
        value: 1.0,
        source_id: "f".into(),
    });
    PrimitiveSource::new(parts).unwrap()
}

#[test]
fn a_directional_ground_that_does_not_span_is_not_assessed_and_is_never_refused_as_a_mechanism() {
    // (1, 1, 0) with the restrained UZ spans a plane: the in-plane direction
    // (1, −1, 0) is ungrounded. The body is not assessed (no witness sought)
    // and proceeds as a numerically unresolved body: it is unresolved at the
    // ceiling, never refused.
    let source = directional_root(&[[1.0, 1.0, 0.0]]);
    assert_eq!(
        geometry_first(&source),
        Ok(vec![BodyGeometry::NotAssessed(vec![(
            0,
            SpringKind::Translation
        )])])
    );
    match solve(source) {
        CaseOutcome::Unresolved {
            reason,
            attempts,
            geometry,
        } => {
            assert_eq!(reason, super::super::adaptive::UnresolvedReason::Ceiling);
            assert_eq!(
                geometry,
                vec![BodyGeometry::NotAssessed(vec![(
                    0,
                    SpringKind::Translation
                )])]
            );
            assert_eq!(
                attempts.iter().map(|a| a.precision).collect::<Vec<_>>(),
                vec![128, 256, 512]
            );
            for a in &attempts {
                assert!(
                    matches!(a.outcome, AttemptOutcome::Failed(AttemptReason::Stop(ref s)) if s.escalates()),
                    "{:?}",
                    a.outcome
                );
            }
        }
        other => panic!("{other:?}"),
    }
    // A second direction spans R³ with UZ: restrained, solved at 128.
    let source = directional_root(&[[1.0, 1.0, 0.0], [1.0, -1.0, 0.0]]);
    assert_eq!(geometry_first(&source), Ok(vec![BodyGeometry::Restrained]));
    match solve(source) {
        CaseOutcome::Selected(solve) => assert_eq!(solve.selected_precision(), 128),
        other => panic!("{other:?}"),
    }
}

// ---------------------------------------------------------------- refinement evidence and the ceiling

#[test]
fn the_p_plus_64_residual_takes_one_correction_on_two_span_and_records_its_basis() {
    // K4-M11's control (ROOT's O5: an evidence-level kill). The shared node's
    // rotational entry is formed at p from two cancelling spans; the residual
    // at p + 64 sees the formation error and one correction removes it.
    let CaseOutcome::Selected(solve) = solve(models::model("TWO-SPAN").source()) else {
        panic!()
    };
    let attempts = &solve.evidence().attempts;
    assert_eq!(attempts.len(), 2);
    assert_eq!(
        attempts
            .iter()
            .map(|a| (a.precision, a.residual_basis, a.corrections))
            .collect::<Vec<_>>(),
        vec![(128, 192, 1), (256, 320, 1)]
    );
}

#[test]
fn the_ceiling_attempt_uses_its_own_k_as_the_residual_basis() {
    let CaseOutcome::Selected(solve) = solve(models::model("SKEW-K1E-60").source()) else {
        panic!()
    };
    assert_eq!(solve.selected_precision(), 512);
    let last = solve.evidence().attempts.last().unwrap();
    assert_eq!(
        (
            last.precision,
            last.residual_basis,
            last.role,
            &last.outcome
        ),
        (
            1024,
            1024,
            AttemptRole::Verification,
            &AttemptOutcome::Verified
        )
    );
    let selected = &solve.evidence().attempts[2];
    assert_eq!((selected.precision, selected.residual_basis), (512, 576));
}
