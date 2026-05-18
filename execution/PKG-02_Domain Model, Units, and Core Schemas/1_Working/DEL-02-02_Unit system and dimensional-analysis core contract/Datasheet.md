---
doc_id: DEL-02-02-DATASHEET
doc_kind: deliverable.datasheet
status: draft
created: 2026-04-30
---

# Datasheet - Unit System and Dimensional-Analysis Core Contract

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-02-02 |
| Package ID | PKG-02 |
| Package | Domain Model, Units, and Core Schemas |
| Deliverable name | Unit system and dimensional-analysis core contract |
| Type | BACKEND_FEATURE_SLICE |
| Scope item | SOW-025 |
| Objectives | OBJ-001, OBJ-012 |
| Anticipated artifacts | `core/units` module contract; unit tests; `docs/SPEC.md` section |
| Sealed context basis | `_CONTEXT.md` revision 0.4; `SOFTWARE_DECOMP.md` revision 0.4 |
| Draft status | Draft/proposal until accepted by a human review gate per `docs/CONTRACT.md` OPS-K-AGENT-4 |

## Attributes

| Attribute | Datasheet value | Source / status |
|---|---|---|
| Core responsibility | Define the software contract for unit-aware calculations, schemas, imports, exports, and rule evaluations. | `docs/_Registers/ScopeLedger.csv` row SOW-025; `docs/CONTRACT.md` OPS-K-UNIT-1 |
| Architectural owner | Domain core owns unit invariant enforcement; adapters must not bypass unit, provenance, or data-boundary checks. | `docs/SPEC.md` Sections 1, 2; `SOFTWARE_DECOMP.md` Section 8.1 AB-00-02 and AB-00-07 |
| Public schema baseline | JSON Schema 2020-12 for public schemas/interchange. | `_CONTEXT.md` Architecture Basis Injection; `SOFTWARE_DECOMP.md` Section 8.2 |
| Persistence/hash baseline | Versioned, unit-aware, schema-governed JSON persistence; JCS-compatible canonical JSON basis where JSON payloads are hashed. | `_CONTEXT.md` Architecture Basis Injection; `SOFTWARE_DECOMP.md` Section 8.1 AB-00-04 and Section 8.2 |
| Runtime baseline | Rust core/application services. Exact dependency versions remain TBD. | `_CONTEXT.md` Architecture Basis Injection; `SOFTWARE_DECOMP.md` Section 8.2 |
| Result/diagnostic envelope link | Unit errors and warnings must fit the project diagnostic/result-envelope model where emitted. | `_CONTEXT.md` applicable AB-00-06; `docs/SPEC.md` Section 7 |
| Unit catalog | TBD. No authoritative unit catalog, conversion table, or dimensional registry was supplied in the accessible references. | `_REFERENCES.md` Package-Specific References; `docs/CONTRACT.md` OPS-K-AGENT-1 |
| Minimal test catalog | TBD. Early unit tests need a minimal source-backed or human-decision-backed unit catalog and conversion source set before executable conversion cases rely on it. | `docs/_Registers/ScopeLedger.csv` row SOW-025; `docs/VALIDATION_STRATEGY.md` Section 2; `docs/CONTRACT.md` OPS-K-AGENT-1 |
| Dimensional basis | ASSUMPTION: the first implementation will need base/derived dimensions covering geometry, force, moment, pressure/stress, density, temperature, angle/rotation, and time-related loads. Human review is required before treating this as normative. | Inferred from `docs/SPEC.md` Sections 3-5 and deliverable scope; not explicitly enumerated in sources |
| Conversion factors | TBD. Conversion factors must be sourced from public/permissive references or well-documented implementation decisions before release use. | `docs/IP_AND_DATA_BOUNDARY.md` Sections 2-4; `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-IP-2, OPS-K-UNIT-1 |
| Test obligations | Automated tests for units, schemas, invalid unit rejection, deterministic conversions, rule-pack unit mismatch, and reproducibility hooks. | `docs/SPEC.md` Section 9; `docs/VALIDATION_STRATEGY.md` Section 2; `SOFTWARE_DECOMP.md` Section 8.1 AB-00-08 |

## Conditions

| Condition | Requirement / note | Source |
|---|---|---|
| No silent defaults | Missing solve-required or rule-check-required values must surface as findings, not implicit defaults. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/DIRECTIVE.md` Sections 2.2, 3 |
| No protected standards data | The unit contract must not embed protected standards text, tables, copyrighted examples, proprietary formulas, protected dimensional tables, or commercial data without documented rights. | `docs/CONTRACT.md` OPS-K-IP-1; `docs/IP_AND_DATA_BOUNDARY.md` Section 3 |
| Professional boundary | Unit correctness supports analysis, but software outputs must not claim certification, approval, sealing, authentication, or code compliance for reliance. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/DIRECTIVE.md` Sections 3-4 |
| Adapter/plugin boundary | Import/export adapters and plugins must validate units and cannot bypass validation, diagnostics, provenance, sandboxing, or data-boundary controls. | `SOFTWARE_DECOMP.md` Section 8.1 AB-00-07; `docs/SPEC.md` Section 1 |
| Determinism | Unit conversion and dimensional checking must be deterministic and testable across calculations, schemas, imports, exports, and rule evaluations. | `docs/_Registers/ScopeLedger.csv` row SOW-025; `docs/SPEC.md` Section 4.5; `docs/VALIDATION_STRATEGY.md` Section 2 |
| Human review | This draft does not resolve exact unit list, conversion constants, numeric representation, or tolerances. | `docs/CONTRACT.md` OPS-K-AGENT-1 and OPS-K-AGENT-4 |

## Construction

The DEL-02-02 implementation surface should be constructed as a domain-core unit contract with at least these artifacts:

| Construct | Draft content |
|---|---|
| `core/units` module contract | Quantity model, unit registry, dimension-signature model, conversion API, dimensional algebra API, diagnostics, and schema serialization rules. Exact Rust module path is TBD unless established by later repository layout decisions. |
| Schema hooks | JSON Schema 2020-12 definitions for quantities, unit references, dimension signatures, and unit-aware fields. Exact schema filenames and code-generation tooling are TBD. |
| Storage convention | Persist unit-bearing values with explicit unit metadata and schema versioning. PROPOSAL: preserve the user-entered unit representation for audit/round-trip behavior and use a canonical calculation representation internally. Human ruling required. |
| Conversion registry | Deterministic registry with explicit source/provenance metadata for each conversion family. Exact factor representation, allowed units, and precision/tolerance policy are TBD. |
| Dimensionless classification | TBD. Explicit dimensionless fields, ratios, percentages, coefficients, and unitless-where-unit-required failures need a human-approved classification before schema/API freeze. |
| Diagnostics | Unit mismatch, unknown unit, ambiguous offset/absolute quantity, missing unit, and unitless-where-unit-required cases should produce structured diagnostics. Exact diagnostic codes are TBD. |
| Decision owner | TBD. Human review/decision ownership is required for open catalog, dimension-basis, storage, conversion, diagnostic-code, and special-quantity decisions before the contract is treated as issued. |
| Tests | Cargo unit tests and schema validation tests covering valid conversions, invalid conversions, dimensional algebra, serialization round trips, imports/exports, rule evaluation, and reproducibility/hash behavior where JSON payloads are hashed. |

## References

- `_CONTEXT.md` revision 0.4 for sealed deliverable identity, package exclusions, artifacts, and SCA-001 basis IDs.
- `_REFERENCES.md` for accessible source set and note that no package-specific source material beyond governance/register content was introduced by PREPARATION.
- `docs/CONTRACT.md` Section 1 for invariants OPS-K-IP-1 through OPS-K-IP-3, OPS-K-DATA-2, OPS-K-AUTH-1, OPS-K-UNIT-1, OPS-K-AGENT-1 through OPS-K-AGENT-4.
- `docs/SPEC.md` Sections 1-3, 6-9, 11 for layer ownership, `core/units`, domain objects with units, rule evaluator constraints, diagnostics, reports, tests, and acceptance semantics.
- `docs/DIRECTIVE.md` Sections 2-5 for unit safety, provenance, no silent defaults, professional boundary, and stop rules.
- `docs/IP_AND_DATA_BOUNDARY.md` Sections 2-7 for public/private data and provenance limits.
- `docs/VALIDATION_STRATEGY.md` Sections 2-5 for unit/schema test and benchmark-source expectations.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4 Sections 4-8 for SOW-025, OBJ-001, OBJ-012, PKG-02, DEL-02-02, and SCA-001 architecture basis.
- `docs/_Registers/Deliverables.csv`, `ScopeLedger.csv`, and `ContextBudgetQA.csv` rows for DEL-02-02 and SOW-025.
