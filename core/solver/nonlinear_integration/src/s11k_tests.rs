//! S11-K tests for this crate (S11 section 9, K7 and K12, and the T3 manager's
//! conformance terms for the typed nonlinear siblings). The crate is serialized
//! with T5: only the typed signatures, the bit-for-bit force check and the
//! adapter's audit seam are added; the loop is unchanged.
use crate::structural_adapter::AssemblyEvidence;
use crate::*;
use open_pipe_stress_frame_kernel::load_ledger::{ForceTerm, ForceTermKind, LoadLedger};
use open_pipe_stress_frame_kernel::structural::SolveQuality;
use open_pipe_stress_frame_kernel::{assemble_global_stiffness, FrameDof, FrameNode, FrameSection};
use open_pipe_stress_nonlinear_supports::{
    ActiveSetState, GapDirection, NonlinearSupport, SupportStateRecord,
};

fn adjacent_input() -> NonlinearFrameSolveInput {
    let section = FrameSection::new(100.0, 40.0, 1.0, 1.0, 1.0, 1.0).unwrap();
    let elements = (0..2)
        .map(|i| {
            FrameElement::new(
                FrameNode::new(i, [i as f64, 0.0, 0.0]).unwrap(),
                FrameNode::new(i + 1, [(i + 1) as f64, 0.0, 0.0]).unwrap(),
                section,
                [0.0, 1.0, 0.0],
            )
            .unwrap()
        })
        .collect();
    let mut force = vec![0.0; 18];
    force[12] = 12.5;
    NonlinearFrameSolveInput {
        node_count: 3,
        elements,
        user_stiffness_elements: vec![],
        curved_bend_elements: vec![],
        force,
        base_restrained_dofs: (0..18).filter(|&i| i != 6 && i != 12).collect(),
        nonlinear_supports: vec![
            NonlinearSupport::gap(
                "g1",
                1,
                FrameDof::Ux,
                f64::from_bits(0.125_f64.to_bits() - 1),
                GapDirection::PositiveDisplacement,
            )
            .unwrap(),
            NonlinearSupport::gap(
                "g2",
                2,
                FrameDof::Ux,
                0.25,
                GapDirection::PositiveDisplacement,
            )
            .unwrap(),
        ],
        initial_states: vec![
            SupportStateRecord::new("g1", ActiveSetState::Active),
            SupportStateRecord::new("g2", ActiveSetState::Inactive),
        ],
        friction_normal_reactions: vec![],
        derived_friction_normal_reactions: vec![],
        convergence: ConvergenceControl::new(
            "DEC-046-fixture-active-set-count-tightening",
            ConvergencePolicyStatus::Accepted,
            0.0,
            0.0,
            4,
        )
        .unwrap(),
    }
}

fn ledger_force(values: &[f64]) -> open_pipe_stress_frame_kernel::load_ledger::AssembledForce {
    let mut ledger = LoadLedger::new();
    for (dof, &v) in values.iter().enumerate() {
        if v != 0.0 {
            ledger.push(format!("load:{dof}"), dof, v);
        }
    }
    ledger.finish(values.len()).unwrap()
}

type Sibling = fn(
    &NonlinearFrameSolveInput,
    &open_pipe_stress_frame_kernel::load_ledger::AssembledForce,
) -> Result<NonlinearFrameSolveResult, NonlinearIntegrationError>;
type Untyped =
    fn(&NonlinearFrameSolveInput) -> Result<NonlinearFrameSolveResult, NonlinearIntegrationError>;

fn siblings() -> Vec<(&'static str, Sibling, Untyped)> {
    vec![
        (
            "solve_active_set_frame",
            |i, f| solve_active_set_frame_assembled(i, f),
            |i| solve_active_set_frame(i),
        ),
        (
            "solve_active_set_frame_with_mode",
            |i, f| {
                solve_active_set_frame_with_mode_assembled(i, f, LinearSolveMode::SparseInteractive)
            },
            |i| solve_active_set_frame_with_mode(i, LinearSolveMode::SparseInteractive),
        ),
        (
            "solve_active_set_frame_with_mode_and_springs",
            |i, f| {
                solve_active_set_frame_with_mode_and_springs_assembled(
                    i,
                    f,
                    LinearSolveMode::DenseScrutiny,
                    &[(7, 5.0)],
                )
            },
            |i| {
                solve_active_set_frame_with_mode_and_springs(
                    i,
                    LinearSolveMode::DenseScrutiny,
                    &[(7, 5.0)],
                )
            },
        ),
    ]
}

#[test]
fn k12_typed_nonlinear_siblings_delegate_on_bit_equal_force() {
    let input = adjacent_input();
    let force = ledger_force(&input.force);
    for (name, typed, untyped) in siblings() {
        assert_eq!(typed(&input, &force), untyped(&input), "{name}");
    }
}

#[test]
fn k12_typed_nonlinear_siblings_refuse_a_one_ulp_difference_in_any_entry() {
    let input = adjacent_input();
    for dof in [0, 6, 12, 17] {
        let mut shifted = input.force.clone();
        shifted[dof] = if shifted[dof] == 0.0 {
            f64::from_bits(1)
        } else {
            f64::from_bits(shifted[dof].to_bits() + 1)
        };
        let force = ledger_force(&shifted);
        for (name, typed, _) in siblings() {
            assert!(
                matches!(
                    typed(&input, &force),
                    Err(NonlinearIntegrationError::InvalidInput { .. })
                ),
                "{name} dof {dof}"
            );
        }
    }
    // A different length is refused too.
    let force = ledger_force(&input.force[..17]);
    for (name, typed, _) in siblings() {
        assert!(
            matches!(
                typed(&input, &force),
                Err(NonlinearIntegrationError::InvalidInput { .. })
            ),
            "{name}"
        );
    }
}

#[test]
fn k12_typed_nonlinear_siblings_treat_negative_zero_as_different_bits() {
    // The ledger's zero is +0.0; an input carrying -0.0 differs bit for bit.
    let mut input = adjacent_input();
    input.force[3] = -0.0;
    let force = ledger_force(&input.force);
    assert_eq!(force.values()[3].to_bits(), 0);
    for (name, typed, _) in siblings() {
        assert!(
            matches!(
                typed(&input, &force),
                Err(NonlinearIntegrationError::InvalidInput { .. })
            ),
            "{name}"
        );
    }
}

// ------------------------------------------------------------- adapter

fn cantilever_evidence() -> (AssemblyEvidence, Vec<Vec<f64>>) {
    let section = FrameSection::new(2.0e11, 7.7e10, 0.01, 8.0e-6, 9.0e-6, 1.7e-5).unwrap();
    let element = FrameElement::new(
        FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap(),
        FrameNode::new(1, [2.0, 0.0, 0.0]).unwrap(),
        section,
        [0.0, 1.0, 0.0],
    )
    .unwrap();
    let k = assemble_global_stiffness(2, &[element.clone()]).unwrap();
    (
        AssemblyEvidence::new(2, &[element], &[], &[], &[]).unwrap(),
        k,
    )
}

fn cancelling_tip_terms(order: [f64; 3]) -> Vec<ForceTerm> {
    order
        .iter()
        .enumerate()
        .map(|(i, &v)| ForceTerm {
            source: format!("load:{i}"),
            dof: 7,
            kind: ForceTermKind::Term(v),
        })
        .collect()
}

#[test]
fn k7_adapter_with_force_terms_flags_the_folded_force_in_both_modes() {
    let (evidence, k) = cantilever_evidence();
    let free: Vec<usize> = (6..12).collect();
    let fixed: Vec<(usize, f64)> = (0..6).map(|d| (d, 0.0)).collect();
    for (g, n) in [(1e8, 0.3), (1e80, 1e-8)] {
        let terms = cancelling_tip_terms([g, n, -g]);
        let mut folded = vec![0.0; 12];
        folded[7] = g + n - g;
        assert_ne!(folded[7], n, "precondition");
        let audited = evidence.clone().with_force_terms(&terms);
        for mode in [
            LinearSolveMode::DenseScrutiny,
            LinearSolveMode::SparseInteractive,
        ] {
            let legacy = evidence.solve(&k, &folded, &free, &fixed, mode).unwrap();
            let detect = audited.solve(&k, &folded, &free, &fixed, mode).unwrap();
            assert_eq!(legacy.load_fidelity, None);
            assert_eq!(detect.displacements, legacy.displacements);
            let report = detect.load_fidelity.expect("flagged");
            assert!(report.rows.iter().any(|r| r.global_dof == 7));
            assert_eq!(detect.report.quality, SolveQuality::Sensitive);
            // The typed route solves the exact net and passes.
            let mut ledger = LoadLedger::new();
            for t in &terms {
                if let ForceTermKind::Term(v) = t.kind {
                    ledger.push(t.source.clone(), t.dof, v);
                }
            }
            let force = ledger.finish(12).unwrap();
            assert_eq!(force.values()[7], n);
            let typed = evidence
                .solve_assembled(&k, &force, &free, &fixed, mode)
                .unwrap();
            assert!(typed.load_fidelity.is_none());
            assert_eq!(typed.report.quality, SolveQuality::Passed);
            let mut net = vec![0.0; 12];
            net[7] = n;
            let reference = evidence.solve(&k, &net, &free, &fixed, mode).unwrap();
            assert_eq!(typed.displacements, reference.displacements);
            assert_eq!(
                format!("{:?}", typed.report),
                format!("{:?}", reference.report)
            );
        }
    }
}

// ------------------------------------ ROOT option (c): the loop's legacy pin

/// The nonlinear active-set loop's closed-gap prescribed solves go through
/// `solve_linearized_system_evidence` (the loop, the influence solves and the
/// unit-force solves). ROOT's option (c) keeps them on the named, unchanged
/// binary64 kernel path; this pin fails if a refactor switches any of the four
/// call targets to an exact KS1-KS3 entry point.
#[test]
fn option_c_nonlinear_loop_is_pinned_to_the_binary64_kernel_path() {
    let src = include_str!("lib.rs");
    let start = src
        .find("fn solve_linearized_system_evidence(")
        .expect("function present");
    let body_start = start + src[start..].find('{').unwrap();
    let mut depth = 0usize;
    let mut end = body_start;
    for (k, ch) in src[body_start..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    end = body_start + k;
                    break;
                }
            }
            _ => {}
        }
    }
    let body = &src[body_start..end];
    for required in [
        "product_equilibrium::evaluate(",
        "reduce_system_with_prescribed_displacements_binary64(",
        "assembly.solve_binary64(",
        "structural::solve_structural_dense_binary64(",
        "structural_adapter::solve_structural_sparse_binary64(",
    ] {
        assert!(
            body.contains(required),
            "missing legacy call target {required}"
        );
    }
    let stripped = body.replace("_binary64(", "#(");
    for forbidden in [
        "reduce_system_with_prescribed_displacements(",
        "assembly.solve(",
        "assembly.solve_assembled(",
        "solve_structural_dense(",
        "solve_structural_sparse(",
        "prepare_structural(",
    ] {
        assert!(
            !stripped.contains(forbidden),
            "exact kernel entry point {forbidden} in the nonlinear loop"
        );
    }
}

/// ROOT on V1 R5-3: `product_equilibrium::evaluate` (the loop's final
/// equilibrium and the gap scrutiny) measures with the public, binary64
/// `structural::evaluate_original_residual`; FK pins that function's bytes
/// (`option_c_public_original_residual_stays_binary64_on_coupled_rows`).
#[test]
fn option_c_product_equilibrium_uses_the_binary64_residual() {
    let src = include_str!("product_equilibrium.rs");
    assert!(src.contains("structural::evaluate_original_residual(system, u)"));
    assert!(!src.contains("evaluate_assembled_original_residual"));
    let adapter = include_str!("structural_adapter.rs");
    assert!(adapter.contains("crate::product_equilibrium::evaluate(&system, &u)"));
}
