# WORKING_ITEMS Run Record - TP-APP-R2-DELPRIMLOAD-001

Date: 2026-06-12
Persona: WORKING_ITEMS
Deliverable: DEL-16-03 User acceptance and operation audit trail
Package: PKG-16 Model Operation and Agent Proposal Framework

## Scope

Bounded app-integration tranche from completion-plan Phase A11:
ensure accepted primitive-load deletion uses the same local-session acceptance
receipt and operation audit evidence as existing applied editor operations.

Write scope for this deliverable:
- `core/model_operations/operation_applier/src/lib.rs`
- `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx`
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/SMOKE.md`

## Changes

- Successful `delete_primitive_load` apply returns an `OperationOutcome` with
  `acceptance_basis=user_initiated_apply_in_local_session`,
  `persistence_status=session_state_only_not_yet_saved`, and
  `acceptance_is_professional_approval=false`.
- The Load Cases manager emits review-only `delete_primitive_load` intents
  with the standard audit boundary:
  `mutation_route=structured_operations_only`,
  `direct_model_mutation_allowed=false`, and
  `mutates_accepted_model_state=false`.
- App-level regression verifies queue -> apply -> receipt for primitive-load
  deletion, confirms the deleted primitive row leaves the manager list, and
  confirms stale solve state is reset.

## Validation

- Focused App Vitest:
  `npm test --workspace apps/desktop -- src/App.test.tsx -t "primitive load deletion"`
  - 1/1 passed
- Full desktop Vitest: `npm test --workspace apps/desktop`
  - 192/192 passed
- Tauri Rust tests: `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 29/29 passed
- Live Chrome smoke at `http://127.0.0.1:5173/`
  - deleted `load:L-100-Y`
  - primitive-load list no longer contained `load:L-100-Y`
  - load-case summary reported `2 load cases; 6 primitive loads; 1 combinations`
  - `load:L-100` reported `primitives=3`
  - review context `0 pending operations; applied_operations=1`
  - receipt retained `professional_approval=false`
  - browser console errors: 0

## Boundary

Acceptance remains a local-session user action only. It is not durable save
evidence, professional approval, certification, sealing, authentication,
release readiness, or a code-compliance claim.

## Handoff

A11 has landed support deletion and primitive-load deletion. Remaining A11
entity deletion families are node, pipe run, load case, and full combination
deletion. Lifecycle state is unchanged.
