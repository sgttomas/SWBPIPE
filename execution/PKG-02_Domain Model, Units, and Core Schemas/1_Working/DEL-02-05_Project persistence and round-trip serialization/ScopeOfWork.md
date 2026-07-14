---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-02-05
package_id: PKG-02
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@69ac259a7113d5a838fb22aa2e84df0e0f109713
project_scope_refs: [SOW-050, SOW-041]
package_objective_refs: [OBJ-001, OBJ-012]
---

# Scope of Work — DEL-02-05

## Purpose and Objective Traceability

This Scope of Work defines `DEL-02-05` in service of project scope [SOW-050, SOW-041] and package objectives [OBJ-001, OBJ-012].

- **OUT-001** — A project-persistence and round-trip serialization contract covering create, open, save, versioned persistence, units, loads, rule-pack references, provenance, and reproducibility metadata is produced for the declared scope and objectives.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet - DEL-02-05 Project Persistence and Round-Trip Serialization

> #### Datasheet - DEL-02-05 Project Persistence and Round-Trip Serialization
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-02-05-DECL-002`.
>

### CLM-003 — PDU-024 Runtime Version Evidence

> ##### PDU-024 Runtime Version Evidence
>
> - Current authored model-document schema: `0.2.0`.
> - Governed legacy path: `0.1.0 -> 0.2.0`, reported as `stale` by version check and `migrated` by migration evidence.
> - Refusal/diagnostic states: `unsupported_schema`, `newer_than_supported`, and `failed`.
> - UI evidence consumers: Project Validation, Export Review, and Report persistence context; all use the shared browser-preview mirror of the DEC-019/DEC-033 evaluator.
>

### CLM-004 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-02-05 |
> | Package ID | PKG-02 |
> | Package | Domain Model, Units, and Core Schemas |
> | Deliverable name | Project persistence and round-trip serialization |
> | Type | DATA_MODEL_CHANGE |
> | Scope items | SOW-050, SOW-041 |
> | Objectives | OBJ-001, OBJ-012 |
> | Context envelope | M |
> | Anticipated artifacts | project file schema; round-trip tests; persistence service contract |
> | Source basis | `_CONTEXT.md` revision 0.7; `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7; `docs/_Registers/*.csv`; `docs/CONTRACT.md`; SCA-001 basis IDs AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08 |
>

### CLM-005 — Attributes

> ##### Attributes
>
> | Attribute | Required / current value | Source |
> |---|---|---|
> | Persistence purpose | Create, open, save, and version project files. | `docs/_Registers/Deliverables.csv` row DEL-02-05; `docs/PRD.md` section 10 FR-001 |
> | Round-trip preservation target | Model, units, loads, rule-pack references, and provenance metadata must round-trip without loss. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-050; `docs/PRD.md` section 10 FR-001 |
> | Schema baseline | JSON Schema 2020-12 is the public schema/interchange baseline. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-041 and section 8.2 |
> | Persistence encoding baseline | Versioned, JSON-schema-governed canonical JSON for JSON payloads. | `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.2 |
> | Hash basis | JSON payload hashes use canonical JSON with JCS-compatible canonicalization. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04 and section 8.2 |
> | Project package/container | TBD; SCA-001 leaves the physical project package/container unresolved. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04, section 8.2, OI-011 |
> | Migration mechanism | Migration-aware persistence is required; migration framework/tooling is TBD. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04, section 8.2, OI-011 |
> | Core data objects affected | Project, Model, LoadCase, RulePack references, Report/audit metadata, and provenance-bearing domain records. | `docs/SPEC.md` section 3 |
> | Application boundary | Storage/persistence must preserve domain invariant enforcement; adapters cannot bypass unit, provenance, diagnostics, or public/private data-boundary checks. | `docs/SPEC.md` section 1; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-02 and AB-00-07 |
> | Diagnostic behavior | Persistence validation, migration, and round-trip failures should be returned through structured diagnostics/result envelopes. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-03 and AB-00-06; `docs/SPEC.md` section 7 |
> | Test evidence | Round-trip, schema, unit/provenance, hash/canonicalization, migration-status, and protected-content/provenance gates are expected. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-08; `docs/VALIDATION_STRATEGY.md` sections 2 and 4 |
>

### CLM-006 — Conditions

> ##### Conditions
>
> - The deliverable is bounded to the PKG-02 persistence and serialization surface. It must not implement numerical solving, GUI views, rule-pack evaluation logic, reporting, external API transport, or local/private storage policy beyond the persistence contract needed by this deliverable. Source: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` PKG-02 row.
> - The public schema and tests must not introduce protected standards text, protected tables, proprietary values, copied formulas, or commercial examples. Source: `docs/CONTRACT.md` OPS-K-IP-1 through OPS-K-IP-3; `docs/IP_AND_DATA_BOUNDARY.md` sections 2-5.
> - Project files may contain private project/rule data in user-controlled contexts, but public fixtures must use invented, original, or permissively licensed data with documented provenance. Source: `docs/IP_AND_DATA_BOUNDARY.md` sections 2, 4, and 6; `docs/CONTRACT.md` OPS-K-PRIV-1.
> - All persisted numerical and engineering-relevant values must be unit-aware or carry enough unit metadata for downstream dimensional checks. Source: `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/PRD.md` section 10 FR-002.
> - Software output and persistence metadata must not claim code compliance, certification, sealing, approval, or professional reliance. Source: `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/TYPES.md` section 4.
>

### CLM-007 — Construction

> ##### Construction
>
> The deliverable construction target is a three-artifact kit:
>
> | Artifact | Minimum construction expectation | Source |
> |---|---|---|
> | Project file schema | Define a versioned project document contract covering project identity, schema version, unit system, model payload, load payloads, rule-pack references, provenance metadata, and validation/migration status. Exact schema file layout is TBD. | `docs/_Registers/Deliverables.csv` row DEL-02-05; `docs/SPEC.md` section 3; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04 |
> | Round-trip tests | Verify parse -> normalize/validate -> serialize -> parse cycles preserve required project content and produce deterministic canonical JSON/hash behavior for JSON payloads. Exact test harness details are TBD. | `docs/PRD.md` section 10 FR-001; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04 and AB-00-08 |
> | Persistence service contract | Define application-service operations for create/open/save/version validation, migration-status reporting, diagnostics/result envelopes, and data-boundary checks. Exact interface language/API shape is TBD. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-02, AB-00-03, AB-00-06, AB-00-07 |
>
> ASSUMPTION: The persisted project document will be treated as durable project state, while session-only UI state and job-progress state remain outside this deliverable unless later injected by an applicable GUI-state brief.
>

### CLM-008 — Persisted Project Envelope Inventory

> ###### Persisted Project Envelope Inventory
>
> This inventory is a logical schema contract checklist, not the final schema file layout. Exact schema file layout, code-generation tooling, physical container, and migration implementation remain TBD under SCA-001.
>
> | Logical slot | Minimum persisted meaning | Source | Open detail |
> |---|---|---|---|
> | Document identity and schema metadata | Document kind, schema version, project identifier/name, and version-handling metadata. | SOW-050; `docs/PRD.md` section 10 FR-001; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04 | Exact field names and schema file layout TBD. |
> | Project basis and storage policy | Project/design-basis metadata, storage/private-data policy flags, and report settings where present. | `docs/SPEC.md` section 3 `Project`; `docs/PRD.md` Appendix A; `docs/CONTRACT.md` OPS-K-PRIV-1 | Exact local-first enforcement fields TBD. |
> | Unit-system reference | Declared unit system or explicit unit metadata sufficient for downstream unit checks. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/PRD.md` section 10 FR-002; SOW-025 | Missing or incompatible unit metadata must become diagnostics, not defaults. |
> | Model payload | Model objects such as nodes, elements, sections, materials, components, supports, and related references where present. | `docs/SPEC.md` section 3; `docs/PRD.md` Appendix A | Field-level object schemas remain coordinated with DEL-02-01. |
> | Load payloads | Load cases and load records with units and source/provenance where engineering reliance is affected. | SOW-050; `docs/SPEC.md` section 3 `LoadCase`; `docs/DIRECTIVE.md` section 2.1 | Exact load object schema remains coordinated with load deliverables. |
> | Rule-pack references | Rule-pack ID/name, version, checksum, source note, redistribution/private status, and required-input linkage where present. | `docs/SPEC.md` section 6; `docs/CONTRACT.md` OPS-K-RULE-3; `docs/PRD.md` section 12.2 | Public fixtures must not embed protected rule formulas or allowables. |
> | Provenance and redistribution metadata | Source/provenance, license or redistribution status, contributor/review fields where public data records are present. | `docs/IP_AND_DATA_BOUNDARY.md` section 4; `docs/CONTRACT.md` OPS-K-DATA-3 | Private records may use private/source labels; public records need complete review metadata. |
> | Validation, diagnostics, and migration status | Validation findings, migration status, and structured diagnostics/result-envelope fields. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-03, AB-00-04, AB-00-06; `docs/SPEC.md` section 7 | Status values beyond unsupported, stale, and failed migration are TBD. |
> | Reproducibility metadata | Model hash, input-manifest compatibility, rule-pack checksum references, and canonical JSON/JCS-compatible hash basis for JSON payloads. | `docs/SPEC.md` section 8; `docs/PRD.md` section 15.3; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.2 | Exact payload/manifest partition remains TBD. |
> | Review records, if present | Human review labels and hashes tied to specific model/rule/report content without automatic compliance claims. | `docs/CONTRACT.md` OPS-K-AUTH-1 and OPS-K-AUTH-2; `docs/TYPES.md` section 4 | Review-record schema is optional and final authority labels remain TBD. |
>

### CLM-009 — References

> ##### References
>
> - `_CONTEXT.md` revision 0.7 for deliverable identity, SOW/objective mapping, accepted architecture basis IDs, and write context.
> - `_DEPENDENCIES.md` for declared dependency status; no human-owned upstream/downstream dependency list was provided.
> - `docs/_Registers/Deliverables.csv` row DEL-02-05.
> - `docs/_Registers/ScopeLedger.csv` rows SOW-050 and SOW-041.
> - `docs/_Registers/ContextBudgetQA.csv` row DEL-02-05.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7, especially SOW-041, SOW-050, PKG-02, DEL-02-05, section 8.1, section 8.2, and OI-011.
> - `docs/CONTRACT.md` invariant catalog.
> - `docs/TYPES.md` sections 3-8.
> - `docs/SPEC.md` sections 1, 3, 7, 8, 9, and 11.
> - `docs/DIRECTIVE.md` sections 2-5.
> - `docs/IP_AND_DATA_BOUNDARY.md` sections 2-7.
> - `docs/PRD.md` section 10 FR-001/FR-002, section 11.8, section 12.4, section 15.3, section 18, section 19, and section 22.1.
> - `docs/VALIDATION_STRATEGY.md` sections 2, 4, and 5.

## Completion and Reliance Basis — Epistemology

### CLM-010 — Specification - DEL-02-05 Project Persistence and Round-Trip Serialization

> #### Specification - DEL-02-05 Project Persistence and Round-Trip Serialization
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-011 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-02-05-DECL-001`.
>

### CLM-012 — Scope

> ##### Scope
>
> This specification defines the bounded requirements for DEL-02-05: create/open/save/versioned project persistence and deterministic round-trip serialization for project models, unit metadata, loads, rule-pack references, and provenance metadata.
>
> In scope:
>
> - project file schema contract;
> - round-trip serialization and deterministic canonicalization behavior for JSON payloads;
> - persistence service contract for create/open/save/version validation and migration-status reporting;
> - validation and test obligations for schema, round-trip, hash stability, provenance, unit metadata, and protected-content boundaries.
>
> Out of scope:
>
> - numerical solving, stress recovery, GUI views, report rendering, and rule-pack expression evaluation;
> - physical project package/container selection, migration framework/tooling, binary asset packaging, exact dependency versions, public API transport, and exact schema file layout, all of which remain TBD under SCA-001;
> - any bundled protected standards content, code-specific formulas, tables, allowables, or proprietary project examples.
>
> Sources: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` PKG-02 and DEL-02-05 rows; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.2.
>

### CLM-013 — PDU-024 Version Contract Integration

> ##### PDU-024 Version Contract Integration
>
> The accepted runtime authoring family is model-document schema `0.2.0`. UI version checks consume the established DEC-019/DEC-033 migration evaluator: `current` identifies `0.2.0`, `stale` identifies an older document with a governed migration path, and `unsupported_schema`, `newer_than_supported`, or `failed` remain explicit refusal/diagnostic states. Separate migration evidence continues to report `migrated` when the `0.1.0 -> 0.2.0` in-memory path is applied. No new status names or migration framework is introduced.
>

### CLM-014 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source |
> |---|---|---|
> | REQ-02-05-001 | The persistence surface shall support project create, open, save, and version handling. | SOW-050; `docs/PRD.md` section 10 FR-001 |
> | REQ-02-05-002 | A project file round trip shall preserve model content, unit metadata, loads, rule-pack references, and provenance metadata without loss. | SOW-050; `docs/PRD.md` section 10 FR-001 |
> | REQ-02-05-003 | The project file schema shall align with the machine-readable schema scope for project, model, material, component, load, result, and report schemas where this persistence deliverable references those objects. | SOW-041; `docs/SPEC.md` section 3 |
> | REQ-02-05-004 | Public schemas/interchange for this deliverable shall use JSON Schema 2020-12 as the baseline. Exact schema file layout and code-generation tooling are TBD. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-041 and section 8.2 |
> | REQ-02-05-005 | JSON persistence payloads shall be deterministic and hash-ready using canonical JSON with JCS-compatible canonicalization. Exact canonicalization library/tooling is TBD. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04 and section 8.2 |
> | REQ-02-05-006 | The schema shall include explicit schema/version metadata and migration status sufficient to detect unsupported, stale, or failed migrations. The current DEC-019/DEC-033 runtime contract implements this classification for the accepted `0.2.0` family; additional migration tooling remains TBD. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04, section 8.2, OI-011 |
> | REQ-02-05-007 | Persisted numerical and engineering-relevant fields shall carry explicit units or references to a declared unit system sufficient for downstream unit checks. Silent unit defaults are not permitted. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/PRD.md` section 10 FR-002; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04 |
> | REQ-02-05-008 | Provenance metadata shall be preserved for materials, components, SIF/flexibility values, allowables, rule-pack values, and other engineering-reliance data when those records are present in a project. Missing provenance shall be representable as a warning/finding, not silently accepted as complete. | `docs/CONTRACT.md` OPS-K-DATA-3 and OPS-K-DATA-2; `docs/DIRECTIVE.md` section 3; `docs/IP_AND_DATA_BOUNDARY.md` section 4 |
> | REQ-02-05-009 | Rule-pack data shall be represented as user/private rule-pack references with version/checksum/source-note capability; the public project schema/tests shall not embed protected rule formulas, allowables, or code-specific values. | `docs/SPEC.md` section 6; `docs/CONTRACT.md` OPS-K-RULE-3 and OPS-K-IP-1; `docs/PRD.md` section 12.4 |
> | REQ-02-05-010 | Persistence operations shall preserve application-service, domain-core, storage, schema, validation, and adapter boundaries; adapters/plugins shall not bypass unit, provenance, diagnostics, sandboxing, envelope, report, or public/private data-boundary checks. | `docs/SPEC.md` section 1; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-02 and AB-00-07 |
> | REQ-02-05-011 | Open/save/validate/migrate failures shall be expressible through structured diagnostics/result envelopes carrying code, class, severity, source, affected object, message, remediation, and provenance where applicable. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-03 and AB-00-06; `docs/SPEC.md` section 7 |
> | REQ-02-05-012 | Persistence artifacts shall support reproducibility metadata needed by downstream reports and audit manifests, including model hash and input-manifest compatibility for JSON payloads. | `docs/SPEC.md` section 8; `docs/PRD.md` section 15.3; `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-039 and AB-00-04 |
> | REQ-02-05-013 | Private project, material, component, and rule-pack data shall remain local-first and shall not be transmitted or committed publicly by default. | `docs/CONTRACT.md` OPS-K-PRIV-1; `docs/PRD.md` section 18 |
> | REQ-02-05-014 | Public fixtures and examples used by this deliverable shall be invented, original, public-domain, or permissively licensed and shall include provenance/redistribution status when data records are present. | `docs/IP_AND_DATA_BOUNDARY.md` sections 2-5; `docs/VALIDATION_STRATEGY.md` section 5 |
> | REQ-02-05-015 | The round-trip test suite shall include schema validation, deterministic reserialization, JSON canonicalization/hash stability, migration-status handling, unit/provenance preservation, private-data boundary checks, and protected-content/provenance gates where relevant. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-08; `docs/VALIDATION_STRATEGY.md` sections 2 and 4; `docs/PRD.md` section 22.1 |
> | REQ-02-05-016 | No persistence status, field, diagnostic, report hook, or example shall claim professional approval, certification, sealing, authentication, or automatic code compliance. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/TYPES.md` section 4; `docs/DIRECTIVE.md` section 3 |
> | REQ-02-05-017 | The project file schema shall enumerate logical envelope slots for document/schema metadata, project identity, unit-system reference, model payload, load payloads, rule-pack references, provenance/redistribution metadata, diagnostics or migration status, reproducibility metadata, and optional review records. | SOW-050; `docs/SPEC.md` section 3; `docs/IP_AND_DATA_BOUNDARY.md` section 4; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04 |
> | REQ-02-05-018 | Round-trip acceptance criteria shall compare semantic equality for model content, unit metadata, loads, rule-pack references, provenance metadata, and reproducibility metadata after parse -> validate/normalize -> serialize -> parse. | SOW-050; `docs/PRD.md` section 10 FR-001; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04 |
> | REQ-02-05-019 | Unit round-trip criteria shall distinguish explicit units, declared unit-system references, missing units, and incompatible unit metadata; missing or incompatible units shall produce findings instead of silent defaults. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/PRD.md` section 10 FR-002; `docs/DIRECTIVE.md` sections 2.2 and 3 |
> | REQ-02-05-020 | Hash and reproducibility criteria shall identify the compared JSON payload, any input manifest, and any referenced non-JSON or binary manifest; exact payload partitioning remains TBD. | `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.2; `docs/SPEC.md` section 8; `docs/PRD.md` section 15.3 |
> | REQ-02-05-021 | Migration handling shall define status semantics and diagnostics for unsupported, stale, and failed migrations. The bounded runtime uses the established schema vocabulary and DEC-019/DEC-033 evaluator; additional migration tooling and the compatibility-window size remain TBD. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04, section 8.2, OI-011 |
> | REQ-02-05-022 | The persistence service contract shall define create, open, save, validate, version-check, and migrate-operation behavior in schema-first command/query/result-envelope terms; exact language/API signatures remain TBD. | SOW-050; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-03 and section 8.2; `docs/SPEC.md` section 1 |
> | REQ-02-05-023 | Diagnostics emitted by persistence operations shall define deterministic class coverage for schema, migration, unit metadata, provenance, rule-pack reference, IP/data-boundary, private-data, and professional-boundary failures or warnings. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06; `docs/SPEC.md` section 7; `docs/CONTRACT.md` invariant index |
> | REQ-02-05-024 | Rule-pack references persisted in projects shall carry enough metadata to identify private/public status, version/checksum, source note, redistribution status, and missing/private-reference diagnostics without exposing protected rule content in public artifacts. | `docs/SPEC.md` section 6; `docs/PRD.md` sections 12.2 and 12.4; `docs/IP_AND_DATA_BOUNDARY.md` sections 3, 6, and 7 |
> | REQ-02-05-025 | Create/open/save behavior shall preserve the local-first/private-data boundary by default; exact enforcement split between persistence operations, export/commit safeguards, and PKG-12 controls remains TBD. | `docs/CONTRACT.md` OPS-K-PRIV-1; `docs/PRD.md` sections 18.1-18.3; `docs/IP_AND_DATA_BOUNDARY.md` section 6 |
> | REQ-02-05-026 | Optional human-review records, if stored, shall bind to specific model/rule/report hashes and use authority labels that do not survive content changes and do not imply software certification or automatic code compliance. | `docs/CONTRACT.md` OPS-K-AUTH-1 and OPS-K-AUTH-2; `docs/TYPES.md` section 4 |
>

### CLM-015 — Contract Detail Tables

> ###### Contract Detail Tables
>

### CLM-016 — Project Envelope Slots

> ###### Project Envelope Slots
>
> | Slot | Minimum contract content | Verification note |
> |---|---|---|
> | `document_metadata` | Document kind, schema version, project/document identity, and version-handling metadata. | Fixture exposes version fields and migration status or diagnostic behavior. |
> | `project` | Project identity, design-basis metadata, storage policy/private-data indicators, and report settings where applicable. | Fields align to `Project` object scope in `docs/SPEC.md` section 3; exact field names TBD. |
> | `units` | Declared unit system and/or explicit unit metadata on numerical values. | Missing units and incompatible dimensions fail with deterministic findings. |
> | `model` | Nodes, elements, sections, materials, components, supports, and references used by the persisted model where present. | Round-trip comparison verifies stable identity and references, not text formatting. |
> | `loads` | Load cases and load records with units and source/provenance where needed. | Round-trip comparison verifies load IDs, types, magnitudes, units, and source fields where present. |
> | `rule_pack_refs` | Rule-pack ID/name, version, checksum, source note, redistribution/private status, and required-input linkages where present. | Public fixtures reference only invented or permissively licensed rule packs. |
> | `provenance` | Source/provenance and redistribution/review metadata for public data records; private/source labels where project records are private. | Public data records follow `docs/IP_AND_DATA_BOUNDARY.md` section 4 fields. |
> | `diagnostics_and_migration` | Validation findings, migration status, and structured result-envelope diagnostics. | Unsupported, stale, and failed migration cases have deterministic diagnostics. |
> | `reproducibility` | Model hash, input-manifest compatibility, solver/report linkage where applicable, and rule-pack checksum references. | Hash inputs are identified and stable for canonical JSON payloads. |
> | `review_records` | Optional human review labels and hashes for project-specific acceptance records. | Any review label remains human authority only and cannot be generated as code compliance. |
>

### CLM-017 — Round-Trip Equality Criteria

> ###### Round-Trip Equality Criteria
>
> | Category | Equal after round trip when | Failure examples |
> |---|---|---|
> | Model content | Stable object IDs, object types, references, coordinates/geometry fields, component/material/section references, and support/load associations remain semantically equivalent. | Missing object, changed reference, changed engineering-relevant value, or unreported schema loss. |
> | Unit metadata | Explicit units or declared unit-system references remain present and dimensionally compatible. | Missing unit, incompatible unit dimension, or silent default insertion. |
> | Loads | Load-case IDs, load types, magnitudes, units, and source/provenance fields where present are preserved. | Changed magnitude, missing units, lost load case, or lost source note. |
> | Rule-pack references | ID/name, version, checksum, source note, private/public or redistribution status, and required-input linkages remain present. | Expanded protected rules in public artifacts, checksum loss, or private/public status loss. |
> | Provenance metadata | Provenance fields required for engineering-reliance data remain present or generate findings when missing. | Lost source, lost redistribution status, or missing provenance accepted as complete. |
> | Reproducibility metadata | Canonical JSON payload hash basis and input-manifest compatibility are reproducible for the selected payload/manifest partition. | Hash includes volatile/session fields without approved basis, or payload partition is undocumented. |
>

### CLM-018 — Migration Status Semantics

> ###### Migration Status Semantics
>
> | Status / case | Required handling | Open detail |
> |---|---|---|
> | Supported/current schema | Validate and round-trip without migration diagnostics. | Runtime version-check status `current`; accepted authoring family `0.2.0`. |
> | Unsupported schema | Return a structured diagnostic and avoid silent coercion. | Runtime status `unsupported_schema`. |
> | Stale schema | Report migration-needed status or diagnostic before accepting as current. | Version-check status `stale`; separate migration evidence reports `migrated` when the governed path succeeds. |
> | Failed migration | Return a structured failure diagnostic with affected object/source where available. | Runtime status `failed`; rollback/transaction details remain bounded by the existing persistence operation. |
> | Newer-than-supported schema | Refuse down-migration and emit a structured diagnostic. | Runtime status `newer_than_supported`. |
>

### CLM-019 — Persistence Service Operations

> ###### Persistence Service Operations
>
> | Operation | Minimum inputs | Minimum outputs / diagnostics | Notes |
> |---|---|---|---|
> | Create project | Project identity, unit system, storage/private-data policy, optional template/reference selection. | Versioned project envelope or diagnostics/result envelope. | Exact constructor/API shape TBD. |
> | Open project | Project artifact reference and caller context. | Parsed project envelope, validation result, migration status, diagnostics. | Must not bypass schema, unit, provenance, or data-boundary checks. |
> | Save project | Validated project envelope, target artifact reference, canonicalization/hash options. | Saved artifact reference, version metadata, hash/manifest evidence, diagnostics. | Physical container remains TBD. |
> | Validate project | Project envelope or artifact reference. | Schema, unit, provenance, rule-pack-reference, private-data, and protected-content diagnostics. | Diagnostics use structured result-envelope fields. |
> | Version check | Project schema/version metadata. | `current`, `stale`, `unsupported_schema`, `failed`, or `newer_than_supported` status and diagnostics. | Implemented for the bounded DEC-019/DEC-033 runtime; no additional names introduced. |
> | Migrate project | Source project, target schema version, migration policy. | Migrated project or failed migration diagnostics. | Migration framework/tooling TBD. |
>

### CLM-020 — Diagnostic Class Coverage

> ###### Diagnostic Class Coverage
>
> | Class / namespace | Applies when | Source basis |
> |---|---|---|
> | `SCHEMA_VALIDATION` (PROPOSAL) | Project envelope or object payload violates schema. | SOW-041; JSON Schema 2020-12 baseline. |
> | `MIGRATION` (PROPOSAL) | Schema version is unsupported, stale, or migration fails. | AB-00-04; OI-011. |
> | `UNIT_METADATA` (PROPOSAL) | Unit metadata is missing or incompatible. | OPS-K-UNIT-1; PRD FR-002. |
> | `PROVENANCE_WARNING` | Source/provenance is missing or weak. | `docs/SPEC.md` section 7; OPS-K-DATA-2/3. |
> | `RULE_CHECK_BLOCKING` | Mechanics may exist, but required user/rule-pack data is missing. | `docs/SPEC.md` section 7; PRD section 14.4. |
> | `IP_BOUNDARY_WARNING` | Public contribution, fixture, report hook, or export may expose protected/private content. | `docs/SPEC.md` section 7; IP/data-boundary policy. |
> | `PRIVATE_DATA` (PROPOSAL) | An operation would transmit, commit, export, or publish private project/rule/library data by default. | OPS-K-PRIV-1; PRD section 18. |
> | `PROFESSIONAL_BOUNDARY` (PROPOSAL) | A status or artifact could imply certification, sealing, approval, authentication, or automatic code compliance. | OPS-K-AUTH-1; `docs/TYPES.md` section 4. |
>
> PROPOSAL: Exact diagnostic code prefixes remain TBD; the table above records required deterministic coverage, not final code names.
>

### CLM-021 — Standards

> ##### Standards
>
> | Standard / basis | Applicability | Status |
> |---|---|---|
> | JSON Schema 2020-12 | Public schema/interchange baseline for the project file schema. | Required by SCA-001; exact schema file layout TBD. |
> | Canonical JSON / JCS-compatible canonicalization | Hash basis for JSON payloads. | Required by SCA-001; exact library/tooling TBD. |
> | OpenPipeStress invariant catalog | Binding constraints for IP boundary, unit safety, provenance, professional boundary, private data, and agent outputs. | Required by `docs/CONTRACT.md`. |
> | OpenPipeStress data-boundary policy | Controls public/private data handling and provenance fields. | Required by `docs/IP_AND_DATA_BOUNDARY.md`. |
>
> No piping design code or protected standards dataset is a governing input to this deliverable. The persistence schema may carry user-supplied rule-pack references and provenance, but it must not bundle protected code content.
>

### CLM-022 — Verification

> ##### Verification
>
> | Requirement IDs | Verification approach |
> |---|---|
> | REQ-02-05-001, REQ-02-05-006, REQ-02-05-010, REQ-02-05-011, REQ-02-05-021, REQ-02-05-022, REQ-02-05-023 | Persistence service contract review: create/open/save/validate/version-check/migrate operations have defined inputs, outputs, diagnostics/result envelopes, boundary checks, and migration-status behavior. |
> | REQ-02-05-002, REQ-02-05-005, REQ-02-05-018, REQ-02-05-020 | Round-trip tests: parse a project fixture, validate/normalize, serialize canonically, reparse, and compare category-level semantic equality plus canonical JSON/hash behavior. |
> | REQ-02-05-003, REQ-02-05-004, REQ-02-05-017 | Schema validation: fixtures validate against JSON Schema 2020-12 contracts; invalid fixtures fail with deterministic diagnostics; project-envelope slots are covered or marked TBD. |
> | REQ-02-05-007, REQ-02-05-008, REQ-02-05-019 | Unit/provenance tests: explicit units and declared unit systems survive round trip; missing/incompatible units and missing/weak provenance produce findings rather than silent defaults. |
> | REQ-02-05-009, REQ-02-05-024 | Rule-pack reference tests: ID/name, version, checksum, source note, and private/public or redistribution status survive round trip; public fixtures do not expand protected rule content. |
> | REQ-02-05-013, REQ-02-05-014, REQ-02-05-016, REQ-02-05-025, REQ-02-05-026 | Protected-content/private-data/professional-boundary review: fixtures contain invented/permissive data only; private references are not publicly committed or transmitted by default; no compliance/certification status appears; optional review records bind to hashes if present. |
> | REQ-02-05-012, REQ-02-05-015, REQ-02-05-020 | Reproducibility tests: model-hash/input-manifest compatibility is stable for deterministic JSON payloads; payload/manifest partition is documented; protected-content/provenance gates run where public examples/templates are present. |
>

### CLM-023 — Documentation

> ##### Documentation
>
> The deliverable should leave or update these records:
>
> - project file schema documentation, including schema version, migration status semantics, canonicalization/hash basis, and unresolved implementation TBDs;
> - persistence service contract documentation for create/open/save/version validation and diagnostics/result envelopes;
> - round-trip test documentation naming fixtures, expected preserved fields, semantic equality checks, canonical hash expectations, and protected-content/provenance gates;
> - diagnostic taxonomy coverage and migration-status case matrix, with exact code names/status names marked TBD where not approved;
> - fixture/public-data review evidence covering provenance, redistribution status, protected-content screening, private-data handling, and professional-boundary language;
> - open decisions for physical project package/container, migration framework/tooling, binary asset packaging, exact schema file layout, dependency/tool selections, and hash payload partitioning;
> - no protected standards excerpts, proprietary data, or code-compliance/certification claims.

- **AC-001** — The contract preserves the accepted source requirements for schema-governed local persistence, deterministic round trips, explicit units and provenance, validated service boundaries, and unresolved physical-container or migration decisions.

## Production and Verification Method — Praxeology

### CLM-024 — Procedure - DEL-02-05 Project Persistence and Round-Trip Serialization

> #### Procedure - DEL-02-05 Project Persistence and Round-Trip Serialization
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-025 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-02-05-DECL-004`.
>

### CLM-026 — PDU-024 Version Contract Check

> ##### PDU-024 Version Contract Check
>
> 1. Evaluate the in-document semantic version through the established migration evaluator.
> 2. Emit `current` only for the accepted `0.2.0` family; map a successful older-version migration path to `stale` at version-check grain and retain `migrated` in the migration record.
> 3. Emit structured `unsupported_schema`, `newer_than_supported`, or `failed` diagnostics without coercion.
> 4. Exercise current, stale, unsupported, newer, and failed classifications in focused contract tests and verify the Project Validation, Export Review, and Report consumers do not contain stale current-version comparisons.
>

### CLM-027 — Purpose

> ##### Purpose
>
> This procedure describes how to produce and check the DEL-02-05 artifacts: project file schema, round-trip tests, and persistence service contract. It is operational guidance for implementation and review; it does not itself implement the schema or service.
>

### CLM-028 — Prerequisites

> ##### Prerequisites
>
> - Confirm the active deliverable is DEL-02-05 under PKG-02 and that write scope is limited to the deliverable-local artifacts. Source: `_CONTEXT.md`; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` section 4.
> - Use `_CONTEXT.md` revision 0.7, `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7, and register rows for DEL-02-05, SOW-050, SOW-041, and ContextBudgetQA DEL-02-05.
> - Apply SCA-001 basis IDs AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, and AB-00-08 only to the extent they constrain this persistence deliverable.
> - Treat `_DEPENDENCIES.md` as human-owned dependency context. Current state: no specific upstream/downstream dependency list was declared.
> - Keep protected standards/code data, proprietary values, and code-compliance/certification claims out of public artifacts. Source: `docs/CONTRACT.md`; `docs/IP_AND_DATA_BOUNDARY.md`.
>

### CLM-029 — Steps

> ##### Steps
>
> 1. Confirm scope and lifecycle state.
>    - Verify the deliverable folder, status, allowed writes, and anticipated artifacts.
>    - Stop or escalate if the task requires protected data or expands outside project persistence.
>
> 2. Establish the source set.
>    - Read `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `docs/_Registers/*.csv` rows for this deliverable, `docs/CONTRACT.md`, and the relevant SOFTWARE_DECOMP sections.
>    - Record missing or inaccessible sources as warnings; do not invent requirements.
>
> 3. Define the persistence boundary.
>    - Include create/open/save/version behavior, project schema, round-trip serialization, and service diagnostics.
>    - Exclude numerical solving, GUI views, rule-pack evaluator behavior, report rendering, external API transport, and physical package/container decisions unless a later brief authorizes them.
>
> 4. Draft or update the project file schema contract.
>    - Include schema/version metadata, migration status, project identity, unit-system references, model payload, load payloads, rule-pack references, and provenance metadata.
>    - Enumerate the logical project-envelope slots before choosing exact files or generated types: document metadata, project identity/storage policy, unit system, model payload, load payloads, rule-pack references, provenance/redistribution metadata, diagnostics/migration status, reproducibility metadata, and optional review records.
>    - For rule-pack references, capture ID/name, version, checksum, source note, redistribution/private status, and missing/private-reference diagnostics without embedding protected rule content in public artifacts.
>    - For provenance-bearing public data records, include source, source location, license or redistribution basis, contributor certification, redistribution status, and review status when applicable.
>    - Use JSON Schema 2020-12 as the public schema/interchange baseline.
>    - Mark exact schema file layout, code-generation tooling, migration framework, and physical project container as TBD unless already resolved by an approved decision.
>
> 5. Define the persistence service contract.
>    - Specify create/open/save/validate/version-check/migrate entry points and expected result envelopes.
>    - For each operation, document minimum inputs, outputs, diagnostics, and boundary checks:
>      - create: project identity, declared unit system, storage/private-data policy, and optional template/reference selection;
>      - open: project artifact reference, schema/version handling, validation, migration status, and diagnostics;
>      - save: validated project envelope, target artifact reference, canonicalization/hash evidence, and diagnostics;
>      - validate: schema, unit, provenance, rule-pack-reference, private-data, and protected-content diagnostics;
>      - version-check/migrate: unsupported, stale, failed, and TBD newer/current status handling.
>    - Require structured diagnostics for validation, migration, protected-content, private-data, unit, provenance, rule-pack-reference, and professional-boundary failures or warnings.
>    - Preserve application-service/domain-core/storage boundaries; adapters and plugins cannot bypass validation or data-boundary checks.
>
> 6. Define canonicalization and reproducibility handling.
>    - Use canonical JSON with JCS-compatible canonicalization as the basis for JSON payload hashes.
>    - Record whether each hash applies to a normalized JSON payload, input manifest, referenced private/rule/library artifact, or non-JSON/binary asset manifest. Exact split is TBD.
>    - Document which fields are excluded from deterministic comparison because they are volatile, environment-specific, or session-only. If exclusion cannot be justified from source or approved decision, mark the partition TBD.
>    - Avoid relying on non-deterministic field ordering, timestamps, machine-local paths, or session-only state for deterministic comparisons.
>
> 7. Build the round-trip test plan.
>    - Include a fixture inventory with valid invented fixtures and invalid fixtures.
>    - Minimum fixture classes: valid minimal invented project; invalid schema shape; missing units; incompatible unit metadata; missing/weak provenance; private rule-pack reference; missing rule-pack reference; unsupported/stale/failed migration version cases; and synthetic protected-content/private-data boundary warnings that do not include actual protected data.
>    - Test parse -> validate/normalize -> serialize -> parse cycles for semantic equality.
>    - Test semantic equality by category: model identity/references, unit metadata, loads, rule-pack references, provenance metadata, and reproducibility metadata.
>    - Test canonical output/hash stability for JSON payloads and record expected canonical JSON/hash outputs for each fixture where the payload partition is known.
>    - Test unit metadata preservation, missing provenance findings, migration status handling, rule-pack reference preservation, and private/protected data boundary checks.
>
> 8. Review public-data safety.
>    - Confirm public fixtures are invented, original, public-domain, or permissively licensed.
>    - Confirm no protected standards text, protected tables, proprietary formulas, material allowables, copied SIF/flexibility data, or commercial examples are present.
>    - Confirm no persisted status claims automatic code compliance or professional approval.
>    - Produce or update a `public_fixture_data_boundary_review` record (PROPOSAL; final path TBD) with fixture name, source/provenance, redistribution status, contributor certification, protected-content review status, private-data review status, and professional-boundary review status.
>
> 9. Record open items.
>    - Keep physical project package/container, migration framework/tooling, binary asset packaging, exact schema file layout, public API transport, dependency versions, diagnostic code names, status labels, and hash payload partitioning as TBD unless resolved by human/architecture decision.
>    - Record the open-decision artifact path as TBD until the repo-level ADR/open-decision convention is approved; AB-00-01 requires decision records, and OI-011 assigns physical container/migration details to PKG-02 / human architecture decision.
>

### CLM-030 — Verification

> ##### Verification
>
> | Check | Expected result |
> |---|---|
> | Scope check | Artifacts address DEL-02-05 only and do not implement solver, GUI, reporting, rule evaluator, or external transport behavior. |
> | Schema check | Project fixtures validate against JSON Schema 2020-12 contracts; invalid fixtures fail deterministically; each logical project-envelope slot is covered or explicitly marked TBD. |
> | Round-trip check | Required model, unit, load, rule-pack reference, provenance, and reproducibility metadata survives round trip without semantic loss. |
> | Equality check | The test plan states per-category equality criteria for model content, unit metadata, loads, rule-pack references, provenance metadata, and reproducibility metadata. |
> | Canonicalization check | JSON payloads produce stable canonical serialization and stable JCS-compatible hash inputs; the payload/manifest partition is documented or marked TBD. |
> | Migration check | Unsupported, stale, and failed migration cases produce explicit migration status or diagnostics; newer/current status labels remain TBD unless approved. |
> | Unit/provenance check | Missing/inconsistent unit metadata and missing/weak provenance are surfaced as findings. |
> | Rule-pack reference check | Rule-pack reference ID/name, version, checksum, source note, and private/public or redistribution status survive round trip without exposing protected rule content. |
> | Boundary check | Public fixtures contain no protected standards/code data, no proprietary values, no private rule-pack expansion, and no compliance/certification claims. |
> | Service-contract check | Create/open/save/validate/version-check/migrate failures return structured diagnostics/result envelopes with affected objects and remediation where applicable. |
> | Review-record check | Public fixture/data-boundary review evidence names provenance, redistribution status, protected-content review, private-data review, and professional-boundary review disposition. |
>

### CLM-031 — Records

> ##### Records
>
> Retain or produce:
>
> - project file schema and schema-version notes;
> - persistence service contract;
> - project-envelope field inventory and schema-version notes;
> - round-trip fixture inventory and expected canonical/hash outputs;
> - expected semantic equality criteria by fixture category;
> - validation diagnostics examples for invalid units, missing provenance, rule-pack reference failures, unsupported/stale/failed migration status, protected/private data warnings, and professional-boundary warnings;
> - open-decision log for physical container, migration tooling, schema layout, dependency versions, diagnostic code names, status labels, and hash payload partitioning, with artifact path TBD until approved;
> - `public_fixture_data_boundary_review` record (PROPOSAL; final path TBD) or equivalent review evidence showing that no protected standards/code data, proprietary values, private data leakage, or professional approval claims were introduced.

- **VER-001** — Validate the contract and review source parity, round-trip semantic preservation, canonical serialization and hash behavior, version diagnostics, private-data and protected-content boundaries, and absence of professional-approval claims.

## Governing Values and Decisions — Axiology

### CLM-032 — Guidance - DEL-02-05 Project Persistence and Round-Trip Serialization

> #### Guidance - DEL-02-05 Project Persistence and Round-Trip Serialization
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-033 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-02-05-DECL-003`.
>

### CLM-034 — PDU-024 Version Handling Guidance

> ##### PDU-024 Version Handling Guidance
>
> Do not compare UI model-document versions against duplicated literals. Use `evaluateModelDocumentLocal` and `modelDocumentVersionCheckStatus`, mirroring the backend DEC-019/DEC-033 evaluator. Treat `0.2.0` as current; treat a migratable `0.1.0` source as stale at version-check grain while retaining `migrated` in migration evidence. Unsupported, newer, and failed versions must remain explicit and must not be coerced.
>

### CLM-035 — Purpose

> ##### Purpose
>
> DEL-02-05 exists to make project files auditable, deterministic, and reusable across the OpenPipeStress workflow. The persistence layer is the handoff surface between domain schemas, unit-aware modeling, rule-pack references, solver inputs, reports, automation, and future adapters. Source: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-050 and OBJ-012.
>

### CLM-036 — Principles

> ##### Principles
>
> - Treat the project file as a versioned domain artifact, not as an implementation dump. Required persisted content must be described by schema and service contracts. Source: `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04.
> - Preserve semantics, not formatting accidents. Round-trip success means required project content is preserved and canonical JSON/hash behavior is deterministic. Source: `docs/PRD.md` section 10 FR-001; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04.
> - Keep units explicit. Every numerical value that can affect solving, rule checks, reports, or exports needs unit metadata or a traceable unit-system reference. Source: `docs/CONTRACT.md` OPS-K-UNIT-1.
> - Preserve provenance. Missing provenance is a finding or warning, not a reason to invent source information. Source: `docs/CONTRACT.md` OPS-K-DATA-2 and OPS-K-DATA-3; `docs/TYPES.md` section 5.
> - Store rule-pack references and checksums without bundling protected rule content into public project examples. Source: `docs/SPEC.md` section 6; `docs/IP_AND_DATA_BOUNDARY.md` sections 3, 6, and 7.
> - Route persistence through application/service/storage boundaries that keep validation and diagnostics enforceable. Source: `docs/SPEC.md` section 1; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-02, AB-00-03, AB-00-06, and AB-00-07.
> - Separate software-computed state from professional approval. Persistence may record user-rule check states or human review records only with appropriate authority labels and hashes; it must not turn software output into code compliance. Source: `docs/TYPES.md` section 4; `docs/CONTRACT.md` OPS-K-AUTH-1 and OPS-K-AUTH-2.
>

### CLM-037 — Considerations

> ##### Considerations
>
> - Use a schema-versioned project envelope with explicit document kind, project identity, schema version, unit-system reference, migration status, and payload sections. ASSUMPTION: this envelope shape is a suitable implementation pattern; the exact schema file layout remains TBD under SCA-001.
> - Make canonicalization scope explicit before hashing. A hash over volatile, environment-specific, or session-only fields will undermine reproducibility. The exact hash payload subset is TBD; the binding basis is canonical JSON with JCS-compatible canonicalization for JSON payloads.
> - Preserve unknowns as `TBD`, `UNKNOWN_SOURCE`, warnings, or diagnostics rather than filling defaults. This applies especially to units, provenance, rule-pack references, and migration status.
> - Keep public fixtures small and invented. They can verify schema shape and round-trip behavior without exposing protected standards data, proprietary values, or commercial software examples.
> - Represent private data boundaries in the contract. Project files may reference private libraries and rule packs, but the public project must not assume those files are redistributable.
> - Design diagnostics for machine and user review. Open/save/migrate failures should identify affected objects, source, severity, remediation, and provenance rather than returning unstructured text only.
> - Avoid overclaiming SCA-001. The architecture basis resolves JSON Schema 2020-12, canonical JSON/JCS-compatible hash basis, schema-first envelopes, and test-gate classes; it does not resolve exact dependency versions, physical container, migration framework, or binary packaging.
> - Record unresolved persistence decisions in an ADR/open-decision mechanism before implementation depends on them. Source: `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-01, section 8.2, and OI-011.
> - Treat local-first behavior as a boundary condition for persistence, not only as a publication policy. Create/open/save must not imply cloud storage, public commit, telemetry, or redistribution of private project/rule/library data by default; the exact split between persistence checks and PKG-12 privacy controls remains TBD. Source: `docs/CONTRACT.md` OPS-K-PRIV-1; `docs/PRD.md` sections 18.1-18.3.
>

### CLM-038 — Vocabulary Normalization

> ###### Vocabulary Normalization
>
> Use these terms consistently until a human/architecture decision replaces them:
>
> | Term | Use in DEL-02-05 | Avoid using it to mean |
> |---|---|---|
> | Project document | The logical persisted project content governed by schema and round-trip requirements. | A specific file extension or physical package format. |
> | Project envelope | The versioned top-level contract containing metadata, payload slots, diagnostics/migration status, and reproducibility metadata. | A final schema file path or code-generated type. |
> | Project file | A user-facing persistence artifact created/opened/saved by the product. | A commitment to a single JSON file. |
> | Project package/container | The physical storage choice, such as single file versus packaged container. | A settled decision; this remains TBD under SCA-001/OI-011. |
> | Input manifest | A reproducibility record that identifies hash inputs, referenced artifacts, and private/rule/library checksums where applicable. | A report approval or compliance claim. |
>

### CLM-039 — Trade-offs

> ##### Trade-offs
>
> | Decision area | Conservative guidance | Why |
> |---|---|---|
> | Single JSON file vs packaged project container | Keep the physical package/container TBD until a human/architecture decision resolves it. | SCA-001 explicitly leaves the container unresolved. |
> | Schema file layout and code generation | Record the exact schema layout/tooling decision in an ADR or open-decision log before implementation depends on it. | SCA-001 selects JSON Schema 2020-12 but leaves exact schema file layout and code-generation tooling TBD. |
> | Embedded rule-pack data vs references | Prefer references with version/checksum/source notes for persistence; avoid embedding protected or private rule content in public examples. | Maintains code-neutral and private-data boundaries. |
> | Human-readable JSON vs strict canonical form | Permit authoring/readability where useful, but tests should compare normalized semantic content and canonical JSON/hash output. | Determinism is required for reproducibility. |
> | Full-file hash vs payload/manifests | Use canonical JSON for JSON payload hashes and manifest hashes for non-JSON/binary assets; exact split is TBD. | Matches SCA-001 without deciding binary package structure prematurely. |
> | Migration now vs migration framework later | Require schema version and migration status now; leave migration framework/tooling TBD. | This records compatibility boundaries without inventing implementation details. |
> | Local project operations vs public sharing | Make create/open/save local-first by default; treat public export, bug-report attachment, telemetry, and repository commit as separately reviewed actions. | PRD section 18 and OPS-K-PRIV-1 prohibit default transmission or public commitment of private project data. |
>

### CLM-040 — Examples

> ##### Examples
>
> TBD. No deliverable-specific project file example was available in the accessible sources. Any future public fixture should use invented or permissively licensed data, include provenance/redistribution fields where data records exist, and avoid protected standards/code content.
>
> Future fixtures should also carry a durable review record with, at minimum, source name/location, license or redistribution basis, contributor certification, review status, and confirmation that the fixture does not include protected standards/code data, proprietary values, or professional approval claims. Source: `docs/IP_AND_DATA_BOUNDARY.md` sections 2-5; `docs/CONTRACT.md` OPS-K-IP-1 through OPS-K-IP-3 and OPS-K-AUTH-1.
>

### CLM-041 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None | No source conflict affecting DEL-02-05 content was identified during Pass 1+2. Open items are tracked as TBD rather than conflicts. | N/A | N/A | N/A | N/A | N/A |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-050 SOW-041 OBJ-001 OBJ-012 | CLM-010 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
