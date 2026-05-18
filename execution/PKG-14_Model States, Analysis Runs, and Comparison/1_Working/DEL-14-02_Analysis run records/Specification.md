# Specification: DEL-14-02 Analysis run records

## Scope

DEL-14-02 defines the analysis-run record data model and the supporting reproducibility test surface for saving analysis runs as immutable, traceable product records.

In scope:

- `schemas/analysis_run.schema.json`.
- Run reproducibility tests.
- Data-model coverage for the SOW-072 binding categories: exact model states, solver versions, settings, units, load cases, diagnostics, results, rule-pack references, library references, and result hashes.
- Professional-boundary, unit, hash, provenance, protected-content, and private-data constraints that apply to persisted run records.

Out of scope:

- Comprehensive commercial prover output ingestion.
- External validation or professional acceptance decisions.
- Exact implementation code, storage container choice, dependency versions, solver numerical library, rule expression grammar/library, and public API transport.
- Field-level schema design beyond source-supported categories; unsupported details remain TBD.

## Requirements

| ID | Requirement | Source | Verification |
|---|---|---|---|
| DEL-14-02-R001 | The deliverable shall produce or define `schemas/analysis_run.schema.json` as the analysis-run record schema artifact. | `_CONTEXT.md` anticipated artifacts; `docs/_Registers/Deliverables.csv` row DEL-14-02 | Artifact exists and is schema-validated once implementation occurs. |
| DEL-14-02-R002 | The analysis-run record shall bind each saved run to an exact model state reference. | SOW-072 in `docs/_Registers/ScopeLedger.csv`; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-14-02 row | Schema contains a required model-state reference or equivalent validated binding. |
| DEL-14-02-R003 | The analysis-run record shall preserve solver version, solver/settings basis, units, load-case basis, diagnostics, results, rule-pack references, library references, and result hashes. | SOW-072; `_CONTEXT.md` scope detail | Schema inspection and reproducibility tests confirm the categories are present or explicitly referenced. |
| DEL-14-02-R004 | Results shall attach to immutable analysis-run records rather than only to mutable current model state. | `docs/_Registers/ScopeLedger.csv` row SOW-072; `_CONTEXT.md` envelope notes | Mutation/round-trip test demonstrates stored run evidence remains associated with the original run basis. |
| DEL-14-02-R005 | Unit-bearing physical values included or referenced by run records shall carry explicit unit metadata unless explicitly classified as dimensionless, ratio, percentage, or coefficient. | `docs/SPEC.md` section 4; `docs/CONTRACT.md` OPS-K-UNIT-1 | Schema validation and negative tests reject or diagnose missing unit metadata where required. |
| DEL-14-02-R006 | Missing solve-required or rule-check-required values shall be explicit findings with diagnostics and provenance, not silent defaults. | `docs/SPEC.md` analysis boundary section; `docs/CONTRACT.md` OPS-K-DATA-2 | Tests confirm missing required values produce structured diagnostics. |
| DEL-14-02-R007 | Hash records shall identify payload scope and use the accepted JSON/JCS-compatible canonical JSON basis where JSON payloads are hashed. | `_CONTEXT.md` architecture basis; `docs/SPEC.md` section 4.4 | Reproducibility tests compare canonical hash outputs for stable JSON payloads. |
| DEL-14-02-R008 | The record shall preserve the software authority boundary: solver results and diagnostics are software outputs; human acceptance is external and hash-bound. | `docs/SPEC.md` analysis boundary section; `docs/CONTRACT.md` OPS-K-AUTH-1 and OPS-K-AUTH-2 | Schema/status tests ensure automatic approval, certification, sealing, authentication, or code-compliance labels are not emitted. |
| DEL-14-02-R009 | Rule-pack and library references shall expose identity/provenance/checksum-style metadata where source-supported, without embedding protected standards text, protected tables, proprietary values, private formulas, or private payloads in public artifacts. | SOW-072; `docs/SPEC.md` result export section; `docs/IP_AND_DATA_BOUNDARY.md` | Protected-content/private-data tests inspect examples and fixtures before acceptance. |
| DEL-14-02-R010 | Run reproducibility tests shall be part of the deliverable output surface. | `_CONTEXT.md` anticipated artifacts | Test files exist and exercise stable run-record/hash behavior once implementation occurs. |
| DEL-14-02-R011 | The schema and tests shall remain compatible with schema-first command/query/job result-envelope architecture. | `_CONTEXT.md` architecture basis; `docs/SPEC.md` runner/result-envelope sections | Service/result-envelope integration tests or contract tests are added when implementation scope reaches that boundary. |

## Standards

| Standard or governing basis | Status in this folder | Notes |
|---|---|---|
| JSON Schema 2020-12 contracts | Applicable architecture basis | `_CONTEXT.md` identifies JSON Schema 2020-12 contracts. Exact `$schema`, `$id`, and schema-module placement remain implementation TBD. |
| Canonical JSON / JCS-compatible hash basis | Applicable where JSON payloads are hashed | `_CONTEXT.md` and `docs/SPEC.md` support the basis. Exact library/dependency choice remains TBD. |
| PRD v0.2 sections 8.7 and 15.2 / FR-CMP-002 | Referenced but not locally accessible as primary source text | Decomposition and registers cite these references. Do not derive clause-level requirements beyond the accessible SOW-072 wording. |
| Protected-content and private-data governance | Applicable | `docs/CONTRACT.md` and `docs/IP_AND_DATA_BOUNDARY.md` govern public/private data handling. |
| Professional responsibility boundary | Applicable | `docs/CONTRACT.md` and `docs/SPEC.md` prohibit software-generated professional approval/compliance claims. |

## Verification

| Verification target | Method | Acceptance signal |
|---|---|---|
| Schema artifact | JSON Schema validation and repository schema gate | `schemas/analysis_run.schema.json` parses and satisfies local schema conventions once implemented. |
| Required binding categories | Schema review plus tests | Model state, solver version/settings, units, load-case basis, diagnostics, results, rule-pack/library references, and result hashes are represented or explicitly referenced. |
| Immutability of run evidence | Round-trip/persistence test | A saved run remains bound to the original model-state/run basis after unrelated model changes. |
| Hash reproducibility | Deterministic canonicalization test | Equivalent canonical JSON payloads produce stable hashes; payload scope is recorded. |
| Unit safety | Negative and positive unit tests | Unit-bearing values include unit metadata or produce blocking diagnostics. |
| Professional boundary | Status/label tests | No automatic human approval, certification, sealing, authentication, or code-compliance label is emitted. |
| Protected/private data boundary | Protected-content and fixture review | Public examples do not embed private/protected rule, library, standards, or proprietary payload content. |

## Documentation

Required or expected records for later implementation:

- `schemas/analysis_run.schema.json`.
- Run reproducibility tests.
- Schema-source notes for each binding category.
- Hash/reproducibility test notes, including payload scope.
- Unit and diagnostic test notes.
- Protected-content/private-data review notes for any public examples or fixtures.

All exact field names, object identities, schema `$id` values, migration behavior, and fixture values are TBD until supported by implementation work or human-approved design decisions.
