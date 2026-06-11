---
doc_id: WORKING-ITEMS-RUN-2026-06-11-COMBINATION-TERM-CREATION-EDITOR-DEL-07-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-07
deliverable_id: DEL-07-02
tranche_id: TP-APP-R2-COMBTERMCREATE-001
---

# WORKING_ITEMS Run - Combination Term Creation Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The right-rail Load
Cases manager now includes a create-term editor for existing combinations.

## Implementation Evidence

- The create-term editor displays the selected combination id, selected load
  case id, factor, rationale, queue button, and structured-operation preview.
- The editor defaults to an unused load case when one is available, but it
  still requires an explicit selected existing load case before queueing.
- Applied combination rows and the property inspector update to include the
  new term after user apply.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 32/32 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 54/54 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests and checks the
  rendered create-term preview.
- In-app browser smoke at `http://127.0.0.1:5175/` applied a newly created
  load case as a new combination term and confirmed post-apply row state,
  zero pending operations, and solve reset.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

This tranche does not add combination term deletion, broader algebra
authoring, code/rule combinations, unit conversion UI, saved-project mutation
without user Save, protected standards content, private data, release
readiness, professional approval, certification, sealing, authentication, or
code-compliance claims.

## Residual

The Load Cases manager still needs combination term deletion, broader algebra
authoring, Phase B unit picker/display retirement, and packaged-Tauri
saved-project smoke over edited load data.
