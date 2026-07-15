---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-14-04
package_id: PKG-14
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@e8f59a63372f38d9e788ac39b39995558f5aba73
project_scope_refs: [SOW-073, SOW-072]
package_objective_refs: [OBJ-016]
---

# Scope of Work — DEL-14-04

## Purpose and Objective Traceability

This Scope of Work defines `DEL-14-04` in service of project scope [SOW-073, SOW-072] and package objectives [OBJ-016].

- **OUT-001** — A deterministic analysis-run comparison contract for exact and manual mappings, unit-aware result deltas, diagnostics, settings, and tolerance-driven classification is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-14-04 Analysis-run comparison engine

> #### Datasheet: DEL-14-04 Analysis-run comparison engine
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-14-04-DECL-002`.
>
> **Generated:** 2026-05-03
> **Document Role:** Descriptive
> **Source Basis:** `_CONTEXT.md`, `_REFERENCES.md`, `Dependencies.csv`, `execution/_Decomposition/SOFTWARE_DECOMP.md`, `docs/CONTRACT.md`, `docs/SPEC.md`, `docs/TYPES.md`, `docs/IP_AND_DATA_BOUNDARY.md`
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value | Source |
> |---|---|---|
> | Deliverable ID | DEL-14-04 | `_CONTEXT.md` |
> | Name | Analysis-run comparison engine | `_CONTEXT.md` |
> | Package ID | PKG-14 | `_CONTEXT.md` |
> | Package name | Model States, Analysis Runs, and Comparison | `_CONTEXT.md` |
> | Type | BACKEND_FEATURE_SLICE | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` |
> | Scope items | SOW-073, SOW-072 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` |
> | Objective support | OBJ-016 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` |
> | Anticipated artifacts | run comparison engine; result delta tests | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Recorded Value |
> |---|---|
> | Primary function | Implement unit-normalized result deltas for mapped entities, diagnostics, and settings. |
> | Comparison subject | Analysis runs and/or model states, as bounded by SOW-073; this deliverable focuses on analysis-run comparison. |
> | Required comparison basis | Stable IDs, manual mappings where required, unit-normalized result deltas, and tolerance profiles. |
> | Analysis-run basis | Exact model states, solver versions, settings, units, load cases, diagnostics, results, rule-pack references, library references, and result hashes. |
> | Entity/result coverage | Mapped nodes, elements, supports, terminals, stress/result locations, diagnostics, and settings. |
> | Role boundary | Diagnostic/audit functionality only; not automatic external validation, professional approval, certification, sealing, authentication, or code-compliance determination. |
> | Architecture basis | Rust core/application services; schema-first command/query/job result envelopes; JSON Schema 2020-12; JCS-compatible hash basis for JSON payloads when hashed; Cargo/Vitest/Playwright/validation/protected-content gates as applicable. |
> | Open implementation decisions | Exact dependency versions, solver numerical library, public API transport, import/export format list, CI provider/coverage thresholds, physical project package/container, comparison tolerance defaults, and mapping workflows remain `TBD` unless separately approved. |
>

### CLM-005 — Conditions

> ##### Conditions
>
> - The comparison engine must remain source-grounded in PKG-14 scope: immutable model states, analysis-run records, deterministic state/run comparison, mappings, tolerances, and comparison exports.
> - `ASSUMPTION`: This deliverable consumes analysis-run records, mapping/tolerance contracts, result-export envelopes, and unit-system contracts as upstream evidence surfaces because `Dependencies.csv` lists those rows as ACTIVE DAG-002 mirror entries.
> - The local dependency mirror records eleven ACTIVE rows and identifies the source of truth as approved DAG-006 coordination. These rows are evidence surface only and are not reclassified by this datasheet.
> - Missing or ambiguous units on unit-bearing physical values are diagnostics, not silent defaults, per `docs/SPEC.md` unit-contract text.
> - Result export envelopes must preserve units, diagnostics, provenance, hashes/status boundaries, and professional-boundary notices per `docs/SPEC.md` and `docs/TYPES.md`.
> - Comparison tolerance defaults and mapping workflows are explicitly `TBD` in `execution/_Decomposition/SOFTWARE_DECOMP.md` open issue OI-014.
>

### CLM-006 — Construction

> ##### Construction
>
> | Construct | Description | Status |
> |---|---|---|
> | Input run references | References to two analysis runs and their exact model-state/run basis. | Required by SOW-072/SOW-073; concrete schema field names `TBD`. |
> | Mapping source | Automatic mappings for unique identical result IDs with matching family/object/basis/dimension; explicit caller-supplied manual mappings for all other comparable relationships. | Implemented as a bounded DEL-14-05 `MappingRecord` consumer/producer projection; no heuristic workflow selected. |
> | Unit normalization | Same-dimension comparison of result values using accepted unit metadata and conversion contracts. | Required by SOW-073 and unit contract; accepted unit catalog/tolerance details `TBD`. |
> | Delta categories | Result, diagnostic, and settings deltas for mapped entities/locations. | Required by DEL-14-04 description; exact enum/API shape `TBD`. |
> | Tolerance profiles | Profile-driven thresholding for comparison outcomes. | Required by SOW-073; defaults/workflows `TBD` under OI-014. |
> | Deterministic output | Stable ordering and reproducible results for the same inputs. | Required by deterministic comparison scope; concrete hash/output fixture basis `TBD`. |
> | Tests | Result delta tests. | Anticipated artifact; acceptance fixture set `TBD`. |
>

### CLM-007 — References

> ##### References
>
> - `_CONTEXT.md` - deliverable identity, scope, architecture-basis injection, and context budget.
> - `_REFERENCES.md` - governing reference list and accessible public context.
> - `Dependencies.csv` - approved DAG-006 mirror/evidence surface for upstream dependency rows.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` - revision 0.7 package, deliverable, scope, objective, and open-issue basis.
> - `docs/CONTRACT.md` - invariant catalog, including no invention, unit-awareness, professional boundary, IP/privacy, and agent constraints.
> - `docs/SPEC.md` - unit contract, analysis boundary, persistence/hash, result export, runner output, and validation mechanics.
> - `docs/TYPES.md` - stable reference, diagnostic, checksum, result, result-export, and analysis-boundary vocabulary.
> - `docs/IP_AND_DATA_BOUNDARY.md` - protected-content and private-data boundary.
>

### CLM-008 — D-41 R5 T2B PDU-011/PDU-047 Evidence State

> ##### D-41 R5 T2B PDU-011/PDU-047 Evidence State
>
> | Gap | Current boundary | Disposition |
> |---|---|---|
> | Comparison output schema | No accepted schema for `AnalysisRunComparison.to_dict()`; mapping/tolerance schemas are input contracts. | Held; no schema or conformance test invented. |
> | Comparison mechanics validation | Explicit caller-supplied conversions and tolerance rules are verified for supported result families. | Independent engineering-validation/suitability basis remains held. |
>

### CLM-009 — D-41 R5 T2C PDU-030 Evidence State

> ##### D-41 R5 T2C PDU-030 Evidence State
>
> | Mapping path | Current boundary | Evidence |
> |---|---|---|
> | Automatic stable ID | Unique identical `result_id` plus matching family, object ref, basis ref, and dimension only. | Produced mapping validates against the DEL-14-05 `MappingRecord` definition and round trips into a delta with preserved mapping/left/right IDs. |
> | Manual | Required for different, duplicate, ambiguous, or semantically inconsistent IDs. | Caller-supplied manual mapping round trips into a delta with both result IDs preserved. |

## Completion and Reliance Basis — Epistemology

### CLM-010 — Specification: DEL-14-04 Analysis-run comparison engine

> #### Specification: DEL-14-04 Analysis-run comparison engine
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-011 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-14-04-DECL-001`.
>
> **Generated:** 2026-05-03
> **Document Role:** Normative draft
> **Source Basis:** `_CONTEXT.md`, `_REFERENCES.md`, `Dependencies.csv`, `execution/_Decomposition/SOFTWARE_DECOMP.md`, `docs/CONTRACT.md`, `docs/SPEC.md`, `docs/TYPES.md`, `docs/IP_AND_DATA_BOUNDARY.md`
>

### CLM-012 — Scope

> ##### Scope
>
> DEL-14-04 covers the backend feature slice for an analysis-run comparison engine. It compares two analysis runs, and where applicable their bound model states, using stable IDs, manual mappings where required, unit-normalized result deltas, diagnostics, settings, and tolerance profiles.
>
> DEL-14-04 excludes comprehensive commercial-prover result ingestion, external validation, professional approval, certification, sealing, authentication, automatic code-compliance determination, and protected standards-data content.
>

### CLM-013 — Requirements

> ##### Requirements
>
> | ReqID | Requirement | Source | Verification |
> |---|---|---|---|
> | R-14-04-001 | The engine shall compare analysis runs deterministically using stable IDs and manual mappings where required. Automatic matching is bounded to unique identical result IDs whose family, object ref, basis ref, and dimension also match; all different, duplicate, ambiguous, or semantically inconsistent IDs require explicit caller-supplied mapping. | SOW-073 in `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md`; DEL-14-05 mapping contract; PDU-030 | Focused tests validate the produced `MappingRecord`, JSON-round-trip automatic and manual mappings, deterministic deltas, and the no-heuristic boundary. |
> | R-14-04-002 | The engine shall treat analysis-run records as bound to exact model states, solver versions, settings, units, load cases, diagnostics, results, rule-pack references, library references, and result hashes. | SOW-072 in `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` | Tests assert comparison input metadata is preserved or surfaced; schema/API field names `TBD`. |
> | R-14-04-003 | The engine shall compute unit-normalized result deltas only where unit and dimension metadata allow valid comparison. | SOW-073; `docs/SPEC.md` unit contract | Unit-normalization tests; missing/ambiguous unit metadata produces diagnostics rather than silent defaults. |
> | R-14-04-004 | The engine shall include mapped nodes, elements, supports, terminals, stress/result locations, diagnostics, and settings within the comparison scope where the upstream result records expose them. | DEL-14-04 decomposition row | Coverage tests by result/entity category; exact category enum `TBD`. |
> | R-14-04-005 | The engine shall use tolerance profiles for comparison classification. | SOW-073; dependency row for DEL-14-05; OI-014 | Tolerance-profile tests after DEL-14-05 contracts are available; defaults and mapping workflows remain `TBD`. |
> | R-14-04-006 | The engine shall preserve professional-boundary language: comparison output is diagnostic/audit evidence and must not claim external validation or engineering acceptance. | SOW-073 notes; `docs/CONTRACT.md` OPS-K-AUTH-1/OPS-K-MECH-2; `docs/SPEC.md` analysis boundary | Protected wording/status tests; report/export assertions when applicable. |
> | R-14-04-007 | The engine shall remain compatible with schema-first result/export envelopes rather than creating an ad hoc comparison result surface. | `_CONTEXT.md` architecture basis; `docs/SPEC.md` result export and runner output text | Contract tests against accepted result/export schema once available; concrete schema path `TBD`. |
> | R-14-04-008 | The engine shall not introduce protected standards text, protected tables, proprietary values, private rule-pack payloads, or invented engineering defaults into public artifacts. | `docs/CONTRACT.md` OPS-K-IP-1/OPS-K-DATA-1/OPS-K-AGENT-1; `docs/IP_AND_DATA_BOUNDARY.md` | Protected-content/provenance checks; review of examples and fixtures. |
> | R-14-04-009 | The deliverable shall include result delta tests as an anticipated artifact. | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` | Test presence and execution through the applicable test gate; command `TBD`. |
>

### CLM-014 — Standards

> ##### Standards
>
> No external engineering standard text is locally present for this deliverable, and no clause-level engineering requirements are asserted here.
>
> Project-governing references for this draft are:
>
> - `docs/CONTRACT.md` for invariants.
> - `docs/SPEC.md` for unit, analysis-boundary, persistence/hash, result-export, runner-output, and validation mechanics.
> - `docs/TYPES.md` for vocabulary and boundary notes.
> - `docs/IP_AND_DATA_BOUNDARY.md` for protected-content/private-data limits.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` for accepted revision 0.7 scope and open issues.
>

### CLM-015 — Verification

> ##### Verification
>
> | Verification Item | Required Evidence | Current Status |
> |---|---|---|
> | Determinism | Same input pair produces identical ordered comparison output. | `TBD` fixture/API shape. |
> | Stable-ID/mapping behavior | Exact stable-ID production and manual-mapping paths are both exercised through JSON round trip. | Implemented for unique exact result IDs using the DEL-14-05 `MappingRecord`; non-identical/ambiguous IDs remain manual-only. |
> | Unit-normalized deltas | Same-dimension values compare after accepted unit normalization; incompatible/missing units emit diagnostics. | `TBD`; depends on DEL-02-02 unit contract maturity. |
> | Diagnostics/settings deltas | Diagnostic and settings differences are surfaced without compliance claims. | `TBD`; depends on result/run record schema. |
> | Tolerance profiles | Tolerance profile affects classification without changing raw delta evidence. | `TBD`; OI-014 open. |
> | Boundary wording | Output statuses avoid human approval, certification, sealing, authentication, and automatic code-compliance labels. | Required by governance; concrete assertions `TBD`. |
>

### CLM-016 — Documentation

> ##### Documentation
>
> The implementation brief or later production work should document:
>
> - comparison input contract;
> - mapping/tolerance profile dependency points;
> - delta output structure;
> - diagnostic/status behavior;
> - unit-normalization behavior and failure diagnostics;
> - deterministic ordering/hash basis;
> - result delta test fixture provenance;
> - limitations and professional-boundary notice.
>

### CLM-017 — D-41 R5 T2B PDU-011/PDU-047 Evidence Disposition (2026-07-12)

> ##### D-41 R5 T2B PDU-011/PDU-047 Evidence Disposition (2026-07-12)
>
> - PDU-011 remains held because no canonical analysis-run comparison-result/export schema exists in the current accepted schema set. `comparison_mapping.schema.json` and `comparison_tolerance.schema.json` govern inputs, not `AnalysisRunComparison.to_dict()` output. A positive/negative output-conformance test would therefore invent a contract.
> - PDU-047 remains held at the engineering-validation/suitability grain. The existing section-property oracle is not a validation basis for unit-normalized comparison mechanics, and `section_property` is not among the engine's current supported result families. No result family, conversion, tolerance, or outcome is added here.
>

### CLM-018 — D-41 R5 T2C PDU-030 Mapping Boundary (2026-07-12)

> ##### D-41 R5 T2C PDU-030 Mapping Boundary (2026-07-12)
>
> PDU-030 is completed at the current comparison grain: the engine now produces
> automatic DEL-14-05-shaped mappings only for unambiguous exact result IDs with
> matching identity metadata, while explicit manual mappings remain the only
> path for different or ambiguous IDs. Round-trip tests preserve mapping ID and
> left/right result identity into comparison deltas. This does not select
> heuristics, manual-review workflow policy, normalization thresholds,
> engineering outcomes, or a comparison-output schema.

- **AC-001** — The contract preserves exact run/model-state basis, automatic mapping only for unique identical IDs with matching identity metadata, manual-only handling for different or ambiguous IDs, raw evidence before tolerance interpretation, explicit unit/dimension diagnostics, schema-first envelope intent, output-schema and engineering-validation holds, source-labeled assumptions, public-safe fixture boundaries, and no external-validation or professional-approval claims.

## Production and Verification Method — Praxeology

### CLM-019 — Procedure: DEL-14-04 Analysis-run comparison engine

> #### Procedure: DEL-14-04 Analysis-run comparison engine
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-020 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-14-04-DECL-004`.
>
> **Generated:** 2026-05-03
> **Document Role:** Operational draft
> **Source Basis:** `_CONTEXT.md`, `_REFERENCES.md`, `Dependencies.csv`, `execution/_Decomposition/SOFTWARE_DECOMP.md`, `docs/CONTRACT.md`, `docs/SPEC.md`, `docs/TYPES.md`, `docs/IP_AND_DATA_BOUNDARY.md`
>

### CLM-021 — Purpose

> ##### Purpose
>
> Describe the conservative production procedure for creating and verifying the DEL-14-04 analysis-run comparison engine artifact without inventing implementation details or engineering values.
>

### CLM-022 — Prerequisites

> ##### Prerequisites
>
> - Read `_CONTEXT.md`, `_REFERENCES.md`, and the current deliverable-local dependency mirror before implementation work.
> - Confirm the applicable scope items are SOW-072 and SOW-073, and that OBJ-016 is the objective context.
> - Treat all existing approved DAG-006 rows in `Dependencies.csv` as ACTIVE evidence rows, not as rows to rewrite during this setup workflow.
> - Confirm upstream contract availability before implementing dependent behavior:
>   - DEL-14-02 analysis run records;
>   - DEL-14-05 comparison mapping, tolerance, and export contracts;
>   - DEL-08-04 result export format;
>   - DEL-02-02 unit system and dimensional-analysis core contract;
>   - architecture-basis rows AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, and AB-00-08.
> - Keep unsupported implementation choices as `TBD` until accepted by the appropriate upstream deliverable or human decision.
>

### CLM-023 — Steps

> ##### Steps
>
> 1. Establish the comparison input pair from exact analysis-run records.
>    - Required evidence includes exact model-state basis, solver version, settings, units, load cases, diagnostics, results, rule-pack references, library references, and result hashes.
>    - Concrete schema/API field names are `TBD`.
>
> 2. Resolve comparison mapping.
>    - Produce `automatic_match` only when a result ID is unique on each side and the ID, family, object ref, basis ref, and dimension all match.
>    - Use explicit caller-supplied `manual_match` records for different or ambiguous IDs as required by SOW-073.
>    - Validate the produced automatic record against the DEL-14-05 mapping contract; do not add heuristic matching or manual-review workflow semantics.
>
> 3. Validate unit comparability before computing deltas.
>    - Compare only same-dimension unit-bearing values after accepted normalization.
>    - Emit diagnostics for missing, ambiguous, or unsupported unit metadata; do not silently supply dimensionless or default units.
>
> 4. Compute result deltas for mapped comparison subjects.
>    - Include mapped nodes, elements, supports, terminals, stress/result locations, diagnostics, and settings where represented by the accepted upstream result/run records.
>    - Preserve raw delta evidence before tolerance-based classification.
>
> 5. Apply tolerance profiles.
>    - Use accepted tolerance profile contracts.
>    - Defaults and mapping workflows remain `TBD` under OI-014.
>
> 6. Produce deterministic comparison output.
>    - Use stable ordering and schema-first envelopes consistent with the accepted architecture basis.
>    - Preserve diagnostics, provenance, units, hashes/status boundaries, and professional-boundary notices.
>
> 7. Create result delta tests.
>    - Cover deterministic repeatability, stable-ID matching, manual mapping, unit-normalized deltas, missing-unit diagnostics, settings deltas, diagnostic deltas, and tolerance classification.
>    - Use invented/public/permissive fixtures only; fixture set and command are `TBD`.
>

### CLM-024 — Verification

> ##### Verification
>
> | Check | Evidence |
> |---|---|
> | Inputs preserve exact run basis | Test or contract assertion that run metadata is available to comparison logic. |
> | Determinism | Repeated comparison of identical input pair yields identical ordered output. |
> | Mapping behavior | DEL-14-05-shaped exact stable-ID mapping and caller-supplied manual mapping both survive JSON round trip with mapping/left/right result IDs preserved; ambiguous IDs do not auto-map. |
> | Unit normalization | Same-dimension conversion path and missing/incompatible-unit diagnostic path both exercised. |
> | Scope coverage | Mapped result locations, diagnostics, and settings are represented where upstream records expose them. |
> | Tolerance behavior | Raw deltas remain available while tolerance classification changes with profile. |
> | Governance boundary | Output avoids professional approval, certification, sealing, authentication, and automatic code-compliance labels. |
> | Protected-content boundary | Fixtures and examples contain no protected standards text/tables, proprietary values, private project data, or private rule-pack payloads. |
>

### CLM-025 — Records

> ##### Records
>
> - Run comparison engine implementation artifact (`TBD` path).
> - Result delta tests (`TBD` path/command).
> - Fixture provenance notes for any public examples.
> - Diagnostics/status assertions.
> - Mapping/tolerance contract references.
> - Any unresolved implementation decisions carried forward as `TBD`.
>

### CLM-026 — D-41 R5 T2B PDU-011/PDU-047 Check

> ##### D-41 R5 T2B PDU-011/PDU-047 Check
>
> Do not claim comparison-output schema conformance until an accepted output schema exists; input mapping/tolerance schema validation is not a substitute. Do not route section-property evidence through the comparison engine or choose conversion/tolerance behavior in this evidence-only tranche. Preserve both gaps explicitly.
>

### CLM-027 — D-41 R5 T2C PDU-030 Check

> ##### D-41 R5 T2C PDU-030 Check
>
> Run the focused exact-ID and manual-mapping round-trip tests. Treat the
> automatic producer as a bounded DEL-14-05 mapping-record path, not as authority
> to infer entities, create heuristic matches, select mapping workflow policy,
> or claim output-schema conformance.

- **VER-001** — Validate the contract and review source parity, exact-ID and manual mapping paths, all named comparison categories, deterministic ordering, raw-delta/tolerance separation, unit-normalization diagnostics, retained PDU-011/PDU-047 and OI-014 holds, schema and fixture boundaries, and professional-authority limits.

## Governing Values and Decisions — Axiology

### CLM-028 — Guidance: DEL-14-04 Analysis-run comparison engine

> #### Guidance: DEL-14-04 Analysis-run comparison engine
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-029 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-14-04-DECL-003`.
>
> **Generated:** 2026-05-03
> **Document Role:** Directional draft
> **Source Basis:** `_CONTEXT.md`, `_REFERENCES.md`, `Dependencies.csv`, `execution/_Decomposition/SOFTWARE_DECOMP.md`, `docs/CONTRACT.md`, `docs/SPEC.md`, `docs/TYPES.md`, `docs/IP_AND_DATA_BOUNDARY.md`
>

### CLM-030 — Purpose

> ##### Purpose
>
> DEL-14-04 exists to make analysis-run comparisons deterministic, unit-aware, traceable, and suitable for diagnostic/audit review. Its output supports design iteration and review; it does not decide external validation or professional reliance.
>

### CLM-031 — Principles

> ##### Principles
>
> - Keep comparison evidence tied to exact run/model-state basis, solver version, settings, units, load cases, diagnostics, result hashes, and relevant rule/library references.
> - Produce automatic mappings only for unique identical result IDs whose family, object ref, basis ref, and dimension also match; use explicit manual mappings for every different or ambiguous identity relationship.
> - Treat unit-bearing values as comparable only when unit and dimension metadata support the comparison. Missing or ambiguous unit metadata is a diagnostic.
> - Preserve raw delta evidence separately from tolerance-based classification. `ASSUMPTION`: this separation is needed so tolerance profiles can change without rewriting source evidence; the exact output structure is `TBD`.
> - Keep diagnostics and settings in scope because the decomposition names them explicitly for DEL-14-04.
> - Preserve the professional boundary: comparison output is review evidence, not approval, certification, sealing, authentication, or code compliance.
>

### CLM-032 — Considerations

> ##### Considerations
>
> | Topic | Guidance |
> |---|---|
> | Mapping | DEL-14-05 owns mapping/tolerance/export contracts. DEL-14-04 produces the existing `automatic_match`/`stable_id_alignment` record shape only for exact unambiguous IDs and consumes explicit `manual_match` records otherwise; it does not invent heuristics. |
> | Tolerances | OI-014 records comparison tolerance defaults and mapping workflows as `TBD`; do not hard-code defaults without human/product approval. |
> | Units | Unit normalization depends on accepted unit metadata and conversion governance. Unsupported conversion semantics should remain `TBD` or become diagnostics. |
> | Result scope | Result export envelopes are the intended review/regression/report-consumption surface. Avoid ad hoc result shapes once the schema contract is available. |
> | Hashes | JSON payload hashes use the accepted JCS-compatible basis where JSON payloads are hashed. Non-JSON and binary partitioning remains `TBD` in the broader architecture. |
> | Protected content | Fixtures, examples, and report sections must not embed protected standards text, protected tables, proprietary values, private project data, or private rule-pack payloads. |
>

### CLM-033 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Direction |
> |---|---|
> | Early implementation vs. contract maturity | Keep unresolved mapping, tolerance, and result-schema choices as `TBD` until upstream contracts are accepted. |
> | Raw deltas vs. classified outcomes | Preserve raw evidence and expose tolerance-classified outcomes as derived interpretation. |
> | Diagnostic detail vs. professional-boundary risk | Include enough diagnostics for review and audit while avoiding compliance/approval language. |
> | Broad comparison scope vs. deliverable boundary | Cover mapped result entities, diagnostics, and settings; defer export semantics and mapping/tolerance contract definition to DEL-14-05. |
>

### CLM-034 — Examples

> ##### Examples
>
> Concrete examples are `TBD`. Future examples should use invented or otherwise permitted data, documented provenance, explicit units, and no protected standards text/tables or proprietary values.
>

### CLM-035 — Open Questions

> ##### Open Questions
>
> | Question ID | Question | Source |
> |---|---|---|
> | OQ-14-04-001 | What are the accepted default tolerance profiles and mapping workflows for comparison classification? | `execution/_Decomposition/SOFTWARE_DECOMP.md` OI-014 |
> | OQ-14-04-002 | What is the final comparison output schema/API shape? | `docs/SPEC.md` result export baseline; DEL-14-05 dependency |
> | OQ-14-04-003 | What fixture set proves deterministic ordering, unit-normalized delta behavior, and diagnostic/settings comparison without protected or private data? | `_CONTEXT.md` anticipated artifact; `docs/IP_AND_DATA_BOUNDARY.md` |
>

### CLM-036 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> No source conflicts were identified in the accessible source set. Open questions above are missing-decision gaps, not contradictions.
>

### CLM-037 — D-41 R5 T2B PDU-011/PDU-047 Boundary

> ##### D-41 R5 T2B PDU-011/PDU-047 Boundary
>
> Schema conformance requires an accepted output contract, not a test-authored shape. Likewise, a section-property oracle cannot validate comparison normalization or tolerance suitability. Keep current deterministic behavior, input-contract tests, and diagnostic boundaries separate from those unresolved validation claims.
>

### CLM-038 — D-41 R5 T2C PDU-030 Boundary

> ##### D-41 R5 T2C PDU-030 Boundary
>
> Stable-ID production is deliberately exact, not fuzzy: equality of a unique
> result ID plus matching family/object/basis/dimension is required. JSON
> round-trip evidence covers both that automatic record and caller-supplied
> manual mappings. No automatic entity inference, heuristic candidate,
> normalization policy, or acceptance judgment follows from this mapping step.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-073 SOW-072 OBJ-016 | CLM-010 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
