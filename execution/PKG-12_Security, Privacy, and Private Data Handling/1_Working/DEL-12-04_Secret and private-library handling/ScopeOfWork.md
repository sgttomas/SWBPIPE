---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-12-04
package_id: PKG-12
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@4d153302c3c4cd42578936db160c2bac1270225a
project_scope_refs: [SOW-040, SOW-029]
package_objective_refs: [OBJ-010]
---

# Scope of Work — DEL-12-04

## Purpose and Objective Traceability

This Scope of Work defines `DEL-12-04` in service of project scope [SOW-040, SOW-029] and package objectives [OBJ-010].

- **OUT-001** — A local-first secret-reference and private-library handling contract for registry metadata, denied-by-default access, release guards, and disclosure boundaries is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-12-04 Secret and private-library handling

> #### Datasheet: DEL-12-04 Secret and private-library handling
>

### CLM-002 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-12-04 |
> | Deliverable name | Secret and private-library handling |
> | Package ID | PKG-12 |
> | Package name | Security, Privacy, and Private Data Handling |
> | Type | SECURITY_CONTROL |
> | Scope items | SOW-040, SOW-029 |
> | Objective | OBJ-010 |
> | Context envelope | M |
> | Lifecycle state | IN_PROGRESS per `_STATUS.md`; June 7 readiness evidence exists but this datasheet does not promote lifecycle state |
>
> Source: `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row `DEL-12-04`; `docs/_Registers/ScopeLedger.csv` rows `SOW-040` and `SOW-029`.
>

### CLM-003 — Attributes

> ##### Attributes
>
> | Attribute | Value | Source |
> |---|---|---|
> | Primary product posture | Local-first; no cloud service required for modeling, solving, rule checking, or reporting | `docs/PRD.md` section 18.1; `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-029 |
> | Private assets in scope | Private rule packs, private material data, private component data, private libraries, project models, paths, and credential references | `docs/PRD.md` sections 17.3 and 18.3; `_CONTEXT.md` Description |
> | Public repository boundary | Public artifacts may define schemas, importers, mechanisms, and invented examples, but must not contain user private rule packs, owner standards, company design bases, protected tables, or proprietary source content | `docs/IP_AND_DATA_BOUNDARY.md` sections 2, 3, and 6 |
> | Registry artifact | Private library registry | `_CONTEXT.md` Anticipated Artifacts |
> | Test artifact | Secret handling tests now have focused invented-fixture coverage in `tests/security/test_secret_private_library_handling.py` | `_CONTEXT.md` Anticipated Artifacts; `_run_records/TASK_RUN_2026-06-07_0942_secret-private-library-alignment.md` |
> | Current product evidence | Metadata-only helper in `core/security/secret_private_library/`, public-boundary documentation in `docs/security/secret_private_library_handling.md`, and focused tests for release guards | `_run_records/TASK_RUN_2026-06-07_0942_secret-private-library-alignment.md`; `WORKING_ITEMS_RUN_2026-06-07_0957_TP-PKG12-REDACTION-SECRET-GUARD-CLOSEOUT.md` |
> | Secret storage posture | Store references to secrets, not secret material, in project, registry, report, or test artifacts | ASSUMPTION from `docs/PRD.md` section 18.3 and `docs/architecture/plugin_boundary.md` Permission Model Skeleton |
> | Hash/checksum posture | Rule packs, libraries, exports, reports, and relevant JSON payloads should be checksum-addressable using the accepted canonical JSON/JCS-compatible basis where applicable | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04; `docs/architecture/plugin_boundary.md` Checksums and Provenance |
>

### CLM-004 — Conditions

> ##### Conditions
>
> | Condition | Required handling |
> |---|---|
> | A private library path is registered | The registry records an opaque path reference, privacy classification, provenance summary, redistribution status, checksum status, and default transmission prohibition. Exact path storage mechanics remain `TBD`. |
> | A credential is needed for import or private storage | The product records an opaque credential reference and required permission, not a credential value. The concrete local secret provider remains `TBD`. |
> | A private library is exported, attached to a bug report, or included in a shared model/report | The product warns, redacts or omits private values unless the user explicitly requests inclusion under their responsibility. |
> | Provenance or redistribution status is missing | The product emits a provenance/private-data diagnostic and does not silently treat the asset as public. |
> | Protected or proprietary content is suspected | Ingestion or publication is stopped, the artifact is quarantined outside public examples, and human/legal review is required. |
> | A plugin or adapter requests private-library access | Access is denied unless a permission grant exists; plugins and adapters cannot bypass schema, unit, provenance, privacy, protected-content, diagnostic, or report controls. |
> | Cloud transmission or telemetry is proposed | It is out of this deliverable unless separately approved; telemetry is off by default and must not include private engineering data. |
> | Runtime provider, storage, or approval mechanics are requested | Exact secret provider, encrypted-storage default, storage roots, permission grant persistence, physical project package/container, public API transport, cloud/network behavior, external secret manager behavior, and approval choices remain `TBD` or deferred to owning workflows. |
>
> Sources: `docs/PRD.md` sections 17.3, 18.2, and 18.3; `docs/IP_AND_DATA_BOUNDARY.md` sections 5 and 6; `docs/architecture/plugin_boundary.md` Private Data Handling; `docs/architecture/extension_domain_contracts.md` Denied-By-Default Behavior.
>

### CLM-005 — Construction

> ##### Construction
>

### CLM-006 — Private Library Registry Record

> ###### Private Library Registry Record
>
> Minimum registry fields for this deliverable are descriptive and implementation-facing, not final schema commitments:
>
> | Field | Purpose | Status |
> |---|---|---|
> | `library_id` | Stable local identifier for a private library record | PROPOSAL |
> | `library_kind` | Private rule pack, material library, component library, owner design basis, or project-local library category | PROPOSAL |
> | `path_ref` | Opaque local path or storage reference; does not expose secret material | PROPOSAL |
> | `privacy_classification` | Marks private/public/export posture | PROPOSAL |
> | `redistribution_status` | Records private-only, public-permissive, unknown, or protected-suspected posture using project vocabulary | PROPOSAL |
> | `source_provenance` | Source/provenance summary without protected tables or formulas | PROPOSAL |
> | `checksum` | Hash/checksum reference when available | PROPOSAL |
> | `credential_ref` | Optional opaque credential reference; never a credential value | PROPOSAL |
> | `default_transmission_allowed` | Defaults to false for private assets | PROPOSAL |
> | `review_status` | Pending, accepted, rejected, quarantined, or `TBD` review disposition | PROPOSAL |
>

### CLM-007 — Current Secret Handling Evidence

> ###### Current Secret Handling Evidence
>
> June 7 evidence records focused invented-fixture tests for current metadata-only guard behavior. These tests do not create real secret storage, runtime private-library storage, cloud/network behavior, external secret-manager integration, legal clearance, security certification, professional approval, or code-compliance approval.
>
> | Test area | Expected evidence |
> |---|---|
> | Registry rejects secret values | Fixture contains only non-secret sentinel markers and opaque references. |
> | Export redaction | Private values are omitted or replaced by non-sensitive markers in report/share paths. |
> | Telemetry exclusion | Private-library and credential-reference fields are not included in telemetry payloads. |
> | Permission denial | Plugin/adapter access to private libraries and secret references fails closed without explicit grant. |
> | Quarantine routing | Suspected protected/private content entering a public contribution path emits a diagnostic and routes to review. |
> | June 7 marker hardening | Tests cover payload markers, cloud/network markers, external secret-manager markers, direct SQL/raw SQLite markers, storage-bypass markers, and concrete path reduction to safe metadata. |
>

### CLM-008 — References

> ##### References
>
> - `_CONTEXT.md` for deliverable identity, scope, anticipated artifacts, and architecture basis injection.
> - `docs/CONTRACT.md` for OPS-K-IP, OPS-K-DATA, OPS-K-AUTH, OPS-K-PRIV, OPS-K-RULE, and OPS-K-AGENT invariants.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 for SOW-029, SOW-040, OBJ-010, PKG-12, and AB-00-01/02/03/04/06/07/08.
> - `docs/_Registers/Deliverables.csv`, `docs/_Registers/ScopeLedger.csv`, and `docs/_Registers/ContextBudgetQA.csv` for machine-readable scope.
> - `docs/PRD.md` sections 12, 13, 15, 17, and 18 for rule-pack, private-library, report, IP, local-first, telemetry, and private-data handling requirements.
> - `docs/IP_AND_DATA_BOUNDARY.md` sections 2 through 7 for public/private data boundary, provenance, quarantine, and report limits.
> - `docs/architecture/plugin_boundary.md`, `docs/architecture/extension_domain_contracts.md`, and `docs/architecture/persistence_contract.md` for architecture-basis constraints on permissions, private-data markers, checksums, and no-bypass behavior.
> - `_run_records/TASK_RUN_2026-06-07_0942_secret-private-library-alignment.md` and package fan-in `WORKING_ITEMS_RUN_2026-06-07_0957_TP-PKG12-REDACTION-SECRET-GUARD-CLOSEOUT.md` for current DEL-12-04 evidence.
> - `docs/security/secret_private_library_handling.md`, `core/security/secret_private_library/`, and `tests/security/test_secret_private_library_handling.py` for current metadata-only helper, documentation, and focused test evidence.

## Completion and Reliance Basis — Epistemology

### CLM-009 — Specification: DEL-12-04 Secret and private-library handling

> #### Specification: DEL-12-04 Secret and private-library handling
>

### CLM-010 — Scope

> ##### Scope
>
> This deliverable specifies a local-first handling surface for private libraries, private paths, and credential references used by imports or private storage. It covers the private library registry concept and secret handling tests listed in `_CONTEXT.md`.
>
> In scope:
>
> - local-first handling for private rule packs, private material libraries, private component libraries, owner design-basis files, and project-local private library references;
> - registry metadata needed to mark privacy, provenance, redistribution status, checksum status, review status, and transmission posture;
> - secret-reference handling for imports or private storage without storing usable secret material in project artifacts, reports, fixtures, or public examples;
> - warning, redaction, quarantine, and deny-by-default behavior at export, report, plugin, adapter, telemetry, and public contribution boundaries.
> - current metadata-only release-guard evidence in `core/security/secret_private_library/`, `docs/security/secret_private_library_handling.md`, and `tests/security/test_secret_private_library_handling.py`.
>
> Out of scope:
>
> - cloud secret management or cloud storage operations unless separately approved;
> - final operating-system credential-store integration details;
> - exact secret provider, encrypted-storage default, storage roots, permission grant persistence, physical project package/container, public API transport, cloud/network behavior, external secret manager behavior, and approval choices;
> - legal sufficiency, certification, approval, sealing, endorsement, or professional code-compliance claims;
> - real private libraries, real credentials, real private project data, protected standards text, protected tables, proprietary formulas, material allowables, SIF/flexibility tables, or protected dimensional data.
>
> Sources: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` PKG-12 and row `DEL-12-04`; `docs/PRD.md` sections 17.3, 18.1, and 18.3.
>

### CLM-011 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source | Verification |
> |---|---|---|---|
> | DEL-12-04-R1 | Private libraries and rule packs shall be stored outside public example directories by default and marked private in metadata. | `docs/PRD.md` section 17.3; `docs/IP_AND_DATA_BOUNDARY.md` section 6 | Registry tests inspect default classification and public-example exclusion behavior. |
> | DEL-12-04-R2 | Private rule packs, component libraries, material data, project files, and calculation results shall not be transmitted by telemetry by default. | `docs/PRD.md` section 18.2; `docs/architecture/plugin_boundary.md` Private Data Handling | Telemetry exclusion tests verify absent private fields. |
> | DEL-12-04-R3 | Private library registry records shall include privacy classification, provenance/source summary, redistribution status, review status, and checksum status where available. | `docs/IP_AND_DATA_BOUNDARY.md` section 4; `docs/PRD.md` sections 13.4 and 18.3; `docs/architecture/plugin_boundary.md` Checksums and Provenance | Registry schema/fixture tests validate required metadata presence without protected content. |
> | DEL-12-04-R4 | Project, registry, report, and test artifacts shall store opaque credential references only and shall not store usable credential values. | ASSUMPTION from `docs/PRD.md` section 18.3 and `docs/architecture/plugin_boundary.md` Permission Model Skeleton | Secret handling tests reject secret-like values and accept non-sensitive reference markers only. |
> | DEL-12-04-R5 | Export, report, issue/bug-report, and shared-model paths shall warn before exposing private data and shall redact or omit protected/private values by default. | `docs/PRD.md` sections 15.2, 17.3, and 18.3; SOW-040 | Export/redaction tests verify default omission and warning diagnostics. |
> | DEL-12-04-R6 | If protected standards data or proprietary source content is suspected in a private-library import or public contribution path, ingestion/publication shall stop and route to quarantine/human review. | `docs/IP_AND_DATA_BOUNDARY.md` section 5; `docs/DIRECTIVE.md` section 5 | Quarantine-routing tests verify diagnostic emission and no public artifact write. |
> | DEL-12-04-R7 | Plugins and adapters shall be denied private-library, private-rule-pack, filesystem, and network access unless an explicit permission grant exists. | `docs/architecture/plugin_boundary.md` Permission Model Skeleton; `docs/architecture/extension_domain_contracts.md` Denied-By-Default Behavior | Permission tests verify fail-closed access behavior. |
> | DEL-12-04-R8 | Plugins, adapters, imports, exports, and private storage shall not bypass schema validation, unit checks, provenance checks, privacy controls, protected-content screening, diagnostics, checksums, or report controls. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-02, AB-00-06, AB-00-07; `docs/architecture/extension_domain_contracts.md` No-Bypass Rules | Boundary tests verify routed control checks and diagnostics. |
> | DEL-12-04-R9 | Reports and exported result payloads may identify private rule packs or libraries by name/version/checksum/source note without exposing protected formulas, private values, or proprietary source content in public templates. | `docs/PRD.md` sections 15.2 and 17.3; `docs/architecture/plugin_boundary.md` Checksums and Provenance | Report/export tests inspect metadata-only references and protected-content omission. |
> | DEL-12-04-R10 | Missing provenance, unknown redistribution status, or uncertain privacy classification shall be surfaced as explicit diagnostics or findings, not silent defaults. | OPS-K-DATA-2/3; `docs/PRD.md` section 13.5; `docs/SPEC.md` section 7 | Validation tests verify `PROVENANCE_WARNING` or `IP_BOUNDARY_WARNING` class behavior. |
>

### CLM-012 — Standards

> ##### Standards
>
> No protected standards text, tables, figures, or proprietary values are needed to execute this deliverable. Governing project references are:
>
> - `docs/CONTRACT.md` invariants OPS-K-IP-1/2/3, OPS-K-DATA-1/2/3, OPS-K-AUTH-1/2, OPS-K-PRIV-1/2, OPS-K-RULE-3, and OPS-K-AGENT-1/2/3/4.
> - `docs/DIRECTIVE.md` non-negotiable product principles and stop rules.
> - `docs/IP_AND_DATA_BOUNDARY.md` public/private data and quarantine policy.
> - `docs/PRD.md` sections 12, 13, 15, 17, and 18.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 architecture basis AB-00-01/02/03/04/06/07/08.
>

### CLM-013 — Verification

> ##### Verification
>
> | Verification item | Acceptance signal |
> |---|---|
> | Four-document source boundary | No real credentials, private libraries, protected standards text, proprietary source content, or usable credential examples appear in deliverable artifacts. |
> | Registry metadata coverage | Current helper/test fixtures cover privacy classification, provenance, redistribution status, checksum status, credential-reference posture, transmission default, and review/quarantine status as metadata-only records; exact registry schema/storage placement remains `TBD`. |
> | Secret-reference safety | Current tests fail when a project or registry artifact contains secret material instead of an opaque reference or non-sensitive sentinel marker. |
> | Local-first default | Modeling, solving, rule checking, reporting, and private-library use require no cloud service by default. |
> | Redaction/export path | June 7 guard evidence aligns with DEL-12-02 redaction/export markers for metadata-only report/shared/downstream release contexts; runtime report/export route integration remains `TBD`. |
> | Permission path | Plugin/adapter access semantics are denied by default in the documented no-bypass control surface; exact permission grant persistence remains `TBD`. |
> | Diagnostic path | Missing provenance, unknown redistribution, suspected protected content, payload markers, cloud/network markers, external secret-manager markers, direct SQL/raw SQLite markers, storage-bypass markers, and concrete path indicators produce structured diagnostics or safe metadata reduction in current focused tests. |
>

### CLM-014 — Documentation

> ##### Documentation
>
> Required deliverable-local artifacts:
>
> - `Datasheet.md`
> - `Specification.md`
> - `Guidance.md`
> - `Procedure.md`
> - `_SEMANTIC.md`
> - `_SEMANTIC_LENSING.md`
> - `Dependencies.csv`
> - `_DEPENDENCIES.md`
> - `_run_records/*` phase records
>
> Current implementation-facing evidence:
>
> - `core/security/secret_private_library/controls.py` and `__init__.py` provide metadata-only reference classification and release-guard helpers;
> - `docs/security/secret_private_library_handling.md` documents guard behavior and non-authority boundaries;
> - `tests/security/test_secret_private_library_handling.py` provides focused invented-fixture coverage;
> - `_run_records/TASK_RUN_2026-06-07_0942_secret-private-library-alignment.md` and package fan-in `WORKING_ITEMS_RUN_2026-06-07_0957_TP-PKG12-REDACTION-SECRET-GUARD-CLOSEOUT.md` record passing focused and paired PKG-12 validation.
>
> Remaining implementation details not resolved by this specification include exact secret provider, encrypted-storage default, storage roots, permission grant persistence, physical project package/container, public API transport, cloud/network behavior, external secret manager behavior, and approval choices.

- **AC-001** — The contract preserves opaque-reference-over-secret behavior, private-library metadata and provenance/checksum posture, redaction/quarantine/permission/no-bypass controls, metadata-only helper and invented-fixture evidence, source-labeled assumptions, visible provider/encryption/storage/grant/cloud/approval TBDs and conflicts, and the separation of guard evidence from legal, security, professional, or code-compliance approval.

## Production and Verification Method — Praxeology

### CLM-015 — Procedure: DEL-12-04 Secret and private-library handling

> #### Procedure: DEL-12-04 Secret and private-library handling
>

### CLM-016 — Purpose

> ##### Purpose
>
> Use this procedure to produce, review, or align DEL-12-04 evidence for private library registry semantics and secret handling tests. The procedure is local-first, source-bound, and limited to the assigned deliverable. June 7 evidence shows metadata-only helper code, public-boundary documentation, and focused tests now exist; this procedure must not treat that evidence as runtime secret storage, cloud behavior, lifecycle acceptance, or approval authority.
>

### CLM-017 — Prerequisites

> ##### Prerequisites
>
> - Read `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, and `_STATUS.md`.
> - Read `INIT.md`, `AGENTS.md`, `docs/CONTRACT.md`, `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7, and the relevant register rows for `DEL-12-04`, `SOW-040`, `SOW-029`, and `OBJ-010`.
> - Confirm write scope is limited to this deliverable folder.
> - Confirm no real secrets, real private libraries, protected standards text, proprietary values, credential examples, or private project data are needed.
> - Treat exact secret provider, encrypted-storage default, storage roots, permission grant persistence, physical project package/container, public API transport, cloud/network behavior, external secret manager behavior, and approval choices as `TBD` unless a later human-approved brief resolves them.
> - For readiness-evidence alignment, read `_run_records/TASK_RUN_2026-06-07_0942_secret-private-library-alignment.md`, package fan-in `WORKING_ITEMS_RUN_2026-06-07_0957_TP-PKG12-REDACTION-SECRET-GUARD-CLOSEOUT.md`, `docs/security/secret_private_library_handling.md`, `core/security/secret_private_library/`, and `tests/security/test_secret_private_library_handling.py` as current evidence.
>
> Sources: TASK brief; `_CONTEXT.md`; `docs/DIRECTIVE.md` section 5; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.
>

### CLM-018 — Steps

> ##### Steps
>
> 1. Establish the private asset boundary.
>    - Use SOW-029 and SOW-040 to identify protected assets: private rule packs, private material data, private component data, private library references, project models, private paths, and credential references.
>    - Verify that no cloud assumption is introduced.
>
> 2. Define or verify the registry metadata surface.
>    - Record privacy classification, provenance/source summary, redistribution status, review status, checksum status, path reference, credential reference if needed, and default transmission posture.
>    - Mark exact schema file placement and storage-provider mechanics as `TBD` unless separately approved.
>
> 3. Define or verify secret-reference behavior.
>    - Persist only opaque credential references or non-sensitive test markers.
>    - Reject or flag artifacts that appear to contain usable credential material.
>    - Do not include credential-like examples in docs, fixtures, reports, or run records.
>
> 4. Define boundary controls.
>    - For import, export, report, bug-report, telemetry, plugin, adapter, and public contribution paths, require explicit privacy/provenance checks.
>    - Default to warning, redaction, omission, denied access, quarantine, or human review when classification is missing or suspicious.
>
> 5. Define or verify tests.
>    - Current focused tests cover metadata completeness, no-secret persistence, redaction/export behavior, telemetry exclusion markers, denied-by-default private/reference release contexts, payload marker blocking, cloud/network marker blocking, external secret-manager marker blocking, direct SQL/raw SQLite marker blocking, storage-bypass marker blocking, concrete path safe metadata reduction, and protected/private-content source boundaries.
>    - Use non-sensitive sentinel markers only.
>
> 6. Check no-bypass constraints.
>    - Confirm adapters/plugins cannot skip schema validation, units, provenance, privacy, protected-content screening, diagnostics, checksums, rule sandboxing, or report controls.
>
> 7. Record unresolved human rulings.
>    - If provider selection, encryption default, permission grant persistence, or transport behavior is required, record it as `TBD` for a future human-approved implementation brief.
>
> 8. Finalize deliverable-local evidence artifacts.
>    - Ensure four documents exist and keep default sections.
>    - Ensure `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, `Dependencies.csv`, and `_DEPENDENCIES.md` are refreshed.
>    - Treat `_STATUS.md` as read-only unless a human-gated lifecycle workflow explicitly authorizes status changes; current `_STATUS.md` records `IN_PROGRESS`.
>    - Add phase run records under `_run_records`.
>

### CLM-019 — Verification

> ##### Verification
>
> | Check | Pass condition |
> |---|---|
> | Scope containment | All changed files are inside the DEL-12-04 folder. |
> | Four-document sections | Datasheet, Specification, Guidance, and Procedure preserve their default sections. |
> | Source boundary | No real secrets, real private libraries, private project data, protected standards content, or usable credential examples are present. |
> | Current helper/docs/tests evidence | June 7 run records cite `core/security/secret_private_library/`, `docs/security/secret_private_library_handling.md`, and `tests/security/test_secret_private_library_handling.py`; focused tests passed. |
> | Semantic setup evidence | `_SEMANTIC.md` has complete final result tables without matrix errors or operator leakage in final cells. |
> | Lensing setup evidence | `_SEMANTIC_LENSING.md` includes A/B/C/F/D/X/E coverage. |
> | Dependency schema | `python3 tools/validation/validate_dependencies_schema.py <ScopePath>/Dependencies.csv` returns valid. |
> | Status boundary | `_STATUS.md` is not edited by this procedure; current state is `IN_PROGRESS`, and any lifecycle transition remains human-gated. |
>

### CLM-020 — Records

> ##### Records
>
> - `Datasheet.md`
> - `Specification.md`
> - `Guidance.md`
> - `Procedure.md`
> - `_SEMANTIC.md`
> - `_SEMANTIC_LENSING.md`
> - `Dependencies.csv`
> - `_DEPENDENCIES.md`
> - `_STATUS.md`
> - `_run_records/TASK_RUN_2026-04-30_1430_four-documents-P1_P2.md`
> - `_run_records/TASK_RUN_2026-04-30_1431_semantic-matrix-build.md`
> - `_run_records/TASK_RUN_2026-04-30_1432_lens-register.md`
> - `_run_records/TASK_RUN_2026-04-30_1433_four-documents-P3_ONLY.md`
> - `_run_records/TASK_RUN_2026-04-30_1434_dependency-extract.md`
> - `_run_records/TASK_RUN_2026-04-30_1435_validation.md`
> - `_run_records/TASK_RUN_2026-06-07_0942_secret-private-library-alignment.md`
> - package fan-in `../_run_records/WORKING_ITEMS_RUN_2026-06-07_0957_TP-PKG12-REDACTION-SECRET-GUARD-CLOSEOUT.md`

- **VER-001** — Validate the contract and review source parity, registry and opaque-reference coverage, privacy/provenance/checksum and transmission defaults, redaction/quarantine/permission/no-bypass controls, metadata-only evidence limits, retained assumptions/conflicts/TBDs, and authority boundaries.

## Governing Values and Decisions — Axiology

### CLM-021 — Guidance: DEL-12-04 Secret and private-library handling

> #### Guidance: DEL-12-04 Secret and private-library handling
>

### CLM-022 — Purpose

> ##### Purpose
>
> This deliverable gives downstream implementation work a bounded, local-first privacy control surface for private libraries and credential references. It exists so OpenPipeStress can support user-owned rule packs, material libraries, component libraries, owner design bases, and project models without treating private or protected content as public repository data.
>
> Sources: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-029, SOW-040, and OBJ-010; `docs/PRD.md` sections 17.3 and 18.3.
>

### CLM-023 — Current Evidence Boundary

> ##### Current Evidence Boundary
>
> June 7 evidence shows DEL-12-04 now has a metadata-only helper, public-boundary documentation, and focused invented-fixture tests:
>
> - `core/security/secret_private_library/` classifies private-library, private-path, and credential-reference metadata without reading referenced files or storing payloads.
> - `docs/security/secret_private_library_handling.md` records metadata-only guard behavior and non-authority boundaries.
> - `tests/security/test_secret_private_library_handling.py` covers private reference metadata, public fixture blocking, redaction/export release contexts, telemetry exclusion markers, concrete path reduction, cloud/network markers, external secret-manager markers, direct SQL/raw SQLite markers, and storage-bypass markers.
> - `_run_records/TASK_RUN_2026-06-07_0942_secret-private-library-alignment.md` and package fan-in `WORKING_ITEMS_RUN_2026-06-07_0957_TP-PKG12-REDACTION-SECRET-GUARD-CLOSEOUT.md` record passing focused and paired PKG-12 validation.
>
> This evidence does not finalize exact secret provider, encrypted-storage default, storage roots, permission grant persistence, physical project package/container, public API transport, cloud/network behavior, external secret manager behavior, approval choices, legal sufficiency, security certification, professional approval, or code-compliance status.
>

### CLM-024 — Principles

> ##### Principles
>
> | Principle | Guidance | Source |
> |---|---|---|
> | Local-first by default | Private-library handling should not require cloud operation, cloud storage, telemetry, or external network access. | `docs/PRD.md` section 18.1 |
> | References over secrets | Store opaque references to credentials and private library locations, not usable secret material. | ASSUMPTION from `docs/PRD.md` section 18.3 and plugin permission controls |
> | Private unless proven public | Unknown provenance or redistribution status should not be promoted to public-safe. | `docs/IP_AND_DATA_BOUNDARY.md` sections 4 and 6 |
> | Fail closed at boundaries | Imports, exports, report generation, bug reports, telemetry, plugin access, and public contribution flows should block, redact, warn, or quarantine when privacy status is uncertain. | `docs/IP_AND_DATA_BOUNDARY.md` section 5; `docs/architecture/plugin_boundary.md` Private Data Handling |
> | Metadata without disclosure | Reports and exported payloads may reference private rule packs or libraries by identity/version/checksum/source note without exposing protected formulas or private values. | `docs/PRD.md` sections 15.2 and 17.3 |
> | No-bypass architecture | Adapters and plugins are translators; they cannot bypass validation, provenance, privacy, diagnostics, checksums, rule sandboxing, or report controls. | AB-00-02, AB-00-06, AB-00-07; `docs/architecture/extension_domain_contracts.md` No-Bypass Rules |
>

### CLM-025 — Considerations

> ##### Considerations
>
> - A private library registry should be treated as a control surface, not a data dump. It should carry enough metadata to validate, warn, hash, redact, and route assets without exposing private values.
> - The exact physical project package/container and local secret provider remain `TBD`; this deliverable should not lock the product to one platform mechanism without human approval.
> - Current and future secret handling tests should use non-sensitive sentinel markers and fixture-only paths, not realistic tokens, passwords, API keys, owner data, material tables, or private component values.
> - Optional encrypted storage for private libraries is called out by the PRD but not fully specified. Treat encryption defaults, key lifecycle, recovery, and platform storage as human/architecture decisions still requiring a later implementation brief.
> - A checksum identifies a referenced artifact or payload. It is not permission to expose the artifact and does not make protected/private content public.
> - Public examples may use invented data only. They should not be made more realistic by copying standards tables, proprietary formulas, commercial examples, or private owner data.
>

### CLM-026 — Trade-offs

> ##### Trade-offs
>
> | Tension | Conservative posture |
> |---|---|
> | Detailed registry fields vs. unresolved schema layout | Specify required metadata semantics now; leave exact schema file placement and code-generation details `TBD`. |
> | User convenience vs. accidental disclosure | Prefer explicit warning, redaction, quarantine, or permission grant over silent inclusion. |
> | Report reproducibility vs. private-content exposure | Include identity/version/checksum/source-note references; omit formulas, protected values, and private library contents by default. |
> | Plugin extensibility vs. privacy boundary | Use denied-by-default permissions and no-bypass rules. |
> | Local encryption expectations vs. platform variability | Record optional encrypted storage as required consideration, not as a completed design. |
> | Current metadata-only evidence vs. runtime storage authority | Treat the helper/tests as readiness evidence for guard semantics only; do not infer runtime storage roots, secret provider, permission grant persistence, cloud/network behavior, or approval authority. |
>

### CLM-027 — Examples

> ##### Examples
>
> The following are non-sensitive behavior examples only. They intentionally avoid real credentials, real private paths, real project data, and protected standards content.
>
> | Scenario | Expected behavior |
> |---|---|
> | A user registers a private material library | The registry marks it private, records provenance and redistribution status, stores an opaque local reference, and prevents default telemetry/export inclusion. |
> | A private rule pack is referenced in a report | The report can show rule-pack identity, version, checksum, source note, warnings, and missing-input findings without embedding protected formulas or private values. |
> | A plugin requests private-library read access | The request is denied unless a grant exists; denial emits a diagnostic instead of bypassing the application service boundary. |
> | Imported data appears to be copied protected content | Import/publication stops, the record is marked suspected, and human review is required before public use. |
> | A test fixture needs credential behavior | The fixture uses a non-sensitive placeholder reference and asserts that no usable secret value is persisted or emitted. |
>

### CLM-028 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | DEL-12-04-CF-001 | Exact local secret provider and encrypted-storage default are not specified. | `docs/PRD.md` section 18.3 | `docs/architecture/plugin_boundary.md` Remaining TBDs | Specification Requirements; Procedure Steps | Keep provider/encryption mechanics `TBD`; require opaque secret references and no real secrets in artifacts now. | TBD |
> | DEL-12-04-CF-002 | Current metadata-only helper/tests exist, but runtime storage roots, permission grant persistence, physical project package/container, public API transport, cloud/network behavior, external secret manager behavior, and approval choices remain unresolved. | June 7 DEL-12-04 TASK run and package fan-in | `_CONTEXT.md` architecture-basis `Still TBD`; `docs/IP_AND_DATA_BOUNDARY.md` private user data policy | Specification Verification; Procedure Verification; Dependencies | Treat current controls as readiness evidence only and carry unresolved mechanics as non-blocking deferrals for owning workflows. | TBD |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-040 SOW-029 OBJ-010 | CLM-009 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
