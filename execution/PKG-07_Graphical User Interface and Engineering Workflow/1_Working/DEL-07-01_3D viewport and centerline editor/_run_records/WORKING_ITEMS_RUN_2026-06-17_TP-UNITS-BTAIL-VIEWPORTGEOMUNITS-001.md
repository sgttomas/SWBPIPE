# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-VIEWPORTGEOMUNITS-001

Date: 2026-06-17
Persona: WORKING_ITEMS
Primary deliverable: DEL-07-01 3D viewport and centerline editor
Supporting deliverable: DEL-02-02 Unit system and dimensional-analysis core contract
Tranche: TP-UNITS-BTAIL-VIEWPORTGEOMUNITS-001
Smoke evidence: TP-MAC-209

## Scope

Bounded Phase B-tail viewport-geometry/unit-validation slice while C5.7
remains human-execution gated. The tranche closes a unit-I/O evidence gap in
explicit viewport node and straight-pipe authoring intents.

## Changes

- `PipeViewport` now imports the shared `unitDimensionValidationStatus`
  helper from `unitCatalogService`.
- Explicit create-node and connect-pipe queued intents receive the current
  viewport unit-catalog route and record
  `length=<unit-dimension-validation-status>` in
  `validation.unit_validation`.
- The visible viewport intent card now exposes the queued unit-validation
  status through `data-testid="viewport-intent-unit-validation-*"` so browser
  smoke can verify the evidence before the operation is applied.
- Generic one-click viewport gesture placeholders remain
  `unit_validation=not_run` because they are intentionally underspecified and
  still blocked rather than inventing geometry/unit payloads.

## Evidence

- Browser preview records
  `length=model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview`.
- Desktop/Tauri catalog routes can record DEC-018 accepted dimension matches,
  mismatch, loading, or unreviewed statuses through the same shared helper.
- The A12 from-blank Playwright journey now checks queued viewport create-node
  and connect-pipe unit-validation evidence in both desktop and compact
  viewports.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` passed 55/55 tests.
- `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` passed
  14/14 Playwright tests.
- `npm test --workspace apps/desktop` passed 18/18 files and 398/398 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  large-chunk warning.

## Boundary Review

No DEC-018 catalog constant change, schema dimension enum change, unit
conversion API change, operation-applier validation semantics, solver behavior,
protected standards content, private payload, lifecycle state transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.

## Residual

Phase B-tail still owns broader app unit entry/pickers outside the covered
viewport/property/load/rule/library surfaces and remaining target-format
conversion witnesses outside the already-covered export boundaries.
