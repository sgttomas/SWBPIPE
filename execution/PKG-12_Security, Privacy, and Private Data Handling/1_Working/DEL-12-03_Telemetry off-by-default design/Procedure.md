# Procedure: DEL-12-03 Telemetry off-by-default design

## Purpose

Use this procedure to produce or review a telemetry design for OpenPipeStress without breaching the local-first privacy boundary. The current setup procedure produces design documents only; it does not implement product code.

## Prerequisites

| Prerequisite | Status |
|---|---|
| Sealed deliverable context for DEL-12-03 | Present in `_CONTEXT.md`. |
| Scope item SOW-037 and objective OBJ-010 | Present in decomposition/registers. |
| Applicable invariants OPS-K-IP, OPS-K-DATA, OPS-K-AUTH, OPS-K-PRIV, OPS-K-AGENT | Present in `docs/CONTRACT.md`. |
| Explicit human/security approval for telemetry implementation | TBD; absent for this setup pass. |
| Product configuration surface and schema | TBD; not selected by this deliverable. |

## Steps

1. Confirm the current brief is scoped only to DEL-12-03 and the assigned folder.
2. Read `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `_STATUS.md`, `INIT.md`, `AGENTS.md`, `docs/CONTRACT.md`, the DEL-12-03 register rows, and the applicable PKG-12/architecture-basis decomposition rows.
3. Treat telemetry as disabled unless explicit approved opt-in configuration exists.
4. If no human/security approval exists for telemetry collection, keep event names, endpoint details, vendor selections, payload fields, and transport behavior as `TBD` or no-op.
5. Define configuration behavior so absent, unset, unknown, or malformed telemetry settings resolve to disabled.
6. Define payload rules so private project, code, rule-pack, material, component, report, path, hash, secret, and protected standards content cannot be collected or transmitted.
7. Define tests that prove default-off behavior, no network activity before opt-in, payload allowlist enforcement, and no bypass by plugins/adapters/reports/private-library paths.
8. Record any unresolved approval, config, endpoint, or payload decision as `TBD` for human/security ruling.
9. Do not write product code or repo-level product artifacts unless a later sealed brief explicitly authorizes that scope.

## Verification

| Check | Pass condition |
|---|---|
| Four-document kit exists | `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` are present. |
| Default sections preserved | Each document retains its required default section headings. |
| Privacy default | Specification states disabled-by-default and fail-closed config behavior. |
| No private data transmission | Specification forbids private project/code/rule/material/component/report/path/hash/secret/protected content in telemetry payloads. |
| No cloud assumption | Documents do not define cloud operation, endpoint, vendor, or upload behavior without human approval. |
| Test expectations | Specification and Procedure include default-off, no-network-before-opt-in, payload-filter, and bypass tests. |
| Lifecycle | `_STATUS.md` remains `SEMANTIC_READY`, not `ISSUED`. |

## Records

Records for this setup pass:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/*`

Future implementation records, if authorized, should include human approval evidence, config schema/default fixtures, payload allowlist, transport-disabled tests, opt-in tests, payload privacy tests, and plugin/adapter bypass tests.
