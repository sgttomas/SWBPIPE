# WORKING_ITEMS RUN - TP-UNITS-BTAIL-HANDOFFUNITWITNESS-001

- **Date:** 2026-06-17
- **Persona:** WORKING_ITEMS
- **Supporting deliverable:** DEL-17-03 Native open JSON export package
- **Primary deliverable:** DEL-15-01 Canonical handoff package schema and manifest
- **Tranche:** TP-UNITS-BTAIL-HANDOFFUNITWITNESS-001
- **SMOKE:** TP-MAC-199

## Supporting Scope

Support DEL-15-01 by recording that the desktop native-open handoff preview
preserves source result units by reference rather than performing target-format
conversion or asserting target compatibility.

## Changes

- The handoff package target profile remains `native_open_json_preview` and
  `stable_ids_only_not_target_specific`.
- The new unit evidence records DEC-018 source units, result units, and
  no-conversion policy for handoff result references.
- The package still carries target field coverage as `TBD` and keeps the loss
  report as the target-specific boundary.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` passed 55/55 tests.
- `npm run test:e2e --workspace apps/desktop -- -g "R2 desktop preview smoke"`
  passed 2/2 Playwright tests.
- `npm test --workspace apps/desktop` passed 18/18 files and 397/397 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  large-chunk warning.

## Boundary

Supporting native-open handoff evidence only. No target writer, downstream
adapter, package schema promotion, target compatibility claim, protected
standards content, private payload, lifecycle transition, release-readiness
claim, professional approval, certification, sealing, authentication, or
code-compliance claim changed.
