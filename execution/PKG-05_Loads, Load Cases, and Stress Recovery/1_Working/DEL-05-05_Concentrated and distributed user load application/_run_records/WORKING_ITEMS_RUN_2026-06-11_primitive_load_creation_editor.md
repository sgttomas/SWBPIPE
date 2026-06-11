---
doc_id: WORKING-ITEMS-RUN-2026-06-11-PRIMITIVE-LOAD-CREATION-EDITOR-DEL-05-05
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-05
deliverable_id: DEL-05-05
tranche_id: TP-APP-R2-PRIMCREATE-001
---

# WORKING_ITEMS Run - Primitive Load Creation Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The GUI can now
author one concentrated nodal-force primitive load using explicit user-entered
fields and existing model references.

## Implementation Evidence

- The Load Cases manager emits only category `concentrated_force`,
  target type `node`, direction `global_x`, `global_y`, or `global_z`,
  dimension `force`, and the project force unit for this sub-slice.
- The operation-applier validates the load case, primitive id uniqueness,
  node target reference, magnitude unit, dimension, finite magnitude, and
  provenance before application.
- This tranche does not change `core/loads/user_loads`; distributed loads,
  concentrated moments, element-station loads, and final result-envelope/API
  integration remain separate work.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 26/26 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 41/41 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke applied the default concentrated force
  `load:L-100-F300` at `250 N` under `load:L-100`.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

This tranche is GUI and operation-seam integration for an existing
concentrated-force concept. It does not add distributed-load authoring,
concentrated moments, imposed displacements, other element families, other
distribution shapes, final result-envelope/API integration, unit conversion,
protected standards data, private data, release readiness, professional
approval, certification, sealing, authentication, or code-compliance claims.

## Residual

DEL-05-05 still has explicit residuals for distributed-load GUI integration,
concentrated moments, element-station authoring, final result-envelope/API/
persistence/report integration, production tolerance policy, release
thresholds, and professional reliance.
