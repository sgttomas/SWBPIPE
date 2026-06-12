# WORKING_ITEMS Run Record - TP-UNITS-B2-EXPORTDISCLOSURE-001

Date: 2026-06-12
Persona: WORKING_ITEMS
Deliverable: DEL-02-02 Unit system and dimensional-analysis core contract
Package: PKG-02 Domain Model, Units, and Core Schemas

## Scope

Bounded B2 app-integration tranche for export unit-aware I/O. The tranche
added DEC-018 unit-system disclosure to PCF, CAEPIPE MBF, and stress-neutral
export packages, including schema/core builder support and desktop JSON/UI
surface coverage.

## Changes

- Added shared desktop `buildExportUnitSystemDisclosure` helper carrying:
  `unit-system:dec-018-si-dual-display`, `entered_units_preserved`, model
  units, result units, target export units, conversion policy/scope, and
  protected/private-content false flags.
- Added `unit_system_disclosure` to PCF, CAEPIPE MBF, and stress-neutral
  desktop export packets and visible export-panel unit summaries.
- Updated strict export schemas and Python package builders to require and
  checksum `unit_system_disclosure.json`.
- Regenerated invented export package fixtures through deterministic builders.

## Validation

- `python3 -m pytest tests/test_caepipe_mbf_export_package.py tests/test_pcf_export_package.py tests/test_stress_neutral_export_package.py -q` - PASS, 32 tests.
- `python3 -m pytest tests -q` - PASS, 356 tests.
- `npm test --workspace apps/desktop -- --run src/App.test.tsx` - PASS, 45 tests.
- `npm test --workspace apps/desktop -- --run` - PASS, 216 tests.
- `npm run build --workspace apps/desktop` - PASS with existing Vite chunk-size warning.
- `npm run test:e2e --workspace apps/desktop -- --project=chromium-desktop apps/desktop/e2e/r2-smoke.spec.ts` - PASS, 2 tests.

## Boundary

No import round-trip claim, target compatibility claim, solver-deck validation
claim, rule-pack unit I/O, protected standards content, private project data,
release-readiness, professional approval, certification, sealing,
authentication, or code-compliance claim was created.

## Residuals

B2 still owns import round-trip unit I/O, target-format conversion witnesses
beyond disclosure, broader app unit entry/pickers outside the already covered
forms, and rule-pack unit I/O. B3 still owns broader conversion witness and
tolerance corpus coverage.
