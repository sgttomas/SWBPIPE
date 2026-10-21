//! RF source-method design §3.3 and §8 (RF-10, the frame-kernel part): the
//! exact-congruence source of a non-axis-aligned member on the functional
//! entry, its validation, the retained operands and their replay, and their
//! charging and sizing before any copy.
//!
//! The members are real `FrameElement`s with the census's RF-SKEW-T-PIN-OFF
//! section: a (3,4,0) and a (1,2,2) direction. The exact congruence is checked
//! against an independent oracle, the direct triple sum
//! Σ_k Σ_l T[k][i]·K[k][l]·T[l][j] over arbitrary-precision integers formed
//! from the operands' binary64 bits, and the selected displacements against
//! Cramer's rule over the whole free system (`block_order_tests::oracle`). It
//! calls none of the producer's expansion arithmetic.
use super::block_order_tests::equals;
use super::block_order_tests::oracle::{self, Int};
use super::*;
use crate::{FrameElement, FrameNode, FrameSection};

const ID: &str = "congruence source";

/// The census's RF-SKEW-T-PIN-OFF section (E, G, A, Iy, Iz, J) as the
/// product forms it for the census's direct method (RF design §1.1, P1).
fn section() -> FrameSection {
    let b = f64::from_bits;
    FrameSection::new(
        b(0x4247_4876_e800_0000),
        b(0x4232_a05f_2000_0000),
        b(0x3f78_72fa_3a37_ac13),
        b(0x3efc_5266_4442_210b),
        b(0x3efc_5266_4442_210b),
        b(0x3f0c_5266_4442_210b),
    )
    .unwrap()
}
/// A member from node 0 at the origin to node 1 at `end`.
fn element(end: [f64; 3], y_reference: [f64; 3]) -> FrameElement {
    FrameElement::new(
        FrameNode::new(0, [0.0; 3]).unwrap(),
        FrameNode::new(1, end).unwrap(),
        section(),
        y_reference,
    )
    .unwrap()
}
/// The census's RF-SKEW-T-PIN-OFF-345 member: direction (3,4,0), y reference
/// (0,0,1).
pub(crate) const OFF_345: ([f64; 3], [f64; 3]) = ([3.0, 4.0, 0.0], [0.0, 0.0, 1.0]);
/// The census's RF-SKEW-T-PIN-OFF-122 member: direction (1,2,2), y reference
/// (1,0,0).
pub(crate) const OFF_122: ([f64; 3], [f64; 3]) = ([1.0, 2.0, 2.0], [1.0, 0.0, 0.0]);
/// A census member's congruence entry over the DOF map 0..12 with
/// contributions 0..144 (its declared `local_stiffness()` and
/// `orientation().transformation_matrix()`), and its rounded
/// `global_stiffness()`.
pub(crate) fn member((end, y_reference): ([f64; 3], [f64; 3])) -> (CongruenceMember, Matrix12) {
    let e = element(end, y_reference);
    let member = CongruenceMember {
        dofs: std::array::from_fn(|i| i),
        local_stiffness: e.local_stiffness().unwrap(),
        transform: e.orientation().unwrap().transformation_matrix(),
        contributions: 0..144,
    };
    (member, e.global_stiffness().unwrap())
}

/// A one-member source of 12 DOFs: the member's 144 rounded global
/// contributions, row-major at its map 0..12, then the ground springs; the
/// aggregate folded in that order (as the product assembles it); the loads as
/// the declared vector; every DOF not free prescribed at zero.
struct Model {
    rounded: Matrix12,
    springs: Vec<(usize, f64)>,
    stiffness: Vec<Vec<f64>>,
    contributions: Vec<StiffnessContribution>,
    force: Vec<f64>,
    free: Vec<usize>,
    fixed: Vec<(usize, f64)>,
    congruence: Vec<CongruenceMember>,
}
impl Model {
    fn new(
        geometry: ([f64; 3], [f64; 3]),
        springs: &[(usize, f64)],
        loads: &[(usize, f64)],
        free: &[usize],
    ) -> Self {
        let (member, rounded) = member(geometry);
        let mut contributions = Vec::new();
        for (row, values) in rounded.iter().enumerate() {
            for (col, &value) in values.iter().enumerate() {
                contributions.push(StiffnessContribution { row, col, value });
            }
        }
        for &(dof, value) in springs {
            contributions.push(StiffnessContribution { row: dof, col: dof, value });
        }
        let mut stiffness = vec![vec![0.0; 12]; 12];
        for c in &contributions {
            stiffness[c.row][c.col] += c.value;
        }
        let mut force = vec![0.0; 12];
        for &(dof, value) in loads {
            force[dof] += value;
        }
        Self {
            rounded,
            springs: springs.to_vec(),
            stiffness,
            contributions,
            force,
            free: free.to_vec(),
            fixed: (0..12).filter(|d| !free.contains(d)).map(|d| (d, 0.0)).collect(),
            congruence: vec![member],
        }
    }
    fn system(&self) -> StructuralSystem<'_> {
        StructuralSystem {
            stiffness: &self.stiffness,
            force: &self.force,
            free_dofs: &self.free,
            prescribed: &self.fixed,
            contributions: Some(&self.contributions),
            contribution_groups: None,
            symmetry: None,
        }
    }
    fn prepare(&self, congruence: &[CongruenceMember], budget: &mut AttemptBudget) -> Result<Context, Error> {
        Context::prepare_with_budget(&self.system(), ID, ForceBasis::DeclaredVector, congruence, budget).result
    }
}
/// RF-SKEW-T-PIN-OFF-345-r1e-08 (j7 census): one (3,4,0) member, every
/// translation fixed, the soft rotational spring at node 0's RX and the two
/// moments at node 1; free blocks [4, 2].
fn skew_345() -> Model {
    let b = f64::from_bits;
    Model::new(
        OFF_345,
        &[(3, b(0x3f81_b1d9_2b7f_e08b))],
        &[(9, b(0x3ea1_6505_a7da_8610)), (10, b(0x3ea7_315c_dfce_0816))],
        &[3, 4, 5, 9, 10, 11],
    )
}
/// The census's (1,2,2) member with node 1 fixed, node 0's translations fixed
/// and moments on its rotations: one free block of order 3.
fn skew_122() -> Model {
    Model::new(OFF_122, &[], &[(3, 0.5), (4, -1.25), (5, 2.0)], &[3, 4, 5])
}

fn budget() -> AttemptBudget {
    AttemptBudget::new(Limits {
        operations: 200_000_000,
        ..Limits::default()
    })
}
fn orders(c: &Context) -> Vec<usize> {
    c.blocks.iter().map(Vec::len).collect()
}

/// The model's exact matrix as integers over 2^scale: the member's congruence
/// by the direct triple sum over the operands' exact integers, plus its
/// springs. The scale also makes the rounded entries and the loads integral.
fn exact_matrix(m: &Model) -> (Vec<Vec<Int>>, u32) {
    let c = &m.congruence[0];
    let shift = |v: &mut dyn Iterator<Item = f64>| v.map(oracle::shift).max().unwrap_or(0);
    let st = shift(&mut c.transform.iter().flatten().copied());
    let sk = shift(&mut c.local_stiffness.iter().flatten().copied());
    let scale = (2 * st + sk)
        .max(shift(&mut m.rounded.iter().flatten().copied()))
        .max(shift(&mut m.springs.iter().map(|s| s.1)))
        .max(shift(&mut m.force.iter().copied()));
    let t: Vec<Vec<Int>> = c.transform.iter().map(|r| r.iter().map(|&x| oracle::scaled(x, st)).collect()).collect();
    let k: Vec<Vec<Int>> = c.local_stiffness.iter().map(|r| r.iter().map(|&x| oracle::scaled(x, sk)).collect()).collect();
    let mut g = vec![vec![Int::zero(); 12]; 12];
    for i in 0..12 {
        for j in 0..12 {
            let mut sum = Int::zero();
            for a in 0..12 {
                for b in 0..12 {
                    sum = sum.add(&t[a][i].mul(&k[a][b]).mul(&t[b][j]));
                }
            }
            g[i][j] = sum.shl(scale - (2 * st + sk));
        }
    }
    for &(dof, value) in &m.springs {
        g[dof][dof] = g[dof][dof].add(&oracle::scaled(value, scale));
    }
    (g, scale)
}
/// The exact sum of the rounded contributions, as integers over 2^scale.
fn rounded_matrix(m: &Model, scale: u32) -> Vec<Vec<Int>> {
    let mut r = vec![vec![Int::zero(); 12]; 12];
    for c in &m.contributions {
        r[c.row][c.col] = r[c.row][c.col].add(&oracle::scaled(c.value, scale));
    }
    r
}
/// The expansion `terms` equals exact/2^scale.
fn same_value(terms: &[f64], exact: &Int, scale: u32) -> bool {
    let t = terms.iter().map(|&x| oracle::shift(x)).max().unwrap_or(0);
    oracle::sum_scaled(terms, t).shl(scale) == exact.shl(t)
}
/// Cramer's rule over the whole free system (every prescribed value is zero):
/// u_a = det(K'_ff with column a replaced by f'_f) / det(K'_ff), with K' and
/// f' the exact integers over the same 2^scale.
fn exact_solution(m: &Model, k: &[Vec<Int>], scale: u32) -> Vec<(Int, Int)> {
    let kff: Vec<Vec<Int>> = m.free.iter().map(|&i| m.free.iter().map(|&j| k[i][j].clone()).collect()).collect();
    let d = oracle::det(&kff);
    assert!(!d.is_zero());
    let mut u = vec![(Int::zero(), Int::from_i64(1)); 12];
    for (a, &i) in m.free.iter().enumerate() {
        let mut replaced = kff.clone();
        for (row, &r) in replaced.iter_mut().zip(&m.free) {
            row[a] = oracle::scaled(m.force[r], scale);
        }
        u[i] = (oracle::det(&replaced), d.clone());
    }
    u
}
/// The full functional chain (DOF leaves and one declared form over the free
/// DOFs), retained on `budget`.
fn retain(m: &Model, budget: &mut AttemptBudget) -> (RetainedFunctionalSet, Vec<FunctionalDescriptor>) {
    let c = m.prepare(&m.congruence, budget).unwrap();
    let r = c.solve_with_budget(budget).result.unwrap();
    let ds = vec![FunctionalDescriptor {
        key: FunctionalKey {
            case_id: "congruence".into(),
            quantity: FunctionalQuantity::DeclaredAffine { label: "free sum".into() },
            unit: FunctionalUnit::Dimensionless,
            convention: FunctionalConvention::DeclaredAffine,
        },
        offset: vec![],
        terms: m
            .free
            .iter()
            .map(|&dof| AffineTerm {
                dof,
                products: vec![vec![1.0]],
            })
            .collect(),
    }];
    let p = FunctionalPlan::new(&c, &ds, budget).result.unwrap();
    let set = r.evaluate_functionals(&p, budget).result.unwrap();
    let mut leaves = Vec::new();
    for i in 0..12 {
        for quantity in [Quantity::Displacement, Quantity::Reaction] {
            leaves.push(r.project_with_budget(quantity, i, 1e-9, budget).result.unwrap());
        }
    }
    let projections: Vec<_> = (0..set.len()).map(|i| set.project(i, 1e-9, budget).result.unwrap()).collect();
    let retained = set.retain(&[], &leaves, &projections, budget).result.unwrap();
    (retained, ds)
}

fn unlimited() -> Work {
    Work {
        limits: Limits {
            operations: usize::MAX,
            ..Limits::default()
        },
        used: 0,
        rejected: WorkTotal::zero(),
    }
}
/// The direct triple sum through `Work`, the form RF design §3.5 compares
/// with: each product (T[k][i]·K[k][l])·T[l][j] with no zero factor,
/// accumulated in (k, l) order from its first product.
fn direct_triple_sum(k: &Matrix12, t: &Matrix12, w: &mut Work) -> Vec<Vec<Expansion>> {
    let mut g = vec![vec![Expansion::default(); 12]; 12];
    for (i, row) in g.iter_mut().enumerate() {
        for (j, entry) in row.iter_mut().enumerate() {
            let mut sum: Option<Expansion> = None;
            for a in 0..12 {
                for b in 0..12 {
                    if t[a][i] == 0.0 || k[a][b] == 0.0 || t[b][j] == 0.0 {
                        continue;
                    }
                    let (x, y, z) = (w.scalar(t[a][i]).unwrap(), w.scalar(k[a][b]).unwrap(), w.scalar(t[b][j]).unwrap());
                    let xy = w.mul(&x, &y).unwrap();
                    let product = w.mul(&xy, &z).unwrap();
                    sum = Some(match sum {
                        None => product,
                        Some(s) => w.sum(&s, &product, 1.0).unwrap(),
                    });
                }
            }
            *entry = sum.unwrap_or_default();
        }
    }
    g
}

#[test]
fn congruence_formation_charges_the_specified_two_stage_work() {
    // RF design §3.5 (P3, under the uniform rule): the census members'
    // two-stage Tᵀ·(K·T) charged 29,416 and 84,268, and the direct triple sum
    // 33,608 and 102,076, with value-equal entries. The producer's formation
    // also reserves its two 144-entry buffers.
    for (geometry, two_stage, direct) in [(OFF_345, 29_416, 33_608), (OFF_122, 84_268, 102_076)] {
        let (c, _) = member(geometry);
        let mut w = unlimited();
        let g = congruence_entries(&c.local_stiffness, &c.transform, &mut w).unwrap();
        assert_eq!(w.used, two_stage + 2 * 144);
        let mut v = unlimited();
        let d = direct_triple_sum(&c.local_stiffness, &c.transform, &mut v);
        assert_eq!(v.used, direct);
        let mut check = unlimited();
        for (a, b) in g.iter().flatten().zip(d.iter().flatten()) {
            assert!(check.sum(a, b, -1.0).unwrap().is_zero());
        }
        // The Context's path: this formation, then each entry's terms added
        // into k (here an empty k), so each entry's value is its G entry's.
        let mut k = vec![vec![Expansion::default(); 12]; 12];
        let mut s = unlimited();
        add_congruence(&mut k, &c, &mut s).unwrap();
        let mut scatter = unlimited();
        for e in g.iter().flatten() {
            let mut x = Expansion::default();
            for &term in &e.terms {
                scatter.add(&mut x, term).unwrap();
            }
        }
        assert_eq!(s.used, w.used + scatter.used);
        for (a, b) in k.iter().flatten().zip(g.iter().flatten()) {
            assert!(check.sum(a, b, -1.0).unwrap().is_zero());
        }
    }
}

#[test]
fn congruence_source_is_the_exact_symmetric_congruence() {
    // (model, its rounded contributions' exactly asymmetric pairs, block
    // orders, and the census operands' K_local[0][0] and T[0][0] bits)
    for (m, asymmetric, expected_orders, anchors) in [
        (skew_345(), 4, vec![4, 2], [0x41ac_7668_a358_7c6d, 0x3fe3_3333_3333_3334]),
        (skew_122(), 0, vec![3], [0x41b7_b801_dd74_67b0, 0x3fd5_5555_5555_5555]),
    ] {
        let declared = &m.congruence[0];
        assert_eq!([declared.local_stiffness[0][0], declared.transform[0][0]].map(f64::to_bits), anchors);
        let (exact, scale) = exact_matrix(&m);
        // The oracle's congruence is exactly symmetric.
        for i in 0..12 {
            for j in 0..i {
                assert_eq!(exact[i][j], exact[j][i], "oracle ({i},{j})");
            }
        }
        // RF design §1.3: the rounded global contributions fl(Tᵀ·fl(K·T)) are
        // exactly asymmetric in four entry pairs for the (3,4,0) member; and
        // for both they differ from the exact congruence (non-vacuous).
        let pairs = (0..12)
            .flat_map(|p| (p + 1..12).map(move |q| (p, q)))
            .filter(|&(p, q)| m.rounded[p][q].to_bits() != m.rounded[q][p].to_bits())
            .count();
        assert_eq!(pairs, asymmetric);
        let rounded = rounded_matrix(&m, scale);
        let differ = (0..12).flat_map(|i| (0..12).map(move |j| (i, j))).filter(|&(i, j)| rounded[i][j] != exact[i][j]).count();
        assert!(differ > 0);

        // The functional entry with the member's congruence: k is the exact
        // congruence (plus the springs), entry by entry.
        let mut b = budget();
        let c = m.prepare(&m.congruence, &mut b).unwrap();
        assert_eq!(orders(&c), expected_orders);
        for i in 0..12 {
            for j in 0..12 {
                assert!(same_value(&c.k[i][j].terms, &exact[i][j], scale), "k ({i},{j})");
            }
        }
        // Coverage is unchanged: the rounded contributions stay in the
        // source, and on the (3,4,0) member's asymmetric entries only the
        // ordered fold (how the aggregate was formed) matches the aggregate.
        assert_eq!(c.source.contributions, m.contributions);
        let ordered_only = (0..12)
            .flat_map(|i| (0..12).map(move |j| (i, j)))
            .filter(|&(i, j)| {
                let projected: f64 = c.k[i][j].terms.iter().sum();
                projected != m.stiffness[i][j] && !correctly_rounded_matches(&c.k[i][j], m.stiffness[i][j])
            })
            .count();
        if asymmetric > 0 {
            assert!(ordered_only > 0);
        }
        // It selects: solve, the exact solution, zero free reactions.
        let r = c.solve_with_budget(&mut b).result.unwrap();
        let u = exact_solution(&m, &exact, scale);
        for (i, ui) in u.iter().enumerate() {
            assert!(equals(r.displacement(i).unwrap(), ui), "u{i}");
        }
        for &i in &m.free {
            assert_eq!(r.reaction(i).unwrap().sign(), Sign::Zero);
            let p = r.project_with_budget(Quantity::Displacement, i, 1e-9, &mut b).result.unwrap();
            assert!(p.relative_error_bound() <= 1e-9);
        }

        // Without the list the rounded contributions are the exact source:
        // the (3,4,0) member is refused as asymmetric, on the functional entry
        // and on the legacy entry (which takes no list); the (1,2,2) member's
        // k is then not its congruence.
        let without = m.prepare(&[], &mut budget());
        if asymmetric > 0 {
            assert_eq!(without.unwrap_err(), Error::Invalid("exact source asymmetry"));
            assert_eq!(
                Context::new(&m.system(), ID, ForceBasis::DeclaredVector, Limits::default()).unwrap_err(),
                Error::Invalid("exact source asymmetry")
            );
        } else {
            let w = without.unwrap();
            assert!((0..12).any(|i| (0..12).any(|j| !same_value(&w.k[i][j].terms, &exact[i][j], scale))));
        }
    }
}

#[test]
fn congruence_list_is_validated_and_keeps_the_coverage_check() {
    let m = skew_345();
    let refuse = |list: &[CongruenceMember], what: &str, expected: Error| {
        assert_eq!(m.prepare(list, &mut budget()).unwrap_err(), expected, "{what}");
    };
    let good = &m.congruence[0];
    let map = Error::Invalid("congruence member map");
    let operands = Error::Invalid("congruence member operands");
    let contributions = Error::Invalid("congruence member contributions");
    let mut x = good.clone();
    x.dofs[11] = 12;
    refuse(&[x], "map outside the source", map);
    for (row, col, value) in [(0, 0, f64::NAN), (1, 5, f64::INFINITY)] {
        let mut x = good.clone();
        x.local_stiffness[row][col] = value;
        refuse(&[x], "non-finite K_local", operands.clone());
        let mut x = good.clone();
        x.transform[row][col] = value;
        refuse(&[x], "non-finite T", operands.clone());
    }
    for (range, what) in [
        (0..143, "short range"),
        (1..145, "shifted range"),
        (2..146, "range past the contributions"),
        (144..0, "reversed range"),
    ] {
        let mut x = good.clone();
        x.contributions = range;
        refuse(&[x], what, contributions.clone());
    }
    let mut x = good.clone();
    x.dofs.swap(0, 1);
    refuse(&[x], "map not at the contributions' positions", contributions.clone());
    refuse(&[good.clone(), good.clone()], "overlapping ranges", contributions.clone());
    // The checks apply to the congruence k: an asymmetric K_local is refused.
    let mut x = good.clone();
    x.local_stiffness[1][5] = f64::from_bits(x.local_stiffness[1][5].to_bits() + 1);
    refuse(&[x], "asymmetric K_local", Error::Invalid("exact source asymmetry"));
    // No coverage bypass on the member's entries: a changed aggregate entry
    // that the member touches is refused.
    for (i, j) in [(4, 10), (3, 3)] {
        let mut changed = skew_345();
        changed.stiffness[i][j] = f64::from_bits(changed.stiffness[i][j].to_bits() + 1024);
        assert_eq!(
            changed.prepare(&changed.congruence, &mut budget()).unwrap_err(),
            Error::Invalid("stiffness source mismatch"),
            "({i},{j})"
        );
    }
    // A well-formed list at any position: the member's range after the spring.
    let mut moved = skew_345();
    let spring = moved.contributions.pop().unwrap();
    moved.contributions.insert(0, spring);
    moved.congruence[0].contributions = 1..145;
    assert_eq!(orders(&moved.prepare(&moved.congruence, &mut budget()).unwrap()), [4, 2]);
}

#[test]
fn congruence_replay_refuses_a_changed_operand_map_or_range() {
    let source = Error::Invalid("functional expected source binding");
    let binding = Error::Invalid("functional complete expected source/plan binding");
    // (model, whether its rounded contributions are exactly asymmetric)
    for (m, asymmetric) in [(skew_345(), true), (skew_122(), false)] {
        let mut b = budget();
        let (retained, ds) = retain(&m, &mut b);
        assert_eq!(retained.response().congruence_members(), &m.congruence[..]);
        let replay = |set: &RetainedFunctionalSet, list: &[CongruenceMember], b: &mut AttemptBudget| {
            set.replay_against(&m.system(), ID, ForceBasis::DeclaredVector, list, &ds, None, &[], 1e-9, b).result
        };
        let bind = |set: &RetainedFunctionalSet, list: &[CongruenceMember], b: &mut AttemptBudget| {
            set.check_binding_with_budget(&m.system(), ID, ForceBasis::DeclaredVector, list, &ds, b).result
        };
        assert!(bind(&retained, &m.congruence, &mut b).is_ok());
        assert_eq!(replay(&retained, &m.congruence, &mut b).unwrap().functionals(), 1);

        let good = &m.congruence[0];
        let flip = |x: f64| f64::from_bits(x.to_bits() ^ 1);
        let mut changed: Vec<(&str, CongruenceMember)> = Vec::new();
        // K_local changed symmetrically (so the re-formed source is valid).
        let mut x = good.clone();
        x.local_stiffness[1][5] = flip(x.local_stiffness[1][5]);
        x.local_stiffness[5][1] = flip(x.local_stiffness[5][1]);
        changed.push(("K_local bit", x));
        let mut x = good.clone();
        x.transform[0][0] = flip(x.transform[0][0]);
        changed.push(("T bit", x));
        let mut x = good.clone();
        x.dofs.swap(3, 4);
        changed.push(("map", x));
        let mut x = good.clone();
        x.contributions = 1..145;
        changed.push(("range", x));
        for (what, x) in &changed {
            // In the expected list: replay re-forms from it, and refuses at
            // the binding (or earlier, at its validation); check_binding
            // refuses at the binding.
            let expected = match *what {
                "map" | "range" => Error::Invalid("congruence member contributions"),
                _ => source.clone(),
            };
            assert_eq!(replay(&retained, std::slice::from_ref(x), &mut b).unwrap_err(), expected, "expected {what}");
            assert_eq!(bind(&retained, std::slice::from_ref(x), &mut b).unwrap_err(), binding, "expected {what}");
            // In the retained source: refused at the binding.
            let mut corrupt = retained.clone();
            corrupt.response.source.congruence[0] = x.clone();
            assert_eq!(replay(&corrupt, &m.congruence, &mut b).unwrap_err(), source, "retained {what}");
            assert_eq!(bind(&corrupt, &m.congruence, &mut b).unwrap_err(), binding, "retained {what}");
        }
        // The list omitted: the rounded contributions are then the source.
        assert_eq!(
            replay(&retained, &[], &mut b).unwrap_err(),
            if asymmetric {
                Error::Invalid("exact source asymmetry")
            } else {
                source.clone()
            }
        );
        assert_eq!(bind(&retained, &[], &mut b).unwrap_err(), binding);
        let mut corrupt = retained.clone();
        corrupt.response.source.congruence.clear();
        assert_eq!(replay(&corrupt, &m.congruence, &mut b).unwrap_err(), source);
        // The legacy replay takes no list: a congruence source cannot bind.
        let legacy = retained.response().replay_against(
            &m.system(),
            ID,
            ForceBasis::DeclaredVector,
            &[],
            1e-9,
            retained.response().limits(),
            usize::MAX,
        );
        assert_eq!(legacy.result.unwrap_err(), Error::Invalid("retained expected-source binding"));
        // Nothing above disturbed the retained proof.
        assert_eq!(replay(&retained, &m.congruence, &mut b).unwrap().functionals(), 1);
    }
}

#[test]
fn congruence_operands_are_charged_and_size_checked_before_any_copy() {
    let m = skew_345();
    let mut b = budget();
    let (retained, ds) = retain(&m, &mut b);
    let snapshot = &retained.response.source;
    // The copy charge carries each member's operands, map and range.
    let mut bare = snapshot.clone();
    bare.congruence.clear();
    assert_eq!(snapshot.copy_charge() - bare.copy_charge(), CONGRUENCE_SCALARS);
    assert_eq!(CONGRUENCE_SCALARS, 2 * 144 + 12 + 2);
    // The preflight reserves 8 per member scalar, and counts the 288 operand
    // values as source terms beside the member's 144 contributions.
    let preflight = |list: &[CongruenceMember], limits: Limits| {
        let mut w = Work {
            limits,
            used: 0,
            rejected: WorkTotal::zero(),
        };
        let result = source_preflight(&m.system(), ID, ForceBasis::DeclaredVector, list, &mut w);
        (result, w.used)
    };
    let (with, charged) = preflight(&m.congruence, Limits::default());
    let (without, bare_charged) = preflight(&[], Limits::default());
    assert!(with.is_ok() && without.is_ok());
    assert_eq!(charged - bare_charged, 8 * CONGRUENCE_SCALARS);
    let count = m.contributions.len() + 12 + CONGRUENCE_VALUES;
    let at = |source_terms| Limits {
        source_terms,
        ..Limits::default()
    };
    assert!(preflight(&m.congruence, at(count)).0.is_ok());
    assert_eq!(preflight(&m.congruence, at(count - 1)), (Err(Error::Budget), 16));
    assert_eq!(preflight(&[], at(count - CONGRUENCE_VALUES)).0, Ok(()));
    // Through the entry: refused by size before the Context is built.
    let mut small = AttemptBudget::new(at(count - 1));
    let refused = Context::prepare_with_budget(&m.system(), ID, ForceBasis::DeclaredVector, &m.congruence, &mut small);
    assert_eq!(refused.result.unwrap_err(), Error::Budget);
    assert_eq!((refused.work.charged, refused.work.rejected.exact().unwrap()), (16, 0));

    // Retention reserves the copies, the operands included, as its last
    // charge, before it copies anything: one below the stage's total refuses
    // exactly that reservation.
    let total = b.report().charged;
    let copies = {
        let r = &retained.response;
        let mut n = snapshot.copy_charge();
        for w in &r.witnesses {
            n += w.scalar_count();
        }
        for x in r.displacement.iter().chain(&r.reactions) {
            n += ratio_charge(x);
        }
        n + 12 * r.projections.len()
    };
    let mut fresh = AttemptBudget::new(Limits {
        operations: total - 1,
        ..Limits::default()
    });
    let c = m.prepare(&m.congruence, &mut fresh).unwrap();
    let r = c.solve_with_budget(&mut fresh).result.unwrap();
    let p = FunctionalPlan::new(&c, &ds, &mut fresh).result.unwrap();
    let set = r.evaluate_functionals(&p, &mut fresh).result.unwrap();
    let mut leaves = Vec::new();
    for i in 0..12 {
        for quantity in [Quantity::Displacement, Quantity::Reaction] {
            leaves.push(r.project_with_budget(quantity, i, 1e-9, &mut fresh).result.unwrap());
        }
    }
    let projections: Vec<_> = (0..set.len()).map(|i| set.project(i, 1e-9, &mut fresh).result.unwrap()).collect();
    let before = fresh.report();
    let denied = set.retain(&[], &leaves, &projections, &mut fresh);
    assert_eq!(denied.result.unwrap_err(), Error::Budget);
    assert_eq!(
        denied.work.rejected.exact().unwrap() - before.rejected.exact().unwrap(),
        copies as u64
    );
    assert_eq!(denied.work.charged, total - copies);

    // The functional replay sizes the retained operands, and check_binding
    // the expected ones, before any copy or re-formation: a list past the
    // source-term cap is refused with nothing but the preflight charged.
    let padded = vec![m.congruence[0].clone(); 60];
    assert!(60 * CONGRUENCE_VALUES > 16_384);
    let descriptors = {
        let mut w = Work {
            limits: Limits::default(),
            used: 0,
            rejected: WorkTotal::zero(),
        };
        descriptors_charge(&ds, 12, &mut w).unwrap();
        w.used
    };
    let mut corrupt = retained.clone();
    corrupt.response.source.congruence = padded.clone();
    let before = b.report();
    let replay = corrupt.replay_against(&m.system(), ID, ForceBasis::DeclaredVector, &m.congruence, &ds, None, &[], 1e-9, &mut b);
    assert_eq!(replay.result.unwrap_err(), Error::Budget);
    assert_eq!(replay.work.charged - before.charged, 24 + 2 * descriptors + 8 + 16);
    assert_eq!(replay.work.rejected, before.rejected);
    let before = b.report();
    let bind = retained.check_binding_with_budget(&m.system(), ID, ForceBasis::DeclaredVector, &padded, &ds, &mut b);
    assert_eq!(bind.result.unwrap_err(), Error::Budget);
    assert_eq!(bind.work.charged - before.charged, 16);
    let before = b.report();
    let replay = retained.replay_against(&m.system(), ID, ForceBasis::DeclaredVector, &padded, &ds, None, &[], 1e-9, &mut b);
    assert_eq!(replay.result.unwrap_err(), Error::Budget);
    assert_eq!(replay.work.rejected, before.rejected);
}
