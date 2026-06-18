# WORKING_ITEMS RUN - TP-UNITS-BTAIL-ADAPTERFRAMEWORKUNITS-001

Date: 2026-06-17

Persona: WORKING_ITEMS

Primary deliverable: DEL-10-02 - Import-export adapter framework

Supporting deliverable: DEL-02-02 - Unit system and dimensional-analysis core contract

## Objective

Advance the Phase B-tail unit-aware I/O remainder while C5.7 remains
human-execution gated by adding explicit DEC-018 unit-policy evidence to the
format-neutral desktop adapter framework preview.

## Scope

- App slice: `apps/desktop/src/features/adapter-framework/AdapterFrameworkPanel.tsx`.
- Test slices: `apps/desktop/src/App.test.tsx` and
  `apps/desktop/e2e/r2-smoke.spec.ts`.
- Evidence surfaces: `apps/desktop/SMOKE.md`,
  `plans/PLAN_COMPLETION_LOG.md`,
  `plans/PLAN_2026-06-17_prd_completion.md`, this run record, and supporting
  deliverable memories/run records.

## Changes

- Added `unit_policy_evidence` to the Adapter Framework exported JSON packet.
- Added visible `adapter-framework-units` UI evidence showing the DEC-018
  disclosure summary, framework unit policy, and witness count.
- Reused the shared `buildExportUnitSystemDisclosure` helper so the packet
  records `unit-system:dec-018-si-dual-display`,
  `entered_units_preserved`, source/result unit disclosure, and
  `conversion_performed=false`.
- Extended App Vitest and R2 Playwright smoke assertions to cover the visible
  row and downloaded JSON fields.

## Validation

- `python3 tests/test_adapter_framework_contract.py` - passed.
- `npm test --workspace apps/desktop -- App.test.tsx` - passed 55/55.
- `npm run test:e2e --workspace apps/desktop -- -g "R2 desktop preview smoke"`
  - passed 2/2.
- `npm test --workspace apps/desktop` - passed 18/18 files and 397/397 tests.
- `npm run build --workspace apps/desktop` - passed with the existing Vite
  large-chunk warning.

## Boundary Review

- No concrete external import/export format was selected.
- No target-specific writer, runtime loader, public endpoint, conversion API,
  external execution, target support claim, target compatibility claim, release
  readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim was introduced.
- No protected standards content, proprietary data, private project payload,
  telemetry, network path, daemon, or repository-default private-data write was
  introduced.

## Residuals

- C5.7 human packaged pass remains human-gated; C5.8 depends on that pass.
- Phase B-tail still owns broader app unit entry/pickers beyond the landed
  surfaces and remaining target-format conversion witnesses outside already
  covered boundaries.
