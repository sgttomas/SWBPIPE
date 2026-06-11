---
doc_id: WORKING-ITEMS-RUN-2026-06-11-PRESSURE-THERMAL-LOAD-CREATION-EDITOR-DEL-16-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-16
deliverable_id: DEL-16-02
tranche_id: TP-APP-R2-PRESSTEMP-001
---

# WORKING_ITEMS Run - Pressure/Thermal Load Creation Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The structured
operation validation and diff-preview seam now accepts explicit pressure and
thermal `create_primitive_load` payloads.

## Implementation Evidence

- Browser local validation and Rust `operation_applier` accept categories
  `pressure` and `thermal` for `change_kind=create_primitive_load`.
- Pressure expects dimension `pressure`, project unit metadata
  `project.units.pressure`, element pipe target, global direction, finite
  magnitude, and provenance.
- Thermal expects dimension `temperature_interval`, project unit metadata
  `project.units.temperature`, element pipe target, global direction, finite
  magnitude, and provenance.
- Unit conversion remains unavailable; intent units must match the project
  unit metadata exactly.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 29/29 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 48/48 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke at `http://127.0.0.1:5175/` confirmed rendered
  pressure and thermal create intents apply only after user action and leave
  persistence at `session_state_only_not_yet_saved`.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

This tranche does not resolve D-01 Phase B unit conversion implementation,
gauge/absolute pressure quantity-kind UI, reference-pressure handling, thermal
absolute-temperature conversion, direct model mutation, protected/private data,
release readiness, professional approval, certification, sealing,
authentication, or code-compliance claims.

## Residual

Operation validation still needs future A4 coverage for imposed-displacement
authoring, combination basis editing, combination term creation/deletion, and
broader algebra authoring, plus Phase B unit conversion and unit picker/display
retirement.
