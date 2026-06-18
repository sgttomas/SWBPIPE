# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-SECTHREATUNITPOLICY-001

Date: 2026-06-18

Persona: WORKING_ITEMS

Supporting deliverable: DEL-08-05 - Report protected-content linter

Primary deliverable: DEL-12-05 - Security threat model

## Scope

Supporting report-lint record for the Security Threat Model unit-policy
inventory slice.

## Evidence

- `UNIT_POLICY_SURFACE_MARKERS` now includes
  `security-threat-model-unit-policy` for
  `apps/desktop/src/features/security-threat-model/SecurityThreatModelPanel.tsx`.
- The visible lint unit-policy row reports `unit_targets=43`,
  `conversion_witness_targets=2`, and `lint_conversion=false`.
- Baseline report lint remains at 46 scanned targets; this tranche adds no new
  scanned public surface, only a unit-policy marker for an already-scanned
  surface.

## Validation

See the primary DEL-12-05 run record with the same tranche id. The focused
App test file passed 56/56 tests; full desktop Vitest passed 18/18 files and
399/399 tests; desktop build passed with the existing Vite large-chunk
warning; focused R2 Playwright passed 2/2; and full single-worker Playwright
passed 18/18.

## Boundary

No report-linter protected-content semantics, legal clearance, redaction
controls, target writer compatibility, unit conversion, private payload,
protected content, lifecycle transition, release-readiness claim,
professional approval, certification, sealing, authentication, or
code-compliance claim changed.
