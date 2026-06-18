# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-PCFCONVWITNESSVISIBLE-001

Date: 2026-06-17

Persona: WORKING_ITEMS

Deliverable: DEL-02-02 - Unit system and dimensional-analysis core contract

Role: supporting evidence for DEL-17-07 PCF conversion-witness visibility tranche.

Smoke row: TP-MAC-208

## Scope

Record supporting unit-system evidence for making existing PCF conversion
witnesses visible in the desktop panel and browser smoke path.

## Implementation Evidence

- The Conservative PCF Export panel now reports the existing
  `conversion_witnesses` sidecar count and target length unit.
- The PCF package's DEC-018 basis, source units, target millimeter fields, and
  conversion witnesses are unchanged.
- Browser smoke now verifies the visible `count=23` and `target_length=MM`
  line in addition to the existing package-level assertions.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` passed with 55/55 tests.
- `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` passed with 14/14 tests.
- `npm test --workspace apps/desktop` passed with 398/398 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite large-chunk warning.

## Boundary

No DEC-018 catalog constant, schema dimension enum, unit conversion API,
target compatibility claim, downstream import evidence, protected standards
content, private data, lifecycle transition, release-readiness claim,
professional approval, certification, sealing, authentication, or
code-compliance claim changed.
