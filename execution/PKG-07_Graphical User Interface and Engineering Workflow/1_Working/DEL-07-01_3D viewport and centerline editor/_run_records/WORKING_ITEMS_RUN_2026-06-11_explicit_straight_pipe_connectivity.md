---
doc_id: WORKING-ITEMS-RUN-2026-06-11-EXPLICIT-STRAIGHT-PIPE-CONNECTIVITY-DEL-07-01
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-07
deliverable_id: DEL-07-01
tranche_id: TP-APP-R2-CONNECTPIPE-001
---

# WORKING_ITEMS Run - Explicit Straight-Pipe Connectivity

## Scope

Bounded app-integration tranche for completion-plan A3. The viewport editor now
has an explicit straight-pipe form for user-entered pipe id, label, endpoint
nodes, material, OD, wall, non-zero `y_reference`, and provenance.

## Implementation Evidence

- Added `PipeDraft` state and a compact explicit straight-pipe form in
  `apps/desktop/src/features/viewport/PipeViewport.tsx`.
- The form queues a structured `connect_pipe_run` intent targeting
  `object_type=Element`, `field_path=pipe_segments`, `before=not_present`,
  and `dimension=length`.
- Legacy one-click `Pipe-run intent` remains an underspecified viewport gesture
  and remains blocked by the operation applier.

## Validation

- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  passed with 22/22 Rust tests.
- `npm test --workspace apps/desktop` passed with 33/33 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.

## Boundaries

This tranche does not implement canvas raycast capture, hidden geometry
defaults, unit conversion, durable persistence, protected standards content,
private project data, release readiness, professional approval,
certification, sealing, authentication, or code-compliance claims.

## Residual

A3 still needs true canvas raycast/gesture geometry capture,
rigid/component authoring, and broader editor coverage as new authoring
surfaces land.
