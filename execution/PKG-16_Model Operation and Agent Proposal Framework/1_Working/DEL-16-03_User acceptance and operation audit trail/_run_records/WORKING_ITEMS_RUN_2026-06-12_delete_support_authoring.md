# WORKING_ITEMS Run Record - TP-APP-R2-DELSUPPORT-001

Date: 2026-06-12
Persona: WORKING_ITEMS
Deliverable: DEL-16-03 User acceptance and operation audit trail
Package: PKG-16 Model Operation and Agent Proposal Framework

## Scope

Bounded app-integration tranche from completion-plan Phase A11:
ensure accepted support deletion uses the same local-session acceptance
receipt and operation audit evidence as existing applied editor operations.

Write scope for this deliverable:
- `core/model_operations/operation_applier/src/lib.rs`
- `apps/desktop/src/App.tsx`
- `apps/desktop/src/features/model-tree/PropertyInspector.tsx`
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/SMOKE.md`

## Changes

- Successful `delete_support` apply returns an `OperationOutcome` with
  `acceptance_basis=user_initiated_apply_in_local_session`,
  `persistence_status=session_state_only_not_yet_saved`, and
  `acceptance_is_professional_approval=false`.
- The Property Inspector emits review-only `delete_support` intents with the
  standard audit boundary:
  `mutation_route=structured_operations_only`,
  `direct_model_mutation_allowed=false`, and
  `mutates_accepted_model_state=false`.
- App-level regression verifies queue -> apply -> receipt for unreferenced
  support deletion, confirms the deleted support leaves the model tree, and
  confirms the selection falls back to the project row.
- App-level blocking regression creates an imposed-displacement primitive load
  and verifies deletion of its target support remains blocked with
  `OP-SUPPORT-DELETE-REFERENCED` and no model mutation.

## Validation

- Focused App Vitest:
  `npm test --workspace apps/desktop -- src/App.test.tsx -t "support deletion"`
  - 2/2 passed
- Full desktop Vitest: `npm test --workspace apps/desktop`
  - 189/189 passed
- Tauri Rust tests: `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 29/29 passed
- Live Chrome smoke at `http://127.0.0.1:5173/`
  - deleted `support:S-120`
  - model tree no longer contained `support:S-120`
  - project row became active as the selection fallback
  - review context `0 pending operations; applied_operations=1`
  - receipt retained `professional_approval=false`
  - browser console errors: 0

## Boundary

Acceptance remains a local-session user action only. It is not durable save
evidence, professional approval, certification, sealing, authentication,
release readiness, or a code-compliance claim.

## Handoff

A11 has landed support deletion only. Remaining A11 entity deletion families
are node, pipe run, load case, primitive load, and full combination deletion.
Lifecycle state is unchanged.
