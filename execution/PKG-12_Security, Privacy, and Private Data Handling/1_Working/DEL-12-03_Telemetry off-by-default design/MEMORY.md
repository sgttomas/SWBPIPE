# Memory: DEL-12-03 Telemetry off-by-default design

## 2026-07-12 - D-41 R5 T5 PDU-042 distinct affirmative request

- Added `Request telemetry enablement review` to the existing DEL-12-03 panel; no new GUI surface was created.
- Initial render records no request. The dedicated click records a one-shot local request explicitly distinct from terms, install, application/project open, and solve actions.
- O7 remains authoritative: the request resolves disabled because consent and allowlist approval are absent. It mutates no product config, grants no opt-in/consent, constructs no payload, persists nothing, and initializes no network behavior.
- Focused telemetry service Vitest passed 3/3; focused App interaction Vitest passed 1/1 selected; disposable copy-out production build passed with the existing large-chunk warning.
- No runtime telemetry, validation promotion, GUI scope expansion, lifecycle, review, dependency/DAG/register/decomposition, or `ISSUED` change.

## 2026-07-12 - D-41 R5 T3 PDU-026/PDU-043 O7-before-E5 seam

- Added `telemetryPolicyService.ts` as the bounded desktop pre-payload seam used by the DEL-12-03 panel. Missing/false configuration and even a capability request remain disabled because no affirmative opt-in or approved allowlist surface exists.
- Negative tests cover payload-shaped attempts, forbidden report fields, persistence keys, capability requests without consent, and invariant false network/endpoint/vendor/queue/upload/persistence/client/background-job evidence.
- PDU-043 remains documented unimplemented outside telemetry-panel attempts: plugins, adapters, import/export, reports, and private-library runtime paths are not claimed to be intercepted. Adapter approval/allowlist remains unselected, and the DEL-10-02 deny-only declaration-admission gate does not close telemetry runtime binding.
- This is verified software behavior only, not security validation, privacy/legal sufficiency, formal review closure, lifecycle transition, or authorization of telemetry transport.

## 2026-07-12 - D-41 R5 T1 PDU-077 telemetry-panel attribution

- DEC-074 O3 attributes `apps/desktop/src/features/telemetry` to DEL-12-03 as
  implementation evidence. The independent receiving-scope audit found no
  conflict with this deliverable's accepted default-off policy scope.
- `TelemetryBoundaryPanel.tsx` constructs a local policy-review artifact, not
  a telemetry payload. It records `payload_constructed=false`, initializes no
  network/runtime behavior, blocks the modeled attempts, and leaves product
  configuration, consent surface, and allowlist approval `TBD`.
- The panel is not consent UI and does not authorize collection, payload
  construction, endpoint/vendor selection, persistence, upload, transport, or
  runtime telemetry. Existing policy/helper tests and deferred runtime choices
  retain their prior authority and status.
- The four-document kit now cites the panel as bounded DEL-12-03 evidence.
  Genuine Remaining items and the D-41 bootstrap are preserved. No code,
  lifecycle, scope, release, professional-approval, certification, sealing,
  authentication, or code-compliance state changed.

## 2026-06-18 - TP-UNITS-BTAIL-EXPORTREVNONUNITBOUNDARY-001 supporting export-review classification

- Supporting role for Export Safety Review matrix cleanup: the
  `telemetry_boundary_review` export row is explicitly classified as
  non-unit-bearing metadata because it records disabled/default-off telemetry
  policy state without quantities, units, dimensions, payload construction, or
  target conversion.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-EXPORTREVNONUNITBOUNDARY-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-271; completion log entry; primary DEL-12-02
  run record and supporting DEL-10-04/DEL-02-02 run records.
- Boundary preserved: no telemetry runtime behavior, endpoint, vendor,
  payload construction, network transport, telemetry persistence, private
  payload, protected standards content, lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## Current Session

2026-05-02 - Implemented from sealed dispatch brief
`execution/_Coordination/DEV-001_DISPATCH_DEL-12-03.md`.

## Decisions And Rulings

- Human project authority authorized implementation from the sealed brief only
  after `DEL-12-05` closeout and `DEL-12-03` brief preparation were committed.
- Implementation stayed within the approved write scope:
  `docs/security/telemetry_policy.md`,
  `tests/security/test_telemetry_policy.py`, this `MEMORY.md`, the dispatch
  brief, and `NEXT_INSTANCE_STATE.md`.
- No lifecycle transition, implementation-evidence registration,
  dependency-register edit, blocker-queue refresh, `DAG-001` change,
  candidate-edge promotion, runtime telemetry code, vendor integration,
  endpoint selection, product config schema edit, plugin/runtime behavior
  change, or network behavior was performed.

## Source Basis

- `execution/_Coordination/DEV-001_DISPATCH_DEL-12-03.md`
- `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-03_Telemetry off-by-default design/_CONTEXT.md`
- `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-03_Telemetry off-by-default design/Specification.md`
- `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-03_Telemetry off-by-default design/Guidance.md`
- `docs/CONTRACT.md`
- `docs/IP_AND_DATA_BOUNDARY.md`
- `docs/security/threat_model.md`

## Implementation Notes

- Added `docs/security/telemetry_policy.md` as the public telemetry policy.
- Defined default-off behavior for absent, unset, empty, unknown, unsupported,
  and malformed telemetry configuration.
- Recorded fail-closed behavior: telemetry remains disabled unless the user
  explicitly opts in through an approved surface and a human-approved event
  allowlist exists.
- Prohibited telemetry initialization of network transport, upload jobs,
  queues, persistence, endpoints, vendors, and external service clients without
  opt-in and allowlist evidence.
- Prohibited telemetry payloads from carrying private project models,
  code-specific rule data, private rule packs, private material/component
  libraries, generated reports, model hashes, local file paths, secrets,
  credentials, protected standards content, or professional/code-compliance
  claims.
- Kept product config schema, consent UI/CLI, endpoint, vendor, transport,
  retention policy, concrete event schema, and support-bundle workflow as
  `TBD`.

## Verification

- `python3 -m pytest tests/security/test_telemetry_policy.py` passed.
- `git diff --check` passed.
- Focused protected-content/prohibited-claim/real-secret scan found only
  guardrail and exclusion wording in the telemetry policy, tests, dispatch
  brief, memory, and state.

## Remaining TBDs

- Product configuration schema and storage location.
- Consent UI or CLI surface.
- Endpoint, vendor, transport, and retention policy.
- Concrete event schema and event allowlist.
- Explicit user-selected support bundle or payload workflow.
- Lifecycle/evidence/local dependency-register alignment and blocker-queue
  refresh for `DEL-12-03`, if later authorized.

## 2026-05-11 TP-RECON-01 Reconciliation

- Reconciled historical implementation evidence for `DEL-12-03` from
  `TP-RECON-01`, archived DEV-001 evidence rows, lifecycle snapshot row, sealed
  dispatch brief, SCA-002 inventory note, and commit `7834b97`.
- Evidence records the 2026-05-02 bounded telemetry policy implementation:
  `docs/security/telemetry_policy.md`,
  `tests/security/test_telemetry_policy.py`, this `MEMORY.md`, the archived
  dispatch brief, and `NEXT_INSTANCE_STATE.md`.
- Implemented slice remains a policy/test guardrail only: default-off and
  fail-closed telemetry behavior, opt-in plus event-allowlist gating,
  forbidden payload classes, and no runtime telemetry transport or vendor
  integration.
- Verification evidence preserved from the dispatch record: focused
  `tests/security/test_telemetry_policy.py` passed, `git diff --check` passed,
  and scans found only guardrail/exclusion wording.
- Deferred scope remains: product config schema, consent UI/CLI, endpoint,
  vendor, transport, retention policy, concrete event schema, support-bundle
  workflow, runtime integration, dependency alignment, and blocker refresh.
- Lifecycle evidence stays `CHECKING`; this reconciliation records history
  only and does not promote lifecycle or expand scope.

## 2026-05-16 - DEV-001 downstream PKG-02 audit memory addendum

Durable context preserved after reconciliation review:
- DEV-001 package-worker audit reviewed this deliverable for downstream compatibility with the accepted PKG-02 foundation contracts.
- Local audit artifacts are `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-03_Telemetry off-by-default design/_REVIEW.md` and `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-03_Telemetry off-by-default design/Review_Findings.csv`.
- Package audit summary is `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`; package run record is `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/_run_records/TASK_RUN_2026-05-16_PKG12_PKG02_DOWNSTREAM_AUDIT.md`.
- This was audit evidence only. It did not change lifecycle state, authorize release, or make a professional, certification, sealing, approval, or code-compliance claim.
- The May 16 package-worker TASK run record did not fully preserve canonical per-deliverable TASK documentation context; this addendum preserves the durable deliverable-local pointer without modifying the completed run record.

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-12-03`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-07 - TP-PKG12 Local Privacy Guards

- TASK Worker B implemented the metadata-only telemetry policy guard module in
  `core/security/telemetry_policy/` and added focused tests in
  `tests/security/test_telemetry_policy.py`.
- The guard resolves absent, empty, unknown, unsupported, malformed, or
  incomplete telemetry configuration to disabled, and rejects unallowlisted or
  forbidden event metadata before payload construction.
- Evidence records:
  `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-03_Telemetry off-by-default design/_run_records/TASK_RUN_2026-06-07_0141.md`
  and
  `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/_run_records/WORKING_ITEMS_RUN_2026-06-07_0150_TP-PKG12-LOCAL-PRIVACY-GUARDS-FANIN.md`.
- Parent validation passed:
  `python3 -m pytest tests/security/test_local_first_storage_policy.py tests/security/test_telemetry_policy.py tests/security/test_redaction_export_controls.py tests/security/test_secret_private_library_handling.py`
  reported 44 passed; `git diff --check` passed.
- Focused protected-content/prohibited-claim scan found only boundary and
  prohibition wording; no lifecycle status, DAG artifact, dependency register,
  approval record, coordination prompt, telemetry schema, runtime telemetry
  module, endpoint, vendor, transport, queue, upload job, persistence,
  professional claim, code-compliance claim, or security-certification claim
  was introduced.

## 2026-06-07 - TP-PKG12 Local Privacy Guards Worker B

- Implemented the metadata-only telemetry guard helper in `core/security/telemetry_policy/` with `TelemetryConfig`, `TelemetryEventAttempt`, `TelemetryDiagnostic`, `TelemetryDecision`, `TelemetryGuardResult`, `resolve_telemetry_config(...)`, and `guard_telemetry_event(...)`.
- Guard behavior remains local and pre-payload: absent, empty, unknown, unsupported, or malformed configuration resolves disabled; enabled telemetry requires explicit opt-in, approved consent surface, and human-approved event allowlist evidence.
- Event attempts are evaluated by event name, field name, and field classification only. Unknown events/fields and private, protected, secret, path, hash, report, or professional-claim classes are rejected before payload construction.
- No `core/telemetry`, `apps/telemetry`, telemetry schema, endpoint, vendor, network transport, upload queue/job, telemetry persistence, or external service client was added.
- Updated `docs/security/telemetry_policy.md` only to document the metadata-only guard helper and its non-authority boundary.
- Verification: `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest tests/security/test_telemetry_policy.py` passed with 15 tests; `git diff --check` passed for the Worker B write set.
- This tranche did not change lifecycle state, dependency registers, approval records, DAG artifacts, coordination prompts, or Worker A local-first storage files.

## 2026-06-07 - DEL-12-03 readiness evidence alignment

- TASK C aligned the local DEL-12-03 kit, dependency summary, review surface, and review findings with June 7 evidence that `docs/security/telemetry_policy.md`, `core/security/telemetry_policy/`, and `tests/security/test_telemetry_policy.py` now exist.
- Updated local dependency rows for DEL-12-05, DEL-01-02, DEL-01-04, and DEL-05-04 from `TBD` to `SATISFIED` only where target status/review/run-record evidence supports current readiness; this does not alter DAG authority or lifecycle state.
- Preserved explicit deferrals for product config schema/storage, consent UI/CLI, endpoint/vendor/transport, event allowlist/schema, retention, support-bundle workflow, runtime telemetry integration, and human/security approval choices.
- No `_STATUS.md`, product code, schemas, tests, DAG artifacts, coordination files, approval records, release files, package registers, or other deliverables were edited.

## 2026-06-17 - Lifecycle Housekeeping

- Housekeeping lifecycle reset: `_STATUS.md` current state set to `IN_PROGRESS` to reflect current code development in progress. This does not change review, issuance, release readiness, professional approval, certification, sealing, authentication, or code-compliance status.
