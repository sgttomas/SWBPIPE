---
doc_id: WORKING-ITEMS-RUN-2026-06-11-COMBINATION-TERM-DELETION-EDITOR-DEL-07-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-07
deliverable_id: DEL-07-02
tranche_id: TP-APP-R2-COMBTERMDELETE-001
---

# WORKING_ITEMS Run - Combination Term Deletion Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The desktop Load Cases
manager now exposes explicit deletion for one selected existing combination
term through the model-tree/property-inspector workflow surface.

## Implementation Evidence

- `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx` renders a
  selected-term delete control after the user selects a
  `combination:<id>` term row.
- The delete preview shows the operation id, current before-value,
  `after=not_present`, unit `none`, dimension `dimensionless`,
  `direct_model_mutation_allowed=false`, and
  `professional_approval=false`.
- `apps/desktop/src/App.test.tsx` selects
  `combination:C-OPER-ALT` term 1, queues
  `op:load-manager-combination:C-OPER-ALT-term-1-delete`, applies it through
  `OperationApplyPanel`, and verifies the manager and property inspector no
  longer show `load:L-200 x 0.5` while `load:L-100 x 1` remains.
- `apps/desktop/e2e/r2-smoke.spec.ts` verifies the rendered delete preview
  without applying the operation, preserving the unchanged fixture solve and
  report path.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 33/33 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 56/56 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke at `http://127.0.0.1:5175/` applied
  `op:load-manager-combination:C-OPER-ALT-term-1-delete` and confirmed
  `load:L-200 x 0.5` was no longer visible in the combination row,
  `load:L-100 x 1` remained, zero pending operations,
  `applied_operations=1`, solve state `not_started`, and no browser console
  errors.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

This tranche does not add whole-term replacement, code/rule combinations,
broader algebra authoring, unit conversion, saved-project mutation without
user Save, lifecycle/status changes, dependency or review-disposition edits,
release claims, professional approval, certification, sealing,
authentication, code-compliance claims, protected standards data, or private
data.

## Residual

A4 still needs broader algebra authoring, Phase B unit picker/display
retirement, and packaged-Tauri saved-project smoke over edited load data.
