---
doc_id: WORKING-ITEMS-RUN-2026-06-11-VIEWPORT-CANVAS-NODE-DRAFTING-DEL-07-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-07
deliverable_id: DEL-07-02
tranche_id: TP-APP-R2-CANVASNODE-001
---

# WORKING_ITEMS Run - Viewport Canvas Node Drafting

## Scope

Bounded app-integration tranche for completion-plan A3. A viewport canvas
gesture now populates the explicit node form, and an accepted queued draft
continues into the model tree and property inspector through the existing
selection contract.

## Implementation Evidence

- `apps/desktop/src/App.test.tsx` now covers a canvas-drafted `node:V-001`
  queued and applied through `OperationApplyPanel`.
- The test verifies the created node becomes the active model-tree row and
  that the property inspector shows the captured position in project length
  units.
- The Playwright smoke verifies the draft-only browser path without mutating
  the fixture model before solve/report checks.

## Validation

- `npm test --workspace apps/desktop` passed with 34/34 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke at `http://127.0.0.1:5175/` confirmed the draft fields
  and enabled Queue node button.

## Boundaries

This tranche does not broaden property-inspector edit semantics, perform
durable persistence, infer engineering values, convert units, add protected
standards content, handle private data, or make release, professional
approval, certification, sealing, authentication, or code-compliance claims.

## Residual

Broader canvas authoring and future editor surfaces remain A3 residual scope.
