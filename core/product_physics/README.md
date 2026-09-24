# Product mechanics adapter

The native desktop and headless runner use `run_linear_static_preview_with_mode`
to map model, material, support and load inputs to the mechanics crates and return
quantities plus diagnostics. The desktop browser fallback is a fixture path; it
is not execution of this Rust solver.

Local model inputs require nonblank provenance. Their text is preserved: a word
such as “invented” or “cleared” is neither required nor evidence of rights,
engineering review or publication clearance. Bundled fixtures and public exports
retain their separate data/privacy/protected-content boundaries. This crate does
not supply code allowables or professional acceptance.

## Input behavior

- Authored moment directions `rotation_x`, `rotation_y` and `rotation_z` use the
  same rotational DOFs as the retained `RX`, `RY` and `RZ` spellings.
- A nonlinear support record retains compatible, disjoint rigid restraints.
  Unsupported overlapping laws, conflicting spring axes, unused stiffness and
  contradictory spring/constant-effort declarations produce explicit diagnostics.
  A constant-effort force axis is not a rigid restraint; separate spring or
  constant-force records may coexist with a contact at the same node and DOF.
- A pressure-dimension primitive with category `hydrotest` currently blocks with
  an unsupported-input diagnostic. It is not silently ignored or treated as an
  implemented hydrotest procedure. Data may still be edited and saved. Full
  hydrotest pressure, fluid and support-state behavior remains separate work.
- Unit normalization uses the shared unit catalog. New project temperature
  defaults use `degC`; legacy `C` is a dimension-scoped compatibility spelling for
  temperature and temperature intervals, without rewriting saved input bytes.
  Pressure units do not select a gauge/absolute reference convention.

## Verification and limits

The maintained input-contract regressions exercise the actual product entrypoint
in both solver modes. Signed and rotated moment controls use independent
Euler–Bernoulli/Saint-Venant references under their stated assumptions. The
`physics_audit_regression/tests/authored_units_product.rs` tests also cross the
real model operation applier, normalization and solver boundary.

Those checks do not establish complete engineering validation. Pressure
formulation/recovery, solve-integrity and publication precision, complete signed
reactions, richer nonlinear states and other recorded findings remain distinct
requirements. Native authoring, solving, result inspection and persistence must
be witnessed on the integrated candidate; browser fixtures and crate passes do
not replace them. Historical result records retain their original producer,
values and hash semantics.
