---
doc_id: WORKING-ITEMS-RUN-2026-06-11-COMBINATION-BASIS-EDITOR-DEL-05-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-05
deliverable_id: DEL-05-02
tranche_id: TP-APP-R2-COMBBASIS-001
---

# WORKING_ITEMS Run - Combination Basis Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The desktop Load Cases
manager now exposes explicit basis editing for existing load combinations in
the invented mechanics preview model.

## Implementation Evidence

- `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx` adds a
  selected-combination basis editor.
- The editor queues structured `update_load` intents targeting
  `Combination.basis` with explicit before/after values, unit `none`,
  dimension `dimensionless`, and local session audit/professional-boundary
  metadata.
- The browser local operation mirror and Rust
  `core/model_operations/operation_applier` crate now treat
  `Combination.basis` as an editable text field.
- Whole `Combination.terms` replacement remains explicitly deferred; term
  creation/deletion, code/rule combinations, and broader algebra authoring
  remain outside this tranche.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 31/31 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 52/52 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke at `http://127.0.0.1:5175/` applied
  `op:load-manager-combination:C-OPER-ALT-basis` and confirmed
  `basis=mechanics_user_review`, zero pending operations,
  `applied_operations=1`, and solve state `not_started`.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

This tranche does not add whole-term replacement, combination term
creation/deletion, public code-specific combination factors, protected
standards content, rule-pack evaluator behavior, stress recovery, allowables,
SIF/flexibility tables, lifecycle/status changes, dependency or
review-disposition edits, release claims, professional approval,
certification, sealing, authentication, code-compliance claims, protected
standards data, or private data.

## Residual

A4 still needs combination term creation/deletion, broader algebra authoring,
Phase B unit picker/display retirement, and packaged-Tauri saved-project smoke
over edited load data.
