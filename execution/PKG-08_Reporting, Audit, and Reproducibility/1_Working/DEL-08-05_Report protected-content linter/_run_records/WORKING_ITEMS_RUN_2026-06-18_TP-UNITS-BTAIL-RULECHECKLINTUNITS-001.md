# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-RULECHECKLINTUNITS-001

Date: 2026-06-18

Persona: WORKING_ITEMS

Primary deliverable: DEL-08-05 - Report protected-content linter

Supporting deliverables:

- DEL-06-03 - Required-input completeness checker
- DEL-02-02 - Unit system and dimensional-analysis core contract

## Scope

Bounded Phase B-tail Report Content Lint inventory slice while C5.7 remains
human-execution gated. The tranche records the Run Rule Checks public
unit-binding policy surface in the Report Content Lint explicit
public-surface inventory.

## Changes

- Added `apps/desktop/src/features/rule-check/RuleCheckRunPanel.tsx` to the
  report-lint public-surface roots and explicit target list.
- Added `rule-check-unit-binding-policy` to `unit_policy_evidence.target_refs`.
- Increased the visible and exported report-lint unit-policy target count from
  33 to 34 while leaving target-format conversion-witness count at two.

## Validation

Passed:

- `npm run test --workspace apps/desktop -- App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
  - 1/1 selected test passed.
- `npm run test --workspace apps/desktop -- RuleCheckRunPanel.test.tsx`
  - 18/18 tests passed.
- `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke|run-rule-checks panel loads the demo pack"`
  - 4/4 focused configured-project tests passed.
- `npm run test --workspace apps/desktop -- App.test.tsx -t "round trips local create, save, and open project controls without external file copies"`
  - 1/1 selected test passed.
- `npm run test --workspace apps/desktop`
  - 18/18 files and 399/399 tests passed.
- `npm run build --workspace apps/desktop`
  - Passed with the existing Vite large-chunk warning.
- `npm run test:e2e --workspace apps/desktop -- --workers=1`
  - 18/18 tests passed.

## Boundary

This tranche changes the Report Content Lint inventory over existing Run Rule
Checks unit-binding policy context only. It does not change rule-pack schema,
expression grammar, parser/text-syntax, backend completeness/evaluator
behavior, rule-pack persistence, solver behavior, report-linter
protected-content semantics, legal clearance, redaction controls, target
writer compatibility, unit-conversion API, DEC-018 catalog constant, schema
dimension enum, protected standards content, private data, lifecycle state,
release readiness, professional approval, certification, sealing,
authentication, or code-compliance posture.

## Handoff

C5.7 remains the next governing R3 item and requires the human packaged pass.
This B-tail slice is complete and can be consumed as supporting evidence for
DEL-08-05, DEL-06-03, and DEL-02-02. DEC-025 sweep evidence remains to be
recorded during git closeout.
