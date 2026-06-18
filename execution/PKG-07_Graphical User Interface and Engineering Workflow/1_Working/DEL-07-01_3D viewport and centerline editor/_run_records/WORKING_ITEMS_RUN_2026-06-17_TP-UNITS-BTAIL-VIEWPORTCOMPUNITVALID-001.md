# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-VIEWPORTCOMPUNITVALID-001

Date: 2026-06-17
Persona: WORKING_ITEMS
Primary deliverable: DEL-07-01 3D viewport and centerline editor
Supporting deliverables: DEL-02-02 Unit system and dimensional-analysis core
contract; DEL-16-02 Operation validation and diff preview
Tranche: TP-UNITS-BTAIL-VIEWPORTCOMPUNITVALID-001
Smoke evidence: TP-MAC-218

## Scope

Bounded Phase B-tail viewport component-symbol unit-validation slice while
C5.7 remains human-execution gated. The tranche closes the remaining live
viewport operation-intent unit-validation marker for the reference-only
component-symbol placeholder.

## Changes

- `PipeViewport` now classifies the generic `insert_component_symbol`
  placeholder as `unit_validation=not_required_dimensionless`.
- The generic `create_node` and `connect_pipe_run` placeholder gestures remain
  `unit_validation=not_run` because they are intentionally underspecified
  geometry gestures; explicit create-node and connect-pipe authoring continue
  to carry length unit-validation evidence through the existing shared helper.
- The visible viewport intent card already renders
  `data-testid="viewport-intent-unit-validation-insert_component_symbol"`,
  so the browser smoke now asserts the component-symbol dimensionless status
  directly.

## Evidence

- App test coverage pins the distinction between underspecified geometry
  placeholders (`not_run`) and the reference-only component-symbol placeholder
  (`not_required_dimensionless`).
- Playwright R2/R3 smoke now includes a focused component-symbol placeholder
  check in both desktop and compact viewports.

## Validation

- `npm test --workspace apps/desktop -- src/App.test.tsx -t "viewport editor intents"`
  passed 1/1 focused App test.
- `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` passed
  16/16 Playwright tests.
- `npm test --workspace apps/desktop` passed 18/18 files and 399/399 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  large-chunk warning.

## Boundary Review

No DEC-018 catalog constant change, schema dimension enum change, unit
conversion API change, operation-applier validation semantics, operation
application behavior, accepted model-state mutation, solver behavior,
protected standards content, private payload, lifecycle state transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.

## Residual

Generic node and pipe-run viewport placeholders remain intentionally
underspecified and continue to report `unit_validation=not_run`; explicit
node and pipe authoring paths already carry length unit-validation evidence.
