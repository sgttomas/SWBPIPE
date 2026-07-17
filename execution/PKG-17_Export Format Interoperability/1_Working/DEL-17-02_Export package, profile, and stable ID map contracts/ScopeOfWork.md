---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-17-02
package_id: PKG-17
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@e8f59a63372f38d9e788ac39b39995558f5aba73
project_scope_refs: [SOW-030, SOW-074]
package_objective_refs: [OBJ-009, OBJ-017]
---

# Scope of Work — DEL-17-02

## Purpose and Objective Traceability

This Scope of Work defines `DEL-17-02` in service of project scope [SOW-030, SOW-074] and package objectives [OBJ-009, OBJ-017].

- **OUT-001** — A schema-first common export-package, target-profile, stable-ID-map, manifest, and loss-report contract for deterministic interoperability is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-17-02 Export package, profile, and stable ID map contracts

> #### Datasheet: DEL-17-02 Export package, profile, and stable ID map contracts
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-17-02-DECL-002`.
>

### CLM-003 — Identity

> ##### Identity
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-17-02 |
> | Package | PKG-17 Export Format Interoperability |
> | Type | API_CONTRACT |
> | Lifecycle state after this tranche | SEMANTIC_READY |
> | Source basis | DEL-17-01 source-basis dossier and admitted references |
> | Downstream consumers | DEL-17-03 through DEL-17-09 |
>

### CLM-004 — Purpose

> ##### Purpose
>
> DEL-17-02 defines the common export contract that later PKG-17 target exporters shall consume before writing target-specific formats. It covers export package structure, export profile metadata, stable ID maps, export manifests, loss reports, and unsupported/approximated/delegated behavior records.
>
> This deliverable is a contract-level planning artifact. It does not implement JSON schemas, Rust modules, exporters, parsers, harnesses, GUI behavior, persistence runtime, public API endpoints, or target-specific file writers.
>

### CLM-005 — Contract Objects

> ##### Contract Objects
>
> | Object | Role | Required downstream behavior |
> |---|---|---|
> | Export package | Binds payloads, target files, manifests, ID maps, diagnostics, and loss reports into one reproducible package concept. | Every exporter shall identify which package members it emits and which are outside its tranche. |
> | Export profile | Declares target name, profile version, target version basis, unit policy, coordinate policy, entity coverage, and target-specific TBDs. | Target exporters shall consume a profile rather than encode hidden target assumptions. |
> | Stable ID map | Maps OpenPipeStress canonical IDs to target record IDs, file names, sidecars, metadata slots, or omitted-target entries. | Every target artifact shall preserve identity directly or through a sidecar map. |
> | Export manifest | Records source model identity, profile identity, package members, hashes where applicable, diagnostics, and boundary notes. | Export packages shall be auditable without relying on unstated runtime context. |
> | Loss report | Records exported, omitted, approximated, delegated, unsupported, and TBD behavior. | Exporters shall not silently approximate unsupported target concepts. |
>

### CLM-006 — Source Consumption

> ##### Source Consumption
>
> | Source | Consumption rule |
> |---|---|
> | DEL-17-01 `Source_Basis_Register.md` | Upstream source-basis authority for admitted public/project source IDs, findings, boundaries, and TBD register. SourceRef: `Source_Basis_Register.md#Public and Official Source Evidence`, `#Accepted Project References`, `#TBD Register`. |
> | DEL-17-01 `CAEPIPE_Question_Dossier.md` | Upstream question register for unresolved CAEPIPE claims. SourceRef: `CAEPIPE_Question_Dossier.md#Question Register`, `#Question Boundary`, `#Closure Rule`. |
> | SCA-004 and DAG-005 | Establish PKG-17 scope and sequencing authority. |
> | `docs/CONTRACT.md` and `docs/IP_AND_DATA_BOUNDARY.md` | Govern protected-content, professional-boundary, and no-bypass constraints. |
> | `schemas/model.schema.yaml` | Supplies canonical model identity vocabulary for mapping, without changing the schema in this tranche. |
>

### CLM-007 — DEL-17-01 Source IDs Carried Forward

> ##### DEL-17-01 Source IDs Carried Forward
>
> | Source ID | DEL-17-02 use | Location basis |
> |---|---|---|
> | CAEPIPE-IMPORT-MBF | MBF text model input and command-line planning evidence for later CAEPIPE profiles. | `Source_Basis_Register.md#Public and Official Source Evidence`; CAEPIPE `IMPORT MBF`, `Command line operation`, `Format of .MBF`. |
> | CAEPIPE-EXPORT-DATA | CAEPIPE model/result export-surface evidence for later CSV/text parser scoping. | `Source_Basis_Register.md#Public and Official Source Evidence`; CAEPIPE `Export data from CAEPIPE`. |
> | CAEPIPE-EXPORT-MBF | CAEPIPE MBF export evidence for later MBF profile/version discussion. | `Source_Basis_Register.md#Public and Official Source Evidence`; CAEPIPE `EXPORT MBF`. |
> | CAEPIPE-BATCH | Optional external-run harness evidence, limited to user-owned licensed execution planning. | `Source_Basis_Register.md#Public and Official Source Evidence`; CAEPIPE batch `RUNNING CAEPIPE/CAEPIPE 3D+ IN BATCH MODE`. |
> | CAEPIPE-PCF | PCF translator planning evidence with mapping/default caveats for conservative downstream subset work. | `Source_Basis_Register.md#Public and Official Source Evidence`; PCF PDF `Reference`, `PCF to CAEPIPE component Mapping`. |
> | GLTF-2.0 | Review-geometry package evidence for glTF/GLB asset, identity, coordinate, and container semantics. | `Source_Basis_Register.md#Public and Official Source Evidence`; glTF `2.4`, `3.2`, `3.3`, `3.4`, `4`. |
> | PLAN-EXPORT-INTEROP | Strategy and priority basis only; section location remains `location TBD`. | `Source_Basis_Register.md#Accepted Project References`; `location TBD within plan`. |
> | CONTRACT / IP-DATA / SPEC / TYPES / DAG-005 | Governance, workflow, lifecycle, provenance, TBD, data-boundary, and active graph-coordination basis. | `Source_Basis_Register.md#Accepted Project References`; local governance files and DAG-006 approval record locations cited there. |
>

### CLM-008 — Local Contract Artifact Inventory

> ##### Local Contract Artifact Inventory
>
> | Artifact | Contract role |
> |---|---|
> | `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md` | Human-readable contract kit for the export package/profile/stable-ID/loss-report surface. |
> | `_SEMANTIC.md` | Semantic matrix lens for coverage review; not an engineering authority. |
> | `_SEMANTIC_LENSING.md` | Warranted enrichment register applied by Pass 3; not an authority source by itself. |
> | `Dependencies.csv` | v3.1 dependency register tying DEL-17-02 to upstream source-basis evidence and downstream PKG-17 consumers. |
> | `_run_records/` | Auditable TASK execution records for sealed population and validation passes. |
>

### CLM-009 — Required Export-Profile Fields

> ##### Required Export-Profile Fields
>
> | Field | Meaning |
> |---|---|
> | `profile_id` | Stable identifier for the export profile. |
> | `profile_version` | Version of the OpenPipeStress profile contract. |
> | `target_family` | Target family such as native JSON, CAEPIPE MBF, stress-neutral CSV/JSON, PCF, GLB/glTF, or adapter SDK target. |
> | `target_version_basis` | Cited target version or `TBD` when not yet source-confirmed. |
> | `source_basis_ids` | DEL-17-01 source IDs and project references used by the profile. |
> | `unit_policy` | Declared output units and conversion responsibility. |
> | `coordinate_policy` | Declared coordinate frame, axis convention, and origin policy. |
> | `entity_coverage` | Supported, omitted, approximated, delegated, unsupported, and TBD entity families. |
> | `stable_id_policy` | Direct ID, sidecar ID, metadata ID, or omitted ID behavior. |
> | `loss_report_policy` | Required loss-report categories and blocking diagnostics. |
> | `external_execution_policy` | Optional user-owned harness metadata only, when applicable. |
> | `boundary_notes` | IP, license, private-data, and professional-boundary constraints. |
>

### CLM-010 — Stable ID Families

> ##### Stable ID Families
>
> The common ID map shall account for at least:
>
> - project and model IDs;
> - line, node, element, component, support, material, section, load case, result case, analysis run, and export IDs;
> - target file IDs, target record IDs, target metadata keys, target sidecar IDs, and omitted-target placeholders.
>
> If a target cannot carry a canonical ID directly, the exporter shall record the mapping in a sidecar and reference that sidecar from the manifest.
>

### CLM-011 — Stable ID Carrier Modes

> ##### Stable ID Carrier Modes
>
> | Mode | Required meaning | DEL-17-02 boundary |
> |---|---|---|
> | Direct target-carried ID | The target artifact contains the canonical OpenPipeStress ID in a source-confirmed target field, metadata slot, name, or equivalent target-carried location. | Allowed only when the downstream target profile cites admitted evidence for the carrier. |
> | Metadata-carried ID | The target artifact carries canonical IDs through a documented metadata surface. | Allowed only when the target metadata behavior is source-confirmed and does not imply solver equivalence. |
> | Sidecar mapping | The target artifact does not carry the canonical ID directly; the export package includes a separate ID-map member referenced from the manifest. | Required fallback whenever direct or metadata carriage is unsupported, ambiguous, or `TBD`. |
> | Omitted-target entry | A canonical ID has no target artifact representation. | Must appear in the ID map and loss report with category, reason, and affected canonical IDs. |
>
> For CAEPIPE MBF, direct target-carried stable ID claims remain blocked by `TBD-17-01-003`. DEL-17-02 permits sidecar-only MBF mapping as a conservative contract mode, but it does not decide the downstream MBF carrier policy.
>

### CLM-012 — Loss Categories

> ##### Loss Categories
>
> | Category | Meaning |
> |---|---|
> | `exported` | Represented directly in the target package. |
> | `omitted` | Not exported, with explicit reason and affected canonical IDs. |
> | `approximated` | Represented by a declared approximation with source and target IDs. |
> | `delegated` | Passed through to a target tool or external workflow without local interpretation. |
> | `unsupported` | Known unsupported behavior that blocks or limits target use. |
> | `tbd` | Behavior not source-confirmed; shall not be represented as supported. |
>
> Each category shall be visible in the export manifest or loss report even when the export completes. `delegated` means passed through to target configuration or a user-owned external workflow without local interpretation; it does not admit local code-checking logic or bundled target execution.
>

### CLM-013 — DEL-17-01 TBD Carryforward

> ##### DEL-17-01 TBD Carryforward
>
> | TBD ID | DEL-17-02 treatment | Downstream handoff |
> |---|---|---|
> | TBD-17-01-001 | Preserve target version/profile as `TBD` in export profiles until CAEPIPE version/profile authority is recorded. | DEL-17-04, DEL-17-05 |
> | TBD-17-01-002 | Preserve first MBF record subset as `TBD`; do not encode a first writer subset here. | DEL-17-04 |
> | TBD-17-01-003 | Preserve MBF direct carrier versus sidecar-only stable-ID policy as `TBD`. | DEL-17-02, DEL-17-04 |
> | TBD-17-01-004 | Preserve CSV result-section parser scope as `TBD`. | DEL-17-05, DEL-17-06 |
> | TBD-17-01-005 | Preserve conservative PCF subset and default-warning policy as `TBD`. | DEL-17-07 |
> | TBD-17-01-006 | Preserve glTF/GLB review-geometry identity metadata and sidecar policy as `TBD`. | DEL-17-08 |
>

### CLM-014 — Boundary Summary

> ##### Boundary Summary
>
> - No bundled commercial solver or target executable.
> - No proprietary examples, private target data, protected standards tables, material allowables, SIF/flexibility values, or owner criteria.
> - No reverse engineering, license bypass, compatibility overclaim, release claim, code-compliance claim, professional-acceptance claim, or validation claim.
> - No target-specific field semantics beyond DEL-17-01 evidence; unresolved behavior remains `TBD`.

## Completion and Reliance Basis — Epistemology

### CLM-015 — Specification: DEL-17-02 Export package, profile, and stable ID map contracts

> #### Specification: DEL-17-02 Export package, profile, and stable ID map contracts
>

### CLM-016 — Normative Scope

> ##### Normative Scope
>
> DEL-17-02 shall define common contract requirements for PKG-17 export packages, profiles, stable ID maps, manifests, and loss reports.
>
> This deliverable shall not implement exporters, schemas, tests, parsers, executable harnesses, GUI features, persistence runtime, public API endpoints, target adapters, or target-file writers. Later deliverables shall implement concrete artifacts only after consuming this contract and their own approved source basis.
>

### CLM-017 — Source Authority Requirements

> ##### Source Authority Requirements
>
> | Req ID | Requirement |
> |---|---|
> | DEL-17-02-REQ-001 | The deliverable shall consume `DEL-17-01` as the source-basis authority for target-format claims. |
> | DEL-17-02-REQ-002 | Target-specific behavior not resolved by `DEL-17-01 Source_Basis_Register.md#TBD Register` or `CAEPIPE_Question_Dossier.md#Question Register` shall remain `TBD`. |
> | DEL-17-02-REQ-003 | The common contract shall cite project-owned schema/governance sources for canonical identity, hashing, data-boundary, provenance, lifecycle, and professional-boundary requirements. |
> | DEL-17-02-REQ-004 | Later `DEL-17-*` deliverables shall not invent target behavior that is absent from the admitted source basis. |
> | DEL-17-02-REQ-005 | Profiles shall record the source IDs consumed from `DEL-17-01`, including `CAEPIPE-IMPORT-MBF`, `CAEPIPE-EXPORT-DATA`, `CAEPIPE-EXPORT-MBF`, `CAEPIPE-BATCH`, `CAEPIPE-PCF`, `GLTF-2.0`, and relevant project references, or explicitly mark the location/source as `TBD`. |
>

### CLM-018 — Architecture-Basis Requirements

> ##### Architecture-Basis Requirements
>
> | Req ID | Requirement |
> |---|---|
> | DEL-17-02-REQ-006 | Future concrete export contract schemas derived from this deliverable shall use JSON Schema 2020-12 unless a later approved architecture-basis change supersedes it. |
> | DEL-17-02-REQ-007 | Deterministic JSON payload hashing shall use the project canonical JSON/JCS-compatible hash basis where hashes are recorded for package members or manifests. |
> | DEL-17-02-REQ-008 | Schema-first command, query, and job-result envelopes remain implementation-facing constraints for later API or adapter work; this deliverable records the contract boundary only. |
>

### CLM-019 — Export Package Requirements

> ##### Export Package Requirements
>
> | Req ID | Requirement |
> |---|---|
> | DEL-17-02-REQ-010 | An export package shall declare a package ID, source model identity, source model version or hash basis, export profile ID, and generation context. |
> | DEL-17-02-REQ-011 | An export package shall list every emitted package member, including target files, JSON payloads, ID maps, manifests, loss reports, diagnostics, and sidecars. |
> | DEL-17-02-REQ-012 | Package members shall have stable paths or declared path-generation rules. |
> | DEL-17-02-REQ-013 | Hashes shall be recorded for deterministic text/JSON members where applicable; binary or target-owned outputs shall either record a hash or record why a hash is unavailable. |
> | DEL-17-02-REQ-014 | Timestamp fields shall be explicitly declared as deterministic, normalized, omitted, or intentionally runtime-dependent. |
>

### CLM-020 — Export Profile Requirements

> ##### Export Profile Requirements
>
> | Req ID | Requirement |
> |---|---|
> | DEL-17-02-REQ-020 | An export profile shall declare target family, profile version, target version basis, source-basis IDs, and profile boundary notes. |
> | DEL-17-02-REQ-021 | An export profile shall declare unit policy, coordinate policy, stable-ID policy, and loss-report policy. |
> | DEL-17-02-REQ-022 | An export profile shall classify supported entity families as exported, omitted, approximated, delegated, unsupported, or `TBD`. |
> | DEL-17-02-REQ-023 | A target-specific profile shall not treat unresolved `DEL-17-01` questions as support claims. |
> | DEL-17-02-REQ-024 | External execution, where relevant, shall be optional and user-owned; the profile may record harness metadata but shall not require bundled commercial executables. |
> | DEL-17-02-REQ-025 | Target code/check options shall be recorded only as pass-through target configuration unless a separate public rule-pack design admits local logic. |
> | DEL-17-02-REQ-026 | `PLAN-EXPORT-INTEROP` may be cited for strategy and target ordering only while its precise section remains `location TBD`; it shall not substitute for target-format field evidence. |
>

### CLM-021 — Stable ID Map Requirements

> ##### Stable ID Map Requirements
>
> | Req ID | Requirement |
> |---|---|
> | DEL-17-02-REQ-030 | The ID map shall preserve links from canonical OpenPipeStress IDs to emitted target records, files, metadata fields, or sidecar rows. |
> | DEL-17-02-REQ-031 | The ID map shall record unmapped canonical IDs with omission or unsupported reasons. |
> | DEL-17-02-REQ-032 | The ID map shall be stable across deterministic re-export when the source model identity and profile are unchanged. |
> | DEL-17-02-REQ-033 | A target file that cannot carry canonical IDs shall be paired with a sidecar mapping referenced by the manifest. |
> | DEL-17-02-REQ-034 | ID-map entries shall distinguish source canonical identity from target-generated identity. |
> | DEL-17-02-REQ-035 | ID-map entries shall classify the carrier as `direct_target_carried`, `metadata_carried`, `sidecar_mapping`, or `omitted_target_entry`. |
> | DEL-17-02-REQ-036 | A downstream profile shall not claim a direct target-carried ID unless the target carrier is source-confirmed for that profile. |
> | DEL-17-02-REQ-037 | CAEPIPE MBF direct ID carrying remains `TBD-17-01-003`; sidecar mapping is the conservative fallback until downstream evidence closes the carrier question. |
>

### CLM-022 — Manifest Requirements

> ##### Manifest Requirements
>
> | Req ID | Requirement |
> |---|---|
> | DEL-17-02-REQ-040 | The manifest shall record source model reference, export package ID, export profile ID, package member inventory, and source-basis references. |
> | DEL-17-02-REQ-041 | The manifest shall include diagnostics and boundary notes sufficient to audit unsupported behavior without opening every package member. |
> | DEL-17-02-REQ-042 | The manifest shall indicate whether target-specific fields are source-confirmed, delegated, unsupported, or `TBD`. |
> | DEL-17-02-REQ-043 | The manifest shall identify sidecar ID-map and loss-report members whenever a target artifact cannot carry canonical identity or loss categories directly. |
>

### CLM-023 — Loss Report Requirements

> ##### Loss Report Requirements
>
> | Req ID | Requirement |
> |---|---|
> | DEL-17-02-REQ-050 | The loss report shall be mandatory for every target exporter, including native project-owned exports. |
> | DEL-17-02-REQ-051 | Loss report entries shall include affected canonical IDs, category, severity, target artifact if any, reason, source-basis reference, and downstream implication. |
> | DEL-17-02-REQ-052 | Approximation entries shall describe the approximation at a contract level and shall not hide target limitations. |
> | DEL-17-02-REQ-053 | Unsupported and `TBD` entries shall be blocking when they affect solver-ready or compatibility-sensitive output. |
> | DEL-17-02-REQ-054 | Loss reports shall use the categories `exported`, `omitted`, `approximated`, `delegated`, `unsupported`, and `tbd`. |
> | DEL-17-02-REQ-055 | `Delegated` entries shall identify the target configuration or external workflow that receives the behavior without claiming local interpretation. |
> | DEL-17-02-REQ-056 | `TBD` entries shall name the governing TBD ID when the gap maps to `TBD-17-01-001` through `TBD-17-01-006`. |
>

### CLM-024 — Downstream Requirements

> ##### Downstream Requirements
>
> | Downstream deliverable | Required use of DEL-17-02 |
> |---|---|
> | DEL-17-03 | Use the common package/profile/ID-map/loss-report contract for the native open JSON package. |
> | DEL-17-04 | Use the contract to define deterministic CAEPIPE MBF writer behavior without hidden unsupported approximations. |
> | DEL-17-05 | Use manifest and profile metadata for optional CAEPIPE external-run harness records and CSV parser boundaries. |
> | DEL-17-06 | Use loss reports and stable IDs for stress-neutral CSV/JSON result packages. |
> | DEL-17-07 | Use profile and loss-report rules for conservative PCF subset exports. |
> | DEL-17-08 | Use stable IDs and metadata/sidecar policy for GLB/glTF review geometry. |
> | DEL-17-09 | Use the common contract as the adapter SDK target contract. |
>

### CLM-025 — Boundary Requirements

> ##### Boundary Requirements
>
> | Req ID | Requirement |
> |---|---|
> | DEL-17-02-REQ-090 | The contract shall not include protected standards values, proprietary target examples, private client data, or reverse-engineered target behavior. |
> | DEL-17-02-REQ-091 | The contract shall not claim CAEPIPE compatibility, PCF completeness, glTF solver validity, release readiness, code compliance, professional acceptance, or formal validation. |
> | DEL-17-02-REQ-092 | All later target deliverables shall carry forward unresolved `DEL-17-01` TBDs until source evidence or human scope authority resolves them. |
> | DEL-17-02-REQ-093 | External target execution shall remain optional, user-owned, environment-specific, and license-bound; this contract shall not require or bundle target executors. |
> | DEL-17-02-REQ-094 | Public artifacts shall not include protected standards text, protected tables, commercial examples, proprietary model files, private owner criteria, or license-bypass guidance. |
>

### CLM-026 — Acceptance Requirements

> ##### Acceptance Requirements
>
> DEL-17-02 is acceptable when:
>
> - the four-document kit exists;
> - semantic and dependency artifacts exist;
> - `Dependencies.csv` validates against the v3.1 dependency schema;
> - the DAG-005/root blocker queue uses the active evidence register and records `DEL-17-01` as committed;
> - `DEL-17-03` through `DEL-17-09` remain unpopulated beyond existing scaffolds in this tranche;
> - no unauthorized schema, code, lifecycle, release, compatibility, or professional claim is made.

- **AC-001** — The contract preserves source-basis identifiers, JSON Schema 2020-12 and no-bypass boundaries, export identity and hash metadata, explicit profile/version and unit/coordinate semantics, stable canonical-ID carrier and sidecar modes, exported/omitted/approximated/delegated/unsupported/TBD loss categories, diagnostics and provenance, downstream target gates, protected/private-data controls, and all carried target-specific TBDs without implying equivalence or professional acceptance.

## Production and Verification Method — Praxeology

### CLM-027 — Procedure: DEL-17-02 Export package, profile, and stable ID map contracts

> #### Procedure: DEL-17-02 Export package, profile, and stable ID map contracts
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-028 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-17-02-DECL-004`.
>

### CLM-029 — Population Procedure

> ##### Population Procedure
>
> 1. Confirm DAG-006 is the approved active graph authority.
> 2. Confirm `DEL-17-01` has committed implementation evidence before treating `DEL-17-02` as the next unblocked PKG-17 population target.
> 3. Read the local `DEL-17-01` four-document kit, `Source_Basis_Register.md`, and `CAEPIPE_Question_Dossier.md`.
> 4. Populate this four-document kit at contract level only.
> 5. Build semantic artifacts that map source basis, requirements, downstream consumers, and boundary exclusions.
> 6. Extract the local dependency register into `Dependencies.csv`.
> 7. Refresh deliverable memory and create a run record.
> 8. Validate the four-document kit, minimum fileset, dependency schema, coordination blocker queue, and diff hygiene.
>

### CLM-030 — Downstream Consumption Procedure

> ##### Downstream Consumption Procedure
>
> For each later PKG-17 target deliverable:
>
> 1. Start from this contract and the `DEL-17-01` source-basis dossier.
> 2. Declare the target profile and target version basis.
> 3. Cite the consumed `DEL-17-01` source IDs or record `location TBD` for unresolved source locations.
> 4. Declare exported, omitted, approximated, delegated, unsupported, and `TBD` behavior.
> 5. Define stable ID behavior before target writing or parser work, distinguishing direct target-carried IDs from sidecar mappings.
> 6. Require an export manifest and loss report.
> 7. Keep target-specific implementation inside that deliverable's approved write scope.
> 8. Preserve project data-boundary and professional-boundary exclusions.
>

### CLM-031 — Contract Population Checklist

> ##### Contract Population Checklist
>
> When refining or consuming this contract:
>
> 1. Confirm `Source_Basis_Register.md#Public and Official Source Evidence` is the source for target source IDs.
> 2. Confirm `CAEPIPE_Question_Dossier.md#Question Register` is the source for CAEPIPE open questions.
> 3. Carry forward `TBD-17-01-001` through `TBD-17-01-006` unless a later admissible source and human scope authority closes the item.
> 4. Treat `TBD-17-01-003` as blocking direct MBF stable-ID carrier claims.
> 5. Use sidecar mappings whenever direct or metadata carrier evidence is absent, ambiguous, or `TBD`.
> 6. Record target executor use only as optional user-owned, license-bound harness metadata.
> 7. Keep target code/check options as pass-through target configuration unless separately admitted by a public rule-pack design.
>

### CLM-032 — Validation Commands

> ##### Validation Commands
>
> Run from repository root:
>
> ```bash
> tools/validation/check_four_documents.sh "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts"
> tools/validation/check_min_viable_fileset.sh "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts"
> python3 tools/validation/validate_dependencies_schema.py "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts/Dependencies.csv"
> python3 tools/validation/validate_semantic_matrix.py "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts"
> python3 tools/validation/validate_lens_register.py "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts"
> rg -n "certify|certified|approve|approved|issue|issued|code compliance|code-compliant|ASME table|protected table|proprietary|validation|validated|formal acceptance|compatibility|CAEPIPE requirement|reverse engineer" "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts" # prohibited-claims screen per PRD §21.2
> rg -n "TBD|tbd|location TBD" "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts"
> git diff --check -- "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts"
> ```
>

### CLM-033 — Current validator-path evidence (D-41 R5 T7 PDU-059)

> ###### Current validator-path evidence (D-41 R5 T7 PDU-059)
>
> On 2026-07-12 all five repository-local validator commands above were re-run
> from the repository root and passed: four-document, minimum-fileset,
> dependency-schema (22 rows), semantic-matrix, and lens-register validation.
> `Review_Findings.csv` RF-001 is retained unchanged as a historical formal
> review record with `HumanDisposition=TBD`; this current evidence does not
> retroactively rewrite that review.
>
> If an unscoped worktree check reports findings outside this project's write scope, record the finding as external-scope noise and do not treat it as a blocker for DEL-17-02 closeout. Scoped deliverable checks remain the controlling diff-hygiene evidence for this deliverable.
>

### CLM-034 — Closeout Procedure

> ##### Closeout Procedure
>
> Closeout requires:
>
> - four-document validation pass;
> - minimum viable fileset validation pass;
> - dependency schema validation pass;
> - semantic matrix validation pass;
> - lens-register validation pass;
> - overclaim/prohibited-term scan reviewed for negative guardrail language versus unsupported affirmative claims;
> - TBD scan reviewed and summarized in `MEMORY.md`;
> - scoped diff-hygiene validation pass, with any outside-project findings recorded as external-scope bypasses rather than blockers;
> - memory update with touched files, validation results, remaining TBDs, and boundary exclusions;
> - run record creation;
> - no edits to `DEL-17-03` through `DEL-17-09` production documents;
> - no lifecycle promotion;
> - no code, schema, release, compatibility, professional, or code-compliance claim.
>

### CLM-035 — Semantic Enrichment Verification

> ##### Semantic Enrichment Verification
>
> Pass 3 semantic-lensing enrichment checked `_SEMANTIC_LENSING.md` items `A-001`, `B-001`, `X-001`, and `E-001` against `Specification.md` architecture-basis requirements, `Datasheet.md` local artifact inventory, this procedure's validation and closeout checks, and `Guidance.md` reviewer checklist. The resulting closeout checks require downstream target profiles to preserve the architecture basis, keep the local contract artifact set auditable, validate semantic/lens artifacts before consumption, and carry semantic readiness evidence into review.

- **VER-001** — Validate the contract and review source parity, every package/profile/ID-map/manifest/loss-report field, canonical hash and unit boundaries, source-ID carryforward, deterministic ordering and diagnostics, no-silent-loss behavior, downstream target gates, retained target/version/carrier/container TBDs, and prohibited compatibility, compliance, release, or professional-authority claims.

## Governing Values and Decisions — Axiology

### CLM-036 — Guidance: DEL-17-02 Export package, profile, and stable ID map contracts

> #### Guidance: DEL-17-02 Export package, profile, and stable ID map contracts
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-037 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-17-02-DECL-003`.
>

### CLM-038 — Reader Orientation

> ##### Reader Orientation
>
> Use this deliverable as the shared contract spine for PKG-17. It tells later exporters what they must disclose and preserve before they write native JSON, CAEPIPE MBF, CAEPIPE harness records, stress-neutral CSV/JSON, PCF, GLB/glTF, or adapter SDK targets.
>
> The important design rule is simple: an export may be partial, but it shall be explicit. Unsupported target behavior, omitted entities, approximations, delegated execution, and unanswered source questions must be visible in the package manifest, ID map, and loss report.
>

### CLM-039 — Design Guidance

> ##### Design Guidance
>

### CLM-040 — Export package

> ###### Export package
>
> An export package should be treated as an auditable bundle concept, not only as a target file. Even when the visible output is a single target file, the project still needs a manifest, stable-ID record, diagnostics, and loss report to explain what the output represents.
>

### CLM-041 — Export profile

> ###### Export profile
>
> An export profile should be the only place where target-specific export assumptions are declared. Later implementation work should consume the profile and fail or warn from its declared coverage. Hidden target defaults are a risk because they make later parser, comparison, and support workflows ambiguous.
>
> Profile authors should prefer narrow, source-confirmed profiles over broad profiles with unproven target behavior. If a target behavior is plausible but not yet sourced, mark it `TBD` and include the affected downstream deliverables.
>
> Use `DEL-17-01 Source_Basis_Register.md` as the upstream authority for admitted source IDs. For non-trivial target statements, cite one or more of `CAEPIPE-IMPORT-MBF`, `CAEPIPE-EXPORT-DATA`, `CAEPIPE-EXPORT-MBF`, `CAEPIPE-BATCH`, `CAEPIPE-PCF`, `GLTF-2.0`, or a project reference such as `CONTRACT`, `IP-DATA`, `SPEC`, `TYPES`, and `DAG-006`. If only `PLAN-EXPORT-INTEROP` supports the planning direction, preserve its plan-location `TBD` and do not convert strategy into target-field evidence.
>

### CLM-042 — Stable ID map

> ###### Stable ID map
>
> Stable IDs are the recovery path when a target format cannot preserve the OpenPipeStress model structure directly. The map should be useful for:
>
> - locating a target artifact from a canonical model entity;
> - tracing a target result row back to a model entity;
> - explaining omitted entities;
> - reconciling external run outputs with OpenPipeStress records;
> - diagnosing target-specific approximations.
>
> Do not use target-generated record order as the only identity mechanism unless the manifest also records why no stronger identity is available.
>
> Distinguish direct target-carried IDs from sidecar mappings. A direct target-carried ID requires a source-confirmed target field, metadata slot, name, or equivalent carrier. A sidecar mapping is the default when the carrier is absent, ambiguous, or still `TBD`. For CAEPIPE MBF, `TBD-17-01-003` remains open, so later MBF work should not claim direct in-file stable-ID carrying until that question is closed.
>

### CLM-043 — Loss report

> ###### Loss report
>
> A loss report should be written for normal exports, not only error cases. A successful target export can still have delegated, omitted, approximated, unsupported, or TBD behavior. This is especially important for CAEPIPE, PCF, and review-geometry targets where public source evidence does not authorize broad compatibility claims.
>
> Use the categories consistently:
>
> - `exported`: represented directly in the target artifact or package member;
> - `omitted`: intentionally not emitted, with affected canonical IDs;
> - `approximated`: emitted through a declared approximation;
> - `delegated`: passed through to target configuration or a user-owned workflow without local interpretation;
> - `unsupported`: known unsupported behavior or target limitation;
> - `tbd`: behavior not source-confirmed.
>
> Do not let `delegated` or `approximated` entries hide target limitations. If target code/check options appear, treat them as pass-through target configuration unless a separate public rule-pack design admits local logic.
>

### CLM-044 — Target-Specific Carryforward

> ##### Target-Specific Carryforward
>
> | Target path | Carryforward from DEL-17-01 |
> |---|---|
> | CAEPIPE MBF | CAEPIPE version/profile, initial MBF record families, required fields, and stable ID carrying strategy remain `TBD` until downstream source review or developer-team clarification closes them. |
> | CAEPIPE external run/CSV | CSV section stability, parser coverage, and licensed executable availability remain `TBD`; harnesses are user-owned and optional. |
> | PCF | Conservative subset and translator-default warning rules remain `TBD`; PCF work shall be loss-report driven. |
> | GLB/glTF | Identity metadata and sidecar policy remain `TBD`; review geometry shall not be treated as solver validation. |
> | Adapter SDK | Additional targets shall inherit this contract and declare their own source basis before target-specific claims. |
>

### CLM-045 — External Execution Guidance

> ##### External Execution Guidance
>
> External target execution is optional and user-owned. A profile may record executable path fields, version fields, license/environment notes, invocation metadata, and output-member hashes where those are useful for a later harness. The contract shall not require bundled executables, copied target examples, license-independent operation, or local interpretation of target solver/code-check behavior.
>

### CLM-046 — Boundary Guidance

> ##### Boundary Guidance
>
> - Use public, official, project-owned, or rights-cleared sources only.
> - Do not copy protected standards tables, commercial examples, proprietary model files, or private target data.
> - Do not describe external tool execution as bundled, automatic, or license-independent.
> - Do not claim compatibility, code compliance, professional acceptance, formal validation, or release readiness from this contract.
> - Do not resolve `TBD` behavior by inference from target names or common industry practice.
>

### CLM-047 — Reviewer Checklist

> ##### Reviewer Checklist
>
> - Does every target-specific statement trace to `DEL-17-01` or a project governance source?
> - Are unresolved questions marked `TBD` instead of softened into support language?
> - Does the ID-map guidance preserve canonical identity even when the target format cannot?
> - Does the loss report include successful-export loss categories?
> - Are later deliverables identified as consumers without being populated in this tranche?
> - Did semantic matrix, lens-register, four-document, minimum-fileset, dependency-schema, and diff-hygiene validation pass before downstream consumption?

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-030 SOW-074 OBJ-009 OBJ-017 | CLM-015 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
