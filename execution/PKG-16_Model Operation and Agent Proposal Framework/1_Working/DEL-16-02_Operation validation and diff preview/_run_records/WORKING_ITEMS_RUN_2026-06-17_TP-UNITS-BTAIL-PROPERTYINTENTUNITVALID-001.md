# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-PROPERTYINTENTUNITVALID-001

Date: 2026-06-17
Persona: WORKING_ITEMS
Supporting deliverable: DEL-16-02 Operation validation and diff preview
Primary deliverable: DEL-07-02 Model tree and property inspector
Tranche: TP-UNITS-BTAIL-PROPERTYINTENTUNITVALID-001
Smoke evidence: TP-MAC-206

## Supporting Role

This record supports the Property Inspector tranche by documenting the
operation-intent evidence effect: unit-bearing editor modify and create
intents now carry a pre-application unit-dimension validation status in the
existing `validation.unit_validation` field.

## Evidence

- Modify intents for node coordinates and load primitive magnitude now display
  the unit-validation status before queue/apply.
- Create-section, create-material, and create-support intents display
  dimension-specific unit-validation statuses before queue/apply.
- The status is preview metadata. It does not bypass validate-only checks or
  accepted-model user application.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` passed 55/55 tests.
- `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` passed 14/14
  Playwright tests.
- `npm test --workspace apps/desktop` passed 18/18 files and 397/397 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  large-chunk warning.

## Boundary Review

No operation schema change, operation application, accepted model-state
mutation, durable acceptance persistence, unit conversion API, protected
standards content, private payload, lifecycle state transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.
