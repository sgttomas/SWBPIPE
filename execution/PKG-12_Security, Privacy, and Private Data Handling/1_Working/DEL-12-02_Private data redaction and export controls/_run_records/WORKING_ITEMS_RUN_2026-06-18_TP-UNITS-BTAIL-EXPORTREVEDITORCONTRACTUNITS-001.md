# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-EXPORTREVEDITORCONTRACTUNITS-001

Date: 2026-06-18

Persona: WORKING_ITEMS

Primary deliverable: DEL-12-02 - Private data redaction and export controls

Supporting deliverable: DEL-07-03 - Material, component, and rule-pack editors

## Scope

Bounded Phase B-tail Export Safety Review unit-evidence matrix cleanup while
C5.7 remains human-execution gated. The DEL-07-03 Editor Contract panel
already exposes the DEL-02-02 unit contract for unit-bearing editor values;
this tranche makes the export-review manifest classify that export row as
unit-evidence-required.

## Evidence

- `ExportReviewPanel` now includes `editor_contract_review` in
  `UNIT_EVIDENCE_REQUIRED_EXPORT_IDS`.
- The editor-contract export row records
  `unit_policy_ref=DEL-02-02:unit_bearing_values_require_explicit_unit_metadata`,
  `unit_evidence_required=true`, `missing_unit_behavior=diagnostic_blocking`,
  and `conversion_performed=false`.
- Solved queued-intent Export Review now reports `covered=21/22`, because
  `agent_proposal_review` remains pending until a proposal exists.
- The proposal path reports 22/22 unit-evidence rows present.

## Validation

- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"` passed 1/1 selected test.
- Full App, full desktop Vitest, build, Playwright, and DEC-025 sweep evidence
  are recorded in closeout artifacts for this tranche.

## Boundary

Export-review inventory only. No editor validation behavior, durable mutation,
private rule-pack payload handling, private-library payload handling, runtime
redaction rule, public transport commitment, target writer, manifest-level
unit conversion, protected standards content, private payload, lifecycle
transition, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim changed.
