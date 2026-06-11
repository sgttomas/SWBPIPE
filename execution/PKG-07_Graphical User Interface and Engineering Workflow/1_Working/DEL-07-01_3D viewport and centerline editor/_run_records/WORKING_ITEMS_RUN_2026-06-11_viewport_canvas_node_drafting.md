---
doc_id: WORKING-ITEMS-RUN-2026-06-11-VIEWPORT-CANVAS-NODE-DRAFTING-DEL-07-01
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-07
deliverable_id: DEL-07-01
tranche_id: TP-APP-R2-CANVASNODE-001
---

# WORKING_ITEMS Run - Viewport Canvas Node Drafting

## Scope

Bounded app-integration tranche for completion-plan A3. The viewport canvas
now captures a primary pointer gesture into an explicit node draft before any
operation is queued.

## Implementation Evidence

- `apps/desktop/src/features/viewport/PipeViewport.tsx` attaches a bounded
  pointer handler to the viewport canvas host.
- In WebGL mode, the handler raycasts from the Three.js camera to the `y=0`
  drafting plane; in fallback/test mode, it maps the pointer into the same
  model-drafting plane.
- The captured draft fills the visible explicit node fields with a generated
  `node:V-###` id, label, and finite x/y/z coordinates. The existing Queue
  node and Apply Operations path remains the only mutation route.

## Validation

- `npm test --workspace apps/desktop` passed with 34/34 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke at `http://127.0.0.1:5175/` confirmed one visible
  canvas, `node:V-001` draft fields, finite coordinates, `y=0`, and Queue
  node enabled.

## Boundaries

This tranche drafts explicit node geometry only. It does not queue/apply
without user action, mutate saved project data, implement pipe/component
gesture capture, infer engineering values, perform unit conversion, add
protected standards content, handle private data, or make release,
professional approval, certification, sealing, authentication, or
code-compliance claims.

## Residual

A3 still needs canvas gesture capture beyond node drafting
(pipe/connectivity and component/rigid authoring), rigid/component authoring,
and broader editor coverage as new authoring surfaces land.
