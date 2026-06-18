# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-PCFCONVWITNESSVISIBLE-001

Date: 2026-06-17

Persona: WORKING_ITEMS

Primary deliverable: DEL-17-07 - Conservative PCF subset exporter

Supporting deliverable: DEL-02-02 - Unit system and dimensional-analysis core contract

Smoke row: TP-MAC-208

## Scope

Add visible desktop evidence for the existing PCF conversion-witness sidecar
while C5.7 remains human-execution gated. The tranche covers the Conservative
PCF Export panel only.

## Implementation

- Added `pcf-export-conversion-witnesses` to
  `apps/desktop/src/features/pcf-export/PcfExportPanel.tsx`.
- The line reports the existing conversion witness count, the converted
  node/pipe length field scope, and the target PCF length unit.
- Extended `apps/desktop/src/App.test.tsx` and
  `apps/desktop/e2e/r2-smoke.spec.ts` to assert the visible witness count and
  `target_length=MM`.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` passed with 55/55 tests.
- `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` passed with 14/14 tests.
- `npm test --workspace apps/desktop` passed with 398/398 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite large-chunk warning.

## Boundary

No PCF package schema, conversion formula, target compatibility claim,
downstream import evidence, solver-validation claim, unit conversion API,
protected standards content, private payload, lifecycle transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.
