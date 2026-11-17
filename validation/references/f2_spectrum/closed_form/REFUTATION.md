# Refutation of the F2 closed-form spectrum references

The refuting TASK's record for `validation/references/f2_spectrum/closed_form`. It answers
DEL-04-10 REQ-008 and VER-006: a second agent that reads no product code tries to refute each
reference package before any product comparison.

- **Date:** 2026-11-12.
- **Basis:** the package as listed in `SHA256SUMS` before this file was added (SHA-256 of
  `SHA256SUMS`: `b7a98b2bd875eae80415ced861b10cb6cfdcbc1e8098e400e5ca83930dbf54e0`). The governing
  scopes are DEL-04-10 and DEL-04-09 at commit bb5c0dfe2.
- **Result:** 68,041 independent value checks were run, plus 44,004 number-format checks. **No
  published value is refuted.** One descriptive statement is refuted: the README's list of the
  modes in normalization case (b). Two further statements need correcting before anyone relies on
  them:
  - the labelling of the repeated-pair rule values, under the repeated-mode rule the scope is
    adopting;
  - the missing limit on skew case (b) at the verification tolerance.

  See [Discrepancies](#discrepancies).

## Independence

- No product code was read, imported or run: nothing under `core/`, `apps/`, `api/`, `tests/`,
  `schemas/`, `fixtures/` or `tools/`. No CAEPIPE material was read.
- **What I read:** `AGENTS.md`, the DEL-04-10 and DEL-04-09 scopes,
  `validation/references/f1_modal/continuum/REFUTATION.md` (for the form of this record), this
  package's README and JSON files, and, as the brief permits, the sibling file
  `discrete/c8_pipe_x.json`. I read that file only after forming my own values.
- **Generators.** I did not reuse, import or run the package's generator code as a check. The only
  runs were the `--check` integrity run that the brief asks for and an AST import scan. I read
  generator source only after an independent disagreement, to locate its cause. That happened once,
  for the torsion frequency in the merged list. The cause was rounding in my own printing; see
  [Merged frequency lists](#merged-frequency-lists-of-the-benchmark-model-not-refuted-176-checks).
- **My checks.** They are my own standard-library Python, run with `python3 -I` from a scratch
  directory outside the repository, one process at a time.
- **Formulas.** Each was derived from CLM-001, DEL-04-09 REQ-001 and REQ-005, and standard
  structural dynamics:
  - Hamilton's principle for the Rayleigh beam and its end conditions, including the tip-spring
    condition;
  - Przemieniecki's Hermitian element matrices, re-integrated exactly from the shape functions;
  - Der Kiureghian's ρ as CLM-001 prints it;
  - my own derivation of the rigid-response recovery (K_e u − F_e and the modal identity).
- **Numerics.** These differ deliberately from the generator's methods:
  - π by the Gauss–Legendre (AGM) iteration, not Machin's formula;
  - sin and cos by Taylor series after ten argument halvings;
  - `decimal` at 70 digits;
  - continuum roots from reduced 2×2 boundary determinants (clamped basis
    cosh − cos, sinh − (a/b) sin; pinned basis sinh, sin), not 4×4 cofactors, by Illinois regula
    falsi or bisection;
  - every beam integral by my own Gauss–Legendre quadrature (40 nodes × 8 panels), not closed-form
    antiderivatives;
  - the true maximum of each shape located by a scan and bisection on W′ = 0;
  - FE eigenvalues by bisection on banded LDLᵀ Sturm counts, and eigenvectors by inverse iteration
    in `decimal`, not Cholesky and Jacobi;
  - the FE rigid response by a literal dense static solve.

## Package integrity: not refuted (3 checks)

- **Regeneration.** Before this file was added, `python3 -I -B generators/generate.py --check`, run
  from the package directory, reported "cross-checks: 40 recorded, 0 disagreeing" and "verified 25
  files byte for byte (SHA256SUMS included)".
- **Hashes.** `shasum -a 256 -c SHA256SUMS` passed for all 24 listed files.
- **Imports.** An AST scan of all 14 generator modules finds only standard-library imports
  (`argparse`, `decimal`, `fractions`, `hashlib`, `json`, `math`, `pathlib`, `sys`, `time`) and the
  package's own sibling modules. It finds no `__import__`, `exec`, `eval`, `compile` or `importlib`.
  The only path manipulation is `sys.path.insert` of the generators' own directory.
- **Note on `--check`.** As in F1, `generate.py` reports any file it does not generate as
  unexpected. With this file present, `--check` therefore reports "MISMATCH: unexpected
  REFUTATION.md". The cause is the driver's file list (`AUTHORED`), not the references. Adding this
  file there is the author's change to make. `shasum -a 256 -c SHA256SUMS` is unaffected.

## Number formats (44,004 checks): not refuted

- **Inputs.** All 889 `{binary64, hex, exact}` triples agree, and each `binary64` is the shortest
  round-trip form.
- **Derived pairs.** For the 18,364 `{value, binary64}` pairs and the 24,320 FE triplets, the
  binary64 is the nearest one to the published value, with five exceptions that are not errors:
  - In five places (`directional.json` D1–D3 `full_y`/`full_x` and D7 `ABSSUM_not_in_scope`, and
    `spectrum_tables.json` T2, linear/linear, row 5 `sS`), the exact rational lies exactly halfway
    between two binary64 numbers.
  - The published `binary64` is that rational rounded half to even, which is correct.
  - The 40-digit `value` string, read back as binary64, falls on the other neighbour.
  - A reader that derives the binary64 from `value` instead of `exact` would be one ulp off there.
- **Exact rationals.** All 431 `{exact, value}` pairs agree to within the 40-digit rounding.

## Results by group

Each group gives the number of checks and the largest relative disagreement. A disagreement below
5·10⁻⁴⁰ is the 40-digit publication rounding.

### Section: not refuted (12 checks)

- I derived the exact ratios EI/m̄, ī/m̄ and EA/m̄ from the binary64 inputs, with π factored out,
  and they are equal as rationals. GJ/j̄ = G/ρ_p = 1560000000/157.
- m̄, EI and ī agree to ≤ 2.2·10⁻⁴⁰.
- ī = ρ_p I excludes the contents (DEL-04-09 REQ-001), and m̄ includes them. That matches the
  scope's mass basis with c_m = 0 and no insulation.

### Continuum beams, four spans × two lengths × Euler–Bernoulli and Rayleigh: not refuted (13,716 checks; largest 4.9·10⁻⁴⁰)

- **Derivation.** The free-end conditions W″ = 0 and W‴ + r²Ω²W′ = 0 and the end forces
  F_L = −(EIw‴ + ω²īw′) and M_L = EIw″ come from my own variational derivation.
- **Frequencies.** Ω, ω and f of all 80 modes agree to ≤ 4.7·10⁻⁴⁰. So do a, b, the normalizing
  point and W at L/4, L/2, 3L/4 and L.
- **Modal quantities.** Γ (including its sign under the package's tie rule), the effective-mass
  fractions and the cumulative fraction over five modes all agree. So do ∫W, ∫W² and ∫W′², by
  quadrature, and the per-unit tip or midspan displacement, F₀, F_L, M₀, M(L/4), M(L/2), V(L/4)
  and V(L/2).
- **Equilibrium.** My own force balance F₀ + F_L + Γm̄L∫W holds to below 10⁻⁵⁰ in every mode.
- **Spectral ordinates and contributions.** All 160 ordinates (FLAT and the TWO_SEGMENT spectra,
  log-log) and all 1,120 signed contributions agree.
- **Combinations.** All 1,120 combined values over five modes agree under SRSS, CQC, grouping, the
  absolute sum and the pairwise ten percent rule. I used ρ at the binary64 damping 0.05.
- **Static coefficients.** All agree exactly with the textbook values, including qL⁴/192EI at the
  propped midspan.
- **Missing mass.** 9,344 checks agree: f_r, the periodic classification, a_r, the disclosure
  fractions, the signed rigid part and all twelve totals per quantity and truncation.
  - The rigid part is a_r(Q_static − Σ_periodic Γ_n q_n/ω_n²).
  - I derived this identity myself from F = a_r(MR − ΣΓMφ) with recovery K_e u − F_e.
  - The literal FE static solve below converges to it, which is independent support.
- **Rigid frequencies.** Each f_r equals 1.01 f_n of the Rayleigh continuum to 6 significant digits.
- **Rotary-inertia shifts.** All 40 agree. At L = 2 m the shifts of modes 1–3 run from 2.2·10⁻⁵ to
  4.7·10⁻⁴. At L = 64 m the largest shift among modes 1–3 is 4.59·10⁻⁷ (fixed-fixed mode 3), as
  stated.

### Finite elements, N = 2, 4, 8, 16, consistent and lumped, all eight realizations: not refuted (49,950 checks; largest 4.9·10⁻⁴⁰)

- **Element matrices.** The printed stiffness, consistent mass and rotary inertia equal my exact
  Hermite integrals.
- **Mode counts.** Every finite-mode count agrees (lumped: the free translations only).
- **Modal quantities.** All 262 frequencies, Γ and effective-mass fractions agree to
  ≤ 4.2·10⁻⁴⁰. The fractions are taken relative to T = m̄L, as DEL-04-09 REQ-005 defines them, with
  p = (MR)_f including the coupling to restrained DOFs.
- **Normalization.** Every normalization case and every equal-maxima flag agrees.
- **Responses.** All 3,520 per-mode responses agree, recovered as (K_e − ω²M_e)u, left element at
  each cut. So do all 4,160 combined values and 16,640 missing-mass values: the literal static solve
  with K_e u − M_e F_e and the reactions assembled at the supports, the periodic classification and
  its "differs" flag, and all four totals.
- **Deviations.** All 24,844 published deviations (`rel_dev`) agree to their 6 digits. The only
  apparent differences are deviations that are zero to rounding, such as 10⁻⁴⁰ against 10⁻⁵⁰.

### Screening summary: not refuted (504 checks)

- I rebuilt `largest_relative_deviation` from my own deviations, not from the published `rel_dev`
  fields. I used the stated definitions:
  - the four kinds;
  - "significant" as at least 1 % of the continuum SRSS over five modes;
  - combined values over the first five (or all finite) modes, against the continuum over the same
    modes;
  - missing-mass totals with and without the same classification;
  - both lengths and both spectra, with the L2 and L64 splits.
- All 504 entries agree to their 3 digits. Each `where` names the same realization, quantity and
  mode or rule, except where two candidates tie exactly. For example, with two modes ρ ≈ 0 makes
  SRSS and CQC equal.
- **Is "largest" the largest?** Yes, over the stated population. That population excludes, without
  saying so, two kinds of quantity:
  - the quarter-point quantities at N = 2, where L/4 is not a node;
  - quantities whose continuum value is zero by symmetry.

  The definitions should say so.

### Sensitivity: not refuted (1,208 checks)

- **Values.** I checked all values of σ_n = d ln S/d ln f, d ln Q_n/d ln f_n (σ_n − 2 for
  displacements, σ_n otherwise) and d ln R_SRSS/d ln f_n = (Q_n/R)²·d ln Q_n/d ln f_n. The largest
  absolute difference is 4.3·10⁻⁴⁰. Modes marked "no contribution" have zero Γ.
- **Largest magnitudes.** All eight agree: 1.99997 (displacement, FLAT), 1.24991 (displacement,
  sloped), and 0.7449, 0.7455 and 0.7436 for force, moment and reaction on the sloped spectrum.
- **Tie.** For the moment, S-L64 M(L/2) and M(L/4) tie exactly. Every contributing mode has the
  ratio ±1/√2 between the two, so either is "the" largest.

### Merged frequency lists of the benchmark model: not refuted (176 checks)

- **Bending entries.** All 174 entries, at both lengths, for the continuum (both models) and the
  eight FE meshes, equal the per-plane frequencies verified above. They are in ascending order.
- **List rule and cutoffs.** These are consistent with my own computation.
- **Torsion entry.** The one torsion entry (L = 2 m, consistent N = 4, 396.56 Hz) agrees to
  1.1·10⁻⁴⁰ with my own Sturm bisection of the assembled 4-element rod. The rod closed form
  6N²(1 − cos θ)/(2 + cos θ) also matches it. I confirmed the rod closed forms at N = 1 by hand.
  - An apparent 5.7·10⁻¹⁷ disagreement here came from my own `%e` formatting, which passed my
    value through binary64. It is not a package error.
- **No other axial or torsion entries.** None falls below a cutoff: at N = 2 the consistent cutoff
  is 271.7 Hz, and the N = 8 and N = 16 torsion modes lie above 342.7 and 339.2 Hz.
- **Y–Z pairs.** The smallest Y–Z gap anywhere is 1.31 %, so "no pair within 1 %" holds.

### Cross-read with `discrete/c8_pipe_x.json`: no disagreement (110 checks)

- **The models differ.** The sibling's C8 is not numerically the same model as this package's
  benchmark model. It has OD 60.3 mm, wall 3.91 mm, E = 200 GPa, no contents and L = 15 m. This
  package uses OD 21.3 mm, E = 203 GPa, water contents and L = 2 or 64 m. So no published number
  overlaps directly.
- **What I compared.** I rebuilt the sibling's model with my own FE, planar decomposition,
  consistent mass and Rayleigh rotary inertia. I then compared:
  - its six frequencies at N = 8 and in all four mesh-series meshes;
  - the C8-I-FLAT CQC magnitudes of B.uy, M4.uy, SA.Fy, SA.Mz, M4.uz, SA.Fz, SA.My and SB.Fz;
  - the SRSS, CQC and grouping series values of six of those quantities at N = 2, 4, 8 and 16.
- **Result.** All agree to ≤ 3.8·10⁻⁴⁰. Neither package is shown wrong by the other.

### Spectrum tables: not refuted (524 checks)

- **Evaluations.** All T1 and T2 evaluations under the four rules agree, at tabulated, interior and
  end points, including:
  - the period query T = 1/f;
  - the g-multiple ordinate times g = 9.80665;
  - the scale factor s = 1.5;
  - both A1 readings for T2's linear-abscissa rules, with the alternative reading interpolated in
    f after converting the table.
- **Exact rationals.** Every exact rational agrees exactly, and every `exact` flag is right.
- **Refusals.** All 20 points outside the range are outside it, and each is marked refused.

### Single degree of freedom: not refuted (112 checks)

- **Derived values.** f = 20/π and T = π/20 agree. For all ten cases, S, sS, u = sS/1600, ku, −ku
  and the three magnitudes agree.
- **The neglected velocity term.** The damper force 2ζku agrees, as does its ratio 2ζ, with ζ the
  binary64 0.05. So does the harmonic peak √(1 + 4ζ²)·ku.

### Two-mass chain: not refuted (414 checks)

- **Modes.** All agree: the eigenvalues from the quadratic, and the shapes from the first row,
  where the package uses the second row. So do the generalized masses, Γ, the effective masses and
  fractions, the per-unit quantities, the range flags, the groups, the pairs and ρ₁₂ = 0.7139 for
  set B.
- **Responses.** All T1 log-log ordinates with s = 1.25, the signed contributions and all five rules
  agree.
- **Set B signs.**
  - u₁, spring 1 and the ground reaction have same-sign pairs, and CQC > SRSS.
  - u₂ and spring 2 have opposite-sign pairs, and CQC < SRSS.
  - The README names u₁, the ground reaction and spring 2. u₂ behaves like spring 2 but is not named
    there, which is not an error.
- **Missing mass.**
  - Set A at 7 Hz: the static load, a static solve that I wrote by hand, the rigid part and the
    totals all agree.
  - Set A at 12 Hz: the rigid part is exactly zero.
  - Set C at 50 Hz: the response equals the hand static solution u₁ = a_r(m₁ + m₂)/k₁.
- **Set C refusal.** Set C without the correction is correctly marked refused, with both modes
  outside T1.

### Directional combination: not refuted (89 checks)

- **Values.** Every exact square of the SRSS, every 100-40-40 permutation and maximum, every
  governing set and every absolute sum agrees exactly.
- **The 40 percent.** Using the binary64 0.4 instead of 2/5 changes 100-40-40 by at most
  2.5·10⁻¹⁷ relative.

### Tip-spring pair and repeated pair: values not refuted (1,238 checks); see D-2 and D-3

- **Frequency equation.** The plane-p equation comes from my own 2×2 determinant with
  W‴ + r²Ω²W′ − κW = 0.
- **Modes and responses.** For cases (a) and (b), under both models, all of the following agree:
  - the six frequencies and the split of 4.995 %;
  - k_eff, κ, c_p, c_q and c_axial;
  - the plane quantities, the global normalizing component and its tie flag, Γ along the excitation
    and the effective masses;
  - all twelve global per-unit quantities, the ordinates, all signed contributions, all five rules
    and the first-pair sign patterns.
- **Equilibrium.** My own global force equilibrium (clamp + spring + inertia) holds below 10⁻⁴⁵ in
  every mode. In case (b) the clamp carries the spring's axial component.
- **Neglected axial coupling.** I derived its size myself by static condensation of the tip's axial
  DOF:
  - δ = 1.741298·10⁻⁶;
  - the first-order frequency effects −8.0614·10⁻⁸, −2.281·10⁻⁹ and −2.906·10⁻¹⁰;
  - 3I/(AL²) = 3.29094·10⁻⁵;
  - the first axial frequency, 591.698 Hz;
  - the static axial response.
- **Repeated pair.** A, f, and every contribution, rule value and invariant value at the eight
  angles agree. At θ = 90°, closed-form zeros are published as values of order 10⁻⁹¹; see the note
  on zeros under Discrepancies.

## Discrepancies

- **D-1. Normalization case (b): refuted statement; the values are correct.**
  - **Where:** README, "Mode normalization", the sentence "Only the fixed-fixed N = 2 consistent
    mode 2 is in case (b)."
  - **What the data show:** `beams_fe.json` itself, and my recomputation, which agrees with every
    published `normalization_case`, put eight mode entries in case (b):
    - fixed-fixed N = 2, consistent, mode 2;
    - pinned-pinned N = 2, consistent, modes 2 and 4;
    - pinned-pinned N = 4, consistent, mode 4;

    each at both L = 2 m and L = 64 m.
  - **Why:** these are antisymmetric modes whose translations vanish at every node.
    - At N = 2 the only free translation is at midspan.
    - The pinned-pinned N = 4 mode 4 resembles sin 4πx, which is zero at x = 0, ¼, ½, ¾ and 1.
    - Each has Γ = 0 and contributes nothing.
  - **Correct text:** "Case (b) applies to the consistent-mass modes whose nodal translations all
    vanish: fixed-fixed N = 2 mode 2, and pinned-pinned N = 2 modes 2 and 4 and N = 4 mode 4, at
    both lengths."
  - **Consequence:** a product test that takes its list of case (b) modes from the README would
    miss three of the four kinds.
- **D-2. Repeated pair under the repeated-mode rule the scope is adopting: the labels mislead; the
  values are correct as per-mode arithmetic.**
  - **The adopted rule:** the modes of a numerically repeated cluster are summed with their signs
    first, and every modal rule is applied to the cluster sums.
  - **What a product would publish:** applying that rule, a product gives uY = A and uZ = 0 for
    SRSS, CQC, grouping and every other rule, at every θ. That is the package's
    `basis_invariant_sum_first`.
  - **What the package shows instead:** the README table and the `repeated_pair.rows` fields
    labelled `SRSS`, `GROUPING`, `ABSSUM_not_in_scope` and `PAIRWISE10_not_in_scope` give the
    basis-dependent values that result from applying each rule to the individual modes.
  - **The same issue in the "Planar" paragraph:** it says repeated pairs make "per-mode
    contributions and SRSS non-unique". That holds only without the cluster step.
  - **Correction:** label these fields as the per-mode application, which is not the adopted
    method, and state that the expected product value for every rule is the invariant one. Do this
    in the same change that adopts the rule.
  - **What the rule's premise does not touch:**
    - The basis is M-orthogonal, so the premise holds.
    - Cases (a) and (b) are not clusters under DEL-04-09's δ_c = 2⁻²⁰. Their closest eigenvalue
      pair is 3·10⁻⁴ apart.
- **D-3. Skew case (b) cannot verify a finite-EA product at 1e-9: a missing limit; the values are
  correct for the stated idealization.**
  - **The idealization:** case (b) is axially rigid.
  - **The size of the error it leaves:**
    - The neglected coupling lowers the first plane-p frequency by 8.06·10⁻⁸ relative.
    - On T1's 1.5–6.25 Hz log-log segment the slope is σ = 0.7025. A displacement contribution
      therefore changes by about (2 − σ)·8.06·10⁻⁸ ≈ 1.0·10⁻⁷.
    - The coupled modes also acquire axial components, which participate through
      c_axial = 1/√3.
  - **Consequence:** a product model has a finite EA. Case (b) is therefore usable only at about the
    10⁻⁶ level, not at the 1e-9 verification tolerance.
  - **Case (a) is unaffected:** its spring is perpendicular to the pipe, so d·a = 0. Case (a) alone
    covers the AC-002 item "ground-spring reaction beside a rigid-end reaction".
  - **Correction:** the README (A9 and Limits) should state this.
- **Numerical discrepancies.** There are none beyond publication rounding. The largest relative
  differences are 4.9·10⁻⁴⁰ for 40-digit values and 6 digits for deviation diagnostics.

**Notes, not refutations.**

- **Zeros.** Quantities that are exactly zero in closed form are published sometimes as "0" and
  sometimes as rounding residue, from 10⁻⁶⁶ to 10⁻⁹¹:
  - antisymmetric continuum W and ∫W;
  - θ = 90° in the repeated pair.

  REQ-008's absolute floors must absorb these, as they must a product's own rounding residue.
- **"Negligible" at L = 64 m.** The label applies to modes 1–3, as both files say. The five-mode
  categories also use modes 4 and 5. Their rotary shifts at L = 64 m are 6.6–8.0·10⁻⁷ and
  1.06–1.25·10⁻⁶; the mode 5 shifts exceed 10⁻⁶ in every span.

## Ambiguities in the scope's method that the package resolved by convention

These should be fixed in the scope's words. Not all are mine to propose; each is listed with how
the package resolved it.

1. **Period tables with a linear abscissa (A1).** Interpolation can be in T or in 1/T. The package
   uses tabulated T as primary and publishes both. REQ-001 does not say which.
2. **Mapping a query frequency onto a period table at the range ends (A2).** In binary64, 1/f can
   fall outside the range and be refused. REQ-001 does not say how the query is mapped.
3. **Sign when equal largest components have opposite signs (A3).** DEL-04-09 REQ-005 says "+1"
   without a tie rule. The package takes the smallest x or the first global component. This fixes
   the sign of Γ and of the shape; no response depends on it.
4. **100-40-40 with fewer than three directions (A4).** The package reads the absent direction as 0.
5. **"0.4" (A5).** The package uses 2/5 exactly, which is immaterial at 1e-9.
6. **Rigid modes outside the tabulated range (A6).** The package admits extracted modes at or above
   f_r. REQ-001 names only periodic modes and f_r, so this is implied but not stated.
7. **The grouping boundary.** CLM-001's "within 10 %" does not say whether equality is included. The
   package includes it, and no published case lies on the boundary.
8. **The repeated-cluster rule (D-2).** This is being adopted. Its wording should name the cluster
   tolerance it uses, which is DEL-04-09's δ_c.

Two further conventions belong to the screening method rather than to DEL-04-10's text, and the
screening-class derivation should state them:

- **(A7)** The sensitivity reading holds one mode's frequency varying alone, with its shape and
  participation fixed, and covers SRSS only.
- **(A8)** The same f_r can classify a different number of modes as periodic on a coarse mesh.

## Not checked

- The `finite_difference` strings in `sensitivity.json`. The closed-form derivatives were checked.
- The `c_cosh_sinh_cos_sin` coefficients of the continuum shapes. They depend on normalization;
  the shapes were checked through W at five points, the integrals and every response.
- The cross-check residual fields (`max`) inside each JSON file.
- The repeated pair's `extremes` block was compared by inspection only: A and A/√2.
- The `cutoff_hz` fields were checked only through the last entry of each list.
- Decisions not taken: any screening class value, tolerance, scope wording or repair of the
  package. The `status` strings and the `AUTHORED` list are the author's to update.

## Additions after the first record

A further refuting TASK, not the author of this package and not the writer of the record above,
examined what was added to or changed in this package after that record (the renamed rule keys, the repeated pair's adopted and mode-by-mode values, and the screening basis under its restated definitions), as committed on
`claude/dynamics-references` at c8c602df6. This section is its record: one record covered this package and
`../discrete` together, the rows below are this package's, and its count of checks is for both. Governing texts: DEL-04-10
CLM-001 (with Repeated modes), REQ-002 to REQ-004; DEL-04-09 REQ-002, REQ-003 (δ_c = 2⁻²⁰), REQ-005.

**Verdict.** Nothing refuted: no value, formula or statement checked below disagrees.

**Independence.** The conditions are those of the benchmark-pack record (`../benchmark_pack/REFUTATION.md`). No product code was read, imported or run,
and no CAEPIPE material was read. The scripts use the standard library only, are my own, run with
`python3 -I`, one process at a time, in a scratch directory outside the repository, and import no
package generator. Every check starts from published inputs, contributions or eigenvalues, or from
first principles (B6's A), with rules written from CLM-001's text.

| Item | What | Checks | Largest disagreement |
|---|---|---|---|
| B5 closed_form | `beam_skew_and_pairs.json` sets a and b, EB and Rayleigh, six modes, every quantity; `two_mass_chain.json` sets A and B (C is a stated refusal); `directional.json` ABSSUM = R_x + R_y + R_z exactly, D1–D7 | 240 + 50 + 7 | 1.9e-39 |
| B6 repeated pair | A rebuilt from first principles: EI/m̄ as an exact rational (equal to the published one), cantilever β₁ from my own determinant, W(1) = 1, ∫W/∫W², T1 log-log at f₁; then, at all eight angles, c₁ = cos²θ A, c₂ = sin²θ A (uY) and ±cos θ sin θ A (uZ); `adopted_cluster_sum_first` per rule = |c₁ + c₂| = A and 0; `basis_invariant_sum_first`; the mode-by-mode fields against the closed forms named for them | 2 + 224 | 2.3e-40 (A to 5e-41) |
| B6 screening | `per_mode` and `per_mode_significant` maxima for displacement and reaction rebuilt from `beams_fe.json` and `beams_continuum.json`, with the 1e-30 exclusion and the 1 % threshold over nm = min(5, finite FE modes) continuum modes, every form and N, `all`, `L2` and `L64`; `largest_sensitivity` for the same two kinds rebuilt as (Q_n/R)²(σ_n − 2) and (Q_n/R)²σ_n with σ_n from each table's log-log slope, so shape, participation and q_n fixed | 96 + 4 | every published 3-digit value equal; where the `where` label differs (15 cases) the two candidates tie to ≤ 1e-37 |

About 11,000 checks in all. The 348 contributions excluded by the 1e-30 rule (displacement and reaction) are all exact zeros.

Notes (not refutations):

- B6 screening: 15 `where` labels name one of several exactly tied maxima, such as FLAT against
  TWO_SEGMENT on a flat segment, or F0 against FL on a symmetric span. The tie-break is not stated,
  but the values are unaffected.

Not checked:

- No model was re-solved for this section.
- This package's generators, and their regeneration, were not run for this section.
