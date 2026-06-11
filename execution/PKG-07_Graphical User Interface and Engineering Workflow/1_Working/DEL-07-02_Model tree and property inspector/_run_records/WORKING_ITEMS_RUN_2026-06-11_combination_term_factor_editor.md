---
doc_id: WORKING-ITEMS-RUN-2026-06-11-COMBINATION-TERM-FACTOR-EDITOR-DEL-07-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-07
deliverable_id: DEL-07-02
tranche_id: TP-APP-R2-COMBFACTOR-001
---

# WORKING_ITEMS Run - Combination Term Factor Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The model tree and
Load Cases manager now support existing combination term-factor edits through
the structured operation queue/apply UX.

## Implementation Evidence

- The Load Cases manager renders selectable existing combination term rows and
  a factor/rationale editor for the selected term.
- The app test applies `combination:C-OPER-ALT` term 1 from `0.5` to `0.75`,
  verifies the manager row and property inspector update, and verifies the
  local review context returns to zero pending operations.
- The Playwright smoke previews the rendered factor intent without queueing so
  the solve/results/report fixture path remains unchanged.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 24/24 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 37/37 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke at `http://127.0.0.1:5175/` confirmed rendered
  queue/apply behavior for the existing combination factor edit.
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
imposed-displacement authoring breadth, combination basis editing, combination
term creation/deletion, broader algebra authoring, and saved-project smoke
over edited load data.
