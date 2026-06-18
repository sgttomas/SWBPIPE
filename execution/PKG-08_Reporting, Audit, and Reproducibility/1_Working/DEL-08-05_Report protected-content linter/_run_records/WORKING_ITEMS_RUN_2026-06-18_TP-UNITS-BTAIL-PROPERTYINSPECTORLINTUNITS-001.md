---
agent: WORKING_ITEMS
tranche_id: TP-UNITS-BTAIL-PROPERTYINSPECTORLINTUNITS-001
smoke_id: TP-MAC-257
date: 2026-06-18
primary_deliverable: DEL-08-05
status: PASS
---

# WORKING_ITEMS Run - Property Inspector Report-Lint Unit Inventory

## Scope

Bounded Phase B-tail report-lint inventory slice while C5.7 remains
human-execution gated. The Property Inspector already exposes unit-validation
metadata for edit, create, and delete operation intents; this run records that
existing public unit-policy surface in the Report Content Lint inventory.

## Changes

- Added `apps/desktop/src/features/model-tree/PropertyInspector.tsx` as
  `target:desktop-property-inspector-template` in `ReportLintPanel`.
- Added `property-inspector-unit-validation-surface` to
  `unit_policy_evidence.target_refs`.
- Updated App/Vitest and Playwright expectations from 39 to 40 public
  unit-policy targets. Static report-lint targets are now 44; solved report
  packets include 45 targets after the generated preview-report JSON target is
  appended.

## Evidence

- `npm run test --workspace apps/desktop -- App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
  passed 1/1 selected test.
- `npm run test --workspace apps/desktop -- App.test.tsx -t "queues and applies explicit (material|section|support)"`
  passed 4/4 selected tests.
- `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
  passed 2/2 configured-project tests.
- `npm run test --workspace apps/desktop` passed 18/18 files and 399/399
  tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  large-chunk warning.
- `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18
  tests.

## Boundary

Inventory-only change. No Property Inspector behavior, operation validation,
operation application, diff preview, accepted model mutation, report-linter
protected-content semantics, legal clearance, redaction controls, target writer
compatibility, unit-conversion API, DEC-018 catalog constant, schema dimension
enum, protected standards content, private data, lifecycle transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.
