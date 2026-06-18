# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-EXPORTREVA11YUNITS-001

Date: 2026-06-18

Persona: WORKING_ITEMS

Primary deliverable: DEL-12-02 - Private data redaction and export controls

Supporting deliverable: DEL-07-06 - Accessibility and usability baseline

## Scope

Bounded Phase B-tail Export Safety Review unit-evidence matrix cleanup while
C5.7 remains human-execution gated. The DEL-07-06 Accessibility Baseline
panel already exposes unit-visibility evidence for unit-bearing review
surfaces; this tranche makes the export-review manifest classify that export
row as unit-evidence-required.

## Evidence

- `ExportReviewPanel` now includes
  `accessibility_usability_baseline_review` in
  `UNIT_EVIDENCE_REQUIRED_EXPORT_IDS`.
- The accessibility export row records
  `unit_policy_ref=unit-visibility-evidence:accessibility-baseline-preview`,
  `unit_evidence_required=true`, `default_units_inferred=false`, and
  `conversion_performed=false`.
- Solved queued-intent Export Review now reports `covered=23/24`, because
  `agent_proposal_review` remains pending until a proposal exists.
- The proposal path reports 24/24 unit-evidence rows present.

## Validation

- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"` passed 1/1 selected test.
- Full App, full desktop Vitest, build, Playwright, and DEC-025 sweep evidence
  are recorded in closeout artifacts for this tranche.

## Boundary

Export-review inventory only. No accessibility finding count, conformance
target selection, runtime accessibility evaluation, color signaling policy,
runtime redaction rule, public transport commitment, target writer,
manifest-level unit conversion, protected standards content, private payload,
lifecycle transition, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim changed.
