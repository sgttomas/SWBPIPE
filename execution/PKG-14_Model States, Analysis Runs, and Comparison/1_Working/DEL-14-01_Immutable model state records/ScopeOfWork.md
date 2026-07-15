---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-14-01
package_id: PKG-14
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@e8f59a63372f38d9e788ac39b39995558f5aba73
project_scope_refs: [SOW-071]
package_objective_refs: [OBJ-016]
---

# Scope of Work — DEL-14-01

## Purpose and Objective Traceability

This Scope of Work defines `DEL-14-01` in service of project scope [SOW-071] and package objectives [OBJ-016].

- **OUT-001** — An immutable model-state record contract for named, read-only snapshots with flexible review metadata, explicit payload-scoped hashes, and reproducible persistence is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-14-01 Immutable model state records

> #### Datasheet: DEL-14-01 Immutable model state records
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-14-01-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value | Source |
> |---|---|---|
> | Deliverable ID | DEL-14-01 | `_CONTEXT.md` |
> | Name | Immutable model state records | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 7 |
> | Package | PKG-14 Model States, Analysis Runs, and Comparison | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 6 |
> | Type | DATA_MODEL_CHANGE | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-14-01 |
> | Scope item | SOW-071 | `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv` row SOW-071 |
> | Objective | OBJ-016 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 5 |
> | Context envelope | M | `_CONTEXT.md`; `docs/_Registers/ContextBudgetQA.csv` row DEL-14-01 |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Current value | Source / status |
> |---|---|---|
> | Record purpose | Save named immutable model states for design iteration and review. | SOW-071 and OBJ-016 in `execution/_Decomposition/SOFTWARE_DECOMP.md` |
> | Required metadata categories | Names, tags, notes, external references, unresolved assumptions, warnings, and deterministic hashes. | SOW-071 in `_CONTEXT.md`, `docs/_Registers/ScopeLedger.csv`, and decomposition section 9 |
> | Snapshot semantics | Read-only snapshot semantics are in scope. | Decomposition section 7, DEL-14-01 sizing notes |
> | Formal prover approval statuses | Excluded; model states are flexible metadata records, not formal prover approval states. | `_CONTEXT.md` Context Envelope; `docs/_Registers/ContextBudgetQA.csv` row DEL-14-01; SOW-071 notes |
> | Anticipated schema artifact | `schemas/model_state.schema.json` | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-14-01 |
> | Anticipated test artifact | model state persistence tests | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-14-01 |
> | Hash basis | JSON payload hashes use the accepted canonical JSON/JCS-compatible basis where the payload is JSON. | `_CONTEXT.md` Architecture Basis Injection; `docs/SPEC.md` section 4.4 |
> | Exact hash algorithm and library | TBD; only the JCS-compatible basis is currently source-supported for JSON payloads. | `_CONTEXT.md` Still TBD; `docs/SPEC.md` section 4.4 |
>

### CLM-005 — Conditions

> ##### Conditions
>
> - The deliverable belongs to a schema-first architecture baseline: Rust core/application services, JSON Schema 2020-12 contracts, schema-first command/query/job result envelopes, and JCS-compatible canonical JSON hash basis where JSON payloads are hashed (`_CONTEXT.md`, Architecture Basis Injection).
> - The model state record must respect OpenPipeStress data-boundary invariants: protected standards text, copied tables, proprietary values, and private project/rule data must not be committed publicly by default (`docs/CONTRACT.md` OPS-K-IP-1, OPS-K-PRIV-1; `docs/IP_AND_DATA_BOUNDARY.md` sections 3 and 6).
> - Missing or unsupported data remains explicit and visible; silent engineering defaults are disallowed (`docs/CONTRACT.md` OPS-K-DATA-2; `docs/DIRECTIVE.md` section 3).
> - Human acceptance or external approval, if referenced later, remains external and hash-bound; the software must not emit professional approval, certification, sealing, authentication, or code-compliance equivalents as automatic statuses (`docs/SPEC.md` sections 4.4 and 9; `docs/CONTRACT.md` OPS-K-AUTH-1 and OPS-K-AUTH-2).
> - Approved DAG-002 mirror rows identify architecture-basis predecessors plus dependencies on canonical model, persistence, audit/hash, and analysis-status vocabulary surfaces (`Dependencies.csv`; `execution/_DAG/DAG-006/APPROVAL_RECORD.md`).
>

### CLM-006 — Construction

> ##### Construction
>
> The source-supported construction target is a schema and persistence-test surface for immutable model state records. The following construction fields are known at setup time:
>
> | Construction item | Status |
> |---|---|
> | Stable deliverable identity | Known: DEL-14-01 |
> | Parent package identity | Known: PKG-14 |
> | Schema filename | Known: `schemas/model_state.schema.json` |
> | Test obligation | Known: model state persistence tests |
> | Minimum record categories | Known: name, tags, notes, external references, unresolved assumptions, warnings, deterministic hashes |
> | Snapshot immutability rule | Required concept; exact enforcement mechanism TBD |
> | Canonical payload scope | TBD: model payload, state envelope, or explicit hash partitioning must be selected later |
> | Schema property names and required/optional cardinality | TBD; no authoritative schema text exists in the accessible source set |
> | Persistence module path and service API | TBD; exact package/module layout remains implementation-level detail |
>

### CLM-007 — References

> ##### References
>
> - `_CONTEXT.md` for deliverable identity, package, artifacts, scope, objective, architecture basis, and envelope notes.
> - `_REFERENCES.md` for the approved source list and authority boundary.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 for SOW-071, OBJ-016, PKG-14, and DEL-14-01 placement.
> - `docs/_Registers/Deliverables.csv`, `ScopeLedger.csv`, and `ContextBudgetQA.csv` for register-backed identity and scope data.
> - `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, `docs/SPEC.md`, `docs/TYPES.md`, and `docs/IP_AND_DATA_BOUNDARY.md` for invariants, technical boundaries, vocabulary, and data-boundary constraints.
> - `Dependencies.csv` as the local approved DAG-006 mirror/evidence surface.

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-14-01 Immutable model state records

> #### Specification: DEL-14-01 Immutable model state records
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-009 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-14-01-DECL-001`.
>

### CLM-010 — Scope

> ##### Scope
>
> This deliverable defines the data-model contract for immutable model state records and the persistence tests needed to verify that saved states remain reproducible and hash-addressable.
>
> In scope:
>
> - A JSON-schema-backed model state record surface for `schemas/model_state.schema.json`.
> - Record support for names, tags, notes, external references, unresolved assumptions, warnings, and deterministic hashes.
> - Read-only snapshot semantics for saved model states.
> - Persistence tests that demonstrate state save/load behavior and deterministic hash handling within the accepted architecture basis.
>
> Out of scope:
>
> - Formal prover approval statuses or automatic professional acceptance states.
> - Comprehensive commercial prover output ingestion.
> - Public bundling of protected standards text, code tables, proprietary values, or private owner/project data.
> - Exact dependency versions, physical project package/container format, hash library selection, and package-specific implementation choices unless approved in a later sealed implementation task.
>
> Sources: `_CONTEXT.md`; SOW-071, OBJ-016, and PKG-14 in `execution/_Decomposition/SOFTWARE_DECOMP.md`; `docs/SPEC.md` sections 4.4 and 9.
>

### CLM-011 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source |
> |---|---|---|
> | DEL-14-01-R1 | The product shall save named immutable model states. | SOW-071 in `_CONTEXT.md`, `docs/_Registers/ScopeLedger.csv`, and decomposition section 9 |
> | DEL-14-01-R2 | A model state record shall carry tags, notes, external references, unresolved assumptions, warnings, and deterministic hashes. | SOW-071 in `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-14-01 |
> | DEL-14-01-R3 | Saved model states shall use read-only snapshot semantics. Exact enforcement mechanism is TBD. | Decomposition section 7 DEL-14-01 sizing notes |
> | DEL-14-01-R4 | JSON payload hashes shall follow the accepted JCS-compatible canonical JSON basis where the payload is JSON. | `_CONTEXT.md` Architecture Basis Injection; `docs/SPEC.md` section 4.4 |
> | DEL-14-01-R5 | Hash records shall identify payload scope; exact state hash partitioning is TBD. | `docs/SPEC.md` section 4.4 |
> | DEL-14-01-R6 | Round-trip persistence must not insert silent engineering defaults for units, provenance, rule-pack values, material data, component data, SIF/flexibility inputs, allowables, or load-basis values. | `docs/SPEC.md` section 4.4 |
> | DEL-14-01-R7 | Model state records must preserve unresolved assumptions and warnings as visible review evidence, not hidden defaults. | `docs/DIRECTIVE.md` sections 2.2 and 3; `docs/TYPES.md` entries `Assumption` and `Diagnostic` |
> | DEL-14-01-R8 | The record surface must not create software-generated professional approval, certification, sealing, authentication, or code-compliance statuses. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/SPEC.md` section 9 |
> | DEL-14-01-R9 | External references must remain explicit and must not bypass schema validation, privacy controls, protected-content screening, or professional-boundary checks. | `docs/TYPES.md` entry `Reference`; `docs/SPEC.md` sections 4.4 and 4.5 |
> | DEL-14-01-R10 | The public repository must not embed protected standards content, proprietary values, or private project/rule data in model state examples or fixtures. | `docs/CONTRACT.md` OPS-K-IP-1 and OPS-K-PRIV-1; `docs/IP_AND_DATA_BOUNDARY.md` sections 3 and 6 |
>

### CLM-012 — Standards

> ##### Standards
>
> | Standard / basis | Applicability | Status |
> |---|---|---|
> | JSON Schema 2020-12 | Public schema/interchange baseline for this data-model change. | Source-supported by `_CONTEXT.md`; exact schema file content TBD |
> | Canonical JSON / JCS-compatible basis | Applies to JSON payload hashing where the state payload is JSON. | Source-supported by `_CONTEXT.md` and `docs/SPEC.md` section 4.4; exact library/algorithm binding TBD |
> | OpenPipeStress governance invariants | Data-boundary, provenance, no-silent-default, and no-professional-approval constraints. | Source-supported by `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, and `docs/IP_AND_DATA_BOUNDARY.md` |
>
> No external code clause text or protected standards text is available or required for this deliverable setup.
>

### CLM-013 — Verification

> ##### Verification
>
> | Requirement(s) | Verification approach |
> |---|---|
> | R1-R3 | Schema validation and persistence tests should prove that saved records retain identity, metadata, and read-only snapshot semantics. Exact test names TBD. |
> | R4-R5 | Hash determinism tests should compare canonicalized JSON payload bytes and verify payload-scope metadata. Exact canonicalization implementation TBD. |
> | R6-R7 | Round-trip tests should confirm that assumptions, warnings, provenance-bearing references, and other review evidence survive save/load without silent defaults. |
> | R8 | Schema and service tests should reject or omit automatic professional approval/code-compliance status fields. |
> | R9-R10 | Protected-content/private-data review should verify examples and fixtures contain only invented or cleared public data and explicit external-reference metadata. |
>

### CLM-014 — Documentation

> ##### Documentation
>
> Required deliverable records:
>
> - `schemas/model_state.schema.json`
> - model state persistence tests
> - test evidence or validation output from the eventual implementation task
> - any unresolved `TBD` decisions for hash scope, canonicalization implementation, schema property names, and persistence service entry points

- **AC-001** — The contract preserves names, tags, notes, external references, unresolved assumptions, warnings, deterministic hash metadata, JSON Schema and canonical-JSON boundaries, no-silent-default behavior, public/private data controls, formal-prover-status exclusions, and visible schema, hash-partition, persistence, and runtime-screening TBDs without implying lifecycle closure, external validation, professional approval, or code compliance.

## Production and Verification Method — Praxeology

### CLM-015 — Procedure: DEL-14-01 Immutable model state records

> #### Procedure: DEL-14-01 Immutable model state records
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-016 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-14-01-DECL-004`.
>

### CLM-017 — Purpose

> ##### Purpose
>
> Define and verify the immutable model state record surface for DEL-14-01 without expanding into analysis-run records, comparison engines, handoff workflows, or professional approval states.
>

### CLM-018 — Prerequisites

> ##### Prerequisites
>
> - Read `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, and `Dependencies.csv` in this deliverable folder.
> - Use `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 for SOW-071, OBJ-016, PKG-14, and DEL-14-01 scope.
> - Apply `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, `docs/SPEC.md`, `docs/TYPES.md`, and `docs/IP_AND_DATA_BOUNDARY.md` for governance, technical, vocabulary, and data-boundary constraints.
> - Treat approved DAG-006 rows as coordination evidence, not as authorization to edit upstream deliverables or dispatch Type 2 implementation.
> - Preserve all unknown implementation choices as `TBD` unless a later accepted source resolves them.
>
> Declared upstream coordination evidence from the local mirror:
>
> | Target | Meaning for this deliverable |
> |---|---|
> | DEL-00-01 through DEL-00-04, DEL-00-06 through DEL-00-08 | Architecture basis context for downstream work. |
> | DEL-02-01 | Canonical model schema predecessor; model states snapshot the canonical model. |
> | DEL-02-05 | Persistence and serialization predecessor. |
> | DEL-08-02 | Audit manifest and model hash predecessor. |
> | DEL-05-04 | Analysis-state/status vocabulary predecessor. |
>

### CLM-019 — Steps

> ##### Steps
>
> 1. Confirm the implementation task is scoped only to DEL-14-01 and `schemas/model_state.schema.json` plus persistence tests.
> 2. Define the model state schema fields only from source-supported categories: name, tags, notes, external references, unresolved assumptions, warnings, deterministic hash metadata, snapshot identity, and source/payload references.
> 3. Mark exact schema property names, required/optional cardinality, hash partitioning, hash library, and persistence API entry points as `TBD` until implementation evidence exists.
> 4. Apply JSON Schema 2020-12 as the schema/interchange baseline.
> 5. Apply the accepted JCS-compatible canonical JSON basis for JSON payload hashes and record payload scope explicitly.
> 6. Add or update persistence tests that demonstrate save/load behavior, stable metadata, read-only snapshot behavior, deterministic hash metadata, and no silent default insertion.
> 7. Add checks or fixtures only with invented or cleared public data; do not include protected standards text, proprietary vendor data, owner standards, private project values, or code-specific values.
> 8. Ensure no automatic professional approval, certification, sealing, authentication, external-prover approval, or code-compliance status is introduced.
> 9. Record all unresolved implementation decisions and unsupported source assumptions in deliverable notes or test evidence.
>

### CLM-020 — Verification

> ##### Verification
>
> - Schema validation passes for the model state schema once implemented.
> - Persistence tests pass and demonstrate stable round-trip behavior.
> - Hash tests demonstrate deterministic behavior for JSON payloads under the selected canonicalization method.
> - Tests or schema checks preserve assumptions, warnings, external references, and provenance-bearing metadata.
> - Protected-content/private-data review finds no bundled protected standards data, proprietary values, or private project/rule data in public fixtures.
> - Review confirms the deliverable does not claim professional approval or external validation.
>

### CLM-021 — Records

> ##### Records
>
> - `schemas/model_state.schema.json`
> - model state persistence test files and outputs
> - hash determinism test evidence
> - protected-content/private-data review evidence for fixtures
> - unresolved `TBD` decision list for implementation choices not source-supported in this setup pass

- **VER-001** — Validate the contract and review source parity, immutable snapshot and metadata coverage, payload-scope and deterministic-hash boundaries, round-trip/no-silent-default intent, external-reference privacy screening residuals, public-fixture controls, retained TBDs, and professional-authority limits.

## Governing Values and Decisions — Axiology

### CLM-022 — Guidance: DEL-14-01 Immutable model state records

> #### Guidance: DEL-14-01 Immutable model state records
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-023 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-14-01-DECL-003`.
>

### CLM-024 — Purpose

> ##### Purpose
>
> Immutable model state records give OpenPipeStress a reproducible saved-state surface for design iteration, comparison, reporting, and handoff workflows. The deliverable is a schema/data-model change, not a professional approval workflow.
>
> Sources: `_CONTEXT.md`; OBJ-016 and PKG-14 in `execution/_Decomposition/SOFTWARE_DECOMP.md`; `docs/SPEC.md` sections 4.4 and 9.
>

### CLM-025 — Principles

> ##### Principles
>
> - Treat the model state as a saved snapshot record, not as a mutable "current model" alias.
> - Keep flexible review metadata such as names, tags, notes, external references, unresolved assumptions, and warnings explicit.
> - Bind reproducibility to deterministic hashes with explicit payload scope; do not imply that a hash proves engineering correctness.
> - Preserve the separation between mechanics/software evidence and human professional reliance.
> - Keep protected standards data, owner standards, proprietary vendor data, and private project data out of public examples and fixtures unless separately reviewed and cleared.
> - Prefer `TBD` over invented schema details when exact property names, cardinality, hash partitioning, or persistence APIs are not source-supported.
>

### CLM-026 — Considerations

> ##### Considerations
>
> The accepted architecture basis makes this deliverable schema-first and hash-aware, but it does not yet select exact dependency versions, physical project package/container details, or package-specific implementation choices (`_CONTEXT.md`, Architecture Basis Injection). Those choices should remain explicit implementation `TBD`s until a later authorized task resolves them.
>
> The local DAG-002 mirror identifies several upstream evidence surfaces: architecture-basis deliverables, canonical model schema, project persistence/serialization, audit manifest/model hash, and analysis-status vocabulary. These rows are coordination evidence only; they do not dispatch implementation work or authorize lifecycle promotion (`Dependencies.csv`; `execution/_DAG/DAG-006/APPROVAL_RECORD.md`).
>
> The model state should not borrow formal prover-status language. SOW-071 and the context budget note both state that model states are flexible metadata records, not formal prover approval states.
>

### CLM-027 — Trade-offs

> ##### Trade-offs
>
> | Topic | Conservative setup position |
> |---|---|
> | Metadata flexibility vs. formal status lifecycle | Support flexible names, tags, notes, references, assumptions, and warnings; avoid hard-coded external approval statuses. |
> | Hashing precision vs. premature implementation choice | Require deterministic hash metadata and payload scope, but leave exact implementation library and non-JSON partitioning TBD. |
> | Snapshot immutability vs. persistence mechanics | Specify read-only snapshot semantics; defer the exact enforcement mechanism to implementation. |
> | External references vs. private-data risk | Permit explicit references while preserving privacy/protected-content checks and avoiding embedded private payloads. |
>

### CLM-028 — Examples

> ##### Examples
>
> No concrete example payload is source-supported in the accessible material. Future examples should use invented or cleared public data, document provenance, and avoid protected standards text or proprietary project values.
>

### CLM-029 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None recorded | No direct source conflict identified during setup. | TBD | TBD | TBD | TBD | TBD |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-071 OBJ-016 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
