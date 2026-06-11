---
doc_id: WORKING-ITEMS-RUN-2026-06-11-COMBINATION-CREATION-EDITOR-DEL-05-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-05
deliverable_id: DEL-05-02
tranche_id: TP-APP-R2-COMBCREATE-001
---

# WORKING_ITEMS Run - Combination Creation Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The desktop Load Cases
manager now exposes explicit creation for a new mechanics-basis load
combination with one existing load-case term in the invented preview model.

## Implementation Evidence

- `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx` adds a
  create-combination editor for new `Combination` records.
- The editor queues structured `create_combination` intents with
  `field_path=combinations`, `before=not_present`, unit `none`, dimension
  `dimensionless`, and a JSON payload containing `id`, `label`,
  `basis=mechanics`, `terms`, and `provenance`.
- The browser local operation mirror and Rust
  `core/model_operations/operation_applier` crate validate and append one new
  combination whose initial term references an existing load case.
- The create path blocks duplicate combination ids, missing referenced load
  cases, empty terms, non-finite factors, duplicate initial operands,
  non-mechanics basis values, invalid payloads, and invalid unit/dimension
  metadata.
- Subtraction/range expression authoring, code/rule combinations, and broader
  algebra schema changes remain outside this tranche.

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

This tranche does not change `core/loads/load_case_algebra` behavior, add
subtraction/range expression authoring, public code-specific combination
factors, protected standards content, rule-pack evaluator behavior, stress
recovery, allowables, SIF/flexibility tables, lifecycle/status changes,
dependency or review-disposition edits, release claims, professional approval,
certification, sealing, authentication, code-compliance claims, protected
standards data, or private data.

## Residual

A4 still needs subtraction/range expression authoring, Phase B unit
picker/display retirement, and packaged-Tauri saved-project smoke over edited
load data.
