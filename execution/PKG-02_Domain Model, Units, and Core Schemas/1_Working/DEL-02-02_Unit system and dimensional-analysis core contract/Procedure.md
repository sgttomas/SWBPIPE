---
doc_id: DEL-02-02-PROCEDURE
doc_kind: deliverable.procedure
status: draft
created: 2026-04-30
---

# Procedure - Unit System and Dimensional-Analysis Core Contract

## Purpose

This procedure describes how to produce and use the DEL-02-02 unit-system contract without exceeding PKG-02 scope. It is intended for later implementation work on the `core/units` module contract, unit tests, and the corresponding `docs/SPEC.md` section.

## Prerequisites

| Prerequisite | Status / source |
|---|---|
| Sealed DEL-02-02 context | Present in `_CONTEXT.md` revision 0.4. |
| Scope and objectives | DEL-02-02, PKG-02, SOW-025, OBJ-001, OBJ-012 from `_CONTEXT.md` and `docs/_Registers/Deliverables.csv`. |
| Applicable invariants | OPS-K-UNIT-1, OPS-K-DATA-2, OPS-K-IP-1 through OPS-K-IP-3, OPS-K-AUTH-1, OPS-K-AGENT-1 through OPS-K-AGENT-4 from `docs/CONTRACT.md`. |
| Architecture basis | AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08 from `_CONTEXT.md`. |
| Human dependency list | Not provided; `_DEPENDENCIES.md` says coordination is externally human-owned. |
| Human decision owner / review gate | TBD; required before open unit catalog, dimension basis, conversion, storage, diagnostic-code, schema-location, and special-quantity decisions can be treated as accepted. |
| Authoritative unit catalog | TBD; not supplied in accessible references. |
| Public/protected data policy | `docs/IP_AND_DATA_BOUNDARY.md` available. |
| Test and validation basis | `docs/SPEC.md` Section 9 and `docs/VALIDATION_STRATEGY.md` available. |

## Steps

1. Confirm scope and boundaries.

   Verify the work is limited to DEL-02-02: unit dimensions, conversion rules, storage conventions, and unit test obligations. Do not implement solver behavior, GUI behavior, rule-pack content, protected standards content, or professional compliance claims.

2. Inventory unit-bearing surfaces.

   Enumerate all domain surfaces that carry unit-bearing values: project units, coordinates, sections, materials, supports, loads, combinations, solver results, stress results, reports, imports, exports, and rule-pack variables. Mark any uncertain surface `TBD`.

3. Define the quantity contract.

   Draft the domain type and schema concept for a quantity. At minimum, include a numeric magnitude and explicit unit reference unless the field is explicitly dimensionless. Classify dimensionless, ratio, percentage, coefficient, and intentionally unitless fields explicitly. Decide whether the persisted payload preserves entered unit representation, canonical calculation representation, or both. Current recommendation is a PROPOSAL, not final.

4. Define the dimension-signature contract.

   Create a stable way to represent dimensional compatibility for units and derived quantities. Identify base dimensions, derived dimensions, exponent rules, and operation rules. Keep the dimension vocabulary `TBD` until human review accepts it.

5. Define conversion behavior.

   Specify deterministic conversion registry behavior: accepted unit identifiers, alias rules, ambiguous-alias rejection, exact or toleranced factor representation, identity/inverse behavior, incompatible conversion rejection, unknown-unit handling, and source/provenance requirements. Do not add conversion constants from protected or undocumented sources.

6. Define special quantity semantics.

   Record explicit decisions for offset temperature, temperature interval, gauge/absolute pressure, angle/rotation, ratios, percentages, and any unitless engineering coefficients. Leave unresolved cases `TBD` and make unsupported conversions fail visibly.

7. Define schema and storage conventions.

   Bind unit-bearing fields to JSON Schema 2020-12 definitions. Require versioned, unit-aware, schema-governed, provenance-preserving, round-trip-testable storage. Where JSON payloads are hashed, use the accepted canonical JSON/JCS-compatible hash basis.

8. Define diagnostics.

   Map unit failures to structured diagnostics/result envelopes where emitted. Include machine-readable code/class/severity/source/affected-object/message/remediation/provenance information. Proposed unit diagnostic codes remain TBD until the diagnostic contract is implemented.

9. Define adapter and rule-evaluator obligations.

   Require imports, exports, plugins, and rule packs to use the domain unit contract. Adapters must reject or flag missing/ambiguous/incompatible units and cannot bypass validation, provenance, sandboxing, report controls, or public/private data boundaries.

10. Define the test plan.

   Build tests for dimension signatures, compatible and incompatible operations, deterministic conversions, serialization round trips, JSON Schema validation, imports/exports, rule-pack unit mismatch, diagnostics, and JSON hash stability where applicable. Fixture data must be invented, original, public-domain, or permissively licensed.

11. Record design decisions.

   Use an ADR or equivalent decision record for accepted decisions that affect public schemas, persistent files, or external contracts: decision owner/review gate, dimension basis, unit namespace, alias policy, numeric representation, canonical calculation basis, schema file layout/tooling, diagnostic-code namespace, and special quantity semantics.

12. Update documentation.

   Update the `core/units` contract and `docs/SPEC.md` section with the accepted contract, remaining `TBD` items, verification obligations, and professional/IP boundary notes.

## Verification

| Check | Evidence |
|---|---|
| Scope remains DEL-02-02 only | No solver, GUI, rule-pack content, or persistence implementation is introduced by this procedure. |
| Default unknown handling | All unsupported dimensions, conversion constants, and special semantics are `TBD`, explicit assumptions, open decisions, or diagnostics. |
| Unit-aware surfaces | Inventory covers calculations, schemas, imports, exports, and rule evaluations from SOW-025. |
| Schema baseline | Quantity and unit-bearing fields use JSON Schema 2020-12. |
| Determinism | Conversion and round-trip tests define repeatable expected outcomes and tolerances once constants are approved. |
| Alias ambiguity | Unit identifier parser/registry tests reject ambiguous aliases once the namespace is approved. |
| Dimensionless classification | Schema and unit tests distinguish explicit dimensionless fields from missing units on unit-bearing fields. |
| Decision ownership | Open contract decisions have a human owner or review gate before schemas/APIs are frozen. |
| Data boundary | Unit/conversion data and fixtures include provenance/redistribution status and pass protected-content review. |
| Professional boundary | Documentation avoids certification, approval, sealing, authentication, or code-compliance claims. |
| Architecture basis | ADRs, schema-first envelopes, deterministic persistence/hash behavior, diagnostics, adapter validation, and layered tests are addressed where applicable. |

## Records

Maintain these records when this procedure is executed by later implementation tasks:

- Unit contract document or module-level design notes for `core/units`.
- JSON Schema 2020-12 definitions for quantities, units, and dimensions.
- ADRs or decision records for accepted unit/dimension/conversion/storage decisions.
- Decision-owner or review-gate record for open contract decisions.
- Unit conversion registry with source/provenance/redistribution fields.
- Unit test plan and test results.
- Schema validation and serialization round-trip results.
- Protected-content/provenance review results for any public fixture or conversion data.
- Open-decision register for unresolved `TBD` items.
- Human review acceptance record before treating the contract as issued.
