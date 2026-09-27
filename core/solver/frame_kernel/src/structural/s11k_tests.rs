//! S11-K tests K7, K11 and K12 (S11 section 9), plus the zero-sign pins the
//! T3 manager required for the legacy path. References are exact dyadic
//! values derived by hand in each test (the stiffness data are chosen so the
//! formed coefficients are exact powers-of-two multiples), never outputs of
//! the code under test. Every kill test first asserts its precondition: the
//! binary64 fold of its terms, in the producer's order, differs from the exact
//! net (S11B-6).
use super::*;
use crate::exact_sum::exact_rounded_dot;
use crate::load_ledger::{ForceTerm, ForceTermKind, LoadLedger};
use crate::{assemble_global_stiffness, FrameElement, FrameNode, FrameSection};

fn element(i: usize, j: usize, xi: f64, xj: f64, section: FrameSection) -> FrameElement {
    FrameElement::new(
        FrameNode::new(i, [xi, 0.0, 0.0]).unwrap(),
        FrameNode::new(j, [xj, 0.0, 0.0]).unwrap(),
        section,
        [0.0, 1.0, 0.0],
    )
    .unwrap()
}

// ---------------------------------------------------------------- K7

const L: f64 = 2.0;

fn probe_a_section() -> FrameSection {
    FrameSection::new(2.0e11, 7.7e10, 0.01, 8.0e-6, 9.0e-6, 1.7e-5).unwrap()
}

/// One full-span local-y uniform load's fixed-end terms, as straight_pipe
/// forms them: (root UY, root RZ, tip UY, tip RZ) at DOFs 1, 5, 7, 11.
fn uniform_terms(q: f64) -> [(usize, f64); 4] {
    let (a, b) = (0.0_f64, 1.0_f64);
    [
        (
            1,
            q * L * ((b - b.powi(3) + 0.5 * b.powi(4)) - (a - a.powi(3) + 0.5 * a.powi(4))),
        ),
        (
            5,
            q * L
                * L
                * ((0.5 * b * b - (2.0 / 3.0) * b.powi(3) + 0.25 * b.powi(4))
                    - (0.5 * a * a - (2.0 / 3.0) * a.powi(3) + 0.25 * a.powi(4))),
        ),
        (
            7,
            q * L * ((b.powi(3) - 0.5 * b.powi(4)) - (a.powi(3) - 0.5 * a.powi(4))),
        ),
        (
            11,
            q * L
                * L
                * (((-b.powi(3) / 3.0) + 0.25 * b.powi(4))
                    - ((-a.powi(3) / 3.0) + 0.25 * a.powi(4))),
        ),
    ]
}

fn uniform_case(order: &[f64]) -> Vec<ForceTerm> {
    let mut terms = Vec::new();
    for (k, &q) in order.iter().enumerate() {
        for (dof, value) in uniform_terms(q) {
            terms.push(ForceTerm {
                source: format!("load:{k}"),
                dof,
                kind: ForceTermKind::Term(value),
            });
        }
    }
    terms
}

fn nodal(terms: &[(usize, f64)]) -> Vec<ForceTerm> {
    terms
        .iter()
        .enumerate()
        .map(|(k, &(dof, value))| ForceTerm {
            source: format!("nodal:{k}"),
            dof,
            kind: ForceTermKind::Term(value),
        })
        .collect()
}

/// Today's binary64 force: each DOF folded in producer (push) order.
fn legacy_force(terms: &[ForceTerm], n: usize) -> Vec<f64> {
    let mut force = vec![0.0; n];
    for t in terms {
        if let ForceTermKind::Term(v) = t.kind {
            force[t.dof] += v;
        }
    }
    force
}

fn exact_net(terms: &[ForceTerm], dof: usize) -> f64 {
    crate::exact_sum::exact_rounded_sum(terms.iter().filter(|t| t.dof == dof).map(
        |t| match t.kind {
            ForceTermKind::Term(v) => v,
            ForceTermKind::Product(..) => unreachable!(),
        },
    ))
    .unwrap()
}

struct Cantilever {
    k: Vec<Vec<f64>>,
    free: Vec<usize>,
    fixed: Vec<(usize, f64)>,
}

fn cantilever() -> Cantilever {
    Cantilever {
        k: assemble_global_stiffness(2, &[element(0, 1, 0.0, L, probe_a_section())]).unwrap(),
        free: (6..12).collect(),
        fixed: (0..6).map(|d| (d, 0.0)).collect(),
    }
}

fn c3_detect(terms: &[ForceTerm]) -> StructuralSolution {
    let c = cantilever();
    let force = legacy_force(terms, 12);
    let system = StructuralSystem {
        stiffness: &c.k,
        force: &force,
        free_dofs: &c.free,
        prescribed: &c.fixed,
        contributions: None,
        symmetry: None,
    };
    solve_structural_dense_with_force_terms(&system, terms).unwrap()
}

fn c3_full(terms: &[ForceTerm]) -> StructuralSolution {
    let c = cantilever();
    let mut ledger = LoadLedger::new();
    for t in terms {
        match t.kind {
            ForceTermKind::Term(v) => ledger.push(t.source.clone(), t.dof, v),
            ForceTermKind::Product(a, b) => ledger.push_product(t.source.clone(), t.dof, a, b),
        }
    }
    let force = ledger.finish(12).unwrap();
    let system = StructuralSystem::assembled(&c.k, &force, &c.free, &c.fixed, None, None);
    solve_assembled_structural_dense(&system).unwrap()
}

fn differs_somewhere(terms: &[ForceTerm]) -> bool {
    let force = legacy_force(terms, 12);
    (0..12).any(|d| {
        force[d].to_bits() != (exact_net(terms, d) + 0.0).to_bits()
            && force[d] != exact_net(terms, d)
    })
}

#[test]
fn k7_audit_flags_every_probe_a_row_from_1e5_and_the_full_route_passes() {
    for (g, n) in [(1e5, 0.3), (1e6, 0.3), (1e7, 0.3), (1e8, 0.3), (1e80, 1e-8)] {
        for order in [[g, n, -g], [n, g, -g]] {
            let terms = uniform_case(&order);
            assert!(differs_somewhere(&terms), "precondition {order:?}");
            let detect = c3_detect(&terms);
            let report = detect.load_fidelity.as_ref().expect("flagged");
            assert_eq!(detect.report.quality, SolveQuality::Sensitive, "{order:?}");
            assert!(report.rows.iter().any(|r| !r.restrained), "{order:?}");
            for row in &report.rows {
                assert!(row.guarded_ratio > row.target);
                assert!(row.sources.iter().all(|s| s.starts_with("load:")));
                assert_eq!(row.completeness_limit, 1e-9 / row.target);
            }
            let full = c3_full(&terms);
            assert!(full.load_fidelity.is_none(), "{order:?}");
            assert_eq!(full.report.quality, SolveQuality::Passed);
        }
    }
}

#[test]
fn k7_audit_flags_v1_check_l_and_passes_every_control() {
    // V1 check L: tip UY (1e80, 1e-8, -1e80) with UZ 2e-8.
    let check_l = nodal(&[(7, 1e80), (7, 1e-8), (7, -1e80), (8, 2e-8)]);
    assert_eq!(legacy_force(&check_l, 12)[7], 0.0);
    let detect = c3_detect(&check_l);
    let flagged: Vec<usize> = detect
        .load_fidelity
        .unwrap()
        .rows
        .iter()
        .map(|r| r.global_dof)
        .collect();
    assert!(flagged.contains(&7));
    // Controls: every one passes, on both routes, and the Debug report of the
    // C3-detect route is byte-identical to the legacy route.
    for control in [
        uniform_case(&[1e8, -1e8, 0.3]),
        uniform_case(&[1e8, 0.3]),
        uniform_case(&[0.1, 0.2, 0.3]),
        uniform_case(&[1234.5, 987.25, 55.125]),
        uniform_case(&[1000.1, -1000.0, 0.05]),
    ] {
        let detect = c3_detect(&control);
        assert!(detect.load_fidelity.is_none());
        assert_eq!(detect.report.quality, SolveQuality::Passed);
        let c = cantilever();
        let force = legacy_force(&control, 12);
        let legacy = solve_structural_dense(&StructuralSystem {
            stiffness: &c.k,
            force: &force,
            free_dofs: &c.free,
            prescribed: &c.fixed,
            contributions: None,
            symmetry: None,
        })
        .unwrap();
        assert_eq!(
            format!("{:?}", detect.report),
            format!("{:?}", legacy.report)
        );
        assert_eq!(detect.displacements, legacy.displacements);
        let full = c3_full(&control);
        assert!(full.load_fidelity.is_none());
    }
}

#[test]
fn k7_restrained_root_row_is_audited() {
    // (1e8, 0.3, -1e8) N on the restrained root UY, plus a tip load.
    let terms = nodal(&[(1, 1e8), (1, 0.3), (1, -1e8), (7, 5.0)]);
    assert_ne!(legacy_force(&terms, 12)[1], 0.3, "precondition");
    let detect = c3_detect(&terms);
    let report = detect.load_fidelity.expect("restrained row flagged");
    let root = report
        .rows
        .iter()
        .find(|r| r.global_dof == 1)
        .expect("root row");
    assert!(root.restrained);
    assert_eq!(root.exact_net_bits, 0.3_f64.to_bits());
    assert_eq!(detect.report.quality, SolveQuality::Sensitive);
    assert!(c3_full(&terms).load_fidelity.is_none());
}

#[test]
fn k7_typed_route_report_is_byte_identical_to_legacy_when_forces_agree() {
    let c = cantilever();
    let mut ledger = LoadLedger::new();
    ledger.push("nodal:a", 7, 5.0);
    ledger.push("nodal:b", 11, -2.5);
    let force = ledger.finish(12).unwrap();
    let typed = solve_assembled_structural_dense(&StructuralSystem::assembled(
        &c.k, &force, &c.free, &c.fixed, None, None,
    ))
    .unwrap();
    let legacy = solve_structural_dense(&StructuralSystem {
        stiffness: &c.k,
        force: force.values(),
        free_dofs: &c.free,
        prescribed: &c.fixed,
        contributions: None,
        symmetry: None,
    })
    .unwrap();
    assert_eq!(
        format!("{:?}", typed.report),
        format!("{:?}", legacy.report)
    );
    assert_eq!(typed.displacements, legacy.displacements);
}

#[test]
fn k7_zero_witness_at_the_intended_action_residual() {
    // An exact-zero intended residual keeps the committed -0.0 (D-S11-1).
    // Scaled diagonal 4 * 2^-2 = 1, so the solve is exact: u = 2.
    let k = vec![vec![4.0]];
    let cs = [StiffnessContribution {
        row: 0,
        col: 0,
        value: 4.0,
    }];
    let system = StructuralSystem {
        stiffness: &k,
        force: &[8.0],
        free_dofs: &[0],
        prescribed: &[],
        contributions: Some(&cs),
        symmetry: None,
    };
    let solution = solve_structural_dense(&system).unwrap();
    let row = &solution.report.intended_residual_rows[0];
    assert_eq!(row.normalized_residual.to_bits(), (-0.0_f64).to_bits());
    assert_eq!(row.residual.to_bits(), (-0.0_f64).to_bits());
    // Every other zero in the kernel's exact paths is +0.0.
    let empty = Expansion::default();
    assert_eq!(empty.round().unwrap().to_bits(), 0);
    assert_eq!(empty.exact_scalar().unwrap().unwrap().to_bits(), 0);
}

// ---------------------------------------------------------------- K11

/// V1's probe P: two 3 m members along x, both end nodes fully restrained,
/// both ends settled by the same UY value g, a nodal moment m at the middle
/// node. Invented section data are chosen so every formed coefficient is exact:
/// E = 2^38 Pa and Iz = 9 * 2^-19 m^4 give k12 = 2^21, k6 = 3 * 2^20,
/// k4 = 3 * 2^21 and k2 = 3 * 2^20. The exact solution is then u_UY = g and
/// u_RZ = m / (2 k4), and the member moments are m/2 (near) and m/4 (far).
fn probe_p_section() -> FrameSection {
    FrameSection::new(
        2.0_f64.powi(38),
        7.7e10,
        0.01,
        1.0e-5,
        9.0 * 2.0_f64.powi(-19),
        1.7e-5,
    )
    .unwrap()
}

const M: f64 = 0.0137;

struct ProbeP {
    k: Vec<Vec<f64>>,
    elements: [FrameElement; 2],
    free: Vec<usize>,
    prescribed: Vec<(usize, f64)>,
    force: Vec<f64>,
}

fn probe_p(g: f64) -> ProbeP {
    let s = probe_p_section();
    let elements = [element(0, 1, 0.0, 3.0, s), element(1, 2, 3.0, 6.0, s)];
    let k = assemble_global_stiffness(3, &elements).unwrap();
    let mut prescribed: Vec<(usize, f64)> = (0..6).chain(12..18).map(|d| (d, 0.0)).collect();
    prescribed[1].1 = g;
    prescribed[7].1 = g;
    assert_eq!(prescribed[7].0, 13);
    let mut force = vec![0.0; 18];
    force[11] = M;
    ProbeP {
        k,
        elements,
        free: (6..12).collect(),
        prescribed,
        force,
    }
}

fn check_exact_coefficients(p: &ProbeP) {
    // The derivation of the exact answer relies on these exact values.
    assert_eq!(p.k[7][1], -(2.0_f64.powi(21)));
    assert_eq!(p.k[7][13], -(2.0_f64.powi(21)));
    assert_eq!(p.k[7][7], 2.0_f64.powi(22));
    assert_eq!(p.k[11][1], 3.0 * 2.0_f64.powi(20));
    assert_eq!(p.k[11][13], -3.0 * 2.0_f64.powi(20));
    assert_eq!(p.k[11][7], 0.0);
    assert_eq!(p.k[11][11], 3.0 * 2.0_f64.powi(22));
}

/// Member end moments (element 0 i and j, element 1 i and j) recovered with
/// the correctly rounded dot of the element row and its displacements (K1
/// verifies that dot against Fraction), so the solve is the only error source.
fn member_moments(p: &ProbeP, u: &[f64]) -> [f64; 4] {
    let mut out = [0.0; 4];
    for (e, element) in p.elements.iter().enumerate() {
        let ke = element.local_stiffness().unwrap();
        let ue = &u[6 * e..6 * e + 12];
        for (slot, row) in [5, 11].into_iter().enumerate() {
            out[2 * e + slot] =
                exact_rounded_dot(ke[row].iter().copied().zip(ue.iter().copied())).unwrap();
        }
    }
    out
}

fn assert_moments(p: &ProbeP, u: &[f64], what: &str) {
    let exact = [M / 4.0, M / 2.0, M / 2.0, M / 4.0];
    let scale = M / 2.0;
    for (observed, expected) in member_moments(p, u).into_iter().zip(exact) {
        let error = (observed - expected).abs();
        assert!(
            error <= 1e-9 * scale,
            "{what}: moment {observed:e} vs {expected:e}, error/scale {:e}",
            error / scale
        );
    }
}

fn precondition(p: &ProbeP) {
    // The kernel's order: prescribed list order.
    let mut b = p.force[11];
    for &(c, g) in &p.prescribed {
        b -= p.k[11][c] * g;
    }
    assert_ne!(
        b.to_bits(),
        M.to_bits(),
        "precondition: fold of f - sum K g is exact"
    );
}

const SETTLEMENTS: [f64; 2] = [0.05, 0.20];

#[test]
fn k11_reduced_right_hand_side_is_correctly_rounded() {
    for g in SETTLEMENTS {
        let p = probe_p(g);
        check_exact_coefficients(&p);
        precondition(&p);
        let dofs: Vec<usize> = p.prescribed.iter().map(|x| x.0).collect();
        let values: Vec<f64> = p.prescribed.iter().map(|x| x.1).collect();
        let reduced =
            crate::reduce_system_with_prescribed_displacements(&p.k, &p.force, &dofs, &values)
                .unwrap();
        // Exact: RZ row m (the couplings cancel exactly); UY row 2 * 2^21 * g.
        assert_eq!(reduced.force[5].to_bits(), M.to_bits());
        assert_eq!(reduced.force[1].to_bits(), (2.0_f64.powi(22) * g).to_bits());
        let mut ledger = LoadLedger::new();
        ledger.push("nodal:m", 11, M);
        let force = ledger.finish(18).unwrap();
        let typed = crate::reduce_assembled_system_with_prescribed_displacements(
            &p.k, &force, &dofs, &values,
        )
        .unwrap();
        assert_eq!(typed.force.values(), reduced.force.as_slice());
        // Through the generic dense kernel, the recovered moments are exact.
        let uf = crate::solve_dense(&reduced.stiffness, &reduced.force).unwrap();
        let mut u = vec![0.0; 18];
        for &(c, v) in &p.prescribed {
            u[c] = v;
        }
        for (r, &d) in reduced.free_dofs.iter().enumerate() {
            u[d] = uf[r];
        }
        assert_moments(&p, &u, "reduce_system_with_prescribed_displacements");
    }
}

#[test]
fn k11_dense_profile_and_typed_structural_paths_are_exact() {
    for g in SETTLEMENTS {
        let p = probe_p(g);
        precondition(&p);
        let system = StructuralSystem {
            stiffness: &p.k,
            force: &p.force,
            free_dofs: &p.free,
            prescribed: &p.prescribed,
            contributions: None,
            symmetry: None,
        };
        let dense = solve_structural_dense(&system).unwrap();
        assert_moments(&p, &dense.displacements, "dense");
        let prepared = prepare_structural(&system).unwrap();
        // KS1: the prepared RZ row is m, radix-scaled, bit for bit.
        assert_eq!(
            prepared.rhs[5].to_bits(),
            radix_scale(M, prepared.scale_exponents[5])
                .unwrap()
                .to_bits()
        );
        let n = p.free.len();
        let order: Vec<usize> = (0..n).collect();
        let first = vec![0; n];
        let profile =
            finish_structural(&factor_structural_profile(&prepared, &order, &first).unwrap())
                .unwrap();
        assert_moments(&p, &profile.displacements, "profile");
        let mut ledger = LoadLedger::new();
        ledger.push("nodal:m", 11, M);
        let force = ledger.finish(18).unwrap();
        let typed = solve_assembled_structural_dense(&StructuralSystem::assembled(
            &p.k,
            &force,
            &p.free,
            &p.prescribed,
            None,
            None,
        ))
        .unwrap();
        assert_moments(&p, &typed.displacements, "typed");
    }
}

#[test]
fn k11_forced_refinement_step_stays_exact() {
    // One refinement step is forced by perturbing the first solution; the
    // correction is solved from the KS3 residual rows.
    for g in SETTLEMENTS {
        let p = probe_p(g);
        precondition(&p);
        let system = StructuralSystem {
            stiffness: &p.k,
            force: &p.force,
            free_dofs: &p.free,
            prescribed: &p.prescribed,
            contributions: None,
            symmetry: None,
        };
        let prepared = prepare_structural(&system).unwrap();
        let factor = factor_structural_cholesky(&prepared).unwrap();
        let first_rhs = prepared.rhs.clone();
        let solution = finish_checked_factor(
            &system,
            &prepared,
            factor.pivots().to_vec(),
            "forced refinement test",
            |rhs: &[f64]| {
                let mut y = factor.solve(rhs)?;
                if rhs == first_rhs.as_slice() {
                    for v in &mut y {
                        *v *= 1.0 + 1e-6;
                    }
                }
                Ok(y)
            },
        )
        .unwrap();
        assert!(solution.report.refinement_attempts >= 1);
        assert_moments(&p, &solution.displacements, "forced refinement");
    }
}

// ------------------------------------------------ legacy zero-sign pins

#[test]
fn legacy_all_zero_prescribed_rows_keep_todays_zero_sign() {
    for coupling in [3.0, -3.0] {
        let k = vec![vec![4.0, coupling], vec![coupling, 4.0]];
        let force = [-0.0, 1.0];
        // Today's expression for the free row, bit for bit.
        let today = -0.0_f64 - coupling * 0.0;
        let reduced =
            crate::reduce_system_with_prescribed_displacements(&k, &force, &[1], &[0.0]).unwrap();
        assert_eq!(
            reduced.force[0].to_bits(),
            today.to_bits(),
            "coupling {coupling}"
        );
        let restrained = crate::reduce_system(&k, &force, &[1]).unwrap();
        assert_eq!(restrained.force[0].to_bits(), today.to_bits());
        let fixed = [(1, 0.0)];
        let system = StructuralSystem {
            stiffness: &k,
            force: &force,
            free_dofs: &[0],
            prescribed: &fixed,
            contributions: None,
            symmetry: None,
        };
        let prepared = prepare_structural(&system).unwrap();
        let b = checked_value(-0.0 - checked_product(coupling, 0.0).unwrap()).unwrap();
        assert_eq!(
            prepared.rhs[0].to_bits(),
            radix_scale(b, prepared.scale_exponents[0])
                .unwrap()
                .to_bits()
        );
    }
}

// ------------------------------------ ROOT option (c): named binary64 path

#[test]
fn option_c_binary64_variants_keep_todays_fold_on_probe_p() {
    for g in SETTLEMENTS {
        let p = probe_p(g);
        precondition(&p);
        // Today's fold of the RZ row, in prescribed-list order.
        let mut fold = p.force[11];
        for &(c, v) in &p.prescribed {
            fold -= p.k[11][c] * v;
        }
        let dofs: Vec<usize> = p.prescribed.iter().map(|x| x.0).collect();
        let values: Vec<f64> = p.prescribed.iter().map(|x| x.1).collect();
        let legacy = crate::reduce_system_with_prescribed_displacements_binary64(
            &p.k, &p.force, &dofs, &values,
        )
        .unwrap();
        assert_eq!(legacy.force[5].to_bits(), fold.to_bits());
        let exact =
            crate::reduce_system_with_prescribed_displacements(&p.k, &p.force, &dofs, &values)
                .unwrap();
        assert_ne!(exact.force[5].to_bits(), legacy.force[5].to_bits());
        let system = StructuralSystem {
            stiffness: &p.k,
            force: &p.force,
            free_dofs: &p.free,
            prescribed: &p.prescribed,
            contributions: None,
            symmetry: None,
        };
        let prepared = prepare_structural_binary64(&system).unwrap();
        let mut b = p.force[11];
        for &(c, v) in &p.prescribed {
            b = checked_value(b - checked_product(p.k[11][c], v).unwrap()).unwrap();
        }
        assert_eq!(
            prepared.rhs[5].to_bits(),
            radix_scale(b, prepared.scale_exponents[5])
                .unwrap()
                .to_bits()
        );
        let solution = solve_structural_dense_binary64(&system).unwrap();
        let dense = solve_structural_dense(&system).unwrap();
        assert_ne!(solution.displacements, dense.displacements);
    }
}

/// ROOT option (c) / V1 R5-3: the public `evaluate_original_residual` (called by
/// `product_equilibrium::evaluate` on the nonlinear loop path and in the gap
/// scrutiny) is today's binary64 evaluation on every row, including rows
/// coupled to a nonzero prescribed value; the exact KS3 numerator is only in
/// the solve's own residual and in the typed evaluation.
#[test]
fn option_c_public_original_residual_stays_binary64_on_coupled_rows() {
    for g in SETTLEMENTS {
        let p = probe_p(g);
        let system = StructuralSystem {
            stiffness: &p.k,
            force: &p.force,
            free_dofs: &p.free,
            prescribed: &p.prescribed,
            contributions: None,
            symmetry: None,
        };
        // A displacement vector with the prescribed values and the free RZ
        // rotation at zero (the moment row before any solve); the RZ row
        // (global 11) is coupled to g through two exact-negative couplings,
        // so its exact numerator is -m while today's binary64 fold absorbs
        // bits of m into the gross coupling (RV1-S2: the precondition below
        // makes this pin discriminate).
        let mut u = vec![0.0; 18];
        for &(c, v) in &p.prescribed {
            u[c] = v;
        }
        u[7] = g;
        let rows = evaluate_original_residual(&system, &u).unwrap();
        let row = rows.iter().find(|r| r.global_dof == 11).unwrap();
        // Today's expression, in the same order, with the row's own exponent.
        let e = row.row_scale_exponent;
        let mut r = -radix_scale(p.force[11], -e).unwrap();
        for (&k, &x) in p.k[11].iter().zip(&u) {
            if k != 0.0 {
                r = checked_value(r + normalized_product(k, x, e).unwrap()).unwrap();
            }
        }
        // Precondition (S11B-6): at this u the binary64 expression and the
        // correctly rounded exact numerator differ.
        let mut exact_numerator = ExactAccumulator::new();
        exact_numerator.add(-M).unwrap();
        for (&k, &x) in p.k[11].iter().zip(&u) {
            exact_numerator.add_product(k, x).unwrap();
        }
        assert_ne!(
            r.to_bits(),
            exact_numerator.round_scaled(-e).unwrap().to_bits(),
            "precondition g={g}"
        );
        assert_eq!(row.normalized_residual.to_bits(), r.to_bits());
        // The typed (exact KS3) evaluation of the same row is the exact sum.
        let mut ledger = LoadLedger::new();
        ledger.push("nodal:m", 11, M);
        let force = ledger.finish(18).unwrap();
        let typed = StructuralSystem::assembled(&p.k, &force, &p.free, &p.prescribed, None, None);
        let exact_rows = evaluate_assembled_original_residual(&typed, &u).unwrap();
        let exact = exact_rows.iter().find(|r| r.global_dof == 11).unwrap();
        let mut acc = ExactAccumulator::new();
        acc.add(-M).unwrap();
        for (&k, &x) in p.k[11].iter().zip(&u) {
            acc.add_product(k, x).unwrap();
        }
        assert_eq!(
            exact.normalized_residual.to_bits(),
            acc.round_scaled(-exact.row_scale_exponent)
                .unwrap()
                .to_bits()
        );
    }
}
