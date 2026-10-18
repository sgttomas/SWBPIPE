//! T4-U5b K-D5 at kernel level (DEL-04-07 Design `u4b-u5b-implementation.md`
//! §3.2 and §7 item 6): the tie rod's p re-formation and rigid null space,
//! the `TieRodReference` family against an exact reference, its closure
//! mutants on the ordinary and W2 routes, a formation-checked rod case, the
//! certified rod recovery against an exact reference, and support reactions
//! with rods. Invented inputs only.
use super::super::{
    certify_support_reaction, certify_tie_rod_recovery, CertifiedMemberLoads, SolveQuality,
    StiffnessContribution, StructuralSystem,
};
use super::*;
use crate::connector::{ConnectorAttachment, ScaledWorkMatrix};
use crate::load_ledger::{AssembledForce, Formation, LoadLedger};
use crate::tie_rod::{TieRodInput, TIE_ROD_REFERENCE_FAMILY};
use crate::{FrameNode, Matrix3};

const THIRD: f64 = 1.0 / 3.0;
const TWO_THIRDS: f64 = 2.0 / 3.0;
/// A skew proper triad with Q.x = (1, 2, 2)/3 (non-dyadic entries).
const SKEW: Matrix3 = [
    [THIRD, TWO_THIRDS, -TWO_THIRDS],
    [TWO_THIRDS, THIRD, TWO_THIRDS],
    [TWO_THIRDS, -TWO_THIRDS, -THIRD],
];
const IDENTITY: Matrix3 = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

fn round(w: &Wide2) -> f64 {
    let mut acc = ExactAccumulator::new();
    for &t in w.split_binary64().unwrap().terms() {
        acc.add(t).unwrap();
    }
    acc.round().unwrap()
}

fn rod(
    offset_i: [f64; 3],
    offset_j: [f64; 3],
    stiffness: f64,
    reference_extension: f64,
) -> TieRodInput {
    TieRodInput {
        offset_i,
        offset_j,
        stiffness,
        reference_extension,
    }
}

fn connector(
    xi: [f64; 3],
    xj: [f64; 3],
    axes: Matrix3,
    rods: &[TieRodInput],
) -> ObjectiveConnector {
    let mut upper = [0.0; 21];
    let diagonal = [5e6, 4e6, 3e6, 2e3, 1e3, 1.5e3];
    let mut at = 0;
    for r in 0..6 {
        for c in r..6 {
            if r == c {
                upper[at] = diagonal[r];
            }
            at += 1;
        }
    }
    ObjectiveConnector::new(
        FrameNode::new(0, xi).unwrap(),
        FrameNode::new(1, xj).unwrap(),
        ConnectorAttachment::global([0.0; 3]),
        ConnectorAttachment::global([0.0; 3]),
        axes,
        ScaledWorkMatrix {
            upper_triangle: upper,
            translation_scale: 1.0,
        },
        [0.0; 6],
    )
    .unwrap()
    .with_tie_rods(rods)
    .unwrap()
}

/// Dyadic node coordinates, x_j − x_i = 0.75·Q.x exactly, three generic
/// rods (one with e_ref = 0, which has no reference summand).
fn skew_case() -> ObjectiveConnector {
    connector(
        [0.25, -0.5, 1.0],
        [0.5, 0.0, 1.5],
        SKEW,
        &[
            rod([-0.05, 0.13, -0.07], [0.06, -0.11, 0.09], 2.5e7, -1.25e-3),
            rod([0.0, -0.09, 0.08], [0.11, 0.1, -0.03], 7.0e6, 0.0),
            rod([0.21, 0.0, 0.0], [-0.17, 0.02, 0.0], 1.0e5, 4.0e-4),
        ],
    )
}

/// n = (2, 3, 6)/7 with Q = I (`tests/tie_rod_element.rs`): 7·C = N.
fn pythagorean() -> (ObjectiveConnector, [f64; 12]) {
    let (oi, oj) = ([0.125, -0.25, -0.5], [-0.375, 0.5, 1.0]);
    let c = connector(
        [0.0; 3],
        [1.0, 0.0, 0.0],
        IDENTITY,
        &[rod(oi, oj, 3.0e6, -1.0e-3)],
    );
    let n7 = [2.0, 3.0, 6.0];
    let cross = |o: [f64; 3]| {
        [
            o[1] * n7[2] - o[2] * n7[1],
            o[2] * n7[0] - o[0] * n7[2],
            o[0] * n7[1] - o[1] * n7[0],
        ]
    };
    let (ai, aj) = (cross(oi), cross(oj));
    let mut numerator = [0.0; 12];
    for k in 0..3 {
        numerator[k] = -n7[k];
        numerator[3 + k] = -ai[k];
        numerator[6 + k] = n7[k];
        numerator[9 + k] = aj[k];
    }
    (c, numerator)
}

fn add_triple(sum: &mut ExactAccumulator, a: f64, b: f64, c: f64) {
    let p = b * c;
    let e = b.mul_add(c, -p);
    sum.add_product(a, p).unwrap();
    sum.add_product(a, e).unwrap();
}

/// Whether |m·scale − exact| ≤ r·scale, decided exactly (`exact` holds
/// −exact already).
fn contains(m: &Wide2, r: f64, scale: f64, mut minus_exact: ExactAccumulator) -> bool {
    for &t in m.split_binary64().unwrap().terms() {
        minus_exact.add_product(scale, t).unwrap();
    }
    let mut low = minus_exact.clone();
    low.add_product(-scale, r).unwrap();
    minus_exact.add_product(scale, r).unwrap();
    low.signum() <= 0 && minus_exact.signum() >= 0
}

fn reference(
    connector: &ObjectiveConnector,
    rod: usize,
    row: usize,
    source: &str,
) -> LoadPrimitive {
    LoadPrimitive {
        term_index: 0,
        identity: LoadSummandIdentity {
            family: TIE_ROD_REFERENCE_FAMILY.into(),
            source: source.into(),
            ordinal: row,
        },
        local_row: row,
        formula: LoadFormula::TieRodReference {
            connector: 0,
            rod,
            tie_rod: connector.tie_rods()[rod],
        },
        force_scale_exponent: 0,
    }
}

#[test]
fn tie_rod_reformation_agrees_with_binary64_and_keeps_the_rigid_null_space() {
    let c = skew_case();
    let mut arith = WideArith::new(FORMATION_PRECISION).unwrap();
    let origin = [7.0, -3.0, 5.0];
    for rod in c.tie_rods() {
        let wide = tie_rod_matrix(&mut arith, rod).unwrap();
        let binary64 = rod.global_stiffness().unwrap();
        let scale = binary64
            .iter()
            .flatten()
            .fold(0.0_f64, |m, v| m.max(v.abs()));
        for r in 0..12 {
            for s in 0..12 {
                assert!(
                    (round(&wide[r][s]) - binary64[r][s]).abs() <= 1e-13 * scale,
                    "[{r}][{s}]"
                );
            }
        }
        // Rigid modes with dyadic coordinates, so d is exact: K_int·d ≈ 0 at p.
        let (xi, xj) = (rod.node_i().coordinates, rod.node_j().coordinates);
        for mode in 0..6 {
            let mut d = [0.0; 12];
            if mode < 3 {
                d[mode] = 1.0;
                d[6 + mode] = 1.0;
            } else {
                let k = mode - 3;
                for (block, x) in [(0, xi), (6, xj)] {
                    let v = [x[0] - origin[0], x[1] - origin[1], x[2] - origin[2]];
                    let u = match k {
                        0 => [0.0, -v[2], v[1]],
                        1 => [v[2], 0.0, -v[0]],
                        _ => [-v[1], v[0], 0.0],
                    };
                    d[block..block + 3].copy_from_slice(&u);
                    d[block + 3 + k] = 1.0;
                }
            }
            for r in 0..12 {
                let mut sum = Wide2::ZERO;
                for s in 0..12 {
                    let t = arith.mul(&wide[r][s], &lift(d[s]).unwrap()).unwrap();
                    sum = arith.add(&sum, &t).unwrap();
                }
                assert!(
                    round(&sum).abs() <= 1e-28 * scale * 16.0,
                    "mode {mode} row {r}: {:e}",
                    round(&sum)
                );
            }
        }
    }
}

#[test]
fn tie_rod_reference_reformation_encloses_the_exact_rational_reference_at_p() {
    let (c, n7) = pythagorean();
    let r0 = &c.tie_rods()[0];
    let (k, e_ref) = (r0.stiffness(), r0.reference_extension());
    let binary64 = r0.reference_load().unwrap().unwrap();
    for exponent in [0, -6, 6] {
        for row in 0..12 {
            let mut p = reference(&c, 0, row, "C1:rodA");
            p.force_scale_exponent = exponent;
            let (m, r) = load_reformation(&p).unwrap();
            let mut minus_exact = ExactAccumulator::new();
            add_triple(&mut minus_exact, -k * 2f64.powi(exponent), e_ref, n7[row]);
            assert!(
                contains(&m[row], r[row], 7.0, minus_exact),
                "b={exponent} row {row}"
            );
            let magnitude = (k * e_ref).abs() * 2f64.powi(exponent);
            assert!(r[row] <= 1e-30 * magnitude, "radius {:e}", r[row]);
            let rounded = round(&m[row]) * 2f64.powi(-exponent);
            assert!(
                (rounded - binary64.values[row]).abs()
                    <= 8.0 * f64::EPSILON * magnitude * 2f64.powi(-exponent)
            );
        }
    }
}

/// The rods of `skew_case` with e_ref ≠ 0 (0 and 2) as one formed term per
/// DOF, each with a manifest of both rods' summands (rod 1, e_ref = 0, is the
/// source-known zero and has none), plus input nodal loads at node 1.
fn rod_ledger(c: &ObjectiveConnector) -> AssembledForce {
    let mut ledger = LoadLedger::new();
    let loads = c
        .tie_rods()
        .iter()
        .map(|r| r.reference_load().unwrap())
        .collect::<Vec<_>>();
    assert!(loads[1].is_none(), "e_ref = 0: no reference summand");
    let ids = ["C1:rodA", "C1:rodB", "C1:rodC"];
    let dofs = element_dof_map(c.node_i().index, c.node_j().index);
    for row in 0..12 {
        let mut value = 0.0;
        let mut bound = 0.0_f64;
        let mut expected = Vec::new();
        let mut supplied = Vec::new();
        for (index, load) in loads.iter().enumerate() {
            let Some(load) = load else { continue };
            value += load.values[row];
            bound = (bound + load.bounds[row]).next_up();
            let mut p = reference(c, index, row, ids[index]);
            p.term_index = row;
            expected.push(p.closure_entry());
            supplied.push(p);
        }
        ledger.push_formed(
            "tie_rod_reference:C1",
            dofs[row],
            value,
            Formation::Bounded { bound },
            0.0,
            true,
        );
        assert!(ledger.set_closure_manifest(row, expected));
        for p in supplied {
            ledger.attach_load_primitive(p);
        }
    }
    for (local, value) in [1000.0, -2000.0, 1500.0, 30.0, -20.0, 10.0]
        .into_iter()
        .enumerate()
    {
        ledger.push("load:N1", 6 + local, value);
    }
    ledger.finish(12).unwrap()
}

fn closure_detail(source: &FormationSource, force: &AssembledForce) -> String {
    match load_closure(source, force) {
        Err(Failure::Closure(detail)) => detail,
        other => panic!("expected a closure failure, got {other:?}"),
    }
}

#[test]
fn tie_rod_reference_closure_kills_each_mutant_by_name_on_ordinary_and_w2() {
    let c = skew_case();
    let force = rod_ledger(&c);
    for exponent in [0, -6, 6] {
        let scale = crate::ForceScale { exponent };
        let scaled = force.force_scaled(scale).unwrap();
        let held = c.force_scaled(scale).unwrap();
        let source = FormationSource {
            node_count: 2,
            loads: scaled.load_primitives().into(),
            connectors: vec![held.clone()],
            ..FormationSource::default()
        };
        load_closure(&source, &scaled).unwrap();
        let row = 4; // a DOF with two summands
        let at = |s: &FormationSource, rod: usize| {
            s.loads
                .iter()
                .position(|p| p.term_index == row && matches!(p.formula, LoadFormula::TieRodReference { rod: r, .. } if r == rod))
                .unwrap()
        };
        let mut mutants: Vec<(&str, FormationSource, &str)> = Vec::new();
        // Drop one of the two summands of a term.
        let mut m = source.clone();
        let i = at(&m, 2);
        m.loads.to_mut().remove(i);
        mutants.push(("drop", m, "missing, duplicate or extra summand"));
        // Duplicate: rod C's summand replaced by a copy of rod A's.
        let mut m = source.clone();
        let (a, b) = (at(&m, 0), at(&m, 2));
        let copy = m.loads[a].clone();
        m.loads.to_mut()[b] = copy;
        mutants.push(("duplicate", m, "duplicate summand"));
        // Extra summand.
        let mut m = source.clone();
        let extra = m.loads[at(&m, 0)].clone();
        m.loads.to_mut().push(extra);
        mutants.push(("extra", m, "missing, duplicate or extra summand"));
        let set =
            |m: &mut FormationSource,
             i: usize,
             f: &dyn Fn(&mut usize, &mut usize, &mut crate::tie_rod::TieRod)| {
                if let LoadFormula::TieRodReference {
                    connector,
                    rod,
                    tie_rod,
                } = &mut m.loads.to_mut()[i].formula
                {
                    f(connector, rod, tie_rod);
                }
            };
        // Wrong rod binding: rod A's summand pointing at source rod B.
        let mut m = source.clone();
        let i = at(&m, 0);
        set(&mut m, i, &|_, rod, _| *rod = 1);
        mutants.push(("wrong rod index", m, "tie-rod source binding"));
        // Rod A's identity re-formed from rod B (e_ref = 0), consistently bound.
        let mut m = source.clone();
        let i = at(&m, 0);
        set(&mut m, i, &|_, rod, tie_rod| {
            *rod = 1;
            *tie_rod = c.tie_rods()[1];
        });
        mutants.push(("wrong rod", m, "wrong operand binding"));
        // A rod index the connector does not have.
        let mut m = source.clone();
        let i = at(&m, 0);
        set(&mut m, i, &|_, rod, _| *rod = 7);
        mutants.push(("absent rod", m, "tie-rod source binding"));
        // Wrong connector binding: another source connector on the same nodes
        // (other K), carrying the same rods.
        let other = {
            let mut upper = [0.0; 21];
            for (at, v) in [
                (0, 9e6),
                (6, 4e6),
                (11, 3e6),
                (15, 2e3),
                (18, 1e3),
                (20, 1.5e3),
            ] {
                upper[at] = v;
            }
            let rods: Vec<_> = c
                .tie_rods()
                .iter()
                .map(|r| {
                    let (i, j) = r.local_offsets();
                    rod(i, j, r.stiffness(), r.reference_extension())
                })
                .collect();
            ObjectiveConnector::new(
                c.node_i(),
                c.node_j(),
                ConnectorAttachment::global([0.0; 3]),
                ConnectorAttachment::global([0.0; 3]),
                SKEW,
                ScaledWorkMatrix {
                    upper_triangle: upper,
                    translation_scale: 1.0,
                },
                [0.0; 6],
            )
            .unwrap()
            .with_tie_rods(&rods)
            .unwrap()
        };
        let mut m = source.clone();
        m.connectors.push(other.force_scaled(scale).unwrap());
        let i = at(&m, 0);
        set(&mut m, i, &|connector, _, _| *connector = 1);
        mutants.push(("wrong connector", m, "wrong operand binding"));
        // A connector index the source does not have.
        let mut m = source.clone();
        let i = at(&m, 0);
        set(&mut m, i, &|connector, _, _| *connector = 5);
        mutants.push(("absent connector", m, "tie-rod source binding"));
        // Wrong operands: rod A's e_ref changed in the primitive only, then
        // also in the source (a manifest mismatch).
        let changed = {
            let rods: Vec<_> = c
                .tie_rods()
                .iter()
                .enumerate()
                .map(|(index, r)| {
                    let (i, j) = r.local_offsets();
                    let e = if index == 0 {
                        2.0 * r.reference_extension()
                    } else {
                        r.reference_extension()
                    };
                    rod(i, j, r.stiffness(), e)
                })
                .collect();
            c.clone().with_tie_rods(&rods).unwrap()
        };
        let mut m = source.clone();
        let i = at(&m, 0);
        set(&mut m, i, &|_, _, tie_rod| *tie_rod = changed.tie_rods()[0]);
        mutants.push(("wrong operands", m, "tie-rod source binding"));
        let mut m = source.clone();
        m.connectors = vec![changed.force_scaled(scale).unwrap()];
        for i in 0..m.loads.len() {
            if matches!(
                m.loads[i].formula,
                LoadFormula::TieRodReference { rod: 0, .. }
            ) {
                set(&mut m, i, &|_, _, tie_rod| *tie_rod = changed.tie_rods()[0]);
            }
        }
        mutants.push(("wrong operands (in source)", m, "wrong operand binding"));
        // Wrong component of the member vector.
        let mut m = source.clone();
        let i = at(&m, 0);
        m.loads.to_mut()[i].local_row = 5;
        mutants.push(("wrong row", m, "term DOF binding"));
        // Wrong force exponent.
        let mut m = source.clone();
        let i = at(&m, 0);
        m.loads.to_mut()[i].force_scale_exponent += 2;
        mutants.push(("wrong exponent", m, "tie-rod source binding"));
        // The stiffness source without the reference's connector.
        let mut m = source.clone();
        m.connectors.clear();
        mutants.push(("no source connector", m, "tie-rod source binding"));
        for (name, mutant, detail) in mutants {
            let found = closure_detail(&mutant, &scaled);
            assert!(found.ends_with(detail), "b={exponent} {name}: {found}");
            assert!(
                found.starts_with("tie_rod_reference:C1"),
                "b={exponent} {name}: {found}"
            );
        }
        // An absent manifest fails closed on this route too.
        let mut ledger = LoadLedger::new();
        ledger.push_formed(
            "tie_rod_reference:C1",
            4,
            1.0,
            Formation::CannotBound,
            0.0,
            true,
        );
        let absent = ledger.finish(12).unwrap().force_scaled(scale).unwrap();
        let empty = FormationSource {
            node_count: 2,
            connectors: vec![held],
            ..FormationSource::default()
        };
        assert!(closure_detail(&empty, &absent).ends_with("absent manifest"));
    }
}

/// The assembled stiffness and its element contributions in slot order
/// (connector, then its rods).
fn assembled(c: &ObjectiveConnector) -> (Vec<Vec<f64>>, Vec<StiffnessContribution>) {
    let k =
        crate::assemble_global_stiffness_with_connectors(2, &[], std::slice::from_ref(c)).unwrap();
    let map = element_dof_map(0, 1);
    let mut contributions = Vec::new();
    let blocks = std::iter::once(c.global_stiffness().unwrap())
        .chain(c.tie_rods().iter().map(|r| r.global_stiffness().unwrap()));
    for block in blocks {
        for r in 0..12 {
            for s in 0..12 {
                contributions.push(StiffnessContribution {
                    row: map[r],
                    col: map[s],
                    value: block[r][s],
                });
            }
        }
    }
    (k, contributions)
}

#[test]
fn a_rod_case_passes_kd5_on_both_routes_and_rods_missing_from_the_source_fail_closed() {
    let physical = skew_case();
    let force = rod_ledger(&physical);
    let free: Vec<usize> = (6..12).collect();
    let prescribed: Vec<(usize, f64)> = (0..6).map(|d| (d, 0.0)).collect();
    let mut tensions = Vec::new();
    for exponent in [0, 6, -6] {
        let scale = crate::ForceScale { exponent };
        let held = physical.force_scaled(scale).unwrap();
        let force = force.force_scaled(scale).unwrap();
        let (k, contributions) = assembled(&held);
        let source = FormationSource {
            node_count: 2,
            loads: force.load_primitives().into(),
            connectors: vec![held.clone()],
            ..FormationSource::default()
        };
        let checked =
            StructuralSystem::assembled(&k, &force, &free, &prescribed, Some(&contributions), None)
                .with_formation_source(&source);
        let solution = super::super::solve_formation_checked_structural_dense(&checked).unwrap();
        assert_eq!(
            solution.report.quality,
            SolveQuality::Passed,
            "b={exponent}"
        );
        assert_eq!(solution.formation_check, None, "b={exponent}");
        let bound = solution.member_bound.as_ref().unwrap();
        // The certified rows use the physical connector on every route.
        let mut row = Vec::new();
        for (index, r) in physical.tie_rods().iter().enumerate() {
            let certified =
                certify_tie_rod_recovery(&physical, index, &solution.displacements, bound).unwrap();
            let d = r.element_displacements(&solution.displacements).unwrap();
            let binary64 = r.recover(&d).unwrap().tension;
            let t = certified.tension.published;
            assert!(
                (t - binary64).abs() <= 1e-9 * t.abs().max(1.0),
                "b={exponent} rod {index}: {t} {binary64}"
            );
            assert!(certified.tension.bound() <= 1e-9 * t.abs().max(1.0));
            assert!((certified.axial_share.published().unwrap() - r.axial_share()).abs() <= 1e-15);
            row.push(certified.tension);
        }
        tensions.push(row);
        // Mutant: the stiffness source omits the rods (the assembly has them).
        let untied = held.clone().with_tie_rods(&[]).unwrap();
        let mut bare = source.clone();
        bare.connectors = vec![untied];
        let checked =
            StructuralSystem::assembled(&k, &force, &free, &prescribed, Some(&contributions), None)
                .with_formation_source(&bare);
        let solution = super::super::solve_formation_checked_structural_dense(&checked).unwrap();
        assert_eq!(
            solution.report.quality,
            SolveQuality::Sensitive,
            "b={exponent}"
        );
        match solution.formation_check.map(|c| c.reason) {
            Some(FormationCheckReason::FormationCheckUnavailable { detail }) => {
                assert!(
                    detail.contains("tie-rod source binding"),
                    "b={exponent}: {detail}"
                )
            }
            other => panic!("b={exponent}: {other:?}"),
        }
    }
    // Without rod references, a stiffness source that omits the rods is
    // demoted: K_int holds each rod's own block, so the residual of the
    // assembled solution against the rod-less intended system is not small
    // (D5C-1 `Estimate`; ε_E's element walk would refuse it next).
    let mut ledger = LoadLedger::new();
    for (local, value) in [1000.0, -2000.0, 1500.0, 30.0, -20.0, 10.0]
        .into_iter()
        .enumerate()
    {
        ledger.push("load:N1", 6 + local, value);
    }
    let inputs = ledger.finish(12).unwrap();
    let (k, contributions) = assembled(&physical);
    for connectors in [
        vec![physical.clone()],
        vec![physical.clone().with_tie_rods(&[]).unwrap()],
    ] {
        let tied = !connectors[0].tie_rods().is_empty();
        let source = FormationSource {
            node_count: 2,
            connectors,
            ..FormationSource::default()
        };
        let checked = StructuralSystem::assembled(
            &k,
            &inputs,
            &free,
            &prescribed,
            Some(&contributions),
            None,
        )
        .with_formation_source(&source);
        let solution = super::super::solve_formation_checked_structural_dense(&checked).unwrap();
        if tied {
            assert_eq!(solution.report.quality, SolveQuality::Passed);
            assert_eq!(solution.formation_check, None);
        } else {
            assert_eq!(solution.report.quality, SolveQuality::Sensitive);
            let check = solution.formation_check.unwrap();
            assert_eq!(check.reason, FormationCheckReason::Estimate);
            assert!(check.ratio > 1e3, "ratio {}", check.ratio);
        }
    }
    // W2's physical rows agree with the ordinary rows within both bounds.
    for row in &tensions[1..] {
        for (w2, ordinary) in row.iter().zip(&tensions[0]) {
            assert!(
                (w2.published - ordinary.published).abs() <= w2.bound() + ordinary.bound() + 1e-300
            );
        }
    }
}

#[test]
fn certified_tie_rod_recovery_contains_the_exact_reference() {
    let (c, n7) = pythagorean();
    let r0 = &c.tie_rods()[0];
    let (k, e_ref) = (r0.stiffness(), r0.reference_extension());
    let u = [
        1.0e-4, -2.5e-4, 5.0e-4, 1.0e-3, -2.0e-3, 0.5e-3, 3.0e-4, 1.25e-4, -7.5e-4, -1.0e-3,
        2.5e-4, 1.5e-3,
    ];
    let w: Vec<f64> = (0..12).map(|i| 2f64.powi(-40) * (i as f64 - 5.5)).collect();
    let w2: Vec<f64> = (0..12).map(|i| 2f64.powi(-70) * (3.0 - i as f64)).collect();
    for lambda_sigma in [0.0, 2f64.powi(-40)] {
        let bound = MemberBoundInputs {
            w: w.clone(),
            w2: w2.clone(),
            exponents: vec![0; 12],
            lambda_sigma,
        };
        let certified = certify_tie_rod_recovery(&c, 0, &u, &bound).unwrap();
        // 7·T = k(N·(u + w + w2)) − 7 k e_ref, exactly; likewise 7·e.
        let exact = |with_reference: bool, tension: bool| {
            let mut sum = ExactAccumulator::new();
            for l in 0..12 {
                for x in [u[l], w[l], w2[l]] {
                    if tension {
                        add_triple(&mut sum, k, n7[l], x);
                    } else {
                        sum.add_product(n7[l], x).unwrap();
                    }
                }
            }
            if with_reference {
                add_triple(&mut sum, -7.0, k, e_ref);
            }
            sum
        };
        let inside = |row: &crate::structural::CertifiedRow, mut exact: ExactAccumulator| {
            exact.add_product(-7.0, row.published).unwrap();
            let mut high = exact.clone();
            high.add_product(-7.0, row.bound()).unwrap();
            exact.add_product(7.0, row.bound()).unwrap();
            high.signum() <= 0 && exact.signum() >= 0
        };
        assert!(
            inside(&certified.tension, exact(true, true)),
            "λσ={lambda_sigma:e}: T outside its bound"
        );
        assert!(
            inside(&certified.extension, exact(false, false)),
            "e outside its bound"
        );
        // The bound discriminates: e_ref ignored lies outside it.
        assert!(
            !inside(&certified.tension, exact(false, true)),
            "e_ref-ignored reference inside the bound"
        );
        let magnitude = certified.tension.published.abs();
        if lambda_sigma == 0.0 {
            assert_eq!(certified.tension.remainder, 0.0);
            assert!(certified.tension.radius <= 1e-25 * magnitude);
            assert!(certified.tension.bound() <= f64::EPSILON * magnitude);
        } else {
            // ‖k C‖₁·Λσ: Σ|k N_a/7| = k·(2·11 + Σ|arms|)/7, upward.
            let norm: f64 = n7.iter().map(|v| k * v.abs() / 7.0).sum();
            assert!(certified.tension.remainder >= norm * lambda_sigma * (1.0 - 1e-12));
            assert!(certified.tension.remainder <= norm * lambda_sigma * (1.0 + 1e-12));
        }
        for (k, v) in [2.0, 3.0, 6.0].into_iter().enumerate() {
            assert!((certified.direction[k].published().unwrap() - v / 7.0).abs() <= f64::EPSILON);
        }
        assert!((certified.axial_share.published().unwrap() - 2.0 / 7.0).abs() <= f64::EPSILON);
    }
    let bound = MemberBoundInputs {
        w,
        w2,
        exponents: vec![0; 12],
        lambda_sigma: 0.0,
    };
    assert!(
        certify_tie_rod_recovery(&c, 1, &u, &bound).is_err(),
        "rod outside the connector"
    );
}

#[test]
fn support_reactions_include_the_rods_and_connector_member_loads_exclude_their_references() {
    let c = skew_case();
    let force = rod_ledger(&c);
    let u: Vec<f64> = (0..12)
        .map(|i| if i < 6 { 0.0 } else { 1e-4 * (i as f64 - 8.5) })
        .collect();
    let bound = MemberBoundInputs {
        w: vec![0.0; 12],
        w2: vec![0.0; 12],
        exponents: vec![i32::MIN; 12],
        lambda_sigma: 0.0,
    };
    let source = FormationSource {
        node_count: 2,
        loads: force.load_primitives().into(),
        connectors: vec![c.clone()],
        ..FormationSource::default()
    };
    let untied = FormationSource {
        connectors: vec![c.clone().with_tie_rods(&[]).unwrap()],
        ..source.clone()
    };
    for dof in 0..6 {
        let with = certify_support_reaction(&source, &force, dof, &u, &bound).unwrap();
        let without = certify_support_reaction(&untied, &force, dof, &u, &bound).unwrap();
        let mut rods = 0.0;
        let mut scale = 0.0_f64;
        for r in c.tie_rods() {
            let ke = r.global_stiffness().unwrap();
            for s in 0..12 {
                rods += ke[dof][s] * u[s];
                scale = scale.max((ke[dof][s] * u[s]).abs());
            }
        }
        let difference = with.published - without.published;
        assert!(
            (difference - rods).abs() <= 1e-9 * scale.max(1.0),
            "dof {dof}: {difference} {rods}"
        );
    }
    // The connector's certified member loads never take a rod reference.
    let loads =
        CertifiedMemberLoads::from_member_primitives(0, 1, force.load_primitives(), "").unwrap();
    let bound = MemberBoundInputs {
        w: vec![0.0; 12],
        w2: vec![0.0; 12],
        exponents: vec![0; 12],
        lambda_sigma: 0.0,
    };
    let with = super::super::certify_connector_recovery(&c, &loads, &u, &bound).unwrap();
    let plain =
        super::super::certify_connector_recovery(&c, &CertifiedMemberLoads::default(), &u, &bound)
            .unwrap();
    for (a, b) in with.ends.iter().zip(&plain.ends) {
        assert_eq!(a.published.to_bits(), b.published.to_bits());
    }
}

/// §3.3's certified S_joint = g_x + Σ T_r·(n_r·Q_x) − Pe from certified rows:
/// `add_row` and `product_value` keep the true value inside the bound.
#[test]
fn certified_rows_combine_into_s_joint_with_their_remainders() {
    let (c, n7) = pythagorean();
    let r0 = &c.tie_rods()[0];
    let (k, e_ref) = (r0.stiffness(), r0.reference_extension());
    let u = [
        1.0e-4, -2.5e-4, 5.0e-4, 1.0e-3, -2.0e-3, 0.5e-3, 3.0e-4, 1.25e-4, -7.5e-4, -1.0e-3,
        2.5e-4, 1.5e-3,
    ];
    let bound = MemberBoundInputs {
        w: vec![0.0; 12],
        w2: vec![0.0; 12],
        exponents: vec![0; 12],
        lambda_sigma: 2f64.powi(-40),
    };
    let rod = certify_tie_rod_recovery(&c, 0, &u, &bound).unwrap();
    let loads = CertifiedMemberLoads::default();
    let connector = super::super::certify_connector_recovery(&c, &loads, &u, &bound).unwrap();
    let pe = crate::structural::CertifiedValue::exact(2.5).unwrap();
    let projected = rod.tension.product_value(rod.axial_share).unwrap();
    let s = connector.g[0]
        .add_row(projected)
        .unwrap()
        .add_value(
            pe.product(crate::structural::CertifiedValue::exact(-1.0).unwrap())
                .unwrap(),
        )
        .unwrap();
    assert_eq!(
        s.remainder.to_bits(),
        add_up(connector.g[0].remainder, projected.remainder).to_bits()
    );
    assert!(projected.remainder >= rod.tension.remainder * 2.0 / 7.0 * (1.0 - 1e-15));
    // The exact S·49 = 49 g_x + 2·(7 T) − 49·2.5, with 7T = k(N·u) − 7 k e_ref
    // and g_x the connector's elastic action (K_xx = 5e6, q_x = B_x·u).
    let b = c.b();
    let mut exact = ExactAccumulator::new();
    for l in 0..12 {
        add_triple(&mut exact, 49.0 * 5e6, b[0][l], u[l]);
        add_triple(&mut exact, 2.0 * k, n7[l], u[l]);
    }
    add_triple(&mut exact, -14.0, k, e_ref);
    exact.add_product(-49.0, 2.5).unwrap();
    exact.add_product(-49.0, s.published).unwrap();
    let mut high = exact.clone();
    high.add_product(-49.0, s.bound()).unwrap();
    exact.add_product(49.0, s.bound()).unwrap();
    assert!(
        high.signum() <= 0 && exact.signum() >= 0,
        "S_joint outside its bound"
    );
}

/// K1 repair 5: a skew-Q, non-dyadic rod's certified T_r, e and n·Q_x
/// contain an independent high-precision reference. Provenance: the
/// constants below were printed by this script (Python 3, exact rationals of
/// the binary64 inputs, one `Decimal` square root at 110 digits; it imports
/// and runs no product code), each value split into three binary64 parts:
///
/// ```text
/// from fractions import Fraction as F; from decimal import Decimal as D, getcontext
/// getcontext().prec = 110
/// T, TT = 1.0/3.0, 2.0/3.0; Q = [[T, TT, -TT], [TT, T, TT], [TT, -TT, -T]]
/// xi, xj = [0.3, -0.7, 1.1], [0.6, -0.1, 1.7]
/// off_i, off_j = [-0.05, 0.13, -0.07], [0.06, -0.11, 0.09]; k, e_ref = 2.5e7, -1.25e-3
/// u = [1.3e-4, -2.7e-4, 5.1e-4, 1.1e-3, -2.3e-3, 0.7e-3, 3.3e-4, 1.9e-4, -7.1e-4, -1.3e-3, 2.9e-4, 1.7e-3]
/// w = [3e-17*(i-5.5) for i in range(12)]; w2 = [1e-30*(3-i) for i in range(12)]
/// o = lambda off: [sum(F(Q[r][m])*F(off[m]) for m in range(3)) for r in range(3)]
/// oi, oj = o(off_i), o(off_j); d = [(F(xj[c])-F(xi[c]))+(oj[c]-oi[c]) for c in range(3)]
/// dec = lambda q: D(q.numerator)/D(q.denominator); l = dec(sum(v*v for v in d)).sqrt()
/// n = [dec(v)/l for v in d]; cross(a, b) = a × b
/// C = [-n, -(oi × n), n, oj × n]; x = u + w + w2 (exact)
/// e = C·x; T = k(e - e_ref); share = n·(Q[0][0], Q[1][0], Q[2][0])
/// triple(v) = (hi = float(v), mid = float(v - hi), lo = float(v - hi - mid))
/// ```
///
/// TENSION = 6.3575455112948586325183332225323170980426e+3,
/// EXTENSION = −9.9569817954820568072011881075081373225716e−4,
/// AXIAL_SHARE = 9.6155596132692465867499748486195087208128e−1.
#[test]
fn certified_skew_non_dyadic_rod_contains_the_independent_100_digit_reference() {
    const TENSION: [f64; 3] = [
        6357.545511294858,
        1.8966605963912016e-13,
        -1.207220542619584e-29,
    ];
    const EXTENSION: [f64; 3] = [
        -0.0009956981795482057,
        -1.1435918918362626e-20,
        -5.377309981442046e-37,
    ];
    const AXIAL_SHARE: [f64; 3] = [
        0.9615559613269247,
        -9.922373752548747e-18,
        -2.454128705382956e-34,
    ];
    let c = connector(
        [0.3, -0.7, 1.1],
        [0.6, -0.1, 1.7],
        SKEW,
        &[rod(
            [-0.05, 0.13, -0.07],
            [0.06, -0.11, 0.09],
            2.5e7,
            -1.25e-3,
        )],
    );
    let u = [
        1.3e-4, -2.7e-4, 5.1e-4, 1.1e-3, -2.3e-3, 0.7e-3, 3.3e-4, 1.9e-4, -7.1e-4, -1.3e-3, 2.9e-4,
        1.7e-3,
    ];
    let bound = MemberBoundInputs {
        w: (0..12).map(|i| 3e-17 * (i as f64 - 5.5)).collect(),
        w2: (0..12).map(|i| 1e-30 * (3.0 - i as f64)).collect(),
        exponents: vec![0; 12],
        lambda_sigma: 0.0,
    };
    let certified = certify_tie_rod_recovery(&c, 0, &u, &bound).unwrap();
    // |published − reference| ≤ bound + slack, decided exactly; the slack
    // covers the reference's three-part truncation (≤ 2^-150 relative).
    let inside = |published: f64, bound: f64, reference: [f64; 3]| {
        let slack = reference[0].abs() * 2f64.powi(-140);
        let mut acc = ExactAccumulator::new();
        acc.add(published).unwrap();
        for part in reference {
            acc.add(-part).unwrap();
        }
        let mut high = acc.clone();
        high.add(-bound).unwrap();
        high.add(-slack).unwrap();
        acc.add(bound).unwrap();
        acc.add(slack).unwrap();
        high.signum() <= 0 && acc.signum() >= 0
    };
    let t = certified.tension;
    assert!(inside(t.published, t.bound(), TENSION), "T_r");
    assert!(
        t.radius <= 1e-25 * TENSION[0].abs(),
        "T_r radius {:e}",
        t.radius
    );
    let e = certified.extension;
    assert!(inside(e.published, e.bound(), EXTENSION), "e");
    let share = certified.axial_share;
    let published = share.published().unwrap();
    // A ball's publication gap is at most u·|published| (correct rounding).
    let gap = published.abs() * f64::EPSILON / 2.0 * (1.0 + f64::EPSILON);
    assert!(
        inside(published, gap + share.radius(), AXIAL_SHARE),
        "n·Q_x"
    );
    // The binary64 element agrees to roundoff (it uses fl(Q·offset)).
    assert!((c.tie_rods()[0].axial_share() - AXIAL_SHARE[0]).abs() <= 4.0 * f64::EPSILON);
    // Discrimination: the reference with e_ref ignored lies outside.
    let wrong = [TENSION[0] - 2.5e7 * 1.25e-3, TENSION[1], TENSION[2]];
    assert!(!inside(t.published, t.bound(), wrong));
}
