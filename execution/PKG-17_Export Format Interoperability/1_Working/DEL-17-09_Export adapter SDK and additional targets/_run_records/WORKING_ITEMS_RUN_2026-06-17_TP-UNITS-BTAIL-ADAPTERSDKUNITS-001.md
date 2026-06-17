# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-ADAPTERSDKUNITS-001

**Date:** 2026-06-17  
**Persona:** WORKING_ITEMS  
**Package / Deliverable:** PKG-17 / DEL-17-09  
**Tranche:** `TP-UNITS-BTAIL-ADAPTERSDKUNITS-001`  
**Smoke record:** `apps/desktop/SMOKE.md` TP-MAC-196

## Scope

Add bounded Phase B-tail unit-policy evidence to the export adapter SDK
admission package and desktop preview while C5.7 remains human-execution gated.

## Files Changed

- `core/handoff/export_adapter_sdk/package.py`
- `schemas/export_adapter_sdk.schema.json`
- `fixtures/export_adapter_sdk/invented/export_adapter_sdk_package.json`
- `tests/test_export_adapter_sdk.py`
- `apps/desktop/src/features/export-adapter-sdk/ExportAdapterSdkPanel.tsx`
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/e2e/r2-smoke.spec.ts`
- `apps/desktop/SMOKE.md`
- `plans/PLAN_2026-06-17_prd_completion.md`
- `plans/PLAN_COMPLETION_LOG.md`
- DEL-17-09 and DEL-02-02 memory/run-record evidence

## Result

The adapter SDK package now emits `unit_policy_evidence` and a
`unit_policy_evidence.json` manifest member. The evidence records DEC-018 unit
system basis, `entered_units_preserved`, model/result/target unit disclosure,
`conversion_policy=no_adapter_sdk_conversion_performed`,
`conversion_performed=false`, target refs, and a witness policy for candidate
targets without claiming target-writer conversion.

The desktop preview displays and downloads the same evidence through the Export
Adapter SDK panel.

## Validation

- `python3 tests/test_export_adapter_sdk.py` - passed.
- `npm test --workspace apps/desktop -- App.test.tsx` - passed 55/55 tests.
- `npm test --workspace apps/desktop` - passed 18/18 files and 393/393 tests.
- `npm run build --workspace apps/desktop` - passed with existing Vite large-chunk warning.
- `npm run test:e2e --workspace apps/desktop -- -g "R2 desktop preview smoke"` - passed 2/2 Playwright tests.

## Boundary Review

Adapter-admission metadata only. No runtime loader, public endpoint,
target-specific writer, unit conversion API, target support claim,
compatibility claim, external execution, protected standards content, private
payload, lifecycle transition, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim changed.

## Residuals

Phase B-tail still carries broader app unit entry/pickers beyond the landed
surfaces and any remaining target-format conversion witnesses outside the
already-covered boundaries.
