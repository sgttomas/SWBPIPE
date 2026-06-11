---
doc_id: WORKING-ITEMS-RUN-2026-06-11-COMBINATION-TERM-FACTOR-EDITOR-DEL-05-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-05
deliverable_id: DEL-05-02
tranche_id: TP-APP-R2-COMBFACTOR-001
---

# WORKING_ITEMS Run - Combination Term Factor Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The desktop Load Cases
manager now exposes existing combination term-factor editing for the invented
mechanics preview combination.

## Implementation Evidence

- `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx` adds
  selectable existing combination term rows and a factor editor.
- The editor queues structured `update_load` intents targeting
  `Combination.terms.N.factor` with explicit before/after values, unit `none`,
  dimension `dimensionless`, and local session audit/professional-boundary
  metadata.
- The tranche edits only an existing term factor. Whole `terms` replacement,
  `basis` editing, term creation/deletion, code/rule combinations, and broader
  algebra authoring remain deferred.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 24/24 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 37/37 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke at `http://127.0.0.1:5175/` applied
  `op:load-manager-combination:C-OPER-ALT-term-1-factor` and confirmed
  `load:L-200 x 0.75`, zero pending operations, session-only persistence,
  professional approval false, and solve state `not_started`.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

This tranche does not add public code-specific combination factors, protected
standards content, rule-pack evaluator behavior, stress recovery, allowables,
SIF/flexibility tables, lifecycle/status changes, dependency or
review-disposition edits, release claims, professional approval,
certification, sealing, authentication, code-compliance claims, protected
standards data, or private data.

## Residual

A4 still needs load-case creation, arbitrary primitive-load creation,
imposed-displacement authoring breadth, combination basis editing, combination
term creation/deletion, broader algebra authoring, Phase B unit picker/display
retirement, and packaged-Tauri saved-project smoke over edited load data.
