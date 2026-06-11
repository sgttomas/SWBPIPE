---
doc_id: WORKING-ITEMS-RUN-2026-06-11-EXPLICIT-STRAIGHT-PIPE-CONNECTIVITY-DEL-16-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-16
deliverable_id: DEL-16-02
tranche_id: TP-APP-R2-CONNECTPIPE-001
---

# WORKING_ITEMS Run - Explicit Straight-Pipe Connectivity

## Scope

Bounded app-integration tranche extending the operation validation/diff/apply
contract to explicit straight-pipe creation.

## Implementation Evidence

- `core/model_operations/operation_applier/src/lib.rs` now resolves
  `connect_pipe_run` only when it targets `Element` + `pipe_segments` with an
  explicit JSON payload.
- Validation requires `before=not_present`, project length unit,
  `dimension=length`, non-duplicate pipe id, existing endpoint nodes/material,
  positive OD/wall quantities, non-empty provenance, and non-zero
  `y_reference`.
- `apps/desktop/src/services/operationService.ts` mirrors the Rust semantics
  for browser fixture/jsdom operation validation.
- The diff preview row is generated only after validation passes; legacy
  `viewport.connect_pipe_run` gestures remain blocked.

## Validation

- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  passed with 22/22 Rust tests, including explicit pipe apply and
  underspecified gesture blocking.
- `npm test --workspace apps/desktop` passed with 33/33 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.

## Boundaries

No unit conversion, hidden geometry defaults, protected standards content,
private project data, durable accepted-state mutation, release readiness,
professional approval, certification, sealing, authentication, or
code-compliance claim was added.

## Residual

Broader connect/edit operation shapes remain future slices: canvas-captured
geometry, rigid/component authoring, endpoint edits, and unit-conversion
support after the Phase B units engine.
