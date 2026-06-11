---
doc_id: WORKING-ITEMS-RUN-2026-06-11-MOMENT-LOAD-CREATION-EDITOR-DEL-16-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-16
deliverable_id: DEL-16-02
tranche_id: TP-APP-R2-MOMENTCREATE-001
---

# WORKING_ITEMS Run - Moment Load Creation Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The structured
operation validation and diff-preview path now accepts explicit concentrated
nodal-moment primitive creation.

## Implementation Evidence

- `apps/desktop/src/services/operationService.ts` validates
  `create_primitive_load` payload category `concentrated_moment`, project
  force and length unit metadata, unit `N*m`, dimension `moment`, existing
  node target, rotational direction, finite magnitude, and provenance.
- `core/model_operations/operation_applier/src/lib.rs` mirrors the same
  validation and diff-preview semantics for the Tauri backend route.
- Validation blocks duplicate primitive ids, missing node targets,
  unsupported primitive categories, incompatible directions, unit mismatches,
  and dimension mismatches.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 28/28 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 45/45 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke applied the concentrated-moment primitive-load
  operation through the diff/apply panel and confirmed the structured receipt.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

The operation path does not perform unit conversion, direct input-model
mutation, hidden target/default inference, durable persistence, solver
validation, protected/private data handling, or release, professional,
certification, sealing, authentication, or code-compliance claims.

## Residual

Future operation-surface work remains for pressure/temperature primitives,
imposed displacements, combination basis editing, combination term
creation/deletion, broader algebra authoring, and Phase B unit-display
retirement.
