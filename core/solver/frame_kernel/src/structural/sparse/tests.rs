//! K1 kernel tests. Invented
//! inputs only: every section, stiffness, spring and load below is stated here.
//!
//! The parity basis is bitwise: the pattern path must reproduce the dense
//! path's values, errors and `Debug` bytes, never within a tolerance. The
//! order-dependent pattern path (RCM) lives in `sparse_direct`; here each
//! comparison fixes one order for both paths.
use super::*;
use crate::load_ledger::LoadLedger;
use crate::{
    assemble_global_stiffness_with_connectors, reduce_assembled_system,
    reduce_assembled_system_with_prescribed_displacements, FrameNode, FrameSection,
};

// ------------------------------------------------------------------ models

/// An invented pipe section (OD 0.2 m, wall 0.01 m) with the given moduli.
fn section(e: f64, g: f64) -> FrameSection {
    let (od, id) = (0.2_f64, 0.18_f64);
    let area = std::f64::consts::PI * (od * od - id * id) / 4.0;
    let inertia = std::f64::consts::PI * (od.powi(4) - id.powi(4)) / 64.0;
    FrameSection::new(e, g, area, inertia, inertia, 2.0 * inertia).unwrap()
}

/// A generated product-shaped model (invented): frames, objective connectors,
/// explicit blocks (the stand-in for realized curved bends, which live in
/// `curved_bend`; the adapter tests use real ones), springs and boundary.
#[derive(Debug, Clone)]
struct Model {
    node_count: usize,
    frames: Vec<FrameElement>,
    connectors: Vec<ObjectiveConnector>,
    blocks: Vec<StiffnessBlock>,
    /// Each block's formation bounds and operation counts (the
    /// `transform_roundoff` of the element the block was formed from).
    block_formation: Vec<TransformationRoundoff>,
    springs: Vec<(usize, f64)>,
    free: Vec<usize>,
    prescribed: Vec<(usize, f64)>,
    loads: Vec<(usize, f64)>,
}

impl Model {
    /// The formation allowances and operation counts (n×n) in the form the
    /// structural adapter's `AssemblyEvidence::new` records them: each
    /// element's `transform_roundoff` bounds, its counts plus one per scatter,
    /// one count per spring, and then `gamma(scatter count) * sum |value|` of
    /// each entry's contributions. A skew member's global matrix is symmetric
    /// only within these allowances (as in the product, which always supplies
    /// them).
    fn dense_symmetry(&self) -> (Vec<Vec<f64>>, Vec<Vec<usize>>) {
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
        (roundoff, counts)
    }
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
    /// The product's dense order: frames and connectors
    /// (`assemble_global_stiffness_with_connectors`), then the explicit
    /// blocks, then the springs, each `+=` on the dense matrix.
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
    /// The contributions in the dense order (both triangles), as the
    /// adapter's evidence lists them.
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
    fn set_boundary(&mut self, prescribed: Vec<(usize, f64)>) {
        let n = self.node_count * 6;
        self.free = (0..n)
            .filter(|d| !prescribed.iter().any(|p| p.0 == *d))
            .collect();
        self.prescribed = prescribed;
    }
}

fn node(index: usize, p: [f64; 3]) -> FrameNode {
    FrameNode::new(index, p).unwrap()
}

/// A skewed, slightly wandering chain of `members` frames (invented
/// geometry), anchored at node 0 with a settlement `settle` at node 0's UY.
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
    let mut model = Model {
        node_count: members + 1,
        frames,
        connectors: Vec::new(),
        blocks: Vec::new(),
        block_formation: Vec::new(),
        springs: Vec::new(),
        free: Vec::new(),
        prescribed: Vec::new(),
        loads: vec![(6 * members + 1, -1200.0), (6 * members + 5, 35.0)],
    };
    model.set_boundary(
        (0..6)
            .map(|d| (d, if d == 1 { settle } else { 0.0 }))
            .collect(),
    );
    model
}

/// A branched model (invented): an axis-aligned trunk with branches, one
/// objective connector (an expansion joint), one explicit block, ground springs
/// (two on one DOF), and prescribed motion. Axis-aligned members leave
/// explicit zeros in the pattern.
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
    let frames = vec![
        f(0, 1, [0.0, 1.0, 0.0]),
        f(1, 2, [0.0, 1.0, 0.0]),
        f(2, 3, [1.0, 0.0, 0.0]),
        f(3, 4, [1.0, 0.0, 0.0]),
        f(1, 5, [1.0, 0.0, 0.0]),
        f(6, 7, [0.0, 0.0, 1.0]),
    ];
    // T4-U3: an objective connector between nodes 2 and 6 (Q.x along the
    // chord, z global; an uncoupled K at Ls = 1 m).
    let chord = [p[6][0] - p[2][0], p[6][1] - p[2][1], p[6][2] - p[2][2]];
    let length = (chord[0] * chord[0] + chord[1] * chord[1] + chord[2] * chord[2]).sqrt();
    let x = [chord[0] / length, chord[1] / length, chord[2] / length];
    let axes = [[x[0], -x[1], 0.0], [x[1], x[0], 0.0], [0.0, 0.0, 1.0]];
    let connectors = vec![ObjectiveConnector::new(
        node(2, p[2]),
        node(6, p[6]),
        crate::connector::ConnectorAttachment::global([0.0; 3]),
        crate::connector::ConnectorAttachment::global([0.0; 3]),
        axes,
        crate::connector::ScaledWorkMatrix {
            upper_triangle: [
                3.0e7, 0.0, 0.0, 0.0, 0.0, 0.0, 4.0e6, 0.0, 0.0, 0.0, 0.0, 4.0e6, 0.0, 0.0, 0.0,
                1.5e5, 0.0, 0.0, 2.5e5, 0.0, 2.5e5,
            ],
            translation_scale: 1.0,
        },
        [0.0; 6],
    )
    .unwrap()];
    // An explicit block (standing in for a realized bend's global matrix):
    // a frame's matrix between nodes 4 and 7, added after the frames and connectors.
    let stand_in = FrameElement::new(node(4, p[4]), node(7, p[7]), s, [0.0, 0.0, 1.0]).unwrap();
    let blocks = vec![StiffnessBlock {
        node_i: 4,
        node_j: 7,
        stiffness: stand_in.global_stiffness().unwrap(),
    }];
    let block_formation = vec![transform_roundoff(
        &stand_in.local_stiffness().unwrap(),
        &stand_in.orientation().unwrap().transformation_matrix(),
    )
    .unwrap()];
    let springs = vec![
        (6 * 5 + 2, 2.0e5),
        (6 * 7 + 1, 7.5e4),
        (6 * 7 + 1, 2.5e4),
        (6 * 3 + 5, 1.0e3),
    ];
    let mut model = Model {
        node_count: 8,
        frames,
        connectors,
        blocks,
        block_formation,
        springs,
        free: Vec::new(),
        prescribed: Vec::new(),
        loads: vec![
            (6 * 4 + 2, -850.0),
            (6 * 7, 120.0),
            (6 * 3 + 3, 14.0),
            (6 * 6 + 1, -300.0),
        ],
    };
    let mut prescribed: Vec<(usize, f64)> = (0..6).map(|d| (d, 0.0)).collect();
    prescribed.push((6 * 5 + 1, -0.0025));
    prescribed.push((6 * 5, 0.0));
    model.set_boundary(prescribed);
    model
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

fn bits(values: &[f64]) -> Vec<u64> {
    values.iter().map(|v| v.to_bits()).collect()
}

fn dense_bits(k: &[Vec<f64>]) -> Vec<Vec<u64>> {
    k.iter().map(|row| bits(row)).collect()
}

// ------------------------------------------------------------------ pattern

#[test]
fn k1_pattern_is_a_function_of_the_positions_only() {
    for (name, model) in models() {
        let k = model.sparse();
        let contributions = model.contributions();
        let from_positions = SparsePattern::from_positions(
            model.node_count * 6,
            contributions.iter().map(|c| (c.row, c.col)),
        )
        .unwrap();
        assert_eq!(k.pattern(), &from_positions, "{name}");
        // Reversed element order and repeated positions give the same pattern
        // (mutation 10: the order depends on neither labels nor input order).
        let reversed = SparsePattern::from_positions(
            model.node_count * 6,
            contributions
                .iter()
                .rev()
                .chain(&contributions)
                .map(|c| (c.row, c.col)),
        )
        .unwrap();
        assert_eq!(k.pattern(), &reversed, "{name}");
        let mut permuted = model.clone();
        permuted.frames.reverse();
        assert_eq!(permuted.sparse().pattern(), k.pattern(), "{name}");
        // Structure: rows ascending, transposes involutive.
        let p = k.pattern();
        for row in 0..p.dimension() {
            assert!(p.row(row).windows(2).all(|w| w[0] < w[1]), "{name}");
            for index in p.row_range(row) {
                let t = p.transpose(index);
                assert_eq!(p.column(t), row);
                assert_eq!(p.transpose(t), index);
            }
        }
    }
}

#[test]
fn k1_coalesced_values_are_bit_identical_to_the_dense_assembly() {
    for (name, model) in models() {
        let dense = model.dense();
        let k = model.sparse();
        assert_eq!(dense_bits(&k.to_dense()), dense_bits(&dense), "{name}");
        // Every stored entry is the dense entry, and every entry outside the
        // pattern is +0.0 in the dense matrix.
        for (i, row) in dense.iter().enumerate() {
            for (j, &value) in row.iter().enumerate() {
                match k.pattern().find(i, j) {
                    Some(index) => assert_eq!(k.values()[index].to_bits(), value.to_bits()),
                    None => assert_eq!(value.to_bits(), 0, "{name} ({i},{j})"),
                }
            }
        }
        // The same values from the contribution list (the evidence's order).
        let coalesced =
            SparseStiffness::from_contributions(model.node_count * 6, &model.contributions())
                .unwrap();
        assert_eq!(bits(coalesced.values()), bits(k.values()), "{name}");
        // Explicit zeros are present in the axis-aligned tree.
        if name.starts_with("tree") {
            assert!(
                k.values().iter().any(|v| *v == 0.0),
                "{name}: explicit zeros"
            );
        }
    }
    // Two modulus bases give two assemblies on the same pattern, each
    // bit-identical to its own dense matrix and different from the other.
    let (a, b) = (tree(200e9), tree(180e9));
    assert_ne!(bits(a.sparse().values()), bits(b.sparse().values()));
    assert_eq!(a.sparse().pattern(), b.sparse().pattern());
}

#[test]
fn k1_assembly_refusals_match_the_dense_assembly() {
    // A frame outside the model: the same error for the same first element.
    let model = tree(200e9);
    let dense = assemble_global_stiffness_with_connectors(7, &model.frames, &model.connectors);
    let sparse = assemble_sparse_stiffness(
        7,
        &model.frames,
        &model.connectors,
        &[],
        &[],
        &SparseAssemblyOptions::new(),
    );
    assert_eq!(sparse.unwrap_err(), dense.unwrap_err());
    // A non-finite coalesced entry after the frames and connectors: two collinear
    // members whose axial stiffness EA/L = 1e300 * 1e8 / 1 = 1e308 each meet
    // at node 1 (every formed term is finite; 12 E I / L^3 = 1.2e298).
    let s = FrameSection::new(1e300, 1.0, 1e8, 1e-3, 1e-3, 2e-3).unwrap();
    let frames = [
        FrameElement::new(
            node(0, [0.0; 3]),
            node(1, [1.0, 0.0, 0.0]),
            s,
            [0.0, 1.0, 0.0],
        )
        .unwrap(),
        FrameElement::new(
            node(1, [1.0, 0.0, 0.0]),
            node(2, [2.0, 0.0, 0.0]),
            s,
            [0.0, 1.0, 0.0],
        )
        .unwrap(),
    ];
    let dense = assemble_global_stiffness_with_connectors(3, &frames, &[]).unwrap_err();
    let sparse =
        assemble_sparse_stiffness(3, &frames, &[], &[], &[], &SparseAssemblyOptions::new())
            .unwrap_err();
    assert!(
        matches!(
            dense,
            FrameKernelError::NonFiniteInput {
                name: "assembled stiffness",
                ..
            }
        ),
        "{dense:?}"
    );
    assert_eq!(format!("{sparse:?}"), format!("{dense:?}"));
    // A spring outside the model is refused (the dense `+=` would panic).
    assert!(matches!(
        assemble_sparse_stiffness(
            3,
            &[],
            &[],
            &[],
            &[(18, 1.0)],
            &SparseAssemblyOptions::new()
        ),
        Err(FrameKernelError::InvalidNodeIndex { .. })
    ));
}

// ------------------------------------------------------------------ gate

/// The dense system and its sparse sibling on the same values.
struct Pair {
    dense: Vec<Vec<f64>>,
    sparse: SparseStiffness,
    force: Vec<f64>,
    ledger: AssembledForce,
    contributions: Vec<StiffnessContribution>,
    free: Vec<usize>,
    prescribed: Vec<(usize, f64)>,
    /// The formation allowances and counts, n×n and per pattern entry.
    roundoff: Vec<Vec<f64>>,
    counts: Vec<Vec<usize>>,
    sparse_roundoff: Vec<f64>,
    sparse_counts: Vec<usize>,
}

const BASIS: &str = "K1 kernel test: the adapter's formation allowances";

impl Pair {
    fn new(model: &Model) -> Self {
        let sparse = model.sparse();
        let (roundoff, counts) = model.dense_symmetry();
        let pattern = sparse.pattern();
        let per_entry = |row: usize| {
            pattern
                .row_range(row)
                .map(move |k| (row, pattern.column(k)))
        };
        let (sparse_roundoff, sparse_counts) = (0..pattern.dimension())
            .flat_map(per_entry)
            .map(|(i, j)| (roundoff[i][j], counts[i][j]))
            .unzip();
        Self {
            dense: model.dense(),
            force: model.force(),
            ledger: model.ledger(),
            contributions: model.contributions(),
            free: model.free.clone(),
            prescribed: model.prescribed.clone(),
            roundoff,
            counts,
            sparse_roundoff,
            sparse_counts,
            sparse,
        }
    }
    fn dense_evidence(&self) -> SymmetryEvidence<'_> {
        SymmetryEvidence {
            absolute_roundoff: &self.roundoff,
            operation_counts: &self.counts,
            basis: BASIS,
        }
    }
    fn sparse_evidence(&self) -> SparseSymmetryEvidence<'_> {
        SparseSymmetryEvidence {
            absolute_roundoff: &self.sparse_roundoff,
            operation_counts: &self.sparse_counts,
            basis: BASIS,
        }
    }
    fn dense_system(&self, contributions: bool) -> StructuralSystem<'_> {
        StructuralSystem {
            stiffness: &self.dense,
            force: &self.force,
            free_dofs: &self.free,
            prescribed: &self.prescribed,
            contributions: contributions.then_some(self.contributions.as_slice()),
            contribution_groups: None,
            symmetry: Some(self.dense_evidence()),
        }
    }
    fn sparse_system(&self, contributions: bool) -> SparseStructuralSystem<'_> {
        SparseStructuralSystem::new(
            &self.sparse,
            &self.force,
            &self.free,
            &self.prescribed,
            contributions.then_some(self.contributions.as_slice()),
            Some(self.sparse_evidence()),
        )
    }
}

/// The dense prepared matrix's skyline in the natural order (the first
/// nonzero column of each row), for a fixed-order comparison.
fn natural_profile(prepared: &PreparedSystem<'_>) -> (Vec<usize>, Vec<usize>) {
    let n = prepared.matrix.len();
    let order: Vec<usize> = (0..n).collect();
    let first = (0..n)
        .map(|i| (0..i).find(|&j| prepared.matrix[i][j] != 0.0).unwrap_or(i))
        .collect();
    (order, first)
}

/// The sparse prepared system equals the dense one entry for entry.
fn assert_prepared_equal(dense: &PreparedSystem<'_>, sparse: &SparsePreparedSystem<'_>, ctx: &str) {
    assert_eq!(bits(&dense.rhs), bits(&sparse.rhs), "{ctx}: rhs");
    assert_eq!(dense.scale_exponents, sparse.scale_exponents, "{ctx}");
    let n = dense.matrix.len();
    let mut rebuilt = vec![vec![0.0; n]; n];
    for (row, col, value) in sparse.lower_entries() {
        rebuilt[row][col] = value;
        rebuilt[col][row] = value;
    }
    assert_eq!(
        dense_bits(&rebuilt),
        dense_bits(&dense.matrix),
        "{ctx}: matrix"
    );
    assert_eq!(
        dense.assembly_relative_perturbation_estimate.to_bits(),
        sparse.assembly_relative_perturbation_estimate.to_bits(),
        "{ctx}"
    );
    assert_eq!(
        dense.assembly_load_perturbation_estimate.to_bits(),
        sparse.assembly_load_perturbation_estimate.to_bits(),
        "{ctx}"
    );
    assert_eq!(
        format!("{:?}", dense.contribution_rounding),
        format!("{:?}", sparse.contribution_rounding),
        "{ctx}"
    );
    assert_eq!(
        dense.symmetry_projection_performed,
        sparse.symmetry_projection_performed
    );
    assert_eq!(
        dense.maximum_scaled_skew.to_bits(),
        sparse.maximum_scaled_skew.to_bits()
    );
}

/// Both paths with the same order: preparation, factor and completion. A
/// preparation refused by both (with the same error) gives `None`.
fn solve_both(
    dense: Result<PreparedSystem<'_>, StructuralError>,
    sparse: Result<SparsePreparedSystem<'_>, StructuralError>,
    ctx: &str,
) -> Option<Result<StructuralSolution, StructuralError>> {
    let (dense, sparse) = match (dense, sparse) {
        (Ok(d), Ok(s)) => (d, s),
        (d, s) => {
            assert_eq!(
                format!("{:?}", d.err()),
                format!("{:?}", s.err()),
                "{ctx}: preparation"
            );
            return None;
        }
    };
    assert_prepared_equal(&dense, &sparse, ctx);
    let (order, first) = natural_profile(&dense);
    let d = factor_structural_profile(&dense, &order, &first).and_then(|f| finish_structural(&f));
    let s = factor_sparse_structural_profile(&sparse, &order, &first)
        .and_then(|f| finish_sparse_structural(&f));
    assert_eq!(format!("{d:?}"), format!("{s:?}"), "{ctx}: solution");
    Some(d)
}

#[test]
fn k1_gate_stages_are_byte_identical_for_a_fixed_order() {
    for (name, model) in models() {
        let pair = Pair::new(&model);
        for contributions in [false, true] {
            let ctx = format!("{name} contributions={contributions}");
            // Legacy vector.
            let ds = pair.dense_system(contributions);
            let ss = pair.sparse_system(contributions);
            let solved = solve_both(
                prepare_structural(&ds),
                prepare_sparse_structural(&ss),
                &ctx,
            );
            assert!(matches!(solved, Some(Ok(_))), "{ctx}: {solved:?}");
            // Typed (S11-K KS1/KS3 on the ledger).
            let dt = StructuralSystem::assembled(
                &pair.dense,
                &pair.ledger,
                &pair.free,
                &pair.prescribed,
                contributions.then_some(pair.contributions.as_slice()),
                Some(pair.dense_evidence()),
            );
            let st = SparseStructuralSystem::assembled(
                &pair.sparse,
                &pair.ledger,
                &pair.free,
                &pair.prescribed,
                contributions.then_some(pair.contributions.as_slice()),
                Some(pair.sparse_evidence()),
            );
            let solved = solve_both(
                prepare_assembled_structural(&dt),
                prepare_assembled_sparse_structural(&st),
                &format!("{ctx} typed"),
            );
            assert!(matches!(solved, Some(Ok(_))), "{ctx} typed: {solved:?}");
            // C3-detect (the load audit against identified terms).
            let terms = pair.ledger.terms();
            let solved = solve_both(
                prepare_structural_with_force_terms(&ds, terms),
                prepare_sparse_structural_with_force_terms(&ss, terms),
                &format!("{ctx} terms"),
            );
            assert!(matches!(solved, Some(Ok(_))), "{ctx} terms: {solved:?}");
            // With no formation allowances, the symmetry audit runs on exact
            // equality: both representations give the same outcome, and for
            // the skew chain the same first asymmetric pair.
            let bare_dense = StructuralSystem {
                symmetry: None,
                ..pair.dense_system(contributions)
            };
            let bare_sparse = SparseStructuralSystem::new(
                &pair.sparse,
                &pair.force,
                &pair.free,
                &pair.prescribed,
                contributions.then_some(pair.contributions.as_slice()),
                None,
            );
            let bare = solve_both(
                prepare_structural(&bare_dense),
                prepare_sparse_structural(&bare_sparse),
                &format!("{ctx} bare"),
            );
            if name.starts_with("chain") {
                assert!(bare.is_none(), "{ctx} bare: {bare:?}");
                assert!(matches!(
                    prepare_sparse_structural(&bare_sparse),
                    Err(StructuralError::Asymmetric { .. })
                ));
            }
        }
    }
}

#[test]
fn k1_s11k_audit_and_ks1_ks3_have_identical_outcomes_in_both_representations() {
    // A cancelling ledger at the tip (G, n, -G) solved as its folded vector:
    // the load audit flags the row in both representations identically (S11
    // section 5.1). The chain is unsettled here: a settlement's large K_ij u_j
    // terms would enter the row's denominator d_i and hide the loss. The typed
    // solve with a nonzero settlement (below) exercises KS1 (exact rhs) and
    // KS3 (exact residual numerator) on coupled rows.
    let mut model = chain(6, 200e9, 0.0);
    model.loads.clear();
    let tip = 6 * 6 + 1;
    let mut ledger = LoadLedger::new();
    // G = 1e12: the fold's loss (about 2.4e-5 N) is well above the audit's
    // target relative to the row's d_i on this six-member chain; at G = 1e8
    // (about 3e-9 N) it is within the target and the row is not flagged.
    ledger.push("load:big", tip, 1e12);
    ledger.push("load:net", tip, 0.3);
    ledger.push("load:cancel", tip, -1e12);
    let force = ledger.finish(model.node_count * 6).unwrap();
    let mut folded = vec![0.0_f64; model.node_count * 6];
    folded[tip] = (1e12 + 0.3) - 1e12;
    assert_ne!(
        folded[tip].to_bits(),
        force.values()[tip].to_bits(),
        "precondition: the fold differs from the exact net"
    );
    let pair = Pair::new(&model);
    let ds = StructuralSystem {
        stiffness: &pair.dense,
        force: &folded,
        free_dofs: &pair.free,
        prescribed: &pair.prescribed,
        contributions: Some(&pair.contributions),
        contribution_groups: None,
        symmetry: Some(pair.dense_evidence()),
    };
    let ss = SparseStructuralSystem::new(
        &pair.sparse,
        &folded,
        &pair.free,
        &pair.prescribed,
        Some(&pair.contributions),
        Some(pair.sparse_evidence()),
    );
    let detected = solve_both(
        prepare_structural_with_force_terms(&ds, force.terms()),
        prepare_sparse_structural_with_force_terms(&ss, force.terms()),
        "C3-detect",
    )
    .unwrap()
    .unwrap();
    assert_eq!(detected.report.quality, SolveQuality::Sensitive);
    assert!(detected.load_fidelity.is_some());
    // The public audits agree row for row.
    assert_eq!(
        format!(
            "{:?}",
            audit_load_fidelity(&ds, &detected.displacements, force.terms())
        ),
        format!(
            "{:?}",
            audit_sparse_load_fidelity(&ss, &detected.displacements, force.terms())
        )
    );
    // KS1-KS3: the typed solve with the settlement.
    let mut settled = chain(6, 200e9, 0.004);
    settled.loads.clear();
    let pair = Pair::new(&settled);
    assert!(pair.prescribed.iter().any(|&(_, g)| g != 0.0));
    let dt = StructuralSystem::assembled(
        &pair.dense,
        &force,
        &pair.free,
        &pair.prescribed,
        Some(&pair.contributions),
        Some(pair.dense_evidence()),
    );
    let st = SparseStructuralSystem::assembled(
        &pair.sparse,
        &force,
        &pair.free,
        &pair.prescribed,
        Some(&pair.contributions),
        Some(pair.sparse_evidence()),
    );
    let typed = solve_both(
        prepare_assembled_structural(&dt),
        prepare_assembled_sparse_structural(&st),
        "KS1-KS3",
    )
    .unwrap()
    .unwrap();
    assert!(typed.report.residual_rows.iter().all(|r| r.passed));
}

#[test]
fn k1_legacy_zero_product_rows_keep_the_dense_zero_sign_and_range_refusal() {
    // Rows with no nonzero prescribed product: every `b - K g` subtracts a
    // signed zero. The dense fold turns b = -0.0 into +0.0 exactly when some
    // product is -0.0, and refuses a subnormal b at the first subtraction.
    let k = vec![
        vec![4.0, 0.0, 1.5],
        vec![0.0, 4.0, 0.0],
        vec![1.5, 0.0, 4.0],
    ];
    let sparse = SparseStiffness::from_dense(&k).unwrap();
    let tiny = f64::MIN_POSITIVE / 4.0;
    let mut outcomes = std::collections::BTreeSet::new();
    for force0 in [-0.0, 0.0, 1.0, tiny, -tiny] {
        for g1 in [0.0, -0.0] {
            for g2 in [0.0, -0.0] {
                // free: 0; prescribed: 1 (absent coupling), 2 (stored coupling).
                let force = [force0, 0.0, 0.0];
                let free = [0];
                let prescribed = [(1, g1), (2, g2)];
                let ds = StructuralSystem {
                    stiffness: &k,
                    force: &force,
                    free_dofs: &free,
                    prescribed: &prescribed,
                    contributions: None,
                    contribution_groups: None,
                    symmetry: None,
                };
                let ss =
                    SparseStructuralSystem::new(&sparse, &force, &free, &prescribed, None, None);
                let d = prepare_structural(&ds);
                let s = prepare_sparse_structural(&ss);
                let ctx = format!("f={force0:?} g=({g1:?},{g2:?})");
                match (&d, &s) {
                    (Ok(d), Ok(s)) => {
                        assert_eq!(bits(&d.rhs), bits(&s.rhs), "{ctx}");
                        outcomes.insert(d.rhs[0].to_bits());
                    }
                    _ => assert_eq!(
                        format!("{:?}", d.as_ref().err()),
                        format!("{:?}", s.as_ref().err()),
                        "{ctx}"
                    ),
                }
            }
        }
    }
    // Not vacuous: both signs of a zero right-hand side occur.
    assert!(outcomes.contains(&0.0_f64.to_bits()) && outcomes.contains(&(-0.0_f64).to_bits()));
}

/// RT-A3 P (K6/K1 for the nonlinear loop): the option-(c) binary64 binding on
/// the pattern. The preparation equals `prepare_structural_binary64` entry for
/// entry (rhs bits, exponents, matrix, audit, projection) and the fixed-order
/// factor and completion equal the dense ones in `Debug`, on every model
/// (settled rows fold nonzero prescribed products in binary64; the tree's
/// prescribed list is not in DOF order), with and without contributions, with
/// and without formation allowances.
#[test]
fn p_binary64_binding_is_byte_identical_for_a_fixed_order() {
    for (name, model) in models() {
        let pair = Pair::new(&model);
        for contributions in [false, true] {
            let ctx = format!("{name} contributions={contributions} binary64");
            let ds = pair.dense_system(contributions);
            let ss = pair.sparse_system(contributions);
            let solved = solve_both(
                prepare_structural_binary64(&ds),
                prepare_sparse_structural_binary64(&ss),
                &ctx,
            );
            assert!(matches!(solved, Some(Ok(_))), "{ctx}: {solved:?}");
            let bare_dense = StructuralSystem {
                symmetry: None,
                ..pair.dense_system(contributions)
            };
            let bare_sparse = SparseStructuralSystem::new(
                &pair.sparse,
                &pair.force,
                &pair.free,
                &pair.prescribed,
                contributions.then_some(pair.contributions.as_slice()),
                None,
            );
            solve_both(
                prepare_structural_binary64(&bare_dense),
                prepare_sparse_structural_binary64(&bare_sparse),
                &format!("{ctx} bare"),
            );
            // The residual entry (product equilibrium's) agrees row for row.
            if let Some(Ok(solution)) = solved {
                assert_eq!(
                    format!("{:?}", evaluate_original_residual(&ds, &solution.displacements)),
                    format!(
                        "{:?}",
                        evaluate_sparse_original_residual(&ss, &solution.displacements)
                    ),
                    "{ctx}: residual"
                );
            }
        }
    }
}

/// RT-A3 P: the binary64 fold is the dense one term for term, so the dense
/// signed-zero rule, its subnormal refusal and the order of a cancelling fold
/// all carry over, on rows with stored and unstored prescribed columns and
/// prescribed lists in and out of DOF order.
#[test]
fn p_binary64_rows_keep_the_dense_fold_sign_range_and_order() {
    let k = vec![
        vec![4.0, 0.0, 1.5, 3.0],
        vec![0.0, 4.0, 0.0, 0.0],
        vec![1.5, 0.0, 4.0, 0.0],
        vec![3.0, 0.0, 0.0, 4.0],
    ];
    let sparse = SparseStiffness::from_dense(&k).unwrap();
    let tiny = f64::MIN_POSITIVE / 4.0;
    let mut outcomes = std::collections::BTreeSet::new();
    let mut refusals = 0;
    for force0 in [-0.0, 0.0, 1.0, tiny, -tiny, 1e300] {
        for g1 in [0.0, -0.0, 1e-300] {
            for g2 in [0.0, -0.0, 0.1, -1e300] {
                for g3 in [0.0, -0.0, 1.0 / 3.0] {
                    for order in [[1, 2, 3], [3, 1, 2], [2, 3, 1]] {
                        let values = [0.0, g1, g2, g3];
                        let prescribed: Vec<(usize, f64)> =
                            order.iter().map(|&d| (d, values[d])).collect();
                        let force = [force0, 0.0, 0.0, 0.0];
                        let free = [0];
                        let ds = StructuralSystem {
                            stiffness: &k,
                            force: &force,
                            free_dofs: &free,
                            prescribed: &prescribed,
                            contributions: None,
                            contribution_groups: None,
                            symmetry: None,
                        };
                        let ss = SparseStructuralSystem::new(
                            &sparse, &force, &free, &prescribed, None, None,
                        );
                        let d = prepare_structural_binary64(&ds);
                        let s = prepare_sparse_structural_binary64(&ss);
                        let ctx = format!("f={force0:?} g=({g1:?},{g2:?},{g3:?}) order={order:?}");
                        match (&d, &s) {
                            (Ok(d), Ok(s)) => {
                                assert_eq!(bits(&d.rhs), bits(&s.rhs), "{ctx}");
                                outcomes.insert(d.rhs[0].to_bits());
                            }
                            _ => {
                                refusals += 1;
                                assert_eq!(
                                    format!("{:?}", d.as_ref().err()),
                                    format!("{:?}", s.as_ref().err()),
                                    "{ctx}"
                                );
                            }
                        }
                        let dense = crate::reduce_system_with_prescribed_displacements_binary64(
                            &k,
                            &force,
                            &prescribed.iter().map(|p| p.0).collect::<Vec<_>>(),
                            &prescribed.iter().map(|p| p.1).collect::<Vec<_>>(),
                        );
                        let pattern = reduce_sparse_system_with_prescribed_displacements_binary64(
                            &sparse,
                            &force,
                            &prescribed.iter().map(|p| p.0).collect::<Vec<_>>(),
                            &prescribed.iter().map(|p| p.1).collect::<Vec<_>>(),
                        );
                        match (dense, pattern) {
                            (Ok(d), Ok(p)) => {
                                assert_eq!(d.free_dofs, p.free_dofs, "{ctx}");
                                assert_eq!(bits(&d.force), bits(&p.force), "{ctx}: reduction");
                            }
                            (d, p) => assert_eq!(
                                format!("{:?}", d.err()),
                                format!("{:?}", p.err()),
                                "{ctx}: reduction"
                            ),
                        }
                    }
                }
            }
        }
    }
    // Not vacuous: both signs of a zero right-hand side occur, and so do refusals.
    assert!(outcomes.contains(&0.0_f64.to_bits()) && outcomes.contains(&(-0.0_f64).to_bits()));
    assert!(refusals > 0);
}

/// RT-A3 P: the binary64 reduction's checks, in the dense order, with the
/// dense errors.
#[test]
fn p_binary64_reduction_refuses_as_the_dense_reduction() {
    let model = chain(3, 200e9, 0.004);
    let pair = Pair::new(&model);
    let dofs: Vec<usize> = pair.prescribed.iter().map(|p| p.0).collect();
    let values: Vec<f64> = pair.prescribed.iter().map(|p| p.1).collect();
    let both = |force: &[f64], dofs: &[usize], values: &[f64], ctx: &str| {
        let d = crate::reduce_system_with_prescribed_displacements_binary64(
            &pair.dense, force, dofs, values,
        );
        let s = reduce_sparse_system_with_prescribed_displacements_binary64(
            &pair.sparse, force, dofs, values,
        );
        match (d, s) {
            (Ok(d), Ok(s)) => {
                assert_eq!(d.free_dofs, s.free_dofs, "{ctx}");
                assert_eq!(bits(&d.force), bits(&s.force), "{ctx}");
                None
            }
            (d, s) => {
                let d = format!("{:?}", d.err());
                assert_eq!(d, format!("{:?}", s.err()), "{ctx}");
                Some(d)
            }
        }
    };
    assert_eq!(both(&pair.force, &dofs, &values, "ordinary"), None);
    let mut bad = pair.force.clone();
    bad[7] = f64::NAN;
    assert!(both(&bad, &dofs, &values, "nan force").is_some());
    assert!(both(&pair.force[1..], &dofs, &values, "short force").is_some());
    assert!(both(&pair.force, &dofs, &values[1..], "short values").is_some());
    let mut inf = values.clone();
    inf[0] = f64::INFINITY;
    assert!(both(&pair.force, &dofs, &inf, "infinite value").is_some());
    let mut out = dofs.clone();
    out[0] = 10_000;
    assert!(both(&pair.force, &out, &values, "out of range").is_some());
    let mut repeated = dofs.clone();
    repeated[1] = repeated[0];
    assert!(both(&pair.force, &repeated, &values, "repeated").is_some());
    let mut huge = values.clone();
    huge[1] = f64::MAX;
    assert!(both(&pair.force, &dofs, &huge, "adjusted overflow").is_some());
}

/// RT-A3 P step 3: a kept factor completes further right-hand sides of the
/// same prepared matrix exactly as separate solves do (the SD RCM order and
/// skyline, here the natural order, are functions of the matrix), and refuses
/// a prepared system whose matrix differs.
#[test]
fn p_a_shared_factor_completes_other_right_hand_sides_as_separate_solves() {
    for (name, model) in models() {
        let pair = Pair::new(&model);
        let first = pair.sparse_system(true);
        let prepared = prepare_sparse_structural_binary64(&first).unwrap();
        let dense_system = pair.dense_system(true);
        let (order, first_columns) =
            natural_profile(&prepare_structural_binary64(&dense_system).unwrap());
        let factor = factor_sparse_structural_profile(&prepared, &order, &first_columns).unwrap();
        let separate_first = finish_sparse_structural(&factor).unwrap();
        let shared = factor.share();
        assert_eq!(
            format!("{separate_first:?}"),
            format!("{:?}", finish_sparse_structural_on(&shared, &prepared).unwrap()),
            "{name}: first"
        );
        for scale in [1.0, -0.5, 3.0] {
            let force: Vec<f64> = pair.force.iter().enumerate().map(|(i, f)| f * scale + i as f64).collect();
            let other = SparseStructuralSystem::new(
                &pair.sparse,
                &force,
                &pair.free,
                &pair.prescribed,
                Some(&pair.contributions),
                Some(pair.sparse_evidence()),
            );
            let prepared = prepare_sparse_structural_binary64(&other).unwrap();
            let separate = factor_sparse_structural_profile(&prepared, &order, &first_columns)
                .and_then(|f| finish_sparse_structural(&f));
            assert_eq!(
                format!("{separate:?}"),
                format!("{:?}", finish_sparse_structural_on(&shared, &prepared)),
                "{name}: scale {scale}"
            );
        }
        // Another matrix (the stiffness tripled) is refused.
        let doubled = SparseStiffness::from_dense(
            &pair.dense.iter().map(|row| row.iter().map(|v| 3.0 * v).collect()).collect::<Vec<Vec<f64>>>(),
        )
        .unwrap();
        let other = SparseStructuralSystem::new(&doubled, &pair.force, &pair.free, &pair.prescribed, None, None);
        if let Ok(prepared) = prepare_sparse_structural_binary64(&other) {
            assert!(matches!(
                finish_sparse_structural_on(&shared, &prepared),
                Err(StructuralError::InvalidInput("prepared matrix differs from the shared factor's"))
            ), "{name}");
        }
    }
}

fn dense_multiply(k: &[Vec<f64>], u: &[f64]) -> Vec<f64> {
    k.iter()
        .map(|row| row.iter().zip(u).map(|(a, b)| a * b).sum())
        .collect()
}

#[test]
fn k1_multiply_reactions_and_reduction_match_the_dense_forms() {
    for (name, model) in models() {
        let pair = Pair::new(&model);
        let n = pair.force.len();
        // Displacements with both signs of zero, as a solve can leave them.
        let u: Vec<f64> = (0..n)
            .map(|i| match i % 5 {
                1 => -0.0,
                2 => 1e-3 * (i as f64),
                3 => -2.5e-4 * (i as f64),
                _ => 0.0,
            })
            .collect();
        assert_eq!(
            bits(&pair.sparse.multiply(&u).unwrap()),
            bits(&dense_multiply(&pair.dense, &u)),
            "{name}"
        );
        for zero in [0.0, -0.0] {
            let z = vec![zero; n];
            assert_eq!(
                bits(&pair.sparse.multiply(&z).unwrap()),
                bits(&dense_multiply(&pair.dense, &z)),
                "{name}"
            );
        }
        // E12: the product's restrained reactions, from sparse rows.
        let expected: Vec<f64> = dense_multiply(&pair.dense, &u)
            .into_iter()
            .enumerate()
            .map(|(dof, internal)| {
                let mut a = ExactAccumulator::new();
                a.add(internal)
                    .and_then(|()| pair.ledger.accumulate_dof(dof, &mut a, true))
                    .and_then(|()| a.round())
                    .unwrap_or(f64::NAN)
            })
            .collect();
        assert_eq!(
            bits(&pair.sparse.reactions(&u, &pair.ledger).unwrap()),
            bits(&expected),
            "{name}"
        );
        // KS2 reduction: free map and reduced force bit for bit, both kinds.
        let boundary: Vec<usize> = pair.prescribed.iter().map(|p| p.0).collect();
        let values: Vec<f64> = pair.prescribed.iter().map(|p| p.1).collect();
        let dense = reduce_assembled_system_with_prescribed_displacements(
            &pair.dense,
            &pair.ledger,
            &boundary,
            &values,
        )
        .unwrap();
        let sparse =
            reduce_assembled_sparse_system(&pair.sparse, &pair.ledger, &boundary, Some(&values))
                .unwrap();
        assert_eq!(dense.free_dofs, sparse.free_dofs, "{name}");
        assert_eq!(
            bits(dense.force.values()),
            bits(sparse.force.values()),
            "{name}"
        );
        for (r, &dof) in sparse.free_dofs.iter().enumerate() {
            assert_eq!(sparse.free_position(dof), Some(r));
        }
        let dense = reduce_assembled_system(&pair.dense, &pair.ledger, &boundary).unwrap();
        let sparse =
            reduce_assembled_sparse_system(&pair.sparse, &pair.ledger, &boundary, None).unwrap();
        assert_eq!(
            bits(dense.force.values()),
            bits(sparse.force.values()),
            "{name}"
        );
        // Boundary errors in the same order.
        let mut repeated = boundary.clone();
        repeated.push(boundary[0]);
        assert_eq!(
            reduce_assembled_system(&pair.dense, &pair.ledger, &repeated).unwrap_err(),
            reduce_assembled_sparse_system(&pair.sparse, &pair.ledger, &repeated, None)
                .unwrap_err()
        );
        let outside = vec![n];
        assert_eq!(
            reduce_assembled_system_with_prescribed_displacements(
                &pair.dense,
                &pair.ledger,
                &outside,
                &[1.0]
            )
            .unwrap_err(),
            reduce_assembled_sparse_system(&pair.sparse, &pair.ledger, &outside, Some(&[1.0]))
                .unwrap_err()
        );
    }
}

// ------------------------------------------------------------------ KREV on sparse

#[test]
fn krev02_on_sparse_prescribed_contribution_error_is_not_zero_fidelity() {
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
        let ds = StructuralSystem {
            stiffness: &matrix,
            force: &force,
            free_dofs: &free,
            prescribed: &bc,
            contributions: Some(&entries),
            contribution_groups: None,
            symmetry: None,
        };
        let ss = SparseStructuralSystem::new(&sparse, &force, &free, &bc, Some(&entries), None);
        if coupling == -1.0 {
            let s = solve_both(
                prepare_structural(&ds),
                prepare_sparse_structural(&ss),
                "krev02",
            )
            .unwrap()
            .unwrap();
            assert_eq!(s.displacements[0], 1.0);
            assert!((s.displacements[1] - 0.5).abs() / 0.5 <= 1e-9);
            assert_eq!(s.report.assembly_load_perturbation_estimate, 0.0);
        } else {
            let prepared = prepare_sparse_structural(&ss).unwrap();
            assert_eq!(prepared.assembly_load_perturbation_estimate, 1.0);
            let solved = solve_both(
                prepare_structural(&ds),
                prepare_sparse_structural(&ss),
                "krev02",
            );
            assert!(matches!(solved, Some(Err(_))));
        }
    }
}

#[test]
fn krev03_on_sparse_offdiagonal_expansion_retains_lost_low_tail() {
    let matrix = vec![vec![2.0, 0.0], vec![0.0, 2.0]];
    let force = [1.0, 1.0];
    let free = [0, 1];
    let mut entries = vec![
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
    ];
    for (row, col) in [(0, 1), (1, 0)] {
        for value in [1e16, 1.0, 1e-16, -1.0, -1e16] {
            entries.push(StiffnessContribution { row, col, value });
        }
    }
    // The stored (0,1) is the dense 0.0 (the binary64 sum cancels); the
    // pattern keeps it as an explicit zero, so the audit sees its lost 1e-16.
    let sparse = SparseStiffness::from_contributions(2, &entries).unwrap();
    assert_eq!(sparse.get(0, 1).to_bits(), 0.0_f64.to_bits());
    assert!(sparse.pattern().find(0, 1).is_some());
    let ss = SparseStructuralSystem::new(&sparse, &force, &free, &[], Some(&entries), None);
    let prepared = prepare_sparse_structural(&ss).unwrap();
    assert!(prepared.assembly_relative_perturbation_estimate > 0.0);
    let entry = prepared
        .contribution_rounding
        .iter()
        .find(|e| e.row == 0 && e.col == 1)
        .unwrap();
    let exact = Expansion {
        terms: entry.difference_expansion.clone(),
        operations: 0,
    };
    assert_eq!(exact.exact_scalar().unwrap(), Some(1e-16));
    let ds = StructuralSystem {
        stiffness: &matrix,
        force: &force,
        free_dofs: &free,
        prescribed: &[],
        contributions: Some(&entries),
        contribution_groups: None,
        symmetry: None,
    };
    solve_both(
        prepare_structural(&ds),
        prepare_sparse_structural(&ss),
        "krev03",
    );
}

#[test]
fn krev04_on_sparse_witness_helpers_reject_internally_corrupted_shape_without_panic() {
    let matrix = vec![vec![1.0, 2.0], vec![2.0, 1.0]];
    let sparse = SparseStiffness::from_dense(&matrix).unwrap();
    let force = [1.0, 1.0];
    let free = [0, 1];
    let ss = SparseStructuralSystem::new(&sparse, &force, &free, &[], None, None);
    let corruptions: [fn(&mut SparsePreparedSystem<'_>); 6] = [
        |p| p.values.clear(),
        |p| {
            p.row_starts.pop();
        },
        |p| p.scale_exponents.clear(),
        |p| p.transpose[0] = 99,
        |p| p.columns[0] = 7,
        |p| p.diagonal[1] = 0,
    ];
    for corrupt in corruptions {
        let mut prepared = prepare_sparse_structural(&ss).unwrap();
        corrupt(&mut prepared);
        assert!(
            std::panic::catch_unwind(|| verify_sparse_negative_direction(&prepared, &[1.0, -1.0]))
                .unwrap()
                .is_err()
        );
        assert!(
            std::panic::catch_unwind(|| sparse_negative_pair_witness(&prepared))
                .unwrap()
                .is_err()
        );
    }
    let mut prepared = prepare_sparse_structural(&ss).unwrap();
    let wrong_free = [0];
    let bad = SparseStructuralSystem::new(&sparse, &force, &wrong_free, &[], None, None);
    prepared.source = &bad;
    assert!(
        std::panic::catch_unwind(|| sparse_negative_pair_witness(&prepared))
            .unwrap()
            .is_err()
    );
}

#[test]
fn krev05_on_sparse_completion_requires_positive_source_bound_factor() {
    let matrix = vec![vec![1.0, 2.0], vec![2.0, 1.0]];
    let sparse = SparseStiffness::from_dense(&matrix).unwrap();
    let force = [1.0, 1.0];
    let free = [0, 1];
    let ss = SparseStructuralSystem::new(&sparse, &force, &free, &[], None, None);
    let prepared = prepare_sparse_structural(&ss).unwrap();
    assert!(factor_sparse_structural_profile(&prepared, &[0, 1], &[0, 0]).is_err());
    assert!(factor_sparse_structural_profile(&prepared, &[0, 0], &[0, 0]).is_err());
    // The profile may not omit a represented coefficient.
    assert!(factor_sparse_structural_profile(&prepared, &[0, 1], &[0, 1]).is_err());
    let positive = vec![vec![2.0, 0.0], vec![0.0, 3.0]];
    let ps = SparseStiffness::from_dense(&positive).unwrap();
    let pss = SparseStructuralSystem::new(&ps, &force, &free, &[], None, None);
    let pp = prepare_sparse_structural(&pss).unwrap();
    let factor = factor_sparse_structural_profile(&pp, &[0, 1], &[0, 1]).unwrap();
    let result = finish_sparse_structural(&factor).unwrap();
    assert!((result.displacements[0] - 0.5).abs() < 1e-15);
    assert!((result.displacements[1] - 1.0 / 3.0).abs() < 1e-15);
}

// ------------------------------------------------------------------ witness

#[test]
fn k1_negative_witness_matches_the_dense_search_and_verification() {
    // Indefinite systems: the first witness (its direction, energy and
    // allowance) is the dense search's, and verifying arbitrary directions
    // gives the dense verdict.
    let cases: Vec<Vec<Vec<f64>>> = vec![
        vec![vec![1.0, 2.0], vec![2.0, 1.0]],
        // The pair (2, 0) is a witness across the unstored pairs (1, 0) and
        // (2, 1): scaled, A = [[1, 0, 1.25], [0, 1, 0], [1.25, 0, 1]].
        vec![
            vec![4.0, 0.0, 5.0],
            vec![0.0, 1.0, 0.0],
            vec![5.0, 0.0, 4.0],
        ],
        vec![
            vec![2.0, -1.0, 0.0, 0.0],
            vec![-1.0, 2.0, 2.5, 0.0],
            vec![0.0, 2.5, 2.0, -0.5],
            vec![0.0, 0.0, -0.5, 1.0],
        ],
    ];
    for k in cases {
        let n = k.len();
        let sparse = SparseStiffness::from_dense(&k).unwrap();
        let force = vec![1.0; n];
        let free: Vec<usize> = (0..n).collect();
        let ds = StructuralSystem {
            stiffness: &k,
            force: &force,
            free_dofs: &free,
            prescribed: &[],
            contributions: None,
            contribution_groups: None,
            symmetry: None,
        };
        let ss = SparseStructuralSystem::new(&sparse, &force, &free, &[], None, None);
        let dp = prepare_structural(&ds).unwrap();
        let sp = prepare_sparse_structural(&ss).unwrap();
        let dw = negative_pair_witness(&dp).unwrap();
        let sw = sparse_negative_pair_witness(&sp).unwrap();
        assert!(dw.is_some());
        assert_eq!(format!("{dw:?}"), format!("{sw:?}"));
        let directions: [Vec<f64>; 3] = [
            vec![1.0; n],
            (0..n)
                .map(|i| if i % 2 == 0 { 1.0 } else { -1.0 })
                .collect(),
            (0..n).map(|i| 0.5 - i as f64).collect(),
        ];
        for direction in directions {
            assert_eq!(
                format!("{:?}", verify_negative_direction(&dp, &direction)),
                format!("{:?}", verify_sparse_negative_direction(&sp, &direction))
            );
        }
    }
}

#[test]
fn k1_negative_witness_visits_only_pattern_pairs() {
    // A long chain (positive definite, no witness): the search visits exactly
    // the stored pairs below the diagonal of the free block, a small fraction
    // of the dense search's n(n-1)/2 pairs (a deterministic count, not a time).
    let model = chain(40, 200e9, 0.0);
    let pair = Pair::new(&model);
    let ss = pair.sparse_system(false);
    let prepared = prepare_sparse_structural(&ss).unwrap();
    let (witness, visited) = sparse_negative_pair_witness_counted(&prepared).unwrap();
    assert!(witness.is_none());
    let n = prepared.dimension();
    let stored_lower = prepared.lower_entries().filter(|&(r, c, _)| c < r).count();
    assert_eq!(visited, stored_lower);
    let dense_pairs = n * (n - 1) / 2;
    assert!(
        visited * 10 < dense_pairs,
        "visited {visited} of {dense_pairs}"
    );
    // The dense search finds no witness either. It is run on a six-member
    // chain: it verifies each of the n(n-1)/2 pairs in O(n^2), which on the
    // 40-member chain above takes over a minute in a debug build.
    let small = Pair::new(&chain(6, 200e9, 0.0));
    let ds = small.dense_system(false);
    let ss = small.sparse_system(false);
    assert!(negative_pair_witness(&prepare_structural(&ds).unwrap())
        .unwrap()
        .is_none());
    assert!(
        sparse_negative_pair_witness(&prepare_sparse_structural(&ss).unwrap())
            .unwrap()
            .is_none()
    );
}

// ------------------------------------------------------------------ references

/// The outcome class: the quality of a solution, or the kind of refusal.
fn class(result: &Result<StructuralSolution, StructuralError>) -> String {
    match result {
        Ok(s) => format!("{:?}", s.report.quality),
        // The variant's name (the refusal kind), without its payload.
        Err(e) => format!("{e:?}")
            .split(|c: char| !c.is_alphanumeric())
            .next()
            .unwrap_or_default()
            .to_string(),
    }
}

#[test]
fn k1_matrix_references_have_the_same_outcome_class_in_both_representations() {
    // Kernel-level reference systems, with the frozen references' stated
    // values (fixtures.json: N07, R01, R04, R06, NP-B's construction, NP-D's
    // skew, nonfinite and duplicate-cancellation controls, R03's lost load).
    // Each is solved through both representations with one order
    // (`solve_both` asserts byte-identical outcomes) and its class is also
    // today's dense Cholesky mode's.
    let r01: f64 = "596902604.182060715307902242823105547997462185881270105985239472538485117194379709739326616815002242".parse().unwrap();
    let r04: f64 = "1193805208.364121430615804485646211095994924371762540211970478945076970234388759419478653233630004484".parse().unwrap();
    let mut r06 = vec![vec![0.0; 6]; 6];
    for (i, row) in r06.iter_mut().enumerate() {
        row[i] = 2.0;
    }
    type Case = (
        &'static str,
        Vec<Vec<f64>>,
        Vec<f64>,
        Vec<usize>,
        Vec<(usize, f64)>,
    );
    let mut cases: Vec<Case> = vec![
        (
            "N07",
            vec![vec![1.0, 2.0], vec![2.0, 1.0]],
            vec![1.0, 1.0],
            vec![0, 1],
            vec![],
        ),
        ("R01", vec![vec![r01]], vec![1000.0], vec![0], vec![]),
        (
            "R04",
            vec![
                vec![r04, -r04, 0.0],
                vec![-r04, 2.0 * r04, -r04],
                vec![0.0, -r04, r04],
            ],
            vec![0.0, 0.0, 0.0],
            vec![1],
            vec![(0, 0.0001), (2, 0.0)],
        ),
        (
            "R06",
            r06,
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            vec![],
            (0..6).map(|d| (d, 0.0)).collect(),
        ),
        (
            "NP-D skew",
            vec![vec![2.0, 1.0], vec![1.01, 2.0]],
            vec![1.0, 1.0],
            vec![0, 1],
            vec![],
        ),
        (
            "NP-D nonfinite",
            vec![vec![f64::NAN, 0.0], vec![0.0, 1.0]],
            vec![1.0, 1.0],
            vec![0, 1],
            vec![],
        ),
    ];
    for n in [8, 64, 80] {
        for c in [0.75, 0.750000000001] {
            let mut lower = vec![vec![0.0; n]; n];
            for i in 0..n {
                lower[i][i] = 1.0;
                if i > 0 {
                    lower[i][i - 1] = c;
                }
                if i > 1 {
                    lower[i][i - 2] = c;
                }
            }
            let k: Vec<Vec<f64>> = (0..n)
                .map(|i| {
                    (0..n)
                        .map(|j| (0..n).map(|q| lower[i][q] * lower[j][q]).sum())
                        .collect()
                })
                .collect();
            let expected: Vec<f64> = (0..n)
                .map(|i| if i % 2 == 0 { 1.0 } else { -1.0 })
                .collect();
            let f: Vec<f64> = k
                .iter()
                .map(|r| r.iter().zip(&expected).map(|(a, b)| a * b).sum())
                .collect();
            cases.push(("NP-B", k, f, (0..n).collect(), vec![]));
        }
    }
    let mut classes = Vec::new();
    for (name, k, f, free, prescribed) in &cases {
        let sparse = SparseStiffness::from_dense(k).unwrap();
        let ds = StructuralSystem {
            stiffness: k,
            force: f,
            free_dofs: free,
            prescribed,
            contributions: None,
            contribution_groups: None,
            symmetry: None,
        };
        let ss = SparseStructuralSystem::new(&sparse, f, free, prescribed, None, None);
        let cholesky = solve_structural_dense(&ds);
        match solve_both(
            prepare_structural(&ds),
            prepare_sparse_structural(&ss),
            name,
        ) {
            // Both preparations refused (NP-D): so does the Cholesky mode.
            None => assert!(cholesky.is_err(), "{name}"),
            // A refused factor (N07): the Cholesky mode refuses too; the
            // product path then runs the witness (`k1_negative_witness_*`).
            Some(Err(_)) => assert!(cholesky.is_err(), "{name}"),
            Some(ok) => assert_eq!(class(&ok), class(&cholesky), "{name}"),
        }
        classes.push((name.to_string(), class(&cholesky)));
    }
    eprintln!("k1 reference classes: {classes:?}");
    // R02 (a stored matrix that differs from its contributions) and R05 (a
    // spring contribution omitted from the stored matrix): the contribution
    // audit refuses both in both representations.
    let r02_used: f64 = "590992677.4079809062454477651713916316806556295854159465200390817212723932617620888508184324901012297".parse().unwrap();
    let r05_spring: f64 = "298451302.091030357653951121411552773998731092940635052992619736269242558597189854869663308407501121".parse().unwrap();
    let r05_frame = r04;
    for (name, stored, entries) in [
        (
            "R02",
            r02_used,
            vec![StiffnessContribution {
                row: 0,
                col: 0,
                value: r01,
            }],
        ),
        (
            "R05",
            r05_frame,
            vec![
                StiffnessContribution {
                    row: 0,
                    col: 0,
                    value: r05_frame,
                },
                StiffnessContribution {
                    row: 0,
                    col: 0,
                    value: r05_spring,
                },
            ],
        ),
    ] {
        let k = vec![vec![stored]];
        let sparse = SparseStiffness::from_dense(&k).unwrap();
        let force = [1000.0];
        let ds = StructuralSystem {
            stiffness: &k,
            force: &force,
            free_dofs: &[0],
            prescribed: &[],
            contributions: Some(&entries),
            contribution_groups: None,
            symmetry: None,
        };
        let ss = SparseStructuralSystem::new(&sparse, &force, &[0], &[], Some(&entries), None);
        let solved = solve_both(
            prepare_structural(&ds),
            prepare_sparse_structural(&ss),
            name,
        );
        assert!(matches!(solved, Some(Err(_))), "{name}: {solved:?}");
        assert!(solve_structural_dense(&ds).is_err(), "{name}");
    }
    // R03: the lost load through the load audit (Sensitive in both).
    let k = vec![vec![r01]];
    let sparse = SparseStiffness::from_dense(&k).unwrap();
    let zero = [0.0];
    let mut ledger = LoadLedger::new();
    ledger.push("load:r03", 0, 1000.0);
    let force = ledger.finish(1).unwrap();
    let ds = StructuralSystem {
        stiffness: &k,
        force: &zero,
        free_dofs: &[0],
        prescribed: &[],
        contributions: None,
        contribution_groups: None,
        symmetry: None,
    };
    let ss = SparseStructuralSystem::new(&sparse, &zero, &[0], &[], None, None);
    let r03 = solve_both(
        prepare_structural_with_force_terms(&ds, force.terms()),
        prepare_sparse_structural_with_force_terms(&ss, force.terms()),
        "R03",
    )
    .unwrap()
    .unwrap();
    assert_eq!(r03.report.quality, SolveQuality::Sensitive);
    // NP-D duplicate cancellation: (1e16, 1, -1e16) coalesces to 0.0 in
    // binary64, so both representations refuse the zero diagonal.
    let entries = [
        StiffnessContribution {
            row: 0,
            col: 0,
            value: 1e16,
        },
        StiffnessContribution {
            row: 0,
            col: 0,
            value: 1.0,
        },
        StiffnessContribution {
            row: 0,
            col: 0,
            value: -1e16,
        },
    ];
    let coalesced = SparseStiffness::from_contributions(1, &entries).unwrap();
    let dense = vec![vec![coalesced.get(0, 0)]];
    let one = [1.0];
    let ds = StructuralSystem {
        stiffness: &dense,
        force: &one,
        free_dofs: &[0],
        prescribed: &[],
        contributions: Some(&entries),
        contribution_groups: None,
        symmetry: None,
    };
    let ss = SparseStructuralSystem::new(&coalesced, &one, &[0], &[], Some(&entries), None);
    assert!(solve_both(
        prepare_structural(&ds),
        prepare_sparse_structural(&ss),
        "NP-D duplicate cancellation"
    )
    .is_none());
}

// ------------------------------------------------------------------ R-7 node blocks (S-K1 F-1)

/// A skewed spring block: k = 5e5
/// along d̃ = (0.75, 1, 0), fl(k·fl(fl(d̃_i·d̃_j)/n)).
fn skew_block() -> [[f64; 3]; 3] {
    let d = [0.75, 1.0, 0.0];
    let n = 1.5625;
    std::array::from_fn(|i| std::array::from_fn(|j| 5e5 * ((d[i] * d[j]) / n)))
}

fn assemble_with(model: &Model, options: &SparseAssemblyOptions) -> Result<SparseStiffness, FrameKernelError> {
    assemble_sparse_stiffness(model.node_count, &model.frames, &model.connectors, &model.blocks, &model.springs, options)
}

#[test]
fn skew_no_node_blocks_is_today_bit_for_bit() {
    for (name, model) in models() {
        let today = model.sparse();
        let empty = assemble_with(&model, &SparseAssemblyOptions::new().with_node_blocks(Vec::new())).unwrap();
        assert_eq!(bits(today.values()), bits(empty.values()), "{name}");
        assert_eq!(today.pattern(), empty.pattern(), "{name}");
        assert!(SparseAssemblyOptions::new().node_blocks().is_empty());
    }
}

#[test]
fn skew_node_block_is_its_dense_addition_after_the_springs() {
    for (name, model) in models() {
        let node = model.node_count - 1;
        let base = 6 * node;
        let today = model.sparse();
        let options = SparseAssemblyOptions::new().with_node_blocks(vec![(node, skew_block()), (node, skew_block())]);
        let with = assemble_with(&model, &options).unwrap();
        assert_eq!(today.pattern(), with.pattern(), "{name}: the pattern is unchanged");
        let mut dense = model.dense();
        for _ in 0..2 {
            for (r, row) in skew_block().iter().enumerate() {
                for (c, &v) in row.iter().enumerate() {
                    dense[base + r][base + c] += v;
                }
            }
        }
        for r in 0..dense.len() {
            for c in 0..dense.len() {
                assert_eq!(with.get(r, c).to_bits(), dense[r][c].to_bits(), "{name}: ({r}, {c})");
            }
        }
    }
}

#[test]
fn skew_node_block_refusals() {
    let model = chain(3, 2.0e11, 0.0);
    // A node with no element: its block is not in the pattern.
    let mut unconnected = chain(3, 2.0e11, 0.0);
    unconnected.node_count += 1;
    let options = SparseAssemblyOptions::new().with_node_blocks(vec![(4, skew_block())]);
    assert_eq!(
        assemble_with(&unconnected, &options).unwrap_err(),
        FrameKernelError::InvalidInput("skew node block outside the stiffness pattern")
    );
    // A node outside the model.
    let options = SparseAssemblyOptions::new().with_node_blocks(vec![(9, skew_block())]);
    assert!(matches!(assemble_with(&model, &options), Err(FrameKernelError::InvalidNodeIndex { node_index: 9, .. })));
    // Skew at a force scale is S-K2's: refused before any formation.
    let options = SparseAssemblyOptions::new()
        .with_force_scale(ForceScale::new(2).unwrap())
        .with_node_blocks(vec![(3, skew_block())]);
    assert_eq!(
        assemble_with(&model, &options).unwrap_err(),
        FrameKernelError::InvalidInput("skew node block at force scale")
    );
}

/// RT-A3's F-1 condition: a factor shared across right-hand sides with node
/// blocks present completes each as its separate solve does.
#[test]
fn p_a_shared_factor_with_skew_node_blocks_completes_other_right_hand_sides() {
    let model = chain(3, 2.0e11, 0.0);
    // The model's symmetry evidence covers the node block: it is exactly symmetric.
    let pair = Pair::new(&model);
    let k = assemble_with(&model, &SparseAssemblyOptions::new().with_node_blocks(vec![(3, skew_block())])).unwrap();
    let dense = k.to_dense();
    let system = SparseStructuralSystem::new(&k, &pair.force, &pair.free, &pair.prescribed, None, Some(pair.sparse_evidence()));
    let prepared = prepare_sparse_structural_binary64(&system).unwrap();
    let dense_system = StructuralSystem {
        stiffness: &dense,
        force: &pair.force,
        free_dofs: &pair.free,
        prescribed: &pair.prescribed,
        contributions: None,
        contribution_groups: None,
        symmetry: Some(pair.dense_evidence()),
    };
    let (order, columns) = natural_profile(&prepare_structural_binary64(&dense_system).unwrap());
    let factor = factor_sparse_structural_profile(&prepared, &order, &columns).unwrap();
    let separate_first = finish_sparse_structural(&factor).unwrap();
    let shared = factor.share();
    assert_eq!(
        format!("{separate_first:?}"),
        format!("{:?}", finish_sparse_structural_on(&shared, &prepared).unwrap())
    );
    for scale in [1.0, -0.5, 3.0] {
        let other_force: Vec<f64> = pair.force.iter().enumerate().map(|(i, f)| f * scale + i as f64).collect();
        let other = SparseStructuralSystem::new(&k, &other_force, &pair.free, &pair.prescribed, None, Some(pair.sparse_evidence()));
        let prepared = prepare_sparse_structural_binary64(&other).unwrap();
        let separate = factor_sparse_structural_profile(&prepared, &order, &columns).and_then(|f| finish_sparse_structural(&f));
        assert_eq!(format!("{separate:?}"), format!("{:?}", finish_sparse_structural_on(&shared, &prepared)), "scale {scale}");
    }
}

// ------------------------------------------------------ K-D5 on the pattern

/// The settled chain (node 0 prescribed, a settlement at its UY; free rows of
/// different extents) with one stored symmetric pair that no contribution
/// forms, between nodes 3 and 9. In an assembled system the contributions
/// cover every stored position, so a walk that dropped stored positions would
/// still visit them through the contributions; this pair is visited only
/// through the stored patterns.
fn kd5_pair(extra: f64) -> (Model, Pair, (usize, usize)) {
    let model = chain(12, 200e9, 0.003);
    let mut pair = Pair::new(&model);
    let n = model.node_count * 6;
    let (i, j) = (6 * 3 + 1, 6 * 9 + 2);
    assert!(pair.dense[i][j] == 0.0 && pair.sparse.pattern().find(i, j).is_none());
    pair.dense[i][j] = extra;
    pair.dense[j][i] = extra;
    let old = pair.sparse.pattern().clone();
    let positions = (0..n)
        .flat_map(|r| old.row(r).iter().map(move |&c| (r, c)))
        .chain([(i, j)]);
    let pattern = SparsePattern::from_positions(n, positions).unwrap();
    let per_entry: Vec<(usize, usize)> = (0..n)
        .flat_map(|r| pattern.row(r).iter().map(move |&c| (r, c)))
        .collect();
    let values = per_entry.iter().map(|&(r, c)| pair.dense[r][c]).collect();
    pair.sparse_roundoff = per_entry.iter().map(|&(r, c)| pair.roundoff[r][c]).collect();
    pair.sparse_counts = per_entry.iter().map(|&(r, c)| pair.counts[r][c]).collect();
    pair.sparse = SparseStiffness::from_pattern_values(pattern, values).unwrap();
    (model, pair, (i, j))
}

/// K-D5 through both pattern handoffs, the first solve's
/// (`Represented::formation_check`) and the confirmation attempt's
/// (`SparseConfirmationAttempt::evaluate_at`), equals the dense
/// representation's, which walks every free pair with the dense callbacks:
/// the epsilon_E bits, the finding and the member-bound inputs (lambda sigma
/// included). K1 requires it: the sparse report is byte-identical in `Debug`
/// to the dense-derived one for the same equations (module documentation),
/// and the stored position without a contribution makes epsilon_E depend on
/// the pattern each handoff passes and on the prepared row each reads.
#[test]
fn kd5_through_both_pattern_handoffs_is_the_dense_full_walk_bit_for_bit() {
    let (model, pair, (i, j)) = kd5_pair(KD5_EXTRA);
    assert!(!pair
        .contributions
        .iter()
        .any(|e| (e.row, e.col) == (i, j) || (e.row, e.col) == (j, i)));
    assert!(pair.prescribed.iter().any(|&(_, g)| g != 0.0));
    let source = FormationSource {
        node_count: model.node_count,
        frames: model.frames.clone(),
        ..FormationSource::default()
    };
    let dense = || {
        StructuralSystem::assembled(
            &pair.dense,
            &pair.ledger,
            &pair.free,
            &pair.prescribed,
            Some(&pair.contributions),
            Some(pair.dense_evidence()),
        )
    };
    let sparse = || {
        SparseStructuralSystem::assembled(
            &pair.sparse,
            &pair.ledger,
            &pair.free,
            &pair.prescribed,
            Some(&pair.contributions),
            Some(pair.sparse_evidence()),
        )
    };
    let dense_checked = dense().with_formation_source(&source);
    let sparse_checked = sparse().with_formation_source(&source);

    // The first solve: the completion hands the prepared pattern over.
    let dp = prepare_formation_checked_structural(&dense_checked).unwrap();
    let sp = prepare_formation_checked_sparse_structural(&sparse_checked).unwrap();
    assert_prepared_equal(&dp, &sp, "kd5 first solve");
    let (starts, columns) = sp.gate_pattern().unwrap();
    let row = |r: usize| &columns[starts[r]..starts[r + 1]];
    assert!((1..sp.dimension()).any(|r| row(r) != row(0)), "free rows differ");
    let (order, first) = natural_profile(&dp);
    let d = finish_structural(&factor_structural_profile(&dp, &order, &first).unwrap()).unwrap();
    let s = finish_sparse_structural(&factor_sparse_structural_profile(&sp, &order, &first).unwrap())
        .unwrap();
    assert!(d.formation_check.is_none(), "{:?}", d.formation_check);
    let bound = d.member_bound.as_ref().expect("the dense case passes K-D5");
    assert!(bound.lambda_sigma > 0.0 && bound.lambda_sigma.is_finite());
    assert_eq!(format!("{:?}", s.formation_check), format!("{:?}", d.formation_check));
    assert_eq!(format!("{:?}", s.member_bound), format!("{:?}", d.member_bound));
    assert_eq!(
        s.member_bound.as_ref().unwrap().lambda_sigma.to_bits(),
        bound.lambda_sigma.to_bits()
    );
    assert_eq!(format!("{s:?}"), format!("{d:?}"));

    // The confirmation attempt: a gate prepared without the source, then the
    // checked system evaluated on the gate's factor.
    let dense_gate = dense();
    let sparse_gate = sparse();
    let dg = prepare_assembled_structural(&dense_gate).unwrap();
    let sg = prepare_assembled_sparse_structural(&sparse_gate).unwrap();
    assert_prepared_equal(&dg, &sg, "kd5 confirmation gate");
    let df = factor_structural_profile(&dg, &order, &first).unwrap();
    let sf = factor_sparse_structural_profile(&sg, &order, &first).unwrap();
    let (d_solution, d_attempt) = finish_confirmation(&df).unwrap();
    let (s_solution, s_attempt) = finish_sparse_confirmation(&sf).unwrap();
    assert_eq!(format!("{s_solution:?}"), format!("{d_solution:?}"));
    let u = &d_solution.displacements;
    assert!(d_attempt.epsilon.get().is_none() && s_attempt.epsilon.get().is_none());
    let d_out = d_attempt.evaluate_at(&dense_checked, u, true).unwrap();
    let s_out = s_attempt.evaluate_at(&sparse_checked, u, true).unwrap();
    let d_epsilon = d_attempt.epsilon.get().expect("the dense attempt formed epsilon_E");
    let s_epsilon = s_attempt.epsilon.get().expect("the sparse attempt formed epsilon_E");
    assert!(d_epsilon > 0.0);
    assert_eq!(s_epsilon.to_bits(), d_epsilon.to_bits(), "epsilon_E");
    assert!(d_out.0.is_none(), "{:?}", d_out.0);
    let d_bound = d_out.1.as_ref().expect("the dense confirmation passes K-D5");
    assert_eq!(format!("{:?}", s_out.0), format!("{:?}", d_out.0));
    assert_eq!(format!("{:?}", s_out.1), format!("{:?}", d_out.1));
    assert_eq!(
        s_out.1.as_ref().unwrap().lambda_sigma.to_bits(),
        d_bound.lambda_sigma.to_bits()
    );
}

/// The stored-only pair's value: it dominates epsilon_E (about 1.8e-14 against
/// 5.7e-15 without it) while the case still passes K-D5 in both handoffs.
const KD5_EXTRA: f64 = 1.0e-6;

/// The stored and prepared patterns of `system`'s first solve with the
/// stored-only pair (i, j) left out of both, so the walk does not visit it:
/// the wrong pattern of mutations W1 and W2, of the right shape.
#[cfg(debug_assertions)]
fn kd5_pattern_without(
    pair: &Pair,
    prepared: &SparsePreparedSystem<'_>,
    (i, j): (usize, usize),
) -> (SparsePattern, Vec<usize>, Vec<usize>) {
    let real = pair.sparse.pattern();
    let n = real.dimension();
    let positions = (0..n)
        .flat_map(|r| real.row(r).iter().map(move |&c| (r, c)))
        .filter(|&p| p != (i, j) && p != (j, i));
    let assembled = SparsePattern::from_positions(n, positions).unwrap();
    assert!(assembled.find(i, j).is_none() && real.find(i, j).is_some());
    let at = |dof: usize| pair.free.iter().position(|&d| d == dof).unwrap();
    let (ri, rj) = (at(i), at(j));
    let (starts, columns) = prepared.gate_pattern().unwrap();
    let mut kept_starts = vec![0];
    let mut kept = Vec::new();
    for r in 0..prepared.dimension() {
        kept.extend(
            columns[starts[r]..starts[r + 1]]
                .iter()
                .copied()
                .filter(|&c| (r, c) != (ri, rj) && (r, c) != (rj, ri)),
        );
        kept_starts.push(kept.len());
    }
    assert_eq!(kept.len() + 2, columns.len());
    (assembled, kept_starts, kept)
}

/// Both handoff tests expect the first unvisited pair that reads anything but
/// +0.0: the stored-only pair, at free row 13 (DOF 19), free column 50 (DOF 56).
///
/// W1 as a test: the first solve's handoff arguments (`Represented::
/// formation_check`: the view, its ledger, no epsilon cache) with a pattern
/// that omits a stored position, on the real prepared system and factor.
#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = "epsilon_E pattern handoff: the assembled callback reads 1e-6, not +0.0, at free row 13 (DOF 19), free column 50 (DOF 56), off the stored pattern")]
fn kd5_first_solve_handoff_with_a_pattern_missing_a_stored_position_panics() {
    let (model, pair, omitted) = kd5_pair(KD5_EXTRA);
    let source = FormationSource {
        node_count: model.node_count,
        frames: model.frames.clone(),
        ..FormationSource::default()
    };
    let checked = SparseStructuralSystem::assembled(
        &pair.sparse,
        &pair.ledger,
        &pair.free,
        &pair.prescribed,
        Some(&pair.contributions),
        Some(pair.sparse_evidence()),
    )
    .with_formation_source(&source);
    let prepared = prepare_formation_checked_sparse_structural(&checked).unwrap();
    let n = prepared.dimension();
    let order: Vec<usize> = (0..n).collect();
    let first: Vec<usize> = (0..n)
        .map(|r| prepared.columns[prepared.row_range(r)][0].min(r))
        .collect();
    let factor = factor_sparse_structural_profile(&prepared, &order, &first).unwrap();
    // The real handoff passes this system.
    let solution = finish_sparse_structural(&factor).unwrap();
    assert!(solution.formation_check.is_none() && solution.member_bound.is_some());
    let solve = |rhs: &[f64]| factor.factor.solve(rhs);
    let condition = prepared.gate_condition(&solve).unwrap();
    let (assembled, starts, columns) = kd5_pattern_without(&pair, &prepared, omitted);
    let s = &checked.system;
    let view = StructuralSystem {
        stiffness: &[],
        force: s.force,
        free_dofs: s.free_dofs,
        prescribed: s.prescribed,
        contributions: s.contributions,
        contribution_groups: s.contribution_groups,
        symmetry: None,
    };
    let _ = formation_check::check_at(
        &view,
        &source,
        checked.force,
        &prepared.scale_exponents,
        &solution.displacements,
        condition,
        &|i, j| s.stiffness.get(i, j),
        &|r, c| prepared.gate_value(r, c),
        &solve,
        true,
        None,
        Some(formation_check::StoredPattern {
            assembled: &assembled,
            prepared_starts: &starts,
            prepared_columns: &columns,
        }),
    );
}

/// W2 as a test: the confirmation attempt's handoff arguments
/// (`SparseConfirmationAttempt::evaluate_at`: the gate's factor, condition and
/// epsilon cache) with a pattern that omits a stored position.
#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = "epsilon_E pattern handoff: the assembled callback reads 1e-6, not +0.0, at free row 13 (DOF 19), free column 50 (DOF 56), off the stored pattern")]
fn kd5_confirmation_handoff_with_a_pattern_missing_a_stored_position_panics() {
    let (model, pair, omitted) = kd5_pair(KD5_EXTRA);
    let source = FormationSource {
        node_count: model.node_count,
        frames: model.frames.clone(),
        ..FormationSource::default()
    };
    let system = || {
        SparseStructuralSystem::assembled(
            &pair.sparse,
            &pair.ledger,
            &pair.free,
            &pair.prescribed,
            Some(&pair.contributions),
            Some(pair.sparse_evidence()),
        )
    };
    let gate = system();
    let checked = system().with_formation_source(&source);
    let prepared = prepare_assembled_sparse_structural(&gate).unwrap();
    let n = prepared.dimension();
    let order: Vec<usize> = (0..n).collect();
    let first: Vec<usize> = (0..n)
        .map(|r| prepared.columns[prepared.row_range(r)][0].min(r))
        .collect();
    let factor = factor_sparse_structural_profile(&prepared, &order, &first).unwrap();
    let (solution, attempt) = finish_sparse_confirmation(&factor).unwrap();
    let (assembled, starts, columns) = kd5_pattern_without(&pair, &prepared, omitted);
    let s = &checked.system;
    let view = StructuralSystem {
        stiffness: &[],
        force: s.force,
        free_dofs: s.free_dofs,
        prescribed: s.prescribed,
        contributions: s.contributions,
        contribution_groups: s.contribution_groups,
        symmetry: None,
    };
    let _ = formation_check::check_at(
        &view,
        checked.source,
        checked.force,
        &prepared.scale_exponents,
        &solution.displacements,
        attempt.condition,
        &|i, j| s.stiffness.get(i, j),
        &|r, c| prepared.gate_value(r, c),
        &|rhs: &[f64]| attempt.factor.factor.solve(rhs),
        true,
        Some(&attempt.epsilon),
        Some(formation_check::StoredPattern {
            assembled: &assembled,
            prepared_starts: &starts,
            prepared_columns: &columns,
        }),
    );
}
