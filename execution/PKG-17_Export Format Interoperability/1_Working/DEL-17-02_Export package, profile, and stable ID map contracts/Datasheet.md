# Datasheet: DEL-17-02 Export package, profile, and stable ID map contracts

## Identity

| Field | Value |
|---|---|
| Deliverable ID | DEL-17-02 |
| Package | PKG-17 Export Format Interoperability |
| Type | API_CONTRACT |
| Lifecycle state after this tranche | SEMANTIC_READY |
| Source basis | DEL-17-01 source-basis dossier and admitted references |
| Downstream consumers | DEL-17-03 through DEL-17-09 |

## Purpose

DEL-17-02 defines the common export contract that later PKG-17 target exporters shall consume before writing target-specific formats. It covers export package structure, export profile metadata, stable ID maps, export manifests, loss reports, and unsupported/approximated/delegated behavior records.

This deliverable is a contract-level planning artifact. It does not implement JSON schemas, Rust modules, exporters, parsers, harnesses, GUI behavior, persistence runtime, public API endpoints, or target-specific file writers.

## Contract Objects

| Object | Role | Required downstream behavior |
|---|---|---|
| Export package | Binds payloads, target files, manifests, ID maps, diagnostics, and loss reports into one reproducible package concept. | Every exporter shall identify which package members it emits and which are outside its tranche. |
| Export profile | Declares target name, profile version, target version basis, unit policy, coordinate policy, entity coverage, and target-specific TBDs. | Target exporters shall consume a profile rather than encode hidden target assumptions. |
| Stable ID map | Maps OpenPipeStress canonical IDs to target record IDs, file names, sidecars, metadata slots, or omitted-target entries. | Every target artifact shall preserve identity directly or through a sidecar map. |
| Export manifest | Records source model identity, profile identity, package members, hashes where applicable, diagnostics, and boundary notes. | Export packages shall be auditable without relying on unstated runtime context. |
| Loss report | Records exported, omitted, approximated, delegated, unsupported, and TBD behavior. | Exporters shall not silently approximate unsupported target concepts. |

## Source Consumption

| Source | Consumption rule |
|---|---|
| DEL-17-01 `Source_Basis_Register.md` | Upstream source-basis authority for admitted public/project source IDs, findings, boundaries, and TBD register. SourceRef: `Source_Basis_Register.md#Public and Official Source Evidence`, `#Accepted Project References`, `#TBD Register`. |
| DEL-17-01 `CAEPIPE_Question_Dossier.md` | Upstream question register for unresolved CAEPIPE claims. SourceRef: `CAEPIPE_Question_Dossier.md#Question Register`, `#Question Boundary`, `#Closure Rule`. |
| SCA-004 and DAG-005 | Establish PKG-17 scope and sequencing authority. |
| `docs/CONTRACT.md` and `docs/IP_AND_DATA_BOUNDARY.md` | Govern protected-content, professional-boundary, and no-bypass constraints. |
| `schemas/model.schema.yaml` | Supplies canonical model identity vocabulary for mapping, without changing the schema in this tranche. |

## DEL-17-01 Source IDs Carried Forward

| Source ID | DEL-17-02 use | Location basis |
|---|---|---|
| CAEPIPE-IMPORT-MBF | MBF text model input and command-line planning evidence for later CAEPIPE profiles. | `Source_Basis_Register.md#Public and Official Source Evidence`; CAEPIPE `IMPORT MBF`, `Command line operation`, `Format of .MBF`. |
| CAEPIPE-EXPORT-DATA | CAEPIPE model/result export-surface evidence for later CSV/text parser scoping. | `Source_Basis_Register.md#Public and Official Source Evidence`; CAEPIPE `Export data from CAEPIPE`. |
| CAEPIPE-EXPORT-MBF | CAEPIPE MBF export evidence for later MBF profile/version discussion. | `Source_Basis_Register.md#Public and Official Source Evidence`; CAEPIPE `EXPORT MBF`. |
| CAEPIPE-BATCH | Optional external-run harness evidence, limited to user-owned licensed execution planning. | `Source_Basis_Register.md#Public and Official Source Evidence`; CAEPIPE batch `RUNNING CAEPIPE/CAEPIPE 3D+ IN BATCH MODE`. |
| CAEPIPE-PCF | PCF translator planning evidence with mapping/default caveats for conservative downstream subset work. | `Source_Basis_Register.md#Public and Official Source Evidence`; PCF PDF `Reference`, `PCF to CAEPIPE component Mapping`. |
| GLTF-2.0 | Review-geometry package evidence for glTF/GLB asset, identity, coordinate, and container semantics. | `Source_Basis_Register.md#Public and Official Source Evidence`; glTF `2.4`, `3.2`, `3.3`, `3.4`, `4`. |
| PLAN-EXPORT-INTEROP | Strategy and priority basis only; section location remains `location TBD`. | `Source_Basis_Register.md#Accepted Project References`; `location TBD within plan`. |
| CONTRACT / IP-DATA / SPEC / TYPES / DAG-005 | Governance, workflow, lifecycle, provenance, TBD, data-boundary, and active graph-coordination basis. | `Source_Basis_Register.md#Accepted Project References`; local governance files and DAG-006 approval record locations cited there. |

## Local Contract Artifact Inventory

| Artifact | Contract role |
|---|---|
| `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md` | Human-readable contract kit for the export package/profile/stable-ID/loss-report surface. |
| `_SEMANTIC.md` | Semantic matrix lens for coverage review; not an engineering authority. |
| `_SEMANTIC_LENSING.md` | Warranted enrichment register applied by Pass 3; not an authority source by itself. |
| `Dependencies.csv` | v3.1 dependency register tying DEL-17-02 to upstream source-basis evidence and downstream PKG-17 consumers. |
| `_run_records/` | Auditable TASK execution records for sealed population and validation passes. |

## Required Export-Profile Fields

| Field | Meaning |
|---|---|
| `profile_id` | Stable identifier for the export profile. |
| `profile_version` | Version of the OpenPipeStress profile contract. |
| `target_family` | Target family such as native JSON, CAEPIPE MBF, stress-neutral CSV/JSON, PCF, GLB/glTF, or adapter SDK target. |
| `target_version_basis` | Cited target version or `TBD` when not yet source-confirmed. |
| `source_basis_ids` | DEL-17-01 source IDs and project references used by the profile. |
| `unit_policy` | Declared output units and conversion responsibility. |
| `coordinate_policy` | Declared coordinate frame, axis convention, and origin policy. |
| `entity_coverage` | Supported, omitted, approximated, delegated, unsupported, and TBD entity families. |
| `stable_id_policy` | Direct ID, sidecar ID, metadata ID, or omitted ID behavior. |
| `loss_report_policy` | Required loss-report categories and blocking diagnostics. |
| `external_execution_policy` | Optional user-owned harness metadata only, when applicable. |
| `boundary_notes` | IP, license, private-data, and professional-boundary constraints. |

## Stable ID Families

The common ID map shall account for at least:

- project and model IDs;
- line, node, element, component, support, material, section, load case, result case, analysis run, and export IDs;
- target file IDs, target record IDs, target metadata keys, target sidecar IDs, and omitted-target placeholders.

If a target cannot carry a canonical ID directly, the exporter shall record the mapping in a sidecar and reference that sidecar from the manifest.

## Stable ID Carrier Modes

| Mode | Required meaning | DEL-17-02 boundary |
|---|---|---|
| Direct target-carried ID | The target artifact contains the canonical OpenPipeStress ID in a source-confirmed target field, metadata slot, name, or equivalent target-carried location. | Allowed only when the downstream target profile cites admitted evidence for the carrier. |
| Metadata-carried ID | The target artifact carries canonical IDs through a documented metadata surface. | Allowed only when the target metadata behavior is source-confirmed and does not imply solver equivalence. |
| Sidecar mapping | The target artifact does not carry the canonical ID directly; the export package includes a separate ID-map member referenced from the manifest. | Required fallback whenever direct or metadata carriage is unsupported, ambiguous, or `TBD`. |
| Omitted-target entry | A canonical ID has no target artifact representation. | Must appear in the ID map and loss report with category, reason, and affected canonical IDs. |

For CAEPIPE MBF, direct target-carried stable ID claims remain blocked by `TBD-17-01-003`. DEL-17-02 permits sidecar-only MBF mapping as a conservative contract mode, but it does not decide the downstream MBF carrier policy.

## Loss Categories

| Category | Meaning |
|---|---|
| `exported` | Represented directly in the target package. |
| `omitted` | Not exported, with explicit reason and affected canonical IDs. |
| `approximated` | Represented by a declared approximation with source and target IDs. |
| `delegated` | Passed through to a target tool or external workflow without local interpretation. |
| `unsupported` | Known unsupported behavior that blocks or limits target use. |
| `tbd` | Behavior not source-confirmed; shall not be represented as supported. |

Each category shall be visible in the export manifest or loss report even when the export completes. `delegated` means passed through to target configuration or a user-owned external workflow without local interpretation; it does not admit local code-checking logic or bundled target execution.

## DEL-17-01 TBD Carryforward

| TBD ID | DEL-17-02 treatment | Downstream handoff |
|---|---|---|
| TBD-17-01-001 | Preserve target version/profile as `TBD` in export profiles until CAEPIPE version/profile authority is recorded. | DEL-17-04, DEL-17-05 |
| TBD-17-01-002 | Preserve first MBF record subset as `TBD`; do not encode a first writer subset here. | DEL-17-04 |
| TBD-17-01-003 | Preserve MBF direct carrier versus sidecar-only stable-ID policy as `TBD`. | DEL-17-02, DEL-17-04 |
| TBD-17-01-004 | Preserve CSV result-section parser scope as `TBD`. | DEL-17-05, DEL-17-06 |
| TBD-17-01-005 | Preserve conservative PCF subset and default-warning policy as `TBD`. | DEL-17-07 |
| TBD-17-01-006 | Preserve glTF/GLB review-geometry identity metadata and sidecar policy as `TBD`. | DEL-17-08 |

## Boundary Summary

- No bundled commercial solver or target executable.
- No proprietary examples, private target data, protected standards tables, material allowables, SIF/flexibility values, or owner criteria.
- No reverse engineering, license bypass, compatibility overclaim, release claim, code-compliance claim, professional-acceptance claim, or validation claim.
- No target-specific field semantics beyond DEL-17-01 evidence; unresolved behavior remains `TBD`.
