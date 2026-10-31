# F1 modal references: continuum (class b), mesh convergence and inertia-risk witnesses

Frozen, independently derived expectations for DEL-04-09, F1 modal analysis
([design](../../../../execution/PKG-04/DEL-04-09/Work/modal-analysis.md) §2.2,
§2.6, §3.3, §3.4 check 4, §4.1–§4.3; AC-003, AC-004). They give:

- closed-form continuum frequencies for beams, rods, lumped controls, a
  cantilever with a tip mass and free rings;
- the matched consistent-mass finite-element frequencies at N = 1, 2, 4, 8, 16
  (and N = 1…16 for the coarse-mesh data), so a product test can show the
  convergence rate;
- small pencils that provoke unpivoted-LDLᵀ inertia errors, each with exact
  eigenvalue counts below stated shifts.

**Status: references with a refutation record.** The author TASK derived and
cross-checked them, and a second TASK, reading no product code, ran 3,785
independent checks without refuting any value ([REFUTATION.md](REFUTATION.md)).
It refuted two descriptive statements (the lumped control's convergence rate
and two premise remarks), which are corrected here; no value changed. No
product output has been compared with them.

**Independence.** The generators import, read and run no product code (nothing
under `core/`, `apps/`, `api/`, `tests/`, `schemas/` or `fixtures/`). All model
data are invented. No CAEPIPE or other proprietary data is used. The formulas
are standard textbook results, restated and re-derived here (see
[Derivations](#derivations)), not copied from tables. Standard claim fence
applies (claims taxonomy per DEC-081).

## Files

| File | Role |
|---|---|
| `generators/numerics.py` | shared numerics: decimal π, sin and cos, bisection, exact rational inertia, banded LDLᵀ counts, JSON encodings |
| `generators/continuum.py` | items 1–6: beams, rods, Rayleigh beams, lumped controls, tip mass, rings → `continuum.json` |
| `generators/fe_convergence.py` | item 7: the generator's own elements, refinement tables, coarse-mesh data → `fe_convergence.json` |
| `generators/inertia_witnesses.py` | item 8: inertia-risk pencils and the round-2 witness → `inertia_witnesses.json` |
| `generators/generate.py` | driver: `--check` (default) regenerates in memory and compares byte for byte; `--write`; `--search-round2` |
| `continuum.json`, `fe_convergence.json`, `inertia_witnesses.json` | the references; each carries its own `cross_checks` |
| `SHA256SUMS` | hashes of every file above (`shasum -a 256 -c SHA256SUMS` in this directory) |

To check or regenerate (standard library only, Python ≥ 3.9, about 11 s):

    python3 generators/generate.py --check
    python3 generators/generate.py --write
    python3 generators/generate.py --search-round2

## Method and number format

- **Inputs** are binary64 numbers. Each is published as `binary64` (the
  shortest round-trip decimal), `hex` and `exact` (the exact rational of the
  binary64 value, from `fractions`).
- **Transcendental quantities** use `decimal` at 90 digits. π comes from
  Machin's formula; sin and cos are Taylor series after reduction modulo 2π;
  exp, sqrt and ln come from `decimal`, which rounds them correctly. Roots are
  found by bisection on a sign change. A binary64 scan with `math` locates
  each bracket, and the sign change is then confirmed in `decimal`. No libm
  value enters a published number.
- **Published reals** have 40 significant digits (`value`) plus the nearest
  binary64 (`binary64`). Finite-element eigenvalues have 36 digits.
- **π-free ratios are exact.** EI/m̄, EA/m̄, GJ/j̄, ī/m̄, A/I and GJ/EI are
  rationals of the binary64 inputs, because π cancels. All nondimensional
  pencils are therefore exact rationals.
- **Exact counts.** Each count is the number of eigenvalues of (K, M) below a
  shift, and the number equal to it. It comes from unpivoted LDLᵀ on exact
  rationals. On an exact zero pivot the generator falls back to symmetric
  elimination with 1×1 and 2×2 pivots, whose inertia is exact by Sylvester's
  law.

## Premises and mapping

- **Sections** (`continuum.json` → `sections`). SEC-A has OD 0.2 m, wall
  0.008 m, steel 7850 kg/m³, contents 1000 kg/m³, no insulation, E = 200 GPa
  and G = 80 GPa. SEC-B has OD 0.1143 m, wall 0.00602 m, 7900 kg/m³, contents
  850 kg/m³, insulation 0.05 m at 120 kg/m³, E = 195 GPa and G = 75 GPa.
  - Stiffness and mass use the same wall, with no mill tolerance or corrosion
    allowance (c_m = c_a = 0).
  - I = π(OD⁴ − ID⁴)/64 and J = 2I, both on the metal only.
  - m̄ = metal + contents + insulation.
  - j̄ = ρ_p·J + ρ_ins·J_ins, with J_ins the polar moment of the insulation
    annulus, and ī = j̄/2. Contents are excluded from both (D-e).
  - Insulation has mass and polar inertia but no stiffness.
- **Nondimensional results are primary.** Beam values are Ω = ω√(m̄L⁴/EI);
  rod values use √(m̄L²/EA) or √(j̄L²/GJ); ring values are
  λ = ω²m̄R⁴/EI. Every dimensional case also gives EI/(m̄L⁴) or EI/(m̄R⁴)
  exactly, so a test can rescale for any section.
- **Planar results.** Each beam result is for one bending plane. In a 3D
  model of a circular pipe, every bending frequency appears twice, once in
  each plane. Axial and torsional frequencies appear once.
- **The product's continuum limit is the Rayleigh beam.** The design's
  straight element always carries the rotary-inertia matrix (§2.2), so a
  refined product mesh converges to the Rayleigh values, not to the
  Euler–Bernoulli ones. For SEC-A at L = 6 m the gap ranges from −1.75·10⁻⁴
  (cantilever mode 1) to −1.4·10⁻² (fixed-fixed mode 6) in frequency. Class (b)
  comparisons of the product should target `rayleigh` cases, or use a
  tolerance wider than this gap; `rel_shift_from_euler_bernoulli` gives it.
- **SEC-A at L = 1 m** (L/OD = 5) is a formulation check only. Shear
  deformation is neglected, and a Timoshenko beam would differ far more
  (Timoshenko is out of scope here, M4).

## Cases

### `continuum.json`

1. **Euler–Bernoulli beams**, `euler_bernoulli`. The first six roots of each
   frequency equation, to 40 digits:
   - cantilever: cos x·cosh x = −1;
   - simply supported: x = nπ;
   - fixed-fixed: cos x·cosh x = 1.

   The frequencies are realized for SEC-A at L = 6 m, SEC-B at 4.5 m and
   SEC-A at 1 m.
2. **Rods**, `rods`. Axial and torsional, cantilever ((2n−1)π/2) and
   fixed-fixed (nπ), four modes, for SEC-A at 6 m and SEC-B at 4.5 m. Axial
   uses m̄, contents included; torsion uses j̄, contents excluded.
3. **Rayleigh beams**, `rayleigh`.
   - Simply supported, in closed form: Ω² = (nπ)⁴/(1 + r²(nπ)²), with
     r² = ī/(m̄L²).
   - Cantilever and fixed-fixed, from the boundary determinant. These are
     supplementary, because the product converges to them.
   - Each for the same three span realizations.
4. **Lumped controls**, `lumped_controls`.
   - A single DOF: ω² = k/m.
   - A point mass on a massless cantilever: ω² = 3EI/(M_tL³). This case also
     gives its one-element kernel pencil (M singular) with exact counts at ω²
     and ω²(1 ± 2⁻⁴⁰).
   - Fixed-free axial chains with N = 3, 5 and 10 equal masses:
     ω_j = 2√(k/m)·sin((2j−1)π/(2(2N+1))). The formula is confirmed by exact
     Sturm counts.
5. **Tip mass**, `tip_mass`. Six roots each for μ = M_t/(m̄L) = 0, 1/10, 1/2,
   1, 2 and 10, plus a dimensional case: SEC-A at 6 m with W = 2000 N and
   a = 9.80665 m/s², so M_t = W/a. The tip is a point mass at the free end,
   with no rotary inertia and no offset.
6. **Free rings**, `rings`, SEC-A at R = 2 m and SEC-B at R = 1 m.
   - For n = 2…6 (each a double eigenvalue), five variants:
     - `inplane_hoppe`;
     - `inplane_inextensional_rotary`;
     - `inplane_extensible_rotary` (flexural and extensional branches);
     - `outofplane_love`;
     - `outofplane_torsion_rotary` (flexural and torsional branches).
   - `complete_low_spectrum` lists every mode of the refined model below the
     lowest branch of any n > 8, with multiplicities and running counts: the
     6 rigid modes, the n = 0 breathing and uniform-twist modes, and the n ≥ 1
     branches. It is the list a kernel-level count with a negative shift can
     be compared with.
   - Premises: a thin Euler–Bernoulli curved bar, with no shear, warping or
     ovalization (flexibility factor 1).

### `fe_convergence.json`

- **The generator's own elements.** Hermitian cubic beams with consistent
  mass, optionally with the Rayleigh rotary-inertia matrix, and linear rods
  with consistent mass. The element matrices are integrated exactly from the
  shape functions (`elements`) and match Przemieniecki (1968, ch. 11) and
  design §2.2.
- **Families:**
  - Euler–Bernoulli cantilever, simply supported and fixed-fixed;
  - Rayleigh beams at r² of SEC-A with L = 6 m and L = 1 m;
  - tip mass for the five nonzero μ;
  - rods, cantilever and fixed-fixed, in closed form
    Ω² = 6N²(1 − cos t)/(2 + cos t) and checked by exact counts;
  - a lumped-mass internal control (simply supported).
- **What each row gives.** For each N, it gives Ω², the relative frequency
  error against the continuum and the exact Sturm brackets: counts k−1 and k
  at λ_k(1 ∓ 10⁻³⁰), as decimal shifts. `observed_order` is
  log₂|e_N/e_2N|.
- **Expected rates.** Consistent Hermitian beams have O(h⁴) frequency error
  from above (Rayleigh–Ritz). The observed order is 3.99 for mode 1 at
  N = 8 → 16, and every consistent row lies above the continuum (checked).
  Linear rods converge at O(h²), observed 2.00. The lumped control converges
  from below here, with no bound; for this simply supported span its observed
  order is about 4 (4.57, 4.15, 4.04 for mode 1), not 2 (corrected after
  refutation; a lumped cantilever does show order 2).
- **Coarse-discretization data** (`coarse_discretization`), for the warning
  factor c of §2.6.
  - Rows cover every Euler–Bernoulli case at N = 1…16 and modes
    1…min(6, dofs), each with f_e/f_n = (4.7300407…·N/(β_nL))² and the
    frequency error.
  - `summary` gives the largest error among rows with f_e/f_n ≥ c, for
    c = 1…10:

    | c | 2 | 2.5 | 3 | 4 | 6 | 8 | 10 |
    |---|---|---|---|---|---|---|---|
    | largest error (all supports) | 11.0 % | 2.9 % | 2.9 % | 1.8 % | 0.85 % | 0.41 % | 0.30 % |

  - The value at c = 2 comes from one simply supported element per span.
  - These are data, not a choice of c.

### `inertia_witnesses.json`

Each witness has binary64-exact K and M and binary64 shifts. For every shift
it gives the exact count below and equal, and the observations of an
unpivoted binary64 LDLᵀ, unscaled and with D = diag(|k_ii|^-½):
- the count;
- min and max |d|;
- max |L|;
- g = max(1, max|L|)²·max|d|/max|a_jj|;
- whether the 1e-12 pivot guard would trip.

The observations follow the operation order stated in the file. They
illustrate the hazard and are not references: another operation order rounds
differently.

- **WIT-MISSED.** K = I, M = diag(1, 2, 3), with eigenvalues 1/3, 1/2 and 1.
  A solver that misses 1/3 meets count 3 ≠ 2 at shift 1.5. The shift 0.5 lies
  exactly on an eigenvalue: equal count 1, and unpivoted breakdown.
- **WIT-NEAR-PAIR and WIT-CLUSTER.** K = BᵀΛB and M = BᵀB with integer B, so
  the eigenvalues are exactly Λ:
  - a pair 2⁻⁴⁰ apart;
  - a cluster of four at spacing 2⁻³⁶.

  Shifts lie in each gap and an ulp-scale step from eigenvalues.
- **WIT-SCALE-1E3 and WIT-SCALE-1E6.** Spring chains whose stiff pair is
  scaled by 10³ or 10⁶ (rigid-family scales). Shifts lie at λ(1 ± 10⁻⁸),
  λ(1 ± 10⁻¹³) and at the binary64 nearest each eigenvalue. Their eigenvalues
  are confirmed by exact counts. Binary64 miscounts occur at 10⁻¹³ offsets
  with and without the diagonal scaling.
- **WIT-ZERO-PIVOT.** At a shift that is not an eigenvalue, the first pivot is
  exactly zero; at 2 ± 2⁻³⁰, |L| is about 2³⁰.
- **WIT-FE-CANT4.** A four-element consistent-mass cantilever pencil, with M
  scaled by 420/h so that every entry is binary64-exact.
- **WIT-ROUND2: Codex's round-2 witness (§3.3).**
  - **Construction.** A 32×32 pencil: K has a unit diagonal and off-diagonal
    −0.45, except K₀₁ = −0.45 − 38·2⁻⁴⁰; M = diag(round(2¹⁶·2^(−i/8))/2¹⁶).
    D = I, so the product's scaling changes nothing.
  - **The shift.** σ = 0.29164152891510925 (0x1.2aa413b36221fp-2) lies
    7.0·10⁻¹⁹ relative below λ₂ = 0.29164152891510924956863854927….
  - **The miscount.** The exact count at σ is 1. Each of three binary64
    LDLᵀ variants counts 2:
    - the Sturm form (b²/d);
    - the l·b form (the dense routine's order);
    - the l²·d form.

    In each, the last pivot is about −1.4 to −3.2·10⁻¹⁰, so the 1e-12 guard
    does not trip.
  - **Growth.** max|L| = 4.356 and g = 22.58 at σ agree to within 6·10⁻⁸ with
    their values at λ(1 ± 10⁻⁹), where every count is right.
  - **Rounding away.** The mirrored shift 2λ − σ, true count 2, rounds to σ.
  - **The search.** The frozen kk = 38 is the first hit of a deterministic
    search, which `--search-round2` re-runs.
  - **Consequence.** A product test should assert that the separation rule
    refuses this shift. It should not assert a particular binary64 count.

## Derivations

Every formula was re-derived; none is taken on trust from the design.
- **Euler–Bernoulli** (Timoshenko, Young & Weaver, *Vibration Problems in
  Engineering*; Blevins, *Formulas for Natural Frequency and Mode Shape*, ch. 8).
  - The equation is EIw'''' + m̄ẅ = 0, with W = A cosh βx + B sinh βx + C cos βx
    + D sin βx and β⁴ = m̄ω²/EI.
  - The cantilever, fixed-fixed and simply supported determinants reduce to
    1 + cos·cosh = 0, 1 − cos·cosh = 0 and sin = 0.
  - The design's §4.2 roots are correct to the digits stated (largest
    difference 2.9·10⁻¹⁰).
- **Tip mass.** At the free end, w'' = 0 and EIw''' + M_tω²w = 0, from
  Hamilton's principle with the tip's kinetic energy. With W = a(cosh − cos)
  + b(sinh − sin), the 2×2 determinant is
  2(1 + cos λ cosh λ) + 2μλ(cos λ sinh λ − sin λ cosh λ) = 0.
  The design's equation is correct.
- **Rayleigh beam.**
  - The equation is m̄ẅ − īẅ'' + EIw'''' = 0. The free-end shear condition
    is EIw''' + īω²w' + M_tω²w = 0.
  - With W = sin αx, EIα⁴ = ω²(m̄ + īα²). The design's limit is correct, and
    it is also the κG → ∞ limit of the design's Timoshenko equation.
  - In the independent route, the characteristic roots are e^{sx} with
    s⁴ + r²Ω²s² − Ω² = 0.
- **Rods.** u(0) = 0 with u'(L) = 0 gives cos kL = 0; u(0) = u(L) = 0 gives
  sin kL = 0.
- **Chain.** For the fixed-free chain, u_i = sin(iθ) satisfies every interior
  equation with λ = 2(k/m)(1 − cos θ). The free end is the interior equation
  with a ghost u_{N+1} = u_N, so cos((2N+1)θ/2) = 0 and θ = (2j−1)π/(2N+1),
  giving λ = 4(k/m)·sin²((2j−1)π/(2(2N+1))). For the consistent linear rod
  elements, the free end is the interior equation with u_{N+1} = u_{N−1},
  so Nθ = (2j−1)π/2.
- **Rings** (Love, *Treatise on the Mathematical Theory of Elasticity*,
  §§ 293–294; Hoppe 1871; Blevins ch. 9). A section's rotation is
  Ω = φt + (w'/R)n.
  - **Out of plane.** The bending curvature is κ = (w'' − ψ)/R² and the twist
    τ = (ψ' + w')/R², with ψ = Rφ and primes taken in θ. The kinetic energy is
    m̄ẇ² + j̄φ̇² + ī(ẇ'/R)².
    - Condensing ψ with j̄ = ī = 0 gives exactly
      λ = n²(n²−1)²/(n² + EI/GJ). This is Love's formula, and the design's
      formula is correct (checked as an exact rational identity).
    - The formula's premise, neglected torsional and rotary inertia, is
      stated in the design (§4.2, as revised). With j̄ and ī included, the flexural branch is
      lower by 0.67 % in λ at n = 2 and by 3.0 % at n = 6, for SEC-A at
      R = 2 m.
  - **In plane.** The strain is ε = (v' + w)/R, the curvature
    κ = (w'' − v')/R² and the rotation χ = (w' − v)/R. The inextensional
    limit with ī = 0 is Hoppe's n²(n²−1)²/(n²+1), and the design's formula is
    correct. With rotary inertia and extension, λ is lower by 0.42 % at n = 2
    and by 2.6 % at n = 6.

## Cross-checks (all recorded in the JSON files; 76 agree, 0 disagree)

- Each frequency-equation root agrees with an independent 4×4
  boundary-condition determinant on the general solution to about 10⁻⁶⁰
  relative. The same holds for the Rayleigh simply supported closed form, so the
  determinant route is validated for r² > 0 before its cantilever and
  fixed-fixed roots are used.
- The equation residuals are below 10⁻⁵⁸ at the 60-digit roots.
- The design's §4.2 roots and both of its §2.2 figures are confirmed. One
  consistent simply supported element gives √120 = 10.954, 11.0 % high; two
  lumped elements give √96, 0.73 % low.
- The tip mass with μ = 0 equals the cantilever. The Rayleigh-quotient
  estimate √(3/(μ + 33/140)) lies above mode 1 by less than 1.5 %, and by
  7.7·10⁻⁶ at μ = 10.
- For rings, Love's formula equals the condensed pencil exactly. The
  extensible pencil at α = 10⁴⁰ reproduces Hoppe and the inextensional rotary
  formula to 10⁻³⁹.
- For the chains and the tip pencil, the closed forms agree with exact Sturm
  counts.
- Every consistent-mass FE frequency lies above the continuum. The exact
  integrals of the element matrices equal the design's matrices.
- The witness counts equal the constructed eigenvalues. For the round-2
  witness, three checks pass: unchanged growth, correct counts at the
  separated probes, and rounding away.

## Limits

- **Classes.** These are class (b) continuum references, plus the generator's
  own matched FE (class a for these elements only). They do not check the
  product's arc stiffness, its half-arc lumps or its component bodies.
- **Ring convergence target.** The product's half-arc rigid lumps carry the
  section's rotary inertia (design §2.4, as revised), so a refined ring tends
  to the refined variants; Hoppe and Love are slender-limit controls.
- **Witness observations.** The binary64 observations are this generator's
  own operation order. The exact counts are the references.
- **Not decided here.** The comparison tolerances, the warning factor c and
  the growth and separation parameters (O-6) are not decided here.
