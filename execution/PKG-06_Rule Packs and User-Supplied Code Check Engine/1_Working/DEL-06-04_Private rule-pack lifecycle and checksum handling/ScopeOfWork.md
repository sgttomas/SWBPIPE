---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-06-04
package_id: PKG-06
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@eaad463c0d481f6f1654e6adb5ee718f566176e9
project_scope_refs: [SOW-042]
package_objective_refs: [OBJ-002, OBJ-005]
---

# Scope of Work — DEL-06-04

## Purpose and Objective Traceability

This Scope of Work defines `DEL-06-04` in service of project scope [SOW-042] and package objectives [OBJ-002, OBJ-005].

- **OUT-001** — A private rule-pack lifecycle and checksum contract covering stable identity and version, source/provenance notice, redistribution and quarantine status, canonical JSON/JCS-compatible payload hashing, non-JSON manifest hashes, diagnostics, and report/audit references is produced for the declared scope and objectives.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-06-04 Private rule-pack lifecycle and checksum handling

> #### Datasheet: DEL-06-04 Private rule-pack lifecycle and checksum handling
>

### CLM-002 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-06-04 |
> | Deliverable name | Private rule-pack lifecycle and checksum handling |
> | Package ID | PKG-06 |
> | Package name | Rule Packs and User-Supplied Code Check Engine |
> | Deliverable type | BACKEND_FEATURE_SLICE |
> | Scope item | SOW-042 |
> | Objectives | OBJ-002, OBJ-005 |
> | Current setup role | Document and setup artifact production only; no implementation files changed |
>

### CLM-003 — Attributes

> ##### Attributes
>
> | Attribute | Setup value |
> |---|---|
> | Rule-pack boundary | Rule packs are user-owned or private design-basis artifacts; public project content is limited to schemas, mechanics, workflows, and invented examples. |
> | Lifecycle subject | Version identity, source/provenance notes, public/private or redistribution status, checksum metadata, and audit/report references for rule packs. |
> | Checksum basis | JSON rule-pack payloads use canonical JSON with JCS-compatible canonicalization before hashing. Non-JSON or binary assets require manifest-level hashes and an explicit payload reference. |
> | Required checksum metadata | Algorithm, canonicalization where applicable, payload reference, checksum value, and statement of what was hashed. |
> | Redistribution status | Must be explicit. Supported values are owned by the rule-pack schema deliverable; project sources require at least private, public-permissive, unknown, and suspected-protected handling. |
> | Public repository posture | Private rule-pack content, licensed standards text, copied formulas, proprietary tables, owner requirements, and vendor data without redistribution rights are not committed publicly. |
> | Report posture | Reports may reference rule-pack ID/name, version, checksum, and source note without exposing protected formulas or private values in public templates. |
>

### CLM-004 — Conditions

> ##### Conditions
>
> | Condition | Constraint |
> |---|---|
> | Protected data | Do not include protected standards text, tables, figures, examples, copied code formulas, material allowables, SIF/flexibility tables, dimensional standards, or proprietary vendor data. |
> | Private data | Private rule packs remain user-controlled and are not transmitted, exported, or committed by default. |
> | Professional responsibility | Rule-pack evaluation is software decision support using user data. It is not certification, sealing, approval, endorsement, or a code-compliance claim. |
> | Unit awareness | Rule-pack values and evaluator inputs must remain unit-aware where numeric quantities are involved; missing required units or values are findings, not silent defaults. |
> | Architecture basis | SCA-001 requires JSON Schema 2020-12 contracts, schema-first command/query/job result envelopes, and canonical JSON/JCS-compatible hash basis where JSON payloads are hashed. |
> | Deferred decisions | Private storage location, encryption defaults, access-control policy defaults, permission grant persistence, and physical project container are deferred to PKG-12/PKG-02 architecture decisions unless separately approved. |
>

### CLM-005 — Construction

> ##### Construction
>
> This setup surface describes the future implementation contract without creating implementation files. A later implementation task for DEL-06-04 should produce a local lifecycle and checksum mechanism that:
>
> - records rule-pack identity, version, source notice, redistribution/private status, checksum metadata, and review/quarantine disposition;
> - binds checksum values to explicit payload references rather than environment-local paths or volatile session state;
> - records diagnostics for missing provenance, missing redistribution status, stale hashes, suspected protected content, and attempts to expose private content publicly;
> - exposes audit-manifest hooks for reports and reproducibility artifacts without embedding private rule-pack formulas or values in public templates;
> - defers storage location, encryption, and access-control defaults to the security/privacy package.
>

### CLM-006 — References

> ##### References
>
> - INIT.md - bootstrap boundaries for open mechanics, private code data, and professional responsibility.
> - docs/CONTRACT.md - OPS-K-RULE-3, OPS-K-DATA-1/2/3, OPS-K-PRIV, OPS-K-IP-1/2/3, OPS-K-UNIT-1, OPS-K-AGENT-1..4.
> - docs/SPEC.md - rule-pack evaluator minimum sections and report/audit requirements.
> - docs/IP_AND_DATA_BOUNDARY.md - public/private data and quarantine policy.
> - docs/PRD.md - rule-pack requirements, user journey, reporting, and private data handling.
> - docs/architecture/persistence_contract.md - canonical JSON/JCS hash basis and hash metadata.
> - docs/architecture/code_neutral_analysis_boundary.md - rule-pack reference and professional-boundary separation.
> - docs/architecture/extension_domain_contracts.md - no-bypass constraints for provenance, privacy, checksums, and protected-content controls.
> - execution/_Decomposition/SOFTWARE_DECOMP.md - DEL-06-04 scope and SCA-001 architecture basis.
>

### CLM-007 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. Private rule-pack lifecycle schemas, fixtures, checksum handling, and tests now exist. Quarantine taxonomy, provider/storage breadth, encryption, and other controls remain residual only where explicitly recorded; this is not a security-assurance ruling.

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-06-04 Private rule-pack lifecycle and checksum handling

> #### Specification: DEL-06-04 Private rule-pack lifecycle and checksum handling
>

### CLM-009 — Scope

> ##### Scope
>
> This deliverable defines the setup contract for private rule-pack lifecycle and checksum handling in PKG-06. It covers metadata, lifecycle guardrails, checksum handling, audit/report hooks, and diagnostics needed to keep rule packs versioned, source-noted, checksum-addressed, and marked for redistribution/private status.
>
> This setup does not implement registry modules, schemas, tests, encryption, private storage, access-control defaults, or private rule-pack content.
>

### CLM-010 — Requirements

> ##### Requirements
>
> | Requirement ID | Requirement | Source basis |
> |---|---|---|
> | R-06-04-001 | A rule-pack lifecycle record shall include stable rule-pack identity, human-readable name, version, source/provenance notice, redistribution/private status, and checksum metadata. | docs/CONTRACT.md OPS-K-RULE-3; docs/SPEC.md Section 6; docs/PRD.md Section 12 |
> | R-06-04-002 | Private rule packs shall be marked private by default unless a public-permissive redistribution basis is explicitly recorded. | docs/PRD.md Section 12.4 and 17.3; docs/IP_AND_DATA_BOUNDARY.md Section 6 |
> | R-06-04-003 | Public project artifacts shall not contain private rule-pack content, protected standards text, protected tables, copied code formulas, proprietary vendor data, owner standards, or company design bases. | docs/CONTRACT.md OPS-K-IP-1/3; docs/IP_AND_DATA_BOUNDARY.md Section 3 |
> | R-06-04-004 | JSON rule-pack payload hashes shall use canonical JSON with JCS-compatible canonicalization and shall record algorithm, canonicalization, payload reference, and checksum value. | docs/architecture/persistence_contract.md Hash Rules; SCA-001 architecture basis |
> | R-06-04-005 | Non-JSON or binary rule-pack-related assets shall be represented by manifest hashes with explicit payload references; exact partitioning remains TBD. | docs/architecture/persistence_contract.md Hash Rules and Remaining TBDs |
> | R-06-04-006 | A checksum shall identify what was hashed and shall not depend on environment-local paths, timestamps, UI session state, or other volatile fields unless their treatment is explicitly documented. | docs/architecture/persistence_contract.md Hash Rules |
> | R-06-04-007 | The lifecycle shall emit diagnostics for missing source notice, missing redistribution status, stale or missing checksum, suspected protected content, attempted public export of private content, and rule-check-required data gaps. | docs/SPEC.md Sections 6-8; docs/architecture/code_neutral_analysis_boundary.md Boundary Rules |
> | R-06-04-008 | Reports and exports may reference private rule-pack identity, version, checksum, and source note without exposing protected formulas or private values in public templates. | docs/IP_AND_DATA_BOUNDARY.md Section 7; docs/PRD.md Section 15 |
> | R-06-04-009 | Rule-pack lifecycle status shall not state or imply professional certification, sealing, approval, endorsement, or automatic code compliance. | INIT.md Agent rule; docs/CONTRACT.md OPS-K-AUTH-1; docs/architecture/code_neutral_analysis_boundary.md Status Separation |
> | R-06-04-010 | Numeric rule-pack inputs and values shall preserve unit metadata where applicable; missing required values or units shall be explicit findings. | docs/CONTRACT.md OPS-K-UNIT-1 and OPS-K-DATA-2/3 |
> | R-06-04-011 | Storage location, encryption default, access-control policy, permission persistence, and private-library secret handling are deferred architecture decisions for PKG-12 and related persistence work. | DEL-06-04 Context Envelope Notes; docs/architecture/persistence_contract.md Remaining TBDs |
> | R-06-04-012 | The implementation boundary shall preserve schema-first service/result envelope behavior and shall not allow adapters or plugins to bypass provenance, privacy, protected-content, checksum, or report controls. | docs/architecture/extension_domain_contracts.md No-Bypass Rules |
>

### CLM-011 — Standards

> ##### Standards
>
> No engineering code or standards-body requirement is implemented by this setup deliverable. Applicable project governance sources are:
>
> - OPS-K-RULE-3: rule packs are versioned, checksummed, source-noted, and marked public/private.
> - OPS-K-DATA-1/2/3: code-specific values are user-supplied/private, missing values are findings, and values carry provenance.
> - OPS-K-PRIV-1/2: private data is not committed or transmitted publicly by default; telemetry is off by default and cannot include private engineering/code data.
> - OPS-K-IP-1/2/3: protected content is excluded, provenance is required for public data, and suspected protected content is quarantined.
> - OPS-K-UNIT-1: calculations, formulas, imported values, and exports are unit-aware and dimensionally checked.
> - OPS-K-AGENT-1..4: no invented engineering values, gaps surfaced, sealed scope respected, outputs draft until accepted.
>

### CLM-012 — Verification

> ##### Verification
>
> | Verification ID | Check |
> |---|---|
> | V-06-04-001 | Four setup documents exist and retain required sections. |
> | V-06-04-002 | `_SEMANTIC.md` exists and contains matrix sections A, B, C, F, D, K, G, X, T, and E. |
> | V-06-04-003 | `_SEMANTIC_LENSING.md` exists and contains lens coverage for matrices A, B, C, F, D, X, and E. |
> | V-06-04-004 | `Dependencies.csv` exists, is parseable, and contains all v3.1 required columns. |
> | V-06-04-005 | Dependency enum fields use canonical values where checked. |
> | V-06-04-006 | `_DEPENDENCIES.md` summarizes active anchors and execution edges consistently with `Dependencies.csv`. |
> | V-06-04-007 | No private rule-pack contents, protected standards data, proprietary formulas, or certification/compliance claims are introduced. |
> | V-06-04-008 | `_STATUS.md` is set to `SEMANTIC_READY` only after setup artifacts and local validations pass. |
>

### CLM-013 — Documentation

> ##### Documentation
>
> Required setup outputs for this session are:
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
> Implementation outputs such as registry modules, schemas, tests, encryption/access policy defaults, private storage paths, and actual private rule-pack files are outside this setup session.
>

### CLM-014 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. Private rule-pack lifecycle schemas, fixtures, checksum handling, and tests now exist. Quarantine taxonomy, provider/storage breadth, encryption, and other controls remain residual only where explicitly recorded; this is not a security-assurance ruling.

- **AC-001** — The contract preserves the accepted local-first privacy, explicit payload-bound checksum, stale-hash, protected-content, unit, and professional-authority boundaries while deferring storage, encryption, access policy, permission persistence, and final redistribution enums to their governed owners.

## Production and Verification Method — Praxeology

### CLM-015 — Procedure: DEL-06-04 Private rule-pack lifecycle and checksum handling

> #### Procedure: DEL-06-04 Private rule-pack lifecycle and checksum handling
>

### CLM-016 — Purpose

> ##### Purpose
>
> This procedure describes how a future bounded implementation task should produce private rule-pack lifecycle and checksum handling while preserving the public/private data boundary. It also records how this setup run should be verified.
>

### CLM-017 — Prerequisites

> ##### Prerequisites
>
> - Read and apply INIT.md, AGENTS.md, docs/CONTRACT.md, docs/SPEC.md, docs/IP_AND_DATA_BOUNDARY.md, docs/PRD.md, and the DEL-06-04 sealed context.
> - Confirm the implementation task has an explicit write scope before editing code, schemas, registry modules, or tests.
> - Treat DEL-06-01 rule-pack schema as an upstream schema source for final field names and enum ownership.
> - Treat DEL-08-02 audit manifest/model hash as a downstream consumer of rule-pack checksum metadata.
> - Treat PKG-12 security/privacy deliverables as the owner for private storage locations, encryption/access defaults, telemetry, redaction, and secret handling.
> - Do not use or create real private rule-pack payloads in public repository artifacts.
>

### CLM-018 — Steps

> ##### Steps
>
> 1. Establish the rule-pack lifecycle metadata boundary.
>    - Include identity, version, source notice, redistribution/private status, checksum metadata, diagnostics, and review/quarantine disposition.
>    - Do not embed protected code text, copied formulas, private values, or proprietary tables.
>
> 2. Define checksum payload boundaries.
>    - For JSON payloads, use canonical JSON with JCS-compatible canonicalization before hashing.
>    - Record algorithm, canonicalization, payload reference, and checksum value.
>    - For non-JSON or binary assets, create a manifest-level hash entry and mark unresolved partition details as `TBD`.
>
> 3. Define stale-check and lifecycle diagnostics.
>    - Missing source note, missing redistribution status, missing checksum, stale checksum, suspected protected content, and attempted public export of private content must be explicit diagnostics.
>    - Missing rule-check-required values or units remain rule/check findings, not defaulted values.
>
> 4. Connect audit/report hooks.
>    - Provide report-facing references to rule-pack ID/name, version, checksum, and source note.
>    - Public templates must not render private formulas, protected tables, or copied standards text.
>
> 5. Preserve professional-boundary wording.
>    - Do not emit `CODE_COMPLIANT`, certification, approval, sealing, endorsement, or equivalent professional reliance language.
>    - Keep human acceptance external and hash-bound if it is introduced by later work.
>
> 6. Defer private storage and access decisions.
>    - Record storage location, encryption defaults, access-control policy, permission persistence, and credential handling as `TBD` or PKG-12-owned unless the sealed brief explicitly authorizes them.
>
> 7. Verify setup artifacts for this run.
>    - Confirm the four documents exist.
>    - Confirm semantic matrix and lensing artifacts exist.
>    - Validate `Dependencies.csv` with the v3.1 schema validator.
>    - Check dependency enum fields with `validate_enum.py`.
>    - Confirm `_STATUS.md` remains `SEMANTIC_READY` only after all setup gates pass.
>

### CLM-019 — Verification

> ##### Verification
>
> | Check | Expected result |
> |---|---|
> | Four-document kit | `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` exist. |
> | Data boundary scan | No private rule-pack payloads, protected standards data, proprietary formulas, or code-compliance claims are present. |
> | Checksum basis | Specification and guidance state JCS-compatible canonical JSON for JSON payload hashes. |
> | Deferred decisions | Storage, encryption, access-control defaults, and physical container decisions remain deferred. |
> | Dependency schema | `Dependencies.csv` validates as v3.1. |
> | Lifecycle state | `_STATUS.md` states `SEMANTIC_READY` after setup artifacts validate. |
>

### CLM-020 — Records

> ##### Records
>
> This setup session records:
>
> - four-document drafting and P3 enrichment in the production documents;
> - semantic matrix output in `_SEMANTIC.md`;
> - semantic lensing register in `_SEMANTIC_LENSING.md`;
> - dependency extraction results in `Dependencies.csv` and `_DEPENDENCIES.md`;
> - run records in `_run_records/`;
> - final lifecycle state in `_STATUS.md`.
>

### CLM-021 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. Private rule-pack lifecycle schemas, fixtures, checksum handling, and tests now exist. Quarantine taxonomy, provider/storage breadth, encryption, and other controls remain residual only where explicitly recorded; this is not a security-assurance ruling.

- **VER-001** — Validate the contract and review source parity, lifecycle metadata, checksum payload boundaries, private/public and quarantine handling, diagnostics, audit hooks, no-bypass constraints, deferred security ownership, and all surviving source conflicts.

## Governing Values and Decisions — Axiology

### CLM-022 — Guidance: DEL-06-04 Private rule-pack lifecycle and checksum handling

> #### Guidance: DEL-06-04 Private rule-pack lifecycle and checksum handling
>

### CLM-023 — Purpose

> ##### Purpose
>
> DEL-06-04 exists to keep rule-pack lifecycle and checksum behavior auditable without putting protected or private rule-pack content into the public repository. The future implementation should let users reference and validate their own design bases while preserving the code-neutral, private-data boundary.
>

### CLM-024 — Principles

> ##### Principles
>
> - Treat the checksum as evidence about a specific payload, not as proof of engineering correctness.
> - Treat privacy and redistribution status as explicit metadata, not an inference from file location.
> - Prefer source notes and provenance over embedded protected text.
> - Keep lifecycle diagnostics visible: missing provenance, missing checksum, stale checksum, private-data export attempts, and suspected protected content are findings.
> - Preserve the mechanics/rule/professional boundary: a user-rule check is not professional approval.
> - Keep storage, encryption, access policy, and secret handling decisions out of this deliverable unless the human project authority records a later architecture decision.
>

### CLM-025 — Considerations

> ##### Considerations
>
> | Topic | Guidance |
> |---|---|
> | JCS basis | For JSON rule-pack payloads, canonicalize with the project-selected JCS-compatible basis before hashing. Record `algorithm`, `canonicalization`, `payload_ref`, and `value`. |
> | Payload boundary | A hash must state what was hashed. Do not hash local paths, timestamps, UI state, or other volatile fields as reproducibility evidence unless excluded or documented in a manifest. |
> | Private marking | A rule pack with unknown redistribution rights should not be treated as public. The safer state is private or unknown until reviewed. |
> | Protected suspected | Suspected protected standards or proprietary content should trigger quarantine/human review rather than public commit. |
> | Audit hooks | Reports and exports should identify rule-pack name or ID, version, checksum, and source note. They should not expose private formulas or protected tables in public templates. |
> | Unit metadata | Rule-pack values used for numeric evaluation need units or dimension metadata where applicable. Missing units should block or warn rather than default silently. |
> | Deferred architecture | Private storage directories, encryption defaults, access grants, revocation, and credential handling belong to PKG-12 and persistence decisions. |
>

### CLM-026 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Direction for this setup |
> |---|---|
> | Strong reproducibility vs. privacy | Reference hashes and source notes without publishing private payloads. |
> | Detailed source trace vs. protected-content risk | Store source notices and provenance summaries; do not copy licensed text or tables into public artifacts. |
> | Local-first convenience vs. governed export | Keep private data local by default; require explicit export/report decisions for any exposure. |
> | Schema stability vs. implementation detail | Record metadata and lifecycle expectations now; leave enum finalization and module contracts to implementation/schema deliverables. |
>

### CLM-027 — Examples

> ##### Examples
>
> Safe public examples for this deliverable may show metadata shape only, using invented placeholders such as:
>
> ```yaml
> rule_pack_ref:
>   id: "invented-demo-only"
>   version: "0.1.0"
>   source_notice: "Invented demonstration values only"
>   redistribution_status: "public_permissive"
>   checksum:
>     algorithm: "sha256"
>     canonicalization: "JCS"
>     payload_ref: "rule_pack"
>     value: "TBD"
> ```
>
> Private or protected formulas, allowables, code text, tables, and owner design-basis values must not be substituted into public examples.
>

### CLM-028 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | C-06-04-001 | Exact private storage location, encryption default, and access-control policy are required for implementation but out of scope for this deliverable. | DEL-06-04 context notes: access control details defer to PKG-12 | docs/PRD.md Section 18.3 lists optional encrypted storage | Datasheet Conditions; Specification R-06-04-011; Procedure Prerequisites | Defer to PKG-12 and persistence architecture; do not decide here. | TBD |
> | C-06-04-002 | Exact schema enum set for redistribution/private status is not finalized by this deliverable. | docs/SPEC.md Section 6 uses `private`, `public_permissive`, `unknown` | docs/IP_AND_DATA_BOUNDARY.md includes `private_only` and `protected_suspected` for data records | Specification R-06-04-003; Datasheet Attributes | Record minimum lifecycle need and defer final enum ownership to DEL-06-01/schema work. | TBD |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-042 OBJ-002 OBJ-005 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
