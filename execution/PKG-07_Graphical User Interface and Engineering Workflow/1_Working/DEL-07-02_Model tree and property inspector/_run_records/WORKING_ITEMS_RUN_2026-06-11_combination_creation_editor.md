---
doc_id: WORKING-ITEMS-RUN-2026-06-11-COMBINATION-CREATION-EDITOR-DEL-07-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-07
deliverable_id: DEL-07-02
tranche_id: TP-APP-R2-COMBCREATE-001
---

# WORKING_ITEMS Run - Combination Creation Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The desktop Load Cases
manager now exposes explicit creation for a new mechanics-basis combination
through the model-tree/property-inspector workflow surface.

## Implementation Evidence

- `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx` renders a
  create-combination form with controls for id, label, basis, initial load
  case, factor, provenance, and rationale.
- The create preview shows the operation id, `before=not_present`, new
  combination id, initial term, unit `none`, dimension `dimensionless`,
  `direct_model_mutation_allowed=false`, and
  `professional_approval=false`.
- `apps/desktop/src/App.test.tsx` queues and applies
  `op:load-manager-create-combination:C-300` through `OperationApplyPanel`,
  verifies the manager summary changes to two combinations, verifies the new
  row with `basis=mechanics` and `load:L-100 x 1`, and confirms selecting the
  row updates the property inspector to `combination:C-300`.
- `apps/desktop/e2e/r2-smoke.spec.ts` verifies the rendered create preview
  and live factor preview update without applying the operation, preserving the
  unchanged fixture solve and report path.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 34/34 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 58/58 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke at `http://127.0.0.1:5175/` applied
  `op:load-manager-create-combination:C-300` and confirmed two combinations,
  `combination:C-300` with `basis=mechanics` and `load:L-100 x 1`, property
  inspector selection, zero pending operations, `applied_operations=1`, solve
  state `not_started`, and no browser console errors.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

This tranche does not add subtraction/range expression authoring, code/rule
combinations, public code-specific factors, unit conversion, saved-project
mutation without user Save, lifecycle/status changes, dependency or
review-disposition edits, release claims, professional approval,
certification, sealing, authentication, code-compliance claims, protected
standards data, or private data.

## Residual

A4 still needs subtraction/range expression authoring, Phase B unit
picker/display retirement, and packaged-Tauri saved-project smoke over edited
load data.
