# WORKING_ITEMS Run Record - TP-UNITS-B2-VIEWPORTDRAFTUNITS-001

## Scope

- Persona: WORKING_ITEMS.
- Tranche: B2/B3 viewport draft length-unit controls.
- Primary deliverable: DEL-07-01 3D viewport and centerline editor.
- Supporting deliverables: DEL-16-02 operation validation/diff preview and
  DEL-02-02 unit system contract.

## Changes

- `PipeViewport` create-node and straight-pipe draft forms now load the
  DEC-018 unit catalog when available and show an explicit browser
  model-metadata fallback when not available.
- Create-node drafts carry a selected coordinate length unit and show
  `Coordinates: ...` basis text before queueing.
- Straight-pipe drafts carry a selected pipe length unit and show
  `Pipe geometry: ...` basis text before queueing.
- Viewport smoke coverage now asserts the unit status, node selector, node
  basis text, pipe selector, and pipe basis text in both desktop and compact
  R2 preview viewports.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` passed 54/54.
- `npm test --workspace apps/desktop` passed 18 files / 388 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  chunk-size warning.
- `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
  passed 2/2.
- `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 10/10.
- In-app Browser at `http://127.0.0.1:4181/` verified
  `browser preview uses model metadata for viewport length units`, node unit
  `m`, pipe unit `m`, model-metadata basis text, disabled empty draft buttons,
  and an enabled filled node draft.
- `python3 tools/release/run_evidence_sweep.py --execute` passed on the dirty
  tree and wrote
  `validation/evidence/sweeps/SWEEP_20260616T033649Z_40c8d2530ca8-dirty.json`.

## Boundary

- No project-unit mutation, hidden unit fallback, protected content, private
  project data, network/telemetry path, release-readiness claim, professional
  approval, certification, sealing, authentication, or code-compliance claim
  changed.

## Residual

- Broader B2/B3 app unit entry/pickers outside material/section/node-
  coordinate/viewport-draft/primitive-load manager/inspector and rule-pack
  declaration/expression forms remain outside this tranche.
- Remaining target-format witnesses and D-04/DEC-026 tolerance corpus remain
  outside this tranche.
