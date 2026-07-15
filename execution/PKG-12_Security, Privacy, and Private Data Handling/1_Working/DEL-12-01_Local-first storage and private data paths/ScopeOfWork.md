---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-12-01
package_id: PKG-12
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@4d153302c3c4cd42578936db160c2bac1270225a
project_scope_refs: [SOW-029]
package_objective_refs: [OBJ-010]
---

# Scope of Work — DEL-12-01

## Purpose and Objective Traceability

This Scope of Work defines `DEL-12-01` in service of project scope [SOW-029] and package objectives [OBJ-010].

- **OUT-001** — A local-first storage and symbolic private-data path contract for user-controlled project, rule-pack, library, report, and transient data is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-12-01 Local-first storage and private data paths

> #### Datasheet: DEL-12-01 Local-first storage and private data paths
>

### CLM-002 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-12-01 |
> | Deliverable Name | Local-first storage and private data paths |
> | Package ID | PKG-12 |
> | Package Name | Security, Privacy, and Private Data Handling |
> | Deliverable Type | SECURITY_CONTROL |
> | Scope Item | SOW-029 |
> | Objective | OBJ-010 |
> | Setup Run Date | 2026-04-30 |
> | Readiness Evidence Basis | 2026-06-07 metadata-only guard code, policy documentation, tests, and fan-in evidence; no lifecycle promotion by this alignment run |
>

### CLM-003 — Attributes

> ##### Attributes
>
> | Attribute | Value |
> |---|---|
> | Product posture | Local-first by default |
> | Protected private data classes | Private rule packs; private material data; private component data; project models; user-owned code/design-basis data |
> | Public repository boundary | Public code, schemas, workflows, and invented examples only; no protected standards data or user-private engineering data |
> | Cloud posture | Out of MVP unless separately approved by the human project authority |
> | Persistence baseline | Versioned JSON-schema-governed persistence with canonical JSON/JCS-compatible hashes for JSON payloads |
> | Physical project package/container mechanics | TBD; SCA-003 establishes the local SQLite-backed storage profile, but OS roots, runtime persistence behavior, and package mechanics remain deferred |
> | Implementation status | Metadata-only local-first storage guard helper exists as of 2026-06-07; runtime storage implementation remains deferred |
> | Real private path creation | None |
> | Secret handling | TBD and owned by DEL-12-04; no real secret storage, encryption, or key-management behavior is selected here |
>

### CLM-004 — Conditions

> ##### Conditions
>
> | Condition | Constraint |
> |---|---|
> | Local-first storage | Private project and library data remain user-controlled by default and are not transmitted or committed publicly by default. |
> | Deterministic persistence | Storage conventions must remain compatible with deterministic round-trip serialization, schema versions, migrations, units, provenance, and reproducibility manifests. |
> | Path convention level | This deliverable records symbolic path classes only; OS-specific roots and the physical package/container remain implementation-level TBD. |
> | Public/private separation | Public repository paths must not be used as default private-library or private-project storage locations. |
> | Export/report boundary | Export and report paths can expose private data and must defer redaction/export controls to DEL-12-02. |
> | Professional boundary | Storage status, rule-pack presence, or report export must not be framed as certification, sealing, approval, or code compliance. |
>

### CLM-005 — Symbolic Path Classes

> ##### Symbolic Path Classes
>
> | Symbolic Path Class | Intended Use | Boundary |
> |---|---|---|
> | `PUBLIC_REPO_ROOT` | Public source, schemas, documentation, invented examples, and validation artifacts safe for redistribution. | Must not hold private rule packs, owner standards, project models, private material libraries, private component libraries, credentials, or protected standards data. |
> | `USER_CHOSEN_PROJECT_PATH` | User-selected local project model storage. | Private by default; actual folder and physical container remain TBD. |
> | `USER_PRIVATE_LIBRARY_ROOT` | User-controlled local location for private rule packs, material libraries, component libraries, and owner/company design-basis data. | Must be outside default public repository paths and must not be committed publicly by default. |
> | `PROJECT_PRIVATE_ASSET_ROOT` | Project-scoped private attachments, local manifests, and non-public supporting files. | Symbolic only in this deliverable; physical packaging and migration strategy remain TBD. |
> | `REPORT_EXPORT_TARGET` | Local report/export destination selected by the user. | May contain private values; redaction/export safeguards belong to DEL-12-02. |
> | `APP_CACHE_OR_SESSION_STATE` | Transient GUI/session/cache state. | Must not become an ungoverned durable private-data store. |
>

### CLM-006 — Construction

> ##### Construction
>
> This artifact began as a documentation-level storage boundary. As of 2026-06-07, readiness evidence also includes a metadata-only guard helper and focused tests. The deliverable output is limited to:
>
> - local-first storage policy requirements in `Specification.md`;
> - private path convention guidance in this datasheet and `Guidance.md`;
> - verification expectations and current metadata-guard evidence in `Procedure.md`;
> - semantic matrix/lensing and dependency setup artifacts.
> - product-level policy documentation in `docs/security/local_first_storage_policy.md`;
> - metadata-only guard code in `core/security/local_first_storage/`;
> - focused tests in `tests/security/test_local_first_storage_policy.py`.
>
> The June 7 guard helper classifies explicit storage metadata only. It does not create a runtime storage service, choose OS-specific roots, create real private directories, store payloads or secrets, open SQLite handles, authorize direct SQL, implement encryption or key management, create cloud service assumptions, or make approval, security-certification, professional, sealing, authentication, or code-compliance claims.
>

### CLM-007 — References

> ##### References
>
> | Source | Use |
> |---|---|
> | `INIT.md` | Bootstrap boundaries: open mechanics, private data, human authority, no certification claims. |
> | `AGENTS.md` | Type 2 scoped execution and write-scope constraints. |
> | `docs/DIRECTIVE.md` | Local/private data principles, hidden cloud/telemetry exclusion, professional boundary. |
> | `docs/CONTRACT.md` | OPS-K-IP, OPS-K-DATA, OPS-K-PRIV, OPS-K-AUTH, OPS-K-UNIT, and agent invariants. |
> | `docs/TYPES.md` | SECURITY_CONTROL type, private/user-supplied data vocabulary, canonical object registry. |
> | `docs/SPEC.md` | Layered architecture, project storage-policy field, private rule-pack schema, report and agentic acceptance semantics. |
> | `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data policy and private user data boundary. |
> | `docs/security/local_first_storage_policy.md` | Product-level local-first storage policy and metadata-only guard helper boundary. |
> | `core/security/local_first_storage/` | Metadata-only storage guard helper evidence; not a runtime storage implementation. |
> | `tests/security/test_local_first_storage_policy.py` | Focused invented-fixture tests for metadata classification and public/private leakage blocking. |
> | `execution/_Decomposition/SOFTWARE_DECOMP.md` | PKG-12, DEL-12-01, SOW-029, OBJ-010, AB-00 architecture basis. |
> | `docs/_Registers/Deliverables.csv` | Deliverable identity, anticipated artifacts, context/risk notes. |
> | `docs/_Registers/ScopeLedger.csv` | Scope ledger row for SOW-029. |
> | `TASK_RUN_2026-06-07_0140.md` and package fan-in run `WORKING_ITEMS_RUN_2026-06-07_0150_TP-PKG12-LOCAL-PRIVACY-GUARDS-FANIN.md` | June 7 implementation and fan-in evidence; no lifecycle, approval, storage-schema, runtime-storage, cloud, encryption, or certification claim. |
>
> PDU-036 does not change the storage evidence inventory: the trace-gap fixture is cross-deliverable verification evidence, not a storage runtime, migration, round-trip, or report/export fixture.

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-12-01 Local-first storage and private data paths

> #### Specification: DEL-12-01 Local-first storage and private data paths
>

### CLM-009 — Scope

> ##### Scope
>
> This deliverable specifies the local-first storage boundary and symbolic private data path conventions for project models, private rule packs, private material data, private component data, and related user-owned code/design-basis data.
>
> The original setup run was documentation production only. Later bounded TASK evidence now exists: a metadata-only `core.security.local_first_storage` guard helper, updated `docs/security/local_first_storage_policy.md`, and focused invented-fixture tests in `tests/security/test_local_first_storage_policy.py`.
>
> This deliverable still does not implement runtime storage, choose OS-specific roots, create private paths, create real secrets or private data, implement encryption or key management, introduce cloud service defaults, or make approval, security-certification, professional, sealing, authentication, or code-compliance claims. SCA-003 establishes the local SQLite-backed MVP storage profile, but detailed physical package/container mechanics remain deferred.
>

### CLM-010 — Requirements

> ##### Requirements
>
> | Requirement ID | Requirement | Source Basis | Verification |
> |---|---|---|---|
> | LFSP-REQ-001 | The product storage posture shall be local-first by default for private project, rule-pack, material, component, and code/design-basis data. | SOW-029; OPS-K-PRIV-1; `docs/DIRECTIVE.md` §6 | Confirm the storage policy text states local-first default and private-data user control. |
> | LFSP-REQ-002 | The public repository shall not be the default durable location for user-private project models, private rule packs, private material libraries, private component libraries, owner standards, credentials, or proprietary data. | OPS-K-IP-1; OPS-K-DATA-1; `docs/IP_AND_DATA_BOUNDARY.md` §§3,6 | Boundary review checks public/private path class separation. |
> | LFSP-REQ-003 | Cloud storage, cloud sync, or cloud service transmission shall be out of MVP unless separately approved by the human project authority through governance or scope change. | SOW-029 notes; `docs/DIRECTIVE.md` §4.2 | Verify no cloud service is assumed in this deliverable's artifacts. |
> | LFSP-REQ-004 | Storage conventions shall align with the architecture persistence baseline: versioned, unit-aware, provenance-preserving, schema-governed, migration-aware, round-trip testable persistence with canonical JSON/JCS-compatible hashes for JSON payloads. | AB-00-04; SOW-050; `execution/_Decomposition/SOFTWARE_DECOMP.md` §8 | Cross-check private path language against the deterministic persistence baseline. |
> | LFSP-REQ-005 | The physical project package/container mechanics, OS-specific roots, and runtime persistence behavior remain deferred even though SCA-003 establishes a local SQLite-backed MVP storage profile. | Acceptance/risk note; AB-00-04; SCA-003 authority reflected in `docs/security/local_first_storage_policy.md` | Verify every package/root/runtime-storage reference preserves the remaining `TBD` decisions and does not treat the metadata-only guard as a storage service. |
> | LFSP-REQ-006 | Path conventions shall use symbolic path classes until implementation chooses OS-specific roots, application data directories, or project package structure. | Acceptance/risk note; setup write-scope constraint | Confirm no real user path, secret path, or environment-specific private data location is created. |
> | LFSP-REQ-007 | Private rule-pack references shall preserve identity, version, checksum, source notice, and redistribution status without bundling protected formulas, code text, allowables, or proprietary values into public artifacts. | OPS-K-RULE-3; `docs/SPEC.md` §6; `docs/IP_AND_DATA_BOUNDARY.md` §§3,6 | Review rule-pack path references for metadata-only public handling. |
> | LFSP-REQ-008 | Material, component, section, rule-pack, report, and project data crossing storage/import/export boundaries shall preserve provenance and redistribution/privacy status. | OPS-K-IP-2; OPS-K-DATA-3; OPS-K-UNIT-1; `docs/TYPES.md` §§7-8 | Future schema and adapter tests check provenance fields and unit-bearing values. |
> | LFSP-REQ-009 | Storage diagnostics and result/report envelopes shall surface private/public boundary warnings without claiming certification, sealing, approval, authentication, or automatic code compliance. | OPS-K-AUTH-1; AB-00-06; `docs/SPEC.md` §§7-8 | Future diagnostics/report tests check warnings and professional-boundary notices. |
> | LFSP-REQ-010 | Adapters, imports, exports, plugins, and private-library access shall not bypass unit checks, provenance checks, diagnostics, sandboxing, or public/private data-boundary controls. | AB-00-02; AB-00-07; OPS-K-PRIV-1 | Future adapter/plugin tests must exercise no-bypass behavior. |
> | LFSP-REQ-011 | Tests for implemented storage behavior shall include private-path resolution, repository-leakage prevention, deterministic round-trip serialization, migration status handling, provenance preservation, and report/export boundary checks. | AB-00-08; `docs/SPEC.md` §§9,11; `TASK_RUN_2026-06-07_0140.md` | Focused metadata-only guard tests now exist for classification and leakage blocking. Runtime storage-service tests, OS-root tests, package/container tests, migration tests, and report/export integration tests remain future work. |
> | LFSP-REQ-012 | Missing storage choices, unresolved roots, and incomplete private-data handling shall be explicit `TBD`, warning, or finding states rather than silent defaults. | OPS-K-DATA-2; OPS-K-AGENT-1; `docs/DIRECTIVE.md` §3 | Review the deliverable for visible TBD/open issue entries. |
>

### CLM-011 — Standards

> ##### Standards
>
> No external engineering code, code clause, standards table, protected formula, material allowable, SIF/flexibility table, protected dimensional table, or proprietary catalog source is used or reproduced by this deliverable.
>
> The controlling project sources for this deliverable are the OpenPipeStress governance and decomposition artifacts listed in `Datasheet.md` and `_REFERENCES.md`, plus the June 7 readiness-evidence run records cited in this alignment.
>

### CLM-012 — Verification

> ##### Verification
>
> | Verification ID | Check | Expected Result |
> |---|---|---|
> | LFSP-VER-001 | Confirm `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` exist. | Four-document kit is present. |
> | LFSP-VER-002 | Validate `Dependencies.csv` with `tools/validation/validate_dependencies_schema.py`. | Schema valid with all 29 v3.1 columns. |
> | LFSP-VER-003 | Confirm `_SEMANTIC.md` audit result is PASS and `_SEMANTIC_LENSING.md` has complete matrix coverage for A, B, C, F, D, X, and E. | Semantic setup gates pass. |
> | LFSP-VER-004 | Confirm June 7 code, policy documentation, and focused tests exist for the metadata-only guard helper. | `core/security/local_first_storage/`, `docs/security/local_first_storage_policy.md`, `tests/security/test_local_first_storage_policy.py`, and `TASK_RUN_2026-06-07_0140.md` are present. |
> | LFSP-VER-005 | Search deliverable and evidence artifacts for disallowed claims or real secret/path examples. | No runtime storage service, real secret, cloud service default, protected data, approval claim, professional claim, code-compliance claim, or security-certification claim is introduced. |
> | LFSP-VER-006 | Confirm this alignment does not edit `_STATUS.md` or promote lifecycle state. | Lifecycle status remains owned by `_STATUS.md` and later human-gated review/transition workflows. |
>

### CLM-013 — Documentation

> ##### Documentation
>
> Required setup and readiness-evidence artifacts:
>
> - `Datasheet.md`;
> - `Specification.md`;
> - `Guidance.md`;
> - `Procedure.md`;
> - `_SEMANTIC.md`;
> - `_SEMANTIC_LENSING.md`;
> - `Dependencies.csv`;
> - `_DEPENDENCIES.md`;
> - `_run_records/*`;
> - `_STATUS.md`.
> - `docs/security/local_first_storage_policy.md`;
> - `core/security/local_first_storage/`;
> - `tests/security/test_local_first_storage_policy.py`.
>
> Implementation artifacts and decisions still deferred:
>
> - runtime storage service behavior;
> - storage schema edits;
> - runtime storage tests beyond the metadata-only guard coverage;
> - physical project package/container mechanics and OS-specific roots;
> - real private paths, secrets, credentials, or private data.
> - cloud exception workflow;
> - encryption, secret storage, and key management;
> - lifecycle acceptance, release approval, or professional/security/code-compliance approval.
>

### CLM-014 — D-41 R5 T4 PDU-036 evidence boundary

> ##### D-41 R5 T4 PDU-036 evidence boundary
>
> The invented DEL-13-04 trace-gap fixture is adjacent evidence for linked omissions/warnings/assumptions only. It does not implement or validate LFSP-REQ-011 runtime private-path resolution, storage package round-trip, migration handling, or report/export boundary integration; those test families remain explicit gaps.

- **AC-001** — The contract preserves the source-defined public/private path classes, local SQLite-backed profile boundary, deterministic persistence and provenance constraints, metadata-only guard evidence, report/export and secret-owner handoffs, visible physical-root/container/runtime/cloud/encryption TBDs and conflict, and the prohibition on treating storage evidence as certification, approval, professional reliance, or code compliance.

## Production and Verification Method — Praxeology

### CLM-015 — Procedure: DEL-12-01 Local-first storage and private data paths

> #### Procedure: DEL-12-01 Local-first storage and private data paths
>

### CLM-016 — Purpose

> ##### Purpose
>
> This procedure describes how to maintain the DEL-12-01 storage-boundary artifact set, align it with implementation evidence, and guide future runtime storage work without crossing the private-data, protected-data, cloud-service-default, secret-handling, or authority-claim boundaries.
>

### CLM-017 — Prerequisites

> ##### Prerequisites
>
> | Prerequisite | Required State |
> |---|---|
> | Sealed deliverable context | DEL-12-01, PKG-12, SOW-029, OBJ-010, explicit write scope |
> | Governance sources | `INIT.md`, `AGENTS.md`, `docs/CONTRACT.md`, `docs/TYPES.md`, `docs/SPEC.md`, and decomposition/register rows read |
> | Architecture basis | AB-00-04 persistence baseline and related AB-00 items injected as constraints, not copied wholesale |
> | Scope boundary | No edits outside this deliverable folder |
> | Protected/private data boundary | No real private values, credentials, protected standards content, or proprietary data introduced |
> | Current metadata-guard evidence | 2026-06-07 run records identify `core/security/local_first_storage/`, `docs/security/local_first_storage_policy.md`, and `tests/security/test_local_first_storage_policy.py` as metadata-only guard evidence |
>

### CLM-018 — Steps

> ##### Steps
>
> | Step | Action | Output |
> |---|---|---|
> | 1 | Confirm DEL-12-01 identity, scope, objective, invariants, and write scope. | `_CONTEXT.md` remains the scope anchor. |
> | 2 | Classify storage-relevant private data classes: project models, private rule packs, material data, component data, owner/code data, report exports. | `Datasheet.md` private data class table. |
> | 3 | Define symbolic path classes rather than physical directories or package/container choices. | `Datasheet.md` symbolic path class table. |
> | 4 | Translate local-first and private-boundary constraints into requirements. | `Specification.md` LFSP requirements and verification table. |
> | 5 | Record implementation guidance, trade-offs, and open issues without deciding the physical container. | `Guidance.md` principles, trade-offs, open issues, and conflict table. |
> | 6 | Build semantic matrix and lensing artifacts after the four documents exist. | `_SEMANTIC.md` and `_SEMANTIC_LENSING.md`. |
> | 7 | Apply P3 lensing by surfacing warranted TBDs or gaps in the four documents only when source evidence supports the edit. | Open issues and verification gaps are visible. |
> | 8 | Extract dependency register rows for anchors and explicit execution information flow. | `Dependencies.csv` and `_DEPENDENCIES.md`. |
> | 9 | Run validation checks and update `_STATUS.md` to `SEMANTIC_READY` only if setup gates pass. | Final status and run records. |
> | 10 | For later readiness-evidence alignment, replace stale setup-only language only where run records, product docs, code, or tests prove new evidence exists. | The four documents distinguish current metadata-only guard evidence from deferred runtime storage and approval decisions. |
>

### CLM-019 — Future Implementation Procedure

> ##### Future Implementation Procedure
>
> When a later implementation task is authorized, it should:
>
> 1. Preserve the symbolic path classes unless a human-approved design decision replaces them.
> 2. Select OS-specific roots and/or physical project packaging only through the appropriate architecture or storage deliverable.
> 3. Keep private data outside default public repository paths.
> 4. Preserve the existing metadata-only guard tests and add repo-leakage, path-resolution, serialization round-trip, provenance, migration, and report/export boundary tests for any future runtime storage behavior.
> 5. Verify no plugin, adapter, import, export, or private library path bypasses validation, units, provenance, diagnostics, sandboxing, or public/private boundary controls.
> 6. Preserve `TBD` or warning status for unresolved storage decisions.
>

### CLM-020 — Verification

> ##### Verification
>
> | Check | Method | Expected Result |
> |---|---|---|
> | Four-document presence | Confirm `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` exist. | Present. |
> | Dependency schema | Run `python3 tools/validation/validate_dependencies_schema.py <deliverable>/Dependencies.csv`. | Valid v3.1 schema. |
> | Enum spot checks | Run `python3 tools/validation/validate_enum.py` against dependency enum values used in the register. | Valid enums. |
> | Semantic audit | Confirm `_SEMANTIC.md` contains `Audit: PASS`. | PASS. |
> | Lensing coverage | Count `_SEMANTIC_LENSING.md` lens rows for matrices A, B, C, F, D, X, and E. | 96 rows. |
> | June 7 metadata-guard evidence | Confirm `core/security/local_first_storage/`, `docs/security/local_first_storage_policy.md`, `tests/security/test_local_first_storage_policy.py`, `TASK_RUN_2026-06-07_0140.md`, and package fan-in evidence exist. | Present; evidence is metadata-only and side-effect-free. |
> | Boundary scan | Search deliverable files for real secrets, hidden cloud defaults, protected standards content, approval claims, professional claims, code-compliance claims, or security-certification claims. | No disallowed content found. |
> | Lifecycle status | Read `_STATUS.md` without editing it. | This alignment does not promote lifecycle state; later lifecycle transitions require human-gated workflow authority. |
>

### CLM-021 — Records

> ##### Records
>
> The original setup workflow left these records in the deliverable folder:
>
> - four production documents;
> - `_SEMANTIC.md`;
> - `_SEMANTIC_LENSING.md`;
> - `Dependencies.csv`;
> - `_DEPENDENCIES.md`;
> - `_run_records/*`;
> - `_STATUS.md`.
>
> The 2026-06-07 readiness evidence also cites product-level artifacts outside this deliverable folder:
>
> - `docs/security/local_first_storage_policy.md`;
> - `core/security/local_first_storage/`;
> - `tests/security/test_local_first_storage_policy.py`.
>
> Do not move any artifact to `ISSUED` during setup or readiness-evidence alignment. Do not use this procedure to approve runtime storage, cloud behavior, encryption/key management, real secret storage, professional reliance, code compliance, security certification, or release readiness.
>
> For PDU-036, classify the transform trace-gap fixture as adjacent verification evidence only. Re-run LFSP-REQ-011 only when the actual runtime path, package/migration, and report/export owners supply their bounded implementations and fixtures.

- **VER-001** — Validate the contract and review source parity, symbolic-path and private-data-class coverage, local-first and deterministic-persistence boundaries, metadata-only guard limits, report/export and secret-owner interfaces, retained conflict and TBDs, and professional-authority limits.

## Governing Values and Decisions — Axiology

### CLM-022 — Guidance: DEL-12-01 Local-first storage and private data paths

> #### Guidance: DEL-12-01 Local-first storage and private data paths
>

### CLM-023 — Purpose

> ##### Purpose
>
> This deliverable keeps OpenPipeStress private-data handling aligned with the product stance: open mechanics and public schemas, but user-controlled private project, rule-pack, material, component, owner, and code/design-basis data.
>
> The guidance remains policy and evidence level. June 7 evidence adds a metadata-only guard helper and tests, but this deliverable still does not select OS-specific roots, create real private paths, store payloads or secrets, implement encryption/key management, or close runtime storage behavior.
>

### CLM-024 — Principles

> ##### Principles
>
> | Principle | Guidance |
> |---|---|
> | Local-first default | Assume private engineering data remains on user-controlled local storage unless a later approved scope change says otherwise. |
> | Public/private separation | Public repository content may include code, schemas, blank templates, import mechanisms, and invented examples; private rule packs, libraries, owner standards, and project models stay outside public paths by default. |
> | Symbolic before physical | Use symbolic path classes in planning. SCA-003 establishes a local SQLite-backed MVP storage profile, but OS-specific roots, runtime persistence behavior, and package/container mechanics remain deferred. |
> | Persistence compatibility | Do not define storage conventions that conflict with versioned schema governance, migration status, canonical JSON/JCS-compatible hashing, provenance, unit awareness, or round-trip reproducibility. |
> | No silent defaults | Unknown roots, package choices, privacy statuses, or provenance fields remain `TBD` or warnings. |
> | No-bypass adapters | Import/export, plugins, and private library adapters must still pass through validation, unit, provenance, diagnostic, and public/private boundary controls. |
> | Human authority | Storage controls and reports support review; they do not certify, seal, approve, authenticate, or declare code compliance. |
>

### CLM-025 — Considerations

> ##### Considerations
>

### CLM-026 — Private Data Classes

> ###### Private Data Classes
>
> Treat these as private by default unless the user intentionally contributes or exports them with documented redistribution rights:
>
> - project models and project-local assets;
> - private rule packs and owner design bases;
> - material libraries and allowable-like values;
> - component libraries, manufacturer/vendor data, geometry catalogs, and stiffness data;
> - code-specific formulas, interpretations, load combinations, SIFs, flexibility factors, and protected standards-derived content;
> - report exports containing private engineering values.
>

### CLM-027 — Path Boundary

> ###### Path Boundary
>
> Prefer planning language like `USER_PRIVATE_LIBRARY_ROOT` and `USER_CHOSEN_PROJECT_PATH` instead of real filesystem examples. Real examples can accidentally normalize a repository path, a cloud-synced folder, or a user-specific secret-bearing directory as the expected pattern.
>

### CLM-028 — Report and Export Boundary

> ###### Report and Export Boundary
>
> Local report/export destinations are still private-data risk surfaces. DEL-12-01 defines path and storage posture; DEL-12-02 owns redaction/export safeguards and should consume the path-class vocabulary from this deliverable.
>

### CLM-029 — Private Library and Secret Boundary

> ###### Private Library and Secret Boundary
>
> DEL-12-04 owns secret and private-library handling. DEL-12-01 should not define credential storage, secret material, encryption, key management, or concrete private-library registry behavior beyond the local-first and public/private path boundary. The 2026-06-07 DEL-12-04 guard alignment is downstream evidence, not authority for DEL-12-01 to select real secret handling.
>

### CLM-030 — Metadata-Only Guard Boundary

> ###### Metadata-Only Guard Boundary
>
> The June 7 `core.security.local_first_storage` helper is a metadata-only guard. It classifies explicit records, blocks unsafe public/shared or remote/direct-SQL/secret/payload markers, and returns safe metadata. It is not a runtime storage service, schema migration layer, physical package/container implementation, cloud exception mechanism, encryption feature, key-management feature, or approval workflow.
>

### CLM-031 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Implication |
> |---|---|
> | User-chosen project path vs. app-managed data directory | User choice improves transparency; app-managed roots can reduce leakage risk. The project has not selected final OS roots or application data directories. |
> | Local SQLite profile vs. final package mechanics | SCA-003 resolves the MVP storage profile as local SQLite-backed, but package/container mechanics, sidecar handling, and user-visible root behavior remain TBD. |
> | Single project package vs. separate private library roots | A package can simplify portability; separate roots can reduce accidental sharing. The detailed package/container mechanics remain TBD. |
> | Checksums in public manifests vs. private metadata | Hashes support reproducibility and tamper detection, but manifests must not reveal private values or protected source text. |
> | Local-first default vs. future collaboration | MVP excludes cloud services unless approved; any later collaboration feature must preserve explicit user intent and no private data transmission by default. |
>

### CLM-032 — Examples

> ##### Examples
>
> The following are symbolic examples only:
>
> | Scenario | Acceptable Planning Expression | Avoid |
> |---|---|---|
> | User project model | `USER_CHOSEN_PROJECT_PATH` plus deterministic project manifest | A real path under this repository or a cloud-synced default |
> | Private rule pack | `USER_PRIVATE_LIBRARY_ROOT/rule_packs/<private-id>` as a symbolic class | A bundled public rule-pack file copied from a protected source |
> | Material library | Private library reference with provenance and redistribution status | Public material allowable table without documented rights |
> | Report export | User-selected local export target with future redaction controls | Public report template containing protected formulas or private values |
>

### CLM-033 — Open Issues and TBDs

> ##### Open Issues and TBDs
>
> | Issue ID | Topic | Status | Notes |
> |---|---|---|---|
> | LFSP-OI-001 | Physical project package/container mechanics | TBD | SCA-003 establishes the local SQLite-backed MVP storage profile; exact package/container mechanics, sidecar behavior, and user-visible root handling remain implementation-level TBD. |
> | LFSP-OI-002 | OS-specific private-data roots | TBD | This run defines symbolic path classes only. |
> | LFSP-OI-003 | Executable storage tests | PARTIAL / TBD | Metadata-only guard tests exist as of 2026-06-07. Runtime storage-service, OS-root, package/container, migration, and integration tests remain future work. |
> | LFSP-OI-004 | Cloud exception approval path | TBD | Cloud services are out of MVP unless separately approved. The exact approval record format is not defined here. |
> | LFSP-OI-005 | Secret storage, encryption, and key management | TBD | Owned by DEL-12-04 and later implementation/governance decisions, not this DEL-12-01 alignment run. |
> | LFSP-OI-006 | Runtime storage service and schema migration behavior | TBD | The metadata-only guard does not read/write project stores, open SQLite handles, or implement migrations. |
> | LFSP-OI-007 | Lifecycle acceptance and approval choices | TBD | Review-readiness evidence is not `ISSUED`, release approval, security certification, professional approval, sealing, authentication, or code-compliance acceptance. |
>

### CLM-034 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | LFSP-CON-001 | The deliverable title implies storage path decisions, while accepted authority only supports a local SQLite-backed MVP storage profile plus metadata-only guard evidence; package mechanics, roots, and runtime behavior remain TBD. | `docs/_Registers/Deliverables.csv` row DEL-12-01; `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 | `docs/security/local_first_storage_policy.md`; `TASK_RUN_2026-06-07_0140.md`; package fan-in `WORKING_ITEMS_RUN_2026-06-07_0150_TP-PKG12-LOCAL-PRIVACY-GUARDS-FANIN.md` | Specification Requirements; Procedure Steps; Open Issues | Preserve symbolic path classes and metadata-only guard evidence; defer physical root/container mechanics and runtime storage decisions. | TBD |
>
> PDU-036 guidance: do not count another deliverable's invented trace-gap fixture as LFSP-REQ-011 runtime-storage validation. Preserve missing owning implementations and test families as explicit gaps.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-029 OBJ-010 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
