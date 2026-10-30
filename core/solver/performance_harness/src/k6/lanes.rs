//! The two legacy DEC-050/053 observation lanes the product still runs
//! (ROOT's K6 ruling Q12), at kernel level.
//!
//! - **The identity-order lane** (`lane-id`): PP builds a reduced list of
//!   explicit element entries and calls `solve_symmetric_system_from_entries`,
//!   which builds the identity-order profile only to count it (PP `:4464`,
//!   `:4601` on main; `PP@F1b:5421`). `reduced_entry_system` is a documented
//!   copy of PP's private `assemble_reduced_sparse_entry_system` and
//!   `append_reduced_element_entries` (`PP@F1b:5646-5823`, commit
//!   `130445db2`) for frames and restraints only: RF-LARGE, the DEC-053 nine
//!   and the Q5 grids have no user elements, curved bends or springs (ROOT's
//!   K6 ruling N19). PP cannot be a dependency (it pulls registry crates).
//! - **The dense LU lane** (`lane-lu`): PP's dense scrutiny forms the dense
//!   view, reduces it, drops the view and runs `solve_dense`
//!   (`legacy_dense_observation`, `PP@F1b:3285-3289`, `:3838-3849`).
//!
//! `identity_profile` is F1b's `observation_lane_profile` rule
//! (`PP@F1b:2966-3002`): O(nnz), with no profile storage.

use open_pipe_stress_frame_kernel::{
    element_dof_map, FrameElement, FrameKernelError, Matrix12, ELEMENT_DOF,
};
use open_pipe_stress_sparse_direct::SymmetricMatrixEntry;

/// PP's `ReducedSparseEntrySystem`.
#[derive(Debug, Clone)]
pub struct ReducedSparseEntrySystem {
    pub dimension: usize,
    pub entries: Vec<SymmetricMatrixEntry>,
    pub force: Vec<f64>,
}

/// PP's `assemble_reduced_sparse_entry_system` for frames only (no user
/// elements, curved bends or springs), with its checks in its order.
pub fn reduced_entry_system(
    node_count: usize,
    frames: &[FrameElement],
    force: &[f64],
    restrained_dofs: &[usize],
) -> Result<ReducedSparseEntrySystem, FrameKernelError> {
    let total_dofs = node_count * open_pipe_stress_frame_kernel::DOF_PER_NODE;
    if force.len() != total_dofs {
        return Err(FrameKernelError::InvalidVectorLength {
            expected: total_dofs,
            actual: force.len(),
        });
    }
    for &value in force {
        if !value.is_finite() {
            return Err(FrameKernelError::NonFiniteInput {
                name: "force entry",
                value,
            });
        }
    }
    let mut constrained = vec![false; total_dofs];
    for &dof in restrained_dofs {
        if dof >= total_dofs {
            return Err(FrameKernelError::RestrainedDofOutOfRange { dof, total_dofs });
        }
        if constrained[dof] {
            return Err(FrameKernelError::RepeatedRestrainedDof { dof });
        }
        constrained[dof] = true;
    }
    let mut global_to_reduced = vec![None; total_dofs];
    let mut reduced_force = Vec::new();
    for global_dof in 0..total_dofs {
        if constrained[global_dof] {
            continue;
        }
        global_to_reduced[global_dof] = Some(reduced_force.len());
        reduced_force.push(force[global_dof]);
    }
    let mut entries = Vec::new();
    for element in frames {
        validate_element_nodes(element.node_i.index, element.node_j.index, node_count)?;
        let element_stiffness = element.global_stiffness()?;
        append_reduced_element_entries(
            &mut entries,
            &global_to_reduced,
            element.node_i.index,
            element.node_j.index,
            &element_stiffness,
        )?;
    }
    Ok(ReducedSparseEntrySystem {
        dimension: reduced_force.len(),
        entries,
        force: reduced_force,
    })
}

/// PP's `validate_element_nodes`.
fn validate_element_nodes(
    node_i: usize,
    node_j: usize,
    node_count: usize,
) -> Result<(), FrameKernelError> {
    for node_index in [node_i, node_j] {
        if node_index >= node_count {
            return Err(FrameKernelError::InvalidNodeIndex {
                node_index,
                node_count,
            });
        }
    }
    Ok(())
}

/// PP's `append_reduced_element_entries`: each element's lower-triangle
/// entries on free DOFs, skipping exact zeros, as (hi, lo).
fn append_reduced_element_entries(
    entries: &mut Vec<SymmetricMatrixEntry>,
    global_to_reduced: &[Option<usize>],
    node_i: usize,
    node_j: usize,
    element_stiffness: &Matrix12,
) -> Result<(), FrameKernelError> {
    let dof_map = element_dof_map(node_i, node_j);
    for local_row in 0..ELEMENT_DOF {
        let global_row = dof_map[local_row];
        let Some(reduced_row) = global_to_reduced[global_row] else {
            continue;
        };
        for local_col in 0..=local_row {
            let global_col = dof_map[local_col];
            let Some(reduced_col) = global_to_reduced[global_col] else {
                continue;
            };
            let value = element_stiffness[local_row][local_col];
            if !value.is_finite() {
                return Err(FrameKernelError::NonFiniteInput {
                    name: "matrix entry",
                    value,
                });
            }
            if value == 0.0 {
                continue;
            }
            let (row, col) = if reduced_row >= reduced_col {
                (reduced_row, reduced_col)
            } else {
                (reduced_col, reduced_row)
            };
            entries.push(SymmetricMatrixEntry { row, col, value });
        }
    }
    Ok(())
}

/// F1b's `observation_lane_profile` (`PP@F1b:2966-3002`): the identity-order
/// profile of `from_entries` over the same entries (a zero entry skipped),
/// as (profile entries, maximum half-bandwidth).
pub fn identity_profile(system: &ReducedSparseEntrySystem) -> (u128, usize) {
    let mut first_columns: Vec<usize> = (0..system.dimension).collect();
    for entry in &system.entries {
        if entry.value == 0.0 || entry.row >= system.dimension || entry.col >= system.dimension {
            continue;
        }
        let (hi, lo) = if entry.row >= entry.col {
            (entry.row, entry.col)
        } else {
            (entry.col, entry.row)
        };
        if lo < first_columns[hi] {
            first_columns[hi] = lo;
        }
    }
    let entries = first_columns
        .iter()
        .enumerate()
        .map(|(row, &first)| (row - first + 1) as u128)
        .sum();
    let bandwidth = first_columns
        .iter()
        .enumerate()
        .map(|(row, &first)| row - first)
        .max()
        .unwrap_or(0);
    (entries, bandwidth)
}

/// The lane's off-diagonal entries (the adjacency pushes two per entry).
pub fn off_diagonal_entries(system: &ReducedSparseEntrySystem) -> usize {
    system
        .entries
        .iter()
        .filter(|e| e.value != 0.0 && e.row != e.col)
        .count()
}
