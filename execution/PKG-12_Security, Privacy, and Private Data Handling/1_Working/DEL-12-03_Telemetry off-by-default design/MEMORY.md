# Memory: DEL-12-03 Telemetry off-by-default design

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
