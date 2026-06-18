# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-EXPORTREVREPORTLINTUNITS-001

Date: 2026-06-18

Persona: WORKING_ITEMS

Primary deliverable: DEL-12-02 - Private data redaction and export controls

Supporting deliverables:

- DEL-08-05 - Report protected-content linter
- DEL-02-02 - Unit system and dimensional-analysis core contract

## Scope

Bounded Phase B-tail Export Safety Review unit-evidence matrix cleanup while
C5.7 remains human-execution gated. The Report Content Lint panel already
exports `unit_policy_evidence` for the public-surface unit-policy inventory;
this tranche makes the export-review manifest classify that lint export row as
unit-evidence-required.

## Evidence

- `ExportReviewPanel` now includes `report_protected_content_lint` in
  `UNIT_EVIDENCE_REQUIRED_EXPORT_IDS`.
- The report-lint export row records
  `unit_policy_ref=unit-policy-evidence:report-lint-public-surfaces`,
  `unit_evidence_required=true`, `unit_policy_target_count=44`,
  `conversion_witness_target_count=2`, `default_units_inferred=false`, and
  `conversion_performed=false`.
- Solved queued-intent Export Review now reports `covered=26/27`, because
  `agent_proposal_review` remains pending until a proposal exists.
- The proposal path reports 27/27 unit-evidence rows present.

## Validation

- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"` passed 1/1 selected test.
- Full App, full desktop Vitest, build, Playwright, and DEC-025 sweep evidence
  are recorded in closeout artifacts for this tranche.

## Boundary

Export-review inventory only. No report-linter protected-content semantics,
legal clearance, redaction certification, release authorization, runtime
redaction rule, public transport commitment, target writer, manifest-level unit
conversion, protected standards content, private payload, lifecycle transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.
