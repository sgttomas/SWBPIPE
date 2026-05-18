# TASK RUN - TP-PHYS-008-B

## Scope

- Deliverable: `DEL-05-01 - Primitive load case engine`
- Tranche: `TP-PHYS-008 Axial Effects For Thermal And Pressure`
- Write scope used: `core/loads/primitive_loads/**`, deliverable
  `MEMORY.md`, and this deliverable-local `_run_records/**`.

## Work Performed

- Added `ElementAxialEffectProperties` with explicit optional properties for
  elastic modulus, area, thermal expansion coefficient, and internal area.
- Added `PrimitiveAxialEffectContribution` and
  `PrimitiveAxialEffectApplication`.
- Added `prepare_straight_pipe_axial_effects(...)` for deterministic
  derivation of thermal axial force `E*A*alpha*DeltaT` and pressure thrust
  `p*A_internal`.
- Added deterministic findings for wrong targets, wrong dimensions, missing
  properties, missing physical properties, invalid/non-finite physical
  properties, non-finite load magnitudes, non-finite computed axial effects,
  and out-of-range element indices. Any finding blocks all returned axial
  effect contributions.
- Added focused unit tests for valid thermal/pressure derivation, blocked
  output behavior, and overflowing computed axial-force rejection.

## Validation

- `cargo fmt --manifest-path core/loads/primitive_loads/Cargo.toml --check`
  passed.
- `cargo test --manifest-path core/loads/primitive_loads/Cargo.toml` passed:
  28 tests, 0 failures.

## Boundary Notes

- No lifecycle state, dependency register, DAG, blocker queue, candidate row,
  DEV-001 finding disposition, coordination artifact, protected standards
  content, private data, code-compliance claim, release claim, or professional
  reliance claim was changed or introduced.
- Remaining TBDs are preserved: canonical unit conversions, production
  tolerance policy, load-case storage, final result envelopes/API integration,
  release thresholds, and professional reliance.
