---
doc_id: WORKING-ITEMS-RUN-2026-06-11-LOAD-CASE-CREATION-EDITOR-DEL-05-01
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-05
deliverable_id: DEL-05-01
tranche_id: TP-APP-R2-LOADCREATE-001
---

# WORKING_ITEMS Run - Load Case Creation Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The desktop Load
Cases manager now exposes explicit empty load-case shell creation for the
invented preview model.

## Implementation Evidence

- `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx` adds a
  create-load-case editor for id, label, kind, status, and provenance.
- The editor queues a structured `create_load_case` intent with
  `field_path=load_cases`, `before=not_present`, unit `none`, dimension
  `dimensionless`, and a JSON payload whose `primitive_loads` collection is
  empty.
- The accepted shell is still only a load-case record. This tranche does not
  introduce primitive loads, imposed displacement loads, combination terms,
  code-specific defaults, or new load semantics in `core/loads`.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 25/25 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 39/39 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke at `http://127.0.0.1:5175/` applied
  `op:load-manager-create-load:L-300` and confirmed `3 load cases; 7
  primitive loads; 1 combinations`, the manager row
  `load:L-300; primitive_user_load; draft; primitives=0`, zero pending
  operations, and solve state `not_started`.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

This tranche does not add arbitrary primitive-load creation,
imposed-displacement authoring, combination basis editing, combination term
creation/deletion, broader algebra authoring, unit conversion, solver
semantics, code-specific defaults, protected standards content, private data,
release readiness, professional approval, certification, sealing,
authentication, or code-compliance claims.

## Residual

A4 still needs arbitrary primitive-load creation, imposed-displacement
authoring breadth, combination basis editing, combination term
creation/deletion, broader algebra authoring, Phase B unit picker/display
retirement, and packaged-Tauri saved-project smoke over edited load data.
