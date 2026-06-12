# WORKING_ITEMS Run Record - TP-APP-R2-DELNODE-001

Date: 2026-06-12
Persona: WORKING_ITEMS
Deliverable: DEL-16-03 User acceptance and operation audit trail
Package: PKG-16 Model Operation and Agent Proposal Framework

## Scope

Bounded app-integration tranche from completion-plan Phase A11:
ensure accepted node deletion uses the same local-session acceptance receipt
and operation audit evidence as existing applied editor operations.

Write scope for this deliverable:
- `core/model_operations/operation_applier/src/lib.rs`
- `apps/desktop/src/App.tsx`
- `apps/desktop/src/features/model-tree/PropertyInspector.tsx`
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/e2e/r2-smoke.spec.ts`
- `apps/desktop/SMOKE.md`

## Changes

- Successful `delete_node` apply returns an `OperationOutcome` with
  `acceptance_basis=user_initiated_apply_in_local_session`,
  `persistence_status=session_state_only_not_yet_saved`, and
  `acceptance_is_professional_approval=false`.
- Referenced node deletion is refused before application with
  `OP-NODE-DELETE-REFERENCED`; the queued operation remains pending for user
  review and does not mutate the model.
- The Property Inspector emits review-only `delete_node` intents with the
  standard audit boundary:
  `mutation_route=structured_operations_only`,
  `direct_model_mutation_allowed=false`, and
  `mutates_accepted_model_state=false`.
- App-level regressions verify both create -> queue -> apply -> receipt for
  an unreferenced node and queue -> blocked diagnostic for a node still
  referenced by pipes/supports.
- The Playwright R2 smoke now asserts the rendered node delete preview in the
  real browser path.

## Validation

- Focused App Vitest:
  `npm test --workspace apps/desktop -- App.test.tsx -t "node deletion"`
  - 2/2 passed
- Full desktop Vitest: `npm test --workspace apps/desktop`
  - 213/213 passed
- Tauri Rust tests: `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 29/29 passed
- Playwright R2 smoke:
  `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts`
  - 1/1 passed
- Live in-app browser smoke at `http://127.0.0.1:5173/`
  - created `node:N-160`
  - applied `op:delete-node-node:N-160`
  - review context `0 pending operations; applied_operations=2`
  - receipt retained `professional_approval=false`
  - browser console errors: 0

## Boundary

Acceptance remains a local-session user action only. It is not durable save
evidence, professional approval, certification, sealing, authentication,
release readiness, or a code-compliance claim.

## Handoff

A11 entity deletion coverage is complete. Next unblocked plan item is A12
from-blank create -> solve -> report rehearsal, followed by A8 journey
automation as the R2 exit-evidence backbone. Lifecycle state is unchanged.
