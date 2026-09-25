//! Typed structural skyline path. Legacy factorize_ldlt/report remains unchanged.
use super::{adjacency_from_dense, reverse_cuthill_mckee, SymmetricProfileMatrix};
use open_pipe_stress_frame_kernel::structural::{
    self, PositiveFactor, PreparedSystem, StructuralError, StructuralSolution, StructuralSystem,
};
pub fn factor_structural_ldlt<'p, 's>(
    prepared: &'p PreparedSystem<'s>,
) -> Result<PositiveFactor<'p, 's>, StructuralError> {
    let matrix = prepared.matrix();
    let adjacency = adjacency_from_dense(matrix)
        .map_err(|_| StructuralError::InvalidInput("sparse adjacency"))?;
    let order = reverse_cuthill_mckee(&adjacency)
        .map_err(|_| StructuralError::InvalidInput("sparse order"))?;
    let profile = SymmetricProfileMatrix::from_dense_with_order(matrix, &order)
        .map_err(|_| StructuralError::InvalidInput("sparse profile"))?;
    structural::factor_structural_profile(prepared, &order, &profile.first_columns)
}
pub fn solve_structural_sparse(
    system: &StructuralSystem<'_>,
) -> Result<StructuralSolution, StructuralError> {
    let prepared = structural::prepare_structural(system)?;
    let factor = match factor_structural_ldlt(&prepared) {
        Ok(f) => f,
        Err(error) => return Err(structural::negative_pair_witness(&prepared)?.unwrap_or(error)),
    };
    structural::finish_structural(&factor)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn run(k: &[Vec<f64>], f: &[f64]) -> Result<StructuralSolution, StructuralError> {
        let free: Vec<usize> = (0..f.len()).collect();
        let s = StructuralSystem {
            stiffness: k,
            force: f,
            free_dofs: &free,
            prescribed: &[],
            contributions: None,
            symmetry: None,
        };
        solve_structural_sparse(&s)
    }
    #[test]
    fn soft_positive_sparse_uses_typed_gate_not_legacy_absolute_guard() {
        let result = run(&[vec![2e-30, 0.0], vec![0.0, 3e-30]], &[4e-30, 9e-30]).unwrap();
        assert!((result.displacements[0] - 2.0).abs() < 1e-14);
        assert!((result.displacements[1] - 3.0).abs() < 1e-14);
        assert!(result.report.pivots.iter().all(|p| p.pivot > p.screen));
    }
    #[test]
    fn rejection_does_not_fall_back_to_lu_or_ignore_triangle() {
        assert!(matches!(
            run(&[vec![1.0, 2.0], vec![2.0, 1.0]], &[1.0, 1.0]),
            Err(StructuralError::NegativeEnergy { .. })
        ));
        assert!(matches!(
            run(&[vec![2.0, 1.0], vec![0.0, 2.0]], &[1.0, 1.0]),
            Err(StructuralError::Asymmetric { .. })
        ));
        assert!(run(&[vec![1.0, -1.0], vec![-1.0, 1.0]], &[0.0, 0.0]).is_err());
    }
    #[test]
    fn mapped_prescribed_equations_and_loaded_constraint() {
        let k = vec![vec![2.0, -1.0], vec![-1.0, 2.0]];
        let f = [99.0, 3.0];
        let free = [1];
        let bc = [(0, 1.0)];
        let s = StructuralSystem {
            stiffness: &k,
            force: &f,
            free_dofs: &free,
            prescribed: &bc,
            contributions: None,
            symmetry: None,
        };
        let result = solve_structural_sparse(&s).unwrap();
        assert_eq!(result.displacements, vec![1.0, 2.0]);
        assert_eq!(result.report.pivots[0].global_dof, 1);
        assert_eq!(result.report.residual_rows[0].global_dof, 1);
    }
    #[test]
    fn krev02_sparse_prescribed_contribution_fidelity() {
        let matrix = vec![vec![2.0, -1.0], vec![-1.0, 2.0]];
        let force = [0.0, 0.0];
        let free = [1];
        let bc = [(0, 1.0)];
        for coupling in [-1.0, -2.0] {
            let entries = [
                structural::StiffnessContribution {
                    row: 0,
                    col: 0,
                    value: 2.0,
                },
                structural::StiffnessContribution {
                    row: 1,
                    col: 1,
                    value: 2.0,
                },
                structural::StiffnessContribution {
                    row: 0,
                    col: 1,
                    value: coupling,
                },
                structural::StiffnessContribution {
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
            if coupling == -1.0 {
                assert_eq!(
                    solve_structural_sparse(&s).unwrap().displacements,
                    vec![1.0, 0.5]
                );
            } else {
                assert!(solve_structural_sparse(&s).is_err());
            }
        }
    }
}
