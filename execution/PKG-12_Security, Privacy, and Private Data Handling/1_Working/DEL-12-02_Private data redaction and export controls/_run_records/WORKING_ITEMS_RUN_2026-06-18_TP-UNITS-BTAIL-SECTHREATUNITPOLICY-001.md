# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-SECTHREATUNITPOLICY-001

Date: 2026-06-18

Persona: WORKING_ITEMS

Supporting deliverable: DEL-12-02 - Private data redaction and export controls

Primary deliverable: DEL-12-05 - Security threat model

## Scope

Supporting export-review record for the Security Threat Model unit-policy
inventory slice.

## Evidence

- Export Safety Review now treats `security_threat_model_review` as
  unit-evidence-required.
- The security threat-model export row carries
  `unit_policy_ref=unit-policy-evidence:security-threat-model-no-bypass`,
  `unit_evidence_required=true`, and `conversion_performed=false`.
- Solved queued-intent export review now reports `covered=18/19` before an
  agent proposal exists. The proposal path reports 19/19 unit-evidence rows
  present once the proposal row is available.

## Validation

See the primary DEL-12-05 run record with the same tranche id. The focused
App test file passed 56/56 tests; full desktop Vitest passed 18/18 files and
399/399 tests; desktop build passed with the existing Vite large-chunk
warning; focused R2 Playwright passed 2/2; and full single-worker Playwright
passed 18/18.

## Boundary

No runtime redaction rule, public transport commitment, target-specific
writer, manifest-level unit conversion, security certification claim,
telemetry authorization, private payload, protected content, lifecycle
transition, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim changed.
