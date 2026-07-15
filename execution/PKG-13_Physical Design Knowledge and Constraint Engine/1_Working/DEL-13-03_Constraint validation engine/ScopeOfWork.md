---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-13-03
package_id: PKG-13
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@2770fda4c63c98ee9f18cffbafd14c9aa59f497f
project_scope_refs: [SOW-068]
package_objective_refs: [OBJ-014]
---

# Scope of Work — DEL-13-03

## Purpose and Objective Traceability

This candidate defines `DEL-13-03` in service of project scope [SOW-068] and package objectives [OBJ-014].

- **OUT-001** — A deterministic constraint validation engine that reports provenance-aware messages for available connectivity, route, clearance, support-zone, slope/drain/vent, and missing-data constraints.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-13-03 Constraint validation engine

> #### Datasheet: DEL-13-03 Constraint validation engine
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-13-03-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value | Source |
> |---|---|---|
> | Deliverable ID | DEL-13-03 | `_CONTEXT.md` |
> | Name | Constraint validation engine | `_CONTEXT.md` |
> | Package ID | PKG-13 | `_CONTEXT.md` |
> | Package name | Physical Design Knowledge and Constraint Engine | `_CONTEXT.md` |
> | Type | BACKEND_FEATURE_SLICE | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-13-03 |
> | Scope item | SOW-068 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW ledger |
> | Objective support | OBJ-014 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` objective map |
> | Context envelope | M | `_CONTEXT.md` |
> | Anticipated artifacts | constraint validation module; validation diagnostics tests | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-13-03 |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Current grounded value |
> |---|---|
> | Primary function | Implement deterministic validation messages for available design constraints and missing data. Source: `_CONTEXT.md` Description. |
> | Validation scope categories | Connectivity, route conflicts, clearance conflicts, support-zone conflicts, slope/drain/vent conflicts, and missing required data. Source: SOW-068 in `_CONTEXT.md` and `execution/_Decomposition/SOFTWARE_DECOMP.md`. |
> | Message boundary | Constraint failures are validation messages, not hidden report prose or agent text. Source: `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-068 Notes. |
> | Authority boundary | The deliverable must not infer hidden owner standards, protected code requirements, or final engineering acceptance logic. Sources: `_CONTEXT.md` Context Envelope; `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-DATA-1, OPS-K-AUTH-1; `docs/IP_AND_DATA_BOUNDARY.md` sections 3 and 6. |
> | Architecture basis | Rust core/application services, JSON Schema 2020-12 contracts, schema-first envelopes, canonical JSON/JCS-compatible hash basis where JSON payloads are hashed, and layered test gates apply as dispatch context. Source: `_CONTEXT.md` Architecture Basis Injection. |
> | Current implementation location | `core/constraints/validation/engine.py`, exported through `core/constraints/validation/__init__.py`. Source: implementation evidence. |
> | Current validation API | `validate_constraint_envelope(constraint_envelope, design_knowledge_envelope=None)` returns `ValidationResult`; `diagnostic_dicts(...)` serializes diagnostics for stable comparison. Source: `core/constraints/validation/engine.py`; `tests/test_constraint_validation.py`. |
> | Current diagnostic record shape | Implemented diagnostics carry `code`, `severity`, `class`, `affected_references`, `source_references`, `message`, and `remediation`; `ValidationResult.to_dict()` adds `has_blocking_findings`. Formal application result-envelope integration remains `TBD`. Source: `core/constraints/validation/engine.py`. |
> | Current severity values | The implemented validator emits `blocking`, `warning`, and `info`; release readiness and human-acceptance policy for those severities remains `TBD`. Source: `core/constraints/validation/engine.py`. |
>

### CLM-005 — Conditions

> ##### Conditions
>
> | Condition | Grounded constraint |
> |---|---|
> | Input knowledge boundary | Validation is over available design knowledge and constraints; missing data is an explicit finding rather than a silently supplied default. Sources: SOW-068; `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` sections 4 and 4.3. |
> | Public/private data boundary | Public artifacts must not bundle protected standards text, protected tables, proprietary values, owner standards, or private project data. Sources: `docs/CONTRACT.md` OPS-K-IP-1 through OPS-K-IP-3; `docs/IP_AND_DATA_BOUNDARY.md` sections 2-6. |
> | Professional boundary | Software outputs are decision support and must not automatically claim certification, approval, sealing, authentication, code compliance, or professional reliance. Sources: `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/SPEC.md` section 4.3; `docs/DIRECTIVE.md` Professional boundary. |
> | Unit and provenance controls | Domain-core and adapter paths may not bypass unit checks, provenance checks, or public/private data boundaries. Sources: `docs/SPEC.md` sections 1 and 4. |
> | Upstream evidence surface | `Dependencies.csv` is a deliverable-local evidence surface under the current DAG-006 coordination basis. It preserves historical DAG-002 dependency IDs for audit while listing ACTIVE rows for architecture basis, design-knowledge schema/provenance, constraint entity/provenance, unit, diagnostics, and persistence predecessors. Source: `_DEPENDENCIES.md`; `Dependencies.csv`; `_COORDINATION.md`. |
> | Design-knowledge reference handling | The validator indexes supplied `design_knowledge.records` by `id` and emits unresolved-reference diagnostics when constraint references are absent from the supplied design-knowledge envelope. Source: `core/constraints/validation/engine.py`; `tests/test_constraint_validation.py`. |
> | Data-boundary handling | The validator checks data-boundary policy fields and emits `IP_BOUNDARY_WARNING` diagnostics for mismatches, protected-suspected provenance, private project data, or professional-boundary failures. Source: `core/constraints/validation/engine.py`; `tests/test_constraint_validation.py`. |
>

### CLM-006 — Construction

> ##### Construction
>
> Current product-code evidence exists and is limited to a stdlib-only Python validation slice. The implemented module accepts supplied mapping data so missing fields can become deterministic diagnostics instead of exceptions or hidden defaults.
>
> | Expected construction element | Status |
> |---|---|
> | Constraint validation module | Implemented at `core/constraints/validation/engine.py`; package exports are listed in `core/constraints/validation/__init__.py`. |
> | Validation diagnostics tests | Implemented in `tests/test_constraint_validation.py` using stdlib assertions and invented public test fixtures. |
> | Represented diagnostic classes | Tests cover `CONNECTIVITY_CONFLICT`, `CLEARANCE_CONFLICT`, `ROUTE_CONFLICT`, `SUPPORT_ZONE_CONFLICT`, `SLOPE_DRAIN_VENT_CONFLICT`, `CONSTRAINT_MISSING_DATA`, and `SCHEMA_VALIDATION`; implementation also emits `PROVENANCE_WARNING`, `UNIT_WARNING`, and `IP_BOUNDARY_WARNING`. |
> | Unit checks | Quantity parameters are checked for `value`, `unit`, `dimension`, and `provenance`; noncanonical dimensions produce `CV-UNIT-DIMENSION-UNKNOWN`. The validator does not convert units or invent tolerances. |
> | Dependency integration | Current documentation refresh does not edit dependency files. Local dependency rows remain an evidence surface under DAG-006 coordination, with preserved historical row IDs where present. |
> | Data examples | Executable invented fixtures exist in `tests/test_constraint_validation.py`. Publication-grade examples, owner/project examples, localization, full geometric conflict solving, runtime integration, release readiness, and human acceptance remain `TBD`. |
>

### CLM-007 — References

> ##### References
>
> - `_CONTEXT.md` - deliverable identity, scope, artifacts, architecture-basis injection.
> - `_REFERENCES.md` - governing reference list for this DEL folder.
> - `_DEPENDENCIES.md` and `Dependencies.csv` - deliverable-local dependency evidence surface under the current DAG-006 coordination basis.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` - accepted revision 0.7 package, scope, objective, and deliverable entries.
> - `docs/_Registers/Deliverables.csv` - row DEL-13-03.
> - `docs/_Registers/ScopeLedger.csv` - row SOW-068.
> - `docs/_Registers/ContextBudgetQA.csv` - row DEL-13-03.
> - `docs/CONTRACT.md` - invariant catalog.
> - `docs/SPEC.md` - technical and agentic implementation specification.
> - `docs/TYPES.md` - vocabulary and epistemic labels.
> - `docs/IP_AND_DATA_BOUNDARY.md` - protected-data and public/private data boundary policy.
> - `core/constraints/validation/engine.py` - implemented validation engine.
> - `core/constraints/validation/__init__.py` - implemented package export surface.
> - `tests/test_constraint_validation.py` - focused validation diagnostics tests and invented fixtures.

### CLM-008 — D-41 R5 T2C PDU-023 Evidence State

> ##### D-41 R5 T2C PDU-023 Evidence State
>
> Paired result-trace scalar paths are schema-valid and unpaired paths are rejected. Runtime envelope production remains `TBD` because this validator has no accepted producer/home.

## Completion and Reliance Basis — Epistemology

### CLM-009 — Specification: DEL-13-03 Constraint validation engine

> #### Specification: DEL-13-03 Constraint validation engine
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-010 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-13-03-DECL-001`.
>

### CLM-011 — Scope

> ##### Scope
>
> DEL-13-03 covers a backend feature slice for deterministic validation messages over available physical-design constraints and missing required data. The in-scope validation categories are connectivity, route conflicts, clearance conflicts, support-zone conflicts, slope/drain/vent conflicts, and missing required data. Sources: `_CONTEXT.md` Scope Detail; `execution/_Decomposition/SOFTWARE_DECOMP.md` rows SOW-068 and DEL-13-03.
>
> Excluded from this deliverable are hidden owner standards, protected code requirements, protected standards content, proprietary project data, final engineering acceptance logic, and automatic professional/code-compliance claims. Sources: `_CONTEXT.md` Package Exclusions; `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-DATA-1, OPS-K-AUTH-1; `docs/IP_AND_DATA_BOUNDARY.md` sections 3 and 6; `docs/SPEC.md` section 4.3.
>

### CLM-012 — Implementation Evidence

> ##### Implementation Evidence
>
> The current implementation evidence is `core/constraints/validation/engine.py`, exported through `core/constraints/validation/__init__.py`, and exercised by `tests/test_constraint_validation.py`. The module is a stdlib-only Python validation slice over supplied mapping data. It does not mutate accepted model state, compute geometric intersections, solve clearances, define owner criteria, or make final engineering decisions.
>
> The implemented API is `validate_constraint_envelope(constraint_envelope, design_knowledge_envelope=None) -> ValidationResult`, with `diagnostic_dicts(...)` for deterministic test comparison. `Diagnostic.to_dict()` emits `code`, `severity`, `class`, `affected_references`, `source_references`, `message`, and `remediation`; `ValidationResult.to_dict()` emits `diagnostics` plus `has_blocking_findings`.
>
> The focused test file uses invented public fixtures for constraint and design-knowledge envelopes. It verifies deterministic output, represented conflict classes, explicit missing-data and unresolved-reference findings, unit metadata and PKG-02 dimension checks, protected/private provenance diagnostics, and absence of prohibited authority-claim text.
>

### CLM-013 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source |
> |---|---|---|
> | DEL-13-03-R1 | The validation engine shall validate available design knowledge for the SOW-068 categories: connectivity, route conflicts, clearance conflicts, support-zone conflicts, slope/drain/vent conflicts, and missing required data. | `_CONTEXT.md` Scope Detail; `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-068 |
> | DEL-13-03-R2 | Validation outputs shall be deterministic validation messages rather than hidden report prose or agent text. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-068 Notes; DEL-13-03 row |
> | DEL-13-03-R3 | Missing required data shall be surfaced as explicit findings/diagnostics and shall not be silently defaulted. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` sections 4 and 4.3 |
> | DEL-13-03-R4 | Validation shall preserve provenance visibility for messages where source or availability of design knowledge affects reliance. | SOW-068 provenance-aware message requirement; `docs/SPEC.md` sections 1 and 4 |
> | DEL-13-03-R5 | The engine shall not infer hidden owner standards, protected code requirements, protected standards values, proprietary values, or final engineering acceptance logic. | `_CONTEXT.md` Context Envelope and Package Exclusions; `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-DATA-1, OPS-K-AUTH-1; `docs/IP_AND_DATA_BOUNDARY.md` sections 3 and 6 |
> | DEL-13-03-R6 | Outputs shall remain compatible with the accepted schema-first architecture basis and diagnostics/result-envelope boundary. The current implementation exposes a Python diagnostic dict shape; formal runtime result-envelope integration remains `TBD`. | `_CONTEXT.md` Architecture Basis Injection; `docs/SPEC.md` sections 1, 4.3, and 9; `core/constraints/validation/engine.py` |
> | DEL-13-03-R7 | The deliverable shall include validation diagnostics tests covering the grounded message categories and boundary conditions. | `_CONTEXT.md` Anticipated Artifacts; `docs/_Registers/Deliverables.csv` row DEL-13-03; `tests/test_constraint_validation.py` |
> | DEL-13-03-R8 | Documentation refresh and later bounded implementation work shall preserve current ACTIVE dependency rows in `Dependencies.csv` as predecessor evidence unless a later human-approved CHANGE process authorizes reclassification. | `_DEPENDENCIES.md`; `Dependencies.csv`; project instruction for this run |
>

### CLM-014 — Standards

> ##### Standards
>
> No external engineering-code text, tables, values, or clause-level requirements are locally available for this deliverable. The governing local standards for this evidence refresh are project governance, architecture sources, and current implementation evidence:
>
> | Reference | Applicability |
> |---|---|
> | `docs/CONTRACT.md` | Binding invariants for IP/data boundary, missing-data handling, units, authority boundaries, and agent non-invention. |
> | `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data and protected-content boundary. |
> | `docs/SPEC.md` | Technical architecture, domain-core boundary, unit/provenance controls, diagnostics/result-envelope posture, and no-compliance-claim boundary. |
> | `docs/TYPES.md` | Epistemic labels and canonical vocabulary for protected data, user-supplied code data, diagnostics, and professional approval. |
> | `execution/_Decomposition/SOFTWARE_DECOMP.md` | Accepted revision 0.7 current decomposition basis for DEL-13-03 scope and exclusions. |
> | `_CONTEXT.md` | Deliverable-local identity, architecture-basis injection, and control-surface constraints. |
> | `core/constraints/validation/engine.py` | Current validation module, diagnostic record implementation, boundary checks, and deterministic ordering. |
> | `tests/test_constraint_validation.py` | Current focused validation diagnostics tests and invented public fixtures. |
>

### CLM-015 — Verification

> ##### Verification
>
> | Requirement | Verification approach |
> |---|---|
> | DEL-13-03-R1 | `tests/test_constraint_validation.py` verifies represented conflict classes and available records for connectivity, clearance, no-go/route, support-zone, slope/drain/vent, access, equipment-interface, and missing-required-data inputs. |
> | DEL-13-03-R2 | `test_validation_is_deterministic_and_covers_represented_classes` repeats validation over deep-copied inputs and compares `diagnostic_dicts(...)`. |
> | DEL-13-03-R3 | `test_missing_data_and_unresolved_references_are_explicit_findings` verifies missing fields, missing targets/parameters, unresolved design-knowledge references, and missing provenance become diagnostics. |
> | DEL-13-03-R4 | Provenance and boundary tests verify diagnostics preserve `source_references` where supplied provenance carries source evidence. |
> | DEL-13-03-R5 | Boundary tests verify protected/private provenance diagnostics and scan output text for prohibited authority-claim terms. |
> | DEL-13-03-R6 | Current verification checks the Python diagnostic dict shape and `ValidationResult.to_dict()`; full application result-envelope integration remains `TBD`. |
> | DEL-13-03-R7 | Validation diagnostics tests are recorded in `tests/test_constraint_validation.py` and executable directly with `python3 tests/test_constraint_validation.py`. |
> | DEL-13-03-R8 | Dependency handling check confirms current ACTIVE local dependency rows remain unchanged unless a later human-approved CHANGE process replaces that instruction. |
>

### CLM-016 — Documentation

> ##### Documentation
>
> Required records and remaining deferrals:
>
> - Constraint validation module path and API boundary: `core/constraints/validation/engine.py`; `validate_constraint_envelope(...)`; `diagnostic_dicts(...)`.
> - Constraint diagnostic record shape: implemented Python dict fields are `code`, `severity`, `class`, `affected_references`, `source_references`, `message`, and `remediation`; formal result-envelope mapping remains `TBD`.
> - Validation diagnostics test inventory: `tests/test_constraint_validation.py`.
> - Public-safe validation fixtures: executable invented fixtures exist in `tests/test_constraint_validation.py`; publication-grade examples and owner/project examples remain `TBD` pending provenance review and human acceptance.
> - Assumptions and unsupported inputs: must be recorded as `TBD` or explicit findings rather than inferred defaults.
>

### CLM-017 — D-41 R5 T2C PDU-023 E6 Evidence (2026-07-12)

> ##### D-41 R5 T2C PDU-023 E6 Evidence (2026-07-12)
>
> Existing `ResultTraceLink` records may carry paired nonempty source/target scalar paths; schema tests reject unpaired paths. DEL-13-03 has no accepted runtime result-envelope producer/home, so R6 integration remains held and `ValidationResult.to_dict()` is not relabeled as an envelope.
> - Dependency evidence handling: preserve current ACTIVE local dependency rows during this evidence-refresh workflow.
> - Legitimate remaining `TBD` items include localization/message cataloging, full geometric conflict solving, owner criteria/rules, GUI presentation, physical-to-analytical transformation, runtime integration, release readiness, and human acceptance.

- **AC-001** — All authoritative source content is preserved and mapped, and the implemented engine retains deterministic explicit diagnostics, provenance visibility, missing-data handling, protected/private-data boundaries, and professional-authority exclusions while formal runtime result-envelope integration remains deferred.

## Production and Verification Method — Praxeology

### CLM-018 — Procedure: DEL-13-03 Constraint validation engine

> #### Procedure: DEL-13-03 Constraint validation engine
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-019 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-13-03-DECL-004`.
>

### CLM-020 — Purpose

> ##### Purpose
>
> This procedure describes how to verify the current DEL-13-03 validation artifact set and how later bounded changes should proceed without exceeding the grounded scope. It is not authority for engineering values, protected standards content, release readiness, or professional acceptance claims.
>

### CLM-021 — Prerequisites

> ##### Prerequisites
>
> | Prerequisite | Basis |
> |---|---|
> | Deliverable scope and identity loaded from `_CONTEXT.md` and `execution/_Decomposition/SOFTWARE_DECOMP.md`. | Four-documents source hierarchy. |
> | Approved local dependency mirror available as `Dependencies.csv`. | `_DEPENDENCIES.md`; `Dependencies.csv`. |
> | Architecture basis constraints applied only as applicable dispatch context. | `_CONTEXT.md` Architecture Basis Injection. |
> | No hidden owner standards, protected code requirements, proprietary values, or protected tables used as public defaults. | `docs/CONTRACT.md`; `docs/IP_AND_DATA_BOUNDARY.md`. |
> | Current validation module and focused tests available. | `core/constraints/validation/engine.py`; `core/constraints/validation/__init__.py`; `tests/test_constraint_validation.py`. |
> | Remaining `TBD` items are limited to deferred integration, presentation, engineering, release, and human-acceptance decisions. | `_STATUS.md`; `MEMORY.md`; current evidence refresh. |
>

### CLM-022 — Steps

> ##### Steps
>
> 1. Confirm the current work is scoped to DEL-13-03 and SOW-068.
>    - Evidence: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-13-03 and SOW-068 rows.
>
> 2. Inspect the implemented validation surface.
>    - Module: `core/constraints/validation/engine.py`.
>    - Package export surface: `core/constraints/validation/__init__.py`.
>    - Focused tests and invented fixtures: `tests/test_constraint_validation.py`.
>
> 3. Load only available design-knowledge and constraint inputs supplied by accepted schemas or test fixtures.
>    - Current API: `validate_constraint_envelope(constraint_envelope, design_knowledge_envelope=None)`.
>    - If required schema or fixture details are unavailable, record `TBD` or emit diagnostics rather than inventing fields.
>
> 4. Validate the SOW-068 categories when supporting input data exists:
>    - connectivity;
>    - route conflicts;
>    - clearance conflicts;
>    - support-zone conflicts;
>    - slope/drain/vent conflicts;
>    - missing required data.
>
> 5. Emit deterministic validation diagnostics for observed findings.
>    - Current diagnostic dict fields: `code`, `severity`, `class`, `affected_references`, `source_references`, `message`, and `remediation`.
>    - `ValidationResult.to_dict()` also reports `has_blocking_findings`.
>    - Messages must preserve source/provenance visibility where available and must not become hidden report prose or agent-only text.
>
> 6. Treat missing required data as explicit findings.
>    - Do not supply engineering defaults, code-specific values, owner-standard criteria, or proprietary values.
>
> 7. Enforce public/private and professional-boundary checks on fixtures, messages, and documentation.
>    - Reject or quarantine suspected protected content according to `docs/IP_AND_DATA_BOUNDARY.md`.
>    - Do not emit software-generated professional approval, code-compliance, certification, sealing, or authentication statuses.
>
> 8. Maintain validation diagnostics tests.
>    - Current coverage: deterministic output behavior, represented SOW-068 category handling, missing-data findings, unresolved references, provenance visibility, unit metadata and dimension checks, and boundary protection.
>    - Public documentation examples remain `TBD` unless invented or otherwise permitted examples are separately reviewed.
>
> 9. Preserve approved DAG-006 dependency evidence.
>    - Do not retire, delete, reclassify, or normalize current ACTIVE local dependency rows during this evidence-refresh workflow.
>

### CLM-023 — Verification

> ##### Verification
>
> | Check | Expected result |
> |---|---|
> | Scope check | Only files authorized by the active bounded brief are changed; product-code changes remain scoped to DEL-13-03 when a product-code brief authorizes them. |
> | Category check | SOW-068 categories are represented in requirements and tests. |
> | Determinism check | Repeated validation over deep-copied inputs yields identical `diagnostic_dicts(...)` output. |
> | Missing-data check | Required missing values produce explicit findings rather than defaults. |
> | Provenance check | Findings retain or reference input provenance through `source_references` where available. |
> | Boundary check | No protected standards text, owner defaults, proprietary values, or professional-approval/compliance statuses are introduced. |
> | Dependency check | Current dependency evidence remains ACTIVE and unchanged in `Dependencies.csv`; historical dependency IDs are preserved where present under the DAG-006 coordination basis. |
> | Syntax check | `python3 -m py_compile core/constraints/validation/engine.py tests/test_constraint_validation.py` completes successfully. |
> | Focused test check | `python3 tests/test_constraint_validation.py` completes successfully. |
>

### CLM-024 — Records

> ##### Records
>
> - Constraint validation module: `core/constraints/validation/engine.py`.
> - Package export surface: `core/constraints/validation/__init__.py`.
> - Validation diagnostics tests and invented fixtures: `tests/test_constraint_validation.py`.
> - Test fixture provenance/review records: required before public examples are relied on outside executable test evidence.
> - `TBD` list for localization, full geometric conflict solving, owner criteria/rules, GUI presentation, physical-to-analytical transformation, runtime integration, release readiness, and human acceptance.
> - Local dependency validation result from `tools/validation/validate_dependencies_schema.py` when `Dependencies.csv` exists.

### CLM-025 — D-41 R5 T2C PDU-023 Check

> ##### D-41 R5 T2C PDU-023 Check
>
> Validate paired field paths at the existing result-trace schema boundary. Do not emit a runtime envelope from the constraint validator until an accepted producer/home exists.

- **VER-001** — Validate the candidate schema and claim map; confirm parity and source/status hashes; run the focused constraint-validation and dependency checks; and independently review source grounding and authority boundaries.

## Governing Values and Decisions — Axiology

### CLM-026 — Guidance: DEL-13-03 Constraint validation engine

> #### Guidance: DEL-13-03 Constraint validation engine
>

### CLM-027 — Purpose

> ##### Purpose
>
> This deliverable exists to make physical-design constraint validation visible, deterministic, and provenance-aware before downstream transformation or review workflows rely on model data. It supports OBJ-014 by contributing to a schema-backed piping design model that captures physical design context, design knowledge, constraints, assumptions, and analytical transformation traceability. Sources: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` OBJ-014 and DEL-13-03 rows.
>

### CLM-028 — Principles

> ##### Principles
>
> | Principle | Guidance | Source |
> |---|---|---|
> | Validate available knowledge only | Treat available design knowledge and explicit constraints as the validation surface. Do not infer hidden owner standards or protected code criteria. | `_CONTEXT.md`; SOW-068; `docs/CONTRACT.md` OPS-K-AGENT-1 |
> | Missing data is a finding | Missing solve-required or rule-check-required values must become explicit diagnostics/findings, not silent defaults. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` sections 4 and 4.3 |
> | Preserve provenance | Where records include source/provenance, diagnostics should preserve reviewable source visibility through `source_references` and affected object references. Broader GUI presentation remains `TBD`. | SOW-068; `docs/SPEC.md` sections 1 and 4; `core/constraints/validation/engine.py` |
> | Keep authority boundaries visible | Constraint validation is not professional approval, code compliance, certification, sealing, or owner-standard interpretation. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/SPEC.md` section 4.3 |
> | Keep public artifacts clean | Do not copy protected standards text, protected tables, proprietary values, private owner data, or code-specific defaults into public fixtures, messages, or docs. | `docs/IP_AND_DATA_BOUNDARY.md` sections 3 and 6 |
> | Use schema-first boundaries | The current implementation exposes a deterministic Python diagnostic dict shape and `ValidationResult.to_dict()`. Application runtime/result-envelope integration remains `TBD`. | `_CONTEXT.md` Architecture Basis Injection; `docs/SPEC.md` sections 1 and 9; `core/constraints/validation/engine.py` |
>

### CLM-029 — Considerations

> ##### Considerations
>
> - The validation engine consumes supplied constraint and optional design-knowledge mapping envelopes. Current upstream DEL-13-02 evidence defines the constraint schema/provenance model; the local `Dependencies.csv` remains an evidence surface and is not edited by this refresh.
> - Constraint categories named by SOW-068 are scope categories, not complete engineering acceptance criteria. Any concrete clearance, slope, vent, drain, route, or support-zone value must come from user/project/private sources or later lawful public fixtures.
> - Provenance-aware messages can reveal where a value came from or that source evidence is missing, but they cannot create redistribution rights or legal clearance for protected data.
> - Deterministic ordering, stable `CV-*` diagnostic codes, and replayable tests now exist in the Python validation slice. Localization/message cataloging and release-grade diagnostic policy remain `TBD`.
> - The implementation validates represented conflict classes and referenced data availability; it does not perform full geometric conflict solving, route intersection, owner criteria/rule evaluation, GUI presentation, physical-to-analytical transformation, runtime integration, release readiness, or human acceptance.
>

### CLM-030 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Conservative direction |
> |---|---|
> | Helpful defaults vs. engineering safety | Prefer explicit missing-data findings over defaults. |
> | Broad validation vs. source authority | Validate only what the available design knowledge and accepted schemas can support. |
> | User-facing clarity vs. protected-content risk | Explain the missing or conflicting condition without quoting protected standards or private owner criteria. |
> | Strict blocking vs. reviewable warning | Use the implemented `blocking`, `warning`, and `info` severities as current diagnostic evidence; release policy and human acceptance remain outside this module. |
> | Early implementation detail vs. architecture stability | Treat the current Python module/API and diagnostic fields as implementation evidence, while keeping formal runtime integration and release architecture decisions human-gated. |
>

### CLM-031 — Examples

> ##### Examples
>
> Executable invented fixtures exist in `tests/test_constraint_validation.py` for deterministic validation, missing-data, unit, provenance, and authority-boundary checks. Public documentation examples remain `TBD`; any later published examples must use invented or otherwise permitted data with provenance/review status.
>

### CLM-032 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> No cross-source conflicts were identified in this evidence refresh. Remaining `TBD` items are bounded deferrals: localization, full geometric conflict solving, owner criteria/rules, GUI presentation, physical-to-analytical transformation, runtime integration, release readiness, and human acceptance.

### CLM-033 — D-41 R5 T2C PDU-023 Boundary

> ##### D-41 R5 T2C PDU-023 Boundary
>
> Schema capability is not runtime integration. Preserve the absent application-service/result-envelope home as a held residual.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-068 OBJ-014 | CLM-009 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
