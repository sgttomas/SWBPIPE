//! K1 (T3 D1 revision 5a.2 §4.8, W3): one sparse representation of the global
//! stiffness, and the M03-INTEGRITY-v1 gate over it.
//!
//! - `SparsePattern` is the structure of the symmetric global stiffness: rows
//!   in ascending DOF order, each with its columns ascending. It is a pure
//!   function of the set of coupled positions (element connectivity in 12×12
//!   blocks, springs on the diagonal), so it depends on neither labels nor
//!   element order (D1 §7.3 mutation 10).
//! - `SparseStiffness` holds one coalesced value per pattern entry. Each value
//!   is summed from +0.0 in exactly the order the dense assembly adds it
//!   (frames, user elements, curved and other explicit blocks, then springs),
//!   so every entry is bit-identical to the dense entry: the parity basis is
//!   bitwise equality, with no tolerance.
//! - `SparseStructuralSystem` is the sparse sibling of `StructuralSystem`. The
//!   gate's validation, preparation, contribution audit, pivot screens,
//!   condition estimate and negative witness are written here over the pattern
//!   in O(nnz) memory. The original-equation residual, the intended-action
//!   audit, the load-fidelity audit, KS1's exact right-hand side and the
//!   completion with K-D5's formation check are the parent module's own
//!   functions, run on this representation through its `Represented` and
//!   `PreparedGate` traits. The skyline LDL is the parent's
//!   `ProfileFactor::factor_structural_profile`.
//! - Every stage reads its terms in the dense stage's order wherever order can
//!   change a bit (sequential binary64 sums, expansion representations, the
//!   first reported error), so the sparse report is byte-identical in `Debug`
//!   to the one the dense-derived path produces for the same equations.
//!
//! Explicit zeros: an entry whose coalesced value is exactly 0 stays in the
//! pattern, so the contribution audit, the residual and the witness see it.
//! It takes no part in adjacency, ordering or profile, exactly like a zero of
//! the dense matrix today: the rule is by value, on the prepared (scaled,
//! symmetrized) matrix that the factor orders.
//!
//! The dense `StructuralSystem` API is unchanged; dense scrutiny materializes
//! `SparseStiffness::to_dense` and runs today's dense Cholesky.
use super::*;
use crate::connector::ObjectiveConnector;
use crate::load_ledger::ReducedForce;
use crate::{
    element_dof_map, force_scaled_matrix, force_scaled_value, ForceScale, FrameElement,
    FrameKernelError, Matrix12, UserStiffnessElement, DOF_PER_NODE, ELEMENT_DOF,
};

// ------------------------------------------------------------------ pattern

/// The structure of a symmetric global stiffness (compressed rows, ascending
/// columns, each entry with the index of its transpose).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SparsePattern {
    dimension: usize,
    row_starts: Vec<usize>,
    columns: Vec<usize>,
    transpose: Vec<usize>,
}

impl SparsePattern {
    /// The symmetric closure of `positions`, deduplicated. A pure function of
    /// the set of positions: their order and repetition do not matter.
    pub fn from_positions(
        dimension: usize,
        positions: impl IntoIterator<Item = (usize, usize)>,
    ) -> Result<Self, StructuralError> {
        let mut rows: Vec<Vec<usize>> = vec![Vec::new(); dimension];
        for (row, col) in positions {
            if row >= dimension || col >= dimension {
                return Err(StructuralError::InvalidInput("pattern position"));
            }
            rows[row].push(col);
            if row != col {
                rows[col].push(row);
            }
        }
        for row in &mut rows {
            row.sort_unstable();
            row.dedup();
        }
        Ok(Self::from_sorted_rows(dimension, rows))
    }

    /// From element connectivity: each element `(node_i, node_j)` couples the
    /// 12 DOFs of its two nodes (four 6×6 node blocks); each DOF in `diagonal`
    /// (a spring) adds its diagonal entry. Node `k` owns DOFs 6k..6k+5.
    pub fn from_connectivity(
        node_count: usize,
        elements: &[(usize, usize)],
        diagonal: &[usize],
    ) -> Result<Self, StructuralError> {
        let dimension = node_count
            .checked_mul(DOF_PER_NODE)
            .ok_or(StructuralError::InvalidInput("pattern dimension"))?;
        let mut neighbours: Vec<Vec<usize>> = vec![Vec::new(); node_count];
        for &(a, b) in elements {
            if a >= node_count || b >= node_count {
                return Err(StructuralError::InvalidInput("pattern element node"));
            }
            neighbours[a].extend([a, b]);
            neighbours[b].extend([a, b]);
        }
        for list in &mut neighbours {
            list.sort_unstable();
            list.dedup();
        }
        let mut spring = vec![false; dimension];
        for &dof in diagonal {
            if dof >= dimension {
                return Err(StructuralError::InvalidInput("pattern diagonal DOF"));
            }
            spring[dof] = true;
        }
        let mut rows = Vec::with_capacity(dimension);
        for (node, list) in neighbours.iter().enumerate() {
            for local in 0..DOF_PER_NODE {
                let dof = node * DOF_PER_NODE + local;
                let row: Vec<usize> = if list.is_empty() {
                    if spring[dof] {
                        vec![dof]
                    } else {
                        Vec::new()
                    }
                } else {
                    // A node with an element couples to itself, so its
                    // diagonal (and any spring on it) is already present.
                    list.iter()
                        .flat_map(|&other| (0..DOF_PER_NODE).map(move |k| other * DOF_PER_NODE + k))
                        .collect()
                };
                rows.push(row);
            }
        }
        Ok(Self::from_sorted_rows(dimension, rows))
    }

    /// Rows must be sorted, deduplicated and structurally symmetric.
    fn from_sorted_rows(dimension: usize, rows: Vec<Vec<usize>>) -> Self {
        let mut row_starts = Vec::with_capacity(dimension + 1);
        row_starts.push(0);
        let mut columns = Vec::new();
        for row in rows {
            columns.extend(row);
            row_starts.push(columns.len());
        }
        let mut pattern = Self {
            dimension,
            row_starts,
            columns,
            transpose: Vec::new(),
        };
        let mut transpose = Vec::with_capacity(pattern.columns.len());
        for row in 0..dimension {
            for index in pattern.row_range(row) {
                let col = pattern.columns[index];
                transpose.push(
                    pattern
                        .find(col, row)
                        .expect("the symmetric closure holds every transpose"),
                );
            }
        }
        pattern.transpose = transpose;
        pattern
    }

    pub fn dimension(&self) -> usize {
        self.dimension
    }
    /// Stored entries (both triangles and the diagonal).
    pub fn entry_count(&self) -> usize {
        self.columns.len()
    }
    /// Stored entries on or below the diagonal.
    pub fn lower_entry_count(&self) -> usize {
        (0..self.dimension)
            .map(|row| self.row(row).iter().filter(|&&col| col <= row).count())
            .sum()
    }
    /// The entry indices of row `row`.
    pub fn row_range(&self, row: usize) -> std::ops::Range<usize> {
        self.row_starts[row]..self.row_starts[row + 1]
    }
    /// The columns of row `row`, ascending.
    pub fn row(&self, row: usize) -> &[usize] {
        &self.columns[self.row_range(row)]
    }
    /// The entry index of `(row, col)`, if stored.
    pub fn find(&self, row: usize, col: usize) -> Option<usize> {
        if row >= self.dimension {
            return None;
        }
        let start = self.row_starts[row];
        self.row(row).binary_search(&col).ok().map(|k| start + k)
    }
    /// The column of entry `index`.
    pub fn column(&self, index: usize) -> usize {
        self.columns[index]
    }
    /// The entry index of the transpose of entry `index`.
    pub fn transpose(&self, index: usize) -> usize {
        self.transpose[index]
    }
}

// ------------------------------------------------------------------ values

/// Deterministic storage counts for the resource guard (D1 §4.8). They are
/// counts of stored values, not measurements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SparseStorageCounts {
    pub dimension: usize,
    /// Stored entries of the global pattern (both triangles).
    pub stored_entries: usize,
    /// Stored entries on or below the diagonal.
    pub lower_entries: usize,
    /// n², the dense matrix's entry count, for comparison.
    pub dense_entries: u128,
}

/// A symmetric global stiffness: one coalesced value per pattern entry.
#[derive(Debug, Clone, PartialEq)]
pub struct SparseStiffness {
    pattern: SparsePattern,
    values: Vec<f64>,
}

impl SparseStiffness {
    /// Coalesces `contributions` on `pattern`: each entry is summed from +0.0
    /// in list order, which is the dense `k[r][c] += v` sequence, so each
    /// entry is bit-identical to the dense assembly of the same list.
    pub fn from_pattern_and_contributions(
        pattern: SparsePattern,
        contributions: &[StiffnessContribution],
    ) -> Result<Self, StructuralError> {
        let mut values = vec![0.0; pattern.entry_count()];
        for contribution in contributions {
            let index = pattern.find(contribution.row, contribution.col).ok_or(
                StructuralError::InvalidInput("contribution outside pattern"),
            )?;
            values[index] += contribution.value;
        }
        Ok(Self { pattern, values })
    }

    /// `from_pattern_and_contributions` on the pattern of the contributions'
    /// own positions.
    pub fn from_contributions(
        dimension: usize,
        contributions: &[StiffnessContribution],
    ) -> Result<Self, StructuralError> {
        let pattern =
            SparsePattern::from_positions(dimension, contributions.iter().map(|c| (c.row, c.col)))?;
        Self::from_pattern_and_contributions(pattern, contributions)
    }

    /// A given dense matrix, stored where either mirror entry is anything but
    /// +0.0 (a -0.0 is kept, so the values are the dense values bit for bit).
    pub fn from_dense(dense: &[Vec<f64>]) -> Result<Self, StructuralError> {
        let n = dense.len();
        if dense.iter().any(|row| row.len() != n) {
            return Err(StructuralError::InvalidInput("matrix dimensions"));
        }
        let stored = |v: f64| v.to_bits() != 0;
        let mut rows: Vec<Vec<usize>> = vec![Vec::new(); n];
        for (i, row) in rows.iter_mut().enumerate() {
            for j in 0..n {
                if stored(dense[i][j]) || stored(dense[j][i]) {
                    row.push(j);
                }
            }
        }
        let pattern = SparsePattern::from_sorted_rows(n, rows);
        let mut values = Vec::with_capacity(pattern.entry_count());
        for i in 0..n {
            for &j in pattern.row(i) {
                values.push(dense[i][j]);
            }
        }
        Ok(Self { pattern, values })
    }

    pub fn pattern(&self) -> &SparsePattern {
        &self.pattern
    }
    pub fn values(&self) -> &[f64] {
        &self.values
    }
    pub fn dimension(&self) -> usize {
        self.pattern.dimension
    }
    /// The entry at `(row, col)`: its stored value, or +0.0.
    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.pattern.find(row, col).map_or(0.0, |k| self.values[k])
    }
    /// Row `row` as (column, value), columns ascending.
    pub fn row(&self, row: usize) -> impl Iterator<Item = (usize, f64)> + Clone + '_ {
        self.pattern
            .row_range(row)
            .map(move |k| (self.pattern.columns[k], self.values[k]))
    }
    /// The dense view of the same values (+0.0 where nothing is stored), for
    /// dense scrutiny and any n ≤ 256 consumer. It allocates n².
    pub fn to_dense(&self) -> Vec<Vec<f64>> {
        let n = self.dimension();
        let mut dense = vec![vec![0.0; n]; n];
        for (i, row) in dense.iter_mut().enumerate() {
            for (j, value) in self.row(i) {
                row[j] = value;
            }
        }
        dense
    }
    pub fn storage_counts(&self) -> SparseStorageCounts {
        let n = self.dimension();
        SparseStorageCounts {
            dimension: n,
            stored_entries: self.pattern.entry_count(),
            lower_entries: self.pattern.lower_entry_count(),
            dense_entries: (n as u128) * (n as u128),
        }
    }

    /// `K * u`, bit-identical to the product's dense
    /// `row.iter().zip(u).map(|(a, b)| a * b).sum()` on the dense view. An
    /// absent entry is +0.0, whose product with `u_j` is a zero signed like
    /// `u_j`; such zeros change a row's sum only through the sign of a zero
    /// result, and only as a set (one +0.0 and one -0.0 stand for all of
    /// them), so one of each is appended when present.
    pub fn multiply(&self, u: &[f64]) -> Result<Vec<f64>, StructuralError> {
        let n = self.dimension();
        if u.len() != n {
            return Err(StructuralError::InvalidInput("displacement vector"));
        }
        let negative_u = u.iter().filter(|x| x.is_sign_negative()).count();
        let mut product = Vec::with_capacity(n);
        for i in 0..n {
            let stored = self.pattern.row(i).len();
            let stored_negative = self
                .pattern
                .row(i)
                .iter()
                .filter(|&&j| u[j].is_sign_negative())
                .count();
            let absent_negative = negative_u - stored_negative;
            let absent_positive = (n - stored) - absent_negative;
            let zeros = [
                (absent_positive > 0).then_some(0.0),
                (absent_negative > 0).then_some(-0.0),
            ];
            product.push(
                self.row(i)
                    .map(|(j, k)| k * u[j])
                    .chain(zeros.into_iter().flatten())
                    .sum::<f64>(),
            );
        }
        Ok(product)
    }

    /// The product's restrained reactions (E12), from sparse rows: for every
    /// DOF, one exact sum of the formed `K * u` (`multiply`) minus that DOF's
    /// ledger terms, rounded once; NaN where the sum is out of range, as the
    /// product keeps it for `require_finite_mechanics` to refuse.
    pub fn reactions(
        &self,
        u: &[f64],
        force: &AssembledForce,
    ) -> Result<Vec<f64>, StructuralError> {
        if force.len() != self.dimension() {
            return Err(StructuralError::InvalidInput("force vector"));
        }
        Ok(self
            .multiply(u)?
            .into_iter()
            .enumerate()
            .map(|(dof, internal)| {
                let mut accumulator = ExactAccumulator::new();
                accumulator
                    .add(internal)
                    .and_then(|()| force.accumulate_dof(dof, &mut accumulator, true))
                    .and_then(|()| accumulator.round())
                    .unwrap_or(f64::NAN)
            })
            .collect())
    }

    /// K2b (D1 §4.7 step 5): the E12 reactions of `dofs` for a system formed
    /// at 2^b. This stiffness is formed at 2^b, `u` holds the solve's
    /// displacements (never scaled) and `force` is the case's **unscaled**
    /// ledger force, whose terms are taken at 2^b
    /// (`AssembledForce::force_scaled`).
    /// - The formed row `K * u` (`multiply`'s binary64 row, the same bits) is
    ///   checked at 2^b (RV11-1, ROOT's rulings on RV11's review): each product
    ///   of nonzero operands, and each partial sum, must be normal (a partial
    ///   sum may also be an exact zero). Otherwise the reaction is refused
    ///   with step 5's `PublicationOutsideBinary64`; it is never published.
    ///   With every value normal, the row is exactly 2^b times the row an
    ///   unbounded exponent range would give, so no bit is lost to the scale.
    /// - Each reaction is then one exact sum of that row and the DOF's terms
    ///   at 2^b, rounded once at 2^-b, with step 5's outcome: normal exact,
    ///   subnormal with its stated precision, and a nonzero underflow or an
    ///   overflow refused (never flushed). An exact zero is +0.0, as in
    ///   `reactions`.
    /// - The check applies at every b, b = 0 included, so no row that left
    ///   the normal range is ever published. Every value published at b = 0
    ///   has the bits of `reactions`.
    pub fn force_scaled_reactions(
        &self,
        u: &[f64],
        force: &AssembledForce,
        force_scale: ForceScale,
        dofs: &[usize],
    ) -> Result<Vec<PublishedValue>, ForceScaledError> {
        if force.len() != self.dimension() {
            return Err(ForceScaledError::Structural(StructuralError::InvalidInput(
                "force vector",
            )));
        }
        if dofs.iter().any(|&dof| dof >= self.dimension()) {
            return Err(ForceScaledError::Structural(StructuralError::InvalidInput(
                "reaction DOF",
            )));
        }
        let owned;
        let scaled = if force_scale.is_unscaled() {
            force
        } else {
            owned = force
                .force_scaled(force_scale)
                .map_err(ForceScaledError::refused)?;
            &owned
        };
        let internal = self.multiply(u).map_err(ForceScaledError::Structural)?;
        dofs.iter()
            .map(|&dof| {
                let outside = ForceScaleReason::PublicationOutsideBinary64 {
                    global_dof: Some(dof),
                };
                if !self.row_product_stays_normal(dof, u) {
                    return Err(ForceScaledError::refused(outside));
                }
                let mut exact = ExactAccumulator::new();
                exact
                    .add(internal[dof])
                    .and_then(|()| scaled.accumulate_dof(dof, &mut exact, true))
                    .map_err(|_| ForceScaledError::refused(outside))?;
                publication_outcome(&exact, -force_scale.exponent(), 0.0, Some(dof))
                    .map_err(ForceScaledError::refused)
            })
            .collect()
    }

    /// K2b (RV11-1): whether row `row`'s formed `K * u`, in `multiply`'s
    /// order, stays in the normal range: every product of nonzero operands is
    /// normal, and every partial sum is normal or an exact zero. (A partial
    /// sum is zero only when the exact sum is zero.) The appended signed
    /// zeros of `multiply` cannot change a magnitude, so they are not
    /// re-checked. `u` has the stiffness's dimension (`multiply` checked it).
    fn row_product_stays_normal(&self, row: usize, u: &[f64]) -> bool {
        let mut partial = 0.0_f64;
        for (j, k) in self.row(row) {
            let product = k * u[j];
            if k != 0.0 && u[j] != 0.0 && !product.is_normal() {
                return false;
            }
            partial += product;
            if partial != 0.0 && !partial.is_normal() {
                return false;
            }
        }
        true
    }
}

// ------------------------------------------------------------------ assembly

/// Options of the sparse assembly entry. K2b adds the formation-time scale b
/// here, so callers built with `SparseAssemblyOptions::new()` need no change.
#[derive(Debug, Clone, Default, PartialEq)]
#[non_exhaustive]
pub struct SparseAssemblyOptions {
    /// K2b (D1 §4.7, formation-time scaling): 2^b, even; `UNSCALED` by default.
    force_scale: ForceScale,
}

impl SparseAssemblyOptions {
    /// Today's formation: unscaled.
    pub fn new() -> Self {
        Self {
            force_scale: ForceScale::UNSCALED,
        }
    }

    /// K2b: form every frame (E and G), user element, block and spring at
    /// 2^b, exactly (`FrameElement::force_scaled`, `force_scaled_matrix`,
    /// `force_scaled_value`).
    pub fn with_force_scale(mut self, force_scale: ForceScale) -> Self {
        self.force_scale = force_scale;
        self
    }

    pub fn force_scale(&self) -> ForceScale {
        self.force_scale
    }
}

/// K2b: the inputs of `assemble_sparse_stiffness` formed at 2^b.
type ForceScaledInputs = (
    Vec<FrameElement>,
    Vec<UserStiffnessElement>,
    Vec<StiffnessBlock>,
    Vec<(usize, f64)>,
);

/// K2b: the frames (E and G), user elements, blocks and springs times 2^b,
/// exactly, in their given order. A value that cannot stay normal is
/// `NumericalRange` with the scaled operand's name.
fn force_scaled_inputs(
    frames: &[FrameElement],
    users: &[UserStiffnessElement],
    blocks: &[StiffnessBlock],
    springs: &[(usize, f64)],
    force_scale: ForceScale,
) -> Result<ForceScaledInputs, FrameKernelError> {
    let frames = frames
        .iter()
        .map(|element| element.force_scaled(force_scale))
        .collect::<Result<Vec<_>, _>>()?;
    let users = users
        .iter()
        .map(|element| element.force_scaled(force_scale))
        .collect::<Result<Vec<_>, _>>()?;
    let blocks = blocks
        .iter()
        .map(|block| {
            Ok(StiffnessBlock {
                node_i: block.node_i,
                node_j: block.node_j,
                stiffness: force_scaled_matrix(
                    "block entry*2^b (force scale)",
                    &block.stiffness,
                    force_scale,
                )?,
            })
        })
        .collect::<Result<Vec<_>, FrameKernelError>>()?;
    let springs = springs
        .iter()
        .map(|&(dof, stiffness)| {
            Ok((
                dof,
                force_scaled_value("spring stiffness*2^b (force scale)", stiffness, force_scale)?,
            ))
        })
        .collect::<Result<Vec<_>, FrameKernelError>>()?;
    Ok((frames, users, blocks, springs))
}

/// An element given by its global 12×12 matrix (a realized curved bend, or
/// any explicit element), added after the frames and user elements, as the
/// product adds its curved contributions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StiffnessBlock {
    pub node_i: usize,
    pub node_j: usize,
    pub stiffness: Matrix12,
}

fn check_node(node_index: usize, node_count: usize) -> Result<(), FrameKernelError> {
    if node_index >= node_count {
        return Err(FrameKernelError::InvalidNodeIndex {
            node_index,
            node_count,
        });
    }
    Ok(())
}

/// The kernel's sparse assembly, per modulus basis (D1 §4.8): the global
/// stiffness of `frames`, `users`, `blocks` and ground `springs` (global DOF,
/// stiffness) on its connectivity pattern.
///
/// It is the sparse sibling of `assemble_global_stiffness_with_user_elements`
/// followed by the product's curved and spring additions, and it keeps their
/// semantics:
/// - each element is formed by the same call (`global_stiffness`) in the same
///   order, after the same node check, so a formation refusal is the same
///   error for the same first element, before any value is accumulated;
/// - values accumulate in the dense order (frames, users, blocks, springs);
/// - after the frames and users, the first non-finite entry in row-major order
///   is refused as `NonFiniteInput { name: "assembled stiffness" }`; blocks
///   and springs are then added unchecked, as the product adds them.
///
/// A block or spring outside the model is refused as `InvalidNodeIndex`
/// (the dense additions would index out of bounds).
pub fn assemble_sparse_stiffness(
    node_count: usize,
    frames: &[FrameElement],
    users: &[UserStiffnessElement],
    blocks: &[StiffnessBlock],
    springs: &[(usize, f64)],
    options: &SparseAssemblyOptions,
) -> Result<SparseStiffness, FrameKernelError> {
    assemble_sparse_stiffness_with_connectors(
        node_count,
        frames,
        users,
        &[],
        blocks,
        springs,
        options,
    )
}

/// T4-U3 (S2): `assemble_sparse_stiffness` with objective connectors, each
/// added as its formed Ke (`ObjectiveConnector::global_stiffness`) after the
/// user elements and before the finiteness check, in the dense order
/// (frames, users, connectors, blocks, springs), so the entries are the
/// dense `assemble_global_stiffness_with_connectors` entries bit for bit.
/// K2b (N-5): at 2^b each connector adds its Ke formed at b = 0 times 2^b
/// (`force_scaled_matrix`), as a realized curved bend does. With no
/// connector it is `assemble_sparse_stiffness`, bit for bit.
pub fn assemble_sparse_stiffness_with_connectors(
    node_count: usize,
    frames: &[FrameElement],
    users: &[UserStiffnessElement],
    connectors: &[ObjectiveConnector],
    blocks: &[StiffnessBlock],
    springs: &[(usize, f64)],
    options: &SparseAssemblyOptions,
) -> Result<SparseStiffness, FrameKernelError> {
    let mut connector_matrices = Vec::with_capacity(connectors.len());
    for connector in connectors {
        connector_matrices.push((
            connector.node_i().index,
            connector.node_j().index,
            connector.force_scaled_global_stiffness(options.force_scale)?,
        ));
    }
    assemble_sparse_formed(
        node_count,
        frames,
        users,
        &connector_matrices,
        blocks,
        springs,
        options,
    )
}

/// The sparse assembly with the connectors' matrices already formed (at the
/// options' 2^b).
fn assemble_sparse_formed(
    node_count: usize,
    frames: &[FrameElement],
    users: &[UserStiffnessElement],
    connectors: &[(usize, usize, Matrix12)],
    blocks: &[StiffnessBlock],
    springs: &[(usize, f64)],
    options: &SparseAssemblyOptions,
) -> Result<SparseStiffness, FrameKernelError> {
    let SparseAssemblyOptions { force_scale } = options;
    if !force_scale.is_unscaled() {
        // K2b: the same assembly, of the inputs formed at 2^b.
        let (frames, users, blocks, springs) =
            force_scaled_inputs(frames, users, blocks, springs, *force_scale)?;
        return assemble_sparse_formed(
            node_count,
            &frames,
            &users,
            connectors,
            &blocks,
            &springs,
            &SparseAssemblyOptions::new(),
        );
    }
    let dimension = node_count * DOF_PER_NODE;
    let mut formed: Vec<(usize, usize, Matrix12)> = Vec::with_capacity(frames.len() + users.len());
    for element in frames {
        check_node(element.node_i.index, node_count)?;
        check_node(element.node_j.index, node_count)?;
        formed.push((
            element.node_i.index,
            element.node_j.index,
            element.global_stiffness()?,
        ));
    }
    for element in users {
        check_node(element.node_i.index, node_count)?;
        check_node(element.node_j.index, node_count)?;
        formed.push((
            element.node_i.index,
            element.node_j.index,
            element.global_stiffness()?,
        ));
    }
    for &(node_i, node_j, matrix) in connectors {
        check_node(node_i, node_count)?;
        check_node(node_j, node_count)?;
        formed.push((node_i, node_j, matrix));
    }
    for block in blocks {
        check_node(block.node_i, node_count)?;
        check_node(block.node_j, node_count)?;
    }
    for &(dof, _) in springs {
        check_node(dof / DOF_PER_NODE, node_count)?;
    }
    let connectivity: Vec<(usize, usize)> = formed
        .iter()
        .map(|&(a, b, _)| (a, b))
        .chain(blocks.iter().map(|b| (b.node_i, b.node_j)))
        .collect();
    let spring_dofs: Vec<usize> = springs.iter().map(|&(dof, _)| dof).collect();
    let pattern = SparsePattern::from_connectivity(node_count, &connectivity, &spring_dofs)
        .expect("nodes and DOFs checked above");
    debug_assert_eq!(pattern.dimension(), dimension);
    let mut values = vec![0.0; pattern.entry_count()];
    for (a, b, matrix) in &formed {
        scatter_block(&pattern, &mut values, *a, *b, matrix);
    }
    // V-K seeded fault VK-F08 (§7.3-8): member 1's i–j coupling block left
    // out, in sparse mode only (the dense assembly keeps it).
    #[cfg(any(test, feature = "mutation-controls"))]
    if super::retained::seeded::active(super::retained::seeded::Fault::F08) {
        if let Some(&(a, b, _)) = formed.first() {
            let map = element_dof_map(a, b);
            for r in 0..6 {
                for c in 6..ELEMENT_DOF {
                    for (row, col) in [(map[r], map[c]), (map[c], map[r])] {
                        let index = pattern.find(row, col).expect("element block in pattern");
                        values[index] = 0.0;
                    }
                }
            }
        }
    }
    for row in 0..dimension {
        for index in pattern.row_range(row) {
            let value = values[index];
            if !value.is_finite() {
                return Err(FrameKernelError::NonFiniteInput {
                    name: "assembled stiffness",
                    value,
                });
            }
        }
    }
    for block in blocks {
        scatter_block(
            &pattern,
            &mut values,
            block.node_i,
            block.node_j,
            &block.stiffness,
        );
    }
    for &(dof, stiffness) in springs {
        let index = pattern.find(dof, dof).expect("spring diagonal in pattern");
        values[index] += stiffness;
    }
    Ok(SparseStiffness { pattern, values })
}

/// Adds one element matrix: each of its 144 entries is one addition to its
/// pattern entry (the dense `assemble_element_contribution`).
fn scatter_block(
    pattern: &SparsePattern,
    values: &mut [f64],
    node_i: usize,
    node_j: usize,
    matrix: &Matrix12,
) {
    let map = element_dof_map(node_i, node_j);
    for (local_row, &row) in map.iter().enumerate() {
        for local_col in 0..ELEMENT_DOF {
            let index = pattern
                .find(row, map[local_col])
                .expect("element block in pattern");
            values[index] += matrix[local_row][local_col];
        }
    }
}

// ------------------------------------------------------------------ reduction

/// The partition of a sparse system into free and prescribed DOFs, with the
/// reduced right-hand side (KS2), over the pattern.
#[derive(Debug, PartialEq)]
pub struct SparseReducedSystem {
    /// Free DOFs, ascending (the dense reduction's order).
    pub free_dofs: Vec<usize>,
    /// Every boundary DOF with its value (0.0 for a restrained DOF).
    pub prescribed: Vec<(usize, f64)>,
    /// `f_i - sum_c K_ic g_c` per free row, one exact sum rounded once:
    /// bit-identical to `reduce_assembled_system*`'s force.
    pub force: ReducedForce,
    free_position: Vec<Option<usize>>,
}

impl SparseReducedSystem {
    /// The reduced index of a global DOF, if free.
    pub fn free_position(&self, dof: usize) -> Option<usize> {
        self.free_position.get(dof).copied().flatten()
    }
}

/// The sparse sibling of `reduce_assembled_system` (`displacements: None`,
/// restrained DOFs at zero) and of
/// `reduce_assembled_system_with_prescribed_displacements` (`Some`), with
/// their errors in their order. The reduced stiffness is not materialized: the
/// pattern-taking solves read `K_ff` and `K_fc` from the global pattern.
pub fn reduce_assembled_sparse_system(
    stiffness: &SparseStiffness,
    force: &AssembledForce,
    boundary_dofs: &[usize],
    displacements: Option<&[f64]>,
) -> Result<SparseReducedSystem, FrameKernelError> {
    let size = stiffness.dimension();
    for row in 0..size {
        for (_, value) in stiffness.row(row) {
            if !value.is_finite() {
                return Err(FrameKernelError::NonFiniteInput {
                    name: "matrix/vector entry",
                    value,
                });
            }
        }
    }
    let values = force.values();
    if values.len() != size {
        return Err(FrameKernelError::InvalidVectorLength {
            expected: size,
            actual: values.len(),
        });
    }
    if let Some(&value) = values.iter().find(|v| !v.is_finite()) {
        return Err(FrameKernelError::NonFiniteInput {
            name: "matrix/vector entry",
            value,
        });
    }
    let zeros = vec![0.0; boundary_dofs.len()];
    let prescribed_kind = displacements.is_some();
    let displacements = displacements.unwrap_or(&zeros);
    if displacements.len() != boundary_dofs.len() {
        return Err(FrameKernelError::InvalidVectorLength {
            expected: boundary_dofs.len(),
            actual: displacements.len(),
        });
    }
    if let Some(&value) = displacements.iter().find(|v| !v.is_finite()) {
        return Err(FrameKernelError::NonFiniteInput {
            name: "prescribed displacement",
            value,
        });
    }
    let mut boundary_value: Vec<Option<f64>> = vec![None; size];
    for (&dof, &g) in boundary_dofs.iter().zip(displacements) {
        if dof >= size {
            return Err(if prescribed_kind {
                FrameKernelError::PrescribedDofOutOfRange {
                    dof,
                    total_dofs: size,
                }
            } else {
                FrameKernelError::RestrainedDofOutOfRange {
                    dof,
                    total_dofs: size,
                }
            });
        }
        if boundary_value[dof].is_some() {
            return Err(if prescribed_kind {
                FrameKernelError::RepeatedPrescribedDof { dof }
            } else {
                FrameKernelError::RepeatedRestrainedDof { dof }
            });
        }
        boundary_value[dof] = Some(g);
    }
    let free_dofs: Vec<usize> = (0..size).filter(|&d| boundary_value[d].is_none()).collect();
    let mut free_position = vec![None; size];
    for (r, &dof) in free_dofs.iter().enumerate() {
        free_position[dof] = Some(r);
    }
    let mut rows = Vec::with_capacity(free_dofs.len());
    for &row in &free_dofs {
        // KS2 on ledger rows: exact, so only the row's stored couplings to
        // boundary DOFs matter (an absent K_ic adds nothing).
        let mut accumulator = ExactAccumulator::new();
        let sum_error = |_| FrameKernelError::NonFiniteInput {
            name: "adjusted force",
            value: f64::INFINITY,
        };
        force
            .accumulate_dof(row, &mut accumulator, false)
            .map_err(sum_error)?;
        for (col, k) in stiffness.row(row) {
            if let Some(g) = boundary_value[col] {
                accumulator.add_product(-k, g).map_err(sum_error)?;
            }
        }
        rows.push(
            accumulator
                .round()
                .map_err(|_| FrameKernelError::NonFiniteInput {
                    name: "adjusted force",
                    value: if accumulator.signum() < 0 {
                        f64::NEG_INFINITY
                    } else {
                        f64::INFINITY
                    },
                })?,
        );
    }
    Ok(SparseReducedSystem {
        free_dofs,
        prescribed: boundary_dofs
            .iter()
            .copied()
            .zip(displacements.iter().copied())
            .collect(),
        force: ReducedForce::from_exact_rows(rows),
        free_position,
    })
}

// ------------------------------------------------------------------ system

/// Per-pattern-entry symmetry provenance (the sparse `SymmetryEvidence`).
#[derive(Debug)]
pub struct SparseSymmetryEvidence<'a> {
    pub absolute_roundoff: &'a [f64],
    pub operation_counts: &'a [usize],
    pub basis: &'a str,
}

/// The sparse sibling of `StructuralSystem`. All global DOFs must occur
/// exactly once in `free_dofs` or `prescribed`; contributions, when supplied,
/// contain both triangles, before coalescence, on the stiffness's pattern.
#[derive(Debug)]
pub struct SparseStructuralSystem<'a> {
    stiffness: &'a SparseStiffness,
    force: &'a [f64],
    free_dofs: &'a [usize],
    prescribed: &'a [(usize, f64)],
    contributions: Option<&'a [StiffnessContribution]>,
    symmetry: Option<SparseSymmetryEvidence<'a>>,
    /// Position in `prescribed` of each global DOF (`usize::MAX` if none).
    prescribed_position: Vec<usize>,
    /// The number of prescribed values with the sign bit set.
    negative_prescribed: usize,
}

impl<'a> SparseStructuralSystem<'a> {
    pub fn new(
        stiffness: &'a SparseStiffness,
        force: &'a [f64],
        free_dofs: &'a [usize],
        prescribed: &'a [(usize, f64)],
        contributions: Option<&'a [StiffnessContribution]>,
        symmetry: Option<SparseSymmetryEvidence<'a>>,
    ) -> Self {
        let mut prescribed_position = vec![usize::MAX; force.len()];
        for (position, &(dof, _)) in prescribed.iter().enumerate() {
            if dof < force.len() && prescribed_position[dof] == usize::MAX {
                prescribed_position[dof] = position;
            }
        }
        let negative_prescribed = prescribed
            .iter()
            .filter(|&&(_, g)| g.is_sign_negative())
            .count();
        Self {
            stiffness,
            force,
            free_dofs,
            prescribed,
            contributions,
            symmetry,
            prescribed_position,
            negative_prescribed,
        }
    }
    /// The typed constructor (S11-K): the force vector is the ledger's.
    pub fn assembled(
        stiffness: &'a SparseStiffness,
        force: &'a AssembledForce,
        free_dofs: &'a [usize],
        prescribed: &'a [(usize, f64)],
        contributions: Option<&'a [StiffnessContribution]>,
        symmetry: Option<SparseSymmetryEvidence<'a>>,
    ) -> AssembledSparseStructuralSystem<'a> {
        AssembledSparseStructuralSystem {
            system: Self::new(
                stiffness,
                force.values(),
                free_dofs,
                prescribed,
                contributions,
                symmetry,
            ),
            force,
        }
    }
    /// Attaches the formation source the D-5 check re-forms from (K-D5).
    pub fn with_formation_source(
        self,
        source: &'a FormationSource,
    ) -> FormationCheckedSparseSystem<'a> {
        FormationCheckedSparseSystem {
            system: self,
            force: None,
            source,
        }
    }
    pub fn stiffness(&self) -> &'a SparseStiffness {
        self.stiffness
    }
    pub fn force(&self) -> &'a [f64] {
        self.force
    }
    pub fn free_dofs(&self) -> &'a [usize] {
        self.free_dofs
    }
    pub fn prescribed(&self) -> &'a [(usize, f64)] {
        self.prescribed
    }
    /// The (position in `prescribed`, K_ic, g_c) of row `i`'s stored entries
    /// in prescribed columns, in `prescribed` order.
    fn prescribed_couplings(&self, i: usize) -> Vec<(usize, f64, f64)> {
        let mut couplings: Vec<(usize, f64, f64)> = self
            .stiffness
            .row(i)
            .filter_map(|(j, k)| {
                let position = *self.prescribed_position.get(j)?;
                (position != usize::MAX).then(|| (position, k, self.prescribed[position].1))
            })
            .collect();
        couplings.sort_unstable_by_key(|&(position, _, _)| position);
        couplings
    }
}

/// The typed sparse system (S11-K's `AssembledStructuralSystem`).
#[derive(Debug)]
pub struct AssembledSparseStructuralSystem<'a> {
    system: SparseStructuralSystem<'a>,
    force: &'a AssembledForce,
}

impl<'a> AssembledSparseStructuralSystem<'a> {
    pub fn system(&self) -> &SparseStructuralSystem<'a> {
        &self.system
    }
    pub fn force(&self) -> &'a AssembledForce {
        self.force
    }
    /// Typed sibling of `SparseStructuralSystem::with_formation_source`.
    pub fn with_formation_source(
        self,
        source: &'a FormationSource,
    ) -> FormationCheckedSparseSystem<'a> {
        FormationCheckedSparseSystem {
            system: self.system,
            force: Some(self.force),
            source,
        }
    }
}

/// A sparse system with its formation source (K-D5's `FormationCheckedSystem`).
#[derive(Debug)]
pub struct FormationCheckedSparseSystem<'a> {
    system: SparseStructuralSystem<'a>,
    force: Option<&'a AssembledForce>,
    source: &'a FormationSource,
}

impl<'a> FormationCheckedSparseSystem<'a> {
    pub fn system(&self) -> &SparseStructuralSystem<'a> {
        &self.system
    }
    pub fn source(&self) -> &'a FormationSource {
        self.source
    }
    fn binding(&self) -> ForceBinding<'a> {
        match self.force {
            Some(force) => ForceBinding::Assembled(force),
            None => ForceBinding::Legacy,
        }
    }
}

/// `validate` over the pattern: the same checks, in the same order, with the
/// same errors. The symmetry audit visits stored pairs below the diagonal in
/// row-major order, which is the dense order of every pair that can differ.
fn validate_sparse(system: &SparseStructuralSystem<'_>) -> Result<(), StructuralError> {
    let n = system.force.len();
    let stiffness = system.stiffness;
    let pattern = &stiffness.pattern;
    if pattern.dimension != n {
        return Err(StructuralError::InvalidInput("matrix dimensions"));
    }
    if system
        .force
        .iter()
        .chain(stiffness.values.iter())
        .any(|x| !x.is_finite())
    {
        return Err(StructuralError::InvalidInput(
            "nonfinite original equations",
        ));
    }
    let nnz = pattern.entry_count();
    if let Some(evidence) = &system.symmetry {
        if evidence.basis.is_empty()
            || evidence.absolute_roundoff.len() != nnz
            || evidence.operation_counts.len() != nnz
            || evidence
                .absolute_roundoff
                .iter()
                .any(|v| !v.is_finite() || *v < 0.0)
        {
            return Err(StructuralError::InvalidInput("symmetry provenance"));
        }
    }
    for i in 0..n {
        for index in pattern.row_range(i) {
            let j = pattern.columns[index];
            if j >= i {
                break;
            }
            let mirror = pattern.transpose[index];
            let a = stiffness.values[index];
            let b = stiffness.values[mirror];
            if a == b {
                continue;
            }
            let scale = a.abs().max(b.abs());
            let skew = (a / scale - b / scale).abs();
            let allowed = system.symmetry.as_ref().map_or(0.0, |e| {
                e.absolute_roundoff[index] / scale + e.absolute_roundoff[mirror] / scale
            });
            if skew > allowed {
                return Err(StructuralError::Asymmetric {
                    row: i,
                    col: j,
                    relative_skew: skew,
                });
            }
        }
    }
    let mut seen = vec![false; n];
    for &i in system.free_dofs {
        if i >= n || seen[i] {
            return Err(StructuralError::InvalidInput("free map"));
        }
        seen[i] = true;
    }
    for &(i, v) in system.prescribed {
        if i >= n || seen[i] || !v.is_finite() {
            return Err(StructuralError::InvalidInput("prescribed map"));
        }
        seen[i] = true;
    }
    if seen.iter().any(|x| !*x) {
        return Err(StructuralError::InvalidInput("incomplete constraint map"));
    }
    Ok(())
}

impl Represented for SparseStructuralSystem<'_> {
    /// One exact expansion per global pattern entry.
    type Sums = Vec<Expansion>;
    fn view(&self) -> Equations<'_> {
        Equations {
            force: self.force,
            free_dofs: self.free_dofs,
            prescribed: self.prescribed,
        }
    }
    fn contributions_supplied(&self) -> bool {
        self.contributions.is_some()
    }
    fn validate_equations(&self) -> Result<(), StructuralError> {
        validate_sparse(self)
    }
    fn row(&self, i: usize) -> impl Iterator<Item = (usize, f64)> + Clone + '_ {
        self.stiffness.row(i)
    }
    fn prescribed_coupled(&self, i: usize) -> bool {
        self.stiffness.row(i).any(|(j, k)| {
            let position = self
                .prescribed_position
                .get(j)
                .copied()
                .unwrap_or(usize::MAX);
            position != usize::MAX && k != 0.0 && self.prescribed[position].1 != 0.0
        })
    }
    fn contribution_sums(&self) -> Result<Option<Self::Sums>, StructuralError> {
        sparse_contribution_sums(self)
    }
    fn sums_row<'b>(
        &'b self,
        sums: &'b Self::Sums,
        i: usize,
    ) -> impl Iterator<Item = (usize, &'b Expansion)> + Clone + 'b {
        let pattern = &self.stiffness.pattern;
        pattern
            .row_range(i)
            .map(move |index| (pattern.columns[index], &sums[index]))
    }
    fn load_fidelity(
        &self,
        u: &[f64],
        terms: &[ForceTerm],
    ) -> Result<LoadFidelityReport, StructuralError> {
        audit_sparse_load_fidelity(self, u, terms)
    }
    fn formation_check<F>(
        &self,
        source: &FormationSource,
        force_terms: Option<&[ForceTerm]>,
        scale_exponents: &[i32],
        u: &[f64],
        solve: &F,
    ) -> Option<FormationCheck>
    where
        F: Fn(&[f64]) -> Result<Vec<f64>, StructuralError>,
    {
        // The check re-forms the intended system from `source`; of the
        // equations it reads only `force` and `free_dofs` (and the attempt's
        // own factor, through `solve`), so it takes the same inputs in both
        // representations. Its stiffness view is left empty.
        let view = StructuralSystem {
            stiffness: &[],
            force: self.force,
            free_dofs: self.free_dofs,
            prescribed: self.prescribed,
            contributions: None,
            symmetry: None,
        };
        formation_check::check(&view, source, force_terms, scale_exponents, u, solve)
    }
}

/// `contribution_sums` per pattern entry (O(nnz) memory), with the same
/// absorbed-diagonal refusal, in contribution order.
fn sparse_contribution_sums(
    system: &SparseStructuralSystem<'_>,
) -> Result<Option<Vec<Expansion>>, StructuralError> {
    let Some(entries) = system.contributions else {
        return Ok(None);
    };
    let n = system.force.len();
    let pattern = &system.stiffness.pattern;
    let mut sums = vec![Expansion::default(); pattern.entry_count()];
    for entry in entries {
        if entry.row >= n || entry.col >= n || !entry.value.is_finite() {
            return Err(StructuralError::InvalidInput("contribution"));
        }
        let index = pattern
            .find(entry.row, entry.col)
            .ok_or(StructuralError::InvalidInput(
                "contribution outside pattern",
            ))?;
        let sum = &mut sums[index];
        let old = sum.round()?;
        sum.add(entry.value)?;
        if entry.row == entry.col && entry.value > 0.0 && old != 0.0 && sum.round()? == old {
            return Err(unresolved(
                "positive diagonal contribution absorbed by assembly; stabilization unresolved",
                Some(entry.row),
            ));
        }
    }
    Ok(Some(sums))
}

// ------------------------------------------------------------------ prepared

/// The sparse sibling of `PreparedSystem`: the scaled, symmetrized free block
/// `A = T K_ff T` in compressed rows over free positions (columns ascending),
/// with the right-hand side and the preparation's audit.
#[derive(Debug)]
pub struct SparsePreparedSystem<'s> {
    source: &'s SparseStructuralSystem<'s>,
    force_binding: ForceBinding<'s>,
    formation: Option<&'s FormationSource>,
    row_starts: Vec<usize>,
    columns: Vec<usize>,
    values: Vec<f64>,
    transpose: Vec<usize>,
    /// The global pattern entry `(free[r], free[c])` of each prepared entry.
    source_entries: Vec<usize>,
    diagonal: Vec<usize>,
    rhs: Vec<f64>,
    scale_exponents: Vec<i32>,
    assembly_relative_perturbation_estimate: f64,
    assembly_load_perturbation_estimate: f64,
    contribution_rounding: Vec<ContributionRounding>,
    symmetry_projection_performed: bool,
    maximum_scaled_skew: f64,
    symmetry_basis: Option<String>,
}

impl SparsePreparedSystem<'_> {
    pub fn free_dofs(&self) -> &[usize] {
        self.source.free_dofs
    }
    pub fn scale_exponents(&self) -> &[i32] {
        &self.scale_exponents
    }
    /// The free block's dimension.
    pub fn dimension(&self) -> usize {
        self.source.free_dofs.len()
    }
    /// Stored entries of the free block (both triangles).
    pub fn entry_count(&self) -> usize {
        self.columns.len()
    }
    /// The stored entries on or below the diagonal as (row, column, value),
    /// row-major, in free positions; explicit zeros included.
    pub fn lower_entries(&self) -> impl Iterator<Item = (usize, usize, f64)> + '_ {
        (0..self.dimension()).flat_map(move |row| {
            self.row_range(row)
                .map(move |index| (row, self.columns[index], self.values[index]))
                .filter(move |&(row, col, _)| col <= row)
        })
    }
    fn row_range(&self, row: usize) -> std::ops::Range<usize> {
        self.row_starts[row]..self.row_starts[row + 1]
    }
}

/// Nearby radix diagonal equilibration over the pattern (`prepare_structural`).
pub fn prepare_sparse_structural<'s>(
    system: &'s SparseStructuralSystem<'s>,
) -> Result<SparsePreparedSystem<'s>, StructuralError> {
    prepare_sparse_bound(system, ForceBinding::Legacy)
}
/// Typed sibling (`prepare_assembled_structural`): KS1 sums the ledger terms exactly.
pub fn prepare_assembled_sparse_structural<'s>(
    system: &'s AssembledSparseStructuralSystem<'s>,
) -> Result<SparsePreparedSystem<'s>, StructuralError> {
    prepare_sparse_bound(&system.system, ForceBinding::Assembled(system.force))
}
/// K-D5 sibling (`prepare_formation_checked_structural`).
pub fn prepare_formation_checked_sparse_structural<'s>(
    system: &'s FormationCheckedSparseSystem<'s>,
) -> Result<SparsePreparedSystem<'s>, StructuralError> {
    let mut prepared = prepare_sparse_bound(&system.system, system.binding())?;
    prepared.formation = Some(system.source);
    Ok(prepared)
}
/// The C3-detect sibling (`prepare_structural_with_force_terms`).
pub fn prepare_sparse_structural_with_force_terms<'s>(
    system: &'s SparseStructuralSystem<'s>,
    force_terms: &'s [ForceTerm],
) -> Result<SparsePreparedSystem<'s>, StructuralError> {
    prepare_sparse_bound(system, ForceBinding::AuditTerms(force_terms))
}

fn prepare_sparse_bound<'s>(
    system: &'s SparseStructuralSystem<'s>,
    force_binding: ForceBinding<'s>,
) -> Result<SparsePreparedSystem<'s>, StructuralError> {
    if force_binding.binary64() {
        // The option-(c) binary64 fold is not offered on the pattern: the
        // nonlinear loop stays on the dense `_binary64` path until F1b.
        return Err(StructuralError::InvalidInput(
            "binary64 binding on the sparse representation",
        ));
    }
    validate_sparse(system)?;
    let stiffness = system.stiffness;
    let pattern = &stiffness.pattern;
    let n = system.free_dofs.len();
    let mut exponents = Vec::with_capacity(n);
    for &i in system.free_dofs {
        let d = stiffness.get(i, i);
        if d < 0.0 {
            let mut direction = vec![0.0; system.force.len()];
            direction[i] = 1.0;
            return Err(StructuralError::NegativeEnergy {
                direction,
                energy: d,
                allowance: 0.0,
            });
        }
        if d == 0.0 {
            return Err(unresolved(
                "zero original diagonal; no physical nullity inferred",
                Some(i),
            ));
        }
        exponents.push(-binary_exponent(d).div_euclid(2));
    }
    let mut free_position = vec![usize::MAX; system.force.len()];
    for (r, &i) in system.free_dofs.iter().enumerate() {
        free_position[i] = r;
    }
    let mut row_starts = Vec::with_capacity(n + 1);
    row_starts.push(0);
    let mut columns = Vec::new();
    let mut values = Vec::new();
    let mut source_entries = Vec::new();
    let mut diagonal = Vec::with_capacity(n);
    let mut rhs = vec![0.0; n];
    for (r, &i) in system.free_dofs.iter().enumerate() {
        let mut entries: Vec<(usize, usize)> = pattern
            .row_range(i)
            .filter_map(|index| {
                let c = free_position[pattern.columns[index]];
                (c != usize::MAX).then_some((c, index))
            })
            .collect();
        entries.sort_unstable_by_key(|&(c, _)| c);
        let start = columns.len();
        for (c, index) in entries {
            values.push(radix_scale(
                stiffness.values[index],
                exponents[r] + exponents[c],
            )?);
            columns.push(c);
            source_entries.push(index);
        }
        diagonal.push(
            start
                + columns[start..].binary_search(&r).map_err(|_| {
                    unresolved(
                        "zero original diagonal; no physical nullity inferred",
                        Some(i),
                    )
                })?,
        );
        row_starts.push(columns.len());
        let couplings = system.prescribed_couplings(i);
        let coupled = couplings.iter().any(|&(_, k, g)| k != 0.0 && g != 0.0);
        if force_binding.assembled().is_some() || coupled {
            rhs[r] = exact_scaled_rhs(
                system.view(),
                force_binding,
                i,
                couplings.iter().map(|&(_, k, g)| (k, g)),
                exponents[r],
            )?;
        } else {
            // No nonzero prescribed product: every `b - K_ij g_j` of the
            // dense fold subtracts a signed zero, which is exact. Its only
            // effects are refusing a subnormal b at the first subtraction and
            // turning b = -0.0 into +0.0 when some product is -0.0.
            let b = legacy_zero_product_fold(system, i, &couplings)?;
            rhs[r] = radix_scale(b, exponents[r])?;
        }
    }
    let mut transpose = Vec::with_capacity(columns.len());
    for r in 0..n {
        for index in row_starts[r]..row_starts[r + 1] {
            let c = columns[index];
            let position = columns[row_starts[c]..row_starts[c + 1]]
                .binary_search(&r)
                .map_err(|_| StructuralError::InvalidInput("prepared pattern symmetry"))?;
            transpose.push(row_starts[c] + position);
        }
    }
    let nnz = pattern.entry_count();
    if let Some(evidence) = &system.symmetry {
        if evidence.basis.is_empty()
            || evidence.absolute_roundoff.len() != nnz
            || evidence.operation_counts.len() != nnz
            || evidence
                .absolute_roundoff
                .iter()
                .any(|v| !v.is_finite() || *v < 0.0)
        {
            return Err(StructuralError::InvalidInput("symmetry provenance"));
        }
    }
    let mut projection = false;
    let mut max_skew: f64 = 0.0;
    for r in 0..n {
        for index in row_starts[r]..row_starts[r + 1] {
            let c = columns[index];
            if c >= r {
                break;
            }
            let mirror = transpose[index];
            let skew = checked_value(values[index] - values[mirror])?.abs();
            max_skew = max_skew.max(skew);
            if skew == 0.0 {
                continue;
            }
            let allowed = if let Some(e) = &system.symmetry {
                checked_value(
                    radix_scale(
                        e.absolute_roundoff[source_entries[index]],
                        exponents[r] + exponents[c],
                    )? + radix_scale(
                        e.absolute_roundoff[source_entries[mirror]],
                        exponents[r] + exponents[c],
                    )?,
                )?
            } else {
                0.0
            };
            if skew > allowed {
                return Err(StructuralError::Asymmetric {
                    row: system.free_dofs[r],
                    col: system.free_dofs[c],
                    relative_skew: skew / values[index].abs().max(values[mirror].abs()),
                });
            }
            let average = values[index] / 2.0 + values[mirror] / 2.0;
            values[index] = average;
            values[mirror] = average;
            projection = true;
        }
    }
    let mut prepared = SparsePreparedSystem {
        source: system,
        force_binding,
        formation: None,
        row_starts,
        columns,
        values,
        transpose,
        source_entries,
        diagonal,
        rhs,
        scale_exponents: exponents,
        assembly_relative_perturbation_estimate: 0.0,
        assembly_load_perturbation_estimate: 0.0,
        contribution_rounding: Vec::new(),
        symmetry_projection_performed: projection,
        maximum_scaled_skew: max_skew,
        symmetry_basis: system.symmetry.as_ref().map(|e| e.basis.to_owned()),
    };
    let (perturbation, load_perturbation, rounding) = sparse_audit_contributions(&prepared)?;
    prepared.assembly_relative_perturbation_estimate = perturbation;
    prepared.assembly_load_perturbation_estimate = load_perturbation;
    prepared.contribution_rounding = rounding;
    Ok(prepared)
}

/// The dense fold `b = checked_value(b - checked_product(K_ij, g_j)?)?` over
/// every prescribed (j, g_j), on a row where every product is a signed zero
/// (no stored coupling has K_ij != 0 and g_j != 0; an absent K_ij is +0.0).
/// Subtracting a signed zero is exact, so the fold reduces to: the first
/// subtraction refuses a subnormal b, as `checked_value` does; and b = -0.0
/// becomes +0.0 if some product is -0.0 (-0.0 - (-0.0) = +0.0). No other value
/// changes, so no binary64 accumulation is formed.
fn legacy_zero_product_fold(
    system: &SparseStructuralSystem<'_>,
    i: usize,
    couplings: &[(usize, f64, f64)],
) -> Result<f64, StructuralError> {
    let b = system.force[i];
    if system.prescribed.is_empty() {
        return Ok(b);
    }
    if b.is_subnormal() {
        return Err(StructuralError::Range("arithmetic outside normal range"));
    }
    if b == 0.0 && b.is_sign_negative() {
        // Products K_ij * g_j with the sign bit set: every absent column
        // contributes g_j's sign; a stored one the product's own sign.
        let all_negative = system.negative_prescribed;
        let stored_negative_g = couplings.iter().filter(|c| c.2.is_sign_negative()).count();
        let stored_negative_product = couplings
            .iter()
            .filter(|c| c.1.is_sign_negative() != c.2.is_sign_negative())
            .count();
        if all_negative - stored_negative_g + stored_negative_product > 0 {
            return Ok(0.0);
        }
    }
    Ok(b)
}

/// `audit_contributions` over the pattern, reading every term in the dense
/// order: the rounding list row-major, each free column's norms down its free
/// rows, and each row's prescribed coupling in `prescribed` order.
fn sparse_audit_contributions(
    prepared: &SparsePreparedSystem<'_>,
) -> Result<(f64, f64, Vec<ContributionRounding>), StructuralError> {
    let system = prepared.source;
    let Some(sums) = sparse_contribution_sums(system)? else {
        return Ok((0.0, 0.0, Vec::new()));
    };
    let stiffness = system.stiffness;
    let pattern = &stiffness.pattern;
    let exponents = &prepared.scale_exponents;
    let mut differences = sums.clone();
    for (index, value) in differences.iter_mut().enumerate() {
        value.add(-stiffness.values[index])?;
    }
    let mut rounding = Vec::new();
    for i in 0..pattern.dimension {
        for index in pattern.row_range(i) {
            let difference = &differences[index];
            if !difference.is_zero() {
                let terms = &sums[index].terms;
                rounding.push(ContributionRounding {
                    row: i,
                    col: pattern.columns[index],
                    accumulated_high: terms.last().copied().unwrap_or(0.0),
                    accumulated_low: terms.iter().rev().skip(1).copied().sum(),
                    stored_difference_high: difference.terms.last().copied().unwrap_or(0.0),
                    stored_difference_low: difference.terms.iter().rev().skip(1).copied().sum(),
                    accumulated_expansion: terms.clone(),
                    difference_expansion: difference.terms.clone(),
                });
            }
        }
    }
    let mut perturbation_norm: f64 = 0.0;
    let mut matrix_norm: f64 = 0.0;
    for c in 0..prepared.dimension() {
        let mut delta_column = 0.0;
        let mut matrix_column = 0.0;
        // Column c's free rows r, ascending: the prepared row c (by symmetry
        // of the pattern), whose source entry (free[c], free[r]) has the
        // transpose (free[r], free[c]).
        for index in prepared.row_range(c) {
            let r = prepared.columns[index];
            let global = pattern.transpose[prepared.source_entries[index]];
            for &part in &differences[global].terms {
                delta_column = checked_value(
                    delta_column + exact_radix(part, exponents[r] + exponents[c])?.abs(),
                )?;
            }
            matrix_column = checked_value(
                matrix_column
                    + radix_scale(stiffness.values[global], exponents[r] + exponents[c])?.abs(),
            )?;
        }
        perturbation_norm = perturbation_norm.max(delta_column);
        matrix_norm = matrix_norm.max(matrix_column);
    }
    let mut delta_rhs_norm = 0.0;
    let mut rhs_norm = 0.0;
    for (r, &i) in system.free_dofs.iter().enumerate() {
        let mut delta = Expansion::default();
        let mut couplings: Vec<(usize, usize)> = pattern
            .row_range(i)
            .filter_map(|index| {
                let position = system.prescribed_position[pattern.columns[index]];
                (position != usize::MAX).then_some((position, index))
            })
            .collect();
        couplings.sort_unstable_by_key(|&(position, _)| position);
        for (position, index) in couplings {
            let value = system.prescribed[position].1;
            for &part in &differences[index].terms {
                delta.add_product(-part, value, exponents[r])?;
            }
        }
        delta_rhs_norm =
            checked_value(delta_rhs_norm + delta.terms.iter().map(|v| v.abs()).sum::<f64>())?;
        rhs_norm = checked_value(rhs_norm + prepared.rhs[r].abs())?;
    }
    if delta_rhs_norm != 0.0 && rhs_norm == 0.0 {
        return Err(unresolved(
            "contribution-preserved prescribed coupling changes zero reduced load",
            None,
        ));
    }
    Ok((
        if matrix_norm == 0.0 {
            0.0
        } else {
            perturbation_norm / matrix_norm
        },
        if rhs_norm == 0.0 {
            0.0
        } else {
            delta_rhs_norm / rhs_norm
        },
        rounding,
    ))
}

/// `validate_prepared` over the pattern, including the internal structure, so
/// a corrupted prepared system is an error, never a panic.
fn validate_sparse_prepared(prepared: &SparsePreparedSystem<'_>) -> Result<(), StructuralError> {
    validate_sparse(prepared.source)?;
    let n = prepared.source.free_dofs.len();
    let nnz = prepared.columns.len();
    let source_nnz = prepared.source.stiffness.pattern.entry_count();
    let shape = StructuralError::InvalidInput("prepared shape/map/finiteness");
    if prepared.row_starts.len() != n + 1
        || prepared.row_starts.first() != Some(&0)
        || prepared.row_starts.last() != Some(&nnz)
        || prepared.row_starts.windows(2).any(|w| w[0] > w[1])
        || prepared.values.len() != nnz
        || prepared.transpose.len() != nnz
        || prepared.source_entries.len() != nnz
        || prepared.diagonal.len() != n
        || prepared.rhs.len() != n
        || prepared.scale_exponents.len() != n
        || prepared.values.iter().any(|v| !v.is_finite())
        || prepared.rhs.iter().any(|v| !v.is_finite())
        || prepared.source_entries.iter().any(|&k| k >= source_nnz)
    {
        return Err(shape);
    }
    for r in 0..n {
        let row = &prepared.columns[prepared.row_range(r)];
        if row.iter().any(|&c| c >= n) || row.windows(2).any(|w| w[0] >= w[1]) {
            return Err(shape);
        }
        let d = prepared.diagonal[r];
        if !prepared.row_range(r).contains(&d) || prepared.columns[d] != r {
            return Err(shape);
        }
        for index in prepared.row_range(r) {
            let t = prepared.transpose[index];
            if t >= nnz
                || prepared.transpose[t] != index
                || !prepared.row_range(prepared.columns[index]).contains(&t)
                || prepared.columns[t] != r
            {
                return Err(shape);
            }
        }
    }
    if (0..nnz).any(|index| prepared.values[index] != prepared.values[prepared.transpose[index]]) {
        return Err(StructuralError::InvalidInput("prepared symmetry"));
    }
    Ok(())
}

impl ConditionMatrix for SparsePreparedSystem<'_> {
    fn dimension(&self) -> usize {
        self.source.free_dofs.len()
    }
    fn square_and_finite(&self) -> bool {
        validate_sparse_prepared(self).is_ok()
    }
    /// Column j down its rows is row j across its columns: the prepared
    /// matrix is exactly symmetric (checked by `validate_sparse_prepared`).
    fn column(&self, j: usize) -> impl Iterator<Item = f64> + '_ {
        self.values[self.row_range(j)].iter().copied()
    }
}

impl PreparedGate for SparsePreparedSystem<'_> {
    fn gate_shape_matches(&self, n: usize) -> bool {
        self.row_starts.len() == n + 1 && self.rhs.len() == n && self.scale_exponents.len() == n
    }
    fn gate_rcond<F>(&self, solve: &F) -> Result<f64, StructuralError>
    where
        F: Fn(&[f64]) -> Result<Vec<f64>, StructuralError>,
    {
        ConditionMatrix::estimate_rcond(self, solve)
    }
    fn gate_rhs(&self) -> &[f64] {
        &self.rhs
    }
    fn gate_scale_exponents(&self) -> &[i32] {
        &self.scale_exponents
    }
    fn gate_binding(&self) -> ForceBinding<'_> {
        self.force_binding
    }
    fn gate_formation(&self) -> Option<&FormationSource> {
        self.formation
    }
    fn gate_assembly(&self) -> (f64, f64) {
        (
            self.assembly_relative_perturbation_estimate,
            self.assembly_load_perturbation_estimate,
        )
    }
    fn gate_contribution_rounding(&self) -> &[ContributionRounding] {
        &self.contribution_rounding
    }
    fn gate_symmetry(&self) -> (bool, f64, Option<String>) {
        (
            self.symmetry_projection_performed,
            self.maximum_scaled_skew,
            self.symmetry_basis.clone(),
        )
    }
}

// ------------------------------------------------------------------ factor

/// A positive skyline factor of a sparse prepared system. Like
/// `PositiveFactor`, it cannot be constructed or relabelled by callers.
pub struct SparsePositiveFactor<'p, 's> {
    prepared: &'p SparsePreparedSystem<'s>,
    factor: ProfileFactor,
    pivots: Vec<PivotEvidence>,
}

impl SparsePositiveFactor<'_, '_> {
    pub fn pivots(&self) -> &[PivotEvidence] {
        &self.pivots
    }
}

/// `factor_structural_profile` on the sparse prepared system: the caller
/// supplies only the ordering and the skyline; the profile rows are filled
/// from the stored entries (zeros elsewhere), and the factorization is the
/// shared `ProfileFactor::factor_structural_profile`.
pub fn factor_sparse_structural_profile<'p, 's>(
    prepared: &'p SparsePreparedSystem<'s>,
    order: &[usize],
    first: &[usize],
) -> Result<SparsePositiveFactor<'p, 's>, StructuralError> {
    validate_sparse_prepared(prepared)?;
    let n = prepared.dimension();
    if order.len() != n || first.len() != n {
        return Err(StructuralError::InvalidInput("profile dimensions"));
    }
    let mut seen = vec![false; n];
    for &i in order {
        if i >= n || seen[i] {
            return Err(StructuralError::InvalidInput("profile permutation"));
        }
        seen[i] = true;
    }
    let mut position = vec![0; n];
    for (k, &i) in order.iter().enumerate() {
        position[i] = k;
    }
    let mut rows = Vec::with_capacity(n);
    for i in 0..n {
        if first[i] > i {
            return Err(StructuralError::InvalidInput("profile first column"));
        }
        let mut row = vec![0.0; i - first[i] + 1];
        for index in prepared.row_range(order[i]) {
            let k = position[prepared.columns[index]];
            let value = prepared.values[index];
            if k < first[i] {
                if value != 0.0 {
                    return Err(StructuralError::InvalidInput(
                        "profile omits represented coefficient",
                    ));
                }
            } else if k <= i {
                row[k - first[i]] = value;
            }
        }
        rows.push(row);
    }
    let factor = ProfileFactor {
        first: first.to_vec(),
        rows,
        order: order.to_vec(),
    };
    let (factor, pivots) = factor.factor_structural_profile(prepared.source.free_dofs)?;
    Ok(SparsePositiveFactor {
        prepared,
        factor,
        pivots,
    })
}

/// The completion of a sparse factor: the shared `finish_checked_factor`, with
/// the skyline label of today's sparse path.
pub fn finish_sparse_structural(
    factor: &SparsePositiveFactor<'_, '_>,
) -> Result<StructuralSolution, StructuralError> {
    finish_checked_factor(
        factor.prepared.source,
        factor.prepared,
        factor.pivots.clone(),
        "positive skyline LDL",
        |rhs| factor.factor.solve(rhs),
    )
}

/// `audit_load_fidelity` on the sparse representation: the same prologue, and
/// each row through the parent's `audit_load_row` (or `unaudited_row`), with
/// the row's products built from the pattern row in ascending column order.
pub fn audit_sparse_load_fidelity(
    system: &SparseStructuralSystem<'_>,
    u: &[f64],
    force_terms: &[ForceTerm],
) -> Result<LoadFidelityReport, StructuralError> {
    validate_sparse(system)?;
    let n = system.force.len();
    if u.len() != n || u.iter().any(|x| !x.is_finite()) {
        return Err(StructuralError::InvalidInput("displacement vector"));
    }
    let mut by_dof: Vec<Vec<&ForceTerm>> = vec![Vec::new(); n];
    for term in force_terms {
        if term.dof >= n || term.source.is_empty() {
            return Err(StructuralError::InvalidInput("force term"));
        }
        by_dof[term.dof].push(term);
    }
    let sums = sparse_contribution_sums(system)?;
    let mut restrained = vec![false; n];
    for &(i, _) in system.prescribed {
        restrained[i] = true;
    }
    let pattern = &system.stiffness.pattern;
    let mut rows = Vec::new();
    for i in 0..n {
        let terms = &by_dof[i];
        let parts: Vec<(f64, f64)> = match &sums {
            Some(sums) => pattern
                .row_range(i)
                .flat_map(|index| {
                    let x = u[pattern.columns[index]];
                    sums[index].terms.iter().map(move |&k| (k, x))
                })
                .collect(),
            None => system
                .stiffness
                .row(i)
                .filter(|&(_, k)| k != 0.0)
                .map(|(j, k)| (k, u[j]))
                .collect(),
        };
        let row = match audit_load_row(system.view(), &parts, &restrained, i, terms) {
            Ok(row) => row,
            Err(StructuralError::Range(reason)) => {
                Some(unaudited_row(system.view(), &restrained, i, terms, reason))
            }
            Err(error) => return Err(error),
        };
        rows.extend(row);
    }
    Ok(LoadFidelityReport {
        rows,
        audit_error: None,
    })
}

// ------------------------------------------------------------------ witness

/// `verify_negative_direction` over the pattern: the energy uTAu sums the
/// same terms in the same (row-major) order, so the verdict is the same.
pub fn verify_sparse_negative_direction(
    prepared: &SparsePreparedSystem<'_>,
    direction: &[f64],
) -> Result<Option<StructuralError>, StructuralError> {
    validate_sparse_prepared(prepared)?;
    let n = prepared.dimension();
    if direction.len() != n || direction.iter().any(|v| !v.is_finite()) {
        return Err(StructuralError::InvalidInput("negative direction"));
    }
    let original = &prepared.source.stiffness.values;
    let mut energy = 0.0;
    let mut magnitude = 0.0;
    let mut terms = 0;
    for r in 0..n {
        for index in prepared.row_range(r) {
            let c = prepared.columns[index];
            if prepared.values[index] == 0.0 || direction[r] == 0.0 || direction[c] == 0.0 {
                continue;
            }
            let k = radix_scale(
                original[prepared.source_entries[index]],
                prepared.scale_exponents[r] + prepared.scale_exponents[c],
            )?;
            let term = checked_product(checked_product(direction[r], k)?, direction[c])?;
            energy = checked_value(energy + term)?;
            magnitude = checked_value(magnitude + term.abs())?;
            terms += 1;
        }
    }
    negative_verdict(prepared, direction, energy, magnitude, terms)
}

/// The verdict of `verify_negative_direction` from its energy sums.
fn negative_verdict(
    prepared: &SparsePreparedSystem<'_>,
    direction: &[f64],
    energy: f64,
    magnitude: f64,
    terms: usize,
) -> Result<Option<StructuralError>, StructuralError> {
    let allowance = 64.0 * gamma(3 * terms + 2) * magnitude;
    if energy < -allowance {
        let system = prepared.source;
        let mut mapped = vec![0.0; system.force.len()];
        for (i, &global) in system.free_dofs.iter().enumerate() {
            mapped[global] = radix_scale(direction[i], prepared.scale_exponents[i])?;
        }
        Ok(Some(StructuralError::NegativeEnergy {
            direction: mapped,
            energy,
            allowance,
        }))
    } else {
        Ok(None)
    }
}

/// `negative_pair_witness` over the pattern, in O(nnz): it visits only the
/// stored pairs below the diagonal, in the dense (row-major) order. A pair
/// with a zero coupling has energy a_ii + a_jj > 0 (prepared diagonals are
/// positive), so it can never be the dense search's witness; skipping the
/// unstored pairs therefore returns the dense search's first witness.
pub fn sparse_negative_pair_witness(
    prepared: &SparsePreparedSystem<'_>,
) -> Result<Option<StructuralError>, StructuralError> {
    sparse_negative_pair_witness_counted(prepared).map(|(witness, _)| witness)
}

/// `sparse_negative_pair_witness` with the number of pairs it visited.
pub(crate) fn sparse_negative_pair_witness_counted(
    prepared: &SparsePreparedSystem<'_>,
) -> Result<(Option<StructuralError>, usize), StructuralError> {
    validate_sparse_prepared(prepared)?;
    let n = prepared.dimension();
    let mut visited = 0;
    for r in 0..n {
        for index in prepared.row_range(r) {
            let c = prepared.columns[index];
            if c >= r {
                break;
            }
            visited += 1;
            let sign = if prepared.values[index] >= 0.0 {
                -1.0
            } else {
                1.0
            };
            let (energy, magnitude, terms) = pair_energy(prepared, r, c, index, sign)?;
            let allowance = 64.0 * gamma(3 * terms + 2) * magnitude;
            if energy < -allowance {
                let mut direction = vec![0.0; n];
                direction[r] = 1.0;
                direction[c] = sign;
                return Ok((
                    negative_verdict(prepared, &direction, energy, magnitude, terms)?,
                    visited,
                ));
            }
        }
    }
    Ok((None, visited))
}

/// The energy sums of the direction e_r + sign·e_c (c < r) in O(1): its only
/// terms, in the dense row-major order, are (c,c), (c,r), (r,c) and (r,r).
fn pair_energy(
    prepared: &SparsePreparedSystem<'_>,
    r: usize,
    c: usize,
    lower: usize,
    sign: f64,
) -> Result<(f64, f64, usize), StructuralError> {
    let original = &prepared.source.stiffness.values;
    let upper = prepared.transpose[lower];
    let cells = [
        (c, c, prepared.diagonal[c], sign, sign),
        (c, r, upper, sign, 1.0),
        (r, c, lower, 1.0, sign),
        (r, r, prepared.diagonal[r], 1.0, 1.0),
    ];
    let mut energy = 0.0;
    let mut magnitude = 0.0;
    let mut terms = 0;
    for (a, b, index, da, db) in cells {
        if prepared.values[index] == 0.0 {
            continue;
        }
        let k = radix_scale(
            original[prepared.source_entries[index]],
            prepared.scale_exponents[a] + prepared.scale_exponents[b],
        )?;
        let term = checked_product(checked_product(da, k)?, db)?;
        energy = checked_value(energy + term)?;
        magnitude = checked_value(magnitude + term.abs())?;
        terms += 1;
    }
    Ok((energy, magnitude, terms))
}

#[cfg(test)]
mod tests;
