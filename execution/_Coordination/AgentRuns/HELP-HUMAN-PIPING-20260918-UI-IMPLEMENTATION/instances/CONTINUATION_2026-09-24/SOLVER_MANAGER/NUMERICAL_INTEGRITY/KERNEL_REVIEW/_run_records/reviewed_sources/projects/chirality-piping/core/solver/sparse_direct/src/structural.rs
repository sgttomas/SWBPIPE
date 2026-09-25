//! Typed structural skyline path. Legacy factorize_ldlt/report remains unchanged.
use super::{adjacency_from_dense, reverse_cuthill_mckee, SymmetricProfileMatrix};
use open_pipe_stress_frame_kernel::structural::{
    self, PivotEvidence, StructuralError, StructuralSolution, StructuralSystem,
};

pub struct StructuralProfileFactor {
    factor: SymmetricProfileMatrix,
    order: Vec<usize>,
    pub pivots: Vec<PivotEvidence>,
}
impl StructuralProfileFactor {
    pub fn solve(&self, rhs: &[f64]) -> Result<Vec<f64>, StructuralError> {
        let n = self.order.len();
        if rhs.len() != n {
            return Err(StructuralError::InvalidInput("sparse RHS length"));
        }
        let mut x: Vec<f64> = self.order.iter().map(|&i| rhs[i]).collect();
        for i in 0..n {
            for j in self.factor.first_columns[i]..i {
                x[i] = structural::checked_value(
                    x[i] - structural::checked_product(self.factor.stored(i, j), x[j])?,
                )?;
            }
        }
        for i in 0..n {
            x[i] = structural::checked_quotient(x[i], self.factor.stored(i, i))?;
        }
        for i in (0..n).rev() {
            let value = x[i];
            for j in self.factor.first_columns[i]..i {
                x[j] = structural::checked_value(
                    x[j] - structural::checked_product(self.factor.stored(i, j), value)?,
                )?;
            }
        }
        let mut out = vec![0.0; n];
        for (i, &original) in self.order.iter().enumerate() {
            out[original] = x[i];
        }
        Ok(out)
    }
}
pub fn factor_structural_ldlt(
    matrix: &[Vec<f64>],
    free_dofs: &[usize],
) -> Result<StructuralProfileFactor, StructuralError> {
    let dimension = matrix.len();
    if free_dofs.len() != dimension
        || matrix
            .iter()
            .any(|r| r.len() != dimension || r.iter().any(|v| !v.is_finite()))
    {
        return Err(StructuralError::InvalidInput(
            "structural sparse dimensions/finiteness",
        ));
    }
    for i in 0..dimension {
        for j in 0..i {
            if matrix[i][j] != matrix[j][i] {
                return Err(StructuralError::InvalidInput(
                    "structural sparse requires audited symmetric matrix",
                ));
            }
        }
    }
    let adjacency = adjacency_from_dense(matrix)
        .map_err(|_| StructuralError::InvalidInput("sparse adjacency"))?;
    let order = reverse_cuthill_mckee(&adjacency)
        .map_err(|_| StructuralError::InvalidInput("sparse order"))?;
    let mut factor = SymmetricProfileMatrix::from_dense_with_order(matrix, &order)
        .map_err(|_| StructuralError::InvalidInput("sparse profile"))?;
    let n = matrix.len();
    if free_dofs.len() != n {
        return Err(StructuralError::InvalidInput("sparse free map"));
    }
    let mut work = vec![0.0; n];
    let mut pivots = Vec::new();
    for i in 0..n {
        let first = factor.first_columns[i];
        for j in first..i {
            let mut sum = factor.stored(i, j);
            for k in first.max(factor.first_columns[j])..j {
                sum = structural::checked_value(
                    sum - structural::checked_product(work[k], factor.stored(j, k))?,
                )?;
            }
            work[j] = sum;
            let coefficient = structural::checked_quotient(sum, factor.stored(j, j))?;
            factor.set_stored(i, j, coefficient);
        }
        let mut pivot = factor.stored(i, i);
        let mut scale = pivot.abs();
        for k in first..i {
            let term = structural::checked_product(work[k], factor.stored(i, k))?;
            pivot = structural::checked_value(pivot - term)?;
            scale = structural::checked_value(scale + term.abs())?;
        }
        pivots.push(structural::screen_pivot(
            i,
            free_dofs[order[i]],
            pivot,
            scale,
            2 * (i - first) + 2,
        )?);
        factor.set_stored(i, i, pivot);
    }
    Ok(StructuralProfileFactor {
        factor,
        order,
        pivots,
    })
}
/// FULL retained global equations are required; symmetry is audited before profile conversion.
pub fn solve_structural_sparse(
    system: &StructuralSystem<'_>,
) -> Result<StructuralSolution, StructuralError> {
    let prepared = structural::prepare_structural(system)?;
    let factor = match factor_structural_ldlt(&prepared.matrix, system.free_dofs) {
        Ok(factor) => factor,
        Err(error) => {
            return Err(structural::negative_pair_witness(system, &prepared)?.unwrap_or(error))
        }
    };
    structural::finish_structural(
        system,
        &prepared,
        factor.pivots.clone(),
        "positive skyline LDL",
        |rhs| factor.solve(rhs),
    )
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
}
