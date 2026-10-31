# Modal Eigen Core

This crate is the dense eigen core of F1 modal analysis, DEL-04-09 (`execution/PKG-04/DEL-04-09/`). It is governed by `Work/modal-analysis.md`, and this README names the design sections each part implements. It uses the Rust standard library and one in-repo path dependency, `sparse_direct` (DEL-04-01; it brings `frame_kernel` and `correct_norm` with it), for the LDLᵀ behind the Sturm counts. No product crate depends on it yet.

## What it does

- **`dd`**: double-double arithmetic (about a 106-bit significand) built from the binary64 error-free transformations TwoSum and TwoProduct (`f64::mul_add`). It provides add, sub, mul, div, sqrt, conversion to and from `f64` (rounding once), and a dot-product accumulator. Design §3.4 check 1 (ii) names this as the precision floor.
- **`dense`**: the dense path of design §3.2, items 2 to 6, for K φ = λ M φ, with K symmetric positive definite and M symmetric positive semidefinite, possibly singular:
  - Cholesky K = R Rᵀ. A non-positive pivot is refused as `NonPositivePivot { index }`.
  - C = R⁻¹ M R⁻ᵀ by two triangular solves, symmetrized from the lower triangle.
  - Householder tridiagonalization with accumulated transformations, then implicit QL with Wilkinson shifts (the tql2 procedure; Golub & Van Loan ch. 8, written in-house). The iteration limit is a typed error.
  - λ = 1/μ and φ = R⁻ᵀ z for the finite modes, in ascending λ. The μ ≤ 0 and the μ at or below a stated relative threshold (infinite eigenvalues from a singular M) are counted, not returned.
  - The finite-mode count of §3.2 item 5. `established_finite_mode_count(M, τ)` is n minus the structurally massless DOFs (rows and columns of M exactly zero), when the remaining block of M factors by Cholesky with every pivot p_j > τ·m_jj; otherwise `None`. The solution flags `finite_count_unresolved` (the `MODAL_FINITE_COUNT_UNRESOLVED` case) when the μ at or below the infinite threshold, μ ≤ 0 included, are not matched one for one by structurally massless DOFs, or a μ is negative beyond the threshold's magnitude.
  - Memory: `DENSE_PATH_PEAK_N2_BUFFERS` = 4 live n² buffers at the peak (K, M, R and one work buffer), so `DENSE_PATH_BYTES_PER_ENTRY` = 32. The work estimate is `dense_path_flop_estimate(n, s)` = 9n³ + ⌈s·n³/3⌉. Both are inputs to the guards of design §3.2 (D-k). They apply no budget.
- **`clusters`**: the one cluster definition of design §3.4, check 5: adjacent ascending λ chain into a cluster when λ_{i+1} − λ_i ≤ δ_c·λ_{i+1}. It also gives the reported set of §1.4: the lowest n, all modes with f ≤ f_c (inclusive), or the smaller of the two; completed to whole clusters; with the stopping bound and the extension count.
- **`modal_quantities`**: the two shape normalizations of §1.5 and the participation quantities.
  - Case (a): unit maximum translation. Case (b): unit maximum rotation, exactly when t ≤ τ_t·ℓ·θ, with no condition on participation. Ties go to the lowest global DOF index.
  - It computes the generalized mass m = φᵀM_ffφ, Γ_k = φᵀp_k/m, m_eff,k = (φᵀp_k)²/m, the fractions m_eff,k/T_k and their cumulative sums. Each is computed in double-double and rounded once.
  - The caller supplies the excitation p_k = (M R_k)_f and the total mass T_k = R_kᵀ M R_k (D-v). The module does not form them.
- **`orthogonality`**: the M-orthogonality measure of §3.4, check 3: max over i ≠ j of |φᵢᵀMφⱼ|/√(m_i m_j), in double-double.
- **`sturm`**: the Sturm counts of §3.2 item 7, §3.3 and §3.4 check 4, on `sparse_direct`'s `factorize_ldlt` and its read-only observation accessor (`lower_factor_entries`, `pivots`):
  - `ShiftPencil` holds K and M over their union pattern, with D = diag(k_ii^-½) and the reverse Cuthill–McKee order of the pattern.
  - Formation: a = fl(k_ij − fl(σ·m_ij)), then fl(fl(d_i·a)·d_j). An operation with nonzero operands whose result is zero, subnormal or infinite puts the shift out of range.
  - e_b = γ_n·‖|L̂||D̂||L̂ᵀ|‖_∞ + c_f·u·‖|K̃| + |σ||M̃|‖_∞ with c_f = 5, the first term in two profile traversals with O(n) workspace; ν, the Hager–Higham estimate of ‖Ã⁻¹‖₁; the growth g; the separation s_j(σ) of each neighbouring eigenvalue.
  - Premises: separation s_j ≥ κ_s·e_b, growth g ≤ g_max, conditioning e_b·ν ≤ 1/κ_c, and the pivot guard. When the factorization is rejected, separation is still judged against the formation term alone (a lower bound on e_b).
  - `count_at_shift` (one given shift), `count_in_gap` (√λ_lo·√λ_hi first, then the caller's dyadic log-positions, at most s_max candidates, never outside the gap) and `count_above_spectrum` (relative offsets above the largest eigenvalue, for a caller with an established finite-mode count). Each returns `Counted { count, evidence }` or `Refused { reason, evidence }`, the reason `Unseparated`, `IllConditioned`, `GrowthLimit` or `OutOfRange`, with every candidate's σ, the two parts of e_b, ν, the product, 1/κ_c, the separation ratio, g and the decision.
  - The parameters (κ_c, κ_s, g_max, s_max, the gap positions and the whole-spectrum offsets) have no default.
  - Memory per count: 32 B per union-pattern position for the pencil, a transient 24 B per position for the formed entries, and the profile and its clone inside the factor; about 32·n² B at the peak on the dense path (module docs).

## Determinism

The crate is single-threaded and runs every loop in a fixed order. It uses only correctly rounded IEEE binary64 operations (`+ − × ÷`, `sqrt`, `mul_add`); `hypot` is formed from `sqrt` rather than taken from the platform library. A repeated run is bit-identical, which a test asserts.

## What it does not do yet

- No double-double re-formation of K or M, no formation difference, and no residual or error estimate on the re-formed pencil (§3.4, checks 1 and 2).
- No sparse path (§3.3, M2), no mass assembly, no weight-basis check (§2), and no product entry or result rows (§3.5, §5).
- No production values for the algorithm parameters: δ_c, τ_t, the infinite-μ threshold, the QL iteration limit, the mass-pivot threshold of the finite-mode count and the Sturm parameters are caller inputs. The values in the tests are test-local; the production values are recorded under DEC-026 (design §9, O-6).
- No check-2 error estimate e_λ: `sturm`'s callers supply it for each neighbouring eigenvalue. The tests use a stand-in on the formed pencil.

## Tests

`cargo test` runs the unit tests in each module, `tests/dense_reference.rs` and `tests/sturm_counts.rs`:

- The dd tests check against exact integer and power-of-two expectations.
- The dense pencils have closed forms: K = I with diagonal M (exact), 2-DOF pencils, the second-difference matrix with M = I, 4I and a non-uniform diagonal, exactly repeated pairs (compared as subspaces), a singular lumped M, and a stiffness scale series from 1e3 to 1e6.
- An independent cyclic Jacobi solver on C, sharing no code with the QL path, is compared for eigenvalues and for vectors (by K-inner-product angle).
- Repeated runs are checked for bit identity.
- The normalization tests cover case (a) with ties, case (b) on a torsion pencil, case (b) with large participation through support coupling (a simply supported element), the fixed axial element whose p = m/2 differs from M_ff·r, and Γ²m = m_eff.

The file header of `tests/dense_reference.rs` derives the tolerance model, and each test states its reason.

### Sturm counts

`tests/sturm_counts.rs` is the inertia-risk suite of design §3.3. Its pencils are in `tests/fixtures/sturm_pencils.txt`, which `tests/fixtures/sturm_pencils.py` writes from two frozen references, read-only, and checks byte for byte (`python3 -I -B core/solver/modal_eigen/tests/fixtures/sturm_pencils.py`): the inertia witnesses of `validation/references/f1_modal/continuum/inertia_witnesses.json`, and nine class (a) cases of `validation/references/f1_modal/mass_discrete/`, whose K_ff and M_ff the references' own generator forms and the script rounds once to binary64, with their published exact Sturm counts.

- Every count taken, at every witness shift and every published shift, equals the exact count. Binary64 miscounts are met and refused.
- Every shift of the well-separated kinds (below the lowest mode, gap geometric means, above all finite modes), and every placement in a gap between computed clusters, is counted: no refusal.
- The design's review witnesses: round 2 (refused `Unseparated`), round 3 (refused; `IllConditioned` in the review's elimination order, where the count is the wrong 40), round 4 (refused), round 6 underflow (`OutOfRange`) and round 6 finite count (unresolved on both halves). Also the missed eigenvalue, the zero pivot, a finite count established with structural zeros, and bit-identical repeated runs.

Test-local parameters: κ_c = 10³, κ_s = 10², g_max = 10¹⁵, s_max = 7, gap positions (1/4, 3/4, 1/8, 3/8, 5/8, 7/8), whole-spectrum offsets (1, 1/2, 1/4), infinite-μ threshold 10⁻¹², mass-pivot threshold 10⁻¹², δ_c = 10⁻⁸. They were set by measurement with κ_c = κ_s = 1 and no growth limit:

| Population | e_b·ν | separation ratio | g |
|---|---|---|---|
| 34 factored shifts whose negative-pivot count was wrong | ≥ 35.8 | ≤ 6.2·10⁻³ | ≤ 7.7·10⁸ |
| 352 well-separated shifts and placements, every count right | 1.8·10⁻¹⁵ to 2.13·10⁻⁴ | 5.65·10³ to 5.6·10¹⁴ | up to 3.98·10¹³ |

The conditioning and separation premises each separate the two populations by orders of magnitude. The growth factor does not: correct counts on well-separated rigid-scale gaps (E-RIGID-SCALE-1e6) reach g ≈ 4·10¹³, above every miscount's g, so g_max is set above them and the growth effect is carried by e_b's factor term.
