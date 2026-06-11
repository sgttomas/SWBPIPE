---
doc_id: WORKING-ITEMS-RUN-2026-06-11-PRIMITIVE-LOAD-CREATION-EDITOR-DEL-16-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-16
deliverable_id: DEL-16-02
tranche_id: TP-APP-R2-PRIMCREATE-001
---

# WORKING_ITEMS Run - Primitive Load Creation Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The structured
operation validation/diff seam now accepts explicit concentrated-force
`create_primitive_load` operations.

## Implementation Evidence

- `apps/desktop/src/services/operationService.ts` adds browser-fixture
  validation, diff preview, and apply support for `create_primitive_load`.
- `core/model_operations/operation_applier/src/lib.rs` adds the matching Rust
  validation, diff preview, and apply behavior.
- Accepted operations must target an existing `Load` with
  `field_path=primitive_loads`, use `before=not_present`, use the project
  force unit and dimension `force`, provide a unique primitive-load id,
  category `concentrated_force`, existing node target, global direction,
  finite magnitude, and provenance.
- New TS and Rust tests verify valid concentrated-force payloads apply without
  mutating the input model and that duplicate primitive ids or missing node
  targets are blocked.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 26/26 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 41/41 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

This tranche adds only explicit concentrated nodal-force primitive create
validation and diff support. It does not add distributed-load creation,
concentrated moments, pressure/temperature primitive creation,
imposed-displacement authoring, combination basis editing, term
creation/deletion, hidden defaults, unit conversion, protected standards
content, private data, release readiness, professional approval,
certification, sealing, authentication, or code-compliance claims.

## Residual

Operation validation still needs future A4 support for distributed
primitive-load creation, concentrated moments, pressure/temperature primitive
creation, imposed-displacement authoring, combination basis editing, term
creation/deletion, and broader algebra authoring when those app surfaces are
selected.
