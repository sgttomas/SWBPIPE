//! A-1 (DEL-04-03 Work `skew-restraints.md` §15 step 6): the adapter's linear skew
//! entry on the references' guided cantilever (`product.spec`: member A–B along +X,
//! anchor at A, rotations held at B), in both solve modes: the M03 gate and K-D5 run on
//! the transformed system, the lifted u_B matches the exact value, dense = sparse, and
//! the nonlinear loop never carries a basis.
use super::*;
use open_pipe_stress_frame_kernel::load_ledger::LoadLedger;
use open_pipe_stress_frame_kernel::restraint_basis::{SkewDirection, SkewSpring, SkewStop};
use open_pipe_stress_frame_kernel::structural::SolveQuality;
use open_pipe_stress_frame_kernel::{FrameNode, FrameSection};

const REFERENCES: &str =
    include_str!("../../../../../validation/references/skew_restraints/references.json");

/// The binary64 value at `key` in the named product case, `index`-th entry of an
/// array of `{binary64, exact}` records (a tiny reader: the references are
/// generated JSON with sorted keys).
fn product_value(case: &str, key: &str, index: usize) -> f64 {
    let at = REFERENCES.find(&format!("\"name\": \"{case}\"")).unwrap_or_else(|| panic!("case {case}"));
    // A case's first key is `expect`; no nested object has one.
    let start = REFERENCES[..at].rfind("\"expect\": {").expect("case start");
    let body = &REFERENCES[start..at];
    let mut cursor = &body[body.find(&format!("\"{key}\"")).unwrap_or_else(|| panic!("{case}: {key}"))..];
    let mut value = 0.0;
    for _ in 0..=index {
        let b = cursor.find("\"binary64\": \"").expect("binary64 field") + "\"binary64\": \"".len();
        let e = cursor[b..].find('"').unwrap();
        value = hex(&cursor[b..b + e]);
        cursor = &cursor[b + e..];
    }
    value
}

fn hex(text: &str) -> f64 {
    let (negative, body) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text),
    };
    let body = body.strip_prefix("0x").expect("hex float");
    let (mantissa, exponent) = body.split_once('p').unwrap();
    let exponent: i32 = exponent.parse().unwrap();
    let (int, frac) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let mut m = u64::from_str_radix(int, 16).unwrap();
    for ch in frac.chars() {
        m = m * 16 + u64::from(ch.to_digit(16).unwrap());
    }
    let mut value = m as f64;
    let mut k = exponent - 4 * frac.len() as i32;
    while k != 0 {
        let step = k.clamp(-1000, 1000);
        value *= 2.0_f64.powi(step);
        k -= step;
    }
    if negative {
        -value
    } else {
        value
    }
}

struct Cantilever {
    frames: Vec<FrameElement>,
    basis: RestraintBasis,
    k: SparseStiffness,
    force: AssembledForce,
    prescribed: Vec<(usize, f64)>,
}

/// The guided cantilever with `stops` and `springs` (authored d, k) at B (node 1).
fn cantilever(held_b: &[usize], stops: &[[f64; 3]], springs: &[([f64; 3], f64)]) -> Cantilever {
    let leaves = |name: &str| {
        let at = REFERENCES.find(&format!("\"{name}\": \"")).unwrap() + name.len() + 5;
        hex(&REFERENCES[at..at + REFERENCES[at..].find('"').unwrap()])
    };
    let (e, g) = (171_798_691_840.0, 68_719_476_736.0);
    let (a, i, j) = (leaves("A_m2"), leaves("I_m4"), leaves("J_m4"));
    let section = FrameSection::new(e, g, a, i, i, j).unwrap();
    let frame = FrameElement::new(
        FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap(),
        FrameNode::new(1, [4.0, 0.0, 0.0]).unwrap(),
        section,
        [0.0, 0.0, 1.0],
    )
    .unwrap();
    let mut axis: Vec<usize> = (0..6).chain([9, 10, 11]).chain(held_b.iter().map(|&t| 6 + t)).collect();
    axis.sort_unstable();
    let basis = RestraintBasis::build(
        2,
        &axis,
        stops
            .iter()
            .enumerate()
            .map(|(n, &d)| SkewStop { support_id: format!("stop:{n}"), node: 1, direction: SkewDirection::new(d).unwrap() })
            .collect(),
        springs
            .iter()
            .enumerate()
            .map(|(n, &(d, k))| SkewSpring {
                support_id: format!("spring:{n}"),
                node: 1,
                direction: SkewDirection::new(d).unwrap(),
                stiffness: k,
            })
            .collect(),
    )
    .unwrap();
    let blocks = (0..basis.springs().len()).map(|s| (1, basis.spring_block(s))).collect();
    let k = assemble_sparse_stiffness(2, &[frame.clone()], &[], &[], &[], &SparseAssemblyOptions::new().with_node_blocks(blocks)).unwrap();
    let mut ledger = LoadLedger::new();
    ledger.push("load:B", 7, -1024.0);
    let force = ledger.finish(12).unwrap();
    let prescribed = basis.axis_restrained().iter().map(|&d| (d, 0.0)).collect();
    Cantilever { frames: vec![frame], basis, k, force, prescribed }
}

fn solve(model: &Cantilever, mode: LinearSolveMode) -> StructuralSolution {
    let evidence = SparseAssemblyEvidence::new_with_restraint_basis(
        model.k.pattern(),
        2,
        &model.frames,
        &[],
        &[],
        &[],
        &model.basis,
    )
    .unwrap();
    assert!(evidence.has_restraint_basis());
    evidence
        .solve_skew_with_formation_check(&model.k, &model.force, &model.prescribed, mode, &[])
        .unwrap_or_else(|e| panic!("{mode:?}: {e:?}"))
}

fn close(actual: f64, expect: f64, scale: f64, what: &str) {
    let tolerance = 1e-9 * if expect == 0.0 { scale } else { expect.abs() };
    assert!((actual - expect).abs() <= tolerance, "{what}: {actual:e} against {expect:e}");
}

#[test]
fn skew_cases_solve_on_the_transformed_system_in_both_modes() {
    let s = 1_048_576.0;
    let cases: [(&str, &[usize], &[[f64; 3]], &[([f64; 3], f64)]); 5] = [
        ("P_R1_stop", &[], &[[3.0, 4.0, 0.0]], &[]),
        ("P_R3_stop_plane_13", &[], &[[1.0, 3.0, 0.0]], &[]),
        ("P_R4_guide_plus_stop", &[2], &[[3.0, 4.0, 5.0]], &[]),
        ("P_S1_spring", &[], &[], &[([3.0, 4.0, 0.0], s)]),
        ("P_RS_stop_and_spring", &[], &[[3.0, 4.0, 0.0]], &[([0.0, 3.0, 4.0], s)]),
    ];
    for (name, held, stops, springs) in cases {
        let model = cantilever(held, stops, springs);
        let expect: Vec<f64> = (0..3).map(|k| product_value(name, "u_B_translations", k)).collect();
        let scale = expect.iter().fold(0.0_f64, |m, x| m.max(x.abs()));
        let mut outcomes = Vec::new();
        for mode in [LinearSolveMode::DenseScrutiny, LinearSolveMode::SparseInteractive] {
            let solution = solve(&model, mode);
            assert_eq!(solution.report.quality, SolveQuality::Passed, "{name} {mode:?}: {:?}", solution.formation_check);
            assert!(solution.formation_check.is_none(), "{name} {mode:?}: {:?}", solution.formation_check);
            let bound = solution.member_bound.clone().unwrap_or_else(|| panic!("{name} {mode:?}: K-D5 bound"));
            let lifted = model.basis.lift_member_bound(&solution.displacements, &bound).unwrap();
            for k in 0..3 {
                close(lifted.u[6 + k], expect[k], scale, &format!("{name} {mode:?}: u_B[{k}]"));
            }
            // The unrounded lift satisfies the authored constraint exactly.
            if model.basis.has_constraint_transform() {
                assert!(model.basis.constraint_residual(1, &solution.displacements).unwrap().is_zero(), "{name}");
            }
            outcomes.push((solution.displacements.clone(), solution.report.quality, solution.formation_check.is_none()));
        }
        // Dense = sparse: both modes carry the basis to K-D5 and reach the same
        // standing; their factors differ in order (as today's), so u agrees within
        // the reference tolerance rather than bit for bit.
        assert_eq!((outcomes[0].1, outcomes[0].2), (outcomes[1].1, outcomes[1].2), "{name}: dense = sparse standing");
        for (a, b) in outcomes[0].0.iter().zip(&outcomes[1].0) {
            close(*a, *b, scale, &format!("{name}: dense = sparse u"));
        }
    }
}

#[test]
fn the_loop_never_carries_a_basis() {
    // Only `new_with_restraint_basis` sets the basis, and only the linear skew entry
    // reads it; every other constructor carries none.
    let model = cantilever(&[], &[[3.0, 4.0, 0.0]], &[]);
    let plain = SparseAssemblyEvidence::new(model.k.pattern(), 2, &model.frames, &[], &[], &[]).unwrap();
    assert!(!plain.has_restraint_basis());
    assert!(matches!(
        plain.solve_skew_with_formation_check(&model.k, &model.force, &model.prescribed, LinearSolveMode::SparseInteractive, &[]),
        Err(StructuralError::InvalidInput("no restraint basis"))
    ));
    // Structurally: the loop, `complete_iteration` and the confirmation never name the
    // skew constructor or entry.
    for (name, source) in [("lib.rs", include_str!("../lib.rs")), ("confirmation.rs", include_str!("../confirmation.rs"))] {
        for token in ["new_with_restraint_basis", "solve_skew_with_formation_check", "restraint_basis", "SkewFormation"] {
            assert!(!source.contains(token), "{name} names {token}");
        }
    }
}
