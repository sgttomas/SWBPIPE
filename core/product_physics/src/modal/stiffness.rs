//! The modal pencil (K_ff, M_ff) of a prepared linear model (DEL-04-09 Design
//! §1.1 to §1.3, §3.2 item 1).
//!
//! - **K** is the static path's own basis stiffness,
//!   `form_basis_stiffness(&built, &boundary.springs, true)`: frames,
//!   connectors and their tie rods, realized bends and the springs, in the
//!   static assembly's order. A basis whose formation leaves the binary64
//!   range (`RangeDeferred`) has no deferred case on the modal path: it is
//!   refused by name (`MODAL_PENCIL_RANGE_UNSUPPORTED`). Any other formation
//!   error is refused as the static entry refuses it.
//! - **The partition** is the static reduction's: the free DOFs and their
//!   positions come from `reduce_assembled_sparse_system` on the formed
//!   stiffness with the prepared restrained DOFs and a zero force, and K_ff
//!   and M_ff are the free rows and columns read through that partition; no
//!   reduction arithmetic is copied. Restrained DOFs, imposed-motion (W-16)
//!   DOFs included, stay restrained, and no motion value enters. Constant-
//!   effort supports were already excluded from the boundary (`build`), so
//!   they contribute neither stiffness nor mass; connectors and tie rods
//!   carry no mass (`mass`).
//! - **Singularity.** Before any eigen-solve, K_ff passes the static dense
//!   path's own structural solve on a zero force: the assembly and symmetry
//!   evidence the static entry builds (`SparseAssemblyEvidence`, as
//!   `solve_preview_reduced_system` builds it) and its `solve_assembled` in
//!   `DenseScrutiny` mode (the M03-INTEGRITY preparation, the dense
//!   Cholesky's pivot screen and its mechanism witness). A mechanism or
//!   singular model is refused there with the static path's integrity code
//!   (`integrity_failure_code`); rigid-body modes are never analysed.
//! - **Resource guard.** The static `dense_scrutiny_guard` (96 B per global
//!   n² entry against the provisional ceiling) is applied before any n²
//!   allocation; the modal path's counted phases (the kernel's eigen core of
//!   `modal_eigen::dense_path_peak_bytes`, the Sturm phase and the formation
//!   accumulators) lie within that formula's count. **This is not modal
//!   product qualification.** The recorded eigen-core bytes and the
//!   operation estimate are stated formulas, not measurements and not a safe
//!   product budget; the work budget of design §3.2 (O-4) is unpinned and
//!   unmeasured, and no work guard is applied here.
//! - **Admission range.** Every nonzero entry of K_ff and M_ff lies in
//!   [2⁻³⁰⁰, 2³⁰⁰] (`modal_eigen::admission`), or the pencil is refused by name
//!   (`MODAL_PENCIL_RANGE_UNSUPPORTED`, naming the DOFs).
use super::integrity::{modal_range_refusal, MODAL_NO_FINITE_MODES, MODAL_PENCIL_RANGE_UNSUPPORTED};
use super::mass::ModalMass;
use super::prepare::Prepared;
use crate::{
    dense_scrutiny_ceiling_bytes, dense_scrutiny_estimate_bytes, dense_scrutiny_guard, dense_scrutiny_refusal_diagnostic,
    diag, form_basis_stiffness, frame_member_identities, integrity_dof_label, integrity_failure_code, BasisStiffness, Diagnostic, DOF_PER_NODE,
};
use open_pipe_stress_frame_kernel::load_ledger::LoadLedger;
use open_pipe_stress_frame_kernel::structural::{reduce_assembled_sparse_system, StructuralError};
use open_pipe_stress_nonlinear_integration::structural_adapter::SparseAssemblyEvidence;
use open_pipe_stress_nonlinear_integration::{CurvedBendStiffnessElement, LinearSolveMode};
use open_pipe_stress_modal_eigen::admission::{check_entry, PencilMatrix};
use open_pipe_stress_modal_eigen::dd::Dd;
use open_pipe_stress_modal_eigen::dense::{dense_path_flop_estimate, dense_path_peak_bytes};
use open_pipe_stress_modal_eigen::modal_quantities::{Dof, FreeDof};
use open_pipe_stress_modal_eigen::SymmetricMatrix;
use open_pipe_stress_sparse_direct::SymmetricMatrixEntry;

/// What the resource guard judged and what it recorded (module docs).
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct DenseGuardRecord {
    /// Global DOFs, the static guard's dimension.
    pub(crate) global_dimension: usize,
    /// Free DOFs, the order of the modal pencil.
    pub(crate) free_dimension: usize,
    /// The static formula's estimate, 96 B × global n², and its ceiling.
    pub(crate) static_estimate_bytes: u128,
    pub(crate) ceiling_bytes: u128,
    /// `modal_eigen::dense_path_peak_bytes(n_free)`: the kernel's stated
    /// eigen-core count. Recorded; not a measurement and not a budget.
    pub(crate) eigen_core_bytes: u128,
    /// `dense_path_flop_estimate(n_free, s_max)`: 9n³ + s_max·n³/3.
    /// Recorded; not a measurement and not a budget (O-4 unpinned).
    pub(crate) operation_estimate: u128,
}

/// The modal pencil on the static partition (module docs).
#[derive(Debug, Clone)]
pub(crate) struct ModalPencil {
    pub(crate) global_dimension: usize,
    /// Free global DOFs, ascending (the static reduction's order).
    pub(crate) free_dofs: Vec<usize>,
    /// The free position of each global DOF, `None` when restrained.
    pub(crate) dof_map: Vec<Option<usize>>,
    /// Each free DOF as (node, DOF), for the normalization.
    pub(crate) free_labels: Vec<FreeDof>,
    /// Each free DOF's model label (`node:UX`).
    pub(crate) labels: Vec<String>,
    pub(crate) stiffness: SymmetricMatrix,
    pub(crate) mass: SymmetricMatrix,
    /// The stored lower entries of K_ff and of M_ff (nonzero), free numbering.
    pub(crate) stiffness_entries: Vec<SymmetricMatrixEntry>,
    pub(crate) mass_entries: Vec<SymmetricMatrixEntry>,
    /// Pattern positions where the formed K's two triangles differ in their
    /// bits; K_ff takes the lower one (the formation difference measures it).
    pub(crate) stiffness_asymmetric_entries: usize,
    /// p_k = (M R_k)_f over the free DOFs, k = X, Y, Z (D-v).
    pub(crate) excitation: [Vec<f64>; 3],
    /// T_k = R_kᵀ M R_k, kg (I2a's mass summary).
    pub(crate) total_mass: [f64; 3],
    /// R_k,fᵀ M_ff R_k,f, kg: a summary row, not the fractions' denominator.
    pub(crate) free_dof_mass: [f64; 3],
    pub(crate) guard: DenseGuardRecord,
}

const DOFS: [Dof; 6] = [Dof::Ux, Dof::Uy, Dof::Uz, Dof::Rx, Dof::Ry, Dof::Rz];

fn blocked(id: &str, code: &str, message: String) -> Vec<Diagnostic> {
    vec![diag(id, code, "blocking", message, vec!["model".to_string()])]
}

fn singular_refusal(prepared: &Prepared, error: &StructuralError) -> Vec<Diagnostic> {
    let mut refs = vec!["model".to_string()];
    if let StructuralError::NumericallyUnresolved { global_dof: Some(dof), .. } = error {
        refs.push(integrity_dof_label(&prepared.model, *dof));
    }
    vec![diag(
        "diagnostic:modal:stiffness:singular",
        integrity_failure_code(error),
        "blocking",
        format!("the reduced stiffness K_ff is refused by the static dense path's structural solve before any eigen-solve ({error}); a mechanism or singular model has no modal solution here, and rigid-body modes are not analysed"),
        refs,
    )]
}

/// The modal pencil of a prepared model and its formed mass (module docs).
/// `max_candidates` is s_max, recorded in the operation estimate only.
pub(crate) fn form_modal_pencil(prepared: &Prepared, mass: &ModalMass, max_candidates: usize) -> Result<ModalPencil, Vec<Diagnostic>> {
    let built = &prepared.built;
    let global_dimension = built.nodes.len() * DOF_PER_NODE;
    let ceiling_bytes = dense_scrutiny_ceiling_bytes();
    if let Err(refusal) = dense_scrutiny_guard(global_dimension, ceiling_bytes) {
        return Err(vec![dense_scrutiny_refusal_diagnostic(&refusal)]);
    }
    let formed = match form_basis_stiffness(built, &prepared.boundary.springs, true) {
        Ok(BasisStiffness::Formed(formed)) => formed,
        Ok(BasisStiffness::RangeDeferred(error)) => {
            return Err(blocked(
                "diagnostic:modal:stiffness:range",
                MODAL_PENCIL_RANGE_UNSUPPORTED,
                format!("the basis stiffness formation leaves the binary64 normal range ({error}); the modal path has no deferred case, so nothing is solved"),
            ))
        }
        Err(error) => return Err(blocked("diagnostic:physics:solver", "SOLVER_SYSTEM_BLOCKED", error.to_string())),
    };
    // The static partition, with a zero force: only its free DOFs and their
    // positions are read.
    let zero = LoadLedger::new()
        .finish(global_dimension)
        .map_err(|error| blocked("diagnostic:physics:solver", "SOLVER_SYSTEM_BLOCKED", format!("{error:?}")))?;
    let restrained = &prepared.boundary.restrained_dofs;
    let reduced = reduce_assembled_sparse_system(&formed, &zero, restrained, None)
        .map_err(|error| blocked("diagnostic:physics:solver", "SOLVER_SYSTEM_BLOCKED", error.to_string()))?;
    let free_dofs = reduced.free_dofs.clone();
    let n = free_dofs.len();
    if n == 0 {
        return Err(blocked(
            "diagnostic:modal:no-finite-modes",
            MODAL_NO_FINITE_MODES,
            "every DOF of the model is restrained, so the modal pencil is empty and has no finite mode".into(),
        ));
    }
    let dof_map: Vec<Option<usize>> = (0..global_dimension).map(|dof| reduced.free_position(dof)).collect();

    // The static dense path's own structural solve, with its assembly and
    // symmetry evidence, on a zero force (module docs).
    {
        let curved = built
            .curved_bend_elements
            .iter()
            .map(|e| CurvedBendStiffnessElement::from_macro_element(e.component_id.clone(), &e.macro_element))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| blocked("diagnostic:physics:solver", "SOLVER_SYSTEM_BLOCKED", format!("curved formation evidence: {error:?}")))?;
        let springs: Vec<(usize, f64)> =
            prepared.boundary.springs.iter().map(|e| (e.node_dof.global_index(), e.stiffness.value)).collect();
        let prescribed: Vec<(usize, f64)> = restrained.iter().map(|&dof| (dof, 0.0)).collect();
        SparseAssemblyEvidence::new(formed.pattern(), built.nodes.len(), &built.frame_elements, &built.connectors, &curved, &springs)
            .map(|assembly| assembly.with_frame_members(&frame_member_identities(built)))
            .and_then(|assembly| assembly.solve_assembled(&formed, &zero, &free_dofs, &prescribed, LinearSolveMode::DenseScrutiny))
            .map_err(|error| singular_refusal(prepared, &error))?;
    }

    let labels: Vec<String> = free_dofs.iter().map(|&dof| integrity_dof_label(&prepared.model, dof)).collect();
    let free_labels: Vec<FreeDof> =
        free_dofs.iter().map(|&dof| FreeDof { node: dof / DOF_PER_NODE, dof: DOFS[dof % DOF_PER_NODE] }).collect();
    let range = |matrix: PencilMatrix, r: usize, c: usize, value: f64| {
        check_entry(matrix, r, c, value).map_err(|range| {
            vec![modal_range_refusal(
                "pencil-range",
                super::integrity::pencil_range_text(&range, &labels),
                vec![labels[r].clone(), labels[c].clone()],
            )]
        })
    };

    // K_ff through the partition, lower triangle from each row.
    let mut k = vec![0.0; n * n];
    let mut stiffness_entries = Vec::new();
    let mut stiffness_asymmetric_entries = 0;
    for (r, &dof) in free_dofs.iter().enumerate() {
        for (col, value) in formed.row(dof) {
            let Some(c) = dof_map[col] else { continue };
            if c > r {
                continue;
            }
            range(PencilMatrix::Stiffness, r, c, value)?;
            if c != r && formed.get(col, dof).to_bits() != value.to_bits() {
                stiffness_asymmetric_entries += 1;
            }
            k[r * n + c] = value;
            k[c * n + r] = value;
            stiffness_entries.push(SymmetricMatrixEntry { row: r, col: c, value });
        }
    }

    // M_ff through the same partition, from I2a's upper node blocks.
    let mut m = vec![0.0; n * n];
    let mut mass_entries = Vec::new();
    for ((i, j), block) in mass.node_blocks() {
        for (a, line) in block.iter().enumerate() {
            for (b, &value) in line.iter().enumerate() {
                let (row, col) = (DOF_PER_NODE * i + a, DOF_PER_NODE * j + b);
                if row > col || value == 0.0 {
                    continue;
                }
                let (Some(p), Some(q)) = (dof_map[row], dof_map[col]) else { continue };
                let (r, c) = if p >= q { (p, q) } else { (q, p) };
                range(PencilMatrix::Mass, r, c, value)?;
                m[r * n + c] = value;
                m[c * n + r] = value;
                mass_entries.push(SymmetricMatrixEntry { row: r, col: c, value });
            }
        }
    }
    if mass_entries.is_empty() {
        return Err(blocked(
            "diagnostic:modal:no-finite-modes",
            MODAL_NO_FINITE_MODES,
            "the free-DOF mass M_ff is zero (all mass is on restrained DOFs, or none is formed), so the pencil has no finite eigenvalue".into(),
        ));
    }
    let to_matrix = |data: Vec<f64>| {
        SymmetricMatrix::from_row_major(n, data)
            .map_err(|error| blocked("diagnostic:modal:pencil", MODAL_PENCIL_RANGE_UNSUPPORTED, format!("the modal pencil is not a finite symmetric matrix ({error:?})")))
    };
    let stiffness = to_matrix(k)?;
    let modal_mass = to_matrix(m)?;

    let mut excitation: [Vec<f64>; 3] = Default::default();
    let mut free_dof_mass = [0.0; 3];
    for (axis, p) in excitation.iter_mut().enumerate() {
        let product = mass.rigid_product(axis);
        *p = free_dofs.iter().map(|&dof| product[dof].to_f64()).collect();
        let mut sum = Dd::ZERO;
        for entry in &mass_entries {
            let (row, col) = (free_dofs[entry.row], free_dofs[entry.col]);
            if row % DOF_PER_NODE == axis && col % DOF_PER_NODE == axis {
                let weight = if entry.row == entry.col { 1.0 } else { 2.0 };
                sum = sum.add(Dd::product(entry.value, weight));
            }
        }
        free_dof_mass[axis] = sum.to_f64();
    }

    let guard = DenseGuardRecord {
        global_dimension,
        free_dimension: n,
        static_estimate_bytes: dense_scrutiny_estimate_bytes((global_dimension as u128) * (global_dimension as u128)),
        ceiling_bytes,
        eigen_core_bytes: dense_path_peak_bytes(n as u64),
        operation_estimate: dense_path_flop_estimate(n as u64, max_candidates as u64),
    };
    Ok(ModalPencil {
        global_dimension,
        free_dofs,
        dof_map,
        free_labels,
        labels,
        stiffness,
        mass: modal_mass,
        stiffness_entries,
        mass_entries,
        stiffness_asymmetric_entries,
        excitation,
        total_mass: mass.summary.total_by_direction,
        free_dof_mass,
        guard,
    })
}
