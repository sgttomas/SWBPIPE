# Modal Eigen Core

This crate is the dense eigen core of F1 modal analysis, DEL-04-09 (`execution/PKG-04/DEL-04-09/`). It is governed by `Work/modal-analysis.md`, and this README names the design sections each part implements. It is std-only, has no dependencies, and no product crate depends on it yet.

## What it does

- **`dd`**: double-double arithmetic (about a 106-bit significand) built from the binary64 error-free transformations TwoSum and TwoProduct (`f64::mul_add`). It provides add, sub, mul, div, sqrt, conversion to and from `f64` (rounding once), and a dot-product accumulator. Design §3.4 check 1 (ii) names this as the precision floor.
- **`dense`**: the dense path of design §3.2, items 2 to 6, for K φ = λ M φ, with K symmetric positive definite and M symmetric positive semidefinite, possibly singular:
  - Cholesky K = R Rᵀ. A non-positive pivot is refused as `NonPositivePivot { index }`.
  - C = R⁻¹ M R⁻ᵀ by two triangular solves, symmetrized from the lower triangle.
  - Householder tridiagonalization with accumulated transformations, then implicit QL with Wilkinson shifts (the tql2 procedure; Golub & Van Loan ch. 8, written in-house). The iteration limit is a typed error.
  - λ = 1/μ and φ = R⁻ᵀ z for the finite modes, in ascending λ. The μ ≤ 0 and the μ at or below a stated relative threshold (infinite eigenvalues from a singular M) are counted, not returned.
  - Memory: `DENSE_PATH_PEAK_N2_BUFFERS` = 4 live n² buffers at the peak (K, M, R and one work buffer), so `DENSE_PATH_BYTES_PER_ENTRY` = 32. The work estimate is `dense_path_flop_estimate(n, s)` = 9n³ + ⌈s·n³/3⌉. Both are inputs to the guards of design §3.2 (D-k). They apply no budget.
- **`clusters`**: the one cluster definition of design §3.4, check 5: adjacent ascending λ chain into a cluster when λ_{i+1} − λ_i ≤ δ_c·λ_{i+1}. It also gives the reported set of §1.4: the lowest n, all modes with f ≤ f_c (inclusive), or the smaller of the two; completed to whole clusters; with the stopping bound and the extension count.
- **`modal_quantities`**: the two shape normalizations of §1.5 and the participation quantities.
  - Case (a): unit maximum translation. Case (b): unit maximum rotation, exactly when t ≤ τ_t·ℓ·θ, with no condition on participation. Ties go to the lowest global DOF index.
  - It computes the generalized mass m = φᵀM_ffφ, Γ_k = φᵀp_k/m, m_eff,k = (φᵀp_k)²/m, the fractions m_eff,k/T_k and their cumulative sums. Each is computed in double-double and rounded once.
  - The caller supplies the excitation p_k = (M R_k)_f and the total mass T_k = R_kᵀ M R_k (D-v). The module does not form them.
- **`orthogonality`**: the M-orthogonality measure of §3.4, check 3: max over i ≠ j of |φᵢᵀMφⱼ|/√(m_i m_j), in double-double.

## Determinism

The crate is single-threaded and runs every loop in a fixed order. It uses only correctly rounded IEEE binary64 operations (`+ − × ÷`, `sqrt`, `mul_add`); `hypot` is formed from `sqrt` rather than taken from the platform library. A repeated run is bit-identical, which a test asserts.

## What it does not do yet

- No Sturm counts, shift placement or growth monitor (§3.2 item 7, §3.4 check 4). They will use `sparse_direct`'s `factorize_ldlt` and its observation accessor, neither of which this crate uses yet.
- No double-double re-formation of K or M, no formation difference, and no residual or error estimate on the re-formed pencil (§3.4, checks 1 and 2).
- No sparse path (§3.3, M2), no mass assembly, no weight-basis check (§2), and no product entry or result rows (§3.5, §5).
- No production values for the algorithm parameters: δ_c, τ_t, the infinite-μ threshold and the QL iteration limit are caller inputs. The values in the tests are test-local.

## Tests

`cargo test` runs the unit tests in each module and `tests/dense_reference.rs`:

- The dd tests check against exact integer and power-of-two expectations.
- The dense pencils have closed forms: K = I with diagonal M (exact), 2-DOF pencils, the second-difference matrix with M = I, 4I and a non-uniform diagonal, exactly repeated pairs (compared as subspaces), a singular lumped M, and a stiffness scale series from 1e3 to 1e6.
- An independent cyclic Jacobi solver on C, sharing no code with the QL path, is compared for eigenvalues and for vectors (by K-inner-product angle).
- Repeated runs are checked for bit identity.
- The normalization tests cover case (a) with ties, case (b) on a torsion pencil, case (b) with large participation through support coupling (a simply supported element), the fixed axial element whose p = m/2 differs from M_ff·r, and Γ²m = m_eff.

The file header of `tests/dense_reference.rs` derives the tolerance model, and each test states its reason.
