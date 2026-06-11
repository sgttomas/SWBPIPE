---
doc_id: WORKING-ITEMS-RUN-2026-06-11-EXPLICIT-STRAIGHT-PIPE-CONNECTIVITY-DEL-16-03
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-16
deliverable_id: DEL-16-03
tranche_id: TP-APP-R2-CONNECTPIPE-001
---

# WORKING_ITEMS Run - Explicit Straight-Pipe Connectivity

## Scope

Bounded app-integration tranche verifying that explicit straight-pipe creation
uses the existing local user-acceptance/audit receipt path.

## Implementation Evidence

- The app test queues and applies `op:viewport-connect-pipe-pipe:P-150-001`
  through `OperationApplyPanel`.
- The applied operation receipt records
  `acceptance=user_initiated_apply_in_local_session`,
  `persistence=session_state_only_not_yet_saved`, and
  `professional_approval=false`.
- Applying the operation appends the pipe to session state only; durable
  project mutation remains behind the separate Save local workflow.

## Validation

- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  passed with 22/22 Rust tests.
- `npm test --workspace apps/desktop` passed with 33/33 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.

## Boundaries

This is local-session acceptance evidence only. It does not imply durable
persistence, lifecycle promotion, release readiness, professional approval,
certification, sealing, authentication, approval, code-compliance, protected
standards content, or private project data handling.

## Residual

Long-term audit retention, durable accepted-project operation ledgers, and
identity/timestamp finalization remain outside this tranche.
