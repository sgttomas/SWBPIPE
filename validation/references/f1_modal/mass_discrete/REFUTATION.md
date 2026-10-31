# Refutation record: f1_modal mass and class (a) discrete references

This is the refuting TASK's record for the candidate references in this
directory (DEL-04-09, AC-001 and AC-003; RT-A7, RT-V1). It covers the README,
`mass_references.json` and `eigen_references.json` at SHA256SUMS as found
(README `3525dc16…`, mass `8ed1f462…`, eigen `53269c70…`). The governing text
is `execution/PKG-04/DEL-04-09/Work/modal-analysis.md`. The assigned basis was
9e1a38f72. While this record was being written the branch advanced to
d24234a68 (design review round 4). That diff leaves §1.3, §1.5, §2.1–§2.5 and
§4.3 unchanged in every respect these references depend on, so the verdicts
hold at both revisions.

Standard claim fence applies (claims taxonomy per DEC-081). This record makes
no acceptance statement and sets no product comparison tolerance.

## Independence

- The refuting scripts read, imported and ran no product code: nothing under
  `core/`, `apps/`, `api/`, `tests/`, `schemas/` or `fixtures/`.
- They did not read, import or reuse the package's generator code. The
  generator was run only for the determinism check (item 5).
- They were written from the design text and textbook formulas, using the
  Python standard library only (`fractions`, `decimal`), and run with
  `python3 -I` from a scratch directory outside the repository. That directory
  has since been removed.
- The textbook bases were:
  - Przemieniecki (1968), ch. 11: consistent Hermitian and Rayleigh mass, and
    rigid-body spatial mass;
  - Castigliano: arc flexibility from the complementary energy
    N²/EA + T²/GJ + k(|M|² − T²)/EI;
  - Householder tridiagonalization and Sturm sequences (Golub & Van Loan).

## Method of the refuting scripts

- **Sections.** The scripts compute exact rational coefficients of π from OD,
  t, c_m, c_a and the densities, on both bases:
  - PF: metal and contents on the wall t − c_m;
  - EX: metal on t, and contents on the bore from t_r = (t − c_m) − c_a.

  j̄ uses π(D_o⁴ − D_i⁴)/32 on the basis's metal annulus plus insulation, with
  ī = j̄/2 and contents excluded. A, I and J = 2I come from the PF wall
  t − c_m. The values are compared with the published `exact` strings for
  equality as Fractions.
- **Straight members.**
  - The local 12×12 is built explicitly from the design's displayed matrices.
    The x–y plane acts on [u_y, θ_z]. The x–z plane acts on [u_z, −θ_y],
    applied as S·P·S with S = diag(1, −1, 1, −1).
  - It is rotated to global axes with a Gram–Schmidt frame from `y_reference`,
    at 150 digits.
  - The sign map is checked independently of any frame-free formula. For a
    rigid rotation ω about node i, qᵀMq must equal the exact rigid-body energy
    m̄L³/3·|ω×e|² + L(j̄(ω·e)² + ī(|ω|² − (ω·e)²)). The Hermitian field
    reproduces a rigid rotation only if the x–z coupling signs are right.
  - The weight identity is checked against the scripts' own consistent load:
    a·m̄L/2·k̂ at each end, and moments ±a·m̄L²/12·(e × k̂).
- **Arc geometry and lumps.**
  - The centre is placed on the side of the chord opposite the bow, which is
    `y_reference` projected normal to the chord. Then φ = 2·asin(c/2R).
  - Each half-arc lump is integrated in global coordinates by a different
    route from the package: exact Fourier-coefficient recovery of the
    trigonometric-polynomial integrand from 12 samples on the full period
    (the aliasing check is below 1e-147), then exact antiderivatives at
    150 digits. The integrand is mass, first moment,
    ∫m̄(|ρ|²I − ρρᵀ)ds and ∫(j̄ttᵀ + ī(I − ttᵀ))ds.
  - Local-frame closed forms in α were derived by hand and compared entrywise.
  - Their Taylor series were derived twice in exact rationals: from the closed
    forms, and by integrating the integrand series term by term. Both were
    compared with every published `leading_terms` coefficient.
- **Arc stiffness.** A 6×6 tip flexibility of the arc clamped at i, from the
  energy above, uses the same Fourier-recovery integration (degree 4,
  20 samples). It is inverted and spread to both ends by rigid-body
  equilibrium, K = [ΓᵀKΓ, −ΓᵀK; −KΓ, K].
- **Components.** The scripts form m = W/a exactly and the 6×6 block
  [[mI, −m[r]×], [m[r]×, m[r]×ᵀ[r]×]]. Eligibility is consumption, then
  mapping, weight and centre of gravity (DEL-03-05 §3). The mapped member's
  pipe mass is replaced and its stiffness is scaled by s. Binary64 premise
  tests use native floats.
- **Eigen.**
  - K_ff and M_ff are formed at 100 digits, then K_ff = LLᵀ and
    C = L⁻¹M_ffL⁻ᵀ.
  - C is reduced by Householder tridiagonalization, and μ = 1/λ is found by
    bisection on the tridiagonal Sturm sequence. This is not the package's
    pencil-LDLᵀ bisection, and not its Jacobi route.
  - The finite count is the number of μ > 1e-40·μ_max.
  - Independent counts come from unpivoted LDLᵀ of K_ff − σM_ff at 100 digits,
    at every published binary64 shift (from its hex form) and at every
    certified-bracket σ.
  - Each published shape is checked against the scripts' own K, M and λ:
    - the residual and Rayleigh quotient;
    - the normalization case by t ≤ 1e-6·ℓ·θ;
    - the normalizing DOF, ties (lowest index) and `compare_sign_invariant`;
    - m_i, Γ and m_eff on the scripts' own p_k = (M R_k)_f, fractions over the
      scripts' own T_k, and cumulative sums;
    - cluster sums and M-orthogonality.
  - Σ m_eff over all finite modes is checked as p_kᵀM_ff⁺p_k, by
    (M_ff + 1e-60·K_ff)⁻¹, which also covers the singular M_ff of
    `E-POINTMASS-*`.

## Results

### Mass references (`mass_references.json`; all 11 cases, PF and EX)

| Check | Largest discrepancy |
|---|---|
| Section m̄ by category, j̄, ī, A, I, J, inner diameters (exact) | identical as rationals, every section, both bases |
| Straight 12×12 vs published decimals (relative to the largest diagonal entry) | 3.2e-40 |
| Straight 12×12 vs published `exact` strings | 8.2e-150 (the working precision); no missing nonzero |
| Rigid-rotation energy (sign map), 6 orientations × 4 ω × both bases | 1.5e-149 |
| M[u_y,i][θ_z,i] = +22m̄L²/420 + ī/10, M[u_z,i][θ_y,i] = −(same), members along +x | 8.5e-150 |
| Straight a·M_e·r_k vs the scripts' own consistent load; published vectors vs it | 2.3e-150; 40-digit agreement on all members, X/Y/Z |
| Arc angle, centre, mass, closed-form centroid | ≤ 8.3e-40 relative |
| Half-arc mass, first moment, centroid offset, J_n, 6×6, section term, centreline-only variant | ≤ 1.4e-39 (relative to m·R²); J_n entrywise relative to its own entry ≤ 2.5e-40 |
| `M-ARC-SMALL` (sin α = 1e-4): Jc_22 ≈ 6.9e-20 kg·m² vs closed form at 150 digits | 4.3e-41 relative (PF), 2.3e-41 (EX) |
| Local-frame S, Jc, Js vs hand-derived closed forms, entrywise | 2.9e-40 relative to own entry; zero entries exactly 0 |
| Series `leading_terms` (10 entries × 3 terms) vs exact rational Taylor coefficients, two derivations | all equal; each is the leading nonzero term |
| Arc lumps' resultant vs closed-form arc weight (scripts' own); published resultants vs it | 1.2e-149; 2.8e-40 |
| Identity reference point = node centroid | equal to binary64 rounding (7.4e-17 absolute) |
| Component m = W/a, 6×6, wrench a·M_c·r_k = (W·k̂, r × W·k̂) | exact (6×6 to 2e-148 absolute, the Decimal rendering of n/157) |
| `range_premise` fl(W/a), fl(a·fl(W/a)) for 2560 N / 9.8125 | identical binary64 |
| Component status, first unmet condition (consumption; centre of gravity) | as published; NO-COG keeps pipe mass and s = 8 |
| Category totals, T_k, free-DOF mass, not-participating mass | ≤ 4.3e-40 relative |
| Assembled full M (every upper entry, both directions of the nonzero pattern) | 3.2e-40 relative to the largest diagonal entry |
| Whole-model nodal a·M·r_k | 4.2e-40 relative |

Range-premise witnesses, evaluated in native binary64:

| Witness | W, a normal | fl(W/a) normal | fl(a·fl(W/a)) normal | spatial block | Published `fails` |
|---|---|---|---|---|---|
| W = 5e-324 | no | no | no | ok | W ✓ |
| W = 1e-300, a = 1e10 | yes | no (1e-310) | yes | ok | W/a ✓ |
| W = max, a = 0.5 | yes | no (inf) | — | — | W/a ✓ |
| W = 1, a = 1e-300, r = (1e10, 0, 0) | yes | yes | yes | overflow | spatial block ✓, and the only failing part |
| W = max, a = 1.001953125 | yes | yes (1.7941888597456251e308) | no (overflow) | ok | a·(W/a) ✓ |

### Eigen references (`eigen_references.json`; all 26 cases, 493 modes)

Every case was rebuilt and solved independently, beyond the five requested:
- the cantilevers 1, 2, 4 and 8, and the inclined cantilever;
- SS-1 to SS-8 and FF-2 to FF-8;
- the four point-mass cases;
- the L-lines and Z-bends at k = 1, 3 and 2.5;
- the four rigid-scale cases;
- the axial witness and the springs case.

| Check | Largest discrepancy |
|---|---|
| Free-DOF list, finite count (rank of M_ff for `E-POINTMASS-*`) | all equal |
| Eigenvalues (36 published digits) | 4.5e-36 relative (`E-RIGID-SCALE-1e6`, mode 1) |
| Frequencies, periods, next eigenvalue above the set | ≤ 5.0e-36 relative |
| Sturm counts at all 1,282 published binary64 shifts, including every fl(λ) and neighbour witness | all equal. The smallest pivot is 1e-19 relative, at `E-POINTMASS-AXIAL` fl(λ₁), resolved at 100 digits |
| Certified-bracket counts (416 brackets, 827 shifts) | all equal |
| Rayleigh quotient of each published shape vs the scripts' own λ | 2.1e-43 relative |
| Residual ‖Kφ − λMφ‖/‖Kφ‖ of the 25-digit shapes | 1.0e-24, except the rigid-scale cases: 5.2e-18 at s = 1e6, where the 1e6 stiffness contrast amplifies the shape's 25-digit rounding. The Rayleigh quotient shows the shape is right |
| Normalization case, normalizing DOF = +1, tie lists, `compare_sign_invariant` | all equal. Case (a) modes have t/(ℓθ) ≥ 1.6e-3, at least 1600× τ_t. Case (b) modes have t = 0 exactly |
| Generalized mass and units (kg; kg·m²·rad⁻²), Γ unit (1; rad/m) | 8.6e-25 relative; units all as §1.5 |
| m_eff / T_k, fractions, cumulative fractions, cluster m_eff sums | ≤ 2.8e-25 absolute |
| Γ | 1.4e-23 of the case's largest Γ. This comes from evaluating Γ on the rounded 25-digit shape; m_eff agrees to 2.3e-25 of T_k |
| T_k; Σ m_eff over all finite modes as p_kᵀM_ff⁺p_k; mass not captured by any mode | ≤ 4.2e-40 relative |
| Bessel's inequality: Σ m_eff ≤ T_k | holds in every case and direction |
| M-orthogonality of published shapes | 1.4e-24 (cosine) |
| Arc stiffness (Castigliano, k = 1, 3, 2.5) as seen through the L-line and Z-bend eigenvalues | 36-digit agreement |

The case-specific checks:
- **`E-AXIAL-WITNESS`.** The exact m/(2k) = m̄L²/(2EA) is
  12654223/75950000000000 s², as the README states. The scripts' modal
  u/A = Γφ/λ agrees to 2.3e-63, and m_eff/T = 3/4.
- **`E-SS-1`.** The first bending pair has all free translations exactly 0. It
  is case (b), and carries 0.83145486366928998975 of T_Y and T_Z through
  M_fc, as Design §1.5 item 2 cites.

### Determinism

- `shasum -a 256 -c SHA256SUMS` passes for all 10 files as found.
- `python3 -I -B generators/f1_mass_discrete.py` reports "check passed" and
  regenerates both JSON files and SHA256SUMS byte for byte. It does so with
  the default jobs (3.1 s) and with `--jobs 1` (16 s).
- **After this record's hash was added to SHA256SUMS** (as assigned), the
  generator's check mode reports `MISMATCH: SHA256SUMS` and exits 1. It
  regenerates SHA256SUMS from its own file list, which does not include
  `REFUTATION.md`. The JSON files still match, and `shasum -a 256 -c` passes
  on all 11 files.
  - This is reported, not repaired. The generator's file list, or its check of
    SHA256SUMS, needs the reference owner's change, for example to carry
    `REFUTATION.md` through, as `../continuum` does.
  - Until then, the README's check command fails on a correct package.

## Consistency with the design text

The README's stated premises match the design. The scripts encode them and
reproduce the published values:
- §1.3, §1.5:
  - M_ff from the same partition;
  - the D-v excitation p_k = (M R_k)_f;
  - fractions over T_k, with the free-DOF mass only a summary;
  - τ_t = 1e-6 and ℓ over all nodes;
  - case (b) with no participation condition, and the units of both cases.
- §2.1: both mass bases.
- §2.2:
  - the displayed matrices and the sign map;
  - j̄ on the basis's metal wall, with contents excluded, and ī = j̄/2.
- §2.3: W/a, and the fourth range part.
- §2.4: the half-arc lumps with section rotary inertia.
- §2.5: the identities.
- §4.3: the arc energy.

The README correctly marks the tie tolerance (1e-25) as assumed. The cluster
tolerance, the 24-mode cap and the cluster basis rule are reference-internal
presentation choices, and are disclosed as such. The eligibility order is cited
to DEL-03-05 §3, and matches that text.

Findings that do not change a published value:

1. **Warning-code attribution.** The README ("Findings for the design owner")
   says the design resolved the warning code to
   `SELF_WEIGHT_COMPONENT_WEIGHT_NOT_APPLIED`. The design text still says
   `COMPONENT_WEIGHT_NOT_APPLIED`, at §2.1 (line 150) and §2.3 (line 203),
   both at 9e1a38f72 and at d24234a68. The published code follows DEL-03-05
   (`rigid-component-stiffness-scaling.md` line 290), which is the stronger
   source. Either the design text or the README's attribution needs
   correcting. Route: the design owner.
2. **An unstated premise in the README.** `mass_references.json` publishes EX
   `stiffness_section` A, I and J on the nominal wall t (for example
   A = 27/16384·π for s1). The README's "Stiffness" premise names only the PF
   wall t − c_m. The values agree with Design §3.4 ("Trusted leaves": "on EX,
   the D-5 nominal-wall basis"), and no eigen case uses them (EX eigen is out
   of scope). The README should state this premise.
3. **Design text, for the design owner.** §2.3 item 6 says "The premise has
   three parts" and then lists four. The references follow the four.
4. **Cosmetic.** Witness 4's note says "m*r_x^2 overflows". m·r_x = 1e310
   already overflows, so both kinds of entry of the fourth part fail. A product
   that names the first failing entry will name m·r_x.
5. **Cosmetic.** The whole-model nodal `a_M_r_nodal` prints some exact zeros
   as decimal residues of up to 4e-79 absolute. Examples are the Y and Z
   force entries of `S-2M36` in `M-STRAIGHT-ORIENT`. The README's "below 1e-50
   printed as 0" rule is stated for shapes and participation only, so this is
   no contradiction. A comparison should use an absolute floor.

## Verdicts

| Case group | Verdict |
|---|---|
| Straight member mass, both bases (`M-STRAIGHT-ORIENT`, and the straights in every case) | not refuted |
| Bending-plane sign map | not refuted |
| Straight weight identity | not refuted |
| Half-arc lumps (`M-ARC`, `M-ARC-SMALL`, the arc in `M-MODEL`), local frames and series | not refuted |
| Arc resultant identity | not refuted |
| Component bodies (`M-COMP-COG-*`), ineligible cases, massless families, `M-MODEL` | not refuted |
| Range-premise witnesses | not refuted |
| Category totals, T_k, free-DOF summary, assembled M | not refuted |
| Eigen: cantilevers, SS, FF, inclined | not refuted |
| Eigen: point-mass cases, including the offset centre of gravity and singular M_ff | not refuted |
| Eigen: L-line and Z-bend (k = 1, 3, 2.5) | not refuted |
| Eigen: rigid-scale cases (s = 1e3 to 1e6) | not refuted |
| Eigen: springs; axial witness | not refuted |
| Sturm counts and certified brackets | not refuted |
| Normalization, participation, clusters | not refuted |
| Determinism and hashes | not refuted |
| README premises vs the design | not refuted. Findings 1 and 2 are wording and attribution corrections for the owner; finding 3 concerns the design text |

Largest discrepancy against the published precision: the eigenvalues agree to
4.5e-36 relative (36 digits published), the masses to 1.4e-39 (40 digits
published), and the shapes and participation to about 1e-24 (25 digits
published). The exception is Γ, at 1.4e-23 of the case's largest Γ, and the
rigid-scale shape residual of 5e-18. Both are explained above by the scripts'
use of rounded shapes, and both are confirmed by the 2e-43 Rayleigh quotient
and the 2e-25 m_eff agreement.
