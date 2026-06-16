# WORKING_ITEMS Run Record - TP-UNITS-B2-VIEWPORTDRAFTUNITS-001

## Scope

- Persona: WORKING_ITEMS.
- Tranche: B2/B3 viewport draft length-unit controls.
- Supporting deliverable: DEL-02-02 unit system and dimensional-analysis core
  contract.

## Changes

- Viewport draft forms now make selected length units explicit for node
  coordinate and pipe geometry authoring.
- The browser preview fallback is labeled as model-metadata based; Tauri-
  capable paths can load accepted DEC-018 catalog entries for length units.
- The operation seam validates compatible DEC-018 length units, normalizes
  node coordinates to `project.units.length`, and preserves entered pipe
  section length units after compatible validation.

## Validation

- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml explicit_`
  passed 28/28 filtered explicit tests.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  passed 60 unit tests plus canonical hash and contract corpus checks.
- Desktop focused/full Vitest, build, focused R2 Playwright, full Playwright,
  and in-app Browser viewport verification passed in the primary DEL-07-01
  validation run.
- `python3 tools/release/run_evidence_sweep.py --execute` passed on the dirty
  tree and wrote
  `validation/evidence/sweeps/SWEEP_20260616T033649Z_40c8d2530ca8-dirty.json`.

## Boundary

- No DEC-018 catalog constant, schema dimension enum, tolerance policy,
  project-unit mutation, hidden unit fallback, protected content, private data,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## Residual

- Remaining B2/B3 unit I/O and witness/tolerance corpus work stays tracked in
  the active completion plan.
