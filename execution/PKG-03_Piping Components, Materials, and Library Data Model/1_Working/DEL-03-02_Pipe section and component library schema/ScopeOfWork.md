---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-03-02
package_id: PKG-03
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@69ac259a7113d5a838fb22aa2e84df0e0f109713
project_scope_refs: [SOW-018]
package_objective_refs: [OBJ-004]
---

# Scope of Work — DEL-03-02

## Purpose and Objective Traceability

This Scope of Work defines `DEL-03-02` in service of project scope [SOW-018] and package objectives [OBJ-004].

- **OUT-001** — A pipe-section and component-library schema contract covering section geometry and properties, component identity, units, provenance, library governance, and explicit completeness diagnostics is produced for the declared scope and objective.

## Deliverable Definition — Ontology

### CLM-001 — Source preamble

> ---
> doc_id: DEL-03-02-DATASHEET
> doc_kind: deliverable.datasheet
> status: draft
> created: 2026-04-30
> deliverable_id: DEL-03-02
> package_id: PKG-03
> ---
>

### CLM-002 — Datasheet: Pipe Section and Component Library Schema

> #### Datasheet: Pipe Section and Component Library Schema
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value | Evidence |
> |---|---|---|
> | Deliverable ID | DEL-03-02 | SourcePath: `_CONTEXT.md`; SectionRef: Context: DEL-03-02 |
> | Name | Pipe section and component library schema | SourcePath: `_CONTEXT.md`; SectionRef: Context: DEL-03-02 |
> | Package | PKG-03 - Piping Components, Materials, and Library Data Model | SourcePath: `_CONTEXT.md`; SectionRef: Package Reference |
> | Type | DATA_MODEL_CHANGE | SourcePath: `_CONTEXT.md`; SectionRef: Type |
> | Scope item | SOW-018 | SourcePath: `docs/_Registers/ScopeLedger.csv`; SectionRef: row SOW-018 |
> | Objective | OBJ-004 | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: objective row OBJ-004 |
> | Implemented schema artifacts | `schemas/component.schema.yaml`; `schemas/section.schema.yaml` | SourcePath: `schemas/component.schema.yaml`; SourcePath: `schemas/section.schema.yaml` |
> | Decomposition basis | `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 | SourcePath: `_CONTEXT.md`; SectionRef: Decomposition Reference |
> | Responsible party | TBD | Not assigned in accessible sources |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Value | Evidence |
> |---|---|---|
> | Primary subject | Private pipe section and component library records | SourcePath: `docs/_Registers/ScopeLedger.csv`; SectionRef: row SOW-018 |
> | Required public boundary | No protected pipe dimensional tables, protected standards data, or proprietary catalog data are bundled as public defaults | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-IP-1, OPS-K-DATA-1 |
> | Section record intent | User-entered dimensions, weights/mass-property inputs, provenance, and redistribution status | SourcePath: `_CONTEXT.md`; SectionRef: Description |
> | Component record intent | User-entered component dimensions, weights, centers of gravity, source/license metadata, and redistribution status | SourcePath: `_CONTEXT.md`; SectionRef: Description |
> | Schema baseline | JSON Schema 2020-12 contracts | SourcePath: `_CONTEXT.md`; SectionRef: Architecture Basis Injection |
> | Persistence baseline | Deterministic, versioned, unit-aware, provenance-preserving, schema-governed, migration-aware, and round-trip testable | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-04 |
> | Diagnostics baseline | Diagnostics/result envelopes carry code, class, severity, source, affected object, message, remediation, and provenance | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-06 |
> | Schema implementation evidence | Implemented with strict JSON Schema 2020-12 documents, split strict fixtures, combined legacy fixture pointers, and focused schema tests | SourcePath: `schemas/section.schema.yaml`; SourcePath: `schemas/component.schema.yaml`; SourcePath: `fixtures/component/invented_section_library_valid.json`; SourcePath: `fixtures/component/invented_component_library_valid.json`; SourcePath: `fixtures/component/invented_section_component_library_valid.json`; SourcePath: `tests/test_component_section_schema.py` |
>

### CLM-005 — Conditions

> ##### Conditions
>
> - Public repository artifacts must not contain protected standards text, tables, figures, examples, copied code formulas, material allowables, SIF/flexibility tables, protected dimensional tables, or proprietary commercial data. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-IP-1.
> - Public data contributions require source, provenance, license/redistribution status, contributor certification, and review disposition. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-IP-2.
> - Suspected protected content must be quarantined and escalated; agents must not paraphrase protected tables into public data. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-IP-3.
> - Code-specific and proprietary component/section values are user-supplied or lawfully imported private data, not bundled public defaults. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-DATA-1.
> - Missing solve-required or rule-check-required values are explicit findings, never silent defaults. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-DATA-2.
> - Component, section, SIF, flexibility, allowable, and rule-pack values carry provenance fields where applicable. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-DATA-3.
> - All dimensional, mass, weight, and property fields must be unit-aware and dimensionally checked. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-UNIT-1.
> - Private project, material, component, and rule-pack data must not be transmitted or committed publicly by default. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-PRIV-1.
>

### CLM-006 — Construction

> ##### Construction
>
> This deliverable now has repository-level schema implementation evidence. This reconciliation records the implemented artifacts without changing schemas, fixtures, tests, lifecycle files, dependency files, or review dispositions.
>
> | Part | Status | Notes |
> |---|---|---|
> | `schemas/section.schema.yaml` | Implemented evidence | Defines a strict pipe/section library schema with `schema_version`, `section_library`, `section_records`, `dimension_definitions`, `property_definitions`, `completeness_rules`, `diagnostics`, and `open_decisions`; it includes unit-bearing section dimension/property slots, provenance, redistribution status, review status, completeness findings, diagnostics, and open decisions. |
> | `schemas/component.schema.yaml` | Implemented evidence | Defines a strict component library schema with `schema_version`, `component_library`, `component_family_contracts`, `component_records`, `field_definitions`, `completeness_rules`, `diagnostics`, and `open_decisions`; it includes component-family contracts, user-entered field slots, protected-value policies, provenance, redistribution status, review status, completeness findings, diagnostics, and open decisions. |
> | Strict section fixture | Implemented evidence | `fixtures/component/invented_section_library_valid.json` validates as schema-shape evidence and intentionally omits engineering values through `missing` value statuses, incomplete completeness findings, and diagnostics. |
> | Strict component fixture | Implemented evidence | `fixtures/component/invented_component_library_valid.json` validates as schema-shape evidence for bend, branch, rigid, and expansion-joint component records while keeping engineering values absent or private/user-supplied only. |
> | Combined legacy fixture | Implemented pointer evidence | `fixtures/component/invented_section_component_library_valid.json` carries section/component content plus references to the strict split fixtures; strict validation relies on the split fixtures. |
> | Test coverage | Implemented evidence | `tests/test_component_section_schema.py` checks strict schema shape, provenance requirements, redistribution statuses, unit-dimension alignment with PKG-02, component enum equality with the canonical model enum, split fixture validation, and forbidden public-data text. |
> | Provenance metadata | Implemented schema mechanism; policy TBDs remain | Schemas require source/provenance, license, contributor certification, redistribution status, and review status. Accepted public source catalogs, source-license disposition, redistribution acceptance, and public contribution approval remain human/policy decisions. |
> | Unit-bearing value model | Implemented schema mechanism | Schemas define quantity-value slots with magnitude, unit, dimension, value status, and provenance; tests assert accepted dimension vocabulary alignment with `schemas/units.schema.yaml`. |
> | Protected-content gate hooks | Implemented schema/test guardrails; workflow policy TBDs remain | Schemas and fixtures represent protected-suspected/rejected states, protected-value policies, diagnostics, and no-public-value policies. Human review workflow and accepted source policy remain unresolved. |
> | Engineering values | Out of scope | No pipe dimensional table values, component catalog values, allowables, SIFs, flexibility factors, or code-derived values are introduced here. |
>

### CLM-007 — References

> ##### References
>
> - `_CONTEXT.md`, accepted basis for DEL-03-02.
> - `_REFERENCES.md`, local reference index for DEL-03-02.
> - `_DEPENDENCIES.md`, human-owned dependency declarations for DEL-03-02.
> - `docs/_Registers/Deliverables.csv`, row DEL-03-02.
> - `docs/_Registers/ScopeLedger.csv`, row SOW-018.
> - `docs/_Registers/ContextBudgetQA.csv`, row DEL-03-02.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md`, revision 0.7, especially PKG-03, DEL-03-02, OBJ-004, SOW-018, and AB-00-01/02/04/06/07/08.
> - `docs/CONTRACT.md`, invariant catalog.
> - `docs/TYPES.md`, epistemic labels and data provenance labels.
> - `docs/SPEC.md`, sections 1, 3, 10, and 11.
> - `docs/DIRECTIVE.md`, data-boundary and stop-rule basis.

## Completion and Reliance Basis — Epistemology

### CLM-008 — Source preamble

> ---
> doc_id: DEL-03-02-SPECIFICATION
> doc_kind: deliverable.specification
> status: draft
> created: 2026-04-30
> deliverable_id: DEL-03-02
> package_id: PKG-03
> ---
>

### CLM-009 — Specification: Pipe Section and Component Library Schema

> #### Specification: Pipe Section and Component Library Schema
>

### CLM-010 — Scope

> ##### Scope
>
> DEL-03-02 defines the pipe section and component library schema artifacts listed in the decomposition: `schemas/section.schema.yaml` and `schemas/component.schema.yaml`. The schema subject is private library records for user-entered dimensions, weights, centers of gravity, stiffness/modifier slots, source/license metadata, provenance, redistribution status, review status, completeness findings, diagnostics, and open decisions.
>
> The repository-level schema files, strict split fixtures, legacy combined fixture pointer, and focused schema tests are implemented evidence for this deliverable. This reconciliation does not edit those repository-level artifacts, does not add public pipe dimensional tables, does not add component catalog values, and does not resolve human-owned policy decisions for public source catalogs, fixture values, redistribution acceptance, dependency satisfaction, lifecycle, or review disposition.
>

### CLM-011 — Requirements

> ##### Requirements
>
> | ID | Requirement | Evidence |
> |---|---|---|
> | DEL-03-02-REQ-01 | The section schema supports private pipe section library records with provenance and redistribution status. | SourcePath: `schemas/section.schema.yaml`; SectionRef: top-level `section_library`, `section_records`, `$defs.Provenance`, `$defs.RedistributionStatus` |
> | DEL-03-02-REQ-02 | The component schema supports private component library records with user-entered dimensions, weights, centers of gravity, stiffness/modifier slots, source/license metadata, provenance, and redistribution status. | SourcePath: `schemas/component.schema.yaml`; SectionRef: top-level `component_library`, `component_family_contracts`, `component_records`, `field_definitions`, `$defs.ComponentFieldKind`, `$defs.Provenance` |
> | DEL-03-02-REQ-03 | Public schema and fixture artifacts do not bundle protected dimensional tables, protected standards data, proprietary catalog data, material allowables, SIF/flexibility tables, or code-derived values. | SourcePath: `tests/test_component_section_schema.py`; SectionRef: `FORBIDDEN_PUBLIC_DATA_TEXT`; SourcePath: `fixtures/component/invented_section_library_valid.json`; SourcePath: `fixtures/component/invented_component_library_valid.json` |
> | DEL-03-02-REQ-04 | Schema and fixture evidence preserve missing required values as explicit value statuses, completeness findings, diagnostics, or validation failures rather than silent defaults. | SourcePath: `schemas/section.schema.yaml`; SectionRef: `$defs.CompletenessFinding`, `$defs.SectionDiagnostic`; SourcePath: `schemas/component.schema.yaml`; SectionRef: `$defs.CompletenessFinding`, `$defs.ComponentDiagnostic`; SourcePath: `tests/test_component_section_schema.py` |
> | DEL-03-02-REQ-05 | Library records carry provenance metadata for values that may affect engineering reliance, including source and redistribution/license status. | SourcePath: `schemas/section.schema.yaml`; SectionRef: `$defs.Provenance`; SourcePath: `schemas/component.schema.yaml`; SectionRef: `$defs.Provenance` |
> | DEL-03-02-REQ-06 | Unit-bearing dimensional, weight, mass-property, center-of-gravity, stiffness, and property fields are unit-aware and dimensionally checked against accepted unit dimensions. | SourcePath: `schemas/section.schema.yaml`; SectionRef: `$defs.QuantityValue`, `$defs.SectionDimension`; SourcePath: `schemas/component.schema.yaml`; SectionRef: `$defs.QuantityValue`, `$defs.ComponentQuantityDimension`; SourcePath: `tests/test_component_section_schema.py` |
> | DEL-03-02-REQ-07 | Suspected protected content is representable as protected-suspected/quarantined/rejected state or protected-content diagnostics, not accepted as public library content. | SourcePath: `schemas/section.schema.yaml`; SectionRef: `$defs.RedistributionStatus`, `$defs.ReviewStatus`, `$defs.SectionDiagnosticCode`; SourcePath: `schemas/component.schema.yaml`; SectionRef: `$defs.RedistributionStatus`, `$defs.ReviewStatus`, `$defs.ComponentDiagnosticCode` |
> | DEL-03-02-REQ-08 | The schema artifacts align with JSON Schema 2020-12 and remain schema-first; service-facing command/query/job result-envelope integration is governed by adjacent architecture deliverables. | SourcePath: `schemas/section.schema.yaml`; SectionRef: `$schema`; SourcePath: `schemas/component.schema.yaml`; SectionRef: `$schema`; SourcePath: `_CONTEXT.md`; SectionRef: Architecture Basis Injection |
> | DEL-03-02-REQ-09 | Persistence-facing records are versioned, unit-aware, provenance-preserving, schema-governed, and testable as strict schema instances. Broader migration and round-trip persistence behavior remains governed outside this reconciliation scope. | SourcePath: `schemas/section.schema.yaml`; SourcePath: `schemas/component.schema.yaml`; SourcePath: `tests/test_component_section_schema.py`; SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-04 |
> | DEL-03-02-REQ-10 | Library import/export or adapter-facing use shall preserve internal/public API boundaries and shall not bypass validation, provenance, redistribution, diagnostics, or public/private data boundaries. Concrete import formats remain `TBD`. | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-07; SourcePath: `schemas/section.schema.yaml`; SectionRef: `$defs.OpenDecision`; SourcePath: `schemas/component.schema.yaml`; SectionRef: `$defs.OpenDecision` |
> | DEL-03-02-REQ-11 | Diagnostics for invalid, incomplete, private, unknown-source, or suspected protected library records include code, class, severity, source, affected object, message, remediation, and provenance where applicable. | SourcePath: `schemas/section.schema.yaml`; SectionRef: `$defs.SectionDiagnostic`; SourcePath: `schemas/component.schema.yaml`; SectionRef: `$defs.ComponentDiagnostic`; SourcePath: `fixtures/component/invented_section_library_valid.json`; SourcePath: `fixtures/component/invented_component_library_valid.json` |
> | DEL-03-02-REQ-12 | Current tests include schema validation, unit-dimension checks, provenance/redistribution gates, protected-content text checks, split fixture validation, and regression coverage for PKG-02 enum/dimension compatibility. Persistence round-trip coverage remains governed outside this reconciliation scope. | SourcePath: `tests/test_component_section_schema.py` |
> | DEL-03-02-REQ-13 | Exact schema filenames are `schemas/component.schema.yaml` and `schemas/section.schema.yaml`; this reconciliation run shall not edit repository-level schema artifacts. | SourcePath: `_CONTEXT.md`; SectionRef: artifact list; SourcePath: `schemas/component.schema.yaml`; SourcePath: `schemas/section.schema.yaml` |
>

### CLM-012 — Standards

> ##### Standards
>
> No protected external engineering standards, code clauses, tables, examples, or proprietary data are incorporated into this deliverable.
>
> Applicable internal standards and baselines:
>
> - `docs/CONTRACT.md`: OPS-K-IP-1, OPS-K-IP-2, OPS-K-IP-3, OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-DATA-3, OPS-K-UNIT-1, OPS-K-PRIV-1, OPS-K-GOV-4, OPS-K-AGENT-1 through OPS-K-AGENT-4.
> - `docs/TYPES.md`: epistemic labels and data provenance labels.
> - `docs/SPEC.md`: domain objects, schema-governed development workflow, and acceptance semantics.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7: SOW-018, OBJ-004, and architecture basis AB-00-01, AB-00-02, AB-00-04, AB-00-06, AB-00-07, AB-00-08.
> - JSON Schema 2020-12 is the accepted schema-contract basis; this document does not reproduce external specification text.
>

### CLM-013 — Verification

> ##### Verification
>
> | Requirement IDs | Verification approach |
> |---|---|
> | REQ-01, REQ-02, REQ-13 | Review implemented schema files for section/component record coverage and traceability to DEL-03-02. |
> | REQ-03, REQ-07 | Protected-content review confirms no protected tables, code-derived values, proprietary catalog defaults, or private data are present in public artifacts. |
> | REQ-04 | Schema and validation tests confirm missing required values become validation findings, not defaults. |
> | REQ-05 | Provenance/redistribution tests confirm required source and status fields exist and are not silently omitted. |
> | REQ-06 | Unit-schema tests confirm dimensional and mass-property fields are unit-bearing and dimensionally checked. |
> | REQ-08, REQ-09, REQ-10 | Architecture review confirms schema-first, deterministic, persistence-aware, and boundary-preserving behavior. |
> | REQ-11 | Diagnostic tests confirm incomplete, unknown-source, private, and suspected-protected records produce governed diagnostics. |
> | REQ-12 | Test review confirms schema, unit, provenance, protected-content, split-fixture, and compatibility regression gates are represented; persistence round-trip coverage remains outside this reconciliation scope. |
>

### CLM-014 — Documentation

> ##### Documentation
>
> Implemented artifacts and open items:
>
> - `schemas/section.schema.yaml` implements schema versioning, unit-bearing section dimension/property fields, provenance/redistribution metadata, review status, completeness status, diagnostics, and open decisions.
> - `schemas/component.schema.yaml` implements schema versioning, component-library record identity, component-family contracts, user-entered dimensions/weights/COG/stiffness/modifier slots, provenance/redistribution metadata, review status, completeness status, diagnostics, and open decisions.
> - `fixtures/component/invented_section_library_valid.json` and `fixtures/component/invented_component_library_valid.json` are strict invented/public-safe fixtures with engineering values intentionally omitted.
> - `fixtures/component/invented_section_component_library_valid.json` remains legacy combined evidence and points to the strict split fixtures.
> - `tests/test_component_section_schema.py` is the focused verification surface for schema/fixture compatibility.
> - Public source catalogs, public fixture value policy, source/license disposition, redistribution acceptance, dependency satisfaction, human review disposition, lifecycle state, and persistence round-trip notes remain `TBD` or governed outside this reconciliation scope.

- **AC-001** — The contract preserves accepted source requirements for unit-aware, provenance-bearing pipe and component records, rights-safe public fixtures, private data boundaries, explicit unknowns, and no silent engineering defaults.

## Production and Verification Method — Praxeology

### CLM-015 — Source preamble

> ---
> doc_id: DEL-03-02-PROCEDURE
> doc_kind: deliverable.procedure
> status: draft
> created: 2026-04-30
> deliverable_id: DEL-03-02
> package_id: PKG-03
> ---
>

### CLM-016 — Procedure: Pipe Section and Component Library Schema

> #### Procedure: Pipe Section and Component Library Schema
>

### CLM-017 — Purpose

> ##### Purpose
>
> This procedure describes how to maintain and reconcile DEL-03-02 schema evidence without introducing protected tables, proprietary catalog data, invented engineering values, or unsupported compliance claims.
>

### CLM-018 — Prerequisites

> ##### Prerequisites
>
> - Sealed DEL-03-02 context with write scope limited to the current authorized files.
> - Register rows for DEL-03-02, SOW-018, and ContextBudgetQA row DEL-03-02.
> - Applicable contract invariants from `docs/CONTRACT.md`, especially IP, provenance, unit safety, privacy, governance review, and agent epistemic constraints.
> - Architecture basis AB-00-01, AB-00-02, AB-00-04, AB-00-06, AB-00-07, and AB-00-08.
> - Implemented evidence in `schemas/section.schema.yaml`, `schemas/component.schema.yaml`, `fixtures/component/invented_section_library_valid.json`, `fixtures/component/invented_component_library_valid.json`, `fixtures/component/invented_section_component_library_valid.json`, and `tests/test_component_section_schema.py`.
> - Human-owned dependency declarations are not tracked in `_DEPENDENCIES.md`; no explicit upstream/downstream list is available.
>

### CLM-019 — Steps

> ##### Steps
>
> 1. Confirm identity and boundary.
>    - Verify deliverable ID `DEL-03-02`, package `PKG-03`, type `DATA_MODEL_CHANGE`, scope item `SOW-018`, and objective `OBJ-004`.
>    - Confirm whether the active brief allows repository-level schema edits. If not, reconcile documentation evidence only.
>
> 2. Gather governing source slices.
>    - Read `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, register rows, decomposition row DEL-03-02, SOW-018, OBJ-004, and applicable contract invariants when they are in scope.
>    - Read the implemented evidence files named in the prerequisites.
>    - Use only accessible local governance, decomposition, schema, fixture, and test sources for evidence.
>
> 3. Reconcile schema intent against implemented evidence.
>    - Identify implemented record classes: section library record and component library record.
>    - Record implemented concepts: identity, schema version, user-entered dimensions/weights/COG/stiffness/modifier slots where applicable, unit-bearing values, provenance, redistribution status, review status, completeness status, diagnostics hooks, open decisions, and protected-value policies.
>    - Preserve human-owned policy/source/catalog/fixture-value/dependency/lifecycle/review-disposition items as `TBD`.
>
> 4. Apply data-boundary rules.
>    - Reject protected standards text, protected dimensional tables, proprietary catalog data, code-derived values, copied formulas, and private project data as public defaults.
>    - Mark suspected protected content for quarantine and human review.
>
> 5. Map requirements to verification.
>    - For each requirement, identify implemented schema validation, unit check, provenance gate, protected-content gate, split-fixture validation, compatibility regression, round-trip test, or architecture review evidence.
>    - Use invented/public-safe fixtures only.
>
> 6. Perform consistency review.
>    - Confirm Datasheet, Specification, Guidance, and Procedure use consistent terms: section library record, component library record, provenance, redistribution status, unit-aware value, protected-content gate, and private/public data boundary.
>    - Convert unsupported details to `TBD`, `ASSUMPTION`, or a Conflict Table entry.
>
> 7. Stop and escalate when needed.
>    - Stop if implementation would require protected tables, proprietary data, code-derived defaults, legal conclusions about redistribution rights, or professional/certification claims.
>

### CLM-020 — Verification

> ##### Verification
>
> - The four active documents exist and retain required sections.
> - No repo-level schema files are edited unless the active sealed brief authorizes them.
> - No protected dimensional tables, standards data, proprietary catalog values, or invented engineering values are introduced.
> - Unknown policy/source/catalog/fixture-value/dependency/lifecycle/review-disposition items remain `TBD`.
> - Requirements cite accessible sources or are labeled as assumptions.
> - Dependency extraction artifacts and lifecycle state remain unchanged unless explicitly in scope.
>

### CLM-021 — Records

> ##### Records
>
> - `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` record active deliverable evidence.
> - `_SEMANTIC.md` records the semantic lens generated for this deliverable.
> - `_SEMANTIC_LENSING.md` records warranted enrichment items.
> - `Dependencies.csv` and `_DEPENDENCIES.md` record extracted dependency evidence.
> - `_run_records/TASK_RUN_*.md` records the required sequence, QA checks, missing inputs, and human rulings needed.

- **VER-001** — Validate the contract and review source parity, pipe-section and component record coverage, units and provenance, public/private data controls, missing-value diagnostics, deterministic persistence compatibility, and unresolved policy decisions.

## Governing Values and Decisions — Axiology

### CLM-022 — Source preamble

> ---
> doc_id: DEL-03-02-GUIDANCE
> doc_kind: deliverable.guidance
> status: draft
> created: 2026-04-30
> deliverable_id: DEL-03-02
> package_id: PKG-03
> ---
>

### CLM-023 — Guidance: Pipe Section and Component Library Schema

> #### Guidance: Pipe Section and Component Library Schema
>

### CLM-024 — Purpose

> ##### Purpose
>
> This guidance explains how to use and extend the implemented DEL-03-02 schema evidence. The deliverable enables private pipe section and component libraries while preserving provenance, redistribution status, review status, unit safety, completeness findings, diagnostics, and the public/private data boundary.
>

### CLM-025 — Principles

> ##### Principles
>
> - Treat the schema as a carrier for user/private or lawfully redistributable library records, not as a source of public engineering tables.
> - Keep protected dimensional tables, protected standards content, proprietary catalog values, allowables, SIFs, flexibility factors, and code-derived values out of public defaults.
> - Make missing values visible as validation findings, completeness flags, or diagnostics; do not fill them silently.
> - Require provenance and redistribution status wherever a value could affect engineering reliance or public contribution review.
> - Keep dimensional and mass-property values unit-aware.
> - Separate detailed component-family mechanics from the generic library schema unless a sealed adjacent DEL-03 deliverable authorizes specialization.
>

### CLM-026 — Considerations

> ##### Considerations
>
> - This reconciliation records implemented evidence in `schemas/component.schema.yaml`, `schemas/section.schema.yaml`, the strict split fixtures, the combined legacy fixture pointer, and `tests/test_component_section_schema.py`; it does not edit those repository-level artifacts.
> - SOW-018 is narrower than several adjacent PKG-03 deliverables. The component schema includes generic bend, branch, rigid, and expansion-joint family contracts and slots, but detailed mechanics, accepted values, and editor behavior remain outside this reconciliation scope.
> - Provenance and redistribution schema mechanisms distinguish private, public-permissive, unknown-source, protected-suspected, rejected, pending, accepted, and quarantined states. Accepted public source catalogs, source/license disposition, redistribution acceptance, and public fixture value policy remain `TBD`.
> - Public example data must be invented, original, public-domain, or otherwise supported by documented redistribution rights.
> - If any implementation needs a protected source table to define defaults, stop and route the value to user-supplied/private data handling instead.
>

### CLM-027 — Trade-offs

> ##### Trade-offs
>
> | Topic | Conservative position |
> |---|---|
> | Exact field names | Implemented schema field names are evidence; changes require a sealed schema review. |
> | Component-family depth | Keep generic family contracts and schema slots here; defer detailed mechanics and accepted engineering values to their owning DEL-03 deliverables. |
> | Default dimensional values | Do not provide. Public defaults risk protected-table or proprietary-data leakage. |
> | Provenance strictness | Prefer explicit `unknown`, `protected_suspected`, `quarantined`, or `TBD` over weak inferred source claims. |
> | Public contribution acceptance | Requires documented rights and review; schema can support the metadata but does not itself approve contribution. |
>

### CLM-028 — Examples

> ##### Examples
>
> - Safe example approach: use invented pipe section/component records in tests with engineering values omitted or clearly non-engineering, plus explicit provenance/review status.
> - Unsafe example approach: copying nominal pipe dimensions, vendor catalog weights, protected code tables, or proprietary component geometry into public fixtures.
>

### CLM-029 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | DEL-03-02-CF-001 | `_CONTEXT.md` identifies SOFTWARE_DECOMP revision 0.7, while `_REFERENCES.md` still describes the decomposition reference as accepted v0.2. | `_CONTEXT.md#Decomposition Reference` | `_REFERENCES.md#Decomposition and Registers` | Datasheet References; Procedure Records | Treat `_CONTEXT.md` and sealed brief revision 0.7 as current basis for this run; route `_REFERENCES.md` cleanup to a metadata owner because it is outside the four-doc write target. | RESOLVED_BY_HUMAN: current `_CONTEXT.md` and `_REFERENCES.md` both cite SOFTWARE_DECOMP revision 0.7; prior v0.2 reference is stale/superseded. |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-018 OBJ-004 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
