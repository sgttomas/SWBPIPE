---
doc_id: WORKING-ITEMS-RUN-2026-06-11-VIEWPORT-PIPE-ENDPOINT-PICKING-DEL-16-03
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-16
deliverable_id: DEL-16-03
tranche_id: TP-APP-R2-PIPEPICK-001
---

# WORKING_ITEMS Run - Viewport Pipe Endpoint Picking

## Scope

Bounded app-integration tranche for completion-plan A3. Endpoint picking feeds
the existing explicit straight-pipe operation form; the user still queues and
applies the structured operation through the local acceptance panel.

## Implementation Evidence

- The endpoint picker fills only `from` and `to` node references in the
  straight-pipe draft.
- The app test then supplies the remaining explicit fields, queues
  `op:viewport-connect-pipe-pipe:P-151-001`, applies it through
  `OperationApplyPanel`, and verifies the created pipe is selected.
- The operation remains a local-session accepted model change with the
  existing `requires_user_acceptance=true` and
  `session_state_only_not_yet_saved` posture.

## Validation

- `npm test --workspace apps/desktop` passed with 35/35 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke at `http://127.0.0.1:5175/` confirmed rendered
  endpoint-pick state and the disabled Queue pipe boundary before remaining
  explicit pipe data is supplied.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

This is local acceptance-trail evidence only. It does not imply durable
persistence, release readiness, professional approval, certification, sealing,
authentication, approval, code compliance, protected standards handling, or
private data handling.

## Residual

A3 still needs broader canvas creation/edit gestures and component/rigid
authoring. Future accepted operation surfaces must keep explicit
user-acceptance and non-claim boundaries visible.
