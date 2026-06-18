---
agent: WORKING_ITEMS
tranche_id: TP-UNITS-BTAIL-RULECOMPLETENESSLINTUNITS-001
smoke_id: TP-MAC-258
date: 2026-06-18
primary_deliverable: DEL-08-05
status: PASS
---

# WORKING_ITEMS Run - Rule Completeness Report-Lint Unit Inventory

## Scope

Bounded Phase B-tail report-lint inventory slice while C5.7 remains
human-execution gated. The Rule-Check Completeness panel is an exported public
review surface for missing rule-check data; this run adds explicit unit-policy
evidence to that surface and records it in the Report Content Lint inventory.

## Changes

- Added visible/exported `unit_policy_evidence` to
  `apps/desktop/src/features/rule-check/RuleCheckPanel.tsx`, including the
  DEC-018 unit-system reference, sorted model units, unit-bearing record count,
  explicit rule-input-unit blocking policy, `RULE_UNIT_MISMATCH`, and
  `conversion_performed=false`.
- Added `apps/desktop/src/features/rule-check/RuleCheckPanel.tsx` as
  `target:desktop-rule-completeness-template` in `ReportLintPanel`.
- Added `rule-completeness-unit-policy` to
  `unit_policy_evidence.target_refs`.
- Updated App/Vitest and Playwright expectations from 40 to 41 public
  unit-policy targets. Static report-lint targets are now 45; solved report
  packets include 46 targets after the generated preview-report JSON target is
  appended.

## Evidence

- `npm run test --workspace apps/desktop -- --run src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
  passed 1/1 selected test.
- `npm run test --workspace apps/desktop -- --run src/App.test.tsx -t "shows computed mechanics diagnostics in results, knowledge, and review-only proposal context"`
  passed 1/1 selected test.
- `npm run test --workspace apps/desktop` passed 18/18 files and 399/399
  tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  large-chunk warning.
- `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
  passed 2/2 configured-project tests.
- `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18
  tests.

## Boundary

Evidence and inventory-only change. No rule evaluator semantics, required-input
completeness logic, analysis status semantics, mechanics solve behavior,
operation application, report-linter protected-content semantics, legal
clearance, redaction controls, target writer compatibility, unit-conversion
API, DEC-018 catalog constant, schema dimension enum, protected standards
content, private data, lifecycle transition, release-readiness claim,
professional approval, certification, sealing, authentication, or
code-compliance claim changed.
