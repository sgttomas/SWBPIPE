# Modal Eigen Core

This crate is the dense eigen core of F1 modal analysis, DEL-04-09 (`execution/PKG-04/DEL-04-09/`), whose scope governs it. The design section numbers below are the labels the code and its tests use for each part. It uses the Rust standard library and two in-repo path dependencies of DEL-04-01: `sparse_direct`, for the LDLᵀ behind the Sturm counts, and `frame_kernel`, for its p128 element re-formation seam (seam (i)); `correct_norm` comes with them. No product crate depends on it yet.

## What it does

- **`admission`**: the admission range of design §3.2 item 5. A pencil is admitted only when every nonzero formed (and re-formed) K and M entry has magnitude in [2⁻³⁰⁰, 2³⁰⁰]; any other is refused at entry by name (`MODAL_PENCIL_RANGE_UNSUPPORTED`, a typed `PencilRangeUnsupported` in each module's error) in `dense`, `sturm`, `orthogonality`, `modal_quantities`, `ReformedPencil` and `formation_difference`. A published value that overflows or is NaN refuses; one that is nonzero but below the binary64 normal range is published as 0 with a flag (`NormalizedMode::below_normal_range` for the shape and participation quantities, the residual's ‖Dr‖ and φᵀM̂φ, the M-orthogonality ratio). Compared values (η, κ_λ, e_λ, the formation-difference norms, the dense results) must be normal or an exact zero, and no reduction passes over a NaN.
- **`dd`**: double-double arithmetic (about a 106-bit significand) built from the binary64 error-free transformations TwoSum and TwoProduct (`f64::mul_add`). It provides add, sub, mul, div, sqrt, conversion to and from `f64` (rounding once), and a dot-product accumulator. Design §3.4 check 1 (ii) names this as the precision floor. `WideDd` adds a separate exponent: products, quotients, sums and roots keep about 106 bits whatever their magnitude, with bounded power-of-two rescaling, non-finite operands propagated before any alignment, and a final rounding to binary64 done once to the destination spacing, subnormals included (checked against exact integer arithmetic on 200 000 pseudo-random products).
- **`exact`**: `ExactSum`, a superaccumulator (signed 32-bit limbs with a movable integer exponent) that sums binary64 values times powers of two exactly. A product of up to three binary64 factors enters as its exact TwoProduct expansion of the factors' significands. The sum is rounded once to a `WideDd` whose high word is correctly rounded and whose low word is the remainder rounded to odd, so later rounding and classification see the exact value's side of every midpoint. Every bilinear form (participation, generalized masses, residual rows, orthogonality, formation differences, re-formed assembly, Sturm mass quotients) is accumulated with it, so no contribution lost to alignment or rounding can be exposed by a cancellation (design §3.2, review round 4).
  - Storage (design §5.3.8, Agent 0's option (c)): a sum's limbs grow only by requests of max(2·capacity, required, 4) limbs (`Vec::reserve_exact`), so a limb vector's requested capacity is at most max(4, 2L − 2) for L the longest it has held. `reset` returns every field to the fresh state and keeps the capacity. Reads normalize into a caller's scratch vector, not a clone. `try_add_scaled` and `try_add_f64` take a limb allotment and refuse (`LimbAllotmentExceeded`) before any growth past it; no allotment value is set here.
- **`workspace`** (crate-private): the per-stage workspaces of option (c). `ReformedPencil` owns the residual and orthogonality stages' workspace (2n row sums, one sum, a limb scratch and a term list), and `ModalBasis` the normalization stage's (one sum, a limb scratch, a below-normal index list). Each is held behind a `Mutex`, reused for every pair, product or mode and retained until its owner is dropped, and it is not part of the owner's value (a clone starts empty, and equality and `Debug` ignore it). Every accumulator is reset before each use, so the outputs are bit-identical to fresh accumulators.
- **`norm_estimate`**: the Hager–Higham estimate of ‖A⁻¹‖₁ from solves, shared by the Sturm counts and the dense resolution floor; a non-finite solve or norm gives +∞.
- **`dense`**: the dense path of design §3.2, items 2 to 6, for K φ = λ M φ, with K symmetric positive definite and M symmetric positive semidefinite, possibly singular:
  - Condensation of the massless subspace (§3.2 item 5): the structurally massless DOFs (rows and columns of M exactly zero) are ordered first in the Cholesky factorization K = R Rᵀ, so its trailing block is the factor of K̂ = K_mm − K_m0·K₀₀⁻¹·K_0m. A non-positive pivot is refused as `NonPositivePivot { index }`, in the caller's numbering.
  - C = R_mm⁻¹ M_mm R_mm⁻ᵀ by two triangular solves, symmetrized from the lower triangle.
  - Householder tridiagonalization with accumulated transformations, then implicit QL with Wilkinson shifts (the tql2 procedure; Golub & Van Loan ch. 8, written in-house). The iteration limit is a typed error.
  - λ = 1/μ and φ = R⁻ᵀ·[0; z] for the finite modes, in ascending λ, which recovers φ₀ = −K₀₀⁻¹·K_0m·φ_m on the massless DOFs. Condensed μ ≤ 0 or at or below a stated relative threshold are classified infinite, counted and not returned.
  - The finite-mode count of §3.2 item 5. `established_finite_mode_count(M, τ)` is n minus the structurally massless DOFs (rows and columns of M exactly zero), when the remaining block of M, equilibrated as D_M·M·D_M with D_M = diag(m_jj)^−½ (unit diagonal), factors by Cholesky with every pivot at least τ (inclusive); otherwise `None`. `solve_dense_pencil` evaluates it with the option τ_m and flags `finite_count_unresolved` (the `MODAL_FINITE_COUNT_UNRESOLVED` case) when any condensed μ is classified infinite (μ ≤ τ_μ·μ_max, of either sign) or the count is not established: the condensed pencil has exactly n_m finite eigenvalues in exact arithmetic when that remaining mass block is positive definite.
  - The stiffness factor, the reduced matrix and every QL eigenvalue are checked finite before classification; a non-finite one refuses (`ReductionOutOfRange`, `MODAL_PENCIL_RANGE_UNSUPPORTED`), so no NaN is counted as μ ≤ 0.
  - Hard classification (§3.2 item 5): μ ≤ max(τ_μ·μ_max, κ_μ·δ_q), with δ_q = c_μ·n·u·μ_max, is never rescued. The broader δ_μ = c_μ·n·u·ν(K̃̂)·‖M̃_mm‖_∞ screens only the remaining band. There, exact positive φᵀMφ and φᵀKφ yield μ_R, λ_R = 1/μ_R; the equilibrated formed η̃ and relative-eigenvalue estimate e_λ must pass explicit thresholds. Recovered μ must remain above both hard floors. The returned pairs are re-sorted in place; low modes outside the band keep their original QL values. All floors and per-candidate decisions/recovery observations are exposed. Invalid signs or ranges reject recovery, and any rejected condensed μ keeps the count unresolved.
  - Memory: `DENSE_PATH_PEAK_N2_BUFFERS` = 4 live n² buffers at the peak (K, M, R and one work buffer of n_m·n entries; the condensation forms K̂ inside R and recovers the shapes in place, adding no buffer), so `DENSE_PATH_BYTES_PER_ENTRY` = 32. The work estimate is `dense_path_flop_estimate(n, s)` = 9n³ + ⌈s·n³/3⌉. Band recovery also has `dense_screen_product_estimate(n, p)` = 4pn² exact-product insertions, plus 2n² scaling visits if a band exists, O(n) scratch and no additional n² buffer. Exact products are not binary64 flops; M1b must price their runtime cost in its work guard. These counts apply no budget or unmeasured coefficient.
- **`clusters`**: the one cluster definition of design §3.4, check 5: adjacent ascending λ chain into a cluster when λ_{i+1} − λ_i ≤ δ_c·λ_{i+1}. It also gives the reported set of §1.4: the lowest n, all modes with f ≤ f_c (inclusive), or the smaller of the two; completed to whole clusters; with the stopping bound and the extension count.
- **`modal_quantities`**: the two shape normalizations of §1.5 and the participation quantities.
  - Case (a): unit maximum translation. Case (b): unit maximum rotation, exactly when t ≤ τ_t·ℓ·θ, with no condition on participation. Ties go to the lowest global DOF index.
  - It computes the generalized mass m = φᵀM_ffφ, Γ_k = φᵀp_k/m, m_eff,k = (φᵀp_k)²/m, the fractions m_eff,k/T_k and their cumulative sums. Each is computed in double-double with an extended exponent (`dd::WideDd`), so no product, sum or quotient underflows or overflows before the single rounding to binary64; a quantity that is not finite is refused (`QuantityOutOfRange`), the generalized mass checked for finiteness before positivity.
  - The caller supplies the excitation p_k = (M R_k)_f and the total mass T_k = R_kᵀ M R_k (D-v). The module does not form them.
- **`orthogonality`**: the M-orthogonality measure of §3.4, check 3: max over i ≠ j of |φᵢᵀMφⱼ|/√(m_i m_j), in double-double, with the denominator √m_i·√m_j; a ratio that is not finite is refused (`RatioOutOfRange`).
- **`reformation`**: check 1's re-formation and formation difference and checks 2 and 3 on the re-formed pencil (design §3.4):
  - `section`: A, I, J = 2I, m̄ by category, j̄ and ī = j̄/2 (contents excluded) re-derived in double-double from OD, wall, the mill deduction and, on EX, the nominal wall and the pressure bore t_r = (t − c_m) − c_a, on `source_od_effective_wall_areas/v1` (PF) and `source_od_nominal_wall_pressure_bore_areas/v1` (EX), with the double-double π; and the section-derivation difference against the binary64 A, I and J a formation used.
  - `mass`: the consistent straight mass with the bending-plane sign map [u_y, θ_z] / [u_z, −θ_y] and Rayleigh rotary inertia, turned with the member frame; the two half-arc lumps with centreline and section rotary inertia, α = 2·atan(s/(1 + c)) in double-double and the cancelling entries (sin α − α, α/2 − sin 2α/4, J_c,22 ≈ α⁵/20, 2(α − sin α)) by their series on the whole admitted range 0 < α < π/2 (the switch point; module docs), summed scaled (f₁/α³, f₃/α⁵) so a small α cannot underflow them, with an arc whose local entries fall below 2⁻⁹⁶⁹ refused (`ArcAngleRange`, α below about 10⁻⁵⁸); the component body m = W/a after the four-part binary64 range premise, refused by operand, its rotational block formed from the mass-weighted products m·r_a·r_b the premise admits, and a non-finite re-formed entry refused.
  - `ElementOperand`, the neutral operand type: straight, arc, component body, axis or rank-one spring, the frame kernel's `ObjectiveConnector` and `TieRod`, with moduli and the rigid-family scale s (s·E and s·G formed exactly). Straights, bends, connectors and tie rods are re-formed through `frame_kernel::structural::element_reformation` from the re-derived A, I and J; springs here.
  - `ReformedPencil` (element by element on the free numbering), `formation_difference` (one double-double accumulator per stored entry of the formed K_ff and M_ff, ‖DΔKD‖_∞, ‖DΔMD‖_∞, off-pattern positions, ‖K̃̂‖_∞, ‖M̃̂‖_∞ and the section-derivation difference), `residual_estimate` (η, κ_λ, e_λ = η·κ_λ) and `cluster_error_estimates`. `orthogonality::m_orthogonality_reformed` evaluates check 3 on M̂.
- **`sturm`**: the Sturm counts of §3.2 item 7, §3.3 and §3.4 check 4, on `sparse_direct`'s `factorize_ldlt` and its read-only observation accessor (`lower_factor_entries`, `pivots`):
  - `ShiftPencil` holds K and M over their union pattern, with D = diag(k_ii^-½) and the reverse Cuthill–McKee order of the pattern.
  - Formation: a = fl(k_ij − fl(σ·m_ij)), then fl(fl(d_i·a)·d_j). An operation with nonzero operands whose result is zero, subnormal or infinite puts the shift out of range.
  - e_b = γ_n·‖|L̂||D̂||L̂ᵀ|‖_∞ + c_f·u·‖|K̃| + |σ||M̃|‖_∞ with c_f = 5, the first term in two profile traversals with O(n) workspace; ν, the Hager–Higham estimate of ‖Ã⁻¹‖₁; the growth g; the separation s_j(σ) of each neighbouring eigenvalue, its mass quotient's weighted terms and sums formed in `WideDd`. A quotient that is not positive and normal is refused (`NeighbourMassQuotient`), and a separation that is not finite refuses the candidate (`SeparationOutOfRange`, set-level `OutOfRange`).
  - Premises: separation s_j ≥ κ_s·e_b, growth g ≤ g_max, conditioning e_b·ν ≤ 1/κ_c, and the pivot guard. When the factorization is rejected, separation is still judged against the formation term alone (a lower bound on e_b).
  - `count_at_shift` (one given shift), `count_in_gap` (√λ_lo·√λ_hi first, then the caller's dyadic log-positions, at most s_max candidates, never outside the gap) and `count_above_spectrum` (relative offsets above the largest eigenvalue, for a caller with an established finite-mode count). Each returns `Counted { count, evidence }` or `Refused { reason, evidence }`, the reason `Unseparated`, `IllConditioned`, `GrowthLimit` or `OutOfRange`, with every candidate's σ, the two parts of e_b, ν, the product, 1/κ_c, the separation ratio, g and the decision.
  - The parameters (κ_c, κ_s, g_max, s_max, the gap positions and the whole-spectrum offsets) have no default.
  - Memory per count: 32 B per union-pattern position for the pencil, a transient 24 B per position for the formed entries, and the profile and its clone inside the factor; about 32·n² B at the peak on the dense path (module docs).

## Determinism

The crate is single-threaded and runs every loop in a fixed order. It uses only correctly rounded IEEE binary64 operations (`+ − × ÷`, `sqrt`, `mul_add`); `hypot` is formed from `sqrt` rather than taken from the platform library. A repeated run is bit-identical, which a test asserts.

## What it does not do yet

- No product mapping into `ElementOperand` (I2), no Timoshenko straight (M4) and no directional flexibility factors beyond the two the bend seam takes.
- No sparse path (§3.3, M2), no mass assembly, no weight-basis check (§2), and no product entry or result rows (§3.5, §5).
- No production values for the algorithm parameters: δ_c, τ_t, the infinite-μ threshold, the QL iteration limit, the mass-pivot threshold of the finite-mode count, the resolution floor's c_μ and κ_μ and the Sturm parameters are caller inputs. The values in the tests are test-local; the production values are recorded under DEC-026 (design §9, O-6).
- No residual threshold τ_r and no orthogonality tolerance: the tests' bounds are test-local (DEC-026). `sturm`'s own tests still use a stand-in e_λ on the formed pencil.

## Tests

`cargo test` runs the unit tests in each module, `tests/dense_reference.rs`, `tests/sturm_counts.rs`, `tests/reformation.rs` and `tests/workspace_reuse.rs`:

- The dd tests check against exact integer and power-of-two expectations.
- The dense pencils have closed forms: K = I with diagonal M (exact), 2-DOF pencils, the second-difference matrix with M = I, 4I and a non-uniform diagonal, exactly repeated pairs (compared as subspaces), a singular lumped M, and a stiffness scale series from 1e3 to 1e6.
- An independent cyclic Jacobi solver on C, sharing no code with the QL path, is compared for eigenvalues and for vectors (by K-inner-product angle).
- Repeated runs are checked for bit identity.
- The normalization tests cover case (a) with ties, case (b) on a torsion pencil, case (b) with large participation through support coupling (a simply supported element), the fixed axial element whose p = m/2 differs from M_ff·r, and Γ²m = m_eff.

The file header of `tests/dense_reference.rs` derives the tolerance model, and each test states its reason.

`tests/workspace_reuse.rs` checks that the workspace reuse changed no output. It writes the residual, orthogonality, normalization, formation and `ExactSum` results of the eleven reference eigen pencils as text, in sequences that reuse a workspace (every mode in turn, refusals followed by valid calls, wide-span vectors before and after narrow ones, empty, zero and cancelling inputs). It compares each section's line count and FNV-1a hash with `tests/fixtures/workspace_reuse_base.txt`, captured by the same file at the revision before the reuse (its header states how).

### Sturm counts

`tests/sturm_counts.rs` is the inertia-risk suite of design §3.3. Its pencils are in `tests/fixtures/sturm_pencils.txt`, which `tests/fixtures/sturm_pencils.py` writes from two frozen references, read-only, and checks byte for byte (`python3 -I -B core/solver/modal_eigen/tests/fixtures/sturm_pencils.py`): the inertia witnesses of `validation/references/f1_modal/continuum/inertia_witnesses.json`, and nine class (a) cases of `validation/references/f1_modal/mass_discrete/`, whose K_ff and M_ff the references' own generator forms and the script rounds once to binary64, with their published exact Sturm counts.

- Every count taken, at every witness shift and every published shift, equals the exact count. Binary64 miscounts are met and refused.
- Every shift of the well-separated kinds (below the lowest mode, gap geometric means, above all finite modes), and every placement in a gap between computed clusters, is counted: no refusal.
- The design's review witnesses: round 2 (refused `Unseparated`), round 3 (refused; `IllConditioned` in the review's elimination order, where the count is the wrong 40), round 4 (refused), round 6 underflow (`OutOfRange`) and round 6 finite count (unresolved on both halves). Also the missed eigenvalue, the zero pivot, a finite count established with structural zeros, and bit-identical repeated runs. The independent review of PR #33 adds: a neighbour vector rescaled to 10⁻²⁰⁰ (still `Unseparated`), quotient and separation range refusals, a negative μ of a structurally massless DOF (resolved, matched by count), a finite μ classified infinite beside a structural zero (unresolved: the counts differ), and equilibrated pivots meeting τ_m inclusively. Review round 2 adds: a structural zero condensed out (resolved, its shape recovered), a finite mode traded for a structural zero (unresolved at τ_μ = 10⁻¹⁸, returned at τ_μ = 0), a finite μ classified infinite beside a structural zero (unresolved), and a weighted square below the binary64 range (2⁻¹²⁰⁰ against 2¹⁰⁰⁰, `Unseparated`).

Test-local parameters: κ_c = 10³, κ_s = 10², g_max = 10¹⁵, s_max = 7, gap positions (1/4, 3/4, 1/8, 3/8, 5/8, 7/8), whole-spectrum offsets (1, 1/2, 1/4), infinite-μ threshold 10⁻¹², mass-pivot threshold 10⁻¹², δ_c = 10⁻⁸. They were set by measurement with κ_c = κ_s = 1 and no growth limit:

| Population | e_b·ν | separation ratio | g |
|---|---|---|---|
| 34 factored shifts whose negative-pivot count was wrong | ≥ 35.8 | ≤ 6.2·10⁻³ | ≤ 7.7·10⁸ |
| 352 well-separated shifts and placements, every count right | 1.8·10⁻¹⁵ to 2.13·10⁻⁴ | 5.65·10³ to 5.6·10¹⁴ | up to 3.98·10¹³ |

The conditioning and separation premises each separate the two populations by orders of magnitude. The growth factor does not: correct counts on well-separated rigid-scale gaps (E-RIGID-SCALE-1e6) reach g ≈ 4·10¹³, above every miscount's g, so g_max is set above them and the growth effect is carried by e_b's factor term.

### Re-formation

`tests/reformation.rs` compares the re-formation with the frozen class (a) references. Its operands and expectations are in `tests/fixtures/reformation_cases.txt`, which `tests/fixtures/reformation_cases.py` writes from `mass_references.json` and `eigen_references.json`, read-only, and checks byte for byte (`python3 -I -B core/solver/modal_eigen/tests/fixtures/reformation_cases.py`). Expectations are double-double pairs within 2⁻¹⁰⁶ of the exact or 80-digit values; the eigen cases' K_ff and M_ff come from the references' generator, and their hi words are the formed binary64 pencil.

- Every comparison is normwise, |x̂ − x| ≤ 10⁻²⁹·max|x| (the file header derives it: about 60 double-double operations per entry at a few units of 2⁻¹⁰⁶). Measured largest differences: section quantities 1.3·10⁻³² relative (638 values, both bases); straight masses 3.0·10⁻³², arc lumps 4.9·10⁻³², M-ARC-SMALL 5.2·10⁻³³ normwise and 9.6·10⁻³² entrywise, component bodies 1.3·10⁻³³, assembled full M 4.9·10⁻³², K̂ and M̂ of eleven eigen cases at most 7.6·10⁻³³ and 2.4·10⁻³².
- The formed pencils' formation differences are rounding, ‖DΔKD‖ ≤ 3.1·10⁻¹⁶. Over every dense mode of the eleven eigen cases η ≤ 1.44·10⁻¹³, asserted below 10⁻¹² (under every τ_r the design allows); the published eigenvalues' actual error is at most 0.85·e_λ.
- Mutations on E-CANT-4: K or M at the tip's u_y diagonal times 1 + 10⁻⁸ gives a relative formation difference 3.1·10⁻⁹ (K) or 6.3·10⁻⁹ (M) and η 2.0·10⁻⁹ or 1.1·10⁻⁹, against 3.3·10⁻¹⁴ unperturbed. Review round 4: a formed K with doubled A gives η 1.0·10⁻², ‖DΔKD‖ 1.1 and a section-derivation difference of 1 in A.
- The range-premise witnesses are refused by operand; repeated runs are bit-identical.
- Check 2 against closed forms: η, κ_λ and e_λ on a 2×2 pencil with hand-perturbed vectors agree within 4u; cluster estimates are the largest over their members; the dense modes are M̂-orthogonal within an empirical fixture threshold, 10⁻⁷ (measured largest 1.3·10⁻⁸). It is not a derived bound and not the production tolerance (DEC-026): the first-order bound n·u·√(λ_iλ_j)/λ₁ was withdrawn after review round 2, which showed a close pencil (K = M = [[2, 1], [1, ½ + 2⁻³⁰]]) at about 2.65·10⁻¹², beyond it; that witness is a test that the ratio is finite and reported.
- The PR #33 review witnesses: component bodies whose |r|² under- or overflows while every m·r_a·r_b is admitted, and the design's admission witnesses (§3.2 item 5), a small-bore one-element cantilever (smallest/largest mass diagonal 6.0·10⁻⁶, equilibrated pivots all ≥ 0.22) and a short-member cantilever (0.3 m then 0.02 m of NPS 6), admitted with their full finite counts.

### Admission range and resolution floor (review round 3)

`tests/admission.rs` asserts each round-3 range witness refused at entry (`MODAL_PENCIL_RANGE_UNSUPPORTED`): K = 2⁻⁵²⁰ with M = 2⁵²⁰, K = 1 with M = 2⁻¹⁰⁴⁰, the shape witness's h = 2¹⁰⁰² (reconstructed as a pencil entry), the residual pencils K = M = 2⁻¹⁰²⁰ and 2¹⁰²², the formation difference's K = 2⁻¹⁰⁰⁰ against K̂ = 2¹⁰⁰⁰, and orthogonality's M = diag(2⁻¹⁰²², 2⁻¹⁰²², 0). Earlier witnesses outside the range assert the refusal and keep an in-range check of their mechanism where one is reachable. In range, a generalized mass of 25·2⁻¹²⁹⁶ gives the true orthogonality ratio 0.8.

### Band-screen candidate (2026-11-01)

The existing test-local coefficients are c_μ = 1, κ_μ = 10 and c_f = 64. Agent 0 authorized explicit **TEST-LOCAL τ_e = 1e-6** for the candidate; there is no production value/default or DEC-026 change. η̃ and e_λ use the equations in `dense.rs` and Work §3.2: e_λ estimates relative eigenvalue error and is dimensionless, with a six-decimal accuracy target for recovered band modes only, independent of a desired count. Above-band QL modes are not judged by τ_e; §3.4 check 2 reports their per-mode accuracy observations. It is neither a frequency tolerance nor an error bound. The prior independent replay's 1e-2 was a historical diagnostic input only.

The former normwise screen is unsound for this purpose. The closed-form nearly massless witness with φ=(0,1), λ=2^52 has η_f < 1e-14 but e_λ > 1e14, regardless of vector normalization. The QL floor rejects the corresponding kernel artifact. An exactly solved pair below either hard floor still stays unresolved. GENERAL's non-coordinate nullspace is not rank-truncated: it returns 21 positive modes with an unresolved census, and the spurious candidate has negative exactly accumulated generalized mass.

On E-RIGID-SCALE-1e6, all six high band modes are recovered; the other modes retain their QL values. Recovery improves the high eigenvalues from relative errors up to 3e-8 to below 1e-14 against the independent 80-digit references. Their equilibrated η̃ is at most about 1.1e-16, and e_λ at most about 3e-8. Diagnostic false-pair estimates were at least 3e3. These observations separate the tested populations; they guarantee nothing outside that domain. At τ_e=1e-7, 1e-6 and 1e-5 the reference result is identical. Tests also check immediate neighbours and equality at hard/band and error thresholds, normalization by 2^±300 and sign, diagonal congruence, positive-denominator premises and bit-identical reruns. Existing frozen expected values are unchanged. The original failed six-test run and diagnostic probe remain run evidence, not test dependencies.

The QL floor limits a resolved whole spectrum at approximately the span 1/(κ_μ c_μ n u); this is an algorithmic screen, not a certified bound. GENERAL and any model without an established census retain `MODAL_FINITE_COUNT_UNRESOLVED` standing when the whole-spectrum endpoint is required. Rank-revealing mass deflation and per-pair recovery below the floors are outside this candidate.

The frame kernel's seam has its own tests (`frame_kernel/src/structural/element_reformation_tests.rs`): with the binary64 leaves as operands, a straight (Euler–Bernoulli and Timoshenko), a bend, a connector and a tie rod match K-D5's p128 re-formation bit for bit.

Independent review of numerical source `0ba65b9d4` found no blocking numerical defect. Its additional exact-spectrum probes found above-band errors of 1.58e-5 in a resolved case and about 0.9% in unresolved cases; this candidate makes no whole-spectrum six-decimal claim. The first-order error estimate can safely reject accurate band pairs. An all-negative mass matrix, outside the PSD premise, receives a generic typed QL-floor range refusal. Work §3.2 records these limits for M1b without accepting them as release limitations. The review-supported wording correction limits recovery by decision, rather than asserting that Rayleigh recovery generally worsens low modes.

### Workspace ledger (design §5.3.8, Agent 0's option (c))

These are requested bytes for a 64-bit layout, taken from source. They are no RSS guarantee:
- Allocator size-class rounding and metadata, B-tree node layout, struct sizes (derived from field types), the lock's OS primitive, stack, closures, the runtime, and the product's custody of the outcome are separate allowances.
- A passing test does not prove any model's heap peak.
- No ceiling, ρ or limb allotment value is adopted; 202 is only a proposed allotment.

**Notation:**
- c(L) = max(4, 2L − 2) limbs bounds the capacity a limb vector *requests* under `reserve_limbs`' growth rule (`exact.rs:59–80`), where L is the longest it has held. `Vec::reserve_exact` may return more, so a vector's actual capacity is an allocator allowance, not c(L). With an allotment A, the request is capped at A.
- s(L) = max(4, 2L).
- Sizes: `ExactSum` 48 B, `Term` 16, `WideDd` 24, a `Vec` header 24.
- n is the free order; n_c the computed pairs; n_r the reported modes.
- Stages run in the order of `run_modal_dense`: T6, T7, T8, T9, T10. The basis and the re-formed pencil both live until the run ends.

**By stage:**
- **T6, normalization.** Workspace W6 = 8·c(L₆) + 8·s(L₆) + 8n (`modal_quantities.rs:323`, `:426`).
  - Outputs: at most 16·n_r·n, at exact length (`:421`, `:441`).
  - A refusal mid-shape frees the partial shape, of capacity n (`:428`).
  - W6's capacity stays retained through T7 to T10.
- **T7, formation.** `new` builds one map per matrix and drops it before building the next (`reformation/mod.rs:521–535`).
  - Each entry: 64 + 8·c_A(L₇) plus map-node overhead (an allowance), plus one 8·s(L₇) scratch.
  - In `formation_difference`:
    - the formed lists, 24·(nnz_l(K) + nnz_l(M)), live for the whole call;
    - the scaling, 16n, is retained in the output;
    - per matrix, one at a time: the map, accumulators of nnz_l·(48 + 8·c_A(L)), a scratch of 8·s(L), terms of 16·c(L), the off-pattern map, and 24n for the norms.
  - Every limb growth of the formation maps' and accumulators' sums is checked against an optional allotment before it is allocated. Scratch and term-list growth is not allotted; it runs with `None` (`exact.rs:328`, `:368`) and is counted in the ledger above. Map nodes are allocated before that check; their count is bounded by Σ_e d_e(d_e + 1)/2, which the caller's P0 must price.
- **T8, residuals.** One workspace for all n_c pairs: W8 ≤ 2n·(48 + 8·c(L*)) + 24·c(L*) + 8·s(L*). It holds the rows, the mass rows, the generalized-mass sum, the term list and the scratch (`mod.rs:1037–1062`).
  - The pencil's element lists and the scaling are live inputs at the same time.
  - Refusals before `:1037` allocate nothing; later ones leave only W8.
- **T9, Sturm.** Add W6 + W8's retained capacity to T9's own peak.
- **T10, orthogonality.** It shares W8: its rows and sum may grow, so W8 uses L* = max(L₈, L₁₀). **All of W8 stays retained while T10 builds the products.**
  - Retained products: P10 = 24·n_r + 24·n_r·n + 16·Σk_ij ≤ 24·n_r + n_r·n·(24 + 16·L₁₀), plus roots of 24·n_r (`orthogonality.rs:130–136`, `:180`).
  - The products are freed on return, whether the stage succeeds or refuses.

**Growth and clones:**
- Growth: one `Vec` grows at a time (`exact.rs:78`). Add one old buffer during the growth: 8·c(L) for limbs, 16·c(L) for terms. The row vector grows once, from 0 to n.
- Clones: none on this route. A pencil or basis clone starts with an empty workspace (`workspace.rs:34`).

**Not reused:**
- band recovery's per-row and per-pair allocations (`dense.rs`);
- two small sums per neighbour in the Sturm mass quotient (`sturm.rs:698–699`).

**Evidence:**
- `tests/workspace_reuse.rs` compares against digests captured at base `68a039e1c`, over 2,420 lines.
- A scratch producer-edge dump was also byte-identical at base and head. It covers 26 references, three lower-count retries and four SEC-A runs, 33 records in all, and is not committed.
- Equal shortest round-trip text establishes identity of finite values, telling −0.0 from 0.0. For NaN it records only `is_nan()`, so NaN payload identity is not established.
