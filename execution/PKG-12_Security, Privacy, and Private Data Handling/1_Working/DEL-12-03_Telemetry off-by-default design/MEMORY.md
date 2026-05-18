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
