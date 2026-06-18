# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-EXPORTREVDESIGNWORKSPACEUNITS-001

Date: 2026-06-18

Persona: WORKING_ITEMS

Primary deliverable: DEL-12-02 - Private data redaction and export controls

Supporting deliverable: DEL-07-08 - Design-authoring state and comparison workspace

## Scope

Bounded Phase B-tail Export Safety Review unit-evidence matrix cleanup while
C5.7 remains human-execution gated. The DEL-07-08 Design Workspace panel
already exposes `unit_policy_evidence` for composed model, result,
analysis-run, and comparison context; this tranche makes the export-review
manifest classify that export row as unit-evidence-required.

## Evidence

- `ExportReviewPanel` now includes
  `design_authoring_comparison_workspace` in
  `UNIT_EVIDENCE_REQUIRED_EXPORT_IDS`.
- The design-workspace export row records
  `unit_policy_ref=unit-policy-evidence:design-workspace-preview`,
  `unit_evidence_required=true`, `default_units_inferred=false`, and
  `conversion_performed=false`.
- Solved queued-intent Export Review now reports `covered=24/25`, because
  `agent_proposal_review` remains pending until a proposal exists.
- The proposal path reports 25/25 unit-evidence rows present.

## Validation

- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"` passed 1/1 selected test.
- Full App, full desktop Vitest, build, Playwright, and DEC-025 sweep evidence
  are recorded in closeout artifacts for this tranche.

## Boundary

Export-review inventory only. No design-workspace composition behavior,
comparison delta math, tolerance profile, operation application, accepted
model-state mutation, runtime redaction rule, public transport commitment,
target writer, manifest-level unit conversion, protected standards content,
private payload, lifecycle transition, release-readiness claim, professional
approval, certification, sealing, authentication, or code-compliance claim
changed.
