# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-AGENTPROPOSALUNITS-001

Date: 2026-06-18

Persona: WORKING_ITEMS

Primary deliverable: DEL-16-04 - Agent rationale and professional-boundary controls

Supporting deliverables:

- DEL-16-02 - Operation validation and diff preview
- DEL-16-03 - User acceptance and operation audit trail
- DEL-08-05 - Report protected-content linter
- DEL-12-02 - Private data redaction and export controls
- DEL-08-04 - Result export format
- DEL-02-02 - Unit system and dimensional-analysis core contract

## Scope

Bounded Phase B-tail Agent Proposal unit-policy inventory slice while C5.7
remains human-execution gated. The Agent Proposal panel is a public
review-only operation surface; this tranche makes its metadata-only
unit-validation status visible and adds it to the report-lint and export
review inventories.

## Changes

- Added a visible Agent Proposal unit-policy section that reports
  `proposal.validation.unit_validation`, the validation source, and
  `conversion=false`.
- Added `unit_validation=not_required_metadata_review_only` to the invented
  proposal fixture and to mechanics-derived review proposals.
- Added `AgentProposalPanel.tsx` to the Report Content Lint public-surface
  inventory as `agent-proposal-unit-policy`.
- Added `agent_proposal_review` to the Export Safety Review manifest as a
  metadata-only local export row. It is pending until a proposal exists and
  available when a proposal is generated.
- Added `agent_proposal_review` to Report packet persistence export readiness.
- Updated App and Playwright assertions for 29 export rows, 42 report-lint
  unit-policy targets, and proposal-driven unit-evidence coverage.

## Validation

Passed:

- `npm run test --workspace apps/desktop -- --run src/App.test.tsx -t "shows computed mechanics diagnostics in results, knowledge, and review-only proposal context"`
  - 1/1 selected test passed after updating proposal unit-policy and export
    inventory assertions.
- `npm run test --workspace apps/desktop -- --run src/App.test.tsx -t "carries queued editor intents into the report packet as review-only operation context"`
  - 1/1 selected test passed after updating pending proposal export coverage.
- `npm run test --workspace apps/desktop -- --run src/App.test.tsx`
  - 56/56 tests passed.
- `npm run test --workspace apps/desktop`
  - 18/18 files passed; 399/399 tests passed.
- `npm run build --workspace apps/desktop`
  - Passed with the existing Vite large-chunk warning.
- `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
  - Initially caught stale report-lint target assertions; after updating
    `apps/desktop/e2e/r2-smoke.spec.ts`, 2/2 configured-project tests passed.
- `npm run test:e2e --workspace apps/desktop -- --workers=1`
  - 18/18 tests passed.

## Boundary

This tranche changes proposal/export/report inventory evidence only. It does
not apply proposals, mutate accepted model state, change operation validation
or diff semantics, add target-format conversion, add unit conversion behavior,
change redaction rules, add target writers, include protected standards
content, include private payloads, change lifecycle state, claim release
readiness, or claim professional approval, certification, sealing,
authentication, or code compliance.

## Handoff

C5.7 remains the next governing R3 item and requires the human packaged pass.
This B-tail slice is complete and can be consumed as supporting evidence for
DEL-16-04, DEL-16-02, DEL-16-03, DEL-08-05, DEL-12-02, DEL-08-04, and
DEL-02-02. DEC-025 sweep evidence remains to be recorded during git closeout.
