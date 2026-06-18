# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-SECTHREATUNITPOLICY-001

Date: 2026-06-18

Persona: WORKING_ITEMS

Primary deliverable: DEL-12-05 - Security threat model

## Scope

Bounded Phase B-tail unit-policy slice for the public security threat-model
review surface while C5.7 remains human-execution gated.

## Implementation

- Added explicit `unit_policy_evidence` to the desktop Security Threat Model
  packet.
- The visible threat-model review now exposes
  `data-testid="security-threat-model-unit-policy"` with unit-check
  no-bypass status, export-workflow count, `conversion=false`, and
  `certification=false`.
- The unit evidence records the existing security no-bypass `unit_checks`
  control for unit-bearing export and handoff workflows without invoking a
  target writer or performing conversion.
- Export Safety Review now marks `security_threat_model_review` as
  unit-evidence-required and references
  `unit-policy-evidence:security-threat-model-no-bypass`.

## Validation

- `npm test --workspace apps/desktop -- src/App.test.tsx` passed 56/56 tests.
- `npm test --workspace apps/desktop` passed 18/18 files and 399/399 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  large-chunk warning.
- `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
  passed 2/2 after stale report-lint target-count assertions were updated.
- `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18.

## Boundary

Security threat-model evidence only. No security sufficiency claim, release
readiness, telemetry authorization, target writer behavior, manifest-level
conversion, DEC-018 catalog constant, schema dimension enum, protected
standards content, private payload, lifecycle transition, professional
approval, certification, sealing, authentication, or code-compliance claim
changed.
