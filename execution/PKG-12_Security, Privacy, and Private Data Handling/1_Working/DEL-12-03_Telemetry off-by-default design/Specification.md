# Specification: DEL-12-03 Telemetry off-by-default design

## Scope

This deliverable specifies the privacy and configuration design for telemetry in OpenPipeStress. It covers policy, default configuration behavior, and tests needed to prove that telemetry is disabled by default and cannot transmit private engineering, code, rule-pack, component, material, report, secret, path, or protected standards data.

Current evidence includes the repo policy in `docs/security/telemetry_policy.md`, a metadata-only guard helper in `core/security/telemetry_policy/`, focused tests in `tests/security/test_telemetry_policy.py`, and the default-off policy-review panel in `apps/desktop/src/features/telemetry/TelemetryBoundaryPanel.tsx`. DEC-074 O3 attributes that panel to DEL-12-03 as implementation evidence. The panel constructs a local telemetry-boundary review artifact, not a telemetry payload: it records `payload_constructed=false`, initializes no network behavior, and leaves product configuration, consent surface, and allowlist approval `TBD`. This evidence does not define a telemetry vendor, create cloud service behavior, choose an endpoint, add transport/upload/persistence behavior, select or authorize a consent UI or CLI, approve telemetry collection, or change lifecycle state. Any runtime telemetry implementation remains blocked on explicit human/security approval.

## Requirements

| ID | Requirement | Verification |
|---|---|---|
| TEL-REQ-001 | Telemetry shall be disabled by default. An absent, unset, empty, unknown, unsupported, malformed, or incomplete telemetry setting shall resolve to disabled. | Covered for metadata input by `tests/security/test_telemetry_policy.py`; product config schema/storage remains `TBD`. |
| TEL-REQ-002 | Telemetry shall not initialize network transport, background upload jobs, queues, or persistence for telemetry unless the user has explicitly opted in. | Metadata helper exposes no network/payload initialization and tests assert no network, endpoint, vendor, upload queue/job, persistence, or external client flags; runtime startup remains `TBD`. |
| TEL-REQ-003 | Telemetry, if later implemented, shall require explicit human/security approval of the event allowlist before any event is collected. | Helper requires allowlist approval metadata before allowing a metadata event; actual approval records and event allowlist remain `TBD`. |
| TEL-REQ-004 | Telemetry shall not collect or transmit private project models, code-specific rule data, private rule packs, private material or component libraries, generated reports, model hashes, local file paths, user secrets, credentials, or protected standards content. | Helper rejects forbidden field classes before payload construction; protected-content/privacy scan evidence is recorded in the June 7 package fan-in. |
| TEL-REQ-005 | Opt-in state shall be represented as a local configuration value whose default is false. The exact storage location and schema are TBD until the implementation deliverable chooses the product config surface. | Metadata resolution tests cover fail-closed behavior; product config-default fixture remains `TBD` until schema exists. |
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
| `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-037 and OI-008 | Telemetry, if implemented, is opt-in and privacy preserving; MVP default is no telemetry; any telemetry design requires explicit human approval. |

## Verification

Current metadata-guard evidence plus minimum test coverage for any later implementation:

| Test ID | Test intent | Expected result |
|---|---|---|
| TEL-TEST-001 | Resolve absent, empty, unknown, unsupported, malformed, or incomplete telemetry metadata. | Current helper tests show telemetry resolves disabled and initializes no transport, endpoint, vendor, upload queue/job, persistence, or external client. |
| TEL-TEST-002 | Load malformed product config once product config schema exists. | Telemetry remains disabled and a validation finding is emitted. This is still `TBD` for product config storage/schema. |
| TEL-TEST-003 | Attempt to emit telemetry before opt-in or without approved allowlist evidence. | Current helper tests show event attempts are blocked locally before payload construction. Runtime network-denial/startup tests remain future scope. |
| TEL-TEST-004 | Attempt to include private model, rule-pack, material, component, report, path, hash, secret, protected, or professional-claim fields. | Current helper tests reject the field before payload construction. |
| TEL-TEST-005 | Enable opt-in using an explicit approved config path. | Only approved allowlist fields are eligible for metadata-only allowance; actual consent surface and approval record remain `TBD`. |
| TEL-TEST-006 | Exercise plugin, adapter, report, and private-library routes. | No route bypasses telemetry opt-in or payload filtering. Runtime route coverage remains future scope until those surfaces call the helper. |
| TEL-TEST-007 | Scan telemetry messages and docs. | No certification, approval, seal, security-certification, or automatic code-compliance claims appear. |

## Current Evidence

| Evidence | Result |
|---|---|
| Worker B run record | `_run_records/TASK_RUN_2026-06-07_0141.md` records successful metadata-only helper implementation, 15 focused telemetry tests passing, and `git diff --check` passing for the worker write set. |
| Package fan-in | `WORKING_ITEMS_RUN_2026-06-07_0150_TP-PKG12-LOCAL-PRIVACY-GUARDS-FANIN.md` records 44 passing PKG-12 privacy-guard tests and no runtime telemetry endpoint, vendor, transport, upload queue/job, persistence, schema, or approval artifact introduced. |
| Policy document | `docs/security/telemetry_policy.md` documents default-off semantics, forbidden payload classes, the metadata-only helper boundary, and open decisions. |
| Desktop policy-review panel | `apps/desktop/src/features/telemetry/TelemetryBoundaryPanel.tsx` presents and locally exports default-off boundary state for review. It is DEL-12-03 implementation evidence under DEC-074 O3, but constructs no telemetry payload, initializes no network/runtime behavior, and keeps config/consent/allowlist decisions `TBD`. |

## Documentation

The deliverable-local artifact set remains:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/*`

Current external evidence artifacts are `docs/security/telemetry_policy.md`, `core/security/telemetry_policy/`, `tests/security/test_telemetry_policy.py`, and `apps/desktop/src/features/telemetry/TelemetryBoundaryPanel.tsx`. Future runtime telemetry, product config schema/storage, consent UI/CLI, endpoint/vendor/transport, retention, event allowlist/schema, and support-bundle implementation artifacts remain `TBD` and shall remain outside this setup folder unless dispatched by a separate sealed brief.
