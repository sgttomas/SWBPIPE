//! Prospective product-preview continuous equilibrium measurement. This evidence
//! supplements, and never replaces, structural factor/fidelity and contact gates.
use open_pipe_stress_frame_kernel::structural::{
    self, ResidualRow, StructuralError, StructuralSystem,
};
pub const POLICY: &str = "M03-PRODUCT-PREVIEW-EQUILIBRIUM-v1";
#[derive(Debug, Clone, PartialEq)]
pub struct WorkRow {
    pub global_dof: usize,
    pub observed_work: f64,
    pub evaluation_allowance: f64,
    pub derived_target: f64,
    pub normalized_work: f64,
    pub normalized_allowance: f64,
    pub normalized_target: f64,
    pub scale_exponent: i32,
    pub passed: bool,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ProductEquilibriumReport {
    pub policy: &'static str,
    pub rows: Vec<ResidualRow>,
    pub work_rows: Vec<WorkRow>,
    /// Governs the measured work only; no exact-arithmetic maximum is claimed.
    pub observed_governing_work_dof: Option<usize>,
    pub passed: bool,
}
fn exponent(x: f64) -> i32 {
    let bits = x.abs().to_bits();
    let e = ((bits >> 52) & 0x7ff) as i32;
    if e != 0 {
        e - 1023
    } else {
        -1074 + (63 - (bits & ((1u64 << 52) - 1)).leading_zeros() as i32)
    }
}
fn record(mut value: f64, mut power: i32) -> Result<f64, StructuralError> {
    while power != 0 {
        let step = power.clamp(-512, 512);
        value *= 2f64.powi(step);
        power -= step;
        if !value.is_finite() {
            return Err(StructuralError::Range(
                "product residual-work record overflow",
            ));
        }
    }
    Ok(value)
}
/// Re-evaluate immutable full original equations and prescribed compatibility.
/// Calling this function alone does not establish structural or contact validity.
pub fn evaluate(
    system: &StructuralSystem<'_>,
    u: &[f64],
) -> Result<ProductEquilibriumReport, StructuralError> {
    let rows = structural::evaluate_original_residual(system, u)?;
    let mut work_rows = Vec::with_capacity(rows.len());
    for row in &rows {
        let value = u[row.global_dof].abs();
        let power = if value == 0.0 { 0 } else { exponent(value) };
        let mantissa = if value == 0.0 {
            0.0
        } else {
            structural::radix_scale(value, -power)?
        };
        let work = structural::checked_product(mantissa, row.normalized_residual.abs())?;
        let allowance = structural::checked_product(mantissa, row.normalized_evaluation_allowance)?;
        let denominator = structural::checked_quotient(
            row.normalized_denominator,
            1.0 + structural::gamma(row.operation_count),
        )?;
        let target = structural::checked_product(
            mantissa,
            structural::checked_product(row.target, denominator)?,
        )?;
        let scale = power + row.row_scale_exponent;
        // The authoritative equilibrium guard already includes evaluation and
        // denominator reserves. Multiplying both sides by |u_i| preserves it;
        // u_i=0 never changes a failed equilibrium row into a passing row.
        work_rows.push(WorkRow {
            global_dof: row.global_dof,
            observed_work: record(work, scale)?,
            evaluation_allowance: record(allowance, scale)?,
            derived_target: record(target, scale)?,
            normalized_work: work,
            normalized_allowance: allowance,
            normalized_target: target,
            scale_exponent: scale,
            passed: row.passed,
        });
    }
    let governing = work_rows
        .iter()
        .max_by(|a, b| a.observed_work.total_cmp(&b.observed_work))
        .map(|r| r.global_dof);
    let passed = rows.iter().all(|r| r.passed) && work_rows.iter().all(|r| r.passed);
    Ok(ProductEquilibriumReport {
        policy: POLICY,
        rows,
        work_rows,
        observed_governing_work_dof: governing,
        passed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn successor_both_backends_original_load_and_prescribed_mutations() {
        for sparse in [false, true] {
            let k = vec![vec![2.0, -1.0], vec![-1.0, 2.0]];
            let system = StructuralSystem {
                stiffness: &k,
                force: &[99.0, 3.0],
                free_dofs: &[1],
                prescribed: &[(0, 1.0)],
                contributions: None,
                symmetry: None,
            };
            let solved = if sparse {
                open_pipe_stress_sparse_direct::structural::solve_structural_sparse(&system)
            } else {
                structural::solve_structural_dense(&system)
            }
            .unwrap();
            let report = evaluate(&system, &solved.displacements).unwrap();
            assert!(report.passed);
            assert_eq!(report.policy, POLICY);
            assert_eq!(report.rows[0].global_dof, 1);
            assert_eq!(solved.displacements[0], 1.0);
            assert!((solved.displacements[1] - 2.0).abs() <= 1e-9 * 2.0);
            let mut wrong = solved.displacements.clone();
            wrong[0] = 0.0;
            assert!(matches!(
                evaluate(&system, &wrong),
                Err(StructuralError::InvalidInput(
                    "prescribed displacement incompatibility"
                ))
            ));
            let tiny = vec![vec![1.0]];
            let original = StructuralSystem {
                stiffness: &tiny,
                force: &[1e-12],
                free_dofs: &[0],
                prescribed: &[],
                contributions: None,
                symmetry: None,
            };
            let solved = if sparse {
                open_pipe_stress_sparse_direct::structural::solve_structural_sparse(&original)
            } else {
                structural::solve_structural_dense(&original)
            }
            .unwrap();
            assert!(evaluate(&original, &solved.displacements).unwrap().passed);
            let omitted = evaluate(&original, &[0.0]).unwrap();
            assert!(!omitted.passed);
            assert!(!omitted.work_rows[0].passed);
            assert_eq!(omitted.work_rows[0].observed_work, 0.0);
            assert!(omitted.rows[0].guarded_ratio > omitted.rows[0].target);
            assert_eq!(omitted.rows[0].residual, -1e-12);
        }
    }
}
