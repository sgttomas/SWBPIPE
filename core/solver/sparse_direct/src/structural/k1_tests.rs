//! K1 pattern-path tests (T3 D1 revision 5a.2 §4.8 parity protocol items 1, 2
//! and 6; the K1 row of §6). Invented inputs only; every value is stated here.
//!
//! Today's dense-derived sparse path (`factor_structural_ldlt` on the dense
//! prepared matrix) and the pattern path (`factor_sparse_structural_ldlt` on
//! the sparse prepared system) must give the same order, the same profile
//! and a `Debug`-identical `StructuralSolution` for the same equations.
use super::*;
use crate::{adjacency_from_dense, adjacency_from_symmetric_entries, reverse_cuthill_mckee};
use open_pipe_stress_frame_kernel::connector::{ConnectorAttachment, ObjectiveConnector, ScaledWorkMatrix};
use open_pipe_stress_frame_kernel::load_ledger::{AssembledForce, LoadLedger};
use open_pipe_stress_frame_kernel::structural::{
    assemble_sparse_stiffness, gamma,
    negative_pair_witness, transform_roundoff, FormationSource, SparseAssemblyOptions,
    SparseStiffness, SparseSymmetryEvidence, StiffnessBlock, StiffnessContribution,
    SymmetryEvidence, TransformationRoundoff,
};
use open_pipe_stress_frame_kernel::{
    assemble_global_stiffness_with_connectors, element_dof_map, FrameElement, FrameNode,
    FrameSection, Matrix12,
};

// ------------------------------------------------------------------ models

fn section(e: f64, g: f64) -> FrameSection {
    let (od, id) = (0.2_f64, 0.18_f64);
    let area = std::f64::consts::PI * (od * od - id * id) / 4.0;
    let inertia = std::f64::consts::PI * (od.powi(4) - id.powi(4)) / 64.0;
    FrameSection::new(e, g, area, inertia, inertia, 2.0 * inertia).unwrap()
}

fn node(index: usize, p: [f64; 3]) -> FrameNode {
    FrameNode::new(index, p).unwrap()
}

/// An invented model with every assembled family (frames, a connector,
/// an explicit block, springs) and its boundary and loads.
struct Model {
    node_count: usize,
    frames: Vec<FrameElement>,
    connectors: Vec<ObjectiveConnector>,
    blocks: Vec<StiffnessBlock>,
    /// Each block's formation bounds and counts (`transform_roundoff` of the
    /// element it was formed from).
    block_formation: Vec<TransformationRoundoff>,
    springs: Vec<(usize, f64)>,
    free: Vec<usize>,
    prescribed: Vec<(usize, f64)>,
    loads: Vec<(usize, f64)>,
}

impl Model {
    fn sparse(&self) -> SparseStiffness {
        assemble_sparse_stiffness(
            self.node_count,
            &self.frames,
            &self.connectors,
            &self.blocks,
            &self.springs,
            &SparseAssemblyOptions::new(),
        )
        .unwrap()
    }
    fn dense(&self) -> Vec<Vec<f64>> {
        let mut k = assemble_global_stiffness_with_connectors(
            self.node_count,
            &self.frames,
            &self.connectors,
        )
        .unwrap();
        for block in &self.blocks {
            let map = element_dof_map(block.node_i, block.node_j);
            for (r, &row) in map.iter().enumerate() {
                for (c, &col) in map.iter().enumerate() {
                    k[row][col] += block.stiffness[r][c];
                }
            }
        }
        for &(dof, value) in &self.springs {
            k[dof][dof] += value;
        }
        k
    }
    fn contributions(&self) -> Vec<StiffnessContribution> {
        let mut out = Vec::new();
        let mut push = |a: usize, b: usize, k: &Matrix12| {
            let map = element_dof_map(a, b);
            for i in 0..12 {
                for j in 0..12 {
                    out.push(StiffnessContribution {
                        row: map[i],
                        col: map[j],
                        value: k[i][j],
                    });
                }
            }
        };
        for e in &self.frames {
            push(
                e.node_i.index,
                e.node_j.index,
                &e.global_stiffness().unwrap(),
            );
        }
        for c in &self.connectors {
            push(
                c.node_i().index,
                c.node_j().index,
                &c.global_stiffness().unwrap(),
            );
        }
        for b in &self.blocks {
            push(b.node_i, b.node_j, &b.stiffness);
        }
        for &(dof, value) in &self.springs {
            out.push(StiffnessContribution {
                row: dof,
                col: dof,
                value,
            });
        }
        out
    }
    fn force(&self) -> Vec<f64> {
        let mut f = vec![0.0; self.node_count * 6];
        for &(dof, v) in &self.loads {
            f[dof] += v;
        }
        f
    }
    fn ledger(&self) -> AssembledForce {
        let mut ledger = LoadLedger::new();
        for (index, &(dof, v)) in self.loads.iter().enumerate() {
            ledger.push(format!("load:{index}"), dof, v);
        }
        ledger.finish(self.node_count * 6).unwrap()
    }
    /// The formation allowances and operation counts in the form the
    /// structural adapter's `AssemblyEvidence::new` records them (n×n), and
    /// the same values per pattern entry of `k`. A skew member's global
    /// matrix is symmetric only within these allowances; the product always
    /// supplies them.
    fn symmetry(&self, k: &SparseStiffness) -> Symmetry {
        let n = self.node_count * 6;
        let mut roundoff = vec![vec![0.0; n]; n];
        let mut counts = vec![vec![0usize; n]; n];
        let mut element = |a: usize, b: usize, evidence: &TransformationRoundoff| {
            let map = element_dof_map(a, b);
            for i in 0..12 {
                for j in 0..12 {
                    roundoff[map[i]][map[j]] += evidence.absolute_roundoff[i][j];
                    counts[map[i]][map[j]] += evidence.operation_counts[i][j] + 1;
                }
            }
        };
        for e in &self.frames {
            let t = e.orientation().unwrap().transformation_matrix();
            let evidence = transform_roundoff(&e.local_stiffness().unwrap(), &t).unwrap();
            element(e.node_i.index, e.node_j.index, &evidence);
        }
        for c in &self.connectors {
            let (absolute_roundoff, operation_counts) = c.formation_roundoff().unwrap();
            let evidence = TransformationRoundoff {
                absolute_roundoff,
                operation_counts,
                basis: "connector",
            };
            element(c.node_i().index, c.node_j().index, &evidence);
        }
        for (b, evidence) in self.blocks.iter().zip(&self.block_formation) {
            element(b.node_i, b.node_j, evidence);
        }
        for &(dof, _) in &self.springs {
            counts[dof][dof] += 1;
        }
        let mut magnitudes = vec![vec![0.0; n]; n];
        let mut scatter = vec![vec![0usize; n]; n];
        for c in self.contributions() {
            magnitudes[c.row][c.col] += c.value.abs();
            scatter[c.row][c.col] += 1;
        }
        for i in 0..n {
            for j in 0..n {
                roundoff[i][j] += gamma(scatter[i][j]) * magnitudes[i][j];
            }
        }
        let pattern = k.pattern();
        let (sparse_roundoff, sparse_counts) = (0..n)
            .flat_map(|row| {
                pattern
                    .row_range(row)
                    .map(move |e| (row, pattern.column(e)))
            })
            .map(|(i, j)| (roundoff[i][j], counts[i][j]))
            .unzip();
        Symmetry {
            roundoff,
            counts,
            sparse_roundoff,
            sparse_counts,
        }
    }
    fn formation_source(&self) -> FormationSource {
        FormationSource {
            node_count: self.node_count,
            frames: self.frames.clone(),
            connectors: self.connectors.clone(),
            curved: Vec::new(),
            springs: self.springs.clone(),
            unavailable: Vec::new(),
        }
    }
}

/// A model's formation allowances, n×n and per pattern entry.
struct Symmetry {
    roundoff: Vec<Vec<f64>>,
    counts: Vec<Vec<usize>>,
    sparse_roundoff: Vec<f64>,
    sparse_counts: Vec<usize>,
}

const BASIS: &str = "K1 sparse_direct test: the adapter's formation allowances";

impl Symmetry {
    fn dense(&self) -> SymmetryEvidence<'_> {
        SymmetryEvidence {
            absolute_roundoff: &self.roundoff,
            operation_counts: &self.counts,
            basis: BASIS,
        }
    }
    fn sparse(&self) -> SparseSymmetryEvidence<'_> {
        SparseSymmetryEvidence {
            absolute_roundoff: &self.sparse_roundoff,
            operation_counts: &self.sparse_counts,
            basis: BASIS,
        }
    }
}

fn boundary(n: usize, prescribed: Vec<(usize, f64)>) -> (Vec<usize>, Vec<(usize, f64)>) {
    let free = (0..n)
        .filter(|d| !prescribed.iter().any(|p| p.0 == *d))
        .collect();
    (free, prescribed)
}

/// A wandering skew chain, anchored at node 0 (settlement `settle` at UY).
fn chain(members: usize, e: f64, settle: f64) -> Model {
    let s = section(e, 0.4 * e);
    let point = |k: usize| {
        let t = k as f64;
        [1.1 * t, 0.37 * t + 0.05 * (t * 0.7).sin(), 0.21 * t]
    };
    let frames = (0..members)
        .map(|k| {
            FrameElement::new(
                node(k, point(k)),
                node(k + 1, point(k + 1)),
                s,
                [0.0, 0.0, 1.0],
            )
            .unwrap()
        })
        .collect();
    let (free, prescribed) = boundary(
        6 * (members + 1),
        (0..6)
            .map(|d| (d, if d == 1 { settle } else { 0.0 }))
            .collect(),
    );
    Model {
        node_count: members + 1,
        frames,
        connectors: Vec::new(),
        blocks: Vec::new(),
        block_formation: Vec::new(),
        springs: Vec::new(),
        free,
        prescribed,
        loads: vec![(6 * members + 1, -1200.0), (6 * members + 5, 35.0)],
    }
}

/// A branched, mostly axis-aligned model (explicit zeros), with an objective
/// connector, an explicit block, ground springs and prescribed motion.
fn tree(e: f64) -> Model {
    let s = section(e, e / 2.6);
    let p = [
        [0.0, 0.0, 0.0],
        [2.0, 0.0, 0.0],
        [4.0, 0.0, 0.0],
        [4.0, 1.5, 0.0],
        [4.0, 1.5, 1.2],
        [2.0, 0.0, -1.0],
        [6.0, 0.3, 0.0],
        [7.5, 0.3, 0.4],
    ];
    let f = |a: usize, b: usize, y: [f64; 3]| {
        FrameElement::new(node(a, p[a]), node(b, p[b]), s, y).unwrap()
    };
    let stand_in = FrameElement::new(node(4, p[4]), node(7, p[7]), s, [0.0, 0.0, 1.0]).unwrap();
    let mut prescribed: Vec<(usize, f64)> = (0..6).map(|d| (d, 0.0)).collect();
    prescribed.push((31, -0.0025));
    prescribed.push((30, 0.0));
    let (free, prescribed) = boundary(48, prescribed);
    Model {
        node_count: 8,
        frames: vec![
            f(0, 1, [0.0, 1.0, 0.0]),
            f(1, 2, [0.0, 1.0, 0.0]),
            f(2, 3, [1.0, 0.0, 0.0]),
            f(3, 4, [1.0, 0.0, 0.0]),
            f(1, 5, [1.0, 0.0, 0.0]),
            f(6, 7, [0.0, 0.0, 1.0]),
        ],
        connectors: vec![chord_connector(node(2, p[2]), node(6, p[6]))],
        blocks: vec![StiffnessBlock {
            node_i: 4,
            node_j: 7,
            stiffness: stand_in.global_stiffness().unwrap(),
        }],
        block_formation: vec![transform_roundoff(
            &stand_in.local_stiffness().unwrap(),
            &stand_in.orientation().unwrap().transformation_matrix(),
        )
        .unwrap()],
        springs: vec![(32, 2.0e5), (43, 7.5e4), (43, 2.5e4), (23, 1.0e3)],
        free,
        prescribed,
        loads: vec![(26, -850.0), (42, 120.0), (21, 14.0), (37, -300.0)],
    }
}

fn models() -> Vec<(String, Model)> {
    let mut out = Vec::new();
    for e in [200e9, 180e9] {
        out.push((format!("chain-12 E={e}"), chain(12, e, 0.0)));
        out.push((format!("chain-12-settle E={e}"), chain(12, e, 0.003)));
        out.push((format!("tree E={e}"), tree(e)));
    }
    out
}

fn normalized(adjacency: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = adjacency.len();
    let mut out = vec![Vec::new(); n];
    for (i, list) in adjacency.into_iter().enumerate() {
        for j in list {
            if i != j {
                out[i].push(j);
                out[j].push(i);
            }
        }
    }
    for list in &mut out {
        list.sort_unstable();
        list.dedup();
    }
    out
}

/// Today's dense-derived sparse sequence on a dense prepared system.
fn today(prepared: PreparedSystem<'_>) -> Result<StructuralSolution, StructuralError> {
    let factor = match factor_structural_ldlt(&prepared) {
        Ok(f) => f,
        Err(error) => return Err(negative_pair_witness(&prepared)?.unwrap_or(error)),
    };
    structural::finish_structural(&factor)
}

// ------------------------------------------------------------------ order

#[test]
fn k1_adjacency_order_and_profile_equal_the_dense_derived_path() {
    for (name, model) in models() {
        let dense_k = model.dense();
        let sparse_k = model.sparse();
        // Global level: the stored nonzero entries have the dense adjacency.
        let entries: Vec<SymmetricMatrixEntry> = (0..sparse_k.dimension())
            .flat_map(|row| {
                sparse_k
                    .row(row)
                    .filter(move |&(col, _)| col <= row)
                    .map(move |(col, value)| SymmetricMatrixEntry { row, col, value })
            })
            .collect();
        assert_eq!(
            normalized(adjacency_from_symmetric_entries(sparse_k.dimension(), &entries).unwrap()),
            normalized(adjacency_from_dense(&dense_k).unwrap()),
            "{name}: global adjacency"
        );
        // Prepared level: the order and the skyline the factors use.
        let force = model.force();
        let symmetry = model.symmetry(&sparse_k);
        let ds = StructuralSystem {
            stiffness: &dense_k,
            force: &force,
            free_dofs: &model.free,
            prescribed: &model.prescribed,
            contributions: None,
            symmetry: Some(symmetry.dense()),
        };
        let ss = SparseStructuralSystem::new(
            &sparse_k,
            &force,
            &model.free,
            &model.prescribed,
            None,
            Some(symmetry.sparse()),
        );
        let dp = structural::prepare_structural(&ds).unwrap();
        let sp = structural::prepare_sparse_structural(&ss).unwrap();
        let adjacency = adjacency_from_dense(dp.matrix()).unwrap();
        let order = reverse_cuthill_mckee(&adjacency).unwrap();
        let profile = SymmetricProfileMatrix::from_dense_with_order(dp.matrix(), &order).unwrap();
        let ordering = order_sparse_structural(&sp).unwrap();
        assert_eq!(ordering.order, order, "{name}: order");
        assert_eq!(
            ordering.first_columns, profile.first_columns,
            "{name}: profile"
        );
        assert_eq!(ordering.profile_entry_count, profile.profile_entry_count());
        assert_eq!(ordering.max_half_bandwidth, profile.max_half_bandwidth());
        // The pattern holds explicit zeros that the dense adjacency skips.
        let explicit_zeros = sp.lower_entries().filter(|e| e.2 == 0.0).count();
        if name.starts_with("tree") {
            assert!(explicit_zeros > 0, "{name}: explicit zeros present");
        }
    }
}

#[test]
fn k1_explicit_zero_by_cancellation_takes_no_part_in_the_order() {
    // A coupling whose contributions cancel exactly (v, -v) is stored as an
    // explicit zero; the order and profile are those of the dense matrix,
    // where the entry is 0.0. A structural rule (zeros included) would add
    // the edge 0-3 and change the RCM order: the value rule keeps today's.
    let mut entries = Vec::new();
    let mut add = |row: usize, col: usize, value: f64| {
        entries.push(StiffnessContribution { row, col, value });
        if row != col {
            entries.push(StiffnessContribution {
                row: col,
                col: row,
                value,
            });
        }
    };
    for i in 0..5 {
        add(i, i, 4.0);
    }
    for i in 0..4 {
        add(i + 1, i, -1.0);
    }
    add(3, 0, 0.625);
    add(3, 0, -0.625);
    let sparse = SparseStiffness::from_contributions(5, &entries).unwrap();
    assert!(sparse.pattern().find(3, 0).is_some());
    assert_eq!(sparse.get(3, 0).to_bits(), 0.0_f64.to_bits());
    let dense = sparse.to_dense();
    let force = [1.0, 0.0, 0.0, 0.0, 2.0];
    let free: Vec<usize> = (0..5).collect();
    let ds = StructuralSystem {
        stiffness: &dense,
        force: &force,
        free_dofs: &free,
        prescribed: &[],
        contributions: None,
        symmetry: None,
    };
    let ss = SparseStructuralSystem::new(&sparse, &force, &free, &[], None, None);
    let dp = structural::prepare_structural(&ds).unwrap();
    let sp = structural::prepare_sparse_structural(&ss).unwrap();
    let order = reverse_cuthill_mckee(&adjacency_from_dense(dp.matrix()).unwrap()).unwrap();
    let ordering = order_sparse_structural(&sp).unwrap();
    assert_eq!(ordering.order, order);
    // The structural alternative would differ (the rule is not vacuous here).
    let structural_entries: Vec<SymmetricMatrixEntry> = sp
        .lower_entries()
        .map(|(row, col, value)| SymmetricMatrixEntry {
            row,
            col,
            value: if value == 0.0 { 1.0 } else { value },
        })
        .collect();
    let structural_order =
        reverse_cuthill_mckee(&adjacency_from_symmetric_entries(5, &structural_entries).unwrap())
            .unwrap();
    assert_ne!(structural_order, order);
    assert_eq!(
        format!("{:?}", today(dp)),
        format!("{:?}", solve_sparse_prepared(sp))
    );
}

// ------------------------------------------------------------------ reports

#[test]
fn k1_pattern_path_report_is_byte_identical_to_the_dense_derived_sparse_path() {
    for (name, mut model) in models() {
        // The free DOFs in ascending order (the product's) and permuted
        // (reversed): the pattern path reads free positions, not DOF indices.
        for permuted in [false, true] {
            if permuted {
                model.free.reverse();
            }
            let name = format!("{name} free-permuted={permuted}");
            let dense_k = model.dense();
            let sparse_k = model.sparse();
            let force = model.force();
            let ledger = model.ledger();
            let contributions = model.contributions();
            let source = model.formation_source();
            let symmetry = model.symmetry(&sparse_k);
            for with_contributions in [false, true] {
                let c = with_contributions.then_some(contributions.as_slice());
                let ctx = format!("{name} contributions={with_contributions}");
                // With no formation allowances the symmetry audit is exact:
                // both paths give the same outcome (for the skew chain, the
                // same first asymmetric pair).
                let bare_ds = StructuralSystem {
                    stiffness: &dense_k,
                    force: &force,
                    free_dofs: &model.free,
                    prescribed: &model.prescribed,
                    contributions: c,
                    symmetry: None,
                };
                let bare_ss = SparseStructuralSystem::new(
                    &sparse_k,
                    &force,
                    &model.free,
                    &model.prescribed,
                    c,
                    None,
                );
                assert_eq!(
                    format!("{:?}", solve_structural_sparse(&bare_ds)),
                    format!("{:?}", solve_sparse_structural(&bare_ss)),
                    "{ctx}: bare"
                );
                // Legacy vector: today's `solve_structural_sparse`.
                let ds = StructuralSystem {
                    stiffness: &dense_k,
                    force: &force,
                    free_dofs: &model.free,
                    prescribed: &model.prescribed,
                    contributions: c,
                    symmetry: Some(symmetry.dense()),
                };
                let ss = SparseStructuralSystem::new(
                    &sparse_k,
                    &force,
                    &model.free,
                    &model.prescribed,
                    c,
                    Some(symmetry.sparse()),
                );
                let legacy_today = solve_structural_sparse(&ds);
                assert!(legacy_today.is_ok(), "{ctx}: {legacy_today:?}");
                assert_eq!(
                    format!("{legacy_today:?}"),
                    format!("{:?}", solve_sparse_structural(&ss)),
                    "{ctx}: legacy"
                );
                // C3-detect: the vector audited against the ledger's terms.
                assert_eq!(
                    format!(
                        "{:?}",
                        today(
                            structural::prepare_structural_with_force_terms(&ds, ledger.terms())
                                .unwrap()
                        )
                    ),
                    format!(
                        "{:?}",
                        solve_sparse_prepared(
                            structural::prepare_sparse_structural_with_force_terms(
                                &ss,
                                ledger.terms()
                            )
                            .unwrap()
                        )
                    ),
                    "{ctx}: C3-detect"
                );
                // Typed (S11-K).
                let dt = StructuralSystem::assembled(
                    &dense_k,
                    &ledger,
                    &model.free,
                    &model.prescribed,
                    c,
                    Some(symmetry.dense()),
                );
                let st = SparseStructuralSystem::assembled(
                    &sparse_k,
                    &ledger,
                    &model.free,
                    &model.prescribed,
                    c,
                    Some(symmetry.sparse()),
                );
                assert_eq!(
                    format!(
                        "{:?}",
                        today(structural::prepare_assembled_structural(&dt).unwrap())
                    ),
                    format!("{:?}", solve_assembled_sparse_structural(&st)),
                    "{ctx}: typed"
                );
                // K-D5: the formation-checked typed solve (FormationCheck records
                // identical, including a demotion where one occurs).
                let dt = StructuralSystem::assembled(
                    &dense_k,
                    &ledger,
                    &model.free,
                    &model.prescribed,
                    c,
                    Some(symmetry.dense()),
                )
                .with_formation_source(&source);
                let st = SparseStructuralSystem::assembled(
                    &sparse_k,
                    &ledger,
                    &model.free,
                    &model.prescribed,
                    c,
                    Some(symmetry.sparse()),
                )
                .with_formation_source(&source);
                assert_eq!(
                    format!(
                        "{:?}",
                        today(structural::prepare_formation_checked_structural(&dt).unwrap())
                    ),
                    format!("{:?}", solve_formation_checked_sparse_structural(&st)),
                    "{ctx}: formation-checked"
                );
            }
        }
    }
}

#[test]
fn k1_krev02_through_the_pattern_path() {
    let matrix = vec![vec![2.0, -1.0], vec![-1.0, 2.0]];
    let sparse = SparseStiffness::from_dense(&matrix).unwrap();
    let force = [0.0, 0.0];
    let free = [1];
    let bc = [(0, 1.0)];
    for coupling in [-1.0, -2.0] {
        let entries = [
            StiffnessContribution {
                row: 0,
                col: 0,
                value: 2.0,
            },
            StiffnessContribution {
                row: 1,
                col: 1,
                value: 2.0,
            },
            StiffnessContribution {
                row: 0,
                col: 1,
                value: coupling,
            },
            StiffnessContribution {
                row: 1,
                col: 0,
                value: coupling,
            },
        ];
        let s = StructuralSystem {
            stiffness: &matrix,
            force: &force,
            free_dofs: &free,
            prescribed: &bc,
            contributions: Some(&entries),
            symmetry: None,
        };
        let ss = SparseStructuralSystem::new(&sparse, &force, &free, &bc, Some(&entries), None);
        let pattern = solve_sparse_structural(&ss);
        assert_eq!(
            format!("{:?}", solve_structural_sparse(&s)),
            format!("{pattern:?}")
        );
        if coupling == -1.0 {
            assert_eq!(pattern.unwrap().displacements, vec![1.0, 0.5]);
        } else {
            assert!(pattern.is_err());
        }
    }
}

// ------------------------------------------------------------------ storage counts

/// RF-LARGE-shaped connectivity (R1's families, invented section): a chain
/// of `n` 3 m members, or a comb (spine of n/2 members, one 3 m branch per
/// spine node, alternating +y and +z), axis-aligned or rotated by
/// Q3 = (1/3)[[1,2,2],[2,1,-2],[-2,2,-1]]; root fixed.
fn large(n: usize, tree: bool, rotated: bool) -> (SparseStiffness, Vec<usize>) {
    let q = |p: [f64; 3]| -> [f64; 3] {
        if !rotated {
            return p;
        }
        [
            (p[0] + 2.0 * p[1] + 2.0 * p[2]) / 3.0,
            (2.0 * p[0] + p[1] - 2.0 * p[2]) / 3.0,
            (-2.0 * p[0] + 2.0 * p[1] - p[2]) / 3.0,
        ]
    };
    let s = section(200e9, 80e9);
    let mut points: Vec<[f64; 3]> = Vec::new();
    let mut members: Vec<(usize, usize, [f64; 3])> = Vec::new();
    if !tree {
        for k in 0..=n {
            points.push([3.0 * k as f64, 0.0, 0.0]);
        }
        for k in 0..n {
            members.push((k, k + 1, [0.0, 0.0, 1.0]));
        }
    } else {
        let spine = n / 2;
        for k in 0..=spine {
            points.push([3.0 * k as f64, 0.0, 0.0]);
        }
        for k in 0..spine {
            members.push((k, k + 1, [0.0, 0.0, 1.0]));
        }
        for k in 1..=(n - spine) {
            let base = points[k.min(spine)];
            let tip = if k % 2 == 1 {
                [base[0], 3.0, 0.0]
            } else {
                [base[0], 0.0, 3.0]
            };
            points.push(tip);
            let y = if k % 2 == 1 {
                [1.0, 0.0, 0.0]
            } else {
                [0.0, 1.0, 0.0]
            };
            members.push((k.min(spine), points.len() - 1, y));
        }
    }
    let frames: Vec<FrameElement> = members
        .iter()
        .map(|&(a, b, y)| {
            FrameElement::new(node(a, q(points[a])), node(b, q(points[b])), s, q(y)).unwrap()
        })
        .collect();
    let k = assemble_sparse_stiffness(
        points.len(),
        &frames,
        &[],
        &[],
        &[],
        &SparseAssemblyOptions::new(),
    )
    .unwrap();
    let free = (6..6 * points.len()).collect();
    (k, free)
}

#[test]
fn k1_storage_counts_of_the_rf_large_chain_and_tree() {
    // Deterministic counts only (no time or memory claim; K6 measures).
    for tree in [false, true] {
        for rotated in [false, true] {
            for n in [10, 100, 1000, 10000] {
                let (k, free) = large(n, tree, rotated);
                let force = vec![0.0; k.dimension()];
                let prescribed: Vec<(usize, f64)> = (0..6).map(|d| (d, 0.0)).collect();
                let ss = SparseStructuralSystem::new(&k, &force, &free, &prescribed, None, None);
                let sp = structural::prepare_sparse_structural(&ss).unwrap();
                let ordering = order_sparse_structural(&sp).unwrap();
                let counts = k.storage_counts();
                let free_lower = sp.lower_entries().count();
                // The skyline holds every nonzero entry; an explicit zero
                // (axis-aligned members) may lie outside it.
                let free_lower_nonzero = sp.lower_entries().filter(|e| e.2 != 0.0).count();
                eprintln!(
                    "k1 storage {} {} n={n}: dofs={} stored={} lower={} free_stored={} free_lower={} free_lower_nonzero={} profile={} half_bandwidth={} dense={}",
                    if tree { "TREE" } else { "CHAIN" },
                    if rotated { "ROT" } else { "AX" },
                    counts.dimension,
                    counts.stored_entries,
                    counts.lower_entries,
                    sp.entry_count(),
                    free_lower,
                    free_lower_nonzero,
                    ordering.profile_entry_count,
                    ordering.max_half_bandwidth,
                    counts.dense_entries,
                );
                assert!(ordering.profile_entry_count >= free_lower_nonzero);
                assert!((counts.stored_entries as u128) < counts.dense_entries);
                // Each member couples two nodes: at most 144 entries per
                // member plus 36 per node, independent of n² growth.
                assert!(counts.stored_entries <= 144 * n + 36 * (n + 1));
            }
        }
    }
}

// ------------------------------------------------------------------ T4-U3 (S10, S-3)

/// T4-I12 round 02 case 22 (U3-KD5-UTM-SKEW-OFFSET-COUPLED) on the pattern
/// path, with no contribution evidence: the connector alone, end j held,
/// loads at end i. At X0 = 0, 5e6 and 7.3e6 m it is not demoted; perturbing
/// only the assembled matrix by δ = 2^-20·max|Ke| at its largest diagonal
/// (k = 3) demotes it.
#[test]
fn k1_connector_case_22_formation_check_on_the_pattern() {
    use open_pipe_stress_frame_kernel::connector::{
        ConnectorAttachment, ObjectiveConnector, ScaledWorkMatrix,
    };
    use open_pipe_stress_frame_kernel::structural::{FormationCheckReason, SolveQuality};
    let (third, two_thirds) = (1.0 / 3.0, 2.0 / 3.0);
    for x0 in [0.0, 5.0e6, 7.3e6] {
        let connector = ObjectiveConnector::new(
            node(0, [x0 + 0.5, -1.25, 2.0]),
            node(1, [x0 + 1.8125, 0.8125, 3.375]),
            ConnectorAttachment::global([0.2, 0.4, -0.2]),
            ConnectorAttachment::global([-0.1, 0.3625, 0.44999999999999996]),
            [
                [third, two_thirds, -two_thirds],
                [two_thirds, third, two_thirds],
                [two_thirds, -two_thirds, -third],
            ],
            ScaledWorkMatrix {
                upper_triangle: [
                    12500.0, 625.0, 0.0, 0.0, 500.0, 0.0, 9375.0, 312.5, 0.0, 0.0, -750.0, 7500.0,
                    250.0, 0.0, 0.0, 800.0, 50.0, 0.0, 900.0, 100.0, 1200.0,
                ],
                translation_scale: 0.25,
            },
            [0.0; 6],
        )
        .unwrap();
        let assembled = assemble_sparse_stiffness(
            2,
            &[],
            &[connector],
            &[],
            &[],
            &SparseAssemblyOptions::new(),
        )
        .unwrap();
        let source = FormationSource {
            node_count: 2,
            connectors: vec![connector],
            ..FormationSource::default()
        };
        let force = [
            1000.0, -2000.0, 1500.0, 300.0, -200.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ];
        let free: Vec<usize> = (0..6).collect();
        let prescribed: Vec<(usize, f64)> = (6..12).map(|d| (d, 0.0)).collect();
        let max = assembled
            .values()
            .iter()
            .fold(0.0_f64, |m, v| m.max(v.abs()));
        for perturbed in [false, true] {
            let mut dense = assembled.to_dense();
            assert_eq!(dense[3][3], max);
            if perturbed {
                dense[3][3] += max * 2f64.powi(-20);
            }
            let k = SparseStiffness::from_dense(&dense).unwrap();
            let system = SparseStructuralSystem::new(&k, &force, &free, &prescribed, None, None);
            let plain = solve_sparse_structural(&system).unwrap();
            assert_eq!(plain.report.quality, SolveQuality::Passed, "X0 {x0}");
            let checked = solve_formation_checked_sparse_structural(
                &SparseStructuralSystem::new(&k, &force, &free, &prescribed, None, None)
                    .with_formation_source(&source),
            )
            .unwrap();
            if perturbed {
                assert_eq!(checked.report.quality, SolveQuality::Sensitive, "X0 {x0}");
                assert_eq!(
                    checked.formation_check.unwrap().reason,
                    FormationCheckReason::Estimate
                );
            } else {
                assert_eq!(checked.formation_check, None, "X0 {x0}");
                assert_eq!(checked.displacements, plain.displacements);
            }
        }
    }
}

/// T4-U3: an objective connector with Q.x along the chord (z global), zero
/// offsets and an uncoupled K at Ls = 1 m, in place of the deleted user element.
fn chord_connector(i: FrameNode, j: FrameNode) -> ObjectiveConnector {
    let d = [
        j.coordinates[0] - i.coordinates[0],
        j.coordinates[1] - i.coordinates[1],
        j.coordinates[2] - i.coordinates[2],
    ];
    assert_eq!(d[2], 0.0);
    let length = (d[0] * d[0] + d[1] * d[1]).sqrt();
    let x = [d[0] / length, d[1] / length];
    ObjectiveConnector::new(
        i,
        j,
        ConnectorAttachment::global([0.0; 3]),
        ConnectorAttachment::global([0.0; 3]),
        [[x[0], -x[1], 0.0], [x[1], x[0], 0.0], [0.0, 0.0, 1.0]],
        ScaledWorkMatrix {
            upper_triangle: [
                3.0e7, 0.0, 0.0, 0.0, 0.0, 0.0, 4.0e6, 0.0, 0.0, 0.0, 0.0, 4.0e6, 0.0, 0.0, 0.0,
                1.5e5, 0.0, 0.0, 2.5e5, 0.0, 2.5e5,
            ],
            translation_scale: 1.0,
        },
        [0.0; 6],
    )
    .unwrap()
}
