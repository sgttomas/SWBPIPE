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
    // Frame, joint and curved bend: rigid translation (1,−2,0.5) and rotation ω = (0.25,0.5,−0.125) (dyadic, so u is exact) about the
    // origin: K_e·u_rigid is zero to p for the re-formed frame and joint.
    let e = skew_frame();
    let user = UserStiffnessElement {
        node_i: e.node_i,
        node_j: e.node_j,
        y_reference: [1.0, 0.0, 0.0],
        axial_stiffness: 2.0e6,
        lateral_stiffness: 0.0,
        angular_stiffness: 3.0e4,
        torsional_stiffness: 5.0e4,
    };
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
        user_matrix(&mut arith, &user).unwrap(),
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
    let d = [grid(30.0 * (pi / 6.0).cos()), grid(30.0 * (pi / 6.0).sin()), 0.0];
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
            symmetry: None,
        };
        let plain = solve_structural_dense(&system()).unwrap();
        assert_eq!(plain.report.quality, SolveQuality::Passed, "phi {nominal}");
        let checked = system().with_formation_source(&source);
        let result = super::super::solve_formation_checked_structural_dense(&checked).unwrap();
        assert_eq!(result.formation_check, None, "phi {nominal}");
        assert_eq!(result.report.quality, SolveQuality::Passed, "phi {nominal}");
    }
}

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
