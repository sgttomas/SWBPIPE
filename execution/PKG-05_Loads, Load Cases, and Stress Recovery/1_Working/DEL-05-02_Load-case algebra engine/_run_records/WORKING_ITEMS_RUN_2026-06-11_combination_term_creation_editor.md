---
doc_id: WORKING-ITEMS-RUN-2026-06-11-COMBINATION-TERM-CREATION-EDITOR-DEL-05-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-05
deliverable_id: DEL-05-02
tranche_id: TP-APP-R2-COMBTERMCREATE-001
---

# WORKING_ITEMS Run - Combination Term Creation Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The desktop Load Cases
manager now exposes explicit child-term creation for existing load
combinations in the invented mechanics preview model.

## Implementation Evidence

- `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx` adds a
  combination-term creation editor for existing `Combination` records.
- The editor queues structured `create_combination_term` intents with
  `field_path=terms`, `before=not_present`, unit `none`, dimension
  `dimensionless`, and JSON payload `{ load_case, factor }`.
- The browser local operation mirror and Rust
  `core/model_operations/operation_applier` crate validate and append one
  child term to an existing combination.
- Existing terms are preserved. Whole `Combination.terms` replacement,
  combination term deletion, code/rule combinations, and broader algebra
  authoring remain outside this tranche.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 32/32 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 54/54 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke at `http://127.0.0.1:5175/` applied `load:L-300`
  creation, selected it in the combination-term form, applied
  `op:load-manager-combination:C-OPER-ALT-term-2-create`, and confirmed
  `load:L-300 x 1`, zero pending operations, `applied_operations=2`, solve
  state `not_started`, and no browser console errors.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

This tranche does not change `core/loads/load_case_algebra` behavior, add
whole-term replacement, combination term deletion, public code-specific
combination factors, protected standards content, rule-pack evaluator
behavior, stress recovery, allowables, SIF/flexibility tables,
lifecycle/status changes, dependency or review-disposition edits, release
claims, professional approval, certification, sealing, authentication,
code-compliance claims, protected standards data, or private data.

## Residual

A4 still needs combination term deletion, broader algebra authoring, Phase B
unit picker/display retirement, and packaged-Tauri saved-project smoke over
edited load data.
