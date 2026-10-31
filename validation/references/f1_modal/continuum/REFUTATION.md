# Refutation of the F1 continuum references

The refuting TASK's record for `validation/references/f1_modal/continuum`. It
answers the independence rule of the design
([modal-analysis.md](../../../../execution/PKG-04/DEL-04-09/Work/modal-analysis.md)
§4.1). The rule requires a second TASK to try to refute a reference before
anything relies on it.

- **Date:** 2026-10-30.
- **Basis:** the package as listed in `SHA256SUMS` before this file was added,
  and the design at commit 85622cfe1. It was checked against §2.2, §2.4,
  §3.3, §3.4 and §4.1–§4.3.
- **Result:** 3,785 independent checks were run, and no reference value is
  refuted. Two kinds of descriptive statement are refuted: the stated
  convergence rate of the lumped control, and two README statements about the
  design that the current design no longer supports. See
  [Discrepancies](#discrepancies).

## Independence

- No product code was read, imported or run: nothing under `core/`, `apps/`,
  `api/`, `tests/`, `schemas/` or `fixtures/`.
- The package's generators were read for their imports only. Their code was
  not reused, imported or run as a check. The one exception is the
  `generate.py --check` integrity run that the brief asks for.
- The checks are my own standard-library Python, run with `python3 -I` from a
  scratch directory outside the repository. The directory was removed
  afterwards.
- Formulas come from textbook statements and my own derivations:
  - Euler–Bernoulli and rods: Timoshenko, Young & Weaver; Blevins.
  - Rayleigh beam and tip-mass boundary conditions: Hamilton's principle, as
    in Han, Benaroya & Wei.
  - Rings: Love and Hoppe; the curved-bar energies were re-derived.
  - Element matrices: Przemieniecki.
  - LDLᵀ backward-error setting: Higham.
- **Numerics.** These differ deliberately from the generator's methods:
  - π from the Gauss–Legendre (AGM) iteration, not Machin's formula.
  - sin and cos from a Taylor series after eight argument halvings.
  - Roots by Newton's method (Euler–Bernoulli) or by a scan followed by
    bisection on reduced 2×2 determinants, not 4×4 ones.
  - `decimal` at 80 digits.
  - Exact inertia from Descartes' rule of signs on the exact characteristic
    polynomial (Faddeev–LeVerrier, or the three-term recurrence for the
    tridiagonal witness). Descartes' rule is exact for real-rooted
    polynomials. Exact unpivoted LDLᵀ in `fractions` was used alongside it,
    and every count was required to agree between the two.

## Scripts (summarized; scratch, not retained)

| Script | Method |
|---|---|
| `mylib.py` | AGM π; sin and cos by halving and Taylor; cosh and sinh from `Decimal.exp`; Newton, bisection and scan |
| `exact_inertia.py` | exact inertia: Faddeev–LeVerrier characteristic polynomial with Descartes' rule; exact LDLᵀ cross-route |
| `check_inputs.py` | every `{binary64, hex, exact}` triple, every `{value, binary64}` pair, every `{exact, value}` pair, all three JSON files |
| `check_continuum.py` | sections from OD and wall (π factored out, exact); Euler–Bernoulli roots by Newton on cos x ± 1/cosh x; rods; Rayleigh beams from my own 2×2 determinants; tip mass from its 2×2 determinant; lumped controls with exact counts |
| `check_rings.py` | my own in-plane and out-of-plane energies per harmonic n, exact 2×2 pencils; all branches for n = 0…30 enumerated and sorted |
| `check_fe.py` | textbook element matrices, assembled in `fractions`; eigenvalues by Sturm bisection at 80 digits; published brackets re-counted exactly |
| `check_coarse.py` | all 305 coarse-mesh rows and the summary for c = 1…10; a lumped cantilever as context |
| `check_witnesses.py` | exact counts at all 106 shifts; constructions; eigenvalues; my own binary64 LDLᵀ in the file's stated operation order |
| `check_round2.py` | my own construction of the 32×32 pencil; two exact count routes; three binary64 LDLᵀ orders; an independent re-run of the kk search |

## Results by case group

### Inputs and package integrity: not refuted

- 205 `{binary64, hex, exact}` triples agree, and each `binary64` is the
  shortest round-trip form.
- 529 `{value, binary64}` pairs give the nearest binary64.
- 90 `{exact, value}` pairs agree to within the 40-digit rounding.
- All witness matrices are written as exact binary64 values.
- Before this file was added, `python3 -I -B generators/generate.py --check`
  reported "cross-checks: 76 recorded, 0 disagreeing; verified 10 files byte
  for byte", and `shasum -a 256 -c SHA256SUMS` passed. See also
  [the note on `--check`](#note-on-generatepy---check).

### Sections and π-free ratios: not refuted (36 checks)

- I re-derived the six exact ratios for both sections from the binary64
  inputs, and all are equal as rationals: ID, EI/m̄, EA/m̄, GJ/j̄, ī/m̄ and
  GJ/EI.
- The twelve derived quantities agree to ≤ 2.6·10⁻⁴⁰ relative:
  - the areas, I, J and J_ins;
  - m̄, j̄ and ī;
  - EI, EA and GJ.
- For SEC-A, GJ/j̄ = G/ρ_p exactly, as it must be with no insulation and the
  contents excluded.

### Euler–Bernoulli beams: not refuted (174 checks)

- All 18 roots (6 per family) and their squares agree to ≤ 3.3·10⁻⁴⁰
  relative. Each frequency-equation residual is below 10⁻⁷⁰.
- The design's §4.2 roots are correct to the digits stated. The largest
  difference is 2.88·10⁻¹⁰, for the first cantilever root.
- The 9 realized cases (ω and f, 6 modes each) agree to ≤ 4.5·10⁻⁴⁰, and
  each EI/(m̄L⁴) is exactly equal.

### Rods: not refuted (81 checks)

- All four coefficients agree for each support.
- The 8 cases' exact stiffness/(inertia·L²) values are equal.
- ω and f agree to ≤ 4.3·10⁻⁴⁰.
- Axial uses m̄ with the contents, and torsion uses j̄ without them (D-e).

### Rayleigh beams: not refuted (183 checks)

- **Derivation.** I re-derived the boundary conditions from Hamilton's
  principle with T = ½∫(m̄ẇ² + īẇ'²):
  - free end: EIW'' = 0 and EIW''' + īω²W' = 0;
  - in nondimensional form: W''' + r²Ω²W' = 0;
  - characteristic equation: s⁴ + r²Ω²s² − Ω² = 0.
- **My route.** It is a reduced 2×2 determinant. It was first validated at
  r² = 0 against my Euler–Bernoulli roots, to below 10⁻⁶⁰.
- **Values.** All 54 published Ω values agree to ≤ 4.1·10⁻⁴⁰ relative:
  - simply supported, in closed form;
  - cantilever and fixed-fixed, from the determinant;
  - for each of SEC-A at 6 m, SEC-B at 4.5 m and SEC-A at 1 m.
- **Shifts.** The `rel_shift_from_euler_bernoulli` values agree to their
  published 6 digits.
- **Exact r².** Each r² equals ī/(m̄L²) exactly.

### Lumped controls: not refuted (46 checks)

- **SDOF.** ω² = k/m exactly.
- **Point mass on a massless cantilever.**
  - ω² = 3EI/(M_tL³) = 5120 exactly.
  - The kernel K equals the one-element Hermitian stiffness with the root
    fixed, and M = diag(M_t, 0).
  - The exact counts at 5120(1 − 2⁻⁴⁰), 5120 and 5120(1 + 2⁻⁴⁰) are (0, 0),
    (0, 1) and (1, 0), as published.
- **Chains, N = 3, 5 and 10.** Every ω_j agrees to ≤ 4.3·10⁻⁴⁰. An exact
  Sturm count on my own fixed-free tridiagonal brackets each λ_j at
  λ_j(1 ∓ 10⁻³⁰).

### Tip mass: not refuted (127 checks)

- **Equation.** My 2×2 determinant uses W = a(cosh − cos) + b(sinh − sin)
  with W''(1) = 0 and W'''(1) + μx⁴W(1) = 0. The end condition comes from
  Hamilton's principle with the tip's kinetic energy.
- **Roots.** It reproduces all 36 nondimensional roots (μ = 0, 1/10, 1/2, 1,
  2 and 10) to ≤ 4.7·10⁻⁴⁰. The package's published closed-form equation has
  a residual below 10⁻⁶⁰ at my roots.
- **Rayleigh quotient.** √(3/(μ + 33/140)) lies above mode 1 by less than
  1.5 % for every μ > 0, and by 7.66·10⁻⁶ at μ = 10.
- **Dimensional case.**
  - M_t = W/a exactly.
  - μ, the six roots and the ω values agree to ≤ 4.7·10⁻⁴⁰.

### Free rings: values not refuted (318 checks); one README statement refuted (D-2)

- **In plane.** I derived the energies independently, with
  ε = (v' + w)/R, κ = (w'' − v')/R² and χ = (w' − v)/R.
- **Out of plane.** κ = (w'' − ψ)/R² and τ = (ψ' + w')/R², with kinetic
  energy m̄ẇ² + j̄φ̇² + ī(ẇ'/R)².
- **Pencils.** The resulting 2×2 pencils equal the `variants` text. Love's
  and Hoppe's formulas, and the inextensional rotary formula, are exact
  rational identities of these pencils (condensation or the constraint
  V = −W/n).
- **Values.** All five variants for n = 2…6 agree to ≤ 4.3·10⁻⁴⁰ in λ, ω
  and f, for both rings. All exact parameters are equal: α, γ, ρ, ρ_t and
  EI/(m̄R⁴).
- **Complete low spectrum.** I enumerated every branch for n = 0…30.
  - Six rigid modes: in-plane n = 0 rotation and n = 1 translations;
    out-of-plane n = 0 translation and n = 1 rotations.
  - Breathing λ = AR²/I and uniform twist λ = m̄R²/j̄.
  - Both cut-offs (the lowest branch with n > 8).
  - Every listed branch, its label, multiplicity and running count.
  - The totals, 44 (SEC-A) and 48 (SEC-B).

  All agree, and no branch with n > 8 lies below the cut-off.
- **README shifts.** The quoted percentage shifts are confirmed: out of plane
  0.666 % (n = 2) and 3.02 % (n = 6); in plane 0.416 % and 2.62 %.

### FE convergence, consistent mass: not refuted (1,143 checks with the lumped control and rods)

- **Element matrices.** Khat, Mhat, Rhat and the rod matrices equal the
  textbook element matrices, and the stated h-scaling rule holds (tested at
  h = 1/3).
- **Assembly and eigenvalues.** I assembled independently in `fractions` and
  computed eigenvalues by Sturm bisection. All 237 published Ω² values that
  I recomputed agree to ≤ 4.3·10⁻³⁶ relative (36 digits are published):
  - Euler–Bernoulli at N = 1, 2, 4 and 8, and the cantilever also at N = 16;
  - Rayleigh SEC-A at L = 6 m to N = 8, and at L = 1 m to N = 4;
  - the tip mass to N = 4 for all μ, and μ = 1 to N = 8;
  - the rods to N = 16;
  - the lumped control to N = 16.
- **Exact brackets.** 182 published exact Sturm brackets (N ≤ 4, and all rod
  rows) give counts k − 1 and k when re-counted exactly.
- **Errors and orders.**
  - Every `rel_freq_error` agrees to its 6 digits, and every
    `observed_order` I could form agrees to its 4 digits.
  - Every consistent-mass row checked lies above the continuum.
  - The cantilever's mode-1 order from N = 8 to 16 is 3.9933. The rods
    converge at order 2.00.
  - One simply supported element gives Ω² = 120.
- **Rod closed form.** Ω² = 6N²(1 − cos t)/(2 + cos t) agrees with my
  eigenvalues to 36 digits.
- **Coarse-discretization data.** All 305 rows agree: boundary, N, mode,
  f_e/f_n and error. So does every entry of `summary` for c = 1…10, both the
  row counts and the largest errors. The README's table is confirmed: 11.0 %,
  2.9 %, 2.9 %, 1.8 %, 0.85 %, 0.41 % and 0.30 %.

### FE lumped control: values not refuted; its stated rate refuted (D-1)

All lumped-control values agree, for example √96 at N = 2. The rate claimed
for it does not hold for this case.

### Inertia witnesses: not refuted (511 checks)

- **Exact counts.** All 106 shifts were counted exactly by two routes, and
  every count below and equal agrees with the file.
- **WIT-MISSED.**
  - K = I and M = diag(1, 2, 3) give eigenvalues 1/3, 1/2 and 1.
  - The count at 1.5 is 3.
  - At 0.5 the counts are 1 below and 1 equal. Binary64 meets an exact zero
    pivot.
- **WIT-NEAR-PAIR and WIT-CLUSTER.** K = BᵀΛB and M = BᵀB hold exactly, and
  the products are exact binary64 values. B is nonsingular, and each
  constructed eigenvalue has an exact count of (i, 1).
- **WIT-SCALE-1E3 and WIT-SCALE-1E6.**
  - The chain K and M equal the stated springs and masses.
  - All 12 eigenvalues, published to 45 digits, are bracketed exactly at
    λ(1 ∓ 10⁻⁴²), and all published brackets re-count.
  - The shifts lie at relative offsets of 10⁻⁸, 10⁻¹³ and ≤ 10⁻¹⁶ (nearest
    binary64), as stated.
- **WIT-ZERO-PIVOT.** The eigenvalues are 2 − 2cos((2j − 1)π/7), confirmed
  to ≤ 10⁻⁴⁴. At 2 ± 2⁻³⁰, max|L| = 2³⁰.
- **WIT-FE-CANT4.** K equals my four-element cantilever K, and M equals my
  consistent M times 420/h. Its eigenvalues equal Ω²h/420 to ≤ 10⁻⁴⁰.
- **Binary64 observations.** My own implementation of the file's stated
  operation order reproduces all 212 recorded observations: count, max|L|, g
  and pivot-guard flag, or breakdown. It also reproduces the 24 listed
  non-round-2 miscounts exactly.
  - In the scale witnesses, miscounts occur at 10⁻¹³ offsets both with and
    without the diagonal scaling, as the README says.
  - These observations depend on the operation order and are not
    references; the exact counts are.

### WIT-ROUND2: not refuted (23 checks)

- **Construction.** I rebuilt it independently:
  - K₀₁ = fl(−0.45) − 38·2⁻⁴⁰ is exact and binary64-representable;
  - M_i = round(2¹⁶·2^(−i/8))/2¹⁶ for all 32 entries.
- **Exact count at σ.** It is **1** by both routes: an exact Sturm recurrence,
  and Descartes on the exact degree-32 characteristic polynomial, which gives
  (negative, zero) = (1, 0).
- **Eigenvalues.** λ₂ agrees to 50 digits, and the neighbours to 28 digits.
  σ is the nearest binary64 to λ₂, at (σ − λ₂)/λ₂ = −6.993·10⁻¹⁹.
- **My binary64 LDLᵀ counts 2** in all three orders:

  | Form | Count | Last pivot |
  |---|---|---|
  | b²/d | 2 | −2.48·10⁻¹⁰ |
  | l·b | 2 | −3.24·10⁻¹⁰ |
  | l²·d | 2 | −1.36·10⁻¹⁰ |

  In each, max|L| = 4.35597 and g = 22.5809, and the 1e-12 guard is not
  tripped.
- **Separated probes.** At λ(1 ± 10⁻⁹) the counts are right, and max|L| and
  g agree with their values at σ to within 6·10⁻⁸.
- **Rounding away.** 2λ − σ rounds to σ, and its exact count is 2.
- **Search.** My own re-run of the stated search criterion over kk = 0…38,
  trying modes 2 and 3, first hits (kk, mode) = (38, 2).

### README premises against the design (item 8): consistent, except D-2

- **Consistent with §2.2 and D-e:**
  - m̄ = metal + contents + insulation;
  - j̄ = ρ_p·J + ρ_ins·J_ins, with contents excluded, and ī = j̄/2;
  - J = 2I on the metal, which equals the design's annulus
    J = A(r_o² + r_i²)/2;
  - J_ins as the insulation annulus;
  - contents in the axial mass;
  - insulation adds mass and inertia but no stiffness.
- **Rayleigh target.** The product converging to the Rayleigh beam matches
  §4.2 (review round 3, finding 17).
- **Wording (not a refutation).** "with no mill tolerance or corrosion
  allowance (c_m = 0)" names only c_m. The corrosion allowance is c_a (EX,
  §2.1). With c_m = c_a = 0, the PF and EX mass bases coincide (metal on t,
  contents on the bore from t), so the sections are route-neutral. Writing
  "c_m = c_a = 0" would say so.

## Discrepancies

- **D-1. Lumped-control rate (refuted claim; values correct).**
  - **Where:** the README (`fe_convergence.json` bullets) says "The lumped
    control converges at O(h²) from below here". `fe_convergence.json`
    `expected.lumped_beam_control` says "frequency error O(h^2)".
  - **What the data show:** the package's own data and my rebuild give
    mode-1 orders of 4.5725 (N = 2 → 4), 4.1547 (4 → 8) and 4.0395 (8 → 16),
    with errors −7.26·10⁻³, −3.05·10⁻⁴, −1.71·10⁻⁵ and −1.04·10⁻⁶. The other
    modes' orders are 4.15 to 5.11. So this simply supported control
    converges at O(h⁴), from below.
  - **Context:** a lumped cantilever, my own run and not in the package,
    shows orders of 1.97, 1.99 and 2.00. The generic O(h²) statement of
    design §2.2 therefore holds; it is the claim about this case that fails.
  - **Likely cause, not proven here:** Hermitian elements are nodally exact
    for static loads. The lumped problem is then a trapezoidal quadrature of
    the exact flexibility, and its O(h²) end term vanishes where the mode
    shape is zero at both supports.
  - **Consequence:** a product test that asserts order ≈ 2 for this control
    would fail against these data. The rate statement needs correcting by
    the author. A cantilever control could be added if an O(h²) witness is
    wanted.
- **D-2. Two README statements contradict the design at 85622cfe1 (refuted).**
  - **Statement (a).** The Limits bullet says the ring convergence target
    "depends on whether the product's half-arc rigid lumps carry the
    section's rotary inertia (§2.4 says 'second moment' without saying
    which)". But §2.4 now states
    J_n = ∫m̄(|ρ|²I − ρρᵀ)ds + ∫T·diag(j̄, ī, ī)·Tᵀds, "the centreline second
    moment plus the section's own rotary inertia". §4.2 adds: "the matching
    ring references include rotary and torsional inertia; Hoppe's and Love's
    formulas neglect both and are slender-limit controls". So the design's
    target is the refined variants (`inplane_extensible_rotary`,
    `outofplane_torsion_rotary`, `complete_low_spectrum`), not a choice still
    open.
  - **Statement (b).** The derivation says Love's premise of neglected
    torsional and rotary inertia "is not stated in the design". §4.2 now
    states it.
  - **Values:** the ring values themselves are unaffected.
- **Numerical discrepancies.** None beyond publication rounding. The largest
  relative differences are:
  - 4.7·10⁻⁴⁰ for 40-digit fields (tip mass μ = 1/2, β_L of mode 4);
  - 4.3·10⁻³⁶ for 36-digit FE Ω²;
  - 3.8·10⁻⁴ relative for 4-digit `observed_order` fields, within their
    rounding.

## Note on `generate.py --check`

`generate.py` regenerates `SHA256SUMS` from README.md, the generators and the
three JSON files. It reports any other file in the directory as unexpected.
Once this file and its hash line are present, `--check` therefore reports
MISMATCH (`SHA256SUMS` and "unexpected REFUTATION.md"), and `--write` would
drop the line. The cause is the driver's file list, not the references. Adding
`REFUTATION.md` to the driver's `AUTHORED` list would fix it; that change is
the author's to make. `shasum -a 256 -c SHA256SUMS` passes with this file
included.

## Not checked here, and decisions not taken

- **FE meshes not recomputed:**
  - Euler–Bernoulli simply supported and fixed-fixed at N = 16;
  - Rayleigh L = 6 m at N = 16, and L = 1 m at N = 8 and 16;
  - the tip mass at N = 8 and 16 except μ = 1.

  The published brackets for N ≥ 8 were not re-counted, except for the rods.
- **Witness observations not compared:** min and max |d| in the witness
  observations.
- **Decisions not taken:** product comparison tolerances, the warning factor
  c, the growth and separation parameters (O-6), and any change to the
  design. The `status` strings in the JSON files are the author's to update.
