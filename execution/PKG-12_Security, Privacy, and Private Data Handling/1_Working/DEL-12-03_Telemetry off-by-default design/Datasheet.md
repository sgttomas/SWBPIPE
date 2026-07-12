# Datasheet: DEL-12-03 Telemetry off-by-default design

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-12-03 |
| Deliverable name | Telemetry off-by-default design |
| Package ID | PKG-12 |
| Package name | Security, Privacy, and Private Data Handling |
| Deliverable type | SECURITY_CONTROL |
| Scope item | SOW-037 |
| Objective | OBJ-010 |
| Context envelope | S |
| Current local lifecycle state | IN_PROGRESS; this evidence alignment does not change `_STATUS.md` or promote lifecycle. |

## Attributes

| Attribute | Value | Source |
|---|---|---|
| MVP telemetry posture | No telemetry by default; telemetry may be a no-op in MVP. | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-037 and OI-008 |
| User consent posture | Any telemetry, if later implemented, is opt-in only and requires explicit human approval before design activation. | `execution/_Decomposition/SOFTWARE_DECOMP.md` OI-008; `docs/CONTRACT.md` OPS-K-PRIV-2 |
| Forbidden telemetry content | Private project data, code-specific data, private rule-pack data, private material/component data, secrets, paths, report content, and protected standards content. | `docs/CONTRACT.md` OPS-K-IP-1/2/3, OPS-K-DATA-1/2/3, OPS-K-PRIV-1/2 |
| Local-first boundary | No cloud operation is included unless separately authorized. | `_CONTEXT.md`; `docs/DIRECTIVE.md` Section 4.2 |
| Current evidence artifacts | Telemetry policy, metadata-only telemetry guard helper, focused policy/helper tests, and a desktop default-off policy-review panel now exist. The panel is DEL-12-03 implementation evidence under DEC-074 O3; it constructs no telemetry payload or network behavior and does not select consent/config/allowlist policy. Runtime telemetry, product config schema/storage, consent surface, endpoint/vendor/transport, event schema/allowlist, retention, and support-bundle workflow remain deferred. | `_CONTEXT.md`; `docs/security/telemetry_policy.md`; `core/security/telemetry_policy/`; `tests/security/test_telemetry_policy.py`; `apps/desktop/src/features/telemetry/TelemetryBoundaryPanel.tsx`; `_run_records/TASK_RUN_2026-06-07_0141.md` |

## Conditions

| Condition | Required handling |
|---|---|
| Configuration key is absent | Treat telemetry as disabled. |
| Configuration value is malformed | Treat telemetry as disabled and emit a diagnostic or validation finding. |
| User has not explicitly opted in | Do not initialize telemetry transport, collection, background jobs, or upload queues. |
| Human approval for runtime telemetry collection is absent | Keep runtime telemetry as no-op/default-off and record telemetry details as TBD. |
| A payload field could expose private or protected data | Exclude the field and record an IP/privacy boundary finding. |

## Construction

This deliverable began as a design-boundary setup kit. Current evidence also includes a metadata-only guard helper, focused tests, and a desktop policy-review panel that harden and expose the default-off boundary before any telemetry payload construction. The panel's local JSON is a review artifact, not a telemetry payload. Neither the helper nor the panel is a runtime telemetry system or authorization for endpoint, vendor, transport, queue, upload job, persistence, retention, consent UI/CLI, support-bundle workflow, or professional/code-compliance authority.

| Artifact | Construction intent | Current evidence result |
|---|---|---|
| Telemetry policy | State the default-off, opt-in, privacy-preserving rule set. | Captured in this kit and implemented as repo policy in `docs/security/telemetry_policy.md`. |
| Config defaults | Define default-disabled behavior for absent, empty, unknown, unsupported, malformed, or incomplete telemetry metadata. | Implemented in `core/security/telemetry_policy/` as metadata-only config resolution; exact product config schema and storage location remain `TBD`. |
| Tests | Prove default-off, fail-closed, allowlist-gated, no-payload, and no-network-default behavior for the metadata-only guard. | Covered by `tests/security/test_telemetry_policy.py`; Worker B reported 15 passing tests and parent fan-in reported 44 passing PKG-12 privacy-guard tests. |

## June 7 Readiness Evidence

| Evidence | Current interpretation |
|---|---|
| `core/security/telemetry_policy/` | Provides metadata-only `TelemetryConfig`, `TelemetryEventAttempt`, diagnostics, decision/result records, config resolution, and event guard behavior before payload construction. |
| `tests/security/test_telemetry_policy.py` | Tests default-off config handling, opt-in/allowlist gates, rejected unknown events/fields, rejected private/protected/secret/path/hash/report/professional-claim field classes, and no payload or network initialization flags. |
| `docs/security/telemetry_policy.md` | Documents the policy and helper non-authority boundary while preserving open `TBD` decisions. |
| `apps/desktop/src/features/telemetry/TelemetryBoundaryPanel.tsx` | Presents and locally exports the default-off policy/guard boundary with `payload_constructed=false` and network/runtime initialization false; consent, config, and allowlist approval remain `TBD`. |
| `_run_records/TASK_RUN_2026-06-07_0141.md` and package fan-in | Record successful focused tests, `git diff --check`, and boundary scan evidence. |

## References

- `INIT.md` for project bootstrap and boundary rules.
- `AGENTS.md` for bounded Type 2 dispatch rules.
- `docs/CONTRACT.md` for OPS-K-IP, OPS-K-DATA, OPS-K-AUTH, OPS-K-PRIV, and OPS-K-AGENT invariants.
- `docs/DIRECTIVE.md` for founding boundaries and out-of-scope hidden cloud telemetry.
- `docs/IP_AND_DATA_BOUNDARY.md` for public/private data handling limits.
- `docs/SPEC.md` for architecture, diagnostics, reports, and acceptance semantics.
- `docs/security/telemetry_policy.md` for the implemented policy and metadata-only guard boundary.
- `core/security/telemetry_policy/` for metadata-only helper evidence.
- `tests/security/test_telemetry_policy.py` for focused default-off/helper tests.
- `apps/desktop/src/features/telemetry/TelemetryBoundaryPanel.tsx` for the DEL-12-03 default-off policy-review surface attributed by DEC-074 O3.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 for PKG-12, DEL-12-03, SOW-037, OBJ-010, OI-008, and AB-00-01/02/03/04/06/07/08.
- `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-03_Telemetry off-by-default design/_run_records/TASK_RUN_2026-06-07_0141.md`.
- `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/_run_records/WORKING_ITEMS_RUN_2026-06-07_0150_TP-PKG12-LOCAL-PRIVACY-GUARDS-FANIN.md`.
- `docs/_Registers/Deliverables.csv` row DEL-12-03.
- `docs/_Registers/ScopeLedger.csv` row SOW-037.
- `docs/_Registers/ContextBudgetQA.csv` row DEL-12-03.
