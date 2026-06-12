# 3D Frame Stiffness Kernel

This crate is the conservative first slice for `DEL-04-01`. It provides a code-neutral mechanics kernel for 3D Euler-Bernoulli frame stiffness, coordinate transforms, boundary-condition reduction, global dense assembly, and a temporary dense solve interface.

## Scope

- Two-node 3D frame elements.
- Six degrees of freedom per node.
- Public node and element DOF mapping helpers for the fixed six-DOF ordering.
- Local 12 by 12 Euler-Bernoulli stiffness terms for axial, torsion, bending about local `y`, and bending about local `z`.
- Direction-cosine transforms from validated local element axes to global axes.
- Dense global assembly and boundary-condition reduction, including force adjustment for prescribed displacements.
- Deterministic dense Gaussian elimination with partial pivoting for small reduced systems.

## DOF Ordering

Each node uses this deterministic ordering:

```text
[ux, uy, uz, rx, ry, rz]
```

For an element, node `i` occupies indices `0..6` and node `j` occupies indices `6..12`.
Use `node_dof_index`, `element_dof_map`, `FrameDof`, and `NODE_DOF_ORDER` when external slices need to bind this ordering without re-deriving offsets.

## Current Limitation

The dense matrix and dense solve functions are interim interfaces. They are suitable for focused kernel verification and small tests only. The project sparse solver strategy is resolved by `DEC-023` as the in-repo skyline LDLᵀ direct solver (`core/solver/sparse_direct`); binding this crate's live solve boundary to that solver remains `TBD` as a separate bounded tranche, and the dense path additionally serves as the parity oracle for the sparse path. This crate intentionally introduces no external numerical dependency.

Inputs to this crate are numeric mechanics quantities that upstream schema and unit layers must already have validated for unit compatibility. The crate validates finite, positive, nondegenerate, orientation, node-index, boundary-condition, and singular-system conditions, but it does not define the project unit catalog, conversion constants, or solver tolerance policy.

The dense solve path has an internal zero-pivot guard for temporary verification behavior. That guard is not a release tolerance policy, conditioning policy, or sparse-solver selection.

Solver-facing external boundaries must bind the raw mechanics fields to `FrameKernelUnitBasis` metadata before schema, service, adapter, report, or persistence use. The unit basis carries caller-supplied unit-system and unit identifiers for length, force, moment, stress, area, `second_moment_area`, displacement, and rotation; it performs no unit conversion and supplies no defaults. Optional `CanonicalModelReference` records bind solver object references to PKG-02 model roles without changing the mechanics kernel.

The shared `CanonicalDimension` metadata vocabulary also exposes `force_per_length` for downstream solver/load boundaries that need to label distributed mechanics quantities. The frame kernel records the dimension identifier only; it still does not define a unit catalog, conversion constants, load defaults, or tolerance policy.

## Non-Compliance Boundary

This crate computes open mechanics quantities only. It does not encode compliance rules, code-specific values, protected standard content, material allowables, SIF/flexibility data, private project data, or engineering approval/certification claims.

## Verification

The unit tests cover six-DOF mapping, local stiffness terms, local-to-global transform behavior, global assembly, boundary-condition reduction, prescribed-displacement force adjustment, the temporary dense solve interface, singular-system detection, metadata boundaries, invalid node topology, repeated restraints, invalid orientation, and non-finite input rejection.

## Future Replacement Point

The intended replacement boundary is the reduced-system solve interface. Future work can preserve element stiffness, transforms, assembly, and reduction while replacing `solve_dense` with an accepted sparse solver adapter.
