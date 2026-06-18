# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-EXPORTREVEXTPROVERUNITS-001

Date: 2026-06-18

Persona: WORKING_ITEMS

Primary deliverable: DEL-12-02 - Private data redaction and export controls

Supporting deliverable: DEL-15-04 - External prover boundary metadata

## Scope

Bounded Phase B-tail Export Safety Review unit-evidence matrix cleanup while
C5.7 remains human-execution gated. The DEL-15-04 external-prover boundary
panel already exposes DEC-018 unit-policy evidence; this tranche makes the
export-review manifest classify that export row as unit-evidence-required.

## Evidence

- `ExportReviewPanel` now includes `external_prover_boundary_metadata` in
  `UNIT_EVIDENCE_REQUIRED_EXPORT_IDS`.
- Solved queued-intent Export Review now reports `covered=19/20`, because
  `agent_proposal_review` remains pending until a proposal exists.
- The proposal path reports 20/20 unit-evidence rows present.
- The downloaded export-review JSON includes
  `external_prover_boundary_metadata` in
  `unit_policy_summary.covered_export_ids`, and that matrix row reports
  `unit_evidence_status=covered_by_target_panel_or_export_packet`.

## Validation

- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"` passed 1/1 selected test.
- `npm test --workspace apps/desktop -- src/App.test.tsx` passed 56/56 tests after updating the stale visible coverage assertion.
- `npm test --workspace apps/desktop` passed 18/18 files and 399/399 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite large-chunk warning.
- `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"` passed 2/2 tests.
- `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18 tests.

## Boundary

Export-review inventory only. No external solver/prover invocation, target
parser, commercial-result ingestion, target writer, manifest-level unit
conversion, redaction behavior, public transport commitment, protected
standards content, private payload, lifecycle transition, release-readiness
claim, professional approval, certification, sealing, authentication, or
code-compliance claim changed.
