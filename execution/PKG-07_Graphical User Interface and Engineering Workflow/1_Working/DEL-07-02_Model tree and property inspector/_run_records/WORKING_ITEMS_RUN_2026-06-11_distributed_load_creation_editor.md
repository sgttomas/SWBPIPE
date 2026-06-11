---
doc_id: WORKING-ITEMS-RUN-2026-06-11-DISTRIBUTED-LOAD-CREATION-EDITOR-DEL-07-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-07
deliverable_id: DEL-07-02
tranche_id: TP-APP-R2-DISTLOAD-001
---

# WORKING_ITEMS Run - Distributed Load Creation Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The Load Cases
manager now includes a visible create-primitive category selector and verifies
that an accepted distributed element-force primitive appears in the manager
list.

## Implementation Evidence

- `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx` switches the
  create target control between node and pipe according to primitive category.
- The preview declares `before=not_present`, target pipe, direction, unit
  `N/m`, dimension `force_per_length`, `direct_model_mutation_allowed=false`,
  and `professional_approval=false`.
- The app-level test queues/applies `load:L-100-D300`, verifies the manager
  summary becomes `2 load cases; 8 primitive loads; 1 combinations`, verifies
  the `load:L-100` primitive count, and verifies the new distributed primitive
  row.
- The Playwright R2 smoke verifies the rendered distributed create preview
  without queueing or applying, preserving the unchanged fixture model for
  solve and report smoke.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 27/27 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 43/43 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke at `http://127.0.0.1:5175/` applied the rendered
  distributed primitive create intent and confirmed the row, zero pending
  operations, receipt boundary, and solve state `not_started`.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

The GUI creates only an explicit distributed element-force primitive after
user queue/apply. It does not infer targets, directions, units, concentrated
moments, pressure/temperature primitives, imposed displacements, saved-project
mutation without Save, protected standards content, private data, or release,
professional approval, certification, sealing, authentication, or
code-compliance claims.

## Residual

The Load Cases manager still needs concentrated moments, pressure/temperature
primitive creation, imposed-displacement authoring breadth, combination basis
editing, combination term creation/deletion, broader algebra authoring, Phase
B unit picker/display retirement, and packaged-Tauri saved-project smoke over
edited load data.
