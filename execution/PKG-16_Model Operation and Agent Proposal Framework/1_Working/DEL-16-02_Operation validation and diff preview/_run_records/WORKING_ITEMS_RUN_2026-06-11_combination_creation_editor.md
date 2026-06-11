---
doc_id: WORKING-ITEMS-RUN-2026-06-11-COMBINATION-CREATION-EDITOR-DEL-16-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-16
deliverable_id: DEL-16-02
tranche_id: TP-APP-R2-COMBCREATE-001
---

# WORKING_ITEMS Run - Combination Creation Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The browser local
operation mirror and Rust operation applier now validate and diff explicit
new-combination creation operations.

## Implementation Evidence

- `apps/desktop/src/services/operationService.ts` accepts
  `change_kind=create_combination` only with `operation_kind=create`, target
  object type `Combination`, `field_path=combinations`,
  `before=not_present`, unit `none`, and dimension `dimensionless`.
- Diff preview resolves the requested new combination and records the payload
  to append without mutating the input model.
- Validation requires a new id, non-empty label, `basis=mechanics`, non-empty
  provenance, at least one term, existing load-case references, finite term
  factors, and no duplicate initial operands.
- Duplicate ids are rejected with `OP-TARGET-ALREADY-EXISTS`. Missing load
  cases are rejected with `OP-COMBINATION-TERM-LOAD-NOT-FOUND`. Invalid
  payloads are rejected with `OP-CREATE-COMBINATION-PAYLOAD-INVALID`.
- `core/model_operations/operation_applier/src/lib.rs` mirrors the same
  validation and apply behavior for the Rust path.

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
combinations, public code-specific factors, unit conversion, lifecycle/status
changes, dependency or review-disposition edits, release claims, professional
approval, certification, sealing, authentication, code-compliance claims,
protected standards data, or private data.

## Residual

A4 still needs subtraction/range expression authoring, Phase B unit
picker/display retirement, and packaged-Tauri saved-project smoke over edited
load data.
