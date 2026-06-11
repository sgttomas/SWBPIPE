---
doc_id: WORKING-ITEMS-RUN-2026-06-11-COMBINATION-TERM-DELETION-EDITOR-DEL-16-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-16
deliverable_id: DEL-16-02
tranche_id: TP-APP-R2-COMBTERMDELETE-001
---

# WORKING_ITEMS Run - Combination Term Deletion Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The browser local
operation mirror and Rust operation applier now validate and diff explicit
indexed combination-term deletion operations.

## Implementation Evidence

- `apps/desktop/src/services/operationService.ts` accepts
  `change_kind=delete_combination_term` only with `operation_kind=delete`,
  target object type `Combination`, `field_path=terms.N`, `after=not_present`,
  unit `none`, and dimension `dimensionless`.
- Diff preview resolves the exact existing term and records
  `{ combinationId, termIndex }`; apply removes that single array entry.
- Stale before-values are rejected with `OP-STALE-BEFORE-VALUE`.
  Out-of-range indices are rejected with `OP-COMBINATION-TERM-NOT-FOUND`.
  Invalid existing term payloads are rejected with
  `OP-COMBINATION-TERM-PAYLOAD-INVALID`.
- `core/model_operations/operation_applier/src/lib.rs` mirrors the same
  validation and apply behavior for the Rust path.
- Whole `Combination.terms` replacement remains blocked separately.

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
broader algebra authoring, unit conversion, lifecycle/status changes,
dependency or review-disposition edits, release claims, professional
approval, certification, sealing, authentication, code-compliance claims,
protected standards data, or private data.

## Residual

A4 still needs broader algebra authoring, Phase B unit picker/display
retirement, and packaged-Tauri saved-project smoke over edited load data.
