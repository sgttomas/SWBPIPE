# WORKING_ITEMS Run Record - TP-UNITS-B2-VIEWPORTDRAFTUNITS-001

## Scope

- Persona: WORKING_ITEMS.
- Tranche: B2/B3 viewport draft length-unit controls.
- Primary evidence for this deliverable: operation-seam handling for viewport
  `create_node` and `connect_pipe_run` intents.

## Changes

- `resolve_create_node` now accepts compatible accepted DEC-018 length units
  instead of requiring the intent unit to equal `project.units.length`.
- Node coordinate values are converted to the model document's stored
  `project.units.length` scalar basis before storage.
- `resolve_connect_pipe_run` now accepts compatible accepted DEC-018 length
  units for OD/wall quantities, checks wall thickness after conversion, and
  preserves the entered pipe-section quantity unit in the created segment.
- Added operation-applier witnesses for millimetre node coordinates normalized
  to metres and millimetre pipe OD/wall values preserved after validation.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check`
  passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml explicit_`
  passed 28/28 filtered explicit tests.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  passed 60 unit tests, the canonical hash test, the contract corpus tests,
  and doc tests.
- Desktop focused/full Vitest, build, focused R2 Playwright, full Playwright,
  and in-app Browser viewport verification passed in the primary DEL-07-01
  validation run.
- `python3 tools/release/run_evidence_sweep.py --execute` passed on the dirty
  tree and wrote
  `validation/evidence/sweeps/SWEEP_20260616T033649Z_40c8d2530ca8-dirty.json`.

## Boundary

- No direct mutation bypass, hidden unit fallback, protected content, private
  project data, network/telemetry path, release-readiness claim, professional
  approval, certification, sealing, authentication, or code-compliance claim
  changed.

## Residual

- This tranche covers viewport create-node/connect-pipe draft length units
  only; broader operation-unit edit/create coverage remains governed by the
  B2/B3 completion-plan residuals.
