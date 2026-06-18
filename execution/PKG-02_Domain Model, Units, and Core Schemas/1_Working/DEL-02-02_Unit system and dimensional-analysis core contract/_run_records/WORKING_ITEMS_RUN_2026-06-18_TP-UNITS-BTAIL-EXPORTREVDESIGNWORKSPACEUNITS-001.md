# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-EXPORTREVDESIGNWORKSPACEUNITS-001

Date: 2026-06-18

Persona: WORKING_ITEMS

Primary deliverable: DEL-12-02 - Private data redaction and export controls

Supporting deliverable: DEL-02-02 - Unit system and dimensional-analysis core contract

## Scope

Supporting unit-policy record for a bounded Phase B-tail Export Safety Review
matrix cleanup. The design-workspace packet already records DEC-018
unit-system identity, entered-unit preservation, result/comparison unit
context, and `conversion_performed=false`; this tranche makes the
export-review manifest inventory that existing unit evidence.

## Evidence

- Export Safety Review now marks
  `design_authoring_comparison_workspace` as unit-evidence-required.
- The row records
  `unit_policy_ref=unit-policy-evidence:design-workspace-preview`,
  `default_units_inferred=false`, and `conversion_performed=false`.
- Solved queued-intent Export Review now reports `covered=24/25`; proposal
  path evidence reports 25/25 when the proposal row is available.

## Validation

- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"` passed 1/1 selected test.
- Full App, full desktop Vitest, build, Playwright, and DEC-025 sweep evidence
  are recorded in closeout artifacts for this tranche.

## Boundary

No DEC-018 catalog constant, schema dimension enum, unit-conversion API,
target writer behavior, design-workspace unit-policy semantics, comparison
tolerance behavior, private payload, protected content, lifecycle transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.
