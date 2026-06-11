---
doc_id: WORKING-ITEMS-RUN-2026-06-11-LOAD-CASE-CREATION-EDITOR-DEL-16-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-16
deliverable_id: DEL-16-02
tranche_id: TP-APP-R2-LOADCREATE-001
---

# WORKING_ITEMS Run - Load Case Creation Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The structured
operation validation/diff seam now accepts explicit empty `create_load_case`
operations.

## Implementation Evidence

- `apps/desktop/src/services/operationService.ts` adds browser-fixture
  validation, diff preview, and apply support for `create_load_case`.
- `core/model_operations/operation_applier/src/lib.rs` adds the matching Rust
  validation, diff preview, and apply behavior.
- Accepted operations must target `Load` with `field_path=load_cases`, use
  `before=not_present`, use unit `none` and dimension `dimensionless`, avoid
  duplicate load-case ids, provide matching JSON id, provide non-empty label,
  kind, status, and provenance, and omit primitives or provide an empty
  `primitive_loads` array.
- New TS and Rust tests verify that valid empty shells apply without mutating
  the input model and that non-empty primitive payloads are rejected.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 25/25 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 39/39 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

This tranche adds only empty load-case shell create validation and diff
support. It does not add primitive-load creation, imposed-displacement
authoring, combination basis editing, term creation/deletion, hidden defaults,
unit conversion, protected standards content, private data, release readiness,
professional approval, certification, sealing, authentication, or
code-compliance claims.

## Residual

Operation validation still needs future A4 support for explicit arbitrary
primitive-load creation, imposed-displacement authoring, combination basis
editing, term creation/deletion, and broader algebra authoring when those app
surfaces are selected.
