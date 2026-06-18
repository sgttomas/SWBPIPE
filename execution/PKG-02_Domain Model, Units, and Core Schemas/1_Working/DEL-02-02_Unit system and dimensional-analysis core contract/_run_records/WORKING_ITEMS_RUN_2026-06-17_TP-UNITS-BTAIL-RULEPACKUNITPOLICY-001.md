# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-RULEPACKUNITPOLICY-001

Date: 2026-06-17

Persona: WORKING_ITEMS

Deliverable: DEL-02-02 - Unit system and dimensional-analysis core contract

Role: supporting evidence for DEL-07-03 rule-pack editor unit-policy tranche.

Smoke row: TP-MAC-207

## Scope

Record supporting unit-system evidence for visible rule-pack declaration and
expression unit-ref validation. The implementation uses the existing DEC-018
desktop catalog route when available and records browser preview as explicit
manual entry with no synthesized fallback catalog.

## Implementation Evidence

- `unitDimensionValidationStatus` centralizes reporting for catalog matches,
  mismatches, review statuses, missing unit/dimension metadata, browser
  catalog unavailability, and loading/catalog-unavailable states.
- Rule-pack declaration and expression authoring surfaces preserve stored unit
  refs and record `conversion=false`.
- Desktop/Tauri catalog routes can report DEC-018 dimension matches or
  mismatches; browser preview reports declared model metadata because catalog
  validation is desktop-only.

## Validation

- `npm test --workspace apps/desktop -- unitCatalogService.test.ts DeclarationsEditor.test.tsx ExpressionComposer.test.tsx RulePackManagerPanel.test.tsx` passed with 67/67 tests.
- `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` passed with 14/14 tests.
- `npm test --workspace apps/desktop` passed with 398/398 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite large-chunk warning.

## Boundary

No DEC-018 catalog constant, schema dimension enum, unit conversion API,
solver/evaluator behavior, protected standards content, private data,
lifecycle transition, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim changed.
