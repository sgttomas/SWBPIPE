# Refutation record: f2_spectrum discrete frame references

This is the refuting TASK's record for the candidate references in this
directory: DEL-04-10, CLM-001, REQ-002 to REQ-005, REQ-008, AC-002 and AC-003.
It covers `README.md`, `c1_one_element.json` to `c10_cluster_witnesses.json`
and `generators/`, as found uncommitted on `claude/dynamics` at bb5c0dfe2. The
SHA256SUMS lines are listed under Determinism below.

The governing texts are:
- DEL-04-10 CLM-001 and REQ-001 to REQ-005;
- DEL-04-09 REQ-001 (mass), REQ-002 and REQ-003 (clusters, δ_c = 2⁻²⁰),
  REQ-005 (normalization and participation) and REQ-006 (skew and snubbers);
- DEL-04-03 REQ-010 (skew rigid restraints and skew springs).

This record makes no acceptance statement and sets no comparison tolerance.

**Verdict.** No published value is refuted. Nine descriptive statements are,
R1 to R9 below; R4 and R5 bear on how the benchmark cases are compared.

## Independence

- The refuting scripts read, imported and ran no product code, and no CAEPIPE
  material was read.
- They import nothing from this package's generators or from
  `f1_modal/mass_discrete/generators`. The generator was read only for its
  imports, its pins and its SHA256SUMS function, and run only for the
  regeneration check.
- The sibling `../closed_form/` was not read.
- The scripts use the Python standard library only (`decimal` at 70 digits,
  `fractions`) and run with `python3 -I`, one process at a time, from
  `/tmp/refute_f2`. That directory holds no instructions.
- The scripts were written from the scopes and textbook frame dynamics:
  - π by the Gauss–Legendre AGM, not Machin.
  - PF section on the wall t − c_m, with A, I and J = 2I. m̄ is metal plus
    contents on the bore; j̄ = ρ_m·J and ī = j̄/2.
  - The 12×12 Euler–Bernoulli stiffness. The x–z plane acts on (w, −θ_y).
  - Consistent mass: linear axial and torsional terms, Hermitian translation
    and Rayleigh rotary inertia ī/(30L)·[36, 3L, …].
  - A Gram–Schmidt member frame from `y_reference`, assembled into global
    axes.
  - Axis restraints are removed. Springs add k on the diagonal; skew springs
    and finite snubbers add k·d dᵀ/(dᵀd).
  - A rigid skew snubber is an exact constraint, through my own d⊥ basis:
    d × e_a with a the largest |d_a|, then d × that vector. This differs from
    the package's choice and leaves physical results unchanged.
- **Eigen route:** Cholesky of M_r, then Householder tridiagonalization and
  implicit QL on L⁻¹K_rL⁻ᵀ. This is not the package's route (Cholesky of K,
  then Jacobi).
- **Response and recovery**, written from CLM-001:
  - The modal response is Γ·s·S(f_i)/ω_i²·φ_i, with S read off the table's own
    abscissa (period for `S-PLG`).
  - End forces are (K_e − ω²M_e)u_e in global axes, on the element.
  - Reactions are (K_tot u − Mg) at rigid restraints and at a rigid skew
    node's translations, and −K_s·u at compliant supports.
- **Rigid response:** a static solve of F = a_r(M R_k − Σ_periodic Γ M φ)
  on the dynamic stiffness. End forces are K_e u − F_e.
- **Combinations:** SRSS, CQC with ρ from the units' lowest ω, GROUPING
  (≤ 10 % of the group's lowest), ABSOLUTE_SUM and PAIRWISE_TEN_PERCENT; the
  rigid–periodic rules; directional SRSS, 100-40-40 and ABSOLUTE_SUM. All are
  applied to δ_c-cluster sums, each mode keeping its own f_i.

## Results

72,820 published reals were compared one by one with the scripts' own values.
Each published decimal (40 digits) was compared, and so was its binary64
partner. Structural items were checked too:
- mode status, normalization case and normalizing DOF;
- grouping groups, CQC units and `in_scope` flags;
- plane labels and `scope_cluster` ids;
- the bounding-box diagonal and the reduced DOF counts.

Every structural item agrees. No binary64 partner disagrees with its decimal.
The only differences in keys are components the package leaves out because
they are identically zero (R6), and the quantities C9-CHAIN omits on purpose.

| Group (all cases) | Number | Largest disagreement |
|---|---|---|
| T_k, eigenvalues, ω, f, period, generalized mass, Γ, shapes, m_eff, fractions, cumulative fractions | ~2,400 | 4.7e-40 relative |
| Spectral ordinates (LL, GG, PLG, GL, FLAT, C8 and C9 tables), scaled ordinates, a_r | ~800 | 2.3e-40 |
| Signed per-mode contributions: displacements, end forces, reactions, springs, skew Fd, u_along_d, u_perp | ~17,000 | 4.5e-40 of the unit scale |
| Signed rigid response and rigid magnitude, with extracted rigid modes | ~2,600 | 4.0e-40 |
| ρ (CQC) | ~1,100 | 2.2e-40 |
| Periodic combinations (5 rules), correction off, per direction, combined | ~19,000 | 4.9e-40 |
| Mutation grids (C3, C4, C8, C9, C9-CHAIN, C10) | ~27,000 | 4.8e-40 |
| Missing-mass disclosure (all 8 entries) | ~250 | 2.8e-40 |
| C8 mesh series N = 2, 4, 8, 16: frequencies, planes, every rule | ~200 | 3.3e-40 |
| `frequency_list`, `close_pair`, `chain`, `single_direction_check` | ~300 | 4.1e-40 |
| W1 `basis_witness` (four bases, Γ, m, M-cross products, sums), reading B SRSS range | ~1,300 | 2.7e-40 |
| W2 `cluster_witness` and `not_the_adopted_reading_mode_by_mode`; W3 `rigid_load_signed` | ~470 | 4.3e-40 |
| `derived_model.member_frames`; rigid total-reaction cross-checks | ~560 | 4.3e-40 |

Every disagreement is within the rounding of the published 40 digits.

Further checks, each independent of the package:
- **Global equilibrium of the published numbers.** For every published
  per-mode contribution (1,110 checks), the support forces and moments were
  set against −R_jᵀMg and −Θ_jᵀMg, with my own M. Worst: 9.0e-39 of the scale.
- **Rigid response.** The scripts' own total reaction along k equals
  −a_r,k(T_k − Σ_periodic m_eff) to 1e-50 in every corrected analysis, W3
  included.
- **C1 witnesses, against hand formulae.**
  - m = T/3, Γ = 3/2 and m_eff = 3T/4, with ω² = (EA/L)/(T/3).
  - A1.Fx = −(3/4)T·S equals −EA/L·u − ω²(T/6)u, so it carries −ω²M_cf u.
  - C1A-MM gives u = 0 and A1.Fx = −a_r·T/4.
  - C1B gives −a_r,k·T_k in X, Y and Z.
  All agree to the last published digit.
- **Reading B range.** The closed form [C²(tr A)²/2, C²‖A‖_F²] was
  confirmed by brute-force rotation of each pair at 720 angles.
- **C8 single direction.** Every quantity receives contributions from one
  direction only, in the modal and the rigid parts.
- **C6 rigid skew snubber.** `SN2.u_along_d` is 0 in every response,
  `SN2.Fz` is 0, and u_perp1 and u_perp2 are nonzero. The rigid response
  differs from C5B's, while C6B's stays close to it.
- **`inputs_binary64`.** All 339 entries agree in shortest decimal, hex and
  exact rational. C9's node coordinates are the correctly rounded binary64
  values of 15k/(8√3).

## Refuted statements

None changes a published number.

**R1. C9-Y, in the Cases table:** "Mode 2 is zero in B.ux, B.rx, SA.Fx and
SA.Mx".
- **Correct:** under Y, mode 1 is zero in B.ux and SA.Fx, and mode 2 is zero
  in B.rx and SA.Mx.
- The published contributions agree with the corrected statement:
  - mode 1: B.ux 0, B.rx −8.348e-2, SA.Fx 0, SA.Mx 786.5;
  - mode 2: B.ux −0.4744, B.rx 0, SA.Fx 37.99, SA.Mx 0.
- **Why:** mode 1's tip moves along (0, 1, −1)/√2, normal to X and to the
  diagonal, so it leaves the X spring unstrained. Its rotation axis is
  ∝ (−2, 1, 1), which has an x component. Mode 2 moves along (−2, 1, 1)/√6
  and rotates about ∝ (0, −1, 1), so its rx is zero.

**R2. "Pairs within 1 % in C9":** "Unextracted pairs from mode 32 up lie
inside δ_c".
- **Correct:** exactly four pairs lie inside δ_c: (25, 26), (27, 28),
  (30, 31) and (32, 33). The file's own `frequency_list.scope_cluster` shows
  this.
- The pairs above 33 do not. For (35, 36) the eigenvalue gap is
  1.03e-6 > 2⁻²⁰ = 9.54e-7.

**R3.** "In every case except C7, C1 and C10, every extracted mode is simple
and alone in its δ_c cluster."
- **Correct:** "except C1, C5A, C7 and C10".
- C5A-RIGID extracts modes 1 and 2, an exactly repeated pair at 594.77 Hz.
  The README itself names C5A among the exact repeats.
- No combined value depends on this: in C5A both modes are rigid.

**R4. Conventions item 2:** "A product working in member axes maps them with
`derived_model.member_frames`, a signed permutation, which maps the combined
magnitudes exactly."
- This is false for C9 and C9-CHAIN. Their frames are not signed
  permutations: the rows are (1,1,1)/√3, (−1,−1,2)/√6 and (1,−1,0)/√2.
- A combined magnitude in global axes cannot be turned into member-axis
  magnitudes. For these cases, a product working in member axes must rotate
  the signed per-mode contributions into global axes and combine there, or
  the references must publish member-axis contributions too.
- This matters for the cross-engine benchmark.

**R5. W2:** "A binary64 eigensolver rotates the two vectors of a pair this
close by about ε/gap ≈ 4e-10. So per-mode contributions of these modes can be
compared with a product only to about that level."
- This holds only for a formulation whose backward error scales with the
  pair's own eigenvalue. One example is Cholesky of K with the largest
  eigenvalues of R⁻¹MR⁻ᵀ, which is the route the references use.
- With Cholesky of M and the lowest eigenvalues of L⁻¹KL⁻ᵀ, the bound is
  ε·λ_max/(λ₂ − λ₁) ≈ 1.1e-16 × 2.62e7 / 9.09e-4 ≈ 3e-6.
- Measured with 16-digit decimal arithmetic as a binary64 proxy, mode 1
  rotates by 4.2e-6 on the M-Cholesky route and by 1.7e-9 on the K-Cholesky
  route.
- The sentence should state its premise and give a level that covers the
  usual formulations. Otherwise a reader may compare C10 per-mode values at
  4e-10, or at the 1e-9 verification tolerance, where a correct product can
  fail. The cluster sums are unaffected.

**R6. Conventions item 3:** "A skew support is published as its global
components, `.Fx/.Fy/.Fz`, plus `.Fd`."
- For finite skew springs and snubbers, the components where d has a zero
  entry are left out: K1 and K2 have no `.Fz` in C4, SN2 has no `.Fz` in C6B,
  and K1 has no `.Fx` in C10.
- The rigid skew SN2 in C6A does publish `.Fz` = 0.
- Either state the omission or publish all three components consistently.

**R7. Smallest-gap table, C9-CHAIN:** 0.028.
- This value is relative to the upper frequency:
  (52.663 − 51.172)/52.663 = 0.0283.
- Every other entry is relative to the lower one. For example, C8's 0.24 is
  (4.4047 − 3.5665)/3.5665; relative to the upper frequency it would be 0.19.
- On the table's own convention the entry is **0.029**.

**R8. `modal_basis.clusters`.** These entries are exact-repeat groups
(1e-30), not the scope's δ_c clusters.
- In C10, modes 1 and 2 appear as separate entries with multiplicity 1,
  although they share one δ_c cluster (`scope_cluster` 0).
- A reader who takes `clusters` for the scope's clusters will combine C10
  mode by mode, which is the reading the package labels "not adopted".
- Rename the key, or say this in the README.

**R9.** "relative eigenvalue gaps of 2.53e-7 (≈ 2⁻²²)": 2⁻²² = 2.38e-7.
Write "2.53e-7" alone. This is cosmetic.

Every other descriptive statement checked holds. That covers:
- the Cases table: frequencies, ρ values, same and opposite signs, CQC
  against SRSS for C4 and C9, statuses, the C8 tables and f_r placements,
  the chain modes and C5A's 594.8 Hz;
- the claim that GROUPING equals PAIRWISE_TEN_PERCENT in C3, C4, C8 and C9,
  and differs in C9-CHAIN in exactly the 8 listed quantities;
- W1: 68.68 and 107.31 kg, v1ᵀMv2 = 39.0 kg, the 0.33 difference, and the
  isotropy. M-orthogonality after the Euclidean rotation needs only equal
  generalized masses;
- the W2 table: 4.522e-3 / 3.658e-3, 2.261e-3 / 3.489e-3, 884.6 / 1365.4 and
  5085.9 / 4114.2;
- W2's CQC at 3e-11 and the −4.43e-10 m sum;
- W3's classification and equilibrium;
- the C7 reading B extremes, 3.197e-3 m and 1251 N.

## Conventions the scope should fix in words

Each of the README's conventions is stated so that a reader can apply it.
Except item 2 (R4) and item 3 (R6), each is consistent with the scope's
formulas.

1. **Reaction sign.** The scope's formulas (K_cf − ω²M_cf)u and −K_s·u
   already give the force the support applies to the piping. DEL-04-10
   CLM-001 should say so in words, in global axes.
2. **End-force frame.** CLM-001 says "(K_e − ω²M_e)u on the element" but
   names no frame. It should name the frame (global, or member axes with the
   member-frame rule) and the ordering. Combined magnitudes do not transform
   between non-permutation frames (R4).
3. **Grouping boundary.** "Within 10 % of the group's lowest" should read
   "≤ 10 %, inclusive". Under the adopted cluster rule, the scope should also
   say that a cluster is represented by its lowest frequency. The references
   avoid the boundary by 1e-6, so no value depends on it.
4. **100-40-40 with fewer than three directions.** The scope's "R_a + 0.4 R_b
   + 0.4 R_c over the permutations" presumes three directions, but C4 uses
   two. State max_a(R_a + 0.4 Σ_{b≠a} R_b). The question of 0.4 against 2/5
   lies below 1e-9.
5. **Period abscissa.** "Linear or logarithmic on each axis" should say "on
   the table's own abscissa, period when period is entered". This matters only
   for a linear period axis (`S-PLG`): a logarithmic axis gives the same
   result in period or in frequency.
6. **Extracted rigid modes.** REQ-006's contribution rows are periodic
   per-mode values plus the rigid response. The references also publish
   rigid-extracted per-mode values; these feed only the correction-off data.
   The scope should say whether a product publishes them.
7. **"Fraction moving with the supports".** REQ-004 does not define it. The
   references use 1 − p_kᵀM_ff⁻¹p_k/T_k: the mass no finite mode reaches,
   including mass held by a rigid skew restraint at a free node. REQ-004
   should state this formula.

## Determinism and imports

- `shasum -a 256 -c SHA256SUMS` passes for all 13 files as found.
- `python3 -I -B generators/f2_spectrum_discrete.py` reports "check passed"
  and regenerates all 10 JSON files and SHA256SUMS byte for byte.
- **Import closure.** The generators import only:
  - standard modules: `argparse`, `decimal`, `fractions`, `hashlib`, `json`,
    `os`, `pathlib`, `sys`;
  - their own `spectrum_cases`;
  - the four f1 modules `elements`, `model`, `eigen` and `numeric`. Their
    SHA-256 values match the pins, and they import only `fractions`,
    `decimal`, `math` (for `isqrt`, exact integer arithmetic) and each other.
  There is no dynamic import, `exec` or `eval`.
- **Note on the pins.** `sys.path` puts `generators/` ahead of the f1
  directory. A file named like a pinned module placed in `generators/` would
  shadow it without tripping the pin. No such file exists. Putting F1 first,
  or importing by path, would close the gap.
- **This file and SHA256SUMS.** As in `../../f1_modal/mass_discrete`,
  `FIXED_FILES` does not carry `REFUTATION.md`. Adding its hash to
  SHA256SUMS would make check mode report a mismatch; leaving it out leaves
  this file unhashed. As assigned, SHA256SUMS was not edited; the author
  decides.

## For the author

- The fix for each of R1 to R9 is given in that finding.
- The status line ("not yet refuted") needs the author's update.
- The README contains accounts of the work: "When this was introduced, the
  outputs of C1–C7 were unchanged byte for byte", "C1–C6 were unchanged byte
  for byte" and "C7's combined blocks were unchanged too". These are not
  verifiable from the package. AGENTS.md places such accounts in the PR
  description, not the repository.

## Not checked

- The generator's self-reported residuals (the README "Cross-checks" table,
  each case's `modal_cross_checks` and `cross_checks` exponents) and its
  `sturm_counts_at_gap_shifts`. They are the generator's own diagnostics. I
  computed every eigenvalue independently instead.
- The generator's refusals (gaps between δ_c/2 and 2δ_c, and flush values
  between 1e-50 and 1e-35) and its 80-digit working precision.
- Whether `inputs_binary64` lists every input of every model. Each listed
  entry was checked, and the model inputs were read as binary64 directly.
- The product, `../closed_form/` and any CAEPIPE comparison.
- The 16-digit experiment in R5 is a decimal stand-in for binary64, not a
  product eigensolver.

## Additions after the first record

A further refuting TASK, not the author of this package and not the writer of the record above,
examined what was added to or changed in this package after that record (the member-axis end-force blocks of C9, the δ_c cluster statements, C7's two bases, the fraction moving with the supports, and the renamed rule keys), as committed on
`claude/dynamics-references` at c8c602df6. This section is its record: one record covered this package and
`../closed_form` together, the rows below are this package's, and its count of checks is for both. Governing texts: DEL-04-10
CLM-001 (with Repeated modes), REQ-002 to REQ-004; DEL-04-09 REQ-002, REQ-003 (δ_c = 2⁻²⁰), REQ-005.

**Verdict.** Nothing refuted: no value, formula or statement checked below disagrees.

**Independence.** The conditions are those of the benchmark-pack record (`../benchmark_pack/REFUTATION.md`). No product code was read, imported or run,
and no CAEPIPE material was read. The scripts use the standard library only, are my own, run with
`python3 -I`, one process at a time, in a scratch directory outside the repository, and import no
package generator. Every check starts from published inputs, contributions or eigenvalues, or from
first principles (B6's A), with rules written from CLM-001's text.

| Item | What | Checks | Largest disagreement |
|---|---|---|---|
| B1 frame | My Gram–Schmidt frame from the binary64 node coordinates and `y_reference` against the README's closed forms x = (1,1,1)/√3, y = (−1,−1,2)/√6, z = (1,−1,0)/√2, `derived_model.member_frames` (rows and lengths) and the block's `frames`, C9 and C9-CHAIN | 448 | 3.4e-40 |
| B1 rotation, C9-Y | Every published global signed end-force contribution (8 members × 12 components × 6 modes) rotated by Λ = rows (x, y, z) against `element_end_forces_member_axes.contributions_signed` | 576 | 4.4e-40 of the mode's scale |
| B1 balance, C9-CHAIN-Y | No global end forces are published, so: P1.i (rotated back) = SA force and moment; P8.j = S1 force, zero moment; P_k.j + P_{k+1}.i = 0 at every interior node; 23 modes | 1,242 | 2.9e-40 |
| B1 combinations | SRSS, CQC (ρ of CLM-001 at the binary64 0.05, units' lowest f), GROUPING, TEN_PERCENT, ABSOLUTE_SUM formed by me on the member-axis contributions; `per_direction_combined` = the request's rule; `combined` (one direction); every `mutations` entry | 2,784 | 7.1e-40 |
| B1 keys and units | The member-axis keys equal `derived_model.member_axes_quantities` (96 each); F in N, M in N·m | 2 cases | — |
| B2 | δ_c chaining recomputed from the published eigenvalues of every case (all ten files), λ_{i+1} − λ_i ≤ 2⁻²⁰ λ_{i+1}: membership of every `scope_clusters` entry and its standing (lowest) frequency; `frequency_list` against the clusters' frequencies; `adjacent_pairs_within_delta_c` (C8, C9, C9-CHAIN, C10-W2, C10-W3), their stated gaps, and `min_relative_eigenvalue_gap_all_modes` | 279 clusters, 167 frequencies, 5 pair lists | exact; gaps to the published 3 digits |
| B3 | C7 `basis_1` and `basis_2`: the pair's cluster sum from each basis's vectors against its `cluster_sum_signed` and the analysis's cluster sum; each of the five rules over the two units (the pair's sum and the published sum of modes 3 and 4) per direction and over directions (SRSS); basis_1 against basis_2; against the analysis's SRSS, CQC and GROUPING and its `combined` | 744 | 5.2e-40 |
| B4 | C6A (rigid skew snubber SN2) and C6B (no skew constraint): reduced DOF count 30 − 13 − 1 = 16 and 30 − 12 = 18, equal to `n_reduced_dofs` and to the number of listed modes; `fraction_moving_with_supports` = 1 − Σ over every finite mode m_eff/T_k and `all_finite_modes_cumulative_fraction` per direction | 12 + 2 | 1.6e-40 |
| B5 discrete | All five renamed rules recomputed from the published contributions in C9-CHAIN-Y (where GROUPING and TEN_PERCENT differ: groups (21, 22), (23); pairs (21, 22), (22, 23)), every C3, C4 and C10 analysis, and C8-I-FLAT | 4,455 | 3.1e-33 (C10-W3, a quantity whose cluster contributions cancel 1219 − 1219 → 2e-4, at the 40-digit rounding of its inputs); 7e-40 elsewhere |

About 11,000 checks in all.

Notes (not refutations):

- B1: the README's statement that the member-axis block's flush scale is its own was not tested.
- B5: in C10-W3, with near-cancelling cluster contributions, a combination recomputed from the
  published 40-digit contributions can differ from the published one by about 1e-33 relative. This
  comes from the inputs' rounding; a reader recomputing at the verification tolerance is unaffected.

Not checked:

- No model was re-solved for Part B (as assigned). The C9-CHAIN member-axis contributions are
  therefore confirmed by equilibrium against the published SA and S1 values, not by a rotation of
  global end forces, which that case does not publish.
- B4: p_rᵀM_r⁻¹p_r was not formed from an assembled mass. The corrected statement was checked
  through its stated equal form, 1 − Σ m_eff/T_k over a modal set shown to span the whole reduced
  space.
- This package's generators, and their regeneration, were not run for this section.
