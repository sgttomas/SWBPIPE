# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-RULEPACKUNITPOLICY-001

Date: 2026-06-17

Persona: WORKING_ITEMS

Primary deliverable: DEL-07-03 - Material, component, and rule-pack editors

Supporting deliverables:

- DEL-02-02 - Unit system and dimensional-analysis core contract
- DEL-06-01 - Rule-pack schema
- DEL-06-02 - Sandboxed unit-aware expression evaluator

Smoke row: TP-MAC-207

## Scope

Add visible unit-policy and unit-dimension validation evidence for rule-pack
declaration and expression authoring while C5 packaged-pass work remains
human-gated. The tranche covers existing rule-pack unit reference surfaces:

- declaration `required_inputs[].quantity_intent`;
- declaration `value_slots[].quantity_intent`;
- expression literal quantity unit refs;
- expression table argument and result unit refs.

## Implementation

- Added `unitDimensionValidationStatus` to
  `apps/desktop/src/services/unitCatalogService.ts`.
- Updated `DeclarationsEditor` to render
  `rule-pack-declarations-unit-policy`, including stored-unit policy, catalog
  route, conversion status, and per-required-input/value-slot validation
  statuses.
- Updated `ExpressionComposer` to render
  `rule-pack-expression-unit-policy`, including stored-unit policy, catalog
  route, conversion status, and per-expression-unit-ref validation statuses.
- Browser preview now explicitly records the catalog route as unavailable
  manual entry instead of implying a fallback catalog.
- Desktop/Tauri routes continue to use DEC-018 unit selectors and can report
  accepted catalog matches, mismatches, review statuses, and loading/catalog
  unavailability without rewriting stored unit refs.

## Validation

- `npm test --workspace apps/desktop -- unitCatalogService.test.ts DeclarationsEditor.test.tsx ExpressionComposer.test.tsx RulePackManagerPanel.test.tsx` passed with 67/67 tests.
- `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` passed with 14/14 tests.
- `npm test --workspace apps/desktop` passed with 398/398 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite large-chunk warning.

## Boundary

No DEC-018 catalog constant, schema dimension enum, `rule_pack.schema`
contract, evaluator normalization, parser/text syntax, backend validation or
persistence behavior, protected standards content, private payload, lifecycle
transition, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim changed.
