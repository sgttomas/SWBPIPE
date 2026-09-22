---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-14-02
package_id: PKG-14
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@00115c71931bcae79909602d653740d3bb72dfa1
project_scope_refs: [SOW-072]
package_objective_refs: [OBJ-016]
---

# Scope of Work — DEL-14-02

Reconciliation evidence: the retired D-41 current-declaration snapshots are preserved at source `00115c71931bcae79909602d653740d3bb72dfa1` and in `execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/`. Current scope and decisions are resolved through `execution/_Decomposition/SOFTWARE_DECOMP.md` and `execution/_Coordination/_DECISIONS/_REGISTER.md`; dependency context is resolved through `execution/_DAG/_LATEST.md`. Open work and lifecycle remain governed by `_STATUS.md`. This text repair establishes no lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

## Purpose and Objective Traceability

This Scope of Work defines `DEL-14-02` in service of project scope [SOW-072] and package objectives [OBJ-016].

- **OUT-001** — An immutable analysis-run record contract binding results to exact model states, solver versions and settings, units, load cases, diagnostics, references, results, and hashes is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-14-02 Analysis run records

> #### Datasheet: DEL-14-02 Analysis run records
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value | Source |
> |---|---|---|
> | Deliverable ID | DEL-14-02 | `_CONTEXT.md` |
> | Name | Analysis run records | `_CONTEXT.md` |
> | Package ID | PKG-14 | `_CONTEXT.md` |
> | Package name | Model States, Analysis Runs, and Comparison | `_CONTEXT.md` |
> | Deliverable type | DATA_MODEL_CHANGE | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` PKG-14 table |
> | Scope item | SOW-072 | `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv` row SOW-072 |
> | Objective support | OBJ-016 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` objective mapping |
> | Anticipated artifacts | `schemas/analysis_run.schema.json`; run reproducibility tests | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-14-02 |
> | Lifecycle role | Draft production-unit document; not implementation evidence | `_CONTEXT.md` PREPARATION notes; `docs/CONTRACT.md` OPS-K-AGENT-4 |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Value | Source |
> |---|---|---|
> | Required product behavior | Save analysis runs bound to exact model states, solver versions, settings, units, load cases, diagnostics, results, rule-pack references, library references, and result hashes. | `docs/_Registers/ScopeLedger.csv` row SOW-072; `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-072 |
> | Record immutability intent | Results attach to immutable analysis runs rather than a mutable current model only. | `docs/_Registers/ScopeLedger.csv` row SOW-072; `_CONTEXT.md` envelope notes |
> | Package boundary | PKG-14 implements immutable model-state records, analysis-run records, deterministic state/run comparison, mappings, tolerances, and comparison exports. | `execution/_Decomposition/SOFTWARE_DECOMP.md` package table |
> | Package exclusion | PKG-14 does not ingest commercial prover outputs comprehensively or determine external validation. | `execution/_Decomposition/SOFTWARE_DECOMP.md` package table |
> | Architecture basis | Rust core/application services; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes. The accepted architecture names a canonical JSON/JCS-compatible objective, while DEL-14-02 retains the 0.1 sorted-compact profile and the D-67 0.2 checked JCS/I-JSON profile as separate versioned contracts. | `_CONTEXT.md` Architecture Basis Injection; `DEC-074` E1 |
> | Unit boundary | Unit-bearing physical values crossing schema, service, solver, import/export, report, or rule-evaluation boundaries must carry explicit unit metadata unless explicitly dimensionless or equivalent. | `docs/SPEC.md` section 4 |
> | Hash boundary | The retained 0.1 profile emits `SORTED_COMPACT_JSON`; the D-67 0.2 profile uses the shared checked `openpipestress_jcs_ijson_v1` canonicalizer. Record revision, computed hashes and received checksum evidence stay distinct; broader non-JSON/binary partitioning is not decided here. | `docs/SPEC.md` section 4.4; `DEC-074` E1 / PDU-003 |
> | Result-envelope relationship | Result export envelopes must identify result set, model/run basis, solver version, unit-system reference, load-case or combination basis, diagnostics, provenance, reproducibility hashes or audit-manifest reference, analysis statuses, rule-pack references where present, and professional-boundary notice. | `docs/SPEC.md` result export section |
> | Professional boundary | Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). | `docs/SPEC.md` analysis boundary section; `docs/CONTRACT.md` OPS-K-AUTH-1 |
> | Protected/private data boundary | Public artifacts must not copy private formulas, protected standards text, protected tables, proprietary values, or private rule-pack payloads. | `docs/SPEC.md` result export section; `docs/IP_AND_DATA_BOUNDARY.md` |
>

### CLM-005 — Conditions

> ##### Conditions
>
> | Condition | Status | Source |
> |---|---|---|
> | Exact analysis-run schema fields | Defined in the D-67 version dispatcher/member schemas; required SOW-072 categories still need producer/persistence conformance. | `_CONTEXT.md`; SOW-072 |
> | Analysis run identifier format | Versioned schema and `core/analysis_runs/compatibility.py` define `run_id` and immutable record-revision identity; new identity semantics remain separately governed. | `_CONTEXT.md`; `docs/TYPES.md` general identifier guidance only |
> | Model-state reference semantics | Must bind to exact model states; detailed state-record schema is upstream and outside this folder. | SOW-072; `Dependencies.csv` DAG-002-E0783 |
> | Solver version and settings representation | Required as part of the saved run record; versioned object evidence is in the selected schema member. | SOW-072 |
> | Unit-system representation | Required as part of the saved run record; missing unit metadata for unit-bearing physical values must be diagnostic rather than silently supplied. | SOW-072; `docs/SPEC.md` section 4 |
> | Load-case or combination basis | Required as part of the saved run record; references/cardinality follow the selected versioned schema. | SOW-072; `docs/SPEC.md` result export section |
> | Diagnostics and analysis statuses | Required as part of the saved run record; must preserve the professional-boundary status model. | SOW-072; `docs/SPEC.md` analysis boundary section |
> | Rule-pack and library references | Required as references/checksum/provenance surfaces; private payloads and protected content are not public artifact content. | SOW-072; `docs/SPEC.md` rule-pack/result export sections |
> | Result hashes | Hash evidence is version-qualified: retained 0.1 Python sorted-compact ASCII-escaped bytes use `SORTED_COMPACT_JSON`; D-67 0.2 uses the checked shared Rust `openpipestress_jcs_ijson_v1` profile. Preserve record-revision identity, payload scope and received-checksum metadata separately. Verify each family against its own schema and vectors; no promotion of legacy bytes or whole-product conformance follows. | D-67 ruling; version dispatcher and member schemas; analysis-run compatibility evidence |
> | Reproducibility tests | Required artifact class; contract/compatibility test sources exist; no new execution pass is claimed. | `_CONTEXT.md` anticipated artifacts |
>

### CLM-006 — Construction

> ##### Construction
>
> This deliverable is a data-model change. The conservative construction surface is:
>
> 1. Define `schemas/analysis_run.schema.json` as the canonical analysis-run record contract.
> 2. Include fields or referenced sub-objects sufficient to bind each analysis run to the source-supported categories in SOW-072: model state, solver version, settings, units, load cases, diagnostics, results, rule-pack references, library references, and result hashes.
> 3. Treat result attachment as immutable run-record evidence, not as mutable "current model" state.
> 4. Preserve unit metadata and dimensional diagnostics across run records wherever unit-bearing values are included or referenced.
> 5. Preserve the versioned hash contract: 0.1 remains `SORTED_COMPACT_JSON`; D-67 0.2 uses checked `openpipestress_jcs_ijson_v1`. Preserve payload scope, record revision and received checksums independently and check each version with its own conformance evidence.
> 6. Preserve professional-boundary and protected/private-data controls in schema shape, diagnostics, tests, and examples.
>
> Implementation evidence now includes `schemas/analysis_run.schema.json` (version dispatcher), its 0.1/legacy-desktop/0.2 members, `core/analysis_runs/records.py`, `core/analysis_runs/compatibility.py`, and the analysis-run contract/compatibility tests. These source artifacts do not establish product persistence completion or a new passing test result.
>

### CLM-007 — References

> ##### References
>
> | Reference | Use in this datasheet |
> |---|---|
> | `_CONTEXT.md` | Deliverable identity, scope, artifact expectations, architecture-basis injection |
> | `_REFERENCES.md` | Reference inventory and authority boundary |
> | `_DEPENDENCIES.md`; `Dependencies.csv` | approved graph resolved through `execution/_DAG/_LATEST.md` local mirror/evidence surface |
> | `execution/_Decomposition/SOFTWARE_DECOMP.md` | SOW-072, OBJ-016, PKG-14, DEL-14-02 decomposition basis |
> | `docs/_Registers/Deliverables.csv` | Deliverable row and anticipated artifacts |
> | `docs/_Registers/ScopeLedger.csv` | Scope item SOW-072 wording |
> | `docs/CONTRACT.md` | Agent, data, unit, report, privacy, and professional-boundary invariants |
> | `docs/SPEC.md` | Unit, persistence/hash, analysis-boundary, result-export, runner/result-envelope guidance |
> | `docs/TYPES.md` | Canonical model/schema vocabulary and boundary notes |
> | `docs/IP_AND_DATA_BOUNDARY.md` | Public/private and protected-content constraints |

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-14-02 Analysis run records

> #### Specification: DEL-14-02 Analysis run records
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-010 — Scope

> ##### Scope
>
> DEL-14-02 defines the analysis-run record data model and the supporting reproducibility test surface for saving analysis runs as immutable, traceable product records.
>
> In scope:
>
> - `schemas/analysis_run.schema.json`.
> - Run reproducibility tests.
> - Data-model coverage for the SOW-072 binding categories: exact model states, solver versions, settings, units, load cases, diagnostics, results, rule-pack references, library references, and result hashes.
> - Professional-boundary, unit, hash, provenance, protected-content, and private-data constraints that apply to persisted run records.
>
> Out of scope:
>
> - Comprehensive commercial prover output ingestion.
> - External validation or professional acceptance decisions.
> - Exact implementation code, storage container choice, dependency versions, solver numerical library, rule expression grammar/library, and public API transport.
> - New field-level/interface amendments beyond the versioned schemas authorized by D-67; unsupported extensions remain separately governed.
>

### CLM-011 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source | Verification |
> |---|---|---|---|
> | DEL-14-02-R001 | The deliverable shall produce or define `schemas/analysis_run.schema.json` as the analysis-run record schema artifact. | `_CONTEXT.md` anticipated artifacts; `docs/_Registers/Deliverables.csv` row DEL-14-02 | Artifact exists and is schema-validated once implementation occurs. |
> | DEL-14-02-R002 | The analysis-run record shall bind each saved run to an exact model state reference. | SOW-072 in `docs/_Registers/ScopeLedger.csv`; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-14-02 row | Schema contains a required model-state reference or equivalent validated binding. |
> | DEL-14-02-R003 | The analysis-run record shall preserve solver version, solver/settings basis, units, load-case basis, diagnostics, results, rule-pack references, library references, and result hashes. | SOW-072; `_CONTEXT.md` scope detail | Schema inspection and reproducibility tests confirm the categories are present or explicitly referenced. |
> | DEL-14-02-R004 | Results shall attach to immutable analysis-run records rather than only to mutable current model state. | `docs/_Registers/ScopeLedger.csv` row SOW-072; `_CONTEXT.md` envelope notes | Mutation/round-trip test demonstrates stored run evidence remains associated with the original run basis. |
> | DEL-14-02-R005 | Unit-bearing physical values included or referenced by run records shall carry explicit unit metadata unless explicitly classified as dimensionless, ratio, percentage, or coefficient. | `docs/SPEC.md` section 4; `docs/CONTRACT.md` OPS-K-UNIT-1 | Schema validation and negative tests reject or diagnose missing unit metadata where required. |
> | DEL-14-02-R006 | Missing solve-required or rule-check-required values shall be explicit findings with diagnostics and provenance, not silent defaults. | `docs/SPEC.md` analysis boundary section; `docs/CONTRACT.md` OPS-K-DATA-2 | Tests confirm missing required values produce structured diagnostics. |
> | DEL-14-02-R007 | Hash records shall identify payload scope, record revision and serialization profile. Retained 0.1 bytes use `SORTED_COMPACT_JSON`; D-67 0.2 computed hashes use checked `openpipestress_jcs_ijson_v1`. Preserve received checksum evidence separately; never relabel legacy bytes. | D-67 ruling; `schemas/analysis_run.schema.json` version dispatcher and member schemas | Check version-specific exact bytes/vectors, mutation sensitivity, received/computed separation and immutable record-revision identity. |
> | DEL-14-02-R008 | The record shall preserve the software authority boundary: solver results and diagnostics are software outputs; human acceptance is external and hash-bound. | `docs/SPEC.md` analysis boundary section; `docs/CONTRACT.md` OPS-K-AUTH-1 and OPS-K-AUTH-2 | Schema/status tests ensure automatic approval, certification, sealing, authentication, or code-compliance labels are not emitted (PRD §21.2). |
> | DEL-14-02-R009 | Rule-pack and library references shall expose identity/provenance/checksum-style metadata where source-supported, without embedding protected standards text, protected tables, proprietary values, private formulas, or private payloads in public artifacts. | SOW-072; `docs/SPEC.md` result export section; `docs/IP_AND_DATA_BOUNDARY.md` | Protected-content/private-data tests inspect examples and fixtures before acceptance. |
> | DEL-14-02-R010 | Run reproducibility tests shall be part of the deliverable output surface. | `_CONTEXT.md` anticipated artifacts | Test files exist and exercise stable run-record/hash behavior once implementation occurs. |
> | DEL-14-02-R011 | The schema and tests shall remain compatible with schema-first command/query/job result-envelope architecture. | `_CONTEXT.md` architecture basis; `docs/SPEC.md` runner/result-envelope sections | Service/result-envelope integration tests or contract tests are added when implementation scope reaches that boundary. |
>

### CLM-012 — Standards

> ##### Standards
>
> | Standard or governing basis | Status in this folder | Notes |
> |---|---|---|
> | JSON Schema 2020-12 contracts | Applicable architecture basis | `_CONTEXT.md` identifies JSON Schema 2020-12 contracts. The dispatcher/member schema files supply `$schema`, `$id` and placement evidence; public changes retain their governing scope. |
> | Deterministic sorted compact JSON hash basis | Hash evidence is version-qualified: retained 0.1 Python sorted-compact ASCII-escaped bytes use `SORTED_COMPACT_JSON`; D-67 0.2 uses the checked shared Rust `openpipestress_jcs_ijson_v1` profile. Preserve record-revision identity, payload scope and received-checksum metadata separately. Verify each family against its own schema and vectors; no promotion of legacy bytes or whole-product conformance follows. | D-67 ruling; version dispatcher and member schemas; analysis-run compatibility evidence |
> | PRD v0.2 sections 8.7 and 15.2 / FR-CMP-002 | Referenced but not locally accessible as primary source text | Decomposition and registers cite these references. Do not derive clause-level requirements beyond the accessible SOW-072 wording. |
> | Protected-content and private-data governance | Applicable | `docs/CONTRACT.md` and `docs/IP_AND_DATA_BOUNDARY.md` govern public/private data handling. |
> | Professional responsibility boundary | Applicable | `docs/CONTRACT.md` and `docs/SPEC.md` prohibit software-generated professional approval/compliance claims. |
>

### CLM-013 — Verification

> ##### Verification
>
> | Verification target | Method | Acceptance signal |
> |---|---|---|
> | Schema artifact | JSON Schema validation and repository schema gate | `schemas/analysis_run.schema.json` parses and satisfies local schema conventions once implemented. |
> | Required binding categories | Schema review plus tests | Model state, solver version/settings, units, load-case basis, diagnostics, results, rule-pack/library references, and result hashes are represented or explicitly referenced. |
> | Immutability of run evidence | Round-trip/persistence test | A saved run remains bound to the original model-state/run basis after unrelated model changes. |
> | Hash reproducibility | Hash evidence is version-qualified: retained 0.1 Python sorted-compact ASCII-escaped bytes use `SORTED_COMPACT_JSON`; D-67 0.2 uses the checked shared Rust `openpipestress_jcs_ijson_v1` profile. Preserve record-revision identity, payload scope and received-checksum metadata separately. Verify each family against its own schema and vectors; no promotion of legacy bytes or whole-product conformance follows. | D-67 ruling; version dispatcher and member schemas; analysis-run compatibility evidence |
> | Unit safety | Negative and positive unit tests | Unit-bearing values include unit metadata or produce blocking diagnostics. |
> | Professional boundary | Status/label tests | No automatic human approval, certification, sealing, authentication, or code-compliance label is emitted (PRD §21.2). |
> | Protected/private data boundary | Protected-content and fixture review | Public examples do not embed private/protected rule, library, standards, or proprietary payload content. |
>

### CLM-014 — Documentation

> ##### Documentation
>
> Required or expected records for later implementation:
>
> - `schemas/analysis_run.schema.json`.
> - Run reproducibility tests.
> - Schema-source notes for each binding category.
> - Hash/reproducibility test notes, including payload scope.
> - Unit and diagnostic test notes.
> - Protected-content/private-data review notes for any public examples or fixtures.
>
> The D-67 schemas and compatibility sources define current versioned fields, identities and projections. New migrations or interface changes need scoped authority; fixture provenance and product conformance remain explicit obligations.

- **AC-001** — The contract preserves schema-first run-record categories, immutable run evidence, explicit unit and dimension diagnostics, version-qualified serialization (0.1 `SORTED_COMPACT_JSON`; D-67 0.2 checked `openpipestress_jcs_ijson_v1`) without re-labeling historical bytes, payload scope, rule-pack/library provenance boundaries, professional-status exclusions, and visible producer diagnostic, unit/dimension, schema-field, and implementation TBDs.

## Production and Verification Method — Praxeology

### CLM-015 — Procedure: DEL-14-02 Analysis run records

> #### Procedure: DEL-14-02 Analysis run records
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-017 — Purpose

> ##### Purpose
>
> This procedure maintains and verifies the versioned analysis-run contract and its evidence. D-67 authorizes the 0.2 family; retained 0.1 evidence remains bound to its own bytes. Product persistence and producer conformance still require their own implementation evidence.
>

### CLM-018 — Prerequisites

> ##### Prerequisites
>
> | Prerequisite | Status | Source |
> |---|---|---|
> | Accepted decomposition basis | Available: `execution/_Decomposition/SOFTWARE_DECOMP.md` with its accepted amendments | `_CONTEXT.md` |
> | Deliverable identity and scope | Available: DEL-14-02, SOW-072, OBJ-016 | `_CONTEXT.md`; registers |
> | Approved dependency evidence | Available as local dependency mirror (current authority: `execution/_DAG/_LATEST.md`; earlier graph IDs are provenance) in `Dependencies.csv` | `_DEPENDENCIES.md`; `Dependencies.csv` |
> | Architecture basis constraints | Available in `_CONTEXT.md`; exact package-specific implementation choices remain decision-gated | `_CONTEXT.md` |
> | Field-level schema design | D-67 versioned schemas exist | `schemas/analysis_run.schema.json` dispatches 0.1, legacy-desktop 0.1 and 0.2 members; new interfaces require their own scope. |
> | Implementation code and tests | Reference/compatibility evidence exists | `core/analysis_runs/records.py`, `compatibility.py` and the analysis-run contract/compatibility tests; product persistence and producer conformance are separate. |
>

### CLM-019 — Steps

> ##### Steps
>
> 1. Re-read `_CONTEXT.md`, `_REFERENCES.md`, `Dependencies.csv`, and the DEL-14-02 rows in the decomposition and registers.
> 2. Confirm that the work remains limited to DEL-14-02 and does not require protected standards text, private project data, or sibling deliverable edits.
> 3. Draft or update `schemas/analysis_run.schema.json` using JSON Schema 2020-12 conventions, subject to repository schema patterns available at implementation time.
> 4. Represent the SOW-072 binding categories: exact model state, solver version, solver/settings basis, units, load-case basis, diagnostics, results, rule-pack references, library references, and result hashes.
> 5. Mark unsupported field-level choices as TBD until a source, accepted architecture decision, or human ruling supports them.
> 6. Ensure unit-bearing values are unit-aware or produce explicit diagnostics for missing/ambiguous units.
> 7. Preserve the versioned hash contract: 0.1 remains `SORTED_COMPACT_JSON`; D-67 0.2 uses checked `openpipestress_jcs_ijson_v1`. Preserve payload scope, record revision and received checksums independently and check each version with its own conformance evidence.
> 8. Preserve professional-boundary constraints: do not generate approval, certification, sealing, authentication, or code-compliance labels as software statuses (PRD §21.2).
> 9. Preserve protected/private-data constraints: do not embed private formulas, protected standards text, protected tables, proprietary values, or private rule-pack/library payloads in public fixtures or examples.
> 10. Add run reproducibility tests covering stable serialization/hash behavior, binding to model-state/run basis, unit diagnostics, and professional-boundary status behavior.
> 11. Record remaining TBDs and assumptions in implementation notes or tests rather than silently defaulting engineering behavior.
>

### CLM-020 — Verification

> ##### Verification
>
> | Check | Expected result |
> |---|---|
> | Source traceability | Requirements and tests cite accessible sources or mark unsupported details as TBD/ASSUMPTION. |
> | Schema parse | `schemas/analysis_run.schema.json` parses and satisfies repository schema validation once implemented. |
> | Binding coverage | Tests or schema review confirm all SOW-072 categories are present or explicitly referenced. |
> | Reproducibility | Hash evidence is version-qualified: retained 0.1 Python sorted-compact ASCII-escaped bytes use `SORTED_COMPACT_JSON`; D-67 0.2 uses the checked shared Rust `openpipestress_jcs_ijson_v1` profile. Preserve record-revision identity, payload scope and received-checksum metadata separately. Verify each family against its own schema and vectors; no promotion of legacy bytes or whole-product conformance follows. |
> | Unit handling | Missing unit metadata for unit-bearing values produces diagnostics rather than silent defaults. |
> | Boundary controls | Statuses and examples avoid automatic professional approval/compliance claims and protected/private data leakage. |
> | Dependency preservation | Current local rows must agree with approved authority through `execution/_DAG/_LATEST.md`, including RETIRED legacy duplicates; earlier DAG-002 identifiers are provenance. |
>

### CLM-021 — Records

> ##### Records
>
> The implementation pass should leave:
>
> - `schemas/analysis_run.schema.json`.
> - Run reproducibility tests.
> - Validation output for schema and tests.
> - Notes for any TBD schema fields, hash partitions, migration behavior, or fixture policies.
> - Protected-content/private-data review notes for public examples.
> - Any dependency-register changes only if they preserve approved DAG-006 rows or have explicit human/change authority.

- **VER-001** — Validate the contract and review source parity, every SOW-072 binding category, immutable run association, exact serialization-label and hash boundaries, unit/dimension and missing-data diagnostics, private/protected reference handling, retained residuals and TBDs, and professional-authority limits.

## Governing Values and Decisions — Axiology

### CLM-022 — Guidance: DEL-14-02 Analysis run records

> #### Guidance: DEL-14-02 Analysis run records
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-024 — Purpose

> ##### Purpose
>
> Analysis run records exist so solver outputs can be reviewed as reproducible records bound to the exact model state, execution context, diagnostics, references, and hashes that produced them. The record is a traceability surface for design iteration and review; it is not an automatic professional approval or external validation state.
>

### CLM-025 — Principles

> ##### Principles
>
> | Principle | Guidance | Source |
> |---|---|---|
> | Bind results to their run basis | Treat the run record as the durable home for result evidence, not as a transient view of the current model. | SOW-072; `_CONTEXT.md` envelope notes |
> | Preserve exact upstream references | Model state, solver version, settings, units, load-case basis, diagnostics, rule packs, libraries, and result hashes are binding categories, not optional narrative notes. | SOW-072 |
> | Keep units explicit | Unit-bearing physical values need unit metadata; missing or ambiguous units become diagnostics rather than inferred defaults. | `docs/SPEC.md` section 4 |
> | Make hashes scoped and reproducible | Hash evidence is version-qualified: retained 0.1 Python sorted-compact ASCII-escaped bytes use `SORTED_COMPACT_JSON`; D-67 0.2 uses the checked shared Rust `openpipestress_jcs_ijson_v1` profile. Preserve record-revision identity, payload scope and received-checksum metadata separately. Verify each family against its own schema and vectors; no promotion of legacy bytes or whole-product conformance follows. | D-67 ruling; version dispatcher and member schemas; analysis-run compatibility evidence |
> | Keep authority domains separate | Solver results and diagnostics are software outputs. User-rule checks use user-supplied rule-pack data. Human acceptance, if used, is external and hash-bound. | `docs/SPEC.md` analysis boundary section |
> | Protect private/protected payloads | Rule-pack and library references may carry identity, version, checksum, provenance, privacy, and review metadata, but public artifacts must not copy private/protected content. | `docs/IP_AND_DATA_BOUNDARY.md`; `docs/SPEC.md` result export section |
>

### CLM-026 — Considerations

> ##### Considerations
>
> - The versioned schema dispatcher and reference/compatibility builders provide implementation evidence. The original setup absence is historical; product persistence, actual producer diagnostics and cross-surface conformance remain delivery work.
> - SOW-072 is sufficient to identify required binding categories, but it is not sufficient to define exact schema field names, cardinality, or migration behavior.
> - The dependency mirror records approved graph resolved through `execution/_DAG/_LATEST.md` predecessor evidence for architecture basis, immutable model state records, analysis status semantics, audit manifest/hash conventions, result export format, and persistence/round-trip support. Those rows are an evidence surface, not independent graph authority.
> - The deliverable should align with `schemas/results.schema.yaml` and result-export envelopes where run records include or point to result payloads, but this folder does not define the final result export schema.
> - `PRD v0.2` references are cited by decomposition/register rows but the PRD source text was not locally read in this task. Requirements must not exceed accessible SOW/decomposition wording.
>

### CLM-027 — Trade-offs

> ##### Trade-offs
>
> | Topic | Conservative posture |
> |---|---|
> | Embedded results vs references | The versioned schemas define `result_refs` and referenced evidence; D-67 0.2 preserves source annotations, semantic contracts and checksum metadata. Do not infer comprehensive embedded result payloads or producer conformance from that contract. |
> | Hash granularity | Require explicit payload scope. Do not decide final partitioning for non-JSON or binary payloads; `docs/SPEC.md` leaves that TBD. |
> | Rule-pack/library metadata depth | Preserve identity/provenance/checksum/reference metadata without copying private formulas, protected tables, proprietary values, or private payloads. |
> | Status vocabulary | Use software status/diagnostic categories only within the professional-boundary model. Do not introduce automatic human approval or code-compliance statuses (PRD §21.2). |
> | Comparison readiness | DEL-14-02 should not pre-solve DEL-14-04 comparison semantics. It should preserve enough run basis and result hash evidence to support deterministic comparison later. |
>

### CLM-028 — Examples

> ##### Examples
>
> Analysis-run schemas, `core/analysis_runs/records.py`, `core/analysis_runs/compatibility.py` and `tests/test_analysis_run_compatibility.py` carry constructed contract examples and fixtures. Presence is implementation evidence, not a fixture rights clearance; public examples still require invented or otherwise redistribution-cleared provenance.
>

### CLM-029 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None identified | No direct source conflict was identified in the accessible source slices. Several details remain TBD because source text is absent or implementation-specific. | N/A | N/A | N/A | N/A | N/A |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-072 OBJ-016 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
