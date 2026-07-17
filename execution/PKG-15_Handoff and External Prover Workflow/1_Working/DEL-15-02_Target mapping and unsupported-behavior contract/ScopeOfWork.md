---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-15-02
package_id: PKG-15
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@e8f59a63372f38d9e788ac39b39995558f5aba73
project_scope_refs: [SOW-074]
package_objective_refs: [OBJ-017]
---

# Scope of Work — DEL-15-02

## Purpose and Objective Traceability

This Scope of Work defines `DEL-15-02` in service of project scope [SOW-074] and package objectives [OBJ-017].

- **OUT-001** — A provider-neutral target-mapping and unsupported or approximate behavior contract for handoff exports is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-15-02 Target mapping and unsupported-behavior contract

> #### Datasheet: DEL-15-02 Target mapping and unsupported-behavior contract
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-15-02-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-15-02 |
> | Name | Target mapping and unsupported-behavior contract |
> | Package ID | PKG-15 |
> | Package name | Handoff and External Prover Workflow |
> | Type | API_CONTRACT |
> | Scope coverage | SOW-074 |
> | Objective support | OBJ-017 |
> | Anticipated artifacts | `schemas/target_mapping.schema.json`; provider-neutral unsupported behavior taxonomy |
> | Source basis | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md`; `docs/_Registers/Deliverables.csv`; `docs/_Registers/ScopeLedger.csv`; `Dependencies.csv` |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Value | Source |
> |---|---|---|
> | Deliverable purpose | Defines metadata for how internal entities map to target fields and how unsupported or approximate target behavior is flagged. | `_CONTEXT.md` Description; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-15-02 row |
> | Package purpose | Implements handoff package structure, manifests, target mapping metadata, unsupported-target flags, and non-authoritative external-prover metadata boundaries. | `_CONTEXT.md` Package Reference; `execution/_Decomposition/SOFTWARE_DECOMP.md` PKG-15 row |
> | Required handoff contents in scope | Schema-compliant handoff packages include model hash, units manifest, entity IDs, library/rule references, unresolved assumptions, warnings, target mapping metadata, and unsupported-target flags. | `_CONTEXT.md` Scope Detail; `docs/_Registers/ScopeLedger.csv` SOW-074 |
> | Target-specific commercial parsers | Deferred. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-074 note; `docs/_Registers/ScopeLedger.csv` SOW-074 note |
> | External professional approval state | Excluded from this package. | `_CONTEXT.md` Package Exclusions; `execution/_Decomposition/SOFTWARE_DECOMP.md` PKG-15 exclusions |
> | Target surfaces | OI-015 names initial export and target surfaces; concrete mappings, target field coverage, target-specific taxonomy extensions, and target-specific implementation remain gated by DEL-17-01 and DEL-17-02. | `execution/_Decomposition/SOFTWARE_DECOMP.md` OI-015 |
> | Canonical package container | TBD. | `execution/_Decomposition/SOFTWARE_DECOMP.md` OI-015 |
> | Target-specific mapping strategy | TBD. | `execution/_Decomposition/SOFTWARE_DECOMP.md` OI-015 |
>

### CLM-005 — Conditions

> ##### Conditions
>
> | Condition | Value | Source |
> |---|---|---|
> | Units | Exports must remain unit-aware and dimensionally checked. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/DIRECTIVE.md` Principles |
> | Missing values | Missing solve-required or rule-check-required values are explicit findings, not silent defaults. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/DIRECTIVE.md` Principles |
> | Provenance | Reliance-affecting data carries source/provenance fields and review status. | `docs/CONTRACT.md` OPS-K-IP-2; `docs/CONTRACT.md` OPS-K-DATA-3; `docs/TYPES.md` Provenance |
> | Private data boundary | Private project, material, component, rule-pack, owner-standard, and company design-basis data are user-controlled and excluded from public surfaces by default. | `docs/IP_AND_DATA_BOUNDARY.md` Private user data; `docs/CONTRACT.md` OPS-K-PRIV-1 |
> | Professional boundary | Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/DIRECTIVE.md` Principles |
> | Handoff role | Shell/solid FEA remains a specialized local-analysis handoff path, not the primary global analysis model. | `docs/CONTRACT.md` OPS-K-MECH-1; `docs/DIRECTIVE.md` Principles |
>

### CLM-006 — Construction

> ##### Construction
>
> | Artifact | Status | Construction notes |
> |---|---|---|
> | Target mapping schema | MATERIALIZED | `schemas/target_mapping.schema.json` is the JSON Schema 2020-12 provider-neutral contract matching `core/handoff/target_mapping/contract.py`; exact commercial target enumeration remains TBD. |
> | Unsupported behavior taxonomy | MATERIALIZED PROVIDER-NEUTRAL | `unsupported_behavior_flags`, `approximate_behavior_flags`, `SUPPORTED_BEHAVIOR_STATUSES`, and diagnostics expose unsupported and approximate behavior without silent loss of assumptions; target-specific taxonomy extensions remain TBD. |
> | Mapping record identity | MATERIALIZED | Mapping records are traceable through `source_entity_ref`, `target_field_ref`, `mapping_status`, optional unit metadata, and provenance fields. |
> | Unsupported/approximate behavior record | MATERIALIZED | Behavior flags carry `behavior_kind`, `behavior_status`, severity, description, source/target references, diagnostics, and provenance. |
> | Hash and manifest binding | MATERIALIZED | `source_context` binds mapping metadata to package/model/unit/entity/library/rule references while preserving target/container TBDs. |
>

### CLM-007 — References

> ##### References
>
> - `_CONTEXT.md` - deliverable identity, scope, package envelope, architecture-basis injection.
> - `_REFERENCES.md` - local reference index.
> - `Dependencies.csv` - approved DAG-006 mirror/evidence surface for predecessor context.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` - accepted revision 0.7 current decomposition basis, PKG-15 and DEL-15-02 rows.
> - `docs/_Registers/Deliverables.csv` - DEL-15-02 row.
> - `docs/_Registers/ScopeLedger.csv` - SOW-074 row.
> - `docs/_Registers/ContextBudgetQA.csv` - DEL-15-02 row.
> - `docs/CONTRACT.md` - invariant catalog.
> - `docs/DIRECTIVE.md` - product principles.
> - `docs/SPEC.md` - adapter/export and boundary specification slices.
> - `docs/TYPES.md` - canonical object and boundary vocabulary.
> - `docs/IP_AND_DATA_BOUNDARY.md` - private-data and public-surface boundary policy.

### CLM-008 — D-41 R5 T3 PDU-017 Evidence State

> ##### D-41 R5 T3 PDU-017 Evidence State
>
> Selected seam: target-mapping builder. Unsafe privacy context produces blocking diagnostics; broader runtime redaction reach remains separately owned.

## Completion and Reliance Basis — Epistemology

### CLM-009 — Specification: DEL-15-02 Target mapping and unsupported-behavior contract

> #### Specification: DEL-15-02 Target mapping and unsupported-behavior contract
>

### CLM-010 — Scope

> ##### Scope
>
> This deliverable defines an API contract for target mapping metadata and unsupported or approximate behavior flags used by handoff exports. It covers the target mapping schema and unsupported behavior taxonomy identified in `_CONTEXT.md`.
>
> This deliverable excludes target-specific commercial parser implementation, automatic external-prover approval states, professional acceptance records, and comprehensive commercial-tool result ingestion; the contract is handoff evidence for external validation, not a validation outcome. These exclusions are grounded in the PKG-15 package exclusion and the professional-boundary constraints in `docs/CONTRACT.md`.
>

### CLM-011 — Requirements

> ##### Requirements
>
> | Req ID | Requirement | Source | Verification |
> |---|---|---|---|
> | DEL-15-02-R001 | The contract shall support target mapping metadata for handoff exports. | `_CONTEXT.md` Description; `docs/_Registers/Deliverables.csv` DEL-15-02 | Schema review confirms a target mapping metadata surface exists. |
> | DEL-15-02-R002 | The contract shall support unsupported-target flags. | `_CONTEXT.md` Scope Detail; `docs/_Registers/ScopeLedger.csv` SOW-074 | Schema review confirms unsupported-target flags are representable. |
> | DEL-15-02-R003 | The contract shall support approximate behavior disclosure when target behavior cannot be represented exactly. | `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-15-02 row | `schemas/target_mapping.schema.json` and `SUPPORTED_BEHAVIOR_STATUSES` cover provider-neutral explicit statuses; target-specific taxonomy values remain TBD. |
> | DEL-15-02-R004 | The contract shall preserve unresolved assumptions and warnings in the handoff context. | `_CONTEXT.md` Scope Detail; `docs/_Registers/ScopeLedger.csv` SOW-074 | Contract review traces mapping or unsupported records to warnings and unresolved assumptions where applicable. |
> | DEL-15-02-R005 | The contract shall preserve unit awareness and dimensional-check boundaries for exported values. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/DIRECTIVE.md` Principles | Schema or validation plan includes unit/dimension metadata or diagnostics for unit-bearing mapped values. |
> | DEL-15-02-R006 | The contract shall preserve source, provenance, redistribution/private-public status, and review status for reliance-affecting references where present. | `docs/CONTRACT.md` OPS-K-IP-2; `docs/CONTRACT.md` OPS-K-DATA-3; `docs/TYPES.md` Provenance | Contract review confirms provenance fields or references are available; private payload copying remains prohibited. |
> | DEL-15-02-R007 | The contract shall not silently replace missing or unsupported values with defaults. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/DIRECTIVE.md` Principles | Validation plan includes blocking diagnostics or explicit TBD/unsupported findings for missing required data. |
> | DEL-15-02-R008 | The contract shall not embed protected standards text, protected tables, proprietary values, private formulas, or private rule-pack payloads into public artifacts. | `docs/IP_AND_DATA_BOUNDARY.md` Private user data; `docs/SPEC.md` result export boundary | Protected-content review confirms only references, checksums, source notes, and allowed metadata are exposed. |
> | DEL-15-02-R009 | The contract shall not create software-generated professional approval, certification, sealing, authentication, or code-compliance statuses (PRD §21.2). | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/SPEC.md` adapter framework boundary | Vocabulary review confirms status fields are diagnostic/handoff support only and do not imply professional reliance approval. |
> | DEL-15-02-R010 | The contract shall remain compatible with the canonical handoff package schema and manifest predecessor. | `Dependencies.csv` DAG-002-E0805; `_DEPENDENCIES.md` authority boundary | Dependency trace confirms DEL-15-01 predecessor relationship remains visible and ACTIVE in the approved mirror. |
> | DEL-15-02-R011 | The contract shall preserve redaction/export-control boundaries. | `Dependencies.csv` DAG-002-E0808; `docs/IP_AND_DATA_BOUNDARY.md` Private user data | Schema review confirms private data can be excluded or referenced without public disclosure; exact redaction workflow is TBD. |
> | DEL-15-02-R012 | The contract shall keep concrete mappings, target field coverage, target-specific taxonomy extensions, canonical package container, and target-specific implementation gated until accepted by the governing workflow. | `execution/_Decomposition/SOFTWARE_DECOMP.md` OI-015; DEL-17-01; DEL-17-02 | Review confirms no unapproved target-specific mapping maturity, package container, or implementation strategy has been asserted. |
>

### CLM-012 — Standards

> ##### Standards
>
> | Standard or governing basis | Applicability | Status |
> |---|---|---|
> | JSON Schema 2020-12 | The architecture basis identifies JSON Schema 2020-12 contracts for schema-first boundaries. | Applicable; schema file path is `schemas/target_mapping.schema.json`. |
> | Canonical JSON / JCS-compatible hash basis | The architecture basis identifies canonical JSON/JCS-compatible hashing where JSON payloads are hashed. | Applicable to hashed JSON payloads; provider-neutral binding fields are in `source_context`, while package container remains TBD. |
> | OpenPipeStress invariant catalog | Governs unit, provenance, data, professional-boundary, report, and privacy constraints. | Applicable via `docs/CONTRACT.md`. |
> | Protected data boundary policy | Governs public/private data handling, protected content, and public export defaults. | Applicable via `docs/IP_AND_DATA_BOUNDARY.md`. |
> | External commercial tool standards | Not established in accessible source material. | TBD; do not infer target-specific clauses or behavior. |
>

### CLM-013 — Verification

> ##### Verification
>
> | Verification ID | Approach | Covers |
> |---|---|---|
> | V-001 | Schema inspection plus `python3 tests/test_target_mapping_contract.py` against `DEL-15-02-R001` through `DEL-15-02-R004`. | Target mapping metadata, unsupported-target flags, approximate behavior, warnings, assumptions. |
> | V-002 | Boundary vocabulary review. | No professional approval/status overclaim; no code-compliance claim (PRD §21.2). |
> | V-003 | Protected-content and privacy review. | No private or protected data copied into public artifacts. |
> | V-004 | Unit/provenance validation plan review. | Unit awareness, dimensional metadata, source/provenance. |
> | V-005 | Dependency mirror check. | Approved DAG-002 predecessor evidence remains ACTIVE and unmodified. |
> | V-006 | TBD review gate. | Concrete mappings, target field coverage, target-specific taxonomy extensions, canonical package container, and target-specific implementation remain gated unless supported by accepted source material. |
>

### CLM-014 — Documentation

> ##### Documentation
>
> Required deliverable documentation artifacts:
>
> - `Datasheet.md`
> - `Specification.md`
> - `Guidance.md`
> - `Procedure.md`
> - `schemas/target_mapping.schema.json`
> - `core/handoff/target_mapping/contract.py`
> - `tests/test_target_mapping_contract.py`
> - validation evidence for schema, protected-content boundary, and professional-boundary wording

### CLM-015 — D-41 R5 T3 PDU-017 O7/E5 Requirement (2026-07-12)

> ##### D-41 R5 T3 PDU-017 O7/E5 Requirement (2026-07-12)
>
> The builder SHALL emit blocking privacy diagnostics when redaction is not affirmed or embedded private/protected/commercial payload is reported. Metadata shape alone is not enforcement.

- **AC-001** — The contract preserves mapping identity, source and target references, mapping and behavior statuses, unit and dimensional metadata, assumptions, warnings, diagnostics, provenance, privacy and redaction boundaries, explicit no-silent-default behavior, target-specific mapping and taxonomy gates, package-container TBDs, the retained dependency-enum conflict, and professional non-authority without implying target equivalence or validation.

## Production and Verification Method — Praxeology

### CLM-016 — Procedure: DEL-15-02 Target mapping and unsupported-behavior contract

> #### Procedure: DEL-15-02 Target mapping and unsupported-behavior contract
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-017 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-15-02-DECL-004`.
>

### CLM-018 — Purpose

> ##### Purpose
>
> Define and check the target mapping and unsupported-behavior contract for handoff exports without inventing target-specific behavior, copying private/protected data, or implying professional approval.
>

### CLM-019 — Prerequisites

> ##### Prerequisites
>
> | Prerequisite | Source | Status |
> |---|---|---|
> | Current deliverable context and accepted decomposition basis. | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` | Available |
> | Approved DAG-002 mirror/evidence surface. | `Dependencies.csv`; `_DEPENDENCIES.md` | Available; preserve rows unchanged in this run. |
> | Canonical handoff package schema and manifest predecessor. | `Dependencies.csv` DAG-002-E0805 | ACTIVE mirror row |
> | Adapter framework predecessor. | `Dependencies.csv` DAG-002-E0806 | ACTIVE mirror row |
> | Local FEA handoff data contract predecessor. | `Dependencies.csv` DAG-002-E0807 | ACTIVE mirror row |
> | Private data redaction and export controls predecessor. | `Dependencies.csv` DAG-002-E0808 | ACTIVE mirror row |
> | Physical-to-analytical transformation predecessor. | `Dependencies.csv` DAG-002-E0809 | ACTIVE mirror row |
> | Comparison mapping, tolerance, and export contracts predecessor. | `Dependencies.csv` DAG-002-E0810 | ACTIVE mirror row |
> | Concrete mappings, target field coverage, target-specific taxonomy extensions, canonical package container, and target-specific implementation. | `execution/_Decomposition/SOFTWARE_DECOMP.md` OI-015; DEL-17-01; DEL-17-02 | Gated |
>

### CLM-020 — Steps

> ##### Steps
>
> 1. Confirm the deliverable identity:
>    - Use DEL-15-02, PKG-15, API_CONTRACT, SOW-074, and OBJ-017 from `_CONTEXT.md`.
>
> 2. Confirm governing boundaries:
>    - Apply unit awareness, provenance, no silent defaults, professional-boundary, protected-content, and private-data constraints from `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, `docs/SPEC.md`, `docs/TYPES.md`, and `docs/IP_AND_DATA_BOUNDARY.md`.
>
> 3. Define the contract scope:
>    - Include target mapping metadata.
>    - Include unsupported-target flags.
>    - Include approximate behavior disclosure as an explicit ASSUMPTION from the decomposition wording.
>    - Use `schemas/target_mapping.schema.json` and `core/handoff/target_mapping/contract.py` for provider-neutral schema path, property names, and taxonomy values.
>    - Keep exact commercial target names and target-specific taxonomy/strategy values as TBD unless accepted source material resolves them.
>
> 4. Map upstream handoff context:
>    - Treat the canonical handoff package schema and manifest as a required predecessor.
>    - Preserve model hash, units manifest, entity IDs, library/rule references, unresolved assumptions, and warnings as handoff context where the final contract supports them.
>
> 5. Define unsupported behavior handling:
>    - Record unsupported or approximate behavior as explicit flags, findings, diagnostics, or TBD slots.
>    - Do not coerce unsupported behavior into defaults.
>    - Do not claim target-tool equivalence, validation, certification, approval, or code compliance (PRD §21.2).
>
> 6. Define privacy and protected-content handling:
>    - Prefer stable references, checksums, source notes, review state, and redaction status.
>    - Exclude private project data, protected standards text, proprietary formulas, proprietary values, and private rule-pack payloads from public artifacts by default.
>
> 7. Check dependencies:
>    - Read the deliverable-local `Dependencies.csv` as an approved DAG-006 mirror/evidence surface.
>    - Preserve all approved DAG-006 rows as ACTIVE for this workflow.
>    - Record any conflict with dependency-extract normalization rules rather than rewriting the mirror.
>
> 8. Record unresolved decisions:
>    - Initial export and target surfaces: named by OI-015.
>    - Canonical package container: TBD.
>    - Concrete mappings and target field coverage: gated by DEL-17-01 and DEL-17-02.
>    - Target-specific implementation strategy: gated by DEL-17-01 and DEL-17-02.
>    - Target-specific unsupported behavior taxonomy extensions: gated by DEL-17-01 and DEL-17-02.
>    - Provider-neutral schema path and property names: materialized in `schemas/target_mapping.schema.json`.
>

### CLM-021 — Verification

> ##### Verification
>
> | Check | Expected result |
> |---|---|
> | Four-document consistency | Datasheet, Specification, Guidance, and Procedure use the same deliverable ID, package ID, scope item, objective, and TBD boundaries. |
> | Source grounding | Non-trivial claims cite `_CONTEXT.md`, decomposition/registers, approved dependency mirror, or listed governing docs. |
> | Unsupported behavior | Unsupported and approximate behavior are explicit; no silent defaults are introduced. |
> | Privacy/IP boundary | Public artifacts do not copy private/protected payloads. |
> | Professional boundary | No field or procedure creates software-generated approval, certification, sealing, authentication, or code-compliance status (PRD §21.2). |
> | Dependency mirror preservation | All existing approved DAG-006 rows in `Dependencies.csv` remain ACTIVE and unmodified. |
> | Schema validation | `python3 tests/test_target_mapping_contract.py` validates generated normal and negative contracts against `schemas/target_mapping.schema.json`; `python3 tools/validation/validate_dependencies_schema.py <deliverable>/Dependencies.csv` passes if `Dependencies.csv` exists. |
>

### CLM-022 — Records

> ##### Records
>
> - `Datasheet.md`
> - `Specification.md`
> - `Guidance.md`
> - `Procedure.md`
> - `_SEMANTIC.md`
> - `_SEMANTIC_LENSING.md`
> - `Dependencies.csv`
> - `_DEPENDENCIES.md`
> - `schemas/target_mapping.schema.json`
> - `core/handoff/target_mapping/contract.py`
> - `tests/test_target_mapping_contract.py`
> - final workflow report from this setup run

### CLM-023 — D-41 R5 T3 PDU-017 Check

> ##### D-41 R5 T3 PDU-017 Check
>
> Exercise the builder with unsafe privacy flags and require blocking `PRIVACY_WARNING` diagnostics; never silently normalize unsafe input into a safe claim.

- **VER-001** — Validate the contract and review source parity, target-mapping and unsupported/approximate behavior coverage, unit/provenance and redaction controls, blocking privacy diagnostics, no-silent-default behavior, retained OI-015 and dependency conflict boundaries, upstream interface visibility, and prohibited approval, certification, compliance, or equivalence claims.

## Governing Values and Decisions — Axiology

### CLM-024 — Guidance: DEL-15-02 Target mapping and unsupported-behavior contract

> #### Guidance: DEL-15-02 Target mapping and unsupported-behavior contract
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-025 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-15-02-DECL-003`.
>

### CLM-026 — Purpose

> ##### Purpose
>
> This deliverable prevents silent loss of critical handoff assumptions or unsupported target behavior. It supplies the contract surface for explaining how internal OpenPipeStress entities map into a handoff target and where a target cannot carry the source semantics exactly.
>

### CLM-027 — Principles

> ##### Principles
>
> | Principle | Guidance | Source |
> |---|---|---|
> | Explicit non-support | Unsupported target behavior must be visible as a finding or flag, not hidden by defaults. | `_CONTEXT.md` Context Envelope; `docs/CONTRACT.md` OPS-K-DATA-2 |
> | Unit safety | Unit-bearing values and exports must preserve unit and dimensional metadata or diagnostics. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/DIRECTIVE.md` Principles |
> | Provenance preservation | Mapped entities, rule/library references, and reliance-affecting metadata should carry source/provenance references where supported by upstream contracts. | `docs/CONTRACT.md` OPS-K-IP-2; `docs/TYPES.md` Provenance |
> | Private-data restraint | Public handoff artifacts should expose references, checksums, review status, and allowed metadata without copying private or protected payloads. | `docs/IP_AND_DATA_BOUNDARY.md` Private user data; `docs/SPEC.md` result export boundary |
> | Professional-boundary restraint | Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). | `docs/CONTRACT.md` OPS-K-AUTH-1; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEC-015 |
> | Deferred target specificity | OI-015 names initial export and target surfaces. Concrete mappings, target field coverage, target-specific taxonomy extensions, and target-specific implementation remain gated by DEL-17-01 and DEL-17-02. | `execution/_Decomposition/SOFTWARE_DECOMP.md` OI-015 |
>

### CLM-028 — Considerations

> ##### Considerations
>
> - Treat target mapping as traceability and disclosure metadata. It does not prove that the target tool reproduced the source model correctly.
> - Treat unsupported and approximate behavior flags as review evidence. They should help a human identify gaps, not resolve those gaps automatically.
> - Favor references to rule packs, libraries, manifests, diagnostics, and hashes over copying private formulas, protected standards text, proprietary values, or private rule-pack payloads.
> - Use `schemas/target_mapping.schema.json` as the provider-neutral schema contract and `core/handoff/target_mapping/contract.py` as the current generated-output implementation surface.
> - Use the materialized provider-neutral behavior fields and statuses for current handoff review. Target-specific mappings, taxonomy extensions, and field coverage remain gated by DEL-17-01 and DEL-17-02.
>

### CLM-029 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Conservative position |
> |---|---|
> | Generic target contract vs target-specific convenience | Prefer the current provider-neutral schema-first metadata until accepted target-specific export/profile work is in scope. |
> | Rich mapping details vs private-data exposure | Preserve traceability and review evidence while excluding private/protected payloads by default. |
> | Approximation support vs false confidence | Allow approximate behavior disclosure, but do not let it imply equivalence, validation, approval, or compliance. |
> | Automated workflow vs human authority | The software may package and disclose handoff evidence; professional reliance remains external and human-owned. |
>

### CLM-030 — Examples

> ##### Examples
>
> | Example topic | Status |
> |---|---|
> | Provider-neutral target mapping record | Materialized in `schemas/target_mapping.schema.json` and validated by `tests/test_target_mapping_contract.py`. |
> | Unsupported behavior taxonomy values | Provider-neutral behavior statuses and flag records are materialized; target-specific taxonomy extensions remain gated by DEL-17-01 and DEL-17-02. |
> | Approximate behavior flag semantics | Materialized as explicit provider-neutral approximate behavior disclosure; target-specific interpretation remains gated. |
>

### CLM-031 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict (short statement) | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling (TBD) |
> |---|---|---|---|---|---|---|
> | CT-001 | Dependency-extract v3.1 enum guidance would normalize dependency rows, but the project task rule requires preserving approved DAG-006 mirror rows as ACTIVE without reclassification. | `skills/dependency-extract/SKILL.md` Function 3 and canonical enums | User task instruction; `_DEPENDENCIES.md` Authority Boundary; `Dependencies.csv` approved DAG-006 rows | `Dependencies.csv`; `_DEPENDENCIES.md`; final workflow report | Preserve the approved mirror unchanged for this run. | TBD |
>

### CLM-032 — References

> ##### References
>
> - `_CONTEXT.md`
> - `_REFERENCES.md`
> - `Dependencies.csv`
> - `execution/_Decomposition/SOFTWARE_DECOMP.md`
> - `docs/CONTRACT.md`
> - `docs/DIRECTIVE.md`
> - `docs/SPEC.md`
> - `docs/TYPES.md`
> - `docs/IP_AND_DATA_BOUNDARY.md`

### CLM-033 — D-41 R5 T3 PDU-017 Boundary

> ##### D-41 R5 T3 PDU-017 Boundary
>
> Preserving privacy metadata is insufficient when it says unsafe payload is embedded. The selected builder seam blocks that state without claiming downstream security sufficiency.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-074 OBJ-017 | CLM-009 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
