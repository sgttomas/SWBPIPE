# MECH-EXPANSION-LOOP-CURVED-BEND-THERMAL

## Purpose

Expansion-loop (L-bend) thermal-bending benchmark with a hand-calculated
known-flexibility reference, authored as the independent reference required by
the D-34 exit-evidence bar (`DEC-070`, packet
`execution/_Coordination/_DECISIONS/D-34_bend_flexibility_stiffness_realization.md`
§5). A plane L-shaped anchor-to-anchor loop with a quarter-circle elbow is
solved by the force method under uniform temperature rise, with the elbow's
in-plane bending flexibility multiplied by a user-entered factor `k`. The
document provides anchor reactions, a tangent-point displacement, and a
k-sweep monotonicity statement for `k in {1, 5, 10, 20}`.

## Epistemic Status

- Draft witness document. All inputs are invented fixture data.
- No code content: `k` is an opaque user-entered value. No piping-code
  flexibility-factor equation, catalog value, SIF, or default is cited,
  derived, or implied anywhere in this document.
- No lifecycle, release-readiness, professional, certification, or
  code-compliance claim is created by this document.
- This reference is not yet wired into any benchmark crate. Solver comparison
  values are `TBD` until the benchmark-wiring tranche lands. The numbers below
  are the independent hand-calculated side only.

## Provenance

- Source: OpenPipeStress original mechanics benchmark.
- Redistribution: project-original-public-content.
- Contributor certification: generated from elementary open mechanics (force
  method / unit-load theorem on a plane frame), not copied from protected
  standards, commercial software examples, or proprietary data.

## Invented Inputs

All values are invented user inputs per the repository invented-fixture
convention. They are not material-standard, dimensional-standard, or
code-derived values, even where they resemble plausible pipe numbers.

| Symbol | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `D_o` | 0.2191 | m | length |
| `t` | 0.00818 | m | length |
| `E` | 200.0e9 | Pa | stress |
| `alpha` | 12.0e-6 | 1/K | thermal_expansion_coefficient |
| `DeltaT` | +150.0 | K | temperature_interval |
| `L1` | 3.0 | m | length |
| `R` | 0.5 | m | length |
| `L2` | 4.0 | m | length |
| `k` | {1, 5, 10, 20} | - | dimensionless (user-entered in-plane bend flexibility factor) |

Geometry (global `X` right, `Y` up, `Z` out of plane; counterclockwise
moments positive):

- Anchor `A` at `(0, 0)`, fully fixed in-plane (`u_x`, `u_y`, `theta_z`).
- Vertical leg from `A` up to tangent point `T1 = (0, L1) = (0, 3.0)`.
- Quarter-circle elbow, centerline radius `R`, arc center `C = (R, L1) =
  (0.5, 3.0)`, from `T1` to tangent point `T2 = (R, L1 + R) = (0.5, 3.5)`.
  Tangents are continuous: vertical at `T1`, horizontal at `T2`.
- Horizontal leg from `T2` to anchor `B = (R + L2, L1 + R) = (4.5, 3.5)`,
  fully fixed in-plane.

The loop lies in one plane; the in-plane problem has 3 redundants. Loading is
uniform temperature rise `DeltaT` only: no pressure, no weight.

Arc parametrization used throughout: `phi in [0, pi/2]`, point
`P(phi) = (R - R cos(phi), L1 + R sin(phi))`, `ds = R dphi`. `phi = 0` is
`T1`; `phi = pi/2` is `T2`.

## Derived Section Properties (longhand)

```text
D_i = D_o - 2 t = 0.2191 - 0.01636 = 0.20274 m

D_o^2 = 0.0480048100 m^2      D_i^2 = 0.0411035076 m^2
D_o^4 = 2.304461783136e-3 m^4  D_i^4 = 1.689498337023e-3 m^4

A = (pi/4) (D_o^2 - D_i^2)
  = (pi/4) (0.0480048100 - 0.0411035076)
  = (pi/4) (0.0069013024)
  = 5.420270230e-3 m^2

I = (pi/64) (D_o^4 - D_i^4)
  = (pi/64) (2.304461783136e-3 - 1.689498337023e-3)
  = (pi/64) (6.149634461130e-4)
  = 3.018694757e-5 m^4

E I = 200.0e9 * 3.018694757e-5 = 6.037389514e6 N m^2

epsilon_T = alpha DeltaT = 12.0e-6 * 150.0 = 1.800000e-3
```

`A` is derived for completeness and for the axial-sensitivity note below; the
flexibility coefficients themselves are bending-only (assumption stated next).

## Method and Stated Assumptions

Force method (unit-load / Castigliano). The primary structure is the
cantilever fixed at `A`, obtained by releasing all three in-plane restraints
at `B`. Redundants are the in-plane reaction components the anchor at `B`
exerts on the pipe:

```text
X1 = H_B  (global +X force at B)
X2 = V_B  (global +Y force at B)
X3 = M_B  (counterclockwise +Z moment at B)
```

**Assumption (bending-only flexibility, axial thermal driving).** The classic
slender-frame treatment is used and carried consistently throughout: the
flexibility coefficients `f_ij` include bending strain energy only (axial and
shear flexibility neglected), while the thermal driving terms retain the full
axial thermal growth kinematics of the released structure. Justification: for
this fixture the slenderness measures `A L1^2 / I = 1616` and
`A L2^2 / I = 2873` make axial flexibility contributions to `f_ij` of order
0.1 % of the bending contributions. The sensitivity is quantified in the
verification appendix: including true axial flexibility (`E A` as derived
above) shifts the reactions by at most 5.5e-3 relative (worst at `k = 1`),
shrinking as `k` grows. Any solver comparison must either replicate the
bending-only assumption (e.g. by scaling up axial stiffness) or account for
this documented deviation in its tolerance basis.

**Elbow flexibility.** The elbow carries the same cross-section; its in-plane
bending flexibility is multiplied by the opaque user factor `k`. Concretely,
every bending integrand over the arc uses effective rigidity `E I / k`, i.e.
the arc contribution to each `EI f_ij` below carries a factor `k`. Only the
in-plane factor is relevant for this plane problem.

**Thermal driving terms.** Under uniform `DeltaT`, the released (statically
determinate) structure expands as a similarity scaling about the fixed anchor
`A` with strain `epsilon_T` and zero curvature (uniform temperature produces
no thermal curvature). The free displacement of the release point `B` is
therefore exact and elementary:

```text
Delta_1T = epsilon_T x_B = 1.800000e-3 * 4.5 = 8.100000000e-3 m
Delta_2T = epsilon_T y_B = 1.800000e-3 * 3.5 = 6.300000000e-3 m
Delta_3T = 0   (no thermal rotation)
```

with `x_B = R + L2 = 4.5 m`, `y_B = L1 + R = 3.5 m`.

Leg-by-leg check of the same result: leg 1 grows `epsilon_T L1` in `+Y`; the
arc's chord from `T1` to `T2` scales by `(1 + epsilon_T)`, adding
`epsilon_T (R, R)`; leg 2 grows `epsilon_T L2` in `+X`. Sum:
`(epsilon_T (R + L2), epsilon_T (L1 + R))`. Same values.

## Unit Moment Fields

Moment at a centerline point `P = (x, y)` (counterclockwise `+Z`, from the
free body between `P` and `B`) due to unit redundants applied at `B`:

```text
m1(P) = y - y_B          (unit +X force at B)
m2(P) = x_B - x          (unit +Y force at B)
m3(P) = 1                (unit +Z moment at B)
```

Cross-product check for `m1`: `r x F` with `r = (x_B - x, y_B - y)`,
`F = (1, 0)` gives `M_z = -(y_B - y) = y - y_B`. Correct.

Flexibility coefficients (bending only, arc weighted by `k`):

```text
f_ij = (1/EI) [ INT_leg1 m_i m_j ds + k INT_arc m_i m_j ds + INT_leg2 m_i m_j ds ]
```

## Closed-Form Flexibility Integrals

On leg 1: `x = 0`, `y in [0, L1]`, `ds = dy`.
On the arc: `x = R(1 - cos phi)`, `y = L1 + R sin phi`, `ds = R dphi`, so

```text
m1 = y - y_B = -R (1 - sin phi)
m2 = x_B - x = L2 + R cos phi
m3 = 1
```

On leg 2: `y = y_B` (so `m1 = 0`), `x in [R, x_B]`, `ds = dx`.

The four arc primitives needed, each verified by differentiating the stated
antiderivative back to the integrand:

```text
INT_0^{pi/2} (1 - sin phi)^2 dphi = 3 pi/4 - 2
    antiderivative: (3/2) phi + 2 cos phi - (1/2) sin phi cos phi
    d/dphi = 3/2 - 2 sin phi - (1/2) cos 2phi = 1 - 2 sin phi + sin^2 phi  OK

INT_0^{pi/2} (1 - sin phi) dphi = pi/2 - 1
INT_0^{pi/2} (1 - sin phi) cos phi dphi = [sin phi - (1/2) sin^2 phi] = 1/2
INT_0^{pi/2} (L2 + R cos phi)^2 dphi = L2^2 pi/2 + 2 L2 R + R^2 pi/4
    (uses INT cos = 1, INT cos^2 = pi/4 over [0, pi/2])
```

Resulting closed forms (all as `EI f_ij`, units m^3 for force-force terms,
m^2 for force-moment, m for moment-moment):

```text
EI f11 = [y_B^3 - (y_B - L1)^3]/3 + k R^3 (3 pi/4 - 2)
EI f22 = x_B^2 L1 + k R [L2^2 pi/2 + 2 L2 R + R^2 pi/4] + L2^3 / 3
EI f33 = L1 + L2 + k R pi/2
EI f12 = -x_B L1 (y_B - L1/2) - k R^2 [L2 (pi/2 - 1) + R/2]
EI f13 = -L1 (y_B - L1/2) - k R^2 (pi/2 - 1)
EI f23 = x_B L1 + k R (L2 pi/2 + R) + L2^2 / 2
```

Numeric constants for this fixture (`L1 = 3`, `R = 0.5`, `L2 = 4`,
`x_B = 4.5`, `y_B = 3.5`):

```text
EI f11 = 14.25          + 0.044524311 k
EI f22 = 82.083333333   + 14.664545385 k     (60.75 + 64/3 = 82.0833...)
EI f33 = 7.0            + 0.785398163 k
EI f12 = -27.0          - 0.633296327 k
EI f13 = -6.0           - 0.142699082 k
EI f23 = 21.5           + 3.391592654 k
```

Independent checks of individual coefficients:

- Leg-2 term of `EI f22` is `INT_R^{x_B} (x_B - x)^2 dx = L2^3 / 3 =
  21.333...`, the standard cantilever tip-load table value `P L^3 / 3EI`.
  Leg-1 term `x_B^2 L1 = 60.75` is the rigid-lever transport of a unit `+Y`
  force at `B` onto the vertical leg (constant moment arm `x_B`). Both match
  Mohr/virtual-work table values.
- Leg-1 term of `EI f11` is `INT_0^{L1} (y - y_B)^2 dy =
  [y_B^3 - (y_B - L1)^3]/3 = (42.875 - 0.125)/3 = 14.25`.
- All six coefficients were additionally verified against midpoint-rule
  numerical quadrature of the defining integrals (200 000 points per member)
  at `k = 1` and at an off-table value `k = 7.3`; maximum relative
  disagreement observed was 4.2e-12 (see verification appendix).

## Compatibility System and Solution

Compatibility at `B` (net displacement and rotation of `B` must vanish):

```text
[f] {X} + {Delta_T} = 0     =>     [EI f] {X} = -EI {Delta_T}

-EI Delta_1T = -6.037389514e6 * 8.100000000e-3 = -4.890285506e4
-EI Delta_2T = -6.037389514e6 * 6.300000000e-3 = -3.803555394e4
-EI Delta_3T = 0
```

The 3x3 symmetric system for each `k` (coefficients from the closed forms
above; solvable by hand elimination):

```text
k = 1:   [ 14.294524311  -27.633296327   -6.142699082 ] {X1}   {-4.890285506e4}
         [-27.633296327   96.747878718   24.891592654 ] {X2} = {-3.803555394e4}
         [ -6.142699082   24.891592654    7.785398163 ] {X3}   { 0            }
         det(EI f) = 7.650302501e2

k = 5:   [ 14.472621556  -30.166481634   -6.713495408 ]        det = 1.800188787e3
         [-30.166481634  155.406060257   38.457963268 ]
         [ -6.713495408   38.457963268   10.926990817 ]

k = 10:  [ 14.695243113  -33.332963268   -7.426990817 ]        det = 3.116740578e3
         [-33.332963268  228.728787181   55.415926536 ]
         [ -7.426990817   55.415926536   14.853981634 ]

k = 20:  [ 15.140486225  -39.665926536   -8.853981634 ]        det = 5.825310923e3
         [-39.665926536  375.374241029   89.331853072 ]
         [ -8.853981634   89.331853072   22.707963268 ]
```

(The right-hand side is the same for all `k`.)

Reactions at `A` follow from rigid-body equilibrium of the whole loop (only
anchor reactions act; thermal load is self-contained):

```text
H_A = -H_B
V_A = -V_B
M_A = -M_B - (x_B V_B - y_B H_B)
```

## Expected Values

Anchor reactions (forces in N, moments in N m; components are the forces the
anchors exert on the pipe in global axes, counterclockwise moments positive):

| Quantity | k = 1 | k = 5 | k = 10 | k = 20 |
|---|---:|---:|---:|---:|
| `H_B` | -1.163612454e4 | -7.461555977e3 | -6.144267267e3 | -5.282037255e3 |
| `V_B` | -7.635238153e3 | -4.329778531e3 | -3.301677847e3 | -2.654674339e3 |
| `M_B` | +1.523056675e4 | +1.065447422e4 | +9.245475309e3 | +8.383839402e3 |
| `H_A` | +1.163612454e4 | +7.461555977e3 | +6.144267267e3 | +5.282037255e3 |
| `V_A` | +7.635238153e3 | +4.329778531e3 | +3.301677847e3 | +2.654674339e3 |
| `M_A` | -2.159843094e4 | -1.728591674e4 | -1.589286043e4 | -1.492493527e4 |

Tangent-point `T2 = (0.5, 3.5)` displacement (m, global axes), derived below:

| Quantity | k = 1 | k = 5 | k = 10 | k = 20 |
|---|---:|---:|---:|---:|
| `u_x(T2)` | -7.200000000e-3 | -7.200000000e-3 | -7.200000000e-3 | -7.200000000e-3 |
| `u_y(T2)` | +6.691964071e-3 | +6.468273963e-3 | +6.417658716e-3 | +6.419030749e-3 |

### Tangent-point displacement derivation

By the unit-load theorem on the primary structure, the displacement of any
point `Q` is the free thermal scaling plus the bending effect of the
redundants:

```text
u_i(Q) = epsilon_T q_i + (1/EI) SUM_j X_j INT_{A->Q} mQ_i m_j ds_eff
```

where `mQ_i` is the moment field of a unit load at `Q` in direction `i`
(nonzero only between `A` and `Q`) and `ds_eff` carries the arc factor `k`.

For `u_x(T2)`: since `T2` and `B` share the same height (`y_Q = y_B`), the
unit-`X` field at `T2` equals `m1` on leg 1 and the arc, and `m1 = 0` on
leg 2 anyway. So the redundant contribution equals the full compatibility sum
`f11 X1 + f12 X2 + f13 X3 = -Delta_1T = -epsilon_T x_B`, giving exactly

```text
u_x(T2) = epsilon_T x_Q - epsilon_T x_B = -epsilon_T L2 = -7.200000000e-3 m
```

for every `k`. Physical reading: under the stated assumptions leg 2 changes
length only thermally, and `B` does not move, so `T2` moves `-epsilon_T L2`
in `X`. This exact `k`-independence is itself a useful solver check.

For `u_y(T2)`: the unit-`Y` field at `T2` is `mQ_2 = x_Q - x` over
`A -> T2`, i.e. `R` on leg 1 and `R cos phi` on the arc. The three
coupling integrals (arc terms carry `k`):

```text
EI g21 = INT mQ_2 m1 = -R L1 (y_B - L1/2) - k R^3 / 2
EI g22 = INT mQ_2 m2 =  R x_B L1 + k R^2 (L2 + R pi/4)
EI g23 = INT mQ_2 m3 =  R L1 + k R^2

u_y(T2) = epsilon_T y_Q + (X1 g21 + X2 g22 + X3 g23)

numeric (EI g2j):        k = 1        k = 5       k = 10       k = 20
  EI g21              -3.0625      -3.3125      -3.6250      -4.2500
  EI g22           7.848174770 12.240873852 17.731747704 28.713495408
  EI g23              1.75         2.75         4.00         6.50
```

with `epsilon_T y_Q = 1.8e-3 * 3.5 = 6.3e-3 m`. Values in the table above.
Note `u_y(T2)` is not monotonic in `k` (minimum near `k ~ 10-20`); no
monotonicity claim is made for displacements.

### Equilibrium check

For each `k`, whole-body equilibrium of the loop under anchor reactions only:

```text
SUM F_x = H_A + H_B
SUM F_y = V_A + V_B
SUM M@A = M_A + M_B + x_B V_B - y_B H_B
```

| Residual | k = 1 | k = 5 | k = 10 | k = 20 |
|---|---:|---:|---:|---:|
| `SUM F_x` (N) | 0.0 | 0.0 | 0.0 | 0.0 |
| `SUM F_y` (N) | 0.0 | 0.0 | 0.0 | 0.0 |
| `SUM M@A` (N m) | 0.0 | 3.6e-12 | 0.0 | 0.0 |

`H_A`/`V_A` cancel `H_B`/`V_B` by construction; the moment residual is an
independent check of `M_A` against the solved `X` and is at double-precision
round-off (largest magnitude 3.6e-12 N m against reactions of order 1e4).

## k-Sweep Monotonicity Statement

The governing anchor moment is `M_A` (largest magnitude at every `k`). Its
magnitude must decrease monotonically as the user flexibility factor `k`
increases (a more flexible elbow relieves thermal restraint). From the table:

```text
|M_A|:  2.159843094e4 > 1.728591674e4 > 1.589286043e4 > 1.492493527e4   (k = 1, 5, 10, 20)
|M_B|:  1.523056675e4 > 1.065447422e4 > 9.245475309e3 > 8.383839402e3
|H|  :  1.163612454e4 > 7.461555977e3 > 6.144267267e3 > 5.282037255e3
|V|  :  7.635238153e3 > 4.329778531e3 > 3.301677847e3 > 2.654674339e3
```

Strict monotone decrease holds for both anchor moments and both force
components across the sweep. This is the reference behavior for the D-34
k-sweep monotonicity evidence item.

## Verification Appendix (how these numbers were checked)

1. **Antiderivative check.** Every arc primitive was verified by
   differentiating its antiderivative back to the integrand (shown inline
   above for the `(1 - sin phi)^2` case).
2. **Quadrature check of `f_ij`.** All six closed-form coefficients were
   compared against midpoint-rule quadrature of the defining member integrals
   with 200 000 points per member, at `k = 1` and `k = 7.3`. Maximum relative
   disagreement: 4.2e-12.
3. **Table-value check.** Straight-leg contributions to `f11`, `f22`
   reproduce standard Mohr/virtual-work cantilever table values (see the
   closed-form section).
4. **Equilibrium residuals.** At double-precision round-off for all four `k`
   (table above).
5. **Independent displacement-method model.** A separate plane-frame
   stiffness-method model (800 straight elements: 200 per leg, arc modeled as
   400 chords with bending rigidity `EI/k`; a refined 400/800 variant was
   also run; axial rigidity scaled by 1e3 to emulate the
   bending-only-flexibility assumption; thermal load as equivalent axial
   nodal forces `EA epsilon_T`) reproduced all reactions and
   `u_y(T2)` within 5.4e-6 relative for every `k` — consistent with that
   model's own chord-discretization and axial-penalty error, independent of
   the force-method derivation. Larger axial-stiffness scaling (1e7) was
   observed to degrade agreement to ~1e-2 through penalty ill-conditioning of
   the assembled matrix, not through error in this reference; comparison
   configurations should avoid extreme penalty scalings.
6. **Axial-flexibility sensitivity (assumption bound).** Rerunning the
   displacement-method model with the true axial rigidity `E A`
   (no scaling) shifts the reactions by at most 5.5e-3 relative at `k = 1`
   (monotonically less at higher `k`; ~1.8e-3 at `k = 20`). This bounds the
   effect of the stated bending-only assumption and must inform the
   comparison configuration or tolerance when this reference is wired into
   the benchmark crate.

Arithmetic was checked with a throwaway script outside the repository; the
verified numbers were transcribed here. No script is part of this witness.

## Proposed Tolerance Basis

Per the `DEC-024`/`DEC-026` class-tiered convention (relative + absolute
pairs; absolute member is the explicit near-zero floor; tighten-only), this
reference belongs to the **analytic benchmark class**. Proposed pair for the
benchmark-crate comparison, contingent on the comparison model replicating
the bending-only-flexibility assumption (e.g. axial rigidity scaled up by
~1e3, not more, per appendix item 5):

| Quantity kind | Relative | Absolute floor |
|---|---:|---:|
| anchor force (N) | 1.0e-9 | 1.0e-5 |
| anchor moment (N m) | 1.0e-9 | 1.0e-5 |
| displacement (m) | 1.0e-9 | 1.0e-12 |

The relative member follows the `DEC-026` analytic-class governed seed
(1.0e-9). If the comparison instead uses the true axial rigidity `E A`, the
documented assumption bound applies and the comparison cannot be tighter than
6.0e-3 relative on reactions at `k = 1` (appendix item 6); that looser basis
would need its own recorded reason and is not the recommended configuration.
These are proposed values only; adoption into the governed tolerance record
is a separate governance step.

## Boundary

This note records a code-neutral mechanics derivation for an invented
fixture. It does not define release tolerances, rule checks, allowables,
stress categories, SIF or flexibility-factor formulas, or project-specific
decisions. The factor `k` is an opaque user-entered value throughout.

Tolerance policy: `TBD`.
