# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-RULEPACKUNITPOLICY-001

Date: 2026-06-17

Persona: WORKING_ITEMS

Deliverable: DEL-06-01 - Rule-pack schema

Role: supporting evidence for DEL-07-03 rule-pack editor unit-policy tranche.

Smoke row: TP-MAC-207

## Scope

Record supporting schema evidence for visible unit-policy reporting over
existing rule-pack schema members. The tranche covers existing declaration
`quantity_intent` fields and expression AST unit refs only.

## Implementation Evidence

- The editor reports stored-unit policy, catalog route, conversion status, and
  unit-dimension validation status for existing rule-pack unit-reference
  fields.
- The report is UI evidence only. It does not add schema members, change
  `rule_pack.schema.yaml`, change schema version, change checksum semantics,
  or add writable expression text syntax.
- Stored unit refs remain the authored data; the UI reports validation status
  without rewriting those refs.

## Validation

- `npm test --workspace apps/desktop -- unitCatalogService.test.ts DeclarationsEditor.test.tsx ExpressionComposer.test.tsx RulePackManagerPanel.test.tsx` passed with 67/67 tests.
- `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` passed with 14/14 tests.
- `npm test --workspace apps/desktop` passed with 398/398 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite large-chunk warning.

## Boundary

No `rule_pack.schema.yaml` contract, schema version, schema enum, evaluator
behavior, parser, writable expression text syntax, protected content, private
data, release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.
