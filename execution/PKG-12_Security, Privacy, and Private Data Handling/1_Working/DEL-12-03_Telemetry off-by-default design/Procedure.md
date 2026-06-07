# Procedure: DEL-12-03 Telemetry off-by-default design

## Purpose

Use this procedure to produce or review a telemetry design for OpenPipeStress without breaching the local-first privacy boundary. The current evidence set includes design documents, `docs/security/telemetry_policy.md`, a metadata-only guard helper in `core/security/telemetry_policy/`, and focused tests in `tests/security/test_telemetry_policy.py`. It does not implement runtime telemetry transport, endpoint, vendor, upload queue/job, telemetry persistence, product config schema/storage, consent UI/CLI, retention, support-bundle workflow, or approval records.

## Prerequisites

| Prerequisite | Status |
|---|---|
| Sealed deliverable context for DEL-12-03 | Present in `_CONTEXT.md`. |
| Scope item SOW-037 and objective OBJ-010 | Present in decomposition/registers. |
| Applicable invariants OPS-K-IP, OPS-K-DATA, OPS-K-AUTH, OPS-K-PRIV, OPS-K-AGENT | Present in `docs/CONTRACT.md`. |
| Telemetry policy documentation | Present in `docs/security/telemetry_policy.md`. |
| Metadata-only guard helper and focused tests | Present in `core/security/telemetry_policy/` and `tests/security/test_telemetry_policy.py`. |
| Explicit human/security approval for runtime telemetry collection | TBD; absent for runtime telemetry implementation. |
| Product configuration surface and schema | TBD; not selected by this deliverable. |

## Steps

1. Confirm the current brief is scoped only to DEL-12-03 and the assigned folder.
2. Read `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `_STATUS.md`, `INIT.md`, `AGENTS.md`, `docs/CONTRACT.md`, the DEL-12-03 register rows, and the applicable PKG-12/architecture-basis decomposition rows.
3. Treat telemetry as disabled unless explicit approved opt-in configuration exists.
4. If no human/security approval exists for telemetry collection, keep event names, endpoint details, vendor selections, payload fields, and transport behavior as `TBD` or no-op.
5. Verify configuration behavior so absent, unset, empty, unknown, unsupported, malformed, or incomplete telemetry metadata resolves to disabled.
6. Verify payload rules so private project, code, rule-pack, material, component, report, path, hash, secret, protected standards content, and professional/code-compliance claim fields cannot be collected or transmitted.
7. Verify tests that prove metadata-only default-off behavior, no payload/network initialization flags, event allowlist enforcement, and forbidden-field rejection. Runtime startup, plugin/adapter/report/private-library route tests remain future scope until those surfaces call the helper.
8. Record any unresolved approval, config, endpoint, or payload decision as `TBD` for human/security ruling.
9. Do not write product code or repo-level product artifacts unless a later sealed brief explicitly authorizes that scope.

## Verification

| Check | Pass condition |
|---|---|
| Four-document kit exists | `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` are present. |
| Telemetry policy artifact exists | `docs/security/telemetry_policy.md` is present and documents default-off policy plus metadata-only helper boundaries. |
| Metadata-only helper exists | `core/security/telemetry_policy/` is present and evaluates config/event metadata before payload construction. |
| Default sections preserved | Each document retains its required default section headings. |
| Privacy default | Specification states disabled-by-default and fail-closed config behavior. |
| No private data transmission | Specification forbids private project/code/rule/material/component/report/path/hash/secret/protected content in telemetry payloads. |
| No cloud assumption | Documents do not define cloud operation, endpoint, vendor, or upload behavior without human approval. |
| Current tests | `tests/security/test_telemetry_policy.py` covers metadata-only default-off, no-payload/no-network flags, allowlist gating, and forbidden-field rejection. |
| Future test expectations | Specification and Procedure keep runtime no-network-before-opt-in, product config, and no-bypass tests explicit for later integration surfaces. |
| Lifecycle | `_STATUS.md` remains outside this run's write scope; current local state is `IN_PROGRESS`, and this evidence alignment is not lifecycle promotion or acceptance. |

## Records

Deliverable-local records:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/*`

Current external evidence records include `docs/security/telemetry_policy.md`, `core/security/telemetry_policy/`, `tests/security/test_telemetry_policy.py`, `_run_records/TASK_RUN_2026-06-07_0141.md`, and the PKG-12 package fan-in run record. Future implementation records, if authorized, should include human approval evidence, config schema/default fixtures, payload allowlist, transport-disabled runtime tests, opt-in tests, payload privacy tests, and plugin/adapter bypass tests.
