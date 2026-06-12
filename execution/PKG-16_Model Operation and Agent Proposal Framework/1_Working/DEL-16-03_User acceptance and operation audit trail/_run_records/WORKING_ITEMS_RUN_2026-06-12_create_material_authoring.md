# WORKING_ITEMS Run Record - TP-APP-R2-CREATEMATERIAL-001

Date: 2026-06-12
Persona: WORKING_ITEMS
Deliverable: DEL-16-03 User acceptance and operation audit trail
Package: PKG-16 Model Operation and Agent Proposal Framework

## Scope

Bounded app-integration tranche from completion-plan Phase A10:
ensure accepted material creation uses the same local-session acceptance
receipt and operation audit evidence as existing applied editor operations.

Write scope for this deliverable:
- `core/model_operations/operation_applier/src/lib.rs`
- `apps/desktop/src/features/model-tree/PropertyInspector.tsx`
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/SMOKE.md`

## Changes

- Successful `create_material` apply now returns an `OperationOutcome` with
  `acceptance_basis=user_initiated_apply_in_local_session`,
  `persistence_status=session_state_only_not_yet_saved`, and
  `acceptance_is_professional_approval=false`.
- The Property Inspector emits review-only `create_material` intents with the
  standard audit boundary:
  `mutation_route=structured_operations_only`,
  `direct_model_mutation_allowed=false`, and
  `mutates_accepted_model_state=false`.
- App-level regression verifies queue -> apply -> receipt for a material
  creation operation and confirms the newly created material is selected while
  prior solve state is cleared.

## Validation

- Focused App Vitest:
  `npm test --workspace apps/desktop -- src/App.test.tsx -t "queues and applies explicit material creation"`
  - 1/1 passed
- Full desktop Vitest: `npm test --workspace apps/desktop`
  - 180/180 passed
- Tauri Rust tests: `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 29/29 passed
- Live Chrome smoke at `http://127.0.0.1:5173/`
  - created and applied `material:M-301`
  - active model-tree row true
  - inspector showed `125000000000 Pa`, `48000000000 Pa`, and `0.00001 1/degC`
  - review context `0 pending operations; applied_operations=1`
  - receipt retained `professional_approval=false`
  - browser console errors: 0

## Boundary

Acceptance remains a local-session user action only. It is not durable save
evidence, professional approval, certification, sealing, authentication,
release readiness, or a code-compliance claim.

## Handoff

A10 section creation is still required for a fully fixture-independent
from-scratch authoring path. Lifecycle state is unchanged.
