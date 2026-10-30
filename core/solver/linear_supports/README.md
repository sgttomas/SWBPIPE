# Linear Supports

This crate is the bounded implementation slice for `DEL-04-03`. It models
linear supports and restraints as explicit mechanics-boundary inputs for the
3D frame-kernel DOF order.

The crate covers anchors, guides, line stops, vertical supports, springs, and
imposed displacement boundary data. It does not implement nonlinear gap,
lift-off, one-way, friction, or active-set behavior; that remains assigned to
`DEL-04-04`.

## Boundary

- Inputs are explicit model values that upstream schema and unit layers must
  validate for unit compatibility.
- Unit-bearing support values retain dimension intent, such as translational
  stiffness, rotational stiffness, displacement, or rotation.
- `SupportQuantity` maps that intent onto the accepted PKG-02 canonical
  dimensions `linear_stiffness`, `rotational_stiffness`, `displacement`, and
  `rotation`, and can carry caller-supplied unit-system and unit identifiers at
  external solver boundaries. The crate records metadata only and performs no
  conversion or defaulting.
- Missing solve-required support data produces deterministic findings rather
  than silent defaults.
- The crate prepares restrained DOFs, linear spring entries, and imposed
  displacement entries.
- `FrameDof` is re-exported from the frame kernel, and support DOF indices use
  the frame-kernel `node_dof_index` boundary instead of re-derived offsets.
- `apply_linear_supports` validates the dense global system through the frame
  kernel, adds prepared spring stiffness to global diagonals, and reduces rigid
  restraints plus imposed displacements through the frame-kernel prescribed
  displacement boundary.
- It does not alter the frame-kernel solver, select a sparse solver, perform
  load-case algebra, run rule-pack checks, or make professional/code-compliance
  claims.

## Tests

The unit tests cover support-family DOF mapping, missing-data findings,
duplicate restraint findings, spring and imposed-displacement validation, and
deterministic boundary preparation/application.
