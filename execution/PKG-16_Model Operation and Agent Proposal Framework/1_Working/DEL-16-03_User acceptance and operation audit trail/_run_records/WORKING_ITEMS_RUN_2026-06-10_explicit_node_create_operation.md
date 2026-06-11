# WORKING_ITEMS Run Record — TP-APP-R2-CREATENODE-001

Date: 2026-06-10

Deliverable: DEL-16-03 — User acceptance and operation audit trail

Scope: Completion-plan Phase A3 sub-slice verifying the acceptance receipt
and audit-boundary behavior for explicit node creation.

Changes:

- Applying an explicit create-node intent records a local-session applied
  operation receipt with operation id, change id, target object/ref, field
  path, before/after payload, application route, acceptance basis, diagnostics,
  and professional-boundary flags.
- The receipt remains labeled
  `acceptance=user_initiated_apply_in_local_session`,
  `persistence=session_state_only_not_yet_saved`, and
  `professional_approval=false`.
- Prior solve results are cleared after the model mutation so stale analysis
  is not represented as current for the edited model.

Validation:

- `npm test --workspace apps/desktop` passed, 28/28, including the viewport
  create-node apply flow and receipt assertions.
- Browser smoke on `http://127.0.0.1:5174/` confirmed the applied receipt
  labels above after creating `node:N-150`; browser warnings/errors after the
  final reload were absent.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  passed, 20/20.

Boundary:

- This receipt is local review/audit evidence for an in-session model change
  only. It is not professional approval, certification, sealing,
  authentication, code compliance, release readiness, durable persistence, or
  protected/private data handling.
