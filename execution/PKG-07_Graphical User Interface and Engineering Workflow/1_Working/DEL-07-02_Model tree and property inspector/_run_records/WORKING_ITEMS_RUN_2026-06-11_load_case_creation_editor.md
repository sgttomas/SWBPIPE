---
doc_id: WORKING-ITEMS-RUN-2026-06-11-LOAD-CASE-CREATION-EDITOR-DEL-07-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-07
deliverable_id: DEL-07-02
tranche_id: TP-APP-R2-LOADCREATE-001
---

# WORKING_ITEMS Run - Load Case Creation Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The Load Cases
manager now includes a visible create-load-case form and verifies that an
accepted empty shell appears in the manager list and property inspector.

## Implementation Evidence

- `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx` adds the
  create-load-case editor before the existing load-case list.
- The default draft id is the first unused `load:L-300`-style identifier; the
  preview declares `primitive_loads=0`, `direct_model_mutation_allowed=false`,
  and `professional_approval=false`.
- The app-level test queues/applies the create intent, verifies the manager
  summary becomes `3 load cases; 7 primitive loads; 1 combinations`, verifies
  the new row `load:L-300; primitive_user_load; draft; primitives=0`, and
  checks the property inspector for the created load case.
- The Playwright R2 smoke verifies the rendered create preview without
  queueing or applying, preserving the unchanged fixture model for solve and
  report smoke.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 25/25 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 39/39 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke at `http://127.0.0.1:5175/` applied the rendered
  create intent and confirmed the empty-shell row, zero pending operations,
  receipt boundary, and solve state `not_started`.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

The GUI creates only a visible empty load-case shell after explicit user
queue/apply. It does not infer primitives, mutate saved project data without
Save, perform unit conversion, add protected standards content, handle private
data, or make release, professional approval, certification, sealing,
authentication, or code-compliance claims.

## Residual

The Load Cases manager still needs arbitrary primitive-load creation,
imposed-displacement authoring breadth, combination basis editing, combination
term creation/deletion, broader algebra authoring, Phase B unit
picker/display retirement, and packaged-Tauri saved-project smoke over edited
load data.
