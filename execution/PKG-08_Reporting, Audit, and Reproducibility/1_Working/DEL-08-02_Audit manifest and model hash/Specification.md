# Specification: DEL-08-02 Audit manifest and model hash

## Scope

This deliverable defines the setup specification for the audit manifest and model-hash feature slice. The future implementation shall capture reproducibility metadata needed to replay or professionally review an OpenPipeStress calculation package: model hash, input manifest, solver version stamp, rule-pack checksum, and referenced asset hashes.

This setup run does not implement hashing code, tests, schemas, source files, or a physical project container. Those remain future implementation work under bounded Type 2 briefs.

## Requirements

| ID | Requirement | Source |
|---|---|---|
| DEL-08-02-R1 | The audit manifest shall identify the exact model input payload used for a solve/report run. | `docs/_Registers/Deliverables.csv` row DEL-08-02; `docs/SPEC.md` section 8 |
| DEL-08-02-R2 | JSON payload hashes shall be based on canonical JSON with JCS-compatible canonicalization. | `docs/_Registers/ScopeLedger.csv` row SOW-039; `docs/_Decomposition/SOFTWARE_DECOMP.md` section 8.2 |
| DEL-08-02-R3 | Non-JSON and binary assets shall be represented through manifest asset hashes rather than folded into the JSON payload hash. | `docs/_Decomposition/SOFTWARE_DECOMP.md` section 8.2 |
| DEL-08-02-R4 | The manifest shall record software version and solver version sufficient to interpret deterministic result reproduction. | `docs/SPEC.md` sections 4.5 and 8 |
| DEL-08-02-R5 | The manifest shall record rule-pack name or ID, version, checksum, source notice, and redistribution status where a rule pack participates in a report/run. | `docs/SPEC.md` sections 6 and 8; `docs/IP_AND_DATA_BOUNDARY.md` section 7 |
| DEL-08-02-R6 | The manifest shall preserve unit-system and unit-aware model context so replay does not depend on hidden unit defaults. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/SPEC.md` section 8 |
| DEL-08-02-R7 | Missing provenance, version, checksum, or required manifest inputs shall be surfaced as warnings/findings, not silently defaulted. | `docs/DIRECTIVE.md` sections 2.2 and 3; `docs/CONTRACT.md` OPS-K-DATA-2 |
| DEL-08-02-R8 | Public manifest/report templates shall not embed protected standards text, protected tables, proprietary formulas, private rule-pack payloads, or private project data. | `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-IP-3, OPS-K-PRIV-1; `docs/IP_AND_DATA_BOUNDARY.md` sections 3, 6, and 7 |
| DEL-08-02-R9 | The manifest shall distinguish mechanics solved, user-rule checked, and human/professional acceptance states; it shall not claim code compliance or professional approval. | `docs/TYPES.md` sections 4 and 8; `docs/CONTRACT.md` OPS-K-AUTH-1 |
| DEL-08-02-R10 | Hash and manifest tests shall verify deterministic stability, checksum changes on material input changes, binary asset manifest handling, and protected/private data exclusion behavior. | `docs/VALIDATION_STRATEGY.md` sections 2 and 4; `docs/SPEC.md` section 9 |

## Standards

| Standard or basis | Applicability | Status |
|---|---|---|
| Canonical JSON / JCS-compatible canonicalization | JSON payload hash basis for model and manifest payloads. | Required by decomposition/register basis; exact library/API TBD. |
| JSON Schema 2020-12 | Public schema/interchange baseline for model/report-related contracts. | Architecture basis; schema files are outside this setup write scope. |
| OpenPipeStress invariant catalog | Legal/data/professional boundary for public artifacts. | Binding project governance draft. |

No protected engineering code, standard clause text, commercial example, or proprietary rule content is used as an authority in this setup artifact.

## Verification

Future implementation acceptance should include the following checks:

| Verification ID | Check | Expected result |
|---|---|---|
| V-1 | Hash the same canonical JSON model payload twice. | Identical digest and manifest metadata. |
| V-2 | Change a semantically relevant model input. | Model hash changes and the manifest identifies the changed payload basis. |
| V-3 | Reorder JSON object keys without changing data. | Model hash remains stable under canonicalization. |
| V-4 | Add or alter a binary/non-JSON asset entry. | Asset manifest digest changes without redefining the model JSON hash basis. |
| V-5 | Omit rule-pack checksum or source notice. | Manifest emits a missing-data/provenance finding. |
| V-6 | Include a private or protected payload candidate in a public template path. | Protected/private data guardrail blocks or flags the inclusion. |
| V-7 | Generate a report from a manifest-backed run. | Report includes version, manifest, hash/checksum, warnings, assumptions, and professional-boundary notice. |

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

Future implementation artifacts anticipated by the register are `audit manifest` and `hash tests`; they are not created in this setup session.
