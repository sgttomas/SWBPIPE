---
doc_id: WORKING-ITEMS-RUN-2026-06-11-VIEWPORT-CANVAS-NODE-DRAFTING-DEL-16-03
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-16
deliverable_id: DEL-16-03
tranche_id: TP-APP-R2-CANVASNODE-001
---

# WORKING_ITEMS Run - Viewport Canvas Node Drafting

## Scope

Bounded app-integration tranche verifying that a canvas-drafted node still
uses the explicit local user-acceptance/audit receipt path before the session
model changes.

## Implementation Evidence

- Canvas pointer capture fills the node form only; it does not enqueue or
  apply an operation.
- The app test explicitly clicks Queue node and applies
  `op:viewport-create-node-node:V-001-001` through `OperationApplyPanel`.
- The mutation remains session-state-only and uses the existing local
  acceptance path.

## Validation

- `npm test --workspace apps/desktop` passed with 34/34 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke verified draft capture without applying a mutation.

## Boundaries

This is local review/audit evidence only. It does not imply durable
persistence, lifecycle promotion, release readiness, professional approval,
certification, sealing, authentication, approval, code-compliance, protected
standards content, or private project data handling.

## Residual

Long-term audit retention, durable accepted-project operation ledgers, and
identity/timestamp finalization remain outside this tranche.
