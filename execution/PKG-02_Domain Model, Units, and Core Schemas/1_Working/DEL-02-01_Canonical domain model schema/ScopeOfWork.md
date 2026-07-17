---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-02-01
package_id: PKG-02
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@69ac259a7113d5a838fb22aa2e84df0e0f109713
project_scope_refs: [SOW-041, SOW-065]
package_objective_refs: [OBJ-001, OBJ-012, OBJ-014]
---

# Scope of Work — DEL-02-01

## Purpose and Objective Traceability

This Scope of Work defines `DEL-02-01` in service of project scope [SOW-041, SOW-065] and package objectives [OBJ-001, OBJ-012, OBJ-014].

- **OUT-001** — A canonical domain-model schema contract covering project, physical and analytical models, materials, components, loads, results, reports, assumptions, traceability, and source-of-truth references is produced for the declared scope and objectives.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-02-01 Canonical domain model schema

> #### Datasheet: DEL-02-01 Canonical domain model schema
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-02-01-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-02-01 |
> | Package ID | PKG-02 |
> | Package | Domain Model, Units, and Core Schemas |
> | Type | DATA_MODEL_CHANGE |
> | Scope item | SOW-041 |
> | Deliverable objective | OBJ-001 |
> | Primary artifact target | `schemas/model.schema.yaml` |
> | Secondary artifact target | `docs/TYPES.md` update |
> | Current drafting basis | `_CONTEXT.md` accepted revision 0.7; `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 |
> | Source posture | Public governance/register sources only; no protected standards/code data introduced |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Value | Source |
> |---|---|---|
> | Schema baseline | JSON Schema 2020-12 | `docs/_Registers/ScopeLedger.csv` row SOW-041; `docs/_Registers/ContextBudgetQA.csv` row DEL-02-01; `execution/_Decomposition/SOFTWARE_DECOMP.md` Section 8.2 |
> | Canonical object families in scope | Project, Model, Node, Element, Material, Component, Load/LoadCase, Result, Report | `_CONTEXT.md` Description; `docs/SPEC.md` Section 3; `docs/DIRECTIVE.md` Section 2.1 |
> | Adjacent referenced object families | Section, Support/Restraint, Combination, RulePack references | `docs/SPEC.md` Sections 3, 5, and 6; `docs/PRD.md` Sections 11.6, 12.2, and 13.2 |
> | Required cross-cutting fields | Stable identity, units where values are dimensional, provenance where engineering reliance may be affected | `docs/DIRECTIVE.md` Section 2.1; `docs/PRD.md` FR-002 and Section 13.5; `docs/CONTRACT.md` OPS-K-DATA-3 and OPS-K-UNIT-1 |
> | Result and diagnostic data | Result/report records must preserve warnings, assumptions, diagnostics, provenance, and no certification/compliance claims | `docs/SPEC.md` Sections 7 and 8; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06; `docs/CONTRACT.md` OPS-K-AUTH-1 |
> | Persistence compatibility | Schema-governed, versioned, deterministic, unit-aware, provenance-preserving, round-trip testable JSON payloads; JCS-compatible hash basis where JSON payloads are hashed | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04 and Section 8.2 |
> | Public/private data boundary | Public schemas may describe user/private data fields, but must not bundle protected standards text, tables, formulas, allowables, proprietary catalogs, or private user data | `docs/CONTRACT.md` OPS-K-IP-1; `docs/IP_AND_DATA_BOUNDARY.md` Sections 2 and 3; `docs/PRD.md` Sections 17.1 and 17.3 |
>

### CLM-005 — Conditions

> ##### Conditions
>
> - The deliverable covers the canonical model schema contract only. It does not implement numerical solving, GUI views, rule evaluation, import/export adapters, report rendering, or library population. Source: `_CONTEXT.md` Package Exclusions; `execution/_Decomposition/SOFTWARE_DECOMP.md` PKG-02 row.
> - The schema must support a code-neutral model: mechanics data and user-owned rule/code data are separable, and software states must not imply automatic code compliance. Source: `docs/TYPES.md` Sections 4 and 6; `docs/DIRECTIVE.md` Sections 2.2 and 3.
> - Missing solve-required or rule-check-required values are explicit findings, not silent defaults. Source: `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` Section 7.
> - ASSUMPTION: DEL-02-01 may define lightweight references or common definitions for sections, supports, combinations, and rule-pack references when required by the canonical model, but detailed material/component library semantics remain in PKG-03 and rule-pack internals remain in PKG-06.
> - TBD: Exact schema file layout, `$id` URI, code-generation tooling, and fixture organization.
> - TBD: Physical project package/container and migration framework remain outside this deliverable unless separately approved. Source: `_CONTEXT.md` Architecture Basis Injection; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04 and Section 8.2.
>

### CLM-006 — Construction

> ##### Construction
>
> The expected schema construction is a public JSON Schema 2020-12 contract with reusable definitions for:
>
> - document/root metadata: schema version, application/schema identity, provenance summary, and migration/compatibility status where persisted;
> - project/model containers: units, coordinate system, storage policy hooks, rule-pack references, report settings, and model object collections;
> - model objects: node coordinates and degrees of freedom, element endpoints/material/section references, components, supports, loads, load cases, and combinations;
> - engineering data records: unit-bearing quantities, provenance records, redistribution status, source quality, and review status;
> - result/report records: result envelopes, warnings/diagnostics, input manifests, hashes/checksums, and professional-boundary notices;
> - validation fixtures: invented or public/permissive examples only, with protected-content and provenance checks.
>
> The construction above is source-grounded as an initial schema plan, not an implemented schema file. The primary artifact path `schemas/model.schema.yaml` is outside the write scope of this Pass 1+2 document run.
>

### CLM-007 — References

> ##### References
>
> - `_CONTEXT.md` revision 0.7 for deliverable identity, accepted decomposition reference, SCA-001 basis IDs, and write-scope constraints.
> - `_REFERENCES.md` for the locally declared reference set.
> - `_DEPENDENCIES.md` for human-owned dependency posture; no concrete upstream/downstream dependency list was declared.
> - `docs/_Registers/Deliverables.csv` row DEL-02-01 for deliverable identity and artifact targets.
> - `docs/_Registers/ScopeLedger.csv` row SOW-041 for machine-readable schema scope and JSON Schema 2020-12 baseline.
> - `docs/_Registers/ContextBudgetQA.csv` row DEL-02-01 for context-envelope and SCA-001 notes.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7, especially PKG-02, DEL-02-01, SOW-041, and AB-00-01/02/03/04/06/07/08.
> - `docs/CONTRACT.md` for IP, data, authority, unit, report, and agent invariants.
> - `docs/TYPES.md` for stable IDs, deliverable types, analysis-status vocabulary, epistemic labels, and provenance labels.
> - `docs/DIRECTIVE.md` for object ontology, no-silent-defaults, code-neutrality, unit safety, provenance, and professional-boundary principles.
> - `docs/SPEC.md` Sections 1 through 9 for layers, domain objects, loads/stress, rule-pack boundary, diagnostics, reports, and validation.
> - `docs/PRD.md` Sections 10, 11.6 through 11.8, 12.1 through 12.2, 13.1 through 13.5, 15, 17, 18, 19.1, 22.1, and Appendix A for product requirements and schema-relevant examples.
> - `docs/IP_AND_DATA_BOUNDARY.md` for public/private data and provenance policy.

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-02-01 Canonical domain model schema

> #### Specification: DEL-02-01 Canonical domain model schema
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-009 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-02-01-DECL-001`.
>

### CLM-010 — Scope

> ##### Scope
>
> This specification governs the DEL-02-01 document-level contract for a canonical domain model schema in PKG-02. The deliverable defines the intended public schema boundary for project, model, node, element, material, component, load, result, and report entities. It also records constraints that a future implementation of `schemas/model.schema.yaml` must satisfy.
>
> In scope:
>
> - canonical object families and common schema definitions required by SOW-041;
> - JSON Schema 2020-12 as the public schema/interchange baseline;
> - unit, provenance, diagnostics, status, reproducibility, and data-boundary requirements that apply to model-schema records;
> - verification expectations for schema validation, examples, and round-trip behavior.
>
> Out of scope:
>
> - numerical solver implementation;
> - GUI model tree/property editor behavior;
> - detailed material/component library population;
> - rule-pack evaluator implementation or code-specific rule content;
> - report generator implementation;
> - physical project-file package/container selection;
> - claims boundary — Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
>

### CLM-011 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source | Verification |
> |---|---|---|---|
> | REQ-02-01-01 | The public model schema shall use JSON Schema 2020-12 as its baseline unless a later human-approved architecture change supersedes SCA-001. | `docs/_Registers/ScopeLedger.csv` SOW-041; `execution/_Decomposition/SOFTWARE_DECOMP.md` Section 8.2 | Schema declares JSON Schema 2020-12 dialect; review verifies no conflicting schema baseline. |
> | REQ-02-01-02 | The schema shall define or reference canonical records for Project, Model, Node, Element, Material, Component, Load/LoadCase, Result, and Report. | `_CONTEXT.md` Description; `docs/SPEC.md` Section 3; `docs/DIRECTIVE.md` Section 2.1 | Schema coverage review maps each required family to a definition or justified reference. |
> | REQ-02-01-03 | Addressable schema records shall carry stable identifiers and use explicit references rather than implicit positional coupling where downstream editing, solving, reporting, or audit depends on identity. | `docs/TYPES.md` Section 2; `docs/DIRECTIVE.md` Section 2.1 | Schema review verifies ID/reference fields for addressable object families. |
> | REQ-02-01-04 | Dimensional values shall be unit-aware; missing or incompatible units shall be representable as validation findings rather than silently accepted defaults. | `docs/CONTRACT.md` OPS-K-UNIT-1 and OPS-K-DATA-2; `docs/PRD.md` FR-002; `docs/SPEC.md` Sections 7 and 9 | Schema tests reject incompatible/missing units where required; findings are explicit. |
> | REQ-02-01-05 | Engineering-reliance data such as materials, components, allowables, SIF/flexibility inputs, loads, rule references, reports, and imported records shall carry source/provenance and redistribution/private-status metadata where applicable. | `docs/CONTRACT.md` OPS-K-DATA-3 and OPS-K-IP-2; `docs/IP_AND_DATA_BOUNDARY.md` Section 4; `docs/PRD.md` Sections 13.4 and 13.5 | Schema includes provenance/status fields or reusable definitions; protected-content review verifies no protected data is bundled. |
> | REQ-02-01-06 | The schema shall not include protected standards text, tables, figures, copied code formulas, material allowable tables, SIF/flexibility tables, protected dimensional tables, proprietary vendor catalogs, or private user data as public defaults. | `docs/CONTRACT.md` OPS-K-IP-1 and OPS-K-IP-3; `docs/IP_AND_DATA_BOUNDARY.md` Sections 2 and 3; `docs/PRD.md` Section 17.1 | Protected-content/provenance gate scans schemas, fixtures, and examples. |
> | REQ-02-01-07 | The schema shall preserve the distinction among mechanics-solved data, user-rule-check data, and human professional approval records; it shall not define automatic `CODE_COMPLIANT` status. | `docs/TYPES.md` Section 4; `docs/DIRECTIVE.md` Section 2.2; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-03 | Status enum review confirms allowed statuses and excludes automatic compliance statuses. |
> | REQ-02-01-08 | Result and report-facing records shall be compatible with diagnostic/result-envelope data carrying code, class, severity, source, affected object, message, remediation, and provenance. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06; `docs/SPEC.md` Sections 7 and 8 | Schema review verifies diagnostic envelope fields or references; tests cover each warning class where applicable. |
> | REQ-02-01-09 | Persisted JSON payloads governed by the schema shall be compatible with deterministic, versioned, migration-aware, round-trip-testable persistence and JCS-compatible hashing where JSON payload hashes are used. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04 and Section 8.2; `docs/PRD.md` FR-001 and Section 15.3 | Round-trip fixture test preserves model, units, loads, rule references, and provenance metadata; hash basis documented where used. |
> | REQ-02-01-10 | Adapters, plugins, and import/export consumers shall not be able to bypass unit, provenance, diagnostics, validation, or public/private data-boundary constraints expressed by the schema. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-02 and AB-00-07; `docs/SPEC.md` Section 1 | API/import/export contract review confirms schema validation remains mandatory at boundaries. |
> | REQ-02-01-11 | Public examples or fixtures associated with the schema shall use invented, public-domain, original, or permissively licensed data with provenance; code-specific examples remain private/user-supplied. | `docs/CONTRACT.md` OPS-K-RULE-1; `docs/IP_AND_DATA_BOUNDARY.md` Sections 2 and 6; `docs/PRD.md` Sections 7.5 and 17.1 | Example-data review verifies provenance labels and absence of protected/code-specific content. |
> | REQ-02-01-12 | Schema acceptance shall include layered checks appropriate to schemas, units, provenance, protected-content boundaries, deterministic round trips, and downstream result/report compatibility. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-08; `docs/SPEC.md` Sections 9 and 11 | Test evidence or review record links each check to a schema artifact or documented deferral. |
>

### CLM-012 — Object-Family Coverage Crosswalk

> ##### Object-Family Coverage Crosswalk
>
> This crosswalk is acceptance-supporting planning content for the future `schemas/model.schema.yaml`; it is not evidence that the schema file has already been implemented. Any new canonical vocabulary still requires the anticipated `docs/TYPES.md` update or a human ruling before it is treated as frozen.
>
> | Family or record group | Expected schema treatment | Source basis | Pass 3 disposition |
> |---|---|---|---|
> | Project | Define a root/project record with stable identity, units, storage/privacy posture, rule-pack references, report settings, and persistence hooks where applicable. | `_CONTEXT.md` Description; `docs/SPEC.md` Section 3; `docs/PRD.md` FR-001 and Appendix A | Required coverage. |
> | Model | Define the model container and collections for nodes, elements, components, supports, loads, libraries, results, and report-facing references. | `docs/SPEC.md` Section 3; `docs/DIRECTIVE.md` Section 2.1 | Required coverage. |
> | Node and Element | Define addressable records for coordinates/DOF context and analytical member connectivity, with explicit references rather than positional coupling. | `docs/SPEC.md` Sections 3 and 4.1; `docs/TYPES.md` Section 2 | Required coverage. |
> | Material and Component | Define reusable records or references with source/provenance, redistribution/private status, and unit-bearing properties where engineering reliance may be affected. | `docs/SPEC.md` Section 3; `docs/PRD.md` Sections 13.1, 13.3, and 13.4; `docs/CONTRACT.md` OPS-K-DATA-3 | Required core records; detailed library population remains outside DEL-02-01. |
> | Section and Support/Restraint | Define shared references or common records needed by the canonical model; keep detailed calculation behavior, library content, and nonlinear solver behavior in their owning deliverables. | `docs/SPEC.md` Sections 3 and 4.4; `docs/PRD.md` Sections 11.4 and 13.2 | Adjacent coverage required at the reference/common-record level; detailed ownership remains `TBD` where not explicit. |
> | LoadCase, Combination, and Load umbrella | Treat `LoadCase` as primitive loading and `Combination` as algebraic combination records; treat `Load` in register prose as the umbrella for load-related schema coverage until vocabulary is frozen. | `_CONTEXT.md` Description; `docs/SPEC.md` Sections 3 and 5; `docs/PRD.md` Sections 10 and 11.6 | Required coverage with no public code-specific combination defaults. |
> | RulePack reference | Define references, redistribution/private status, checksum/source notes, and required-input links; do not define protected rule formulas or evaluator internals here. | `docs/SPEC.md` Section 6; `docs/CONTRACT.md` OPS-K-RULE-3; `docs/PRD.md` Sections 12.1 through 12.4 | Reference-only coverage for DEL-02-01. |
> | Result | Define result-facing records compatible with mechanical outputs, warning classes, diagnostics, affected-object references, provenance, and authority/status boundaries. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-03 and AB-00-06; `docs/SPEC.md` Sections 7 and 8 | Required coverage. |
> | Report | Define report-facing records for input manifest, model hash/checksum references, warnings, assumptions, provenance summary, limitations, and professional-boundary notices. | `docs/SPEC.md` Section 8; `docs/PRD.md` Section 15 | Required coverage; report rendering remains outside DEL-02-01. |
>

### CLM-013 — Acceptance Crosswalks

> ##### Acceptance Crosswalks
>

### CLM-014 — Unit Applicability

> ###### Unit Applicability
>
> | Field class | Minimum acceptance evidence | Source basis |
> |---|---|---|
> | Coordinates, geometry, dimensions, section properties, stiffnesses, weights, pressures, temperatures, and imposed displacements/rotations | Schema fields carry units or explicit unit references; missing/incompatible units can be reported as validation findings. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/PRD.md` FR-002; `docs/SPEC.md` Sections 3, 5, and 9 |
> | Loads, load cases, combinations, results, reactions, forces, moments, and stresses | Unit context is preserved through input, result, and report-facing records. | `docs/PRD.md` FR-002 and FR-016; `docs/SPEC.md` Sections 5 and 8 |
> | Dimensionless statuses, labels, identifiers, provenance records, and notices | Schema distinguishes non-dimensional metadata from dimensional quantities rather than forcing meaningless units. | `docs/TYPES.md` Sections 2, 4, 5, and 7 |
>

### CLM-015 — Provenance And Status Applicability

> ###### Provenance And Status Applicability
>
> | Record family | Minimum acceptance evidence | Source basis |
> |---|---|---|
> | Public data records, examples, and fixtures | Reusable provenance/status fields cover source name, source location, source license, contributor, contributor certification, redistribution status, and review status, or the review record documents an explicit deferral. | `docs/IP_AND_DATA_BOUNDARY.md` Section 4; `docs/CONTRACT.md` OPS-K-IP-2 |
> | Materials, components, sections, SIF/flexibility inputs, allowables, loads, rule references, reports, and imported records | Source/provenance and redistribution/private-status metadata are available where engineering reliance or public redistribution may be affected. | `docs/CONTRACT.md` OPS-K-DATA-3; `docs/PRD.md` Sections 13.1 through 13.5 |
> | Analysis and authority states | Status values preserve mechanics-solved, user-rule-checked, and human-approved-for-project distinctions, and exclude automatic code-compliance statuses. | `docs/TYPES.md` Section 4; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-03; `docs/CONTRACT.md` OPS-K-AUTH-1 |
>

### CLM-016 — Diagnostics, Results, And Reports

> ###### Diagnostics, Results, And Reports
>
> | Envelope field or concept | Result schema expectation | Report schema expectation | Source basis |
> |---|---|---|---|
> | code, class, severity | Diagnostic/result record can carry machine-readable code/class/severity and supported warning classes. | Report record can disclose warning classes and blocking/nonblocking status without converting them into compliance claims. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06; `docs/SPEC.md` Section 7 |
> | source and affected object | Diagnostic/result record references the emitting subsystem/source and the affected model object where applicable. | Report record can trace warnings, assumptions, and missing data back to model objects or source notes. | `docs/SPEC.md` Sections 7 and 8; `docs/PRD.md` Section 15.1 |
> | message and remediation | Diagnostic/result record can carry human-readable message and remediation/next-action text. | Report record can include limitations, assumptions, and warnings without embedding protected standards content. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06; `docs/PRD.md` Sections 15.1 and 15.2 |
> | provenance, input manifest, and hash/checksum references | Result record preserves provenance and reproducibility links where available. | Report record can include input manifest, model hash/checksum references, rule-pack checksum, provenance summary, and professional-boundary notice. | `docs/SPEC.md` Section 8; `docs/PRD.md` Sections 15.1 and 15.3 |
>

### CLM-017 — Hash And Persistence Boundary

> ###### Hash And Persistence Boundary
>
> - When JSON payload hashes are used, the review evidence must identify the payload boundary being hashed and confirm the canonical JSON/JCS-compatible basis from AB-00-04. If no hash is generated for a schema path or fixture, record the item as a documented deferral rather than implying coverage.
> - DEL-02-01 may define schema compatibility hooks for version, migration status, canonicalization/hash metadata, units, loads, rule-pack references, diagnostics, and provenance. Physical project package/container choice and migration implementation remain tied to DEL-02-05 or a later human-approved architecture decision.
>

### CLM-018 — Standards

> ##### Standards
>
> | Standard or governing basis | Applicability | Status |
> |---|---|---|
> | JSON Schema 2020-12 | Public schema/interchange baseline for `schemas/model.schema.yaml` | Required by SOW-041 and SCA-001 |
> | Canonical JSON / JCS-compatible hashing | Applies where JSON payload hashes are generated for reproducibility/audit | Required by AB-00-04 for hashed JSON payloads; exact implementation details TBD |
> | OpenPipeStress invariants in `docs/CONTRACT.md` | Binding constraints for IP boundary, data provenance, units, authority, reports, privacy, and agents | Required |
> | OpenPipeStress vocabulary in `docs/TYPES.md` | Stable IDs, analysis statuses, epistemic labels, and provenance labels | Required unless superseded by human-approved change |
> | Protected engineering standards/codes | May be referenced as user/private source context only; must not be embedded as public schema content or examples | Public redistribution prohibited without explicit rights |
>

### CLM-019 — Verification

> ##### Verification
>
> Minimum verification expectations for the eventual schema artifact:
>
> - JSON Schema dialect check confirms JSON Schema 2020-12.
> - Coverage check maps Project, Model, Node, Element, Material, Component, Load/LoadCase, Result, and Report to schema definitions or justified references.
> - Object-family coverage check uses the crosswalk above to classify required coverage, adjacent reference/common-record coverage, and documented deferrals.
> - Unit-field checks confirm dimensional values carry units or explicit unit references.
> - Provenance checks confirm engineering-reliance and imported/private data can carry source, license/redistribution, contributor/review, and privacy status.
> - Protected-content review confirms no standards text, protected tables, code formulas, proprietary values, or private user data are bundled.
> - Status/authority review confirms no automatic `CODE_COMPLIANT` or certification-like status exists.
> - Round-trip fixture check confirms model data, units, loads, rule-pack references, diagnostics, hashes/checksums, and provenance metadata can survive serialization without loss.
> - Hash-scope review confirms each JSON payload hash has a documented payload boundary and canonicalization basis, or a visible deferral.
> - Diagnostics check confirms warning/result envelope compatibility.
> - Fixture review confirms each public example or validation fixture has a provenance manifest and protected-content disposition.
> - Documentation review confirms TBDs and assumptions are visible.
>

### CLM-020 — Documentation

> ##### Documentation
>
> Required or expected records:
>
> - `schemas/model.schema.yaml` as the future primary schema artifact. This path is outside the current four-document write scope.
> - `docs/TYPES.md` update if new canonical type names, statuses, provenance labels, or schema vocabulary are introduced. This path is outside the current four-document write scope.
> - Schema validation/test evidence, including object-family coverage, unit applicability, provenance/status applicability, diagnostic/result/report mapping, hash-scope decisions or deferrals, fixture provenance manifests, and protected-content gates when implementation proceeds.
> - Human review notes for schema file layout, code-generation tooling, `$id` URI, migration metadata, and any objective-mapping discrepancy recorded in `Guidance.md`.
> - `_run_records/TASK_RUN_*.md` for document drafting/enrichment runs.

- **AC-001** — The contract preserves the accepted source requirements and boundaries for the canonical model schema, including unit, provenance, diagnostic, professional-reliance, and protected-content constraints, with unresolved decisions retained as stated.

## Production and Verification Method — Praxeology

### CLM-021 — Procedure: DEL-02-01 Canonical domain model schema

> #### Procedure: DEL-02-01 Canonical domain model schema
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-022 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-02-01-DECL-004`.
>

### CLM-023 — Purpose

> ##### Purpose
>
> This procedure describes how to produce and review the canonical domain model schema for DEL-02-01 without crossing the sealed write scope or introducing protected/code-specific data. It is operational guidance for the future `schemas/model.schema.yaml` artifact and its review evidence.
>

### CLM-024 — Prerequisites

> ##### Prerequisites
>
> - Confirm the active deliverable is DEL-02-01 in PKG-02 and that the accepted context is `_CONTEXT.md` current-basis context.
> - Confirm the schema baseline is JSON Schema 2020-12 from SOW-041 and SCA-001.
> - Read the applicable invariants in `docs/CONTRACT.md`, especially IP/data boundary, units, provenance, authority, reports, privacy, and agent constraints.
> - Read `docs/TYPES.md` for stable identifiers, analysis-status vocabulary, epistemic labels, and provenance labels.
> - Read `docs/DIRECTIVE.md`, `docs/SPEC.md`, and the relevant PRD sections for object ontology, domain layers, diagnostics, reports, and data-boundary requirements.
> - Confirm no human-declared dependency list exists in `_DEPENDENCIES.md`; treat dependency coordination as externally managed until humans provide a concrete list.
> - Confirm implementation-level decisions still marked TBD by SCA-001 are not silently resolved in this deliverable.
>

### CLM-025 — Steps

> ##### Steps
>
> 1. Confirm scope and status.
>    - Verify `_STATUS.md` permits work.
>    - Verify write scope before editing; do not modify `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, or `_SEMANTIC.md`.
>
> 2. Inventory required schema object families.
>    - Start with the DEL-02-01 description: project, model, node, element, material, component, load, result, and report.
>    - Cross-check `docs/SPEC.md` Section 3 for adjacent object families such as Section, Support, Combination, and RulePack references.
>    - Record each required, adjacent, or deferred family in the object-family coverage crosswalk before claiming schema coverage.
>    - Mark any ownership ambiguity as `TBD` rather than expanding into another package.
>
> 3. Define root schema metadata.
>    - Set JSON Schema 2020-12 dialect.
>    - Add schema version and identity fields.
>    - Add hooks for persistence compatibility, migration status, and canonical hash metadata where applicable.
>    - Leave concrete `$id` URI and code-generation tooling as `TBD` until approved.
>
> 4. Define common reusable records.
>    - Stable local identifiers and references.
>    - Unit-bearing quantity structure.
>    - Provenance/source and redistribution/private-status structure.
>    - Diagnostics/result envelope structure.
>    - Analysis-status and epistemic-label references.
>    - Record which field classes require units or unit references, and which record families require provenance/status fields.
>
> 5. Define model object records.
>    - Project and Model containers.
>    - Node coordinates and six-degree-of-freedom context.
>    - Element endpoints, type, material/section references, local coordinate basis, and result-station hooks.
>    - Material, Component, Section, Support/Restraint, LoadCase, Combination, Result, and Report records at the level authorized for DEL-02-01.
>
> 6. Apply data-boundary guards.
>    - Do not populate protected standards values, copied formulas, dimensional tables, allowable tables, commercial software examples, proprietary vendor catalog data, or private user data.
>    - Require provenance or explicit `TBD` for public data examples.
>    - Represent code-specific values as user-supplied/private inputs or rule-pack references, not public defaults.
>
> 7. Encode status and authority boundaries.
>    - Use analysis statuses consistent with `docs/TYPES.md`.
>    - Exclude any automatic code-compliant/certified/approved status.
>    - Preserve separate mechanics result, user-rule-check, and human-review/acceptance records.
>
> 8. Prepare examples and fixtures only when authorized.
>    - Use invented, original, public-domain, or permissively licensed data with provenance.
>    - Mark examples as non-code and non-compliance demonstrations.
>    - Create a fixture provenance manifest or record a `TBD` manifest path before example-based tests are accepted.
>    - Quarantine and escalate any suspected protected or proprietary source material.
>
> 9. Verify schema behavior.
>    - Run JSON Schema validation against valid and invalid invented fixtures.
>    - Check object-family coverage against the crosswalk in `Specification.md`.
>    - Check required units, provenance, status, diagnostics, and public/private boundary behavior.
>    - Check diagnostic/result/report envelope compatibility, including code/class/severity, source, affected object, message, remediation, and provenance.
>    - Check hash payload scope and canonicalization basis where JSON payload hashes are generated; record explicit deferrals where hashes are not yet used.
>    - Perform deterministic round-trip checks when persistence work is available.
>    - Record deferred checks as `TBD` rather than implying they passed.
>
> 10. Prepare review handoff.
>    - Summarize schema coverage against SOW-041 and DEL-02-01.
>    - List TBDs and conflict-table items needing human ruling.
>    - Reference the durable human-ruling record for objective/revision conflicts, schema layout, `$id` URI, code-generation tooling, fixture organization, and persistence/hashing decisions; if no record exists, mark the record path `TBD`.
>    - Identify any required `docs/TYPES.md` update without modifying it unless the write scope authorizes that file.
>

### CLM-026 — Verification

> ##### Verification
>
> | Check | Expected result |
> |---|---|
> | Scope check | Only DEL-02-01 authorized files are modified during this run. |
> | Default document check | `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` exist and retain their default sections. |
> | Source-grounding check | Non-trivial requirements cite local governance/register/source slices or are labeled `ASSUMPTION`/`TBD`. |
> | Schema baseline check | JSON Schema 2020-12 remains the declared baseline. |
> | Object-family check | Project, Model, Node, Element, Material, Component, Load/LoadCase, Result, and Report are covered. |
> | Object-family crosswalk check | Required, adjacent, reference-only, and deferred object families are classified before acceptance. |
> | Unit/provenance check | Unit-aware and provenance/status requirements are visible and mapped to field or record families. |
> | Diagnostic/report crosswalk check | Result/report records can carry diagnostic envelope fields, warning classes, provenance, input manifest, and report notices. |
> | Hash-scope check | JSON payload hashes have a documented payload boundary and canonicalization basis, or an explicit deferral. |
> | Fixture provenance check | Public examples and fixtures have a provenance manifest or a visible `TBD` manifest path. |
> | Data-boundary check | No protected standards/code text, copied formulas, protected tables, proprietary data, or private user data are introduced. |
> | Authority check | No compliance, certification, approval, sealing, or professional-reliance claim is made by the software/schema (PRD §21.2). |
> | Conflict check | Objective/revision discrepancies are surfaced in `Guidance.md` for human ruling. |
> | Status check | `_STATUS.md` moves from `OPEN` to `INITIALIZED` only after Pass 1+2 document creation succeeds. |
>

### CLM-027 — Records

> ##### Records
>
> Records produced by document drafting/enrichment runs:
>
> - `Datasheet.md`
> - `Specification.md`
> - `Guidance.md`
> - `Procedure.md`
> - `_run_records/TASK_RUN_2026-04-30_0120.md`
> - `_run_records/TASK_RUN_2026-04-30_0141.md`
> - `_run_records/TASK_RUN_2026-04-30_0941.md`
> - `_run_records/TASK_RUN_2026-04-30_0941_001.md`
> - `_run_records/TASK_RUN_2026-04-30_0941_002.md`
> - `_run_records/TASK_RUN_2026-04-30_0941_003.md`
> - `_STATUS.md` safe lifecycle update, if a Pass 1+2 run completes from `OPEN`; Pass 3 does not update `_STATUS.md`
>
> Records produced by dependency extraction:
>
> - `Dependencies.csv`
> - `_DEPENDENCIES.md`
> - `_run_records/TASK_RUN_2026-04-30_0941_004.md`
>
> Records expected from future implementation/review work:
>
> - `schemas/model.schema.yaml`
> - schema validation fixtures and results
> - object-family coverage crosswalk review evidence
> - unit-applicability and provenance/status acceptance evidence
> - diagnostic/result/report envelope crosswalk evidence
> - hash payload-scope decision or documented deferral
> - fixture provenance manifest for public examples or validation fixtures
> - protected-content/provenance gate evidence
> - deterministic round-trip evidence
> - any approved `docs/TYPES.md` update
> - human rulings for conflict-table entries and unresolved `TBD` decisions, with ruling record path or ID

- **VER-001** — Validate the contract and review source parity, object-family coverage, unit and provenance boundaries, deterministic persistence compatibility, and absence of protected content or compliance claims.

## Governing Values and Decisions — Axiology

### CLM-028 — Guidance: DEL-02-01 Canonical domain model schema

> #### Guidance: DEL-02-01 Canonical domain model schema
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-029 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-02-01-DECL-003`.
>

### CLM-030 — Purpose

> ##### Purpose
>
> DEL-02-01 exists to make the OpenPipeStress domain model explicit and machine-readable before solver, GUI, reporting, persistence, plugin, and validation work depend on it. The schema should be a public contract for open mechanics and workflow data while preserving the boundary that code-specific values, protected standards data, proprietary libraries, and professional approval remain user/private or human-controlled. Sources: `docs/DIRECTIVE.md` Sections 1 through 3; `docs/SPEC.md` Sections 1 through 3; `docs/_Registers/ScopeLedger.csv` row SOW-041.
>

### CLM-031 — Principles

> ##### Principles
>
> - Use the schema as a boundary contract, not as a place to encode protected engineering standards or project-specific compliance rules.
> - Keep every addressable model object stable enough for editing, solving, reporting, diagnostics, and audit trails.
> - Treat units as schema-visible data. Dimensional values should not be plain numbers without unit context unless the value is dimensionless and the schema makes that clear.
> - Treat provenance and redistribution/private status as first-class data where engineering reliance, public contribution, or report disclosure may be affected.
> - Preserve the analysis-status vocabulary: mechanics solved, user rule checked, and human-approved-for-project are different states. The software must not turn any of them into automatic code compliance.
> - Design result/report records so warnings, assumptions, missing data, diagnostics, source notes, hashes/checksums, and limitations can be reproduced in reports.
> - Keep implementation choices that SCA-001 leaves TBD, such as schema code-generation tooling and physical project packaging, visible as TBD until a human-approved decision exists.
>

### CLM-032 — Considerations

> ##### Considerations
>
> - PKG-02 owns the canonical schema surface, but detailed material/component library behavior belongs to PKG-03, rule-pack internals belong to PKG-06, reports belong to PKG-08, adapters/APIs belong to PKG-10, and privacy controls belong to PKG-12. DEL-02-01 should define shared records and references without silently taking over those deliverables.
> - `docs/SPEC.md` Section 3 lists a broader minimum domain object set than the short DEL-02-01 description. Include required references or reusable definitions where the canonical model cannot function without them, and mark boundary decisions as `TBD` when the correct owner is unclear.
> - The PRD Appendix A example is useful only as a schema-shape illustration. Do not copy it wholesale into public fixtures, and do not treat its demonstration values as engineering defaults.
> - Public examples and validation fixtures should be original/invented or documented public/permissive data. If a value looks code-derived, proprietary, or copied from a protected source, quarantine/escalate rather than adapting it.
> - The anticipated `docs/TYPES.md` update is outside this run's write scope. New type names or status/provenance labels should be proposed for human review before the file is changed.
> - PROPOSAL: Treat object-family names as draft schema vocabulary until `docs/TYPES.md` is updated or a human ruling freezes the names. In particular, use `LoadCase` for primitive solved loads, `Combination` for algebraic combinations, and `Load` only as an umbrella term when following register/decomposition prose.
> - Human rulings for conflict-table rows, schema file layout, `$id` URI, code-generation tooling, fixture organization, and persistence/hashing handoff need a durable reference before downstream acceptance depends on them. The exact ruling record type or path is `TBD` until the project authority selects one.
>
> Vocabulary and boundary notes:
>
> | Term or boundary | DEL-02-01 usage guidance | Deferred or owner-sensitive content |
> |---|---|---|
> | `Load`, `LoadCase`, `Combination` | Model the primitive and algebraic records explicitly; keep `Load` as a broad planning label unless the future type vocabulary says otherwise. | Code-specific load-combination requirements remain user/rule-pack supplied and must not become public defaults. |
> | `Support` and `Restraint` | Use a consistent schema vocabulary or alias relationship before freezing names; preserve enough structure for model references and diagnostics. | Nonlinear behavior details and solver convergence semantics remain with solver/support deliverables unless explicitly pulled into shared schema records. |
> | `Section` | Include common reference/record structure where elements, materials, components, and reports need it. | Detailed section property calculators and dimensional table population remain outside DEL-02-01. |
> | `RulePack` reference | Store reference/checksum/source/private-status links needed by projects, results, and reports. | Rule expression grammar, protected formulas, and evaluator behavior remain outside this deliverable. |
> | Result and Report records | Preserve diagnostics, affected-object references, provenance, input manifest, hash/checksum links, warnings, assumptions, and professional-boundary notices. | Report rendering, professional signoff workflow, and human approval records require their owning deliverables or human rulings. |
>

### CLM-033 — Trade-offs

> ##### Trade-offs
>
> | Topic | Option | Trade-off | Guidance |
> |---|---|---|---|
> | Schema shape | Single `schemas/model.schema.yaml` with `$defs` | Matches the anticipated artifact but can become large | Use `$defs` and clear references; split later only by human-approved schema architecture change. |
> | Domain breadth | Include every object in one schema | Improves consistency but risks crossing into PKG-03/PKG-06/PKG-08 details | Define common identities, references, and envelopes; defer detailed library/rule/report internals to their packages. |
> | Required fields | Require all potentially needed engineering data | Strong validation but may block mechanics-only workflows | Distinguish solve-required, rule-check-required, report-required, and optional fields; missing values become explicit findings. |
> | Public examples | Use realistic industrial data | Better realism but high IP/provenance risk | Use invented or permissively licensed examples with provenance and non-compliance notices. |
> | Hashing and canonicalization | Define implementation now | Improves reproducibility but overlaps DEL-02-05 persistence details | Keep schema compatible with AB-00-04; leave physical package and migration framework TBD. |
> | Persistence handoff | Encode compatibility hooks now, but defer storage implementation | Keeps model schema aligned with deterministic round trips while avoiding premature ownership of package/container and migration mechanics | Record version, migration-status, unit, provenance, rule-pack reference, diagnostic, and hash metadata hooks where needed; route physical package/container and migration framework decisions to DEL-02-05 or human authority. |
>

### CLM-034 — Examples

> ##### Examples
>
> Example schema-shape guidance, not a final schema:
>
> - A `Project` record should identify the project container, unit system, storage/privacy posture, rule-pack references, and report settings.
> - A `Node` record should expose coordinates with units and should support six-degree-of-freedom solver semantics by reference to the solver/domain model.
> - An `Element` record should reference start/end nodes, material and section/component data, local coordinate basis, and result-station metadata where needed.
> - A `Material` or `Component` record should carry user/private or public/permissive provenance status before engineering reliance or redistribution.
> - A `Result` record should distinguish mechanical result quantities from user-rule-check outcomes and should attach diagnostics/warnings to affected objects.
> - A `Report` record should identify input manifest, model hash, warnings, assumptions, provenance summary, result summaries, rule-pack checksum references, and professional-boundary notice.
>
> All examples above are structural guidance from `docs/SPEC.md` Sections 3, 7, and 8 and `docs/PRD.md` Sections 10, 13, and 15. They are not code-compliance requirements and do not supply engineering values.
>

### CLM-035 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict (short statement) | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | C-02-01-001 | Objective mapping differs: DEL-02-01 context/deliverable register lists OBJ-001, while SOW-041 scope ledger also maps OBJ-012. | `_CONTEXT.md` Objective Support; `docs/_Registers/Deliverables.csv` row DEL-02-01; `execution/_Decomposition/SOFTWARE_DECOMP.md` Objective table | `docs/_Registers/ScopeLedger.csv` row SOW-041 | Datasheet Identification; Specification Scope; Procedure Records | Treat OBJ-001 as the deliverable-owned objective for this run and treat OBJ-012 as directionally relevant package/scope context until a human updates registers. | TBD |
> | C-02-01-002 | Decomposition revision wording is stale in pointers: `_REFERENCES.md` says accepted v0.2 and `docs/README.md` status says v0.3, while `_CONTEXT.md`, the user brief, and the current decomposition basis identify revision 0.7. | `_REFERENCES.md` Decomposition and Registers; `docs/README.md` Status | `_CONTEXT.md` Decomposition Reference and Architecture Basis Injection; user brief | Datasheet References; Procedure Prerequisites; run record | Use `_CONTEXT.md` revision 0.7 and the user-provided `DECOMPOSITION_REF` for this run; leave metadata/governance pointers unchanged because they are outside write scope. | TBD |
>
> No engineering-value conflict was found in the accessible sources. No protected standards/code data was used.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-041 SOW-065 OBJ-001 OBJ-012 OBJ-014 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
