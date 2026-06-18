# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-EXPORTREVMISSINGDATAUNITS-001

Date: 2026-06-18

Persona: WORKING_ITEMS

Primary deliverable: DEL-12-02 - Private data redaction and export controls

Supporting deliverable: DEL-07-04 - Missing-data warning and blocking UX

## Scope

Bounded Phase B-tail Export Safety Review unit-evidence matrix cleanup while
C5.7 remains human-execution gated. The DEL-07-04 Missing Data Blocking panel
already exposes explicit unit-input policy evidence; this tranche makes the
export-review manifest classify that export row as unit-evidence-required.

## Evidence

- `ExportReviewPanel` now includes `missing_data_warning_blocking_review` in
  `UNIT_EVIDENCE_REQUIRED_EXPORT_IDS`.
- The missing-data export row records
  `unit_policy_ref=unit-input-policy-evidence:missing-data-warning-blocking-review`,
  `unit_evidence_required=true`, and `conversion_performed=false`.
- Solved queued-intent Export Review now reports `covered=20/21`, because
  `agent_proposal_review` remains pending until a proposal exists.
- The proposal path reports 21/21 unit-evidence rows present.

## Validation

- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"` passed 1/1 selected test.
- `npm test --workspace apps/desktop -- src/App.test.tsx` passed 56/56 tests.
- `npm test --workspace apps/desktop` passed 18/18 files and 399/399 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite large-chunk warning.
- `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"` passed 2/2 tests.
- `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18 tests.

## Boundary

Export-review inventory only. No missing-data blocking behavior, remediation
flow, accepted model mutation, rule-check execution, runtime redaction rule,
public transport commitment, target writer, manifest-level unit conversion,
protected standards content, private payload, lifecycle transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.
