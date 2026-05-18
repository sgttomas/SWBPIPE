# TASK RUN - TP-PHYS-008-A

## Scope

- Deliverable: `DEL-04-02 - Straight pipe element`
- Tranche: `TP-PHYS-008 Axial Effects For Thermal And Pressure`
- Write scope used: `core/solver/straight_pipe/**`, deliverable `MEMORY.md`,
  and this deliverable-local `_run_records/**`.

## Work Performed

- Added `StraightPipeAxialEffect` with finite axial-force validation.
- Added local and global equivalent nodal load recovery for straight-pipe
  axial effects. Positive axial force applies `-F` at node I and `+F` at node
  J along local X before global transformation.
- Added local-force, end-resultant, station-resultant, and station-sweep
  recovery helpers that correct recovered local forces for axial effects.
- Added focused tests for orientation-aware global loads, fixed-end axial
  recovery, ordered station sweep recovery, and non-finite axial-force
  rejection.

## Validation

- `cargo fmt --manifest-path core/solver/straight_pipe/Cargo.toml --check`
  passed.
- `cargo test --manifest-path core/solver/straight_pipe/Cargo.toml` passed:
  30 tests, 0 failures.

## Boundary Notes

- No lifecycle state, dependency register, DAG, blocker queue, candidate row,
  DEV-001 finding disposition, coordination artifact, protected standards
  content, private data, code-compliance claim, release claim, or professional
  reliance claim was changed or introduced.
- Remaining TBDs are preserved: sparse solver library, global tolerance
  policy, canonical unit conversions, final result envelopes/export/API
  integration, release thresholds, and professional reliance.
