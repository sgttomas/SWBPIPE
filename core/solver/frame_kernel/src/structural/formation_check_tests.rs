//! Kernel-level K-D5 tests (the adapter's tests cover both solve modes and
//! the product models; `nonlinear_integration/src/structural_adapter/kd5_tests.rs`).
use super::super::{
    factor_structural_cholesky, prepare_formation_checked_structural, solve_structural_dense,
    SolveQuality, StructuralSystem, SymmetryEvidence,
};
use super::*;
use crate::load_ledger::ForceTermKind;
use crate::{FrameNode, FrameSection, Matrix12};

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
    // A realized quarter bend with dyadic geometry (so u is exact), E1's section.
    let bend = CurvedFormation {
        node_i: 0,
        node_j: 1,
        coordinates_i: e.node_i.coordinates,
        coordinates_j: e.node_j.coordinates,
        center: [1.0, 0.75, 1.0], // equidistant (squared radius 2.5625), included angle 139 degrees
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
fn spring_system(force: &[f64]) -> (Vec<Vec<f64>>, FormationSource) {
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
        symmetry: None,
    };
    let checked = StructuralSystem {
        stiffness: &k,
        force: &force,
        free_dofs: &[0],
        prescribed: &prescribed,
        contributions: None,
        symmetry: None,
    }
    .with_formation_source(&source);
    let prepared = prepare_formation_checked_structural(&checked).unwrap();
    let factor = factor_structural_cholesky(&prepared).unwrap();
    let solution = super::super::finish_structural(&factor).unwrap();
    assert_eq!(solution.displacements, vec![0.0; 6]);
    assert_eq!(solution.report.quality, SolveQuality::Passed);
    let term = |v: f64| ForceTerm {
        source: "ledger".into(),
        dof: 0,
        kind: ForceTermKind::Term(v),
    };
    let differing = [term(1e80), term(-1e80), term(1.0)];
    let cancelling = [term(1e80), term(-1e80)];
    let solve = |r: &[f64]| factor.solve(r);
    let fired = check(
        &system,
        &source,
        Some(&differing[..]),
        prepared.scale_exponents(),
        &solution.displacements,
        &solve,
    )
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
            Some(&cancelling[..]),
            prepared.scale_exponents(),
            &solution.displacements,
            &solve
        ),
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
            symmetry: None,
        };
        let plain = solve_structural_dense(&system).unwrap();
        let checked = system.with_formation_source(source);
        let result = super::super::solve_formation_checked_structural_dense(&checked).unwrap();
        (plain, result)
    };
    // Precondition: the exact spring re-forms and the case stays Passed.
    let (plain, result) = make(&source);
    assert_eq!(plain.report.quality, SolveQuality::Passed);
    assert_eq!(result.formation_check, None);
    assert_eq!(
        format!("{:?}", plain.report),
        format!("{:?}", result.report)
    );
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
    // Two AngleDomain refusals inside the check, each a WideError: the case is
    // demoted with formation_check_unavailable, never passed and never an Err.
    // (a) collinear radial vectors, sin φ = 0;
    // (b) addendum 3 (RV2 N5): an included angle 5e-20 below π, where cos φ
    //     rounds to exactly −1 at p = 128, so 1 + cos φ = 0. The product itself
    //     refuses a bend within 1e-9 of π; the kernel path fails closed anyway.
    let bend = |xi: [f64; 3], xj: [f64; 3]| CurvedFormation {
        node_i: 0,
        node_j: 1,
        coordinates_i: xi,
        coordinates_j: xj,
        center: [0.0, 0.0, 0.0],
        elastic_modulus: 200e9,
        shear_modulus: 80e9,
        area: 0.006,
        second_moment: 2.7e-5,
        torsion_constant: 5.4e-5,
        in_plane_flexibility_factor: 1.0,
        out_of_plane_flexibility_factor: 1.0,
    };
    let near_pi = bend([1.0, 0.0, 0.0], [-1.0, 5e-20, 0.0]);
    let angle = (5e-20_f64).atan2(-1.0);
    assert!(std::f64::consts::PI - angle < 1e-19);
    for (label, b) in [
        ("collinear", bend([1.0, 0.0, 0.0], [2.0, 0.0, 0.0])),
        ("near_pi", near_pi),
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
            symmetry: Some(SymmetryEvidence {
                absolute_roundoff: &roundoff,
                operation_counts: &counts,
                basis: "test",
            }),
        };
        // Precondition: the ordinary solve publishes Passed.
        let plain = solve_structural_dense(&system()).unwrap();
        assert_eq!(plain.report.quality, SolveQuality::Passed, "{label}");
        let checked = system().with_formation_source(&source);
        let result = super::super::solve_formation_checked_structural_dense(&checked).unwrap();
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
                symmetry: None,
            };
            let plain = solve_structural_dense(&system).unwrap();
            assert_eq!(plain.report.quality, SolveQuality::Passed, "X0 {x0}");
            let checked = system.with_formation_source(&source);
            let result = super::super::solve_formation_checked_structural_dense(&checked).unwrap();
            if perturbed {
                assert_eq!(result.report.quality, SolveQuality::Sensitive, "X0 {x0}");
                let check = result.formation_check.unwrap();
                assert_eq!(check.reason, FormationCheckReason::Estimate, "X0 {x0}");
            } else {
                assert_eq!(result.formation_check, None, "X0 {x0}");
                assert_eq!(
                    format!("{:?}", plain.report),
                    format!("{:?}", result.report)
                );
            }
        }
    }
}
