---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-08-02
package_id: PKG-08
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@eaad463c0d481f6f1654e6adb5ee718f566176e9
project_scope_refs: [SOW-039]
package_objective_refs: [OBJ-007, OBJ-012]
---

# Scope of Work — DEL-08-02

## Purpose and Objective Traceability

This Scope of Work defines `DEL-08-02` in service of project scope [SOW-039] and package objectives [OBJ-007, OBJ-012].

- **OUT-001** — An audit-manifest and model-identity contract covering canonical JSON model hashes, separate non-JSON asset hashes, solver and software versions, units, rule-pack checksums, provenance, warnings, and replay references is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-08-02 Audit manifest and model hash

> #### Datasheet: DEL-08-02 Audit manifest and model hash
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-08-02-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-08-02 |
> | Deliverable name | Audit manifest and model hash |
> | Package ID | PKG-08 |
> | Package name | Reporting, Audit, and Reproducibility |
> | Deliverable type | BACKEND_FEATURE_SLICE |
> | Scope item | SOW-039 |
> | Supported objectives | OBJ-007; OBJ-012 |
> | Setup status | Draft setup artifact; not implementation |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Draft setup value |
> |---|---|
> | Primary artifact family | Audit manifest and reproducibility metadata for OpenPipeStress runs. |
> | Hash basis for JSON payloads | Canonical JSON with JCS-compatible canonicalization. Source: `docs/_Registers/ScopeLedger.csv` row SOW-039; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.2. |
> | Hash basis for non-JSON or binary assets | Manifest hashes recorded as separate asset entries. Source: `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.2. |
> | Required reproducibility markers | Model hash, solver version, rule-pack checksum, and input manifest. Source: `docs/_Registers/Deliverables.csv` row DEL-08-02. |
> | Rule-pack boundary | Rule packs are user/private design-basis artifacts; public artifacts may reference ID, version, checksum, and source note without embedding protected formulas. Source: `docs/SPEC.md` sections 6 and 8; `docs/IP_AND_DATA_BOUNDARY.md` section 7. |
> | Professional boundary | The manifest supports review and reproducibility; it does not certify, seal, approve, or authenticate engineering work. Source: `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/DIRECTIVE.md` section 3. |
> | Physical project container | TBD. The architecture basis says the physical package/container remains implementation-level TBD. Source: `_CONTEXT.md` Architecture Basis Injection. |
>

### CLM-005 — Conditions

> ##### Conditions
>
> - The setup artifact is constrained to document production only; no hashing code, schemas, or test files are implemented here.
> - The audit manifest must avoid private/protected payload inclusion by default. It may identify private assets by controlled references and checksums, but public templates must not embed protected standards text, proprietary formulas, private rule-pack payloads, or owner data.
> - Hashes must be deterministic for equivalent canonical input payloads. Solver version, rule-pack checksum, unit system, and manifest inputs are part of the reproducibility context.
> - Missing source, provenance, version, or rule-pack checksum information is a finding to expose, not a default to fill silently.
>

### CLM-006 — Construction

> ##### Construction
>
> The future implementation surface is expected to include:
>
> | Component | Purpose | Boundary |
> |---|---|---|
> | Input manifest | Lists canonical model inputs, rule-pack references, unit basis, selected solver settings, and external asset references needed to reproduce a run. | Records references and provenance; does not copy protected/private payloads into public artifacts. |
> | Model hash | Stable digest over the canonical JSON representation of the model payload selected for hashing. | Uses JCS-compatible canonicalization; exact hashing API is implementation work, not this setup run. |
> | Solver version stamp | Captures solver/application version and deterministic settings relevant to replay. | Does not imply validation or professional approval. |
> | Rule-pack checksum capture | Records rule-pack identity, version, checksum, source notice, and redistribution status. | Rule-pack formulas and protected values remain user/private unless lawfully redistributable. |
> | Binary/non-JSON asset manifest | Records digest, media/type, source/provenance, and inclusion policy for assets not covered by canonical JSON hashing. | Physical project container remains TBD. |
>

### CLM-007 — References

> ##### References
>
> - `_CONTEXT.md` for deliverable identity, architecture basis IDs, and setup constraints.
> - `docs/_Registers/Deliverables.csv` row DEL-08-02 for artifact and objective mapping.
> - `docs/_Registers/ScopeLedger.csv` row SOW-039 for hash-basis acceptance notes.
> - `docs/SPEC.md` sections 6, 8, 9, and 11 for rule-pack, report, V&V, and acceptance constraints.
> - `docs/IP_AND_DATA_BOUNDARY.md` sections 6 and 7 for private data and report-boundary rules.
> - `docs/CONTRACT.md` for OPS-K-IP, OPS-K-DATA, OPS-K-UNIT, OPS-K-RULE-3, OPS-K-PRIV, OPS-K-AUTH, and OPS-K-AGENT invariants.

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-08-02 Audit manifest and model hash

> #### Specification: DEL-08-02 Audit manifest and model hash
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-009 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-08-02-DECL-001`.
>

### CLM-010 — Scope

> ##### Scope
>
> This deliverable defines the setup specification for the audit manifest and model-hash feature slice. The future implementation shall capture reproducibility metadata needed to replay or professionally review an OpenPipeStress calculation package: model hash, input manifest, solver version stamp, rule-pack checksum, and referenced asset hashes.
>
> This setup run does not implement hashing code, tests, schemas, source files, or a physical project container. Those remain future implementation work under bounded Type 2 briefs.
>

### CLM-011 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source |
> |---|---|---|
> | DEL-08-02-R1 | The audit manifest shall identify the exact model input payload used for a solve/report run. | `docs/_Registers/Deliverables.csv` row DEL-08-02; `docs/SPEC.md` section 8 |
> | DEL-08-02-R2 | JSON payload hashes shall be based on canonical JSON with JCS-compatible canonicalization. | `docs/_Registers/ScopeLedger.csv` row SOW-039; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.2 |
> | DEL-08-02-R3 | Non-JSON and binary assets shall be represented through manifest asset hashes rather than folded into the JSON payload hash. | `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.2 |
> | DEL-08-02-R4 | The manifest shall record software version and solver version sufficient to interpret deterministic result reproduction. | `docs/SPEC.md` sections 4.5 and 8 |
> | DEL-08-02-R5 | The manifest shall record rule-pack name or ID, version, checksum, source notice, and redistribution status where a rule pack participates in a report/run. | `docs/SPEC.md` sections 6 and 8; `docs/IP_AND_DATA_BOUNDARY.md` section 7 |
> | DEL-08-02-R6 | The manifest shall preserve unit-system and unit-aware model context so replay does not depend on hidden unit defaults. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/SPEC.md` section 8 |
> | DEL-08-02-R7 | Missing provenance, version, checksum, or required manifest inputs shall be surfaced as warnings/findings, not silently defaulted. | `docs/DIRECTIVE.md` sections 2.2 and 3; `docs/CONTRACT.md` OPS-K-DATA-2 |
> | DEL-08-02-R8 | Public manifest/report templates shall not embed protected standards text, protected tables, proprietary formulas, private rule-pack payloads, or private project data. | `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-IP-3, OPS-K-PRIV-1; `docs/IP_AND_DATA_BOUNDARY.md` sections 3, 6, and 7 |
> | DEL-08-02-R9 | The manifest shall distinguish mechanics solved, user-rule checked, and human/professional acceptance states; it shall not claim code compliance or professional approval. | `docs/TYPES.md` sections 4 and 8; `docs/CONTRACT.md` OPS-K-AUTH-1 |
> | DEL-08-02-R10 | Hash and manifest tests shall verify deterministic stability, checksum changes on material input changes, binary asset manifest handling, and protected/private data exclusion behavior. | `docs/VALIDATION_STRATEGY.md` sections 2 and 4; `docs/SPEC.md` section 9 |
>

### CLM-012 — Standards

> ##### Standards
>
> | Standard or basis | Applicability | Status |
> |---|---|---|
> | Canonical JSON / JCS-compatible canonicalization | JSON payload hash basis for model and manifest payloads. | Required by decomposition/register basis; exact library/API TBD. |
> | JSON Schema 2020-12 | Public schema/interchange baseline for model/report-related contracts. | Architecture basis; schema files are outside this setup write scope. |
> | OpenPipeStress invariant catalog | Legal/data/professional boundary for public artifacts. | Binding project governance draft. |
>
> No protected engineering code, standard clause text, commercial example, or proprietary rule content is used as an authority in this setup artifact.
>

### CLM-013 — Verification

> ##### Verification
>
> Future implementation acceptance should include the following checks:
>
> | Verification ID | Check | Expected result |
> |---|---|---|
> | V-1 | Hash the same canonical JSON model payload twice. | Identical digest and manifest metadata. |
> | V-2 | Change a semantically relevant model input. | Model hash changes and the manifest identifies the changed payload basis. |
> | V-3 | Reorder JSON object keys without changing data. | Model hash remains stable under canonicalization. |
> | V-4 | Add or alter a binary/non-JSON asset entry. | Asset manifest digest changes without redefining the model JSON hash basis. |
> | V-5 | Omit rule-pack checksum or source notice. | Manifest emits a missing-data/provenance finding. |
> | V-6 | Include a private or protected payload candidate in a public template path. | Protected/private data guardrail blocks or flags the inclusion. |
> | V-7 | Generate a report from a manifest-backed run. | Report includes version, manifest, hash/checksum, warnings, assumptions, and professional-boundary notice. |
>

### CLM-014 — Documentation

> ##### Documentation
>
> This setup deliverable produces:
>
> - `Datasheet.md`
> - `Specification.md`
> - `Guidance.md`
> - `Procedure.md`
> - `_SEMANTIC.md`
> - `_SEMANTIC_LENSING.md`
> - `Dependencies.csv`
> - `_DEPENDENCIES.md`
> - `_run_records/*`
> - `_STATUS.md`
>
> Future implementation artifacts anticipated by the register are `audit manifest` and `hash tests`; they are not created in this setup session.

- **AC-001** — The contract preserves the explicit hash boundary and deterministic canonicalization basis, complete reproducibility metadata, missing-data findings, protected/private payload exclusion, and separation of computation identity from compliance or professional acceptance without selecting an unapproved library or container.

## Production and Verification Method — Praxeology

### CLM-015 — Procedure: DEL-08-02 Audit manifest and model hash

> #### Procedure: DEL-08-02 Audit manifest and model hash
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-016 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-08-02-DECL-004`.
>

### CLM-017 — Purpose

> ##### Purpose
>
> This procedure defines how the future audit-manifest feature should be produced and verified. In this setup session it is documentation only; no hashing code, schemas, source implementation, or tests are created.
>

### CLM-018 — Prerequisites

> ##### Prerequisites
>
> Before implementation work proceeds, the following inputs or upstream contracts should be available or explicitly marked `TBD`:
>
> | Prerequisite | Expected source | Reason |
> |---|---|---|
> | Canonical model/persistence payload boundary | DEL-02-05 Project persistence and round-trip serialization | The model hash needs a defined canonical JSON payload boundary and migration/versioning posture. |
> | Unit-system and unit-bearing value conventions | DEL-02-02 Unit system and dimensional-analysis core contract | Reproducible model identity must not depend on implicit unit defaults. |
> | Rule-pack checksum and lifecycle fields | DEL-06-04 Private rule-pack lifecycle and checksum handling | The manifest records rule-pack identity/version/checksum/source status without copying private payloads. |
> | Solver/version and deterministic run metadata | DEL-10-05 Headless CLI and structured I/O analysis runner | Automated replay/report workflows need a consistent run envelope and version stamp. |
> | Report renderer integration point | DEL-08-01 Calculation report generator | Reports consume manifest fields for review and reproducibility. |
> | Result export contract | DEL-08-04 Result export format | Exported results should carry or reference manifest/hash metadata for regression and review. |
>
> If any prerequisite is unavailable during future implementation, record the missing dependency as `TBD` or a warning rather than inventing a default.
>

### CLM-019 — Steps

> ##### Steps
>
> 1. Define the manifest boundary for the run: model payload reference, unit system, solver/application version, rule-pack references, external asset references, warnings, assumptions, and professional-boundary notices.
> 2. Canonicalize JSON payloads using the project-approved JCS-compatible basis before hashing.
> 3. Record non-JSON and binary assets as manifest entries with digest, provenance, media/type, and inclusion policy.
> 4. Capture rule-pack identity, version, checksum, source notice, and redistribution status without copying protected formulas or private values into public artifacts.
> 5. Capture solver version and deterministic settings relevant to replay or regression comparison.
> 6. Emit missing-input, missing-provenance, protected-content, or private-data warnings when manifest fields cannot be populated safely.
> 7. Expose manifest and hash metadata to report generation and result export surfaces without claiming professional approval or code compliance.
> 8. Verify deterministic behavior through hash/reproducibility tests in a later implementation brief.
>

### CLM-020 — Verification

> ##### Verification
>
> For this setup run, verification consists of:
>
> - the four production documents exist and preserve the default section roles;
> - `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` exist and remain lens artifacts, not engineering authorities;
> - `Dependencies.csv` validates against the v3.1 schema;
> - `_DEPENDENCIES.md` summarizes extracted dependency rows consistently;
> - `_STATUS.md` is `SEMANTIC_READY` only after the setup sequence succeeds;
> - no private/protected payloads or certification claims are introduced.
>
> For future implementation, run the hash determinism, manifest stability, protected-content, and report reproducibility tests described in `Specification.md`.
>

### CLM-021 — Records

> ##### Records
>
> This setup sequence records:
>
> - document drafts in `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`;
> - semantic setup outputs in `_SEMANTIC.md` and `_SEMANTIC_LENSING.md`;
> - dependency outputs in `Dependencies.csv` and `_DEPENDENCIES.md`;
> - execution evidence in `_run_records/`;
> - lifecycle evidence in `_STATUS.md`.

- **VER-001** — Validate the contract and review source parity, canonicalization and hash-boundary rules, deterministic-change behavior, asset-manifest handling, version/unit/provenance fields, protected/private data limits, professional-boundary language, and every retained TBD or governed residual.

## Governing Values and Decisions — Axiology

### CLM-022 — Guidance: DEL-08-02 Audit manifest and model hash

> #### Guidance: DEL-08-02 Audit manifest and model hash
>

### CLM-023 — Purpose

> ##### Purpose
>
> The audit manifest is the bridge between a computed model run and a reviewable evidence package. It should let a reviewer determine which model payload, rule-pack reference, solver version, unit basis, warnings, and external assets were involved without treating the software output as professional approval.
>

### CLM-024 — Principles

> ##### Principles
>
> - Prefer deterministic identity over convenience. Equivalent JSON payloads should hash the same way after canonicalization; materially different inputs should produce different identifiers.
> - Keep the hash boundary explicit. JSON payloads use the canonical JSON/JCS-compatible basis; binary and other non-JSON assets use manifest entries with their own hashes.
> - Record references without leaking payloads. Rule-pack IDs, versions, checksums, source notices, and redistribution statuses are useful; protected formulas, private owner data, and copyrighted standard text do not belong in public artifacts.
> - Treat missing data as evidence. Missing provenance, checksum, solver version, or manifest inputs should be visible warnings/findings.
> - Preserve the professional boundary. A manifest can support replay and review; it cannot certify code compliance, seal a calculation, or replace competent engineering review.
>

### CLM-025 — Considerations

> ##### Considerations
>
> | Topic | Guidance |
> |---|---|
> | Canonicalization | The canonicalization basis is selected at the architecture/decomposition level. This deliverable should not choose a physical project container or hashing library without a later implementation brief or human decision. |
> | Units | The manifest should make the unit system and unit-bearing values reproducible enough that replay does not depend on implicit defaults. |
> | Rule packs | Rule-pack checksum capture should reference the user/private artifact without copying protected rule text or proprietary values into public outputs. |
> | Solver version | Version stamping should be granular enough to explain deterministic replay and regression comparison. Exact versioning format remains implementation work. |
> | Binary assets | Non-JSON assets should be individually addressed in the manifest with digest and provenance fields, because the physical project container is still TBD. |
> | Reports | Report generation should consume the manifest so report reproducibility and protected-content linting can be tested. |
>

### CLM-026 — Trade-offs

> ##### Trade-offs
>
> | Choice | Benefit | Risk / open item |
> |---|---|---|
> | Hash only canonical JSON payloads | Stable digest under key ordering and formatting changes. | Requires a strict canonicalization contract and tests. |
> | Include binary assets by manifest entry | Keeps non-JSON handling explicit and container-independent. | Requires clear asset inclusion/exclusion policy in future implementation. |
> | Record private rule-pack references instead of payloads | Protects user/proprietary content while preserving reproducibility metadata. | Replay may require private artifacts that cannot be shared publicly. |
> | Surface missing fields as warnings | Avoids silent defaults and supports professional review. | Review workflows must decide which missing fields are blockers. |
>

### CLM-027 — Examples

> ##### Examples
>
> No concrete engineering model, code-specific rule, material allowable, SIF/flexibility value, protected standard example, or private project payload is included in this setup artifact.
>
> Illustrative manifest slots, without values, are:
>
> - `model_hash`
> - `canonicalization_basis`
> - `solver_version`
> - `unit_system`
> - `rule_pack_refs[]`
> - `input_payload_refs[]`
> - `asset_manifest[]`
> - `warnings[]`
> - `professional_boundary_notice`
>

### CLM-028 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None | No source conflict identified in setup pass. | N/A | N/A | N/A | N/A | N/A |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-039 OBJ-007 OBJ-012 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
