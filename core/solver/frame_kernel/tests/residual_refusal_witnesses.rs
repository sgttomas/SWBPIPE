//! DEL-04-01 AC-012 and VER-007, second half: a solve whose bounded refinement
//! does not bring every free row within REQ-014's limit returns no result and the
//! named finding, through each finish, on a constructed system.
//!
//! Two fixtures in residual_refusal/, each an arrowhead placed at the condition
//! gate's edge with a solution spread over many binades (construction in each
//! file's header, regenerated and checked bit for bit below):
//! - sparse_natural_order_arrowhead.hex: the sparse finish under natural order and
//!   the first-nonzero profile refuses; the dense finish returns a result for it.
//! - dense_arrowhead.hex: the dense finish refuses; the sparse finish under
//!   natural order returns a result for it.
//! The same system can be refused by one finish and returned by the other, in
//! either direction. Each refusal says that this solve does not meet the limit
//! within the kernel's refinement policy, not that the system cannot satisfy the
//! residual requirement. Every result returned for these systems is marked
//! sensitive (their condition estimates are near the gate). The product's
//! ordering for the sparse finish (sparse_direct's reverse Cuthill-McKee) returns
//! a result for both: core/solver/sparse_direct/tests/residual_refusal_rcm.rs.

use open_pipe_stress_frame_kernel::structural::{
    factor_sparse_structural_profile, finish_sparse_structural, prepare_sparse_structural, solve_structural_dense,
    SolveQuality, SparseStiffness, SparseStructuralSystem, StructuralError, StructuralSolution, StructuralSystem,
};

const REASON: &str = "original-equation residual failed after bounded refinement";
const SPARSE_WITNESS: &str = include_str!("residual_refusal/sparse_natural_order_arrowhead.hex");
const DENSE_WITNESS: &str = include_str!("residual_refusal/dense_arrowhead.hex");

fn dense(k: &[Vec<f64>], f: &[f64]) -> Result<StructuralSolution, StructuralError> {
    let free: Vec<usize> = (0..k.len()).collect();
    solve_structural_dense(&StructuralSystem {
        stiffness: k,
        force: f,
        free_dofs: &free,
        prescribed: &[],
        contributions: None,
        contribution_groups: None,
        symmetry: None,
    })
}

/// The sparse finish under natural order and the first-nonzero profile, as the
/// kernel's caller-supplied-ordering entry accepts and validates them.
fn sparse_natural(k: &[Vec<f64>], f: &[f64]) -> Result<StructuralSolution, StructuralError> {
    let n = k.len();
    let free: Vec<usize> = (0..n).collect();
    let stiffness = SparseStiffness::from_dense(k)?;
    let system = SparseStructuralSystem::new(&stiffness, f, &free, &[], None, None);
    let prepared = prepare_sparse_structural(&system)?;
    let order: Vec<usize> = (0..n).collect();
    let first: Vec<usize> = (0..n).map(|i| (0..i).find(|&j| k[i][j] != 0.0).unwrap_or(i)).collect();
    let factor = factor_sparse_structural_profile(&prepared, &order, &first)?;
    finish_sparse_structural(&factor)
}

fn assert_refused(result: Result<StructuralSolution, StructuralError>, what: &str) {
    match result {
        Err(StructuralError::NumericallyUnresolved { reason, global_dof }) => {
            assert_eq!(reason, REASON, "{what}");
            assert_eq!(global_dof, None, "{what}");
        }
        Err(other) => panic!("{what}: refused for another reason: {other:?}"),
        Ok(solution) => panic!("{what}: a result was returned: {:?}", solution.report.residual_rows),
    }
}

/// A returned result, marked sensitive, meeting REQ-014 row by row.
fn assert_returned_sensitive(result: Result<StructuralSolution, StructuralError>, what: &str) {
    let solution = result.unwrap_or_else(|e| panic!("{what}: refused: {e:?}"));
    assert_eq!(solution.report.quality, SolveQuality::Sensitive, "{what}");
    assert!(solution.report.residual_rows.iter().all(|r| r.passed), "{what}: REQ-014 rows");
}

/// K, f and x from a fixture's hex.
fn parse(text: &str) -> (Vec<Vec<f64>>, Vec<f64>, Vec<f64>) {
    let hex = |s: &str| f64::from_bits(u64::from_str_radix(s, 16).expect("hex"));
    let mut lines = text.lines().filter(|l| !l.starts_with('#'));
    let n: usize = lines.next().and_then(|l| l.strip_prefix("n ")).expect("n").parse().expect("n");
    let k: Vec<Vec<f64>> = (0..n).map(|_| lines.next().expect("row").split(' ').map(hex).collect()).collect();
    let f: Vec<f64> = lines.next().and_then(|l| l.strip_prefix("f ")).expect("f").split(' ').map(hex).collect();
    let x: Vec<f64> = lines.next().and_then(|l| l.strip_prefix("x ")).expect("x").split(' ').map(hex).collect();
    assert!(lines.next().is_none(), "trailing lines");
    (k, f, x)
}

struct Construction {
    n: usize,
    spread: u64,
    seed: u64,
    step: u64,
    range: i32,
    t_bits: u64,
}

/// The construction in the fixtures' headers, in plain binary64 arithmetic.
fn regenerate(c: &Construction) -> (Vec<Vec<f64>>, Vec<f64>, Vec<f64>) {
    struct SplitMix(u64);
    impl SplitMix {
        fn next(&mut self) -> u64 {
            self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
            let mut z = self.0;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
            z ^ (z >> 31)
        }
    }
    let n = c.n;
    let t = f64::from_bits(c.t_bits);
    let mut rng = SplitMix(11000 + c.seed * 977 + n as u64);
    let springs: Vec<f64> = (1..n).map(|_| 2.0f64.powi((rng.next() % (c.spread + 1)) as i32)).collect();
    let grounds: Vec<f64> = (1..n).map(|_| (1.0 + (rng.next() % 7) as f64) / t).collect();
    let mut k = vec![vec![0.0; n]; n];
    k[0][0] = 1.0 / t;
    for (i, (&s, &g)) in springs.iter().zip(&grounds).enumerate() {
        let j = i + 1;
        k[0][0] += s;
        k[j][j] += s + g;
        k[0][j] -= s;
        k[j][0] -= s;
    }
    let mut rng = SplitMix(12000 + c.seed + c.step * 7 + c.range as u64);
    let x: Vec<f64> = (0..n)
        .map(|i| {
            let exponent = if i % 2 == 1 { c.range } else { (rng.next() % 4) as i32 };
            2.0f64.powi(exponent) * if rng.next() % 2 == 0 { 1.0 } else { -1.0 }
        })
        .collect();
    let f: Vec<f64> = k.iter().map(|row| row.iter().zip(&x).map(|(a, b)| a * b).sum()).collect();
    (k, f, x)
}

const SPARSE_CONSTRUCTION: Construction = Construction { n: 64, spread: 20, seed: 1, step: 12, range: 40, t_bits: 0x41b9a3c9f5983550 };
const DENSE_CONSTRUCTION: Construction = Construction { n: 96, spread: 30, seed: 8, step: 10, range: 46, t_bits: 0x411a63a067f6457d };

fn assert_bits_equal(text: &str, c: &Construction, name: &str) {
    let (k, f, x) = parse(text);
    let (k2, f2, x2) = regenerate(c);
    let bits = |v: &[f64]| v.iter().map(|x| x.to_bits()).collect::<Vec<_>>();
    assert_eq!(k.iter().map(|r| bits(r)).collect::<Vec<_>>(), k2.iter().map(|r| bits(r)).collect::<Vec<_>>(), "{name}: K");
    assert_eq!(bits(&f), bits(&f2), "{name}: f");
    assert_eq!(bits(&x), bits(&x2), "{name}: x");
}

#[test]
fn each_fixture_is_its_stated_construction() {
    assert_bits_equal(SPARSE_WITNESS, &SPARSE_CONSTRUCTION, "sparse witness");
    assert_bits_equal(DENSE_WITNESS, &DENSE_CONSTRUCTION, "dense witness");
}

#[test]
fn the_sparse_finish_under_natural_order_returns_no_result_and_the_named_finding() {
    let (k, f, _) = parse(SPARSE_WITNESS);
    assert_refused(sparse_natural(&k, &f), "sparse witness, sparse finish, natural order");
    // Control: the dense finish returns a result for the same system.
    assert_returned_sensitive(dense(&k, &f), "sparse witness, dense finish");
}

#[test]
fn the_dense_finish_returns_no_result_and_the_named_finding() {
    let (k, f, _) = parse(DENSE_WITNESS);
    assert_refused(dense(&k, &f), "dense witness, dense finish");
    // Control: the sparse finish under natural order returns a result for the same system.
    assert_returned_sensitive(sparse_natural(&k, &f), "dense witness, sparse finish, natural order");
}
