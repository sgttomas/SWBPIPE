---
doc_id: WORKING-ITEMS-RUN-2026-06-11-EXPLICIT-STRAIGHT-PIPE-CONNECTIVITY-DEL-07-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-07
deliverable_id: DEL-07-02
tranche_id: TP-APP-R2-CONNECTPIPE-001
---

# WORKING_ITEMS Run - Explicit Straight-Pipe Connectivity

## Scope

Bounded app-integration tranche for completion-plan A3. Applying an explicit
viewport pipe intent now appends a straight-pipe segment to the session model,
selects the created pipe, and exposes it through the existing model tree and
property inspector.

## Implementation Evidence

- `apps/desktop/src/App.test.tsx` now covers queue/apply of
  `pipe:P-150`, active model-tree selection, viewport selection-layer binding,
  property-inspector visibility, and local-session acceptance receipt evidence.
- `apps/desktop/src/types.ts` allows optional `y_reference` on pipe segments so
  newly created explicit pipe payloads preserve local reference-vector data
  while existing fixtures remain compatible.

## Validation

- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  passed with 22/22 Rust tests.
- `npm test --workspace apps/desktop` passed with 33/33 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.

## Boundaries

The inspector did not gain new endpoint-edit semantics. This tranche does not
perform durable persistence, hidden geometry defaults, unit conversion,
protected/private data handling, release readiness, professional approval,
certification, sealing, authentication, or code-compliance claims.

## Residual

A3 still needs canvas raycast/gesture geometry capture, rigid/component
authoring, and broader editor coverage. Endpoint editing remains a separate
operation-contract decision/slice.
