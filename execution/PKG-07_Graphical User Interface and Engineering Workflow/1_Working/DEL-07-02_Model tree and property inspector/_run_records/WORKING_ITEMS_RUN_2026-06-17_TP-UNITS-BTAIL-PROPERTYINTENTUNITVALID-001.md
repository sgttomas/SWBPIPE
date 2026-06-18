# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-PROPERTYINTENTUNITVALID-001

Date: 2026-06-17
Persona: WORKING_ITEMS
Primary deliverable: DEL-07-02 Model tree and property inspector
Supporting deliverables: DEL-02-02 Unit system and dimensional-analysis core contract; DEL-16-02 Operation validation and diff preview
Tranche: TP-UNITS-BTAIL-PROPERTYINTENTUNITVALID-001
Smoke evidence: TP-MAC-206

## Scope

Bounded Phase B-tail Property Inspector/unit-validation slice while C5.7
remains human-execution gated. The tranche records unit-dimension validation
status on Property Inspector unit-bearing modify intents and explicit
create-section/create-material/create-support intents.

## Changes

- `PropertyInspector` passes the loaded DEC-018 unit-catalog route into its
  editor-intent builders.
- Unit-bearing modify intents now derive `validation.unit_validation` from the
  existing `describeUnitBasis` metadata instead of reporting `not_run`.
- Create-section, create-material, and create-support intents report
  dimension-specific unit-validation statuses for length, stress, thermal
  expansion, and linear stiffness quantities.
- Browser preview remains explicit: because the reviewed catalog command is
  desktop-only, it records
  `model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview`.
- App Vitest and Playwright smoke assertions now pin the visible
  `editor-intent-validation` status on edit and create paths.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` passed 55/55 tests.
- `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` passed 14/14
  Playwright tests.
- `npm test --workspace apps/desktop` passed 18/18 files and 397/397 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  large-chunk warning.

## Boundary Review

Property Inspector operation-intent metadata only. No DEC-018 catalog constant
change, schema dimension enum change, unit conversion API change, solver
behavior, operation application semantics, accepted model-state mutation,
protected standards content, private payload, lifecycle state transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.

## Handoff

- Completion plan B-tail row updated with TP-MAC-206.
- `apps/desktop/SMOKE.md` and `plans/PLAN_COMPLETION_LOG.md` updated.
- Remaining B-tail scope stays open: broader app unit entry/pickers beyond
  landed surfaces and target-format conversion witnesses outside already
  covered boundaries.
