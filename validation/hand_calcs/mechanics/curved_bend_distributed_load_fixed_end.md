# MECH-CURVED-BEND-DISTRIBUTED-FIXED-END

## Purpose

Curved-beam distributed-load reference for the two remaining D-34/DEC-070
arc residuals: (1) arc-consistent equivalent nodal loads (fixed-end forces
and moments) for a uniform distributed load on a curved-bend macro-element,
and (2) interior-station section resultants along a loaded arc. A
quarter-circle circular arc, clamped at both ends, carries a uniform load of
constant direction (one in-plane case, one out-of-plane case). The document
provides, by the force method on closed-form unit-load integrals: the
free-tip deflections of the anchored arc, the clamped-end reactions at both
ends, and the interior-station bending/torsion distribution at the quarter,
mid, and three-quarter arc points, for user-entered bending flexibility
factors `k in {1, 2}`.

## Epistemic Status

- Witness document; all inputs are invented fixture data.
- No code content: `k` is an opaque user-entered value applied identically
  to the in-plane and out-of-plane bending strain energy (matching the
  product's single-user-factor mapping). No piping-code flexibility-factor
  equation, catalog value, SIF, or default is cited, derived, or implied.
- No lifecycle, release-readiness, professional, certification, or
  code-compliance claim is created by this document.
- Solver comparison: wired in the same tranche into
  `validation/benchmarks/mechanics` as fixture
  `MECH-CURVED-BEND-DISTRIBUTED-FIXED-END` (see the fixture inventory row in
  `validation/hand_calcs/mechanics/README.md`). Both sides are closed-form,
  so the comparison tolerance is the analytic-class 1.0e-9 relative tier
  (`DEC-026`), tighten-only.

## Provenance

- Source: OpenPipeStress original mechanics benchmark.
- Redistribution: project-original-public-content.
- Contributor certification: generated from elementary open mechanics
  (force method / unit-load theorem on a circular arc), not copied from
  protected standards, commercial software examples, or proprietary data.

## Invented Inputs

All values are invented user inputs per the repository invented-fixture
convention. They are not material-standard, dimensional-standard, or
code-derived values, even where they resemble plausible pipe numbers.

| Symbol | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `R` | 1.2 | m | length |
| `Phi` | pi/2 | rad | angle |
| `D_o` | 0.1683 | m | length |
| `t` | 0.0071 | m | length |
| `E` | 200.0e9 | Pa | stress |
| `G` | 80.0e9 | Pa | stress |
| `w_y` | -1500.0 | N/m | force_per_length (in-plane case) |
| `w_z` | -800.0 | N/m | force_per_length (out-of-plane case) |
| `k` | {1, 2} | - | dimensionless (user-entered bending flexibility factor) |

Geometry and frame (the local frame of the arc): the arc center sits at the
origin; node `A` (end i) at `(R, 0, 0)`, node `B` (end j) at `(0, R, 0)`.
Arc parametrization `theta in [0, Phi]`, point
`P(theta) = R (cos theta, sin theta, 0)`, `ds = R dtheta`; `theta = 0` is
`A`, `theta = Phi = pi/2` is `B`. Local `z` is the bend-plane normal; the
section tangent is `t(theta) = (-sin theta, cos theta, 0)` and the outward
radial is `r(theta) = (cos theta, sin theta, 0)`. Both intensities are
constant-direction loads per unit arc length: the in-plane case acts along
local `-y`, the out-of-plane case along local `-z`.

Interior stations are reported in the right-handed **section frame**
`[x_s = t(theta), y_s = z x t(theta) = -r(theta), z_s = z]` (x toward `B`,
y toward the arc center), the same frame the product rows use for arc
stations. `bending_z` is the in-plane moment (about the bend normal),
`bending_y` the out-of-plane moment (about the inward radial), `torsion`
the moment about the tangent.

## Derived Section Properties (longhand)

```text
D_i = D_o - 2 t = 0.1683 - 0.0142 = 0.1541 m

A = (pi/4) (D_o^2 - D_i^2) = (pi/4)(0.02832489 - 0.02374681)
  = 3.595615623887e-3 m^2
I = (pi/64) (D_o^4 - D_i^4) = (pi/64)(8.02299379e-4 - 5.63910623e-4)
  = 1.170186363015e-5 m^4
J = 2 I = 2.340372726029e-5 m^4

E A = 7.191231247773e8 N
E I = 2.340372726029e6 N m^2
G J = 1.872298180823e6 N m^2
```

## Method and Stated Assumptions

Force method (unit-load / Castigliano) on the primary cantilever fixed at
`A` with all six restraints at `B` released. The strain-energy model matches
the macro-element formulation exactly: Euler-Bernoulli bending (in-plane and
out-of-plane, each multiplied by the user factor `k`), St. Venant torsion,
and axial stretch; shear deformation excluded. Internal actions at `theta`
come from equilibrium of the free segment `[theta, Phi]`.

For a unit load case `m` at `B` and the distributed load `q`, the tip
deflection integrals are

```text
delta0[m] = R * Int_0^Phi [ k M_m M_q / (E I)  (in-plane)
                          + k Mo_m Mo_q / (E I) (out-of-plane)
                          + T_m T_q / (G J) + N_m N_q / (E A) ] dtheta
```

Distributed-load action fields (segment equilibrium, uniform local intensity
`w = (w_x, w_y, w_z)` per arc length, here with one nonzero component per
case):

```text
a_x(theta) = (sin Phi - sin theta) - (Phi - theta) cos theta
a_y(theta) = (cos theta - cos Phi) - (Phi - theta) sin theta

in-plane moment   M_q(theta)  = R^2 (a_x w_y - a_y w_x)
out-of-plane      Mo_q(theta) = R^2 w_z (1 - cos(Phi - theta))
torsion           T_q(theta)  = R^2 w_z ((Phi - theta) - sin(Phi - theta))
axial             N_q(theta)  = R (Phi - theta)(w_y cos theta - w_x sin theta)
```

Clamped-clamped redundants (support-on-element forces at `B`):
`X = -F^-1 delta0`, with `F` the closed-form end-flexibility block of the
released DOFs (in-plane block `(u_x, u_y, theta_z)`; out-of-plane block
`(u_z, theta_x, theta_y)`; the plane arc decouples them exactly). Reactions
at `A` follow from rigid equilibrium with the load resultant. Consistent
equivalent nodal loads are the negated clamped reactions,
`p_A = -g_A`, `p_B = -X`.

Interior-station resultants (clamped-clamped state, section at
`theta = f Phi`) from equilibrium of `[theta, Phi]` under `X` at `B` plus
the distributed load on the segment:

```text
M(theta) = M_X + (P(Phi) - P(theta)) x F_X + R^2 a(theta) x w
F(theta) = F_X + R (Phi - theta) w
```

## Closed-Form Quarter-Circle Integrals (Phi = pi/2)

Free-tip deflections of the anchored arc (independent longhand
antiderivatives; each verified by differentiation and by quadrature in the
appendix):

```text
In-plane, uniform w_y:
  u_x  = w_y [ k R^4 (3 - 7 pi/8) / (E I) - R^2 pi / (8 E A) ]
  u_y  = w_y [ k R^4 (pi^2/16 - 1/4) / (E I) + R^2 (pi^2/16 + 1/4) / (E A) ]
  th_z = k R^3 w_y (pi/2 - 2) / (E I)

Out-of-plane, uniform w_z:
  u_z  = R^4 w_z [ k / (2 E I) + (pi^2/8 - pi/2 + 1/2) / (G J) ]
  th_x = R^3 w_z [ k / (2 E I) - (pi/2 - 3/2) / (G J) ]
  th_y = R^3 w_z (1 - pi/4) [ k / (E I) + 1 / (G J) ]
```

In-plane end-flexibility block at `B` (rows/cols `u_x, u_y, theta_z`):

```text
f_xx = k R^3 (3 pi/4 - 2)/(E I) + R pi/(4 E A)
f_xy = k R^3 / (2 E I) - R/(2 E A)
f_xm = k R^2 (1 - pi/2)/(E I)
f_yy = k R^3 (pi/4)/(E I) + R pi/(4 E A)
f_ym = -k R^2/(E I)
f_mm = k R (pi/2)/(E I)
```

Out-of-plane block at `B` (rows/cols `u_z, theta_x, theta_y`), with
`C1 = pi/4`, `C2 = 3 pi/4 - 2`:

```text
f_zz = R^3 (k C1/(E I) + C2/(G J))
f_zx = R^2 (k C1/(E I) + (pi/4 - 1)/(G J))
f_zy = R^2 (k/(2 E I) + 1/(2 G J))
f_xx = R (k C1/(E I) + C1/(G J))
f_xy = R (k/(2 E I) - 1/(2 G J))
f_yy = R (k C1/(E I) + C1/(G J))
```

## Results

All values in the local frame above; forces in N, moments in N m,
displacements in m, rotations in rad. Reactions are support-on-element.

### In-plane case (`w_y = -1500 N/m`)

Free-tip deflections of the anchored arc:

| k | `u_x` | `u_y` | `theta_z` |
|---:|---:|---:|---:|
| 1 | -3.325456995161e-4 | -4.901547357913e-4 | +4.753498913120e-4 |
| 2 | -6.662709328105e-4 | -9.777057497307e-4 | +9.506997826241e-4 |

Clamped-clamped end reactions:

| k | `F_x(B)` | `F_y(B)` | `M_z(B)` | `F_x(A)` | `F_y(A)` | `M_z(A)` |
|---:|---:|---:|---:|---:|---:|---:|
| 1 | +518.6887093849 | +675.8314673820 | +152.2772588244 | -518.6887093849 | +2151.6019208490 | +48.2268874190 |
| 2 | +603.4037708891 | +589.0892325630 | +122.9516066821 | -603.4037708891 | +2238.3441556680 | +75.1199315834 |

Interior-station in-plane bending `M_z` (clamped-clamped, `theta = f Phi`):

| k | f = 0.25 | f = 0.5 | f = 0.75 |
|---:|---:|---:|---:|
| 1 | +36.3621332981 | -36.5602035248 | -45.2741760268 |
| 2 | +40.4485183865 | -22.0575891752 | -42.5043088892 |

### Out-of-plane case (`w_z = -800 N/m`)

Free-tip deflections of the anchored arc:

| k | `u_z` | `theta_x` | `theta_y` |
|---:|---:|---:|---:|
| 1 | -4.987403008669e-4 | -2.430655343790e-4 | -2.852099347872e-4 |
| 2 | -8.531453880460e-4 | -5.384031070283e-4 | -4.119699058038e-4 |

Clamped-clamped end reactions:

| k | `F_z(B)` | `M_x(B)` | `M_y(B)` | `F_z(A)` | `M_x(A)` | `M_y(A)` |
|---:|---:|---:|---:|---:|---:|---:|
| 1 | +753.9822368616 | -15.0729196640 | -262.2942354301 | +753.9822368616 | +262.2942354301 | +15.0729196640 |
| 2 | +753.9822368616 | -12.2125547407 | -259.4338705068 | +753.9822368616 | +259.4338705068 | +12.2125547407 |

By the mirror symmetry of the quarter arc about its bisector, each clamp
carries exactly half the total load
`|w_z| R Phi = 800 * 1.2 * pi/2 = 1507.9644737232 N`, independent of `k`;
this is an exact structural check on the tabulated `F_z` values.

Interior-station section-frame resultants (clamped-clamped): `torsion`
about the tangent and `bending_y` about the inward radial:

| k | quantity | f = 0.25 | f = 0.5 | f = 0.75 |
|---:|---|---:|---:|---:|
| 1 | torsion | +29.1159479459 | 0 (exact, symmetry) | -29.1159479459 |
| 1 | bending_y | -10.4566016240 | -106.2339587762 | -10.4566016240 |
| 2 | torsion | +30.6639662874 | 0 (exact, symmetry) | -30.6639662874 |
| 2 | bending_y | -14.1938484988 | -110.2791256441 | -14.1938484988 |

The mirror symmetry forces the torsion distribution to be antisymmetric in
`f` about 0.5 (zero at midspan) and `bending_y` symmetric; the tabulated
values satisfy both exactly.

## Verification Appendix

Independent checks performed on every tabulated value (both `k` values,
both load cases):

1. **Antiderivative differentiation.** Every closed-form integral above was
   re-derived by differentiating the stated antiderivatives back to the
   integrands (`Int theta cos = Phi sin Phi + cos Phi - 1`,
   `Int theta sin^2 = Phi^2/4 - Phi sin 2Phi/4 - (cos 2Phi - 1)/8`, etc.).
2. **200k-point midpoint quadrature** of the unit-load deflection integrals
   against the closed forms: maximum relative deviation 4.0e-12 (in-plane)
   and 1.1e-11 (out-of-plane), consistent with quadrature discretization
   error.
3. **Independent 4000-element displacement-method model** (straight
   Euler-Bernoulli segments along the arc, bending stiffness divided by `k`,
   consistent 50/50 fine lumping of the load, both ends clamped): clamped
   reactions agree with the force-method values to at most 2.4e-5 relative
   (discretization error of the fine model); interior-station moments by
   segment equilibrium of the fine model agree to at most 7.8e-5 relative.
4. **Rigid equilibrium residuals** of the tabulated closed-form reactions
   against the exact load resultants: zero to round-off (worst residual
   4.6e-13 of the load scale).
5. **Symmetry checks** (out-of-plane case): equal end shears, antisymmetric
   torsion with an exact midspan zero, symmetric `bending_y` — satisfied
   exactly by the closed forms.

## Boundaries

- The tabulated interior stations describe the clamped-clamped
  configuration; the general station statement (arbitrary end force at `B`)
  is exercised by the benchmark and product tests through the same segment
  equilibrium.
- Values are reference mechanics for verification only; nothing here is a
  design-code result, an allowable, or a compliance statement.
