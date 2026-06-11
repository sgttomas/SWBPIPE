---
doc_id: WORKING-ITEMS-RUN-2026-06-11-IMPOSED-DISPLACEMENT-LOAD-CREATION-EDITOR-DEL-05-01
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-05
deliverable_id: DEL-05-01
tranche_id: TP-APP-R2-IMPOSED-001
---

# WORKING_ITEMS Run - Imposed-Displacement Load Creation Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The desktop Load
Cases manager now exposes explicit imposed-displacement primitive-load
creation for existing load cases in the invented preview model.

## Implementation Evidence

- `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx` extends the
  create-primitive editor with category `imposed_displacement`, existing
  support targets, support DOF options `UX|UY|UZ|RX|RY|RZ`, project length
  unit `m` for translational DOFs, project angle unit `rad` for rotational
  DOFs, and dimensions `displacement` or `rotation`.
- `fixtures/product_preview/invented_preview_model.json` now records
  `project.units.angle = "rad"` so rotational imposed-displacement authoring
  consumes explicit project unit metadata.
- The editor queues structured `create_primitive_load` intents with
  `field_path=primitive_loads`, `before=not_present`, category
  `imposed_displacement`, and target
  `{ type: "support", support: <existing support id>, dof: <matching DOF> }`.
- The browser local operation mirror and Rust
  `core/model_operations/operation_applier` crate validate, diff, and apply
  imposed-displacement primitive paths while preserving concentrated-force,
  distributed-force, concentrated-moment, pressure, and thermal paths.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 30/30 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 50/50 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke at `http://127.0.0.1:5175/` applied
  `op:load-manager-load:L-100-load:L-100-I300-primitive` in a clean session.
  It confirmed `2 load cases; 8 primitive loads; 1 combinations`, manager row
  `load:L-100-I300; support:support:S-100; UZ; dimension=displacement`, zero
  pending operations, and solve state `not_started`.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

This tranche does not add support coordinate policy, stiffness or default
restraint behavior, solver boundary behavior, combination basis editing,
combination term creation/deletion, broader algebra authoring, unit
conversion, solver validation, code-specific defaults, protected standards
content, private data, release readiness, professional approval,
certification, sealing, authentication, or code-compliance claims.

## Residual

A4 still needs combination basis editing, combination term creation/deletion,
broader algebra authoring, Phase B unit picker/display retirement, and
packaged-Tauri saved-project smoke over edited load data.
