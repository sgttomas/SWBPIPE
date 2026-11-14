//! DEL-04-01 AC-012 and VER-007, second half: a solve that cannot meet REQ-014
//! returns no result and the named finding, in both modes.
//!
//! No admitted system is known to reach this refusal: with an exact residual and
//! binary64 corrections, the first solve already met the per-row limit on every
//! system that passes the pivot screen in the search recorded with this test's
//! pull request. So the non-converging refinement is forced at kernel level by
//! the test-only seam `residual_seam` (a biased backend), and this test checks
//! that the loop's bound then refuses with the named reason and returns no
//! solution, through the dense and the sparse finish.

use super::residual_seam;
use super::*;

const REASON: &str = "original-equation residual failed after bounded refinement";

fn k() -> Vec<Vec<f64>> {
    vec![vec![4.0, 0.0, 1.5], vec![0.0, 4.0, 0.0], vec![1.5, 0.0, 4.0]]
}
const FORCE: [f64; 3] = [1.0, -2.0, 3.0];
const FREE: [usize; 3] = [0, 1, 2];

fn dense(k: &[Vec<f64>]) -> Result<StructuralSolution, StructuralError> {
    solve_structural_dense(&StructuralSystem {
        stiffness: k,
        force: &FORCE,
        free_dofs: &FREE,
        prescribed: &[],
        contributions: None,
        contribution_groups: None,
        symmetry: None,
    })
}

fn sparse(k: &[Vec<f64>]) -> Result<StructuralSolution, StructuralError> {
    let stiffness = SparseStiffness::from_dense(k)?;
    let system = SparseStructuralSystem::new(&stiffness, &FORCE, &FREE, &[], None, None);
    let prepared = prepare_sparse_structural(&system)?;
    let n = FREE.len();
    let order: Vec<usize> = (0..n).collect();
    let first: Vec<usize> = (0..n).map(|i| (0..i).find(|&j| k[i][j] != 0.0).unwrap_or(i)).collect();
    let factor = factor_sparse_structural_profile(&prepared, &order, &first)?;
    finish_sparse_structural(&factor)
}

fn assert_refused(result: Result<StructuralSolution, StructuralError>, mode: &str) {
    match result {
        Err(StructuralError::NumericallyUnresolved { reason, global_dof }) => {
            assert_eq!(reason, REASON, "{mode}");
            assert_eq!(global_dof, None, "{mode}");
        }
        Err(other) => panic!("{mode}: refused for another reason: {other:?}"),
        Ok(solution) => panic!("{mode}: a solution was returned: {:?}", solution.report.residual_rows),
    }
}

#[test]
fn a_refinement_that_cannot_converge_returns_no_solution_and_the_named_finding_in_both_modes() {
    let k = k();
    // Control: the same system solves, and meets REQ-014, without the seam.
    for (mode, result) in [("dense", dense(&k)), ("sparse", sparse(&k))] {
        let solution = result.unwrap_or_else(|e| panic!("{mode}: control refused: {e:?}"));
        assert!(solution.report.residual_rows.iter().all(|r| r.passed), "{mode}: control residual");
    }
    // Forced: the biased backend keeps every iterate outside the limit.
    let _seam = residual_seam::Active::new();
    assert_refused(dense(&k), "dense");
    assert_refused(sparse(&k), "sparse");
}

#[test]
fn the_seam_is_off_unless_a_test_holds_it() {
    let mut y = vec![1.0, -3.0];
    residual_seam::bias(&mut y);
    assert_eq!(y, vec![1.0, -3.0]);
    {
        let _seam = residual_seam::Active::new();
        residual_seam::bias(&mut y);
        assert_ne!(y, vec![1.0, -3.0]);
    }
    let after = y.clone();
    residual_seam::bias(&mut y);
    assert_eq!(y, after, "dropping the guard turns the seam off");
}
