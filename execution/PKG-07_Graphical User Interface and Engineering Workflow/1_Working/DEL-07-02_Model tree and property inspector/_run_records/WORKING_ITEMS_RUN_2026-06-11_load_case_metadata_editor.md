---
doc_id: WORKING-ITEMS-RUN-2026-06-11-LOAD-CASE-METADATA-EDITOR-DEL-07-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-07
deliverable_id: DEL-07-02
tranche_id: TP-APP-R2-LOADMETA-001
---

# WORKING_ITEMS Run - Load Case Metadata Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The model tree and
Load Cases manager now support selected load-case metadata edits through the
existing structured operation queue/apply UX.

## Implementation Evidence

- The Load Cases manager displays the selected load-case metadata field,
  current value, draft value, rationale, and queue button.
- The app test applies `load:L-100` status from `preview_only` to `TBD`,
  verifies the manager row and property inspector update, and verifies the
  local review context returns to zero pending operations.
- The same test verifies the `kind` editor previews
  `primitive_user_load -> TBD` without applying a second operation.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 23/23 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 36/36 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke at `http://127.0.0.1:5175/` confirmed rendered
  queue/apply behavior for the status edit.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

This tranche changes local GUI editor behavior only. It does not expand
durable persistence, change lifecycle state, add hidden defaults, perform unit
conversion, add protected standards content, handle private data, or make
release, professional approval, certification, sealing, authentication, or
code-compliance claims.

## Residual

A4 still needs load-case creation, arbitrary primitive-load creation,
imposed-displacement authoring breadth, full combination editing/algebra
authoring, and saved-project smoke over edited load data.
