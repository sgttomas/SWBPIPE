# F2 benchmark pack: closed-form third values for the cross-engine benchmark's pipe

These are the closed-form and generator-FE values the cross-engine spectrum benchmark quotes for its
one exact pipe: case C (one pipe, cantilever in one plane and propped cantilever in the other) and
case D (the pipe on the space diagonal with a skew tip spring). They serve DEL-04-10 REQ-008 and
AC-006, and the spectrum screening class of the release target's benchmark method.

**Status: candidate references, not yet refuted.** The author TASK derived and cross-checked them
(15 cross-checks, none disagreeing, including agreement with the sibling discrete package to
3.4·10⁻⁴⁰). No second agent has tried to refute them yet, and no product output has been compared
with them.

**Independence.** No product code is read, imported or run, and no CAEPIPE material is used. The
generator reuses the `closed_form` package's numerics, table rules, continuum engine, finite elements
and beam helpers. It imports them read-only by path, and each module is pinned by SHA-256
(`generators/pinned.py`): a changed module stops the run. It writes no bytecode, so nothing is written
next to the pinned modules. It reads `../discrete/c8_pipe_x.json`, also pinned, only for one
cross-check, and copies no value from it. The derivations are in `../closed_form/README.md`; this
README states only what is new.

## Files

| File | Role |
|---|---|
| `case_c.json` | case C: merged frequency lists; FLAT and TWO responses; truncation case; cross-checks, including the one against the sibling |
| `case_d.json` | case D: continuum closed form, first two modes (modes 3 to 6 for information) |
| `pack_summary.json` | flat rows per benchmark case id: continuum, consistent and lumped values as [40-digit decimal, binary64] |
| `predicted_formulation_difference.json` | per case id and compared quantity: consistent against lumped at the same member count, recorded before any comparison |
| `generators/pinned.py` | hash-pinned import of `closed_form/generators/{numerics,rules,continuum,fe,beams}.py` |
| `generators/pack_c.py`, `pack_d.py`, `pack_summary.py`, `pack_predicted.py`, `generate.py` | case C, case D, the summary, the predicted formulation difference and the driver |
| `SHA256SUMS` | hashes of every file above |

To check or regenerate (standard library only, single process, about 10 s):

    python3 generators/generate.py --check
    python3 generators/generate.py --write

## The pipe

| Input | Value |
|---|---|
| Outside diameter | 0.0603 m |
| Wall | 0.00391 m |
| Length L | 15 m |
| Steel density | 7850 kg/m³ |
| E | 200 GPa |
| G | 78.125 GPa |
| Contents, insulation, allowances | none |

Derived: m̄ = ρA (≈ 5.4375 kg/m), total mass 81.562 kg, ī = ρI (contents excluded; there are none),
J = 2I and j̄ = ρJ.

## Case C

The pipe lies along X, fully fixed at A, with only UZ restrained at the tip B.

- **Y plane** (UY, RZ): a cantilever.
- **Z plane** (UZ, RY): a propped cantilever.
- **Uncoupled modes:** axial X and torsion RX modes are uncoupled and unexcited, but they belong to the
  merged list. The lumped model has no torsional inertia, so it has no torsion modes.

The two spectra are both linear on both axes, with damping 0.05 and scale factor 1:

- **FLAT:** (0.125 Hz, 2.5 m/s²) to (1000 Hz, 2.5 m/s²);
- **TWO:** (0.125, 1.5), (2, 4.5), (16, 2.25).

**Sign conventions.** Reactions are the forces and moments the supports apply to the pipe.

- UY, UZ, FY and FZ are positive along the axes.
- MZ is conjugate to RZ = dUY/dX.
- MY is conjugate to RY = −dUZ/dX, so MY = −M₀ of the planar XZ problem.

Combined magnitudes do not depend on these signs.

**Response quantities.**

- From Y: tip UY, base FY, base MZ.
- From Z: mid-span UZ, base FZ, base MY, tip reaction FZ.

Each receives contributions only from its own plane's modes. The other plane's modes and the axial
and torsional modes contribute exactly zero (published as 0).

**Merged frequency list.** For the continuum (Euler–Bernoulli and Rayleigh) and for N = 1, 2, 4, 8,
16 under both formulations: the first 12 modes, each with its family, plus the total finite-mode
count. No Y–Z pair is within 1 % at any N.

**Periodic set.** The first six modes of the model's merged list are periodic, with no correction.
Where fewer than six finite modes exist, all of them are periodic.

| Model | Modes in the periodic set |
|---|---|
| Consistent N = 1 | Y1, Z1, Y2, RX1, X1. Z1 is a pure rotation (normalized by rotation, DEL-04-09 case (b)) and participates through the consistent coupling to restrained DOFs |
| Lumped N = 1 | Y1 and X1 only; no finite Z-plane mode, because its only Z-plane DOF is a massless rotation |
| Lumped N = 2 | Y1, Z1, Y2, X1, X2 |
| Every other model | Y1, Z1, Y2, Z2, Y3, Z3 |

Results are given under FLAT at every N, and under TWO at N = 8 and in the continuum. The rules are
SRSS, ABSSUM and PAIRWISE10, the last two not in the scope today. Per-mode signed contributions are
given for each of the six modes.

**Not published (stated in place, null in the summary):**

- mid-span UZ at N = 1, because mid-span is not a node;
- every Z quantity of lumped N = 1.

**Truncation case** (N = 8 and the continuum): the first two merged modes (Y1 and Z1) are periodic,
with the correction on, f_r = 1.25 Hz and FLAT (a_r = 2.5 m/s²).

- **Rigid part:** for direction k it is the static response to a_r(M R_k − Γ₁ₖ M φ₁ₖ) over all DOFs.
  Element forces are K_e u − F_e and reactions K_cf u − F_c (closed_form README).
- **Combination:** the rigid part is combined with the single periodic contribution by SRSS and by
  absolute sum.
- **Continuum:** the rigid part is a_r(Q_static − Γ₁q₁/ω₁²).
- **Lumped formulation:** the rigid load is formed on all DOFs, restrained ones included. A restrained
  translation's lumped mass (m̄h/2 at A, and at B in the Z plane) is absent from the modes but present
  in the total mass, so it enters the rigid load at its own DOF and reacts there directly.
- **Total reaction check:** for every formulation, the rigid reactions sum to −a_r(T − periodic
  effective mass). The check is published per direction, with the lumped restrained mass stated.

## Case D

The same pipe lies along the space diagonal from the origin toward (1,1,1), fully fixed at A. A
linear spring of 8 N/m acts along global X at the tip. Excitation is FLAT along Y only, with six
modes periodic and no correction.

The continuum uses the general plane form of `../closed_form/README.md`:

- a = (1,1,1)/√3, p = (2,−1,−1)/√6, q = (0,1,−1)/√2;
- k_eff = 2k/3 = 16/3 N/m in plane p, κ = 0.3253;
- c_p = −1/√6, c_q = 1/√2.

The first pair is 5.12 % apart: plane q at 0.2509 Hz and plane p at 0.2637 Hz.

Results are given under Euler–Bernoulli and Rayleigh, for the first two modes:

- frequencies;
- per-mode and combined tip displacement, base reactions in global X, Y and Z, and the spring
  reaction;
- the combined values under SRSS, ABSSUM, PAIRWISE10 and CQC (damping 0.05).

Modes 3 to 6 and the six-mode combination are added, labelled for information only.

**Case D is comparison only, not for verification at the analytic tolerance** (`use` in
`case_d.json`). The continuum assumes an axially rigid pipe. The neglected coupling lowers the first
plane-p frequency by about 1.4·10⁻⁸ relative, which moves the plane-p mode 1 displacement
contributions by about 3·10⁻⁸ under FLAT through 1/ω² alone. That is well above the 1e-9
verification tolerance, and the coupled modes also acquire axial components. These values are
cross-engine comparison data and cannot verify a product model with finite axial stiffness at 1e-9.

**Neglected axial coupling.** The pipe is taken as axially rigid under the spring's axial component.
The spring seen by plane p is reduced by δ = k d_a²(L/EA)/(1 + k d_a² L/EA) = 2.9·10⁻⁷. The
first-order effect on the first plane-p frequency is −1.4·10⁻⁸ relative. The ratio of axial to bending
tip flexibility is 3I/(AL²) = 5.3·10⁻⁶.

**Axial excitation.** c_axial = 1/√3 excites axial modes (first near 84 Hz). They are not among the
first six modes, so without the correction they do not enter.

**The N = 8 values for case D are the sibling discrete package's, not this package's.**

## pack_summary.json

Case ids:

- `cpb-dyn-c-n01-flat`, `-n02-flat`, `-n04-flat`, `-n08-flat`, `-n16-flat`;
- `cpb-dyn-c-n08-two`, `cpb-dyn-c-n08-trunc`;
- `cpb-dyn-d-n08-flat`.

Each row gives a quantity (and its rule) with the Rayleigh continuum value (the product's continuum
limit), the consistent value and the lumped value at that member count, in SI as [decimal, binary64].
Case C rows also give the frequencies of Y1–Y3 and Z1–Z3.

Case D rows give the continuum only, for modes 1–2 and (for information) modes 1–6. Their
consistent and lumped entries are null.

## predicted_formulation_difference.json

The file opens with this fixed statement, which is a constant in the generator, not read from the
clock:

> Predicted before any cross-engine comparison. Recorded 2026-11-12. It states, per member count, how
> far a plain translational lumped-mass formulation is expected to differ from SWBPIPE's stated
> formulation on these cases, so that when sealed captures are opened the expected convention
> difference is on record from before and is not fitted afterwards. It is a prediction about a
> formulation, not about any program: whether another program's mass formulation is this one is not
> asserted.

For each case id and each compared quantity and rule of `pack_summary.json`, it gives:

- the consistent value and the lumped value at the same member count;
- their relative difference (lumped / consistent − 1), as a 6-digit decimal and the nearest
  binary64;
- per case, the largest absolute relative difference by kind: frequency, displacement, force or
  moment, support reaction.

Case C's compared forces and moments are all support reactions, so "force or moment" is null there.
Case D has no consistent or lumped values in this package.

Every number comes from the 40-digit decimals that `pack_summary.json` publishes; the generator
asserts this, and the file adds nothing else numerically.

**This file is never regenerated with different numbers after a comparison.** A later change to it is
a change to a frozen reference and needs independent grounds.

**Deviations.** Every response in `case_c.json` carries rel_dev against the Rayleigh continuum with the
case's own periodic set (six modes, or Y1 and Z1 for the truncation case). Per-mode values are matched
to the continuum mode of the same plane and index. At N = 1 and N = 2 the FE models have fewer modes,
so their deviations include the missing modes.

## Cross-checks (15, all agree)

Against the sibling's `c8_pipe_x.json`:

- the first six frequencies at N = 8 consistent agree to 2.2·10⁻⁴⁰;
- the signed per-mode contributions of the seven quantities under FLAT and TWO (its C8-I-FLAT and
  C8-II-TWO-LL) agree to 3.4·10⁻⁴⁰;
- the rigid response per unit rigid acceleration, scaled by its a_r = 3.5 m/s² (C8-III-TRUNCATION),
  agrees to 2.1·10⁻⁴⁰;
- every sign agrees.

Within the pack:

- continuum per-mode reaction against −m_eff;
- Euler–Bernoulli roots against cos cosh = −1 and tan = tanh;
- static coefficients against the textbook values;
- FE Jacobi eigenvalues against Sturm bisection;
- FE per-mode reaction against −m_eff;
- FE rigid displacement against the non-periodic modes' sum;
- the total reaction check for every formulation;
- rod closed forms against Sturm counts;
- case D: the tip-spring equation against the determinant, and per-mode equilibrium in 3D.

## Limits

- Planar (case C) or general plane form (case D) Euler–Bernoulli and Rayleigh beams only: no shear
  deformation.
- The consistent FE is the product's planar formulation. The lumped FE is a comparison device only.
- These are data. They set no screening class or tolerance.
