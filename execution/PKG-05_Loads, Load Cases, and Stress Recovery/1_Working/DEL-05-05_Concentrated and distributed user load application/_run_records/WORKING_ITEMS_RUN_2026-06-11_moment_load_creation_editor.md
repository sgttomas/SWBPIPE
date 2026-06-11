---
doc_id: WORKING-ITEMS-RUN-2026-06-11-MOMENT-LOAD-CREATION-EDITOR-DEL-05-05
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-05
deliverable_id: DEL-05-05
tranche_id: TP-APP-R2-MOMENTCREATE-001
---

# WORKING_ITEMS Run - Moment Load Creation Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The desktop Load
Cases manager now consumes DEL-05-05 concentrated moment design authority by
authoring explicit `concentrated_moment` node primitives for existing load
cases.

## Implementation Evidence

- `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx` emits a
  concentrated moment payload with target `{ type: "node", node:
  "node:N-100" }`, rotational direction, finite magnitude, `N*m`, dimension
  `moment`, and provenance.
- `apps/desktop/src/services/operationService.ts` and
  `core/model_operations/operation_applier/src/lib.rs` reject missing node
  targets, duplicate primitive-load ids, translational directions for moment
  payloads, mismatched units, and mismatched dimensions.
- Existing concentrated-force and distributed-force creation remain accepted
  through the same `create_primitive_load` operation kind.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 28/28 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 45/45 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke applied the rendered concentrated-moment primitive-load
  create intent and confirmed `concentrated_moment; 250 N*m`,
  `node:node:N-100; rotation_z; dimension=moment`, zero pending operations,
  receipt persistence `session_state_only_not_yet_saved`, and no professional
  approval.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

This tranche does not add pressure/temperature primitive creation, imposed
displacements, final result-envelope/API/persistence/report integration,
production tolerance policy, release thresholds, professional reliance,
protected standards content, or private data.

## Residual

Remaining DEL-05-05 residuals include pressure/temperature primitive creation,
imposed displacements, final result-envelope/API/persistence/report
integration, production tolerance policy, release thresholds, and
professional reliance.
