---
doc_id: WORKING-ITEMS-RUN-2026-06-11-VIEWPORT-PIPE-ENDPOINT-PICKING-DEL-07-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-07
deliverable_id: DEL-07-02
tranche_id: TP-APP-R2-PIPEPICK-001
---

# WORKING_ITEMS Run - Viewport Pipe Endpoint Picking

## Scope

Bounded app-integration tranche for completion-plan A3. Viewport node-target
selection now doubles as an explicit pipe endpoint-picking workflow when the
user arms a `from` or `to` pick control.

## Implementation Evidence

- The viewport selection layer keeps its existing model-tree/inspector
  selection behavior while also filling the armed straight-pipe endpoint when
  the selected target is a node.
- The app test picks `node:N-100` and `node:N-140` from viewport targets,
  fills the remaining explicit pipe fields, queues and applies `pipe:P-151`,
  and verifies the created pipe becomes active in the model tree, viewport
  layer, and property inspector.
- Endpoint picking leaves material, section, `y_reference`, and provenance to
  the existing explicit form validation rather than silently inventing them.

## Validation

- `npm test --workspace apps/desktop` passed with 35/35 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke at `http://127.0.0.1:5175/` confirmed endpoint values
  and pick-mode state in the rendered UI.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

This tranche changes local editor selection behavior only. It does not expand
durable persistence, broaden the property inspector's editable field map,
perform unit conversion, add protected standards content, handle private data,
or make release, professional approval, certification, sealing,
authentication, or code-compliance claims.

## Residual

A3 still needs broader canvas creation/edit gestures, component/rigid
authoring, and future editor-surface coverage.
