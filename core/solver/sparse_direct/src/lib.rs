//! Code-neutral in-repo sparse skyline (profile) direct solver.
//!
//! This crate implements the human ruling `DEC-023` (decision D-03, Option C):
//! deterministic bandwidth/profile-reducing DOF ordering (reverse
//! Cuthill-McKee), skyline (profile) storage of the symmetric stiffness
//! matrix, and an in-repo LDL^T profile factorization and solve.
//!
//! Determinism posture: single-threaded, fixed operation order, no randomized
//! tie-breaking, Rust standard library only. The crate contains open
//! numerical-mechanics routines only. It does not encode design code
//! compliance checks, protected standards content, or private project data.

use std::collections::VecDeque;
use std::error::Error;
use std::fmt;

/// Internal sparse verification pivot guard. This mirrors the in-repo dense
/// path's `DENSE_SOLVE_ZERO_PIVOT_GUARD` precedent (1.0e-12 in
/// `core/solver/frame_kernel`). It is not the project solver tolerance
/// policy; tolerance policy is governed under D-04 (`DEC-026`).
pub const SPARSE_SOLVE_ZERO_PIVOT_GUARD: f64 = 1.0e-12;

#[derive(Debug, Clone, PartialEq)]
pub enum SparseDirectError {
    InvalidMatrixDimensions {
        rows: usize,
        cols: usize,
    },
    InvalidVectorLength {
        expected: usize,
        actual: usize,
    },
    NonFiniteInput {
        name: &'static str,
        value: f64,
    },
    InvalidOrdering {
        detail: &'static str,
    },
    /// The LDL^T factorization met a pivot whose magnitude is at or below the
    /// internal zero-pivot guard. `ordered_index` is the pivot position in the
    /// profile-ordered numbering; `original_index` is the corresponding row
    /// index of the input matrix (for a reduced stiffness system, the reduced
    /// DOF index before ordering).
    SingularPivot {
        ordered_index: usize,
        original_index: usize,
    },
}

impl fmt::Display for SparseDirectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMatrixDimensions { rows, cols } => {
                write!(f, "matrix must be square, got {rows} by {cols}")
            }
            Self::InvalidVectorLength { expected, actual } => {
                write!(f, "vector length must be {expected}, got {actual}")
            }
            Self::NonFiniteInput { name, value } => {
                write!(f, "{name} must be finite, got {value}")
            }
            Self::InvalidOrdering { detail } => write!(f, "invalid ordering: {detail}"),
            Self::SingularPivot {
                ordered_index,
                original_index,
            } => write!(
                f,
                "singular pivot at ordered index {ordered_index} (original index {original_index})"
            ),
        }
    }
}

impl Error for SparseDirectError {}

/// Pivot location reported in both the profile-ordered numbering and the
/// caller's original (pre-ordering) numbering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PivotLocation {
    pub ordered_index: usize,
    pub original_index: usize,
}

/// Deterministic observations recorded by the LDL^T profile factorization.
///
/// `pivot_condition_ratio_estimate` is `max |d| / min |d|` over the accepted
/// LDL^T pivots. It is a documented conditioning proxy, not a true
/// condition-number estimate; classification thresholds remain governed by
/// D-04 and are never asserted here.
#[derive(Debug, Clone, PartialEq)]
pub struct FactorizationReport {
    pub dimension: usize,
    pub profile_entry_count: usize,
    pub max_half_bandwidth: usize,
    pub min_abs_pivot: Option<f64>,
    pub max_abs_pivot: Option<f64>,
    pub pivot_condition_ratio_estimate: Option<f64>,
    /// Count of accepted pivots with `d < 0` (magnitudes above the zero-pivot
    /// guard). A symmetric stiffness system is expected positive definite;
    /// any nonpositive pivot is surfaced, never silently absorbed.
    pub nonpositive_pivot_count: usize,
    pub first_nonpositive_pivot: Option<PivotLocation>,
}

/// Symmetric matrix stored in skyline (profile) form.
///
/// Storage is the lower triangle by rows: row `i` stores the contiguous span
/// of columns `first_columns[i] ..= i`. The input is treated as symmetric;
/// for an index pair the entry read is always the original lower-triangle
/// entry `dense[max][min]`, which keeps construction deterministic and
/// well-defined even when transform round-off leaves last-ulp asymmetry in an
/// assembled dense matrix.
#[derive(Debug, Clone, PartialEq)]
pub struct SymmetricProfileMatrix {
    dimension: usize,
    original_indices: Vec<usize>,
    first_columns: Vec<usize>,
    row_starts: Vec<usize>,
    values: Vec<f64>,
}

impl SymmetricProfileMatrix {
    /// Builds profile storage from a dense symmetric matrix in its given
    /// (identity) ordering.
    pub fn from_dense(dense: &[Vec<f64>]) -> Result<Self, SparseDirectError> {
        let dimension = validate_square(dense)?;
        let identity: Vec<usize> = (0..dimension).collect();
        Self::from_dense_with_order(dense, &identity)
    }

    /// Builds profile storage from a dense symmetric matrix under the given
    /// ordering permutation, where `order[k]` is the original index of the
    /// `k`-th ordered row/column.
    pub fn from_dense_with_order(
        dense: &[Vec<f64>],
        order: &[usize],
    ) -> Result<Self, SparseDirectError> {
        let dimension = validate_square(dense)?;
        validate_permutation(order, dimension)?;

        let mut first_columns = vec![0usize; dimension];
        for row in 0..dimension {
            let original_row = order[row];
            let mut first = row;
            for col in 0..row {
                if symmetric_entry(dense, original_row, order[col]) != 0.0 {
                    first = col;
                    break;
                }
            }
            first_columns[row] = first;
        }

        let mut row_starts = Vec::with_capacity(dimension + 1);
        row_starts.push(0usize);
        let mut values = Vec::new();
        for row in 0..dimension {
            let original_row = order[row];
            for col in first_columns[row]..=row {
                values.push(symmetric_entry(dense, original_row, order[col]));
            }
            row_starts.push(values.len());
        }

        Ok(Self {
            dimension,
            original_indices: order.to_vec(),
            first_columns,
            row_starts,
            values,
        })
    }

    pub fn dimension(&self) -> usize {
        self.dimension
    }

    /// Number of stored profile (skyline) entries, including diagonals.
    pub fn profile_entry_count(&self) -> usize {
        self.values.len()
    }

    /// Maximum row half-bandwidth `i - first_columns[i]` over all rows.
    pub fn max_half_bandwidth(&self) -> usize {
        (0..self.dimension)
            .map(|row| row - self.first_columns[row])
            .max()
            .unwrap_or(0)
    }

    /// Ordering permutation carried by this matrix (`original_indices[k]` is
    /// the original index of ordered row `k`).
    pub fn original_indices(&self) -> &[usize] {
        &self.original_indices
    }

    /// Entry at ordered position `(row, col)`; zero outside the stored
    /// profile.
    pub fn get(&self, row: usize, col: usize) -> f64 {
        let (hi, lo) = if row >= col { (row, col) } else { (col, row) };
        if hi >= self.dimension || lo < self.first_columns[hi] {
            return 0.0;
        }
        self.values[self.row_starts[hi] + (lo - self.first_columns[hi])]
    }

    fn stored(&self, row: usize, col: usize) -> f64 {
        self.values[self.row_starts[row] + (col - self.first_columns[row])]
    }

    fn set_stored(&mut self, row: usize, col: usize, value: f64) {
        let index = self.row_starts[row] + (col - self.first_columns[row]);
        self.values[index] = value;
    }
}

/// LDL^T factorization of a [`SymmetricProfileMatrix`]. The strictly-lower
/// profile entries hold `L` (unit diagonal implied) and the diagonal holds
/// `D`.
#[derive(Debug, Clone, PartialEq)]
pub struct ProfileFactorization {
    factor: SymmetricProfileMatrix,
    report: FactorizationReport,
}

impl ProfileFactorization {
    pub fn report(&self) -> &FactorizationReport {
        &self.report
    }

    /// Solves `A x = rhs` in the ordered numbering of the factored matrix.
    /// Operation order is fixed: forward substitution with ascending rows and
    /// ascending columns, diagonal scaling ascending, then backward
    /// substitution with descending rows and ascending columns.
    pub fn solve_ordered(&self, rhs: &[f64]) -> Result<Vec<f64>, SparseDirectError> {
        let dimension = self.factor.dimension;
        if rhs.len() != dimension {
            return Err(SparseDirectError::InvalidVectorLength {
                expected: dimension,
                actual: rhs.len(),
            });
        }
        validate_finite_slice("right-hand-side entry", rhs)?;

        let mut solution = rhs.to_vec();

        for row in 0..dimension {
            let mut sum = solution[row];
            for col in self.factor.first_columns[row]..row {
                sum -= self.factor.stored(row, col) * solution[col];
            }
            solution[row] = sum;
        }

        for row in 0..dimension {
            solution[row] /= self.factor.stored(row, row);
        }

        for row in (0..dimension).rev() {
            let value = solution[row];
            for col in self.factor.first_columns[row]..row {
                solution[col] -= self.factor.stored(row, col) * value;
            }
        }

        Ok(solution)
    }
}

/// LDL^T profile factorization with a fixed, deterministic operation order
/// (rows ascending; within a row, columns ascending; inner sums ascending).
/// No pivot reordering is performed; pivots at or below
/// [`SPARSE_SOLVE_ZERO_PIVOT_GUARD`] fail as [`SparseDirectError::SingularPivot`]
/// and accepted negative pivots are counted and located in the report.
pub fn factorize_ldlt(
    matrix: &SymmetricProfileMatrix,
) -> Result<ProfileFactorization, SparseDirectError> {
    let dimension = matrix.dimension;
    let mut factor = matrix.clone();
    let mut work = vec![0.0f64; dimension];
    let mut min_abs_pivot = f64::INFINITY;
    let mut max_abs_pivot: f64 = 0.0;
    let mut nonpositive_pivot_count = 0usize;
    let mut first_nonpositive_pivot: Option<PivotLocation> = None;

    for row in 0..dimension {
        let first = factor.first_columns[row];
        for col in first..row {
            let col_first = factor.first_columns[col];
            let start = if first > col_first { first } else { col_first };
            let mut sum = factor.stored(row, col);
            for inner in start..col {
                sum -= work[inner] * factor.stored(col, inner);
            }
            work[col] = sum;
            let pivot = factor.stored(col, col);
            factor.set_stored(row, col, sum / pivot);
        }

        let mut diagonal = factor.stored(row, row);
        for inner in first..row {
            diagonal -= work[inner] * factor.stored(row, inner);
        }

        if !diagonal.is_finite() {
            return Err(SparseDirectError::NonFiniteInput {
                name: "factorization pivot",
                value: diagonal,
            });
        }
        if diagonal.abs() <= SPARSE_SOLVE_ZERO_PIVOT_GUARD {
            return Err(SparseDirectError::SingularPivot {
                ordered_index: row,
                original_index: factor.original_indices[row],
            });
        }
        if diagonal < 0.0 {
            nonpositive_pivot_count += 1;
            if first_nonpositive_pivot.is_none() {
                first_nonpositive_pivot = Some(PivotLocation {
                    ordered_index: row,
                    original_index: factor.original_indices[row],
                });
            }
        }
        let abs_pivot = diagonal.abs();
        if abs_pivot < min_abs_pivot {
            min_abs_pivot = abs_pivot;
        }
        if abs_pivot > max_abs_pivot {
            max_abs_pivot = abs_pivot;
        }
        factor.set_stored(row, row, diagonal);
    }

    let report = FactorizationReport {
        dimension,
        profile_entry_count: factor.profile_entry_count(),
        max_half_bandwidth: factor.max_half_bandwidth(),
        min_abs_pivot: (dimension > 0).then_some(min_abs_pivot),
        max_abs_pivot: (dimension > 0).then_some(max_abs_pivot),
        pivot_condition_ratio_estimate: (dimension > 0).then_some(max_abs_pivot / min_abs_pivot),
        nonpositive_pivot_count,
        first_nonpositive_pivot,
    };

    Ok(ProfileFactorization { factor, report })
}

/// Builds the undirected node adjacency of a dense symmetric matrix from its
/// exact nonzero pattern (an edge exists when either mirrored entry is
/// nonzero). Exact zero tests keep the pattern deterministic.
pub fn adjacency_from_dense(dense: &[Vec<f64>]) -> Result<Vec<Vec<usize>>, SparseDirectError> {
    let dimension = validate_square(dense)?;
    let mut adjacency = vec![Vec::new(); dimension];
    for row in 0..dimension {
        for col in 0..row {
            if dense[row][col] != 0.0 || dense[col][row] != 0.0 {
                adjacency[row].push(col);
                adjacency[col].push(row);
            }
        }
    }
    Ok(adjacency)
}

/// Deterministic reverse Cuthill-McKee ordering.
///
/// Returns a permutation where `order[k]` is the original index of the `k`-th
/// node in the new ordering. Determinism properties (documented in the crate
/// README): the result is a pure function of the graph structure; adjacency
/// lists are symmetrized, sorted, and deduplicated internally; components are
/// processed in order of their lowest original index; the start node of each
/// component is a pseudo-peripheral node found by deterministic
/// level-structure iteration; all ties break by ascending `(degree, original
/// index)`. No randomization is used anywhere.
pub fn reverse_cuthill_mckee(adjacency: &[Vec<usize>]) -> Result<Vec<usize>, SparseDirectError> {
    let node_count = adjacency.len();

    let mut neighbors: Vec<Vec<usize>> = vec![Vec::new(); node_count];
    for (node, raw_list) in adjacency.iter().enumerate() {
        for &other in raw_list {
            if other >= node_count {
                return Err(SparseDirectError::InvalidOrdering {
                    detail: "adjacency index outside node count",
                });
            }
            if other != node {
                neighbors[node].push(other);
                neighbors[other].push(node);
            }
        }
    }
    for list in &mut neighbors {
        list.sort_unstable();
        list.dedup();
    }

    let degrees: Vec<usize> = neighbors.iter().map(Vec::len).collect();
    for list in &mut neighbors {
        list.sort_by_key(|&node| (degrees[node], node));
    }

    let mut visited = vec![false; node_count];
    let mut order = Vec::with_capacity(node_count);
    for seed in 0..node_count {
        if visited[seed] {
            continue;
        }
        let start = pseudo_peripheral_start(seed, &neighbors, &degrees);

        visited[start] = true;
        let mut queue = VecDeque::new();
        queue.push_back(start);
        while let Some(node) = queue.pop_front() {
            order.push(node);
            for &next in &neighbors[node] {
                if !visited[next] {
                    visited[next] = true;
                    queue.push_back(next);
                }
            }
        }
    }

    order.reverse();
    Ok(order)
}

fn pseudo_peripheral_start(seed: usize, neighbors: &[Vec<usize>], degrees: &[usize]) -> usize {
    let component = bfs_reachable(seed, neighbors);
    let mut candidate = min_by_degree_then_index(&component, degrees).unwrap_or(seed);

    let (mut candidate_eccentricity, mut last_level) = bfs_eccentricity(candidate, neighbors);
    loop {
        let next = match min_by_degree_then_index(&last_level, degrees) {
            Some(node) => node,
            None => break,
        };
        let (next_eccentricity, next_last_level) = bfs_eccentricity(next, neighbors);
        if next_eccentricity > candidate_eccentricity {
            candidate = next;
            candidate_eccentricity = next_eccentricity;
            last_level = next_last_level;
        } else {
            break;
        }
    }
    candidate
}

fn bfs_reachable(seed: usize, neighbors: &[Vec<usize>]) -> Vec<usize> {
    let mut marked = vec![false; neighbors.len()];
    marked[seed] = true;
    let mut reachable = vec![seed];
    let mut queue = VecDeque::new();
    queue.push_back(seed);
    while let Some(node) = queue.pop_front() {
        for &next in &neighbors[node] {
            if !marked[next] {
                marked[next] = true;
                reachable.push(next);
                queue.push_back(next);
            }
        }
    }
    reachable
}

fn bfs_eccentricity(start: usize, neighbors: &[Vec<usize>]) -> (usize, Vec<usize>) {
    let mut marked = vec![false; neighbors.len()];
    marked[start] = true;
    let mut eccentricity = 0usize;
    let mut current_level = vec![start];
    let mut last_level = vec![start];
    loop {
        let mut next_level = Vec::new();
        for &node in &current_level {
            for &next in &neighbors[node] {
                if !marked[next] {
                    marked[next] = true;
                    next_level.push(next);
                }
            }
        }
        if next_level.is_empty() {
            break;
        }
        eccentricity += 1;
        last_level = next_level.clone();
        current_level = next_level;
    }
    (eccentricity, last_level)
}

fn min_by_degree_then_index(nodes: &[usize], degrees: &[usize]) -> Option<usize> {
    nodes
        .iter()
        .copied()
        .min_by_key(|&node| (degrees[node], node))
}

/// Result of the full ordered sparse solve, including ordering and
/// factorization observations for benchmark evidence (FR-008, DEC-023).
#[derive(Debug, Clone, PartialEq)]
pub struct SparseSolveResult {
    /// Solution in the caller's original numbering.
    pub solution: Vec<f64>,
    /// Reverse Cuthill-McKee permutation used (`ordering[k]` is the original
    /// index of ordered position `k`).
    pub ordering: Vec<usize>,
    pub original_max_half_bandwidth: usize,
    pub ordered_max_half_bandwidth: usize,
    pub original_profile_entry_count: usize,
    pub ordered_profile_entry_count: usize,
    pub factorization: FactorizationReport,
}

/// Orders, stores, factors, and solves a dense symmetric system through the
/// sparse skyline path: reverse Cuthill-McKee ordering, profile storage,
/// LDL^T factorization, forward/diagonal/backward substitution, and inverse
/// permutation back to the caller's numbering.
pub fn solve_symmetric_system(
    dense: &[Vec<f64>],
    force: &[f64],
) -> Result<SparseSolveResult, SparseDirectError> {
    let dimension = validate_square(dense)?;
    if force.len() != dimension {
        return Err(SparseDirectError::InvalidVectorLength {
            expected: dimension,
            actual: force.len(),
        });
    }
    validate_finite_slice("force entry", force)?;

    let adjacency = adjacency_from_dense(dense)?;
    let ordering = reverse_cuthill_mckee(&adjacency)?;

    let original_profile = SymmetricProfileMatrix::from_dense(dense)?;
    let ordered_profile = SymmetricProfileMatrix::from_dense_with_order(dense, &ordering)?;
    let original_max_half_bandwidth = original_profile.max_half_bandwidth();
    let original_profile_entry_count = original_profile.profile_entry_count();
    let ordered_max_half_bandwidth = ordered_profile.max_half_bandwidth();
    let ordered_profile_entry_count = ordered_profile.profile_entry_count();

    let factorization = factorize_ldlt(&ordered_profile)?;

    let mut ordered_force = vec![0.0f64; dimension];
    for (ordered_index, &original_index) in ordering.iter().enumerate() {
        ordered_force[ordered_index] = force[original_index];
    }
    let ordered_solution = factorization.solve_ordered(&ordered_force)?;
    let mut solution = vec![0.0f64; dimension];
    for (ordered_index, &original_index) in ordering.iter().enumerate() {
        solution[original_index] = ordered_solution[ordered_index];
    }

    Ok(SparseSolveResult {
        solution,
        ordering,
        original_max_half_bandwidth,
        ordered_max_half_bandwidth,
        original_profile_entry_count,
        ordered_profile_entry_count,
        factorization: factorization.report,
    })
}

fn symmetric_entry(dense: &[Vec<f64>], a: usize, b: usize) -> f64 {
    let (hi, lo) = if a >= b { (a, b) } else { (b, a) };
    dense[hi][lo]
}

fn validate_square(matrix: &[Vec<f64>]) -> Result<usize, SparseDirectError> {
    let rows = matrix.len();
    for row in matrix {
        if row.len() != rows {
            return Err(SparseDirectError::InvalidMatrixDimensions {
                rows,
                cols: row.len(),
            });
        }
        validate_finite_slice("matrix entry", row)?;
    }
    Ok(rows)
}

fn validate_finite_slice(name: &'static str, values: &[f64]) -> Result<(), SparseDirectError> {
    for &value in values {
        if !value.is_finite() {
            return Err(SparseDirectError::NonFiniteInput { name, value });
        }
    }
    Ok(())
}

fn validate_permutation(order: &[usize], dimension: usize) -> Result<(), SparseDirectError> {
    if order.len() != dimension {
        return Err(SparseDirectError::InvalidOrdering {
            detail: "ordering length must match matrix dimension",
        });
    }
    let mut seen = vec![false; dimension];
    for &index in order {
        if index >= dimension {
            return Err(SparseDirectError::InvalidOrdering {
                detail: "ordering index outside matrix dimension",
            });
        }
        if seen[index] {
            return Err(SparseDirectError::InvalidOrdering {
                detail: "ordering repeats an index",
            });
        }
        seen[index] = true;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_pipe_stress_frame_kernel::{
        assemble_global_stiffness, reduce_system, solve_dense, FrameElement, FrameNode,
        FrameSection, DOF_PER_NODE,
    };

    // Parity tolerance basis: DEC-026 (SOFTWARE_DECOMP.md §12) seeds the
    // analytic reference-result class at 1.0e-9 relative. The sparse-vs-dense
    // comparisons below scale that relative seed by the dense solution
    // magnitude rather than inventing a new tolerance.
    const DEC026_ANALYTIC_RELATIVE_SEED: f64 = 1.0e-9;

    fn invented_section() -> FrameSection {
        FrameSection::new(2.0e11, 7.7e10, 0.01, 8.0e-6, 8.0e-6, 1.6e-5).unwrap()
    }

    fn invented_chain(element_count: usize) -> (Vec<Vec<f64>>, Vec<f64>) {
        let section = invented_section();
        let nodes: Vec<FrameNode> = (0..=element_count)
            .map(|index| FrameNode::new(index, [index as f64, 0.0, 0.0]).unwrap())
            .collect();
        let elements: Vec<FrameElement> = (0..element_count)
            .map(|index| {
                FrameElement::new(nodes[index], nodes[index + 1], section, [0.0, 1.0, 0.0]).unwrap()
            })
            .collect();
        let node_count = element_count + 1;
        let stiffness = assemble_global_stiffness(node_count, &elements).unwrap();
        let mut force = vec![0.0; node_count * DOF_PER_NODE];
        force[element_count * DOF_PER_NODE + 1] = -1.0e3;
        let restrained: Vec<usize> = (0..DOF_PER_NODE).collect();
        let reduced = reduce_system(&stiffness, &force, &restrained).unwrap();
        (reduced.stiffness, reduced.force)
    }

    fn invented_grid(x_count: usize, y_count: usize) -> (Vec<Vec<f64>>, Vec<f64>) {
        let section = invented_section();
        let node_count = x_count * y_count;
        let mut nodes = Vec::with_capacity(node_count);
        for y in 0..y_count {
            for x in 0..x_count {
                nodes.push(FrameNode::new(y * x_count + x, [x as f64, y as f64, 0.0]).unwrap());
            }
        }
        let mut elements = Vec::new();
        for y in 0..y_count {
            for x in 0..x_count {
                let index = y * x_count + x;
                if x + 1 < x_count {
                    elements.push(
                        FrameElement::new(nodes[index], nodes[index + 1], section, [0.0, 1.0, 0.0])
                            .unwrap(),
                    );
                }
                if y + 1 < y_count {
                    elements.push(
                        FrameElement::new(
                            nodes[index],
                            nodes[index + x_count],
                            section,
                            [0.0, 0.0, 1.0],
                        )
                        .unwrap(),
                    );
                }
            }
        }
        let stiffness = assemble_global_stiffness(node_count, &elements).unwrap();
        let mut force = vec![0.0; node_count * DOF_PER_NODE];
        force[(node_count - 1) * DOF_PER_NODE] = 1.0e3;
        let restrained: Vec<usize> = (0..x_count * DOF_PER_NODE).collect();
        let reduced = reduce_system(&stiffness, &force, &restrained).unwrap();
        (reduced.stiffness, reduced.force)
    }

    fn assert_dense_parity(stiffness: &[Vec<f64>], force: &[f64]) {
        let dense_solution = solve_dense(stiffness, force).unwrap();
        let sparse = solve_symmetric_system(stiffness, force).unwrap();
        let scale = dense_solution
            .iter()
            .map(|value| value.abs())
            .fold(0.0, f64::max);
        assert!(scale > 0.0, "dense reference solution must be nonzero");
        let max_delta = dense_solution
            .iter()
            .zip(sparse.solution.iter())
            .map(|(dense, sparse)| (dense - sparse).abs())
            .fold(0.0, f64::max);
        assert!(
            max_delta <= DEC026_ANALYTIC_RELATIVE_SEED * scale,
            "sparse-vs-dense delta {max_delta} exceeds DEC-026 relative seed at scale {scale}"
        );
    }

    #[test]
    fn rcm_orders_path_graph_deterministically() {
        let adjacency = vec![vec![1], vec![0, 2], vec![1, 3], vec![2]];
        let order = reverse_cuthill_mckee(&adjacency).unwrap();
        assert_eq!(order, vec![3, 2, 1, 0]);
    }

    #[test]
    fn rcm_orders_star_graph_with_degree_then_index_tie_break() {
        let adjacency = vec![vec![1, 2, 3], vec![0], vec![0], vec![0]];
        let order = reverse_cuthill_mckee(&adjacency).unwrap();
        assert_eq!(order, vec![3, 2, 0, 1]);
    }

    #[test]
    fn rcm_processes_components_by_lowest_index_and_handles_isolated_nodes() {
        let adjacency = vec![vec![1], vec![0], vec![], vec![4], vec![3]];
        let order = reverse_cuthill_mckee(&adjacency).unwrap();
        assert_eq!(order, vec![4, 3, 2, 1, 0]);
    }

    #[test]
    fn rcm_symmetrizes_and_deduplicates_input_adjacency() {
        // One-directional and duplicated edges describe the same path graph.
        let adjacency = vec![vec![1, 1], vec![2], vec![], vec![2, 2]];
        let order = reverse_cuthill_mckee(&adjacency).unwrap();
        assert_eq!(order, vec![3, 2, 1, 0]);
    }

    #[test]
    fn rcm_rejects_out_of_range_adjacency() {
        let error = reverse_cuthill_mckee(&[vec![5]]).unwrap_err();
        assert_eq!(
            error,
            SparseDirectError::InvalidOrdering {
                detail: "adjacency index outside node count"
            }
        );
    }

    #[test]
    fn rcm_reduces_bandwidth_of_scrambled_path_numbering() {
        // Path graph with scrambled labels: 0-2-4-1-3.
        let mut dense = vec![vec![0.0; 5]; 5];
        for index in 0..5 {
            dense[index][index] = 4.0;
        }
        for (a, b) in [(0usize, 2usize), (2, 4), (4, 1), (1, 3)] {
            dense[a][b] = 1.0;
            dense[b][a] = 1.0;
        }
        let force = vec![1.0; 5];

        let result = solve_symmetric_system(&dense, &force).unwrap();

        assert_eq!(result.original_max_half_bandwidth, 3);
        assert_eq!(result.original_profile_entry_count, 12);
        assert_eq!(result.ordered_max_half_bandwidth, 1);
        assert_eq!(result.ordered_profile_entry_count, 9);
    }

    #[test]
    fn profile_storage_round_trips_dense_entries() {
        let dense = vec![
            vec![4.0, 2.0, 0.0],
            vec![2.0, 5.0, 3.0],
            vec![0.0, 3.0, 6.0],
        ];
        let profile = SymmetricProfileMatrix::from_dense(&dense).unwrap();

        assert_eq!(profile.dimension(), 3);
        assert_eq!(profile.profile_entry_count(), 5);
        assert_eq!(profile.max_half_bandwidth(), 1);
        for row in 0..3 {
            for col in 0..3 {
                assert_eq!(profile.get(row, col), dense[row][col]);
            }
        }
    }

    #[test]
    fn ldlt_factorization_and_solve_match_hand_computation_exactly() {
        // All factor and solution values below are exactly representable, so
        // the comparisons are exact (no tolerance needed).
        let dense = vec![
            vec![4.0, 2.0, 0.0],
            vec![2.0, 5.0, 3.0],
            vec![0.0, 3.0, 6.0],
        ];
        let profile = SymmetricProfileMatrix::from_dense(&dense).unwrap();
        let factorization = factorize_ldlt(&profile).unwrap();

        let report = factorization.report();
        assert_eq!(report.dimension, 3);
        assert_eq!(report.nonpositive_pivot_count, 0);
        assert_eq!(report.first_nonpositive_pivot, None);
        assert_eq!(report.min_abs_pivot, Some(3.75));
        assert_eq!(report.max_abs_pivot, Some(4.0));
        assert_eq!(report.pivot_condition_ratio_estimate, Some(4.0 / 3.75));

        // A x = [8, 13.5, 9] has the exact solution [1, 2, 0.5].
        let solution = factorization.solve_ordered(&[8.0, 13.5, 9.0]).unwrap();
        assert_eq!(solution, vec![1.0, 2.0, 0.5]);
    }

    #[test]
    fn singular_pivot_reports_ordered_and_original_location() {
        let dense = vec![vec![1.0, 1.0], vec![1.0, 1.0]];
        let profile = SymmetricProfileMatrix::from_dense(&dense).unwrap();

        let error = factorize_ldlt(&profile).unwrap_err();

        assert_eq!(
            error,
            SparseDirectError::SingularPivot {
                ordered_index: 1,
                original_index: 1
            }
        );
    }

    #[test]
    fn singular_pivot_location_maps_back_through_ordering() {
        // Rows 0 and 2 are identical, so the system is exactly singular. The
        // RCM ordering for this pattern is [1, 2, 0], so the zero pivot is
        // met at ordered index 2 and must map back to original index 0.
        let scrambled = vec![
            vec![1.0, 0.0, 1.0],
            vec![0.0, 4.0, 0.0],
            vec![1.0, 0.0, 1.0],
        ];

        let error = solve_symmetric_system(&scrambled, &[1.0, 1.0, 1.0]).unwrap_err();

        assert_eq!(
            error,
            SparseDirectError::SingularPivot {
                ordered_index: 2,
                original_index: 0
            }
        );
    }

    #[test]
    fn nonpositive_pivot_is_counted_and_located_not_silently_absorbed() {
        let dense = vec![vec![1.0, 0.0], vec![0.0, -2.0]];
        let profile = SymmetricProfileMatrix::from_dense(&dense).unwrap();

        let factorization = factorize_ldlt(&profile).unwrap();
        let report = factorization.report();

        assert_eq!(report.nonpositive_pivot_count, 1);
        assert_eq!(
            report.first_nonpositive_pivot,
            Some(PivotLocation {
                ordered_index: 1,
                original_index: 1
            })
        );

        // The indefinite system still solves exactly.
        let solution = factorization.solve_ordered(&[1.0, -4.0]).unwrap();
        assert_eq!(solution, vec![1.0, 2.0]);
    }

    #[test]
    fn zero_dimension_system_solves_to_empty_solution() {
        let result = solve_symmetric_system(&[], &[]).unwrap();
        assert!(result.solution.is_empty());
        assert_eq!(result.factorization.dimension, 0);
        assert_eq!(result.factorization.min_abs_pivot, None);
        assert_eq!(result.factorization.pivot_condition_ratio_estimate, None);
    }

    #[test]
    fn invalid_inputs_are_rejected() {
        let nonsquare = solve_symmetric_system(&[vec![1.0, 0.0], vec![0.0]], &[1.0, 2.0]);
        assert_eq!(
            nonsquare.unwrap_err(),
            SparseDirectError::InvalidMatrixDimensions { rows: 2, cols: 1 }
        );

        let bad_length = solve_symmetric_system(&[vec![1.0]], &[1.0, 2.0]);
        assert_eq!(
            bad_length.unwrap_err(),
            SparseDirectError::InvalidVectorLength {
                expected: 1,
                actual: 2
            }
        );

        let nonfinite = solve_symmetric_system(&[vec![f64::NAN]], &[1.0]).unwrap_err();
        match nonfinite {
            SparseDirectError::NonFiniteInput { name, value } => {
                assert_eq!(name, "matrix entry");
                assert!(value.is_nan());
            }
            other => panic!("expected non-finite rejection, got {other:?}"),
        }

        let bad_order = SymmetricProfileMatrix::from_dense_with_order(
            &[vec![1.0, 0.0], vec![0.0, 1.0]],
            &[0, 0],
        );
        assert_eq!(
            bad_order.unwrap_err(),
            SparseDirectError::InvalidOrdering {
                detail: "ordering repeats an index"
            }
        );
    }

    #[test]
    fn sparse_path_matches_dense_solve_on_small_invented_chain() {
        let (stiffness, force) = invented_chain(4);
        assert_dense_parity(&stiffness, &force);
    }

    #[test]
    fn sparse_path_matches_dense_solve_on_small_invented_grid_frame() {
        let (stiffness, force) = invented_grid(4, 3);
        assert_dense_parity(&stiffness, &force);
    }

    #[test]
    fn sparse_path_matches_dense_solve_on_larger_generated_banded_chain() {
        // Larger generated banded model (invented/synthetic geometry only):
        // 60 elements, 61 nodes, 366 total DOFs, 360 reduced DOFs.
        let (stiffness, force) = invented_chain(60);
        assert_eq!(stiffness.len(), 360);
        assert_dense_parity(&stiffness, &force);
    }

    #[test]
    fn repeat_sparse_solves_are_bitwise_identical() {
        let (stiffness, force) = invented_grid(4, 3);

        let first = solve_symmetric_system(&stiffness, &force).unwrap();
        let second = solve_symmetric_system(&stiffness, &force).unwrap();

        assert_eq!(first.ordering, second.ordering);
        assert_eq!(first.solution.len(), second.solution.len());
        for (a, b) in first.solution.iter().zip(second.solution.iter()) {
            assert_eq!(a.to_bits(), b.to_bits());
        }
        assert_eq!(first.factorization, second.factorization);
    }

    #[test]
    fn chain_solve_reports_positive_definite_banded_factorization() {
        let (stiffness, force) = invented_chain(8);
        let result = solve_symmetric_system(&stiffness, &force).unwrap();

        assert_eq!(result.factorization.nonpositive_pivot_count, 0);
        assert_eq!(result.factorization.first_nonpositive_pivot, None);
        assert!(result.factorization.pivot_condition_ratio_estimate.unwrap() >= 1.0);
        // Banded chain: the ordered profile must not exceed the identity
        // profile (chain numbering is already contiguous).
        assert!(result.ordered_profile_entry_count <= result.original_profile_entry_count);
        assert!(result.ordered_max_half_bandwidth <= result.original_max_half_bandwidth);
    }
}
