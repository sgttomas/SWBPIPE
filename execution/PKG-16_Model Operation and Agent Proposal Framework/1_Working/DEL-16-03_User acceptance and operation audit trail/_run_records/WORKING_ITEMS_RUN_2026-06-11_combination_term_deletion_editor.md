---
doc_id: WORKING-ITEMS-RUN-2026-06-11-COMBINATION-TERM-DELETION-EDITOR-DEL-16-03
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-16
deliverable_id: DEL-16-03
tranche_id: TP-APP-R2-COMBTERMDELETE-001
---

# WORKING_ITEMS Run - Combination Term Deletion Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The desktop
operation-acceptance flow now carries explicit combination-term deletion from
manager preview through queue, apply, local receipt, and session audit state.

## Implementation Evidence

- The Load Cases manager queues `delete_combination_term` intents through the
  same pending-operation path as other editor operations.
- The operation preview records `direct_model_mutation_allowed=false`,
  `professional_approval=false`, and local-only persistence semantics.
- `apps/desktop/src/App.test.tsx` applies
  `op:load-manager-combination:C-OPER-ALT-term-1-delete` through
  `OperationApplyPanel`, verifies zero pending operations, confirms
  `applied_operations=1`, and checks solve state resets to `not_started`.
- In-app browser smoke repeated the queue/apply path and observed no browser
  console errors.

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

This tranche does not create lifecycle/status changes, dependency or
review-disposition edits, release claims, professional approval,
certification, sealing, authentication, code-compliance claims, protected
standards data, private data, telemetry, network behavior, or saved-project
mutation without user Save.

## Residual

A4 still needs broader algebra authoring, Phase B unit picker/display
retirement, and packaged-Tauri saved-project smoke over edited load data.
