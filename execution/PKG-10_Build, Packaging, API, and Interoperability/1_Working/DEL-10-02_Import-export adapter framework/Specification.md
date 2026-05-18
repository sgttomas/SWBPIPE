# Specification: DEL-10-02 Import/export adapter framework

## Scope

This specification covers the setup definition for the future import/export adapter framework under `DEL-10-02`. It establishes adapter-interface obligations, validation hooks, governance boundaries, and acceptance checks. It does not implement adapter source code, sample adapters, tests, external format support, package manifests, public API transport, or repo-level artifacts.

Concrete import/export formats remain `TBD`. Protected standards data, proprietary commercial formats, copied code tables, proprietary examples, and private user libraries are not bundled defaults.

## Requirements

| ID | Requirement | Source | Verification |
|---|---|---|---|
| REQ-10-02-01 | The adapter framework shall operate through schema-first command/query/job result envelopes and shall not bypass the public API/plugin boundary. | docs/_Decomposition/SOFTWARE_DECOMP.md AB-00-03 and AB-00-07 | Review adapter-interface design before implementation. |
| REQ-10-02-02 | Every import path shall validate units before accepting external data into domain workflows. | docs/PRD.md section 6.6; docs/CONTRACT.md OPS-K-UNIT-1 | Unit-validation test plan in future implementation. |
| REQ-10-02-03 | Every import path shall capture source/provenance, license or redistribution status, and review disposition for data records that may be contributed publicly or reused. | docs/CONTRACT.md OPS-K-IP-2; docs/IP_AND_DATA_BOUNDARY.md section 4 | Provenance-field checks in future implementation. |
| REQ-10-02-04 | Imported records with missing required fields, inconsistent units, missing provenance, unclear redistribution status, suspected protected origin, or out-of-range user checks shall produce diagnostics rather than silent defaults. | docs/PRD.md section 13.5; docs/CONTRACT.md OPS-K-DATA-2 | Diagnostic coverage review. |
| REQ-10-02-05 | Export paths shall check private/public data boundary status before writing payloads that may leave the local project context. | docs/PRD.md section 18.3; docs/CONTRACT.md OPS-K-PRIV-1 | Export-boundary checklist in future implementation. |
| REQ-10-02-06 | Adapter diagnostics shall include code, class, severity, source, affected object, message, remediation, and provenance when applicable. | docs/_Decomposition/SOFTWARE_DECOMP.md AB-00-06 | Result-envelope schema review. |
| REQ-10-02-07 | Adapter hooks for rule packs shall preserve rule-pack sandboxing, unit awareness, versioning, checksums, and public/private status. | docs/CONTRACT.md OPS-K-RULE-1 through OPS-K-RULE-3; docs/_Decomposition/SOFTWARE_DECOMP.md AB-00-07 | Rule-hook boundary review. |
| REQ-10-02-08 | Adapter hooks for reports shall preserve report warnings, assumptions, provenance, limitations, protected-content controls, and professional-responsibility notices. | docs/CONTRACT.md OPS-K-REPORT-1, OPS-K-REPORT-2, OPS-K-AUTH-1 | Report-boundary review. |
| REQ-10-02-09 | The framework shall not choose protected external formats, proprietary tool behavior, or specific commercial integration defaults without human approval and documented redistribution rights. | docs/IP_AND_DATA_BOUNDARY.md; docs/_Decomposition/SOFTWARE_DECOMP.md OI-004 and DEC-012 | Human decision log required before format selection. |
| REQ-10-02-10 | The framework shall leave mechanics solve, user-rule check, and professional approval states distinct in all adapter result payloads. | docs/TYPES.md section 4; docs/CONTRACT.md OPS-K-AUTH-1 | Status-field review in future interface artifacts. |

## Standards

No external engineering code or proprietary format standard is incorporated by this setup artifact. Applicable project standards are internal governance invariants:

- OPS-K-IP-1, OPS-K-IP-2, OPS-K-IP-3
- OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-DATA-3
- OPS-K-UNIT-1
- OPS-K-RULE-1, OPS-K-RULE-2, OPS-K-RULE-3
- OPS-K-PRIV-1, OPS-K-PRIV-2
- OPS-K-AUTH-1
- OPS-K-AGENT-1 through OPS-K-AGENT-4

## Verification

Future implementation verification should include:

- schema validation for adapter request/result envelopes;
- unit conversion and dimensional-consistency tests;
- provenance and redistribution-status rejection/flagging tests;
- private-data export-warning tests;
- protected-content/provenance gates for public examples and fixtures;
- diagnostics envelope tests for import, export, and failed validation paths;
- rule-pack hook tests proving adapters cannot execute arbitrary rule code or bypass evaluator sandboxing;
- report-boundary tests proving exports do not suppress warnings, limitations, or professional-responsibility notices.

## Documentation

Required future artifacts remain:

- adapter interfaces;
- sample invented adapter using invented, non-code, non-proprietary data only.

This setup run does not create those implementation artifacts.
