# WORKING_ITEMS Run Record - TP-UNITS-B2-RULEEXPRUNITS-001

Date: 2026-06-15
Persona: WORKING_ITEMS
Deliverable: DEL-02-02 Unit system and dimensional-analysis core contract
Package: PKG-02 Domain Model, Units, and Core Schemas

## Scope

Bounded B2/B3 app-integration tranche for rule-pack expression literal/table
unit I/O. The slice extends the accepted DEC-018 desktop unit-catalog selector
pattern from rule-pack declarations into the C2 expression composer fields for
literal `quantity.unit_ref`, table `argument_unit_ref`, and table
`result_unit_ref`.

## Changes

- Updated `apps/desktop/src/features/rule-packs/ExpressionComposer.tsx` so the
  expression composer loads the desktop `get_unit_catalog` route only when the
  Tauri runtime is present.
- In desktop mode, literal and table unit fields render dimension-filtered
  DEC-018 selectors: accepted compatible units are offered, incompatible
  catalog units are absent, and stored out-of-catalog units remain visible as
  the current value rather than being silently snapped.
- In browser preview mode, literal and table unit fields keep the previous
  manual text-entry path and do not synthesize a fallback unit catalog.
- Extended `ExpressionComposer` mocked-Tauri tests for literal and table unit
  selectors, out-of-catalog preservation, and browser manual fallback.
- Extended the existing Playwright rule-pack manager journey to assert browser
  manual entry for expression literal and table unit metadata.

## Validation

- `npm test --workspace apps/desktop -- ExpressionComposer` - PASS, 19 tests.
- `npm test --workspace apps/desktop` - PASS, 384 tests across 18 files.
- `npm run build --workspace apps/desktop` - PASS with the existing Vite
  chunk-size warning.
- `npm run test:e2e --workspace apps/desktop` - PASS, 10 Playwright tests
  across the configured desktop/compact Chromium projects.

## Boundary

Frontend-only. No schema, backend command, rule evaluator, expression grammar,
project persistence, rule-pack checksum, protected standards content, private
value embedding, browser fallback catalog, release-readiness, professional
approval, certification, sealing, authentication, or code-compliance claim was
created.

## Residuals

B2 still owns broader app unit entry/pickers outside material/section/
primitive-load create/edit and rule-pack declaration/expression forms, import
round-trip unit I/O, and target-format conversion witnesses beyond disclosure.
B3 still owns broader conversion-witness, incompatible-unit rejection, and
D-04/DEC-026 tolerance corpus coverage outside the product-physics,
create/edit operation, and rule-pack unit-picker boundaries.
