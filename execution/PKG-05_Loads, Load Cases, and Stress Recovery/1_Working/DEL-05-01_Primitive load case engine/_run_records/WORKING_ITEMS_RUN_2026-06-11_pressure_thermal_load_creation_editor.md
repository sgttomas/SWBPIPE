---
doc_id: WORKING-ITEMS-RUN-2026-06-11-PRESSURE-THERMAL-LOAD-CREATION-EDITOR-DEL-05-01
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-05
deliverable_id: DEL-05-01
tranche_id: TP-APP-R2-PRESSTEMP-001
---

# WORKING_ITEMS Run - Pressure/Thermal Load Creation Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The desktop Load
Cases manager now exposes explicit pressure and thermal primitive-load
creation for existing load cases in the invented preview model.

## Implementation Evidence

- `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx` extends the
  create-primitive editor with categories `pressure` and `thermal`, pipe
  targets, global direction options, unit `Pa` for pressure, unit `degC` for
  thermal interval input, and dimensions `pressure` and
  `temperature_interval`.
- `fixtures/product_preview/invented_preview_model.json` now records
  `project.units.pressure = "Pa"` so pressure creation consumes explicit
  project unit metadata instead of an implicit pressure fallback.
- The editor queues structured `create_primitive_load` intents with
  `field_path=primitive_loads`, `before=not_present`, and JSON payloads for
  categories `pressure` and `thermal`.
- The browser local operation mirror and Rust
  `core/model_operations/operation_applier` crate validate, diff, and apply
  pressure and thermal primitive paths while preserving concentrated-force,
  distributed-force, and concentrated-moment paths.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 29/29 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 48/48 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke at `http://127.0.0.1:5175/` applied
  `op:load-manager-load:L-100-load:L-100-P300-primitive` and
  `op:load-manager-load:L-100-load:L-100-T300-primitive` in clean sessions.
  It confirmed `2 load cases; 8 primitive loads; 1 combinations`, manager
  rows `load:L-100-P300; element:pipe:P-100; global_x; dimension=pressure`
  and `load:L-100-T300; element:pipe:P-100; global_z;
  dimension=temperature_interval`, zero pending operations, and solve state
  `not_started`.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

This tranche does not add gauge/absolute pressure conversion,
reference-pressure defaults, thermal absolute-temperature conversion,
imposed-displacement authoring, combination basis editing, combination term
creation/deletion, broader algebra authoring, unit conversion, solver
validation, code-specific defaults, protected standards content, private
data, release readiness, professional approval, certification, sealing,
authentication, or code-compliance claims.

## Residual

A4 still needs imposed-displacement authoring breadth, combination basis
editing, combination term creation/deletion, broader algebra authoring, Phase
B unit picker/display retirement, and packaged-Tauri saved-project smoke over
edited load data.
