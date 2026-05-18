# Specification: DEL-08-04 Result export format

## Scope

This deliverable defines the setup specification for a machine-readable result export contract. The baseline is a schema-first JSON result envelope suitable for review, regression comparison, and downstream tooling.

This setup run does not implement exporter code, edit `schemas/results.schema.yaml`, create tests, choose additional export formats as final, or modify documentation outside this deliverable folder. Those remain future implementation work under bounded Type 2 briefs.

## Requirements

| ID | Requirement | Source |
|---|---|---|
| DEL-08-04-R1 | The result export baseline shall be a schema-first JSON result envelope. | `docs/_Registers/ScopeLedger.csv` row SOW-046; `_CONTEXT.md` Architecture Basis Injection |
| DEL-08-04-R2 | Additional concrete export formats shall remain `TBD` unless later approved and scoped; this deliverable shall not finalize CSV, spreadsheet, FEA, HDF5, or other external formats. | `docs/_Decomposition/SOFTWARE_DECOMP.md` OI-004; sealed brief acceptance notes |
| DEL-08-04-R3 | Result envelopes shall preserve units and dimensional metadata for exported values and shall not rely on hidden unit defaults. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/SPEC.md` section 8 |
| DEL-08-04-R4 | Result envelopes shall carry or reference provenance and reproducibility metadata, including model/run identity, solver version basis, rule-pack checksum where applicable, warnings, assumptions, and source notes. | `docs/SPEC.md` sections 4.5 and 8; `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04 |
| DEL-08-04-R5 | Result envelopes shall carry structured diagnostics using the PKG-00 baseline fields: code, class, severity, source, affected object, message, remediation, and provenance. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06; `docs/SPEC.md` section 7 |
| DEL-08-04-R6 | Exported statuses shall distinguish mechanics solved, user-rule checked, rule-input incomplete, human-review required, and human-approved record states where present. | `docs/TYPES.md` sections 4 and 8; `docs/CONTRACT.md` OPS-K-AUTH-1 |
| DEL-08-04-R7 | Result exports shall not claim certification, sealing, professional approval, authentication, or automatic engineering code compliance. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/TYPES.md` section 4 |
| DEL-08-04-R8 | Result exports and public examples shall not embed protected standards text, copied standards tables, proprietary formulas, private rule-pack payloads, or private project data by default. | `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-IP-3, OPS-K-PRIV-1; `docs/IP_AND_DATA_BOUNDARY.md` sections 3, 6, and 7 |
| DEL-08-04-R9 | Exporters and adapters shall not bypass validation, unit checks, diagnostics, provenance handling, report controls, or public/private data-boundary checks. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-07; `docs/SPEC.md` section 1 |
| DEL-08-04-R10 | Export ordering and identifiers should be deterministic enough for regression comparison of equivalent result sets. | `docs/SPEC.md` sections 4.5 and 9; `docs/_Registers/ScopeLedger.csv` row SOW-046 |
| DEL-08-04-R11 | Missing solve-required, rule-check-required, provenance, or unit metadata shall be represented as diagnostics/findings, not silently filled with defaults. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` section 7 |
| DEL-08-04-R12 | The contract shall support downstream consumption by report generation, GUI results review, headless automation, and import/export adapters without creating a bypass around governed result envelopes. | `docs/_Registers/Deliverables.csv` rows DEL-07-05, DEL-08-01, DEL-10-02, DEL-10-05; `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-03 and AB-00-07 |

## Standards

| Standard or basis | Applicability | Status |
|---|---|---|
| JSON Schema 2020-12 | Baseline schema technology for public result envelope contracts. | Architecture basis; schema file editing is outside this setup write scope. |
| Schema-first command/query/job result envelope baseline | Result export/API boundary and no-bypass contract. | Required by PKG-00 architecture basis referenced in `_CONTEXT.md`. |
| Canonical JSON / JCS-compatible canonicalization | Relevant when exported JSON payloads or manifests are hashed for reproducibility. | Hash implementation belongs primarily to DEL-08-02 / future implementation; this deliverable shall preserve compatible references. |
| OpenPipeStress invariant catalog | IP, data, units, rule-pack, privacy, professional-responsibility, and agent-boundary constraints. | Binding project governance draft. |

No protected engineering code, standard clause text, standards table, commercial example, or proprietary rule content is used as an authority in this setup artifact.

## Verification

Future implementation acceptance should include the following checks:

| Verification ID | Check | Expected result |
|---|---|---|
| V-1 | Validate a representative JSON result envelope against the result schema. | Schema validation passes and required envelope fields are present. |
| V-2 | Export a result set with unit-bearing displacements, rotations, forces, moments, reactions, stresses, and ratios. | Each value family carries explicit unit/dimensional metadata or a blocking diagnostic. |
| V-3 | Export a run with missing rule-pack input or missing provenance. | Export includes structured diagnostics/findings instead of silent defaults. |
| V-4 | Export a run with rule-pack references. | Envelope records rule-pack identity/version/checksum/source status without copying private formulas or protected values. |
| V-5 | Re-export an equivalent result set. | Stable identifiers and ordering allow deterministic regression comparison. |
| V-6 | Route the export through an adapter or downstream tool. | Unit, provenance, diagnostics, and professional-boundary fields are preserved. |
| V-7 | Scan public fixtures/templates for protected content and approval/compliance claims. | Protected/private data and certification/compliance claims are absent or blocked. |
| V-8 | Generate a report or GUI result view from the export. | Consumer can distinguish mechanics, user-rule, diagnostics, and human-review-required statuses. |

## Documentation

This setup deliverable produces:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/*`
- `_STATUS.md`

Future implementation artifacts anticipated by the register are `schemas/results.schema.yaml`, exporter source, and tests; they are not created or edited in this setup session.
