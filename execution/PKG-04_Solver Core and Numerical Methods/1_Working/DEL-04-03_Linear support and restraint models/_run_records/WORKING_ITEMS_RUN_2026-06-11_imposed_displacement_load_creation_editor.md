---
doc_id: WORKING-ITEMS-RUN-2026-06-11-IMPOSED-DISPLACEMENT-LOAD-CREATION-EDITOR-DEL-04-03
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-04
deliverable_id: DEL-04-03
tranche_id: TP-APP-R2-IMPOSED-001
---

# WORKING_ITEMS Run - Imposed-Displacement Load Creation Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The desktop Load
Cases manager now uses existing support records as explicit targets for
imposed-displacement primitive-load authoring.

## Implementation Evidence

- The create-primitive editor exposes an existing support selector only for
  category `imposed_displacement`.
- The support-target payload stores the existing support id and the selected
  support DOF as `{ type: "support", support: <support id>, dof: <DOF> }`.
- Translational DOFs `UX|UY|UZ` use project length unit metadata and
  dimension `displacement`; rotational DOFs `RX|RY|RZ` use project angle unit
  metadata and dimension `rotation`.
- Operation validation checks that the referenced support exists and that the
  payload target DOF matches the selected primitive-load direction.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 30/30 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 50/50 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke at `http://127.0.0.1:5175/` confirmed support
  `support:S-100` is selected for `load:L-100-I300`, the preview renders
  `target=support:S-100; direction=UZ; unit=m; displacement`, and the applied
  manager row records `support:support:S-100`.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

This tranche does not change `core/solver/linear_supports`, support stiffness
assembly, coordinate-system policy, restraint defaults, support guide behavior,
solver boundary application, sparse solver integration, result envelopes,
protected standards content, private data, release readiness, professional
approval, certification, sealing, authentication, or code-compliance claims.

## Residual

Support-target imposed-displacement authoring is now surfaced in the manager,
but support coordinate policy and solver-boundary behavior remain outside this
app-integration tranche.
