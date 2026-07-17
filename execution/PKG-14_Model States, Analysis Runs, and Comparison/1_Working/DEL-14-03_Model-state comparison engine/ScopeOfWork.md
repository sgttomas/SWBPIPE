---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-14-03
package_id: PKG-14
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@e8f59a63372f38d9e788ac39b39995558f5aba73
project_scope_refs: [SOW-073, SOW-071]
package_objective_refs: [OBJ-016]
---

# Scope of Work — DEL-14-03

## Purpose and Objective Traceability

This Scope of Work defines `DEL-14-03` in service of project scope [SOW-073, SOW-071] and package objectives [OBJ-016].

- **OUT-001** — A deterministic model-state comparison contract using stable IDs and explicit mappings to classify added, removed, changed, and unchanged entities is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-14-03 Model-state comparison engine

> #### Datasheet: DEL-14-03 Model-state comparison engine
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-14-03-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-14-03 |
> | Name | Model-state comparison engine |
> | Package ID | PKG-14 |
> | Package Name | Model States, Analysis Runs, and Comparison |
> | Type | BACKEND_FEATURE_SLICE |
> | Description | Implement deterministic state diffs using stable IDs and explicit mapping records. |
> | Anticipated artifacts | state comparison engine; state diff tests |
> | Scope coverage | SOW-073; SOW-071 |
> | Objective support | OBJ-016 |
> | Context envelope | L |
>
> Source: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` section "PKG-14 - Model States, Analysis Runs, and Comparison"; `docs/_Registers/Deliverables.csv` row `DEL-14-03`.
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Value | Source |
> |---|---|---|
> | Primary function | Compute deterministic added, removed, changed, and unchanged model-entity diffs between model states. | `_CONTEXT.md` Context Envelope; `execution/_Decomposition/SOFTWARE_DECOMP.md` row `DEL-14-03` |
> | Identity basis | Stable IDs are the primary matching surface. | `docs/_Registers/ScopeLedger.csv` row `SOW-073`; `docs/TYPES.md` `Reference` definition |
> | Mapping basis | Explicit mapping records are required where direct stable-ID comparison is insufficient. Mapping workflow details remain `TBD`. | `docs/_Registers/ScopeLedger.csv` row `SOW-073`; `execution/_Decomposition/SOFTWARE_DECOMP.md` open issue `OI-014` |
> | State basis | Model states are named immutable records with tags, notes, external references, unresolved assumptions, warnings, and deterministic hashes. | `docs/_Registers/ScopeLedger.csv` row `SOW-071` |
> | Authority boundary | Comparison output is diagnostic/audit functionality, not external validation or professional acceptance. | `docs/_Registers/ScopeLedger.csv` row `SOW-073`; `docs/CONTRACT.md` invariants `OPS-K-AUTH-1`, `OPS-K-MECH-2` |
> | Architecture basis | Rust core/application services, schema-first command/query/job result envelopes, JSON Schema 2020-12, and JCS-compatible hash basis where JSON payloads are hashed. | `_CONTEXT.md` Architecture Basis Injection; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.1 |
>

### CLM-005 — Conditions

> ##### Conditions
>
> | Condition | Status |
> |---|---|
> | Input state schema | Depends on `DEL-14-01` immutable model state records. |
> | Mapping and tolerance contracts | Depends on `DEL-14-05`; tolerance defaults and mapping workflows remain `TBD`. |
> | Unit-aware comparison of unit-bearing values | Depends on `DEL-02-02` unit system and dimensional-analysis core contract. |
> | Analysis-run result deltas | Out of this deliverable except where SOW-073 context informs state comparison; the analysis-run comparison engine is `DEL-14-04`. |
> | External validation/prover status | Excluded. The package does not ingest commercial prover outputs comprehensively or determine external validation. |
>
> Source: `Dependencies.csv` approved DAG-002 mirror rows `DAG-002-E0792`, `DAG-002-E0793`, `DAG-002-E0794`; `_CONTEXT.md` Package Exclusions; `execution/_Decomposition/SOFTWARE_DECOMP.md` rows `DEL-14-03`, `DEL-14-04`, `DEL-14-05`.
>

### CLM-006 — Construction

> ##### Construction
>
> | Construction surface | Conservative construction note |
> |---|---|
> | Backend slice | Implement as a backend/service feature aligned with the Rust core/application-service architecture basis. Exact module path is `TBD`. |
> | Input contract | Accept two immutable model-state references or payloads once `DEL-14-01` defines the state record contract. |
> | Matching contract | Prefer stable IDs. Require explicit mapping records for intentional correspondence where IDs differ. |
> | Diff contract | Report added, removed, changed, and unchanged model entities. Exact entity categories and payload normalization are `TBD` pending state schema detail. |
> | Hash/canonicalization | Where JSON payloads are hashed, use the accepted JCS-compatible canonical JSON basis. |
> | Diagnostics | Preserve warnings, unresolved assumptions, provenance, and professional-boundary status in result envelopes. |
> | Tests | Provide deterministic state diff tests covering stable-ID matches, mapped entities, and unmatched entities. Exact fixture model content is `TBD` and must avoid protected/private data. |
>

### CLM-007 — References

> ##### References
>
> - `_CONTEXT.md` - deliverable identity, scope, architecture-basis injection, and dependency envelope.
> - `_REFERENCES.md` - local reference index for this deliverable.
> - `Dependencies.csv` - approved DAG-006 local mirror/evidence surface.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` - accepted revision 0.7 current decomposition basis.
> - `docs/_Registers/Deliverables.csv` - deliverable row `DEL-14-03`.
> - `docs/_Registers/ScopeLedger.csv` - rows `SOW-071` and `SOW-073`.
> - `docs/CONTRACT.md` - invariants for IDs, units, professional authority, data, and agent behavior.
> - `docs/SPEC.md` - unit, persistence, analysis-boundary, reporting/result-envelope, and validation mechanics.
> - `docs/TYPES.md` - reference, traceability, checksum, diagnostic, result, and report boundary definitions.

### CLM-008 — D-41 R5 T2B unit evidence (2026-07-12)

> ##### D-41 R5 T2B unit evidence (2026-07-12)
>
> Focused positive/negative tests cover missing metadata, incompatible dimensions, different units without normalization, and same-unit/same-dimension structured changes. No numeric conversion or engineering validation is claimed.

## Completion and Reliance Basis — Epistemology

### CLM-009 — Specification: DEL-14-03 Model-state comparison engine

> #### Specification: DEL-14-03 Model-state comparison engine
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-010 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-14-03-DECL-001`.
>

### CLM-011 — Scope

> ##### Scope
>
> This deliverable specifies a backend feature slice for deterministic comparison of immutable model states. It covers the model-state side of SOW-073 and the SOW-071 state-record context needed by that comparison. It must report added, removed, changed, and unchanged model entities using stable IDs and explicit mapping records where required.
>
> This deliverable excludes the full analysis-run comparison engine (`DEL-14-04`), the mapping/tolerance/export contract definition (`DEL-14-05`), and the immutable state data model itself (`DEL-14-01`). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
>
> Sources: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` rows `DEL-14-01` through `DEL-14-05`; `docs/_Registers/ScopeLedger.csv` rows `SOW-071` and `SOW-073`.
>

### CLM-012 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source | Verification |
> |---|---|---|---|
> | REQ-14-03-001 | The engine shall compare two immutable model-state records or references once the `DEL-14-01` state contract is available. | `docs/_Registers/ScopeLedger.csv` row `SOW-071`; `Dependencies.csv` row `DAG-002-E0792` | Unit test with two known state fixtures; schema validation once state schema exists. |
> | REQ-14-03-002 | The engine shall use stable IDs as the primary entity matching basis. | `docs/_Registers/ScopeLedger.csv` row `SOW-073`; `docs/TYPES.md` `Reference` definition | Unit tests showing order-independent, stable-ID-based matching. |
> | REQ-14-03-003 | The engine shall require explicit mapping records where comparison cannot rely on direct stable-ID correspondence. | `docs/_Registers/ScopeLedger.csv` row `SOW-073`; `Dependencies.csv` row `DAG-002-E0793` | Unit tests for manually mapped entities after `DEL-14-05` defines mapping records. |
> | REQ-14-03-004 | The engine shall classify model entities as added, removed, changed, or unchanged. | `_CONTEXT.md` Context Envelope; `execution/_Decomposition/SOFTWARE_DECOMP.md` row `DEL-14-03` | Unit tests for each classification. |
> | REQ-14-03-005 | The same two input states, mappings, and comparison settings shall produce deterministic diff output. | `docs/_Registers/ScopeLedger.csv` row `SOW-073`; `docs/CONTRACT.md` `OPS-K-ID-1`; `docs/SPEC.md` section 4.4 | Repeat-run tests comparing serialized result envelopes or canonical hashes where available. |
> | REQ-14-03-006 | The comparison shall preserve relevant state metadata, including unresolved assumptions, warnings, notes, external references, and deterministic hashes when those fields are present in the state contract. | `docs/_Registers/ScopeLedger.csv` row `SOW-071`; `docs/SPEC.md` section 3 Domain objects | Tests confirming metadata is not discarded from comparison context. |
> | REQ-14-03-007 | Unit-bearing changed values shall not be compared as bare numbers without unit/dimension metadata or an explicit unit-normalization contract. | `docs/SPEC.md` section 4 Unit system and dimensional analysis; `Dependencies.csv` row `DAG-002-E0794` | Tests for unit metadata handling; `TBD` until `DEL-02-02` and `DEL-14-05` provide final contracts. |
> | REQ-14-03-008 | Comparison result envelopes shall carry diagnostics/provenance sufficient to expose missing mappings, unsupported comparison categories, unresolved assumptions, and professional-boundary limits. | `docs/SPEC.md` sections 4.3 and 9; `execution/_Decomposition/SOFTWARE_DECOMP.md` `AB-00-06` | Tests asserting diagnostic fields and no forbidden status labels. |
> | REQ-14-03-009 | Outputs shall not claim certification, sealing, authentication, professional approval, code compliance, or external validation (PRD §21.2). | `docs/CONTRACT.md` `OPS-K-AUTH-1`; `docs/DIRECTIVE.md` section 3; `docs/_Registers/ScopeLedger.csv` row `SOW-073` | Protected-claim review of result labels and report/export-facing strings. |
> | REQ-14-03-010 | State diff tests shall use invented, public-safe fixtures and must not include protected standards text, proprietary values, or private project data. | `docs/CONTRACT.md` `OPS-K-IP-1`, `OPS-K-RULE-1`; `docs/SPEC.md` section 10 | Fixture provenance review and protected-content gate. |
>

### CLM-013 — Standards

> ##### Standards
>
> No external engineering standard text is locally available or required for this deliverable. Governing project standards are the local OpenPipeStress documents listed in `_REFERENCES.md`, especially:
>
> - `docs/CONTRACT.md` for invariant boundaries.
> - `docs/SPEC.md` for unit, persistence, result-envelope, and validation expectations.
> - `docs/TYPES.md` for stable references, traceability links, checksums, diagnostics, and result/report boundary concepts.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 for scope partitioning and architecture-basis constraints.
>
> Any clause-level external standard requirement is `TBD` and must not be inferred.
>

### CLM-014 — Verification

> ##### Verification
>
> | Verification area | Minimum check |
> |---|---|
> | Stable-ID matching | Reorder equivalent entity lists and confirm unchanged diff output. |
> | Added/removed classification | Compare fixtures with entities present in only one state. |
> | Changed classification | Compare fixtures with the same stable ID and intentionally changed public-safe fields. |
> | Explicit mapping | Use mapping records from `DEL-14-05` once available; until then, mark detailed mapping behavior `TBD`. |
> | Unit-bearing value handling | Confirm missing or incompatible unit metadata produces diagnostics rather than silent numeric comparison. |
> | Determinism | Repeat the same comparison and compare canonical serialized output or result hash where implemented. |
> | Boundary language | Assert no output status or label implies professional acceptance, code compliance, certification, sealing, or external validation (PRD §21.2). |
>

### CLM-015 — Documentation

> ##### Documentation
>
> Required local evidence for this deliverable:
>
> - state comparison engine artifact or module path, `TBD` until implementation;
> - state diff tests;
> - notes identifying dependency on `DEL-14-01`, `DEL-14-05`, and `DEL-02-02`;
> - comparison result-envelope shape or service contract, `TBD` until implementation;
> - protected-content/provenance status for fixtures and examples.

### CLM-016 — D-41 R5 T2B unit-comparison boundary (2026-07-12)

> ##### D-41 R5 T2B unit-comparison boundary (2026-07-12)
>
> DEL-14-03 performs structural state comparison, not unit conversion. A changed field declared unit-bearing must carry explicit unit and dimension metadata on both sides. Missing metadata, incompatible dimensions, or different units without a governed normalization contract block classification as a normal change. Same-unit/same-dimension values are preserved as structured left/right evidence; the engine does not emit a bare numeric delta.

- **AC-001** — The contract preserves immutable-state context, stable-ID-first matching, explicit rather than heuristic mappings, deterministic output, metadata and diagnostic preservation, structured unit/dimension blocking behavior without bare numeric deltas, invented/public-safe fixtures, the retained dependency-enum conflict and review residuals, and the separation from run comparison, export policy, external validation, or professional approval.

## Production and Verification Method — Praxeology

### CLM-017 — Procedure: DEL-14-03 Model-state comparison engine

> #### Procedure: DEL-14-03 Model-state comparison engine
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-018 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-14-03-DECL-004`.
>

### CLM-019 — Purpose

> ##### Purpose
>
> Produce or use the model-state comparison engine in a way that remains deterministic, source-grounded, unit-aware where applicable, and bounded to diagnostic/audit comparison of immutable model states.
>

### CLM-020 — Prerequisites

> ##### Prerequisites
>
> | Prerequisite | Status / source |
> |---|---|
> | Immutable model-state record contract | Upstream dependency `DEL-14-01`; approved DAG-002 mirror row `DAG-002-E0792`. |
> | Mapping/tolerance contract | Upstream dependency `DEL-14-05`; approved DAG-002 mirror row `DAG-002-E0793`; details `TBD` per `OI-014`. |
> | Unit system and dimensional-analysis contract | Upstream dependency `DEL-02-02`; approved DAG-002 mirror row `DAG-002-E0794`. |
> | Architecture basis | `AB-00-01`, `AB-00-02`, `AB-00-03`, `AB-00-04`, `AB-00-06`, `AB-00-07`, and `AB-00-08` in `_CONTEXT.md`. |
> | Professional/IP boundary | `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, and `docs/IP_AND_DATA_BOUNDARY.md`. |
>

### CLM-021 — Steps

> ##### Steps
>
> 1. Confirm the work is limited to `DEL-14-03` model-state entity diffs. Escalate if the task expands into analysis-run result deltas, export formats, GUI overlays, external prover status, or professional approval workflow.
>
> 2. Identify the immutable state input contract from `DEL-14-01`. If the contract is unavailable, record the input schema as `TBD` and do not invent entity fields.
>
> 3. Identify the comparison mapping contract from `DEL-14-05`. If unavailable, implement only stable-ID matching assumptions that are source-supported, and mark manual mapping behavior `TBD`.
>
> 4. Identify unit-bearing fields through the unit contract from `DEL-02-02`. Do not compare unit-bearing values as bare numbers; missing or incompatible unit metadata should produce diagnostics.
>
> 5. Define the comparison input envelope as two model-state references or payloads plus any explicit mapping/settings record. Exact service/API syntax is `TBD`.
>
> 6. Match entities by stable ID. Apply explicit mapping records where the mapping contract allows them. Treat unresolved correspondence as unmatched or diagnostic rather than hidden equivalence.
>
> 7. Classify entities into added, removed, changed, and unchanged categories. Exact field-level normalization and change significance rules are `TBD` pending state schema and mapping/tolerance contract detail.
>
> 8. Preserve review context: state hashes, warnings, unresolved assumptions, notes, external references, provenance, and diagnostics where present.
>
> 9. Return a deterministic result envelope with professional-boundary language. Do not emit automatic certification, sealing, code-compliance, external-validation, or human-approval statuses (PRD §21.2).
>
> 10. Build state diff tests using invented/public-safe fixtures. Cover stable-ID matching, added/removed/changed/unchanged classifications, mapping behavior once available, deterministic repeatability, unit metadata diagnostics, and forbidden-claim checks.
>

### CLM-022 — Verification

> ##### Verification
>
> | Check | Expected evidence |
> |---|---|
> | Stable-ID determinism | Repeated comparisons of the same fixtures produce identical classifications and result serialization where implemented. |
> | Mapping behavior | Explicit mappings produce expected correspondence; missing mappings remain visible. `TBD` until `DEL-14-05` exists. |
> | Changed classification | Source-supported relevant field changes are reported as changed. Exact normalization policy is `TBD`. |
> | Unit metadata | Unit-bearing fields require unit/dimension context or produce diagnostics. |
> | Metadata preservation | Warnings, assumptions, external references, notes, provenance, and hashes are not silently dropped. |
> | Boundary language | No result label claims professional acceptance, code compliance, certification, sealing, authentication, or external validation (PRD §21.2). |
> | Fixture governance | Test fixtures are invented/public-safe and do not include protected standards text, proprietary values, or private project data. |
>

### CLM-023 — Records

> ##### Records
>
> - Implementation/module path: `TBD`.
> - State comparison service or engine contract: `TBD`.
> - State diff test artifact path: `TBD`.
> - Fixture provenance notes: `TBD`.
> - Dependency notes: preserve the existing DAG-002 mirror rows as ACTIVE unless a future approved coordination workflow changes the mirror.

### CLM-024 — D-41 R5 T2B unit check (2026-07-12)

> ##### D-41 R5 T2B unit check (2026-07-12)
>
> For every changed field listed in `unit_bearing_fields`, require unit and dimension on both sides; block missing metadata, dimension mismatch, and unnormalized unit mismatch; confirm accepted same-unit changes retain full left/right records and contain no derived bare-number delta.

- **VER-001** — Validate the contract and review source parity, stable-ID and mapping behavior, all four classifications, deterministic repeatability, state metadata preservation, missing/incompatible/unnormalized unit diagnostics, retained conflict and TBDs, fixture governance, deliverable boundaries, and prohibited authority claims.

## Governing Values and Decisions — Axiology

### CLM-025 — Guidance: DEL-14-03 Model-state comparison engine

> #### Guidance: DEL-14-03 Model-state comparison engine
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-026 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-14-03-DECL-003`.
>

### CLM-027 — Purpose

> ##### Purpose
>
> This deliverable exists to make immutable model states reviewable through deterministic comparison. The comparison should support design iteration by identifying added, removed, changed, and unchanged model entities without converting that comparison into an external validation or professional approval state.
>
> Sources: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` row `DEL-14-03`; `docs/_Registers/ScopeLedger.csv` rows `SOW-071` and `SOW-073`.
>

### CLM-028 — Principles

> ##### Principles
>
> | Principle | Guidance |
> |---|---|
> | Stable identity first | Treat stable IDs and typed references as the primary comparison surface. Positional or display-name matching is an implementation risk unless explicitly justified by a mapping contract. |
> | Explicit mappings | When two states intentionally refer to corresponding entities with different IDs, use explicit mapping records. Do not infer hidden engineering equivalence from proximity, naming, or apparent similarity without a source-backed rule. |
> | Deterministic output | The same inputs should produce the same classifications and serialized result shape. Hashing or canonical serialization details are governed by the persistence/hash basis where JSON payloads are involved. |
> | No silent defaults | Missing mappings, missing units, unsupported entity categories, and unresolved assumptions should become diagnostics or `TBD`, not implicit acceptance. |
> | Metadata preservation | Notes, warnings, external references, unresolved assumptions, provenance, and hashes attached to model states are part of the review context. |
> | Boundary clarity | The comparison is diagnostic/audit functionality. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). |
>
> Sources: `docs/DIRECTIVE.md` sections 2.2 and 3; `docs/CONTRACT.md` invariants `OPS-K-DATA-2`, `OPS-K-AUTH-1`, `OPS-K-MECH-2`; `docs/SPEC.md` sections 4, 4.3, and 4.4.
>

### CLM-029 — Considerations

> ##### Considerations
>
> | Topic | Consideration |
> |---|---|
> | Model-state versus analysis-run comparison | `DEL-14-03` is scoped to model-state diffs. Analysis-run result deltas are separately assigned to `DEL-14-04`; mapping/tolerance/export contracts are assigned to `DEL-14-05`. |
> | Tolerances | Comparison tolerance defaults and mapping workflows are an open issue (`OI-014`). Treat detailed tolerance behavior as `TBD` until resolved. |
> | Unit-normalized values | SOW-073 mentions unit-normalized result deltas, but the unit contract dependency (`DEL-02-02`) and comparison contract dependency (`DEL-14-05`) must be honored before numeric comparison policy is finalized. |
> | Entity granularity | The exact model entity categories are `TBD` until the immutable state schema and canonical domain model detail are available. |
> | Fixture content | Tests should use invented/public-safe fixtures and should carry provenance status. Do not import protected standards examples or proprietary project data. |
> | Context size | The deliverable has an `L` context envelope and WATCH risk; scope should be split or escalated if it expands beyond deterministic model-state entity diffs. |
>

### CLM-030 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Direction |
> |---|---|
> | Strict stable-ID matching vs. heuristic matching | Prefer strict stable-ID matching. Use explicit mapping records for intentional correspondence. Heuristics are `TBD` and require human-approved scope if introduced. |
> | Rich diff payload vs. bounded backend slice | Start with source-supported classifications and diagnostic metadata. Defer UI overlays, report formatting, and export semantics to their owning deliverables. |
> | Canonical hash comparison vs. field-level comparison | Hashes support reproducibility, but field-level changed classification needs a source-defined normalization policy. The exact field comparison policy remains `TBD`. |
> | Numeric tolerance vs. exact equality | Numeric tolerance policy is not yet source-defined for this deliverable. Unit and tolerance dependencies must be resolved before implementation claims are made. |
>

### CLM-031 — Examples

> ##### Examples
>
> Source-grounded example categories are limited to:
>
> - an entity appears only in the later state: added;
> - an entity appears only in the earlier state: removed;
> - an entity appears in both states with the same stable ID or explicit mapping and no relevant difference: unchanged;
> - an entity appears in both states with the same stable ID or explicit mapping and a source-supported relevant difference: changed.
>
> Concrete fixture entities, fields, values, units, tolerances, and mapping records are `TBD` until the state schema and mapping/tolerance contract are available.
>

### CLM-032 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | C-14-03-001 | Dependency-extract v3.1 write-form enums would normalize/reclassify several approved DAG-006 mirror values, but the project task rule says to preserve approved DAG-006 rows as ACTIVE without reclassification. | `skills/dependency-extract/SKILL.md` "Canonical enums"; `Dependencies.csv` approved mirror rows | User task instruction for `DEL-14-03` dependency handling | Dependency register handling; final report | Preserve the approved mirror unchanged and record the conflict. | TBD |

### CLM-033 — D-41 R5 T2B unit guidance (2026-07-12)

> ##### D-41 R5 T2B unit guidance (2026-07-12)
>
> Do not compare changed unit-bearing fields as bare numbers. Normalize only through a separately governed contract; otherwise preserve the structured quantities and emit a blocking unit/dimension diagnostic.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-073 SOW-071 OBJ-016 | CLM-009 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
