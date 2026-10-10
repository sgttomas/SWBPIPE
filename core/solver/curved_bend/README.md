# Curved Bend Macro-Element

This crate is the P1 formulation sub-tranche of the `DEC-070` ruling (D-34
Option O-B): a dedicated arc-consistent curved-bend macro-element carrying
user-entered in-plane and out-of-plane flexibility factors, assembled per the
expansion-joint `UserStiffnessElement` precedent. Formulation only — no
product-physics integration, schema change, or recovery path.

## Scope

- Two-node circular-arc bend element defined by its end nodes, the user bend
  radius `R` and a plane reference vector `y_reference` (T4-U1): the
  parameterization `(x_i, x_j, R, y_reference)`. No absolute arc centre is
  formed, so the element depends on the nodes only through `d = x_j - x_i`.
- 12x12 global stiffness in the frame-kernel DOF order ([ux, uy, uz, rx,
  ry, rz] at node `i` then node `j`), plus the local end-flexibility matrix
  and the local frame used.
- User-entered in-plane and out-of-plane bending flexibility factors consumed
  as validated opaque numbers (finite, positive). Factors of 1 reproduce the
  plain Euler-Bernoulli curved beam.

## Method

The element is built by a flexibility formulation. With node `i` fixed, the
6x6 end-flexibility matrix `F` at node `j` is the unit-load (Castigliano)
integral over the arc angle `theta` in `[0, phi]` of bending, torsion, and
axial strain-energy products:

```
F_ab = R * integral( k_in  * Mip_a * Mip_b / (E I)
                   + k_out * Mop_a * Mop_b / (E I)
                   +         T_a   * T_b   / (G J)
                   +         N_a   * N_b   / (E A) ) d theta
```

where, for each of the six unit loads applied at node `j`, `Mip` is the
bending moment about the bend-plane normal (in-plane bending), `Mop` the
bending moment about the local radial axis (out-of-plane bending), `T` the
torsion about the local tangent, and `N` the axial force. Every internal
action is a linear combination of `{1, cos theta, sin theta}`, so each entry
of `F` reduces to exact closed-form integrals of pairwise products of that
basis — no numerical quadrature. Shear deformation is excluded, consistent
with the frame kernel.

Stable form (`src/arc_integrals.rs`): `F` is written as non-cancelling terms
plus eight integrals that cancel as `phi -> 0` (`phi - S`, `phi - S C`,
`S - phi C`, `(1 - C) - phi S/2` and four squared or cross terms of
`S - sin theta`, `C - cos theta`, `1 - cos psi`, with `S = sin phi`,
`C = cos phi`). Each is evaluated as a fixed-length Horner series
`phi^m * sum a_k phi^(2k)` (exact rational coefficients rounded once) when the
half-angle sine `s < 1/2` (`phi < pi/3`), and otherwise in closed form from
the half-angle quantities (`S = 2sc`, `C = 1 - 2s^2`, `1 - C = 2s^2`). The
switch compares the correctly rounded `s` with the exact constant 1/2, so it
is bitwise deterministic and libm-independent; the two forms agree at the
switch to rounding (jump at most 5.0e-15 relative).

`k_in` scales only the in-plane bending strain-energy term and `k_out` only
the out-of-plane bending-curvature term; torsion and axial terms are
untouched. Out-of-plane bending and torsion couple through the arc geometry;
that coupling is carried by the integrals themselves, with `k_out` applied
inside the bending term only.

The tip stiffness is `K_jj = F^{-1}` (dense partial-pivot solve via the
frame-kernel `solve_dense`, one unit column at a time, then symmetrized).
`F` is first multiplied by one power of two `c >= 1` that brings its largest
diagonal entry into `[1, 2)`, and the solution by `c` again: every operation
scales exactly, so the inverse is bit for bit the unscaled one wherever that
is formed, while the solver's absolute pivot guard acts at the matrix's own
scale and does not refuse short arcs for the size of their flexibilities.
The full 12x12 follows from the rigid equilibrium transfer
`H = [[I, 0], [skew(x_j - x_i), I]]`:

```
K = [[ H K_jj H^T, -H K_jj ],
     [ -K_jj H^T,   K_jj   ]]
```

formed in the local bend-plane frame and rotated to global coordinates with
the frame-kernel orientation transform. `H` uses the chord in the local frame,
`(-sL, cL, 0)`, so `K` annihilates the rigid motions of the actual nodes.

Geometry (`objective_arc`, shared with `arc_geometry`): with `L = |d|`,
`d^ = d/L` and `n^` the unit component of `y_reference` normal to `d^` (two
Gram-Schmidt passes; the arc bows toward `+n^`), `|d|^2` and the span
`4R^2 - |d|^2` are each formed from exact products and rounded once, so the
geometry stays accurate near `pi`. Then `s = sin(phi/2) = L/(2R)`,
`c = cos(phi/2) = sqrt(4R^2 - L^2)/(2R)` and `phi = 2 atan2(s, c)`. The local
axes are `x = -s d^ + c n^` (radial at node `i`, outward), `y = c d^ + s n^`
(tangent at node `i`) and `z = n^ x d^` (bend-plane normal); node `i` sits at
arc angle 0. Inputs are refused rather than guessed at: non-finite or
nonpositive inputs, coincident nodes (`DegenerateArc`), `4R^2 <= |d|^2`
(`RadiusCannotSpanChord`), a `y_reference` parallel to the chord
(`DegenerateArc`), and an included angle outside `[1e-9, pi - 1e-9]`
(`IncludedAngleOutOfRange`).

## Boundary

All flexibility factors are user-entered opaque numbers. This crate does not
provide or evaluate any flexibility-factor or stress-intensification formula,
pipe tables, material defaults, code-specific checks, protected standards
data, or engineering approval.

## Verification

In-crate unit tests cover: straight-limit convergence to the frame-kernel
straight element over a fixed chord (first-order in the included angle;
documented relative tolerance 1e-3 at `phi = 1e-3`, with a convergence
check); quarter-circle in-plane and out-of-plane cantilever tip responses
against independently hand-written closed-form integrals, including the
out-of-plane bending-torsion coupling; exact affine scaling of the
flexibility by each user factor with torsion and axial parts unchanged;
rigid-body nullspace; symmetry and nonnegative strain energy; congruent
transformation under a rigid rotation of the whole geometry; constructor
rejection of degenerate geometry and nonpositive or nonfinite inputs; the
stable series against the closed forms and their switch (`arc_integrals`);
and short arcs (0.06 to 5 degrees) against frozen high-precision references,
bit-identical under translation (`short_arc_tests`). The product's reference
tests against T4-I6's frozen element live in `core/product_physics/tests/
t4_u1_reference.rs`.
