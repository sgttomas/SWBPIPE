---
doc_id: WORKING-ITEMS-RUN-2026-06-11-COMBINATION-BASIS-EDITOR-DEL-07-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-07
deliverable_id: DEL-07-02
tranche_id: TP-APP-R2-COMBBASIS-001
---

# WORKING_ITEMS Run - Combination Basis Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The right-rail Load
Cases manager now includes a selected-combination basis editor.

## Implementation Evidence

- Combination rows can be selected independently of term rows.
- The basis editor displays the selected combination id, current basis, path
  `basis`, replacement basis input, rationale input, queue button, and preview.
- Applied combination rows and the property inspector update to the edited
  basis after user apply.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 31/31 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 52/52 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests and checks the
  basis edit preview.
- In-app browser smoke at `http://127.0.0.1:5175/` applied the basis edit via
  the manager and confirmed post-apply row state, zero pending operations, and
  solve reset.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

This tranche does not add combination term creation/deletion, broader algebra
authoring, code/rule combinations, unit conversion UI, saved-project mutation
without user Save, protected standards content, private data, release
readiness, professional approval, certification, sealing, authentication, or
code-compliance claims.

## Residual

The Load Cases manager still needs combination term creation/deletion, broader
algebra authoring, Phase B unit picker/display retirement, and packaged-Tauri
saved-project smoke over edited load data.
