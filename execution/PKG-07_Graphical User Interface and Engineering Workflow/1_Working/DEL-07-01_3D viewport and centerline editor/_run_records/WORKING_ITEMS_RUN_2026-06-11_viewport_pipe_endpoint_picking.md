---
doc_id: WORKING-ITEMS-RUN-2026-06-11-VIEWPORT-PIPE-ENDPOINT-PICKING-DEL-07-01
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-07
deliverable_id: DEL-07-01
tranche_id: TP-APP-R2-PIPEPICK-001
---

# WORKING_ITEMS Run - Viewport Pipe Endpoint Picking

## Scope

Bounded app-integration tranche for completion-plan A3. The viewport editor
now lets the user pick rendered node targets into the explicit straight-pipe
`from` and `to` fields.

## Implementation Evidence

- `apps/desktop/src/features/viewport/PipeViewport.tsx` adds an endpoint-pick
  mode for the explicit straight-pipe form.
- Arming `Pick` for `from` and selecting a viewport node fills the `from`
  field, advances to `to` picking, and preserves the normal viewport
  selection behavior.
- Selecting a second viewport node fills `to` and clears pick mode. If the same
  node would occupy both endpoints, the opposite endpoint is cleared so the
  existing validation still blocks an invalid zero-length pipe.
- Picking endpoints does not infer pipe id, material, section geometry,
  `y_reference`, provenance, or any engineering default.

## Validation

- `npm test --workspace apps/desktop` passed with 35/35 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke at `http://127.0.0.1:5175/` confirmed
  `from=node:N-100`, `to=node:N-140`, pick-mode advancement/clearing, and
  Queue pipe disabled until the remaining explicit fields are supplied.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

This tranche supplies explicit node references only. It does not queue/apply
without user action, mutate saved project data, infer material or section
values, implement component/rigid authoring, perform unit conversion, add
protected standards content, handle private data, or make release,
professional approval, certification, sealing, authentication, or
code-compliance claims.

## Residual

A3 still needs broader canvas creation/edit gestures, component/rigid
authoring, and editor coverage as new authoring surfaces land.
