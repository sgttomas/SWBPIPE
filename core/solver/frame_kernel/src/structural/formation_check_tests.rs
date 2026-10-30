//! Kernel-level K-D5 tests (the adapter's tests cover both solve modes and
//! the product models; `nonlinear_integration/src/structural_adapter/kd5_tests.rs`).
use super::super::{
    factor_structural_cholesky, prepare_formation_checked_structural, solve_structural_dense,
    SolveQuality, StructuralSystem, SymmetryEvidence,
};
use super::*;

use crate::{FrameNode, FrameSection, Matrix12};

fn input_ledger(values: &[f64]) -> crate::load_ledger::AssembledForce {
    let mut ledger = crate::load_ledger::LoadLedger::new();
    for (dof, &value) in values.iter().enumerate() {
        ledger.push("test input", dof, value);
    }
    ledger.finish(values.len()).unwrap()
}
/// Preserve the frozen represented matrix while supplying today's required typed ledger/block trace.
fn closed_test_solve(
    system: StructuralSystem<'_>,
    source: &FormationSource,
) -> super::super::StructuralSolution {
    use super::super::StiffnessContribution;
    let force = input_ledger(system.force);
    let mut contributions = Vec::new();
    let endpoints = source
        .frames
        .iter()
        .map(|e| (e.node_i.index, e.node_j.index))
        .chain(
            source
                .connectors
                .iter()
                .map(|e| (e.node_i().index, e.node_j().index)),
        )
        .chain(source.curved.iter().map(|e| (e.node_i, e.node_j)));
    for (ni, nj) in endpoints {
        let map = element_dof_map(ni, nj);
        for r in 0..12 {
            for c in 0..12 {
                let mut value = system.stiffness[map[r]][map[c]];
                if r == c {
                    for &(dof, k) in &source.springs {
                        if dof == map[r] {
                            value -= k;
                        }
                    }
                }
                contributions.push(StiffnessContribution {
                    row: map[r],
                    col: map[c],
                    value,
                });
            }
        }
    }
    for &(dof, value) in &source.springs {
        contributions.push(StiffnessContribution {
            row: dof,
            col: dof,
            value,
        });
    }
    if contributions.is_empty() {
        for (r, row) in system.stiffness.iter().enumerate() {
            for (c, &value) in row.iter().enumerate() {
                if value != 0.0 {
                    contributions.push(StiffnessContribution {
                        row: r,
                        col: c,
                        value,
                    });
                }
            }
        }
    }
    let checked = StructuralSystem::assembled(
        system.stiffness,
        &force,
        system.free_dofs,
        system.prescribed,
        Some(&contributions),
        system.symmetry,
    )
    .with_formation_source(source);
    super::super::solve_formation_checked_structural_dense(&checked).unwrap()
}

fn round(w: &Wide2) -> f64 {
    let mut acc = ExactAccumulator::new();
    for &t in w.split_binary64().unwrap().terms() {
        acc.add(t).unwrap();
    }
    acc.round().unwrap()
}

fn skew_frame() -> FrameElement {
    let pi = std::f64::consts::PI;
    let (od, id) = (0.2_f64, 0.18_f64);
    let i4 = pi * ((od * od) * (od * od) - (id * id) * (id * id)) / 64.0;
    let section = FrameSection::new(
        200e9,
        80e9,
        pi * (od * od - id * id) / 4.0,
        i4,
        i4,
        2.0 * i4,
    )
    .unwrap();
    FrameElement::new(
        FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap(),
        FrameNode::new(1, [1.0, 2.0, 2.0]).unwrap(),
        section,
        [1.0, 0.0, 0.0],
    )
    .unwrap()
}

#[test]
fn kd5_frame_reformation_agrees_with_the_binary64_formation_to_roundoff() {
    let e = skew_frame();
    let mut arith = WideArith::new(FORMATION_PRECISION).unwrap();
    let wide = frame_matrix(&mut arith, &e).unwrap();
    let binary64: Matrix12 = e.global_stiffness().unwrap();
    let scale = binary64
        .iter()
        .flatten()
        .fold(0.0_f64, |m, v| m.max(v.abs()));
    let mut differs = false;
    for r in 0..12 {
        for c in 0..12 {
            let w = round(&wide[r][c]);
            assert!(
                (w - binary64[r][c]).abs() <= 1e-13 * scale,
                "[{r}][{c}] {w} {}",
                binary64[r][c]
            );
            differs |= w != binary64[r][c];
        }
    }
    // The re-formation is not the binary64 matrix (nothing is shared with it).
    assert!(differs);
    assert!(arith.work().rounded_operations() > 0);
}

#[test]
fn kd5_reformed_elements_have_the_rigid_body_null_space() {
    // Frame, objective connector and curved bend: rigid translation (1,−2,0.5) and rotation ω = (0.25,0.5,−0.125) (dyadic, so u is exact) about the
    // origin: K_e·u_rigid is zero to p for the re-formed frame and connector.
    // T4-U3: the connector carries offsets, a skew Q (Q.x along r) and a
    // coupled K (case 22's H), on the frame's nodes.
    let e = skew_frame();
    let (third, two_thirds) = (1.0 / 3.0, 2.0 / 3.0);
    let connector = ObjectiveConnector::new(
        e.node_i,
        e.node_j,
        crate::connector::ConnectorAttachment::global([0.125, -0.25, 0.375]),
        crate::connector::ConnectorAttachment::global([0.375, 0.25, 0.875]),
        [
            [third, two_thirds, -two_thirds],
            [two_thirds, third, two_thirds],
            [two_thirds, -two_thirds, -third],
        ],
        crate::connector::ScaledWorkMatrix {
            upper_triangle: [
                12500.0, 625.0, 0.0, 0.0, 500.0, 0.0, 9375.0, 312.5, 0.0, 0.0, -750.0, 7500.0,
                250.0, 0.0, 0.0, 800.0, 50.0, 0.0, 900.0, 100.0, 1200.0,
            ],
            translation_scale: 0.25,
        },
        [0.0; 6],
    )
    .unwrap();
    let mut arith = WideArith::new(FORMATION_PRECISION).unwrap();
    let omega = [0.25, 0.5, -0.125];
    let t = [1.0, -2.0, 0.5];
    let motion = |x: [f64; 3]| {
        let r = [
            omega[1] * x[2] - omega[2] * x[1],
            omega[2] * x[0] - omega[0] * x[2],
            omega[0] * x[1] - omega[1] * x[0],
        ];
        [
            t[0] + r[0],
            t[1] + r[1],
            t[2] + r[2],
            omega[0],
            omega[1],
            omega[2],
        ]
    };
    let mut u = [0.0; 12];
    u[..6].copy_from_slice(&motion(e.node_i.coordinates));
    u[6..].copy_from_slice(&motion(e.node_j.coordinates));
    // A realized bend with dyadic geometry (so u is exact), E1's section:
    // chord (1, 2, 2) of length 3, R = 1.625, bowing toward (−0.5, 0.25, 0)
    // projected off the chord; included angle about 135 degrees.
    let bend = CurvedFormation {
        node_i: 0,
        node_j: 1,
        coordinates_i: e.node_i.coordinates,
        coordinates_j: e.node_j.coordinates,
        radius: 1.625,
        y_reference: [-0.5, 0.25, 0.0],
        elastic_modulus: 200e9,
        shear_modulus: 80e9,
        area: e.section.area,
        second_moment: e.section.second_moment_y,
        torsion_constant: e.section.torsion_constant,
        in_plane_flexibility_factor: 5.0,
        out_of_plane_flexibility_factor: 5.0,
    };
    let before = arith.work().rounded_operations();
    let curved = curved_matrix(&mut arith, &bend).unwrap();
    let cost = arith.work().rounded_operations() - before;
    eprintln!(
        "kd5 curved re-formation cost: {cost} rounded Wide<2> operations, {} arctangent",
        1
    );
    assert!(cost < 5000, "{cost}");
    for k in [
        frame_matrix(&mut arith, &e).unwrap(),
        connector_matrix(&mut arith, &connector).unwrap(),
        curved,
    ] {
        let scale = k
            .iter()
            .flatten()
            .map(|v| round(v).abs())
            .fold(0.0, f64::max);
        for row in &k {
            let mut acc = ExactAccumulator::new();
            for (kv, &x) in row.iter().zip(&u) {
                kv.add_product_to(&mut acc, x).unwrap();
            }
            assert!(acc.round().unwrap().abs() <= 1e-25 * scale);
        }
    }
}

/// One node, DOF 0 free on a spring of 2; DOFs 1–5 prescribed at zero.
fn spring_system(force: &[f64]) -> (Vec<Vec<f64>>, FormationSource<'static>) {
    let mut k = vec![vec![0.0; 6]; 6];
    k[0][0] = 2.0;
    let source = FormationSource {
        node_count: 1,
        springs: vec![(0, 2.0)],
        ..FormationSource::default()
    };
    assert_eq!(force.len(), 6);
    (k, source)
}

#[test]
fn kd5_zero_scale_clause_fires_on_ledger_terms_that_differ_from_the_solve_force() {
    // N-2: unreachable in the product; here the ledger terms net 1 while the
    // force the solve used is 0, so u = 0 (S* = 0) and w = K⁻¹ρ = 1/2 ≠ 0.
    let force = [0.0; 6];
    let (k, source) = spring_system(&force);
    let prescribed = [(1, 0.0), (2, 0.0), (3, 0.0), (4, 0.0), (5, 0.0)];
    let system = StructuralSystem {
        stiffness: &k,
        force: &force,
        free_dofs: &[0],
        prescribed: &prescribed,
        contributions: None,
        contribution_groups: None,
        symmetry: None,
    };
    let checked = StructuralSystem {
        stiffness: &k,
        force: &force,
        free_dofs: &[0],
        prescribed: &prescribed,
        contributions: None,
        contribution_groups: None,
        symmetry: None,
    }
    .with_formation_source(&source);
    let prepared = prepare_formation_checked_structural(&checked).unwrap();
    let factor = factor_structural_cholesky(&prepared).unwrap();
    let solution = super::super::finish_structural(&factor).unwrap();
    assert_eq!(solution.displacements, vec![0.0; 6]);
    assert_eq!(solution.report.quality, SolveQuality::Sensitive);
    assert!(
        matches!(solution.formation_check.as_ref().unwrap().reason,FormationCheckReason::FormationCheckUnavailable{ref detail} if detail=="load closure: no ledger")
    );
    let make = |last: Option<f64>| {
        let mut l = crate::load_ledger::LoadLedger::new();
        l.push("ledger", 0, 1e80);
        l.push("ledger", 0, -1e80);
        if let Some(v) = last {
            l.push("ledger", 0, v);
        }
        l.finish(6).unwrap()
    };
    let differing = make(Some(1.0));
    let cancelling = make(None);
    let contributions = [super::super::StiffnessContribution {
        row: 0,
        col: 0,
        value: 2.0,
    }];
    let system = StructuralSystem {
        contributions: Some(&contributions),
        ..system
    };
    let condition =
        super::super::ConditionMatrix::estimate_condition(prepared.matrix(), &|r| factor.solve(r))
            .unwrap();

    let solve = |r: &[f64]| factor.solve(r);
    let fired = check(
        &system,
        &source,
        Some(&differing),
        prepared.scale_exponents(),
        &solution.displacements,
        condition,
        &|i, j| k[i][j],
        &|r, c| prepared.matrix()[r][c],
        &solve,
    )
    .0
    .expect("zero-scale clause");
    assert_eq!(fired.reason, FormationCheckReason::Estimate);
    assert_eq!(fired.global_dof, Some(0));
    assert_eq!(fired.scale, 0.0);
    assert_eq!(fired.ratio, f64::INFINITY);
    assert!((fired.doubled_correction - 1.0).abs() < 1e-15, "{fired:?}");
    // Control: terms whose exact net equals the solve's force do not fire.
    assert_eq!(
        check(
            &system,
            &source,
            Some(&cancelling),
            prepared.scale_exponents(),
            &solution.displacements,
            condition,
            &|i, j| k[i][j],
            &|r, c| prepared.matrix()[r][c],
            &solve
        )
        .0,
        None
    );
}

#[test]
fn kd5_unavailable_family_and_wide_error_fail_closed_and_never_err() {
    let force = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let (k, mut source) = spring_system(&force);
    let prescribed = [(1, 0.0), (2, 0.0), (3, 0.0), (4, 0.0), (5, 0.0)];
    let make = |source: &FormationSource| {
        let system = StructuralSystem {
            stiffness: &k,
            force: &force,
            free_dofs: &[0],
            prescribed: &prescribed,
            contributions: None,
            contribution_groups: None,
            symmetry: None,
        };
        let plain = solve_structural_dense(&system).unwrap();
        let result = closed_test_solve(system, source);
        (plain, result)
    };
    // Precondition: the exact spring re-forms and the case stays Passed.
    let (plain, result) = make(&source);
    assert_eq!(plain.report.quality, SolveQuality::Passed);
    assert_eq!(result.formation_check, None);
    assert_eq!(plain.displacements, result.displacements);
    assert_eq!(plain.report.quality, result.report.quality);
    assert!(result.member_bound.is_some());
    // A listed family the check cannot re-form demotes (D5C-2).
    source.unavailable.push("seeded_family".into());
    let (plain, result) = make(&source);
    assert_eq!(result.report.quality, SolveQuality::Sensitive);
    assert_eq!(result.displacements, plain.displacements);
    assert_eq!(
        result.formation_check.unwrap().reason,
        FormationCheckReason::FormationCheckUnavailable {
            detail: "seeded_family".into()
        }
    );
    // A formation source whose DOF map does not match fails closed.
    let bad = FormationSource {
        node_count: 2,
        ..FormationSource::default()
    };
    let (_, result) = make(&bad);
    assert_eq!(result.report.quality, SolveQuality::Sensitive);
    assert!(matches!(
        result.formation_check.unwrap().reason,
        FormationCheckReason::FormationCheckUnavailable { .. }
    ));
}

#[test]
fn kd5_curved_arctangent_domain_errors_fail_closed() {
    // AngleDomain refusals inside the check, each a WideError: the case is
    // demoted with formation_check_unavailable, never passed and never an Err.
    // With the (d, R, y) inputs of T4-U1 an arc at or beyond π is one whose
    // radius does not exceed half the chord (4R² − L² ≤ 0 at p):
    // (a) R = L/2 exactly (φ = π; addendum 3, RV2 N5's limit);
    // (b) R < L/2 (no arc). The product refuses both before forming a bend
    //     (and any bend within 1e-9 of π); the kernel path fails closed anyway.
    let bend = |xi: [f64; 3], xj: [f64; 3], radius: f64| CurvedFormation {
        node_i: 0,
        node_j: 1,
        coordinates_i: xi,
        coordinates_j: xj,
        radius,
        y_reference: [0.0, 1.0, 0.0],
        elastic_modulus: 200e9,
        shear_modulus: 80e9,
        area: 0.006,
        second_moment: 2.7e-5,
        torsion_constant: 5.4e-5,
        in_plane_flexibility_factor: 1.0,
        out_of_plane_flexibility_factor: 1.0,
    };
    for (label, b) in [
        ("at_pi", bend([1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], 1.0)),
        ("beyond_pi", bend([1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], 0.75)),
    ] {
        let mut arith = WideArith::new(FORMATION_PRECISION).unwrap();
        assert!(
            matches!(curved_matrix(&mut arith, &b), Err(WideError::AngleDomain)),
            "{label}"
        );
        let mut k = vec![vec![0.0; 12]; 12];
        for (d, row) in k.iter_mut().enumerate() {
            row[d] = 1.0;
        }
        let force = [1.0; 12];
        let free: Vec<usize> = (0..12).collect();
        let source = FormationSource {
            node_count: 2,
            curved: vec![b],
            ..FormationSource::default()
        };
        let roundoff = vec![vec![0.0; 12]; 12];
        let counts = vec![vec![0usize; 12]; 12];
        let system = || StructuralSystem {
            stiffness: &k,
            force: &force,
            free_dofs: &free,
            prescribed: &[],
            contributions: None,
            contribution_groups: None,
            symmetry: Some(SymmetryEvidence {
                absolute_roundoff: &roundoff,
                operation_counts: &counts,
                basis: "test",
            }),
        };
        // Precondition: the ordinary solve publishes Passed.
        let plain = solve_structural_dense(&system()).unwrap();
        assert_eq!(plain.report.quality, SolveQuality::Passed, "{label}");
        let result = closed_test_solve(system(), &source);
        assert_eq!(result.report.quality, SolveQuality::Sensitive, "{label}");
        assert_eq!(result.displacements, plain.displacements, "{label}");
        match result.formation_check.unwrap().reason {
            FormationCheckReason::FormationCheckUnavailable { detail } => {
                assert!(
                    detail.starts_with("retained arithmetic"),
                    "{label}: {detail}"
                )
            }
            other => panic!("{label}: {other:?}"),
        }
    }
}

// ------------------------------------------------------------------ T4-U3 (S10)

/// T4-I12 round 02 U3-KD5-UTM-SKEW-OFFSET-COUPLED (case 22) at x offset X0:
/// binary64 offsets with tails below the UTM grid, the binary64 rounding of
/// a rational skew rotation, U3-GENERIC's coupled PD H at Ls = 1/4 m, and a
/// dyadic q_ref.
fn kd5_case22(x0: f64) -> ObjectiveConnector {
    use crate::connector::{ConnectorAttachment, ScaledWorkMatrix};
    let (third, two_thirds) = (1.0 / 3.0, 2.0 / 3.0);
    let q = [
        [third, two_thirds, -two_thirds],
        [two_thirds, third, two_thirds],
        [two_thirds, -two_thirds, -third],
    ];
    let h = [
        12500.0, 625.0, 0.0, 0.0, 500.0, 0.0, 9375.0, 312.5, 0.0, 0.0, -750.0, 7500.0, 250.0, 0.0,
        0.0, 800.0, 50.0, 0.0, 900.0, 100.0, 1200.0,
    ];
    ObjectiveConnector::new(
        FrameNode::new(0, [x0 + 0.5, -1.25, 2.0]).unwrap(),
        FrameNode::new(1, [x0 + 1.8125, 0.8125, 3.375]).unwrap(),
        ConnectorAttachment::global([0.2, 0.4, -0.2]),
        ConnectorAttachment::global([-0.1, 0.3625, 0.44999999999999996]),
        q,
        ScaledWorkMatrix {
            upper_triangle: h,
            translation_scale: 0.25,
        },
        [
            0.0009765625,
            -0.00048828125,
            0.000244140625,
            0.001953125,
            0.0,
            -0.0009765625,
        ],
    )
    .unwrap()
}

#[test]
fn kd5_connector_reformation_agrees_and_has_the_rigid_null_space() {
    for x0 in [0.0, 5.0e6, 7.3e6] {
        let c = kd5_case22(x0);
        let mut arith = WideArith::new(FORMATION_PRECISION).unwrap();
        let wide = connector_matrix(&mut arith, &c).unwrap();
        let binary64 = c.global_stiffness().unwrap();
        let scale = binary64
            .iter()
            .flatten()
            .fold(0.0_f64, |m, v| m.max(v.abs()));
        for r in 0..12 {
            for col in 0..12 {
                let w = round(&wide[r][col]);
                assert!(
                    (w - binary64[r][col]).abs() <= 1e-13 * scale,
                    "X0 {x0} [{r}][{col}] {w} {}",
                    binary64[r][col]
                );
            }
        }
        // A dyadic rigid motion about the origin (exact in binary64 at UTM).
        let omega = [0.25, -0.5, 0.125];
        let t = [1.0, -2.0, 0.5];
        let motion = |x: [f64; 3]| {
            [
                t[0] + (omega[1] * x[2] - omega[2] * x[1]),
                t[1] + (omega[2] * x[0] - omega[0] * x[2]),
                t[2] + (omega[0] * x[1] - omega[1] * x[0]),
                omega[0],
                omega[1],
                omega[2],
            ]
        };
        let mut u = [0.0; 12];
        u[..6].copy_from_slice(&motion(c.node_i().coordinates));
        u[6..].copy_from_slice(&motion(c.node_j().coordinates));
        let magnitude = u.iter().fold(0.0_f64, |m, v| m.max(v.abs()));
        for row in &wide {
            let mut acc = ExactAccumulator::new();
            for (kv, &x) in row.iter().zip(&u) {
                kv.add_product_to(&mut acc, x).unwrap();
            }
            assert!(
                acc.round().unwrap().abs() <= 1e-25 * scale * magnitude,
                "X0 {x0}"
            );
        }
    }
}

/// S-3 at the kernel level (dense): the connector case is not demoted at
/// X0 = 0, 5e6 and 7.3e6 m, and perturbing only the assembled matrix by
/// δ = 2^-20·max|Ke| at the largest diagonal (k = 3) demotes it.
#[test]
fn kd5_connector_case_is_not_demoted_and_its_perturbed_matrix_is() {
    for x0 in [0.0, 5.0e6, 7.3e6] {
        let c = kd5_case22(x0);
        let ke = c.global_stiffness().unwrap();
        let max = ke.iter().flatten().fold(0.0_f64, |m, v| m.max(v.abs()));
        assert_eq!(ke[3][3], max, "k = 3 is the largest diagonal");
        let source = FormationSource {
            node_count: 2,
            connectors: vec![c],
            ..FormationSource::default()
        };
        let force = [
            1000.0, -2000.0, 1500.0, 300.0, -200.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ];
        let free: Vec<usize> = (0..6).collect();
        let prescribed: Vec<(usize, f64)> = (6..12).map(|d| (d, 0.0)).collect();
        for perturbed in [false, true] {
            let mut k: Vec<Vec<f64>> = ke.iter().map(|row| row.to_vec()).collect();
            if perturbed {
                k[3][3] += max * 2f64.powi(-20);
            }
            let system = StructuralSystem {
                stiffness: &k,
                force: &force,
                free_dofs: &free,
                prescribed: &prescribed,
                contributions: None,
                contribution_groups: None,
                symmetry: None,
            };
            let plain = solve_structural_dense(&system).unwrap();
            assert_eq!(plain.report.quality, SolveQuality::Passed, "X0 {x0}");
            let result = closed_test_solve(system, &source);
            if perturbed {
                assert_eq!(result.report.quality, SolveQuality::Sensitive, "X0 {x0}");
                let check = result.formation_check.unwrap();
                assert_eq!(check.reason, FormationCheckReason::Estimate, "X0 {x0}");
            } else {
                assert_eq!(result.formation_check, None, "X0 {x0}");
                assert_eq!(plain.displacements, result.displacements);
                assert_eq!(plain.report.quality, result.report.quality);
                assert!(result.member_bound.is_some());
            }
        }
    }
}

#[test]
fn kd5_long_nearly_straight_bend_at_the_floor_does_not_demote() {
    // RV131 O4 / S-5 (T4-U1's stable form 1 − cos φ = 2s² at p = 128): a
    // rigidly rooted, axially loaded 30 m chord bent by φ ≈ 1e-9 and 2e-9
    // rad. With the cancelling 1 − cos φ the K-D5 re-formation's in-plane
    // coupling error falsely demoted these (trigger 47–198). The system
    // matrix is the intended element (T4-I6's frozen generator `curved_ref.py`
    // at 110 digits, each entry rounded once to binary64; generated by
    // `validation/references/t4_i6/t4_i14_l30_matrix.py`), independent of
    // the K-D5 code under test and of the product's element, so a correct
    // check must not demote and a reversion to the cancelling form does.
    // RV131 `rv_o4_axial`'s configuration: d = grid(30·(cos 30°, sin 30°, 0))
    // on the 2^-30 m grid, y = (0, 1, 0), R from the nominal angle, N0 fixed,
    // a unit force at N1 along the chord.
    let grid = |v: f64| (v * 2f64.powi(30)).round() / 2f64.powi(30);
    let pi = std::f64::consts::PI;
    let d = [
        grid(30.0 * (pi / 6.0).cos()),
        grid(30.0 * (pi / 6.0).sin()),
        0.0,
    ];
    let length = (d[0] * d[0] + d[1] * d[1]).sqrt();
    for (nominal, frozen) in [(1.0e-9_f64, &L30_PHI_1E_9), (2.0e-9, &L30_PHI_2E_9)] {
        let radius = length / (2.0 * (0.5 * nominal).sin());
        let bend = CurvedFormation {
            node_i: 0,
            node_j: 1,
            coordinates_i: [0.0, 0.0, 0.0],
            coordinates_j: d,
            radius,
            y_reference: [0.0, 1.0, 0.0],
            elastic_modulus: 200e9,
            shear_modulus: 80e9,
            area: 0.005969026041820614,
            second_moment: 2.700984283923829e-05,
            torsion_constant: 5.401968567847658e-05,
            in_plane_flexibility_factor: 1.0,
            out_of_plane_flexibility_factor: 1.0,
        };
        let k: Vec<Vec<f64>> = frozen.iter().map(|row| row.to_vec()).collect();
        let mut force = [0.0; 12];
        force[6] = d[0] / length;
        force[7] = d[1] / length;
        let free: Vec<usize> = (6..12).collect();
        let prescribed: Vec<(usize, f64)> = (0..6).map(|d| (d, 0.0)).collect();
        let source = FormationSource {
            node_count: 2,
            curved: vec![bend],
            ..FormationSource::default()
        };
        let system = || StructuralSystem {
            stiffness: &k,
            force: &force,
            free_dofs: &free,
            prescribed: &prescribed,
            contributions: None,
            contribution_groups: None,
            symmetry: None,
        };
        let plain = solve_structural_dense(&system()).unwrap();
        assert_eq!(plain.report.quality, SolveQuality::Passed, "phi {nominal}");
        let result = closed_test_solve(system(), &source);
        assert_eq!(result.formation_check, None, "phi {nominal}");
        assert_eq!(result.report.quality, SolveQuality::Passed, "phi {nominal}");
    }
}

#[test]
fn kd5_k2_formula_chord_system_demotes_against_the_actual_chord() {
    // K2 (T4-I6 round 01 item 3; RV131 S-2, S-3): the K1 cantilever bend
    // (φ = 1e-8 rad, in-plane and skew) whose system matrix is the exact B1
    // tip stiffness and axes with H built from the binary64 formula chord
    // c_f = (R(cos φ_b − 1), R sin φ_b, 0) instead of x_j − x_i, each entry
    // rounded once (frozen in `validation/references/t4_i6/round_01/
    // u1_reference_cases_r01.json`, `k2_kernel_kill`; independent of CB and
    // of K-D5). |c_f − A·d|/L = 5e-9, so the published u is wrong by about 6
    // criteria against the intended element. A correct check re-forms H from
    // the actual chord and demotes (trigger 12.74 / 12.45, emulated); a check
    // that builds H from the same binary64 formula chord (M31b), or takes the
    // curved K_int from the system's own element (M31a), does not.
    // Precondition (pinned): the kill needs the mutant's chord bit-equal to
    // the frozen c_f, so fl(cos φ_b) = 1.0 exactly and R·sin φ_b is the
    // frozen literal.
    let cases = [
        (
            "K2-IP",
            [0.25980762112885714, 0.15000000037252903, 0.0],
            30000000.018065747,
            [0.0, 1.0, 0.0],
            &K2_IP,
            0.3000000001806575,
            12.743305,
        ),
        (
            "K2-SK",
            [
                0.09999999962747097,
                0.20000000018626451,
                0.20000000018626451,
            ],
            30000000.012417633,
            [1.0, -1.0, 0.5],
            &K2_SK,
            0.3000000001241764,
            12.449873,
        ),
    ];
    for (label, xj, radius, y_reference, frozen, chord_y, expected) in cases {
        let [dx, dy, dz]: [f64; 3] = xj;
        let length = ((dx * dx + dy * dy) + dz * dz).sqrt();
        let phi_b = 2.0 * (length / (2.0 * radius)).asin();
        assert_eq!(phi_b.cos(), 1.0, "{label}: precondition fl(cos phi_b) = 1");
        assert_eq!(radius * (phi_b.cos() - 1.0), 0.0, "{label}");
        assert_eq!(radius * phi_b.sin(), chord_y, "{label}: frozen c_f,y");
        let bend = CurvedFormation {
            node_i: 0,
            node_j: 1,
            coordinates_i: [0.0, 0.0, 0.0],
            coordinates_j: xj,
            radius,
            y_reference,
            elastic_modulus: 200e9,
            shear_modulus: 80e9,
            area: 0.005969026041820614,
            second_moment: 2.700984283923829e-05,
            torsion_constant: 5.401968567847658e-05,
            in_plane_flexibility_factor: 1.0,
            out_of_plane_flexibility_factor: 1.0, // K2 element
        };
        let springs = vec![(3, 1e6), (4, 1e6), (5, 1e6)];
        let mut k: Vec<Vec<f64>> = frozen.iter().map(|row| row.to_vec()).collect();
        for &(dof, value) in &springs {
            k[dof][dof] += value;
        }
        let mut force = [0.0; 12];
        force[9] = 1.0;
        force[10] = 1.0;
        force[11] = 1.0;
        let free: Vec<usize> = (3..12).collect();
        let prescribed: Vec<(usize, f64)> = (0..3).map(|d| (d, 0.0)).collect();
        let source = FormationSource {
            node_count: 2,
            curved: vec![bend],
            springs,
            ..FormationSource::default()
        };
        let system = || StructuralSystem {
            stiffness: &k,
            force: &force,
            free_dofs: &free,
            prescribed: &prescribed,
            contributions: None,
            contribution_groups: None,
            symmetry: None,
        };
        let plain = solve_structural_dense(&system()).unwrap();
        assert_eq!(plain.report.quality, SolveQuality::Passed, "{label}");
        let result = closed_test_solve(system(), &source);
        assert_eq!(result.report.quality, SolveQuality::Sensitive, "{label}");
        assert_eq!(result.displacements, plain.displacements, "{label}");
        let record = result.formation_check.expect("demoted");
        assert_eq!(record.reason, FormationCheckReason::Estimate, "{label}");
        eprintln!(
            "kd5 {label} trigger={} dof={:?}",
            record.ratio, record.global_dof
        );
        assert!(
            (record.ratio / expected - 1.0).abs() < 0.01,
            "{label}: trigger {} expected {expected}",
            record.ratio
        );
    }
}

// Frozen K2 element matrices (`k2_kernel_kill[*].system_element_matrix_binary64`
// of `validation/references/t4_i6/round_01/u1_reference_cases_r01.json`).
#[rustfmt::skip]
const K2_IP: [[f64; 12]; 12] = [
    [3584731746.3047485, 683500062.1200134, 0.0, 0.0, 0.0, -180065625.08103397, -3584731746.3047485, -683500062.1200134, 0.0, 0.0, 0.0, -180065618.1885964],
    [683500062.1200134, 2795493860.55204, 0.0, 0.0, 0.0, 311882796.62693065, -683500062.1200134, -2795493860.55204, 0.0, 0.0, 0.0, 311882800.60628134],
    [0.0, 0.0, 2400874914.7060437, 180065622.7927447, -311882797.94807506, 0.0, 0.0, 0.0, -2400874914.7060437, 180065618.4263855, -311882800.4689937, 0.0],
    [0.0, 0.0, 180065622.7927447, 28810499.626762662, -24950624.394792657, 0.0, 0.0, 0.0, -180065622.7927447, -1800655.90685918, -21831796.575027242, 0.0],
    [0.0, 0.0, -311882797.94807506, -24950624.394792657, 57620997.40675156, 0.0, 0.0, 0.0, 311882797.94807506, -21831795.81875164, 23408530.165237732, 0.0],
    [-180065625.08103397, 311882796.62693065, 0.0, 0.0, 0.0, 72026247.52792853, 180065625.08103397, -311882796.62693065, 0.0, 0.0, 0.0, 36013123.76396425],
    [-3584731746.3047485, -683500062.1200134, 0.0, 0.0, 0.0, 180065625.08103397, 3584731746.3047485, 683500062.1200134, 0.0, 0.0, 0.0, 180065618.1885964],
    [-683500062.1200134, -2795493860.55204, 0.0, 0.0, 0.0, -311882796.62693065, 683500062.1200134, 2795493860.55204, 0.0, 0.0, 0.0, -311882800.60628134],
    [0.0, 0.0, -2400874914.7060437, -180065622.7927447, 311882797.94807506, 0.0, 0.0, 0.0, 2400874914.7060437, -180065618.4263855, 311882800.4689937, 0.0],
    [0.0, 0.0, 180065618.4263855, -1800655.90685918, -21831795.81875164, 0.0, 0.0, 0.0, -180065618.4263855, 28810498.971808773, -24950624.01665486, 0.0],
    [0.0, 0.0, -311882800.4689937, -21831796.575027242, 23408530.165237732, 0.0, 0.0, 0.0, 311882800.4689937, -24950624.01665486, 57620998.06170544, 0.0],
    [-180065618.1885964, 311882800.60628134, 0.0, 0.0, 0.0, 36013123.76396425, 180065618.1885964, -311882800.60628134, 0.0, 0.0, 0.0, 72026247.52792852],
];
#[rustfmt::skip]
const K2_SK: [[f64; 12]; 12] = [
    [2576261112.036592, 350772393.5824255, 350772393.5824255, -0.5871753013571793, 240087492.97674474, -240087488.36158225, -2576261112.036592, -350772393.5824255, -350772393.5824255, 0.06101671035671492, 240087491.62022826, -240087492.3709991],
    [350772393.5824255, 3102419706.493758, 701544790.4316734, -240087494.44468302, -0.5871753046960649, 120043749.41130698, -350772393.5824255, -3102419706.493758, -701544790.4316734, -240087491.46768647, 0.061016711042334984, 120043744.75369804],
    [350772393.5824255, 701544790.4316734, 3102419706.493758, 240087487.77440694, -120043748.82413168, 1.1743506060532443, -350772393.5824255, -701544790.4316734, -3102419706.493758, 240087492.4320158, -120043744.81471474, -0.12203342139904991],
    [-0.5871753013571793, -240087494.44468302, 240087487.77440694, 65623914.19158617, -12804666.327274458, -12804666.54335321, 0.5871753013571793, 240087494.44468302, -240087487.77440694, 30411082.221627656, -11204082.718249228, -11204082.934327967],
    [240087492.97674474, -0.5871753046960649, -120043748.82413168, -12804666.327274458, 46416915.63200225, -25609332.341644187, -240087492.97674474, 0.5871753046960649, 120043748.82413168, -11204083.22243296, 13604958.08584737, -22408166.117054585],
    [-240087488.36158225, 120043749.41130698, 1.1743506060532443, -12804666.54335321, -25609332.341644187, 46416914.76768728, 240087488.36158225, -120043749.41130698, -1.1743506060532443, -11204083.186419835, -22408165.612870853, 13604957.725716133],
    [-2576261112.036592, -350772393.5824255, -350772393.5824255, 0.5871753013571793, -240087492.97674474, 240087488.36158225, 2576261112.036592, 350772393.5824255, 350772393.5824255, -0.06101671035671492, -240087491.62022826, 240087492.3709991],
    [-350772393.5824255, -3102419706.493758, -701544790.4316734, 240087494.44468302, 0.5871753046960649, -120043749.41130698, 350772393.5824255, 3102419706.493758, 701544790.4316734, 240087491.46768647, -0.061016711042334984, -120043744.75369804],
    [-350772393.5824255, -701544790.4316734, -3102419706.493758, -240087487.77440694, 120043748.82413168, -1.1743506060532443, 350772393.5824255, 701544790.4316734, 3102419706.493758, -240087492.4320158, 120043744.81471474, 0.12203342139904991],
    [0.06101671035671492, -240087491.46768647, 240087492.4320158, 30411082.221627656, -11204083.22243296, -11204083.186419835, -0.06101671035671492, 240087491.46768647, -240087492.4320158, 65623914.52770863, -12804666.159213213, -12804666.123200089],
    [240087491.62022826, 0.061016711042334984, -120043744.81471474, -11204082.718249228, 13604958.08584737, -22408165.612870853, -240087491.62022826, -0.061016711042334984, 120043744.81471474, -12804666.159213213, 46416914.95975725, -25609332.50970542],
    [-240087492.3709991, 120043744.75369804, -0.12203342139904991, -11204082.934327967, -22408166.117054585, 13604957.725716133, 240087492.3709991, -120043744.75369804, 0.12203342139904991, -12804666.123200089, -25609332.50970542, 46416915.10380975],
];

// Frozen system matrices for `kd5_long_nearly_straight_bend_at_the_floor_does_not_demote`
// (global 12x12, node 0 then node 1; R = 30000000000.245842 and 15000000000.122921 m).
#[rustfmt::skip]
const L30_PHI_1E_9: [[f64; 12]; 12] = [
    [29845730.427751273, 17230054.350654174, 0.0, 0.0, 0.0, -18006.648047852646, -29845730.427751273, -17230054.350654174, 0.0, 0.0, 0.0, -18006.475736913046],
    [17230054.350654174, 9950177.39231236, 0.0, 0.0, 0.0, 31188.230325697776, -17230054.350654174, -9950177.39231236, 0.0, 0.0, 0.0, 31188.329809465136],
    [0.0, 0.0, 2400.8749189843793, 18006.5618986205, -31188.280063980143, 0.0, 0.0, 0.0, -2400.8749189843793, 18006.56188614519, -31188.28007118277, 0.0],
    [0.0, 0.0, 18006.5618986205, 288104.9903740513, -249506.24059467134, 0.0, 0.0, 0.0, -18006.5618986205, -18006.561894743812, -218317.96058110957, 0.0],
    [0.0, 0.0, -31188.280063980143, -249506.24059467134, 576209.980474491, 0.0, 0.0, 0.0, 31188.280063980143, -218317.96036503083, 234085.30460687942, 0.0],
    [-18006.648047852646, 31188.230325697776, 0.0, 0.0, 0.0, 720262.4757071189, 18006.648047852646, -31188.230325697776, 0.0, 0.0, 0.0, 360131.2378535591],
    [-29845730.427751273, -17230054.350654174, 0.0, 0.0, 0.0, 18006.648047852646, 29845730.427751273, 17230054.350654174, 0.0, 0.0, 0.0, 18006.475736913046],
    [-17230054.350654174, -9950177.39231236, 0.0, 0.0, 0.0, -31188.230325697776, 17230054.350654174, 9950177.39231236, 0.0, 0.0, 0.0, -31188.329809465136],
    [0.0, 0.0, -2400.8749189843793, -18006.5618986205, 31188.280063980143, 0.0, 0.0, 0.0, 2400.8749189843793, -18006.56188614519, 31188.28007118277, 0.0],
    [0.0, 0.0, 18006.56188614519, -18006.561894743812, -218317.96036503083, 0.0, 0.0, 0.0, -18006.56188614519, 288104.99018692167, -249506.24048663196, 0.0],
    [0.0, 0.0, -31188.28007118277, -218317.96058110957, 234085.30460687942, 0.0, 0.0, 0.0, 31188.28007118277, -249506.24048663196, 576209.9806616207, 0.0],
    [-18006.475736913046, 31188.329809465136, 0.0, 0.0, 0.0, 360131.2378535591, 18006.475736913046, -31188.329809465136, 0.0, 0.0, 0.0, 720262.4757071189],
];
#[rustfmt::skip]
const L30_PHI_2E_9: [[f64; 12]; 12] = [
    [29845730.42775125, 17230054.35065416, 0.0, 0.0, 0.0, -18006.73420332245, -29845730.42775125, -17230054.35065416, 0.0, 0.0, 0.0, -18006.389581443244],
    [17230054.35065416, 9950177.392312352, 0.0, 0.0, 0.0, 31188.180583814094, -17230054.35065416, -9950177.392312352, 0.0, 0.0, 0.0, 31188.37955134882],
    [0.0, 0.0, 2400.8749189843793, 18006.561904858158, -31188.28006037883, 0.0, 0.0, 0.0, -2400.8749189843793, 18006.561879907535, -31188.28007478408, 0.0],
    [0.0, 0.0, 18006.561904858158, 288104.9904676162, -249506.24064869102, 0.0, 0.0, 0.0, -18006.561904858158, -18006.561894743812, -218317.96068914892, 0.0],
    [0.0, 0.0, -31188.28006037883, -249506.24064869102, 576209.9803809263, 0.0, 0.0, 0.0, 31188.28006037883, -218317.96025699144, 234085.30460687942, 0.0],
    [-18006.73420332245, 31188.180583814094, 0.0, 0.0, 0.0, 720262.4757071197, 18006.73420332245, -31188.180583814094, 0.0, 0.0, 0.0, 360131.2378535583],
    [-29845730.42775125, -17230054.35065416, 0.0, 0.0, 0.0, 18006.73420332245, 29845730.42775125, 17230054.35065416, 0.0, 0.0, 0.0, 18006.389581443244],
    [-17230054.35065416, -9950177.392312352, 0.0, 0.0, 0.0, -31188.180583814094, 17230054.35065416, 9950177.392312352, 0.0, 0.0, 0.0, -31188.37955134882],
    [0.0, 0.0, -2400.8749189843793, -18006.561904858158, 31188.28006037883, 0.0, 0.0, 0.0, 2400.8749189843793, -18006.561879907535, 31188.28007478408, 0.0],
    [0.0, 0.0, 18006.561879907535, -18006.561894743812, -218317.96025699144, 0.0, 0.0, 0.0, -18006.561879907535, 288104.9900933568, -249506.24043261228, 0.0],
    [0.0, 0.0, -31188.28007478408, -218317.96068914892, 234085.30460687942, 0.0, 0.0, 0.0, 31188.28007478408, -249506.24043261228, 576209.9807551856, 0.0],
    [-18006.389581443244, 31188.37955134882, 0.0, 0.0, 0.0, 360131.2378535583, 18006.389581443244, -31188.37955134882, 0.0, 0.0, 0.0, 720262.4757071197],
];

fn axial_frame() -> FrameElement {
    FrameElement::new(
        FrameNode::new(0, [0.0; 3]).unwrap(),
        FrameNode::new(1, [4.0, 0.0, 0.0]).unwrap(),
        FrameSection::new(8.0, 4.0, 2.0, 1.0, 1.0, 2.0).unwrap(),
        [0.0, 1.0, 0.0],
    )
    .unwrap()
}
fn primitive(formula: LoadFormula, row: usize, ordinal: usize) -> LoadPrimitive {
    LoadPrimitive {
        term_index: 0,
        identity: LoadSummandIdentity {
            family: "family".into(),
            source: "member".into(),
            ordinal,
        },
        local_row: row,
        formula,
        force_scale_exponent: 0,
    }
}

/// L1a: compare the actual closure and formation-checked solve with the former
/// owned primitive copy. The derived source Debug must still print Vec's bytes.
#[test]
fn formation_source_borrow_is_byte_identical() {
    // L1a retains the priced vector header. This const assertion also runs
    // when the tests are compiled for wasm; no ABI equality is assumed.
    const { assert!(std::mem::size_of::<Cow<'_, [LoadPrimitive]>>() == std::mem::size_of::<Vec<LoadPrimitive>>()); }
    let mut ledger = crate::load_ledger::LoadLedger::new();
    for (index, (dof, magnitude, factor)) in [(0, 3.0, 2.0), (4, -4.0, 0.5)].into_iter().enumerate() {
        let mut p = primitive(LoadFormula::NodalProduct { global_dof: dof, magnitude, factor }, dof, index);
        p.term_index = index;
        ledger.push_formed("member", dof, magnitude * factor,
            crate::load_ledger::Formation::RoundedProduct { k: 1.0, a: magnitude, b: factor }, 0.0, false);
        assert!(ledger.set_closure_manifest(index, vec![p.closure_entry()]));
        ledger.attach_load_primitive(p);
    }
    let force = ledger.finish(6).unwrap();
    let source = FormationSource {
        node_count: 1,
        loads: force.load_primitives().into(),
        springs: (0..6).map(|dof| (dof, 3.0)).collect(),
        ..FormationSource::default()
    };
    assert!(matches!(source.loads, Cow::Borrowed(_)));
    assert!(std::ptr::eq(source.loads.as_ptr(), force.load_primitives().as_ptr()));
    let former_loads = force.load_primitives().to_vec();
    let former = FormationSource { loads: Cow::Owned(former_loads.clone()), ..source.clone() };
    let former_debug = format!(
        "FormationSource {{ node_count: {:?}, loads: {:?}, frames: {:?}, frame_members: {:?}, connectors: {:?}, curved: {:?}, springs: {:?}, unavailable: {:?} }}",
        source.node_count, former_loads, source.frames, source.frame_members, source.connectors, source.curved, source.springs, source.unavailable);
    assert_eq!(format!("{source:?}").as_bytes(), former_debug.as_bytes());
    assert_eq!(format!("{source:?}"), format!("{former:?}"));
    let closure: Vec<_> = source.loads.iter().map(LoadPrimitive::closure_entry).collect();
    // Pre-lever closure representation: formula tag, DOF, operand bits, local row, b.
    assert_eq!(closure, vec![
        LoadClosureEntry { identity: LoadSummandIdentity { family: "family".into(), source: "member".into(), ordinal: 0 },
            operand_bits: vec![7, 0, 4613937818241073152, 4611686018427387904, 0, 0] },
        LoadClosureEntry { identity: LoadSummandIdentity { family: "family".into(), source: "member".into(), ordinal: 1 },
            operand_bits: vec![7, 4, 13839561654909534208, 4602678819172646912, 4, 0] },
    ]);
    for s in [&source, &former] { load_closure(s, &force).unwrap(); }
    let k: Vec<Vec<f64>> = (0..6).map(|r| (0..6).map(|c| if r == c { 3.0 } else { 0.0 }).collect()).collect();
    let free: Vec<_> = (0..6).collect();
    let contributions: Vec<_> = source.springs.iter().map(|&(dof, value)| super::super::StiffnessContribution { row: dof, col: dof, value }).collect();
    let solve = |s| {
        let system = StructuralSystem::assembled(&k, &force, &free, &[], Some(&contributions), None).with_formation_source(s);
        super::super::solve_formation_checked_structural_dense(&system).unwrap()
    };
    let borrowed = solve(&source);
    let owned = solve(&former);
    assert_eq!(borrowed.report.quality, SolveQuality::Passed);
    assert!(borrowed.member_bound.is_some());
    assert_eq!(format!("{borrowed:?}").as_bytes(), format!("{owned:?}").as_bytes());
}

#[test]
fn kd5_closed_manifest_kills_missing_duplicate_extra_wrong_binding_and_input_on_ordinary_and_w2() {
    let recipe = LoadFormula::UniformStraight {
        frame: axial_frame(),
        intensity: IntensityPrimitive::Authored([1.0, 0.0, 0.0]),
        start_fraction: 0.0,
        end_fraction: 1.0,
    };
    // Expected identities come from complete source enumeration, before supplied entries.
    let expected: Vec<_> = (0..2)
        .map(|ordinal| primitive(recipe.clone(), 0, ordinal).closure_entry())
        .collect();
    let mut ledger = crate::load_ledger::LoadLedger::new();
    ledger.push_formed(
        "group",
        0,
        4.0,
        crate::load_ledger::Formation::CannotBound,
        0.0,
        false,
    );
    assert!(ledger.set_closure_manifest(0, expected));
    for ordinal in 0..2 {
        ledger.attach_load_primitive(primitive(recipe.clone(), 0, ordinal));
    }
    let force = ledger.finish(12).unwrap();
    for exponent in [-3, 0, 3] {
        let scale = crate::ForceScale { exponent };
        let scaled = force.force_scaled(scale).unwrap();
        assert_eq!(scaled.closure_records()[0].formed, true);
        assert!(!scaled.has_formation_records() || exponent == 0); // numeric records are deliberately omitted.
        let source = FormationSource {
            node_count: 2,
            loads: scaled.load_primitives().into(),
            ..FormationSource::default()
        };
        load_closure(&source, &scaled).unwrap();
        let mut mutants = Vec::new();
        let mut p = source.clone();
        p.loads.to_mut().pop();
        mutants.push(p);
        let mut p = source.clone();
        p.loads.to_mut()[1] = p.loads[0].clone();
        mutants.push(p);
        let mut p = source.clone();
        let extra = p.loads[0].clone();
        p.loads.to_mut().push(extra);
        mutants.push(p);
        let mut p = source.clone();
        p.loads.to_mut()[0].local_row = 1;
        mutants.push(p);
        let mut p = source.clone();
        p.loads.to_mut()[0].term_index = 1;
        mutants.push(p);
        let mut p = source.clone();
        p.loads.to_mut()[0].force_scale_exponent += 1;
        mutants.push(p);
        let mut p = source.clone();
        if let LoadFormula::UniformStraight { intensity, .. } = &mut p.loads.to_mut()[0].formula {
            *intensity = IntensityPrimitive::Authored([2.0, 0.0, 0.0]);
        }
        mutants.push(p);
        for mutant in mutants {
            assert!(matches!(
                load_closure(&mutant, &scaled),
                Err(Failure::Closure(_))
            ));
        }
    }
    let input = input_ledger(&[0.0; 12]);
    let source = FormationSource {
        node_count: 2,
        loads: force.load_primitives().into(),
        ..FormationSource::default()
    };
    assert!(matches!(
        load_closure(&source, &input),
        Err(Failure::Closure(_))
    ));
    let mut absent = crate::load_ledger::LoadLedger::new();
    absent.push_formed(
        "no manifest",
        0,
        1.0,
        crate::load_ledger::Formation::CannotBound,
        0.0,
        false,
    );
    assert!(matches!(
        load_closure(&FormationSource::default(), &absent.finish(12).unwrap()),
        Err(Failure::Closure(_))
    ));
}
#[test]
fn kd5_load_families_reform_source_values_and_scaling_without_forming_on_the_heap() {
    let f = axial_frame();
    let thermal = EigenstrainPrimitive {
        thermal: ThermalPrimitive::ConstantAlphaInterval {
            coefficient_per_kelvin: 0.125,
            temperature_change_kelvin: 2.0,
        },
        fit: FitPrimitive::EngineeringStrain { strain: 0.5 },
    };
    // ε_total=.5+.25+.5*.25=.875, EA=16 => thrust14, all exact dyadics.
    let cases = [
        (
            LoadFormula::UniformStraight {
                frame: f,
                intensity: IntensityPrimitive::Authored([1.0, 0.0, 0.0]),
                start_fraction: 0.0,
                end_fraction: 1.0,
            },
            0,
            2.0,
        ),
        (
            LoadFormula::UniformStraight {
                frame: f,
                intensity: IntensityPrimitive::Product {
                    factors: vec![2.0, 0.25],
                    direction: [1.0, 0.0, 0.0],
                },
                start_fraction: 0.0,
                end_fraction: 0.5,
            },
            0,
            0.75,
        ),
        (
            LoadFormula::UniformStraight {
                frame: f,
                intensity: IntensityPrimitive::Wind {
                    pressure: 2.0,
                    shape_factor: 0.5,
                    outer_diameter: 2.0,
                    insulation_thickness: 0.5,
                    direction: [1.0, 0.0, 0.0],
                },
                start_fraction: 0.0,
                end_fraction: 1.0,
            },
            0,
            6.0,
        ),
        (
            LoadFormula::FreeStrainStraight {
                frame: f,
                strain: thermal,
            },
            6,
            14.0,
        ),
    ];
    for (formula, row, exact) in cases {
        let p = primitive(formula, row, 0);
        let (values, radius) = load_reformation(&p).unwrap();
        assert_eq!(round(&values[row]), exact);
        assert!(radius[row].is_finite() && radius[row] >= 0.0);
        for exponent in [-3, 3] {
            let scaled = p.force_scaled(exponent).unwrap();
            let (v, r) = load_reformation(&scaled).unwrap();
            assert_eq!(round(&v[row]), exact * 2.0f64.powi(exponent));
            assert!(r[row] >= radius[row] * 2.0f64.powi(exponent));
        }
    }
    // The library's Wide arithmetic and all element/load vectors are stack only.
    let source = include_str!("formation_check.rs");
    let reform = &source[source.find("fn intended_stiffness_pass").unwrap()
        ..source.find("fn subtract_action").unwrap()];
    assert!(!reform.contains("Vec<Element>"));
    assert!(!reform.contains("collect"));
    let reform = &source
        [source.find("fn load_reformation").unwrap()..source.find("fn pressure_force").unwrap()];
    assert!(!reform.contains("Vec"));
    assert!(!reform.contains("collect"));
}
#[test]
fn kd5_second_solve_contraction_neumann_zero_over_zero_and_no_ledger_fail_closed() {
    let force = input_ledger(&[2.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    let (k, source) = spring_system(force.values());
    let contributions = [super::super::StiffnessContribution {
        row: 0,
        col: 0,
        value: 2.0,
    }];
    let prescribed = [(1, 0.0), (2, 0.0), (3, 0.0), (4, 0.0), (5, 0.0)];
    let system = StructuralSystem {
        stiffness: &k,
        force: force.values(),
        free_dofs: &[0],
        prescribed: &prescribed,
        contributions: Some(&contributions),
        contribution_groups: None,
        symmetry: None,
    };
    let u = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let solve = |rhs: &[f64]| Ok(vec![rhs[0] / 2.0]);
    let (finding, bound) = check(
        &system,
        &source,
        Some(&force),
        &[0],
        &u,
        (1.0, 0.5, 2.0),
        &|i, j| k[i][j],
        &|_, _| 2.0,
        &solve,
    );
    assert!(finding.is_none());
    let bound = bound.unwrap();
    assert_eq!(bound.w, vec![0.0; 6]);
    assert_eq!(bound.w2, vec![0.0; 6]);
    assert_eq!(bound.lambda_sigma, 0.0);
    assert_eq!(bound.exponents[1], i32::MIN);
    let (finding, _) = check(
        &system,
        &source,
        None,
        &[0],
        &u,
        (1.0, 0.5, 2.0),
        &|i, j| k[i][j],
        &|_, _| 2.0,
        &solve,
    );
    assert!(
        matches!(finding.unwrap().reason,FormationCheckReason::FormationCheckUnavailable{detail}if detail=="load closure: no ledger")
    );
    let mut changed = crate::load_ledger::LoadLedger::new();
    changed.push("held", 0, 2.0);
    changed.push("small", 0, 2.0f64.powi(-40));
    let changed = changed.finish(6).unwrap();
    for contraction in [false, true] {
        let count = std::cell::Cell::new(0);
        let faulty = |rhs: &[f64]| {
            let call = count.get();
            count.set(call + 1);
            if call == 0 {
                Ok(vec![rhs[0] / 2.0 + 2.0f64.powi(-40)])
            } else if contraction {
                Ok(vec![rhs[0] * 2.0f64.powi(30)])
            } else {
                Err(StructuralError::InvalidInput("seeded second solve"))
            }
        };
        let (finding, bound) = check(
            &system,
            &source,
            Some(&changed),
            &[0],
            &u,
            (1.0, 0.5, 2.0),
            &|i, j| k[i][j],
            &|_, _| 2.0,
            &faulty,
        );
        assert!(bound.is_none());
        let FormationCheckReason::FormationCheckUnavailable { detail } = finding.unwrap().reason
        else {
            panic!("unavailable expected")
        };
        assert!(
            detail.contains(if contraction {
                "contraction"
            } else {
                "second solve"
            }),
            "{detail}"
        );
        assert_eq!(count.get(), 2);
    }
    let (finding, bound) = check(
        &system,
        &source,
        Some(&force),
        &[0],
        &u,
        (1.0, 1.0, 3.0),
        &|i, j| if i == j && i == 0 { 3.0 } else { 0.0 },
        &|_, _| 3.0,
        &solve,
    );
    assert!(bound.is_none());
    assert!(
        matches!(finding.unwrap().reason,FormationCheckReason::FormationCheckUnavailable{detail}if detail=="Neumann")
    );
}
#[test]
fn kd5_epsilon_triangle_includes_formation_assembly_projection_and_both_norms() {
    let f = axial_frame();
    let global = f.global_stiffness().unwrap();
    let map = element_dof_map(0, 1);
    let mut contributions = Vec::new();
    for r in 0..12 {
        for c in 0..12 {
            contributions.push(super::super::StiffnessContribution {
                row: map[r],
                col: map[c],
                value: global[r][c],
            });
        }
    }
    let source = FormationSource {
        node_count: 2,
        frames: vec![f],
        ..FormationSource::default()
    };
    let free = [0, 6, 1];
    let mut index = vec![None; 12];
    for (r, &i) in free.iter().enumerate() {
        index[i] = Some(r);
    }
    let system = StructuralSystem {
        stiffness: &[],
        force: &[0.0; 12],
        free_dofs: &free,
        prescribed: &[],
        contributions: Some(&contributions),
        contribution_groups: None,
        symmetry: None,
    };
    // Added stored assembly shift2 and nonsymmetric projection3 at (0,6) give
    // triangle5, though their signed net is1. Other row entries make the row
    // norm differ from the column norm. Test both independently and the triangle.
    let assembled = |i: usize, j: usize| global[i][j] + if i == 0 && j == 6 { 2.0 } else { 0.0 };
    let projected = |r: usize, c: usize| {
        assembled(free[r], free[c])
            + if r == 0 && c == 1 {
                3.0
            } else if r == 0 && c == 2 {
                4.0
            } else {
                0.0
            }
    };
    let eps = epsilon_e(&system, &source, &index, &[0, 0, 0], &assembled, &projected).unwrap();
    assert!(eps >= 9.0 && eps < 9.0 + 1e-12, "{eps}");
}

#[test]
fn kd5_pi_families_enclose_independent_256_bit_reference() {
    use super::super::retained::wide::multi::WideContext;
    use super::super::retained::wide::Wide;
    // Independently computed by Decimal Machin's identity 16atan(1/5)-4atan(1/239)
    // at 100 digits, rounded to the p256 grid. Its error is below 2^-254.
    let pi = Wide::<4>::from_parts(
        false,
        1,
        [
            0x020bbea63b139b22,
            0x29024e088a67cc74,
            0xc4c6628b80dc1cd1,
            0xc90fdaa22168c234,
        ],
    )
    .unwrap();
    let mut a = WideContext::<4>::new(256).unwrap();
    let m = MassPrimitive {
        outer_diameter: 2.0,
        wall_thickness: 0.5,
        mill_tolerance: 0.0,
        pipe_density: 2.0,
        contents_density: 1.0,
        insulation_thickness: 0.25,
        insulation_density: 0.5,
        corrosion_allowance: None,
    };
    let f = axial_frame();
    let cases = [
        (
            LoadFormula::UniformStraight {
                frame: f,
                intensity: IntensityPrimitive::SelfWeight {
                    mass: m,
                    gravity: [2.0, 0.0, 0.0],
                },
                start_fraction: 0.0,
                end_fraction: 1.0,
            },
            0,
            8.125,
        ),
        (
            LoadFormula::UniformStraight {
                frame: f,
                intensity: IntensityPrimitive::Seismic {
                    mass: m,
                    coefficient: 0.5,
                    gravity: 2.0,
                    direction: [1.0, 0.0, 0.0],
                },
                start_fraction: 0.0,
                end_fraction: 1.0,
            },
            0,
            4.0625,
        ),
        // T4-U6 (D-5 mass): metal on the nominal wall 0.5 (0.75 pi x 2), the
        // contents on the pressure bore 1 - ((0.5 - 0.125) - 0.125) = 0.75
        // (0.5625 pi x 1), insulation as above (0.28125 pi): 2.34375 pi.
        (
            LoadFormula::UniformStraight {
                frame: f,
                intensity: IntensityPrimitive::SelfWeight {
                    mass: MassPrimitive {
                        mill_tolerance: 0.125,
                        corrosion_allowance: Some(0.125),
                        ..m
                    },
                    gravity: [2.0, 0.0, 0.0],
                },
                start_fraction: 0.0,
                end_fraction: 1.0,
            },
            0,
            9.375,
        ),
        (
            LoadFormula::PressureOperand {
                geometry: PressureGeometry::Straight { frame: f },
                pressure: 2.0,
                outer_diameter: 2.0,
                wall_thickness: 0.5,
                coefficient: PressureCoefficient::Poisson {
                    poisson_ratio: 0.125,
                    sign: -2.0,
                },
                direction: PressureDirection::Chord { axis: 0 },
            },
            0,
            -0.125,
        ),
        (
            LoadFormula::PressureOperand {
                geometry: PressureGeometry::Straight { frame: f },
                pressure: 2.0,
                outer_diameter: 2.0,
                wall_thickness: 0.5,
                coefficient: PressureCoefficient::Constant(1.0),
                direction: PressureDirection::Chord { axis: 0 },
            },
            0,
            0.5,
        ),
    ];
    for (formula, row, factor) in cases {
        let (value, radius) = load_reformation(&primitive(formula, row, 0)).unwrap();
        let reference = a.mul(&pi, &Wide::<4>::from_f64(factor).unwrap()).unwrap();
        let error = a.sub(&value[row].widen::<4>(), &reference).unwrap().abs();
        let bound = Wide::<4>::from_f64(radius[row]).unwrap();
        assert!(
            error.cmp_value(&bound).is_le(),
            "factor {factor}: {error:?} > {bound:?}"
        );
        assert!(radius[row] < 1e-30);
    }
}
/// T4-U6 (Design §1, D-5 mass): the allowance is a binding leaf. A preview
/// mass binds as before D-5 (no added word); an exact mass, c_a = 0 included,
/// binds its basis and its allowance, so neither stands in for the other.
#[test]
fn d5_mass_allowance_is_a_closure_binding_leaf() {
    let m = MassPrimitive {
        outer_diameter: 2.0,
        wall_thickness: 0.5,
        mill_tolerance: 0.125,
        pipe_density: 2.0,
        contents_density: 1.0,
        insulation_thickness: 0.0,
        insulation_density: 0.0,
        corrosion_allowance: None,
    };
    let bits = |mass: MassPrimitive| {
        primitive(
            LoadFormula::UniformStraight {
                frame: axial_frame(),
                intensity: IntensityPrimitive::SelfWeight { mass, gravity: [2.0, 0.0, 0.0] },
                start_fraction: 0.0,
                end_fraction: 1.0,
            },
            0,
            0,
        )
        .closure_entry()
        .operand_bits
    };
    let preview = bits(m);
    let zero = bits(MassPrimitive { corrosion_allowance: Some(0.0), ..m });
    let exact = bits(MassPrimitive { corrosion_allowance: Some(0.125), ..m });
    // The mass binding is followed by gravity (3 words), the two fractions,
    // the local row and the exponent: the allowance leaf sits between.
    let leaf = preview.len() - 7;
    assert_eq!(zero.len(), preview.len() + 2);
    assert_eq!(zero[..leaf], preview[..leaf]);
    assert_eq!(zero[leaf + 2..], preview[leaf..]);
    assert_eq!(zero[leaf..leaf + 2], [u64::MAX - 1, 0.0f64.to_bits()]);
    assert_eq!(exact[leaf..leaf + 2], [u64::MAX - 1, 0.125f64.to_bits()]);
    // The two bases re-form different intensities whenever c_m > 0.
    let reform = |mass| {
        load_reformation(&primitive(
            LoadFormula::UniformStraight {
                frame: axial_frame(),
                intensity: IntensityPrimitive::SelfWeight { mass, gravity: [2.0, 0.0, 0.0] },
                start_fraction: 0.0,
                end_fraction: 1.0,
            },
            0,
            0,
        ))
        .unwrap()
        .0[0]
    };
    let zero_value = reform(MassPrimitive { corrosion_allowance: Some(0.0), ..m });
    assert!(reform(m).cmp_value(&zero_value).is_ne());
}
fn quarter_arc() -> CurvedFormation {
    CurvedFormation {
        node_i: 0,
        node_j: 1,
        coordinates_i: [1.0, 0.0, 0.0],
        coordinates_j: [0.0, 1.0, 0.0],
        radius: 1.0,
        y_reference: [1.0, 1.0, 0.0],
        elastic_modulus: 8.0,
        shear_modulus: 4.0,
        area: 2.0,
        second_moment: 1.0,
        torsion_constant: 2.0,
        in_plane_flexibility_factor: 1.0,
        out_of_plane_flexibility_factor: 1.0,
    }
}
#[test]
fn kd5_curved_load_family_encloses_independent_resultant_and_free_strain_uses_same_matrix() {
    use super::super::retained::wide::multi::WideContext;
    use super::super::retained::wide::Wide;
    let arc = quarter_arc();
    let mut a = WideContext::<4>::new(256).unwrap();
    let pi = Wide::<4>::from_parts(
        false,
        1,
        [
            0x020bbea63b139b22,
            0x29024e088a67cc74,
            0xc4c6628b80dc1cd1,
            0xc90fdaa22168c234,
        ],
    )
    .unwrap();
    let (value, radius) = load_reformation(&primitive(
        LoadFormula::UniformCurved {
            arc,
            intensity: IntensityPrimitive::Authored([0.0, -1.0, 0.0]),
        },
        0,
        0,
    ))
    .unwrap();
    let total = a
        .add(&value[1].widen::<4>(), &value[7].widen::<4>())
        .unwrap();
    let target = pi.mul_pow2(-1).unwrap().neg();
    let error = a.sub(&total, &target).unwrap().abs();
    let bound = Wide::<4>::from_f64(add_up(radius[1], radius[7])).unwrap();
    assert!(error.cmp_value(&bound).is_le());
    let moment0 = a
        .add(&value[5].widen::<4>(), &value[11].widen::<4>())
        .unwrap();
    let moment1 = a.add(&moment0, &value[1].widen::<4>()).unwrap();
    let moment = a.sub(&moment1, &value[6].widen::<4>()).unwrap();
    let target = Wide::<4>::from_f64(-1.0).unwrap();
    let error = a.sub(&moment, &target).unwrap().abs();
    let bound = Wide::<4>::from_f64(add_up(
        add_up(radius[5], radius[11]),
        add_up(radius[1], radius[6]),
    ))
    .unwrap();
    assert!(error.cmp_value(&bound).is_le());
    let strain = EigenstrainPrimitive {
        thermal: ThermalPrimitive::ExplicitIntervalStrain { strain: 0.25 },
        fit: FitPrimitive::None,
    };
    let (value, radius) = load_reformation(&primitive(
        LoadFormula::FreeStrainCurved {
            arc,
            strain,
            column: None,
        },
        0,
        0,
    ))
    .unwrap();
    let mut arith = WideArith::new(128).unwrap();
    let k = curved_matrix(&mut arith, &arc).unwrap();
    for r in 0..12 {
        let col6 = a
            .mul(&k[r][6].widen::<4>(), &Wide::<4>::from_f64(-0.25).unwrap())
            .unwrap();
        let col7 = a
            .mul(&k[r][7].widen::<4>(), &Wide::<4>::from_f64(0.25).unwrap())
            .unwrap();
        let reference = a.add(&col6, &col7).unwrap();
        let error = a.sub(&value[r].widen::<4>(), &reference).unwrap().abs();
        assert!(error
            .cmp_value(&Wide::<4>::from_f64(radius[r]).unwrap())
            .is_le());
    }
}
#[test]
fn kd5_connector_reference_and_thermal_laws_have_exact_family_controls() {
    use crate::connector::{ConnectorAttachment, ScaledWorkMatrix};
    let f = axial_frame();
    let mut upper = [0.0; 21];
    let mut at = 0;
    for r in 0..6 {
        for c in r..6 {
            upper[at] = if r == c { 8.0 } else { 0.0 };
            at += 1;
        }
    }
    let c = ObjectiveConnector::new(
        f.node_i,
        f.node_j,
        ConnectorAttachment::global([0.0; 3]),
        ConnectorAttachment::global([0.0; 3]),
        [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        ScaledWorkMatrix {
            upper_triangle: upper,
            translation_scale: 1.0,
        },
        [0.25, 0.0, 0.0, 0.0, 0.0, 0.0],
    )
    .unwrap();
    let (v, r) = load_reformation(&primitive(
        LoadFormula::ConnectorReference { connector: c },
        0,
        0,
    ))
    .unwrap();
    assert_eq!(round(&v[0]), -2.0);
    assert_eq!(round(&v[6]), 2.0);
    assert!(r.iter().all(|v| v.is_finite()));
    let points = vec![
        ThermalPointPrimitive {
            temperature_kelvin: 0.0,
            value: 0.25,
        },
        ThermalPointPrimitive {
            temperature_kelvin: 2.0,
            value: 0.25,
        },
    ];
    for (law, exact) in [
        (
            ExpansionLawPrimitive::EngineeringSecant {
                datum_kelvin: 0.0,
                data: CoefficientPrimitive::Constant(0.25),
            },
            0.5,
        ),
        (
            ExpansionLawPrimitive::EngineeringSecant {
                datum_kelvin: 0.0,
                data: CoefficientPrimitive::Table(points.clone()),
            },
            0.5,
        ),
        (
            ExpansionLawPrimitive::DifferentialPerDatumLength {
                datum_kelvin: 0.0,
                points: points.clone(),
            },
            0.5,
        ),
        (
            ExpansionLawPrimitive::EngineeringDilation {
                datum_kelvin: 0.0,
                points: vec![
                    ThermalPointPrimitive {
                        temperature_kelvin: 0.0,
                        value: 0.0,
                    },
                    ThermalPointPrimitive {
                        temperature_kelvin: 2.0,
                        value: 0.5,
                    },
                ],
            },
            0.5,
        ),
    ] {
        let strain = EigenstrainPrimitive {
            thermal: ThermalPrimitive::FreeLengthState {
                installation_kelvin: 0.0,
                operating_kelvin: 2.0,
                law,
            },
            fit: FitPrimitive::None,
        };
        let (v, _) = load_reformation(&primitive(
            LoadFormula::FreeStrainStraight { frame: f, strain },
            6,
            0,
        ))
        .unwrap();
        assert_eq!(round(&v[6]), 16.0 * exact);
    }
    let strain = EigenstrainPrimitive {
        thermal: ThermalPrimitive::UnchangedReference,
        fit: FitPrimitive::NaturalLengthChange {
            change_m: 1.0,
            reference_length_m: 4.0,
        },
    };
    let (v, _) = load_reformation(&primitive(
        LoadFormula::FreeStrainStraight { frame: f, strain },
        6,
        0,
    ))
    .unwrap();
    assert_eq!(round(&v[6]), 4.0);
}

/// U5a's p-formation against an independently expanded exact dyadic triple
/// product. The expansion has more than 128 significant bits, so a binary64
/// midpoint with a narrow radius cannot pass even if its display rounds right.
#[test]
fn u5a_joint_effective_area_reformation_encloses_exact_three_factor_source_at_p() {
    use crate::connector::{ConnectorAttachment, ScaledWorkMatrix};
    use crate::exact_sum::ExactAccumulator;
    let mut upper=[0.0;21];let mut at=0;
    for r in 0..6 {for c in r..6 {upper[at]=if r==c {8.0}else{0.0};at+=1;}}
    let connector=ObjectiveConnector::new(
        FrameNode::new(0,[0.0;3]).unwrap(),FrameNode::new(1,[0.6,0.8,0.0]).unwrap(),
        ConnectorAttachment::global([0.0;3]),ConnectorAttachment::global([0.0;3]),
        [[0.6,-0.8,0.0],[0.8,0.6,0.0],[0.0,0.0,1.0]],
        ScaledWorkMatrix{upper_triangle:upper,translation_scale:1.0},[0.0;6],
    ).unwrap();
    let dp=2.0_f64.powi(-51);let da=2.0_f64.powi(-50);
    let pressure=1.0+dp;let area=1.0+da;
    let mut nonzero_errors=0;
    for end in 0..2 {for axis in 0..3 {for exponent in [-20,0,30] {
        let sign=if end==0 {-1.0}else{1.0};
        let mut operand=primitive(LoadFormula::JointEffectiveArea{connector:connector.clone(),end,pressure,effective_area:area,axis,sign},axis,0);
        operand.force_scale_exponent=exponent;
        let (midpoint,radius)=load_reformation(&operand).unwrap();
        let d=sign*connector.axes()[axis][0]*2.0_f64.powi(exponent);
        let split=midpoint[axis].split_binary64().unwrap();
        // Exact identity: (1+dp)(1+da)d = d + dp*d + da*d + dp*da*d.
        // ExactAccumulator's integer products/sums share no Balls/Wide law.
        for side in [-1.0,1.0] {
            let mut difference=ExactAccumulator::new();
            for factor in [1.0,dp,da,dp*da] {difference.add_product(d,factor).unwrap();}
            for term in split.terms() {difference.add(-term).unwrap();}
            if !difference.is_zero() {nonzero_errors+=1;}
            difference.add(side*radius[axis]).unwrap();
            assert!(if side<0.0 {difference.signum()<=0}else{difference.signum()>=0},"end={end} axis={axis} W2=2^{exponent}: midpoint is outside the source enclosure");
        }
        let reference_scale=(d*pressure*area).abs();
        assert!(radius[axis]<=2.0_f64.powi(-120)*reference_scale,"2^-120 source level: radius={:e} scale={reference_scale:e}",radius[axis]);
    }}}
    assert!(nonzero_errors>0,"the exact source extends below the midpoint precision");
}
#[test]
fn kd5_load_radius_is_not_a_shift_and_all_prescribed_rows_still_reform() {
    let formula = LoadFormula::UniformStraight {
        frame: axial_frame(),
        intensity: IntensityPrimitive::Authored([1.0, 0.0, 0.0]),
        start_fraction: 0.0,
        end_fraction: 1.0,
    };
    let p = primitive(formula, 0, 0);
    let mut ledger = crate::load_ledger::LoadLedger::new();
    ledger.push_formed(
        "uniform",
        0,
        2.0,
        crate::load_ledger::Formation::CannotBound,
        0.0,
        false,
    );
    ledger.set_closure_manifest(0, vec![p.closure_entry()]);
    ledger.attach_load_primitive(p);
    let force = ledger.finish(12).unwrap();
    let source = FormationSource {
        node_count: 2,
        loads: force.load_primitives().into(),
        springs: vec![(0, 2.0)],
        ..FormationSource::default()
    };
    let mut k = vec![vec![0.0; 12]; 12];
    k[0][0] = 2.0;
    let contributions = [super::super::StiffnessContribution {
        row: 0,
        col: 0,
        value: 2.0,
    }];
    let prescribed: Vec<_> = (1..12).map(|i| (i, 0.0)).collect();
    let mut u = [0.0; 12];
    u[0] = 1.0;
    let system = StructuralSystem {
        stiffness: &k,
        force: force.values(),
        free_dofs: &[0],
        prescribed: &prescribed,
        contributions: Some(&contributions),
        contribution_groups: None,
        symmetry: None,
    };
    let (finding, bound) = check(
        &system,
        &source,
        Some(&force),
        &[0],
        &u,
        (1.0, 0.5, 2.0),
        &|i, j| k[i][j],
        &|_, _| 2.0,
        &|rhs| Ok(vec![rhs[0] / 2.0]),
    );
    assert!(finding.is_none());
    let bound = bound.unwrap();
    assert!(bound.w.iter().chain(&bound.w2).all(|v| *v == 0.0));
    assert!(bound.lambda_sigma > 0.0);
    let f = axial_frame();
    let k: Vec<_> = f
        .global_stiffness()
        .unwrap()
        .into_iter()
        .map(|r| r.to_vec())
        .collect();
    let mut prescribed: Vec<_> = (0..12).map(|i| (i, 0.0)).collect();
    prescribed[11].1 = 0.125;
    let source = FormationSource {
        node_count: 2,
        frames: vec![f],
        ..FormationSource::default()
    };
    let system = StructuralSystem {
        stiffness: &k,
        force: &[0.0; 12],
        free_dofs: &[],
        prescribed: &prescribed,
        contributions: None,
        contribution_groups: None,
        symmetry: None,
    };
    let solution = closed_test_solve(system, &source);
    assert_eq!(solution.report.quality, SolveQuality::Passed);
    let bound = solution.member_bound.unwrap();
    assert_eq!(bound.exponents, vec![i32::MIN; 12]);
    assert_eq!(bound.lambda_sigma, 0.0);
    assert!(bound.w.iter().chain(&bound.w2).all(|v| *v == 0.0));
}

#[test]
fn kd5_full_attempt_bound_keeps_w2_physical_remainder_invariant() {
    let recipe = LoadFormula::UniformStraight {
        frame: axial_frame(),
        intensity: IntensityPrimitive::Authored([1.0, 0.0, 0.0]),
        start_fraction: 0.0,
        end_fraction: 1.0,
    };
    let p = primitive(recipe, 0, 0);
    let mut ledger = crate::load_ledger::LoadLedger::new();
    ledger.push_formed(
        "uniform",
        0,
        2.0,
        crate::load_ledger::Formation::CannotBound,
        0.0,
        false,
    );
    ledger.set_closure_manifest(0, vec![p.closure_entry()]);
    ledger.attach_load_primitive(p);
    let force = ledger.finish(12).unwrap();
    let prescribed: Vec<_> = (1..12).map(|i| (i, 0.0)).collect();
    let mut baseline: Option<f64> = None;
    // Public W2 admits only even exponents. Odd controls below exercise the
    // lower-level algebra with an internal scale, without widening admission.
    for exponent in [0, -6, 6, -3, 3] {
        let scale = crate::ForceScale { exponent };
        let force = force.force_scaled(scale).unwrap();
        let stiffness = 2.0 * 2.0f64.powi(exponent);
        let mut k = vec![vec![0.0; 12]; 12];
        k[0][0] = stiffness;
        let contributions = [super::super::StiffnessContribution {
            row: 0,
            col: 0,
            value: stiffness,
        }];
        let source = FormationSource {
            node_count: 2,
            loads: force.load_primitives().into(),
            springs: vec![(0, stiffness)],
            ..FormationSource::default()
        };
        let checked =
            StructuralSystem::assembled(&k, &force, &[0], &prescribed, Some(&contributions), None)
                .with_formation_source(&source);
        let solution = super::super::solve_formation_checked_structural_dense(&checked).unwrap();
        assert_eq!(
            solution.report.quality,
            SolveQuality::Passed,
            "b={exponent}"
        );
        let bound = solution.member_bound.as_ref().unwrap();
        let mut corrected = ExactAccumulator::new();
        corrected.add(solution.displacements[0]).unwrap();
        corrected.add(bound.w[0]).unwrap();
        corrected.add(bound.w2[0]).unwrap();
        assert_eq!(corrected.round().unwrap(), 1.0, "b={exponent}");
        let physical_remainder = mul_up(scale_up(2.0, bound.exponents[0]), bound.lambda_sigma);
        let scaled_remainder = mul_up(
            scale_up(2.0, exponent + bound.exponents[0]),
            bound.lambda_sigma,
        );
        let unscaled_remainder = scale_up(scaled_remainder, -exponent);
        assert!(
            (physical_remainder / unscaled_remainder - 1.0).abs() < 1e-14,
            "b={exponent}: physical and scaled-A remainder laws"
        );
        assert!(physical_remainder > 0.0);
        if let Some(base) = baseline {
            // Even exponents retain this fixture's factor arithmetic. Odd
            // attempts can have a tighter s2; no cross-attempt Lambda law is assumed.
            if exponent.rem_euclid(2) == 0 {
                assert!(
                    (physical_remainder / base - 1.0).abs() < 1e-12,
                    "b={exponent}: R={physical_remainder}, base={base}"
                );
            }
        } else {
            baseline = Some(physical_remainder);
        }
        let before = bound.clone();
        let published = super::super::unscale_structural_solution(
            solution,
            scale,
            &input_ledger(&[2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
        );
        assert_eq!(published.solution.member_bound.unwrap(), before);
    }
}
#[test]
fn kd5_logarithmic_strain_encloses_independent_p256_series() {
    use super::super::retained::wide::multi::WideContext;
    use super::super::retained::wide::Wide;
    let points = vec![
        ThermalPointPrimitive {
            temperature_kelvin: 0.0,
            value: 0.25,
        },
        ThermalPointPrimitive {
            temperature_kelvin: 2.0,
            value: 0.25,
        },
    ];
    let strain = EigenstrainPrimitive {
        thermal: ThermalPrimitive::FreeLengthState {
            installation_kelvin: 0.0,
            operating_kelvin: 2.0,
            law: ExpansionLawPrimitive::LogarithmicPerCurrentLength {
                datum_kelvin: 0.0,
                points,
            },
        },
        fit: FitPrimitive::None,
    };
    let (value, radius) = load_reformation(&primitive(
        LoadFormula::FreeStrainStraight {
            frame: axial_frame(),
            strain,
        },
        6,
        0,
    ))
    .unwrap();
    let mut a = WideContext::<4>::new(256).unwrap();
    let one = Wide::<4>::from_f64(1.0).unwrap();
    let x = Wide::<4>::from_f64(0.5).unwrap();
    let mut term = one;
    let mut sum = one;
    for n in 1..=100 {
        let product = a.mul(&term, &x).unwrap();
        term = a
            .div(&product, &Wide::<4>::from_f64(n as f64).unwrap())
            .unwrap();
        sum = a.add(&sum, &term).unwrap();
    }
    let strain = a.sub(&sum, &one).unwrap();
    let reference = a.mul(&strain, &Wide::<4>::from_f64(16.0).unwrap()).unwrap();
    let error = a.sub(&value[6].widen::<4>(), &reference).unwrap().abs();
    assert!(error
        .cmp_value(&Wide::<4>::from_f64(radius[6]).unwrap())
        .is_le());
    assert!(radius[6] < 1e-30);
}

#[test]
fn kd5_sparse_and_dense_closed_source_and_drop_one_use_the_same_boundary() {
    use super::super::{
        factor_sparse_structural_profile, finish_sparse_structural,
        prepare_formation_checked_sparse_structural, SparseStiffness, SparseStructuralSystem,
    };
    let f = axial_frame();
    let recipe = LoadFormula::UniformStraight {
        frame: f,
        intensity: IntensityPrimitive::Authored([0.5, 0.0, 0.0]),
        start_fraction: 0.0,
        end_fraction: 1.0,
    };
    let mut ledger = crate::load_ledger::LoadLedger::new();
    ledger.push_formed(
        "group",
        0,
        2.0,
        crate::load_ledger::Formation::CannotBound,
        0.0,
        false,
    );
    let mut expected = Vec::new();
    for ordinal in 0..2 {
        expected.push(primitive(recipe.clone(), 0, ordinal).closure_entry());
    }
    ledger.set_closure_manifest(0, expected);
    for ordinal in 0..2 {
        ledger.attach_load_primitive(primitive(recipe.clone(), 0, ordinal));
    }
    let force = ledger.finish(12).unwrap();
    let contributions = [super::super::StiffnessContribution {
        row: 0,
        col: 0,
        value: 2.0,
    }];
    let sparse = SparseStiffness::from_contributions(12, &contributions).unwrap();
    let dense = sparse.to_dense();
    let prescribed: Vec<_> = (1..12).map(|i| (i, 0.0)).collect();
    for drop_one in [false, true] {
        let mut source = FormationSource {
            node_count: 2,
            loads: force.load_primitives().into(),
            springs: vec![(0, 2.0)],
            ..FormationSource::default()
        };
        if drop_one {
            source.loads.to_mut().pop();
        }
        let checked = StructuralSystem::assembled(
            &dense,
            &force,
            &[0],
            &prescribed,
            Some(&contributions),
            None,
        )
        .with_formation_source(&source);
        let d = super::super::solve_formation_checked_structural_dense(&checked).unwrap();
        let checked = SparseStructuralSystem::assembled(
            &sparse,
            &force,
            &[0],
            &prescribed,
            Some(&contributions),
            None,
        )
        .with_formation_source(&source);
        let prepared = prepare_formation_checked_sparse_structural(&checked).unwrap();
        let factor = factor_sparse_structural_profile(&prepared, &[0], &[0]).unwrap();
        let s = finish_sparse_structural(&factor).unwrap();
        assert!((d.displacements[0] - s.displacements[0]).abs() <= f64::EPSILON);
        assert_eq!(d.report.quality, s.report.quality);
        if drop_one {
            assert_eq!(
                d.formation_check.as_ref().unwrap().reason,
                s.formation_check.as_ref().unwrap().reason
            );
            assert!(d.member_bound.is_none() && s.member_bound.is_none());
        } else {
            for result in [&d, &s] {
                let bound = result.member_bound.as_ref().unwrap();
                let mut corrected = ExactAccumulator::new();
                corrected.add(result.displacements[0]).unwrap();
                corrected.add(bound.w[0]).unwrap();
                corrected.add(bound.w2[0]).unwrap();
                assert_eq!(corrected.round().unwrap(), 1.0);
                assert!(bound.lambda_sigma.is_finite());
            }
        }
        assert_eq!(
            d.report.quality,
            if drop_one {
                SolveQuality::Sensitive
            } else {
                SolveQuality::Passed
            }
        );
    }
}

#[test]
fn kd5_curved_pressure_families_enclose_independent_p256_operands() {
    use super::super::retained::wide::multi::WideContext;
    use super::super::retained::wide::Wide;
    let arc = quarter_arc();
    let pi = Wide::<4>::from_parts(
        false,
        1,
        [
            0x020bbea63b139b22,
            0x29024e088a67cc74,
            0xc4c6628b80dc1cd1,
            0xc90fdaa22168c234,
        ],
    )
    .unwrap();
    let mut a = WideContext::<4>::new(256).unwrap();
    let two = Wide::<4>::from_f64(2.0).unwrap();
    let root = a.sqrt(&two).unwrap();
    let halfpi = pi.mul_pow2(-1).unwrap();
    for (direction, reference) in [
        (
            PressureDirection::Chord { axis: 0 },
            a.div(&halfpi, &root).unwrap().neg(),
        ),
        (
            PressureDirection::Tangent {
                end_j: false,
                axis: 0,
            },
            Wide::<4>::from_f64(0.0).unwrap(),
        ),
        (
            PressureDirection::Tangent {
                end_j: true,
                axis: 0,
            },
            halfpi.neg(),
        ),
    ] {
        let formula = LoadFormula::PressureOperand {
            geometry: PressureGeometry::Curved { arc },
            pressure: 2.0,
            outer_diameter: 2.0,
            wall_thickness: 0.5,
            coefficient: PressureCoefficient::Constant(1.0),
            direction,
        };
        let (v, r) = load_reformation(&primitive(formula, 0, 0)).unwrap();
        let error = a.sub(&v[0].widen::<4>(), &reference).unwrap().abs();
        assert!(error.cmp_value(&Wide::<4>::from_f64(r[0]).unwrap()).is_le());
    }
    let (v, r) = load_reformation(&primitive(
        LoadFormula::PressureFreeStrainCurved {
            arc,
            pressure: 2.0,
            outer_diameter: 2.0,
            wall_thickness: 0.5,
            poisson_ratio: 0.125,
            column: None,
        },
        0,
        0,
    ))
    .unwrap();
    let factor = Wide::<4>::from_f64(3.0 / 128.0).unwrap();
    let eps = a.mul(&pi, &factor).unwrap();
    let mut arith = WideArith::new(128).unwrap();
    let k = curved_matrix(&mut arith, &arc).unwrap();
    for row in 0..12 {
        let left = a.mul(&k[row][6].widen::<4>(), &eps.neg()).unwrap();
        let right = a.mul(&k[row][7].widen::<4>(), &eps).unwrap();
        let reference = a.add(&left, &right).unwrap();
        let error = a.sub(&v[row].widen::<4>(), &reference).unwrap().abs();
        assert!(error
            .cmp_value(&Wide::<4>::from_f64(r[row]).unwrap())
            .is_le());
    }
}
#[test]
fn kd5_all_prescribed_formed_load_reformation_failure_is_named() {
    let f = axial_frame();
    let formula = LoadFormula::UniformStraight {
        frame: f,
        intensity: IntensityPrimitive::Authored([1.0, 0.0, 0.0]),
        start_fraction: 1.0,
        end_fraction: 0.0,
    };
    let p = primitive(formula, 0, 0);
    let mut ledger = crate::load_ledger::LoadLedger::new();
    ledger.push_formed(
        "bad prescribed uniform",
        0,
        0.0,
        crate::load_ledger::Formation::CannotBound,
        0.0,
        false,
    );
    ledger.set_closure_manifest(0, vec![p.closure_entry()]);
    ledger.attach_load_primitive(p);
    let force = ledger.finish(12).unwrap();
    let source = FormationSource {
        node_count: 2,
        loads: force.load_primitives().into(),
        ..FormationSource::default()
    };
    let prescribed: Vec<_> = (0..12).map(|i| (i, 0.0)).collect();
    let system = StructuralSystem {
        stiffness: &[],
        force: force.values(),
        free_dofs: &[],
        prescribed: &prescribed,
        contributions: Some(&[]),
        contribution_groups: None,
        symmetry: None,
    };
    let (finding, bound) = check(
        &system,
        &source,
        Some(&force),
        &[],
        &[0.0; 12],
        (1.0, 0.0, 0.0),
        &|_, _| 0.0,
        &|_, _| 0.0,
        &|_| Ok(Vec::new()),
    );
    assert!(bound.is_none());
    assert!(
        matches!(finding.unwrap().reason,FormationCheckReason::FormationCheckUnavailable{detail}if detail.contains("load closure: bad prescribed uniform: reformation"))
    );
}

#[test]
fn kd5_fit_reference_lengths_are_reformed_from_raw_straight_and_curved_geometry() {
    use super::super::retained::wide::multi::WideContext;
    use super::super::retained::wide::Wide;
    let mut ref_arith = WideContext::<4>::new(256).unwrap();
    let change = Wide::<4>::from_f64(0.125).unwrap();
    let points = ([7.3e6, 500000.0, 20.0], [7.3e6 + 1.0, 500001.0, 21.0]);
    let length = ref_arith.sqrt(&Wide::<4>::from_f64(3.0).unwrap()).unwrap();
    let reference = ref_arith.div(&change, &length).unwrap();
    let straight = EigenstrainPrimitive {
        thermal: ThermalPrimitive::UnchangedReference,
        fit: FitPrimitive::NaturalLengthChangeStraight {
            change_m: 0.125,
            coordinates_i: points.0,
            coordinates_j: points.1,
        },
    };
    let mut balls = Balls::new().unwrap();
    let result = eigenstrain_reformation(&mut balls, &straight).unwrap();
    let error = ref_arith
        .sub(&result.m.widen::<4>(), &reference)
        .unwrap()
        .abs();
    assert!(error
        .cmp_value(&Wide::<4>::from_f64(result.r).unwrap())
        .is_le());
    assert!(result.r < 1e-30);
    // A binary64 lift of the derived sqrt(3) loses real source bits and is
    // detectably outside the raw geometry enclosure.
    let held = EigenstrainPrimitive {
        thermal: ThermalPrimitive::UnchangedReference,
        fit: FitPrimitive::NaturalLengthChange {
            change_m: 0.125,
            reference_length_m: 3.0f64.sqrt(),
        },
    };
    let held = eigenstrain_reformation(&mut Balls::new().unwrap(), &held).unwrap();
    let error = ref_arith
        .sub(&held.m.widen::<4>(), &reference)
        .unwrap()
        .abs();
    assert!(error
        .cmp_value(&Wide::<4>::from_f64(result.r).unwrap())
        .is_gt());
    let pi = Wide::<4>::from_parts(
        false,
        1,
        [
            0x020bbea63b139b22,
            0x29024e088a67cc74,
            0xc4c6628b80dc1cd1,
            0xc90fdaa22168c234,
        ],
    )
    .unwrap();
    let arc = quarter_arc();
    let reference = ref_arith.div(&change, &pi.mul_pow2(-1).unwrap()).unwrap();
    let curved = EigenstrainPrimitive {
        thermal: ThermalPrimitive::UnchangedReference,
        fit: FitPrimitive::NaturalLengthChangeCurved {
            change_m: 0.125,
            arc,
        },
    };
    let result = eigenstrain_reformation(&mut Balls::new().unwrap(), &curved).unwrap();
    let error = ref_arith
        .sub(&result.m.widen::<4>(), &reference)
        .unwrap()
        .abs();
    assert!(error
        .cmp_value(&Wide::<4>::from_f64(result.r).unwrap())
        .is_le());
    assert!(result.r < 1e-30);
    let held = EigenstrainPrimitive {
        thermal: ThermalPrimitive::UnchangedReference,
        fit: FitPrimitive::NaturalLengthChange {
            change_m: 0.125,
            reference_length_m: std::f64::consts::FRAC_PI_2,
        },
    };
    let held = eigenstrain_reformation(&mut Balls::new().unwrap(), &held).unwrap();
    let error = ref_arith
        .sub(&held.m.widen::<4>(), &reference)
        .unwrap()
        .abs();
    assert!(error
        .cmp_value(&Wide::<4>::from_f64(result.r).unwrap())
        .is_gt());
    // Midpoint uses exactly the K_int geometry path and its length at p128.
    let mut arith = WideArith::new(128).unwrap();
    let geometry = curved_geometry_values(&mut arith, &arc).unwrap();
    let length = arith.mul(&geometry.radius, &geometry.phi).unwrap();
    let expected = arith
        .div(&Wide2::from_f64(0.125).unwrap(), &length)
        .unwrap();
    assert_eq!(result.m, expected);
}
#[test]
fn kd5_fit_wrong_raw_geometry_binding_refuses_on_ordinary_and_w2() {
    let frame = axial_frame();
    let recipes = [
        LoadFormula::FreeStrainStraight {
            frame,
            strain: EigenstrainPrimitive {
                thermal: ThermalPrimitive::UnchangedReference,
                fit: FitPrimitive::NaturalLengthChangeStraight {
                    change_m: 0.125,
                    coordinates_i: frame.node_i.coordinates,
                    coordinates_j: frame.node_j.coordinates,
                },
            },
        },
        LoadFormula::FreeStrainCurved {
            arc: quarter_arc(),
            strain: EigenstrainPrimitive {
                thermal: ThermalPrimitive::UnchangedReference,
                fit: FitPrimitive::NaturalLengthChangeCurved {
                    change_m: 0.125,
                    arc: quarter_arc(),
                },
            },
            column: None,
        },
    ];
    for recipe in recipes {
        let p = primitive(recipe, 0, 0);
        let mut ledger = crate::load_ledger::LoadLedger::new();
        ledger.push_formed(
            "raw fit",
            0,
            0.0,
            crate::load_ledger::Formation::CannotBound,
            0.0,
            true,
        );
        ledger.set_closure_manifest(0, vec![p.closure_entry()]);
        ledger.attach_load_primitive(p);
        let original = ledger.finish(12).unwrap();
        for exponent in [0, 6] {
            let force = original
                .force_scaled(crate::ForceScale::new(exponent).unwrap())
                .unwrap();
            let mut source = FormationSource {
                node_count: 2,
                loads: force.load_primitives().into(),
                ..FormationSource::default()
            };
            load_closure(&source, &force).unwrap();
            let strain = match &mut source.loads.to_mut()[0].formula {
                LoadFormula::FreeStrainStraight { strain, .. }
                | LoadFormula::FreeStrainCurved { strain, .. } => strain,
                _ => unreachable!(),
            };
            match &mut strain.fit {
                FitPrimitive::NaturalLengthChangeStraight { coordinates_j, .. } => {
                    coordinates_j[0] = coordinates_j[0].next_up()
                }
                FitPrimitive::NaturalLengthChangeCurved { arc, .. } => {
                    arc.radius = arc.radius.next_up()
                }
                _ => unreachable!(),
            };
            assert!(
                matches!(load_closure(&source,&force),Err(Failure::Closure(detail))if detail.contains("wrong operand binding"))
            );
        }
    }
}

#[test]
fn kd5_raw_eigenstrain_zero_query_rejects_binary64_cancellation_and_enclosure_noise() {
    let step = 2.0f64.powi(-52);
    let alpha = 1.0 + step;
    let dt = 1.0 - step;
    let fit = -0.5;
    let folded_thermal = alpha * dt;
    let mut ordinary = ExactAccumulator::new();
    ordinary.add(fit).unwrap();
    ordinary.add(folded_thermal).unwrap();
    ordinary.add_product(fit, folded_thermal).unwrap();
    assert_eq!(ordinary.round().unwrap(), 0.0);
    let raw = EigenstrainPrimitive {
        thermal: ThermalPrimitive::ConstantAlphaInterval {
            coefficient_per_kelvin: alpha,
            temperature_change_kelvin: dt,
        },
        fit: FitPrimitive::EngineeringStrain { strain: fit },
    };
    let intended = eigenstrain_reformation(&mut Balls::new().unwrap(), &raw).unwrap();
    assert_eq!(intended.m, Wide2::from_f64(-2.0f64.powi(-105)).unwrap());
    assert!(!eigenstrain_is_exact_zero(&raw));
    // All terms in the reference polynomial are independent exact binary64
    // products/sums: (1+h)(1-h)/2-1/2 = -h²/2 = -2^-105.
    let mut reference = ExactAccumulator::new();
    reference.add(-2.0f64.powi(-105)).unwrap();
    for &term in intended.m.split_binary64().unwrap().terms() {
        reference.add(-term).unwrap();
    }
    assert!(reference.is_zero());
    let raw = EigenstrainPrimitive {
        thermal: ThermalPrimitive::ExplicitIntervalStrain { strain: 1.0 },
        fit: FitPrimitive::EngineeringStrain { strain: -0.5 },
    };
    let intended = eigenstrain_reformation(&mut Balls::new().unwrap(), &raw).unwrap();
    assert!(intended.m.is_zero() && intended.r > 0.0);
    assert!(!eigenstrain_is_exact_zero(&raw));
    for fit in [
        FitPrimitive::None,
        FitPrimitive::EngineeringStrain { strain: 0.0 },
        FitPrimitive::NaturalLengthChangeStraight {
            change_m: 0.0,
            coordinates_i: [0.0; 3],
            coordinates_j: [1.0, 1.0, 1.0],
        },
        FitPrimitive::NaturalLengthChangeCurved {
            change_m: 0.0,
            arc: quarter_arc(),
        },
    ] {
        let raw = EigenstrainPrimitive {
            thermal: ThermalPrimitive::UnchangedReference,
            fit,
        };
        assert!(eigenstrain_is_exact_zero(&raw));
    }
    let raw = EigenstrainPrimitive {
        thermal: ThermalPrimitive::UnchangedReference,
        fit: FitPrimitive::NaturalLengthChange {
            change_m: 0.0,
            reference_length_m: 0.0,
        },
    };
    assert!(!eigenstrain_is_exact_zero(&raw));
}
#[test]
fn kd5_nodal_source_factor_reforms_exact_product_and_binds_its_source_and_dof() {
    let step = 2.0f64.powi(-52);
    let magnitude = 1.0 + step;
    let factor = 1.0 - step;
    let formula = LoadFormula::NodalProduct {
        global_dof: 10,
        magnitude,
        factor,
    };
    let p = primitive(formula, 4, 0);
    let (value, radius) = load_reformation(&p).unwrap();
    let mut reference = ExactAccumulator::new();
    reference.add_product(magnitude, factor).unwrap();
    for &term in value[4].split_binary64().unwrap().terms() {
        reference.add(-term).unwrap();
    }
    assert!(reference.is_zero());
    assert!(!value[4].cmp_value(&Wide2::ONE).is_eq());
    assert!(radius[4].is_finite());
    let mut ledger = crate::load_ledger::LoadLedger::new();
    ledger.push_formed(
        "factored nodal source",
        10,
        magnitude * factor,
        crate::load_ledger::Formation::RoundedProduct {
            k: 1.0,
            a: magnitude,
            b: factor,
        },
        0.0,
        false,
    );
    ledger.set_closure_manifest(0, vec![p.closure_entry()]);
    ledger.attach_load_primitive(p);
    let original = ledger.finish(12).unwrap();
    for exponent in [0, 6] {
        let force = original
            .force_scaled(crate::ForceScale::new(exponent).unwrap())
            .unwrap();
        let source = FormationSource {
            node_count: 2,
            loads: force.load_primitives().into(),
            ..FormationSource::default()
        };
        load_closure(&source, &force).unwrap();
        for wrong_dof in [false, true] {
            let mut mutant = source.clone();
            if let LoadFormula::NodalProduct {
                global_dof, factor, ..
            } = &mut mutant.loads.to_mut()[0].formula
            {
                if wrong_dof {
                    *global_dof = 9;
                } else {
                    *factor = factor.next_up();
                }
            }
            assert!(
                matches!(load_closure(&mutant,&force),Err(Failure::Closure(detail))if detail.contains(if wrong_dof{"nodal source DOF binding"}else{"wrong operand binding"}))
            );
        }
    }
}

#[test]
fn kd5_positive_w2_lambda_sigma_unscale_mutant_underbounds_actual_reference_error() {
    use super::super::retained::wide::multi::WideContext;
    use super::super::retained::wide::Wide;
    // The public even-b admission and actual prepared Cholesky/gate algorithm
    // are used. K_int=3, f_int=1 have independent truth u*=1/3. The affine
    // force row A*u-f with physical A=3 has intended truth exactly zero.
    // There are no load enclosure radii; the third residual is nonzero.
    let exponent = 14;
    let scale = crate::ForceScale::new(exponent).unwrap();
    let original = input_ledger(&[1.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    let force = original.force_scaled(scale).unwrap();
    let stiffness = 3.0 * 2.0f64.powi(exponent);
    let mut k = vec![vec![0.0; 6]; 6];
    k[0][0] = stiffness;
    let contributions = [super::super::StiffnessContribution {
        row: 0,
        col: 0,
        value: stiffness,
    }];
    let prescribed: Vec<_> = (1..6).map(|i| (i, 0.0)).collect();
    let source = FormationSource {
        node_count: 1,
        springs: vec![(0, stiffness)],
        ..FormationSource::default()
    };
    let checked =
        StructuralSystem::assembled(&k, &force, &[0], &prescribed, Some(&contributions), None)
            .with_formation_source(&source);
    let scaled_solution = super::super::solve_formation_checked_structural_dense(&checked).unwrap();
    // Exercise the actual force-unscale entry too: a production mutant that
    // scales LambdaSigma alone here must fail the independent error comparison.
    let solution =
        super::super::unscale_structural_solution(scaled_solution, scale, &original).solution;
    assert_eq!(solution.report.quality, SolveQuality::Passed);
    let bound = solution.member_bound.as_ref().unwrap();
    // Independent exact accumulator evaluation of the intended row. In
    // particular this must not become zero by rounding corrected u first.
    let mut reference_error = ExactAccumulator::new();
    reference_error.add(-1.0).unwrap();
    for value in [solution.displacements[0], bound.w[0], bound.w2[0]] {
        reference_error.add_product(3.0, value).unwrap();
    }
    assert!(!reference_error.is_zero());
    let actual = reference_error.round().unwrap().abs();
    assert!(actual > 0.0);
    // Independent p256 solve and corrected-row evaluation agree with the exact
    // dyadic residual, well beyond the precision of the tested f64 bounds.
    let mut arithmetic = WideContext::<4>::new(256).unwrap();
    let one = Wide::<4>::from_f64(1.0).unwrap();
    let three = Wide::<4>::from_f64(3.0).unwrap();
    let truth = arithmetic.div(&one, &three).unwrap();
    let mut corrected = Wide::<4>::from_f64(solution.displacements[0]).unwrap();
    for value in [bound.w[0], bound.w2[0]] {
        corrected = arithmetic
            .add(&corrected, &Wide::<4>::from_f64(value).unwrap())
            .unwrap();
    }
    let displacement_error = arithmetic.sub(&corrected, &truth).unwrap();
    let error = arithmetic.mul(&three, &displacement_error).unwrap().abs();
    let measured = Wide::<4>::from_f64(actual).unwrap();
    let reference_difference = arithmetic.sub(&error, &measured).unwrap().abs();
    assert!(reference_difference
        .cmp_value(&measured.mul_pow2(-80).unwrap())
        .is_lt());
    let operator_norm = scale_up(3.0, bound.exponents[0]);
    let correct = mul_up(operator_norm, bound.lambda_sigma);
    let mutant = mul_up(operator_norm, scale_up(bound.lambda_sigma, -exponent));
    assert!(
        Wide::<4>::from_f64(correct)
            .unwrap()
            .cmp_value(&error)
            .is_ge(),
        "the actual member remainder must enclose the intended row error"
    );
    assert!(
        Wide::<4>::from_f64(mutant)
            .unwrap()
            .cmp_value(&error)
            .is_lt(),
        "standalone LambdaSigma unscaling underbounds the actual nonzero intended error"
    );
    eprintln!("positive W2 b={exponent}: actual={actual:e}, R={correct:e}, mutant={mutant:e}, R/actual={}, mutant/actual={}",correct/actual,mutant/actual);
}
/// §2 step 7 and §7 ("drop the 2^10"): Λ = 2^10·est/(1 − 2^10·est·ε_E). A
/// seeded inexact solve leaves the exact second residual s₂ = −2^-89 (σ = 2^-89,
/// ψ = 0, ε_E = 0), so Λσ is 2^10·est·2^-89 within its few upward roundings; and
/// est·ε_E < 1 ≤ 2^10·est·ε_E must fail closed on the Neumann condition.
#[test]
fn kd5_lambda_carries_the_two_to_ten_inverse_factor_and_its_neumann_boundary() {
    let force = input_ledger(&[2.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    let (k, source) = spring_system(force.values());
    let contributions = [super::super::StiffnessContribution { row: 0, col: 0, value: 2.0 }];
    let prescribed = [(1, 0.0), (2, 0.0), (3, 0.0), (4, 0.0), (5, 0.0)];
    let system = StructuralSystem {
        stiffness: &k,
        force: force.values(),
        free_dofs: &[0],
        prescribed: &prescribed,
        contributions: Some(&contributions),
        contribution_groups: None,
        symmetry: None,
    };
    // u = 1 + 2^-40: ρ = −2^-39; the solve returns (rhs/2)(1 + 2^-25), exactly.
    let u = [1.0 + 2.0f64.powi(-40), 0.0, 0.0, 0.0, 0.0, 0.0];
    let inexact = |rhs: &[f64]| Ok(vec![rhs[0] / 2.0 * (1.0 + 2.0f64.powi(-25))]);
    let est = 0.5;
    let (finding, bound) = check(&system, &source, Some(&force), &[0], &u, (1.0, est, 2.0), &|i, j| k[i][j], &|_, _| 2.0, &inexact);
    assert!(finding.is_none(), "{finding:?}");
    let bound = bound.unwrap();
    let sigma = 2.0f64.powi(-89);
    let expected = 1024.0 * est * sigma;
    assert!(bound.lambda_sigma >= expected, "Λσ={:e} must carry 2^10·est·σ={expected:e}", bound.lambda_sigma);
    assert!(bound.lambda_sigma <= expected * (1.0 + 2.0f64.powi(-46)), "Λσ={:e} vs {expected:e}", bound.lambda_sigma);
    // ε_E is the assembly rounding 1 (stored 3 against the contributed 2):
    // est·ε_E = 1/64 < 1, but 2^10·est·ε_E = 16 ≥ 1.
    let exact = |rhs: &[f64]| Ok(vec![rhs[0] / 2.0]);
    let (finding, bound) = check(&system, &source, Some(&force), &[0], &[1.0, 0.0, 0.0, 0.0, 0.0, 0.0], (1.0, 1.0 / 64.0, 3.0),
        &|i, j| if i == j && i == 0 { 3.0 } else { 0.0 }, &|_, _| 3.0, &exact);
    assert!(bound.is_none());
    assert!(matches!(finding.unwrap().reason, FormationCheckReason::FormationCheckUnavailable { detail } if detail == "Neumann"));
}
