---
doc_id: WORKING-ITEMS-RUN-2026-06-11-COMBINATION-BASIS-EDITOR-DEL-16-03
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-16
deliverable_id: DEL-16-03
tranche_id: TP-APP-R2-COMBBASIS-001
---

# WORKING_ITEMS Run - Combination Basis Editor

## Scope

Bounded app-integration tranche for completion-plan A4. User acceptance for
combination basis editing remains explicit: queue first, apply second, session
state only until Save.

## Implementation Evidence

- The Load Cases manager creates an editor intent for
  `op:load-manager-combination:C-OPER-ALT-basis`.
- `OperationApplyPanel` applies the queued intent only after user action and
  records the same operation id in the applied-operation receipt.
- Applied edits clear stale solve results and preserve
  `session_state_only_not_yet_saved`.
- The operation receipt continues to report `professional_approval=false`.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 31/31 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 52/52 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke at `http://127.0.0.1:5175/` applied the
  combination-basis edit and confirmed zero pending operations,
  `applied_operations=1`, session-only persistence, no professional approval
  claim, and solve state `not_started`.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

This tranche does not add saved-project mutation without user Save, external
publication, release readiness, professional approval, certification, sealing,
authentication, protected/private data handling changes, code/rule
combination claims, or code-compliance claims.

## Residual

Future A4 user-acceptance coverage still needs combination term
creation/deletion, broader algebra authoring, Phase B unit picker/display
retirement, and packaged-Tauri saved-project smoke over edited load data.
