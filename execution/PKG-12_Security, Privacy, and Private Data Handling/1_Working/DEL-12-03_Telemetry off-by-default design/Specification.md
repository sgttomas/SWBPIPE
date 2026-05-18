# Specification: DEL-12-03 Telemetry off-by-default design

## Scope

This deliverable specifies the privacy and configuration design for telemetry in OpenPipeStress. It covers policy, default configuration behavior, and tests needed to prove that telemetry is disabled by default and cannot transmit private engineering, code, rule-pack, component, material, report, secret, path, or protected standards data.

This setup pass does not implement product code, define a telemetry vendor, create cloud service behavior, choose an endpoint, or approve telemetry collection. Any later telemetry implementation remains blocked on explicit human/security approval.

## Requirements

| ID | Requirement | Verification |
|---|---|---|
| TEL-REQ-001 | Telemetry shall be disabled by default. An absent, unset, or malformed telemetry setting shall resolve to disabled. | Default configuration tests; malformed-config validation test. |
| TEL-REQ-002 | Telemetry shall not initialize network transport, background upload jobs, queues, or persistence for telemetry unless the user has explicitly opted in. | Startup test with default settings; transport initialization test. |
| TEL-REQ-003 | Telemetry, if later implemented, shall require explicit human/security approval of the event allowlist before any event is collected. | Review-gate checklist; allowlist approval record. |
| TEL-REQ-004 | Telemetry shall not collect or transmit private project models, code-specific rule data, private rule packs, private material or component libraries, generated reports, model hashes, local file paths, user secrets, credentials, or protected standards content. | Payload schema test; redaction/denylist test; protected-content/privacy scan. |
| TEL-REQ-005 | Opt-in state shall be represented as a local configuration value whose default is false. The exact storage location and schema are TBD until the implementation deliverable chooses the product config surface. | Config-default fixture test once schema exists; TBD tracked in implementation notes. |
| TEL-REQ-006 | Turning telemetry on shall require an affirmative user action distinct from accepting general terms, installing the software, opening a project, or running a solve. | UX/service acceptance test once UI or CLI surface exists. |
| TEL-REQ-007 | Telemetry settings and diagnostics shall preserve the distinction between mechanics solved, user-rule checked, and human-approved states; telemetry shall not create professional approval or compliance claims. | Report/diagnostic text scan; status vocabulary test. |
| TEL-REQ-008 | Telemetry-related diagnostics, if emitted, shall use the project diagnostic envelope when available and shall not include private payload content in messages. | Diagnostic schema test; privacy snapshot test. |
| TEL-REQ-009 | Plugins, adapters, import/export paths, reports, and private-library mechanisms shall not bypass telemetry opt-in or privacy filters. | Adapter/plugin boundary tests when those surfaces exist. |
| TEL-REQ-010 | If telemetry remains unimplemented in MVP, product behavior shall remain a no-op with tests proving no outbound telemetry behavior is reachable by default. | No-op smoke test; network-denial or transport-mock test. |

## Standards

| Source | Applicable constraint |
|---|---|
| `docs/CONTRACT.md` OPS-K-PRIV-1 | Private project, material, component, and rule-pack data must not be transmitted or committed publicly by default. |
| `docs/CONTRACT.md` OPS-K-PRIV-2 | Telemetry is off by default and cannot include private engineering or code data. |
| `docs/CONTRACT.md` OPS-K-IP-1/2/3 | Protected standards text, tables, examples, copied formulas, and proprietary data must not enter public artifacts or telemetry payloads. |
| `docs/CONTRACT.md` OPS-K-AUTH-1/2 | Software and agents must not claim certification, sealing, approval, authentication, or engineering code compliance for reliance. |
| `docs/DIRECTIVE.md` Section 4.2 | Hidden cloud storage or telemetry of private project/rule data is out of scope. |
| `docs/IP_AND_DATA_BOUNDARY.md` Sections 3 and 6 | Private rule packs, owner standards, company design bases, component catalogs, and project data remain user controlled. |
| `docs/_Decomposition/SOFTWARE_DECOMP.md` SOW-037 and OI-008 | Telemetry, if implemented, is opt-in and privacy preserving; MVP default is no telemetry; any telemetry design requires explicit human approval. |

## Verification

Minimum test coverage for any later implementation:

| Test ID | Test intent | Expected result |
|---|---|---|
| TEL-TEST-001 | Start application with no telemetry config. | No telemetry transport, queue, endpoint, or upload job is initialized. |
| TEL-TEST-002 | Load malformed telemetry config. | Telemetry remains disabled and a validation finding is emitted. |
| TEL-TEST-003 | Attempt to emit telemetry before opt-in. | Event is dropped locally without network activity. |
| TEL-TEST-004 | Attempt to include private model, rule-pack, material, component, report, path, hash, or secret fields. | Field is rejected before payload construction. |
| TEL-TEST-005 | Enable opt-in using an explicit approved config path. | Only approved allowlist fields are eligible for collection; private data remains excluded. |
| TEL-TEST-006 | Exercise plugin, adapter, report, and private-library routes. | No route bypasses telemetry opt-in or payload filtering. |
| TEL-TEST-007 | Scan telemetry messages and docs. | No certification, approval, seal, or automatic code-compliance claims appear. |

## Documentation

The setup artifact set is:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/*`

Future implementation artifacts are TBD and shall remain outside this setup folder unless dispatched by a separate sealed brief.
