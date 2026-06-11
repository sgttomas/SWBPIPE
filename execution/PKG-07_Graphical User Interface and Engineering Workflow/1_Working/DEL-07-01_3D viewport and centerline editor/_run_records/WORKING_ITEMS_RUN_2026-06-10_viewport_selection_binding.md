---
run_id: WORKING_ITEMS_RUN_2026-06-10_viewport_selection_binding
agent: WORKING_ITEMS
tranche: TP-APP-R2-VIEWSELECT-001
date: 2026-06-10
status: SUCCESS
deliverable_id: DEL-07-01
package_id: PKG-07
completion_plan_item: Phase A3
---

# WORKING_ITEMS Run Record - Viewport Selection Binding

## Scope

Implemented a bounded A3 app-integration sub-slice for the desktop viewport:
selectable viewport controls for loaded nodes, straight pipe segments,
supports, and component markers.

## Changed App Surfaces

- `apps/desktop/src/features/viewport/PipeViewport.tsx`
- `apps/desktop/src/App.tsx`
- `apps/desktop/src/styles.css`
- `apps/desktop/src/App.test.tsx`

## Evidence Surfaces

- `apps/desktop/SMOKE.md` TP-MAC-84
- `plans/PLAN_2026-06-10_prd_completion.md`
- this deliverable `MEMORY.md`

## Validation

- `npm test --workspace apps/desktop` - passed, 26/26 Vitest tests.
- `npm run build --workspace apps/desktop` - passed, including `tsc -b` and
  Vite production build.
- Browser smoke on `http://127.0.0.1:5174/` - passed. The viewport exposed 14
  selection targets; selecting `pipe:P-120` updated the inspector to
  `Rack span`, the model-tree row to `tree-row active`, the viewport target to
  `aria-pressed=true`, and the toolbar to `Selected: pipe:P-120`. Console
  warnings/errors were absent.

No Rust or Python source changed in this tranche.

## Boundary Review

- The viewport selection layer updates shared UI selection only.
- It does not mutate persisted project payloads, create geometry values,
  apply operations, perform unit conversion, access network/cloud/telemetry,
  include protected standards data, include private project data, or make
  release, professional approval, certification, sealing, authentication, or
  code-compliance claims.
- Geometry-creation viewport intents remain blocked at apply until a later A3
  tranche supplies explicit coordinates/connectivity.

## Residuals

- True canvas raycast/gesture geometry capture.
- Node and straight-pipe creation tools with explicit coordinates and
  connectivity.
- Undo/redo.
- Inline validation messages.

## Lifecycle

`DEL-07-01` remains `CHECKING`. This run is app-integration evidence only and
does not issue the deliverable.
