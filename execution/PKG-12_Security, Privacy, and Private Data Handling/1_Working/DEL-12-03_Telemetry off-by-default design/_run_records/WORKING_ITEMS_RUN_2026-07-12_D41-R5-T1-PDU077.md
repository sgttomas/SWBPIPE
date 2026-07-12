# WORKING_ITEMS Run Record - D41-R5-T1-PDU077

Date: 2026-07-12

Persona: WORKING_ITEMS

Deliverable: DEL-12-03 - Telemetry off-by-default design

## Authority

- D-41 R4 ruling / DEC-074, owner/product option O3 A.
- `PROPOSED_DELIVERABLE_UPDATES.csv` PDU-077.
- `R5_TRANCHE_PLAN.md` T1 ownership and attribution tranche.
- Independent receiving-scope audit: PASS for
  `apps/desktop/src/features/telemetry` against DEL-12-03.

## Scope clarification

`apps/desktop/src/features/telemetry/TelemetryBoundaryPanel.tsx` is DEL-12-03
implementation evidence for the default-off policy-review boundary. It builds
a local review artifact for display/download, not a telemetry payload. Its
recorded state keeps `payload_constructed=false`, network/runtime
initialization false, and config/consent/allowlist decisions `TBD`.

The panel is not a consent UI and does not authorize telemetry collection,
payload construction, endpoint/vendor selection, persistence, upload,
transport, or runtime integration.

## Changes

- Added the panel to the four-document kit's bounded current evidence.
- Preserved the policy/helper/test evidence and all deferred runtime,
  configuration, consent, allowlist, retention, and support-workflow choices.
- Appended `MEMORY.md`, `_STATUS.md` history, and this run record. Existing
  `_STATUS.md ## Remaining` content, including the D-41 bootstrap, is
  unchanged.

## Validation

- Four-document presence check.
- Dependency-register schema validation (read-only; `Dependencies.csv` was
  not changed).
- Read-only source assertions over `TelemetryBoundaryPanel.tsx`: DEL-12-03
  identity, `payload_constructed=false`, network/runtime initialization false,
  consent/config/allowlist `TBD`, and non-authorizing boundary text.
- Touched-scope whitespace and diff review.

## Boundaries

No application code, telemetry payload/runtime behavior, schema, test,
dependency/DAG/register surface, lifecycle state, scope expansion,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.
