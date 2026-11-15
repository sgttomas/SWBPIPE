# Refutation record: f2_spectrum benchmark pack

This is the refuting TASK's record for the candidate references in this directory: `README.md`,
`case_c.json`, `case_d.json`, `pack_summary.json`, `predicted_formulation_difference.json` and
`generators/`, as committed on `claude/dynamics-references` at c8c602df6.

The governing texts are DEL-04-10 CLM-001 (with Repeated modes), REQ-001 to REQ-004, REQ-008, AC-002,
AC-006 and VER-006, and DEL-04-09 REQ-001 (mass), REQ-002 and REQ-003 (clusters, δ_c = 2⁻²⁰) and
REQ-005 (normalization and participation).

This record makes no acceptance statement and sets no tolerance or screening class.

**Verdict.** No published value is refuted. Three descriptive statements are refuted (R1 to R3
below), and one convention the files apply is not stated (S1).

## Independence

- The refuting scripts read, imported and ran no product code, and no CAEPIPE material was read.
- They import nothing from this package's generators or from `../closed_form/generators`. The
  generators were read only after every comparison below had been made: `pack_predicted.py` for its
  statement constant, `generate.py` and `pinned.py` for their imports, pins and bytecode setting.
- The scripts use the Python standard library only (`decimal` at 50 to 80 digits, `fractions`), run
  with `python3 -I -B`, one process at a time, from a scratch directory outside the repository that
  holds no instructions.
- They were written from the two scopes and textbook beam dynamics:
  - π by Machin's formula; sin and cos by Taylor series with reduction; `Decimal.exp` and `ln`.
  - Section ratios as exact rationals of the binary64 inputs, d = D − 2t exactly.
  - Continuum: the Rayleigh beam equation from Hamilton's principle, its clamped, pinned, free and
    tip-spring end rows derived independently; roots by sign scan and bisection on the 2 × 2
    determinant left after the clamped conditions; ∫W, ∫W² and ∫W'² in closed form on the
    exponential–trigonometric form of W (no quadrature); reactions from the boundary terms of the
    variation.
  - FE: planar Hermitian stiffness, consistent translational mass and Rayleigh rotary mass assembled
    by hand; lumped mass m̄h/2 per member end; rotations condensed for the lumped mass; Cholesky of
    M and cyclic Jacobi on L⁻¹KL⁻ᵀ; rods assembled as matrices and solved the same way (not by the
    closed forms the README states).
  - Recovery as CLM-001 states: (K − ω²M)u at restrained DOFs; the rigid part as the static solve of
    a_r(M R − Γ₁Mφ₁) over all DOFs with reactions K_cf u − F_c.
  - Case D in the general plane form derived independently: k_eff = k(d·p)², the axial spring
    component carried to the clamp, the moment along a × ĵ, Γ under the normalization with the
    scope's "largest component +1" and the pack's tie rule, and the neglected coupling from the
    static condensation of the axial tip DOF, checked against a re-solve at κ(1 − δ).

## Results

| Group | Checks | Largest disagreement |
|---|---|---|
| Section values and exact ratios (EI/m̄, ī/m̄, EA/m̄, GJ/j̄ identical as rationals) | 7 | 3.8e-41 |
| Merged lists: every frequency of every list (continuum EB and Rayleigh; N = 1, 2, 4, 8, 16, both masses), family, family index and order | 118 | 3.8e-40 |
| Mode totals and their family splits (6N − 1: 2N, 2N − 1, N, N; 3N − 1: N, N − 1, N, 0) | 10 | exact |
| Spectral ordinates sS (FLAT, TWO) | 90 | 2.2e-40 |
| Per-mode signed contributions, seven quantities | 629 | 5.8e-40 |
| SRSS, ABSSUM, TEN_PERCENT combinations | 321 | 7.2e-40 |
| Truncation case: periodic, rigid, SRSS, ABSSUM; m_eff; −a_r(T − m_eff); sum of rigid reactions; lumped restrained masses | 132 | 5.8e-40 |
| `rel_dev` of responses, frequencies and the truncation case against my own Rayleigh values | 761 | agree to the 6 published digits (formula: S1) |
| Case D: k_eff, κ, c_p, c_q, c_a, split, f, Γ, m_eff, fractions, per-unit quantities, signed contributions, SRSS, ABSSUM, TEN_PERCENT and CQC for modes 1–2 and 1–6, EB and Rayleigh | 420 | 4.3e-40 |
| Case D neglected coupling: δ, 3I/(AL²), the three first-order frequency effects | 10 | agree to the published digits; first-order effect equal to the re-solve to 7 digits |
| `pack_summary.json`: every non-null value against my own, and its binary64 partner | 582 + 582 | 7.2e-40; every binary64 is the nearest |
| `predicted_formulation_difference.json`: rows equal to the summary, relative difference (6 digits and binary64), kind, per-case maxima | 156 rows, 8 cases | exact |
| Sibling `../discrete/c8_pipe_x.json` against my own N = 8 consistent values: all 47 frequencies and planes, C8-I-FLAT and C8-II-TWO-LL contributions, C8-III rigid response | 222 | 3.4e-40 |
| Smallest adjacent eigenvalue gap over every full FE list, 5.53e-4 (lumped N = 16, Z15–Y16) | 1 | agrees |

About 4,000 numerical checks in all. CQC in case D matches only with ζ the binary64 0.05 (with the
decimal 0.05 the difference is 4e-17), as the pack's input convention says.

Further confirmed:

- The ordinary normalization cases published in `normalization_cases`, and that Z1 at consistent
  N = 1 has no nodal translation and participates through the consistent coupling to A and B.
- The periodic sets of the README's table; every Y–Z pair in the 12-mode lists is more than 1 % apart
  (the 13th mode of lumped N = 8, Y7 at 25.25 Hz, is 0.9 % above Z6, outside the list).
- The continuum lists are the true first 12: every family's first mode not built lies above the 12th.
- The README's formulas suffice to rebuild every value, except the `rel_dev` sign (S1).
- The dated head statement is a literal constant of `pack_predicted.py` (`STATEMENT`, "constants, not
  read from the clock"); neither it nor the README asserts anything about a particular program (the
  only program name is in the README's independence line).
- The README carries no account of how the work went and no rule of conduct.

## Refuted statements

**R1.** README, Case C, periodic-set table, row "Consistent N = 1": "Z1 has no nodal translation and is
normalized by rotation (DEL-04-09 REQ-005 case (b), the only such mode in this pack)".
Every torsion mode of the consistent models is a pure RX rotation, so it is case (b) as well. That
includes RX1 in this same periodic set and RX1 and RX2 in the N = 2 list. Correct: "Z1 is the only
bending mode in this pack normalized by rotation (case (b)). Every torsion mode (RX) is case (b) too."

**R2.** README, Case C, Deviations: "At N = 1 and N = 2 the FE models have fewer modes, so their
deviations include the missing modes." Consistent N = 2 has 11 finite modes, and its periodic set is
Y1, Z1, Y2, Z2, Y3, Z3, the continuum's six (the README's own table, "Every other model"). Correct: "The
consistent N = 1 and the lumped N = 1 and N = 2 models have fewer than six bending modes in their
periodic sets, so their combined deviations include the missing modes."

**R3.** `case_c.json`, `fe.consistent_N1.responses.FLAT.quantities.mid_UZ_m.per_mode_signed`
publishes `"0"` at merged indices 1, 3, 4 and 5 and `null` at 2. Both the README ("Not published:
mid-span UZ at N = 1, because mid-span is not a node") and the `combined` entry say the quantity is
not published. A location that is not a node has no value, so every entry should be `null`.
(For lumped N = 1 the per-mode `"0"` values of the Z quantities are true zeros.)

## Statement to add

**S1.** The README does not state the formula of `rel_dev`. It is (value − continuum)/|continuum|,
by my recomputation of all 761 published deviations. For every negative quantity (base FY, MZ and
FZ, tip reaction FZ and others) this has the opposite sign of value/continuum − 1, the other usual
reading. `../closed_form/README.md` states its own formula. The README should state this one.

## For the scope

DEL-04-09 REQ-005 makes "the largest translational component +1" but gives no rule when two
components share the largest magnitude. Case D's plane-q modes have equal and opposite Y and Z
components, (0, 1, −1)/√2. This pack takes "the first of X, Y, Z", and the sign of the published Γ
and shape depends on that choice. The scope should state a tie rule over both components and nodes,
or say that the sign on a tie is unconstrained and only invariant quantities are compared.

## Regeneration and imports

- `python3 -I -B generators/generate.py --check`: exit 0, "20 recorded, 0 disagreeing (12 s)",
  "verified 12 files byte for byte". `shasum -a 256 -c SHA256SUMS`: every line OK.
- The import scan (Python `ast` over every generator and the five pinned modules) finds the standard
  library, the pack's own modules and `numerics`, `rules`, `continuum`, `fe` and `beams` of
  `../closed_form/generators` only. Their SHA-256 and that of `../discrete/c8_pipe_x.json` equal the
  pins in `pinned.py`. `generate.py` sets `sys.dont_write_bytecode`. No product path is named.
- The README's "about 10 s" was 12 s here.

## Not checked

- The lumped and consistent FE values at N = 1, 2 and 4 under TWO, which the pack does not publish.
- The generators' internal cross-checks other than by reproducing their values.
- The 90-digit precision claim and the absence of libm values in published numbers.
- Whether the "Recorded 2026-11-12" date precedes any comparison.
