---
doc_id: DEL-02-02-SPECIFICATION
doc_kind: deliverable.specification
status: draft
created: 2026-04-30
---

# Specification - Unit System and Dimensional-Analysis Core Contract

## Scope

This specification covers the DEL-02-02 backend feature slice: a draft contract for unit dimensions, conversion rules, storage conventions, and unit test obligations for OpenPipeStress unit-aware data flow.

In scope:

- Unit and dimension concepts needed by calculations, schemas, imports, exports, and rule evaluations.
- Domain-core contract boundaries for `core/units`.
- JSON Schema 2020-12 quantity/storage hooks.
- Deterministic conversion and dimensional-checking requirements.
- Unit-related diagnostics and test obligations.

Out of scope:

- Numerical solver implementation, GUI views, and full persistence implementation, per PKG-02 exclusions in `_CONTEXT.md`.
- Protected standards/code data, proprietary unit tables, protected dimensional tables, vendor catalog data, or code-compliance logic.
- Final selection of exact unit catalog, conversion constants, numeric representation, precision/tolerance policy, and schema filenames. These are `TBD` unless later accepted by human review.

## Requirements

| ID | Requirement | Source / status |
|---|---|---|
| U-001 | The unit contract shall make all calculations, formulas, imported values, exports, schemas, and rule evaluations unit-aware and dimensionally checked. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/_Registers/ScopeLedger.csv` row SOW-025 |
| U-002 | The domain core shall own unit invariant enforcement. Adapters, plugins, and import/export paths shall validate units instead of bypassing the domain contract. | `docs/SPEC.md` Section 1; `SOFTWARE_DECOMP.md` Section 8.1 AB-00-02 and AB-00-07 |
| U-003 | Unit-bearing numeric values crossing schema, service, import/export, persistence, solver, load, stress, report, or rule-pack boundaries shall carry explicit unit metadata unless explicitly typed as dimensionless. | Derived from `docs/CONTRACT.md` OPS-K-UNIT-1 and `docs/SPEC.md` Sections 3, 6, 8; ASSUMPTION for exact field shape |
| U-004 | The unit contract shall define a stable dimensional-signature representation for each supported unit. The exact base dimensions, derived dimensions, and dimension identifier vocabulary are TBD. | Deliverable description; `docs/CONTRACT.md` OPS-K-ID-1; `docs/CONTRACT.md` OPS-K-AGENT-1 |
| U-005 | Addition, subtraction, comparison, conversion, and rule-pack checks shall reject incompatible dimensions. Multiplication/division and exponent operations shall produce explicit derived dimensions where supported. | Derived from dimensional-analysis purpose and OPS-K-UNIT-1; ASSUMPTION pending human approval of operation set |
| U-006 | Unit conversion shall be deterministic and testable. Unknown units, missing units, ambiguous conversions, or incompatible conversions shall produce explicit diagnostics rather than fallback defaults. | `docs/_Registers/ScopeLedger.csv` row SOW-025; `docs/CONTRACT.md` OPS-K-DATA-2; `docs/DIRECTIVE.md` Section 3 |
| U-007 | Conversion-factor records shall include source/provenance and redistribution status when stored as public data. Records suspected of protected or proprietary origin shall be quarantined and escalated. | `docs/IP_AND_DATA_BOUNDARY.md` Sections 3-5; `docs/CONTRACT.md` OPS-K-IP-1 through OPS-K-IP-3 |
| U-008 | Persisted project/schema payloads shall be versioned, unit-aware, schema-governed, provenance-preserving, migration-aware, and round-trip testable. JSON payload hashes shall use the accepted canonical JSON/JCS-compatible basis where JSON payloads are hashed. | `SOFTWARE_DECOMP.md` Section 8.1 AB-00-04; `_CONTEXT.md` Architecture Basis Injection |
| U-009 | Public schemas/interchange definitions for quantities, units, dimensions, and unit-bearing fields shall use the JSON Schema 2020-12 baseline. Exact schema location and code-generation tooling are TBD. | `_CONTEXT.md` Architecture Basis Injection; `SOFTWARE_DECOMP.md` Section 8.2; `docs/_Registers/ScopeLedger.csv` row SOW-041 as related context |
| U-010 | Unit-related diagnostics and result-envelope fields, where emitted, shall include enough machine-readable context to identify code, class, severity, source, affected object, message, remediation, and provenance. Exact unit diagnostic code names are TBD. | `_CONTEXT.md` applicable AB-00-06; `SOFTWARE_DECOMP.md` Section 8.1 AB-00-06 |
| U-011 | The unit contract shall preserve the boundary between mechanics solve, user-rule checks, and human approval. Unit checks must not be presented as code compliance, certification, approval, sealing, or authentication. | `docs/CONTRACT.md` OPS-K-AUTH-1, OPS-K-MECH-2; `SOFTWARE_DECOMP.md` Section 8.1 AB-00-03 |
| U-012 | The unit test set shall cover dimension compatibility, incompatible-unit rejection, deterministic conversion behavior, serialization round trips, schema validation, import/export paths, rule-pack unit mismatch, and hash/reproducibility behavior for JSON payloads where applicable. | `docs/SPEC.md` Section 9; `docs/VALIDATION_STRATEGY.md` Section 2; `SOFTWARE_DECOMP.md` Section 8.1 AB-00-08 |
| U-013 | Missing values, unsupported unit dimensions, unresolved conversion constants, and unapproved assumptions shall remain visible as `TBD`, diagnostics, or open issues; the implementation shall not silently invent engineering values. | `docs/CONTRACT.md` OPS-K-AGENT-1 and OPS-K-AGENT-2; `INIT.md` Agent rule |
| U-014 | Public examples and test fixtures for unit behavior shall use original, invented, public-domain, or permissively licensed data. Protected code examples and commercial software examples require documented permission before use. | `docs/IP_AND_DATA_BOUNDARY.md` Sections 2-3; `docs/VALIDATION_STRATEGY.md` Section 5 |
| U-015 | Fields represented as dimensionless, ratio, percentage, coefficient, or intentionally unitless shall be classified explicitly. A missing physical unit where a unit-bearing value is required shall be rejected or diagnosed rather than treated as dimensionless. Exact categories and schema fields are TBD. | `docs/CONTRACT.md` OPS-K-UNIT-1 and OPS-K-DATA-2; `docs/DIRECTIVE.md` Section 3; ASSUMPTION for exact classification vocabulary |
| U-016 | Unit identifiers and aliases shall be deterministic and shall reject ambiguous parsing. Exact namespace, alias policy, and parser behavior are TBD until accepted by human review. | `docs/_Registers/ScopeLedger.csv` row SOW-025; `docs/CONTRACT.md` OPS-K-UNIT-1 and OPS-K-AGENT-1 |

### Open Contract Decisions

| Decision | Current status | Required disposition |
|---|---|---|
| Base dimension vector | TBD | Human-approved design decision before schema/API freeze. |
| Unit identifier namespace and aliases | TBD | Human-approved design decision; aliases must not create ambiguous parsing. |
| Dimensionless classification | TBD | Human-approved design decision for dimensionless fields, ratios, percentages, coefficients, and unitless-where-unit-required diagnostics before schema/API freeze. |
| Numeric representation for conversion factors and stored magnitudes | TBD | Human-approved design decision; must support deterministic testing. |
| Offset quantities, especially temperature scale versus temperature interval | TBD | Human-approved design decision before accepting temperature conversions. |
| Gauge versus absolute pressure semantics | TBD | Human-approved design decision before pressure-bearing schemas rely on it. |
| Ratios, percentages, and unitless engineering coefficients | TBD | Human-approved design decision before related schemas or rule-pack variables rely on these as dimensionless. |
| Angle and rotation dimensional treatment | ASSUMPTION: explicit semantics needed even if represented as dimensionless in some calculations. | Human ruling required. |
| Canonical calculation unit basis | TBD | Human-approved design decision tied to solver and persistence behavior. |
| Persisted quantity shape and hash canonicalization | TBD; current draft direction is proposal only. | Decide whether persistent payloads store entered units, canonical calculation representation, or both, and how this interacts with canonical JSON/JCS-compatible hashing where JSON payloads are hashed. |
| Schema file layout and tooling | TBD | Decide exact schema filenames, locations, and code-generation or validation tooling for JSON Schema 2020-12 quantity definitions. |
| Unit diagnostic code namespace | TBD | Decide stable unit diagnostic code names and map them to result-envelope fields before downstream consumers depend on them. |
| Conversion constants and tolerance policy | TBD | Executable deterministic conversion tests must wait for approved constants, representation, and tolerances; placeholders may be tracked before then. |
| Public unit/conversion source set | TBD | Must satisfy provenance and redistribution requirements. |
| Human decision owner / review gate | TBD | Identify the human owner or review gate for open contract decisions before treating the unit contract as issued. |

## Standards

No protected engineering code or standards-body source text was available or used for this deliverable. The governing standards for this draft are project-local governance and architecture-basis sources:

| Source | Applicable content |
|---|---|
| `docs/CONTRACT.md` | Binding invariants for unit safety, protected data, missing data, professional boundary, and agent discipline. |
| `docs/SPEC.md` | Technical architecture, `core/units` location, domain objects, rule evaluator constraints, diagnostics, reporting, and V&V expectations. |
| `docs/DIRECTIVE.md` | Founding intent, unit-safety principle, no silent defaults, data boundary, and stop rules. |
| `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data and provenance rules for conversion/unit data and fixtures. |
| `docs/VALIDATION_STRATEGY.md` | Unit/schema verification expectations and benchmark-source restrictions. |
| `docs/_Decomposition/SOFTWARE_DECOMP.md` | Revision 0.4 scope, SOW-025, DEL-02-02, OBJ-001, OBJ-012, and SCA-001 architecture basis IDs. |
| `_CONTEXT.md` | Sealed context, applicable architecture basis IDs, package exclusions, and anticipated artifacts. |

## Verification

| Requirement IDs | Verification approach |
|---|---|
| U-001, U-003, U-004, U-005 | Unit tests for dimension signatures, compatible operations, incompatible operation rejection, and dimensionless exceptions. |
| U-003, U-015 | Schema and unit tests proving explicit dimensionless classifications are accepted only where intended and unit-bearing fields reject missing or disguised units. |
| U-002, U-006, U-010 | Contract/API tests proving adapter-facing and service-facing calls cannot bypass unit validation and return structured diagnostics for unit failures. |
| U-004, U-016 | Parser/registry tests for unit identifiers and aliases, including ambiguous alias rejection once the namespace is approved. |
| U-006, U-012 | Deterministic conversion test matrix with identity, inverse, round-trip, and incompatible conversion cases. Exact constants and tolerances are TBD; executable expected values are gated on approved constants, representation, and tolerance policy. |
| U-010 | Diagnostic mapping tests proving unit error codes populate result-envelope fields consistently once the unit diagnostic namespace is approved. |
| U-007, U-014 | Protected-content/provenance review for unit/conversion data and test fixtures. |
| U-008, U-009 | JSON Schema validation and serialization round-trip tests for unit-bearing fields. |
| U-008 | Canonical JSON/JCS-compatible hash stability tests where unit-bearing JSON payloads are hashed. |
| U-011 | Report/result-envelope review to confirm unit checks are not described as professional approval or code compliance. |
| U-013 | Review checklist confirming all unknowns are represented as `TBD`, explicit assumptions, diagnostics, or open decisions. |

## Documentation

Required deliverable documentation artifacts:

- `core/units` module contract documenting quantity shape, dimension signature, conversion API, operation rules, diagnostics, and storage/serialization hooks.
- Unit test inventory or test plan covering the verification matrix above.
- `docs/SPEC.md` unit-system section or equivalent linked technical specification update.
- ADR or decision record for any accepted unit registry, conversion-factor representation, numeric representation, canonical calculation basis, or offset/gauge semantics that affect public schemas or persistent files, per AB-00-01.
- Schema location/tooling decision record for JSON Schema 2020-12 quantity definitions before downstream schema consumers implement against them.
- Diagnostic-code decision record for stable unit error names and result-envelope mapping before downstream consumers implement against them.
- Open-decision list for unresolved `TBD` items.
