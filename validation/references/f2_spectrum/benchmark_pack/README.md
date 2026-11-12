# F2 benchmark pack: closed-form third values for the cross-engine benchmark's pipe

These are the closed-form and generator-FE values the cross-engine spectrum benchmark quotes for its
one exact pipe. They serve DEL-04-10 REQ-008 and AC-006, and the spectrum screening class of the
release target's benchmark method.

- **Case C:** one pipe, a cantilever in one plane and a propped cantilever in the other.
- **Case D:** the pipe on the space diagonal with a skew tip spring.

**Status: references with a refutation record (`REFUTATION.md`).**

**Independence.**

- No product code is read, imported or run, and no CAEPIPE material is used.
- The generator reuses the `closed_form` package's numerics, table rules, continuum engine, finite
  elements and beam helpers, imported read-only by path.
- Each reused module is pinned by SHA-256 (`generators/pinned.py`), and a changed module stops the
  run. No bytecode is written.
- `../discrete/c8_pipe_x.json`, also pinned, is read only for one cross-check. No value is copied from
  it.
- Everything needed to rebuild the values is derived below.

## Files

| File | Role |
|---|---|
| `case_c.json` | case C: merged frequency lists with mode totals; FLAT and TWO responses; truncation case; cross-checks, including the one against the sibling |
| `case_d.json` | case D: continuum closed form, first two modes (modes 3 to 6 for information) |
| `pack_summary.json` | flat rows per benchmark case id: continuum, consistent and lumped values as [40-digit decimal, binary64] |
| `predicted_formulation_difference.json` | per case id and compared quantity: consistent against lumped at the same member count |
| `generators/pinned.py` | hash-pinned import of `closed_form/generators/{numerics,rules,continuum,fe,beams}.py` |
| `generators/pack_c.py`, `pack_d.py`, `pack_summary.py`, `pack_predicted.py`, `generate.py` | case C, case D, the summary, the predicted formulation difference and the driver |
| `REFUTATION.md` | the refutation record (authored; verified by hash, not generated) |
| `SHA256SUMS` | hashes of every file above |

To check or regenerate (standard library only, single process, about 12 s):

    python3 generators/generate.py --check
    python3 generators/generate.py --write

## Number format

- **Inputs** are binary64, published as shortest decimal, hex and exact rational.
- **Derived reals** have 40 significant digits plus the nearest binary64.
- **Transcendentals** are computed in `decimal` at 90 digits (FE eigen-solutions at 60), with no
  libm value in a published number.
- **Diagnostics.** `rel_dev` and cross-check residuals are short diagnostics, not references.
- **Units.** SI throughout.

## The pipe

| Input | Value |
|---|---|
| Outside diameter D | 0.0603 m |
| Wall t | 0.00391 m |
| Length L | 15 m |
| Steel density ρ | 7850 kg/m³ |
| E | 200 GPa |
| G | 78.125 GPa |
| Contents, insulation, allowances | none |

With d = D − 2t:

- A = π(D² − d²)/4 and I = π(D⁴ − d⁴)/64;
- m̄ = ρA (≈ 5.4375 kg/m), so the total mass T = m̄L = 81.562 kg;
- ī = ρI, the Rayleigh rotary inertia (contents excluded; there are none);
- J = 2I and j̄ = ρJ.

EI/m̄, ī/m̄, EA/m̄ and GJ/j̄ are exact rationals of the binary64 inputs, because π cancels.

## Methods shared by both cases

**Spectra.** Both tables are linear on both axes, with damping ratio 0.05 and scale factor s = 1.
Linear on both axes, S(f) = S₀ + (S₁ − S₀)(f − f₀)/(f₁ − f₀) on the segment containing f, and a
tabulated point returns its ordinate.

- **FLAT:** (0.125 Hz, 2.5 m/s²) to (1000 Hz, 2.5 m/s²).
- **TWO:** (0.125, 1.5), (2, 4.5), (16, 2.25).

**Continuum beam.** In nondimensional form (x = X/L), W'''' + r²Ω²W'' − Ω²W = 0, with
Ω² = ω²m̄L⁴/EI and r² = ī/(m̄L²) (r² = 0 is Euler–Bernoulli).

- W = c₁cosh ax + c₂sinh ax + c₃cos bx + c₄sin bx, with a², b² = (∓r²Ω² + √(r⁴Ω⁴ + 4Ω²))/2.
- End conditions, from Hamilton's principle with strain energy ½EI∫w''² and kinetic energy
  ½∫(m̄ẇ² + īẇ'²):

  | End | Conditions |
  |---|---|
  | clamped | W = W' = 0 |
  | pinned | W = W'' = 0 |
  | free | W'' = 0 and W''' + r²Ω²W' = 0 |
  | tip spring of stiffness k | W'' = 0 and W''' + r²Ω²W' − κW = 0, with κ = kL³/EI |

- The frequencies are the roots of the 4×4 determinant of these rows, found by bisection in decimal;
  the shape is the determinant's null vector.
- Euler–Bernoulli checks:
  - cantilever, cos β cosh β = −1;
  - propped cantilever, tan β = tanh β;
  - tip spring, β³(1 + cos β cosh β) − κ(cos β sinh β − sin β cosh β) = 0;
  - in each, Ω = β².
- Then ω = Ω√(EI/m̄)/L² and f = ω/(2π).

**Modal quantities.** W is normalized so its largest |W| on [0, 1] is +1. Then:

- D = ∫W² + r²∫W'²;
- Γ = ∫W/D;
- effective mass m̄L(∫W)²/D.

Per unit spectral acceleration, the planar quantities of a mode are:

- displacement: Γ W(x) L⁴/((EI/m̄)Ω²);
- F₀ = Γm̄L(W'''(0)/Ω² + r²W'(0));
- F_L = −Γm̄L(W'''(1)/Ω² + r²W'(1));
- M₀ = −Γm̄L²W''(0)/Ω².

Each is multiplied by s S(f). Reactions are the forces and moments the supports apply to the pipe.

**Static response** to a uniform load m̄ per unit acceleration (Euler–Bernoulli statics). With
ŵ'''' = 1 and w = (m̄L⁴/EI)ŵ:

| Span | ŵ(x) | Response | Reactions |
|---|---|---|---|
| cantilever | x²(6 − 4x + x²)/24 | tip qL⁴/8EI | F₀ = −qL, M₀ = −qL²/2 |
| propped cantilever | x²(3 − 5x + 2x²)/48 | mid-span qL⁴/192EI | F₀ = −5qL/8, M₀ = −qL²/8, F_L = −3qL/8 |

**Finite elements** (N equal members, h = L/N, planar DOFs w and θ = dw/dx per node):

- **Stiffness:** EI/h³[12, 6h, −12, 6h; 6h, 4h², −6h, 2h²; −12, −6h, 12, −6h; 6h, 2h², −6h, 4h²].
- **Consistent:** m̄h/420[156, 22h, 54, −13h; 22h, 4h², 13h, −3h²; 54, 13h, 156, −22h;
  −13h, −3h², −22h, 4h²], plus ī/(30h)[36, 3h, −36, 3h; 3h, 4h², −3h, −h²; −36, −3h, 36, −3h;
  3h, −h², −3h, 4h²]. This is the product's formulation, DEL-04-09 REQ-001.
- **Lumped:** m̄h/2 at each end translation and nothing else (no rotary inertia). This is NOT the
  product's formulation.
- **Rods** (fixed-free, N linear members, EA or GJ over h). The consistent mass is m̄h[1/3, 1/6;
  1/6, 1/3] (axial) or j̄h[1/3, 1/6; 1/6, 1/3] (torsion). The lumped mass is m̄h/2 at each end, with no torsional
  inertia. The frequencies come from closed forms, with θ_j = (2j − 1)π/(2N):
  - consistent: λ̂_j = 6N²(1 − cos θ_j)/(2 + cos θ_j);
  - lumped: λ̂_j = 2N²(1 − cos θ_j);
  - f = √(λ̂·EA/m̄)/(2πL), or with GJ/j̄ for torsion.
- **Eigen-solution:** all free DOFs. Consistent: Cholesky reduction and cyclic Jacobi. Lumped: the
  massless rotations are condensed first. Each eigenvalue is re-found by Sturm-count bisection.
- **Excitation and recovery:**
  - p = (M R)_f with R the unit translation of all translational DOFs, restrained ones included, and
    Γ = φᵀp/φᵀMφ.
  - The modal displacement per unit spectral acceleration is u = Γφ/ω².
  - Reactions are the residuals (K − ω²M)u at the restrained DOFs (times m̄ in the per-unit-m̄ form
    the generator uses).
  - Mid-span is node N/2, which exists for even N.

**Modal rules** (DEL-04-10 CLM-001), on each quantity's signed per-mode values R_i:

- SRSS: √(ΣR_i²);
- `ABSSUM`, the absolute sum: Σ|R_i|;
- `TEN_PERCENT`, the ten percent method: √(ΣR_i² + 2Σ|R_iR_j|), the second sum over every pair
  i < j (ascending frequency) with (f_j − f_i)/f_i ≤ 0.1;
- CQC (case D): √(ΣΣρ_ijR_iR_j), with ρ_ij = 8ζ²(1 + r)r^{3/2}/((1 − r²)² + 4ζ²r(1 + r)²),
  r = ω_j/ω_i and ζ = 0.05.

No two modes of these models lie within DEL-04-09's cluster tolerance δ_c = 2⁻²⁰ (checked for case
C; the case D pair is 5 % apart). So no cluster is summed first under CLM-001's repeated-mode rule,
and the rules apply mode by mode.

## Case C

The pipe lies along X, fully fixed at A, with only UZ restrained at the tip B.

- **Y plane** (UY, RZ): the cantilever.
- **Z plane** (UZ, RY): the propped cantilever.
- **Axial (X) and torsion (RX):** fixed-free rods, uncoupled and unexcited by transverse motion; they
  count as modes.

**Sign conventions.**

- UY, UZ, FY and FZ are positive along the axes.
- MZ is conjugate to RZ = dUY/dX, so MZ = M₀ of the XY problem.
- MY is conjugate to RY = −dUZ/dX, so MY = −M₀ of the XZ problem.
- Combined magnitudes do not depend on these signs.

**Response quantities.**

- From Y: tip UY, base FY (= F₀), base MZ.
- From Z: mid-span UZ, base FZ (= F₀), base MY, and the tip reaction FZ (= F_L).

Each receives contributions only from its own plane's modes. The other plane's modes and the axial
and torsional modes contribute exactly zero (published as 0).

**Mode totals** (`finite_modes_total`, with `finite_modes_derivation`).

- **Consistent:** the 3D model has 6(N + 1) DOFs, of which 7 are restrained (6 at A, UZ at B), so
  6N − 1 are free. The consistent mass (translational, Rayleigh rotary, axial, torsional) is positive
  definite on all of them, so there are **6N − 1** finite modes: Y bending 2N, Z bending 2N − 1,
  axial N, torsion N.

  | N | 1 | 2 | 4 | 8 | 16 |
  |---|---|---|---|---|---|
  | Finite modes | 5 | 11 | 23 | 47 | 95 |

- **Lumped:** only translations carry mass. The free translations are Y: N, Z: N − 1 (the tip UZ is
  restrained) and X: N; rotations are massless. So there are **3N − 1** finite modes: Y N, Z N − 1,
  axial N, no torsion.

  | N | 1 | 2 | 4 | 8 | 16 |
  |---|---|---|---|---|---|
  | Finite modes | 2 | 5 | 11 | 23 | 47 |

- **Continuum:** infinitely many. The generator checks each FE count against the modes it builds.

**Merged frequency list** (`first_modes`). It is the true first 12 modes of the model, in ascending
order, each with its family.

- **FE:** every finite mode of every family is built, and the list is their first 12 (all of them
  where fewer).
- **Continuum:** bending is built to 8 modes per plane and axial and torsion to 3. The generator
  asserts that each family's first mode not built lies above the 12th listed frequency.

No Y–Z pair is within 1 % in any list.

**Periodic set.** The first six modes of the model's merged list are periodic, with no correction.
Where fewer than six finite modes exist, all of them are periodic.

| Model | Modes in the periodic set |
|---|---|
| Consistent N = 1 | Y1, Z1, Y2, RX1, X1. Z1 has no nodal translation and is normalized by rotation (DEL-04-09 REQ-005 case (b)): it is the only bending mode in this pack normalized by rotation, and every torsion mode (RX) is case (b) too. Z1 participates through the consistent coupling to restrained DOFs |
| Lumped N = 1 | Y1 and X1 only; no finite Z-plane mode, because its only Z-plane DOF is a massless rotation |
| Lumped N = 2 | Y1, Z1, Y2, X1, X2 |
| Every other model | Y1, Z1, Y2, Z2, Y3, Z3 |

Results are given under FLAT at every N, and under TWO at N = 8 and in the continuum, with the rules
SRSS, ABSSUM and TEN_PERCENT and the per-mode signed contributions.

**Not published (stated in place, null in the summary):**

- mid-span UZ at N = 1, because mid-span is not a node;
- every Z quantity of lumped N = 1.

**Truncation case** (N = 8 and the continuum). The first two merged modes (Y1 and Z1) are periodic,
with the correction on, f_r = 1.25 Hz and FLAT (a_r = 2.5 m/s²).

- **Rigid part, FE:** for direction k it is the static solve of K_ff u = F_f, with
  F = a_r M (R_k − Γ₁ₖ φ̂₁ₖ) over all DOFs. The reactions are K_cf u − F_c.
- **Rigid part, continuum:** a_r(Q_static − Γ₁q₁/ω₁²), with q₁ the mode-1 quantity per unit Γ.
- **Combination:** the rigid part is combined with the single periodic contribution by SRSS and by
  absolute sum.
- **Lumped formulation:** the rigid load is formed on all DOFs, restrained ones included. A restrained
  translation's lumped mass (m̄h/2 at A, and at B in the Z plane) is absent from the modes but present
  in the total mass, so it enters the rigid load at its own DOF and reacts there directly.
- **Total reaction check:** for every formulation the rigid reactions sum to −a_r(T − periodic
  effective mass). The check is published per direction, with the lumped restrained mass stated.

**Deviations.** Every response carries `rel_dev` against the Rayleigh continuum with the case's own
periodic set: six modes, or Y1 and Z1 for the truncation case. Per-mode values are matched to the
continuum mode of the same plane and index. `rel_dev` is (value − continuum)/|continuum|, so for a
negative quantity its sign is the opposite of value/continuum − 1. The consistent N = 1 and the lumped
N = 1 and N = 2 models have fewer than six bending modes in their periodic sets, so their combined
deviations include the missing modes.

## Case D

The same pipe lies along the space diagonal from the origin toward (1,1,1), fully fixed at A. A
linear spring of k = 8 N/m acts along global X at the tip. Excitation is FLAT along Y only, with six
modes periodic and no correction.

**General plane form.** The axis is a = (1,1,1)/√3, and the bending planes are p = (2,−1,−1)/√6 and
q = (0,1,−1)/√2.

- **Spring:** d = X gives d·p = 2/√6, d·q = 0 and d·a = 1/√3. So k_eff = k(d·p)² = 2k/3 = 16/3 N/m
  acts in plane p only, and κ = 0.3253.
- **Excitation components:** c_p = Y·p = −1/√6, c_q = Y·q = 1/√2, c_a = Y·a = 1/√3.
- **Planes:** plane p is a cantilever with a tip spring of κ; plane q is the plain cantilever.

For mode n of plane j (shape W, unit vector ĵ), with A_n = c_j(∫W/D)/ω_n², per unit spectral
acceleration:

- tip displacement: A_n W(1) ĵ;
- clamp force: (EI/L³)A_n W'''(0) ĵ, plus, for plane-p modes, the spring's axial component
  −(F_s·a)a carried by the axially rigid pipe;
- clamp moment: −(EI/L²)A_n W''(0)(a × ĵ);
- spring reaction (plane p): F_s = −k(d·u_tip)d;
- effective mass along Y: c_j²m̄L(∫W)²/D;
- Γ along Y under the scope's normalization (largest global translational component +1, the first
  of X, Y, Z on a tie): c_j∫W/(σD), with σ = 1/(W(x*)·ĵ_i*).

The first pair is 5.12 % apart: plane q at 0.2509 Hz and plane p at 0.2637 Hz. Results are given
under Euler–Bernoulli and Rayleigh for the first two modes:

- frequencies;
- per-mode and combined tip displacement, base reactions in global X, Y and Z, and the spring
  reaction;
- the combined values under SRSS, ABSSUM, TEN_PERCENT and CQC.

Modes 3 to 6 and the six-mode combination are added, labelled for information only.

**Case D is comparison only, not for verification at the analytic tolerance** (`use` in
`case_d.json`).

- **Neglected coupling:** the continuum assumes an axially rigid pipe. The spring seen by plane p is
  reduced by δ = k d_a²(L/EA)/(1 + k d_a² L/EA) = 2.9·10⁻⁷.
- **Effect on frequency:** the first-order effect on a plane-p frequency is −δκW(1)²/(2Ω²D), which is
  −1.4·10⁻⁸ for mode 1.
- **Effect on displacement:** this moves the plane-p mode 1 displacement contributions by about
  3·10⁻⁸ under FLAT through 1/ω² alone, and the coupled modes acquire axial components.
- **Flexibility ratio:** axial to bending tip flexibility is 3I/(AL²) = 5.3·10⁻⁶.

**Axial excitation.** c_a = 1/√3 excites axial modes (first near 84 Hz). They are not among the first
six modes, so without the correction they do not enter.

**The N = 8 values for case D are the sibling discrete package's, not this package's.**

## pack_summary.json

Case ids:

- `cpb-dyn-c-n01-flat`, `-n02-flat`, `-n04-flat`, `-n08-flat`, `-n16-flat`;
- `cpb-dyn-c-n08-two`, `cpb-dyn-c-n08-trunc`;
- `cpb-dyn-d-n08-flat`.

Each row gives a quantity (and its rule) with the Rayleigh continuum value (the product's continuum
limit), the consistent value and the lumped value at that member count, in SI as [decimal, binary64].
Case C rows also give the frequencies of Y1–Y3 and Z1–Z3. Case D rows give the continuum only, for
modes 1–2 and (for information) modes 1–6.

## predicted_formulation_difference.json

The file opens with a fixed statement, a constant in the generator:

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
Case D has no consistent or lumped values in this package. Every number comes from the 40-digit
decimals `pack_summary.json` publishes; the generator asserts this.

## Cross-checks

Against the sibling's `c8_pipe_x.json` (its C8-I-FLAT, C8-II-TWO-LL and C8-III-TRUNCATION):

- the first six frequencies at N = 8 consistent;
- the signed per-mode contributions of the seven quantities under FLAT and TWO;
- the rigid response per unit rigid acceleration, scaled by its a_r;
- the signs.

Within the pack:

- continuum per-mode reaction against −m_eff;
- Euler–Bernoulli roots against cos cosh = −1 and tan = tanh;
- static coefficients against the textbook values;
- FE eigenvalues against Sturm bisection;
- FE per-mode reaction against −m_eff;
- FE rigid displacement against the non-periodic modes' sum;
- the total reaction check for every formulation;
- rod closed forms against Sturm counts;
- mode totals against the formulation counts;
- completeness and consistency of the continuum lists;
- no cluster within δ_c;
- case D: the tip-spring equation against the determinant, and per-mode equilibrium in 3D.

## Limits

- Planar (case C) or general plane form (case D) Euler–Bernoulli and Rayleigh beams only: no shear
  deformation.
- The consistent FE is the product's planar formulation. The lumped FE is a comparison device only.
- These are data. They set no screening class or tolerance.
