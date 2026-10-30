//! T4-U5b round 3: the absorbed-diagonal refusal decided per element
//! (`ElementAbsorption`), on the dense and the sparse contribution sums.
use super::*;
use crate::structural::sparse::{SparseStiffness, SparseStructuralSystem};

fn c(row: usize, col: usize, value: f64) -> StiffnessContribution {
    StiffnessContribution { row, col, value }
}

/// Two DOFs. Element A stiffens both diagonals to 1. Element B adds 2^-60 to
/// DOF 0 (absorbed) and, when `represented`, 2^-10 to DOF 1 (not absorbed).
fn entries(represented: bool) -> Vec<StiffnessContribution> {
    vec![
        c(0, 0, 1.0),
        c(1, 1, 1.0),
        c(0, 0, 2f64.powi(-60)),
        c(1, 1, if represented { 2f64.powi(-10) } else { 2f64.powi(-61) }),
    ]
}

fn dense(entries: &[StiffnessContribution], groups: Option<&[usize]>) -> Result<(), StructuralError> {
    dense_with(entries, groups, &[0, 1], &[])
}

fn dense_with(entries: &[StiffnessContribution], groups: Option<&[usize]>, free: &[usize], prescribed: &[(usize, f64)]) -> Result<(), StructuralError> {
    let k = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
    let force = vec![0.0, 0.0];
    let system = StructuralSystem {
        stiffness: &k,
        force: &force,
        free_dofs: free,
        prescribed,
        contributions: Some(entries),
        contribution_groups: groups,
        symmetry: None,
    };
    contribution_sums(&system).map(|_| ())
}

fn sparse(entries: &[StiffnessContribution], groups: Option<&[usize]>) -> Result<(), StructuralError> {
    sparse_with(entries, groups, &[0, 1], &[])
}

fn sparse_with(entries: &[StiffnessContribution], groups: Option<&[usize]>, free: &[usize], prescribed: &[(usize, f64)]) -> Result<(), StructuralError> {
    // A full 2x2 pattern: the sums read only the pattern, never these values.
    let k = SparseStiffness::from_dense(&[vec![1.0, 0.5], vec![0.5, 1.0]]).unwrap();
    let force = vec![0.0, 0.0];
    let system = SparseStructuralSystem::new(&k, &force, free, prescribed, Some(entries), None);
    let system = match groups { Some(g) => system.with_contribution_groups(g), None => system };
    sparse::sparse_contribution_sums(&system).map(|_| ())
}

fn refused_at(result: Result<(), StructuralError>, dof: usize) {
    match result {
        Err(StructuralError::NumericallyUnresolved { reason, global_dof }) => {
            assert_eq!(reason, "positive diagonal contribution absorbed by assembly; stabilization unresolved");
            assert_eq!(global_dof, Some(dof));
        }
        other => panic!("expected the absorption refusal at {dof}: {other:?}"),
    }
}

#[test]
fn without_groups_every_contribution_is_its_own_element() {
    for check in [dense, sparse] {
        refused_at(check(&entries(true), None), 0);
    }
}

#[test]
fn an_element_is_refused_only_when_every_candidate_is_absorbed() {
    let groups = [2, 4];
    for check in [dense, sparse] {
        // B's DOF 1 entry is represented: B is not refused.
        assert!(check(&entries(true), Some(&groups)).is_ok());
        // Both of B's candidates absorbed: refused at its first absorbed DOF.
        refused_at(check(&entries(false), Some(&groups)), 0);
        // A's entries open their diagonals (no candidates); one element over all.
        assert!(check(&entries(true), Some(&[4])).is_ok());
    }
}

#[test]
fn malformed_groups_are_refused_as_invalid_input() {
    for groups in [&[1, 1, 4][..], &[0, 4], &[2, 3], &[3, 2, 4], &[]] {
        for check in [dense, sparse] {
            assert!(matches!(check(&entries(true), Some(groups)), Err(StructuralError::InvalidInput("contribution groups"))), "{groups:?}");
        }
    }
}

/// A represented diagonal at a prescribed DOF never enters the free system:
/// with DOF 1 prescribed, B's only candidate (DOF 0) is absorbed and B is
/// refused at it; with both DOFs free, B's DOF 1 entry is represented.
#[test]
fn diagonals_at_prescribed_dofs_are_no_candidates() {
    let groups = [2, 4];
    for check in [dense_with, sparse_with] {
        refused_at(check(&entries(true), Some(&groups), &[0], &[(1, 0.0)]), 0);
        assert!(check(&entries(true), Some(&groups), &[0, 1], &[]).is_ok());
    }
}

/// Round 4 (Codex): A = diag(1, 0), B = v v^T with v = (2^-30, 1). B's DOF 0
/// entry is absorbed, but its DOF 1 entry opens a zero free diagonal and is
/// represented, so B is not refused; without groups the per-entry rule still
/// refuses the absorbed entry. With DOF 1 prescribed, B has no represented
/// candidate and is refused.
#[test]
fn an_element_opening_a_free_zero_diagonal_is_represented() {
    let v = [2f64.powi(-30), 1.0];
    let mut entries = vec![c(0, 0, 1.0)];
    for (i, &a) in v.iter().enumerate() {
        for (j, &b) in v.iter().enumerate() {
            entries.push(c(i, j, a * b));
        }
    }
    let groups = [1, 5];
    for (check, check_with) in [(dense as fn(&[StiffnessContribution], Option<&[usize]>) -> Result<(), StructuralError>, dense_with as fn(&[StiffnessContribution], Option<&[usize]>, &[usize], &[(usize, f64)]) -> Result<(), StructuralError>), (sparse, sparse_with)] {
        assert!(check(&entries, Some(&groups)).is_ok());
        refused_at(check(&entries, None), 0);
        refused_at(check_with(&entries, Some(&groups), &[0], &[(1, 0.0)]), 0);
    }
}
