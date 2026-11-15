//! DEL-04-01 VER-007: the two constructed systems of
//! core/solver/frame_kernel/tests/residual_refusal_witnesses.rs, under the
//! product's own ordering for the sparse finish (reverse Cuthill-McKee, through
//! both of this crate's entries). Each returns a result, marked sensitive,
//! meeting REQ-014 row by row. Together with the kernel witnesses this documents
//! that the sparse refusal depends on the natural order (it puts the arrowhead's
//! hub first; RCM puts it second-last, and the factor has no numerical fill), so
//! a change to either outcome is caught. It does not exercise the product end to end.

use open_pipe_stress_frame_kernel::structural::{
    SolveQuality, SparseStiffness, SparseStructuralSystem, StructuralError, StructuralSolution, StructuralSystem,
};
use open_pipe_stress_sparse_direct::structural::{solve_sparse_structural, solve_structural_sparse};

const SPARSE_WITNESS: &str = include_str!("../../frame_kernel/tests/residual_refusal/sparse_natural_order_arrowhead.hex");
const DENSE_WITNESS: &str = include_str!("../../frame_kernel/tests/residual_refusal/dense_arrowhead.hex");

fn parse(text: &str) -> (Vec<Vec<f64>>, Vec<f64>) {
    let hex = |s: &str| f64::from_bits(u64::from_str_radix(s, 16).expect("hex"));
    let mut lines = text.lines().filter(|l| !l.starts_with('#'));
    let n: usize = lines.next().and_then(|l| l.strip_prefix("n ")).expect("n").parse().expect("n");
    let k: Vec<Vec<f64>> = (0..n).map(|_| lines.next().expect("row").split(' ').map(hex).collect()).collect();
    let f: Vec<f64> = lines.next().and_then(|l| l.strip_prefix("f ")).expect("f").split(' ').map(hex).collect();
    (k, f)
}

fn assert_returned_sensitive(result: Result<StructuralSolution, StructuralError>, what: &str) {
    let solution = result.unwrap_or_else(|e| panic!("{what}: refused: {e:?}"));
    assert_eq!(solution.report.quality, SolveQuality::Sensitive, "{what}");
    assert!(solution.report.residual_rows.iter().all(|r| r.passed), "{what}: REQ-014 rows");
}

fn both_entries(text: &str, name: &str) {
    let (k, f) = parse(text);
    let free: Vec<usize> = (0..k.len()).collect();
    let stiffness = SparseStiffness::from_dense(&k).expect("stiffness");
    let system = SparseStructuralSystem::new(&stiffness, &f, &free, &[], None, None);
    assert_returned_sensitive(solve_sparse_structural(&system), &format!("{name}: pattern path"));
    assert_returned_sensitive(
        solve_structural_sparse(&StructuralSystem {
            stiffness: &k,
            force: &f,
            free_dofs: &free,
            prescribed: &[],
            contributions: None,
            contribution_groups: None,
            symmetry: None,
        }),
        &format!("{name}: dense-derived path"),
    );
}

#[test]
fn both_constructed_systems_return_a_result_under_the_products_ordering() {
    both_entries(SPARSE_WITNESS, "sparse witness");
    both_entries(DENSE_WITNESS, "dense witness");
}
