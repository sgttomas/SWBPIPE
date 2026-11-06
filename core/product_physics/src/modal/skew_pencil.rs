//! R-6/R-7 (DEL-04-03 Work `skew-restraints.md` §3.4, §11.1 D-1 and §11.3 C1-C9): the
//! modal pencil of a model with a skewed restraint, on the static path's reduced
//! coordinates.
//!
//! - **The basis** is the static path's (`BuiltModel::skew`, one `build` per model, (e)).
//!   The restrained set must be its axis set; nonlinear supports never reach here
//!   (§2.2 item 3).
//! - **K′** is `transform_stiffness` of the static basis stiffness, the partition is
//!   `reduce_assembled_sparse_system` on `restrained_coordinates` (C1), and the
//!   singularity evidence is the static path's own coordinate solve
//!   (`skew::solve_coordinates`, NI's linear skew entry) on a zero force in
//!   `DenseScrutiny`.
//! - **M′** (C2): each lower entry is Σ over `congruence_terms(r, c)` of
//!   q_ir·M_ij·q_jc over the full symmetric double-double M (M_ij above the
//!   diagonal, M_jiᵀ below it, `mass_entry_dd`), accumulated exactly and rounded
//!   once; the upper triangle mirrors it, so M′ is symmetric bit for bit. Every
//!   entry has at most 9 terms (4 in S-K1), so the work is countable (f).
//! - **The excitation** (b, C6) is Q̃ᵀ(M r_k) from the global M and r_k, each
//!   coordinate's sum rounded once from double-double. **The kept-subspace mass**
//!   (C4) is a_kᵀM′a_k, a_k = Q̃_fᵀr_k, accumulated in double-double and rounded once.
//! - **Labels** (C5): a coordinate at a stop's node is labelled by its node, its
//!   coordinate slot and the stop's id; every other coordinate keeps today's label.
//!   No label names a global DOF for a mixed coordinate.
//! - **Re-formation** (a): each global DOF that the basis mixes carries its weighted
//!   free coordinates (`Coordinates`); the re-formed pencil is Q̃ᵀK̂Q̃, Q̃ᵀM̂Q̃.
use super::mass::ModalMass;
use crate::{integrity_dof_label, Diagnostic, PreviewModel, DOF_PER_NODE};
use open_pipe_stress_frame_kernel::restraint_basis::RestraintBasis;
use open_pipe_stress_modal_eigen::dd::Dd;
use open_pipe_stress_modal_eigen::exact::ExactSum;
use open_pipe_stress_modal_eigen::reformation::Coordinates;

/// The basis a reduced pencil is formed on, for the lift and the re-formation.
#[derive(Debug, Clone)]
pub(crate) struct SkewPencil {
    pub(crate) basis: RestraintBasis,
    /// Per global DOF: `Some` (its weighted free coordinates) exactly where the
    /// basis mixes it.
    pub(crate) coordinates: Vec<Option<Coordinates>>,
}

impl SkewPencil {
    /// φ = Q̃φ′ over the global DOFs, in binary64 (C6: published shapes are lifted
    /// in binary64), from the free-coordinate vector `free` on `free_dofs`.
    pub(crate) fn lift(&self, free_dofs: &[usize], free: &[f64]) -> Option<Vec<f64>> {
        let mut v = vec![0.0; self.basis.dimension()];
        for (&dof, &value) in free_dofs.iter().zip(free) {
            v[dof] = value;
        }
        self.basis.lift(&v).ok()
    }
}

/// The weighted free coordinates of every global DOF the basis mixes.
pub(crate) fn coordinates(basis: &RestraintBasis, dof_map: &[Option<usize>]) -> Option<Vec<Option<Coordinates>>> {
    let mut out = vec![None; dof_map.len()];
    for (dof, slot) in out.iter_mut().enumerate() {
        if !basis.is_constrained_translation(dof) {
            continue;
        }
        let mut entries = [(0, 0.0); 3];
        let mut len = 0;
        for &(coordinate, q) in basis.q_row(dof).as_slice() {
            if let Some(free) = dof_map[coordinate] {
                entries[len] = (free, q);
                len += 1;
            }
        }
        *slot = Some(Coordinates::new(&entries[..len])?);
    }
    Some(out)
}

/// C5: a coordinate's label.
pub(crate) fn label(model: &PreviewModel, basis: &RestraintBasis, coordinate: usize) -> String {
    let node = coordinate / DOF_PER_NODE;
    match basis.node_basis(node).filter(|_| basis.is_constrained_translation(coordinate)) {
        Some(nb) => {
            let id = model.nodes.get(node).map_or_else(|| format!("node {node}"), |n| n.id.clone());
            let stop = basis.stops().get(nb.stop).map_or("?", |s| s.support_id.as_str());
            format!("{id}:skew coordinate {} of {stop}", coordinate % DOF_PER_NODE)
        }
        None => integrity_dof_label(model, coordinate),
    }
}

/// C2: M′'s lower entries (free numbering, (row, col, value) with row ≥ col), each
/// the exact congruence sum rounded once.
pub(crate) fn mass_entries(basis: &RestraintBasis, mass: &ModalMass, dof_map: &[Option<usize>]) -> Vec<(usize, usize, f64)> {
    let mut out = Vec::new();
    let mut sum = ExactSum::new();
    for ((i, j), _) in mass.node_blocks() {
        for a in 0..DOF_PER_NODE {
            for b in 0..DOF_PER_NODE {
                let (r, c) = (DOF_PER_NODE * i + a, DOF_PER_NODE * j + b);
                // Within a diagonal node block, each lower pair once.
                if i == j && r < c {
                    continue;
                }
                let (Some(p), Some(q)) = (dof_map[r], dof_map[c]) else { continue };
                sum.reset();
                for term in basis.congruence_terms(r, c).as_slice() {
                    let m = mass.mass_entry_dd(term.i, term.j);
                    for word in [m.hi, m.lo] {
                        sum.add_product(&[term.left, word, term.right]);
                    }
                }
                let value = sum.to_f64();
                if value != 0.0 {
                    out.push((p.max(q), p.min(q), value));
                }
            }
        }
    }
    out
}

/// (b): p′_k = Q̃_fᵀ(M r_k), each coordinate rounded once from its exact sum of the
/// double-double M r_k.
pub(crate) fn excitation(basis: &RestraintBasis, mass: &ModalMass, free_dofs: &[usize]) -> [Vec<f64>; 3] {
    let mut out: [Vec<f64>; 3] = Default::default();
    let mut sum = ExactSum::new();
    for (k, p) in out.iter_mut().enumerate() {
        let product: Vec<Dd> = mass.rigid_product(k);
        *p = free_dofs
            .iter()
            .map(|&coordinate| {
                sum.reset();
                for &(dof, q) in basis.q_column(coordinate).as_slice() {
                    sum.add_dd_product(product[dof], &[q]);
                }
                sum.to_f64()
            })
            .collect();
    }
    out
}

/// C4: the kept-subspace mass a_kᵀM′a_k, a_k = Q̃_fᵀr_k, accumulated in double-double
/// and rounded once, from M′'s stored lower entries.
pub(crate) fn kept_subspace_mass(basis: &RestraintBasis, free_dofs: &[usize], lower: &[(usize, usize, f64)]) -> [f64; 3] {
    let mut out = [0.0; 3];
    for (k, value) in out.iter_mut().enumerate() {
        let a: Vec<f64> = free_dofs
            .iter()
            .map(|&coordinate| {
                // N3: Q is node-local, so a column has at most one entry per axis k; the
                // binary64 sum is that one entry (or 0), exact.
                let column = basis.q_column(coordinate);
                let mut on_axis = column.as_slice().iter().filter(|&&(dof, _)| dof % DOF_PER_NODE == k).map(|&(_, q)| q);
                let q = on_axis.next().unwrap_or(0.0);
                debug_assert!(on_axis.next().is_none(), "a node-local column has one entry per axis");
                q
            })
            .collect();
        let mut sum = Dd::ZERO;
        for &(p, q, m) in lower {
            if a[p] == 0.0 || a[q] == 0.0 {
                continue;
            }
            let weight = if p == q { 1.0 } else { 2.0 };
            sum = sum.add(Dd::product(m, a[p]).mul_f64(a[q] * weight));
        }
        *value = sum.to_f64();
    }
    out
}

/// Pattern positions where the formed global K's two triangles differ in their
/// bits, over physical DOFs that no axis restraint holds (C2: measured on the global
/// K, before the transform).
pub(crate) fn global_asymmetric_entries(formed: &open_pipe_stress_frame_kernel::structural::SparseStiffness, basis: &RestraintBasis) -> usize {
    let held = basis.axis_restrained();
    let mut count = 0;
    for row in 0..basis.dimension() {
        if held.binary_search(&row).is_ok() {
            continue;
        }
        for (col, value) in formed.row(row) {
            if col < row && held.binary_search(&col).is_err() && formed.get(col, row).to_bits() != value.to_bits() {
                count += 1;
            }
        }
    }
    count
}

/// C1's static evidence failure, mapped as the static path maps it.
pub(crate) fn evidence_diagnostics(error: open_pipe_stress_frame_kernel::structural::StructuralError) -> Vec<Diagnostic> {
    vec![crate::diag(
        "diagnostic:modal:stiffness:singular",
        crate::integrity_failure_code(&error),
        "blocking",
        format!("the reduced coordinate stiffness K′_ff is refused by the static skew path's structural solve before any eigen-solve ({error}); a mechanism or singular model has no modal solution here, and rigid-body modes are not analysed"),
        vec!["model".to_string()],
    )]
}
