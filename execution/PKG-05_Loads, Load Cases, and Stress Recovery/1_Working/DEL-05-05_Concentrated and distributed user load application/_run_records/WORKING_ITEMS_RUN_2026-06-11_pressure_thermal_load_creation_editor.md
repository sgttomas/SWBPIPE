---
doc_id: WORKING-ITEMS-RUN-2026-06-11-PRESSURE-THERMAL-LOAD-CREATION-EDITOR-DEL-05-05
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-05
deliverable_id: DEL-05-05
tranche_id: TP-APP-R2-PRESSTEMP-001
---

# WORKING_ITEMS Run - Pressure/Thermal Load Creation Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The desktop Load
Cases manager now consumes the explicit element-targeted pressure and thermal
primitive-load path through the same structured operation seam as the already
landed concentrated-force, distributed-force, and concentrated-moment
primitive paths.

## Implementation Evidence

- Pressure primitive creation accepts category `pressure`, existing pipe
  target, `global_x|global_y|global_z` direction, dimension `pressure`,
  project pressure unit `Pa`, finite magnitude, and provenance.
- Thermal primitive creation accepts category `thermal`, existing pipe target,
  `global_x|global_y|global_z` direction, dimension
  `temperature_interval`, project temperature interval unit `degC`, finite
  magnitude, and provenance.
- Missing pipe targets, duplicate primitive ids, unit mismatches, and
  dimension mismatches remain blocking validation findings.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 29/29 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 48/48 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke at `http://127.0.0.1:5175/` applied
  `load:L-100-P300` and `load:L-100-T300`, confirmed element pipe targets,
  dimensions `pressure` and `temperature_interval`, zero pending operations,
  `persistence=session_state_only_not_yet_saved`, and solve state
  `not_started`.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

No `core/loads/user_loads` source behavior changed. This tranche does not add
gauge/absolute pressure conversion, reference-pressure defaults, thermal
absolute-temperature conversion, imposed displacements, element-station
authoring, final result-envelope/API/persistence/report integration,
production tolerance policy, release thresholds, professional reliance,
protected standards content, or private project data.

## Residual

Remaining DEL-05-05 residuals include imposed displacements, final
result-envelope/API/persistence/report integration, production tolerance
policy, release thresholds, and professional reliance.
