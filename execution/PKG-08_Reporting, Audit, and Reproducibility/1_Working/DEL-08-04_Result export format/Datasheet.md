# Datasheet: DEL-08-04 Result export format

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-08-04 |
| Deliverable name | Result export format |
| Package ID | PKG-08 |
| Package name | Reporting, Audit, and Reproducibility |
| Deliverable type | API_CONTRACT |
| Scope item | SOW-046 |
| Supported objectives | OBJ-007; OBJ-009 |
| Setup status | Draft setup artifact; not implementation |

## Attributes

| Attribute | Draft setup value |
|---|---|
| Primary artifact family | Machine-readable result export contract for review, regression comparison, and downstream tooling. |
| Baseline format | Schema-first JSON result envelopes. Source: `docs/_Registers/ScopeLedger.csv` row SOW-046; `_CONTEXT.md` Architecture Basis Injection. |
| Additional export formats | TBD; this setup deliverable does not choose CSV, HDF5, neutral-file, spreadsheet, or external tool formats as final. |
| Anticipated implementation artifacts | `schemas/results.schema.yaml`, exporter source, and tests. Source: `_CONTEXT.md` Anticipated Artifacts; not created or edited in this setup session. |
| Envelope content categories | Result identity, model/run references, unit-aware value arrays, diagnostics, provenance, analysis status, warnings, and reproducibility references. Source: `docs/SPEC.md` sections 4.5, 7, 8, and 9; `docs/_Decomposition/SOFTWARE_DECOMP.md` architecture basis rows AB-00-03, AB-00-04, AB-00-06, and AB-00-07. |
| Review boundary | Exports support review and comparison; they do not certify, seal, approve, authenticate, or declare code compliance. Source: `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/TYPES.md` section 4. |
| Unit boundary | Exported values must be unit-aware and dimensionally traceable; missing units are findings, not silent defaults. Source: `docs/CONTRACT.md` OPS-K-UNIT-1 and OPS-K-DATA-2. |
| Protected-data boundary | Public export contracts must not embed protected standards text, copied standards tables, proprietary formulas, private rule-pack payloads, or private project data by default. Source: `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-IP-3, OPS-K-PRIV-1; `docs/IP_AND_DATA_BOUNDARY.md` sections 3, 6, and 7. |

## Conditions

- This session is setup/document production only; no schema file, exporter source, tests, docs outside this folder, or repository-level artifacts are modified.
- The export baseline is a JSON result envelope contract. Any additional export format remains `TBD` until a later bounded decision or implementation brief.
- Result exports must preserve the PKG-00 no-bypass baseline: adapters and downstream tools cannot bypass unit checks, diagnostics, provenance, public/private data boundaries, or professional-responsibility notices.
- Mechanics results, user-rule-check results, and human review/approval states must remain distinguishable in exported data.
- Exported diagnostics must carry enough structured information for review and regression triage without relying on prose-only warnings.

## Construction

The future result export contract is expected to include these setup-level components:

| Component | Purpose | Boundary |
|---|---|---|
| Result envelope header | Identifies export schema version, result set ID, model/run reference, solver version reference, and creation context. | Does not imply validation, certification, or professional acceptance. |
| Unit-aware result payloads | Carries displacement, rotation, force, moment, reaction, stress, ratio, or rule-check result values with unit metadata and dimensional categories. | No hidden unit defaults; code-specific values remain user/rule-pack supplied. |
| Diagnostics block | Carries structured diagnostics with code, class, severity, source, affected object, message, remediation, and provenance. | Diagnostics must not be dropped by exporters or adapters. |
| Provenance and reproducibility references | Links to model hash, manifest, rule-pack checksum, source/provenance notes, and input/run identifiers where available. | References private data safely; does not copy protected or private payloads into public artifacts. |
| Analysis status fields | Distinguishes `MECHANICS_SOLVED`, `RULE_INPUTS_INCOMPLETE`, `USER_RULE_CHECKED`, `HUMAN_REVIEW_REQUIRED`, and related statuses. | Must not emit automatic `CODE_COMPLIANT` or professional approval status. |
| Regression comparison surface | Provides stable identifiers, deterministic ordering, and numeric/unit metadata suitable for comparison tests. | Exact comparison tolerances and tooling remain future validation work. |
| Downstream tooling handoff | Provides a stable machine-readable contract for headless CLI, report generation, GUI results viewer, and adapters. | External transport and concrete non-JSON formats remain `TBD`. |

## References

- `_CONTEXT.md` for deliverable identity, architecture basis IDs, and write-scope constraints.
- `docs/_Registers/Deliverables.csv` row DEL-08-04 for artifact and objective mapping.
- `docs/_Registers/ScopeLedger.csv` row SOW-046 for export baseline acceptance notes.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` rows AB-00-03, AB-00-04, AB-00-06, AB-00-07, and OI-004 for envelope/API/no-bypass and format-TBD constraints.
- `docs/SPEC.md` sections 4.5, 7, 8, 9, and 11 for deterministic results, diagnostics, reporting, validation, and acceptance semantics.
- `docs/TYPES.md` sections 4 and 8 for analysis-status vocabulary and `Result` object boundary.
- `docs/IP_AND_DATA_BOUNDARY.md` sections 3, 6, and 7 for public/private data and report/export boundaries.
- `docs/CONTRACT.md` for OPS-K-IP, OPS-K-DATA, OPS-K-UNIT, OPS-K-RULE, OPS-K-PRIV, OPS-K-AUTH, and OPS-K-AGENT invariants.
