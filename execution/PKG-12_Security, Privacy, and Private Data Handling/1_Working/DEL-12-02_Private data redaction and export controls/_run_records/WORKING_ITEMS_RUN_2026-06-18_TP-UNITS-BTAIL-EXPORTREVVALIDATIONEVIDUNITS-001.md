# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-EXPORTREVVALIDATIONEVIDUNITS-001

Date: 2026-06-18

Persona: WORKING_ITEMS

Primary deliverable: DEL-12-02 - Private data redaction and export controls

Supporting deliverables:

- DEL-09-04 - Validation manual skeleton
- DEL-09-05 - Release quality gate checklist
- DEL-02-02 - Unit system and dimensional-analysis core contract

## Scope

Bounded Phase B-tail Export Safety Review unit-evidence matrix cleanup while
C5.7 remains human-execution gated. The Validation Evidence panel already
exports `unit_policy_evidence` for project unit context and the validation
manual unit/schema section; this tranche makes the export-review manifest
classify that export row as unit-evidence-required.

## Evidence

- `ExportReviewPanel` now includes
  `validation_release_evidence_review` in
  `UNIT_EVIDENCE_REQUIRED_EXPORT_IDS`.
- The validation-evidence export row records
  `unit_policy_ref=unit-policy-evidence:validation-release-evidence-review`,
  `unit_evidence_required=true`, `default_units_inferred=false`, and
  `conversion_performed=false`.
- Solved queued-intent Export Review now reports `covered=25/26`, because
  `agent_proposal_review` remains pending until a proposal exists.
- The proposal path reports 26/26 unit-evidence rows present.

## Validation

- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"` passed 1/1 selected test.
- Full App, full desktop Vitest, build, Playwright, and DEC-025 sweep evidence
  are recorded in closeout artifacts for this tranche.

## Boundary

Export-review inventory only. No validation manual section content, release
threshold, release authorization, release-readiness claim, professional
approval, runtime redaction rule, public transport commitment, target writer,
manifest-level unit conversion, protected standards content, private payload,
lifecycle transition, certification, sealing, authentication, or
code-compliance claim changed.
