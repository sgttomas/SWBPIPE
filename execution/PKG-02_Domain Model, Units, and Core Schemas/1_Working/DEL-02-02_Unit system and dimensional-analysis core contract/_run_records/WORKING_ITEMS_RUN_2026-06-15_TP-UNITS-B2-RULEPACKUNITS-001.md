# WORKING_ITEMS Run Record - TP-UNITS-B2-RULEPACKUNITS-001

Date: 2026-06-15
Persona: WORKING_ITEMS
Deliverable: DEL-02-02 Unit system and dimensional-analysis core contract
Package: PKG-02 Domain Model, Units, and Core Schemas

## Scope

Bounded B2 app-integration tranche for rule-pack declaration unit I/O. The
slice connected the C2 rule-pack declarations form-builder to the accepted
DEC-018 desktop unit catalog for declaration `quantity_intent.unit_ref` fields,
without changing schemas, backend commands, persistence, expression grammar, or
rule evaluation.

## Changes

- Updated `apps/desktop/src/features/rule-packs/DeclarationsEditor.tsx` so
  required-input and value-slot unit fields load the desktop `get_unit_catalog`
  route only when the Tauri runtime is present.
- In desktop mode, unit fields render dimension-filtered DEC-018 selectors:
  accepted compatible units are offered, incompatible catalog units are absent,
  and stored out-of-catalog units remain visible as the current value instead
  of being silently snapped.
- In browser preview mode, the editor keeps the previous manual text-entry path
  and does not synthesize a fallback unit catalog.
- Added focused mocked-Tauri component coverage and extended the existing
  Playwright rule-pack manager journey to assert the browser manual-entry path.

## Validation

- `npm test --workspace apps/desktop -- DeclarationsEditor.test.tsx` - PASS,
  29 tests.
- `npm test --workspace apps/desktop` - PASS, 381 tests across 18 files.
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

B2 still owns rule-pack expression literal/table unit I/O and broader
unit-entry/import/export conversion work outside the already-covered forms. B3
still owns broader conversion-witness, incompatible-unit rejection, and
D-04/DEC-026 tolerance corpus coverage outside the product-physics,
create/edit operation, and rule-pack declaration-picker boundaries.
