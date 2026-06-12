# WORKING_ITEMS Run Record - TP-APP-R2-DELCOMBINATION-001

Date: 2026-06-12
Persona: WORKING_ITEMS
Deliverable: DEL-16-03 User acceptance and operation audit trail
Package: PKG-16 Model Operation and Agent Proposal Framework

## Scope

Bounded app-integration tranche from completion-plan Phase A11:
ensure accepted full-combination deletion uses the same local-session
acceptance receipt and operation audit evidence as existing applied editor
operations.

Write scope for this deliverable:
- `core/model_operations/operation_applier/src/lib.rs`
- `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx`
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/e2e/r2-smoke.spec.ts`
- `apps/desktop/SMOKE.md`

## Changes

- Successful `delete_combination` apply returns an `OperationOutcome` with
  `acceptance_basis=user_initiated_apply_in_local_session`,
  `persistence_status=session_state_only_not_yet_saved`, and
  `acceptance_is_professional_approval=false`.
- The Load Cases manager emits review-only `delete_combination` intents with
  the standard audit boundary:
  `mutation_route=structured_operations_only`,
  `direct_model_mutation_allowed=false`, and
  `mutates_accepted_model_state=false`.
- App-level regression verifies queue -> apply -> receipt for whole
  combination deletion, confirms the deleted combination row leaves the
  manager list, and confirms stale solve state is reset.
- The Playwright R2 smoke now asserts the rendered whole-combination delete
  preview in the real browser path.

## Validation

- Focused App Vitest:
  `npm test --workspace apps/desktop -- src/App.test.tsx -t "combination deletion"`
  - 1/1 passed
- Full desktop Vitest: `npm test --workspace apps/desktop`
  - 195/195 passed
- Tauri Rust tests: `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 29/29 passed
- Playwright R2 smoke: `npm run test:e2e --workspace apps/desktop`
  - 1/1 passed
- Live in-app browser smoke at `http://127.0.0.1:5173/`
  - deleted `combination:C-OPER-ALT`
  - combination list no longer contained `combination:C-OPER-ALT`
  - load-case summary reported `2 load cases; 7 primitive loads; 0 combinations`
  - review context `0 pending operations; applied_operations=1`
  - receipt retained `professional_approval=false`
  - browser console errors: 0

## Boundary

Acceptance remains a local-session user action only. It is not durable save
evidence, professional approval, certification, sealing, authentication,
release readiness, or a code-compliance claim.

## Handoff

A11 has landed support deletion, primitive-load deletion, and full
combination deletion. Remaining A11 entity deletion families are node, pipe
run, and load case deletion. Lifecycle state is unchanged.
