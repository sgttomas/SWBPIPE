---
doc_id: WORKING-ITEMS-RUN-2026-06-11-LOAD-CASE-METADATA-EDITOR-DEL-16-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-16
deliverable_id: DEL-16-02
tranche_id: TP-APP-R2-LOADMETA-001
---

# WORKING_ITEMS Run - Load Case Metadata Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The structured
operation validation/diff seam now accepts `Load.status` and `Load.kind` as
explicit text metadata fields.

## Implementation Evidence

- `core/model_operations/operation_applier/src/lib.rs` adds `status` and
  `kind` to `Load` field rules and removes those two paths from the deferred
  field list.
- `apps/desktop/src/services/operationService.ts` mirrors the same browser
  fixture behavior.
- Existing combination `basis` and `terms` fields remain explicitly deferred,
  preserving the A4 residual boundary for combination editing.
- New Rust tests verify `Load.status` and `Load.kind` apply without mutating
  the input model and that a deferred combination field still blocks with an
  explicit scope finding.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 23/23 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 36/36 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

This tranche adds only explicit text-field validation and diff support for
existing load-case metadata. It does not add load-case creation, arbitrary
primitive-load creation, combination editing, hidden defaults, unit conversion,
protected standards content, private data, release readiness, professional
approval, certification, sealing, authentication, or code-compliance claims.

## Residual

Operation validation still needs future A4 support for explicit load-case
creation, primitive-load creation/edit breadth, imposed-displacement authoring,
and combination editing when those app surfaces are selected.
