---
doc_id: WORKING-ITEMS-RUN-2026-06-11-PRIMITIVE-LOAD-CREATION-EDITOR-DEL-05-01
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-05
deliverable_id: DEL-05-01
tranche_id: TP-APP-R2-PRIMCREATE-001
---

# WORKING_ITEMS Run - Primitive Load Creation Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The desktop Load
Cases manager now exposes explicit concentrated nodal-force primitive-load
creation for existing load cases in the invented preview model.

## Implementation Evidence

- `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx` adds a
  create-primitive editor for load case, primitive id, existing node target,
  global direction, magnitude, and provenance.
- The editor queues a structured `create_primitive_load` intent with
  `field_path=primitive_loads`, `before=not_present`, the project force unit,
  dimension `force`, and a JSON payload for category `concentrated_force`.
- `core/loads/primitive_loads` source behavior did not change; this tranche
  consumes existing primitive-load model evidence in the GUI.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 26/26 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 41/41 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke at `http://127.0.0.1:5175/` applied
  `op:load-manager-load:L-100-load:L-100-F300-primitive` and confirmed `2
  load cases; 8 primitive loads; 1 combinations`, the manager row
  `load:L-100-F300; node:node:N-100; global_y; dimension=force`, zero pending
  operations, and solve state `not_started`.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

This tranche does not add distributed primitive-load creation, concentrated
moments, pressure/temperature primitive creation, imposed-displacement
authoring, combination basis editing, combination term creation/deletion,
broader algebra authoring, unit conversion, solver validation, code-specific
defaults, protected standards content, private data, release readiness,
professional approval, certification, sealing, authentication, or
code-compliance claims.

## Residual

A4 still needs distributed primitive-load creation, concentrated moments,
pressure/temperature primitive creation, imposed-displacement authoring
breadth, combination basis editing, combination term creation/deletion,
broader algebra authoring, Phase B unit picker/display retirement, and
packaged-Tauri saved-project smoke over edited load data.
