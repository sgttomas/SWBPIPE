---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-02-02
package_id: PKG-02
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@69ac259a7113d5a838fb22aa2e84df0e0f109713
project_scope_refs: [SOW-025]
package_objective_refs: [OBJ-001, OBJ-012]
---

# Scope of Work — DEL-02-02

## Purpose and Objective Traceability

This Scope of Work defines `DEL-02-02` in service of project scope [SOW-025] and package objectives [OBJ-001, OBJ-012].

- **OUT-001** — A unit-system and dimensional-analysis core contract specifying dimensions, conversion rules, storage conventions, and unit-test obligations is produced for the declared scope and objectives.

## Deliverable Definition — Ontology

### CLM-001 — Source preamble

> ---
> doc_id: DEL-02-02-DATASHEET
> doc_kind: deliverable.datasheet
> status: draft
> created: 2026-04-30
> ---
>

### CLM-002 — Datasheet - Unit System and Dimensional-Analysis Core Contract

> #### Datasheet - Unit System and Dimensional-Analysis Core Contract
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-02-02 |
> | Package ID | PKG-02 |
> | Package | Domain Model, Units, and Core Schemas |
> | Deliverable name | Unit system and dimensional-analysis core contract |
> | Type | BACKEND_FEATURE_SLICE |
> | Scope item | SOW-025 |
> | Objectives | OBJ-001, OBJ-012 |
> | Anticipated artifacts | `core/units` module contract; unit tests; `docs/SPEC.md` section |
> | Sealed context basis | `_CONTEXT.md` revision 0.7; `SOFTWARE_DECOMP.md` revision 0.7 |
> | Draft status | Draft/proposal until accepted by a human review gate per `docs/CONTRACT.md` OPS-K-AGENT-4 |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Datasheet value | Source / status |
> |---|---|---|
> | Core responsibility | Define the software contract for unit-aware calculations, schemas, imports, exports, and rule evaluations. | `docs/_Registers/ScopeLedger.csv` row SOW-025; `docs/CONTRACT.md` OPS-K-UNIT-1 |
> | Architectural owner | Domain core owns unit invariant enforcement; adapters must not bypass unit, provenance, or data-boundary checks. | `docs/SPEC.md` Sections 1, 2; `SOFTWARE_DECOMP.md` Section 8.1 AB-00-02 and AB-00-07 |
> | Public schema baseline | JSON Schema 2020-12 for public schemas/interchange. | `_CONTEXT.md` Architecture Basis Injection; `SOFTWARE_DECOMP.md` Section 8.2 |
> | Persistence/hash baseline | Versioned, unit-aware, schema-governed JSON persistence. The current Python persistence boundary is sorted-key compact ASCII-escaped JSON labeled `SORTED_COMPACT_JSON`, explicitly not JCS; RFC 8785 requires a later governed implementation and proof. | `_CONTEXT.md` Architecture Basis Injection; `SOFTWARE_DECOMP.md` Section 8.1 AB-00-04 and Section 8.2; D-41 R5 T2A |
> | Runtime baseline | Rust core/application services. Exact dependency versions remain TBD. | `_CONTEXT.md` Architecture Basis Injection; `SOFTWARE_DECOMP.md` Section 8.2 |
> | Result/diagnostic envelope link | Unit errors and warnings must fit the project diagnostic/result-envelope model where emitted. | `_CONTEXT.md` applicable AB-00-06; `docs/SPEC.md` Section 7 |
> | Unit catalog | TBD. No authoritative unit catalog, conversion table, or dimensional registry was supplied in the accessible references. | `_REFERENCES.md` Package-Specific References; `docs/CONTRACT.md` OPS-K-AGENT-1 |
> | Minimal test catalog | TBD. Early unit tests need a minimal source-backed or human-decision-backed unit catalog and conversion source set before executable conversion cases rely on it. | `docs/_Registers/ScopeLedger.csv` row SOW-025; `docs/VALIDATION_STRATEGY.md` Section 2; `docs/CONTRACT.md` OPS-K-AGENT-1 |
> | Dimensional basis | ASSUMPTION: the first implementation will need base/derived dimensions covering geometry, force, moment, pressure/stress, density, temperature, angle/rotation, and time-related loads. Human review is required before treating this as normative. | Inferred from `docs/SPEC.md` Sections 3-5 and deliverable scope; not explicitly enumerated in sources |
> | Conversion factors | TBD. Conversion factors must be sourced from public/permissive references or well-documented implementation decisions before release use. | `docs/IP_AND_DATA_BOUNDARY.md` Sections 2-4; `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-IP-2, OPS-K-UNIT-1 |
> | Test obligations | Automated tests for units, schemas, invalid unit rejection, deterministic conversions, rule-pack unit mismatch, and reproducibility hooks. | `docs/SPEC.md` Section 9; `docs/VALIDATION_STRATEGY.md` Section 2; `SOFTWARE_DECOMP.md` Section 8.1 AB-00-08 |
>

### CLM-005 — Conditions

> ##### Conditions
>
> | Condition | Requirement / note | Source |
> |---|---|---|
> | No silent defaults | Missing solve-required or rule-check-required values must surface as findings, not implicit defaults. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/DIRECTIVE.md` Sections 2.2, 3 |
> | No protected standards data | The unit contract must not embed protected standards text, tables, copyrighted examples, proprietary formulas, protected dimensional tables, or commercial data without documented rights. | `docs/CONTRACT.md` OPS-K-IP-1; `docs/IP_AND_DATA_BOUNDARY.md` Section 3 |
> | Professional boundary | Unit correctness supports analysis. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/DIRECTIVE.md` Sections 3-4 |
> | Adapter/plugin boundary | Import/export adapters and plugins must validate units and cannot bypass validation, diagnostics, provenance, sandboxing, or data-boundary controls. | `SOFTWARE_DECOMP.md` Section 8.1 AB-00-07; `docs/SPEC.md` Section 1 |
> | Determinism | Unit conversion and dimensional checking must be deterministic and testable across calculations, schemas, imports, exports, and rule evaluations. | `docs/_Registers/ScopeLedger.csv` row SOW-025; `docs/SPEC.md` Section 4.5; `docs/VALIDATION_STRATEGY.md` Section 2 |
> | Human review | This draft does not resolve exact unit list, conversion constants, numeric representation, or tolerances. | `docs/CONTRACT.md` OPS-K-AGENT-1 and OPS-K-AGENT-4 |
>

### CLM-006 — Construction

> ##### Construction
>
> The DEL-02-02 implementation surface should be constructed as a domain-core unit contract with at least these artifacts:
>
> | Construct | Draft content |
> |---|---|
> | `core/units` module contract | Quantity model, unit registry, dimension-signature model, conversion API, dimensional algebra API, diagnostics, and schema serialization rules. Exact Rust module path is TBD unless established by later repository layout decisions. |
> | Schema hooks | JSON Schema 2020-12 definitions for quantities, unit references, dimension signatures, and unit-aware fields. Exact schema filenames and code-generation tooling are TBD. |
> | Storage convention | Persist unit-bearing values with explicit unit metadata and schema versioning. PROPOSAL: preserve the user-entered unit representation for audit/round-trip behavior and use a canonical calculation representation internally. Human ruling required. |
> | Conversion registry | Deterministic registry with explicit source/provenance metadata for each conversion family. Exact factor representation, allowed units, and precision/tolerance policy are TBD. |
> | Dimensionless classification | TBD. Explicit dimensionless fields, ratios, percentages, coefficients, and unitless-where-unit-required failures need a human-approved classification before schema/API freeze. |
> | Diagnostics | Unit mismatch, unknown unit, ambiguous offset/absolute quantity, missing unit, and unitless-where-unit-required cases should produce structured diagnostics. Exact diagnostic codes are TBD. |
> | Decision owner | TBD. Human review/decision ownership is required for open catalog, dimension-basis, storage, conversion, diagnostic-code, and special-quantity decisions before the contract is treated as issued. |
> | Tests | Cargo unit tests and schema validation tests covering valid conversions, invalid conversions, dimensional algebra, serialization round trips, imports/exports, rule evaluation, and reproducibility/hash behavior where JSON payloads are hashed. |
>

### CLM-007 — References

> ##### References
>
> - `_CONTEXT.md` revision 0.7 for sealed deliverable identity, package exclusions, artifacts, and SCA-001 basis IDs.
> - `_REFERENCES.md` for accessible source set and note that no package-specific source material beyond governance/register content was introduced by PREPARATION.
> - `docs/CONTRACT.md` Section 1 for invariants OPS-K-IP-1 through OPS-K-IP-3, OPS-K-DATA-2, OPS-K-AUTH-1, OPS-K-UNIT-1, OPS-K-AGENT-1 through OPS-K-AGENT-4.
> - `docs/SPEC.md` Sections 1-3, 6-9, 11 for layer ownership, `core/units`, domain objects with units, rule evaluator constraints, diagnostics, reports, tests, and acceptance semantics.
> - `docs/DIRECTIVE.md` Sections 2-5 for unit safety, provenance, no silent defaults, professional boundary, and stop rules.
> - `docs/IP_AND_DATA_BOUNDARY.md` Sections 2-7 for public/private data and provenance limits.
> - `docs/VALIDATION_STRATEGY.md` Sections 2-5 for unit/schema test and benchmark-source expectations.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 Sections 4-8 for SOW-025, OBJ-001, OBJ-012, PKG-02, DEL-02-02, and SCA-001 architecture basis.
> - `docs/_Registers/Deliverables.csv`, `ScopeLedger.csv`, and `ContextBudgetQA.csv` rows for DEL-02-02 and SOW-025.

### CLM-008 — D-41 R5 T2A persistence hash evidence (2026-07-12)

> ##### D-41 R5 T2A persistence hash evidence (2026-07-12)
>
> Project persistence emits `SORTED_COMPACT_JSON` checksum metadata and `sorted_compact_json_payload` canonical-truth metadata for sorted-key compact ASCII-escaped Python JSON. These labels describe the implemented deterministic byte contract and do not claim RFC 8785/JCS.
>

### CLM-009 — D-41 R5 T2B unit-authority evidence (2026-07-12)

> ##### D-41 R5 T2B unit-authority evidence (2026-07-12)
>
> `core/units/schema_vocabulary.py` reads the accepted `DimensionId` enum from `schemas/units.schema.yaml`; DEL-16-02 validation-preview code now consumes that adapter instead of owning a parallel 30-ID set. Focused tests prove exact vocabulary identity and absence of the former literal mirror.
>

### CLM-010 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. DEC-018 and the implemented core-units slice now provide the current dimensional basis. Alias/parser behavior, diagnostics, B2/B3 coverage, and validation matters survive only where explicitly recorded as residuals; this documentation refresh makes no lifecycle or validation ruling.

## Completion and Reliance Basis — Epistemology

### CLM-011 — Source preamble

> ---
> doc_id: DEL-02-02-SPECIFICATION
> doc_kind: deliverable.specification
> status: draft
> created: 2026-04-30
> ---
>

### CLM-012 — Specification - Unit System and Dimensional-Analysis Core Contract

> #### Specification - Unit System and Dimensional-Analysis Core Contract
>

### CLM-013 — Scope

> ##### Scope
>
> This specification covers the DEL-02-02 backend feature slice: a draft contract for unit dimensions, conversion rules, storage conventions, and unit test obligations for OpenPipeStress unit-aware data flow.
>
> In scope:
>
> - Unit and dimension concepts needed by calculations, schemas, imports, exports, and rule evaluations.
> - Domain-core contract boundaries for `core/units`.
> - JSON Schema 2020-12 quantity/storage hooks.
> - Deterministic conversion and dimensional-checking requirements.
> - Unit-related diagnostics and test obligations.
>
> Out of scope:
>
> - Numerical solver implementation, GUI views, and full persistence implementation, per PKG-02 exclusions in `_CONTEXT.md`.
> - Protected standards/code data, proprietary unit tables, protected dimensional tables, vendor catalog data, or code-compliance logic.
> - Final selection of exact unit catalog, conversion constants, numeric representation, precision/tolerance policy, and schema filenames. These are `TBD` unless later accepted by human review.
>

### CLM-014 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source / status |
> |---|---|---|
> | U-001 | The unit contract shall make all calculations, formulas, imported values, exports, schemas, and rule evaluations unit-aware and dimensionally checked. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/_Registers/ScopeLedger.csv` row SOW-025 |
> | U-002 | The domain core shall own unit invariant enforcement. Adapters, plugins, and import/export paths shall validate units instead of bypassing the domain contract. | `docs/SPEC.md` Section 1; `SOFTWARE_DECOMP.md` Section 8.1 AB-00-02 and AB-00-07 |
> | U-003 | Unit-bearing numeric values crossing schema, service, import/export, persistence, solver, load, stress, report, or rule-pack boundaries shall carry explicit unit metadata unless explicitly typed as dimensionless. | Derived from `docs/CONTRACT.md` OPS-K-UNIT-1 and `docs/SPEC.md` Sections 3, 6, 8; ASSUMPTION for exact field shape |
> | U-004 | The unit contract shall define a stable dimensional-signature representation for each supported unit. The exact base dimensions, derived dimensions, and dimension identifier vocabulary are TBD. | Deliverable description; `docs/CONTRACT.md` OPS-K-ID-1; `docs/CONTRACT.md` OPS-K-AGENT-1 |
> | U-005 | Addition, subtraction, comparison, conversion, and rule-pack checks shall reject incompatible dimensions. Multiplication/division and exponent operations shall produce explicit derived dimensions where supported. | Derived from dimensional-analysis purpose and OPS-K-UNIT-1; ASSUMPTION pending human approval of operation set |
> | U-006 | Unit conversion shall be deterministic and testable. Unknown units, missing units, ambiguous conversions, or incompatible conversions shall produce explicit diagnostics rather than fallback defaults. | `docs/_Registers/ScopeLedger.csv` row SOW-025; `docs/CONTRACT.md` OPS-K-DATA-2; `docs/DIRECTIVE.md` Section 3 |
> | U-007 | Conversion-factor records shall include source/provenance and redistribution status when stored as public data. Records suspected of protected or proprietary origin shall be quarantined and escalated. | `docs/IP_AND_DATA_BOUNDARY.md` Sections 3-5; `docs/CONTRACT.md` OPS-K-IP-1 through OPS-K-IP-3 |
> | U-008 | Persisted project/schema payloads shall be versioned, unit-aware, schema-governed, provenance-preserving, migration-aware, and round-trip testable. The architecture objective is deterministic canonical JSON; the current Python persistence boundary uses the explicitly non-JCS `SORTED_COMPACT_JSON` byte contract unless a later governed RFC 8785 implementation is proven. | `SOFTWARE_DECOMP.md` Section 8.1 AB-00-04; `_CONTEXT.md` Architecture Basis Injection; D-41 R5 T2A |
> | U-009 | Public schemas/interchange definitions for quantities, units, dimensions, and unit-bearing fields shall use the JSON Schema 2020-12 baseline. Exact schema location and code-generation tooling are TBD. | `_CONTEXT.md` Architecture Basis Injection; `SOFTWARE_DECOMP.md` Section 8.2; `docs/_Registers/ScopeLedger.csv` row SOW-041 as related context |
> | U-010 | Unit-related diagnostics and result-envelope fields, where emitted, shall include enough machine-readable context to identify code, class, severity, source, affected object, message, remediation, and provenance. Exact unit diagnostic code names are TBD. | `_CONTEXT.md` applicable AB-00-06; `SOFTWARE_DECOMP.md` Section 8.1 AB-00-06 |
> | U-011 | The unit contract shall preserve the boundary between mechanics solve, user-rule checks, and human approval. Unit checks must not be presented as code compliance, certification, approval, sealing, or authentication (PRD §21.2). | `docs/CONTRACT.md` OPS-K-AUTH-1, OPS-K-MECH-2; `SOFTWARE_DECOMP.md` Section 8.1 AB-00-03 |
> | U-012 | The unit test set shall cover dimension compatibility, incompatible-unit rejection, deterministic conversion behavior, serialization round trips, schema validation, import/export paths, rule-pack unit mismatch, and hash/reproducibility behavior for JSON payloads where applicable. | `docs/SPEC.md` Section 9; `docs/VALIDATION_STRATEGY.md` Section 2; `SOFTWARE_DECOMP.md` Section 8.1 AB-00-08 |
> | U-013 | Missing values, unsupported unit dimensions, unresolved conversion constants, and unapproved assumptions shall remain visible as `TBD`, diagnostics, or open issues; the implementation shall not silently invent engineering values. | `docs/CONTRACT.md` OPS-K-AGENT-1 and OPS-K-AGENT-2; `INIT.md` Agent rule |
> | U-014 | Public examples and test fixtures for unit behavior shall use original, invented, public-domain, or permissively licensed data. Protected code examples and commercial software examples require documented permission before use. | `docs/IP_AND_DATA_BOUNDARY.md` Sections 2-3; `docs/VALIDATION_STRATEGY.md` Section 5 |
> | U-015 | Fields represented as dimensionless, ratio, percentage, coefficient, or intentionally unitless shall be classified explicitly. A missing physical unit where a unit-bearing value is required shall be rejected or diagnosed rather than treated as dimensionless. Exact categories and schema fields are TBD. | `docs/CONTRACT.md` OPS-K-UNIT-1 and OPS-K-DATA-2; `docs/DIRECTIVE.md` Section 3; ASSUMPTION for exact classification vocabulary |
> | U-016 | Unit identifiers and aliases shall be deterministic and shall reject ambiguous parsing. Exact namespace, alias policy, and parser behavior are TBD until accepted by human review. | `docs/_Registers/ScopeLedger.csv` row SOW-025; `docs/CONTRACT.md` OPS-K-UNIT-1 and OPS-K-AGENT-1 |
>

### CLM-015 — Open Contract Decisions

> ###### Open Contract Decisions
>
> | Decision | Current status | Required disposition |
> |---|---|---|
> | Base dimension vector | TBD | Human-approved design decision before schema/API freeze. |
> | Unit identifier namespace and aliases | TBD | Human-approved design decision; aliases must not create ambiguous parsing. |
> | Dimensionless classification | TBD | Human-approved design decision for dimensionless fields, ratios, percentages, coefficients, and unitless-where-unit-required diagnostics before schema/API freeze. |
> | Numeric representation for conversion factors and stored magnitudes | TBD | Human-approved design decision; must support deterministic testing. |
> | Offset quantities, especially temperature scale versus temperature interval | TBD | Human-approved design decision before accepting temperature conversions. |
> | Gauge versus absolute pressure semantics | TBD | Human-approved design decision before pressure-bearing schemas rely on it. |
> | Ratios, percentages, and unitless engineering coefficients | TBD | Human-approved design decision before related schemas or rule-pack variables rely on these as dimensionless. |
> | Angle and rotation dimensional treatment | ASSUMPTION: explicit semantics needed even if represented as dimensionless in some calculations. | Human ruling required. |
> | Canonical calculation unit basis | TBD | Human-approved design decision tied to solver and persistence behavior. |
> | Persisted quantity shape and hash canonicalization | Quantity-shape choice remains TBD; current persistence hash bytes are ruled evidence, not a quantity-shape ruling. | Decide whether persistent payloads store entered units, canonical calculation representation, or both. The current Python persistence hash boundary is sorted-compact and explicitly non-JCS; a later JCS change requires governed implementation and evidence. |
> | Schema file layout and tooling | TBD | Decide exact schema filenames, locations, and code-generation or validation tooling for JSON Schema 2020-12 quantity definitions. |
> | Unit diagnostic code namespace | TBD | Decide stable unit diagnostic code names and map them to result-envelope fields before downstream consumers depend on them. |
> | Conversion constants and tolerance policy | TBD | Executable deterministic conversion tests must wait for approved constants, representation, and tolerances; placeholders may be tracked before then. |
> | Public unit/conversion source set | TBD | Must satisfy provenance and redistribution requirements. |
> | Human decision owner / review gate | TBD | Identify the human owner or review gate for open contract decisions before treating the unit contract as issued. |
>

### CLM-016 — Standards

> ##### Standards
>
> No protected engineering code or standards-body source text was available or used for this deliverable. The governing standards for this draft are project-local governance and architecture-basis sources:
>
> | Source | Applicable content |
> |---|---|
> | `docs/CONTRACT.md` | Binding invariants for unit safety, protected data, missing data, professional boundary, and agent discipline. |
> | `docs/SPEC.md` | Technical architecture, `core/units` location, domain objects, rule evaluator constraints, diagnostics, reporting, and V&V expectations. |
> | `docs/DIRECTIVE.md` | Founding intent, unit-safety principle, no silent defaults, data boundary, and stop rules. |
> | `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data and provenance rules for conversion/unit data and fixtures. |
> | `docs/VALIDATION_STRATEGY.md` | Unit/schema verification expectations and benchmark-source restrictions. |
> | `execution/_Decomposition/SOFTWARE_DECOMP.md` | Revision 0.7 scope, SOW-025, DEL-02-02, OBJ-001, OBJ-012, and SCA-001 architecture basis IDs. |
> | `_CONTEXT.md` | Sealed context, applicable architecture basis IDs, package exclusions, and anticipated artifacts. |
>

### CLM-017 — Verification

> ##### Verification
>
> | Requirement IDs | Verification approach |
> |---|---|
> | U-001, U-003, U-004, U-005 | Unit tests for dimension signatures, compatible operations, incompatible operation rejection, and dimensionless exceptions. |
> | U-003, U-015 | Schema and unit tests proving explicit dimensionless classifications are accepted only where intended and unit-bearing fields reject missing or disguised units. |
> | U-002, U-006, U-010 | Contract/API tests proving adapter-facing and service-facing calls cannot bypass unit validation and return structured diagnostics for unit failures. |
> | U-004, U-016 | Parser/registry tests for unit identifiers and aliases, including ambiguous alias rejection once the namespace is approved. |
> | U-006, U-012 | Deterministic conversion test matrix with identity, inverse, round-trip, and incompatible conversion cases. Exact constants and tolerances are TBD; executable expected values are gated on approved constants, representation, and tolerance policy. |
> | U-010 | Diagnostic mapping tests proving unit error codes populate result-envelope fields consistently once the unit diagnostic namespace is approved. |
> | U-007, U-014 | Protected-content/provenance review for unit/conversion data and test fixtures. |
> | U-008, U-009 | JSON Schema validation and serialization round-trip tests for unit-bearing fields. |
> | U-008 | Exact-byte, fixed-hash, ordering, mutation, and round-trip tests for the declared serialization label; do not claim JCS without RFC 8785 conformance evidence. |
> | U-011 | Report/result-envelope review to confirm unit checks are not described as professional approval or code compliance (PRD §21.2). |
> | U-013 | Review checklist confirming all unknowns are represented as `TBD`, explicit assumptions, diagnostics, or open decisions. |
>

### CLM-018 — Documentation

> ##### Documentation
>
> Required deliverable documentation artifacts:
>
> - `core/units` module contract documenting quantity shape, dimension signature, conversion API, operation rules, diagnostics, and storage/serialization hooks.
> - Unit test inventory or test plan covering the verification matrix above.
> - `docs/SPEC.md` unit-system section or equivalent linked technical specification update.
> - ADR or decision record for any accepted unit registry, conversion-factor representation, numeric representation, canonical calculation basis, or offset/gauge semantics that affect public schemas or persistent files, per AB-00-01.
> - Schema location/tooling decision record for JSON Schema 2020-12 quantity definitions before downstream schema consumers implement against them.
> - Diagnostic-code decision record for stable unit error names and result-envelope mapping before downstream consumers implement against them.
> - Open-decision list for unresolved `TBD` items.

### CLM-019 — D-41 R5 T2A persistence hash requirement (2026-07-12)

> ##### D-41 R5 T2A persistence hash requirement (2026-07-12)
>
> Project persistence SHALL label existing sorted-key compact ASCII-escaped Python JSON hashes `SORTED_COMPACT_JSON` and canonical payload truth `sorted_compact_json_payload`. Those labels SHALL NOT imply RFC 8785/JCS conformance.
>

### CLM-020 — D-41 R5 T2B unit-authority boundary (2026-07-12)

> ##### D-41 R5 T2B unit-authority boundary (2026-07-12)
>
> Python adapter/application validation SHALL derive the canonical `DimensionId` vocabulary from `schemas/units.schema.yaml` through the PKG-02 schema-vocabulary adapter rather than maintaining a parallel literal set. This removes the identified DEL-16-02 mirror but does not assert system-wide B2/B3 completion or independent numeric validation.
>

### CLM-021 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. DEC-018 and the implemented core-units slice now provide the current dimensional basis. Alias/parser behavior, diagnostics, B2/B3 coverage, and validation matters survive only where explicitly recorded as residuals; this documentation refresh makes no lifecycle or validation ruling.

- **AC-001** — The contract preserves the accepted source requirements for explicit dimensions, compatible conversion and storage behavior, deterministic checks, and fail-closed treatment of missing or incompatible units.

## Production and Verification Method — Praxeology

### CLM-022 — Source preamble

> ---
> doc_id: DEL-02-02-PROCEDURE
> doc_kind: deliverable.procedure
> status: draft
> created: 2026-04-30
> ---
>

### CLM-023 — Procedure - Unit System and Dimensional-Analysis Core Contract

> #### Procedure - Unit System and Dimensional-Analysis Core Contract
>

### CLM-024 — Purpose

> ##### Purpose
>
> This procedure describes how to produce and use the DEL-02-02 unit-system contract without exceeding PKG-02 scope. It is intended for later implementation work on the `core/units` module contract, unit tests, and the corresponding `docs/SPEC.md` section.
>

### CLM-025 — Prerequisites

> ##### Prerequisites
>
> | Prerequisite | Status / source |
> |---|---|
> | Sealed DEL-02-02 context | Present in `_CONTEXT.md` revision 0.7. |
> | Scope and objectives | DEL-02-02, PKG-02, SOW-025, OBJ-001, OBJ-012 from `_CONTEXT.md` and `docs/_Registers/Deliverables.csv`. |
> | Applicable invariants | OPS-K-UNIT-1, OPS-K-DATA-2, OPS-K-IP-1 through OPS-K-IP-3, OPS-K-AUTH-1, OPS-K-AGENT-1 through OPS-K-AGENT-4 from `docs/CONTRACT.md`. |
> | Architecture basis | AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08 from `_CONTEXT.md`. |
> | Human dependency list | Not provided; `_DEPENDENCIES.md` says coordination is externally human-owned. |
> | Human decision owner / review gate | TBD; required before open unit catalog, dimension basis, conversion, storage, diagnostic-code, schema-location, and special-quantity decisions can be treated as accepted. |
> | Authoritative unit catalog | TBD; not supplied in accessible references. |
> | Public/protected data policy | `docs/IP_AND_DATA_BOUNDARY.md` available. |
> | Test and validation basis | `docs/SPEC.md` Section 9 and `docs/VALIDATION_STRATEGY.md` available. |
>

### CLM-026 — Steps

> ##### Steps
>
> 1. Confirm scope and boundaries.
>
>    Verify the work is limited to DEL-02-02: unit dimensions, conversion rules, storage conventions, and unit test obligations. Do not implement solver behavior, GUI behavior, rule-pack content, protected standards content, or professional compliance claims.
>
> 2. Inventory unit-bearing surfaces.
>
>    Enumerate all domain surfaces that carry unit-bearing values: project units, coordinates, sections, materials, supports, loads, combinations, solver results, stress results, reports, imports, exports, and rule-pack variables. Mark any uncertain surface `TBD`.
>
> 3. Define the quantity contract.
>
>    Draft the domain type and schema concept for a quantity. At minimum, include a numeric magnitude and explicit unit reference unless the field is explicitly dimensionless. Classify dimensionless, ratio, percentage, coefficient, and intentionally unitless fields explicitly. Decide whether the persisted payload preserves entered unit representation, canonical calculation representation, or both. Current recommendation is a PROPOSAL, not final.
>
> 4. Define the dimension-signature contract.
>
>    Create a stable way to represent dimensional compatibility for units and derived quantities. Identify base dimensions, derived dimensions, exponent rules, and operation rules. Keep the dimension vocabulary `TBD` until human review accepts it.
>
> 5. Define conversion behavior.
>
>    Specify deterministic conversion registry behavior: accepted unit identifiers, alias rules, ambiguous-alias rejection, exact or toleranced factor representation, identity/inverse behavior, incompatible conversion rejection, unknown-unit handling, and source/provenance requirements. Do not add conversion constants from protected or undocumented sources.
>
> 6. Define special quantity semantics.
>
>    Record explicit decisions for offset temperature, temperature interval, gauge/absolute pressure, angle/rotation, ratios, percentages, and any unitless engineering coefficients. Leave unresolved cases `TBD` and make unsupported conversions fail visibly.
>
> 7. Define schema and storage conventions.
>
>    Bind unit-bearing fields to JSON Schema 2020-12 definitions. Require versioned, unit-aware, schema-governed, provenance-preserving, round-trip-testable storage. At the current Python persistence boundary, hash the declared sorted-key compact JSON bytes and label them `SORTED_COMPACT_JSON`; do not claim JCS without a governed RFC 8785 implementation and conformance evidence.
>
> 8. Define diagnostics.
>
>    Map unit failures to structured diagnostics/result envelopes where emitted. Include machine-readable code/class/severity/source/affected-object/message/remediation/provenance information. Proposed unit diagnostic codes remain TBD until the diagnostic contract is implemented.
>
> 9. Define adapter and rule-evaluator obligations.
>
>    Require imports, exports, plugins, and rule packs to use the domain unit contract. Adapters must reject or flag missing/ambiguous/incompatible units and cannot bypass validation, provenance, sandboxing, report controls, or public/private data boundaries.
>
> 10. Define the test plan.
>
>    Build tests for dimension signatures, compatible and incompatible operations, deterministic conversions, serialization round trips, JSON Schema validation, imports/exports, rule-pack unit mismatch, diagnostics, and JSON hash stability where applicable. Fixture data must be invented, original, public-domain, or permissively licensed.
>
> 11. Record design decisions.
>
>    Use an ADR or equivalent decision record for accepted decisions that affect public schemas, persistent files, or external contracts: decision owner/review gate, dimension basis, unit namespace, alias policy, numeric representation, canonical calculation basis, schema file layout/tooling, diagnostic-code namespace, and special quantity semantics.
>
> 12. Update documentation.
>
>    Update the `core/units` contract and `docs/SPEC.md` section with the accepted contract, remaining `TBD` items, verification obligations, and professional/IP boundary notes.
>

### CLM-027 — Verification

> ##### Verification
>
> | Check | Evidence |
> |---|---|
> | Scope remains DEL-02-02 only | No solver, GUI, rule-pack content, or persistence implementation is introduced by this procedure. |
> | Default unknown handling | All unsupported dimensions, conversion constants, and special semantics are `TBD`, explicit assumptions, open decisions, or diagnostics. |
> | Unit-aware surfaces | Inventory covers calculations, schemas, imports, exports, and rule evaluations from SOW-025. |
> | Schema baseline | Quantity and unit-bearing fields use JSON Schema 2020-12. |
> | Determinism | Conversion and round-trip tests define repeatable expected outcomes and tolerances once constants are approved. |
> | Alias ambiguity | Unit identifier parser/registry tests reject ambiguous aliases once the namespace is approved. |
> | Dimensionless classification | Schema and unit tests distinguish explicit dimensionless fields from missing units on unit-bearing fields. |
> | Decision ownership | Open contract decisions have a human owner or review gate before schemas/APIs are frozen. |
> | Data boundary | Unit/conversion data and fixtures include provenance/redistribution status and pass protected-content review. |
> | Professional boundary | Documentation avoids certification, approval, sealing, authentication, or code-compliance claims (PRD §21.2). |
> | Architecture basis | ADRs, schema-first envelopes, deterministic persistence/hash behavior, diagnostics, adapter validation, and layered tests are addressed where applicable. |
>

### CLM-028 — Records

> ##### Records
>
> Maintain these records when this procedure is executed by later implementation tasks:
>
> - Unit contract document or module-level design notes for `core/units`.
> - JSON Schema 2020-12 definitions for quantities, units, and dimensions.
> - ADRs or decision records for accepted unit/dimension/conversion/storage decisions.
> - Decision-owner or review-gate record for open contract decisions.
> - Unit conversion registry with source/provenance/redistribution fields.
> - Unit test plan and test results.
> - Schema validation and serialization round-trip results.
> - Protected-content/provenance review results for any public fixture or conversion data.
> - Open-decision register for unresolved `TBD` items.
> - Human review acceptance record before treating the contract as issued.

### CLM-029 — D-41 R5 T2A persistence hash check (2026-07-12)

> ##### D-41 R5 T2A persistence hash check (2026-07-12)
>
> Run exact-byte and fixed-SHA vectors, equivalent-order stability, mutation sensitivity, schema/fixture validation, SQLite round-trip tests, and a no-JCS assertion for the project persistence byte contract.
>

### CLM-030 — D-41 R5 T2B unit-authority check (2026-07-12)

> ##### D-41 R5 T2B unit-authority check (2026-07-12)
>
> Load `DimensionId` through the PKG-02 schema-vocabulary adapter, compare it to the schema enum, and scan the consuming Python engine for a reintroduced literal mirror. Continue to surface unbound system boundaries and unselected namespace/diagnostic policies as held residuals.
>

### CLM-031 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. DEC-018 and the implemented core-units slice now provide the current dimensional basis. Alias/parser behavior, diagnostics, B2/B3 coverage, and validation matters survive only where explicitly recorded as residuals; this documentation refresh makes no lifecycle or validation ruling.

- **VER-001** — Validate the contract and review source parity, dimensional consistency, conversion and storage rules, deterministic unit checks, explicit findings, and professional-reliance boundaries.

## Governing Values and Decisions — Axiology

### CLM-032 — Source preamble

> ---
> doc_id: DEL-02-02-GUIDANCE
> doc_kind: deliverable.guidance
> status: draft
> created: 2026-04-30
> ---
>

### CLM-033 — Guidance - Unit System and Dimensional-Analysis Core Contract

> #### Guidance - Unit System and Dimensional-Analysis Core Contract
>

### CLM-034 — Purpose

> ##### Purpose
>
> DEL-02-02 exists to give the solver, schema layer, rule evaluator, adapters, persistence, and reporting surfaces one conservative unit contract. Its central job is to prevent untagged or dimensionally incompatible engineering values from moving through the system silently.
>
> This guidance is source-grounded in the sealed DEL-02-02 context, SOW-025, the project invariant catalog, the technical specification, the IP/data-boundary policy, validation strategy, and the SCA-001 architecture basis. It does not introduce protected code text, standards tables, commercial examples, or code-compliance claims.
>

### CLM-035 — Principles

> ##### Principles
>
> | Principle | Guidance | Source |
> |---|---|---|
> | Unit safety is mandatory | Treat unit metadata and dimensional compatibility as part of the domain model, not as UI decoration or optional import metadata. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/DIRECTIVE.md` Section 3 |
> | Missing data is visible | Unknown units, missing units, unsupported conversion constants, or incomplete rule-check inputs should become diagnostics, `TBD`, or explicit open issues. | `docs/CONTRACT.md` OPS-K-DATA-2; `INIT.md` Agent rule |
> | Preserve boundaries | Unit checks support mechanics and rule evaluation; acceptance and professional judgment remain with the responsible engineer. | `docs/CONTRACT.md` OPS-K-AUTH-1; `SOFTWARE_DECOMP.md` Section 8.1 AB-00-03 |
> | Keep public data clean | Unit and conversion data introduced as public records need provenance and redistribution status; suspected protected content is stopped and escalated. | `docs/IP_AND_DATA_BOUNDARY.md` Sections 3-5 |
> | Determinism beats convenience | Prefer explicit registries, strict parsers, and repeatable conversion behavior over permissive parsing or hidden fallback conversion. | `docs/_Registers/ScopeLedger.csv` row SOW-025; `docs/VALIDATION_STRATEGY.md` Section 2 |
> | Make adapters boring | Imports, exports, plugins, and adapters should pass through the same unit validation and diagnostics as internal services. | `SOFTWARE_DECOMP.md` Section 8.1 AB-00-07; `docs/SPEC.md` Section 1 |
> | Keep schema and runtime aligned | JSON Schema quantity definitions and Rust domain-core types should represent the same contract. | `_CONTEXT.md` Architecture Basis Injection; `docs/SPEC.md` Sections 1-2 |
>

### CLM-036 — Considerations

> ##### Considerations
>

### CLM-037 — Unit catalog and conversion sources

> ###### Unit catalog and conversion sources
>
> The accessible sources do not provide an authoritative unit catalog or conversion-factor list. A future implementation should therefore avoid presenting any first-pass unit list as final. Each accepted unit family and conversion factor should be backed by a permissible source or explicit project decision record. Public fixtures should remain invented or permissively sourced.
>
> Before executable conversion tests or schema examples depend on a catalog, record the minimal early-test unit set and conversion source set as `TBD` or as a human-approved decision. The lensing worklist does not supply those units or factors.
>

### CLM-038 — Dimensional signatures

> ###### Dimensional signatures
>
> Dimensional signatures should be stable identifiers or vectors that support equality and compatibility checks across calculations, schemas, imports, exports, rule evaluation, and reports. The exact basis is TBD. ASSUMPTION: likely first-pass dimensions include length, force, mass, time, temperature, angle/rotation, pressure/stress, moment, density, and stiffness-related derived dimensions, because the domain objects and solver/load/stress surfaces in `docs/SPEC.md` require these concepts. This assumption requires human review.
>

### CLM-039 — Offset and reference quantities

> ###### Offset and reference quantities
>
> Some engineering quantities require more than multiplicative scaling. Temperature scales versus temperature intervals, gauge versus absolute pressure, rotations versus mathematical angles, and normalized ratios need explicit semantics. Until those semantics are accepted, related conversions should remain `TBD` or emit blocking diagnostics.
>

### CLM-040 — Storage and reproducibility

> ###### Storage and reproducibility
>
> The architecture basis requires unit-aware, schema-governed, deterministic round trips and an explicit JSON hash byte contract. The current Python persistence implementation is sorted-key compact ASCII-escaped JSON and is not JCS. Preserving the user-entered unit representation helps auditability and round-trip behavior, while a canonical calculation representation helps deterministic computation. The recommended draft direction is to preserve the entered unit plus use explicit rules for calculations and hashes, but the final quantity data shape remains a human design decision.
>

### CLM-041 — Diagnostics

> ###### Diagnostics
>
> Unit errors should be specific enough for a GUI, CLI, report, rule evaluator, or adapter to identify what failed and how to remediate it. Use project diagnostic classes where applicable. For example, missing physical units for a solve-required load may be solve-blocking; a rule-pack input with incompatible units may be rule-check-blocking. Exact diagnostic code names are TBD.
>

### CLM-042 — Protected-content boundary

> ###### Protected-content boundary
>
> Do not seed the unit system with copied standards tables, protected dimensional tables, code-derived formulas, commercial benchmark files, or proprietary vendor data. Unit mechanics and schemas may be public; protected or private engineering data belongs in user-controlled private paths unless redistribution rights are documented.
>

### CLM-043 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Conservative direction |
> |---|---|
> | Broad unit catalog now vs. small verified catalog first | Start with the smallest source-backed or decision-backed catalog needed for tests and early solver work; expand through review. |
> | Preserve entered units vs. normalize everything | Preserve entered units for audit/round-trip behavior and define a separate canonical calculation/hash path if accepted. |
> | Floating point factors vs. exact/rational/decimal factors | TBD. Choose only after considering deterministic testing, Rust implementation cost, schema representation, and required tolerances. |
> | Permissive import parsing vs. strict imports | Prefer strict imports with explicit diagnostics; adapters should not guess units silently. |
> | Unit mismatch warning vs. hard error | Treat incompatible dimensions in calculations and schema validation as blocking. Use warnings only for provenance weakness or assumptions that do not invalidate the computation. |
> | Embed conversion examples vs. keep examples deferred | Keep numeric examples deferred until sources or invented examples are reviewed for provenance and protected-content safety. |
>

### CLM-044 — Examples

> ##### Examples
>
> TBD. The accessible sources do not provide source-backed numeric unit-conversion examples or an accepted quantity schema. Do not add numeric examples until the schema, unit catalog, conversion constants, and tolerance policy have accepted sources or human decisions. Future examples should be original/invented or public/permissive, and should include:
>
> - a valid same-dimension conversion case;
> - an invalid cross-dimension conversion case;
> - a unit-aware load or material-property schema fragment;
> - a rule-pack unit mismatch case;
> - a serialization round-trip case showing preserved unit metadata;
> - a JSON hash-stability case where unit-bearing JSON payloads are hashed.
>

### CLM-045 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | C-DEL-02-02-001 | Repository overview text still describes v0.3 as the current downstream decomposition basis, while the sealed deliverable context and decomposition for this run identify revision 0.7 with SCA-001 injection. | `docs/README.md` governance map text | `_CONTEXT.md` Decomposition Reference and Architecture Basis Injection; `execution/_Decomposition/SOFTWARE_DECOMP.md` revision notes | References, run record, source basis | Use the sealed `_CONTEXT.md` revision 0.7 and `SOFTWARE_DECOMP.md` revision 0.7 for this deliverable. | TBD |

### CLM-046 — D-41 R5 T2A persistence hash guidance (2026-07-12)

> ##### D-41 R5 T2A persistence hash guidance (2026-07-12)
>
> Keep project persistence, model-state consumers, headless-runner consumers, schemas, and fixtures aligned to the narrow sorted-compact labels. Do not call this serializer JCS without a governed RFC 8785 implementation and conformance suite.
>

### CLM-047 — D-41 R5 T2B unit-authority guidance (2026-07-12)

> ##### D-41 R5 T2B unit-authority guidance (2026-07-12)
>
> Use the schema-vocabulary adapter for Python dimension membership checks. A matching copied set is not an authority binding. Rust mechanics, imports/exports, rule packs, reports, and remaining adapters still require their own accepted boundary wiring and evidence; do not infer system-wide closure from this one corrected consumer.
>

### CLM-048 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. DEC-018 and the implemented core-units slice now provide the current dimensional basis. Alias/parser behavior, diagnostics, B2/B3 coverage, and validation matters survive only where explicitly recorded as residuals; this documentation refresh makes no lifecycle or validation ruling.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-025 OBJ-001 OBJ-012 | CLM-011 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
