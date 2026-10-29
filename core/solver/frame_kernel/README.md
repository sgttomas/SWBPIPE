# 3D Frame Stiffness Kernel

This crate is the conservative first slice for `DEL-04-01`. It provides a code-neutral mechanics kernel for 3D frame stiffness (Euler-Bernoulli, or the exact static Timoshenko element), coordinate transforms, boundary-condition reduction, global dense assembly, and a temporary dense solve interface.

## Scope

- Two-node 3D frame elements.
- Six degrees of freedom per node.
- Public node and element DOF mapping helpers for the fixed six-DOF ordering.
- Local 12 by 12 stiffness terms for axial, torsion, bending about local `y`, and bending about local `z`, for either bending theory of the section's `BeamTheory` leaf:
  - `EulerBernoulli` (`FrameSection::new`): 12EI/L³, 6EI/L², 4EI/L and 2EI/L, as before T4-U8.
  - `Timoshenko { shear_area }` (`FrameSection::timoshenko`, T4-U8/D-6): the exact static two-node element with shear rigidity `G * shear_area` in both planes. Per plane phi = 12EI/(G A_s L²), and the same positions hold 12EI/(L³(1+phi)), 6EI/(L²(1+phi)), (4+phi)EI/(L(1+phi)) and (2-phi)EI/(L(1+phi)). Axial EA/L uses the metal area and torsion GJ/L is unchanged. Nodal rotations are section rotations, not slopes. Every intermediate is a named K2a checked formation; 2-phi and its product may be exactly zero.
- `shear::annulus_energy_shear_factor` and `shear::shear_area`: the energy-matched annular shear factor kappa_E (`timoshenko_annulus_energy_v1`, not Cowper's or 5/6) in a fixed binary64 operation order, and A_s = kappa_E * A.
- The K-D5 formation check re-forms a Timoshenko frame at p from its own leaves (phi, 1+phi, 4+phi, 2-phi) and binds every straight load primitive's frame, leaf included, to its formation-source frame; a Timoshenko partial-span uniform load is re-formed as a ball enclosing its exact Timoshenko vector, with phi from the frame's own leaves, and a failure of that re-formation is unavailable (fail closed). Since O4 the retained W1 element forms Timoshenko members on route E (DEL-04-08 Design `o4-source-extension.md`); route L carries no shear model and refuses a Timoshenko leaf by name (`RETAINED_TIMOSHENKO_REFUSAL`); its D1 declines a pressure-free Timoshenko document first.
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

The unit tests cover six-DOF mapping, local stiffness terms, local-to-global transform behavior, global assembly, boundary-condition reduction, prescribed-displacement force adjustment, the temporary dense solve interface, singular-system detection, metadata boundaries, invalid node topology, repeated restraints, invalid orientation, and non-finite input rejection. `tests/timoshenko_element.rs` and the K-D5 Timoshenko unit tests check the Timoshenko element against SHEAR_REFERENCE's frozen controls (S-A, S-B, S-C, S-D).

## Future Replacement Point

The intended replacement boundary is the reduced-system solve interface. Future work can preserve element stiffness, transforms, assembly, and reduction while replacing `solve_dense` with an accepted sparse solver adapter.
