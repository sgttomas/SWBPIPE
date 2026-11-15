# F2 spectrum references: closed form and hand calculation

Frozen, independently derived expectations for DEL-04-10, response-spectrum
analysis ([scope](../../../../execution/PKG-04/DEL-04-10/ScopeOfWork.md);
REQ-001 to REQ-004, REQ-008; AC-002, AC-006). They give:

- spectrum table evaluation under the four interpolation rules (item 1);
- the single-degree-of-freedom oscillator (item 2);
- a two-mass chain with every modal combination and the missing-mass correction (item 3);
- directional combination (item 4);
- a uniform pipe as cantilever, propped cantilever, pinned-pinned and fixed-fixed span, in
  continuum closed form and in the generator's own finite elements at N = 2, 4, 8, 16 under two
  mass formulations (item 5), with each response's sensitivity to a mode's frequency (item 6);
- a closely spaced pair split by a tip spring, in general plane form, and the basis ambiguity of an
  exactly repeated pair;
- `screening_basis.json`, the data the spectrum screening class of the release target's benchmark
  method is to be set from. It proposes no class value.

**Status: references with a refutation record** ([REFUTATION.md](REFUTATION.md)).

**Independence.** The generators import, read and run no product code (nothing under `core/`,
`apps/`, `api/`, `tests/`, `schemas/`, `fixtures/` or `tools/`). The references rest on the DEL-04-10
and DEL-04-09 scopes and standard structural dynamics, derived below. No CAEPIPE material was used.
All model data are invented. No formula or value is copied from a table.

## Files

| File | Item | Role |
|---|---|---|
| `spectrum_tables.json` | 1 | tables T1 and T2 under the four rules, at tabulated, interior, end and outside points |
| `sdof.json` | 2 | mass-spring oscillator under T1 and T2, each rule; the neglected velocity term |
| `two_mass_chain.json` | 3 | chains A (separated), B (closely spaced), C (stiff); combinations; missing mass |
| `directional.json` | 4 | SRSS, 100-40-40 and absolute sum over directions |
| `beams_continuum.json` | 5 | continuum Euler–Bernoulli and Rayleigh results for 4 spans at 2 lengths |
| `beams_fe.json` | 5 | FE results, N = 2, 4, 8, 16, consistent and lumped, with deviations; merged frequency list of the benchmark model |
| `sensitivity.json` | 6 | d ln R / d ln f_n, closed form and finite difference |
| `beam_skew_and_pairs.json` | additions | tip-spring pair, cases (a) and (b); the rotated repeated pair |
| `screening_basis.json` | summary | largest deviations per kind, formulation and N; largest sensitivities |
| `REFUTATION.md` | | the refutation record (authored; verified by hash, not generated) |
| `generators/*.py` | | `numerics` (decimal transcendentals, quadrature, JSON), `rules` (tables, combinations), one module per item, `generate` (driver) |
| `SHA256SUMS` | | hashes of every file above |

To check or regenerate (standard library only, Python ≥ 3.9, about 50 s):

    python3 generators/generate.py --check     # default: regenerate in memory, compare byte for byte
    python3 generators/generate.py --write

## Method and number format

- **Inputs** are binary64 numbers. Each is published as `binary64` (shortest round trip), `hex` and
  `exact` (the exact rational of the binary64 value).
- **Transcendental quantities** use `decimal` at 90 digits (finite-element eigen-solutions at 60).
  π comes from Machin's formula; sin and cos are Taylor series after reduction modulo 2π; exp, ln
  and sqrt come from `decimal`. Roots are found by bisection on a sign change. A binary64 scan with
  `math` only locates each bracket, and the sign change is confirmed in `decimal`. No libm value
  enters a published number.
- **Published reals** have 40 significant digits (`value`) plus the nearest binary64. Where the
  computation is rational (linear interpolation, the chain's set A eigenvalues, 100-40-40) the exact
  rational is given as well.
- **Diagnostics.** Relative deviations (`rel_dev`) and cross-check residuals are short strings. They
  are not references.
- **FE triplets.** In `beams_fe.json` each response is `[value, binary64, rel_dev]`.
- **Exact ties.** In five places the exact rational lies exactly halfway between two binary64
  numbers: `directional.json` D1–D3 `full_y`/`full_x` and D7 `ABSSUM`, and
  `spectrum_tables.json` T2, linear/linear, row 5 `sS`. The published `binary64` is the exact
  rational rounded half to even. The 40-digit `value`, read back as binary64, falls on the other
  neighbour, so derive the binary64 from `exact` there, not from `value`.
- **Zeros.** Some quantities that are exactly zero in closed form are published as rounding
  residue of 10⁻⁶⁶ to 10⁻⁹¹, not as "0": W and ∫W of antisymmetric continuum modes, and the θ = 90°
  row of the repeated pair. A comparison's absolute floor must absorb them.
- **Units.** SI: m, s, Hz, kg, N, N·m, m/s².

## Conventions taken from the scope, and where a number depends on them

- **Pseudo-acceleration.** u_n = Γ_n s S(f_n)/ω_n² · φ_n, relative to the supports (CLM-001).
- **Recovery at the displacement extremum.** Element end forces are (K_e − ω² M_e) u. Reactions are
  (K_cf − ω² M_cf) u at rigid supports and −K_s u at compliant ones. The velocity term is
  neglected (REQ-002).
- **Reaction sign.** A reaction at a rigid restraint is the full residual at the restrained DOF:
  (K_cf − ω_n² M_cf) u for a modal contribution, and K_cf u − F_c for the rigid response (F_c the
  restrained rows of the rigid load). At a compliant support it is −K_s u. Each is the force the
  support applies to the structure, positive along the global axis. Element end forces are the forces
  applied on the element at its nodes.
  Combined magnitudes do not depend on these signs. CQC and grouping need only one consistent sign
  per quantity.
- **Mode normalization.** DEL-04-09 REQ-005: the largest translational component is +1. Rotation
  normalization (case (b)) applies when the largest translation is at most 10⁻⁶·L times the largest
  rotation. Case (b) applies to the consistent-mass modes whose nodal translations all vanish:
  fixed-fixed N = 2 mode 2, and pinned-pinned N = 2 modes 2 and 4 and N = 4 mode 4, at both lengths.
  Each has Γ = 0 and contributes nothing.
  - The shape, Γ and the generalized mass depend on the normalization.
  - Normalization-invariant: the frequency, the effective mass and its fraction, Γφ, every signed
    modal response and every combination.
  - FE shapes are normalized at the nodes and continuum shapes at the true maximum, so FE Γ is not
    comparable with continuum Γ. Only invariant quantities carry deviations.
- **Rigid response.** With f_r given, modes with f < f_r are periodic (CLM-001 classifies a cluster by
  its lowest frequency; no published case has a cluster, other than the repeated pair, which has no
  missing-mass case). The rigid part per unit rigid
  acceleration is the static response to M R − Σ_periodic Γ_n M φ̂_n. Element forces are K_e u − F_e
  and reactions K_cf u − F_c (CLM-001). It equals a_r(Q_static − Σ_periodic Γ_n q_n/ω_n²), where
  q_n is the quantity of φ_n recovered with (K − ω_n² M). The continuum uses this identity, and the
  FE does the static solve literally.
- **Grouping.** A mode joins the current group when f − f_low ≤ f_low/10 (inclusive). No published
  case lies on the boundary.
- **The modal rules** are CLM-001's five, keyed `SRSS`, `CQC`, `GROUPING`, `TEN_PERCENT` and
  `ABSSUM`:
  - SRSS: √(Σ R_i²);
  - CQC: √(Σ_i Σ_j ρ_ij R_i R_j), with CLM-001's ρ_ij at the spectra's damping ratio;
  - grouping: groups in ascending order within 10 % of the group's lowest,
    √(Σ R_i² + 2 Σ_groups Σ_{i<j in the group} |R_i R_j|);
  - the ten percent method: √(Σ R_i² + 2 Σ |R_i R_j|), the second sum over every pair i < j with
    (f_j − f_i)/f_i ≤ 0.1, whether or not the two fall in one group;
  - the absolute sum: Σ |R_i|.
- **Directional rules** (`directional.json`): SRSS, 100-40-40 and the absolute sum (`ABSSUM`).
- **Repeated modes** (CLM-001, Repeated modes): the modes of one cluster (DEL-04-09's δ_c) are
  summed with their signs first and every rule is applied to the cluster sums. The only cluster in
  these files is the exactly repeated pair (`repeated_pair`); every other mode set has none, so its
  rules apply mode by mode as the scope's text does.

## Conventions the scope leaves open, as these files apply them

1. **Linear interpolation on a period table.** Both readings are published for T2 and give
   different values.
   - Primary: interpolation on the tabulated axis, in T = 1/f. In `spectrum_tables.json` it is the
     evaluation entry whose `reading` begins "interpolated on the tabulated axis"; in `sdof.json`,
     `reading: "tabulated"`.
   - Alternative: the tabulated periods are converted to frequencies 1/T and interpolated in f. In
     `spectrum_tables.json` it is the entry whose `reading` begins "tabulated periods converted to
     frequencies"; in `sdof.json`, `reading: "frequency"`.
   - A logarithmic abscissa gives the same value on either axis, so only the primary is published
     there.
2. **A query frequency on a period table at the range ends.** The query is the binary64 f. It maps
   to the exact rational T = 1/f, which is compared exactly with the tabulated periods. T2's periods
   are powers of two, so 1/T is exact in binary64 and the queries at the tabulated points and range
   ends land exactly on them. A product that rounds 1/f to binary64 for a period that is not a
   power of two can fall just outside a range end; the scope does not say how to map it.
3. **Equal largest components of opposite sign.** "The largest component is +1" does not fix the
   sign. The files take, as +1:
   - in the continuum, the maximum at the smallest x;
   - in the FE results, the lowest free DOF index;
   - in the tip-spring cases, the first global component in the order X, Y, Z.

   The case is flagged where it occurs. Γ's sign and the shape's sign depend on this choice; no
   response does.
4. **100-40-40 with fewer than three directions.** The absent direction contributes 0 (case D7).
5. **The 40 percent factor** is exactly 2/5. The binary64 0.4 differs from it by 5.5·10⁻¹⁷ relative.
6. **Rigid modes outside the tabulated range** are admitted. Only a periodic mode or f_r outside the
   range is refused (REQ-001), as in chain C.
7. **The grouping boundary** is inclusive: a mode joins the current group when f − f_low ≤ f_low/10.
   No published case lies on the boundary.
8. **Repeated-mode cluster tolerance.** The scope's repeated-mode rule (CLM-001, Repeated modes)
   uses DEL-04-09's cluster; these files take its tolerance as DEL-04-09 REQ-003 states it,
   δ_c = 2⁻²⁰ on eigenvalues: λ_{i+1} − λ_i ≤ δ_c λ_{i+1}. The only cluster in these files is the
   exactly repeated pair (`repeated_pair`). The tip-spring pairs are not clusters: their closest
   eigenvalues are 3·10⁻⁴ apart.

Readings these files add, which belong to the screening derivation:

- **Sensitivity (item 6).** A conditional SRSS sensitivity: one mode's frequency varies alone, with
  its shape, its participation and its force-recovery coefficient q_n held fixed. It excludes
  changes of the CQC coefficient, of the rigid response, of the shape and recovery, and rotations.
- **Missing mass on a coarse mesh.** The same f_r on an FE model can classify a different number of
  modes as periodic. Each case is flagged (`periodic_count_differs_from_continuum`), and
  `screening_basis.json` separates these cases.

## Item 1: spectrum tables (REQ-001, REQ-002; AC-002 "each interpolation rule")

- **T1** has frequency abscissae (0.3, 1.5, 6.25, 25, 80) Hz and acceleration ordinates
  (0.9, 3.6, 9.81, 7.5, 2.2) m/s².
- **T2** has period abscissae (2⁻⁵, 2⁻³, 2⁻¹, 2, 4) s and ordinates (0.35, 1.1, 0.8, 0.25, 0.1)
  as multiples of g = 9.80665 m/s².
- Both have damping ratio 0.05 and scale factor s = 1.5.

On a segment [x₀, x₁] with ordinates y₀ and y₁ (x the tabulated abscissa):

- the parameter is t = (x − x₀)/(x₁ − x₀) for a linear abscissa, or t = ln(x/x₀)/ln(x₁/x₀) for a
  logarithmic one;
- the ordinate is S = y₀ + (y₁ − y₀)t for a linear ordinate, or S = y₀ exp(t ln(y₁/y₀)) for a
  logarithmic one.

Further rules:

- A tabulated point returns its ordinate exactly.
- Linear on both axes gives an exact rational.
- Outside [x_first, x_last] the expected outcome is "refused, no extrapolation".
- Scaling by g commutes with every rule (checked). The scale factor multiplies the result.

Each table is evaluated at every tabulated point, at one strictly interior point per segment, at
both ends, and at two outside points.

## Item 2: single degree of freedom (AC-002 first and second items)

The oscillator has m = 125 kg and k = 2·10⁵ N/m, so ω = 40 rad/s exactly and f = 20/π Hz. Hence
φ = 1, Γ = 1, the effective mass is m, and the fraction is 1. Then:

- u = s S(f)/ω²;
- the spring force is k u, which equals m s S (a cross-check);
- the rigid-support reaction is K_cf u = −k u.

These are published under every rule for T1 and T2, with both period-table readings (convention 1).

The damping enters only through S. At the displacement extremum the velocity is zero, so the
published force and reaction are the same for any ζ.

**Information, not adopted.** For a viscous damper c = 2ζmω in parallel with the spring, the damper
force at the pseudo-velocity ωu is 2ζ k u, which is 10 % of the spring force at ζ = 0.05. A
harmonic estimate of the peak spring-plus-damper force is √(1 + 4ζ²) k u.

## Item 3: two-mass chain (AC-002 MDOF, CQC pair, rigid response, stiff model)

The chain is ground–k₁–m₁–k₂–m₂, excited along the chain:

- K = [[k₁+k₂, −k₂], [−k₂, k₂]], M = diag(m₁, m₂), R = (1, 1).
- The frequencies come from det(K − λM) = 0:
  m₁m₂λ² − (m₁k₂ + m₂(k₁+k₂))λ + k₁k₂ = 0.
- The shape is φ₁/φ₂ = (k₂ − λm₂)/k₂ (cross-checked from the first row), normalized so its
  largest-magnitude component is +1.
- Γ = φᵀM1/φᵀMφ and m_eff = (φᵀM1)²/φᵀMφ.

Quantities:

- the displacements u₁ and u₂;
- the spring forces k₁u₁ and k₂(u₂ − u₁), tension positive;
- the ground reaction −k₁u₁. This is the same whether k₁ is an element to a restrained node or a
  ground spring.

Per mode, the ground reaction per unit spectral acceleration equals −m_eff (checked).

The three parameter sets use the T1 log-log rule, s = 1.25 and ζ = 0.05:

- **A** (m₁, m₂, k₁, k₂) = (200, 100, 5·10⁵, 1.5·10⁵):
  - λ = 1000 and 3750 exactly (5.03 Hz and 9.75 Hz);
  - missing mass at f_r = 7 Hz: mode 2 is rigid, and the static solve reproduces Γ₂φ₂a_r/ω₂² with
    its sign (checked);
  - missing mass at f_r = 12 Hz, above both modes: the rigid part is exactly 0;
  - each modal rule is combined with the rigid part by SRSS and by absolute sum.
- **B** (1000, 4, 4·10⁵, 1600), a tuned-absorber pair:
  - frequency ratio 1.065, one group, ρ₁₂ = 0.714;
  - u₁ and the ground reaction have same-sign contributions, so CQC > SRSS;
  - the spring 2 force has opposite-sign contributions, so CQC < SRSS (checked).
- **C** (200, 100, 5·10⁸, 1.5·10⁸), at 159 Hz and 308 Hz:
  - without the correction the request is refused (periodic modes outside T1);
  - with f_r = 50 Hz both modes are rigid, and the response equals the static response to a_r:
    u₁ = a_r(m₁+m₂)/k₁ and u₂ = u₁ + a_r m₂/k₂ (checked).

## Item 4: directional combination (AC-002 "100-40-40, SRSS and absolute-sum")

The absolute sum is R_x + R_y + R_z (`ABSSUM`).

R_a + ⅖(R_b + R_c) = ⅗R_a + ⅖ΣR, so the permutation whose full direction has the largest magnitude
governs, and equal largest magnitudes tie exactly. The cases are:

- x, y and z governing in turn;
- an x–y tie;
- a triple tie;
- a zero direction;
- two directions requested.

Values are exact rationals, and SRSS is given in decimal with its exact square.

## Item 5: the pipe family (REQ-002 to REQ-004; benchmark method)

**Planar.** Each case is one bending plane: the pipe is restrained out of that plane, or only that
plane is considered. A circular pipe free in three dimensions has exactly repeated bending pairs
with an arbitrary basis. Applied mode by mode, per-mode contributions and SRSS are then basis
dependent. Under the scope's repeated-mode rule (CLM-001, Repeated modes: signed sum within the
cluster first), every rule is basis invariant. See the repeated pair below.

**Section and lengths.** The section has OD 21.3 mm, wall 2.77 mm, steel 7850 kg/m³, water
1000 kg/m³, E = 203 GPa and G = 78 GPa. Then:

- m̄ is metal plus contents;
- ī = ρ_pipe·I, with contents excluded (DEL-04-09 REQ-001);
- EI/m̄ and ī/m̄ are exact rationals (π cancels).

The two lengths are:

- **L = 2 m:** rotary inertia matters. It lowers the first three frequencies by 2.2·10⁻⁵ to
  4.7·10⁻⁴ relative.
- **L = 64 m:** rotary inertia is negligible. It changes the first three frequencies by less than
  10⁻⁶ in every span (largest 4.6·10⁻⁷, checked).

**Continuum.** In nondimensional form (x = X/L), W'''' + r²Ω²W'' − Ω²W = 0, where Ω² = ω²m̄L⁴/EI and
r² = ī/(m̄L²) (r² = 0 is Euler–Bernoulli). With W = e^{sx}:

- s⁴ + r²Ω²s² − Ω² = 0, so s² = a² or −b²;
- a², b² = (∓r²Ω² + √(r⁴Ω⁴ + 4Ω²))/2;
- W = c₁ cosh ax + c₂ sinh ax + c₃ cos bx + c₄ sin bx.

The variational form gives the end forces. With ½EI∫w''² as strain energy and ½∫(m̄ẇ² + īẇ'²) as
kinetic energy, the external transverse force and moment at x = L are:

- F_L = −(EI w''' + ω²ī w');
- M_L = EI w''.

At x = 0 they are the negatives.

| End | Conditions |
|---|---|
| clamped | W = 0, W' = 0 |
| pinned | W = 0, W'' = 0 |
| free | W'' = 0, W''' + r²Ω²W' = 0 |

The frequencies are roots of the 4×4 boundary determinant, and the shape is its cofactor null
vector. Two independent routes check them:

- the Euler–Bernoulli textbook equations:
  - cantilever, cos β cosh β = −1;
  - propped cantilever, tan β = tanh β;
  - fixed-fixed, cos β cosh β = 1;
- for the pinned-pinned span, Ω² = (nπ)⁴/(1 + r²(nπ)²) and W = sin nπx, checked against the
  determinant.

The shape is normalized at its true maximum on [0, 1]. Then:

- Γ = ∫W / D with D = ∫W² + r²∫W'²;
- the effective-mass fraction is (∫W)²/D (relative to m̄L).

The integrals are closed form, from the antiderivatives of products of cosh, sinh, cos and sin.
For example, ∫cosh(ax)cos(bx) = (a sinh ax cos bx + b cosh ax sin bx)/(a² + b²). Gauss–Legendre
quadrature checks them.

**Response quantities**, per unit spectral acceleration of mode n (each multiplied by s S(f_n)):

- displacement: Γ W(x) L⁴/((EI/m̄)Ω²), at the tip (C) or midspan;
- F₀ = Γ m̄L(W'''(0)/Ω² + r²W'(0));
- F_L = −Γ m̄L(W'''(1)/Ω² + r²W'(1));
- M₀ = −Γ m̄L² W''(0)/Ω²;
- internal moment M(x) = Γ m̄L² W''(x)/Ω²;
- internal shear V(x) = −Γ m̄L(W'''(x)/Ω² + r²W'(x));
- at x = L/2 and L/4 (V(L/2) for C and P only).

They are checked per mode by:

- force balance, F₀ + F_L + Γm̄L∫W = 0;
- total reaction equal to −m_eff;
- moment balance about x = 0, including rotary inertia: M₀ + M_L + L F_L + Γm̄L²(∫xW + r²(W(1) − W(0))) = 0.

**Static response** to a uniform load m̄ per unit acceleration (Euler–Bernoulli statics; rotary
inertia has no static effect): ŵ'''' = 1 with the span's end conditions, solved exactly. It is
checked against the textbook values:

- qL⁴/8EI, qL⁴/192EI (propped midspan), 5qL⁴/384EI and qL⁴/384EI;
- the end reactions −qL, −5qL/8, −3qL/8 and −qL/2;
- the moments −qL²/2, −qL²/8 and −qL²/12.

**Missing mass.** Truncation after 1, 2, 3 and 5 modes uses f_r = 1.01 f_n of the Rayleigh
continuum, rounded to 6 significant digits (each a binary64 input). The continuum totals are given
for the SRSS, CQC and grouping modal rules, each combined with the rigid part by SRSS and by
absolute sum.

**Spectra.** Both use damping ratio 0.05 and s = 1:

- FLAT: 2.5 m/s² from 10⁻⁴ to 10⁶ Hz.
- TWO_SEGMENT_L2: (2, 0.5), (32, 4.0), (8192, 4.0) Hz, m/s², log-log. The rising segment's slope
  is d ln S/d ln f = 3/4.
- TWO_SEGMENT_L64: the same table at 1/1024 of the frequencies.

**Finite elements** (`fe.py`):

- **Element matrices.** Hermitian cubic stiffness, consistent mass m̄h/420[156, 22h, 54, −13h; …]
  and rotary inertia ī/(30h)[36, 3h, −36, 3h; …]. They are integrated exactly from the shape
  functions and checked against the printed forms.
- **consistent**: the product's formulation (DEL-04-09 REQ-001), restricted to one plane.
- **lumped**: m̄h/2 at each end translation and no rotary inertia. This is not the product's
  formulation; it only measures that modelling difference.
- **Eigen-solution.**
  - The pencils are formed per unit m̄ as exact rationals.
  - Consistent: Cholesky reduction, then cyclic Jacobi in decimal.
  - Lumped: the massless rotations are statically condensed first.
  - Every eigenvalue is re-found by bisection on banded LDLᵀ Sturm counts.
- **Mode counts.** `finite_modes` per planar model is the count of the full eigen-solution, checked
  against the formulation:

  | Formulation | Cantilever | Propped | Pinned-pinned | Fixed-fixed |
  |---|---|---|---|---|
  | consistent (every free planar DOF) | 2N | 2N − 1 | 2N | 2N − 2 |
  | lumped (the free translations) | N | N − 1 | N − 1 | N − 1 |

- **Excitation.** p = (M R)_f, including the consistent coupling to restrained DOFs.
- **Rigid response.** The literal static solve, checked two ways:
  - its displacement equals the sum over non-periodic FE modes of Γφ/λ (the FE basis is complete);
  - its translational reactions balance the total static load.
- **Deviations** are taken against the Rayleigh continuum (the consistent FE's limit), mode by mode.
  Combined values are compared with the continuum combined over the same number of modes, since a
  coarse mesh has fewer than five finite modes.

**Benchmark model** (`beams_fe.json`, `benchmark_model_merged_frequencies`). One pipe along X is
fully fixed at A, with only UZ restrained at the tip B:

- excited along Y it is the cantilever, with results in UY and RZ;
- excited along Z it is the propped cantilever, with results in UZ and RY.

Its merged ascending frequency list is given per length, per N and per formulation, with each
mode's family: Y bending, Z bending, X axial, RX torsion.

- **List rule and cutoff.** The cutoff is the larger of the two planes' fifth bending frequency (the
  last, where a plane has fewer). The list holds the first five bending modes of each plane, every
  further bending mode at or below the cutoff, and every axial and torsional mode at or below it.
- **Completeness.** For the FE models every finite bending mode is built. For the continuum, and
  for the rods, the generator checks that each family's first mode not built (continuum bending
  mode 6, rod mode 4) lies above the cutoff (`MERGED-list-rule`). The list therefore holds every
  mode below the cutoff.
- **No totals.** The list is not a mode inventory and publishes no mode total.

- The axial and torsional modes are uncoupled and unexcited, but count as modes.
- Their FE frequencies come from closed forms bracketed by Sturm counts. For a consistent
  fixed-free rod, λ̂ = 6N²(1 − cos θ)/(2 + cos θ); for a lumped one, 2N²(1 − cos θ); in both,
  θ = (2j−1)π/(2N).
- The lumped model has no torsional inertia, so it has no torsion modes.

No Y–Z pair falls within 1 % at any N or length (the flag list is empty everywhere). At L = 2 m:

- consistent N = 4 places torsion mode 1 between Y5 and Z5 (at L = 64 m the axial and torsional
  modes lie far above the bending modes);
- N = 2 and lumped N = 4 have fewer than five modes in a plane.

## Item 6: sensitivity

These are conditional SRSS sensitivities. The reading holds mode n's shape, its participation and
its force-recovery coefficient q_n (the force, moment or reaction of the mode per unit spectral
acceleration, divided by Γ_n) fixed, and varies only its frequency:

- displacement contributions scale as S(f_n)/f_n², so d ln Q_n/d ln f_n = σ_n − 2;
- force, moment and reaction contributions are Γ_n q_n s S(f_n) with q_n fixed, so they scale as
  S(f_n) and the derivative is σ_n;
- σ_n = d ln S/d ln f at f_n: 0 on a flat segment and 3/4 on the rising one.

For SRSS, d ln R/d ln f_n = (Q_n/R)² · d ln Q_n/d ln f_n. A central difference with relative step
10⁻²⁰ through the table rule checks it. This is given for every item 5 quantity and mode (Rayleigh
continuum) under both spectra.

They exclude changes of the CQC coefficient, of the rigid response, of the shape and recovery, and
rotations. The largest magnitudes are
2.0 (displacement, flat), 1.25 (displacement, sloped) and 0.75 (force, moment and reaction,
sloped). On the flat spectrum, forces, moments and reactions are 0.

## Additions: closely spaced pair from a tip spring, and the repeated pair

**General form** (`beam_skew_and_pairs.json`). The cantilever lies along a, with orthonormal
bending planes p and q. A tip spring k_eff acts in plane p only, and the excitation direction has
components c_p and c_q.

- Plane p is a cantilever with a tip spring. With κ = k_eff L³/EI and the free end's shear
  condition W''' + r²Ω²W' − κW = 0, the Euler–Bernoulli frequency equation is
  β³(1 + cos β cosh β) − κ(cos β sinh β − sin β cosh β) = 0 (derived from
  W = (cosh − cos) + B(sinh − sin); checked against the determinant route).
- Plane q is the plain cantilever.

Per unit spectral acceleration, with A_n = c_j(∫W/D)/ω_n²:

- displacement field: A_n W(x) ĵ;
- effective mass: c_j² m̄L(∫W)²/D;
- clamp force: (EI/L³)A_n W'''(0) ĵ;
- clamp moment: −(EI/L²)A_n W''(0)(a × ĵ);
- spring reaction: −K_s u_tip.

Contributions to the excitation component carry the sign of c_j², the same for both planes.
Contributions to the perpendicular component carry the sign of c_p c_q, opposite for the two
planes. Force and moment equilibrium in global components is checked per mode.

There are two parameter sets, both with L = 2 m, T1 log-log, s = 1, ζ = 0.05 and three modes per
plane. k_eff = 57 N/m (κ = 0.3175) splits the first pair by 4.995 %.

- **(a)** Pipe along X, spring along (0,1,1) with k = 57 N/m, excitation along Y:
  c_p = c_q = 1/√2.
- **(b)** Pipe along (1,1,1)/√3, spring along X with k = 85.5 N/m, excitation along Y:
  - p = (2,−1,−1)/√6 and q = (0,1,−1)/√2;
  - k_eff = 2k/3 = 57 N/m exactly;
  - c_p = −1/√6 and c_q = 1/√2.

  The pipe is taken as axially rigid, so the clamp carries the spring's axial component.

  **Case (b) is comparison only, not for verification at the analytic tolerance** (its JSON block
  carries `use` saying so). The neglected coupling lowers the first plane-p frequency by about
  8.1·10⁻⁸ relative. It changes the plane-p mode 1 contributions by about 10⁻⁷, a hundred times the
  1e-9 verification tolerance, and the coupled modes acquire axial components that participate
  through c_axial. Case (b) is cross-engine comparison data and cannot verify a product model with
  finite axial stiffness at 1e-9. Case (a) is unaffected (d·a = 0).

  The neglected axial flexibility reduces the spring seen by plane p by
  δ = k d_a²(L/EA)/(1 + k d_a² L/EA) = 1.74·10⁻⁶. Its first-order effect on a plane-p frequency is
  −δκW(1)²/(2Ω²D), at most −8.1·10⁻⁸ relative (mode 1). The ratio of axial to bending tip
  flexibility is 3I/(AL²) = 3.3·10⁻⁵.

Both sets are published under Euler–Bernoulli and Rayleigh. The tip displacement and the clamp and
spring reactions are given in global X, Y and Z, per mode and under all five modal rules. In both
sets the first pair contributes with the same sign to uY and with opposite signs to uZ.

**Repeated pair.** A circular cantilever free in three dimensions has mode 1 as an exactly repeated
pair, one shape in each bending plane.

*A, the in-plane response.* Take the Euler–Bernoulli cantilever of the item 5 section at L = 2 m:

- Ω₁ = β₁² with cos β cosh β = −1 (β₁ = 1.8751…);
- ω₁ = Ω₁√(EI/m̄)/L² and f₁ = ω₁/(2π) (4.387 Hz);
- W normalized so that W(1) = +1 (the tip is its maximum).

A = (∫₀¹W / ∫₀¹W²) · W(1) · s S(f₁)/ω₁², with S from T1 under log-abscissa/log-ordinate and s = 1.
It is the tip displacement of the mode under excitation in its own plane. The file gives A,
`frequency_hz` and the derivation in `derivation_of_A`.

*The rotated basis.* With e₁ = (cos θ, sin θ) and e₂ = (−sin θ, cos θ) in (Y, Z) and excitation
along Y, each mode's participation along Y is proportional to its Y component. So mode e₁
contributes c₁ = cos²θ·A to uY and cos θ sin θ·A to uZ, and mode e₂ contributes c₂ = sin²θ·A and
−sin θ cos θ·A.

**Adopted rule** (DEL-04-10 CLM-001, Repeated modes). The two modes form one cluster (equal
eigenvalues, within δ_c), so their signed contributions are summed first, c₁ + c₂, and every modal
rule is applied to that single cluster sum. A rule applied to one value v returns |v|, and the
cluster's frequency stands for it in CQC, grouping and the ten percent method:

- uY: |cos²θ + sin²θ|·A = A;
- uZ: |cos θ sin θ − sin θ cos θ|·A = 0;
- for every rule and every θ.

These are the values a product is compared with. Each row gives them per rule as
`adopted_cluster_sum_first`, equal to `basis_invariant_sum_first` (checked).

**Mode by mode, NOT the adopted reading, published to show the basis dependence.** In each row, the
fields `SRSS`, `CQC`, `GROUPING`, `ABSSUM` and `TEN_PERCENT`, and the `closed_forms` and `extremes`
entries named after them, apply each rule to the two modes separately:

| Mode by mode (not the adopted reading) | Closed form in θ | Range |
|---|---|---|
| SRSS of uY | A√(cos⁴θ + sin⁴θ) | A/√2 to A |
| SRSS of uZ | A\|sin 2θ\|/√2 | 0 to A/√2 |
| grouping (and absolute sum) of uZ | A\|sin 2θ\| | 0 to A |

Mode-by-mode CQC with ρ = 1 happens to equal the invariant value for every θ. Values are given at
θ = 0, 15, 22.5, 30, 45, 60, 67.5 and 90°. The invariance of the cluster sum is exact for an exactly
repeated eigenvalue and an M-orthogonal basis, as here.

## Coverage of AC-002

| AC-002 item | Where |
|---|---|
| SDOF under each interpolation rule and the pseudo-acceleration convention | `sdof.json` |
| damped force and reaction at the displacement extremum | `sdof.json` (and the information term) |
| one-element consistent-mass and restrained-mass witnesses | not here: left to the `discrete` package. Item 5's N = 2 FE and the consistent rigid reactions (mass coupled to restraints) are related but are not the one-element witness |
| ground-spring reaction beside a rigid-end reaction, in equilibrium | tip-spring case (a): spring and clamp reactions with per-mode equilibrium (case (b) is comparison only) |
| MDOF modal responses and each modal combination; CQC on a close pair with opposite-sign contributions | `two_mass_chain.json` A and B; the tip-spring pair |
| an exactly repeated pair in two bases giving one combined response under every rule | `repeated_pair` (rotated bases at eight angles; `adopted_cluster_sum_first`) |
| 100-40-40, SRSS and absolute-sum directional | `directional.json` |
| rigid response with an extracted rigid mode in the static solve; stiff model equal to the static response | `two_mass_chain.json` A (f_r = 7 Hz) and C |

Not covered here:

- the grouping and ten percent methods on a set of modes where the two differ: every published set
  gives equal values;
- a cluster of two modes with different frequencies within δ_c, including a quantity whose two
  contributions nearly cancel;
- a cluster that straddles f_r;
- AC-003 (snubbers) and AC-009 (independent support motion).

## Cross-checks

Each JSON file carries its own `cross_checks`. The principal ones are:

- the tables recomputed from the segment's other end, and g scaling commuting with interpolation;
- SDOF and chain balance (k u = m sS; reaction = −m_eff sS);
- the chain shape from both rows, Viète's relations and Σm_eff = total mass;
- the rigid response against the rigid modes' sum, and against the hand static solution;
- 100-40-40 against its closed form;
- beam roots against the textbook equations (≈10⁻⁶⁵);
- closed-form integrals against quadrature (≈10⁻⁶⁶);
- per-mode force, reaction and moment balance (≈10⁻⁸²);
- static coefficients against the textbook values (exact);
- FE: element matrices exact; Jacobi against Sturm bisection (≈10⁻⁴¹); eigen-residuals (≈10⁻⁴⁹);
  reaction = −m_eff; rigid modal and balance checks;
- rod closed forms against Sturm counts;
- the benchmark-model merged lists against their list rule (every family's first unbuilt mode above
  the cutoff);
- the sensitivity finite difference;
- the tip-spring equation, equilibrium in 3D, and, for the repeated pair, CQC invariance and every
  rule under the adopted reading equal to the invariant value.

## Limits

- These are class (b) references, plus the generator's own FE. The consistent FE at a given N is the
  product's planar formulation, so a product model of the same N should reproduce those values
  closely. Agreement at the verification tolerance also depends on the product's operation order.
  The lumped FE is a comparison device only.
- Euler–Bernoulli and Rayleigh beams only: no shear deformation and no Timoshenko mass (DEL-04-09
  refuses Timoshenko straights).
- Skew case (b) is comparison only: its axially rigid idealization leaves an error of about 10⁻⁷ in
  the plane-p mode 1 contributions, above the 1e-9 verification tolerance.
- The screening summary is data. It sets no screening class, verification tolerance or comparison
  tolerance.

## For the screening derivation

`screening_basis.json` (`definitions.for_the_screening_derivation`) states:

- The sensitivity is a conditional SRSS sensitivity: shape, participation and force-recovery
  coefficient q_n held fixed. It excludes changes of the CQC coefficient, of the rigid response, of
  the shape and recovery, and rotations.
- On a coarse mesh the same rigid frequency can classify a different number of modes as periodic.
  `missing_mass_total` includes those cases and `missing_mass_total_same_classification` excludes
  them.
- What the "largest" maxima leave out: the quarter-point quantities at N = 2, where L/4 is not a
  node, and quantities whose continuum value is zero by symmetry.
- `per_mode` excludes contributions whose continuum value does not exceed 10⁻³⁰ times the largest
  absolute continuum contribution of that quantity (over the five continuum modes).
- The significance threshold of `per_mode_significant` is 1 % of the continuum SRSS over
  nm = min(5, the FE model's finite mode count) continuum modes, not always five.
- At L = 64 m, "rotary inertia negligible" applies to modes 1–3 only. Modes 4 and 5 shift by
  6.6·10⁻⁷ to 1.23·10⁻⁶ (mode 5 above 10⁻⁶ in every span), and the five-mode categories include them.
