//! Dense-path references: closed-form pencils and an independent cyclic
//! Jacobi solver on C = L⁻¹ M L⁻ᵀ.
//!
//! The reference code in this file (Cholesky, triangular solves, Jacobi)
//! shares no code with the crate's QL path; it uses only the public API to
//! call the path under test.
//!
//! Tolerances. Two first-order error sources are bounded, each with a margin
//! of 64·n for the unstated constants, and in the ∞-norm (an upper bound on
//! the 2-norm of a symmetric matrix):
//! 1. Forming and solving C: a backward error of about u·‖C‖₂ ≤
//!    u·‖M‖₂·‖K⁻¹‖₂ (Golub & Van Loan, 4th ed., §8.7), an absolute error in μ.
//! 2. The Cholesky factorization: R̂R̂ᵀ = K + E with |E| ≤ γ·|R||Rᵀ| (Higham,
//!    *Accuracy and Stability of Numerical Algorithms*, Thm 10.3). By the
//!    Rayleigh quotient it moves λ (and so μ) by a relative ‖E‖·‖K⁻¹‖,
//!    which is u·g_R with g_R = ‖|R||Rᵀ|‖∞·‖K⁻¹‖∞. Where the factorization
//!    cancels (a stiff element beside a soft one), g_R is about κ(K).
//! So |μ̂ − μ| ≤ TOL_μ(μ) = 64·n·u·(‖M‖∞‖K⁻¹‖∞ + μ·g_R).
//! For vectors, z = Rᵀφ is the eigenvector of C, so the angle between two
//! shapes is measured in the K inner product. The two sources perturb the
//! operator K⁻¹M, in that inner product, by at most TOL_μ(μ_max); by
//! Davis–Kahan an eigenvector moves by sin θ ≤ TOL_μ(μ_max) / gap, with gap
//! the distance from μ to the rest of the spectrum. A comparison of two
//! computed results allows twice each bound.

use open_pipe_stress_modal_eigen::clusters::cluster_ids;
use open_pipe_stress_modal_eigen::dense::{solve_dense_pencil, DenseEigenSolution, DenseOptions};
use open_pipe_stress_modal_eigen::modal_quantities::{
    cumulative_effective_mass_fractions, normalize_mode, Dof, FreeDof, ModalBasis,
    NormalizationCase,
};
use open_pipe_stress_modal_eigen::orthogonality::m_orthogonality;
use open_pipe_stress_modal_eigen::SymmetricMatrix;

const U: f64 = f64::EPSILON / 2.0;

/// Test-local resolution-floor parameters c_μ and κ_μ (design §3.2 item 5);
/// the production values are recorded under DEC-026, not here. With
/// c_μ = 1, the review-round-3 artifact lies about 2× below δ_μ and the
/// smallest retained μ of every admitted fixture case at least 7.4·10² above
/// it (E-RIGID-SCALE-1e6), so κ_μ = 10 sits between them (README).
const RESOLUTION_FLOOR_CONSTANT: f64 = 1.0;
const RESOLUTION_FLOOR_MULTIPLE: f64 = 10.0;

/// The smallest retained μ over κ_μ·δ_μ: how far the resolution floor is
/// below the retained spectrum (+∞ with a zero floor or no mode).
fn floor_margin(solution: &DenseEigenSolution) -> f64 {
    let smallest = solution.mu.iter().copied().fold(f64::INFINITY, f64::min);
    smallest / (RESOLUTION_FLOOR_MULTIPLE * solution.resolution_floor)
}

/// Test-local algorithm parameters; the production values are fixed by the
/// design (M1b), not by these tests.
fn options() -> DenseOptions {
    DenseOptions {
        infinite_mu_relative_threshold: 1e-12,
        mass_pivot_relative_threshold: 1e-12,
        resolution_floor_constant: RESOLUTION_FLOOR_CONSTANT,
        resolution_floor_multiple: RESOLUTION_FLOOR_MULTIPLE,
        max_ql_iterations_per_eigenvalue: 30,
    }
}

type Rows = Vec<Vec<f64>>;

fn sym(rows: &Rows) -> SymmetricMatrix {
    SymmetricMatrix::from_rows(rows).unwrap()
}

fn solve(k: &Rows, m: &Rows) -> DenseEigenSolution {
    solve_dense_pencil(&sym(k), &sym(m), &options()).unwrap()
}

fn diagonal(values: &[f64]) -> Rows {
    let n = values.len();
    let mut rows = vec![vec![0.0; n]; n];
    for i in 0..n {
        rows[i][i] = values[i];
    }
    rows
}

fn second_difference(n: usize) -> Rows {
    let mut rows = vec![vec![0.0; n]; n];
    for i in 0..n {
        rows[i][i] = 2.0;
        if i + 1 < n {
            rows[i][i + 1] = -1.0;
            rows[i + 1][i] = -1.0;
        }
    }
    rows
}

/// Fixed-free spring chain with element stiffnesses `k` (element 0 ties
/// DOF 0 to the support).
fn spring_chain(k: &[f64]) -> Rows {
    let n = k.len();
    let mut rows = vec![vec![0.0; n]; n];
    for (e, &ke) in k.iter().enumerate() {
        rows[e][e] += ke;
        if e > 0 {
            rows[e - 1][e - 1] += ke;
            rows[e - 1][e] -= ke;
            rows[e][e - 1] -= ke;
        }
    }
    rows
}

// ---- Independent reference: Cholesky, solves, cyclic Jacobi ----

/// Right-looking (outer-product) Cholesky, A = L Lᵀ.
fn ref_cholesky(a: &Rows) -> Rows {
    let n = a.len();
    let mut w = a.clone();
    let mut l = vec![vec![0.0; n]; n];
    for j in 0..n {
        let pivot = w[j][j].sqrt();
        assert!(pivot > 0.0, "reference Cholesky: not SPD at {j}");
        l[j][j] = pivot;
        for i in j + 1..n {
            l[i][j] = w[i][j] / pivot;
        }
        for i in j + 1..n {
            for c in j + 1..=i {
                w[i][c] -= l[i][j] * l[c][j];
            }
        }
    }
    l
}

/// Solves L X = B column by column.
fn ref_lower_solve(l: &Rows, b: &Rows) -> Rows {
    let n = l.len();
    let cols = b[0].len();
    let mut x = vec![vec![0.0; cols]; n];
    for c in 0..cols {
        for i in 0..n {
            let mut s = b[i][c];
            for p in 0..i {
                s -= l[i][p] * x[p][c];
            }
            x[i][c] = s / l[i][i];
        }
    }
    x
}

fn ref_transpose(a: &Rows) -> Rows {
    let (r, c) = (a.len(), a[0].len());
    (0..c).map(|j| (0..r).map(|i| a[i][j]).collect()).collect()
}

/// C = L⁻¹ M L⁻ᵀ = L⁻¹ (L⁻¹ M)ᵀ, symmetrized by averaging.
fn ref_c(l: &Rows, m: &Rows) -> Rows {
    let y = ref_lower_solve(l, m);
    let c = ref_lower_solve(l, &ref_transpose(&y));
    let n = c.len();
    (0..n)
        .map(|i| (0..n).map(|j| 0.5 * (c[i][j] + c[j][i])).collect())
        .collect()
}

/// Cyclic Jacobi (row-cyclic sweeps, Rutishauser's rotation formulas) until
/// the off-diagonal Frobenius norm is below 1e-15·‖A‖_F of the input, which
/// is below the 64·n·u test tolerances. Returns eigenvalues ascending and the
/// matching eigenvectors as rows.
fn ref_jacobi(a: &Rows) -> (Vec<f64>, Rows) {
    let n = a.len();
    let mut a = a.clone();
    let mut v: Rows = (0..n)
        .map(|i| (0..n).map(|j| if i == j { 1.0 } else { 0.0 }).collect())
        .collect();
    let frob = a.iter().flatten().map(|x| x * x).sum::<f64>().sqrt();
    for _sweep in 0..100 {
        let off: f64 = (0..n)
            .flat_map(|i| (0..n).filter(move |&j| j != i).map(move |j| (i, j)))
            .map(|(i, j)| a[i][j] * a[i][j])
            .sum::<f64>()
            .sqrt();
        if off <= 1e-15 * frob {
            break;
        }
        for p in 0..n {
            for q in p + 1..n {
                if a[p][q] == 0.0 {
                    continue;
                }
                let theta = (a[q][q] - a[p][p]) / (2.0 * a[p][q]);
                let t = theta.signum() / (theta.abs() + (theta * theta + 1.0).sqrt());
                let c = 1.0 / (t * t + 1.0).sqrt();
                let s = t * c;
                for k in 0..n {
                    let (akp, akq) = (a[k][p], a[k][q]);
                    a[k][p] = c * akp - s * akq;
                    a[k][q] = s * akp + c * akq;
                }
                for k in 0..n {
                    let (apk, aqk) = (a[p][k], a[q][k]);
                    a[p][k] = c * apk - s * aqk;
                    a[q][k] = s * apk + c * aqk;
                }
                for k in 0..n {
                    let (vkp, vkq) = (v[k][p], v[k][q]);
                    v[k][p] = c * vkp - s * vkq;
                    v[k][q] = s * vkp + c * vkq;
                }
            }
        }
    }
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&i, &j| a[i][i].partial_cmp(&a[j][j]).unwrap());
    let values = order.iter().map(|&i| a[i][i]).collect();
    let vectors = order
        .iter()
        .map(|&i| (0..n).map(|k| v[k][i]).collect())
        .collect();
    (values, vectors)
}

/// Reference modes: μ descending (λ ascending) and φ = L⁻ᵀ z.
fn ref_modes(k: &Rows, m: &Rows) -> (Vec<f64>, Rows) {
    let l = ref_cholesky(k);
    let (mut mu, mut z) = ref_jacobi(&ref_c(&l, m));
    mu.reverse();
    z.reverse();
    let n = k.len();
    let phis = z
        .iter()
        .map(|zi| {
            // Lᵀ φ = z by back substitution.
            let mut phi = vec![0.0; n];
            for i in (0..n).rev() {
                let mut s = zi[i];
                for p in i + 1..n {
                    s -= l[p][i] * phi[p];
                }
                phi[i] = s / l[i][i];
            }
            phi
        })
        .collect();
    (mu, phis)
}

fn inf_norm(a: &Rows) -> f64 {
    a.iter()
        .map(|row| row.iter().map(|x| x.abs()).sum::<f64>())
        .fold(0.0, f64::max)
}

/// TOL_μ(μ) = 64·n·u·(‖M‖∞‖K⁻¹‖∞ + μ·g_R) (see the file header).
struct Tolerance {
    scale: f64,
    reduction: f64,
    cholesky_growth: f64,
    mu_max: f64,
}

impl Tolerance {
    fn of(k: &Rows, m: &Rows) -> Self {
        let n = k.len();
        let l = ref_cholesky(k);
        let identity = diagonal(&vec![1.0; n]);
        let linv = ref_lower_solve(&l, &identity);
        // K⁻¹ = L⁻ᵀ L⁻¹, and |L||Lᵀ|.
        let kinv: Rows = (0..n)
            .map(|i| {
                (0..n)
                    .map(|j| (0..n).map(|p| linv[p][i] * linv[p][j]).sum())
                    .collect()
            })
            .collect();
        let abs_llt: Rows = (0..n)
            .map(|i| {
                (0..n)
                    .map(|j| (0..n).map(|p| (l[i][p] * l[j][p]).abs()).sum())
                    .collect()
            })
            .collect();
        let kinv_norm = inf_norm(&kinv);
        let (ref_mu, _) = ref_modes(k, m);
        Self {
            scale: 64.0 * n as f64 * U,
            reduction: inf_norm(m) * kinv_norm,
            cholesky_growth: inf_norm(&abs_llt) * kinv_norm,
            mu_max: ref_mu[0],
        }
    }

    /// Bound on |μ̂ − μ| at μ.
    fn mu(&self, mu: f64) -> f64 {
        self.scale * (self.reduction + mu * self.cholesky_growth)
    }

    /// Bound on sin θ in the K inner product, for a mode isolated by `gap`.
    fn vector(&self, gap: f64) -> f64 {
        self.mu(self.mu_max) / gap
    }
}

fn quad(a: &Rows, x: &[f64], y: &[f64]) -> f64 {
    (0..x.len())
        .map(|i| x[i] * (0..y.len()).map(|j| a[i][j] * y[j]).sum::<f64>())
        .sum()
}

/// sin of the K-inner-product angle between `x` and span(`basis`), with the
/// basis K-orthogonal. Formed from the projection residual, not from
/// 1 − MAC, so it resolves angles near u rather than near √u.
fn k_sin_angle(k: &Rows, x: &[f64], basis: &[&[f64]]) -> f64 {
    let mut r = x.to_vec();
    for b in basis {
        let coeff = quad(k, b, x) / quad(k, b, b);
        for (ri, bi) in r.iter_mut().zip(b.iter()) {
            *ri -= coeff * bi;
        }
    }
    (quad(k, &r, &r).max(0.0) / quad(k, x, x)).sqrt()
}

/// Distance from μ_i to the nearest other value in `mu`.
fn gap(mu: &[f64], i: usize) -> f64 {
    mu.iter()
        .enumerate()
        .filter(|&(j, _)| j != i)
        .map(|(_, &v)| (v - mu[i]).abs())
        .fold(f64::INFINITY, f64::min)
}

/// Asserts μ̂ against exact μ within TOL_μ(μ).
fn assert_mu(solution: &DenseEigenSolution, exact_mu: &[f64], tol: &Tolerance) {
    assert_eq!(solution.mode_count(), exact_mu.len());
    for (i, (&got, &want)) in solution.mu.iter().zip(exact_mu).enumerate() {
        let bound = tol.mu(want);
        assert!(
            (got - want).abs() <= bound,
            "mode {i}: μ {got:e} vs {want:e}, tol {bound:e}"
        );
        assert_eq!(solution.eigenvalues[i], 1.0 / got);
    }
}

/// Deterministic generator (64-bit LCG, Knuth's MMIX constants), values in
/// [−1, 1).
struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 52) as f64) - 1.0
    }
}

/// K = BBᵀ/n + I and either a diagonal M or a dense SPD M = diag + 0.1·CCᵀ/n.
fn random_pencil(n: usize, seed: u64, dense_mass: bool) -> (Rows, Rows) {
    let mut g = Lcg(seed);
    let b: Rows = (0..n).map(|_| (0..n).map(|_| g.next()).collect()).collect();
    let mut k = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            k[i][j] = (0..n).map(|p| b[i][p] * b[j][p]).sum::<f64>() / n as f64;
        }
        k[i][i] += 1.0;
    }
    let mut m = diagonal(&(0..n).map(|_| 1.5 + g.next()).collect::<Vec<_>>());
    if dense_mass {
        let c: Rows = (0..n).map(|_| (0..n).map(|_| g.next()).collect()).collect();
        for i in 0..n {
            for j in 0..n {
                m[i][j] += 0.1 * (0..n).map(|p| c[i][p] * c[j][p]).sum::<f64>() / n as f64;
            }
        }
    }
    (k, m)
}

// ---- Tests ----

#[test]
fn two_dof_pencils_match_closed_forms() {
    // K = [[2, −1], [−1, 1]], M = I: λ² − 3λ + 1 = 0, λ = (3 ∓ √5)/2.
    let k = vec![vec![2.0, -1.0], vec![-1.0, 1.0]];
    let m = diagonal(&[1.0, 1.0]);
    let large = (3.0 + 5.0f64.sqrt()) / 2.0;
    let small = 1.0 / large; // the roots' product is 1
    assert_mu(
        &solve(&k, &m),
        &[1.0 / small, 1.0 / large],
        &Tolerance::of(&k, &m),
    );

    // K = diag(1, 3), consistent-like M = [[2, 1], [1, 2]]:
    // 3λ² − 8λ + 3 = 0, λ = (4 ∓ √7)/3, product 1.
    let k = diagonal(&[1.0, 3.0]);
    let m = vec![vec![2.0, 1.0], vec![1.0, 2.0]];
    let large = (4.0 + 7.0f64.sqrt()) / 3.0;
    let small = 1.0 / large;
    let solution = solve(&k, &m);
    assert_mu(
        &solution,
        &[1.0 / small, 1.0 / large],
        &Tolerance::of(&k, &m),
    );
    // The shapes are M-orthogonal to the u level.
    let a = solution.vector(0);
    let b = solution.vector(1);
    let ratio = quad(&m, a, b).abs() / (quad(&m, a, a) * quad(&m, b, b)).sqrt();
    assert!(
        ratio <= 64.0 * 2.0 * U * solution.mu_max / solution.mu[1],
        "{ratio:e}"
    );
}

#[test]
fn second_difference_spectrum() {
    let n = 20;
    let k = second_difference(n);
    let theta = |j: usize| j as f64 * std::f64::consts::PI / (n + 1) as f64;
    // λ_j = 2 − 2cos(jπ/(n+1)) = 4 sin²(jπ/(2(n+1))), ascending in j.
    let exact_lambda: Vec<f64> = (1..=n)
        .map(|j| 4.0 * (theta(j) / 2.0).sin().powi(2))
        .collect();

    // M = I, with the closed-form shapes φ_j(i) = sin(i·jπ/(n+1)).
    let m = diagonal(&vec![1.0; n]);
    let tol = Tolerance::of(&k, &m);
    let solution = solve(&k, &m);
    let exact_mu: Vec<f64> = exact_lambda.iter().map(|l| 1.0 / l).collect();
    assert_mu(&solution, &exact_mu, &tol);
    for j in 0..n {
        let shape: Vec<f64> = (1..=n).map(|i| (i as f64 * theta(j + 1)).sin()).collect();
        let sin = k_sin_angle(&k, solution.vector(j), &[&shape]);
        assert!(
            sin <= tol.vector(gap(&exact_mu, j)),
            "mode {j}: sin θ {sin:e}"
        );
    }

    // M = 4I: λ/4, so μ = 4/λ.
    let m4 = diagonal(&vec![4.0; n]);
    let exact_mu4: Vec<f64> = exact_lambda.iter().map(|l| 4.0 / l).collect();
    assert_mu(&solve(&k, &m4), &exact_mu4, &Tolerance::of(&k, &m4));

    // Non-uniform diagonal M = D² with K' = D K D: K'φ = λD²φ ⇔ K(Dφ) = λ(Dφ),
    // so the spectrum is unchanged. D's entries are powers of two, so K' and
    // M are formed exactly.
    let d: Vec<f64> = (0..n).map(|i| [1.0, 2.0, 0.5][i % 3]).collect();
    let kd: Rows = (0..n)
        .map(|i| (0..n).map(|j| d[i] * k[i][j] * d[j]).collect())
        .collect();
    let md = diagonal(&d.iter().map(|x| x * x).collect::<Vec<_>>());
    assert_mu(&solve(&kd, &md), &exact_mu, &Tolerance::of(&kd, &md));
}

#[test]
fn exactly_repeated_pairs_form_clusters_and_subspaces() {
    // Two identical decoupled planes, interleaved: K = T₈ ⊗ I₂, M = I. Each
    // eigenvalue of T₈ is an exact pair, as in a straight circular pipe.
    let n_line = 8;
    let t = second_difference(n_line);
    let n = 2 * n_line;
    let mut k = vec![vec![0.0; n]; n];
    for i in 0..n_line {
        for j in 0..n_line {
            k[2 * i][2 * j] = t[i][j];
            k[2 * i + 1][2 * j + 1] = t[i][j];
        }
    }
    let m = diagonal(&vec![1.0; n]);
    let tol = Tolerance::of(&k, &m);
    let solution = solve(&k, &m);
    let theta = |j: usize| j as f64 * std::f64::consts::PI / (n_line + 1) as f64;
    let line_mu: Vec<f64> = (1..=n_line)
        .map(|j| 1.0 / (4.0 * (theta(j) / 2.0).sin().powi(2)))
        .collect();
    let exact_mu: Vec<f64> = line_mu.iter().flat_map(|&mu| [mu, mu]).collect();
    assert_mu(&solution, &exact_mu, &tol);

    // Test-local δ_c = 1e-10: the computed pairs differ by about u·μ_max,
    // far below it, and adjacent distinct values differ by more than 10%.
    let ids = cluster_ids(&solution.eigenvalues, 1e-10).unwrap();
    assert_eq!(ids, (0..n).map(|i| i / 2).collect::<Vec<_>>());

    for c in 0..n_line {
        let line: Vec<f64> = (1..=n_line)
            .map(|i| (i as f64 * theta(c + 1)).sin())
            .collect();
        let plane = |offset: usize| -> Vec<f64> {
            let mut v = vec![0.0; n];
            for i in 0..n_line {
                v[2 * i + offset] = line[i];
            }
            v
        };
        let (b0, b1) = (plane(0), plane(1));
        let cluster_gap = line_mu
            .iter()
            .enumerate()
            .filter(|&(j, _)| j != c)
            .map(|(_, &v)| (v - line_mu[c]).abs())
            .fold(f64::INFINITY, f64::min);
        for member in [2 * c, 2 * c + 1] {
            let sin = k_sin_angle(&k, solution.vector(member), &[&b0, &b1]);
            assert!(
                sin <= tol.vector(cluster_gap),
                "cluster {c} member {member}: sin θ {sin:e}"
            );
        }
        // Inside the cluster the two shapes are K-orthogonal (z is
        // orthonormal to about n·u), so they span the whole pair.
        let (a, b) = (solution.vector(2 * c), solution.vector(2 * c + 1));
        let cos = quad(&k, a, b).abs() / (quad(&k, a, a) * quad(&k, b, b)).sqrt();
        assert!(cos <= 64.0 * n as f64 * U, "cluster {c}: cos {cos:e}");
    }
}

#[test]
fn singular_lumped_mass_reports_infinite_eigenvalues() {
    // T₁₇ with mass on the odd DOFs only (8 masses, 9 massless). Condensing
    // the massless DOFs gives ½·T₈ with M = I, so λ_j = ½(2 − 2cos(jπ/9)),
    // and 9 eigenvalues are infinite.
    let n_mass = 8;
    let n = 2 * n_mass + 1;
    let k = second_difference(n);
    let m = diagonal(
        &(0..n)
            .map(|i| if i % 2 == 1 { 1.0 } else { 0.0 })
            .collect::<Vec<_>>(),
    );
    let solution = solve(&k, &m);
    // The 9 infinite eigenvalues are the condensed massless DOFs; no μ of
    // the condensed pencil is classified infinite.
    assert_eq!(solution.structurally_massless_count, n_mass + 1);
    assert_eq!(solution.condensed_dimension, n_mass);
    assert_eq!(
        solution.nonpositive_mu_count + solution.below_threshold_mu_count,
        0
    );
    assert!(!solution.finite_count_unresolved);
    eprintln!(
        "singular lumped mass: resolution-floor margin {:.2e}",
        floor_margin(&solution)
    );
    let exact_mu: Vec<f64> = (1..=n_mass)
        .map(|j| {
            1.0 / (2.0
                * (j as f64 * std::f64::consts::PI / (2 * (n_mass + 1)) as f64)
                    .sin()
                    .powi(2))
        })
        .collect();
    assert_mu(&solution, &exact_mu, &Tolerance::of(&k, &m));
    // The shapes are M-orthogonal (computed in double-double) to the
    // file-header bound for the worst pair, 64·n·u·μ_max/μ_min.
    let vectors: Vec<&[f64]> = (0..solution.mode_count())
        .map(|i| solution.vector(i))
        .collect();
    let ortho = m_orthogonality(&sym(&m), &vectors).unwrap();
    let bound = 64.0 * n as f64 * U * solution.mu_max / solution.mu[n_mass - 1];
    assert!(
        ortho.max_ratio <= bound,
        "{:e} > {bound:e}",
        ortho.max_ratio
    );
}

#[test]
fn stiffness_scale_series() {
    for exponent in 3..=6 {
        let s = 10f64.powi(exponent);
        // 2-DOF: K = [[1 + s, −s], [−s, s]], M = I: λ² − (1 + 2s)λ + s = 0.
        // (1 + 2s)² − 4s = 1 + 4s² is exact in binary64 for these s.
        let k = vec![vec![1.0 + s, -s], vec![-s, s]];
        let m = diagonal(&[1.0, 1.0]);
        let large = ((1.0 + 2.0 * s) + (1.0 + 4.0 * s * s).sqrt()) / 2.0;
        let small = s / large;
        assert_mu(
            &solve(&k, &m),
            &[1.0 / small, 1.0 / large],
            &Tolerance::of(&k, &m),
        );

        // 12-DOF chain with one element s times stiffer, against Jacobi.
        let mut ke = vec![1.0; 12];
        ke[6] = s;
        let k = spring_chain(&ke);
        let m = diagonal(
            &(0..12)
                .map(|i| 1.0 + 0.25 * (i % 4) as f64)
                .collect::<Vec<_>>(),
        );
        // A priori, g_R ≈ κ(K) grows with s (the factorization cancels
        // beside the stiff element), so the angle bound is about 2e-9·s.
        // The lowest three modes, the ones a modal set reports first, must
        // be compared.
        let compared = compare_with_jacobi(&k, &m, 1e-2);
        assert_eq!(&compared[..3], &[0, 1, 2], "s = {s:e}: {compared:?}");
    }
}

/// QL and Jacobi results each lie within TOL_μ(μ) of the exact μ and within
/// TOL_μ(μ_max)/gap in angle, so they agree within twice those.
///
/// Vectors are compared only where the a priori angle bound is at most
/// `max_vector_bound`, so far below 1 that a wrong mode (sin θ near 1) would
/// fail; elsewhere the bound says nothing. That is a precondition on the
/// pencil, not a tolerance. Returns the modes whose vectors were compared,
/// which the caller asserts.
fn compare_with_jacobi(k: &Rows, m: &Rows, max_vector_bound: f64) -> Vec<usize> {
    let tol = Tolerance::of(k, m);
    let solution = solve(k, m);
    let (ref_mu, ref_phi) = ref_modes(k, m);
    assert_eq!(solution.mode_count(), ref_mu.len());
    let mut compared = Vec::new();
    for i in 0..ref_mu.len() {
        let (got, want) = (solution.mu[i], ref_mu[i]);
        assert!(
            (got - want).abs() <= 2.0 * tol.mu(want),
            "mode {i}: μ {got:e} vs {want:e}"
        );
        let g = gap(&ref_mu, i);
        if 2.0 * tol.vector(g) > max_vector_bound {
            continue;
        }
        compared.push(i);
        let sin = k_sin_angle(k, solution.vector(i), &[&ref_phi[i]]);
        assert!(
            sin <= 2.0 * tol.vector(g),
            "mode {i}: sin θ {sin:e}, bound {:e}",
            2.0 * tol.vector(g)
        );
    }
    compared
}

#[test]
fn agrees_with_independent_jacobi() {
    for (n, seed, dense_mass) in [(30, 1, false), (30, 2, true), (60, 3, true)] {
        let (k, m) = random_pencil(n, seed, dense_mass);
        // Well-conditioned: every mode's vector is compared.
        let compared = compare_with_jacobi(&k, &m, 1e-6);
        assert_eq!(compared, (0..n).collect::<Vec<_>>(), "seed {seed}");
    }
}

#[test]
fn repeated_runs_are_bit_identical() {
    let (k, m) = random_pencil(40, 7, true);
    let (k, m) = (sym(&k), sym(&m));
    let first = solve_dense_pencil(&k, &m, &options()).unwrap();
    for _ in 0..3 {
        let again = solve_dense_pencil(&k, &m, &options()).unwrap();
        let bits = |v: &[f64]| v.iter().map(|x| x.to_bits()).collect::<Vec<_>>();
        assert_eq!(bits(&first.eigenvalues), bits(&again.eigenvalues));
        assert_eq!(bits(&first.mu), bits(&again.mu));
        assert_eq!(bits(&first.vectors), bits(&again.vectors));
        assert_eq!(first.ql_iterations, again.ql_iterations);
        assert_eq!(first.mu_max.to_bits(), again.mu_max.to_bits());
    }
}

#[test]
fn torsion_modes_take_unit_max_rotation() {
    // Four free nodes on the x axis at 1..4 m above a fixed node at 0 (so
    // ℓ = 4 m). Free DOFs per node: Ux (axial chain, k = 100), Uy (lateral
    // chain, k = 1) and Rx (torsion chain, k = 37); lumped M: 1 kg on Ux and
    // Uy, 0.1 kg·m² on Rx; the fixed node carries 0.5 kg. The three families
    // are decoupled, so the four torsion modes have no translation in exact
    // arithmetic.
    let nodes = 4;
    let n = 3 * nodes;
    let families = [
        (Dof::Ux, 100.0, 1.0),
        (Dof::Uy, 1.0, 1.0),
        (Dof::Rx, 37.0, 0.1),
    ];
    let mut k = vec![vec![0.0; n]; n];
    let mut m_diag = vec![0.0; n];
    let mut free_dofs = Vec::new();
    for node in 0..nodes {
        for (f, &(dof, _, mass)) in families.iter().enumerate() {
            free_dofs.push(FreeDof {
                node: node + 1,
                dof,
            });
            m_diag[3 * node + f] = mass;
        }
    }
    for (f, &(_, stiffness, _)) in families.iter().enumerate() {
        let chain = spring_chain(&vec![stiffness; nodes]);
        for i in 0..nodes {
            for j in 0..nodes {
                k[3 * i + f][3 * j + f] = chain[i][j];
            }
        }
    }
    let m = diagonal(&m_diag);
    let solution = solve(&k, &m);
    assert_eq!(solution.mode_count(), n);

    // Lumped mass: no coupling to the fixed node, so p_k = M_ff r_k. The
    // totals include the fixed node's 0.5 kg; Uz is restrained everywhere.
    let mut p = [vec![0.0; n], vec![0.0; n], vec![0.0; n]];
    for (i, free) in free_dofs.iter().enumerate() {
        if free.dof.is_translation() {
            p[free.dof.index()][i] = m_diag[i];
        }
    }
    let total = [4.5, 4.5, 4.5];
    let mass = sym(&m);
    // Test-local τ_t = 1e-6.
    let basis =
        ModalBasis::new(&mass, &free_dofs, [&p[0], &p[1], &p[2]], total, 4.0, 1e-6).unwrap();
    let modes: Vec<_> = (0..n)
        .map(|i| normalize_mode(i, solution.vector(i), &basis).unwrap())
        .collect();
    let torsion: Vec<_> = modes
        .iter()
        .filter(|mode| mode.case == NormalizationCase::UnitMaxRotation)
        .collect();
    assert_eq!(torsion.len(), nodes);
    for mode in &modes {
        assert_eq!(mode.shape[mode.reference_dof], 1.0);
        let reference = free_dofs[mode.reference_dof].dof;
        match mode.case {
            NormalizationCase::UnitMaxRotation => {
                assert_eq!(reference, Dof::Rx);
                // Not a condition of case (b): with lumped mass and decoupled
                // families, a torsion mode's translational participation is
                // rounding-level in this pencil, so 1e-6 is a loose bound.
                for k in 0..3 {
                    assert!(mode.effective_mass_fraction[k].unwrap() <= 1e-6);
                }
            }
            NormalizationCase::UnitMaxTranslation => assert!(reference.is_translation()),
        }
        for k in 0..3 {
            // Γ²m = m_eff within 4u (see modal_quantities' unit test), plus
            // an absolute floor for the near-zero cross-direction values.
            let g = mode.participation[k];
            let lhs = g * g * mode.generalized_mass;
            assert!(
                (lhs - mode.effective_mass[k]).abs()
                    <= 4.0 * U * mode.effective_mass[k] + f64::MIN_POSITIVE
            );
        }
    }
    // Torsion eigenvalues: 37/0.1 = 370 times the unit fixed-free chain's
    // 2 − 2cos((2j − 1)π/9).
    let chain = |j: usize| 2.0 - 2.0 * ((2 * j - 1) as f64 * std::f64::consts::PI / 9.0).cos();
    let mut torsion_lambdas: Vec<f64> = (0..n)
        .filter(|&i| modes[i].case == NormalizationCase::UnitMaxRotation)
        .map(|i| solution.eigenvalues[i])
        .collect();
    torsion_lambdas.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let tol = Tolerance::of(&k, &m);
    for (j, &lambda) in torsion_lambdas.iter().enumerate() {
        let exact = 370.0 * chain(j + 1);
        assert!(
            (1.0 / lambda - 1.0 / exact).abs() <= tol.mu(1.0 / exact),
            "torsion {j}: {lambda} vs {exact}"
        );
    }

    // Over every mode the x and y fractions sum to the free share 4/4.5;
    // the 0.5 kg on the fixed node moves with the support. The sum equals
    // that share for an exactly M-orthogonal set, and a Gram matrix I + E
    // changes it by at most ‖E‖₂ ≤ n·max|E_ij| of itself, plus rounding.
    let vectors: Vec<&[f64]> = modes.iter().map(|mode| mode.shape.as_slice()).collect();
    let ortho = m_orthogonality(&mass, &vectors).unwrap().max_ratio;
    let cumulative = cumulative_effective_mass_fractions(&modes);
    let last = cumulative.last().unwrap();
    for k in 0..2 {
        let share = 4.0 / 4.5;
        let sum = last[k].unwrap();
        assert!(
            (sum - share).abs() <= n as f64 * ortho * share + 8.0 * n as f64 * U,
            "k={k}: {sum}"
        );
    }
    assert_eq!(last[2], Some(0.0));
}
