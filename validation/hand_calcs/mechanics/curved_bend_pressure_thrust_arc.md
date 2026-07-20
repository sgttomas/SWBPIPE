# MECH-CURVED-BEND-PRESSURE-THRUST-ARC

## Purpose

Exact statics of internal-pressure loading on a circular-arc pipe span
realized as a curved-bend macro-element, replacing the recorded
straight-chord axial pressure-thrust treatment on macro spans. The document
derives, from elementary statics on invented geometry: the cap/wall
decomposition of the complete pressure system; the worked integrals of the
distributed radial wall load (resultant and center moment); the exact
self-equilibrium of the end-cap pair plus wall load; why a
statically-lumped-to-nodes version of the complete system cancels to zero
(so the exact work-equivalent consistent nodal vector is required); the
membrane-state station identity (wall tension `+p A` along the local
tangent, zero shear, zero internal moment at every interior station); the
small-angle reduction to the existing straight-chord treatment; and the
closed-form values recorded by the benchmark fixture
`MECH-CURVED-BEND-PRESSURE-THRUST-ARC`.

## Epistemic Status

- Witness document; all inputs are invented fixture data.
- No code content: no piping-code flexibility-factor equation, catalog
  value, SIF, pressure-design rule, or default is cited, derived, or
  implied. The user-entered bending flexibility factor `k` is an opaque
  number; the tabulated fixture values are exactly independent of it.
- No lifecycle, release-readiness, professional, certification, or
  code-compliance claim is created by this document.
- Solver comparison: wired in the same tranche into
  `validation/benchmarks/mechanics` as fixture
  `MECH-CURVED-BEND-PRESSURE-THRUST-ARC` (see the fixture inventory row in
  `validation/hand_calcs/mechanics/README.md`). Both sides are closed-form,
  so the comparison tolerance is the analytic-class 1.0e-9 relative tier
  (`DEC-026`), tighten-only.

## Provenance

- Source: OpenPipeStress original mechanics benchmark.
- Redistribution: project-original-public-content.
- Contributor certification: generated from elementary open statics
  (control-volume decomposition and direct integration on a circular arc),
  not copied from protected standards, commercial software examples, or
  proprietary data.

## Geometry, Frame, and Notation

Circular arc of radius `R`, included angle `Phi in (0, pi)`, in the local
bend-plane frame of the curved-bend macro-element: arc center at the
origin, node `i` at arc angle 0, node `j` at `Phi`, measured about local
`z` (the bend-plane normal). Point, tangent, and outward radial:

```text
P(theta) = R (cos theta, sin theta, 0),   ds = R dtheta
t(theta) = (-sin theta, cos theta, 0)     (unit tangent, oriented i -> j)
n(theta) = (cos theta, sin theta, 0)      (outward radial unit vector)

t_i = t(0) = (0, 1, 0)
t_j = t(Phi) = (-sin Phi, cos Phi, 0)
```

`p` is the internal pressure and `A` the thrust-generating area (pipe
internal area, or a user-entered expansion-joint effective area), so the
thrust magnitude is `F_p = p A`. `A_s` denotes the structural (steel)
cross-section area and `E A_s` the axial rigidity; the two areas are
distinct and never interchanged below.

## 1. Cap/Wall Decomposition of the Complete Pressure System

Cut the span at both ends and close the fluid column with virtual end
caps. The pressure loading of the closed-section arc span decomposes
exactly into:

- **(a) End-cap forces.** The fluid pushes on each cap with force `p A`
  along the outward cut normal, i.e. `-p A t_i` at node `i` and
  `+p A t_j` at node `j` (the pressure-tension transmission the existing
  straight treatment models along its chord).
- **(b) Distributed radial wall load.** Control volume on the fluid
  column between arc sections `theta_1 < theta_2`: momentum balance of
  the static fluid gives, for the wall-on-fluid force `W`,
  `p A t(theta_1) - p A t(theta_2) + W = 0`. Differentiating in arc
  length, the fluid-on-wall load per unit arc length is

  ```text
  q(theta) = -p A (dt/ds) = (p A / R) n(theta)
  ```

  since `dt/ds = -(1/R) n(theta)`. The wall load is radially OUTWARD.

## 2. Worked Integrals of the Wall Load

Resultant of the distributed radial wall load:

```text
Int_0^Phi q ds = (p A / R) Int_0^Phi (cos theta, sin theta, 0) R dtheta
              = p A (sin Phi, 1 - cos Phi, 0)
              = p A (t_i - t_j)
```

using `t_i - t_j = (0,1,0) - (-sin Phi, cos Phi, 0)
= (sin Phi, 1 - cos Phi, 0)`.

Total moment of the wall load about the arc center:

```text
Int_0^Phi P(theta) x q(theta) ds
  = Int_0^Phi [R n(theta)] x [(p A / R) n(theta)] R dtheta = 0
```

exactly, because every element `n x n = 0` pointwise.

Per-cap moments about the arc center: for a force `p A t(theta)` applied
at `P(theta) = R n(theta)`, `P x (p A t) = p A R (n x t) = p A R z`.
Hence

```text
-p A t_i at node i:  moment  -p A R z   (magnitude p A R, NOT zero)
+p A t_j at node j:  moment  +p A R z   (magnitude p A R, NOT zero)
```

Each cap force separately carries center-moment magnitude `p A R`; only
the PAIR-sum vanishes: `(-p A R) + (+p A R) = 0`.

## 3. Exact Self-Equilibrium of the Complete System

Net force of caps plus wall:

```text
(-p A t_i) + (+p A t_j) + p A (t_i - t_j) = 0     (identically)
```

Net moment about the arc center: `(-p A R) + (+p A R) + 0 = 0`. With zero
net force, transporting the moment to any other point `P` changes nothing
(`M_P = M_O + (r_O - r_P) x F = M_O`), so the complete system has zero
net force and zero net moment about EVERY point. Neither part alone is a
statically equivalent replacement for the other: the cap pair is the wall
load's equilibrant (their sum is zero), not its equivalent.

## 4. Why Static Lumping Cancels to Zero (Consistent Vector Required)

The canonical segment-equilibrium lumping of the wall load to the two end
nodes is `+p A t_i` at `i` and `-p A t_j` at `j`: it reproduces the wall
resultant `p A (t_i - t_j)` and its zero center moment
(`+p A R - p A R = 0`). Added to the cap pair `{-p A t_i, +p A t_j}` this
cancels to exactly zero AT EACH NODE. A statically-lumped-to-nodes
version of the complete system is therefore the zero vector and produces
no deformation at all — the spatial distribution of the wall load IS the
physics. The wall load must enter the assembled solve through its exact
work-equivalent consistent nodal vector (fixed-end forces and moments
from the force method on the same strain-energy model as the element
stiffness), exactly as the crate's uniform distributed load already does.
The same conclusion holds for any statics-only two-node reduction of the
complete self-equilibrated system: its nodal resultants are zero by
Section 3, so only a work-equivalent vector carries the load.

## 5. Consistent-Vector Derivation Basis (Existing Closed-Form Machinery)

Internal actions of the wall load at arc angle `theta`, from equilibrium
of the far segment `[theta, Phi]` (the same section convention as the
crate's uniform-load machinery): segment force
`f(theta) = Int_theta^Phi q ds = p A (sin Phi - sin theta,
cos theta - cos Phi, 0)` and segment moment about the section point
`m(theta) = Int_theta^Phi (P(u) - P(theta)) x q du`. Projections:

```text
axial            N_q(theta)  = f . t = p A (1 - cos(Phi - theta))
in-plane moment  M_q(theta)  = m . z = p A R (cos(Phi - theta) - 1)
out-of-plane moment = torsion = 0    (the load lies in the bend plane)
```

Both actions expand over the plain `{1, cos theta, sin theta}` basis:

```text
N_q: p A   [1, -cos Phi, -sin Phi]
M_q: p A R [-1, cos Phi,  sin Phi]
```

— strictly inside the crate's existing exact trigonometric integration
family (the uniform-load case needs the extended
`{1, cos, sin, theta, theta cos, theta sin}` basis; the radial load needs
only the plain sub-basis). The free-tip deflection integrals, the
clamped-tip redundant `X = -K_jj delta0`, and the rigid node-i share
with the load resultant `p A (sin Phi, 1 - cos Phi, 0)` and node-i moment
`p A R (cos Phi - 1) z` (from
`Int_0^Phi (P - P_i) x q ds`, worked directly:
`(P(theta) - P(0)) x q ds = p A (-sin theta) z R dtheta`, integral
`p A R (cos Phi - 1) z`) therefore all evaluate in closed form with the
machinery already used by the uniform consistent-load path. No new
integration technology is required.

## 6. Membrane-State Station Identity

Equilibrium of the near segment `[0, theta]` of the completely loaded arc
(cap force `-p A t_i` at node `i` plus wall load over `[0, theta]`, whose
resultant is `p A (t_i - t(theta))`):

```text
external force on near segment = -p A t_i + p A (t_i - t(theta))
                               = -p A t(theta)
```

so the far side acts on the cut with internal force `+p A t(theta)`:
tension of magnitude `p A` along the LOCAL TANGENT at every interior
station, with exactly zero transverse (shear) component. Internal moment
about the section point: the cap force contributes
`(P(0) - P(theta)) x (-p A t_i)`, z-component
`-p A R (1 - cos theta)`; the near-segment wall load contributes
`+p A R (1 - cos theta)` (same integral as Section 5 taken over
`[0, theta]`); the sum is identically zero. The completely loaded arc is
therefore in the pure MEMBRANE state — internal axial `+p A` along the
local tangent, zero shear, zero internal moment at every interior station
— the closed-vessel wall-tension result emerging with no ad-hoc chord
correction. Equivalently in the far-segment convention: with the true
node-j member force `+p A t_j`, section force
`p A t_j + p A (t(theta) - t_j) = p A t(theta)` and section moment
`p A R (1 - cos(Phi - theta)) + p A R (cos(Phi - theta) - 1) = 0`.

## 7. Small-Angle Reduction to the Chord Treatment

As `Phi -> 0`, both unit end tangents converge to the unit chord `c`
(`|t(theta) - c| <= Phi/2 + O(Phi^2)`), so the cap pair
`{-p A t_i, +p A t_j}` reduces to the existing straight-span treatment
`{-p A c, +p A c}`; the wall-load resultant
`|p A (t_i - t_j)| = 2 p A sin(Phi/2) -> 0` vanishes as `O(Phi)`, and its
consistent nodal vector vanishes with it. The complete arc treatment
therefore contains the recorded straight-chord treatment as its exact
straight limit.

## 8. Closed-Form Fixture Values (Quarter-Circle, Anchored-Free)

### Invented Inputs

All values are invented user inputs per the repository invented-fixture
convention. They are not material-standard, dimensional-standard, or
code-derived values, even where they resemble plausible pipe numbers.

| Symbol | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `R` | 1.4 | m | length |
| `Phi` | pi/2 | rad | angle |
| `D_o` | 0.2191 | m | length |
| `t_w` | 0.0081 | m | length |
| `E` | 195.0e9 | Pa | stress |
| `G` | 76.0e9 | Pa | stress |
| `p` | 2.5e6 | Pa | pressure |
| `k` | {1, 2} | - | dimensionless (user-entered bending flexibility factor) |

Geometry and frame: arc center at the origin; node `A` (end i) at
`(R, 0, 0)`, node `B` (end j) at `(0, R, 0)`; the arc local frame
coincides with the global frame. End tangents `t_A = (0, 1, 0)`,
`t_B = (-1, 0, 0)`. Node `A` is fully clamped (all six DOFs); node `B` is
free. The only loading is the complete pressure system of this document:
cap force `+F_p t_B` applied at `B` (the cap force at the clamped node
`A` passes into the support), plus the consistent nodal vector of the
outward radial wall load `q = (F_p / R) n(theta)`.

### Derived Section Properties (longhand)

```text
D_i = D_o - 2 t_w = 0.2191 - 0.0162 = 0.2029 m

A   = (pi/4) D_i^2               = 3.233359360399314e-2 m^2   (internal / thrust area)
A_s = (pi/4) (D_o^2 - D_i^2)     = 5.369296004250315e-3 m^2   (steel area)
I   = (pi/64) (D_o^4 - D_i^4)    = 2.992483836450838e-5 m^4
J   = 2 I                        = 5.984967672901677e-5 m^4

F_p  = p A   = 2.5e6 * 3.233359360399314e-2 = 8.083398400998286e4 N
E A_s        = 1.047012720828812e9 N
```

### Membrane Consequences (exact, independent of `k`, `E I`, `G J`)

By Sections 3 and 6 the completely loaded arc is in the pure membrane
state, so:

- **Anchor reactions at `A` are identically zero** (all six components):
  the applied system is self-equilibrated, so the support carries
  nothing.
- **True member force at `B`** is the cap force itself,
  `+F_p t_B = (-F_p, 0, 0)`; in the section frame at `B` this is pure
  axial `+F_p`.
- **Interior stations** at fractions {0.25, 0.5, 0.75}: axial `+F_p`,
  zero shear (both components), zero torsion, zero bending (both
  components).

### Tip Displacements (unit-load theorem on the membrane state)

With internal actions `N = F_p`, `M = T = 0` everywhere, the tip
deflection for unit tip load case `m` reduces to the axial term alone:
`delta[m] = (F_p R / (E A_s)) Int_0^Phi N_m dtheta`, with unit-case axial
fields `N_m` (far-segment convention): `N_Fx = -sin theta`,
`N_Fy = cos theta`, all other cases zero. Hence, for any `Phi`:

```text
u_x  = -(F_p R / (E A_s)) (1 - cos Phi)
u_y  = +(F_p R / (E A_s)) sin Phi
u_z = th_x = th_y = th_z = 0
```

(Cross-check by deformed geometry: with `M = 0` the turning per unit
reference arc length is unchanged, so the arc stretches uniformly by
`eps = F_p / (E A_s)` at constant total turning: deformed radius
`R (1 + eps)` — the pressurized bend expands radially outward and tends
to open, never to close. Node `B` relative to the clamped end `A` moves
by `eps R (-(1 - cos Phi), sin Phi, 0)`, identical to the unit-load
result.)

Quarter circle (`Phi = pi/2`), numeric values:

```text
eps = F_p / (E A_s) = 8.083398400998286e4 / 1.047012720828812e9
u_x = -F_p R / (E A_s) = -1.080861534560850e-4 m
u_y = +F_p R / (E A_s) = +1.080861534560850e-4 m
```

These values are exactly independent of the user factor `k` (and of
`E I`, `G J`): the fixture records them for `k in {1, 2}` and the
equality of the two rows is itself a structural check that the complete
load system excites no bending.

### Recorded Fixture Table

| Quantity | k = 1 | k = 2 | Unit |
|---|---:|---:|---|
| `u_x(B)` | -1.080861534560850e-4 | -1.080861534560850e-4 | m |
| `u_y(B)` | +1.080861534560850e-4 | +1.080861534560850e-4 | m |
| member force `F_x(B)` (local frame) | -8.083398400998286e4 | -8.083398400998286e4 | N |
| station axial, f in {0.25, 0.5, 0.75} | +8.083398400998286e4 | +8.083398400998286e4 | N |
| station shear/torsion/bending, all f | 0 (exact) | 0 (exact) | N, N m |
| anchor reaction components at `A` | 0 (exact) | 0 (exact) | N, N m |

Zero rows are compared with the DEC-026 near-zero floor convention
already recorded by the mechanics suite (relative tier with a
fixture-local absolute floor for exact-zero targets; this fixture uses a
1.0 N / N m / m floor, holding zero targets to 1.0e-9 absolute — about
1e-14 of the 8.1e4 N thrust scale, single-ulp closed-form roundoff
headroom only).

## Verification Appendix

Independent checks performed on the derivation:

1. **Resultant integral (Section 2)** re-derived two ways: direct
   antiderivatives (`Int cos = sin`, `Int sin = 1 - cos` over `[0, Phi]`)
   and the telescoping identity
   `Int q ds = -p A Int (dt/ds) ds = p A (t_i - t_j)`.
2. **Zero wall moment about the center** is pointwise exact
   (`n x n = 0`); no integration needed.
3. **Membrane identity (Section 6)** verified in both segment
   conventions (near segment `[0, theta]` and far segment
   `[theta, Phi]`), which agree identically; the shear and moment
   cancellations are term-by-term algebraic identities.
4. **Static-lumping cancellation (Section 4)** verified node-by-node:
   `(-p A t_i) + (+p A t_i) = 0` at `i`, `(+p A t_j) + (-p A t_j) = 0`
   at `j`.
5. **Tip displacements** re-derived by the independent deformed-geometry
   argument above; both routes give
   `eps R (-(1 - cos Phi), sin Phi, 0)` exactly.
6. **Small-angle reduction (Section 7)**: `|t_i - c|` and `|t_j - c|`
   equal `sin(Phi/2) - ...` bounded by `Phi/2`; the wall resultant
   magnitude `2 p A sin(Phi/2)` vanishes linearly. Both are elementary
   trigonometric identities.

The same identities are enforced as executable checks in the
curved-bend crate tests, the product-physics tests, and the benchmark
fixture comparison of this tranche.

## Boundaries

- The derivation is beam-level statics on the arc centerline; shell
  effects (cross-section ovalization, Bourdon straightening beyond the
  beam-level opening tendency) are outside the model, consistent with the
  macro-element's strain-energy formulation.
- Values are reference mechanics for verification only; nothing here is a
  design-code result, an allowable, or a compliance statement.
