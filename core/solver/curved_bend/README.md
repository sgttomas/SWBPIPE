# Curved Bend Macro-Element

This crate is the P1 formulation sub-tranche of the `DEC-070` ruling (D-34
Option O-B): a dedicated arc-consistent curved-bend macro-element carrying
user-entered in-plane and out-of-plane flexibility factors, assembled per the
expansion-joint `UserStiffnessElement` precedent. Formulation only — no
product-physics integration, schema change, or recovery path.

## Scope

- Two-node circular-arc bend element defined by end nodes and the arc center.
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
basis (`phi`, `sin phi`, `1 - cos phi`, `phi/2 +- sin(2 phi)/4`,
`sin^2(phi)/2`) — no numerical quadrature. Shear deformation is excluded,
consistent with the frame kernel.

`k_in` scales only the in-plane bending strain-energy term and `k_out` only
the out-of-plane bending-curvature term; torsion and axial terms are
untouched. Out-of-plane bending and torsion couple through the arc geometry;
that coupling is carried by the integrals themselves, with `k_out` applied
inside the bending term only.

The tip stiffness is `K_jj = F^{-1}` (dense partial-pivot solve via the
frame-kernel `solve_dense`, one unit column at a time, then symmetrized), and
the full 12x12 follows from the rigid equilibrium transfer
`H = [[I, 0], [skew(x_j - x_i), I]]`:

```
K = [[ H K_jj H^T, -H K_jj ],
     [ -K_jj H^T,   K_jj   ]]
```

formed in the local bend-plane frame (x radial at node `i`, z the bend-plane
normal, node `i` at arc angle 0) and rotated to global coordinates with the
frame-kernel orientation transform.

Geometry is parameterized by the arc center: the constructor requires both
end nodes equidistant from the center (relative tolerance 1e-9) and an
included angle strictly inside `(0, pi)`; the bend plane is derived from
`(i - center) x (j - center)`, so degenerate and collinear inputs are
rejected rather than guessed at.

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
transformation under a rigid rotation of the whole geometry; and constructor
rejection of degenerate geometry and nonpositive or nonfinite inputs.
